//! CC-Box 远程连接协议正本（唯一事实来源）
//!
//! 应用层消息定义（手机 ↔ 桌面），局域网直连与中继模式共用。
//! JSON 固定样例在 `samples/`，由本 crate 测试与前端 Vitest 契约测试共同执行，
//! 保证两端序列化不漂移。修改协议时同步四处：
//! 共享 crate 定义 → protocol.ts → JSON 样例与契约测试 → remote/docs/protocol.md。

use serde::{Deserialize, Serialize};

/// 协议版本
pub const PROTOCOL_VERSION: u32 = 1;

/// 手机 → 桌面消息
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all_fields = "camelCase")]
pub enum ClientMessage {
    /// 连接认证（必须是首条消息）
    #[serde(rename = "hello")]
    Hello {
        ver: u32,
        client: String,
        token: String,
    },
    /// 请求会话列表
    #[serde(rename = "sessions.list")]
    SessionsList,
    /// 请求项目列表（手机端新建会话用）
    #[serde(rename = "projects.list")]
    ProjectsList,
    /// 在指定项目新建 Claude 会话；带 sessionId 时为恢复历史会话（--resume）；
    /// skipPermissions/customArgs 与桌面新建选项一致（拼 CLI 参数）；
    /// sessionName 为恢复会话的显示名（历史列表提供，桌面建 Tab 用，防编码名）；
    /// instanceId 为多实例路由：设备级 Gateway 按此转发到成员实例执行，
    /// 旧手机端不带 = 网关本机执行（serde default 兼容）
    #[serde(rename = "session.create")]
    SessionCreate {
        path: String,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        session_id: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        skip_permissions: Option<bool>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        custom_args: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        session_name: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        instance_id: Option<String>,
    },
    /// 关闭（杀掉）指定会话；quiet=true 为重启场景的内部清理——不广播
    /// remote-session-closed（各端保留 Tab 等 respawn，pty-exit 只置 stopped），
    /// 默认显式关闭（先广播再 kill，各端联动删 Tab）
    #[serde(rename = "session.kill")]
    SessionKill {
        id: String,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        quiet: Option<bool>,
    },
    /// 查询指定项目的历史会话（恢复用）
    #[serde(rename = "sessions.history")]
    SessionsHistory { path: String },
    /// 订阅某会话的输出流（只读，不接管）
    #[serde(rename = "session.subscribe")]
    SessionSubscribe { id: String },
    /// 取消订阅
    #[serde(rename = "session.unsubscribe")]
    SessionUnsubscribed { id: String },
    /// 接管会话（独占：抢占桌面或其他手机连接），附手机端终端尺寸
    #[serde(rename = "session.attach")]
    SessionAttach { id: String, cols: u16, rows: u16 },
    /// 主动归还会话
    #[serde(rename = "session.detach")]
    SessionDetach { id: String },
    /// 输入注入（仅 active 端有效）
    #[serde(rename = "pty.input")]
    PtyInput { id: String, data: String },
    /// 调整大小（仅 active 端有效）
    #[serde(rename = "pty.resize")]
    PtyResize { id: String, cols: u16, rows: u16 },
    #[serde(rename = "ping")]
    Ping,
    /// 通用 RPC：浏览器端调用 Tauri command（白名单见 src-tauri rpc_dispatch）
    #[serde(rename = "rpc.call")]
    RpcCall {
        id: u64,
        cmd: String,
        args: serde_json::Value,
    },
    /// 【deprecated】全局控制权已 per-session 化：attach/claim 即会话级抢占。
    /// 旧版手机缓存 UI 仍可能发送本消息——服务端 no-op 静默忽略（保留 variant
    /// 是为让旧帧可反序列化，删掉会导致旧端整帧解析失败）
    #[serde(rename = "controller.takeover")]
    ControllerTakeover,
}

/// 桌面 → 手机消息
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all_fields = "camelCase")]
pub enum ServerMessage {
    #[serde(rename = "hello.ok")]
    HelloOk {
        ver: u32,
        device_name: String,
        /// 本连接唯一标识（前端判定会话遮罩归属的依据之一）
        #[serde(default)]
        conn_id: u64,
        /// 【deprecated】全局控制权已 per-session 化，服务端恒发 null（字段保留
        /// 兼容旧端缓存 UI，勿再消费）；serde default 兼容旧桌面端不下发该字段
        #[serde(default, skip_serializing_if = "Option::is_none")]
        controller: Option<u64>,
        /// 服务端状态版本基线（接管/重连后全量刷新对齐；防旧事件覆盖）
        #[serde(default)]
        revision: u64,
        /// 票据连接：本连接只能拉取该文件的下载分块（受限会话）。
        /// None=普通连接；旧桌面不下发（serde default）
        #[serde(default, skip_serializing_if = "Option::is_none")]
        download_scope: Option<DownloadScope>,
    },
    #[serde(rename = "hello.error")]
    HelloError { error: String },
    #[serde(rename = "sessions")]
    Sessions {
        list: Vec<RemoteSession>,
        /// 状态版本快照（全量刷新基线；旧桌面端缺省 0）
        #[serde(default)]
        revision: u64,
    },
    #[serde(rename = "projects")]
    Projects { list: Vec<RemoteProject> },
    /// 新会话创建成功
    #[serde(rename = "session.created")]
    SessionCreated { id: String, path: String },
    /// 新会话创建失败
    #[serde(rename = "session.create.failed")]
    SessionCreateFailed { error: String },
    /// 项目历史会话列表（恢复用）
    #[serde(rename = "history.sessions")]
    HistorySessions { list: Vec<RemoteHistorySession> },
    /// 订阅成功，附最近输出 tail
    #[serde(rename = "session.subscribed")]
    SessionSubscribed {
        id: String,
        tail: String,
        /// tail 末尾对应的输出序号（客户端据此校准后续 PtyOutput；旧桌面端缺省 0）
        #[serde(default, skip_serializing_if = "is_zero_u64")]
        tail_seq: u64,
    },
    #[serde(rename = "session.unsubscribed")]
    SessionUnsubscribed { id: String },
    /// 接管成功，附快照 tail
    #[serde(rename = "session.attached")]
    SessionAttached {
        id: String,
        cols: u16,
        rows: u16,
        tail: String,
        /// tail 末尾对应的输出序号（客户端据此校准后续 PtyOutput；旧桌面端缺省 0）
        #[serde(default, skip_serializing_if = "is_zero_u64")]
        tail_seq: u64,
    },
    /// attach 被拒（现仅 "session not found"：会话已退出。per-session 控制权下
    /// attach 即抢占，不再有控制权拒绝）
    #[serde(rename = "session.attach.denied")]
    SessionAttachDenied { id: String, reason: String },
    /// 会话被收回（桌面接管 / 连接断开自动归还）
    #[serde(rename = "session.detached")]
    SessionDetached { id: String, reason: String },
    /// 会话工作状态变化
    #[serde(rename = "session.status")]
    SessionStatus { id: String, working: bool, pending: bool },
    /// 会话已退出
    #[serde(rename = "session.exit")]
    SessionExit { id: String },
    #[serde(rename = "pty.output")]
    PtyOutput {
        id: String,
        data: String,
        /// 会话内单调递增输出序号（客户端检测丢帧缺口；旧桌面端缺省 0）
        #[serde(default, skip_serializing_if = "is_zero_u64")]
        seq: u64,
    },
    #[serde(rename = "pong")]
    Pong,
    /// RPC 调用结果（与 rpc.call 的 id 对应）
    #[serde(rename = "rpc.result")]
    RpcResult {
        id: u64,
        ok: bool,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        data: Option<serde_json::Value>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        error: Option<String>,
    },
    /// 控制面事件转发（hook-event / session-taken-over 等，payload 与 Tauri 事件同构；
    /// 数据流 pty.output 走专用消息不经过此隧道）
    #[serde(rename = "rpc.event")]
    RpcEvent {
        name: String,
        payload: serde_json::Value,
    },
}

/// 序列化辅助：0 值省略，保证旧样例/旧客户端的 JSON 形态不变
fn is_zero_u64(v: &u64) -> bool {
    *v == 0
}

/// 票据连接的下载范围（受限会话：只允许拉取这一个文件的分块）
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DownloadScope {
    pub file_name: String,
    pub size: u64,
}

/// 远程可见的会话信息
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RemoteSession {
    pub id: String,
    #[serde(rename = "type")]
    pub pty_type: String,
    pub cwd: String,
    pub working: bool,
    pub pending: bool,
    /// Claude 会话 id（hook SessionStart 建立；shell 会话与旧版桌面端缺失）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub session_id: Option<String>,
    /// 会话所属 CC-Box 实例 id（设备级 Gateway 多实例聚合用；单实例/旧版缺失）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub instance_id: Option<String>,
    /// 实例展示名（如「实例 2」；缺失时手机端降级为不分组平铺）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub instance_name: Option<String>,
}

/// 远程可见的项目（新建会话用）
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RemoteProject {
    pub path: String,
    #[serde(default)]
    pub name: Option<String>,
}

/// 历史会话条目（恢复用）
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RemoteHistorySession {
    pub id: String,
    #[serde(default)]
    pub name: Option<String>,
    pub last_active_at: u64,
}

#[cfg(test)]
mod tests {
    use super::*;

    fn round_trip_client(raw: &str) {
        let msg: ClientMessage = serde_json::from_str(raw).expect("client deserialize failed");
        let out = serde_json::to_string(&msg).expect("client serialize failed");
        let original: serde_json::Value = serde_json::from_str(raw).unwrap();
        let round: serde_json::Value = serde_json::from_str(&out).unwrap();
        assert_eq!(original, round, "client round-trip mismatch");
    }

    fn round_trip_server(raw: &str) {
        let msg: ServerMessage = serde_json::from_str(raw).expect("server deserialize failed");
        let out = serde_json::to_string(&msg).expect("server serialize failed");
        let original: serde_json::Value = serde_json::from_str(raw).unwrap();
        let round: serde_json::Value = serde_json::from_str(&out).unwrap();
        assert_eq!(original, round, "server round-trip mismatch");
    }

    #[test]
    fn contract_client_hello() {
        round_trip_client(include_str!("../samples/client-hello.json"));
    }

    #[test]
    fn contract_client_sessions_list() {
        round_trip_client(include_str!("../samples/client-sessions-list.json"));
    }

    #[test]
    fn contract_client_projects_list() {
        round_trip_client(include_str!("../samples/client-projects-list.json"));
    }

    #[test]
    fn contract_client_session_create() {
        round_trip_client(include_str!("../samples/client-session-create.json"));
    }

    #[test]
    fn contract_client_session_create_resume() {
        round_trip_client(include_str!("../samples/client-session-create-resume.json"));
    }

    #[test]
    fn contract_client_session_create_opts() {
        round_trip_client(include_str!("../samples/client-session-create-opts.json"));
    }

    /// 多实例路由：session.create 带 instanceId（Gateway 转发到成员实例执行）
    #[test]
    fn contract_client_session_create_instance() {
        round_trip_client(include_str!("../samples/client-session-create-instance.json"));
        let msg: ClientMessage =
            serde_json::from_str(include_str!("../samples/client-session-create-instance.json"))
                .unwrap();
        assert_eq!(
            msg,
            ClientMessage::SessionCreate {
                path: "D:\\projects\\demo".to_string(),
                session_id: None,
                skip_permissions: None,
                custom_args: None,
                session_name: None,
                instance_id: Some("a3bb189e8f14ca27169d9a391f98a3ab".to_string()),
            }
        );
    }

    #[test]
    fn contract_client_sessions_history() {
        round_trip_client(include_str!("../samples/client-sessions-history.json"));
    }

    #[test]
    fn contract_client_session_kill() {
        round_trip_client(include_str!("../samples/client-session-kill.json"));
    }

    /// 重启场景的静默 kill：quiet=true 不广播 remote-session-closed（各端
    /// 保留 Tab 等 respawn），与显式关闭（默认，联动删 Tab）区分
    #[test]
    fn contract_client_session_kill_quiet() {
        round_trip_client(include_str!("../samples/client-session-kill-quiet.json"));
        let msg: ClientMessage =
            serde_json::from_str(include_str!("../samples/client-session-kill-quiet.json"))
                .unwrap();
        assert_eq!(
            msg,
            ClientMessage::SessionKill {
                id: "0b6b2d8e-1a2b-4c3d-9e8f-7a6b5c4d3e2f".to_string(),
                quiet: Some(true),
            }
        );
    }

    #[test]
    fn contract_client_session_subscribe() {
        round_trip_client(include_str!("../samples/client-session-subscribe.json"));
    }

    #[test]
    fn contract_client_session_unsubscribe() {
        round_trip_client(include_str!("../samples/client-session-unsubscribe.json"));
    }

    #[test]
    fn contract_client_session_attach() {
        round_trip_client(include_str!("../samples/client-session-attach.json"));
    }

    #[test]
    fn contract_client_session_detach() {
        round_trip_client(include_str!("../samples/client-session-detach.json"));
    }

    #[test]
    fn contract_client_pty_input() {
        round_trip_client(include_str!("../samples/client-pty-input.json"));
    }

    #[test]
    fn contract_client_pty_resize() {
        round_trip_client(include_str!("../samples/client-pty-resize.json"));
    }

    #[test]
    fn contract_client_ping() {
        round_trip_client(include_str!("../samples/client-ping.json"));
    }

    #[test]
    fn contract_client_rpc_call() {
        round_trip_client(include_str!("../samples/client-rpc-call.json"));
    }

    #[test]
    fn contract_client_controller_takeover() {
        round_trip_client(include_str!("../samples/client-controller-takeover.json"));
    }

    #[test]
    fn contract_server_session_attach_denied() {
        round_trip_server(include_str!("../samples/server-session-attach-denied.json"));
    }

    /// 向后兼容：旧桌面端 hello.ok 不带 connId/controller/revision（serde default）
    #[test]
    fn hello_ok_旧字段缺失可反序列化() {
        let msg: ServerMessage =
            serde_json::from_str(r#"{"type":"hello.ok","ver":1,"deviceName":"CC-Box"}"#).unwrap();
        match msg {
            ServerMessage::HelloOk { conn_id, controller, revision, .. } => {
                assert_eq!(conn_id, 0);
                assert_eq!(controller, None);
                assert_eq!(revision, 0);
            }
            other => panic!("expected HelloOk, got {:?}", other),
        }
        // 旧 sessions 消息缺 revision
        let msg: ServerMessage = serde_json::from_str(
            r#"{"type":"sessions","list":[]}"#,
        )
        .unwrap();
        match msg {
            ServerMessage::Sessions { list, revision } => {
                assert!(list.is_empty());
                assert_eq!(revision, 0);
            }
            other => panic!("expected Sessions, got {:?}", other),
        }
    }

    #[test]
    fn contract_server_hello_ok() {
        round_trip_server(include_str!("../samples/server-hello-ok.json"));
    }

    /// 票据连接：hello.ok 携带受限下载范围
    #[test]
    fn contract_server_hello_ok_download() {
        round_trip_server(include_str!("../samples/server-hello-ok-download.json"));
    }

    #[test]
    fn contract_server_hello_error() {
        round_trip_server(include_str!("../samples/server-hello-error.json"));
    }

    #[test]
    fn contract_server_sessions() {
        round_trip_server(include_str!("../samples/server-sessions.json"));
    }

    /// 向后兼容：旧桌面端 sessions 列表不带实例字段（serde default → None，
    /// 手机端降级为不分组平铺）
    #[test]
    fn contract_server_sessions_ungrouped() {
        round_trip_server(include_str!("../samples/server-sessions-ungrouped.json"));
        let msg: ServerMessage =
            serde_json::from_str(include_str!("../samples/server-sessions-ungrouped.json"))
                .unwrap();
        match msg {
            ServerMessage::Sessions { list, .. } => {
                assert!(list.iter().all(|s| s.instance_id.is_none() && s.instance_name.is_none()));
            }
            other => panic!("expected Sessions, got {:?}", other),
        }
    }

    #[test]
    fn contract_server_projects() {
        round_trip_server(include_str!("../samples/server-projects.json"));
    }

    #[test]
    fn contract_server_session_created() {
        round_trip_server(include_str!("../samples/server-session-created.json"));
    }

    #[test]
    fn contract_server_session_create_failed() {
        round_trip_server(include_str!("../samples/server-session-create-failed.json"));
    }

    #[test]
    fn contract_server_history_sessions() {
        round_trip_server(include_str!("../samples/server-history-sessions.json"));
    }

    #[test]
    fn contract_server_session_subscribed() {
        round_trip_server(include_str!("../samples/server-session-subscribed.json"));
    }

    #[test]
    fn contract_server_session_unsubscribed() {
        round_trip_server(include_str!("../samples/server-session-unsubscribed.json"));
    }

    #[test]
    fn contract_server_session_attached() {
        round_trip_server(include_str!("../samples/server-session-attached.json"));
    }

    #[test]
    fn contract_server_session_detached() {
        round_trip_server(include_str!("../samples/server-session-detached.json"));
    }

    #[test]
    fn contract_server_session_status() {
        round_trip_server(include_str!("../samples/server-session-status.json"));
    }

    #[test]
    fn contract_server_session_exit() {
        round_trip_server(include_str!("../samples/server-session-exit.json"));
    }

    #[test]
    fn contract_server_pty_output() {
        round_trip_server(include_str!("../samples/server-pty-output.json"));
    }

    #[test]
    fn contract_server_pong() {
        round_trip_server(include_str!("../samples/server-pong.json"));
    }

    #[test]
    fn contract_server_rpc_result_ok() {
        round_trip_server(include_str!("../samples/server-rpc-result-ok.json"));
    }

    #[test]
    fn contract_server_rpc_result_error() {
        round_trip_server(include_str!("../samples/server-rpc-result-error.json"));
    }

    #[test]
    fn contract_server_rpc_event() {
        round_trip_server(include_str!("../samples/server-rpc-event.json"));
    }

    #[test]
    fn unknown_type_is_rejected() {
        assert!(serde_json::from_str::<ClientMessage>(r#"{"type":"nonexistent"}"#).is_err());
        assert!(serde_json::from_str::<ServerMessage>(r#"{"type":"nonexistent"}"#).is_err());
    }

    /// 输出序号：新字段往返 + 旧帧缺省 0
    #[test]
    fn seq_fields_roundtrip_and_defaults() {
        let out = ServerMessage::PtyOutput {
            id: "p".into(),
            data: "x".into(),
            seq: 7,
        };
        let json = serde_json::to_string(&out).unwrap();
        assert!(json.contains(r#""seq":7"#));
        assert_eq!(serde_json::from_str::<ServerMessage>(&json).unwrap(), out);

        let old: ServerMessage =
            serde_json::from_str(r#"{"type":"pty.output","id":"p","data":"x"}"#).unwrap();
        assert!(matches!(old, ServerMessage::PtyOutput { seq: 0, .. }));

        let att = ServerMessage::SessionAttached {
            id: "p".into(),
            cols: 80,
            rows: 24,
            tail: "t".into(),
            tail_seq: 9,
        };
        let json = serde_json::to_string(&att).unwrap();
        assert!(json.contains(r#""tailSeq":9"#));
        assert_eq!(serde_json::from_str::<ServerMessage>(&json).unwrap(), att);

        let old: ServerMessage = serde_json::from_str(
            r#"{"type":"session.attached","id":"p","cols":80,"rows":24,"tail":"t"}"#,
        )
        .unwrap();
        assert!(matches!(
            old,
            ServerMessage::SessionAttached { tail_seq: 0, .. }
        ));

        let sub = ServerMessage::SessionSubscribed {
            id: "p".into(),
            tail: "t".into(),
            tail_seq: 3,
        };
        let json = serde_json::to_string(&sub).unwrap();
        assert!(json.contains(r#""tailSeq":3"#));
        assert_eq!(serde_json::from_str::<ServerMessage>(&json).unwrap(), sub);
    }

    #[test]
    fn missing_fields_are_rejected() {
        // hello 缺 token
        assert!(serde_json::from_str::<ClientMessage>(r#"{"type":"hello","ver":1,"client":"x"}"#).is_err());
        // session.subscribe 缺 id
        assert!(serde_json::from_str::<ClientMessage>(r#"{"type":"session.subscribe"}"#).is_err());
    }
}
