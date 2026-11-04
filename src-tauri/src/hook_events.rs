use serde::Serialize;
use serde_json::Value;

/// 发送给前端的完整 hook 事件 payload
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct HookPayload {
    pub pty_id: Option<String>,
    pub session_id: Option<String>,
    pub event_name: String,
    pub state: String,
    pub timestamp: i64,
    pub detail: HookEventDetail,
}

/// 各事件类型的结构化详情
#[derive(Debug, Clone, Serialize)]
#[serde(tag = "type", content = "data", rename_all = "camelCase")]
pub enum HookEventDetail {
    SessionStart(SessionStartData),
    SessionEnd(SessionEndData),
    UserPromptSubmit(UserPromptSubmitData),
    PreToolUse(PreToolUseData),
    PostToolUse(PostToolUseData),
    PostToolUseFailure(PostToolUseFailureData),
    Stop(StopData),
    StopFailure(StopFailureData),
    Notification(NotificationData),
    SubagentStart(SubagentStartData),
    SubagentStop(SubagentStopData),
    PreCompact(PreCompactData),
    PostCompact(PostCompactData),
    /// 未识别事件，保留原始 JSON
    Unknown(Value),
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SessionStartData {
    pub model: Option<String>,
    pub cwd: Option<String>,
    pub transcript_path: Option<String>,
    pub source: Option<String>,
    /// 会话官方标题（/resume 切换的历史会话自带；旧版 CLI 无此字段为 None）
    pub session_title: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SessionEndData {
    pub reason: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UserPromptSubmitData {
    pub prompt: Option<String>,
    /// 会话官方标题（用户显式命名时每条 prompt 携带；旧版 CLI 为 None）
    pub session_title: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PreToolUseData {
    pub tool_name: Option<String>,
    pub tool_use_id: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PostToolUseData {
    pub tool_name: Option<String>,
    pub tool_use_id: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PostToolUseFailureData {
    pub tool_name: Option<String>,
    pub tool_use_id: Option<String>,
    pub error: Option<String>,
    pub is_interrupt: Option<bool>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct StopData {
    pub stop_hook_active: Option<bool>,
    pub last_assistant_message: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct StopFailureData {
    pub error: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NotificationData {
    pub notification_type: Option<String>,
    pub message: Option<String>,
    pub title: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SubagentStartData {
    pub agent_id: Option<String>,
    pub agent_type: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SubagentStopData {
    pub agent_id: Option<String>,
    pub agent_type: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PreCompactData {
    pub trigger: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PostCompactData {
    pub trigger: Option<String>,
}

// ---- 提取逻辑 ----

impl HookPayload {
    pub fn from_raw(pty_id: Option<String>, event: Value) -> Self {
        let event_name = event
            .get("hook_event_name")
            .and_then(|v| v.as_str())
            .unwrap_or("unknown")
            .to_string();

        let session_id = str_field(&event, "session_id");
        let state = derive_state(&event_name, &event);
        let detail = extract_detail(&event_name, &event);

        Self {
            pty_id,
            session_id,
            event_name,
            state,
            timestamp: chrono::Utc::now().timestamp_millis(),
            detail,
        }
    }
}

fn str_field(v: &Value, key: &str) -> Option<String> {
    v.get(key).and_then(|v| v.as_str()).map(|s| s.to_string())
}

fn bool_field(v: &Value, key: &str) -> Option<bool> {
    v.get(key).and_then(|v| v.as_bool())
}

pub(crate) fn extract_detail(event_name: &str, event: &Value) -> HookEventDetail {
    match event_name {
        "SessionStart" => HookEventDetail::SessionStart(SessionStartData {
            model: str_field(event, "model"),
            cwd: str_field(event, "cwd"),
            transcript_path: str_field(event, "transcript_path"),
            source: str_field(event, "source"),
            session_title: str_field(event, "session_title"),
        }),
        "SessionEnd" => HookEventDetail::SessionEnd(SessionEndData {
            reason: str_field(event, "reason"),
        }),
        "UserPromptSubmit" => HookEventDetail::UserPromptSubmit(UserPromptSubmitData {
            prompt: str_field(event, "prompt"),
            session_title: str_field(event, "session_title"),
        }),
        "PreToolUse" => HookEventDetail::PreToolUse(PreToolUseData {
            tool_name: str_field(event, "tool_name"),
            tool_use_id: str_field(event, "tool_use_id"),
        }),
        "PostToolUse" => HookEventDetail::PostToolUse(PostToolUseData {
            tool_name: str_field(event, "tool_name"),
            tool_use_id: str_field(event, "tool_use_id"),
        }),
        "PostToolUseFailure" => HookEventDetail::PostToolUseFailure(PostToolUseFailureData {
            tool_name: str_field(event, "tool_name"),
            tool_use_id: str_field(event, "tool_use_id"),
            error: str_field(event, "error"),
            is_interrupt: bool_field(event, "is_interrupt"),
        }),
        "Stop" => HookEventDetail::Stop(StopData {
            stop_hook_active: bool_field(event, "stop_hook_active"),
            last_assistant_message: str_field(event, "last_assistant_message"),
        }),
        "StopFailure" => HookEventDetail::StopFailure(StopFailureData {
            error: str_field(event, "error"),
        }),
        "Notification" => HookEventDetail::Notification(NotificationData {
            notification_type: str_field(event, "notification_type"),
            message: str_field(event, "message"),
            title: str_field(event, "title"),
        }),
        "SubagentStart" => HookEventDetail::SubagentStart(SubagentStartData {
            agent_id: str_field(event, "agent_id"),
            agent_type: str_field(event, "agent_type"),
        }),
        "SubagentStop" => HookEventDetail::SubagentStop(SubagentStopData {
            agent_id: str_field(event, "agent_id"),
            agent_type: str_field(event, "agent_type"),
        }),
        "PreCompact" => HookEventDetail::PreCompact(PreCompactData {
            trigger: str_field(event, "trigger"),
        }),
        "PostCompact" => HookEventDetail::PostCompact(PostCompactData {
            trigger: str_field(event, "trigger"),
        }),
        _ => HookEventDetail::Unknown(event.clone()),
    }
}

pub(crate) fn derive_state(event_name: &str, event: &Value) -> String {
    match event_name {
        "UserPromptSubmit" => "thinking".into(),
        "PreToolUse" => "tool_executing".into(),
        "PostToolUse" | "PostToolUseFailure" => "thinking".into(),
        "Stop" => "idle".into(),
        "StopFailure" => "error".into(),
        "Notification" => {
            let ntype = str_field(event, "notification_type").unwrap_or_default();
            match ntype.as_str() {
                "permission_prompt" | "worker_permission_prompt" => "waiting_permission",
                "idle_prompt" => "idle",
                _ => "unknown",
            }
            .into()
        }
        "SubagentStart" => "subagent_running".into(),
        "SubagentStop" => "thinking".into(),
        "PreCompact" => "compacting".into(),
        "PostCompact" => "thinking".into(),
        "SessionStart" => "idle".into(),
        "SessionEnd" => "idle".into(),
        _ => "unknown".into(),
    }
}
