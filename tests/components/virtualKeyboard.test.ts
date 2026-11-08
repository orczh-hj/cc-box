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
  // 布局完整性：数字排（Esc 最左）、三排字母、底排修饰键与方向键。
  // F1-F12 已移除：Claude Code 无 F 键绑定，整排占屏无收益
  it('VirtualKeyboard_RealLayout_001', () => {
    const wrapper = mount(VirtualKeyboard)
    for (const label of [
      'esc',
      '`', '1', '0', '-', '=', 'backspace',
      'tab', 'q', 'p', '[', ']', '\\',
      'caps', 'a', 'l', ';', "'", 'enter',
      'shift', 'z', 'm', ',', '.', '/',
      'ctrl', 'alt', 'space', 'up', 'down', 'left', 'right',
    ]) {
      key(wrapper, label)
    }
  })

  // 已精简的键不应存在：F1-F12（无绑定）、Win/⌘（终端无序列）、右 Alt（重复）
  it('VirtualKeyboard_RemovedKeys_017', () => {
    const wrapper = mount(VirtualKeyboard)
    const labels = wrapper.findAll('button').map(b => b.attributes('aria-label'))
    for (const removed of ['f1', 'f5', 'f12', 'win']) {
      expect(labels).not.toContain(removed)
    }
    // 仅保留一个 Alt（左侧）
    expect(labels.filter(l => l === 'alt')).toHaveLength(1)
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

  // 电脑键盘默认显示小写字母，发送小写（Shift 时键帽与输出同步变大写）
  it('VirtualKeyboard_ShowsUpperSendsLower_003', async () => {
    const wrapper = mount(VirtualKeyboard)
    const cBtn = key(wrapper, 'c')
    expect(cBtn.text()).toBe('c')
    await cBtn.trigger('click')
    expect(wrapper.emitted('send')![0]).toEqual(['c'])
    await key(wrapper, 'shift').trigger('click')
    expect(cBtn.text()).toBe('C')
    await cBtn.trigger('click')
    expect(wrapper.emitted('send')![1]).toEqual(['C'])
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

  // 平台差异：macOS 目标 Alt 显示 ⌥，Windows 目标显示 Alt（Win 键已移除）
  it('VirtualKeyboard_PlatformLabels_011', () => {
    const mac = mount(VirtualKeyboard, { props: { platform: 'macos' } })
    const macAlt = mac.findAll('button').filter(b => b.attributes('aria-label') === 'alt')
    expect(macAlt[0].text()).toBe('⌥')
    mac.unmount()

    const win = mount(VirtualKeyboard, { props: { platform: 'windows' } })
    const winAlt = win.findAll('button').filter(b => b.attributes('aria-label') === 'alt')
    expect(winAlt[0].text()).toBe('Alt')
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

  // 长按退格连发：300ms 后开始，每 80ms 一次；松手立即停止且不补发 click
  it('VirtualKeyboard_LongPressBackspaceRepeats_018', async () => {
    vi.useFakeTimers()
    try {
      const wrapper = mount(VirtualKeyboard)
      const bsBtn = key(wrapper, 'backspace')
      await bsBtn.trigger('pointerdown')
      expect(wrapper.emitted('send')).toBeUndefined() // 延迟期内未发送

      vi.advanceTimersByTime(300)
      expect(wrapper.emitted('send')).toHaveLength(1) // 首次连发

      vi.advanceTimersByTime(160)
      expect(wrapper.emitted('send')).toHaveLength(3) // +2 次

      await wrapper.find('.virtual-keyboard').trigger('pointerup')
      vi.advanceTimersByTime(500)
      expect(wrapper.emitted('send')).toHaveLength(3) // 松手后停止

      // 长按已连发：松手补发的 click 应被抑制（否则多删一个）
      await bsBtn.trigger('click')
      expect(wrapper.emitted('send')).toHaveLength(3)

      // 每次连发的内容都是退格
      for (const e of wrapper.emitted('send')!) expect(e).toEqual(['\x7f'])
    } finally {
      vi.useRealTimers()
    }
  })

  // 短按（未达长按阈值）：松手 + click 只发送一次，不触发连发
  it('VirtualKeyboard_ShortPressBackspaceOnce_019', async () => {
    vi.useFakeTimers()
    try {
      const wrapper = mount(VirtualKeyboard)
      const bsBtn = key(wrapper, 'backspace')
      await bsBtn.trigger('pointerdown')
      vi.advanceTimersByTime(100) // 未到 300ms
      await wrapper.find('.virtual-keyboard').trigger('pointerup')
      await bsBtn.trigger('click')
      expect(wrapper.emitted('send')).toEqual([['\x7f']])

      vi.advanceTimersByTime(500) // 计时器应已清除，不再发送
      expect(wrapper.emitted('send')).toHaveLength(1)
    } finally {
      vi.useRealTimers()
    }
  })

  // 非退格键不长按连发：按住字母只发 click 一次
  it('VirtualKeyboard_NonRepeatableKeys_020', async () => {
    vi.useFakeTimers()
    try {
      const wrapper = mount(VirtualKeyboard)
      const aBtn = key(wrapper, 'a')
      await aBtn.trigger('pointerdown')
      vi.advanceTimersByTime(1000)
      expect(wrapper.emitted('send')).toBeUndefined()
      await wrapper.find('.virtual-keyboard').trigger('pointerup')
      await aBtn.trigger('click')
      expect(wrapper.emitted('send')).toEqual([['a']])
    } finally {
      vi.useRealTimers()
    }
  })

  // 键帽动态显示——Shift 粘滞时上档符号：, → <、. → >、/ → ?、[ → {、1 → !
  it('VirtualKeyboard_ShiftChangesLabels_021', async () => {
    const wrapper = mount(VirtualKeyboard)
    expect(key(wrapper, ',').text()).toBe(',')
    await key(wrapper, 'shift').trigger('click')
    expect(key(wrapper, ',').text()).toBe('<')
    expect(key(wrapper, '.').text()).toBe('>')
    expect(key(wrapper, '/').text()).toBe('?')
    expect(key(wrapper, '[').text()).toBe('{')
    expect(key(wrapper, '1').text()).toBe('!')
    // 字母大小写随 Shift 切换
    expect(key(wrapper, 'a').text()).toBe('A')
    await key(wrapper, 'x').trigger('click') // 组合后粘滞清空，恢复下档
    expect(key(wrapper, ',').text()).toBe(',')
    expect(key(wrapper, 'a').text()).toBe('a')
  })

  // 键帽动态显示——Caps 粘滞时字母大写（与 Shift 异或），符号不变
  it('VirtualKeyboard_CapsChangesLetterLabels_022', async () => {
    const wrapper = mount(VirtualKeyboard)
    expect(key(wrapper, 'a').text()).toBe('a')
    await key(wrapper, 'caps').trigger('click')
    expect(key(wrapper, 'a').text()).toBe('A')
    expect(key(wrapper, ',').text()).toBe(',') // Caps 不影响符号
    await key(wrapper, 'shift').trigger('click')
    expect(key(wrapper, 'a').text()).toBe('a') // Caps+Shift = 小写
    await key(wrapper, 'shift').trigger('click') // 关闭 shift
    await key(wrapper, 'caps').trigger('click') // 关闭 caps
    expect(key(wrapper, 'a').text()).toBe('a')
  })

  // 键帽动态显示——全角模式：符号显示全角形态；与 Shift 叠加显示全角上档
  it('VirtualKeyboard_FullwidthChangesLabels_023', async () => {
    const wrapper = mount(VirtualKeyboard)
    await key(wrapper, 'ime').trigger('click')
    expect(key(wrapper, ',').text()).toBe('，')
    expect(key(wrapper, 'a').text()).toBe('ａ')
    expect(key(wrapper, '1').text()).toBe('１')
    await key(wrapper, 'shift').trigger('click')
    expect(key(wrapper, ',').text()).toBe('《') // Shift+, → 《（中文书名号）
    expect(key(wrapper, '1').text()).toBe('！')
    expect(key(wrapper, 'a').text()).toBe('Ａ')
    // 静态功能键不受影响
    expect(key(wrapper, 'esc').text()).toBe('Esc')
    expect(key(wrapper, 'enter').text()).toBe('Enter')
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
