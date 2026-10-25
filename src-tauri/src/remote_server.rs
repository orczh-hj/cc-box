//! 远程访问服务器（局域网直连模式）
//!
//! - `GET /` 手机 Web UI（cc-box-web-assets：CC_BOX_WEB_DIR 磁盘目录优先、
//!   内嵌 dist 兜底，与 relay 共用同一实现）
//! - `GET /ws` WebSocket，首条消息必须是 hello（token 认证）
//! - 绑定 0.0.0.0:port（默认 8747），仅在远程开关开启时启动

use std::io::SeekFrom;
use std::net::SocketAddr;
use std::sync::atomic::{AtomicBool, AtomicU16, AtomicU64, Ordering};
use std::sync::LazyLock;
use std::time::Duration;

use axum::body::Body;
use axum::extract::ws::{Message, WebSocket, WebSocketUpgrade};
use axum::extract::Path as AxumPath;
use axum::http::{header, HeaderMap, Method, StatusCode};
use axum::response::{IntoResponse, Response};
use axum::routing::get;
use axum::Router;
use futures_util::{sink::SinkExt, StreamExt};
use tokio::io::{AsyncReadExt, AsyncSeekExt};
use tokio_util::io::ReaderStream;
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
/// 共享配置监视代际：每次 start 递增，旧 watchdog 在下一轮自行退出
static WATCHDOG_GEN: AtomicU64 = AtomicU64::new(0);

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
        // 一次性票据直链（微信/系统浏览器不带 token 也能下载）
        .route("/dl/:ticket", get(dl_handler))
        // Web UI 静态资源：磁盘目录（CC_BOX_WEB_DIR）优先、内嵌兜底
        .fallback(|uri| cc_box_web_assets::serve_static(WEB_DIR_ENV, uri));

    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    let listener = match bind_with_retry(addr, port).await {
        Ok(Some(l)) => l,
        // 宿主制：本机另一实例已提供服务（token 共享），跳过启动
        Ok(None) => return Ok(()),
        Err(e) => {
            log::error!("[remote] Failed to bind {}: {}", addr, e);
            // bind_with_retry 已给出完整、可执行的错误文案（含僵死实例提示）
            return Err(e);
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
    spawn_config_watchdog();

    log::info!("[remote] Remote server listening on http://0.0.0.0:{}", port);
    // URL 含 token fragment，日志只打码展示（完整地址经 UI 复制/二维码给出）
    log::info!(
        "[remote] Open on phone: http://<LAN-IP>:{}/#token={}****",
        port,
        &token[..4.min(token.len())]
    );
    Ok(())
}

/// 远程开关是共享配置：任一实例关闭后，正在服务的宿主必须跟随停止。
/// 多实例没有跨进程通道，用低频轮询兜底（读一个小 JSON，3s 一次开销可忽略）。
/// 只在真正监听成功（非宿主制跳过）时启动；代际保证重开时旧 watchdog 退出。
fn spawn_config_watchdog() {
    let generation = WATCHDOG_GEN.fetch_add(1, Ordering::SeqCst) + 1;
    tokio::spawn(async move {
        loop {
            tokio::time::sleep(Duration::from_secs(3)).await;
            if WATCHDOG_GEN.load(Ordering::SeqCst) != generation || !is_running() {
                return;
            }
            let enabled = match crate::store::get_app_config() {
                // 只有明确 false 才停；配置缺失/读失败都不视为关闭，避免误停
                Ok(c) => c.remote.and_then(|r| r.enabled),
                Err(_) => continue,
            };
            if enabled != Some(false) {
                continue;
            }
            // 停服前复查代际与配置：另一实例可能刚重新开启（其 start 会推进
            // 代际），此时不能把新服务误停
            if WATCHDOG_GEN.load(Ordering::SeqCst) != generation || !is_running() {
                return;
            }
            let still_disabled = crate::store::get_app_config()
                .map(|c| c.remote.and_then(|r| r.enabled) == Some(false))
                .unwrap_or(false);
            if !still_disabled {
                continue;
            }
            log::info!(
                "[remote] shared config disabled by another instance, stopping server"
            );
            stop().await;
            crate::remote_client::disconnect();
            return;
        }
    });
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
    let task = SERVER_TASK.lock().take();
    RUNNING.store(false, Ordering::SeqCst);
    if let Some(task) = task {
        task.abort();
        // abort 只是请求取消；必须等 axum::serve 的 future 真正 drop，listener
        // 才会关闭。不等就会在 Windows 上留下「LISTENING 但不再 accept/响应」
        // 的僵死 socket：后续 bind 10048，且 probe_host 也认不出宿主。
        // 2s 上限防极端情况下挂死 UI；超时后仍走下方端口轮询。
        let _ = tokio::time::timeout(Duration::from_secs(2), task).await;
    }
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
    // 被占用但不响应 = 上一实例的僵死 listener（历史上 abort 未 await 导致）；
    // 给用户可执行的提示，而不是笼统的「可能已有实例运行」
    let stale = tokio::time::timeout(
        Duration::from_millis(300),
        tokio::net::TcpStream::connect(("127.0.0.1", port)),
    )
    .await
    .map(|r| r.is_ok())
    .unwrap_or(false);
    if stale {
        Err(format!(
            "failed to bind port {}: {} (port accepts connections but does not respond — \
             restart the stale CC-Box instance)",
            port, last_err
        ))
    } else {
        Err(format!(
            "failed to bind port {} (another CC-Box instance may be running): {}",
            port, last_err
        ))
    }
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

/// 保存中继已配对设备 secret 列表（整体替换），保留其他字段
pub fn save_paired_secrets(secrets: &[String]) -> anyhow::Result<()> {
    let current = crate::store::get_app_config()?;
    let mut remote: serde_json::Value =
        serde_json::to_value(current.remote.clone().unwrap_or_default())?;
    remote["pairedSecrets"] = serde_json::json!(secrets);
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
    REMOTE_STATE.add_conn(conn_id, tx, false);

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

// ==================== 一次性票据直链 ====================

enum RangeSpec {
    Full,
    Partial(u64, u64),
    /// 起点 >= 文件长度：按 RFC 返回 416 + `Content-Range: bytes */size`
    Unsatisfiable,
}

/// 直连流结束/中断时释放票据（move 进 stream，drop 即回收）
struct TicketGuard(String);

impl Drop for TicketGuard {
    fn drop(&mut self) {
        crate::file_transfer::remove_ticket(&self.0);
    }
}

/// 票据直链下载：微信把链接交给系统浏览器/系统下载器后不带 token，
/// 票据本身即权限（128bit、TTL 10 分钟、单文件）。HEAD 只探测不扣次数。
async fn dl_handler(
    method: Method,
    AxumPath(ticket): AxumPath<String>,
    headers: HeaderMap,
) -> Response {
    let is_head = method == Method::HEAD;
    let access = if is_head {
        crate::file_transfer::peek_ticket(&ticket)
    } else {
        crate::file_transfer::take_ticket(&ticket).ok()
    };
    let Some(access) = access else {
        return dl_error(
            StatusCode::GONE,
            "download ticket not found or expired",
        );
    };

    let mut file = match tokio::fs::File::open(&access.path).await {
        Ok(f) => f,
        Err(_) => return dl_error(StatusCode::NOT_FOUND, "file not found"),
    };
    let md = match file.metadata().await {
        Ok(m) => m,
        Err(_) => return dl_error(StatusCode::NOT_FOUND, "cannot read source"),
    };
    // 打开后按句柄元数据复核（消除 stat→open 的 TOCTOU）
    if !crate::file_transfer::source_unchanged(&md, access.size, access.mtime) {
        return dl_error(
            StatusCode::CONFLICT,
            "source file changed during download",
        );
    }

    let (start, end, status) = match parse_range(&headers, access.size) {
        RangeSpec::Partial(s, e) => (s, e, StatusCode::PARTIAL_CONTENT),
        RangeSpec::Full => (0, access.size.saturating_sub(1), StatusCode::OK),
        RangeSpec::Unsatisfiable => {
            return Response::builder()
                .status(StatusCode::RANGE_NOT_SATISFIABLE)
                .header(header::CONTENT_RANGE, format!("bytes */{}", access.size))
                .body(Body::empty())
                .unwrap_or_else(|_| {
                    dl_error(StatusCode::INTERNAL_SERVER_ERROR, "response build failed")
                });
        }
    };
    let len = if access.size == 0 { 0 } else { end - start + 1 };

    if start > 0 && file.seek(SeekFrom::Start(start)).await.is_err() {
        return dl_error(StatusCode::INTERNAL_SERVER_ERROR, "cannot seek source");
    }

    let body = if is_head {
        Body::empty()
    } else {
        let guard = TicketGuard(ticket.clone());
        Body::from_stream(ReaderStream::new(file.take(len)).map(move |chunk| {
            let _ = &guard; // 流结束或客户端断开时 drop → 释放票据
            chunk
        }))
    };

    let mut builder = Response::builder()
        .status(status)
        .header(header::CONTENT_TYPE, "application/octet-stream")
        .header(header::CONTENT_LENGTH, len.to_string())
        .header(
            header::CONTENT_DISPOSITION,
            content_disposition(&access.file_name),
        )
        .header(header::CACHE_CONTROL, "no-store")
        .header(header::ACCEPT_RANGES, "bytes")
        .header("referrer-policy", "no-referrer");
    if status == StatusCode::PARTIAL_CONTENT {
        builder = builder.header(
            header::CONTENT_RANGE,
            format!("bytes {}-{}/{}", start, end, access.size),
        );
    }
    builder.body(body).unwrap_or_else(|_| {
        dl_error(StatusCode::INTERNAL_SERVER_ERROR, "response build failed")
    })
}

fn dl_error(status: StatusCode, msg: &str) -> Response {
    (status, msg.to_string()).into_response()
}

/// 单区间 Range（多区间与 `bytes=-N` 尾部范围不支持 → 回退全量 200）
fn parse_range(headers: &HeaderMap, size: u64) -> RangeSpec {
    if size == 0 {
        return RangeSpec::Full;
    }
    let Some(raw) = headers.get(header::RANGE).and_then(|v| v.to_str().ok()) else {
        return RangeSpec::Full;
    };
    let Some(spec) = raw.strip_prefix("bytes=") else {
        return RangeSpec::Full;
    };
    if spec.contains(',') {
        return RangeSpec::Full;
    }
    let Some((a, b)) = spec.split_once('-') else {
        return RangeSpec::Full;
    };
    let Ok(start) = a.trim().parse::<u64>() else {
        return RangeSpec::Full;
    };
    if start >= size {
        return RangeSpec::Unsatisfiable;
    }
    let end = if b.trim().is_empty() {
        size - 1
    } else {
        match b.trim().parse::<u64>() {
            Ok(v) => v.min(size - 1),
            Err(_) => return RangeSpec::Full,
        }
    };
    if end < start {
        return RangeSpec::Full;
    }
    RangeSpec::Partial(start, end)
}

/// RFC 5987 文件名（中文/空格等）+ ASCII fallback，保证各下载器都能存对名字
fn content_disposition(name: &str) -> String {
    let ascii: String = name
        .chars()
        .map(|c| {
            if c.is_ascii_graphic() && c != '"' && c != '\\' {
                c
            } else {
                '_'
            }
        })
        .collect();
    format!(
        "attachment; filename=\"{}\"; filename*=UTF-8''{}",
        ascii,
        rfc5987_encode(name)
    )
}

fn rfc5987_encode(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    for b in s.as_bytes() {
        match b {
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'.' | b'_' | b'~' => {
                out.push(*b as char)
            }
            _ => out.push_str(&format!("%{:02X}", b)),
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    fn range_header(v: &str) -> HeaderMap {
        let mut h = HeaderMap::new();
        h.insert(header::RANGE, v.parse().unwrap());
        h
    }

    #[test]
    fn RemoteServer_Range_Partial_1() {
        assert!(matches!(
            parse_range(&range_header("bytes=10-19"), 100),
            RangeSpec::Partial(10, 19)
        ));
        assert!(matches!(
            parse_range(&range_header("bytes=10-"), 100),
            RangeSpec::Partial(10, 99)
        ));
    }

    #[test]
    fn RemoteServer_Range_ClampsEnd_1() {
        assert!(matches!(
            parse_range(&range_header("bytes=90-200"), 100),
            RangeSpec::Partial(90, 99)
        ));
    }

    #[test]
    fn RemoteServer_Range_Unsatisfiable_1() {
        // 起点越界 → 416
        assert!(matches!(
            parse_range(&range_header("bytes=100-"), 100),
            RangeSpec::Unsatisfiable
        ));
        assert!(matches!(
            parse_range(&range_header("bytes=1000-1200"), 100),
            RangeSpec::Unsatisfiable
        ));
    }

    #[test]
    fn RemoteServer_Range_UnsupportedFallsBackFull_1() {
        // 多区间 / 尾部范围 / 反向 / 无 Range / 空文件 → 全量 200
        assert!(matches!(
            parse_range(&range_header("bytes=0-1,5-6"), 100),
            RangeSpec::Full
        ));
        assert!(matches!(
            parse_range(&range_header("bytes=-10"), 100),
            RangeSpec::Full
        ));
        assert!(matches!(
            parse_range(&range_header("bytes=10-5"), 100),
            RangeSpec::Full
        ));
        assert!(matches!(parse_range(&HeaderMap::new(), 100), RangeSpec::Full));
        assert!(matches!(
            parse_range(&range_header("bytes=0-"), 0),
            RangeSpec::Full
        ));
    }

    #[test]
    fn RemoteServer_ContentDisposition_Rfc5987_1() {
        let cd = content_disposition("报告 2024.pdf");
        assert!(cd.starts_with("attachment; "));
        assert!(cd.contains("filename*=UTF-8''"));
        // 中文百分号编码（报 = E6 8A A5）
        assert!(cd.contains("%E6%8A%A5%E5%91%8A"));
        assert!(cd.contains("filename=\""));
    }
}
