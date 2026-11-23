import { describe, it, expect, vi, beforeEach } from 'vitest'
import { mount, flushPromises } from '@vue/test-utils'
import { setActivePinia, createPinia } from 'pinia'

const mockT = vi.fn((key: string) => key)
vi.mock('vue-i18n', () => ({
  useI18n: () => ({ t: mockT }),
  createI18n: () => ({ global: { locale: { value: 'en' } } }),
}))

// ecab98c 重构把 lifecycleScope 改名为 scopedProjectPath 后漏改 update/uninstall
// 调用点——点击「更新/卸载」抛 ReferenceError，请求根本没发到后端（CLI 手动执行正常、
// Rust 日志无失败痕迹）。本测试守卫：更新/卸载必须真正调用对应 API。
const mockUpdate = vi.fn()
const mockUninstall = vi.fn()
vi.mock('@/api/tauri', () => ({
  updatePlugin: (...a: unknown[]) => mockUpdate(...a),
  uninstallPlugin: (...a: unknown[]) => mockUninstall(...a),
}))

vi.mock('@/composables/useTerminalCommand', () => ({
  sendTerminalCommand: vi.fn(),
}))

const mockConfirm = vi.fn()
vi.mock('@/composables/useConfirm', () => ({
  uiConfirm: (...a: unknown[]) => mockConfirm(...a),
}))

vi.mock('@/api/transport', () => ({
  transport: { listen: vi.fn().mockResolvedValue(() => {}) },
}))

vi.mock('@/stores/sidebar', () => ({
  useSidebarStore: () => ({
    loadPlugins: vi.fn(),
    togglePluginEnabled: vi.fn().mockResolvedValue(undefined),
  }),
}))

// cwd 可变：projectPath prop 与 cwd 兜底必须能区分（同值断言无信息量）
let mockCwd: string | undefined = 'D:/other'
vi.mock('@/stores/app', () => ({
  useAppStore: () => ({ cwd: mockCwd }),
}))

import PluginItem from '@/components/plugins/PluginItem.vue'
import type { PluginInfo } from '@/types'

const plugin = {
  id: 'paper-tool@ccbox-market',
  name: 'paper-tool',
  version: '2.7.0',
  scope: 'user',
  enabled: true,
} as unknown as PluginInfo

describe('PluginItem 更新/卸载调用', () => {
  beforeEach(() => {
    setActivePinia(createPinia())
    mockUpdate.mockReset().mockResolvedValue(undefined)
    mockUninstall.mockReset().mockResolvedValue(undefined)
    mockConfirm.mockReset().mockResolvedValue(true)
  })

  // user 作用域点「更新」必须以 (id, 'user', undefined) 调 updatePlugin，
  // 且不抛 ReferenceError（曾因 lifecycleScope 未定义而前端崩溃）
  it('PluginItem_UpdateCallsApi_1', async () => {
    const w = mount(PluginItem, { props: { plugin } })
    await w.find('.pa-btn').trigger('click')
    await flushPromises()
    expect(mockUpdate).toHaveBeenCalledWith('paper-tool@ccbox-market', 'user', undefined)
    expect(w.find('.plugin-action-error').exists()).toBe(false)
  })

  // user 作用域点「卸载」：二次确认后必须调用 uninstallPlugin
  it('PluginItem_UninstallCallsApiAfterConfirm_2', async () => {
    const w = mount(PluginItem, { props: { plugin } })
    const btns = w.findAll('.pa-btn')
    await btns[1].trigger('click')
    await flushPromises()
    expect(mockConfirm).toHaveBeenCalled()
    expect(mockUninstall).toHaveBeenCalledWith('paper-tool@ccbox-market', 'user', undefined)
  })

  // project 作用域必须带上 projectPath（CLI 需在该项目 cwd 执行）；prop 优先于
  // cwd 兜底（cwd='D:/other' ≠ 'D:/proj'，断言值只能来自 prop）
  it('PluginItem_UpdateProjectScopeRequiresPath_3', async () => {
    const w = mount(PluginItem, {
      props: { plugin: { ...plugin, scope: 'project', projectPath: 'D:/proj' } as PluginInfo },
    })
    await w.find('.pa-btn').trigger('click')
    await flushPromises()
    expect(mockUpdate).toHaveBeenCalledWith('paper-tool@ccbox-market', 'project', 'D:/proj')
  })

  // project 作用域且 prop 与 cwd 均无路径 → 拒绝调用（防兜底 cwd 误操作另一个项目）
  it('PluginItem_UpdateProjectScopeRejectsWithoutPath_4', async () => {
    mockCwd = undefined
    const w = mount(PluginItem, {
      props: { plugin: { ...plugin, scope: 'project' } as PluginInfo },
    })
    await w.find('.pa-btn').trigger('click')
    await flushPromises()
    expect(mockUpdate).not.toHaveBeenCalled()
    expect(w.find('.plugin-action-error').exists()).toBe(true)
  })
})
