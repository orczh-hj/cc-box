import { describe, it, expect } from 'vitest'
import { shouldInterceptImeInput } from '@/utils/ime-paste'

/** 构造 InputEvent 形态（jsdom 无法 dispatch 带 data 的 InputEvent，直接传同构对象） */
function ev(partial: Partial<InputEvent> & { data?: string | null }): InputEvent {
  return {
    inputType: 'insertText',
    isComposing: false,
    data: null,
    ...partial,
  } as InputEvent
}

describe('shouldInterceptImeInput 原生键盘粘贴拦截判定', () => {
  // 普通逐字符输入：不拦截（xterm 正常处理，拦截会打断输入）
  it('ImePaste_NormalTyping_001', () => {
    expect(shouldInterceptImeInput(ev({ data: 'a' }))).toBe(false)
    expect(shouldInterceptImeInput(ev({ data: '你好' }))).toBe(false)
  })

  // 带换行的 insertText（Android 键盘整段粘贴）：拦截改走 term.paste
  // （真实 bug：xterm 原样发 PTY，\n 不归一、无括号粘贴 → 换行丢失）
  it('ImePaste_InsertTextWithNewline_002', () => {
    expect(shouldInterceptImeInput(ev({ data: 'line1\nline2\n' }))).toBe(true)
  })

  // insertFromPaste（长按菜单/键盘剪贴板粘贴）：拦截
  // （真实 bug：xterm _inputEvent 只认 insertText，此形态被整体忽略 → 粘贴无反应）
  it('ImePaste_InsertFromPaste_003', () => {
    expect(shouldInterceptImeInput(ev({ inputType: 'insertFromPaste', data: 'pasted' }))).toBe(true)
    expect(shouldInterceptImeInput(ev({ inputType: 'insertReplacementText', data: 'replaced' }))).toBe(true)
  })

  // 输入法组合中（拼音上屏过程）：不拦截
  it('ImePaste_Composing_004', () => {
    expect(shouldInterceptImeInput(ev({ data: 'a\nb', isComposing: true }))).toBe(false)
  })

  // 无 data（删除等操作）：不拦截
  it('ImePaste_NoData_005', () => {
    expect(shouldInterceptImeInput(ev({ inputType: 'deleteContentBackward', data: null }))).toBe(false)
  })
})
