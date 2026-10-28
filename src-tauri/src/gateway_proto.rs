//! 设备级 Broker 实例间 IPC 协议（Broker ↔ GUI 实例）
//!
//! 一台电脑只有一个 Broker（设备身份/LAN/relay/实例注册），
//! 所有 GUI 实例都是成员：通过本协议注册、上报会话、接收路由后的手机消息。
//! 旧版 GUI 仍可经 `/internal/gw-ws` 以 v1 协议接入；新版经 Named Pipe /
//! Unix Domain Socket 以 v2 协议接入。手机协议消息
//! （ClientMessage/ServerMessage）作为 payload 原样内嵌透传。

use cc_box_remote_protocol::{ClientMessage, RemoteSession, ServerMessage};

/// 本进程实例身份（uuid v4 simple；内存持有，进程生命周期内稳定）
pub fn instance_id() -> &'static str {
    static ID: std::sync::LazyLock<String> =
        std::sync::LazyLock::new(|| uuid::Uuid::new_v4().simple().to_string());
    &ID
}

/// 新版 IPC 协议版本（Named Pipe / UDS）
pub const GW_PROTO_VER: u32 = 2;
/// 旧版 loopback WebSocket 成员协议版本（仅兼容旧 GUI）
pub const LEGACY_PROTO_VER: u32 = 1;

/// Broker → 成员（JSON 帧）
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(tag = "type", rename_all_fields = "camelCase")]
pub enum GwDown {
    /// 注册成功（Broker 分配的展示名）
    #[serde(rename = "register.ok")]
    RegisterOk {
        instance_name: String,
        /// Broker 的实例 id（旧字段名保留；断连后成员可据此立即回收过期锁）
        #[serde(default)]
        gateway_id: String,
    },
    /// 注册被拒（密钥错/协议版本不匹配/重复 instanceId）
    #[serde(rename = "reject")]
    Reject { reason: String },
    /// 手机消息原样转发（conn_id 透传；via_relay 保证文件限额档位正确）
    #[serde(rename = "client.msg")]
    ClientMsg {
        conn_id: u64,
        via_relay: bool,
        msg: ClientMessage,
    },
    /// 手机断开：成员侧 remove_conn（归还 active + resize 恢复桌面尺寸）
    #[serde(rename = "conn.drop")]
    ConnDrop { conn_id: u64 },
    /// Broker 退出/停止（v1 兼容；v2 用 BrokerExit）
    #[serde(rename = "shutdown")]
    Shutdown,
    /// 设备级中继状态镜像（旧字段形态；v2 用 BrokerStatus）
    #[serde(rename = "relay.status")]
    RelayStatus {
        enabled: bool,
        connected: bool,
        url: String,
        pair_code: String,
    },
    /// v2：Broker 运行状态全量镜像（远程面板/命令用）
    #[serde(rename = "broker.status")]
    BrokerStatus { status: BrokerStatus },
    /// v2：Broker 自身桌面事件扇出（state-changed 等）
    #[serde(rename = "desktop.event")]
    DesktopEvent {
        name: String,
        payload: serde_json::Value,
    },
    /// v2：Broker 生命周期退出（quit/update/idle）
    #[serde(rename = "broker.exit")]
    BrokerExit { reason: BrokerExitReason },
    /// v2：心跳应答
    #[serde(rename = "heartbeat.ack")]
    HeartbeatAck { ts: u64 },
    /// v2：Broker 要求窗口执行动作（目前仅关闭；重启由 Broker 关闭后重新拉起）
    #[serde(rename = "window.command")]
    WindowCommand { action: WindowAction },
}

/// 窗口动作（Broker → GUI）
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum WindowAction {
    /// 结束本窗口全部 PTY 并退出进程
    Close,
    /// 还原/置顶并聚焦窗口
    Show,
    /// 显示窗口并打开远程连接面板
    OpenRemote,
    /// 显示窗口并跳转到设置 → 更新（托盘「更新到新版本」）
    OpenUpdate,
}

/// 成员 → Broker（JSON 帧）
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(tag = "type", rename_all_fields = "camelCase")]
pub enum GwUp {
    /// 注册首条消息（密钥 + 本实例当前会话快照）
    #[serde(rename = "member.hello")]
    MemberHello {
        ver: u32,
        instance_id: String,
        secret: String,
        /// v2：实例展示名提示（GUI 用首个会话 cwd basename / hostname）；
        /// Broker 保证唯一，旧成员缺省
        #[serde(default, skip_serializing_if = "Option::is_none")]
        label: Option<String>,
        /// v2：本 GUI 进程 PID（托盘关闭/重启窗口的强杀兜底）；v1 缺省 0
        #[serde(default)]
        pid: u32,
        /// v2：GUI 窗口当前标题（托盘窗口列表展示）；缺省回退 label
        #[serde(default, skip_serializing_if = "Option::is_none")]
        window_title: Option<String>,
        sessions: Vec<RemoteSession>,
    },
    /// 会话增量：新建/sessionId 建立/工作状态变化
    #[serde(rename = "session.upsert")]
    SessionUpsert { session: RemoteSession },
    /// 会话移除
    #[serde(rename = "session.remove")]
    SessionRemove { id: String },
    /// 定向上行：发给指定手机连接的协议消息
    #[serde(rename = "to.conn")]
    ToConn { conn_id: u64, msg: ServerMessage },
    /// 成员主动退出（进程退出前的礼貌通知；断连同样触发清理）
    #[serde(rename = "done")]
    Done,
    /// v2：窗口标题变化（项目切换等）；Broker 更新托盘展示与手机分组名
    #[serde(rename = "window.title")]
    WindowTitle { title: String },
    /// v2：成员本地写命令后的 state-changed；由 Broker 重新分配设备级
    /// revision 并扇出给所有成员/手机，解决跨窗口状态不同步。
    #[serde(rename = "state.changed")]
    StateChanged { section: String },
    /// v2：心跳
    #[serde(rename = "heartbeat")]
    Heartbeat { ts: u64 },
    /// v2：成员优雅退出（等价 Done，语义更明确）
    #[serde(rename = "bye")]
    Bye,
}

/// Broker 生命周期退出原因
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum BrokerExitReason {
    /// 用户从托盘显式退出：持久抑制 GUI 自动拉起
    Quit,
    /// 安装更新前退出：版本变化后解除抑制
    Update,
    /// 旧版空闲回收（新版 Broker 常驻，不再发送；仅兼容旧 Broker）
    Idle,
    /// Broker 主动关闭远程服务（不应导致退出；保留给协议完整性）
    Shutdown,
}

/// Broker 运行状态快照（v2）
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BrokerStatus {
    pub enabled: bool,
    pub lan_running: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub lan_error: Option<String>,
    pub legacy_owned: bool,
    pub instance_count: u32,
    pub port: u16,
    pub relay_enabled: bool,
    pub relay_connected: bool,
    pub relay_url: String,
    pub relay_pair_code: String,
    pub revision: u64,
    pub app_version: String,
    pub broker_pid: u32,
    /// 开机自启动是否已启用
    #[serde(default)]
    pub autostart: bool,
}

/// 管理操作（与成员上行帧共用连接，首帧 type = "admin"）
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(tag = "type", rename_all_fields = "camelCase")]
pub enum BrokerAdminRequest {
    #[serde(rename = "admin")]
    Admin {
        id: u64,
        secret: String,
        op: BrokerOp,
    },
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(tag = "op", rename_all = "camelCase", rename_all_fields = "camelCase")]
pub enum BrokerOp {
    GetStatus,
    SetEnabled { enabled: bool },
    RestartRemote,
    RefreshPairCode,
    RotateSecret,
    SetRelayUrl {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        url: Option<String>,
    },
    Quit,
    /// 安装更新前退出；`keep_instance_id` 为发起更新的窗口，其余窗口先关闭
    PrepareForUpdate {
        #[serde(default)]
        keep_instance_id: Option<String>,
    },
    /// 开/关开机自启动
    SetAutostart { enabled: bool },
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(tag = "type", rename_all_fields = "camelCase")]
pub enum BrokerAdminResponse {
    #[serde(rename = "admin.result")]
    Result {
        id: u64,
        ok: bool,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        status: Option<BrokerStatus>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        error: Option<String>,
    },
}

impl GwDown {
    /// v2-only 消息：不得发给旧版成员
    pub fn requires_v2(&self) -> bool {
        matches!(
            self,
            GwDown::BrokerStatus { .. }
                | GwDown::DesktopEvent { .. }
                | GwDown::BrokerExit { .. }
                | GwDown::HeartbeatAck { .. }
                | GwDown::WindowCommand { .. }
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn GatewayProto_DownRoundTrip_1() {
        let down = GwDown::ClientMsg {
            conn_id: 7,
            via_relay: true,
            msg: ClientMessage::PtyInput {
                id: "abc".into(),
                data: "ls\r".into(),
            },
        };
        let json = serde_json::to_string(&down).unwrap();
        assert!(json.contains(r#""type":"client.msg""#));
        let back: GwDown = serde_json::from_str(&json).unwrap();
        assert_eq!(down, back);
    }

    #[test]
    fn GatewayProto_UpRoundTrip_2() {
        let up = GwUp::ToConn {
            conn_id: 7,
            msg: ServerMessage::PtyOutput {
                id: "abc".into(),
                data: "hello".into(),
                seq: 3,
            },
        };
        let json = serde_json::to_string(&up).unwrap();
        assert!(json.contains(r#""type":"to.conn""#));
        let back: GwUp = serde_json::from_str(&json).unwrap();
        assert_eq!(up, back);
    }

    #[test]
    fn GatewayProto_RegisterFlow_3() {
        let hello = GwUp::MemberHello {
            ver: GW_PROTO_VER,
            instance_id: instance_id().to_string(),
            secret: "s".into(),
            label: Some("proj".into()),
            pid: 1234,
            window_title: Some("Proj".into()),
            sessions: vec![],
        };
        let json = serde_json::to_string(&hello).unwrap();
        let back: GwUp = serde_json::from_str(&json).unwrap();
        assert_eq!(hello, back);

        let ok = GwDown::RegisterOk {
            instance_name: "实例 1".into(),
            gateway_id: String::new(),
        };
        let back: GwDown = serde_json::from_str(&serde_json::to_string(&ok).unwrap()).unwrap();
        assert_eq!(ok, back);
    }

    #[test]
    fn GatewayProto_UnknownTypeRejected_4() {
        assert!(serde_json::from_str::<GwDown>(r#"{"type":"nonexistent"}"#).is_err());
        assert!(serde_json::from_str::<GwUp>(r#"{"type":"nonexistent"}"#).is_err());
    }

    #[test]
    fn GatewayProto_LegacyControllerFramesDropped_5() {
        // 控制权已 per-session 化：controller.set / desktop.takeover variant 已删。
        // 旧成员/旧 Broker 发出的这两类帧反序列化失败被上层 log 忽略（连接不断）
        assert!(serde_json::from_str::<GwDown>(
            r#"{"type":"controller.set","holder":7}"#
        )
        .is_err());
        assert!(serde_json::from_str::<GwUp>(
            r#"{"type":"desktop.takeover"}"#
        )
        .is_err());
    }

    #[test]
    fn GatewayProto_LegacyHelloWithoutLabel_6() {
        // v1 成员不带 label：serde default 兼容
        let raw = r#"{"type":"member.hello","ver":1,"instanceId":"i","secret":"s","sessions":[]}"#;
        let hello: GwUp = serde_json::from_str(raw).unwrap();
        match hello {
            GwUp::MemberHello { ver, label, .. } => {
                assert_eq!(ver, LEGACY_PROTO_VER);
                assert_eq!(label, None);
            }
            other => panic!("unexpected {:?}", other),
        }
    }

    #[test]
    fn GatewayProto_LegacyClientMsgHolderIgnored_7() {
        // 旧 Broker 的 client.msg 帧（per-session 化之前）携带 holder 字段：
        // 新成员解析时未知字段被忽略（serde 默认），消息本身不受影响
        let raw = r#"{"type":"client.msg","connId":1,"viaRelay":false,"msg":{"type":"ping"},"holder":null}"#;
        let down: GwDown = serde_json::from_str(raw).unwrap();
        assert!(matches!(down, GwDown::ClientMsg { conn_id: 1, .. }));
    }

    #[test]
    fn GatewayProto_AdminRoundTrip_8() {
        let req = BrokerAdminRequest::Admin {
            id: 1,
            secret: "s".into(),
            op: BrokerOp::SetEnabled { enabled: true },
        };
        let json = serde_json::to_string(&req).unwrap();
        assert!(json.contains(r#""type":"admin""#));
        assert!(json.contains(r#""op":"setEnabled""#));
        let back: BrokerAdminRequest = serde_json::from_str(&json).unwrap();
        assert_eq!(req, back);

        let resp = BrokerAdminResponse::Result {
            id: 1,
            ok: true,
            status: None,
            error: None,
        };
        let json = serde_json::to_string(&resp).unwrap();
        assert!(json.contains(r#""type":"admin.result""#));
    }

    #[test]
    fn GatewayProto_RequiresV2_9() {
        assert!(GwDown::BrokerExit {
            reason: BrokerExitReason::Quit
        }
        .requires_v2());
        assert!(!GwDown::Shutdown.requires_v2());
        assert!(!GwDown::RelayStatus {
            enabled: false,
            connected: false,
            url: String::new(),
            pair_code: String::new()
        }
        .requires_v2());
    }

    #[test]
    fn GatewayProto_WindowCommandAndPid_10() {
        for action in [
            WindowAction::Close,
            WindowAction::Show,
            WindowAction::OpenRemote,
            WindowAction::OpenUpdate,
        ] {
            let cmd = GwDown::WindowCommand { action };
            let json = serde_json::to_string(&cmd).unwrap();
            let back: GwDown = serde_json::from_str(&json).unwrap();
            assert_eq!(cmd, back);
            assert!(cmd.requires_v2());
        }

        let title = GwUp::WindowTitle {
            title: "project".into(),
        };
        let json = serde_json::to_string(&title).unwrap();
        assert_eq!(serde_json::from_str::<GwUp>(&json).unwrap(), title);

        let state = GwUp::StateChanged {
            section: "providers".into(),
        };
        let json = serde_json::to_string(&state).unwrap();
        assert!(json.contains(r#""type":"state.changed""#));
        assert_eq!(serde_json::from_str::<GwUp>(&json).unwrap(), state);

        let with_pid: GwUp = serde_json::from_str(
            r#"{"type":"member.hello","ver":2,"instanceId":"i","secret":"s","label":"p","pid":4321,"windowTitle":"Proj","sessions":[]}"#,
        )
        .unwrap();
        assert!(matches!(
            with_pid,
            GwUp::MemberHello {
                pid: 4321,
                window_title: Some(ref t),
                ..
            } if t == "Proj"
        ));

        let legacy: GwUp = serde_json::from_str(
            r#"{"type":"member.hello","ver":1,"instanceId":"i","secret":"s","sessions":[]}"#,
        )
        .unwrap();
        assert!(matches!(legacy, GwUp::MemberHello { pid: 0, .. }));
    }

    #[test]
    fn GatewayProto_PrepareForUpdateCompat_11() {
        // 旧 GUI 发送的 unit 形态（无 keepInstanceId）
        let old: BrokerAdminRequest = serde_json::from_str(
            r#"{"type":"admin","id":1,"secret":"s","op":{"op":"prepareForUpdate"}}"#,
        )
        .unwrap();
        assert!(matches!(
            old,
            BrokerAdminRequest::Admin {
                op: BrokerOp::PrepareForUpdate {
                    keep_instance_id: None
                },
                ..
            }
        ));

        let new: BrokerAdminRequest = serde_json::from_str(
            r#"{"type":"admin","id":2,"secret":"s","op":{"op":"prepareForUpdate","keepInstanceId":"abc"}}"#,
        )
        .unwrap();
        assert!(matches!(
            new,
            BrokerAdminRequest::Admin {
                op: BrokerOp::PrepareForUpdate {
                    keep_instance_id: Some(ref id)
                },
                ..
            } if id == "abc"
        ));

        let auto = BrokerAdminRequest::Admin {
            id: 3,
            secret: "s".into(),
            op: BrokerOp::SetAutostart { enabled: true },
        };
        let json = serde_json::to_string(&auto).unwrap();
        assert!(json.contains(r#""op":"setAutostart""#));
        assert_eq!(
            serde_json::from_str::<BrokerAdminRequest>(&json).unwrap(),
            auto
        );
    }
}
