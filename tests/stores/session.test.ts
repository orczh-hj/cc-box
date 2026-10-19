import { describe, it, expect, beforeEach, vi } from 'vitest'
import { setActivePinia, createPinia } from 'pinia'
import { useSessionStore } from '@/stores/session'
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

// Mock @/api/tauri
vi.mock('@/api/tauri', () => ({
  ptyKill: vi.fn().mockResolvedValue(true),
  getSessionCount: vi.fn().mockResolvedValue(0),
  getSessions: vi.fn().mockResolvedValue([]),
  searchSessionMessages: vi.fn().mockResolvedValue([]),
}))

describe('session store', () => {
  beforeEach(() => {
    setActivePinia(createPinia())
  })

  // ==================== createTab ====================

  describe('createTab', () => {
    // 创建 tab 包含正确的 projectPath
    it('CreateTab_ProjectPath_001', () => {
      const store = useSessionStore()
      const tabId = store.createTab('/home/user/project-a')
      const tab = store.tabs.get(tabId)!
      expect(tab.projectPath).toBe('/home/user/project-a')
    })

    // 设置 sessionId="abc12345def" 时 name 取前 8 字符 "abc12345"
    it('CreateTab_SessionIdName_001', () => {
      const store = useSessionStore()
      const tabId = store.createTab('/project', { sessionId: 'abc12345def' })
      const tab = store.tabs.get(tabId)!
      expect(tab.name).toBe('abc12345')
    })

    // 不设 sessionId 时 name 为 "New Session"
    it('CreateTab_DefaultName_001', () => {
      const store = useSessionStore()
      const tabId = store.createTab('/project')
      const tab = store.tabs.get(tabId)!
      expect(tab.name).toBe('New Session')
    })

    // 初始 status 为 "stopped"
    it('CreateTab_InitialState_001', () => {
      const store = useSessionStore()
      const tabId = store.createTab('/project')
      const tab = store.tabs.get(tabId)!
      expect(tab.status).toBe('stopped')
      expect(tab.ptyId).toBeNull()
      expect(tab.sessionId).toBeNull()
      expect(tab.working).toBe(false)
      expect(tab.pending).toBe(false)
    })

    // 连续创建两个 tab 的 tabId 不同
    it('CreateTab_UniqueId_001', () => {
      const store = useSessionStore()
      const id1 = store.createTab('/project')
      const id2 = store.createTab('/project')
      expect(id1).not.toBe(id2)
    })
  })

  // ==================== closeTab ====================

  describe('closeTab', () => {
    // 关闭 tab 后从 store 中移除
    it('CloseTab_Remove_001', async () => {
      const store = useSessionStore()
      const tabId = store.createTab('/project')
      store.setActiveTab(tabId)
      await store.closeTab(tabId)
      expect(store.tabs.has(tabId)).toBe(false)
    })

    // 关闭活跃 tab 时 activeTabId 切换到同项目其他 tab
    it('CloseTab_SwitchActive_001', async () => {
      const store = useSessionStore()
      const id1 = store.createTab('/project-x')
      const id2 = store.createTab('/project-x')
      store.setActiveTab(id2)
      await store.closeTab(id2)
      expect(store.activeTabId).toBe(id1)
    })

    // 关闭项目中最后一个 tab 时 activeTabId 变为 null
    it('CloseTab_LastTab_001', async () => {
      const store = useSessionStore()
      const tabId = store.createTab('/project-y')
      store.setActiveTab(tabId)
      await store.closeTab(tabId)
      expect(store.activeTabId).toBeNull()
    })

    // 关闭有 PTY 的 tab 时，tab 立即删除，PTY 异步 kill 不阻塞
    it('CloseTab_ImmediateDelete_001', async () => {
      const { ptyKill } = await import('@/api/tauri')
      const mockKill = ptyKill as ReturnType<typeof vi.fn>

      const store = useSessionStore()
      const tabId = store.createTab('/project')
      store.setTabPty(tabId, 'pty-immediate')

      // closeTab 不再 await ptyKill，tab 应立即删除
      await store.closeTab(tabId)
      expect(store.tabs.has(tabId)).toBe(false)
      expect(mockKill).toHaveBeenCalledWith('pty-immediate')
    })

    // 关闭 tab 后 claimedSessionIds 释放，历史会话自动显示
    it('CloseTab_ReleaseClaim_001', async () => {
      const store = useSessionStore()
      const tabId = store.createTab('/project', { sessionId: 'sess-release' })

      expect(store.claimedSessionIds.has('sess-release')).toBe(true)
      await store.closeTab(tabId)
      expect(store.claimedSessionIds.has('sess-release')).toBe(false)
    })
  })

  // ==================== handlePtyExit ====================

  describe('handlePtyExit', () => {
    // PTY 退出后 tab 状态设为 stopped
    it('PtyExit_Status_001', () => {
      const store = useSessionStore()
      const tabId = store.createTab('/project')
      store.setTabPty(tabId, 'pty-001')
      store.handlePtyExit('pty-001')
      const tab = store.tabs.get(tabId)!
      expect(tab.status).toBe('stopped')
    })

    // PTY 退出后 ptyId 清空
    it('PtyExit_ClearPtyId_001', () => {
      const store = useSessionStore()
      const tabId = store.createTab('/project')
      store.setTabPty(tabId, 'pty-002')
      store.handlePtyExit('pty-002')
      const tab = store.tabs.get(tabId)!
      expect(tab.ptyId).toBeNull()
    })

    // PTY 退出后 working 设为 false
    it('PtyExit_ClearWorking_001', () => {
      const store = useSessionStore()
      const tabId = store.createTab('/project')
      store.setTabPty(tabId, 'pty-003')
      const tab = store.tabs.get(tabId)!
      tab.working = true
      store.handlePtyExit('pty-003')
      expect(tab.working).toBe(false)
    })
  })

  // ==================== claimedSessionIds ====================

  describe('claimedSessionIds', () => {
    // 两个 tab 分别设置 sessionId 时集合包含两个值
    it('ClaimedIds_Include_001', () => {
      const store = useSessionStore()
      const id1 = store.createTab('/project', { sessionId: 'sess-alpha' })
      const id2 = store.createTab('/project', { sessionId: 'sess-beta' })
      store.setTabSessionId(id1, 'sess-alpha')
      store.setTabSessionId(id2, 'sess-beta')
      const claimed = store.claimedSessionIds
      expect(claimed.has('sess-alpha')).toBe(true)
      expect(claimed.has('sess-beta')).toBe(true)
      expect(claimed.size).toBe(2)
    })

    // sessionId 为 null 的 tab 不出现在集合中
    it('ClaimedIds_ExcludeNull_001', () => {
      const store = useSessionStore()
      store.createTab('/project') // no sessionId
      const claimed = store.claimedSessionIds
      expect(claimed.size).toBe(0)
    })
  })

  // ==================== isResume ====================

  describe('isResume', () => {
    // 创建 tab 时传入 sessionId，isResume 为 true
    it('IsResume_True_001', () => {
      const store = useSessionStore()
      const tabId = store.createTab('/project', { sessionId: 'sess-resume-001' })
      const tab = store.tabs.get(tabId)!
      expect(tab.isResume).toBe(true)
    })

    // 创建 tab 时不传 sessionId，isResume 为 false
    it('IsResume_False_001', () => {
      const store = useSessionStore()
      const tabId = store.createTab('/project')
      const tab = store.tabs.get(tabId)!
      expect(tab.isResume).toBe(false)
    })

    // 创建 tab 时传入 sessionId 和 name，isResume 仍为 true
    it('IsResume_WithName_001', () => {
      const store = useSessionStore()
      const tabId = store.createTab('/project', { sessionId: 'sess-123', name: 'My Session' })
      const tab = store.tabs.get(tabId)!
      expect(tab.isResume).toBe(true)
      expect(tab.name).toBe('My Session')
    })
  })

  // ==================== adoptRemoteSessions（浏览器远程模式：运行中会话恢复入口） ====================

  describe('adoptRemoteSessions', () => {
    // 空列表采纳 2 个运行中会话：各建 running Tab，绑定 ptyId，name 取目录名
    it('AdoptRemoteSessions_NewSessions_001', () => {
      const store = useSessionStore()
      store.adoptRemoteSessions([
        { id: 'pty-1', type: 'claude', cwd: 'D:\\work\\proj-a', working: true, pending: false },
        { id: 'pty-2', type: 'claude', cwd: '/home/user/proj-b', working: false, pending: true },
      ])
      expect(store.tabs.size).toBe(2)
      const tab1 = store.getTabByPtyId('pty-1')!
      expect(tab1.status).toBe('running')
      expect(tab1.projectPath).toBe('D:\\work\\proj-a')
      expect(tab1.name).toBe('proj-a')
      expect(tab1.working).toBe(true)
      const tab2 = store.getTabByPtyId('pty-2')!
      expect(tab2.name).toBe('proj-b')
      expect(tab2.pending).toBe(true)
    })

    // 重复采纳同 ptyId：不新建 Tab，仅更新 working/pending
    it('AdoptRemoteSessions_DedupByPtyId_001', () => {
      const store = useSessionStore()
      store.adoptRemoteSessions([
        { id: 'pty-1', type: 'claude', cwd: '/proj', working: false, pending: false },
      ])
      const tabId = store.getTabByPtyId('pty-1')!.tabId
      store.adoptRemoteSessions([
        { id: 'pty-1', type: 'claude', cwd: '/proj', working: true, pending: false },
      ])
      expect(store.tabs.size).toBe(1)
      const tab = store.getTabByPtyId('pty-1')!
      expect(tab.tabId).toBe(tabId)
      expect(tab.working).toBe(true)
    })

    // 已退出的会话不在列表中返回时，已有 Tab 不受影响（等待 pty-exit 事件清理）
    it('AdoptRemoteSessions_KeepExistingTabs_001', () => {
      const store = useSessionStore()
      store.adoptRemoteSessions([
        { id: 'pty-1', type: 'claude', cwd: '/proj', working: false, pending: false },
      ])
      store.adoptRemoteSessions([])
      expect(store.tabs.size).toBe(1)
    })
  })
})
