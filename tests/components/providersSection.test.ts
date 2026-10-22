import { describe, it, expect, vi, beforeEach } from 'vitest'
import { mount } from '@vue/test-utils'
import { defineComponent, h } from 'vue'
import type { Provider } from '@/types/provider'

const toastMock = vi.hoisted(() => ({
  success: vi.fn(),
  info: vi.fn(),
  error: vi.fn(),
  remove: vi.fn(),
  toasts: [],
}))

const mockT = vi.fn((key: string) => key)
vi.mock('vue-i18n', () => ({
  useI18n: () => ({ t: mockT }),
}))

// 隔离 store：只提供模板用到的字段与保存方法
const storeMock = {
  providers: [] as Provider[],
  isLoading: false,
  activeProviderId: null as string | null,
  hasCcSwitchDb: false,
  commonConfig: { enabled: false, settings: {} },
  loadProvidersConfig: vi.fn(),
  create: vi.fn(),
  update: vi.fn(),
  remove: vi.fn(),
  reorder: vi.fn(),
  activate: vi.fn(),
  testConnection: vi.fn(),
  importCcSwitch: vi.fn(),
}

vi.mock('@/stores/providers', () => ({
  useProvidersStore: () => storeMock,
}))

// 子面板（虽被 stub）模块仍会被加载，避免其 app store 拉入真实 i18n
vi.mock('@/stores/app', () => ({
  useAppStore: () => ({ theme: 'dark' }),
}))

// 全局通知：断言调用参数（实时渲染由 ToastHost 测试覆盖）
vi.mock('@/composables/useToast', () => ({
  useToast: () => toastMock,
}))

import ProvidersSection from '@/components/settings/sections/ProvidersSection.vue'

const ProviderListStub = defineComponent({
  name: 'ProviderList',
  props: ['providers', 'activeId', 'testingId'],
  emits: ['edit', 'activate', 'test', 'delete', 'reorder'],
  setup(_, { emit }) {
    return () => h('button', { class: 'stub-edit', onClick: () => emit('edit', provider) }, 'edit')
  },
})

const ProviderEditPanelStub = defineComponent({
  name: 'ProviderEditPanel',
  props: ['provider', 'commonConfig'],
  emits: ['save', 'close', 'openCommonConfig'],
  setup(_, { emit }) {
    return () => h('button', { class: 'stub-save', onClick: () => emit('save', provider) }, 'save')
  },
})

const provider: Provider = {
  id: 'p1',
  name: 'Provider-p1',
  settingsConfig: { env: { ANTHROPIC_BASE_URL: 'https://example.com' } },
  createdAt: Date.now(),
  inFailoverQueue: false,
}

function mountSection() {
  storeMock.providers = [provider]
  return mount(ProvidersSection, {
    global: {
      stubs: {
        ProviderList: ProviderListStub,
        ProviderEditPanel: ProviderEditPanelStub,
        ProviderPresetPanel: true,
        CommonConfigPanel: true,
      },
    },
  })
}

describe('ProvidersSection 全局通知', () => {
  beforeEach(() => {
    vi.clearAllMocks()
    storeMock.update.mockResolvedValue(provider)
    storeMock.create.mockResolvedValue(provider)
    storeMock.hasCcSwitchDb = false
  })

  // 编辑后保存成功：成功 toast 提示需点击激活（不阻断、不自动激活）
  it('ProviderSection_SaveShowsActivateToast_001', async () => {
    const wrapper = mountSection()

    await wrapper.find('.stub-edit').trigger('click')
    await wrapper.find('.stub-save').trigger('click')

    expect(storeMock.update).toHaveBeenCalledWith('p1', expect.objectContaining({ name: 'Provider-p1' }))
    expect(storeMock.activate).not.toHaveBeenCalled()
    expect(toastMock.success).toHaveBeenCalledWith('providerSavedNeedActivate', 5000)
  })

  // 保存失败：错误 toast（此前仅 console.error，用户无感知）
  it('ProviderSection_SaveFailureShowsError_001', async () => {
    storeMock.update.mockRejectedValue(new Error('boom'))
    const wrapper = mountSection()

    await wrapper.find('.stub-edit').trigger('click')
    await wrapper.find('.stub-save').trigger('click')

    expect(toastMock.error).toHaveBeenCalledWith('providerSaveFailed', 6000)
  })

  // 导入成功：多行结果合并为单条成功 toast
  it('ProviderSection_ImportSuccessToast_001', async () => {
    storeMock.hasCcSwitchDb = true
    storeMock.importCcSwitch.mockResolvedValue({
      count: 2,
      importedCommonConfig: true,
      activeProviderName: 'P',
    })
    const wrapper = mountSection()

    await wrapper.findAll('button').find(b => b.text() === 'importFromCcSwitch')!.trigger('click')
    await new Promise(r => setTimeout(r))

    const text = toastMock.success.mock.calls[0][0] as string
    expect(text).toContain('importSuccess')
    expect(text).toContain('commonConfigImported')
    expect(text).toContain('activeProvider')
  })

  // 导入无可导入项：信息 toast
  it('ProviderSection_ImportEmptyToast_001', async () => {
    storeMock.hasCcSwitchDb = true
    storeMock.importCcSwitch.mockResolvedValue({ count: 0 })
    const wrapper = mountSection()

    await wrapper.findAll('button').find(b => b.text() === 'importFromCcSwitch')!.trigger('click')
    await new Promise(r => setTimeout(r))

    expect(toastMock.info).toHaveBeenCalledWith('noProvidersToImport')
  })

  // 导入失败：错误 toast
  it('ProviderSection_ImportFailureToast_001', async () => {
    storeMock.hasCcSwitchDb = true
    storeMock.importCcSwitch.mockRejectedValue(new Error('db locked'))
    const wrapper = mountSection()

    await wrapper.findAll('button').find(b => b.text() === 'importFromCcSwitch')!.trigger('click')
    await new Promise(r => setTimeout(r))

    expect(toastMock.error).toHaveBeenCalledWith('importFailed', 6000)
  })
})
