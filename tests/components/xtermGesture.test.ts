import { describe, it, expect, vi, beforeEach, afterEach } from 'vitest'
import { mount, flushPromises } from '@vue/test-utils'
import { setActivePinia, createPinia } from 'pinia'
import { nextTick } from 'vue'

// ---- xterm 及其 addon：jsdom 无 canvas/WebGL，全部用最小桩 ----
const terminalMock = vi.hoisted(() => ({
  writes: [] as string[],
  disposeCount: 0,
  focusCount: 0,
  lastTextarea: null as HTMLTextAreaElement | null,
  lastTerminal: null as any,
  dataCb: null as ((data: string) => void) | null,
  resizeCb: null as ((size: { cols: number; rows: number }) => void) | null,
}))

vi.mock('@xterm/xterm', () => ({
  Terminal: class {
    element: HTMLElement | null = null
    textarea: HTMLTextAreaElement
    cols = 80
    rows = 24
    options: Record<string, unknown> = {}
    buffer = { active: { length: 1 } }
    unicode = { activeVersion: '6' }
    constructor() {
      this.textarea = document.createElement('textarea')
      terminalMock.lastTextarea = this.textarea
      terminalMock.lastTerminal = this
    }
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
    focus() { terminalMock.focusCount += 1 }
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

// ---- 传输层 ----
const spawnMock = vi.hoisted(() => ({
  ptySpawn: vi.fn(),
  logMessage: vi.fn(),
  ptyInput: vi.fn(),
  ptyResize: vi.fn(),
}))
vi.mock('@/api/tauri', () => ({
  ptySpawn: spawnMock.ptySpawn,
  ptyInput: spawnMock.ptyInput,
  ptyResize: spawnMock.ptyResize,
  ptyKill: vi.fn(),
  ptyClaim: vi.fn(),
  onPtyOutput: vi.fn(async () => () => {}),
  onPtyExit: vi.fn(async () => () => {}),
  onPtyError: vi.fn(async () => () => {}),
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
// 触摸/手势逻辑仅移动端启用，测试固定为 true
vi.mock('@/composables/useMediaQuery', async () => {
  const { ref } = await import('vue')
  return { useIsMobile: () => ref(true) }
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

/** 挂载组件并启动 t1 tab（spawn 成功路径）。attachTo：事件冒泡/activeElement 均需真实 DOM 树 */
async function mountAndStart() {
  spawnMock.ptySpawn.mockResolvedValue({ id: 'pty1', type: 'claude', cwd: '/tmp/project' })
  const wrapper = mount(XTermTerminal, { props: { fontSize: 14 }, attachTo: document.body })
  const vm = wrapper.vm as unknown as { startTab: (id: string) => Promise<void> }
  await vm.startTab('t1')
  await nextTick()
  await flushPromises()
  await new Promise(r => setTimeout(r, 10))
  return wrapper
}

/** 派发 xterm Gesture 自定义事件（属性直接挂在事件顶层，与 lib 行为一致） */
function dispatchGesture(el: Element, type: string, props: Record<string, number>) {
  const ev = new Event(type)
  for (const [k, v] of Object.entries(props)) {
    // 与 xterm 真实事件一致：属性可写（组件会按需改 translationY）
    Object.defineProperty(ev, k, { value: v, writable: true, configurable: true })
  }
  el.dispatchEvent(ev)
}

describe('XTermTerminal 触摸手势（xterm 6 原生 Gesture）', () => {
  beforeEach(() => {
    setActivePinia(createPinia())
    terminalMock.writes.length = 0
    spawnMock.ptySpawn.mockReset()
    spawnMock.logMessage.mockReset()
    spawnMock.ptyInput.mockReset()
    spawnMock.ptyResize.mockReset()
    terminalMock.disposeCount = 0
    terminalMock.focusCount = 0
    terminalMock.lastTextarea = null
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

  // 滚动手势（translationY ≥ 10）必须收起已聚焦的软键盘
  it('XtermGesture_BlurOnScroll_001', async () => {
    const wrapper = await mountAndStart()
    const ta = terminalMock.lastTextarea!
    document.body.appendChild(ta)
    ta.focus()
    expect(document.activeElement).toBe(ta)
    const blurSpy = vi.spyOn(ta, 'blur')

    const termEl = wrapper.find('.terminal-wrapper[data-tab="t1"]').element
    dispatchGesture(termEl, '-xterm-gesturechange', { translationY: 30 })

    expect(blurSpy).toHaveBeenCalled()
    wrapper.unmount()
  })

  // 轻微位移（< 10，tap 抖动范围）不得误收软键盘
  it('XtermGesture_BlurThreshold_002', async () => {
    const wrapper = await mountAndStart()
    const ta = terminalMock.lastTextarea!
    ta.focus()
    const blurSpy = vi.spyOn(ta, 'blur')

    const termEl = wrapper.find('.terminal-wrapper[data-tab="t1"]').element
    dispatchGesture(termEl, '-xterm-gesturechange', { translationY: 5 })

    expect(blurSpy).not.toHaveBeenCalled()
    wrapper.unmount()
  })

  // tap 不再自动唤出软键盘（用户反馈：点击终端非输入区弹键盘打扰阅读；
  // 唤出唯一入口 = 工具条「键盘」按钮）。tap 只保留 mouse reporting 合成链路，
  // 不得聚焦 textarea
  it('XtermGesture_TapNoAutoFocus_003', async () => {
    const wrapper = await mountAndStart()
    const ta = terminalMock.lastTextarea!
    document.body.appendChild(ta)

    const termEl = wrapper.find('.terminal-wrapper[data-tab="t1"]').element
    dispatchGesture(termEl, '-xterm-gesturetap', {})

    expect(document.activeElement).not.toBe(ta)
    wrapper.unmount()
  })

  // 回归（用户实测：点击终端仍弹键盘）：mouse reporting 激活时 tap 会合成
  // mousedown 派发到 term.element，而 xterm 的 _handleMouseDown 无条件
  // textarea.focus()——合成事件期间必须屏蔽 focus，否则鼠标合成链路又成了
  // 隐藏的唤出路径。这里用监听器模拟 xterm 内部的聚焦行为。
  it('XtermGesture_TapMouseReportNoFocus_006', async () => {
    const wrapper = await mountAndStart()
    const ta = terminalMock.lastTextarea!
    document.body.appendChild(ta)
    const termEl = wrapper.find('.terminal-wrapper[data-tab="t1"]').element as HTMLElement
    termEl.classList.add('enable-mouse-events')
    // 模拟 xterm _handleMouseDown 的 i.focus()
    let downCount = 0
    termEl.addEventListener('mousedown', () => { downCount += 1; ta.focus() })
    const focusSpy = vi.spyOn(ta, 'focus')

    dispatchGesture(termEl, '-xterm-gesturetap', { pageX: 10, pageY: 20, tapCount: 1 })

    expect(downCount).toBe(1)
    expect(focusSpy).not.toHaveBeenCalled()
    expect(document.activeElement).not.toBe(ta)
    wrapper.unmount()
  })

  // 键盘开关（工具条「键盘」按钮）：未聚焦时聚焦 textarea（唤出软键盘），
  // 已聚焦时 blur 收起——toggle 语义闭环
  it('XtermGesture_KeyboardToggle_005', async () => {
    const wrapper = await mountAndStart()
    const vm = wrapper.vm as unknown as { toggleSoftKeyboard: () => void }
    const ta = terminalMock.lastTextarea!
    document.body.appendChild(ta)

    vm.toggleSoftKeyboard()
    expect(document.activeElement).toBe(ta)

    vm.toggleSoftKeyboard()
    expect(document.activeElement).not.toBe(ta)
    wrapper.unmount()
  })

  // 回归复现：旧触摸自管层在容器 capture 阶段 stopPropagation 吞掉 touch 事件，
  // 导致 xterm 6 原生 Gesture（document 级监听）收不到 → iPhone 完全无法滚动。
  // touch 事件必须能不被拦截地冒泡到 document 层
  it('XtermGesture_TouchNotIntercepted_004', async () => {
    const wrapper = await mountAndStart()
    let docHits = 0
    document.addEventListener('touchstart', () => { docHits += 1 })

    wrapper.find('.terminal-wrapper[data-tab="t1"]').element.dispatchEvent(
      new Event('touchstart', { bubbles: true, cancelable: true })
    )

    expect(docHits).toBe(1)
    wrapper.unmount()
  })

  // 组件卸载后 gesture 监听必须移除（removeEventListener），不留泄漏
  it('XtermGesture_UnmountCleanup_005', async () => {
    const wrapper = await mountAndStart()
    const ta = terminalMock.lastTextarea!
    document.body.appendChild(ta)
    ta.focus()
    const blurSpy = vi.spyOn(ta, 'blur')

    wrapper.unmount()
    // 容器元素已脱离 DOM，但引用仍在：若监听未移除，直接向其派发仍会触发
    dispatchGesture(wrapper.element, '-xterm-gesturechange', { translationY: 30 })

    expect(blurSpy).not.toHaveBeenCalled()
  })

  // 惯性滚动事件补坐标：xterm 6 beta 的 _inertia 派发的 CHANGE 不带 clientX/clientY，
  // mouse reporting 下 getMouseReportCoords 用缺失坐标换算出 NaN，编码出 "aN;NaNM"
  // 垃圾序列（加速滑动抬起后必现）。GUI 须在 capture 链先于 xterm 补上最后一次
  // 有效坐标
  it('XtermGesture_InertiaCoordsPatched_006', async () => {
    const wrapper = await mountAndStart()
    const termEl = wrapper.find('.terminal-wrapper[data-tab="t1"]').element

    // 模拟手指拖动中的事件（带有效坐标）→ 记录为最后有效坐标
    dispatchGesture(termEl, '-xterm-gesturechange', { clientX: 100, clientY: 200, translationY: 30 })

    // 模拟抬手后的惯性事件（无坐标）→ 组件补齐后 xterm 层应读到有效坐标
    let seen: { clientX?: number; clientY?: number } | null = null
    termEl.addEventListener('-xterm-gesturechange', ev => {
      seen = ev as Event & { clientX?: number; clientY?: number }
    }, true)
    dispatchGesture(termEl, '-xterm-gesturechange', { translationY: 30 })

    expect(seen!.clientX).toBe(100)
    expect(seen!.clientY).toBe(200)
    wrapper.unmount()
  })

  // 手机端滚轮/方向键通道：Claude Code 的 CLAUDE_CODE_SCROLL_SPEED 放大
  // 每步行数，GUI 在 capture 链把 translationY 减半后再交给 xterm
  it('XtermGesture_TouchScrollHalf_008', async () => {
    const wrapper = await mountAndStart()
    const termEl = wrapper.find('.terminal-wrapper[data-tab="t1"]').element
    let seen = 0
    termEl.addEventListener('-xterm-gesturechange', (ev) => {
      seen = (ev as Event & { translationY?: number }).translationY ?? 0
    }, true)
    dispatchGesture(termEl, '-xterm-gesturechange', { translationY: 30 })
    expect(seen).toBe(15)
    wrapper.unmount()
  })

  // normal scrollback 是像素 1:1 直滚，不能缩放
  it('XtermGesture_TouchScrollNormalUnscaled_009', async () => {
    const wrapper = await mountAndStart()
    terminalMock.lastTerminal.buffer.active.type = 'normal'
    const termEl = wrapper.find('.terminal-wrapper[data-tab="t1"]').element
    let seen = 0
    termEl.addEventListener('-xterm-gesturechange', (ev) => {
      seen = (ev as Event & { translationY?: number }).translationY ?? 0
    }, true)
    dispatchGesture(termEl, '-xterm-gesturechange', { translationY: 30 })
    expect(seen).toBe(30)
    wrapper.unmount()
  })

  // 移动端软键盘只由用户 tap 终端唤出：sendText 任何路径（工具条/侧边栏插入）
  // 都不得聚焦终端，否则突然弹出软键盘打断操作
  it('XtermGesture_QuickKeyNoFocus_007', async () => {
    const wrapper = await mountAndStart()
    const vm = wrapper.vm as unknown as { sendText: (text: string, focusAfter?: boolean) => void }
    const before = terminalMock.focusCount

    vm.sendText('\x1b', false)
    expect(terminalMock.focusCount).toBe(before)

    vm.sendText('x')
    expect(terminalMock.focusCount).toBe(before)
    wrapper.unmount()
  })
})
