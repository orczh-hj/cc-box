use serde_json::json;
use crate::providers::deep_merge_json;
use crate::providers::extract_test_params;
use crate::providers::merge_provider_into_settings;
use crate::providers::strip_cc_state_fields;
use crate::providers::strip_core_env;

// source 覆盖 target 中同名的叶值
#[test]
fn DeepMerge_LeafValue_001() {
    let target = json!({"a": 1});
    let source = json!({"a": 2});
    let result = deep_merge_json(&target, &source);
    assert_eq!(result, json!({"a": 2}));
}

// source 中 target 没有的 key 被添加
#[test]
fn DeepMerge_NewKey_001() {
    let target = json!({"a": 1});
    let source = json!({"b": 2});
    let result = deep_merge_json(&target, &source);
    assert_eq!(result, json!({"a": 1, "b": 2}));
}

// 嵌套对象递归合并而非整体替换
#[test]
fn DeepMerge_Nested_001() {
    let target = json!({"a": {"x": 1}});
    let source = json!({"a": {"y": 2}});
    let result = deep_merge_json(&target, &source);
    assert_eq!(result, json!({"a": {"x": 1, "y": 2}}));
}

// source 为非对象类型时替换整个 target
#[test]
fn DeepMerge_SourcePrimitive_001() {
    let target = json!({"a": 1});
    let source = json!("string");
    let result = deep_merge_json(&target, &source);
    assert_eq!(result, json!("string"));
}

// target 为非对象类型时被 source 对象替换
#[test]
fn DeepMerge_TargetPrimitive_001() {
    let target = json!(42);
    let source = json!({"a": 1});
    let result = deep_merge_json(&target, &source);
    assert_eq!(result, json!({"a": 1}));
}

// 三层嵌套合并保留所有中间层 key
#[test]
fn DeepMerge_ThreeLevels_001() {
    let target = json!({"l1": {"l2": {"a": 1, "b": 2}}});
    let source = json!({"l1": {"l2": {"b": 3, "c": 4}}});
    let result = deep_merge_json(&target, &source);
    assert_eq!(result, json!({"l1": {"l2": {"a": 1, "b": 3, "c": 4}}}));
}

// 空 source 返回 target 不变
#[test]
fn DeepMerge_EmptySource_001() {
    let target = json!({"a": 1});
    let source = json!({});
    let result = deep_merge_json(&target, &source);
    assert_eq!(result, json!({"a": 1}));
}

// 空 target 返回 source
#[test]
fn DeepMerge_EmptyTarget_001() {
    let target = json!({});
    let source = json!({"a": 1});
    let result = deep_merge_json(&target, &source);
    assert_eq!(result, json!({"a": 1}));
}

// source 中 null 值覆盖 target 对应 key
#[test]
fn DeepMerge_NullValue_001() {
    let target = json!({"a": 1});
    let source = json!({"a": null});
    let result = deep_merge_json(&target, &source);
    assert_eq!(result, json!({"a": null}));
}

// 数组在 source 中直接替换 target 数组不做合并
#[test]
fn DeepMerge_Array_001() {
    let target = json!({"a": [1]});
    let source = json!({"a": [2]});
    let result = deep_merge_json(&target, &source);
    assert_eq!(result, json!({"a": [2]}));
}

// ---------- extract_test_params ----------

// 设置 ANTHROPIC_AUTH_TOKEN 时正确提取 api_key
#[test]
fn ExtractParams_AuthToken_001() {
    let config = json!({
        "env": {
            "ANTHROPIC_AUTH_TOKEN": "sk-test-123",
            "ANTHROPIC_BASE_URL": "https://api.example.com",
            "ANTHROPIC_MODEL": "claude-sonnet-4-6"
        }
    });
    let params = extract_test_params(&config).expect("should extract params");
    assert_eq!(params.api_key, "sk-test-123");
    assert_eq!(params.url, "https://api.example.com/v1/messages");
    assert_eq!(params.model, "claude-sonnet-4-6");
}

// 设置 ANTHROPIC_API_KEY（无 AUTH_TOKEN）时回退提取 api_key
#[test]
fn ExtractParams_ApiKeyFallback_001() {
    let config = json!({
        "env": {
            "ANTHROPIC_API_KEY": "sk-ant-fallback",
            "ANTHROPIC_BASE_URL": "https://api.anthropic.com"
        }
    });
    let params = extract_test_params(&config).expect("should extract params");
    assert_eq!(params.api_key, "sk-ant-fallback");
}

// ANTHROPIC_AUTH_TOKEN 优先于 ANTHROPIC_API_KEY
#[test]
fn ExtractParams_TokenPriority_001() {
    let config = json!({
        "env": {
            "ANTHROPIC_AUTH_TOKEN": "token-primary",
            "ANTHROPIC_API_KEY": "key-secondary"
        }
    });
    let params = extract_test_params(&config).expect("should extract params");
    assert_eq!(params.api_key, "token-primary");
}

// env 中无 API Key 字段时返回 None
#[test]
fn ExtractParams_NoApiKey_001() {
    let config = json!({
        "env": {
            "ANTHROPIC_BASE_URL": "https://api.anthropic.com"
        }
    });
    assert!(extract_test_params(&config).is_none(), "expected None when no api key");
}

// env 中 API Key 为空字符串时返回 None
#[test]
fn ExtractParams_EmptyApiKey_001() {
    let config = json!({
        "env": {
            "ANTHROPIC_AUTH_TOKEN": ""
        }
    });
    assert!(extract_test_params(&config).is_none(), "expected None when api key is empty");
}

// 无 ANTHROPIC_BASE_URL 时默认 https://api.anthropic.com
#[test]
fn ExtractParams_DefaultBaseUrl_001() {
    let config = json!({
        "env": {
            "ANTHROPIC_AUTH_TOKEN": "sk-test"
        }
    });
    let params = extract_test_params(&config).expect("should extract params");
    assert_eq!(params.url, "https://api.anthropic.com/v1/messages");
}

// 无 ANTHROPIC_MODEL 时默认 claude-sonnet-4-6
#[test]
fn ExtractParams_DefaultModel_001() {
    let config = json!({
        "env": {
            "ANTHROPIC_AUTH_TOKEN": "sk-test"
        }
    });
    let params = extract_test_params(&config).expect("should extract params");
    assert_eq!(params.model, "claude-sonnet-4-6");
}

// ANTHROPIC_BASE_URL 尾部有斜杠时拼接 URL 正确去除
#[test]
fn ExtractParams_TrailingSlash_001() {
    let config = json!({
        "env": {
            "ANTHROPIC_AUTH_TOKEN": "sk-test",
            "ANTHROPIC_BASE_URL": "https://api.example.com/"
        }
    });
    let params = extract_test_params(&config).expect("should extract params");
    assert_eq!(params.url, "https://api.example.com/v1/messages");
}

// settingsConfig 中无 env 字段时返回 None
#[test]
fn ExtractParams_NoEnv_001() {
    let config = json!({"model": "claude-sonnet-4-6"});
    assert!(extract_test_params(&config).is_none(), "expected None when no env field");
}

// settingsConfig 为空对象时返回 None
#[test]
fn ExtractParams_EmptyConfig_001() {
    let config = json!({});
    assert!(extract_test_params(&config).is_none(), "expected None for empty config");
}

// ---------- strip_core_env ----------

// 设置 6 个核心 env 后全部被剥离
#[test]
fn StripCoreEnv_AllSix_001() {
    let common = json!({
        "env": {
            "ANTHROPIC_AUTH_TOKEN": "sk-common",
            "ANTHROPIC_BASE_URL": "https://common.example.com",
            "ANTHROPIC_MODEL": "claude-opus-4-7",
            "ANTHROPIC_DEFAULT_HAIKU_MODEL": "haiku",
            "ANTHROPIC_DEFAULT_SONNET_MODEL": "sonnet",
            "ANTHROPIC_DEFAULT_OPUS_MODEL": "opus",
            "CLAUDE_CODE_SCROLL_SPEED": "5"
        },
        "permissions": { "allow": [], "deny": [] }
    });
    let result = strip_core_env(&common);
    let env = result.get("env").unwrap().as_object().unwrap();
    assert!(env.get("ANTHROPIC_AUTH_TOKEN").is_none(), "ANTHROPIC_AUTH_TOKEN should be stripped");
    assert!(env.get("ANTHROPIC_BASE_URL").is_none(), "ANTHROPIC_BASE_URL should be stripped");
    assert!(env.get("ANTHROPIC_MODEL").is_none(), "ANTHROPIC_MODEL should be stripped");
    assert!(env.get("ANTHROPIC_DEFAULT_HAIKU_MODEL").is_none(), "HAIKU should be stripped");
    assert!(env.get("ANTHROPIC_DEFAULT_SONNET_MODEL").is_none(), "SONNET should be stripped");
    assert!(env.get("ANTHROPIC_DEFAULT_OPUS_MODEL").is_none(), "OPUS should be stripped");
    assert_eq!(env.get("CLAUDE_CODE_SCROLL_SPEED").unwrap(), "5", "non-core env should remain");
}

// 设置非核心 env 时全部保留
#[test]
fn StripCoreEnv_NonCorePreserved_001() {
    let common = json!({
        "env": {
            "CLAUDE_CODE_SCROLL_SPEED": "5",
            "MY_CUSTOM_VAR": "hello"
        }
    });
    let result = strip_core_env(&common);
    let env = result.get("env").unwrap().as_object().unwrap();
    assert_eq!(env.len(), 2, "both non-core env vars should remain");
}

// env 为空对象时 strip_core_env 不报错
#[test]
fn StripCoreEnv_EmptyEnv_001() {
    let common = json!({ "env": {} });
    let result = strip_core_env(&common);
    assert_eq!(result, json!({ "env": {} }));
}

// 无 env 字段时 strip_core_env 原样返回
#[test]
fn StripCoreEnv_NoEnvField_001() {
    let common = json!({ "permissions": { "allow": [] } });
    let result = strip_core_env(&common);
    assert_eq!(result, common);
}

// ---------- strip_cc_state_fields ----------

// 4 个 Claude Code 状态字段同时存在时全部被剥离
#[test]
fn StripCcState_RemovesAllKnownKeys_01() {
    let settings = json!({
        "enabledPlugins": { "paper-tool@orczh": true },
        "disabledPlugins": { "foo@bar": true },
        "extraKnownMarketplaces": { "orczh-plugins": { "source": { "source": "directory", "path": "C:/x" } } },
        "hooks": { "PreToolUse": [] },
        "env": { "ANTHROPIC_MODEL": "glm-5" },
        "permissions": { "deny": ["WebSearch"] }
    });
    let result = strip_cc_state_fields(&settings);
    assert!(result.get("enabledPlugins").is_none(), "enabledPlugins should be stripped");
    assert!(result.get("disabledPlugins").is_none(), "disabledPlugins should be stripped");
    assert!(result.get("extraKnownMarketplaces").is_none(), "extraKnownMarketplaces should be stripped");
    assert!(result.get("hooks").is_none(), "hooks should be stripped");
    assert!(result.get("env").is_some(), "env should remain");
    assert!(result.get("permissions").is_some(), "permissions should remain");
}

// 单独剥离 enabledPlugins，保留 env
#[test]
fn StripCcState_RemovesEnabledPlugins_02() {
    let settings = json!({
        "enabledPlugins": { "paper-tool@orczh": true, "user-tool@orczh": false },
        "env": { "ANTHROPIC_BASE_URL": "https://api.example.com" }
    });
    let result = strip_cc_state_fields(&settings);
    assert!(result.get("enabledPlugins").is_none());
    assert_eq!(
        result.get("env").unwrap().get("ANTHROPIC_BASE_URL").unwrap(),
        "https://api.example.com"
    );
}

// permissions、theme、attribution 等用户偏好字段保留不变
#[test]
fn StripCcState_PreservesPermissions_03() {
    let settings = json!({
        "permissions": { "allow": ["Bash(ls:*)"], "deny": ["WebSearch"] },
        "theme": "dark",
        "attribution": { "commit": "Author <a@b.c>" },
        "statusLine": { "type": "command", "command": "echo hi" }
    });
    let result = strip_cc_state_fields(&settings);
    assert_eq!(result, settings, "non-state fields should be preserved verbatim");
}

// 无任何状态键时原样返回
#[test]
fn StripCcState_NoStateKeys_04() {
    let settings = json!({
        "env": { "FOO": "bar" },
        "model": "claude-sonnet-4-6"
    });
    let result = strip_cc_state_fields(&settings);
    assert_eq!(result, settings);
}

// 非对象输入（数组、字符串、数字）原样返回，不报错
#[test]
fn StripCcState_NonObjectInput_05() {
    let arr = json!([1, 2, 3]);
    assert_eq!(strip_cc_state_fields(&arr), arr);

    let str_val = json!("hello");
    assert_eq!(strip_cc_state_fields(&str_val), str_val);

    let num = json!(42);
    assert_eq!(strip_cc_state_fields(&num), num);
}

// 不修改传入的原始 Value（不可变性）
#[test]
fn StripCcState_DoesNotMutateInput_06() {
    let settings = json!({
        "enabledPlugins": { "foo": true },
        "env": { "X": "1" }
    });
    let snapshot = settings.clone();
    let _ = strip_cc_state_fields(&settings);
    assert_eq!(settings, snapshot, "input must not be mutated");
}

// ---------- merge_provider_into_settings ----------

// 核心 bug 回归：current 的 enabledPlugins 在 provider 配置（已剥离）merge 后保留
// 这覆盖了"用户禁用 plugin → 切换 Provider → plugin 状态被回滚"的原始 bug
#[test]
fn MergeSettings_PreservesEnabledPlugins_01() {
    let current = json!({
        "enabledPlugins": { "paper-tool@orczh": false, "user-tool@orczh": true },
        "env": { "ANTHROPIC_MODEL": "glm-5" }
    });
    // 模拟 activate 内部 strip 后的 provider 配置（无 enabledPlugins）
    let provider_cfg = strip_cc_state_fields(&json!({
        "enabledPlugins": { "paper-tool@orczh": true, "user-tool@orczh": true },
        "env": { "ANTHROPIC_AUTH_TOKEN": "new-token" }
    }));
    let merged = merge_provider_into_settings(&current, &provider_cfg);
    assert_eq!(
        merged.get("enabledPlugins").unwrap(),
        &json!({ "paper-tool@orczh": false, "user-tool@orczh": true }),
        "用户 CLI 改的禁用状态必须保留"
    );
    assert_eq!(
        merged.get("env").unwrap().get("ANTHROPIC_AUTH_TOKEN").unwrap(),
        "new-token",
        "Provider 的 API 字段应覆盖"
    );
}

// Provider 的 model 字段覆盖 current
#[test]
fn MergeSettings_OverwritesModel_02() {
    let current = json!({ "model": "old-model" });
    let provider_cfg = json!({ "model": "new-model" });
    let merged = merge_provider_into_settings(&current, &provider_cfg);
    assert_eq!(merged.get("model").unwrap(), "new-model");
}

// env 子键深度合并，而非整体替换
#[test]
fn MergeSettings_MergesEnvDeep_03() {
    let current = json!({
        "env": { "ANTHROPIC_MODEL": "glm-5", "CUSTOM_VAR": "keep-me" }
    });
    let provider_cfg = json!({
        "env": { "ANTHROPIC_MODEL": "glm-6", "ANTHROPIC_AUTH_TOKEN": "tok" }
    });
    let merged = merge_provider_into_settings(&current, &provider_cfg);
    let env = merged.get("env").unwrap().as_object().unwrap();
    assert_eq!(env.get("ANTHROPIC_MODEL").unwrap(), "glm-6", "provider 应覆盖");
    assert_eq!(env.get("CUSTOM_VAR").unwrap(), "keep-me", "current 独有键应保留");
    assert_eq!(env.get("ANTHROPIC_AUTH_TOKEN").unwrap(), "tok");
}

// ---------- 导入合并场景复现 ----------

// 两个 Provider 各自有不同 API Key，通用配置也包含 API Key，
// 剥离后合并不会覆盖 Provider 独有的 API Key
#[test]
fn ImportMerge_PreserveApiKey_001() {
    // Provider A: API Key = key-A
    let provider_a = json!({
        "env": {
            "ANTHROPIC_AUTH_TOKEN": "key-A",
            "ANTHROPIC_BASE_URL": "https://a.example.com",
            "ANTHROPIC_MODEL": "claude-sonnet-4-6"
        },
        "permissions": { "allow": [], "deny": [] }
    });
    // Provider B: API Key = key-B
    let provider_b = json!({
        "env": {
            "ANTHROPIC_AUTH_TOKEN": "key-B",
            "ANTHROPIC_BASE_URL": "https://b.example.com",
            "ANTHROPIC_MODEL": "claude-sonnet-4-6"
        },
        "permissions": { "allow": [], "deny": [] }
    });
    // cc-switch 通用配置包含 API Key 和其他设置
    let common = json!({
        "env": {
            "ANTHROPIC_AUTH_TOKEN": "sk-common-key",
            "ANTHROPIC_BASE_URL": "https://common.example.com",
            "ANTHROPIC_MODEL": "claude-opus-4-7",
            "CLAUDE_CODE_SCROLL_SPEED": "5"
        },
        "permissions": { "allow": ["tool-a"], "deny": [] }
    });

    // 模拟导入流程：先剥离核心 env，再合并
    let safe_common = strip_core_env(&common);
    let merged_a = deep_merge_json(&provider_a, &safe_common);
    let merged_b = deep_merge_json(&provider_b, &safe_common);

    // 验证：各 Provider 的 API Key 保留原值，不被通用配置覆盖
    assert_eq!(
        merged_a["env"]["ANTHROPIC_AUTH_TOKEN"],
        "key-A",
        "Provider A should keep its own API key"
    );
    assert_eq!(
        merged_a["env"]["ANTHROPIC_BASE_URL"],
        "https://a.example.com",
        "Provider A should keep its own base URL"
    );
    assert_eq!(
        merged_b["env"]["ANTHROPIC_AUTH_TOKEN"],
        "key-B",
        "Provider B should keep its own API key"
    );
    assert_eq!(
        merged_b["env"]["ANTHROPIC_BASE_URL"],
        "https://b.example.com",
        "Provider B should keep its own base URL"
    );
    // 验证：非核心 env 被正确合并进来
    assert_eq!(
        merged_a["env"]["CLAUDE_CODE_SCROLL_SPEED"],
        "5",
        "non-core env should be merged in"
    );
    assert_eq!(
        merged_b["env"]["CLAUDE_CODE_SCROLL_SPEED"],
        "5",
        "non-core env should be merged in"
    );
}

// 不剥离时，deep_merge 会用通用配置的 Key 覆盖 Provider 的 Key（复现原始 bug）
#[test]
fn ImportMerge_WithoutStrip_OverwritesKey_001() {
    let provider = json!({
        "env": { "ANTHROPIC_AUTH_TOKEN": "original-key" }
    });
    let common = json!({
        "env": { "ANTHROPIC_AUTH_TOKEN": "common-key" }
    });
    let merged = deep_merge_json(&provider, &common);
    // 验证：不剥离时，deep_merge 确实会覆盖（这就是 bug 的根因）
    assert_eq!(
        merged["env"]["ANTHROPIC_AUTH_TOKEN"],
        "common-key",
        "without strip, common config overwrites provider key"
    );
}
