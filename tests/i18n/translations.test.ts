import { describe, it, expect } from 'vitest'
import { baseCompile } from '@intlify/message-compiler'
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

  // 每条文案都能通过 vue-i18n 消息编译器：字面 { } 会抛 SyntaxError 且在渲染时
  // 打崩所在组件（曾致 MCP 新建弹窗、Hook JSON 模式整窗消失），必须用 {'{'} 转义
  it('I18n_MessageCompile_001', () => {
    const locales: Array<[string, Record<string, unknown>]> = [['zh', zh], ['en', en]]
    for (const [name, messages] of locales) {
      for (const [key, value] of Object.entries(messages)) {
        const errors: string[] = []
        baseCompile(value as string, { onError: e => errors.push(e.message) })
        expect(errors, `${name}.${key}: ${String(value)}`).toEqual([])
      }
    }
  })
})
