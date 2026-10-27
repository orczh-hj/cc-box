//! 宿主制探测（remote_server::probe_host）单测：
//! 多实例共用 8747 时，后开实例凭探测结果判定"宿主已存在"而非端口冲突报错。

use crate::remote_server;

/// 复现 8747 孤儿端口缺陷的直接原因：Windows 上 socket 句柄默认可继承
/// （HANDLE_FLAG_INHERIT=1）——宿主实例 spawn 新实例/WebView2 子进程时
/// （bInheritHandles=TRUE）listener 句柄被连带继承，宿主退出后端口被
/// 存活子实例拖住（netstat 显示已死 PID 持有 LISTEN，bind 10048）
#[cfg(target_os = "windows")]
#[tokio::test]
async fn Listener_默认句柄可继承_复现孤儿端口_001() {
    #[link(name = "kernel32")]
    extern "system" {
        fn GetHandleInformation(object: isize, flags: *mut u32) -> i32;
    }
    const HANDLE_FLAG_INHERIT: u32 = 0x1;

    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    use std::os::windows::io::AsRawSocket;
    let mut flags = 0u32;
    let ok = unsafe { GetHandleInformation(listener.as_raw_socket() as isize, &mut flags) };
    assert_eq!(ok, 1, "GetHandleInformation 应成功");
    assert_ne!(
        flags & HANDLE_FLAG_INHERIT,
        0,
        "tokio listener 默认可继承（缺陷存在）"
    );
}

/// 修复验证：clear_socket_inherit 后句柄不可继承——spawn 子实例不再
/// 拖住宿主端口，宿主退出后端口随进程释放，新实例可正常 bind 接管
#[cfg(target_os = "windows")]
#[tokio::test]
async fn Listener_清除继承位后不可继承_002() {
    #[link(name = "kernel32")]
    extern "system" {
        fn GetHandleInformation(object: isize, flags: *mut u32) -> i32;
    }
    const HANDLE_FLAG_INHERIT: u32 = 0x1;

    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    remote_server::clear_socket_inherit(&listener);
    use std::os::windows::io::AsRawSocket;
    let mut flags = 0u32;
    let ok = unsafe { GetHandleInformation(listener.as_raw_socket() as isize, &mut flags) };
    assert_eq!(ok, 1, "GetHandleInformation 应成功");
    assert_eq!(
        flags & HANDLE_FLAG_INHERIT,
        0,
        "清除后 listener 句柄不应可继承"
    );
}

/// 辅助：起一个静默 TCP 服务（accept 后持有连接、不回任何数据），
/// 模拟端口被「可连但不响应 HTTP」的进程占用（僵死实例/残骸特征）。
/// 必须绑 0.0.0.0：Windows 通配绑定与具体地址（127.0.0.1）同端口不冲突，
/// 绑回环挡不住 remote_server 的 0.0.0.0 bind
async fn spawn_silent_tcp_owner() -> u16 {
    let listener = tokio::net::TcpListener::bind("0.0.0.0:0").await.unwrap();
    let port = listener.local_addr().unwrap().port();
    tokio::spawn(async move {
        loop {
            let Ok((_conn, _)) = listener.accept().await else { break };
        }
    });
    port
}

/// 旧版 Gateway 占口：Broker 进入 legacy-owned，不启动 LAN、不抢所有权；
/// 面板据此显示“设备级多实例控制暂不可用”。
#[tokio::test]
async fn Broker_旧版Gateway占口进入legacy_001() {
    let _lock = super::REMOTE_SERVER_TEST_LOCK.lock();
    let _cfg_lock = super::CONFIG_DIR_TEST_LOCK.lock();
    let _cfg = super::TestConfigDirGuard::new("broker-legacy");

    // 模拟旧版实例的 HTTP 服务（200 + CC-Box 特征、无 /gateway.ok 路由）
    let app = axum::Router::new().route(
        "/",
        axum::routing::get(|| async { "<html><head><title>CC-Box</title></head><body>ok</body></html>" }),
    );
    let listener = tokio::net::TcpListener::bind("0.0.0.0:0").await.unwrap();
    let port = listener.local_addr().unwrap().port();
    tokio::spawn(async move {
        let _ = axum::serve(listener, app).await;
    });
    let mut ready = false;
    for _ in 0..20 {
        if remote_server::probe_host(port).await {
            ready = true;
            break;
        }
        tokio::time::sleep(std::time::Duration::from_millis(50)).await;
    }
    assert!(ready, "模拟旧版服务应先就绪");
    remote_server::save_remote_config(Some(true), None, Some(port)).unwrap();

    crate::broker::start_remote_service().await;
    assert!(crate::broker::legacy_owned(), "旧版占口应进入 legacy-owned");
    assert!(!remote_server::is_running(), "legacy-owned 不应启动 LAN");
    assert!(crate::broker::lan_error().is_some());
    assert!(remote_server::hosted_elsewhere().await);

    crate::broker::stop_remote_service().await;
    assert!(!crate::broker::legacy_owned());
}

/// 端口被静默 TCP（可连但不响应）占用：start 应 Err 且文案区分 stale
/// （提示重启僵死实例），不得误判宿主制、不得置运行态
#[tokio::test]
async fn Start_静默占用报stale文案_002() {
    let _lock = super::REMOTE_SERVER_TEST_LOCK.lock();
    let _cfg_lock = super::CONFIG_DIR_TEST_LOCK.lock();
    let _cfg = super::TestConfigDirGuard::new("remote-silent-occupy");

    let port = spawn_silent_tcp_owner().await;
    remote_server::save_remote_config(None, None, Some(port)).unwrap();

    let err = remote_server::start().await.unwrap_err();
    assert!(
        err.contains("does not respond"),
        "静默占用应报 stale 文案，实际: {}",
        err
    );
    assert!(!remote_server::is_running());
}

/// Broker 管理操作：停止远程只写配置并停服务，不退出进程、不影响成员连接。
#[tokio::test]
async fn Broker_Control停止远程写配置_003() {
    let _lock = super::REMOTE_SERVER_TEST_LOCK.lock();
    let _cfg_lock = super::CONFIG_DIR_TEST_LOCK.lock();
    let _cfg = super::TestConfigDirGuard::new("broker-control-stop");

    fn read_enabled() -> Option<bool> {
        crate::store::get_app_config()
            .unwrap()
            .remote
            .and_then(|r| r.enabled)
    }

    let port = spawn_silent_tcp_owner().await;
    remote_server::save_remote_config(Some(true), None, Some(port)).unwrap();
    crate::broker::control(crate::gateway_proto::BrokerOp::SetEnabled { enabled: false })
        .await
        .unwrap();
    assert_eq!(read_enabled(), Some(false));
    assert!(!remote_server::is_running());
}

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

/// stop() 后端口必须立即可重绑 + 立即可再开启（start/stop 全链路）。
/// 背景：abort serve 任务后 Windows 上端口释放有延迟（IOCP pending accept
/// 清理，实测 ~100ms 起且不稳定）——曾实测 v0.15.1 实例关闭远程后端口长期
/// LISTENING 残留且不响应 HTTP，「停止→再开启」与多实例宿主接管均 10048。
/// stop 内部已轮询等待端口可重绑，此处断言对外语义：stop 返回即端口可用
#[tokio::test]
async fn RemoteServer_Stop后端口立即可重绑_001() {
    let _lock = super::REMOTE_SERVER_TEST_LOCK.lock();
    // 配置目录隔离与 remote_state 的 Rpc 写命令链测试互斥（override 全局单值）
    let _cfg_lock = super::CONFIG_DIR_TEST_LOCK.lock();
    let _cfg = super::TestConfigDirGuard::new("remote-stop");

    // 随机端口：防与真实实例（8747）及其他并行测试冲突
    let probe = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let port = probe.local_addr().unwrap().port();
    drop(probe);
    remote_server::save_remote_config(None, None, Some(port)).unwrap();

    remote_server::start().await.unwrap();
    assert!(remote_server::is_running(), "start 后应处于运行态");

    remote_server::stop().await;
    assert!(!remote_server::is_running(), "stop 后应处于停止态");

    // 断言 1：stop 返回即端口可重绑（不依赖外部等待）
    let rebound = tokio::net::TcpListener::bind(("0.0.0.0", port)).await;
    assert!(
        rebound.is_ok(),
        "stop 后端口应立即可重绑，实际: {:?}",
        rebound.err()
    );
    drop(rebound);

    // 断言 2：立即再开启也成功（「停止→开启」快速切换路径）
    remote_server::start().await.unwrap();
    assert!(remote_server::is_running());
    remote_server::stop().await;
}
