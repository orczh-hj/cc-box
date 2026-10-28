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
│   │   ├── lib.rs              # 初始化、插件注册、Command 注册
│   │   ├── pty.rs              # PTY 管理（portable-pty 封装）
│   │   ├── commands.rs         # Tauri IPC 命令
│   │   ├── store.rs            # Claude Code 原生数据读取
│   │   ├── providers.rs        # Provider 管理（存储、合并、激活、cc-switch 导入）
│   │   ├── mcp.rs              # MCP 协议客户端（HTTP/SSE + stdio）
│   │   ├── hook_events.rs      # Hook 事件数据结构与提取
│   │   ├── hook_server.rs      # Hook HTTP 服务器（接收 Claude 运行时事件）
│   │   ├── hook_config.rs      # Hook Plugin 文件管理
│   │   ├── remote_server.rs    # 远程直连服务器（axum WS + Web UI + token 认证）
│   │   ├── remote_state.rs     # 远程状态与消息处理（会话聚合、per-session 控制权、revision、广播）
│   │   ├── remote_client.rs    # 中继出站客户端（连接 relay、手机桥接、断线重连）
│   │   ├── file_transfer.rs    # 远程文件传输（分块上传/下载、票据直链、双限额、三层清理）
│   │   ├── rpc_dispatch.rs     # RPC 命令单一注册表（registry! 宏表）
│   │   ├── events.rs           # 事件单一出口（事件名常量 + emit_all）
│   │   ├── checks.rs           # 环境检查
│   │   ├── logger.rs           # 日志系统（FileLogger、文件轮转）
│   │   └── updater.rs          # 自动更新（GitHub Releases）
│   ├── plugin/                 # Claude Code Plugin 文件（编译时嵌入）
│   ├── macros/                 # cc-box-macros（#[cc_command] 写命令出口广播）
│   ├── capabilities/           # Tauri 权限配置
│   ├── Cargo.toml
│   └── tauri.conf.json
│
├── src/                        # Vue 3 前端
│   ├── api/                    # 传输层（transport 按环境选桌面/浏览器实现）+ 各 API 封装
│   ├── components/             # UI 组件（App、终端、sessions/skills/agents/mcp/plugins/files/settings 面板）
│   ├── config/                 # providerPresets.ts（生成物勿手改）+ providerCategories.ts
│   ├── stores/                 # Pinia：app、session、sidebar、config、providers
│   ├── state/sections.ts       # 状态同步中心（state-changed → loader 注册表）
│   ├── types/                  # TypeScript 类型定义
│   ├── composables/            # useAppShortcuts、useTerminalCommand、useStatusMonitor 等
│   ├── utils/                  # 平台与终端工具
│   └── styles/global.css       # CSS 变量与全局样式
│
├── remote/                     # 远程连接（协议正本 protocol/、中继 relay/、web-assets/、docs/）
├── docs/                       # 详细文档
├── package.json
└── vite.config.ts
```

> Provider 预设生成：`npm run sync:presets -- --zip <zip路径>`，策略见 docs/provider-management.md。

## 核心数据流

```
桌面：  xterm.js ←→ transport(tauri) ←→ Tauri invoke/listen ←→ pty.rs (Rust) ←→ portable-pty ←→ Claude CLI
浏览器：xterm.js ←→ transport(remote) ←→ WS(rpc.call/session.*) ←→ remote_state/rpc_dispatch ←→ pty.rs
```

- 用户输入 → `onData` → `transport.sessionInput` → PTY writer → Claude CLI
- CLI 输出 → PTY reader → `decode_output()`（UTF-8 优先，失败回退 GBK，兼容 Windows 中文子进程）→ `pty-output` 事件 → `term.write()`
- 浏览器远程模式：TokenGate 认证（#token=/#pair=）→ WS → rpc.call 调 Tauri command（rpc_dispatch 白名单）；PTY 数据流不走 rpc，走 session.create/attach/pty.input 专用消息（attach 即订阅、独占语义）
- Hook 监控：`hook_server.rs` 接收 CLI hook 上报 → Rust `apply_hook_event` 单点状态机 → `hook-status` 事件 → `useStatusMonitor`（前端仅应用结果）
- Claude CLI 启动：`platform::cli_target()` 是检查与 spawn 的唯一解析入口——配置路径可运行则用完整路径，否则 Unix 先经 `find_executable` 解析出可运行路径再 exec、Windows 用裸名交给 Git Bash；自动检测不写配置（只由用户设置/安装器写）；失败经 `pty-error` → `appStore.cliHealth` 统一展示与重试

详细架构 → [docs/terminal-integration.md](docs/terminal-integration.md)、[docs/hook-monitor.md](docs/hook-monitor.md)

## 开发命令

```bash
npm install                # 安装依赖
npm run tauri:dev          # 开发模式（前端 :1420 + Rust 热重载）
npm run tauri:build        # 生产构建
```

改 `remote/`（协议/relay）或远程相关前端/Rust 后，**先在本地跑通中继全链路再部署**。命令与注意事项 → [docs/development.md](docs/development.md)「远程功能本地全链路测试」、[remote/docs/deploy.md](remote/docs/deploy.md)「本地测试（开发环境）」。

**本地中继测试标准启动：一条命令 `npm run dev:relay`（脚本自动探测局域网 IP、拉起 relay + tauri dev；手机访问 `http://<本机IP>:8800`，任一进程退出全部退出）。** 多设备联调必须共用同一个 relay（设备凭证按页面 origin 隔离）：第二台机器用 `CC_BOX_RELAY_URL=<第一台IP>:8800 npm run dev:relay`（不启动本地 relay），手机只打开第一台的 relay 页面配对两台电脑。

## 环境与仓库（速查）

- **Windows**：MSVC 工具链 + C++ Build Tools（link.exe 须为 MSVC 版本）；配置细节 → [docs/development.md](docs/development.md)「Windows 环境配置」
- **双仓库**：GitHub（需代理）/ Gitee（直连）；**始终分 URL 推送，勿用多 URL origin** → [docs/development.md](docs/development.md)「双仓库同步」
- **打包**：首次下载 NSIS 组件需代理 → [docs/development.md](docs/development.md)「Windows 环境配置」

## 版本发布（速查）

```bash
npm run release -- --bump patch --notes "### Features\n- Add feature"
```

- 脚本自动完成：版本号 → CHANGELOG → 提交推送 → 标签 → CI → Release → OSS；失败后**重跑同一完整命令自动续跑**（不会二次 bump）
- **`--bump major/minor` 只在用户明确指定时使用**；`--notes` 必填且必须用中文
- **relay 二进制不内嵌 Web UI**（`web-assets` 以 `--no-default-features` 构建，UI 与 relay 部署完全独立）：生产 relay 必须配置 `CC_BOX_RELAY_WEB_DIR=/opt/cc-box/web`，前端（src/）改动只需 `npm run deploy:web`（构建 → staging → md5 校验 → 原子切换 → 线上校验，完整镜像替换、自动清理旧 bundle；勿直接用 `scp -r dist/*` 覆盖，否则历史 `index-*.js` 会永久堆叠），免重编、免重启；只有 relay 源码/服务逻辑（remote/relay/、web-assets 代码）改动才跑 `npm run deploy:relay`（不再构建前端；先桌面后 relay）。桌面安装包保留内嵌 UI（默认 feature）
- 详细参数、断点续跑与远程组件发布清单 → [docs/release-process.md](docs/release-process.md)

## 详细文档

| 文档                                                           | 内容                                                    |
|--------------------------------------------------------------|-------------------------------------------------------|
| [docs/development.md](docs/development.md)                   | **定向细节手册**：环境/代理/双仓库、开发检查清单完整版、控制权机制、会话管理、移动端适配 |
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
| [remote/README.md](remote/README.md)                         | **远程连接设计文档**：双模式架构、统一管理、消息协议、会话控制权、中继部署、实施阶段 |
| [remote/docs/deploy.md](remote/docs/deploy.md)               | **中继部署指南**：构建、systemd、nginx 反代、桌面接入、本地测试、日常更新部署 |
| [remote/docs/protocol.md](remote/docs/protocol.md)           | **协议与行为规则正本**：消息定义、会话控制权（per-session）、文件传输                 |
| [remote/docs/multi-instance-gateway.md](remote/docs/multi-instance-gateway.md) | **多实例远程实施与操作说明**：设备级 Broker 架构、窗口管理与生命周期、路由与控制权、桌面/手机用户操作逻辑、兼容矩阵、已知限制 |
| [remote/docs/manual-test-cases.md](remote/docs/manual-test-cases.md) | **远程手动测试条目**：真机访问、状态圆点、断线重连、token 安全、快捷键工具条、窗口管理/托盘 |

外部参考：[Claude Code 线上文档](https://code.claude.com/docs/llms.txt)、[Tauri 2.x JS API 线上文档](https://v2.tauri.org.cn/reference/javascript/api/)

## 开发约定

- **远程操作一致性**：远程（手机）操作与桌面直接操作效果一致——桌面端原生实现单点执行，远程只是触发器 + 事件同步的镜像，不另写一份执行逻辑
- **远程文件下载走票据直链（所有浏览器统一）**：`file_download_ticket` 签发一次性票据 → 直连 `GET /dl/<ticket>` 流式返回、中继 `/?dl=<device>.<ticket>` 票据页（受限 WS 只允许 `file_download_ticket_chunk`/`ping`）。票据 **不绑连接**（系统浏览器/下载器接管后 WS 必断，不能被 `cleanup_conn` 回收）、128bit、TTL 10 分钟、256 次；旧桌面/旧 relay 经 `command not allowed` 与 HEAD 探测自动降级 Blob。relay 零改动（只按 token 前缀路由），详见 [remote/docs/protocol.md](remote/docs/protocol.md)「一次性下载票据」
- **避免重复实现**：同一逻辑只留一份（如 `spawn_pty_core`、`apply_hook_event`）；两端各自需要的状态靠事件同步（`session-renamed`、`hook-status`），不靠复制代码
- **macOS 原生编辑快捷键（焦点驱动 Edit 菜单）**：WKWebView 粘贴无 keydown 默认行为，输入框 Cmd+V 依赖原生 Edit 菜单（`lib.rs` macos cfg 构建，paste:/copy: selector）；但菜单 key equivalent 会在按键到达 WebView 前被系统拦截，故 `useAppShortcuts` focusin 联动——xterm-helper-textarea 持焦时 `set_native_edit_menu(false)` 切空菜单，其余焦点切回 Edit 菜单；**终端按键处理永不经菜单**；判定逻辑见 `src/utils/mac-edit-keys.ts`，禁止在前端 JS 模拟编辑键（execCommand 仿真）
- **离散操作必广播**：用户离散操作（建/关/重命名会话）经 Rust 广播事件让对端走同一个原生函数；远程安全守卫（白名单/并发上限/独占）作为前置检查包住共用核心，不因统一而放宽
- **用户级资源开关（skills/agents/mcp/plugins）**：后端单点 `store.rs` 的 `set_*_enabled`（桌面/远程同一函数）。skill/agent 禁用=移入 `~/.cc-box/disabled/<category>`，必须 **symlink 感知**（相对链接按目标位置重算、存在性用 `symlink_metadata`、disabled 扫描列出断链以便恢复）；前端 `state-changed` 刷新必须**静默**（`loadXxx(cwd, { silent: true })`，不切 loading 分支，否则整列表卸载重建）；**本端写回环必须抑制**（`markSelfWrite` 登记 → `applyStateSection` 消费并跳过该 section，条目原地响应；plugin 仍级联子项），后端列表按名称**稳定排序**（顺序与 enabled 解耦）；开关按 `(sourceType=user, name)` 定位，禁止裸 name。providers 系写命令同样登记抑制（activate/delete/create/update/reorder/updateCommon），激活返回更新后的 `appliedSettings` 供发起端精确同步、免全量刷新
- 每次修改后：通用规则速查更新到 CLAUDE.md，定向细节更新到 docs/development.md 或对应正本文档
- Rust 结构体返回前端时统一使用 `#[serde(rename_all = "camelCase")]`
- **生产构建安全**：`npm run build` 末尾自动跑 `scripts/verify-dist-safety.ts` 扫描 dist 坏产物；`scripts/lib/patch-xterm-enum.ts` 修补 xterm 死枚举 IIFE 的 esbuild 压缩缺陷（终端写队列永久卡死的根因）。**该类问题 dev 不复现**，升级 xterm/esbuild/Vite 后必须跑生产构建；整个构建零命中告警时复核补丁 → [docs/development.md](docs/development.md)「生产构建安全」
- 添加新 Tauri Command：commands.rs 定义 → lib.rs 注册 → api/tauri.ts 封装（并完成下方检查清单登记）
- 添加新 Tauri JS API 调用时，必须确认 `capabilities/default.json` 中有对应权限（`<plugin>:default` 不包含大部分写操作，需显式添加）→ 详见 [docs/capabilities.md](docs/capabilities.md)

### 新增功能检查清单（速查）

UI 已浏览器化（一套 UI 走桌面 Tauri / 宽屏浏览器 / 手机浏览器），新功能必须登记到对应单一注册链：

- **新 Tauri command** → `rpc_dispatch.rs` 的 `registry!` 宏表登记（`read()` / `write(Sect)` / `write(None)`），或 `EXCLUDED` 显式排除系统级命令
- **新写命令**（改跨端可见状态）→ registry! 登记 `write(Sect)` + commands.rs 函数标 `#[cc_command(write = "sect")]`（出口自动广播 state-changed；写但不广播用 `#[cc_command(write)]`）
- **新事件** → `events.rs` 常量 + `emit_all`；远程会话类事件用 `remote_state::emit_event` / `emit_event_ex` / `emit_desktop_only`
- **新协议消息** → `remote/protocol/` 四处同步：crate 定义 → protocol.ts → samples/ → remote/docs/protocol.md
- **CLI 启动参数** → 只改 `pty::build_claude_args`，勿在前端另写一份
- **CLI 进程启动** → 目标只由 `platform::cli_target()` 决定（配置可运行 → 完整路径；否则 PathLookup → Unix execvp 裸名 / Windows Git Bash `-c` 裸名）；Unix 不经 shell 中转；遵守 spawn「返回即存活」契约
- **前端 invoke/listen** → 一律经 `src/api/transport.ts`（组件禁止直接 import @tauri-apps/api）

守卫测试、背景与踩坑说明 → [docs/development.md](docs/development.md)「新增功能检查清单（完整版）」

### 会话控制权 per-session（速查）

**冲突单位是单个会话的输入流，不是整台设备**——每个 PTY 会话独立归属（`remote_state.rs` `SessionEntry.active`），不同设备可同时操作不同会话。attach/claim 无条件抢占（后到优先，单用户多设备无协商），旧持有者收 `session-taken-over` 遮罩并保留订阅；**同一浏览器连接同一时刻只持有一个焦点会话**（切 Tab 先 detach+subscribe 旧会话再 attach 新会话，桌面 claim 也以 taken-over 通知而非 detached）；PtyInput/Resize 仅 active 持有连接生效；断连自动归还。离散写命令（rpc write）**不做控制权互斥**（白名单 + revision 排序 + state-changed 广播收敛）。全局控制权层（ControllerHolder/ControllerMask/controller-changed/controller.takeover）已退役；旧手机缓存 UI 的 takeover 消息 no-op、hello.ok controller 字段保留恒不发。**写命令必须走 `#[cc_command]` 宏出口广播 state-changed，且刷新回调链不得包含写操作（防写→广播→读→写死循环）**。

行为规则正本 → [remote/docs/protocol.md](remote/docs/protocol.md)「会话控制权（per-session active）」；机制摘要 → [docs/development.md](docs/development.md)「会话控制权（per-session active）与状态同步」

### 多实例设备级 Broker（速查）

一台电脑 = 一个常驻后台 **Broker**（一个设备身份/LAN 入口/relay 连接，存共享 config 不迁移）；多个 GUI 窗口 = Broker 成员。**GUI 不再参与 Gateway 选举**，PTY 始终留在 GUI。手机协议不感知窗口拓扑（会话条目带 instanceId/instanceName 分组字段，路由按 `pty_id → instance` 反查转发）。要点：

- **进程模型**：`cc-box --broker` 无 GUI 窗口；Windows/macOS 带托盘（标题 `CC-Box`），Linux 纯 tokio 无头。每个 GUI 启动确保 Broker 在线；`broker.lock`（OS 排他锁）保证单实例；**所有窗口关闭后 Broker 不退出**（托盘/远程入口保留）。
- **窗口管理**：Broker 记录成员 pid 与真实窗口标题；托盘顶部只有 `CC-Box`，**每个窗口直接是一级菜单项，单击即显示/聚焦**（不再有「窗口」父级，也不提供单窗口关闭/重启）；`window.command{close|show|openRemote}` 仍用于整体退出/更新/远程连接。新建窗口带 `--new-window`，停在项目页不自动进终端，Windows 用 `CreateProcessW(bInheritHandles=FALSE, CREATE_NO_WINDOW)` 避免控制台与句柄泄漏。零窗口时手机/桌面可 `rpc.call "spawn_new_instance"` 拉起空窗口；手机终端页有 stopped Tab 时也显示「启动窗口」覆盖层。
- **窗口名**：托盘与手机分组/实例选择器统一使用真实窗口标题（打开项目=项目名，项目页/未打开=`CC-Box`；`window.title` 更新时同步 `MemberEntry.name` 与已有会话 `instanceName`，同名去重）。
- **远程快捷动作**：托盘「远程连接」显示第一个可控制窗口并打开面板（无窗口先新建）；菜单顺序为 远程连接 → 刷新远程密钥 → 关闭/开启远程连接。**有更新时**：Broker 每 2h 检查（启动等 30s），菜单标题下出现「更新到 vX.Y.Z」且托盘图标叠红点；点击＝显示窗口并跳转 设置→更新（下载/安装/失败恢复仍在 GUI 更新页）。**破坏性操作统一用原生确认对话框**：关闭远程连接、刷新远程密钥、退出 CC-Box；窗口单击显示/新建/开启远程/自启动/更新不确认。
- **开机自启动**：仅 Broker（不开窗口）；入口在**设置 → 启动**与托盘勾选项，不在远程面板；Windows Run / macOS LaunchAgent / Linux XDG autostart，命令带 `--broker --autostart`；无托盘环境用 `cc-box --quit-broker`。
- **IPC**（`gateway_proto.rs` / `broker/ipc.rs`）：Windows Named Pipe（显式 DACL）/ Unix UDS（0700+0600+peer UID）；v2 首帧 `member.hello`（含 pid/windowTitle）+ `broker.secret`；成员上行 `window.title`/`state.changed`；管理操作独立 `admin` 首帧。旧版 GUI 仍走 `/internal/gw-ws` + `gateway.secret`（v1），无 pid/窗口指令。
- **状态同步**：成员本地写命令的 `state-changed` 经 `state.changed` 上行，Broker 重分配设备级 revision 后扇出其他 v2 成员与手机（窗口 A 修改刷新 B/C）；**发起成员不回发**（其本端已有 Tauri 本地通知 + 写回环抑制，二次回环会击穿抑制触发全量刷新——隐藏项目列表闪动回归），**成员经 Broker 时不再本地广播手机**（本地 revision + 设备级 revision 双发会让手机判 revision 跳变触发 state-resync-needed 全量重拉）；`PtyInput/PtyResize` 只校验该会话 active 归属、无控制面副作用（必须已有 attach）；PTY 退出由成员上行 `session.remove`，Broker 只广播一次 `session.exit`。
- **退出/更新**：Quit/PrepareForUpdate 先写回 `admin.result` 再退出；更新安装失败调用 `resume_broker_connection` 清除 update 标记/抑制并恢复；gateway.lock 心跳校验 owner + 原子写，解析失败按 mtime 判 stale；IPC server 创建失败直接退出释放 broker.lock。
- **客户端细节**：`tailSeq` 基线 claim/重 attach 时只抬高不降低；Linux AppImage 拉起 Broker 优先 `$APPIMAGE`；设置页开机自启动每 5s 校准。
- **单一有序队列**：每成员/连接一条队列；控制面与 `PtyInput` 可靠投递（`send().await`，5s 超时断开重同步），**绝不 try_send 静默丢输入**；`PtyOutput` 可丢但带 `seq`，客户端检测缺口并提示。
- **路由**：`gateway::dispatch` 分类 DeviceGlobal / SessionScoped / Create；`SessionCreated` **先登记路由再投递手机**；未知 pty_id 拒绝 + 推 `sessions` 快照，绝不回退 Broker 本地；成员断连清理路由/缓存/粘连/待处理 RPC/创建；重连全量快照 + `reset_remote_views`。
- **生命周期**：`broker.state.json` 区分 `user_quit`（退出 CC-Box，持久抑制；手动 GUI/`--autostart` 清除）、`update_in_progress`（安装前退出，版本变化后解除）；`remote.enabled=false` 不是生命周期状态（Broker 继续运行，仅停 LAN/relay）。旧 `idle_exit` 标记仅兼容读取。
- **所有窗口内容一致**：远程面板一律读 Broker 镜像，不区分宿主角色、不显示在线窗口数量；三个操作（更新连接码/刷新密钥/停止远程）与开机自启动都经 Broker 执行。
- 正本 → [remote/docs/protocol.md](remote/docs/protocol.md)「设备级 Broker IPC」；实施细节 → [remote/docs/multi-instance-gateway.md](remote/docs/multi-instance-gateway.md)

### 设备列表连通性（速查）

手机设备列表（TokenGate / 首页切换弹窗）打开时，对每个**非当前**设备做一次真实 WS `hello` 探测（复用 relay/LAN 同一 `/ws` 协议，不改后端），显示在线/离线/令牌失效/无法连接：`hello.ok` → 在线、`hello.error`（非 busy）→ 令牌失效、`device.status online:false` → 离线、超时/连接失败 → 不可达。探测连接拿到结论立即关闭，不重连、不写存储；当前设备沿用主连接 `connState`。不可达条目点击被拦截并在行内说明原因，配对可直接移除；HTTPS 页面无法探测 HTTP LAN 设备（混合内容）标「无法检测」。主 UI 已就绪后连接被拒（denied）时叠加 TokenGate 覆盖层（`:auto-boot="false"`，不销毁主 UI），提供改选设备/重新输入 token。

实现：`src/api/remote-probe.ts`（单次探测）+ `src/composables/useDeviceProbe.ts`（编排，key = `id|token`）；测试见 `tests/api/remote-probe.test.ts`、`tests/composables/useDeviceProbe.test.ts`。

### 响应式适配（速查，手机 = 视口 ≤768px 且触屏）

- **手机判定 = `(max-width: 768px) and (hover: none)`**：只按宽度会把桌面分屏右半屏误判为手机（底部 IconBar、快捷键工具条、设置顶部 tab 全冒出来）
- **纯样式分支**写 `@media (max-width: 768px) and (hover: none)`；**结构分支**用 `useIsMobile()`（`MOBILE_MEDIA_QUERY`，与 CSS 一致）；纯内容宽度自适应（按钮换行等）仍可用纯宽度断点
- **快捷键工具条（QuickKeyBar）只在 `isMobile` 渲染**——桌面端（鼠标设备）无论窗口多窄都不出现；「键盘」图标开关 + 「选择复制」同属工具条；**软键盘只由该图标开关唤出**（点击终端不弹键盘，滚动仍自动收起）
- 触屏设备所有 "hover 才显示" 的操作按钮必须包 `@media (hover: hover)`
- 终端字号手机 = 桌面设置 × 0.9；底部栏用 `--safe-area-bottom`；软键盘经 `visualViewport.height` 驱动终端 fit
- 触摸手势、复制阅读器、符号兜底、构建产物供给、会话身份链等细节 → [docs/development.md](docs/development.md)「响应式与触屏适配」「会话管理约定」

## 测试要求

- **开发必须搭配测试**：新增功能、修改逻辑、修复 bug 时同步编写或更新测试；**bug 修复必须先写测试复现**，确认失败后再修复
- **断言必须有独立真值来源**：期望值来自手算字面量、已知输入输出对或协议规范；禁止自闭环断言——期望值按实现同款逻辑算出、mock 返回什么就断言什么、set 后 get 回读、空输入不报错、常量/查表复述、序列化 round-trip 等于自身。写前自问「这条断言失败对应哪个真实 bug」，答不上来就不写
- **自动测试优先**：能用自动测试覆盖的场景必须写成自动测试；手动测试仅记录自动化无法覆盖的场景 → [docs/manual-test-cases.md](docs/manual-test-cases.md)
- **测试文件独立存放**：前端项目根目录 `tests/`（`npm test`）；后端 `src-tauri/src/tests/`（`cd src-tauri && cargo test`）
- **测试基础设施**：前端 Vitest + jsdom + `@tauri-apps/api/mocks`（mockIPC）；后端 `#[cfg(test)]` 模块、被测函数 `pub(crate)`；Store 测试 `setActivePinia(createPinia())` + mockIPC
- **命名与范围**：英文函数名 `Feature_SubFeature_SeqNum` + 中文注释；测纯函数、数据转换、解析、状态管理、边界与错误路径；不测 getter/setter、类型定义、简单 props 传递、第三方库
- 编写原则详情 → [docs/测试编写原则.md](docs/测试编写原则.md)
