//! RPC 命令分发器（统一管理：命令单一注册链）
//!
//! 浏览器端（经远程 WS）调用 Tauri command 的唯一入口。`registry!` 声明宏
//! 集中定义——一行一命令（名字 → 读/写声明 → 参数签名 → commands.rs 函数），
//! 宏展开一次生成四样注册：RPC 白名单、分发 match、广播 section、远程写
//! 权限标记，新增命令只写一行。
//!
//! **差集守卫**：测试解析 `lib.rs` invoke_handler 注册表，断言注册项 ⊆
//! RPC_WHITELIST + EXCLUDED（两项并集），新增 Tauri command 时若忘记在此
//! 登记，测试失败强制评估。
//! PTY 操作（spawn/input/resize/kill/claim）不走 RPC，一律走远程协议专用消息
//! （session.create 带项目白名单与并发上限、attach 独占校验）。

use cc_box_remote_protocol::ServerMessage;
use serde::de::DeserializeOwned;
use serde_json::{Map, Value};

/// 显式排除（不可经远程调用）：installer 12 个系统级 + PTY 6 个（走专用消息）
/// + 文件管理器/app_path/log/开机自启动（桌面级）。
/// 注意：`spawn_new_instance` 不在此列——远程零窗口时需要 Broker 拉起窗口。
pub const EXCLUDED: &[&str] = &[
    // installer（系统级：下载/安装/杀进程）
    "get_latest_versions",
    "check_installed_versions",
    "check_claude_cli_update",
    "check_claude_running",
    "kill_claude_processes",
    "download_and_install_claude",
    "cancel_auto_install",
    "list_claude_versions",
    "download_claude_version",
    "cancel_claude_download",
    "install_claude_version",
    // 无条件收录（不带 cfg）：差集守卫按源码文本对拍 lib.rs 的 generate_handler
    // 列表（文本解析不执行 cfg 条目），若此处随 cfg 剔除，非 Windows 守卫必失败
    "download_and_install_git",
    // PTY（走 session.create / pty.input 等专用消息，有白名单与独占校验）
    "pty_spawn",
    "pty_input",
    "pty_resize",
    "pty_kill",
    "pty_kill_all",
    "pty_claim",
    // 桌面专属接管（远程走 controller.takeover 协议消息）
    "controller_takeover",
    // 桌面级（打开本地文件/文件夹）
    "open_in_file_manager",
    "open_claude_user_settings",
    "get_app_path",
    // 窗口标题回传 / 待处理远程面板请求（仅桌面窗口）
    "update_window_title",
    "take_pending_open_remote",
    "take_pending_open_update",
    "resume_broker_connection",
    // 安全：中继重定向=窃取 device_key 的通道（详见 rpc_table 尾注释）
    "set_relay_url",
    // 桌面专属：重申一次性配对码，新码仅经桌面扫码面板展示，远程调用无意义
    "refresh_relay_pair_code",
    // 桌面专属：密钥轮换只应由桌面「刷新密钥」按钮发起（会踢掉一切连接，
    // 远程调用等于让已连接设备自毁）
    "rotate_remote_secret",
    // 桌面专属：更新安装前退出 Broker（远程调用无意义且会中断后台服务）
    "prepare_for_update",
    // 桌面专属：开机自启动是本机配置，远程设备不应改动宿主登录项
    "set_broker_autostart",
];

/// 状态广播 section（state-changed 事件 payload 的 id 字段）。
/// 线上格式 = 变体名小写（as_str 对拍测试守卫），前端 App.vue 状态中心与
/// 远程 UI 消费同一组字符串
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StateSection {
    Skills,
    Agents,
    Mcp,
    Plugins,
    Providers,
    Config,
    Projects,
}

impl StateSection {
    pub fn as_str(&self) -> &'static str {
        match self {
            StateSection::Skills => "skills",
            StateSection::Agents => "agents",
            StateSection::Mcp => "mcp",
            StateSection::Plugins => "plugins",
            StateSection::Providers => "providers",
            StateSection::Config => "config",
            StateSection::Projects => "projects",
        }
    }
}

/// 访问声明 → 广播 section（registry! 内部用）
macro_rules! section_of {
    (read) => { None };
    (write None) => { None };
    (write $section:ident) => { Some(crate::rpc_dispatch::StateSection::$section) };
}

/// 访问声明 → 是否写命令（registry! 内部用）
macro_rules! is_write_access {
    (read) => { false };
    (write) => { true };
}

/// 一行一命令：名字（与 Tauri command 同名）→ 读/写声明 → 参数（Rust
/// snake_case 签名，经 camelCase 键从 args 提取，与 Tauri invoke 惯例一致）
/// → handler。读/写声明三态：
/// - `read()`        只读，不参与远程写权限拦截
/// - `write(Sect)`   写命令：远程写权限拦截 + 落盘成功后广播 state-changed(Sect)
/// - `write(None)`   写但不广播（已有专用事件或无跨端 UI 状态）
/// 宏展开一次生成：RPC_WHITELIST、dispatch_table、write_section、is_remote_write
macro_rules! registry {
    ( $( $cmd:literal => $access:ident ($($section:ident)?) => | $($arg:ident : $ty:ty),* | $handler:path ),* $(,)? ) => {
        /// RPC 白名单（可远程调用的命令名集合）
        pub const RPC_WHITELIST: &[&str] = &[ $($cmd),* ];

        /// 执行宏表命令；未知命令返回 Err
        pub async fn dispatch_table(
            cmd: &str,
            args: &Map<String, Value>,
        ) -> Result<Value, String> {
            match cmd {
                $( $cmd => {
                    $( let $arg: $ty = take_arg(args, stringify!($arg))?; )*
                    let res = $handler($($arg),*).await;
                    match res {
                        Ok(v) => serde_json::to_value(v).map_err(|e| e.to_string()),
                        Err(e) => Err(e),
                    }
                } )*
                other => Err(format!("command not allowed: {}", other)),
            }
        }

        /// 写命令的广播 section（read() / write(None) → None）。守卫测试强制
        /// write(Sect) 命令在 commands.rs 成功路径调用 emit_state_changed
        pub fn write_section(cmd: &str) -> Option<crate::rpc_dispatch::StateSection> {
            match cmd {
                $( $cmd => section_of!($access $($section)?), )*
                _ => None,
            }
        }

        /// 是否远程写命令（权限拦截用：非控制者调用被拒，见 remote_state 控制权规则）
        pub fn is_remote_write(cmd: &str) -> bool {
            match cmd {
                $( $cmd => is_write_access!($access), )*
                _ => false,
            }
        }
    };
}

registry! {
    // ==== 检查 / 首页 ====
    "get_home_data" => read() => |project_limit: Option<usize>, session_limit: Option<usize>| crate::commands::get_home_data,
    "get_check_results" => read() => | | crate::commands::get_check_results,
    "run_checks" => read() => | | crate::commands::run_checks,
    "test_communication" => read() => |message: String| crate::commands::test_communication,
    "get_system_locale" => read() => | | crate::commands::get_system_locale,
    // 本地已装 Claude 版本号（纯本地只读，无 HTTP 无副作用）：手机端
    // Claude CLI 版本页展示需要；下载/安装类 installer 命令仍排除
    "get_installed_claude_version" => read() => | | crate::installer::get_installed_claude_version,
    // 前端日志上报（高频调用，浏览器端统一进桌面日志）
    "log_message" => read() => |level: String, message: String| log_message_rpc,

    // ==== 项目 / 会话（只读查询） ====
    "get_projects" => read() => |limit: Option<usize>, offset: Option<usize>| crate::commands::get_projects,
    // 隐藏/恢复展示：跨端可见的首页列表状态变更，广播 projects section
    "hide_project" => write(Projects) => |path: String| crate::commands::hide_project,
    "unhide_project" => write(Projects) => |path: String| crate::commands::unhide_project,
    "list_directories" => read() => |path: Option<String>| crate::commands::list_directories,
    "get_project_info" => read() => |path: String| crate::commands::get_project_info,
    "get_sessions" => read() => |project_path: String, limit: Option<usize>, offset: Option<usize>| crate::commands::get_sessions,
    "get_session_count" => read() => |project_path: String| crate::commands::get_session_count,
    "get_all_recent_sessions" => read() => |limit: Option<usize>| crate::commands::get_all_recent_sessions,
    "get_session_details" => read() => |project_path: String, session_id: String| crate::commands::get_session_details,
    "search_session_messages" => read() => |project_path: String, query: String, limit: Option<usize>| crate::commands::search_session_messages,
    "get_project_config" => read() => |project_path: String| crate::commands::get_project_config,
    "save_last_project" => read() => |path: String| crate::commands::save_last_project,
    // 会话重命名双端同步（广播中转，session-renamed 事件回环应用）
    "rename_session" => write(None) => |pty_id: String, name: String| crate::commands::rename_session,

    // ==== 配置 ====
    // get_app_config 经 rpc 脱敏：remote 凭证（token/deviceId/deviceKey）不下发
    // 浏览器端——LAN 明文 WS 下被动嗅探即得 device_key，可冒充桌面注册中继
    "get_app_config" => read() => | | get_app_config_sanitized,
    "update_app_config" => write(Config) => |updates: Value| crate::commands::update_app_config,
    "get_default_claude_options" => read() => | | crate::commands::get_default_claude_options,
    "save_default_claude_options" => write(Config) => |options: crate::store::DefaultClaudeOptions| crate::commands::save_default_claude_options,

    // ==== Agents / Skills / MCP / Plugins ====
    "get_all_agents" => read() => |project_path: String| crate::commands::get_all_agents,
    "get_all_skills" => read() => |project_path: String| crate::commands::get_all_skills,
    "get_all_mcp_servers" => read() => |project_path: String| crate::commands::get_all_mcp_servers,
    "get_all_plugins" => read() => |project_path: String| crate::commands::get_all_plugins,
    "set_skill_enabled" => write(Skills) => |name: String, enabled: bool| crate::commands::set_skill_enabled,
    "set_agent_enabled" => write(Agents) => |name: String, enabled: bool| crate::commands::set_agent_enabled,
    "set_mcp_server_enabled" => write(Mcp) => |name: String, enabled: bool| crate::commands::set_mcp_server_enabled,
    "set_plugin_enabled" => write(Plugins) => |plugin_id: String, enabled: bool| crate::commands::set_plugin_enabled,
    "get_mcp_server_detail" => read() => |project_path: String, server_name: String, force_refresh: bool| crate::commands::get_mcp_server_detail,

    // ==== Provider ====
    "get_providers_config" => read() => | | crate::commands::get_providers_config,
    "save_providers_config" => write(Providers) => |config: crate::providers::ProvidersConfig| crate::commands::save_providers_config,
    "activate_provider" => write(Providers) => |provider_id: String| crate::commands::activate_provider,
    "create_provider" => write(Providers) => |name: String, settings_config: Value, website_url: Option<String>, category: Option<String>, icon: Option<String>, icon_color: Option<String>, meta: Option<crate::providers::ProviderMeta>| crate::commands::create_provider,
    "update_provider" => write(Providers) => |id: String, name: Option<String>, settings_config: Option<Value>, notes: Option<String>, meta: Option<crate::providers::ProviderMeta>| crate::commands::update_provider,
    "delete_provider" => write(Providers) => |id: String| crate::commands::delete_provider,
    "update_provider_sort_order" => write(Providers) => |provider_ids: Vec<String>| crate::commands::update_provider_sort_order,
    "update_common_config" => write(Providers) => |enabled: bool, settings: Value| crate::commands::update_common_config,
    "check_cc_switch_db_exists" => read() => | | crate::commands::check_cc_switch_db_exists,
    "import_from_cc_switch" => write(Providers) => | | crate::commands::import_from_cc_switch,
    "test_provider_connection" => read() => |provider_id: String| crate::commands::test_provider_connection,

    // ==== 窗口管理 ====
    // 远程在零窗口时拉起新的 GUI 窗口（Broker 本地执行，无需成员在线）。
    // 服务端在 broker/windows.rs 做 12 窗口上限与 2s 冷却。
    "spawn_new_instance" => write(None) => | | spawn_new_instance_rpc,

    // ==== 远程设置（浏览器端查看/管理远程开关） ====
    "get_remote_status" => read() => | | crate::commands::get_remote_status,
    "set_remote_enabled" => write(None) => |enabled: bool| crate::commands::set_remote_enabled,
    // 全局控制权只读查询（桌面 UI 启动补快照；浏览器端错过事件后校准）
    "controller_status" => read() => | | crate::commands::controller_status,
    // set_relay_url 不远程暴露：已连客户端可把中继重定向到攻击者服务器，
    // 下次注册即送出 device_key（身份窃取）。中继地址由环境变量/内置默认管理

    // ==== 文件传输（远程浏览器模式上传/下载；实现单点 file_transfer.rs） ====
    // conn_id/via_relay 由 handle_rpc 服务端注入（覆盖客户端伪造值），
    // 用于会话归属校验与限额选档（中继流量按量计费从严）
    "file_transfer_limits" => read() => | | crate::commands::file_transfer_limits,
    "file_list" => read() => |path: String| crate::commands::file_list,
    "file_upload_begin" => write(None) => |pty_id: String, file_name: String, size: u64, conn_id: Option<u64>, via_relay: Option<bool>| crate::commands::file_upload_begin,
    "file_upload_chunk" => write(None) => |upload_id: String, offset: u64, data_b64: String, conn_id: Option<u64>| crate::commands::file_upload_chunk,
    "file_upload_finish" => write(None) => |upload_id: String, conn_id: Option<u64>| crate::commands::file_upload_finish,
    "file_upload_abort" => write(None) => |upload_id: String, conn_id: Option<u64>| crate::commands::file_upload_abort,
    "file_download_begin" => write(None) => |path: String, conn_id: Option<u64>, via_relay: Option<bool>| crate::commands::file_download_begin,
    "file_download_chunk" => write(None) => |download_id: String, offset: u64, conn_id: Option<u64>| crate::commands::file_download_chunk,
    // 一次性票据：直链下载（微信/系统浏览器不带 token），urlPath 由服务端拼
    "file_download_ticket" => write(None) => |path: String, conn_id: Option<u64>, via_relay: Option<bool>| crate::commands::file_download_ticket,
    // 票据分块：票据即权限，只读登记（受限连接无控制权，不能被写守卫拦截）
    "file_download_ticket_chunk" => read() => |ticket: String, offset: u64| crate::commands::file_download_ticket_chunk,
}

/// 抹掉 rpc 响应中的远程凭证字段（token/deviceId/deviceKey）
fn sanitize_config_for_rpc(mut cfg: crate::store::AppConfig) -> crate::store::AppConfig {
    if let Some(remote) = cfg.remote.as_mut() {
        remote.token = None;
        remote.device_id = None;
        remote.device_key = None;
    }
    cfg
}

/// rpc 版 get_app_config：脱敏后再下发（见 rpc_table 内注释）
async fn get_app_config_sanitized() -> Result<crate::store::AppConfig, String> {
    let cfg = crate::commands::get_app_config().await?;
    Ok(sanitize_config_for_rpc(cfg))
}

/// RPC 总入口
pub async fn dispatch(cmd: &str, args: &Map<String, Value>) -> Result<Value, String> {
    dispatch_table(cmd, args).await
}

/// log_message 的 Result 包装（高频前端日志上报，浏览器端日志统一进桌面日志）
async fn log_message_rpc(level: String, message: String) -> Result<(), String> {
    crate::commands::log_message(level, message).await;
    Ok(())
}

/// 远程零窗口拉起 GUI（handler 是同步 Tauri command，包成 async）
async fn spawn_new_instance_rpc() -> Result<(), String> {
    crate::commands::spawn_new_instance().await
}

/// 文件传输命令的服务端权威注入：connId 供会话归属校验（他人 uploadId/
/// downloadId 拒绝）；viaRelay 供限额选档（中继流量按量计费，未知连接按
/// 中继从严）。insert 覆盖客户端任何同名伪造值；take_arg 优先 camelCase，
/// 故客户端 snake_case 变体同样无法绕过
pub(crate) fn inject_file_transfer_args(
    conn_id: u64,
    cmd: &str,
    args: &mut Map<String, Value>,
    via_relay: bool,
) {
    if !cmd.starts_with("file_") {
        return;
    }
    args.insert("viaRelay".into(), Value::Bool(via_relay));
    args.insert("connId".into(), Value::Number(conn_id.into()));
}

/// 从 args 对象提取参数：优先 camelCase 键（Tauri invoke 惯例），回退原 snake_case；
/// Option<T> 缺失时反序列化为 None，必填缺失报错
fn take_arg<T: DeserializeOwned>(args: &Map<String, Value>, name: &str) -> Result<T, String> {
    let camel = snake_to_camel(name);
    let v = args.get(&camel).or_else(|| args.get(name));
    match v {
        Some(v) => serde_json::from_value(v.clone()).map_err(|e| format!("arg {}: {}", name, e)),
        None => serde_json::from_value(Value::Null)
            .map_err(|_| format!("missing required arg: {}", name)),
    }
}

fn snake_to_camel(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    let mut upper_next = false;
    for c in s.chars() {
        if c == '_' {
            upper_next = true;
        } else if upper_next {
            out.extend(c.to_uppercase());
            upper_next = false;
        } else {
            out.push(c);
        }
    }
    out
}

/// 是否为写操作（远程调用时打 info 日志便于审计）
pub(crate) fn is_write_cmd(cmd: &str) -> bool {
    let prefixes = [
        "update_", "save_", "set_", "delete_", "create_", "activate_", "import_", "run_",
    ];
    prefixes.iter().any(|p| cmd.starts_with(p))
}

/// 写命令注册表豁免清单（is_write_cmd 前缀命中但实际只读/无跨端状态）：
/// run_checks 仅重跑环境检查、save_last_project 仅记录"上次项目"偏好
pub const WRITE_EXEMPT: &[&str] = &["run_checks", "save_last_project"];

/// RPC 入口（remote_state 的 RpcCall 分支调用）：线程内执行（dispatch 是 async；
/// 调用频率低，独立线程 + current_thread runtime，与 SessionCreate 先例一致）。
/// 写命令受全局控制权准入（设备互斥）：他人持有时拒绝、不执行；执行成功后
/// 无人持有则发起者自动持有（write_success_acquire，与 attach/create 同规则）
pub fn handle_rpc(conn_id: u64, rpc_id: u64, cmd: &str, args: &Value) {
    let cmd = cmd.to_string();
    let mut args = args.as_object().cloned().unwrap_or_default();
    // 文件传输命令：服务端权威注入连接身份与中继标志（覆盖客户端任何伪造值）
    if cmd.starts_with("file_") {
        let via = crate::remote_state::REMOTE_STATE
            .conn_via_relay(conn_id)
            .unwrap_or(true); // 未知连接按中继从严
        inject_file_transfer_args(conn_id, &cmd, &mut args, via);
    }
    let is_write = is_remote_write(&cmd);
    if is_write {
        log::info!("[remote] rpc write cmd '{}' from conn {}", cmd, conn_id);
        // 服务端兜底拦截（前端全局遮罩是第一道）：非控制设备的写操作被拒
        if let Err(holder) = crate::remote_state::REMOTE_STATE.controller_gate(conn_id) {
            log::warn!(
                "[remote] rpc write '{}' denied (conn {} != controller {})",
                cmd,
                conn_id,
                holder.id_label()
            );
            crate::remote_state::REMOTE_STATE.try_send(
                conn_id,
                ServerMessage::RpcResult {
                    id: rpc_id,
                    ok: false,
                    data: None,
                    error: Some("controller held by another device, takeover first".to_string()),
                },
            );
            return;
        }
    }
    std::thread::spawn(move || {
        let rt = match tokio::runtime::Builder::new_current_thread().enable_all().build() {
            Ok(rt) => rt,
            Err(e) => {
                log::error!("[remote] rpc runtime build failed: {}", e);
                return;
            }
        };
        let result = rt.block_on(dispatch(&cmd, &args));
        if is_write && result.is_ok() {
            // 写操作成功 → 无人持有时自动成为控制者（广播 controller-changed）
            crate::remote_state::REMOTE_STATE.write_success_acquire(conn_id);
        }
        let msg = match result {
            Ok(data) => ServerMessage::RpcResult {
                id: rpc_id,
                ok: true,
                data: Some(data),
                error: None,
            },
            Err(e) => {
                log::warn!("[remote] rpc '{}' failed: {}", cmd, e);
                ServerMessage::RpcResult {
                    id: rpc_id,
                    ok: false,
                    data: None,
                    error: Some(e),
                }
            }
        };
        crate::remote_state::REMOTE_STATE.try_send(conn_id, msg);
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn Rpc_SpawnNewInstanceWhitelisted_001() {
        assert!(RPC_WHITELIST.contains(&"spawn_new_instance"));
        assert!(!EXCLUDED.contains(&"spawn_new_instance"));
        assert!(is_remote_write("spawn_new_instance"));
        assert!(write_section("spawn_new_instance").is_none());
    }

    /// 从 lib.rs 源码提取 invoke_handler 注册的命令名：截取 `generate_handler![`
    /// 到 `])` 的块，按行取 `module::name` 的 name 段（跳过注释与属性行；
    /// 无法识别的非空行直接 panic——静默跳过会让差集守卫对该命令失效）
    fn parse_registered_commands(src: &str) -> Vec<String> {
        let start = src
            .find("generate_handler![")
            .expect("lib.rs 必须有 generate_handler!");
        let rest = &src[start + "generate_handler![".len()..];
        // 列表以 "])" 收尾；块内无嵌套 "])" 序列（cfg 属性是 ")]"）
        let end = rest.find("])").expect("generate_handler 列表未闭合");
        rest[..end]
            .lines()
            .filter_map(|line| {
                let line = line.trim();
                if line.is_empty() || line.starts_with('#') || line.starts_with("//") {
                    return None; // 空行 / cfg 等属性行（作用于下一项，项本身仍解析）
                }
                let Some(name) = line
                    .trim_end_matches(',')
                    .rsplit("::")
                    .next()
                    .map(|s| s.trim())
                else {
                    panic!("generate_handler 列表存在无法解析的行: '{}'", line);
                };
                if !name.chars().all(|c| c.is_ascii_alphanumeric() || c == '_') {
                    panic!("generate_handler 列表项 '{}' 不是合法命令名", name);
                }
                Some(name.to_string())
            })
            .collect()
    }

    /// 差集守卫：每个注册命令必须在 RPC 白名单或排除表中（防新增命令漏评估）。
    /// 注册表直接解析 lib.rs 源码（无人肉镜像，漏登记即测试失败）
    #[test]
    fn rpc_白名单与排除表覆盖全部注册命令() {
        let registered = parse_registered_commands(include_str!("lib.rs"));
        assert!(
            registered.len() >= 50,
            "lib.rs 解析结果异常（{} 条），检查 generate_handler 块格式",
            registered.len()
        );
        for cmd in &registered {
            let listed = RPC_WHITELIST.contains(&cmd.as_str()) || EXCLUDED.contains(&cmd.as_str());
            assert!(
                listed,
                "command '{}' 未在 rpc_dispatch 的 registry!/EXCLUDED 中登记；\
                 新增 Tauri command 必须二选一登记（差集守卫）",
                cmd
            );
        }
        // 白名单与排除表无交集
        for cmd in RPC_WHITELIST {
            assert!(!EXCLUDED.contains(cmd), "{} 同时在白名单与排除表", cmd);
        }
        // 白名单内无重复
        let mut sorted = RPC_WHITELIST.to_vec();
        sorted.sort_unstable();
        let len = sorted.len();
        sorted.dedup();
        assert_eq!(sorted.len(), len, "RPC_WHITELIST 存在重复项");
    }

    /// lib.rs 解析函数自身行为锚定（属性/注释行跳过、name 段提取）
    #[test]
    fn rpc_解析注册命令函数() {
        let src = r#"
            .invoke_handler(tauri::generate_handler![
                commands::get_home_data,
                // Provider Commands
                #[cfg(target_os = "windows")]
                installer::download_and_install_git,
                commands::update_app_config,
            ])
            .run(tauri::generate_context!())
        "#;
        assert_eq!(
            parse_registered_commands(src),
            vec!["get_home_data", "download_and_install_git", "update_app_config"]
        );
    }

    /// StateSection 线上格式对拍（协议兼容：前端状态中心消费同一组小写字符串）
    #[test]
    fn rpc_section字符串格式锚定() {
        let all = [
            (StateSection::Skills, "skills"),
            (StateSection::Agents, "agents"),
            (StateSection::Mcp, "mcp"),
            (StateSection::Plugins, "plugins"),
            (StateSection::Providers, "providers"),
            (StateSection::Config, "config"),
            (StateSection::Projects, "projects"),
        ];
        for (sec, s) in all {
            assert_eq!(sec.as_str(), s);
        }
    }

    #[tokio::test]
    async fn rpc_未知命令被拒绝() {
        let args = Map::new();
        let err = dispatch("download_and_install_claude", &args).await.unwrap_err();
        assert!(err.contains("not allowed"));

        let err = dispatch("nonexistent_cmd", &args).await.unwrap_err();
        assert!(err.contains("not allowed"));
    }

    /// installer 系统级命令整体排除，但纯本地只读的 get_installed_claude_version
    /// 必须放行（手机端 Claude CLI 版本页展示依赖；曾因一刀切排除报 not allowed）
    #[tokio::test]
    async fn rpc_已装版本号只读命令放行() {
        let args = Map::new();
        // 不再被白名单拒绝（调用成功与否取决于本机是否装了 claude，只断言通过白名单）
        let result = dispatch("get_installed_claude_version", &args).await;
        assert!(
            result.is_ok() || !result.unwrap_err().contains("not allowed"),
            "get_installed_claude_version 不应被 not allowed 拒绝"
        );
    }

    #[tokio::test]
    async fn rpc_只读命令可执行() {
        let mut args = Map::new();
        args.insert("limit".to_string(), serde_json::json!(3));
        let v = dispatch("get_projects", &args).await.unwrap();
        assert!(v.is_array());
    }

    #[tokio::test]
    async fn rpc_必填参数缺失报错() {
        let args = Map::new();
        let err = dispatch("get_project_info", &args).await.unwrap_err();
        assert!(err.contains("missing required arg"));
    }

    #[tokio::test]
    async fn rpc_参数camelCase键提取() {
        let mut args = Map::new();
        // Tauri invoke 惯例：Rust snake_case 参数名 ↔ JS camelCase 键
        args.insert("projectPath".to_string(), serde_json::json!("Z:/__no_such__"));
        let v = dispatch("get_session_count", &args).await.unwrap();
        assert_eq!(v, serde_json::json!(0));
    }

    #[test]
    fn rpc_snake转camel() {
        assert_eq!(snake_to_camel("project_path"), "projectPath");
        assert_eq!(snake_to_camel("id"), "id");
        assert_eq!(snake_to_camel("provider_ids"), "providerIds");
    }

    /// 文件传输命令注入覆盖客户端伪造值（安全前提：会话归属/限额选档不可伪造）
    #[test]
    fn rpc_文件命令注入权威覆盖() {
        // 客户端伪造 camelCase 注入键 → 被覆盖
        let mut args = Map::new();
        args.insert("connId".into(), serde_json::json!(999_999));
        args.insert("viaRelay".into(), serde_json::json!(false));
        inject_file_transfer_args(1001, "file_upload_chunk", &mut args, true);
        assert_eq!(args["connId"], serde_json::json!(1001));
        assert_eq!(args["viaRelay"], serde_json::json!(true));
        // take_arg 优先 camelCase：注入值胜出
        let got: Option<u64> = take_arg(&args, "conn_id").unwrap();
        assert_eq!(got, Some(1001));

        // 客户端 snake_case 变体无法绕过（camelCase 注入优先）
        let mut args = Map::new();
        args.insert("conn_id".into(), serde_json::json!(42));
        args.insert("via_relay".into(), serde_json::json!(false));
        inject_file_transfer_args(1002, "file_download_begin", &mut args, true);
        let got: Option<u64> = take_arg(&args, "conn_id").unwrap();
        assert_eq!(got, Some(1002));

        // 非文件命令不注入
        let mut args = Map::new();
        inject_file_transfer_args(1003, "get_projects", &mut args, false);
        assert!(args.is_empty());
    }
}
