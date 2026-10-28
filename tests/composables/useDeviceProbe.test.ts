import { describe, it, expect, beforeEach, vi } from 'vitest'
import type { StoredDesktop } from '@/api/remote-desktops'

const probeMock = vi.fn()
vi.mock('@/api/remote-probe', () => ({
  probeDesktop: (...args: unknown[]) => probeMock(...args),
}))

type ProbeModule = typeof import('@/composables/useDeviceProbe')
let mod: ProbeModule

const A: StoredDesktop = {
  id: 'a#1', kind: 'relay', token: 't1', origin: 'http://a', name: 'A', savedAt: 1, lastUsedAt: 2,
}
const B: StoredDesktop = {
  id: 'b#2', kind: 'relay', token: 't2', origin: 'http://b', name: 'B', savedAt: 1, lastUsedAt: 1,
}

/** 等探测 promise 的 then/finally 微任务落定 */
const flush = () => new Promise<void>(resolve => setTimeout(resolve, 0))

describe('useDeviceProbe 探测编排', () => {
  beforeEach(async () => {
    vi.resetModules()
    probeMock.mockReset()
    mod = await import('@/composables/useDeviceProbe')
  })

  // 当前连接设备不探测（状态由主连接 connState 提供）
  it('DeviceProbe_SkipActive_001', () => {
    probeMock.mockResolvedValue('online')
    mod.probeAll([A, B], A.id)
    expect(probeMock).toHaveBeenCalledTimes(1)
    expect(probeMock).toHaveBeenCalledWith(B)
    expect(mod.probeStatusOf(B)).toBe('checking')
    expect(mod.probeStatusOf(A)).toBeUndefined()
  })

  // 探测结果写入内存状态
  it('DeviceProbe_StoreResult_002', async () => {
    probeMock.mockResolvedValue('online')
    mod.probeAll([A, B], A.id)
    await flush()
    expect(mod.probeStatusOf(B)).toBe('online')
  })

  // token 变化 → 新 key；旧 key 的结果被清理
  it('DeviceProbe_TokenChangeNewKey_003', async () => {
    probeMock.mockResolvedValue('online')
    mod.probeAll([B], null)
    await flush()
    expect(mod.probeStatusOf(B)).toBe('online')

    const B2: StoredDesktop = { ...B, token: 't2-new' }
    probeMock.mockResolvedValue('auth-failed')
    mod.probeAll([B2], null)
    await flush()
    expect(mod.probeStatusOf(B2)).toBe('auth-failed')
    expect(mod.probeStatusOf(B)).toBeUndefined()
    expect(probeMock).toHaveBeenCalledTimes(2)
  })

  // recheck 强制重新探测，旧一轮在途结果由代次过滤
  it('DeviceProbe_RecheckReprobes_004', async () => {
    const first: { resolve?: (v: string) => void } = {}
    probeMock.mockImplementationOnce(() => new Promise<string>((r) => { first.resolve = r }))
    mod.probeAll([B], null)
    expect(probeMock).toHaveBeenCalledTimes(1)

    probeMock.mockResolvedValue('online')
    mod.recheckDesktops([B], null)
    expect(probeMock).toHaveBeenCalledTimes(2)
    expect(mod.probeStatusOf(B)).toBe('checking')

    // 旧一轮返回不得覆盖新一轮
    first.resolve?.('offline')
    await flush()
    expect(mod.probeStatusOf(B)).toBe('online')
  })

  // 列表移除条目 → 结果清理
  it('DeviceProbe_PruneRemoved_005', async () => {
    probeMock.mockResolvedValue('online')
    mod.probeAll([B], null)
    await flush()
    mod.probeAll([], null)
    expect(mod.probeStatusOf(B)).toBeUndefined()
  })

  // 条目变成当前设备 → 结果清理（改由 connState 展示）
  it('DeviceProbe_PruneWhenBecameActive_006', async () => {
    probeMock.mockResolvedValue('online')
    mod.probeAll([B], null)
    await flush()
    mod.probeAll([B], B.id)
    expect(mod.probeStatusOf(B)).toBeUndefined()
  })
})
