//! relay 集成测试：内存起 server + 双 WS 客户端（桌面 /ws/device + 手机 /ws）
//! 覆盖设计文档第 11 节承诺：配对→注册→路由→令牌生命周期、配对码过期/一次性、离线通知、顶替

use std::time::Duration;

use futures_util::{SinkExt, StreamExt};
use serde_json::{Value, json};
use tokio::net::TcpStream;
use tokio_tungstenite::tungstenite::Message;
use tokio_tungstenite::{MaybeTlsStream, WebSocketStream, connect_async};

type Ws = WebSocketStream<MaybeTlsStream<TcpStream>>;

/// 测试用短 TTL（配对码过期场景）
const SHORT_TTL: Duration = Duration::from_millis(80);
/// 合法 device_key（64 hex）
const KEY_A: &str = "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef";
const KEY_B: &str = "fedcba9876543210fedcba9876543210fedcba9876543210fedcba9876543210";

/// 内存起 relay（随机端口），返回端口
async fn spawn_relay(pair_ttl: Duration) -> u16 {
    let state = cc_box_relay::new_state(pair_ttl);
    let app = cc_box_relay::build_router(state)
        .into_make_service_with_connect_info::<std::net::SocketAddr>();
    let listener = tokio::net::TcpListener::bind(("127.0.0.1", 0)).await.unwrap();
    let port = listener.local_addr().unwrap().port();
    tokio::spawn(async move {
        axum::serve(listener, app).await.unwrap();
    });
    port
}

async fn ws_connect(port: u16, path: &str) -> Ws {
    let (ws, _) = connect_async(format!("ws://127.0.0.1:{}{}", port, path))
        .await
        .expect("ws connect");
    ws
}

async fn send(ws: &mut Ws, v: Value) {
    ws.send(Message::Text(v.to_string())).await.unwrap();
}

async fn recv_json(ws: &mut Ws) -> Value {
    let msg = tokio::time::timeout(Duration::from_secs(3), ws.next())
        .await
        .expect("recv timeout")
        .expect("stream closed")
        .expect("ws error");
    match msg {
        Message::Text(t) => serde_json::from_str(&t).unwrap(),
        other => panic!("unexpected frame: {:?}", other),
    }
}

/// 收到指定 type 的消息（跳过中间其他消息，如 device.status）
async fn recv_of_type(ws: &mut Ws, ty: &str) -> Value {
    for _ in 0..20 {
        let v = recv_json(ws).await;
        if v["type"] == ty {
            return v;
        }
    }
    panic!("expected message type {} not received within 20 frames", ty);
}

/// 原始 HTTP POST（避免引入 HTTP client 依赖），返回 body
async fn http_post(port: u16, path: &str, body: &str) -> String {
    use tokio::io::{AsyncReadExt, AsyncWriteExt};
    let mut s = TcpStream::connect(("127.0.0.1", port)).await.unwrap();
    let req = format!(
        "POST {} HTTP/1.1\r\nHost: t\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}",
        path,
        body.len(),
        body
    );
    s.write_all(req.as_bytes()).await.unwrap();
    let mut buf = String::new();
    s.read_to_string(&mut buf).await.unwrap();
    buf.split("\r\n\r\n").nth(1).unwrap_or("").to_string()
}

/// 建立已注册的桌面连接
async fn device_online(port: u16, device_id: &str, key: &str) -> Ws {
    let mut ws = ws_connect(port, "/ws/device").await;
    send(
        &mut ws,
        json!({"type": "register", "deviceId": device_id, "deviceKey": key}),
    )
    .await;
    let v = recv_of_type(&mut ws, "registered").await;
    assert_eq!(v["deviceId"], device_id);
    ws
}

/// 桌面在线 + 已配对，返回 (桌面 ws, access_token)
async fn paired_device(port: u16, device_id: &str) -> (Ws, String) {
    let mut dev = device_online(port, device_id, KEY_A).await;
    send(&mut dev, json!({"type": "pair.create"})).await;
    let v = recv_of_type(&mut dev, "pair.created").await;
    let code = v["code"].as_str().unwrap().to_string();
    let resp = http_post(port, "/api/pair", &json!({"code": code}).to_string()).await;
    let resp: Value = serde_json::from_str(&resp).unwrap();
    assert_eq!(resp["ok"], true, "pair should succeed: {}", resp);
    let token = resp["token"].as_str().unwrap().to_string();
    (dev, token)
}

/// 手机带 token 连接并完成 hello 透传，返回 (手机 ws, conn_id)
async fn client_online(port: u16, token: &str, dev: &mut Ws) -> (Ws, u64) {
    let mut ph = ws_connect(port, "/ws").await;
    send(
        &mut ph,
        json!({"type": "hello", "ver": 1, "client": "web", "token": token}),
    )
    .await;
    let v = recv_of_type(&mut ph, "device.status").await;
    assert_eq!(v["online"], true);
    let v = recv_of_type(dev, "data").await;
    assert_eq!(v["payload"]["type"], "hello");
    let conn = v["from"].as_u64().unwrap();
    (ph, conn)
}

// ==================== 测试用例 ====================

/// 完整链路：注册 → 配对码 → 换 token → 手机接入 → hello 透传 → 双向应用层路由
#[tokio::test]
async fn Relay_FullFlow_RegisterPairRoute_1() {
    let port = spawn_relay(cc_box_relay::PAIR_TTL).await;
    let (mut dev, token) = paired_device(port, "dev-1").await;

    // 手机接入（hello 已透传到桌面）
    let (mut ph, conn) = client_online(port, &token, &mut dev).await;
    assert_eq!(conn, 1);

    // 桌面 → 手机：data 路由到指定 conn（payload 原样透传）
    send(
        &mut dev,
        json!({"type": "data", "to": conn, "payload": {"type": "hello.ok", "ver": 1, "deviceName": "Test-Desktop"}}),
    )
    .await;
    let v = recv_of_type(&mut ph, "hello.ok").await;
    assert_eq!(v["deviceName"], "Test-Desktop");

    // 手机 → 桌面：应用层消息包装为 data{from} 透传
    send(&mut ph, json!({"type": "sessions.list"})).await;
    let v = recv_of_type(&mut dev, "data").await;
    assert_eq!(v["from"], conn);
    assert_eq!(v["payload"]["type"], "sessions.list");
}

/// device_key 校验：形态非法拒绝；同 device_id 重连 key 不匹配拒绝
#[tokio::test]
async fn Relay_DeviceKey_Validation_2() {
    let port = spawn_relay(cc_box_relay::PAIR_TTL).await;

    // 形态非法（非 64 hex）
    let mut bad = ws_connect(port, "/ws/device").await;
    send(
        &mut bad,
        json!({"type": "register", "deviceId": "dev-2", "deviceKey": "short"}),
    )
    .await;
    let v = recv_of_type(&mut bad, "register.failed").await;
    assert_eq!(v["reason"], "bad device_key");

    // 首次合法注册（绑定 KEY_A）
    let _dev = device_online(port, "dev-2", KEY_A).await;

    // 同 device_id 不同 key → mismatch
    let mut intruder = ws_connect(port, "/ws/device").await;
    send(
        &mut intruder,
        json!({"type": "register", "deviceId": "dev-2", "deviceKey": KEY_B}),
    )
    .await;
    let v = recv_of_type(&mut intruder, "register.failed").await;
    assert_eq!(v["reason"], "device_key mismatch");
}

/// 无效 token：手机 hello 被拒（hello.error）
#[tokio::test]
async fn Relay_Token_Invalid_3() {
    let port = spawn_relay(cc_box_relay::PAIR_TTL).await;
    let mut ph = ws_connect(port, "/ws").await;
    send(
        &mut ph,
        json!({"type": "hello", "ver": 1, "client": "web", "token": "not-a-real-token"}),
    )
    .await;
    let v = recv_of_type(&mut ph, "hello.error").await;
    assert_eq!(v["error"], "invalid token");
}

/// 配对码生命周期：过期失效 + 一次性（同码不可复用）
#[tokio::test]
async fn Relay_PairCode_TtlAndOneShot_4() {
    let port = spawn_relay(SHORT_TTL).await;
    let mut dev = device_online(port, "dev-4", KEY_A).await;

    // 过期
    send(&mut dev, json!({"type": "pair.create"})).await;
    let code = recv_of_type(&mut dev, "pair.created").await["code"]
        .as_str()
        .unwrap()
        .to_string();
    tokio::time::sleep(SHORT_TTL * 2).await;
    let resp = http_post(port, "/api/pair", &json!({"code": code}).to_string()).await;
    let resp: Value = serde_json::from_str(&resp).unwrap();
    assert_eq!(resp["ok"], false);
    assert_eq!(resp["error"], "invalid or expired code");

    // 一次性：新码第一次成功，第二次同码失败
    send(&mut dev, json!({"type": "pair.create"})).await;
    let code = recv_of_type(&mut dev, "pair.created").await["code"]
        .as_str()
        .unwrap()
        .to_string();
    let resp = http_post(port, "/api/pair", &json!({"code": &code}).to_string()).await;
    assert_eq!(serde_json::from_str::<Value>(&resp).unwrap()["ok"], true);
    let resp = http_post(port, "/api/pair", &json!({"code": code}).to_string()).await;
    let resp: Value = serde_json::from_str(&resp).unwrap();
    assert_eq!(resp["ok"], false, "pair code must be one-shot");
}

/// 桌面离线：在线手机收到 device.status offline；此后手机接入收到 offline（token 仍有效）
#[tokio::test]
async fn Relay_DeviceOffline_NotifyClients_5() {
    let port = spawn_relay(cc_box_relay::PAIR_TTL).await;
    let (mut dev, token) = paired_device(port, "dev-5").await;
    let (mut ph, _) = client_online(port, &token, &mut dev).await;

    // 桌面断开
    dev.close(None).await.unwrap();
    let v = recv_of_type(&mut ph, "device.status").await;
    assert_eq!(v["online"], false);

    // 桌面离线期间手机接入：token 有效但设备不在线
    let mut ph2 = ws_connect(port, "/ws").await;
    send(
        &mut ph2,
        json!({"type": "hello", "ver": 1, "client": "web", "token": token}),
    )
    .await;
    let v = recv_of_type(&mut ph2, "device.status").await;
    assert_eq!(v["online"], false);
}

/// 手机断开：桌面收到 client.disconnected{conn}
#[tokio::test]
async fn Relay_ClientDisconnect_NotifyDevice_6() {
    let port = spawn_relay(cc_box_relay::PAIR_TTL).await;
    let (mut dev, token) = paired_device(port, "dev-6").await;
    let (mut ph, conn) = client_online(port, &token, &mut dev).await;

    ph.close(None).await.unwrap();
    let v = recv_of_type(&mut dev, "client.disconnected").await;
    assert_eq!(v["conn"], conn);
}

/// 桌面重连顶替：同 device_id 同 key 新连接注册成功，旧连接收到 replaced
#[tokio::test]
async fn Relay_DeviceReplace_OldConnectionReplaced_7() {
    let port = spawn_relay(cc_box_relay::PAIR_TTL).await;
    let mut old = device_online(port, "dev-7", KEY_A).await;

    let _new = device_online(port, "dev-7", KEY_A).await;

    let v = recv_of_type(&mut old, "replaced").await;
    assert_eq!(v["type"], "replaced");
}

/// 配对码强度：8 位 hex（16^8 组合空间，配合 TTL 与失败锁定抗暴力猜解）
#[tokio::test]
async fn Relay_PairCode_Strength_8() {
    let port = spawn_relay(cc_box_relay::PAIR_TTL).await;
    let mut dev = device_online(port, "dev-8", KEY_A).await;

    send(&mut dev, json!({"type": "pair.create"})).await;
    let v = recv_of_type(&mut dev, "pair.created").await;
    let code = v["code"].as_str().unwrap();
    assert_eq!(code.len(), 8, "pair code must be 8 hex chars");
    assert!(code.chars().all(|c| c.is_ascii_hexdigit()), "pair code must be hex");
}

/// /api/pair 失败锁定：同 IP 连续失败达到阈值后，正确配对码也被暂时拒绝
#[tokio::test]
async fn Relay_PairFailLockout_9() {
    let port = spawn_relay(cc_box_relay::PAIR_TTL).await;
    let mut dev = device_online(port, "dev-9", KEY_A).await;

    // 现取一个正确码（不使用）
    send(&mut dev, json!({"type": "pair.create"})).await;
    let code = recv_of_type(&mut dev, "pair.created").await["code"]
        .as_str()
        .unwrap()
        .to_string();

    // 连续 5 次错误码
    for _ in 0..5 {
        let resp = http_post(port, "/api/pair", &json!({"code": "deadbeef"}).to_string()).await;
        let resp: Value = serde_json::from_str(&resp).unwrap();
        assert_eq!(resp["ok"], false);
    }

    // 第 6 次即使码正确也被锁定拒绝
    let resp = http_post(port, "/api/pair", &json!({"code": code}).to_string()).await;
    let resp: Value = serde_json::from_str(&resp).unwrap();
    assert_eq!(resp["ok"], false);
    assert_eq!(resp["error"], "too many attempts, try later");
}
