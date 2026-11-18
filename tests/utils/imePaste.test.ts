import { describe, it, expect } from 'vitest'
import { shouldInterceptImeInput, imeAppendDelta, imeBackspaceDelta } from '@/utils/ime-paste'

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

describe('imeInsertDelta 纯插入的 value 差分（任意位置）', () => {
  // 模型：不看 InputEvent.data 的语义（增量/全量/形态各内核不一），只对
  // textarea.value 做 LCP/LCS 差分——纯插入（任意位置）发送插入段。
  // 原始 bug 场景：自动补全双引号后光标在中间，空格是「中间插入」——
  // 前缀追加模型会吞掉空格，必须按任意位置纯插入判定
  it('ImePaste_中间插入_001', () => {
    // 光标在 '“|”' 中间输入空格 → '“ ”'（LCP='“'、LCS='”'，插入段 ' '）
    expect(imeAppendDelta('“”', '“ ”')).toBe(' ')
    expect(imeAppendDelta('“”', '“,”')).toBe(',')
  })

  // 尾部追加（全量/增量内核统一）：插入段即增量
  it('ImePaste_尾部追加_002', () => {
    expect(imeAppendDelta('“”', '“” ')).toBe(' ')
    expect(imeAppendDelta('', '“”')).toBe('“”')
    expect(imeAppendDelta('', 'a')).toBe('a')
  })

  // 无变化 / 含删除成分（替换、重写）：null（删除走 imeBackspaceDelta，其余吞掉）
  it('ImePaste_非纯插入_003', () => {
    expect(imeAppendDelta('“”', '“”')).toBeNull() // 无变化（纯重复提交）
    expect(imeAppendDelta('ab', 'b')).toBeNull() // 删除
    expect(imeAppendDelta('ab', 'ba')).toBeNull() // 重写
    expect(imeAppendDelta('ab', 'axby')).toBeNull() // 插入+删除（替换形态）
  })

  // 整段粘贴（含换行）：差分捕获全文，发送层走 term.paste 归一
  it('ImePaste_整段粘贴差分_004', () => {
    expect(imeAppendDelta('', 'line1\nline2\n')).toBe('line1\nline2\n')
  })
})

describe('imeBackspaceDelta 后缀删除换算（接管 xterm keydown-229 删除路径）', () => {
  // 后缀删除（IME 退格/长按删除）：换算为终端退格（每个 code point 一个 \x7f）
  it('ImePaste_后缀删除换算退格_001', () => {
    expect(imeBackspaceDelta('“”', '“')).toBe('\x7f')
    expect(imeBackspaceDelta('abc', '')).toBe('\x7f\x7f\x7f')
  })

  // 非删除 / 非后缀（中间删除、替换）：无法在终端流中表达 → null（重置基线吞掉）
  it('ImePaste_非后缀删除不处理_002', () => {
    expect(imeBackspaceDelta('abc', 'abc')).toBeNull()
    expect(imeBackspaceDelta('abc', 'bc')).toBeNull() // 中间删除
    expect(imeBackspaceDelta('ab', 'xy')).toBeNull() // 替换
  })
})
