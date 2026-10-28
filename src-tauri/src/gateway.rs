//! Broker 侧设备级路由与成员注册表
//!
//! Broker 永远是设备级 Gateway，不再有 GUI 竞选。每个 GUI 实例都是一名成员：
//! 注册时上报实例 id/label/协议版本/会话快照，Broker 维护
//! `pty_id -> instance_id` 路由、全局控制权真源、聚合适配器会话列表。
//!
//! 关键不变量：
//! - **单一有序队列**：每个成员只有一条 `mpsc<GwDown>`，控制面与输入同序；
//!   满时对可靠消息断开成员触发重注册（绝不静默丢弃 `PtyInput`）。
//! - **先登记再回成功**：成员的 `SessionCreated{id}` 到达时先写路由，再投递
//!   给手机，消除“创建成功却 attach 不到”的竞态。
//! - **未知 pty_id 不回退本机**：Broker 没有 PTY，拒绝并触发会话重拉。
//! - **断连清理**：路由、会话缓存、粘连路由、待处理 RPC/创建全部回收，并向
//!   手机发 `SessionDetached` 避免幽灵挂载。

use std::collections::{HashMap, HashSet};
use std::sync::LazyLock;
use std::time::Duration;

use axum::extract::ws::{Message, WebSocket, WebSocketUpgrade};
use axum::extract::ConnectInfo;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use futures_util::{SinkExt, StreamExt};
use parking_lot::Mutex;
use tokio::sync::{mpsc, watch};

use cc_box_remote_protocol::{ClientMessage, RemoteSession, ServerMessage};

use crate::gateway_proto::{
    instance_id, GwDown, GwUp, WindowAction, GW_PROTO_VER, LEGACY_PROTO_VER,
};

/// Brokered member outbound queue capacity
const MEMBER_QUEUE_CAPACITY: usize = 4096;
/// 托盘窗口标题兜底（窗口尚未上报真实标题时）
const TITLE_FALLBACK: &str = "CC-Box";
/// Reliable enqueue timeout; on expiry the member is treated as unavailable.
const RELIABLE_ENQUEUE_TIMEOUT: Duration = Duration::from_secs(5);

pub(crate) struct MemberEntry {
    pub tx: mpsc::Sender<GwDown>,
    pub close_tx: watch::Sender<bool>,
    pub name: String,
    /// GUI 窗口标题（托盘展示；缺省回退 name）
    pub title: String,
    pub order: u32,
    pub proto_ver: u32,
    /// v2 GUI 进程 PID；旧版成员/未知为 0
    pub pid: u32,
    pub sessions: HashMap<String, RemoteSession>,
}

/// 托盘/窗口管理所需的成员快照
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct WindowInfo {
    pub id: String,
    pub name: String,
    pub title: String,
    pub pid: u32,
    pub proto_ver: u32,
    pub session_count: usize,
    /// v2 且有 PID 才能由 Broker 关闭/重启
    pub closable: bool,
}

#[derive(Default)]
struct GatewayStateInner {
    members: HashMap<String /*instance_id*/, MemberEntry>,
    /// session id -> instance id
    route: HashMap<String, String>,
    /// file upload stickiness: conn_id -> instance id
    conn_rpc_route: HashMap<u64, String>,
    next_instance_no: u32,
    /// 旧手机不带 instanceId 时的默认目标（最近注册的成员）
    default_member: Option<String>,
    /// (conn_id, instance_id) -> 待完成的创建请求数
    pending_creates: HashMap<(u64, String), u32>,
    /// (conn_id, rpc_id) -> instance_id
    pending_rpcs: HashMap<(u64, u64), String>,
}

static GW_STATE: LazyLock<Mutex<GatewayStateInner>> =
    LazyLock::new(|| Mutex::new(GatewayStateInner::default()));

pub fn member_count() -> usize {
    GW_STATE.lock().members.len()
}

fn route_of(id: &str) -> Option<String> {
    GW_STATE.lock().route.get(id).cloned()
}

fn default_member() -> Option<String> {
    GW_STATE.lock().default_member.clone()
}

/// 手机连接断开：清理粘连路由与待处理 RPC（创建请求由成员断连清理）
pub(crate) fn conn_dropped(conn_id: u64) {
    let mut st = GW_STATE.lock();
    st.conn_rpc_route.remove(&conn_id);
    st.pending_rpcs.retain(|(c, _), _| *c != conn_id);
    // Broker 常驻：手机断连时创建请求可能永远等不到成员应答
    st.pending_creates.retain(|(c, _), _| *c != conn_id);
}

/// 实例名单（设备级）：所有已注册 GUI 成员，按注册序。
/// Broker 自身不是实例（没有 PTY/窗口），因此不再注入“实例 1”。
pub(crate) fn instance_roster() -> Vec<(String, String)> {
    let st = GW_STATE.lock();
    let mut members: Vec<_> = st
        .members
        .iter()
        .map(|(id, m)| (m.order, id.clone(), m.name.clone()))
        .collect();
    members.sort_by_key(|(order, _, _)| *order);
    members.into_iter().map(|(_, id, name)| (id, name)).collect()
}

/// 窗口快照（托盘窗口子菜单/关闭重启用），按注册序。
pub(crate) fn window_roster() -> Vec<WindowInfo> {
    let st = GW_STATE.lock();
    let mut members: Vec<_> = st
        .members
        .iter()
        .map(|(id, m)| {
            (
                m.order,
                WindowInfo {
                    id: id.clone(),
                    name: m.name.clone(),
                    title: m.title.clone(),
                    pid: m.pid,
                    proto_ver: m.proto_ver,
                    session_count: m.sessions.len(),
                    closable: m.proto_ver >= GW_PROTO_VER && m.pid != 0,
                },
            )
        })
        .collect();
    members.sort_by_key(|(order, _)| *order);
    members.into_iter().map(|(_, w)| w).collect()
}

/// `solicited=true` 表示该事件是对 `sessions.list` 的定向应答（只是补发名单）；
/// 主动通知（实例上/下线、会话增删）为 false，前端必须重拉会话列表。
fn roster_payload(solicited: bool) -> serde_json::Value {
    serde_json::json!({
        "instances": instance_roster()
            .into_iter()
            .map(|(id, name)| serde_json::json!({ "id": id, "name": name }))
            .collect::<Vec<_>>(),
        "solicited": solicited,
    })
}

pub(crate) fn notify_sessions_changed() {
    crate::remote_state::REMOTE_STATE.broadcast_event(
        crate::events::names::INSTANCES_CHANGED,
        roster_payload(false),
    );
}

/// 成员注册。返回分配的展示名；重复 instance_id 返回 None。
/// `label` 为 GUI 的稳定展示名提示（cwd basename / hostname），同名时追加序号。
pub(crate) fn register_member(
    instance_id: String,
    label: Option<String>,
    proto_ver: u32,
    pid: u32,
    window_title: Option<String>,
    mut sessions: Vec<RemoteSession>,
    tx: mpsc::Sender<GwDown>,
    close_tx: watch::Sender<bool>,
) -> Option<String> {
    let mut st = GW_STATE.lock();
    if st.members.contains_key(&instance_id) {
        return None;
    }
    st.next_instance_no += 1;
    let order = st.next_instance_no;
    let window_title = window_title
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty());
    // 手机分组/实例选择器同样使用真实窗口标题（新窗口为 CC-Box，打开项目后
    // 更新为项目名）；旧客户端没有 windowTitle 时才回退 label。
    let base = window_title.clone().unwrap_or_else(|| {
        label
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
            .unwrap_or_else(|| format!("窗口 {}", order))
    });
    let existing: HashSet<String> = st.members.values().map(|m| m.name.clone()).collect();
    let mut name = base.clone();
    let mut n = 2;
    while existing.contains(&name) {
        name = format!("{} ({})", base, n);
        n += 1;
    }
    for s in sessions.iter_mut() {
        s.instance_id = Some(instance_id.clone());
        s.instance_name = Some(name.clone());
        st.route.insert(s.id.clone(), instance_id.clone());
    }
    let sessions = sessions
        .into_iter()
        .map(|s| (s.id.clone(), s))
        .collect::<HashMap<_, _>>();
    // 托盘展示真实的 GUI 窗口标题；旧客户端/读取失败时回退 CC-Box，
    // 不再展示 hostname/label（用户明确要求窗口名用 Title）。
    let title = window_title.unwrap_or_else(|| TITLE_FALLBACK.to_string());
    st.members.insert(
        instance_id.clone(),
        MemberEntry {
            tx,
            close_tx,
            name: name.clone(),
            title,
            order,
            proto_ver,
            pid,
            sessions,
        },
    );
    st.default_member = Some(instance_id);
    Some(name)
}

/// 成员断开清理：路由/缓存/粘连/待处理请求全部回收，并通知手机会话已脱离。
pub(crate) fn unregister_member(instance_id: &str) -> Vec<RemoteSession> {
    let (sessions, failed_creates, failed_rpcs) = {
        let mut st = GW_STATE.lock();
        let sessions: Vec<RemoteSession> = st
            .members
            .remove(instance_id)
            .map(|e| e.sessions.into_values().collect())
            .unwrap_or_default();
        for s in &sessions {
            st.route.remove(&s.id);
        }
        st.conn_rpc_route.retain(|_, inst| inst != instance_id);
        // 待处理创建：逐个回失败（同一 conn 可能并发多个 create）
        let mut failed_creates: Vec<u64> = Vec::new();
        let keys: Vec<(u64, String)> = st
            .pending_creates
            .keys()
            .filter(|k| k.1.as_str() == instance_id)
            .cloned()
            .collect();
        for key in keys {
            if let Some(count) = st.pending_creates.remove(&key) {
                for _ in 0..count {
                    failed_creates.push(key.0);
                }
            }
        }
        // 待处理 RPC：先摘除，锁外再回 member disconnected（避免持 GW_STATE
        // 调 REMOTE_STATE.try_send 取 inner 锁，与事件 hook 形成 ABBA 死锁）
        let rpcs: Vec<(u64, u64)> = st
            .pending_rpcs
            .iter()
            .filter(|(_, inst)| inst.as_str() == instance_id)
            .map(|(k, _)| *k)
            .collect();
        for key in &rpcs {
            st.pending_rpcs.remove(key);
        }
        // 默认成员转移：取剩余最小注册序，保持旧手机创建目标稳定
        if st.default_member.as_deref() == Some(instance_id) {
            st.default_member = st
                .members
                .iter()
                .min_by_key(|(_, m)| m.order)
                .map(|(id, _)| id.clone());
        }
        (sessions, failed_creates, rpcs)
    };

    for (conn_id, rpc_id) in failed_rpcs {
        crate::remote_state::REMOTE_STATE.try_send(
            conn_id,
            ServerMessage::RpcResult {
                id: rpc_id,
                ok: false,
                data: None,
                error: Some("instance disconnected".to_string()),
            },
        );
    }
    for conn_id in failed_creates {
        crate::remote_state::REMOTE_STATE.try_send(
            conn_id,
            ServerMessage::SessionCreateFailed {
                error: "target instance unavailable".to_string(),
            },
        );
    }
    for s in &sessions {
        // 会话仍存活于成员进程：发 detached 让手机保留 Tab 但标记断开，
        // 成员重连后由 instances-changed/sessions 列表触发 re-attach。
        crate::remote_state::REMOTE_STATE.broadcast_to_all(ServerMessage::SessionDetached {
            id: s.id.clone(),
            reason: "instance disconnected".to_string(),
        });
    }
    // 在线实例数变化 → 刷新其余 GUI 的状态镜像（面板/托盘一致）
    broadcast_broker_status();
    sessions
}

/// 窗口标题变化：同时更新手机分组展示名（去重），并刷新该成员已有会话的
/// instance_name；托盘使用 raw title + 自己的去重逻辑。
fn update_member_title(instance_id: &str, title: String) {
    let title = title.trim().to_string();
    if title.is_empty() {
        return;
    }
    let changed = {
        let mut st = GW_STATE.lock();
        if !st.members.contains_key(instance_id) {
            return;
        }
        let existing: HashSet<String> = st
            .members
            .iter()
            .filter(|(id, _)| id.as_str() != instance_id)
            .map(|(_, m)| m.name.clone())
            .collect();
        let mut name = title.clone();
        let mut n = 2;
        while existing.contains(&name) {
            name = format!("{} ({})", title, n);
            n += 1;
        }
        let Some(m) = st.members.get_mut(instance_id) else {
            return;
        };
        m.title = title;
        if m.name == name {
            false
        } else {
            m.name = name.clone();
            for s in m.sessions.values_mut() {
                s.instance_name = Some(name.clone());
            }
            true
        }
    };
    if changed {
        notify_sessions_changed();
    }
}

fn member_session_upsert(instance_id: &str, mut session: RemoteSession) {
    let is_new = {
        let mut st = GW_STATE.lock();
        if !st.members.contains_key(instance_id) {
            return;
        }
        let sid = session.id.clone();
        let is_new = !st.route.contains_key(&sid);
        st.route.insert(sid.clone(), instance_id.to_string());
        let Some(m) = st.members.get_mut(instance_id) else {
            return;
        };
        session.instance_id = Some(instance_id.to_string());
        session.instance_name = Some(m.name.clone());
        m.sessions.insert(sid, session);
        is_new
    };
    if is_new {
        notify_sessions_changed();
    }
}

fn member_session_remove(instance_id: &str, id: &str) {
    {
        let mut st = GW_STATE.lock();
        if let Some(m) = st.members.get_mut(instance_id) {
            m.sessions.remove(id);
        }
        st.route.remove(id);
    }
    crate::remote_state::REMOTE_STATE.broadcast_to_all(ServerMessage::SessionExit {
        id: id.to_string(),
    });
    notify_sessions_changed();
}

/// 成员完成创建：**先登记路由**（最小会话条目），调用方随后才把
/// `SessionCreated` 投递给手机。后续 `SessionUpsert` 覆盖最小条目。
fn member_route_from_session_created(instance_id: &str, id: &str, path: &str) -> bool {
    let is_new = {
        let mut st = GW_STATE.lock();
        if !st.members.contains_key(instance_id) {
            return false;
        }
        let is_new = !st.route.contains_key(id);
        st.route.insert(id.to_string(), instance_id.to_string());
        let name = st
            .members
            .get(instance_id)
            .map(|m| m.name.clone())
            .unwrap_or_default();
        if let Some(m) = st.members.get_mut(instance_id) {
            m.sessions.entry(id.to_string()).or_insert_with(|| RemoteSession {
                id: id.to_string(),
                pty_type: "claude".to_string(),
                cwd: path.to_string(),
                working: false,
                pending: false,
                session_id: None,
                instance_id: Some(instance_id.to_string()),
                instance_name: Some(name),
            });
        }
        is_new
    };
    if is_new {
        notify_sessions_changed();
    }
    is_new
}

fn clear_pending_create(conn_id: u64, instance_id: &str) {
    let mut st = GW_STATE.lock();
    let key = (conn_id, instance_id.to_string());
    if let Some(count) = st.pending_creates.get_mut(&key) {
        if *count > 1 {
            *count -= 1;
        } else {
            st.pending_creates.remove(&key);
        }
    }
}

fn clear_pending_rpc(conn_id: u64, rpc_id: u64) {
    GW_STATE.lock().pending_rpcs.remove(&(conn_id, rpc_id));
}

/// 设备级中继状态镜像：旧成员收 RelayStatus，v2 成员另收完整 BrokerStatus。
pub fn sync_relay_status() {
    let (enabled, connected, url, pair_code) = crate::remote_client::status_snapshot();
    broadcast_down(GwDown::RelayStatus {
        enabled,
        connected,
        url,
        pair_code,
    });
    if let Some(status) = crate::broker::status_snapshot() {
        broadcast_down(GwDown::BrokerStatus { status });
    }
}

/// v2 成员状态镜像刷新（实例上/下线后在线数量变化）
pub(crate) fn broadcast_broker_status() {
    if let Some(status) = crate::broker::status_snapshot() {
        broadcast_down(GwDown::BrokerStatus { status });
    }
}

/// v2 桌面事件扇出（Broker 本地 state-changed 等）
pub(crate) fn broadcast_desktop_event(name: &str, payload: serde_json::Value) {
    broadcast_down(GwDown::DesktopEvent {
        name: name.to_string(),
        payload,
    });
}

/// 向所有成员广播。可靠消息入队失败 → 关闭该成员（重注册 + 快照）；
/// v2-only 消息自动跳过旧成员。
pub(crate) fn broadcast_down(msg: GwDown) {
    broadcast_down_excluding(msg, None);
}

/// 同 [`broadcast_down`]，但跳过指定实例：发起成员的 state-changed 已由
/// 成员本地 Tauri 事件通知，且本端有写回环抑制——二次回发会击穿抑制，
/// 触发发起端全量刷新（桌面隐藏项目列表闪动回归）。
pub(crate) fn broadcast_down_excluding(msg: GwDown, exclude: Option<&str>) {
    let targets: Vec<(String, mpsc::Sender<GwDown>, u32)> = {
        let st = GW_STATE.lock();
        st.members
            .iter()
            .filter(|(id, _)| Some(id.as_str()) != exclude)
            .map(|(id, m)| (id.clone(), m.tx.clone(), m.proto_ver))
            .collect()
    };
    for (id, tx, ver) in targets {
        if msg.requires_v2() && ver < GW_PROTO_VER {
            continue;
        }
        if tx.try_send(msg.clone()).is_err() {
            log::warn!("[broker-router] member {} queue full; forcing reconnect", short(&id));
            close_member(&id);
        }
    }
}

fn close_member(instance_id: &str) {
    let close = GW_STATE
        .lock()
        .members
        .get(instance_id)
        .map(|m| m.close_tx.clone());
    if let Some(tx) = close {
        let _ = tx.send(true);
    }
}

fn short(id: &str) -> &str {
    &id[..8.min(id.len())]
}

#[cfg(test)]
pub(crate) fn test_route_of(id: &str) -> Option<String> {
    route_of(id)
}

#[cfg(test)]
pub(crate) fn test_reset() {
    *GW_STATE.lock() = GatewayStateInner::default();
}

/// 可靠投递到成员：单一有序队列 + 5s 超时。超时视为成员失联并关闭连接。
pub(crate) async fn send_to_member(instance_id: &str, msg: GwDown) -> Result<(), String> {
    let tx = {
        let st = GW_STATE.lock();
        st.members
            .get(instance_id)
            .filter(|m| !(msg.requires_v2() && m.proto_ver < GW_PROTO_VER))
            .map(|m| m.tx.clone())
    };
    let Some(tx) = tx else {
        return Err("member not found".to_string());
    };
    match tokio::time::timeout(RELIABLE_ENQUEUE_TIMEOUT, tx.send(msg)).await {
        Ok(Ok(())) => Ok(()),
        _ => {
            log::warn!(
                "[broker-router] reliable enqueue to {} timed out; forcing reconnect",
                short(instance_id)
            );
            close_member(instance_id);
            Err("member unavailable".to_string())
        }
    }
}

/// 窗口控制指令（v2-only；v1 成员会被 `send_to_member` 拒绝）。
pub(crate) async fn send_window_command(
    instance_id: &str,
    action: WindowAction,
) -> Result<(), String> {
    send_to_member(instance_id, GwDown::WindowCommand { action }).await
}

/// 聚合会话列表。Broker 本地通常为空；仅当存在本机会话（测试/旧路径）时
/// 才注入“实例 1”，避免 Broker 进程伪造一个不存在的实例。
pub fn merged_session_list(mut local: Vec<RemoteSession>) -> Vec<RemoteSession> {
    if !local.is_empty() {
        let me = instance_id().to_string();
        for s in local.iter_mut() {
            s.instance_id = Some(me.clone());
            s.instance_name = Some("窗口 1".to_string());
        }
    }
    let st = GW_STATE.lock();
    let mut members: Vec<&MemberEntry> = st.members.values().collect();
    members.sort_by_key(|m| m.order);
    for m in members {
        for s in m.sessions.values() {
            local.push(s.clone());
        }
    }
    local
}

enum RouteTarget {
    Local,
    Member(String),
    UnknownSession,
    NoInstance,
}

/// 手机消息路由入口（LAN/relay 收包点）。`async` 以支持可靠投递/背压。
pub async fn dispatch(conn_id: u64, msg: &ClientMessage) {
    use crate::remote_state::REMOTE_STATE;
    if !REMOTE_STATE.conn_message_allowed(conn_id, msg) {
        return;
    }
    if matches!(msg, ClientMessage::SessionsList) {
        REMOTE_STATE.send_event_to(
            conn_id,
            crate::events::names::INSTANCES_CHANGED,
            roster_payload(true),
        );
    }
    if let ClientMessage::RpcCall { cmd, .. } = msg {
        if cmd == "file_upload_begin" {
            GW_STATE.lock().conn_rpc_route.remove(&conn_id);
        }
    }

    let target = match msg {
        ClientMessage::Hello { .. }
        | ClientMessage::Ping
        | ClientMessage::SessionsList
        | ClientMessage::ProjectsList
        | ClientMessage::SessionsHistory { .. }
        | ClientMessage::ControllerTakeover => RouteTarget::Local,
        ClientMessage::SessionCreate {
            instance_id: Some(inst),
            ..
        } => RouteTarget::Member(inst.clone()),
        ClientMessage::SessionCreate {
            instance_id: None, ..
        } => match default_member() {
            Some(inst) => RouteTarget::Member(inst),
            None => RouteTarget::NoInstance,
        },
        ClientMessage::SessionAttach { id, .. }
        | ClientMessage::SessionDetach { id }
        | ClientMessage::SessionSubscribe { id }
        | ClientMessage::SessionUnsubscribed { id }
        | ClientMessage::SessionKill { id, .. }
        | ClientMessage::PtyInput { id, .. }
        | ClientMessage::PtyResize { id, .. } => match route_of(id) {
            Some(inst) => RouteTarget::Member(inst),
            None => RouteTarget::UnknownSession,
        },
        ClientMessage::RpcCall { cmd, args, .. } => {
            match rpc_call_target(conn_id, cmd, args) {
                RpcTarget::Member(inst) => RouteTarget::Member(inst),
                RpcTarget::Local => RouteTarget::Local,
                RpcTarget::Unknown => RouteTarget::UnknownSession,
            }
        }
    };

    match target {
        RouteTarget::Local => REMOTE_STATE.handle_client_message(conn_id, msg),
        RouteTarget::NoInstance => {
            REMOTE_STATE.try_send(
                conn_id,
                ServerMessage::SessionCreateFailed {
                    error: "no GUI instance available".to_string(),
                },
            );
        }
        RouteTarget::UnknownSession => reject_unknown_session(conn_id, msg),
        RouteTarget::Member(inst) => {
            let down = GwDown::ClientMsg {
                conn_id,
                via_relay: REMOTE_STATE.conn_via_relay(conn_id).unwrap_or(true),
                msg: msg.clone(),
            };
            match send_to_member(&inst, down).await {
                Ok(()) => {
                    let mut st = GW_STATE.lock();
                    match msg {
                        ClientMessage::SessionCreate { .. } => {
                            *st.pending_creates
                                .entry((conn_id, inst.clone()))
                                .or_insert(0) += 1;
                        }
                        ClientMessage::RpcCall { id, cmd, .. } => {
                            st.pending_rpcs.insert((conn_id, *id), inst.clone());
                            if cmd == "file_upload_begin" {
                                st.conn_rpc_route.insert(conn_id, inst.clone());
                            }
                        }
                        _ => {}
                    }
                }
                Err(e) => {
                    log::warn!("[broker-router] forward to {} failed: {}", short(&inst), e);
                    match msg {
                        ClientMessage::SessionCreate { .. } => REMOTE_STATE.try_send(
                            conn_id,
                            ServerMessage::SessionCreateFailed {
                                error: "target instance unavailable".to_string(),
                            },
                        ),
                        ClientMessage::RpcCall { id, .. } => REMOTE_STATE.try_send(
                            conn_id,
                            ServerMessage::RpcResult {
                                id: *id,
                                ok: false,
                                data: None,
                                error: Some("target instance unavailable".to_string()),
                            },
                        ),
                        _ => {
                            // 无应答的可靠消息（PtyInput/Resize 等）投递失败：
                            // 显式断开手机连接，浏览器重连后重新 attach/同步。
                            REMOTE_STATE.remove_conn(conn_id);
                        }
                    }
                }
            }
        }
    }

    if let ClientMessage::RpcCall { cmd, .. } = msg {
        if !cmd.starts_with("file_upload")
            || cmd == "file_upload_finish"
            || cmd == "file_upload_abort"
        {
            GW_STATE.lock().conn_rpc_route.remove(&conn_id);
        }
    }
}

enum RpcTarget {
    Local,
    Member(String),
    Unknown,
}

fn rpc_call_target(conn_id: u64, cmd: &str, args: &serde_json::Value) -> RpcTarget {
    if let Some(pid) = args.get("ptyId").and_then(|v| v.as_str()) {
        return match route_of(pid) {
            Some(inst) => RpcTarget::Member(inst),
            None => RpcTarget::Unknown,
        };
    }
    if cmd.starts_with("file_upload") {
        return match GW_STATE.lock().conn_rpc_route.get(&conn_id).cloned() {
            Some(inst) => RpcTarget::Member(inst),
            None => RpcTarget::Unknown,
        };
    }
    RpcTarget::Local
}

fn reject_unknown_session(conn_id: u64, msg: &ClientMessage) {
    use crate::remote_state::REMOTE_STATE;
    match msg {
        ClientMessage::SessionAttach { id, .. } => REMOTE_STATE.try_send(
            conn_id,
            ServerMessage::SessionAttachDenied {
                id: id.clone(),
                reason: "session not found".to_string(),
            },
        ),
        ClientMessage::SessionSubscribe { id } => REMOTE_STATE.try_send(
            conn_id,
            ServerMessage::SessionExit { id: id.clone() },
        ),
        ClientMessage::SessionDetach { id } => REMOTE_STATE.try_send(
            conn_id,
            ServerMessage::SessionDetached {
                id: id.clone(),
                reason: "session not found".to_string(),
            },
        ),
        ClientMessage::RpcCall { id, .. } => REMOTE_STATE.try_send(
            conn_id,
            ServerMessage::RpcResult {
                id: *id,
                ok: false,
                data: None,
                error: Some("session not found".to_string()),
            },
        ),
        ClientMessage::PtyInput { id, .. } | ClientMessage::PtyResize { id, .. } => {
            log::debug!("[broker-router] input for unknown session {} ignored", id);
        }
        ClientMessage::SessionKill { .. } => {}
        _ => {}
    }
    // 触发状态同步：手机端重拉聚合列表（列表与路由真源对齐）
    notify_sessions_changed();
    let list = merged_session_list(Vec::new());
    let revision = REMOTE_STATE.revision();
    REMOTE_STATE.try_send(conn_id, ServerMessage::Sessions { list, revision });
}

/// 成员上行处理（IPC 与 legacy WS 共用）。
/// 返回 false 表示连接应关闭（Done/Bye）。
pub(crate) async fn handle_member_up(instance_id: &str, up: GwUp) -> bool {
    match up {
        GwUp::SessionUpsert { session } => member_session_upsert(instance_id, session),
        GwUp::SessionRemove { id } => member_session_remove(instance_id, &id),
        GwUp::ToConn { conn_id, msg } => {
            let msg = match msg {
                // 创建成功：先登记路由，再投递手机（消除 attach 竞态）
                ServerMessage::SessionCreated { id, path } => {
                    member_route_from_session_created(instance_id, &id, &path);
                    clear_pending_create(conn_id, instance_id);
                    ServerMessage::SessionCreated { id, path }
                }
                ServerMessage::SessionCreateFailed { error } => {
                    clear_pending_create(conn_id, instance_id);
                    ServerMessage::SessionCreateFailed { error }
                }
                ServerMessage::RpcResult {
                    id,
                    ok,
                    data,
                    error,
                } => {
                    clear_pending_rpc(conn_id, id);
                    ServerMessage::RpcResult {
                        id,
                        ok,
                        data,
                        error,
                    }
                }
                // 成员进程的 revision 与 Broker 不可比：重新分配设备级 revision，
                // 否则手机按 appliedRevision 会把成员写操作当旧事件永久丢弃。
                ServerMessage::RpcEvent {
                    name,
                    mut payload,
                } if name == crate::events::names::STATE_CHANGED => {
                    let rev = crate::remote_state::REMOTE_STATE.next_revision();
                    if let serde_json::Value::Object(ref mut map) = payload {
                        map.insert(
                            "data".to_string(),
                            serde_json::Value::String(rev.to_string()),
                        );
                    }
                    ServerMessage::RpcEvent { name, payload }
                }
                other => other,
            };
            crate::remote_state::REMOTE_STATE.deliver_to_conn(conn_id, msg);
        }
        GwUp::WindowTitle { title } => update_member_title(instance_id, title),
        GwUp::StateChanged { section } => {
            // 成员 revision 与 Broker 不可比：重新分配设备级 revision 后
            // 扇出给其他 v2 成员（DesktopEvent）与手机（RpcEvent）。
            // 发起成员不回发：其本端已有本地 Tauri 通知 + 写回环抑制，
            // 二次回环会击穿抑制触发全量刷新。
            let rev = crate::remote_state::REMOTE_STATE.next_revision();
            let payload =
                serde_json::json!({ "id": section, "data": rev.to_string() });
            broadcast_down_excluding(
                GwDown::DesktopEvent {
                    name: crate::events::names::STATE_CHANGED.to_string(),
                    payload: payload.clone(),
                },
                Some(instance_id),
            );
            crate::remote_state::REMOTE_STATE
                .broadcast_event(crate::events::names::STATE_CHANGED, payload);
        }
        GwUp::Heartbeat { ts } => {
            let _ = send_to_member(instance_id, GwDown::HeartbeatAck { ts }).await;
        }
        GwUp::Done | GwUp::Bye => return false,
        GwUp::MemberHello { .. } => {}
    }
    true
}

// ==================== HTTP/WS 路由（挂在 Broker 的 LAN 服务上） ====================

/// GET /gateway.ok：新版 Broker 探针（旧版无此路由 → 404）
pub async fn gateway_ok_handler() -> Response {
    let instances = GW_STATE.lock().members.len();
    (
        StatusCode::OK,
        axum::Json(serde_json::json!({
            "gateway": true,
            "ver": GW_PROTO_VER,
            "instances": instances,
        })),
    )
        .into_response()
}

/// GET /internal/gw-ws：旧版成员注册 WS（仅 loopback + 密钥认证）
pub async fn gw_ws_handler(
    ConnectInfo(addr): ConnectInfo<std::net::SocketAddr>,
    ws: WebSocketUpgrade,
) -> Response {
    if !addr.ip().is_loopback() {
        return StatusCode::FORBIDDEN.into_response();
    }
    ws.on_upgrade(handle_member_socket)
}

async fn handle_member_socket(mut socket: WebSocket) {
    let first = match tokio::time::timeout(Duration::from_secs(10), socket.recv()).await {
        Ok(Some(Ok(msg))) => msg,
        _ => return,
    };
    let (ver, id, secret, label, pid, window_title, sessions) = match first {
        Message::Text(t) => match serde_json::from_str::<GwUp>(&t) {
            Ok(GwUp::MemberHello {
                ver,
                instance_id: id,
                secret,
                label,
                pid,
                window_title,
                sessions,
            }) => (ver, id, secret, label, pid, window_title, sessions),
            _ => return,
        },
        _ => return,
    };

    let expected = read_gateway_secret();
    let ok = (ver == LEGACY_PROTO_VER || ver == GW_PROTO_VER)
        && expected.as_deref() == Some(secret.as_str());
    if !ok {
        let _ = socket
            .send(Message::Text(
                serde_json::to_string(&GwDown::Reject {
                    reason: "invalid secret or protocol version".to_string(),
                })
                .unwrap_or_default(),
            ))
            .await;
        return;
    }

    let (down_tx, mut down_rx) = mpsc::channel::<GwDown>(MEMBER_QUEUE_CAPACITY);
    let (close_tx, mut close_rx) = watch::channel(false);
    let Some(name) = register_member(
        id.clone(),
        label,
        ver,
        pid,
        window_title,
        sessions,
        down_tx.clone(),
        close_tx,
    ) else {
        let _ = socket
            .send(Message::Text(
                serde_json::to_string(&GwDown::Reject {
                    reason: "duplicate instance id".to_string(),
                })
                .unwrap_or_default(),
            ))
            .await;
        return;
    };
    log::info!("[broker-router] member registered: {} ({})", name, short(&id));

    let _ = down_tx
        .send(GwDown::RegisterOk {
            instance_name: name.clone(),
            gateway_id: instance_id().to_string(),
        })
        .await;
    let (enabled, connected, url, pair_code) = crate::remote_client::status_snapshot();
    let _ = down_tx
        .send(GwDown::RelayStatus {
            enabled,
            connected,
            url,
            pair_code,
        })
        .await;
    if ver >= GW_PROTO_VER {
        if let Some(status) = crate::broker::status_snapshot() {
            let _ = down_tx.send(GwDown::BrokerStatus { status }).await;
        }
    }

    notify_sessions_changed();
    broadcast_broker_status();

    let (mut sender, mut receiver) = socket.split();
    let send_task = tokio::spawn(async move {
        while let Some(msg) = down_rx.recv().await {
            let Ok(text) = serde_json::to_string(&msg) else {
                continue;
            };
            if sender.send(Message::Text(text)).await.is_err() {
                break;
            }
        }
    });

    let mut shutdown = crate::remote_server::shutdown_signal();
    let _ = shutdown.borrow_and_update();
    loop {
        tokio::select! {
            _ = close_rx.changed() => {
                if *close_rx.borrow() { break; }
            }
            _ = shutdown.changed() => break,
            msg = receiver.next() => match msg {
                Some(Ok(Message::Text(t))) => match serde_json::from_str::<GwUp>(&t) {
                    Ok(up) => {
                        if !handle_member_up(&id, up).await { break; }
                    }
                    Err(e) => log::debug!(
                        "[broker-router] member {} invalid frame ignored: {}",
                        short(&id),
                        e
                    ),
                },
                Some(Ok(Message::Close(_))) | None | Some(Err(_)) => break,
                Some(Ok(_)) => {}
            },
        }
    }

    send_task.abort();
    let sessions = unregister_member(&id);
    log::info!(
        "[broker-router] member {} left ({} sessions)",
        name,
        sessions.len()
    );
    notify_sessions_changed();
}

// ==================== 兼容转发（由 ownership 模块提供） ====================

pub(crate) fn read_gateway_secret() -> Option<String> {
    crate::broker::ownership::read_gateway_secret()
}

