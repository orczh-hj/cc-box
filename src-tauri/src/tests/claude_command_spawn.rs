//! 非 PTY claude 子进程 spawn 复现测试（Windows）
//!
//! 事故（2026-11-25 用户日志）：用户环境 PATH 只有 npm 的 `claude` sh shim +
//! `claude.cmd`（无 `claude.exe`），`claude_command()` 的 Windows PathLookup
//! 分支用裸名交给 CreateProcess → `program not found`，插件市场安装/注册
//! 市场全部失败（终端 PTY 经 Git Bash 正常，形成「终端能用、市场不能用」）。
//! 修复：PathLookup 先 `find_executable` 解析，再 `spawnable_claude_command`
//! 补 `.cmd`。本测试构造与用户环境一致的「只有 .cmd shim」PATH 验证可 spawn。

#![cfg(target_os = "windows")]

use std::fs;
use std::path::PathBuf;

use std::sync::Mutex;

/// PATH 是进程级全局，串行化本文件内改 PATH 的测试，防止互相踩踏
static PATH_LOCK: Mutex<()> = Mutex::new(());

/// 临时目录里伪造 npm 安装布局：`claude`（无扩展名 sh shim）+ `claude.cmd`。
/// `.cmd` 输出标记串证明跑到的确实是这个 shim（独立真值，非回环断言）。
fn fake_cmd_only_layout() -> PathBuf {
    let dir = std::env::temp_dir().join(format!(
        "ccbox-claude-shim-test-{}",
        std::process::id()
    ));
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(&dir).unwrap();
    // npm 布局：同目录同时存在 sh shim（无扩展名）与 .cmd 包装
    fs::write(dir.join("claude"), "#!/bin/sh\nexec node claude.js\n").unwrap();
    fs::write(
        dir.join("claude.cmd"),
        "@echo off\r\necho CLAUDE_CMD_SHIM_OK\r\n",
    )
    .unwrap();
    dir
}

/// 结束时恢复 PATH
struct PathRestore(String);
impl Drop for PathRestore {
    fn drop(&mut self) {
        std::env::set_var("PATH", &self.0);
    }
}

// PATH 只有 .cmd shim（无 exe）时，claude_command() 必须能成功 spawn
//（修复前：裸名 CreateProcess 找不到 program，spawn 直接报错）
#[test]
fn ClaudeCommand_CmdOnlyPath_001() {
    let _guard = PATH_LOCK.lock().unwrap();
    let dir = fake_cmd_only_layout();
    let old_path = std::env::var("PATH").unwrap_or_default();
    let _restore = PathRestore(old_path.clone());

    // 只保留 System32（`where` 自身要能找到），去掉真实 claude 所在目录
    let has_system32 = old_path
        .split(';')
        .any(|e| e.eq_ignore_ascii_case(r"C:\Windows\System32"));
    if !has_system32 {
        // 无 System32 的非常规环境下无法可靠构造，跳过（不算失败）
        return;
    }
    std::env::set_var("PATH", format!("{};C:\\Windows\\System32", dir.display()));

    let mut cmd = crate::store::claude_command_with_target(None);
    cmd.arg("--version");
    let output = cmd.output().expect(
        "claude_command() 在「PATH 仅含 claude.cmd shim」环境下 spawn 失败\
         （program not found）——PathLookup 分支未经 find_executable/.cmd 解析",
    );
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("CLAUDE_CMD_SHIM_OK"),
        "应执行到伪造的 claude.cmd shim，实际 stdout: {:?}",
        stdout
    );

    let _ = fs::remove_dir_all(&dir);
}
