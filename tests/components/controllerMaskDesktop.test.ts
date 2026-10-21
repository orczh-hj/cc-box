import { describe, it, expect, vi, beforeEach } from 'vitest'
import { mount, flushPromises } from '@vue/test-utils'
import { nextTick } from 'vue'

const mockT = vi.fn((key: string) => key)
vi.mock('vue-i18n', () => ({
  useI18n: () => ({ t: mockT }),
}))

// 桌面端环境
vi.mock('@/api/platform', () => ({ isTauriEnv: true }))

const bus = vi.hoisted(() => ({
  listeners: new Map<string, Set<(p: unknown) => void>>(),
}))
vi.mock('@/api/transport', () => ({
  transport: {
    listen: async (name: string, cb: (p: unknown) => void) => {
      let set = bus.listeners.get(name)
      if (!set) {
        set = new Set()
        bus.listeners.set(name, set)
      }
      set.add(cb)
      return () => set!.delete(cb)
    },
  },
}))

// controllerStatus：桌面端挂载时查询当前控制者（修复点：初始快照）
const statusMock = vi.hoisted(() => vi.fn(async () => ''))
vi.mock('@/api/tauri', () => ({
  controllerTakeover: vi.fn(async () => {}),
  controllerStatus: () => statusMock(),
}))

import ControllerMask from '@/components/ControllerMask.vue'

function emit(name: string, payload: unknown) {
  for (const cb of bus.listeners.get(name) ?? []) cb(payload)
}

describe('ControllerMask 桌面端（初始快照）', () => {
  beforeEach(() => {
    bus.listeners.clear()
    statusMock.mockReset()
  })

  it('ControllerMask_DesktopSnapshot_挂载时他人已持有则遮罩', async () => {
    // 场景：手机持有中，桌面 UI 启动/重载（如切 Tab reload）错过历史
    // controller-changed 事件 → 必须主动查询当前控制者补初始快照，
    // 否则双端同时操作且桌面无任何遮罩提示
    statusMock.mockResolvedValue('7')
    const wrapper = mount(ControllerMask)
    await flushPromises()

    expect(statusMock).toHaveBeenCalledTimes(1)
    expect(wrapper.find('.controller-mask').exists()).toBe(true)
  })

  it('ControllerMask_DesktopSnapshot_无人持有不遮罩', async () => {
    statusMock.mockResolvedValue('')
    const wrapper = mount(ControllerMask)
    await flushPromises()

    expect(wrapper.find('.controller-mask').exists()).toBe(false)
  })

  it('ControllerMask_DesktopSnapshot_桌面自己持有不遮罩', async () => {
    statusMock.mockResolvedValue('desktop')
    const wrapper = mount(ControllerMask)
    await flushPromises()

    expect(wrapper.find('.controller-mask').exists()).toBe(false)
  })

  it('ControllerMask_DesktopSnapshot_事件解除遮罩', async () => {
    statusMock.mockResolvedValue('7')
    const wrapper = mount(ControllerMask)
    await flushPromises()
    expect(wrapper.find('.controller-mask').exists()).toBe(true)

    // 持有者断连 → 服务端广播空 id → 解除
    emit('controller-changed', { id: '' })
    await nextTick()
    expect(wrapper.find('.controller-mask').exists()).toBe(false)
  })
})
