//! 子进程构造守卫：src-tauri/src 内（封装本体与测试除外）禁止裸
//! `Command::new` —— Windows GUI 进程直接 spawn 控制台程序（git/claude/cmd）
//! 会弹 cmd 黑窗。所有构造必须走 `platform::new_command`（自动
//! CREATE_NO_WINDOW），或经 `store::run_command_with_timeout`（内部已应用
//! 平台配置）。真实事故：市场 git clone 裸构造导致点开市场弹 git.exe 黑窗。

use std::fs;
use std::path::Path;

/// 允许出现裸 Command::new 的文件（相对 src/，附理由）
const ALLOWED: &[(&str, &str)] = &[
    // 封装本体：所有 Command 构造的唯一入口
    ("platform.rs", "封装本体（new_command/configure_command）"),
];

#[test]
fn ProcessSpawn_禁止裸CommandNew_001() {
    let src = Path::new(env!("CARGO_MANIFEST_DIR")).join("src");
    let mut violations = Vec::new();
    scan_dir(&src, &src, &mut violations);
    assert!(
        violations.is_empty(),
        "发现绕过 platform::new_command 的裸 Command::new（Windows 会弹 cmd 黑窗）：\n{}\
         \n请改用 platform::new_command（自动 CREATE_NO_WINDOW）或经 store::run_command_with_timeout。",
        violations.join("\n")
    );
}

fn scan_dir(root: &Path, dir: &Path, violations: &mut Vec<String>) {
    let Ok(entries) = fs::read_dir(dir) else {
        return;
    };
    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
            scan_dir(root, &path, violations);
            continue;
        }
        if path.extension().and_then(|e| e.to_str()) != Some("rs") {
            continue;
        }
        // 相对 src/ 的路径（如 src/store.rs → store.rs、src/broker/tray.rs → broker/tray.rs）
        let rel_in_src = path
            .strip_prefix(root)
            .unwrap_or(&path)
            .to_string_lossy()
            .replace('\\', "/");
        // 测试目录不适用（tests 构造不受 GUI 弹窗约束）
        if rel_in_src.starts_with("tests/") {
            continue;
        }
        if ALLOWED.iter().any(|(f, _)| *f == rel_in_src) {
            continue;
        }
        let Ok(text) = fs::read_to_string(&path) else {
            continue;
        };
        // 逐行找裸构造（注释行忽略）
        for (i, line) in text.lines().enumerate() {
            let trimmed = line.trim_start();
            if trimmed.starts_with("//") || trimmed.starts_with('*') {
                continue;
            }
            if line.contains("Command::new(") {
                violations.push(format!("src/{}:{}: {}", rel_in_src, i + 1, trimmed));
            }
        }
    }
}
