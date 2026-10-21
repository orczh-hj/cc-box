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
    /// hook 状态机单点计算的 working/pending（Rust apply_hook_event 唯一实现，
    /// 桌面/手机前端仅消费此事件应用状态，不再各自复刻状态机）
    pub const HOOK_STATUS: &str = "hook-status";
    pub const SESSION_TAKEN_OVER: &str = "session-taken-over";
    pub const SESSION_RECLAIMED: &str = "session-reclaimed";
    pub const REMOTE_SESSION_CREATED: &str = "remote-session-created";
    /// 用户显式关闭会话（桌面 pty_kill / 远程 session.kill 双路径统一广播）：
    /// 各端联动删 Tab。与自然退出（pty-exit 只置 stopped 保留 Tab）相区分
    pub const REMOTE_SESSION_CLOSED: &str = "remote-session-closed";
    pub const SESSION_RENAMED: &str = "session-renamed";
    pub const DOWNLOAD_PROGRESS: &str = "download-progress";
    /// 写命令统一状态广播（payload {id: section, data: revision}；全端按 section 刷新）
    pub const STATE_CHANGED: &str = "state-changed";
    /// 全局控制权变更（payload {id: connId 或空串, data: revision}；远程端显示/解除全局遮罩）
    pub const CONTROLLER_CHANGED: &str = "controller-changed";
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
