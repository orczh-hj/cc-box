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

/// 桌面端接管被远程持有的会话（per-session 抢占：解除遮罩 + 收回该终端归属）
#[tauri::command]
pub async fn pty_claim(id: String) -> Result<(), String> {
    crate::remote_state::REMOTE_STATE.claim(&id);
    Ok(())
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

/// 杀掉 PTY（用户显式关闭：桌面 closeTab/closeAllTabs 唯一后端路径）；
/// quiet=true 为重启场景的内部清理（restartTab 清旧 PTY）——不广播
/// remote-session-closed，pty-exit 照常到达只置 stopped 保留 Tab 等 respawn
#[tauri::command]
pub async fn pty_kill(id: String, quiet: Option<bool>) -> Result<bool, String> {
    let manager = get_pty_manager().ok_or_else(|| "PTY manager not initialized".to_string())?;

    // 先广播 remote-session-closed 再 kill（与远程 session.kill 同一语义与
    // 顺序）：各端（含其他浏览器端）联动删 Tab，"打开的标签页"不残留 stopped
    // 条目；自然退出（pty-exit 只置 stopped 保留 Tab）与此相区分。先发保证
    // closed 先于 exit 到达（同连接保序），晚到会因 ptyId 已清而匹配不到 Tab。
    // quiet（重启清理）跳过广播：closed 会让各端 closeTab 删 Tab，重启被
    // 误判为关闭（Tab 已删，respawn 的 setTabPty 无处可绑）
    if !quiet.unwrap_or(false) {
        crate::remote_state::emit_event(crate::events::names::REMOTE_SESSION_CLOSED, &id, "");
    }

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

/// 隐藏项目（首页不再主动展示；state-changed(projects) 广播全端刷新）
#[cc_command(write = "projects")]
#[tauri::command]
pub async fn hide_project(path: String) -> Result<(), String> {
    crate::store::hide_project(&path).map_err(|e| e.to_string())
}

/// 取消隐藏项目（用户重新添加该项目时恢复展示）
#[cc_command(write = "projects")]
#[tauri::command]
pub async fn unhide_project(path: String) -> Result<(), String> {
    crate::store::unhide_project(&path).map_err(|e| e.to_string())
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

// ==================== File Transfer Commands（远程浏览器模式文件上传/下载） ====
// 实现单点在 file_transfer.rs（安全校验/会话/分块），此处仅薄包装。
// conn_id/via_relay 由 rpc_dispatch::handle_rpc 服务端注入（远程路径），
// 桌面 invoke 时为 None → 上传/下载拒绝（remote only）；limits/list 只读不拒。

/// 当前文件传输限额（直连/中继两档；前端预检与提示展示）
#[tauri::command]
pub async fn file_transfer_limits() -> Result<crate::file_transfer::TransferLimits, String> {
    Ok(crate::file_transfer::limits())
}

/// 列目录（含文件，下载浏览器用）：范围锁定已知项目内，过滤上传临时文件
#[tauri::command]
pub async fn file_list(path: String) -> Result<Vec<crate::file_transfer::FileEntryLite>, String> {
    crate::file_transfer::list(&path)
}

/// 上传开启：落盘目标 = 会话项目 cwd 下 `.cc-box-files/`（服务端拼接，
/// 客户端不可指定路径）。写命令：受全局控制权拦截（他人持有被拒）
#[cc_command(write)]
#[tauri::command]
pub async fn file_upload_begin(
    pty_id: String,
    file_name: String,
    size: u64,
    conn_id: Option<u64>,
    via_relay: Option<bool>,
) -> Result<crate::file_transfer::UploadBeginResult, String> {
    crate::file_transfer::upload_begin(conn_id, &pty_id, &file_name, size, via_relay)
}

/// 上传分块追加（幂等：ACK 丢失重发同块安全）
#[cc_command(write)]
#[tauri::command]
pub async fn file_upload_chunk(
    upload_id: String,
    offset: u64,
    data_b64: String,
    conn_id: Option<u64>,
) -> Result<crate::file_transfer::UploadChunkResult, String> {
    crate::file_transfer::upload_chunk(conn_id, &upload_id, offset, &data_b64)
}

/// 上传完成：hard_link 原子提交（不覆盖已有文件，冲突自动 `(n)` 改名）
#[cc_command(write)]
#[tauri::command]
pub async fn file_upload_finish(
    upload_id: String,
    conn_id: Option<u64>,
) -> Result<crate::file_transfer::UploadFinishResult, String> {
    crate::file_transfer::upload_finish(conn_id, &upload_id)
}

/// 放弃上传：删临时文件与会话（幂等）
#[cc_command(write)]
#[tauri::command]
pub async fn file_upload_abort(upload_id: String, conn_id: Option<u64>) -> Result<(), String> {
    crate::file_transfer::upload_abort(conn_id, &upload_id)
}

/// 下载开启（需控制权）：白名单（已知项目内）+ is_file + 限额；
/// 会话固化 size/mtime 供后续 chunk 一致性校验
#[cc_command(write)]
#[tauri::command]
pub async fn file_download_begin(
    path: String,
    conn_id: Option<u64>,
    via_relay: Option<bool>,
) -> Result<crate::file_transfer::DownloadBeginResult, String> {
    crate::file_transfer::download_begin(conn_id, &path, via_relay)
}

/// 下载分块拉取：逐块校验归属连接与源文件 size/mtime 未变（变化即拒）
#[cc_command(write)]
#[tauri::command]
pub async fn file_download_chunk(
    download_id: String,
    offset: u64,
    conn_id: Option<u64>,
) -> Result<crate::file_transfer::DownloadChunkResult, String> {
    crate::file_transfer::download_chunk(conn_id, &download_id, offset)
}

/// 下载票据签发结果：urlPath 由服务端按连接类型给出（直连 /dl/<ticket>、
/// 中继 /?dl=<device_id>.<ticket>），前端只拼 origin，不猜 host
#[derive(serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DownloadTicketResult {
    pub ticket: String,
    pub url_path: String,
    pub file_name: String,
    pub size: u64,
    pub expires_at: u64,
}

/// 签发一次性下载票据：供系统浏览器/系统下载器不带 token 直接取文件。
/// 准入校验与 file_download_begin 完全一致（白名单/限额）。
#[cc_command(write)]
#[tauri::command]
pub async fn file_download_ticket(
    path: String,
    conn_id: Option<u64>,
    via_relay: Option<bool>,
) -> Result<DownloadTicketResult, String> {
    // 中继链路先确认设备身份，再占用票据槽位（否则失败时票据无人可达却占槽 10 分钟）
    let relay = via_relay.unwrap_or(true);
    let device_id = if relay {
        Some(
            crate::remote_client::current_device_id()
                .ok_or("relay not connected: cannot build download link")?,
        )
    } else {
        None
    };
    let info = crate::file_transfer::create_download_ticket(conn_id, &path, via_relay)?;
    let url_path = match device_id {
        Some(id) => format!("/?dl={}.{}", id, info.ticket),
        None => format!("/dl/{}", info.ticket),
    };
    Ok(DownloadTicketResult {
        ticket: info.ticket,
        url_path,
        file_name: info.file_name,
        size: info.size,
        expires_at: info.expires_at,
    })
}

/// 票据分块读取（中继票据页面用）：票据即权限，不绑定连接。
/// 只读登记——受限连接不持有控制权，不能被写命令守卫拦截。
#[tauri::command]
pub async fn file_download_ticket_chunk(
    ticket: String,
    offset: u64,
) -> Result<crate::file_transfer::DownloadChunkResult, String> {
    crate::file_transfer::download_ticket_chunk(&ticket, offset)
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

/// 远程连接状态（弹窗展示：局域直连 + 中继；所有 GUI 实例内容一致）
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RemoteStatus {
    pub enabled: bool,
    pub port: u16,
    pub token: String,
    /// 手机直连 URL（含 token fragment）；未开启/旧版占用时为空
    pub url: String,
    pub relay_enabled: bool,
    pub relay_connected: bool,
    pub relay_url: String,
    pub relay_pair_code: String,
    pub relay_pair_url: String,
    /// 与 Broker 的 IPC 是否已注册
    pub broker_connected: bool,
    /// 旧版 CC-Box 占用设备入口：设备级多实例控制暂不可用
    pub legacy_owned: bool,
    pub lan_running: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lan_error: Option<String>,
    /// 开机自启动（仅 Broker）
    pub autostart: bool,
}

/// 当前可用的 Broker 状态：GUI 用镜像，Broker 内用本地快照
fn broker_status_now() -> Option<crate::gateway_proto::BrokerStatus> {
    crate::broker_client::status_mirror().or_else(crate::broker::status_snapshot)
}

fn build_remote_status(
    config: &crate::store::AppConfig,
    status: Option<crate::gateway_proto::BrokerStatus>,
) -> RemoteStatus {
    // 设备级字段一律以 Broker 为准；Broker 离线时才回退本地配置。
    let enabled = status
        .as_ref()
        .map(|s| s.enabled)
        .unwrap_or_else(|| {
            config
                .remote
                .as_ref()
                .and_then(|r| r.enabled)
                .unwrap_or(false)
        });
    let port = status
        .as_ref()
        .map(|s| s.port)
        .unwrap_or_else(|| {
            config
                .remote
                .as_ref()
                .and_then(|r| r.port)
                .unwrap_or(crate::remote_server::DEFAULT_PORT)
        });
    let token = crate::remote_server::current_token();
    let legacy = status.as_ref().map(|s| s.legacy_owned).unwrap_or(false);
    let lan_running = status.as_ref().map(|s| s.lan_running).unwrap_or(false);
    let url = if token.is_empty() || legacy || !lan_running {
        String::new()
    } else {
        crate::remote_server::lan_url(&token, port)
    };
    let relay_url_raw = status
        .as_ref()
        .map(|s| s.relay_url.clone())
        .unwrap_or_default();
    let relay_display = relay_url_raw
        .replacen("wss://", "https://", 1)
        .replacen("ws://", "http://", 1);
    let pair_code = status
        .as_ref()
        .map(|s| s.relay_pair_code.clone())
        .unwrap_or_default();
    let relay_pair_url = if pair_code.is_empty() {
        String::new()
    } else {
        format!("{}/#pair={}", relay_display, pair_code)
    };
    RemoteStatus {
        enabled,
        port,
        token,
        url,
        relay_enabled: status.as_ref().map(|s| s.relay_enabled).unwrap_or(false),
        relay_connected: status
            .as_ref()
            .map(|s| s.relay_connected)
            .unwrap_or(false),
        relay_url: relay_display,
        relay_pair_code: pair_code,
        relay_pair_url,
        broker_connected: status.is_some(),
        legacy_owned: legacy,
        lan_running,
        lan_error: status.as_ref().and_then(|s| s.lan_error.clone()),
        autostart: status
            .as_ref()
            .map(|s| s.autostart)
            .unwrap_or_else(|| crate::broker::autostart::is_enabled()),
    }
}

#[tauri::command]
pub async fn get_remote_status() -> Result<RemoteStatus, String> {
    let config = crate::store::get_app_config().map_err(|e| e.to_string())?;
    // GUI 只读 Broker 镜像；不做跨 IPC 的阻塞刷新（面板 2s 轮询保持轻量）。
    // 配对码的按需重申由 Broker 的 GetStatus/状态同步任务负责。
    let status = broker_status_now();
    Ok(build_remote_status(&config, status))
}

/// 开/关远程访问（设备级；由 Broker 执行）。所有 GUI 实例语义一致，
/// 面板三个按钮（更新连接码/刷新密钥/停止远程）都可操作。
/// 写但不广播 state-changed；registry! 登记 write(None)
#[cc_command(write)]
#[tauri::command]
pub async fn set_remote_enabled(enabled: bool) -> Result<RemoteStatus, String> {
    let status =
        crate::broker_client::control(crate::gateway_proto::BrokerOp::SetEnabled { enabled })
            .await?;
    let config = crate::store::get_app_config().map_err(|e| e.to_string())?;
    Ok(build_remote_status(&config, status.or_else(broker_status_now)))
}

/// 配置/停用中继（url 为空 = 停用），返回最新状态
#[tauri::command]
pub async fn set_relay_url(url: Option<String>) -> Result<RemoteStatus, String> {
    let status = crate::broker_client::control(crate::gateway_proto::BrokerOp::SetRelayUrl {
        url,
    })
    .await?;
    let config = crate::store::get_app_config().map_err(|e| e.to_string())?;
    Ok(build_remote_status(&config, status.or_else(broker_status_now)))
}

/// 手动更新中继配对码（一次性码被设备消费后，为下一台设备重申新码），
/// 等新码到达才返回（面板即时刷新二维码）。桌面专属 → rpc_dispatch EXCLUDED
#[tauri::command]
pub async fn refresh_relay_pair_code() -> Result<RemoteStatus, String> {
    let status =
        crate::broker_client::control(crate::gateway_proto::BrokerOp::RefreshPairCode).await?;
    let config = crate::store::get_app_config().map_err(|e| e.to_string())?;
    Ok(build_remote_status(&config, status.or_else(broker_status_now)))
}

/// 刷新访问密钥（显式轮换全部凭证 + 踢掉一切已连接设备）。
/// 桌面专属 → rpc_dispatch EXCLUDED
#[tauri::command]
pub async fn rotate_remote_secret() -> Result<RemoteStatus, String> {
    let status =
        crate::broker_client::control(crate::gateway_proto::BrokerOp::RotateSecret).await?;
    let config = crate::store::get_app_config().map_err(|e| e.to_string())?;
    Ok(build_remote_status(&config, status.or_else(broker_status_now)))
}

/// 开/关开机自启动（设备级；桌面专属 → rpc_dispatch EXCLUDED）。
/// Linux 无托盘时由此面板开关替代托盘菜单项。
#[tauri::command]
pub async fn set_broker_autostart(enabled: bool) -> Result<RemoteStatus, String> {
    let status = crate::broker_client::control(
        crate::gateway_proto::BrokerOp::SetAutostart { enabled },
    )
    .await?;
    let config = crate::store::get_app_config().map_err(|e| e.to_string())?;
    Ok(build_remote_status(&config, status.or_else(broker_status_now)))
}

/// 更新安装前调用：让 Broker 写 update_in_progress 并退出，释放 exe 锁。
/// 桌面专属 → rpc_dispatch EXCLUDED
#[tauri::command]
pub async fn prepare_for_update() -> Result<(), String> {
    crate::broker_client::prepare_for_update().await
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

/// GUI 窗口标题变化上报 Broker（仅桌面窗口；Broker 托盘展示用）
#[tauri::command]
pub fn update_window_title(title: String) {
    crate::broker_client::set_window_title(title);
}

/// 启动时领取“远程连接”请求（防止 openRemote 事件早于前端监听）
#[tauri::command]
pub fn take_pending_open_remote() -> bool {
    crate::take_pending_open_remote()
}

/// 启动时领取“更新到新版本”请求（防止 openUpdate 事件早于前端监听）
#[tauri::command]
pub fn take_pending_open_update() -> bool {
    crate::take_pending_open_update()
}

/// 更新安装失败：清除 update 抑制并恢复 Broker 连接（桌面专属）
#[tauri::command]
pub fn resume_broker_connection() {
    crate::broker_client::resume_connection();
}

/// 启动新的应用实例（桌面直接调用；远程经 Broker 的 RPC 白名单调用）
#[cc_command(write)]
#[tauri::command]
pub async fn spawn_new_instance() -> Result<(), String> {
    crate::broker::windows::spawn_window()
}

// ==================== Provider Commands ====================

/// 获取 Provider 配置
#[tauri::command]
pub async fn get_providers_config() -> Result<ProvidersConfig, String> {
    crate::providers::get_providers_config().map_err(|e| e.to_string())
}

/// 打开用户级 Claude settings.json（「打开用户配置」按钮，桌面专属）。
///
/// 必须走 Rust 侧 `Shell::open`：JS 侧 `@tauri-apps/plugin-shell` 的 open
/// 受 scope 正则限制，只允许 http(s)/mailto/tel，本地路径会被拒绝。
#[tauri::command]
pub async fn open_claude_user_settings(app: tauri::AppHandle) -> Result<(), String> {
    use tauri_plugin_shell::ShellExt;

    let path = crate::providers::claude_user_settings_path().map_err(|e| e.to_string())?;
    app.shell()
        .open(path, None)
        .map_err(|e| format!("Failed to open user settings: {}", e))
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
