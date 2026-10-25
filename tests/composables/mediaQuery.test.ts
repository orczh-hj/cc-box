import { describe, it, expect, vi, beforeEach, afterEach } from 'vitest'
import { useMediaQuery, useIsMobile } from '@/composables/useMediaQuery'

/** 构造可控的 matchMedia stub（初始 matches + change 事件监听） */
function stubMatchMedia(initialMatches: boolean) {
  const listeners: ((e: { matches: boolean }) => void)[] = []
  const mql = {
    matches: initialMatches,
    addEventListener: (_: string, cb: (e: { matches: boolean }) => void) => listeners.push(cb),
    removeEventListener: (_: string, cb: (e: { matches: boolean }) => void) => {
      const i = listeners.indexOf(cb)
      if (i >= 0) listeners.splice(i, 1)
    },
  }
  vi.stubGlobal('matchMedia', vi.fn(() => mql))
  return {
    /** 模拟断点变化（触发所有注册的 change 监听器） */
    set(matches: boolean) {
      mql.matches = matches
      listeners.forEach(cb => cb({ matches }))
    },
    listenerCount: () => listeners.length,
  }
}

describe('useMediaQuery', () => {
  beforeEach(() => {
    vi.unstubAllGlobals()
  })
  afterEach(() => {
    vi.unstubAllGlobals()
  })

  // 初始匹配 → ref 为 true，且随 change 事件更新
  it('MediaQuery_Matches_001', () => {
    const stub = stubMatchMedia(true)
    const matches = useMediaQuery('(max-width: 768px)')
    expect(matches.value).toBe(true)

    stub.set(false)
    expect(matches.value).toBe(false)

    stub.set(true)
    expect(matches.value).toBe(true)
  })

  // 初始不匹配 → false；变化后 → true
  it('MediaQuery_NoMatch_001', () => {
    const stub = stubMatchMedia(false)
    const matches = useMediaQuery('(max-width: 768px)')
    expect(matches.value).toBe(false)

    stub.set(true)
    expect(matches.value).toBe(true)
  })

  // 多实例互不干扰，各自持有监听器
  it('MediaQuery_MultiInstance_001', () => {
    const stub = stubMatchMedia(false)
    const a = useMediaQuery('(max-width: 768px)')
    const b = useMediaQuery('(min-width: 769px)')
    expect(stub.listenerCount()).toBe(2)

    stub.set(true)
    expect(a.value).toBe(true)
    expect(b.value).toBe(true)
  })

  // 环境 mock 掉 matchMedia（jsdom 原生不支持）→ 恒 false 不抛错
  it('MediaQuery_NoMatchMedia_001', () => {
    vi.stubGlobal('matchMedia', undefined)
    const matches = useMediaQuery('(max-width: 768px)')
    expect(matches.value).toBe(false)
  })

  // useIsMobile 断点表达式与 CSS 断点一致（≤768px 且触屏）
  it('MediaQuery_IsMobile_001', () => {
    const real = window.matchMedia
    const spy = vi.fn(() => ({ matches: false, addEventListener: () => {}, removeEventListener: () => {} }))
    vi.stubGlobal('matchMedia', spy)
    useIsMobile()
    expect(spy).toHaveBeenCalledWith('(max-width: 768px) and (hover: none)')
    vi.stubGlobal('matchMedia', real)
  })

  // 桌面窄窗（宽度 ≤768px 但有 hover 的鼠标输入）不算手机：
  // 仅按宽度判断会把桌面右半屏误判为手机，故查询必须带设备能力条件
  it('MediaQuery_IsMobile_DesktopNarrow_001', () => {
    const real = window.matchMedia
    const queries: string[] = []
    const spy = vi.fn((q: string) => {
      queries.push(q)
      return { matches: false, addEventListener: () => {}, removeEventListener: () => {} }
    })
    vi.stubGlobal('matchMedia', spy)
    useIsMobile()
    expect(queries).toHaveLength(1)
    expect(queries[0]).toContain('max-width: 768px')
    expect(queries[0]).toContain('hover: none')
    vi.stubGlobal('matchMedia', real)
  })
})
