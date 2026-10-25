//! 远程文件传输单测：sanitize / 锚点逃逸 / 幂等分块 / 并发提交 / 限额 /
//! 会话归属 / 一致性 / 三层清理。覆盖权限绕过、断线重试、并发冲突、
//! 路径逃逸、异常清理五类安全场景。

use std::path::{Path, PathBuf};
use std::sync::LazyLock;

use base64::Engine;

use crate::file_transfer;
use crate::remote_state::REMOTE_STATE;

/// 全局态互斥：UPLOADS/DOWNLOADS 会话表与 KNOWN_PROJECTS_OVERRIDE 必须串行
/// （与 CONFIG_DIR_TEST_LOCK 的顺序约定：先本锁再 CONFIG 锁，无环）。
/// parking_lot：测试断言失败 panic 不 poison（std Mutex 会连锁炸后续测试）
static FILE_TRANSFER_TEST_LOCK: LazyLock<parking_lot::Mutex<()>> =
    LazyLock::new(|| parking_lot::Mutex::new(()));

const CONN_A: u64 = 900_001;
const CONN_B: u64 = 900_002;

/// 测试环境（RAII）：临时项目根 + 白名单 override + 假 PTY 会话，
/// Drop 时全部还原（panic 安全）
struct FtEnv {
    _lock: parking_lot::MutexGuard<'static, ()>,
    _dir: tempfile::TempDir,
    pty_id: String,
}

impl FtEnv {
    fn new(tag: &str) -> Self {
        let lock = FILE_TRANSFER_TEST_LOCK.lock();
        let dir = tempfile::tempdir().unwrap();
        let cwd = dir.path().to_string_lossy().into_owned();
        file_transfer::set_known_projects_override(Some(vec![cwd.clone()]));
        let pty_id = format!("ft-pty-{}", tag);
        REMOTE_STATE.register_pty(&pty_id, "claude", &cwd);
        FtEnv {
            _lock: lock,
            _dir: dir,
            pty_id,
        }
    }

    fn cwd(&self) -> String {
        self._dir.path().to_string_lossy().into_owned()
    }

    fn files_dir(&self) -> PathBuf {
        self._dir.path().join(file_transfer::UPLOAD_DIR_NAME)
    }
}

impl Drop for FtEnv {
    fn drop(&mut self) {
        // 不调 remove_pty：它会向全部连接广播 SessionExit，污染并行运行的
        // 其他 remote_state 测试。SessionEntry 残留在全局表无干扰（各测试
        // 用唯一 pty_id，register_pty 幂等 insert）
        file_transfer::set_known_projects_override(None);
        // 清残留传输会话（失败用例的中途会话不外溢到其他测试）
        file_transfer::test_expire_sessions();
        file_transfer::sweep_stale();
    }
}

fn b64(data: &[u8]) -> String {
    base64::engine::general_purpose::STANDARD.encode(data)
}

fn unb64(data: &str) -> Vec<u8> {
    base64::engine::general_purpose::STANDARD.decode(data).unwrap()
}

/// 跨平台目录链接（Windows junction 免管理员权限 / Unix symlink）
fn make_dir_link(link: &Path, target: &Path) -> bool {
    #[cfg(windows)]
    {
        std::process::Command::new("cmd")
            .args(["/C", "mklink", "/J"])
            .arg(link)
            .arg(target)
            .output()
            .map(|o| o.status.success())
            .unwrap_or(false)
    }
    #[cfg(unix)]
    {
        std::os::unix::fs::symlink(target, link).is_ok()
    }
}

/// 完整上传链路辅助：begin → 全部分块 → finish
fn upload_full(env: &FtEnv, conn: u64, name: &str, data: &[u8]) -> Result<String, String> {
    let begin = file_transfer::upload_begin(
        Some(conn),
        &env.pty_id,
        name,
        data.len() as u64,
        Some(false),
    )?;
    for (i, chunk) in data.chunks(file_transfer::CHUNK_SIZE).enumerate() {
        file_transfer::upload_chunk(
            Some(conn),
            &begin.upload_id,
            (i * file_transfer::CHUNK_SIZE) as u64,
            &b64(chunk),
        )?;
    }
    let fin = file_transfer::upload_finish(Some(conn), &begin.upload_id)?;
    Ok(fin.target_path)
}

// ==================== sanitize ====================

#[test]
#[cfg(windows)]
#[test]
fn FileTransfer_DisplayPath_StripsVerbatim_1() {
    // canonicalize 的 verbatim 前缀剥离（UNC 保留双反斜杠形式）
    assert_eq!(
        file_transfer::display_path(Path::new(r"\\?\D:\demo\a.txt")),
        r"D:\demo\a.txt"
    );
    assert_eq!(
        file_transfer::display_path(Path::new(r"\\?\UNC\server\share\a.txt")),
        r"\\server\share\a.txt"
    );
    assert_eq!(
        file_transfer::display_path(Path::new(r"D:\demo\a.txt")),
        r"D:\demo\a.txt"
    );
}

fn FileTransfer_SanitizeFileName_1() {
    use file_transfer::sanitize_file_name as s;
    // 合法：中文、空格、多点、Unicode
    assert_eq!(s("报告 v2.pdf").unwrap(), "报告 v2.pdf");
    assert_eq!(s("a.b.c.txt").unwrap(), "a.b.c.txt");
    assert_eq!(s("文件.png").unwrap(), "文件.png");
    // 非法全拒（含 Windows 非法字符/ADS 冒号——避免传完才在提交时失败）
    for bad in [
        "", "..", "a..b", "a/b", "a\\b", "a\0b", " a", "a ", "a.", "CON", "CON.txt",
        "com3.log", "LPT1", "aux", "nul.txt", "a:b.txt", "a?b", "a*b", "a|b", "a<b", "a>b",
        "a\"b",
    ] {
        assert!(s(bad).is_err(), "应拒绝: {:?}", bad);
    }
    // 超长（UTF-16 单元 > 200；astral 字符占 2 单元）
    let long = "x".repeat(201);
    assert!(s(&long).is_err());
    assert!(s(&"x".repeat(200)).is_ok());
    assert!(s(&"😀".repeat(101)).is_err());
    // 控制字符
    assert!(s("a\x1bb").is_err());
}

// ==================== 路径逃逸（junction/symlink） ====================

#[test]
fn FileTransfer_UploadBegin_DirLinkEscape_1() {
    let env = FtEnv::new("escape");
    let outside = tempfile::tempdir().unwrap();
    let files = env.files_dir();
    std::fs::create_dir_all(files.parent().unwrap()).unwrap();
    // 预置 .cc-box-files 为指向外部目录的链接（junction/symlink）
    if !make_dir_link(&files, outside.path()) {
        eprintln!("skip: 无法创建目录链接（环境限制）");
        return;
    }
    let err = file_transfer::upload_begin(Some(CONN_A), &env.pty_id, "a.txt", 3, Some(false))
        .unwrap_err();
    assert!(
        err.contains("outside"),
        "junction/symlink 逃逸应被锚点校验拒绝: {}",
        err
    );
}

#[test]
fn FileTransfer_DownloadBegin_OutsideProjects_1() {
    let env = FtEnv::new("outside");
    let outside = tempfile::tempdir().unwrap();
    let f = outside.path().join("secret.txt");
    std::fs::write(&f, "s3cret").unwrap();
    // 白名单外路径：下载拒绝、列目录拒绝
    let err = file_transfer::download_begin(Some(CONN_A), &f.to_string_lossy(), Some(false))
        .unwrap_err();
    assert!(err.contains("authorized"), "{}", err);
    let err = file_transfer::list(&outside.path().to_string_lossy()).unwrap_err();
    assert!(err.contains("authorized"), "{}", err);
    // 项目内路径：通过
    let inside = env.cwd();
    std::fs::write(Path::new(&inside).join("ok.txt"), "ok").unwrap();
    assert!(file_transfer::list(&inside).is_ok());
}

// ==================== 上传 round-trip / 幂等 / offset ====================

#[test]
fn FileTransfer_UploadRoundTrip_1() {
    let env = FtEnv::new("roundtrip");
    let data: Vec<u8> = (0..file_transfer::CHUNK_SIZE + 100).map(|i| (i % 251) as u8).collect();
    let target = upload_full(&env, CONN_A, "data.bin", &data).unwrap();
    assert!(
        target.contains(file_transfer::UPLOAD_DIR_NAME),
        "落盘应在 {} 内: {}",
        file_transfer::UPLOAD_DIR_NAME,
        target
    );
    // 下发路径不得带 Windows canonicalize 的 verbatim 前缀
    assert!(!target.starts_with(r"\\?\"), "路径不应含 verbatim 前缀: {}", target);
    assert_eq!(std::fs::read(&target).unwrap(), data, "字节应全等");
}

#[test]
fn FileTransfer_UploadRelativePath_1() {
    let env = FtEnv::new("relative");
    let begin =
        file_transfer::upload_begin(Some(CONN_A), &env.pty_id, "图 1.jpg", 2, Some(false)).unwrap();
    assert_eq!(begin.relative_path, ".cc-box-files/图 1.jpg");
    file_transfer::upload_chunk(Some(CONN_A), &begin.upload_id, 0, &b64(b"xx")).unwrap();
    let fin = file_transfer::upload_finish(Some(CONN_A), &begin.upload_id).unwrap();
    assert_eq!(fin.relative_path, ".cc-box-files/图 1.jpg");
    // 冲突改名后 relative_path 跟随最终文件名
    let fin2 = {
        let b = file_transfer::upload_begin(Some(CONN_A), &env.pty_id, "图 1.jpg", 1, Some(false))
            .unwrap();
        file_transfer::upload_chunk(Some(CONN_A), &b.upload_id, 0, &b64(b"y")).unwrap();
        file_transfer::upload_finish(Some(CONN_A), &b.upload_id).unwrap()
    };
    assert_eq!(fin2.relative_path, ".cc-box-files/图 1 (1).jpg");
}

#[test]
fn FileTransfer_UploadChunk_IdempotentResend_1() {
    let env = FtEnv::new("idempotent");
    let begin = file_transfer::upload_begin(
        Some(CONN_A),
        &env.pty_id,
        "i.txt",
        11,
        Some(false),
    )
    .unwrap();
    let r1 = file_transfer::upload_chunk(Some(CONN_A), &begin.upload_id, 0, &b64(b"hello ")).unwrap();
    assert_eq!(r1.received, 6);
    // ACK 丢失重发同块（offset 0 + 6 == written 6）：幂等返回、不重写
    let r2 = file_transfer::upload_chunk(Some(CONN_A), &begin.upload_id, 0, &b64(b"hello ")).unwrap();
    assert_eq!(r2.received, 6);
    // 继续下一块仍可用
    file_transfer::upload_chunk(Some(CONN_A), &begin.upload_id, 6, &b64(b"world")).unwrap();
    let fin = file_transfer::upload_finish(Some(CONN_A), &begin.upload_id).unwrap();
    assert_eq!(std::fs::read(&fin.target_path).unwrap(), b"hello world");
}

#[test]
fn FileTransfer_UploadChunk_OffsetMismatch_1() {
    let env = FtEnv::new("mismatch");
    let begin =
        file_transfer::upload_begin(Some(CONN_A), &env.pty_id, "m.txt", 10, Some(false)).unwrap();
    // 跳块（offset 5 但 written 0）拒绝
    let err = file_transfer::upload_chunk(Some(CONN_A), &begin.upload_id, 5, &b64(b"12345"))
        .unwrap_err();
    assert!(err.contains("offset mismatch"), "{}", err);
    // 拒绝后会话保留：正确 offset 仍可继续
    assert!(file_transfer::upload_chunk(Some(CONN_A), &begin.upload_id, 0, &b64(b"0123456789")).is_ok());
    // 超申报 size（written 10 == expected，再写必超）
    let err = file_transfer::upload_chunk(Some(CONN_A), &begin.upload_id, 10, &b64(b"x"))
        .unwrap_err();
    assert!(err.contains("declared size"), "{}", err);
}

#[test]
fn FileTransfer_Upload_EmptyFile_1() {
    let env = FtEnv::new("empty");
    let target = upload_full(&env, CONN_A, "empty.txt", b"").unwrap();
    assert_eq!(std::fs::read(&target).unwrap(), b"");
}

#[test]
fn FileTransfer_UploadChunk_TruncatesDirtyPrefix_1() {
    let env = FtEnv::new("truncate");
    let begin =
        file_transfer::upload_begin(Some(CONN_A), &env.pty_id, "t.txt", 5, Some(false)).unwrap();
    let part = env
        .files_dir()
        .join(format!(".upload-{}.part", begin.upload_id));
    // 模拟上次 write_all 半途失败：文件有脏前缀，但内存计数仍为 0
    std::fs::write(&part, b"dirty-prefix").unwrap();
    file_transfer::upload_chunk(Some(CONN_A), &begin.upload_id, 0, &b64(b"hello")).unwrap();
    let fin = file_transfer::upload_finish(Some(CONN_A), &begin.upload_id).unwrap();
    // set_len 截断脏前缀后写入：内容精确为 hello（无静默损坏）
    assert_eq!(std::fs::read(&fin.target_path).unwrap(), b"hello");
}

#[test]
fn FileTransfer_UploadFinish_DiskLengthMismatch_1() {
    let env = FtEnv::new("dirty");
    let begin =
        file_transfer::upload_begin(Some(CONN_A), &env.pty_id, "d.txt", 5, Some(false)).unwrap();
    file_transfer::upload_chunk(Some(CONN_A), &begin.upload_id, 0, &b64(b"hello")).unwrap();
    // 模拟上次 write_all 半途失败留下的脏字节：文件比计数长
    let part = env
        .files_dir()
        .join(format!(".upload-{}.part", begin.upload_id));
    use std::io::Write;
    let mut f = std::fs::OpenOptions::new().append(true).open(&part).unwrap();
    f.write_all(b"garbage").unwrap();
    drop(f);
    // finish 以落盘长度校验拒绝提交（不产生损坏文件）
    let err = file_transfer::upload_finish(Some(CONN_A), &begin.upload_id).unwrap_err();
    assert!(err.contains("inconsistent"), "{}", err);
    assert!(!part.exists(), "不一致的临时文件应被删除");
    assert!(!env.files_dir().join("d.txt").exists(), "不得提交损坏文件");
}

// ==================== 并发提交（不覆盖） ====================

#[test]
fn FileTransfer_UploadFinish_TargetCreatedExternally_1() {
    let env = FtEnv::new("external");
    let begin =
        file_transfer::upload_begin(Some(CONN_A), &env.pty_id, "e.txt", 3, Some(false)).unwrap();
    file_transfer::upload_chunk(Some(CONN_A), &begin.upload_id, 0, &b64(b"new")).unwrap();
    // finish 前外部创建同名文件：hard_link 不覆盖，自动改名
    let target = env.files_dir().join("e.txt");
    std::fs::write(&target, "old").unwrap();
    let fin = file_transfer::upload_finish(Some(CONN_A), &begin.upload_id).unwrap();
    assert!(fin.target_path.contains("e (1).txt"), "{}", fin.target_path);
    assert_eq!(std::fs::read(&target).unwrap(), b"old", "外部文件不得被覆盖");
    assert_eq!(std::fs::read(&fin.target_path).unwrap(), b"new");
}

#[test]
fn FileTransfer_UploadFinish_ConcurrentSameName_1() {
    let env = FtEnv::new("concurrent");
    let b1 = file_transfer::upload_begin(Some(CONN_A), &env.pty_id, "c.txt", 1, Some(false)).unwrap();
    // 第二个会话同名 begin（目标尚不存在 → 暂定同名；提交时 hard_link 仲裁）
    let b2 = file_transfer::upload_begin(Some(CONN_B), &env.pty_id, "c.txt", 1, Some(false)).unwrap();
    file_transfer::upload_chunk(Some(CONN_A), &b1.upload_id, 0, &b64(b"1")).unwrap();
    file_transfer::upload_chunk(Some(CONN_B), &b2.upload_id, 0, &b64(b"2")).unwrap();
    let f1 = file_transfer::upload_finish(Some(CONN_A), &b1.upload_id).unwrap();
    let f2 = file_transfer::upload_finish(Some(CONN_B), &b2.upload_id).unwrap();
    // 两会话各得不同文件名，内容各自完整
    assert_ne!(f1.target_path, f2.target_path);
    assert_eq!(std::fs::read(&f1.target_path).unwrap(), b"1");
    assert_eq!(std::fs::read(&f2.target_path).unwrap(), b"2");
}

// ==================== 会话归属 / remote only ====================

#[test]
fn FileTransfer_SessionOwnership_1() {
    let env = FtEnv::new("ownership");
    // 桌面路径（conn_id None）拒绝
    let err = file_transfer::upload_begin(None, &env.pty_id, "o.txt", 3, None).unwrap_err();
    assert!(err.contains("remote-only"), "{}", err);
    let begin =
        file_transfer::upload_begin(Some(CONN_A), &env.pty_id, "o.txt", 3, Some(false)).unwrap();
    // 他人连接用我的 uploadId：与不存在同文案（不泄露存在性）
    let err = file_transfer::upload_chunk(Some(CONN_B), &begin.upload_id, 0, &b64(b"abc"))
        .unwrap_err();
    assert_eq!(err, "transfer session not found");
    let err = file_transfer::upload_finish(Some(CONN_B), &begin.upload_id).unwrap_err();
    assert_eq!(err, "transfer session not found");
    // 未知 uploadId 同文案
    let err = file_transfer::upload_chunk(Some(CONN_A), "no-such-id", 0, &b64(b"abc")).unwrap_err();
    assert_eq!(err, "transfer session not found");
    // 无会话 pty
    let err = file_transfer::upload_begin(Some(CONN_A), "no-such-pty", "o.txt", 3, Some(false))
        .unwrap_err();
    assert!(err.contains("no active session"), "{}", err);
}

// ==================== 限额（按连接类型） ====================

/// 写 config.json 的 remote 节（TestConfigDirGuard 隔离；CONFIG_DIR_TEST_LOCK 串行）
fn write_limits_config(relay: u64, direct: u64) -> crate::tests::TestConfigDirGuard {
    let guard = crate::tests::TestConfigDirGuard::new("ft-limits");
    let cfg = serde_json::json!({
        "remote": { "fileTransferMaxRelay": relay, "fileTransferMaxDirect": direct }
    });
    std::fs::write(guard.0.join("config.json"), cfg.to_string()).unwrap();
    guard
}

#[test]
fn FileTransfer_SizeLimit_ByMode_1() {
    let env = FtEnv::new("limits");
    let _cfg_lock = crate::tests::CONFIG_DIR_TEST_LOCK.lock();
    let _cfg = write_limits_config(1024, 10 * 1024);
    // 中继档超限拒
    let err = file_transfer::upload_begin(Some(CONN_A), &env.pty_id, "big.bin", 2048, Some(true))
        .unwrap_err();
    assert!(err.starts_with("file too large") && err.contains("relay"), "{}", err);
    // 直连档（上限更大）通过
    assert!(file_transfer::upload_begin(
        Some(CONN_A),
        &env.pty_id,
        "big.bin",
        2048,
        Some(false)
    )
    .is_ok());
    // 下载同理：中继拒、直连过
    let f = env.files_dir();
    std::fs::create_dir_all(&f).unwrap();
    let big = f.join("big.dat");
    std::fs::write(&big, vec![0u8; 2048]).unwrap();
    let err = file_transfer::download_begin(Some(CONN_A), &big.to_string_lossy(), Some(true))
        .unwrap_err();
    assert!(err.starts_with("file too large") && err.contains("relay"), "{}", err);
    assert!(file_transfer::download_begin(Some(CONN_A), &big.to_string_lossy(), Some(false)).is_ok());
}

#[test]
fn FileTransfer_SizeLimit_ZeroDisables_1() {
    let env = FtEnv::new("disable");
    let _cfg_lock = crate::tests::CONFIG_DIR_TEST_LOCK.lock();
    // 空 config 目录（config.json 不存在）：默认值生效
    let guard = crate::tests::TestConfigDirGuard::new("ft-defaults");
    let l = file_transfer::limits();
    assert_eq!(l.direct_max, file_transfer::DEFAULT_FILE_MAX_DIRECT);
    assert_eq!(l.relay_max, file_transfer::DEFAULT_FILE_MAX_RELAY);
    // 写 0 限额：禁用（任何大小都拒）
    std::fs::write(
        guard.0.join("config.json"),
        serde_json::json!({ "remote": { "fileTransferMaxRelay": 0, "fileTransferMaxDirect": 10240 } })
            .to_string(),
    )
    .unwrap();
    let err = file_transfer::upload_begin(Some(CONN_A), &env.pty_id, "x.txt", 1, Some(true))
        .unwrap_err();
    assert!(err.starts_with("file too large"), "{}", err);
}

// ==================== 并发上限 ====================

#[test]
fn FileTransfer_UploadBegin_ConcurrencyLimit_1() {
    let env = FtEnv::new("maxconn");
    let mut ids = Vec::new();
    for i in 0..8 {
        let b = file_transfer::upload_begin(
            Some(CONN_A),
            &env.pty_id,
            &format!("f{}.txt", i),
            1,
            Some(false),
        )
        .unwrap();
        ids.push(b.upload_id);
    }
    let err = file_transfer::upload_begin(Some(CONN_A), &env.pty_id, "f9.txt", 1, Some(false))
        .unwrap_err();
    assert!(err.contains("concurrent"), "{}", err);
    // abort 一个后可再 begin
    file_transfer::upload_abort(Some(CONN_A), &ids[0]).unwrap();
    assert!(file_transfer::upload_begin(Some(CONN_A), &env.pty_id, "f9.txt", 1, Some(false)).is_ok());
}

// ==================== 下载 round-trip / 一致性 ====================

#[test]
fn FileTransfer_DownloadRoundTrip_1() {
    let env = FtEnv::new("dl");
    let f = env.cwd();
    let p = Path::new(&f).join("dl.bin");
    let data: Vec<u8> = (0..file_transfer::CHUNK_SIZE + 88).map(|i| (i % 249) as u8).collect();
    std::fs::write(&p, &data).unwrap();
    let begin =
        file_transfer::download_begin(Some(CONN_A), &p.to_string_lossy(), Some(false)).unwrap();
    assert_eq!(begin.size, data.len() as u64);
    assert_eq!(begin.file_name, "dl.bin");

    // 首块 512K 非_eof
    let c1 = file_transfer::download_chunk(Some(CONN_A), &begin.download_id, 0).unwrap();
    assert_eq!(c1.bytes as usize, file_transfer::CHUNK_SIZE);
    assert!(!c1.eof);
    // 次块 88B eof
    let c2 =
        file_transfer::download_chunk(Some(CONN_A), &begin.download_id, file_transfer::CHUNK_SIZE as u64)
            .unwrap();
    assert_eq!(c2.bytes, 88);
    assert!(c2.eof);
    // 拼装字节全等
    let mut got = unb64(&c1.data_b64);
    got.extend(unb64(&c2.data_b64));
    assert_eq!(got, data);
    // eof 块幂等重发（ACK 丢失场景；会话保留至 TTL）
    let c2r =
        file_transfer::download_chunk(Some(CONN_A), &begin.download_id, file_transfer::CHUNK_SIZE as u64)
            .unwrap();
    assert!(c2r.eof);
}

#[test]
fn FileTransfer_Download_SourceChanged_1() {
    let env = FtEnv::new("changed");
    let p = Path::new(&env.cwd()).join("src.txt");
    std::fs::write(&p, "v1-content").unwrap();
    let begin =
        file_transfer::download_begin(Some(CONN_A), &p.to_string_lossy(), Some(false)).unwrap();
    file_transfer::download_chunk(Some(CONN_A), &begin.download_id, 0).unwrap();
    // 下载中源文件被修改（mtime/size 变）→ 拒绝继续
    std::fs::write(&p, "v2-content-longer").unwrap();
    let err = file_transfer::download_chunk(Some(CONN_A), &begin.download_id, 4).unwrap_err();
    assert_eq!(err, "source file changed during download");
    // 越界 offset 拒
    let err = file_transfer::download_chunk(Some(CONN_A), &begin.download_id, 999_999).unwrap_err();
    assert!(err.contains("beyond"), "{}", err);
}

#[test]
fn FileTransfer_DownloadChunk_SessionMissing_1() {
    let env = FtEnv::new("nosession");
    // 跳过 begin 直接 chunk → 拒
    let err = file_transfer::download_chunk(Some(CONN_A), "no-such", 0).unwrap_err();
    assert_eq!(err, "transfer session not found");
    // 非 file 桌面路径
    let err = file_transfer::download_chunk(None, "x", 0).unwrap_err();
    assert!(err.contains("remote-only"), "{}", err);
    let _ = env; // 仅持锁
}

#[test]
fn FileTransfer_DownloadChunk_Ownership_1() {
    let env = FtEnv::new("dl-owner");
    let p = Path::new(&env.cwd()).join("o.txt");
    std::fs::write(&p, "secret").unwrap();
    let begin =
        file_transfer::download_begin(Some(CONN_A), &p.to_string_lossy(), Some(false)).unwrap();
    // 他人连接用我的 downloadId → 同文案拒绝（不泄露存在性）
    let err = file_transfer::download_chunk(Some(CONN_B), &begin.download_id, 0).unwrap_err();
    assert_eq!(err, "transfer session not found");
    // 本连接正常
    assert!(file_transfer::download_chunk(Some(CONN_A), &begin.download_id, 0).is_ok());
}

// ==================== 列文件 ====================

#[test]
fn FileTransfer_List_1() {
    let env = FtEnv::new("list");
    let dir = env.files_dir();
    std::fs::create_dir_all(&dir).unwrap();
    std::fs::write(dir.join("a.txt"), "aaa").unwrap();
    std::fs::write(dir.join("z.txt"), "zzz").unwrap();
    std::fs::create_dir_all(dir.join("sub")).unwrap();
    std::fs::write(dir.join(".upload-abc.part"), "stale").unwrap();
    let entries = file_transfer::list(&dir.to_string_lossy()).unwrap();
    // .part 过滤、目录在前、按名排序
    let names: Vec<&str> = entries.iter().map(|e| e.name.as_str()).collect();
    assert_eq!(names, vec!["sub", "a.txt", "z.txt"]);
    let a = entries.iter().find(|e| e.name == "a.txt").unwrap();
    assert_eq!(a.size, Some(3));
    assert!(a.modified.is_some());
    assert!(!a.is_dir);
    // 下发路径不得带 verbatim 前缀（否则前端无法与项目根比较）
    assert!(!a.path.starts_with(r"\\?\"), "列目录路径不应含 verbatim 前缀: {}", a.path);
    assert!(entries.iter().find(|e| e.name == "sub").unwrap().is_dir);
}

// ==================== 清理（abort / TTL / 启动） ====================

#[test]
fn FileTransfer_Abort_RemovesPart_1() {
    let env = FtEnv::new("abort");
    let begin =
        file_transfer::upload_begin(Some(CONN_A), &env.pty_id, "ab.txt", 10, Some(false)).unwrap();
    file_transfer::upload_chunk(Some(CONN_A), &begin.upload_id, 0, &b64(b"12345")).unwrap();
    let part = env.files_dir().join(format!(".upload-{}.part", begin.upload_id));
    assert!(part.exists());
    // abort：幂等（二次不报错）、.part 删除、会话终结
    file_transfer::upload_abort(Some(CONN_A), &begin.upload_id).unwrap();
    file_transfer::upload_abort(Some(CONN_A), &begin.upload_id).unwrap();
    assert!(!part.exists());
    let err = file_transfer::upload_chunk(Some(CONN_A), &begin.upload_id, 5, &b64(b"67890"))
        .unwrap_err();
    assert_eq!(err, "transfer session not found");
}

#[test]
fn FileTransfer_SweepStale_TtlCleanup_1() {
    let env = FtEnv::new("sweep");
    let begin =
        file_transfer::upload_begin(Some(CONN_A), &env.pty_id, "sw.txt", 10, Some(false)).unwrap();
    let part = env.files_dir().join(format!(".upload-{}.part", begin.upload_id));
    assert!(part.exists());
    // 过期后任一入口触发 sweep：.part 删 + 会话清
    file_transfer::test_expire_sessions();
    file_transfer::sweep_stale();
    assert!(!part.exists());
    let err = file_transfer::upload_finish(Some(CONN_A), &begin.upload_id).unwrap_err();
    assert_eq!(err, "transfer session not found");
}

/// 把文件 mtime 设为 age 之前（模拟崩溃遗留；std File::set_modified）
fn age_file(p: &Path, age: std::time::Duration) {
    let f = std::fs::OpenOptions::new().write(true).open(p).unwrap();
    let t = std::time::SystemTime::now()
        .checked_sub(age)
        .unwrap_or(std::time::SystemTime::now());
    f.set_modified(t).unwrap();
}

#[test]
fn FileTransfer_StartupCleanup_1() {
    let env = FtEnv::new("startup");
    let dir = env.files_dir();
    std::fs::create_dir_all(&dir).unwrap();
    let stale = dir.join(".upload-deadbeef.part");
    let fresh = dir.join(".upload-fresh.part");
    let keep = dir.join("keep.txt");
    std::fs::write(&stale, "stale").unwrap();
    std::fs::write(&fresh, "in-flight").unwrap();
    std::fs::write(&keep, "ok").unwrap();
    age_file(&stale, std::time::Duration::from_secs(3600));
    age_file(&keep, std::time::Duration::from_secs(3600));
    file_transfer::startup_cleanup();
    assert!(!stale.exists(), "过期遗留 .part 应被启动清理删除");
    assert!(fresh.exists(), "新 .part 可能是并行实例在途传输，不得误删");
    assert!(keep.exists(), "正常文件不受影响");
}

#[test]
fn FileTransfer_StartupCleanup_UnlinkedDirSkipped_1() {
    let env = FtEnv::new("startup-link");
    let outside = tempfile::tempdir().unwrap();
    let victim = outside.path().join(".upload-victim.part");
    std::fs::write(&victim, "should survive").unwrap();
    age_file(&victim, std::time::Duration::from_secs(3600));
    let files = env.files_dir();
    if !make_dir_link(&files, outside.path()) {
        eprintln!("skip: 无法创建目录链接（环境限制）");
        return;
    }
    // .cc-box-files 指向项目外：启动清理不得删除项目外文件
    file_transfer::startup_cleanup();
    assert!(victim.exists(), "链接到项目外的目录不应被清理");
}

// ==================== 一次性下载票据 ====================

#[test]
fn FileTransfer_TicketSignAndRead_1() {
    let env = FtEnv::new("ticket-ok");
    let path = env._dir.path().join("hello.txt");
    std::fs::write(&path, b"hello ticket").unwrap();
    let path_s = path.to_string_lossy().into_owned();

    let info =
        file_transfer::create_download_ticket(Some(CONN_A), &path_s, Some(false)).unwrap();
    assert_eq!(info.file_name, "hello.txt");
    assert_eq!(info.size, b"hello ticket".len() as u64);
    assert!(info.expires_at > 0);

    let chunk = file_transfer::download_ticket_chunk(&info.ticket, 0).unwrap();
    assert_eq!(chunk.bytes, b"hello ticket".len() as u64);
    assert!(chunk.eof);
    assert_eq!(unb64(&chunk.data_b64), b"hello ticket");
}

/// 票据不绑连接：手机切到系统下载器后 WS 断开，票据必须仍然可用
#[test]
fn FileTransfer_TicketSurvivesConnCleanup_1() {
    let env = FtEnv::new("ticket-conn");
    let path = env._dir.path().join("keep.bin");
    std::fs::write(&path, b"payload").unwrap();
    let path_s = path.to_string_lossy().into_owned();

    let info =
        file_transfer::create_download_ticket(Some(CONN_A), &path_s, Some(false)).unwrap();
    file_transfer::cleanup_conn(CONN_A);
    let chunk = file_transfer::download_ticket_chunk(&info.ticket, 0).unwrap();
    assert_eq!(unb64(&chunk.data_b64), b"payload");
}

#[test]
fn FileTransfer_TicketExpires_1() {
    let env = FtEnv::new("ticket-expire");
    let path = env._dir.path().join("expire.txt");
    std::fs::write(&path, b"x").unwrap();
    let path_s = path.to_string_lossy().into_owned();

    let info =
        file_transfer::create_download_ticket(Some(CONN_A), &path_s, Some(false)).unwrap();
    file_transfer::test_expire_sessions();
    file_transfer::sweep_stale();
    assert!(file_transfer::peek_ticket(&info.ticket).is_none());
    assert!(file_transfer::download_ticket_chunk(&info.ticket, 0).is_err());
}

/// HEAD 探测（peek）不扣使用次数
#[test]
fn FileTransfer_TicketPeekDoesNotConsume_1() {
    let env = FtEnv::new("ticket-peek");
    let path = env._dir.path().join("peek.txt");
    std::fs::write(&path, b"peek").unwrap();
    let path_s = path.to_string_lossy().into_owned();

    let info =
        file_transfer::create_download_ticket(Some(CONN_A), &path_s, Some(false)).unwrap();
    for _ in 0..100 {
        assert!(file_transfer::peek_ticket(&info.ticket).is_some());
    }
    assert!(file_transfer::download_ticket_chunk(&info.ticket, 0).is_ok());
}

#[test]
fn FileTransfer_TicketOutsideProjectRejected_1() {
    let _env = FtEnv::new("ticket-outside");
    let outside = tempfile::tempdir().unwrap();
    let victim = outside.path().join("secret.txt");
    std::fs::write(&victim, b"secret").unwrap();
    let path_s = victim.to_string_lossy().into_owned();

    assert!(file_transfer::create_download_ticket(Some(CONN_A), &path_s, Some(false)).is_err());
}

#[test]
fn FileTransfer_TicketSourceChangedRejected_1() {
    let env = FtEnv::new("ticket-changed");
    let path = env._dir.path().join("grow.txt");
    std::fs::write(&path, b"one").unwrap();
    let path_s = path.to_string_lossy().into_owned();

    let info =
        file_transfer::create_download_ticket(Some(CONN_A), &path_s, Some(false)).unwrap();
    std::fs::write(&path, b"one-and-then-some-more").unwrap();
    let err = file_transfer::download_ticket_chunk(&info.ticket, 0).unwrap_err();
    assert!(err.contains("changed during download"), "unexpected: {}", err);
}

#[test]
fn FileTransfer_TicketRequiresRemoteConn_1() {
    let env = FtEnv::new("ticket-desktop");
    let path = env._dir.path().join("local.txt");
    std::fs::write(&path, b"local").unwrap();
    let path_s = path.to_string_lossy().into_owned();

    assert!(file_transfer::create_download_ticket(None, &path_s, Some(false)).is_err());
}

/// 票据使用次数耗尽后失效
#[test]
fn FileTransfer_TicketExhaustsUses_1() {
    let env = FtEnv::new("ticket-uses");
    let path = env._dir.path().join("uses.txt");
    std::fs::write(&path, b"x").unwrap();
    let path_s = path.to_string_lossy().into_owned();

    let info =
        file_transfer::create_download_ticket(Some(CONN_A), &path_s, Some(false)).unwrap();
    let mut count = 0u32;
    while file_transfer::take_ticket(&info.ticket).is_ok() {
        count += 1;
        assert!(count <= 1024, "ticket uses did not terminate");
    }
    assert_eq!(count, 256, "票据使用次数应为 256");
    assert!(file_transfer::take_ticket(&info.ticket).is_err());
}
