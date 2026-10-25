# CC-Box 开发细节手册

> CLAUDE.md 收缩后的**定向信息归档**：主文档只保留通用规则速查，本文件承载环境配置、开发检查清单完整版、远程机制摘要、会话管理与移动端适配细节。**处理对应事项时读取对应章节**，无需全读。

## Windows 环境配置

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

## 双仓库同步

项目同步推送到 GitHub 和 Gitee 两个仓库，方便国内访问。

- **GitHub**：`https://github.com/orczh-hj/cc-box`（需要代理）
- **Gitee**：`https://gitee.com/orczh/cc-box`（国内直连）

**同步机制**（2026-10 修正：**GitHub Actions 并不会自动同步 Gitee**——release.yml 注释声称同步但无此代码，Gitee 同步只能靠手动推送）：
1. release.js 发布脚本内置分仓库推送：GitHub（代理，阻断）+ Gitee（直连，失败仅告警可后补）——**不用多 URL `git push origin`**（一条命令跨两域，代理环境 Gitee 必失败）
2. 日常手动推送：分 URL 推（GitHub 带代理、Gitee 直连），勿用多 URL origin

**推送时注意**：推送到 GitHub 需先设置代理，推送到 Gitee 无需代理。

**注意**：2026-08 起发布流程不再创建 Gitee Release（代码仓库同步保留）；如需恢复，token 仍在 `git config --local gitee.token`。

## 远程功能本地全链路测试（改远程相关代码后的标准动作）

改 `remote/`（协议/relay）或远程相关前端/Rust 后，**在本地把中继链路完整跑一遍再部署**（relay debug 实时读 dist，两端都是新代码；比只测 8747 直连多覆盖 relay 透传与嵌入 UI）：

```bash
npm run dev:relay
```

一条命令完成（`scripts/dev-relay.mjs`）：自动探测局域网 IP → 启动本地 relay（`0.0.0.0:8800`，手机可达）+ 桌面 tauri dev（注入 `CC_BOX_RELAY_URL=<局域网IP>:8800`，勿用 127.0.0.1——手机打不开配对 URL）；任一进程退出即全部退出。手机扫桌面面板出的中继配对码（走本地 relay）或局域码（8747 直连）均可验证。探测按接口名分层（`scripts/lib/lan-ip.mjs`）：`en*` 物理网卡优先，`feth`/`utun`/`bridge`/`vmenet` 等虚拟接口仅在无其他候选时兜底（虚拟网卡常带 `192.168.x.x` 且排在前面，选中会让手机配对 URL 不可达）；多网卡仍有偏差时用 `CC_BOX_RELAY_IP=<IP> npm run dev:relay` 手工指定。详细环境变量与注意事项 → [remote/docs/deploy.md](../remote/docs/deploy.md)「本地测试（开发环境）」。

## 新增功能检查清单（完整版）

UI 已浏览器化（一套 UI 走桌面 Tauri / 宽屏浏览器 / 手机浏览器），新功能必须登记到对应单一注册链，防两端漂移：

- **新 Tauri command** → `rpc_dispatch.rs` 的 `registry!` 宏表登记（一行一命令：`"cmd" => read()/write(Sect)/write(None) => |参数| handler`）或 `EXCLUDED` 显式排除（系统级）。差集守卫测试直接解析 `lib.rs` 的 invoke_handler 注册表断言注册项 ⊆ RPC_WHITELIST + EXCLUDED，漏登记时测试失败强制评估
- **新写命令**（修改任何跨端可见状态：开关/Provider/配置）→ registry! 登记 `write(Sect)`，commands.rs 函数标 `#[cc_command(write = "sect")]`（置于 `#[tauri::command]` 之上，出口 Ok 自动 `emit_state_changed`——桌面 invoke 与远程 rpc 同一广播出口，无手动 emit）；写但不广播（专用事件）标 `#[cc_command(write)]`。守卫测试（`tests/state_broadcast.rs`）强制：写前缀命令必须登记、registry 与宏属性双向对拍、宏出口语义锚定（成功才广播）——漏接时"远程操作后其他端 UI 不刷新"由测试拦截，不再靠人肉记忆
- **新事件** → `events.rs` 常量 + `emit_all`；远程会话类事件用 `remote_state::emit_event`（桌面+广播）/`emit_event_ex`（排除接管者）/`emit_desktop_only`（仅桌面）
- **新协议消息** → `remote/protocol/` 四处同步：crate 定义 → protocol.ts → samples/ JSON 样例 → remote/docs/protocol.md（Rust 与 Vitest 契约测试共跑样例）
- **CLI 启动参数拼接** → 只改 `pty::build_claude_args`（Rust 单一核心，session.create 与桌面 spawn 共用），勿在前端另写一份
- **CLI 进程启动方式** → 目标由 `platform::cli_target()` 唯一决定：配置路径通过 `is_runnable_cli` → 完整路径；否则 `PathLookup` → Unix 用 `find_executable` 解析出可运行路径再 exec / Windows 经 Git Bash `-c` 裸命令（bash 自己做 PATHEXT/shim 查找）。Unix 不经 shell 中转（`bash -i -c` 的 job control 在 PTY 下会把 TUI 子进程停入后台致 Mac 白屏）；PATH 由 `refresh_path` 从登录 shell 刷新，是裸命令可用的前提。**spawn 契约 = 返回即存活**：`startup_check`（spawn_claude/spawn_shell 共用）在 50ms 启动期后 `try_wait()` 检测秒退，秒退转 Err 并携带 `StartupBuf` 缓存的 PTY 尾部输出（claude 自身报错不丢，前端红字显示）——否则事件落在前端终端实例建立之前全部丢失、表现为纯白屏
- **前端 invoke/listen** → 一律经 `src/api/transport.ts`（组件禁止直接 import @tauri-apps/api；桌面专属 API 用 `transport.env.isTauri` 守卫）

## 生产构建安全：xterm 死枚举 IIFE（终端卡死根因）

**症状**：生产构建（0.16.4 及之前）终端无输出、输入无回显、拖动窗口不 resize，但应用本身仍响应；`~/.cc-box/logs/*.error.log` 反复出现
`ReferenceError: Can't find variable: i @ .../xterm-vendor-*.js`。

**根因链**（勿只凭 dev 模式排查——dev 加载未压缩源码，不会复现）：

1. `@xterm/xterm` 的 `lib/xterm.mjs` 含 TS enum 编译产物 `let r; (te => (...))(r ||= {})` 的死代码（枚举建好后从未读取）。`@xterm/addon-webgl` 同类。
2. `vite.config.ts` 的 `build.target` 含 `safari13`，esbuild 先把 `||=` 降级为 `|| (r = {})`；随后语法压缩把未初始化的 `r` 折叠成 `void 0`，并删掉已无读取的 `let r` 声明，产出 `(void 0||(i={}))` —— 对未声明标识符赋值（esbuild 0.21.5/0.24.2/0.25.0 均复现）。
3. ESM 严格模式下该行抛 `ReferenceError`。它发生在 xterm `WriteBuffer._innerWrite → parser.parse` 内：抛错跳过 `_bufferOffset` 前进与后续 `_scheduleInnerWrite`，此后 `term.write()` 只入队不处理——输出、回显、resize 重绘全部消失。
4. Claude CLI 启动时发送 DECRQM（`CSI ? Ps $ p`）进入 `requestMode`，因此每次新建会话都会触发，卡死的是该 Terminal 实例。

**修复**：

- `scripts/lib/patch-xterm-enum.ts`：Vite `enforce:'pre'` 插件（仅 build），把所有 `@xterm/*/lib/*.mjs` 中的 `)(x||={})` 改写为等价的 `)(x||(x={}))`，避开降级折叠路径。单个 addon 无此模式属正常；**整个构建一次都没命中**时 `buildEnd` 会 warn——说明依赖形态已变，需复核补丁是否仍需要。
- `scripts/lib/dist-safety.ts` + `scripts/verify-dist-safety.ts`：`npm run build` 在 vite build 后扫描 `dist/**/*.js`，命中 `void 0||(x={})` 这类坏赋值即失败退出（dist 缺失/无 JS 也失败）。桌面与 relay/手机 Web UI 共用同一份 dist，因此同一道闸覆盖两端。
- 回归测试：`tests/scripts/xterm-bundle-safety.test.ts`（函数作用域 fixture 必须保留——顶层 `let` 不复现；用与生产一致的 target/minify 断言）。

**不要**改用「去掉 safari13」（`minimumSystemVersion` 为 10.13，旧 WebKit 会整包解析失败）或「升级 esbuild」快修（新版对 safari13 的降级直接报错）作为默认方案。

**发布注意**：前端产物修复后 `npm run build` + `scp -r dist/* root@orczh.top:/opt/cc-box/web/`（生产已磁盘分离，免重编/免重启）；仅 relay 二进制改动才需 `npm run deploy:relay`。手机端强刷获取新 bundle。

## 远程服务机制要点

- `remote_server.rs`：局域网直连（axum WS + Web UI，web-assets 磁盘优先/内嵌兜底 + token 认证）；**stop 轮询等待端口释放**——Windows abort 后 IOCP 清理延迟曾致端口长期 LISTENING 残留、bind 10048
- `remote_state.rs`：会话聚合、**256KB TailBuffer 输出回放**、working/pending 状态机唯一实现、局域网/中继共用唯一 handler
- `remote_client.rs`：连接 cc-box-relay、手机连接桥接、断线重连

### 多实例与端口释放（2026-10-25 修复）

**原问题**：多实例共享 `~/.cc-box/config.json` 时，在实例 A 关闭远程只停 A 的 server；端口实际由先启动的实例 B 监听时，B 继续服务而两端 UI 都显示已关闭。叠加 `stop()` 只 `abort()` 不等待任务结束，Windows 上可能留下「LISTENING 但 HTTP 无响应」的僵死 listener——新实例 bind 10048，`probe_host` 也因超时认不出宿主。

**修复**：
- `stop()` abort 后 `await` serve 任务（2s 上限），确保 `axum::serve` 的 future 被 drop、listener 真正关闭。
- `start()` 成功后启动 config watchdog（3s 低频轮询共享配置）：任一实例写 `enabled=false`，正在监听的宿主随之 `stop()` 并断开中继。
- `set_remote_enabled(true)` 启动失败时**恢复原 enabled 值**（不再一律写 false），避免把另一实例的正常服务误停。
- `bind_with_retry` 最终失败时探测端口是否「可连但不响应」，错误信息提示重启僵死实例。
- `dev-relay.mjs` 用同步 `taskkill /T` 等待进程树杀完再 `process.exit`，避免 relay/vite 残留占用 8800/1420。

## 全局控制权（设备互斥）与状态同步

> **行为规则正本见 [remote/docs/protocol.md](../remote/docs/protocol.md)「全局控制权」**（唯一权威；attach 不能抢占/接管入口/revision 过滤与跳变重同步细节以正本为准），本节为机制摘要。

同一时间只有一个设备可操作——**桌面与所有远程设备对等参与互斥**。控制权由 Rust `remote_state.rs` `StateInner.controller`（`ControllerHolder::Desktop | Remote(connId)`）单点裁决，前端只展示（`ControllerMask.vue` 全局遮罩+接管按钮，桌面/浏览器同构挂载）：

- **远程写操作即持有**：rpc 写命令/session.create/kill/attach 在无人持有时自动持有并广播 `controller-changed`（payload id：`"desktop"`/连接 id/空串；桌面同时经 Tauri emit 下发）；**桌面普通操作不自动持有**（不抢占远程）；他人（含桌面）持有时远程写被服务端拒绝（attach 回 `session.attach.denied`、写命令回错误）
- **显式接管是唯一转移方式**：远程 `controller.takeover` 消息；桌面 `controller_takeover` command（遮罩「接管」按钮）与 `pty_claim`（终端接管回来顺带获得）。转移 + 剥离旧持有者全部终端归属归还桌面；接管确认按"重连"语义全量重同步（sessions.list 重拉 + 终端重 attach，见 remote-transport `applyControllerChanged`）
- **断连自动释放**；持有期间其他设备（含桌面）整界面遮罩（ControllerMask `selfLabel` 响应式维护——重连换连接 id 后旧快照会自己遮自己；桌面 UI 启动/重载经 `controller_status` command 补初始快照）
- **revision 版本号**：每次状态变化 +1，附在 `state-changed`/`controller-changed` payload 与 hello.ok（controller：null=无人/0=桌面/连接 id）/sessions；前端（remote-transport `filterStaleEvent`）单调过滤旧事件防乱序覆盖，**版本跳变触发 `state-resync-needed` 全 section 权威重拉**（兜底乱序漏刷）；**服务端保序**：`emit_state_changed`/`broadcast_controller_changed` 取号与入队同一锁窗口（到达序=取号序，并发回归测试守卫），否则低版本事件后到被前端丢弃、状态永久丢失
- **state-changed 统一广播**：写命令落盘后（`#[cc_command]` 宏出口自动 emit）全端按 section 权威刷新（前端状态中心 `src/state/sections.ts` 查表分发：section → loader 注册表，plugins 级联子项、providers/config 设置面板打开时暂存防覆盖编辑中表单、关闭补刷；App.vue 只订阅转发，新功能在注册表加一行）。**注意**：刷新回调链不得包含写操作（曾因 loadAppConfig 内无条件 doSyncEnv 写回 + 事件刷新形成 写→广播→读→写 死循环，风暴塞满通道致 RpcResult 丢失、config.json 原子写窗口被击穿丢失 remote 配置）

### 用户级资源开关（skills/agents/mcp/plugins）

后端单点 `store.rs` 的 `set_skill_enabled` / `set_agent_enabled` / `set_mcp_server_enabled` / `set_plugin_enabled`（桌面 invoke 与远程 rpc 调同一函数，两端行为天然一致）：

- **skill/agent 禁用 = 移动到 `~/.cc-box/disabled/<category>/`**。用户级 skill/agent 常以**符号链接**安装（`~/.claude/skills/x -> ../../.agents/skills/x`），移动必须 symlink 感知：
  - 相对链接按**目标位置**重算相对路径（`move_entry`/`relocate_symlink`），不能直接 rename——否则链接搬进 disabled 目录即断链；
  - **相对路径必须以真实目录为基准**：`src.parent()`/`dst.parent()` 先 `canonicalize`（`canonical_dir`）再拼目标。`~/.claude` 本身是符号链接（dotfiles/stow/chezmoi）时，内核按真实目录解析 `..`，词法计算会算出不存在的路径；
  - 存在性/冲突判定一律用 `symlink_metadata`（`path_entry_exists`），断链也算"存在"；`Path::exists()` 会把断链判成不存在，导致备份既扫不到也恢复不了；目标位是断链残留时直接覆盖（active/disabled 两侧同理），否则对列表不可见的残留会永久阻塞恢复；
  - disabled 扫描（`scan_disabled_skills`）**必须列出断链**（description 为空），否则该类备份只能手工 `mv` 恢复；历史断链在 enable 时按目标目录重新解析并修复；
  - agent 的列表名来自 frontmatter `name`，移动用 `find_agent_file` 扫描定位：**frontmatter 命中优先于文件名命中**，有 frontmatter 名却不等的文件不能按 stem 匹配（否则会移动错 agent）；多匹配报 ambiguous。
- **前端刷新必须静默**：`state-changed` 回环/级联刷新走 `loadXxx(cwd, { silent: true })`——只换数据、不切 `loading`。面板用 `v-if="loading"` 渲染，切 loading 会让整块列表卸载重建（展开态、滚动位置全丢）；首屏/切项目才显示 loading。loader 内置请求序号丢弃乱序响应。
- **本端写回环抑制（条目原地响应）**：开关成功会广播 `state-changed` 并回环给发起方；发起方已乐观更新，再刷新会替换数组/重排列表。因此 toggle 前 `markSelfWrite(section)`（失败 `cancelSelfWrite`），`applyStateSection` 消费一次后**跳过该 section 的 loader**；plugin 的级联子项（skills/agents/mcp）仍照常刷新。其他设备没有登记，照常静默刷新（跨端同步不受影响）。登记带 5s TTL，切项目时清空，避免广播丢失后误吞后续事件；**`state-resync-needed` 触发的权威重拉必须带 `{ resync: true }` 绕过并清除登记**（兜底路径不能被回环抑制吞掉）。失败回滚的身份包含 `sourceLabel`（active/disabled 同名时不能只按 `enabled` 定位，会翻错行）。
- **列表顺序与 enabled 解耦**：`get_all_skills` / `get_all_agents` / `get_all_mcp_servers` 末尾 `sort_by_name` 稳定排序。否则禁用项从 disabled 目录读出后被追加到用户组末尾，开关会让条目"跳位"。
- **开关定位用 `(sourceType=user, name)`**（`findToggleIndex`），不能用裸 `name`：store 同时含 project/user/plugin 项，同名 project 项排前面会被误改（点 user 项却改 project 项 → 看起来"操作无效"）。
- **plugin 开关**走 `claude plugin enable/disable` 子进程（可能 >15s），远程调用传 `timeoutMs: 60000` 避免远程先超时；`plugins list --json` 有 2s TTL 缓存（一次开关级联刷新 4 个 section，缓存把 4 个子进程收敛为 1 个），plugin 变更后主动失效。
- **CLAUDE_CONFIG_DIR**：skills/agents 的读取与开关已尊重该环境变量（`claude_config_dir()`）；MCP/plugins 等历史路径仍为 `~/.claude`。

### 远程文件传输（浏览器模式专属，FilesPanel）

手机经 WS 与桌面互传文件，**纯 rpc 分块命令**（无新协议消息，relay 零改动）：512KB 串行 base64 块（< relay 4MiB 消息上限），实现单点 `file_transfer.rs`，传输逻辑层 `api/fileTransfer.ts`，UI `components/files/FilesPanel.vue`（IconBar 按钮 `v-if="!isTauriEnv"`）。正本见 [remote/docs/protocol.md](../remote/docs/protocol.md)「文件传输」：

- **权限模型**：读写范围锁定**授权项目内**（~/.claude/projects 出现过的路径，`is_within_known_project` canonicalize 锚点校验防 junction/symlink 逃逸）；上传写入收敛到会话项目 `.cc-box-files/`（客户端只传 ptyId+裸文件名，路径服务端拼接 + sanitize）；上传/下载均 write(None)（受 controller_gate，无控制权被拒）
- **可靠性**：幂等 chunk（`offset+len==written` 重发不重写，ACK 丢失安全）；finish 用 `hard_link` 原子 no-replace 提交（绝不覆盖，冲突自动 `(n)` 改名）；下载会话固化 size/mtime，源变化即拒；会话绑定 conn_id（`connId`/`viaRelay` 由 handle_rpc 服务端注入覆盖伪造值）
- **双限额（流量成本）**：`config.remote.fileTransferMaxDirect`（默认 200MB）/`fileTransferMaxRelay`（默认 20MB，中继流量 ≈2.7× 文件大小；`0`=禁用）；判定依据是「经由中继」（连接表 `via_relay`），非「公网」——局域 relay 本地测试可调高
- **清理三层**：RPC 入口惰性 TTL（10 分钟）+ setup 定时 sweep + 启动扫已知项目 `.upload-*.part`

## 会话管理约定

- **会话身份链**：CLI `--resume` 会 fork 新 sessionId，`TerminalTab.sessionIds` 记录演变历史（旧 id 保留认领）；**所有恢复入口（App/TerminalView 历史、remote-session-created）统一走 `sessionStore.resumeSessionTab`**（复用优先：命中切 Tab/重启，未命中才新建），防二次 spawn、重复 Tab 与名字变 sessionId 编码
- **用户显式关闭双路径对称广播 `remote-session-closed`**（桌面 pty_kill / 远程 session.kill，先广播后 kill）→ 各端联动删 Tab，自然退出（pty-exit）只置 stopped 保留 Tab
- **重启（restartTab）清旧 PTY 走 quiet kill**（pty_kill/session.kill 带 `quiet=true`，不广播 closed——否则各端 closeTab 删 Tab，重启被误判为关闭；pty-exit 照常置 stopped 保留 Tab 等 respawn 绑回）
- **CLI 路径解析单一实现（四条不变量）**：
  1. **共享解析策略**：`platform::cli_target()` 是检查与 spawn 唯一入口——配置路径必须通过 `is_runnable_cli`（Unix 普通文件 + 执行位；Windows PATHEXT 扩展名），否则返回 `PathLookup`（Unix：`find_executable` 解析出可运行路径后 exec；Windows：裸名交给 Git Bash/PowerShell）。不要在两处各写一份判断。
  2. **运行时只读配置**：spawn / `run_checks` 都不写 `config.json`；`claudePath` 只由用户设置或安装器写入。自动检测结果仅作为运行态（`CheckResult.source`）返回。
  3. **显式标记来源**：`CheckResult.source = "configured" | "path"`，UI 可区分用户配置与 PATH 检测，不靠优先级隐式推断。
  4. **统一状态反馈**：`appStore.cliHealth` 是唯一状态源（检查结果 / `pty-error` / spawn 成功三路更新），TerminalView 状态条消费它并提供重试入口。
  - 反例（Mac 事故）：只判 `exists()` 会把目录或缺执行位路径直接拿去 exec，execve 报 EACCES，表象是「终端打不开、白屏」；裸命令 + PATH 查找则由平台自然跳过坏文件。
- **spawn 失败必须可见**：`XTermTerminal` 的失败 Tab 也要登记占位实例（`ptyId: ''`）并把错误红字写进终端——wrapper 由 `terminalInstances` 驱动渲染，不登记则 `waitForElement` 永远等不到元素，错误只进 console，用户看到纯白屏。占位实例没有 PTY 事件，由 `tabs.size` 变化兜底回收。
- remote 会话列表经 `RemoteSession.sessionId` 下发（hook 建立 ptyId→sessionId 映射），浏览器端采纳 Tab 名优先取历史会话名（未加载时回退项目名，加载后 backfillTabNamesFromHistory 回填）
- **会话结束并入近期会话缓存（本地合并，零请求）**：`cachedRecentSessions` 启动时只加载一次，会话结束（closeTab/closeAllTabs/closeOtherTabs/handlePtyExit）时把 Tab 已知信息（sessionId/名称/项目路径）经 `mergeTabIntoRecentSessions` → `appStore.mergeRecentSession` 按 sessionId 去重就地并入缓存，首页「近期会话」即时可见——不重拉 getHomeData（低效且重置项目分页）；无 sessionId 的 Tab 不并入；缓存条目 working/pending 清 false（已结束）

## 响应式与触屏适配（手机 = 视口 ≤768px 且触屏）

一套 UI 走桌面 Tauri / 宽屏浏览器 / 手机浏览器。**手机判定必须同时满足视口 ≤768px 与 `hover: none`**——只按宽度会把桌面分屏（右半屏常 <768px）误判为手机，导致桌面出现底部 IconBar、快捷键工具条、设置顶部 tab 等移动布局。

- **纯样式分支**一律写 `@media (max-width: 768px) and (hover: none)`（IconBar 底部横栏、SidebarPanel 抽屉、ProjectSelectView 单栏折叠、TerminalView 工具条排布等）
- **纯内容宽度自适应**（与设备能力无关，桌面窄窗仍需）：ProviderCard 按钮换行、StartupSection env-key 比例宽度，保持 `@media (max-width: 768px)`
- **结构分支**（两套 DOM，如 SettingsView 竖导航/顶部 tab）用 `useIsMobile()`（`src/composables/useMediaQuery.ts` 的 `MOBILE_MEDIA_QUERY`，与 CSS 断点一致）
- **终端字号**：手机 = 桌面设置 × 0.9（`XTermTerminal` 的 `effectiveFontSize`），断点跨越自动重算
- **软键盘**：App.vue 同步 `visualViewport.height` 到 app-root 高度，驱动终端 fit
- **safe-area**：底部栏用 `--safe-area-bottom`（env(safe-area-inset-bottom)），视口 meta 带 viewport-fit=cover
- **触屏适配**：所有"hover 才显示"的操作按钮必须包 `@media (hover: hover)`（触屏设备 hover:none 无悬停，按钮常显——SessionItem/ProviderCard 已改）；快捷键工具条（QuickKeyBar）补软键盘按不出的控制键（Esc/⇧Tab/Ctrl+J/Ctrl+U/↑↓←→）+「选择复制」action 按钮（软键盘由点击终端区域唤出，「键盘」按钮已按用户决策移除）
- **触摸滚动**＝xterm 6 内置 Gesture 系统原生接管（`XTermTerminal.vue` 不再自管触摸，**禁止在容器 capture 阶段 stopPropagation touch 事件**——那会阻断 Gesture 的 document 级监听；且 iOS 上首个 touchmove 未 preventDefault 即被 WebKit 手势接管发 touchcancel，旧自管层正是因此 iPhone 完全无法滚动）。Gesture 自带：像素跟手滚动 + 惯性衰减；按 mouse-reporting→滚轮序列 / 有 scrollback→viewport 滚动 / 否则→方向键三通道分发。GUI 侧仅补两件 Gesture 不管的事（容器 capture 监听 xterm 手势自定义事件，事件 dispatch 在 `.xterm-screen` 且 `bubbles=false`，事件名 `-xterm-gesturechange`/`-xterm-gesturetap` 为 lib 内部常量 typings 未导出，升级 xterm 后须 grep lib 复核）：① gesturechange |translationY|≥10 时 blur 收起已弹出的软键盘（高度回升只 fit 不滚底）；② gesturetap 兜底 `term.focus()`——Gesture 对触点在 `.xterm-screen` 内的 touchstart preventDefault 杀死了合成 mousedown，而 xterm lib 内无 TAP 消费者，不兜底则软键盘无法唤出。**软键盘只由用户 tap 终端唤出，其余程序化路径一律不聚焦**（`sendText` 移动端跳过 focus——工具条按键/侧边栏插入/接管回来聚焦会突然弹键盘打断操作；桌面无软键盘问题保持聚焦；`focus()` expose 已随「键盘」按钮移除删除）。**复制部分内容**＝「选择复制」按钮打开复制阅读器（TerminalCopyReader）：`captureSnapshot` 抓 buffer 全量快照 → `reflowSnapshot`（`src/utils/terminal-reflow.ts`）反渲染还原逻辑段落（isWrapped 拼接/围栏原样/满宽启发式 CJK 显示宽度/表格 diff 保护）→ 全屏 pre 展示，**系统原生长按选择手柄**选择 → getSelection → `copyText`（http 非安全上下文 execCommand 降级）；`@contextmenu.stop` 放行 Android 长按菜单（main.ts 拦截仅 PROD，须生产构建验证）；智能/原始双模式，误拼兜底
- **触屏已知限制**：Provider 列表拖拽排序（HTML5 drag）移动端不可用，待需要时补上/下移按钮
- **终端符号兜底替换**：浏览器远程模式下 CLI 输出的媒体控制符（"⏵⏵" 提示/⏸ 暂停，U+23EF/U+23F4–U+23FA）移动端字体缺字形，写入 xterm 前经 `mapTerminalGlyphs`（`src/utils/terminal-glyphs.ts`）替换为 ▶◀▲▼‖■●（均 1 列不影响布局）；桌面端有系统字体回退原样写入（`!isTauriEnv` 路径才替换）
- **构建产物供给**（remote/web-assets 共享 crate，两端同一实现）：默认 rust-embed 内嵌（debug 实时读 dist，改前端后 `npm run build` 在 8747 即时生效）；配置 `CC_BOX_RELAY_WEB_DIR` / `CC_BOX_WEB_DIR` 指向磁盘目录后优先读盘（miss 回退内嵌）——**生产 relay 已配置 `/opt/cc-box/web`，纯前端更新 `npm run build` + scp dist 即生效，免重编免重启**；未配置磁盘目录的部署仍是编译期嵌入、须重编 relay——完整流程见 [remote/docs/deploy.md](../remote/docs/deploy.md)「日常更新部署（复用手册）」
