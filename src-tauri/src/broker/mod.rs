//! 设备级后台 Broker（`cc-box --broker`）
//!
//! 无 GUI 窗口、无 PTY：持有设备身份、LAN 入口、relay 连接、全局控制权与
//! 成员注册/路由。GUI 实例通过本机 IPC 注册为成员，PTY 始终留在 GUI。
//!
//! 生命周期（详见 lifecycle.rs）：
//! - `running`：IPC + 远程服务在线（远程开关可开可关），窗口全关也常驻
//! - `user_quit`：托盘「退出 CC-Box」，GUI 不再自动拉起
//! - `update_in_progress`：安装前退出，版本变化后解除抑制
//! - `idle_exit`：旧版标记，仅兼容读取（新版不再写）

pub mod autostart;
pub mod ipc;
pub mod lifecycle;
pub mod ownership;
pub mod status;
pub mod tray;
pub mod update;
pub mod windows;

use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::LazyLock;
use std::time::Duration;

use parking_lot::Mutex;
use tokio::sync::watch;

use crate::gateway_proto::{BrokerExitReason, BrokerOp, BrokerStatus, GwDown};

static LAN_ERROR: Mutex<Option<String>> = Mutex::new(None);
static LEGACY_OWNED: AtomicBool = AtomicBool::new(false);
static TAKEOVER_GEN: AtomicU64 = AtomicU64::new(0);
static SHUTDOWN: LazyLock<Mutex<Option<watch::Sender<bool>>>> = LazyLock::new(|| Mutex::new(None));

pub(crate) fn lan_error() -> Option<String> {
    LAN_ERROR.lock().clone()
}

pub(crate) fn legacy_owned() -> bool {
    LEGACY_OWNED.load(Ordering::SeqCst)
}

/// 当前进程是否为 Broker（与 remote_state::is_broker 同源）
pub fn is_broker() -> bool {
    crate::remote_state::is_broker()
}

/// Broker 运行状态快照（GUI 进程返回 None）
pub fn status_snapshot() -> Option<BrokerStatus> {
    status::snapshot()
}

/// 请求 Broker 主循环退出（托盘 Quit / 更新准备）
pub fn request_exit() {
    if let Some(tx) = SHUTDOWN.lock().as_ref() {
        let _ = tx.send(true);
    }
}

fn remote_enabled() -> bool {
    status::remote_enabled_from_config()
}

fn broadcast_exit(reason: BrokerExitReason) {
    // v1 成员只认 Shutdown；v2 成员收 BrokerExit（含原因）
    crate::gateway::broadcast_down(GwDown::Shutdown);
    crate::gateway::broadcast_down(GwDown::BrokerExit { reason });
}

fn sync_status() {
    crate::gateway::sync_relay_status();
}

/// Broker 主入口：纯 tokio，无 Tauri。由 main.rs 在 `--broker` 时分派。
pub async fn run_headless(autostart: bool) {
    crate::logger::init();
    let _guard = match ownership::BrokerInstanceGuard::acquire() {
        Ok(g) => g,
        Err(e) => {
            log::info!("[broker] not starting: {}", e);
            return;
        }
    };
    // 登录自启是新的用户会话：清除上次“退出 CC-Box”的持久抑制。
    if autostart {
        lifecycle::clear_user_quit();
    }
    // 竞态兜底：托盘 user_quit / 更新标记有效时，即使被并发拉起也不覆盖标记
    if !lifecycle::should_autostart() {
        log::info!("[broker] lifecycle marker suppresses startup; exiting");
        return;
    }
    crate::remote_state::set_broker_mode(true);
    if let Err(e) = ownership::ensure_broker_secret() {
        log::error!("[broker] failed to prepare broker secret: {}", e);
    }
    let _ = lifecycle::write(lifecycle::LifecycleState::Running, None);

    #[cfg(not(test))]
    crate::remote_state::set_event_hook(Box::new(|name, id, data| {
        crate::gateway::broadcast_desktop_event(
            name,
            serde_json::json!({ "id": id, "data": data }),
        );
    }));

    let (tx, mut rx) = watch::channel(false);
    *SHUTDOWN.lock() = Some(tx);

    if let Err(e) = ipc::spawn_server().await {
        // 没有 IPC 入口的 Broker 只会占着 broker.lock，GUI 永远无法注册；
        // 退出释放锁，让 GUI 的重试循环重新拉起干净的 Broker。
        log::error!("[broker] IPC server failed; exiting: {}", e);
        return;
    }

    if remote_enabled() {
        start_remote_service().await;
    }

    log::info!(
        "[broker] started (pid {}, remote_enabled={})",
        std::process::id(),
        remote_enabled()
    );

    // 状态镜像周期同步：配对码 5 分钟 TTL 过期后自动重申并广播新码，
    // 避免 GUI 长期展示失效二维码
    spawn_status_sync();

    // 常驻：窗口全部关闭也不退出。只有托盘/命令显式 Quit 或更新准备才结束。
    let _ = rx.changed().await;
    log::info!("[broker] shutting down");
    stop_remote_service().await;
    // 退出原因已由 Quit / PrepareForUpdate 写入；此处不再覆盖。
}

/// 启动/恢复设备级远程入口。LAN 与 relay 独立：
/// - 第三方占口 → 记录 lan_error，relay 继续；
/// - 旧版 Gateway 占口 → legacy_owned，不连 relay，周期探测接管；
/// - 正常 → 持有 gateway.lock + 生成旧版密钥 + LAN + relay。
pub async fn start_remote_service() {
    let port = crate::remote_server::preferred_port();

    if ownership::legacy_gateway_owns_port(port).await {
        enter_legacy_owned("legacy CC-Box owns the remote entry");
        return;
    }

    if !ownership::try_acquire_gateway_lock() {
        // 旧版实例持有设备所有权：不抢锁、不连 relay（同 device_id 会互踢）
        enter_legacy_owned("device gateway locked by a legacy instance");
        return;
    }
    ownership::spawn_lock_heartbeat();
    if let Err(e) = ownership::ensure_gateway_secret() {
        log::error!("[broker] failed to write gateway secret: {}", e);
        ownership::release_gateway_lock();
        return;
    }
    LEGACY_OWNED.store(false, Ordering::SeqCst);
    TAKEOVER_GEN.fetch_add(1, Ordering::SeqCst);

    match crate::remote_server::start().await {
        Ok(()) => {
            *LAN_ERROR.lock() = None;
        }
        Err(e) => {
            if crate::remote_server::probe_host(port).await {
                // 取锁后才发现端口被旧版占用：必须先释放锁，否则本进程心跳
                // 让 lock_is_live() 永真，接管循环永远无法重试
                ownership::release_gateway_lock();
                enter_legacy_owned("port held by a legacy CC-Box instance");
                return;
            }
            log::warn!("[broker] LAN unavailable (relay only): {}", e);
            *LAN_ERROR.lock() = Some(e);
            spawn_lan_retry(port);
        }
    }

    // relay 是设备身份的唯一出站通道，独立于 LAN 成败
    crate::remote_client::connect().await;
    sync_status();
}

pub async fn stop_remote_service() {
    crate::remote_server::stop().await;
    crate::remote_client::disconnect();
    ownership::release_gateway_lock();
    LEGACY_OWNED.store(false, Ordering::SeqCst);
    TAKEOVER_GEN.fetch_add(1, Ordering::SeqCst);
    *LAN_ERROR.lock() = None;
    sync_status();
}

pub async fn restart_remote_service() {
    stop_remote_service().await;
    if remote_enabled() {
        start_remote_service().await;
    }
}

fn enter_legacy_owned(reason: &str) {
    LEGACY_OWNED.store(true, Ordering::SeqCst);
    *LAN_ERROR.lock() = Some(reason.to_string());
    log::warn!("[broker] legacy-owned: {}", reason);
    spawn_takeover_loop(reason.to_string());
    sync_status();
}

fn spawn_takeover_loop(reason: String) {
    let gen = TAKEOVER_GEN.fetch_add(1, Ordering::SeqCst) + 1;
    tokio::spawn(async move {
        loop {
            tokio::time::sleep(Duration::from_secs(5)).await;
            if TAKEOVER_GEN.load(Ordering::SeqCst) != gen
                || !LEGACY_OWNED.load(Ordering::SeqCst)
            {
                return;
            }
            if !remote_enabled() {
                return;
            }
            let port = crate::remote_server::preferred_port();
            let port_free = !crate::remote_server::probe_host(port).await;
            let lock_free = !ownership::lock_is_live();
            if port_free && lock_free {
                log::info!("[broker] legacy entry released; taking over");
                LEGACY_OWNED.store(false, Ordering::SeqCst);
                *LAN_ERROR.lock() = None;
                start_remote_service().await;
                return;
            }
        }
        #[allow(unreachable_code)]
        {
            let _ = reason;
        }
    });
}

fn spawn_lan_retry(port: u16) {
    let gen = TAKEOVER_GEN.fetch_add(1, Ordering::SeqCst) + 1;
    tokio::spawn(async move {
        loop {
            tokio::time::sleep(Duration::from_secs(5)).await;
            if TAKEOVER_GEN.load(Ordering::SeqCst) != gen || !remote_enabled() {
                return;
            }
            if crate::remote_server::is_running() {
                return;
            }
            if tokio::net::TcpListener::bind(("0.0.0.0", port))
                .await
                .is_ok()
            {
                if crate::remote_server::start().await.is_ok() {
                    *LAN_ERROR.lock() = None;
                    sync_status();
                    log::info!("[broker] LAN listener recovered on port {}", port);
                    return;
                }
            }
        }
    });
}

fn pair_code_needs_refresh(relay_connected: bool, pair_code: &str) -> bool {
    relay_connected && pair_code.is_empty()
}

/// 周期同步设备级状态；配对码过期（status_snapshot 过滤后为空）时重申。
fn spawn_status_sync() {
    tokio::spawn(async move {
        loop {
            tokio::time::sleep(Duration::from_secs(30)).await;
            if !remote_enabled() {
                continue;
            }
            let (_, connected, _, pair_code) = crate::remote_client::status_snapshot();
            if pair_code_needs_refresh(connected, &pair_code) {
                log::debug!("[broker] pair code missing/expired; requesting a new one");
                crate::remote_client::request_pair_code();
            }
            // 广播当前镜像（含空码状态；新码到达时 relay 回调会再次广播）
            sync_status();
        }
    });
}

/// 管理操作唯一实现（GUI 经 IPC 转发；Broker 内 RPC 直达）
pub async fn control(op: BrokerOp) -> Result<Option<BrokerStatus>, String> {
    match op {
        BrokerOp::GetStatus => {
            // 配对码缺失/过期且中继在线：按需重申（非阻塞，结果经广播镜像）
            if let Some(s) = status_snapshot() {
                if s.relay_connected && s.relay_pair_code.is_empty() && !s.legacy_owned {
                    crate::remote_client::request_pair_code();
                }
            }
            Ok(status_snapshot())
        }
        BrokerOp::SetEnabled { enabled } => {
            // 与 config watchdog / 密钥轮换互斥：防「复查 disabled → stop」误杀
            // 用户刚开启的新服务
            let _guard = crate::remote_server::REMOTE_OP_LOCK.lock().await;
            crate::remote_server::save_remote_config(Some(enabled), None, None)
                .map_err(|e| e.to_string())?;
            if enabled {
                // 显式开启：清除 user_quit / 更新失败残留标记
                let _ = lifecycle::clear();
                start_remote_service().await;
            } else {
                stop_remote_service().await;
            }
            Ok(status_snapshot())
        }
        BrokerOp::RestartRemote => {
            let _guard = crate::remote_server::REMOTE_OP_LOCK.lock().await;
            restart_remote_service().await;
            Ok(status_snapshot())
        }
        BrokerOp::RefreshPairCode => {
            crate::remote_client::refresh_pair_code().await?;
            Ok(status_snapshot())
        }
        BrokerOp::RotateSecret => {
            if legacy_owned() {
                return Err(
                    "legacy CC-Box owns the remote entry; retry after it exits".to_string()
                );
            }
            rotate_secret().await?;
            Ok(status_snapshot())
        }
        BrokerOp::SetRelayUrl { url } => {
            crate::remote_client::set_relay(url).await?;
            Ok(status_snapshot())
        }
        BrokerOp::SetAutostart { enabled } => {
            autostart::set_enabled(enabled)?;
            sync_status();
            Ok(status_snapshot())
        }
        BrokerOp::Quit => {
            let _ = lifecycle::write(lifecycle::LifecycleState::UserQuit, None);
            // 先关闭所有窗口，再广播 BrokerExit：成员收到 exit 会停止重连，
            // 若先广播会收不到后续的 WindowCommand。
            windows::close_all_windows(Duration::from_secs(5)).await;
            broadcast_exit(BrokerExitReason::Quit);
            stop_remote_service().await;
            // 不在此处 request_exit：admin IPC 需要先写回响应；托盘调用方
            // 在 control 返回后自行 request_exit。
            Ok(None)
        }
        BrokerOp::PrepareForUpdate { keep_instance_id } => {
            let _ = lifecycle::write(
                lifecycle::LifecycleState::UpdateInProgress,
                Some(env!("APP_VERSION").to_string()),
            );
            // 保留发起更新的窗口（前端还要继续 install/relaunch），关闭其余窗口。
            windows::close_windows(keep_instance_id.as_deref(), Duration::from_secs(5)).await;
            broadcast_exit(BrokerExitReason::Update);
            stop_remote_service().await;
            // 同 Quit：先让 admin 响应写出，再退出
            Ok(None)
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn Broker_PairCodeRefreshPredicate_1() {
        assert!(super::pair_code_needs_refresh(true, ""));
        assert!(!super::pair_code_needs_refresh(false, ""));
        assert!(!super::pair_code_needs_refresh(true, "ABCD"));
    }
}

async fn rotate_secret() -> Result<(), String> {
    let _guard = crate::remote_server::REMOTE_OP_LOCK.lock().await;
    crate::remote_server::rotate_token().map_err(|e| e.to_string())?;
    crate::remote_client::rotate_identity();
    if crate::remote_server::is_running() {
        crate::remote_server::stop().await;
        crate::remote_server::start()
            .await
            .map_err(|e| e.to_string())?;
    }
    if remote_enabled() {
        crate::remote_client::disconnect();
        crate::remote_client::connect().await;
    }
    sync_status();
    Ok(())
}
