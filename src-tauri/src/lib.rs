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
mod remote_state;
mod remote_server;
mod remote_client;
mod file_transfer;
mod rpc_dispatch;
mod events;
#[cfg(test)]
mod tests;

use tauri::Manager;
use tauri::Emitter;
#[cfg(target_os = "macos")]
use tauri::menu::MenuBuilder;

/// 全局缓存环境检查结果（setup 前执行，仅一次）
use std::sync::LazyLock;
use std::sync::Mutex;
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

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run(initial_dir: Option<String>) {
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
        .setup(|app| {
            logger::init();

            // Provider 配置一次性迁移（schema v1 → v2）。
            // 放在启动阶段，保证读命令（含远程只读 RPC）不产生隐式写盘。
            if let Err(e) = providers::ensure_providers_schema() {
                log::error!("[providers] schema migration failed: {}", e);
            }

            // macOS: 注册原生 Copy 菜单项，使 Cmd+C 在 WebView 中生效
            #[cfg(target_os = "macos")]
            {
                let menu = MenuBuilder::new(app)
                    .copy()
                    .build()?;
                let _ = app.set_menu(menu);
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

                // 远程访问服务器（局域网直连，开关开启时才监听）
                remote_server::init(handle.clone()).await;

                // 远程中继出站客户端（配置了 relay_url 时自动连接）
                remote_client::init().await;

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

                // 如果通过命令行参数传入了目录（右键菜单打开），延迟 emit 事件给前端
                if let Some(dir) = initial_dir {
                    tokio::time::sleep(std::time::Duration::from_millis(500)).await;
                    let _ = handle.emit("open-directory", dir);
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
            commands::controller_takeover,
            commands::controller_status,
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
            commands::spawn_new_instance,
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
            commands::refresh_relay_pair_code,
            commands::rotate_remote_secret,
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
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
