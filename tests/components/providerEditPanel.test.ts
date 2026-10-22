import { describe, it, expect, vi, beforeEach } from 'vitest'
import { mount } from '@vue/test-utils'
import { defineComponent, h, nextTick } from 'vue'
import type { CommonConfig, Provider } from '@/types/provider'

vi.mock('vue-i18n', () => ({
  useI18n: () => ({ t: (key: string) => key }),
}))

vi.mock('@/stores/app', () => ({
  useAppStore: () => ({ theme: 'dark' }),
}))

// CodeMirror 替身：用 textarea 承载 v-model，便于断言编辑器内容
vi.mock('vue-codemirror', () => ({
  Codemirror: defineComponent({
    name: 'Codemirror',
    props: { modelValue: { type: String, default: '' } },
    emits: ['update:modelValue', 'change'],
    setup(props, { emit }) {
      return () => h('textarea', {
        class: 'cm-stub',
        value: props.modelValue,
        onInput: (e: Event) => emit('update:modelValue', (e.target as HTMLTextAreaElement).value),
      })
    },
  }),
}))

import ProviderEditPanel from '@/components/settings/providers/ProviderEditPanel.vue'

const REQUIRED = {
  ANTHROPIC_AUTH_TOKEN: 'sk-test',
  ANTHROPIC_BASE_URL: 'https://api.example.com',
  ANTHROPIC_MODEL: 'model-a',
  ANTHROPIC_DEFAULT_HAIKU_MODEL: 'haiku',
  ANTHROPIC_DEFAULT_SONNET_MODEL: 'sonnet',
  ANTHROPIC_DEFAULT_OPUS_MODEL: 'opus',
}

const commonConfig = {
  enabled: true,
  settings: {
    env: { CLAUDE_CODE_SCROLL_SPEED: '5' },
    effortLevel: 'high',
  },
}

function makeProvider(): Provider {
  return {
    id: 'p1',
    name: 'P1',
    settingsConfig: { env: { ...REQUIRED } },
    meta: { commonConfigEnabled: true, commonConfigRemoved: [] },
    inFailoverQueue: false,
  }
}

async function mountPanel(
  provider = makeProvider(),
  common: CommonConfig = commonConfig
) {
  const wrapper = mount(ProviderEditPanel, {
    props: { provider, commonConfig: common },
  })
  await nextTick()
  return wrapper
}

function editorValue(wrapper: ReturnType<typeof mount>): Record<string, any> {
  return JSON.parse((wrapper.find('textarea.cm-stub').element as HTMLTextAreaElement).value)
}

async function setEditor(wrapper: ReturnType<typeof mount>, value: Record<string, any>) {
  const textarea = wrapper.find('textarea.cm-stub')
  ;(textarea.element as HTMLTextAreaElement).value = JSON.stringify(value, null, 2)
  await textarea.trigger('input')
  await nextTick()
}

describe('ProviderEditPanel 分层合并', () => {
  beforeEach(() => {
    vi.clearAllMocks()
  })

  it('ProviderEditPanel_ShowsEffective_001', async () => {
    const wrapper = await mountPanel()
    const value = editorValue(wrapper)
    expect(value.env.CLAUDE_CODE_SCROLL_SPEED).toBe('5')
    expect(value.effortLevel).toBe('high')
    expect(value.env.ANTHROPIC_AUTH_TOKEN).toBe('sk-test')
  })

  it('ProviderEditPanel_ToggleReversible_001', async () => {
    const wrapper = await mountPanel()

    // 取消勾选 → 隐藏继承值
    await wrapper.find('input.toggle-checkbox').setValue(false)
    await nextTick()
    let value = editorValue(wrapper)
    expect(value.env.CLAUDE_CODE_SCROLL_SPEED).toBeUndefined()
    expect(value.effortLevel).toBeUndefined()

    // 重新勾选 → 继承值恢复
    await wrapper.find('input.toggle-checkbox').setValue(true)
    await nextTick()
    value = editorValue(wrapper)
    expect(value.env.CLAUDE_CODE_SCROLL_SPEED).toBe('5')
    expect(value.effortLevel).toBe('high')
  })

  it('ProviderEditPanel_SaveStoresOverridesOnly_001', async () => {
    const wrapper = await mountPanel()
    await wrapper.find('button.btn-save').trigger('click')

    const saved = wrapper.emitted('save')?.[0]?.[0] as Provider
    // 与通用配置相同的值不落盘
    expect(saved.settingsConfig.effortLevel).toBeUndefined()
    expect(saved.settingsConfig.env.CLAUDE_CODE_SCROLL_SPEED).toBeUndefined()
    // Provider 自有值保留
    expect(saved.settingsConfig.env.ANTHROPIC_AUTH_TOKEN).toBe('sk-test')
    expect(saved.meta?.commonConfigEnabled).toBe(true)
    expect(saved.meta?.commonConfigRemoved).toEqual([])
  })

  it('ProviderEditPanel_SaveRecordsRemovedPath_001', async () => {
    const wrapper = await mountPanel()
    const effective = editorValue(wrapper)
    delete effective.env.CLAUDE_CODE_SCROLL_SPEED
    await setEditor(wrapper, effective)
    await wrapper.find('button.btn-save').trigger('click')

    const saved = wrapper.emitted('save')?.[0]?.[0] as Provider
    expect(saved.meta?.commonConfigRemoved).toEqual(['/env/CLAUDE_CODE_SCROLL_SPEED'])
    // 被删除的继承项不应变成空值覆盖
    expect(saved.settingsConfig.env.CLAUDE_CODE_SCROLL_SPEED).toBeUndefined()
  })

  it('ProviderEditPanel_SaveWithoutCommon_001', async () => {
    const provider = makeProvider()
    provider.meta = { commonConfigEnabled: false, commonConfigRemoved: [] }
    const wrapper = await mountPanel(provider)

    expect(editorValue(wrapper).effortLevel).toBeUndefined()
    await wrapper.find('button.btn-save').trigger('click')

    const saved = wrapper.emitted('save')?.[0]?.[0] as Provider
    expect(saved.settingsConfig.env.ANTHROPIC_AUTH_TOKEN).toBe('sk-test')
    expect(saved.meta?.commonConfigEnabled).toBe(false)
  })
})

describe('ProviderEditPanel 边界与回归', () => {
  beforeEach(() => {
    vi.clearAllMocks()
  })

  // H1：空覆盖层 = 完全继承，不能用默认模板凭空产生空 token 覆盖 common
  it('ProviderEditPanel_EmptyOverridesInheritsCommon_001', async () => {
    const commonWithToken = {
      enabled: true,
      settings: { env: { ANTHROPIC_AUTH_TOKEN: 'sk-common' } },
    }
    const provider = makeProvider()
    provider.settingsConfig = {}
    const wrapper = await mountPanel(provider, commonWithToken)

    // 展示 common 的 token，而不是空模板
    expect(editorValue(wrapper).env.ANTHROPIC_AUTH_TOKEN).toBe('sk-common')

    await wrapper.find('button.btn-save').trigger('click')
    const saved = wrapper.emitted('save')?.[0]?.[0] as Provider
    expect(saved.settingsConfig).toEqual({})
  })

  // H2：缺少必需 env 的 Provider 原样保存，不得新增空字符串覆盖用户 settings.json
  it('ProviderEditPanel_SaveDoesNotInjectEmptyEnv_001', async () => {
    const provider = makeProvider()
    provider.settingsConfig = { env: { ANTHROPIC_AUTH_TOKEN: 'sk-test' } }
    const wrapper = await mountPanel(provider)

    await wrapper.find('button.btn-save').trigger('click')
    const saved = wrapper.emitted('save')?.[0]?.[0] as Provider

    expect(saved.settingsConfig).toEqual({ env: { ANTHROPIC_AUTH_TOKEN: 'sk-test' } })
    expect(saved.settingsConfig.env.ANTHROPIC_MODEL).toBeUndefined()
  })

  // H3：非对象根拒绝保存
  it('ProviderEditPanel_RejectsNonObjectRoot_001', async () => {
    const provider = makeProvider()
    provider.meta = { commonConfigEnabled: false }
    const wrapper = await mountPanel(provider)

    const textarea = wrapper.find('textarea.cm-stub')
    ;(textarea.element as HTMLTextAreaElement).value = '[1,2]'
    await textarea.trigger('input')
    await wrapper.find('button.btn-save').trigger('click')

    expect(wrapper.emitted('save')).toBeUndefined()
    expect(wrapper.find('.json-status').text()).toBe('configRootMustBeObject')
  })

  // M1：取消勾选只是暂停继承，不得清空已记录的屏蔽路径
  it('ProviderEditPanel_UncheckPreservesRemoved_001', async () => {
    const provider = makeProvider()
    provider.settingsConfig = { env: { ...REQUIRED } }
    provider.meta = {
      commonConfigEnabled: true,
      commonConfigRemoved: ['/env/CLAUDE_CODE_SCROLL_SPEED'],
    }
    const wrapper = await mountPanel(provider)

    await wrapper.find('input.toggle-checkbox').setValue(false)
    await nextTick()
    await wrapper.find('button.btn-save').trigger('click')

    const saved = wrapper.emitted('save')?.[0]?.[0] as Provider
    expect(saved.meta?.commonConfigEnabled).toBe(false)
    expect(saved.meta?.commonConfigRemoved).toEqual(['/env/CLAUDE_CODE_SCROLL_SPEED'])
  })

  // M5：common 外部被清空后，非 dirty 面板应回到覆盖层视图，避免把旧继承值冻结成覆盖
  it('ProviderEditPanel_CommonBecomesInactive_001', async () => {
    const wrapper = await mountPanel()
    expect(editorValue(wrapper).effortLevel).toBe('high')

    await wrapper.setProps({ commonConfig: { enabled: true, settings: {} } })
    await nextTick()

    const value = editorValue(wrapper)
    expect(value.effortLevel).toBeUndefined()
    expect(value.env.CLAUDE_CODE_SCROLL_SPEED).toBeUndefined()

    await wrapper.find('button.btn-save').trigger('click')
    const saved = wrapper.emitted('save')?.[0]?.[0] as Provider
    expect(saved.settingsConfig.env.CLAUDE_CODE_SCROLL_SPEED).toBeUndefined()
    expect(saved.settingsConfig.effortLevel).toBeUndefined()
  })
})

describe('ProviderEditPanel 空 env 与并发状态', () => {
  beforeEach(() => {
    vi.clearAllMocks()
  })

  // 预设空 token 不得覆盖 common 的有效 token，也不得落盘空串
  it('ProviderEditPanel_StripsEmptyCoreEnv_001', async () => {
    const commonWithToken = {
      enabled: true,
      settings: { env: { ANTHROPIC_AUTH_TOKEN: 'sk-common' } },
    }
    const provider = makeProvider()
    provider.settingsConfig = { env: { ANTHROPIC_AUTH_TOKEN: '', CUSTOM: 'x' } }
    const wrapper = await mountPanel(provider, commonWithToken)

    expect(editorValue(wrapper).env.ANTHROPIC_AUTH_TOKEN).toBe('sk-common')

    await wrapper.find('button.btn-save').trigger('click')
    const saved = wrapper.emitted('save')?.[0]?.[0] as Provider
    expect(saved.settingsConfig.env.ANTHROPIC_AUTH_TOKEN).toBeUndefined()
    expect(saved.settingsConfig.env.CUSTOM).toBe('x')
  })

  // 缺失的必需 env 仍要展示输入行（空值不写入 JSON）
  it('ProviderEditPanel_ShowsRequiredEnvRows_001', async () => {
    const provider = makeProvider()
    provider.settingsConfig = { env: {} }
    const wrapper = await mountPanel(provider, { enabled: false, settings: {} })

    expect(wrapper.findAll('.env-key').length).toBe(6)
  })

  // dirty 期间 common 变为 inactive：保存仍按打开时的 common 快照差分，
  // 不得把旧继承值冻结成覆盖层
  it('ProviderEditPanel_DirtyCommonInactive_001', async () => {
    const wrapper = await mountPanel()
    const edited = editorValue(wrapper)
    edited.env.ANTHROPIC_MODEL = 'edited-model'
    await setEditor(wrapper, edited)

    await wrapper.setProps({ commonConfig: { enabled: true, settings: {} } })
    await nextTick()
    await wrapper.find('button.btn-save').trigger('click')

    const saved = wrapper.emitted('save')?.[0]?.[0] as Provider
    expect(saved.settingsConfig.env.ANTHROPIC_MODEL).toBe('edited-model')
    expect(saved.settingsConfig.env.CLAUDE_CODE_SCROLL_SPEED).toBeUndefined()
    expect(saved.settingsConfig.effortLevel).toBeUndefined()
  })
})
