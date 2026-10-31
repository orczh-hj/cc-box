use serde_json::json;

use crate::store::{
    expand_env_vars, extract_md_description, extract_session_name, infer_server_type,
    merge_json_values, parse_agents_list_output, parse_mcp_server_entry, parse_skill_description,
    parse_timestamp, search_session_messages_in_dirs, set_agent_enabled_in,
    set_mcp_server_enabled_in, set_skill_enabled_in, AgentInfo,
};

use std::collections::HashMap;

// ==================== merge_json_values ====================

// updates 新增 key 到 base: merge({"a":1}, {"b":2}) 包含 a 和 b
#[test]
fn MergeJson_NewKey_001() {
    let base = json!({"a": 1});
    let updates = json!({"b": 2});
    let result = merge_json_values(base, updates);
    assert_eq!(result["a"], 1);
    assert_eq!(result["b"], 2);
}

// updates 覆盖已有 key: merge({"a":1}, {"a":2}) → {"a":2}
#[test]
fn MergeJson_Overwrite_001() {
    let base = json!({"a": 1});
    let updates = json!({"a": 2});
    let result = merge_json_values(base, updates);
    assert_eq!(result["a"], 2);
    assert_eq!(result.as_object().unwrap().len(), 1);
}

// null 值 update 删除 base 中对应的 key: merge({"a":1,"b":2}, {"a":null}) → {"b":2}
#[test]
fn MergeJson_NullDelete_001() {
    let base = json!({"a": 1, "b": 2});
    let updates = json!({"a": null});
    let result = merge_json_values(base, updates);
    assert_eq!(result.as_object().unwrap().len(), 1);
    assert_eq!(result["b"], 2);
    assert!(result.get("a").is_none());
}

// 非 object 的 updates 替换整个 base: merge({"a":1}, "text") → "text"
#[test]
fn MergeJson_PrimitiveReplace_001() {
    let base = json!({"a": 1});
    let updates = json!("text");
    let result = merge_json_values(base, updates);
    assert_eq!(result, json!("text"));
}

// 空 updates 返回 base 不变: merge({"a":1}, {}) → {"a":1}
#[test]
fn MergeJson_EmptyUpdate_001() {
    let base = json!({"a": 1});
    let updates = json!({});
    let result = merge_json_values(base, updates);
    assert_eq!(result["a"], 1);
    assert_eq!(result.as_object().unwrap().len(), 1);
}

// ==================== parse_mcp_server_entry ====================

// 解析 stdio server：带 command/args/env
#[test]
fn ParseMcpEntry_StdioServer_001() {
    let config = json!({
        "command": "npx",
        "args": ["-y", "chrome-devtools-mcp@latest"],
        "env": { "CHROME_PATH": "/usr/bin/chrome" }
    });
    let result = parse_mcp_server_entry("chrome-devtools", &config, "user", None);
    assert!(result.is_some());
    let info = result.unwrap();
    assert_eq!(info.name, "chrome-devtools");
    assert_eq!(info.command.as_deref(), Some("npx"));
    assert_eq!(info.args.as_ref().unwrap().len(), 2);
    assert_eq!(info.args.as_ref().unwrap()[0], "-y");
    assert_eq!(info.env.as_ref().unwrap().get("CHROME_PATH").unwrap(), "/usr/bin/chrome");
    assert_eq!(info.server_type.as_deref(), Some("stdio"));
    assert_eq!(info.source_type, "user");
    assert!(info.url.is_none());
}

// 解析 HTTP server：带 url/headers
#[test]
fn ParseMcpEntry_HttpServer_001() {
    let config = json!({
        "type": "http",
        "url": "https://api.example.com/mcp",
        "headers": { "Authorization": "Bearer token123" }
    });
    let result = parse_mcp_server_entry("zread", &config, "user", None);
    assert!(result.is_some());
    let info = result.unwrap();
    assert_eq!(info.name, "zread");
    assert_eq!(info.url.as_deref(), Some("https://api.example.com/mcp"));
    assert_eq!(info.server_type.as_deref(), Some("http"));
    assert_eq!(info.headers.as_ref().unwrap().get("Authorization").unwrap(), "Bearer token123");
    assert!(info.command.is_none());
}

// 解析 SSE server：带 type:"sse"
#[test]
fn ParseMcpEntry_SseServer_001() {
    let config = json!({
        "type": "sse",
        "url": "https://mcp.example.com/sse"
    });
    let result = parse_mcp_server_entry("slack", &config, "project", None);
    assert!(result.is_some());
    let info = result.unwrap();
    assert_eq!(info.server_type.as_deref(), Some("sse"));
    assert_eq!(info.source_type, "project");
}

// 非对象配置返回 None
#[test]
fn ParseMcpEntry_NotObject_001() {
    let config = json!("just a string");
    let result = parse_mcp_server_entry("test", &config, "user", None);
    assert!(result.is_none());
}

// ==================== infer_server_type ====================

// 有 command 字段无 type → stdio
#[test]
fn InferType_Stdio_001() {
    let config = json!({ "command": "npx", "args": ["-y", "some-package"] });
    assert_eq!(infer_server_type(&config), "stdio");
}

// 有 url + type:"sse" → sse
#[test]
fn InferType_Sse_001() {
    let config = json!({ "type": "sse", "url": "https://example.com/sse" });
    assert_eq!(infer_server_type(&config), "sse");
}

// 有 url + type:"http" → http
#[test]
fn InferType_Http_001() {
    let config = json!({ "type": "http", "url": "https://example.com/mcp" });
    assert_eq!(infer_server_type(&config), "http");
}

// 有 url 无 type → http（默认）
#[test]
fn InferType_UrlNoType_001() {
    let config = json!({ "url": "https://example.com/mcp" });
    assert_eq!(infer_server_type(&config), "http");
}

// 无 command/url → stdio（兜底）
#[test]
fn InferType_Default_001() {
    let config = json!({});
    assert_eq!(infer_server_type(&config), "stdio");
}

// 非 JSON 对象 → stdio（兜底）
#[test]
fn InferType_NonObject_001() {
    let config = json!("string");
    assert_eq!(infer_server_type(&config), "stdio");
}

// ==================== expand_env_vars ====================

// extra_env 中的变量被展开
#[test]
fn ExpandEnvVars_ExtraEnv_001() {
    let mut extra = HashMap::new();
    extra.insert("CLAUDE_PLUGIN_ROOT".to_string(), "C:/plugins/paper".to_string());
    let result = expand_env_vars("${CLAUDE_PLUGIN_ROOT}/sub", Some(&extra));
    assert_eq!(result, "C:/plugins/paper/sub");
}

// ${VAR:-default} 使用默认值
#[test]
fn ExpandEnvVars_Default_001() {
    let result = expand_env_vars("${NONEXISTENT_VAR:-fallback}", None);
    assert_eq!(result, "fallback");
}

// 不含变量的字符串不变
#[test]
fn ExpandEnvVars_NoVars_001() {
    let result = expand_env_vars("plain string", None);
    assert_eq!(result, "plain string");
}

// 多个变量同时展开
#[test]
fn ExpandEnvVars_Multiple_001() {
    std::env::set_var("CC_BOX_TEST_A", "hello");
    let mut extra = HashMap::new();
    extra.insert("CC_BOX_TEST_B".to_string(), "world".to_string());
    let result = expand_env_vars("${CC_BOX_TEST_A}-${CC_BOX_TEST_B}", Some(&extra));
    assert_eq!(result, "hello-world");
    std::env::remove_var("CC_BOX_TEST_A");
}

// plugin scope 中 CLAUDE_PLUGIN_ROOT 被展开到 args
#[test]
fn ParseMcpEntry_PluginEnvExpand_001() {
    let config = json!({
        "command": "uv",
        "args": ["run", "--directory", "${CLAUDE_PLUGIN_ROOT}/paper-search", "mcp_server.py"]
    });
    let mut extra = HashMap::new();
    extra.insert("CLAUDE_PLUGIN_ROOT".to_string(), "C:/plugins/paper-tool".to_string());
    let result = parse_mcp_server_entry("plugin:paper-tool:paper", &config, "plugin", Some(&extra));
    assert!(result.is_some());
    let info = result.unwrap();
    let args = info.args.unwrap();
    assert_eq!(args[2], "C:/plugins/paper-tool/paper-search");
}

// ==================== parse_agents_list_output ====================

// 解析 Built-in agents 段，source_type 为 builtin
#[test]
fn ParseAgents_Builtin_001() {
    let input = "Built-in agents:\n  claude-code-guide · haiku\n  Explore · inherit";
    let mut agents: Vec<AgentInfo> = Vec::new();
    parse_agents_list_output(input, &mut agents);
    assert!(!agents.is_empty());
    let builtin_agents: Vec<&AgentInfo> = agents
        .iter()
        .filter(|a| a.source_type == "builtin")
        .collect();
    assert!(builtin_agents.len() >= 2);
    assert_eq!(builtin_agents[0].name, "claude-code-guide");
    assert_eq!(builtin_agents[0].model.as_deref(), Some("haiku"));
}

// 解析 Plugin agents 段，source_type 为 plugin
#[test]
fn ParseAgents_Plugin_001() {
    let input = "Plugin agents:\n  paper-tool:paper-search · inherit";
    let mut agents: Vec<AgentInfo> = Vec::new();
    parse_agents_list_output(input, &mut agents);
    assert_eq!(agents.len(), 1);
    assert_eq!(agents[0].source_type, "plugin");
    assert_eq!(agents[0].name, "paper-tool:paper-search");
}

// 空输入返回空 Vec 不崩溃
#[test]
fn ParseAgents_EmptyInput_001() {
    let mut agents: Vec<AgentInfo> = Vec::new();
    parse_agents_list_output("", &mut agents);
    assert!(agents.is_empty());
}

// ==================== extract_md_description ====================

// YAML frontmatter 中有 description 字段时提取其值
#[test]
fn ExtractMd_Frontmatter_001() {
    let content = "---\ndescription: This is a skill\n---\n# Skill Title\nBody text";
    let dir = tempfile::tempdir().unwrap();
    let file_path = dir.path().join("test.md");
    std::fs::write(&file_path, content).unwrap();
    let result = extract_md_description(&file_path);
    assert!(result.is_some());
    assert_eq!(result.unwrap(), "This is a skill");
}

// 无 frontmatter 时取第一个非空非标题行作为描述
#[test]
fn ExtractMd_BodyFallback_001() {
    let content = "# Title\n\nFirst body line is the description\nMore text";
    let dir = tempfile::tempdir().unwrap();
    let file_path = dir.path().join("test.md");
    std::fs::write(&file_path, content).unwrap();
    let result = extract_md_description(&file_path);
    assert!(result.is_some());
    assert_eq!(result.unwrap(), "First body line is the description");
}

// frontmatter 长描述完整返回，不截断不加省略号
#[test]
fn ExtractMd_FrontmatterLongDesc_001() {
    let long_desc: String = "x".repeat(250);
    let content = format!("---\ndescription: {}\n---\nBody", long_desc);
    let dir = tempfile::tempdir().unwrap();
    let file_path = dir.path().join("test.md");
    std::fs::write(&file_path, content).unwrap();
    let result = extract_md_description(&file_path).unwrap();
    assert_eq!(result, long_desc);
    assert!(!result.ends_with("..."));
}

// 正文长描述完整返回，不截断不加省略号
#[test]
fn ExtractMd_BodyLongDesc_001() {
    let long_body: String = "a".repeat(150);
    let content = format!("# Title\n\n{}", long_body);
    let dir = tempfile::tempdir().unwrap();
    let file_path = dir.path().join("test.md");
    std::fs::write(&file_path, content).unwrap();
    let result = extract_md_description(&file_path).unwrap();
    assert_eq!(result, long_body);
    assert!(!result.ends_with("..."));
}

// 空内容返回 "No description"
#[test]
fn ExtractMd_EmptyContent_001() {
    let dir = tempfile::tempdir().unwrap();
    let file_path = dir.path().join("test.md");
    std::fs::write(&file_path, "").unwrap();
    let result = extract_md_description(&file_path);
    assert!(result.is_some());
    assert_eq!(result.unwrap(), "No description");
}

// ==================== parse_skill_description ====================

// frontmatter 中 description 字段完整返回
#[test]
fn ParseSkill_Frontmatter_001() {
    let content = "---\ndescription: Build skill\n---\n# Title\nBody";
    let dir = tempfile::tempdir().unwrap();
    let file_path = dir.path().join("SKILL.md");
    std::fs::write(&file_path, content).unwrap();
    let result = parse_skill_description(&file_path).unwrap();
    assert_eq!(result, "Build skill");
}

// 无 frontmatter 时取正文第一行非空非标题行
#[test]
fn ParseSkill_BodyFallback_001() {
    let content = "# Title\n\nFirst body line\nMore text";
    let dir = tempfile::tempdir().unwrap();
    let file_path = dir.path().join("SKILL.md");
    std::fs::write(&file_path, content).unwrap();
    let result = parse_skill_description(&file_path).unwrap();
    assert_eq!(result, "First body line");
}

// frontmatter 长描述完整返回不截断
#[test]
fn ParseSkill_FrontmatterLongDesc_001() {
    let long_desc: String = "y".repeat(300);
    let content = format!("---\ndescription: {}\n---\nBody", long_desc);
    let dir = tempfile::tempdir().unwrap();
    let file_path = dir.path().join("SKILL.md");
    std::fs::write(&file_path, content).unwrap();
    let result = parse_skill_description(&file_path).unwrap();
    assert_eq!(result, long_desc);
    assert!(!result.ends_with("..."));
}

// 无 frontmatter 时正文长描述完整返回不截断
#[test]
fn ParseSkill_BodyLongDesc_001() {
    let long_body: String = "b".repeat(200);
    let content = format!("# Title\n\n{}", long_body);
    let dir = tempfile::tempdir().unwrap();
    let file_path = dir.path().join("SKILL.md");
    std::fs::write(&file_path, content).unwrap();
    let result = parse_skill_description(&file_path).unwrap();
    assert_eq!(result, long_body);
    assert!(!result.ends_with("..."));
}

// 折叠块标量 `>-`：多行合并为单行（用户实测 SKILL.md 曾把描述解析成 ">-"）
#[test]
fn ParseSkill_BlockScalarFold_001() {
    let content = "---\nname: nature\ndescription: >-\n  Search academic papers across\n  multiple sources.\nmodel: sonnet\n---\nBody";
    let dir = tempfile::tempdir().unwrap();
    let file_path = dir.path().join("SKILL.md");
    std::fs::write(&file_path, content).unwrap();
    let result = parse_skill_description(&file_path).unwrap();
    assert_eq!(result, "Search academic papers across multiple sources.");
}

// 字面量块标量 `|`：保留行间换行
#[test]
fn ParseSkill_BlockScalarLiteral_001() {
    let content = "---\ndescription: |\n  第一行\n  第二行\n---\nBody";
    let dir = tempfile::tempdir().unwrap();
    let file_path = dir.path().join("SKILL.md");
    std::fs::write(&file_path, content).unwrap();
    let result = parse_skill_description(&file_path).unwrap();
    assert_eq!(result, "第一行\n第二行");
}

// 单/双引号包裹的 descrption 去掉引号
#[test]
fn ParseSkill_QuotedDescription_001() {
    let content = "---\ndescription: \"quoted value\"\n---\nBody";
    let dir = tempfile::tempdir().unwrap();
    let file_path = dir.path().join("SKILL.md");
    std::fs::write(&file_path, content).unwrap();
    let result = parse_skill_description(&file_path).unwrap();
    assert_eq!(result, "quoted value");
}

// extract_md_description 同样支持块标量（plugin 里的 md 共用该解析）
#[test]
fn ExtractMd_BlockScalar_001() {
    let content = "---\ndescription: >-\n  folded line one\n  folded line two\n---\nBody";
    let dir = tempfile::tempdir().unwrap();
    let file_path = dir.path().join("cmd.md");
    std::fs::write(&file_path, content).unwrap();
    let result = extract_md_description(&file_path).unwrap();
    assert_eq!(result, "folded line one folded line two");
}

// ==================== parse_timestamp ====================

// 有效 ISO 8601 时间戳解析为准确的毫秒值（2024-01-15T10:30:00Z = 1705314600000）
#[test]
fn ParseTimestamp_ValidIso_001() {
    let result = parse_timestamp("2024-01-15T10:30:00Z");
    assert_eq!(result, 1705314600000);
}

// "not-a-date" 返回 0
#[test]
fn ParseTimestamp_InvalidString_001() {
    let result = parse_timestamp("not-a-date");
    assert_eq!(result, 0);
}

// 空字符串返回 0
#[test]
fn ParseTimestamp_EmptyString_001() {
    let result = parse_timestamp("");
    assert_eq!(result, 0);
}

// ==================== extract_session_name ====================

// 多条用户消息时返回第一条有效消息，而非最后一条
#[test]
fn ExtractSessionName_FirstUserMessage_001() {
    let lines = vec![
        r#"{"type":"user","message":{"content":"First prompt here"},"isMeta":false}"#,
        r#"{"type":"assistant","message":{"content":"response"}}"#,
        r#"{"type":"user","message":{"content":"Second prompt here"},"isMeta":false}"#,
        r#"{"type":"user","message":{"content":"Third prompt here"},"isMeta":false}"#,
    ];
    let dir = tempfile::tempdir().unwrap();
    let file_path = dir.path().join("session.jsonl");
    std::fs::write(&file_path, lines.join("\n")).unwrap();
    let result = extract_session_name(&file_path);
    assert_eq!(result, "First prompt here");
}

// custom-title 优先级高于用户消息
#[test]
fn ExtractSessionName_CustomTitlePriority_001() {
    let lines = vec![
        r#"{"type":"user","message":{"content":"User message"},"isMeta":false}"#,
        r#"{"type":"custom-title","customTitle":"My Custom Title"}"#,
    ];
    let dir = tempfile::tempdir().unwrap();
    let file_path = dir.path().join("session.jsonl");
    std::fs::write(&file_path, lines.join("\n")).unwrap();
    let result = extract_session_name(&file_path);
    assert_eq!(result, "My Custom Title");
}

// isMeta=true 的消息被过滤，不作为名称
#[test]
fn ExtractSessionName_SkipMeta_001() {
    let lines = vec![
        r#"{"type":"user","message":{"content":"meta prompt"},"isMeta":true}"#,
        r#"{"type":"user","message":{"content":"real prompt"},"isMeta":false}"#,
    ];
    let dir = tempfile::tempdir().unwrap();
    let file_path = dir.path().join("session.jsonl");
    std::fs::write(&file_path, lines.join("\n")).unwrap();
    let result = extract_session_name(&file_path);
    assert_eq!(result, "real prompt");
}

// 以 < 开头的系统注入消息被过滤
#[test]
fn ExtractSessionName_SkipSystemInject_001() {
    let lines = vec![
        r#"{"type":"user","message":{"content":"<system-reminder>some system text</system-reminder>"},"isMeta":false}"#,
        r#"{"type":"user","message":{"content":"actual user message"},"isMeta":false}"#,
    ];
    let dir = tempfile::tempdir().unwrap();
    let file_path = dir.path().join("session.jsonl");
    std::fs::write(&file_path, lines.join("\n")).unwrap();
    let result = extract_session_name(&file_path);
    assert_eq!(result, "actual user message");
}

// 超过 50 字符的消息被截断并加省略号
#[test]
fn ExtractSessionName_TruncateLong_001() {
    let long_msg: String = "a".repeat(60);
    let lines = vec![format!(
        r#"{{"type":"user","message":{{"content":"{}"}},"isMeta":false}}"#,
        long_msg
    )];
    let dir = tempfile::tempdir().unwrap();
    let file_path = dir.path().join("session.jsonl");
    std::fs::write(&file_path, lines.join("\n")).unwrap();
    let result = extract_session_name(&file_path);
    assert!(result.ends_with("..."));
    // 50 chars + "..." = 53
    assert_eq!(result.len(), 53);
}

// 无用户消息也无 custom-title 时返回 "Unnamed session"
#[test]
fn ExtractSessionName_NoMessages_001() {
    let dir = tempfile::tempdir().unwrap();
    let file_path = dir.path().join("session.jsonl");
    std::fs::write(&file_path, "").unwrap();
    let result = extract_session_name(&file_path);
    assert_eq!(result, "Unnamed session");
}

// ==================== find_valid_plugin_path / resolve_marketplace_plugin_path ====================
// 全部走临时目录 fixture，不依赖本机 ~/.claude

/// 构造 marketplace fixture，返回 (tempdir, .claude 目录, marketplace installLocation)：
/// root/.claude/plugins/known_marketplaces.json → {"test-mp": {"installLocation": <mp>}}
/// <mp>/.claude-plugin/marketplace.json → local-plugin(字符串 source) + obj-plugin(对象 source)
/// <mp>/plugins/local-plugin、<mp>/plugins/obj-plugin、root/cache-dir/plugin-cache 目录存在
fn build_marketplace_fixture() -> (
    tempfile::TempDir,
    std::path::PathBuf,
    std::path::PathBuf,
) {
    let root = tempfile::tempdir().unwrap();
    let claude_dir = root.path().join(".claude");
    let mp_dir = root.path().join("mp");
    std::fs::create_dir_all(claude_dir.join("plugins")).unwrap();
    std::fs::create_dir_all(mp_dir.join(".claude-plugin")).unwrap();
    std::fs::create_dir_all(mp_dir.join("plugins").join("local-plugin")).unwrap();
    std::fs::create_dir_all(mp_dir.join("plugins").join("obj-plugin")).unwrap();
    std::fs::create_dir_all(root.path().join("cache-dir").join("plugin-cache")).unwrap();

    let marketplace_json = serde_json::json!({
        "plugins": [
            { "name": "local-plugin", "source": "./plugins/local-plugin" },
            { "name": "obj-plugin", "source": { "source": "./plugins/obj-plugin" } }
        ]
    });
    std::fs::write(
        mp_dir.join(".claude-plugin").join("marketplace.json"),
        marketplace_json.to_string(),
    )
    .unwrap();

    let known = serde_json::json!({
        "test-mp": { "installLocation": mp_dir.to_string_lossy() }
    });
    std::fs::write(
        claude_dir.join("plugins").join("known_marketplaces.json"),
        known.to_string(),
    )
    .unwrap();

    (root, claude_dir, mp_dir)
}

// Step 1：cache 路径存在时原样返回，不查 marketplace
#[test]
fn FindPlugin_Step1CacheExists_001() {
    let (root, claude_dir, _mp) = build_marketplace_fixture();
    let cache = root.path().join("cache-dir").join("plugin-cache");
    let result =
        crate::store::find_valid_plugin_path_in(cache.to_str().unwrap(), "local-plugin@test-mp", &claude_dir);
    assert_eq!(result.unwrap(), cache.to_string_lossy());
}

// Step 2：cache 不存在时回退 marketplace，source 为字符串形式解析成功
// （join 不归一化 "./" 段是真实行为：返回原样拼接路径，路径仍真实存在）
#[test]
fn FindPlugin_Step2StringSource_001() {
    let (_root, claude_dir, mp) = build_marketplace_fixture();
    let result = crate::store::find_valid_plugin_path_in(
        "Z:\\nonexistent\\cache\\local-plugin",
        "local-plugin@test-mp",
        &claude_dir,
    );
    let expected = mp.join("./plugins/local-plugin");
    let got = result.unwrap();
    assert_eq!(got, expected.to_string_lossy());
    assert!(std::path::Path::new(&got).exists());
}

// Step 2：cache 不存在时回退 marketplace，source 为对象形式同样解析成功
#[test]
fn FindPlugin_Step2ObjectSource_001() {
    let (_root, claude_dir, mp) = build_marketplace_fixture();
    let result = crate::store::find_valid_plugin_path_in(
        "Z:\\nonexistent\\cache\\obj-plugin",
        "obj-plugin@test-mp",
        &claude_dir,
    );
    let expected = mp.join("./plugins/obj-plugin");
    let got = result.unwrap();
    assert_eq!(got, expected.to_string_lossy());
    assert!(std::path::Path::new(&got).exists());
}

// cache 不存在且 plugin 不在 marketplace 中时返回 None
#[test]
fn FindPlugin_NoFallback_001() {
    let (_root, claude_dir, _mp) = build_marketplace_fixture();
    let result = crate::store::find_valid_plugin_path_in(
        "Z:\\nonexistent\\cache\\ghost",
        "ghost@test-mp",
        &claude_dir,
    );
    assert!(result.is_none());
}

// plugin_id 无 @ 分隔时返回 None（known_marketplaces.json 存在，真正走到格式校验分支）
#[test]
fn ResolveMarketplace_BadFormat_001() {
    let (_root, claude_dir, _mp) = build_marketplace_fixture();
    let result = crate::store::resolve_marketplace_plugin_path_in(&claude_dir, "no-at-sign");
    assert!(result.is_none());
}

// known_marketplaces.json 缺失时返回 None
#[test]
fn ResolveMarketplace_NoKnownFile_001() {
    let empty = tempfile::tempdir().unwrap();
    let result = crate::store::resolve_marketplace_plugin_path_in(empty.path(), "local-plugin@test-mp");
    assert!(result.is_none());
}

// ==================== search_session_messages_in_dirs ====================

// 构造一行 JSONL 消息（user/assistant，content 为 string）
fn build_jsonl_line(msg_type: &str, content: &str) -> String {
    let t = if msg_type == "user" { "user" } else { "assistant" };
    format!(r#"{{"type":"{}","message":{{"content":"{}"}}}}"#, t, content)
}

// 单文件单消息按 query 匹配，返回 snippet
#[test]
fn SearchSession_BasicMatch_001() {
    let dir = tempfile::tempdir().unwrap();
    let file_path = dir.path().join("session-abc.jsonl");
    std::fs::write(
        &file_path,
        format!("{}\n", build_jsonl_line("user", "hello world")),
    )
    .unwrap();

    let dirs = vec![dir.path().to_path_buf()];
    let results = search_session_messages_in_dirs(&dirs, "/proj", "hello", 10);
    assert_eq!(results.len(), 1);
    assert_eq!(results[0].session_id, "session-abc");
    assert!(results[0].snippet.contains("hello"));
}

// 大小写不敏感匹配
#[test]
fn SearchSession_CaseInsensitive_001() {
    let dir = tempfile::tempdir().unwrap();
    let file_path = dir.path().join("s1.jsonl");
    std::fs::write(
        &file_path,
        format!("{}\n", build_jsonl_line("assistant", "Hello WORLD")),
    )
    .unwrap();

    let dirs = vec![dir.path().to_path_buf()];
    let results = search_session_messages_in_dirs(&dirs, "/proj", "HELLO", 10);
    assert_eq!(results.len(), 1);
    assert!(results[0].snippet.to_lowercase().contains("hello"));
}

// 超过 200 行的文件，老消息（前 200 行之外）也能被匹配
#[test]
fn SearchSession_LongFile_OldMessage_001() {
    let dir = tempfile::tempdir().unwrap();
    let file_path = dir.path().join("long.jsonl");

    // 前 250 行是不匹配的填充，第 1 行（最老）才是目标
    let mut content = String::new();
    content.push_str(&format!("{}\n", build_jsonl_line("user", "TARGET_KEYWORD_HERE")));
    for i in 0..250 {
        content.push_str(&format!("{}\n", build_jsonl_line("assistant", &format!("filler {}", i))));
    }
    std::fs::write(&file_path, content).unwrap();

    let dirs = vec![dir.path().to_path_buf()];
    let results = search_session_messages_in_dirs(&dirs, "/proj", "TARGET_KEYWORD", 10);
    assert_eq!(results.len(), 1, "old message outside newest 200 lines should be matched");
    assert!(results[0].snippet.contains("TARGET_KEYWORD"));
}

// 同一文件多条匹配，snippet 取最新（最末尾的匹配）
#[test]
fn SearchSession_LatestMatchFirst_001() {
    let dir = tempfile::tempdir().unwrap();
    let file_path = dir.path().join("multi.jsonl");
    let mut content = String::new();
    content.push_str(&format!("{}\n", build_jsonl_line("user", "KEYWORD old match")));
    content.push_str(&format!("{}\n", build_jsonl_line("assistant", "no match here")));
    content.push_str(&format!("{}\n", build_jsonl_line("user", "KEYWORD new match")));
    std::fs::write(&file_path, content).unwrap();

    let dirs = vec![dir.path().to_path_buf()];
    let results = search_session_messages_in_dirs(&dirs, "/proj", "KEYWORD", 10);
    assert_eq!(results.len(), 1);
    assert!(results[0].snippet.contains("new match"));
    assert!(!results[0].snippet.contains("old match"));
}

// agent- 开头的文件被跳过
#[test]
fn SearchSession_AgentFilesSkipped_001() {
    let dir = tempfile::tempdir().unwrap();
    std::fs::write(
        dir.path().join("agent-sub.jsonl"),
        format!("{}\n", build_jsonl_line("user", "secret keyword")),
    )
    .unwrap();
    std::fs::write(
        dir.path().join("normal.jsonl"),
        format!("{}\n", build_jsonl_line("user", "no match")),
    )
    .unwrap();

    let dirs = vec![dir.path().to_path_buf()];
    let results = search_session_messages_in_dirs(&dirs, "/proj", "secret", 10);
    assert_eq!(results.len(), 0, "agent-* files must be skipped");
}

// limit 截断生效
#[test]
fn SearchSession_LimitApplied_001() {
    let dir = tempfile::tempdir().unwrap();
    for i in 0..5 {
        std::fs::write(
            dir.path().join(format!("s{}.jsonl", i)),
            format!("{}\n", build_jsonl_line("user", "shared keyword")),
        )
        .unwrap();
    }

    let dirs = vec![dir.path().to_path_buf()];
    let results = search_session_messages_in_dirs(&dirs, "/proj", "shared", 3);
    assert_eq!(results.len(), 3);
}

// 非 .jsonl / .txt 文件被忽略
#[test]
fn SearchSession_NonJsonlIgnored_001() {
    let dir = tempfile::tempdir().unwrap();
    std::fs::write(
        dir.path().join("notes.md"),
        format!("{}\n", build_jsonl_line("user", "keyword in md")),
    )
    .unwrap();

    let dirs = vec![dir.path().to_path_buf()];
    let results = search_session_messages_in_dirs(&dirs, "/proj", "keyword", 10);
    assert_eq!(results.len(), 0);
}

// content 为 array（多模态）的消息目前不匹配（仅 string content 才匹配）
#[test]
fn SearchSession_ArrayContentSkipped_001() {
    let dir = tempfile::tempdir().unwrap();
    let file_path = dir.path().join("multi-modal.jsonl");
    std::fs::write(
        &file_path,
        r#"{"type":"user","message":{"content":[{"type":"text","text":"keyword in array"}]}}
"#,
    )
    .unwrap();

    let dirs = vec![dir.path().to_path_buf()];
    let results = search_session_messages_in_dirs(&dirs, "/proj", "keyword", 10);
    assert_eq!(results.len(), 0, "array content not yet supported");
}

// ==================== set_skill_enabled_in ====================

// 禁用 skill：目录从 active 移到 disabled
#[test]
fn SetSkillEnabled_Disable_MovesDir_001() {
    let active = tempfile::tempdir().unwrap();
    let disabled = tempfile::tempdir().unwrap();
    let skill_dir = active.path().join("deploy");
    std::fs::create_dir(&skill_dir).unwrap();
    std::fs::write(skill_dir.join("SKILL.md"), "---\ndescription: x\n---\n").unwrap();

    set_skill_enabled_in(active.path(), disabled.path(), "deploy", false).unwrap();

    assert!(!skill_dir.exists(), "active skill dir should be gone");
    assert!(
        disabled.path().join("deploy").join("SKILL.md").exists(),
        "disabled backup should exist"
    );
}

// 启用 skill：目录从 disabled 移回 active
#[test]
fn SetSkillEnabled_Enable_MovesBack_001() {
    let active = tempfile::tempdir().unwrap();
    let disabled = tempfile::tempdir().unwrap();
    let backup = disabled.path().join("deploy");
    std::fs::create_dir(&backup).unwrap();
    std::fs::write(backup.join("SKILL.md"), "content").unwrap();

    set_skill_enabled_in(active.path(), disabled.path(), "deploy", true).unwrap();

    assert!(active.path().join("deploy").join("SKILL.md").exists());
    assert!(!backup.exists());
}

// 禁用不存在的 skill → Err
#[test]
fn SetSkillEnabled_Disable_NotFound_001() {
    let active = tempfile::tempdir().unwrap();
    let disabled = tempfile::tempdir().unwrap();
    let r = set_skill_enabled_in(active.path(), disabled.path(), "ghost", false);
    assert!(r.is_err());
}

// 目录被占用判定：Windows ACCESS_DENIED(5)/SHARING_VIOLATION(32) 为占用，
// 其余（跨卷 17/18、不存在等）不算（决定 disable 失败时的提示文案）
#[test]
fn SetSkillEnabled_BusyErrorClass_001() {
    let busy5 = std::io::Error::from_raw_os_error(5);
    let busy32 = std::io::Error::from_raw_os_error(32);
    let crossvol = std::io::Error::from_raw_os_error(17);
    let notfound = std::io::Error::from_raw_os_error(2);
    assert!(crate::store::is_dir_busy_error(&busy5));
    assert!(crate::store::is_dir_busy_error(&busy32));
    assert!(!crate::store::is_dir_busy_error(&crossvol));
    assert!(!crate::store::is_dir_busy_error(&notfound));
}

// rename 失败的错误消息必须带底层 os 原因（disabled 父路径被文件占据 →
// 路径非法）——不带原因时远程端只见 "Failed to disable skill 'x'" 无法排查
#[test]
fn SetSkillEnabled_Disable_ErrorIncludesCause_001() {
    let active = tempfile::tempdir().unwrap();
    // disabled 基路径是一个文件：dst = <file>/deploy 非法 → rename 失败
    let disabled_file = active.path().join("not-a-dir");
    std::fs::write(&disabled_file, "x").unwrap();
    let skill_dir = active.path().join("deploy");
    std::fs::create_dir(&skill_dir).unwrap();

    let err = set_skill_enabled_in(active.path(), &disabled_file, "deploy", false)
        .unwrap_err()
        .to_string();
    assert!(err.contains("deploy"), "消息应含 skill 名: {}", err);
    assert!(err.contains("os error"), "消息应含底层原因: {}", err);
}

// 重复禁用（disabled 已存在）→ Err
#[test]
fn SetSkillEnabled_Disable_AlreadyDisabled_001() {
    let active = tempfile::tempdir().unwrap();
    let disabled = tempfile::tempdir().unwrap();
    std::fs::create_dir(active.path().join("deploy")).unwrap();
    std::fs::create_dir(disabled.path().join("deploy")).unwrap();

    let r = set_skill_enabled_in(active.path(), disabled.path(), "deploy", false);
    assert!(r.is_err());
}

// 启用时 active 已存在同名 → conflict Err
#[test]
fn SetSkillEnabled_Enable_Conflict_001() {
    let active = tempfile::tempdir().unwrap();
    let disabled = tempfile::tempdir().unwrap();
    std::fs::create_dir(active.path().join("deploy")).unwrap();
    std::fs::create_dir(disabled.path().join("deploy")).unwrap();

    let r = set_skill_enabled_in(active.path(), disabled.path(), "deploy", true);
    assert!(r.is_err());
}

// 启用未禁用的 skill（backup 不存在）→ Err
#[test]
fn SetSkillEnabled_Enable_NotDisabled_001() {
    let active = tempfile::tempdir().unwrap();
    let disabled = tempfile::tempdir().unwrap();

    let r = set_skill_enabled_in(active.path(), disabled.path(), "deploy", true);
    assert!(r.is_err());
}

// 路径穿越 → Err
#[test]
fn SetSkillEnabled_PathTraversal_001() {
    let active = tempfile::tempdir().unwrap();
    let disabled = tempfile::tempdir().unwrap();

    let r1 = set_skill_enabled_in(active.path(), disabled.path(), "../escape", false);
    let r2 = set_skill_enabled_in(active.path(), disabled.path(), "a/b", false);
    let r3 = set_skill_enabled_in(active.path(), disabled.path(), "a\\b", false);

    assert!(r1.is_err());
    assert!(r2.is_err());
    assert!(r3.is_err());
}

// ==================== set_agent_enabled_in ====================

// 禁用 agent：文件从 active 移到 disabled
#[test]
fn SetAgentEnabled_Disable_MovesFile_001() {
    let active = tempfile::tempdir().unwrap();
    let disabled = tempfile::tempdir().unwrap();
    std::fs::write(active.path().join("reviewer.md"), "content").unwrap();

    set_agent_enabled_in(active.path(), disabled.path(), "reviewer", false).unwrap();

    assert!(!active.path().join("reviewer.md").exists());
    assert!(disabled.path().join("reviewer.md").exists());
}

// 启用 agent：文件从 disabled 移回 active
#[test]
fn SetAgentEnabled_Enable_MovesBack_001() {
    let active = tempfile::tempdir().unwrap();
    let disabled = tempfile::tempdir().unwrap();
    std::fs::write(disabled.path().join("reviewer.md"), "content").unwrap();

    set_agent_enabled_in(active.path(), disabled.path(), "reviewer", true).unwrap();

    assert!(active.path().join("reviewer.md").exists());
    assert!(!disabled.path().join("reviewer.md").exists());
}

// 禁用不存在的 agent → Err
#[test]
fn SetAgentEnabled_Disable_NotFound_001() {
    let active = tempfile::tempdir().unwrap();
    let disabled = tempfile::tempdir().unwrap();
    let r = set_agent_enabled_in(active.path(), disabled.path(), "ghost", false);
    assert!(r.is_err());
}

// 重复禁用 → Err
#[test]
fn SetAgentEnabled_Disable_AlreadyDisabled_001() {
    let active = tempfile::tempdir().unwrap();
    let disabled = tempfile::tempdir().unwrap();
    std::fs::write(active.path().join("reviewer.md"), "x").unwrap();
    std::fs::write(disabled.path().join("reviewer.md"), "x").unwrap();

    let r = set_agent_enabled_in(active.path(), disabled.path(), "reviewer", false);
    assert!(r.is_err());
}

// 启用冲突 → Err
#[test]
fn SetAgentEnabled_Enable_Conflict_001() {
    let active = tempfile::tempdir().unwrap();
    let disabled = tempfile::tempdir().unwrap();
    std::fs::write(active.path().join("reviewer.md"), "x").unwrap();
    std::fs::write(disabled.path().join("reviewer.md"), "x").unwrap();

    let r = set_agent_enabled_in(active.path(), disabled.path(), "reviewer", true);
    assert!(r.is_err());
}

// 路径穿越 → Err
#[test]
fn SetAgentEnabled_PathTraversal_001() {
    let active = tempfile::tempdir().unwrap();
    let disabled = tempfile::tempdir().unwrap();

    let r = set_agent_enabled_in(active.path(), disabled.path(), "../escape", false);
    assert!(r.is_err());
}

// 列表用 frontmatter name，文件名可与它不同：禁用/启用都必须找到真实文件
// 并按原文件名往返（否则报 not found，开关"无效"）
#[test]
fn SetAgentEnabled_FrontmatterNameDiffers_RoundTrip_001() {
    let active = tempfile::tempdir().unwrap();
    let disabled = tempfile::tempdir().unwrap();
    std::fs::write(
        active.path().join("reviewer.md"),
        "---\nname: code-reviewer\ndescription: x\n---\n",
    )
    .unwrap();

    set_agent_enabled_in(active.path(), disabled.path(), "code-reviewer", false).unwrap();
    assert!(!active.path().join("reviewer.md").exists());
    assert!(
        disabled.path().join("reviewer.md").exists(),
        "禁用后应保留原文件名"
    );

    set_agent_enabled_in(active.path(), disabled.path(), "code-reviewer", true).unwrap();
    assert!(active.path().join("reviewer.md").exists());
    assert!(!disabled.path().join("reviewer.md").exists());
}

// frontmatter 名与别的文件名撞车：必须按 frontmatter 身份移动，不能直查文件名
#[test]
fn SetAgentEnabled_FrontmatterCollision_MovesCorrectFile_001() {
    let active = tempfile::tempdir().unwrap();
    let disabled = tempfile::tempdir().unwrap();
    // foo.md 的 frontmatter name 是 bar；bar.md 的 frontmatter name 是 baz
    std::fs::write(
        active.path().join("foo.md"),
        "---\nname: bar\ndescription: x\n---\n",
    )
    .unwrap();
    std::fs::write(
        active.path().join("bar.md"),
        "---\nname: baz\ndescription: y\n---\n",
    )
    .unwrap();

    set_agent_enabled_in(active.path(), disabled.path(), "bar", false).unwrap();

    assert!(
        disabled.path().join("foo.md").exists(),
        "应移动 frontmatter 名为 bar 的 foo.md"
    );
    assert!(
        active.path().join("bar.md").exists(),
        "frontmatter 名为 baz 的 bar.md 不能被移动"
    );
}

// 多个文件声明同一 frontmatter 名 → 拒绝（不能静默挑一个移动）
#[test]
fn SetAgentEnabled_AmbiguousName_Rejected_001() {
    let active = tempfile::tempdir().unwrap();
    let disabled = tempfile::tempdir().unwrap();
    for file in ["a.md", "b.md"] {
        std::fs::write(
            active.path().join(file),
            "---\nname: dup\ndescription: x\n---\n",
        )
        .unwrap();
    }

    let r = set_agent_enabled_in(active.path(), disabled.path(), "dup", false);
    assert!(r.is_err());
}

// 词法归一化：相对路径的前导 ".." 不能被后续 ".." 误折叠
#[test]
fn NormalizeLexical_RelativeParent_001() {
    use std::path::Path;

    for (input, expected) in [
        ("a/../../..", "../.."),
        ("a/b/../c", "a/c"),
        ("../a/..", ".."),
        ("./a/./b", "a/b"),
        ("/a/b/../../c", "/c"),
    ] {
        assert_eq!(
            crate::store::normalize_lexical(Path::new(input)),
            Path::new(expected),
            "input = {}",
            input
        );
    }
}

// 稳定按名称排序：启用状态不参与排序键，开关前后条目位置不变
#[test]
fn SortByName_StableOrder_001() {
    let mut items = vec![("b", false), ("a", true), ("c", false)];

    crate::store::sort_by_name(&mut items, |x| x.0);

    let names: Vec<&str> = items.iter().map(|x| x.0).collect();
    assert_eq!(names, vec!["a", "b", "c"]);
    // enabled 不参与排序：a 仍为 true、b/c 仍为 false
    assert_eq!(items[0].1, true);
    assert_eq!(items[1].1, false);

    // 稳定排序：同名项保持原有相对顺序（active 在前、disabled 在后）
    let mut same_name = vec![("a", true), ("a", false)];
    crate::store::sort_by_name(&mut same_name, |x| x.0);
    assert_eq!(same_name, vec![("a", true), ("a", false)]);
}

// ==================== 符号链接（相对链接）往返 ====================
//
// 回归：`~/.claude/skills/x -> ../../.agents/skills/x` 这类相对链接被 rename
// 搬进 disabled 目录后目标失效，备份既扫不到也恢复不了。

/// 构造 active/disabled 与链接目标：active 下的 `<name>` 是指向 `../../agents/<name>` 的相对链接
#[cfg(unix)]
fn setup_relative_symlink_skill() -> (
    tempfile::TempDir,
    std::path::PathBuf,
    std::path::PathBuf,
) {
    let root = tempfile::tempdir().unwrap();
    let target = root.path().join("agents").join("deploy");
    std::fs::create_dir_all(&target).unwrap();
    std::fs::write(target.join("SKILL.md"), "---\ndescription: x\n---\n").unwrap();

    let active = root.path().join("claude").join("skills");
    let disabled = root.path().join("ccbox").join("disabled").join("skills");
    std::fs::create_dir_all(&active).unwrap();
    std::fs::create_dir_all(&disabled).unwrap();
    std::os::unix::fs::symlink("../../agents/deploy", active.join("deploy")).unwrap();
    (root, active, disabled)
}

#[cfg(unix)]
#[test]
fn SetSkillEnabled_SymlinkRelative_RoundTrip_001() {
    let (_root, active, disabled) = setup_relative_symlink_skill();

    set_skill_enabled_in(&active, &disabled, "deploy", false).unwrap();
    let moved = disabled.join("deploy");
    assert!(
        std::fs::symlink_metadata(&moved)
            .unwrap()
            .file_type()
            .is_symlink(),
        "禁用后应仍是符号链接（不能把目标目录整体搬走）"
    );
    assert!(
        moved.join("SKILL.md").exists(),
        "禁用后链接必须仍可解析（相对目标需按新位置重算）"
    );

    set_skill_enabled_in(&active, &disabled, "deploy", true).unwrap();
    assert!(active.join("deploy").join("SKILL.md").exists());
    assert!(!crate::store::path_entry_exists(&moved));
}

// 历史 bug 产物：disabled 里的断链相对链接仍能从 GUI 恢复。
// 链接文本原本相对 active 目录生成 → 恢复时按目标目录解析即修复。
#[cfg(unix)]
#[test]
fn SetSkillEnabled_BrokenBackup_Recoverable_001() {
    let root = tempfile::tempdir().unwrap();
    let target = root.path().join("agents").join("deploy");
    std::fs::create_dir_all(&target).unwrap();
    std::fs::write(target.join("SKILL.md"), "x").unwrap();

    let active = root.path().join("claude").join("skills");
    let disabled = root.path().join("ccbox").join("disabled").join("skills");
    std::fs::create_dir_all(&active).unwrap();
    std::fs::create_dir_all(&disabled).unwrap();
    // 相对文本本应相对 active 解析，被搬进 disabled 后断链
    std::os::unix::fs::symlink("../../agents/deploy", disabled.join("deploy")).unwrap();
    assert!(crate::store::is_broken_symlink(&disabled.join("deploy")));

    set_skill_enabled_in(&active, &disabled, "deploy", true).unwrap();

    assert!(
        active.join("deploy").join("SKILL.md").exists(),
        "断链备份恢复后必须可解析"
    );
}

// 再次禁用时，目标位的断链残留直接覆盖（否则报 already disabled，开关无效）
#[cfg(unix)]
#[test]
fn SetSkillEnabled_Disable_ReplacesBrokenBackup_001() {
    let root = tempfile::tempdir().unwrap();
    let active = root.path().join("claude").join("skills");
    let disabled = root.path().join("ccbox").join("disabled").join("skills");
    std::fs::create_dir_all(active.join("deploy")).unwrap();
    std::fs::write(active.join("deploy").join("SKILL.md"), "x").unwrap();
    std::fs::create_dir_all(&disabled).unwrap();
    std::os::unix::fs::symlink("../../nowhere", disabled.join("deploy")).unwrap();

    set_skill_enabled_in(&active, &disabled, "deploy", false).unwrap();

    assert!(disabled.join("deploy").join("SKILL.md").exists());
}

// 祖先目录本身是符号链接（dotfiles/stow 常见）：相对链接必须按真实目录解析，
// 词法拼接会算出不存在的路径
#[cfg(unix)]
#[test]
fn SetSkillEnabled_SymlinkAncestor_RoundTrip_001() {
    let root = tempfile::tempdir().unwrap();
    // 真实布局：<root>/real/claude/skills + <root>/real/.agents/skills/deploy
    let real_claude = root.path().join("real").join("claude");
    let target = root
        .path()
        .join("real")
        .join(".agents")
        .join("skills")
        .join("deploy");
    std::fs::create_dir_all(&target).unwrap();
    std::fs::write(target.join("SKILL.md"), "---\ndescription: x\n---\n").unwrap();
    std::fs::create_dir_all(real_claude.join("skills")).unwrap();

    // <root>/home/claude -> <root>/real/claude（配置目录本身是链接）
    let home = root.path().join("home");
    std::fs::create_dir_all(&home).unwrap();
    std::os::unix::fs::symlink(&real_claude, home.join("claude")).unwrap();
    let active = home.join("claude").join("skills");
    // 链接文本相对真实目录：real/claude/skills -> ../../.agents/skills/deploy
    std::os::unix::fs::symlink("../../.agents/skills/deploy", active.join("deploy")).unwrap();
    assert!(
        active.join("deploy").join("SKILL.md").exists(),
        "前置：链接应可解析"
    );

    let disabled = root.path().join("ccbox").join("disabled").join("skills");
    std::fs::create_dir_all(&disabled).unwrap();

    set_skill_enabled_in(&active, &disabled, "deploy", false).unwrap();
    assert!(
        disabled.join("deploy").join("SKILL.md").exists(),
        "祖先为符号链接时，禁用后链接仍须可解析"
    );

    set_skill_enabled_in(&active, &disabled, "deploy", true).unwrap();
    assert!(
        active.join("deploy").join("SKILL.md").exists(),
        "恢复后链接仍须可解析"
    );
}

// disabled 侧祖先目录也是符号链接时同样要按真实目录重算（与 src 侧对称）
#[cfg(unix)]
#[test]
fn SetSkillEnabled_SymlinkDstAncestor_RoundTrip_001() {
    let root = tempfile::tempdir().unwrap();
    let real = root.path().join("real");
    let target = real.join(".agents").join("skills").join("deploy");
    std::fs::create_dir_all(&target).unwrap();
    std::fs::write(target.join("SKILL.md"), "x").unwrap();
    let active = real.join("claude").join("skills");
    std::fs::create_dir_all(&active).unwrap();
    std::os::unix::fs::symlink("../../.agents/skills/deploy", active.join("deploy")).unwrap();
    // <root>/ccbox -> <root>/real/ccbox（disabled 侧祖先为链接）
    std::fs::create_dir_all(real.join("ccbox").join("disabled").join("skills")).unwrap();
    std::os::unix::fs::symlink(real.join("ccbox"), root.path().join("ccbox")).unwrap();
    let disabled = root.path().join("ccbox").join("disabled").join("skills");

    set_skill_enabled_in(&active, &disabled, "deploy", false).unwrap();
    assert!(
        disabled.join("deploy").join("SKILL.md").exists(),
        "disabled 侧祖先为符号链接时链接仍须可解析"
    );

    set_skill_enabled_in(&active, &disabled, "deploy", true).unwrap();
    assert!(active.join("deploy").join("SKILL.md").exists());
}

// 启用时允许覆盖 active 侧的断链残留（该残留对列表不可见，否则永久阻塞恢复）
#[cfg(unix)]
#[test]
fn SetSkillEnabled_Enable_ReplacesBrokenActiveLeftover_001() {
    let active = tempfile::tempdir().unwrap();
    let disabled = tempfile::tempdir().unwrap();
    std::os::unix::fs::symlink("../../nowhere", active.path().join("deploy")).unwrap();
    std::fs::create_dir_all(disabled.path().join("deploy")).unwrap();
    std::fs::write(disabled.path().join("deploy").join("SKILL.md"), "x").unwrap();

    set_skill_enabled_in(active.path(), disabled.path(), "deploy", true).unwrap();

    assert!(active.path().join("deploy").join("SKILL.md").exists());
}

// disabled 扫描必须列出断链（否则从列表消失，无法恢复）；描述为 None
#[cfg(unix)]
#[test]
fn ScanDisabledSkills_BrokenSymlink_Listed_001() {
    let dir = tempfile::tempdir().unwrap();
    std::os::unix::fs::symlink("../../nowhere", dir.path().join("deploy")).unwrap();

    let items = crate::store::scan_disabled_skills(dir.path());

    assert_eq!(items.len(), 1, "断链备份必须列出");
    assert_eq!(items[0].0, "deploy");
    assert!(items[0].1.is_none());
}

// active 扫描跳过断链（CLI 也加载不到），但正常相对链接必须可见
#[cfg(unix)]
#[test]
fn ScanActiveSkills_SymlinkVisibility_001() {
    let root = tempfile::tempdir().unwrap();
    let target = root.path().join("agents").join("deploy");
    std::fs::create_dir_all(&target).unwrap();
    std::fs::write(target.join("SKILL.md"), "---\ndescription: ok\n---\n").unwrap();
    let active = root.path().join("claude").join("skills");
    std::fs::create_dir_all(&active).unwrap();
    std::os::unix::fs::symlink("../../agents/deploy", active.join("deploy")).unwrap();
    std::os::unix::fs::symlink("../../nowhere", active.join("broken")).unwrap();

    let items = crate::store::scan_active_skills(&active);

    assert_eq!(items.len(), 1);
    assert_eq!(items[0].0, "deploy");
    assert_eq!(items[0].1.as_deref(), Some("ok"));
}

// ==================== set_mcp_server_enabled_in ====================

// 禁用 MCP：从 ~/.claude.json::mcpServers.<name> 剪切到 backup，其他字段保留
#[test]
fn SetMcpEnabled_Disable_CutsEntry_001() {
    let tmp = tempfile::tempdir().unwrap();
    let claude_json = tmp.path().join(".claude.json");
    let disabled_dir = tmp.path().join("disabled_mcp");
    std::fs::create_dir_all(&disabled_dir).unwrap();
    std::fs::write(
        &claude_json,
        r#"{
            "otherConfig": {"keepMe": true},
            "mcpServers": {
                "zread": {"type":"http","url":"https://x"},
                "other": {"command":"foo"}
            }
        }"#,
    )
    .unwrap();

    set_mcp_server_enabled_in(&claude_json, &disabled_dir, "zread", false).unwrap();

    // backup 文件含单条 server 配置
    let backup = std::fs::read_to_string(disabled_dir.join("zread.json")).unwrap();
    assert!(backup.contains("https://x"), "backup should contain url content");
    assert!(!backup.contains("\"other\""), "backup should only contain zread");

    // 主配置保留其他字段和其他 server
    let main = std::fs::read_to_string(&claude_json).unwrap();
    assert!(main.contains("\"keepMe\""), "other config must be preserved");
    assert!(main.contains("\"other\""), "other server must be preserved");
    assert!(!main.contains("zread"), "zread should be removed from main config");
}

// 启用 MCP：backup 内容贴回 mcpServers，backup 文件删除
#[test]
fn SetMcpEnabled_Enable_PastesBack_001() {
    let tmp = tempfile::tempdir().unwrap();
    let claude_json = tmp.path().join(".claude.json");
    let disabled_dir = tmp.path().join("disabled_mcp");
    std::fs::create_dir_all(&disabled_dir).unwrap();
    std::fs::write(
        &claude_json,
        r#"{"otherConfig":{"keepMe":true},"mcpServers":{"other":{"command":"foo"}}}"#,
    )
    .unwrap();
    std::fs::write(
        disabled_dir.join("zread.json"),
        r#"{"type":"http","url":"https://x"}"#,
    )
    .unwrap();

    set_mcp_server_enabled_in(&claude_json, &disabled_dir, "zread", true).unwrap();

    let main = std::fs::read_to_string(&claude_json).unwrap();
    assert!(main.contains("zread"), "zread should be back in main config");
    assert!(main.contains("https://x"), "zread config content should be intact");
    assert!(main.contains("\"keepMe\""), "other config preserved");
    assert!(main.contains("\"other\""), "other server preserved");
    assert!(!disabled_dir.join("zread.json").exists(), "backup file should be removed");
}

// 禁用不存在的 server → Err
#[test]
fn SetMcpEnabled_Disable_NotFound_001() {
    let tmp = tempfile::tempdir().unwrap();
    let claude_json = tmp.path().join(".claude.json");
    let disabled_dir = tmp.path().join("disabled_mcp");
    std::fs::create_dir_all(&disabled_dir).unwrap();
    std::fs::write(&claude_json, r#"{"mcpServers":{"other":{"command":"x"}}}"#).unwrap();

    let r = set_mcp_server_enabled_in(&claude_json, &disabled_dir, "ghost", false);
    assert!(r.is_err());
}

// 重复禁用（backup 已存在）→ Err
#[test]
fn SetMcpEnabled_Disable_AlreadyDisabled_001() {
    let tmp = tempfile::tempdir().unwrap();
    let claude_json = tmp.path().join(".claude.json");
    let disabled_dir = tmp.path().join("disabled_mcp");
    std::fs::create_dir_all(&disabled_dir).unwrap();
    std::fs::write(
        &claude_json,
        r#"{"mcpServers":{"zread":{"url":"x"}}}"#,
    )
    .unwrap();
    std::fs::write(disabled_dir.join("zread.json"), r#"{"url":"x"}"#).unwrap();

    let r = set_mcp_server_enabled_in(&claude_json, &disabled_dir, "zread", false);
    assert!(r.is_err());
}

// 启用时主配置已有同名 → conflict Err
#[test]
fn SetMcpEnabled_Enable_Conflict_001() {
    let tmp = tempfile::tempdir().unwrap();
    let claude_json = tmp.path().join(".claude.json");
    let disabled_dir = tmp.path().join("disabled_mcp");
    std::fs::create_dir_all(&disabled_dir).unwrap();
    std::fs::write(
        &claude_json,
        r#"{"mcpServers":{"zread":{"url":"old"}}}"#,
    )
    .unwrap();
    std::fs::write(disabled_dir.join("zread.json"), r#"{"url":"new"}"#).unwrap();

    let r = set_mcp_server_enabled_in(&claude_json, &disabled_dir, "zread", true);
    assert!(r.is_err());
}

// 启用时 backup 不存在 → Err
#[test]
fn SetMcpEnabled_Enable_NotDisabled_001() {
    let tmp = tempfile::tempdir().unwrap();
    let claude_json = tmp.path().join(".claude.json");
    let disabled_dir = tmp.path().join("disabled_mcp");
    std::fs::create_dir_all(&disabled_dir).unwrap();
    std::fs::write(&claude_json, r#"{"mcpServers":{}}"#).unwrap();

    let r = set_mcp_server_enabled_in(&claude_json, &disabled_dir, "zread", true);
    assert!(r.is_err());
}

// .claude.json 不存在时禁用 → Err
#[test]
fn SetMcpEnabled_Disable_NoClaudeJson_001() {
    let tmp = tempfile::tempdir().unwrap();
    let claude_json = tmp.path().join(".claude.json");
    let disabled_dir = tmp.path().join("disabled_mcp");
    std::fs::create_dir_all(&disabled_dir).unwrap();

    let r = set_mcp_server_enabled_in(&claude_json, &disabled_dir, "zread", false);
    assert!(r.is_err());
}

// 启用时主配置文件不存在，会自动创建并加入
#[test]
fn SetMcpEnabled_Enable_CreatesClaudeJson_001() {
    let tmp = tempfile::tempdir().unwrap();
    let claude_json = tmp.path().join(".claude.json");
    let disabled_dir = tmp.path().join("disabled_mcp");
    std::fs::create_dir_all(&disabled_dir).unwrap();
    std::fs::write(disabled_dir.join("zread.json"), r#"{"url":"x"}"#).unwrap();

    set_mcp_server_enabled_in(&claude_json, &disabled_dir, "zread", true).unwrap();

    let main = std::fs::read_to_string(&claude_json).unwrap();
    assert!(main.contains("zread"));
    assert!(main.contains("mcpServers"));
}

// 路径穿越 → Err
#[test]
fn SetMcpEnabled_PathTraversal_001() {
    let tmp = tempfile::tempdir().unwrap();
    let claude_json = tmp.path().join(".claude.json");
    let disabled_dir = tmp.path().join("disabled_mcp");
    std::fs::create_dir_all(&disabled_dir).unwrap();
    std::fs::write(&claude_json, r#"{"mcpServers":{}}"#).unwrap();

    let r = set_mcp_server_enabled_in(&claude_json, &disabled_dir, "../escape", false);
    assert!(r.is_err());
}

// ==================== 隐藏项目 ====================

use crate::store::{
    get_app_config, hide_project, is_hidden_project, normalize_project_key, unhide_project,
};
use std::collections::HashSet;

// 归一化匹配：大小写/分隔符/尾斜杠差异视为同一路径，不同路径不误伤
#[test]
fn IsHiddenProject_NormalizeMatch_001() {
    let keys: HashSet<String> = ["D:\\Work\\Demo", "/home/user/proj/"]
        .iter()
        .map(|p| normalize_project_key(p))
        .collect();

    assert!(is_hidden_project(&keys, "d:/work/demo"));
    assert!(is_hidden_project(&keys, "D:\\Work\\Demo"));
    assert!(is_hidden_project(&keys, "/home/user/proj"));
    assert!(!is_hidden_project(&keys, "D:/Work/other"));
}

// 隐藏落盘 + 归一化去重（同路径不同写法只记一条）
#[test]
fn HideProject_PersistAndDedup_002() {
    let _cfg_lock = super::CONFIG_DIR_TEST_LOCK.lock();
    let _cfg_guard = super::TestConfigDirGuard::new("hide-project");

    hide_project("D:\\Work\\Demo").unwrap();
    hide_project("/home/user/proj").unwrap();
    // 同路径不同写法 → 去重
    hide_project("d:/work/demo/").unwrap();

    let hidden = get_app_config().unwrap().hidden_projects.unwrap();
    assert_eq!(hidden.len(), 2);
}

// 取消隐藏：按归一化移除；未隐藏路径调用无副作用
#[test]
fn UnhideProject_RemoveNormalized_003() {
    let _cfg_lock = super::CONFIG_DIR_TEST_LOCK.lock();
    let _cfg_guard = super::TestConfigDirGuard::new("unhide-project");

    hide_project("D:\\Work\\Demo").unwrap();
    hide_project("/home/user/proj").unwrap();
    // 不同分隔符/大小写写法也能解除
    unhide_project("d:/work/demo").unwrap();
    // 未隐藏路径 → 无错误、列表不变
    unhide_project("/never/hidden").unwrap();

    let hidden = get_app_config().unwrap().hidden_projects.unwrap();
    assert_eq!(hidden, vec!["/home/user/proj".to_string()]);
}
