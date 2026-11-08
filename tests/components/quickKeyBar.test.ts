import { describe, it, expect, vi, beforeEach } from 'vitest'
import { mount } from '@vue/test-utils'

const mockT = vi.fn((key: string) => key)
vi.mock('vue-i18n', () => ({
  useI18n: () => ({ t: mockT }),
}))

import QuickKeyBar from '@/components/QuickKeyBar.vue'

/** 工具条常驻高频键 → 期望终端序列（字面量，来自 VT100 规范） */
const HOT_KEYS: Record<string, string> = {
  esc: '\x1b',
  enter: '\r',
  newline: '\x0a', // 换行不提交（Ctrl+J），与 Enter 提交互补
  up: '\x1b[A',
  down: '\x1b[B',
  left: '\x1b[D',
  right: '\x1b[C',
}

describe('QuickKeyBar（手机快捷键工具条）', () => {
  beforeEach(() => {
    mockT.mockClear()
  })

  // 工具条 = 键盘面板入口（图标）+ 选择复制 + 粘贴 + 六个高频键
  it('QuickKeyBar_RendersEntries_001', () => {
    const wrapper = mount(QuickKeyBar)
    const buttons = wrapper.findAll('button')
    expect(buttons).toHaveLength(3 + Object.keys(HOT_KEYS).length)
    expect(buttons[0].find('svg').exists()).toBe(true)
    expect(buttons[0].attributes('aria-label')).toBe('keyboard')
    expect(buttons[1].text()).toBe('quickKeySelectCopy')
    expect(buttons[2].text()).toBe('quickKeyPaste')
  })

  // 键盘按钮 / 粘贴 / 选择复制：各自 emit 动作事件，不发字节
  it('QuickKeyBar_ActionEmits_002', async () => {
    const wrapper = mount(QuickKeyBar)
    const buttons = wrapper.findAll('button')
    await buttons[0].trigger('click')
    await buttons[1].trigger('click')
    await buttons[2].trigger('click')
    expect(wrapper.emitted('toggleKeyboard')).toHaveLength(1)
    expect(wrapper.emitted('select')).toHaveLength(1)
    expect(wrapper.emitted('paste')).toHaveLength(1)
    expect(wrapper.emitted('send')).toBeUndefined()
  })

  // 高频键直发字面序列（Esc / Enter / 四个方向键）
  it('QuickKeyBar_HotKeysSendBytes_003', async () => {
    const wrapper = mount(QuickKeyBar)
    const hotBtns = wrapper.findAll('button').filter(b => b.attributes('aria-label') in HOT_KEYS)
    expect(hotBtns).toHaveLength(Object.keys(HOT_KEYS).length)
    for (const btn of hotBtns) {
      await btn.trigger('click')
    }
    const emitted = wrapper.emitted('send')!
    hotBtns.forEach((btn, i) => {
      expect(emitted[i]).toEqual([HOT_KEYS[btn.attributes('aria-label')!]])
    })
  })

  // 面板开启态键盘按钮常亮
  it('QuickKeyBar_OpenHighlight_004', () => {
    const closed = mount(QuickKeyBar, { props: { keyboardOpen: false } })
    expect(closed.findAll('button')[0].classes()).not.toContain('qk-open')
    closed.unmount()

    const open = mount(QuickKeyBar, { props: { keyboardOpen: true } })
    expect(open.findAll('button')[0].classes()).toContain('qk-open')
    open.unmount()
  })

  // 点击按钮不转移焦点：容器 mousedown preventDefault
  it('QuickKeyBar_MousedownPrevented_005', async () => {
    const wrapper = mount(QuickKeyBar)
    let prevented = false
    wrapper.find('.quick-key-bar').element.addEventListener('mousedown', (e) => {
      prevented = e.defaultPrevented
    })
    await wrapper.findAll('button')[0].trigger('mousedown')
    expect(prevented).toBe(true)
  })
})
