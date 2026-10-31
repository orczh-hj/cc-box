//! Broker 侧更新流水线：检查 → 下载 → 判窗口 → 安装。
//!
//! 检查到新版本**无条件自动下载**（下载与安装解耦：下载期间用户照常使用，
//! 安装时机单独评估）；「自动更新」开关开启且零 GUI 窗口时自动安装重启。
//! 托盘「更新到 vX」手动更新复用同一条安装链（用户已二次确认，先关窗再装）。
//! 托盘菜单与图标红点的数据来源。仅 Windows/macOS（带托盘）启动。
//! 检查失败静默保留上次结果（开机网络未就绪时常见）。

use parking_lot::Mutex;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::Duration;
use tauri_plugin_updater::Update;

/// 已检查到的新版本号；None = 无更新/尚未成功检查（托盘渲染用）
static AVAILABLE: Mutex<Option<String>> = Mutex::new(None);

/// 已下载待安装的更新包（版本号 + 包字节，跨检查保留；版本换代时覆盖重下）。
/// Update 对象含裸指针非 Send，不入静态存储；安装时重新获取。
static DOWNLOADED: Mutex<Option<(String, Vec<u8>)>> = Mutex::new(None);

/// 安装进行中标志：防止周期检查与手动更新双安装重入
static INSTALLING: AtomicBool = AtomicBool::new(false);

/// 当前可获得的新版本号（托盘渲染用）
pub(crate) fn available() -> Option<String> {
    AVAILABLE.lock().clone()
}

/// 自动更新开关解析：config 缺字段（旧版本写入的 config）= 默认开启
pub(crate) fn resolve_auto_update(cfg: Option<bool>) -> bool {
    cfg.unwrap_or(true)
}

/// 自动更新开关当前值（读共享 config）
fn auto_update_enabled() -> bool {
    let cfg = crate::store::get_app_config().ok().and_then(|c| c.auto_update);
    resolve_auto_update(cfg)
}

/// 单次检查后的决策（纯函数，可单测）。
/// `has_update_ver` = 线上可用版本；`downloaded_ver` = 已下载版本。
fn decide(
    has_update_ver: Option<&str>,
    downloaded_ver: Option<&str>,
    auto_enabled: bool,
    window_count: usize,
    installing: bool,
) -> Decision {
    let Some(ver) = has_update_ver else {
        return Decision::None;
    };
    if downloaded_ver != Some(ver) {
        return Decision::Download;
    }
    if auto_enabled && window_count == 0 && !installing {
        Decision::Install
    } else {
        Decision::None
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Decision {
    None,
    Download,
    Install,
}

/// 启动周期检查：启动立即检查一次，之后每 2h 一次。
/// Update 对象非 Send，不能进 tauri::async_runtime::spawn 的 future；
/// 流水线整体跑在专用线程（自带 runtime，block_on 非 Send future 合法）。
pub(crate) fn spawn_checker(app: tauri::AppHandle) {
    std::thread::spawn(move || {
        let rt = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .expect("failed to build updater runtime");
        rt.block_on(async move {
            loop {
                run_cycle(&app).await;
                tokio::time::sleep(Duration::from_secs(2 * 60 * 60)).await;
            }
        });
    });
}

/// 托盘「检查更新」：专用线程立即执行一次周期逻辑（fire-and-forget）
pub(crate) fn check_now(app: tauri::AppHandle) {
    std::thread::spawn(move || {
        let rt = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .expect("failed to build updater runtime");
        rt.block_on(run_cycle(&app));
    });
}

/// 托盘「更新到 vX」：专用线程执行手动更新（关窗 → 确保已下载 → 安装重启）
pub(crate) fn run_manual_update(app: tauri::AppHandle) {
    std::thread::spawn(move || {
        let rt = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .expect("failed to build updater runtime");
        rt.block_on(manual_update(&app));
    });
}

/// 一次完整的「检查 → 下载 → 判窗口 → 安装」
async fn run_cycle(app: &tauri::AppHandle) {
    let has_update = match check_once(app).await {
        Ok(v) => v,
        // 失败保留上次结果：网络抖动不应让红点闪烁
        Err(e) => {
            log::debug!("[broker-update] check failed: {}", e);
            return;
        }
    };
    *AVAILABLE.lock() = has_update.clone();

    let downloaded_ver = DOWNLOADED.lock().as_ref().map(|(v, _)| v.clone());
    let window_count = crate::gateway::window_roster().len();
    let installing = INSTALLING.load(Ordering::SeqCst);
    match decide(
        has_update.as_deref(),
        downloaded_ver.as_deref(),
        auto_update_enabled(),
        window_count,
        installing,
    ) {
        Decision::None => {}
        Decision::Download => {
            if let Some(update) = fetch_update(app).await {
                match update.download(|_, _| {}, || {}).await {
                    Ok(bytes) => {
                        log::info!(
                            "[broker-update] downloaded v{} ({} windows open, install deferred)",
                            update.version,
                            window_count
                        );
                        *DOWNLOADED.lock() = Some((update.version.clone(), bytes));
                    }
                    Err(e) => log::warn!("[broker-update] download failed: {}", e),
                }
            }
        }
        Decision::Install => {
            if let Some((version, bytes)) = DOWNLOADED.lock().take() {
                if let Some(update) = fetch_update(app).await {
                    install_and_restart(app, &version, &update, &bytes).await;
                } else {
                    // Update 对象取回失败：字节放回，下次检查再试
                    *DOWNLOADED.lock() = Some((version, bytes));
                }
            }
        }
    }
}

/// 托盘「更新到 vX」手动更新：关窗 → 确保已下载 → 安装重启。
/// 用户已二次确认，有窗口也安装（先让窗口整体退出）。
async fn manual_update(app: &tauri::AppHandle) {
    if INSTALLING.swap(true, Ordering::SeqCst) {
        log::warn!("[broker-update] manual update: already in progress");
        return;
    }
    // guard 结构保证失败路径 INSTALLING 复位；成功路径 restart 不返回
    struct Guard;
    impl Drop for Guard {
        fn drop(&mut self) {
            INSTALLING.store(false, Ordering::SeqCst);
        }
    }
    let _guard = Guard;

    crate::broker::windows::close_all_windows(Duration::from_secs(10)).await;

    // 已下载当前版本则直接用；否则现场检查 + 下载
    let has_update = available();
    let cached = DOWNLOADED
        .lock()
        .as_ref()
        .filter(|(v, _)| Some(v.as_str()) == has_update.as_deref())
        .map(|(_, b)| b.clone());
    let (version, bytes) = match cached {
        Some(bytes) => (has_update.unwrap_or_default(), bytes),
        None => {
            let Ok(Some(ver)) = check_once(app).await else {
                log::warn!("[broker-update] manual update: no update available");
                return;
            };
            let Some(update) = fetch_update(app).await else {
                log::warn!("[broker-update] manual update: fetch failed");
                return;
            };
            let Ok(bytes) = update.download(|_, _| {}, || {}).await else {
                log::warn!("[broker-update] manual update: download failed");
                return;
            };
            *DOWNLOADED.lock() = Some((ver.clone(), bytes.clone()));
            (ver, bytes)
        }
    };
    log::info!("[broker-update] manual update: installing v{}", version);
    let Some(update) = fetch_update(app).await else {
        log::warn!("[broker-update] manual update: fetch failed before install");
        return;
    };
    install_and_restart(app, &version, &update, &bytes).await;
}

/// 写 update_in_progress → 安装 → 重启（成功时 restart 不返回）。
/// 安装失败：清标记继续运行（INSTALLING 复位由调用方 guard 负责）。
async fn install_and_restart(
    app: &tauri::AppHandle,
    version: &str,
    update: &Update,
    bytes: &[u8],
) {
    if let Err(e) = crate::broker::lifecycle::write(
        crate::broker::lifecycle::LifecycleState::UpdateInProgress,
        Some(env!("APP_VERSION").to_string()),
    ) {
        log::warn!("[broker-update] write lifecycle failed: {}", e);
    }
    match update.install(bytes) {
        Ok(()) => {
            log::info!("[broker-update] installed v{}; restarting", version);
            app.restart();
        }
        Err(e) => {
            let _ = crate::broker::lifecycle::clear();
            log::warn!("[broker-update] install failed: {}", e);
        }
    }
}

async fn check_once(app: &tauri::AppHandle) -> Result<Option<String>, String> {
    use tauri_plugin_updater::UpdaterExt;
    let updater = app.updater().map_err(|e| e.to_string())?;
    let update = updater.check().await.map_err(|e| e.to_string())?;
    Ok(update.map(|u| u.version.clone()))
}

/// 拉取 Update 对象（含下载地址），供 download 使用
async fn fetch_update(app: &tauri::AppHandle) -> Option<Update> {
    use tauri_plugin_updater::UpdaterExt;
    let updater = app.updater().ok()?;
    updater.check().await.ok()?
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

    #[test]
    fn Update_DownloadAlways_2() {
        // 有新版本且开关关/有窗口：仍要下载，不安装
        assert_eq!(decide(Some("1.0"), None, false, 3, false), Decision::Download);
        assert_eq!(decide(Some("1.0"), None, true, 3, false), Decision::Download);
    }

    #[test]
    fn Update_InstallOnlyZeroWindowsEnabled_2() {
        assert_eq!(decide(Some("1.0"), Some("1.0"), true, 0, false), Decision::Install);
        // 有窗口等下次
        assert_eq!(decide(Some("1.0"), Some("1.0"), true, 2, false), Decision::None);
        // 开关关不自动装
        assert_eq!(decide(Some("1.0"), Some("1.0"), false, 0, false), Decision::None);
        // 安装中不重入
        assert_eq!(decide(Some("1.0"), Some("1.0"), true, 0, true), Decision::None);
        // 无更新无事可做
        assert_eq!(decide(None, None, true, 0, false), Decision::None);
    }

    #[test]
    fn Update_VersionSwap_2() {
        // 已下载 0.9、线上已是 1.0：需重新下载，不得安装旧版
        assert_eq!(decide(Some("1.0"), Some("0.9"), true, 0, false), Decision::Download);
    }
}
