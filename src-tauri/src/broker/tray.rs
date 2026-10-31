//! Broker 托盘/菜单栏：CC-Box 管理器
//!
//! - Windows/macOS：Tauri 原生托盘（非 Linux 才编译 tray-icon）
//! - Linux：无头运行（无系统托盘），窗口/远程/自启动由桌面设置或远程面板管理
//!
//! 菜单：CC-Box 标题、各窗口（一级项，单击直接显示/聚焦）、开机自启动、
//! 新建窗口、远程连接、刷新远程密钥、关闭/开启远程连接、退出 CC-Box。
//! 破坏性动作统一弹原生确认对话框；菜单由 Broker 状态与成员名单驱动（2s 轮询）。

use std::collections::HashMap;
use std::time::Duration;

use crate::gateway::WindowInfo;
use crate::gateway_proto::{BrokerOp, BrokerStatus};

const TRAY_ID: &str = "cc-box-broker";
const TITLE: &str = "CC-Box";

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct TrayWindow {
    pub id: String,
    pub label: String,
    pub show_id: String,
    /// v2 + 有 PID 才能下发 Show；旧版窗口仅展示
    pub enabled: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct TrayLabels {
    pub title: String,
    pub empty_windows: String,
    pub autostart: String,
    pub autostart_checked: bool,
    pub new_window: String,
    pub toggle: String,
    pub remote_connect: String,
    pub rotate_secret: String,
    pub rotate_enabled: bool,
    pub quit: String,
    /// 有新版本时的菜单项文案（None = 不显示该项、托盘图标无红点）
    pub update: Option<String>,
    /// 无更新时显示的「检查更新」文案（有更新时由 build_menu 跳过渲染）
    pub check_update: String,
    pub windows: Vec<TrayWindow>,
}

fn zh() -> bool {
    sys_locale::get_locale()
        .map(|l| l.to_ascii_lowercase().starts_with("zh"))
        .unwrap_or(false)
}

/// 纯函数：根据 Broker 状态与窗口名单生成菜单文案（可单测）。
/// 窗口是一级菜单项，名称为 GUI 真实标题；同名追加序号。
pub(crate) fn build_model(
    status: Option<&BrokerStatus>,
    windows: &[WindowInfo],
    autostart: bool,
    update: Option<&str>,
    zh: bool,
) -> TrayLabels {
    let enabled = status.map(|s| s.enabled).unwrap_or(false);
    let legacy = status.map(|s| s.legacy_owned).unwrap_or(false);

    let mut seen: HashMap<String, u32> = HashMap::new();
    let windows = windows
        .iter()
        .map(|w| {
            let base = if w.title.trim().is_empty() {
                w.name.clone()
            } else {
                w.title.clone()
            };
            let n = seen.entry(base.clone()).or_insert(0);
            *n += 1;
            let mut label = if *n == 1 {
                base
            } else {
                format!("{} ({})", base, n)
            };
            if !w.closable {
                label = if zh {
                    format!("{}（旧版，无法控制）", label)
                } else {
                    format!("{} (legacy, cannot control)", label)
                };
            }
            TrayWindow {
                id: w.id.clone(),
                label,
                show_id: format!("win:{}:show", w.id),
                enabled: w.closable,
            }
        })
        .collect();

    TrayLabels {
        title: TITLE.to_string(),
        empty_windows: if zh {
            "（暂无窗口）".to_string()
        } else {
            "(no windows)".to_string()
        },
        autostart: if zh {
            "开机自启动".to_string()
        } else {
            "Launch at login".to_string()
        },
        autostart_checked: autostart,
        new_window: if zh {
            "新建窗口".to_string()
        } else {
            "New window".to_string()
        },
        toggle: if enabled {
            if zh {
                "关闭远程连接".to_string()
            } else {
                "Stop remote".to_string()
            }
        } else if zh {
            "开启远程连接".to_string()
        } else {
            "Start remote".to_string()
        },
        remote_connect: if zh {
            "远程连接".to_string()
        } else {
            "Connect remotely".to_string()
        },
        rotate_secret: if zh {
            "刷新远程密钥".to_string()
        } else {
            "Rotate remote secret".to_string()
        },
        rotate_enabled: !legacy,
        quit: if zh {
            "退出 CC-Box".to_string()
        } else {
            "Quit CC-Box".to_string()
        },
        update: update.map(|v| {
            if zh {
                format!("更新到 v{}", v)
            } else {
                format!("Update to v{}", v)
            }
        }),
        check_update: if zh {
            "检查更新".to_string()
        } else {
            "Check for updates".to_string()
        },
        windows,
    }
}

fn current_model() -> TrayLabels {
    let status = crate::broker::status_snapshot();
    let windows = crate::gateway::window_roster();
    let autostart = status
        .as_ref()
        .map(|s| s.autostart)
        .unwrap_or_else(crate::broker::autostart::is_enabled);
    let update = crate::broker::update::available();
    build_model(status.as_ref(), &windows, autostart, update.as_deref(), zh())
}

fn window_signature(model: &TrayLabels) -> String {
    let windows = model
        .windows
        .iter()
        .map(|w| format!("{}:{}:{}", w.id, w.label, w.enabled))
        .collect::<Vec<_>>()
        .join("|");
    // 更新项出现/消失/版本变化都要重建菜单
    format!("{}#{}", windows, model.update.as_deref().unwrap_or(""))
}

#[cfg(not(target_os = "linux"))]
struct MenuHandles {
    remote: tauri::menu::MenuItem<tauri::Wry>,
    rotate: tauri::menu::MenuItem<tauri::Wry>,
    autostart: tauri::menu::CheckMenuItem<tauri::Wry>,
}

#[cfg(not(target_os = "linux"))]
fn build_menu(
    app: &tauri::AppHandle,
    model: &TrayLabels,
) -> tauri::Result<(tauri::menu::Menu<tauri::Wry>, MenuHandles)> {
    use tauri::menu::{CheckMenuItem, IsMenuItem, Menu, MenuItem, PredefinedMenuItem};

    let title_item = MenuItem::with_id(app, "title", &model.title, false, None::<&str>)?;
    let autostart_item = CheckMenuItem::with_id(
        app,
        "autostart",
        &model.autostart,
        true,
        model.autostart_checked,
        None::<&str>,
    )?;
    let autostart_handle = autostart_item.clone();
    let new_item = MenuItem::with_id(app, "new", &model.new_window, true, None::<&str>)?;
    let remote_connect =
        MenuItem::with_id(app, "remote-connect", &model.remote_connect, true, None::<&str>)?;
    let rotate_item = MenuItem::with_id(
        app,
        "rotate",
        &model.rotate_secret,
        model.rotate_enabled,
        None::<&str>,
    )?;
    let rotate_handle = rotate_item.clone();
    let remote_item = MenuItem::with_id(app, "toggle", &model.toggle, true, None::<&str>)?;
    let remote_handle = remote_item.clone();
    let quit_item = MenuItem::with_id(app, "quit", &model.quit, true, None::<&str>)?;

    let mut items: Vec<Box<dyn IsMenuItem<tauri::Wry>>> = Vec::new();
    items.push(Box::new(title_item));
    items.push(Box::new(PredefinedMenuItem::separator(app)?));

    // 有新版本：紧贴标题下方给最高可见度（配合托盘图标红点）
    if let Some(label) = &model.update {
        items.push(Box::new(MenuItem::with_id(
            app,
            "update",
            label,
            true,
            None::<&str>,
        )?));
        items.push(Box::new(PredefinedMenuItem::separator(app)?));
    } else {
        // 无更新：「检查更新」立即查一次（结果经菜单签名变化自动重建菜单）
        items.push(Box::new(MenuItem::with_id(
            app,
            "check-update",
            &model.check_update,
            true,
            None::<&str>,
        )?));
        items.push(Box::new(PredefinedMenuItem::separator(app)?));
    }

    // 每个窗口直接是一级菜单项，单击直接显示/聚焦
    if model.windows.is_empty() {
        items.push(Box::new(MenuItem::with_id(
            app,
            "windows-empty",
            &model.empty_windows,
            false,
            None::<&str>,
        )?));
    } else {
        for w in &model.windows {
            items.push(Box::new(MenuItem::with_id(
                app,
                &w.show_id,
                &w.label,
                w.enabled,
                None::<&str>,
            )?));
        }
    }

    items.push(Box::new(PredefinedMenuItem::separator(app)?));
    items.push(Box::new(autostart_item));
    items.push(Box::new(new_item));
    items.push(Box::new(PredefinedMenuItem::separator(app)?));
    items.push(Box::new(remote_connect));
    items.push(Box::new(rotate_item));
    items.push(Box::new(remote_item));
    items.push(Box::new(PredefinedMenuItem::separator(app)?));
    items.push(Box::new(quit_item));

    let refs: Vec<&dyn IsMenuItem<tauri::Wry>> =
        items.iter().map(|b| b.as_ref()).collect();
    let menu = Menu::with_items(app, &refs)?;
    Ok((
        menu,
        MenuHandles {
            remote: remote_handle,
            rotate: rotate_handle,
            autostart: autostart_handle,
        },
    ))
}

/// 原生确认对话框：destructive 托盘动作统一走这里。
#[cfg(not(target_os = "linux"))]
fn confirm_destructive(
    app: &tauri::AppHandle,
    title: &str,
    message: &str,
    confirm_label: &str,
    action: impl FnOnce() + Send + 'static,
) {
    use tauri_plugin_dialog::{DialogExt, MessageDialogButtons};
    let cancel = if zh() { "取消" } else { "Cancel" };
    app.dialog()
        .message(message)
        .title(title)
        .buttons(MessageDialogButtons::OkCancelCustom(
            confirm_label.to_string(),
            cancel.to_string(),
        ))
        .show(move |confirmed| {
            if confirmed {
                action();
            }
        });
}

#[cfg(not(target_os = "linux"))]
fn handle_menu_event(app: &tauri::AppHandle, id: &str) {
    let zh = zh();
    if id == "update" {
        // 二次确认后 Broker 直接执行更新链（关窗 → 下载/复用 → 安装重启）
        let (title, message, confirm) = if zh {
            (
                "更新 CC-Box",
                "将关闭所有窗口并更新到新版本，更新完成后可重新打开窗口。确认更新？",
                "更新",
            )
        } else {
            (
                "Update CC-Box",
                "All windows will be closed and CC-Box will update to the new version. Continue?",
                "Update",
            )
        };
        confirm_destructive(app, title, message, confirm, {
            let app = app.clone();
            move || {
                crate::broker::update::run_manual_update(app.clone());
            }
        });
        return;
    }
    if id == "check-update" {
        crate::broker::update::check_now(app.clone());
        return;
    }
    if id == "new" {
        if let Err(e) = crate::broker::windows::spawn_window_user() {
            log::warn!("[broker-tray] new window failed: {}", e);
        }
        return;
    }
    if id == "remote-connect" {
        tauri::async_runtime::spawn(async move {
            if let Err(e) = crate::broker::windows::open_remote_panel().await {
                log::warn!("[broker-tray] remote connect failed: {}", e);
            }
        });
        return;
    }
    if id == "rotate" {
        if crate::broker::status_snapshot()
            .map(|s| s.legacy_owned)
            .unwrap_or(false)
        {
            return;
        }
        let (title, message, confirm) = if zh {
            (
                "刷新远程密钥",
                "所有已配对的手机/浏览器将立即断开，需要重新扫码。确认刷新？",
                "刷新",
            )
        } else {
            (
                "Rotate remote secret",
                "All paired phones/browsers will be disconnected and must pair again. Continue?",
                "Rotate",
            )
        };
        confirm_destructive(app, title, message, confirm, || {
            tauri::async_runtime::spawn(async move {
                if let Err(e) = crate::broker::control(BrokerOp::RotateSecret).await {
                    log::warn!("[broker-tray] rotate secret failed: {}", e);
                }
            });
        });
        return;
    }
    if id == "toggle" {
        let enabled = crate::broker::status_snapshot()
            .map(|s| s.enabled)
            .unwrap_or(false);
        if enabled {
            let (title, message, confirm) = if zh {
                (
                    "关闭远程连接",
                    "所有已连接的手机/浏览器将断开；后台服务与窗口会继续运行。确认关闭？",
                    "关闭",
                )
            } else {
                (
                    "Stop remote",
                    "All connected phones/browsers will disconnect. The background service and windows keep running. Continue?",
                    "Stop",
                )
            };
            confirm_destructive(app, title, message, confirm, || {
                tauri::async_runtime::spawn(async move {
                    if let Err(e) = crate::broker::control(BrokerOp::SetEnabled {
                        enabled: false,
                    })
                    .await
                    {
                        log::warn!("[broker-tray] stop remote failed: {}", e);
                    }
                });
            });
        } else {
            // 开启远程无破坏性，直接执行
            tauri::async_runtime::spawn(async move {
                if let Err(e) = crate::broker::control(BrokerOp::SetEnabled {
                    enabled: true,
                })
                .await
                {
                    log::warn!("[broker-tray] start remote failed: {}", e);
                }
            });
        }
        return;
    }
    if id == "autostart" {
        let enabled = crate::broker::status_snapshot()
            .map(|s| s.autostart)
            .unwrap_or_else(crate::broker::autostart::is_enabled);
        tauri::async_runtime::spawn(async move {
            if let Err(e) = crate::broker::control(BrokerOp::SetAutostart { enabled: !enabled }).await {
                log::warn!("[broker-tray] toggle autostart failed: {}", e);
            }
        });
        return;
    }
    if id == "quit" {
        let legacy = crate::gateway::window_roster()
            .iter()
            .any(|w| !w.closable);
        let (title, confirm) = if zh {
            ("退出 CC-Box", "退出")
        } else {
            ("Quit CC-Box", "Quit")
        };
        let message = if zh {
            if legacy {
                "将关闭所有窗口并退出后台服务。注意：旧版窗口无法被关闭，可能仍会运行。确认退出？"
            } else {
                "将关闭所有窗口并退出后台服务。确认退出？"
            }
        } else if legacy {
            "All windows will be closed and the background service will exit. Note: legacy-version windows cannot be closed and may keep running. Continue?"
        } else {
            "All windows will be closed and the background service will exit. Continue?"
        };
        confirm_destructive(app, title, message, confirm, || {
            tauri::async_runtime::spawn(async move {
                match crate::broker::control(BrokerOp::Quit).await {
                    Ok(_) => crate::broker::request_exit(),
                    Err(e) => log::warn!("[broker-tray] quit failed: {}", e),
                }
            });
        });
        return;
    }
    if let Some(rest) = id.strip_prefix("win:") {
        if let Some((instance_id, "show")) = rest.rsplit_once(':') {
            let instance_id = instance_id.to_string();
            tauri::async_runtime::spawn(async move {
                if let Err(e) =
                    crate::broker::windows::show_window(&instance_id).await
                {
                    log::warn!("[broker-tray] show window failed: {}", e);
                }
            });
        }
    }
}

/// 红点几何（纯函数，可单测）：半径缩小到边长 16%，圆心贴右上角（边缘留 2px）
#[cfg(not(target_os = "linux"))]
fn badge_geometry(w: u32, h: u32) -> (i32, i32, i32) {
    let radius = ((w.min(h) as f32) * 0.16).round().max(3.0) as i32;
    let cx = w as i32 - 2 - radius;
    let cy = 2 + radius;
    (cx, cy, radius)
}

/// 托盘图标叠加红点（有新版本时）：在右上角画实心圆，不改动基础图标资源
#[cfg(not(target_os = "linux"))]
fn badge_icon(app: &tauri::AppHandle) -> Option<tauri::image::Image<'static>> {
    let base = app.default_window_icon()?.clone();
    let (w, h) = (base.width(), base.height());
    let mut rgba = base.rgba().to_vec();
    let (cx, cy, radius) = badge_geometry(w, h);
    for y in 0..h as i32 {
        for x in 0..w as i32 {
            let dx = x - cx;
            let dy = y - cy;
            if dx * dx + dy * dy <= radius * radius {
                let i = ((y as u32 * w + x as u32) * 4) as usize;
                rgba[i] = 235;
                rgba[i + 1] = 64;
                rgba[i + 2] = 64;
                rgba[i + 3] = 255;
            }
        }
    }
    Some(tauri::image::Image::new_owned(rgba, w, h))
}

#[cfg(not(target_os = "linux"))]
pub(crate) fn install(app: &tauri::AppHandle) -> tauri::Result<()> {
    use tauri::tray::TrayIconBuilder;
    use tauri::Manager;

    let model = current_model();
    let (menu, mut handles) = build_menu(app, &model)?;
    let mut last_signature = window_signature(&model);

    let mut builder = TrayIconBuilder::with_id(TRAY_ID)
        .menu(&menu)
        .tooltip(TITLE)
        .on_menu_event(|_app, event| {
            handle_menu_event(_app, event.id().as_ref());
        });
    if let Some(icon) = app.default_window_icon().cloned() {
        builder = builder.icon(icon);
    } else if let Ok(icon) =
        tauri::image::Image::from_bytes(include_bytes!("../../icons/32x32.png"))
    {
        builder = builder.icon(icon);
    }
    #[cfg(target_os = "macos")]
    {
        builder = builder.icon_as_template(true);
    }
    let tray = builder.build(app)?;
    app.manage(tray);

    let app = app.clone();
    tauri::async_runtime::spawn(async move {
        let mut last_badge: Option<bool> = None;
        loop {
            tokio::time::sleep(Duration::from_secs(2)).await;
            let model = current_model();
            let signature = window_signature(&model);
            if signature != last_signature {
                match build_menu(&app, &model) {
                    Ok((menu, new_handles)) => {
                        if let Some(tray) = app.tray_by_id(TRAY_ID) {
                            if let Err(e) = tray.set_menu(Some(menu)) {
                                log::warn!("[broker-tray] set menu failed: {}", e);
                            }
                        }
                        handles = new_handles;
                        last_signature = signature;
                    }
                    Err(e) => log::warn!("[broker-tray] rebuild menu failed: {}", e),
                }
            }
            // 托盘图标红点：有新版本时叠加，恢复到无更新时还原基础图标
            let has_update = model.update.is_some();
            if last_badge != Some(has_update) {
                if let Some(tray) = app.tray_by_id(TRAY_ID) {
                    let icon = if has_update {
                        badge_icon(&app).or_else(|| app.default_window_icon().cloned())
                    } else {
                        app.default_window_icon().cloned()
                    };
                    if let Some(icon) = icon {
                        let _ = tray.set_icon(Some(icon));
                    }
                    #[cfg(target_os = "macos")]
                    {
                        // 红点需要彩色渲染；无更新时恢复模板图标
                        let _ = tray.set_icon_as_template(!has_update);
                    }
                }
                last_badge = Some(has_update);
            }
            let _ = handles.remote.set_text(&model.toggle);
            let _ = handles.rotate.set_text(&model.rotate_secret);
            let _ = handles.rotate.set_enabled(model.rotate_enabled);
            let _ = handles.autostart.set_checked(model.autostart_checked);
        }
    });
    Ok(())
}

#[cfg(target_os = "linux")]
pub(crate) fn install(_app: &tauri::AppHandle) -> tauri::Result<()> {
    log::warn!(
        "[broker-tray] Linux tray not built in this binary; running headless. \
         Window/remote/autostart controls remain available in Settings and the remote panel."
    );
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn status(enabled: bool, legacy: bool) -> BrokerStatus {
        BrokerStatus {
            enabled,
            lan_running: !legacy,
            lan_error: None,
            legacy_owned: legacy,
            instance_count: 2,
            port: 8747,
            relay_enabled: true,
            relay_connected: true,
            relay_url: "wss://example".to_string(),
            relay_pair_code: String::new(),
            revision: 0,
            app_version: "test".to_string(),
            broker_pid: 1,
            autostart: false,
        }
    }

    fn window(id: &str, title: &str, closable: bool) -> WindowInfo {
        WindowInfo {
            id: id.to_string(),
            name: format!("name-{id}"),
            title: title.to_string(),
            pid: if closable { 42 } else { 0 },
            proto_ver: if closable { 2 } else { 1 },
            session_count: 1,
            closable,
        }
    }

    #[test]
    fn Tray_TitleOnlyAndRemoteToggle_1() {
        let stopped = build_model(Some(&status(false, false)), &[], false, None, true);
        assert_eq!(stopped.title, "CC-Box");
        assert!(stopped.toggle.contains("开启"));
        let running = build_model(Some(&status(true, false)), &[], false, None, true);
        assert!(running.toggle.contains("关闭"));
        assert!(running.remote_connect.contains("远程连接"));
        assert!(running.rotate_secret.contains("刷新"));
        assert!(running.rotate_enabled);
    }

    #[test]
    fn Tray_WindowFirstLevelItems_2() {
        let windows = vec![
            window("a", "project-a", true),
            window("b", "project-a", true),
            window("c", "legacy", false),
        ];
        let model = build_model(Some(&status(true, false)), &windows, true, None, true);
        assert_eq!(model.windows[0].label, "project-a");
        assert_eq!(model.windows[1].label, "project-a (2)");
        assert_eq!(model.windows[0].show_id, "win:a:show");
        assert!(model.windows[0].enabled);
        assert!(model.windows[1].enabled);
        assert!(!model.windows[2].enabled);
        assert!(model.windows[2].label.contains("旧版"));
        assert!(model.autostart_checked);
        assert!(model.quit.contains("退出 CC-Box"));
    }

    #[test]
    fn Tray_RotateAndLegacy_3() {
        let model = build_model(Some(&status(true, false)), &[], false, None, true);
        assert!(model.rotate_secret.contains("刷新"));
        let legacy = build_model(Some(&status(true, true)), &[], false, None, true);
        assert!(!legacy.rotate_enabled);
    }

    #[test]
    fn Tray_EmptyWindows_4() {
        let model = build_model(None, &[], false, None, true);
        assert!(model.windows.is_empty());
        assert!(!model.empty_windows.is_empty());
        assert!(model.new_window.contains("新建窗口"));
    }

    // 有新版本：更新项出现在标题下方并进入签名（触发菜单重建）；无更新时不存在
    #[test]
    fn Tray_UpdateItem_5() {
        let none = build_model(Some(&status(true, false)), &[], false, None, true);
        assert!(none.update.is_none());
        assert_eq!(window_signature(&none), "#");

        let some = build_model(Some(&status(true, false)), &[], false, Some("0.17.0"), true);
        assert_eq!(some.update.as_deref(), Some("更新到 v0.17.0"));
        assert_eq!(window_signature(&some), "#更新到 v0.17.0");

        let en = build_model(Some(&status(true, false)), &[], false, Some("0.17.0"), false);
        assert_eq!(en.update.as_deref(), Some("Update to v0.17.0"));
    }

    // 无更新时提供「检查更新」文案（有更新时不渲染该项，由 build_menu 决定）
    #[test]
    fn Tray_CheckUpdateItem_6() {
        let none = build_model(Some(&status(true, false)), &[], false, None, true);
        assert_eq!(none.check_update, "检查更新");
        let en = build_model(Some(&status(true, false)), &[], false, None, false);
        assert_eq!(en.check_update, "Check for updates");
        let some = build_model(Some(&status(true, false)), &[], false, Some("0.17.0"), true);
        assert_eq!(some.check_update, "检查更新");
    }

    // 红点缩小且更贴右上角边缘
    #[cfg(not(target_os = "linux"))]
    #[test]
    fn Tray_BadgeGeometry_7() {
        let (cx, cy, r) = badge_geometry(32, 32);
        assert_eq!(r, 5); // 32*0.16 ≈ 5.12 → 5
        assert_eq!(cx, 32 - 2 - 5); // 右缘留 2px
        assert_eq!(cy, 2 + 5); // 圆心距顶 2px + 半径
    }
}
