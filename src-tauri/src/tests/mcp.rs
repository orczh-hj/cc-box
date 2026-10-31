use crate::mcp::{
    parse_capabilities, parse_prompts, parse_resources, parse_server_info, parse_sse_response,
    parse_tools,
};
use serde_json::json;

// ==================== parse_sse_response ====================

// 解析含 data: 前缀的有效 SSE 行返回 Ok
#[test]
fn ParseSse_ValidData_001() {
    let body = r#"data:{"jsonrpc":"2.0","id":1,"result":{"tools":[]}}"#;
    let response = parse_sse_response(body).unwrap();
    assert_eq!(response.id, 1);
    assert!(response.result.is_some());
}

// data 中包含 error 对象时返回 Err
#[test]
fn ParseSse_RpcError_001() {
    let body = r#"data:{"jsonrpc":"2.0","id":2,"error":{"code":-32600,"message":"Invalid Request"}}"#;
    let result = parse_sse_response(body);
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("RPC error"));
}

// 无 data: 前缀的输入返回 Err
#[test]
fn ParseSse_NoDataLine_001() {
    let body = r#"event:message
id:1
{"jsonrpc":"2.0","id":1,"result":{}}"#;
    let result = parse_sse_response(body);
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("No data line"));
}

// 多行 SSE 中正确找到 data 行
#[test]
fn ParseSse_MultiLine_001() {
    let body = "id:1\nevent:message\ndata:{\"jsonrpc\":\"2.0\",\"id\":1,\"result\":{\"status\":\"ok\"}}\n\n";
    let response = parse_sse_response(body).unwrap();
    assert_eq!(response.id, 1);
    assert_eq!(response.result.unwrap()["status"], "ok");
}

// data: 后有空格仍正确解析
#[test]
fn ParseSse_SpacedPrefix_001() {
    let body = r#"data: {"jsonrpc":"2.0","id":3,"result":{}}"#;
    let response = parse_sse_response(body).unwrap();
    assert_eq!(response.id, 3);
}

// ==================== parse_server_info ====================

// 从 serverInfo 提取 name=test, version=1.0
#[test]
fn ParseServerInfo_Complete_001() {
    let result = json!({
        "serverInfo": {"name": "test", "version": "1.0"}
    });
    let info = parse_server_info(&result).unwrap();
    assert_eq!(info.name, "test");
    assert_eq!(info.version, "1.0");
}

// 缺少 serverInfo 字段返回 None
#[test]
fn ParseServerInfo_Missing_001() {
    let result = json!({"capabilities": {}});
    assert!(parse_server_info(&result).is_none());
}

// serverInfo 中只有 name 时 version 为空字符串
#[test]
fn ParseServerInfo_Partial_001() {
    let result = json!({"serverInfo": {"name": "partial"}});
    let info = parse_server_info(&result).unwrap();
    assert_eq!(info.name, "partial");
    assert_eq!(info.version, "");
}

// ==================== parse_capabilities ====================

// capabilities 含 tools 字段时 tools=true
#[test]
fn ParseCap_Tools_001() {
    let result = json!({"capabilities": {"tools": {}}});
    let caps = parse_capabilities(&result);
    assert!(caps.tools);
    assert!(!caps.prompts);
    assert!(!caps.resources);
}

// capabilities 含 prompts 字段时 prompts=true
#[test]
fn ParseCap_Prompts_001() {
    let result = json!({"capabilities": {"prompts": {}}});
    let caps = parse_capabilities(&result);
    assert!(!caps.tools);
    assert!(caps.prompts);
    assert!(!caps.resources);
}

// capabilities 含 resources 字段时 resources=true
#[test]
fn ParseCap_Resources_001() {
    let result = json!({"capabilities": {"resources": {}}});
    let caps = parse_capabilities(&result);
    assert!(!caps.tools);
    assert!(!caps.prompts);
    assert!(caps.resources);
}

// 缺少 capabilities 时三个字段均为 false
#[test]
fn ParseCap_Missing_001() {
    let result = json!({});
    let caps = parse_capabilities(&result);
    assert!(!caps.tools);
    assert!(!caps.prompts);
    assert!(!caps.resources);
}

// ==================== parse_tools ====================

// 解析含 name 和 description 的工具列表
#[test]
fn ParseTools_List_001() {
    let response = serde_json::from_value(json!({
        "jsonrpc": "2.0",
        "id": 1,
        "result": {
            "tools": [
                {"name": "read_file", "description": "Read a file"},
                {"name": "write_file", "description": "Write a file"}
            ]
        }
    }))
    .unwrap();
    let tools = parse_tools(&response);
    assert_eq!(tools.len(), 2);
    assert_eq!(tools[0].name, "read_file");
    assert_eq!(tools[0].description, Some("Read a file".to_string()));
    assert_eq!(tools[1].name, "write_file");
    assert_eq!(tools[1].description, Some("Write a file".to_string()));
}

// 工具缺少 description 时为 None
#[test]
fn ParseTools_NoDesc_001() {
    let response = serde_json::from_value(json!({
        "jsonrpc": "2.0",
        "id": 1,
        "result": {
            "tools": [{"name": "tool_a"}]
        }
    }))
    .unwrap();
    let tools = parse_tools(&response);
    assert_eq!(tools.len(), 1);
    assert_eq!(tools[0].name, "tool_a");
    assert!(tools[0].description.is_none());
}

// 工具缺少 inputSchema 时为 None
#[test]
fn ParseTools_NoSchema_001() {
    let response = serde_json::from_value(json!({
        "jsonrpc": "2.0",
        "id": 1,
        "result": {
            "tools": [{"name": "tool_b", "description": "desc"}]
        }
    }))
    .unwrap();
    let tools = parse_tools(&response);
    assert_eq!(tools.len(), 1);
    assert!(tools[0].input_schema.is_none());
}

// 无 tools 字段时返回空 Vec
#[test]
fn ParseTools_Empty_001() {
    let response = serde_json::from_value(json!({
        "jsonrpc": "2.0",
        "id": 1,
        "result": {}
    }))
    .unwrap();
    let tools = parse_tools(&response);
    assert!(tools.is_empty());
}

// ==================== parse_prompts ====================

// 解析含 arguments 数组的 prompt
#[test]
fn ParsePrompts_WithArgs_001() {
    let response = serde_json::from_value(json!({
        "jsonrpc": "2.0",
        "id": 1,
        "result": {
            "prompts": [{
                "name": "greet",
                "description": "Greet someone",
                "arguments": [
                    {"name": "target", "description": "Who to greet", "required": true}
                ]
            }]
        }
    }))
    .unwrap();
    let prompts = parse_prompts(&response);
    assert_eq!(prompts.len(), 1);
    assert_eq!(prompts[0].name, "greet");
    let args = prompts[0].arguments.as_ref().unwrap();
    assert_eq!(args.len(), 1);
    assert_eq!(args[0].name, "target");
    assert_eq!(args[0].description, Some("Who to greet".to_string()));
    assert!(args[0].required);
}

// prompt 缺少 arguments 时为 None
#[test]
fn ParsePrompts_NoArgs_001() {
    let response = serde_json::from_value(json!({
        "jsonrpc": "2.0",
        "id": 1,
        "result": {
            "prompts": [{"name": "simple_prompt", "description": "No args"}]
        }
    }))
    .unwrap();
    let prompts = parse_prompts(&response);
    assert_eq!(prompts.len(), 1);
    assert!(prompts[0].arguments.is_none());
}

// argument 中 required 字段正确提取
#[test]
fn ParsePrompts_Required_001() {
    let response = serde_json::from_value(json!({
        "jsonrpc": "2.0",
        "id": 1,
        "result": {
            "prompts": [{
                "name": "p",
                "arguments": [
                    {"name": "opt_arg", "required": false},
                    {"name": "req_arg", "required": true},
                    {"name": "no_req"}
                ]
            }]
        }
    }))
    .unwrap();
    let prompts = parse_prompts(&response);
    let args = prompts[0].arguments.as_ref().unwrap();
    assert!(!args[0].required);
    assert!(args[1].required);
    assert!(!args[2].required);
}

// ==================== parse_resources ====================

// 解析含 uri 和 name 的资源列表
#[test]
fn ParseRes_List_001() {
    let response = serde_json::from_value(json!({
        "jsonrpc": "2.0",
        "id": 1,
        "result": {
            "resources": [
                {"uri": "file:///a.txt", "name": "a", "mimeType": "text/plain"},
                {"uri": "file:///b.json", "name": "b", "mimeType": "application/json"}
            ]
        }
    }))
    .unwrap();
    let resources = parse_resources(&response);
    assert_eq!(resources.len(), 2);
    assert_eq!(resources[0].uri, "file:///a.txt");
    assert_eq!(resources[0].name, "a");
    assert_eq!(resources[0].mime_type, Some("text/plain".to_string()));
    assert_eq!(resources[1].uri, "file:///b.json");
    assert_eq!(resources[1].mime_type, Some("application/json".to_string()));
}

// 资源缺少 mimeType 时为 None
#[test]
fn ParseRes_NoMime_001() {
    let response = serde_json::from_value(json!({
        "jsonrpc": "2.0",
        "id": 1,
        "result": {
            "resources": [{"uri": "file:///c.txt", "name": "c"}]
        }
    }))
    .unwrap();
    let resources = parse_resources(&response);
    assert_eq!(resources.len(), 1);
    assert!(resources[0].mime_type.is_none());
}

// 无 resources 字段时返回空 Vec
#[test]
fn ParseRes_Empty_001() {
    let response = serde_json::from_value(json!({
        "jsonrpc": "2.0",
        "id": 1,
        "result": {}
    }))
    .unwrap();
    let resources = parse_resources(&response);
    assert!(resources.is_empty());
}

/// Windows `cmd /S /C` 命令行拼装：含空格的命令/参数必须加引号——
/// 曾经 `C:/Program Files (x86)/.../msedge.exe` 被按空格拆成多个参数
/// （cmd 报 "too many arguments"，playwright 详情获取失败）
#[cfg(target_os = "windows")]
#[test]
fn WindowsCmdLine_含空格token加引号() {
    use crate::mcp::windows_cmd_line;

    // 手算真值：空格 token 加引号，无空格原样
    let line = windows_cmd_line(
        "cmd",
        &["/c".to_string(), "C:/Program Files (x86)/Microsoft/Edge/Application/msedge.exe".to_string()],
    );
    assert_eq!(
        line,
        "cmd /c \"C:/Program Files (x86)/Microsoft/Edge/Application/msedge.exe\""
    );

    // 全部无空格：原样拼装
    assert_eq!(
        windows_cmd_line("npx", &["-y".to_string(), "@playwright/mcp@latest".to_string()]),
        "npx -y @playwright/mcp@latest"
    );

    // 空参数：引号包裹
    assert_eq!(windows_cmd_line("node", &[String::new()]), "node \"\"");
}

/// cmd 元字符转义：无空格但含 &|" 的 token 也必须加引号/转义，
/// 否则 `a&b` 会被 cmd 当命令分隔执行（命令注入面，来源含项目级 .mcp.json）
#[cfg(target_os = "windows")]
#[test]
fn WindowsCmdLine_元字符token转义() {
    use crate::mcp::windows_cmd_line;

    // 手算真值：& 触发加引号；内层 " 转义为 \"
    assert_eq!(
        windows_cmd_line("node", &["a&b".to_string(), "echo".to_string()]),
        "node \"a&b\" echo"
    );
    assert_eq!(
        windows_cmd_line("node", &["he\"llo|x".to_string()]),
        "node \"he\\\"llo|x\""
    );
    // 纯净 token 原样
    assert_eq!(windows_cmd_line("npx", &["-y".to_string()]), "npx -y");
}
