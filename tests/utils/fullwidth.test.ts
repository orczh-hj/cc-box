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

  // 符号：ASCII 0x21-0x7E 区间整体平移 0xFEE0
  it('FullWidth_Symbols_002', () => {
    expect(toFullWidth('!')).toBe('！') // 0x21 → 0xFF01
    expect(toFullWidth(',')).toBe('，') // 0x2C → 0xFF0C
    expect(toFullWidth('.')).toBe('．') // 0x2E → 0xFF0E
    expect(toFullWidth('?')).toBe('？')
    expect(toFullWidth('_')).toBe('＿')
    expect(toFullWidth('~')).toBe('～') // 0x7E → 0xFF5E
    expect(toFullWidth('\\')).toBe('＼')
  })

  // 空格 → 表意空格 U+3000
  it('FullWidth_Space_003', () => {
    expect(toFullWidth(' ')).toBe('　')
    expect(toFullWidth(' ').charCodeAt(0)).toBe(0x3000)
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
