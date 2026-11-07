import { describe, it, expect, vi } from 'vitest'
import { mount } from '@vue/test-utils'

// i18n mock：t 返回键名（组件断言不依赖文案内容）
vi.mock('vue-i18n', () => ({
  useI18n: () => ({ t: (key: string) => key }),
}))

import VirtualKeyboard from '@/components/VirtualKeyboard.vue'

/**
 * 完整虚拟键盘弹层（手机端）：布局、粘滞修饰键状态机、字节发射。
 * 期望值全部为字面 ANSI 序列（xterm/VT100 规范），禁止用 vkeyToBytes 计算
 * 期望值（自闭环断言无信息量）。
 */
// 按 aria-label 定位按键（字母键 aria-label 即字母，修饰键为 Ctrl/Alt/Shift）
function key(wrapper: ReturnType<typeof mount>, label: string) {
  const btn = wrapper.findAll('button').find(b => b.attributes('aria-label') === label)
  expect(btn, `key ${label} should exist`).toBeDefined()
  return btn!
}

describe('VirtualKeyboard（手机完整虚拟键盘弹层）', () => {
  // 布局 spot check：QWERTY 字母、数字、功能键、方向键、修饰键、收起按钮齐全
  it('VirtualKeyboard_RendersLayout_001', () => {
    const wrapper = mount(VirtualKeyboard)
    for (const label of ['q', 'w', 'e', 'r', 't', 'y', 'a', 'z', '1', '0', 'space', 'enter', 'ctrl', 'alt', 'shift', 'esc', 'tab', 'up', 'down', 'left', 'right', 'delete', 'backspace', 'f1', 'f12']) {
      key(wrapper, label)
    }
  })

  // 普通键直发：c 发 'c'、enter 发 '\r'、up 发 '\x1b[A'（字面 ANSI 期望）
  it('VirtualKeyboard_PlainKeySends_002', async () => {
    const wrapper = mount(VirtualKeyboard)
    await key(wrapper, 'c').trigger('click')
    await key(wrapper, 'enter').trigger('click')
    await key(wrapper, 'up').trigger('click')
    const emitted = wrapper.emitted('send')!
    expect(emitted[0]).toEqual(['c'])
    expect(emitted[1]).toEqual(['\r'])
    expect(emitted[2]).toEqual(['\x1b[A'])
  })

  // 粘滞 Ctrl：点 Ctrl 高亮（vk-sticky）→ 点 c 发 '\x03' → 粘滞自动清
  //（再点 a 发 'a' 而非 '\x01'）
  it('VirtualKeyboard_StickyCtrl_003', async () => {
    const wrapper = mount(VirtualKeyboard)
    const ctrlBtn = key(wrapper, 'ctrl')
    expect(ctrlBtn.classes()).not.toContain('vk-sticky')
    await ctrlBtn.trigger('click')
    expect(ctrlBtn.classes()).toContain('vk-sticky')
    expect(wrapper.emitted('send')).toBeUndefined() // 修饰键自身不发字节
    await key(wrapper, 'c').trigger('click')
    expect(wrapper.emitted('send')![0]).toEqual(['\x03'])
    expect(ctrlBtn.classes()).not.toContain('vk-sticky') // 组合后自动释放
    await key(wrapper, 'a').trigger('click')
    expect(wrapper.emitted('send')![1]).toEqual(['a'])
  })

  // 粘滞 Shift：Shift → Tab 发 '\x1b[Z'（Claude CLI 模式切换）；组合后粘滞已清
  // （再点 c 发 'c'），重新粘滞 Shift 后 c 发 'C'
  it('VirtualKeyboard_StickyShiftTab_004', async () => {
    const wrapper = mount(VirtualKeyboard)
    await key(wrapper, 'shift').trigger('click')
    await key(wrapper, 'tab').trigger('click')
    expect(wrapper.emitted('send')![0]).toEqual(['\x1b[Z'])
    await key(wrapper, 'c').trigger('click')
    expect(wrapper.emitted('send')![1]).toEqual(['c']) // 粘滞已自动释放
    await key(wrapper, 'shift').trigger('click')
    await key(wrapper, 'c').trigger('click')
    expect(wrapper.emitted('send')![2]).toEqual(['C'])
  })

  // 无映射组合（Ctrl+space）：不 emit send 且粘滞已清（再点 a 发 'a'）
  it('VirtualKeyboard_UnmappedReleasesSticky_005', async () => {
    const wrapper = mount(VirtualKeyboard)
    const ctrlBtn = key(wrapper, 'ctrl')
    await ctrlBtn.trigger('click')
    await key(wrapper, 'space').trigger('click')
    expect(wrapper.emitted('send')).toBeUndefined()
    expect(ctrlBtn.classes()).not.toContain('vk-sticky')
    await key(wrapper, 'a').trigger('click')
    expect(wrapper.emitted('send')![0]).toEqual(['a'])
  })

  // 再点同一修饰键取消粘滞；修饰键可叠加（Ctrl+Shift 粘滞 → 方向键参数化序列）
  it('VirtualKeyboard_ToggleAndStackSticky_006', async () => {
    const wrapper = mount(VirtualKeyboard)
    const ctrlBtn = key(wrapper, 'ctrl')
    await ctrlBtn.trigger('click')
    await ctrlBtn.trigger('click') // 再点取消
    expect(ctrlBtn.classes()).not.toContain('vk-sticky')
    await key(wrapper, 'a').trigger('click')
    expect(wrapper.emitted('send')![0]).toEqual(['a']) // 未参与组合

    await ctrlBtn.trigger('click')
    await key(wrapper, 'shift').trigger('click') // 叠加
    await key(wrapper, 'left').trigger('click')
    expect(wrapper.emitted('send')![1]).toEqual(['\x1b[1;6D']) // ctrl+shift+left
  })

  // 「收起」按钮：emit close 不 emit send
  it('VirtualKeyboard_CloseButton_007', async () => {
    const wrapper = mount(VirtualKeyboard)
    const closeBtn = wrapper.findAll('button').find(b => b.attributes('aria-label') === 'close')
    expect(closeBtn).toBeDefined()
    await closeBtn!.trigger('click')
    expect(wrapper.emitted('close')).toHaveLength(1)
    expect(wrapper.emitted('send')).toBeUndefined()
  })

  // mousedown 被阻止（不偷终端 textarea 焦点——同 QuickKeyBar 焦点保持机制）
  it('VirtualKeyboard_MousedownPrevented_008', async () => {
    const wrapper = mount(VirtualKeyboard)
    let prevented = false
    wrapper.find('.virtual-keyboard').element.addEventListener('mousedown', (e) => {
      prevented = e.defaultPrevented
    })
    await key(wrapper, 'a').trigger('mousedown')
    expect(prevented).toBe(true)
    await key(wrapper, 'a').trigger('click')
    expect(wrapper.emitted('send')).toHaveLength(1)
  })
})
