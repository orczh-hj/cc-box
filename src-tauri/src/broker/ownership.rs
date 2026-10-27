//! 设备所有权锁与成员注册密钥
//!
//! - `broker.lock`：OS 排他锁，保证本机只有一个 Broker 进程（崩溃自动释放）。
//! - `gateway.lock`：设备所有权锁，兼容旧版 GUI 的选举协议。Broker 作为活动
//!   Gateway 时持有并每 2 秒续心跳；`legacy_owned`（旧版 Gateway 占口）时绝不
//!   写它，避免抢走旧版所有权。
//! - `gateway.secret`：旧版 `/internal/gw-ws` 成员注册密钥。
//! - `broker.secret`：新版 Named Pipe/UDS 成员与管理操作密钥（0600/ACL）。

use std::io::Write as _;
use std::path::PathBuf;
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use crate::gateway_proto::instance_id;

/// `gateway.lock` 过期时间：进程崩溃后最多 8 秒可被接管（心跳 2 秒）
const LOCK_TTL_MS: u128 = 8_000;

static LOCK_OWNED: AtomicBool = AtomicBool::new(false);
static LOCK_GEN: AtomicU64 = AtomicU64::new(0);

#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
struct LockInfo {
    instance_id: String,
    pid: u32,
    ts: u128,
}

fn config_dir() -> anyhow::Result<PathBuf> {
    Ok(crate::store::get_gui_config_dir()?)
}

pub(crate) fn gateway_lock_path() -> anyhow::Result<PathBuf> {
    Ok(config_dir()?.join("gateway.lock"))
}

pub(crate) fn gateway_secret_path() -> anyhow::Result<PathBuf> {
    Ok(config_dir()?.join("gateway.secret"))
}

pub(crate) fn broker_secret_path() -> anyhow::Result<PathBuf> {
    Ok(config_dir()?.join("broker.secret"))
}

pub(crate) fn broker_lock_path() -> anyhow::Result<PathBuf> {
    Ok(config_dir()?.join("broker.lock"))
}

fn now_ms() -> u128 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_millis())
        .unwrap_or(0)
}

fn read_lock_info() -> Option<LockInfo> {
    let path = gateway_lock_path().ok()?;
    let text = std::fs::read_to_string(path).ok()?;
    serde_json::from_str(&text).ok()
}

pub(crate) fn lock_is_live() -> bool {
    read_lock_info()
        .map(|i| now_ms().saturating_sub(i.ts) < LOCK_TTL_MS)
        .unwrap_or(false)
}

/// 锁文件是否过期。解析失败时用文件 mtime 兜底：另一个进程可能正在
/// `create_new` 后写入（短暂空文件），绝不能立刻判 stale 并删除活锁。
fn lock_is_stale(path: &std::path::Path) -> bool {
    match read_lock_info() {
        Some(i) => now_ms().saturating_sub(i.ts) >= LOCK_TTL_MS,
        None => std::fs::metadata(path)
            .and_then(|m| m.modified())
            .ok()
            .and_then(|t| t.elapsed().ok())
            .map(|e| e.as_millis() >= LOCK_TTL_MS)
            .unwrap_or(false),
    }
}

/// 尝试取得设备 Gateway 所有权。成功即由 Broker 持有，旧版 GUI 探测到
/// `/gateway.ok` 后会注册为成员而不是重新选举。
pub(crate) fn try_acquire_gateway_lock() -> bool {
    if LOCK_OWNED.load(Ordering::SeqCst) {
        return true;
    }
    let Ok(path) = gateway_lock_path() else {
        return false;
    };
    let info = LockInfo {
        instance_id: instance_id().to_string(),
        pid: std::process::id(),
        ts: now_ms(),
    };
    for _ in 0..3 {
        match std::fs::OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(&path)
        {
            Ok(mut f) => {
                let ok = f
                    .write_all(serde_json::to_string(&info).unwrap_or_default().as_bytes())
                    .is_ok();
                if !ok {
                    let _ = std::fs::remove_file(&path);
                    return false;
                }
                LOCK_OWNED.store(true, Ordering::SeqCst);
                return true;
            }
            Err(e) if e.kind() == std::io::ErrorKind::AlreadyExists => {
                if !lock_is_stale(&path) {
                    return false;
                }
                let _ = std::fs::remove_file(&path);
            }
            Err(_) => return false,
        }
    }
    false
}

/// 旧 Gateway 已确认不可达时立即回收其锁（免等 TTL；保留给旧版兼容恢复）
#[allow(dead_code)]
pub(crate) fn force_reclaim_gateway_lock(owner_instance_id: &str) -> bool {
    if read_lock_info()
        .map(|i| i.instance_id == owner_instance_id)
        .unwrap_or(false)
    {
        let Ok(path) = gateway_lock_path() else {
            return false;
        };
        return std::fs::remove_file(path).is_ok();
    }
    false
}

pub(crate) fn refresh_gateway_lock() {
    if !LOCK_OWNED.load(Ordering::SeqCst) {
        return;
    }
    let Ok(path) = gateway_lock_path() else {
        return;
    };
    // 锁可能已被旧版/其他 Broker 合法回收：不再无条件写回，避免双 Gateway。
    let Some(current) = read_lock_info() else {
        LOCK_OWNED.store(false, Ordering::SeqCst);
        log::warn!("[gateway-lock] lock disappeared; stop heartbeat");
        return;
    };
    if current.instance_id != instance_id() {
        LOCK_OWNED.store(false, Ordering::SeqCst);
        log::warn!("[gateway-lock] ownership lost; stop heartbeat");
        return;
    }
    let info = LockInfo {
        instance_id: instance_id().to_string(),
        pid: std::process::id(),
        ts: now_ms(),
    };
    let Ok(text) = serde_json::to_vec(&info) else {
        return;
    };
    // 原子写：避免旧版读者看到截断文件后误判 stale 并抢锁。
    if let Err(e) = atomic_write_private(&path, &text) {
        log::warn!("[gateway-lock] heartbeat write failed: {}", e);
    }
}

pub(crate) fn release_gateway_lock() {
    LOCK_GEN.fetch_add(1, Ordering::SeqCst);
    if !LOCK_OWNED.swap(false, Ordering::SeqCst) {
        return;
    }
    if read_lock_info()
        .map(|i| i.instance_id == instance_id().to_string())
        .unwrap_or(false)
    {
        if let Ok(path) = gateway_lock_path() {
            let _ = std::fs::remove_file(path);
        }
    }
}

/// 锁心跳：持有期间每 2 秒刷新 ts。代际用于进程内重启/停止。
pub(crate) fn spawn_lock_heartbeat() {
    let gen = LOCK_GEN.fetch_add(1, Ordering::SeqCst) + 1;
    tokio::spawn(async move {
        loop {
            tokio::time::sleep(Duration::from_secs(2)).await;
            if LOCK_GEN.load(Ordering::SeqCst) != gen || !LOCK_OWNED.load(Ordering::SeqCst) {
                return;
            }
            refresh_gateway_lock();
        }
    });
}

/// 生成旧版成员注册密钥（64 hex，覆盖旧值）
pub(crate) fn ensure_gateway_secret() -> anyhow::Result<()> {
    let secret = random_secret();
    let path = gateway_secret_path()?;
    atomic_write_private(&path, secret.as_bytes())
}

pub(crate) fn read_gateway_secret() -> Option<String> {
    let path = gateway_secret_path().ok()?;
    let secret = std::fs::read_to_string(path).ok()?;
    let trimmed = secret.trim();
    if trimmed.is_empty() {
        None
    } else {
        Some(trimmed.to_string())
    }
}

/// 新版 IPC 密钥：首次启动生成；已有则复用（Broker 重启后 GUI 仍可连接）
pub(crate) fn ensure_broker_secret() -> anyhow::Result<String> {
    let path = broker_secret_path()?;
    if let Ok(text) = std::fs::read_to_string(&path) {
        let trimmed = text.trim();
        if !trimmed.is_empty() {
            return Ok(trimmed.to_string());
        }
    }
    let secret = random_secret();
    atomic_write_private(&path, secret.as_bytes())?;
    Ok(secret)
}

pub(crate) fn read_broker_secret() -> Option<String> {
    let path = broker_secret_path().ok()?;
    let text = std::fs::read_to_string(path).ok()?;
    let trimmed = text.trim();
    if trimmed.is_empty() {
        None
    } else {
        Some(trimmed.to_string())
    }
}

fn random_secret() -> String {
    format!(
        "{}{}",
        uuid::Uuid::new_v4().simple(),
        uuid::Uuid::new_v4().simple()
    )
}

/// 原子写 + 仅当前用户可读（Unix 0600；Windows 使用默认用户 ACL）
pub(crate) fn atomic_write_private(path: &std::path::Path, data: &[u8]) -> anyhow::Result<()> {
    let dir = path
        .parent()
        .ok_or_else(|| anyhow::anyhow!("no parent for {}", path.display()))?;
    std::fs::create_dir_all(dir)?;
    let tmp = dir.join(format!(
        ".{}.{}.tmp",
        path.file_name().and_then(|s| s.to_str()).unwrap_or("secret"),
        uuid::Uuid::new_v4().simple()
    ));
    {
        let mut f = std::fs::File::create(&tmp)?;
        f.write_all(data)?;
        f.sync_all()?;
    }
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let _ = std::fs::set_permissions(&tmp, std::fs::Permissions::from_mode(0o600));
    }
    // Windows: rename 到已存在目标会失败（与 Unix 语义不同）。先短重试，
    // 再退回 remove+rename（短暂缺失窗口可接受：密钥缺失会重新生成，
    // 生命周期标记缺失按 running/允许拉起处理）。
    if std::fs::rename(&tmp, path).is_err() {
        std::thread::sleep(std::time::Duration::from_millis(20));
        if std::fs::rename(&tmp, path).is_err() {
            let _ = std::fs::remove_file(path);
            std::fs::rename(&tmp, path)?;
        }
    }
    Ok(())
}

/// 探测本机是否有可用新版 Broker（GET /gateway.ok）
pub(crate) async fn probe_gateway(port: u16) -> bool {
    tokio::time::timeout(Duration::from_secs(1), async {
        use tokio::io::{AsyncReadExt, AsyncWriteExt};
        let Ok(mut stream) = tokio::net::TcpStream::connect(("127.0.0.1", port)).await else {
            return false;
        };
        let req = format!("GET /gateway.ok HTTP/1.0\r\nHost: 127.0.0.1:{}\r\n\r\n", port);
        if stream.write_all(req.as_bytes()).await.is_err() {
            return false;
        }
        let mut buf = vec![0u8; 512];
        let mut total = Vec::new();
        loop {
            let n = stream.read(&mut buf).await.unwrap_or(0);
            if n == 0 {
                break;
            }
            total.extend_from_slice(&buf[..n]);
            if total.len() >= 512 {
                break;
            }
        }
        let body = String::from_utf8_lossy(&total);
        body.starts_with("HTTP/") && body.contains(" 200 ") && body.contains("\"gateway\":true")
    })
    .await
    .unwrap_or(false)
}

/// 判断端口是否被旧版 CC-Box 占用（返回首页含 "CC-Box" 且无 /gateway.ok）
pub(crate) async fn legacy_gateway_owns_port(port: u16) -> bool {
    if probe_gateway(port).await {
        return false;
    }
    crate::remote_server::probe_host(port).await
}

/// OS 排他单实例锁：成功返回 guard（持有到进程退出；崩溃自动释放）
pub(crate) struct BrokerInstanceGuard {
    _lock: crate::file_lock::FileLock,
}

impl BrokerInstanceGuard {
    pub(crate) fn acquire() -> anyhow::Result<Self> {
        let path = broker_lock_path()?;
        if let Some(dir) = path.parent() {
            std::fs::create_dir_all(dir)?;
        }
        let lock = crate::file_lock::FileLock::try_exclusive(&path)
            .map_err(|_| anyhow::anyhow!("another broker instance is running"))?;
        let info = serde_json::json!({
            "pid": std::process::id(),
            "ts": now_ms(),
        });
        let _ = std::fs::write(&path, info.to_string());
        Ok(Self { _lock: lock })
    }
}
