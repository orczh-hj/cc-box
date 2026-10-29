import { Marked, type Tokens } from 'marked'

export interface TocEntry {
  id: string
  label: string
}

export interface RenderedDoc {
  html: string
  toc: TocEntry[]
}

// 渲染文档 Markdown：h1 保留为组标题；h2 加锚点 id 并收集为目录项
export function renderMarkdown(md: string): RenderedDoc {
  const toc: TocEntry[] = []
  const seen = new Map<string, number>()
  const slug = (raw: string) => {
    const base = raw.trim().replace(/\s+/g, '-')
    const n = seen.get(base) ?? 0
    seen.set(base, n + 1)
    return n === 0 ? base : `${base}-${n}`
  }

  const marked = new Marked({
    renderer: {
      heading(token: Tokens.Heading) {
        const inline = this.parser.parseInline(token.tokens)
        if (token.depth === 1) return `<h1>${inline}</h1>`
        if (token.depth === 2) {
          const label = token.raw.replace(/^#+\s*/, '').trim()
          const id = slug(label)
          toc.push({ id, label })
          return `<h2 id="${id}">${inline}</h2>`
        }
        return `<h3>${inline}</h3>`
      },
      image(token: Tokens.Image) {
        const alt = token.text.replace(/"/g, '&quot;')
        return `<figure class="md-fig"><img src="${token.href}" alt="${alt}" loading="lazy"><figcaption>${alt}</figcaption></figure>`
      },
      // 表格外包一层滚动容器：窄屏下列多、内容长时不撑破页面布局
      table(token: Tokens.Table) {
        const cell = (c: Tokens.TableCell, tag: 'th' | 'td') => {
          const align = c.align ? ` style="text-align:${c.align}"` : ''
          return `<${tag}${align}>${this.parser.parseInline(c.tokens)}</${tag}>`
        }
        const head = token.header.map((c) => cell(c, 'th')).join('')
        const body = token.rows
          .map((row) => `<tr>${row.map((c) => cell(c, 'td')).join('')}</tr>`)
          .join('')
        return `<div class="md-table-wrap"><table><thead><tr>${head}</tr></thead><tbody>${body}</tbody></table></div>`
      },
    },
  })

  const html = marked.parse(md, { async: false }) as string
  return { html, toc }
}
