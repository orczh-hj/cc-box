//! 宿主制探测（remote_server::probe_host）单测：
//! 多实例共用 8747 时，后开实例凭探测结果判定"宿主已存在"而非端口冲突报错。

use crate::remote_server;

/// 起一个最小 HTTP 服务（返回 200），probe_host 应判定宿主存在
#[tokio::test]
async fn ProbeHost_Http200Detected_001() {
    use axum::{Router, routing::get};

    // 模拟真实 index（<title>CC-Box</title>）——probe 以 "CC-Box" body 特征判定
    let app = Router::new().route("/", get(|| async { "<html><head><title>CC-Box</title></head><body>ok</body></html>" }));
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let port = listener.local_addr().unwrap().port();
    tokio::spawn(async move {
        let _ = axum::serve(listener, app).await;
    });

    // 等服务就绪后探测
    let mut ok = false;
    for _ in 0..20 {
        if remote_server::probe_host(port).await {
            ok = true;
            break;
        }
        tokio::time::sleep(std::time::Duration::from_millis(50)).await;
    }
    assert!(ok, "HTTP 200 服务应被识别为宿主");
}

/// 无监听的端口：探测必须为 false（否则多实例会误判宿主、自身永远起不来）
#[tokio::test]
async fn ProbeHost_ClosedPortNotHost_002() {
    // 绑一个端口然后立刻释放，确保它空闲
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let port = listener.local_addr().unwrap().port();
    drop(listener);

    assert!(!remote_server::probe_host(port).await, "空闲端口不应判定为宿主");
}

/// 裸 TCP 服务（accept 后不回任何数据）：探测必须受超时保护及时返回 false，
/// 不得永久挂起（否则 set_remote_enabled 命令不返回、UI 卡死）
#[tokio::test]
async fn ProbeHost_SilentTcpTimesOut_003() {
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let port = listener.local_addr().unwrap().port();
    tokio::spawn(async move {
        loop {
            let Ok((_conn, _)) = listener.accept().await else { break };
            // 持有连接不读写——模拟被非 HTTP 服务占用
        }
    });

    let start = std::time::Instant::now();
    let hosted = remote_server::probe_host(port).await;
    assert!(!hosted, "无 HTTP 响应不应判定为宿主");
    assert!(
        start.elapsed() < std::time::Duration::from_secs(6),
        "超时保护应生效（3s 超时 + 余量）"
    );
}
