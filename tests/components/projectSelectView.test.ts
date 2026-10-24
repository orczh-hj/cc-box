import { describe, it, expect, vi, beforeEach } from 'vitest'
import { mount } from '@vue/test-utils'
import { setActivePinia, createPinia } from 'pinia'
import { mockIPC } from '@tauri-apps/api/mocks'

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

import ProjectSelectView from '@/components/ProjectSelectView.vue'
import { useAppStore } from '@/stores/app'
import { useSessionStore } from '@/stores/session'

function mountView() {
  return mount(ProjectSelectView)
}

describe('ProjectSelectView 隐藏项目弹窗确认', () => {
  beforeEach(() => {
    setActivePinia(createPinia())
    mockIPC(() => null)
    const store = useAppStore()
    store.cacheLoaded = true
    store.cachedProjects = [
      { path: 'D:\\Work\\A', name: 'A' },
      { path: '/home/b', name: 'b' },
    ]
  })

  // 点 × 只弹确认框（含提示文案），不执行隐藏；确认后才调 hide_project
  it('HideConfirm_Dialog_001', async () => {
    const calls: Array<[string, unknown]> = []
    mockIPC((cmd, args) => {
      calls.push([cmd, args])
      return null
    })
    const wrapper = mountView()

    await wrapper.findAll('.project-hide-btn')[0].trigger('click')
    expect(calls).not.toContainEqual(['hide_project', expect.anything()])
    const overlay = wrapper.find('.confirm-overlay')
    expect(overlay.exists()).toBe(true)
    expect(overlay.text()).toContain('hideProjectTitle')
    expect(overlay.text()).toContain('hideProjectHint')

    await overlay.find('.btn-danger').trigger('click')
    expect(wrapper.find('.confirm-overlay').exists()).toBe(false)
    const hideCall = calls.find(([cmd]) => cmd === 'hide_project')
    expect(hideCall).toBeTruthy()
    expect((hideCall![1] as { path: string }).path).toBe('D:\\Work\\A')
  })

  // 取消 / 点遮罩：关闭弹窗且不执行隐藏
  it('HideConfirm_Cancel_Noop_002', async () => {
    const calls: string[] = []
    mockIPC((cmd) => {
      calls.push(cmd)
      return null
    })
    const wrapper = mountView()

    await wrapper.findAll('.project-hide-btn')[0].trigger('click')
    await wrapper.find('.confirm-overlay .btn-cancel').trigger('click')
    expect(wrapper.find('.confirm-overlay').exists()).toBe(false)

    await wrapper.findAll('.project-hide-btn')[0].trigger('click')
    await wrapper.find('.confirm-overlay').trigger('click')
    expect(wrapper.find('.confirm-overlay').exists()).toBe(false)
    expect(calls).not.toContain('hide_project')
  })

  // 隐藏项目的运行中会话不进近期会话；例外：用户本端打开过该项目（setCwd，
  // 打开即对该项目全部会话可见——项目级判定）
  it('RecentSessions_HiddenProjectFiltered_003', async () => {
    mockIPC(() => null)
    const appStore = useAppStore()
    const sessionStore = useSessionStore()
    appStore.hiddenProjects = ['D:\\Work\\A', '/hidden/c']

    // 远程自动采纳的 Tab（用户从未打开过该项目）：running 但近期会话不显示
    const tabAuto = sessionStore.createTab('/hidden/c', { sessionId: 's-auto' })
    sessionStore.setTabPty(tabAuto, 'pty-a')
    // 用户打开过项目（不同大小写写法）的运行 Tab：例外显示
    appStore.setCwd('d:/work/A')
    const tabOpened = sessionStore.createTab('D:\\Work\\A', { sessionId: 's-open' })
    sessionStore.setTabPty(tabOpened, 'pty-b')
    // 非隐藏项目的运行 Tab 正常显示
    const tabOther = sessionStore.createTab('/home/b', { sessionId: 's-other' })
    sessionStore.setTabPty(tabOther, 'pty-c')

    const wrapper = mountView()
    const sessionNames = wrapper.findAll('.session-item .session-name').map(n => n.text())
    expect(sessionNames).toContain('s-open')
    expect(sessionNames).toContain('s-other')
    expect(sessionNames).not.toContain('s-auto')
  })
})
