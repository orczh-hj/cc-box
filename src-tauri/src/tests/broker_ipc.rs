//! Broker IPC 集成测试：真实 Named Pipe/UDS、密钥/版本认证、成员注册、
//! 管理操作、断连清理。

use std::time::Duration;

use cc_box_remote_protocol::RemoteSession;
use tokio::io::{AsyncRead, AsyncWrite};

use crate::broker::ipc;
use crate::gateway_proto::{
    instance_id, BrokerAdminRequest, BrokerAdminResponse, BrokerOp, GwDown, GwUp, GW_PROTO_VER,
};

fn session(id: &str) -> RemoteSession {
    RemoteSession {
        id: id.to_string(),
        pty_type: "claude".to_string(),
        cwd: "/proj".to_string(),
        working: false,
        pending: false,
        session_id: None,
        instance_id: None,
        instance_name: None,
    }
}

async fn connect_with_retry() -> ipc::BoxStream {
    for _ in 0..100 {
        if let Ok(s) = ipc::connect_client().await {
            return s;
        }
        tokio::time::sleep(Duration::from_millis(30)).await;
    }
    panic!("broker IPC not reachable");
}

async fn read_down<S>(rd: &mut S) -> GwDown
where
    S: AsyncRead + Unpin,
{
    let frame = tokio::time::timeout(Duration::from_secs(3), ipc::read_frame(rd))
        .await
        .expect("frame timeout")
        .expect("read ok")
        .expect("frame present");
    serde_json::from_slice(&frame).expect("valid GwDown")
}

async fn write_up<S, T>(wr: &mut S, msg: &T) -> std::io::Result<()>
where
    S: AsyncWrite + Unpin,
    T: serde::Serialize,
{
    let bytes = serde_json::to_vec(msg).map_err(std::io::Error::other)?;
    ipc::write_frame(wr, &bytes).await
}

/// 即使断言 panic 也恢复全局 broker 模式，避免拖垮并行测试
struct BrokerModeGuard;

impl Drop for BrokerModeGuard {
    fn drop(&mut self) {
        crate::remote_state::set_broker_mode(false);
        crate::gateway::test_reset();
    }
}

#[tokio::test]
async fn BrokerIpc_RegisterAdminAndCleanup_1() {
    let _lock = super::REMOTE_SERVER_TEST_LOCK.lock();
    let _cfg_lock = super::CONFIG_DIR_TEST_LOCK.lock();
    let _cfg = super::TestConfigDirGuard::new("broker-ipc");

    crate::remote_state::set_broker_mode(true);
    let _mode_guard = BrokerModeGuard;
    crate::gateway::test_reset();
    let secret = crate::broker::ownership::ensure_broker_secret().expect("secret");
    ipc::spawn_server().await.expect("ipc server");

    // ---- 正常注册 ----
    let inst = instance_id().to_string();
    let stream = connect_with_retry().await;
    let (mut rd, mut wr) = tokio::io::split(stream);
    write_up(
        &mut wr,
        &GwUp::MemberHello {
            ver: GW_PROTO_VER,
            instance_id: inst.clone(),
            secret: secret.clone(),
            label: Some("ipc-test".to_string()),
            pid: std::process::id(),
            window_title: Some("IPC Test".to_string()),
            sessions: vec![session("s-1")],
        },
    )
    .await
    .unwrap();

    let mut registered = false;
    let mut got_status = false;
    for _ in 0..8 {
        match read_down(&mut rd).await {
            GwDown::RegisterOk { instance_name, .. } => {
                assert_eq!(instance_name, "IPC Test");
                registered = true;
            }
            GwDown::BrokerStatus { status } => {
                assert!(!status.app_version.is_empty());
                got_status = true;
            }
            _ => {}
        }
        if registered && got_status {
            break;
        }
    }
    assert!(registered, "RegisterOk expected");
    assert!(got_status, "BrokerStatus expected");
    assert_eq!(crate::gateway::member_count(), 1);
    let roster = crate::gateway::window_roster();
    assert_eq!(roster.len(), 1);
    assert_eq!(roster[0].pid, std::process::id());
    assert_eq!(roster[0].title, "IPC Test");
    assert_eq!(roster[0].name, "IPC Test");
    assert!(roster[0].closable);
    assert_eq!(
        crate::gateway::test_route_of("s-1").as_deref(),
        Some(inst.as_str())
    );

    // ---- 管理操作：GetStatus ----
    let admin_stream = connect_with_retry().await;
    let (mut ard, mut awr) = tokio::io::split(admin_stream);
    write_up(
        &mut awr,
        &BrokerAdminRequest::Admin {
            id: 7,
            secret: secret.clone(),
            op: BrokerOp::GetStatus,
        },
    )
    .await
    .unwrap();
    let frame = tokio::time::timeout(Duration::from_secs(3), ipc::read_frame(&mut ard))
        .await
        .expect("admin timeout")
        .unwrap()
        .unwrap();
    match serde_json::from_slice::<BrokerAdminResponse>(&frame).unwrap() {
        BrokerAdminResponse::Result { id, ok, status, .. } => {
            assert_eq!(id, 7);
            assert!(ok);
            assert_eq!(status.map(|s| s.instance_count), Some(1));
        }
    }

    // ---- 错误密钥：Reject ----
    let bad_stream = connect_with_retry().await;
    let (mut brd, mut bwr) = tokio::io::split(bad_stream);
    write_up(
        &mut bwr,
        &GwUp::MemberHello {
            ver: GW_PROTO_VER,
            instance_id: "intruder".to_string(),
            secret: "wrong".to_string(),
            label: None,
            pid: std::process::id(),
            window_title: None,
            sessions: vec![],
        },
    )
    .await
    .unwrap();
    match read_down(&mut brd).await {
        GwDown::Reject { .. } => {}
        other => panic!("expected Reject, got {:?}", other),
    }

    // ---- 断连清理：成员注销、路由移除 ----
    drop(rd);
    drop(wr);
    let mut cleaned = false;
    for _ in 0..100 {
        if crate::gateway::member_count() == 0 {
            cleaned = true;
            break;
        }
        tokio::time::sleep(Duration::from_millis(30)).await;
    }
    assert!(cleaned, "member should unregister on disconnect");
    assert!(crate::gateway::test_route_of("s-1").is_none());

    crate::gateway::test_reset();
    crate::remote_state::set_broker_mode(false);
}
