use crate::hook_events::{derive_state, extract_detail, HookPayload, HookEventDetail};
use serde_json::json;

// ---- derive_state 测试 ----

// UserPromptSubmit 映射为 thinking
#[test]
fn DeriveState_PromptSubmit_001() {
    let data = json!({});
    assert_eq!(derive_state("UserPromptSubmit", &data), "thinking");
}

// PreToolUse 映射为 tool_executing
#[test]
fn DeriveState_PreToolUse_001() {
    let data = json!({});
    assert_eq!(derive_state("PreToolUse", &data), "tool_executing");
}

// PostToolUse 映射为 thinking
#[test]
fn DeriveState_PostToolUse_001() {
    let data = json!({});
    assert_eq!(derive_state("PostToolUse", &data), "thinking");
}

// PostToolUseFailure 映射为 thinking
#[test]
fn DeriveState_PostToolFail_001() {
    let data = json!({});
    assert_eq!(derive_state("PostToolUseFailure", &data), "thinking");
}

// Stop 映射为 idle
#[test]
fn DeriveState_Stop_001() {
    let data = json!({});
    assert_eq!(derive_state("Stop", &data), "idle");
}

// StopFailure 映射为 error
#[test]
fn DeriveState_StopFail_001() {
    let data = json!({});
    assert_eq!(derive_state("StopFailure", &data), "error");
}

// Notification + worker_permission_prompt 映射为 waiting_permission
#[test]
fn DeriveState_NotificationWorkerPerm_001() {
    let data = json!({"notification_type": "worker_permission_prompt"});
    assert_eq!(derive_state("Notification", &data), "waiting_permission");
}

// Notification + permission_prompt 映射为 waiting_permission
#[test]
fn DeriveState_NotificationPerm_001() {
    let data = json!({"notification_type": "permission_prompt"});
    assert_eq!(derive_state("Notification", &data), "waiting_permission");
}

// Notification + idle_prompt 映射为 idle
#[test]
fn DeriveState_NotificationIdle_001() {
    let data = json!({"notification_type": "idle_prompt"});
    assert_eq!(derive_state("Notification", &data), "idle");
}

// Notification + computer_use_enter 映射为 unknown（不改变工作状态）
#[test]
fn DeriveState_NotificationComputerUse_001() {
    let data = json!({"notification_type": "computer_use_enter"});
    assert_eq!(derive_state("Notification", &data), "unknown");
}

// Notification + elicitation_complete 映射为 unknown
#[test]
fn DeriveState_NotificationElicitation_001() {
    let data = json!({"notification_type": "elicitation_complete"});
    assert_eq!(derive_state("Notification", &data), "unknown");
}

// Notification + 未知 notification_type 映射为 unknown
#[test]
fn DeriveState_NotificationOther_001() {
    let data = json!({"notification_type": "something_else"});
    assert_eq!(derive_state("Notification", &data), "unknown");
}

// SubagentStart 映射为 subagent_running
#[test]
fn DeriveState_SubagentStart_001() {
    let data = json!({});
    assert_eq!(derive_state("SubagentStart", &data), "subagent_running");
}

// SubagentStop 映射为 thinking
#[test]
fn DeriveState_SubagentStop_001() {
    let data = json!({});
    assert_eq!(derive_state("SubagentStop", &data), "thinking");
}

// PreCompact 映射为 compacting
#[test]
fn DeriveState_PreCompact_001() {
    let data = json!({});
    assert_eq!(derive_state("PreCompact", &data), "compacting");
}

// PostCompact 映射为 thinking
#[test]
fn DeriveState_PostCompact_001() {
    let data = json!({});
    assert_eq!(derive_state("PostCompact", &data), "thinking");
}

// SessionStart 映射为 idle
#[test]
fn DeriveState_SessionStart_001() {
    let data = json!({});
    assert_eq!(derive_state("SessionStart", &data), "idle");
}

// SessionEnd 映射为 idle
#[test]
fn DeriveState_SessionEnd_001() {
    let data = json!({});
    assert_eq!(derive_state("SessionEnd", &data), "idle");
}

// 未知事件名映射为 unknown
#[test]
fn DeriveState_UnknownEvent_001() {
    let data = json!({});
    assert_eq!(derive_state("NonExistentEvent", &data), "unknown");
}

// ---- extract_detail 测试 ----

// SessionStart 提取 model, cwd, transcript_path, source 四个字段
#[test]
fn ExtractDetail_SessionStart_001() {
    let data = json!({
        "model": "claude-sonnet-4-20250514",
        "cwd": "/home/user/project",
        "transcript_path": "/tmp/transcript.jsonl",
        "source": "cli"
    });
    let detail = extract_detail("SessionStart", &data);
    match detail {
        HookEventDetail::SessionStart(sd) => {
            assert_eq!(sd.model.as_deref(), Some("claude-sonnet-4-20250514"));
            assert_eq!(sd.cwd.as_deref(), Some("/home/user/project"));
            assert_eq!(sd.transcript_path.as_deref(), Some("/tmp/transcript.jsonl"));
            assert_eq!(sd.source.as_deref(), Some("cli"));
        }
        _ => panic!("Expected SessionStart variant"),
    }
}

// SessionEnd 提取 reason 字段
#[test]
fn ExtractDetail_SessionEnd_001() {
    let data = json!({"reason": "prompt_input_exit"});
    let detail = extract_detail("SessionEnd", &data);
    match detail {
        HookEventDetail::SessionEnd(d) => {
            assert_eq!(d.reason.as_deref(), Some("prompt_input_exit"));
        }
        _ => panic!("Expected SessionEnd variant"),
    }
}

// UserPromptSubmit 提取 prompt 字段
#[test]
fn ExtractDetail_PromptSubmit_001() {
    let data = json!({"prompt": "hello world"});
    let detail = extract_detail("UserPromptSubmit", &data);
    match detail {
        HookEventDetail::UserPromptSubmit(d) => {
            assert_eq!(d.prompt.as_deref(), Some("hello world"));
        }
        _ => panic!("Expected UserPromptSubmit variant"),
    }
}

// PreToolUse 提取 tool_name, tool_use_id
#[test]
fn ExtractDetail_PreToolUse_001() {
    let data = json!({"tool_name": "Write", "tool_use_id": "tool-123"});
    let detail = extract_detail("PreToolUse", &data);
    match detail {
        HookEventDetail::PreToolUse(d) => {
            assert_eq!(d.tool_name.as_deref(), Some("Write"));
            assert_eq!(d.tool_use_id.as_deref(), Some("tool-123"));
        }
        _ => panic!("Expected PreToolUse variant"),
    }
}

// PostToolUse 提取 tool_name, tool_use_id
#[test]
fn ExtractDetail_PostToolUse_001() {
    let data = json!({"tool_name": "Read", "tool_use_id": "tool-456"});
    let detail = extract_detail("PostToolUse", &data);
    match detail {
        HookEventDetail::PostToolUse(d) => {
            assert_eq!(d.tool_name.as_deref(), Some("Read"));
            assert_eq!(d.tool_use_id.as_deref(), Some("tool-456"));
        }
        _ => panic!("Expected PostToolUse variant"),
    }
}

// PostToolUseFailure 提取 tool_name, error, is_interrupt
#[test]
fn ExtractDetail_PostToolFail_001() {
    let data = json!({
        "tool_name": "Bash",
        "tool_use_id": "tool-789",
        "error": "command failed",
        "is_interrupt": true
    });
    let detail = extract_detail("PostToolUseFailure", &data);
    match detail {
        HookEventDetail::PostToolUseFailure(d) => {
            assert_eq!(d.tool_name.as_deref(), Some("Bash"));
            assert_eq!(d.tool_use_id.as_deref(), Some("tool-789"));
            assert_eq!(d.error.as_deref(), Some("command failed"));
            assert_eq!(d.is_interrupt, Some(true));
        }
        _ => panic!("Expected PostToolUseFailure variant"),
    }
}

// Stop 提取 stop_hook_active, last_assistant_message
#[test]
fn ExtractDetail_Stop_001() {
    let data = json!({
        "stop_hook_active": true,
        "last_assistant_message": "Done!"
    });
    let detail = extract_detail("Stop", &data);
    match detail {
        HookEventDetail::Stop(d) => {
            assert_eq!(d.stop_hook_active, Some(true));
            assert_eq!(d.last_assistant_message.as_deref(), Some("Done!"));
        }
        _ => panic!("Expected Stop variant"),
    }
}

// StopFailure 提取 error
#[test]
fn ExtractDetail_StopFail_001() {
    let data = json!({"error": "API error"});
    let detail = extract_detail("StopFailure", &data);
    match detail {
        HookEventDetail::StopFailure(d) => {
            assert_eq!(d.error.as_deref(), Some("API error"));
        }
        _ => panic!("Expected StopFailure variant"),
    }
}

// Notification 提取 notification_type, message, title
#[test]
fn ExtractDetail_Notification_001() {
    let data = json!({
        "notification_type": "idle_prompt",
        "message": "waiting for input",
        "title": "Claude"
    });
    let detail = extract_detail("Notification", &data);
    match detail {
        HookEventDetail::Notification(d) => {
            assert_eq!(d.notification_type.as_deref(), Some("idle_prompt"));
            assert_eq!(d.message.as_deref(), Some("waiting for input"));
            assert_eq!(d.title.as_deref(), Some("Claude"));
        }
        _ => panic!("Expected Notification variant"),
    }
}

// SubagentStart 提取 agent_id, agent_type
#[test]
fn ExtractDetail_SubagentStart_001() {
    let data = json!({"agent_id": "agent-abc", "agent_type": "Explore"});
    let detail = extract_detail("SubagentStart", &data);
    match detail {
        HookEventDetail::SubagentStart(d) => {
            assert_eq!(d.agent_id.as_deref(), Some("agent-abc"));
            assert_eq!(d.agent_type.as_deref(), Some("Explore"));
        }
        _ => panic!("Expected SubagentStart variant"),
    }
}

// SubagentStop 提取 agent_id, agent_type
#[test]
fn ExtractDetail_SubagentStop_001() {
    let data = json!({"agent_id": "agent-xyz", "agent_type": "general-purpose"});
    let detail = extract_detail("SubagentStop", &data);
    match detail {
        HookEventDetail::SubagentStop(d) => {
            assert_eq!(d.agent_id.as_deref(), Some("agent-xyz"));
            assert_eq!(d.agent_type.as_deref(), Some("general-purpose"));
        }
        _ => panic!("Expected SubagentStop variant"),
    }
}

// PreCompact 提取 trigger
#[test]
fn ExtractDetail_PreCompact_001() {
    let data = json!({"trigger": "auto"});
    let detail = extract_detail("PreCompact", &data);
    match detail {
        HookEventDetail::PreCompact(d) => {
            assert_eq!(d.trigger.as_deref(), Some("auto"));
        }
        _ => panic!("Expected PreCompact variant"),
    }
}

// PostCompact 提取 trigger
#[test]
fn ExtractDetail_PostCompact_001() {
    let data = json!({"trigger": "manual"});
    let detail = extract_detail("PostCompact", &data);
    match detail {
        HookEventDetail::PostCompact(d) => {
            assert_eq!(d.trigger.as_deref(), Some("manual"));
        }
        _ => panic!("Expected PostCompact variant"),
    }
}

// 未知事件名返回 Unknown 变体保留原始 JSON
#[test]
fn ExtractDetail_Unknown_001() {
    let data = json!({"foo": "bar", "num": 42});
    let detail = extract_detail("CustomEvent", &data);
    match detail {
        HookEventDetail::Unknown(v) => {
            assert_eq!(v["foo"], "bar");
            assert_eq!(v["num"], 42);
        }
        _ => panic!("Expected Unknown variant"),
    }
}

// CLI 上报原始 JSON 文本（含转义引号/换行/伪嵌套 JSON/中文/emoji）解码为原始文本，
// 字符串字段不做二次解析——锚定 stdin 直转链路的转义还原
#[test]
fn ExtractDetail_StopEscapedText_001() {
    let raw = concat!(
        r#"{"hook_event_name":"Stop","stop_hook_active":false,"#,
        r#""last_assistant_message":"第一行\n第二行 \"quoted\" {\"nested\":true} 中文🎉"}"#,
    );
    let data: serde_json::Value = serde_json::from_str(raw).unwrap();
    let detail = extract_detail("Stop", &data);
    match detail {
        HookEventDetail::Stop(d) => {
            assert_eq!(d.stop_hook_active, Some(false));
            assert_eq!(
                d.last_assistant_message.as_deref(),
                Some("第一行\n第二行 \"quoted\" {\"nested\":true} 中文🎉")
            );
        }
        _ => panic!("Expected Stop variant"),
    }
}

// UserPromptSubmit 的 prompt 含多字节中文与 emoji，从原始上报文本解码不截断
#[test]
fn ExtractDetail_PromptMultibyte_001() {
    let raw = r#"{"hook_event_name":"UserPromptSubmit","prompt":"重构 store.rs 并验证 🚀 完成"}"#;
    let data: serde_json::Value = serde_json::from_str(raw).unwrap();
    let detail = extract_detail("UserPromptSubmit", &data);
    match detail {
        HookEventDetail::UserPromptSubmit(d) => {
            assert_eq!(d.prompt.as_deref(), Some("重构 store.rs 并验证 🚀 完成"));
        }
        _ => panic!("Expected UserPromptSubmit variant"),
    }
}

// ---- HookPayload::from_raw 测试 ----

// 从 JSON 中提取 hook_event_name 字段
#[test]
fn FromRaw_EventName_001() {
    let data = json!({"hook_event_name": "PreToolUse"});
    let payload = HookPayload::from_raw(None, data);
    assert_eq!(payload.event_name, "PreToolUse");
}

// 从 JSON 中提取 session_id 字段
#[test]
fn FromRaw_SessionId_001() {
    let data = json!({"hook_event_name": "Stop", "session_id": "sess-abc123"});
    let payload = HookPayload::from_raw(None, data);
    assert_eq!(payload.session_id.as_deref(), Some("sess-abc123"));
}

// 缺少 hook_event_name 时默认为 "unknown"
#[test]
fn FromRaw_DefaultName_001() {
    let data = json!({"session_id": "sess-xyz"});
    let payload = HookPayload::from_raw(None, data);
    assert_eq!(payload.event_name, "unknown");
}
