import { describe, it, expect } from 'vitest'
import { lightTheme, darkTheme } from '@/utils/terminal-theme'

/** WCAG 2.x 相对亮度 */
function relativeLuminance(hex: string): number {
  const [r, g, b] = (hex.replace('#', '').match(/../g) ?? []).map(
    part => parseInt(part, 16) / 255
  )
  const linear = (c: number) => (c <= 0.03928 ? c / 12.92 : Math.pow((c + 0.055) / 1.055, 2.4))
  return 0.2126 * linear(r) + 0.7152 * linear(g) + 0.0722 * linear(b)
}

/** WCAG 对比度（1:1 ~ 21:1） */
function contrastRatio(a: string, b: string): number {
  const l1 = relativeLuminance(a)
  const l2 = relativeLuminance(b)
  const [hi, lo] = l1 > l2 ? [l1, l2] : [l2, l1]
  return (hi + 0.05) / (lo + 0.05)
}

describe('终端主题可读性', () => {
  // 回归：Win10 ConPTY 把「bold + 默认前景」重编码为 SGR 97（brightWhite）、
  // 把反色光标块重编码为 SGR 47（white 背景）。白槽位贴近纯白时，浅色主题下
  // 粗体与光标块消失。两个槽位必须与背景保持至少 4.5:1 对比度（WCAG AA）。
  it('TerminalTheme_LightAnsiWhite_IsReadable_001', () => {
    expect(contrastRatio(lightTheme.white, lightTheme.background)).toBeGreaterThanOrEqual(4.5)
    expect(contrastRatio(lightTheme.brightWhite, lightTheme.background)).toBeGreaterThanOrEqual(4.5)
  })

  // 深色主题的白槽位保持原样，不能被浅色修复影响
  it('TerminalTheme_DarkAnsiWhite_IsReadable_002', () => {
    expect(contrastRatio(darkTheme.white, darkTheme.background)).toBeGreaterThanOrEqual(4.5)
    expect(contrastRatio(darkTheme.brightWhite, darkTheme.background)).toBeGreaterThanOrEqual(4.5)
  })
})
