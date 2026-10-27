//! GUI 侧 Broker 客户端
//!
//! - 确保本机 Broker 运行（`cc-box --broker`，按 lifecycle 标记抑制自动拉起）；
//! - 通过 Named Pipe / UDS 注册为成员（密钥 + 协议版本 + 会话快照）；
//! - 维护状态上行（可靠）与每连接输出桥（可丢 + seq 缺口）；
//! - 断开后重连/重启 Broker，不终止本地 PTY。

use std::collections::HashMap;
use std::path::Path;
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::LazyLock;
use std::time::{Duration, Instant};

use parking_lot::Mutex;
use tokio::sync::{mpsc, watch};

use cc_box_remote_protocol::ServerMessage;

use crate::broker::ipc;
use crate::gateway_proto::{
    instance_id, BrokerAdminRequest, BrokerAdminResponse, BrokerOp, BrokerStatus, GwDown, GwUp,
    GW_PROTO_VER,
};

static MEMBER_GEN: LazyLock<(watch::Sender<u64>, watch::Receiver<u64>)> =
    LazyLock::new(|| watch::channel(0));
static REGISTERED: AtomicBool = AtomicBool::new(false);
static SUPPRESSED: AtomicBool = AtomicBool::new(false);
static INSTANCE_NAME: LazyLock<Mutex<Option<String>>> = LazyLock::new(|| Mutex::new(None));
static STATUS_MIRROR: LazyLock<Mutex<Option<BrokerStatus>>> = LazyLock::new(|| Mutex::new(None));
static BRIDGES: LazyLock<Mutex<HashMap<u64, mpsc::Sender<ServerMessage>>>> =
    LazyLock::new(|| Mutex::new(HashMap::new()));
static SPAWN_AT: LazyLock<Mutex<Option<Instant>>> = LazyLock::new(|| Mutex::new(None));
static NEXT_ADMIN_ID: AtomicU64 = AtomicU64::new(1);

/// 输出队列保留给可靠消息的 headroom（容量 4096 的 25%）
const DATA_QUEUE_CAPACITY: usize = 4096;
const DATA_RESERVE: usize = DATA_QUEUE_CAPACITY / 4;

pub fn is_registered() -> bool {
    REGISTERED.load(Ordering::SeqCst)
}

pub fn instance_name() -> Option<String> {
    INSTANCE_NAME.lock().clone()
}

/// 窗口标题变化上报 Broker（托盘窗口列表展示；未注册时静默丢弃，
/// 下次 MemberHello 会带上最新标题）
pub fn set_window_title(title: String) {
    let title = title.trim().to_string();
    if title.is_empty() {
        return;
    }
    crate::remote_state::uplink_send(crate::gateway_proto::GwUp::WindowTitle { title });
}

/// 设备级状态镜像（远程面板/命令用）
pub fn status_mirror() -> Option<BrokerStatus> {
    STATUS_MIRROR.lock().clone()
}

/// 旧字段兼容： (enabled, connected, url, pair_code)
pub fn relay_mirror() -> Option<(bool, bool, String, String)> {
    STATUS_MIRROR.lock().as_ref().map(|s| {
        (
            s.relay_enabled,
            s.relay_connected,
            s.relay_url.clone(),
            s.relay_pair_code.clone(),
        )
    })
}

/// 停止成员连接（远程关闭/进程退出）
pub fn stop_member() {
    REGISTERED.store(false, Ordering::SeqCst);
    *INSTANCE_NAME.lock() = None;
    let _ = MEMBER_GEN.0.send(MEMBER_GEN.1.borrow().wrapping_add(1));
}

/// 用户显式开启远程/启动后台服务：解除所有自动拉起抑制
/// （user_quit，以及安装失败残留的 update_in_progress）
pub fn clear_suppression() {
    SUPPRESSED.store(false, Ordering::SeqCst);
    let _ = crate::broker::lifecycle::clear();
}

/// 启动连接循环（幂等：代际替换旧 loop）
pub fn spawn_connect_loop() {
    static SPAWN_LOCK: LazyLock<Mutex<()>> = LazyLock::new(|| Mutex::new(()));
    let _guard = SPAWN_LOCK.lock();
    let gen = MEMBER_GEN.1.borrow().wrapping_add(1);
    let _ = MEMBER_GEN.0.send(gen);
    REGISTERED.store(false, Ordering::SeqCst);
    tokio::spawn(async move {
        run_loop(gen).await;
    });
}

fn is_current_gen(gen: u64) -> bool {
    *MEMBER_GEN.1.borrow() == gen
}

async fn run_loop(gen: u64) {
    let mut last_error_log = Instant::now() - Duration::from_secs(60);
    loop {
        if !is_current_gen(gen) || SUPPRESSED.load(Ordering::SeqCst) {
            return;
        }
        if !crate::broker::lifecycle::should_autostart() {
            log::info!("[broker-client] autostart suppressed by lifecycle marker");
            return;
        }
        ensure_running();

        match connect_once(gen).await {
            MemberEvent::Registered => {
                if !is_current_gen(gen) {
                    return;
                }
                cleanup_bridges();
                if SUPPRESSED.load(Ordering::SeqCst) {
                    return;
                }
                // Broker 可能瞬时重启：短退避后重连
                tokio::time::sleep(Duration::from_millis(300)).await;
            }
            MemberEvent::Rejected(reason) => {
                if !is_current_gen(gen) {
                    return;
                }
                log::warn!("[broker-client] registration rejected: {}", reason);
                cleanup_bridges();
                tokio::time::sleep(Duration::from_millis(800)).await;
            }
            MemberEvent::NoBroker => {
                if !is_current_gen(gen) {
                    return;
                }
                cleanup_bridges();
                if last_error_log.elapsed() > Duration::from_secs(30) {
                    log::warn!("[broker-client] broker not reachable; retrying");
                    last_error_log = Instant::now();
                }
                ensure_running();
                tokio::time::sleep(Duration::from_millis(500)).await;
            }
            MemberEvent::Exiting => {
                if is_current_gen(gen) {
                    cleanup_bridges();
                    SUPPRESSED.store(true, Ordering::SeqCst);
                }
                return;
            }
            // 旧循环在握手/重连期间被新 spawn_connect_loop 取代：
            // 不得清理新循环已建立的 uplink/bridge
            MemberEvent::Stale => return,
        }
    }
}

enum MemberEvent {
    Registered,
    Rejected(String),
    NoBroker,
    Exiting,
    Stale,
}

fn instance_label() -> Option<String> {
    let sessions = crate::remote_state::REMOTE_STATE.session_list();
    if let Some(s) = sessions.first() {
        if let Some(name) = Path::new(&s.cwd).file_name().and_then(|n| n.to_str()) {
            if !name.is_empty() {
                return Some(name.to_string());
            }
        }
    }
    hostname::get()
        .ok()
        .map(|h| h.to_string_lossy().into_owned())
}

async fn connect_once(gen: u64) -> MemberEvent {
    // 代际在入口捕获：握手期间若 spawn_connect_loop 提升代际，旧循环必须
    // 立即放弃且不得改写全局 uplink/bridge（新循环已接管）
    let mut gen_rx = MEMBER_GEN.1.clone();
    let _ = gen_rx.borrow_and_update();
    if !is_current_gen(gen) {
        return MemberEvent::Stale;
    }

    let Some(secret) = crate::broker::ownership::read_broker_secret() else {
        return MemberEvent::NoBroker;
    };
    let stream = match ipc::connect_client().await {
        Ok(s) => s,
        Err(_) => return MemberEvent::NoBroker,
    };
    if !is_current_gen(gen) {
        return MemberEvent::Stale;
    }
    let (mut rd, mut wr) = tokio::io::split(stream);

    // 先挂状态上行再发快照：注册窗口内新 spawn/改名的会话会排队，注册成功
    // 后按序补发，不会因“快照时不存在、uplink 尚未设置”而丢失。
    let (state_tx, mut state_rx) = mpsc::unbounded_channel::<GwUp>();
    if !is_current_gen(gen) {
        return MemberEvent::Stale;
    }
    crate::remote_state::set_uplink(Some(state_tx));
    let (data_tx, mut data_rx) = mpsc::channel::<GwUp>(DATA_QUEUE_CAPACITY);

    let hello = GwUp::MemberHello {
        ver: GW_PROTO_VER,
        instance_id: instance_id().to_string(),
        secret,
        label: instance_label(),
        pid: std::process::id(),
        window_title: crate::gui_window_title(),
        sessions: crate::remote_state::REMOTE_STATE.session_list(),
    };
    let Ok(bytes) = serde_json::to_vec(&hello) else {
        if is_current_gen(gen) {
            crate::remote_state::set_uplink(None);
        }
        return MemberEvent::NoBroker;
    };
    if ipc::write_frame(&mut wr, &bytes).await.is_err() {
        if is_current_gen(gen) {
            crate::remote_state::set_uplink(None);
        }
        return MemberEvent::NoBroker;
    }

    // 读首帧期间也要能被新代际打断（最长 10s，不能阻塞旧循环退出）
    let first = tokio::select! {
        _ = gen_rx.changed() => return MemberEvent::Stale,
        res = tokio::time::timeout(Duration::from_secs(10), ipc::read_frame(&mut rd)) => {
            match res {
                Ok(Ok(Some(frame))) => frame,
                _ => {
                    if is_current_gen(gen) {
                        crate::remote_state::set_uplink(None);
                    }
                    return MemberEvent::NoBroker;
                }
            }
        }
    };
    if !is_current_gen(gen) {
        return MemberEvent::Stale;
    }
    match serde_json::from_slice::<GwDown>(&first) {
        Ok(GwDown::RegisterOk { instance_name, .. }) => {
            REGISTERED.store(true, Ordering::SeqCst);
            *INSTANCE_NAME.lock() = Some(instance_name.clone());
            log::info!("[broker-client] registered as '{}'", instance_name);
        }
        Ok(GwDown::Reject { reason }) => {
            if is_current_gen(gen) {
                crate::remote_state::set_uplink(None);
            }
            return MemberEvent::Rejected(reason);
        }
        _ => {
            if is_current_gen(gen) {
                crate::remote_state::set_uplink(None);
            }
            return MemberEvent::NoBroker;
        }
    }

    let write_task = tokio::spawn(async move {
        loop {
            tokio::select! {
                biased;
                Some(up) = state_rx.recv() => {
                    let Ok(bytes) = serde_json::to_vec(&up) else { continue };
                    if ipc::write_frame(&mut wr, &bytes).await.is_err() { break; }
                }
                Some(up) = data_rx.recv() => {
                    let Ok(bytes) = serde_json::to_vec(&up) else { continue };
                    if ipc::write_frame(&mut wr, &bytes).await.is_err() { break; }
                }
                else => break,
            }
        }
    });

    let mut interval = tokio::time::interval(Duration::from_secs(10));
    interval.set_missed_tick_behavior(tokio::time::MissedTickBehavior::Delay);
    let mut exiting = false;

    loop {
        tokio::select! {
            _ = gen_rx.changed() => break,
            _ = interval.tick() => {
                let ts = std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .map(|d| d.as_millis() as u64)
                    .unwrap_or(0);
                if data_tx.try_send(GwUp::Heartbeat { ts }).is_err() {
                    // 心跳属可靠控制面；队列满说明连接已失效 → 重连
                    break;
                }
            }
            frame = ipc::read_frame(&mut rd) => {
                let Ok(Some(frame)) = frame else { break };
                let Ok(down) = serde_json::from_slice::<GwDown>(&frame) else {
                    log::debug!("[broker-client] invalid frame ignored");
                    continue;
                };
                match down {
                    GwDown::ClientMsg { conn_id, via_relay, msg, holder } => {
                        if let Some(holder) = holder {
                            crate::remote_state::REMOTE_STATE
                                .sync_controller_shadow_if_changed(holder);
                        }
                        ensure_bridge(conn_id, via_relay, data_tx.clone());
                        crate::remote_state::REMOTE_STATE.handle_client_message(conn_id, &msg);
                    }
                    GwDown::ConnDrop { conn_id } => {
                        BRIDGES.lock().remove(&conn_id);
                        crate::remote_state::REMOTE_STATE.remove_conn(conn_id);
                    }
                    GwDown::ControllerSet { holder } => {
                        crate::remote_state::REMOTE_STATE.sync_controller_shadow(holder);
                    }
                    GwDown::ControllerStrip { conn_id } => {
                        crate::remote_state::REMOTE_STATE.strip_controller_conn(conn_id);
                    }
                    GwDown::RelayStatus { .. } => {}
                    GwDown::BrokerStatus { status } => {
                        *STATUS_MIRROR.lock() = Some(status);
                    }
                    GwDown::DesktopEvent { name, payload } => {
                        let id = payload.get("id").and_then(|v| v.as_str()).unwrap_or("");
                        let data = payload.get("data").and_then(|v| v.as_str()).unwrap_or("");
                        crate::remote_state::fire_event_hook(&name, id, data);
                    }
                    GwDown::BrokerExit { reason } => {
                        log::info!("[broker-client] broker exit: {:?}", reason);
                        if reason
                            == crate::gateway_proto::BrokerExitReason::Idle
                        {
                            // 空闲回收不是用户意图：按普通断连重连/拉起新 Broker，
                            // 不设置持久抑制
                            break;
                        }
                        SUPPRESSED.store(true, Ordering::SeqCst);
                        exiting = true;
                        break;
                    }
                    GwDown::HeartbeatAck { .. } => {}
                    GwDown::WindowCommand { action } => {
                        log::info!("[broker-client] window command: {:?}", action);
                        match action {
                            crate::gateway_proto::WindowAction::Close => {
                                crate::request_gui_exit();
                                exiting = true;
                                break;
                            }
                            crate::gateway_proto::WindowAction::Show => {
                                crate::request_gui_show();
                            }
                            crate::gateway_proto::WindowAction::OpenRemote => {
                                crate::request_gui_show();
                                crate::request_gui_open_remote();
                            }
                            crate::gateway_proto::WindowAction::OpenUpdate => {
                                crate::request_gui_show();
                                crate::request_gui_open_update();
                            }
                        }
                    }
                    GwDown::Shutdown => {
                        // v1 兼容器：v2 成员以 BrokerExit 为准；单独 Shutdown
                        // 视为远程被停止而非进程退出，保持连接继续等待。
                    }
                    GwDown::RegisterOk { .. } | GwDown::Reject { .. } => {}
                }
            }
        }
    }

    write_task.abort();
    REGISTERED.store(false, Ordering::SeqCst);
    *INSTANCE_NAME.lock() = None;
    if exiting {
        MemberEvent::Exiting
    } else {
        MemberEvent::Registered
    }
}

/// 首见 conn_id 建桥；输出消息走 headroom + 真实 seq（丢帧由客户端检测）。
fn ensure_bridge(conn_id: u64, via_relay: bool, data_tx: mpsc::Sender<GwUp>) {
    {
        let bridges = BRIDGES.lock();
        if bridges.contains_key(&conn_id) {
            return;
        }
    }
    let (tx, mut rx) = mpsc::channel::<ServerMessage>(crate::remote_state::CONN_CHANNEL_CAPACITY);
    crate::remote_state::REMOTE_STATE.add_member_conn(conn_id, tx.clone(), via_relay);
    BRIDGES.lock().insert(conn_id, tx);
    tokio::spawn(async move {
        while let Some(msg) = rx.recv().await {
            // controller-changed 真源在 Broker，成员影子事件不上行；
            // state-changed 必须上行（成员桌面/成员路由写操作导致的配置变化，
            // Broker 转发时会重新分配设备级 revision）
            if let ServerMessage::RpcEvent { ref name, .. } = msg {
                if name == crate::events::names::CONTROLLER_CHANGED {
                    continue;
                }
            }
            let is_output = matches!(msg, ServerMessage::PtyOutput { .. });
            let up = GwUp::ToConn { conn_id, msg };
            if is_output {
                // 保留 25% 给可靠控制/输入；输出丢弃时 seq 缺口可见
                if data_tx.capacity() <= DATA_RESERVE {
                    log::debug!("[broker-client] output dropped for conn {} (backpressure)", conn_id);
                    continue;
                }
                if data_tx.try_send(up).is_err() {
                    log::debug!("[broker-client] output enqueue failed for conn {}", conn_id);
                }
            } else if data_tx.send(up).await.is_err() {
                break;
            }
        }
    });
}

/// 断连清理：桥、上行、远端视图、状态镜像
fn cleanup_bridges() {
    REGISTERED.store(false, Ordering::SeqCst);
    *INSTANCE_NAME.lock() = None;
    let bridges = {
        let mut b = BRIDGES.lock();
        std::mem::take(&mut *b)
    };
    for conn_id in bridges.keys() {
        crate::remote_state::REMOTE_STATE.remove_conn(*conn_id);
    }
    crate::remote_state::REMOTE_STATE.reset_remote_views();
    crate::remote_state::set_uplink(None);
    *STATUS_MIRROR.lock() = None;
}

/// 拉起 Broker 的可执行文件。Linux AppImage 的 current_exe 指向临时挂载目录，
/// 优先使用稳定的 `$APPIMAGE` 原文件。
fn broker_executable() -> Option<std::path::PathBuf> {
    #[cfg(target_os = "linux")]
    {
        if let Ok(appimage) = std::env::var("APPIMAGE") {
            if !appimage.is_empty() {
                return Some(std::path::PathBuf::from(appimage));
            }
        }
    }
    std::env::current_exe().ok()
}

/// 按需拉起 Broker（3s 限流；单实例由 broker.lock 兜底）
fn ensure_running() {
    if SUPPRESSED.load(Ordering::SeqCst) || !crate::broker::lifecycle::should_autostart() {
        return;
    }
    {
        let mut at = SPAWN_AT.lock();
        if at.map(|t| t.elapsed() < Duration::from_secs(3)).unwrap_or(false) {
            return;
        }
        *at = Some(Instant::now());
    }
    let Some(exe) = broker_executable() else {
        return;
    };
    let mut cmd = std::process::Command::new(exe);
    cmd.arg("--broker")
        .stdin(std::process::Stdio::null())
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null());
    #[cfg(windows)]
    {
        use std::os::windows::process::CommandExt;
        const CREATE_NO_WINDOW: u32 = 0x08000000;
        const CREATE_NEW_PROCESS_GROUP: u32 = 0x00000200;
        cmd.creation_flags(CREATE_NO_WINDOW | CREATE_NEW_PROCESS_GROUP);
    }
    #[cfg(unix)]
    {
        use std::os::unix::process::CommandExt;
        cmd.process_group(0);
    }
    match cmd.spawn() {
        Ok(_) => log::info!("[broker-client] broker process spawned"),
        Err(e) => log::warn!("[broker-client] failed to spawn broker: {}", e),
    }
}

/// 管理操作（短连接）：GUI 与 Broker 内命令共用
pub async fn control(op: BrokerOp) -> Result<Option<BrokerStatus>, String> {
    if crate::broker::is_broker() {
        return crate::broker::control(op).await;
    }
    if matches!(op, BrokerOp::SetEnabled { enabled: true }) {
        clear_suppression();
        // 托盘退出后 run_loop 已结束；显式开启必须重新启动连接循环
        spawn_connect_loop();
    }
    if matches!(op, BrokerOp::PrepareForUpdate { .. })
        && !crate::broker::lifecycle::should_autostart()
    {
        // 已被用户退出：无需再拉起一个 Broker 来退出它
        return Ok(None);
    }
    if !crate::broker::lifecycle::should_autostart()
        && !matches!(op, BrokerOp::SetEnabled { enabled: true })
    {
        return Err("background broker is stopped".to_string());
    }
    ensure_running();
    admin_call(op).await
}

/// 更新安装失败等场景：清除 update/user_quit 抑制并恢复 Broker 连接。
pub fn resume_connection() {
    let _ = crate::broker::lifecycle::clear();
    clear_suppression();
    spawn_connect_loop();
}

/// 直接发送管理请求，不自动拉起 Broker。
/// 用于 `--quit-broker`：没有 Broker 时应当报错，而不是先拉起再退出。
pub async fn control_direct(op: BrokerOp) -> Result<Option<BrokerStatus>, String> {
    if crate::broker::is_broker() {
        return crate::broker::control(op).await;
    }
    admin_call(op).await
}

async fn admin_call(op: BrokerOp) -> Result<Option<BrokerStatus>, String> {
    // Broker 可能刚被拉起：secret 生成 + IPC 监听有启动窗口，短重试
    let deadline = Instant::now() + Duration::from_secs(5);
    let (secret, stream) = loop {
        if let Some(secret) = crate::broker::ownership::read_broker_secret() {
            if let Ok(stream) = ipc::connect_client().await {
                break (secret, stream);
            }
        }
        if Instant::now() >= deadline {
            return Err("broker unavailable".to_string());
        }
        tokio::time::sleep(Duration::from_millis(100)).await;
    };
    let (mut rd, mut wr) = tokio::io::split(stream);
    let id = NEXT_ADMIN_ID.fetch_add(1, Ordering::SeqCst);
    let req = BrokerAdminRequest::Admin { id, secret, op };
    let bytes = serde_json::to_vec(&req).map_err(|e| e.to_string())?;
    ipc::write_frame(&mut wr, &bytes)
        .await
        .map_err(|e| e.to_string())?;
    let frame = tokio::time::timeout(Duration::from_secs(15), ipc::read_frame(&mut rd))
        .await
        .map_err(|_| "broker admin timeout".to_string())?
        .map_err(|e| e.to_string())?
        .ok_or_else(|| "broker closed admin connection".to_string())?;
    let resp: BrokerAdminResponse =
        serde_json::from_slice(&frame).map_err(|e| e.to_string())?;
    match resp {
        BrokerAdminResponse::Result {
            ok, status, error, ..
        } => {
            if ok {
                Ok(status)
            } else {
                Err(error.unwrap_or_else(|| "broker operation failed".to_string()))
            }
        }
    }
}

/// 更新安装前调用：让 Broker 写 update_in_progress 并退出，释放可执行文件锁。
/// 保留当前窗口（前端还要继续 install/relaunch），其余窗口由 Broker 关闭。
pub async fn prepare_for_update() -> Result<(), String> {
    control(BrokerOp::PrepareForUpdate {
        keep_instance_id: Some(crate::gateway_proto::instance_id().to_string()),
    })
    .await
    .map(|_| ())
}
