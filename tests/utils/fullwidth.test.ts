import { describe, it, expect } from 'vitest'
import { toFullWidth, isPrintableAscii } from '@/utils/fullwidth'

/**
 * 半角 → 全角映射。期望值来自 Unicode Halfwidth and Fullwidth Forms 块规范
 * （U+FF01-FF5E = ASCII 0x21-0x7E + 0xFEE0），独立于实现。
 */
describe('toFullWidth（半角 → 全角）', () => {
  // 字母与数字
  it('FullWidth_LettersDigits_001', () => {
    expect(toFullWidth('a')).toBe('ａ') // U+FF41，保持小写
    expect(toFullWidth('z')).toBe('ｚ')
    expect(toFullWidth('A')).toBe('Ａ') // U+FF21
    expect(toFullWidth('0')).toBe('０')
    expect(toFullWidth('9')).toBe('９')
  })

  // 符号：ASCII 0x21-0x7E 区间整体平移 0xFEE0（中文标点覆盖见 _005）
  it('FullWidth_Symbols_002', () => {
    expect(toFullWidth('!')).toBe('！') // 0x21 → 0xFF01
    expect(toFullWidth('?')).toBe('？')
    expect(toFullWidth('(')).toBe('（')
    expect(toFullWidth(')')).toBe('）')
  })

  // 空格 → 表意空格 U+3000
  it('FullWidth_Space_003', () => {
    expect(toFullWidth(' ')).toBe('　')
    expect(toFullWidth(' ').charCodeAt(0)).toBe(0x3000)
  })

  // 中文标点惯例（用户实测中文输入法行为，优先于 +0xFEE0 平移）：
  // ~！@#￥%……&*（）——+{}|：“《》？
  it('FullWidth_CjkPunctuation_005', () => {
    // 有专用中文形式的键
    expect(toFullWidth(',')).toBe('，') // U+FF0C（平移恰好一致）
    expect(toFullWidth('.')).toBe('。') // U+3002 句号，非全角句点 ．
    expect(toFullWidth('\\')).toBe('、') // U+3001 顿号
    expect(toFullWidth('[')).toBe('【') // U+3010
    expect(toFullWidth(']')).toBe('】') // U+3011
    expect(toFullWidth(';')).toBe('；')
    expect(toFullWidth("'")).toBe('‘') // U+2018 开单引号（与 “ 对称）
    expect(toFullWidth('"')).toBe('“') // U+201C 开双引号
    expect(toFullWidth('$')).toBe('￥') // U+FFE5 人民币符，非全角 ＄
    expect(toFullWidth('^')).toBe('……') // U+2026 省略号
    expect(toFullWidth('_')).toBe('——') // U+2014×2 破折号
    expect(toFullWidth('<')).toBe('《') // U+300A 书名号（Shift+,）
    expect(toFullWidth('>')).toBe('》') // U+300B（Shift+.）
    // 中文输入法下保持半角的键（不转全角）
    for (const half of '~@#%&*+{}|') {
      expect(toFullWidth(half)).toBe(half)
    }
  })

  // 控制字符 / 非 ASCII 原样返回（转义序列不能被全角化）
  it('FullWidth_NonPrintableUntouched_004', () => {
    expect(toFullWidth('\x03')).toBe('\x03')
    expect(toFullWidth('\x1b')).toBe('\x1b')
    expect(toFullWidth('中')).toBe('中')
  })
})

describe('isPrintableAscii（决定是否转换）', () => {
  it('PrintableAscii_Detection_001', () => {
    expect(isPrintableAscii('a')).toBe(true)
    expect(isPrintableAscii('!')).toBe(true)
    expect(isPrintableAscii(' ')).toBe(true)
    expect(isPrintableAscii('\x03')).toBe(false) // 控制字符
    expect(isPrintableAscii('\x1b[A')).toBe(false) // 转义序列
    expect(isPrintableAscii('中')).toBe(false)
    expect(isPrintableAscii('')).toBe(false)
  })
})
