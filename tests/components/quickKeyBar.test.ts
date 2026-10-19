import { describe, it, expect, vi, beforeEach } from 'vitest'
import { mount } from '@vue/test-utils'

const mockT = vi.fn((key: string) => key)
vi.mock('vue-i18n', () => ({
  useI18n: () => ({ t: mockT }),
}))

import QuickKeyBar from '@/components/QuickKeyBar.vue'

/** label → 期望发送的终端转义序列 */
const EXPECT: Record<string, string> = {
  'Esc': '\x1b',
  '⇧Tab': '\x1b[Z',   // Shift+Tab：Claude CLI 工作模式切换
  'Ctrl+J': '\x0a',    // 换行不提交
  '^C': '\x03',
  '↑': '\x1b[A',
  '↓': '\x1b[B',
}

describe('QuickKeyBar（手机快捷键工具条）', () => {
  beforeEach(() => {
    mockT.mockClear()
  })

  // 六个控制键齐全（软键盘按不出的键全由按钮补齐）
  it('QuickKeyBar_RendersAllKeys_001', () => {
    const wrapper = mount(QuickKeyBar)
    const labels = wrapper.findAll('button').map(b => b.text())
    expect(labels).toEqual(Object.keys(EXPECT))
  })

  // 点击发射对应转义序列（重点：模式切换与换行）
  it('QuickKeyBar_EmitsEscapeSequences_002', async () => {
    const wrapper = mount(QuickKeyBar)
    for (const btn of wrapper.findAll('button')) {
      await btn.trigger('click')
    }
    const emitted = wrapper.emitted('send')!
    expect(emitted).toHaveLength(6)
    const labels = wrapper.findAll('button').map(b => b.text())
    labels.forEach((label, i) => {
      expect(emitted[i]).toEqual([EXPECT[label]])
    })
  })
})
