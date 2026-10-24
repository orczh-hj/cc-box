//! cc-box-relay：CC-Box 远程中继服务器（库部分）
//!
//! 部署在公网 VPS（如 cc.orczh.top），职责（详见 remote/README.md）：
//! - `/ws/device` 桌面端注册（device_key 认证，出站长连接穿透 NAT）
//! - `/ws` 手机端连接（token v2 `<device_id>.<secret>` 前缀路由，secret 由桌面校验）
//! - `POST /api/pair` 配对码换 token v2（一次性，TTL 5 分钟）
//! - `GET /` 手机 Web UI（cc-box-web-assets：CC_BOX_RELAY_WEB_DIR 磁盘目录
//!   优先，内嵌 dist 兜底——前端可独立 scp 更新免重编，见 deploy.md）
//!
//! 路由原则：**不解析、不存储、不改写应用层 payload**（透传），为 E2E 加密留路。
//! 无凭证存储：token 自带路由前缀、secret 校验下沉桌面（token 表随重启清空的
//! 问题不复存在——已配对手机在 relay 重启后自动重连恢复）。
//! 全内存态（重启即清在线连接），无数据库。
//!
//! lib/bin 拆分是为了集成测试能在内存中起 server（tests/integration.rs）。

use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::Arc;
use std::time::{Duration, Instant};

use axum::extract::ws::{Message, WebSocket, WebSocketUpgrade};
use axum::extract::{ConnectInfo, State};
use axum::response::Response;
use axum::routing::{get, post};
use axum::{Json, Router};
use futures_util::sink::SinkExt;
use futures_util::StreamExt;
use parking_lot::Mutex;
use serde::Deserialize;
use serde::Serialize;
use tokio::sync::mpsc;

/// Web UI 磁盘目录环境变量（优先于内嵌；前端独立更新通道——目录内容
/// scp 即时生效，环境变量增删需重启 relay）
const WEB_DIR_ENV: &str = "CC_BOX_RELAY_WEB_DIR";

/// 配对码有效期
pub const PAIR_TTL: Duration = Duration::from_secs(5 * 60);

// ==================== 状态 ====================

#[derive(Default)]
struct AppState {
    /// 在线桌面：device_id → 出站通道
    devices: HashMap<String, mpsc::Sender<DeviceOutMsg>>,
    /// 在线手机：conn_id → 绑定的 device_id 与出站通道
    clients: HashMap<u64, ClientConn>,
    /// 配对码 → 配对信息（一次性，TTL 内有效）
    pair_codes: HashMap<String, PairCode>,
    /// 首次注册绑定的 device_key：device_id → key（后续重连需匹配）
    device_keys: HashMap<String, String>,
    /// /api/pair 失败节流：IP → (连续失败次数, 锁定截止时间)
    pair_fails: HashMap<std::net::IpAddr, (u32, Option<Instant>)>,
    next_conn_id: u64,
}

/// 配对码绑定信息：token v2 = `<device_id>.<secret>`。
/// secret 由桌面 pair.create 提供（新版桌面）；旧版桌面不带时 relay 生成
/// （fallback），并在配对成功时经 `pair.granted` 下发桌面固化——该桌面
/// 升级新版后已配对手机不失效
struct PairCode {
    device_id: String,
    secret: String,
    /// secret 来源为桌面（true）或 relay fallback 生成（false，需下发）
    from_desktop: bool,
    exp: Instant,
}

struct ClientConn {
    device_id: String,
    tx: mpsc::Sender<String>,
    /// 断连通知（client.kick）：唤醒 handle_client 接收循环退出以关闭 WS
    kick: std::sync::Arc<tokio::sync::Notify>,
}

/// 桌面出站消息（JSON 字符串，直接写 WS）
type DeviceOutMsg = String;

/// pair_ttl 与基础状态分开放（测试用短 TTL 注入，生产用 PAIR_TTL）
pub type SharedState = Arc<Mutex<AppStateInner>>;

pub struct AppStateInner {
    st: AppState,
    pair_ttl: Duration,
}

/// 构造 relay 状态（pair_ttl 生产用 PAIR_TTL，集成测试可缩短）
pub fn new_state(pair_ttl: Duration) -> SharedState {
    Arc::new(Mutex::new(AppStateInner { st: AppState::default(), pair_ttl }))
}

/// 构建完整 Router（main 与集成测试共用）
pub fn build_router(state: SharedState) -> Router {
    Router::new()
        .route("/ws/device", get(ws_device))
        .route("/ws", get(ws_client))
        .route("/api/pair", post(api_pair))
        // Web UI 静态资源：磁盘目录（CC_BOX_RELAY_WEB_DIR）优先、内嵌兜底
        .fallback(|uri| cc_box_web_assets::serve_static(WEB_DIR_ENV, uri))
        .with_state(state)
}

// ==================== 配对 API ====================

#[derive(Deserialize)]
struct PairRequest {
    code: String,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct PairResponse {
    ok: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    token: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    error: Option<String>,
}

/// 配对失败锁定阈值：连续失败 N 次后该 IP 锁定 60s（防配对码暴力猜解）
const PAIR_FAIL_LIMIT: u32 = 5;
const PAIR_LOCKOUT: Duration = Duration::from_secs(60);

async fn api_pair(
    State(state): State<SharedState>,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    headers: axum::http::HeaderMap,
    Json(req): Json<PairRequest>,
) -> Json<PairResponse> {
    // 经 nginx 反代时 ConnectInfo 恒为 127.0.0.1（全局误锁）：优先取可信
    // 反代注入的 X-Real-IP，取不到再回退连接地址
    let ip = headers
        .get("x-real-ip")
        .and_then(|v| v.to_str().ok())
        .and_then(|s| s.parse().ok())
        .unwrap_or_else(|| addr.ip());
    let mut inner = state.lock();

    // 失败锁定检查（顺带清理已过期的锁定项，防表膨胀）
    inner
        .st
        .pair_fails
        .retain(|_, (_, until)| until.is_none() || *until > Some(Instant::now()));
    if let Some((fails, until)) = inner.st.pair_fails.get(&ip) {
        if *fails >= PAIR_FAIL_LIMIT && until.is_some_and(|u| u > Instant::now()) {
            return Json(PairResponse { ok: false, token: None, error: Some("too many attempts, try later".into()) });
        }
    }

    let fail = |st: &mut AppStateInner, msg: &str| {
        let entry = st.st.pair_fails.entry(ip).or_insert((0, None));
        entry.0 += 1;
        if entry.0 >= PAIR_FAIL_LIMIT {
            entry.1 = Some(Instant::now() + PAIR_LOCKOUT);
        }
        PairResponse { ok: false, token: None, error: Some(msg.into()) }
    };

    // 清理过期配对码
    inner.st.pair_codes.retain(|_, pc| pc.exp > Instant::now());

    let Some(pc) = inner.st.pair_codes.remove(&req.code) else {
        return Json(fail(&mut inner, "invalid or expired code"));
    };
    // 设备必须在线
    let Some(device_tx) = inner.st.devices.get(&pc.device_id).cloned() else {
        return Json(fail(&mut inner, "device offline"));
    };

    // token v2：`<device_id>.<secret>`——relay 只按前缀路由，secret 校验在桌面。
    // fallback（旧版桌面未提供 secret，由 relay 生成）时下发桌面固化
    let token = format!("{}.{}", pc.device_id, pc.secret);
    if !pc.from_desktop {
        let _ = device_tx.try_send(
            serde_json::to_string(&DeviceOut::PairGranted { secret: pc.secret }).unwrap(),
        );
    }
    // 成功后清该 IP 的失败计数
    inner.st.pair_fails.remove(&ip);
    Json(PairResponse { ok: true, token: Some(token), error: None })
}

// ==================== 桌面连接 /ws/device ====================

#[derive(Deserialize, Serialize)]
#[serde(tag = "type", rename_all_fields = "camelCase")]
enum DeviceInMsg {
    #[serde(rename = "register")]
    Register { device_id: String, device_key: String },
    #[serde(rename = "pair.create")]
    PairCreate {
        /// token v2 的 secret（新版桌面预生成并暂存，配对成功后校验/固化；
        /// 缺省 = 旧版桌面，relay 生成并经 pair.granted 回发）
        #[serde(default)]
        secret: Option<String>,
    },
    #[serde(rename = "data")]
    Data {
        to: u64,
        payload: serde_json::Value,
    },
    /// 桌面校验手机 token 失败：向该手机下发 hello.error(reason) 并断开其连接。
    /// token v2 校验在桌面、连接挂在 relay——不显式踢则恶意客户端可无视
    /// hello.error 长期占住 clients 表（MAX_CLIENTS 被打满拒绝正常手机）
    #[serde(rename = "client.kick")]
    ClientKick { conn: u64, reason: String },
    /// 中继层 keepalive：桌面定时发送，防止 nginx 等反代掐断空闲连接
    #[serde(rename = "ping")]
    Ping,
}

#[derive(Serialize)]
#[serde(tag = "type", rename_all_fields = "camelCase")]
enum DeviceOut {
    #[serde(rename = "registered")]
    Registered { device_id: String },
    #[serde(rename = "pair.created")]
    PairCreated { code: String },
    /// fallback 配对成功（旧版桌面未带 secret）时下发，桌面固化进持久列表
    #[serde(rename = "pair.granted")]
    PairGranted { secret: String },
    #[serde(rename = "data")]
    Data {
        from: u64,
        payload: serde_json::Value,
    },
    #[serde(rename = "client.disconnected")]
    ClientDisconnected { conn: u64 },
    #[serde(rename = "pong")]
    Pong,
}

/// 资源上限（公网暴露面防护：防连接/凭证表打爆小规格 VPS）
const MAX_DEVICES: usize = 64;
const MAX_CLIENTS: usize = 256;
/// WS 单消息上限（默认 64MB 过大；应用层最大为 rpc 参数/设置导入，4MB 足够）
const MAX_WS_MESSAGE: usize = 4 * 1024 * 1024;

async fn ws_device(
    State(state): State<SharedState>,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    ws: WebSocketUpgrade,
) -> Response {
    ws.max_message_size(MAX_WS_MESSAGE)
        .on_upgrade(move |socket| handle_device(socket, state, addr))
}

async fn handle_device(mut socket: WebSocket, state: SharedState, addr: SocketAddr) {
    // 首条必须是 register
    let first = match tokio::time::timeout(Duration::from_secs(10), socket.recv()).await {
        Ok(Some(Ok(Message::Text(t)))) => t,
        _ => return,
    };
    let Ok(DeviceInMsg::Register { device_id, device_key }) =
        serde_json::from_str::<DeviceInMsg>(&first)
    else {
        return;
    };
    // device_key 形态校验（64 hex）与重连一致性
    if device_key.len() != 64 || !device_key.chars().all(|c| c.is_ascii_hexdigit()) {
        let _ = socket
            .send(Message::Text(r#"{"type":"register.failed","reason":"bad device_key"}"#.into()))
            .await;
        return;
    }
    let key_mismatch = {
        let inner = state.lock();
        match inner.st.device_keys.get(&device_id) {
            Some(bound) => *bound != device_key,
            None => false,
        }
    };
    if key_mismatch {
        let _ = socket
            .send(Message::Text(r#"{"type":"register.failed","reason":"device_key mismatch"}"#.into()))
            .await;
        return;
    }

    let (tx, mut rx) = mpsc::channel::<DeviceOutMsg>(256);
    {
        // 设备表上限（公网开放注册：防打爆）；guard 在块内结束，不跨 await
        let device_slots_full = {
            let inner = state.lock();
            inner.st.devices.len() >= MAX_DEVICES && !inner.st.devices.contains_key(&device_id)
        };
        if device_slots_full {
            let _ = socket
                .send(Message::Text(r#"{"type":"register.failed","reason":"relay busy"}"#.into()))
                .await;
            return;
        }
        let mut inner = state.lock();
        // 旧连接被顶替
        inner.st.device_keys.entry(device_id.clone()).or_insert_with(|| device_key.clone());
        if let Some(old) = inner.st.devices.insert(device_id.clone(), tx.clone()) {
            let _ = old.try_send(r#"{"type":"replaced"}"#.into());
        }
        // 该设备旧手机连接断开（设备重连视为网络抖动，手机会自行重连）
    }

    println!("[relay] device online: {} from {}", device_id, addr);

    let _ = socket
        .send(Message::Text(
            serde_json::to_string(&DeviceOut::Registered { device_id: device_id.clone() }).unwrap(),
        ))
        .await;

    let (mut sender, mut receiver) = socket.split();
    let send_task = tokio::spawn(async move {
        while let Some(text) = rx.recv().await {
            if sender.send(Message::Text(text)).await.is_err() {
                break;
            }
        }
    });

    let device_id_for_cleanup = device_id.clone();
    let pair_ttl = state.lock().pair_ttl;
    while let Some(Ok(msg)) = receiver.next().await {
        let Message::Text(text) = msg else {
            if matches!(msg, Message::Close(_)) {
                break;
            }
            continue;
        };
        let Ok(parsed) = serde_json::from_str::<DeviceInMsg>(&text) else {
            continue;
        };
        match parsed {
            DeviceInMsg::Register { .. } => {}
            DeviceInMsg::PairCreate { secret } => {
                // 8 位 hex（16^8 ≈ 43 亿）+ TTL 5 分钟 + /api/pair 失败锁定，抵御暴力猜解。
                // secret 由桌面提供（校验/固化都在桌面）；旧版桌面不带时 relay 生成 fallback
                let (secret, from_desktop) = match secret {
                    Some(s) if s.len() == 32 && s.chars().all(|c| c.is_ascii_hexdigit()) => {
                        (s, true)
                    }
                    _ => (uuid::Uuid::new_v4().simple().to_string(), false),
                };
                let code = uuid::Uuid::new_v4().simple().to_string()[..8].to_uppercase();
                state.lock().st.pair_codes.insert(
                    code.clone(),
                    PairCode {
                        device_id: device_id.clone(),
                        secret,
                        from_desktop,
                        exp: Instant::now() + pair_ttl,
                    },
                );
                let out = serde_json::to_string(&DeviceOut::PairCreated { code }).unwrap();
                let _ = tx.try_send(out);
            }
            DeviceInMsg::Ping => {
                let _ = tx.try_send(serde_json::to_string(&DeviceOut::Pong).unwrap());
            }
            DeviceInMsg::Data { to, payload } => {
                // 路由到指定手机连接（原样透传 payload）——必须校验目标连接
                // 归属本设备：设备注册完全开放，不校验则任何公网攻击者可注册
                // 假设备后向他人手机注入伪造输出（假冒终端内容/钓鱼）
                let client_tx = {
                    let inner = state.lock();
                    match inner.st.clients.get(&to) {
                        Some(c) if c.device_id == device_id => Some(c.tx.clone()),
                        _ => None,
                    }
                };
                if let Some(client_tx) = client_tx {
                    if let Ok(payload_text) = serde_json::to_string(&payload) {
                        let _ = client_tx.try_send(payload_text);
                    }
                }
            }
            DeviceInMsg::ClientKick { conn, reason } => {
                // 校验归属（与 Data 同规则，防跨设备踢他人连接）后：
                // 先发 hello.error（client_tx 有序，先于断连送达）再唤醒
                // kick 关闭连接；断开清理走 handle_client 统一路径
                // （client.disconnected 回发桌面，本地映射随回环清理）
                let kick = {
                    let inner = state.lock();
                    match inner.st.clients.get(&conn) {
                        Some(c) if c.device_id == device_id => {
                            let err = serde_json::json!({
                                "type": "hello.error",
                                "error": reason,
                            });
                            let _ = c.tx.try_send(err.to_string());
                            Some(c.kick.clone())
                        }
                        _ => None,
                    }
                };
                if let Some(kick) = kick {
                    kick.notify_one();
                    println!("[relay] client kicked: conn={} device={}", conn, device_id);
                }
            }
        }
    }

    // 清理：仅当仍是本连接注册时才移除（防顶替竞态）
    {
        let mut inner = state.lock();
        let still_current = inner
            .st
            .devices
            .get(&device_id_for_cleanup)
            .map(|t| t.same_channel(&tx))
            .unwrap_or(false);
        if still_current {
            inner.st.devices.remove(&device_id_for_cleanup);
        }
        inner.st.pair_codes.retain(|_, pc| pc.device_id != device_id_for_cleanup);
        // 通知该设备的手机连接：桌面离线
        let offline = r#"{"type":"device.status","online":false}"#.to_string();
        inner.st.clients.retain(|_, c| {
            if c.device_id == device_id_for_cleanup {
                let _ = c.tx.try_send(offline.clone());
                false
            } else {
                true
            }
        });
    }
    send_task.abort();
    println!("[relay] device offline: {}", device_id_for_cleanup);
}

// ==================== 手机连接 /ws ====================

async fn ws_client(
    State(state): State<SharedState>,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    ws: WebSocketUpgrade,
) -> Response {
    ws.max_message_size(MAX_WS_MESSAGE)
        .on_upgrade(move |socket| handle_client(socket, state, addr))
}

async fn handle_client(mut socket: WebSocket, state: SharedState, addr: SocketAddr) {
    // 首条必须是 hello（应用层），从中取 token 完成 relay 认证
    let first = match tokio::time::timeout(Duration::from_secs(10), socket.recv()).await {
        Ok(Some(Ok(Message::Text(t)))) => t,
        _ => return,
    };
    let Ok(parsed) = serde_json::from_str::<serde_json::Value>(&first) else {
        return;
    };
    if parsed.get("type").and_then(|v| v.as_str()) != Some("hello") {
        return;
    }
    let token = parsed
        .get("token")
        .and_then(|v| v.as_str())
        .unwrap_or("")
        .to_string();

    // token v2 路由：`<device_id>.<secret>`——恰一个点、两段各 32 hex，
    // 前缀即目标桌面。secret 校验在桌面（relay 不存凭证表）。
    // 格式非法（含 v1 无点 token，relay 升级后存量手机需重扫一次）直接拒绝
    let is_32_hex = |s: &str| s.len() == 32 && s.chars().all(|c| c.is_ascii_hexdigit());
    let device_id = match token.split_once('.') {
        Some((prefix, secret))
            if !secret.contains('.') && is_32_hex(prefix) && is_32_hex(secret) =>
        {
            prefix.to_string()
        }
        _ => {
            let _ = socket
                .send(Message::Text(r#"{"type":"hello.error","error":"invalid token"}"#.into()))
                .await;
            return;
        }
    };
    let device_tx = state.lock().st.devices.get(&device_id).cloned();
    let Some(device_tx) = device_tx else {
        let _ = socket
            .send(Message::Text(r#"{"type":"device.status","online":false}"#.into()))
            .await;
        return;
    };
    // 连接数上限（认证后仍限制：防表打爆；须在 split 拥有 socket 前应答）
    if state.lock().st.clients.len() >= MAX_CLIENTS {
        let _ = socket
            .send(Message::Text(r#"{"type":"hello.error","error":"relay busy"}"#.into()))
            .await;
        return;
    }

    let (mut ws_sender, mut ws_receiver) = socket.split();
    let (client_tx, mut client_rx) = mpsc::channel::<String>(256);
    let kick = std::sync::Arc::new(tokio::sync::Notify::new());

    let conn_id = {
        let mut inner = state.lock();
        inner.st.next_conn_id += 1;
        let conn_id = inner.st.next_conn_id;
        inner.st.clients.insert(
            conn_id,
            ClientConn {
                device_id: device_id.clone(),
                tx: client_tx.clone(),
                kick: kick.clone(),
            },
        );
        conn_id
    };

    // 手机 hello 转发桌面（透传原始消息）
    let _ = device_tx.try_send(
        serde_json::to_string(&DeviceOut::Data {
            from: conn_id,
            payload: parsed.clone(),
        })
        .unwrap(),
    );
    let _ = client_tx.try_send(r#"{"type":"device.status","online":true}"#.to_string());

    // 桥任务：本连接出站（桌面 data → 手机，原样透传）
    let bridge = tokio::spawn(async move {
        while let Some(text) = client_rx.recv().await {
            if ws_sender.send(Message::Text(text)).await.is_err() {
                break;
            }
        }
    });

    println!("[relay] client online: conn={} device={} from {}", conn_id, device_id, addr);

    // 接收循环：手机应用层消息 → 包 Data{from} 转发桌面；
    // kick.notified（桌面 client.kick 校验踢人）退出循环即关闭连接
    loop {
        let msg = tokio::select! {
            _ = kick.notified() => break,
            msg = ws_receiver.next() => match msg {
                Some(Ok(m)) => m,
                _ => break,
            },
        };
        let Message::Text(text) = msg else {
            if matches!(msg, Message::Close(_)) {
                break;
            }
            continue;
        };
        let Ok(payload) = serde_json::from_str::<serde_json::Value>(&text) else {
            continue;
        };
        let out = serde_json::to_string(&DeviceOut::Data {
            from: conn_id,
            payload,
        })
        .unwrap();
        if device_tx.try_send(out).is_err() {
            break; // 桌面离线
        }
    }

    // 清理
    state.lock().st.clients.remove(&conn_id);
    let _ = device_tx.try_send(
        serde_json::to_string(&DeviceOut::ClientDisconnected { conn: conn_id }).unwrap(),
    );
    bridge.abort();
    println!("[relay] client offline: conn={}", conn_id);
}
