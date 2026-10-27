// Prevents additional console window on Windows in release
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
    let args: Vec<String> = std::env::args().collect();
    // 命令行退出 Broker（关闭所有窗口）；无托盘环境/脚本使用
    if args.iter().skip(1).any(|a| a == "--quit-broker") {
        if let Err(e) = cc_box::quit_broker_blocking() {
            eprintln!("failed to quit CC-Box: {}", e);
            std::process::exit(1);
        }
        return;
    }
    // 后台 Broker：同一二进制、无 GUI 窗口；--autostart 表示由登录项启动
    if args.iter().skip(1).any(|a| a == "--broker") {
        let autostart = args.iter().skip(1).any(|a| a == "--autostart");
        cc_box::run_broker(autostart);
        return;
    }
    let initial_dir = args
        .iter()
        .skip(1)
        .find(|p| !p.starts_with('-') && std::path::Path::new(p).is_dir())
        .cloned();
    let new_window = args.iter().skip(1).any(|a| a == "--new-window");
    cc_box::run(initial_dir, new_window)
}
