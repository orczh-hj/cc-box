import { describe, expect, it } from 'vitest'
import { renderMarkdown } from '../../website/src/lib/markdown'

/** 从渲染结果里提取第一张表的 colgroup 百分比宽度 */
const colWidths = (html: string): number[] => {
  const m = html.match(/<colgroup>(.*?)<\/colgroup>/)
  if (!m) return []
  return [...m[1].matchAll(/width:([\d.]+)%/g)].map((x) => parseFloat(x[1]))
}

describe('renderMarkdown 表格列宽分配', () => {
  it('Markdown_Table_1 内容最长的列拿到最大宽度比例（纠偏短内容列被压窄）', () => {
    const { html } = renderMarkdown(
      ['| 快捷键 | 描述 | 上下文 |', '| :- | :- | :- |', '| `Ctrl+C` | 中断 | 中断正在运行的操作，长内容说明文字若干若干若干若干 |'].join('\n'),
    )
    const w = colWidths(html)
    expect(w).toHaveLength(3)
    expect(w[2]).toBeGreaterThan(w[0])
    expect(w[2]).toBeGreaterThan(w[1])
  })

  it('Markdown_Table_2 各列百分比合计为 100（末列吸收取整误差）', () => {
    const { html } = renderMarkdown(
      ['| a | b | c |', '| :- | :- | :- |', '| 短 | 中等内容 | 特别长的内容特别长的内容特别长的内容特别长的内容 |'].join('\n'),
    )
    const sum = colWidths(html).reduce((a, b) => a + b, 0)
    expect(sum).toBe(100)
  })

  it('Markdown_Table_3 超长单列被封顶（≥3 列时不超过 55%），其余列不被饿死', () => {
    const { html } = renderMarkdown(
      [
        '| 键 | 值 | 备注 |',
        '| :- | :- | :- |',
        `| k | v | ${'超长'.repeat(80)} |`,
      ].join('\n'),
    )
    const w = colWidths(html)
    expect(w[2]).toBeLessThanOrEqual(55)
    expect(w[0]).toBeGreaterThan(5)
    expect(w[1]).toBeGreaterThan(5)
  })

  it('Markdown_Table_4 极短列有 8% 兜底宽度（fixed 布局下列宽即终值，防不可读）', () => {
    const { html } = renderMarkdown(
      [
        '| 参数 | 说明 |',
        '| :- | :- |',
        '| ✓ | 默认值即可启用，说明文字比较长一些一些一些一些一些一些一些 |',
      ].join('\n'),
    )
    const w = colWidths(html)
    expect(w[0]).toBeGreaterThanOrEqual(8)
  })

  it('Markdown_Table_5 单列表不输出 colgroup', () => {
    const { html } = renderMarkdown(['| 仅一列 |', '| :- |', '| 内容 |'].join('\n'))
    expect(html).not.toContain('<colgroup>')
  })
})

describe('renderMarkdown 标题锚点', () => {
  it('Markdown_Heading_1 h1 生成与 h2 相同的 slug id（docs 首章节可深链）', () => {
    const { html, toc } = renderMarkdown('# CC-Box\n\n## 快速 开始\n\n正文')
    expect(html).toContain('<h1 id="CC-Box">')
    expect(html).toContain('<h2 id="快速-开始">')
    expect(toc.map((t) => t.id)).toEqual(['快速-开始'])
  })

  it('Markdown_Heading_2 h1 与 h2 同名时后者加 -1 后缀，互不覆盖', () => {
    const { html } = renderMarkdown('# 概述\n\n## 概述\n\n正文')
    expect(html).toContain('<h1 id="概述">')
    expect(html).toContain('<h2 id="概述-1">')
  })
})
