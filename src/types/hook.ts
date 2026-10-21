/** Hook 监控系统类型定义 — 与 Rust hook_events.rs 一一对应 */

/** Claude 运行时状态 */
export type ClaudeState =
  | 'idle'
  | 'thinking'
  | 'tool_executing'
  | 'waiting_permission'
  | 'subagent_running'
  | 'compacting'
  | 'error'
  | 'unknown'

// ---- 事件数据结构 ----

/** SessionStart 事件提取的数据 */
export interface SessionStartData {
  model?: string
  cwd?: string
  transcriptPath?: string
  source?: string
}

/** SessionEnd 事件提取的数据 */
export interface SessionEndData {
  reason?: string
}

/** UserPromptSubmit 事件提取的数据 */
export interface UserPromptSubmitData {
  prompt?: string
}

/** PreToolUse 事件提取的数据 */
export interface PreToolUseData {
  toolName?: string
  toolUseId?: string
}

/** PostToolUse 事件提取的数据 */
export interface PostToolUseData {
  toolName?: string
  toolUseId?: string
}

/** PostToolUseFailure 事件提取的数据 */
export interface PostToolUseFailureData {
  toolName?: string
  toolUseId?: string
  error?: string
  isInterrupt?: boolean
}

/** Stop 事件提取的数据 */
export interface StopData {
  stopHookActive?: boolean
  lastAssistantMessage?: string
}

/** StopFailure 事件提取的数据 */
export interface StopFailureData {
  error?: string
}

/** Notification 事件提取的数据 */
export interface NotificationData {
  notificationType?: string
  message?: string
  title?: string
}

/** SubagentStart 事件提取的数据 */
export interface SubagentStartData {
  agentId?: string
  agentType?: string
}

/** SubagentStop 事件提取的数据 */
export interface SubagentStopData {
  agentId?: string
  agentType?: string
}

/** PreCompact 事件提取的数据 */
export interface PreCompactData {
  trigger?: string
}

/** PostCompact 事件提取的数据 */
export interface PostCompactData {
  trigger?: string
}

// ---- 带标签的事件详情 ----

export type HookEventDetail =
  | { type: 'sessionStart'; data: SessionStartData }
  | { type: 'sessionEnd'; data: SessionEndData }
  | { type: 'userPromptSubmit'; data: UserPromptSubmitData }
  | { type: 'preToolUse'; data: PreToolUseData }
  | { type: 'postToolUse'; data: PostToolUseData }
  | { type: 'postToolUseFailure'; data: PostToolUseFailureData }
  | { type: 'stop'; data: StopData }
  | { type: 'stopFailure'; data: StopFailureData }
  | { type: 'notification'; data: NotificationData }
  | { type: 'subagentStart'; data: SubagentStartData }
  | { type: 'subagentStop'; data: SubagentStopData }
  | { type: 'preCompact'; data: PreCompactData }
  | { type: 'postCompact'; data: PostCompactData }
  | { type: 'unknown'; data: Record<string, unknown> }

// ---- 完整 payload ----

/** 从 Rust 后端 emit 的完整事件 payload */
export interface HookEventPayload {
  ptyId: string | null
  sessionId: string | null
  eventName: string
  state: ClaudeState
  timestamp: number
  detail: HookEventDetail
}

/** hook-status 事件 payload（Rust 状态机单点计算，前端 Tab working/pending 唯一来源） */
export interface HookStatusPayload {
  ptyId: string
  working: boolean
  pending: boolean
  sessionId: string | null
  /** 触发本次计算的事件（供前端 UI 决策：自动命名 / sessionId 分配） */
  trigger: HookEventDetail
}

// ---- 前端聚合状态 ----

/** 每个会话的聚合 hook 状态 */
export interface SessionHookState {
  ptyId: string
  sessionId?: string
  state: ClaudeState
  model?: string
  lastUpdatedAt: number
}
