mod pty;
mod pty_decoder;
mod commands;
mod store;
mod checks;
mod mcp;
mod logger;
mod hook_events;
mod hook_server;
mod hook_config;
mod installer;
mod providers;
mod platform;
mod file_lock;
mod remote_state;
mod remote_server;
mod remote_client;
mod gateway;
mod gateway_proto;
mod broker;
mod broker_client;
mod file_transfer;
mod rpc_dispatch;
mod events;
#[cfg(test)]
mod tests;

use tauri::Manager;
use tauri::Emitter;
#[cfg(target_os = "macos")]
use tauri::menu::MenuBuilder;

/// macOS 原生 Edit 菜单与空菜单（输入框编辑快捷键切换用，setup 构建一次）
#[cfg(target_os = "macos")]
pub(crate) static NATIVE_EDIT_MENUS: OnceLock<(
    tauri::menu::Menu<tauri::Wry>,
    tauri::menu::Menu<tauri::Wry>,
)> = OnceLock::new();

/// 全局缓存环境检查结果（setup 前执行，仅一次）
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{LazyLock, Mutex, OnceLock};

/// GUI 主进程句柄：Broker 要求关闭/重启窗口时由 broker_client 调用。
static GUI_APP_HANDLE: OnceLock<tauri::AppHandle> = OnceLock::new();
/// 新建窗口注册后 Broker 很快下发 openRemote；前端可能尚未挂载监听。
/// 记录最近一次请求，前端启动时取走（5s 内有效）。
static PENDING_OPEN_REMOTE_MS: AtomicU64 = AtomicU64::new(0);
/// 托盘「更新到新版本」请求（与 openRemote 同机制：前端尚未挂载时暂存 5s）
static PENDING_OPEN_UPDATE_MS: AtomicU64 = AtomicU64::new(0);

fn now_ms() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_millis() as u64)
        .unwrap_or(0)
}

/// 结束本 GUI：先杀掉全部 PTY，再退出 Tauri 事件循环。
/// Broker 下发的 `WindowCommand::Close` 走这里（测试环境无窗口句柄时安全返回）。
pub(crate) fn request_gui_exit() {
    if let Some(manager) = pty::get_pty_manager() {
        manager.kill_all();
    }
    match GUI_APP_HANDLE.get() {
        Some(handle) => {
            log::info!("[gui] exit requested by broker");
            handle.exit(0);
        }
        None => log::warn!("[gui] exit requested before app handle init; ignored"),
    }
}

/// 当前 GUI 窗口标题（注册时上报给 Broker，供托盘窗口列表展示）。
pub(crate) fn gui_window_title() -> Option<String> {
    let handle = GUI_APP_HANDLE.get()?;
    let window = handle.get_webview_window("main")?;
    window
        .title()
        .ok()
        .map(|t| t.trim().to_string())
        .filter(|t| !t.is_empty())
}

/// 还原、显示并聚焦本窗口（托盘“显示”）。
pub(crate) fn request_gui_show() {
    if let Some(handle) = GUI_APP_HANDLE.get() {
        if let Some(window) = handle.get_webview_window("main") {
            let _ = window.show();
            let _ = window.unminimize();
            let _ = window.set_focus();
        }
    }
}

/// 请求前端打开远程连接面板（托盘“远程连接”）。
pub(crate) fn request_gui_open_remote() {
    PENDING_OPEN_REMOTE_MS.store(now_ms(), Ordering::SeqCst);
    if let Some(handle) = GUI_APP_HANDLE.get() {
        let _ = handle.emit("open-remote-panel", ());
    }
}

/// 前端启动时领取尚未消费的 openRemote 请求（5s 内有效）。
pub(crate) fn take_pending_open_remote() -> bool {
    let ts = PENDING_OPEN_REMOTE_MS.swap(0, Ordering::SeqCst);
    ts != 0 && now_ms().saturating_sub(ts) < 5_000
}

/// 请求前端打开设置 → 更新（托盘「更新到新版本」）。
pub(crate) fn request_gui_open_update() {
    PENDING_OPEN_UPDATE_MS.store(now_ms(), Ordering::SeqCst);
    if let Some(handle) = GUI_APP_HANDLE.get() {
        let _ = handle.emit("open-update", ());
    }
}

/// 前端启动时领取尚未消费的 openUpdate 请求（5s 内有效）。
pub(crate) fn take_pending_open_update() -> bool {
    let ts = PENDING_OPEN_UPDATE_MS.swap(0, Ordering::SeqCst);
    ts != 0 && now_ms().saturating_sub(ts) < 5_000
}
static CHECK_RESULTS: LazyLock<Mutex<Vec<checks::CheckResult>>> = LazyLock::new(|| {
    let result = checks::run_checks();
    for failed in result.failed_checks() {
        log::error!("[Check Failed] {}: {}", failed.name, failed.message);
    }
    if result.all_passed() {
        log::info!("Environment checks passed");
    }
    Mutex::new(result.checks)
});

/// 获取缓存的检查结果
pub fn get_check_results() -> Vec<checks::CheckResult> {
    CHECK_RESULTS.lock().unwrap().clone()
}

/// 重新运行检查并更新缓存
pub fn rerun_checks() -> Vec<checks::CheckResult> {
    let result = checks::run_checks();
    for failed in result.failed_checks() {
        log::error!("[Check Failed] {}: {}", failed.name, failed.message);
    }
    if result.all_passed() {
        log::info!("Environment checks passed");
    }
    let mut cache = CHECK_RESULTS.lock().unwrap();
    *cache = result.checks.clone();
    result.checks
}

/// Broker 模式：纯 tokio 无头进程（无 Tauri 窗口/托盘初始化）。
pub fn run_broker_headless(autostart: bool) {
    let rt = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .expect("failed to build broker runtime");
    rt.block_on(broker::run_headless(autostart));
}

/// Broker 模式入口：Windows/macOS 带托盘，Linux 无头（托盘缺失时优雅降级）。
pub fn run_broker(autostart: bool) {
    #[cfg(any(target_os = "windows", target_os = "macos"))]
    {
        run_broker_with_tray(autostart);
    }
    #[cfg(not(any(target_os = "windows", target_os = "macos")))]
    {
        run_broker_headless(autostart);
    }
}

/// `--quit-broker`：无托盘/无窗口场景下发送退出请求（会关闭所有窗口）。
pub fn quit_broker_blocking() -> Result<(), String> {
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .map_err(|e| e.to_string())?;
    rt.block_on(broker_client::control_direct(
        gateway_proto::BrokerOp::Quit,
    ))
    .map(|_| ())
}

/// 统一构造 Tauri Context（Broker 托盘 / GUI 共用）。
///
/// macOS 上 `generate_context!()` 会经 embed_plist 生成
/// `#[no_mangle] static _EMBED_INFO_PLIST`，同一二进制重复展开即重复符号。
/// 因此宏只能在单一位置展开，两种启动模式复用同一构造函数。
fn app_context() -> tauri::Context<tauri::Wry> {
    tauri::generate_context!()
}

#[cfg(any(target_os = "windows", target_os = "macos"))]
fn run_broker_with_tray(autostart: bool) {
    tauri::Builder::default()
        // 托盘破坏性操作的原生确认对话框
        .plugin(tauri_plugin_dialog::init())
        // 托盘更新提示：Broker 常驻期间定期检查新版本
        .plugin(tauri_plugin_updater::Builder::new().build())
        .setup(move |app| {
            #[cfg(target_os = "macos")]
            app.set_activation_policy(tauri::ActivationPolicy::Accessory);

            let handle = app.handle().clone();
            // 托盘先装上：即使服务启动失败/已有实例，用户也能看到并退出
            if let Err(e) = broker::tray::install(&handle) {
                log::warn!("[broker] tray unavailable: {}", e);
            }
            // 托盘更新红点：Broker 常驻期间定期检查新版本
            broker::update::spawn_checker(handle.clone());

            // Broker 服务跑在独立 tokio 线程；Tauri 事件循环只负责托盘
            std::thread::spawn(move || {
                let rt = tokio::runtime::Builder::new_multi_thread()
                    .enable_all()
                    .build()
                    .expect("failed to build broker runtime");
                rt.block_on(async move {
                    broker::run_headless(autostart).await;
                    handle.exit(0);
                });
            });
            Ok(())
        })
        .build(app_context())
        .expect("error while building broker application")
        .run(|_app, event| {
            // 只有外部事件导致的 ExitRequested（code=None）才阻止；
            // app.exit(0) 必须能真正退出
            if let tauri::RunEvent::ExitRequested { code: None, api, .. } = event {
                api.prevent_exit();
            }
        });
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run(initial_dir: Option<String>, new_window: bool) {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_clipboard_manager::init())
        .plugin(tauri_plugin_updater::Builder::new().build())
        .plugin(tauri_plugin_process::init())
        .on_window_event(|_window, event| {
            match event {
                tauri::WindowEvent::CloseRequested { .. } => {
                    log::info!("Window close requested, cleaning up PTYs...");
                    if let Some(manager) = pty::get_pty_manager() {
                        manager.kill_all();
                    }
                }
                _ => {}
            }
        })
        .setup(move |app| {
            logger::init();

            // 主窗口由代码创建（tauri.conf.json 的 windows 为空，供 Broker
            // 模式复用同一 Tauri 上下文但不弹窗）
            let window_builder = tauri::WebviewWindowBuilder::new(
                app,
                "main",
                tauri::WebviewUrl::App("index.html".into()),
            )
            .title("CC-Box")
            .inner_size(800.0, 600.0)
            .min_inner_size(500.0, 400.0)
            .resizable(true);
            #[cfg(target_os = "macos")]
            let window_builder =
                window_builder.title_bar_style(tauri::TitleBarStyle::Overlay);
            if let Err(e) = window_builder.build() {
                log::error!("Failed to create main window: {}", e);
                return Err(e.into());
            }
            let _ = GUI_APP_HANDLE.set(app.handle().clone());

            // Provider 配置一次性迁移（schema v1 → v2）。
            // 放在启动阶段，保证读命令（含远程只读 RPC）不产生隐式写盘。
            if let Err(e) = providers::ensure_providers_schema() {
                log::error!("[providers] schema migration failed: {}", e);
            }

            // macOS: 原生 Edit 菜单——输入框 Cmd+C/V/X/A/Z 经系统 paste:/copy:
            // selector 原生响应（WKWebView 中粘贴没有 keydown 默认行为，无菜单即失效）。
            // 菜单 key equivalent 会在按键到达 WebView 前被系统拦截，终端持焦时前端调
            // set_native_edit_menu(false) 切到空菜单，xterm 按键处理完全不受影响
            #[cfg(target_os = "macos")]
            {
                let edit = MenuBuilder::new(app)
                    .undo()
                    .redo()
                    .separator()
                    .cut()
                    .copy()
                    .paste()
                    .select_all()
                    .build()?;
                // 空菜单里的禁用占位项（无 key equivalent，不拦截任何按键），
                // 避免终端持焦时菜单栏完全空白
                use tauri::menu::MenuItemBuilder;
                let placeholder = MenuItemBuilder::with_id("cc-box-placeholder", "CC-Box")
                    .enabled(false)
                    .build(app)?;
                let plain = MenuBuilder::new(app).item(&placeholder).build()?;
                let _ = app.set_menu(edit.clone());
                let _ = NATIVE_EDIT_MENUS.set((edit, plain));
            }

            pty::init_pty_manager(app.handle().clone());
            // 远程事件转发到桌面前端（session-taken-over / session-reclaimed /
            // remote-session-created）。测试构建排除：emit 闭包会拉入 comctl32 v6
            // (SetWindowSubclass) 导入，测试 exe 无 side-by-side manifest 导致
            // 加载失败 (STATUS_ENTRYPOINT_NOT_FOUND)
            #[cfg(not(test))]
            {
                let handle = app.handle().clone();
                remote_state::set_event_hook(Box::new(move |name, id, data| {
                    // 桌面通知（远程广播由 remote_state::emit_event* 系列按受众精准分发）
                    use tauri::Emitter;
                    let _ = handle.emit(name, serde_json::json!({ "id": id, "data": data }));
                }));
            }
            log::info!("PTY manager initialized");

            // Windows: 移除原生标题栏（UI 相关，尽早执行）
            #[cfg(target_os = "windows")]
            {
                if let Some(win) = app.get_webview_window("main") {
                    let _ = win.set_decorations(false);
                }
            }

            // Windows: 禁用 WebView2 浏览器加速键（Ctrl+L/D 等不再被 WebView2 拦截）
            #[cfg(target_os = "windows")]
            {
                if let Some(ww) = app.get_webview_window("main") {
                    let _ = ww.with_webview(|webview| {
                        use windows_core::Interface;
                        use webview2_com::Microsoft::Web::WebView2::Win32::ICoreWebView2Settings3;
                        let controller = webview.controller();
                        if let Ok(core_wv) = unsafe { controller.CoreWebView2() } {
                            if let Ok(settings) = unsafe { core_wv.Settings() } {
                                if let Ok(settings3) = settings.cast::<ICoreWebView2Settings3>() {
                                    if let Err(e) = unsafe { settings3.SetAreBrowserAcceleratorKeysEnabled(false) } {
                                        log::warn!("Failed to disable browser accelerator keys: {}", e);
                                    } else {
                                        log::info!("WebView2 browser accelerator keys disabled");
                                    }
                                }
                            }
                        }
                    });
                }
            }

            // 异步执行非关键初始化（不阻塞 UI 显示）
            let handle = app.handle().clone();
            tauri::async_runtime::spawn(async move {
                // Plugin 文件部署（版本匹配时跳过）
                if let Err(e) = hook_config::ensure_plugin_files() {
                    log::warn!("Failed to create plugin files: {}. Hook monitoring may not work.", e);
                }
                // Hook HTTP 服务器
                hook_server::init(handle.clone()).await;

                // 手动启动 GUI = 重新打开 CC-Box：清除上次“退出 CC-Box”的
                // 持久抑制，恢复 Broker（托盘退出后没有 GUI 会再拉起它）。
                broker::lifecycle::clear_user_quit();
                // GUI 不再直接持有 LAN/relay：连接/拉起本机 Broker，注册为成员。
                // 即使远程开关为关也保持 Broker 在线（托盘可再次开启远程）。
                broker_client::spawn_connect_loop();

                // 文件传输清理：启动扫一次（删崩溃遗留的上传临时文件）
                file_transfer::startup_cleanup();
                // 定时 TTL sweep（惰性清理的兜底：连接断开后再无 file_ 命令触发）
                tauri::async_runtime::spawn(async move {
                    let mut interval =
                        tokio::time::interval(std::time::Duration::from_secs(600));
                    interval
                        .set_missed_tick_behavior(tokio::time::MissedTickBehavior::Delay);
                    loop {
                        interval.tick().await;
                        file_transfer::sweep_stale();
                    }
                });

                // 启动模式事件：右键目录打开 / 新建窗口（新建窗口停留在项目页，
                // 不自动进入终端）
                if initial_dir.is_some() || new_window {
                    tokio::time::sleep(std::time::Duration::from_millis(500)).await;
                    if let Some(dir) = initial_dir {
                        let _ = handle.emit("open-directory", dir);
                    }
                    if new_window {
                        let _ = handle.emit(
                            "launch-mode",
                            serde_json::json!({ "newWindow": true }),
                        );
                    }
                }
            });

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::get_home_data,
            commands::get_check_results,
            commands::run_checks,
            commands::pty_spawn,
            commands::pty_input,
            commands::pty_resize,
            commands::pty_kill,
            commands::pty_kill_all,
            commands::pty_claim,
            commands::rename_session,
            commands::get_projects,
            commands::hide_project,
            commands::unhide_project,
            commands::list_directories,
            commands::get_project_info,
            commands::get_sessions,
            commands::get_session_count,
            commands::get_all_recent_sessions,
            commands::get_session_details,
            commands::search_session_messages,
            commands::get_app_config,
            commands::update_app_config,
            commands::get_system_locale,
            commands::get_default_claude_options,
            commands::save_default_claude_options,
            commands::save_last_project,
            commands::open_in_file_manager,
            commands::get_project_config,
            commands::get_all_agents,
            commands::get_all_skills,
            commands::get_all_mcp_servers,
            commands::get_all_plugins,
            commands::set_skill_enabled,
            commands::set_agent_enabled,
            commands::set_mcp_server_enabled,
            commands::set_plugin_enabled,
            commands::get_mcp_server_detail,
            commands::test_communication,
            commands::get_app_path,
            commands::update_window_title,
            commands::take_pending_open_remote,
            commands::take_pending_open_update,
            commands::resume_broker_connection,
            commands::spawn_new_instance,
            commands::set_native_edit_menu,
            commands::log_message,
            installer::get_latest_versions,
            installer::check_installed_versions,
            installer::check_claude_cli_update,
            installer::check_claude_running,
            installer::kill_claude_processes,
            installer::download_and_install_claude,
            installer::cancel_auto_install,
            // 本地只读版本查询经 commands.rs 统一入口（远程 RPC 白名单同源）
            commands::get_installed_claude_version,
            installer::list_claude_versions,
            installer::download_claude_version,
            installer::cancel_claude_download,
            installer::install_claude_version,
            #[cfg(target_os = "windows")]
            installer::download_and_install_git,
            // Provider Commands
            commands::get_providers_config,
            commands::open_claude_user_settings,
            commands::save_providers_config,
            commands::activate_provider,
            commands::create_provider,
            commands::update_provider,
            commands::delete_provider,
            commands::update_provider_sort_order,
            commands::update_common_config,
            commands::check_cc_switch_db_exists,
            commands::import_from_cc_switch,
            commands::test_provider_connection,
            // Remote Commands
            commands::get_remote_status,
            commands::set_remote_enabled,
            commands::set_relay_url,
            commands::set_broker_autostart,
            commands::refresh_relay_pair_code,
            commands::rotate_remote_secret,
            commands::prepare_for_update,
            // File Transfer Commands（远程浏览器模式；桌面 invoke 走 conn_id=None 拒绝上传/下载）
            commands::file_transfer_limits,
            commands::file_list,
            commands::file_upload_begin,
            commands::file_upload_chunk,
            commands::file_upload_finish,
            commands::file_upload_abort,
            commands::file_download_begin,
            commands::file_download_chunk,
            commands::file_download_ticket,
            commands::file_download_ticket_chunk,
        ])
        .run(app_context())
        .expect("error while running tauri application");
}
