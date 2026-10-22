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
    attachCustomKeyEventHandler() {}
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

// ---- 传输层：唯一被驱动的入口是 ptySpawn ----
const spawnMock = vi.hoisted(() => ({
  ptySpawn: vi.fn(),
  logMessage: vi.fn(),
  ptyInput: vi.fn(),
  ptyResize: vi.fn(),
  ptyErrorCb: null as ((p: { id: string; error: string; stage: string }) => void) | null,
}))
vi.mock('@/api/tauri', () => ({
  ptySpawn: spawnMock.ptySpawn,
  ptyInput: spawnMock.ptyInput,
  ptyResize: spawnMock.ptyResize,
  ptyKill: vi.fn(),
  ptyClaim: vi.fn(),
  onPtyOutput: vi.fn(async () => () => {}),
  onPtyExit: vi.fn(async () => () => {}),
  onPtyError: vi.fn(async (cb: (p: { id: string; error: string; stage: string }) => void) => {
    spawnMock.ptyErrorCb = cb
    return () => {}
  }),
  onSessionTakenOver: vi.fn(async () => () => {}),
  onSessionReclaimed: vi.fn(async () => () => {}),
  logMessage: spawnMock.logMessage,
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
const appMock = vi.hoisted(() => ({
  setCliFailure: vi.fn(),
  clearCliFailure: vi.fn(),
}))
vi.mock('@/stores/app', () => ({
  useAppStore: () => ({
    theme: 'dark',
    cwd: '/tmp',
    claudeOptions: {},
    resetClaudeOptions: vi.fn(),
    setCliFailure: appMock.setCliFailure,
    clearCliFailure: appMock.clearCliFailure,
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

describe('XTermTerminal spawn 失败展示', () => {
  beforeEach(() => {
    setActivePinia(createPinia())
    terminalMock.writes.length = 0
    spawnMock.ptySpawn.mockReset()
    spawnMock.logMessage.mockReset()
    spawnMock.ptyInput.mockReset()
    spawnMock.ptyResize.mockReset()
    spawnMock.ptyErrorCb = null
    terminalMock.disposeCount = 0
    terminalMock.dataCb = null
    terminalMock.resizeCb = null
    appMock.setCliFailure.mockReset()
    appMock.clearCliFailure.mockReset()
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

  // spawn 失败时必须渲染终端容器并写入红字错误，而不是静默白屏
  it('SpawnError_RendersTerminalWithMessage_001', async () => {
    spawnMock.ptySpawn.mockRejectedValue(
      "Failed to spawn claude '/Users/me/.local/bin/claude': EACCES: Permission denied"
    )

    const wrapper = mount(XTermTerminal, { props: { fontSize: 14 } })
    await (wrapper.vm as unknown as { startTab: (id: string) => Promise<void> }).startTab('t1')
    await nextTick()
    await flushPromises()
    await new Promise(r => setTimeout(r, 30))

    // wrapper 元素由 terminalInstances 驱动，失败也必须存在
    expect(wrapper.find('.terminal-wrapper[data-tab="t1"]').exists()).toBe(true)
    expect(terminalMock.writes.some(w =>
      w.includes('启动 Claude CLI 失败') && w.includes('EACCES')
    )).toBe(true)

    wrapper.unmount()
  })

  // 失败后允许再次启动（isPtyStarting 已复位，占位实例可被 restartTab 清理）
  it('SpawnError_AllowsRetry_002', async () => {
    spawnMock.ptySpawn.mockRejectedValue(new Error('boom'))
    const wrapper = mount(XTermTerminal, { props: { fontSize: 14 } })
    const vm = wrapper.vm as unknown as {
      startTab: (id: string) => Promise<void>
      restartTab: (id: string) => Promise<void>
    }

    await vm.startTab('t1')
    await flushPromises()
    await vm.restartTab('t1')
    await flushPromises()

    expect(spawnMock.ptySpawn).toHaveBeenCalledTimes(2)
    wrapper.unmount()
  })

  // 占位实例没有 PTY 事件：Tab 关闭时必须由 tabs 变化兜底回收错误面板
  it('SpawnError_RemovedWhenTabClosed_003', async () => {
    spawnMock.ptySpawn.mockRejectedValue(new Error('boom'))
    const wrapper = mount(XTermTerminal, { props: { fontSize: 14 } })
    const vm = wrapper.vm as unknown as { startTab: (id: string) => Promise<void> }

    await vm.startTab('t1')
    await nextTick()
    await flushPromises()
    await new Promise(r => setTimeout(r, 30))
    expect(wrapper.find('.terminal-wrapper[data-tab="t1"]').exists()).toBe(true)

    useSessionStore().tabs.delete('t1')
    await nextTick()
    await flushPromises()
    expect(wrapper.find('.terminal-wrapper[data-tab="t1"]').exists()).toBe(false)

    wrapper.unmount()
  })

  // pty-error 反哺 CLI 状态：仅 CLI 相关阶段（detection/spawn/startup）上报
  it('PtyError_ReportsCliFailure_004', async () => {
    const wrapper = mount(XTermTerminal, { props: { fontSize: 14 } })
    await flushPromises()

    spawnMock.ptyErrorCb?.({ id: 'pty1', error: 'boom', stage: 'spawn' })
    expect(appMock.setCliFailure).toHaveBeenCalledWith('boom', 'spawn')

    spawnMock.ptyErrorCb?.({ id: 'pty1', error: 'missing cwd', stage: 'validation' })
    expect(appMock.setCliFailure).toHaveBeenCalledTimes(1)

    wrapper.unmount()
  })

  // spawn 成功清除上一次的执行失败状态
  it('SpawnSuccess_ClearsCliFailure_005', async () => {
    spawnMock.ptySpawn.mockResolvedValue({ id: 'pty1', type: 'claude', cwd: '/tmp/project' })
    const wrapper = mount(XTermTerminal, { props: { fontSize: 14 } })
    const vm = wrapper.vm as unknown as { startTab: (id: string) => Promise<void> }

    await vm.startTab('t1')
    await flushPromises()

    expect(appMock.clearCliFailure).toHaveBeenCalled()
    wrapper.unmount()
  })

  // 失败占位实例没有底层 PTY：按键与尺寸变化不得转发到空 ptyId
  it('SpawnError_PlaceholderDoesNotForward_006', async () => {
    spawnMock.ptySpawn.mockRejectedValue(new Error('claude boom'))
    const wrapper = mount(XTermTerminal, { props: { fontSize: 14 } })
    const vm = wrapper.vm as unknown as { startTab: (id: string) => Promise<void> }

    await vm.startTab('t1')
    await nextTick()
    await flushPromises()
    await new Promise(r => setTimeout(r, 30))

    terminalMock.dataCb?.('x')
    terminalMock.resizeCb?.({ cols: 100, rows: 30 })

    expect(spawnMock.ptyInput).not.toHaveBeenCalled()
    expect(spawnMock.ptyResize).not.toHaveBeenCalled()
    wrapper.unmount()
  })

  // 对失败 Tab 再次 startTab：先释放旧占位实例，避免终端 DOM 叠加/泄漏
  it('SpawnError_RepeatedStartReplacesStale_007', async () => {
    spawnMock.ptySpawn.mockRejectedValue(new Error('claude boom'))
    const wrapper = mount(XTermTerminal, { props: { fontSize: 14 } })
    const vm = wrapper.vm as unknown as { startTab: (id: string) => Promise<void> }

    await vm.startTab('t1')
    await nextTick()
    await flushPromises()
    const disposedAfterFirst = terminalMock.disposeCount

    await vm.startTab('t1')
    await nextTick()
    await flushPromises()

    expect(spawnMock.ptySpawn).toHaveBeenCalledTimes(2)
    expect(terminalMock.disposeCount).toBe(disposedAfterFirst + 1)
    wrapper.unmount()
  })
})
