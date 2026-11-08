import { describe, it, expect, vi } from 'vitest'
import { mount } from '@vue/test-utils'

vi.mock('vue-i18n', () => ({
  useI18n: () => ({ t: (key: string) => key }),
}))

import KeyboardPanel from '@/components/KeyboardPanel.vue'

/**
 * 键盘面板（三模式）：tab 切换、快捷键项直发、原生键盘触发。
 * 快捷键项期望值为字面 ANSI 序列（readline/xterm 规范 + Claude Code 文档键位），
 * 独立于实现。
 */
function shortcutBtn(wrapper: ReturnType<typeof mount>, bytes: string) {
  const btn = wrapper.findAll('button').find(b => b.attributes('aria-label') === bytes)
  expect(btn, `shortcut ${JSON.stringify(bytes)} should exist`).toBeDefined()
  return btn!
}

describe('KeyboardPanel（手机键盘三模式）', () => {
  // 三个 tab 均渲染，active 高亮跟随 prop
  it('KeyboardPanel_RendersThreeTabs_001', () => {
    const wrapper = mount(KeyboardPanel, { props: { activeTab: 'shortcuts' } })
    const labels = wrapper.findAll('.kp-tab').map(b => b.text())
    expect(labels).toEqual(['kbTabNative', 'kbTabShortcuts', 'kbTabPc'])
    expect(wrapper.findAll('.kp-tab')[1].classes()).toContain('active')
  })

  // 切 tab：点击 emit update:activeTab
  it('KeyboardPanel_SwitchTab_002', async () => {
    const wrapper = mount(KeyboardPanel, { props: { activeTab: 'shortcuts' } })
    await wrapper.findAll('.kp-tab')[2].trigger('click')
    expect(wrapper.emitted('update:activeTab')![0]).toEqual(['pc'])
  })

  // 快捷键项直发字面序列：中断 Ctrl+C → \x03；换行 Ctrl+J → \x0a；
  // 和弦 Ctrl+X Ctrl+K → \x18\x0b（Claude Code 文档定义的键位）
  it('KeyboardPanel_ShortcutSendsBytes_003', async () => {
    const wrapper = mount(KeyboardPanel, { props: { activeTab: 'shortcuts' } })
    await shortcutBtn(wrapper, '\x03').trigger('click')
    await shortcutBtn(wrapper, '\x0a').trigger('click')
    await shortcutBtn(wrapper, '\x18\x0b').trigger('click')
    const emitted = wrapper.emitted('send')!
    expect(emitted[0]).toEqual(['\x03'])
    expect(emitted[1]).toEqual(['\x0a'])
    expect(emitted[2]).toEqual(['\x18\x0b'])
  })

  // 功能名渲染为文案、键位作为副标题（不把组合键当主文案）
  it('KeyboardPanel_ShowsActionNameNotKeys_004', () => {
    const wrapper = mount(KeyboardPanel, { props: { activeTab: 'shortcuts' } })
    const item = shortcutBtn(wrapper, '\x0a')
    expect(item.find('.kp-item-label').text()).toBe('scNewline')
    expect(item.find('.kp-item-keys').text()).toBe('Ctrl+J')
  })

  // 平台差异：macOS 目标 Alt 类快捷键显示 ⌥P（而非 Alt+P）
  it('KeyboardPanel_MacKeyLabels_005', () => {
    const wrapper = mount(KeyboardPanel, { props: { activeTab: 'shortcuts', platform: 'macos' } })
    const item = shortcutBtn(wrapper, '\x1bp')
    expect(item.find('.kp-item-keys').text()).toBe('⌥P')
  })

  // 原生键盘 tab：只切状态（系统软键盘由 TerminalView 监听 activeTab 开合），
  // 且原生模式下不渲染快捷键/电脑键盘内容
  it('KeyboardPanel_NativeTab_006', async () => {
    const wrapper = mount(KeyboardPanel, { props: { activeTab: 'shortcuts' } })
    await wrapper.findAll('.kp-tab')[0].trigger('click')
    expect(wrapper.emitted('update:activeTab')![0]).toEqual(['native'])
    expect(wrapper.emitted('send')).toBeUndefined()

    const native = mount(KeyboardPanel, { props: { activeTab: 'native' } })
    expect(native.findAll('.kp-tab')).toHaveLength(3) // 切换条常驻
    expect(native.find('.kp-shortcuts').exists()).toBe(false)
    expect(native.find('.virtual-keyboard').exists()).toBe(false)
    native.unmount()
  })

  // 电脑键盘 tab：渲染真实布局并透传 send
  it('KeyboardPanel_PcTabForwardsSend_007', async () => {
    const wrapper = mount(KeyboardPanel, { props: { activeTab: 'pc' } })
    const cBtn = wrapper.findAll('button').find(b => b.attributes('aria-label') === 'c')
    expect(cBtn).toBeDefined()
    await cBtn!.trigger('click')
    expect(wrapper.emitted('send')![0]).toEqual(['c'])
  })

  // 粘贴项（本地动作）：emit paste，不发字节
  it('KeyboardPanel_PasteAction_009', async () => {
    const wrapper = mount(KeyboardPanel, { props: { activeTab: 'shortcuts' } })
    const pasteBtn = wrapper.findAll('button').find(b => b.attributes('aria-label') === 'paste')
    expect(pasteBtn).toBeDefined()
    await pasteBtn!.trigger('click')
    expect(wrapper.emitted('paste')).toHaveLength(1)
    expect(wrapper.emitted('send')).toBeUndefined()
  })

  // 终端范围外的键不收录：退出会话(Ctrl+D)、外部编辑器(Ctrl+G)、暂停(Ctrl+Z)、
  // 图像粘贴(Alt+V)、外部编辑器和弦(Ctrl+X Ctrl+E)
  it('KeyboardPanel_ExcludesOutOfScopeKeys_010', () => {
    const wrapper = mount(KeyboardPanel, { props: { activeTab: 'shortcuts' } })
    const labels = wrapper.findAll('button').map(b => b.attributes('aria-label'))
    for (const bytes of ['\x04', '\x07', '\x1a', '\x1bv', '\x18\x05']) {
      expect(labels).not.toContain(bytes)
    }
  })

  // 补充的非官方操作：光标左移 / 右移（终端常用，Claude 文档绑定表外）
  it('KeyboardPanel_ExtraCursorKeys_011', async () => {
    const wrapper = mount(KeyboardPanel, { props: { activeTab: 'shortcuts' } })
    await shortcutBtn(wrapper, '\x1b[D').trigger('click')
    await shortcutBtn(wrapper, '\x1b[C').trigger('click')
    const emitted = wrapper.emitted('send')!
    expect(emitted[0]).toEqual(['\x1b[D'])
    expect(emitted[1]).toEqual(['\x1b[C'])
  })

  // 关闭按钮：emit close，不发字节
  it('KeyboardPanel_Close_008', async () => {
    const wrapper = mount(KeyboardPanel, { props: { activeTab: 'shortcuts' } })
    await wrapper.find('.kp-close').trigger('click')
    expect(wrapper.emitted('close')).toHaveLength(1)
    expect(wrapper.emitted('send')).toBeUndefined()
  })
})
