use crate::tests::{TestConfigDirGuard, CONFIG_DIR_TEST_LOCK};

// 启动检查运行时只读：失效的 claudePath 不会被自动改写，
// 自动检测结果只作为运行状态返回（持久化配置只由用户设置/安装器写入）
#[test]
fn RunChecks_DoesNotWriteConfig_001() {
    let _lock = CONFIG_DIR_TEST_LOCK.lock();
    let _guard = TestConfigDirGuard::new("checks-readonly");

    let bad = if cfg!(windows) {
        "C:\\nonexistent\\claude-xyz"
    } else {
        "/nonexistent/claude-xyz"
    };
    crate::store::update_app_config(serde_json::json!({ "claudePath": bad }))
        .expect("seed config");

    let result = crate::checks::run_checks();
    assert!(!result.checks.is_empty());

    let config = crate::store::get_app_config().expect("read config");
    assert_eq!(
        config.claude_path.as_deref(),
        Some(bad),
        "run_checks must not rewrite claudePath"
    );
}
