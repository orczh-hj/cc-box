use crate::platform::*;

// ---- decode_output ----

// 有效 UTF-8 字节原样返回字符串
#[test]
fn DecodeOutput_Utf8_001() {
    let input = b"hello world";
    let result = decode_output(input);
    assert_eq!(result, "hello world");
}

// UTF-8 编码的中文字符正确解码
#[test]
fn DecodeOutput_Chinese_001() {
    let input = "你好世界".as_bytes();
    let result = decode_output(input);
    assert_eq!(result, "你好世界");
}

// GBK 编码的字节在 Windows 上正确解码为中文
#[cfg(target_os = "windows")]
#[test]
fn DecodeOutput_Gbk_001() {
    let (cow, _, _) = encoding_rs::GBK.encode("你好");
    let gbk_bytes: Vec<u8> = cow.into_owned();
    let result = decode_output(&gbk_bytes);
    assert_eq!(result, "你好");
}

// 空字节切片返回空字符串
#[test]
fn DecodeOutput_Empty_001() {
    let result = decode_output(&[]);
    assert_eq!(result, "");
}

// 混合 ASCII 和 UTF-8 多字节字符
#[test]
fn DecodeOutput_Mixed_001() {
    let input = "hello 你好 world 世界".as_bytes();
    let result = decode_output(input);
    assert_eq!(result, "hello 你好 world 世界");
}

// UTF-8 中文 + GBK 中文混合：两段各自正确解码（核心新场景，防止整体 GBK 回退污染）
#[test]
fn DecodeOutput_MixedUtf8Gbk_001() {
    let mut bytes = "前".as_bytes().to_vec(); // UTF-8: E5 89 8D
    let (gbk_cow, _, _) = encoding_rs::GBK.encode("后");
    bytes.extend_from_slice(&gbk_cow); // GBK: BA F3
    let result = decode_output(&bytes);
    assert_eq!(result, "前后");
}

// GBK 编码字节全平台正确解码（不再限定 Windows，新实现统一处理）
#[test]
fn DecodeOutput_Gbk_AnyPlatform_001() {
    let (cow, _, _) = encoding_rs::GBK.encode("你好");
    let gbk_bytes = cow.into_owned();
    let result = decode_output(&gbk_bytes);
    assert_eq!(result, "你好");
}

#[test]
fn Platform_PidAliveAndSelfTerminate_001() {
    assert!(is_pid_alive(std::process::id()));
    assert!(!is_pid_alive(0));
    assert!(terminate_pid(std::process::id(), true).is_err());
}

// 孤立非法字节（0x80）被 U+FFFD 替换，前后 ASCII 不被 GBK 污染
#[test]
fn DecodeOutput_IsolatedInvalid_001() {
    let bytes = [b'A', 0x80, b'B'];
    let result = decode_output(&bytes);
    assert_eq!(result, "A\u{FFFD}B");
}

// 4 字节 UTF-8 emoji 正确解码（验证多字节 UTF-8 路径）
#[test]
fn DecodeOutput_FourByteUtf8_001() {
    let bytes = "🎉".as_bytes();
    let result = decode_output(bytes);
    assert_eq!(result, "🎉");
}

// ---- is_runnable_cli ----

// 当前测试二进制是可运行的普通文件
#[test]
fn IsRunnableCli_CurrentExe_001() {
    let exe = std::env::current_exe().expect("current_exe");
    assert!(is_runnable_cli(&exe.to_string_lossy()));
}

// 目录即使 exists() 为 true 也不能作为执行目标（execve 会返回 EACCES）
#[test]
fn IsRunnableCli_Directory_001() {
    let dir = std::env::temp_dir();
    assert!(!is_runnable_cli(&dir.to_string_lossy()));
}

// 不存在的路径不可运行
#[test]
fn IsRunnableCli_NotFound_001() {
    let missing = if cfg!(windows) {
        "C:\\nonexistent\\claude-xyz"
    } else {
        "/nonexistent/claude-xyz"
    };
    assert!(!is_runnable_cli(missing));
}

// Unix：普通文件缺少执行位时不可运行（Mac 事故场景：被复制后权限丢失）
#[cfg(unix)]
#[test]
fn IsRunnableCli_NoExecBit_001() {
    use std::os::unix::fs::PermissionsExt;

    let path = std::env::temp_dir().join(format!(
        "cc-box-noexec-{}-{:?}",
        std::process::id(),
        std::thread::current().id()
    ));
    std::fs::write(&path, b"#!/bin/sh\necho hi\n").expect("write temp file");
    std::fs::set_permissions(&path, std::fs::Permissions::from_mode(0o644))
        .expect("set permissions");

    assert!(!is_runnable_cli(&path.to_string_lossy()));
    let _ = std::fs::remove_file(&path);
}

// Windows：PATHEXT 之外的文件不视为可运行
#[cfg(windows)]
#[test]
fn IsRunnableCli_WindowsExtension_001() {
    let path = std::env::temp_dir().join(format!("cc-box-cli-{}.txt", std::process::id()));
    std::fs::write(&path, b"not an executable").expect("write temp file");
    assert!(!is_runnable_cli(&path.to_string_lossy()));
    let _ = std::fs::remove_file(&path);
}

// ---- cli_target ----

// 配置有效 → Explicit（用户显式指定的路径被尊重）
#[test]
fn CliTarget_Explicit_001() {
    let exe = std::env::current_exe().expect("current_exe");
    let exe = exe.to_string_lossy().to_string();
    match cli_target(Some(&exe)) {
        CliTarget::Explicit(path) => assert_eq!(path, exe),
        CliTarget::PathLookup => panic!("expected Explicit"),
    }
}

// 配置失效（不存在 / 目录 / 缺执行位）→ PathLookup，绝不返回坏路径
#[test]
fn CliTarget_BrokenConfig_FallsBackToPathLookup_002() {
    let dir = std::env::temp_dir().to_string_lossy().to_string();
    assert!(matches!(cli_target(Some(&dir)), CliTarget::PathLookup));

    let missing = if cfg!(windows) {
        "C:\\nonexistent\\claude-xyz"
    } else {
        "/nonexistent/claude-xyz"
    };
    assert!(matches!(cli_target(Some(missing)), CliTarget::PathLookup));
}

// 未配置 → PathLookup（与终端输入裸命令一致）
#[test]
fn CliTarget_NoConfig_003() {
    assert!(matches!(cli_target(None), CliTarget::PathLookup));
}

// ---- find_executable ----

// 查找系统自带的可执行文件返回 Some
#[test]
fn FindExe_SystemCmd_001() {
    #[cfg(target_os = "windows")]
    let name = "cmd";
    #[cfg(not(target_os = "windows"))]
    let name = "sh";
    let result = find_executable(name);
    assert!(result.is_some(), "Should find '{}' on PATH", name);
}

// 查找不存在的可执行文件返回 None
#[test]
fn FindExe_NotFound_001() {
    let result = find_executable("nonexistent_binary_xyz_12345");
    assert!(result.is_none());
}

// find_executable 返回存在的路径
#[test]
fn FindExe_PathExists_001() {
    #[cfg(target_os = "windows")]
    let name = "cmd";
    #[cfg(not(target_os = "windows"))]
    let name = "sh";
    if let Some(path) = find_executable(name) {
        assert!(
            std::path::Path::new(&path).exists(),
            "Returned path should exist: {}",
            path
        );
    }
}

// ---- find_all_executables ----

// 查找系统命令返回非空列表
#[test]
fn FindAllExe_SystemCmd_001() {
    #[cfg(target_os = "windows")]
    let name = "cmd";
    #[cfg(not(target_os = "windows"))]
    let name = "sh";
    let results = find_all_executables(name);
    assert!(!results.is_empty(), "Should find '{}' on PATH", name);
}

// 查找不存在的命令返回空列表
#[test]
fn FindAllExe_NotFound_001() {
    let results = find_all_executables("nonexistent_binary_xyz_12345");
    assert!(results.is_empty());
}

// ---- get_platform_id ----

// 返回的平台字符串包含已知前缀
#[test]
fn GetPlatformId_Prefix_001() {
    let platform = get_platform_id();
    let valid = platform.starts_with("win32-")
        || platform.starts_with("darwin-")
        || platform.starts_with("linux-");
    assert!(
        valid,
        "Platform '{}' should start with known prefix",
        platform
    );
}

// ---- get_claude_shell ----

// 无 git_bash_path 时 Windows 返回 powershell；Unix 不再 shell 中转（空返回）
#[test]
fn GetClaudeShell_NoGitBash_001() {
    let (program, args) = get_claude_shell("claude", None);
    #[cfg(target_os = "windows")]
    {
        assert_eq!(program, "powershell.exe");
        assert!(args.contains(&"-Command".to_string()));
    }
    #[cfg(not(target_os = "windows"))]
    {
        assert_eq!(program, "");
        assert!(args.is_empty());
    }
}

// 有 git_bash_path 时 Windows 返回 bash -c；Unix 同样空返回
#[test]
fn GetClaudeShell_WithGitBash_001() {
    let (program, args) = get_claude_shell("claude --foo", Some("C:\\bash.exe"));
    #[cfg(target_os = "windows")]
    {
        assert_eq!(program, "C:\\bash.exe");
        assert!(args.contains(&"-c".to_string()));
        assert!(args.contains(&"claude --foo".to_string()));
    }
    #[cfg(not(target_os = "windows"))]
    {
        assert_eq!(program, "");
        assert!(args.is_empty());
    }
}
