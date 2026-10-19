//! cc-box-relay 入口：环境变量 CC_BOX_RELAY_PORT（默认 8800），绑定 127.0.0.1（nginx 反代）
//! 全部逻辑在 lib.rs（便于集成测试内存起 server）

use std::net::SocketAddr;

use cc_box_relay::{PAIR_TTL, build_router, new_state};

#[tokio::main]
async fn main() {
    let port: u16 = std::env::var("CC_BOX_RELAY_PORT")
        .ok()
        .and_then(|p| p.parse().ok())
        .unwrap_or(8800);
    // 默认 127.0.0.1（生产经 nginx 反代）；本地真机测试可设 CC_BOX_RELAY_BIND=0.0.0.0
    let bind: std::net::IpAddr = std::env::var("CC_BOX_RELAY_BIND")
        .ok()
        .and_then(|b| b.parse().ok())
        .unwrap_or(std::net::IpAddr::from([127, 0, 0, 1]));

    let state = new_state(PAIR_TTL);
    let app = build_router(state);

    let addr = SocketAddr::from((bind, port));
    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .expect("bind failed (is the port in use?)");
    println!("[relay] cc-box-relay listening on http://{}", addr);

    axum::serve(listener, app.into_make_service_with_connect_info::<SocketAddr>())
        .await
        .unwrap();
}
