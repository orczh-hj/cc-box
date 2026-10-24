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

use super::TestConfigDirGuard;

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
    state.add_conn(conn_id, tx, false);
    (conn_id, rx)
}

async fn recv_timeout(rx: &mut mpsc::Receiver<ServerMessage>) -> ServerMessage {
    match tokio::time::timeout(std::time::Duration::from_millis(RECV_TIMEOUT_MS), rx.recv()).await {
        Ok(Some(msg)) => msg,
        Ok(None) => panic!("channel closed"),
        Err(_) => panic!("recv timeout: expected message never arrived"),
    }
}

/// 读取下一条 controller-changed 广播（跳过途中其他帧，如 taken-over），
/// 返回 payload.id（空串 = 无人持有）。attach/takeover/断连都会触发该广播
async fn recv_controller_changed(rx: &mut mpsc::Receiver<ServerMessage>) -> String {
    loop {
        match recv_timeout(rx).await {
            ServerMessage::RpcEvent { name, payload } if name == "controller-changed" => {
                return payload["id"].as_str().unwrap_or("").to_string()
            }
            _ => continue,
        }
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
        ServerMessage::Sessions { list, .. } => {
            assert_eq!(list.len(), 2);
            assert_eq!(list[0].id, "s1");
            assert_eq!(list[0].pty_type, "claude");
            assert_eq!(list[1].id, "s2");
        }
        other => panic!("expected Sessions, got {:?}", other),
    }
}

#[tokio::test]
async fn RemoteState_SessionList_下发sessionId() {
    let state = RemoteState::new();
    state.register_pty("s10", "claude", "/a");
    state.register_pty("s11", "shell", "/b");
    // hook SessionStart 记录 sessionId（重复设置幂等）
    state.set_session_id("s10", "sess-fork-123");
    state.set_session_id("s10", "sess-fork-123");

    let (conn, mut rx) = setup_conn(&state);
    state.handle_client_message(conn, &ClientMessage::SessionsList);
    match recv_timeout(&mut rx).await {
        ServerMessage::Sessions { list, .. } => {
            // claude 会话携带 sessionId，shell 会话为 None
            assert_eq!(list[0].session_id.as_deref(), Some("sess-fork-123"));
            assert_eq!(list[1].session_id, None);
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

    // 写入超过预算的内容（含多字节中文字符；15B × 20000 = 300KB > 256KB）
    let chunk = "中文内容块".repeat(20000);
    state.push_output("p10", &format!("HEAD{}", chunk));
    state.push_output("p10", "TAIL_MARKER");

    let (conn, mut rx) = setup_conn(&state);
    state.handle_client_message(conn, &ClientMessage::SessionSubscribe { id: "p10".into() });
    match recv_timeout(&mut rx).await {
        ServerMessage::SessionSubscribed { tail, .. } => {
            assert!(tail.len() <= crate::remote_state::TAIL_BUDGET + "TAIL_MARKER".len());
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
async fn RemoteState_Tail_超预算淘汰整块并对齐转义边界() {
    // 复现：TAIL_LIMIT=16KB 时代，attach 运行中会话回放只剩一屏转义序列
    let state = RemoteState::new();
    state.register_pty("t1", "claude", "/tmp");

    // 块 A（130KB）：含 HEAD 标记
    let a = format!("HEAD-AAA\x1b[31m{}", "a".repeat(130 * 1024));
    state.push_output("t1", &a);
    // 块 B（130KB）：以残缺转义延续 "1m" 开头（模拟块边界切在转义中间），
    // 块内含完整 ESC 序列
    let b = format!("1m{}b{}\x1b[32m{}", "b".repeat(97), "b".repeat(102), "b".repeat(130 * 1024 - 200));
    state.push_output("t1", &b);
    // 块 C：结尾标记
    state.push_output("t1", "\x1b[0mTAIL_MARKER");

    let (conn, mut rx) = setup_conn(&state);
    state.handle_client_message(conn, &ClientMessage::SessionSubscribe { id: "t1".into() });
    match recv_timeout(&mut rx).await {
        ServerMessage::SessionSubscribed { tail, .. } => {
            // 总量 ~260KB > 256KB 预算：块 A 被淘汰，回放保留约 256KB
            assert!(!tail.contains("HEAD-AAA"), "oldest chunk should be evicted");
            assert!(tail.contains("TAIL_MARKER"));
            // 新头部对齐到 ESC 边界：丢弃块 B 开头的残缺转义延续 "1m"
            assert!(tail.starts_with("\x1b[32m") || tail.starts_with("\x1b[0m"),
                "snapshot head should align to ANSI escape, got: {:?}",
                &tail[..tail.len().min(24)]);
            // UTF-8 边界安全
            assert!(std::str::from_utf8(tail.as_bytes()).is_ok());
            // 256KB 预算下回放应显著大于旧 16KB 行为（保留了大段中段内容）
            assert!(tail.len() > 100 * 1024, "replay should keep ~256KB, got {}", tail.len());
        }
        other => panic!("expected SessionSubscribed, got {:?}", other),
    }
}

#[tokio::test]
async fn RemoteState_Tail_小块并入尾块内容保序() {
    // 逐键回显场景：海量小块（<64B）不积累独立块，snapshot 内容完整有序
    let state = RemoteState::new();
    state.register_pty("t4", "claude", "/tmp");
    for i in 0..5000u32 {
        state.push_output("t4", &format!("k{};", i));
    }
    state.push_output("t4", "\x1b[0mEND");

    let (conn, mut rx) = setup_conn(&state);
    state.handle_client_message(conn, &ClientMessage::SessionSubscribe { id: "t4".into() });
    match recv_timeout(&mut rx).await {
        ServerMessage::SessionSubscribed { tail, .. } => {
            assert!(tail.ends_with("\x1b[0mEND"));
            // 顺序完整：首个与最后一个小块都在
            assert!(tail.contains("k0;"));
            assert!(tail.contains("k4999;"));
            assert!(std::str::from_utf8(tail.as_bytes()).is_ok());
        }
        other => panic!("expected SessionSubscribed, got {:?}", other),
    }
}

#[tokio::test]
async fn RemoteState_Tail_单块超预算块内截断保留尾部() {
    let state = RemoteState::new();
    state.register_pty("t2", "claude", "/tmp");

    // 单块 300KB：块内截断保留尾部 ~256KB
    let big = format!("X{}", "x".repeat(300 * 1024));
    state.push_output("t2", &big);
    state.push_output("t2", "TAIL_MARKER");

    let (conn, mut rx) = setup_conn(&state);
    state.handle_client_message(conn, &ClientMessage::SessionSubscribe { id: "t2".into() });
    match recv_timeout(&mut rx).await {
        ServerMessage::SessionSubscribed { tail, .. } => {
            assert!(tail.ends_with("TAIL_MARKER"));
            assert!(tail.len() <= 256 * 1024 + "TAIL_MARKER".len() + 8,
                "single-chunk overflow should truncate in-place, got {}", tail.len());
            assert!(!tail.contains(&"X".repeat(50 * 1024)), "head should be truncated");
        }
        other => panic!("expected SessionSubscribed, got {:?}", other),
    }
}

#[tokio::test]
async fn RemoteState_Attach_快照先于增量入队() {
    // 并发守护：attach 期间对端持续 push_output，连接收到的首包必须是
    // SessionAttached（快照），不能是 PtyOutput（旧实现锁外发快照存在
    // 订阅插入后、快照入队前的乱序窗口）
    let state = RemoteState::new();
    state.register_pty("t3", "claude", "/tmp");
    state.push_output("t3", "history\n");

    let push_state = std::sync::Arc::new(state);
    let stop = std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false));
    let producer = {
        let s = push_state.clone();
        let stop = stop.clone();
        std::thread::spawn(move || {
            let mut i = 0u64;
            while !stop.load(std::sync::atomic::Ordering::Relaxed) {
                s.push_output("t3", &format!("out{}", i));
                i += 1;
            }
        })
    };

    for _ in 0..20 {
        let (conn, mut rx) = setup_conn(&push_state);
        push_state.handle_client_message(
            conn,
            &ClientMessage::SessionAttach { id: "t3".into(), cols: 80, rows: 24 },
        );
        match recv_timeout(&mut rx).await {
            ServerMessage::SessionAttached { .. } => {}
            other => {
                stop.store(true, std::sync::atomic::Ordering::Relaxed);
                producer.join().unwrap();
                panic!("first message must be SessionAttached snapshot, got {:?}", other);
            }
        }
        push_state.remove_conn(conn);
    }
    stop.store(true, std::sync::atomic::Ordering::Relaxed);
    producer.join().unwrap();
}


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
    // attach 自动持有控制权 → controller-changed 广播
    let _ = recv_controller_changed(&mut rx).await;

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
    // conn1 自动持有控制权的广播
    assert_eq!(recv_controller_changed(&mut rx1).await, conn1.to_string());
    let _ = recv_controller_changed(&mut rx2).await;

    // 设备互斥：conn2 直接 attach 不能抢占（被拒，须先 takeover）
    state.handle_client_message(
        conn2,
        &ClientMessage::SessionAttach { id: "a2".into(), cols: 100, rows: 30 },
    );
    match recv_timeout(&mut rx2).await {
        ServerMessage::SessionAttachDenied { .. } => {}
        other => panic!("expected SessionAttachDenied, got {:?}", other),
    }

    // conn2 显式接管后再 attach：conn1 的终端被剥离（takeover），attach 抢占桌面
    state.handle_client_message(conn2, &ClientMessage::ControllerTakeover);
    assert_eq!(recv_controller_changed(&mut rx1).await, conn2.to_string());
    let _ = recv_controller_changed(&mut rx2).await; // 接管确认

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
    let _ = recv_controller_changed(&mut rx).await; // attach 自动持有

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
    let _ = recv_controller_changed(&mut rx).await; // attach 自动持有

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
    let _ = recv_controller_changed(&mut rx).await; // attach 自动持有

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
    let _ = recv_controller_changed(&mut rx).await; // attach 自动持有

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
        ServerMessage::HelloOk { ver, device_name, .. } => {
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
            session_name: None,
        },
    );
    match recv_timeout(&mut rx).await {
        ServerMessage::SessionCreateFailed { error } => assert_eq!(error, "unknown project"),
        other => panic!("expected SessionCreateFailed, got {:?}", other),
    }
}

/// session.kill（用户显式关闭）先广播 remote-session-closed 给远程连接：
/// 各端联动删 Tab（"打开的标签页"不残留 stopped 条目）。广播在 kill 之前
/// （无 pty manager 时 kill 失败仅告警，不影响广播发出）。
/// emit_event 固定走全局 REMOTE_STATE（生产语义），连接须注册在全局上
#[tokio::test]
async fn RemoteState_SessionKill_广播closed给远程连接() {
    let state = &crate::remote_state::REMOTE_STATE;
    state.register_pty("pk1", "claude", "/tmp");
    let (tx, mut rx) = mpsc::channel(16);
    let conn = crate::remote_state::next_conn_id();
    state.add_conn(conn, tx, false);

    state.handle_client_message(conn, &ClientMessage::SessionKill { id: "pk1".into(), quiet: None });

    let result = match recv_timeout(&mut rx).await {
        ServerMessage::RpcEvent { name, payload } => {
            assert_eq!(name, "remote-session-closed");
            assert_eq!(payload["id"], "pk1");
            ()
        }
        other => panic!("expected RpcEvent(remote-session-closed), got {:?}", other),
    };

    state.remove_conn(conn);
    result
}

/// session.kill quiet=true（重启场景内部清理）不广播 remote-session-closed：
/// 广播会让各端 closeTab 删 Tab，重启被误判为关闭（Tab 删后 respawn 无处绑回）
#[tokio::test]
async fn RemoteState_SessionKillQuiet_不广播closed() {
    let state = &crate::remote_state::REMOTE_STATE;
    state.register_pty("pkq1", "claude", "/tmp");
    let (tx, mut rx) = mpsc::channel(16);
    let conn = crate::remote_state::next_conn_id();
    state.add_conn(conn, tx, false);

    state.handle_client_message(
        conn,
        &ClientMessage::SessionKill { id: "pkq1".into(), quiet: Some(true) },
    );

    // 观察窗口内不应收到本 pty 的 closed（全局实例上并行测试的无关广播可能
    // 先入队，跳过；无 pty manager 时 kill 仅告警无输出）。窗口 300ms：
    // 覆盖慢环境下 handler spawn 线程的调度延迟，防回归事件晚到漏报
    let deadline = std::time::Instant::now() + std::time::Duration::from_millis(300);
    loop {
        let remain = deadline.saturating_duration_since(std::time::Instant::now());
        if remain.is_zero() {
            break;
        }
        match tokio::time::timeout(remain, rx.recv()).await {
            Err(_) => break, // 窗口内无更多消息：quiet 未广播 ✓
            Ok(Some(ServerMessage::RpcEvent { name, payload })) => {
                assert!(
                    !(name == "remote-session-closed" && payload["id"] == "pkq1"),
                    "quiet kill 不应广播本会话 closed"
                );
                // 其他测试的串扰广播，跳过
            }
            Ok(other) => panic!("unexpected message: {:?}", other),
        }
    }

    state.remove_conn(conn);
}

/// session.create 在途上限：达到上限后立即拒绝（不进线程、不查白名单）
#[tokio::test]
async fn RemoteState_SessionCreate_并发上限拒绝() {
    let state = RemoteState::new();
    let (conn, mut rx) = setup_conn(&state);

    crate::remote_state::REMOTE_SPAWN_INFLIGHT.store(8, std::sync::atomic::Ordering::SeqCst);
    state.handle_client_message(
        conn,
        &ClientMessage::SessionCreate { path: "/any".into(), session_id: None, skip_permissions: None, custom_args: None, session_name: None },
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

/// SessionEnd 结束 working 但不设 turnEnded：后续 activity（如新会话的
/// 首个工具调用）可恢复 working——与桌面 useStatusMonitor 语义一致
#[test]
fn RemoteState_HookStateMachine_SessionEnd后activity恢复working() {
    let state = RemoteState::new();
    state.register_pty("p-se", "claude", "/tmp");

    state.on_hook_payload(&hook("p-se", user_prompt()));
    state.on_hook_payload(&hook(
        "p-se",
        HookEventDetail::SessionEnd(crate::hook_events::SessionEndData { reason: None }),
    ));
    assert!(!state.session_list()[0].working);

    // turnEnded 未设：activity 恢复 working（区别于 Stop 后的 recap 防护）
    state.on_hook_payload(&hook("p-se", post_tool_use()));
    assert!(state.session_list()[0].working);
}

/// on_hook_payload 返回值 = hook-status 事件载荷：与 SessionStatus 广播同源同值；
/// 会话未注册返回 None（hook_server 据此跳过事件下发）
#[tokio::test]
async fn RemoteState_HookStatus_返回值与SessionStatus广播一致() {
    let state = RemoteState::new();
    state.register_pty("p-hs", "claude", "/tmp");
    let (_conn, mut rx) = setup_conn(&state);

    // 未注册的 pty：None
    let none = state.on_hook_payload(&hook("unknown", user_prompt()));
    assert!(none.is_none());

    // userPromptSubmit：状态变化（false,true → true,false）
    let st = state.on_hook_payload(&hook("p-hs", user_prompt())).unwrap();
    assert_eq!(st.pty_id, "p-hs");
    assert!(st.working);
    assert!(!st.pending);
    // SessionStatus 广播与返回值同值
    match recv_timeout(&mut rx).await {
        ServerMessage::SessionStatus { id, working, pending } => {
            assert_eq!(id, "p-hs");
            assert_eq!(working, st.working);
            assert_eq!(pending, st.pending);
        }
        other => panic!("expected SessionStatus, got {:?}", other),
    }

    // 回合内 activity（working 已 true、pending 已 false）：状态无变化，
    // 无 SessionStatus 广播，但 hook-status 返回值仍给出当前状态（命名 trigger 不丢）
    let st2 = state.on_hook_payload(&hook("p-hs", post_tool_use())).unwrap();
    assert!(st2.working && !st2.pending);
    assert!(
        tokio::time::timeout(std::time::Duration::from_millis(200), rx.recv())
            .await
            .is_err(),
        "状态未变化不应广播 SessionStatus"
    );
}

/// rename_session：双端同步的广播中转——经 rpc 分发后以 session-renamed
/// 事件（payload {id, data} 与桌面 Tauri 事件同构）广播给全部远程连接
#[tokio::test]
async fn Rpc_RenameSession_广播sessionRenamed事件() {
    // emit_event 固定走全局单例（生产路径；测试构建无 EVENT_HOOK，仅广播可验）
    let (_conn, mut rx) = setup_conn(&crate::remote_state::REMOTE_STATE);

    let mut args = serde_json::Map::new();
    args.insert("ptyId".to_string(), serde_json::json!("p-rn1"));
    args.insert("name".to_string(), serde_json::json!("新名字"));
    crate::rpc_dispatch::dispatch("rename_session", &args).await.unwrap();

    match recv_timeout(&mut rx).await {
        ServerMessage::RpcEvent { name, payload } => {
            assert_eq!(name, "session-renamed");
            assert_eq!(payload["id"], "p-rn1");
            assert_eq!(payload["data"], "新名字");
        }
        other => panic!("expected RpcEvent, got {:?}", other),
    }
}

/// state-changed 统一广播：写命令落盘后广播 {id: section, data: revision}，
/// revision 随每次广播单调递增（前端过滤旧事件的依据）
#[tokio::test]
async fn RemoteState_StateChanged_广播半链与revision单调递增() {
    let state = RemoteState::new();
    let (_conn, mut rx) = setup_conn(&state);

    state.emit_state_changed(crate::rpc_dispatch::StateSection::Skills);
    let rev1 = match recv_timeout(&mut rx).await {
        ServerMessage::RpcEvent { name, payload } => {
            assert_eq!(name, "state-changed");
            assert_eq!(payload["id"], "skills");
            payload["data"].as_str().unwrap().parse::<u64>().unwrap()
        }
        other => panic!("expected RpcEvent, got {:?}", other),
    };
    assert_eq!(rev1, state.revision());

    state.emit_state_changed(crate::rpc_dispatch::StateSection::Providers);
    let rev2 = match recv_timeout(&mut rx).await {
        ServerMessage::RpcEvent { name, payload } => {
            assert_eq!(name, "state-changed");
            assert_eq!(payload["id"], "providers");
            payload["data"].as_str().unwrap().parse::<u64>().unwrap()
        }
        other => panic!("expected RpcEvent, got {:?}", other),
    };
    assert!(rev2 > rev1, "revision 必须单调递增");
    assert_eq!(state.revision(), rev2);
}

/// rename 桌面半链：EVENT_HOOK（lib.rs 设置为 Tauri app.emit）在 rename_session
/// 分发后必须收到 ("session-renamed", ptyId, name)——与广播半链测试共同覆盖双发。
/// HOOK_TEST_LOCK：与其他整体 set/clear 全局 hook 的测试串行（防并行覆盖竞态）
#[tokio::test]
async fn Rpc_RenameSession_桌面事件半链emitViaHook() {
    use std::sync::{Arc, Mutex as StdMutex};
    let _hook_lock = crate::tests::HOOK_TEST_LOCK.lock();
    let received: Arc<StdMutex<Vec<(String, String, String)>>> = Arc::new(StdMutex::new(Vec::new()));
    let sink = received.clone();
    crate::remote_state::set_event_hook(Box::new(move |name, id, data| {
        sink.lock().unwrap().push((name.to_string(), id.to_string(), data.to_string()));
    }));

    let mut args = serde_json::Map::new();
    args.insert("ptyId".to_string(), serde_json::json!("p-rn2"));
    args.insert("name".to_string(), serde_json::json!("桌面名"));
    crate::rpc_dispatch::dispatch("rename_session", &args).await.unwrap();

    let got = received.lock().unwrap().clone();
    assert!(
        got.iter().any(|(n, i, d)| n == "session-renamed" && i == "p-rn2" && d == "桌面名"),
        "EVENT_HOOK 未收到 session-renamed，实际: {:?}", got
    );

    // 清理 hook，避免影响其他测试
    crate::remote_state::clear_event_hook();
}

// ==================== 全局控制权（设备互斥）规则矩阵 ====================

/// attach 在无人持有时自动获得控制权并广播 controller-changed
#[tokio::test]
async fn Controller_Attach无人时自动持有并广播() {
    let state = RemoteState::new();
    state.register_pty("pc1", "claude", "/tmp");
    let (conn_a, mut rx_a) = setup_conn(&state);

    assert_eq!(state.controller(), None);
    state.handle_client_message(
        conn_a,
        &ClientMessage::SessionAttach { id: "pc1".into(), cols: 80, rows: 24 },
    );
    assert_eq!(state.controller(), Some(crate::remote_state::ControllerHolder::Remote(conn_a)));
    let _ = recv_timeout(&mut rx_a).await; // SessionAttached 先入队
    assert_eq!(recv_controller_changed(&mut rx_a).await, conn_a.to_string());
}

/// attach 不能抢占：他人持有时被拒（SessionAttachDenied），不转移控制权
#[tokio::test]
async fn Controller_Attach他人持有被拒() {
    let state = RemoteState::new();
    state.register_pty("pc2", "claude", "/tmp");
    let (conn_a, mut rx_a) = setup_conn(&state);
    let (conn_b, mut rx_b) = setup_conn(&state);

    // A 先持有（B 旁观：收到 taken-over + controller-changed）
    state.handle_client_message(
        conn_a,
        &ClientMessage::SessionAttach { id: "pc2".into(), cols: 80, rows: 24 },
    );
    let _ = recv_timeout(&mut rx_a).await; // SessionAttached
    let _ = recv_controller_changed(&mut rx_a).await;
    let _ = recv_controller_changed(&mut rx_b).await; // B 消化 A 持有的广播

    // B attach 被拒
    state.handle_client_message(
        conn_b,
        &ClientMessage::SessionAttach { id: "pc2".into(), cols: 80, rows: 24 },
    );
    match recv_timeout(&mut rx_b).await {
        ServerMessage::SessionAttachDenied { id, reason } => {
            assert_eq!(id, "pc2");
            assert!(reason.contains("controller"));
        }
        other => panic!("expected SessionAttachDenied, got {:?}", other),
    }
    assert_eq!(state.controller(), Some(crate::remote_state::ControllerHolder::Remote(conn_a)), "控制权不应被 attach 抢走");
}

/// takeover：转移控制权 + 剥离旧持有者终端归属（active 归还桌面）+ 广播
#[tokio::test]
async fn Controller_Takeover转移并剥离终端归属() {
    let state = RemoteState::new();
    state.register_pty("pc3", "claude", "/tmp");
    let (conn_a, mut rx_a) = setup_conn(&state);
    let (conn_b, mut rx_b) = setup_conn(&state);

    // A attach 持有终端与控制权（B 旁观收 taken-over + controller-changed）
    state.handle_client_message(
        conn_a,
        &ClientMessage::SessionAttach { id: "pc3".into(), cols: 80, rows: 24 },
    );
    let _ = recv_timeout(&mut rx_a).await; // SessionAttached
    let _ = recv_controller_changed(&mut rx_a).await;
    let _ = recv_controller_changed(&mut rx_b).await;
    assert_eq!(state.controller(), Some(crate::remote_state::ControllerHolder::Remote(conn_a)));
    assert!(state.is_remote_active("pc3"));

    // B takeover：控制权转移 + 终端归还桌面
    state.handle_client_message(conn_b, &ClientMessage::ControllerTakeover);
    assert_eq!(state.controller(), Some(crate::remote_state::ControllerHolder::Remote(conn_b)));
    assert!(!state.is_remote_active("pc3"), "旧持有者终端应被剥离归还桌面");

    // A 收到 controller-changed（不再是自己 → 显示全局遮罩）
    assert_eq!(recv_controller_changed(&mut rx_a).await, conn_b.to_string());
    // B 也收到（接管确认 → 解锁 + 全量刷新）
    assert_eq!(recv_controller_changed(&mut rx_b).await, conn_b.to_string());
}

/// 持有者断连：控制权释放（广播空 id），终端归还桌面
#[tokio::test]
async fn Controller_持有者断连释放() {
    let state = RemoteState::new();
    state.register_pty("pc4", "claude", "/tmp");
    let (conn_a, mut rx_a) = setup_conn(&state);
    let (conn_b, mut rx_b) = setup_conn(&state);

    state.handle_client_message(
        conn_a,
        &ClientMessage::SessionAttach { id: "pc4".into(), cols: 80, rows: 24 },
    );
    let _ = recv_timeout(&mut rx_a).await; // SessionAttached
    let _ = recv_controller_changed(&mut rx_a).await;
    let _ = recv_controller_changed(&mut rx_b).await; // B 消化 A 持有的广播

    state.remove_conn(conn_a);
    assert_eq!(state.controller(), None, "断连后控制权应释放");
    assert!(!state.is_remote_active("pc4"));

    // B 收到释放广播（空 id）
    match recv_timeout(&mut rx_b).await {
        ServerMessage::RpcEvent { name, payload } => {
            assert_eq!(name, "controller-changed");
            assert_eq!(payload["id"], "");
        }
        other => panic!("expected controller-changed release, got {:?}", other),
    }
}

/// takeover 后旧持有者的终端输入被拒（controller 不匹配，服务端兜底）
#[tokio::test]
async fn Controller_旧持有者输入被拒() {
    let state = RemoteState::new();
    state.register_pty("pc5", "claude", "/tmp");
    let (conn_a, mut rx_a) = setup_conn(&state);
    let (conn_b, _rx_b) = setup_conn(&state);

    // A attach（终端 + 控制权都是 A）
    state.handle_client_message(
        conn_a,
        &ClientMessage::SessionAttach { id: "pc5".into(), cols: 80, rows: 24 },
    );
    let _ = recv_timeout(&mut rx_a).await;
    let _ = recv_timeout(&mut rx_a).await;

    // B takeover（A 终端被剥离）；此后 A 的输入应被拒（无 panic 即可——
    // manager 未初始化时不写入，条件判断不通过走 debug log 分支）
    state.handle_client_message(conn_b, &ClientMessage::ControllerTakeover);
    state.handle_client_message(
        conn_a,
        &ClientMessage::PtyInput { id: "pc5".into(), data: "echo hi".into() },
    );
    assert_eq!(state.controller(), Some(crate::remote_state::ControllerHolder::Remote(conn_b)));
}

/// hello.ok 携带 conn_id / controller / revision（前端遮罩与基线判定依据）
#[tokio::test]
async fn Controller_HelloOk携带控制权快照() {
    let state = RemoteState::new();
    let (conn_a, mut rx_a) = setup_conn(&state);

    state.handle_client_message(conn_a, &ClientMessage::Hello {
        ver: 1,
        client: "web-test".into(),
        token: "t".into(),
    });
    match recv_timeout(&mut rx_a).await {
        ServerMessage::HelloOk { conn_id, controller, revision, .. } => {
            assert_eq!(conn_id, conn_a);
            assert_eq!(controller, None);
            assert_eq!(revision, 0);
        }
        other => panic!("expected HelloOk, got {:?}", other),
    }
}

// ==================== Hello 分支中继 token v2 校验 ====================

/// token v2 测试身份（32 hex）
const V2_DEVICE_ID: &str = "a1b2c3d4e5f6a7b8c9d0e1f2a3b4c5d6";
const V2_DEVICE_KEY: &str = "00112233445566778899aabbccddeeff00112233445566778899aabbccddeeff";
const V2_SECRET: &str = "0123456789abcdef0123456789abcdef";

/// 注册指定类型的测试连接（via_relay 区分中继/局域网）
fn setup_conn_via(state: &RemoteState, via_relay: bool) -> (u64, mpsc::Receiver<ServerMessage>) {
    let (tx, rx) = mpsc::channel(64);
    let conn_id = crate::remote_state::next_conn_id();
    state.add_conn(conn_id, tx, via_relay);
    (conn_id, rx)
}

/// 隔离 config 并预置设备身份与已配对 secret（RAII Drop 还原）
fn setup_v2_config() {
    crate::remote_server::save_relay_config(None, Some((V2_DEVICE_ID, V2_DEVICE_KEY))).unwrap();
    crate::remote_server::save_paired_secrets(&[V2_SECRET.to_string()]).unwrap();
}

/// 中继连接 + 伪造/未知 secret → hello.error（relay 只路由不校验，桌面是校验闭环）
#[tokio::test]
async fn RemoteState_Hello_ViaRelay_未知secret被拒() {
    let _cfg_lock = super::CONFIG_DIR_TEST_LOCK.lock();
    let _cfg = TestConfigDirGuard::new("hello-relay-reject");
    setup_v2_config();

    let state = RemoteState::new();
    let (conn, mut rx) = setup_conn_via(&state, true);
    // 前缀正确但 secret 不在已配对列表（伪造场景）
    state.handle_client_message(conn, &ClientMessage::Hello {
        ver: 1,
        client: "web".into(),
        token: format!("{}.ffffffffffffffffffffffffffffffff", V2_DEVICE_ID),
    });
    match recv_timeout(&mut rx).await {
        ServerMessage::HelloError { error } => assert_eq!(error, "invalid token"),
        other => panic!("expected HelloError, got {:?}", other),
    }
}

/// 中继连接 + 已配对 secret → hello.ok（正常路径）
#[tokio::test]
async fn RemoteState_Hello_ViaRelay_已配对secret通过() {
    let _cfg_lock = super::CONFIG_DIR_TEST_LOCK.lock();
    let _cfg = TestConfigDirGuard::new("hello-relay-accept");
    setup_v2_config();

    let state = RemoteState::new();
    let (conn, mut rx) = setup_conn_via(&state, true);
    state.handle_client_message(conn, &ClientMessage::Hello {
        ver: 1,
        client: "web".into(),
        token: format!("{}.{}", V2_DEVICE_ID, V2_SECRET),
    });
    match recv_timeout(&mut rx).await {
        ServerMessage::HelloOk { .. } => {}
        other => panic!("expected HelloOk, got {:?}", other),
    }
}

/// 局域网连接不触发 v2 校验（token 已在连接层完成等值校验，任意值直接通过）
#[tokio::test]
async fn RemoteState_Hello_Lan_不走v2校验() {
    let state = RemoteState::new();
    let (conn, mut rx) = setup_conn_via(&state, false);
    state.handle_client_message(conn, &ClientMessage::Hello {
        ver: 1,
        client: "web".into(),
        token: "any-lan-token".into(),
    });
    match recv_timeout(&mut rx).await {
        ServerMessage::HelloOk { .. } => {}
        other => panic!("expected HelloOk, got {:?}", other),
    }
}

/// 认证门闩：中继连接 Hello 校验失败（假 secret）后，同一连接继续发
/// controller.takeover / 任意消息不得生效（恶意客户端可无视 hello.error
/// 继续发消息——校验必须伴随「失败断连 + 未认证忽略」才有真实拦截力）
#[tokio::test]
async fn RemoteState_Hello_ViaRelay_校验失败后拒绝后续消息() {
    let _cfg_lock = super::CONFIG_DIR_TEST_LOCK.lock();
    let _cfg = TestConfigDirGuard::new("hello-relay-gate");
    setup_v2_config();

    let state = RemoteState::new();
    let (conn, mut rx) = setup_conn_via(&state, true);
    // 假 secret 的 v2 token → HelloError
    state.handle_client_message(conn, &ClientMessage::Hello {
        ver: 1,
        client: "web".into(),
        token: format!("{}.ffffffffffffffffffffffffffffffff", V2_DEVICE_ID),
    });
    match recv_timeout(&mut rx).await {
        ServerMessage::HelloError { .. } => {}
        other => panic!("expected HelloError, got {:?}", other),
    }

    // 恶意客户端无视错误继续接管：不得获得控制权
    state.handle_client_message(conn, &ClientMessage::ControllerTakeover);
    assert_ne!(
        state.controller(),
        Some(crate::remote_state::ControllerHolder::Remote(conn)),
        "未认证连接不得通过后续消息获得控制权"
    );
    // 后续读类消息同样被忽略（不产生任何回复）
    state.handle_client_message(conn, &ClientMessage::SessionsList);
    let res = tokio::time::timeout(std::time::Duration::from_millis(150), rx.recv()).await;
    assert!(res.is_err() || matches!(res, Ok(None)), "未认证连接的消息不应有任何应答");
}

/// 复现手机端 rpc 超时：get_app_config 全链路（handle_rpc 线程 → dispatch →
///RpcResult 回连接）。页面端该命令 15s 超时，此处验证服务端是否发出应答
#[tokio::test]
async fn Rpc_GetAppConfig_全链路应答() {
    let (_conn, mut rx) = setup_conn(&crate::remote_state::REMOTE_STATE);
    crate::rpc_dispatch::handle_rpc(
        _conn,
        4242,
        "get_app_config",
        &serde_json::json!({}),
    );
    let msg = loop {
        match recv_timeout(&mut rx).await {
            m @ ServerMessage::RpcResult { .. } => break m,
            // 全局实例上并行测试的广播可能先入队，跳过
            ServerMessage::RpcEvent { .. } => continue,
            other => panic!("expected RpcResult, got {:?}", other),
        }
    };
    if let ServerMessage::RpcResult { id, ok, .. } = msg {
        assert_eq!(id, 4242);
        assert!(ok, "get_app_config 应成功");
    }
}

/// update_app_config 写命令全链路 + 成功后自动持有 controller（复现接管风暴源头）。
/// 落盘必须隔离到临时目录：走真实命令链路意味着真实写 ~/.cc-box/config.json
/// （曾把各开发者的 theme 污染成 dark）
#[tokio::test]
async fn Rpc_UpdateAppConfig_写命令自动持有() {
    // GUI_CONFIG_DIR_OVERRIDE 全局互斥：与其他配置隔离用户并行会互相清除
    let _cfg_lock = super::CONFIG_DIR_TEST_LOCK.lock();
    // 真实 ~/.cc-box/config.json 快照：测试结束前后必须字节一致（污染守卫）
    let real_config_path = dirs::home_dir().map(|h| h.join(".cc-box").join("config.json"));
    let real_before = real_config_path
        .as_ref()
        .and_then(|p| std::fs::read_to_string(p).ok());

    // GUI 配置目录指到独立临时目录（RAII Drop 还原覆盖并清理）
    let _cfg_guard = TestConfigDirGuard::new("update-app-config");

    // dispatch 走全局 REMOTE_STATE 的 broadcast；持有逻辑在 handle_rpc 走全局，
    // 因此在全局实例上验证（先记录原 controller 以便还原）
    let (conn, mut rx) = setup_conn(&crate::remote_state::REMOTE_STATE);
    let before = crate::remote_state::REMOTE_STATE.controller();

    crate::rpc_dispatch::handle_rpc(
        conn,
        4243,
        "update_app_config",
        &serde_json::json!({ "updates": { "theme": "dark" } }),
    );
    // 广播（state-changed / controller-changed）先于 RpcResult 入队，循环收到为止
    let result = loop {
        match recv_timeout(&mut rx).await {
            ServerMessage::RpcResult { id, ok, error, .. } => {
                assert_eq!(id, 4243);
                assert!(ok, "update_app_config 应成功: {:?}", error);
                break;
            }
            ServerMessage::RpcEvent { .. } => continue,
            other => panic!("expected RpcResult, got {:?}", other),
        }
    };
    let _ = result;
    // 成功后无人持有时自动成为 controller
    if before.is_none() {
        assert_eq!(crate::remote_state::REMOTE_STATE.controller(), Some(crate::remote_state::ControllerHolder::Remote(conn)));
    }
    // 命令确实落盘——但落在隔离目录（而非真实 ~/.cc-box）
    let isolated = std::fs::read_to_string(_cfg_guard.0.join("config.json"))
        .expect("隔离 config.json 应已落盘");
    let isolated_json: serde_json::Value = serde_json::from_str(&isolated).unwrap();
    assert_eq!(isolated_json["theme"], "dark", "写命令应真实执行（写到隔离目录）");
    // 真实配置未被污染（修复前此处失败：theme 被写成 dark）
    let real_after = real_config_path
        .as_ref()
        .and_then(|p| std::fs::read_to_string(p).ok());
    assert_eq!(
        real_before, real_after,
        "rpc 写命令测试不得污染真实 ~/.cc-box/config.json"
    );
    // 还原：移除测试连接（顺带释放其可能持有的 controller），避免污染其他测试
    crate::remote_state::REMOTE_STATE.remove_conn(conn);
}

/// 把 GUI 配置目录临时指到独立临时目录（RAII Drop 还原覆盖并清理）。
/// 全局覆盖与并行测试：当前仅 update_app_config 全链路测试需要写隔离；
/// 并行只读测试（Rpc_GetAppConfig）可能短暂读到隔离目录的默认配置，无害
/// 桌面显式接管（controller_takeover command）：桌面获得控制权 + 剥离远程终端
/// + 广播（hello.ok 视角 controller=Some(0)=桌面；远程写操作被拒）
#[tokio::test]
async fn Controller_桌面接管获得控制权并拒绝远程写() {
    let state = RemoteState::new();
    state.register_pty("pd1", "claude", "/tmp");
    let (conn_a, mut rx_a) = setup_conn(&state);

    // 手机 attach 持有
    state.handle_client_message(
        conn_a,
        &ClientMessage::SessionAttach { id: "pd1".into(), cols: 80, rows: 24 },
    );
    let _ = recv_timeout(&mut rx_a).await; // attached
    let _ = recv_controller_changed(&mut rx_a).await;
    assert!(state.is_remote_active("pd1"));

    // 桌面接管：控制权归桌面 + 远程终端剥离
    state.desktop_takeover();
    assert_eq!(state.controller(), Some(crate::remote_state::ControllerHolder::Desktop));
    assert!(!state.is_remote_active("pd1"));
    // 广播 id 标签为 "desktop"
    assert_eq!(recv_controller_changed(&mut rx_a).await, "desktop");

    // 桌面持有期间远程写操作被拒
    assert!(state.controller_gate(conn_a).is_err());

    // 幂等：再次桌面接管不重复广播（无新消息到达）
    state.desktop_takeover();
    assert!(
        tokio::time::timeout(std::time::Duration::from_millis(150), rx_a.recv())
            .await
            .is_err(),
        "重复接管不应再广播"
    );
}

/// 桌面 claim（终端接管回来）同时获得全局控制权
#[tokio::test]
async fn Controller_桌面claim同时获得控制权() {
    let state = RemoteState::new();
    state.register_pty("pd2", "claude", "/tmp");
    let (conn_a, mut rx_a) = setup_conn(&state);

    state.handle_client_message(
        conn_a,
        &ClientMessage::SessionAttach { id: "pd2".into(), cols: 80, rows: 24 },
    );
    let _ = recv_timeout(&mut rx_a).await; // attached
    let _ = recv_controller_changed(&mut rx_a).await;

    state.claim("pd2");
    assert_eq!(state.controller(), Some(crate::remote_state::ControllerHolder::Desktop));
    // claim 的 reclaimed 走全局 emit_event（独立实例收不到），controller-changed 走 self
    assert_eq!(recv_controller_changed(&mut rx_a).await, "desktop");
}

/// revision 取号与广播入队必须同锁窗口（保序）：跨线程并发 emit_state_changed
/// 与 controller takeover 时，若取号（bump_revision）与入队（broadcast_event）
/// 分属两个锁窗口，后取号的事件可能先入队——浏览器端 filterStaleEvent 按
/// revision 单调过滤，低版本后到即被当旧事件丢弃（遮罩/刷新状态永久丢失）。
/// 回归：两线程交错广播，同一连接到达序的 revision 必须严格递增
#[test]
fn Controller_并发广播到达序revision严格递增() {
    let state = std::sync::Arc::new(RemoteState::new());
    let (tx, mut rx) = mpsc::channel(8192);
    // 只注册一个接收连接（每帧广播对每个连接各入队一份，双连接会成对到达）；
    // takeover 在两连接 id 间交替（transfer 无幂等检查，每次都触发广播），
    // conn_b 不注册 channel 也能发消息——广播仍只入队 conn_a 一份
    let conn_a = crate::remote_state::next_conn_id();
    let conn_b = crate::remote_state::next_conn_id();
    state.add_conn(conn_a, tx, false);

    const N: usize = 2000;
    // conn_b 须注册连接（认证门闩会忽略未注册连接的消息）
    let (tx_b, _rx_b) = mpsc::channel(8);
    state.add_conn(conn_b, tx_b, false);
    let s1 = state.clone();
    let writer = std::thread::spawn(move || {
        for _ in 0..N {
            s1.emit_state_changed(crate::rpc_dispatch::StateSection::Config);
        }
    });
    let s2 = state.clone();
    let taker = std::thread::spawn(move || {
        for i in 0..N {
            let who = if i % 2 == 0 { conn_a } else { conn_b };
            s2.handle_client_message(who, &ClientMessage::ControllerTakeover);
        }
    });

    writer.join().unwrap();
    taker.join().unwrap();

    let mut last: u64 = 0;
    let mut count = 0;
    while let Ok(msg) = rx.try_recv() {
        if let ServerMessage::RpcEvent { name, payload } = msg {
            if name == "state-changed" || name == "controller-changed" {
                let rev: u64 = payload["data"].as_str().and_then(|s| s.parse().ok()).unwrap_or(0);
                assert!(
                    rev > last,
                    "到达序 revision 回退：{}({}) 后于 {}",
                    rev,
                    name,
                    last
                );
                last = rev;
                count += 1;
            }
        }
    }
    // 2N 次广播、容量充足：丢帧容忍，但不应大面积缺失
    assert!(count > N, "广播帧过少：{}", count);
}

/// attach 打开过即已读：pending（等待关注）会话被 attach 后服务端清 false，
/// 其他连接收到 session-viewed 广播（发起端排除，本地 setActiveTab 已清），
/// 发起端自身不收到。防"手机看过又切走 → sessions.list 把金黄冲回"回归。
/// 负向分支：只读订阅（subscribe）与他人持权时 attach 被拒均不清 pending
#[tokio::test]
async fn RemoteState_Attach_清除pending并广播已读() {
    let _hook_lock = crate::tests::HOOK_TEST_LOCK.lock();
    use std::sync::{Arc, Mutex as StdMutex};
    let received: Arc<StdMutex<Vec<(String, String, String)>>> = Arc::new(StdMutex::new(Vec::new()));
    let sink = received.clone();
    crate::remote_state::set_event_hook(Box::new(move |name, id, data| {
        sink.lock().unwrap().push((name.to_string(), id.to_string(), data.to_string()));
    }));
    // 断言失败也清 hook（RAII），防泄漏串扰后续测试
    struct HookGuard;
    impl Drop for HookGuard {
        fn drop(&mut self) {
            crate::remote_state::clear_event_hook();
        }
    }
    let _guard = HookGuard;

    let state = RemoteState::new();
    state.register_pty("t-vw", "claude", "/tmp");
    // 状态机先开回合（working），任务完成通知 → pending=true
    state.on_hook_payload(&hook("t-vw", user_prompt()));
    state.on_hook_payload(&hook(
        "t-vw",
        HookEventDetail::Notification(NotificationData {
            notification_type: Some("idle_prompt".into()),
            message: None,
            title: None,
        }),
    ));
    assert!(
        state.session_list().iter().any(|s| s.id == "t-vw" && s.pending),
        "前置：attach 前 pending 应为 true"
    );

    let (conn1, mut rx1) = setup_conn(&state);
    let (_conn2, mut rx2) = setup_conn(&state);
    state.handle_client_message(
        conn1,
        &ClientMessage::SessionAttach { id: "t-vw".into(), cols: 80, rows: 24 },
    );

    // 服务端权威值已清（sessions.list 刷新不再冲回金黄）
    assert!(
        state.session_list().iter().any(|s| s.id == "t-vw" && !s.pending),
        "attach 后服务端 pending 应清 false"
    );

    // 其他端收到 session-viewed 广播（可能伴随 controller-changed，循环找目标）
    let mut viewed = false;
    for _ in 0..4 {
        match recv_timeout(&mut rx2).await {
            ServerMessage::RpcEvent { name, payload } if name == "session-viewed" => {
                assert_eq!(payload["id"].as_str(), Some("t-vw"));
                viewed = true;
                break;
            }
            _ => continue,
        }
    }
    assert!(viewed, "其他连接应收到 session-viewed 广播");

    // 发起端被排除：快照之后不应再收到 session-viewed
    if let ServerMessage::SessionAttached { .. } = recv_timeout(&mut rx1).await {
    } else {
        panic!("发起端首包应为 SessionAttached");
    }
    while let Ok(msg) = rx1.try_recv() {
        assert!(
            !matches!(&msg, ServerMessage::RpcEvent { name, .. } if name == "session-viewed"),
            "发起端不应收到 session-viewed（本地已清）"
        );
    }

    // 桌面半链：EVENT_HOOK 收到 (session-viewed, ptyId, "")
    assert!(
        received
            .lock()
            .unwrap()
            .iter()
            .any(|(n, id, _)| n == "session-viewed" && id == "t-vw"),
        "桌面 EVENT_HOOK 应收到 session-viewed"
    );

    // ---- 负向分支：重新置 pending 后验证两种"非打开"路径不清 ----
    // 会话已归还桌面（attach 者断连/主动归还后 active=Desktop；此处直接重新置
    // pending 验证状态语义即可）
    state.on_hook_payload(&hook("t-vw", user_prompt()));
    state.on_hook_payload(&hook(
        "t-vw",
        HookEventDetail::Notification(NotificationData {
            notification_type: Some("idle_prompt".into()),
            message: None,
            title: None,
        }),
    ));
    assert!(state.session_list().iter().any(|s| s.id == "t-vw" && s.pending));

    // ① 只读订阅（subscribe）不是"打开"：pending 保持
    state.handle_client_message(_conn2, &ClientMessage::SessionSubscribe { id: "t-vw".into() });
    assert!(
        state.session_list().iter().any(|s| s.id == "t-vw" && s.pending),
        "只读订阅不应清除 pending"
    );
    state.handle_client_message(_conn2, &ClientMessage::SessionUnsubscribed { id: "t-vw".into() });

    // ② 他人持有控制权时 attach 被拒：pending 保持
    state.handle_client_message(conn1, &ClientMessage::ControllerTakeover);
    state.handle_client_message(
        _conn2,
        &ClientMessage::SessionAttach { id: "t-vw".into(), cols: 80, rows: 24 },
    );
    assert!(
        state.session_list().iter().any(|s| s.id == "t-vw" && s.pending),
        "attach 被拒（他人持权）不应清除 pending"
    );
}
