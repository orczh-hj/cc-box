/**
 * Provider Store
 * API Provider 配置管理
 */
import { defineStore } from 'pinia'
import { ref, computed } from 'vue'
import { uuid } from '@/utils/uuid'
import type { Provider, ProvidersConfig, CommonConfig, ImportResult, TestConnectionResult } from '@/types/provider'
import {
  getProvidersConfig,
  saveProvidersConfig,
  activateProvider,
  createProvider,
  updateProvider,
  deleteProvider,
  updateProviderSortOrder,
  updateCommonConfig,
  checkCcSwitchDbExists,
  importFromCcSwitch,
  testProviderConnection
} from '@/api/provider'
import { providerPresets } from '@/config/providerPresets'
import type { ProviderPreset } from '@/types/provider'
import { deepMergeSettings, replaceInSettings } from '@/utils/json'

export const useProvidersStore = defineStore('providers', () => {
  const providers = ref<Provider[]>([])
  const commonConfig = ref<CommonConfig>({
    enabled: false,
    settings: {}
  })
  const activeProviderId = ref<string | null>(null)
  const isLoading = ref(false)
  const hasCcSwitchDb = ref(false)
  const presets = ref<ProviderPreset[]>(providerPresets)

  const activeProvider = computed(() => {
    if (!activeProviderId.value) return null
    return providers.value.find(p => p.id === activeProviderId.value) || null
  })

  const providersByCategory = computed(() => {
    const grouped: Record<string, Provider[]> = {}
    for (const provider of providers.value) {
      const category = provider.category || 'custom'
      if (!grouped[category]) {
        grouped[category] = []
      }
      grouped[category].push(provider)
    }
    return grouped
  })

  async function loadProvidersConfig() {
    isLoading.value = true
    try {
      const config = await getProvidersConfig()
      providers.value = config.providers
      commonConfig.value = config.commonConfig
      activeProviderId.value = config.activeProviderId

      // 检测 cc-switch 数据库
      hasCcSwitchDb.value = await checkCcSwitchDbExists()
    } catch (err) {
      console.error('Failed to load providers config:', err)
    } finally {
      isLoading.value = false
    }
  }

  async function saveConfig() {
    try {
      const config: ProvidersConfig = {
        providers: providers.value,
        commonConfig: commonConfig.value,
        activeProviderId: activeProviderId.value
      }
      await saveProvidersConfig(config)
    } catch (err) {
      console.error('Failed to save providers config:', err)
    }
  }

  async function activate(id: string) {
    try {
      await activateProvider(id)
      activeProviderId.value = id
    } catch (err) {
      console.error('Failed to activate provider:', err)
      throw err
    }
  }

  /** 从预设创建本地 Provider 对象（不写入后端），编辑面板保存时才持久化 */
  function createLocalFromPreset(preset: ProviderPreset, customName?: string): Provider {
    const name = customName || preset.name
    const settingsConfig = JSON.parse(JSON.stringify(preset.settingsConfig))
    const commonConfigEnabled = true

    // 处理 templateValues（替换占位符）
    if (preset.templateValues) {
      for (const [key, valueConfig] of Object.entries(preset.templateValues)) {
        const placeholder = `\${${key}}`
        const value = valueConfig.editorValue || valueConfig.defaultValue || ''
        replaceInSettings(settingsConfig, placeholder, value)
      }
    }

    // 若勾选了应用通用配置，合并通用配置到 settingsConfig
    if (commonConfigEnabled && commonConfig.value.enabled &&
        commonConfig.value.settings && Object.keys(commonConfig.value.settings).length > 0) {
      const merged = deepMergeSettings(settingsConfig, commonConfig.value.settings)
      Object.keys(merged).forEach(k => { settingsConfig[k] = merged[k] })
    }

    return {
      id: uuid(),
      name,
      settingsConfig,
      websiteUrl: preset.websiteUrl,
      category: preset.category,
      createdAt: Date.now(),
      sortIndex: undefined,
      notes: undefined,
      meta: { commonConfigEnabled },
      icon: preset.icon,
      iconColor: preset.iconColor,
      inFailoverQueue: false,
    }
  }

  /** 将新 Provider 持久化到后端，并加入 store */
  async function create(provider: Provider): Promise<Provider> {
    const created = await createProvider(
      provider.name,
      provider.settingsConfig,
      provider.websiteUrl,
      provider.category,
      provider.icon,
      provider.iconColor,
      provider.meta
    )
    providers.value.push(created)
    return created
  }

  async function update(id: string, updates: {
    name?: string
    settingsConfig?: Record<string, any>
    notes?: string
    meta?: { commonConfigEnabled?: boolean }
  }): Promise<Provider> {
    try {
      const provider = await updateProvider(
        id,
        updates.name,
        updates.settingsConfig,
        updates.notes,
        updates.meta
      )

      const index = providers.value.findIndex(p => p.id === id)
      if (index >= 0) {
        providers.value[index] = provider
      }

      return provider
    } catch (err) {
      console.error('Failed to update provider:', err)
      throw err
    }
  }

  async function remove(id: string) {
    try {
      await deleteProvider(id)
      const index = providers.value.findIndex(p => p.id === id)
      if (index >= 0) {
        providers.value.splice(index, 1)
      }
      if (activeProviderId.value === id) {
        activeProviderId.value = null
      }
    } catch (err) {
      console.error('Failed to delete provider:', err)
      throw err
    }
  }

  async function reorder(newOrder: string[]) {
    try {
      await updateProviderSortOrder(newOrder)

      // 更新本地排序
      const reordered: Provider[] = []
      for (const id of newOrder) {
        const provider = providers.value.find(p => p.id === id)
        if (provider) {
          reordered.push(provider)
        }
      }
      // 添加未在 newOrder 中的 Provider
      for (const provider of providers.value) {
        if (!newOrder.includes(provider.id)) {
          reordered.push(provider)
        }
      }
      providers.value = reordered
    } catch (err) {
      console.error('Failed to reorder providers:', err)
    }
  }

  async function updateCommon(settings: Record<string, any>) {
    try {
      await updateCommonConfig(true, settings)
      commonConfig.value = { enabled: true, settings }
      // 后端已批量合并到所有 commonConfigEnabled 的 Provider，重载最新数据
      const config = await getProvidersConfig()
      providers.value = config.providers
      // 重新激活当前活跃的 Provider（settingsConfig 已包含最新通用配置）
      if (activeProviderId.value) {
        await activateProvider(activeProviderId.value)
      }
    } catch (err) {
      console.error('Failed to update common config:', err)
      throw err
    }
  }

  async function importCcSwitch(): Promise<ImportResult> {
    try {
      const result = await importFromCcSwitch()
      if (result.count > 0) {
        await loadProvidersConfig()
      }
      return result
    } catch (err) {
      console.error('Failed to import from cc-switch:', err)
      throw err
    }
  }

  function getPresetByCategory(category?: string): ProviderPreset[] {
    if (!category) return presets.value.filter(p => !p.hidden)
    return presets.value.filter(p => p.category === category && !p.hidden)
  }

  async function testConnection(id: string): Promise<TestConnectionResult> {
    return await testProviderConnection(id)
  }

  return {
    providers,
    commonConfig,
    activeProviderId,
    isLoading,
    hasCcSwitchDb,
    presets,
    activeProvider,
    providersByCategory,
    loadProvidersConfig,
    saveConfig,
    activate,
    createLocalFromPreset,
    create,
    update,
    remove,
    reorder,
    updateCommon,
    importCcSwitch,
    getPresetByCategory,
    testConnection
  }
})