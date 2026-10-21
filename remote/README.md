# CC-Box 远程连接设计

> 状态：**R1–R3 + UI 统一化已实现**（局域网直连 + attach 独占交互 + 手机新建/关闭 + 公网中继 + 主应用 UI 浏览器化）；通知（R4）按产品决策不做。本目录承载远程连接相关的全部代码与文档。

## 1. 目标

- **填补官方方案空白**：Claude Code Remote Control 不支持 API key，CC-Box 用户大量使用第三方 Provider（GLM 等），无远程方案可用
- 手机浏览器（PWA）打开即用：查看会话状态与终端、输入、审批
- **双模式网络**：局域网直连（低延迟）+ 自建中继（orczh.top，公网穿透，桌面零入站端口）
- 一次配对长期有效、可吊销；远程功能默认关闭
- **一套 UI 走天下**：主应用 UI（src/）同时运行于桌面 Tauri、宽屏浏览器、手机浏览器；不再维护独立手机 Web UI（原 remote/web 已删除，Web UI = 主应用 vite 构建产物 dist，由 remote/web-assets 共享 crate 供给两端：磁盘目录优先、rust-embed 内嵌兜底）

非目标：云执行（代码始终在用户电脑上）、E2E 加密（协议预留）、多用户协作、原生 App。

## 2. 参考

| 方案 | 借鉴 | 教训/局限 |
|---|---|---|
| ZCode Remote | 桌面出站长连接 + 云中继 + QR 配对 | 扫码凭证临时，网页关闭须重扫 |
| Claude 官方 RC | 同拓扑，扫码即用 | 仅 Pro/Max，不支持 API key |
| Happy Coder | 本地 CLI → 中继 → 手机 App，推送 | 依赖第三方云 |
| SSH + tmux | attach/detach 语义、隧道思路 | 配置门槛高，无配对/推送 |

## 3. 架构

```
局域网：手机 ──HTTP/WS──► remote_server.rs（axum，桌面）
公网：  手机 ──WSS──► nginx(cc.orczh.top) ──► cc-box-relay ◄──出站WSS── remote_client.rs（桌面）
```

两种传输汇合到桌面 Rust 侧**同一套应用层 handler**，最终都落在：

```
remote handler ──► pty.rs（tee 输出 / write 注入 / resize）
              ──► hook_server.rs（working/pending 状态转发）
```

要点：

- **桌面是权威**：会话、PTY、权限判定全在桌面 Rust；中继纯路由，不解析应用层 payload
- 手机扫码 QR 携带双地址（直连 + 中继），先试直连、不通走中继，自动切换

## 4. 统一管理（核心原则）

全链路环节多（手机 Web UI ↔ relay ↔ remote_client/remote_server ↔ PTY/hook），任何协议变更横跨多处代码。对策：**每个概念只有一个事实来源、每条逻辑只有一条代码路径**。

| 统一点 | 做法 | 防什么错 |
|---|---|---|
| **协议定义** | `remote/protocol/` 是唯一正本：Rust 共享 crate（relay 与 src-tauri 共用）+ `protocol.ts`（前端用）+ 每条消息一份固定 JSON 样例 | 改消息漏改某端 |
| **契约测试** | Rust（serde round-trip）与 Vitest（TS 解析）跑**同一批 JSON 样例**，序列化永不漂移 | 两端字段不一致 |
| **消息处理** | 局域网/中继两个传输共用 `remote_state.rs` 同一套 handler，新增消息只改一处 | 两条传输行为分叉 |
| **命令单一注册表** | `rpc_dispatch.rs` 的 `registry!` 宏表一行一命令：`"cmd" => read()/write(Sect)/write(None) => \|参数\| handler`，一次生成 RPC 白名单、分发 match、广播 section、远程写权限标记四样注册（浏览器经 rpc.call 调 Tauri command）；**差集守卫测试**直接解析 `lib.rs` invoke_handler 注册表断言 ⊆ RPC_WHITELIST + EXCLUDED（无人肉镜像） | 新增 command 漏配白名单/写权限/广播范围 / 多处签名漂移 |
| **写命令自动广播** | `#[cc_command(write = "sect")]` 属性宏（`src-tauri/macros`）置于 `#[tauri::command]` 之上，函数出口 Ok 自动 `emit_state_changed`——桌面 invoke 与远程 rpc 调同一函数，广播单点在出口，无手动 emit；守卫测试三道（写前缀必须登记、registry↔宏属性双向对拍、宏出口成功才广播的行为锚定） | 漏广播导致"远程操作后其他端 UI 不刷新" / 桌面远程双口径漂移 |
| **前端状态同步中心** | `src/state/sections.ts`：section → loader 注册表（查表分发、`alsoLoad` 级联、`deferWhileSettingsOpen` 暂存防覆盖），App.vue 只订阅转发——新功能登记一行，不改 App.vue 与多个 Store | state-changed 消费逻辑散落多文件、新 section 漏接 |
| **事件单一出口** | `events.rs` 常量 + `emit_all`（Tauri emit + 远程广播二合一）；会话类事件 `remote_state::emit_event` / `emit_event_ex`（排除接管者）/ `emit_desktop_only` | 每个 emit 点手写两遍、事件名散落 |
| **会话创建单一核心** | `pty::build_claude_args(resume, skip, custom_args)` 纯函数，session.create 与桌面 spawn 共用同一参数拼接规则 | 前端与 Rust 拼接规则两份漂移 |
| **前端单一传输层** | `src/api/transport.ts` 三件套（接口 / tauri / remote），tauri.ts 与 provider.ts 导出签名不变，组件零感知环境 | 60 个 invoke 封装双轨 |
| **Web UI 构建** | 一条 `vite build` 产出主应用 dist；静态服务收敛在共享 crate `remote/web-assets`（ServeDir 磁盘优先 + rust-embed 内嵌兜底），relay 与桌面 remote_server 共用同一实现，且支持磁盘目录热更（见 deploy.md 3.5「仅前端更新」） | 两端 UI 版本漂移、功能不对等、静态服务逻辑两份 |
| **接管状态机** | `active_view` 只在 Rust 一处判定，桌面遮罩/手机提示均由事件驱动渲染 | 多端状态不一致 |
| **日志追踪** | 三端统一 `[remote]` 前缀 + conn_id + 消息 type，一眼定位断链环节 | 链路长难排查 |
| **协议演进** | 中继不解析 payload，应用层升级只动桌面 + 手机两端，relay 二进制不用动 | 升级需全链路协同 |

**协议变更检查清单**（改任何消息时逐项核对）：共享 crate 定义 → protocol.ts → JSON 样例与契约测试 → `remote/docs/protocol.md`。

## 5. 组件与目录

```
remote/
├── README.md            # 本设计文档
├── protocol/            # 协议唯一正本
│   ├── Cargo.toml       #   cc-box-remote-protocol crate（src-tauri 与 relay 共用）
│   ├── src/lib.rs       #   消息定义 + Rust 契约测试
│   ├── protocol.ts      #   TS 类型与解析（src/api/remote-transport 引用，Vitest 契约测试共跑样例）
│   └── samples/         #   JSON 固定样例（两端契约）
├── web-assets/          # cc-box-web-assets 共享 crate（Web UI 静态服务：磁盘目录优先 + 内嵌兜底，relay 与桌面共用）
├── relay/               # cc-box-relay 中继服务器（R3，独立 Rust 二进制，依赖 protocol + web-assets crate）
└── docs/                # protocol.md、deploy.md、manual-test-cases.md
```

**Web UI 已统一**：不再有独立手机 Web UI（原 remote/web 删除）。浏览器访问的是主应用 UI（src/，Vue 3 + xterm.js），经 `src/api/transport.ts` 传输层自动切换到 WS 远程实现；构建即主应用 `vite build`，产物 dist 由 `remote/web-assets` crate 统一供给——环境变量指向的磁盘目录优先（`CC_BOX_RELAY_WEB_DIR` / `CC_BOX_WEB_DIR`，前端可独立 scp 更新免重编），未配置/未命中回退 rust-embed 编译期内嵌。

桌面侧模块（挂 `src-tauri/src/`，依赖 protocol crate）：

| 模块 | 职责 |
|---|---|
| `remote_server.rs` | 局域网 WS 服务器 + Web UI 静态资源（经 web-assets crate：`CC_BOX_WEB_DIR` 磁盘优先、内嵌兜底） |
| `remote_client.rs` | 中继出站客户端（Bearer device_key，指数退避重连 1s→30s） |
| `remote_state.rs` | 应用层 handler 唯一实现：会话聚合、消息分发、`active_view` 状态机、全局控制权、state-changed 广播（revision 取号与入队同锁窗口保序） |
| `rpc_dispatch.rs` | **命令单一注册表**：`registry!` 宏表（read()/write(Sect)/write(None) 三态）+ `StateSection` 枚举 + 远程写权限拦截 + 差集守卫测试（解析 lib.rs） |
| `macros/`（crate） | `cc-box-macros` proc-macro：`#[cc_command(write = "sect")]` 写命令出口自动广播（桌面 invoke 与远程 rpc 同一口径） |
| `pty.rs`（改动） | 输出 tee + ring buffer（256KB，快照从换行边界对齐防 ANSI 截断）+ `active_view` 字段 |
| `hook_server.rs`（改动） | hook 事件在 emit 前端之外同步 broadcast 到远程连接 |

Web UI 页面（= 主应用 UI）：TokenGate 认证门（token/配对码）→ 与桌面完全相同的视图流（项目选择 → 终端主视图 → 侧边栏面板 → 设置），仅系统级功能（installer、更新、文件管理器、新实例）在浏览器模式隐藏。功能范围：**全功能减系统级**——RPC 白名单覆盖项目/会话/配置/agents/skills/MCP/plugins/provider 全量命令。

## 6. 协议

### 6.1 中继层（relay 边界，仅路由）

```json
桌面→中继  {"type":"register","device_id":"..."}            // 头带 device_key
桌面→中继  {"type":"ping"}                                    // 25s keepalive（防反代掐断空闲连接）
中继→手机  {"type":"device_status","online":true}
手机→中继  {"type":"data","payload":{...}}                   // 路由到绑定桌面
桌面→中继  {"type":"data","to":"<conn_id>","payload":{...}}  // 路由到指定手机连接
```

### 6.2 应用层（手机 ↔ 桌面，两种传输同构）

R1 已实现消息集（ver=1；正本在 `remote/protocol/src/lib.rs` 与 `protocol.ts`，样例在 `samples/`）：

| 方向 | type | payload | 说明 |
|---|---|---|---|
| 手机→桌面 | `hello` | `{ver, client, token}` | 连接认证（必须是首条消息；token 校验在连接层/relay，`hello.ok` 统一由唯一 handler 回复——中继模式依赖此路径） |
| 桌面→手机 | `hello.ok` / `hello.error` | `{ver, deviceName}` / `{error}` | 认证结果（错误即断开） |
| 手机→桌面 | `sessions.list` → `sessions` | `{}` → `{list:[{id,type,cwd,working,pending}]}` | 多端可见，不受独占限制 |
| 手机→桌面 | `projects.list` → `projects` | `{}` → `{list:[{path,name}]}` | 项目列表（新建/恢复入口） |
| 手机→桌面 | `sessions.history` → `history.sessions` | `{path}` → `{list:[{id,name,lastActiveAt}]}` | 项目历史会话（恢复用） |
| 手机→桌面 | `session.create` → `session.created` / `session.create.failed` | `{path, sessionId?, skipPermissions?, customArgs?}` / `{id,path}` / `{error}` | 新建（无 sessionId）或恢复（--resume）会话；启动选项与桌面一致（拼 CLI 参数） |
| 手机→桌面 | `session.kill` | `{id, quiet?}` | 关闭会话：显式关闭先广播 `remote-session-closed`（各端删 Tab）；`quiet=true` 重启场景内部清理不广播（保留 Tab 等 respawn） |
| 手机→桌面 | `session.subscribe` → `session.subscribed` | `{id}` → `{id, tail}` | 只读订阅，附输出回放快照（TailBuffer 256KB，ANSI 边界对齐） |
| 手机→桌面 | `session.unsubscribe` → `session.unsubscribed` | `{id}` | 取消订阅 |
| 桌面→手机 | `pty.output` | `{id, data}` | 增量输出流 |
| 桌面→手机 | `session.status` | `{id, working, pending}` | hook 状态机变化推送 |
| 桌面→手机 | `session.exit` | `{id}` | 会话退出；订阅不存在会话也以此回复 |
| 手机→桌面 | `rpc.call` → `rpc.result` | `{id, cmd, args}` → `{id, ok, data?, error?}` | **RPC 桥**：调用任意白名单 Tauri command（rpc_dispatch 宏表，约 44 个；installer/PTY/桌面级命令显式排除） |
| 桌面→手机 | `rpc.event` | `{name, payload}` | 控制类事件隧道（hook-event、session-taken-over/reclaimed 等；remote-transport 映射回 Tauri 同名事件，组件零改动） |
| 双向 | `ping` / `pong` | `{}` | 30s 心跳 |

R2 已实现（控制权独占，见 6.3）：`session.attach`（接管 + ring buffer 快照）、`pty.input`、`pty.resize`、`session.detach`、`session.detached`、桌面 Tauri 事件 `session-taken-over` / `session-reclaimed`。**PTY 数据流不走 rpc**——继续走 session.create/attach/pty.input/pty.resize/PtyOutput 专用消息（attach 即订阅、单 mpsc 保序、独占语义免费）；rpc.event 仅作控制面事件隧道。

桌面内部 Tauri 事件：`emit("session-taken-over")` → 前端该 tab 全遮罩"已在远程端接管"；`emit("session-reclaimed")` → 解除遮罩。

### 6.3 active_view 状态机（控制权独占）

产品决策：**同一时间只有一端能看到和控制某个终端**。状态：`Desktop | Remote(conn_id)`。

```
远程 attach        Desktop ──► Remote(new)：resize 远程尺寸，taken-over 广播（桌面遮罩）
远程互抢（新规则）  Remote(old) ──takeover──► Desktop ──attach──► Remote(new)：
                   takeover 剥离旧持有者终端（reclaimed），attach 时 taken-over 广播（旧端遮罩；订阅保留）
被接管端点"接管回来" Remote(new) ──► Remote(me)：takeover（或桌面持有期间直接 attach）+ taken-over 广播
桌面点"接管回来"    Remote ──► Desktop：resize 桌面尺寸，reclaimed 广播（全部解除遮罩）
远程 WS 断开       Remote ──► Desktop：自动归还 + resize 回桌面（持有权不赖在远程端上；控制权释放）
PTY 退出           清理状态
```

规则：**终端级** active_view 由 attach 驱动（远程端受全局控制权约束，见 6.4）；输入/resize 只接受 active 端；**被接管只是显示层暂时遮罩——旧持有者（桌面或另一远程端）订阅保留，输出流持续同步**，遮罩上的「接管回来」按钮随时可重 attach 抢回（claim 传当前实际尺寸，重 attach 的 tail 快照跳过防重复）。`sessions.list` 与 `hook.event` 不受独占限制。

### 6.4 全局控制权（设备互斥）

> **规则正本见 [docs/protocol.md](docs/protocol.md)「全局控制权」**（唯一权威，此处仅为设计摘要；attach 不能抢占、接管入口、revision 过滤与跳变重同步等细节以正本为准）

产品决策：**同一时间只有一个设备可操作——桌面与所有远程设备对等参与互斥**。状态 `StateInner.controller: Option<ControllerHolder>`（`Desktop | Remote(connId)`），Rust 单点裁决，前端只展示（`ControllerMask.vue` 全局遮罩 +「接管」按钮，桌面/浏览器同构挂载）：

| 场景 | 行为 |
|---|---|
| 无人持有 | 远程写操作（rpc 写命令/session.create/kill/attach）自动使发起者持有 + 广播 `controller-changed`；**桌面普通操作不自动持有**（不抢占远程） |
| 他人（含桌面）持有 | 远程写操作被服务端拒绝：attach 回 `session.attach.denied`、写命令回 rpc 错误、create 回 failed；**attach 不能抢占**；桌面被遮罩时 UI 挡住操作入口 |
| 显式接管（唯一转移方式） | 远程 `controller.takeover` 消息；桌面 `controller_takeover` command（ControllerMask「接管」按钮）与 `pty_claim`（终端「接管回来」顺带获得）。转移 + 剥离旧持有者全部终端归属（归还桌面 + resize 恢复）+ 广播；接管者以广播确认并按"重连"语义全量重同步（sessions.list 重拉 + 终端重 attach） |
| 持有者断连 | 自动释放 + 广播空 id（其他设备遮罩消失） |

配套 **revision 版本号**（`StateInner.revision`）：每次状态变化 +1，附在 `state-changed`（写命令统一广播，section = skills/agents/mcp/plugins/providers/config）/`controller-changed` payload 与 `hello.ok`/`sessions`；前端 `filterStaleEvent` 单调过滤旧事件防乱序覆盖，版本跳变触发 `state-resync-needed` 权威重拉兜底。写命令广播由 `#[cc_command]` 宏在函数出口自动执行（登记见 `rpc_dispatch.rs` `registry!` 宏表，守卫测试三道强制），协议细节 → [docs/protocol.md](docs/protocol.md)。

**运行中会话恢复**：浏览器端 Tabs 是内存态，刷新/新设备连接后由 `App.vue` 常驻订阅（`remote-sessions` 事件）重拉 `sessions.list` 重建 Tabs（不自动 attach）；Tab 名优先取历史会话名（sessionId 匹配），历史未加载时回退项目名、加载完成后 `backfillTabNamesFromHistory` 自动修正。用户点击 Tab/进入项目时 `ensureBrowserAttached` 才 attach 接管。WS 重连后（`remote-connected` 事件）attach 标记失效，自动重拉列表并重 attach 当前 Tab（接管确认同此语义）。

## 7. 配对与凭证

| 凭证 | 持有方 | 生命周期 | 用途 |
|---|---|---|---|
| `device_key` | 桌面 ↔ 中继 | 长期，存 `~/.cc-box/config.json` | 桌面注册 |
| `pair_code` | 一次性 | TTL 5 分钟 | QR 配对 |
| `access_token` | 手机 ↔ 中继 | 长期，可吊销，localStorage | 手机连接 |

流程：桌面开启远程 → 连上中继 → 显示 QR（`https://<relay>/#pair=<码>`；局域网 QR 为 `http://<IP>:<port>/#token=<token>`，fragment 不进服务器日志）→ 手机扫码提交配对码 → 中继校验后颁发 access_token → 此后打开即连，桌面设置页可管理/吊销已配对设备。

权限：首版配对即读写；协议预留 `role: readonly`。

## 8. 安全

- 中继模式全程 TLS（nginx + Let's Encrypt）；局域网明文 HTTP + token（同网段可接受）
- 桌面零入站端口（出站 WSS 穿透 NAT）；远程默认关闭；停用/换中继时在途连接立即拆除
- 中继不解析 payload：明文仅自托管中继可见，未来 E2E 只改两端不动中继
- 令牌均为随机 32 字节 base64url；配对码 8 位 hex、5 分钟 TTL、`/api/pair` 按 IP 失败锁定（5 次锁 60s），抗暴力猜解
- conn_id 全局唯一命名空间（局域网与中继共用分配器，双模式并存不冲突）；远程会话创建限已知项目白名单 + 并发上限

## 9. 部署（orczh.top）

```bash
scp cc-box-relay user@orczh.top:~        # SSH 仅部署通道，不进数据面
# systemd 服务监听 127.0.0.1:8xxx，nginx 新增子域反代（复用现有实例）：
server {
  server_name cc.orczh.top;
  location / {
    proxy_pass http://127.0.0.1:8xxx;
    proxy_http_version 1.1;
    proxy_set_header Upgrade $http_upgrade;
    proxy_set_header Connection "upgrade";
    proxy_read_timeout 3600s;
  }
}
# certbot -d cc.orczh.top
```

relay 全内存态（设备路由表 + 配对码 + 令牌），重启即清，无数据库。随版本发布二进制 + docker-compose.yml，社区用户可自托管。

## 10. 实施阶段

| 阶段 | 内容 | 交付判据 | 状态 |
|---|---|---|---|
| **R1 局域网只读** | protocol crate + remote_server + hook 转发 + 输出 tee + Web UI 会话列表与输出 tail | 同 WiFi 手机看到会话状态与输出 | ✅ |
| **R2 局域网交互** | attach 独占接管 + xterm.js 渲染 + 输入注入 + resize + 桌面遮罩 | 手机完整操作终端，与桌面互抢无缝 | ✅ |
| **R3 中继公网** | relay + remote_client + 一次性配对码 + 手机新建/关闭会话 | 公网打开即连，断线自动恢复 | ✅ |
| **UI 统一化** | 删除 remote/web；主应用 UI 浏览器化（transport 传输层 + rpc.call 桥 + TokenGate）；主 dist 双端嵌入 | 宽屏浏览器全功能可用（全功能减系统级），Playwright 端到端通过 | ✅ |
| **注册链收敛**（2026-10） | 命令单一注册表（registry! 合并两表）+ 写命令自动广播（#[cc_command] 宏）+ 前端状态同步中心（sections.ts）+ Web UI 与二进制解耦（web-assets crate，磁盘目录热更通道） | 新增普通业务功能只需「实现业务逻辑 + 声明命令元数据（registry! 一行 + 宏属性）+ 提供 UI（loader 登记）」，权限/广播/同步/守卫测试由公共机制承担；仅前端更新可 scp dist 免重编 relay | ✅ |
| ~~R4 通知审批~~ | Web Push | — | 产品决策：不做 |
| **R5 远程启停扩展**（远期） | 恢复历史会话、多桌面切换 | 桌面前端退化为 Rust 权威的另一客户端 | 远期 |

实现要点（代码位置）：

- 桌面 Rust：`remote_server.rs`（局域网 WS + Web UI 静态（web-assets crate：`CC_BOX_WEB_DIR` 磁盘优先、内嵌兜底）+ token 认证 + 动态启停）、`remote_state.rs`（唯一 handler：会话聚合、256KB TailBuffer 输出回放（快照先于增量入队、慢消费者断连重拉）、working/pending 状态机、active_view 独占、订阅广播、新建/关闭、事件分发三变体）、`remote_client.rs`（中继出站连接 + 手机连接桥接）、`rpc_dispatch.rs`（`registry!` 命令单一注册表 + StateSection 枚举 + 差集守卫）、`macros/`（`#[cc_command]` 出口自动广播）、`events.rs`（事件常量 + emit_all）
- 挂载点：`pty.rs` 输出 tee（emit 后 `push_output`）、spawn 注册、exit/kill 清理、`build_claude_args` 单一拼接核心；`hook_server.rs` 事件转发；`commands.rs` pty_resize 让位远程持有 + pty_claim 接管 + pty_kill 广播 remote-session-closed（与 session.kill 双路径对称）
- 前端：`src/api/transport.ts`（接口）+ `tauri-transport.ts` / `remote-transport.ts`（WS + rpc 桥 + 消息→事件映射）；`src/state/sections.ts`（状态同步中心）；`TokenGate.vue` 认证门；IconBar「远程」按钮 → `RemoteModal`（直连 QR + 中继配置/配对 QR）；`XTermTerminal` taken-over 遮罩 + 接管回来；`TerminalView` 监听 remote-session-created 自动建 tab
- 中继：`remote/relay`（透传 hub + /api/pair 配对 + device_key 绑定校验；静态资源经 web-assets crate（`CC_BOX_RELAY_WEB_DIR` 磁盘优先、内嵌兜底）；部署见 [docs/deploy.md](docs/deploy.md)）

R1–R3 未动既有前端会话管理（手机新建会话经事件桥接建 tab，而非会话权威下沉）；UI 统一化后浏览器端与桌面共用同一套前端会话管理。

## 11. 测试

遵循 [docs/测试编写原则.md](../docs/测试编写原则.md)，远程手动测试条目记 `remote/docs/manual-test-cases.md`。

- **契约测试（第一优先）**：protocol 的 JSON 样例由 Rust 与 Vitest 共跑（见第 4 节）
- 自动测试：消息序列化容错、`active_view` 状态机全路径、ring buffer 容量淘汰与换行对齐、relay 集成（内存起 server + 双 WS 客户端：配对→注册→路由→吊销）、令牌生命周期
- 手动测试：真机扫码/软键盘/PWA 加主屏、切后台 30s 归还与抢回、公网弱网断连恢复、桌面切 Tab reload 后远程不受影响

## 12. 遗留决策

| 问题 | 倾向 |
|---|---|
| 手机端只读角色粒度 | 预留 `role` 字段，按需求激活 |
| E2E 加密时间点 | 自托管场景暂缓，协议已预留 |
| 多桌面切换 UI | 协议天然支持（device_id 隔离），首版单设备 |
