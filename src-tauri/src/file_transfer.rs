//! 远程文件传输服务端核心（rpc.call 分块命令的实现，commands.rs 仅薄包装）。
//!
//! - 上传：`file_upload_begin/chunk/finish/abort`，落盘到会话项目根下
//!   `.cc-box-files/<文件名>`；`.upload-<id>.part` 临时文件 + finish 时
//!   hard_link 原子 no-replace 提交（绝不覆盖已有文件，冲突自动 `(1)` 改名）
//! - 下载：`file_download_begin/chunk`，范围锁定已知项目内（is_known_project
//!   白名单）；会话固化 size/mtime，源文件变化即拒（防新旧版本拼接）
//!
//! 安全模型：
//! - 读写范围锁定授权项目目录内（`.cc-box-files` canonicalize 后必须仍在
//!   项目根内——防符号链接/junction 逃逸）
//! - `connId`/`viaRelay` 由 rpc_dispatch::handle_rpc 服务端注入（覆盖客户端
//!   任何伪造值）；桌面路径无注入 → 上传/下载直接拒绝（remote only）
//! - 会话绑定连接：uploadId/downloadId 归属校验（他人 id 与不存在同文案，
//!   不泄露存在性）
//! - 大小限额按连接类型：中继流量按量计费（消耗 ≈2.7× 文件大小），从严；
//!   直连走局域网，宽松。`0` = 禁用传输

use std::collections::HashMap;
use std::fs::{self, File, OpenOptions};
use std::io::{Read, Seek, SeekFrom, Write};
use std::path::{Path, PathBuf};
use std::sync::LazyLock;
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

use parking_lot::Mutex;

use base64::Engine;
use serde::Serialize;

use crate::remote_state::REMOTE_STATE;

/// 分块大小（字节）：base64 后 ~683KB，远低于 relay 4MiB 消息上限
pub const CHUNK_SIZE: usize = 512 * 1024;
pub const DEFAULT_FILE_MAX_DIRECT: u64 = 200 * 1024 * 1024;
pub const DEFAULT_FILE_MAX_RELAY: u64 = 20 * 1024 * 1024;
const SESSION_TTL: Duration = Duration::from_secs(600);
const MAX_ACTIVE_UPLOADS: usize = 8;
/// 下载票据 TTL：微信把链接交给系统浏览器/用户稍后打开都可能延迟
const TICKET_TTL: Duration = Duration::from_secs(600);
/// 票据并发上限（防止内存表被大量签发打满；成功下载会立即释放，正常不会接近）
const MAX_TICKETS: usize = 64;
/// 票据可用次数：20MB/512KB=40 块，再加 Range/重试余量
const TICKET_USES: u32 = 256;
/// 项目内上传目录名（会话 cwd 下按需创建）
pub(crate) const UPLOAD_DIR_NAME: &str = ".cc-box-files";
/// 上传临时文件名：.upload-<transferId>.part（file_list 过滤、启动清理识别）
const PART_PREFIX: &str = ".upload-";
const PART_SUFFIX: &str = ".part";

// ==================== 限额 ====================

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TransferLimits {
    pub direct_max: u64,
    pub relay_max: u64,
}

/// 当前生效限额（config.remote 覆盖默认值；0 = 禁用传输）
pub fn limits() -> TransferLimits {
    let remote = crate::store::get_app_config()
        .ok()
        .and_then(|c| c.remote)
        .unwrap_or_default();
    TransferLimits {
        direct_max: remote
            .file_transfer_max_direct
            .unwrap_or(DEFAULT_FILE_MAX_DIRECT),
        relay_max: remote
            .file_transfer_max_relay
            .unwrap_or(DEFAULT_FILE_MAX_RELAY),
    }
}

fn limit_for(via_relay: bool) -> u64 {
    let l = limits();
    if via_relay { l.relay_max } else { l.direct_max }
}

fn conn_kind(via_relay: bool) -> &'static str {
    if via_relay { "relay" } else { "direct" }
}

fn fmt_mb(bytes: u64) -> String {
    format!("{:.1}MB", bytes as f64 / 1024.0 / 1024.0)
}

/// 展示/回传路径：Windows canonicalize 返回 verbatim 前缀（`\\?\D:\...`），
/// 直接下发会让前端路径比较（项目根 vs 子目录）失败、界面出现怪前缀——
/// 统一剥离（`\\?\UNC\server\share` → `\\server\share`）
pub(crate) fn display_path(p: &Path) -> String {
    let s = p.to_string_lossy().into_owned();
    #[cfg(windows)]
    {
        if let Some(rest) = s.strip_prefix(r"\\?\UNC\") {
            return format!(r"\\{}", rest);
        }
        if let Some(rest) = s.strip_prefix(r"\\?\") {
            return rest.to_string();
        }
    }
    s
}

/// 项目内相对路径（正斜杠，Claude CLI 在项目根运行，可直接使用）
fn relative_upload_path(file_name: &str) -> String {
    format!("{}/{}", UPLOAD_DIR_NAME, file_name)
}

// ==================== 文件名 sanitize ====================

/// 校验上传文件名（裸文件名，路径由服务端拼接）。
/// 拒绝：空/超长/路径分隔符/空字节/`..`/控制字符/首尾空白/尾点/Windows 保留名
pub fn sanitize_file_name(name: &str) -> Result<String, String> {
    if name.is_empty() {
        return Err("invalid file name: empty".into());
    }
    // Windows 文件名字符集（跨平台同样从严）：拒绝路径分隔符、ADS 冒号、
    // 通配/重定向字符——否则能 begin/传完，最终 hard_link 才失败白费传输
    if name
        .chars()
        .any(|c| matches!(c, '/' | '\\' | ':' | '<' | '>' | '"' | '|' | '?' | '*') || c == '\0')
    {
        return Err("invalid file name: illegal character".into());
    }
    // NTFS 文件名上限 255 个 UTF-16 单元（astral 字符占 2 单元）
    if name.encode_utf16().count() > 200 {
        return Err("invalid file name: too long (max 200)".into());
    }
    if name.contains("..") {
        return Err("invalid file name: '..' not allowed".into());
    }
    if name.chars().any(|c| c.is_control()) {
        return Err("invalid file name: control character".into());
    }
    if name.trim() != name {
        return Err("invalid file name: leading/trailing whitespace".into());
    }
    if name.ends_with('.') {
        return Err("invalid file name: trailing dot".into());
    }
    // Windows 保留设备名：主干（首个 '.' 前）大写化匹配 CON/PRN/AUX/NUL/COM1-9/LPT1-9
    let stem_upper = name.split('.').next().unwrap_or("").to_uppercase();
    let is_reserved = matches!(
        stem_upper.as_str(),
        "CON" | "PRN" | "AUX" | "NUL"
    ) || (stem_upper.len() == 4
        && (stem_upper.starts_with("COM") || stem_upper.starts_with("LPT"))
        && matches!(stem_upper.as_bytes()[3], b'1'..=b'9'));
    if is_reserved {
        return Err("invalid file name: reserved device name".into());
    }
    Ok(name.to_string())
}

// ==================== 锚点校验（路径安全核心） ====================

/// 测试隔离：覆盖已知项目白名单源（真实源读 ~/.claude/projects，测试不可写）。
/// 仿 store::GUI_CONFIG_DIR_OVERRIDE 先例
#[cfg(test)]
static KNOWN_PROJECTS_OVERRIDE: LazyLock<Mutex<Option<Vec<String>>>> =
    LazyLock::new(|| Mutex::new(None));

#[cfg(test)]
pub(crate) fn set_known_projects_override(paths: Option<Vec<String>>) {
    *KNOWN_PROJECTS_OVERRIDE.lock() = paths;
}

/// 已知项目根列表（服务端可信：~/.claude/projects 中出现过的路径）
fn known_project_paths() -> Vec<String> {
    #[cfg(test)]
    {
        if let Some(v) = KNOWN_PROJECTS_OVERRIDE.lock().clone() {
            return v;
        }
    }
    crate::store::get_projects(None, None)
        .map(|ps| ps.into_iter().map(|p| p.path).collect())
        .unwrap_or_default()
}

/// 上传目录解析：`<cwd>/.cc-box-files`（按需创建），canonicalize 后必须仍在
/// 项目根内——`.cc-box-files` 被预置为指向外部的符号链接/junction 时，
/// canonicalize 已解析成外部真实路径，前缀比对即拦截。
fn resolve_upload_dir(cwd: &str) -> Result<PathBuf, String> {
    let root_c = Path::new(cwd)
        .canonicalize()
        .map_err(|e| format!("project dir not accessible: {}", e))?;
    let dir = root_c.join(UPLOAD_DIR_NAME);
    fs::create_dir_all(&dir).map_err(|e| format!("cannot create {}: {}", UPLOAD_DIR_NAME, e))?;
    let dir_c = dir
        .canonicalize()
        .map_err(|e| format!("cannot resolve {}: {}", UPLOAD_DIR_NAME, e))?;
    if !dir_c.starts_with(&root_c) {
        return Err(format!("{} resolves outside the project dir", UPLOAD_DIR_NAME));
    }
    Ok(dir_c)
}

/// 路径（canonicalize 形式）是否位于某已知项目内（等于项目根或其子路径）。
/// 下载/列文件/上传 cwd 的白名单校验
fn is_within_known_project(p_c: &Path) -> bool {
    known_project_paths().iter().any(|proj| {
        Path::new(proj)
            .canonicalize()
            .map(|root_c| p_c.starts_with(root_c))
            .unwrap_or(false)
    })
}

// ==================== 会话表 ====================

struct UploadSession {
    conn_id: u64,
    /// 原始文件名（冲突重命名基于它重算 `stem (n).ext`）
    file_name: String,
    /// 提交目标（begin 时 unique_target 暂定，finish 冲突可变）
    target: PathBuf,
    part: PathBuf,
    expected: u64,
    written: u64,
    via_relay: bool,
    last_active: Instant,
}

struct DownloadSession {
    conn_id: u64,
    path: PathBuf,
    file_name: String,
    size: u64,
    mtime: Option<SystemTime>,
    via_relay: bool,
    last_active: Instant,
}

/// 一次性下载票据：让系统浏览器/系统下载器不带 token 也能直接取文件
/// （微信把 blob:/WebView 内 URL 交给外部进程时两者都不可用）。
/// **不绑连接**——手机切到系统下载器后 WS 常立即断开，绑 conn 会让票据
/// 随 cleanup_conn 一起失效；只按 TTL/次数回收。
struct DownloadTicket {
    path: PathBuf,
    file_name: String,
    size: u64,
    mtime: Option<SystemTime>,
    created: Instant,
    uses_left: u32,
    last_active: Instant,
}

static UPLOADS: LazyLock<Mutex<HashMap<String, UploadSession>>> =
    LazyLock::new(|| Mutex::new(HashMap::new()));
static DOWNLOADS: LazyLock<Mutex<HashMap<String, DownloadSession>>> =
    LazyLock::new(|| Mutex::new(HashMap::new()));
static TICKETS: LazyLock<Mutex<HashMap<String, DownloadTicket>>> =
    LazyLock::new(|| Mutex::new(HashMap::new()));

fn is_hex32(s: &str) -> bool {
    s.len() == 32 && s.chars().all(|c| c.is_ascii_hexdigit())
}

fn now_ms() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_millis() as u64)
        .unwrap_or(0)
}

fn new_transfer_id() -> String {
    uuid::Uuid::new_v4().simple().to_string()
}

fn mtime_ms(md: &fs::Metadata) -> Option<u64> {
    system_time_ms(md.modified().ok())
}

fn system_time_ms(t: Option<SystemTime>) -> Option<u64> {
    t.and_then(|t| t.duration_since(UNIX_EPOCH).ok())
        .map(|d| d.as_millis() as u64)
}

/// 会话归属校验：他人 id 与不存在同文案（不泄露存在性）
fn upload_session_of<'a>(
    map: &'a mut HashMap<String, UploadSession>,
    conn_id: u64,
    upload_id: &str,
) -> Result<&'a mut UploadSession, String> {
    match map.get_mut(upload_id) {
        Some(s) if s.conn_id == conn_id => Ok(s),
        _ => Err("transfer session not found".into()),
    }
}

fn download_session_of<'a>(
    map: &'a mut HashMap<String, DownloadSession>,
    conn_id: u64,
    download_id: &str,
) -> Result<&'a mut DownloadSession, String> {
    match map.get_mut(download_id) {
        Some(s) if s.conn_id == conn_id => Ok(s),
        _ => Err("transfer session not found".into()),
    }
}

// ==================== 上传 ====================

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UploadBeginResult {
    pub upload_id: String,
    pub target_path: String,
    /// 项目内相对路径（`.cc-box-files/<name>`）——前端插入终端/展示用
    pub relative_path: String,
    pub file_name: String,
}

/// 冲突不覆盖取名：`a.pdf` → `a (1).pdf` → `a (2).pdf`…（上限 100 次）
fn unique_target(dir: &Path, name: &str) -> Result<PathBuf, String> {
    let candidate = dir.join(name);
    if !candidate.exists() {
        return Ok(candidate);
    }
    let p = Path::new(name);
    let stem = p.file_stem().and_then(|s| s.to_str()).unwrap_or(name);
    let ext = p.extension().and_then(|e| e.to_str());
    for i in 1..=100u32 {
        let file_name = match ext {
            Some(e) => format!("{} ({}).{}", stem, i, e),
            None => format!("{} ({})", stem, i),
        };
        let candidate = dir.join(&file_name);
        if !candidate.exists() {
            return Ok(candidate);
        }
    }
    Err("too many duplicate files".into())
}

/// 上传会话开启：cwd 由服务端从 ptyId 反查（客户端不可指定路径），
/// 落盘面收敛到 `<授权项目>/.cc-box-files/<sanitize 后文件名>`。
/// conn_id/via_relay 由 handle_rpc 注入；桌面路径（None）拒绝。
pub fn upload_begin(
    conn_id: Option<u64>,
    pty_id: &str,
    file_name: &str,
    size: u64,
    via_relay: Option<bool>,
) -> Result<UploadBeginResult, String> {
    sweep_stale();
    let conn_id = conn_id.ok_or("file transfer is remote-only")?;
    let via_relay = via_relay.unwrap_or(true); // 未知按中继从严

    let limit = limit_for(via_relay);
    if limit == 0 || size > limit {
        return Err(format!(
            "file too large: {} exceeds {} limit {}",
            fmt_mb(size),
            conn_kind(via_relay),
            fmt_mb(limit)
        ));
    }

    let cwd = REMOTE_STATE
        .cwd_of(pty_id)
        .ok_or("no active session for upload")?;
    let cwd_c = Path::new(&cwd)
        .canonicalize()
        .map_err(|e| format!("project dir not accessible: {}", e))?;
    if !is_within_known_project(&cwd_c) {
        return Err("project not authorized for file transfer".into());
    }
    let name = sanitize_file_name(file_name)?;
    let dir_c = resolve_upload_dir(&cwd)?;
    let target = unique_target(&dir_c, &name)?;
    let target_for_resp = target.to_string_lossy().into_owned();

    let upload_id = new_transfer_id();
    let part = dir_c.join(format!("{}{}{}", PART_PREFIX, upload_id, PART_SUFFIX));

    {
        let mut uploads = UPLOADS.lock();
        if uploads.len() >= MAX_ACTIVE_UPLOADS {
            return Err("too many concurrent uploads".into());
        }
        // create_new 独占创建；失败即残留同名临时文件（概率≈0，TTL/启动清理兜底）
        OpenOptions::new()
            .create_new(true)
            .write(true)
            .open(&part)
            .map_err(|e| format!("cannot create temp file: {}", e))?;
        uploads.insert(
            upload_id.clone(),
            UploadSession {
                conn_id,
                file_name: name.clone(),
                target,
                part,
                expected: size,
                written: 0,
                via_relay,
                last_active: Instant::now(),
            },
        );
    }

    let target_path = display_path(Path::new(&target_for_resp));
    log::info!(
        "[file-transfer] upload begin conn={} via={} name={} size={} dir={}",
        conn_id,
        conn_kind(via_relay),
        name,
        size,
        display_path(&dir_c)
    );
    Ok(UploadBeginResult {
        upload_id,
        target_path,
        relative_path: relative_upload_path(&name),
        file_name: name,
    })
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UploadChunkResult {
    pub received: u64,
}

/// 追加写分块（幂等）：
/// - `offset == written`：正常写入（先截断到 written——上次 write_all 若半途
///   失败会留下脏字节，重试同块时丢弃半块，避免"脏前缀 + 完整块"的静默损坏）
/// - `offset + n == written`：该块已写过（ACK 丢失重发），不重写直接返回
/// - 其他：Err（乱序/跳块），会话保留可重试
pub fn upload_chunk(
    conn_id: Option<u64>,
    upload_id: &str,
    offset: u64,
    data_b64: &str,
) -> Result<UploadChunkResult, String> {
    let conn_id = conn_id.ok_or("file transfer is remote-only")?;
    let data = base64::engine::general_purpose::STANDARD
        .decode(data_b64)
        .map_err(|e| format!("invalid base64: {}", e))?;
    let n = data.len() as u64;
    // 单帧上限（直连 axum 默认 64MiB，防超大帧内存放大）
    if data.len() > CHUNK_SIZE {
        return Err(format!(
            "chunk too large: {} bytes (max {})",
            data.len(),
            CHUNK_SIZE
        ));
    }

    let mut uploads = UPLOADS.lock();
    let s = upload_session_of(&mut uploads, conn_id, upload_id)?;
    s.last_active = Instant::now();

    let idempotent = offset
        .checked_add(n)
        .map(|end| end == s.written)
        .unwrap_or(false);

    if offset == s.written {
        // 谎报 size / 限额兜底（begin 校验后配置可能调低）
        if s.written + n > s.expected {
            return Err(format!(
                "upload exceeds declared size ({} > {})",
                s.written + n, s.expected
            ));
        }
        let limit = limit_for(s.via_relay);
        if s.written + n > limit {
            return Err(format!(
                "file too large: {} exceeds {} limit {}",
                fmt_mb(s.written + n),
                conn_kind(s.via_relay),
                fmt_mb(limit)
            ));
        }
        let mut f = OpenOptions::new()
            .write(true)
            .open(&s.part)
            .map_err(|e| format!("cannot write temp file: {}", e))?;
        // 截断到已确认位置（丢弃上次半写失败的残字节）后从该位置写
        f.set_len(s.written)
            .map_err(|e| format!("temp file truncate failed: {}", e))?;
        f.seek(SeekFrom::Start(s.written))
            .map_err(|e| format!("temp file seek failed: {}", e))?;
        f.write_all(&data)
            .map_err(|e| format!("temp file write failed: {}", e))?;
        s.written += n;
    } else if idempotent {
        // 已写过：幂等返回
    } else {
        return Err(format!("offset mismatch: expected {}", s.written));
    }
    Ok(UploadChunkResult { received: s.written })
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UploadFinishResult {
    pub target_path: String,
    /// 项目内相对路径（冲突改名后的最终文件名，正斜杠）
    pub relative_path: String,
}

/// 完成上传：校验字节数后 hard_link 原子提交（no-replace，绝不覆盖已有
/// 文件；冲突自动 `(n)` 改名重试，消除 begin 定名与提交间的并发竞态）
pub fn upload_finish(conn_id: Option<u64>, upload_id: &str) -> Result<UploadFinishResult, String> {
    let conn_id = conn_id.ok_or("file transfer is remote-only")?;
    let session = {
        let mut uploads = UPLOADS.lock();
        // 落盘长度校验：内存计数一致但文件实际长度不符（如半写残留）→ 拒提交
        let (expected, on_disk) = {
            let s = upload_session_of(&mut uploads, conn_id, upload_id)?;
            if s.written != s.expected {
                return Err(format!(
                    "incomplete upload: {} of {} bytes",
                    s.written, s.expected
                ));
            }
            let on_disk = fs::metadata(&s.part).map(|m| m.len()).unwrap_or(0);
            (s.expected, on_disk)
        };
        if on_disk != expected {
            if let Some(s) = uploads.remove(upload_id) {
                let _ = fs::remove_file(&s.part);
            }
            return Err(format!(
                "temp file inconsistent: {} of {} bytes on disk",
                on_disk, expected
            ));
        }
        uploads.remove(upload_id).unwrap()
    };

    let dir = session.part.parent().ok_or("invalid temp path")?.to_path_buf();
    let mut target = session.target.clone();
    let mut committed: Option<PathBuf> = None;
    let mut err: Option<String> = None;
    for _ in 0..5 {
        match fs::hard_link(&session.part, &target) {
            Ok(()) => {
                committed = Some(target.clone());
                break;
            }
            Err(e) if e.kind() == std::io::ErrorKind::AlreadyExists => {
                // 并发同名提交/外部创建：基于原始名重取可用名
                match unique_target(&dir, &session.file_name) {
                    Ok(t) => target = t,
                    Err(e) => {
                        err = Some(e);
                        break;
                    }
                }
            }
            Err(e) => {
                err = Some(format!("commit failed: {}", e));
                break;
            }
        }
    }
    let _ = fs::remove_file(&session.part);

    if let Some(e) = err {
        return Err(e);
    }
    let committed = committed.ok_or("commit failed: name conflicts")?;
    let final_name = committed
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or(&session.file_name)
        .to_string();
    let target_path = display_path(&committed);
    log::info!(
        "[file-transfer] upload finish conn={} path={} bytes={}",
        conn_id,
        target_path,
        session.expected
    );
    Ok(UploadFinishResult {
        target_path,
        relative_path: relative_upload_path(&final_name),
    })
}

/// 放弃上传：删临时文件与会话（连接归属校验）
pub fn upload_abort(conn_id: Option<u64>, upload_id: &str) -> Result<(), String> {
    let conn_id = conn_id.ok_or("file transfer is remote-only")?;
    let session = {
        let mut uploads = UPLOADS.lock();
        // 归属校验后移除（不存在的 id 静默成功——abort 幂等）
        match uploads.get_mut(upload_id) {
            Some(s) if s.conn_id == conn_id => uploads.remove(upload_id),
            _ => None,
        }
    };
    if let Some(s) = session {
        let _ = fs::remove_file(&s.part);
        log::info!(
            "[file-transfer] upload abort conn={} name={} written={}/{}",
            conn_id,
            s.file_name,
            s.written,
            s.expected
        );
    }
    Ok(())
}

// ==================== 下载 ====================

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DownloadBeginResult {
    pub download_id: String,
    pub file_name: String,
    pub size: u64,
    /// 源文件修改时间（Unix 毫秒；前端展示与调试用）
    pub mtime: Option<u64>,
}

/// 下载源校验：白名单（已知项目内）+ is_file + 限额。
/// 分块会话与一次性票据共用，保证两条下载路径的准入完全一致。
fn validate_download_source(
    path: &str,
    via_relay: bool,
) -> Result<(PathBuf, String, u64, Option<SystemTime>), String> {
    let p_c = Path::new(path)
        .canonicalize()
        .map_err(|_| format!("not found: {}", path))?;
    if !p_c.is_file() {
        return Err(format!("not a file: {}", path));
    }
    if !is_within_known_project(&p_c) {
        return Err("path outside authorized projects".into());
    }
    let md = fs::metadata(&p_c).map_err(|e| format!("cannot read source: {}", e))?;
    let size = md.len();
    let limit = limit_for(via_relay);
    if limit == 0 || size > limit {
        return Err(format!(
            "file too large: {} exceeds {} limit {}",
            fmt_mb(size),
            conn_kind(via_relay),
            fmt_mb(limit)
        ));
    }
    let file_name = p_c
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("download")
        .to_string();
    Ok((p_c, file_name, size, md.modified().ok()))
}

/// 源文件一致性校验（打开后按句柄 metadata 比对——消除 stat→open 的 TOCTOU）
pub(crate) fn source_unchanged(
    md: &fs::Metadata,
    size: u64,
    mtime: Option<SystemTime>,
) -> bool {
    md.len() == size && md.modified().ok() == mtime
}

/// 下载会话开启：白名单（已知项目内）+ is_file + 限额；会话固化
/// path/size/mtime（一致性锚点，后续 chunk 逐块比对）
pub fn download_begin(
    conn_id: Option<u64>,
    path: &str,
    via_relay: Option<bool>,
) -> Result<DownloadBeginResult, String> {
    sweep_stale();
    let conn_id = conn_id.ok_or("file transfer is remote-only")?;
    let via_relay = via_relay.unwrap_or(true);

    let (p_c, file_name, size, mtime) = validate_download_source(path, via_relay)?;

    let download_id = new_transfer_id();
    {
        let mut downloads = DOWNLOADS.lock();
        if downloads.len() >= MAX_ACTIVE_UPLOADS {
            return Err("too many concurrent transfers".into());
        }
        downloads.insert(
            download_id.clone(),
            DownloadSession {
                conn_id,
                path: p_c.clone(),
                file_name: file_name.clone(),
                size,
                mtime,
                via_relay,
                last_active: Instant::now(),
            },
        );
    }
    log::info!(
        "[file-transfer] download begin conn={} via={} path={} size={}",
        conn_id,
        conn_kind(via_relay),
        p_c.display(),
        size
    );
    Ok(DownloadBeginResult {
        download_id,
        file_name,
        size,
        mtime: system_time_ms(mtime),
    })
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DownloadChunkResult {
    pub data_b64: String,
    pub bytes: u64,
    pub eof: bool,
}

/// 拉取分块：逐块校验源文件 size/mtime 未变（变化即拒，防拼接新旧版本）。
/// eof 不立即清会话（最后一块 ACK 丢失重发需幂等），由 TTL sweep 回收。
pub fn download_chunk(
    conn_id: Option<u64>,
    download_id: &str,
    offset: u64,
) -> Result<DownloadChunkResult, String> {
    let conn_id = conn_id.ok_or("file transfer is remote-only")?;
    let (path, size, mtime) = {
        let mut downloads = DOWNLOADS.lock();
        let s = download_session_of(&mut downloads, conn_id, download_id)?;
        s.last_active = Instant::now();
        (s.path.clone(), s.size, s.mtime)
    };

    if offset > size {
        return Err(format!("offset beyond end of file (size {})", size));
    }

    // 先 open 再按句柄 metadata 校验（消除 stat→open 之间路径被替换的 TOCTOU）；
    // 源文件变化（size/mtime）→ 拒绝继续（会话保留，TTL 回收）
    let mut f = File::open(&path).map_err(|e| format!("cannot read source: {}", e))?;
    let md = f
        .metadata()
        .map_err(|e| format!("cannot read source: {}", e))?;
    if md.len() != size || md.modified().ok() != mtime {
        return Err("source file changed during download".into());
    }
    f.seek(SeekFrom::Start(offset))
        .map_err(|e| format!("cannot seek source: {}", e))?;
    let want = ((size - offset) as usize).min(CHUNK_SIZE);
    let mut buf = vec![0u8; want];
    let n = f
        .read(&mut buf)
        .map_err(|e| format!("source read failed: {}", e))?;
    buf.truncate(n);

    let eof = offset + n as u64 >= size;
    Ok(DownloadChunkResult {
        data_b64: base64::engine::general_purpose::STANDARD.encode(&buf),
        bytes: n as u64,
        eof,
    })
}

// ==================== 一次性下载票据 ====================

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DownloadTicketInfo {
    pub ticket: String,
    pub file_name: String,
    pub size: u64,
    /// 过期时间（Unix 毫秒）
    pub expires_at: u64,
}

/// 票据解析结果（直连流式与中继分块共用）
pub(crate) struct TicketAccess {
    pub path: PathBuf,
    pub file_name: String,
    pub size: u64,
    pub mtime: Option<SystemTime>,
}

/// 签发票据：准入校验与分块下载完全一致（validate_download_source）。
/// conn_id 仅用于日志，**不写入票据**——下载交给系统浏览器后 WS 会断开。
pub fn create_download_ticket(
    conn_id: Option<u64>,
    path: &str,
    via_relay: Option<bool>,
) -> Result<DownloadTicketInfo, String> {
    sweep_stale();
    let conn_id = conn_id.ok_or("file transfer is remote-only")?;
    let via_relay = via_relay.unwrap_or(true);
    let (p_c, file_name, size, mtime) = validate_download_source(path, via_relay)?;

    let ticket = new_transfer_id();
    {
        let mut tickets = TICKETS.lock();
        if tickets.len() >= MAX_TICKETS {
            return Err("too many pending downloads".into());
        }
        tickets.insert(
            ticket.clone(),
            DownloadTicket {
                path: p_c.clone(),
                file_name: file_name.clone(),
                size,
                mtime,
                created: Instant::now(),
                uses_left: TICKET_USES,
                last_active: Instant::now(),
            },
        );
    }
    log::info!(
        "[file-transfer] ticket issued conn={} via={} path={} size={}",
        conn_id,
        conn_kind(via_relay),
        p_c.display(),
        size
    );
    Ok(DownloadTicketInfo {
        ticket,
        file_name,
        size,
        expires_at: now_ms() + TICKET_TTL.as_millis() as u64,
    })
}

/// 票据是否有效（认证/探活）：只续期，不扣次数
pub(crate) fn touch_ticket(ticket: &str) -> bool {
    if !is_hex32(ticket) {
        return false;
    }
    let mut tickets = TICKETS.lock();
    let valid = match tickets.get(ticket) {
        Some(t) => t.created.elapsed() <= TICKET_TTL && t.uses_left > 0,
        None => false,
    };
    if !valid {
        tickets.remove(ticket);
        return false;
    }
    if let Some(t) = tickets.get_mut(ticket) {
        t.last_active = Instant::now();
    }
    true
}

/// 只看票据不扣次数（HTTP HEAD 探测：前端用它判断直链能力与错误）
pub(crate) fn peek_ticket(ticket: &str) -> Option<TicketAccess> {
    if !is_hex32(ticket) {
        return None;
    }
    let tickets = TICKETS.lock();
    let t = tickets.get(ticket)?;
    if t.created.elapsed() > TICKET_TTL || t.uses_left == 0 {
        return None;
    }
    Some(TicketAccess {
        path: t.path.clone(),
        file_name: t.file_name.clone(),
        size: t.size,
        mtime: t.mtime,
    })
}

/// 取票据并扣一次使用额度（HTTP GET / 数据块读取前调用）
pub(crate) fn take_ticket(ticket: &str) -> Result<TicketAccess, String> {
    if !is_hex32(ticket) {
        return Err("invalid download ticket".into());
    }
    let mut tickets = TICKETS.lock();
    let valid = match tickets.get(ticket) {
        Some(t) => t.created.elapsed() <= TICKET_TTL && t.uses_left > 0,
        None => false,
    };
    if !valid {
        tickets.remove(ticket);
        return Err("download ticket not found or expired".into());
    }
    let t = tickets.get_mut(ticket).expect("checked above");
    t.uses_left -= 1;
    t.last_active = Instant::now();
    Ok(TicketAccess {
        path: t.path.clone(),
        file_name: t.file_name.clone(),
        size: t.size,
        mtime: t.mtime,
    })
}

/// 票据分块读取（中继模式）：票据即权限，不要求 conn 绑定
pub fn download_ticket_chunk(ticket: &str, offset: u64) -> Result<DownloadChunkResult, String> {
    let access = take_ticket(ticket)?;
    if offset > access.size {
        return Err(format!("offset beyond end of file (size {})", access.size));
    }

    let mut f = File::open(&access.path).map_err(|e| format!("cannot read source: {}", e))?;
    let md = f
        .metadata()
        .map_err(|e| format!("cannot read source: {}", e))?;
    if !source_unchanged(&md, access.size, access.mtime) {
        return Err("source file changed during download".into());
    }
    f.seek(SeekFrom::Start(offset))
        .map_err(|e| format!("cannot seek source: {}", e))?;
    let want = ((access.size - offset) as usize).min(CHUNK_SIZE);
    let mut buf = vec![0u8; want];
    let n = f
        .read(&mut buf)
        .map_err(|e| format!("source read failed: {}", e))?;
    buf.truncate(n);

    let eof = offset + n as u64 >= access.size;
    if eof {
        // 中继下载完成：立即释放槽位（直连由 TicketGuard 在流结束时释放）
        TICKETS.lock().remove(ticket);
    }
    Ok(DownloadChunkResult {
        data_b64: base64::engine::general_purpose::STANDARD.encode(&buf),
        bytes: n as u64,
        eof,
    })
}

/// 释放票据（直连流结束/中断时由 TicketGuard 调用；幂等）
pub(crate) fn remove_ticket(ticket: &str) {
    TICKETS.lock().remove(ticket);
}

// ==================== 列文件 ====================

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FileEntryLite {
    pub name: String,
    pub path: String,
    pub is_dir: bool,
    pub size: Option<u64>,
    pub modified: Option<u64>,
}

/// 列目录（含文件，供下载浏览器）：范围锁定已知项目内；
/// 过滤上传临时文件（.upload-*.part）；目录在前、名称不区分大小写排序
pub fn list(path: &str) -> Result<Vec<FileEntryLite>, String> {
    sweep_stale();
    let p_c = Path::new(path)
        .canonicalize()
        .map_err(|_| format!("not found: {}", path))?;
    if !p_c.is_dir() {
        return Err(format!("not a directory: {}", path));
    }
    if !is_within_known_project(&p_c) {
        return Err("path outside authorized projects".into());
    }

    let mut out = Vec::new();
    let entries =
        fs::read_dir(&p_c).map_err(|e| format!("cannot read {}: {}", p_c.display(), e))?;
    for entry in entries.flatten() {
        let name = entry.file_name().to_string_lossy().into_owned();
        // 隐藏上传临时文件
        if name.starts_with(PART_PREFIX) && name.ends_with(PART_SUFFIX) {
            continue;
        }
        // 剥离 canonicalize 的 verbatim 前缀（否则前端无法与项目根比较、
        // 且路径展示带 `\\?\`）
        let full = display_path(&entry.path());
        let Ok(md) = entry.metadata() else { continue };
        let is_dir = md.is_dir();
        out.push(FileEntryLite {
            name,
            path: full,
            is_dir,
            size: if is_dir { None } else { Some(md.len()) },
            modified: mtime_ms(&md),
        });
    }
    out.sort_by(|a, b| {
        b.is_dir
            .cmp(&a.is_dir)
            .then_with(|| a.name.to_lowercase().cmp(&b.name.to_lowercase()))
    });
    Ok(out)
}

// ==================== 清理（三层：惰性 / 定时 / 启动） ====================

/// TTL 惰性清理：任一 file_ 命令入口调用；定时任务（lib.rs setup spawn）同调
pub fn sweep_stale() {
    let now = Instant::now();
    let mut expired_parts = Vec::new();
    UPLOADS.lock().retain(|_, s| {
        if now.duration_since(s.last_active) > SESSION_TTL {
            expired_parts.push(s.part.clone());
            false
        } else {
            true
        }
    });
    for p in expired_parts {
        let _ = fs::remove_file(p);
    }
    DOWNLOADS.lock().retain(|_, s| now.duration_since(s.last_active) <= SESSION_TTL);
    // 票据按签发时刻绝对过期（滑动续期会让泄露链接长期有效）；
    // 已被使用且闲置 1 分钟的票据视为下载已结束/放弃，立即释放槽位
    TICKETS.lock().retain(|_, t| {
        let expired = now.duration_since(t.created) > TICKET_TTL;
        let consumed_idle = t.uses_left < TICKET_USES
            && now.duration_since(t.last_active) > Duration::from_secs(60);
        !expired && !consumed_idle
    });
}

/// 测试辅助：把全部会话/票据标记为过期（触发 TTL sweep 路径）
#[cfg(test)]
pub(crate) fn test_expire_sessions() {
    let stale = Instant::now()
        .checked_sub(SESSION_TTL + Duration::from_secs(1))
        .unwrap_or_else(Instant::now);
    for s in UPLOADS.lock().values_mut() {
        s.last_active = stale;
    }
    for s in DOWNLOADS.lock().values_mut() {
        s.last_active = stale;
    }
    for t in TICKETS.lock().values_mut() {
        t.created = stale;
    }
}

/// 连接断开时清理其传输会话（防僵死会话占满配额；由 remote_state remove_conn 调用）。
/// **不清理 TICKETS**：票据用于把下载交给系统浏览器/下载器，此时 WS 必然断开，
/// 随连接回收会让直链立即失效；票据只按 TTL/次数回收。
pub fn cleanup_conn(conn_id: u64) {
    let expired_parts: Vec<PathBuf> = {
        let mut uploads = UPLOADS.lock();
        let mut parts = Vec::new();
        uploads.retain(|_, s| {
            if s.conn_id == conn_id {
                parts.push(s.part.clone());
                false
            } else {
                true
            }
        });
        parts
    };
    for p in expired_parts {
        let _ = fs::remove_file(p);
    }
    DOWNLOADS.lock().retain(|_, s| s.conn_id != conn_id);
}

/// 启动清理：删已知项目 `.cc-box-files` 下**过期**的 `.upload-*.part`
/// （崩溃遗留；按 mtime 阈值过滤——多实例并行时另一实例的在途传输不会被误删）。
/// 目录 canonicalize 后必须仍在项目根内（防项目内 `.cc-box-files` 为
/// junction/符号链接时删除项目外文件）
pub fn startup_cleanup() {
    const STALE_AFTER: Duration = Duration::from_secs(600);
    let projects = known_project_paths();
    let now = SystemTime::now();
    let mut removed = 0usize;
    for proj in projects {
        let Ok(root_c) = Path::new(&proj).canonicalize() else {
            continue;
        };
        let dir = root_c.join(UPLOAD_DIR_NAME);
        let Ok(dir_c) = dir.canonicalize() else { continue };
        if !dir_c.starts_with(&root_c) {
            // 目录被链接到项目外：不在启动时对其做任何删除
            log::warn!(
                "[file-transfer] skip cleanup: {} resolves outside project",
                dir.display()
            );
            continue;
        }
        let Ok(entries) = fs::read_dir(&dir_c) else { continue };
        for entry in entries.flatten() {
            let name = entry.file_name().to_string_lossy().into_owned();
            if !(name.starts_with(PART_PREFIX) && name.ends_with(PART_SUFFIX)) {
                continue;
            }
            let stale = entry
                .metadata()
                .and_then(|m| m.modified())
                .ok()
                .and_then(|t| now.duration_since(t).ok())
                .map(|age| age > STALE_AFTER)
                .unwrap_or(true); // 取不到 mtime：按遗留处理
            if stale && fs::remove_file(entry.path()).is_ok() {
                removed += 1;
            }
        }
    }
    if removed > 0 {
        log::info!("[file-transfer] startup cleanup removed {} stale temp files", removed);
    }
}
