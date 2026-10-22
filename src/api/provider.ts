/**
 * Provider API 函数
 * 封装 Tauri invoke 调用（经 transport 统一层，浏览器远程模式走 rpc.call）
 */
import { transport } from './transport'
import type { Provider, ProvidersConfig, ImportResult, ProviderMeta, TestConnectionResult } from '@/types/provider'

const invoke = <T,>(cmd: string, args?: Record<string, unknown>): Promise<T> =>
  transport.invoke<T>(cmd, args);

/** 获取 Provider 配置 */
export async function getProvidersConfig(): Promise<ProvidersConfig> {
  return invoke('get_providers_config')
}

/**
 * 用系统默认程序打开用户级 Claude settings.json（桌面专属）。
 * Rust 侧直接调用系统 open：JS shell 插件的 open 只允许 URL，本地路径会被 scope 拒绝。
 */
export async function openClaudeUserSettings(): Promise<void> {
  return invoke('open_claude_user_settings')
}

/** 保存 Provider 配置 */
export async function saveProvidersConfig(config: ProvidersConfig): Promise<void> {
  return invoke('save_providers_config', { config })
}

/** 激活 Provider */
export async function activateProvider(providerId: string): Promise<void> {
  return invoke('activate_provider', { providerId })
}

/** 创建 Provider */
export async function createProvider(
  name: string,
  settingsConfig: Record<string, any>,
  websiteUrl?: string,
  category?: string,
  icon?: string,
  iconColor?: string,
  meta?: ProviderMeta
): Promise<Provider> {
  return invoke('create_provider', {
    name,
    settingsConfig,
    websiteUrl,
    category,
    icon,
    iconColor,
    meta
  })
}

/** 更新 Provider */
export async function updateProvider(
  id: string,
  name?: string,
  settingsConfig?: Record<string, any>,
  notes?: string,
  meta?: ProviderMeta
): Promise<Provider> {
  return invoke('update_provider', {
    id,
    name,
    settingsConfig,
    notes,
    meta
  })
}

/** 删除 Provider */
export async function deleteProvider(id: string): Promise<void> {
  return invoke('delete_provider', { id })
}

/** 更新 Provider 排序 */
export async function updateProviderSortOrder(providerIds: string[]): Promise<void> {
  return invoke('update_provider_sort_order', { providerIds })
}

/** 更新通用配置 */
export async function updateCommonConfig(enabled: boolean, settings: Record<string, any>): Promise<void> {
  return invoke('update_common_config', { enabled, settings })
}

/** 检测 cc-switch 数据库是否存在 */
export async function checkCcSwitchDbExists(): Promise<boolean> {
  return invoke('check_cc_switch_db_exists')
}

/** 从 cc-switch 数据库导入 Provider */
export async function importFromCcSwitch(): Promise<ImportResult> {
  return invoke('import_from_cc_switch')
}

/** 测试 Provider 连接 */
export async function testProviderConnection(providerId: string): Promise<TestConnectionResult> {
  return invoke('test_provider_connection', { providerId })
}