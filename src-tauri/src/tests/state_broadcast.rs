//! 写命令状态广播守卫测试
//!
//! 广播实现已收敛到 `#[cc_command(write = "section")]` 属性宏（函数出口
//! Ok 自动 emit_state_changed，桌面 invoke 与远程 rpc 同一口径）。守卫对拍
//! 两侧声明的一致性：
//! 1. **注册表差集**：RPC 白名单中写前缀命令（is_write_cmd，豁免清单除外）
//!    必须登记 write(...)（防新增写命令漏登记 → 状态不同步复发）
//! 2. **正向对拍**：registry! 登记 write(Sect) 的命令，commands.rs 函数前
//!    必须有 `#[cc_command(write = "sect")]` 属性（宏才是广播的执行者）
//! 3. **反向对拍**：源码中每个 #[cc_command(...)] 所属命令的 registry! 分类
//!    必须与属性一致（防拼写错误静默失效 / 漏登记 / 读写分类漂移）
//! section 值合法性由 StateSection 枚举类型在编译期保证

use crate::rpc_dispatch::{
    is_remote_write, is_write_cmd, write_section, RPC_WHITELIST, WRITE_EXEMPT,
};

/// commands.rs 中 `pub async fn {cmd}` 签名前的属性窗口（覆盖 doc 注释与
/// #[cc_command]/#[tauri::command] 属性；300 字符足够，函数体不含属性文本）
fn command_attr_window(src: &str, cmd: &str) -> Option<String> {
    let sig = format!("pub async fn {}", cmd);
    let pos = src.find(&sig)?;
    // 回退到上一个函数签名之后（窗口只含本函数的 doc 注释与属性，
    // 防短函数场景固定长度窗口跨函数误取上一函数的宏属性）
    let start = src[..pos]
        .rfind("pub async fn")
        .map(|p| p + "pub async fn".len())
        .unwrap_or(0);
    Some(src[start..pos].to_string())
}

/// 提取源码中全部 #[cc_command 属性及其所属命令名（向后最近的 pub async fn）
fn cc_command_decls(src: &str) -> Vec<(String, String)> {
    let mut out = Vec::new();
    let mut search_from = 0;
    while let Some(rel) = src[search_from..].find("#[cc_command") {
        let attr_start = search_from + rel;
        let attr_end = attr_start + src[attr_start..]
            .find("]")
            .expect("#[cc_command 属性未闭合");
        let attr = src[attr_start..=attr_end].to_string();
        let fn_rel = src[attr_end..]
            .find("pub async fn ")
            .expect("#[cc_command 后无函数定义");
        let fn_start = attr_end + fn_rel + "pub async fn ".len();
        let name_end = src[fn_start..]
            .find('(')
            .expect("函数签名无参数列表");
        out.push((src[fn_start..fn_start + name_end].trim().to_string(), attr));
        search_from = attr_end;
    }
    out
}

/// 守卫 1：白名单中的写命令必须登记 write(...)（豁免清单除外）。
/// is_remote_write 由 registry! 生成——write(Sect)/write(None) 均算登记
#[test]
fn 状态广播_白名单写命令必须登记注册表() {
    for cmd in RPC_WHITELIST {
        if !is_write_cmd(cmd) || WRITE_EXEMPT.contains(cmd) {
            continue;
        }
        assert!(
            is_remote_write(cmd),
            "写命令 '{}' 命中写前缀但未在 registry! 登记 write(...)；\
             新增写命令必须登记（write(Sect) 接入 state-changed 广播，\
             write(None) 表示写但不广播），或确认只读后加入 WRITE_EXEMPT",
            cmd
        );
    }
}

/// 守卫 2（正向）：registry! 的登记与 commands.rs 的宏属性一一对应——
/// write(Sect) → #[cc_command(write = "sect")]、write(None) → #[cc_command(write)]、
/// read() → 无属性。宏是广播的唯一执行者，漏标即漏广播
#[test]
fn 状态广播_注册表登记与宏属性一致() {
    let src = include_str!("../commands.rs");
    for cmd in RPC_WHITELIST {
        let window = command_attr_window(src, cmd)
            .unwrap_or_else(|| panic!("commands.rs 中找不到命令 '{}' 的函数", cmd));
        if let Some(section) = write_section(cmd) {
            let expect = format!("#[cc_command(write = \"{}\")]", section.as_str());
            assert!(
                window.contains(&expect),
                "命令 '{}' registry! 登记 write({:?}) 但函数前缺 {}；\
                 广播由 #[cc_command] 在函数出口执行，漏标即漏广播（守卫强制）",
                cmd,
                section,
                expect
            );
        } else if is_remote_write(cmd) {
            assert!(
                window.contains("#[cc_command(write)]"),
                "命令 '{}' registry! 登记 write(None)，函数前应有裸标记 #[cc_command(write)]",
                cmd
            );
        } else {
            assert!(
                !window.contains("#[cc_command("),
                "命令 '{}' registry! 登记为 read()，函数前不应出现 #[cc_command；\
                 若改为写命令需同步 registry! 与宏属性",
                cmd
            );
        }
    }
}

/// 守卫 3（反向）：源码中每个 #[cc_command 声明的分类必须与 registry! 一致
/// （防 section 拼写错误静默广播错 section、防新写命令只加宏漏登记白名单）
#[test]
fn 状态广播_宏属性声明必须在注册表中() {
    let src = include_str!("../commands.rs");
    let decls = cc_command_decls(src);
    assert!(
        decls.len() >= 16,
        "commands.rs 宏属性解析异常（{} 条），检查 #[cc_command 使用处",
        decls.len()
    );
    for (name, attr) in decls {
        let listed = RPC_WHITELIST.contains(&name.as_str());
        assert!(
            listed,
            "命令 '{}' 标了 {} 但不在 registry! 白名单中（远程不可达或漏登记）",
            name, attr
        );
        if let Some(section_str) = attr
            .strip_prefix("#[cc_command(write = \"")
            .and_then(|rest| rest.strip_suffix("\")]"))
        {
            let section = write_section(&name)
                .unwrap_or_else(|| panic!("命令 '{}' 标 {} 但 registry! 未登记广播 section", name, attr));
            assert_eq!(
                section.as_str(),
                section_str,
                "命令 '{}' 宏属性 section 与 registry! 登记不一致",
                name
            );
        }
    }
}

/// 宏行为锚定：#[cc_command(write = "...")] 在函数出口广播——成功（Ok）才
/// 广播对应 section、失败不广播。广播代码由宏展开生成（函数体源码无 emit），
/// 守卫 2/3 只对拍声明，本测试锚定宏展开语义本身（含 section 字符串 → 枚举
/// 变体名的转换："config" → StateSection::Config）。
/// HOOK_TEST_LOCK：EVENT_HOOK 是全局单例，与其他整体 set/clear hook 的测试
/// （如 remote_state 的 rename 半链）串行执行，防并行互相覆盖
#[tokio::test]
async fn 状态广播_宏出口成功才emit() {
    use cc_box_macros::cc_command;
    use std::sync::{Arc, Mutex as StdMutex};

    #[cc_command(write = "config")]
    async fn probe_ok() -> Result<u32, String> {
        Ok(7)
    }
    #[cc_command(write = "skills")]
    async fn probe_err() -> Result<u32, String> {
        Err("boom".into())
    }

    struct HookGuard;
    impl Drop for HookGuard {
        fn drop(&mut self) {
            crate::remote_state::clear_event_hook();
        }
    }

    let _lock = crate::tests::HOOK_TEST_LOCK.lock();
    let received: Arc<StdMutex<Vec<(String, String)>>> = Arc::new(StdMutex::new(Vec::new()));
    let sink = received.clone();
    crate::remote_state::set_event_hook(Box::new(move |name, id, _data| {
        sink.lock().unwrap().push((name.to_string(), id.to_string()));
    }));
    // 断言失败也清 hook（RAII），防泄漏串扰后续测试
    let _guard = HookGuard;

    assert_eq!(probe_ok().await.unwrap(), 7);
    assert!(probe_err().await.is_err());

    let got = received.lock().unwrap().clone();
    assert!(
        got.iter().any(|(n, id)| n == "state-changed" && id == "config"),
        "成功路径未广播 state-changed(config)，实际: {:?}",
        got
    );
    assert!(
        !got.iter().any(|(n, id)| n == "state-changed" && id == "skills"),
        "失败路径不应广播 state-changed(skills)，实际: {:?}",
        got
    );
}
