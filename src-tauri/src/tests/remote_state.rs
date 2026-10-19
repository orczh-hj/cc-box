//! remote_state 单元测试：working/pending 状态机、tail 截断、订阅广播、消息分发
//!
//! 每个测试使用独立的 RemoteState 实例（避免并行测试共享全局单例互相干扰，
//! 且 recv 均带超时防御，防止挂起）。

use cc_box_remote_protocol::{ClientMessage, ServerMessage};
use tokio::sync::mpsc;

use crate::hook_events::{
    HookEventDetail, HookPayload, NotificationData, PostToolUseData, StopData, UserPromptSubmitData,
};
use crate::remote_state::RemoteState;

/// recv 超时（ms）：任何未到达的消息视为测试失败而非永久挂起
/// 部分回包经 spawn 线程异步发出（如 session.create 白名单拒绝），留足调度余量
const RECV_TIMEOUT_MS: u64 = 3000;

/// 构造最小 HookPayload（detail 类型由调用方给）
fn hook(pty_id: &str, detail: HookEventDetail) -> HookPayload {
    HookPayload {
        pty_id: Some(pty_id.to_string()),
        session_id: None,
        event_name: String::new(),
        state: String::new(),
        timestamp: 0,
        detail,
    }
}

fn user_prompt() -> HookEventDetail {
    HookEventDetail::UserPromptSubmit(UserPromptSubmitData { prompt: None })
}

fn post_tool_use() -> HookEventDetail {
    HookEventDetail::PostToolUse(PostToolUseData { tool_name: None, tool_use_id: None })
}

fn stop() -> HookEventDetail {
    HookEventDetail::Stop(StopData { stop_hook_active: None, last_assistant_message: None })
}

fn notification(ntype: &str) -> HookEventDetail {
    HookEventDetail::Notification(NotificationData {
        notification_type: Some(ntype.to_string()),
        message: None,
        title: None,
    })
}

/// 注册一个测试连接，返回 (conn_id, receiver)
fn setup_conn(state: &RemoteState) -> (u64, mpsc::Receiver<ServerMessage>) {
    let (tx, rx) = mpsc::channel(64);
    let conn_id = crate::remote_state::next_conn_id();
    state.add_conn(conn_id, tx);
    (conn_id, rx)
}

async fn recv_timeout(rx: &mut mpsc::Receiver<ServerMessage>) -> ServerMessage {
    match tokio::time::timeout(std::time::Duration::from_millis(RECV_TIMEOUT_MS), rx.recv()).await {
        Ok(Some(msg)) => msg,
        Ok(None) => panic!("channel closed"),
        Err(_) => panic!("recv timeout: expected message never arrived"),
    }
}

#[test]
fn RemoteState_HookStateMachine_UserPrompt开启working_Stop后pending() {
    let state = RemoteState::new();
    state.register_pty("p1", "claude", "/tmp");

    state.on_hook_payload(&hook("p1", user_prompt()));
    let list = state.session_list();
    assert_eq!(list.len(), 1);
    assert!(list[0].working);
    assert!(!list[0].pending);

    state.on_hook_payload(&hook("p1", stop()));
    let list = state.session_list();
    assert!(!list[0].working);
    assert!(list[0].pending);
}

#[test]
fn RemoteState_HookStateMachine_回合结束后activity不恢复working() {
    let state = RemoteState::new();
    state.register_pty("p2", "claude", "/tmp");

    // prompt → working，stop → 回合结束
    state.on_hook_payload(&hook("p2", user_prompt()));
    state.on_hook_payload(&hook("p2", stop()));
    assert!(!state.session_list()[0].working);

    // 回合结束后的 activity（如 recap 的 PostToolUse）不应恢复 working
    state.on_hook_payload(&hook("p2", post_tool_use()));
    assert!(!state.session_list()[0].working);

    // 新 prompt 重新开启回合
    state.on_hook_payload(&hook("p2", user_prompt()));
    assert!(state.session_list()[0].working);

    // 回合内的 activity 恢复 working 并清 pending
    state.on_hook_payload(&hook("p2", post_tool_use()));
    let list = state.session_list();
    assert!(list[0].working);
    assert!(!list[0].pending);
}

#[test]
fn RemoteState_HookStateMachine_Permission通知进入pending且授权后恢复() {
    let state = RemoteState::new();
    state.register_pty("p3", "claude", "/tmp");

    state.on_hook_payload(&hook("p3", user_prompt()));

    // permission_prompt：working=false + pending=true，回合未结束
    state.on_hook_payload(&hook("p3", notification("permission_prompt")));
    let list = state.session_list();
    assert!(!list[0].working);
    assert!(list[0].pending);

    // 授权后的 activity 恢复 working（回合未结束）
    state.on_hook_payload(&hook("p3", post_tool_use()));
    let list = state.session_list();
    assert!(list[0].working);
    assert!(!list[0].pending);
}

#[test]
fn RemoteState_HookStateMachine_Idle通知结束回合进入pending() {
    let state = RemoteState::new();
    state.register_pty("p4", "claude", "/tmp");

    state.on_hook_payload(&hook("p4", user_prompt()));
    state.on_hook_payload(&hook("p4", notification("idle_prompt")));
    let list = state.session_list();
    assert!(!list[0].working);
    assert!(list[0].pending);

    // 回合已结束，后续 activity 不恢复
    state.on_hook_payload(&hook("p4", post_tool_use()));
    assert!(!state.session_list()[0].working);
}

#[test]
fn RemoteState_HookStateMachine_未注册的pty事件被忽略() {
    let state = RemoteState::new();
    state.on_hook_payload(&hook("ghost", user_prompt()));
    assert!(state.session_list().is_empty());
}

#[tokio::test]
async fn RemoteState_Subscribe_订阅后收到输出与状态推送() {
    let state = RemoteState::new();
    state.register_pty("p5", "claude", "/tmp");
    let (conn, mut rx) = setup_conn(&state);

    // 订阅：先收到 subscribed（tail 为空）
    state.handle_client_message(conn, &ClientMessage::SessionSubscribe { id: "p5".into() });
    match recv_timeout(&mut rx).await {
        ServerMessage::SessionSubscribed { id, tail } => {
            assert_eq!(id, "p5");
            assert!(tail.is_empty());
        }
        other => panic!("expected SessionSubscribed, got {:?}", other),
    }

    // 输出 tee → pty.output
    state.push_output("p5", "hello ");
    state.push_output("p5", "world");
    let mut got = String::new();
    for _ in 0..2 {
        match recv_timeout(&mut rx).await {
            ServerMessage::PtyOutput { data, .. } => got.push_str(&data),
            other => panic!("expected PtyOutput, got {:?}", other),
        }
    }
    assert_eq!(got, "hello world");

    // hook 状态变化 → session.status
    state.on_hook_payload(&hook("p5", user_prompt()));
    match recv_timeout(&mut rx).await {
        ServerMessage::SessionStatus { working, pending, .. } => {
            assert!(working);
            assert!(!pending);
        }
        other => panic!("expected SessionStatus, got {:?}", other),
    }

    // PTY 移除 → session.exit
    state.remove_pty("p5");
    match recv_timeout(&mut rx).await {
        ServerMessage::SessionExit { id } => assert_eq!(id, "p5"),
        other => panic!("expected SessionExit, got {:?}", other),
    }
}

#[tokio::test]
async fn RemoteState_Subscribe_订阅不存在的会话收到exit() {
    let state = RemoteState::new();
    let (conn, mut rx) = setup_conn(&state);
    state.handle_client_message(conn, &ClientMessage::SessionSubscribe { id: "nope".into() });
    match recv_timeout(&mut rx).await {
        ServerMessage::SessionExit { id } => assert_eq!(id, "nope"),
        other => panic!("expected SessionExit, got {:?}", other),
    }
}

#[tokio::test]
async fn RemoteState_Subscribe_退订后不再收到输出() {
    let state = RemoteState::new();
    state.register_pty("p6", "claude", "/tmp");
    let (conn, mut rx) = setup_conn(&state);

    state.handle_client_message(conn, &ClientMessage::SessionSubscribe { id: "p6".into() });
    let _ = recv_timeout(&mut rx).await; // subscribed

    state.handle_client_message(conn, &ClientMessage::SessionUnsubscribed { id: "p6".into() });
    match recv_timeout(&mut rx).await {
        ServerMessage::SessionUnsubscribed { .. } => {}
        other => panic!("expected SessionUnsubscribed, got {:?}", other),
    }

    state.push_output("p6", "after unsubscribe");
    let res = tokio::time::timeout(std::time::Duration::from_millis(100), rx.recv()).await;
    assert!(res.is_err(), "should not receive output after unsubscribe");
}

#[tokio::test]
async fn RemoteState_Conn_连接断开清理全部订阅() {
    let state = RemoteState::new();
    state.register_pty("p7", "claude", "/tmp");
    state.register_pty("p8", "shell", "/tmp");
    let (conn, mut rx) = setup_conn(&state);

    state.handle_client_message(conn, &ClientMessage::SessionSubscribe { id: "p7".into() });
    state.handle_client_message(conn, &ClientMessage::SessionSubscribe { id: "p8".into() });
    let _ = recv_timeout(&mut rx).await;
    let _ = recv_timeout(&mut rx).await;

    state.remove_conn(conn);
    state.push_output("p7", "gone");
    state.push_output("p8", "gone");
    // remove_conn 会 drop sender：recv 可能超时（Err）也可能立即返回 None（通道关闭），
    // 两者都表示"收不到输出"
    let res = tokio::time::timeout(std::time::Duration::from_millis(100), rx.recv()).await;
    assert!(
        res.is_err() || matches!(res, Ok(None)),
        "connection removal should clear all subscriptions"
    );
}

#[tokio::test]
async fn RemoteState_SessionsList_按注册顺序返回() {
    let state = RemoteState::new();
    state.register_pty("s1", "claude", "/a");
    state.register_pty("s2", "shell", "/b");
    let (conn, mut rx) = setup_conn(&state);

    state.handle_client_message(conn, &ClientMessage::SessionsList);
    match recv_timeout(&mut rx).await {
        ServerMessage::Sessions { list } => {
            assert_eq!(list.len(), 2);
            assert_eq!(list[0].id, "s1");
            assert_eq!(list[0].pty_type, "claude");
            assert_eq!(list[1].id, "s2");
        }
        other => panic!("expected Sessions, got {:?}", other),
    }
}

#[tokio::test]
async fn RemoteState_Ping_回复Pong() {
    let state = RemoteState::new();
    let (conn, mut rx) = setup_conn(&state);
    state.handle_client_message(conn, &ClientMessage::Ping);
    match recv_timeout(&mut rx).await {
        ServerMessage::Pong => {}
        other => panic!("expected Pong, got {:?}", other),
    }
}

#[tokio::test]
async fn RemoteState_Tail_订阅时附带最近输出() {
    let state = RemoteState::new();
    state.register_pty("p9", "claude", "/tmp");
    state.push_output("p9", "line1\n");
    state.push_output("p9", "line2\n");

    let (conn, mut rx) = setup_conn(&state);
    state.handle_client_message(conn, &ClientMessage::SessionSubscribe { id: "p9".into() });
    match recv_timeout(&mut rx).await {
        ServerMessage::SessionSubscribed { tail, .. } => {
            assert_eq!(tail, "line1\nline2\n");
        }
        other => panic!("expected SessionSubscribed, got {:?}", other),
    }
}

#[tokio::test]
async fn RemoteState_Tail_超过上限截断保留尾部() {
    let state = RemoteState::new();
    state.register_pty("p10", "claude", "/tmp");

    // 写入超过 TAIL_LIMIT 的内容（含多字节中文字符）
    let chunk = "中文内容块".repeat(5000);
    state.push_output("p10", &format!("HEAD{}", chunk));
    state.push_output("p10", "TAIL_MARKER");

    let (conn, mut rx) = setup_conn(&state);
    state.handle_client_message(conn, &ClientMessage::SessionSubscribe { id: "p10".into() });
    match recv_timeout(&mut rx).await {
        ServerMessage::SessionSubscribed { tail, .. } => {
            assert!(tail.len() <= crate::remote_state::TAIL_LIMIT + "TAIL_MARKER".len());
            assert!(tail.ends_with("TAIL_MARKER"));
            assert!(!tail.contains("HEAD"), "head should be truncated");
            // UTF-8 边界安全：起点是字符边界，不含残缺字节
            assert!(std::str::from_utf8(tail.as_bytes()).is_ok());
        }
        other => panic!("expected SessionSubscribed, got {:?}", other),
    }
}

// ==================== R2：attach 独占接管 ====================

#[tokio::test]
async fn RemoteState_Attach_接管成功附快照并成为输出订阅者() {
    let state = RemoteState::new();
    state.register_pty("a1", "claude", "/tmp");
    state.push_output("a1", "snapshot content\n");
    let (conn, mut rx) = setup_conn(&state);

    state.handle_client_message(
        conn,
        &ClientMessage::SessionAttach { id: "a1".into(), cols: 80, rows: 24 },
    );
    match recv_timeout(&mut rx).await {
        ServerMessage::SessionAttached { id, cols, rows, tail } => {
            assert_eq!(id, "a1");
            assert_eq!((cols, rows), (80, 24));
            assert_eq!(tail, "snapshot content\n");
        }
        other => panic!("expected SessionAttached, got {:?}", other),
    }

    // attach 后收到输出流
    state.push_output("a1", "live output");
    match recv_timeout(&mut rx).await {
        ServerMessage::PtyOutput { data, .. } => assert_eq!(data, "live output"),
        other => panic!("expected PtyOutput, got {:?}", other),
    }
}

#[tokio::test]
async fn RemoteState_Attach_抢占其他远程连接时旧连接收到遮罩广播且保留订阅() {
    let state = RemoteState::new();
    state.register_pty("a2", "claude", "/tmp");
    let (conn1, mut rx1) = setup_conn(&state);
    let (conn2, mut rx2) = setup_conn(&state);

    // conn1 接管（从桌面抢）：旁观者 conn2 也收到 taken-over 广播（其 UI 遮罩）
    state.handle_client_message(
        conn1,
        &ClientMessage::SessionAttach { id: "a2".into(), cols: 80, rows: 24 },
    );
    let _ = recv_timeout(&mut rx1).await; // attached
    match recv_timeout(&mut rx2).await {
        ServerMessage::RpcEvent { name, .. } => assert_eq!(name, "session-taken-over"),
        other => panic!("conn2 should observe taken-over broadcast, got {:?}", other),
    }

    // conn2 抢占
    state.handle_client_message(
        conn2,
        &ClientMessage::SessionAttach { id: "a2".into(), cols: 100, rows: 30 },
    );
    // 旧持有者收到 taken-over 广播（遮罩 + 接管回来按钮），不再是 detached
    match recv_timeout(&mut rx1).await {
        ServerMessage::RpcEvent { name, payload } => {
            assert_eq!(name, "session-taken-over");
            assert_eq!(payload["id"], "a2");
        }
        other => panic!("expected RpcEvent(session-taken-over), got {:?}", other),
    }
    match recv_timeout(&mut rx2).await {
        ServerMessage::SessionAttached { .. } => {}
        other => panic!("expected SessionAttached, got {:?}", other),
    }

    // 旧持有者保留订阅：数据层继续同步（被接管 = 暂时不显示，不是断流）
    state.push_output("a2", "for conn2");
    match recv_timeout(&mut rx2).await {
        ServerMessage::PtyOutput { data, .. } => assert_eq!(data, "for conn2"),
        other => panic!("expected PtyOutput, got {:?}", other),
    }
    match recv_timeout(&mut rx1).await {
        ServerMessage::PtyOutput { data, .. } => assert_eq!(data, "for conn2"),
        other => panic!("old holder should keep receiving output, got {:?}", other),
    }
}

#[tokio::test]
async fn RemoteState_Attach_非持有者输入被忽略不注入() {
    let state = RemoteState::new();
    state.register_pty("a3", "claude", "/tmp");
    let (conn1, mut rx1) = setup_conn(&state);
    let (conn2, _rx2) = setup_conn(&state);

    state.handle_client_message(
        conn1,
        &ClientMessage::SessionAttach { id: "a3".into(), cols: 80, rows: 24 },
    );
    let _ = recv_timeout(&mut rx1).await; // attached

    // conn2 未持有：input/resize 被忽略（无 PTY manager 也不 panic，静默跳过）
    state.handle_client_message(
        conn2,
        &ClientMessage::PtyInput { id: "a3".into(), data: "evil".into() },
    );
    state.handle_client_message(
        conn2,
        &ClientMessage::PtyResize { id: "a3".into(), cols: 10, rows: 10 },
    );
    // 无异常即通过（conn2 无消息回报）
}

#[tokio::test]
async fn RemoteState_Attach_主动detach归还会话() {
    let state = RemoteState::new();
    state.register_pty("a4", "claude", "/tmp");
    let (conn, mut rx) = setup_conn(&state);

    state.handle_client_message(
        conn,
        &ClientMessage::SessionAttach { id: "a4".into(), cols: 80, rows: 24 },
    );
    let _ = recv_timeout(&mut rx).await; // attached

    state.handle_client_message(conn, &ClientMessage::SessionDetach { id: "a4".into() });
    match recv_timeout(&mut rx).await {
        ServerMessage::SessionDetached { reason, .. } => assert_eq!(reason, "self"),
        other => panic!("expected SessionDetached, got {:?}", other),
    }

    // detach 后不再是订阅者
    state.push_output("a4", "after detach");
    let res = tokio::time::timeout(std::time::Duration::from_millis(100), rx.recv()).await;
    assert!(res.is_err() || matches!(res, Ok(None)));
}

#[tokio::test]
async fn RemoteState_Attach_连接断开自动归还会话() {
    let state = RemoteState::new();
    state.register_pty("a5", "claude", "/tmp");
    let (conn, mut rx) = setup_conn(&state);

    state.handle_client_message(
        conn,
        &ClientMessage::SessionAttach { id: "a5".into(), cols: 80, rows: 24 },
    );
    let _ = recv_timeout(&mut rx).await; // attached

    // 断连 → 自动归还
    state.remove_conn(conn);

    // 归还后另一连接可正常接管
    let (conn2, mut rx2) = setup_conn(&state);
    state.handle_client_message(
        conn2,
        &ClientMessage::SessionAttach { id: "a5".into(), cols: 90, rows: 25 },
    );
    match recv_timeout(&mut rx2).await {
        ServerMessage::SessionAttached { .. } => {}
        other => panic!("expected SessionAttached after reclaim, got {:?}", other),
    }
}

#[tokio::test]
async fn RemoteState_Attach_桌面claim收回并通知手机() {
    let state = RemoteState::new();
    state.register_pty("a6", "claude", "/tmp");
    let (conn, mut rx) = setup_conn(&state);

    state.handle_client_message(
        conn,
        &ClientMessage::SessionAttach { id: "a6".into(), cols: 80, rows: 24 },
    );
    let _ = recv_timeout(&mut rx).await; // attached

    // 桌面接管
    state.claim("a6");
    match recv_timeout(&mut rx).await {
        ServerMessage::SessionDetached { reason, .. } => assert_eq!(reason, "desktop"),
        other => panic!("expected SessionDetached, got {:?}", other),
    }

    // 手机输入不再有效（无消息回报、不 panic 即可）
    state.handle_client_message(
        conn,
        &ClientMessage::PtyInput { id: "a6".into(), data: "x".into() },
    );
}

#[tokio::test]
async fn RemoteState_Attach_会话退出时清理() {
    let state = RemoteState::new();
    state.register_pty("a7", "claude", "/tmp");
    let (conn, mut rx) = setup_conn(&state);

    state.handle_client_message(
        conn,
        &ClientMessage::SessionAttach { id: "a7".into(), cols: 80, rows: 24 },
    );
    let _ = recv_timeout(&mut rx).await; // attached

    state.remove_pty("a7");
    match recv_timeout(&mut rx).await {
        ServerMessage::SessionExit { id } => assert_eq!(id, "a7"),
        other => panic!("expected SessionExit, got {:?}", other),
    }
}

// ==================== hello 回复与 SessionCreate 防护（审核修复回归） ====================

/// hello 统一由唯一 handler 回复 hello.ok（中继模式依赖此路径，否则手机停在认证中）
#[tokio::test]
async fn RemoteState_Hello_统一回复HelloOk() {
    let state = RemoteState::new();
    let (conn, mut rx) = setup_conn(&state);

    state.handle_client_message(
        conn,
        &ClientMessage::Hello { ver: 1, client: "web".into(), token: "t".into() },
    );
    match recv_timeout(&mut rx).await {
        ServerMessage::HelloOk { ver, device_name } => {
            assert_eq!(ver, 1);
            assert!(!device_name.is_empty());
        }
        other => panic!("expected HelloOk, got {:?}", other),
    }
}

/// session.create 未知项目被白名单拒绝（防任意目录 spawn Claude 进程）
#[tokio::test]
async fn RemoteState_SessionCreate_未知项目拒绝() {
    let state = RemoteState::new();
    let (conn, mut rx) = setup_conn(&state);

    state.handle_client_message(
        conn,
        &ClientMessage::SessionCreate {
            path: "Z:/__cc_box_nonexistent_dir__".into(),
            session_id: None,
            skip_permissions: None,
            custom_args: None,
        },
    );
    match recv_timeout(&mut rx).await {
        ServerMessage::SessionCreateFailed { error } => assert_eq!(error, "unknown project"),
        other => panic!("expected SessionCreateFailed, got {:?}", other),
    }
}

/// session.create 在途上限：达到上限后立即拒绝（不进线程、不查白名单）
#[tokio::test]
async fn RemoteState_SessionCreate_并发上限拒绝() {
    let state = RemoteState::new();
    let (conn, mut rx) = setup_conn(&state);

    crate::remote_state::REMOTE_SPAWN_INFLIGHT.store(8, std::sync::atomic::Ordering::SeqCst);
    state.handle_client_message(
        conn,
        &ClientMessage::SessionCreate { path: "/any".into(), session_id: None, skip_permissions: None, custom_args: None },
    );
    match recv_timeout(&mut rx).await {
        ServerMessage::SessionCreateFailed { error } => {
            assert_eq!(error, "too many concurrent requests")
        }
        other => panic!("expected SessionCreateFailed, got {:?}", other),
    }
    crate::remote_state::REMOTE_SPAWN_INFLIGHT.store(0, std::sync::atomic::Ordering::SeqCst);
}

/// 路径等价：canonicalize 消大小写/分隔符差异，fallback 归一化字符串
#[test]
fn RemoteState_PathsEquivalent_归一化比较() {
    use crate::remote_state::paths_equivalent;
    // 双方都存在的路径：canonicalize 后一致（Windows 大小写不敏感）
    let cwd = std::env::current_dir().unwrap();
    let mixed = if cfg!(windows) {
        cwd.to_string_lossy().to_uppercase()
    } else {
        cwd.to_string_lossy().into_owned()
    };
    assert!(paths_equivalent(cwd.to_str().unwrap(), &mixed));
    // 不存在的路径：归一化字符串比较（尾分隔符忽略、大小写折叠）
    assert!(paths_equivalent("Z:/no/such/dir/", "z:/no/such/dir"));
    assert!(!paths_equivalent("Z:/no/such/dir", "Z:/no/such/other"));
}
