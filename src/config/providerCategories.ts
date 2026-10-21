/**
 * Provider 分类辅助（从 providerPresets.ts 迁出——该文件为同步脚本生成物，
 * 只导出预设数组；本文件承载手写的分类展示逻辑）
 */
import type { ProviderPreset, ProviderCategory } from '@/types/provider'

/** 获取分类标签（i18n 之外的兜底，正常展示走 ProviderPresetPanel 的 i18n） */
export function getCategoryLabel(category?: ProviderCategory): string {
  if (!category) return ''
  const labels: Record<ProviderCategory, string> = {
    official: 'Official',
    cn_official: 'CN',
    cloud_provider: 'Cloud',
    aggregator: 'Aggregator',
    third_party: 'Third-party',
    local: 'Local',
    custom: 'Custom',
    omo: 'OMO',
    'omo-slim': 'OMO Slim',
  }
  return labels[category] || category
}

/** 按分类筛选预设 */
export function filterPresetsByCategory(presets: ProviderPreset[], category?: ProviderCategory): ProviderPreset[] {
  if (!category) return presets.filter(p => !p.hidden)
  return presets.filter(p => p.category === category && !p.hidden)
}
