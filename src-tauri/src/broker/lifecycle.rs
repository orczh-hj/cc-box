//! Broker 生命周期标记：区分 user_quit / update_in_progress。
//!
//! 文件 `~/.cc-box/broker.state.json` 由 Broker 原子写；所有 GUI 在自动拉起
//! Broker 前读取该标记，避免“退出 CC-Box 后又被残留 GUI 立刻拉起”和
//! “更新期间重新锁住待替换的可执行文件”。Broker 常驻，不再写 idle_exit；
//! 该状态仅为读取旧版本遗留标记而保留。

use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum LifecycleState {
    Running,
    UserQuit,
    UpdateInProgress,
    /// 旧版空闲回收标记：新版不再写入，仅兼容读取
    IdleExit,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LifecycleRecord {
    pub generation: u64,
    pub state: LifecycleState,
    #[serde(default)]
    pub app_version: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub from_version: Option<String>,
    #[serde(default)]
    pub pid: u32,
    #[serde(default)]
    pub ts: u128,
}

fn path() -> Option<PathBuf> {
    crate::store::get_gui_config_dir()
        .ok()
        .map(|d| d.join("broker.state.json"))
}

fn now_ms() -> u128 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_millis())
        .unwrap_or(0)
}

pub(crate) fn read() -> Option<LifecycleRecord> {
    let text = std::fs::read_to_string(path()?).ok()?;
    serde_json::from_str(&text).ok()
}

pub(crate) fn write(state: LifecycleState, from_version: Option<String>) -> std::io::Result<()> {
    let Some(path) = path() else {
        return Ok(());
    };
    let prev = read().map(|r| r.generation).unwrap_or(0);
    let rec = LifecycleRecord {
        generation: prev.saturating_add(1),
        state,
        app_version: env!("APP_VERSION").to_string(),
        from_version,
        pid: std::process::id(),
        ts: now_ms(),
    };
    let text = serde_json::to_vec(&rec).map_err(std::io::Error::other)?;
    super::ownership::atomic_write_private(&path, &text).map_err(|e| std::io::Error::other(e.to_string()))
}

pub(crate) fn clear() -> std::io::Result<()> {
    if let Some(path) = path() {
        match std::fs::remove_file(path) {
            Ok(()) => Ok(()),
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(()),
            Err(e) => Err(e),
        }
    } else {
        Ok(())
    }
}

/// GUI 自动拉起判定。`update_in_progress` 只在版本未变化且 5 分钟内有效；
/// `user_quit` 需要用户显式开启远程/启动后台服务来清除。
pub(crate) fn should_autostart() -> bool {
    let Some(rec) = read() else {
        return true;
    };
    match rec.state {
        LifecycleState::UserQuit => false,
        LifecycleState::UpdateInProgress => {
            let same_version = rec.from_version.as_deref() == Some(env!("APP_VERSION"));
            let fresh = now_ms().saturating_sub(rec.ts) < 5 * 60 * 1000;
            if same_version && fresh {
                false
            } else {
                let _ = clear();
                true
            }
        }
        LifecycleState::Running | LifecycleState::IdleExit => true,
    }
}

/// 用户显式开启远程/启动后台服务时清除 `user_quit`
pub(crate) fn clear_user_quit() {
    if read().map(|r| r.state == LifecycleState::UserQuit).unwrap_or(false) {
        let _ = clear();
    }
}
