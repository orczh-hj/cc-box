import { describe, it, expect, beforeEach, vi } from 'vitest'
import { setActivePinia, createPinia } from 'pinia'
import { randomUUID } from 'crypto'
import { useSessionStore } from '@/stores/session'

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

vi.mock('@/api/tauri', () => ({
  ptyKill: vi.fn().mockResolvedValue(true),
  getSessionCount: vi.fn().mockResolvedValue(0),
  getSessions: vi.fn().mockResolvedValue([]),
  searchSessionMessages: vi.fn().mockResolvedValue([]),
}))

describe('hasActivePtys 检测（runningTabIds）', () => {
  beforeEach(() => {
    setActivePinia(createPinia())
  })

  // 启动 PTY 后返回 true
  it('ActivePtys_HasPtyId_001', () => {
    const store = useSessionStore()
    const tabId = store.createTab('/project')
    store.setTabPty(tabId, 'pty-001')
    expect(store.runningTabIds.length > 0).toBe(true)
  })

  // 多 tab 混合：部分运行中，部分未启动，返回 true
  it('ActivePtys_MixedTabs_001', () => {
    const store = useSessionStore()
    const id1 = store.createTab('/project-a')
    store.createTab('/project-b')
    store.setTabPty(id1, 'pty-100')
    expect(store.runningTabIds.length > 0).toBe(true)
  })

  // PTY 退出后返回 false
  it('ActivePtys_AfterPtyExit_001', () => {
    const store = useSessionStore()
    const tabId = store.createTab('/project')
    store.setTabPty(tabId, 'pty-200')
    expect(store.runningTabIds.length > 0).toBe(true)
    store.handlePtyExit('pty-200')
    expect(store.runningTabIds.length > 0).toBe(false)
  })
})
