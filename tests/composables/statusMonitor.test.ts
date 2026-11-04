import { describe, it, expect, beforeEach, vi } from 'vitest'
import { createPinia, setActivePinia } from 'pinia'
import { ref, nextTick } from 'vue'
import { randomUUID } from 'crypto'

// crypto.randomUUID polyfill for jsdom
if (typeof globalThis.crypto === 'undefined' || !globalThis.crypto.randomUUID) {
  Object.defineProperty(globalThis, 'crypto', {
    value: {
      ...globalThis.crypto,
      randomUUID: () => randomUUID(),
    },
    writable: true,
    configurable: true,
  })
}

// 捕获 vue 生命周期回调
let mountedCbs: (() => void)[] = []
let unmountedCbs: (() => void)[] = []

vi.mock('vue', async () => {
  const actual = await vi.importActual<typeof import('vue')>('vue')
  return {
    ...actual,
    onMounted: (fn: () => void) => mountedCbs.push(fn),
    onUnmounted: (fn: () => void) => unmountedCbs.push(fn),
  }
})

// 任务栏跳动 mock（hoisted：vi.mock 工厂引用）
const { requestUserAttention } = vi.hoisted(() => ({
  requestUserAttention: vi.fn().mockResolvedValue(undefined),
}))

// Mock @/api/tauri — 捕获 onHookStatus 回调（状态机在 Rust，前端只消费结果）
let capturedDispatch: ((payload: any) => void) | null = null

vi.mock('@/api/tauri', () => ({
  ptyKill: vi.fn().mockResolvedValue(true),
  getSessionCount: vi.fn().mockResolvedValue(0),
  getSessions: vi.fn().mockResolvedValue([]),
  searchSessionMessages: vi.fn().mockResolvedValue([]),
  onHookStatus: vi.fn((cb: (p: any) => void) => {
    capturedDispatch = cb
    return Promise.resolve(() => {})
  }),
  onSessionViewed: vi.fn(() => Promise.resolve(() => {})),
}))

// Mock Tauri window API
vi.mock('@tauri-apps/api/window', () => ({
  getCurrentWindow: () => ({
    requestUserAttention,
  }),
  UserAttentionType: { Critical: 'Critical' },
}))

import { useSessionStore } from '@/stores/session'
import type { HookStatusPayload, HookEventDetail } from '@/types/hook'
import { useStatusMonitor } from '@/composables/useStatusMonitor'

/** 各事件类型对应的默认 data */
const DEFAULT_DATA: Record<string, unknown> = {
  userPromptSubmit: { prompt: 'test prompt' },
  sessionStart: { model: 'claude-sonnet-4-6', cwd: '/project' },
  preToolUse: { toolName: 'Read' },
  postToolUse: { toolName: 'Read' },
  notification: { notificationType: 'idle_prompt', message: 'test' },
  stop: {},
}

/**
 * 构建 hook-status 载荷：working/pending 是 Rust 状态机已算好的最终状态
 * （状态语义由 Rust 测试覆盖，这里只测前端应用与 UI 决策）
 */
function makeStatus(
  triggerType: HookEventDetail['type'],
  ptyId: string = 'pty1',
  overrides?: { working?: boolean; pending?: boolean; sessionId?: string | null; data?: unknown },
): HookStatusPayload {
  return {
    ptyId,
    working: overrides?.working ?? false,
    pending: overrides?.pending ?? false,
    sessionId: overrides?.sessionId ?? null,
    trigger: {
      type: triggerType,
      data: (overrides?.data ?? DEFAULT_DATA[triggerType] ?? null) as any,
    } as HookEventDetail,
  }
}

/** 创建一个处于 running 状态且绑定 ptyId 的 tab */
function createRunningTab(ptyId = 'pty1', projectPath = '/project'): string {
  const store = useSessionStore()
  const tabId = store.createTab(projectPath)
  const tab = store.tabs.get(tabId)!
  tab.status = 'running'
  tab.ptyId = ptyId
  return tabId
}

/** 挂载 useStatusMonitor 并触发 onMounted */
function mountMonitor() {
  const isFocused = ref(true)
  const isTerminalVisible = ref(true)

  useStatusMonitor({ isFocused, isTerminalVisible })

  // 触发 onMounted 回调
  mountedCbs.forEach((fn) => fn())

  return { isFocused, isTerminalVisible }
}

/** 通过捕获的 onHookStatus 回调发送状态 */
function emit(status: HookStatusPayload) {
  capturedDispatch!(status)
}

describe('useStatusMonitor（消费 Rust hook-status）', () => {
  beforeEach(() => {
    setActivePinia(createPinia())
    capturedDispatch = null
    mountedCbs = []
    unmountedCbs = []
    requestUserAttention.mockClear()
  })

  // ==================== 状态应用 ====================

  describe('状态应用', () => {
    it('StatusMonitor_ApplyStatus_Working_001', () => {
      createRunningTab('pty1')
      mountMonitor()

      // Rust 状态机：userPromptSubmit → working=true
      emit(makeStatus('userPromptSubmit', 'pty1', { working: true }))
      expect(useSessionStore().getTabByPtyId('pty1')!.working).toBe(true)
    })

    it('StatusMonitor_ApplyStatus_Pending_001', () => {
      createRunningTab('pty1')
      const { isTerminalVisible } = mountMonitor()
      const store = useSessionStore()

      // 用户没在看（visible=false）：pending 保持，提示用户
      isTerminalVisible.value = false
      emit(makeStatus('stop', 'pty1', { working: false, pending: true }))

      const tab = store.getTabByPtyId('pty1')!
      expect(tab.working).toBe(false)
      expect(tab.pending).toBe(true)
    })

    it('StatusMonitor_NotRunningTab_Ignored_001', () => {
      const store = useSessionStore()
      const tabId = store.createTab('/project')
      store.tabs.get(tabId)!.status = 'stopped' // 非 running
      store.tabs.get(tabId)!.ptyId = 'pty1'
      mountMonitor()

      emit(makeStatus('userPromptSubmit', 'pty1', { working: true }))

      expect(store.tabs.get(tabId)!.working).toBe(false)
    })

    it('StatusMonitor_UnknownPty_Ignored_001', () => {
      createRunningTab('pty1')
      mountMonitor()

      emit(makeStatus('userPromptSubmit', 'pty-other', { working: true }))

      expect(useSessionStore().getTabByPtyId('pty1')!.working).toBe(false)
    })
  })

  // ==================== pending 的 UI 决策 ====================

  describe('pending 的 UI 决策', () => {
    // 用户正在看 tab（聚焦 + 可见 + 激活）→ pending 立即清除
    it('StatusMonitor_WhileWatching_NoPending_001', () => {
      createRunningTab('pty1')
      mountMonitor()
      const store = useSessionStore()
      store.activeTabId = store.getTabByPtyId('pty1')!.tabId

      emit(makeStatus('stop', 'pty1', { working: false, pending: true }))

      const tab = store.getTabByPtyId('pty1')!
      expect(tab.pending).toBe(false)
      expect(requestUserAttention).not.toHaveBeenCalled()
    })

    // 应用失焦 + pending → 任务栏跳动
    it('StatusMonitor_BlurPending_Attention_001', () => {
      createRunningTab('pty1')
      const { isFocused } = mountMonitor()
      isFocused.value = false

      emit(makeStatus('stop', 'pty1', { working: false, pending: true }))

      expect(useSessionStore().getTabByPtyId('pty1')!.pending).toBe(true)
      expect(requestUserAttention).toHaveBeenCalledTimes(1)
    })

    // pending 边沿触发：状态未变化的重复 pending:true 不重复跳动
    it('StatusMonitor_PendingEdge_NoRepeatAttention_001', () => {
      createRunningTab('pty1')
      const { isFocused } = mountMonitor()
      isFocused.value = false

      emit(makeStatus('stop', 'pty1', { working: false, pending: true }))
      // 新 prompt 开回合（Rust 语义：working=true，pending 保持 true）
      emit(makeStatus('userPromptSubmit', 'pty1', { working: true, pending: true }))

      expect(useSessionStore().getTabByPtyId('pty1')!.pending).toBe(true)
      expect(requestUserAttention).toHaveBeenCalledTimes(1)
    })

    // 聚焦 + 终端可见 + tab 激活 → 清除 pending
    it('StatusMonitor_FocusWatch_ClearsPending_001', async () => {
      createRunningTab('pty1')
      const { isFocused, isTerminalVisible } = mountMonitor()
      const store = useSessionStore()
      store.activeTabId = store.getTabByPtyId('pty1')!.tabId

      // 失焦 + 不可见时进入 pending
      isFocused.value = false
      isTerminalVisible.value = false
      emit(makeStatus('stop', 'pty1', { working: false, pending: true }))
      expect(store.getTabByPtyId('pty1')!.pending).toBe(true)

      // 用户回来聚焦 → watch 清除 pending
      isFocused.value = true
      isTerminalVisible.value = true
      await nextTick()

      expect(store.getTabByPtyId('pty1')!.pending).toBe(false)
    })
  })

  // ==================== trigger 的 UI 决策 ====================

  describe('trigger 的 UI 决策', () => {
    // sessionStart 分配 sessionId/model
    it('StatusMonitor_SessionStart_AssignSessionId_001', () => {
      createRunningTab('pty1')
      mountMonitor()

      emit(
        makeStatus('sessionStart', 'pty1', {
          working: true,
          sessionId: 'new-session-id',
          data: { model: 'claude-sonnet-4-6' },
        }),
      )

      const tab = useSessionStore().getTabByPtyId('pty1')!
      expect(tab.sessionId).toBe('new-session-id')
      expect(tab.model).toBe('claude-sonnet-4-6')
    })

    // sessionStart 无 sessionId（旧版桌面端/shell）不分配
    it('StatusMonitor_SessionStart_NoSessionId_001', () => {
      createRunningTab('pty1')
      mountMonitor()

      emit(makeStatus('sessionStart', 'pty1', { sessionId: null }))

      expect(useSessionStore().getTabByPtyId('pty1')!.sessionId).toBeNull()
    })

    // sessionEnd（clear/resume）：旧 sessionId 解除关联、释放认领（sessionIds）、
    // 名重置占位——旧对话的 prompt 名不得残留到新对话
    it('StatusMonitor_SessionEnd_ClearsSessionId_001', () => {
      createRunningTab('pty1')
      mountMonitor()
      const store = useSessionStore()
      store.assignSessionIdByPtyId('pty1', 'old-session-id', 'claude-sonnet-4-6')
      store.updateTabName(store.getTabByPtyId('pty1')!.tabId, '旧对话的 prompt 名')
      expect(store.getTabByPtyId('pty1')!.sessionId).toBe('old-session-id')

      emit(
        makeStatus('sessionEnd', 'pty1', {
          sessionId: 'old-session-id',
          data: { reason: 'clear' },
        }),
      )

      const tab = store.getTabByPtyId('pty1')!
      expect(tab.sessionId).toBeNull()
      expect(tab.sessionIds).toEqual([])
      expect(tab.name).toBe('New Session')
    })

    // /clear 重置后，新对话首条 prompt 自动命名重新生效（名称与实际会话一致）
    it('StatusMonitor_SessionEnd_PromptRenameAfterClear_001', () => {
      createRunningTab('pty1')
      mountMonitor()
      const store = useSessionStore()
      store.assignSessionIdByPtyId('pty1', 'old-session-id', 'claude-sonnet-4-6')
      store.updateTabName(store.getTabByPtyId('pty1')!.tabId, '旧对话的 prompt 名')

      emit(makeStatus('sessionEnd', 'pty1', { sessionId: 'old-session-id', data: { reason: 'clear' } }))
      emit(makeStatus('userPromptSubmit', 'pty1', { working: true, data: { prompt: '新对话首个问题' } }))

      expect(store.getTabByPtyId('pty1')!.name).toBe('新对话首个问题')
    })

    // sessionStart 带 session_title（CLI 内 /rename、--name、resume 历史标题）：
    // Tab 名对齐官方标题（修复同一会话两个名字）
    it('StatusMonitor_SessionStart_AlignsSessionTitle_001', () => {
      createRunningTab('pty1')
      mountMonitor()
      const store = useSessionStore()
      store.assignSessionIdByPtyId('pty1', 'sid-1', 'claude-sonnet-4-6')

      emit(
        makeStatus('sessionStart', 'pty1', {
          sessionId: 'sid-2',
          data: { model: 'claude-sonnet-4-6', source: 'resume', sessionTitle: '登录模块重构' },
        }),
      )

      const tab = store.getTabByPtyId('pty1')!
      expect(tab.name).toBe('登录模块重构')
    })

    // userPromptSubmit 带 session_title：官方标题优先于 prompt 自动命名
    it('StatusMonitor_UserPromptSubmit_SessionTitleWins_001', () => {
      createRunningTab('pty1')
      mountMonitor()

      emit(
        makeStatus('userPromptSubmit', 'pty1', {
          working: true,
          data: { prompt: '随便的第一句话', sessionTitle: '官方标题' },
        }),
      )

      expect(useSessionStore().getTabByPtyId('pty1')!.name).toBe('官方标题')
    })

    // sessionEnd（退出类 reason）：sessionIds 必须保留——stopped Tab 重启
    // 走 --resume 继续，清掉会退化为新会话
    it('StatusMonitor_SessionEnd_ExitKeepsSessionIds_001', () => {
      createRunningTab('pty1')
      mountMonitor()
      const store = useSessionStore()
      store.assignSessionIdByPtyId('pty1', 'sess-still-alive', 'claude-sonnet-4-6')

      emit(
        makeStatus('sessionEnd', 'pty1', {
          sessionId: 'sess-still-alive',
          data: { reason: 'prompt_input_exit' },
        }),
      )

      expect(store.getTabByPtyId('pty1')!.sessionIds).toContain('sess-still-alive')
    })

    // userPromptSubmit：默认名（New Session）时用首条 prompt 命名
    it('StatusMonitor_UserPromptSubmit_AutoName_001', () => {
      createRunningTab('pty1')
      mountMonitor()

      emit(
        makeStatus('userPromptSubmit', 'pty1', {
          working: true,
          data: { prompt: 'fix the login bug' },
        }),
      )

      expect(useSessionStore().getTabByPtyId('pty1')!.name).toBe('fix the login bug')
    })

    // 超 50 字符的 prompt 截断
    it('StatusMonitor_UserPromptSubmit_LongPromptTruncated_001', () => {
      createRunningTab('pty1')
      mountMonitor()

      // 截断 200 与 Rust/CLI 一致（唯一源同长）
      const long = 'a'.repeat(260)
      emit(makeStatus('userPromptSubmit', 'pty1', { working: true, data: { prompt: long } }))

      const name = useSessionStore().getTabByPtyId('pty1')!.name
      expect(name).toBe('a'.repeat(200) + '…')
    })

    // 用户已自定义标题 → 不覆盖
    it('StatusMonitor_UserPromptSubmit_CustomNameKept_001', () => {
      createRunningTab('pty1')
      mountMonitor()
      const store = useSessionStore()
      store.updateTabName(store.getTabByPtyId('pty1')!.tabId, '我的会话')

      emit(makeStatus('userPromptSubmit', 'pty1', { working: true, data: { prompt: 'other' } }))

      expect(store.getTabByPtyId('pty1')!.name).toBe('我的会话')
    })
  })
})
