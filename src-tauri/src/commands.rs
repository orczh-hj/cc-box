//! Tauri Commands 模块
//! 定义所有 IPC 命令

use anyhow::Result;
use serde::{Deserialize, Serialize};
use tauri::AppHandle;

use crate::checks::CheckResult;
use crate::pty::get_pty_manager;
use crate::store::{
    AgentInfo, AppConfig, HomeData, McpServerInfo, PluginInfo, Project, ProjectConfig,
    SessionDetails, SessionInfo, SessionSearchResult, SkillInfo,
};
use crate::providers::{
    Provider, ProvidersConfig, ProviderMeta, ImportResult, TestConnectionResult,
};

// ==================== PTY Commands ====================

#[derive(Debug, Deserialize)]
pub struct PtySpawnOptions {
    cwd: String,
    #[serde(rename = "type")]
    pty_type: String, // "claude" | "shell"
    cols: Option<u16>,
    rows: Option<u16>,
    args: Option<Vec<String>>,
}

#[derive(Debug, Serialize)]
pub struct PtySpawnResult {
    id: String,
    #[serde(rename = "type")]
    pty_type: String,
    cwd: String,
}

/// 启动 PTY
#[tauri::command]
pub async fn pty_spawn(
    options: PtySpawnOptions,
    _app_handle: AppHandle,
) -> Result<Option<PtySpawnResult>, String> {
    let cols = options.cols.unwrap_or(80);
    let rows = options.rows.unwrap_or(24);

    let manager = get_pty_manager().ok_or_else(|| "PTY manager not initialized".to_string())?;

    let result = if options.pty_type == "shell" {
        manager.spawn_shell(&options.cwd, cols, rows)
    } else {
        manager.spawn_claude(&options.cwd, cols, rows, options.args)
    };

    match result {
        Ok(info) => Ok(Some(PtySpawnResult {
            id: info.id,
            pty_type: info.pty_type,
            cwd: info.cwd,
        })),
        Err(e) => Err(e.to_string()),
    }
}

/// 写入 PTY 输入
#[tauri::command]
pub async fn pty_input(id: String, data: String) -> Result<bool, String> {
    let manager = get_pty_manager().ok_or_else(|| "PTY manager not initialized".to_string())?;

    manager
        .write(&id, &data)
        .map(|_| true)
        .map_err(|e| e.to_string())
}

/// resize PTY
#[tauri::command]
pub async fn pty_resize(id: String, cols: u16, rows: u16) -> Result<bool, String> {
    let manager = get_pty_manager().ok_or_else(|| "PTY manager not initialized".to_string())?;

    manager
        .resize(&id, cols, rows)
        .map(|_| true)
        .map_err(|e| e.to_string())
}

/// 杀掉 PTY
#[tauri::command]
pub async fn pty_kill(id: String) -> Result<bool, String> {
    let manager = get_pty_manager().ok_or_else(|| "PTY manager not initialized".to_string())?;

    manager.kill(&id).map(|_| true).map_err(|e| e.to_string())
}

/// 杀掉所有 PTY
#[tauri::command]
pub async fn pty_kill_all() -> Result<(), String> {
    let manager = get_pty_manager().ok_or_else(|| "PTY manager not initialized".to_string())?;

    manager.kill_all();
    Ok(())
}

// ==================== Store Commands ====================

/// 获取环境检查结果
#[tauri::command]
pub async fn get_check_results() -> Result<Vec<CheckResult>, String> {
    Ok(crate::get_check_results())
}

/// 重新运行环境检查
#[tauri::command]
pub async fn run_checks() -> Result<Vec<CheckResult>, String> {
    Ok(crate::rerun_checks())
}

/// 一次获取首页数据（项目列表 + 近期会话），避免重复 IO
#[tauri::command]
pub async fn get_home_data(
    project_limit: Option<usize>,
    session_limit: Option<usize>,
) -> Result<HomeData, String> {
    let project_limit = project_limit.unwrap_or(12);
    let session_limit = session_limit.unwrap_or(20);
    crate::store::get_home_data(project_limit, session_limit).map_err(|e| e.to_string())
}

/// 获取项目列表（支持分页）
#[tauri::command]
pub async fn get_projects(
    limit: Option<usize>,
    offset: Option<usize>,
) -> Result<Vec<Project>, String> {
    crate::store::get_projects(limit, offset).map_err(|e| e.to_string())
}

/// 获取项目信息
#[tauri::command]
pub async fn get_project_info(path: String) -> Result<Option<Project>, String> {
    crate::store::get_project_info(&path).map_err(|e| e.to_string())
}

/// 获取会话列表
#[tauri::command]
pub async fn get_sessions(
    project_path: String,
    limit: Option<usize>,
    offset: Option<usize>,
) -> Result<Vec<SessionInfo>, String> {
    let limit = limit.unwrap_or(20);
    let offset = offset.unwrap_or(0);
    crate::store::get_sessions(&project_path, limit, offset).map_err(|e| e.to_string())
}

/// 获取会话总数
#[tauri::command]
pub async fn get_session_count(project_path: String) -> Result<usize, String> {
    crate::store::get_session_count(&project_path).map_err(|e| e.to_string())
}

/// 获取所有项目的近期会话
#[tauri::command]
pub async fn get_all_recent_sessions(limit: Option<usize>) -> Result<Vec<SessionInfo>, String> {
    let limit = limit.unwrap_or(20);
    crate::store::get_all_recent_sessions(limit).map_err(|e| e.to_string())
}

/// 获取会话详情
#[tauri::command]
pub async fn get_session_details(
    project_path: String,
    session_id: String,
) -> Result<Option<SessionDetails>, String> {
    crate::store::get_session_details(&project_path, &session_id).map_err(|e| e.to_string())
}

/// 搜索会话消息内容
#[tauri::command]
pub async fn search_session_messages(
    project_path: String,
    query: String,
    limit: Option<usize>,
) -> Result<Vec<SessionSearchResult>, String> {
    let limit = limit.unwrap_or(20);
    crate::store::search_session_messages(&project_path, &query, limit).map_err(|e| e.to_string())
}

/// 获取应用配置
#[tauri::command]
pub async fn get_app_config() -> Result<AppConfig, String> {
    crate::store::get_app_config().map_err(|e| e.to_string())
}

/// 更新应用配置
#[tauri::command]
pub async fn update_app_config(updates: serde_json::Value) -> Result<(), String> {
    crate::store::update_app_config(updates).map_err(|e| e.to_string())
}

/// 获取系统 locale（BCP-47 标签，如 "zh-CN"、"en-US"）
///
/// 通过 `sys-locale` crate 调用平台原生 API（Windows GetUserDefaultLocaleName /
/// macOS NSLocale currentLocale / POSIX setlocale），比前端的 `navigator.language`
/// 在 Windows WebView2 中更可靠。
#[tauri::command]
pub async fn get_system_locale() -> Result<String, String> {
    let locale = sys_locale::get_locale().unwrap_or_else(|| "en".to_string());
    Ok(locale)
}

/// 获取默认 Claude 选项
#[tauri::command]
pub async fn get_default_claude_options() -> Result<crate::store::DefaultClaudeOptions, String> {
    crate::store::get_default_claude_options().map_err(|e| e.to_string())
}

/// 保存默认 Claude 选项
#[tauri::command]
pub async fn save_default_claude_options(
    options: crate::store::DefaultClaudeOptions,
) -> Result<(), String> {
    crate::store::save_default_claude_options(options).map_err(|e| e.to_string())
}

/// 保存最近打开项目
#[tauri::command]
pub async fn save_last_project(path: String) -> Result<(), String> {
    crate::store::save_last_project(&path).map_err(|e| e.to_string())
}

/// 在文件管理器中打开
#[tauri::command]
pub async fn open_in_file_manager(path: String) -> Result<(), String> {
    crate::platform::open_in_file_manager(&path)
}

/// 测试命令（验证通信）
#[tauri::command]
pub async fn test_communication(message: String) -> Result<String, String> {
    Ok(format!("Received: {}", message))
}

/// 获取项目配置
#[tauri::command]
pub async fn get_project_config(project_path: String) -> Result<ProjectConfig, String> {
    crate::store::get_project_config(&project_path).map_err(|e| e.to_string())
}

/// 获取所有 Agents（包括 built-in、plugin、user、project）
#[tauri::command]
pub async fn get_all_agents(project_path: String) -> Result<Vec<AgentInfo>, String> {
    crate::store::get_all_agents(&project_path).map_err(|e| e.to_string())
}

/// 获取所有 Skills（包括 project、user、plugin）
#[tauri::command]
pub async fn get_all_skills(project_path: String) -> Result<Vec<SkillInfo>, String> {
    crate::store::get_all_skills(&project_path).map_err(|e| e.to_string())
}

/// 获取所有 MCP Servers（包括 plugin 和配置的）
#[tauri::command]
pub async fn get_all_mcp_servers(project_path: String) -> Result<Vec<McpServerInfo>, String> {
    crate::store::get_all_mcp_servers(&project_path).map_err(|e| e.to_string())
}

/// 获取所有 Plugins（使用 --json）
#[tauri::command]
pub async fn get_all_plugins(project_path: String) -> Result<Vec<PluginInfo>, String> {
    crate::store::get_all_plugins(&project_path).map_err(|e| e.to_string())
}

/// 切换用户级 Skill 启用状态（移动目录到 ~/.cc-box/disabled/skills/）
#[tauri::command]
pub async fn set_skill_enabled(name: String, enabled: bool) -> Result<(), String> {
    crate::store::set_skill_enabled(&name, enabled).map_err(|e| e.to_string())
}

/// 切换用户级 Agent 启用状态（移动文件到 ~/.cc-box/disabled/agents/）
#[tauri::command]
pub async fn set_agent_enabled(name: String, enabled: bool) -> Result<(), String> {
    crate::store::set_agent_enabled(&name, enabled).map_err(|e| e.to_string())
}

/// 切换用户级 MCP Server 启用状态（剪切 ~/.claude.json::mcpServers.<name> 条目）
#[tauri::command]
pub async fn set_mcp_server_enabled(name: String, enabled: bool) -> Result<(), String> {
    crate::store::set_mcp_server_enabled(&name, enabled).map_err(|e| e.to_string())
}

/// 切换 Plugin 启用状态（调用 claude plugin enable/disable）
#[tauri::command]
pub async fn set_plugin_enabled(plugin_id: String, enabled: bool) -> Result<(), String> {
    crate::store::set_plugin_enabled(&plugin_id, enabled).map_err(|e| e.to_string())
}

/// 获取 MCP Server 详情（通过 MCP 协议）
#[tauri::command]
pub async fn get_mcp_server_detail(
    project_path: String,
    server_name: String,
    force_refresh: bool,
) -> Result<Option<crate::mcp::McpServerDetail>, String> {
    log::info!(
        "[MCP] get_mcp_server_detail called: name={}, force_refresh={}",
        server_name,
        force_refresh
    );

    // 先从 store 获取 server 的 URL、command 和 headers
    let servers = crate::store::get_all_mcp_servers(&project_path).map_err(|e| {
        log::error!("[MCP] get_all_mcp_servers failed: {}", e);
        e.to_string()
    })?;
    let server = servers.iter().find(|s| s.name == server_name);

    if server.is_none() {
        log::warn!("[MCP] Server '{}' not found in config", server_name);
        return Ok(None);
    }

    let server = server.unwrap();
    log::info!(
        "[MCP] Server '{}' config: type={:?}, url={:?}, command={:?}, args={:?}",
        server_name,
        server.server_type,
        server.url,
        server.command,
        server.args
    );

    let url = server.url.as_deref();
    let command = server.command.as_deref();
    let args = server.args.as_ref();
    let env = server.env.as_ref();
    let headers = server.headers.as_ref();

    let result = crate::mcp::get_mcp_server_detail_cached(
        &server_name, url, command, args, env, headers, force_refresh,
    )
    .await;

    match &result {
        Ok(Some(detail)) => log::info!(
            "[MCP] Detail fetched for '{}': tools={}, prompts={}, resources={}",
            server_name,
            detail.tools.len(),
            detail.prompts.len(),
            detail.resources.len()
        ),
        Ok(None) => log::warn!("[MCP] No detail returned for '{}'", server_name),
        Err(e) => log::error!("[MCP] Detail fetch failed for '{}': {}", server_name, e),
    }

    result
}

// ==================== Logging Commands ====================

/// 前端日志写入
#[tauri::command]
pub async fn log_message(level: String, message: String) {
    match level.as_str() {
        "error" => log::error!("[Frontend] {}", message),
        "warn" => log::warn!("[Frontend] {}", message),
        "info" => log::info!("[Frontend] {}", message),
        "debug" => log::debug!("[Frontend] {}", message),
        _ => log::info!("[Frontend] {}", message),
    }
}

/// 获取当前应用可执行文件路径（用于启动新实例）
#[tauri::command]
pub fn get_app_path() -> String {
    std::env::current_exe()
        .unwrap_or_default()
        .to_string_lossy()
        .to_string()
}

/// 启动新的应用实例
#[tauri::command]
pub fn spawn_new_instance() -> Result<(), String> {
    let app_path = std::env::current_exe().map_err(|e| e.to_string())?;
    let mut cmd = crate::platform::new_command(&app_path.to_string_lossy());
    cmd.spawn().map_err(|e| e.to_string())?;
    Ok(())
}

// ==================== Provider Commands ====================

/// 获取 Provider 配置
#[tauri::command]
pub async fn get_providers_config() -> Result<ProvidersConfig, String> {
    crate::providers::get_providers_config().map_err(|e| e.to_string())
}

/// 保存 Provider 配置
#[tauri::command]
pub async fn save_providers_config(config: ProvidersConfig) -> Result<(), String> {
    crate::providers::save_providers_config(&config).map_err(|e| e.to_string())
}

/// 激活 Provider
#[tauri::command]
pub async fn activate_provider(provider_id: String) -> Result<(), String> {
    crate::providers::activate_provider(&provider_id).map_err(|e| e.to_string())
}

/// 创建 Provider
#[tauri::command]
pub async fn create_provider(
    name: String,
    settings_config: serde_json::Value,
    website_url: Option<String>,
    category: Option<String>,
    icon: Option<String>,
    icon_color: Option<String>,
    meta: Option<ProviderMeta>,
) -> Result<Provider, String> {
    crate::providers::create_provider(name, settings_config, website_url, category, icon, icon_color, meta)
        .map_err(|e| e.to_string())
}

/// 更新 Provider
#[tauri::command]
pub async fn update_provider(
    id: String,
    name: Option<String>,
    settings_config: Option<serde_json::Value>,
    notes: Option<String>,
    meta: Option<ProviderMeta>,
) -> Result<Provider, String> {
    crate::providers::update_provider(&id, name, settings_config, notes, meta)
        .map_err(|e| e.to_string())
}

/// 删除 Provider
#[tauri::command]
pub async fn delete_provider(id: String) -> Result<(), String> {
    crate::providers::delete_provider(&id).map_err(|e| e.to_string())
}

/// 更新 Provider 排序
#[tauri::command]
pub async fn update_provider_sort_order(provider_ids: Vec<String>) -> Result<(), String> {
    crate::providers::update_provider_sort_order(provider_ids).map_err(|e| e.to_string())
}

/// 更新通用配置
#[tauri::command]
pub async fn update_common_config(enabled: bool, settings: serde_json::Value) -> Result<(), String> {
    crate::providers::update_common_config(enabled, settings).map_err(|e| e.to_string())
}

/// 检测 cc-switch 数据库是否存在
#[tauri::command]
pub async fn check_cc_switch_db_exists() -> Result<bool, String> {
    Ok(crate::providers::check_cc_switch_db_exists())
}

/// 从 cc-switch 数据库导入 Provider
#[tauri::command]
pub async fn import_from_cc_switch() -> Result<ImportResult, String> {
    crate::providers::import_from_cc_switch().map_err(|e| e.to_string())
}

/// 测试 Provider 连接
#[tauri::command]
pub async fn test_provider_connection(provider_id: String) -> Result<TestConnectionResult, String> {
    crate::providers::test_provider_connection(&provider_id)
        .await
        .map_err(|e| e.to_string())
}
