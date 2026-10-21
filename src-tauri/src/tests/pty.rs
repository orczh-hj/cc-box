// ===================== PTY 输出解码回归测试 =====================
// 历史 bug：PTY reader 用 String::from_utf8_lossy 把非 UTF-8 字节
// （如 Windows cmd.exe / 某些 git 输出的 GBK 字节）替换为 U+FFFD，
// 导致终端出现黑色方块乱码。修复后改用 decode_output（贪心扫描：
// UTF-8 优先 + GBK 双字节兜底）。
//
// utf8_complete_boundary / utf8_seq_len 已被 PtyDecoder 替代，
// 边界与跨 read 行为由 tests/pty_decoder.rs 覆盖。

// 复现旧 bug：from_utf8_lossy 把 GBK 字节 "你好" 替换为 U+FFFD
#[cfg(target_os = "windows")]
#[test]
fn PtyDecode_GbkBytes_LossyCorrupts_001() {
    let (cow, _, _) = encoding_rs::GBK.encode("你好");
    let gbk_bytes = cow.into_owned();
    let lossy = String::from_utf8_lossy(&gbk_bytes).to_string();
    assert!(
        lossy.contains('\u{FFFD}'),
        "from_utf8_lossy 应将 GBK 字节替换为 U+FFFD（这是 bug 源头）"
    );
}

// 验证修复：decode_output 把同样的 GBK 字节正确解码为中文
#[cfg(target_os = "windows")]
#[test]
fn PtyDecode_GbkBytes_DecodeCorrect_001() {
    let (cow, _, _) = encoding_rs::GBK.encode("你好");
    let gbk_bytes = cow.into_owned();
    let decoded = crate::platform::decode_output(&gbk_bytes);
    assert_eq!(decoded, "你好");
}

// UTF-8 字节两种解码方式结果一致，确保修复不破坏 UTF-8 主场景
#[test]
fn PtyDecode_Utf8Bytes_BothCorrect_001() {
    let utf8_bytes = "你好世界".as_bytes();
    let lossy = String::from_utf8_lossy(utf8_bytes).to_string();
    let decoded = crate::platform::decode_output(utf8_bytes);
    assert_eq!(lossy, "你好世界");
    assert_eq!(decoded, "你好世界");
}

// ===================== 启动期输出缓存与 ANSI 清理 =====================
// 秒退检测（startup_check）的错误信息携带 PTY 尾部输出：claude 自身的
// 报错文本不再随「前端终端尚未建立」丢失（Mac native claude 秒退白屏）。

use crate::pty::{StartupBuf, STARTUP_BUF_MAX};

// active=false 后不再收集（spawn 已收尾，缓存退役）
#[test]
fn StartupBuf_InactiveStopsCollecting_001() {
    let mut buf = StartupBuf {
        active: false,
        data: Vec::new(),
    };
    buf.append(b"late output");
    assert!(buf.data.is_empty());
}

// 超出上限丢弃头部、保留尾部（报错通常在最后）
#[test]
fn StartupBuf_CapsAtLimit_002() {
    let mut buf = StartupBuf {
        active: true,
        data: Vec::new(),
    };
    let big = vec![b'x'; STARTUP_BUF_MAX + 100];
    buf.append(&big);
    assert_eq!(buf.data.len(), STARTUP_BUF_MAX);
    // 头部 100 字节被丢弃
    assert_eq!(&buf.data[..100], &vec![b'x'; 100][..]);
}

// SGR 颜色序列被移除，文本保留
#[test]
fn StripAnsi_RemovesSgr_001() {
    assert_eq!(
        crate::pty::strip_ansi("\x1b[31merror\x1b[0m: bad args"),
        "error: bad args"
    );
}

// 光标移动等多参数 CSI 与 OSC（标题）序列同样移除
#[test]
fn StripAnsi_RemovesCsiAndOsc_002() {
    assert_eq!(crate::pty::strip_ansi("\x1b[2J\x1b[Hhi\x1b]0;title\x07end"), "hiend");
}

// 纯文本与中文不受影响
#[test]
fn StripAnsi_KeepsPlainText_003() {
    assert_eq!(crate::pty::strip_ansi("你好 world\ttab"), "你好 world\ttab");
}
