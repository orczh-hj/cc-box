//! Tauri Commands 模块
//! 定义所有 IPC 命令

use anyhow::Result;
use cc_box_macros::cc_command;
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
    pub(crate) cwd: String,
    #[serde(rename = "type")]
    pub(crate) pty_type: String, // "claude" | "shell"
    pub(crate) cols: Option<u16>,
    pub(crate) rows: Option<u16>,
    pub(crate) args: Option<Vec<String>>,
}

#[derive(Debug, Serialize)]
pub struct PtySpawnResult {
    pub(crate) id: String,
    #[serde(rename = "type")]
    pub(crate) pty_type: String,
    pub(crate) cwd: String,
}

/// spawn 核心：桌面 pty_spawn command 与远程 session.create 的单一实现
/// （同步阻塞——spawn_claude 含子进程启动等待，调用方自行放线程）
pub(crate) fn spawn_pty_core(options: PtySpawnOptions) -> Result<PtySpawnResult, String> {
    let cols = options.cols.unwrap_or(80);
    let rows = options.rows.unwrap_or(24);

    let manager = get_pty_manager().ok_or_else(|| "PTY manager not initialized".to_string())?;

    let result = if options.pty_type == "shell" {
        manager.spawn_shell(&options.cwd, cols, rows)
    } else {
        manager.spawn_claude(&options.cwd, cols, rows, options.args)
    };

    match result {
        Ok(info) => Ok(PtySpawnResult {
            id: info.id,
            pty_type: info.pty_type,
            cwd: info.cwd,
        }),
        Err(e) => Err(e.to_string()),
    }
}

/// 启动 PTY
#[tauri::command]
pub async fn pty_spawn(
    options: PtySpawnOptions,
    _app_handle: AppHandle,
) -> Result<Option<PtySpawnResult>, String> {
    spawn_pty_core(options).map(Some)
}

/// 写入 PTY 输入
#[tauri::command]
pub async fn pty_input(id: String, data: String) -> Result<bool, String> {
    // 控制权在远程（手机）手中时拒绝桌面输入——独占不能只靠 UI 遮罩，
    // 拖放/全局命令等绕过 DOM 的输入路径在 Rust 侧统一把关
    if crate::remote_state::REMOTE_STATE.is_remote_active(&id) {
        return Ok(false);
    }

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

    // 记录桌面端尺寸（远程会话归还时恢复）
    crate::remote_state::REMOTE_STATE.record_size(&id, cols, rows);

    // 控制权在远程（手机）手中时，桌面 fit 不抢占 PTY 尺寸
    if crate::remote_state::REMOTE_STATE.is_remote_active(&id) {
        return Ok(true);
    }

    manager
        .resize(&id, cols, rows)
        .map(|_| true)
        .map_err(|e| e.to_string())
}

/// 桌面端接管被远程持有的会话（解除遮罩 + 收回控制权 + 获得全局控制权）
#[tauri::command]
pub async fn pty_claim(id: String) -> Result<(), String> {
    crate::remote_state::REMOTE_STATE.claim(&id);
    Ok(())
}

/// 桌面端显式接管全局控制权（设备互斥：ControllerMask「接管」按钮调用；
/// 剥离远程持有者全部终端归属并广播，桌面普通操作不触发）
#[tauri::command]
pub async fn controller_takeover() -> Result<(), String> {
    crate::remote_state::REMOTE_STATE.desktop_takeover();
    Ok(())
}

/// 当前全局控制权持有者（''=无人；'desktop'=桌面；数字串=远程连接 id）。
/// 桌面 UI 启动/重载时错过历史 controller-changed 事件，挂载时查询补初始快照
/// （远程端 hello.ok 自带快照，此命令作统一校准入口）
#[tauri::command]
pub async fn controller_status() -> Result<String, String> {
    Ok(crate::remote_state::REMOTE_STATE
        .controller()
        .map(|c| c.id_label())
        .unwrap_or_default())
}

/// 会话重命名双端同步（广播中转：Rust 不存 Tab 元数据，桌面/手机两端各自
/// 应用同一个 updateTabName；发起端也收回环，同名幂等无副作用）。
/// 写但不广播 state-changed（走 SESSION_RENAMED 专用事件），registry! 登记 write(None)
#[cc_command(write)]
#[tauri::command]
pub async fn rename_session(pty_id: String, name: String) -> Result<(), String> {
    crate::remote_state::emit_event(crate::events::names::SESSION_RENAMED, &pty_id, &name);
    Ok(())
}

/// 杀掉 PTY（用户显式关闭：桌面 closeTab/closeAllTabs 唯一后端路径）
#[tauri::command]
pub async fn pty_kill(id: String) -> Result<bool, String> {
    let manager = get_pty_manager().ok_or_else(|| "PTY manager not initialized".to_string())?;

    // 先广播 remote-session-closed 再 kill（与远程 session.kill 同一语义与
    // 顺序）：各端（含其他浏览器端）联动删 Tab，"打开的标签页"不残留 stopped
    // 条目；自然退出（pty-exit 只置 stopped 保留 Tab）与此相区分。先发保证
    // closed 先于 exit 到达（同连接保序），晚到会因 ptyId 已清而匹配不到 Tab
    crate::remote_state::emit_event(crate::events::names::REMOTE_SESSION_CLOSED, &id, "");

    manager.kill(&id).map(|_| true).map_err(|e| e.to_string())
}

/// 杀掉所有 PTY
#[tauri::command]
pub async fn pty_kill_all() -> Result<(), String> {
    let manager = get_pty_manager().ok_or_else(|| "PTY manager not initialized".to_string())?;

    manager.kill_all();
    Ok(())
}

/// 本地已装 Claude CLI 版本号（纯本地只读；转发 installer 实现，桌面
/// invoke 与远程 rpc 白名单共用此统一入口）
#[tauri::command]
pub async fn get_installed_claude_version() -> Result<Option<String>, String> {
    crate::installer::get_installed_claude_version().await
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

/// 目录浏览条目（远程端"添加项目"的电脑目录选择器用）
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DirEntryLite {
    pub name: String,
    pub path: String,
    /// 修改时间（Unix 毫秒；取不到为 None）——前端"最近修改"排序用
    pub modified: Option<u64>,
}

/// 列目录（仅子目录，不列文件——项目选择只需要目录，最小暴露面）。
/// path=None 返回根：Windows 盘符 / Unix 根分区子目录
#[tauri::command]
pub async fn list_directories(path: Option<String>) -> Result<Vec<DirEntryLite>, String> {
    fn mtime_ms(md: &std::fs::Metadata) -> Option<u64> {
        md.modified()
            .ok()
            .and_then(|t| t.duration_since(std::time::UNIX_EPOCH).ok())
            .map(|d| d.as_millis() as u64)
    }

    fn list_subdirs(p: &std::path::Path) -> Result<Vec<DirEntryLite>, String> {
        let mut out = Vec::new();
        let entries = std::fs::read_dir(p).map_err(|e| format!("cannot read {}: {}", p.display(), e))?;
        for entry in entries.flatten() {
            // 跳过符号链接循环； junction/符号链接目录允许进入（Windows 常见）
            if entry.file_type().map(|t| t.is_dir()).unwrap_or(false) {
                let name = entry.file_name().to_string_lossy().into_owned();
                let Ok(full) = entry.path().into_os_string().into_string() else { continue };
                let modified = entry.metadata().as_ref().ok().and_then(mtime_ms);
                out.push(DirEntryLite { name, path: full, modified });
            }
        }
        out.sort_by(|a, b| a.name.to_lowercase().cmp(&b.name.to_lowercase()));
        Ok(out)
    }

    match path {
        None => {
            // 根视图：Windows 枚举存在的盘符；Unix 列 / 子目录
            if cfg!(windows) {
                let mut drives = Vec::new();
                for letter in b'A'..=b'Z' {
                    let root = format!("{}:\\", letter as char);
                    if std::path::Path::new(&root).exists() {
                        drives.push(DirEntryLite { name: root.clone(), path: root, modified: None });
                    }
                }
                Ok(drives)
            } else {
                list_subdirs(std::path::Path::new("/"))
            }
        }
        Some(p) => {
            let path = std::path::PathBuf::from(&p);
            if !path.is_dir() {
                return Err(format!("not a directory: {}", p));
            }
            list_subdirs(&path)
        }
    }
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

/// 更新应用配置（state-changed 广播：config section，全端权威刷新——由
/// #[cc_command] 在函数出口自动广播）
#[cc_command(write = "config")]
#[tauri::command]
pub async fn update_app_config(updates: serde_json::Value) -> Result<(), String> {
    crate::store::update_app_config(updates).map_err(|e| e.to_string())
}

// ==================== Remote Commands ====================

/// 远程连接状态（弹窗展示：直连 + 中继）
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RemoteStatus {
    pub enabled: bool,
    pub running: bool,
    /// 宿主制：远程由本机另一实例提供（本实例未监听）；url 仍为宿主可用地址
    pub hosted_elsewhere: bool,
    pub port: u16,
    pub token: String,
    /// 手机直连 URL（含 token fragment）；token 未生成时为空
    pub url: String,
    /// 中继已启用
    pub relay_enabled: bool,
    /// 中继已连接
    pub relay_connected: bool,
    /// 中继地址（如 https://cc.orczh.top）
    pub relay_url: String,
    /// 当前有效配对码（5 分钟内）；无则空
    pub relay_pair_code: String,
    /// 中继配对二维码内容（relayUrl + #pair=code）；无效则空
    pub relay_pair_url: String,
}

#[tauri::command]
pub async fn get_remote_status() -> Result<RemoteStatus, String> {
    let config = crate::store::get_app_config().map_err(|e| e.to_string())?;
    let enabled = config.remote.as_ref().and_then(|r| r.enabled).unwrap_or(false);
    let port = config
        .remote
        .as_ref()
        .and_then(|r| r.port)
        .unwrap_or(crate::remote_server::DEFAULT_PORT);
    // token 持久于 config（宿主与本实例共享），未开启过的实例读到的与宿主一致
    let token = crate::remote_server::current_token();
    let hosted_elsewhere = crate::remote_server::hosted_elsewhere().await;
    let url = if token.is_empty() {
        String::new()
    } else {
        crate::remote_server::lan_url(&token, port)
    };
    let (relay_enabled, relay_connected, relay_url, pair_code) = crate::remote_client::status_snapshot();
    // 中继 URL 显示为 https/http 形式
    let relay_display = relay_url
        .replacen("wss://", "https://", 1)
        .replacen("ws://", "http://", 1);
    let relay_pair_url = if pair_code.is_empty() {
        String::new()
    } else {
        format!("{}/#pair={}", relay_display, pair_code)
    };
    // 配对码缺失/过期且中继在线：按需重申（一次性码 5 分钟 TTL，仅在面板
    // 查询时触发——结果经 pair.created 异步写入，面板 2s 轮询下一轮拿到）
    if relay_connected && pair_code.is_empty() {
        crate::remote_client::request_pair_code();
    }
    Ok(RemoteStatus {
        enabled,
        running: crate::remote_server::is_running(),
        hosted_elsewhere,
        port,
        token,
        url,
        relay_enabled,
        relay_connected,
        relay_url: relay_display,
        relay_pair_code: pair_code,
        relay_pair_url,
    })
}

/// 远程开关操作互斥：enable/disable 交错执行会产生「enabled=false 但服务运行」
/// 的鬼服状态（手机仍可控而用户以为已关）
static REMOTE_OP_LOCK: std::sync::LazyLock<tokio::sync::Mutex<()>> =
    std::sync::LazyLock::new(|| tokio::sync::Mutex::new(()));

/// 开/关远程访问（动态启停服务器），返回最新状态。
/// 写但不广播 state-changed（开关结果由返回值/连接状态自身体现），registry! 登记 write(None)
#[cc_command(write)]
#[tauri::command]
pub async fn set_remote_enabled(enabled: bool) -> Result<RemoteStatus, String> {
    let _op_guard = REMOTE_OP_LOCK.lock().await;
    if enabled {
        // 安全阻断语义：「停止→开启」轮换全部访问凭证——旧局域二维码（token
        // 已换）与已配对手机（access_token 绑定旧 device_id，relay 查在线表
        // miss）一律失效；用户在面板重扫新码即可。已开启状态重复开启、
        // 或远程由另一实例宿主时不轮换（避免干扰宿主的有效凭证）
        if !crate::remote_server::is_running()
            && !crate::remote_server::hosted_elsewhere().await
        {
            // 轮换失败必须报错（安全阻断语义依赖它）：旧凭证未失效就继续启动
            // 等于向持有旧码的人保持开放
            crate::remote_server::rotate_token().map_err(|e| e.to_string())?;
            crate::remote_client::rotate_identity();
        }
        crate::remote_server::save_remote_config(Some(true), None, None).map_err(|e| e.to_string())?;
        // 启动失败（端口被占等）显式报错给 UI，并回滚开关（否则残留 enabled=true，
        // 重启后自动尝试启动又失败）
        if let Err(e) = crate::remote_server::start().await {
            let _ = crate::remote_server::save_remote_config(Some(false), None, None);
            return Err(e);
        }
        // 中继即默认远程方式：开启时联动连接（内置 orczh.top:8800，
        // 环境变量 CC_BOX_RELAY_URL 可覆盖；幂等）
        crate::remote_client::connect().await;
    } else {
        crate::remote_server::save_remote_config(Some(false), None, None).map_err(|e| e.to_string())?;
        crate::remote_server::stop().await;
        // 安全闭环：停止必须同时断开中继连接，否则手机仍可经公网中继控制
        crate::remote_client::disconnect();
    }
    get_remote_status().await
}

/// 配置/停用中继（url 为空 = 停用），返回最新状态
#[tauri::command]
pub async fn set_relay_url(url: Option<String>) -> Result<RemoteStatus, String> {
    crate::remote_client::set_relay(url).await?;
    get_remote_status().await
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

/// 保存默认 Claude 选项（state-changed 广播：config section，#[cc_command] 自动）
#[cc_command(write = "config")]
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

/// 切换用户级 Skill 启用状态（移动目录到 ~/.cc-box/disabled/skills/；
/// state-changed 广播：skills section，全端权威刷新——#[cc_command] 自动）
#[cc_command(write = "skills")]
#[tauri::command]
pub async fn set_skill_enabled(name: String, enabled: bool) -> Result<(), String> {
    crate::store::set_skill_enabled(&name, enabled).map_err(|e| e.to_string())
}

/// 切换用户级 Agent 启用状态（移动文件到 ~/.cc-box/disabled/agents/；
/// state-changed 广播：agents section，#[cc_command] 自动）
#[cc_command(write = "agents")]
#[tauri::command]
pub async fn set_agent_enabled(name: String, enabled: bool) -> Result<(), String> {
    crate::store::set_agent_enabled(&name, enabled).map_err(|e| e.to_string())
}

/// 切换用户级 MCP Server 启用状态（剪切 ~/.claude.json::mcpServers.<name> 条目；
/// state-changed 广播：mcp section，#[cc_command] 自动）
#[cc_command(write = "mcp")]
#[tauri::command]
pub async fn set_mcp_server_enabled(name: String, enabled: bool) -> Result<(), String> {
    crate::store::set_mcp_server_enabled(&name, enabled).map_err(|e| e.to_string())
}

/// 切换 Plugin 启用状态（调用 claude plugin enable/disable；
/// state-changed 广播：plugins section（其 skills/agents/mcp 子项由前端统一刷新），
/// #[cc_command] 自动）
#[cc_command(write = "plugins")]
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

/// 保存 Provider 配置（state-changed 广播：providers section，#[cc_command] 自动）
#[cc_command(write = "providers")]
#[tauri::command]
pub async fn save_providers_config(config: ProvidersConfig) -> Result<(), String> {
    crate::providers::save_providers_config(&config).map_err(|e| e.to_string())
}

/// 激活 Provider（state-changed 广播：providers section，#[cc_command] 自动）
#[cc_command(write = "providers")]
#[tauri::command]
pub async fn activate_provider(provider_id: String) -> Result<(), String> {
    crate::providers::activate_provider(&provider_id).map_err(|e| e.to_string())
}

/// 创建 Provider（state-changed 广播：providers section，#[cc_command] 自动）
#[cc_command(write = "providers")]
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

/// 更新 Provider（state-changed 广播：providers section，#[cc_command] 自动）
#[cc_command(write = "providers")]
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

/// 删除 Provider（state-changed 广播：providers section，#[cc_command] 自动）
#[cc_command(write = "providers")]
#[tauri::command]
pub async fn delete_provider(id: String) -> Result<(), String> {
    crate::providers::delete_provider(&id).map_err(|e| e.to_string())
}

/// 更新 Provider 排序（state-changed 广播：providers section，#[cc_command] 自动）
#[cc_command(write = "providers")]
#[tauri::command]
pub async fn update_provider_sort_order(provider_ids: Vec<String>) -> Result<(), String> {
    crate::providers::update_provider_sort_order(provider_ids).map_err(|e| e.to_string())
}

/// 更新通用配置（state-changed 广播：providers section，#[cc_command] 自动）
#[cc_command(write = "providers")]
#[tauri::command]
pub async fn update_common_config(enabled: bool, settings: serde_json::Value) -> Result<(), String> {
    crate::providers::update_common_config(enabled, settings).map_err(|e| e.to_string())
}

/// 检测 cc-switch 数据库是否存在
#[tauri::command]
pub async fn check_cc_switch_db_exists() -> Result<bool, String> {
    Ok(crate::providers::check_cc_switch_db_exists())
}

/// 从 cc-switch 数据库导入 Provider（state-changed 广播：providers section，
/// #[cc_command] 自动）
#[cc_command(write = "providers")]
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
