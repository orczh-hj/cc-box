//! 远程会话状态与应用层消息处理
//!
//! 局域网直连（remote_server）与中继模式（R3 remote_client）共用的唯一 handler 实现。
//! 不强依赖 Tauri AppHandle（测试不设置即可）：事件 emit 在设置后启用。

use std::collections::{HashMap, HashSet};
use std::sync::atomic::{AtomicBool, AtomicU64, AtomicUsize, Ordering};
use std::sync::LazyLock;

use parking_lot::Mutex;
use tokio::sync::mpsc;

use cc_box_remote_protocol::{ClientMessage, DownloadScope, RemoteSession, ServerMessage};

use crate::hook_events::{HookEventDetail, HookPayload};
use crate::rpc_dispatch::StateSection;

/// 每会话输出回放预算（字节）：分块淘汰整块。16KB 时代 Claude TUI 全屏
/// 重绘使回放几乎只剩一屏转义序列，attach 后历史不可见（remote/README.md
/// 原定 256KB ring buffer）
pub const TAIL_BUDGET: usize = 256 * 1024;

/// 小块并入尾块阈值（字节）
const TAIL_MERGE_THRESHOLD: usize = 64;

/// 每连接出站通道容量；快照帧（~256KB）不再与增量挤 256 槽
pub(crate) const CONN_CHANNEL_CAPACITY: usize = 1024;

/// 会话归属（per-session 控制权，唯一裁决层）：同一时间只有一端能操作某个
/// 终端。attach/claim 无条件抢占（后到优先，单用户多设备无协商）；不同连接
/// 可同时持有不同会话。离散写命令（rpc write）不做控制权互斥（靠白名单 +
/// revision 排序 + state-changed 广播收敛）
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
    tail: TailBuffer,
    seq: u64,
    active: ActiveView,
    /// 桌面端最近一次 resize 的尺寸（归还时恢复）
    desktop_size: Option<(u16, u16)>,
    /// Claude 会话 id（hook SessionStart 建立；会话列表下发给手机端防重复恢复）
    session_id: Option<String>,
}

/// 输出回放缓冲：分块 ring buffer。淘汰整块，新头部对齐 ANSI 转义边界——
/// 半截转义序列回放会把 xterm 已写内容抹掉（块边界不保证在转义边界上）
#[derive(Default)]
struct TailBuffer {
    chunks: std::collections::VecDeque<String>,
    total: usize,
}

impl TailBuffer {
    fn push(&mut self, s: &str) {
        // 小块并入尾块：远程逐键回显等小帧会积累数十万独立块，
        // String 头开销放大内存且 snapshot 拼接变慢
        if s.len() < TAIL_MERGE_THRESHOLD {
            if let Some(tail) = self.chunks.back_mut() {
                tail.push_str(s);
                self.total += s.len();
                self.enforce_budget();
                return;
            }
        }
        self.chunks.push_back(s.to_string());
        self.total += s.len();
        self.enforce_budget();
    }

    fn enforce_budget(&mut self) {
        while self.total > TAIL_BUDGET {
            if self.chunks.len() > 1 {
                // 前面有完整块：淘汰最旧块，新头部对齐转义边界
                if let Some(front) = self.chunks.pop_front() {
                    self.total -= front.len();
                }
                if let Some(head) = self.chunks.front_mut() {
                    let before = head.len();
                    align_head_to_ansi(head);
                    self.total -= before - head.len();
                }
            } else {
                // 只剩单块且超预算：块内截断保留尾部（UTF-8 边界 + 转义对齐）
                let Some(only) = self.chunks.front_mut() else { break };
                let mut keep_from = only.len().saturating_sub(TAIL_BUDGET);
                while keep_from < only.len() && !only.is_char_boundary(keep_from) {
                    keep_from += 1;
                }
                let removed = keep_from;
                only.drain(..keep_from);
                self.total -= removed;
                let before = only.len();
                align_head_to_ansi(only);
                self.total -= before - only.len();
                break;
            }
        }
    }

    fn snapshot(&self) -> String {
        let mut s = String::with_capacity(self.total);
        for c in &self.chunks {
            s.push_str(c);
        }
        s
    }
}

/// 头部对齐 ANSI 转义边界：丢弃首个 ESC(0x1b) 之前的残缺片段；无 ESC 保留
/// （纯文本；ESC 单字节，起点必是 UTF-8 边界）
fn align_head_to_ansi(s: &mut String) {
    if s.starts_with('\x1b') {
        return;
    }
    if let Some(pos) = s.find('\x1b') {
        s.drain(..pos);
    }
}

/// 连接表条目：出站通道 + 是否经中继（文件传输按连接类型限额——中继流量按量计费）
pub(crate) struct ConnEntry {
    pub tx: mpsc::Sender<ServerMessage>,
    pub via_relay: bool,
    /// 认证门闩：LAN 在连接层等值校验过 token（注册即 true）；中继连接须
    /// Hello 分支校验 token v2 通过后置 true。校验失败即 remove_conn，且
    /// 未认证期间非 Hello 消息一律忽略——恶意客户端可无视 hello.error
    /// 继续发消息（takeover → attach → pty.input），校验必须伴随断连+门闩
    pub authed: bool,
    /// 票据连接（`<device_id>.<ticket>` 认证）：只允许拉取该文件分块与心跳，
    /// 不支持会话/控制权等任何其他消息
    pub download_scope: Option<DownloadScope>,
}

#[derive(Default)]
struct StateInner {
    sessions: HashMap<String, SessionEntry>,
    /// pty_id → 订阅它的连接集合（attach 同时视为订阅）
    subscribers: HashMap<String, HashSet<u64>>,
    /// conn_id → 出站通道
    conns: HashMap<u64, ConnEntry>,
    /// 全局状态版本号：每次状态变化（写命令广播/会话建关）+1。
    /// 附在 state-changed payload 与 HelloOk 下发，前端单调过滤防旧事件
    /// 乱序覆盖新状态；重连后以服务端值为基线
    revision: u64,
}

/// 票据受限连接的放行规则：只允许拉取该文件分块与心跳
pub(crate) fn ticket_conn_allows(msg: &ClientMessage) -> bool {
    matches!(msg, ClientMessage::Ping)
        || matches!(msg, ClientMessage::RpcCall { cmd, .. } if cmd == "file_download_ticket_chunk")
}

pub struct RemoteState {
    inner: Mutex<StateInner>,
}

pub static REMOTE_STATE: LazyLock<RemoteState> =
    LazyLock::new(|| RemoteState { inner: Mutex::new(StateInner::default()) });

static NEXT_CONN_ID: AtomicU64 = AtomicU64::new(1);
static NEXT_SEQ: AtomicU64 = AtomicU64::new(1);

/// Broker 模式：本进程没有 PTY，禁止本地 spawn/kill 会话。
static BROKER_MODE: AtomicBool = AtomicBool::new(false);

pub(crate) fn set_broker_mode(enabled: bool) {
    BROKER_MODE.store(enabled, Ordering::SeqCst);
}

pub(crate) fn is_broker() -> bool {
    BROKER_MODE.load(Ordering::SeqCst)
}

/// 成员状态上行通道（SessionUpsert/SessionRemove）。
/// **无界**：状态帧数量受会话数约束，可靠优先于内存上限；输出流不走这里，
/// 由每连接的 bridge 单独限流并带 seq 缺口标记。
static UPLINK: LazyLock<Mutex<Option<mpsc::UnboundedSender<crate::gateway_proto::GwUp>>>> =
    LazyLock::new(|| Mutex::new(None));

/// 设置/清除成员状态上行通道（成员注册成功/断开时）
pub(crate) fn set_uplink(tx: Option<mpsc::UnboundedSender<crate::gateway_proto::GwUp>>) {
    *UPLINK.lock() = tx;
}

pub(crate) fn uplink_send(up: crate::gateway_proto::GwUp) {
    let guard = UPLINK.lock();
    if let Some(tx) = guard.as_ref() {
        if tx.send(up).is_err() {
            log::debug!("[remote] member uplink closed, state frame dropped");
        }
    }
}

/// 当前进程是否以 Broker 成员身份注册（有上行通道）
fn has_uplink() -> bool {
    UPLINK.lock().is_some()
}

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

/// 清除事件回调（测试收尾用，避免跨测试污染）
#[cfg(test)]
pub fn clear_event_hook() {
    *EVENT_HOOK.lock() = None;
}

/// 桌面通知 + 远程广播（payload 与 Tauri 事件同构 {id, data}）。
/// 事件单一出口：emit_event / emit_event_ex / emit_desktop_only 三变体按受众选择。
/// （pub：commands::rename_session 等桌面 command 也经此双发）
pub fn emit_event(name: &str, id: &str, data: &str) {
    emit_via_hook(name, id, data);
    REMOTE_STATE.broadcast_event(name, serde_json::json!({ "id": id, "data": data }));
}

/// 写命令落盘后的统一状态广播（commands.rs 写命令成功路径单点调用）：
/// 全端收到 `state-changed {id: section, data: revision}` 后按 section 权威刷新。
/// revision 随广播单调递增，前端过滤旧事件防乱序覆盖。
/// 注册表见 rpc_dispatch::write_section（守卫测试强制登记与实现）。
pub fn emit_state_changed(section: StateSection) {
    REMOTE_STATE.emit_state_changed(section);
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

/// Broker 扇出的桌面事件（DesktopEvent）直接投递到本地 Tauri EventHook，
/// 不再进入 remote_state 广播（避免回环/重复）。
pub(crate) fn fire_event_hook(name: &str, id: &str, data: &str) {
    emit_via_hook(name, id, data);
}

impl RemoteState {
    /// 构造独立实例（单元测试用；生产代码使用全局 REMOTE_STATE）
    pub(crate) fn new() -> Self {
        RemoteState { inner: Mutex::new(StateInner::default()) }
    }

    /// 当前状态版本（HelloOk / sessions.list 快照用）
    pub fn revision(&self) -> u64 {
        self.inner.lock().revision
    }

    // ==================== 会话归属（per-session 控制权） ====================

    /// 成员重连/断连清理：所有远端接管的会话归还桌面、清空订阅索引。
    pub(crate) fn reset_remote_views(&self) {
        let ids = {
            let mut inner = self.inner.lock();
            inner.subscribers.clear();
            let mut ids = Vec::new();
            for (id, entry) in inner.sessions.iter_mut() {
                if matches!(entry.active, ActiveView::Remote(_)) {
                    entry.active = ActiveView::Desktop;
                    ids.push(id.clone());
                }
            }
            ids
        };
        for id in ids {
            emit_event("session-reclaimed", &id, "");
            resize_pty(&id, self.desktop_size_of(&id));
            log::info!("[remote] session {} reset to desktop (member reconnect)", id);
        }
    }

    /// state-changed 广播（实例方法：单元测试用独立实例验证广播半链）。
    /// 桌面 hook emit + 全体远程连接广播，payload {id: section, data: revision}。
    /// 取号与入队在同一锁窗口（保序：并发时低 revision 事件若后入队，
    /// 会被前端 filterStaleEvent 当旧事件永久丢弃）
    pub fn emit_state_changed(&self, section: StateSection) {
        let name = crate::events::names::STATE_CHANGED;
        let section = section.as_str();
        let rev = {
            let mut inner = self.inner.lock();
            inner.revision += 1;
            let rev = inner.revision;
            let msg = ServerMessage::RpcEvent {
                name: name.to_string(),
                payload: serde_json::json!({ "id": section, "data": rev.to_string() }),
            };
            for e in inner.conns.values() {
                if e.download_scope.is_some() {
                    continue; // 票据受限连接不接收状态广播
                }
                let _ = e.tx.try_send(msg.clone());
            }
            rev
        };
        // 锁外调用 hook：Broker 的事件 hook 会取 GW_STATE，若在 inner 锁内调用
        // 会与 unregister_member 的 GW_STATE→inner 形成 ABBA 死锁
        emit_via_hook(name, section, &rev.to_string());
        // 成员模式：上行给 Broker，由 Broker 重新分配设备级 revision 后
        // 扇出给其他 GUI 与全部手机。Broker 模式下 UPLINK 为 None，安全 no-op。
        uplink_send(crate::gateway_proto::GwUp::StateChanged {
            section: section.to_string(),
        });
    }

    /// Broker 为成员上报的 state-changed 重新分配设备级 revision：
    /// 成员进程的 revision 与 Broker 不可比，直接透传会被手机按旧事件丢弃。
    pub(crate) fn next_revision(&self) -> u64 {
        let mut inner = self.inner.lock();
        inner.revision += 1;
        inner.revision
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
                tail: TailBuffer::default(),
                seq: NEXT_SEQ.fetch_add(1, Ordering::Relaxed),
                active: ActiveView::Desktop,
                desktop_size: None,
                session_id: None,
            },
        );
        drop(inner);
        // 成员模式：向 Gateway 上报新会话（手机聚合列表刷新依据）
        uplink_send(crate::gateway_proto::GwUp::SessionUpsert {
            session: self.session_snapshot(id),
        });
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
        // 成员模式由 Broker 统一广播 SessionExit（见 SessionRemove）；
        // 本地再广播会经 ToConn 让桥接手机收到两次。
        if !has_uplink() {
            self.broadcast_to_all(ServerMessage::SessionExit { id: id.to_string() });
        }
        uplink_send(crate::gateway_proto::GwUp::SessionRemove { id: id.to_string() });
    }

    /// 输出 tee（pty.rs 读取循环调用）：更新 tail 并转发给订阅者
    pub fn push_output(&self, id: &str, data: &str) {
        if data.is_empty() {
            return;
        }
        let seq = {
            let mut inner = self.inner.lock();
            match inner.sessions.get_mut(id) {
                Some(entry) => {
                    entry.tail.push(data);
                    entry.seq = entry.seq.wrapping_add(1);
                    entry.seq
                }
                None => return,
            }
        };
        self.broadcast_to_subscribers(
            id,
            ServerMessage::PtyOutput {
                id: id.to_string(),
                data: data.to_string(),
                seq,
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

    /// 记录会话 id（hook_server 收到 SessionStart 等 hook 事件时调用；幂等）
    pub fn set_session_id(&self, pty_id: &str, session_id: &str) {
        let changed = {
            let mut inner = self.inner.lock();
            match inner.sessions.get_mut(pty_id) {
                Some(entry) => {
                    if entry.session_id.as_deref() != Some(session_id) {
                        entry.session_id = Some(session_id.to_string());
                        true
                    } else {
                        false
                    }
                }
                None => false,
            }
        };
        if changed {
            // 成员模式：sessionId 建立后上报（手机端防二次 spawn 的关联依据）
            uplink_send(crate::gateway_proto::GwUp::SessionUpsert {
                session: self.session_snapshot(pty_id),
            });
        }
    }

    /// 单会话快照（uplink 上报用；不存在时 None）
    fn session_snapshot(&self, id: &str) -> cc_box_remote_protocol::RemoteSession {
        let inner = self.inner.lock();
        let entry = inner.sessions.get(id);
        cc_box_remote_protocol::RemoteSession {
            id: id.to_string(),
            pty_type: entry
                .map(|e| e.pty_type.clone())
                .unwrap_or_default(),
            cwd: entry.map(|e| e.cwd.clone()).unwrap_or_default(),
            working: entry.map(|e| e.working).unwrap_or(false),
            pending: entry.map(|e| e.pending).unwrap_or(false),
            session_id: entry.and_then(|e| e.session_id.clone()),
            instance_id: None,
            instance_name: None,
        }
    }

    /// hook 事件 → working/pending 状态机（唯一实现，以 pty_id 为键）：
    /// 返回计算后的状态，由调用方（hook_server）以 hook-status 事件双端下发；
    /// SessionStatus 广播（手机列表页圆点）与本返回值同源同值
    pub fn on_hook_payload(&self, payload: &HookPayload) -> Option<HookStatus> {
        let Some(pty_id) = &payload.pty_id else { return None };
        let (status_msg, status) = {
            let mut inner = self.inner.lock();
            let Some(entry) = inner.sessions.get_mut(pty_id) else { return None };
            let before = (entry.working, entry.pending);
            apply_hook_event(entry, &payload.detail);
            let after = (entry.working, entry.pending);
            let msg = if before != after {
                Some(ServerMessage::SessionStatus {
                    id: pty_id.clone(),
                    working: after.0,
                    pending: after.1,
                })
            } else {
                None
            };
            (
                msg,
                HookStatus {
                    pty_id: pty_id.clone(),
                    working: after.0,
                    pending: after.1,
                },
            )
        };
        if let Some(msg) = status_msg {
            // 广播给全部连接而非仅订阅者：列表页 working/pending 圆点也要实时变化（R1-2）
            self.broadcast_to_all(msg.clone());
            // 成员模式：状态变化上报 Gateway 缓存（聚合列表下次拉取时一致）
            uplink_send(crate::gateway_proto::GwUp::SessionUpsert {
                session: self.session_snapshot(pty_id),
            });
        }
        Some(status)
    }

    /// 连接注册（remote_server 直连传 false / remote_client 中继桥接传 true）。
    /// LAN 在连接层已校验 token（authed 直接置 true）；中继连接待 Hello 校验
    pub fn add_conn(&self, conn_id: u64, sender: mpsc::Sender<ServerMessage>, via_relay: bool) {
        self.inner.lock().conns.insert(
            conn_id,
            ConnEntry {
                tx: sender,
                via_relay,
                authed: !via_relay,
                download_scope: None,
            },
        );
    }

    /// 成员桥连接注册（Gateway 转发来的手机连接）：conn_id 透传，authed 直置
    /// true（认证已在 Gateway 完成）；via_relay 透传保证文件限额档位正确。
    /// 注册后所有广播（PtyOutput/SessionStatus/RpcEvent 等）自动发到桥通道，
    /// 由成员桥任务封 ToConn 上行 Gateway——广播路径零改动
    pub fn add_member_conn(&self, conn_id: u64, sender: mpsc::Sender<ServerMessage>, via_relay: bool) {
        self.inner.lock().conns.insert(
            conn_id,
            ConnEntry {
                tx: sender,
                via_relay,
                authed: true,
                download_scope: None,
            },
        );
    }

    /// 测试辅助：把连接标记为票据受限会话（验证广播隔离）
    #[cfg(test)]
    pub(crate) fn test_mark_download_scope(&self, conn_id: u64, scope: DownloadScope) {
        if let Some(e) = self.inner.lock().conns.get_mut(&conn_id) {
            e.download_scope = Some(scope);
            e.authed = true;
        }
    }

    /// 连接是否经中继（文件传输限额选档；未知连接返回 None，调用方按中继从严处理）
    pub fn conn_via_relay(&self, conn_id: u64) -> Option<bool> {
        self.inner.lock().conns.get(&conn_id).map(|e| e.via_relay)
    }

    /// 连接断开后清理全部订阅，并归还其接管的会话
    pub fn remove_conn(&self, conn_id: u64) {
        // 文件传输会话随连接回收（删 .part，防僵死会话占满配额）
        crate::file_transfer::cleanup_conn(conn_id);
        // Gateway 侧：清 rpc 粘连路由（file_upload 后续命令不再跟随）
        crate::gateway::conn_dropped(conn_id);
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
        // 多实例：通知成员清桥（成员侧 remove_conn 自带归还 + 传输会话回收）。
        // 成员进程自身调用时 members 为空，广播为 no-op
        crate::gateway::broadcast_down(crate::gateway_proto::GwDown::ConnDrop { conn_id });
    }

    fn desktop_size_of(&self, id: &str) -> Option<(u16, u16)> {
        self.inner.lock().sessions.get(id).and_then(|e| e.desktop_size)
    }

    /// ptyId → 项目 cwd（文件上传定位会话项目的 .cc-box-files 用）
    pub fn cwd_of(&self, id: &str) -> Option<String> {
        self.inner.lock().sessions.get(id).map(|e| e.cwd.clone())
    }

    /// 该会话控制权当前是否在远程连接手中（桌面 resize 应让位，仅记录尺寸）
    pub fn is_remote_active(&self, id: &str) -> bool {
        let inner = self.inner.lock();
        matches!(
            inner.sessions.get(id).map(|e| &e.active),
            Some(ActiveView::Remote(_))
        )
    }

    /// 桌面端"接管回来"（Tauri 命令 pty_claim 调用）：无条件收回该终端的
    /// active 归属（per-session 抢占，单用户多设备无协商），通知旧远程持有者
    pub fn claim(&self, id: &str) {
        let old = {
            let mut inner = self.inner.lock();
            let Some(entry) = inner.sessions.get_mut(id) else { return };
            std::mem::replace(&mut entry.active, ActiveView::Desktop)
        };
        if let ActiveView::Remote(old_conn) = old {
            // 只收回控制权、不摘订阅：旧远程保留输出流，显示 taken-over 遮罩
            // 可随时重新 attach 抢回。发 SessionDetached 会让对方把 Tab 置
            // stopped（表现为"断开但仍见内容"），语义错误。
            self.try_send(
                old_conn,
                ServerMessage::RpcEvent {
                    name: "session-taken-over".to_string(),
                    payload: serde_json::json!({ "id": id, "data": "" }),
                },
            );
        }
        // 仅桌面需要清遮罩；广播 reclaimed 会把刚发给旧远程的 taken-over
        // 又清掉（同一连接队列后到者生效）
        emit_desktop_only("session-reclaimed", id, "");
        resize_pty(id, self.desktop_size_of(id));
    }

    /// 应用层消息分发（认证后的消息，局域网/中继同一条路径）
    /// 连接级消息门闩：除 Hello（认证本身）外，未知/未认证连接一律拒绝；
    /// 票据受限连接只放行该文件的下载分块与心跳。handler 与 Gateway 转发
    /// 路径共用（转发到成员前必须先过同一道门，否则未认证中继连接可借成员
    /// 桥的 authed 直置绕过认证）
    pub fn conn_message_allowed(&self, conn_id: u64, msg: &ClientMessage) -> bool {
        if matches!(msg, ClientMessage::Hello { .. }) {
            return true;
        }
        let inner = self.inner.lock();
        let Some(e) = inner.conns.get(&conn_id) else {
            return false;
        };
        if !e.authed {
            return false;
        }
        if e.download_scope.is_some() && !ticket_conn_allows(msg) {
            return false;
        }
        true
    }

    pub fn handle_client_message(&self, conn_id: u64, msg: &ClientMessage) {
        // 认证门闩（takeover/attach 等分支不查 conns 表，无门闩则可被盲注入）
        if !self.conn_message_allowed(conn_id, msg) {
            return;
        }
        // Broker 没有 PTY：会话级消息只能由 gateway::dispatch 路由到成员。
        // 到达本地说明路由表缺失/代码遗漏——显式拒绝，绝不落到空 PTY manager。
        if is_broker() {
            match msg {
                ClientMessage::SessionCreate { .. } => {
                    self.try_send(
                        conn_id,
                        ServerMessage::SessionCreateFailed {
                            error: "no GUI instance available".to_string(),
                        },
                    );
                    return;
                }
                ClientMessage::SessionKill { .. }
                | ClientMessage::SessionAttach { .. }
                | ClientMessage::SessionDetach { .. }
                | ClientMessage::SessionSubscribe { .. }
                | ClientMessage::SessionUnsubscribed { .. }
                | ClientMessage::PtyInput { .. }
                | ClientMessage::PtyResize { .. } => {
                    log::warn!(
                        "[remote] broker received un-routed session message for conn {}",
                        conn_id
                    );
                    return;
                }
                _ => {}
            }
        }
        match msg {
            // hello 的 token 认证：LAN 由连接层（remote_server）完成；中继链路
            // token v2 由本端校验（relay 只按前缀路由不校验）——校验失败经
            // client.kick 让 relay 下发 hello.error 并断连（恶意客户端可无视
            // 错误继续发消息，必须门闩 + 真断连）
            ClientMessage::Hello { token, .. } => {
                let mut download_scope = None;
                // via_relay：None（未知连接）按中继从严处理
                if self.conn_via_relay(conn_id) != Some(false) {
                    // 先按普通 token v2 校验；失败再看是否是下载票据
                    // （`<device_id>.<ticket>`，relay 只按前缀路由不校验）
                    if !crate::remote_client::verify_relay_token(token) {
                        match crate::remote_client::verify_download_ticket(token) {
                            Some(scope) => download_scope = Some(scope),
                            None => {
                                crate::remote_client::kick_relay_client(
                                    conn_id,
                                    "invalid token",
                                );
                                self.remove_conn(conn_id);
                                return;
                            }
                        }
                    }
                }
                let revision = {
                    let mut inner = self.inner.lock();
                    if let Some(e) = inner.conns.get_mut(&conn_id) {
                        e.authed = true;
                        e.download_scope = download_scope.clone();
                    }
                    inner.revision
                };
                self.try_send(
                    conn_id,
                    ServerMessage::HelloOk {
                        ver: cc_box_remote_protocol::PROTOCOL_VERSION,
                        device_name: device_name(),
                        conn_id,
                        // 控制权已 per-session 化：恒 null（字段保留兼容旧端缓存 UI）
                        controller: None,
                        revision,
                        download_scope,
                    },
                );
            }
            ClientMessage::SessionsList => {
                // 多实例聚合：Gateway 侧合并本机与成员实例的会话（带实例分组
                // 字段）；单实例/无成员时输出与本机列表一致（实例字段照填）
                let list = crate::gateway::merged_session_list(self.session_list());
                let revision = self.revision();
                self.try_send(conn_id, ServerMessage::Sessions { list, revision });
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
            ClientMessage::SessionCreate { path, session_id, skip_permissions, custom_args, session_name, .. } => {
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
                let session_name = session_name.clone().unwrap_or_default();
                let cli_args = crate::pty::build_claude_args(
                    session_id.as_deref(),
                    skip_permissions.unwrap_or(false),
                    custom_args.as_deref().unwrap_or(""),
                );
                let args = (!cli_args.is_empty()).then_some(cli_args);
                let conn_tx = self.inner.lock().conns.get(&conn_id).map(|e| e.tx.clone());
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
                    // spawn 阻塞（含子进程启动等待）
                    // 包装走 commands::spawn_pty_core 单一实现（与桌面 pty_spawn
                    // 共用），参数拼接走 pty::build_claude_args 单一核心
                    let spawned = crate::commands::spawn_pty_core(crate::commands::PtySpawnOptions {
                        cwd: path.clone(),
                        pty_type: "claude".to_string(),
                        cols: Some(80),
                        rows: Some(24),
                        args,
                    });
                    match spawned {
                        Ok(info) => {
                            // spawn 内部已 register_pty；通知桌面建 tab 绑定该 PTY
                            // data 为 JSON（cwd 含 \t 等特殊字符时 \t 拼接会截断）；
                            // name 为恢复会话的显示名（历史列表提供，桌面建 Tab 防编码名）
                            let data = serde_json::json!({
                                "cwd": info.cwd,
                                "resume": resume,
                                "name": session_name,
                            })
                            .to_string();
                            emit_desktop_only("remote-session-created", &info.id, &data);
                            reply(ServerMessage::SessionCreated {
                                id: info.id,
                                path: info.cwd,
                            });
                        }
                        Err(e) => {
                            log::warn!("[remote] session create failed for {}: {}", path, e);
                            reply(ServerMessage::SessionCreateFailed {
                                error: e,
                            });
                        }
                    }
                });
            }
            ClientMessage::SessionsHistory { path } => {
                // 白名单同 session.create（未知项目回空列表，手机 UI 不报错）；
                // is_known_project 与 get_sessions 都涉及目录扫描，放线程执行。
                // 有意不走 RPC get_sessions：远程需白名单 + 固定条数守卫，
                // RPC 层无会话粒度过滤点（迁移等于放宽守卫）；底层 store 数据源已单一
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
            ClientMessage::SessionKill { id, quiet } => {
                // 用户显式关闭 = 各端联动删 Tab（桌面 closeTab / 其他浏览器端
                // 均从"打开的标签页"移除）。先 emit 后 kill：manager.kill 内部会
                // 先 emit pty-exit（前端收到即清 tab.ptyId 置 stopped），若本事件
                // 晚到将匹配不到 Tab，各端残留"断开"状态的 Tab；先发保证先执行
                // 删 Tab（随后到达的 pty-exit 因 Tab 已删而 no-op）。closeTab
                // 内部的异步 ptyKill 与下方 kill 并发，manager.kill 持锁幂等。
                // remove_pty 链路（SessionExit）自动通知手机列表移除。
                // quiet=true（重启场景内部清理，与桌面 pty_kill quiet 同语义）：
                // 跳过 closed 广播——各端保留 Tab 等 respawn，pty-exit 只置 stopped
                let id = id.clone();
                let quiet = quiet.unwrap_or(false);
                std::thread::spawn(move || {
                    if !quiet {
                        emit_event(crate::events::names::REMOTE_SESSION_CLOSED, &id, "");
                    }
                    if let Some(manager) = crate::pty::get_pty_manager() {
                        if let Err(e) = manager.kill(&id) {
                            log::warn!("[remote] session kill {} failed: {}", id, e);
                        }
                    }
                });
            }
            ClientMessage::SessionSubscribe { id } => {
                // 快照先于增量入队（与 handle_attach 同一时序保证）：
                // 锁内先发 SessionSubscribed，再插入订阅者
                let queued = {
                    let mut inner = self.inner.lock();
                    let tail = inner
                        .sessions
                        .get_mut(id)
                        .map(|e| (e.tail.snapshot(), e.seq));
                    match tail {
                        Some((tail, tail_seq)) => {
                            let ok = send_locked(
                                &inner,
                                conn_id,
                                ServerMessage::SessionSubscribed {
                                    id: id.clone(),
                                    tail,
                                    tail_seq,
                                },
                            );
                            inner
                                .subscribers
                                .entry(id.clone())
                                .or_default()
                                .insert(conn_id);
                            Some(ok)
                        }
                        None => None,
                    }
                };
                match queued {
                    Some(true) => {}
                    Some(false) => {
                        // 快照入队失败：断开慢消费者，重连重拉
                        log::warn!(
                            "[remote] subscribe snapshot queue failed for conn {}",
                            conn_id
                        );
                        self.remove_conn(conn_id);
                    }
                    None => {
                        // 会话不存在：以 exit 告知，客户端按已退出处理
                        self.try_send(conn_id, ServerMessage::SessionExit { id: id.clone() });
                    }
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
                // attach 即抢占式接管（handle_attach）：不同连接可同时 attach
                // 不同会话；同一会话后到者优先，旧持有者收 taken-over 遮罩
                self.handle_attach(conn_id, id, *cols, *rows);
            }
            // 全局控制权已 per-session 化（attach/claim 即会话级抢占）。
            // 旧版手机缓存 UI 仍会发送本消息——no-op 静默忽略（删 variant 会让
            // 旧端整帧反序列化失败），不产生任何状态变化
            ClientMessage::ControllerTakeover => {
                log::debug!("[remote] legacy controller.takeover ignored (conn {})", conn_id);
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
                // 会话级裁决：仅该会话的 active 持有者可写入（被抢占/断连归还后
                // active 不再匹配，旧持有者输入自然被拒）
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
            let old_and_tail = inner.sessions.get_mut(id).map(|entry| {
                let old = std::mem::replace(&mut entry.active, ActiveView::Remote(conn_id));
                // 打开过即已读：attach = 用户查看该会话，pending（等待用户
                // 关注）清 false——否则服务端持续为 true，浏览器端会话面板
                // 打开时 sessions.list 会把本地已清的"已读"冲回金黄。
                // working 是任务实时状态，不代表"待查看"，不清
                let pending_cleared = entry.pending;
                if entry.pending {
                    entry.pending = false;
                }
                (old, entry.tail.snapshot(), entry.seq, pending_cleared)
            });
            let Some((old, tail, tail_seq, pending_cleared)) = old_and_tail else {
                drop(inner);
                self.try_send(conn_id, ServerMessage::SessionExit { id: id.to_string() });
                return;
            };
            // 快照必须先于一切增量入队：在同一锁窗口内、插入 subscribers 之前
            // 发送（push_output 的广播也持此锁，时序必然在快照之后）
            let snapshot_queued = send_locked(
                &inner,
                conn_id,
                ServerMessage::SessionAttached {
                    id: id.to_string(),
                    cols,
                    rows,
                    tail,
                    tail_seq,
                },
            );
            // attach 同时视为输出订阅者（旧订阅者保留，与桌面 claim 语义一致）
            let subs = inner.subscribers.entry(id.to_string()).or_default();
            subs.insert(conn_id);
            AttachOutcome { old, snapshot_queued, pending_cleared }
        };

        // 快照入队失败（通道满/已关）：主动断开，客户端重连重拉快照
        // （好过静默黑屏；否则订阅者将持续收增量而永远缺历史）
        if !outcome.snapshot_queued {
            log::warn!(
                "[remote] attach snapshot queue failed for conn {}, dropping connection",
                conn_id
            );
            self.remove_conn(conn_id);
            return;
        }

        // pending 已读清除：双端广播 session-viewed（发起端本地 setActiveTab
        // 已清、且被排除，受众是其他浏览器端与桌面 Tab 状态点；payload 与
        // Tauri 事件同构 {id, data}，桌面经 EVENT_HOOK 转发）
        if outcome.pending_cleared {
            emit_via_hook(crate::events::names::SESSION_VIEWED, id, "");
            self.broadcast_event_excluding(
                crate::events::names::SESSION_VIEWED,
                serde_json::json!({ "id": id, "data": "" }),
                Some(conn_id),
            );
            // 成员实例：pending 变化立即上报 Gateway 聚合缓存，否则手机端
            // 列表的待关注圆点要等下一次 hook/status 上报才消失
            if let Some(s) = self.session_list().into_iter().find(|s| s.id == *id) {
                uplink_send(crate::gateway_proto::GwUp::SessionUpsert { session: s });
            }
        }

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
                        session_id: e.session_id.clone(),
                        // 多实例聚合由 Gateway 侧 merged_session_list 填充（P3）；
                        // 此处 None = 旧桌面同构输出（手机端降级平铺）
                        instance_id: None,
                        instance_name: None,
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
                if let Some(e) = inner.conns.get(conn_id) {
                    // 票据受限连接不接收任何广播（纵深防御：当前 attach 已被门闩拦截）
                    if e.download_scope.is_some() {
                        continue;
                    }
                    let _ = e.tx.try_send(msg.clone());
                }
            }
        }
    }

    /// 广播给全部在线连接（会话列表级信息：exit、working/pending 状态）。
    /// 票据受限连接不接收任何广播——它们只允许文件分块与心跳，
    /// 不该拿到会话/项目/配置等元数据
    pub(crate) fn broadcast_to_all(&self, msg: ServerMessage) {
        let inner = self.inner.lock();
        for e in inner.conns.values() {
            if e.download_scope.is_some() {
                continue;
            }
            let _ = e.tx.try_send(msg.clone());
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

    /// 定向事件（单个连接）：实例名单等按需补发，避免广播噪声
    pub fn send_event_to(&self, conn_id: u64, name: &str, payload: serde_json::Value) {
        let inner = self.inner.lock();
        if let Some(e) = inner.conns.get(&conn_id) {
            let _ = e.tx.try_send(ServerMessage::RpcEvent {
                name: name.to_string(),
                payload,
            });
        }
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
        for (conn_id, e) in inner.conns.iter() {
            if Some(*conn_id) == exclude || e.download_scope.is_some() {
                continue;
            }
            let _ = e.tx.try_send(msg.clone());
        }
    }

    pub(crate) fn try_send(&self, conn_id: u64, msg: ServerMessage) {
        let inner = self.inner.lock();
        if let Some(tx) = inner.conns.get(&conn_id).map(|e| &e.tx) {
            let _ = tx.try_send(msg);
        }
    }

    /// 成员上行消息投递给手机（非阻塞）：
    /// - `PtyOutput` 可丢（成员侧已按 headroom 丢弃并保留真实 seq，客户端检测缺口）；
    /// - 其余可靠消息入队失败（队列满/已关）→ 断开该手机（浏览器重连后全量
    ///   拉取），绝不静默丢弃 attach 应答/RPC 结果/创建结果。
    ///
    /// 这里刻意不做 `send().await`：该函数在成员 IPC 读循环内联调用，等待慢
    /// 手机会队头阻塞整个成员的上行（SessionUpsert/心跳/其他连接应答），
    /// 最终把成员拖成重连。断开慢手机是显式、可恢复的。
    pub(crate) fn deliver_to_conn(&self, conn_id: u64, msg: ServerMessage) {
        let tx = {
            let inner = self.inner.lock();
            inner.conns.get(&conn_id).map(|e| e.tx.clone())
        };
        let Some(tx) = tx else {
            return;
        };
        if matches!(msg, ServerMessage::PtyOutput { .. }) {
            if tx.try_send(msg).is_err() {
                log::debug!(
                    "[remote] pty output dropped for conn {} (client backpressure)",
                    conn_id
                );
            }
            return;
        }
        if tx.try_send(msg).is_err() {
            log::warn!(
                "[remote] reliable delivery to conn {} failed; dropping connection",
                conn_id
            );
            self.remove_conn(conn_id);
        }
    }
}

struct AttachOutcome {
    old: ActiveView,
    snapshot_queued: bool,
    /// attach 时清掉了 pending（等待用户关注），需广播 session-viewed
    pending_cleared: bool,
}

/// 持锁发送（快照顺序保证路径）：与 push_output 的广播共用同一把锁，
/// 保证快照先于增量入队。返回是否成功入队
fn send_locked(inner: &StateInner, conn_id: u64, msg: ServerMessage) -> bool {
    match inner.conns.get(&conn_id).map(|e| &e.tx) {
        Some(tx) => tx.try_send(msg).is_ok(),
        None => false,
    }
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

/// hook-status 事件载荷（on_hook_payload 计算后返回；hook_server 双端下发，
/// 桌面/手机前端的 Tab working/pending 唯一状态来源）
pub struct HookStatus {
    pub pty_id: String,
    pub working: bool,
    pub pending: bool,
}

/// working/pending 状态机（唯一实现，前端不再复刻）：
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
