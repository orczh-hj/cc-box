//! 窗口管理：拉起 GUI、优雅关闭、超时强杀、重启。
//!
//! Broker 自身没有窗口；它只管理成员进程。关闭优先走 v2 IPC 的
//! `WindowCommand::Close`（GUI 杀 PTY 后退出），5s 未退出再按 PID 强杀。
//! 重启语义 = 关闭（结束会话）后重新拉起空窗口。

use std::path::Path;
#[cfg(unix)]
use std::process::Command;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::Duration;

use parking_lot::Mutex;

use crate::gateway_proto::WindowAction;

/// 防止远程/误触无限开窗
const MAX_WINDOWS: usize = 12;
const SPAWN_COOLDOWN_MS: u64 = 2_000;
/// 新窗口从项目列表开始，不自动进入终端
const NEW_WINDOW_ARG: &str = "--new-window";

static LAST_SPAWN_MS: AtomicU64 = AtomicU64::new(0);
static SPAWN_LOCK: Mutex<()> = Mutex::new(());

fn now_ms() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_millis() as u64)
        .unwrap_or(0)
}

/// 远程/命令触发的新窗口：受冷却限制防刷
pub(crate) fn spawn_window() -> Result<(), String> {
    spawn_window_inner(true)
}

/// 托盘等显式用户动作：跳过冷却（仍受最大窗口数约束）
pub(crate) fn spawn_window_user() -> Result<(), String> {
    spawn_window_inner(false)
}

fn spawn_window_inner(enforce_cooldown: bool) -> Result<(), String> {
    if crate::broker::is_broker() {
        let _guard = SPAWN_LOCK.lock();
        if crate::gateway::member_count() >= MAX_WINDOWS {
            return Err(format!("too many windows (max {})", MAX_WINDOWS));
        }
        let now = now_ms();
        let last = LAST_SPAWN_MS.load(Ordering::SeqCst);
        if enforce_cooldown
            && last != 0
            && now.saturating_sub(last) < SPAWN_COOLDOWN_MS
        {
            return Err("window spawn is cooling down; retry shortly".to_string());
        }
        LAST_SPAWN_MS.store(now, Ordering::SeqCst);
    }

    let exe = std::env::current_exe().map_err(|e| e.to_string())?;
    spawn_process(&exe)
}

#[cfg(target_os = "windows")]
fn spawn_process(exe: &Path) -> Result<(), String> {
    use std::os::windows::ffi::OsStrExt;
    use windows_sys::Win32::Foundation::CloseHandle;
    use windows_sys::Win32::System::Threading::{
        CreateProcessW, CREATE_NO_WINDOW, PROCESS_INFORMATION, STARTUPINFOW,
    };
    use windows_sys::Win32::UI::WindowsAndMessaging::SW_HIDE;

    // windows-sys 未导出该宏
    const STARTF_USESHOWWINDOW: u32 = 0x0000_0001;

    let exe_w: Vec<u16> = exe.as_os_str().encode_wide().chain(Some(0)).collect();
    let mut cmdline: Vec<u16> = format!("\"{}\" {}", exe.display(), NEW_WINDOW_ARG)
        .encode_utf16()
        .chain(Some(0))
        .collect();

    let mut si: STARTUPINFOW = unsafe { std::mem::zeroed() };
    si.cb = std::mem::size_of::<STARTUPINFOW>() as u32;
    si.dwFlags = STARTF_USESHOWWINDOW;
    si.wShowWindow = SW_HIDE as u16;
    let mut pi: PROCESS_INFORMATION = unsafe { std::mem::zeroed() };

    // bInheritHandles=FALSE：不把 Broker 的 LAN/relay socket 泄漏给 GUI；
    // CREATE_NO_WINDOW + SW_HIDE：debug 构建也不弹控制台窗口。
    let ok = unsafe {
        CreateProcessW(
            exe_w.as_ptr(),
            cmdline.as_mut_ptr(),
            std::ptr::null_mut(),
            std::ptr::null_mut(),
            0,
            CREATE_NO_WINDOW,
            std::ptr::null_mut(),
            std::ptr::null(),
            &si,
            &mut pi,
        )
    };
    if ok != 0 {
        unsafe {
            let _ = CloseHandle(pi.hProcess);
            let _ = CloseHandle(pi.hThread);
        }
        log::info!("[broker-window] spawned new window");
        return Ok(());
    }

    let err = std::io::Error::last_os_error();
    log::warn!(
        "[broker-window] CreateProcessW failed ({}); falling back to Command",
        err
    );
    let mut cmd = crate::platform::new_command(&exe.to_string_lossy());
    cmd.arg(NEW_WINDOW_ARG)
        .spawn()
        .map(|_| log::info!("[broker-window] spawned new window via Command"))
        .map_err(|e| format!("failed to spawn window: {}", e))
}

#[cfg(target_os = "macos")]
fn spawn_process(exe: &Path) -> Result<(), String> {
    if let Some(bundle) = find_app_bundle(exe) {
        let status = Command::new("open")
            .arg("-n")
            .arg(&bundle)
            .args(["--args", NEW_WINDOW_ARG])
            .status()
            .map_err(|e| format!("failed to run open: {}", e))?;
        if status.success() {
            log::info!("[broker-window] spawned new window via open -n");
            return Ok(());
        }
        log::warn!("[broker-window] open -n failed; falling back to direct spawn");
    }
    spawn_unix(exe)
}

#[cfg(all(unix, not(target_os = "macos")))]
fn spawn_process(exe: &Path) -> Result<(), String> {
    // AppImage 的 current_exe 指向临时挂载目录；优先启动稳定的 AppImage 文件
    if let Ok(appimage) = std::env::var("APPIMAGE") {
        if !appimage.is_empty() {
            return spawn_unix(Path::new(&appimage));
        }
    }
    spawn_unix(exe)
}

#[cfg(unix)]
fn spawn_unix(exe: &Path) -> Result<(), String> {
    let mut child = Command::new(exe)
        .arg(NEW_WINDOW_ARG)
        .stdin(std::process::Stdio::null())
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .spawn()
        .map_err(|e| format!("failed to spawn window: {}", e))?;
    // 回收子进程，避免 Broker 长期运行时积累僵尸进程
    std::thread::spawn(move || {
        let _ = child.wait();
    });
    log::info!("[broker-window] spawned new window");
    Ok(())
}

#[cfg(target_os = "macos")]
fn find_app_bundle(exe: &Path) -> Option<std::path::PathBuf> {
    exe.ancestors()
        .find(|p| p.extension().map(|e| e == "app").unwrap_or(false))
        .map(|p| p.to_path_buf())
}

fn member_exists(instance_id: &str) -> bool {
    crate::gateway::window_roster()
        .iter()
        .any(|w| w.id == instance_id)
}

/// 显示（还原/置顶/聚焦）单个窗口（托盘一级窗口项单击）。
pub(crate) async fn show_window(instance_id: &str) -> Result<(), String> {
    let window = crate::gateway::window_roster()
        .into_iter()
        .find(|w| w.id == instance_id);
    let Some(window) = window else {
        return Ok(());
    };
    if !window.closable {
        return Err(format!(
            "window '{}' uses a legacy client and cannot be controlled",
            window.title
        ));
    }
    crate::gateway::send_window_command(instance_id, WindowAction::Show).await
}

/// 托盘“远程连接”：显示第一个可控制窗口并打开其远程面板；
/// 没有可控制窗口时先拉起一个新窗口，再下发 OpenRemote。
pub(crate) async fn open_remote_panel() -> Result<(), String> {
    let target = match crate::gateway::window_roster()
        .into_iter()
        .find(|w| w.closable)
    {
        Some(w) => w.id,
        None => {
            spawn_window_user()?;
            let deadline = tokio::time::Instant::now() + Duration::from_secs(5);
            loop {
                if let Some(w) = crate::gateway::window_roster()
                    .into_iter()
                    .find(|w| w.closable)
                {
                    break w.id;
                }
                if tokio::time::Instant::now() >= deadline {
                    return Err("new window did not register in time".to_string());
                }
                tokio::time::sleep(Duration::from_millis(100)).await;
            }
        }
    };
    crate::gateway::send_window_command(&target, WindowAction::OpenRemote).await
}

/// 关闭所有可控制窗口（`except` 用于更新时保留发起者）。
pub(crate) async fn close_windows(except: Option<&str>, timeout: Duration) {
    let windows: Vec<_> = crate::gateway::window_roster()
        .into_iter()
        .filter(|w| w.closable && except.map(|k| k != w.id).unwrap_or(true))
        .collect();

    for w in &windows {
        if let Err(e) =
            crate::gateway::send_window_command(&w.id, WindowAction::Close).await
        {
            log::warn!(
                "[broker-window] close command to '{}' failed: {}",
                w.name,
                e
            );
        }
    }

    let deadline = tokio::time::Instant::now() + timeout;
    loop {
        let remaining = windows.iter().filter(|w| member_exists(&w.id)).count();
        if remaining == 0 || tokio::time::Instant::now() >= deadline {
            break;
        }
        tokio::time::sleep(Duration::from_millis(100)).await;
    }

    for w in &windows {
        if member_exists(&w.id) {
            log::warn!(
                "[broker-window] '{}' still alive after {:?}; terminating pid {}",
                w.name,
                timeout,
                w.pid
            );
            let _ = crate::platform::terminate_pid(w.pid, true);
        }
    }
    // 给断开/退出一点收尾时间
    tokio::time::sleep(Duration::from_millis(200)).await;
}

pub(crate) async fn close_all_windows(timeout: Duration) {
    close_windows(None, timeout).await;
}
