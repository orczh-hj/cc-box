import { describe, it, expect, vi, beforeEach, afterEach } from 'vitest'
import { mount, flushPromises } from '@vue/test-utils'
import { setActivePinia, createPinia } from 'pinia'
import { nextTick } from 'vue'

// ---- xterm 及 addon：jsdom 无 canvas/WebGL，全部用最小桩 ----
const terminalMock = vi.hoisted(() => ({
  writes: [] as string[],
  lastTextarea: null as HTMLTextAreaElement | null,
  lastTerminal: null as any,
}))

vi.mock('@xterm/xterm', () => ({
  Terminal: class {
    element: HTMLElement | null = null
    textarea: HTMLTextAreaElement
    cols = 80
    rows = 24
    options: Record<string, unknown> = {}
    buffer = { active: { length: 1, type: 'normal' } }
    unicode = { activeVersion: '6' }
    constructor() {
      this.textarea = document.createElement('textarea')
      terminalMock.lastTextarea = this.textarea
      terminalMock.lastTerminal = this
    }
    loadAddon() {}
    open(el: HTMLElement) { this.element = el }
    write(data: string) { terminalMock.writes.push(data) }
    onData() { return { dispose() {} } }
    onResize() { return { dispose() {} } }
    onRender() { return { dispose() {} } }
    attachCustomKeyEventHandler() {}
    getSelection() { return '' }
    paste() {}
    focus() {}
    refresh() {}
    scrollToBottom() {}
    clearTextureAtlas() {}
    dispose() { this.element = null }
  },
}))
vi.mock('@xterm/addon-fit', () => ({ FitAddon: class { fit() {} } }))
vi.mock('@xterm/addon-webgl', () => ({ WebglAddon: class { onContextLoss() {} dispose() {} } }))
vi.mock('@xterm/addon-unicode11', () => ({ Unicode11Addon: class { dispose() {} } }))
vi.mock('@fontsource/cascadia-code/400.css', () => ({}))
vi.mock('vue-i18n', () => ({ useI18n: () => ({ t: (key: string) => key }) }))

// ---- 传输层 / Tauri API ----
const apiMock = vi.hoisted(() => ({
  ptyClaim: vi.fn(async () => undefined),
  ptySpawn: vi.fn(),
  sendClient: vi.fn(),
}))

vi.mock('@/api/tauri', () => ({
  ptySpawn: apiMock.ptySpawn,
  ptyInput: vi.fn(),
  ptyResize: vi.fn(),
  ptyKill: vi.fn(),
  ptyClaim: apiMock.ptyClaim,
  onPtyOutput: vi.fn(async () => () => {}),
  onPtyExit: vi.fn(async () => () => {}),
  onPtyError: vi.fn(async () => () => {}),
  onSessionTakenOver: vi.fn(async () => () => {}),
  onSessionReclaimed: vi.fn(async () => () => {}),
  logMessage: vi.fn(),
}))

vi.mock('@/api/platform', () => ({
  isTauriEnv: false,
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
  transport: {
    listen: vi.fn(async () => () => {}),
    sendClient: apiMock.sendClient,
  },
}))

vi.mock('@/composables/useTerminalCommand', () => ({ registerTerminalCommand: vi.fn() }))
vi.mock('@/composables/useMediaQuery', async () => {
  const { ref } = await import('vue')
  return { useIsMobile: () => ref(false) }
})
vi.mock('@/utils/ime', () => ({ attachImePositionSync: vi.fn() }))

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

class ResizeObserverStub {
  observe() {}
  unobserve() {}
  disconnect() {}
}

function seedTabs() {
  const store = useSessionStore()
  for (const [tabId, ptyId, name] of [
    ['t1', 'p1', '会话一'],
    ['t2', 'p2', '会话二'],
  ] as const) {
    store.tabs.set(tabId, {
      tabId, projectPath: '/tmp/project', ptyId, sessionId: null,
      name, status: 'running', createdAt: 0, lastActiveAt: 0,
      working: false, pending: false, isResume: false, sessionIds: [],
    })
  }
  store.activeTabId = null
  return store
}

function sent(): Array<Record<string, unknown>> {
  return apiMock.sendClient.mock.calls.map(([msg]) => msg as Record<string, unknown>)
}

async function flushAll() {
  await nextTick()
  await flushPromises()
  await new Promise(r => setTimeout(r, 20))
  await nextTick()
}

describe('XTermTerminal 浏览器焦点切换', () => {
  beforeEach(() => {
    setActivePinia(createPinia())
    apiMock.ptyClaim.mockClear()
    apiMock.ptySpawn.mockReset()
    apiMock.sendClient.mockClear()
    terminalMock.writes.length = 0

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

  // 切 Tab = 控制权转移：旧会话 detach + subscribe（保留输出），新会话 attach；
  // 切回已订阅会话走 claim（跳过 tail 防重复）
  it('XtermFocus_SwitchTabTransfersControl_001', async () => {
    const store = seedTabs()
    const wrapper = mount(XTermTerminal, { props: { fontSize: 14 }, attachTo: document.body })

    store.activeTabId = 't1'
    await flushAll()
    expect(sent()).toContainEqual({ type: 'session.attach', id: 'p1', cols: 80, rows: 24 })

    apiMock.sendClient.mockClear()
    store.activeTabId = 't2'
    await flushAll()
    expect(sent()).toContainEqual({ type: 'session.detach', id: 'p1' })
    expect(sent()).toContainEqual({ type: 'session.subscribe', id: 'p1' })
    expect(sent()).toContainEqual({ type: 'session.attach', id: 'p2', cols: 80, rows: 24 })

    apiMock.sendClient.mockClear()
    apiMock.ptyClaim.mockClear()
    store.activeTabId = 't1'
    await flushAll()
    expect(apiMock.ptyClaim).toHaveBeenCalledWith('p1', 80, 24)
    expect(sent()).toContainEqual({ type: 'session.detach', id: 'p2' })
    expect(sent()).toContainEqual({ type: 'session.subscribe', id: 'p2' })

    wrapper.unmount()
  })

  // 切到 stopped Tab：只释放旧焦点，不接管任何会话
  it('XtermFocus_SwitchToStoppedReleasesOnly_002', async () => {
    const store = seedTabs()
    store.tabs.set('t3', {
      tabId: 't3', projectPath: '/tmp/project', ptyId: null, sessionId: null,
      name: '已停止', status: 'stopped', createdAt: 0, lastActiveAt: 0,
      working: false, pending: false, isResume: false, sessionIds: [],
    })
    const wrapper = mount(XTermTerminal, { props: { fontSize: 14 }, attachTo: document.body })

    store.activeTabId = 't1'
    await flushAll()
    apiMock.sendClient.mockClear()

    store.activeTabId = 't3'
    await flushAll()
    expect(sent()).toContainEqual({ type: 'session.detach', id: 'p1' })
    expect(sent()).toContainEqual({ type: 'session.subscribe', id: 'p1' })
    // 没有新的 attach
    expect(sent().some(m => m.type === 'session.attach')).toBe(false)

    wrapper.unmount()
  })

  // 新建会话：spawn 内部 attach 新会话前，必须先释放上一个焦点
  it('XtermFocus_SpawnReleasesPrevious_003', async () => {
    const store = seedTabs()
    apiMock.ptySpawn.mockResolvedValue({ id: 'p3', type: 'claude', cwd: '/tmp/project' })
    const wrapper = mount(XTermTerminal, { props: { fontSize: 14 }, attachTo: document.body })

    store.activeTabId = 't1'
    await flushAll()
    apiMock.sendClient.mockClear()

    // 新建 Tab 后立即 startTab（activeTab watch 尚未 flush，覆盖 spawn 前释放路径）
    const tabId = store.createTab('/tmp/project')
    store.activeTabId = tabId
    const vm = wrapper.vm as unknown as { startTab: (id: string) => Promise<void> }
    await vm.startTab(tabId)
    await flushAll()

    expect(apiMock.ptySpawn).toHaveBeenCalled()
    expect(
      sent().filter(m => m.type === 'session.detach' && m.id === 'p1'),
    ).toHaveLength(1)

    wrapper.unmount()
  })
})
