//! Plugin 市场模块测试：输入校验、市场清单读取（注入临时目录 + installed 表）、
//! 带超时命令执行、安装生命周期的前置校验。

use std::collections::HashMap;
use std::path::Path;

use crate::store::{
    get_plugin_marketplaces_in, install_plugin, install_scope_cwd, run_command_with_timeout,
    validate_install_scope, validate_marketplace_source, validate_plugin_ref, with_builtin_market,
    CustomMarket, MarketPreset,
};

// ---------- 输入校验 ----------

/// 市场来源校验：空/换行/超长拒绝；owner/repo、URL、本地路径、首尾空白接受
#[test]
fn ValidateMarketplaceSource_拒绝非法并修剪空白() {
    assert!(validate_marketplace_source("").is_err());
    assert!(validate_marketplace_source("   ").is_err());
    assert!(validate_marketplace_source("a\nb").is_err());
    assert!(validate_marketplace_source("a\rb").is_err());
    assert!(validate_marketplace_source(&"x".repeat(513)).is_err());

    // 合法形态：GitHub 简写 / URL / 本地路径
    for ok in ["anthropics/claude-plugins-official", "https://example.com/marketplace.json", "D:\\repos\\my-market"] {
        assert!(validate_marketplace_source(ok).is_ok(), "{} should be ok", ok);
    }
    // 首尾空白修剪（CLI 接收修剪后的值）
    assert_eq!(
        validate_marketplace_source("  anthropics/claude-plugins-official  ").unwrap(),
        "anthropics/claude-plugins-official"
    );
}

/// 插件引用校验：必须 name@marketplace 且不含路径分隔符
#[test]
fn ValidatePluginRef_必须有市场后缀且拒绝路径符() {
    assert!(validate_plugin_ref("paper-tool@orczh").is_ok());
    assert!(validate_plugin_ref("plain-name").is_err());
    assert!(validate_plugin_ref("a/b@c").is_err());
    assert!(validate_plugin_ref("a\\b@c").is_err());
    assert!(validate_plugin_ref("..@market").is_err());
    assert!(validate_plugin_ref("").is_err());
}

/// 安装作用域校验：仅 user/project/local；project 必须提供项目路径作为 cwd
#[test]
fn ValidateInstallScope_白名单与项目路径要求() {
    assert!(validate_install_scope("user").is_ok());
    assert!(validate_install_scope("project").is_ok());
    assert!(validate_install_scope("local").is_ok());
    assert!(validate_install_scope("global").is_err());
    assert!(validate_install_scope("").is_err());

    // project 作用域缺项目路径 → Err；提供了 → Some(路径)
    assert!(install_scope_cwd("project", None).is_err());
    assert!(install_scope_cwd("project", Some("  ")).is_err());
    let cwd = install_scope_cwd("project", Some("D:/proj")).unwrap().unwrap();
    assert_eq!(cwd, Path::new("D:/proj"));

    // 非 project/local 作用域：cwd 为 None（即使误传了 project_path 也不影响）
    assert!(install_scope_cwd("user", Some("D:/proj")).unwrap().is_none());
    // local 与 project 同为项目级：必须有项目路径（CLI 记 projectPath=CWD）
    assert!(install_scope_cwd("local", None).is_err());
    assert_eq!(
        install_scope_cwd("local", Some("D:/proj")).unwrap().unwrap(),
        Path::new("D:/proj")
    );
}

/// 安装生命周期在 spawn CLI 之前完成校验（非法输入不产生子进程）
#[test]
fn InstallPlugin_前置校验先于子进程() {
    // 缺 @marketplace 后缀
    let err = install_plugin("paper-tool", "user", None, None).unwrap_err().to_string();
    assert!(err.contains("name@marketplace"), "got: {}", err);
    // 非法作用域
    let err = install_plugin("a@b", "global", None, None).unwrap_err().to_string();
    assert!(err.contains("Invalid install scope"), "got: {}", err);
    // project 缺项目路径
    let err = install_plugin("a@b", "project", None, None).unwrap_err().to_string();
    assert!(err.contains("project_path"), "got: {}", err);
}

// ---------- 市场清单读取（注入临时目录 + installed 表） ----------

/// 构造一个市场目录：`<root>/<name>/.claude-plugin/marketplace.json`
fn make_marketplace(root: &Path, name: &str, plugins_json: &str) -> std::path::PathBuf {
    let dir = root.join(name).join(".claude-plugin");
    std::fs::create_dir_all(&dir).unwrap();
    let path = dir.join("marketplace.json");
    std::fs::write(&path, plugins_json).unwrap();
    root.join(name)
}

/// 清单读取：两个市场按名称稳定排序；installed/enabled 徽标按 `name@市场名`
/// 精确匹配注入表；tags 解析；缺字段容忍
#[test]
fn GetPluginMarketplacesIn_多市场排序与徽标() {
    let tmp = tempfile::tempdir().unwrap();
    let mkt_a = make_marketplace(
        tmp.path(),
        "market-a",
        r#"{
            "name": "market-a",
            "owner": { "name": "a" },
            "plugins": [
                { "name": "x-plugin", "description": "X desc", "version": "1.0.0",
                  "category": "dev", "tags": ["x", "fast"] },
                { "name": "y-plugin" }
            ]
        }"#,
    );
    let mkt_b = make_marketplace(
        tmp.path(),
        "market-b",
        r#"{
            "name": "market-b",
            "owner": { "name": "b" },
            "plugins": [
                { "name": "z-plugin", "description": "Z desc" }
            ]
        }"#,
    );

    // known_marketplaces.json 故意先写 market-b 再写 market-a，验证输出按名称排序
    let known = format!(
        r#"{{ "market-b": {{ "installLocation": "{}" }}, "market-a": {{ "installLocation": "{}", "lastUpdated": "2026-04-17T12:24:21.971Z" }} }}"#,
        mkt_b.to_string_lossy().replace('\\', "\\\\"),
        mkt_a.to_string_lossy().replace('\\', "\\\\"),
    );
    let plugins_dir = tmp.path().join("plugins");
    std::fs::create_dir_all(&plugins_dir).unwrap();
    std::fs::write(plugins_dir.join("known_marketplaces.json"), known).unwrap();

    // 独立真值：x-plugin@market-a 已安装且启用；z-plugin@market-b 已安装但停用；
    // y-plugin 未安装
    let mut installed = HashMap::new();
    installed.insert(
        "x-plugin@market-a".to_string(),
        vec![crate::store::InstalledLite { enabled: true, version: None, scope: "user".into(), project_path: None }],
    );
    installed.insert(
        "z-plugin@market-b".to_string(),
        vec![crate::store::InstalledLite { enabled: false, version: None, scope: "user".into(), project_path: None }],
    );

    let mut out = get_plugin_marketplaces_in(tmp.path(), &installed, &[], &[], "").unwrap();
    assert_eq!(out.len(), 2);
    // 名称稳定排序：market-a 在前
    assert_eq!(out[0].name, "market-a");
    assert_eq!(out[1].name, "market-b");

    let a = &out[0];
    // 本地缓存时间透传（前端据此提示是否需要刷新；市场仓库由 Claude 克隆）
    assert_eq!(a.last_updated.as_deref(), Some("2026-04-17T12:24:21.971Z"));
    assert_eq!(out[1].last_updated, None);
    assert_eq!(a.plugins.len(), 2);
    // 条目按名称排序：x 在 y 前
    assert_eq!(a.plugins[0].name, "x-plugin");
    assert_eq!(a.plugins[0].description.as_deref(), Some("X desc"));
    assert_eq!(a.plugins[0].version.as_deref(), Some("1.0.0"));
    assert_eq!(a.plugins[0].category.as_deref(), Some("dev"));
    assert_eq!(a.plugins[0].tags, vec!["x".to_string(), "fast".to_string()]);
    assert!(a.plugins[0].installed);
    assert_eq!(a.plugins[0].enabled, Some(true));

    assert_eq!(a.plugins[1].name, "y-plugin");
    // 未安装：enabled 为 None（而非 false）
    assert!(!a.plugins[1].installed);
    assert_eq!(a.plugins[1].enabled, None);

    let b = &out[1];
    assert!(b.plugins[0].installed);
    assert_eq!(b.plugins[0].enabled, Some(false));
    assert_eq!(b.install_location, mkt_b.to_string_lossy());
}

/// 插件按热度（unique_installs）降序，同热度按名称；无热度数据的排最后
#[test]
fn GetPluginMarketplacesIn_按安装量排序_001() {
    let tmp = tempfile::tempdir().unwrap();
    let mkt = make_marketplace(
        tmp.path(),
        "hot",
        r#"{
            "plugins": [
                { "name": "cold-plugin" },
                { "name": "popular-b" },
                { "name": "popular-a" },
                { "name": "mid-plugin" }
            ]
        }"#,
    );
    let plugins_dir = tmp.path().join("plugins");
    std::fs::create_dir_all(&plugins_dir).unwrap();
    std::fs::write(
        plugins_dir.join("known_marketplaces.json"),
        format!(r#"{{ "hot": {{ "installLocation": "{}" }} }}"#, mkt.to_string_lossy().replace('\\', "\\\\")),
    )
    .unwrap();
    // Claude 的插件目录缓存：键为 name@marketplace
    std::fs::write(
        plugins_dir.join("plugin-catalog-cache.json"),
        r#"{
            "version": 1,
            "catalog": {
                "plugins": {
                    "popular-a@hot": { "unique_installs": 3000 },
                    "popular-b@hot": { "unique_installs": 3000 },
                    "mid-plugin@hot": { "unique_installs": 42 }
                }
            }
        }"#,
    )
    .unwrap();

    let out = get_plugin_marketplaces_in(tmp.path(), &HashMap::new(), &[], &[], "").unwrap();
    let names: Vec<&str> = out[0].plugins.iter().map(|p| p.name.as_str()).collect();
    // 独立真值：3000 并列按名称 popular-a < popular-b，其次 42，最后无数据
    assert_eq!(names, vec!["popular-a", "popular-b", "mid-plugin", "cold-plugin"]);
    assert_eq!(out[0].plugins[0].installs, Some(3000));
    assert_eq!(out[0].plugins[2].installs, Some(42));
    assert_eq!(out[0].plugins[3].installs, None);
}

/// 已装版本透传与更新判定：清单版本 vs plugins list 的已装版本（手算真值）
#[test]
fn GetPluginMarketplacesIn_已装版本与更新判定_001() {
    let tmp = tempfile::tempdir().unwrap();
    let mkt = make_marketplace(
        tmp.path(),
        "ver",
        r#"{
            "plugins": [
                { "name": "behind", "version": "2.0.0" },
                { "name": "current", "version": "1.5.0" },
                { "name": "no-version" },
                { "name": "not-installed", "version": "3.0.0" }
            ]
        }"#,
    );
    let plugins_dir = tmp.path().join("plugins");
    std::fs::create_dir_all(&plugins_dir).unwrap();
    std::fs::write(
        plugins_dir.join("known_marketplaces.json"),
        format!(r#"{{ "ver": {{ "installLocation": "{}" }} }}"#, mkt.to_string_lossy().replace('\\', "\\\\")),
    )
    .unwrap();

    // 已装：behind v1.4.0（清单 2.0.0 → 有更新）；current v1.5.0（相等 → 无更新）；
    // no-version 已装 v0.1.0（清单缺版本 → 无法判定）
    let mut installed = HashMap::new();
    for (id, ver) in [
        ("behind@ver", Some("1.4.0")),
        ("current@ver", Some("1.5.0")),
        ("no-version@ver", Some("0.1.0")),
    ] {
        installed.insert(
            id.to_string(),
            vec![crate::store::InstalledLite { enabled: true, version: ver.map(str::to_string), scope: "user".into(), project_path: None }],
        );
    }

    let out = get_plugin_marketplaces_in(tmp.path(), &installed, &[], &[], "").unwrap();
    let find = |name: &str| {
        out[0].plugins.iter().find(|p| p.name == name).unwrap().clone()
    };
    let behind = find("behind");
    assert_eq!(behind.installed_version.as_deref(), Some("1.4.0"));
    assert_eq!(behind.update_available, Some(true));
    let current = find("current");
    assert_eq!(current.update_available, Some(false));
    // 清单缺版本：不下结论
    assert_eq!(find("no-version").update_available, None);
    // 未安装：无已装版本、不判定
    let not_installed = find("not-installed");
    assert!(!not_installed.installed);
    assert_eq!(not_installed.installed_version, None);
    assert_eq!(not_installed.update_available, None);
}

/// installed 徽标按项目视角判定（与 Plugins 面板共用 plugin_matches_project_view）：
/// user 恒匹配；project 仅路径相等匹配；local 不匹配（面板同口径）
#[test]
fn GetPluginMarketplacesIn_项目视角匹配_001() {
    let tmp = tempfile::tempdir().unwrap();
    let mkt = make_marketplace(
        tmp.path(),
        "pv",
        r#"{ "plugins": [
            { "name": "u-plug" },
            { "name": "p-plug" },
            { "name": "l-plug" },
            { "name": "n-plug" }
        ] }"#,
    );
    let plugins_dir = tmp.path().join("plugins");
    std::fs::create_dir_all(&plugins_dir).unwrap();
    std::fs::write(
        plugins_dir.join("known_marketplaces.json"),
        format!(r#"{{ "pv": {{ "installLocation": "{}" }} }}"#, mkt.to_string_lossy().replace('\\', "\\\\")),
    )
    .unwrap();

    let mut installed: HashMap<String, Vec<crate::store::InstalledLite>> = HashMap::new();
    installed.insert(
        "u-plug@pv".to_string(),
        vec![crate::store::InstalledLite { enabled: true, version: None, scope: "user".into(), project_path: None }],
    );
    // 同插件两份安装：user（停用）在前 + project（proj-a，启用）在后——
    // 命中顺序必须显式优先项目份（CLI 输出顺序不受控，不能依赖 find 顺序）
    installed.insert(
        "p-plug@pv".to_string(),
        vec![
            crate::store::InstalledLite { enabled: false, version: None, scope: "user".into(), project_path: None },
            crate::store::InstalledLite { enabled: true, version: None, scope: "project".into(), project_path: Some("D:/proj-a".into()) },
        ],
    );
    installed.insert(
        "l-plug@pv".to_string(),
        vec![crate::store::InstalledLite { enabled: true, version: None, scope: "local".into(), project_path: Some("D:/proj-a".into()) }],
    );

    let find = |out: &Vec<crate::store::MarketplaceInfo>, name: &str| {
        out[0].plugins.iter().find(|p| p.name == name).unwrap().clone()
    };
    // 视角 = D:/proj-a：project 命中（enabled/matched_* 取该份）；local 不匹配
    let out = get_plugin_marketplaces_in(tmp.path(), &installed, &[], &[], "D:/proj-a").unwrap();
    assert!(find(&out, "u-plug").installed);
    let p = find(&out, "p-plug");
    assert!(p.installed);
    assert_eq!(p.enabled, Some(true));
    assert_eq!(p.matched_scope.as_deref(), Some("project"));
    assert_eq!(p.matched_project_path.as_deref(), Some("D:/proj-a"));
    // local 与 project 同口径：路径相等即相关（enabled/matched 对准该份）
    let l = find(&out, "l-plug");
    assert!(l.installed);
    assert_eq!(l.matched_scope.as_deref(), Some("local"));
    assert_eq!(l.matched_project_path.as_deref(), Some("D:/proj-a"));
    assert!(!find(&out, "n-plug").installed);

    // 视角 = 其他项目：project 份不命中，user 份兜底
    let out2 = get_plugin_marketplaces_in(tmp.path(), &installed, &[], &[], "D:/proj-b").unwrap();
    let p2 = find(&out2, "p-plug");
    assert_eq!(p2.matched_scope.as_deref(), Some("user"));
    assert_eq!(p2.enabled, Some(false));
    // **项目隔离**：l-plug 只装在 proj-a（local），其他项目视角下必须未安装
    // （市场显示「安装」而非「已安装」，面板同样不显示）
    let l2 = find(&out2, "l-plug");
    assert!(!l2.installed);
    assert_eq!(l2.matched_scope, None);
    assert_eq!(l2.matched_project_path, None);

    // 空视角（首页/旧前端未传）：仅 user 匹配
    let out3 = get_plugin_marketplaces_in(tmp.path(), &installed, &[], &[], "").unwrap();
    let p3 = find(&out3, "p-plug");
    assert_eq!(p3.matched_scope.as_deref(), Some("user"));
}

/// 清单条目无 version 时从插件源码目录的 plugin.json 读后备
/// （官方精选清单不逐条写 version，真实 bug：我的市场插件全都不显示版本）
#[test]
fn GetPluginMarketplacesIn_清单无版本读插件json后备_001() {
    let tmp = tempfile::tempdir().unwrap();
    let mkt = make_marketplace(
        tmp.path(),
        "cur",
        r#"{ "plugins": [
            { "name": "from-plugin-json", "source": "./plugins/from-plugin-json" },
            { "name": "no-both", "source": "./plugins/no-both" }
        ] }"#,
    );
    // 插件源码目录带 plugin.json（version 在其中，清单层缺失）
    let pj_dir = mkt.join("plugins").join("from-plugin-json").join(".claude-plugin");
    std::fs::create_dir_all(&pj_dir).unwrap();
    std::fs::write(pj_dir.join("plugin.json"), r#"{ "name": "from-plugin-json", "version": "2.1.0" }"#).unwrap();

    // 已装 2.0.0（清单层无版本但后备有）→ 版本显示 2.1.0 且有更新
    let mut installed = HashMap::new();
    installed.insert(
        "from-plugin-json@cur".to_string(),
        vec![crate::store::InstalledLite { enabled: true, version: Some("2.0.0".into()), scope: "user".into(), project_path: None }],
    );

    let plugins_dir = tmp.path().join("plugins");
    std::fs::create_dir_all(&plugins_dir).unwrap();
    std::fs::write(
        plugins_dir.join("known_marketplaces.json"),
        format!(r#"{{ "cur": {{ "installLocation": "{}" }} }}"#, mkt.to_string_lossy().replace('\\', "\\\\")),
    )
    .unwrap();

    let out = get_plugin_marketplaces_in(tmp.path(), &installed, &[], &[], "").unwrap();
    let hit = out[0].plugins.iter().find(|p| p.name == "from-plugin-json").unwrap();
    // 独立真值：清单层无 version → 后备 plugin.json 的 2.1.0；> 已装 2.0.0 → 有更新
    assert_eq!(hit.version.as_deref(), Some("2.1.0"));
    assert_eq!(hit.installed_version.as_deref(), Some("2.0.0"));
    assert_eq!(hit.update_available, Some(true));
    // 两处都无版本：version 为 None，不判定更新
    let miss = out[0].plugins.iter().find(|p| p.name == "no-both").unwrap();
    assert_eq!(miss.version, None);
    assert_eq!(miss.update_available, None);
}

/// 宽松版本比较（手算真值）：v 前缀、数值段、缺段补零、预发布段
#[test]
fn Store_MarketVersionNewer_宽松比较_001() {
    use crate::store::market_version_newer;
    assert!(market_version_newer("1.2.0", "1.1.9"));
    assert!(market_version_newer("v1.10", "v1.9")); // v 前缀 + 数值比较（10 > 9）
    assert!(market_version_newer("1.0.1", "1.0")); // 缺段视为 0
    assert!(market_version_newer("1.0.0-beta.2", "1.0.0-beta.1"));
    assert!(!market_version_newer("1.0.0", "1.0.0"));
    assert!(!market_version_newer("0.9", "1.0"));
    assert!(!market_version_newer("1.0", "1.0.1"));
}

/// 精选置顶/排序：pinned 全在前（按 order），随后按 order，再按安装量，最后名称
#[test]
fn GetPluginMarketplacesIn_精选置顶与排序_001() {
    let tmp = tempfile::tempdir().unwrap();
    let mkt = make_marketplace(
        tmp.path(),
        "curated",
        r#"{
            "plugins": [
                { "name": "normal-hot" },
                { "name": "featured-b", "ccbox": { "pinned": true, "order": 2 } },
                { "name": "featured-a", "category": "官方", "ccbox": { "pinned": true, "order": 1, "category": "回退分类", "icon": "🧪" } },
                { "name": "ordered", "ccbox": { "order": 1, "category": "工具" } }
            ]
        }"#,
    );
    let plugins_dir = tmp.path().join("plugins");
    std::fs::create_dir_all(&plugins_dir).unwrap();
    std::fs::write(
        plugins_dir.join("known_marketplaces.json"),
        format!(r#"{{ "curated": {{ "installLocation": "{}" }} }}"#, mkt.to_string_lossy().replace('\\', "\\\\")),
    )
    .unwrap();
    // 热度很高也不能压过精选位
    std::fs::write(
        plugins_dir.join("plugin-catalog-cache.json"),
        r#"{ "catalog": { "plugins": { "normal-hot@curated": { "unique_installs": 9000 } } } }"#,
    )
    .unwrap();

    let out = get_plugin_marketplaces_in(tmp.path(), &HashMap::new(), &[], &[], "").unwrap();
    let names: Vec<&str> = out[0].plugins.iter().map(|p| p.name.as_str()).collect();
    // 独立真值：置顶 → featured-a(1), featured-b(2)；随后 ordered(1)；再按热度 normal-hot
    assert_eq!(names, vec!["featured-a", "featured-b", "ordered", "normal-hot"]);
    assert!(out[0].plugins[0].pinned);
    assert_eq!(out[0].plugins[0].order, Some(1));
    // 分类：顶层 category 优先于 ccbox.category；图标来自 ccbox.icon
    assert_eq!(out[0].plugins[0].category.as_deref(), Some("官方"));
    assert_eq!(out[0].plugins[0].icon.as_deref(), Some("🧪"));
    // 未写顶层 category 的条目回退 ccbox.category
    assert_eq!(out[0].plugins[2].category.as_deref(), Some("工具"));
    assert!(!out[0].plugins[2].pinned);
    assert_eq!(out[0].plugins[2].icon, None);
    assert_eq!(out[0].plugins[2].order, Some(1));
    assert_eq!(out[0].plugins[3].order, None);
}

/// 写文件（自动建父目录）
fn write_file(path: &Path, content: &str) {
    std::fs::create_dir_all(path.parent().unwrap()).unwrap();
    std::fs::write(path, content).unwrap();
}

/// 组件清单解析：清单声明的路径落到插件目录 → 按类型解析（目录扫描/文件读内容），
/// 未声明 → 扫描插件目录（commands/agents/skills/hooks/.mcp.json 自动发现）
#[test]
fn GetPluginMarketplacesIn_组件清单解析() {
    let tmp = tempfile::tempdir().unwrap();
    let mkt = make_marketplace(
        tmp.path(),
        "m",
        r#"{
            "plugins": [
                {
                    "name": "full",
                    "source": "./plugins/full",
                    "agents": ["agents/rev.md"],
                    "hooks": "./hooks/hooks.json",
                    "mcpServers": [".mcp.json"],
                    "skills": ["skills/notes"]
                },
                { "name": "bare" },
                { "name": "empty" }
            ]
        }"#,
    );

    // full 插件：清单未声明 commands → 目录扫描；其余按声明解析
    let full_dir = mkt.join("plugins").join("full");
    write_file(&full_dir.join(".claude-plugin").join("plugin.json"), r#"{ "name": "full" }"#);
    write_file(&full_dir.join("commands").join("b-cmd.md"), "# b");
    write_file(&full_dir.join("commands").join("a-cmd.md"), "# a");
    write_file(&full_dir.join("agents").join("rev.md"), "# rev");
    write_file(&full_dir.join("skills").join("notes").join("SKILL.md"), "# notes");
    write_file(&full_dir.join("hooks").join("hooks.json"), r#"{ "hooks": { "Stop": [], "PreToolUse": [] } }"#);
    write_file(&full_dir.join(".mcp.json"), r#"{ "example-server": { "type": "http" } }"#);

    // bare 插件：无本地源码，但已安装缓存里可扫描
    let cache_dir = tmp
        .path()
        .join("plugins")
        .join("cache")
        .join("m")
        .join("bare")
        .join("1.0.0");
    write_file(&cache_dir.join("skills").join("scanned").join("SKILL.md"), "# scanned");

    std::fs::create_dir_all(tmp.path().join("plugins")).unwrap();
    std::fs::write(
        tmp.path().join("plugins").join("known_marketplaces.json"),
        format!(r#"{{ "m": {{ "installLocation": "{}" }} }}"#, mkt.to_string_lossy().replace('\\', "\\\\")),
    )
    .unwrap();

    let out = get_plugin_marketplaces_in(tmp.path(), &HashMap::new(), &[], &[], "").unwrap();
    // 插件按名称稳定排序：bare / empty / full
    let by_name = |n: &str| out[0].plugins.iter().find(|p| p.name == n).unwrap();

    let comps = by_name("full").components.as_ref().expect("full 应有组件清单");
    // 独立真值：目录扫描结果（排序后）
    assert_eq!(comps.commands, vec!["a-cmd".to_string(), "b-cmd".to_string()]);
    // 声明为文件路径 → 取文件名去扩展名
    assert_eq!(comps.agents, vec!["rev".to_string()]);
    // 声明 hooks 文件 → 读事件名（排序）
    assert_eq!(comps.hooks, vec!["PreToolUse".to_string(), "Stop".to_string()]);
    // 声明 mcp 文件 → 服务器名
    assert_eq!(comps.mcp_servers, vec!["example-server".to_string()]);
    // 声明 skills 目录 → 扫描含 SKILL.md 的子目录
    assert_eq!(comps.skills, vec!["notes".to_string()]);

    // 无源码但已安装缓存 → 扫描缓存目录
    let bare = by_name("bare").components.as_ref().expect("bare 应从缓存扫描");
    assert_eq!(bare.skills, vec!["scanned".to_string()]);

    // 无任何组件 → None
    assert!(by_name("empty").components.is_none());
}

/// 清单声明为纯名称（非路径）时原样展示，兼容 `{ name }` 对象数组
#[test]
fn GetPluginMarketplacesIn_纯名称声明与对象数组() {
    let tmp = tempfile::tempdir().unwrap();
    let mkt = make_marketplace(
        tmp.path(),
        "m2",
        r#"{
            "plugins": [
                {
                    "name": "named",
                    "commands": ["cmd-a.md", { "name": "cmd-b", "description": "B" }],
                    "hooks": ["PreToolUse"]
                }
            ]
        }"#,
    );
    std::fs::create_dir_all(tmp.path().join("plugins")).unwrap();
    std::fs::write(
        tmp.path().join("plugins").join("known_marketplaces.json"),
        format!(r#"{{ "m2": {{ "installLocation": "{}" }} }}"#, mkt.to_string_lossy().replace('\\', "\\\\")),
    )
    .unwrap();

    let out = get_plugin_marketplaces_in(tmp.path(), &HashMap::new(), &[], &[], "").unwrap();
    let comps = out[0].plugins[0].components.as_ref().expect("应有组件清单");
    // 无本地源码：路径声明退化为归一化名
    assert_eq!(comps.commands, vec!["cmd-a".to_string(), "cmd-b".to_string()]);
    assert_eq!(comps.hooks, vec!["PreToolUse".to_string()]);
}

/// 无 known_marketplaces.json（首次使用）→ 空列表而非报错
#[test]
fn GetPluginMarketplacesIn_无注册文件返回空列表() {
    let tmp = tempfile::tempdir().unwrap();
    let out = get_plugin_marketplaces_in(tmp.path(), &HashMap::new(), &[], &[], "").unwrap();
    assert!(out.is_empty());
}

/// marketplace.json 损坏：市场仍列出（load_error 记录原因），不拖垮整表
#[test]
fn GetPluginMarketplacesIn_单个市场损坏不影响整体() {
    let tmp = tempfile::tempdir().unwrap();
    let bad_dir = tmp.path().join("bad-mkt").join(".claude-plugin");
    std::fs::create_dir_all(&bad_dir).unwrap();
    std::fs::write(bad_dir.join("marketplace.json"), "{ not json").unwrap();

    let known = format!(
        r#"{{ "bad-mkt": {{ "installLocation": "{}" }} }}"#,
        tmp.path().join("bad-mkt").to_string_lossy().replace('\\', "\\\\"),
    );
    let plugins_dir = tmp.path().join("plugins");
    std::fs::create_dir_all(&plugins_dir).unwrap();
    std::fs::write(plugins_dir.join("known_marketplaces.json"), known).unwrap();

    let out = get_plugin_marketplaces_in(tmp.path(), &HashMap::new(), &[], &[], "").unwrap();
    assert_eq!(out.len(), 1);
    assert_eq!(out[0].name, "bad-mkt");
    assert!(out[0].load_error.is_some(), "expected load_error");
    assert!(out[0].plugins.is_empty());
}

/// file 源市场：installLocation 直接指向 .json 文件（而非目录）
#[test]
fn ReadMarketplaceJson_直接指向Json文件() {
    let tmp = tempfile::tempdir().unwrap();
    let file = tmp.path().join("my-market.json");
    std::fs::write(&file, r#"{ "name": "my-market", "plugins": [ { "name": "p1" } ] }"#).unwrap();

    let known = format!(
        r#"{{ "my-market": {{ "installLocation": "{}" }} }}"#,
        file.to_string_lossy().replace('\\', "\\\\"),
    );
    std::fs::create_dir_all(tmp.path().join("plugins")).unwrap();
    std::fs::write(tmp.path().join("plugins").join("known_marketplaces.json"), known).unwrap();

    let out = get_plugin_marketplaces_in(tmp.path(), &HashMap::new(), &[], &[], "").unwrap();
    assert_eq!(out.len(), 1);
    assert!(out[0].load_error.is_none());
    assert_eq!(out[0].plugins.len(), 1);
    assert_eq!(out[0].plugins[0].name, "p1");
}

// ---------- 推荐市场（OSS 下发数据） ----------

/// 推荐市场解析：修剪空白、丢弃空名/空来源、按名称去重（保留首个）
#[test]
fn ParseMarketPresets_清洗与去重_001() {
    let json = r#"{
        "version": 1,
        "markets": [
            { "name": " official ", "source": "anthropics/claude-plugins-official", "description": " desc ", "pinned": true, "order": 1 },
            { "name": "OFFICIAL", "source": "dup-ignored" },
            { "name": "", "source": "x" },
            { "name": "bad", "source": "   " },
            { "name": "local", "source": "D:\\repos\\market" }
        ]
    }"#;
    let out = crate::store::parse_market_presets(json).unwrap();
    assert_eq!(out.len(), 2);
    assert_eq!(out[0].name, "official");
    assert_eq!(out[0].source, "anthropics/claude-plugins-official");
    assert_eq!(out[0].description.as_deref(), Some("desc"));
    assert!(out[0].pinned);
    assert_eq!(out[0].order, Some(1));
    assert!(!out[1].pinned);
    assert_eq!(out[1].order, None);
    assert_eq!(out[1].name, "local");
}

/// 数据文件缺失/非法：返回错误（调用方回退磁盘缓存）
#[test]
fn ParseMarketPresets_非法JSON_001() {
    assert!(crate::store::parse_market_presets("{ not json").is_err());
    assert!(crate::store::parse_market_presets("{}").unwrap().is_empty());
}

// ---------- 带超时命令执行 ----------
// （用例借助 cmd/ping 模拟长任务，仅 Windows 环境执行；超时/管道逻辑平台无关）

/// 成功路径：捕获 stdout；失败路径：非零退出返回 Err 且优先 stderr
#[test]
#[cfg(windows)]
fn RunCommandWithTimeout_成功与失败路径() {
    // 成功：捕获 stdout
    let mut ok = std::process::Command::new("cmd");
    ok.args(["/C", "echo hello"]);
    let out = run_command_with_timeout(&mut ok, std::time::Duration::from_secs(10)).unwrap();
    assert_eq!(out.trim(), "hello");

    // 失败：非零退出，Err 优先含 stderr
    let mut bad = std::process::Command::new("cmd");
    bad.args(["/C", "echo boom 1>&2 & exit 3"]);
    let err = run_command_with_timeout(&mut bad, std::time::Duration::from_secs(10))
        .unwrap_err()
        .to_string();
    assert!(err.contains("boom"), "stderr 优先返回: {}", err);
    assert!(err.contains("3"), "含退出码: {}", err);
}

/// 超时路径：超时返回 Err 且进程被 kill（不悬挂）
#[test]
#[cfg(windows)]
fn RunCommandWithTimeout_超时返回Err() {
    // ping -n 30 ≈ 挂 30s（cmd 无 sleep 内建）
    let mut slow = std::process::Command::new("cmd");
    slow.args(["/C", "ping -n 30 127.0.0.1 > nul"]);
    let started = std::time::Instant::now();
    let err = run_command_with_timeout(&mut slow, std::time::Duration::from_millis(300))
        .unwrap_err()
        .to_string();
    assert!(err.contains("timed out"), "got: {}", err);
    // 实际等待 ≈ 300ms，而非 30s
    assert!(started.elapsed() < std::time::Duration::from_secs(5));
}

// ---------- 插件组件解析（commands / hooks） ----------

/// commands/hooks 解析：默认目录（commands/*.md、hooks/hooks.json、根 hooks.json）
/// 与 plugin.json 声明（commands 自定义路径、hooks 内联 object）都要覆盖；
/// 默认 commands 目录非空时不再吃声明的自定义路径
#[test]
fn ParsePluginCommandsHooks_默认目录与plugin_json声明() {
    let tmp = tempfile::tempdir().unwrap();
    let dir = tmp.path();

    std::fs::create_dir_all(dir.join("commands")).unwrap();
    std::fs::write(dir.join("commands/hello.md"), "# hello").unwrap();
    std::fs::create_dir_all(dir.join("hooks")).unwrap();
    std::fs::write(
        dir.join("hooks/hooks.json"),
        r#"{"hooks":{"SessionStart":[{"matcher":"*","hooks":[]}]}}"#,
    )
    .unwrap();
    std::fs::create_dir_all(dir.join(".claude-plugin")).unwrap();
    std::fs::write(
        dir.join(".claude-plugin/plugin.json"),
        r#"{"name":"p","hooks":{"Stop":[{"hooks":[]}]},"commands":"./custom/x.md"}"#,
    )
    .unwrap();
    std::fs::create_dir_all(dir.join("custom")).unwrap();
    std::fs::write(dir.join("custom/x.md"), "x").unwrap();

    let (commands, hooks) = crate::store::parse_plugin_commands_hooks(dir);
    // 手算真值：默认 commands 目录只有 hello.md（非空 → 声明路径被忽略）
    assert_eq!(commands, vec!["hello".to_string()]);
    // hooks = hooks.json 的 SessionStart + plugin.json 内联的 Stop
    assert_eq!(hooks, vec!["SessionStart".to_string(), "Stop".to_string()]);
}

/// hooks 路径式声明（plugin.json "hooks": "./config/hooks.json"）也要被读取
#[test]
fn ParsePluginCommandsHooks_hooks路径式声明() {
    let tmp = tempfile::tempdir().unwrap();
    let dir = tmp.path();

    std::fs::create_dir_all(dir.join(".claude-plugin")).unwrap();
    std::fs::write(
        dir.join(".claude-plugin/plugin.json"),
        r#"{"name":"p","hooks":"./config/extra-hooks.json"}"#,
    )
    .unwrap();
    std::fs::create_dir_all(dir.join("config")).unwrap();
    std::fs::write(
        dir.join("config/extra-hooks.json"),
        r#"{"hooks":{"PreToolUse":[{"hooks":[]}]}}"#,
    )
    .unwrap();

    let (commands, hooks) = crate::store::parse_plugin_commands_hooks(dir);
    assert!(commands.is_empty());
    assert_eq!(hooks, vec!["PreToolUse".to_string()]);
}

/// lsp / monitors / themes 解析：默认位置（.lsp.json、monitors/monitors.json、themes/）
/// 与 plugin.json 内联（lspServers object、monitors [{name}]）双形态；
/// hooks 也支持路径数组
#[test]
fn ParsePluginExtraComponents_lsp_monitors_themes双形态() {
    let tmp = tempfile::tempdir().unwrap();
    let dir = tmp.path();

    // 默认位置：.lsp.json / monitors/monitors.json / themes/
    std::fs::write(dir.join(".lsp.json"), r#"{"go":{"command":"gopls"}}"#).unwrap();
    std::fs::create_dir_all(dir.join("monitors")).unwrap();
    std::fs::write(
        dir.join("monitors/monitors.json"),
        r#"[{"name":"deploy-status","command":"tail -F x.log"}]"#,
    )
    .unwrap();
    std::fs::create_dir_all(dir.join("themes")).unwrap();
    std::fs::write(dir.join("themes/dracula.json"), r#"{"name":"Dracula"}"#).unwrap();

    // plugin.json：lspServers 内联 object、monitors 内联 [{name}]、hooks 路径数组、themes 额外目录
    std::fs::create_dir_all(dir.join(".claude-plugin")).unwrap();
    std::fs::write(
        dir.join(".claude-plugin/plugin.json"),
        r#"{"name":"p","lspServers":{"python":{"command":"pylsp"}},"monitors":[{"name":"error-log","command":"tail"}],"hooks":["./config/extra.json"],"themes":"./styles/"}"#,
    )
    .unwrap();
    std::fs::create_dir_all(dir.join("config")).unwrap();
    std::fs::write(dir.join("config/extra.json"), r#"{"hooks":{"PreToolUse":[{"hooks":[]}]}}"#).unwrap();
    std::fs::create_dir_all(dir.join("styles")).unwrap();
    std::fs::write(dir.join("styles/nord.json"), r#"{"name":"Nord"}"#).unwrap();

    let (commands, hooks) = crate::store::parse_plugin_commands_hooks(dir);
    assert!(commands.is_empty());
    assert_eq!(hooks, vec!["PreToolUse".to_string()]);

    let (lsp, monitors, themes) = crate::store::parse_plugin_extra_components(dir);
    // .lsp.json 的 go + plugin.json 内联的 python
    assert_eq!(lsp, vec!["go".to_string(), "python".to_string()]);
    // monitors.json 的 deploy-status + 内联的 error-log
    assert_eq!(monitors, vec!["deploy-status".to_string(), "error-log".to_string()]);
    // themes/ 的 dracula + styles/ 的 nord
    assert_eq!(themes, vec!["dracula".to_string(), "nord".to_string()]);
}

/// hooks 只读明细：按 matcher 条目展开为逐条 hook（事件/matcher/类型/摘要/超时），
/// 事件名稳定排序
#[test]
fn ParsePluginHooksDetail_逐条展开含matcher与摘要() {
    let tmp = tempfile::tempdir().unwrap();
    let dir = tmp.path();

    std::fs::create_dir_all(dir.join("hooks")).unwrap();
    std::fs::write(
        dir.join("hooks/hooks.json"),
        r#"{"hooks":{
            "Stop":[{"hooks":[{"type":"http","url":"https://x/post"}]}],
            "PostToolUse":[{"matcher":"Write|Edit","hooks":[{"type":"command","command":"fmt.sh","timeout":30}]}]
        }}"#,
    )
    .unwrap();

    let detail = crate::store::parse_plugin_hooks_detail(dir);
    // 手算真值：PostToolUse 排在 Stop 前；PostToolUse 展开出 1 条带 matcher 的 command hook
    assert_eq!(detail.len(), 2);
    assert_eq!(detail[0].event, "PostToolUse");
    assert_eq!(detail[0].matcher.as_deref(), Some("Write|Edit"));
    assert_eq!(detail[0].hook_type.as_deref(), Some("command"));
    assert_eq!(detail[0].summary.as_deref(), Some("fmt.sh"));
    assert_eq!(detail[0].timeout, Some(serde_json::json!(30)));
    // Stop：无 matcher，http 类型摘要取 url
    assert_eq!(detail[1].event, "Stop");
    assert!(detail[1].matcher.is_none());
    assert_eq!(detail[1].hook_type.as_deref(), Some("http"));
    assert_eq!(detail[1].summary.as_deref(), Some("https://x/post"));
}

// ---------- 市场浏览与注册解耦 ----------

/// 来源分类：URL(.json) / git URL / GitHub 简写 / 本地目录 / 非法
#[test]
fn ClassifyMarketSource_四类来源() {
    use crate::store::MarketSource;

    fn mk(dir: &std::path::Path) -> &std::path::Path {
        std::fs::create_dir_all(dir).unwrap();
        dir
    }
    let tmp = tempfile::tempdir().unwrap();

    assert!(matches!(
        crate::store::classify_market_source("https://x.cc/marketplace.json").unwrap(),
        MarketSource::JsonUrl(_)
    ));
    assert!(matches!(
        crate::store::classify_market_source("https://gitee.com/orczh/ccbox-market.git").unwrap(),
        MarketSource::Git(_)
    ));
    // GitHub owner/repo 简写 → git URL
    match crate::store::classify_market_source("anthropics/claude-plugins-official").unwrap() {
        MarketSource::Git(url) => assert_eq!(url, "https://github.com/anthropics/claude-plugins-official.git"),
        _ => panic!("should be git"),
    }
    assert!(matches!(
        crate::store::classify_market_source(mk(&tmp.path().join("local-market")).to_str().unwrap()).unwrap(),
        MarketSource::Dir(_)
    ));
    assert!(crate::store::classify_market_source("not a source!!").is_err());
}

/// 三来源合并：custom 已注册的合并出 origin=user；preset 过滤掉已存在名称；
/// 排序 = pinned → order → 名称
#[test]
fn GetPluginMarketplacesIn_三来源合并与排序() {
    let tmp = tempfile::tempdir().unwrap();
    let base = tmp.path();

    // 注册市场 alpha（无 lastUpdated）
    std::fs::create_dir_all(base.join("plugins")).unwrap();
    std::fs::write(
        base.join("plugins/known_marketplaces.json"),
        r#"{"alpha": {"installLocation": "REG"}}"#,
    )
    .unwrap();
    let reg_dir = tmp.path().join("reg-alpha");
    std::fs::create_dir_all(reg_dir.join(".claude-plugin")).unwrap();
    std::fs::write(
        reg_dir.join(".claude-plugin/marketplace.json"),
        r#"{"name":"alpha","plugins":[{"name":"a1","description":"d"}]}"#,
    )
    .unwrap();
    std::fs::write(
        base.join("plugins/known_marketplaces.json"),
        &format!(
            r#"{{"alpha": {{"installLocation": {}}}}}"#,
            serde_json::to_string(reg_dir.to_str().unwrap()).unwrap()
        ),
    )
    .unwrap();

    let custom = vec![crate::store::CustomMarket {
        name: "alpha".into(),
        source: "gitee.com/x/alpha.git".into(),
    }];
    let presets = vec![
        crate::store::MarketPreset {
            name: "ccbox-market".into(),
            source: "https://gitee.com/orczh/ccbox-market.git".into(),
            description: Some("精选".into()),
            pinned: true,
            order: Some(1),
            icon: None,
            category: None,
        },
        crate::store::MarketPreset {
            name: "alpha".into(), // 已存在（custom）→ 被过滤
            source: "ignored".into(),
            description: None,
            pinned: false,
            order: None,
            icon: None,
            category: None,
        },
    ];

    let out = crate::store::get_plugin_marketplaces_in(base, &HashMap::new(), &custom, &presets, "").unwrap();
    // 手算真值：ccbox-market（pinned）置顶，其次 alpha（合并 origin=user）
    assert_eq!(out.len(), 2);
    assert_eq!(out[0].name, "ccbox-market");
    assert_eq!(out[0].origin, "preset");
    assert!(!out[0].registered);
    assert_eq!(out[0].source.as_deref(), Some("https://gitee.com/orczh/ccbox-market.git"));
    assert_eq!(out[1].name, "alpha");
    assert_eq!(out[1].origin, "user");
    assert!(out[1].registered);
    // 合并保留注册市场的插件解析
    assert_eq!(out[1].plugins.len(), 1);
    assert_eq!(out[1].plugins[0].name, "a1");
}

/// 条目内联 lspServers 声明与 .lsp.json 路径声明都要解析进组件明细；
/// mcp.json（agent-plugins 新 schema）与 .mcp.json 同义
#[test]
fn ParseComponents_lsp声明与mcp_json() {
    let tmp = tempfile::tempdir().unwrap();
    let dir = tmp.path();

    // 条目内联 lspServers（object 键=服务器名）
    let entry = serde_json::json!({
        "name": "p",
        "lspServers": {"typescript": {"command": "typescript-language-server"}},
    });
    let c = crate::store::parse_components(&entry, None).unwrap();
    assert_eq!(c.lsp_servers, vec!["typescript".to_string()]);

    // 插件目录：mcp.json（无点前缀）也能扫出 MCP 服务器名
    std::fs::write(dir.join("mcp.json"), r#"{"mcpServers":{"chrome-devtools":{}}}"#).unwrap();
    let entry2 = serde_json::json!({"name": "q"});
    let c2 = crate::store::parse_components(&entry2, Some(dir)).unwrap();
    assert_eq!(c2.mcp_servers, vec!["chrome-devtools".to_string()]);
}

/// 已安装插件无条件置顶：即使未安装条目是精选（pinned/order）或高热度，
/// 排序第一优先级仍是 installed
#[test]
fn GetPluginMarketplacesIn_已安装置顶优先于精选与热度() {
    let tmp = tempfile::tempdir().unwrap();
    let mkt = make_marketplace(
        tmp.path(),
        "m",
        r#"{
            "plugins": [
                { "name": "plain", "ccbox": { "pinned": true, "order": 1 } },
                { "name": "installed-plain" },
                { "name": "hot", "ccbox": { "pinned": true, "order": 2 } }
            ]
        }"#,
    );
    let plugins_dir = tmp.path().join("plugins");
    std::fs::create_dir_all(&plugins_dir).unwrap();
    std::fs::write(
        plugins_dir.join("known_marketplaces.json"),
        format!(r#"{{ "m": {{ "installLocation": "{}" }} }}"#, mkt.to_string_lossy().replace('\\', "\\\\")),
    )
    .unwrap();
    std::fs::write(
        plugins_dir.join("plugin-catalog-cache.json"),
        r#"{"catalog": {"plugins": {"hot@m": {"unique_installs": 9999}}}}"#,
    )
    .unwrap();

    let mut installed = HashMap::new();
    installed.insert(
        "installed-plain@m".to_string(),
        vec![crate::store::InstalledLite { enabled: true, version: None, scope: "user".into(), project_path: None }],
    );

    let out = get_plugin_marketplaces_in(tmp.path(), &installed, &[], &[], "").unwrap();
    // 手算真值：installed-plain 已安装 → 第一；其余按精选排序
    assert_eq!(
        out[0].plugins.iter().map(|p| p.name.as_str()).collect::<Vec<_>>(),
        vec!["installed-plain", "plain", "hot"]
    );
}

// ---------- ccbox-market 内置个人市场 ----------

fn preset_of(name: &str, source: &str) -> MarketPreset {
    MarketPreset {
        name: name.into(),
        source: source.into(),
        description: None,
        pinned: false,
        order: None,
        icon: None,
        category: None,
    }
}

/// ccbox-market 默认进入个人市场：列表为空时按 preset 来源合成（preset 缺失回退内置源），
/// 已存在（含同名不同大小写）不重复添加——对应全新机器打开市场看不到官方市场、
/// 点精选「＋」被保留名守卫拒绝的 bug
#[test]
fn WithBuiltinMarket_为空时合成且不重复() {
    // 空列表 + preset 提供来源 → 合成，用 preset 的源（可热更新换仓库）
    let presets = vec![preset_of("ccbox-market", "https://gitee.com/orczh/ccbox-market.git")];
    let out = with_builtin_market(&[], &presets);
    assert_eq!(out.len(), 1);
    assert_eq!(out[0].name, "ccbox-market");
    assert_eq!(out[0].source, "https://gitee.com/orczh/ccbox-market.git");

    // preset 拉取失败（离线）→ 回退内置 Gitee 直连源
    let out = with_builtin_market(&[], &[]);
    assert_eq!(out.len(), 1);
    assert_eq!(out[0].name, "ccbox-market");
    assert_eq!(out[0].source, "https://gitee.com/orczh/ccbox-market.git");

    // 已在列表（大小写不同）→ 原样返回，不重复、不改写用户条目
    let custom = vec![CustomMarket {
        name: "CCBox-Market".into(),
        source: "https://example.com/other.git".into(),
    }];
    let out = with_builtin_market(&custom, &presets);
    assert_eq!(out.len(), 1);
    assert_eq!(out[0].name, "CCBox-Market");
    assert_eq!(out[0].source, "https://example.com/other.git");

    // 与其他市场共存：只追加内置条目
    let custom = vec![CustomMarket {
        name: "alpha".into(),
        source: "anthropics/claude-plugins-official".into(),
    }];
    let out = with_builtin_market(&custom, &[]);
    assert_eq!(out.len(), 2);
    assert!(out.iter().any(|m| m.name == "ccbox-market"));
}

// ---------- claude 子进程 spawn 形态（Windows npm shim）----------

/// 无扩展名 npm sh shim（`…\npm\claude`）必须解析到同目录 `.cmd`——
/// CreateProcess 无法执行 sh 文本脚本（用户环境实测：插件市场所有 CLI 调用
/// 报 spawn 失败，而 PTY 经 Git Bash 执行同一文件正常）
#[test]
#[cfg(target_os = "windows")]
fn ClaudeCommand_NpmShim解析到同目录Cmd_1() {
    let dir = tempfile::tempdir().unwrap();
    let shim = dir.path().join("claude");
    std::fs::write(&shim, b"#!/bin/sh").unwrap();
    std::fs::write(dir.path().join("claude.cmd"), b"@echo off").unwrap();

    let cmd = crate::store::spawnable_claude_command(shim.to_str().unwrap());
    assert_eq!(cmd.get_program(), dir.path().join("claude.cmd"));
}

/// 无扩展名且同目录无 `.cmd`：维持原路径（不臆造目标，spawn 失败走错误链上报）
#[test]
#[cfg(target_os = "windows")]
fn ClaudeCommand_无Cmd时保持原路径_2() {
    let dir = tempfile::tempdir().unwrap();
    let shim = dir.path().join("claude");
    std::fs::write(&shim, b"#!/bin/sh").unwrap();

    let cmd = crate::store::spawnable_claude_command(shim.to_str().unwrap());
    assert_eq!(cmd.get_program(), shim);
}

/// 带可执行扩展名（.exe/.cmd）直接使用，不做改写
#[test]
#[cfg(target_os = "windows")]
fn ClaudeCommand_有扩展名直接使用_3() {
    let exe = r"C:\Users\u\.local\bin\claude.exe";
    assert_eq!(
        crate::store::spawnable_claude_command(exe).get_program(),
        std::path::Path::new(exe)
    );
}

// ---------- 市场浏览缓存残留 ----------

/// 上次 clone 半途失败留下无 marketplace.json 的非空残留目录时，重新添加市场
/// 不得被锁死（Mac 实测：报 destination path already exists and is not an
/// empty directory，且每次重试都撞同一目录）。用本地 file:// 仓库复现。
#[test]
fn MarketBrowseCopy_残留目录不锁死重新添加_4() {
    // CONFIG_DIR 是全局 override，TestConfigDirGuard 用户必须串行（见 tests/mod.rs）
    let _lock = crate::tests::CONFIG_DIR_TEST_LOCK.lock();
    let _guard = crate::tests::TestConfigDirGuard::new("market-residue");

    // 源仓库：含 .claude-plugin/marketplace.json 的本地 git repo
    let src = tempfile::tempdir().unwrap();
    let run_git = |args: &[&str]| {
        let mut c = std::process::Command::new("git");
        c.args(["-c", "user.name=t", "-c", "user.email=t@t"]).args(args).current_dir(src.path());
        crate::store::run_command_with_timeout(&mut c, std::time::Duration::from_secs(30)).unwrap();
    };
    std::fs::create_dir_all(src.path().join(".claude-plugin")).unwrap();
    std::fs::write(
        src.path().join(".claude-plugin").join("marketplace.json"),
        r#"{"name":"residue-test"}"#,
    ).unwrap();
    run_git(&["init", "-q"]);
    run_git(&["add", "-A"]);
    run_git(&["commit", "-qm", "init"]);

    let url = format!("file://{}", src.path().to_string_lossy().replace('\\', "/"));

    // 预置残留：缓存目录已存在、非空、但无 marketplace.json（半途失败的形态）
    let cache_dir = crate::store::market_browse_cache_dir().unwrap();
    let residue = cache_dir.join(format!("src-{}", crate::store::short_hash(&url)));
    std::fs::create_dir_all(&residue).unwrap();
    std::fs::write(residue.join("half-written.tmp"), b"garbage").unwrap();

    let crate::store::MarketSource::Dir(p) = crate::store::market_browse_local_copy(&url)
        .expect("残留目录不应锁死添加市场（clone 应清理残留后成功）")
    else {
        panic!("expected Dir copy")
    };
    assert!(p.join(".claude-plugin").join("marketplace.json").is_file());
}

/// git 源浏览优先 raw JSON 直拉：浏览只需清单（KB 级），不再为看列表 clone 整个
/// 仓库（首次打开 30s 的主因）；raw 命中即写 src-<hash>.json 缓存
#[test]
fn MarketBrowseCopy_Git源优先Raw直拉_5() {
    // 同上：TestConfigDirGuard 用户串行
    let _lock = crate::tests::CONFIG_DIR_TEST_LOCK.lock();
    let _guard = crate::tests::TestConfigDirGuard::new("market-raw");
    let url = "https://gitee.com/orczh/ccbox-market.git";
    let manifest = r#"{"name":"ccbox-market","plugins":[]}"#;

    let fake = move |_: &str| -> Option<String> { Some(manifest.to_string()) };
    let crate::store::MarketSource::Dir(p) =
        crate::store::market_browse_local_copy_with(url, &fake)
            .expect("raw 直拉应成功，不应回退 clone")
    else {
        panic!("expected Dir")
    };
    // 缓存为 json 文件形态（非 clone 目录），内容即拉取结果
    assert_eq!(p.extension().and_then(|e| e.to_str()), Some("json"));
    assert_eq!(std::fs::read_to_string(&p).unwrap(), manifest);

    // 第二次调用直接命中缓存（fake 计数为 0 也应成功）
    let never = |_: &str| -> Option<String> { panic!("应命中缓存，不应再拉取") };
    let crate::store::MarketSource::Dir(p2) =
        crate::store::market_browse_local_copy_with(url, &never).unwrap()
    else {
        panic!("expected Dir")
    };
    assert_eq!(p2, p);
}
