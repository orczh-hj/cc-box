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

// ---- configure_command / new_command ----

// new_command 返回的 Command 包含正确的 program
#[test]
fn NewCommand_Program_001() {
    let cmd = new_command("echo");
    assert!(format!("{:?}", cmd).contains("echo"));
}

// configure_command 不改变 Command 的 program
#[test]
fn ConfigureCmd_Program_001() {
    let mut cmd = std::process::Command::new("echo");
    configure_command(&mut cmd);
    assert!(format!("{:?}", cmd).contains("echo"));
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

// 多次调用返回相同结果
#[test]
fn GetPlatformId_Consistent_001() {
    assert_eq!(get_platform_id(), get_platform_id());
}

// ---- get_default_shell ----

// 返回非空程序名
#[test]
fn GetDefaultShell_Program_001() {
    let (program, _) = get_default_shell();
    assert!(!program.is_empty());
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

// ---- refresh_path ----

// refresh_path 不 panic
#[test]
fn RefreshPath_NoPanic_001() {
    refresh_path();
}

// ---- open_in_file_manager ----

// 调用不 panic（不验证结果，因为依赖系统环境）
#[test]
fn OpenFileManager_NoPanic_001() {
    let _ = open_in_file_manager("/nonexistent/path/xyz123");
}
