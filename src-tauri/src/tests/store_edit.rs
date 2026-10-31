//! 扩展编辑域测试：MCP 三作用域读写、skill/agent 表单与源码模式、
//! hook 合并/定位/启停、项目级开关隔离。
//!
//! 全部走注入式 `*_in` 函数 + 临时目录，不触真实 ~/.claude / ~/.cc-box。

use std::path::Path;

use crate::store_edit::{
    build_agent_md, build_hook_entry, build_skill_md, delete_hook_in, delete_mcp_entry_in,
    delete_resource_in, find_hook_index, hook_entry_key, hook_key, hook_signature, project_key,
    save_hook_in, save_mcp_entry_in, save_skill_in, set_hook_enabled_in, set_mcp_server_local_in,
    validate_md_frontmatter, HookOriginal, SaveHookRequest, SaveSkillRequest,
};

fn read_json(path: &Path) -> serde_json::Value {
    serde_json::from_str(&std::fs::read_to_string(path).unwrap()).unwrap()
}

fn write_json(path: &Path, v: &serde_json::Value) {
    std::fs::write(path, serde_json::to_string_pretty(v).unwrap()).unwrap();
}

// ==================== frontmatter 构造与校验 ====================

/// 表单模式拼装：name/description（YAML 双引号转义）+ 正文；独立真值对拍
#[test]
fn BuildSkillMd_拼装与转义() {
    let md = build_skill_md("commit-helper", "多行\"描述\"\n第二行", "正文内容");
    // 独立手写期望：frontmatter 闭合后接正文
    let expected = "---\nname: commit-helper\ndescription: \"多行\\\"描述\\\"\\n第二行\"\n---\n\n正文内容\n";
    assert_eq!(md, expected);
    // 产物自身可通过源码校验
    assert!(validate_md_frontmatter(&md, "commit-helper").is_ok());
}

/// Agent 拼装：tools/model 缺省时不输出该字段
#[test]
fn BuildAgentMd_可选字段() {
    let with = build_agent_md("rev", "审查", Some("Read, Grep"), Some("sonnet"), "清单");
    assert!(with.contains("tools: Read, Grep"), "got: {}", with);
    assert!(with.contains("model: sonnet"));

    let without = build_agent_md("rev", "审查", None, None, "清单");
    assert!(!without.contains("tools:"), "got: {}", without);
    assert!(!without.contains("model:"));
    assert!(without.starts_with("---\nname: rev\ndescription: \"审查\"\n---\n"));
}

/// 源码模式校验：无 frontmatter / 未闭合 / name 不一致 均拒绝
#[test]
fn ValidateMdFrontmatter_拒绝非法() {
    assert!(validate_md_frontmatter("没有 frontmatter", "x").is_err());
    assert!(validate_md_frontmatter("---\nname: x\n没有闭合", "x").is_err());
    assert!(validate_md_frontmatter("---\nname: y\n---\n正文", "x").is_err());
    assert!(validate_md_frontmatter("---\ndescription: 无name也可\n---\n正文", "x").is_ok());
}

// ==================== MCP 三作用域 ====================

fn valid_stdio_config() -> serde_json::Value {
    serde_json::json!({ "command": "npx", "args": ["-y", "srv"] })
}

/// user 形态：根 mcpServers 合并单 key；重命名移除旧 key
#[test]
fn SaveMcpEntryIn_根形态与重命名() {
    let tmp = tempfile::tempdir().unwrap();
    let file = tmp.path().join(".claude.json");
    write_json(&file, &serde_json::json!({ "other": 1, "mcpServers": { "old": { "command": "c" } } }));

    save_mcp_entry_in(&file, None, "new", &valid_stdio_config(), Some("old")).unwrap();

    let root = read_json(&file);
    assert_eq!(root["other"], 1);
    assert!(root["mcpServers"].get("old").is_none(), "旧 key 应被移除");
    assert_eq!(root["mcpServers"]["new"]["command"], "npx");
    assert_eq!(root["mcpServers"]["new"]["args"][0], "-y");
}

/// local 形态：projects[<p>].mcpServers 嵌套；同名覆盖（编辑语义）
#[test]
fn SaveMcpEntryIn_嵌套形态与覆盖() {
    let tmp = tempfile::tempdir().unwrap();
    let file = tmp.path().join(".claude.json");
    write_json(&file, &serde_json::json!({ "projects": { "D:/proj": { "mcpServers": { "srv": { "command": "old" } } } } }));

    save_mcp_entry_in(&file, Some("D:/proj"), "srv", &valid_stdio_config(), None).unwrap();

    let root = read_json(&file);
    assert_eq!(root["projects"]["D:/proj"]["mcpServers"]["srv"]["command"], "npx");

    // 缺 project 路径的项目不存在时自动建结构
    save_mcp_entry_in(&file, Some("D:/other"), "s2", &valid_stdio_config(), None).unwrap();
    let root = read_json(&file);
    assert_eq!(root["projects"]["D:/other"]["mcpServers"]["s2"]["command"], "npx");
}

/// 非法配置拒绝：既无 command 也无 url
#[test]
fn SaveMcpEntryIn_拒绝非法配置() {
    let tmp = tempfile::tempdir().unwrap();
    let file = tmp.path().join(".claude.json");
    let err = save_mcp_entry_in(&file, None, "bad", &serde_json::json!({ "env": {} }), None)
        .unwrap_err()
        .to_string();
    assert!(err.contains("command") || err.contains("url"), "got: {}", err);

    // 路径穿越名拒绝
    assert!(save_mcp_entry_in(&file, None, "../evil", &valid_stdio_config(), None).is_err());
}

/// 删除：彻底移除（不再写备份），历史 disabled 备份一并清理；目标不存在报错
#[test]
fn DeleteMcpEntryIn_彻底删除() {
    let tmp = tempfile::tempdir().unwrap();
    let file = tmp.path().join(".claude.json");
    let config = serde_json::json!({ "command": "c", "env": { "K": "V" } });
    write_json(&file, &serde_json::json!({ "mcpServers": { "srv": config } }));
    let disabled = tmp.path().join("disabled");
    // 预置历史备份（模拟此前禁用过）：删除应一并清理
    std::fs::create_dir_all(&disabled).unwrap();
    std::fs::write(disabled.join("srv.json"), "{}").unwrap();

    delete_mcp_entry_in(&file, None, &disabled, "srv").unwrap();

    let root = read_json(&file);
    assert!(root["mcpServers"].get("srv").is_none());
    assert!(!disabled.join("srv.json").exists(), "disabled 备份应一并清理");

    assert!(delete_mcp_entry_in(&file, None, &disabled, "srv").is_err(), "不存在应报错");
}

/// 删除已禁用的 MCP：条目只存在于 disabled 备份——删备份即完成删除
#[test]
fn DeleteMcpEntryIn_禁用条目删除() {
    let tmp = tempfile::tempdir().unwrap();
    let file = tmp.path().join(".claude.json");
    let config = serde_json::json!({ "command": "c" });
    // 配置里没有该条目（已被禁用），仅剩 disabled 备份
    write_json(&file, &serde_json::json!({ "mcpServers": {} }));
    let disabled = tmp.path().join("disabled");
    std::fs::create_dir_all(&disabled).unwrap();
    std::fs::write(disabled.join("srv.json"), serde_json::to_string_pretty(&config).unwrap()).unwrap();

    delete_mcp_entry_in(&file, None, &disabled, "srv").unwrap();

    assert!(!disabled.join("srv.json").exists());
    let root = read_json(&file);
    assert!(root["mcpServers"].get("srv").is_none());
}

/// local 作用域启停：disable 剪切到备份；enable 找回；路径细节保持
#[test]
fn SetMcpServerLocalIn_剪切粘贴() {
    let tmp = tempfile::tempdir().unwrap();
    let file = tmp.path().join(".claude.json");
    write_json(
        &file,
        &serde_json::json!({ "projects": { "D:/proj": { "mcpServers": { "srv": { "url": "http://x/mcp", "headers": { "A": "b" } } } } } }),
    );
    let disabled = tmp.path().join("disabled");
    std::fs::create_dir_all(&disabled).unwrap();

    set_mcp_server_local_in(&file, &disabled, "D:/proj", "srv", false).unwrap();
    let root = read_json(&file);
    assert!(root["projects"]["D:/proj"]["mcpServers"].get("srv").is_none());
    let backup = read_json(&disabled.join("srv.json"));
    assert_eq!(backup["url"], "http://x/mcp");

    // 重复禁用报错
    assert!(set_mcp_server_local_in(&file, &disabled, "D:/proj", "srv", false).is_err());

    set_mcp_server_local_in(&file, &disabled, "D:/proj", "srv", true).unwrap();
    let root = read_json(&file);
    assert_eq!(root["projects"]["D:/proj"]["mcpServers"]["srv"]["headers"]["A"], "b");
    assert!(!disabled.join("srv.json").exists(), "启用后备份应删除");
}

// ==================== Skill ====================

fn skill_req(name: &str, as_source: bool, content: &str, desc: Option<&str>, orig: Option<&str>) -> SaveSkillRequest {
    serde_json::from_value(serde_json::json!({
        "scope": "user", "name": name, "asSource": as_source,
        "description": desc, "content": content, "originalName": orig,
        "projectPath": null
    }))
    .unwrap()
}

/// 表单模式：生成目录 + SKILL.md；缺 description 报错
#[test]
fn SaveSkillIn_表单模式() {
    let tmp = tempfile::tempdir().unwrap();
    let base = tmp.path().join("skills");
    let disabled = tmp.path().join("disabled");

    let req: SaveSkillRequest = serde_json::from_value(serde_json::json!({
        "scope": "user", "name": "demo", "asSource": false,
        "description": "示例", "content": "# 步骤\n1. 做"
    }))
    .unwrap();
    let path = save_skill_in(&base, &disabled, &req).unwrap();
    assert_eq!(path, base.join("demo").join("SKILL.md"));
    let md = std::fs::read_to_string(&path).unwrap();
    assert!(md.starts_with("---\nname: demo\ndescription: \"示例\"\n---\n"), "got: {}", md);
    assert!(md.contains("# 步骤"));

    // 缺 description → Err
    let req2: SaveSkillRequest = serde_json::from_value(serde_json::json!({
        "scope": "user", "name": "demo2", "asSource": false, "content": "x"
    }))
    .unwrap();
    assert!(save_skill_in(&base, &disabled, &req2).is_err());
}

/// 源码模式 + 重命名：frontmatter 校验；旧目录整体 move
#[test]
fn SaveSkillIn_源码模式与重命名() {
    let tmp = tempfile::tempdir().unwrap();
    let base = tmp.path().join("skills");
    let disabled = tmp.path().join("disabled");
    std::fs::create_dir_all(base.join("old")).unwrap();
    std::fs::write(base.join("old").join("extra.txt"), "附属文件").unwrap();

    // name 不一致拒绝
    let bad = skill_req("new", true, "---\nname: mismatch\n---\n正文", None, None);
    assert!(save_skill_in(&base, &disabled, &bad).is_err());

    // 重命名 + 源码写入
    let good = skill_req("new", true, "---\nname: new\n---\n新正文", None, Some("old"));
    save_skill_in(&base, &disabled, &good).unwrap();
    assert!(!base.join("old").exists(), "旧目录应已 move");
    assert!(base.join("new").join("extra.txt").exists(), "附属文件随目录保留");
    let md = std::fs::read_to_string(base.join("new").join("SKILL.md")).unwrap();
    assert!(md.contains("新正文"));
}

/// 彻底删除：目录（含附属文件）删除 + disabled 备份一并清理
#[test]
fn DeleteResourceIn_目录与备份清理() {
    let tmp = tempfile::tempdir().unwrap();
    let base = tmp.path().join("skills");
    let disabled = tmp.path().join("disabled");
    std::fs::create_dir_all(base.join("demo")).unwrap();
    std::fs::write(base.join("demo").join("SKILL.md"), "x").unwrap();
    std::fs::create_dir_all(disabled.join("demo")).unwrap();
    std::fs::write(disabled.join("demo").join("SKILL.md"), "y").unwrap();

    delete_resource_in(&base, &disabled, "demo", None).unwrap();
    assert!(!base.join("demo").exists());
    assert!(!disabled.join("demo").exists(), "disabled 备份应一并清理");
    assert!(delete_resource_in(&base, &disabled, "demo", None).is_err());
}

/// 删除已禁用的 skill：active 侧不存在，仅剩 disabled 备份——删备份即完成删除
#[test]
fn DeleteResourceIn_禁用条目删除() {
    let tmp = tempfile::tempdir().unwrap();
    let base = tmp.path().join("skills");
    let disabled = tmp.path().join("disabled");
    std::fs::create_dir_all(disabled.join("demo")).unwrap();
    std::fs::write(disabled.join("demo").join("SKILL.md"), "x").unwrap();

    delete_resource_in(&base, &disabled, "demo", None).unwrap();

    assert!(!disabled.join("demo").exists(), "禁用条目应被彻底删除");
    assert!(delete_resource_in(&base, &disabled, "demo", None).is_err());
}

// ==================== Hook ====================

/// command 类型 hook 请求（hook 对象由命令构造）
fn hook_req(event: &str, matcher: Option<&str>, command: &str, orig: Option<HookOriginal>) -> SaveHookRequest {
    SaveHookRequest {
        scope: "user".into(),
        event: event.into(),
        matcher: matcher.map(|s| s.into()),
        hook: serde_json::json!({ "type": "command", "command": command }),
        project_path: None,
        original: orig,
        original_scope: None,
    }
}

/// 由 settings 条目算定位键（测试布置与断言共用）
fn entry_key(event: &str, matcher: Option<&str>, entry: &serde_json::Value) -> String {
    crate::store_edit::hook_entry_key(event, matcher, entry)
}

/// 条目构造：matcher 可选；hook 对象原样放入
#[test]
fn BuildHookEntry_形态() {
    let hook = serde_json::json!({ "type": "command", "command": "prettier --write", "timeout": 30 });
    let e = build_hook_entry(Some("Edit|Write"), &hook);
    assert_eq!(e["matcher"], "Edit|Write");
    assert_eq!(e["hooks"][0]["type"], "command");
    assert_eq!(e["hooks"][0]["command"], "prettier --write");
    assert_eq!(e["hooks"][0]["timeout"], 30);

    let e2 = build_hook_entry(None, &serde_json::json!({ "type": "command", "command": "echo hi" }));
    assert!(e2.get("matcher").is_none());
}

/// 签名：command 类型保持原文（老备份 key 兼容）；其他类型为对象 canonical JSON
#[test]
fn HookSignature_类型区分() {
    let cmd = serde_json::json!({ "type": "command", "command": "fmt" });
    assert_eq!(hook_signature(&cmd), "fmt");
    // type 缺省视为 command
    assert_eq!(hook_signature(&serde_json::json!({ "command": "x" })), "x");

    let http = serde_json::json!({ "type": "http", "url": "http://x/h" });
    assert_eq!(hook_signature(&http), serde_json::to_string(&http).unwrap());
    // 不同类型/字段签名不同
    assert_ne!(hook_signature(&http), hook_signature(&cmd));
}

/// 定位：按 (matcher, 条目键) 匹配；matcher 缺失/空白都视为 None
#[test]
fn FindHookIndex_匹配规则() {
    let e0 = serde_json::json!({ "matcher": "Edit", "hooks": [{ "type": "command", "command": "a" }] });
    let e1 = serde_json::json!({ "hooks": [{ "type": "command", "command": "b" }] });
    let entries = vec![e0.clone(), e1.clone()];
    assert_eq!(find_hook_index(&entries, "PreToolUse", Some("Edit"), &entry_key("PreToolUse", Some("Edit"), &e0)), Some(0));
    assert_eq!(find_hook_index(&entries, "PreToolUse", None, &entry_key("PreToolUse", None, &e1)), Some(1));
    // matcher 不匹配 / 键不匹配 → None
    assert_eq!(find_hook_index(&entries, "PreToolUse", Some("Edit"), &entry_key("PreToolUse", None, &e1)), None);
    // 空白 matcher 等价缺失
    let entries2 = vec![serde_json::json!({ "matcher": "  ", "hooks": [{ "command": "c" }] })];
    let k = entry_key("Stop", None, &entries2[0]);
    assert_eq!(find_hook_index(&entries2, "Stop", None, &k), Some(0));
}

/// 新增：settings 为空时创建 hooks.<event>；重复拒绝；非 command 类型可保存
#[test]
fn SaveHookIn_新增与重复拒绝() {
    let tmp = tempfile::tempdir().unwrap();
    let settings = tmp.path().join("settings.json");

    save_hook_in(&settings, &hook_req("PostToolUse", Some("Edit"), "fmt", None)).unwrap();
    let root = read_json(&settings);
    assert_eq!(root["hooks"]["PostToolUse"][0]["matcher"], "Edit");
    assert_eq!(root["hooks"]["PostToolUse"][0]["hooks"][0]["command"], "fmt");

    // 同 (matcher, hook) 重复
    let err = save_hook_in(&settings, &hook_req("PostToolUse", Some("Edit"), "fmt", None))
        .unwrap_err()
        .to_string();
    assert!(err.contains("已存在"), "got: {}", err);

    // 同 matcher 不同命令：允许（同一 matcher 多条命令）
    save_hook_in(&settings, &hook_req("PostToolUse", Some("Edit"), "lint", None)).unwrap();
    let root = read_json(&settings);
    assert_eq!(root["hooks"]["PostToolUse"].as_array().unwrap().len(), 2);

    // 非 command 类型（http/prompt）原样写入
    save_hook_in(
        &settings,
        &SaveHookRequest {
            scope: "user".into(),
            event: "Stop".into(),
            matcher: None,
            hook: serde_json::json!({ "type": "http", "url": "http://x/hook", "timeout": 5 }),
            project_path: None,
            original: None,
            original_scope: None,
        },
    )
    .unwrap();
    let root = read_json(&settings);
    assert_eq!(root["hooks"]["Stop"][0]["hooks"][0]["type"], "http");
    assert_eq!(root["hooks"]["Stop"][0]["hooks"][0]["url"], "http://x/hook");

    // command 类型缺 command → 拒绝
    let err = save_hook_in(
        &settings,
        &SaveHookRequest {
            scope: "user".into(),
            event: "Stop".into(),
            matcher: None,
            hook: serde_json::json!({ "type": "command" }),
            project_path: None,
            original: None,
            original_scope: None,
        },
    )
    .unwrap_err()
    .to_string();
    assert!(err.contains("command"), "got: {}", err);
}

/// 编辑：原条目被替换（含 matcher 变更）；外部改动导致定位失败要报"已被外部修改"
#[test]
fn SaveHookIn_替换与定位失败() {
    let tmp = tempfile::tempdir().unwrap();
    let settings = tmp.path().join("settings.json");
    let old_entry = serde_json::json!({ "matcher": "Bash", "hooks": [{ "type": "command", "command": "old" }] });
    write_json(
        &settings,
        &serde_json::json!({ "hooks": { "PreToolUse": [old_entry.clone()] } }),
    );
    let old_key = entry_key("PreToolUse", Some("Bash"), &old_entry);

    // 替换（matcher 也变，换成 prompt 类型）
    save_hook_in(
        &settings,
        &SaveHookRequest {
            scope: "user".into(),
            event: "PreToolUse".into(),
            matcher: Some("Bash|Python".into()),
            hook: serde_json::json!({ "type": "prompt", "prompt": "检查命令", "model": "haiku" }),
            project_path: None,
            original: Some(HookOriginal { event: "PreToolUse".into(), matcher: Some("Bash".into()), key: old_key }),
            original_scope: None,
        },
    )
    .unwrap();
    let root = read_json(&settings);
    let arr = root["hooks"]["PreToolUse"].as_array().unwrap();
    assert_eq!(arr.len(), 1);
    assert_eq!(arr[0]["matcher"], "Bash|Python");
    assert_eq!(arr[0]["hooks"][0]["type"], "prompt");
    assert_eq!(arr[0]["hooks"][0]["prompt"], "检查命令");

    // 定位失败（键对不上）
    let err = save_hook_in(
        &settings,
        &hook_req(
            "PreToolUse",
            None,
            "x",
            Some(HookOriginal { event: "PreToolUse".into(), matcher: Some("NoSuch".into()), key: "deadbeef".into() }),
        ),
    )
    .unwrap_err()
    .to_string();
    assert!(err.contains("已被外部修改"), "got: {}", err);
}

/// 跨事件编辑：从原事件移除、向新事件追加
#[test]
fn SaveHookIn_跨事件移动() {
    let tmp = tempfile::tempdir().unwrap();
    let settings = tmp.path().join("settings.json");
    let entry = serde_json::json!({ "hooks": [{ "type": "command", "command": "notify" }] });
    write_json(
        &settings,
        &serde_json::json!({ "hooks": { "Stop": [entry.clone()] } }),
    );
    let key = entry_key("Stop", None, &entry);

    save_hook_in(
        &settings,
        &hook_req(
            "SessionStart",
            None,
            "notify",
            Some(HookOriginal { event: "Stop".into(), matcher: None, key }),
        ),
    )
    .unwrap();

    let root = read_json(&settings);
    assert!(root["hooks"].get("Stop").is_none(), "空事件键应移除");
    assert_eq!(root["hooks"]["SessionStart"][0]["hooks"][0]["command"], "notify");
}

/// 删除：彻底移除（不留备份），历史 disabled 备份一并清理；空事件键清理
#[test]
fn DeleteHookIn_彻底删除() {
    let tmp = tempfile::tempdir().unwrap();
    let settings = tmp.path().join("settings.json");
    let disabled = tmp.path().join("disabled");
    let e0 = serde_json::json!({ "matcher": "M", "hooks": [{ "type": "command", "command": "c1" }] });
    let e1 = serde_json::json!({ "hooks": [{ "type": "command", "command": "c2" }] });
    write_json(
        &settings,
        &serde_json::json!({ "hooks": { "UserPromptSubmit": [e0.clone(), e1.clone()] } }),
    );
    // 预置历史备份（模拟此前禁用过）：删除应一并清理
    std::fs::create_dir_all(&disabled).unwrap();
    std::fs::write(disabled.join("hook-x.json"), "{}").unwrap();

    let key1 = entry_key("UserPromptSubmit", None, &e1);
    delete_hook_in(&settings, &disabled, "UserPromptSubmit", &key1, "user").unwrap();
    let root = read_json(&settings);
    let arr = root["hooks"]["UserPromptSubmit"].as_array().unwrap();
    assert_eq!(arr.len(), 1);
    assert!(!disabled.join(format!("hook-{}.json", key1)).exists(), "删除不应留备份");

    // 再删最后一个 → 事件键整个移除
    let key0 = entry_key("UserPromptSubmit", Some("M"), &e0);
    delete_hook_in(&settings, &disabled, "UserPromptSubmit", &key0, "user").unwrap();
    let root = read_json(&settings);
    assert!(root["hooks"].get("UserPromptSubmit").is_none());
}

/// 删除已禁用的 hook：条目只存在于 disabled 备份——删备份即完成删除
#[test]
fn DeleteHookIn_禁用条目删除() {
    let tmp = tempfile::tempdir().unwrap();
    let settings = tmp.path().join("settings.json");
    let disabled = tmp.path().join("disabled");
    // settings 无该事件（条目已被禁用），仅剩 disabled 备份
    write_json(&settings, &serde_json::json!({ "hooks": {} }));
    let entry = serde_json::json!({ "hooks": [{ "type": "command", "command": "c" }] });
    let key = entry_key("Stop", None, &entry);
    std::fs::create_dir_all(&disabled).unwrap();
    std::fs::write(
        disabled.join(format!("hook-{}.json", key)),
        serde_json::json!({ "key": key, "event": "Stop", "entry": entry }).to_string(),
    )
    .unwrap();

    delete_hook_in(&settings, &disabled, "Stop", &key, "user").unwrap();

    assert!(!disabled.join(format!("hook-{}.json", key)).exists());
    // 不存在的 key 仍报错
    assert!(delete_hook_in(&settings, &disabled, "Stop", "nope", "user").is_err());
}

/// 启停：禁用摘出存备份；启用找回删备份
#[test]
fn SetHookEnabledIn_启停往返() {
    let tmp = tempfile::tempdir().unwrap();
    let settings = tmp.path().join("settings.json");
    let disabled = tmp.path().join("disabled");
    let entry = serde_json::json!({ "hooks": [{ "type": "command", "command": "notify" }] });
    write_json(
        &settings,
        &serde_json::json!({ "hooks": { "Stop": [entry.clone()] } }),
    );
    let key = entry_key("Stop", None, &entry);

    set_hook_enabled_in(&settings, &disabled, "Stop", &key, false, "user", None).unwrap();
    let root = read_json(&settings);
    assert!(root["hooks"].get("Stop").is_none());
    let backup = read_json(&disabled.join(format!("hook-user-{}.json", key)));
    assert_eq!(backup["entry"]["hooks"][0]["command"], "notify");

    set_hook_enabled_in(&settings, &disabled, "Stop", &key, true, "user", None).unwrap();
    let root = read_json(&settings);
    assert_eq!(root["hooks"]["Stop"][0]["hooks"][0]["command"], "notify");
    assert!(!disabled.join(format!("hook-user-{}.json", key)).exists());
}
// ==================== 键与隔离 ====================

/// hook_key / project_key：确定性（跨进程一致）+ 区分度
#[test]
fn Keys_确定与区分() {
    assert_eq!(hook_key("Stop", None, "a"), hook_key("Stop", None, "a"));
    assert_ne!(hook_key("Stop", None, "a"), hook_key("Stop", Some("M"), "a"));
    assert_ne!(hook_key("Stop", None, "a"), hook_key("Stop", None, "b"));

    assert_eq!(project_key("D:/a"), project_key("D:/a"));
    assert_ne!(project_key("D:/a"), project_key("D:/b"));
    // 形态：p + 16 位 hex（disabled/hooks 目录扫描依赖此约定）
    assert_eq!(project_key("D:/a").len(), 17);
    assert!(project_key("D:/a").starts_with('p'));
}

// ==================== 源码读取（含禁用兜底） ====================

/// get_skill_source / get_agent_source：active 正常读；条目已被禁用
/// （移入 ~/.cc-box/disabled/）时从备份目录读取——否则面板灰显条目
/// 点开编辑报「Failed to read ...\\.claude\\skills\\...」。
/// 例外于「纯注入」约定：这两个 API 依赖 claude_config_dir/disabled 根，
/// 经 CLAUDE_CONFIG_DIR + TestConfigDirGuard 隔离到临时目录。
#[test]
fn GetResourceSource_禁用条目从备份读取() {
    let _cfg_lock = crate::tests::CONFIG_DIR_TEST_LOCK.lock();
    let tmp = tempfile::tempdir().unwrap();
    let claude_dir = tmp.path().join("claude");
    std::env::set_var("CLAUDE_CONFIG_DIR", &claude_dir);
    let _guard = crate::tests::TestConfigDirGuard::new("resource-source");
    // guard 目录路径规则见 tests/mod.rs（temp_dir + tag + pid），此处按同一规则
    // 在其 disabled/ 子树下布置备份
    let ccbox_dir = std::env::temp_dir().join(format!(
        "cc-box-rpc-test-resource-source-{}",
        std::process::id()
    ));

    // 禁用 skill：active 侧不存在，备份目录有完整目录
    let dskill = ccbox_dir.join("disabled").join("skills").join("d-sk");
    std::fs::create_dir_all(&dskill).unwrap();
    std::fs::write(dskill.join("SKILL.md"), "---\nname: d-sk\n---\n备份正文").unwrap();
    let (content, path) = crate::store_edit::get_skill_source("user", "d-sk", None).unwrap();
    assert_eq!(content, "---\nname: d-sk\n---\n备份正文");
    assert!(path.contains("disabled"), "skill path: {}", path);

    // 禁用 agent：备份目录为单 md 文件
    let dagent = ccbox_dir.join("disabled").join("agents");
    std::fs::create_dir_all(&dagent).unwrap();
    std::fs::write(dagent.join("d-ag.md"), "---\nname: d-ag\n---\nagent备份").unwrap();
    let (content, path) = crate::store_edit::get_agent_source("user", "d-ag", None).unwrap();
    assert_eq!(content, "---\nname: d-ag\n---\nagent备份");
    assert!(path.contains("disabled"), "agent path: {}", path);

    // active 正常读取不走 disabled（路径落在 claude 配置目录下）
    let askill = claude_dir.join("skills").join("a-sk");
    std::fs::create_dir_all(&askill).unwrap();
    std::fs::write(askill.join("SKILL.md"), "active内容").unwrap();
    let (content, path) = crate::store_edit::get_skill_source("user", "a-sk", None).unwrap();
    assert_eq!(content, "active内容");
    assert!(!path.contains("disabled"), "skill path: {}", path);

    // 两边都没有 → 保持原报错语义
    assert!(crate::store_edit::get_skill_source("user", "missing", None).is_err());

    std::env::remove_var("CLAUDE_CONFIG_DIR");
}

/// 跨作用域编辑：skill 整目录 move（user→project）、hook 从旧 settings 移除后
/// 写入新作用域；mcp 从旧作用域配置移除原 key 后写入新作用域。
/// 经 CLAUDE_CONFIG_DIR + TestConfigDirGuard 隔离（同 GetResourceSource 测试）。
#[test]
fn SaveResource_跨作用域移动() {
    let _cfg_lock = crate::tests::CONFIG_DIR_TEST_LOCK.lock();
    let tmp = tempfile::tempdir().unwrap();
    let claude_dir = tmp.path().join("claude");
    std::env::set_var("CLAUDE_CONFIG_DIR", &claude_dir);
    let _guard = crate::tests::TestConfigDirGuard::new("cross-scope");
    let proj = tmp.path().join("proj");
    std::fs::create_dir_all(&proj).unwrap(); // project scope 的 is_dir 校验要求真实存在

    // ---- skill：user → project，同时改名 ----
    let uskill = claude_dir.join("skills").join("old-name");
    std::fs::create_dir_all(&uskill).unwrap();
    std::fs::write(uskill.join("SKILL.md"), "正文").unwrap();
    let skill_req: crate::store_edit::SaveSkillRequest = serde_json::from_value(serde_json::json!({
        "scope": "project", "name": "new-name", "projectPath": proj.to_string_lossy(),
        "originalName": "old-name", "originalScope": "user", "asSource": false,
        "description": "d", "content": "正文"
    }))
    .unwrap();
    crate::store_edit::save_skill(skill_req).unwrap();
    assert!(!uskill.exists(), "旧作用域目录应已 move");
    let moved = proj.join(".claude").join("skills").join("new-name").join("SKILL.md");
    assert!(moved.exists(), "moved path: {}", moved.display());

    // ---- hook：user → project ----
    let settings_user = claude_dir.join("settings.json");
    let entry = serde_json::json!({ "hooks": [{ "type": "command", "command": "notify" }] });
    std::fs::write(
        &settings_user,
        serde_json::to_string_pretty(&serde_json::json!({ "hooks": { "Stop": [entry.clone()] } })).unwrap(),
    )
    .unwrap();
    let key = crate::store_edit::hook_entry_key("Stop", None, &entry);
    let hook_req: crate::store_edit::SaveHookRequest = serde_json::from_value(serde_json::json!({
        "scope": "project", "event": "Stop", "matcher": null,
        "hook": { "type": "command", "command": "notify" },
        "projectPath": proj.to_string_lossy(),
        "original": { "event": "Stop", "matcher": null, "key": key },
        "originalScope": "user"
    }))
    .unwrap();
    crate::store_edit::save_hook(hook_req).unwrap();
    let user_root: serde_json::Value =
        serde_json::from_str(&std::fs::read_to_string(&settings_user).unwrap()).unwrap();
    assert!(user_root.get("hooks").and_then(|h| h.get("Stop")).is_none(), "旧 settings 条目应移除");
    let proj_settings = proj.join(".claude").join("settings.json");
    let proj_root: serde_json::Value =
        serde_json::from_str(&std::fs::read_to_string(&proj_settings).unwrap()).unwrap();
    assert_eq!(proj_root["hooks"]["Stop"][0]["hooks"][0]["command"], "notify");

    // ---- mcp：user → project（.claude.json 尊重 CLAUDE_CONFIG_DIR 后可测）----
    let claude_json = claude_dir.join(".claude.json");
    std::fs::write(
        &claude_json,
        serde_json::to_string_pretty(&serde_json::json!({
            "mcpServers": { "srv": { "type": "stdio", "command": "npx" } }
        }))
        .unwrap(),
    )
    .unwrap();
    let mcp_req: crate::store_edit::SaveMcpRequest = serde_json::from_value(serde_json::json!({
        "scope": "project", "name": "srv",
        "config": { "type": "stdio", "command": "npx" },
        "projectPath": proj.to_string_lossy(),
        "originalName": null, "originalScope": "user"
    }))
    .unwrap();
    crate::store_edit::save_mcp_server(mcp_req).unwrap();
    let json_root: serde_json::Value =
        serde_json::from_str(&std::fs::read_to_string(&claude_json).unwrap()).unwrap();
    assert!(json_root.get("mcpServers").and_then(|m| m.get("srv")).is_none(), "user 作用域原 key 应移除");
    let mcp_json = proj.join(".mcp.json");
    let mcp_root: serde_json::Value =
        serde_json::from_str(&std::fs::read_to_string(&mcp_json).unwrap()).unwrap();
    assert_eq!(mcp_root["mcpServers"]["srv"]["command"], "npx");

    std::env::remove_var("CLAUDE_CONFIG_DIR");
}

/// agent 条目是 <name>.md 文件：删除必须拼上 .md 后缀
/// （曾漏拼后缀，agents/123.md 静默 not found，用户删除无反应）
#[test]
fn DeleteResourceIn_agent文件名条目() {
    let tmp = tempfile::tempdir().unwrap();
    let base = tmp.path().join("agents");
    let disabled = tmp.path().join("disabled");
    std::fs::create_dir_all(&base).unwrap();
    std::fs::create_dir_all(&disabled).unwrap();
    std::fs::write(base.join("123.md"), "x").unwrap();
    std::fs::write(disabled.join("123.md"), "y").unwrap();

    delete_resource_in(&base, &disabled, "123", Some("123.md")).unwrap();
    assert!(!base.join("123.md").exists());
    assert!(!disabled.join("123.md").exists(), "disabled 备份应一并清理");
    // 再删：两侧皆无 → 报错
    assert!(delete_resource_in(&base, &disabled, "123", Some("123.md")).is_err());
}
