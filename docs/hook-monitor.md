# Hook 监控系统

通过 Claude Code Plugin 机制注入 Hook，实时采集 Claude 运行时状态，经 HTTP 发送到 CC-Box 后端，再推送到前端展示。

## 设计理念

Hook 系统只采集**对 GUI 有意义**的信息：

1. **session_id** — 关联手动创建的 Claude 会话
2. **模型** — 从 SessionStart 提取，保留供未来使用
3. **工作状态** — 用户发消息后进入 working，响应完成后退出 working

其他事件详情不在后端提取。原始 JSON 仍完整传输，架构保持可扩展。

## 架构总览

```
┌─ CC-Box 启动 ────────────────────────────────────────────────┐
│  1. 生成 plugin 文件到 ~/.cc-box/claude-plugin/               │
│  2. 启动 axum HTTP 服务器（127.0.0.1:随机端口）                │
│  3. 端口写入 OnceLock，供 PTY spawn 读取                      │
└──────────────────────────────────────────────────────────────┘

┌─ 用户启动会话 ───────────────────────────────────────────────┐
│  pty.rs spawn_claude()                                        │
│  ├─ 命令追加：--plugin-dir ~/.cc-box/claude-plugin            │
│  └─ 环境变量：                                                │
│       CC_BOX_HOOK_PORT=<端口>     ← 共享，一个 CC-Box 实例一个 │
│       CC_BOX_SESSION_ID=<pty_id>  ← 每个 PTY 唯一             │
└──────────────────────────────────────────────────────────────┘

┌─ Claude 运行中 ─────────────────────────────────────────────┐
│  Hook 触发 → report-hook.sh 执行                              │
│  ├─ 检查 CC_BOX_HOOK_PORT（未设置则 exit 0）                  │
│  ├─ 检查 curl 可用性                                          │
│  ├─ 读取 stdin（Claude 传入的 JSON）                          │
│  └─ curl POST → http://127.0.0.1:$CC_BOX_HOOK_PORT/hook      │
│         header: X-CC-Box-Session: $CC_BOX_SESSION_ID         │
│         body: <hook 事件 JSON>                                │
└──────────────────────────────────────────────────────────────┘

┌─ CC-Box Rust 后端 ──────────────────────────────────────────┐
│  hook_server.rs handle_hook()                                 │
│  ├─ 从 header 读 pty_id（区分多终端）                         │
│  ├─ 从 body 读 session_id                                     │
│  ├─ 建立 session_id ↔ pty_id 映射                             │
│  ├─ hook_events.rs：推导状态 + 提取模型                        │
│  ├─ emit "hook-event" → 前端（原始事件，诊断用）              │
│  └─ remote_state.on_hook_payload → apply_hook_event           │
│       ├─ SessionStatus 广播（手机列表页圆点）                  │
│       └─ emit "hook-status" {ptyId, working, pending,         │
│            sessionId, trigger} → 桌面/手机前端                 │
└──────────────────────────────────────────────────────────────┘

┌─ Vue 前端（桌面/手机同一份代码） ─────────────────────────────┐
│  composables/useStatusMonitor.ts — hook-status 消费者         │
│  ├─ 应用 Rust 状态机算好的 working/pending（不复制状态逻辑）  │
│  ├─ 新 pending 的 UI 决策：正看清除 / 失焦任务栏跳动          │
│  ├─ trigger=sessionStart → 分配 session_id                   │
│  ├─ trigger=userPromptSubmit → 首条 prompt 自动命名           │
│  ├─ watch 聚焦+Tab 切换 → 清除 active tab 的 pending          │
│  └─ session-viewed → 某端 attach 打开会话即"已读"，           │
│       服务端清 pending 并双端广播，各端 Tab 同步去金黄        │
│                                                               │
│  SessionItem.vue ← tab.working → 指示灯样式                   │
└──────────────────────────────────────────────────────────────┘
```

## 多终端区分

| 标识 | 来源 | 用途 |
|------|------|------|
| `CC_BOX_SESSION_ID`（= PTY ID） | spawn 时注入环境变量 | 标识**哪个终端 tab** |
| `session_id` | Claude hook 事件 JSON | 标识 **Claude 内部会话** |
| `CC_BOX_HOOK_PORT` | CC-Box 启动时分配 | **HTTP 服务器端口**（所有终端共享） |

## Plugin 文件

源文件位于 `src-tauri/plugin/`，编译时通过 `include_str!()` 嵌入二进制，运行时写入 `~/.cc-box/claude-plugin/`。

```
src-tauri/plugin/                       运行时目标 ~/.cc-box/claude-plugin/
├── .claude-plugin/                     ├── .claude-plugin/
│   └── plugin.json                     │   └── plugin.json
├── hooks/                              ├── hooks/
│   └── hooks.json     ← 11 个事件定义   │   └── hooks.json
└── scripts/                            └── scripts/
    └── report-hook.sh                     └── report-hook.sh
```

**加载方式**：`--plugin-dir` 按 session 加载，仅在 CC-Box 启动的 Claude 会话中生效。
**更新方式**：修改 `src-tauri/plugin/` 下的文件，重新编译。`hook_config.rs` 的 `write_if_changed` 检测变化后覆盖。

## 采集的事件与数据

所有事件提取定义集中在 `src-tauri/src/hook_events.rs`，前端类型在 `src/types/hook.ts`，一一对应。

### SessionStart 事件

SessionStart 是最重要的 hook 事件，包含完整的会话元数据：

```json
{
  "session_id": "abc123",
  "transcript_path": "/Users/.../.claude/projects/.../abc123.jsonl",
  "cwd": "/Users/.../project",
  "hook_event_name": "SessionStart",
  "source": "startup",
  "model": "claude-sonnet-4-6"
}
```

**提取字段**：
| 字段 | 用途 |
|------|------|
| `session_id` | payload 顶层字段，用于给新建 Tab 分配 sessionId |
| `cwd` | 真实项目路径 |
| `transcript_path` | JSONL 文件路径，可解析出 Claude 项目目录 |
| `model` | 模型标识符 |
| `source` | 启动来源：startup/resume/clear/compact |

**前端处理**（useStatusMonitor.ts）：
```typescript
if (payload.detail.type === 'sessionStart') {
  const sessionId = payload.sessionId  // 从 payload 顶层获取
  if (sessionId) {
    const data = payload.detail.data    // SessionStartData
    sessionStore.assignSessionIdByPtyId(ptyId, sessionId, data.model)
  }
}
```

**后端处理**（hook_server.rs）：
- 收到 SessionStart 时自动 `invalidate_project_path_mapping()`
- 建立 `session_id ↔ pty_id` 映射（用于反向查询）

### 已注册的 11 个事件

| 事件 | 提取数据 | 推导状态 |
|------|----------|----------|
| `SessionStart` | session_id(顶层), cwd, transcript_path, model, source | → idle |
| `SessionEnd` | — | → idle |
| `UserPromptSubmit` | — | → thinking |
| `PreToolUse` | — | → tool_executing |
| `PostToolUse` | — | → thinking |
| `PostToolUseFailure` | — | → thinking |
| `Stop` | — | → idle |
| `StopFailure` | — | → error |
| `Notification` | — | → waiting_permission / waiting_input |
| `SubagentStart` | — | → subagent_running |
| `SubagentStop` | — | → thinking |

### 状态机

状态由 `derive_state()` 从事件类型无状态推导（不依赖前序状态），对丢事件具有容错性。

```
idle ──[UserPromptSubmit]──→ thinking
thinking ──[PreToolUse]──→ tool_executing ──[PostToolUse]──→ thinking
thinking ──[Notification:permission_prompt]──→ waiting_permission
thinking ──[Notification:idle_prompt]──→ waiting_input
thinking ──[Stop]──→ idle
any ──[SubagentStart]──→ subagent_running ──[SubagentStop]──→ thinking
any ──[StopFailure]──→ error
```

### 状态指示灯 UI

**判断优先级**（SessionItem.vue 的 dotClass）：

```
1. stopped + 非激活 → hollow ring（空心圆，已关闭）
2. stopped + 激活 → gray dot（灰色，已停止但当前选中）
3. working → green pulsing（绿色脉冲，正在工作）
4. pending + 非激活 → gold pulsing（金色脉冲，需要关注）
5. running → green solid（绿色静止，空闲运行）
6. 默认 → gray dot（灰色，无状态数据）
```

| 状态 | 颜色 | CSS 变量 | 动画 | 含义 |
|------|------|----------|------|------|
| `working` | 墨蓝 | `--status-info` | 温和脉冲 | Claude 正在处理 |
| `pending`（非激活 Tab） | 琥珀金 | `--accent-gold` | 温和脉冲 | 等待用户关注 |
| `running`（idle） | 墨绿 | `--status-success` | 无动画 | 空闲运行态 |
| `stopped`（激活） | 浅灰 | `--text-tertiary` | 无 | 已停止 |
| `stopped`（非激活） | 空心灰 | `--border-color` | 无 | 已关闭 |

**pending 的"打开过即已读"语义**：`pending` 表示"等待用户关注"，一旦任一端**打开**（attach）该会话即视为已读——`remote_state.handle_attach` 在锁窗口内把 `SessionEntry.pending` 清 false 并双端广播 `session-viewed`（`{id, data:""}`，发起端排除），各端 `useStatusMonitor` 收到后清 Tab 金黄。服务端同步清除是关键：浏览器端会话面板打开时 `sessions.list` 会用服务端 `working/pending` 覆盖本地 Tab，若服务端不清除，"手机看过又切走"会被列表刷新冲回金黄。`working`（任务实时状态）不代表"待查看"，attach 不清。

### 首页 Recent Sessions 状态展示

`ProjectSelectView.vue` 的 Recent Sessions 列表同样展示 working/pending 状态：

- 从 `sessionStore.tabs` 读取运行中 session 的 `working`/`pending` 字段
- 通过 `sessionDotClass()` 返回 `working`/`pending`/`running`
- 样式与 SessionItem 一致（绿色脉冲、金色脉冲、绿色静止）

**IconBar session 角标**：仅显示当前项目内非激活 tab 的 pending 数量，通过 `sessionStore.getProjectTabs(cwd)` 过滤。

### 新增/修改事件的步骤

1. 在 `src-tauri/plugin/hooks/hooks.json` 注册新事件
2. 在 `src-tauri/src/hook_events.rs` 的 `extract_detail` + `derive_state` 添加分支
3. 在 `src/types/hook.ts` 添加对应类型
4. 如需消费新事件，在对应 composable 中调用 `hookStore.subscribe([type], handler)`

## 前端数据结构

### HookEventPayload（从 Rust 后端 emit）

```typescript
interface HookEventPayload {
  ptyId: string | null
  sessionId: string | null
  eventName: string
  state: ClaudeState
  timestamp: number
  detail: HookEventDetail
}
```

### TerminalTab 状态字段（session.ts）

```typescript
interface TerminalTab {
  // ...
  working: boolean    // 正在工作中（用户发消息后、响应返回前）
  pending: boolean    // 需要用户关注（响应完成但用户未看到）
  model?: string      // 模型名（从 SessionStart 提取）
  // ...
}
```

## 稳定性保障

**原则：Hook 是监控通道，不是核心功能。异常不能影响 CC-Box 和 Claude 正常运行。**

| 层级 | 保障措施 |
|------|----------|
| **脚本** | 始终 `exit 0`；curl 不可用时跳过；`--max-time 3` 限超时；hook `timeout: 5` |
| **HTTP 服务器** | 启动失败仅日志告警；handler 内 catch 所有错误；独立 tokio task |
| **PTY** | `--plugin-dir` 路径不存在时 Claude 忽略正常启动 |
| **前端** | 无 hook 事件时 `working = false`，指示灯降级为默认样式 |

## 前端状态消费（hook-status）

状态机语义**单点在 Rust** `remote_state.rs apply_hook_event`（回合保护、recap 防护、
permission 恢复等，见其测试），前端不再复刻（原 `stores/hook.ts` 事件总线已删除——
唯一消费者 useStatusMonitor 改为直连事件后无存在必要）。

`hook-status` 事件由 `hook_server.rs` 在每个 hook 事件处理后 emit_all 双端下发：

```typescript
// useStatusMonitor.ts —— 桌面/手机同一份代码，消费同一事件
onHookStatus((status) => {
  // status: { ptyId, working, pending, sessionId, trigger }
  // working/pending 直接应用（Rust 已算好）；trigger 供 UI 决策
})
```

### 状态监控模块 (useStatusMonitor)

挂载在 `TerminalView.vue`，消费 `hook-status` 事件维护 Tab 状态：

#### working/pending 状态

直接应用 Rust 状态机算好的最终值（`tab.working = status.working` 等）。
状态语义参考 Rust 侧 `apply_hook_event`：

| 事件（trigger） | Rust 状态机操作 |
|------|------|
| `userPromptSubmit` | `working = true`，开启新回合 |
| `stop` / `stopFailure` / `notification(idle_prompt)` | `working = false` + `pending = true`，回合结束 |
| `notification(permission_prompt)` | `working = false` + `pending = true`，回合不结束 |
| activity（tool/subagent/compact） | 回合内恢复 `working`；回合结束后忽略（recap 防护） |
| `sessionEnd` | `working = false`，不设 pending、不结束回合 |

#### pending 状态（用户注意力管理）

**设置 pending 的时机**：stop/stopFailure/notification 事件到达，且 working=true 时：

```
事件到达（stop/stopFailure/notification）
  ├─ tab.working = false
  ├─ tab.pending = true（默认设置）
  ├─ 检查三条件：聚焦 + 终端可见 + tab激活
  │   ├─ 全满足 → tab.pending = false（用户正在看，不需要提醒）
  │   └─ 不满足 → 保持 pending
  └─ 应用失焦时 → 触发任务栏跳动（requestUserAttention）
```

**清除 pending 的时机**：watcher 监听三条件变化：

```
watch([isFocused, isTerminalVisible, activeTabId])
  ├─ 三条件全满足 → 清除当前 activeTab 的 pending
  └─ 其他情况 → 不清除（保留其他 tab 的 pending）
```

**设计要点**：
- 用户在首页时（`isTerminalVisible=false`），所有完成的对话都设 pending
- 用户在终端但窗口失焦时，完成的对话设 pending + 任务栏跳动
- 用户切换 tab 后，旧 tab 的 pending 保留（切换回来能看到需要关注的 session）
- 只有"用户真正看到"（三条件全满足）才清除 pending

**设计理念**：pending 代表"需要用户注意力"，只有用户真正看到才消失，确保不错过任何需要关注的对话完成。

## 文件清单

| 文件 | 职责 |
|------|------|
| `src-tauri/src/hook_events.rs` | 事件数据结构、提取逻辑、状态推导 |
| `src-tauri/src/hook_server.rs` | HTTP 服务器：接收事件、路由、emit |
| `src-tauri/src/hook_config.rs` | Plugin 文件管理 |
| `src-tauri/plugin/` | Plugin 源文件（plugin.json + hooks.json + report-hook.sh） |
| `src/types/hook.ts` | 前端类型定义（含 HookStatusPayload） |
| `src/types/session.ts` | SessionInfo 类型（含 working/pending 字段） |
| `src/composables/useStatusMonitor.ts` | 状态消费：hook-status 事件 → Tab working/pending + UI 决策 |
| `src/composables/useWindowAttention.ts` | 窗口聚焦状态 + 取消任务栏跳动 |
| `src/components/sessions/SessionItem.vue` | Tab 状态指示灯 UI |
| `src/components/ProjectSelectView.vue` | 首页 Recent Sessions 状态展示 |
| `src/components/IconBar.vue` | Session 角标（按项目过滤 pending） |
