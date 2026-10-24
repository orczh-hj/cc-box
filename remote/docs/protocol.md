# CC-Box 远程连接协议

协议正本双端定义：Rust crate `remote/protocol/src/lib.rs`（桌面/relay 用）与 TypeScript `remote/protocol/protocol.ts`（浏览器 UI 用）。固定 JSON 样例在 `remote/protocol/samples/`，由 Rust 测试与 Vitest 契约测试（`tests/remote/protocol.test.ts`）共同执行，保证两端序列化不漂移。

> **行为规则唯一权威**：设备互斥/控制权（谁能操作、如何接管）、revision 过滤等**行为规则以本文档为正本**；README（设计背景）、CLAUDE.md（索引）、manual-test-cases（测试步骤）中的相关描述均为摘要引用，如有不一致以本文档为准，且不应在别处复述完整规则（防多处维护漂移）。

**协议变更检查清单**（改任何消息时逐项核对）：共享 crate 定义 → protocol.ts → JSON 样例与契约测试 → 本文档。

- 版本：`PROTOCOL_VERSION = 1`（`hello` / `hello.ok` 握手校验）
- 中继（relay）不解析 payload，应用层消息升级无需动 relay 二进制

## 消息总表

### 手机 → 桌面（ClientMessage）

| type | 字段 | 说明 |
|---|---|---|
| `hello` | `ver, client, token` | 认证握手。token 两种：局域直连 = 桌面静态 token（连接层等值校验）；中继 = token v2 `<device_id>.<secret>`（relay 按前缀路由，**secret 由桌面 Hello 分支校验**——命中配对 pending 时固化为持久设备，LRU 上限 16） |
| `sessions.list` | `{}` | 拉取运行中会话列表 |
| `projects.list` | `{}` | 可操作项目白名单 |
| `session.create` | `path, sessionId?, skipPermissions?, customArgs?` | 新建（无 sessionId）或恢复（--resume sessionId）会话 |
| `session.kill` | `id, quiet?` | 关闭会话（ptyId）。显式关闭：先广播 `remote-session-closed` 再 kill（各端联动删 Tab），与桌面 `pty_kill` 双路径同语义；`quiet=true` 为重启场景内部清理——不广播 closed（各端保留 Tab 等 respawn，pty-exit 只置 stopped） |
| `sessions.history` | `path` | 项目历史会话（恢复入口） |
| `session.subscribe` | `id` | 只读订阅输出（附回放快照） |
| `session.unsubscribe` | `id` | 取消订阅 |
| `session.attach` | `id, cols, rows` | 接管会话（附回放快照 + resize）。**仅无人持有全局控制权时自动获得控制权，不能抢占**——他人持有时被拒（`session.attach.denied`），须先 `controller.takeover` |
| `session.detach` | `id` | 归还终端给桌面 |
| `pty.input` | `id, data` | 输入注入（仅 active 端 + 控制者生效） |
| `pty.resize` | `id, cols, rows` | 终端尺寸（仅 active 端 + 控制者生效） |
| `ping` | `{}` | 30s 心跳 |
| `rpc.call` | `id, cmd, args` | RPC 桥：调白名单 Tauri command（rpc_dispatch 宏表；**写命令受全局控制权拦截**） |
| `controller.takeover` | `{}` | 抢占全局控制权（设备互斥唯一转移方式；成功以 `controller-changed` 广播确认，旧持有者终端被剥离归还桌面） |

### 桌面 → 手机（ServerMessage）

| type | 字段 | 说明 |
|---|---|---|
| `hello.ok` | `ver, deviceName, connId, controller?, revision` | 认证通过（桌面设备名）。`connId` = 本连接唯一标识；`controller` = 当前全局控制者（null=无人持有；**0=桌面持有**（conn_id 从 1 起，0 保留）；>0=远程连接 id。前端显示"被其他设备控制中"遮罩的判定依据）；`revision` = 状态版本基线（旧桌面端缺省 0） |
| `hello.error` | `error` | 认证失败 |
| `sessions` | `list: RemoteSession[], revision` | 运行中会话列表 + 状态版本快照（全量刷新基线） |
| `projects` | `list: RemoteProject[]` | 项目列表 |
| `session.created` | `id, path` | 新建成功（id = ptyId；发起者自动持有控制权） |
| `session.create.failed` | `error` | 新建失败（含控制权被他人持有时拒绝） |
| `history.sessions` | `list: RemoteHistorySession[]` | 历史会话 |
| `session.subscribed` | `id, tail` | 订阅成功，`tail` = 回放快照 |
| `session.unsubscribed` | `id` | 退订确认 |
| `session.attached` | `id, cols, rows, tail` | 接管成功，`tail` = 回放快照；发起者自动持有控制权 |
| `session.attach.denied` | `id, reason` | attach 被拒：全局控制权被其他设备持有（先 `controller.takeover`） |
| `session.detached` | `id, reason` | 归还确认 |
| `session.status` | `id, working, pending` | hook 状态机变化 |
| `session.exit` | `id` | 会话退出 |
| `pty.output` | `id, data` | 增量输出流 |
| `pong` | `{}` | 心跳回复 |
| `rpc.result` | `id, ok, data?, error?` | RPC 应答（写命令被控制权拦截时 `ok=false` 提示接管） |
| `rpc.event` | `name, payload` | 控制面事件隧道（与 Tauri 事件同构）：`state-changed`（写命令状态广播）、`controller-changed`（全局控制权）、`remote-session-created/closed`（会话建立/显式关闭联动）、`session-renamed`、`session-viewed`（attach 打开即已读，服务端清 pending 并双端广播）、`hook-status` 等 |

## 全局控制权（设备互斥）与 revision

同一时间只有一个设备可操作——**桌面与所有远程设备对等参与互斥**。控制权由 Rust 后端裁决（`remote_state.rs` `StateInner.controller: Option<ControllerHolder>`，`Desktop | Remote(connId)`），前端只做展示（`ControllerMask.vue` 全局遮罩 + 接管按钮，桌面/浏览器同构）：

- **远程自动持有**：远程写操作（rpc 写命令、`session.create/kill`、`session.attach`）在无人持有时自动使发起者持有；**桌面普通操作不抢占**（无自动持有路径）
- **不能抢占**：他人（含桌面）持有时远程写操作被服务端拒绝（`rpc.result` 错误 / `session.attach.denied` / `session.create.failed`）；桌面被遮罩时 UI 挡住操作入口
- **显式接管（唯一转移方式）**：远程 `controller.takeover` 消息 / 桌面 `controller_takeover` command（ControllerMask 接管按钮）/ 桌面 `pty_claim`（终端「接管回来」顺带获得）——转移控制权 + 剥离旧持有者全部终端归属（归还桌面并恢复桌面尺寸）+ 广播
- **广播**：`rpc.event("controller-changed", {id, data: revision})`，`id` = `"desktop"` | 连接 id 数字串 | 空串（无人）；桌面端同时经 Tauri emit 同名事件下发（桌面遮罩判定）；接管者收到广播确认后按"重连"语义全量重同步（`sessions.list` 重拉 + 终端重 attach）
- **初始快照**：远程端 `hello.ok` 自带 `controller`/`revision`；桌面 UI 启动/重载（如切 Tab reload）会错过历史广播，ControllerMask 挂载时经 `controller_status` command（rpc 白名单已登记，只读）主动查询补快照
- **断连释放**：远程持有者连接断开自动释放并广播空 id
- **revision**：每次状态变化（写命令广播、控制权变更）+1，附在 `controller-changed` / `state-changed`（写命令统一状态广播，payload `{id: section, data: revision}`）与 `hello.ok` / `sessions` 下发；前端单调过滤（不大于已应用版本的事件丢弃），防止旧事件乱序覆盖新状态。**版本跳变**（到达 revision > 已应用+1，中间事件可能丢失——如全量应答基线先于在途事件入队）时不丢弃该事件，并触发一次**权威重同步**（前端全 section 重拉，transport 广播 `state-resync-needed`），兜底乱序漏刷。**服务端保序**：`emit_state_changed` / `broadcast_controller_changed` 的 revision 取号与入队在同一锁窗口完成（跨线程并发时到达序 = 取号序，回归测试 `Controller_并发广播到达序revision严格递增` 守卫）——否则低版本事件后到被前端过滤丢弃，状态永久丢失

## 写命令统一状态广播（state-changed）

任何一端（桌面或远程）执行写命令（开关 skill/agent/mcp/plugin、Provider CRUD/激活、配置保存）成功后，广播 `rpc.event("state-changed", {id: section, data: revision})`（桌面经 Tauri emit 同构下发），全端按 section 权威刷新。

- **广播执行**：`#[cc_command(write = "sect")]` 属性宏（`src-tauri/macros`）置于 `#[tauri::command]` 之上，函数出口 Ok 自动 `emit_state_changed`——桌面 invoke 与远程 rpc 调用同一函数，广播单点在出口（失败不广播），无手动 emit
- **声明登记**：`rpc_dispatch.rs` 的 `registry!` 宏表一行一命令（`read()` / `write(Sect)` / `write(None)` 三态），同时生成 RPC 白名单、写权限拦截标记与广播 section；section 值由 `StateSection` 枚举编译期约束（线上小写字符串：skills/agents/mcp/plugins/providers/config）
- **守卫测试**（`tests/state_broadcast.rs`）：写前缀命令必须登记 write(...)、registry 与宏属性双向对拍（防拼写错误/漏登记）、宏出口行为锚定（成功才广播）
- **前端消费**：状态同步中心 `src/state/sections.ts` 查表分发（section → loader；`alsoLoad` 级联、设置面板打开期间 defer 暂存关闭补刷），App.vue 只订阅转发

## 会话显式关闭与自然退出

用户显式关闭（桌面 `pty_kill` command / 远程 `session.kill`，双路径同语义）**先广播 `remote-session-closed`（payload `{id: ptyId}`）再 kill**：各端（桌面及其他浏览器端）收到后联动删 Tab，"打开的标签页"不残留；先发保证 closed 先于 pty-exit 到达（同连接保序，晚到会因 ptyId 已清而匹配不到 Tab）。自然退出（CLI 内 `/exit` 或进程崩溃）只走 `pty.output`/`session.exit` 链路，Tab 保留显示"已停止"（可重启）。

## 文件传输（rpc.call 命令，无专用消息）

浏览器远程模式与桌面互传文件，**不新增协议消息**——全部经 `rpc.call` 白名单命令（8 个 `file_*`），512KB 串行 base64 分块（单消息 < relay 4MiB 上限；串行等 ack 天然背压）。实现单点 `src-tauri/src/file_transfer.rs`，传输逻辑 `src/api/fileTransfer.ts`，UI `FilesPanel.vue`（仅浏览器模式渲染）。

| 命令 | 读/写 | 参数（camelCase） | 返回 |
|---|---|---|---|
| `file_transfer_limits` | read | — | `{directMax, relayMax}` |
| `file_list` | read | `path`（须在授权项目内） | `[{name, path, isDir, size?, modified?}]`（过滤 `.upload-*.part`，目录在前） |
| `file_upload_begin` | write(None) | `ptyId, fileName, size` | `{uploadId, targetPath, fileName}` |
| `file_upload_chunk` | write(None) | `uploadId, offset, dataB64` | `{received}` |
| `file_upload_finish` | write(None) | `uploadId` | `{targetPath}`（冲突自动 `(n)` 改名） |
| `file_upload_abort` | write(None) | `uploadId` | `null`（幂等） |
| `file_download_begin` | write(None) | `path` | `{downloadId, fileName, size, mtime?}` |
| `file_download_chunk` | write(None) | `downloadId, offset` | `{dataB64, bytes, eof}` |

- **服务端注入**：`handle_rpc` 对 `file_` 前缀命令权威注入 `connId`/`viaRelay`（覆盖客户端任何伪造值；未知连接按中继从严）。会话（uploadId/downloadId）绑定发起连接，他人使用返回同一错误文案（不泄露存在性）；桌面 invoke 路径（conn_id=None）上传/下载直接拒绝
- **权限**：读写范围锁定授权项目内（~/.claude/projects 已知项目，canonicalize 锚点校验防 `.cc-box-files` 符号链接/junction 逃逸）；上传路径服务端拼接（客户端只传裸文件名 + sanitize：拒 `..`/分隔符/Windows 保留名等），落盘 `<会话项目>/.cc-box-files/`；上传与下载均受全局控制权拦截（他人持有被拒）
- **可靠性**：chunk 幂等（`offset+len == written` 的重发不重写，ACK 丢失后前端重试安全）；finish 用 `hard_link` 原子 no-replace 提交，绝不覆盖已有文件；下载会话固化 size/mtime，源文件变化即返回 `source file changed during download`；eof 不清会话（最后一块重发幂等），TTL 10 分钟回收
- **双限额（流量成本）**：`config.json` `remote.fileTransferMaxDirect`（默认 200MB）/`remote.fileTransferMaxRelay`（默认 20MB——中继流量 ≈2.7× 文件大小：双程 + base64 膨胀；`0` = 禁用）。判定依据是「经由中继」（连接表 `via_relay`：relay 桥接 true / LAN 直连 false），非「公网」——局域网自建 relay 可调高
- **清理三层**：begin/list 入口惰性 TTL sweep（chunk/finish 不触发）+ lib.rs setup 定时（600s）+ 启动扫描已知项目 `.cc-box-files/*.upload-*.part`（崩溃遗留，仅 mtime >10 分钟且目录 canonicalize 后仍在项目内才删）；连接断开时即时回收该连接的传输会话
- **审计**：begin/finish/abort/download_begin 记 `~/.cc-box/logs`（方向/连接/路径/大小）

## 关键结构

### RemoteSession

```json
{ "id": "<ptyId>", "type": "claude|shell", "cwd": "<path>", "working": false, "pending": false, "sessionId": "<可选>" }
```

- `id` 是 ptyId（Rust 注册表键，与 Claude sessionId 不同）
- `sessionId`（可选）：Claude 会话 id，hook `SessionStart` 等事件建立映射后由桌面端填充；shell 会话与旧版桌面端缺失。手机端用它把重连恢复的 Tab 与历史会话关联，**防止从历史恢复同一对话时二次 spawn**

### 回放快照（tail）

`session.subscribed` / `session.attached` 的 `tail` 字段为该会话自 spawn 起的输出回放（attach/subscribe 即可完整恢复终端内容，无需重开会话）：

- 预算 `TAIL_BUDGET = 256KB`（remote_state.rs），分块 ring buffer，超预算淘汰最旧块（小于 64B 的小块并入尾块，防逐键回显积累海量小块）
- 淘汰后头部对齐 ANSI 转义边界（丢弃首个 ESC 前的残缺片段），避免半截转义序列把 xterm 已写内容抹掉
- 快照在锁内先于订阅插入入队（同锁 + mpsc FIFO 保证 `SessionAttached` 先于后续 `PtyOutput` 增量）。`push_output` 的 tail 写入与订阅者广播是两个锁窗口，attach 恰插入其间时新订阅者可能重复收到一帧（快照已含该帧且增量再发一次）；窗口极窄且终端 TUI 全屏重绘场景无感
- 快照入队失败（慢消费者）时桌面主动断开该连接，客户端重连重拉

## 样例

每个消息在 `remote/protocol/samples/` 有一个固定 JSON 样例（`client-*.json` / `server-*.json`），两端契约测试按目录共跑：改动消息结构时同步更新样例，两端测试强制失败直至一致。
