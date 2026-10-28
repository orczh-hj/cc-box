import { describe, it, expect } from 'vitest'
import en from '@/i18n/locales/en'
import zh from '@/i18n/locales/zh'

/** 获取对象的全部顶层 key */
function getKeys(obj: Record<string, unknown>): string[] {
  return Object.keys(obj)
}

describe('i18n 翻译文件一致性', () => {
  // en 和 zh 的 key 结构完全一致
  it('I18n_KeysMatch_001', () => {
    const enKeys = getKeys(en).sort()
    const zhKeys = getKeys(zh).sort()
    expect(enKeys).toEqual(zhKeys)
  })

  // 没有空值
  it('I18n_NoEmptyValues_001', () => {
    for (const [key, value] of Object.entries(en)) {
      expect(value, `en.${key} should not be empty`).toBeTruthy()
    }
    for (const [key, value] of Object.entries(zh)) {
      expect(value, `zh.${key} should not be empty`).toBeTruthy()
    }
  })

  // 插值占位符在两个 locale 中匹配
  it('I18n_InterpolationMatch_001', () => {
    const placeholderPattern = /\{(\w+)\}/g
    for (const key of getKeys(en)) {
      const enPlaceholders = [...(en[key] as string).matchAll(placeholderPattern)].map(m => m[1]).sort()
      const zhPlaceholders = [...(zh[key] as string).matchAll(placeholderPattern)].map(m => m[1]).sort()
      expect(zhPlaceholders, `zh.${key} placeholders should match en`).toEqual(enPlaceholders)
    }
  })
})
