import { describe, it, expect } from 'vitest'
import { providerPresets } from '@/config/providerPresets'
import meta from '@/config/providerPresets.meta.json'

const VALID_CATEGORIES = [
  'official',
  'cn_official',
  'cloud_provider',
  'aggregator',
  'third_party',
  'local',
  'custom',
]

/** 推广跟踪码特征（生成时已剥离，出现即回归） */
const PROMO_PATTERNS = /[?&](track_id|aff|aff_id|ic|from|code|source|invite|ac|rc|utm_[a-z_]+)=/i

describe('providerPresets 生成物守卫（scripts/sync-provider-presets.mts 产出）', () => {
  // 每项必有 name/category/settingsConfig；Custom 之外必有 BASE_URL
  it('ProviderPresetsGuard_RequiredFields_001', () => {
    expect(providerPresets.length).toBeGreaterThan(30)
    for (const p of providerPresets) {
      expect(p.name, `name 缺失: ${JSON.stringify(p).slice(0, 80)}`).toBeTruthy()
      expect(p.category, `${p.name} category 缺失`).toBeTruthy()
      expect(p.settingsConfig, `${p.name} settingsConfig 缺失`).toBeTruthy()
      if (p.category !== 'custom' && !p.isOfficial) {
        expect(
          (p.settingsConfig.env as Record<string, unknown>)?.ANTHROPIC_BASE_URL,
          `${p.name} 缺 ANTHROPIC_BASE_URL`,
        ).toBeTruthy()
      }
    }
  })

  // category 必须在合法枚举内（含 cc-switch 新增的 local）
  it('ProviderPresetsGuard_CategoryEnum_002', () => {
    for (const p of providerPresets) {
      expect(VALID_CATEGORIES, `${p.name} 非法 category: ${p.category}`).toContain(p.category)
    }
    // 本机推理分类已随上游引入
    expect(providerPresets.some((p) => p.category === 'local')).toBe(true)
  })

  // 赞助体系与族折叠字段不得出现（生成策略锚定）
  it('ProviderPresetsGuard_NoPartnerFields_003', () => {
    for (const p of providerPresets) {
      for (const banned of ['isPartner', 'partnerPromotionKey', 'primePartner', 'theme', 'family', 'planKey', 'regionKey']) {
        expect(p, `${p.name} 残留 ${banned}`).not.toHaveProperty(banned)
      }
    }
  })

  // URL 无推广跟踪码残留
  it('ProviderPresetsGuard_NoPromoParams_004', () => {
    for (const p of providerPresets) {
      for (const url of [p.websiteUrl, p.apiKeyUrl]) {
        if (url) expect(PROMO_PATTERNS.test(url), `${p.name} 推广码残留: ${url}`).toBe(false)
      }
    }
  })

  // Custom Provider 唯一且恒在尾部；nameKey 供 i18n
  it('ProviderPresetsGuard_CustomTail_005', () => {
    const customs = providerPresets.filter((p) => p.category === 'custom')
    expect(customs.length).toBe(1)
    expect(providerPresets[providerPresets.length - 1].name).toBe('Custom Provider')
    expect(customs[0].nameKey).toBe('customProvider')
  })

  // 数量与 meta.json 一致（生成物与 meta 同步产出的锚定）
  it('ProviderPresetsGuard_MetaCount_006', () => {
    expect(providerPresets.length).toBe(meta.count)
  })

  // 分类分组 + 组内名称排序（生成排序策略锚定；Custom 在 custom 组不参与）
  it('ProviderPresetsGuard_SortedInCategory_007', () => {
    const order: Record<string, number> = {
      official: 0,
      cn_official: 1,
      cloud_provider: 2,
      aggregator: 3,
      third_party: 4,
      local: 5,
      custom: 6,
    }
    for (let i = 1; i < providerPresets.length - 1; i++) {
      const a = providerPresets[i - 1]
      const b = providerPresets[i]
      const ca = order[a.category ?? ''] ?? 99
      const cb = order[b.category ?? ''] ?? 99
      expect(
        ca < cb || (ca === cb && a.name.localeCompare(b.name) <= 0),
        `排序异常: ${a.name} 应先于 ${b.name}`,
      ).toBe(true)
    }
  })

  // 上游主流厂商在场（同步完整性的抽样锚定）
  it('ProviderPresetsGuard_KeyVendors_008', () => {
    const names = new Set(providerPresets.map((p) => p.name))
    for (const vendor of ['Claude Official', 'DeepSeek', 'Zhipu GLM', 'Kimi', 'MiniMax', 'Ollama', 'LM Studio', 'xAI (Grok)', 'AWS Bedrock (AKSK)']) {
      expect(names.has(vendor), `缺少关键厂商: ${vendor}`).toBe(true)
    }
  })
})
