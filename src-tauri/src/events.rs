//! 事件单一出口（统一管理：桌面 Tauri emit 与远程浏览器端转发二合一）
//!
//! 所有"桌面 UI 与浏览器 UI 都需要"的控制类事件一律经 `emit_all` 发出：
//! 内部同时 Tauri emit（桌面前端）+ REMOTE_STATE.broadcast_event（浏览器端经
//! rpc.event 收取，payload 与 Tauri 事件同构）。
//!
//! 注意：数据流（pty.output / pty.exit）**不走这里**——pty.output 走远程协议
//! 专用 PtyOutput 消息（attach 即订阅、保序），在 pty.rs 的 push_output 链路。
//!
//! 新增事件的检查清单：事件名常量在此登记 → emit 点改用 emit_all →
//! 前端 transport.listen 名字与常量一致。

#![allow(dead_code)]

/// 事件名常量（前端 listen 字符串与此处必须一致）
pub mod names {
    pub const HOOK_EVENT: &str = "hook-event";
    pub const SESSION_TAKEN_OVER: &str = "session-taken-over";
    pub const SESSION_RECLAIMED: &str = "session-reclaimed";
    pub const REMOTE_SESSION_CREATED: &str = "remote-session-created";
    pub const DOWNLOAD_PROGRESS: &str = "download-progress";
}

/// 控制类事件统一出口：Tauri emit + 远程 rpc.event 广播
/// （#[cfg(not(test))] 同 lib.rs EVENT_HOOK 门控：emit 闭包拉入 comctl32 v6 导入，
/// 测试 exe 无 side-by-side manifest）
#[cfg(not(test))]
pub fn emit_all(app: &tauri::AppHandle, name: &str, payload: serde_json::Value) {
    use tauri::Emitter;
    let _ = app.emit(name, payload.clone());
    crate::remote_state::REMOTE_STATE.broadcast_event(name, payload);
}

/// 测试构建：仅远程广播（无 Tauri emit；remote_state 测试依赖此路径验证转发）
#[cfg(test)]
pub fn emit_all(_app: &tauri::AppHandle, name: &str, payload: serde_json::Value) {
    crate::remote_state::REMOTE_STATE.broadcast_event(name, payload);
}
