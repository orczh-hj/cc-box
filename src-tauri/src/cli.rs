//! 命令行入口解析。
//!
//! 同一二进制承担四种角色（GUI / 后台 Broker / 退出 Broker / 帮助与版本），
//! 由参数决定。解析抽成纯函数便于单测；`main.rs` 只负责分发。
//!
//! 注意：发布构建是 `windows_subsystem = "windows"`（GUI 子系统），进程默认
//! 没有控制台，`println!/eprintln!` 在终端里看不到。需要向用户输出短文本时
//! 用 [`print_console`]（附加父进程控制台 → 写 CONOUT$ → 断开）。

/// 启动角色
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Invocation {
    /// `-h` / `--help`：打印帮助
    Help,
    /// `-V` / `--version`：打印版本
    Version,
    /// `--quit-broker`：关闭所有窗口并退出后台 Broker
    QuitBroker,
    /// `--broker`：无窗口常驻后台
    Broker,
    /// 默认：GUI；可带初始项目目录与「新开窗口」
    Gui {
        initial_dir: Option<String>,
        new_window: bool,
    },
}

/// 帮助文本（随版本号展示）
pub fn help_text(version: &str) -> String {
    format!(
        "CC-Box {version}

用法: cc-box [选项] [项目目录]

选项:
  -h, --help        显示本帮助并退出
  -V, --version     显示版本号并退出
  --broker          以常驻后台 Broker 模式运行（无窗口，托盘常驻）
  --quit-broker     关闭所有窗口并退出后台 Broker（脚本 / 无托盘环境）
  --new-window      新开一个窗口，停在项目页不自动进入终端

参数:
  项目目录          启动后直接打开该项目（省略则进入项目页）
"
    )
}

/// 解析参数（`args` 不含 argv[0]）。
/// 帮助/版本优先于其他角色；未知选项忽略（保持向后兼容）。
pub fn parse(args: &[String]) -> Invocation {
    let has = |flag: &str| args.iter().any(|a| a == flag);

    if has("-h") || has("--help") {
        return Invocation::Help;
    }
    if has("-V") || has("--version") {
        return Invocation::Version;
    }
    if has("--quit-broker") {
        return Invocation::QuitBroker;
    }
    if has("--broker") {
        return Invocation::Broker;
    }
    let initial_dir = args
        .iter()
        .find(|p| !p.starts_with('-') && std::path::Path::new(p).is_dir())
        .cloned();
    Invocation::Gui {
        initial_dir,
        new_window: has("--new-window"),
    }
}

/// 向用户可见地输出一段文本（GUI 子系统构建下也能在终端里看到）。
///
/// Windows：附加到父进程控制台 → 写 CONOUT$ → 立即断开（不长期占用控制台，
/// 避免 Broker 常驻进程随终端关闭收到 CTRL_CLOSE 而退出）。
/// 非 Windows / 已带控制台的调试构建：直接 stdout。
pub fn print_console(text: &str) {
    #[cfg(windows)]
    {
        use std::io::Write;
        use windows_sys::Win32::System::Console::{AttachConsole, FreeConsole, ATTACH_PARENT_PROCESS};
        unsafe {
            AttachConsole(ATTACH_PARENT_PROCESS);
        }
        let wrote = std::fs::OpenOptions::new()
            .write(true)
            .open("CONOUT$")
            .ok()
            .and_then(|mut f| f.write_all(text.as_bytes()).ok())
            .is_some();
        if !wrote {
            // 没有可用控制台（从资源管理器/计划任务启动）：输出丢弃即可
            let _ = std::io::stdout().write_all(text.as_bytes());
        }
        unsafe {
            FreeConsole();
        }
    }
    #[cfg(not(windows))]
    {
        use std::io::Write;
        let _ = std::io::stdout().write_all(text.as_bytes());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn args(list: &[&str]) -> Vec<String> {
        list.iter().map(|s| s.to_string()).collect()
    }

    #[test]
    fn Cli_HelpAndVersion_1() {
        assert_eq!(parse(&args(&["-h"])), Invocation::Help);
        assert_eq!(parse(&args(&["--help"])), Invocation::Help);
        assert_eq!(parse(&args(&["-V"])), Invocation::Version);
        assert_eq!(parse(&args(&["--version"])), Invocation::Version);
        // 帮助优先于其他角色
        assert_eq!(parse(&args(&["--broker", "-h"])), Invocation::Help);
    }

    #[test]
    fn Cli_BrokerModes_2() {
        assert_eq!(parse(&args(&["--broker"])), Invocation::Broker);
        // 旧版注册的登录项仍带 --autostart：未知参数忽略，行为不变
        assert_eq!(
            parse(&args(&["--broker", "--autostart"])),
            Invocation::Broker
        );
        assert_eq!(parse(&args(&["--quit-broker"])), Invocation::QuitBroker);
    }

    #[test]
    fn Cli_GuiArgs_3() {
        assert_eq!(
            parse(&args(&["--new-window"])),
            Invocation::Gui { initial_dir: None, new_window: true }
        );
        // 未知选项忽略
        assert_eq!(
            parse(&args(&["--unknown"])),
            Invocation::Gui { initial_dir: None, new_window: false }
        );
        // 仅存在的目录作为初始项目；文件/不存在路径忽略
        let dir = std::env::temp_dir();
        let dir_str = dir.to_string_lossy().to_string();
        assert_eq!(
            parse(&args(&[&dir_str])),
            Invocation::Gui { initial_dir: Some(dir_str), new_window: false }
        );
        assert_eq!(
            parse(&args(&["Z:/__no_such_dir__"])),
            Invocation::Gui { initial_dir: None, new_window: false }
        );
    }

    #[test]
    fn Cli_HelpListsAllFlags_4() {
        let text = help_text("1.2.3");
        for flag in [
            "-h, --help", "-V, --version", "--broker", "--quit-broker", "--new-window",
        ] {
            assert!(text.contains(flag), "help text missing {flag}");
        }
        assert!(text.contains("1.2.3"));
    }
}
