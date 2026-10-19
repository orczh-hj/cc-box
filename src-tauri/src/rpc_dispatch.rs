//! RPC 命令分发器（统一管理：命令单一注册链）
//!
//! 浏览器端（经远程 WS）调用 Tauri command 的唯一入口。白名单用 `rpc_table!`
//! 声明宏集中定义——一行一命令（名字 → 参数签名 → commands.rs 函数），宏展开
//! 生成参数提取与 match，新增命令只写一行。
//!
//! **差集守卫**：测试断言 `lib.rs invoke_handler 注册表 == RPC_WHITELIST + EXCLUDED`
//! （两项并集），新增 Tauri command 时若忘记在此登记，测试失败强制评估。
//! PTY 操作（spawn/input/resize/kill/claim）不走 RPC，一律走远程协议专用消息
//! （session.create 带项目白名单与并发上限、attach 独占校验）。

use cc_box_remote_protocol::ServerMessage;
use serde::de::DeserializeOwned;
use serde_json::{Map, Value};

/// 显式排除（不可经远程调用）：installer 13 个系统级 + PTY 6 个（走专用消息）
/// + 文件管理器/新实例（桌面级）+ app_path/log（仅桌面有意义）
pub const EXCLUDED: &[&str] = &[
    // installer（系统级：下载/安装/杀进程）
    "get_latest_versions",
    "check_installed_versions",
    "check_claude_cli_update",
    "check_claude_running",
    "kill_claude_processes",
    "download_and_install_claude",
    "cancel_auto_install",
    "get_installed_claude_version",
    "list_claude_versions",
    "download_claude_version",
    "cancel_claude_download",
    "install_claude_version",
    #[cfg(target_os = "windows")]
    "download_and_install_git",
    // PTY（走 session.create / pty.input 等专用消息，有白名单与独占校验）
    "pty_spawn",
    "pty_input",
    "pty_resize",
    "pty_kill",
    "pty_kill_all",
    "pty_claim",
    // 桌面级
    "open_in_file_manager",
    "spawn_new_instance",
    "get_app_path",
    // 安全：中继重定向=窃取 device_key 的通道（详见 rpc_table 尾注释）
    "set_relay_url",
];

/// 一行一命令：名字（与 Tauri command 同名）→ 参数（Rust snake_case 签名）
/// → handler。参数经 camelCase 键从 args 对象提取（与 Tauri invoke 惯例一致）。
macro_rules! rpc_table {
    ( $( $cmd:literal => | $($arg:ident : $ty:ty),* | $handler:path ),* $(,)? ) => {
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
    };
}

rpc_table! {
    // ==== 检查 / 首页 ====
    "get_home_data" => |project_limit: Option<usize>, session_limit: Option<usize>| crate::commands::get_home_data,
    "get_check_results" => | | crate::commands::get_check_results,
    "run_checks" => | | crate::commands::run_checks,
    "test_communication" => |message: String| crate::commands::test_communication,
    "get_system_locale" => | | crate::commands::get_system_locale,
    // 前端日志上报（高频调用，浏览器端统一进桌面日志）
    "log_message" => |level: String, message: String| log_message_rpc,

    // ==== 项目 / 会话（只读查询） ====
    "get_projects" => |limit: Option<usize>, offset: Option<usize>| crate::commands::get_projects,
    "list_directories" => |path: Option<String>| crate::commands::list_directories,
    "get_project_info" => |path: String| crate::commands::get_project_info,
    "get_sessions" => |project_path: String, limit: Option<usize>, offset: Option<usize>| crate::commands::get_sessions,
    "get_session_count" => |project_path: String| crate::commands::get_session_count,
    "get_all_recent_sessions" => |limit: Option<usize>| crate::commands::get_all_recent_sessions,
    "get_session_details" => |project_path: String, session_id: String| crate::commands::get_session_details,
    "search_session_messages" => |project_path: String, query: String, limit: Option<usize>| crate::commands::search_session_messages,
    "get_project_config" => |project_path: String| crate::commands::get_project_config,
    "save_last_project" => |path: String| crate::commands::save_last_project,

    // ==== 配置 ====
    // get_app_config 经 rpc 脱敏：remote 凭证（token/deviceId/deviceKey）不下发
    // 浏览器端——LAN 明文 WS 下被动嗅探即得 device_key，可冒充桌面注册中继
    "get_app_config" => | | get_app_config_sanitized,
    "update_app_config" => |updates: Value| crate::commands::update_app_config,
    "get_default_claude_options" => | | crate::commands::get_default_claude_options,
    "save_default_claude_options" => |options: crate::store::DefaultClaudeOptions| crate::commands::save_default_claude_options,

    // ==== Agents / Skills / MCP / Plugins ====
    "get_all_agents" => |project_path: String| crate::commands::get_all_agents,
    "get_all_skills" => |project_path: String| crate::commands::get_all_skills,
    "get_all_mcp_servers" => |project_path: String| crate::commands::get_all_mcp_servers,
    "get_all_plugins" => |project_path: String| crate::commands::get_all_plugins,
    "set_skill_enabled" => |name: String, enabled: bool| crate::commands::set_skill_enabled,
    "set_agent_enabled" => |name: String, enabled: bool| crate::commands::set_agent_enabled,
    "set_mcp_server_enabled" => |name: String, enabled: bool| crate::commands::set_mcp_server_enabled,
    "set_plugin_enabled" => |plugin_id: String, enabled: bool| crate::commands::set_plugin_enabled,
    "get_mcp_server_detail" => |project_path: String, server_name: String, force_refresh: bool| crate::commands::get_mcp_server_detail,

    // ==== Provider ====
    "get_providers_config" => | | crate::commands::get_providers_config,
    "save_providers_config" => |config: crate::providers::ProvidersConfig| crate::commands::save_providers_config,
    "activate_provider" => |provider_id: String| crate::commands::activate_provider,
    "create_provider" => |name: String, settings_config: Value, website_url: Option<String>, category: Option<String>, icon: Option<String>, icon_color: Option<String>, meta: Option<crate::providers::ProviderMeta>| crate::commands::create_provider,
    "update_provider" => |id: String, name: Option<String>, settings_config: Option<Value>, notes: Option<String>, meta: Option<crate::providers::ProviderMeta>| crate::commands::update_provider,
    "delete_provider" => |id: String| crate::commands::delete_provider,
    "update_provider_sort_order" => |provider_ids: Vec<String>| crate::commands::update_provider_sort_order,
    "update_common_config" => |enabled: bool, settings: Value| crate::commands::update_common_config,
    "check_cc_switch_db_exists" => | | crate::commands::check_cc_switch_db_exists,
    "import_from_cc_switch" => | | crate::commands::import_from_cc_switch,
    "test_provider_connection" => |provider_id: String| crate::commands::test_provider_connection,

    // ==== 远程设置（浏览器端查看/管理远程开关） ====
    "get_remote_status" => | | crate::commands::get_remote_status,
    "set_remote_enabled" => |enabled: bool| crate::commands::set_remote_enabled,
    // set_relay_url 不远程暴露：已连客户端可把中继重定向到攻击者服务器，
    // 下次注册即送出 device_key（身份窃取）。中继地址由环境变量/内置默认管理
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
fn is_write_cmd(cmd: &str) -> bool {
    let prefixes = [
        "update_", "save_", "set_", "delete_", "create_", "activate_", "import_", "run_",
    ];
    prefixes.iter().any(|p| cmd.starts_with(p))
}

/// RPC 入口（remote_state 的 RpcCall 分支调用）：线程内执行（dispatch 是 async；
/// 调用频率低，独立线程 + current_thread runtime，与 SessionCreate 先例一致）
pub fn handle_rpc(conn_id: u64, rpc_id: u64, cmd: &str, args: &Value) {
    let cmd = cmd.to_string();
    let args = args.as_object().cloned().unwrap_or_default();
    if is_write_cmd(&cmd) {
        log::info!("[remote] rpc write cmd '{}' from conn {}", cmd, conn_id);
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

    /// lib.rs invoke_handler 注册表全集（与注册处同步维护；
    /// 差集守卫强制新增 command 时在 RPC_WHITELIST / EXCLUDED 间做决策）
    const REGISTERED: &[&str] = &[
        "get_home_data", "get_check_results", "run_checks",
        "pty_spawn", "pty_input", "pty_resize", "pty_kill", "pty_kill_all", "pty_claim",
        "get_projects", "get_project_info", "get_sessions", "get_session_count",
        "list_directories",
        "get_all_recent_sessions", "get_session_details", "search_session_messages",
        "get_app_config", "update_app_config", "get_system_locale",
        "get_default_claude_options", "save_default_claude_options", "save_last_project",
        "open_in_file_manager", "get_project_config",
        "get_all_agents", "get_all_skills", "get_all_mcp_servers", "get_all_plugins",
        "set_skill_enabled", "set_agent_enabled", "set_mcp_server_enabled", "set_plugin_enabled",
        "get_mcp_server_detail", "test_communication", "get_app_path", "spawn_new_instance",
        "log_message",
        "get_latest_versions", "check_installed_versions", "check_claude_cli_update",
        "check_claude_running", "kill_claude_processes", "download_and_install_claude",
        "cancel_auto_install", "get_installed_claude_version", "list_claude_versions",
        "download_claude_version", "cancel_claude_download", "install_claude_version",
        #[cfg(target_os = "windows")]
        "download_and_install_git",
        "get_providers_config", "save_providers_config", "activate_provider",
        "create_provider", "update_provider", "delete_provider", "update_provider_sort_order",
        "update_common_config", "check_cc_switch_db_exists", "import_from_cc_switch",
        "test_provider_connection",
        "get_remote_status", "set_remote_enabled", "set_relay_url",
    ];

    /// 差集守卫：每个注册命令必须在 RPC 白名单或排除表中（防新增命令漏评估）
    #[test]
    fn rpc_白名单与排除表覆盖全部注册命令() {
        for cmd in REGISTERED {
            let listed = RPC_WHITELIST.contains(cmd) || EXCLUDED.contains(cmd);
            assert!(
                listed,
                "command '{}' 未在 rpc_dispatch 的 RPC_WHITELIST/EXCLUDED 中登记；\
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

    #[tokio::test]
    async fn rpc_未知命令被拒绝() {
        let args = Map::new();
        let err = dispatch("download_and_install_claude", &args).await.unwrap_err();
        assert!(err.contains("not allowed"));

        let err = dispatch("nonexistent_cmd", &args).await.unwrap_err();
        assert!(err.contains("not allowed"));
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
}
