//! 远程访问服务器（局域网直连模式）
//!
//! - `GET /` 手机 Web UI（cc-box-web-assets：CC_BOX_WEB_DIR 磁盘目录优先、
//!   内嵌 dist 兜底，与 relay 共用同一实现）
//! - `GET /ws` WebSocket，首条消息必须是 hello（token 认证）
//! - 绑定 0.0.0.0:port（默认 8747），仅在远程开关开启时启动

use std::net::SocketAddr;
use std::sync::atomic::{AtomicBool, AtomicU16, Ordering};
use std::sync::LazyLock;
use std::time::Duration;

use axum::extract::ws::{Message, WebSocket, WebSocketUpgrade};
use axum::response::Response;
use axum::routing::get;
use axum::Router;
use futures_util::{sink::SinkExt, StreamExt};
use parking_lot::Mutex;
use tauri::AppHandle;
use tokio::sync::{mpsc, watch};

use cc_box_remote_protocol::{ClientMessage, ServerMessage};

use crate::remote_state::{next_conn_id, REMOTE_STATE};

/// Web UI 磁盘目录环境变量（优先于内嵌；debug 构建本就动态读 dist，
/// release 借此获得同样的免重编更新通道）
const WEB_DIR_ENV: &str = "CC_BOX_WEB_DIR";

pub const DEFAULT_PORT: u16 = 8747;

/// 关停信号用 watch（带值，不丢信号）：stop 置 true，所有在线 WS 连接 select 到即退出。
/// 不能只靠 graceful_shutdown——它等待存量连接自然结束，而长连 WS 永不主动结束，会挂死 stop()。
static SHUTDOWN: LazyLock<(watch::Sender<bool>, watch::Receiver<bool>)> =
    LazyLock::new(|| watch::channel(false));
static SERVER_TASK: LazyLock<Mutex<Option<tokio::task::JoinHandle<()>>>> =
    LazyLock::new(|| Mutex::new(None));
static RUNNING: AtomicBool = AtomicBool::new(false);
/// 实际监听端口（bind 成功时记录）：stop 后等待端口释放以它为准——
/// 不能现读配置（用户可能在停止前改过端口配置，等错端口）
static CURRENT_PORT: AtomicU16 = AtomicU16::new(0);

/// 应用启动时调用：开关开启则启动服务器
pub async fn init(_app_handle: AppHandle) {
    let enabled = crate::store::get_app_config()
        .ok()
        .and_then(|c| c.remote)
        .and_then(|r| r.enabled)
        .unwrap_or(false);
    if enabled {
        if let Err(e) = start().await {
            log::error!("[remote] Remote server not started: {}", e);
        }
    }
}

/// 启动服务器（已运行则跳过）。失败返回错误原因（UI 可见），如端口被占用。
pub async fn start() -> Result<(), String> {
    if RUNNING.load(Ordering::SeqCst) {
        return Ok(());
    }

    let port = crate::store::get_app_config()
        .ok()
        .and_then(|c| c.remote)
        .and_then(|r| r.port)
        .unwrap_or(DEFAULT_PORT);
    let token = match ensure_token() {
        Ok(t) => t,
        Err(e) => {
            log::error!("[remote] Failed to prepare token, remote server not started: {}", e);
            return Err(format!("failed to prepare token: {}", e));
        }
    };

    let app = Router::new()
        .route("/ws", get(ws_handler))
        // Web UI 静态资源：磁盘目录（CC_BOX_WEB_DIR）优先、内嵌兜底
        .fallback(|uri| cc_box_web_assets::serve_static(WEB_DIR_ENV, uri));

    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    let listener = match bind_with_retry(addr, port).await {
        Ok(Some(l)) => l,
        // 宿主制：本机另一实例已提供服务（token 共享），跳过启动
        Ok(None) => return Ok(()),
        Err(e) => {
            log::error!("[remote] Failed to bind {}: {}", addr, e);
            return Err(format!(
                "failed to bind port {} (another CC-Box instance may be running): {}",
                port, e
            ));
        }
    };

    // 先记端口再置运行态：若顺序相反，微窗口内并发的 stop() 会 swap 到 0、
    // 跳过等待端口释放（错失本轮清理窗口）
    CURRENT_PORT.store(port, Ordering::SeqCst);
    RUNNING.store(true, Ordering::SeqCst);
    let _ = SHUTDOWN.0.send(false); // 重置关停信号（再次开启时）
    let task = tokio::spawn(async move {
        if let Err(e) = axum::serve(listener, app).await {
            log::error!("[remote] Server error: {}", e);
        }
    });
    *SERVER_TASK.lock() = Some(task);

    log::info!("[remote] Remote server listening on http://0.0.0.0:{}", port);
    // URL 含 token fragment，日志只打码展示（完整地址经 UI 复制/二维码给出）
    log::info!(
        "[remote] Open on phone: http://<LAN-IP>:{}/#token={}****",
        port,
        &token[..4.min(token.len())]
    );
    Ok(())
}

/// 停止服务器：abort 监听任务 + 通知所有在线 WS 连接退出（各自清理后 remove_conn）。
/// abort 后 Windows 上端口释放有延迟（IOCP pending accept 清理，实测 ~100ms 起，
/// 有在线连接时更久——曾实测 v0.15.1 实例停止后端口长期不释放、新实例 bind
/// 10048）：轮询 bind 试探直至端口可重绑（试探 listener 未挂 accept，drop 即
/// 释放不产生 TIME_WAIT），上限 5s——放行后若仍被占，start 的 bind 会给出
/// 明确错误，不能为等待释放挂死 stop（UI 等待该命令返回）
pub async fn stop() {
    if !RUNNING.load(Ordering::SeqCst) {
        return;
    }
    let _ = SHUTDOWN.0.send(true);
    let port = CURRENT_PORT.swap(0, Ordering::SeqCst);
    if let Some(task) = SERVER_TASK.lock().take() {
        task.abort();
    }
    RUNNING.store(false, Ordering::SeqCst);
    if port != 0 {
        let addr = SocketAddr::from(([0, 0, 0, 0], port));
        // 注：等待期间若并发 start() 抢先绑到端口，此处会空转满 5s 才返回——
        // 状态无破坏（新 server 正常运行），仅本次 stop 返回偏慢，可接受
        for _ in 0..50 {
            if tokio::net::TcpListener::bind(addr).await.is_ok() {
                break;
            }
            tokio::time::sleep(Duration::from_millis(100)).await;
        }
    }
    log::info!("[remote] Remote server stopped");
}

/// bind + 失败时的宿主探测与短重试：
/// - Ok(Some(listener))：绑定成功
/// - Ok(None)：端口被本机另一 CC-Box 实例持有且服务可达（宿主制跳过，面板
///   显示宿主 URL）；仅首轮探测——宿主在 ~1.5s 重试窗口内出现的竞态概率极
///   低且误报后重开一次即命中跳过，不值得为此多付一次 3s 探测超时
/// - Err：真错误。探测不通可能是「宿主刚停止、端口释放延迟」的残骸（TCP
///   可连但不响应）——短窗口重试 bind；仍失败即端口被其他程序占用或残骸
///   未消（v0.15.1 stop 泄漏场景），报错由用户处置。
///   最坏耗时 ~4.5s（首轮探测 3s + 5×300ms），start 为 UI await 命令，
///   失败路径的等待时长按此上限控制
async fn bind_with_retry(
    addr: SocketAddr,
    port: u16,
) -> Result<Option<tokio::net::TcpListener>, String> {
    let mut last_err = String::new();
    for attempt in 0..6 {
        match tokio::net::TcpListener::bind(addr).await {
            Ok(l) => return Ok(Some(l)),
            Err(e) => {
                last_err = e.to_string();
                if attempt == 0 && probe_host(port).await {
                    log::info!(
                        "[remote] Remote already hosted by another instance on port {}, skipping start",
                        port
                    );
                    return Ok(None);
                }
                if attempt < 5 {
                    tokio::time::sleep(Duration::from_millis(300)).await;
                }
            }
        }
    }
    Err(last_err)
}

pub fn is_running() -> bool {
    RUNNING.load(Ordering::SeqCst)
}

/// 配置首选端口（用户设定或默认 8747）
fn preferred_port() -> u16 {
    crate::store::get_app_config()
        .ok()
        .and_then(|c| c.remote)
        .and_then(|r| r.port)
        .unwrap_or(DEFAULT_PORT)
}

/// 探测本机端口是否已有 CC-Box 远程服务（宿主实例）：
/// TCP 可连、HTTP GET / 返回 200 且 body 含 "CC-Box"（index 特征，防其他
/// HTTP 服务占用造成误判）。整体 3s 超时——端口被不响应的服务占用时不得挂起。
pub async fn probe_host(port: u16) -> bool {
    tokio::time::timeout(std::time::Duration::from_secs(3), async {
        use tokio::io::{AsyncReadExt, AsyncWriteExt};
        let Ok(mut stream) = tokio::net::TcpStream::connect(("127.0.0.1", port)).await else {
            return false;
        };
        let req = format!("GET / HTTP/1.0\r\nHost: 127.0.0.1:{}\r\n\r\n", port);
        if stream.write_all(req.as_bytes()).await.is_err() {
            return false;
        }
        let mut buf = vec![0u8; 2048];
        let mut total = Vec::new();
        loop {
            let n = stream.read(&mut buf).await.unwrap_or(0);
            if n == 0 {
                break;
            }
            total.extend_from_slice(&buf[..n]);
            if total.len() >= 2048 {
                break;
            }
        }
        let body = String::from_utf8_lossy(&total);
        body.starts_with("HTTP/") && body.contains(" 200") && body.contains("CC-Box")
    })
    .await
    .unwrap_or(false)
}

/// 宿主制：本实例未运行远程、但本机已有其他实例提供（探测动态判定，
/// 宿主退出后探测失败，本实例下次开启即可无缝接管）
pub async fn hosted_elsewhere() -> bool {
    if is_running() {
        return false;
    }
    probe_host(preferred_port()).await
}

/// 供设置页展示的直连 URL（含 token fragment）
pub fn lan_url(token: &str, port: u16) -> String {
    let ip = lan_ip().map(|i| i.to_string()).unwrap_or_else(|| "<本机IP>".to_string());
    format!("http://{}:{}/#token={}", ip, port, token)
}

/// 取局域网出口 IP（UDP connect 不实际发包）
fn lan_ip() -> Option<std::net::IpAddr> {
    let s = std::net::UdpSocket::bind("0.0.0.0:0").ok()?;
    s.connect("8.8.8.8:80").ok()?;
    s.local_addr().ok().map(|a| a.ip())
}

/// 读取 token；不存在则生成（32 字节熵，64 hex）并写回配置
fn ensure_token() -> anyhow::Result<String> {
    let config = crate::store::get_app_config()?;
    if let Some(remote) = &config.remote {
        if let Some(t) = &remote.token {
            if !t.is_empty() {
                return Ok(t.clone());
            }
        }
    }

    let token = generate_token();
    let port = config.remote.as_ref().and_then(|r| r.port);
    save_remote_config(None, Some(&token), port)?;
    Ok(token)
}

/// 生成直连 token（32 字节熵，64 hex）
fn generate_token() -> String {
    format!(
        "{}{}",
        uuid::Uuid::new_v4().simple(),
        uuid::Uuid::new_v4().simple()
    )
}

/// 轮换直连 token（安全阻断语义：「停止→开启」后旧局域二维码全部失效，
/// 面板展示新码，用户重扫即可）
pub fn rotate_token() -> anyhow::Result<String> {
    let config = crate::store::get_app_config()?;
    let port = config.remote.as_ref().and_then(|r| r.port);
    let token = generate_token();
    save_remote_config(None, Some(&token), port)?;
    log::info!("[remote] token rotated (old LAN QR codes invalidated)");
    Ok(token)
}

/// 字段级更新 remote 配置（None = 保留现值；update_app_config 为浅合并，
/// 必须先读现有值合并后整体写回，否则会丢 relay/device 字段）
pub fn save_remote_config(
    enabled: Option<bool>,
    token: Option<&str>,
    port: Option<u16>,
) -> anyhow::Result<()> {
    let current = crate::store::get_app_config()?;
    let mut remote: serde_json::Value =
        serde_json::to_value(current.remote.clone().unwrap_or_default())?;
    if let Some(v) = enabled {
        remote["enabled"] = serde_json::json!(v);
    }
    if let Some(t) = token {
        remote["token"] = serde_json::json!(t);
    }
    if let Some(p) = port {
        remote["port"] = serde_json::json!(p);
    }
    crate::store::update_app_config(serde_json::json!({ "remote": remote }))?;
    Ok(())
}

/// 保存中继配置（url 与设备身份），保留其他字段
pub fn save_relay_config(
    url: Option<&str>,
    identity: Option<(&str, &str)>,
) -> anyhow::Result<()> {
    let current = crate::store::get_app_config()?;
    let mut remote: serde_json::Value =
        serde_json::to_value(current.remote.clone().unwrap_or_default())?;
    if let Some(u) = url {
        remote["relayUrl"] = serde_json::json!(u);
    }
    if let Some((id, key)) = identity {
        remote["deviceId"] = serde_json::json!(id);
        remote["deviceKey"] = serde_json::json!(key);
    }
    crate::store::update_app_config(serde_json::json!({ "remote": remote }))?;
    Ok(())
}

/// 当前有效 token（供 WS 认证与状态查询；未生成时返回空）
pub fn current_token() -> String {
    crate::store::get_app_config()
        .ok()
        .and_then(|c| c.remote)
        .and_then(|r| r.token)
        .filter(|t| !t.is_empty())
        .unwrap_or_default()
}

// ==================== WebSocket ====================

async fn ws_handler(ws: WebSocketUpgrade) -> Response {
    ws.on_upgrade(handle_socket)
}

async fn handle_socket(mut socket: WebSocket) {
    let conn_id = next_conn_id();

    // 认证阶段：首条消息必须是 hello，10 秒超时
    let first = match tokio::time::timeout(Duration::from_secs(10), socket.recv()).await {
        Ok(Some(Ok(msg))) => msg,
        _ => { return; }
    };
    let hello = match first {
        Message::Text(t) => serde_json::from_str::<ClientMessage>(&t),
        _ => { return; }
    };
    let expected = current_token();
    let authenticated = match &hello {
        Ok(ClientMessage::Hello { token, .. }) if !expected.is_empty() && *token == expected => true,
        _ => false,
    };
    if !authenticated {
        let err = ServerMessage::HelloError {
            error: "invalid token".to_string(),
        };
        let _ = socket
            .send(Message::Text(serde_json::to_string(&err).unwrap_or_default()))
            .await;
        return;
    }

    log::info!("[remote] conn {} authenticated", conn_id);

    let (tx, mut rx) = mpsc::channel::<ServerMessage>(crate::remote_state::CONN_CHANNEL_CAPACITY);
    REMOTE_STATE.add_conn(conn_id, tx);

    // hello.ok 由 remote_state 的唯一 handler 回复（先 add_conn 再分发，
    // 与中继桥接路径完全一致——hello 应答不在连接层重复实现）
    REMOTE_STATE.handle_client_message(conn_id, hello.as_ref().unwrap());

    let (mut sender, mut receiver) = socket.split();
    let send_task = tokio::spawn(async move {
        while let Some(msg) = rx.recv().await {
            let Ok(text) = serde_json::to_string(&msg) else { continue };
            if sender.send(Message::Text(text)).await.is_err() {
                break;
            }
        }
    });

    // 接收循环：select 关停信号（stop() 时主动退出，否则 graceful 关闭会挂死）；
    // WS 层 Ping 由 tungstenite 自动应答；应用层 ping/pong 走协议。
    // 注意：watch 新 Receiver 必须先消费初始值，否则 changed() 会立即就绪一次（连接秒断）
    let mut shutdown = SHUTDOWN.1.clone();
    let _ = shutdown.borrow_and_update();
    loop {
        tokio::select! {
            _ = shutdown.changed() => break,
            msg = receiver.next() => {
                match msg {
                    Some(Ok(Message::Text(t))) => {
                        match serde_json::from_str::<ClientMessage>(&t) {
                            Ok(m) => REMOTE_STATE.handle_client_message(conn_id, &m),
                            Err(e) => {
                                log::debug!("[remote] conn {} invalid message ignored: {}", conn_id, e);
                            }
                        }
                    }
                    Some(Ok(Message::Close(_))) | None | Some(Err(_)) => break,
                    Some(Ok(_)) => {}
                }
            }
        }
    }

    REMOTE_STATE.remove_conn(conn_id);
    send_task.abort();
    log::info!("[remote] conn {} closed", conn_id);
}
