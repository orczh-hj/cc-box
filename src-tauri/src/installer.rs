//! 依赖安装模块
//!
//! 从 OSS 下载 Claude CLI 和 Git 便携版，放置到用户目录并添加到 PATH。
//!
//! 安装路径：
//! - Claude: %USERPROFILE%\.local\bin\claude.exe (Windows) / ~/.local/bin/claude (Unix)
//! - Git: %LOCALAPPDATA%\PortableGit\bin\bash.exe

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::io::{self, Read, Write};
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, LazyLock, Mutex};
#[cfg(not(target_os = "windows"))]
use std::process::Command;
use tauri::{AppHandle, Emitter};

/// OSS 配置
const OSS_BASE_URL: &str = "https://cc-box.oss-cn-beijing.aliyuncs.com";

/// 构建直连 OSS 的 HTTP 客户端。
///
/// OSS 为国内资源，必须绕过所有代理（系统代理 + 环境变量）：
/// reqwest 默认读系统代理，当 Clash Verge 等代理工具退出后系统代理设置
/// 残留指向已死端口时，请求会以「unexpected EOF during handshake」失败。
fn oss_client(timeout: std::time::Duration) -> reqwest::Result<reqwest::blocking::Client> {
    reqwest::blocking::Client::builder()
        .timeout(timeout)
        .no_proxy()
        .build()
}

/// 正在进行的 Claude 历史版本下载：version → 取消标志
///
/// key 为 Claude CLI version（如 "2.1.177"），用于精确取消某次下载。
/// 同一版本同时只允许一个活动下载；新发起会复用现有 flag。
static ACTIVE_CLAUDE_DOWNLOADS: LazyLock<Mutex<HashMap<String, Arc<AtomicBool>>>> =
    LazyLock::new(|| Mutex::new(HashMap::new()));

/// 正在进行的自动安装（首次启动 Auto Install）：单一取消标志
///
/// 同一时间最多一次自动安装（Auto Install 按钮触发），不存在并发，用单一 Option 即可。
/// `cancel_auto_install` 命令把它置为 true，下载循环检测后立即中断。
static ACTIVE_AUTO_INSTALL: LazyLock<Mutex<Option<Arc<AtomicBool>>>> =
    LazyLock::new(|| Mutex::new(None));

/// 安装路径
#[cfg(target_os = "windows")]
fn get_install_dirs() -> (PathBuf, PathBuf) {
    let user_profile = std::env::var("USERPROFILE")
        .unwrap_or_else(|_| format!("C:\\Users\\{}", std::env::var("USERNAME").unwrap_or_default()));
    let claude_dir = PathBuf::from(&user_profile).join(".local").join("bin");

    let local_app_data = std::env::var("LOCALAPPDATA")
        .unwrap_or_else(|_| format!("{}\\AppData\\Local", user_profile));
    let git_dir = PathBuf::from(&local_app_data).join("PortableGit");
    (claude_dir, git_dir)
}

#[cfg(not(target_os = "windows"))]
fn get_install_dirs() -> (PathBuf, PathBuf) {
    let home = std::env::var("HOME").unwrap_or_else(|_| "/tmp".to_string());
    let claude_dir = PathBuf::from(&home).join(".local").join("bin");
    let git_dir = PathBuf::from(&home).join("PortableGit"); // macOS/Linux 不需要 Git 便携版
    (claude_dir, git_dir)
}

/// Latest.json 结构
#[derive(Debug, Deserialize, Serialize)]
pub struct ClaudeLatestInfo {
    pub version: String,
    pub release_date: String,
    pub platforms: std::collections::HashMap<String, PlatformInfo>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PlatformInfo {
    pub url: String,
    pub checksum: String,
    pub size: u64,
}

/// versions.json 中的单个版本条目
#[derive(Debug, Deserialize, Serialize)]
pub struct ClaudeVersionEntry {
    pub version: String,
    pub release_date: String,
    pub platforms: std::collections::HashMap<String, PlatformInfo>,
}

/// versions.json 顶层结构
#[derive(Debug, Deserialize, Serialize)]
pub struct ClaudeVersions {
    pub latest: String,
    pub updated_at: String,
    pub versions: Vec<ClaudeVersionEntry>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct GitLatestInfo {
    pub version: String,
    pub release_date: String,
    pub file: String,
    pub url: String,
    pub size: u64,
}

/// 最新版本信息
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LatestVersions {
    pub claude: ClaudeLatestInfo,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub git: Option<GitLatestInfo>,
}

/// 下载进度事件
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DownloadProgress {
    pub item: String,       // "claude" | "git"
    pub stage: String,      // "fetching" | "downloading" | "extracting" | "placing" | "done" | "error"
    pub progress: u8,       // 0-100
    pub message: String,
}

// ============================================
// HTTP 下载
// ============================================

/// 从 URL 下载文件（无代理，直接访问 OSS）
///
/// `cancel_flag` 非 None 时，每次循环都会检查；为 true 时立即停止下载，
/// 删除半成品文件，返回 `io::ErrorKind::Interrupted` 错误。
fn download_file(
    url: &str,
    output_path: &Path,
    app: &AppHandle,
    item: &str,
    cancel_flag: Option<&AtomicBool>,
) -> io::Result<()> {
    log::info!("[Installer] Downloading {} from {}", item, url);

    // 发送进度事件
    emit_progress(app, item, "downloading", 0, "开始下载...");

    // 显式超时，避免网络挂起时前端 invoke 永久 pending
    let client = oss_client(std::time::Duration::from_secs(600))
        .map_err(|e| {
            let msg = format!("构建 HTTP 客户端失败：{}", e);
            emit_progress(app, item, "error", 0, msg.as_str());
            io::Error::new(io::ErrorKind::Other, msg)
        })?;

    let mut response = client.get(url).send().map_err(|e| {
        let msg = format!("请求失败：{}", e);
        emit_progress(app, item, "error", 0, msg.as_str());
        io::Error::new(io::ErrorKind::Other, msg)
    })?;

    if !response.status().is_success() {
        let msg = format!("HTTP 错误：{}", response.status());
        emit_progress(app, item, "error", 0, msg.as_str());
        return Err(io::Error::new(io::ErrorKind::Other, msg));
    }

    let total_size = response.content_length().unwrap_or(0);
    let mut file = fs::File::create(output_path)?;
    let mut downloaded: u64 = 0;
    let mut buffer = vec![0u8; 8192];

    loop {
        // 检查取消标志
        if let Some(flag) = cancel_flag {
            if flag.load(Ordering::SeqCst) {
                // 同步刷新并关闭文件后再删除，避免 Windows 上文件占用
                drop(file);
                let _ = fs::remove_file(output_path);
                emit_progress(app, item, "cancelled", 0, "已取消");
                return Err(io::Error::new(io::ErrorKind::Interrupted, "Download cancelled"));
            }
        }

        let bytes_read = response.read(&mut buffer)?;
        if bytes_read == 0 {
            break;
        }
        file.write_all(&buffer[..bytes_read])?;
        downloaded += bytes_read as u64;

        if total_size > 0 {
            let percent = ((downloaded as f64 / total_size as f64) * 100.0) as u8;
            emit_progress(
                app,
                item,
                "downloading",
                percent,
                format!("下载中 {}%", percent),
            );
        }
    }

    file.flush()?;
    emit_progress(app, item, "downloading", 100, "下载完成");
    log::info!("[Installer] Downloaded {} bytes", downloaded);

    Ok(())
}

/// 发送进度事件
fn emit_progress(app: &AppHandle, item: &str, stage: &str, progress: u8, message: impl Into<String>) {
    let progress_event = DownloadProgress {
        item: item.to_string(),
        stage: stage.to_string(),
        progress,
        message: message.into(),
    };
    if let Err(e) = app.emit("download-progress", progress_event.clone()) {
        log::warn!("[Installer] Failed to emit progress: {}", e);
    }
}

// ============================================
// Claude CLI 安装
// ============================================

/// 获取 Claude 最新版本信息
fn fetch_claude_latest() -> io::Result<ClaudeLatestInfo> {
    let url = format!("{}/deps/claude/latest.json", OSS_BASE_URL);
    log::info!("[Installer] Fetching Claude latest.json from {}", url);

    let client = oss_client(std::time::Duration::from_secs(15))
        .map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))?;
    let response = client
        .get(&url)
        .send()
        .map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))?;

    if !response.status().is_success() {
        return Err(io::Error::new(
            io::ErrorKind::Other,
            format!("Failed to fetch latest.json: {}", response.status()),
        ));
    }

    let info: ClaudeLatestInfo = response
        .json()
        .map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))?;

    log::info!("[Installer] Claude latest version: {}", info.version);
    Ok(info)
}

/// 获取当前平台标识
fn get_current_platform() -> String {
    crate::platform::get_platform_id()
}

/// 下载并安装 Claude CLI
#[tauri::command]
pub async fn download_and_install_claude(app: AppHandle) -> Result<(), String> {
    log::info!("[Installer] Starting Claude CLI installation");

    // 注册取消标志到全局 AUTO INSTALL map
    let cancel_flag = Arc::new(AtomicBool::new(false));
    {
        let mut map = ACTIVE_AUTO_INSTALL.lock().unwrap();
        *map = Some(cancel_flag.clone());
    }

    // 退出时清理标志
    let cleanup = || {
        let mut map = ACTIVE_AUTO_INSTALL.lock().unwrap();
        *map = None;
    };

    emit_progress(&app, "claude", "fetching", 0, "获取版本信息...");

    // 获取最新版本信息
    let latest = match tokio::task::spawn_blocking(fetch_claude_latest).await {
        Ok(Ok(info)) => info,
        Ok(Err(e)) => {
            let msg = format!("获取版本信息失败：{}", e);
            emit_progress(&app, "claude", "error", 0, msg.as_str());
            cleanup();
            return Err(msg);
        }
        Err(e) => {
            let msg = format!("任务调度失败：{}", e);
            emit_progress(&app, "claude", "error", 0, msg.as_str());
            cleanup();
            return Err(msg);
        }
    };

    // 检查取消
    if cancel_flag.load(Ordering::SeqCst) {
        emit_progress(&app, "claude", "cancelled", 0, "已取消");
        cleanup();
        return Err("cancelled".to_string());
    }

    emit_progress(&app, "claude", "fetching", 100, format!("版本: {}", latest.version));

    // 获取平台信息
    let platform = get_current_platform();
    let platform_info = match latest.platforms.get(&platform) {
        Some(info) => info,
        None => {
            let msg = format!("当前平台 {} 暂不支持", platform);
            emit_progress(&app, "claude", "error", 0, msg.as_str());
            cleanup();
            return Err(msg);
        }
    };

    let download_url = format!("{}/{}", OSS_BASE_URL, platform_info.url);
    let filename = if platform.starts_with("win32") {
        "claude.exe"
    } else {
        "claude"
    };

    // 创建安装目录
    let (claude_dir, _) = get_install_dirs();
    if let Err(e) = fs::create_dir_all(&claude_dir) {
        let msg = format!("创建安装目录失败：{}", e);
        emit_progress(&app, "claude", "error", 0, msg.as_str());
        cleanup();
        return Err(msg);
    }

    let claude_path = claude_dir.join(filename);

    // 下载文件（带取消标志）
    emit_progress(&app, "claude", "downloading", 0, "开始下载...");
    let claude_path_clone = claude_path.clone();
    let app_clone = app.clone();
    let cancel_for_task = cancel_flag.clone();
    let download_result = tokio::task::spawn_blocking(move || {
        download_file(
            &download_url,
            &claude_path_clone,
            &app_clone,
            "claude",
            Some(&cancel_for_task),
        )
    })
    .await
    .map_err(|e| {
        let msg = format!("任务调度失败：{}", e);
        emit_progress(&app, "claude", "error", 0, msg.as_str());
        msg
    })?;

    if let Err(e) = download_result {
        cleanup();
        if e.kind() == io::ErrorKind::Interrupted {
            return Err("cancelled".to_string());
        }
        // download_file 已经 emit 了 error 事件和具体原因
        return Err(e.to_string());
    }

    // 设置权限（Unix）
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        if let Err(e) = fs::set_permissions(&claude_path, fs::Permissions::from_mode(0o755)) {
            let msg = format!("设置可执行权限失败：{}", e);
            emit_progress(&app, "claude", "error", 0, msg.as_str());
            cleanup();
            return Err(msg);
        }
    }

    // 添加到 PATH（进程级 + 用户级持久化）
    emit_progress(&app, "claude", "placing", 80, "添加到 PATH...");
    add_to_path(&claude_dir);
    add_to_user_path_permanent(&claude_dir);

    // 保存路径到配置文件
    save_install_path_to_config("claudePath", &claude_path);

    emit_progress(&app, "claude", "done", 100, "安装完成");
    cleanup();

    log::info!("[Installer] Claude CLI installed to {}", claude_path.display());
    Ok(())
}

/// 取消正在进行的自动安装（首次启动 Auto Install 触发的）
///
/// 返回 true 表示找到了活动安装并已标记取消；false 表示当前没有活动安装。
#[tauri::command]
pub async fn cancel_auto_install() -> Result<bool, String> {
    let flag = {
        let map = ACTIVE_AUTO_INSTALL.lock().unwrap();
        map.as_ref().cloned()
    };
    if let Some(f) = flag {
        f.store(true, Ordering::SeqCst);
        log::info!("[Installer] Cancel requested for auto install");
        Ok(true)
    } else {
        log::info!("[Installer] Cancel requested but no active auto install");
        Ok(false)
    }
}

// ============================================
// Git 便携版安装（Windows）
// ============================================

#[cfg(target_os = "windows")]
/// 获取 Git 最新版本信息
fn fetch_git_latest() -> io::Result<GitLatestInfo> {
    let url = format!("{}/deps/git/latest.json", OSS_BASE_URL);
    log::info!("[Installer] Fetching Git latest.json from {}", url);

    let client = oss_client(std::time::Duration::from_secs(15))
        .map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))?;
    let response = client
        .get(&url)
        .send()
        .map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))?;

    if !response.status().is_success() {
        return Err(io::Error::new(
            io::ErrorKind::Other,
            format!("Failed to fetch Git latest.json: {}", response.status()),
        ));
    }

    let info: GitLatestInfo = response
        .json()
        .map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))?;

    log::info!("[Installer] Git latest version: {}", info.version);
    Ok(info)
}

#[cfg(target_os = "windows")]
/// 解压 PortableGit.7z.exe
fn extract_portable_git(archive_path: &Path, target_dir: &Path, app: &AppHandle) -> io::Result<()> {
    log::info!("[Installer] Extracting PortableGit to {}", target_dir.display());

    emit_progress(app, "git", "extracting", 0, "开始解压...");

    // PortableGit.7z.exe 自解压后内容直接放在输出目录
    // 所以需要：解压到临时目录 → 移动到目标目录

    let local_app_data = std::env::var("LOCALAPPDATA")
        .unwrap_or_else(|_| format!("{}\\AppData\\Local", std::env::var("USERPROFILE").unwrap_or_default()));

    // 创建临时解压目录
    let temp_extract_dir = PathBuf::from(&local_app_data).join("PortableGit-temp");
    if temp_extract_dir.exists() {
        fs::remove_dir_all(&temp_extract_dir)?;
    }
    fs::create_dir_all(&temp_extract_dir)?;

    // 解压到临时目录
    // PortableGit.7z.exe 参数：-y (自动确认), -o"path" (输出目录)
    let temp_extract_str = temp_extract_dir.to_string_lossy().to_string();

    let mut cmd = crate::platform::new_command(&archive_path.to_string_lossy());
    cmd.args(["-y", &format!("-o{}", temp_extract_str)]);
    let status = cmd
        .spawn()
        .map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))?
        .wait()
        .map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))?;

    if !status.success() {
        // 清理临时目录
        fs::remove_dir_all(&temp_extract_dir).ok();
        return Err(io::Error::new(
            io::ErrorKind::Other,
            format!("Extract failed with status: {}", status),
        ));
    }

    emit_progress(app, "git", "extracting", 50, "移动文件...");

    // 如果目标目录已存在，先删除
    if target_dir.exists() {
        log::info!("[Installer] Removing existing PortableGit directory");
        fs::remove_dir_all(target_dir)?;
    }

    // 移动临时目录到目标目录
    fs::rename(&temp_extract_dir, target_dir)?;
    log::info!("[Installer] PortableGit moved to {}", target_dir.display());

    // 验证解压结果
    let bash_path = target_dir.join("bin").join("bash.exe");
    if !bash_path.exists() {
        // 某些版本 bash.exe 在 usr/bin
        let alt_bash_path = target_dir.join("usr").join("bin").join("bash.exe");
        if alt_bash_path.exists() {
            log::info!("[Installer] bash.exe found in usr/bin");
        } else {
            return Err(io::Error::new(
                io::ErrorKind::Other,
                format!("bash.exe not found after extraction"),
            ));
        }
    }

    emit_progress(app, "git", "extracting", 100, "解压完成");
    log::info!("[Installer] PortableGit extracted successfully");
    Ok(())
}

#[cfg(target_os = "windows")]
/// 下载并安装 Git 便携版
#[tauri::command]
pub async fn download_and_install_git(app: AppHandle) -> Result<(), String> {
    log::info!("[Installer] Starting Git portable installation");

    // 注册取消标志到全局 AUTO INSTALL map
    let cancel_flag = Arc::new(AtomicBool::new(false));
    {
        let mut map = ACTIVE_AUTO_INSTALL.lock().unwrap();
        *map = Some(cancel_flag.clone());
    }

    let cleanup = || {
        let mut map = ACTIVE_AUTO_INSTALL.lock().unwrap();
        *map = None;
    };

    emit_progress(&app, "git", "fetching", 0, "获取版本信息...");

    // 获取最新版本信息
    let latest = match tokio::task::spawn_blocking(fetch_git_latest).await {
        Ok(Ok(info)) => info,
        Ok(Err(e)) => {
            let msg = format!("获取 Git 版本信息失败：{}", e);
            emit_progress(&app, "git", "error", 0, msg.as_str());
            cleanup();
            return Err(msg);
        }
        Err(e) => {
            let msg = format!("任务调度失败：{}", e);
            emit_progress(&app, "git", "error", 0, msg.as_str());
            cleanup();
            return Err(msg);
        }
    };

    // 检查取消
    if cancel_flag.load(Ordering::SeqCst) {
        emit_progress(&app, "git", "cancelled", 0, "已取消");
        cleanup();
        return Err("cancelled".to_string());
    }

    emit_progress(&app, "git", "fetching", 100, format!("版本: {}", latest.version));

    let download_url = format!("{}/{}", OSS_BASE_URL, latest.url);

    // 创建安装目录
    let (_, git_dir) = get_install_dirs();
    let _parent_dir = git_dir.parent().unwrap();

    // 下载到临时目录
    let temp_dir = std::env::temp_dir();
    let archive_path = temp_dir.join(&latest.file);

    emit_progress(&app, "git", "downloading", 0, "开始下载...");
    let archive_path_clone = archive_path.clone();
    let app_clone = app.clone();
    let cancel_for_task = cancel_flag.clone();
    let download_result = tokio::task::spawn_blocking(move || {
        download_file(
            &download_url,
            &archive_path_clone,
            &app_clone,
            "git",
            Some(&cancel_for_task),
        )
    })
    .await
    .map_err(|e| {
        let msg = format!("任务调度失败：{}", e);
        emit_progress(&app, "git", "error", 0, msg.as_str());
        msg
    })?;

    if let Err(e) = download_result {
        cleanup();
        if e.kind() == io::ErrorKind::Interrupted {
            return Err("cancelled".to_string());
        }
        return Err(e.to_string());
    }

    // 解压
    emit_progress(&app, "git", "extracting", 0, "开始解压...");
    let archive_path_clone = archive_path.clone();
    let git_dir_clone = git_dir.clone();
    let app_clone = app.clone();
    let extract_result = tokio::task::spawn_blocking(move || {
        extract_portable_git(&archive_path_clone, &git_dir_clone, &app_clone)
    })
    .await
    .map_err(|e| {
        let msg = format!("任务调度失败：{}", e);
        emit_progress(&app, "git", "error", 0, msg.as_str());
        msg
    })?;

    if let Err(e) = extract_result {
        let msg = format!("解压 PortableGit 失败：{}", e);
        emit_progress(&app, "git", "error", 0, msg.as_str());
        cleanup();
        return Err(msg);
    }

    // 添加到 PATH（进程级 + 用户级持久化）
    let git_bin_dir = git_dir.join("bin");
    emit_progress(&app, "git", "placing", 80, "添加到 PATH...");
    add_to_path(&git_bin_dir);
    add_to_user_path_permanent(&git_bin_dir);

    // 保存 bash.exe 路径到配置文件
    let bash_path = git_bin_dir.join("bash.exe");
    save_install_path_to_config("gitBashPath", &bash_path);

    // 清理临时文件
    fs::remove_file(&archive_path).ok();

    emit_progress(&app, "git", "done", 100, "安装完成");
    cleanup();

    log::info!("[Installer] Git portable installed to {}", git_dir.display());
    Ok(())
}

// ============================================
// PATH 管理
// ============================================

/// 保存安装路径到配置文件
fn save_install_path_to_config(key: &str, path: &Path) {
    let path_str = path.to_string_lossy().to_string();

    // 构建更新 JSON
    let mut updates = serde_json::Map::new();
    updates.insert(key.to_string(), serde_json::Value::String(path_str));

    // 调用 store 模块的更新函数
    match crate::store::update_app_config(serde_json::Value::Object(updates)) {
        Ok(()) => log::info!("[Installer] Saved {} to config: {}", key, path.display()),
        Err(e) => log::warn!("[Installer] Failed to save {} to config: {}", key, e),
    }
}

/// 清理 PATH 字符串并将目录添加到开头（纯函数）
///
/// 移除已有条目（大小写不敏感）后添加到开头，确保最高优先级。
pub(crate) fn clean_and_prepend_path(path: &str, dir: &str, sep: char) -> String {
    let dir_lower = dir.to_lowercase();
    let entries: Vec<&str> = path
        .split(sep)
        .filter(|e| e.trim().to_lowercase() != dir_lower)
        .collect();
    let clean_path = entries.join(&sep.to_string());

    if clean_path.is_empty() {
        dir.to_string()
    } else {
        format!("{}{}{}", dir, sep, clean_path)
    }
}

/// 清理 rc 文件内容并将 export 行追加到末尾（纯函数）
///
/// 移除包含任一 marker 的旧行（大小写不敏感），追加新的 export 行。
/// markers 用于同时匹配绝对路径和 $HOME 相对路径两种格式。
pub(crate) fn clean_rc_content(content: &str, markers: &[&str], export_line: &str) -> String {
    let markers_lower: Vec<String> = markers.iter().map(|m| m.to_lowercase()).collect();
    let filtered: Vec<&str> = content
        .lines()
        .filter(|line| {
            let line_lower = line.to_lowercase();
            !markers_lower.iter().any(|m| line_lower.contains(m.as_str()))
        })
        .collect();
    let mut result = filtered.join("\n");
    if !result.is_empty() && !result.ends_with('\n') {
        result.push('\n');
    }
    result.push_str(&format!("{}\n", export_line));
    result
}

/// 添加目录到进程 PATH（立即生效）
///
/// 始终将目录移到 PATH 最前面，确保最高优先级。
fn add_to_path(dir: &Path) {
    let dir_str = dir.to_string_lossy().to_string();
    let current_path = std::env::var("PATH").unwrap_or_default();
    let sep = if cfg!(windows) { ';' } else { ':' };

    let new_path = clean_and_prepend_path(&current_path, &dir_str, sep);
    std::env::set_var("PATH", &new_path);
    log::info!("[Installer] Ensured {} at PATH beginning", dir_str);
}

/// 添加目录到用户环境变量 PATH（持久化）
#[cfg(target_os = "windows")]
fn add_to_user_path_permanent(dir: &Path) {
    let dir_str = dir.to_string_lossy().to_string();

    // 获取当前用户 PATH
    let mut cmd = crate::platform::new_command("powershell");
    cmd.args([
        "-Command",
        "[Environment]::GetEnvironmentVariable('PATH', 'User')",
    ]);
    let output = cmd.output();

    let current_user_path = match output {
        Ok(o) => String::from_utf8_lossy(&o.stdout).trim().to_string(),
        Err(e) => {
            log::warn!("[Installer] Failed to get user PATH: {}", e);
            return;
        }
    };

    // 移除已有条目后添加到开头，确保最高优先级
    let new_user_path = clean_and_prepend_path(&current_user_path, &dir_str, ';');

    // 使用 PowerShell 设置用户 PATH
    let mut set_cmd = crate::platform::new_command("powershell");
    set_cmd.args([
        "-Command",
        &format!(
            "[Environment]::SetEnvironmentVariable('PATH', '{}', 'User')",
            new_user_path
        ),
    ]);
    let set_result = set_cmd.output();

    match set_result {
        Ok(o) => {
            if o.status.success() {
                log::info!("[Installer] Permanently added {} to user PATH", dir_str);
            } else {
                log::warn!(
                    "[Installer] Failed to set user PATH: {}",
                    String::from_utf8_lossy(&o.stderr)
                );
            }
        }
        Err(e) => {
            log::warn!("[Installer] Failed to set user PATH: {}", e);
        }
    }
}

#[cfg(not(target_os = "windows"))]
fn add_to_user_path_permanent(dir: &Path) {
    let home = std::env::var("HOME").unwrap_or_else(|_| "/tmp".to_string());
    let dir_str = dir.to_string_lossy();

    // 使用 $HOME 相对路径，避免硬编码绝对路径
    let rel_path = if dir_str.starts_with(&home) {
        format!("$HOME{}", &dir_str[home.len()..])
    } else {
        dir_str.to_string()
    };
    let export_line = format!("export PATH=\"{}:$PATH\"", rel_path);

    // macOS 默认 zsh → 写入 ~/.zshenv（所有 zsh shell 都加载）
    // Linux 默认 bash → 写入 ~/.bashrc（交互式 shell 加载）
    let shell = std::env::var("SHELL").unwrap_or_default();
    let rc_file = if shell.contains("zsh") {
        PathBuf::from(&home).join(".zshenv")
    } else {
        PathBuf::from(&home).join(".bashrc")
    };

    // 移除已有的该路径相关行，然后追加新行确保最高优先级
    // 同时匹配绝对路径和 $HOME 相对路径格式
    let existing = fs::read_to_string(&rc_file).unwrap_or_default();
    let marker_abs = rel_path.replace("$HOME", &home);
    let markers = [marker_abs.as_str(), rel_path.as_str()];
    let new_content = clean_rc_content(&existing, &markers, &export_line);

    match fs::write(&rc_file, &new_content) {
        Ok(()) => log::info!("[Installer] Ensured {} at PATH beginning in {}", rel_path, rc_file.display()),
        Err(e) => log::warn!("[Installer] Failed to write to {}: {}", rc_file.display(), e),
    }
}

/// 获取最新版本信息（用于前端显示）
#[tauri::command]
pub async fn get_latest_versions() -> Result<LatestVersions, String> {
    log::info!("[Installer] Fetching latest versions info");

    // 获取 Claude 信息
    let claude = tokio::task::spawn_blocking(|| fetch_claude_latest())
        .await
        .map_err(|e| e.to_string())?
        .map_err(|e| e.to_string())?;

    // 获取 Git 信息（Windows only）
    #[cfg(target_os = "windows")]
    let git = Some(
        tokio::task::spawn_blocking(|| fetch_git_latest())
            .await
            .map_err(|e| e.to_string())?
            .map_err(|e| e.to_string())?,
    );

    #[cfg(not(target_os = "windows"))]
    let git: Option<GitLatestInfo> = None;

    Ok(LatestVersions { claude, git })
}

// ============================================
// Claude CLI 更新检查
// ============================================

/// Claude CLI 更新检查结果
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ClaudeCliUpdateInfo {
    pub installed_version: Option<String>,
    pub latest_version: String,
    pub has_update: bool,
    pub not_installed: bool,
}

/// 获取已安装的 Claude CLI 版本号（纯本地，无 HTTP）
pub(crate) fn read_local_claude_version() -> Option<String> {
    let exe_name = if cfg!(target_os = "windows") {
        "claude.exe"
    } else {
        "claude"
    };

    // 1. 优先使用配置路径
    if let Ok(config) = crate::store::get_app_config() {
        if let Some(ref path) = config.claude_path {
            if Path::new(path).exists() {
                if let Some(v) = run_version_command(Path::new(path)) {
                    return Some(v);
                }
            }
        }
    }

    // 2. 检查标准安装目录
    let (claude_dir, _) = get_install_dirs();
    let claude_path = claude_dir.join(exe_name);
    if claude_path.exists() {
        if let Some(v) = run_version_command(&claude_path) {
            return Some(v);
        }
    }

    // 3. 检查 PATH 中的 claude
    run_version_command(Path::new("claude"))
}

/// 执行 claude --version 并解析版本号
#[cfg(target_os = "windows")]
fn run_version_command(program: &Path) -> Option<String> {
    let program_str = program.to_string_lossy();
    let mut cmd = crate::platform::new_command("cmd");
    cmd.args(["/C", &*program_str, "--version"]);
    let output = cmd.output().ok()?;
    if !output.status.success() {
        return None;
    }
    let stdout = crate::platform::decode_output(&output.stdout);
    parse_version_output(&stdout)
}

#[cfg(not(target_os = "windows"))]
fn run_version_command(program: &Path) -> Option<String> {
    let output = Command::new(program)
        .arg("--version")
        .output()
        .ok()?;
    if !output.status.success() {
        return None;
    }
    let stdout = crate::platform::decode_output(&output.stdout);
    parse_version_output(&stdout)
}

/// 解析版本输出，提取 x.y.z 格式的版本号
pub(crate) fn parse_version_output(output: &str) -> Option<String> {
    for part in output.split_whitespace() {
        if let Some(v) = extract_semver(part) {
            return Some(v);
        }
    }
    None
}

/// 从字符串中提取 semver 版本号 (x.y.z)
pub(crate) fn extract_semver(s: &str) -> Option<String> {
    let bytes = s.as_bytes();
    let len = bytes.len();
    let mut i = 0;
    while i < len {
        if bytes[i].is_ascii_digit() {
            let start = i;
            let mut dot_count = 0u8;
            while i < len && (bytes[i].is_ascii_digit() || bytes[i] == b'.') {
                if bytes[i] == b'.' {
                    dot_count += 1;
                    // 取到第3段时停止（2个点 = x.y.z）
                    if dot_count == 3 {
                        break;
                    }
                }
                i += 1;
            }
            if dot_count >= 2 {
                // 回退到最后一个数字（去除尾部点号）
                let mut j = i;
                while j > start && !bytes[j - 1].is_ascii_digit() {
                    j -= 1;
                }
                let version = &s[start..j];
                if !version.is_empty() && version.ends_with(|c: char| c.is_ascii_digit()) {
                    return Some(version.to_string());
                }
            }
            // 跳过剩余数字/点号
            while i < len && (bytes[i].is_ascii_digit() || bytes[i] == b'.') {
                i += 1;
            }
        } else {
            i += 1;
        }
    }
    None
}

/// 简单的 semver 比较：latest > current 则返回 true
pub(crate) fn is_newer_version(latest: &str, current: &str) -> bool {
    let parse = |v: &str| -> Vec<u32> {
        v.trim_start_matches('v')
            .split('.')
            .filter_map(|s| s.parse().ok())
            .collect()
    };
    let l = parse(latest);
    let c = parse(current);
    for i in 0..std::cmp::max(l.len(), c.len()) {
        let lv = l.get(i).unwrap_or(&0);
        let cv = c.get(i).unwrap_or(&0);
        if lv > cv {
            return true;
        }
        if lv < cv {
            return false;
        }
    }
    false
}

/// 检查 Claude CLI 是否有更新
#[tauri::command]
pub async fn check_claude_cli_update() -> Result<ClaudeCliUpdateInfo, String> {
    // 1. 获取已安装版本
    let installed = tokio::task::spawn_blocking(|| read_local_claude_version())
        .await
        .map_err(|e| e.to_string())?;

    // 2. 获取最新版本信息
    let latest = tokio::task::spawn_blocking(|| fetch_claude_latest())
        .await
        .map_err(|e| e.to_string())?
        .map_err(|e| e.to_string())?;

    let not_installed = installed.is_none();
    let has_update = match &installed {
        Some(v) => is_newer_version(&latest.version, v),
        None => false,
    };

    Ok(ClaudeCliUpdateInfo {
        installed_version: installed,
        latest_version: latest.version,
        has_update,
        not_installed,
    })
}

// ============================================
// Claude CLI 进程管理
// ============================================

/// 检测是否有 claude 进程在运行
#[tauri::command]
pub async fn check_claude_running() -> Result<bool, String> {
    tokio::task::spawn_blocking(|| {
        #[cfg(target_os = "windows")]
        let name = "claude.exe";
        #[cfg(not(target_os = "windows"))]
        let name = "claude";
        crate::platform::is_process_running(name)
    })
    .await
    .map_err(|e| e.to_string())?
}

/// 杀死所有 claude 进程
#[tauri::command]
pub async fn kill_claude_processes() -> Result<(), String> {
    tokio::task::spawn_blocking(|| {
        #[cfg(target_os = "windows")]
        let name = "claude.exe";
        #[cfg(not(target_os = "windows"))]
        let name = "claude";
        crate::platform::kill_processes_by_name(name)?;
        log::info!("[Installer] Killed claude processes");
        Ok(())
    })
    .await
    .map_err(|e| e.to_string())?
}

/// 检查已安装版本
#[tauri::command]
pub async fn check_installed_versions() -> Result<std::collections::HashMap<String, bool>, String> {
    let mut result = std::collections::HashMap::new();

    #[cfg(target_os = "windows")]
    let (claude_dir, git_dir) = get_install_dirs();
    #[cfg(not(target_os = "windows"))]
    let (claude_dir, _git_dir) = get_install_dirs();

    // 检查 Claude
    let platform = get_current_platform();
    let claude_file = if platform.starts_with("win32") {
        "claude.exe"
    } else {
        "claude"
    };
    result.insert("claude".to_string(), claude_dir.join(claude_file).exists());

    // 检查 Git（Windows only）
    #[cfg(target_os = "windows")]
    {
        result.insert("git".to_string(), git_dir.join("bin").join("bash.exe").exists());
    }

    Ok(result)
}

// ============================================
// Claude CLI 版本列表与历史版本下载
// ============================================

/// 获取本地已安装的 Claude CLI 版本号（轻量命令，纯本地，无 HTTP）
#[tauri::command]
pub async fn get_installed_claude_version() -> Result<Option<String>, String> {
    tokio::task::spawn_blocking(|| read_local_claude_version())
        .await
        .map_err(|e| e.to_string())
}

/// 拉取 OSS 上的 Claude versions.json（同步实现，由 spawn_blocking 调用）
fn fetch_claude_versions() -> io::Result<ClaudeVersions> {
    let url = format!("{}/deps/claude/versions.json", OSS_BASE_URL);
    log::info!("[Installer] Fetching Claude versions.json from {}", url);

    // 显式设置 15 秒超时，避免网络挂起导致前端 invoke 永久 pending
    let client = oss_client(std::time::Duration::from_secs(15))
        .map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))?;

    let response = client
        .get(&url)
        .send()
        .map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))?;

    if !response.status().is_success() {
        return Err(io::Error::new(
            io::ErrorKind::Other,
            format!("HTTP error: {}", response.status()),
        ));
    }

    let versions: ClaudeVersions = response
        .json()
        .map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))?;

    log::info!(
        "[Installer] Got {} Claude versions, latest: {}",
        versions.versions.len(),
        versions.latest
    );
    Ok(versions)
}

/// 从 OSS 拉取所有支持的 Claude CLI 历史版本列表
#[tauri::command]
pub async fn list_claude_versions() -> Result<ClaudeVersions, String> {
    // 必须用 spawn_blocking 包装 reqwest::blocking，否则会阻塞 tokio runtime
    tokio::task::spawn_blocking(fetch_claude_versions)
        .await
        .map_err(|e| e.to_string())?
        .map_err(|e| e.to_string())
}

/// 下载动作决策（纯函数，便于单测）
///
/// 综合考虑"本地缓存是否可用"和"是否被取消"，给出下一步动作：
/// - `Cancelled`：优先级最高，即使有缓存也直接退出
/// - `ReuseCache`：缓存可用，跳过下载
/// - `Download`：需要发起下载
#[derive(Debug, PartialEq, Eq)]
pub(crate) enum DownloadAction {
    Download,
    ReuseCache,
    Cancelled,
}

pub(crate) fn decide_download_action(
    cache_exists: bool,
    cache_size: u64,
    expected_size: u64,
    cancelled: bool,
) -> DownloadAction {
    if cancelled {
        return DownloadAction::Cancelled;
    }
    if cache_exists && cache_size == expected_size {
        DownloadAction::ReuseCache
    } else {
        DownloadAction::Download
    }
}

/// 下载指定历史版本的 Claude CLI 二进制到本地（用户手动安装）
///
/// 下载到用户下载目录（无则 home 目录），返回保存的绝对路径。
/// 前端拿到路径后调用 shell open 打开父目录，由用户手动复制/安装。
///
/// 行为：
/// - 若本地已存在同名文件且 size 与 OSS 记录一致，直接复用，跳过下载
/// - 下载过程可通过 `cancel_claude_download(version)` 取消
/// - 下载完成后再次校验 size，不匹配则删除文件并报错
#[tauri::command]
pub async fn download_claude_version(
    app: AppHandle,
    version: String,
) -> Result<String, String> {
    log::info!("[Installer] Downloading Claude CLI version {}", version);

    emit_progress(&app, "claude-history", "fetching", 0, "获取版本信息...");

    // 拉取 versions.json
    let versions = tokio::task::spawn_blocking(|| {
        let url = format!("{}/deps/claude/versions.json", OSS_BASE_URL);
        let client = oss_client(std::time::Duration::from_secs(15))
            .map_err(|e| format!("Failed to build HTTP client: {}", e))?;
        let resp = client
            .get(&url)
            .send()
            .map_err(|e| format!("Failed to fetch versions.json: {}", e))?;
        if !resp.status().is_success() {
            return Err(format!("HTTP error: {}", resp.status()));
        }
        let v: ClaudeVersions = resp
            .json()
            .map_err(|e| format!("Failed to parse versions.json: {}", e))?;
        Ok::<ClaudeVersions, String>(v)
    })
    .await
    .map_err(|e| e.to_string())??;

    // 查找对应版本条目
    let entry = versions
        .versions
        .iter()
        .find(|e| e.version == version)
        .ok_or_else(|| format!("Version {} not found in versions.json", version))?;

    // 取当前平台
    let platform = get_current_platform();
    let platform_info = entry
        .platforms
        .get(&platform)
        .ok_or_else(|| format!("Version {} not available for platform {}", version, platform))?;

    let expected_size = platform_info.size;
    let download_url = format!("{}/{}", OSS_BASE_URL, platform_info.url);

    // 文件名：claude-{version}.{ext}
    let url_path = std::path::Path::new(&platform_info.url);
    let original_name = url_path
        .file_name()
        .map(|n| n.to_string_lossy().to_string())
        .unwrap_or_else(|| "claude".to_string());
    let original_path = std::path::Path::new(&original_name);
    let stem = original_path
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("claude");
    let filename = match original_path.extension().and_then(|s| s.to_str()) {
        Some(ext) => format!("{}-{}.{}", stem, version, ext),
        None => format!("{}-{}", stem, version),
    };

    // 下载目录：用户下载目录 → home 目录 → 临时目录
    let download_dir = dirs::download_dir()
        .or_else(|| dirs::home_dir())
        .unwrap_or_else(|| std::env::temp_dir());
    let save_path = download_dir.join(&filename);

    // 注册取消标志（提前到决策之前，便于在缓存检查阶段也能响应取消）
    let cancel_flag = Arc::new(AtomicBool::new(false));
    {
        let mut map = ACTIVE_CLAUDE_DOWNLOADS.lock().unwrap();
        map.insert(version.clone(), cancel_flag.clone());
    }

    // 本地缓存复用决策
    let cache_exists = save_path.exists();
    let cache_size = if cache_exists {
        fs::metadata(&save_path).map(|m| m.len()).unwrap_or(0)
    } else {
        0
    };
    let cancelled = cancel_flag.load(Ordering::SeqCst);
    match decide_download_action(cache_exists, cache_size, expected_size, cancelled) {
        DownloadAction::ReuseCache => {
            log::info!(
                "[Installer] Reusing local cache: {} ({} bytes)",
                save_path.display(),
                cache_size
            );
            emit_progress(
                &app,
                "claude-history",
                "done",
                100,
                format!("v{} 已存在本地缓存", version),
            );
            // 注销取消标志后返回
            let mut map = ACTIVE_CLAUDE_DOWNLOADS.lock().unwrap();
            map.remove(&version);
            return Ok(save_path.to_string_lossy().to_string());
        }
        DownloadAction::Cancelled => {
            // 极罕见：用户在拉 versions.json 期间就取消
            let mut map = ACTIVE_CLAUDE_DOWNLOADS.lock().unwrap();
            map.remove(&version);
            return Err("cancelled".to_string());
        }
        DownloadAction::Download => {
            if cache_exists {
                log::info!(
                    "[Installer] Local file size mismatch ({} vs {}), removing",
                    cache_size,
                    expected_size
                );
                let _ = fs::remove_file(&save_path);
            }
        }
    }

    log::info!(
        "[Installer] Downloading {} to {}",
        download_url,
        save_path.display()
    );

    emit_progress(
        &app,
        "claude-history",
        "downloading",
        0,
        format!("开始下载 v{}", version),
    );

    // 等待下载完成（带取消标志）
    let save_path_clone = save_path.clone();
    let app_clone = app.clone();
    let cancel_for_task = cancel_flag.clone();
    let download_result = tokio::task::spawn_blocking(move || {
        download_file(
            &download_url,
            &save_path_clone,
            &app_clone,
            "claude-history",
            Some(&cancel_for_task),
        )
    })
    .await
    .map_err(|e| e.to_string())?;

    // 无论成功失败，移除取消标志
    {
        let mut map = ACTIVE_CLAUDE_DOWNLOADS.lock().unwrap();
        map.remove(&version);
    }

    if let Err(e) = download_result {
        // download_file 已处理半成品清理（取消 / 失败）
        // 但保险起见再清理一次（cancel 路径下文件可能因 Windows 文件占用未删干净）
        let _ = fs::remove_file(&save_path);

        if e.kind() == io::ErrorKind::Interrupted {
            return Err("cancelled".to_string());
        }
        return Err(e.to_string());
    }

    // 下载完成后再次校验 size（OSS 内容长度异常时拦截）
    let actual_size = fs::metadata(&save_path).map(|m| m.len()).unwrap_or(0);
    if actual_size != expected_size {
        log::warn!(
            "[Installer] Downloaded size {} != expected {}",
            actual_size,
            expected_size
        );
        let _ = fs::remove_file(&save_path);
        return Err(format!(
            "Downloaded file size mismatch: got {} bytes, expected {}",
            actual_size, expected_size
        ));
    }

    emit_progress(
        &app,
        "claude-history",
        "done",
        100,
        format!("v{} 下载完成", version),
    );

    log::info!(
        "[Installer] Claude CLI v{} downloaded to {}",
        version,
        save_path.display()
    );
    Ok(save_path.to_string_lossy().to_string())
}

/// 取消指定版本的 Claude CLI 历史版本下载
///
/// 返回 true 表示找到了活动下载并已标记取消；false 表示无活动下载（可能已完成或已取消）。
#[tauri::command]
pub async fn cancel_claude_download(version: String) -> Result<bool, String> {
    let flag = {
        let map = ACTIVE_CLAUDE_DOWNLOADS.lock().unwrap();
        map.get(&version).cloned()
    };
    if let Some(f) = flag {
        f.store(true, Ordering::SeqCst);
        log::info!("[Installer] Cancel requested for Claude CLI v{}", version);
        Ok(true)
    } else {
        log::info!(
            "[Installer] Cancel requested but no active download for v{}",
            version
        );
        Ok(false)
    }
}

/// 把本地下载好的 Claude CLI 二进制覆盖安装到标准安装目录
///
/// - Windows: `%USERPROFILE%\.local\bin\claude.exe`
/// - macOS/Linux: `~/.local/bin/claude`
///
/// 行为：
/// - 若检测到 claude 进程正在运行，返回 `claude-running` 错误字符串供前端识别并提示用户
/// - 复制（覆盖）源文件到目标路径
/// - Unix 下设置 0o755 权限
/// - 保存路径到 app config 的 `claudePath`
///
/// 前端典型调用流：
/// 1. 第一次调用，若返回 `claude-running` 错误 → 弹窗提示用户
/// 2. 用户确认 → 调用 `kill_claude_processes` 关闭所有 Claude 进程
/// 3. 再次调用本命令完成覆盖安装
#[tauri::command]
pub async fn install_claude_version(
    app: AppHandle,
    source_path: String,
    version: String,
) -> Result<String, String> {
    log::info!(
        "[Installer] Installing Claude CLI v{} from {}",
        version,
        source_path
    );

    let source = PathBuf::from(&source_path);
    if !source.exists() {
        return Err(format!("Source file not found: {}", source_path));
    }

    emit_progress(&app, "claude-history", "installing", 0, "检查 Claude 进程...");

    // 检测 claude 进程是否在运行
    let claude_running = tokio::task::spawn_blocking(|| {
        #[cfg(target_os = "windows")]
        let name = "claude.exe";
        #[cfg(not(target_os = "windows"))]
        let name = "claude";
        crate::platform::is_process_running(name)
    })
    .await
    .map_err(|e| e.to_string())?
    .map_err(|e| e.to_string())?;

    if claude_running {
        log::warn!("[Installer] Claude is running, aborting install");
        return Err("claude-running".to_string());
    }

    // 目标路径
    let (claude_dir, _) = get_install_dirs();
    fs::create_dir_all(&claude_dir)
        .map_err(|e| format!("Failed to create claude dir: {}", e))?;

    let filename = if cfg!(target_os = "windows") {
        "claude.exe"
    } else {
        "claude"
    };
    let target_path = claude_dir.join(filename);

    emit_progress(&app, "claude-history", "installing", 50, "复制文件...");

    let source_clone = source.clone();
    let target_clone = target_path.clone();
    tokio::task::spawn_blocking(move || fs::copy(&source_clone, &target_clone))
        .await
        .map_err(|e| e.to_string())?
        .map_err(|e| format!("Failed to copy file: {}", e))?;

    // 验证 size（fs::copy 不会改变字节数，但防御性校验）
    let src_size = fs::metadata(&source).map(|m| m.len()).unwrap_or(0);
    let tgt_size = fs::metadata(&target_path).map(|m| m.len()).unwrap_or(0);
    if src_size != tgt_size {
        return Err(format!(
            "Size mismatch after copy: source={}, target={}",
            src_size, tgt_size
        ));
    }

    // Unix 设置权限
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        fs::set_permissions(&target_path, fs::Permissions::from_mode(0o755))
            .map_err(|e| format!("Failed to set permissions: {}", e))?;
    }

    // 保存路径到 app config
    save_install_path_to_config("claudePath", &target_path);

    emit_progress(
        &app,
        "claude-history",
        "done",
        100,
        format!("v{} 安装完成", version),
    );

    log::info!(
        "[Installer] Claude CLI v{} installed to {}",
        version,
        target_path.display()
    );
    Ok(target_path.to_string_lossy().to_string())
}