// Prevents additional console window on Windows in release
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use cc_box::cli::{help_text, parse, print_console, Invocation};

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    match parse(&args) {
        // -h / -V：发布构建是 GUI 子系统，需附加父控制台才能看到输出
        Invocation::Help => print_console(&help_text(env!("APP_VERSION"))),
        Invocation::Version => print_console(&format!("CC-Box {}\n", env!("APP_VERSION"))),
        Invocation::QuitBroker => {
            // 命令行退出 Broker（关闭所有窗口）；无托盘环境/脚本使用
            if let Err(e) = cc_box::quit_broker_blocking() {
                print_console(&format!("failed to quit CC-Box: {}\n", e));
                std::process::exit(1);
            }
        }
        Invocation::Broker => cc_box::run_broker(),
        Invocation::Gui {
            initial_dir,
            new_window,
        } => cc_box::run(initial_dir, new_window),
    }
}
