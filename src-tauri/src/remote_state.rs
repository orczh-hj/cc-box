//! 远程会话状态与应用层消息处理
//!
//! 局域网直连（remote_server）与中继模式（R3 remote_client）共用的唯一 handler 实现。
//! 不强依赖 Tauri AppHandle（测试不设置即可）：事件 emit 在设置后启用。

use std::collections::{HashMap, HashSet};
use std::sync::atomic::{AtomicU64, AtomicUsize, Ordering};
use std::sync::LazyLock;

use parking_lot::Mutex;
use tokio::sync::mpsc;

use cc_box_remote_protocol::{ClientMessage, RemoteSession, ServerMessage};

use crate::hook_events::{HookEventDetail, HookPayload};

/// 每会话输出 tail 上限（字节），超出丢弃头部
pub const TAIL_LIMIT: usize = 16 * 1024;

/// 每连接出站通道容量；输出高峰时 try_send 满则丢帧（只读监控可接受）
pub(crate) const CONN_CHANNEL_CAPACITY: usize = 256;

/// 控制权持有者：同一时间只有一端能操作某个终端（产品决策）
#[derive(Debug, Clone, PartialEq)]
pub enum ActiveView {
    Desktop,
    Remote(u64),
}

struct SessionEntry {
    pty_type: String,
    cwd: String,
    working: bool,
    pending: bool,
    turn_ended: bool,
    tail: String,
    seq: u64,
    active: ActiveView,
    /// 桌面端最近一次 resize 的尺寸（归还时恢复）
    desktop_size: Option<(u16, u16)>,
}

#[derive(Default)]
struct StateInner {
    sessions: HashMap<String, SessionEntry>,
    /// pty_id → 订阅它的连接集合（attach 同时视为订阅）
    subscribers: HashMap<String, HashSet<u64>>,
    /// conn_id → 出站通道
    conns: HashMap<u64, mpsc::Sender<ServerMessage>>,
}

pub struct RemoteState {
    inner: Mutex<StateInner>,
}

pub static REMOTE_STATE: LazyLock<RemoteState> =
    LazyLock::new(|| RemoteState { inner: Mutex::new(StateInner::default()) });

static NEXT_CONN_ID: AtomicU64 = AtomicU64::new(1);
static NEXT_SEQ: AtomicU64 = AtomicU64::new(1);

/// 远程触发的会话创建并发上限（每消息一线程 + spawn Claude 进程，防资源耗尽）
pub(crate) static REMOTE_SPAWN_INFLIGHT: AtomicUsize = AtomicUsize::new(0);
const REMOTE_SPAWN_LIMIT: usize = 8;

/// 桌面设备名（手机端 hello.ok 展示，用于区分多桌面）
fn device_name() -> String {
    static NAME: LazyLock<String> = LazyLock::new(|| {
        hostname::get()
            .map(|h| h.to_string_lossy().into_owned())
            .unwrap_or_else(|_| "CC-Box".to_string())
    });
    NAME.clone()
}

/// 远程可操作的项目白名单：仅允许已知项目（Claude projects 目录中出现过的路径）
fn is_known_project(path: &str) -> bool {
    let Ok(projects) = crate::store::get_projects(None, None) else {
        return false;
    };
    projects.iter().any(|p| paths_equivalent(&p.path, path))
}

/// 路径等价比较：优先 canonicalize（消大小写/分隔符差异），失败则归一化字符串
pub(crate) fn paths_equivalent(a: &str, b: &str) -> bool {
    let norm = |s: &str| -> String {
        std::path::Path::new(s)
            .canonicalize()
            .map(|p| p.to_string_lossy().to_lowercase())
            .unwrap_or_else(|_| s.trim_end_matches(['/', '\\']).to_lowercase())
    };
    norm(a) == norm(b)
}

/// 会话创建并发计数 guard（线程退出时自动释放）
struct SpawnGuard;
impl Drop for SpawnGuard {
    fn drop(&mut self) {
        REMOTE_SPAWN_INFLIGHT.fetch_sub(1, Ordering::AcqRel);
    }
}

/// 桌面事件回调（遮罩/建 tab 事件转发；remote_state 不直接依赖 tauri，测试环境不设置）
type EventHook = Box<dyn Fn(&str, &str, &str) + Send + Sync>;
static EVENT_HOOK: LazyLock<Mutex<Option<EventHook>>> = LazyLock::new(|| Mutex::new(None));

pub fn next_conn_id() -> u64 {
    NEXT_CONN_ID.fetch_add(1, Ordering::Relaxed)
}

/// 注册事件回调（lib.rs 初始化时调用一次；参数：事件名、pty_id、附加数据如 cwd）
pub fn set_event_hook(hook: EventHook) {
    *EVENT_HOOK.lock() = Some(hook);
}

/// 桌面通知 + 远程广播（payload 与 Tauri 事件同构 {id, data}）。
/// 事件单一出口：emit_event / emit_event_ex / emit_desktop_only 三变体按受众选择。
fn emit_event(name: &str, id: &str, data: &str) {
    emit_via_hook(name, id, data);
    REMOTE_STATE.broadcast_event(name, serde_json::json!({ "id": id, "data": data }));
}

/// 同 emit_event，但远程广播排除指定连接（如接管者自己不应收到 taken-over 遮罩）
fn emit_event_ex(name: &str, id: &str, data: &str, exclude_conn: Option<u64>) {
    emit_via_hook(name, id, data);
    REMOTE_STATE.broadcast_event_excluding(
        name,
        serde_json::json!({ "id": id, "data": data }),
        exclude_conn,
    );
}

/// 仅桌面通知（如 remote-session-created 建 tab：浏览器发起者已从应答拿到结果，
/// 广播会造成重复建 tab）
fn emit_desktop_only(name: &str, id: &str, data: &str) {
    emit_via_hook(name, id, data);
}

fn emit_via_hook(name: &str, id: &str, data: &str) {
    let guard = EVENT_HOOK.lock();
    if let Some(hook) = guard.as_ref() {
        hook(name, id, data);
    }
}

impl RemoteState {
    /// 构造独立实例（单元测试用；生产代码使用全局 REMOTE_STATE）
    pub(crate) fn new() -> Self {
        RemoteState { inner: Mutex::new(StateInner::default()) }
    }

    /// PTY 启动后注册（pty.rs spawn 成功时调用）
    pub fn register_pty(&self, id: &str, pty_type: &str, cwd: &str) {
        let mut inner = self.inner.lock();
        inner.sessions.insert(
            id.to_string(),
            SessionEntry {
                pty_type: pty_type.to_string(),
                cwd: cwd.to_string(),
                working: false,
                pending: false,
                turn_ended: false,
                tail: String::new(),
                seq: NEXT_SEQ.fetch_add(1, Ordering::Relaxed),
                active: ActiveView::Desktop,
                desktop_size: None,
            },
        );
    }

    /// PTY 退出/被杀后清理，通知所有连接。
    /// 状态读取与清理在**同一锁窗口**完成，消除与 handle_attach 的竞态
    /// （否则 attach 可能拿到已死会话并返回 SessionAttached，手机端挂着无通知的死终端）。
    /// exit 广播给全部连接而非仅订阅者：列表页未订阅也要实时移除卡片（R2-5）。
    pub fn remove_pty(&self, id: &str) {
        let was_remote = {
            let mut inner = self.inner.lock();
            let was_remote = inner
                .sessions
                .get(id)
                .map(|e| e.active != ActiveView::Desktop)
                .unwrap_or(false);
            inner.sessions.remove(id);
            inner.subscribers.remove(id);
            was_remote
        };
        if was_remote {
            emit_event("session-reclaimed", id, "");
        }
        self.broadcast_to_all(ServerMessage::SessionExit { id: id.to_string() });
    }

    /// 输出 tee（pty.rs 读取循环调用）：更新 tail 并转发给订阅者
    pub fn push_output(&self, id: &str, data: &str) {
        if data.is_empty() {
            return;
        }
        {
            let mut inner = self.inner.lock();
            match inner.sessions.get_mut(id) {
                Some(entry) => {
                    entry.tail.push_str(data);
                    truncate_tail_keep_end(&mut entry.tail, TAIL_LIMIT);
                }
                None => return,
            }
        }
        self.broadcast_to_subscribers(
            id,
            ServerMessage::PtyOutput {
                id: id.to_string(),
                data: data.to_string(),
            },
        );
    }

    /// 桌面端 resize 时记录尺寸（归还时恢复）
    pub fn record_size(&self, id: &str, cols: u16, rows: u16) {
        let mut inner = self.inner.lock();
        if let Some(entry) = inner.sessions.get_mut(id) {
            entry.desktop_size = Some((cols, rows));
        }
    }

    /// hook 事件 → working/pending 状态机（复刻 useStatusMonitor 语义，以 pty_id 为键）
    pub fn on_hook_payload(&self, payload: &HookPayload) {
        let Some(pty_id) = &payload.pty_id else { return };
        let status_msg = {
            let mut inner = self.inner.lock();
            let Some(entry) = inner.sessions.get_mut(pty_id) else { return };
            let before = (entry.working, entry.pending);
            apply_hook_event(entry, &payload.detail);
            let after = (entry.working, entry.pending);
            if before != after {
                Some(ServerMessage::SessionStatus {
                    id: pty_id.clone(),
                    working: after.0,
                    pending: after.1,
                })
            } else {
                None
            }
        };
        if let Some(msg) = status_msg {
            // 广播给全部连接而非仅订阅者：列表页 working/pending 圆点也要实时变化（R1-2）
            self.broadcast_to_all(msg);
        }
    }

    /// 连接认证通过后注册（remote_server 调用）
    pub fn add_conn(&self, conn_id: u64, sender: mpsc::Sender<ServerMessage>) {
        self.inner.lock().conns.insert(conn_id, sender);
    }

    /// 连接断开后清理全部订阅，并归还其接管的会话
    pub fn remove_conn(&self, conn_id: u64) {
        let reclaimed = {
            let mut inner = self.inner.lock();
            inner.conns.remove(&conn_id);
            for subs in inner.subscribers.values_mut() {
                subs.remove(&conn_id);
            }
            let mut ids = Vec::new();
            for (id, entry) in inner.sessions.iter_mut() {
                if entry.active == ActiveView::Remote(conn_id) {
                    entry.active = ActiveView::Desktop;
                    ids.push(id.clone());
                }
            }
            ids
        };
        for id in &reclaimed {
            // 手机断连：自动归还桌面（持有权不赖在手机上）
            emit_event("session-reclaimed", id, "");
            resize_pty(id, self.desktop_size_of(id));
            log::info!("[remote] session {} auto-reclaimed (conn {} closed)", id, conn_id);
        }
    }

    fn desktop_size_of(&self, id: &str) -> Option<(u16, u16)> {
        self.inner.lock().sessions.get(id).and_then(|e| e.desktop_size)
    }

    /// 该会话控制权当前是否在远程连接手中（桌面 resize 应让位，仅记录尺寸）
    pub fn is_remote_active(&self, id: &str) -> bool {
        let inner = self.inner.lock();
        matches!(
            inner.sessions.get(id).map(|e| &e.active),
            Some(ActiveView::Remote(_))
        )
    }

    /// 桌面端"接管回来"（Tauri 命令 pty_claim 调用）
    pub fn claim(&self, id: &str) {
        let old = {
            let mut inner = self.inner.lock();
            let Some(entry) = inner.sessions.get_mut(id) else { return };
            std::mem::replace(&mut entry.active, ActiveView::Desktop)
        };
        if let ActiveView::Remote(old_conn) = old {
            self.try_send(
                old_conn,
                ServerMessage::SessionDetached {
                    id: id.to_string(),
                    reason: "desktop".to_string(),
                },
            );
        }
        emit_event("session-reclaimed", id, "");
        resize_pty(id, self.desktop_size_of(id));
    }

    /// 应用层消息分发（认证后的消息，局域网/中继同一条路径）
    pub fn handle_client_message(&self, conn_id: u64, msg: &ClientMessage) {
        match msg {
            // hello 的 token 认证由连接层（LAN）或 relay 完成，此处统一回复 hello.ok——
            // 中继模式下手机消息经桥接直达这里，若无此回复手机将永远停在认证中
            ClientMessage::Hello { .. } => {
                self.try_send(
                    conn_id,
                    ServerMessage::HelloOk {
                        ver: cc_box_remote_protocol::PROTOCOL_VERSION,
                        device_name: device_name(),
                    },
                );
            }
            ClientMessage::SessionsList => {
                let list = self.session_list();
                self.try_send(conn_id, ServerMessage::Sessions { list });
            }
            ClientMessage::ProjectsList => {
                let list = crate::store::get_projects(Some(30), None)
                    .unwrap_or_default()
                    .into_iter()
                    .map(|p| cc_box_remote_protocol::RemoteProject {
                        path: p.path,
                        name: Some(p.name),
                    })
                    .collect();
                self.try_send(conn_id, ServerMessage::Projects { list });
            }
            ClientMessage::SessionCreate { path, session_id, skip_permissions, custom_args } => {
                // 并发上限（先于白名单：检查最便宜且无需 IO）
                if REMOTE_SPAWN_INFLIGHT.fetch_add(1, Ordering::AcqRel) >= REMOTE_SPAWN_LIMIT {
                    REMOTE_SPAWN_INFLIGHT.fetch_sub(1, Ordering::AcqRel);
                    self.try_send(
                        conn_id,
                        ServerMessage::SessionCreateFailed {
                            error: "too many concurrent requests".to_string(),
                        },
                    );
                    return;
                }
                // 白名单检查（is_known_project 含目录扫描 + canonicalize）与
                // spawn_claude 都放线程执行——在收包任务同步做会阻塞所有
                // 连接的消息处理与中继 keepalive。回包经克隆的连接通道直发
                // （不依赖全局 REMOTE_STATE，独立实例的测试同样可收）
                let path = path.clone();
                let resume = session_id.clone().unwrap_or_default();
                let cli_args = crate::pty::build_claude_args(
                    session_id.as_deref(),
                    skip_permissions.unwrap_or(false),
                    custom_args.as_deref().unwrap_or(""),
                );
                let args = (!cli_args.is_empty()).then_some(cli_args);
                let conn_tx = { self.inner.lock().conns.get(&conn_id).cloned() };
                let reply = move |msg: ServerMessage| {
                    if let Some(tx) = &conn_tx {
                        let _ = tx.try_send(msg);
                    } else {
                        REMOTE_STATE.try_send(conn_id, msg);
                    }
                };
                std::thread::spawn(move || {
                    let _guard = SpawnGuard;
                    if !is_known_project(&path) {
                        log::warn!("[remote] session create rejected (unknown project): {}", path);
                        reply(ServerMessage::SessionCreateFailed {
                            error: "unknown project".to_string(),
                        });
                        return;
                    }
                    // spawn_claude 阻塞（含子进程启动等待）
                    // 参数拼接走 pty::build_claude_args 单一核心（与桌面前端规则锚定一致）
                    let spawned = crate::pty::get_pty_manager()
                        .ok_or_else(|| anyhow::anyhow!("PTY manager not initialized"))
                        .and_then(|m| m.spawn_claude(&path, 80, 24, args));
                    match spawned {
                        Ok(info) => {
                            // spawn_claude 内部已 register_pty；通知桌面建 tab 绑定该 PTY
                            // data 为 JSON（cwd 含 \t 等特殊字符时 \t 拼接会截断）
                            let data = serde_json::json!({"cwd": info.cwd, "resume": resume}).to_string();
                            emit_desktop_only("remote-session-created", &info.id, &data);
                            reply(ServerMessage::SessionCreated {
                                id: info.id,
                                path: info.cwd,
                            });
                        }
                        Err(e) => {
                            log::warn!("[remote] session create failed for {}: {}", path, e);
                            reply(ServerMessage::SessionCreateFailed {
                                error: e.to_string(),
                            });
                        }
                    }
                });
            }
            ClientMessage::SessionsHistory { path } => {
                // 白名单同 session.create（未知项目回空列表，手机 UI 不报错）；
                // is_known_project 与 get_sessions 都涉及目录扫描，放线程执行
                let path = path.clone();
                std::thread::spawn(move || {
                    let list = if !is_known_project(&path) {
                        Vec::new()
                    } else {
                        crate::store::get_sessions(&path, 15, 0)
                            .unwrap_or_default()
                            .into_iter()
                            .map(|s| cc_box_remote_protocol::RemoteHistorySession {
                                id: s.session_id,
                                name: Some(s.name),
                                last_active_at: s.last_active_at,
                            })
                            .collect()
                    };
                    REMOTE_STATE.try_send(conn_id, ServerMessage::HistorySessions { list });
                });
            }
            ClientMessage::SessionKill { id } => {
                // kill 内部等待子进程退出（最多 5s），放线程执行；
                // kill 成功后 pty-exit/remove_pty 链路自动通知两端
                let id = id.clone();
                std::thread::spawn(move || {
                    if let Some(manager) = crate::pty::get_pty_manager() {
                        if let Err(e) = manager.kill(&id) {
                            log::warn!("[remote] session kill {} failed: {}", id, e);
                        }
                    }
                });
            }
            ClientMessage::SessionSubscribe { id } => {
                let (exists, tail) = {
                    let mut inner = self.inner.lock();
                    let tail = inner.sessions.get(id).map(|e| e.tail.clone());
                    match tail {
                        Some(tail) => {
                            inner
                                .subscribers
                                .entry(id.clone())
                                .or_default()
                                .insert(conn_id);
                            (true, tail)
                        }
                        None => (false, String::new()),
                    }
                };
                if exists {
                    self.try_send(
                        conn_id,
                        ServerMessage::SessionSubscribed {
                            id: id.clone(),
                            tail,
                        },
                    );
                } else {
                    // 会话不存在：以 exit 告知，客户端按已退出处理
                    self.try_send(conn_id, ServerMessage::SessionExit { id: id.clone() });
                }
            }
            ClientMessage::SessionUnsubscribed { id } => {
                self.inner
                    .lock()
                    .subscribers
                    .get_mut(id)
                    .map(|subs| subs.remove(&conn_id));
                self.try_send(
                    conn_id,
                    ServerMessage::SessionUnsubscribed { id: id.clone() },
                );
            }
            ClientMessage::SessionAttach { id, cols, rows } => {
                self.handle_attach(conn_id, id, *cols, *rows);
            }
            ClientMessage::SessionDetach { id } => {
                let was_active = {
                    let mut inner = self.inner.lock();
                    match inner.sessions.get_mut(id) {
                        Some(entry) if entry.active == ActiveView::Remote(conn_id) => {
                            entry.active = ActiveView::Desktop;
                            true
                        }
                        _ => false,
                    }
                };
                if was_active {
                    self.inner
                        .lock()
                        .subscribers
                        .get_mut(id)
                        .map(|subs| subs.remove(&conn_id));
                    emit_event("session-reclaimed", id, "");
                    resize_pty(id, self.desktop_size_of(id));
                }
                self.try_send(
                    conn_id,
                    ServerMessage::SessionDetached {
                        id: id.clone(),
                        reason: "self".to_string(),
                    },
                );
            }
            ClientMessage::PtyInput { id, data } => {
                let allowed = {
                    let inner = self.inner.lock();
                    inner
                        .sessions
                        .get(id)
                        .map(|e| e.active == ActiveView::Remote(conn_id))
                        .unwrap_or(false)
                };
                if allowed {
                    if let Some(manager) = crate::pty::get_pty_manager() {
                        if let Err(e) = manager.write(id, data) {
                            log::warn!("[remote] pty input {} failed: {}", id, e);
                        }
                    }
                } else {
                    log::debug!("[remote] conn {} input ignored (not active) for {}", conn_id, id);
                }
            }
            ClientMessage::PtyResize { id, cols, rows } => {
                let allowed = {
                    let inner = self.inner.lock();
                    inner
                        .sessions
                        .get(id)
                        .map(|e| e.active == ActiveView::Remote(conn_id))
                        .unwrap_or(false)
                };
                if allowed {
                    resize_pty(id, Some((*cols, *rows)));
                }
            }
            ClientMessage::Ping => self.try_send(conn_id, ServerMessage::Pong),
            // 通用 RPC：浏览器端调用 Tauri command（白名单与分发见 rpc_dispatch）
            ClientMessage::RpcCall { id, cmd, args } => {
                crate::rpc_dispatch::handle_rpc(conn_id, *id, cmd, args);
            }
        }
    }

    /// attach：抢占式接管（后 attach 优先）。旧持有者（桌面或另一远程端）保留
    /// 订阅继续收输出——被接管只是显示层暂时遮罩（数据层同步），随时可重 attach 抢回
    fn handle_attach(&self, conn_id: u64, id: &str, cols: u16, rows: u16) {
        let outcome = {
            let mut inner = self.inner.lock();
            let Some(entry) = inner.sessions.get_mut(id) else {
                drop(inner);
                self.try_send(conn_id, ServerMessage::SessionExit { id: id.to_string() });
                return;
            };
            let old = std::mem::replace(&mut entry.active, ActiveView::Remote(conn_id));
            let tail = entry.tail.clone();
            // attach 同时视为输出订阅者（旧订阅者保留，与桌面 claim 语义一致）
            let subs = inner.subscribers.entry(id.to_string()).or_default();
            subs.insert(conn_id);
            AttachOutcome { old, tail }
        };

        // 旧持有者遮罩通知：远程互抢与桌面被抢统一走 taken-over 广播
        // （遮罩 + 接管回来按钮）；排除接管者自己（不应给自己加遮罩）。
        // 广播走 self（emit_event_ex 走全局 REMOTE_STATE，单元测试的独立实例收不到）
        let notify_old = match outcome.old {
            ActiveView::Remote(old_conn) => old_conn != conn_id,
            ActiveView::Desktop => true,
        };
        if notify_old {
            emit_via_hook("session-taken-over", id, "");
            self.broadcast_event_excluding(
                "session-taken-over",
                serde_json::json!({ "id": id, "data": "" }),
                Some(conn_id),
            );
        }

        // resize 为手机端尺寸
        resize_pty(id, Some((cols, rows)));

        self.try_send(
            conn_id,
            ServerMessage::SessionAttached {
                id: id.to_string(),
                cols,
                rows,
                tail: outcome.tail,
            },
        );
    }

    /// 会话列表（按注册顺序）
    pub fn session_list(&self) -> Vec<RemoteSession> {
        let inner = self.inner.lock();
        let mut entries: Vec<(u64, RemoteSession)> = inner
            .sessions
            .iter()
            .map(|(id, e)| {
                (
                    e.seq,
                    RemoteSession {
                        id: id.clone(),
                        pty_type: e.pty_type.clone(),
                        cwd: e.cwd.clone(),
                        working: e.working,
                        pending: e.pending,
                    },
                )
            })
            .collect();
        entries.sort_by_key(|(seq, _)| *seq);
        entries.into_iter().map(|(_, s)| s).collect()
    }

    fn broadcast_to_subscribers(&self, id: &str, msg: ServerMessage) {
        let inner = self.inner.lock();
        if let Some(subs) = inner.subscribers.get(id) {
            for conn_id in subs {
                if let Some(tx) = inner.conns.get(conn_id) {
                    let _ = tx.try_send(msg.clone());
                }
            }
        }
    }

    /// 广播给全部在线连接（会话列表级信息：exit、working/pending 状态）
    fn broadcast_to_all(&self, msg: ServerMessage) {
        let inner = self.inner.lock();
        for tx in inner.conns.values() {
            let _ = tx.try_send(msg.clone());
        }
    }

    /// 控制面事件转发（浏览器端经 rpc.event 收取；数据流 pty.output 走专用消息
    /// 不经过此隧道）。事件单一出口：remote_state::emit_event / events.rs::emit_all
    pub fn broadcast_event(&self, name: &str, payload: serde_json::Value) {
        self.broadcast_to_all(ServerMessage::RpcEvent {
            name: name.to_string(),
            payload,
        });
    }

    /// 同 broadcast_event，排除指定连接（受众精准化的广播）
    pub fn broadcast_event_excluding(
        &self,
        name: &str,
        payload: serde_json::Value,
        exclude: Option<u64>,
    ) {
        let inner = self.inner.lock();
        let msg = ServerMessage::RpcEvent {
            name: name.to_string(),
            payload,
        };
        for (conn_id, tx) in inner.conns.iter() {
            if Some(*conn_id) == exclude {
                continue;
            }
            let _ = tx.try_send(msg.clone());
        }
    }

    pub(crate) fn try_send(&self, conn_id: u64, msg: ServerMessage) {
        let inner = self.inner.lock();
        if let Some(tx) = inner.conns.get(&conn_id) {
            let _ = tx.try_send(msg);
        }
    }
}

struct AttachOutcome {
    old: ActiveView,
    tail: String,
}

/// 执行 PTY resize（manager 未初始化或会话不存在时跳过——单元测试环境）
fn resize_pty(id: &str, size: Option<(u16, u16)>) {
    let Some((cols, rows)) = size else { return };
    if let Some(manager) = crate::pty::get_pty_manager() {
        if let Err(e) = manager.resize(id, cols, rows) {
            log::debug!("[remote] resize {} to {}x{} failed: {}", id, cols, rows, e);
        }
    }
}

/// working/pending 状态机（语义与前端 useStatusMonitor 一致）：
/// - userPromptSubmit 开启新回合 working
/// - 回合结束（stop/notification idle）后，activity 事件不恢复 working
/// - permission 通知：working=false + pending=true，回合不结束（授权后 activity 恢复）
fn apply_hook_event(entry: &mut SessionEntry, detail: &HookEventDetail) {
    match detail {
        HookEventDetail::UserPromptSubmit(_) => {
            entry.turn_ended = false;
            entry.working = true;
        }
        HookEventDetail::PreToolUse(_)
        | HookEventDetail::PostToolUse(_)
        | HookEventDetail::PostToolUseFailure(_)
        | HookEventDetail::SubagentStart(_)
        | HookEventDetail::SubagentStop(_)
        | HookEventDetail::PreCompact(_)
        | HookEventDetail::PostCompact(_) => {
            if entry.turn_ended {
                return;
            }
            entry.working = true;
            entry.pending = false;
        }
        HookEventDetail::Notification(n) => {
            if !entry.working {
                return;
            }
            match n.notification_type.as_deref() {
                Some("idle_prompt") => {
                    entry.turn_ended = true;
                    entry.working = false;
                    entry.pending = true;
                }
                Some("permission_prompt") | Some("worker_permission_prompt") => {
                    entry.working = false;
                    entry.pending = true;
                }
                _ => {}
            }
        }
        HookEventDetail::Stop(_) | HookEventDetail::StopFailure(_) => {
            if !entry.working {
                return;
            }
            entry.turn_ended = true;
            entry.working = false;
            entry.pending = true;
        }
        HookEventDetail::SessionEnd(_) => {
            if !entry.working {
                return;
            }
            entry.working = false;
        }
        _ => {}
    }
}

/// 截断到 limit 字节以内，保留尾部；起点对齐 UTF-8 字符边界
fn truncate_tail_keep_end(s: &mut String, limit: usize) {
    if s.len() <= limit {
        return;
    }
    let mut keep_from = s.len() - limit;
    while keep_from < s.len() && !s.is_char_boundary(keep_from) {
        keep_from += 1;
    }
    s.drain(..keep_from);
}
