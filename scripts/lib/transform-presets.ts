/**
 * cc-switch → CC-Box 预设转换纯函数（无 IO，同步脚本与单测共用）。
 *
 * 策略（2026-10 用户决策）：
 * - 全量同步上游 claudeProviderPresets（仅排除 hidden）
 * - 剥离全部推广跟踪码（URL 查询参数黑名单 + 路径推广段）
 * - 不引入 cc-switch 的赞助商体系（partner 字段）与预设族折叠
 *   （family/planKey/regionKey）——CC-Box 平铺展示，UI 按名称排序
 */
import type { ProviderCategory, ProviderPreset } from '../../src/types/provider'

/** 推广/跟踪用查询参数黑名单（全量剥离，含 utm_* 家族与已知返利码） */
export const PROMO_QUERY_PARAMS = [
  'track_id',
  'aff',
  'aff_id',
  'ic',
  'from',
  'code',
  'source',
  'invite',
  'ac',
  'rc',
  'utm_source',
  'utm_medium',
  'utm_campaign',
  'utm_content',
  'utm_term',
]

/**
 * 路径尾段推广模式：/invite/<code>、/i/<code>（邀请/返利短链段）。
 * 码长度 ≥6 才剥——短的 /i/ab 可能是正常路由，宁漏勿伤
 */
const PROMO_PATH_SEGMENT = /\/(?:invite|i)\/[A-Za-z0-9_-]{6,}$/

/** 剥离 URL 中的推广跟踪码，保留纯净地址 */
export function stripPromoParams(url: string): string {
  if (!url) return url
  let out = url.trim()
  out = out.replace(PROMO_PATH_SEGMENT, '')
  const qIndex = out.indexOf('?')
  if (qIndex === -1) return out
  const base = out.slice(0, qIndex)
  const query = out.slice(qIndex + 1)
  const kept = query.split('&').filter((kv) => {
    if (!kv) return false
    const key = kv.split('=')[0].toLowerCase()
    return !PROMO_QUERY_PARAMS.includes(key)
  })
  return kept.length ? `${base}?${kept.join('&')}` : base
}

/** 上游预设（cc-switch claudeProviderPresets 的 ProviderPreset，宽松结构） */
export interface UpstreamPreset {
  name: string
  nameKey?: string
  websiteUrl: string
  apiKeyUrl?: string
  settingsConfig: Record<string, unknown>
  isOfficial?: boolean
  isPartner?: boolean
  primePartner?: boolean
  partnerPromotionKey?: string
  category?: ProviderCategory
  apiKeyField?: string
  templateValues?: Record<string, UpstreamTemplateValue>
  endpointCandidates?: string[]
  theme?: unknown
  family?: string
  planKey?: string
  regionKey?: string
  icon?: string
  iconColor?: string
  apiFormat?: string
  providerType?: string
  requiresOAuth?: boolean
  hidden?: boolean
  modelsUrl?: string
}

interface UpstreamTemplateValue {
  label?: string
  placeholder?: string
  defaultValue?: string
  editorValue?: string
}

/** CC-Box 自有兜底条目（上游没有，恒在尾部） */
export const CUSTOM_PRESET: ProviderPreset = {
  name: 'Custom Provider',
  nameKey: 'customProvider',
  websiteUrl: '',
  settingsConfig: {
    env: {
      ANTHROPIC_BASE_URL: '',
      ANTHROPIC_AUTH_TOKEN: '',
    },
  },
  category: 'custom',
  icon: 'custom',
  iconColor: '#6366F1',
}

/** 转换单个预设：hidden 排除、字段挑选、推广码剥离；返回 null 表示跳过 */
export function transformPreset(p: UpstreamPreset): ProviderPreset | null {
  if (p.hidden) return null

  const out: ProviderPreset = {
    name: p.name,
    websiteUrl: stripPromoParams(p.websiteUrl || ''),
    settingsConfig: JSON.parse(JSON.stringify(p.settingsConfig ?? { env: {} })),
  }
  if (p.apiKeyUrl) out.apiKeyUrl = stripPromoParams(p.apiKeyUrl)
  if (p.isOfficial) out.isOfficial = true
  if (p.category) out.category = p.category
  if (p.apiKeyField) out.apiKeyField = p.apiKeyField as ProviderPreset['apiKeyField']
  if (p.apiFormat) out.apiFormat = p.apiFormat as ProviderPreset['apiFormat']
  if (p.providerType) out.providerType = p.providerType
  if (p.requiresOAuth) out.requiresOAuth = true
  if (p.modelsUrl) out.modelsUrl = p.modelsUrl
  if (p.icon) out.icon = p.icon
  if (p.iconColor) out.iconColor = p.iconColor
  if (Array.isArray(p.endpointCandidates) && p.endpointCandidates.length > 0) {
    out.endpointCandidates = [...p.endpointCandidates]
  }
  if (p.templateValues) {
    out.templateValues = {}
    for (const [key, v] of Object.entries(p.templateValues)) {
      out.templateValues[key] = {
        label: v.label ?? key,
        placeholder: v.placeholder ?? '',
        // CC-Box 单一 editorValue 语义：取上游 editorValue，缺省回落 defaultValue
        editorValue: v.editorValue ?? v.defaultValue ?? '',
      }
    }
  }
  return out
}

/** 分类展示顺序（custom 恒在尾部由 CUSTOM_PRESET 追加保证） */
export const CATEGORY_ORDER: Record<string, number> = {
  official: 0,
  cn_official: 1,
  cloud_provider: 2,
  aggregator: 3,
  third_party: 4,
  local: 5,
}

/** 转换全部预设：过滤 hidden → 分类分组 + 名称排序 → 追加 Custom */
export function transformAll(upstream: UpstreamPreset[]): ProviderPreset[] {
  const list = upstream
    .map(transformPreset)
    .filter((p): p is ProviderPreset => p !== null)
  list.sort((a, b) => {
    const ca = CATEGORY_ORDER[a.category ?? ''] ?? 99
    const cb = CATEGORY_ORDER[b.category ?? ''] ?? 99
    if (ca !== cb) return ca - cb
    // 显式 zh-CN：默认 locale 随系统变化（zh 拼音序 vs en 结果相反），
    // 会导致不同机器重新生成时顺序漂移、CI（英文 locale）守卫失败
    return a.name.localeCompare(b.name, 'zh-CN')
  })
  list.push({ ...CUSTOM_PRESET })
  return list
}

/** 序列化为项目风格（单引号、尾逗号）的 TS 数组字面量文本 */
export function serializePresets(presets: ProviderPreset[]): string {
  function serValue(v: unknown, indent: number): string {
    const pad = ' '.repeat(indent)
    const padIn = ' '.repeat(indent + 2)
    if (v === null || v === undefined) return 'null'
    if (typeof v === 'string') return `'${v.replace(/\\/g, '\\\\').replace(/'/g, "\\'")}'`
    if (typeof v === 'number' || typeof v === 'boolean') return String(v)
    if (Array.isArray(v)) {
      if (v.length === 0) return '[]'
      const items = v.map((item) => `${padIn}${serValue(item, indent + 2)}`)
      return `[\n${items.join(',\n')},\n${pad}]`
    }
    if (typeof v === 'object') {
      const entries = Object.entries(v as Record<string, unknown>).filter(
        ([, val]) => val !== undefined,
      )
      if (entries.length === 0) return '{}'
      const lines = entries.map(
        ([key, val]) => `${padIn}${/^[A-Za-z_$][A-Za-z0-9_$]*$/.test(key) ? key : `'${key}'`}: ${serValue(val, indent + 2)}`,
      )
      return `{\n${lines.join(',\n')},\n${pad}}`
    }
    return 'null'
  }
  return serValue(presets, 0)
}
