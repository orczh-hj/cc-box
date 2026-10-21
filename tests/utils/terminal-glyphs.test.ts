import { describe, it, expect } from 'vitest'
import { mapTerminalGlyphs } from '@/utils/terminal-glyphs'

describe('mapTerminalGlyphs 终端符号兜底替换', () => {
  it('TerminalGlyphs_替换ClaudeCLI的⏵⏵提示符为▶▶', () => {
    // 用户报告的具体场景：远程端字体不支持 ⏵⏵（U+23F5）
    expect(mapTerminalGlyphs('\u23F5\u23F5 submit')).toBe('\u25B6\u25B6 submit')
  })

  it('TerminalGlyphs_替换暂停符号⏸为‖', () => {
    expect(mapTerminalGlyphs('\u23F8 pause')).toBe('\u2016 pause')
  })

  it('TerminalGlyphs_覆盖媒体控制符全家族', () => {
    expect(mapTerminalGlyphs('\u23EF')).toBe('\u25B6') // ⏯ → ▶
    expect(mapTerminalGlyphs('\u23F4')).toBe('\u25C0') // ⏴ → ◀
    expect(mapTerminalGlyphs('\u23F6')).toBe('\u25B2') // ⏶ → ▲
    expect(mapTerminalGlyphs('\u23F7')).toBe('\u25BC') // ⏷ → ▼
    expect(mapTerminalGlyphs('\u23F9')).toBe('\u25A0') // ⏹ → ■
    expect(mapTerminalGlyphs('\u23FA')).toBe('\u25CF') // ⏺ → ●
  })

  it('TerminalGlyphs_普通文本与中文原样保留', () => {
    const text = 'hello 世界 → ← ● ✻ ✎ done'
    expect(mapTerminalGlyphs(text)).toBe(text)
  })

  it('TerminalGlyphs_ANSI转义序列与混合内容不受影响', () => {
    // 转义序列中的数字/字母不命中；仅 ⏵ 被替换
    const input = '\x1b[32mok\x1b[0m \u23F5\u23F5 run more'
    expect(mapTerminalGlyphs(input)).toBe('\x1b[32mok\x1b[0m \u25B6\u25B6 run more')
  })

  it('TerminalGlyphs_空字符串与无命中字符串返回原值', () => {
    expect(mapTerminalGlyphs('')).toBe('')
    const plain = 'no special glyphs here'
    expect(mapTerminalGlyphs(plain)).toBe(plain)
  })

  it('TerminalGlyphs_连续调用无lastIndex状态泄漏', () => {
    // /g 正则复用安全：test/replace 不应携带上一次匹配位置
    expect(mapTerminalGlyphs('\u23F5')).toBe('\u25B6')
    expect(mapTerminalGlyphs('plain')).toBe('plain')
    expect(mapTerminalGlyphs('\u23F5\u23F8')).toBe('\u25B6\u2016')
  })
})
