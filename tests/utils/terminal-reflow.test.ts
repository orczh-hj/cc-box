import { describe, it, expect } from 'vitest'
import { reflowSnapshot, type SnapshotData } from '@/utils/terminal-reflow'

/** 便捷构造：全部行非 isWrapped、cols=80、未截断 */
function snap(lines: string[], opts?: Partial<SnapshotData>): SnapshotData {
  return {
    lines,
    wrapped: lines.map(() => false),
    cols: opts?.cols ?? 80,
    truncated: opts?.truncated ?? false,
    ...opts,
  }
}

/** 指定 wrapped 标志的构造 */
function wrappedSnap(lines: string[], wrapped: boolean[], cols = 80): SnapshotData {
  return { lines, wrapped, cols, truncated: false }
}

describe('terminal-reflow 反渲染', () => {
  it('ReflowSnapshot_空输入_001：空 lines 返回两个空串', () => {
    const r = reflowSnapshot(snap([]))
    expect(r.smart).toBe('')
    expect(r.raw).toBe('')
  })

  it('ReflowSnapshot_单行_002：单行原样返回', () => {
    const r = reflowSnapshot(snap(['hello world']))
    expect(r.smart).toBe('hello world')
    expect(r.raw).toBe('hello world')
  })

  it('ReflowSnapshot_IsWrapped无缝拼接_003：xterm 真软换行直接拼接', () => {
    const r = reflowSnapshot(
      wrappedSnap(['line1 wraps over the terminal width and contin', 'ues here'], [false, true])
    )
    expect(r.smart).toBe('line1 wraps over the terminal width and continues here')
    expect(r.raw).toBe('line1 wraps over the terminal width and contin\nues here')
  })

  it('ReflowSnapshot_满宽英文补空格_004：wrap-ansi 吃掉的词间空格补回', () => {
    const a = 'the quick brown f' // 17 字符
    const b = 'ox jumps over'
    const r = reflowSnapshot(snap([a, b], { cols: 18 })) // 17 ≥ 18-1 满宽
    expect(r.smart).toBe('the quick brown f ox jumps over')
  })

  it('ReflowSnapshot_满宽中文直拼_005：中文满宽折行无缝拼接', () => {
    const a = '这是一段很长的中文内容超过了终端宽度被强制折行到下一行继续输出文字内容呢' // 36 字
    const b = '这里是下一行的延续内容'
    const r = reflowSnapshot(snap([a, b], { cols: 36 }))
    expect(r.smart).toBe(a + b)
  })

  it('ReflowSnapshot_中文显示宽度_005b：中文字符数 < cols 但显示宽度满宽仍拼接', () => {
    // 40 个中文 = 显示宽 80 满宽占满 80 列终端，但字符串 length 只有 40
    const a = '前'.repeat(40)
    const b = '后'.repeat(8)
    const r = reflowSnapshot(snap([a, b], { cols: 80 }))
    expect(r.smart).toBe(a + b)
  })

  it('ReflowSnapshot_不足宽不拼接_006：短行后的换行是真实的', () => {
    const r = reflowSnapshot(snap(['第一行结束', '第二行开始'], { cols: 80 }))
    expect(r.smart).toBe('第一行结束\n第二行开始')
  })

  it('ReflowSnapshot_空行分段_007：B 为空行时保留换行形成段落', () => {
    const a = 'x'.repeat(79) // 满宽，但下一行是空行 → 不拼接
    const r = reflowSnapshot(snap([a, '', 'new paragraph'], { cols: 80 }))
    expect(r.smart).toBe(`${a}\n\nnew paragraph`)
  })

  it('ReflowSnapshot_Markdown结构保留_008：标题/列表/引用/表格开头的行不拼接', () => {
    const a = 'x'.repeat(79)
    const r = reflowSnapshot(snap([a, '## 标题', a, '- 列表项', a, '> 引用'], { cols: 80 }))
    expect(r.smart).toContain(`${a}\n## 标题`)
    expect(r.smart).toContain(`${a}\n- 列表项`)
    expect(r.smart).toContain(`${a}\n> 引用`)
  })

  it('ReflowSnapshot_有序列表保留_009：数字列表开头保留换行', () => {
    const a = 'x'.repeat(79)
    const r = reflowSnapshot(snap([a, '1. first item'], { cols: 80 }))
    expect(r.smart).toContain(`${a}\n1. first item`)
  })

  it('ReflowSnapshot_Diff标记保留_009b：diff 行（+/-/@@）不拼接', () => {
    const a = 'x'.repeat(79)
    const r = reflowSnapshot(snap([a, '+ added line', a, '- removed line', '@@ diff hunk @@'], { cols: 80 }))
    expect(r.smart).toContain(`${a}\n+ added line`)
    expect(r.smart).toContain(`${a}\n- removed line`)
    expect(r.smart).toContain('\n@@ diff hunk @@')
  })

  it('ReflowSnapshot_代码围栏原样_010：围栏内不拼接、保留空行与缩进', () => {
    const lines = ['```rust', 'fn main() {', '    println!("hello");', '', '}', '```', 'after code']
    const r = reflowSnapshot(snap(lines, { cols: 80 }))
    expect(r.smart).toBe(lines.join('\n'))
  })

  it('ReflowSnapshot_围栏后正文正常拼接_011：闭合围栏后的满宽行恢复正常规则', () => {
    const a = 'x'.repeat(79)
    const b = 'y'.repeat(20)
    const r = reflowSnapshot(snap(['```', 'code', '```', a, b], { cols: 80 }))
    // 英文边界拼接补一个空格
    expect(r.smart).toContain(`\`\`\`\n${a} ${b}`)
  })

  it('ReflowSnapshot_BoxDrawing保护_012：ASCII 图/边框行不拼接', () => {
    const a = 'x'.repeat(79)
    const r = reflowSnapshot(snap([a, '│ code content', '│ more'], { cols: 80 }))
    expect(r.smart).toContain(`${a}\n│ code content`)
  })

  it('ReflowSnapshot_表格行保护_013：markdown 表格行保留换行', () => {
    const a = 'x'.repeat(79)
    const header = '| col1 | col2 |'
    const r = reflowSnapshot(snap([a, header, '| v1 | v2 |'], { cols: 80 }))
    expect(r.smart).toContain(`${a}\n${header}\n| v1 | v2 |`)
  })

  it('ReflowSnapshot_Raw透传_014：raw 始终为原始行 join', () => {
    const r = reflowSnapshot(wrappedSnap(['ab', 'cd'], [false, true]))
    expect(r.raw).toBe('ab\ncd')
  })

  it('ReflowSnapshot_多段落混合_015：整段混合场景端到端', () => {
    // 英文满宽段落（补空格）+ 空行分段 + 中文满宽段落（直拼，显示宽度 18 ≥ 19-1）
    const p1a = 'aaaaaaaaaaaaaaaaaa' // 18
    const p1b = 'bbbbbbbbbbbbbbbbbb'
    const p2a = '中文内容满宽折行例' // 9 字，显示宽 18
    const p2b = '继续内容'
    const r = reflowSnapshot(snap([p1a, p1b, '', p2a, p2b], { cols: 19 }))
    expect(r.smart).toBe(`${p1a} ${p1b}\n\n${p2a}${p2b}`)
  })
})
