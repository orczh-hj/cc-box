import { describe, it, expect, vi, beforeEach, afterEach } from 'vitest'
import { mount, flushPromises } from '@vue/test-utils'
import { setActivePinia, createPinia } from 'pinia'
import { nextTick } from 'vue'

// ---- xterm 及其 addon：jsdom 无 canvas/WebGL，全部用最小桩 ----
const terminalMock = vi.hoisted(() => ({
  writes: [] as string[],
  disposeCount: 0,
  dataCb: null as ((data: string) => void) | null,
  resizeCb: null as ((size: { cols: number; rows: number }) => void) | null,
  keyHandler: null as ((e: KeyboardEvent) => boolean) | null,
}))
vi.mock('@xterm/xterm', () => ({
  Terminal: class {
    element: HTMLElement | null = null
    textarea = document.createElement('textarea')
    cols = 80
    rows = 24
    options: Record<string, unknown> = {}
    buffer = { active: { length: 1 } }
    unicode = { activeVersion: '6' }
    loadAddon() {}
    open(el: HTMLElement) { this.element = el }
    write(data: string) { terminalMock.writes.push(data) }
    onData(cb: (data: string) => void) { terminalMock.dataCb = cb; return { dispose() {} } }
    onResize(cb: (size: { cols: number; rows: number }) => void) {
      terminalMock.resizeCb = cb
      return { dispose() {} }
    }
    onRender() { return { dispose() {} } }
    attachCustomKeyEventHandler(cb: (e: KeyboardEvent) => boolean) { terminalMock.keyHandler = cb }
    getSelection() { return '' }
    paste() {}
    focus() {}
    refresh() {}
    scrollToBottom() {}
    clearTextureAtlas() {}
    dispose() { terminalMock.disposeCount += 1; this.element = null }
  },
}))
vi.mock('@xterm/addon-fit', () => ({ FitAddon: class { fit() {} } }))
vi.mock('@xterm/addon-webgl', () => ({
  WebglAddon: class { onContextLoss() {} dispose() {} },
}))
vi.mock('@xterm/addon-unicode11', () => ({ Unicode11Addon: class { dispose() {} } }))
vi.mock('@fontsource/cascadia-code/400.css', () => ({}))
vi.mock('vue-i18n', () => ({ useI18n: () => ({ t: (key: string) => key }) }))

// ---- 传输层：驱动入口 ptySpawn + onPtyExit 回调捕获 ----
const apiMock = vi.hoisted(() => ({
  ptySpawn: vi.fn(),
  logMessage: vi.fn(),
  ptyInput: vi.fn(),
  ptyResize: vi.fn(),
  ptyExitCb: null as ((p: { id: string; exitCode: number }) => void) | null,
}))
vi.mock('@/api/tauri', () => ({
  ptySpawn: apiMock.ptySpawn,
  ptyInput: apiMock.ptyInput,
  ptyResize: apiMock.ptyResize,
  ptyKill: vi.fn(),
  ptyClaim: vi.fn(),
  onPtyOutput: vi.fn(async () => () => {}),
  onPtyExit: vi.fn(async (cb: (p: { id: string; exitCode: number }) => void) => {
    apiMock.ptyExitCb = cb
    return () => {}
  }),
  onPtyError: vi.fn(async () => () => {}),
  onSessionTakenOver: vi.fn(async () => () => {}),
  onSessionReclaimed: vi.fn(async () => () => {}),
  logMessage: apiMock.logMessage,
}))
vi.mock('@/api/platform', () => ({
  isTauriEnv: true,
  copyText: vi.fn(),
  readClipboardText: vi.fn(async () => ''),
}))
vi.mock('@tauri-apps/api/window', () => ({
  getCurrentWindow: () => ({
    onResized: async () => () => {},
    isMinimized: async () => false,
    setTitle: () => Promise.resolve(),
  }),
}))
vi.mock('@tauri-apps/api/webview', () => ({
  getCurrentWebview: () => ({ onDragDropEvent: async () => () => {} }),
}))
vi.mock('@/api/transport', () => ({
  transport: { listen: async () => () => {}, sendClient: vi.fn() },
}))
vi.mock('@/composables/useTerminalCommand', () => ({
  registerTerminalCommand: vi.fn(),
}))
vi.mock('@/composables/useMediaQuery', async () => {
  const { ref } = await import('vue')
  return { useIsMobile: () => ref(false) }
})
vi.mock('@/utils/ime', () => ({ attachImePositionSync: vi.fn() }))

// app store 依赖大量启动期 IPC，仅提供组件读取的字段与 CLI 状态接口
// （reloadRecentSessions：handlePtyExit 会触发近期会话重拉）
vi.mock('@/stores/app', () => ({
  useAppStore: () => ({
    theme: 'dark',
    cwd: '/tmp',
    claudeOptions: {},
    resetClaudeOptions: vi.fn(),
    setCliFailure: vi.fn(),
    clearCliFailure: vi.fn(),
    reloadRecentSessions: vi.fn().mockResolvedValue(undefined),
  }),
}))

import XTermTerminal from '@/components/XTermTerminal.vue'
import { useSessionStore } from '@/stores/session'

// jsdom 缺失的浏览器 API
class ResizeObserverStub {
  observe() {}
  unobserve() {}
  disconnect() {}
}

function seedTab() {
  const store = useSessionStore()
  store.tabs.set('t1', {
    tabId: 't1', projectPath: '/tmp/project', ptyId: null, sessionId: null,
    name: '会话A', status: 'stopped', createdAt: 0, lastActiveAt: 0,
    working: false, pending: false, isResume: false, sessionIds: [],
  })
  store.activeTabId = 't1'
  return store
}

describe('XTermTerminal pty-exit 保留报错现场', () => {
  beforeEach(() => {
    setActivePinia(createPinia())
    terminalMock.writes.length = 0
    apiMock.ptySpawn.mockReset()
    apiMock.logMessage.mockReset()
    apiMock.ptyInput.mockReset()
    apiMock.ptyResize.mockReset()
    apiMock.ptyExitCb = null
    terminalMock.disposeCount = 0
    terminalMock.dataCb = null
    terminalMock.resizeCb = null
    terminalMock.keyHandler = null
    seedTab()

    vi.stubGlobal('ResizeObserver', ResizeObserverStub)
    vi.stubGlobal('requestAnimationFrame', (cb: FrameRequestCallback) => {
      setTimeout(() => cb(0), 0)
      return 0
    })
    Object.defineProperty(document, 'fonts', {
      configurable: true,
      value: { ready: Promise.resolve() },
    })
  })

  afterEach(() => {
    vi.unstubAllGlobals()
  })

  async function mountAndStart() {
    apiMock.ptySpawn.mockResolvedValue({ id: 'pty1', type: 'claude', cwd: '/tmp/project' })
    const wrapper = mount(XTermTerminal, { props: { fontSize: 14 } })
    const vm = wrapper.vm as unknown as { startTab: (id: string) => Promise<void> }
    await vm.startTab('t1')
    await nextTick()
    await flushPromises()
    await new Promise(r => setTimeout(r, 30))
    return wrapper
  }

  // CLI 退出后终端实例必须保留（报错原文留在 scrollback），不得销毁清屏
  it('PtyExit_RetainsTerminal_001', async () => {
    const wrapper = await mountAndStart()
    expect(wrapper.find('.terminal-wrapper[data-tab="t1"]').exists()).toBe(true)

    apiMock.ptyExitCb?.({ id: 'pty1', exitCode: 1 })
    await nextTick()
    await flushPromises()

    // 终端容器仍在、Terminal 未被销毁（报错现场保留，不白屏）
    expect(wrapper.find('.terminal-wrapper[data-tab="t1"]').exists()).toBe(true)
    expect(terminalMock.disposeCount).toBe(0)
    // Tab 状态照旧转 stopped
    const tab = useSessionStore().tabs.get('t1')
    expect(tab?.status).toBe('stopped')
    wrapper.unmount()
  })

  // 退出后实例降级为占位（ptyId 空）：按键与尺寸变化不得转发到已死进程
  it('PtyExit_DoesNotForwardInput_002', async () => {
    const wrapper = await mountAndStart()
    apiMock.ptyExitCb?.({ id: 'pty1', exitCode: 0 })
    await nextTick()
    await flushPromises()

    terminalMock.dataCb?.('x')
    terminalMock.resizeCb?.({ cols: 100, rows: 30 })
    expect(apiMock.ptyInput).not.toHaveBeenCalled()
    expect(apiMock.ptyResize).not.toHaveBeenCalled()
    wrapper.unmount()
  })

  // 占位实例上的快捷键注入（Shift+Enter 换行 / Undo）同样不得转发空 ptyId
  it('PtyExit_DoesNotForwardShortcuts_005', async () => {
    const wrapper = await mountAndStart()
    apiMock.ptyExitCb?.({ id: 'pty1', exitCode: 0 })
    await nextTick()
    await flushPromises()

    const handler = terminalMock.keyHandler!
    handler({ type: 'keydown', shiftKey: true, key: 'Enter', preventDefault() {} } as KeyboardEvent)
    handler({ type: 'keydown', metaKey: true, key: 'z', preventDefault() {} } as KeyboardEvent)
    expect(apiMock.ptyInput).not.toHaveBeenCalled()
    wrapper.unmount()
  })

  // 退出保留的占位实例在重启时被替换，不叠加 DOM
  it('PtyExit_RestartReplacesRetained_003', async () => {
    const wrapper = await mountAndStart()
    apiMock.ptyExitCb?.({ id: 'pty1', exitCode: 1 })
    await nextTick()
    await flushPromises()

    const vm = wrapper.vm as unknown as { restartTab: (id: string) => Promise<void> }
    await vm.restartTab('t1')
    await nextTick()
    await flushPromises()

    expect(apiMock.ptySpawn).toHaveBeenCalledTimes(2)
    expect(terminalMock.disposeCount).toBe(1)
    expect(wrapper.findAll('.terminal-wrapper[data-tab="t1"]').length).toBe(1)
    wrapper.unmount()
  })

  // 关闭 Tab 时保留的实例由 tabs 变化兜底回收
  it('PtyExit_RemovedWhenTabClosed_004', async () => {
    const wrapper = await mountAndStart()
    apiMock.ptyExitCb?.({ id: 'pty1', exitCode: 1 })
    await nextTick()
    await flushPromises()
    expect(wrapper.find('.terminal-wrapper[data-tab="t1"]').exists()).toBe(true)

    useSessionStore().tabs.delete('t1')
    await nextTick()
    await flushPromises()
    expect(wrapper.find('.terminal-wrapper[data-tab="t1"]').exists()).toBe(false)
    wrapper.unmount()
  })
})
