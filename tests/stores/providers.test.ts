import { setActivePinia, createPinia } from 'pinia'
import { mockIPC } from '@tauri-apps/api/mocks'
import { beforeEach, describe, it, expect, vi } from 'vitest'
import { useProvidersStore } from '@/stores/providers'
import type { Provider } from '@/types/provider'

// Mock @/api/provider so remove/reorder do not invoke Tauri commands
vi.mock('@/api/provider', () => ({
  getProvidersConfig: vi.fn(),
  saveProvidersConfig: vi.fn(),
  activateProvider: vi.fn(),
  createProvider: vi.fn(),
  updateProvider: vi.fn(),
  deleteProvider: vi.fn(),
  updateProviderSortOrder: vi.fn(),
  updateCommonConfig: vi.fn(),
  checkCcSwitchDbExists: vi.fn(),
  importFromCcSwitch: vi.fn(),
}))

// Mock @/config/providerPresets (large static data, not needed here)
vi.mock('@/config/providerPresets', () => ({
  providerPresets: [],
}))

// Mock @/utils/json (store 的 deepMergeSettings 已提取到此处)
function mockDeepMerge(target: any, source: any): any {
  if (target !== null && typeof target === 'object' && !Array.isArray(target) &&
      source !== null && typeof source === 'object' && !Array.isArray(source)) {
    const result = { ...target }
    for (const key of Object.keys(source)) {
      result[key] = key in result ? mockDeepMerge(result[key], source[key]) : source[key]
    }
    return result
  }
  return source
}

vi.mock('@/utils/json', () => ({
  deepMergeSettings: mockDeepMerge,
  replaceInSettings: (obj: any, _placeholder: string, _value: string) => obj,
}))

function makeProvider(id: string, category?: string): Provider {
  return {
    id,
    name: `Provider-${id}`,
    settingsConfig: {},
    category,
    createdAt: Date.now(),
    inFailoverQueue: false,
  }
}

beforeEach(() => {
  setActivePinia(createPinia())
  mockIPC(() => {})
})

// ---------- remove ----------

describe('remove', () => {
  // 删除 provider 后列表长度减少 1
  it('ProviderRemove_Decrease_001', async () => {
    const store = useProvidersStore()
    store.providers = [makeProvider('p1'), makeProvider('p2'), makeProvider('p3')]

    await store.remove('p2')

    expect(store.providers).toHaveLength(2)
    expect(store.providers.map(p => p.id)).toEqual(['p1', 'p3'])
  })

  // 删除 activeProviderId 对应的 provider 时 activeProviderId 变为 null
  it('ProviderRemove_ClearActive_001', async () => {
    const store = useProvidersStore()
    store.providers = [makeProvider('p1'), makeProvider('p2')]
    store.activeProviderId = 'p1'

    await store.remove('p1')

    expect(store.activeProviderId).toBeNull()
  })

  // 删除非活跃 provider 时 activeProviderId 不变
  it('ProviderRemove_KeepActive_001', async () => {
    const store = useProvidersStore()
    store.providers = [makeProvider('p1'), makeProvider('p2')]
    store.activeProviderId = 'p1'

    await store.remove('p2')

    expect(store.activeProviderId).toBe('p1')
  })
})

// ---------- reorder ----------

describe('reorder', () => {
  // 设置 newOrder=["id3","id1"] 后 providers 按此顺序排列
  it('ProviderReorder_ByOrder_001', async () => {
    const store = useProvidersStore()
    store.providers = [makeProvider('id1'), makeProvider('id2'), makeProvider('id3')]

    await store.reorder(['id3', 'id1'])

    expect(store.providers.map(p => p.id)).toEqual(['id3', 'id1', 'id2'])
  })

  // newOrder 中未包含的 provider 追加到末尾
  it('ProviderReorder_AppendMissing_001', async () => {
    const store = useProvidersStore()
    store.providers = [makeProvider('a'), makeProvider('b'), makeProvider('c'), makeProvider('d')]

    await store.reorder(['c', 'a'])

    expect(store.providers.map(p => p.id)).toEqual(['c', 'a', 'b', 'd'])
  })

  // 设置空 newOrder 时所有 provider 保持原顺序
  it('ProviderReorder_EmptyOrder_001', async () => {
    const store = useProvidersStore()
    const original = [makeProvider('x'), makeProvider('y'), makeProvider('z')]
    store.providers = [...original]

    await store.reorder([])

    expect(store.providers.map(p => p.id)).toEqual(['x', 'y', 'z'])
  })
})

// ---------- providersByCategory ----------

describe('providersByCategory', () => {
  // 设置 3 个 provider 的 category 为 "official" 时归入同一组
  it('ProviderByCat_Group_001', () => {
    const store = useProvidersStore()
    store.providers = [
      makeProvider('p1', 'official'),
      makeProvider('p2', 'official'),
      makeProvider('p3', 'official'),
    ]

    const grouped = store.providersByCategory

    expect(Object.keys(grouped)).toEqual(['official'])
    expect(grouped['official']).toHaveLength(3)
    expect(grouped['official'].map(p => p.id)).toEqual(['p1', 'p2', 'p3'])
  })

  // category 为 undefined 的 provider 归入 "custom" 组
  it('ProviderByCat_DefaultCustom_001', () => {
    const store = useProvidersStore()
    store.providers = [makeProvider('p1'), makeProvider('p2', 'official')]

    const grouped = store.providersByCategory

    expect(Object.keys(grouped)).toEqual(['custom', 'official'])
    expect(grouped['custom']).toHaveLength(1)
    expect(grouped['custom'][0].id).toBe('p1')
  })

  // providers 为空数组时返回 {}
  it('ProviderByCat_EmptyStore_001', () => {
    const store = useProvidersStore()
    store.providers = []

    const grouped = store.providersByCategory

    expect(grouped).toEqual({})
  })
})

// ---------- 本端写回环抑制（providers section） ----------

describe('providers 写命令回环抑制', () => {
  // activate 成功：登记回环抑制（广播回来时状态中心跳过全量刷新），
  // appliedSettings 用命令返回值精确更新（后端激活会改写托管路径）
  it('Providers_Activate_MarksSelfWrite_001', async () => {
    const { activateProvider } = await import('@/api/provider')
    const applied = { providerId: 'p1', paths: ['/env/ANTHROPIC_BASE_URL'] }
    vi.mocked(activateProvider).mockResolvedValue(applied)

    const { useSidebarStore } = await import('@/stores/sidebar')
    const sidebar = useSidebarStore()
    const store = useProvidersStore()

    await store.activate('p1')

    expect(store.activeProviderId).toBe('p1')
    expect(store.appliedSettings).toEqual(applied)
    expect(sidebar.consumeSelfWrite('providers')).toBe(true)
  })

  // activate 失败：撤销登记（不会有对应广播，避免吞掉其他端事件）
  it('Providers_Activate_FailureCancels_002', async () => {
    const { activateProvider } = await import('@/api/provider')
    vi.mocked(activateProvider).mockRejectedValue(new Error('not found'))

    const { useSidebarStore } = await import('@/stores/sidebar')
    const sidebar = useSidebarStore()
    const store = useProvidersStore()

    await expect(store.activate('p1')).rejects.toThrow('not found')
    expect(sidebar.consumeSelfWrite('providers')).toBe(false)
  })

  // remove 成功：登记回环抑制
  it('Providers_Remove_MarksSelfWrite_003', async () => {
    const { deleteProvider } = await import('@/api/provider')
    vi.mocked(deleteProvider).mockResolvedValue(undefined)

    const { useSidebarStore } = await import('@/stores/sidebar')
    const sidebar = useSidebarStore()
    const store = useProvidersStore()
    store.providers = [makeProvider('p1')]

    await store.remove('p1')
    expect(store.providers).toHaveLength(0)
    expect(sidebar.consumeSelfWrite('providers')).toBe(true)
  })

  // remove 失败：撤销登记
  it('Providers_Remove_FailureCancels_004', async () => {
    const { deleteProvider } = await import('@/api/provider')
    vi.mocked(deleteProvider).mockRejectedValue(new Error('io error'))

    const { useSidebarStore } = await import('@/stores/sidebar')
    const sidebar = useSidebarStore()
    const store = useProvidersStore()
    store.providers = [makeProvider('p1')]

    await expect(store.remove('p1')).rejects.toThrow('io error')
    expect(store.providers).toHaveLength(1)
    expect(sidebar.consumeSelfWrite('providers')).toBe(false)
  })

  // updateCommon：重激活用返回值更新 appliedSettings，不再两次全量拉取；
  // 两次写命令（update_common_config + 重激活）各登记一次回环抑制
  it('Providers_UpdateCommon_NoFullReload_005', async () => {
    const { updateCommonConfig, activateProvider, getProvidersConfig } = await import('@/api/provider')
    vi.mocked(updateCommonConfig).mockResolvedValue(undefined)
    const applied = { providerId: 'p1', paths: ['/x'] }
    vi.mocked(activateProvider).mockResolvedValue(applied)

    const { useSidebarStore } = await import('@/stores/sidebar')
    const sidebar = useSidebarStore()
    const store = useProvidersStore()
    store.activeProviderId = 'p1'

    // settings 里混入 CC 状态字段：本地镜像必须与后端落盘同规则剥离
    await store.updateCommon({ env: { ANTHROPIC_BASE_URL: 'https://x' }, hooks: { k: 1 } })

    expect(getProvidersConfig).not.toHaveBeenCalled()
    expect(store.appliedSettings).toEqual(applied)
    expect(store.commonConfig.settings).toEqual({ env: { ANTHROPIC_BASE_URL: 'https://x' } })
    expect(sidebar.consumeSelfWrite('providers')).toBe(true)
    expect(sidebar.consumeSelfWrite('providers')).toBe(true)
  })
})
