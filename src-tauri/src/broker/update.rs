//! Broker 侧更新检查：托盘「更新到新版本」菜单项与图标红点的数据来源。
//!
//! 设置→更新由用户在 GUI 中手动检查；Broker 常驻（窗口全关也在线），
//! 由它定期检查才能让托盘主动提示。仅 Windows/macOS（带托盘）启动。
//! 检查失败静默保留上次结果（开机网络未就绪时常见）。

use parking_lot::Mutex;
use std::time::Duration;

/// 有新版本时为其版本号；None = 无更新/尚未成功检查
static AVAILABLE: Mutex<Option<String>> = Mutex::new(None);

/// 当前可获得的新版本号（托盘渲染用）
pub(crate) fn available() -> Option<String> {
    AVAILABLE.lock().clone()
}

/// 自动更新开关解析：config 缺字段（旧版本写入的 config）= 默认开启
pub(crate) fn resolve_auto_update(cfg: Option<bool>) -> bool {
    cfg.unwrap_or(true)
}

/// 启动周期检查：开机网络/代理未就绪时先等 30s，之后每 2h 一次
pub(crate) fn spawn_checker(app: tauri::AppHandle) {
    tauri::async_runtime::spawn(async move {
        tokio::time::sleep(Duration::from_secs(30)).await;
        loop {
            match check_once(&app).await {
                Ok(Some(version)) => {
                    let changed = AVAILABLE.lock().as_deref() != Some(version.as_str());
                    if changed {
                        log::info!("[broker-update] new version available: v{}", version);
                    }
                    *AVAILABLE.lock() = Some(version);
                }
                Ok(None) => *AVAILABLE.lock() = None,
                // 失败保留上次结果：网络抖动不应让红点闪烁
                Err(e) => log::debug!("[broker-update] check failed: {}", e),
            }
            tokio::time::sleep(Duration::from_secs(2 * 60 * 60)).await;
        }
    });
}

async fn check_once(app: &tauri::AppHandle) -> Result<Option<String>, String> {
    use tauri_plugin_updater::UpdaterExt;
    let updater = app.updater().map_err(|e| e.to_string())?;
    let update = updater.check().await.map_err(|e| e.to_string())?;
    Ok(update.map(|u| u.version.clone()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn Update_AutoUpdateDefaultOn_1() {
        // 旧 config / 字段缺失 = 默认开启
        assert!(resolve_auto_update(None));
        assert!(resolve_auto_update(Some(true)));
        assert!(!resolve_auto_update(Some(false)));
    }
}
