//! Broker 路由测试：单一有序队列、先登记后投递、未知会话拒绝、
//! 控制权门、待处理请求清理、v1/v2 消息隔离。

use cc_box_remote_protocol::{ClientMessage, RemoteSession, ServerMessage};
use tokio::sync::{mpsc, watch};

use crate::gateway;
use crate::gateway_proto::{GwDown, GwUp, WindowAction, GW_PROTO_VER, LEGACY_PROTO_VER};
use crate::remote_state::REMOTE_STATE;

fn make_session(id: &str) -> RemoteSession {
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

async fn next_client_msg(
    rx: &mut mpsc::Receiver<GwDown>,
) -> (u64, bool, ClientMessage) {
    for _ in 0..16 {
        let down = tokio::time::timeout(std::time::Duration::from_millis(500), rx.recv())
            .await
            .expect("frame expected")
            .expect("channel open");
        if let GwDown::ClientMsg {
            conn_id,
            via_relay,
            msg,
        } = down
        {
            return (conn_id, via_relay, msg);
        }
    }
    panic!("no ClientMsg within frames");
}

async fn assert_no_client_msg(rx: &mut mpsc::Receiver<GwDown>) {
    for _ in 0..8 {
        match tokio::time::timeout(std::time::Duration::from_millis(120), rx.recv()).await {
            Err(_) => return,
            Ok(None) => return,
            Ok(Some(GwDown::ClientMsg { .. })) => panic!("unexpected ClientMsg"),
            Ok(Some(_)) => continue,
        }
    }
}

async fn register_mock_member(
    tag: &str,
    session_ids: &[&str],
) -> (mpsc::Receiver<GwDown>, String) {
    register_mock_member_with(tag, GW_PROTO_VER, session_ids).await
}

async fn register_mock_member_with(
    tag: &str,
    ver: u32,
    session_ids: &[&str],
) -> (mpsc::Receiver<GwDown>, String) {
    let (down_tx, down_rx) = mpsc::channel(4096);
    let (close_tx, _close_rx) = watch::channel(false);
    let inst = format!("inst-{}", tag);
    let sessions: Vec<cc_box_remote_protocol::RemoteSession> =
        session_ids.iter().map(|s| make_session(s)).collect();
    let _name = gateway::register_member(
        inst.clone(),
        Some(tag.to_string()),
        ver,
        std::process::id(),
        None,
        sessions,
        down_tx,
        close_tx,
    )
    .expect("register should succeed");
    (down_rx, inst)
}

fn cleanup(members: &[String], ptys: &[&str]) {
    for m in members {
        gateway::unregister_member(m);
    }
    for id in ptys {
        REMOTE_STATE.remove_pty(id);
    }
}

async fn recv_until<F: Fn(&ServerMessage) -> bool>(
    rx: &mut mpsc::Receiver<ServerMessage>,
    pred: F,
) -> ServerMessage {
    for _ in 0..16 {
        let msg = tokio::time::timeout(std::time::Duration::from_millis(500), rx.recv())
            .await
            .expect("reply expected")
            .expect("channel open");
        if pred(&msg) {
            return msg;
        }
    }
    panic!("expected message not received");
}

/// 成员会话按 pty_id 转发（per-session 化后无控制权拦截，直通）
#[tokio::test]
async fn Gateway_Dispatch_会话路由转发成员_1() {
    let _lock = super::REMOTE_SERVER_TEST_LOCK.lock();
    gateway::test_reset();
    let (mut down_rx, inst) = register_mock_member("route", &["member-1"]).await;

    let (tx, _rx) = mpsc::channel(16);
    REMOTE_STATE.add_conn(9, tx, false);
    gateway::dispatch(
        9,
        &ClientMessage::SessionAttach {
            id: "member-1".into(),
            cols: 80,
            rows: 24,
        },
    )
    .await;
    let (conn_id, via_relay, msg) = next_client_msg(&mut down_rx).await;
    assert_eq!(conn_id, 9);
    assert!(!via_relay);
    assert!(matches!(msg, ClientMessage::SessionAttach { ref id, .. } if id == "member-1"));

    // 未知会话不再回退本机：不转发，手机收到拒绝 + 快照
    REMOTE_STATE.remove_conn(9);
    cleanup(&[inst], &[]);
}

/// 聚合列表：只有成员条目（Broker 自身不是实例）
#[tokio::test]
async fn Gateway_聚合列表_实例字段_2() {
    let _lock = super::REMOTE_SERVER_TEST_LOCK.lock();
    gateway::test_reset();
    let (_, inst) = register_mock_member("merged", &["m-a", "m-b"]).await;

    let list = gateway::merged_session_list(Vec::new());
    let ma = list.iter().find(|s| s.id == "m-a").expect("member listed");
    assert_eq!(ma.instance_id.as_deref(), Some(inst.as_str()));
    assert_eq!(ma.instance_name.as_deref(), Some("merged"));
    assert!(list.iter().all(|s| s.id != "local-a"));
    cleanup(&[inst], &[]);
}

/// 写命令转发直通（per-session 化：无控制权拦截，任何连接可发）
#[tokio::test]
async fn Gateway_Dispatch_写命令转发直通_3() {
    let _lock = super::REMOTE_SERVER_TEST_LOCK.lock();
    gateway::test_reset();
    let (mut down_rx, inst) = register_mock_member("gate", &["gm-1"]).await;

    let (tx, mut rx) = mpsc::channel(64);
    REMOTE_STATE.add_conn(11, tx, false);
    // 连续两个连接的写类消息都直通转发（全局控制权门已退役）
    gateway::dispatch(
        11,
        &ClientMessage::SessionAttach {
            id: "gm-1".into(),
            cols: 80,
            rows: 24,
        },
    )
    .await;
    let (conn_id, _, msg) = next_client_msg(&mut down_rx).await;
    assert_eq!(conn_id, 11);
    assert!(matches!(msg, ClientMessage::SessionAttach { .. }));
    // 无拒绝帧到达手机（SessionAttachDenied 已随控制权门退役）
    assert!(
        !tokio::time::timeout(std::time::Duration::from_millis(150), rx.recv())
            .await
            .map(|m| matches!(m, Some(ServerMessage::SessionAttachDenied { .. })))
            .unwrap_or(false),
        "不应再有控制权拒绝帧"
    );

    REMOTE_STATE.remove_conn(11);
    cleanup(&[inst], &[]);
}

/// rpc 上传粘连跨 chunk 保持；非上传命令清除
#[tokio::test]
async fn Gateway_Dispatch_rpc粘连跟随成员_4() {
    let _lock = super::REMOTE_SERVER_TEST_LOCK.lock();
    gateway::test_reset();
    let (mut down_rx, inst) = register_mock_member("rpc", &["rp-1"]).await;

    let (tx, _rx) = mpsc::channel(16);
    REMOTE_STATE.add_conn(12, tx, false);

    gateway::dispatch(
        12,
        &ClientMessage::RpcCall {
            id: 1,
            cmd: "file_upload_begin".into(),
            args: serde_json::json!({ "ptyId": "rp-1", "fileName": "a.txt", "size": 1 }),
        },
    )
    .await;
    let (conn_id, _, _) = next_client_msg(&mut down_rx).await;
    assert_eq!(conn_id, 12);

    gateway::dispatch(
        12,
        &ClientMessage::RpcCall {
            id: 2,
            cmd: "file_upload_chunk".into(),
            args: serde_json::json!({ "uploadId": "u1", "offset": 0 }),
        },
    )
    .await;
    let (_, _, msg) = next_client_msg(&mut down_rx).await;
    assert!(matches!(msg, ClientMessage::RpcCall { ref cmd, .. } if cmd == "file_upload_chunk"));

    // 非上传命令（带 ptyId）转发后清粘连
    gateway::dispatch(
        12,
        &ClientMessage::RpcCall {
            id: 3,
            cmd: "rename_session".into(),
            args: serde_json::json!({ "ptyId": "rp-1", "name": "n" }),
        },
    )
    .await;
    let (_, _, msg) = next_client_msg(&mut down_rx).await;
    assert!(matches!(msg, ClientMessage::RpcCall { ref cmd, .. } if cmd == "rename_session"));

    // 粘连已清：后续无 ptyId 的 chunk 被拒绝（未知会话），不再转发
    gateway::dispatch(
        12,
        &ClientMessage::RpcCall {
            id: 4,
            cmd: "file_upload_chunk".into(),
            args: serde_json::json!({ "uploadId": "u2", "offset": 0 }),
        },
    )
    .await;
    assert_no_client_msg(&mut down_rx).await;
    REMOTE_STATE.remove_conn(12);
    cleanup(&[inst], &[]);
}

/// create 带 instanceId 精确路由；旧手机不带 → 默认成员
#[tokio::test]
async fn Gateway_Dispatch_Create按实例路由_5() {
    let _lock = super::REMOTE_SERVER_TEST_LOCK.lock();
    gateway::test_reset();
    let (mut down_rx, inst) = register_mock_member("create", &[]).await;

    let (tx, _rx) = mpsc::channel(16);
    REMOTE_STATE.add_conn(13, tx, false);

    gateway::dispatch(
        13,
        &ClientMessage::SessionCreate {
            path: "/proj".into(),
            session_id: None,
            skip_permissions: None,
            custom_args: None,
            session_name: None,
            instance_id: Some(inst.clone()),
        },
    )
    .await;
    let (conn_id, _, _) = next_client_msg(&mut down_rx).await;
    assert_eq!(conn_id, 13);
    gateway::dispatch(
        13,
        &ClientMessage::SessionCreate {
            path: "/proj".into(),
            session_id: None,
            skip_permissions: None,
            custom_args: None,
            session_name: None,
            instance_id: None,
        },
    )
    .await;
    let (conn_id, _, _) = next_client_msg(&mut down_rx).await;
    assert_eq!(conn_id, 13, "旧手机创建应落到默认成员");

    REMOTE_STATE.remove_conn(13);
    cleanup(&[inst], &[]);
}

/// 手机断连（ConnDrop）：成员侧 remove_conn 自动归还该连接接管的终端
#[tokio::test]
async fn Gateway_断连归还终端_6() {
    let _lock = super::REMOTE_SERVER_TEST_LOCK.lock();
    REMOTE_STATE.register_pty("strip-1", "claude", "/p");
    let (tx, mut rx) = mpsc::channel(16);
    REMOTE_STATE.add_member_conn(42, tx, false);
    REMOTE_STATE.handle_client_message(
        42,
        &ClientMessage::SessionAttach {
            id: "strip-1".into(),
            cols: 80,
            rows: 24,
        },
    );
    let _ = recv_until(&mut rx, |m| matches!(m, ServerMessage::SessionAttached { .. })).await;
    assert!(REMOTE_STATE.is_remote_active("strip-1"));

    REMOTE_STATE.remove_conn(42);
    assert!(!REMOTE_STATE.is_remote_active("strip-1"), "断连后终端归还桌面");

    REMOTE_STATE.remove_pty("strip-1");
}

/// 实例名单：所有注册成员（含零会话），不再注入 Broker 自身
#[tokio::test]
async fn Gateway_实例名单_含零会话实例_7() {
    let _lock = super::REMOTE_SERVER_TEST_LOCK.lock();
    gateway::test_reset();
    let (_rx_a, inst_a) = register_mock_member("roster-a", &[]).await;
    let (_rx_b, inst_b) = register_mock_member("roster-b", &[]).await;

    let roster = gateway::instance_roster();
    let pos_a = roster.iter().position(|(id, _)| id == &inst_a).expect("A");
    let pos_b = roster.iter().position(|(id, _)| id == &inst_b).expect("B");
    assert!(pos_b > pos_a);
    assert!(!roster.iter().any(|(_, name)| name == "实例 1"));

    let (tx, mut rx) = mpsc::channel(16);
    REMOTE_STATE.add_conn(77, tx, false);
    gateway::dispatch(77, &ClientMessage::SessionsList).await;
    let payload = recv_until(&mut rx, |m| {
        matches!(m, ServerMessage::RpcEvent { name, .. } if name == crate::events::names::INSTANCES_CHANGED)
    })
    .await;
    if let ServerMessage::RpcEvent { payload, .. } = payload {
        let list = payload.get("instances").and_then(|v| v.as_array()).unwrap();
        assert!(list.iter().any(|i| i.get("id").and_then(|v| v.as_str()) == Some(inst_a.as_str())));
    }

    REMOTE_STATE.remove_conn(77);
    cleanup(&[inst_a, inst_b], &[]);
}

/// 未认证中继连接不得经 Broker 转发到成员
#[tokio::test]
async fn Gateway_转发_未认证连接被拒_8() {
    let _lock = super::REMOTE_SERVER_TEST_LOCK.lock();
    gateway::test_reset();
    let (mut down_rx, inst) = register_mock_member("auth", &["auth-1"]).await;

    let (tx, _rx) = mpsc::channel(16);
    REMOTE_STATE.add_conn(21, tx, true);
    gateway::dispatch(
        21,
        &ClientMessage::SessionAttach {
            id: "auth-1".into(),
            cols: 80,
            rows: 24,
        },
    )
    .await;
    assert_no_client_msg(&mut down_rx).await;
    cleanup(&[inst], &[]);
}

/// 目标成员下线：SessionCreate 显式失败
#[tokio::test]
async fn Gateway_Create_目标实例不可用回失败_9() {
    let _lock = super::REMOTE_SERVER_TEST_LOCK.lock();
    gateway::test_reset();
    let (_down_rx, inst) = register_mock_member("gone", &[]).await;
    gateway::unregister_member(&inst);

    let (tx, mut rx) = mpsc::channel(16);
    REMOTE_STATE.add_conn(22, tx, false);
    gateway::dispatch(
        22,
        &ClientMessage::SessionCreate {
            path: "/proj".into(),
            session_id: None,
            skip_permissions: None,
            custom_args: None,
            session_name: None,
            instance_id: Some(inst),
        },
    )
    .await;
    let _ = recv_until(&mut rx, |m| matches!(m, ServerMessage::SessionCreateFailed { .. })).await;
    REMOTE_STATE.remove_conn(22);
    cleanup(&[], &[]);
}

/// 多分块上传粘连保持到 finish
#[tokio::test]
async fn Gateway_上传粘连_跨分块保持_10() {
    let _lock = super::REMOTE_SERVER_TEST_LOCK.lock();
    gateway::test_reset();
    let (mut down_rx, inst) = register_mock_member("up", &["up-1"]).await;

    let (tx, _rx) = mpsc::channel(16);
    REMOTE_STATE.add_conn(23, tx, false);

    gateway::dispatch(
        23,
        &ClientMessage::RpcCall {
            id: 1,
            cmd: "file_upload_begin".into(),
            args: serde_json::json!({ "ptyId": "up-1", "fileName": "a.bin", "size": 3 }),
        },
    )
    .await;
    let _ = next_client_msg(&mut down_rx).await;

    for (n, id) in [2u64, 3].iter().enumerate() {
        gateway::dispatch(
            23,
            &ClientMessage::RpcCall {
                id: *id,
                cmd: "file_upload_chunk".into(),
                args: serde_json::json!({ "uploadId": "u1", "offset": n * 1024 }),
            },
        )
        .await;
        let (conn_id, _, msg) = next_client_msg(&mut down_rx).await;
        assert_eq!(conn_id, 23);
        assert!(matches!(msg, ClientMessage::RpcCall { ref cmd, .. } if cmd == "file_upload_chunk"));
    }

    gateway::dispatch(
        23,
        &ClientMessage::RpcCall {
            id: 4,
            cmd: "file_upload_finish".into(),
            args: serde_json::json!({ "uploadId": "u1" }),
        },
    )
    .await;
    let _ = next_client_msg(&mut down_rx).await;
    gateway::dispatch(
        23,
        &ClientMessage::RpcCall {
            id: 5,
            cmd: "file_upload_chunk".into(),
            args: serde_json::json!({ "uploadId": "u2", "offset": 0 }),
        },
    )
    .await;
    assert_no_client_msg(&mut down_rx).await;
    REMOTE_STATE.remove_conn(23);
    cleanup(&[inst], &[]);
}

/// 未知 pty_id：拒绝 + 定向会话快照，不转发成员
#[tokio::test]
async fn Gateway_未知pty拒绝并回快照_11() {
    let _lock = super::REMOTE_SERVER_TEST_LOCK.lock();
    gateway::test_reset();
    let (_down_rx, inst) = register_mock_member("unknown", &[]).await;
    let (tx, mut rx) = mpsc::channel(64);
    REMOTE_STATE.add_conn(31, tx, false);

    gateway::dispatch(
        31,
        &ClientMessage::SessionAttach {
            id: "nope".into(),
            cols: 80,
            rows: 24,
        },
    )
    .await;
    let _ = recv_until(&mut rx, |m| {
        matches!(m, ServerMessage::SessionAttachDenied { id, .. } if id == "nope")
    })
    .await;
    // 定向快照
    let _ = recv_until(&mut rx, |m| matches!(m, ServerMessage::Sessions { .. })).await;

    REMOTE_STATE.remove_conn(31);
    cleanup(&[inst], &[]);
}

/// SessionCreated 到达时先登记路由，再投递手机
#[tokio::test]
async fn Gateway_SessionCreated先登记再投递_12() {
    let _lock = super::REMOTE_SERVER_TEST_LOCK.lock();
    gateway::test_reset();
    let (_down_rx, inst) = register_mock_member("created", &[]).await;
    let (tx, mut rx) = mpsc::channel(16);
    REMOTE_STATE.add_conn(32, tx, false);

    let ok = gateway::handle_member_up(
        &inst,
        GwUp::ToConn {
            conn_id: 32,
            msg: ServerMessage::SessionCreated {
                id: "new-1".into(),
                path: "/proj".into(),
            },
        },
    )
    .await;
    assert!(ok);
    assert_eq!(gateway::test_route_of("new-1").as_deref(), Some(inst.as_str()));
    let _ = recv_until(&mut rx, |m| {
        matches!(m, ServerMessage::SessionCreated { id, .. } if id == "new-1")
    })
    .await;
    assert!(gateway::merged_session_list(Vec::new()).iter().any(|s| s.id == "new-1"));

    REMOTE_STATE.remove_conn(32);
    cleanup(&[inst], &[]);
}

/// 成员断连：待处理创建/RPC 明确失败
#[tokio::test]
async fn Gateway_断连清理待处理请求_13() {
    let _lock = super::REMOTE_SERVER_TEST_LOCK.lock();
    gateway::test_reset();
    let (mut down_rx, inst) = register_mock_member("pending", &["p-1"]).await;
    let (tx, mut rx) = mpsc::channel(32);
    REMOTE_STATE.add_conn(33, tx, false);

    gateway::dispatch(
        33,
        &ClientMessage::SessionCreate {
            path: "/proj".into(),
            session_id: None,
            skip_permissions: None,
            custom_args: None,
            session_name: None,
            instance_id: Some(inst.clone()),
        },
    )
    .await;
    let _ = next_client_msg(&mut down_rx).await;
    gateway::dispatch(
        33,
        &ClientMessage::RpcCall {
            id: 99,
            cmd: "file_upload_begin".into(),
            args: serde_json::json!({ "ptyId": "p-1" }),
        },
    )
    .await;
    let _ = next_client_msg(&mut down_rx).await;

    gateway::unregister_member(&inst);
    let mut got_create_failed = false;
    let mut got_rpc_failed = false;
    for _ in 0..16 {
        match tokio::time::timeout(std::time::Duration::from_millis(500), rx.recv()).await {
            Ok(Some(ServerMessage::SessionCreateFailed { .. })) => got_create_failed = true,
            Ok(Some(ServerMessage::RpcResult { id: 99, ok: false, .. })) => {
                got_rpc_failed = true
            }
            Ok(Some(_)) => {}
            _ => break,
        }
        if got_create_failed && got_rpc_failed {
            break;
        }
    }
    assert!(got_create_failed, "pending create must fail explicitly");
    assert!(got_rpc_failed, "pending rpc must fail explicitly");

    REMOTE_STATE.remove_conn(33);
    cleanup(&[], &[]);
}

/// 成员上报的 state-changed 由 Broker 重新分配设备级 revision
#[tokio::test]
async fn Gateway_成员StateChanged重排Revision_15() {
    let _lock = super::REMOTE_SERVER_TEST_LOCK.lock();
    gateway::test_reset();
    let (_down_rx, inst) = register_mock_member("state", &[]).await;
    let (tx, mut rx) = mpsc::channel(16);
    REMOTE_STATE.add_conn(35, tx, false);

    let member_rev = "99";
    let ok = gateway::handle_member_up(
        &inst,
        GwUp::ToConn {
            conn_id: 35,
            msg: ServerMessage::RpcEvent {
                name: crate::events::names::STATE_CHANGED.to_string(),
                payload: serde_json::json!({ "id": "config", "data": member_rev }),
            },
        },
    )
    .await;
    assert!(ok);

    let msg = recv_until(&mut rx, |m| {
        matches!(m, ServerMessage::RpcEvent { name, .. } if name == crate::events::names::STATE_CHANGED)
    })
    .await;
    let (id, data) = match msg {
        ServerMessage::RpcEvent { payload, .. } => (
            payload.get("id").and_then(|v| v.as_str()).unwrap_or("").to_string(),
            payload.get("data").and_then(|v| v.as_str()).unwrap_or("").to_string(),
        ),
        _ => unreachable!(),
    };
    assert_eq!(id, "config");
    assert_ne!(data, member_rev, "不得透传成员 revision");
    assert_eq!(
        data.parse::<u64>().ok(),
        Some(REMOTE_STATE.revision()),
        "应使用 Broker 当前 revision"
    );

    REMOTE_STATE.remove_conn(35);
    cleanup(&[inst], &[]);
}

/// instances-changed 区分 solicited：sessions.list 应答 vs 主动通知
#[tokio::test]
async fn Gateway_实例事件区分Solicited_16() {
    let _lock = super::REMOTE_SERVER_TEST_LOCK.lock();
    gateway::test_reset();
    let (_down_rx, inst) = register_mock_member("roster2", &[]).await;
    let (tx, mut rx) = mpsc::channel(16);
    REMOTE_STATE.add_conn(36, tx, false);

    gateway::dispatch(36, &ClientMessage::SessionsList).await;
    let msg = recv_until(&mut rx, |m| {
        matches!(m, ServerMessage::RpcEvent { name, .. } if name == crate::events::names::INSTANCES_CHANGED)
    })
    .await;
    if let ServerMessage::RpcEvent { payload, .. } = msg {
        assert_eq!(payload.get("solicited").and_then(|v| v.as_bool()), Some(true));
    }

    gateway::notify_sessions_changed();
    let msg = recv_until(&mut rx, |m| {
        matches!(m, ServerMessage::RpcEvent { name, .. } if name == crate::events::names::INSTANCES_CHANGED)
    })
    .await;
    if let ServerMessage::RpcEvent { payload, .. } = msg {
        assert_eq!(payload.get("solicited").and_then(|v| v.as_bool()), Some(false));
    }

    REMOTE_STATE.remove_conn(36);
    cleanup(&[inst], &[]);
}

/// 窗口快照：v2 带 PID 可关闭；关闭指令进入成员有序队列
#[tokio::test]
async fn Gateway_WindowRosterAndCommand_15() {
    let _lock = super::REMOTE_SERVER_TEST_LOCK.lock();
    gateway::test_reset();
    let (mut down_rx, inst) = register_mock_member("w1", &["s-win"]).await;

    let roster = gateway::window_roster();
    assert_eq!(roster.len(), 1);
    assert_eq!(roster[0].id, inst);
    assert_eq!(roster[0].pid, std::process::id());
    assert!(roster[0].closable);

    gateway::handle_member_up(
        &inst,
        GwUp::WindowTitle {
            title: "project-a".to_string(),
        },
    )
    .await;
    let roster = gateway::window_roster();
    assert_eq!(roster[0].title, "project-a");
    // 手机分组名也跟随窗口标题，而不是注册时的 hostname/label
    assert_eq!(roster[0].name, "project-a");
    let merged = gateway::merged_session_list(Vec::new());
    assert!(
        merged
            .iter()
            .all(|s| s.instance_name.as_deref() == Some("project-a")),
        "existing sessions should use the updated window title"
    );

    gateway::send_window_command(&inst, WindowAction::Close)
        .await
        .expect("v2 command should enqueue");
    let got = tokio::time::timeout(std::time::Duration::from_millis(500), down_rx.recv())
        .await
        .expect("window command expected")
        .expect("open");
    assert_eq!(
        got,
        GwDown::WindowCommand {
            action: WindowAction::Close
        }
    );

    // 显示 / 打开远程面板同样走有序队列
    for action in [WindowAction::Show, WindowAction::OpenRemote] {
        gateway::send_window_command(&inst, action)
            .await
            .expect("window action should enqueue");
        let got = tokio::time::timeout(
            std::time::Duration::from_millis(500),
            down_rx.recv(),
        )
        .await
        .expect("window action expected")
        .expect("open");
        assert_eq!(got, GwDown::WindowCommand { action });
    }
    cleanup(&[inst], &[]);
}

/// 成员 state-changed 上行后，Broker 扇出给**其他**成员与手机；
/// 不得回发发起成员——发起端已有本地 Tauri 通知 + 本端写回环抑制，
/// 二次回环会击穿抑制，触发发起端全量刷新（桌面隐藏项目列表闪动回归）
#[tokio::test]
async fn Gateway_StateChangedFanout_18() {
    let _lock = super::REMOTE_SERVER_TEST_LOCK.lock();
    gateway::test_reset();
    let (mut origin_rx, origin) = register_mock_member("state", &[]).await;
    let (mut peer_rx, peer) = register_mock_member("state-peer", &[]).await;

    let (tx, mut rx) = mpsc::channel(16);
    REMOTE_STATE.add_conn(31, tx, false);

    gateway::handle_member_up(
        &origin,
        GwUp::StateChanged {
            section: "providers".to_string(),
        },
    )
    .await;

    // 其他成员照常收到扇出
    let member_msg = tokio::time::timeout(
        std::time::Duration::from_millis(500),
        peer_rx.recv(),
    )
    .await
    .expect("desktop event expected")
    .expect("open");
    match member_msg {
        GwDown::DesktopEvent { name, payload } => {
            assert_eq!(name, crate::events::names::STATE_CHANGED);
            assert_eq!(payload.get("id").and_then(|v| v.as_str()), Some("providers"));
            assert!(payload
                .get("data")
                .and_then(|v| v.as_str())
                .and_then(|v| v.parse::<u64>().ok())
                .unwrap_or(0)
                > 0);
        }
        other => panic!("unexpected member frame: {:?}", other),
    }

    // 发起成员不得回发（回环抑制只登记一次，二次事件即全量刷新）
    match tokio::time::timeout(
        std::time::Duration::from_millis(150),
        origin_rx.recv(),
    )
    .await
    {
        Err(_) | Ok(None) => {} // 超时/关闭 = 未回发，符合预期
        Ok(Some(frame)) => {
            panic!("origin member must not receive its own state-changed: {:?}", frame)
        }
    }

    let deadline = tokio::time::Instant::now() + std::time::Duration::from_millis(500);
    let mut saw_state = false;
    while !saw_state {
        let msg = tokio::time::timeout_at(deadline, rx.recv())
            .await
            .expect("phone event expected")
            .expect("open");
        saw_state = matches!(
            msg,
            ServerMessage::RpcEvent { ref name, .. }
                if name == crate::events::names::STATE_CHANGED
        );
    }

    REMOTE_STATE.remove_conn(31);
    cleanup(&[origin, peer], &[]);
}

/// 裸 pty.input 只被原样转发（无任何控制面副作用；未 attach 时成员端丢弃输入）
#[tokio::test]
async fn Gateway_PtyInputBareForwardOnly_19() {
    let _lock = super::REMOTE_SERVER_TEST_LOCK.lock();
    gateway::test_reset();
    let (mut down_rx, inst) = register_mock_member("input", &["in-1"]).await;

    let (tx, _rx) = mpsc::channel(16);
    REMOTE_STATE.add_conn(32, tx, false);

    gateway::dispatch(
        32,
        &ClientMessage::PtyInput {
            id: "in-1".into(),
            data: "x".into(),
        },
    )
    .await;
    let (conn_id, _, msg) = next_client_msg(&mut down_rx).await;
    assert_eq!(conn_id, 32);
    assert!(matches!(msg, ClientMessage::PtyInput { ref id, .. } if id == "in-1"));

    REMOTE_STATE.remove_conn(32);
    cleanup(&[inst], &[]);
}

/// 托盘“远程连接”/“显示”选择第一个可控制窗口并下发对应动作
#[tokio::test]
async fn Gateway_ShowAndOpenRemote_17() {
    let _lock = super::REMOTE_SERVER_TEST_LOCK.lock();
    gateway::test_reset();
    let (mut down_rx, inst) = register_mock_member("remote", &[]).await;

    crate::broker::windows::open_remote_panel()
        .await
        .expect("open remote should enqueue");
    let got = tokio::time::timeout(std::time::Duration::from_millis(500), down_rx.recv())
        .await
        .expect("open remote command expected")
        .expect("open");
    assert_eq!(
        got,
        GwDown::WindowCommand {
            action: WindowAction::OpenRemote
        }
    );

    crate::broker::windows::show_window(&inst)
        .await
        .expect("show should enqueue");
    let got = tokio::time::timeout(std::time::Duration::from_millis(500), down_rx.recv())
        .await
        .expect("show command expected")
        .expect("open");
    assert_eq!(
        got,
        GwDown::WindowCommand {
            action: WindowAction::Show
        }
    );
    cleanup(&[inst], &[]);
}

/// v1 旧窗口没有 PID/WindowCommand：名单可见但不可控制
#[tokio::test]
async fn Gateway_WindowRoster_LegacyNotClosable_16() {
    let _lock = super::REMOTE_SERVER_TEST_LOCK.lock();
    gateway::test_reset();
    let (_down_rx, inst) = register_mock_member_with("old", LEGACY_PROTO_VER, &[]).await;

    let roster = gateway::window_roster();
    assert_eq!(roster.len(), 1);
    assert!(!roster[0].closable);
    assert!(gateway::send_window_command(&inst, WindowAction::Close)
        .await
        .is_err());
    cleanup(&[inst], &[]);
}

/// v2-only 消息不发给旧版成员；通用消息仍可达
#[tokio::test]
async fn Gateway_V2Only不发给V1_14() {
    let _lock = super::REMOTE_SERVER_TEST_LOCK.lock();
    gateway::test_reset();
    let (mut down_rx, inst) = register_mock_member_with("v1", LEGACY_PROTO_VER, &[]).await;

    gateway::broadcast_down(GwDown::BrokerExit {
        reason: crate::gateway_proto::BrokerExitReason::Idle,
    });
    assert_no_client_msg(&mut down_rx).await;
    // 非 ClientMsg 控制帧可能在通道里；显式断言没有 BrokerExit
    while let Ok(msg) = down_rx.try_recv() {
        assert!(!matches!(msg, GwDown::BrokerExit { .. }));
    }

    gateway::broadcast_down(GwDown::Shutdown);
    // 并行测试可能向全局成员广播其它控制帧；在超时内找到 Shutdown 即可
    let deadline = tokio::time::Instant::now() + std::time::Duration::from_millis(500);
    loop {
        let got = tokio::time::timeout_at(deadline, down_rx.recv())
            .await
            .expect("shutdown expected")
            .expect("open");
        if matches!(got, GwDown::Shutdown) {
            break;
        }
        assert!(
            !matches!(got, GwDown::BrokerExit { .. }),
            "v2-only BrokerExit must not reach v1"
        );
    }
    cleanup(&[inst], &[]);
}
