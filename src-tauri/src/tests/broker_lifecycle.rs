//! Broker 生命周期标记与单实例锁测试。

use crate::broker::lifecycle::{self, LifecycleState};

#[test]
fn Lifecycle_UserQuitSuppressesUntilExplicitClear_1() {
    let _cfg_lock = super::CONFIG_DIR_TEST_LOCK.lock();
    let _cfg = super::TestConfigDirGuard::new("lifecycle-user-quit");

    lifecycle::write(LifecycleState::UserQuit, None).unwrap();
    assert!(!lifecycle::should_autostart(), "user_quit 必须抑制自动拉起");

    lifecycle::clear_user_quit();
    assert!(lifecycle::should_autostart(), "显式清除后允许拉起");
    let _ = lifecycle::clear();
}

#[test]
fn Lifecycle_UpdateInProgressVersionGate_2() {
    let _cfg_lock = super::CONFIG_DIR_TEST_LOCK.lock();
    let _cfg = super::TestConfigDirGuard::new("lifecycle-update");

    // 同版本 + 新鲜标记 → 抑制（更新尚未完成）
    lifecycle::write(
        LifecycleState::UpdateInProgress,
        Some(env!("APP_VERSION").to_string()),
    )
    .unwrap();
    assert!(!lifecycle::should_autostart());

    // 版本已变化 → 视为更新完成，清除并允许
    lifecycle::write(LifecycleState::UpdateInProgress, Some("0.0.0".to_string())).unwrap();
    assert!(lifecycle::should_autostart());
    assert!(lifecycle::read().is_none(), "版本变化应清除标记");
    let _ = lifecycle::clear();
}

#[test]
fn Lifecycle_RunningAndIdleAllowAutostart_3() {
    let _cfg_lock = super::CONFIG_DIR_TEST_LOCK.lock();
    let _cfg = super::TestConfigDirGuard::new("lifecycle-running");

    lifecycle::write(LifecycleState::Running, None).unwrap();
    assert!(lifecycle::should_autostart());

    lifecycle::write(LifecycleState::IdleExit, None).unwrap();
    assert!(lifecycle::should_autostart(), "空闲回收不抑制下次启动");
    let _ = lifecycle::clear();
}

#[test]
fn BrokerInstanceLock_SecondAcquireFails_4() {
    let _cfg_lock = super::CONFIG_DIR_TEST_LOCK.lock();
    let _cfg = super::TestConfigDirGuard::new("broker-single-instance");

    let first = crate::broker::ownership::BrokerInstanceGuard::acquire().unwrap();
    assert!(
        crate::broker::ownership::BrokerInstanceGuard::acquire().is_err(),
        "同一配置目录只允许一个 Broker"
    );
    drop(first);
    assert!(
        crate::broker::ownership::BrokerInstanceGuard::acquire().is_ok(),
        "释放后可重新获取"
    );
}
