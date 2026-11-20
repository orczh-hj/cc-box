//! logger 纯函数测试：级别解析/优先级、错误链展开（多层/防环）、跨零点滚动判断、日志行格式。
//! 不触碰 set_logger/set_max_level 全局状态。

use crate::logger::{error_chain, format_line, parse_level, resolve_level, should_rollover};

#[derive(Debug)]
struct ChainError {
    msg: &'static str,
    source: Option<Box<dyn std::error::Error>>,
}

impl std::fmt::Display for ChainError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.msg)
    }
}

impl std::error::Error for ChainError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.source.as_deref()
    }
}

fn chained(msg: &'static str, source: Option<Box<dyn std::error::Error>>) -> ChainError {
    ChainError { msg, source }
}

/// CC_BOX_LOG 合法取值解析（大小写不敏感、容忍空白）
#[test]
fn Logger_ParseLevel_Valid_001() {
    assert_eq!(parse_level("error"), Some(log::LevelFilter::Error));
    assert_eq!(parse_level("warn"), Some(log::LevelFilter::Warn));
    assert_eq!(parse_level("info"), Some(log::LevelFilter::Info));
    assert_eq!(parse_level("debug"), Some(log::LevelFilter::Debug));
    assert_eq!(parse_level("trace"), Some(log::LevelFilter::Trace));
}

#[test]
fn Logger_ParseLevel_CaseInsensitive_001() {
    assert_eq!(parse_level("DEBUG"), Some(log::LevelFilter::Debug));
    assert_eq!(parse_level("  Info "), Some(log::LevelFilter::Info));
}

#[test]
fn Logger_ParseLevel_Invalid_001() {
    assert_eq!(parse_level("verbose"), None);
    assert_eq!(parse_level(""), None);
    assert_eq!(parse_level("1"), None);
}

/// 级别优先级：env 合法值覆盖编译模式；env 非法/缺失回退编译默认
#[test]
fn Logger_ResolveLevel_Priority_001() {
    // env 覆盖：release 下放开 debug（排障场景）
    assert_eq!(
        resolve_level(Some("debug"), false),
        log::LevelFilter::Debug
    );
    // env 覆盖：debug 编译下收紧 error
    assert_eq!(
        resolve_level(Some("error"), true),
        log::LevelFilter::Error
    );
    // env 非法 → 编译默认
    assert_eq!(resolve_level(Some("bogus"), true), log::LevelFilter::Debug);
    assert_eq!(resolve_level(Some("bogus"), false), log::LevelFilter::Info);
    // env 缺失 → 编译默认
    assert_eq!(resolve_level(None, true), log::LevelFilter::Debug);
    assert_eq!(resolve_level(None, false), log::LevelFilter::Info);
}

/// 错误链展开：三层嵌套 source 全部按 " -> " 连接（对应 commit 9983380 丢链踩坑）
#[test]
fn Logger_ErrorChain_MultiLayer_001() {
    let inner = chained("inner", None);
    let mid = chained("mid", Some(Box::new(inner)));
    let outer = chained("outer", Some(Box::new(mid)));
    assert_eq!(error_chain(&outer), "outer -> mid -> inner");
}

/// 单层错误无 source 时不追加箭头
#[test]
fn Logger_ErrorChain_SingleLayer_001() {
    let e = chained("only", None);
    assert_eq!(error_chain(&e), "only");
}

/// 深度上限：12 层深链被截到最多 8 段（防环上限的行为锚定，环是该上限的极端场景）
#[test]
fn Logger_ErrorChain_CycleCapped_001() {
    let mut e: Box<dyn std::error::Error> = Box::new(chained("layer-12", None));
    for i in (0..11).rev() {
        e = Box::new(chained(Box::leak(format!("layer-{}", i).into_boxed_str()), Some(e)));
    }
    let out = error_chain(e.as_ref());
    let segs = out.split(" -> ").count();
    assert!(segs <= 8, "depth not capped: {} segments", segs);
}

/// 跨零点滚动判断：日期前缀不同即滚动
#[test]
fn Logger_ShouldRollover_001() {
    assert!(should_rollover("2026-11-19", "2026-11-20"));
    assert!(!should_rollover("2026-11-20", "2026-11-20"));
    assert!(should_rollover("", "2026-11-20"));
}

/// 日志行格式锚定：[ts][LEVEL] [target] args
#[test]
fn Logger_FormatLine_001() {
    let record = log::Record::builder()
        .level(log::Level::Error)
        .target("cc_box::tests")
        .args(format_args!("pty={} spawn failed", "abc"))
        .build();
    let line = format_line(&record);
    assert!(line.starts_with('['), "missing timestamp prefix: {}", line);
    assert!(line.contains("][ERROR] [cc_box::tests] pty=abc spawn failed"), "line: {}", line);
    assert!(line.ends_with('\n'));
}
