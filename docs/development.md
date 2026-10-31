# CC-Box 开发细节手册

> CLAUDE.md 收缩后的**定向信息归档**：主文档只保留通用规则速查，本文件承载环境配置、开发检查清单完整版、远程机制摘要、会话管理与移动端适配细节。**处理对应事项时读取对应章节**，无需全读。

## Windows 环境配置

- **UTF-8 源文件禁止用 PowerShell Get-Content/Set-Content 处理**：Windows PowerShell 5.1 的 `Get-Content -Raw` 默认按系统 ANSI（GBK）解码，配 `Set-Content -Encoding utf8` 会把整个文件的中文写成不可逆乱码（`?` 替换丢字节，曾致 700 行测试文件只能靠会话 transcript 重放恢复）。批量文本处理用 Edit/Write 工具，或 Bash + sed/node。
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

**多设备联调**：设备凭证存在浏览器 localStorage，按页面 **origin** 隔离——两台电脑各跑 `dev:relay` 会产生两个 origin，手机上永远只看到当前一台。让第二台电脑连接第一台的 relay（不启动本地 relay）：

```bash
CC_BOX_RELAY_URL=<第一台IP>:8800 npm run dev:relay
```

手机只打开第一台的 relay 页面，分别配对两台电脑，之后即可在首页设备列表切换。LAN 直连（各设备自己的 IP）同样跨 origin，不能共享列表。

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

### 跨进程句柄继承拖死 8747（2026-10-25 修复，前项修复的更深层根因）

**现象**：实例 A 开远程（宿主 8747）→ 从 A 内 `spawn_new_instance` 打开 B → 关闭 A → B 开远程报"8747 被占用"。`netstat` 显示端口仍 LISTENING、归属**已死**的 A PID；`probe_host` 能连上但无 HTTP 响应（accept 逻辑随 A 死了），判不出宿主。

**根因**：WinSock socket 句柄默认 `HANDLE_FLAG_INHERIT=1`。B 由 A spawn（`bInheritHandles=TRUE`）时连带继承了 A 的 8747 listener 句柄；A 退出后端口被活着的 B 拖住——内核按句柄创建者（A）显示归属。A 的 WebView2 子进程（mojo 管道同样要求继承句柄）同理可能成为持有者。此路径下 A 的 `stop()` 再干净也救不了：句柄在别的进程手里。前项修复（stop await + 僵死探测）只覆盖本进程内的泄漏。

**修复**：`remote_server.rs` `clear_socket_inherit`——bind 成功后立即 `SetHandleInformation(sock, HANDLE_FLAG_INHERIT, 0)`（SOCKET 可直接作 Win32 HANDLE，Go net/libuv 同款做法），句柄只归本进程，端口随进程退出必释放。测试：`tests/remote_host.rs` `Listener_默认句柄可继承_复现孤儿端口_001`（锚定默认行为=缺陷面）+ `Listener_清除继承位后不可继承_002`。

**排查手段**（无 Sysinternals handle.exe 时）：`Get-CimInstance Win32_Process -Filter "ParentProcessId=<死PID>"` 找存活子进程（CC-Box 子实例 / 残留 msedgewebview2），持有者必在其一；清理它们或重启子实例即可释放端口。

### 多实例远程开关竞态加固（2026-10-25 修复二）

- **watchdog 跟随停服持锁**：`REMOTE_OP_LOCK` 从 commands.rs 移到 remote_server.rs（pub(crate)），config watchdog 的「复查 disabled → stop()」全程持锁——否则与用户并发 `set_remote_enabled(true)`（持锁写 true + start）竞争时，stop 会把刚 start 的新服务误杀，终态「开关显示开、实际没开」。锁内复查代际+配置，stop 内部不抢锁（无死锁）；watchdog 持锁最长 ~7s（2s 任务等待 + 5s 端口轮询），期间用户开关命令排队等待，可接受。
- **同类句柄继承闭环**：stop() 内 bind 试探 listener、hook_server 的随机端口 listener 同样 bind 后 `clear_socket_inherit`（hook 端口随机无冲突，但属同类泄漏，未来固定端口即复现 8747 问题）。
- **测试补缺**（tests/remote_host.rs）：`Start_宿主制跳过_001`（他实例 HTTP 在监听 → start Ok 且不置运行态）、`Start_静默占用报stale文案_002`、`SetRemoteEnabled_启动失败回滚enabled_003`（prev=false/true 双场景，防误停宿主）。注意测试占端口须绑 **0.0.0.0**：Windows 通配绑定与回环（127.0.0.1）同端口不冲突，绑回环挡不住 remote_server 的通配 bind。
- **历史说明**：以上为旧 GUI 宿主制；2026-10-26 起改为独立后台 Broker（见下节），不再有宿主/非宿主实例转移或重新选举。

### 设备级后台 Broker（2026-10-26 重构）

远程设备入口从「GUI 实例竞选 Gateway」改为「独立后台 Broker」：

- **进程**：`cc-box --broker`；Windows/macOS 带托盘（标题 `CC-Box`），Linux 纯 tokio 无头（托盘缺失优雅降级）。GUI 启动时按需拉起；`broker.lock` OS 排他锁保证单实例；**所有窗口关闭后常驻**，不再空闲退出；`cc-box --quit-broker` 可退出。
- **窗口管理**：成员 `member.hello` 带 pid/windowTitle，项目切换经 `window.title` 更新；托盘顶部只有 `CC-Box`，**每个窗口直接是一级菜单项，单击显示/聚焦**；不再提供单窗口关闭/重启；`window.command{close|show|openRemote}` 仍用于整体退出/更新/远程连接。新建窗口带 `--new-window`，停在项目页；Windows 用 `CreateProcessW` + `CREATE_NO_WINDOW` 避免 debug 控制台与句柄继承。零窗口时远程 `rpc.call "spawn_new_instance"` 由 Broker 拉起空窗口（12 上限 + 2s 冷却）。
- **远程快捷动作**：托盘「远程连接」显示第一个可控制窗口并打开面板（无窗口先新建）；顺序为 远程连接 → 刷新远程密钥 → 关闭/开启远程连接。破坏性操作弹原生确认对话框（关闭远程连接、刷新密钥、退出 CC-Box）。
- **跨窗口状态**：成员写命令的 `state-changed` 经 `GwUp::StateChanged` 上行，Broker 重分配 revision 后扇出所有 v2 成员与手机；PtyInput/PtyResize 只校验该会话 active 归属、无控制面副作用；PTY 退出只上行 `SessionRemove`，Broker 统一广播一次 `SessionExit`。
- **退出/更新可靠性**：Quit/PrepareForUpdate 先写回 admin 响应再退出；更新安装失败调用 `resume_broker_connection` 清除 update 标记/抑制并恢复 Broker；gateway.lock 心跳校验 owner + 原子写；IPC server 创建失败直接退出释放锁。
- **客户端细节**：`tailSeq` 在 claim/重 attach 时只抬高不降低；Linux AppImage 拉起 Broker 优先 `$APPIMAGE`；设置页自启动状态每 5s 校准。
- **开机自启动**：仅 Broker；入口在**设置 → 启动**与托盘勾选项，不在远程面板；Windows Run / macOS LaunchAgent / Linux XDG autostart，命令 `<exe> --broker --autostart`。
- **IPC**：Windows Named Pipe（显式 DACL）/ Unix UDS（0700 + 0600 + peer UID），4 字节长度 + JSON；v2 首帧 `member.hello`（含 pid）+ `broker.secret`；旧版 GUI 仍走 `/internal/gw-ws` + `gateway.secret`。
- **路由**：`gateway.rs` 不再有 Role/选举/Waiting；Broker 永远是路由中心。`SessionCreated` 先登记路由再回手机；未知 `pty_id` 拒绝并推行快照；成员断连清理待处理 RPC/创建并重置远端视图。
- **队列**：每成员/连接单一有序队列；输入/控制面可靠（超时断开重同步），`PtyOutput` 带 `seq`，客户端显示缺口。
- **生命周期**：`broker.state.json` 区分 `user_quit`/`update_in_progress`（旧 `idle_exit` 仅兼容读取）；`remote.enabled=false` 只是停 LAN/relay，不退出进程。手动打开 GUI 或 `--autostart` 清除 `user_quit`；更新安装前调用 `prepare_for_update`（保留发起窗口、关闭其余窗口）释放 exe。
- **统一面板**：所有窗口远程面板读同一份 Broker 镜像，不显示宿主角色/在线数量。
- **测试**：`cargo test`（`tests/gateway.rs`、`broker_ipc.rs`、`broker_lifecycle.rs`）+ `npm test`；macOS/Linux 行为需在对应平台本地构建验证（macOS 注意 locale 相关的排序断言需显式指定 locale）。
- **本地手工验证**：启动两个 GUI → 确认只有一个 `cc-box --broker`；任务管理器结束 Broker → GUI 自动恢复；托盘关闭单个/全部窗口 → Broker 仍存活；手机零窗口「启动窗口」；「退出 CC-Box」关闭全部窗口；开机自启动项开/关；LAN 端口被第三方占用时 relay 仍可用；旧版占口时面板显示 legacy 警告。

## 会话控制权（per-session active）与状态同步

> **行为规则正本见 [remote/docs/protocol.md](../remote/docs/protocol.md)「会话控制权（per-session active）」**（唯一权威；attach 抢占/断连归还/revision 过滤与跳变重同步细节以正本为准），本节为机制摘要。

**冲突单位是单个会话的输入流，不是整台设备**——每个 PTY 会话独立归属（`remote_state.rs` `SessionEntry.active: ActiveView`），不同设备可同时操作不同会话；离散写命令不做控制权互斥。全局控制权层（`ControllerHolder`/ControllerMask/controller-changed/controller.takeover）已退役：

- **attach 即抢占**（后到优先，单用户多设备无协商）：`session.attach` 无条件置 active；旧持有者收 `session-taken-over` 广播（排除接管者）并保留订阅继续收输出；桌面/手机「接管回来」= `pty_claim`（桌面，收回单终端并向旧持有者发 taken-over，仅桌面清遮罩）或重新 attach（浏览器，`sessionClaim` 单条消息，无前置）
- **同一连接单一焦点**：浏览器一次只看一个终端。切 Tab 时前端先 `session.detach` 旧会话 + `session.subscribe` 保留后台输出，再 attach/claim 新会话；旧 Tab 保持 running（自触发的 `session.detached` 忽略），桌面只对当前查看会话显示遮罩。离开终端页同样释放焦点
- **输入独占**：`pty.input`/`pty.resize` 仅该会话 active 持有连接生效；**断连自动归还**该连接全部会话（active → Desktop + `session-reclaimed` + 恢复桌面尺寸）
- **写命令直通**：rpc 写命令/session.create/kill 不做控制权检查（白名单/并发上限/revision 保留），靠 state-changed 广播收敛
- **旧版兼容**：旧手机缓存 UI 发 `controller.takeover` → no-op；hello.ok 的 controller 字段保留但服务端恒不发（serde skip），旧端解析 null=无人持有不遮罩；**新 Broker + 旧 GUI 成员混部**时旧成员本地 controller 残留可能致跨会话互斥退化（单会话自洽，双端升级后消除）
- **revision 版本号**：每次状态变化 +1，附在 `state-changed` payload 与 hello.ok/sessions；前端（remote-transport `filterStaleEvent`）单调过滤旧事件防乱序覆盖，**版本跳变触发 `state-resync-needed` 全 section 权威重拉**（兜底乱序漏刷）；**服务端保序**：`emit_state_changed` 取号与入队同一锁窗口（到达序=取号序，并发回归测试守卫），否则低版本事件后到被前端丢弃、状态永久丢失
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

- **权限模型**：读写范围锁定**授权项目内**（~/.claude/projects 出现过的路径，`is_within_known_project` canonicalize 锚点校验防 junction/symlink 逃逸）；上传写入收敛到会话项目 `.cc-box-files/`（客户端只传 ptyId+裸文件名，路径服务端拼接 + sanitize）；上传/下载均 write(None)（无控制权互斥，直通）
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
- **触屏适配**：所有"hover 才显示"的操作按钮必须包 `@media (hover: hover)`（触屏设备 hover:none 无悬停，按钮常显——SessionItem/ProviderCard 已改）；快捷键工具条（QuickKeyBar）补软键盘按不出的控制键（Esc/⇧Tab/Ctrl+J/Ctrl+U/↑↓←→）+「键盘」图标开关（软键盘唯一唤出入口，开启态 `qk-open` 高亮）+「选择复制」action 按钮；**点击终端区域不再自动唤出键盘**（用户反馈阅读输出时误弹打扰）
- **触摸滚动**＝xterm 6 内置 Gesture 系统原生接管（`XTermTerminal.vue` 不再自管触摸，**禁止在容器 capture 阶段 stopPropagation touch 事件**——那会阻断 Gesture 的 document 级监听；且 iOS 上首个 touchmove 未 preventDefault 即被 WebKit 手势接管发 touchcancel，旧自管层正是因此 iPhone 完全无法滚动）。Gesture 自带：像素跟手滚动 + 惯性衰减；按 mouse-reporting→滚轮序列 / 有 scrollback→viewport 滚动 / 否则→方向键三通道分发。GUI 侧仅补两件 Gesture 不管的事（容器 capture 监听 xterm 手势自定义事件，事件 dispatch 在 `.xterm-screen` 且 `bubbles=false`，事件名 `-xterm-gesturechange`/`-xterm-gesturetap` 为 lib 内部常量 typings 未导出，升级 xterm 后须 grep lib 复核）：① gesturechange |translationY|≥10 时 blur 收起已弹出的软键盘（高度回升只 fit 不滚底）；② gesturetap **不再聚焦**（点击终端不自动唤出软键盘，改由工具条「键盘」图标开关 `toggleSoftKeyboard` 聚焦/收起裸 textarea）——Gesture 对触点在 `.xterm-screen` 内的 touchstart preventDefault 仍会杀死合成 mousedown，故 ③ 的鼠标合成必须保留；③ 同一 gesturetap 里 mouse reporting 激活时（`term.element` 含 `enable-mouse-events` class，CLI 开启 1006 后 MouseService 按协议自动维护）按 TAP 的 pageX/pageY 合成 `mousedown`+`mouseup` 派发到 `term.element`（`buildTapMouseInit`/`src/utils/tap-mouse.ts` 换算 client 坐标，tapCount→detail）——②中被杀的合成鼠标事件同时切断了 xterm 的鼠标上报链路（SGR 序列只挂在 mousedown/mouseup 上），不补则手机 tap（点击切光标、vim/htop 点击）到不了 CLI；xterm 不检查 `isTrusted`，mousedown 内部会把 mouseup listener 动态注册到 document，bubbles 的合成 mouseup 冒泡触发 UP 序列并自动清理。**未开启 mouse reporting 时不合成**（点击维持聚焦语义，复制走快照阅读器；此时 SelectionService 未 disable，合成会误触拖选）。**软键盘只由工具条「键盘」图标（`toggleSoftKeyboard`）唤出，其余程序化路径一律不聚焦**（tap 终端不聚焦；`sendText` 移动端跳过 focus——工具条按键/侧边栏插入/接管回来聚焦会突然弹键盘打断操作；桌面无软键盘问题保持聚焦；`softKeyboardOpen`/`toggleSoftKeyboard` 经 defineExpose 暴露给 TerminalView，聚焦态由容器 focusin/focusout 统一跟踪）。**复制部分内容**＝「选择复制」按钮打开复制阅读器（TerminalCopyReader）：`captureSnapshot` 抓 buffer 全量快照 → `reflowSnapshot`（`src/utils/terminal-reflow.ts`）反渲染还原逻辑段落（isWrapped 拼接/围栏原样/满宽启发式 CJK 显示宽度/表格 diff 保护）→ 全屏 pre 展示，**系统原生长按选择手柄**选择 → getSelection → `copyText`（http 非安全上下文 execCommand 降级）；`@contextmenu.stop` 放行 Android 长按菜单（main.ts 拦截仅 PROD，须生产构建验证）；智能/原始双模式，误拼兜底
- **触屏已知限制**：Provider 列表拖拽排序（HTML5 drag）移动端不可用，待需要时补上/下移按钮
- **终端符号兜底替换**：浏览器远程模式下 CLI 输出的媒体控制符（"⏵⏵" 提示/⏸ 暂停，U+23EF/U+23F4–U+23FA）移动端字体缺字形，写入 xterm 前经 `mapTerminalGlyphs`（`src/utils/terminal-glyphs.ts`）替换为 ▶◀▲▼‖■●（均 1 列不影响布局）；桌面端有系统字体回退原样写入（`!isTauriEnv` 路径才替换）
- **构建产物供给**（remote/web-assets 共享 crate，两端同一实现）：磁盘目录优先；桌面默认 `embed` feature 内嵌兜底（debug 实时读 dist，改前端后 `npm run build` 在 8747 即时生效）；**relay 关闭 `embed`，二进制不含 UI，只服务 `CC_BOX_RELAY_WEB_DIR` 磁盘目录（未配置/ miss → 404）**——生产 relay 已配置 `/opt/cc-box/web`，纯前端更新 `npm run build` + scp dist 即生效，免重编免重启；`npm run deploy:relay` 也不再构建前端——完整流程见 [remote/docs/deploy.md](../remote/docs/deploy.md)「日常更新部署（复用手册）」
