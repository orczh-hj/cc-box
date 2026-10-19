import { watch, onMounted, onUnmounted, type Ref } from 'vue'
import { getCurrentWindow, UserAttentionType } from '@tauri-apps/api/window'
import { useHookStore, type HookEventType, type HookEventHandler } from '@/stores/hook'
import { useSessionStore } from '@/stores/session'
import { isTauriEnv } from '@/api/platform'
import type { HookEventPayload, NotificationData } from '@/types/hook'

const STATUS_EVENTS: HookEventType[] = [
  'sessionStart',
  'userPromptSubmit',
  'preToolUse',
  'postToolUse',
  'postToolUseFailure',
  'subagentStart',
  'subagentStop',
  'stop',
  'stopFailure',
  'notification',
  'sessionEnd',
  'preCompact',
  'postCompact',
]

/** 表示 Claude 正在主动工作的事件 */
const ACTIVITY_EVENTS: Set<HookEventType> = new Set([
  'preToolUse',
  'postToolUse',
  'postToolUseFailure',
  'subagentStart',
  'subagentStop',
  'preCompact',
  'postCompact',
])

export function useStatusMonitor(options: { isFocused: Ref<boolean>; isTerminalVisible: Ref<boolean> }) {
  const hookStore = useHookStore()
  const sessionStore = useSessionStore()
  // 任务栏跳动为桌面专属；浏览器模式 win 为 null（其余状态监控逻辑照常）
  const win = isTauriEnv ? getCurrentWindow() : null

  let unsubscribe: (() => void) | null = null

  /** 跟踪每个 tab 的回合是否已结束（Stop 后 recap 等内部操作不应恢复 working） */
  const turnEnded = new Map<string, boolean>()

  const handler: HookEventHandler = (payload: HookEventPayload) => {
    const ptyId = payload.ptyId!
    const tab = sessionStore.getTabByPtyId(ptyId)
    if (!tab || tab.status !== 'running') return

    // sessionStart：直接分配 session_id
    if (payload.detail.type === 'sessionStart') {
      const sessionId = payload.sessionId
      if (sessionId) {
        const data = payload.detail.data as { model?: string }
        sessionStore.assignSessionIdByPtyId(ptyId, sessionId, data.model)
      }
      return
    }

    // userPromptSubmit → 进入 working + 设置标题 + 开始新回合
    if (payload.detail.type === 'userPromptSubmit') {
      turnEnded.delete(tab.tabId)
      tab.working = true
      // 无自定义标题时，用首条用户消息作为标题
      if (tab.name === 'New Session' || tab.name === tab.sessionId?.slice(0, 8)) {
        const prompt = (payload.detail.data as { prompt?: string }).prompt?.trim()
        if (prompt) {
          sessionStore.updateTabName(tab.tabId, prompt.length > 50 ? prompt.slice(0, 50) + '…' : prompt)
        }
      }
      return
    }

    // 活跃事件：回合结束后忽略（防止 recap 等内部操作恢复 working）
    if (ACTIVITY_EVENTS.has(payload.detail.type)) {
      if (turnEnded.get(tab.tabId)) return
      tab.working = true
      tab.pending = false
      return
    }

    // notification：根据 notification_type 区分处理
    if (payload.detail.type === 'notification') {
      if (!tab.working) return

      const data = payload.detail.data as NotificationData
      const ntype = data.notificationType

      if (ntype === 'idle_prompt') {
        // 回合结束，等待用户下一条消息
        turnEnded.set(tab.tabId, true)
        tab.working = false
        setPendingWithAttention(tab)
        return
      }

      if (ntype === 'permission_prompt' || ntype === 'worker_permission_prompt') {
        // 等待用户授权（回合未结束，不设 turnEnded，授权后 PostToolUse 恢复 working）
        tab.working = false
        setPendingWithAttention(tab)
        return
      }

      // 其他通知类型不改变工作状态
      return
    }

    // stop/stopFailure/sessionEnd：仅在 working 时才处理
    if (!tab.working) return

    // stop/stopFailure 标记回合结束
    if (payload.detail.type !== 'sessionEnd') {
      turnEnded.set(tab.tabId, true)
    }

    tab.working = false

    // sessionEnd 不需要 pending/attention 提示
    if (payload.detail.type === 'sessionEnd') return

    setPendingWithAttention(tab)
  }

  /** 设置 pending 状态：用户正在看时立即清除，否则触发任务栏跳动 */
  function setPendingWithAttention(tab: { tabId: string; pending: boolean }) {
    tab.pending = true
    // 用户正在看这个 tab → 不需要 pending 提示
    if (options.isFocused.value && options.isTerminalVisible.value && tab.tabId === sessionStore.activeTabId) {
      tab.pending = false
      return
    }
    // 应用失焦时触发任务栏跳动
    if (!options.isFocused.value) {
      win?.requestUserAttention(UserAttentionType.Critical).catch(() => {})
    }
  }

  // 聚焦 + 终端可见 + tab 激活 → 清除 pending
  watch(
    [() => options.isFocused.value, () => options.isTerminalVisible.value, () => sessionStore.activeTabId],
    ([focused, visible, activeTabId]) => {
      if (focused && visible && activeTabId) {
        const tab = sessionStore.tabs.get(activeTabId)
        if (tab) tab.pending = false
      }
    },
    { immediate: true }
  )

  onMounted(() => {
    unsubscribe = hookStore.subscribe(STATUS_EVENTS, handler)
  })

  onUnmounted(() => {
    unsubscribe?.()
    unsubscribe = null
  })
}
