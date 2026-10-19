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
mod env;
#[cfg(test)]
mod installer;
#[cfg(test)]
mod platform;
#[cfg(test)]
mod remote_state;
#[cfg(test)]
mod remote_host;
#[cfg(test)]
mod dir_listing;

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
