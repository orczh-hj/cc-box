//! Store 模块
//! Claude Code 原生数据读取

use anyhow::{bail, Context, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
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
pub struct HookItem {
    pub event: String,
    pub matcher: Option<String>,
    pub command: Option<String>,
    #[serde(rename = "hookType")]
    pub hook_type: Option<String>,
    pub source: ConfigSource,
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
    dirs::home_dir()
        .map(|h| h.join(".claude"))
        .context("Home directory not found")
}

/// serde 默认值=true（用于 enabled 字段向后兼容）
fn default_true() -> bool {
    true
}

/// 测试注入：覆盖 GUI 配置目录（隔离真实 ~/.cc-box）。remote_state 的 rpc
/// 全链路测试经真实 update_app_config 落盘，曾把各开发者的 theme 污染成 dark
#[cfg(test)]
pub(crate) static GUI_CONFIG_DIR_OVERRIDE: std::sync::LazyLock<
    parking_lot::Mutex<Option<PathBuf>>,
> = std::sync::LazyLock::new(|| parking_lot::Mutex::new(None));

/// 获取 GUI 配置目录
fn get_gui_config_dir() -> Result<PathBuf> {
    #[cfg(test)]
    if let Some(dir) = GUI_CONFIG_DIR_OVERRIDE.lock().clone() {
        return Ok(dir);
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
    let home = dirs::home_dir().context("Home directory not found")?;

    // 用户级 ~/.claude/settings.json
    let user_settings = home.join(".claude").join("settings.json");
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

    // 用户级 ~/.claude/skills/<name>/SKILL.md
    let home = dirs::home_dir().context("Home directory not found")?;
    let user_skills_dir = home.join(".claude").join("skills");
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

    // 用户级 ~/.claude/agents/*.md
    let home = dirs::home_dir().context("Home directory not found")?;
    let user_agents_dir = home.join(".claude").join("agents");
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

    // 用户级 ~/.claude/agents/*.md
    let home = dirs::home_dir().context("Home directory not found")?;
    let user_agents_dir = home.join(".claude").join("agents");
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

fn read_hooks_config(project_path: &str) -> Result<Vec<HookItem>> {
    let mut result = Vec::new();

    let home = dirs::home_dir().context("Home directory not found")?;

    let settings_paths: Vec<(PathBuf, &str)> = vec![
        (home.join(".claude").join("settings.json"), "user"),
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
                                    result.push(HookItem {
                                        event: event.to_string(),
                                        matcher: hook
                                            .get("matcher")
                                            .and_then(|v| v.as_str())
                                            .map(|s| s.to_string()),
                                        command: hook
                                            .get("command")
                                            .and_then(|v| v.as_str())
                                            .map(|s| s.to_string())
                                            .or_else(|| {
                                                hook.get("hooks")
                                                    .and_then(|h| h.as_array())
                                                    .and_then(|a| a.first())
                                                    .and_then(|h| h.get("command"))
                                                    .and_then(|v| v.as_str())
                                                    .map(|s| s.to_string())
                                            }),
                                        hook_type: hook
                                            .get("type")
                                            .and_then(|v| v.as_str())
                                            .map(|s| s.to_string()),
                                        source: ConfigSource {
                                            source_type: source_type.to_string(),
                                            label: source_type.to_string(),
                                            path: Some(path.to_string_lossy().to_string()),
                                        },
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
            // 在 frontmatter 中查找 description 字段
            for line in lines.iter().skip(1) {
                if line.trim() == "---" {
                    break;
                }
                if let Some(desc) = line.strip_prefix("description:") {
                    let desc = desc.trim();
                    if !desc.is_empty() {
                        return Some(desc.to_string());
                    }
                }
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

    for line in lines.iter().take(end_idx).skip(1) {
        let trimmed = line.trim();
        if let Some(val) = trimmed.strip_prefix("name:") {
            let val = val.trim();
            if !val.is_empty() {
                name = Some(val.to_string());
            }
        } else if let Some(val) = trimmed.strip_prefix("description:") {
            let val = val.trim();
            if !val.is_empty() {
                description = Some(val.to_string());
            }
        } else if let Some(val) = trimmed.strip_prefix("model:") {
            let val = val.trim();
            if !val.is_empty() {
                model = Some(val.to_string());
            }
        }
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
            // 从 frontmatter 提取 description
            for line in lines.iter().take(end).skip(1) {
                let trimmed = line.trim();
                if let Some(val) = trimmed.strip_prefix("description:") {
                    let val = val.trim();
                    if !val.is_empty() {
                        return Some(val.to_string());
                    }
                }
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

    Ok(agents)
}

/// 获取所有 Skills（包括 project、user、plugin）
/// 与 CLI 源码 loadSkillsDir.ts 一致：从 skills/name/SKILL.md frontmatter 解析 description
pub fn get_all_skills(project_path: &str) -> Result<Vec<SkillInfo>> {
    let mut skills = Vec::new();

    // 1. 从项目目录读取 skills（与源码 loadSkillsFromSkillsDir 一致：只支持 dir/SKILL.md 格式）
    let project_skills_dir = PathBuf::from(project_path).join(".claude").join("skills");
    if project_skills_dir.exists() {
        if let Ok(entries) = fs::read_dir(&project_skills_dir) {
            for entry in entries.flatten() {
                if entry.path().is_dir() {
                    let skill_file = entry.path().join("SKILL.md");
                    if skill_file.exists() {
                        let name = entry.file_name().to_string_lossy().to_string();
                        let description = parse_skill_description(&skill_file);
                        skills.push(SkillInfo {
                            name: name.clone(),
                            display_name: name.clone(),
                            description,
                            source_type: "project".to_string(),
                            source_label: "Project".to_string(),
                            invoke_format: format!("/{}", name),
                            enabled: true,
                        });
                    }
                }
            }
        }
    }

    // 2. 从用户目录读取 skills
    let home = dirs::home_dir().context("Home directory not found")?;
    let user_skills_dir = home.join(".claude").join("skills");
    if user_skills_dir.exists() {
        if let Ok(entries) = fs::read_dir(&user_skills_dir) {
            for entry in entries.flatten() {
                if entry.path().is_dir() {
                    let skill_file = entry.path().join("SKILL.md");
                    if skill_file.exists() {
                        let name = entry.file_name().to_string_lossy().to_string();
                        let description = parse_skill_description(&skill_file);
                        skills.push(SkillInfo {
                            name: name.clone(),
                            display_name: name.clone(),
                            description,
                            source_type: "user".to_string(),
                            source_label: "User".to_string(),
                            invoke_format: format!("/{}", name),
                            enabled: true,
                        });
                    }
                }
            }
        }
    }

    // 2b. 从 disabled 备份目录读取用户级 skills（原位灰显）
    if let Ok(disabled_skills_dir) = get_disabled_subdir("skills") {
        if disabled_skills_dir.exists() {
            if let Ok(entries) = fs::read_dir(&disabled_skills_dir) {
                for entry in entries.flatten() {
                    if entry.path().is_dir() {
                        let skill_file = entry.path().join("SKILL.md");
                        if skill_file.exists() {
                            let name = entry.file_name().to_string_lossy().to_string();
                            let description = parse_skill_description(&skill_file);
                            skills.push(SkillInfo {
                                name: name.clone(),
                                display_name: name.clone(),
                                description,
                                source_type: "user".to_string(),
                                source_label: "User (Disabled)".to_string(),
                                invoke_format: format!("/{}", name),
                                enabled: false,
                            });
                        }
                    }
                }
            }
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

/// 执行 claude 命令并获取输出（设置 git-bash 环境）
fn run_claude_command(args: &str) -> Result<String> {
    // Windows 上需要设置 git-bash 环境
    let mut cmd = std::process::Command::new("claude");
    cmd.args(args.split_whitespace());

    // 设置 git-bash 环境
    if cfg!(target_os = "windows") {
        // 尝试从环境变量获取
        if let Ok(git_bash_path) = std::env::var("CLAUDE_CODE_GIT_BASH_PATH") {
            if Path::new(&git_bash_path).exists() {
                cmd.env("CLAUDE_CODE_GIT_BASH_PATH", git_bash_path);
            }
        } else {
            // 检测 git-bash 路径
            let git_bash_path = detect_git_bash_path();
            if let Some(path) = git_bash_path {
                cmd.env("CLAUDE_CODE_GIT_BASH_PATH", path);
            }
        }
    }

    // Windows 上禁止子进程创建控制台窗口
    crate::platform::configure_command(&mut cmd);

    let output = cmd.output().context("Failed to run claude command")?;

    if output.status.success() {
        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    } else {
        // 即使失败也返回输出，可能包含有用信息
        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    }
}

/// 严格版的 claude 命令执行：exit code != 0 时返回 Err（含 stderr）
/// 用于 plugin enable/disable 等需要明确成败判断的场景
fn run_claude_command_strict(args: &str) -> Result<String> {
    let mut cmd = std::process::Command::new("claude");
    cmd.args(args.split_whitespace());

    if cfg!(target_os = "windows") {
        if let Ok(git_bash_path) = std::env::var("CLAUDE_CODE_GIT_BASH_PATH") {
            if Path::new(&git_bash_path).exists() {
                cmd.env("CLAUDE_CODE_GIT_BASH_PATH", git_bash_path);
            }
        } else {
            let git_bash_path = detect_git_bash_path();
            if let Some(path) = git_bash_path {
                cmd.env("CLAUDE_CODE_GIT_BASH_PATH", path);
            }
        }
    }

    crate::platform::configure_command(&mut cmd);

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
    // 使用 CLI 命令获取 plugin 信息
    let output = run_claude_command("plugins list --json")?;
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
    // Step 1: cache 路径存在
    let original = PathBuf::from(original_path);
    if original.exists() {
        return Some(original_path.to_string());
    }

    // Step 2: 回退到 marketplace source 路径
    resolve_marketplace_plugin_path(plugin_id)
}

/// 从 known_marketplaces.json + marketplace.json 解析 plugin 的实际路径
/// plugin_id 格式: "paper-tool@orczh" → 找 orczh marketplace → 找 paper-tool 的 source
pub(crate) fn resolve_marketplace_plugin_path(plugin_id: &str) -> Option<String> {
    let home = dirs::home_dir()?;
    let marketplaces_file = home.join(".claude").join("plugins").join("known_marketplaces.json");

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
fn validate_simple_name(name: &str) -> Result<()> {
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

// ---------- Skill（目录移动）----------

/// 切换用户级 skill 启用状态（公开 API）
pub fn set_skill_enabled(name: &str, enabled: bool) -> Result<()> {
    let home = dirs::home_dir().context("Home directory not found")?;
    let claude_skills = home.join(".claude").join("skills");
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
        if !dst.exists() {
            bail!("Skill '{}' is not disabled (no backup found)", name);
        }
        if src.exists() {
            bail!("Skill '{}' already exists in active directory (conflict)", name);
        }
        (dst, src, "enable")
    } else {
        if !src.exists() {
            bail!("Skill '{}' not found in active directory", name);
        }
        if dst.exists() {
            bail!("Skill '{}' already disabled (backup exists)", name);
        }
        (src, dst, "disable")
    };

    if let Err(e) = fs::rename(&from, &to) {
        // Windows：skill 目录被运行中的进程占用（典型：Claude Code 会话已加载
        // 该 skill，目录句柄未共享删除）→ 给出可行动的提示而非裸 os error
        if is_dir_busy_error(&e) {
            bail!(
                "Skill '{}' is in use (directory locked, likely by a running Claude \
                 Code session); close the session and retry",
                name
            );
        }
        // 跨卷时 rename 可能失败，回退到 copy + remove
        if e.raw_os_error() == Some(17) || e.raw_os_error() == Some(18) {
            copy_dir_recursive(&from, &to)
                .with_context(|| format!("Failed to {} skill '{}'", op, name))?;
            if from.is_dir() {
                fs::remove_dir_all(&from)
                    .with_context(|| format!("Failed to {} skill '{}'", op, name))?;
            } else {
                fs::remove_file(&from)
                    .with_context(|| format!("Failed to {} skill '{}'", op, name))?;
            }
        } else {
            // 消息带上底层原因：错误经 anyhow to_string 只显最外层 context，
            // 不带 (e) 时 os error（如拒绝访问的具体码）在链路中丢失
            let cause = e.to_string();
            return Err(e).with_context(move || {
                format!("Failed to {} skill '{}' ({})", op, name, cause)
            });
        }
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
    let home = dirs::home_dir().context("Home directory not found")?;
    let claude_agents = home.join(".claude").join("agents");
    let disabled_agents = get_disabled_subdir("agents")?;
    set_agent_enabled_in(&claude_agents, &disabled_agents, name, enabled)
}

pub(crate) fn set_agent_enabled_in(
    claude_base: &Path,
    disabled_base: &Path,
    name: &str,
    enabled: bool,
) -> Result<()> {
    validate_simple_name(name)?;
    let filename = format!("{}.md", name);
    let src = claude_base.join(&filename);
    let dst = disabled_base.join(&filename);

    let (from, to, op) = if enabled {
        if !dst.exists() {
            bail!("Agent '{}' is not disabled", name);
        }
        if src.exists() {
            bail!("Agent '{}' already exists (conflict)", name);
        }
        (dst, src, "enable")
    } else {
        if !src.exists() {
            bail!("Agent '{}' not found", name);
        }
        if dst.exists() {
            bail!("Agent '{}' already disabled", name);
        }
        (src, dst, "disable")
    };

    fs::rename(&from, &to).with_context(|| format!("Failed to {} agent '{}'", op, name))?;
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

/// 原子写 JSON：先写 .tmp 再 rename
fn write_json_atomic(path: &Path, value: &serde_json::Value) -> Result<()> {
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
    run_claude_command_strict(&args)
        .with_context(|| format!("Failed to {} plugin '{}'", action, plugin_id))?;
    Ok(())
}

