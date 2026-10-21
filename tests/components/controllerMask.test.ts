import { describe, it, expect, vi, beforeEach } from 'vitest'
import { mount, flushPromises } from '@vue/test-utils'
import { nextTick } from 'vue'

const mockT = vi.fn((key: string) => key)
vi.mock('vue-i18n', () => ({
  useI18n: () => ({ t: mockT }),
}))

// 可控的远程传输单例替身：connId/controllerLabel 由测试直接改写
// （重连后服务端经 next_conn_id 重新分配连接 id，旧 id 永不复用）
const remote = vi.hoisted(() => ({ connId: null as number | null, controllerLabel: '' }))
vi.mock('@/api/remote-transport', () => ({ remoteTransport: remote }))

// 浏览器端（isTauriEnv=false）：ControllerMask 的 selfLabel/controllerLabel 均来自 remoteTransport
vi.mock('@/api/platform', () => ({ isTauriEnv: false }))

// 事件总线替身：transport.listen 注册，测试手动 emit 模拟服务端事件流
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

const takeoverMock = vi.hoisted(() => vi.fn(async () => {}))
vi.mock('@/api/tauri', () => ({ controllerTakeover: (...a: []) => takeoverMock(...a) }))

import ControllerMask from '@/components/ControllerMask.vue'

function emit(name: string, payload: unknown) {
  for (const cb of bus.listeners.get(name) ?? []) cb(payload)
}

function mountMask() {
  return mount(ControllerMask)
}

describe('ControllerMask 浏览器端（重连换 connId 后自我遮罩）', () => {
  beforeEach(() => {
    bus.listeners.clear()
    takeoverMock.mockClear()
    remote.connId = null
    remote.controllerLabel = ''
  })

  it('ControllerMask_SelfLabel_重连换connId后自己持有不遮罩', async () => {
    // 首次连接：connId=1，挂载时自己持有（controllerLabel='1'）→ 无遮罩
    remote.connId = 1
    remote.controllerLabel = '1'
    const wrapper = mountMask()
    await flushPromises()
    expect(wrapper.find('.controller-mask').exists()).toBe(false)

    // 断连重连：服务端分配新 connId=2；hello.ok 快照无人持有 → 事件流：
    // remote-connected（hello.ok 触发）→ controller-changed {id:''}
    remote.connId = 2
    remote.controllerLabel = ''
    emit('remote-connected', {})
    emit('controller-changed', { id: '' })
    await nextTick()

    // 重连后 attach 终端 → 写操作自动持有 → 广播 {id:'2'}（新 id）
    remote.controllerLabel = '2'
    emit('controller-changed', { id: '2' })
    await nextTick()

    // 修复点：selfLabel 必须随 remote-connected 刷新为 '2'，
    // 否则旧快照 '1' ≠ '2' → 自己遮自己（bug：接管也解不开）
    expect(wrapper.find('.controller-mask').exists()).toBe(false)
  })

  it('ControllerMask_SelfLabel_重连后接管确认解除遮罩', async () => {
    // connId=1 挂载，无人持有
    remote.connId = 1
    remote.controllerLabel = ''
    const wrapper = mountMask()
    await flushPromises()
    expect(wrapper.find('.controller-mask').exists()).toBe(false)

    // 断连重连（connId=2），桌面趁机持有 → 手机被遮罩（正确）
    remote.connId = 2
    remote.controllerLabel = 'desktop'
    emit('remote-connected', {})
    emit('controller-changed', { id: 'desktop' })
    await nextTick()
    expect(wrapper.find('.controller-mask').exists()).toBe(true)

    // 点「接管」→ controller.takeover → 服务端转移并广播 {id:2} +
    // 接管确认触发 remote-connected（remote-transport applyControllerChanged）
    await wrapper.find('.controller-mask-btn').trigger('click')
    expect(takeoverMock).toHaveBeenCalledTimes(1)
    remote.controllerLabel = '2'
    emit('controller-changed', { id: '2' })
    await nextTick()
    emit('remote-connected', {})

    // 修复点：新 connId '2' == 刷新后的 selfLabel → 遮罩解除
    expect(wrapper.find('.controller-mask').exists()).toBe(false)
  })
})
