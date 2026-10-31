#[cfg(test)]
mod providers;
#[cfg(test)]
mod store;
#[cfg(test)]
mod hook_events;
#[cfg(test)]
mod checks;
#[cfg(test)]
mod pty;
#[cfg(test)]
mod pty_decoder;
#[cfg(test)]
mod mcp;
#[cfg(test)]
mod installer;
#[cfg(test)]
mod platform;
#[cfg(test)]
mod remote_state;
#[cfg(test)]
mod remote_host;
#[cfg(test)]
mod gateway;
#[cfg(test)]
mod broker_ipc;
#[cfg(test)]
mod broker_lifecycle;
#[cfg(test)]
mod dir_listing;
#[cfg(test)]
mod state_broadcast;
#[cfg(test)]
mod file_transfer;
#[cfg(test)]
mod plugin_market;
#[cfg(test)]
mod store_edit;

/// EVENT_HOOK 全局单例的测试互斥锁：整体 set/clear hook 的测试（remote_state
/// 的 rename 半链、state_broadcast 的宏出口锚定）必须串行执行——并行时互相
/// 覆盖/清除对方的 hook 会导致断言偶发失败（hook 是 remote_state 全局态）
#[cfg(test)]
pub(crate) static HOOK_TEST_LOCK: std::sync::LazyLock<parking_lot::Mutex<()>> =
    std::sync::LazyLock::new(|| parking_lot::Mutex::new(()));

/// remote_server 全局态（RUNNING/SERVER_TASK/CURRENT_PORT）的测试互斥锁：
/// start/stop 级测试必须串行执行，并行互相 stop 对方的 server
#[cfg(test)]
pub(crate) static REMOTE_SERVER_TEST_LOCK: std::sync::LazyLock<parking_lot::Mutex<()>> =
    std::sync::LazyLock::new(|| parking_lot::Mutex::new(()));

/// GUI_CONFIG_DIR_OVERRIDE 全局的测试互斥锁：TestConfigDirGuard 用户
/// （remote_state 的 Rpc 写命令链、remote_host 的 start/stop 链）必须串行——
/// 并行时一方 Drop 清除 override，另一方读配置回退真实 ~/.cc-box（端口变
/// 8747 等，测试随环境漂移）
#[cfg(test)]
pub(crate) static CONFIG_DIR_TEST_LOCK: std::sync::LazyLock<parking_lot::Mutex<()>> =
    std::sync::LazyLock::new(|| parking_lot::Mutex::new(()));

/// UPLINK 全局单例（remote_state 静态上行通道）的测试互斥锁：设置方
/// （成员模式跳过本地广播测试会 set_uplink(Some)）与一切依赖「uplink 为
/// None」行为的测试（如订阅后 remove_pty 期望本地 SessionExit 广播）都必须
/// 持锁——remove_pty 的本地广播被 `!has_uplink()` 守卫，并行时帧会被静默吞掉
#[cfg(test)]
pub(crate) static UPLINK_TEST_LOCK: std::sync::LazyLock<parking_lot::Mutex<()>> =
    std::sync::LazyLock::new(|| parking_lot::Mutex::new(()));

/// GUI 配置目录隔离（RAII）：指向独立临时目录，Drop 还原并清理。落盘类
/// 测试必须使用（走真实命令链路意味着真实写 ~/.cc-box/config.json）
#[cfg(test)]
pub(crate) struct TestConfigDirGuard(std::path::PathBuf);

#[cfg(test)]
impl TestConfigDirGuard {
    pub(crate) fn new(tag: &str) -> Self {
        let dir = std::env::temp_dir()
            .join(format!("cc-box-rpc-test-{}-{}", tag, std::process::id()));
        let _ = std::fs::create_dir_all(&dir);
        *crate::store::GUI_CONFIG_DIR_OVERRIDE.lock() = Some(dir.clone());
        Self(dir)
    }
}

#[cfg(test)]
impl Drop for TestConfigDirGuard {
    fn drop(&mut self) {
        *crate::store::GUI_CONFIG_DIR_OVERRIDE.lock() = None;
        let _ = std::fs::remove_dir_all(&self.0);
    }
}

#[cfg(test)]
mod build_claude_args_tests {
    use crate::pty::build_claude_args;

    /// 参数拼接规则锚定：与桌面前端 buildClaudeArgs（XTermTerminal.vue）语义一致
    #[test]
    fn BuildClaudeArgs_规则锚定() {
        // 空
        assert!(build_claude_args(None, false, "").is_empty());
        // 仅 resume
        assert_eq!(
            build_claude_args(Some("abc"), false, ""),
            vec!["--resume", "abc"]
        );
        // resume + skip + custom（空白拆分，多空格容忍）
        assert_eq!(
            build_claude_args(Some("abc"), true, "--model opus   --verbose"),
            vec!["--resume", "abc", "--dangerously-skip-permissions", "--model", "opus", "--verbose"]
        );
        // 空 resume 字符串（浏览器端 resume 为空串时跳过）
        assert_eq!(build_claude_args(Some(""), false, ""), Vec::<String>::new());
        // 仅 skip + custom
        assert_eq!(
            build_claude_args(None, true, "--model opus"),
            vec!["--dangerously-skip-permissions", "--model", "opus"]
        );
    }
}

#[cfg(test)]
mod normalize_relay_url_tests {
    use crate::remote_client::normalize_relay_url;

    /// 中继地址归一化：显式 scheme 优先；裸地址默认 ws://（免证书端口直连）
    #[test]
    fn NormalizeRelayUrl_规则锚定() {
        // 裸地址（带端口）→ ws://（用户免证书直连 relay 进程）
        assert_eq!(normalize_relay_url("orczh.top:8800"), "ws://orczh.top:8800");
        // https → wss（有证书，走 nginx 443）
        assert_eq!(normalize_relay_url("https://cc.orczh.top"), "wss://cc.orczh.top");
        // http → ws
        assert_eq!(normalize_relay_url("http://orczh.top:8800"), "ws://orczh.top:8800");
        // ws/wss 原样
        assert_eq!(normalize_relay_url("ws://a.b:1"), "ws://a.b:1");
        assert_eq!(normalize_relay_url("wss://a.b"), "wss://a.b");
        // 尾部斜杠与 /ws/device 路径剥离
        assert_eq!(normalize_relay_url("orczh.top:8800/"), "ws://orczh.top:8800");
        assert_eq!(
            normalize_relay_url("https://cc.orczh.top/ws/device"),
            "wss://cc.orczh.top"
        );
    }
}

#[cfg(test)]
mod resolve_relay_url_tests {
    use crate::remote_client::resolve_relay_url;

    /// 中继地址解析优先级：环境变量 CC_BOX_RELAY_URL > 内置默认 wss://orczh.top:8800
    /// （生产 8800 由 nginx TLS 终止；中继即默认远程方式，环境变量仅本地调试覆盖）
    #[test]
    fn ResolveRelayUrl_默认与环境变量覆盖() {
        // 无环境变量 → 内置默认（wss 原样保留，TLS 经 nginx 终止）
        std::env::remove_var("CC_BOX_RELAY_URL");
        assert_eq!(resolve_relay_url(), "wss://orczh.top:8800");

        // 环境变量覆盖（本地 relay 调试场景，裸地址归一化 ws://）
        std::env::set_var("CC_BOX_RELAY_URL", "192.168.5.105:8800");
        assert_eq!(resolve_relay_url(), "ws://192.168.5.105:8800");
        std::env::remove_var("CC_BOX_RELAY_URL");
    }
}
