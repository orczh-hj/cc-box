import { beforeEach, describe, expect, it, vi } from 'vitest'
import { createPinia, setActivePinia } from 'pinia'
import { useSessionStore } from '@/stores/session'
import {
  canLaunchNewWindow,
  shouldOfferLaunchWindow,
  waitForNewInstance,
} from '@/utils/remoteWindows'

describe('remoteWindows 窗口拉起判定', () => {
  // 判定真值：远程（非 Tauri）且 Gateway 名单已知——与窗口数无关的入口
  // （终端页空状态仍要求零窗口）由需求区分，非实现复述
  it('RemoteWindows_HomeButtonIgnoresInstanceCount_1', () => {
    expect(canLaunchNewWindow(false, true)).toBe(true) // 多窗口也允许新建
  })

  it('RemoteWindows_HomeButtonRequiresRemoteAndKnown_2', () => {
    expect(canLaunchNewWindow(true, true)).toBe(false) // 桌面自身是成员
    expect(canLaunchNewWindow(false, false)).toBe(false) // 名单未加载（避免误显示）
  })

  it('RemoteWindows_EmptyStateStillRequiresZero_3', () => {
    expect(shouldOfferLaunchWindow(false, true, 0)).toBe(true)
    expect(shouldOfferLaunchWindow(false, true, 2)).toBe(false)
    expect(shouldOfferLaunchWindow(true, true, 0)).toBe(false)
  })
})

describe('waitForNewInstance 等待新实例上线', () => {
  beforeEach(() => {
    setActivePinia(createPinia())
  })

  it('WaitForNewInstance_ResolvesNewId_1', async () => {
    const store = useSessionStore()
    store.setRemoteInstances([{ id: 'a', name: 'A' }])
    const promise = waitForNewInstance(new Set(['a']), 5000)
    // 新窗口注册 → 名单变化
    store.setRemoteInstances([
      { id: 'a', name: 'A' },
      { id: 'b', name: 'CC-Box' },
    ])
    await expect(promise).resolves.toBe('b')
  })

  it('WaitForNewInstance_IgnoresKnownIds_TimeoutNull_2', async () => {
    vi.useFakeTimers()
    try {
      const store = useSessionStore()
      store.setRemoteInstances([{ id: 'a', name: 'A' }])
      const promise = waitForNewInstance(new Set(['a']), 100)
      // 名单变化但全是已知 id：不 resolve
      store.setRemoteInstances([{ id: 'a', name: 'A (2)' }])
      await vi.advanceTimersByTimeAsync(150)
      await expect(promise).resolves.toBeNull()
    } finally {
      vi.useRealTimers()
    }
  })

  it('WaitForNewInstance_AlreadyArrived_RaceSafe_3', async () => {
    // spawn 已返回、instances-changed 在 watch 注册前已到达：不得漏掉
    const store = useSessionStore()
    store.setRemoteInstances([{ id: 'new', name: 'CC-Box' }])
    await expect(waitForNewInstance(new Set(['old']))).resolves.toBe('new')
  })
})
