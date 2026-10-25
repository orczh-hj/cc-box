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
│   │   ├── remote_state.rs     # 远程状态与消息处理（会话聚合、控制权、revision、广播）
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

**本地中继测试标准启动：一条命令 `npm run dev:relay`（脚本自动探测局域网 IP、拉起 relay + tauri dev；手机访问 `http://<本机IP>:8800`，任一进程退出全部退出）。**

## 环境与仓库（速查）

- **Windows**：MSVC 工具链 + C++ Build Tools（link.exe 须为 MSVC 版本）；配置细节 → [docs/development.md](docs/development.md)「Windows 环境配置」
- **双仓库**：GitHub（需代理）/ Gitee（直连）；**始终分 URL 推送，勿用多 URL origin** → [docs/development.md](docs/development.md)「双仓库同步」
- **打包**：首次下载 NSIS 组件需代理 → [docs/development.md](docs/development.md)「Windows 环境配置」

## 版本发布（速查）

```bash
npm run release -- --bump patch --notes "### Features\n- Add feature"
```

- 脚本自动完成：版本号 → CHANGELOG → 提交推送 → 标签 → CI → Release → OSS；失败后**重跑同一完整命令自动续跑**（不会二次 bump）
- **`--bump major/minor` 只在用户明确指定时使用**；`--notes` 必填且必须用英文
- **生产 relay 已启用磁盘分离**（`CC_BOX_RELAY_WEB_DIR=/opt/cc-box/web`）：前端（src/）改动只需 `npm run build` + `scp -r dist/* root@orczh.top:/opt/cc-box/web/`，免重编、免重启；只有 relay 二进制（remote/relay/、web-assets）改动才跑 `npm run deploy:relay`（先桌面后 relay）。未配置磁盘目录的部署仍编译期内嵌，须重编 relay
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
| [remote/README.md](remote/README.md)                         | **远程连接设计文档**：双模式架构、统一管理、消息协议、独占接管、中继部署、实施阶段 |
| [remote/docs/deploy.md](remote/docs/deploy.md)               | **中继部署指南**：构建、systemd、nginx 反代、桌面接入、本地测试、日常更新部署 |
| [remote/docs/protocol.md](remote/docs/protocol.md)           | **协议与行为规则正本**：消息定义、全局控制权、文件传输                       |
| [remote/docs/manual-test-cases.md](remote/docs/manual-test-cases.md) | **远程手动测试条目**：真机访问、状态圆点、断线重连、token 安全、快捷键工具条、多实例宿主制 |

外部参考：[Claude Code 线上文档](https://code.claude.com/docs/llms.txt)、[Tauri 2.x JS API 线上文档](https://v2.tauri.org.cn/reference/javascript/api/)

## 开发约定

- **远程操作一致性**：远程（手机）操作与桌面直接操作效果一致——桌面端原生实现单点执行，远程只是触发器 + 事件同步的镜像，不另写一份执行逻辑
- **远程文件下载走票据直链（所有浏览器统一）**：`file_download_ticket` 签发一次性票据 → 直连 `GET /dl/<ticket>` 流式返回、中继 `/?dl=<device>.<ticket>` 票据页（受限 WS 只允许 `file_download_ticket_chunk`/`ping`）。票据 **不绑连接**（系统浏览器/下载器接管后 WS 必断，不能被 `cleanup_conn` 回收）、128bit、TTL 10 分钟、256 次；旧桌面/旧 relay 经 `command not allowed` 与 HEAD 探测自动降级 Blob。relay 零改动（只按 token 前缀路由），详见 [remote/docs/protocol.md](remote/docs/protocol.md)「一次性下载票据」
- **避免重复实现**：同一逻辑只留一份（如 `spawn_pty_core`、`apply_hook_event`）；两端各自需要的状态靠事件同步（`session-renamed`、`hook-status`），不靠复制代码
- **离散操作必广播**：用户离散操作（建/关/重命名会话）经 Rust 广播事件让对端走同一个原生函数；远程安全守卫（白名单/并发上限/独占）作为前置检查包住共用核心，不因统一而放宽
- **用户级资源开关（skills/agents/mcp/plugins）**：后端单点 `store.rs` 的 `set_*_enabled`（桌面/远程同一函数）。skill/agent 禁用=移入 `~/.cc-box/disabled/<category>`，必须 **symlink 感知**（相对链接按目标位置重算、存在性用 `symlink_metadata`、disabled 扫描列出断链以便恢复）；前端 `state-changed` 刷新必须**静默**（`loadXxx(cwd, { silent: true })`，不切 loading 分支，否则整列表卸载重建）；**本端写回环必须抑制**（`markSelfWrite` 登记 → `applyStateSection` 消费并跳过该 section，条目原地响应；plugin 仍级联子项），后端列表按名称**稳定排序**（顺序与 enabled 解耦）；开关按 `(sourceType=user, name)` 定位，禁止裸 name
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

### 全局控制权与状态同步（速查）

同一时间只有一个设备可操作——桌面与所有远程设备对等互斥，由 Rust `remote_state.rs` 单点裁决，前端只展示（ControllerMask）。远程写操作无人持有时自动持有；显式接管是唯一转移方式；断连自动释放；revision 版本号单调过滤旧事件、跳变触发全量重拉。**写命令必须走 `#[cc_command]` 宏出口广播 state-changed，且刷新回调链不得包含写操作（防写→广播→读→写死循环）**。

行为规则正本 → [remote/docs/protocol.md](remote/docs/protocol.md)「全局控制权」；机制摘要 → [docs/development.md](docs/development.md)「全局控制权与状态同步」

### 响应式适配（速查，断点 ≤768px）

- **纯样式分支**写 `@media (max-width: 768px)`；**结构分支**用 `useIsMobile()`（与 CSS 断点一致）
- 触屏设备所有 "hover 才显示" 的操作按钮必须包 `@media (hover: hover)`
- 终端字号手机 = 桌面设置 × 0.9；底部栏用 `--safe-area-bottom`；软键盘经 `visualViewport.height` 驱动终端 fit
- 触摸手势、复制阅读器、符号兜底、构建产物供给、会话身份链等细节 → [docs/development.md](docs/development.md)「响应式与触屏适配」「会话管理约定」

## 测试要求

- **开发必须搭配测试**：新增功能、修改逻辑、修复 bug 时同步编写或更新测试；**bug 修复必须先写测试复现**，确认失败后再修复
- **自动测试优先**：能用自动测试覆盖的场景必须写成自动测试；手动测试仅记录自动化无法覆盖的场景 → [docs/manual-test-cases.md](docs/manual-test-cases.md)
- **测试文件独立存放**：前端项目根目录 `tests/`（`npm test`）；后端 `src-tauri/src/tests/`（`cd src-tauri && cargo test`）
- **测试基础设施**：前端 Vitest + jsdom + `@tauri-apps/api/mocks`（mockIPC）；后端 `#[cfg(test)]` 模块、被测函数 `pub(crate)`；Store 测试 `setActivePinia(createPinia())` + mockIPC
- **命名与范围**：英文函数名 `Feature_SubFeature_SeqNum` + 中文注释；测纯函数、数据转换、解析、状态管理、边界与错误路径；不测 getter/setter、类型定义、简单 props 传递、第三方库
- 编写原则详情 → [docs/测试编写原则.md](docs/测试编写原则.md)
