# CC-Box 远程连接设计文档

> 状态：设计阶段，未开始实施
>
> 本目录承载远程连接相关的全部代码与文档：中继服务器、手机 Web UI、协议 spec、部署指南、手动测试条目。

## 1. 背景与目标

### 1.1 为什么做

- Claude Code 官方 Remote Control 仅支持 Pro/Max 订阅、**不支持 API key**——CC-Box 用户大量使用第三方 Provider（GLM 等），官方方案覆盖不到，这是本功能的核心价值定位
- 多会话重度用户的核心场景：任务挂着跑，人离开电脑。需要在外**查看进度、审批权限请求、继续对话**

### 1.2 目标

- 手机浏览器打开即用（PWA，可加主屏，无原生 App）
- **看到**：会话列表、working/pending 状态、终端内容
- **控制**：输入、审批（多会话远程启停为远期目标）
- **双模式网络**：局域网直连（低延迟）+ 自建中继（公网穿透，桌面零入站端口）
- **一次配对，长期有效，可吊销**

### 1.3 非目标

- 不做云执行——代码始终跑在用户自己电脑上，手机只是控制界面
- 不做端到端加密（首版，协议预留升级位）
- 不做多用户协作
- 不做原生手机 App

## 2. 参考方案

| 方案 | 架构 | 借鉴点 | 教训/局限 |
|---|---|---|---|
| **ZCode Remote** | 桌面出站长连接 → Z.ai 云中继（闭源）→ 手机 H5（扫码 + 账号） | 出站连接拓扑、QR 配对、Web 客户端 | 扫码凭证是临时的，网页关闭/过期必须重扫，用户抱怨最大的点 |
| **Claude Code Remote Control** | claude.ai 官方中继，本地只出不进 | 同上；扫码即用体验 | 仅 Pro/Max、不支持 API key |
| **Happy Coder** | 本地 CLI → 中继服务器 → 手机 App，E2E 加密 | 三层架构 | 依赖第三方云 |
| **SSH + Tailscale + tmux** | 自组网隧道 | 隧道思路、attach/detach 语义 | 配置门槛高，无配对/通知能力 |

## 3. 核心决策

| # | 决策 | 理由 |
|---|---|---|
| 1 | 手机端 = 移动浏览器 PWA | 零安装、双端（iOS/Android）覆盖、由服务器直接 serve |
| 2 | **桌面是权威（Host），中继是纯路由器** | 中继不解析应用层 payload；会话状态、PTY、权限判定都在桌面 Rust 侧 |
| 3 | 双模式网络：局域网直连 + 自建中继 | 在家直连低延迟，在外走中继；应用层协议完全同一套 |
| 4 | **控制权独占**：同一时间只有一端能看和操作终端 | 消除 resize 冲突；`active_view` 状态机，tmux attach/detach 语义 |
| 5 | 数据层双写：独占期间输出流仍同时写入两端 buffer | 桌面接管回来内容零缺失，手机断连重连无缝 |
| 6 | 状态监控不受独占限制 | 会话列表、working/pending 始终多端可见——"看进度"和"操作终端"分离 |
| 7 | 一次配对长期有效 + 可吊销 | 吸取 ZCode 临时凭证教训，做到"打开即连" |
| 8 | 中继自建（orczh.top），单二进制透传 hub | 几百行 Rust；换来配对管理、PWA 入口、**Web Push 通知**（唯一需要常驻 HTTPS 服务端的能力） |
| 9 | 远程功能默认关闭 | 安全默认，显式开启 |

## 4. 总体架构

### 4.1 局域网直连模式

```
手机浏览器 ── HTTP/WS ──► remote_server.rs（axum，桌面 Rust 侧）
                          ├── serve 手机 Web UI（内嵌静态资源）
                          ├── pty.rs：tee 输出 / write 注入 / resize
                          └── hook_server.rs：hook 事件转发
```

### 4.2 中继模式（公网）

```
手机浏览器（PWA）                              家里电脑（NAT 后）
   │                                            │
   │ HTTPS/WSS                       出站 WSS 长连接（桌面只出不进）
   ▼                                            ▼
┌─────────────────────────────────────────────────┐
│  VPS（orczh.top）                                 │
│  nginx（已有实例，新增子域 + 证书）                  │
│    └─ 反代 → cc-box-relay（127.0.0.1:8xxx）       │
│         ├─ WS hub：设备路由表（纯透传）             │
│         ├─ 配对/令牌管理（内存态）                  │
│         └─ 手机 Web UI 静态资源（rust-embed）      │
└─────────────────────────────────────────────────┘
```

两种模式并行运行：手机扫码时 QR 同时携带直连地址与中继地址，手机端先试直连、不通走中继，自动切换。

## 5. 目录规划

```
remote/
├── README.md                  # 本设计文档
├── relay/                     # cc-box-relay 中继服务器（独立 Rust crate，非 workspace 成员）
│   ├── Cargo.toml
│   └── src/main.rs
├── web/                       # 手机端 Web UI（独立 Vite 子应用，Vue 3 + xterm.js）
│   ├── package.json
│   ├── vite.config.ts
│   └── src/
└── docs/                      # 远程功能专属文档
    ├── protocol.md            # 消息协议 spec（实施时从本文档拆出维护）
    ├── deploy.md              # 中继部署指南（nginx/systemd/certbot）
    └── manual-test-cases.md   # 手动测试条目（真机、公网等自动化无法覆盖的场景）
```

桌面侧 Rust 模块（挂在 `src-tauri/src/`，不放入本目录，遵循现有架构）：

| 模块 | 职责 |
|---|---|
| `remote_server.rs` | 局域网直连服务器：WS + 静态资源 serve + 应用层消息处理 |
| `remote_client.rs` | 中继出站客户端：连接 relay、断线重连、把中继路由消息交给同一套应用层 handler |
| `remote_state.rs` | 应用层共享状态：会话聚合、`active_view` 状态机、ring buffer 索引 |
| `pty.rs`（改动） | 输出 tee、ring buffer 写入、`active_view` 字段 |
| `hook_server.rs`（改动） | hook 事件除 emit 前端外，同步 broadcast 到远程客户端 |

> 构建产物流动：`remote/web` 构建出的 `dist/` 分别被 `remote/relay`（rust-embed）和 `src-tauri`（局域网 serve）嵌入，两端 UI 永远同版本。

## 6. 组件设计

### 6.1 remote_server.rs（桌面，局域网直连）

- axum + `axum::extract::ws`，绑定 `0.0.0.0:<随机端口>`（开启远程时），与 hook_server 同栈
- 端点：`GET /`（Web UI 静态资源）、`WS /ws`（应用层协议）
- 认证：URL fragment 携带 token（`#token=...`，fragment 不会进入服务器日志）
- handler 与 remote_client 共用 `remote_state.rs` 的逻辑——**局域网直连和中继是同一条应用层代码的两个传输入口**

### 6.2 remote_client.rs（桌面，中继出站连接）

- 启动条件：配置中启用了中继（`~/.cc-box/config.json` 的 `remote.relay_url`）
- `device_key` 首次生成后持久化；连接 `wss://<relay>/ws/device`，头携带 `Authorization: Bearer <device_key>`
- 断线重连：指数退避 1s → 30s 封顶，网络恢复自动回来
- 收到手机的 `data` 消息 → 应用层 handler；响应与事件按 `to: <conn_id>` 原路回程

### 6.3 pty.rs 改动

**输出 tee**：PTY 读取循环中，`decode_output()` 解码完成后（此时已是 UTF-8 String），在 `emit("pty-output")` 之外同步：

1. 写入该 PTY 的 ring buffer（快照源）
2. 若 `active_view` 是某远程连接，推送到该连接的输出队列

**ring buffer**（快照源，不依赖桌面前端存活——桌面前端可能被切 Tab reload 或最小化）：

- 每 PTY 一个环形字节缓冲，默认 256KB（可配置）
- 快照发送时从最近的换行符边界开始，避免截断 ANSI 转义序列导致乱码

**active_view 字段**：`PtyManager` 中每个 PTY 增加 `active_view: Desktop | Remote(conn_id)`。

### 6.4 cc-box-relay（中继服务器）

单二进制，全内存态（重启即清，无数据库），systemd 托管。

端点：

```
GET  /              手机 Web UI（rust-embed）
WS   /ws/device     桌面端注册（Bearer device_key）
WS   /ws/client     手机端连接（Bearer access_token）
POST /api/pair      手机提交配对码换 access_token
POST /api/revoke    吊销手机令牌（桌面端发起）
```

状态：

- `devices: HashMap<DeviceId, WsSender>` —— 在线桌面路由表
- `pair_codes: HashMap<Code, (DeviceId, expires_at)>` —— 一次性配对码，TTL 5 分钟
- `access_tokens: HashMap<Token, DeviceId>` —— 校验（可选持久化 JSON）

职责边界：**只做注册、路由、配对、在线状态广播；不解析、不存储、不改写应用层 payload**。这条原则为未来端到端加密留路（届时中继只见密文）。

### 6.5 手机 Web UI（remote/web）

- Vue 3 + xterm.js（移动浏览器可用），与会话状态相关的轻量视图
- 页面结构：
  - **设备页**：配对入口（输码/扫码）、已绑定设备、在线状态
  - **会话列表页**：多端可见，working/pending 状态圆点（复用现有状态语义）
  - **终端页**：xterm.js 渲染 + 输入框（移动端软键盘适配）；attach 时接管，显示"已归还桌面"时禁用输入
- PWA：manifest + service worker（R4 加推送时引入，前期仅 manifest 加主屏）

## 7. 消息协议

### 7.1 中继层（relay 边界）

```json
桌面→中继  {"type":"register","device_id":"..."}          // 连接头带 device_key
中继→桌面  {"type":"registered","device_id":"..."}
中继→手机  {"type":"device_status","online":true}
手机→中继  {"type":"data","payload":{...}}                 // 路由到该手机绑定的桌面
桌面→中继  {"type":"data","to":"<conn_id>","payload":{...}} // 路由到指定手机连接
```

### 7.2 应用层（手机 ↔ 桌面，局域网与中继模式同构）

JSON 帧，`type` 区分：

| 方向 | type | payload | 说明 |
|---|---|---|---|
| 手机→桌面 | `hello` | `{ver, client}` | 版本协商 |
| 桌面→手机 | `hello_ok` | `{ver, device_name}` | |
| 手机→桌面 | `sessions.list` | `{}` | 会话列表（多端可见，不受独占限制） |
| 桌面→手机 | `sessions` | `{list:[{id,name,project,working,pending}]}` | |
| 手机→桌面 | `session.attach` | `{id}` | 请求接管该终端（独占，抢占式） |
| 桌面→手机 | `session.attached` | `{id, cols, rows, snapshot}` | 接管成功 + ring buffer 快照 |
| 桌面→手机 | `pty.output` | `{id, data}` | 增量输出流 |
| 手机→桌面 | `pty.input` | `{id, data}` | 输入注入（仅 active 端有效） |
| 手机→桌面 | `pty.resize` | `{id, cols, rows}` | 仅 active 端有效 |
| 手机→桌面 | `session.detach` | `{id}` | 主动归还 |
| 桌面→手机 | `session.detached` | `{id, reason}` | 被桌面接管/断连归还 |
| 桌面→手机 | `hook.event` | `{...}` | hook 事件（状态监控） |
| 双向 | `ping` / `pong` | `{}` | 心跳，30s 间隔 |

桌面内部 Tauri 事件：

| 事件 | 说明 |
|---|---|
| `emit("session-taken-over", {id})` | 前端对应 tab 显示"已在手机端接管"全遮罩（显示层独占，数据层继续写入 xterm buffer） |
| `emit("session-reclaimed", {id})` | 前端解除遮罩，接管回来内容零缺失 |

### 7.3 active_view 状态机（控制权独占）

产品决策：**同一时间只有一端能看到和控制某个终端**。

```
状态: Desktop | Remote(conn_id)

事件处理:
  手机 session.attach     Desktop ──► Remote(new)
                          ├─ PTY resize 为手机 cols/rows
                          └─ emit('session-taken-over') → 桌面遮罩

  手机 session.attach     Remote(old) ──► Remote(new)
                          └─ 旧连接收 session.detached（手机重连抢回）

  桌面点"接管回来"        Remote ──► Desktop
  (invoke pty_claim)      ├─ PTY resize 回桌面尺寸
                          └─ 手机收 session.detached

  手机 WS 断开            Remote ──► Desktop
  (切后台/锁屏/弱网)       ├─ 自动归还（持有权不赖在手机上）
                          └─ resize 回桌面尺寸

  PTY 退出                清理 ring buffer 与状态
```

规则：

1. **后 attach 优先**（抢占式，无需对端确认）——首版最简
2. 输入与 resize 只接受 active 端请求，其余直接忽略
3. 输出流例外：tee 双写（桌面遮罩挡视线，但 xterm buffer 持续写入）
4. 手机断连即归还桌面；手机重新打开时重新 attach 抢回
5. `sessions.list` 与 `hook.event` 不受独占限制

## 8. 配对与凭证

三级凭证：

| 凭证 | 持有方 | 生命周期 | 用途 |
|---|---|---|---|
| `device_key` | 桌面 ↔ 中继 | 长期，存 `~/.cc-box/config.json` | 桌面注册 |
| `pair_code` | 一次性 | TTL 5 分钟 | QR 配对 |
| `access_token` | 手机 ↔ 中继 | 长期，可吊销，手机 localStorage | 手机连接 |

流程：

```
1. 桌面设置页开启远程 → 生成 device_key → 连上中继 → 显示 QR
   QR = https://<relay>/#pair=<一次性配对码>
   （局域网场景 QR = http://<LAN-IP>:<port>/#token=<局域网 token>）
2. 手机扫码 → 打开页面 → 提交配对码 → 中继校验（设备在线 + 码有效）
   → 颁发 access_token（绑定该桌面）→ 手机持久化
3. 之后打开即连，无需再扫
4. 桌面设置页显示"已配对设备"列表，可随时吊销
```

权限：首版配对即读写单级；协议预留 `role: readonly` 字段。

## 9. 安全模型

- **传输**：中继模式全程 TLS（nginx + Let's Encrypt）；局域网模式明文 HTTP + token（同网段可接受）
- **桌面零入站端口**（中继模式）：出站 WSS 穿透 NAT，不暴露攻击面
- **远程功能默认关闭**，显式开启后才绑定端口/连接中继
- **中继不解析 payload**：明文仅对自托管中继可见；未来 E2E（桌面/手机配对时协商共享密钥）只改两端不动中继
- 令牌均为随机 32 字节（base64url）

## 10. 部署（orczh.top）

```bash
# 1. 上传二进制（SSH 仅作部署通道，不进数据面）
scp cc-box-relay user@orczh.top:~

# 2. systemd 服务，监听 127.0.0.1:8xxx

# 3. nginx 新增 server 块（复用现有实例，不影响其他服务）
server {
  server_name cc.orczh.top;
  location / {
    proxy_pass http://127.0.0.1:8xxx;
    proxy_http_version 1.1;
    proxy_set_header Upgrade $http_upgrade;
    proxy_set_header Connection "upgrade";
    proxy_read_timeout 3600s;    # WS 长连接
  }
}

# 4. certbot -d cc.orczh.top
```

开源发布：relay 二进制 + `docker-compose.yml` 一并提供，社区用户可自托管——与项目"可逆性、不依赖第三方云"原则一致。

## 11. 实施阶段

| 阶段 | 内容 | 交付判据 |
|---|---|---|
| **R1 局域网只读** | `remote_server.rs`（WS + 静态 serve）、hook 事件转发、输出 tee、Web UI 会话列表 + 输出 tail | 同 WiFi 手机能看到所有会话状态与输出 |
| **R2 局域网交互** | ring buffer 快照、xterm 渲染、输入注入、`active_view` 独占接管、桌面遮罩 | 手机完整操作终端，与桌面互抢无缝 |
| **R3 中继公网** | `cc-box-relay`、`remote_client.rs`、QR 配对、设备管理/吊销、断线重连、PWA manifest | 公网"打开即连"，桌面重启自动恢复 |
| **R4 通知与审批** | Web Push（中继持 VAPID 密钥）、任务完成/待审批推送、审批快捷操作（y/n 一键） | 锁屏收到推送，点开直达审批 |
| **R5 远程启停会话**（远期） | 会话权威从前端 sessionStore 下沉到 Rust，手机新建/恢复/关闭会话 | 桌面前端退化为 Rust 权威的另一客户端 |

R5 是架构演进的关键：对齐 ZCode "Host 权威"模型，桌面前端与手机端平等。R1–R4 不需要动现有前端会话管理。

## 12. 测试要求

遵循 [docs/测试编写原则.md](../docs/测试编写原则.md)，测试文件存放遵从现有约定（前端 `tests/`、Rust `src-tauri/src/tests/`）；**远程专属手动测试条目记录到 `remote/docs/manual-test-cases.md`**（不混入 docs/manual-test-cases.md）。

自动测试覆盖（自动优先）：

- 应用层消息序列化/反序列化、未知 type 容错
- `active_view` 状态机全路径（抢占、归还、断连归还、PTY 退出清理）
- ring buffer：容量淘汰、快照换行对齐
- relay 路由：内存起 server + 双 WS 客户端的集成测试（配对 → 注册 → 路由 → 吊销）
- token/配对码生命周期

手动测试条目（示例，实施时扩充）：

- 真机（iOS Safari / Android Chrome）扫码、软键盘、PWA 加主屏
- 手机切后台 30s 回来：自动归还桌面后重新抢回
- 公网弱网：断连重连、快照恢复无乱码
- 桌面切 Tab reload（WebGL 修复场景）后远程体验不受影响

## 13. 遗留决策点

| # | 问题 | 倾向 |
|---|---|---|
| 1 | 域名形态：独立子域 `cc.orczh.top` vs 现有域名路径 | 子域（证书干净，WS 反代互不干扰） |
| 2 | 协议定义是否抽共享 crate（relay 与 src-tauri 各自定义） | 首版各自定义（消息十余种），增多再抽 |
| 3 | 手机端只读角色粒度 | 预留 `role` 字段，按需求激活 |
| 4 | E2E 加密时间点 | 自托管场景暂缓，协议预留 |
| 5 | 多桌面切换 UI | 协议天然支持（device_id 隔离），首版单设备 |
