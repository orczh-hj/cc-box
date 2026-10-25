use axum::body::Bytes;
use axum::extract::Json;
use axum::http::HeaderMap;
use axum::{routing::post, Router};
use serde_json::{json, Value};
use std::net::SocketAddr;
use tauri::AppHandle;
use tokio::sync::Mutex;
use tower::ServiceBuilder;

use crate::hook_events::HookPayload;
use crate::store::invalidate_project_path_mapping;

/// session_id → pty_id 映射
type SessionMap = Mutex<std::collections::HashMap<String, String>>;

/// 同步存储端口
static HOOK_PORT: std::sync::OnceLock<u16> = std::sync::OnceLock::new();

/// 异步会话映射
static SESSIONS: once_cell::sync::Lazy<SessionMap> =
    once_cell::sync::Lazy::new(|| Mutex::new(std::collections::HashMap::new()));

pub async fn init(app_handle: AppHandle) {
    match start_server(app_handle).await {
        Ok(port) => {
            HOOK_PORT.set(port).ok();
            log::info!("Hook server started on port {}", port);
        }
        Err(e) => {
            log::error!(
                "Failed to start hook server: {}. CC-Box will continue without hook monitoring.",
                e
            );
        }
    }
}

async fn start_server(app_handle: AppHandle) -> Result<u16, String> {
    let app = Router::new()
        .route("/hook", post(handle_hook))
        .layer(ServiceBuilder::new());

    let addr = SocketAddr::from(([127, 0, 0, 1], 0));
    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .map_err(|e| format!("bind failed: {}", e))?;
    // 与 remote_server 的 8747 listener 同理：清掉默认可继承位，防被
    // spawn 出的 CLI/WebView2 子进程拖住（随机端口无 bind 冲突，但属
    // 同类句柄泄漏，且未来若固定端口即复现 8747 孤儿端口问题）
    crate::remote_server::clear_socket_inherit(&listener);

    let port = listener
        .local_addr()
        .map_err(|e| format!("local_addr: {}", e))?
        .port();

    tokio::spawn(async move {
        let app = app.layer(axum::extract::Extension(app_handle));
        if let Err(e) = axum::serve(listener, app).await {
            log::error!("Hook server error: {}", e);
        }
    });

    Ok(port)
}

pub fn get_port() -> Option<u16> {
    HOOK_PORT.get().copied()
}

async fn handle_hook(
    headers: HeaderMap,
    axum::extract::Extension(app_handle): axum::extract::Extension<AppHandle>,
    body: Bytes,
) -> Json<Value> {
    // 从原始字节转换为 UTF-8 字符串（处理可能的编码问题）
    let body_str = match String::from_utf8(body.to_vec()) {
        Ok(s) => s,
        Err(e) => {
            log::warn!("[hook-server] UTF-8 decode error: {}", e);
            // 使用 lossy 转换作为 fallback
            String::from_utf8_lossy(&body).into_owned()
        }
    };

    let event: Value = match serde_json::from_str(&body_str) {
        Ok(v) => v,
        Err(e) => {
            log::warn!("[hook-server] JSON parse error: {} (body length={})", e, body_str.len());
            return Json(json!({}));
        }
    };

    // 从 header 获取 PTY ID
    let pty_id = headers
        .get("X-CC-Box-Session")
        .and_then(|v| v.to_str().ok())
        .filter(|s| !s.is_empty())
        .map(|s| s.to_string());

    let event_name = event
        .get("hook_event_name")
        .and_then(|v| v.as_str())
        .unwrap_or("unknown")
        .to_string();
    log::info!("[hook-server] received: {} pty={}", event_name, pty_id.as_deref().unwrap_or("?"));

    // 建立 session_id ↔ pty_id 映射；同步记入远程会话表（手机端
    // sessions.list 下发，防止从历史恢复时二次 spawn 同一对话）
    if let Some(ref sid) = event.get("session_id").and_then(|v| v.as_str()).map(|s| s.to_string()) {
        if let Some(ref pty) = pty_id {
            let mut sessions = SESSIONS.lock().await;
            sessions.insert(sid.to_string(), pty.clone());
            crate::remote_state::REMOTE_STATE.set_session_id(pty, sid);
        }
    }

    // 统一提取结构化数据并发送
    let payload = HookPayload::from_raw(pty_id, event);

    // SessionStart 时 invalidate 项目路径缓存（确保新项目会话可见）
    if event_name == "SessionStart" {
        invalidate_project_path_mapping();
        log::info!("[hook-server] SessionStart received, invalidated project path cache");
    }

    // 事件单一出口：桌面 emit + 浏览器端 rpc.event 广播（payload 与 Tauri 事件同构）
    if let Ok(payload_json) = serde_json::to_value(&payload) {
        crate::events::emit_all(&app_handle, crate::events::names::HOOK_EVENT, payload_json);
    }

    // 状态机单点消费（Rust 唯一实现）：计算 working/pending 并以 hook-status
    // 事件双端下发（桌面/手机前端的 Tab 状态唯一来源，不再各自复刻状态机；
    // SessionStatus 手机列表页广播由 on_hook_payload 内部按状态变化触发）
    if let Some(status) = crate::remote_state::REMOTE_STATE.on_hook_payload(&payload) {
        let trigger = serde_json::to_value(&payload.detail).unwrap_or(serde_json::Value::Null);
        let status_json = serde_json::json!({
            "ptyId": status.pty_id,
            "working": status.working,
            "pending": status.pending,
            "sessionId": payload.session_id,
            // trigger 供前端做 UI 决策（userPromptSubmit 自动命名 / sessionStart 分配 sessionId）
            "trigger": trigger,
        });
        crate::events::emit_all(&app_handle, crate::events::names::HOOK_STATUS, status_json);
    }

    Json(json!({}))
}
