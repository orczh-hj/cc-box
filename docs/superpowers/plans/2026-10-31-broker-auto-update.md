# Broker 自动更新实施计划

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Broker 检查到新版本即自动下载；「自动更新」开关开启且零窗口时自动安装重启；托盘菜单直接触发更新/检查；红点缩小贴角；设置页新增开关。

**Architecture:** 全部更新执行收敛到 `src-tauri/src/broker/update.rs` 一条「下载 → 判窗口 → 安装」流水线（tauri updater 插件，Broker 进程内执行）；托盘与周期检查都是这条流水线的触发器；GUI 只新增一个写共享 config 的开关，不参与执行。

**Tech Stack:** Rust (tauri 2, tauri-plugin-updater)、Vue 3 + Pinia、Vitest、cargo test。

**Spec:** `docs/superpowers/specs/2026-10-31-auto-update-design.md`

## Global Constraints

- Rust 结构体返回/序列化字段一律 camelCase（`#[serde(rename = "autoUpdate")]`）。
- 前端 i18n 文案里字面大括号必须转义（`{'{'}`）；新增文案必须过 `tests/i18n` MessageCompile 测试。
- 不新增 Tauri command / 事件 / 协议消息（复用 `update_app_config`、`get_app_config`）。
- `autoUpdate` 缺省（旧 config / 字段缺失）= 开启（true）。
- Linux 无头 Broker 无 Tauri AppHandle，本功能仅 Windows/macOS Broker（现状即如此，不扩 Linux）。
- 测试：`cd src-tauri && cargo test`；前端 `npm test`。

## Review Focus

1. **旧 config 无 `autoUpdate` 字段** → 必须按 true 处理（老用户升级后自动更新默认生效）→ Task 1 测试。
2. **下载成功但开关关闭且有窗口** → 绝不安装，只亮红点 → Task 2 decide 测试。
3. **安装失败** → 必须清 `update_in_progress` 标记并继续运行，否则 Broker 永久抑制 → Task 2 decide/恢复路径测试（标记写在 install 前、失败路径清除在实现内，decide 层断言 `installing` 标志复位）。
4. **版本更新换代**（已下载 vA，下次检查发现 vB）→ 丢弃 vA 重新下载 vB，不得安装旧版本 → Task 2 测试。
5. **手动更新与自动安装并发** → 第二个触发必须被 `installing` 标志挡住，不得双安装 → Task 2 测试。

---

### Task 1: `autoUpdate` 配置字段（Rust + TS 类型 + 默认值纯函数）

**Files:**
- Modify: `src-tauri/src/store.rs:54-84`（AppConfig）
- Modify: `src/types/app.ts:23`（AppConfig TS）
- Create: `src-tauri/src/broker/update.rs` 内新增纯函数（与 Task 2 同文件，本任务先落默认值函数）

**Interfaces:**
- Produces: AppConfig 字段 `pub auto_update: Option<bool>`（serde 名 `autoUpdate`，`#[serde(default)]`）；`update.rs` 纯函数 `pub(crate) fn resolve_auto_update(cfg: Option<bool>) -> bool`（`None → true`）；TS `AppConfig.autoUpdate?: boolean`。

- [ ] **Step 1: 写失败测试（Rust）**

`src-tauri/src/broker/update.rs` 底部加 `#[cfg(test)] mod tests`（tray.rs 同款内联风格）：

```rust
#[test]
fn Update_AutoUpdateDefaultOn_1() {
    assert!(resolve_auto_update(None));      // 旧 config / 字段缺失 = 默认开启
    assert!(resolve_auto_update(Some(true)));
    assert!(!resolve_auto_update(Some(false)));
}
```

- [ ] **Step 2: 运行确认失败**

Run: `cd src-tauri && cargo test Update_AutoUpdateDefaultOn`
Expected: FAIL（`resolve_auto_update` 未定义）

- [ ] **Step 3: 实现**

`update.rs` 加 `pub(crate) fn resolve_auto_update(cfg: Option<bool>) -> bool { cfg.unwrap_or(true) }`；`store.rs` AppConfig 增加字段（放 `custom_markets` 之后，带注释「自动更新：默认 true」）；`src/types/app.ts` AppConfig 加 `autoUpdate?: boolean`。

- [ ] **Step 4: 运行确认通过**

Run: `cd src-tauri && cargo test Update_AutoUpdateDefaultOn`
Expected: PASS

- [ ] **Step 5: Commit** `git add -A && git commit -m "feat(auto-update): autoUpdate 配置字段与默认值"`

---

### Task 2: Broker 更新流水线（下载 → 判窗口 → 安装）

**Files:**
- Modify: `src-tauri/src/broker/update.rs`（主体重写）
- Modify: `src-tauri/src/lib.rs:203-204`（spawn_checker 调用处注释同步）
- Test: 同文件 `#[cfg(test)] mod tests`

**Interfaces:**
- Consumes: `crate::store::get_app_config()`、`crate::gateway::window_roster()`、`crate::broker::lifecycle::{write, LifecycleState}`、`crate::broker::windows::close_all_windows`、`tauri_plugin_updater::UpdaterExt`。
- Produces:
  - `pub(crate) fn available() -> Option<String>`（不变，托盘渲染用）
  - `pub(crate) fn spawn_checker(app: tauri::AppHandle)`（启动**立即**检查，随后每 2h；Linux 侧不调用，维持现状）
  - `pub(crate) async fn check_now(app: tauri::AppHandle)`（托盘「检查更新」触发一次同款周期逻辑）
  - `pub(crate) async fn run_manual_update(app: tauri::AppHandle) -> Result<(), String>`（托盘「更新到 vX」：关窗 → 确保已下载 → 安装重启）
  - 纯决策函数 `fn decide(has_update_ver: Option<&str>, downloaded_ver: Option<&str>, auto_enabled: bool, window_count: usize, installing: bool) -> Decision`，`Decision { None, Download, Install }`

- [ ] **Step 1: 写失败测试（决策纯函数）**

```rust
#[test]
fn Update_DownloadAlways_2() {
    // 有新版本且开关关/有窗口：仍要下载，不安装
    assert_eq!(decide(Some("1.0"), None, false, 3, false), Decision::Download);
    assert_eq!(decide(Some("1.0"), None, true, 3, false), Decision::Download);
}
#[test]
fn Update_InstallOnlyZeroWindowsEnabled_2() {
    assert_eq!(decide(Some("1.0"), Some("1.0"), true, 0, false), Decision::Install);
    assert_eq!(decide(Some("1.0"), Some("1.0"), true, 2, false), Decision::None); // 有窗口等下次
    assert_eq!(decide(Some("1.0"), Some("1.0"), false, 0, false), Decision::None); // 开关关不自动装
    assert_eq!(decide(Some("1.0"), Some("1.0"), true, 0, true), Decision::None);  // 安装中不重入
    assert_eq!(decide(None, None, true, 0, false), Decision::None);
}
#[test]
fn Update_VersionSwap_2() {
    // 已下载 0.9、线上已是 1.0：需重新下载（Decision::Download），不得安装旧版
    assert_eq!(decide(Some("1.0"), Some("0.9"), true, 0, false), Decision::Download);
}
```

- [ ] **Step 2: 运行确认失败**

Run: `cd src-tauri && cargo test Update_`
Expected: 新测试 FAIL（decide 未定义）

- [ ] **Step 3: 实现**

要点（签名与测试未定的部分）：
- 状态：`static AVAILABLE: Mutex<Option<String>>`（含义改为「已检查到」）；新增 `static DOWNLOADED: Mutex<Option<(String, tauri_plugin_updater::Update)>>`（版本 + 已 download 的 Update 对象，跨检查保留）；`static INSTALLING: std::sync::atomic::AtomicBool`。
- `decide`：`has_update_ver != downloaded_ver` 且有更新 → `Download`；相等且 `auto_enabled && window_count == 0 && !installing` → `Install`；否则 `None`。
- 周期体 `async fn run_cycle(app: &AppHandle)`：`updater.check()` → 更新 AVAILABLE → `decide(...)`（`auto_enabled` 取 `resolve_auto_update(crate::store::get_app_config().ok().and_then(|c| c.auto_update))`，窗口数取 `crate::gateway::window_roster().len()`）→ `Download` 分支 `update.download(|_| {}).await` 后存入 DOWNLOADED（失败仅 log，下次重试）；`Install` 分支调 `install_and_restart`。
- `spawn_checker`：去掉 30s 延迟，先 `run_cycle` 再 `loop { sleep(2h); run_cycle }`；`check_now` = spawn 单次 `run_cycle`。
- `async fn install_and_restart(app, update)`：`INSTALLING` 置位 → `lifecycle::write(LifecycleState::UpdateInProgress, Some(env!("APP_VERSION")))` → `update.install().await` → `app.restart()`（不返回）；`install/restart` 前任何 Err：`INSTALLING` 复位 + `lifecycle::clear()` + log。用 guard 结构（Drop 复位）保证失败路径标志清理。
- `run_manual_update`：`INSTALLING` 已置位则返回 Err("update in progress")；`windows::close_all_windows(Duration::from_secs(10)).await` → 若 DOWNLOADED 无当前版本则现场 check+download → `install_and_restart`。安装失败同样经 guard 清理后返回 Err。

- [ ] **Step 4: 运行确认通过**

Run: `cd src-tauri && cargo test`
Expected: 全部 PASS

- [ ] **Step 5: Commit** `git commit -m "feat(auto-update): Broker 下载-判窗口-安装流水线"`

---

### Task 3: 托盘菜单与红点

**Files:**
- Modify: `src-tauri/src/broker/tray.rs`（TrayLabels/build_model/build_menu/handle_menu_event/badge_icon）
- Test: 同文件 tests 模块

**Interfaces:**
- Consumes: Task 2 的 `check_now` / `run_manual_update`。
- Produces: `TrayLabels` 新增 `pub check_update: String`（update 为 None 时显示的「检查更新」文案）。

- [ ] **Step 1: 写失败测试**

在既有 `Tray_UpdateItem_5` 旁新增：

```rust
#[test]
fn Tray_CheckUpdateItem_6() {
    let none = build_model(Some(&status(true, false)), &[], false, None, true);
    assert_eq!(none.check_update, "检查更新");
    let en = build_model(Some(&status(true, false)), &[], false, None, false);
    assert_eq!(en.check_update, "Check for updates");
    let some = build_model(Some(&status(true, false)), &[], false, Some("0.17.0"), true);
    // 有更新项时 check_update 文案仍在结构体里，但不渲染（由 build_menu 决定）
    assert_eq!(some.check_update, "检查更新");
}
```

红点参数：把 `badge_icon` 中半径/圆心计算抽成纯函数 `fn badge_geometry(w: u32, h: u32) -> (i32 /*cx*/, i32 /*cy*/, i32 /*radius*/)`，测试：

```rust
#[test]
fn Tray_BadgeGeometry_7() {
    let (cx, cy, r) = badge_geometry(32, 32);
    assert_eq!(r, 5);            // 32*0.16 四舍五入
    assert_eq!(cx, 32 - 2 - 1);  // 更贴右缘（边缘留 2px）
    assert_eq!(cy, 2);           // 更贴上缘
}
```

（`badge_geometry` 与测试都加 `#[cfg(not(target_os = "linux"))]`。）

- [ ] **Step 2: 运行确认失败**

Run: `cd src-tauri && cargo test Tray_`
Expected: 新测试 FAIL

- [ ] **Step 3: 实现**

- `build_model` 填充 `check_update`（zh「检查更新」/ en「Check for updates」）。
- `build_menu`：现有 `if let Some(label) = &model.update` 块加 `else` 分支，插入 id 为 `"check-update"` 的 MenuItem（enabled，同一分隔符位置）。
- `window_signature` 无需改（check 项出现 ⇔ update 为 None，已由签名覆盖；初始菜单就带该项）。
- `handle_menu_event`：
  - `"update"`：改为 `confirm_destructive`（文案 zh「更新 CC-Box」/「将关闭所有窗口并更新到新版本，确认？」，en 对应；确认按钮「更新」/「Update」）确认后 spawn `crate::broker::update::run_manual_update(app.clone())`，Err 仅 log。
  - `"check-update"`：spawn `crate::broker::update::check_now(app.clone())`。
- `badge_geometry`：`radius = ((min*0.16).round()).max(3)`；`cx = w - 2 - radius`、`cy = 2`（按测试值校准，保持 ≤ w-1）；`badge_icon` 改用它。

- [ ] **Step 4: 运行确认通过**

Run: `cd src-tauri && cargo test`
Expected: PASS（含既有 Tray_ 1-5 不回归）

- [ ] **Step 5: Commit** `git commit -m "feat(auto-update): 托盘检查更新项、直接更新与红点缩小"`

---

### Task 4: 设置页「更多设置」折叠区 + 自动更新开关（前端）

**Files:**
- Modify: `src/components/settings/sections/UpdateSection.vue`
- Modify: `src/i18n/locales/zh.ts` / `en.ts`
- Test: `tests/settings/resolve-auto-update.test.ts`（若目录无先例，放 `tests/utils/` 亦可，与现有 tests/ 结构对齐）

**Interfaces:**
- Consumes: `getAppConfig` / `updateAppConfig`（`src/api/tauri.ts:244-248`）、Task 1 的 `AppConfig.autoUpdate`。
- Produces: 无（叶子 UI）。

- [ ] **Step 1: 写失败测试（默认值解析）**

新建 `src/utils/autoUpdate.ts`：`export function resolveAutoUpdate(v: boolean | undefined): boolean`（undefined → true）。测试断言 `true/false/undefined` 三态（对应 Review Focus #1 的前端侧）。

- [ ] **Step 2: 运行确认失败**

Run: `npm test -- tests/utils`（按实际路径）
Expected: FAIL（模块不存在）

- [ ] **Step 3: 实现**

- `UpdateSection.vue`：在 CC-Box 更新卡片之后、Claude CLI 卡片之前插入「更多设置」折叠块——header 行（标题 + chevron 旋转）点击切换 `showAdvanced`（默认收起）；展开区内一行开关（复用设置页现有 toggle 样式，参考 `SettingsModal.vue`/其他 section 的开关写法），标签「自动更新」+ 说明文案「有新版本时自动下载，并在无窗口运行时自动安装重启」。
- `onMounted` 里 `getAppConfig()` 读 `autoUpdate`（经 `resolveAutoUpdate`），切换时 `updateAppConfig({ autoUpdate: value })`。
- i18n：zh/en 各加 `updateMoreSettings`、`autoUpdateLabel`、`autoUpdateDesc`（不含裸大括号；跑 MessageCompile 守卫）。

- [ ] **Step 4: 运行确认通过**

Run: `npm test`
Expected: PASS（含 tests/i18n MessageCompile）

- [ ] **Step 5: Commit** `git commit -m "feat(auto-update): 设置页自动更新开关（更多设置折叠区）"`

---

### Task 5: 收尾核对与文档

- [ ] **Step 1:** `cd src-tauri && cargo test && cargo clippy`（零新告警）；`npm test`；`npm run build`（生产构建安全规则，verify-dist-safety 自动跑）。
- [ ] **Step 2:** 手动测试条目登记：`docs/manual-test-cases.md` 增补（托盘检查更新/直接更新、零窗口自动安装、开关关闭只下载、更新失败恢复）。
- [ ] **Step 3:** 速查更新：CLAUDE.md「多实例设备级 Broker」段补一行自动更新规则；定向细节进 `remote/docs/multi-instance-gateway.md` 或 `docs/release-process.md` 对应小节。
- [ ] **Step 4: Commit** `git commit -m "docs(auto-update): 手动测试条目与速查更新"`
