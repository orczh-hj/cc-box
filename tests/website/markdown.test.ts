import { describe, expect, it } from 'vitest'
import { renderMarkdown } from '../../website/src/lib/markdown'

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
