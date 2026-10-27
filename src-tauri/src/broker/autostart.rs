//! 开机自启动（仅 Broker，不打开窗口）
//!
//! - Windows：HKCU Run 项 `CC-Box`
//! - macOS：`~/Library/LaunchAgents/com.cc-box.broker.plist`
//! - Linux：`$XDG_CONFIG_HOME/autostart/cc-box-broker.desktop`
//!
//! 统一命令：`<exe> --broker --autostart`。`--autostart` 让 Broker 忽略上次
//! “退出 CC-Box”留下的 user_quit 标记。

use std::path::{Path, PathBuf};

const ARGS: &str = "--broker --autostart";
const WINDOWS_VALUE_NAME: &str = "CC-Box";

/// 自启动应指向的可执行文件。Linux AppImage 的临时挂载路径每次变化，
/// 优先使用稳定的 `$APPIMAGE` 原文件。
fn autostart_exe() -> Result<PathBuf, String> {
    #[cfg(target_os = "linux")]
    {
        if let Ok(appimage) = std::env::var("APPIMAGE") {
            if !appimage.is_empty() {
                return Ok(PathBuf::from(appimage));
            }
        }
    }
    std::env::current_exe().map_err(|e| e.to_string())
}

pub(crate) fn is_enabled() -> bool {
    platform_is_enabled()
}

pub(crate) fn set_enabled(enabled: bool) -> Result<(), String> {
    let exe = autostart_exe()?;
    platform_set_enabled(enabled, &exe)
}

/// 纯函数：Windows Run 值
pub(crate) fn windows_run_value(exe: &Path) -> String {
    format!("\"{}\" {}", exe.display(), ARGS)
}

/// 纯函数：macOS LaunchAgent plist
pub(crate) fn launch_agent_plist(exe: &Path) -> String {
    format!(
        r#"<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
  <key>Label</key>
  <string>com.cc-box.broker</string>
  <key>ProgramArguments</key>
  <array>
    <string>{}</string>
    <string>--broker</string>
    <string>--autostart</string>
  </array>
  <key>RunAtLoad</key>
  <true/>
  <key>ProcessType</key>
  <string>Interactive</string>
</dict>
</plist>
"#,
        xml_escape(&exe.to_string_lossy())
    )
}

/// Desktop Entry Exec 的路径转义：双引号内需转义 `\`、`"`、`` ` ``、`$`，
/// 字段码 `%` 必须写成 `%%`。
fn desktop_exec_quote(exe: &Path) -> String {
    let raw = exe.to_string_lossy().replace('%', "%%");
    let escaped = raw
        .replace('\\', "\\\\")
        .replace('"', "\\\"")
        .replace('`', "\\`")
        .replace('$', "\\$");
    format!("\"{}\"", escaped)
}

/// 纯函数：Linux XDG autostart
pub(crate) fn xdg_desktop_entry(exe: &Path) -> String {
    format!(
        "[Desktop Entry]\n\
         Type=Application\n\
         Name=CC-Box\n\
         Comment=CC-Box background service\n\
         Exec={} {}\n\
         Terminal=false\n\
         X-GNOME-Autostart-enabled=true\n",
        desktop_exec_quote(exe),
        ARGS
    )
}

fn xml_escape(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&apos;")
}

// ==================== Windows ====================

#[cfg(target_os = "windows")]
fn wide(s: &str) -> Vec<u16> {
    s.encode_utf16().chain(Some(0)).collect()
}

#[cfg(target_os = "windows")]
fn platform_is_enabled() -> bool {
    use windows_sys::Win32::System::Registry::{
        RegCloseKey, RegOpenKeyExW, RegQueryValueExW, HKEY, HKEY_CURRENT_USER, KEY_QUERY_VALUE,
        REG_SZ,
    };
    const RUN_KEY: &str = "Software\\Microsoft\\Windows\\CurrentVersion\\Run";
    let key_w = wide(RUN_KEY);
    let value_w = wide(WINDOWS_VALUE_NAME);
    let mut key: HKEY = std::ptr::null_mut();
    let rc = unsafe {
        RegOpenKeyExW(
            HKEY_CURRENT_USER,
            key_w.as_ptr(),
            0,
            KEY_QUERY_VALUE,
            &mut key,
        )
    };
    if rc != 0 {
        return false;
    }
    let mut ty = 0u32;
    let mut size = 0u32;
    let rc = unsafe {
        RegQueryValueExW(
            key,
            value_w.as_ptr(),
            std::ptr::null(),
            &mut ty,
            std::ptr::null_mut(),
            &mut size,
        )
    };
    unsafe {
        let _ = RegCloseKey(key);
    }
    rc == 0 && ty == REG_SZ && size > 2
}

#[cfg(target_os = "windows")]
fn platform_set_enabled(enabled: bool, exe: &Path) -> Result<(), String> {
    use windows_sys::Win32::System::Registry::{
        RegCloseKey, RegDeleteValueW, RegOpenKeyExW, RegSetValueExW, HKEY, HKEY_CURRENT_USER,
        KEY_SET_VALUE, REG_SZ,
    };
    const RUN_KEY: &str = "Software\\Microsoft\\Windows\\CurrentVersion\\Run";
    let key_w = wide(RUN_KEY);
    let value_w = wide(WINDOWS_VALUE_NAME);
    let mut key: HKEY = std::ptr::null_mut();
    let rc = unsafe {
        RegOpenKeyExW(
            HKEY_CURRENT_USER,
            key_w.as_ptr(),
            0,
            KEY_SET_VALUE,
            &mut key,
        )
    };
    if rc != 0 {
        return Err(format!("RegOpenKeyExW failed: {}", rc));
    }
    let result = if enabled {
        let data = wide(&windows_run_value(exe));
        let rc = unsafe {
            RegSetValueExW(
                key,
                value_w.as_ptr(),
                0,
                REG_SZ,
                data.as_ptr() as *const u8,
                (data.len() * std::mem::size_of::<u16>()) as u32,
            )
        };
        if rc == 0 {
            Ok(())
        } else {
            Err(format!("RegSetValueExW failed: {}", rc))
        }
    } else {
        let rc = unsafe { RegDeleteValueW(key, value_w.as_ptr()) };
        // 2 = ERROR_FILE_NOT_FOUND：已不存在视为成功
        if rc == 0 || rc == 2 {
            Ok(())
        } else {
            Err(format!("RegDeleteValueW failed: {}", rc))
        }
    };
    unsafe {
        let _ = RegCloseKey(key);
    }
    result
}

// ==================== macOS ====================

#[cfg(target_os = "macos")]
fn plist_path() -> Option<PathBuf> {
    dirs::home_dir().map(|h| {
        h.join("Library")
            .join("LaunchAgents")
            .join("com.cc-box.broker.plist")
    })
}

#[cfg(target_os = "macos")]
fn platform_is_enabled() -> bool {
    plist_path().map(|p| p.exists()).unwrap_or(false)
}

#[cfg(target_os = "macos")]
fn platform_set_enabled(enabled: bool, exe: &Path) -> Result<(), String> {
    use std::process::Command;
    let path = plist_path().ok_or_else(|| "home directory unavailable".to_string())?;
    let uid = unsafe { libc::getuid() };
    if enabled {
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent).map_err(|e| e.to_string())?;
        }
        let content = launch_agent_plist(exe);
        super::ownership::atomic_write_private(&path, content.as_bytes())
            .map_err(|e| e.to_string())?;
        // 已登录时立即加载；失败只记日志（下次登录仍会生效）
        let status = Command::new("launchctl")
            .args([
                "bootstrap",
                &format!("gui/{}", uid),
                &path.to_string_lossy(),
            ])
            .status();
        match status {
            Ok(s) if s.success() => {}
            Ok(s) => log::warn!("[autostart] launchctl bootstrap exited {}", s),
            Err(e) => log::warn!("[autostart] launchctl bootstrap failed: {}", e),
        }
        Ok(())
    } else {
        let _ = Command::new("launchctl")
            .args(["bootout", &format!("gui/{}/com.cc-box.broker", uid)])
            .status();
        match std::fs::remove_file(&path) {
            Ok(()) => Ok(()),
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(()),
            Err(e) => Err(e.to_string()),
        }
    }
}

// ==================== Linux ====================

#[cfg(target_os = "linux")]
fn desktop_path() -> Option<PathBuf> {
    let base = std::env::var_os("XDG_CONFIG_HOME")
        .map(PathBuf::from)
        .or_else(|| dirs::home_dir().map(|h| h.join(".config")))?;
    Some(base.join("autostart").join("cc-box-broker.desktop"))
}

#[cfg(target_os = "linux")]
fn platform_is_enabled() -> bool {
    desktop_path().map(|p| p.exists()).unwrap_or(false)
}

#[cfg(target_os = "linux")]
fn platform_set_enabled(enabled: bool, exe: &Path) -> Result<(), String> {
    let path = desktop_path().ok_or_else(|| "home directory unavailable".to_string())?;
    if enabled {
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent).map_err(|e| e.to_string())?;
        }
        let content = xdg_desktop_entry(exe);
        super::ownership::atomic_write_private(&path, content.as_bytes())
            .map_err(|e| e.to_string())
    } else {
        match std::fs::remove_file(&path) {
            Ok(()) => Ok(()),
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(()),
            Err(e) => Err(e.to_string()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn Autostart_ContentBuilders_1() {
        let exe = Path::new("/tmp/CC Box/cc-box");
        assert_eq!(
            windows_run_value(exe),
            "\"/tmp/CC Box/cc-box\" --broker --autostart"
        );
        let plist = launch_agent_plist(exe);
        assert!(plist.contains("<key>RunAtLoad</key>"));
        assert!(plist.contains("--autostart"));
        let desktop = xdg_desktop_entry(exe);
        assert!(desktop.contains("Exec=\"/tmp/CC Box/cc-box\" --broker --autostart"));
        assert!(desktop.contains("X-GNOME-Autostart-enabled=true"));
    }

    #[test]
    fn Autostart_XmlEscape_2() {
        let plist = launch_agent_plist(Path::new("/tmp/A&B<C>\"D\".app"));
        assert!(plist.contains("A&amp;B&lt;C&gt;&quot;D&quot;.app"));
    }

    #[test]
    fn Autostart_DesktopExecEscape_3() {
        let desktop = xdg_desktop_entry(Path::new("/tmp/A B/\\$X%Y\"Z"));
        assert!(
            desktop.contains(r#"Exec="/tmp/A B/\\\$X%%Y\"Z" --broker --autostart"#),
            "unexpected desktop entry: {}",
            desktop
        );
    }
}
