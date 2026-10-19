# CC-Box

针对 Claude Code 开发的多终端管理器，用 Tauri 2 + Vue 3 + xterm.js + portable-pty 直连 Claude CLI，在原生终端体验基础上增加外围功能和信息呈现。

## 核心思想

**面向 Claude Code 重度用户的多会话管理器。GUI 做增强不做替代，让 CLI 做不好的事变得容易。**

### 产品定位

- **面向谁**：已熟练使用 Claude Code CLI 的开发者，尤其是需要同时管理多个会话、多个项目的重度用户
- **解决什么问题**：CLI 在单会话交互上已经足够好，但在多会话并行、信息总览、跨会话状态追踪上力不从心
- **核心价值**：
  1. **多会话并行管理** — 一个窗口内同时运行多个 Claude 会话，快速切换、互不干扰
  2. **信息可视化增强** — MCP 工具详情等 CLI 不方便展示的信息，通过侧边栏面板呈现
  3. **工作流加速** — 快捷命令、prompt 片段、项目预设等 CLI 之外的外围辅助

### 设计原则

- **CLI 优先，GUI 增强** — 直接运行 Claude CLI 二进制文件，输入行为与原生终端完全一致；GUI 只做 CLI 做不好或做起来不方便的事
- **轻量透明** — JSON 文件存储，不引入数据库/路由/状态机；原生数据只读，GUI 配置独立保存在 `~/.cc-box/config.json`
- **功能边界** — CLI 里已经很好用的功能（对话交互、slash 命令、快捷键、模型切换），不在 GUI 里重复实现；GUI 专注于管理、可视化、辅助三类增强
- **可逆性** — 用户可随时回到纯 CLI，GUI 不修改任何 Claude Code 原生配置文件，不留副作用
- **最小依赖** — 完全兼容 Claude Code 任何更新，无需适配 SDK API；新功能通过读取原生配置文件和 CLI 命令获取

### 不做什么

- 不做 AI 补全/输入建议（CLI 已有）
- 不做 slash 命令的 GUI 封装（CLI 已有）
- 不做对话消息的结构化展示（终端原生渲染足够好）
- 不做 Claude Code 配置的编辑器（只做只读展示，编辑由 CLI/settings.json 完成）
- 不做独立的 prompt 管理系统（CLI 的 /memory 和 CLAUDE.md 已覆盖）

## 技术栈

Tauri 2.x (Rust) + Vue 3 + TypeScript + Vite + xterm.js + portable-pty + Pinia + 自定义 CSS

## 项目架构

```
cc-box/
├── src-tauri/                  # Rust 后端
│   ├── src/
│   │   ├── main.rs             # 入口
│   │   ├── lib.rs              # 初始化、插件注册、Command 注册
│   │   ├── pty.rs              # PTY 管理（portable-pty 封装）
│   │   ├── commands.rs         # Tauri IPC 命令
│   │   ├── store.rs            # Claude Code 原生数据读取
│   │   ├── providers.rs        # **Provider 管理**（存储、合并、激活、cc-switch 导入）。激活走 deep merge，剥离 `enabledPlugins`/`disabledPlugins`/`extraKnownMarketplaces`/`hooks` 状态字段，不整文件覆盖 settings.json
│   │   ├── mcp.rs              # MCP 协议客户端（HTTP/SSE + stdio）
│   │   ├── hook_events.rs      # Hook 事件数据结构与提取
│   │   ├── hook_server.rs      # Hook HTTP 服务器（接收 Claude 运行时事件）
│   │   ├── hook_config.rs      # Hook Plugin 文件管理
│   │   ├── remote_server.rs    # **远程访问服务器**（局域网直连：axum WS + Web UI 嵌入 + token 认证）
│   │   ├── remote_state.rs     # **远程状态与消息处理**（会话聚合、输出 tee、working/pending 状态机、attach 独占、新建/关闭；局域网/中继共用唯一 handler）
│   │   ├── remote_client.rs    # **中继出站客户端**（连接 cc-box-relay、手机连接桥接、断线重连）
│   │   ├── rpc_dispatch.rs     # **RPC 命令分发器**（rpc_table! 宏表：一行一命令；浏览器经 WS 调用 Tauri command 的唯一入口；差集守卫测试）
│   │   ├── events.rs           # **事件单一出口**（事件名常量 + emit_all：Tauri emit 与远程广播二合一）
│   │   ├── checks.rs           # 环境检查
│   │   ├── logger.rs           # 日志系统（FileLogger、文件轮转）
│   │   └── updater.rs          # 自动更新（GitHub Releases）
│   ├── plugin/                 # Claude Code Plugin 文件（编译时嵌入）
│   ├── capabilities/           # Tauri 权限配置
│   ├── Cargo.toml
│   └── tauri.conf.json
│
├── src/                        # Vue 3 前端
│   ├── api/
│   │   ├── transport.ts        # **传输层接口**（invoke/listen/会话 API；按运行环境选实现，组件零感知桌面/浏览器）
│   │   ├── env.ts              # 环境检测（isTauri = '__TAURI_INTERNALS__' in window）
│   │   ├── tauri-transport.ts  # 桌面实现（转发 Tauri invoke/listen）
│   │   ├── remote-transport.ts # 浏览器实现（WS + rpc.call 桥 + 事件映射 pty.output→pty-output 等）
│   │   ├── tauri.ts            # invoke/listen 封装（经 transport，签名与 @tauri-apps/api 一致）
│   │   ├── provider.ts         # **Provider API**（经 transport）
│   │   └── platform.ts         # 平台能力降级（剪贴板 execCommand 回退、openExternal）
│   ├── components/             # UI 组件
│   │   ├── App.vue             # 视图切换 + 环境检查 + TokenGate 认证门
│   │   ├── TokenGate.vue       # 浏览器远程模式 token 认证（#token=/#pair= 解析）
│   │   ├── TitleBar.vue        # 自定义标题栏（Windows；浏览器模式隐藏）
│   │   ├── WelcomeView.vue     # 欢迎页
│   │   ├── ProjectSelectView.vue # 项目选择页
│   │   ├── TerminalView.vue    # 终端主视图容器
│   │   ├── XTermTerminal.vue   # xterm.js 终端核心
│   │   ├── TerminalHeader.vue  # 终端标题栏
│   │   ├── IconBar.vue         # 左侧图标栏
│   │   ├── ShortcutsModal.vue  # 快捷键弹窗
│   │   ├── sessions/           # 会话面板（SessionList > SessionItem > SessionStatus）
│   │   ├── skills/             # Skills 面板（SkillsPanel > SkillGroup > SkillItem）
│   │   ├── agents/             # Agents 面板（AgentsPanel > AgentGroup > AgentItem）
│   │   ├── mcp/                # MCP 面板（McpPanel > McpGroup > McpItem > McpSubItem）
│   │   ├── plugins/            # Plugins 面板（PluginsPanel > PluginGroup > PluginItem）
│   │   ├── sidebar/            # 侧边栏容器（SidebarPanel > PanelHeader）
│   │   └── settings/           # 设置（SettingsOverlay > SettingsView + sections/）
│   │       └── providers/      # **Provider 组件**（ProviderList > ProviderCard、EditPanel、PresetPanel、CommonConfigPanel）
│   ├── config/
│   │   └── providerPresets.ts  # **Provider 预设模板**（50+ 厂商）
│   ├── stores/                 # Pinia：app、session、sidebar、config、hook、providers
│   ├── types/                  # TypeScript 类型定义（pty、session、project、config、app、hook、provider）
│   ├── composables/            # useAppShortcuts、useTerminalCommand、useStatusMonitor、useWindowAttention、useMediaQuery
│   ├── utils/                  # platform 工具
│   └── styles/global.css       # CSS 变量与全局样式
│
├── remote/                     # **远程连接**（协议正本 protocol/、中继服务器 relay/、专属文档 docs/；Web UI = 主应用 dist，无独立子应用）
│
├── docs/                       # 详细文档
├── package.json
└── vite.config.ts
```

## 核心数据流

```
桌面：  xterm.js ←→ transport(tauri) ←→ Tauri invoke/listen ←→ pty.rs (Rust) ←→ portable-pty ←→ Claude CLI
浏览器：xterm.js ←→ transport(remote) ←→ WS(rpc.call/session.*) ←→ remote_state/rpc_dispatch ←→ pty.rs
```

- 用户输入 → `onData` → `transport.sessionInput` →（桌面 pty_input / 浏览器 pty.input 消息）→ PTY writer → Claude CLI
- CLI 输出 → PTY reader → `decode_output()`（UTF-8 优先，失败回退 GBK，兼容 Windows 中文子进程）→（桌面 `emit('pty-output')` / 远程 push_output 广播 PtyOutput，remote-transport 映射回同名事件）→ `term.write()` → xterm.js
- 浏览器远程模式：TokenGate 认证（#token=/#pair=）→ WS → rpc.call 调 Tauri command（rpc_dispatch 白名单）；PTY 数据流不走 rpc，走 session.create/attach/pty.input 专用消息（attach 即订阅、独占语义）

### Hook 监控数据流

```
Claude CLI hook 触发 → report-hook.sh → curl POST → hook_server.rs (axum) → emit('hook-event') → stores/hook.ts (事件总线) → useStatusMonitor (状态监控)
```

- Plugin 通过 `--plugin-dir` 按 session 加载，注入 11 个 hook 事件
- 每个 PTY 注入 `CC_BOX_HOOK_PORT`（服务器端口）和 `CC_BOX_SESSION_ID`（终端标识）
- `stores/hook.ts`：纯事件总线，模块通过 `subscribe(eventTypes[], handler)` 注册消费
- `useStatusMonitor`：hook 事件 → Tab 的 `working`/`pending` 状态 + 任务栏跳动
- 详细架构 → [docs/hook-monitor.md](docs/hook-monitor.md)

详细架构 → [docs/terminal-integration.md](docs/terminal-integration.md)

## 开发命令

```bash
npm install                # 安装依赖
npm run tauri:dev          # 开发模式（前端 :1420 + Rust 热重载）
npm run tauri:build        # 生产构建
```

### Windows 环境配置
- **Rust 工具链**：使用 MSVC 工具链（`rustup default stable-x86_64-pc-windows-msvc`）
- **前置依赖**：需安装 [Microsoft C++ Build Tools](https://visualstudio.microsoft.com/visual-cpp-build-tools/)，勾选「C++ build tools」和「Windows 11 SDK」
  - 安装后确保 `link.exe` 为 MSVC 版本而非 Git coreutils 的 `link`
  - 若 Git bash 中 `cargo build/test` 提示 `link: extra operand`，在 `.cargo/config.toml` 中显式指定 linker 路径
- **代理设置**：
  - 推送到 GitHub 需要代理：
    ```bash
    set HTTP_PROXY=http://127.0.0.1:33210
    set HTTPS_PROXY=http://127.0.0.1:33210
    ```
  - 推送到 Gitee 不需要代理
- **打包代理设置**：首次打包下载 NSIS 组件时需要代理，设置环境变量：
  ```bash
  set HTTP_PROXY=http://127.0.0.1:33210
  set HTTPS_PROXY=http://127.0.0.1:33210
  npm run build:win
  ```

### 双仓库同步

项目同步推送到 GitHub 和 Gitee 两个仓库，方便国内访问。

- **GitHub**：`https://github.com/orczh-hj/cc-box`（需要代理）
- **Gitee**：`https://gitee.com/orczh/cc-box`（国内直连）

**同步机制**：
1. Git 多 URL 远程：`git push origin` 自动推送到两个仓库
2. GitHub Actions：push 到 main 或 release 发布时自动同步到 Gitee

**推送时注意**：推送到 GitHub 需先设置代理，推送到 Gitee 无需代理。

**注意**：2026-08 起发布流程不再创建 Gitee Release（代码仓库同步保留）；如需恢复，token 仍在 `git config --local gitee.token`。

### 版本发布（全自动）

**一句话发布**：
```bash
npm run release -- --bump patch --notes "### Features\n- Add feature"
```

脚本自动完成：更新版本号 → 更新 CHANGELOG → 提交推送 → 创建标签 → 监控 CI → 发布 Release → 上传 OSS。

**参数说明**：
| 参数 | 说明 |
|------|------|
| `--bump <type>` | 版本类型：`major` / `minor` / `patch`（与 `--exact` 二选一） |
| `--exact` | 使用当前版本发布，不 bump 版本号（适用于重新发布） |
| `--notes "<text>"` | Release notes，`\n` 表示换行（必填），必须用英文写 |
| `--skip-ci` | 跳过 CI 监控（标签已构建时用） |
| `--oss-only <ver>` | 仅上传 OSS（如 `--oss-only v0.5.1`） |

**常用示例**：
```bash
# 新版本发布
npm run release -- --bump patch --notes "### Fixed\n- Fix copy issue"
npm run release -- --bump minor --notes "### Features\n- Add feature"

# 重新发布当前版本（CI 已构建）
npm run release -- --exact --notes "### Fixed\n- Fix issue" --skip-ci

# 仅上传 OSS（补传某个版本）
npm run release -- --oss-only v0.5.1
```

详细流程 → [docs/release-process.md](docs/release-process.md)

**远程改动发布提醒**：前端（src/）、relay（remote/relay/）、协议（remote/protocol/）任一改动，除桌面版本发布外**必须同步重编部署 relay**（手机 UI 是 relay 编译时嵌入的 dist），顺序先桌面后 relay——每次版本发布的完整清单见 [docs/release-process.md](docs/release-process.md)「远程组件发布」一节。

## 详细文档

| 文档                                                           | 内容                                                    |
|--------------------------------------------------------------|-------------------------------------------------------|
| [docs/测试编写原则.md](docs/测试编写原则.md)   | 项目如何编写测试                                              |
| [docs/manual-test-cases.md](docs/manual-test-cases.md)   | **手动测试条目**：自动化无法覆盖的 UI 交互与端到端测试                  |
| [docs/terminal-integration.md](docs/terminal-integration.md) | 终端集成架构、PTY 生命周期、IPC 命令与事件对照                           |
| [docs/webgl-corruption-fix.md](docs/webgl-corruption-fix.md) | **WebGL Renderer 乱码修复方案**：atlas corruption 根因、方案对比、切 Tab reload 实施 |
| [docs/hook-monitor.md](docs/hook-monitor.md)                 | **Hook 监控系统**：Plugin 注入、事件采集、状态机、多终端区分                |
| [docs/provider-management.md](docs/provider-management.md)   | **Provider 管理**：数据结构、激活流程、通用配置合并、CRUD、cc-switch 导入    |
| [docs/provider-test-cases.md](docs/provider-test-cases.md)   | **Provider 测试条目**：9 大类 80+ 测试用例，覆盖 CRUD、激活合并、导入、UI 交互 |
| [docs/layout-design.md](docs/layout-design.md)               | 布局设计、窗口结构、色彩系统、排版规范                                   |
| [docs/components.md](docs/components.md)                     | 组件树、各组件职责与 props/events、Store 结构                      |
| [docs/interaction.md](docs/interaction.md)                   | **快捷键处理架构**、三场景输入处理、DOM 捕获期监听                         |
| [docs/capabilities.md](docs/capabilities.md)                 | **Tauri 权限管理**、查询/确认/添加 capabilities 权限的方法            |
| [docs/data-persistence.md](docs/data-persistence.md)         | 数据存储架构、文件路径、JSON 结构                                   |
| [docs/env-injection.md](docs/env-injection.md)               | **环境变量注入**：PTY 启动时注入环境变量、注入顺序、扩展方式       |
| [docs/startup-checks.md](docs/startup-checks.md)             | 启动先决条件检查、路径检测与自动保存                                    |
| [docs/roadmap.md](docs/roadmap.md)                           | 开发路线图、进度跟踪、待办事项                                       |
| [docs/logging.md](docs/logging.md)                           | 日志文件路径、级别策略、轮转与清理机制                                   |
| [docs/release-process.md](docs/release-process.md)           | 版本号管理、本地打包、CI/CD 发布、签名与分发                             |
| [remote/README.md](remote/README.md)                         | **远程连接设计文档**：双模式架构、统一管理、消息协议、独占接管、中继部署、实施阶段 |
| [remote/docs/deploy.md](remote/docs/deploy.md)               | **中继部署指南**：cc-box-relay 构建、systemd、nginx 反代、桌面接入（环境变量）、**本地测试环境变量**（CC_BOX_RELAY_URL/BIND）与多实例宿主制 |
| [remote/docs/manual-test-cases.md](remote/docs/manual-test-cases.md) | **远程手动测试条目**：真机访问、状态圆点、断线重连、token 安全、快捷键工具条、多实例宿主制 |

外部参考：[Claude Code 线上文档](https://code.claude.com/docs/llms.txt)

外部参考：[Tauri 2.x JS API线上文档](https://v2.tauri.org.cn/reference/javascript/api/)

## 约定

- 每次修改后，核心更新同步到 CLAUDE.md，细节更新同步到 docs/*.md
- Rust 结构体返回前端时统一使用 `#[serde(rename_all = "camelCase")]`
- 添加新 Tauri Command：commands.rs 定义 → lib.rs 注册 → api/tauri.ts 封装
- 添加新 Tauri JS API 调用时，必须确认 `capabilities/default.json` 中有对应权限（`<plugin>:default` 不包含大部分写操作，需显式添加）→ 详见 [docs/capabilities.md](docs/capabilities.md)

### 新增功能检查清单（统一管理）

UI 已浏览器化（一套 UI 走桌面 Tauri / 宽屏浏览器 / 手机浏览器），新功能必须登记到对应单一注册链，防两端漂移：

- **新 Tauri command** → `rpc_dispatch.rs` 的 `rpc_table!` 宏表登记（一行一命令）或 `EXCLUDED` 显式排除（系统级）。差集守卫测试断言 `lib.rs invoke_handler 注册表 == RPC_WHITELIST + EXCLUDED`，漏登记时测试失败强制评估
- **新事件** → `events.rs` 常量 + `emit_all`；远程会话类事件用 `remote_state::emit_event`（桌面+广播）/`emit_event_ex`（排除接管者）/`emit_desktop_only`（仅桌面）
- **新协议消息** → `remote/protocol/` 四处同步：crate 定义 → protocol.ts → samples/ JSON 样例 → remote/docs/protocol.md（Rust 与 Vitest 契约测试共跑样例）
- **CLI 启动参数拼接** → 只改 `pty::build_claude_args`（Rust 单一核心，session.create 与桌面 spawn 共用），勿在前端另写一份
- **前端 invoke/listen** → 一律经 `src/api/transport.ts`（组件禁止直接 import @tauri-apps/api；桌面专属 API 用 `transport.env.isTauri` 守卫）

### 响应式适配约定（手机断点 ≤768px）

一套 UI 走桌面 Tauri / 宽屏浏览器 / 手机浏览器，断点统一 **768px**：

- **纯样式分支**一律写 `@media (max-width: 768px)`（IconBar 底部横栏、SidebarPanel 抽屉、ProjectSelectView 单栏折叠、ProviderCard 按钮换行等）
- **结构分支**（两套 DOM，如 SettingsView 竖导航/顶部 tab）用 `useIsMobile()`（`src/composables/useMediaQuery.ts`，与 CSS 断点一致）
- **终端字号**：手机 = 桌面设置 × 0.75（`XTermTerminal` 的 `effectiveFontSize`），断点跨越自动重算
- **软键盘**：App.vue 同步 `visualViewport.height` 到 app-root 高度，驱动终端 fit
- **safe-area**：底部栏用 `--safe-area-bottom`（env(safe-area-inset-bottom)），视口 meta 带 viewport-fit=cover
- **触屏适配**：所有"hover 才显示"的操作按钮必须包 `@media (hover: hover)`（触屏设备 hover:none 无悬停，按钮常显——SessionItem/ProviderCard 已改）；快捷键工具条（QuickKeyBar）补软键盘按不出的控制键
- **触屏已知限制**：Provider 列表拖拽排序（HTML5 drag）移动端不可用，待需要时补上/下移按钮
- 构建产物由 remote_server/relay 的 rust-embed 嵌入：改前端后 `npm run build` 在 8747 即时生效（debug 实时读 dist）；**中继端（orczh.top:8800）是 release 编译时嵌入，必须重编 relay 并重部署**——完整流程见 remote/docs/deploy.md「日常更新部署（复用手册）」

### 测试要求

- **开发必须搭配测试**：新增功能、修改逻辑、修复 bug 时，同步编写或更新对应测试。遵循 [测试编写原则](docs/测试编写原则.md)
- **Bug 修复必须先写测试**：修复 bug 时，先编写测试复现问题，确认测试失败，然后修复代码直至测试通过
- **自动测试优先**：能用自动测试覆盖的场景，必须写成自动测试，不要写入手动测试文档
- **手动测试条目**：仅记录自动化测试无法覆盖的场景（如真实 PTY 进程环境、跨组件端到端交互、视觉表现），记录到 `docs/manual-test-cases.md`，每个条目包含：测试目标、前置条件、操作步骤、预期结果
- **测试文件独立存放**：
  - 前端：项目根目录 `tests/` 文件夹，运行 `npm test`
  - 后端：`src-tauri/src/tests/` 文件夹，运行 `cd src-tauri && cargo test`
- **测试基础设施**：
  - 前端：Vitest + jsdom + `@tauri-apps/api/mocks`（mockIPC）
  - 后端：Rust `#[cfg(test)]` 模块，被测函数 `pub(crate)` 可见性
  - Store 测试：`setActivePinia(createPinia())` + `mockIPC`
- **命名规范**：英文函数名 `Feature_SubFeature_SeqNum` 格式，中文注释描述目标
- **什么要测**：纯函数、数据转换、解析逻辑、状态管理、边界条件和错误路径
- **什么不测**：getter/setter、类型定义、简单 props 传递、第三方库能力
