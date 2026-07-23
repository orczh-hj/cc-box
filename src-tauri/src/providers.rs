//! Provider 模块
//! API Provider 配置管理（完全兼容 cc-switch 数据结构）

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

/// Provider 元数据（完全兼容 cc-switch ProviderMeta，22 个字段）
/// JSON 存储在 providers.meta 列中
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ProviderMeta {
    pub common_config_enabled: Option<bool>,
    pub endpoint_auto_select: Option<bool>,
    pub api_format: Option<String>,
    pub api_key_field: Option<String>,
    pub is_full_url: Option<bool>,
    pub provider_type: Option<String>,
    pub cost_multiplier: Option<String>,
    pub pricing_model_source: Option<String>,
    pub limit_daily_usd: Option<String>,
    pub limit_monthly_usd: Option<String>,
    pub is_partner: Option<bool>,
    pub partner_promotion_key: Option<String>,
    pub test_config: Option<serde_json::Value>,
    pub usage_script: Option<serde_json::Value>,
    pub auth_binding: Option<serde_json::Value>,
    pub prompt_cache_key: Option<String>,
    pub codex_fast_mode: Option<bool>,
    pub live_config_managed: Option<bool>,
    pub github_account_id: Option<String>,
    pub custom_endpoints: Option<serde_json::Value>,
}

/// Provider 实例（完全兼容 cc-switch Provider）
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Provider {
    pub id: String,
    pub name: String,
    pub settings_config: serde_json::Value,
    pub website_url: Option<String>,
    pub category: Option<String>,
    pub created_at: Option<u64>,
    pub sort_index: Option<u64>,
    pub notes: Option<String>,
    pub meta: Option<ProviderMeta>,
    pub icon: Option<String>,
    pub icon_color: Option<String>,
    #[serde(default)]
    pub in_failover_queue: bool,
}

/// 通用配置片段
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct CommonConfig {
    pub enabled: bool,
    pub settings: serde_json::Value,
}

/// Provider 配置文件结构
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ProvidersConfig {
    pub providers: Vec<Provider>,
    pub common_config: CommonConfig,
    pub active_provider_id: Option<String>,
}

/// 导入结果
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ImportResult {
    pub count: usize,
    pub imported_common_config: bool,
    pub active_provider_name: Option<String>,
}

/// 获取 GUI 配置目录
fn get_gui_config_dir() -> Result<PathBuf> {
    dirs::home_dir()
        .map(|h| h.join(".cc-box"))
        .context("Home directory not found")
}

/// 获取 providers.json 文件路径
fn get_providers_config_path() -> Result<PathBuf> {
    get_gui_config_dir().map(|d| d.join("providers.json"))
}

/// 获取 Claude settings.json 文件路径
fn get_claude_settings_path() -> Result<PathBuf> {
    dirs::home_dir()
        .map(|h| h.join(".claude").join("settings.json"))
        .context("Home directory not found")
}

/// 获取 cc-switch 数据库路径
fn get_cc_switch_db_path() -> Result<PathBuf> {
    dirs::home_dir()
        .map(|h| h.join(".cc-switch").join("cc-switch.db"))
        .context("Home directory not found")
}

/// 读取 Provider 配置
pub fn get_providers_config() -> Result<ProvidersConfig> {
    let config_path = get_providers_config_path()?;

    if !config_path.exists() {
        return Ok(ProvidersConfig {
            providers: Vec::new(),
            common_config: CommonConfig {
                enabled: false,
                settings: serde_json::json!({}),
            },
            active_provider_id: None,
        });
    }

    let content = fs::read_to_string(&config_path)?;
    let config: ProvidersConfig =
        serde_json::from_str(&content).context("Failed to parse providers.json")?;

    Ok(config)
}

/// 保存 Provider 配置
pub fn save_providers_config(config: &ProvidersConfig) -> Result<()> {
    let config_path = get_providers_config_path()?;
    let config_dir = config_path
        .parent()
        .context("Could not get parent directory")?;

    if !config_dir.exists() {
        fs::create_dir_all(config_dir)?;
    }

    let content = serde_json::to_string_pretty(config)?;
    fs::write(&config_path, content)?;

    Ok(())
}

/// 从通用配置的 env 中剥离 6 个核心 Provider 字段，防止导入时覆盖各 Provider 独有的配置
const CORE_ENV_KEYS: &[&str] = &[
    "ANTHROPIC_AUTH_TOKEN",
    "ANTHROPIC_BASE_URL",
    "ANTHROPIC_DEFAULT_HAIKU_MODEL",
    "ANTHROPIC_DEFAULT_OPUS_MODEL",
    "ANTHROPIC_DEFAULT_SONNET_MODEL",
    "ANTHROPIC_MODEL",
];

pub(crate) fn strip_core_env(settings: &serde_json::Value) -> serde_json::Value {
    let mut result = settings.clone();
    if let Some(env) = result.get_mut("env").and_then(|v| v.as_object_mut()) {
        for key in CORE_ENV_KEYS {
            env.remove(*key);
        }
    }
    result
}

/// Claude Code 自管的状态字段，activate 时不得用 Provider 快照覆盖
/// 否则会回滚用户后续通过 CLI（如 `claude plugin enable/disable`）做的修改
const CC_STATE_KEYS: &[&str] = &[
    "enabledPlugins",
    "disabledPlugins",
    "extraKnownMarketplaces",
    "hooks",
];

pub(crate) fn strip_cc_state_fields(settings: &serde_json::Value) -> serde_json::Value {
    let mut result = settings.clone();
    if let Some(obj) = result.as_object_mut() {
        for key in CC_STATE_KEYS {
            obj.remove(*key);
        }
    }
    result
}

/// 深度合并两个 JSON 对象（与 cc-switch json_deep_merge 兼容）
/// source 的值覆盖 target 中同名键，对象递归合并，非对象直接覆盖
pub(crate) fn deep_merge_json(target: &serde_json::Value, source: &serde_json::Value) -> serde_json::Value {
    match (target, source) {
        (serde_json::Value::Object(target_map), serde_json::Value::Object(source_map)) => {
            let mut merged = target_map.clone();
            for (key, source_value) in source_map {
                match merged.get_mut(key) {
                    Some(target_value) => {
                        *target_value = deep_merge_json(target_value, source_value);
                    }
                    None => {
                        merged.insert(key.clone(), source_value.clone());
                    }
                }
            }
            serde_json::Value::Object(merged)
        },
        (_, source) => source.clone(),
    }
}

/// activate 时把 Provider 配置 merge 进当前 settings.json 的纯计算逻辑
/// current = 磁盘现状；provider_cfg 应已通过 strip_cc_state_fields 剥离状态字段
pub(crate) fn merge_provider_into_settings(
    current: &serde_json::Value,
    provider_cfg: &serde_json::Value,
) -> serde_json::Value {
    deep_merge_json(current, provider_cfg)
}

/// 激活 Provider
/// 把 Provider 的 settingsConfig deep merge 进当前 ~/.claude/settings.json
/// 剥离 Claude Code 状态字段（enabledPlugins 等），避免覆盖用户后续 CLI 修改
pub fn activate_provider(provider_id: &str) -> Result<()> {
    let config = get_providers_config()?;

    let provider = config
        .providers
        .iter()
        .find(|p| p.id == provider_id)
        .context("Provider not found")?;

    let settings_path = get_claude_settings_path()?;
    let settings_dir = settings_path
        .parent()
        .context("Could not get Claude settings directory")?;

    if !settings_dir.exists() {
        fs::create_dir_all(settings_dir)?;
    }

    // 读取当前 settings.json（不存在或解析失败视为空对象，保留 Provider 全量字段）
    let current_settings = fs::read_to_string(&settings_path)
        .ok()
        .and_then(|s| serde_json::from_str::<serde_json::Value>(&s).ok())
        .unwrap_or(serde_json::json!({}));

    // 剥离状态字段后 merge 进当前 settings，避免覆盖 CLI 后续修改
    let stripped = strip_cc_state_fields(&provider.settings_config);
    let merged = merge_provider_into_settings(&current_settings, &stripped);

    let content = serde_json::to_string_pretty(&merged)?;
    fs::write(&settings_path, content)?;

    let mut updated_config = config;
    updated_config.active_provider_id = Some(provider_id.to_string());
    save_providers_config(&updated_config)?;

    Ok(())
}

/// 创建 Provider
pub fn create_provider(
    name: String,
    settings_config: serde_json::Value,
    website_url: Option<String>,
    category: Option<String>,
    icon: Option<String>,
    icon_color: Option<String>,
    meta: Option<ProviderMeta>,
) -> Result<Provider> {
    let id = uuid::Uuid::new_v4().to_string();
    let created_at = Some(
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_millis() as u64)
            .unwrap_or(0),
    );

    let provider = Provider {
        id,
        name,
        settings_config,
        website_url,
        category,
        created_at,
        sort_index: None,
        notes: None,
        meta,
        icon,
        icon_color,
        in_failover_queue: false,
    };

    let mut config = get_providers_config()?;
    config.providers.push(provider.clone());
    save_providers_config(&config)?;

    Ok(provider)
}

/// 更新 Provider
pub fn update_provider(
    id: &str,
    name: Option<String>,
    settings_config: Option<serde_json::Value>,
    notes: Option<String>,
    meta: Option<ProviderMeta>,
) -> Result<Provider> {
    let mut config = get_providers_config()?;

    let provider = config
        .providers
        .iter_mut()
        .find(|p| p.id == id)
        .context("Provider not found")?;

    if let Some(n) = name {
        provider.name = n;
    }
    if let Some(s) = settings_config {
        provider.settings_config = s;
    }
    if let Some(n) = notes {
        provider.notes = if n.is_empty() { None } else { Some(n) };
    }
    if let Some(m) = meta {
        provider.meta = Some(m);
    }

    let result = provider.clone();
    save_providers_config(&config)?;

    Ok(result)
}

/// 删除 Provider
pub fn delete_provider(id: &str) -> Result<()> {
    let mut config = get_providers_config()?;

    let index = config
        .providers
        .iter()
        .position(|p| p.id == id)
        .context("Provider not found")?;

    config.providers.remove(index);

    if config.active_provider_id.as_ref() == Some(&id.to_string()) {
        config.active_provider_id = None;
    }

    save_providers_config(&config)?;

    Ok(())
}

/// 更新 Provider 排序
pub fn update_provider_sort_order(provider_ids: Vec<String>) -> Result<()> {
    let mut config = get_providers_config()?;

    for (index, id) in provider_ids.iter().enumerate() {
        if let Some(provider) = config.providers.iter_mut().find(|p| p.id == *id) {
            provider.sort_index = Some(index as u64);
        }
    }

    save_providers_config(&config)?;

    Ok(())
}

/// 更新通用配置
/// 保存通用配置后，将新内容 deepMerge 到所有 meta.commonConfigEnabled === true 的 Provider
/// 入参中的 Claude Code 状态字段会被剥离，避免后续 activate 写回 settings.json 时覆盖 CLI 修改
pub fn update_common_config(enabled: bool, settings: serde_json::Value) -> Result<()> {
    let mut config = get_providers_config()?;
    let safe_settings = strip_cc_state_fields(&settings);
    config.common_config.enabled = enabled;
    config.common_config.settings = safe_settings.clone();

    // 批量合并：遍历所有勾选了"应用通用配置"的 Provider
    for provider in &mut config.providers {
        let should_merge = provider
            .meta
            .as_ref()
            .and_then(|m| m.common_config_enabled)
            .unwrap_or(false);

        if should_merge {
            provider.settings_config = deep_merge_json(&provider.settings_config, &safe_settings);
        }
    }

    save_providers_config(&config)?;

    Ok(())
}

/// 检测 cc-switch 数据库是否存在
pub fn check_cc_switch_db_exists() -> bool {
    get_cc_switch_db_path().map(|p| p.exists()).unwrap_or(false)
}

/// 从 cc-switch 数据库导入 Provider（含通用配置）
/// 读取 providers 表全部字段 + settings 表通用配置 + 识别 is_current
#[cfg(feature = "sqlite")]
pub fn import_from_cc_switch() -> Result<ImportResult> {
    use rusqlite::Connection;

    let db_path = get_cc_switch_db_path()?;

    if !db_path.exists() {
        return Ok(ImportResult {
            count: 0,
            imported_common_config: false,
            active_provider_name: None,
        });
    }

    let conn = Connection::open(&db_path)
        .context("Failed to open cc-switch database")?;

    // 读取 providers 表全部字段，含 is_current
    let mut stmt = conn
        .prepare(
            "SELECT id, name, settings_config, website_url, category, created_at, \
             sort_index, notes, icon, icon_color, meta, in_failover_queue, is_current \
             FROM providers WHERE app_type = 'claude'"
        )
        .context("Failed to prepare query")?;

    let mut current_provider_id: Option<String> = None;
    let rows: Vec<Result<Provider, rusqlite::Error>> = stmt
        .query_map([], |row| {
            let id: String = row.get(0)?;
            let name: String = row.get(1)?;
            let settings_config_str: String = row.get(2)?;
            let settings_config: serde_json::Value =
                serde_json::from_str(&settings_config_str).unwrap_or(serde_json::json!({}));

            let website_url: Option<String> = row.get(3)?;
            let category: Option<String> = row.get(4)?;
            let created_at: Option<u64> = row.get(5)?;
            let sort_index: Option<u64> = row.get(6)?;
            let notes: Option<String> = row.get(7)?;
            let icon: Option<String> = row.get(8)?;
            let icon_color: Option<String> = row.get(9)?;
            let meta_str: Option<String> = row.get(10)?;
            let meta: Option<ProviderMeta> = meta_str
                .and_then(|s| serde_json::from_str(&s).ok());
            let in_failover_queue: bool = row.get(11)?;
            let is_current: bool = row.get::<_, i32>(12)? != 0;

            if is_current {
                current_provider_id = Some(id.clone());
            }

            Ok(Provider {
                id,
                name,
                settings_config,
                website_url,
                category,
                created_at,
                sort_index,
                notes,
                meta,
                icon,
                icon_color,
                in_failover_queue,
            })
        })
        .context("Failed to query providers")?
        .collect();

    let imported_providers: Vec<Provider> = rows
        .into_iter()
        .filter_map(|p| p.ok())
        .collect();
    let count = imported_providers.len();

    // 查找当前激活 Provider 的名称
    let active_provider_name = current_provider_id.as_ref().and_then(|id| {
        imported_providers.iter().find(|p| p.id == *id).map(|p| p.name.clone())
    });

    // 合并到现有配置
    let mut config = get_providers_config()?;

    for provider in imported_providers {
        if !config.providers.iter().any(|p| p.id == provider.id) {
            config.providers.push(provider);
        }
    }

    // 如果识别到 cc-switch 的当前 Provider，且 CC-Box 尚无激活的，设置之
    if config.active_provider_id.is_none() {
        if let Some(ref id) = current_provider_id {
            config.active_provider_id = Some(id.clone());
        }
    }

    // 导入通用配置
    let mut imported_common_config = false;
    if let Ok(common_str) = conn.query_row(
        "SELECT value FROM settings WHERE key = 'common_config_claude'",
        [],
        |row| row.get::<_, String>(0),
    ) {
        if let Ok(common_settings) = serde_json::from_str::<serde_json::Value>(&common_str) {
            if !common_settings.is_null()
                && common_settings.as_object().map_or(false, |o| !o.is_empty())
            {
                config.common_config.enabled = true;
                config.common_config.settings = common_settings.clone();
                imported_common_config = true;

                // 剥离核心 env 字段后再合并，防止覆盖各 Provider 独有的 API Key / 模型等
                // 同时剥离 Claude Code 状态字段，避免 commonConfig 污染 Provider 后再被 activate 写回
                let safe_settings = strip_cc_state_fields(&strip_core_env(&common_settings));

                // 按原则：将通用配置合并到所有 commonConfigEnabled === true 的 Provider
                for provider in &mut config.providers {
                    let should_merge = provider
                        .meta
                        .as_ref()
                        .and_then(|m| m.common_config_enabled)
                        .unwrap_or(false);

                    if should_merge {
                        provider.settings_config =
                            deep_merge_json(&provider.settings_config, &safe_settings);
                    }
                }
            }
        }
    }

    save_providers_config(&config)?;

    Ok(ImportResult {
        count,
        imported_common_config,
        active_provider_name,
    })
}

#[cfg(not(feature = "sqlite"))]
pub fn import_from_cc_switch() -> Result<ImportResult> {
    Ok(ImportResult {
        count: 0,
        imported_common_config: false,
        active_provider_name: None,
    })
}

/// 测试连接结果
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TestConnectionResult {
    pub success: bool,
    pub message: String,
    pub latency_ms: Option<u64>,
}

/// 测试 Provider 连接参数
pub(crate) struct TestConnectionParams {
    pub api_key: String,
    pub model: String,
    pub url: String,
}

/// 从 Provider 的 settingsConfig.env 提取测试连接所需参数
/// 返回 None 表示 api_key 为空（未配置）
pub(crate) fn extract_test_params(settings_config: &serde_json::Value) -> Option<TestConnectionParams> {
    let env = settings_config
        .get("env")
        .and_then(|v| v.as_object())?;

    let api_key = env
        .get("ANTHROPIC_AUTH_TOKEN")
        .or_else(|| env.get("ANTHROPIC_API_KEY"))
        .and_then(|v| v.as_str())
        .unwrap_or("")
        .to_string();

    if api_key.is_empty() {
        return None;
    }

    let base_url = env
        .get("ANTHROPIC_BASE_URL")
        .and_then(|v| v.as_str())
        .unwrap_or("https://api.anthropic.com")
        .to_string();

    let model = env
        .get("ANTHROPIC_MODEL")
        .and_then(|v| v.as_str())
        .unwrap_or("claude-sonnet-4-6")
        .to_string();

    let url = format!("{}/v1/messages", base_url.trim_end_matches('/'));

    Some(TestConnectionParams {
        api_key,
        model,
        url,
    })
}

/// 测试 Provider 连接
/// 从 provider 的 settingsConfig.env 中提取 AUTH_TOKEN、BASE_URL、MODEL，
/// 发送一个最小的 Anthropic Messages API 请求（流式），等待第一个 SSE 事件即判定成功。
pub async fn test_provider_connection(provider_id: &str) -> Result<TestConnectionResult> {
    use futures_util::StreamExt;

    let config = get_providers_config()?;
    let provider = config
        .providers
        .iter()
        .find(|p| p.id == provider_id)
        .context("Provider not found")?;

    let params = match extract_test_params(&provider.settings_config) {
        Some(p) => p,
        None => {
            return Ok(TestConnectionResult {
                success: false,
                message: "未配置 API Key（ANTHROPIC_AUTH_TOKEN 或 ANTHROPIC_API_KEY）".to_string(),
                latency_ms: None,
            });
        }
    };

    let url = params.url;
    let model = params.model;
    let api_key = params.api_key;

    let request_body = serde_json::json!({
        "model": model,
        "max_tokens": 1,
        "stream": true,
        "messages": [
            {
                "role": "user",
                "content": "Hi"
            }
        ]
    });

    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(30))
        .build()
        .context("Failed to build HTTP client")?;

    let start = std::time::Instant::now();

    let response = client
        .post(&url)
        .header("x-api-key", &api_key)
        .header("anthropic-version", "2023-06-01")
        .header("content-type", "application/json")
        .json(&request_body)
        .send()
        .await;

    match response {
        Ok(resp) => {
            let status = resp.status();
            if !status.is_success() {
                let body = resp.text().await.unwrap_or_else(|_| "无法读取响应".to_string());
                return Ok(TestConnectionResult {
                    success: false,
                    message: format!("HTTP {}: {}", status, body.chars().take(200).collect::<String>()),
                    latency_ms: Some(start.elapsed().as_millis() as u64),
                });
            }

            // 读取 SSE 流，等待第一个 data 事件
            let mut stream = resp.bytes_stream();
            let mut buffer = String::new();

            while let Some(chunk) = stream.next().await {
                match chunk {
                    Ok(bytes) => {
                        buffer.push_str(&String::from_utf8_lossy(&bytes));
                        // 检查是否收到了 SSE 事件
                        if buffer.contains("event:") || buffer.contains("data:") {
                            let elapsed = start.elapsed().as_millis() as u64;
                            return Ok(TestConnectionResult {
                                success: true,
                                message: format!("连接成功（模型: {}）", model),
                                latency_ms: Some(elapsed),
                            });
                        }
                    }
                    Err(e) => {
                        return Ok(TestConnectionResult {
                            success: false,
                            message: format!("流读取错误: {}", e),
                            latency_ms: Some(start.elapsed().as_millis() as u64),
                        });
                    }
                }
            }

            // 流结束但没有收到 SSE 事件
            Ok(TestConnectionResult {
                success: false,
                message: "连接已建立但未收到有效响应".to_string(),
                latency_ms: Some(start.elapsed().as_millis() as u64),
            })
        }
        Err(e) => {
            let msg = if e.is_timeout() {
                "连接超时（30秒）".to_string()
            } else if e.is_connect() {
                format!("连接失败: {}", e)
            } else {
                format!("请求错误: {}", e)
            };
            Ok(TestConnectionResult {
                success: false,
                message: msg,
                latency_ms: Some(start.elapsed().as_millis() as u64),
            })
        }
    }
}

