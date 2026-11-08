import { describe, it, expect, vi } from 'vitest'
import { mount } from '@vue/test-utils'

// i18n mock：t 返回键名（组件断言不依赖文案内容）
vi.mock('vue-i18n', () => ({
  useI18n: () => ({ t: (key: string) => key }),
}))

import VirtualKeyboard from '@/components/VirtualKeyboard.vue'

/**
 * 电脑键盘（真实 PC 布局）：布局完整性、粘滞修饰键、CapsLock、平台键名。
 * 期望值全部为字面 ANSI 序列（xterm/VT100 规范），禁止用 vkeyToBytes 计算
 * 期望值（自闭环断言无信息量）。
 */
function key(wrapper: ReturnType<typeof mount>, label: string) {
  const btn = wrapper.findAll('button').find(b => b.attributes('aria-label') === label)
  expect(btn, `key ${label} should exist`).toBeDefined()
  return btn!
}

describe('VirtualKeyboard（电脑键盘）', () => {
  // 布局完整性：功能排、数字排、三排字母、底排修饰键与方向键
  it('VirtualKeyboard_RealLayout_001', () => {
    const wrapper = mount(VirtualKeyboard)
    for (const label of [
      'esc', 'f1', 'f12',
      '`', '1', '0', '-', '=', 'backspace',
      'tab', 'q', 'p', '[', ']', '\\',
      'caps', 'a', 'l', ';', "'", 'enter',
      'shift', 'z', 'm', ',', '.', '/',
      'ctrl', 'alt', 'space', 'up', 'down', 'left', 'right',
    ]) {
      key(wrapper, label)
    }
  })

  // 普通键直发：c → 'c'、enter → '\r'、up → '\x1b[A'（字面 ANSI 期望）
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

  // 电脑键盘按键显示大写字母，但无 Shift 时发送小写
  it('VirtualKeyboard_ShowsUpperSendsLower_003', async () => {
    const wrapper = mount(VirtualKeyboard)
    const cBtn = key(wrapper, 'c')
    expect(cBtn.text()).toBe('C')
    await cBtn.trigger('click')
    expect(wrapper.emitted('send')![0]).toEqual(['c'])
  })

  // 粘滞 Ctrl：高亮 → 点 c 发 '\x03' → 粘滞自动清（再点 a 发 'a'）
  it('VirtualKeyboard_StickyCtrl_004', async () => {
    const wrapper = mount(VirtualKeyboard)
    const ctrlBtns = wrapper.findAll('button').filter(b => b.attributes('aria-label') === 'ctrl')
    await ctrlBtns[0].trigger('click')
    expect(ctrlBtns[0].classes()).toContain('vk-sticky')
    expect(wrapper.emitted('send')).toBeUndefined()
    await key(wrapper, 'c').trigger('click')
    expect(wrapper.emitted('send')![0]).toEqual(['\x03'])
    expect(ctrlBtns[0].classes()).not.toContain('vk-sticky')
    await key(wrapper, 'a').trigger('click')
    expect(wrapper.emitted('send')![1]).toEqual(['a'])
  })

  // 粘滞 Shift + Tab → '\x1b[Z'（Claude CLI 模式切换）
  it('VirtualKeyboard_StickyShiftTab_005', async () => {
    const wrapper = mount(VirtualKeyboard)
    await key(wrapper, 'tab').trigger('click') // 无修饰：普通 Tab
    await wrapper.findAll('button').filter(b => b.attributes('aria-label') === 'shift')[0].trigger('click')
    await key(wrapper, 'tab').trigger('click')
    const emitted = wrapper.emitted('send')!
    expect(emitted[0]).toEqual(['\t'])
    expect(emitted[1]).toEqual(['\x1b[Z'])
  })

  // 修饰叠加：Ctrl+Shift + 方向左 → 参数化序列 '\x1b[1;6D'
  it('VirtualKeyboard_StackedModifiers_006', async () => {
    const wrapper = mount(VirtualKeyboard)
    await wrapper.findAll('button').filter(b => b.attributes('aria-label') === 'ctrl')[0].trigger('click')
    await wrapper.findAll('button').filter(b => b.attributes('aria-label') === 'shift')[0].trigger('click')
    await key(wrapper, 'left').trigger('click')
    expect(wrapper.emitted('send')![0]).toEqual(['\x1b[1;6D'])
  })

  // 无映射组合（Ctrl+空格）：不发送且粘滞清空
  it('VirtualKeyboard_UnmappedReleasesSticky_007', async () => {
    const wrapper = mount(VirtualKeyboard)
    const ctrlBtns = wrapper.findAll('button').filter(b => b.attributes('aria-label') === 'ctrl')
    await ctrlBtns[0].trigger('click')
    await key(wrapper, 'space').trigger('click')
    expect(wrapper.emitted('send')).toBeUndefined()
    expect(ctrlBtns[0].classes()).not.toContain('vk-sticky')
  })

  // CapsLock：开启后字母发大写；与 Shift 异或（Caps 开 + Shift = 小写）
  it('VirtualKeyboard_CapsLock_008', async () => {
    const wrapper = mount(VirtualKeyboard)
    const capsBtn = key(wrapper, 'caps')
    await capsBtn.trigger('click')
    expect(capsBtn.classes()).toContain('vk-caps')
    await key(wrapper, 'a').trigger('click')
    expect(wrapper.emitted('send')![0]).toEqual(['A'])

    await wrapper.findAll('button').filter(b => b.attributes('aria-label') === 'shift')[0].trigger('click')
    await key(wrapper, 'b').trigger('click')
    expect(wrapper.emitted('send')![1]).toEqual(['b']) // Caps 开 + Shift = 小写
  })

  // 数字排 Shift 符号：Shift + 1 → '!'（符号表正确性）
  it('VirtualKeyboard_ShiftSymbols_009', async () => {
    const wrapper = mount(VirtualKeyboard)
    await wrapper.findAll('button').filter(b => b.attributes('aria-label') === 'shift')[0].trigger('click')
    await key(wrapper, '1').trigger('click')
    expect(wrapper.emitted('send')![0]).toEqual(['!'])
  })

  // CapsLock 不影响数字排符号（Shift+1 仍为 '!'）
  it('VirtualKeyboard_CapsDoesNotAffectSymbols_010', async () => {
    const wrapper = mount(VirtualKeyboard)
    await key(wrapper, 'caps').trigger('click')
    await key(wrapper, '2').trigger('click')
    expect(wrapper.emitted('send')![0]).toEqual(['2'])
  })

  // 平台差异：macOS 目标 Alt 显示 ⌥、Win 显示 ⌘ 且禁用；Windows 目标 Alt/Win
  it('VirtualKeyboard_PlatformLabels_011', () => {
    const mac = mount(VirtualKeyboard, { props: { platform: 'macos' } })
    const macAlt = mac.findAll('button').filter(b => b.attributes('aria-label') === 'alt')
    expect(macAlt[0].text()).toBe('⌥')
    const macWin = mac.findAll('button').filter(b => b.attributes('aria-label') === 'win')
    expect(macWin[0].text()).toBe('⌘')
    expect(macWin[0].attributes('disabled')).toBeDefined()
    mac.unmount()

    const win = mount(VirtualKeyboard, { props: { platform: 'windows' } })
    const winAlt = win.findAll('button').filter(b => b.attributes('aria-label') === 'alt')
    expect(winAlt[0].text()).toBe('Alt')
    const winKey = win.findAll('button').filter(b => b.attributes('aria-label') === 'win')
    expect(winKey[0].text()).toBe('Win')
    expect(winKey[0].attributes('disabled')).toBeDefined()
    win.unmount()
  })

  // 旧桌面端未下发 platform：按 Windows 显示（Alt/Win）
  it('VirtualKeyboard_NoPlatformDefaultsWindows_012', () => {
    const wrapper = mount(VirtualKeyboard)
    const alt = wrapper.findAll('button').filter(b => b.attributes('aria-label') === 'alt')
    expect(alt[0].text()).toBe('Alt')
  })

  // 中/英（全/半角）切换：开启后字母/数字/符号输出全角；控制键不受影响
  it('VirtualKeyboard_FullwidthToggle_014', async () => {
    const wrapper = mount(VirtualKeyboard)
    const imeBtn = key(wrapper, 'ime')
    expect(imeBtn.text()).toBe('En') // 默认半角

    await imeBtn.trigger('click')
    expect(imeBtn.text()).toBe('中')
    expect(imeBtn.classes()).toContain('vk-ime')

    await key(wrapper, 'a').trigger('click')
    await key(wrapper, '1').trigger('click')
    await key(wrapper, 'space').trigger('click')
    const emitted = wrapper.emitted('send')!
    expect(emitted[0]).toEqual(['ａ']) // U+FF41
    expect(emitted[1]).toEqual(['１']) // U+FF11
    expect(emitted[2]).toEqual(['　']) // U+3000 表意空格

    // 控制键不转换：回车仍为 \r，方向键仍为转义序列
    await key(wrapper, 'enter').trigger('click')
    await key(wrapper, 'up').trigger('click')
    expect(wrapper.emitted('send')![3]).toEqual(['\r'])
    expect(wrapper.emitted('send')![4]).toEqual(['\x1b[A'])

    // 关闭后恢复半角
    await imeBtn.trigger('click')
    expect(imeBtn.text()).toBe('En')
    await key(wrapper, 'a').trigger('click')
    expect(wrapper.emitted('send')![5]).toEqual(['a'])
  })

  // 全角 + Shift 符号：Shift+1 先得 '!'，再全角化为 '！'
  it('VirtualKeyboard_FullwidthShiftSymbol_015', async () => {
    const wrapper = mount(VirtualKeyboard)
    await key(wrapper, 'ime').trigger('click')
    await wrapper.findAll('button').filter(b => b.attributes('aria-label') === 'shift')[0].trigger('click')
    await key(wrapper, '1').trigger('click')
    expect(wrapper.emitted('send')![0]).toEqual(['！'])
  })

  // 平台提示：Win 显示 Shift+Space 习惯，Mac 显示 Caps Lock 习惯
  it('VirtualKeyboard_ImeHintPerPlatform_016', () => {
    const win = mount(VirtualKeyboard, { props: { platform: 'windows' } })
    expect(win.findAll('button').filter(b => b.attributes('aria-label') === 'ime')[0]
      .attributes('title')).toBe('vkImeHintWin')
    win.unmount()

    const mac = mount(VirtualKeyboard, { props: { platform: 'macos' } })
    expect(mac.findAll('button').filter(b => b.attributes('aria-label') === 'ime')[0]
      .attributes('title')).toBe('vkImeHintMac')
    mac.unmount()
  })

  // mousedown 被阻止（不偷终端 textarea 焦点）
  it('VirtualKeyboard_MousedownPrevented_013', async () => {
    const wrapper = mount(VirtualKeyboard)
    let prevented = false
    wrapper.find('.virtual-keyboard').element.addEventListener('mousedown', (e) => {
      prevented = e.defaultPrevented
    })
    await key(wrapper, 'a').trigger('mousedown')
    expect(prevented).toBe(true)
  })
})
