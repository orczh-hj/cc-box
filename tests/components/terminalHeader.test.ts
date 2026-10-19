import { describe, it, expect, vi, beforeEach } from 'vitest'
import { mount } from '@vue/test-utils'
import { setActivePinia, createPinia } from 'pinia'

const mockT = vi.fn((key: string) => key)
vi.mock('vue-i18n', () => ({
  useI18n: () => ({ t: mockT }),
  // app store 经 @/i18n 间接调用 createI18n（composition 模式 locale 为 ref）
  createI18n: () => ({ global: { locale: { value: 'en' } } }),
}))

// snapWindow 内部调 Tauri 窗口 API，mock 隔离
vi.mock('@/composables/useAppShortcuts', () => ({
  snapWindow: vi.fn(),
}))

// isTauriEnv 按用例切换：getter 转发到可变状态（vi.mock 工厂不能引用普通外部变量）
const mockEnv = vi.hoisted(() => ({ isTauri: true }))
vi.mock('@/api/platform', () => ({
  get isTauriEnv() { return mockEnv.isTauri },
}))

import TerminalHeader from '@/components/TerminalHeader.vue'
import { useSessionStore } from '@/stores/session'

function mountHeader() {
  return mount(TerminalHeader, { props: { projectName: 'demo-proj' } })
}

/** 活跃 Tab 名为「会话A」的公共前置 */
function seedActiveTab() {
  const sessionStore = useSessionStore()
  sessionStore.tabs.set('t1', {
    tabId: 't1', projectPath: 'D:/demo', ptyId: null, sessionId: null,
    name: '会话A', status: 'running', createdAt: 0, lastActiveAt: 0,
    working: false, pending: false, isResume: false,
  })
  sessionStore.activeTabId = 't1'
}

describe('TerminalHeader（项目名显示 + 浏览器模式窗口按钮隐藏）', () => {
  beforeEach(() => {
    setActivePinia(createPinia())
    seedActiveTab()
  })

  // 软件端（Tauri）只显示会话名，不显示项目名（用户决策：软件端不显示项目名）
  it('Header_ShowsProjectAndSession_001', () => {
    mockEnv.isTauri = true
    const wrapper = mountHeader()
    expect(wrapper.find('.header-project').exists()).toBe(false)
    expect(wrapper.find('.header-title').text()).toBe('会话A')
  })

  // 浏览器远程端：项目名与会话名同屏（无原生标题栏，header 是唯一展示位）
  it('Header_BrowserShowsProject_004', () => {
    mockEnv.isTauri = false
    const wrapper = mountHeader()
    expect(wrapper.find('.header-project').text()).toBe('demo-proj')
    expect(wrapper.find('.header-title').text()).toBe('会话A')
  })

  // 桌面 Tauri：窗口管理按钮（置顶/左/右半屏）+ 返回齐全
  it('Header_TauriWindowButtons_002', () => {
    mockEnv.isTauri = true
    const wrapper = mountHeader()
    expect(wrapper.findAll('.header-btn')).toHaveLength(4)
  })

  // 浏览器模式（任意宽度）：窗口管理按钮不渲染，仅保留返回
  it('Header_BrowserHidesWindowButtons_003', () => {
    mockEnv.isTauri = false
    const wrapper = mountHeader()
    expect(wrapper.find('.window-btn').exists()).toBe(false)
    expect(wrapper.findAll('.header-btn')).toHaveLength(1)
  })
})
