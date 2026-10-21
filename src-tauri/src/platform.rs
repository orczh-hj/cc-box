//! 跨平台工具模块
//!
//! 统一管理所有平台特定代码，消除跨模块重复。
//! 包含：命令执行、可执行文件查找、输出解码、文件管理器、进程管理、架构检测、Shell 选择、PATH 刷新。

use std::path::Path;
use std::process::Command;

// ---- 1. Command 装饰 ----

/// Windows 子进程标志：禁止创建控制台窗口
#[cfg(target_os = "windows")]
const CREATE_NO_WINDOW: u32 = 0x08000000;

/// 为 Command 设置平台特定的创建标志
///
/// Windows 上调用 `.creation_flags(CREATE_NO_WINDOW)` 抑制子进程控制台窗口。
/// Unix 上无操作。
pub(crate) fn configure_command(cmd: &mut Command) {
    #[cfg(target_os = "windows")]
    {
        use std::os::windows::process::CommandExt;
        cmd.creation_flags(CREATE_NO_WINDOW);
    }
    #[cfg(not(target_os = "windows"))]
    {
        let _ = cmd;
    }
}

/// 创建一个已应用平台默认配置的 Command
pub(crate) fn new_command(program: &str) -> Command {
    let mut cmd = Command::new(program);
    configure_command(&mut cmd);
    cmd
}

// ---- 2. 可执行文件查找 ----

/// 用 where (Windows) / which (Unix) 查找可执行文件，返回第一个匹配
pub(crate) fn find_executable(name: &str) -> Option<String> {
    let output = run_locate(name)?;

    if !output.status.success() {
        return None;
    }

    decode_output(&output.stdout)
        .lines()
        .next()
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty() && Path::new(s).exists())
}

/// 用 where (Windows) / which (Unix) 查找可执行文件，返回所有匹配
pub(crate) fn find_all_executables(name: &str) -> Vec<String> {
    let output = match run_locate(name) {
        Some(o) => o,
        None => {
            log::warn!("[Platform] Failed to run locate for '{}'", name);
            return Vec::new();
        }
    };

    if !output.status.success() {
        log::warn!(
            "[Platform] locate '{}' exited with status: {}",
            name,
            output.status
        );
        let stderr = decode_output(&output.stderr);
        if !stderr.trim().is_empty() {
            log::warn!("[Platform] stderr: {}", stderr.trim());
        }
        log::debug!(
            "[Platform] Current PATH: {}",
            std::env::var("PATH").unwrap_or_default()
        );
        return Vec::new();
    }

    let results: Vec<String> = decode_output(&output.stdout)
        .lines()
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty() && Path::new(s).exists())
        .collect();

    log::debug!(
        "[Platform] locate '{}' found {} result(s): {:?}",
        name,
        results.len(),
        results
    );
    results
}

#[cfg(target_os = "windows")]
fn run_locate(name: &str) -> Option<std::process::Output> {
    let mut cmd = Command::new("cmd");
    cmd.args(["/C", "where", name]);
    configure_command(&mut cmd);
    cmd.output().ok()
}

#[cfg(not(target_os = "windows"))]
fn run_locate(name: &str) -> Option<std::process::Output> {
    Command::new("which").arg(name).output().ok()
}

// ---- 3. 输出解码 ----

/// 解码子进程输出为 UTF-8 字符串
///
/// 贪心扫描：每个位置优先尝试 UTF-8 多字节序列，失败再尝试 GBK 双字节，
/// 都不合法才用 U+FFFD 替换。这样 UTF-8 与 GBK 混合输出（如 Claude CLI
/// UTF-8 + Windows cmd.exe GBK）能各自正确解码，互不污染。
/// 全平台统一实现（GBK 在 Unix 也无害，UTF-8 字节流不会被误判）。
pub(crate) fn decode_output(bytes: &[u8]) -> String {
    let mut result = String::with_capacity(bytes.len());
    let mut i = 0;
    while i < bytes.len() {
        let b = bytes[i];

        // ASCII 快速路径
        if b < 0x80 {
            result.push(b as char);
            i += 1;
            continue;
        }

        // 多字节 UTF-8 前导？尝试按 UTF-8 解码
        let utf8_len = utf8_expected_len(b);
        if utf8_len >= 2
            && i + utf8_len <= bytes.len()
            && std::str::from_utf8(&bytes[i..i + utf8_len]).is_ok()
        {
            // SAFETY: 已通过 from_utf8 校验
            let s = std::str::from_utf8(&bytes[i..i + utf8_len]).unwrap();
            result.push_str(s);
            i += utf8_len;
            continue;
        }

        // UTF-8 不合法，尝试 GBK 双字节
        if i + 2 <= bytes.len() && is_gbk_lead(b) && is_gbk_trail(bytes[i + 1]) {
            let (cow, _, _) = encoding_rs::GBK.decode(&bytes[i..i + 2]);
            result.push_str(&cow);
            i += 2;
            continue;
        }

        // 兜底：单字节替换为 U+FFFD
        result.push('\u{FFFD}');
        i += 1;
    }
    result
}

/// 根据 UTF-8 前导字节判断序列期望长度（0 表示非前导：续字节或非法字节）
pub(crate) fn utf8_expected_len(byte: u8) -> usize {
    if byte & 0xE0 == 0xC0 {
        2
    } else if byte & 0xF0 == 0xE0 {
        3
    } else if byte & 0xF8 == 0xF0 {
        4
    } else {
        0
    }
}

/// GBK 双字节字符的首字节范围
pub(crate) fn is_gbk_lead(b: u8) -> bool {
    (0x81..=0xFE).contains(&b)
}

/// GBK 双字节字符的次字节范围
pub(crate) fn is_gbk_trail(b: u8) -> bool {
    (0x40..=0x7E).contains(&b) || (0x80..=0xFE).contains(&b)
}

// ---- 4. 文件管理器 ----

/// 在系统文件管理器中打开指定路径
pub(crate) fn open_in_file_manager(path: &str) -> Result<(), String> {
    #[cfg(target_os = "windows")]
    {
        let mut cmd = new_command("explorer");
        cmd.arg(path);
        cmd.spawn().map_err(|e| e.to_string())?;
    }
    #[cfg(target_os = "macos")]
    {
        Command::new("open")
            .arg(path)
            .spawn()
            .map_err(|e| e.to_string())?;
    }
    #[cfg(target_os = "linux")]
    {
        Command::new("xdg-open")
            .arg(path)
            .spawn()
            .map_err(|e| e.to_string())?;
    }
    Ok(())
}

// ---- 5. 进程管理 ----

/// 杀死指定名称的所有进程
///
/// Windows: `taskkill /F /IM <name>`，Unix: `pkill -x <name>`。
pub(crate) fn kill_processes_by_name(name: &str) -> Result<(), String> {
    #[cfg(target_os = "windows")]
    {
        let mut cmd = new_command("taskkill");
        cmd.args(["/F", "/IM", name]);
        let output = cmd
            .output()
            .map_err(|e| format!("Failed to run taskkill: {}", e))?;
        if !output.status.success() {
            let stderr = decode_output(&output.stderr);
            if !stderr.contains("not found") && !stderr.is_empty() {
                log::warn!("[Platform] kill_processes_by_name stderr: {}", stderr);
            }
        }
    }
    #[cfg(not(target_os = "windows"))]
    {
        let status = Command::new("pkill")
            .args(["-x", name])
            .status()
            .map_err(|e| format!("Failed to run pkill: {}", e))?;
        if !status.success() {
            log::warn!(
                "[Platform] pkill exited non-zero (may be no processes)"
            );
        }
    }
    Ok(())
}

/// 检查指定名称的进程是否存在
pub(crate) fn is_process_running(name: &str) -> Result<bool, String> {
    #[cfg(target_os = "windows")]
    {
        let mut cmd = new_command("tasklist");
        cmd.args(["/FI", &format!("IMAGENAME eq {}", name), "/NH"]);
        let output = cmd
            .output()
            .map_err(|e| format!("Failed to run tasklist: {}", e))?;
        let stdout = decode_output(&output.stdout);
        Ok(stdout.contains(name))
    }
    #[cfg(not(target_os = "windows"))]
    {
        let status = Command::new("pgrep")
            .args(["-x", name])
            .status()
            .map_err(|e| format!("Failed to run pgrep: {}", e))?;
        Ok(status.success())
    }
}

// ---- 6. 架构检测 ----

/// 获取当前平台标识字符串
///
/// Windows: "win32-x64" 或 "win32-arm64"
/// macOS: "darwin-arm64" 或 "darwin-x64"
/// Linux: "linux-x64" / "linux-x64-musl" / "linux-arm64" / "linux-arm64-musl"
pub(crate) fn get_platform_id() -> String {
    #[cfg(target_os = "windows")]
    {
        if std::env::var("PROCESSOR_ARCHITECTURE")
            .unwrap_or_default()
            .contains("ARM64")
        {
            "win32-arm64".to_string()
        } else {
            "win32-x64".to_string()
        }
    }
    #[cfg(target_os = "macos")]
    {
        if let Ok(output) = Command::new("sysctl")
            .args(["-n", "hw.optional.arm64"])
            .output()
        {
            if output.status.success() {
                let val = String::from_utf8_lossy(&output.stdout);
                if val.trim() == "1" {
                    return "darwin-arm64".to_string();
                }
            }
        }
        "darwin-x64".to_string()
    }
    #[cfg(target_os = "linux")]
    {
        let arch = Command::new("uname")
            .arg("-m")
            .output()
            .ok()
            .map(|o| String::from_utf8_lossy(&o.stdout).trim().to_string())
            .unwrap_or_default();

        if arch.contains("aarch64") || arch.contains("arm64") {
            if std::path::Path::new("/lib/libc.musl-aarch64.so.1").exists() {
                "linux-arm64-musl".to_string()
            } else {
                "linux-arm64".to_string()
            }
        } else if std::path::Path::new("/lib/libc.musl-x86_64.so.1").exists() {
            "linux-x64-musl".to_string()
        } else {
            "linux-x64".to_string()
        }
    }
}

// ---- 7. Shell 选择 ----

/// 获取启动 Claude CLI 的 shell 程序和参数（仅 Windows 使用）
///
/// `git_bash_path` 传入检测到的 Git Bash 路径，为 None 时回退到
/// PowerShell。Unix 不经 shell 中转（pty.rs 直接 exec claude 完整路径），
/// 保留签名仅供跨平台编译与测试引用。
pub(crate) fn get_claude_shell(
    claude_cmd: &str,
    git_bash_path: Option<&str>,
) -> (String, Vec<String>) {
    #[cfg(target_os = "windows")]
    {
        if let Some(git_bash) = git_bash_path {
            (
                git_bash.to_string(),
                vec!["-c".to_string(), claude_cmd.to_string()],
            )
        } else {
            (
                "powershell.exe".to_string(),
                vec![
                    "-NoLogo".to_string(),
                    "-Command".to_string(),
                    claude_cmd.to_string(),
                ],
            )
        }
    }
    #[cfg(not(target_os = "windows"))]
    {
        let _ = (claude_cmd, git_bash_path);
        (String::new(), Vec::new())
    }
}

/// 获取 PTY 默认 shell
pub(crate) fn get_default_shell() -> (&'static str, Vec<&'static str>) {
    #[cfg(target_os = "windows")]
    {
        ("cmd.exe", vec![])
    }
    #[cfg(not(target_os = "windows"))]
    {
        ("/bin/bash", vec!["-i"])
    }
}

// ---- 8. PATH 刷新 ----

/// 刷新进程 PATH 环境变量（修复 GUI 应用继承问题）
///
/// Windows: 添加便携版安装目录到 PATH。
/// Unix: 从登录 shell 获取完整 PATH。
pub(crate) fn refresh_path() {
    #[cfg(target_os = "windows")]
    {
        refresh_path_windows();
    }
    #[cfg(unix)]
    {
        refresh_path_unix();
    }
}

#[cfg(target_os = "windows")]
fn refresh_path_windows() {
    let mut path = std::env::var("PATH").unwrap_or_default();

    let user_profile = std::env::var("USERPROFILE").unwrap_or_else(|_| {
        format!(
            "C:\\Users\\{}",
            std::env::var("USERNAME").unwrap_or_default()
        )
    });

    let local_app_data = std::env::var("LOCALAPPDATA")
        .unwrap_or_else(|_| format!("{}\\AppData\\Local", user_profile));

    // Claude: %USERPROFILE%\.local\bin
    let claude_dir = format!("{}\\.local\\bin", user_profile);
    if Path::new(&claude_dir).exists() {
        let lower_path = path.to_lowercase();
        if !lower_path.contains(&claude_dir.to_lowercase()) {
            path = format!("{};{}", claude_dir, path);
            log::info!("[Platform] Added Claude to PATH: {}", claude_dir);
        }
    }

    // Git 便携版: %LOCALAPPDATA%\PortableGit\bin
    let git_bin = format!("{}\\PortableGit\\bin", local_app_data);
    if Path::new(&git_bin).exists() {
        let lower_path = path.to_lowercase();
        if !lower_path.contains(&git_bin.to_lowercase()) {
            path = format!("{};{}", git_bin, path);
            log::info!("[Platform] Added PortableGit to PATH: {}", git_bin);
        }
    }

    std::env::set_var("PATH", &path);
    log::debug!("[Platform] Windows PATH refreshed");
}

#[cfg(unix)]
fn refresh_path_unix() {
    let shell =
        std::env::var("SHELL").unwrap_or_else(|_| "/bin/bash".to_string());
    log::info!(
        "[Platform] Refreshing PATH via: {} -l -c 'printenv PATH'",
        shell
    );
    log::debug!(
        "[Platform] Original PATH: {}",
        std::env::var("PATH").unwrap_or_default()
    );

    let output = Command::new(&shell)
        .args(["-l", "-c", "printenv PATH"])
        .output();

    match output {
        Ok(output) if output.status.success() => {
            let login_path =
                String::from_utf8_lossy(&output.stdout).trim().to_string();
            if !login_path.is_empty() {
                log::info!(
                    "[Platform] PATH refreshed from login shell ({} entries)",
                    login_path.split(':').count()
                );
                log::debug!("[Platform] Refreshed PATH: {}", login_path);
                std::env::set_var("PATH", &login_path);
            } else {
                log::warn!(
                    "[Platform] Login shell returned empty PATH, keeping default"
                );
            }
        }
        Ok(output) => {
            let stderr = String::from_utf8_lossy(&output.stderr);
            log::warn!(
                "[Platform] Login shell failed (exit {}): {}",
                output.status,
                stderr.trim()
            );
        }
        Err(e) => {
            log::warn!("[Platform] Failed to run '{}': {}", shell, e);
        }
    }
}
