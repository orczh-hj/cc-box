import { watch, onMounted, onUnmounted, type Ref } from 'vue'
import { getCurrentWindow, UserAttentionType } from '@tauri-apps/api/window'
import { useSessionStore } from '@/stores/session'
import { isTauriEnv } from '@/api/platform'
import { onHookStatus, onSessionViewed } from '@/api/tauri'
import type { HookStatusPayload } from '@/types/hook'

/**
 * Tab working/pending 状态监控：消费 Rust 状态机的 hook-status 事件。
 * 状态语义（回合保护、recap 防护、permission 恢复等）在 Rust
 * apply_hook_event 单点实现，桌面/手机同一份前端仅应用结果——不再复刻状态机。
 * 前端保留的 UI 决策：新 pending 的「正看不清 / 失焦任务栏跳动」、
 * userPromptSubmit 自动命名、sessionStart 的 sessionId 分配。
 */
export function useStatusMonitor(options: { isFocused: Ref<boolean>; isTerminalVisible: Ref<boolean> }) {
  const sessionStore = useSessionStore()
  // 任务栏跳动为桌面专属；浏览器模式 win 为 null（其余状态监控逻辑照常）
  const win = isTauriEnv ? getCurrentWindow() : null

  let unlisten: (() => void) | null = null
  let unlistenViewed: (() => void) | null = null

  const handler = (status: HookStatusPayload) => {
    const tab = sessionStore.getTabByPtyId(status.ptyId)
    if (!tab || tab.status !== 'running') return

    // Rust 状态机算好的最终状态直接应用
    const wasPending = tab.pending
    tab.working = status.working
    tab.pending = status.pending

    // 新进入 pending 的 UI 决策：用户正在看 → 不产生提示；应用失焦 → 任务栏跳动
    if (status.pending && !wasPending) {
      if (
        options.isFocused.value &&
        options.isTerminalVisible.value &&
        tab.tabId === sessionStore.activeTabId
      ) {
        tab.pending = false
        return
      }
      if (!options.isFocused.value) {
        win?.requestUserAttention(UserAttentionType.Critical).catch(() => {})
      }
    }

    // sessionStart：分配 sessionId（Tab 元数据在前端；Rust 侧 set_session_id
    // 已维护远程会话表，两处存储各司其职）
    if (status.trigger.type === 'sessionStart') {
      const sessionId = status.sessionId
      if (sessionId) {
        const data = status.trigger.data
        sessionStore.assignSessionIdByPtyId(status.ptyId, sessionId, data.model)
      }
      return
    }

    // userPromptSubmit：无自定义标题时，用首条用户消息作为标题
    if (status.trigger.type === 'userPromptSubmit') {
      if (tab.name === 'New Session' || tab.name === tab.sessionId?.slice(0, 8)) {
        const prompt = status.trigger.data.prompt?.trim()
        if (prompt) {
          sessionStore.updateTabName(tab.tabId, prompt.length > 50 ? prompt.slice(0, 50) + '…' : prompt)
        }
      }
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
    { immediate: true },
  )

  onMounted(async () => {
    unlisten = await onHookStatus(handler)
    // 某端打开（attach）了 pending 会话：服务端已清"待关注"，本端 Tab 状态点同步
    // 去金黄（不监听则会被后续 sessions.list/hook-status 的服务端旧值冲回）
    unlistenViewed = await onSessionViewed(({ id }) => {
      const tab = sessionStore.getTabByPtyId(id)
      if (tab) tab.pending = false
    })
  })

  onUnmounted(() => {
    unlisten?.()
    unlisten = null
    unlistenViewed?.()
    unlistenViewed = null
  })
}
