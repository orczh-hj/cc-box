//! Broker 运行状态快照（成员镜像 / GUI 远程面板 / 托盘共用）

use crate::gateway_proto::BrokerStatus;

pub(crate) fn remote_enabled_from_config() -> bool {
    crate::store::get_app_config()
        .ok()
        .and_then(|c| c.remote)
        .and_then(|r| r.enabled)
        .unwrap_or(false)
}

/// None = 当前进程不是 Broker（GUI 实例）
pub(crate) fn snapshot() -> Option<BrokerStatus> {
    if !crate::remote_state::is_broker() {
        return None;
    }
    let enabled = remote_enabled_from_config();
    let (relay_enabled, relay_connected, relay_url, pair_code) =
        crate::remote_client::status_snapshot();
    Some(BrokerStatus {
        enabled,
        lan_running: crate::remote_server::is_running(),
        lan_error: super::lan_error(),
        legacy_owned: super::legacy_owned(),
        instance_count: crate::gateway::member_count() as u32,
        port: crate::remote_server::preferred_port(),
        relay_enabled,
        relay_connected,
        relay_url,
        relay_pair_code: pair_code,
        revision: crate::remote_state::REMOTE_STATE.revision(),
        app_version: env!("APP_VERSION").to_string(),
        broker_pid: std::process::id(),
        autostart: super::autostart::is_enabled(),
    })
}
