import { describe, it, expect, vi } from 'vitest'

const mocks = vi.hoisted(() => ({ isMac: true, isTauri: true }))
// getter 传递：工厂重跑时读取 mocks 最新值（直接写值会被按值固定）
vi.mock('@/utils/platform', () => ({ get isMac() { return mocks.isMac } }))
vi.mock('@/api/platform', () => ({ get isTauriEnv() { return mocks.isTauri } }))

import { suppressNativeEditMenu, nativeEditMenuSupported } from '@/utils/mac-edit-keys'

/** 构造带指定 class 的元素 */
function el(tag: string, className?: string): HTMLElement {
  const node = document.createElement(tag)
  if (className) node.classList.add(className)
  return node
}

describe('suppressNativeEditMenu（终端持焦抑制原生 Edit 菜单）', () => {
  it('Suppress_TerminalTextarea_001 — xterm 隐藏 textarea 持焦时抑制菜单', () => {
    // 契约：菜单 key equivalent 会拦截按键，xterm-helper-textarea 持焦
    // 必须让路，否则 xterm 的 Cmd+C/V/Z keydown 处理收不到事件
    expect(suppressNativeEditMenu(el('textarea', 'xterm-helper-textarea'))).toBe(true)
  })

  it('Suppress_InputNotSuppressed_002 — 普通输入元素持焦不抑制', () => {
    // 契约：输入框持焦挂 Edit 菜单，Cmd+C/V/X/A/Z 走系统 selector 原生响应
    expect(suppressNativeEditMenu(el('input'))).toBe(false)
    expect(suppressNativeEditMenu(el('textarea'))).toBe(false)
    expect(suppressNativeEditMenu(el('div'))).toBe(false)
  })

  it('Suppress_NonElementTarget_003 — 非元素焦点目标不抑制', () => {
    // focusin 的 target 理论上恒为 Element，null/window 兜底不抑制（保持默认可用）
    expect(suppressNativeEditMenu(null)).toBe(false)
    expect(suppressNativeEditMenu(window)).toBe(false)
  })

  it('Suppress_ClassIsolation_004 — 同标签无该 class 不误伤', () => {
    // 契约：判定只看 xterm-helper-textarea class，普通 textarea（如设置面板多行输入）
    // 即使同为 textarea 也必须挂菜单，否则粘贴失效回归
    const plain = document.createElement('textarea')
    plain.className = 'some-other-input'
    expect(suppressNativeEditMenu(plain)).toBe(false)
  })
})

describe('nativeEditMenuSupported（平台守卫）', () => {
  it('Supported_MacDesktop_001 — mac 桌面启用联动', async () => {
    mocks.isMac = true
    mocks.isTauri = true
    vi.resetModules()
    const mod = await import('@/utils/mac-edit-keys')
    expect(mod.nativeEditMenuSupported).toBe(true)
  })

  it('Supported_BrowserRemote_002 — 浏览器远程模式不启用', async () => {
    // 契约：远程模式无本地 NSMenu，联动与 invoke 均不应执行
    mocks.isMac = true
    mocks.isTauri = false
    vi.resetModules()
    const mod = await import('@/utils/mac-edit-keys')
    expect(mod.nativeEditMenuSupported).toBe(false)
  })

  it('Supported_WindowsDesktop_003 — Windows 桌面不启用', async () => {
    // 契约：Windows 无 NSMenu 拦截问题，WebView2 原生处理 Ctrl+C/V，不联动
    mocks.isMac = false
    mocks.isTauri = true
    vi.resetModules()
    const mod = await import('@/utils/mac-edit-keys')
    expect(mod.nativeEditMenuSupported).toBe(false)
  })
})
