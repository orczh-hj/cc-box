//! 中继出站客户端：连接 cc-box-relay（公网 VPS），把手机经中继的消息交给
//! remote_state 的同一套 handler（局域网直连与中继是同一条应用层代码路径）。
//!
//! 桥接模型：每个手机中继连接（relay 分配的 conn_id）注册为 REMOTE_STATE 的
//! 一个连接，出站消息经桥任务封装为中继层 `data{to}` 帧原路回程。
//!
//! 生命周期：RELAY_GEN（watch 代际）控制 run_loop 存活——启停/换 URL 时代际
//! +1，旧 loop（含在途 WS 连接与全部桥接）立即退出，杜绝停用后残留连接。

use std::collections::HashMap;
use std::sync::LazyLock;
use std::time::{Duration, Instant};

use futures_util::{SinkExt, StreamExt};
use parking_lot::Mutex;
use tokio::sync::{mpsc, watch};
use tokio_tungstenite::tungstenite::Message;

use cc_box_remote_protocol::{ClientMessage, ServerMessage};

use crate::remote_state::{next_conn_id, REMOTE_STATE};

/// 桥接的手机中继连接：relay conn_id → 本地 conn_id。
/// 本地 id 来自全局分配器（与局域网共用同一命名空间），避免两种模式
/// 同时启用时 add_conn 相互覆盖 / remove_conn 误回收对方持有的会话。
static BRIDGED_CONNS: LazyLock<Mutex<HashMap<u64, u64>>> =
    LazyLock::new(|| Mutex::new(HashMap::new()));

/// run_loop 代际：每次启停/换 URL +1，旧 loop 感知变化后整体退出
static RELAY_GEN: LazyLock<(watch::Sender<u64>, watch::Receiver<u64>)> =
    LazyLock::new(|| watch::channel(0));

/// 中继层 keepalive 周期（须小于 nginx proxy_read_timeout 默认 60s）
const KEEPALIVE: Duration = Duration::from_secs(25);

/// 内置默认中继（生产部署）：8800 由 nginx TLS 终止（手机浏览器对 http 强制
/// 升级 https，纯 HTTP 端口打不开）——中继即默认远程方式，无需用户配置；
/// 环境变量 CC_BOX_RELAY_URL 仅作本地调试覆盖
const DEFAULT_RELAY_URL: &str = "wss://orczh.top:8800";

/// 设备出站通道（run_once 注册成功后挂载，断开时摘除）：
/// 供按需重申配对码等设备侧主动请求使用
static DEVICE_TX: LazyLock<Mutex<Option<mpsc::Sender<String>>>> =
    LazyLock::new(|| Mutex::new(None));

/// 解析中继地址：环境变量 CC_BOX_RELAY_URL > 内置默认 wss://orczh.top:8800。
/// （config 的 relay_url 是历史字段，不再读取——避免旧残留覆盖内置默认）
pub(crate) fn resolve_relay_url() -> String {
    std::env::var("CC_BOX_RELAY_URL")
        .ok()
        .map(|u| normalize_relay_url(u.trim()))
        .filter(|u| !u.is_empty())
        .unwrap_or_else(|| normalize_relay_url(DEFAULT_RELAY_URL))
}

/// 发送 pair.create；中继未连接（DEVICE_TX 未挂载）或通道关闭时报错
fn send_pair_create() -> Result<(), String> {
    let Some(tx) = DEVICE_TX.lock().clone() else {
        return Err("relay not connected".into());
    };
    tx.try_send(r#"{"type":"pair.create"}"#.to_string())
        .map_err(|_| "relay channel closed".to_string())
}

/// 按需重申配对码：当前码缺失/过期（面板显示中继视图但无码可扫）时，
/// 由 get_remote_status 触发；结果经 pair.created 异步写入状态，面板轮询拿到
pub fn request_pair_code() {
    let _ = send_pair_create();
}

/// 手动重申配对码并等待新码到达（面板「更新连接码」按钮）。
/// 配对码一次性消费：设备 A 配对成功后旧码即失效，且本地状态仍留着已
/// 消费的旧码（非空）导致按需重申不触发——靠此入口立即为设备 B 重申新码。
/// 返回新码；中继未连接或超时内未收到 pair.created 报错
pub async fn refresh_pair_code() -> Result<String, String> {
    let old_code = RELAY.lock().pair_code.clone();
    send_pair_create()?;
    // 等 run_once 接收循环把 pair.created 写入状态（50ms 轮询，3s 超时）
    let deadline = Instant::now() + Duration::from_secs(3);
    loop {
        let (code, at) = {
            let st = RELAY.lock();
            (st.pair_code.clone(), st.pair_code_at)
        };
        let fresh = at.is_some_and(|t| t.elapsed() < Duration::from_secs(5 * 60));
        if fresh && code != old_code {
            return Ok(code);
        }
        if Instant::now() >= deadline {
            return Err("relay did not return a new pair code".into());
        }
        tokio::time::sleep(Duration::from_millis(50)).await;
    }
}

/// 中继运行状态（设置页展示）
static RELAY: LazyLock<Mutex<RelayStatus>> = LazyLock::new(|| {
    Mutex::new(RelayStatus {
        enabled: false,
        connected: false,
        url: String::new(),
        pair_code: String::new(),
        pair_code_at: None,
    })
});

pub struct RelayStatus {
    pub enabled: bool,
    pub connected: bool,
    pub url: String,
    pub pair_code: String,
    pub pair_code_at: Option<Instant>,
}

/// 当前中继状态快照
pub fn status_snapshot() -> (bool, bool, String, String) {
    let st = RELAY.lock();
    // 配对码 5 分钟过期
    let code = match st.pair_code_at {
        Some(at) if at.elapsed() < Duration::from_secs(5 * 60) => st.pair_code.clone(),
        _ => String::new(),
    };
    (st.enabled, st.connected, st.url.clone(), code)
}

/// 取消当前所有 run_loop（代际 +1）。启停/换 URL 前调用。
fn bump_generation() {
    RELAY_GEN.0.send_modify(|g| *g += 1);
}

/// 仅断开运行时中继连接，不修改 config（"停止远程访问"联动：
/// 停止后手机不得再经中继控制；环境变量配置的中继在下次重启后恢复自动连接）
pub fn disconnect() {
    bump_generation();
    let mut st = RELAY.lock();
    st.enabled = false;
    st.connected = false;
    log::info!("[remote] relay disconnected (remote access stopped)");
}

/// 配置/启停中继（url 为空 = 停用）。返回错误仅当配置写入失败。
pub async fn set_relay(url: Option<String>) -> Result<(), String> {
    // 先取消旧 loop（含在途 WS 连接——停用后手机必须立即失去控制权）
    bump_generation();

    let Some(url) = url.filter(|u| !u.trim().is_empty()) else {
        // 停用
        {
            let mut st = RELAY.lock();
            st.enabled = false;
            st.connected = false;
            st.url.clear();
        }
        crate::remote_server::save_relay_config(None, None).map_err(|e| e.to_string())?;
        log::info!("[remote] relay disabled");
        return Ok(());
    };

    let url = normalize_relay_url(url.trim());
    // 宿主制：非宿主实例不注册中继（同 device_id 互踢），见 init() 注释
    if crate::remote_server::hosted_elsewhere().await {
        return Err("remote hosted by another instance; relay not started".into());
    }
    let (device_id, device_key) = ensure_device_identity()?;

    crate::remote_server::save_relay_config(Some(&url), None).map_err(|e| e.to_string())?;

    {
        let mut st = RELAY.lock();
        st.enabled = true;
        st.url = url.clone();
    }

    log::info!("[remote] relay enabled: {}", url);
    tokio::spawn(run_loop(url, device_id, device_key));
    Ok(())
}

/// 应用启动时调用：远程总开关开启则连中继。
/// 地址优先级：环境变量 `CC_BOX_RELAY_URL`（本地调试覆盖，仅本次运行生效、
/// 不落盘）> 内置默认 `orczh.top:8800`（生产部署，中继即默认远程方式）
pub async fn init() {
    let enabled = crate::store::get_app_config()
        .ok()
        .and_then(|c| c.remote)
        .and_then(|r| r.enabled)
        .unwrap_or(false);
    if !enabled {
        return;
    }
    connect().await;
}

/// 连接中继（幂等：已连接则跳过）。远程开启（set_remote_enabled）与启动恢复调用
pub async fn connect() {
    if RELAY.lock().connected {
        return;
    }
    // 防重入：若上一轮 loop 尚在连接中，整体替换（并发双 loop 会同 device_id 互踢）
    bump_generation();
    // 宿主制：本机已有其他实例提供远程服务时跳过中继连接——
    // 多实例共享 config 中的 device_id，同时注册同一中继会互相顶替（互踢）
    if crate::remote_server::hosted_elsewhere().await {
        log::info!("[remote] relay skipped: remote hosted by another instance");
        return;
    }
    let url = resolve_relay_url();
    let (device_id, device_key) = match ensure_device_identity() {
        Ok(id) => id,
        Err(e) => {
            log::error!("[remote] {}", e);
            return;
        }
    };
    {
        let mut st = RELAY.lock();
        st.enabled = true;
        st.url = url.clone();
    }
    log::info!("[remote] relay enabled: {}", url);
    tokio::spawn(run_loop(url, device_id, device_key));
}

/// 轮换设备身份（安全阻断语义）：旧 device_id 永不再上线——已配对手机的
/// access_token 在 relay 侧映射到旧 device_id（查在线表 miss）→ 全部失效；
/// 新身份注册后用户重扫新配对码即可
pub fn rotate_identity() {
    let device_id = uuid::Uuid::new_v4().simple().to_string();
    let device_key = format!(
        "{}{}",
        uuid::Uuid::new_v4().simple(),
        uuid::Uuid::new_v4().simple()
    );
    let _ = crate::remote_server::save_relay_config(None, Some((&device_id, &device_key)));
    log::info!("[remote] device identity rotated (paired phones invalidated)");
}

/// 读取/生成设备身份（device_id + device_key 持久化于 config）。
/// config 读取失败时报错而非静默生成新身份——新身份会让 relay 侧旧
/// access_token 全部失配（已配对手机静默失效）
fn ensure_device_identity() -> Result<(String, String), String> {
    let config = crate::store::get_app_config()
        .map_err(|e| format!("failed to read config for device identity: {}", e))?;
    let remote = config.remote.clone().unwrap_or_default();
    let device_id = remote
        .device_id
        .filter(|s| !s.is_empty())
        .unwrap_or_else(|| uuid::Uuid::new_v4().simple().to_string());
    let device_key = remote
        .device_key
        .filter(|s| s.len() == 64)
        .unwrap_or_else(|| {
            format!(
                "{}{}",
                uuid::Uuid::new_v4().simple(),
                uuid::Uuid::new_v4().simple()
            )
        });
    let _ = crate::remote_server::save_relay_config(None, Some((&device_id, &device_key)));
    Ok((device_id, device_key))
}

/// 中继地址归一化：显式 scheme 优先（https→wss、http→ws）；
/// 裸地址（如 `orczh.top:8800`）默认 **ws://**——带端口的裸地址几乎总是
/// 直连 relay 进程（无 nginx/TLS），免证书部署
pub(crate) fn normalize_relay_url(url: &str) -> String {
    let url = url
        .trim_end_matches('/')
        .trim_end_matches("/ws/device")
        .to_string();
    if url.starts_with("ws://") || url.starts_with("wss://") {
        url
    } else if url.starts_with("https://") {
        format!("wss://{}", &url[8..])
    } else if url.starts_with("http://") {
        format!("ws://{}", &url[7..])
    } else {
        format!("ws://{}", url)
    }
}

/// 一轮连接的结束原因
enum RoundEnd {
    /// 被取消（启停/换 URL）：整个 loop 退出
    Cancelled,
    /// 被同 device_id 的新连接顶替：重连只会互相踢，整个 loop 退出
    Replaced,
    /// 普通断线：退避后重连
    Disconnected,
}

/// 连接循环：断线指数退避重连；代际变化或被顶替时整体退出
async fn run_loop(url: String, device_id: String, device_key: String) {
    let mut gen_rx = RELAY_GEN.1.clone();
    let my_gen = *gen_rx.borrow_and_update();
    let mut backoff = 1u64;

    loop {
        if *gen_rx.borrow_and_update() != my_gen {
            break;
        }
        let end = run_once(&url, &device_id, &device_key, &mut gen_rx).await;
        // 清桥：所有桥接的手机连接失效（手机会经 relay 自动重连）
        for (_, local_id) in BRIDGED_CONNS.lock().drain() {
            REMOTE_STATE.remove_conn(local_id);
        }
        RELAY.lock().connected = false;

        match end {
            RoundEnd::Cancelled => {
                log::info!("[remote] relay loop cancelled (config changed)");
                break;
            }
            RoundEnd::Replaced => {
                log::warn!("[remote] relay loop stopped: replaced by another connection with same device_id");
                break;
            }
            RoundEnd::Disconnected => {
                log::info!("[remote] relay disconnected, retry in {}s", backoff);
            }
        }

        // 退避等待（可被取消打断）
        tokio::select! {
            _ = gen_rx.changed() => {
                if *gen_rx.borrow_and_update() != my_gen {
                    break;
                }
            }
            _ = tokio::time::sleep(Duration::from_secs(backoff)) => {}
        }
        backoff = (backoff * 2).min(30);
    }
}

/// 一轮连接：注册 → keepalive 收发循环
async fn run_once(
    url: &str,
    device_id: &str,
    device_key: &str,
    gen_rx: &mut watch::Receiver<u64>,
) -> RoundEnd {
    let ws_url = format!("{}/ws/device", url);
    let ws = match tokio_tungstenite::connect_async(&ws_url).await {
        Ok((ws, _)) => ws,
        Err(e) => {
            log::warn!("[remote] relay connect failed: {}", e);
            return RoundEnd::Disconnected;
        }
    };
    log::info!("[remote] relay connected: {}", ws_url);

    let (mut sink, mut stream) = ws.split();

    // 注册
    let register = serde_json::json!({
        "type": "register",
        "deviceId": device_id,
        "deviceKey": device_key,
    });
    if sink.send(Message::Text(register.to_string())).await.is_err() {
        return RoundEnd::Disconnected;
    }

    // 出站通道：桥任务把中继帧统一交给写任务
    let (out_tx, mut out_rx) = mpsc::channel::<String>(256);
    let write_task = tokio::spawn(async move {
        while let Some(text) = out_rx.recv().await {
            if sink.send(Message::Text(text)).await.is_err() {
                break;
            }
        }
    });

    // 收包：等 registered
    let first = tokio::time::timeout(Duration::from_secs(10), stream.next()).await;
    let registered_text = match first {
        Ok(Some(Ok(Message::Text(t)))) => t,
        _ => {
            write_task.abort();
            return RoundEnd::Disconnected;
        }
    };
    let v: serde_json::Value = match serde_json::from_str(&registered_text) {
        Ok(v) => v,
        Err(_) => {
            write_task.abort();
            return RoundEnd::Disconnected;
        }
    };
    match v["type"].as_str() {
        Some("registered") => {}
        Some("register.failed") => {
            // device_key 冲突等致命配置错误：重连无意义
            log::error!("[remote] relay register rejected: {}", registered_text);
            write_task.abort();
            RELAY.lock().enabled = false;
            return RoundEnd::Cancelled;
        }
        _ => {
            write_task.abort();
            return RoundEnd::Disconnected;
        }
    }

    {
        let mut st = RELAY.lock();
        st.connected = true;
    }
    // 申请配对码
    let _ = out_tx.try_send(r#"{"type":"pair.create"}"#.to_string());
    // 挂载设备出站通道（按需重申配对码等设备侧主动请求经此发送）
    *DEVICE_TX.lock() = Some(out_tx.clone());
    log::info!("[remote] relay registered, device_id={}", device_id);

    // 接收循环：keepalive 定时 ping（防反代掐断空闲连接），select 代际取消
    let mut ping = tokio::time::interval(KEEPALIVE);
    ping.set_missed_tick_behavior(tokio::time::MissedTickBehavior::Delay);
    let mut end = RoundEnd::Disconnected;

    loop {
        tokio::select! {
            _ = ping.tick() => {
                if out_tx.send(r#"{"type":"ping"}"#.to_string()).await.is_err() {
                    break;
                }
            }
            _ = gen_rx.changed() => {
                end = RoundEnd::Cancelled;
                break;
            }
            msg = stream.next() => {
                let msg = match msg {
                    Some(Ok(m)) => m,
                    _ => break,
                };
                let Message::Text(text) = msg else {
                    if matches!(msg, Message::Close(_)) {
                        break;
                    }
                    continue;
                };
                let Ok(v) = serde_json::from_str::<serde_json::Value>(&text) else {
                    continue;
                };
                match v["type"].as_str() {
                    Some("pair.created") => {
                        let code = v["code"].as_str().unwrap_or("").to_string();
                        // 配对码敏感，日志只留前 3 位（完整码经 UI 展示/二维码给出）
                        log::info!("[remote] relay pair code: {}***", &code[..3.min(code.len())]);
                        let mut st = RELAY.lock();
                        st.pair_code = code;
                        st.pair_code_at = Some(Instant::now());
                    }
                    Some("data") => {
                        let from = v["from"].as_u64().unwrap_or(0);
                        if from == 0 {
                            continue;
                        }
                        // 首见连接 → 分配本地 conn_id（全局命名空间，与局域网共用）
                        let (local_id, first_seen) = {
                            let mut bridged = BRIDGED_CONNS.lock();
                            match bridged.get(&from) {
                                Some(&id) => (id, false),
                                None => {
                                    let id = next_conn_id();
                                    bridged.insert(from, id);
                                    (id, true)
                                }
                            }
                        };
                        if first_seen {
                            // 建桥：REMOTE_STATE 出站消息封装为中继层 data{to} 帧回程
                            let out_tx = out_tx.clone();
                            let (tx, mut rx) = mpsc::channel::<ServerMessage>(
                                crate::remote_state::CONN_CHANNEL_CAPACITY,
                            );
                            REMOTE_STATE.add_conn(local_id, tx, true);
                            tokio::spawn(async move {
                                while let Some(msg) = rx.recv().await {
                                    let Ok(payload) = serde_json::to_value(&msg) else { continue };
                                    let frame = serde_json::json!({
                                        "type": "data",
                                        "to": from,
                                        "payload": payload,
                                    });
                                    if out_tx.send(frame.to_string()).await.is_err() {
                                        break;
                                    }
                                }
                            });
                            log::info!("[remote] relay client {} bridged (local {})", from, local_id);
                        }
                        // 应用层消息：与局域网直连同一条处理路径
                        if let Ok(client_msg) =
                            serde_json::from_value::<ClientMessage>(v["payload"].clone())
                        {
                            REMOTE_STATE.handle_client_message(local_id, &client_msg);
                        }
                    }
                    Some("client.disconnected") => {
                        let conn = v["conn"].as_u64().unwrap_or(0);
                        if conn != 0 {
                            if let Some(local_id) = BRIDGED_CONNS.lock().remove(&conn) {
                                REMOTE_STATE.remove_conn(local_id);
                                log::info!("[remote] relay client {} disconnected", conn);
                            }
                        }
                    }
                    Some("replaced") => {
                        end = RoundEnd::Replaced;
                        break;
                    }
                    _ => {} // registered / pong / 未知类型
                }
            }
        }
    }

    write_task.abort();
    *DEVICE_TX.lock() = None;
    end
}

#[cfg(test)]
mod tests {
    use super::*;

    /// DEVICE_TX/RELAY 全局单例：测试必须串行（并行时挂载/清空互相踩踏）
    static TEST_LOCK: LazyLock<Mutex<()>> = LazyLock::new(|| Mutex::new(()));

    /// 测试后恢复全局态，勿污染其他测试
    fn reset_globals() {
        *DEVICE_TX.lock() = None;
        let mut st = RELAY.lock();
        st.pair_code.clear();
        st.pair_code_at = None;
    }

    /// 中继未连接（DEVICE_TX 未挂载）→ 立即报错，不进等待循环
    #[tokio::test]
    async fn RefreshPairCode_未连接报错() {
        let _g = TEST_LOCK.lock();
        reset_globals();
        let err = refresh_pair_code().await.unwrap_err();
        assert_eq!(err, "relay not connected");
        reset_globals();
    }

    /// 设备 A 消费一次性配对码后手动重申：模拟 relay 往返（接收端收到
    /// pair.create 后延迟写入 pair.created 新码）→ 等待逻辑返回新码
    #[tokio::test]
    async fn RefreshPairCode_新码到达返回新码() {
        let _g = TEST_LOCK.lock();
        reset_globals();
        // 旧码已被设备 A 消费，但本地状态仍保留（本 bug 的卡死状态）
        {
            let mut st = RELAY.lock();
            st.pair_code = "OLDCODE1".into();
            st.pair_code_at = Some(Instant::now());
        }
        // 模拟 run_once 接收循环：DEVICE_TX 挂载，收到 pair.create 后写新码
        let (tx, mut rx) = mpsc::channel::<String>(8);
        *DEVICE_TX.lock() = Some(tx);
        tokio::spawn(async move {
            let _ = rx.recv().await; // pair.create 到达
            tokio::time::sleep(Duration::from_millis(100)).await;
            let mut st = RELAY.lock();
            st.pair_code = "NEWCODE99".into();
            st.pair_code_at = Some(Instant::now());
        });

        let code = refresh_pair_code().await.expect("new pair code");
        assert_eq!(code, "NEWCODE99");
        reset_globals();
    }
}
