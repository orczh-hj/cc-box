# 多实例远程控制：设备级后台 Broker

> 面向开发者与测试同学。协议正本见 [protocol.md](protocol.md)「设备级 Broker IPC」；
> 手动测试矩阵见 [manual-test-cases.md](manual-test-cases.md)「U-18」。

## 1. 一句话概括

一台电脑 = **一个后台 Broker** = 一个设备身份、一个 LAN 入口、一条 relay 连接。
这台电脑上的每个 CC-Box GUI 都是 Broker 的**成员**：PTY 留在各自进程，Broker 只做注册、路由、认证与状态聚合。手机扫任意实例的码，看到的都是整台设备；会话按实例分组，可控制所有实例。

---

## 2. 用户逻辑与操作流程

> 使用与支持视角：谁在哪里做什么、操作后果是什么。实现与协议细节见后续章节；托盘菜单细节见第 9 节。

### 2.1 概念模型（三层角色）

| 角色 | 是什么 | 用户需要知道的 |
|---|---|---|
| 设备（后台服务） | 一台电脑的常驻 CC-Box（Broker），窗口全关也在线 | 持有唯一的远程身份：配对码、密钥、局域入口（8747）、中继连接 |
| 窗口（GUI） | 工作区，可同时开多个 | 会话属于各自窗口；关窗口只丢该窗口的会话；窗口名 = 项目名或 `CC-Box` |
| 手机 / 浏览器 | 远程操作端 | 看到与操作的是**整台设备**（所有窗口的会话），按窗口分组 |

### 2.2 桌面用户流程

1. **启动即常驻**：打开任一 GUI → 自动确保后台服务在线（托盘出现 `CC-Box`），用户无需感知 Broker 的存在
2. **多窗口**：托盘「新建窗口」→ 新窗口停在项目页，各窗口独立选项目、独立开会话；窗口标题实时同步到托盘菜单与手机分组名
3. **远程**：任一窗口打开远程面板——所有窗口的面板内容完全一致（同一份二维码与状态），且都是**设备级**操作；打开面板只读状态，开启/停止由用户显式操作
4. **开机自启动**：设置 → 启动（或托盘勾选）→ 开机只启动后台服务，不自动开窗口
5. **退出语义**：窗口红叉 = 只关该工作区，后台服务与其余窗口不受影响；托盘「退出 CC-Box」（确认框）或 `cc-box --quit-broker` = 关全部窗口并退出后台；停止远程 ≠ 退出（后台与窗口保留）

### 2.3 手机 / 浏览器用户流程

1. **配对**：扫桌面面板的中继码（跨网络）或局域码（同一网络直连）；可保存多台电脑，TokenGate 设备列表点击切换
2. **会话列表**：按窗口分组，分组名 = 窗口真实标题；圆点实时反映各会话工作状态
3. **进入会话**：点开即 attach 抢占该会话（per-session 控制权）；不同设备可同时操作不同会话；同会话后 attach 优先，旧操作端收遮罩可抢回；操作端断连自动归还
4. **新建/恢复会话**：设备上有 ≥2 个窗口时先弹「选择实例」；只有 1 个窗口不弹直接创建；取消则不创建
5. **零窗口**：桌面窗口全关时，手机项目页出现「启动窗口」→ 桌面拉起空窗口（停在项目页）；停留在终端页时同样显示「启动窗口」入口
6. **状态同步**：任一窗口或手机修改 provider/配置/开关，其他所有端 UI 自动刷新，无需手动操作

### 2.4 操作入口与后果速查

| 用户动作 | 入口 | 后果 |
|---|---|---|
| 新建窗口 | 托盘 | 新 GUI 停在项目页，不自动进终端 |
| 显示/聚焦窗口 | 托盘窗口一级菜单项 | 还原并置顶（无确认） |
| 开启/停止远程 | 任一窗口远程面板 / 托盘 | 设备级：局域 + 中继同开同停；后台与窗口不受影响（托盘入口有确认） |
| 更新连接码 | 远程面板 | 旧一次性配对码失效，新码原位刷新 |
| 刷新密钥 | 远程面板（两段确认）/ 托盘（原生确认） | 所有已配对设备断开，须重新扫码 |
| 开机自启动 | 设置 → 启动 / 托盘勾选 | 仅后台服务随系统启动，不开窗口 |
| 退出 CC-Box | 托盘（原生确认）/ `--quit-broker` | 关全部窗口 + 后台退出；手动重开 GUI 恢复 |
| 启动窗口（零窗口时） | 手机项目页 / 终端页覆盖层 | 桌面拉起空窗口并自动注册 |
| 接管会话（终端遮罩「接管回来」） | 终端 Tab 遮罩按钮 | 重新 attach/claim 抢占该会话；原操作端收遮罩 |

> 生命周期状态（`user_quit` / `update_in_progress` 等）与操作语义的正本见第 4 节；会话控制权规则正本见 [protocol.md](protocol.md)「会话控制权（per-session active）」。

## 3. 进程模型

```
cc-box                 → GUI 模式（创建一个窗口，PTY/hook 留在本进程）
cc-box --broker        → Broker 模式（无 GUI 窗口）
cc-box --broker --autostart → 登录项启动（忽略上次 user_quit）
cc-box --quit-broker   → 通过 IPC 退出 Broker（会关闭所有窗口）
  Windows/macOS        → Tauri 托盘/菜单栏 + 后台服务线程
  Linux                → 纯 tokio 无头进程（当前构建不链接 libappindicator）
```

- **按需启动 + 常驻**：每个 GUI 启动时确保 Broker 在线；**所有窗口关闭后 Broker 不退出**（托盘与远程入口保留）。
- **单实例**：`~/.cc-box/broker.lock` 使用 OS 排他锁（Windows `LockFileEx` / Unix `flock`），进程崩溃自动释放；并发启动的第二个 Broker 立即退出。
- **设备所有权**：活动 Broker 同时持有 `~/.cc-box/gateway.lock` 并每 2s 续心跳，旧版 GUI 探测到 `/gateway.ok` 后注册为成员，不再重新选举。
- **窗口管理**：Broker 记录每个成员的 PID 与 GUI 窗口标题；托盘可新建/显示/重启/关闭窗口，退出 CC-Box 时先关闭所有可控制窗口再退出。
- **新建窗口**：以 `--new-window` 启动，停留在项目列表，不自动进入终端。
- **开机自启动**：只启动 Broker（不开窗口）；Windows Run / macOS LaunchAgent / Linux XDG autostart。入口在**设置 → 启动**与托盘勾选项，不在远程面板。
- **崩溃恢复**：GUI 检测 IPC 断开 → 重新拉起 Broker → 全量 `MemberHello.sessions` 快照重建路由；不终止任何 GUI 的 PTY。

---

## 4. 生命周期状态

`~/.cc-box/broker.state.json`（原子写，仅当前用户可读）：

```json
{
  "generation": 7,
  "state": "running | user_quit | update_in_progress",
  "appVersion": "0.16.10",
  "fromVersion": "0.16.9",
  "pid": 12345,
  "ts": 1760000000000
}
```

| 状态 | 语义 | GUI 自动拉起 |
|---|---|---|
| `running` | 后台服务在线（远程可开可关），窗口全关也保持 | 是 |
| `user_quit` | 用户从托盘「退出 CC-Box」，持久抑制 | 否；手动打开 GUI 或 `--autostart` 清除 |
| `update_in_progress` | 安装前退出，释放可执行文件 | 同版本且 5 分钟内抑制；版本变化/超时后清除 |

> 旧版本留下的 `idle_exit` 标记仍可读取（视为允许拉起），但新版 Broker 不再写它。

**`remote_disabled` 不是 Broker 生命周期状态**：它是配置 `remote.enabled=false`，Broker 继续运行（托盘可见、可再开启），只是 LAN/relay 停止。

操作语义严格区分：

| 操作 | 结果 |
|---|---|
| 关闭单个窗口 | 该成员注销并结束其 PTY；其他窗口/PTY 不受影响 |
| 关闭最后一个窗口 | Broker 与远程入口保持运行；手机仍可连接并「启动窗口」 |
| 停止远程服务（面板/托盘） | 配置 `enabled=false`；Broker 与成员连接保留；可再开启 |
| 退出 CC-Box（托盘/`--quit-broker`） | 写 `user_quit`；先关闭所有可控制窗口（旧版窗口会提示无法关闭）；广播 `BrokerExit{Quit}`；退出 |
| 安装更新 | 写 `update_in_progress`；关闭除发起者外的窗口；`BrokerExit{Update}`；退出；新版本 GUI 按版本判断 |

---

## 5. IPC

| 平台 | 传输 | 权限 |
|---|---|---|
| Windows | Named Pipe `\\.\pipe\cc-box-broker-<user-hash>` | 显式 DACL：当前用户 + SYSTEM + Administrators；`reject_remote_clients` |
| Unix | `~/.cc-box/run/broker.sock` | 目录 `0700`、socket `0600`、`peer_cred` UID 校验 |

- 帧格式：4 字节大端长度 + UTF-8 JSON，上限 16 MiB。
- 首帧：
  - `member.hello`（v2）：`instanceId` + `broker.secret` + label + `pid` + `windowTitle` + 会话快照；
  - 成员上行 `window.title { title }`：项目切换后更新托盘窗口名；Broker 不广播给手机。
  - `{"type":"admin", ...}`：一次管理操作（GetStatus / SetEnabled / RestartRemote / RefreshPairCode / RotateSecret / SetRelayUrl / SetAutostart / Quit / PrepareForUpdate{keepInstanceId?}），执行后关闭。
- v2 Broker → 成员 `window.command { action }`：`close`（结束 PTY 退出，5s 超时按 pid 强杀）、`show`（还原/置顶/聚焦）、`openRemote`（显示并打开远程面板）。
- 远程零窗口：手机通过既有 `rpc.call "spawn_new_instance"` 让 Broker 拉起空窗口（Broker 本地执行，无需成员在线；上限 12 窗口 + 2s 冷却）。
- 认证失败/版本不匹配/重复 instanceId → `Reject` + 断开；首帧 10s 超时。
- `broker.secret` 与旧版 `gateway.secret` 分离：前者用于新 IPC，后者仅用于旧版 `/internal/gw-ws` 成员。

---

## 6. 消息顺序与可靠性

**每个方向每个成员/连接只有一条有序队列**，不依赖跨队列排序。

| 类别 | 消息 | 策略 |
|---|---|---|
| 可靠控制面/输入 | ConnDrop、Shutdown/BrokerExit、SessionUpsert/Remove、SessionCreated/Attached/...、RpcResult、Heartbeat、PtyInput、PtyResize、SessionCreate/Kill/Attach、RPC | 异步路径 `send().await`；5s 超时断开成员并触发重注册；同步广播 `try_send` 满则断开成员。**绝不静默丢输入** |
| 可丢数据面 | `PtyOutput` | 队列保留 25% headroom；超限丢弃，但每条带 `seq`，客户端检测缺口并显示「输出中断」 |

- `SessionAttached.tail_seq` / `SessionSubscribed.tail_seq` 是快照基线；客户端丢弃 `seq <= tailSeq` 的重叠帧。
- `ClientMsg` 不再携带 `holder`（全局控制权已 per-session 化）；旧 Broker 帧中的该字段被成员忽略。
- 成员本地写命令的 `state-changed` 经 `state.changed` 上行；Broker 重分配设备级 revision 后同时扇出给所有 v2 成员与手机，保证窗口 A 的 provider/config 修改能刷新窗口 B/C。
- `PtyInput/PtyResize` 只校验该会话 active 归属、不产生控制面副作用；必须已有 attach。
- PTY 退出由成员上行 `session.remove`，Broker 统一广播一次 `session.exit`；成员不再本地重复广播给桥接手机。
- `tailSeq` 基线在 claim/重 attach 时只抬高不降低，避免把已收到的实时序号误判为缺口。

---

## 7. 路由

- 注册表：`pty_id -> instance_id`；成员注册时用会话快照重建。
- `SessionCreated{id}` 到达 Broker 时**先登记路由**（最小会话条目）再投递给手机：手机收到成功即可立即 attach，消除竞态。随后 `SessionUpsert` 覆盖最小条目。
- 未知 `pty_id`：按消息类型回 `SessionExit` / `SessionAttachDenied` / `RpcResult error`，并推送 `Sessions` 快照；**绝不在 Broker 本地执行**（Broker 没有 PTY）。
- `SessionCreate` 不带 `instanceId`（旧手机）：落到最近注册的默认成员；无成员时显式 `SessionCreateFailed`。
- 成员断连清理：路由、会话缓存、上传粘连、待处理 RPC、待处理创建全部回收；向手机发 `SessionDetached{instance disconnected}`；待处理创建/RPC 明确回失败。
- 成员重连：成员 `reset_remote_views()` 把旧的远端 active 会话归还桌面，避免残留在旧 conn。
- per-session 控制权：成员侧 `handle_client_message` 对 `PtyInput/PtyResize` 只校验该会话 active；Attach/Create/Kill/RpcCall 直通转发（无控制权门）。

---

## 8. 旧版兼容矩阵

| 组合 | 行为 |
|---|---|
| 新 Broker + 新 GUI | IPC v2，全部功能（含窗口重启/关闭、开机自启动、远程启动窗口） |
| 新 Broker + 旧 GUI | 旧 GUI 经 `/internal/gw-ws` v1 注册；Broker 只发 v1 消息；旧 GUI 收不到 `DesktopEvent`/`WindowCommand`，也没有 pid → 托盘只展示、不能关闭/重启该窗口。**控制权混部退化**：旧 GUI 成员的 PtyInput 仍含全局 controller 双检查（本地残留），多设备各用不同会话时可能出现先到者输入被拒——双端升级后消除 |
| 新 Web UI（web 目录独立部署）+ 旧桌面 | 旧桌面仍按全局控制权 gate 写命令/attach（新 UI 无全局遮罩，被拒时 rpc 错误 / attach.denied 原样显示）；新 UI 发的 controller.takeover 旧桌面照常处理——行为等价旧 UI 的接管流，瞬态混部可接受。前端与 relay 部署解耦（见 deploy.md），前端更新通常先/随桌面一起上线 |
| 旧 Gateway + 新 GUI | Broker 进入 `legacy_owned`：不启动 LAN、不连 relay、不抢 `gateway.lock`；面板显示「旧版本实例占用远程入口」；旧版退出后自动接管 |
| 旧 GUI 在 Broker 运行时启动 | 旧 GUI 探到 `/gateway.ok` → 作为 v1 成员加入 |
| 第三方程序占 8747 | Broker 记录 `lanError`，**仍连接 relay**；面板提示局域网不可用 |
| 第三方占口 + 旧 GUI | Broker 无 LAN 入口，旧 GUI 无法加入（新 GUI 仍可经 IPC/relay 工作） |

---

## 9. 托盘

Windows 通知区域 / macOS 菜单栏（Tauri 原生托盘）；Linux 当前构建为无头降级（不链接 libappindicator，避免无托盘宿主时 Broker 无法启动）。

托盘标题为 **CC-Box**（Broker 作为 CC-Box 管理器）。菜单由真实状态驱动（2s 刷新）：

```
CC-Box
更新到 vX.Y.Z           ← 仅有新版本时出现（托盘图标叠加红点）
---
<GUI 窗口标题 1>        ← 单击直接显示/聚焦
<GUI 窗口标题 2>
（暂无窗口）
---
开机自启动（勾选）
新建窗口
远程连接
刷新远程密钥
关闭远程连接 / 开启远程连接
---
退出 CC-Box
```

- **窗口是一级菜单项**：点击窗口标题直接还原/置顶/聚焦该窗口；同名追加序号。托盘不再提供单窗口关闭/重启（这些操作已移除）。
- **窗口名**使用 GUI 真实窗口标题（打开项目时为项目名；项目页/未打开项目时为 `CC-Box`）；手机分组/实例选择器也使用同一展示名；旧版窗口显示为禁用项。
- 「远程连接」显示第一个可控制窗口并打开远程面板；没有窗口时先拉起一个新窗口。
- **更新提示**：Broker 常驻期间每 2 小时检查更新（启动后先等 30s，失败保留上次结果）；有新版本时菜单标题下出现「更新到 vX.Y.Z」且托盘图标右上角叠加红点（macOS 红点期间关闭模板图标渲染）。点击＝显示第一个可控制窗口并跳转**设置 → 更新**（无窗口先拉起一个）；下载/安装/失败恢复仍由 GUI 的更新页完成，Broker 不重复实现。
- **二次确认（原生对话框）**：关闭远程连接、刷新远程密钥、退出 CC-Box 都会先弹出确认；窗口单击显示/新建/开启远程连接/开机自启动/更新不确认。
- 「刷新远程密钥」会断开所有已配对设备，需重新扫码。
- 「关闭远程连接」只停止 LAN/relay，后台 Broker 与窗口继续运行。
- 「退出 CC-Box」先向所有可控制窗口发送关闭指令并等待，超时强杀，再退出 Broker；旧版窗口无法关闭，确认框会明确提示。
- 窗口全关后托盘与远程入口保持；“新建窗口”和远程“启动窗口”都只打开空窗口并停留在项目页；手机停留在终端页时，即使还有 stopped Tab 也会显示“启动窗口”覆盖层。
- 托盘创建失败只记日志，不阻断 Broker；Linux 无托盘时通过桌面设置/远程面板与 `--quit-broker` 管理。

---

## 10. 已知限制

1. **Linux 托盘**：当前构建未引入 `ksni`，Linux Broker 以无头方式运行；窗口/远程/开机自启动由桌面设置或远程面板管理，无窗口时可用 `cc-box --quit-broker` 退出。窗口全关后 Broker 仍保持。
2. **旧 GUI 不接收 Broker 桌面事件/窗口指令**：v1 协议无 `DesktopEvent`/`WindowCommand`，旧窗口只能展示、不能从托盘显示/关闭/重启。
3. **旧版占口期间新 GUI 的会话不在旧设备入口内**：legacy-owned 是明确的降级状态，接管后恢复。
4. **更新期间不自动拉起 Broker**：`update_in_progress` 由 `prepare_for_update` 写入；失败/取消后 5 分钟或新版本启动自动解除。更新会关闭除发起窗口外的其他窗口。
5. **窗口名**：托盘与手机分组都使用 GUI 真实窗口标题（项目名/`CC-Box`），同名追加序号；`instanceId` 仍是分组主键。
6. **重启窗口会丢失该窗口的会话**：PTY 生命周期与 GUI 进程绑定，这是已确认的产品语义。
7. **旧窗口无法强杀**：v1/无 pid 的窗口没有可靠 PID，退出 CC-Box 时不会被强杀。

---

## 11. 相关测试

- Rust：`src-tauri/src/tests/gateway.rs`（路由/窗口名单/关闭指令/断连清理）、`broker_ipc.rs`（IPC 注册/管理/认证/pid/清理）、`broker_lifecycle.rs`（生命周期 + 单实例锁）、`remote_host.rs`（legacy/端口占用）、`broker/autostart.rs`（三平台命令生成）、`broker/tray.rs`（窗口菜单模型）、`remote_state.rs`（per-session 控制权规则矩阵）。
- 前端：`tests/components/remoteModal.test.ts`（统一 Broker 状态/legacy 警告）、`tests/components/startupSection.test.ts`（开机自启动开关）、`tests/utils/remoteWindows.test.ts`（零窗口启动入口）、`tests/stores/session.test.ts`（空名单已知状态）、`tests/api/remote-transport.test.ts`（seq 缺口）。
- 手动：`remote/docs/manual-test-cases.md` U-18。
