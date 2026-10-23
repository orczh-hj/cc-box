import { describe, it, expect, vi, beforeEach } from 'vitest'
import { mount } from '@vue/test-utils'

const mockT = vi.fn((key: string) => key)
vi.mock('vue-i18n', () => ({
  useI18n: () => ({ t: mockT }),
}))

import QuickKeyBar from '@/components/QuickKeyBar.vue'

/** label → 期望发送的终端转义序列（action 按钮 select 不在此列） */
const EXPECT: Record<string, string> = {
  'Esc': '\x1b',
  'Enter': '\r',      // 回车：提交（与 Ctrl+J 换行不提交互补）
  '⇧Tab': '\x1b[Z',   // Shift+Tab：Claude CLI 工作模式切换
  'Ctrl+J': '\x0a',    // 换行不提交
  'Ctrl+U': '\x15',    // 清空当前输入行
  '↑': '\x1b[A',
  '↓': '\x1b[B',
  '←': '\x1b[D',
  '→': '\x1b[C',
  'Home': '\x1b[H',
  'End': '\x1b[F',
}

describe('QuickKeyBar（手机快捷键工具条）', () => {
  beforeEach(() => {
    mockT.mockClear()
  })

  // 键序：选择复制 action 按钮（第一位）+ 十一个字节键（Enter 在 Esc 后）；
  // ^C 与「键盘」按钮（点击终端已可唤起软键盘）已按用户决策移除
  it('QuickKeyBar_RendersAllKeys_001', () => {
    const wrapper = mount(QuickKeyBar)
    const labels = wrapper.findAll('button').map(b => b.text())
    expect(labels).toEqual(['quickKeySelectCopy', ...Object.keys(EXPECT)])
    expect(labels).not.toContain('^C')
    expect(labels).not.toContain('quickKeyKeyboard')
  })

  // 点击字节键发射对应转义序列（重点：模式切换、换行与清行）
  it('QuickKeyBar_EmitsEscapeSequences_002', async () => {
    const wrapper = mount(QuickKeyBar)
    const byteBtns = wrapper.findAll('button').filter(b => !b.classes().includes('qk-action'))
    for (const btn of byteBtns) {
      await btn.trigger('click')
    }
    const emitted = wrapper.emitted('send')!
    expect(emitted).toHaveLength(Object.keys(EXPECT).length)
    byteBtns.forEach((btn, i) => {
      expect(emitted[i]).toEqual([EXPECT[btn.text()]])
    })
  })

  // 「选择复制」action 按钮：不发字节，emit select（进入终端选择模式）
  it('QuickKeyBar_SelectAction_003', async () => {
    const wrapper = mount(QuickKeyBar)
    const selectBtn = wrapper.findAll('button').find(b => b.text() === 'quickKeySelectCopy')
    expect(selectBtn!.classes()).toContain('qk-action')
    await selectBtn!.trigger('click')
    expect(wrapper.emitted('select')).toHaveLength(1)
    expect(wrapper.emitted('send')).toBeUndefined()
  })

  // 点击按键不动键盘状态：容器 mousedown preventDefault（不把焦点从终端
  // textarea 移走——移走即 blur 收起软键盘），click 仍正常派发字节
  it('QuickKeyBar_MousedownPrevented_KeyboardStateKept_004', async () => {
    const wrapper = mount(QuickKeyBar)
    // trigger 不执行浏览器默认行为，preventDefault 效果经监听观察
    let prevented = false
    wrapper.find('.quick-key-bar').element.addEventListener('mousedown', (e) => {
      prevented = e.defaultPrevented
    })
    const btn = wrapper.find('.qk-btn:not(.qk-action)')
    await btn.trigger('mousedown')
    expect(prevented).toBe(true)
    await btn.trigger('click')
    expect(wrapper.emitted('send')).toHaveLength(1)
  })

  // 按压高亮（:active 被 mousedown.prevent 吞掉后由 pointer 事件模拟）：
  // pointerdown 落在按键上加 pressing，抬起/离开清除
  it('QuickKeyBar_PressingHighlight_005', async () => {
    const wrapper = mount(QuickKeyBar)
    const btn = wrapper.findAll('.qk-btn:not(.qk-action)')[0]
    await btn.trigger('pointerdown')
    expect(btn.classes()).toContain('pressing')
    await btn.trigger('pointerup')
    expect(btn.classes()).not.toContain('pressing')
  })
})
