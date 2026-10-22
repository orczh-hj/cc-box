//! Provider 模块
//! API Provider 配置管理（完全兼容 cc-switch 数据结构）

use anyhow::{anyhow, Context, Result};
use serde::{Deserialize, Serialize};
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};

/// Provider 元数据（完全兼容 cc-switch ProviderMeta，22 个字段）
/// JSON 存储在 providers.meta 列中
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ProviderMeta {
    pub common_config_enabled: Option<bool>,
    /// 需显式屏蔽的通用配置继承路径（JSON Pointer，如 `/env/FOO`）。
    /// 仅在该 Provider 启用通用配置时生效；只抑制继承，不影响显式覆盖。
    #[serde(default)]
    pub common_config_removed: Option<Vec<String>>,
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
    /// 保留 cc-switch 未来新增 / 本结构未声明的字段，避免迁移与保存时静默丢弃
    #[serde(flatten)]
    pub extra: serde_json::Map<String, serde_json::Value>,
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
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CommonConfig {
    pub enabled: bool,
    pub settings: serde_json::Value,
}

impl Default for CommonConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            settings: serde_json::json!({}),
        }
    }
}

/// 上次激活时由 cc-box 写入 settings.json 的托管路径（JSON Pointer）。
/// 用于切换 Provider / 取消继承时删除不再应有的残留键，且不误删用户自己加的键。
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct AppliedSettings {
    pub provider_id: Option<String>,
    pub paths: Vec<String>,
}

/// 当前 providers.json 语义版本。
/// 1 = 旧版（通用配置物化进 Provider）；2 = 分层存储（通用默认 + Provider 覆盖）。
pub const CURRENT_SCHEMA_VERSION: u32 = 2;

fn default_schema_version() -> u32 {
    1
}

/// Provider 配置文件结构
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProvidersConfig {
    /// 旧文件缺失该字段 → 1（需要迁移）
    #[serde(default = "default_schema_version")]
    pub schema_version: u32,
    pub providers: Vec<Provider>,
    pub common_config: CommonConfig,
    pub active_provider_id: Option<String>,
    #[serde(default)]
    pub applied_settings: AppliedSettings,
}

impl Default for ProvidersConfig {
    fn default() -> Self {
        Self {
            schema_version: CURRENT_SCHEMA_VERSION,
            providers: Vec::new(),
            common_config: CommonConfig::default(),
            active_provider_id: None,
            applied_settings: AppliedSettings::default(),
        }
    }
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

/// 获取 Claude settings.json 文件路径（尊重 CLAUDE_CONFIG_DIR，与 store.rs 同一实现）
fn get_claude_settings_path() -> Result<PathBuf> {
    Ok(crate::store::claude_config_dir()?.join("settings.json"))
}

/// 用户级 Claude 配置路径（供「打开用户配置」按钮使用）
pub fn claude_user_settings_path() -> Result<String> {
    Ok(get_claude_settings_path()?.to_string_lossy().to_string())
}

/// 获取 cc-switch 数据库路径
fn get_cc_switch_db_path() -> Result<PathBuf> {
    dirs::home_dir()
        .map(|h| h.join(".cc-switch").join("cc-switch.db"))
        .context("Home directory not found")
}

/// 读取 Provider 配置。
/// 纯读：legacy 文件只在内存中按 v2 语义解释，不写盘；
/// 落盘迁移由应用启动时调用 `ensure_providers_schema()` 完成。
pub fn get_providers_config() -> Result<ProvidersConfig> {
    let config_path = get_providers_config_path()?;
    if !config_path.exists() {
        return Ok(ProvidersConfig::default());
    }

    let content = fs::read_to_string(&config_path)?;
    let mut config: ProvidersConfig =
        serde_json::from_str(&content).context("Failed to parse providers.json")?;

    // 防御：旧版本 cc-box 可能在迁移后又写回 legacy 结构
    if config.schema_version < CURRENT_SCHEMA_VERSION {
        migrate_config(&mut config);
    }

    Ok(config)
}

/// 一次性迁移 legacy providers.json（schema v1 → v2）。
/// 迁移前备份 `providers.json.cc-box.bak`，只执行一次（schemaVersion 守门）。
pub fn ensure_providers_schema() -> Result<()> {
    let config_path = get_providers_config_path()?;
    if !config_path.exists() {
        return Ok(());
    }

    let content = fs::read_to_string(&config_path)?;
    let mut config: ProvidersConfig =
        serde_json::from_str(&content).context("Failed to parse providers.json")?;

    if !migrate_config(&mut config) {
        return Ok(());
    }

    backup_file(&config_path)?;
    save_providers_config(&config)?;
    log::info!("[providers] migrated providers.json to schema v{}", CURRENT_SCHEMA_VERSION);
    Ok(())
}

/// 校验整包配置：Provider / 通用配置根节点必须是对象。
/// `save_providers_config` 可经 RPC 调用，不能只依赖 create/update 的校验。
pub(crate) fn validate_providers_config(config: &ProvidersConfig) -> Result<()> {
    if !config.common_config.settings.is_object() {
        return Err(anyhow!("commonConfig.settings must be a JSON object"));
    }
    for provider in &config.providers {
        if !provider.settings_config.is_object() {
            return Err(anyhow!(
                "Provider {} settingsConfig must be a JSON object",
                provider.id
            ));
        }
    }
    Ok(())
}

/// 保存 Provider 配置
pub fn save_providers_config(config: &ProvidersConfig) -> Result<()> {
    validate_providers_config(config)?;
    let config_path = get_providers_config_path()?;
    let config_dir = config_path
        .parent()
        .context("Could not get parent directory")?;

    if !config_dir.exists() {
        fs::create_dir_all(config_dir)?;
    }

    let content = serde_json::to_string_pretty(config)?;
    write_file_atomic(&config_path, &content)?;

    Ok(())
}

/// 覆盖前备份（`<file>.cc-box.bak`，每次覆盖更新一份）
pub(crate) fn backup_file(path: &Path) -> Result<()> {
    if !path.exists() {
        return Ok(());
    }
    let file_name = path
        .file_name()
        .and_then(|n| n.to_str())
        .context("Invalid file path")?;
    let backup = path.with_file_name(format!("{}.cc-box.bak", file_name));
    fs::copy(path, backup)?;
    Ok(())
}

/// settings.json 常被用户软链到 dotfiles 仓库；直接 rename 会替换链接本身。
/// 写入目标解析为链接指向的真实文件（不存在/断链时回退原路径）。
fn resolve_write_target(path: &Path) -> PathBuf {
    match fs::symlink_metadata(path) {
        Ok(meta) if meta.file_type().is_symlink() => {
            fs::canonicalize(path).unwrap_or_else(|_| path.to_path_buf())
        }
        _ => path.to_path_buf(),
    }
}

/// 原子写入：同目录临时文件 + rename；Windows 目标已存在时先删除再 rename。
pub(crate) fn write_file_atomic(path: &Path, content: &str) -> Result<()> {
    let target = resolve_write_target(path);
    let file_name = target
        .file_name()
        .and_then(|n| n.to_str())
        .context("Invalid file path")?;
    // 唯一临时名：同进程并发写（迁移 + 激活）不会互相截断
    let tmp = target.with_file_name(format!(
        ".{}.{}.{}.tmp",
        file_name,
        std::process::id(),
        uuid::Uuid::new_v4()
    ));

    let write_result = (|| -> Result<()> {
        let mut options = fs::OpenOptions::new();
        options.write(true).create(true).truncate(true);
        // 新建文件默认 0600：settings.json 含 token，不能依赖 umask
        #[cfg(unix)]
        {
            use std::os::unix::fs::OpenOptionsExt;
            options.mode(0o600);
        }
        let mut file = options.open(&tmp)?;
        file.write_all(content.as_bytes())?;
        file.sync_all()?;
        Ok(())
    })();
    if let Err(e) = write_result {
        let _ = fs::remove_file(&tmp);
        return Err(e);
    }

    // 继承目标文件权限：settings.json 常含 API token，不能被 umask 放宽成 0644。
    // 仅 Unix：Windows 只读属性会阻止后续 rename/覆盖。
    #[cfg(unix)]
    if let Ok(metadata) = fs::metadata(&target) {
        if let Err(e) = fs::set_permissions(&tmp, metadata.permissions()) {
            log::warn!(
                "[providers] failed to copy permissions to {}: {}",
                tmp.display(),
                e
            );
        }
    }

    match fs::rename(&tmp, &target) {
        Ok(()) => Ok(()),
        Err(first_err) => {
            // Unix rename 覆盖是原子的，失败说明是权限/磁盘等真实错误，删原文件反而扩大损失。
            // Windows rename 不能覆盖已存在文件：用 copy 覆盖（不先删除目标），
            // 避免二次失败时原文件已被删掉。
            #[cfg(windows)]
            {
                match fs::copy(&tmp, &target) {
                    Ok(_) => {
                        let _ = fs::remove_file(&tmp);
                        Ok(())
                    }
                    Err(e) => {
                        let _ = fs::remove_file(&tmp);
                        Err(anyhow!(
                            "Failed to replace {} (rename: {}, copy: {})",
                            target.display(),
                            first_err,
                            e
                        ))
                    }
                }
            }
            #[cfg(not(windows))]
            {
                let _ = fs::remove_file(&tmp);
                Err(anyhow!(
                    "Failed to write {}: {}",
                    target.display(),
                    first_err
                ))
            }
        }
    }
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

/// 核心 env 的空字符串视为「未设置」：预设大量 `ANTHROPIC_AUTH_TOKEN: ''`，
/// 若按显式值参与分层合成会把用户/通用配置里的有效值覆盖成空串。
pub(crate) fn strip_empty_core_env(settings: &serde_json::Value) -> serde_json::Value {
    let mut result = settings.clone();
    if let Some(env) = result.get_mut("env").and_then(|v| v.as_object_mut()) {
        for key in CORE_ENV_KEYS {
            if env.get(*key).and_then(|v| v.as_str()) == Some("") {
                env.remove(*key);
            }
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

/// 通用配置是否对该 Provider 生效：全局开关 + Provider meta（缺省 true）
pub(crate) fn provider_common_enabled(
    common: &CommonConfig,
    provider: &Provider,
) -> bool {
    if !common.enabled {
        return false;
    }
    provider
        .meta
        .as_ref()
        .and_then(|m| m.common_config_enabled)
        .unwrap_or(true)
}

/// 解析 Provider 的最终生效配置（覆盖层优先）：
/// `deepMerge(通用配置 - removed 路径, Provider 覆盖层)`。
pub(crate) fn resolve_effective(common: &CommonConfig, provider: &Provider) -> serde_json::Value {
    let overrides = strip_empty_core_env(&strip_cc_state_fields(&provider.settings_config));
    if !provider_common_enabled(common, provider) {
        return overrides;
    }

    let mut base = strip_empty_core_env(&strip_cc_state_fields(&common.settings));
    if let Some(removed) = provider
        .meta
        .as_ref()
        .and_then(|m| m.common_config_removed.as_ref())
    {
        for pointer in removed {
            delete_at_json_pointer(&mut base, pointer);
        }
    }

    deep_merge_json(&base, &overrides)
}

/// JSON 深度相等（对象键序无关）
pub(crate) fn deep_equal(a: &serde_json::Value, b: &serde_json::Value) -> bool {
    match (a, b) {
        (serde_json::Value::Object(am), serde_json::Value::Object(bm)) => {
            am.len() == bm.len()
                && am
                    .iter()
                    .all(|(k, av)| bm.get(k).map_or(false, |bv| deep_equal(av, bv)))
        }
        (serde_json::Value::Array(aa), serde_json::Value::Array(ba)) => {
            aa.len() == ba.len() && aa.iter().zip(ba).all(|(x, y)| deep_equal(x, y))
        }
        _ => a == b,
    }
}

/// 从 value 中移除与 base 相同的叶子（递归），用于迁移旧版物化配置：
/// 与通用配置相同的值视为「继承」，不同或 base 没有的视为「显式覆盖」。
/// 返回 None 表示该子树与 base 完全一致、可以整体丢弃。
pub(crate) fn subtract_equal(
    value: &serde_json::Value,
    base: &serde_json::Value,
) -> Option<serde_json::Value> {
    if deep_equal(value, base) {
        return None;
    }
    match (value, base) {
        (serde_json::Value::Object(vm), serde_json::Value::Object(bm)) => {
            let mut out = serde_json::Map::new();
            for (key, vv) in vm {
                match bm.get(key) {
                    Some(bv) => {
                        if let Some(reduced) = subtract_equal(vv, bv) {
                            out.insert(key.clone(), reduced);
                        }
                    }
                    None => {
                        out.insert(key.clone(), vv.clone());
                    }
                }
            }
            if out.is_empty() {
                None
            } else {
                Some(serde_json::Value::Object(out))
            }
        }
        _ => Some(value.clone()),
    }
}

/// JSON Pointer token 转义（RFC 6901）
pub(crate) fn json_pointer_escape(token: &str) -> String {
    token.replace('~', "~0").replace('/', "~1")
}

fn json_pointer_unescape(token: &str) -> String {
    token.replace("~1", "/").replace("~0", "~")
}

/// 按 JSON Pointer 删除对象叶子；路径不存在返回 false。
/// 路径只遍历对象（数组在设计中是叶子，不会被指向内部）。
pub(crate) fn delete_at_json_pointer(root: &mut serde_json::Value, pointer: &str) -> bool {
    let Some(rest) = pointer.strip_prefix('/') else {
        return false;
    };
    if rest.is_empty() {
        return false;
    }
    let tokens: Vec<String> = rest.split('/').map(json_pointer_unescape).collect();

    let mut current = root;
    for token in &tokens[..tokens.len() - 1] {
        match current.get_mut(token) {
            Some(next) => current = next,
            None => return false,
        }
    }

    match current.as_object_mut() {
        Some(obj) => obj.remove(tokens.last().unwrap()).is_some(),
        None => false,
    }
}

/// 收集叶子路径（对象递归；数组/标量视作叶子）。
pub(crate) fn collect_leaf_paths(
    value: &serde_json::Value,
    prefix: &str,
    out: &mut Vec<String>,
) {
    match value {
        serde_json::Value::Object(map) => {
            for (key, child) in map {
                let path = format!("{}/{}", prefix, json_pointer_escape(key));
                collect_leaf_paths(child, &path, out);
            }
        }
        _ => {
            if !prefix.is_empty() {
                out.push(prefix.to_string());
            }
        }
    }
}

pub(crate) fn leaf_paths(value: &serde_json::Value) -> Vec<String> {
    let mut out = Vec::new();
    collect_leaf_paths(value, "", &mut out);
    out.sort();
    out
}

/// 激活合并的纯计算：先删除上次托管、本次不应存在的残留路径，再合并 effective。
pub(crate) fn apply_effective_to_settings(
    current: &serde_json::Value,
    effective: &serde_json::Value,
    stale_paths: &[String],
) -> serde_json::Value {
    let mut merged = current.clone();
    for pointer in stale_paths {
        delete_at_json_pointer(&mut merged, pointer);
    }
    deep_merge_json(&merged, effective)
}

/// 解析用户 settings.json；坏 JSON / 非对象根节点都拒绝覆盖。
pub(crate) fn parse_settings_json(content: &str) -> Result<serde_json::Value> {
    let value: serde_json::Value =
        serde_json::from_str(content).context("settings.json is not valid JSON")?;
    if !value.is_object() {
        return Err(anyhow!("settings.json root must be a JSON object"));
    }
    Ok(value)
}

/// 迁移 legacy（schema v1）配置到分层语义。
/// 旧版把通用配置物化进了 Provider，且通用配置赢；
/// 移除与通用配置相同的叶子后，effective 与迁移前完全一致。
/// 返回是否发生了迁移。
pub(crate) fn migrate_config(config: &mut ProvidersConfig) -> bool {
    if config.schema_version >= CURRENT_SCHEMA_VERSION {
        return false;
    }

    if !config.common_config.settings.is_object() {
        config.common_config.settings = serde_json::json!({});
    }
    let common = strip_cc_state_fields(&config.common_config.settings);
    let common_active = config.common_config.enabled;

    for provider in &mut config.providers {
        let enabled = provider
            .meta
            .as_ref()
            .and_then(|m| m.common_config_enabled)
            .unwrap_or(true);

        // 状态字段不参与分层，统一从存储中清掉（激活时本来也会被剥离）
        provider.settings_config = strip_cc_state_fields(&provider.settings_config);
        // 非法根节点（数组/null/标量）归一化为空覆盖层，避免激活写坏 settings.json
        if !provider.settings_config.is_object() {
            log::warn!(
                "[providers] provider {} has non-object settingsConfig; reset to {{}}",
                provider.id
            );
            provider.settings_config = serde_json::json!({});
        }

        if common_active && enabled {
            provider.settings_config = subtract_equal(&provider.settings_config, &common)
                .unwrap_or_else(|| serde_json::json!({}));
        }

        // 显式化缺省值，消除前端 true / 后端 false 的历史歧义
        let meta = provider.meta.get_or_insert_with(ProviderMeta::default);
        meta.common_config_enabled = Some(enabled);
        if meta.common_config_removed.is_none() {
            meta.common_config_removed = Some(Vec::new());
        }
    }

    config.schema_version = CURRENT_SCHEMA_VERSION;
    true
}

/// 旧版激活合并的纯计算逻辑（仅测试保留，回归用户现状保护语义）。
/// 现行激活走 `apply_effective_to_settings`。
#[cfg(test)]
pub(crate) fn merge_provider_into_settings(
    current: &serde_json::Value,
    provider_cfg: &serde_json::Value,
) -> serde_json::Value {
    deep_merge_json(current, provider_cfg)
}

/// 激活 Provider
/// effective = 通用默认层 + Provider 覆盖层（Provider 优先，状态字段已剥离），
/// 再 merge 进用户 ~/.claude/settings.json：用户独有键保留，旧托管键按 appliedSettings 清理。
pub fn activate_provider(provider_id: &str) -> Result<()> {
    let config = get_providers_config()?;

    let effective = {
        let provider = config
            .providers
            .iter()
            .find(|p| p.id == provider_id)
            .context("Provider not found")?;
        resolve_effective(&config.common_config, provider)
    };
    // 非对象根会把 settings.json 写坏，后续所有激活都被拒绝
    if !effective.is_object() {
        return Err(anyhow!(
            "Provider config root must be a JSON object (id={})",
            provider_id
        ));
    }
    let new_paths = leaf_paths(&effective);

    let settings_path = get_claude_settings_path()?;
    let settings_dir = settings_path
        .parent()
        .context("Could not get Claude settings directory")?;
    if !settings_dir.exists() {
        fs::create_dir_all(settings_dir)?;
    }

    // 用户现状是底座；坏 JSON 拒绝覆盖（此前会静默当成 {} 写回，造成数据丢失）
    let current_settings = if settings_path.exists() {
        let raw = fs::read_to_string(&settings_path)?;
        parse_settings_json(&raw)
            .with_context(|| format!("Refusing to overwrite {}", settings_path.display()))?
    } else {
        serde_json::json!({})
    };

    // 覆盖前备份（settings.json.cc-box.bak）
    backup_file(&settings_path)?;

    // 只清理「上次由 cc-box 写入、本次 effective 不再包含」的路径
    let stale_paths: Vec<String> = config
        .applied_settings
        .paths
        .iter()
        .filter(|p| !new_paths.contains(p))
        .cloned()
        .collect();

    let merged = apply_effective_to_settings(&current_settings, &effective, &stale_paths);
    let content = serde_json::to_string_pretty(&merged)?;
    write_file_atomic(&settings_path, &content)?;

    let mut updated_config = config;
    updated_config.active_provider_id = Some(provider_id.to_string());
    updated_config.applied_settings = AppliedSettings {
        provider_id: Some(provider_id.to_string()),
        paths: new_paths,
    };
    if let Err(e) = save_providers_config(&updated_config) {
        // settings.json 已更新但 providers.json 失败：记录后返回，避免静默状态漂移
        log::error!(
            "[providers] settings.json updated but providers.json save failed: {}",
            e
        );
        return Err(e);
    }

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
    if !settings_config.is_object() {
        return Err(anyhow!("Provider config root must be a JSON object"));
    }
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

/// 合并 Provider meta：incoming 未提供的关键字段沿用现有值。
/// `ProviderMeta` 是整对象替换语义，旧客户端只传部分字段时会丢新字段。
fn meta_to_non_null_value(meta: &ProviderMeta) -> serde_json::Value {
    let mut value = serde_json::to_value(meta).unwrap_or_else(|_| serde_json::json!({}));
    if let Some(obj) = value.as_object_mut() {
        // Option None 序列化为 null；视为「未提供」，不参与覆盖
        obj.retain(|_, v| !v.is_null());
    }
    value
}

pub(crate) fn merge_provider_meta(
    existing: &Option<ProviderMeta>,
    incoming: ProviderMeta,
) -> ProviderMeta {
    let Some(existing) = existing else {
        return incoming;
    };
    // existing 打底 + incoming 非空字段覆盖；未知字段（extra/flatten）也一并保留
    let merged = deep_merge_json(
        &meta_to_non_null_value(existing),
        &meta_to_non_null_value(&incoming),
    );
    serde_json::from_value(merged).unwrap_or(incoming)
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
        if !s.is_object() {
            return Err(anyhow!("Provider config root must be a JSON object"));
        }
        provider.settings_config = s;
    }
    if let Some(n) = notes {
        provider.notes = if n.is_empty() { None } else { Some(n) };
    }
    if let Some(m) = meta {
        // 部分更新（旧前端/第三方客户端可能只传 commonConfigEnabled）不得清空
        // commonConfigRemoved 等新字段
        let merged = merge_provider_meta(&provider.meta, m);
        provider.meta = Some(merged);
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

/// 更新通用配置（分层语义：只保存默认层，不改写任何 Provider）。
/// 调用方保存后重激活当前 Provider，新的默认值才会写入 settings.json。
/// Claude Code 状态字段在这里剥离，避免经通用配置回写覆盖 CLI 状态。
pub fn update_common_config(enabled: bool, settings: serde_json::Value) -> Result<()> {
    if !settings.is_object() {
        return Err(anyhow!("Common config must be a JSON object"));
    }

    let mut config = get_providers_config()?;
    config.common_config.enabled = enabled;
    config.common_config.settings = strip_cc_state_fields(&settings);
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

    // 合并到现有配置（get 内部已完成 legacy → v2 迁移）
    let mut config = get_providers_config()?;

    // 先导入通用配置：分层语义下它只作为默认层，不再物化进 Provider
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
                // 与旧行为一致：核心 env 不作为通用默认值，避免共享 API Key / 模型
                config.common_config.enabled = true;
                config.common_config.settings =
                    strip_cc_state_fields(&strip_core_env(&common_settings));
                imported_common_config = true;
            }
        }
    }

    // 导入的 Provider 是 cc-switch 物化后的配置：归一化为覆盖层。
    // （与通用配置相同的叶子视为继承；不同或通用配置没有的保留为显式覆盖）
    for mut provider in imported_providers {
        if config.providers.iter().any(|p| p.id == provider.id) {
            continue;
        }

        let enabled = provider
            .meta
            .as_ref()
            .and_then(|m| m.common_config_enabled)
            .unwrap_or(true);

        // 导入数据可能是非对象根节点，先归一化
        if !provider.settings_config.is_object() {
            provider.settings_config = serde_json::json!({});
        }

        if config.common_config.enabled && enabled {
            provider.settings_config =
                subtract_equal(&provider.settings_config, &config.common_config.settings)
                    .unwrap_or_else(|| serde_json::json!({}));
        }

        let meta = provider.meta.get_or_insert_with(ProviderMeta::default);
        meta.common_config_enabled = Some(enabled);
        if meta.common_config_removed.is_none() {
            meta.common_config_removed = Some(Vec::new());
        }

        config.providers.push(provider);
    }

    // 如果识别到 cc-switch 的当前 Provider，且 CC-Box 尚无激活的，设置之
    if config.active_provider_id.is_none() {
        if let Some(ref id) = current_provider_id {
            config.active_provider_id = Some(id.clone());
        }
    }

    config.schema_version = CURRENT_SCHEMA_VERSION;
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
    // 测试连接必须使用合并通用配置后的生效配置（覆盖层优先）
    let effective = resolve_effective(&config.common_config, provider);

    let params = match extract_test_params(&effective) {
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

