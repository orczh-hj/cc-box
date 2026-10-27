/**
 * SessionList 分组渲染测试（多实例远程）
 *
 * 多实例：tab 按 instanceName 分组（组头可折叠）；历史会话不参与分组。
 * 降级：全部无实例字段（旧桌面/单实例）→ 无组头平铺（与现状一致）。
 */
import { describe, it, expect, vi } from 'vitest'
import { mount } from '@vue/test-utils'

const mockT = vi.fn((key: string) => key)
vi.mock('vue-i18n', () => ({
  useI18n: () => ({ t: mockT }),
}))

import SessionList from '@/components/sessions/SessionList.vue'
import type { TerminalTab, HistorySession } from '@/stores/session'

function tab(over: Partial<TerminalTab>): TerminalTab {
  return {
    tabId: 't1',
    projectPath: '/p',
    ptyId: null,
    sessionId: null,
    sessionIds: [],
    name: 'proj',
    status: 'running',
    createdAt: 1,
    lastActiveAt: 1,
    working: false,
    pending: false,
    isResume: false,
    ...over,
  }
}

const hist: HistorySession = {
  sessionId: 'sess-h1',
  name: '历史会话',
  projectPath: '/p',
  lastActiveAt: 1,
}

describe('SessionList 多实例分组渲染', () => {
  it('SessionList_GroupedRender_多实例显示组头_001', () => {
    const wrapper = mount(SessionList, {
      props: {
        tabs: [
          tab({ tabId: 't1', ptyId: 'p1', instanceId: 'inst-1', instanceName: '实例 1' }),
          tab({ tabId: 't2', ptyId: 'p2', instanceId: 'inst-2', instanceName: '实例 2' }),
          tab({ tabId: 't3', ptyId: 'p3', instanceId: 'inst-2', instanceName: '实例 2' }),
        ],
        activeId: null,
      },
    })
    const headers = wrapper.findAll('.group-header')
    expect(headers).toHaveLength(2)
    expect(headers[0].text()).toContain('实例 1')
    expect(headers[1].text()).toContain('实例 2')
    // 全部 tab 条目渲染
    expect(wrapper.findAllComponents({ name: 'SessionItem' }).length).toBe(3)
  })

  it('SessionList_DegradedRender_无实例字段平铺_002', () => {
    const wrapper = mount(SessionList, {
      props: {
        tabs: [
          tab({ tabId: 't1', ptyId: 'p1' }),
          tab({ tabId: 't2', ptyId: 'p2' }),
        ],
        history: [hist],
        activeId: null,
      },
    })
    // 降级：无组头，全部条目（含历史）平铺
    expect(wrapper.findAll('.group-header')).toHaveLength(0)
    expect(wrapper.findAllComponents({ name: 'SessionItem' }).length).toBe(3)
  })

  it('SessionList_SingleGroup_单实例全部平铺不丢条目_005', () => {
    // 单实例 Gateway：会话全带同一 instanceId——不显示组头且全部条目可见
    //（若单组走分组渲染分支会整组丢失，此处回归防护）
    const wrapper = mount(SessionList, {
      props: {
        tabs: [
          tab({ tabId: 't1', ptyId: 'p1', instanceId: 'inst-1', instanceName: '实例 1' }),
          tab({ tabId: 't2', ptyId: 'p2', instanceId: 'inst-1', instanceName: '实例 1' }),
        ],
        history: [hist],
        activeId: null,
      },
    })
    expect(wrapper.findAll('.group-header')).toHaveLength(0)
    const items = wrapper.findAllComponents({ name: 'SessionItem' })
    expect(items.length).toBe(3)
    const names = items.map(i => i.props('name'))
    expect(names).toContain('proj')
    expect(names).toContain('历史会话')
  })

  it('SessionList_HistoryNotGrouped_历史不参与分组_003', () => {
    const wrapper = mount(SessionList, {
      props: {
        tabs: [
          tab({ tabId: 't1', ptyId: 'p1', instanceId: 'inst-1', instanceName: '实例 1' }),
          tab({ tabId: 't2', ptyId: 'p2', instanceId: 'inst-2', instanceName: '实例 2' }),
        ],
        history: [hist],
        activeId: null,
      },
    })
    // 两组 + 历史条目平铺在外（不属任何组）
    expect(wrapper.findAll('.group-header')).toHaveLength(2)
    const items = wrapper.findAllComponents({ name: 'SessionItem' })
    expect(items.length).toBe(3)
    // 历史条目在组外容器（session-list 直接子级遍历 flatItems）
    const names = items.map(i => i.props('name'))
    expect(names).toContain('历史会话')
  })

  it('SessionList_GroupCollapse_组头点击折叠_004', async () => {
    const wrapper = mount(SessionList, {
      props: {
        tabs: [
          tab({ tabId: 't1', ptyId: 'p1', instanceId: 'inst-1', instanceName: '实例 1' }),
          tab({ tabId: 't2', ptyId: 'p2', instanceId: 'inst-2', instanceName: '实例 2' }),
        ],
        activeId: null,
      },
    })
    expect(wrapper.findAllComponents({ name: 'SessionItem' }).length).toBe(2)
    // 折叠实例 1 组：该组条目隐藏，实例 2 组不受影响
    await wrapper.findAll('.group-header')[0].trigger('click')
    expect(wrapper.findAllComponents({ name: 'SessionItem' }).length).toBe(1)
    // 再点恢复
    await wrapper.findAll('.group-header')[0].trigger('click')
    expect(wrapper.findAllComponents({ name: 'SessionItem' }).length).toBe(2)
  })
})
