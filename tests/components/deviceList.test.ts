import { describe, it, expect, vi } from 'vitest'
import { mount } from '@vue/test-utils'
import DeviceList from '@/components/remote/DeviceList.vue'
import type { ProbeStatus } from '@/api/remote-probe'

vi.mock('vue-i18n', () => ({ useI18n: () => ({ t: (key: string) => key }) }))

const desktops = [
  {
    id: 'a#1', kind: 'relay' as const, token: 't1', origin: 'http://a',
    name: '桌面 A', savedAt: 1, lastUsedAt: 2,
  },
  {
    id: 'b#2', kind: 'lan' as const, token: 't2', origin: 'http://b',
    name: '桌面 B', savedAt: 1, lastUsedAt: 1,
  },
]

type ConnState = 'connected' | 'denied' | 'closed'

function mountList(activeId: string | null = 'a#1', connState: ConnState = 'connected') {
  return mount(DeviceList, {
    props: { desktops, activeId, connState },
  })
}

describe('DeviceList 移除配对', () => {
  // 当前正在连接的设备不显示移除按钮，其他设备显示
  it('DeviceList_ActiveConnectedCannotRemove_001', () => {
    const wrapper = mountList('a#1', 'connected')
    const rows = wrapper.findAll('.device-item')
    expect(rows[0].find('.device-remove').exists()).toBe(false)
    expect(rows[1].find('.device-remove').exists()).toBe(true)
  })

  // 连接已被拒/已关闭：当前设备并非“正在连接”，允许移除
  it('DeviceList_ActiveDeniedCanRemove_002', async () => {
    const wrapper = mountList('a#1', 'denied')
    const rows = wrapper.findAll('.device-item')
    expect(rows[0].find('.device-remove').exists()).toBe(true)

    await rows[0].find('.device-remove').trigger('click')
    await wrapper.find('.confirm-ok').trigger('click')
    expect(wrapper.emitted('remove')?.[0]).toEqual(['a#1'])
  })

  // 非当前设备：第一次点 × 只进入待确认态，确认后才移除
  it('DeviceList_RemoveNeedsConfirm_003', async () => {
    const wrapper = mountList('a#1')
    await wrapper.find('.device-remove').trigger('click')
    expect(wrapper.emitted('remove')).toBeUndefined()
    expect(wrapper.find('.device-confirm').exists()).toBe(true)
    expect(wrapper.find('.device-confirm-hint').text()).toContain('deviceRemoveConfirm')

    await wrapper.find('.confirm-ok').trigger('click')
    expect(wrapper.emitted('remove')?.[0]).toEqual(['b#2'])
    expect(wrapper.find('.device-confirm').exists()).toBe(false)
  })

  // 取消不触发移除
  it('DeviceList_RemoveCancelDoesNotEmit_004', async () => {
    const wrapper = mountList('a#1')
    await wrapper.find('.device-remove').trigger('click')
    await wrapper.find('.confirm-cancel').trigger('click')
    expect(wrapper.emitted('remove')).toBeUndefined()
    expect(wrapper.find('.device-confirm').exists()).toBe(false)
  })

  // 待确认项被父组件从列表移除 → 清除待确认态
  it('DeviceList_PendingClearedWhenItemRemoved_005', async () => {
    const wrapper = mountList('a#1')
    await wrapper.find('.device-remove').trigger('click')
    expect(wrapper.find('.device-confirm').exists()).toBe(true)

    await wrapper.setProps({ desktops: [desktops[0]] })
    expect(wrapper.find('.device-confirm').exists()).toBe(false)
    expect(wrapper.find('.device-confirm-hint').exists()).toBe(false)
  })

  // 待确认设备在外部切换后成为当前设备 → 取消移除
  it('DeviceList_PendingClearedWhenBecameActive_006', async () => {
    const wrapper = mountList('a#1')
    await wrapper.find('.device-remove').trigger('click')
    expect(wrapper.find('.device-confirm').exists()).toBe(true)

    await wrapper.setProps({ activeId: 'b#2' })
    expect(wrapper.find('.device-confirm').exists()).toBe(false)
    expect(wrapper.findAll('.device-item')[1].find('.device-remove').exists()).toBe(false)
  })

  // 点击另一行：清除待确认并选择该设备
  it('DeviceList_ClickRowClearsPendingAndSelects_007', async () => {
    const wrapper = mountList('a#1')
    await wrapper.find('.device-remove').trigger('click')
    expect(wrapper.find('.device-confirm').exists()).toBe(true)

    await wrapper.findAll('.device-item')[1].trigger('click')
    expect(wrapper.emitted('select')?.[0]).toEqual(['b#2'])
    expect(wrapper.find('.device-confirm').exists()).toBe(false)
  })

  // 键盘：内层按钮 Enter 不得冒泡触发行选择；行自身 Enter/Space 可选中
  it('DeviceList_KeyboardEnterDoesNotBubble_008', async () => {
    const wrapper = mountList('a#1')
    const rows = wrapper.findAll('.device-item')

    await rows[1].find('.device-remove').trigger('keydown.enter')
    expect(wrapper.emitted('select')).toBeUndefined()

    await rows[1].trigger('keydown.enter')
    expect(wrapper.emitted('select')?.[0]).toEqual(['b#2'])

    await rows[1].trigger('keydown', { key: ' ' })
    expect(wrapper.emitted('select')?.[1]).toEqual(['b#2'])
  })
})

describe('DeviceList 连通性状态', () => {
  function mountWithStatuses(statuses: Record<string, ProbeStatus>) {
    return mount(DeviceList, {
      props: { desktops, activeId: 'a#1', connState: 'connected', statuses },
    })
  }

  // 非当前设备按探测结果展示状态标签
  it('DeviceList_ProbeStatusTag_009', () => {
    const wrapper = mountWithStatuses({ 'b#2|t2': 'auth-failed' })
    const row = wrapper.findAll('.device-item')[1]
    expect(row.find('.tag.status-auth-failed').text()).toBe('deviceStatusAuthFailed')
  })

  // 探测不可达：点击不切换，行内展开原因，删除入口仍可用
  it('DeviceList_UnreachableClickBlocked_010', async () => {
    const wrapper = mountWithStatuses({ 'b#2|t2': 'offline' })
    const row = wrapper.findAll('.device-item')[1]

    await row.trigger('click')
    expect(wrapper.emitted('select')).toBeUndefined()
    expect(row.find('.device-blocked').text()).toBe('deviceBlockedOffline')
    expect(row.classes()).toContain('blocked')
    expect(row.find('.device-remove').exists()).toBe(true)
  })

  // 在线设备正常切换
  it('DeviceList_OnlineClickSelects_011', async () => {
    const wrapper = mountWithStatuses({ 'b#2|t2': 'online' })
    await wrapper.findAll('.device-item')[1].trigger('click')
    expect(wrapper.emitted('select')?.[0]).toEqual(['b#2'])
  })

  // 「重新检测」向上抛出，由调用方重新探测
  it('DeviceList_RecheckEmits_012', async () => {
    const wrapper = mountWithStatuses({})
    await wrapper.find('.device-recheck').trigger('click')
    expect(wrapper.emitted('recheck')).toHaveLength(1)
  })

  // 当前设备始终以主连接状态为准，探测结果不覆盖
  it('DeviceList_ActiveUsesConnState_013', () => {
    const wrapper = mount(DeviceList, {
      props: {
        desktops,
        activeId: 'a#1',
        connState: 'denied',
        statuses: { 'a#1|t1': 'online' },
      },
    })
    expect(wrapper.findAll('.device-item')[0].find('.tag.status-auth-failed').exists()).toBe(true)
  })

  // 重新检测（checking）期间保持拦截提示：复测仍不可达时不需要再点一次
  it('DeviceList_RecheckKeepsBlockedHint_014', async () => {
    const wrapper = mountWithStatuses({ 'b#2|t2': 'offline' })
    const row = wrapper.findAll('.device-item')[1]

    await row.trigger('click')
    expect(row.find('.device-blocked').exists()).toBe(true)

    await wrapper.setProps({ statuses: { 'b#2|t2': 'checking' } })
    expect(wrapper.findAll('.device-item')[1].find('.device-blocked').exists()).toBe(true)

    // 复测恢复在线 → 收起
    await wrapper.setProps({ statuses: { 'b#2|t2': 'online' } })
    expect(wrapper.findAll('.device-item')[1].find('.device-blocked').exists()).toBe(false)
  })
})
