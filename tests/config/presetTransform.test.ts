import { describe, it, expect } from 'vitest'
import {
  stripPromoParams,
  transformPreset,
  transformAll,
  serializePresets,
  CUSTOM_PRESET,
} from '../../scripts/lib/transform-presets'

describe('stripPromoParams（推广跟踪码剥离）', () => {
  // cc-switch Kimi 的 track_id + aff 返利码
  it('PresetTransform_StripPromo_001', () => {
    expect(
      stripPromoParams('https://platform.kimi.com?track_id=track-7cf2b91d&aff=cc-switch'),
    ).toBe('https://platform.kimi.com')
  })

  // 火山 utm 全家桶 + ac/rc 返利码；保留无推广参数的纯净地址
  it('PresetTransform_StripPromo_002', () => {
    expect(
      stripPromoParams(
        'https://www.volcengine.com/activity/codingplan?ac=MMAP8JTTC&rc=6J6FV5&utm_campaign=hw&utm_source=OWO',
      ),
    ).toBe('https://www.volcengine.com/activity/codingplan')
  })

  // 智谱 ic 推广码 / 算力云 from 码
  it('PresetTransform_StripPromo_003', () => {
    expect(stripPromoParams('https://www.bigmodel.cn/claude-code?ic=RRVJPB5SII')).toBe(
      'https://www.bigmodel.cn/claude-code',
    )
    expect(stripPromoParams('https://www.shengsuanyun.com/?from=CH_4HHXMRYF')).toBe(
      'https://www.shengsuanyun.com/',
    )
  })

  // 路径推广段：/invite/CC-SWITCH、/i/drGuwc9k（邀请短链）
  it('PresetTransform_StripPromoPath_004', () => {
    expect(stripPromoParams('https://aigocode.net/invite/CC-SWITCH')).toBe('https://aigocode.net')
    expect(stripPromoParams('https://cloud.siliconflow.cn/i/drGuwc9k')).toBe(
      'https://cloud.siliconflow.cn',
    )
  })

  // 非推广参数保留；短推广段（<6 字符）不剥——宁漏勿伤
  it('PresetTransform_KeepNonPromo_005', () => {
    expect(stripPromoParams('https://a.com/settings?tab=keys&lang=en')).toBe(
      'https://a.com/settings?tab=keys&lang=en',
    )
    expect(stripPromoParams('https://a.com/i/ab')).toBe('https://a.com/i/ab')
  })

  // 无参数/空串原样返回
  it('PresetTransform_NoParams_006', () => {
    expect(stripPromoParams('https://open.bigmodel.cn')).toBe('https://open.bigmodel.cn')
    expect(stripPromoParams('')).toBe('')
  })
})

describe('transformPreset（单条转换）', () => {
  // hidden 条目排除（上游 Mistral AI 隐藏条目）
  it('PresetTransform_SkipHidden_001', () => {
    expect(transformPreset({ name: 'X', websiteUrl: '', settingsConfig: {}, hidden: true })).toBeNull()
  })

  // partner 体系与族折叠字段全部剔除；推广码剥离；正常字段保留
  it('PresetTransform_DropPartnerFields_002', () => {
    const out = transformPreset({
      name: 'Kimi',
      family: 'kimi',
      planKey: 'payg',
      regionKey: 'cn',
      primePartner: true,
      isPartner: true,
      partnerPromotionKey: 'kimi',
      theme: { backgroundColor: '#f64551' },
      nameKey: 'preset.kimi',
      websiteUrl: 'https://platform.kimi.com?aff=cc-switch',
      category: 'cn_official',
      icon: 'kimi',
      iconColor: '#6366F1',
      settingsConfig: { env: { ANTHROPIC_BASE_URL: 'https://api.moonshot.cn/anthropic' } },
    })
    expect(out).not.toBeNull()
    expect(out!.websiteUrl).toBe('https://platform.kimi.com')
    expect(out!.category).toBe('cn_official')
    expect(out!.icon).toBe('kimi')
    // 剔除的字段不得出现在输出
    for (const dropped of ['isPartner', 'partnerPromotionKey', 'theme', 'family', 'planKey', 'regionKey', 'nameKey']) {
      expect(out!).not.toHaveProperty(dropped)
    }
  })

  // templateValues：editorValue 缺省时回落 defaultValue
  it('PresetTransform_TemplateValueFallback_003', () => {
    const out = transformPreset({
      name: 'AWS',
      websiteUrl: 'https://aws.amazon.com/bedrock/',
      settingsConfig: {},
      templateValues: {
        AWS_REGION: { label: 'Region', placeholder: 'us-west-2', defaultValue: 'us-west-2', editorValue: 'us-west-2' },
        AWS_KEY: { label: 'Key', placeholder: 'k', defaultValue: 'fallback' },
      },
    })
    expect(out!.templateValues!.AWS_REGION.editorValue).toBe('us-west-2')
    expect(out!.templateValues!.AWS_KEY.editorValue).toBe('fallback')
  })

  // settingsConfig 深拷贝（改输出不影响输入对象）
  it('PresetTransform_DeepCopy_004', () => {
    const input = { env: { ANTHROPIC_BASE_URL: 'https://a.com' } }
    const out = transformPreset({ name: 'X', websiteUrl: '', settingsConfig: input })
    ;(out!.settingsConfig.env as Record<string, string>).ANTHROPIC_BASE_URL = 'https://mutated.com'
    expect(input.env.ANTHROPIC_BASE_URL).toBe('https://a.com')
  })
})

describe('transformAll（全量转换）', () => {
  const mk = (name: string, category: string) => ({
    name,
    websiteUrl: '',
    settingsConfig: {},
    category: category as never,
  })

  // 分类分组排序（official → … → local）→ 组内名称排序 → Custom 恒在尾部
  it('PresetTransform_SortAndCustomTail_001', () => {
    const out = transformAll([
      mk('Zed', 'local'),
      mk('BaiLing', 'cn_official'),
      mk('AiHub', 'aggregator'),
      mk('DeepSeek', 'cn_official'),
      { ...mk('HiddenOne', 'official'), hidden: true },
    ])
    expect(out.map((p) => p.name)).toEqual(['BaiLing', 'DeepSeek', 'AiHub', 'Zed', 'Custom Provider'])
    expect(out[out.length - 1]).toEqual(CUSTOM_PRESET)
  })
})

describe('serializePresets（TS 字面量序列化）', () => {
  // 单引号风格、转义、数组合法（生成物可被 TS 解析的最小锚定）
  it('PresetTransform_Serialize_001', () => {
    const text = serializePresets([
      {
        name: "It's",
        websiteUrl: 'https://a.com',
        settingsConfig: { env: {} },
        endpointCandidates: ['https://a.com'],
      },
    ])
    expect(text).toContain("'It\\'s'")
    expect(text).toContain('endpointCandidates: [')
    expect(text).toContain("'https://a.com'")
  })
})
