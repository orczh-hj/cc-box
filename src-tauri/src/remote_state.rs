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
use crate::rpc_dispatch::StateSection;

/// 每会话输出回放预算（字节）：分块淘汰整块。16KB 时代 Claude TUI 全屏
/// 重绘使回放几乎只剩一屏转义序列，attach 后历史不可见（remote/README.md
/// 原定 256KB ring buffer）
pub const TAIL_BUDGET: usize = 256 * 1024;

/// 小块并入尾块阈值（字节）
const TAIL_MERGE_THRESHOLD: usize = 64;

/// 每连接出站通道容量；快照帧（~256KB）不再与增量挤 256 槽
pub(crate) const CONN_CHANNEL_CAPACITY: usize = 1024;

/// 控制权持有者：同一时间只有一端能操作某个终端（产品决策）
#[derive(Debug, Clone, PartialEq)]
pub enum ActiveView {
    Desktop,
    Remote(u64),
}

/// 全局控制权持有者：桌面与所有远程设备对等参与互斥（产品决策：
/// 同一时间只有一个设备操作，任何被遮端点「接管」即可开始操作）。
/// 桌面获得控制权仅通过显式接管（ControllerMask 接管按钮 / 终端接管回来），
/// 桌面普通操作不抢占远程（远程普通写操作在无人持有时自动持有）
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ControllerHolder {
    Desktop,
    Remote(u64),
}

impl ControllerHolder {
    /// hello.ok / controller-changed 的 id 表示：桌面 = 0（conn_id 从 1 起，
    /// 0 不与远程连接冲突）；远程 = conn_id
    pub fn as_id(self) -> u64 {
        match self {
            ControllerHolder::Desktop => 0,
            ControllerHolder::Remote(c) => c,
        }
    }

    pub(crate) fn id_label(self) -> String {
        match self {
            ControllerHolder::Desktop => "desktop".to_string(),
            ControllerHolder::Remote(c) => c.to_string(),
        }
    }
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
}

#[derive(Default)]
struct StateInner {
    sessions: HashMap<String, SessionEntry>,
    /// pty_id → 订阅它的连接集合（attach 同时视为订阅）
    subscribers: HashMap<String, HashSet<u64>>,
    /// conn_id → 出站通道
    conns: HashMap<u64, ConnEntry>,
    /// 全局状态版本号：每次状态变化（写命令广播/会话建关/控制权变更）+1。
    /// 附在 state-changed/controller-changed payload 与 HelloOk 下发，前端
    /// 单调过滤防旧事件乱序覆盖新状态；重连/接管后以服务端值为基线
    revision: u64,
    /// 全局控制权持有者（None=无人持有）。设备互斥（桌面与远程对等）：
    /// - 远程写操作（rpc 写命令/session.create/kill/attach）无人持有时自动持有
    /// - 他人（含桌面）持有时远程写操作被服务端拒绝；attach 不能抢占
    /// - 转移仅显式接管：远程 controller.takeover / 桌面 controller_takeover
    ///   command（ControllerMask 接管按钮）与 pty_claim（终端接管回来）
    /// - takeover 剥离旧持有者全部终端归属；断连自动释放
    controller: Option<ControllerHolder>,
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

impl RemoteState {
    /// 构造独立实例（单元测试用；生产代码使用全局 REMOTE_STATE）
    pub(crate) fn new() -> Self {
        RemoteState { inner: Mutex::new(StateInner::default()) }
    }

    /// 当前状态版本（HelloOk / sessions.list 快照用）
    pub fn revision(&self) -> u64 {
        self.inner.lock().revision
    }

    // ==================== 全局控制权（设备互斥） ====================

    /// 当前控制者（None=无人持有；HelloOk 下发前端判定遮罩，Desktop=0）
    pub fn controller(&self) -> Option<ControllerHolder> {
        self.inner.lock().controller
    }

    /// 远程写操作准入：他人（含桌面）持有 → Err(持有者)；无人/自己 → Ok。
    /// 只做检查不改变状态（持有由 write_success_acquire / attach 完成）
    pub(crate) fn controller_gate(&self, conn_id: u64) -> Result<(), ControllerHolder> {
        match self.controller() {
            Some(ControllerHolder::Remote(c)) if c == conn_id => Ok(()),
            Some(other) => Err(other),
            None => Ok(()),
        }
    }

    /// 无人持有时自动持有并广播（远程写操作成功后调用；已持有幂等）。
    /// 返回是否发生了持有变化
    pub(crate) fn write_success_acquire(&self, conn_id: u64) -> bool {
        let changed = {
            let mut inner = self.inner.lock();
            if inner.controller.is_some() {
                false
            } else {
                inner.controller = Some(ControllerHolder::Remote(conn_id));
                true
            }
        };
        if changed {
            self.broadcast_controller_changed();
        }
        changed
    }

    /// 广播 controller-changed（payload {id: "desktop"|connId|空串, data: revision}）：
    /// 桌面 Tauri emit（桌面 ControllerMask 遮罩判定）+ 远程 WS 广播；
    /// 各端据此显示/解除"被其他设备控制中"全局遮罩；revision 随之推进。
    /// 取号与入队在同一锁窗口（保序）：跨线程与 state-changed 并发时若分属
    /// 两个锁窗口，低 revision 事件可能后入队，被前端 filterStaleEvent
    /// 当旧事件永久丢弃（遮罩/刷新状态丢失）
    fn broadcast_controller_changed(&self) {
        let name = crate::events::names::CONTROLLER_CHANGED;
        let mut inner = self.inner.lock();
        inner.revision += 1;
        let rev = inner.revision;
        let id = inner.controller.map(|c| c.id_label()).unwrap_or_default();
        emit_via_hook(name, &id, &rev.to_string());
        let msg = ServerMessage::RpcEvent {
            name: name.to_string(),
            payload: serde_json::json!({ "id": id, "data": rev.to_string() }),
        };
        for e in inner.conns.values() {
            let _ = e.tx.try_send(msg.clone());
        }
    }

    /// 接管核心（远程 takeover 与桌面 controller_takeover 共用）：控制权转移给
    /// new_holder + 剥离旧持有者（远程）全部终端归属（归还桌面：resize 恢复
    /// 桌面尺寸）；旧持有者由 controller-changed 全局遮罩盖住，接管者以同一
    /// 广播确认并触发全量刷新
    fn transfer_controller(&self, new_holder: ControllerHolder) {
        let stripped = {
            let mut inner = self.inner.lock();
            let old = inner.controller;
            inner.controller = Some(new_holder);
            let mut ids = Vec::new();
            if let Some(ControllerHolder::Remote(old_conn)) = old {
                if ControllerHolder::Remote(old_conn) != new_holder {
                    for (id, entry) in inner.sessions.iter_mut() {
                        if entry.active == ActiveView::Remote(old_conn) {
                            entry.active = ActiveView::Desktop;
                            ids.push(id.clone());
                        }
                    }
                }
            }
            ids
        };
        for id in &stripped {
            // 与 remove_conn 归还同构：遮罩解除 + 尺寸恢复
            emit_event("session-reclaimed", id, "");
            resize_pty(id, self.desktop_size_of(id));
            log::info!("[remote] session {} stripped to desktop (takeover by {:?})", id, new_holder);
        }
        self.broadcast_controller_changed();
    }

    /// 远程显式接管（controller.takeover 消息）
    fn handle_takeover(&self, conn_id: u64) {
        self.transfer_controller(ControllerHolder::Remote(conn_id));
    }

    /// 桌面显式接管（Tauri command controller_takeover / pty_claim 调用）：
    /// 桌面获得全局控制权 + 剥离当前远程持有者的全部终端归属
    pub fn desktop_takeover(&self) {
        if self.controller() == Some(ControllerHolder::Desktop) {
            return; // 已持有，幂等
        }
        log::info!("[remote] desktop takeover");
        self.transfer_controller(ControllerHolder::Desktop);
    }

    /// 控制权释放（远程持有者断连时调用；广播空 id = 无人持有）
    fn release_controller(&self, conn_id: u64) {
        let was_holder = {
            let mut inner = self.inner.lock();
            if inner.controller == Some(ControllerHolder::Remote(conn_id)) {
                inner.controller = None;
                true
            } else {
                false
            }
        };
        if was_holder {
            log::info!("[remote] controller released (conn {} closed)", conn_id);
            self.broadcast_controller_changed();
        }
    }

    /// state-changed 广播（实例方法：单元测试用独立实例验证广播半链）。
    /// 桌面 hook emit + 全体远程连接广播，payload {id: section, data: revision}。
    /// 取号与入队在同一锁窗口（保序，同 broadcast_controller_changed）
    pub fn emit_state_changed(&self, section: StateSection) {
        let name = crate::events::names::STATE_CHANGED;
        let section = section.as_str();
        let mut inner = self.inner.lock();
        inner.revision += 1;
        let rev = inner.revision;
        emit_via_hook(name, section, &rev.to_string());
        let msg = ServerMessage::RpcEvent {
            name: name.to_string(),
            payload: serde_json::json!({ "id": section, "data": rev.to_string() }),
        };
        for e in inner.conns.values() {
            let _ = e.tx.try_send(msg.clone());
        }
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
                Some(entry) => entry.tail.push(data),
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

    /// 记录会话 id（hook_server 收到 SessionStart 等 hook 事件时调用；幂等）
    pub fn set_session_id(&self, pty_id: &str, session_id: &str) {
        let mut inner = self.inner.lock();
        if let Some(entry) = inner.sessions.get_mut(pty_id) {
            if entry.session_id.as_deref() != Some(session_id) {
                entry.session_id = Some(session_id.to_string());
            }
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
            self.broadcast_to_all(msg);
        }
        Some(status)
    }

    /// 连接认证通过后注册（remote_server 直连传 false / remote_client 中继桥接传 true）
    pub fn add_conn(&self, conn_id: u64, sender: mpsc::Sender<ServerMessage>, via_relay: bool) {
        self.inner.lock().conns.insert(
            conn_id,
            ConnEntry { tx: sender, via_relay },
        );
    }

    /// 连接是否经中继（文件传输限额选档；未知连接返回 None，调用方按中继从严处理）
    pub fn conn_via_relay(&self, conn_id: u64) -> Option<bool> {
        self.inner.lock().conns.get(&conn_id).map(|e| e.via_relay)
    }

    /// 连接断开后清理全部订阅，并归还其接管的会话
    pub fn remove_conn(&self, conn_id: u64) {
        // 文件传输会话随连接回收（删 .part，防僵死会话占满配额）
        crate::file_transfer::cleanup_conn(conn_id);
        // 控制权释放（持有者断连；广播空 id = 其他设备可自由操作）
        self.release_controller(conn_id);
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

    /// 桌面端"接管回来"（Tauri 命令 pty_claim 调用）。显式接管动作：
    /// 收回该终端 + 同时获得全局控制权（设备互斥语义，桌面遮罩随广播解除）
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
        // 全局控制权：桌面接管回来同时获得（其他远程端显示遮罩）
        if self.controller() != Some(ControllerHolder::Desktop) {
            self.transfer_controller(ControllerHolder::Desktop);
        }
    }

    /// 应用层消息分发（认证后的消息，局域网/中继同一条路径）
    pub fn handle_client_message(&self, conn_id: u64, msg: &ClientMessage) {
        match msg {
            // hello 的 token 认证由连接层（LAN）或 relay 完成，此处统一回复 hello.ok——
            // 中继模式下手机消息经桥接直达这里，若无此回复手机将永远停在认证中
            ClientMessage::Hello { .. } => {
                let (controller, revision) = {
                    let inner = self.inner.lock();
                    (inner.controller.map(|c| c.as_id()), inner.revision)
                };
                self.try_send(
                    conn_id,
                    ServerMessage::HelloOk {
                        ver: cc_box_remote_protocol::PROTOCOL_VERSION,
                        device_name: device_name(),
                        conn_id,
                        // 桌面持有 = Some(0)（conn_id 从 1 起，0 保留给桌面）
                        controller,
                        revision,
                    },
                );
            }
            ClientMessage::SessionsList => {
                let list = self.session_list();
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
            ClientMessage::SessionCreate { path, session_id, skip_permissions, custom_args, session_name } => {
                // 设备互斥：写操作，他人持有时拒绝（前端遮罩是第一道，此处服务端兜底）
                if let Err(holder) = self.controller_gate(conn_id) {
                    log::warn!(
                        "[remote] session create denied (conn {} != controller {})",
                        conn_id,
                        holder.id_label()
                    );
                    self.try_send(
                        conn_id,
                        ServerMessage::SessionCreateFailed {
                            error: "controller held by another device".to_string(),
                        },
                    );
                    return;
                }
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
                            // 写操作成功 → 无人持有时自动成为控制者（设备互斥）
                            REMOTE_STATE.write_success_acquire(conn_id);
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
                // 设备互斥：写操作，他人持有时拒绝（kill 无应答通道，忽略 + log）
                if let Err(holder) = self.controller_gate(conn_id) {
                    log::warn!(
                        "[remote] session kill denied (conn {} != controller {})",
                        conn_id,
                        holder.id_label()
                    );
                    return;
                }
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
                    let tail = inner.sessions.get_mut(id).map(|e| e.tail.snapshot());
                    match tail {
                        Some(tail) => {
                            let ok = send_locked(
                                &inner,
                                conn_id,
                                ServerMessage::SessionSubscribed {
                                    id: id.clone(),
                                    tail,
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
                // 设备互斥：attach 不能抢占（产品决策）——他人持有控制权时拒绝，
                // 前端提示接管（controller.takeover 是唯一转移方式）
                if let Err(holder) = self.controller_gate(conn_id) {
                    log::warn!(
                        "[remote] attach denied for session {} (conn {} != controller {})",
                        id,
                        conn_id,
                        holder.id_label()
                    );
                    self.try_send(
                        conn_id,
                        ServerMessage::SessionAttachDenied {
                            id: id.clone(),
                            reason: "controller held by another device".to_string(),
                        },
                    );
                    return;
                }
                self.handle_attach(conn_id, id, *cols, *rows);
                // attach 成功（无人持有或已是自己）→ 自动持有控制权
                self.write_success_acquire(conn_id);
            }
            ClientMessage::ControllerTakeover => {
                self.handle_takeover(conn_id);
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
                // 终端级 active 检查 + 全局控制权兜底（takeover 剥离后旧持有者
                // 不应再写入；正常流两者恒一致，此处双保险）
                let allowed = {
                    let inner = self.inner.lock();
                    inner
                        .sessions
                        .get(id)
                        .map(|e| e.active == ActiveView::Remote(conn_id))
                        .unwrap_or(false)
                        && matches!(inner.controller, Some(ControllerHolder::Remote(c)) if c == conn_id)
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
                        && matches!(inner.controller, Some(ControllerHolder::Remote(c)) if c == conn_id)
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
                (old, entry.tail.snapshot())
            });
            let Some((old, tail)) = old_and_tail else {
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
                },
            );
            // attach 同时视为输出订阅者（旧订阅者保留，与桌面 claim 语义一致）
            let subs = inner.subscribers.entry(id.to_string()).or_default();
            subs.insert(conn_id);
            AttachOutcome { old, snapshot_queued }
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
                if let Some(tx) = inner.conns.get(conn_id).map(|e| &e.tx) {
                    let _ = tx.try_send(msg.clone());
                }
            }
        }
    }

    /// 广播给全部在线连接（会话列表级信息：exit、working/pending 状态）
    fn broadcast_to_all(&self, msg: ServerMessage) {
        let inner = self.inner.lock();
        for e in inner.conns.values() {
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
            if Some(*conn_id) == exclude {
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
}

struct AttachOutcome {
    old: ActiveView,
    snapshot_queued: bool,
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
