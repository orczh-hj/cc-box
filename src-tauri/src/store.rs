//! Store 模块
//! Claude Code 原生数据读取

use anyhow::{bail, Context, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::{Component, Path, PathBuf};
use std::sync::Mutex;

/// 项目信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Project {
    pub path: String,
    pub name: String,
    pub last_session_id: Option<String>,
    pub last_cost: Option<f64>,
    #[serde(rename = "lastDuration")]
    pub last_duration: Option<u64>,
}

/// 会话信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionInfo {
    #[serde(rename = "sessionId")]
    pub session_id: String,
    pub name: String,
    #[serde(rename = "projectPath")]
    pub project_path: String,
    #[serde(rename = "lastActiveAt")]
    pub last_active_at: u64,
}

/// 会话详情
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionDetails {
    #[serde(rename = "sessionId")]
    pub session_id: String,
    pub name: String,
    #[serde(rename = "messageCount")]
    pub message_count: usize,
    #[serde(rename = "totalTokens")]
    pub total_tokens: Option<u64>,
    #[serde(rename = "totalCost")]
    pub total_cost: Option<f64>,
    #[serde(rename = "createdAt")]
    pub created_at: Option<u64>,
    #[serde(rename = "lastActiveAt")]
    pub last_active_at: u64,
}

/// 应用配置
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AppConfig {
    #[serde(rename = "defaultContinue")]
    pub default_continue: Option<bool>,
    #[serde(rename = "defaultSkipPermissions")]
    pub default_skip_permissions: Option<bool>,
    #[serde(rename = "defaultCustomArgs")]
    pub default_custom_args: Option<String>,
    pub theme: Option<String>,
    #[serde(rename = "fontSize")]
    pub font_size: Option<u16>,
    #[serde(rename = "autoConnectIde")]
    pub auto_connect_ide: Option<bool>,
    #[serde(rename = "hiddenProjects")]
    pub hidden_projects: Option<Vec<String>>,
    #[serde(rename = "lastOpenedProject")]
    pub last_opened_project: Option<String>,
    #[serde(rename = "windowSize")]
    pub window_size: Option<WindowSize>,
    #[serde(rename = "claudePath")]
    pub claude_path: Option<String>,
    #[serde(rename = "gitBashPath")]
    pub git_bash_path: Option<String>,
    #[serde(rename = "claudeEnvVars")]
    pub claude_env_vars: Option<std::collections::HashMap<String, String>>,
    pub language: Option<String>,
    #[serde(rename = "remote")]
    pub remote: Option<RemoteConfig>,
    /// 用户添加的市场（仅入 CC-Box 列表可浏览；安装插件时才懒注册到 Claude）
    #[serde(rename = "customMarkets", default)]
    pub custom_markets: Option<Vec<CustomMarket>>,
}

/// 用户添加的市场记录（CC-Box 自己的列表，非 Claude 注册表）
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct CustomMarket {
    pub name: String,
    /// GitHub owner/repo / git URL / marketplace.json URL / 本地目录
    pub source: String,
}

/// 远程连接配置（局域网直连 + 中继）
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct RemoteConfig {
    pub enabled: Option<bool>,
    pub token: Option<String>,
    pub port: Option<u16>,
    /// 中继服务器地址（如 https://cc.orczh.top；空 = 未启用）
    pub relay_url: Option<String>,
    /// 设备身份（首次生成后持久化）
    pub device_id: Option<String>,
    /// 设备密钥（64 hex，注册中继用）
    pub device_key: Option<String>,
    /// 文件传输大小上限（字节）：直连默认 200MB（局域网零流量成本）；0 = 禁用传输
    pub file_transfer_max_direct: Option<u64>,
    /// 文件传输大小上限（字节）：经中继默认 20MB（中继流量按量计费，实际消耗 ≈2.7×
    /// 文件大小：双程 + base64 膨胀 1.33×；局域网自建 relay 可调高）；0 = 禁用
    pub file_transfer_max_relay: Option<u64>,
    /// 中继已配对设备 secret 列表（token v2 `<device_id>.<secret>` 的桌面校验依据；
    /// 首次连接时固化，LRU 上限 16；rotate 设备身份时清空）
    pub paired_secrets: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WindowSize {
    pub width: u32,
    pub height: u32,
}

/// 默认 Claude 选项
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DefaultClaudeOptions {
    #[serde(rename = "continue")]
    pub continue_opt: Option<bool>,
    pub resume: Option<String>,
    #[serde(rename = "skipPermissions")]
    pub skip_permissions: Option<bool>,
    #[serde(rename = "customArgs")]
    pub custom_args: Option<String>,
}

/// 配置来源
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfigSource {
    #[serde(rename = "type")]
    pub source_type: String,
    pub label: String,
    pub path: Option<String>,
}

/// 基本配置项
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BasicConfigItem {
    pub model: Option<String>,
    pub theme: Option<String>,
    #[serde(rename = "editorMode")]
    pub editor_mode: Option<String>,
    #[serde(rename = "autoConnectIde")]
    pub auto_connect_ide: Option<bool>,
    pub permissions: Option<PermissionsConfig>,
    pub env: Option<serde_json::Value>,
    pub source: ConfigSource,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PermissionsConfig {
    pub allow: Option<Vec<String>>,
    pub deny: Option<Vec<String>>,
}

/// MCP 服务器配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct McpServerItem {
    pub name: String,
    #[serde(rename = "type")]
    pub server_type: Option<String>,
    pub command: Option<String>,
    pub args: Option<Vec<String>>,
    pub env: Option<serde_json::Value>,
    pub url: Option<String>,
    pub source: ConfigSource,
}

/// Skill 配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkillItem {
    pub name: String,
    pub description: Option<String>,
    pub path: String,
    pub source: ConfigSource,
}

/// Agent 配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentItem {
    pub name: String,
    pub description: Option<String>,
    pub path: String,
    pub source: ConfigSource,
}

/// Hook 配置
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HookItem {
    pub event: String,
    pub matcher: Option<String>,
    /// 展示摘要：command 类型 = 命令；http = url；prompt/agent = prompt（CSS 截断）
    pub command: Option<String>,
    #[serde(rename = "hookType")]
    pub hook_type: Option<String>,
    /// matcher 条目内 hook 命令数（>1 时前端禁止表单编辑，防止整条替换丢命令）
    #[serde(default)]
    pub hook_count: usize,
    /// 首个 hook 的超时（秒；编辑器回显用）
    #[serde(default)]
    pub timeout: Option<u64>,
    pub source: ConfigSource,
    /// 原始 matcher 层条目（get_hook_list 计算定位键用，不下发前端）
    #[serde(default, skip_serializing)]
    pub raw: Option<serde_json::Value>,
}

/// matcher 条目首个 hook 对象（hook_type/command/timeout/raw 的公共取值）
pub(crate) fn hook_first(hook_entry: &serde_json::Value) -> Option<&serde_json::Value> {
    hook_entry
        .get("hooks")
        .and_then(|h| h.as_array())
        .and_then(|a| a.first())
}

/// 展示摘要：command → url → prompt（多类型 hook 的面板摘要）
pub(crate) fn hook_summary(first: Option<&serde_json::Value>) -> Option<String> {
    let first = first?;
    for key in ["command", "url", "prompt"] {
        if let Some(s) = first.get(key).and_then(|v| v.as_str()) {
            if !s.trim().is_empty() {
                return Some(s.to_string());
            }
        }
    }
    None
}

/// 项目配置结果
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ProjectConfig {
    pub basic: Vec<BasicConfigItem>,
    pub mcp: Vec<McpServerItem>,
    pub skills: Vec<SkillItem>,
    pub agents: Vec<AgentItem>,
    pub hooks: Vec<HookItem>,
}

/// 真实项目路径 → Claude 项目目录列表的缓存
/// 同一 real_path 可能对应多个 Claude 项目目录（如编码规则变更后新旧目录共存）
static PROJECT_PATH_MAPPING: Mutex<Option<HashMap<String, Vec<PathBuf>>>> = Mutex::new(None);

// ==================== 辅助函数 ====================

/// 获取 Claude 配置目录
fn get_claude_dir() -> Result<PathBuf> {
    claude_config_dir()
}

/// Claude 配置目录：尊重 `CLAUDE_CONFIG_DIR`（CLI 支持自定义配置目录），
/// 未设置时回退 `~/.claude`。硬编码 home 会让自定义配置目录的用户
/// 在 GUI 里改到另一份数据（操作对 CLI 完全无效）。
pub(crate) fn claude_config_dir() -> Result<PathBuf> {
    if let Some(dir) = std::env::var_os("CLAUDE_CONFIG_DIR") {
        let dir = PathBuf::from(dir);
        if !dir.as_os_str().is_empty() {
            return Ok(dir);
        }
    }
    dirs::home_dir()
        .map(|h| h.join(".claude"))
        .context("Home directory not found")
}

/// serde 默认值=true（用于 enabled 字段向后兼容）
fn default_true() -> bool {
    true
}

/// Claude 全局状态文件 `.claude.json`（mcpServers / projects 等）：
/// 设置了 `CLAUDE_CONFIG_DIR` 时在其内（测试注入也走这里），否则在
/// **家目录根** `~/.claude.json`（不在 `~/.claude/` 里——写错位置 =
/// 保存对 Claude 与面板完全无效，曾因此把远程保存写进幽灵文件）。
pub(crate) fn claude_state_json_path() -> Result<PathBuf> {
    if let Some(dir) = std::env::var_os("CLAUDE_CONFIG_DIR") {
        let dir = PathBuf::from(dir);
        if !dir.as_os_str().is_empty() {
            return Ok(dir.join(".claude.json"));
        }
    }
    dirs::home_dir()
        .map(|h| h.join(".claude.json"))
        .context("Home directory not found")
}

/// 测试注入：覆盖 GUI 配置目录（隔离真实 ~/.cc-box）。remote_state 的 rpc
/// 全链路测试经真实 update_app_config 落盘，曾把各开发者的 theme 污染成 dark
#[cfg(test)]
pub(crate) static GUI_CONFIG_DIR_OVERRIDE: std::sync::LazyLock<
    parking_lot::Mutex<Option<PathBuf>>,
> = std::sync::LazyLock::new(|| parking_lot::Mutex::new(None));

/// 获取 GUI 配置目录
pub(crate) fn get_gui_config_dir() -> Result<PathBuf> {
    #[cfg(test)]
    if let Some(dir) = GUI_CONFIG_DIR_OVERRIDE.lock().clone() {
        return Ok(dir);
    }
    // CI/E2E 隔离：生产 Broker 与 GUI 共享同一目录
    if let Some(dir) = std::env::var_os("CC_BOX_CONFIG_DIR") {
        return Ok(PathBuf::from(dir));
    }
    dirs::home_dir()
        .map(|h| h.join(".cc-box"))
        .context("Home directory not found")
}

/// 获取 GUI 配置文件路径
fn get_gui_config_path() -> Result<PathBuf> {
    get_gui_config_dir().map(|d| d.join("config.json"))
}

/// 扫描 ~/.claude/projects/ 构建真实路径到项目目录的映射
/// 每个目录通过读取 JSONL 中的 cwd 字段获取真实项目路径
fn build_project_path_mapping() -> HashMap<String, Vec<PathBuf>> {
    let claude_dir = match get_claude_dir() {
        Ok(d) => d,
        Err(_) => return HashMap::new(),
    };
    let projects_dir = claude_dir.join("projects");

    if !projects_dir.exists() {
        return HashMap::new();
    }

    let mut mapping: HashMap<String, Vec<PathBuf>> = HashMap::new();

    if let Ok(entries) = fs::read_dir(&projects_dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if !path.is_dir() {
                continue;
            }

            if let Some(real_path) = extract_project_path_from_jsonl(&path) {
                mapping.entry(real_path).or_default().push(path);
            }
        }
    }

    mapping
}

/// 根据真实项目路径查找对应的 Claude 项目目录列表
/// 使用缓存避免重复扫描
fn get_project_dirs(project_path: &str) -> Vec<PathBuf> {
    let mut cache = PROJECT_PATH_MAPPING
        .lock()
        .unwrap_or_else(|e| e.into_inner());

    if cache.is_none() {
        *cache = Some(build_project_path_mapping());
    }

    cache
        .as_ref()
        .and_then(|m| m.get(project_path))
        .cloned()
        .unwrap_or_default()
}

/// 清除项目路径映射缓存（供外部调用以强制刷新）
pub fn invalidate_project_path_mapping() {
    let mut cache = PROJECT_PATH_MAPPING
        .lock()
        .unwrap_or_else(|e| e.into_inner());
    *cache = None;
}

/// 从 JSONL 文件提取真实项目路径
///
/// 日志分级：目录里存在可读的会话文件却提取不出 cwd 才 warn（CLI 格式
/// 变化信号）；空目录 / 仅 agent- 文件 / 文件读取失败是正常状态（会话
/// 删除后的残留目录、权限、编码），debug 即可——否则 error.log 被无害
/// 信息刷屏（Mac 实例曾每次首页刷新刷 3 条 warn）
fn extract_project_path_from_jsonl(project_dir: &Path) -> Option<String> {
    if !project_dir.exists() {
        return None;
    }

    // 从 JSONL/TXT 内容提取 cwd（单个目录项读取失败跳过该项，不放弃整个目录）
    let mut saw_readable_session_file = false;
    for entry in fs::read_dir(project_dir).ok()?.flatten() {
        let path = entry.path();

        let ext = path.extension().and_then(|e| e.to_str()).unwrap_or("");
        // 只读取非 agent 开头的 jsonl 或 txt 文件
        if (ext == "jsonl" || ext == "txt")
            && !path
                .file_name()
                .map(|n| n.to_str().unwrap_or("").starts_with("agent-"))
                .unwrap_or(false)
        {
            match fs::read_to_string(&path) {
                Ok(content) => {
                    saw_readable_session_file = true;
                    // 读取所有行直到找到 cwd（通常在前几行，但不确定具体位置）
                    for line in content.lines() {
                        if let Ok(json) = serde_json::from_str::<serde_json::Value>(line) {
                            if let Some(cwd) = json.get("cwd").and_then(|v| v.as_str()) {
                                return Some(cwd.to_string());
                            }
                        }
                    }
                }
                Err(e) => {
                    log::debug!("Could not read session file {:?}: {}", path, e);
                }
            }
        }
    }

    if saw_readable_session_file {
        log::warn!(
            "Session files exist but no cwd found in {:?}",
            project_dir
        );
    } else {
        log::debug!(
            "No readable session files in {:?} (empty or agent-only)",
            project_dir
        );
    }

    None
}

/// 获取项目目录最后修改时间
fn get_project_last_modified(project_dir: &Path) -> u64 {
    if !project_dir.exists() {
        return 0;
    }

    let mut max_time = 0u64;

    if let Ok(entries) = fs::read_dir(project_dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            let ext = path.extension().and_then(|e| e.to_str()).unwrap_or("");
            if (ext == "jsonl" || ext == "txt")
                && !path
                    .file_name()
                    .map(|n| n.to_str().unwrap_or("").starts_with("agent-"))
                    .unwrap_or(false)
            {
                if let Ok(meta) = fs::metadata(&path) {
                    if let Ok(modified) = meta.modified() {
                        let millis = modified
                            .duration_since(std::time::UNIX_EPOCH)
                            .map(|d| d.as_millis() as u64)
                            .unwrap_or(0);
                        max_time = max_time.max(millis);
                    }
                }
            }
        }
    }

    if max_time == 0 {
        if let Ok(meta) = fs::metadata(project_dir) {
            if let Ok(modified) = meta.modified() {
                max_time = modified
                    .duration_since(std::time::UNIX_EPOCH)
                    .map(|d| d.as_millis() as u64)
                    .unwrap_or(0);
            }
        }
    }

    max_time
}

// ==================== 公开函数 ====================

/// 首页数据（一次遍历同时返回项目和近期会话）
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct HomeData {
    pub projects: Vec<Project>,
    pub recent_sessions: Vec<SessionInfo>,
    pub has_more: bool,
}

/// 一次遍历获取首页所需全部数据，避免重复 IO
pub fn get_home_data(project_limit: usize, session_limit: usize) -> Result<HomeData> {
    let claude_dir = get_claude_dir()?;
    let projects_dir = claude_dir.join("projects");

    if !projects_dir.exists() {
        return Ok(HomeData {
            projects: Vec::new(),
            recent_sessions: Vec::new(),
            has_more: false,
        });
    }

    let mut projects = Vec::new();
    let mut all_sessions = Vec::new();

    // 隐藏项目不展示（其历史会话也一并在近期会话中过滤）
    let hidden = hidden_project_keys();

    for entry in fs::read_dir(&projects_dir)? {
        let entry = entry?;
        let path = entry.path();

        if !path.is_dir() {
            continue;
        }

        let real_path = match extract_project_path_from_jsonl(&path) {
            Some(p) => p,
            None => continue,
        };

        // 跳过原始路径已不存在的项目
        if !Path::new(&real_path).exists() {
            continue;
        }

        // 隐藏项目跳过（会话收集在同一循环内，连带跳过）
        if is_hidden_project(&hidden, &real_path) {
            continue;
        }

        let last_modified = get_project_last_modified(&path);

        projects.push(Project {
            path: real_path.clone(),
            name: Path::new(&real_path)
                .file_name()
                .and_then(|n| n.to_str())
                .unwrap_or(&real_path)
                .to_string(),
            last_session_id: None,
            last_cost: None,
            last_duration: Some(last_modified),
        });

        // 同一次遍历中收集每个项目的前 3 条会话
        if let Ok(sessions) = get_sessions(&real_path, 3, 0) {
            all_sessions.extend(sessions);
        }
    }

    // 按最后修改时间排序
    projects.sort_by(|a, b| {
        b.last_duration
            .unwrap_or(0)
            .cmp(&a.last_duration.unwrap_or(0))
    });

    let has_more = projects.len() > project_limit;
    let paginated_projects: Vec<Project> = projects.into_iter().take(project_limit).collect();

    all_sessions.sort_by(|a, b| b.last_active_at.cmp(&a.last_active_at));
    all_sessions.truncate(session_limit);

    Ok(HomeData {
        projects: paginated_projects,
        recent_sessions: all_sessions,
        has_more,
    })
}

/// 获取项目列表（支持分页）
pub fn get_projects(limit: Option<usize>, offset: Option<usize>) -> Result<Vec<Project>> {
    let claude_dir = get_claude_dir()?;
    let projects_dir = claude_dir.join("projects");

    if !projects_dir.exists() {
        return Ok(Vec::new());
    }

    let mut projects = Vec::new();

    // 隐藏项目不展示（get_all_recent_sessions 等也经此过滤）
    let hidden = hidden_project_keys();

    for entry in fs::read_dir(&projects_dir)? {
        let entry = entry?;
        let path = entry.path();

        if !path.is_dir() {
            continue;
        }

        // 提取失败（空残留目录/无 cwd）直接跳过，日志已在函数内分级记录
        let real_path = match extract_project_path_from_jsonl(&path) {
            Some(p) => p,
            None => continue,
        };

        // 跳过原始路径已不存在的项目
        if !Path::new(&real_path).exists() {
            continue;
        }

        // 隐藏项目跳过
        if is_hidden_project(&hidden, &real_path) {
            continue;
        }

        let last_modified = get_project_last_modified(&path);

        let project = Project {
            path: real_path.clone(),
            name: Path::new(&real_path)
                .file_name()
                .and_then(|n| n.to_str())
                .unwrap_or(&real_path)
                .to_string(),
            last_session_id: None,
            last_cost: None,
            last_duration: Some(last_modified),
        };

        projects.push(project);
    }

    // 按最后修改时间排序
    projects.sort_by(|a, b| {
        b.last_duration
            .unwrap_or(0)
            .cmp(&a.last_duration.unwrap_or(0))
    });

    // 分页
    let offset = offset.unwrap_or(0);
    let limit = limit.unwrap_or(projects.len());
    let start = offset.min(projects.len());
    let end = (offset + limit).min(projects.len());

    Ok(projects[start..end].to_vec())
}

/// 获取项目信息
pub fn get_project_info(path: &str) -> Result<Option<Project>> {
    let projects = get_projects(None, None)?;
    Ok(projects.iter().find(|p| p.path == path).cloned())
}

// ==================== 隐藏项目 ====================

/// 项目路径归一化（比对键）：统一分隔符、去尾斜杠、小写。
/// 隐藏记录与列表路径来自不同来源（用户隐藏时的列表路径 / 重新添加时的
/// 选择器路径），Windows 大小写与分隔符差异必须归一后比对
pub(crate) fn normalize_project_key(path: &str) -> String {
    path.replace('\\', "/").trim_end_matches('/').to_lowercase()
}

/// 隐藏项目键集合（config.hiddenProjects → 归一化）
fn hidden_project_keys() -> std::collections::HashSet<String> {
    get_app_config()
        .ok()
        .and_then(|c| c.hidden_projects)
        .unwrap_or_default()
        .iter()
        .map(|p| normalize_project_key(p))
        .collect()
}

/// 项目是否隐藏（路径归一化后比对）
pub(crate) fn is_hidden_project(keys: &std::collections::HashSet<String>, path: &str) -> bool {
    keys.contains(&normalize_project_key(path))
}

/// 隐藏项目：记录路径，首页项目列表/近期会话不再主动展示
/// （用户重新添加该项目时经 unhide_project 解除）
pub fn hide_project(path: &str) -> Result<()> {
    let hidden = get_app_config()?.hidden_projects.unwrap_or_default();
    let key = normalize_project_key(path);
    if hidden.iter().any(|p| normalize_project_key(p) == key) {
        return Ok(());
    }
    let mut next = hidden;
    next.push(path.to_string());
    update_app_config(serde_json::json!({ "hiddenProjects": next }))
}

/// 取消隐藏项目（新增项目再次打开时调用）
pub fn unhide_project(path: &str) -> Result<()> {
    let hidden = get_app_config()?.hidden_projects.unwrap_or_default();
    let key = normalize_project_key(path);
    let next: Vec<String> = hidden
        .into_iter()
        .filter(|p| normalize_project_key(p) != key)
        .collect();
    update_app_config(serde_json::json!({ "hiddenProjects": next }))
}

/// 从 JSONL 提取会话名称
pub(crate) fn extract_session_name(jsonl_path: &Path) -> String {
    if let Ok(content) = fs::read_to_string(jsonl_path) {
        let mut custom_title: Option<String> = None;
        let mut first_user_message: Option<String> = None;

        for line in content.lines() {
            if let Ok(json) = serde_json::from_str::<serde_json::Value>(line) {
                // 查找 custom-title（优先级最高）
                if json.get("type").and_then(|v| v.as_str()) == Some("custom-title") {
                    if let Some(title) = json.get("customTitle").and_then(|v| v.as_str()) {
                        custom_title = Some(title.to_string());
                    }
                }

                // 查找用户消息（只取第一条）
                if json.get("type").and_then(|v| v.as_str()) == Some("user")
                    && first_user_message.is_none()
                {
                    if let Some(msg_content) = json
                        .get("message")
                        .and_then(|m| m.get("content"))
                        .and_then(|c| c.as_str())
                    {
                        let is_meta = json
                            .get("isMeta")
                            .and_then(|v| v.as_bool())
                            .unwrap_or(false);

                        // 过滤所有系统注入消息：以 < 开头的都是系统标记
                        let is_system_inject = msg_content.trim_start().starts_with('<');

                        if !is_meta && !is_system_inject {
                            let truncated: String = msg_content.chars().take(50).collect();
                            first_user_message = if msg_content.chars().count() > 50 {
                                Some(format!("{}...", truncated))
                            } else {
                                Some(msg_content.to_string())
                            };
                        }
                    }
                }
            }
        }

        if let Some(title) = custom_title {
            return title;
        }

        if let Some(msg) = first_user_message {
            return msg;
        }
    }

    "Unnamed session".to_string()
}

/// 轻量会话条目（仅文件元数据，不读内容）
struct SessionEntry {
    session_id: String,
    path: PathBuf,
    last_active_at: u64,
}

/// 获取会话列表
pub fn get_sessions(project_path: &str, limit: usize, offset: usize) -> Result<Vec<SessionInfo>> {
    let project_dirs = get_project_dirs(project_path);

    if project_dirs.is_empty() {
        return Ok(Vec::new());
    }

    // 第一遍：只扫文件名和元数据（不读 JSONL 内容）
    let mut entries: Vec<SessionEntry> = Vec::new();

    for project_dir in &project_dirs {
        if !project_dir.exists() {
            continue;
        }

        for entry in fs::read_dir(project_dir)? {
            let entry = entry?;
            let path = entry.path();

            let ext = path.extension().and_then(|e| e.to_str()).unwrap_or("");
            if (ext == "jsonl" || ext == "txt")
                && !path
                    .file_name()
                    .map(|n| n.to_str().unwrap_or("").starts_with("agent-"))
                    .unwrap_or(false)
            {
                let session_id = path
                    .file_stem()
                    .and_then(|n| n.to_str())
                    .unwrap_or("")
                    .to_string();

                let last_active_at = fs::metadata(&path)
                    .and_then(|m| m.modified())
                    .map(|t| {
                        t.duration_since(std::time::UNIX_EPOCH)
                            .map(|d| d.as_millis() as u64)
                            .unwrap_or(0)
                    })
                    .unwrap_or(0);

                entries.push(SessionEntry {
                    session_id,
                    path,
                    last_active_at,
                });
            }
        }
    }

    // 按最后活跃时间排序
    entries.sort_by(|a, b| b.last_active_at.cmp(&a.last_active_at));

    // 分页
    let start = offset.min(entries.len());
    let end = (offset + limit).min(entries.len());

    // 第二遍：只读分页后的少量文件提取名称
    let sessions: Vec<SessionInfo> = entries[start..end]
        .iter()
        .map(|e| SessionInfo {
            session_id: e.session_id.clone(),
            name: extract_session_name(&e.path),
            project_path: project_path.to_string(),
            last_active_at: e.last_active_at,
        })
        .collect();

    Ok(sessions)
}

/// 获取所有项目的近期会话（跨项目，按 lastActiveAt 降序排列）
pub fn get_all_recent_sessions(limit: usize) -> Result<Vec<SessionInfo>> {
    let projects = get_projects(None, None)?;
    let mut all_sessions = Vec::new();

    // 每个项目只取前 3 条，减少 IO 开销
    let per_project = 3.min(limit);

    for project in &projects {
        if let Ok(sessions) = get_sessions(&project.path, per_project, 0) {
            all_sessions.extend(sessions);
        }
    }

    // 按 lastActiveAt 降序排列
    all_sessions.sort_by(|a, b| b.last_active_at.cmp(&a.last_active_at));

    // 截断到 limit 条
    all_sessions.truncate(limit);

    Ok(all_sessions)
}

/// 获取会话总数
pub fn get_session_count(project_path: &str) -> Result<usize> {
    let project_dirs = get_project_dirs(project_path);

    if project_dirs.is_empty() {
        return Ok(0);
    }

    let mut count = 0;
    for project_dir in &project_dirs {
        if !project_dir.exists() {
            continue;
        }

        count += fs::read_dir(project_dir)?
            .filter(|e| {
                e.as_ref()
                    .ok()
                    .map(|entry| {
                        let path = entry.path();
                        let ext = path.extension().and_then(|e| e.to_str()).unwrap_or("");
                        (ext == "jsonl" || ext == "txt")
                            && !path
                                .file_name()
                                .map(|n| n.to_str().unwrap_or("").starts_with("agent-"))
                                .unwrap_or(false)
                    })
                    .unwrap_or(false)
            })
            .count();
    }

    Ok(count)
}

/// 获取会话详情
pub fn get_session_details(project_path: &str, session_id: &str) -> Result<Option<SessionDetails>> {
    let project_dirs = get_project_dirs(project_path);

    let session_file = project_dirs
        .iter()
        .flat_map(|dir| {
            let jsonl = dir.join(format!("{}.jsonl", session_id));
            let txt = dir.join(format!("{}.txt", session_id));
            vec![jsonl, txt]
        })
        .find(|f| f.exists());

    let session_file = match session_file {
        Some(f) => f,
        None => return Ok(None),
    };

    let content = fs::read_to_string(&session_file)?;
    let mut message_count = 0;
    let mut total_tokens = 0u64;
    let mut total_cost = 0.0;
    let mut created_at: Option<u64> = None;
    let mut name = "Unnamed session".to_string();

    for line in content.lines() {
        if let Ok(json) = serde_json::from_str::<serde_json::Value>(line) {
            // 创建时间
            if created_at.is_none() {
                if let Some(ts) = json.get("timestamp").and_then(|v| v.as_str()) {
                    created_at = Some(parse_timestamp(ts));
                }
            }

            // 消息计数
            let msg_type = json.get("type").and_then(|v| v.as_str());
            if msg_type == Some("user") || msg_type == Some("assistant") {
                message_count += 1;
            }

            // Token 统计
            if let Some(usage) = json.get("usage") {
                if let Some(input) = usage.get("input_tokens").and_then(|v| v.as_u64()) {
                    total_tokens += input;
                }
                if let Some(output) = usage.get("output_tokens").and_then(|v| v.as_u64()) {
                    total_tokens += output;
                }
            }

            // 成本
            if let Some(cost) = json.get("costUSD").and_then(|v| v.as_f64()) {
                total_cost += cost;
            }

            // 名称
            if msg_type == Some("custom-title") {
                if let Some(title) = json.get("customTitle").and_then(|v| v.as_str()) {
                    name = title.to_string();
                }
            }
        }
    }

    if name == "Unnamed session" {
        name = extract_session_name(&session_file);
    }

    let last_active_at = fs::metadata(&session_file)
        .and_then(|m| m.modified())
        .map(|t| {
            t.duration_since(std::time::UNIX_EPOCH)
                .map(|d| d.as_millis() as u64)
                .unwrap_or(0)
        })
        .unwrap_or(0);

    Ok(Some(SessionDetails {
        session_id: session_id.to_string(),
        name,
        message_count,
        total_tokens: if total_tokens > 0 {
            Some(total_tokens)
        } else {
            None
        },
        total_cost: if total_cost > 0.0 {
            Some(total_cost)
        } else {
            None
        },
        created_at,
        last_active_at,
    }))
}

/// 会话搜索结果（含消息片段）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionSearchResult {
    #[serde(rename = "sessionId")]
    pub session_id: String,
    pub name: String,
    #[serde(rename = "projectPath")]
    pub project_path: String,
    #[serde(rename = "lastActiveAt")]
    pub last_active_at: u64,
    pub snippet: String,
}

/// 搜索会话消息内容
pub fn search_session_messages(
    project_path: &str,
    query: &str,
    limit: usize,
) -> Result<Vec<SessionSearchResult>> {
    let project_dirs = get_project_dirs(project_path);
    if project_dirs.is_empty() {
        return Ok(Vec::new());
    }

    Ok(search_session_messages_in_dirs(&project_dirs, project_path, query, limit))
}

/// 在指定项目目录下搜索会话消息（核心逻辑，可测试）
/// 从每个文件最新行往回扫描全部行，命中第一个匹配即停止，snippet 取最新匹配
pub(crate) fn search_session_messages_in_dirs(
    project_dirs: &[PathBuf],
    project_path: &str,
    query: &str,
    limit: usize,
) -> Vec<SessionSearchResult> {
    let query_lower = query.to_lowercase();
    let mut results = Vec::new();

    for project_dir in project_dirs {
        if !project_dir.exists() {
            continue;
        }

        let entries = match fs::read_dir(project_dir) {
            Ok(e) => e,
            Err(_) => continue,
        };

        for entry in entries {
            let entry = match entry {
                Ok(e) => e,
                Err(_) => continue,
            };
            let path = entry.path();

            let ext = path.extension().and_then(|e| e.to_str()).unwrap_or("");
            let is_agent = path
                .file_name()
                .map(|n| n.to_str().unwrap_or("").starts_with("agent-"))
                .unwrap_or(false);
            if (ext != "jsonl" && ext != "txt") || is_agent {
                continue;
            }

            let session_id = path
                .file_stem()
                .and_then(|n| n.to_str())
                .unwrap_or("")
                .to_string();

            let content = match fs::read_to_string(&path) {
                Ok(c) => c,
                Err(_) => continue,
            };

            let name = extract_session_name(&path);
            let lines: Vec<&str> = content.lines().collect();
            let mut matched_snippet: Option<String> = None;

            // 从末尾开始扫描全部行（最新消息优先）
            for line in lines.iter().rev() {
                if let Ok(json) = serde_json::from_str::<serde_json::Value>(line) {
                    let msg_type = json.get("type").and_then(|v| v.as_str());

                    if msg_type == Some("user") || msg_type == Some("assistant") {
                        if let Some(msg_content) = json
                            .get("message")
                            .and_then(|m| m.get("content"))
                            .and_then(|c| c.as_str())
                        {
                            if msg_content.to_lowercase().contains(&query_lower) {
                                let chars: Vec<char> = msg_content.chars().collect();
                                let lower_content: String =
                                    chars.iter().collect::<String>().to_lowercase();
                                let match_pos =
                                    lower_content.find(&query_lower).unwrap_or(0);
                                let char_match_pos =
                                    lower_content[..match_pos].chars().count();
                                let start = char_match_pos.saturating_sub(30);
                                let end = (char_match_pos + query.chars().count() + 70)
                                    .min(chars.len());
                                let snippet_raw: String =
                                    chars[start..end].iter().collect();
                                matched_snippet = Some(if start > 0 || end < chars.len() {
                                    format!("...{}...", snippet_raw)
                                } else {
                                    snippet_raw
                                });
                                break;
                            }
                        }
                    }
                }
            }

            if let Some(snippet) = matched_snippet {
                let last_active_at = fs::metadata(&path)
                    .and_then(|m| m.modified())
                    .map(|t| {
                        t.duration_since(std::time::UNIX_EPOCH)
                            .map(|d| d.as_millis() as u64)
                            .unwrap_or(0)
                    })
                    .unwrap_or(0);

                results.push(SessionSearchResult {
                    session_id,
                    name,
                    project_path: project_path.to_string(),
                    last_active_at,
                    snippet,
                });
            }
        }
    }

    results.sort_by(|a, b| b.last_active_at.cmp(&a.last_active_at));
    results.truncate(limit);
    results
}

/// 解析 ISO 时间戳
pub(crate) fn parse_timestamp(ts: &str) -> u64 {
    // 简单解析 ISO 格式时间戳
    // 2024-01-01T00:00:00Z -> milliseconds
    if let Ok(dt) = chrono::DateTime::parse_from_rfc3339(ts) {
        dt.timestamp_millis() as u64
    } else {
        0
    }
}

/// 获取应用配置
pub fn get_app_config() -> Result<AppConfig> {
    let config_path = get_gui_config_path()?;

    if !config_path.exists() {
        return Ok(AppConfig {
            default_continue: Some(true),
            default_skip_permissions: Some(false),
            default_custom_args: Some("".to_string()),
            theme: Some("light".to_string()),
            font_size: Some(12),
            hidden_projects: Some(Vec::new()),
            ..Default::default()
        });
    }

    let content = fs::read_to_string(&config_path)?;
    let config: AppConfig =
        serde_json::from_str(&content).context("Failed to parse config.json")?;

    Ok(config)
}

/// config.json 写入互斥：多线程（多实例共享文件）读-合并-写竞争防护
static CONFIG_WRITE_LOCK: std::sync::Mutex<()> = std::sync::Mutex::new(());

/// 跨进程配置锁：Broker 与多个 GUI 共享 config.json，进程内互斥不够。
/// OS advisory lock 在进程崩溃时自动释放；最多等待 500ms，超时继续写
/// （宁可极小概率丢一个字段，也不能让 UI 命令卡死）。
struct ConfigFileLock(#[allow(dead_code)] crate::file_lock::FileLock);

fn acquire_config_file_lock(dir: &std::path::Path) -> Option<ConfigFileLock> {
    let path = dir.join("config.lock");
    for _ in 0..20 {
        if let Ok(lock) = crate::file_lock::FileLock::try_exclusive(&path) {
            return Some(ConfigFileLock(lock));
        }
        std::thread::sleep(std::time::Duration::from_millis(25));
    }
    log::warn!("[config] cross-process lock timeout; proceeding");
    None
}

/// 更新应用配置（读-合并-写全程持锁；临时文件 + rename 原子落盘——
/// 直接 fs::write 截断写会让并发读者拿到半截 JSON，进而触发身份重建
/// 等灾难性默认值）
pub fn update_app_config(updates: serde_json::Value) -> Result<()> {
    let _guard = CONFIG_WRITE_LOCK.lock();

    let config_path = get_gui_config_path()?;
    let config_dir = config_path
        .parent()
        .context("Could not get parent directory of config path")?;

    if !config_dir.exists() {
        fs::create_dir_all(config_dir)?;
    }
    // 跨进程互斥必须覆盖 read-merge-write 全过程（Broker 与 GUI 并发写）
    let _file_lock = acquire_config_file_lock(config_dir);

    let existing = get_app_config()?;
    let existing_json = serde_json::to_value(existing)?;

    let merged = merge_json_values(existing_json, updates);

    let content = serde_json::to_string_pretty(&merged)?;
    let tmp_path = config_path.with_extension("json.tmp");
    fs::write(&tmp_path, &content)?;
    // Windows: rename 到已存在目标偶发失败（目标被读方短暂打开等）；先重试
    // 一次，仍失败才走 remove+rename。remove 与 rename 之间被杀会丢整个文件
    // （曾因前端写风暴高频触发此窗口，remote 配置整体丢失），remove 前先把
    // 现文件挪到 .bak 保底可恢复
    if fs::rename(&tmp_path, &config_path).is_err() {
        std::thread::sleep(std::time::Duration::from_millis(20));
        if fs::rename(&tmp_path, &config_path).is_err() {
            let _ = fs::rename(&config_path, config_path.with_extension("json.bak"));
            let _ = fs::remove_file(&config_path);
            fs::rename(&tmp_path, &config_path)?;
        }
    }

    Ok(())
}

/// 合并 JSON 值
pub(crate) fn merge_json_values(base: serde_json::Value, updates: serde_json::Value) -> serde_json::Value {
    match (base, updates) {
        (serde_json::Value::Object(mut base_map), serde_json::Value::Object(updates_map)) => {
            for (key, value) in updates_map {
                if value.is_null() {
                    base_map.remove(&key);
                } else {
                    base_map.insert(key, value);
                }
            }
            serde_json::Value::Object(base_map)
        }
        (_, updates) => updates,
    }
}

/// 获取默认 Claude 选项
pub fn get_default_claude_options() -> Result<DefaultClaudeOptions> {
    let config = get_app_config()?;
    Ok(DefaultClaudeOptions {
        continue_opt: config.default_continue,
        resume: None,
        skip_permissions: config.default_skip_permissions,
        custom_args: config.default_custom_args,
    })
}

/// 保存默认 Claude 选项
pub fn save_default_claude_options(options: DefaultClaudeOptions) -> Result<()> {
    let updates = serde_json::json!({
        "defaultContinue": options.continue_opt,
        "defaultSkipPermissions": options.skip_permissions,
        "defaultCustomArgs": options.custom_args,
    });
    update_app_config(updates)?;
    Ok(())
}

/// 保存最近打开项目
pub fn save_last_project(path: &str) -> Result<()> {
    let updates = serde_json::json!({
        "lastOpenedProject": path,
    });
    update_app_config(updates)?;
    Ok(())
}

/// 获取项目配置
pub fn get_project_config(project_path: &str) -> Result<ProjectConfig> {
    let mut config = ProjectConfig::default();

    // 读取基本配置
    config.basic = read_basic_config(project_path)?;

    // 读取 MCP 配置
    config.mcp = read_mcp_config(project_path)?;

    // 读取 Skills 配置
    config.skills = read_skills_config(project_path)?;

    // 读取 Agents 配置
    config.agents = read_agents_config(project_path)?;

    // 读取 Hooks 配置
    config.hooks = read_hooks_config(project_path)?;

    Ok(config)
}

// ==================== 配置读取辅助函数 ====================

fn read_basic_config(project_path: &str) -> Result<Vec<BasicConfigItem>> {
    let mut result = Vec::new();

    // 用户级 <claude config>/settings.json（尊重 CLAUDE_CONFIG_DIR）
    let user_settings = claude_config_dir()?.join("settings.json");
    if user_settings.exists() {
        if let Ok(content) = fs::read_to_string(&user_settings) {
            if let Ok(settings) = serde_json::from_str::<serde_json::Value>(&content) {
                result.push(BasicConfigItem {
                    model: settings
                        .get("model")
                        .and_then(|v| v.as_str())
                        .map(|s| s.to_string()),
                    theme: settings
                        .get("theme")
                        .and_then(|v| v.as_str())
                        .map(|s| s.to_string()),
                    editor_mode: settings
                        .get("editorMode")
                        .and_then(|v| v.as_str())
                        .map(|s| s.to_string()),
                    auto_connect_ide: settings.get("autoConnectIde").and_then(|v| v.as_bool()),
                    permissions: parse_permissions(&settings),
                    env: settings.get("env").cloned(),
                    source: ConfigSource {
                        source_type: "user".to_string(),
                        label: "User".to_string(),
                        path: Some(user_settings.to_string_lossy().to_string()),
                    },
                });
            }
        }
    }

    // 项目级 .claude/settings.json
    let project_settings = PathBuf::from(project_path)
        .join(".claude")
        .join("settings.json");
    if project_settings.exists() {
        if let Ok(content) = fs::read_to_string(&project_settings) {
            if let Ok(settings) = serde_json::from_str::<serde_json::Value>(&content) {
                result.push(BasicConfigItem {
                    model: settings
                        .get("model")
                        .and_then(|v| v.as_str())
                        .map(|s| s.to_string()),
                    theme: settings
                        .get("theme")
                        .and_then(|v| v.as_str())
                        .map(|s| s.to_string()),
                    editor_mode: settings
                        .get("editorMode")
                        .and_then(|v| v.as_str())
                        .map(|s| s.to_string()),
                    auto_connect_ide: settings.get("autoConnectIde").and_then(|v| v.as_bool()),
                    permissions: parse_permissions(&settings),
                    env: settings.get("env").cloned(),
                    source: ConfigSource {
                        source_type: "project".to_string(),
                        label: "Project".to_string(),
                        path: Some(project_settings.to_string_lossy().to_string()),
                    },
                });
            }
        }
    }

    // 本地级 .claude/settings.local.json
    let local_settings = PathBuf::from(project_path)
        .join(".claude")
        .join("settings.local.json");
    if local_settings.exists() {
        if let Ok(content) = fs::read_to_string(&local_settings) {
            if let Ok(settings) = serde_json::from_str::<serde_json::Value>(&content) {
                result.push(BasicConfigItem {
                    model: None,
                    theme: None,
                    editor_mode: None,
                    auto_connect_ide: None,
                    permissions: parse_permissions(&settings),
                    env: settings.get("env").cloned(),
                    source: ConfigSource {
                        source_type: "local".to_string(),
                        label: "Local".to_string(),
                        path: Some(local_settings.to_string_lossy().to_string()),
                    },
                });
            }
        }
    }

    Ok(result)
}

fn parse_permissions(settings: &serde_json::Value) -> Option<PermissionsConfig> {
    settings.get("permissions").map(|p| PermissionsConfig {
        allow: p.get("allow").and_then(|v| v.as_array()).map(|a| {
            a.iter()
                .filter_map(|v| v.as_str())
                .map(|s| s.to_string())
                .collect()
        }),
        deny: p.get("deny").and_then(|v| v.as_array()).map(|a| {
            a.iter()
                .filter_map(|v| v.as_str())
                .map(|s| s.to_string())
                .collect()
        }),
    })
}

fn read_mcp_config(project_path: &str) -> Result<Vec<McpServerItem>> {
    let mut result = Vec::new();

    // 项目级 .mcp.json
    let project_mcp = PathBuf::from(project_path).join(".mcp.json");
    if project_mcp.exists() {
        if let Ok(content) = fs::read_to_string(&project_mcp) {
            if let Ok(mcp) = serde_json::from_str::<serde_json::Value>(&content) {
                if let Some(servers) = mcp.get("mcpServers").and_then(|v| v.as_object()) {
                    for (name, config) in servers {
                        result.push(parse_mcp_server(name, config, "project", &project_mcp));
                    }
                }
            }
        }
    }

    // 用户级 ~/.claude.json 的 mcpServers
    let home = dirs::home_dir().context("Home directory not found")?;
    let user_config_path = home.join(".claude.json");
    if user_config_path.exists() {
        if let Ok(content) = fs::read_to_string(&user_config_path) {
            if let Ok(config) = serde_json::from_str::<serde_json::Value>(&content) {
                if let Some(servers) = config.get("mcpServers").and_then(|v| v.as_object()) {
                    for (name, server_config) in servers {
                        result.push(parse_mcp_server(
                            name,
                            server_config,
                            "user",
                            &user_config_path,
                        ));
                    }
                }
            }
        }
    }

    Ok(result)
}

fn parse_mcp_server(
    name: &str,
    config: &serde_json::Value,
    source_type: &str,
    path: &Path,
) -> McpServerItem {
    McpServerItem {
        name: name.to_string(),
        server_type: config
            .get("type")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string()),
        command: config
            .get("command")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string()),
        args: config.get("args").and_then(|v| v.as_array()).map(|a| {
            a.iter()
                .filter_map(|v| v.as_str())
                .map(|s| s.to_string())
                .collect()
        }),
        env: config.get("env").cloned(),
        url: config
            .get("url")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string()),
        source: ConfigSource {
            source_type: source_type.to_string(),
            label: source_type.to_string(),
            path: Some(path.to_string_lossy().to_string()),
        },
    }
}

fn read_skills_config(project_path: &str) -> Result<Vec<SkillItem>> {
    let mut result = Vec::new();

    // 项目级
    let project_skills_dir = PathBuf::from(project_path).join(".claude").join("skills");
    if project_skills_dir.exists() {
        for entry in fs::read_dir(&project_skills_dir)? {
            let entry = entry?;
            if entry.path().is_dir() {
                let skill_file = entry.path().join("SKILL.md");
                if skill_file.exists() {
                    let name = entry.file_name().to_string_lossy().to_string();
                    let description = extract_md_description(&skill_file);
                    result.push(SkillItem {
                        name,
                        description,
                        path: skill_file.to_string_lossy().to_string(),
                        source: ConfigSource {
                            source_type: "project".to_string(),
                            label: "Project".to_string(),
                            path: Some(skill_file.to_string_lossy().to_string()),
                        },
                    });
                }
            }
        }
    }

    // 用户级 <claude config>/skills/<name>/SKILL.md（尊重 CLAUDE_CONFIG_DIR）
    let user_skills_dir = claude_config_dir()?.join("skills");
    if user_skills_dir.exists() {
        for entry in fs::read_dir(&user_skills_dir)? {
            let entry = entry?;
            if entry.path().is_dir() {
                let skill_file = entry.path().join("SKILL.md");
                if skill_file.exists() {
                    let name = entry.file_name().to_string_lossy().to_string();
                    let description = extract_md_description(&skill_file);
                    result.push(SkillItem {
                        name,
                        description,
                        path: skill_file.to_string_lossy().to_string(),
                        source: ConfigSource {
                            source_type: "user".to_string(),
                            label: "User".to_string(),
                            path: Some(skill_file.to_string_lossy().to_string()),
                        },
                    });
                }
            }
        }
    }

    Ok(result)
}

fn read_agents_config(project_path: &str) -> Result<Vec<AgentItem>> {
    let mut result = Vec::new();

    // 项目级
    let project_agents_dir = PathBuf::from(project_path).join(".claude").join("agents");
    if project_agents_dir.exists() {
        for entry in fs::read_dir(&project_agents_dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.extension().map(|e| e == "md").unwrap_or(false) {
                let (name, description, _) = parse_agent_frontmatter(&path)
                    .unwrap_or_else(|| {
                        let fallback = path.file_stem()
                            .map(|n| n.to_string_lossy().to_string())
                            .unwrap_or_default();
                        (fallback, extract_md_description(&path), None)
                    });
                result.push(AgentItem {
                    name,
                    description,
                    path: path.to_string_lossy().to_string(),
                    source: ConfigSource {
                        source_type: "project".to_string(),
                        label: "Project".to_string(),
                        path: Some(path.to_string_lossy().to_string()),
                    },
                });
            }
        }
    }

    // 用户级 <claude config>/agents/*.md（尊重 CLAUDE_CONFIG_DIR）
    let user_agents_dir = claude_config_dir()?.join("agents");
    if user_agents_dir.exists() {
        for entry in fs::read_dir(&user_agents_dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.extension().map(|e| e == "md").unwrap_or(false) {
                let (name, description, _) = parse_agent_frontmatter(&path)
                    .unwrap_or_else(|| {
                        let fallback = path.file_stem()
                            .map(|n| n.to_string_lossy().to_string())
                            .unwrap_or_default();
                        (fallback, extract_md_description(&path), None)
                    });
                result.push(AgentItem {
                    name,
                    description,
                    path: path.to_string_lossy().to_string(),
                    source: ConfigSource {
                        source_type: "user".to_string(),
                        label: "User".to_string(),
                        path: Some(path.to_string_lossy().to_string()),
                    },
                });
            }
        }
    }

    Ok(result)
}

/// 从文件系统读取 user/project agents，返回 AgentInfo（与 CLI 源码 loadMarkdownFilesForSubdir 一致）
fn read_agents_from_filesystem(project_path: &str) -> Result<Vec<AgentInfo>> {
    let mut agents = Vec::new();

    // 项目级
    let project_agents_dir = PathBuf::from(project_path).join(".claude").join("agents");
    if project_agents_dir.exists() {
        if let Ok(entries) = fs::read_dir(&project_agents_dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.extension().map(|e| e == "md").unwrap_or(false) {
                    let (name, description, model) = parse_agent_frontmatter(&path)
                        .unwrap_or_else(|| {
                            let fallback = path.file_stem()
                                .map(|n| n.to_string_lossy().to_string())
                                .unwrap_or_default();
                            (fallback, extract_md_description(&path), None)
                        });
                    agents.push(AgentInfo {
                        name: name.clone(),
                        display_name: name.clone(),
                        description,
                        source_type: "project".to_string(),
                        source_label: "Project".to_string(),
                        model,
                        invoke_format: format!("@\"{} (agent)\"", name),
                        enabled: true,
                    });
                }
            }
        }
    }

    // 用户级 <claude config>/agents/*.md（尊重 CLAUDE_CONFIG_DIR）
    let user_agents_dir = claude_config_dir()?.join("agents");
    if user_agents_dir.exists() {
        if let Ok(entries) = fs::read_dir(&user_agents_dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.extension().map(|e| e == "md").unwrap_or(false) {
                    let (name, description, model) = parse_agent_frontmatter(&path)
                        .unwrap_or_else(|| {
                            let fallback = path.file_stem()
                                .map(|n| n.to_string_lossy().to_string())
                                .unwrap_or_default();
                            (fallback, extract_md_description(&path), None)
                        });
                    agents.push(AgentInfo {
                        name: name.clone(),
                        display_name: name.clone(),
                        description,
                        source_type: "user".to_string(),
                        source_label: "User".to_string(),
                        model,
                        invoke_format: format!("@\"{} (agent)\"", name),
                        enabled: true,
                    });
                }
            }
        }
    }

    // 3. 从 disabled 备份目录读取用户级 agents（原位灰显）
    if let Ok(disabled_agents_dir) = get_disabled_subdir("agents") {
        if disabled_agents_dir.exists() {
            if let Ok(entries) = fs::read_dir(&disabled_agents_dir) {
                for entry in entries.flatten() {
                    let path = entry.path();
                    if path.extension().map(|e| e == "md").unwrap_or(false) {
                        let name = path
                            .file_stem()
                            .map(|n| n.to_string_lossy().to_string())
                            .unwrap_or_default();
                        let (parsed_name, description, model) = parse_agent_frontmatter(&path)
                            .unwrap_or_else(|| {
                                (name.clone(), extract_md_description(&path), None)
                            });
                        agents.push(AgentInfo {
                            name: parsed_name.clone(),
                            display_name: parsed_name.clone(),
                            description,
                            source_type: "user".to_string(),
                            source_label: "User (Disabled)".to_string(),
                            model,
                            invoke_format: format!("@\"{} (agent)\"", parsed_name),
                            enabled: false,
                        });
                    }
                }
            }
        }
    }

    Ok(agents)
}

pub(crate) fn read_hooks_config(project_path: &str) -> Result<Vec<HookItem>> {
    let mut result = Vec::new();

    let settings_paths: Vec<(PathBuf, &str)> = vec![
        (claude_config_dir()?.join("settings.json"), "user"),
        (
            PathBuf::from(project_path)
                .join(".claude")
                .join("settings.json"),
            "project",
        ),
        (
            PathBuf::from(project_path)
                .join(".claude")
                .join("settings.local.json"),
            "local",
        ),
    ];

    for (path, source_type) in settings_paths {
        if path.exists() {
            if let Ok(content) = fs::read_to_string(&path) {
                if let Ok(settings) = serde_json::from_str::<serde_json::Value>(&content) {
                    if let Some(hooks) = settings.get("hooks").and_then(|v| v.as_object()) {
                        for (event, hook_list) in hooks {
                            if let Some(list) = hook_list.as_array() {
                                for hook in list {
                                    let hook_list = hook.get("hooks").and_then(|h| h.as_array());
                                    let first = crate::store::hook_first(hook);
                                    result.push(HookItem {
                                        event: event.to_string(),
                                        matcher: hook
                                            .get("matcher")
                                            .and_then(|v| v.as_str())
                                            .map(|s| s.to_string()),
                                        command: hook_summary(first),
                                        // hook 类型在首个 hook 对象上（type: command/http/…）
                                        hook_type: first
                                            .and_then(|h| h.get("type"))
                                            .and_then(|v| v.as_str())
                                            .map(|s| s.to_string()),
                                        hook_count: hook_list.map(|a| a.len()).unwrap_or(1),
                                        timeout: hook_list
                                            .and_then(|a| a.first())
                                            .and_then(|h| h.get("timeout"))
                                            .and_then(|v| v.as_u64()),
                                        source: ConfigSource {
                                            source_type: source_type.to_string(),
                                            label: source_type.to_string(),
                                            path: Some(path.to_string_lossy().to_string()),
                                        },
                                        raw: Some(hook.clone()),
                                    });
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    Ok(result)
}

/// 从 Markdown 提取描述
/// 优先从 YAML frontmatter 的 description 字段读取，其次从正文提取第一行非空非标题内容
pub(crate) fn extract_md_description(path: &Path) -> Option<String> {
    if let Ok(content) = fs::read_to_string(path) {
        let lines = content.lines().collect::<Vec<_>>();

        // 检查是否有 YAML frontmatter
        if lines.first().map(|l| l.trim() == "---") == Some(true) {
            // 在 frontmatter 中查找 description 字段（支持 `>-` / `|` 块标量）
            let mut i = 1;
            while i < lines.len() {
                if lines[i].trim() == "---" {
                    break;
                }
                if let Some(desc) = lines[i].strip_prefix("description:") {
                    let desc = desc.trim();
                    if is_block_scalar_indicator(desc) {
                        let (block, next) = parse_block_scalar(&lines, i + 1, desc);
                        if !block.is_empty() {
                            return Some(block);
                        }
                        i = next;
                        continue;
                    }
                    if !desc.is_empty() {
                        return Some(unquote_yaml(desc));
                    }
                }
                i += 1;
            }
        }

        // 后备：跳过 frontmatter，从正文提取第一行非空非标题
        let mut start = 0;
        if lines.first().map(|l| l.trim() == "---") == Some(true) {
            for (i, line) in lines.iter().skip(1).enumerate() {
                if line.trim() == "---" {
                    start = i + 2;
                    break;
                }
            }
        }

        for line in lines.iter().skip(start) {
            let trimmed = line.trim();
            if !trimmed.is_empty() && !trimmed.starts_with('#') && !trimmed.starts_with("---") {
                return Some(trimmed.to_string());
            }
        }
    }

    Some("No description".to_string())
}

// ==================== Agent 和 MCP 动态获取 ====================

/// Skill 信息（用于面板显示）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkillInfo {
    /// Skill 名称（如 "deploy" 或 "paper-tool:paper-search"）
    pub name: String,
    /// 显示名称（去除前缀）
    #[serde(rename = "displayName")]
    pub display_name: String,
    /// 描述
    pub description: Option<String>,
    /// 来源类型：project、user、plugin
    #[serde(rename = "sourceType")]
    pub source_type: String,
    /// 来源标签
    #[serde(rename = "sourceLabel")]
    pub source_label: String,
    /// 调用格式（如 "/deploy" 或 "/paper-tool:paper-search"）
    #[serde(rename = "invokeFormat")]
    pub invoke_format: String,
    /// 是否启用（仅 user 源可能为 false；project/plugin 始终 true）
    #[serde(rename = "enabled", default = "default_true")]
    pub enabled: bool,
}

/// Plugin 内部 Skill 信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginSkill {
    /// Skill 名称（不含 plugin 前缀）
    pub name: String,
    /// 描述
    pub description: Option<String>,
    /// 调用格式
    #[serde(rename = "invokeFormat")]
    pub invoke_format: String,
}

/// Plugin 内部 Agent 信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginAgent {
    /// Agent 名称
    pub name: String,
    /// 描述
    pub description: Option<String>,
    /// 模型（如 haiku、sonnet、inherit）
    pub model: Option<String>,
    /// 调用格式
    #[serde(rename = "invokeFormat")]
    pub invoke_format: String,
}

/// YAML 块标量指示符（`>` 折叠 / `|` 字面量，可带 `-`/`+` chomping）
pub(crate) fn is_block_scalar_indicator(v: &str) -> bool {
    matches!(v, ">" | ">-" | ">+" | "|" | "|-" | "|+")
}

/// 去掉 YAML 标量两侧的配对引号（不做转义解码，仅去掉字面引号）
pub(crate) fn unquote_yaml(v: &str) -> String {
    let t = v.trim();
    if t.len() >= 2 {
        let b = t.as_bytes();
        if (b[0] == b'"' && b[t.len() - 1] == b'"') || (b[0] == b'\'' && b[t.len() - 1] == b'\'') {
            return t[1..t.len() - 1].to_string();
        }
    }
    t.to_string()
}

/// 读取块标量体：从 start 行起收集所有缩进行（空行属于体内），到首个同级/更高层键止。
/// 折叠（`>`）相邻非空行以空格连接；字面量（`|`）保留换行。返回（值, 下一行索引）。
pub(crate) fn parse_block_scalar(lines: &[&str], start: usize, indicator: &str) -> (String, usize) {
    let mut body: Vec<&str> = Vec::new();
    let mut i = start;
    while i < lines.len() {
        let line = lines[i];
        if line.trim().is_empty() {
            body.push("");
            i += 1;
            continue;
        }
        if line.starts_with(' ') || line.starts_with('\t') {
            body.push(line);
            i += 1;
        } else {
            break;
        }
    }
    let indent = body
        .iter()
        .filter(|l| !l.trim().is_empty())
        .map(|l| l.len() - l.trim_start().len())
        .min()
        .unwrap_or(0);
    let mut out = String::new();
    if indicator.starts_with('>') {
        let mut prev_blank = false;
        for l in &body {
            let t = l.get(indent..).unwrap_or("").trim_end();
            if t.is_empty() {
                prev_blank = true;
                continue;
            }
            if !out.is_empty() && !prev_blank {
                out.push(' ');
            }
            out.push_str(t);
            prev_blank = false;
        }
    } else {
        for l in &body {
            out.push_str(l.get(indent..).unwrap_or("").trim_end());
            out.push('\n');
        }
    }
    (out.trim().to_string(), i)
}

/// 从 agent .md 文件解析 frontmatter（与 CLI 源码 frontmatterParser.ts + loadAgentsDir.ts 一致）
/// 提取 name、description、model 三个字段
fn parse_agent_frontmatter(path: &Path) -> Option<(String, Option<String>, Option<String>)> {
    let content = fs::read_to_string(path).ok()?;
    let lines: Vec<&str> = content.lines().collect();

    if lines.first().map(|l| l.trim()) != Some("---") {
        return None;
    }

    let mut end_idx = None;
    for (i, line) in lines.iter().enumerate().skip(1) {
        if line.trim() == "---" {
            end_idx = Some(i);
            break;
        }
    }
    let end_idx = end_idx?;

    let mut name: Option<String> = None;
    let mut description: Option<String> = None;
    let mut model: Option<String> = None;

    let mut i = 1;
    while i < end_idx {
        let trimmed = lines[i].trim();
        if let Some(val) = trimmed.strip_prefix("description:") {
            let val = val.trim();
            if is_block_scalar_indicator(val) {
                let (block, next) = parse_block_scalar(&lines, i + 1, val);
                if !block.is_empty() {
                    description = Some(block);
                }
                i = next;
                continue;
            }
            if !val.is_empty() {
                description = Some(unquote_yaml(val));
            }
        } else if let Some(val) = trimmed.strip_prefix("name:") {
            let val = val.trim();
            if !val.is_empty() {
                name = Some(unquote_yaml(val));
            }
        } else if let Some(val) = trimmed.strip_prefix("model:") {
            let val = val.trim();
            if !val.is_empty() {
                model = Some(unquote_yaml(val));
            }
        }
        i += 1;
    }

    // name 是必需字段（与源码 parseAgentFromMarkdown 一致）
    name.map(|n| (n, description, model))
}

/// 从 SKILL.md 文件解析 frontmatter 中的 description（与 CLI 源码 loadSkillsDir.ts 一致）
/// 优先取 frontmatter description，后备取正文第一行非空非标题
pub(crate) fn parse_skill_description(path: &Path) -> Option<String> {
    let content = fs::read_to_string(path).ok()?;
    let lines: Vec<&str> = content.lines().collect();

    if lines.first().map(|l| l.trim()) == Some("---") {
        let mut end_idx = None;
        for (i, line) in lines.iter().enumerate().skip(1) {
            if line.trim() == "---" {
                end_idx = Some(i);
                break;
            }
        }

        if let Some(end) = end_idx {
            // 从 frontmatter 提取 description（支持 `>-` / `|` 块标量多行写法）
            let mut i = 1;
            while i < end {
                let trimmed = lines[i].trim();
                if let Some(val) = trimmed.strip_prefix("description:") {
                    let val = val.trim();
                    if is_block_scalar_indicator(val) {
                        let (block, _) = parse_block_scalar(&lines, i + 1, val);
                        if !block.is_empty() {
                            return Some(block);
                        }
                    } else if !val.is_empty() {
                        return Some(unquote_yaml(val));
                    }
                }
                i += 1;
            }

            // 后备：跳过 frontmatter，从正文提取第一行非空非标题
            let start = end + 1;
            for line in lines.iter().skip(start) {
                let trimmed = line.trim();
                if !trimmed.is_empty() && !trimmed.starts_with('#') && !trimmed.starts_with("---") {
                    return Some(trimmed.to_string());
                }
            }
        }
    } else {
        // 无 frontmatter，从正文提取
        for line in lines.iter() {
            let trimmed = line.trim();
            if !trimmed.is_empty() && !trimmed.starts_with('#') && !trimmed.starts_with("---") {
                return Some(trimmed.to_string());
            }
        }
    }

    Some("No description".to_string())
}

/// Agent 信息（用于面板显示）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentInfo {
    /// Agent 名称（如 "code-reviewer" 或 "paper-tool:paper-search"）
    pub name: String,
    /// 显示名称（去除前缀）
    #[serde(rename = "displayName")]
    pub display_name: String,
    /// 描述
    pub description: Option<String>,
    /// 来源类型：builtin、plugin、user、project
    #[serde(rename = "sourceType")]
    pub source_type: String,
    /// 来源标签
    #[serde(rename = "sourceLabel")]
    pub source_label: String,
    /// 模型（如 haiku、sonnet、inherit）
    pub model: Option<String>,
    /// 调用格式（如 "@agent-code-reviewer" 或 "@agent-paper-tool:paper-search"）
    #[serde(rename = "invokeFormat")]
    pub invoke_format: String,
    /// 是否启用（仅 user 源可能为 false；其他始终 true）
    #[serde(rename = "enabled", default = "default_true")]
    pub enabled: bool,
}

/// MCP Server 信息（用于面板显示）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct McpServerInfo {
    /// Server 名称
    pub name: String,
    /// 显示名称（去除 plugin: 前缀）
    #[serde(rename = "displayName")]
    pub display_name: String,
    /// 描述
    pub description: Option<String>,
    /// 来源类型：builtin、plugin、user、project、local
    #[serde(rename = "sourceType")]
    pub source_type: String,
    /// 来源标签
    #[serde(rename = "sourceLabel")]
    pub source_label: String,
    /// 类型：stdio、http、sse
    #[serde(rename = "serverType")]
    pub server_type: Option<String>,
    /// 连接状态
    pub status: Option<String>,
    /// URL（HTTP/SSE server）
    pub url: Option<String>,
    /// 命令（stdio server 可执行文件名）
    pub command: Option<String>,
    /// 命令参数（stdio server）
    pub args: Option<Vec<String>>,
    /// 环境变量（stdio server）
    pub env: Option<HashMap<String, String>>,
    /// HTTP Headers（用于认证）
    pub headers: Option<HashMap<String, String>>,
    /// 可用的 prompts 列表
    pub prompts: Vec<McpPromptInfo>,
    /// 是否启用（仅 user 源可能为 false；其他始终 true）
    #[serde(rename = "enabled", default = "default_true")]
    pub enabled: bool,
}

/// MCP Prompt 信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct McpPromptInfo {
    /// Prompt 名称
    pub name: String,
    /// 描述
    pub description: Option<String>,
    /// 调用格式（如 "/mcp__github__list_prs"）
    #[serde(rename = "invokeFormat")]
    pub invoke_format: String,
}

/// 获取所有 Agents（包括 plugin、user、project）
/// 与 CLI 源码 loadAgentsDir.ts 一致：从 .md 文件 frontmatter 解析 name/description/model
pub fn get_all_agents(project_path: &str) -> Result<Vec<AgentInfo>> {
    let mut agents = Vec::new();

    // 1. 从 plugins 获取 plugin agents（被禁用的 plugin 跳过：其子项不在侧边栏展示）
    let plugins = get_all_plugins(project_path)?;
    for plugin in &plugins {
        if !plugin.enabled {
            continue;
        }
        if let Some(plugin_agents) = &plugin.agents {
            for agent in plugin_agents {
                agents.push(AgentInfo {
                    name: format!("{}:{}", plugin.name, agent.name),
                    display_name: agent.name.clone(),
                    description: agent.description.clone(),
                    source_type: "plugin".to_string(),
                    source_label: format!("Plugin · {}", plugin.name),
                    model: agent.model.clone(),
                    invoke_format: agent.invoke_format.clone(),
                    enabled: true,
                });
            }
        }
    }

    // 2. 从文件系统读取 user 和 project agents（与 CLI 源码 loadMarkdownFilesForSubdir 一致）
    let user_project_agents = read_agents_from_filesystem(project_path)?;
    for agent in user_project_agents {
        agents.push(agent);
    }

    // 顺序与 enabled 解耦：开关不改变条目在面板中的位置
    sort_by_name(&mut agents, |a| a.name.as_str());
    Ok(agents)
}

/// 获取所有 Skills（包括 project、user、plugin）
/// 扫描一个 skills 目录中的可用 skill：真实目录或可解析的符号链接 + 可读 SKILL.md。
/// 返回 (目录名, description)。
pub(crate) fn scan_active_skills(dir: &Path) -> Vec<(String, Option<String>)> {
    let mut out = Vec::new();
    if !dir.exists() {
        return out;
    }
    let Ok(entries) = fs::read_dir(dir) else {
        return out;
    };
    for entry in entries.flatten() {
        let path = entry.path();
        if !is_dir_or_link(&path) {
            continue;
        }
        let skill_file = path.join("SKILL.md");
        // 断链/无 SKILL.md：CLI 同样加载不到，跳过
        if !skill_file.exists() {
            continue;
        }
        out.push((
            entry.file_name().to_string_lossy().to_string(),
            parse_skill_description(&skill_file),
        ));
    }
    out
}

/// 扫描 disabled 备份目录：目录与符号链接（**含断链**）都列出。
/// 断链是历史 bug 的产物（相对链接被搬断），必须列出才能从 GUI 恢复；
/// 其 SKILL.md 读不到，description 为 None。
pub(crate) fn scan_disabled_skills(dir: &Path) -> Vec<(String, Option<String>)> {
    let mut out = Vec::new();
    if !dir.exists() {
        return out;
    }
    let Ok(entries) = fs::read_dir(dir) else {
        return out;
    };
    for entry in entries.flatten() {
        let path = entry.path();
        if !is_dir_or_link(&path) {
            continue;
        }
        let skill_file = path.join("SKILL.md");
        let description = if skill_file.exists() {
            parse_skill_description(&skill_file)
        } else {
            None
        };
        out.push((
            entry.file_name().to_string_lossy().to_string(),
            description,
        ));
    }
    out
}

/// 稳定按名称排序：列表顺序与 `enabled` 解耦——条目在 active/disabled 之间
/// 移动后仍停在原位，配合前端"本端写回环抑制"实现开关不重排列表。
pub(crate) fn sort_by_name<T, F>(items: &mut [T], name: F)
where
    F: for<'a> Fn(&'a T) -> &'a str,
{
    items.sort_by(|a, b| name(a).cmp(name(b)));
}

/// 与 CLI 源码 loadSkillsDir.ts 一致：从 skills/name/SKILL.md frontmatter 解析 description
pub fn get_all_skills(project_path: &str) -> Result<Vec<SkillInfo>> {
    let mut skills = Vec::new();

    // 1. 从项目目录读取 skills（与源码 loadSkillsFromSkillsDir 一致：只支持 dir/SKILL.md 格式）
    let project_skills_dir = PathBuf::from(project_path).join(".claude").join("skills");
    for (name, description) in scan_active_skills(&project_skills_dir) {
        skills.push(SkillInfo {
            invoke_format: format!("/{}", name),
            display_name: name.clone(),
            name,
            description,
            source_type: "project".to_string(),
            source_label: "Project".to_string(),
            enabled: true,
        });
    }

    // 2. 从用户目录读取 skills（尊重 CLAUDE_CONFIG_DIR）
    let user_skills_dir = claude_config_dir()?.join("skills");
    let user_skills = scan_active_skills(&user_skills_dir);
    let active_names: std::collections::HashSet<String> =
        user_skills.iter().map(|(name, _)| name.clone()).collect();
    for (name, description) in user_skills {
        skills.push(SkillInfo {
            invoke_format: format!("/{}", name),
            display_name: name.clone(),
            name,
            description,
            source_type: "user".to_string(),
            source_label: "User".to_string(),
            enabled: true,
        });
    }

    // 2b. 从 disabled 备份目录读取用户级 skills（原位灰显；断链也列出以便恢复）
    if let Ok(disabled_skills_dir) = get_disabled_subdir("skills") {
        for (name, description) in scan_disabled_skills(&disabled_skills_dir) {
            // active 已有同名项：残留备份不作为独立条目展示（避免同组重复 key）
            if active_names.contains(&name) {
                continue;
            }
            skills.push(SkillInfo {
                invoke_format: format!("/{}", name),
                display_name: name.clone(),
                name,
                description,
                source_type: "user".to_string(),
                source_label: "User (Disabled)".to_string(),
                enabled: false,
            });
        }
    }

    // 3. 从 plugins 获取 skills（被禁用的 plugin 跳过：其子项不在侧边栏展示）
    let plugins = get_all_plugins(project_path)?;
    for plugin in plugins {
        if !plugin.enabled {
            continue;
        }
        if let Some(plugin_skills) = plugin.skills {
            for skill in plugin_skills {
                let full_name = skill
                    .invoke_format
                    .strip_prefix('/')
                    .unwrap_or(&skill.invoke_format);
                skills.push(SkillInfo {
                    name: full_name.to_string(),
                    display_name: skill.name.clone(),
                    description: skill.description,
                    source_type: "plugin".to_string(),
                    source_label: format!("Plugin · {}", full_name.split(':').next().unwrap_or(&skill.name)),
                    invoke_format: skill.invoke_format,
                    enabled: true,
                });
            }
        }
    }

    // 顺序与 enabled 解耦：禁用项混在用户组里按名称定位，开关不会让条目"跳位"
    sort_by_name(&mut skills, |s| s.name.as_str());
    Ok(skills)
}

/// 获取所有 MCP Servers（直接读配置文件 + plugin，不依赖 claude mcp list）
pub fn get_all_mcp_servers(project_path: &str) -> Result<Vec<McpServerInfo>> {
    let mut servers: Vec<McpServerInfo> = Vec::new();
    let mut seen_names = std::collections::HashSet::new();

    // 收集所有源（按优先级从低到高：plugin → user → project → local）
    // 1. Plugin scope: 从 plugin 列表加载 MCP servers（被禁用的 plugin 跳过：其子项不在侧边栏展示）
    if let Ok(plugins) = get_all_plugins(project_path) {
        for plugin in &plugins {
            if !plugin.enabled {
                continue;
            }
            if let Some(mcp_value) = &plugin.mcp_servers {
                // plugin .mcp.json 格式：直接是 server 映射（不需要 mcpServers 包装）
                // 也可能是带 mcpServers 包装的对象
                let mcp_obj = if let Some(wrapped) = mcp_value.get("mcpServers").and_then(|v| v.as_object()) {
                    wrapped
                } else if let Some(direct) = mcp_value.as_object() {
                    // 直接映射格式（plugin .mcp.json 常见格式）
                    // 排除看起来像配置包装的字段
                    direct
                } else {
                    continue;
                };

                for (server_name, config) in mcp_obj {
                    // plugin server 名称为 "plugin:{plugin_name}:{server_name}"
                    let full_name = format!("plugin:{}:{}", plugin.name, server_name);
                    let display_name = server_name.clone();

                    // 注入 CLAUDE_PLUGIN_ROOT 环境变量
                    let mut plugin_env = HashMap::new();
                    plugin_env.insert("CLAUDE_PLUGIN_ROOT".to_string(), plugin.install_path.clone());

                    if let Some(mut info) = parse_mcp_server_entry(&full_name, config, "plugin", Some(&plugin_env)) {
                        info.display_name = display_name;
                        if seen_names.insert(full_name.clone()) {
                            servers.push(info);
                        } else if let Some(existing) = servers.iter_mut().find(|s| s.name == full_name) {
                            *existing = info;
                        }
                    }
                }
            }
        }
    }

    // 2-4. 配置文件来源
    let sources = collect_mcp_config_sources(project_path);

    for (path, scope) in &sources {
        if let Ok(content) = fs::read_to_string(path) {
            if let Ok(json) = serde_json::from_str::<serde_json::Value>(&content) {
                let mcp_obj = json.get("mcpServers").and_then(|v| v.as_object());
                if let Some(mcp_obj) = mcp_obj {
                    for (name, config) in mcp_obj {
                        if let Some(info) = parse_mcp_server_entry(name, config, scope, None) {
                            if seen_names.insert(name.clone()) {
                                servers.push(info);
                            } else {
                                // 同名 server：高优先级覆盖低优先级
                                if let Some(existing) = servers.iter_mut().find(|s| s.name == *name) {
                                    *existing = info;
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    // 5. 从 disabled 备份目录读取用户级 MCP servers（原位灰显）
    if let Ok(disabled_mcp_dir) = get_disabled_subdir("mcp") {
        if disabled_mcp_dir.exists() {
            if let Ok(entries) = fs::read_dir(&disabled_mcp_dir) {
                for entry in entries.flatten() {
                    let path = entry.path();
                    if path.extension().map(|e| e == "json").unwrap_or(false) {
                        let name = path
                            .file_stem()
                            .map(|n| n.to_string_lossy().to_string())
                            .unwrap_or_default();
                        if let Ok(content) = fs::read_to_string(&path) {
                            if let Ok(config) = serde_json::from_str::<serde_json::Value>(&content)
                            {
                                if let Some(mut info) =
                                    parse_mcp_server_entry(&name, &config, "user", None)
                                {
                                    info.enabled = false;
                                    info.source_label = "User (Disabled)".to_string();
                                    if seen_names.insert(name.clone()) {
                                        servers.push(info);
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    // 顺序与 enabled 解耦：开关不改变条目在面板中的位置
    sort_by_name(&mut servers, |s| s.name.as_str());
    Ok(servers)
}

/// 收集 MCP 配置文件路径和对应 scope
fn collect_mcp_config_sources(project_path: &str) -> Vec<(PathBuf, String)> {
    let mut sources = Vec::new();

    // 1. User scope: ~/.claude.json
    if let Some(home) = dirs::home_dir() {
        sources.push((home.join(".claude.json"), "user".to_string()));
    }

    // 2. Project scope: 从项目目录向上查找所有 .mcp.json（子目录覆盖父目录）
    let project_mcp_files = walk_up_find_mcp_jsons(project_path);
    for path in project_mcp_files {
        sources.push((path, "project".to_string()));
    }

    // 3. Local scope: {project_path}/.claude/settings.json
    let local_settings = PathBuf::from(project_path)
        .join(".claude")
        .join("settings.json");
    sources.push((local_settings, "local".to_string()));

    sources
}

/// 从项目目录向上遍历查找所有 .mcp.json（父目录先、子目录后，后者覆盖前者）
fn walk_up_find_mcp_jsons(start_dir: &str) -> Vec<PathBuf> {
    let mut paths = Vec::new();
    let start = PathBuf::from(start_dir);

    let mut current = start.as_path();
    loop {
        let mcp_json = current.join(".mcp.json");
        if mcp_json.exists() {
            paths.push(mcp_json);
        }
        match current.parent() {
            Some(parent) if parent != current => current = parent,
            _ => break,
        }
    }

    // 反转：父目录在前，子目录在后（子目录覆盖父目录）
    paths.reverse();
    paths
}

/// 展开字符串中的 ${VAR} 和 ${VAR:-default} 环境变量
pub(crate) fn expand_env_vars(s: &str, extra_env: Option<&HashMap<String, String>>) -> String {
    let mut result = s.to_string();
    // 反复匹配 ${...} 模式
    loop {
        if let Some(start) = result.find("${") {
            if let Some(end) = result[start + 2..].find('}') {
                let expr = &result[start + 2..start + 2 + end];
                let (var_name, default) = if let Some(colon_pos) = expr.find(":-") {
                    (&expr[..colon_pos], Some(&expr[colon_pos + 2..]))
                } else {
                    (expr, None)
                };

                let value = extra_env
                    .and_then(|e| e.get(var_name))
                    .cloned()
                    .or_else(|| std::env::var(var_name).ok())
                    .or_else(|| default.map(|d| d.to_string()));

                match value {
                    Some(v) => {
                        result.replace_range(start..start + 2 + end + 1, &v);
                    }
                    None => {
                        // 变量未找到，保留原样
                        break;
                    }
                }
            } else {
                break;
            }
        } else {
            break;
        }
    }
    result
}

/// 解析单个 MCP server 配置条目
pub(crate) fn parse_mcp_server_entry(
    name: &str,
    config: &serde_json::Value,
    scope: &str,
    extra_env: Option<&HashMap<String, String>>,
) -> Option<McpServerInfo> {
    let obj = config.as_object()?;

    // 解析 command（可执行文件名），展开环境变量
    let command = obj
        .get("command")
        .and_then(|v| v.as_str())
        .map(|s| expand_env_vars(s, extra_env));

    // 解析 args（数组），展开环境变量
    let args = obj
        .get("args")
        .and_then(|v| v.as_array())
        .map(|arr| {
            arr.iter()
                .filter_map(|v| v.as_str().map(|s| expand_env_vars(s, extra_env)))
                .collect()
        });

    // 解析 env（对象），展开环境变量值
    let env = obj
        .get("env")
        .and_then(|v| v.as_object())
        .map(|obj| {
            obj.iter()
                .filter_map(|(k, v)| {
                    v.as_str().map(|s| (k.clone(), expand_env_vars(s, extra_env)))
                })
                .collect()
        });

    // 解析 url，展开环境变量
    let url = obj
        .get("url")
        .and_then(|v| v.as_str())
        .map(|s| expand_env_vars(s, extra_env));

    // 解析 headers，展开环境变量值
    let headers = obj
        .get("headers")
        .and_then(|v| v.as_object())
        .map(|obj| {
            obj.iter()
                .filter_map(|(k, v)| {
                    v.as_str().map(|s| (k.clone(), expand_env_vars(s, extra_env)))
                })
                .collect()
        });

    // 推断 server type
    let server_type = infer_server_type(config);

    let display_name = if scope == "plugin" {
        name.split(':').last().unwrap_or(name).to_string()
    } else {
        name.to_string()
    };

    Some(McpServerInfo {
        name: name.to_string(),
        display_name,
        description: None,
        source_type: scope.to_string(),
        source_label: capitalize_scope(scope),
        server_type: Some(server_type),
        status: None,
        url,
        command,
        args,
        env,
        headers,
        prompts: Vec::new(),
        enabled: true,
    })
}

/// 推断 MCP server 类型（与 Claude Code CLI 一致）
pub(crate) fn infer_server_type(config: &serde_json::Value) -> String {
    let obj = match config.as_object() {
        Some(o) => o,
        None => return "stdio".to_string(),
    };

    // 显式 type 字段
    if let Some(type_val) = obj.get("type").and_then(|v| v.as_str()) {
        return match type_val {
            "sse" => "sse".to_string(),
            "http" => "http".to_string(),
            "ws" => "ws".to_string(),
            _ => type_val.to_string(),
        };
    }

    // 有 command 字段 → stdio
    if obj.contains_key("command") {
        return "stdio".to_string();
    }

    // 有 url 字段 → 默认 http
    if obj.contains_key("url") {
        return "http".to_string();
    }

    "stdio".to_string()
}

/// 转换 scope 为显示标签
fn capitalize_scope(scope: &str) -> String {
    match scope {
        "plugin" => "Plugin".to_string(),
        "managed" => "Managed".to_string(),
        "local" => "Local".to_string(),
        "user" => "User".to_string(),
        "project" => "Project".to_string(),
        _ => scope.to_string(),
    }
}

/// 构造 claude 子进程：目标解析与 PTY 启动共用 `platform::cli_target`
/// （CLAUDE.md 约定：CLI 启动目标只在单点决定，不能在别处直接写 `claude`——
/// 用户配置了非 PATH 内的 CLI 时，子进程会找不到可执行文件）。
fn claude_command() -> std::process::Command {
    let configured = get_app_config().ok().and_then(|c| c.claude_path);
    let mut cmd = match crate::platform::cli_target(configured.as_deref()) {
        crate::platform::CliTarget::Explicit(path) => std::process::Command::new(path),
        crate::platform::CliTarget::PathLookup => {
            #[cfg(unix)]
            {
                // Unix：解析成可运行路径再 exec，避免依赖 GUI 继承的 PATH
                match crate::platform::find_executable("claude") {
                    Some(path) => std::process::Command::new(path),
                    None => std::process::Command::new("claude"),
                }
            }
            #[cfg(not(unix))]
            {
                // Windows：CreateProcess 不能直接执行 npm 的 .cmd 包装，这里依赖
                // PATH 能解析到可执行文件（npm 安装通常同时提供 claude.exe）。
                // 该行为与本次修复前一致，未在本机（macOS）验证。
                std::process::Command::new("claude")
            }
        }
    };

    // Windows 上需要设置 git-bash 环境
    if cfg!(target_os = "windows") {
        if let Ok(git_bash_path) = std::env::var("CLAUDE_CODE_GIT_BASH_PATH") {
            if Path::new(&git_bash_path).exists() {
                cmd.env("CLAUDE_CODE_GIT_BASH_PATH", git_bash_path);
            }
        } else if let Some(git_bash_path) = detect_git_bash_path() {
            cmd.env("CLAUDE_CODE_GIT_BASH_PATH", git_bash_path);
        }
    }

    // Windows 上禁止子进程创建控制台窗口
    crate::platform::configure_command(&mut cmd);
    cmd
}

/// 执行 claude 命令并获取输出（设置 git-bash 环境）
fn run_claude_command(args: &str) -> Result<String> {
    let mut cmd = claude_command();
    cmd.args(args.split_whitespace());
    let output = cmd.output().context("Failed to run claude command")?;

    // 即使失败也返回输出：调用方（如 plugins list）可能仍能从 stdout 取到信息
    Ok(String::from_utf8_lossy(&output.stdout).to_string())
}

/// 严格版的 claude 命令执行：exit code != 0 时返回 Err（含 stderr）
/// 用于 plugin enable/disable 等需要明确成败判断的场景
fn run_claude_command_strict(args: &str) -> Result<String> {
    let mut cmd = claude_command();
    cmd.args(args.split_whitespace());

    let output = cmd.output().context("Failed to run claude command")?;
    if output.status.success() {
        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr).to_string();
        let stdout = String::from_utf8_lossy(&output.stdout).to_string();
        bail!(
            "claude {} failed (exit {:?}): {}",
            args,
            output.status.code(),
            if stderr.is_empty() { stdout.trim() } else { stderr.trim() }
        );
    }
}

/// plugins list --json 短 TTL 缓存：一次 plugin 开关会级联刷新
/// plugins/skills/agents/mcp 四个 section，每个 section 都会 spawn 一个 claude
/// 子进程（`get_all_skills` 等内部都要读 plugin 列表）。2s 内复用同一份输出，
/// 把「一次点击 4 个子进程」收敛为 1 个；plugin 自身变更后主动失效。
static PLUGINS_LIST_CACHE: Mutex<Option<(std::time::Instant, String)>> = Mutex::new(None);
/// single-flight：并发刷新时只有一个线程真正 spawn CLI，其余等待后命中缓存
static PLUGINS_LIST_FETCH_LOCK: Mutex<()> = Mutex::new(());
/// 失效代次：防止"失效前开始、失效后才完成"的旧读取回填缓存
static PLUGINS_LIST_GEN: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);
const PLUGINS_LIST_TTL: std::time::Duration = std::time::Duration::from_secs(2);

fn plugins_list_json() -> Result<String> {
    // 锁中毒时降级为不串行（仍可读缓存），不 panic
    let _flight = PLUGINS_LIST_FETCH_LOCK
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner());
    let gen = PLUGINS_LIST_GEN.load(std::sync::atomic::Ordering::SeqCst);

    if let Ok(guard) = PLUGINS_LIST_CACHE.lock() {
        if let Some((at, raw)) = guard.as_ref() {
            if at.elapsed() < PLUGINS_LIST_TTL {
                return Ok(raw.clone());
            }
        }
    }
    let raw = run_claude_command("plugins list --json")?;
    // 代次校验与回填必须在同一把 CACHE 锁内完成：否则 invalidate 可能恰好
    // 卡在两者之间（清空后旧 raw 又带着新时间戳写回，最多 2s 读到旧状态）
    if let Ok(mut guard) = PLUGINS_LIST_CACHE.lock() {
        if PLUGINS_LIST_GEN.load(std::sync::atomic::Ordering::SeqCst) == gen {
            *guard = Some((std::time::Instant::now(), raw.clone()));
        }
    }
    Ok(raw)
}

/// 失效 plugins 列表缓存（plugin 安装/开关后必须调用，避免读到旧状态）
pub(crate) fn invalidate_plugins_list_cache() {
    // 持锁自增代次 + 清空：与 plugins_list_json 的"校验后回填"互斥
    if let Ok(mut guard) = PLUGINS_LIST_CACHE.lock() {
        PLUGINS_LIST_GEN.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        *guard = None;
    }
    // 市场条目里的 installed/enabled 徽标来自 plugins list，一并失效
    invalidate_plugin_market_cache();
}

/// 检测 Git Bash 路径（Windows）
fn detect_git_bash_path() -> Option<String> {
    if !cfg!(target_os = "windows") {
        return None;
    }

    let git_path = crate::platform::find_executable("git")?;
    let parent = Path::new(&git_path).parent()?;
    let bash_path = parent.join("bash.exe");

    if bash_path.exists() {
        Some(bash_path.to_string_lossy().to_string())
    } else {
        None
    }
}

/// 解析 claude agents list 输出
pub(crate) fn parse_agents_list_output(output: &str, agents: &mut Vec<AgentInfo>) {
    // 输出格式示例：
    // Plugin agents:
    //   paper-tool:paper-search · inherit
    // Built-in agents:
    //   claude-code-guide · haiku
    //   Explore · haiku
    //   general-purpose · inherit
    //   Plan · inherit
    //   statusline-setup · sonnet

    let mut current_section = "";

    for line in output.lines() {
        let trimmed = line.trim();

        if trimmed.starts_with("Plugin agents:") {
            current_section = "plugin";
            continue;
        }
        if trimmed.starts_with("Built-in agents:") {
            current_section = "builtin";
            continue;
        }
        if trimmed.starts_with("User agents:") {
            current_section = "user";
            continue;
        }
        if trimmed.starts_with("Project agents:") {
            current_section = "project";
            continue;
        }

        // 解析 agent 行：name · model
        if trimmed.starts_with("·") || trimmed.is_empty() || !trimmed.contains("·") {
            continue;
        }

        let parts: Vec<&str> = trimmed.splitn(2, '·').collect();
        if parts.len() == 2 {
            let name = parts[0].trim();
            let model = parts[1].trim();

            // 调用格式：@"name (agent)" 或 @"plugin:agent (agent)"
            let invoke_format = format!("@\"{} (agent)\"", name);

            // 显示名称：去除前缀
            let display_name = if name.contains(':') {
                name.split(':').last().unwrap_or(name)
            } else {
                name
            };

            let source_label = if current_section == "plugin" {
                "Plugin"
            } else if current_section == "builtin" {
                "Built-in"
            } else {
                current_section
            };

            agents.push(AgentInfo {
                name: name.to_string(),
                display_name: display_name.to_string(),
                description: None,
                source_type: current_section.to_string(),
                source_label: source_label.to_string(),
                model: Some(model.to_string()),
                invoke_format,
                enabled: true,
            });
        }
    }
}

// ==================== Plugin 信息获取 ====================

/// 插件提供的单条 hook（只读展示：事件 + matcher + 类型 + 摘要 + 超时）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginHookEntry {
    pub event: String,
    #[serde(rename = "matcher", skip_serializing_if = "Option::is_none")]
    pub matcher: Option<String>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub hook_type: Option<String>,
    /// 摘要：command → url → prompt 回退链
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout: Option<serde_json::Value>,
}

/// Plugin 信息（用于面板显示）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginInfo {
    /// Plugin ID（如 "paper-tool@orczh"）
    pub id: String,
    /// 显示名称（去除 @publisher 后缀）
    pub name: String,
    /// 版本
    pub version: String,
    /// 作用域：user、project
    pub scope: String,
    /// 是否启用
    pub enabled: bool,
    /// 安装路径（修正后的有效路径）
    #[serde(rename = "installPath")]
    pub install_path: String,
    /// 安装时间
    #[serde(rename = "installedAt")]
    pub installed_at: Option<String>,
    /// 最后更新时间
    #[serde(rename = "lastUpdated")]
    pub last_updated: Option<String>,
    /// 项目路径（仅 project scope）
    #[serde(rename = "projectPath")]
    pub project_path: Option<String>,
    /// Plugin 提供的 skills（详细列表）
    pub skills: Option<Vec<PluginSkill>>,
    /// Plugin 提供的 agents（详细列表）
    pub agents: Option<Vec<PluginAgent>>,
    /// Plugin 提供的 MCP servers
    #[serde(rename = "mcpServers")]
    pub mcp_servers: Option<serde_json::Value>,
    /// Plugin 提供的 commands（commands/ 目录或 plugin.json 声明路径的 .md 文件名）
    pub commands: Vec<String>,
    /// Plugin 提供的 hook 明细（事件 + matcher + 摘要，只读展示）
    pub hooks: Vec<PluginHookEntry>,
    /// Plugin 提供的 LSP 服务器名（.lsp.json / plugin.json lspServers）
    pub lsp: Vec<String>,
    /// Plugin 提供的 monitor 名（monitors/monitors.json / plugin.json monitors）
    pub monitors: Vec<String>,
    /// Plugin 提供的主题名（themes/ 目录 / plugin.json themes）
    pub themes: Vec<String>,
}

/// CLI 返回的 Plugin JSON 结构
#[derive(Debug, Clone, Deserialize)]
struct CliPluginInfo {
    id: String,
    version: String,
    scope: String,
    enabled: bool,
    #[serde(rename = "installPath")]
    install_path: String,
    #[serde(rename = "installedAt")]
    installed_at: Option<String>,
    #[serde(rename = "lastUpdated")]
    last_updated: Option<String>,
    #[serde(rename = "projectPath")]
    project_path: Option<String>,
    #[serde(rename = "mcpServers")]
    mcp_servers: Option<serde_json::Value>,
}

/// 获取所有 Plugins（使用 CLI 命令），包含内部 skills/agents
pub fn get_all_plugins(project_path: &str) -> Result<Vec<PluginInfo>> {
    // 使用 CLI 命令获取 plugin 信息（短 TTL 缓存，见 plugins_list_json）
    let output = plugins_list_json()?;
    let cli_plugins: Vec<CliPluginInfo> =
        serde_json::from_str(&output).context("Failed to parse plugins list JSON")?;

    let mut plugins = Vec::new();

    for cli_plugin in cli_plugins {
        // 过滤当前项目相关的 plugins
        let is_relevant = if cli_plugin.scope == "user" {
            true
        } else if cli_plugin.scope == "project" {
            cli_plugin
                .project_path
                .as_ref()
                .map(|p| p == project_path)
                .unwrap_or(false)
        } else {
            false
        };

        if !is_relevant {
            continue;
        }

        // Plugin 名称（去除 @publisher）
        let plugin_name = cli_plugin.id.split('@').next().unwrap_or(&cli_plugin.id);

        // 验证/修正 install_path
        let valid_path = find_valid_plugin_path(&cli_plugin.install_path, &cli_plugin.id);

        if valid_path.is_none() {
            log::warn!(
                "Plugin {} install path not valid: {}",
                cli_plugin.id,
                cli_plugin.install_path
            );
            continue;
        }

        let install_path = valid_path.unwrap();

        // 解析内部 skills 和 agents
        let plugin_skills = parse_plugin_skills(&install_path, plugin_name);
        let plugin_agents = parse_plugin_agents(&install_path, plugin_name);

        // commands/hooks/lsp/monitors/themes（默认位置 + plugin.json 声明/内联，覆盖官方插件形态）
        let (plugin_commands, _) = parse_plugin_commands_hooks(Path::new(&install_path));
        let plugin_hooks = parse_plugin_hooks_detail(Path::new(&install_path));
        let (plugin_lsp, plugin_monitors, plugin_themes) =
            parse_plugin_extra_components(Path::new(&install_path));

        // 读取 mcpServers（优先从 CLI 输出，后备从文件读取）
        let mcp_servers = cli_plugin
            .mcp_servers
            .clone()
            .or_else(|| read_plugin_mcp_servers(&install_path));

        plugins.push(PluginInfo {
            id: cli_plugin.id.clone(),
            name: plugin_name.to_string(),
            version: cli_plugin.version.clone(),
            scope: cli_plugin.scope.clone(),
            enabled: cli_plugin.enabled,
            install_path: install_path.clone(),
            installed_at: cli_plugin.installed_at.clone(),
            last_updated: cli_plugin.last_updated.clone(),
            project_path: cli_plugin.project_path.clone(),
            skills: plugin_skills,
            agents: plugin_agents,
            mcp_servers,
            commands: plugin_commands,
            hooks: plugin_hooks,
            lsp: plugin_lsp,
            monitors: plugin_monitors,
            themes: plugin_themes,
        });
    }

    Ok(plugins)
}

/// 查找有效的 plugin 安装路径
/// 查找链路（与 Claude CLI 源码 pluginLoader.ts 一致）：
/// 1. cache 路径（installPath）存在 → 直接使用（cache-only 模式）
/// 2. 不存在 → 回退到 marketplace source 路径（full loader 模式）
///
/// Claude CLI 源码中，source 为相对路径字符串的插件（本地 directory marketplace），
/// 即使在 cache-only 模式下也是直接从 marketplaceInstallLocation + source 解析，
/// 不依赖 installPath。只有 source 为对象（npm/github/url）的插件才依赖 installPath。
pub(crate) fn find_valid_plugin_path(original_path: &str, plugin_id: &str) -> Option<String> {
    let home = dirs::home_dir()?;
    find_valid_plugin_path_in(original_path, plugin_id, &home.join(".claude"))
}

/// find_valid_plugin_path 的可注入版本：base_dir 为 .claude 目录（测试用临时目录替换）
pub(crate) fn find_valid_plugin_path_in(
    original_path: &str,
    plugin_id: &str,
    base_dir: &Path,
) -> Option<String> {
    // Step 1: cache 路径存在
    let original = PathBuf::from(original_path);
    if original.exists() {
        return Some(original_path.to_string());
    }

    // Step 2: 回退到 marketplace source 路径
    resolve_marketplace_plugin_path_in(base_dir, plugin_id)
}

/// 从 known_marketplaces.json + marketplace.json 解析 plugin 的实际路径
/// plugin_id 格式: "paper-tool@orczh" → 找 orczh marketplace → 找 paper-tool 的 source
pub(crate) fn resolve_marketplace_plugin_path(plugin_id: &str) -> Option<String> {
    let home = dirs::home_dir()?;
    resolve_marketplace_plugin_path_in(&home.join(".claude"), plugin_id)
}

/// resolve_marketplace_plugin_path 的可注入版本：base_dir 为 .claude 目录（测试用临时目录替换）
pub(crate) fn resolve_marketplace_plugin_path_in(
    base_dir: &Path,
    plugin_id: &str,
) -> Option<String> {
    let marketplaces_file = base_dir.join("plugins").join("known_marketplaces.json");

    if !marketplaces_file.exists() {
        return None;
    }

    // 解析 plugin_id: "paper-tool@orczh" → ("paper-tool", "orczh")
    let parts: Vec<&str> = plugin_id.split('@').collect();
    if parts.len() != 2 {
        return None;
    }
    let plugin_name = parts[0];
    let marketplace_name = parts[1];

    // 读取 known_marketplaces.json
    let content = fs::read_to_string(&marketplaces_file).ok()?;
    let marketplaces: serde_json::Value = serde_json::from_str(&content).ok()?;

    let marketplace = marketplaces.get(marketplace_name)?;
    let install_location = marketplace.get("installLocation")?.as_str()?;

    // 读取 marketplace.json
    let marketplace_json = PathBuf::from(install_location)
        .join(".claude-plugin")
        .join("marketplace.json");

    if !marketplace_json.exists() {
        return None;
    }

    let mp_content = fs::read_to_string(&marketplace_json).ok()?;
    let mp_data: serde_json::Value = serde_json::from_str(&mp_content).ok()?;

    // 在 plugins 数组中找匹配的 plugin
    let plugins = mp_data.get("plugins")?.as_array()?;
    for plugin in plugins {
        if plugin.get("name").and_then(|n| n.as_str()) == Some(plugin_name) {
            // source 可以是字符串 "./paper-tool" 或对象 {"source": "url", "url": "..."}
            if let Some(source_str) = plugin.get("source").and_then(|s| s.as_str()) {
                let resolved = PathBuf::from(install_location).join(source_str);
                if resolved.exists() {
                    return Some(resolved.to_string_lossy().to_string());
                }
            } else if let Some(source_obj) = plugin.get("source").and_then(|s| s.as_object()) {
                // 对象格式: {"source": "./plugins/frontend-design"} 或 {"source": "url", "url": "..."}
                if let Some(path) = source_obj.get("source").and_then(|s| s.as_str()) {
                    let resolved = PathBuf::from(install_location).join(path);
                    if resolved.exists() {
                        return Some(resolved.to_string_lossy().to_string());
                    }
                }
            }
        }
    }

    None
}

/// 读取 plugin 的 MCP servers 配置（从 .mcp.json）
fn read_plugin_mcp_servers(install_path: &str) -> Option<serde_json::Value> {
    // 先尝试读取 .mcp.json（plugin 标准位置）
    let mcp_file = PathBuf::from(install_path).join(".mcp.json");
    if mcp_file.exists() {
        if let Ok(content) = fs::read_to_string(&mcp_file) {
            if let Ok(config) = serde_json::from_str::<serde_json::Value>(&content) {
                // .mcp.json 直接是 servers 的映射，不需要 mcpServers 包装
                return Some(config);
            }
        }
    }

    // 后备：读取 .claude-plugin/plugin.json
    let plugin_config = PathBuf::from(install_path)
        .join(".claude-plugin")
        .join("plugin.json");
    if plugin_config.exists() {
        if let Ok(content) = fs::read_to_string(&plugin_config) {
            if let Ok(config) = serde_json::from_str::<serde_json::Value>(&content) {
                return config.get("mcpServers").cloned();
            }
        }
    }
    None
}

/// 解析 plugin 目录中的 skills
/// 解析 plugin 目录中的 skills（从 SKILL.md frontmatter 提取 description，与 CLI 源码一致）
fn parse_plugin_skills(install_path: &str, plugin_name: &str) -> Option<Vec<PluginSkill>> {
    let skills_dir = PathBuf::from(install_path).join("skills");
    if !skills_dir.exists() {
        return None;
    }

    let mut skills = Vec::new();
    if let Ok(entries) = fs::read_dir(&skills_dir) {
        for entry in entries.flatten() {
            if entry.path().is_dir() {
                let skill_file = entry.path().join("SKILL.md");
                if skill_file.exists() {
                    let skill_name = entry.file_name().to_string_lossy().to_string();
                    let description = parse_skill_description(&skill_file);
                    skills.push(PluginSkill {
                        name: skill_name.clone(),
                        description,
                        invoke_format: format!("/{}:{}", plugin_name, skill_name),
                    });
                }
            }
        }
    }

    if skills.is_empty() {
        None
    } else {
        Some(skills)
    }
}

/// 解析 plugin 目录中的 agents（从 .md 文件 frontmatter 提取，与 CLI 源码一致）
fn parse_plugin_agents(install_path: &str, plugin_name: &str) -> Option<Vec<PluginAgent>> {
    let agents_dir = PathBuf::from(install_path).join("agents");
    if !agents_dir.exists() {
        return None;
    }

    let mut agents = Vec::new();
    if let Ok(entries) = fs::read_dir(&agents_dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.extension().map(|e| e == "md").unwrap_or(false) {
                if let Some((name, description, model)) = parse_agent_frontmatter(&path) {
                    agents.push(PluginAgent {
                        name: name.clone(),
                        description,
                        model,
                        invoke_format: format!("@\"{}:{} (agent)\"", plugin_name, name),
                    });
                }
            }
        }
    }

    if agents.is_empty() {
        None
    } else {
        Some(agents)
    }
}

// ==================== 用户级资源开关 ====================

/// 获取（必要时创建）disabled 备份目录：~/.cc-box/disabled/<category>
pub fn get_disabled_subdir(category: &str) -> Result<PathBuf> {
    let dir = get_gui_config_dir()?.join("disabled").join(category);
    if !dir.exists() {
        fs::create_dir_all(&dir)?;
    }
    Ok(dir)
}

/// 校验名称不含路径穿越字符
pub(crate) fn validate_simple_name(name: &str) -> Result<()> {
    if name.is_empty()
        || name.contains('/')
        || name.contains('\\')
        || name.contains("..")
        || name.contains('\0')
    {
        bail!("Invalid name: {:?}", name);
    }
    Ok(())
}

// ---------- 路径项与符号链接工具 ----------
//
// 背景：用户级 skill/agent 常以**符号链接**安装（macOS/Linux 常见，尤其
// `~/.claude/skills/x -> ../../.agents/skills/x` 这类相对链接）。开关实现必须
// 区分"链接本身"与"链接目标"，且存在性判定不能跟随链接——否则相对链接被 move
// 到 disabled 目录后目标失效，备份既扫不到也恢复不了（历史事故）。

/// 判断目录项是否"存在"——**不跟随**符号链接。
/// `Path::exists()` 对断链符号链接返回 false，会让备份从 UI 消失且无法恢复。
pub(crate) fn path_entry_exists(path: &Path) -> bool {
    fs::symlink_metadata(path).is_ok()
}

/// 是否为断链符号链接（链接本身存在，但目标解析不到）
pub(crate) fn is_broken_symlink(path: &Path) -> bool {
    match fs::symlink_metadata(path) {
        Ok(meta) if meta.file_type().is_symlink() => fs::metadata(path).is_err(),
        _ => false,
    }
}

/// 目录项是否可视为用户级资源：真实目录或符号链接（含断链）
fn is_dir_or_link(path: &Path) -> bool {
    fs::symlink_metadata(path)
        .map(|meta| meta.is_dir() || meta.file_type().is_symlink())
        .unwrap_or(false)
}

/// 词法归一化路径（不触碰文件系统，可用于断链目标）：折叠 "." 与 ".."。
/// 前导 ".." 单独保留——不能把已压入的 ".." 当成普通组件再弹掉
/// （否则相对路径 `a/../../..` 会被错误折成空）。
pub(crate) fn normalize_lexical(path: &Path) -> PathBuf {
    let mut out: Vec<Component> = Vec::new();
    for comp in path.components() {
        match comp {
            Component::CurDir => {}
            Component::ParentDir => match out.last() {
                Some(Component::Normal(_)) => {
                    out.pop();
                }
                // 相对路径的 ".." 无法再回退：累积保留
                Some(Component::ParentDir) | None if !path.is_absolute() => out.push(comp),
                // 绝对路径回到根后忽略多余 ".."
                _ => {}
            },
            other => out.push(other),
        }
    }
    let mut result = PathBuf::new();
    for comp in out {
        result.push(comp.as_os_str());
    }
    result
}

/// 目录的真实路径（穿透祖先符号链接）。相对链接里的 ".." 由内核按**真实目录**
/// 解析，词法拼接会得到错误结果（如 `~/.claude` 本身是 dotfiles 符号链接）。
/// 目录不存在/不可解析时退回原路径。
fn canonical_dir(dir: &Path) -> PathBuf {
    fs::canonicalize(dir).unwrap_or_else(|_| dir.to_path_buf())
}

/// 计算 from_dir → to 的相对路径（两者需为绝对路径）；无有效结果为 None
fn relative_path(from_dir: &Path, to: &Path) -> Option<PathBuf> {
    let from = normalize_lexical(from_dir);
    let to = normalize_lexical(to);
    if !from.is_absolute() || !to.is_absolute() {
        return None;
    }
    let from_comps: Vec<_> = from.components().collect();
    let to_comps: Vec<_> = to.components().collect();
    let common = from_comps
        .iter()
        .zip(to_comps.iter())
        .take_while(|(a, b)| a == b)
        .count();
    let mut out = PathBuf::new();
    for _ in common..from_comps.len() {
        out.push("..");
    }
    for comp in to_comps.iter().skip(common) {
        out.push(comp.as_os_str());
    }
    if out.as_os_str().is_empty() {
        None
    } else {
        Some(out)
    }
}

/// 删除一个目录项（不跟随符号链接：链接本身被删除）。
/// 目录符号链接在 Windows 上必须用 remove_dir，这里做双路兜底。
fn remove_entry(path: &Path) -> std::io::Result<()> {
    let meta = fs::symlink_metadata(path)?;
    if meta.file_type().is_symlink() {
        fs::remove_file(path).or_else(|_| fs::remove_dir(path))
    } else if !meta.is_dir() {
        fs::remove_file(path)
    } else {
        fs::remove_dir_all(path)
    }
}

#[cfg(unix)]
fn create_symlink(target: &Path, link: &Path) -> std::io::Result<()> {
    std::os::unix::fs::symlink(target, link)
}

#[cfg(windows)]
fn create_symlink(target: &Path, link: &Path) -> std::io::Result<()> {
    // 目标可能是相对路径：按链接所在目录解析后判断目录/文件类型
    let resolved = match link.parent() {
        Some(dir) if !target.is_absolute() => dir.join(target),
        _ => target.to_path_buf(),
    };
    if resolved.is_dir() {
        std::os::windows::fs::symlink_dir(target, link)
    } else {
        std::os::windows::fs::symlink_file(target, link)
    }
}

/// 在 dst 处重建符号链接，保持"解析后目标不变"。
/// 相对链接按 dst 所在目录**重算**相对路径（否则搬进 disabled 目录即断链）；
/// 绝对链接原样保留。返回写入的新目标文本。
fn relocate_symlink(src: &Path, dst: &Path) -> std::io::Result<PathBuf> {
    let target = fs::read_link(src)?;
    let new_target = if target.is_absolute() {
        target
    } else {
        // 内核按"链接所在目录的真实路径"解析相对目标（祖先若为符号链接——
        // 如 `~/.claude -> ~/dotfiles/claude`——词法拼接会得到不存在的路径）。
        let src_dir = canonical_dir(src.parent().unwrap_or_else(|| Path::new(".")));
        let dst_dir = canonical_dir(dst.parent().unwrap_or_else(|| Path::new(".")));
        let resolved = if fs::metadata(src).is_err() {
            // 源链接已断链：可能是历史 bug 留下的备份（相对文本原本相对恢复目标
            // 目录生成）。优先按目标目录重新解析——命中则顺手修复该备份。
            let via_dst = normalize_lexical(&dst_dir.join(&target));
            if fs::metadata(&via_dst).is_ok() {
                via_dst
            } else {
                normalize_lexical(&src_dir.join(&target))
            }
        } else {
            normalize_lexical(&src_dir.join(&target))
        };
        relative_path(&dst_dir, &resolved).unwrap_or(resolved)
    };
    // 目标位残留（历史断链备份）先清掉，避免 create 报 EEXIST
    if path_entry_exists(dst) {
        remove_entry(dst)?;
    }
    create_symlink(&new_target, dst)?;
    Ok(new_target)
}

/// 移动目录项：符号链接按"目标不变"重建，其余走 rename（跨卷回退 copy+remove）。
/// 调用方负责冲突判定；这里只清理目标位的断链残留。
pub(crate) fn move_entry(src: &Path, dst: &Path) -> std::io::Result<()> {
    let meta = fs::symlink_metadata(src)?;
    if meta.file_type().is_symlink() {
        match relocate_symlink(src, dst) {
            Ok(_) => {
                // 链接已在新位置建好，删除原链接；失败则回滚，避免出现双份
                if let Err(e) = fs::remove_file(src).or_else(|_| fs::remove_dir(src)) {
                    let _ = remove_entry(dst);
                    return Err(e);
                }
                return Ok(());
            }
            Err(e) => {
                // 无法重建（如 Windows 无权限）：退回 rename 旧行为。
                // 绝对链接仍可用；相对链接会断，但至少不阻断用户操作。
                log::warn!(
                    "[Store] Rebuild symlink failed ({}), falling back to rename: {} -> {}",
                    e,
                    src.display(),
                    dst.display()
                );
            }
        }
    }

    if path_entry_exists(dst) {
        remove_entry(dst)?;
    }

    match fs::rename(src, dst) {
        Ok(()) => Ok(()),
        Err(e) if e.raw_os_error() == Some(17) || e.raw_os_error() == Some(18) => {
            // 跨卷（EXDEV/EEXIST 兼容旧逻辑）：copy + remove。
            // 复制失败时清掉半成品，避免留下"半个备份"阻塞后续开关。
            if let Err(copy_err) = copy_dir_recursive(src, dst) {
                let _ = remove_entry(dst);
                return Err(std::io::Error::other(copy_err.to_string()));
            }
            if src.is_dir() {
                fs::remove_dir_all(src)
            } else {
                fs::remove_file(src)
            }
        }
        Err(e) => Err(e),
    }
}

// ---------- Skill（目录移动）----------

/// 切换用户级 skill 启用状态（公开 API）
pub fn set_skill_enabled(name: &str, enabled: bool) -> Result<()> {
    let claude_skills = claude_config_dir()?.join("skills");
    let disabled_skills = get_disabled_subdir("skills")?;
    set_skill_enabled_in(&claude_skills, &disabled_skills, name, enabled)
}

/// 内部实现（注入 base 路径，便于单元测试）
pub(crate) fn set_skill_enabled_in(
    claude_base: &Path,
    disabled_base: &Path,
    name: &str,
    enabled: bool,
) -> Result<()> {
    validate_simple_name(name)?;
    let src = claude_base.join(name);
    let dst = disabled_base.join(name);

    let (from, to, op) = if enabled {
        // 存在性一律用 symlink_metadata：断链备份也算"存在"，否则永远恢复不了
        if !path_entry_exists(&dst) {
            bail!("Skill '{}' is not disabled (no backup found)", name);
        }
        // active 侧的断链残留对列表不可见，直接覆盖——否则会永久阻塞恢复
        if path_entry_exists(&src) && !is_broken_symlink(&src) {
            bail!("Skill '{}' already exists in active directory (conflict)", name);
        }
        (dst, src, "enable")
    } else {
        if !path_entry_exists(&src) {
            bail!("Skill '{}' not found in active directory", name);
        }
        // 只有"有效备份"才算冲突；断链是历史 bug 的产物，直接覆盖以恢复可用
        if path_entry_exists(&dst) && !is_broken_symlink(&dst) {
            bail!("Skill '{}' already disabled (backup exists)", name);
        }
        (src, dst, "disable")
    };

    if let Err(e) = move_entry(&from, &to) {
        // Windows：skill 目录被运行中的进程占用（典型：Claude Code 会话已加载
        // 该 skill，目录句柄未共享删除）→ 给出可行动的提示而非裸 os error
        if is_dir_busy_error(&e) {
            bail!(
                "Skill '{}' is in use (directory locked, likely by a running Claude \
                 Code session); close the session and retry",
                name
            );
        }
        // 消息带上底层原因：错误经 anyhow to_string 只显最外层 context，
        // 不带 (e) 时 os error（如拒绝访问的具体码）在链路中丢失
        let cause = e.to_string();
        return Err(e)
            .with_context(move || format!("Failed to {} skill '{}' ({})", op, name, cause));
    }
    Ok(())
}

/// 目录被占用判定（Windows：ERROR_ACCESS_DENIED=5 / ERROR_SHARING_VIOLATION=32）
pub(crate) fn is_dir_busy_error(e: &std::io::Error) -> bool {
    matches!(e.raw_os_error(), Some(5) | Some(32))
}

/// 递归复制目录（用于跨卷 fallback）
fn copy_dir_recursive(src: &Path, dst: &Path) -> Result<()> {
    if src.is_dir() {
        fs::create_dir_all(dst)?;
        for entry in fs::read_dir(src)? {
            let entry = entry?;
            let from = entry.path();
            let to = dst.join(entry.file_name());
            copy_dir_recursive(&from, &to)?;
        }
    } else {
        fs::copy(src, dst)?;
    }
    Ok(())
}

// ---------- Agent（文件移动）----------

/// 切换用户级 agent 启用状态（公开 API）
pub fn set_agent_enabled(name: &str, enabled: bool) -> Result<()> {
    let claude_agents = claude_config_dir()?.join("agents");
    let disabled_agents = get_disabled_subdir("agents")?;
    set_agent_enabled_in(&claude_agents, &disabled_agents, name, enabled)
}

/// 在目录中按"frontmatter name 或文件名 stem"定位 agent 文件。
/// 列表展示的是 frontmatter 名，直接拼 `<name>.md` 会在两者不一致时误报 not found；
/// 返回真实文件路径，移动时沿用其文件名（保证 enable/disable 往返不改名）。
fn find_agent_file(dir: &Path, name: &str) -> Result<Option<PathBuf>> {
    if !dir.exists() {
        return Ok(None);
    }
    // 不能先直查 `<name>.md`：列表展示的是 frontmatter name，若 `bar.md` 的
    // frontmatter 是 `baz`、`foo.md` 的 frontmatter 是 `bar`，直查会把 "bar"
    // 错定位到 `bar.md`（该 agent 实际叫 baz）。因此统一扫描，并让
    // frontmatter 命中优先于文件名命中。
    let mut fm_matches = Vec::new();
    let mut stem_matches = Vec::new();
    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if !path.extension().map(|e| e == "md").unwrap_or(false) {
                continue;
            }
            let stem = path
                .file_stem()
                .map(|s| s.to_string_lossy().to_string())
                .unwrap_or_default();
            match parse_agent_frontmatter(&path).map(|(n, _, _)| n) {
                Some(fm) if fm == name => fm_matches.push(path),
                // 有 frontmatter 名却不是用户点的那个 agent：它代表别的条目，
                // 文件名不构成身份依据（否则会移动错文件）
                Some(_) => {}
                // 无 frontmatter：只能退回文件名匹配（与列表的 fallback 一致）
                None if stem == name => stem_matches.push(path),
                None => {}
            }
        }
    }
    let mut matches = if fm_matches.is_empty() {
        stem_matches
    } else {
        fm_matches
    };
    match matches.len() {
        0 => Ok(None),
        1 => Ok(matches.pop()),
        _ => bail!("Agent '{}' is ambiguous (multiple matching files)", name),
    }
}

pub(crate) fn set_agent_enabled_in(
    claude_base: &Path,
    disabled_base: &Path,
    name: &str,
    enabled: bool,
) -> Result<()> {
    validate_simple_name(name)?;

    let (from_dir, to_dir, op) = if enabled {
        (disabled_base, claude_base, "enable")
    } else {
        (claude_base, disabled_base, "disable")
    };

    let from = find_agent_file(from_dir, name)?
        .with_context(|| format!("Agent '{}' not found", name))?;
    let filename = from
        .file_name()
        .with_context(|| format!("Invalid agent path for '{}'", name))?
        .to_owned();
    let to = to_dir.join(&filename);

    // 存在性用 symlink_metadata：断链链接也算存在（否则永远恢复不了）。
    // 目标位是断链残留时直接覆盖（active 侧残留对列表不可见，不放行会永久阻塞恢复）
    if path_entry_exists(&to) && !is_broken_symlink(&to) {
        if enabled {
            bail!("Agent '{}' already exists (conflict)", name);
        }
        bail!("Agent '{}' already disabled", name);
    }

    if let Err(e) = move_entry(&from, &to) {
        // 与 skill 一致：带上底层 os 原因（anyhow 只显最外层 context）
        let cause = e.to_string();
        return Err(e)
            .with_context(move || format!("Failed to {} agent '{}' ({})", op, name, cause));
    }
    Ok(())
}

// ---------- MCP（JSON 剪切/粘贴）----------

/// 切换用户级 MCP server 启用状态（公开 API）
pub fn set_mcp_server_enabled(name: &str, enabled: bool) -> Result<()> {
    let home = dirs::home_dir().context("Home directory not found")?;
    let claude_json = home.join(".claude.json");
    let disabled_dir = get_disabled_subdir("mcp")?;
    set_mcp_server_enabled_in(&claude_json, &disabled_dir, name, enabled)
}

pub(crate) fn set_mcp_server_enabled_in(
    claude_json: &Path,
    disabled_dir: &Path,
    name: &str,
    enabled: bool,
) -> Result<()> {
    validate_simple_name(name)?;
    let backup_file = disabled_dir.join(format!("{}.json", name));

    if enabled {
        // backup → mcpServers.<name>
        if !backup_file.exists() {
            bail!("MCP server '{}' is not disabled", name);
        }
        let entry_content = fs::read_to_string(&backup_file)
            .with_context(|| format!("Failed to read backup for '{}'", name))?;
        let entry: serde_json::Value = serde_json::from_str(&entry_content)
            .with_context(|| format!("Invalid JSON in backup for '{}'", name))?;

        let mut root: serde_json::Value = if claude_json.exists() {
            let raw = fs::read_to_string(claude_json)
                .with_context(|| "Failed to read .claude.json")?;
            serde_json::from_str(&raw).with_context(|| "Failed to parse .claude.json")?
        } else {
            serde_json::json!({})
        };

        let root_obj = root
            .as_object_mut()
            .context(".claude.json root is not an object")?;
        let mcp_field = root_obj
            .entry("mcpServers".to_string())
            .or_insert_with(|| serde_json::json!({}));
        let mcp_obj = mcp_field
            .as_object_mut()
            .context("mcpServers is not an object")?;

        if mcp_obj.contains_key(name) {
            bail!("MCP server '{}' already exists in active config (conflict)", name);
        }
        mcp_obj.insert(name.to_string(), entry);

        write_json_atomic(claude_json, &root)?;
        fs::remove_file(&backup_file)
            .with_context(|| format!("Failed to remove backup for '{}'", name))?;
    } else {
        // mcpServers.<name> → backup
        if !claude_json.exists() {
            bail!(".claude.json not found, cannot disable '{}'", name);
        }
        if backup_file.exists() {
            bail!("MCP server '{}' already disabled", name);
        }
        let raw = fs::read_to_string(claude_json)
            .with_context(|| "Failed to read .claude.json")?;
        let mut root: serde_json::Value = serde_json::from_str(&raw)
            .with_context(|| "Failed to parse .claude.json")?;

        let mcp_obj = root
            .get_mut("mcpServers")
            .and_then(|v| v.as_object_mut())
            .context("mcpServers not found or not an object")?;

        let entry = mcp_obj
            .remove(name)
            .with_context(|| format!("MCP server '{}' not found in active config", name))?;

        fs::create_dir_all(disabled_dir)?;
        let pretty = serde_json::to_string_pretty(&entry)?;
        fs::write(&backup_file, pretty)
            .with_context(|| format!("Failed to write backup for '{}'", name))?;

        write_json_atomic(claude_json, &root)?;
    }
    Ok(())
}

/// 原子写文本：先写 .tmp 再 rename（市场浏览缓存等非配置文件）
pub(crate) fn write_text_atomic(path: &Path, content: &[u8]) -> Result<()> {
    let tmp = path.with_extension("tmp");
    fs::write(&tmp, content)
        .with_context(|| format!("Failed to write {}", tmp.display()))?;
    fs::rename(&tmp, path).with_context(|| "Failed to rename .tmp to target")?;
    Ok(())
}

/// 原子写 JSON：先写 .tmp 再 rename
pub(crate) fn write_json_atomic(path: &Path, value: &serde_json::Value) -> Result<()> {
    let tmp = path.with_extension("json.tmp");
    let content = serde_json::to_string_pretty(value)?;
    fs::write(&tmp, &content).with_context(|| "Failed to write .tmp file")?;
    fs::rename(&tmp, path).with_context(|| "Failed to rename .tmp to target")?;
    Ok(())
}

// ---------- Plugin（CLI 调用）----------

/// 切换 plugin 启用状态（调用 claude plugin enable/disable）
pub fn set_plugin_enabled(plugin_id: &str, enabled: bool) -> Result<()> {
    validate_simple_name(plugin_id).context("Invalid plugin id")?;
    // plugin_id 可能含 @（如 paper-tool@orczh），允许这个字符
    if plugin_id.contains('/') || plugin_id.contains('\\') || plugin_id.contains("..") {
        bail!("Invalid plugin id: {:?}", plugin_id);
    }
    let action = if enabled { "enable" } else { "disable" };
    let args = format!("plugin {} {}", action, plugin_id);
    // 无论成败都失效缓存：CLI 可能已部分写入，且失败后下次读取必须反映真实状态
    // （市场列表缓存含 installed/enabled 徽标，一并失效）
    let result = run_claude_command_strict(&args);
    invalidate_plugins_list_cache();
    invalidate_plugin_market_cache();
    result.with_context(|| format!("Failed to {} plugin '{}'", action, plugin_id))?;
    Ok(())
}

// ---------- Plugin 市场（浏览 + 安装管理，全部包装 claude plugin CLI）----------
//
// 市场注册事实源是 Claude 自己的 known_marketplaces.json，CC-Box 只读；
// 添加/移除/刷新/安装/卸载/更新全部转发 CLI（marketplace add 解析 URL/GitHub/
// 本地路径的能力在 CLI 侧），CC-Box 不建第二份注册表。

/// 市场操作（clone/网络下载）的超时上限
const CLI_MARKET_TIMEOUT: std::time::Duration = std::time::Duration::from_secs(120);

/// 带超时执行已配置好的命令（argv 语义，供长耗时 CLI 调用使用）。
/// 成功 → stdout；失败（非零退出/超时/spawn 失败）→ Err（优先 stderr）。
/// 管道读取放独立线程：避免子进程写满管道阻塞而我们仍在轮询的死锁；
/// 超时分支不 join 读线程（kill 后孙进程可能持有管道），直接 bail。
pub(crate) fn run_command_with_timeout(
    cmd: &mut std::process::Command,
    timeout: std::time::Duration,
) -> Result<String> {
    use std::process::Stdio;

    cmd.stdout(Stdio::piped()).stderr(Stdio::piped());
    let mut child = cmd.spawn().context("Failed to spawn command")?;

    let stdout_pipe = child.stdout.take().context("missing stdout pipe")?;
    let stderr_pipe = child.stderr.take().context("missing stderr pipe")?;
    let stdout_join = std::thread::spawn(move || {
        let mut buf = String::new();
        let _ = std::io::Read::read_to_string(&mut { stdout_pipe }, &mut buf);
        buf
    });
    let stderr_join = std::thread::spawn(move || {
        let mut buf = String::new();
        let _ = std::io::Read::read_to_string(&mut { stderr_pipe }, &mut buf);
        buf
    });

    let start = std::time::Instant::now();
    let poll = std::time::Duration::from_millis(50);
    let status = loop {
        match child.try_wait() {
            Ok(Some(status)) => break status,
            Ok(None) => {
                if start.elapsed() >= timeout {
                    kill_process_tree(child.id());
                    let _ = child.wait();
                    bail!("command timed out after {}s", timeout.as_secs());
                }
                std::thread::sleep(poll);
            }
            Err(e) => bail!("wait failed: {}", e),
        }
    };

    let stdout = stdout_join.join().unwrap_or_default();
    let stderr = stderr_join.join().unwrap_or_default();

    if status.success() {
        Ok(stdout)
    } else {
        bail!(
            "command failed (exit {:?}): {}",
            status.code(),
            if stderr.trim().is_empty() { stdout.trim() } else { stderr.trim() }
        );
    }
}

/// 终止进程树：Windows 上 cmd/npm 等 wrapper 会派生孙进程（claude → node），
/// 只 kill 外层会留下持有管道的孤儿——用 taskkill /T 连树杀；Unix 直接 SIGKILL。
fn kill_process_tree(pid: u32) {
    #[cfg(windows)]
    {
        use std::os::windows::process::CommandExt;
        let _ = std::process::Command::new("taskkill")
            .args(["/F", "/T", "/PID", &pid.to_string()])
            .creation_flags(0x0800_0000) // CREATE_NO_WINDOW
            .output();
    }
    #[cfg(not(windows))]
    {
        let _ = std::process::Command::new("kill")
            .args(["-9", &pid.to_string()])
            .output();
    }
}

/// argv 版 claude CLI 调用（现有 run_claude_command 用 split_whitespace，
/// 无法携带含空格的参数，也没有 cwd 与超时）
fn run_claude_command_argv(args: &[&str], cwd: Option<&Path>) -> Result<String> {
    let mut cmd = claude_command();
    cmd.args(args);
    if let Some(dir) = cwd {
        cmd.current_dir(dir);
    }
    run_command_with_timeout(&mut cmd, CLI_MARKET_TIMEOUT)
        .with_context(|| format!("claude {} failed", args.join(" ")))
}

#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MarketplacePluginEntry {
    pub name: String,
    pub description: Option<String>,
    pub version: Option<String>,
    pub category: Option<String>,
    pub tags: Vec<String>,
    /// 市场清单声明的组件清单（安装确认的信任提示依据）
    pub components: Option<MarketplaceComponents>,
    /// 安装量（Claude 的 plugin-catalog-cache.json::unique_installs，按热度排序依据）
    pub installs: Option<u64>,
    /// 精选置顶（marketplace.json 条目的 `ccbox.pinned`，自有市场用）
    #[serde(default)]
    pub pinned: bool,
    /// 精选排序（`ccbox.order`，升序；与 pinned 一起决定顺序）
    #[serde(default)]
    pub order: Option<i64>,
    /// 插件图标（`ccbox.icon`：emoji / 短文本 / http(s) 图片 URL）
    #[serde(default)]
    pub icon: Option<String>,
    /// 该插件是否已安装（按 `name@市场名` 匹配已安装列表）
    pub installed: bool,
    /// 已安装时的启用状态
    pub enabled: Option<bool>,
}

/// 插件组件清单（marketplace.json 的 commands/agents/hooks/mcpServers/skills，
/// 每项可为相对路径字符串或 {name, description} 对象，统一取名称）
#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MarketplaceComponents {
    pub commands: Vec<String>,
    pub agents: Vec<String>,
    pub hooks: Vec<String>,
    pub mcp_servers: Vec<String>,
    pub skills: Vec<String>,
    pub lsp_servers: Vec<String>,
}

impl MarketplaceComponents {
    fn is_empty(&self) -> bool {
        self.commands.is_empty()
            && self.agents.is_empty()
            && self.hooks.is_empty()
            && self.mcp_servers.is_empty()
            && self.skills.is_empty()
            && self.lsp_servers.is_empty()
    }
}

/// 组件名展示归一化：路径 → 末段，去 .md/.json 扩展名
/// （市场清单常写 "./skills/xlsx"、"./hooks/hooks.json"）
fn normalize_component_name(raw: &str) -> Option<String> {
    let s = raw.trim().trim_end_matches(['/', '\\']);
    if s.is_empty() {
        return None;
    }
    let seg = s.rsplit(['/', '\\']).next().unwrap_or(s);
    let name = seg
        .strip_suffix(".md")
        .or_else(|| seg.strip_suffix(".json"))
        .unwrap_or(seg)
        .trim();
    (!name.is_empty()).then(|| name.to_string())
}

/// 单类组件展示上限（防止超大型插件把详情撑爆）
const MAX_COMPONENT_NAMES: usize = 40;

fn push_unique(out: &mut Vec<String>, item: String) {
    if out.len() < MAX_COMPONENT_NAMES && !out.iter().any(|x| x == &item) {
        out.push(item);
    }
}

#[derive(Clone, Copy, PartialEq)]
enum CompKind {
    /// commands / agents：*.md 文件名
    Md,
    /// skills：含 SKILL.md 的子目录
    Skills,
    /// hooks：hooks.json 的事件名
    Hooks,
    /// mcpServers：.mcp.json 的服务器名
    Mcp,
}

/// 扫描命令/agent 目录：*.md 文件名（去扩展名）
fn scan_md_dir(dir: &Path, out: &mut Vec<String>) {
    let Ok(rd) = fs::read_dir(dir) else { return };
    let mut names: Vec<String> = rd
        .filter_map(|e| e.ok())
        .filter(|e| e.path().is_file())
        .filter_map(|e| {
            let n = e.file_name().to_string_lossy().to_string();
            n.strip_suffix(".md").map(|s| s.to_string())
        })
        .collect();
    names.sort();
    for n in names {
        push_unique(out, n);
    }
}

/// 扫描 skills 目录：含 SKILL.md 的子目录名
fn scan_skills_dir(dir: &Path, out: &mut Vec<String>) {
    let Ok(rd) = fs::read_dir(dir) else { return };
    let mut names: Vec<String> = rd
        .filter_map(|e| e.ok())
        .filter(|e| e.path().join("SKILL.md").is_file())
        .map(|e| e.file_name().to_string_lossy().to_string())
        .collect();
    names.sort();
    for n in names {
        push_unique(out, n);
    }
}

/// hooks 文件 → 事件名（兼容 { hooks: { Event: [...] } } 与 { Event: [...] }）
fn scan_hooks_file(file: &Path, out: &mut Vec<String>) {
    let Ok(content) = fs::read_to_string(file) else { return };
    let Ok(v) = serde_json::from_str::<serde_json::Value>(&content) else { return };
    let Some(map) = v
        .get("hooks")
        .and_then(|h| h.as_object())
        .or_else(|| v.as_object())
    else {
        return;
    };
    let mut names: Vec<String> = map
        .iter()
        .filter(|(_, val)| val.is_array())
        .map(|(k, _)| k.clone())
        .collect();
    names.sort();
    for n in names {
        push_unique(out, n);
    }
}

/// mcp 配置 → 服务器名（兼容 { mcpServers: {...} } 与直接 {...}）
fn scan_mcp_file(file: &Path, out: &mut Vec<String>) {
    let Ok(content) = fs::read_to_string(file) else { return };
    let Ok(v) = serde_json::from_str::<serde_json::Value>(&content) else { return };
    let Some(map) = v
        .get("mcpServers")
        .and_then(|m| m.as_object())
        .or_else(|| v.as_object())
    else {
        return;
    };
    let mut names: Vec<String> = map.keys().cloned().collect();
    names.sort();
    for n in names {
        push_unique(out, n);
    }
}

/// monitors 配置 → 监视器名（条目 name 字段；兼容 { monitors: [...] } 包装）
fn scan_monitors_file(file: &Path, out: &mut Vec<String>) {
    let Ok(content) = fs::read_to_string(file) else { return };
    let Ok(v) = serde_json::from_str::<serde_json::Value>(&content) else { return };
    let Some(items) = v.as_array().or_else(|| v.get("monitors").and_then(|m| m.as_array())) else {
        return;
    };
    for item in items {
        if let Some(n) = item.get("name").and_then(|n| n.as_str()) {
            push_unique(out, n.to_string());
        }
    }
}

/// 目录 → *.json 文件名（去扩展名，如 themes/）
fn scan_json_names(dir: &Path, out: &mut Vec<String>) {
    let Ok(rd) = fs::read_dir(dir) else { return };
    let mut names: Vec<String> = rd
        .filter_map(|e| e.ok())
        .filter(|e| e.path().is_file())
        .filter_map(|e| {
            let n = e.file_name().to_string_lossy().to_string();
            n.strip_suffix(".json").map(|s| s.to_string())
        })
        .collect();
    names.sort();
    for n in names {
        push_unique(out, n);
    }
}

/// 按类型扫描一个路径（目录或文件）
fn scan_path(kind: CompKind, path: &Path, out: &mut Vec<String>) {
    match kind {
        CompKind::Md => scan_md_dir(path, out),
        CompKind::Skills => scan_skills_dir(path, out),
        CompKind::Hooks => {
            if path.is_dir() {
                scan_hooks_file(&path.join("hooks.json"), out);
            } else {
                scan_hooks_file(path, out);
            }
        }
        CompKind::Mcp => {
            if path.is_dir() {
                scan_mcp_file(&path.join(".mcp.json"), out);
            } else {
                scan_mcp_file(path, out);
            }
        }
    }
}

/// 清单声明的组件项：字符串 / 数组（字符串或 {name} 对象）/ 对象键
fn declared_raw(entry: &serde_json::Value, key: &str) -> Vec<String> {
    match entry.get(key) {
        Some(serde_json::Value::String(s)) => vec![s.clone()],
        Some(serde_json::Value::Array(a)) => a
            .iter()
            .filter_map(|item| {
                item.as_str()
                    .map(|s| s.to_string())
                    .or_else(|| item.get("name").and_then(|n| n.as_str()).map(|s| s.to_string()))
            })
            .collect(),
        Some(serde_json::Value::Object(o)) => o.keys().cloned().collect(),
        _ => Vec::new(),
    }
}

/// 展开清单声明：能落到插件目录里的路径按类型扫描（目录→扫，hooks/mcp 文件→读），
/// 否则按归一化名展示
fn expand_declared(
    entry: &serde_json::Value,
    key: &str,
    plugin_dir: Option<&Path>,
    kind: CompKind,
    out: &mut Vec<String>,
) {
    for raw in declared_raw(entry, key) {
        if let Some(dir) = plugin_dir {
            let p = Path::new(&raw);
            let path = if p.is_absolute() {
                p.to_path_buf()
            } else {
                dir.join(&raw)
            };
            let file_like = matches!(kind, CompKind::Hooks | CompKind::Mcp);
            if path.is_dir() || (file_like && path.is_file()) {
                scan_path(kind, &path, out);
                continue;
            }
        }
        if let Some(n) = normalize_component_name(&raw) {
            push_unique(out, n);
        }
    }
}

fn read_plugin_json(dir: &Path) -> Option<serde_json::Value> {
    // 规范位置 .claude-plugin/plugin.json；agent-plugins 新 schema 也允许根目录 plugin.json
    let content = fs::read_to_string(dir.join(".claude-plugin").join("plugin.json"))
        .or_else(|_| fs::read_to_string(dir.join("plugin.json")))
        .ok()?;
    serde_json::from_str(&content).ok()
}

/// hooks 配置文件 → (事件名, matcher 条目对象) 列表
/// （兼容 { hooks: { Event: [...] } } 与 { Event: [...] }）
fn hooks_file_entries(file: &Path, out: &mut Vec<(String, serde_json::Value)>) {
    let Ok(content) = fs::read_to_string(file) else { return };
    let Ok(v) = serde_json::from_str::<serde_json::Value>(&content) else { return };
    let Some(map) = v
        .get("hooks")
        .and_then(|h| h.as_object())
        .or_else(|| v.as_object())
    else {
        return;
    };
    for (event, val) in map {
        if let Some(arr) = val.as_array() {
            for e in arr {
                out.push((event.clone(), e.clone()));
            }
        }
    }
}

/// 插件 hooks 的单一收集来源：hooks/hooks.json、根 hooks.json、
/// plugin.json 声明路径（字符串/数组）与内联 object（键=事件名）
fn collect_plugin_hook_entries(install_path: &Path) -> Vec<(String, serde_json::Value)> {
    let mut out = Vec::new();
    hooks_file_entries(&install_path.join("hooks").join("hooks.json"), &mut out);
    hooks_file_entries(&install_path.join("hooks.json"), &mut out);
    if let Some(pj) = read_plugin_json(install_path) {
        for raw in declared_raw(&pj, "hooks") {
            let file = install_path.join(&raw);
            if file.is_file() {
                hooks_file_entries(&file, &mut out);
            } else if let Some(arr) = pj
                .get("hooks")
                .and_then(|h| h.as_object())
                .and_then(|m| m.get(&raw))
                .and_then(|v| v.as_array())
            {
                // 内联 object：raw 为事件名，值为 matcher 条目数组
                for e in arr {
                    out.push((raw.clone(), e.clone()));
                }
            }
        }
    }
    out
}

/// 插件提供的 commands 与 hook 事件（供面板明细与 Hooks 面板插件组）。
/// 覆盖官方形态：commands/ 目录、hooks/hooks.json、根 hooks.json、
/// plugin.json 声明的 commands 路径与内联（object 键=事件名）/路径式（含数组）hooks。
pub(crate) fn parse_plugin_commands_hooks(install_path: &Path) -> (Vec<String>, Vec<String>) {
    let mut commands = Vec::new();
    scan_md_dir(&install_path.join("commands"), &mut commands);
    let mut hooks = Vec::new();
    let mut seen: Vec<String> = Vec::new();
    for (event, _) in collect_plugin_hook_entries(install_path) {
        if !seen.contains(&event) {
            seen.push(event);
        }
    }
    seen.sort();
    for event in seen {
        push_unique(&mut hooks, event);
    }
    if let Some(pj) = read_plugin_json(install_path) {
        if commands.is_empty() {
            expand_declared(&pj, "commands", Some(install_path), CompKind::Md, &mut commands);
        }
    }
    (commands, hooks)
}

/// hook 对象摘要：command → url → prompt 回退链（与 HookItem 一致）
fn plugin_hook_summary(h: &serde_json::Value) -> Option<String> {
    for key in ["command", "url", "prompt"] {
        if let Some(s) = h.get(key).and_then(|v| v.as_str()) {
            return Some(s.to_string());
        }
    }
    None
}

/// 插件 hooks 只读明细（Hooks 面板「插件」组）：展开为逐条 hook，
/// 按事件名稳定排序
pub(crate) fn parse_plugin_hooks_detail(install_path: &Path) -> Vec<PluginHookEntry> {
    let mut entries = collect_plugin_hook_entries(install_path);
    entries.sort_by(|a, b| a.0.cmp(&b.0));
    let mut out = Vec::new();
    for (event, entry) in entries {
        let matcher = entry.get("matcher").and_then(|m| m.as_str()).map(String::from);
        if let Some(hooks) = entry.get("hooks").and_then(|h| h.as_array()) {
            for h in hooks {
                out.push(PluginHookEntry {
                    event: event.clone(),
                    matcher: matcher.clone(),
                    hook_type: h.get("type").and_then(|t| t.as_str()).map(String::from),
                    summary: plugin_hook_summary(h),
                    timeout: h.get("timeout").cloned().filter(|v| !v.is_null()),
                });
            }
        } else {
            out.push(PluginHookEntry {
                event,
                matcher,
                hook_type: None,
                summary: None,
                timeout: None,
            });
        }
    }
    out
}

/// 插件提供的 lsp / monitors / themes 组件名（默认位置 + plugin.json 声明/内联）。
/// lsp：根 .lsp.json 或 plugin.json lspServers（内联 object 键=服务器名 / 路径）；
/// monitors：monitors/monitors.json 或 plugin.json monitors（内联条目 name / 路径）；
/// themes：themes/ 目录 *.json 或 plugin.json themes 路径（目录→扫，文件→名）。
pub(crate) fn parse_plugin_extra_components(install_path: &Path) -> (Vec<String>, Vec<String>, Vec<String>) {
    let mut lsp = Vec::new();
    scan_mcp_file(&install_path.join(".lsp.json"), &mut lsp);
    let mut monitors = Vec::new();
    scan_monitors_file(&install_path.join("monitors").join("monitors.json"), &mut monitors);
    let mut themes = Vec::new();
    scan_json_names(&install_path.join("themes"), &mut themes);
    if let Some(pj) = read_plugin_json(install_path) {
        for raw in declared_raw(&pj, "lspServers") {
            let file = install_path.join(&raw);
            if file.is_file() {
                scan_mcp_file(&file, &mut lsp);
            } else {
                push_unique(&mut lsp, raw);
            }
        }
        for raw in declared_raw(&pj, "monitors") {
            let file = install_path.join(&raw);
            if file.is_file() {
                scan_monitors_file(&file, &mut monitors);
            } else {
                push_unique(&mut monitors, raw);
            }
        }
        for raw in declared_raw(&pj, "themes") {
            let p = install_path.join(&raw);
            if p.is_dir() {
                scan_json_names(&p, &mut themes);
            } else if let Some(n) = normalize_component_name(&raw) {
                push_unique(&mut themes, n);
            }
        }
    }
    (lsp, monitors, themes)
}

/// 组件清单：清单/plugin.json 声明优先，缺失时扫描插件目录
/// （Claude 原生插件按 commands/ agents/ skills/ hooks/hooks.json .mcp.json 自动发现）
pub(crate) fn parse_components(entry: &serde_json::Value, plugin_dir: Option<&Path>) -> Option<MarketplaceComponents> {
    let pj = plugin_dir.and_then(read_plugin_json);
    let mut c = MarketplaceComponents {
        commands: Vec::new(),
        agents: Vec::new(),
        hooks: Vec::new(),
        mcp_servers: Vec::new(),
        skills: Vec::new(),
        lsp_servers: Vec::new(),
    };

    let mut decl = |key: &str, kind: CompKind, out: &mut Vec<String>| {
        expand_declared(entry, key, plugin_dir, kind, out);
        if let Some(pj) = &pj {
            expand_declared(pj, key, plugin_dir, kind, out);
        }
    };
    decl("commands", CompKind::Md, &mut c.commands);
    decl("agents", CompKind::Md, &mut c.agents);
    decl("hooks", CompKind::Hooks, &mut c.hooks);
    decl("mcpServers", CompKind::Mcp, &mut c.mcp_servers);
    decl("skills", CompKind::Skills, &mut c.skills);
    // LSP：条目/plugin.json 内联声明（object 键=服务器名）或 .lsp.json 路径
    decl("lspServers", CompKind::Mcp, &mut c.lsp_servers);

    if let Some(dir) = plugin_dir {
        if c.commands.is_empty() {
            scan_md_dir(&dir.join("commands"), &mut c.commands);
        }
        if c.agents.is_empty() {
            scan_md_dir(&dir.join("agents"), &mut c.agents);
        }
        if c.skills.is_empty() {
            scan_skills_dir(&dir.join("skills"), &mut c.skills);
        }
        if c.hooks.is_empty() {
            scan_hooks_file(&dir.join("hooks").join("hooks.json"), &mut c.hooks);
            scan_hooks_file(&dir.join("hooks.json"), &mut c.hooks);
        }
        if c.mcp_servers.is_empty() {
            scan_mcp_file(&dir.join(".mcp.json"), &mut c.mcp_servers);
            // agent-plugins 新 schema：mcp.json 与 .mcp.json 同义
            scan_mcp_file(&dir.join("mcp.json"), &mut c.mcp_servers);
        }
        if c.lsp_servers.is_empty() {
            scan_mcp_file(&dir.join(".lsp.json"), &mut c.lsp_servers);
        }
    }

    if c.is_empty() {
        None
    } else {
        Some(c)
    }
}

/// 读取 Claude 维护的插件热度（`plugins/plugin-catalog-cache.json` 的
/// `catalog.plugins[*].unique_installs`，键为 `name@marketplace`）。
/// 文件缺失/损坏 → 空表（列表退化为按名称排序）。
fn load_plugin_installs(config_dir: &Path) -> HashMap<String, u64> {
    let path = config_dir
        .join("plugins")
        .join("plugin-catalog-cache.json");
    let Ok(content) = fs::read_to_string(&path) else {
        return HashMap::new();
    };
    let Ok(v) = serde_json::from_str::<serde_json::Value>(&content) else {
        return HashMap::new();
    };
    let Some(map) = v
        .pointer("/catalog/plugins")
        .and_then(|p| p.as_object())
    else {
        return HashMap::new();
    };
    map.iter()
        .filter_map(|(id, entry)| {
            entry
                .get("unique_installs")
                .and_then(|n| n.as_u64())
                .map(|n| (id.clone(), n))
        })
        .collect()
}

/// 解析插件源码目录：市场条目相对 source 指向仓库内路径；
/// 远程源未安装时没有本地源码，退到已安装缓存
/// `<config>/plugins/cache/<市场>/<插件>/<版本>/`。
/// market_dir = None（json 文件形态的浏览缓存）时无仓库目录，直接走缓存回退
/// ——不能传空 Path，否则相对 source 会按进程 CWD 探测出无关目录。
fn resolve_plugin_dir(
    market_dir: Option<&Path>,
    market_name: &str,
    entry: &serde_json::Value,
    plugin_name: &str,
    config_dir: &Path,
) -> Option<PathBuf> {
    if let Some(dir) = market_dir {
        // installLocation 直接指向 .json（file 源市场）时没有仓库目录可扫描
        let has_repo_dir = dir.extension().map(|e| e != "json").unwrap_or(true);
        if let Some(serde_json::Value::String(src)) = entry.get("source") {
            // 只接受市场仓库内的相对路径（防止远程清单指向本地任意目录）
            if has_repo_dir && !src.contains("..") {
                let p = Path::new(src);
                if !p.is_absolute() {
                    let plugin = dir.join(src);
                    if plugin.is_dir() {
                        return Some(plugin);
                    }
                }
            }
        }
    }
    let root = config_dir
        .join("plugins")
        .join("cache")
        .join(market_name)
        .join(plugin_name);
    let mut dirs: Vec<(std::time::SystemTime, PathBuf)> = fs::read_dir(&root)
        .ok()?
        .filter_map(|e| e.ok())
        .filter(|e| e.path().is_dir())
        .map(|e| {
            let mtime = e
                .metadata()
                .and_then(|m| m.modified())
                .unwrap_or(std::time::UNIX_EPOCH);
            (mtime, e.path())
        })
        .collect();
    dirs.sort_by(|a, b| a.0.cmp(&b.0));
    dirs.pop().map(|(_, p)| p)
}

#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MarketplaceInfo {
    pub name: String,
    pub install_location: String,
    /// 上次拉取时间（注册市场 = Claude 的 known_marketplaces.json::lastUpdated；
    /// 未注册市场 = CC-Box 浏览缓存的拉取时间）
    pub last_updated: Option<String>,
    pub plugins: Vec<MarketplacePluginEntry>,
    /// marketplace.json 读取/解析失败原因（市场仍列出，便于用户排查）
    pub load_error: Option<String>,
    /// 来源：preset（OSS 推荐，未添加）/ user（用户添加）/ registered（仅 Claude 注册）
    pub origin: String,
    /// 是否已注册到 Claude（known_marketplaces.json）
    pub registered: bool,
    /// 市场来源（owner/repo、git URL、marketplace.json URL、本地路径；仅 preset/user）
    pub source: Option<String>,
    /// preset 展示字段
    pub description: Option<String>,
    pub pinned: bool,
    pub order: Option<i64>,
    pub icon: Option<String>,
}

/// 市场列表内存缓存（组件目录扫描 + plugins list 的代价随插件数量增长）
static MARKET_LIST_CACHE: Mutex<Option<(std::time::Instant, Vec<MarketplaceInfo>)>> = Mutex::new(None);
const MARKET_LIST_TTL: std::time::Duration = std::time::Duration::from_secs(60);

/// 失效市场列表缓存（市场增删/刷新、插件安装或启停后调用）
pub(crate) fn invalidate_plugin_market_cache() {
    if let Ok(mut guard) = MARKET_LIST_CACHE.lock() {
        *guard = None;
    }
}

/// 读取 Claude 已注册市场与各市场插件条目（含 installed/enabled 徽标）。
/// 未注册任何市场（文件不存在）时返回空列表而非报错（首次使用场景）。
/// 结果按 60s 内存缓存：打开市场浏览器与刷新后立即重读都不再重复扫描插件目录。
pub fn get_plugin_marketplaces() -> Result<Vec<MarketplaceInfo>> {
    if let Ok(guard) = MARKET_LIST_CACHE.lock() {
        if let Some((at, data)) = guard.as_ref() {
            if at.elapsed() < MARKET_LIST_TTL {
                return Ok(data.clone());
            }
        }
    }
    let data = load_plugin_marketplaces()?;
    if let Ok(mut guard) = MARKET_LIST_CACHE.lock() {
        *guard = Some((std::time::Instant::now(), data.clone()));
    }
    Ok(data)
}

fn load_plugin_marketplaces() -> Result<Vec<MarketplaceInfo>> {
    // installed 徽标数据来自 plugins list --json（短 TTL 缓存）
    let mut installed: std::collections::HashMap<String, bool> = Default::default();
    if let Ok(raw) = plugins_list_json() {
        if let Ok(list) =
            serde_json::from_str::<Vec<CliPluginLite>>(&raw)
        {
            for p in list {
                installed.insert(p.id, p.enabled);
            }
        }
    }
    let custom = get_app_config()?.custom_markets.unwrap_or_default();
    // 推荐清单获取失败不阻塞市场列表（只影响 preset 分组）
    let presets = get_market_presets().unwrap_or_default();
    get_plugin_marketplaces_in(&claude_config_dir()?, &installed, &custom, &presets)
}

/// 市场来源分类（未注册市场的浏览方式）
pub(crate) enum MarketSource {
    /// 本地目录（直读，不缓存）
    Dir(PathBuf),
    /// marketplace.json URL（HTTP GET → 缓存文件）
    JsonUrl(String),
    /// git 仓库（owner/repo 简写或 git URL → clone --depth 1 到缓存目录）
    Git(String),
}

/// 分类市场来源：URL(.json) / URL(git) / 本地目录 / GitHub owner-repo 简写
pub(crate) fn classify_market_source(source: &str) -> Result<MarketSource> {
    let s = source.trim();
    let lower = s.to_lowercase();
    if lower.starts_with("http://") || lower.starts_with("https://") {
        if lower.split('?').next().unwrap_or("").ends_with(".json") {
            return Ok(MarketSource::JsonUrl(s.to_string()));
        }
        return Ok(MarketSource::Git(s.to_string()));
    }
    let p = Path::new(s);
    if p.exists() {
        return Ok(MarketSource::Dir(p.to_path_buf()));
    }
    // GitHub owner/repo 简写（无 scheme、一段斜杠、无路径分隔之外的字符）
    let parts: Vec<&str> = s.split('/').collect();
    if parts.len() == 2
        && !s.contains('\\')
        && parts.iter().all(|seg| !seg.is_empty() && !seg.contains(':') && !seg.contains(' '))
    {
        return Ok(MarketSource::Git(format!("https://github.com/{}.git", s)));
    }
    bail!("无法识别的市场来源：{:?}（支持 GitHub owner/repo、git URL、marketplace.json URL、本地目录）", source);
}

/// CC-Box 市场浏览缓存根目录（~/.cc-box/cache/markets/）
fn market_browse_cache_dir() -> Result<PathBuf> {
    let dir = get_gui_config_dir()?.join("cache").join("markets");
    fs::create_dir_all(&dir)?;
    Ok(dir)
}

/// 未注册市场的浏览本地副本路径（目录 = git 克隆；文件 = 下载的 marketplace.json）
fn market_browse_local_copy(source: &str) -> Result<MarketSource> {
    match classify_market_source(source)? {
        MarketSource::Dir(p) => Ok(MarketSource::Dir(p)),
        MarketSource::JsonUrl(url) => {
            let cache = market_browse_cache_dir()?.join(format!("src-{}.json", short_hash(&url)));
            if cache.is_file() {
                return Ok(MarketSource::Dir(cache));
            }
            let text = http_get_market_json(&url)?;
            write_text_atomic(&cache, text.as_bytes())?;
            Ok(MarketSource::Dir(cache))
        }
        MarketSource::Git(url) => {
            let dir = market_browse_cache_dir()?.join(format!("src-{}", short_hash(&url)));
            if dir.join(".claude-plugin").join("marketplace.json").is_file() {
                return Ok(MarketSource::Dir(dir));
            }
            clone_market_repo(&url, &dir)?;
            Ok(MarketSource::Dir(dir))
        }
    }
}

/// 强制刷新浏览缓存（先清理再重新拉取）
fn refresh_market_browse_copy(source: &str) -> Result<MarketSource> {
    match classify_market_source(source)? {
        MarketSource::Dir(p) => Ok(MarketSource::Dir(p)),
        MarketSource::JsonUrl(url) => {
            let cache = market_browse_cache_dir()?.join(format!("src-{}.json", short_hash(&url)));
            let text = http_get_market_json(&url)?;
            write_text_atomic(&cache, text.as_bytes())?;
            Ok(MarketSource::Dir(cache))
        }
        MarketSource::Git(url) => {
            let dir = market_browse_cache_dir()?.join(format!("src-{}", short_hash(&url)));
            let _ = fs::remove_dir_all(&dir);
            clone_market_repo(&url, &dir)?;
            Ok(MarketSource::Dir(dir))
        }
    }
}

/// 清理某来源的浏览缓存
fn clean_market_browse_copy(source: &str) {
    let Ok(kind) = classify_market_source(source) else { return };
    let name = match kind {
        MarketSource::Dir(_) => return,
        MarketSource::JsonUrl(url) => format!("src-{}.json", short_hash(&url)),
        MarketSource::Git(url) => format!("src-{}", short_hash(&url)),
    };
    if let Ok(root) = market_browse_cache_dir() {
        let _ = fs::remove_dir_all(root.join(&name));
        let _ = fs::remove_file(root.join(&name));
    }
}

/// 稳定短 hash（缓存目录名，非安全用途）
fn short_hash(s: &str) -> String {
    // FNV-1a 64bit
    let mut h: u64 = 0xcbf29ce484222325;
    for b in s.as_bytes() {
        h ^= *b as u64;
        h = h.wrapping_mul(0x100000001b3);
    }
    format!("{:016x}", h)
}

/// HTTP 拉取 marketplace.json（直连优先，失败走系统代理兜底；与 MCP 详情同策略）
fn http_get_market_json(url: &str) -> Result<String> {
    let fetch = |client: reqwest::blocking::Client| -> Result<String> {
        let resp = client
            .get(url)
            .timeout(std::time::Duration::from_secs(20))
            .send()
            .with_context(|| format!("请求失败: {}", url))?;
        let status = resp.status();
        let text = resp.text().with_context(|| "读取响应失败")?;
        if !status.is_success() {
            bail!("HTTP {} for {}", status, url);
        }
        Ok(text)
    };
    let direct = reqwest::blocking::Client::builder()
        .no_proxy()
        .build()
        .context("构建 HTTP 客户端失败")?;
    match fetch(direct) {
        Ok(t) => Ok(t),
        Err(e) => {
            let via_proxy = reqwest::blocking::Client::builder()
                .build()
                .context("构建 HTTP 客户端失败")?;
            fetch(via_proxy).with_context(|| format!("直连失败（{}），系统代理亦失败", e))
        }
    }
}

/// git clone --depth 1（120s 超时树杀，与 plugin CLI 同一超时策略）
fn clone_market_repo(url: &str, dir: &Path) -> Result<()> {
    let mut cmd = std::process::Command::new("git");
    cmd.arg("clone").arg("--depth").arg("1").arg(url).arg(dir);
    run_command_with_timeout(&mut cmd, std::time::Duration::from_secs(120))
        .with_context(|| format!("git clone 失败: {}（GitHub 源可能需要代理）", url))?;
    Ok(())
}

/// get_plugin_marketplaces 的可注入版本（测试：base_dir + installed 表 + 自定义市场 + 推荐清单）
pub(crate) fn get_plugin_marketplaces_in(
    base_dir: &Path,
    installed: &std::collections::HashMap<String, bool>,
    custom_markets: &[CustomMarket],
    presets: &[MarketPreset],
) -> Result<Vec<MarketplaceInfo>> {
    // 热度：Claude 自己维护的插件目录缓存（unique_installs，键为 name@marketplace）
    let installs = load_plugin_installs(base_dir);

    let mut result: Vec<MarketplaceInfo> = Vec::new();

    // 1. Claude 已注册市场
    let file = base_dir.join("plugins").join("known_marketplaces.json");
    if let Ok(content) = fs::read_to_string(&file) {
        let data: serde_json::Value =
            serde_json::from_str(&content).context("Failed to parse known_marketplaces.json")?;
        if let Some(map) = data.as_object() {
            let mut names: Vec<&String> = map.keys().collect();
            names.sort_by(|a, b| a.to_lowercase().cmp(&b.to_lowercase()));
            for name in names {
                let entry = &map[name.as_str()];
                let install_location = entry
                    .get("installLocation")
                    .and_then(|v| v.as_str())
                    .unwrap_or("")
                    .to_string();

                let mut info = MarketplaceInfo {
                    name: name.clone(),
                    install_location: install_location.clone(),
                    last_updated: entry
                        .get("lastUpdated")
                        .and_then(|v| v.as_str())
                        .map(|s| s.to_string()),
                    plugins: Vec::new(),
                    load_error: None,
                    origin: "registered".to_string(),
                    registered: true,
                    source: None,
                    description: None,
                    pinned: false,
                    order: None,
                    icon: None,
                };

                match read_marketplace_json(Path::new(&install_location)) {
                    Ok(mp) => {
                        info.plugins = parse_marketplace_plugins(
                            &mp,
                            name,
                            Some(Path::new(&install_location)),
                            base_dir,
                            installed,
                            &installs,
                        );
                    }
                    Err(e) => {
                        info.load_error = Some(e.to_string());
                    }
                }
                result.push(info);
            }
        }
    }

    // 2. 用户添加市场：已注册的合并（origin=user + source），未注册的走浏览缓存
    let mut customs: Vec<&CustomMarket> = custom_markets.iter().collect();
    customs.sort_by(|a, b| a.name.to_lowercase().cmp(&b.name.to_lowercase()));
    for custom in customs {
        if let Some(existing) = result
            .iter_mut()
            .find(|m| m.name.to_lowercase() == custom.name.to_lowercase())
        {
            existing.origin = "user".to_string();
            existing.source = Some(custom.source.clone());
            continue;
        }
        let mut info = MarketplaceInfo {
            name: custom.name.clone(),
            install_location: custom.source.clone(),
            last_updated: None,
            plugins: Vec::new(),
            load_error: None,
            origin: "user".to_string(),
            registered: false,
            source: Some(custom.source.clone()),
            description: None,
            pinned: false,
            order: None,
            icon: None,
        };
        match market_browse_local_copy(&custom.source) {
            Ok(MarketSource::Dir(root)) => {
                let mtime = fs::metadata(&root)
                    .and_then(|m| m.modified())
                    .ok()
                    .and_then(|t| t.duration_since(std::time::UNIX_EPOCH).ok())
                    .and_then(|d| chrono::DateTime::from_timestamp(d.as_secs() as i64, 0))
                    .map(|dt| dt.to_rfc3339());
                info.last_updated = mtime.filter(|s| !s.is_empty());
                match read_marketplace_json(&root) {
                    Ok(mp) => {
                        // json 文件缓存没有插件源码目录（相对 source 不探测，防按 CWD 误扫）
                        let has_repo = root.is_dir()
                            && root
                                .extension()
                                .map(|e| e != "json")
                                .unwrap_or(true);
                        info.plugins = parse_marketplace_plugins(
                            &mp,
                            &custom.name,
                            if has_repo { Some(&root) } else { None },
                            base_dir,
                            installed,
                            &installs,
                        );
                    }
                    Err(e) => info.load_error = Some(e.to_string()),
                }
            }
            Ok(_) => unreachable!("market_browse_local_copy 只返回 Dir"),
            Err(e) => info.load_error = Some(e.to_string()),
        }
        result.push(info);
    }

    // 3. 推荐市场（OSS 清单）：过滤已添加/已注册的，只列元数据（添加前不拉取）
    for preset in presets {
        let exists = result
            .iter()
            .any(|m| m.name.to_lowercase() == preset.name.to_lowercase());
        if exists {
            continue;
        }
        result.push(MarketplaceInfo {
            name: preset.name.clone(),
            install_location: String::new(),
            last_updated: None,
            plugins: Vec::new(),
            load_error: None,
            origin: "preset".to_string(),
            registered: false,
            source: Some(preset.source.clone()),
            description: preset.description.clone(),
            pinned: preset.pinned,
            order: preset.order,
            icon: preset.icon.clone(),
        });
    }

    // 排序：精选置顶 → order 升序 → 名称
    result.sort_by(|a, b| {
        b.pinned
            .cmp(&a.pinned)
            .then_with(|| a.order.unwrap_or(i64::MAX).cmp(&b.order.unwrap_or(i64::MAX)))
            .then_with(|| a.name.to_lowercase().cmp(&b.name.to_lowercase()))
    });
    Ok(result)
}

/// 解析市场插件条目列表（注册市场与浏览缓存共用）
#[allow(clippy::too_many_arguments)]
fn parse_marketplace_plugins(
    mp: &serde_json::Value,
    market_name: &str,
    market_dir: Option<&Path>,
    base_dir: &Path,
    installed: &std::collections::HashMap<String, bool>,
    installs: &HashMap<String, u64>,
) -> Vec<MarketplacePluginEntry> {
    let mut plugins = Vec::new();
    if let Some(list) = mp.get("plugins").and_then(|v| v.as_array()) {
        for p in list {
            let Some(pname) = p.get("name").and_then(|n| n.as_str()) else {
                continue;
            };
            let full_id = format!("{}@{}", pname, market_name);
            let enabled = installed.get(&full_id).copied();
            plugins.push(MarketplacePluginEntry {
                name: pname.to_string(),
                description: p
                    .get("description")
                    .and_then(|v| v.as_str())
                    .map(|s| s.to_string()),
                version: p.get("version").and_then(|v| v.as_str()).map(|s| s.to_string()),
                // 分类：官方字段 category 优先，缺失时用精选扩展 ccbox.category
                category: p
                    .get("category")
                    .and_then(|v| v.as_str())
                    .or_else(|| {
                        p.get("ccbox")
                            .and_then(|c| c.get("category"))
                            .and_then(|v| v.as_str())
                    })
                    .map(|s| s.trim().to_string())
                    .filter(|s| !s.is_empty()),
                tags: p
                    .get("tags")
                    .and_then(|v| v.as_array())
                    .map(|a| {
                        a.iter()
                            .filter_map(|t| t.as_str())
                            .map(|t| t.to_string())
                            .collect()
                    })
                    .unwrap_or_default(),
                components: parse_components(
                    p,
                    resolve_plugin_dir(market_dir, market_name, p, pname, base_dir).as_deref(),
                ),
                installs: installs.get(&full_id).copied(),
                pinned: p
                    .get("ccbox")
                    .and_then(|c| c.get("pinned"))
                    .and_then(|v| v.as_bool())
                    .unwrap_or(false),
                order: p
                    .get("ccbox")
                    .and_then(|c| c.get("order"))
                    .and_then(|v| v.as_i64()),
                icon: p
                    .get("ccbox")
                    .and_then(|c| c.get("icon"))
                    .and_then(|v| v.as_str())
                    .map(|s| s.trim().to_string())
                    .filter(|s| !s.is_empty()),
                installed: enabled.is_some(),
                enabled,
            });
        }
    }
    // 顺序：已安装置顶（无论精选/热度）→ 精选置顶 → 精选 order 升序 → 安装量降序 → 名称；
    // 与 enabled 解耦（开关不改变顺序）
    plugins.sort_by(|a, b| {
        b.installed
            .cmp(&a.installed)
            .then_with(|| b.pinned.cmp(&a.pinned))
            .then_with(|| a.order.unwrap_or(i64::MAX).cmp(&b.order.unwrap_or(i64::MAX)))
            .then_with(|| b.installs.unwrap_or(0).cmp(&a.installs.unwrap_or(0)))
            .then_with(|| a.name.to_lowercase().cmp(&b.name.to_lowercase()))
    });
    plugins
}

/// 读取市场清单：installLocation 为目录 → `<dir>/.claude-plugin/marketplace.json`；
/// 直接指向 .json 文件（file 源市场）→ 本身。
fn read_marketplace_json(install_location: &Path) -> Result<serde_json::Value> {
    if install_location.extension().map(|e| e == "json").unwrap_or(false) {
        let content = fs::read_to_string(install_location)
            .with_context(|| format!("Failed to read {}", install_location.display()))?;
        return serde_json::from_str(&content)
            .with_context(|| format!("Failed to parse {}", install_location.display()));
    }
    let path = install_location
        .join(".claude-plugin")
        .join("marketplace.json");
    let content = fs::read_to_string(&path)
        .with_context(|| format!("Failed to read {}", path.display()))?;
    serde_json::from_str(&content)
        .with_context(|| format!("Failed to parse {}", path.display()))
}

/// plugins list --json 的轻量解析（只要 id/enabled，容忍字段缺失）
#[derive(serde::Deserialize)]
struct CliPluginLite {
    id: String,
    #[serde(default)]
    enabled: bool,
}

// ---------- 推荐市场（OSS data/plugin-markets.json，与客户端版本解耦、可热更新） ----------
//
// 不把推荐源写死在代码里：启动/打开市场面板时从 OSS 拉取，磁盘缓存供离线兜底。
// OSS 直连（no_proxy，与 installer 一致），data/ 目录独立于官网与更新通道。

/// 推荐市场数据地址（OSS 默认域名；公网自定义域名同路径亦可）
const MARKET_PRESETS_URL: &str = "https://cc-box.oss-cn-beijing.aliyuncs.com/data/plugin-markets.json";
const PRESET_FETCH_TIMEOUT: std::time::Duration = std::time::Duration::from_secs(5);
const PRESET_TTL: std::time::Duration = std::time::Duration::from_secs(6 * 60 * 60);

static PRESET_MEM_CACHE: Mutex<Option<(std::time::Instant, Vec<MarketPreset>)>> = Mutex::new(None);

/// 推荐市场条目（下发给前端「推荐市场」分组）
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MarketPreset {
    pub name: String,
    /// marketplace add 的来源：GitHub owner/repo、marketplace.json URL 或本地目录
    pub source: String,
    #[serde(default)]
    pub description: Option<String>,
    /// 置顶推荐（排在推荐列表最前）
    #[serde(default)]
    pub pinned: bool,
    /// 推荐排序（升序；与 pinned 一起决定顺序）
    #[serde(default)]
    pub order: Option<i64>,
    /// 市场图标（emoji / 短文本 / http(s) 图片 URL）
    #[serde(default)]
    pub icon: Option<String>,
    /// 市场分类（如「官方」「精选」），前端展示用
    #[serde(default)]
    pub category: Option<String>,
}

#[derive(Debug, Deserialize)]
struct MarketPresetFile {
    #[serde(default)]
    markets: Vec<MarketPreset>,
}

/// 解析推荐市场文件：丢弃空名/空来源，按名称（不区分大小写）去重
pub(crate) fn parse_market_presets(json: &str) -> Result<Vec<MarketPreset>> {
    let file: MarketPresetFile = serde_json::from_str(json).context("Failed to parse plugin markets")?;
    let mut seen = std::collections::HashSet::new();
    let mut out = Vec::new();
    for m in file.markets {
        let name = m.name.trim().to_string();
        let source = m.source.trim().to_string();
        if name.is_empty() || source.is_empty() {
            continue;
        }
        if !seen.insert(name.to_lowercase()) {
            continue;
        }
        out.push(MarketPreset {
            name,
            source,
            description: m
                .description
                .map(|d| d.trim().to_string())
                .filter(|d| !d.is_empty()),
            pinned: m.pinned,
            order: m.order,
            icon: m.icon.map(|s| s.trim().to_string()).filter(|s| !s.is_empty()),
            category: m
                .category
                .map(|s| s.trim().to_string())
                .filter(|s| !s.is_empty()),
        });
    }
    Ok(out)
}

fn preset_cache_file() -> Option<PathBuf> {
    get_gui_config_dir()
        .ok()
        .map(|d| d.join("cache").join("plugin-markets.json"))
}

fn fetch_and_cache_presets() -> Result<Vec<MarketPreset>> {
    let client = reqwest::blocking::Client::builder()
        .timeout(PRESET_FETCH_TIMEOUT)
        .no_proxy()
        .build()
        .context("Failed to build HTTP client")?;
    let resp = client
        .get(MARKET_PRESETS_URL)
        .send()
        .context("Failed to fetch plugin markets")?;
    if !resp.status().is_success() {
        bail!("plugin markets returned HTTP {}", resp.status());
    }
    let text = resp.text().context("Failed to read plugin markets body")?;
    let presets = parse_market_presets(&text)?;
    if let Some(path) = preset_cache_file() {
        if let Some(dir) = path.parent() {
            let _ = fs::create_dir_all(dir);
        }
        // 缓存失败不影响本次结果
        let _ = fs::write(&path, &text);
    }
    Ok(presets)
}

/// 推荐市场：内存缓存 → OSS 拉取 → 磁盘缓存（离线兜底）→ 空列表。
/// 返回空列表表示「当前拿不到推荐源」，前端只是不显示推荐分组，不报错。
pub fn get_market_presets() -> Result<Vec<MarketPreset>> {
    if let Ok(guard) = PRESET_MEM_CACHE.lock() {
        if let Some((at, data)) = guard.as_ref() {
            if at.elapsed() < PRESET_TTL {
                return Ok(data.clone());
            }
        }
    }

    let result = match fetch_and_cache_presets() {
        Ok(list) => list,
        Err(e) => {
            log::warn!("[Market] fetch presets failed: {}", e);
            preset_cache_file()
                .and_then(|p| fs::read_to_string(p).ok())
                .and_then(|s| parse_market_presets(&s).ok())
                .unwrap_or_default()
        }
    };

    if let Ok(mut guard) = PRESET_MEM_CACHE.lock() {
        *guard = Some((std::time::Instant::now(), result.clone()));
    }
    Ok(result)
}

pub(crate) fn validate_marketplace_source(source: &str) -> Result<String> {
    let s = source.trim();
    if s.is_empty() || s.len() > 512 || s.contains('\n') || s.contains('\r') || s.contains('\0') {
        bail!("Invalid marketplace source");
    }
    Ok(s.to_string())
}

pub(crate) fn validate_plugin_ref(plugin_ref: &str) -> Result<()> {
    validate_simple_name(plugin_ref)?;
    if !plugin_ref.contains('@') {
        bail!("Plugin reference must be name@marketplace: {:?}", plugin_ref);
    }
    Ok(())
}

pub(crate) fn validate_install_scope(scope: &str) -> Result<()> {
    if !matches!(scope, "user" | "project" | "local") {
        bail!("Invalid install scope: {:?}", scope);
    }
    Ok(())
}

/// project 作用域必须提供项目路径作为 CLI cwd；其他作用域不需要
pub(crate) fn install_scope_cwd(scope: &str, project_path: Option<&str>) -> Result<Option<PathBuf>> {
    if scope == "project" {
        let p = project_path
            .filter(|s| !s.trim().is_empty())
            .context("project scope requires project_path")?;
        Ok(Some(PathBuf::from(p)))
    } else {
        Ok(None)
    }
}

/// 市场是否已注册到 Claude（known_marketplaces.json 键，大小写不敏感）
fn market_registered(name: &str) -> Result<bool> {
    let file = claude_config_dir()?.join("plugins").join("known_marketplaces.json");
    let Ok(content) = fs::read_to_string(&file) else {
        return Ok(false);
    };
    let Ok(v) = serde_json::from_str::<serde_json::Value>(&content) else {
        return Ok(false);
    };
    Ok(v
        .as_object()
        .map(|m| m.keys().any(|k| k.eq_ignore_ascii_case(name)))
        .unwrap_or(false))
}

/// 用户市场列表中按名找来源
fn custom_market_source(name: &str) -> Option<String> {
    get_app_config()
        .ok()?
        .custom_markets
        .unwrap_or_default()
        .into_iter()
        .find(|m| m.name.eq_ignore_ascii_case(name))
        .map(|m| m.source)
}

fn set_custom_markets(list: &[CustomMarket]) -> Result<()> {
    update_app_config(serde_json::json!({ "customMarkets": list }))
}

/// 添加市场：**只入 CC-Box 列表**（可浏览），不注册 Claude。
/// 先拉取浏览副本并校验 marketplace.json，返回清单里的市场名（与 Claude 注册键一致）。
pub fn add_plugin_marketplace(source: &str) -> Result<String> {
    let src = validate_marketplace_source(source)?;
    let root = match market_browse_local_copy(&src)? {
        MarketSource::Dir(p) => p,
        _ => bail!("内部错误：浏览副本应为本地路径"),
    };
    let mp = read_marketplace_json(&root)?;
    let name = mp
        .get("name")
        .and_then(|v| v.as_str())
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .context("marketplace.json 缺少 name 字段")?;
    // 官方精选市场名不可被外部来源抢占（同名会与置顶/不可移除守卫冲突，
    // 且懒注册会用抢占者的 source 执行 marketplace add）
    if name.eq_ignore_ascii_case(CCBOX_MARKET_NAME) {
        bail!("ccbox-market 是 CC-Box 官方精选市场名，不可被其他来源占用");
    }
    let mut list = get_app_config()?.custom_markets.unwrap_or_default();
    if list.iter().any(|m| m.name.eq_ignore_ascii_case(&name)) {
        bail!("市场 {} 已在列表中", name);
    }
    list.push(CustomMarket {
        name: name.clone(),
        source: src,
    });
    set_custom_markets(&list)?;
    invalidate_plugin_market_cache();
    log::info!("[PluginMarket] added to browse list: {} ({})", name, source);
    Ok(name)
}

/// 移除/取消注册（两个独立动作，前端两个按钮）：
/// - also_unregister=false：**移除该市场** = 仅从 CC-Box 列表删除（+清理浏览缓存），
///   不动 Claude 注册（registered-only 市场不在列表，报错）。
/// - also_unregister=true：**取消注册** = 仅 `claude plugin marketplace remove`
///   （列表条目保留，仍可浏览；未注册报错）。
/// CC-Box 官方精选市场名：不可移除/卸载
pub(crate) const CCBOX_MARKET_NAME: &str = "ccbox-market";

pub fn remove_plugin_marketplace(name: &str, also_unregister: bool) -> Result<()> {
    validate_simple_name(name)?;
    // ccbox-market 是 CC-Box 官方精选来源：不可移除也不可卸载
    if name.eq_ignore_ascii_case(CCBOX_MARKET_NAME) {
        bail!("ccbox-market 是 CC-Box 官方精选市场，不可移除");
    }
    if also_unregister {
        if !market_registered(name)? {
            bail!("市场 {} 未注册", name);
        }
        let res =
            run_claude_command_argv(&["plugin", "marketplace", "remove", name], None).map(|_| ());
        invalidate_plugin_market_cache();
        return res;
    }
    let mut list = get_app_config()?.custom_markets.unwrap_or_default();
    let removed: Vec<CustomMarket> = list
        .iter()
        .filter(|m| m.name.eq_ignore_ascii_case(name))
        .cloned()
        .collect();
    if removed.is_empty() {
        bail!("市场 {} 不在列表中", name);
    }
    list.retain(|m| !m.name.eq_ignore_ascii_case(name));
    set_custom_markets(&list)?;
    for m in &removed {
        clean_market_browse_copy(&m.source);
    }
    invalidate_plugin_market_cache();
    Ok(())
}

/// 注册该市场（「卸载市场」的逆操作）：显式 `claude plugin marketplace add <source>`。
/// 市场须在 CC-Box 列表中（来源可查），已注册时幂等成功。
pub fn register_plugin_marketplace(name: &str) -> Result<()> {
    validate_simple_name(name)?;
    if market_registered(name)? {
        return Ok(());
    }
    let source = custom_market_source(name)
        .context(format!("市场 {} 不在列表中，无法获取来源", name))?;
    run_claude_command_argv(&["plugin", "marketplace", "add", &source], None)
        .with_context(|| format!("注册市场 {} 失败", name))?;
    // 注册成功后浏览切到 Claude 克隆，CC-Box 浏览缓存即刻清理
    // （失败保留缓存——下次浏览不用重新 clone）
    clean_market_browse_copy(&source);
    invalidate_plugin_market_cache();
    Ok(())
}

/// 刷新市场：已注册 → Claude `marketplace update`（git pull）；
/// 未注册（仅列表）→ 强制重拉 CC-Box 浏览缓存。
pub fn refresh_plugin_marketplace(name: &str) -> Result<()> {
    validate_simple_name(name)?;
    let res = if market_registered(name)? {
        run_claude_command_argv(&["plugin", "marketplace", "update", name], None).map(|_| ())
    } else if let Some(source) = custom_market_source(name) {
        refresh_market_browse_copy(&source).map(|_| ())
    } else {
        bail!("市场 {} 不存在", name);
    };
    invalidate_plugin_market_cache();
    res?;
    Ok(())
}

fn install_plugin_lifecycle(
    action: &str,
    plugin_ref: &str,
    scope: &str,
    project_path: Option<&str>,
    market_source: Option<&str>,
) -> Result<()> {
    validate_plugin_ref(plugin_ref)?;
    validate_install_scope(scope)?;
    let cwd = install_scope_cwd(scope, project_path)?;
    // 安装懒注册：市场未注册时先 add（来源 = 前端传参或用户市场列表）
    if action == "install" {
        if let Some(market) = plugin_ref.rsplit('@').next() {
            if !market_registered(market)? {
                let source = market_source
                    .map(|s| s.to_string())
                    .or_else(|| custom_market_source(market))
                    .context(format!(
                        "市场 {} 未注册且无来源可自动添加；请先将该市场加入列表",
                        market
                    ))?;
                run_claude_command_argv(&["plugin", "marketplace", "add", &source], None)?;
            }
        }
    }
    let res = run_claude_command_argv(
        &["plugin", action, plugin_ref, "-s", scope],
        cwd.as_deref(),
    );
    // 无论成败都失效缓存：CLI 可能已部分写入
    invalidate_plugins_list_cache();
    invalidate_plugin_market_cache();
    res.with_context(|| format!("Failed to {} '{}'", action, plugin_ref))?;
    Ok(())
}

pub fn install_plugin(
    plugin_ref: &str,
    scope: &str,
    project_path: Option<&str>,
    market_source: Option<&str>,
) -> Result<()> {
    install_plugin_lifecycle("install", plugin_ref, scope, project_path, market_source)
}

pub fn uninstall_plugin(plugin_ref: &str, scope: &str, project_path: Option<&str>) -> Result<()> {
    install_plugin_lifecycle("uninstall", plugin_ref, scope, project_path, None)
}

pub fn update_plugin(plugin_ref: &str, scope: &str, project_path: Option<&str>) -> Result<()> {
    install_plugin_lifecycle("update", plugin_ref, scope, project_path, None)
}

