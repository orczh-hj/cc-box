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

/** 剥离官方文档每篇开头的 `> ## Documentation Index …` 引用块（不影响正文里其他引用） */
export function stripLeadingBlockquote(md: string): string {
  const m = md.match(/^(?:\s*>.*\n)+/)
  if (m && /Documentation Index|llms\.txt/.test(m[0])) {
    return md.slice(m[0].length)
  }
  return md
}

/**
 * 官方文档的 Mintlify 引用块标记统一转为「标记节点 + 普通正文」，渲染后由 enhanceBlocks 折叠：
 * - `> [tab] 标题`：多选一 tab（可嵌套，`> > [tab]` 是外层面板内的第二组）
 * - `> [accordion] 标题`：可折叠段（原生 details）
 * - `> [!NOTE/TIP/WARNING/CAUTION/IMPORTANT]`：提示盒
 * 标记行换成对应标记节点；其后同一引用块按标记的引用深度剥掉 `>`；
 * 引用块结束时按栈深发出等量 `<div class="md-block-end">` 供 DOM 侧界定边界。
 * 只处理 ``` 围栏之外的行。
 */
const END_MARK = '<div class="md-block-end"></div>'

export function transformBlocks(md: string): string {
  const lines = md.split('\n')
  let inFence = false
  const stack: number[] = []
  // 空行结束引用块，但下一非空行若是同底深度的 tab 标记则同组延续（官方文档里
  // 同一组平台 tab 之间隔着空行）；否则每个未闭合标记发一个结束节点。
  // 标记/结束节点前后保留空行——紧贴其他行会被 marked 并入 HTML 块吞掉内容。
  let pendingEnds = 0
  let clearedBase = 0
  const depthOf = (l: string) => {
    const m = l.match(/^((?:>\s?)+)/)
    return m ? (m[1].match(/>/g) ?? []).length : 0
  }
  const endsLines = (n: number) => Array(n).fill(END_MARK)

  const out: string[] = []
  // 空行去重：标记/结束节点前后需空行，但不与已有空行叠加
  const pushBlank = () => {
    if (out.length && out[out.length - 1] !== '') out.push('')
  }
  // 结束节点独立成块：前后补空行，避免 marked 把相邻 markdown 并进 HTML 块
  const flushEnds = () => {
    if (pendingEnds > 0) {
      pushBlank()
      out.push(...endsLines(pendingEnds))
      out.push('')
    }
    pendingEnds = 0
  }

  for (const line of lines) {
    if (/^\s*(```|~~~)/.test(line)) {
      flushEnds()
      stack.length = 0
      inFence = !inFence
      out.push(line)
      continue
    }
    if (inFence) {
      out.push(line)
      continue
    }

    const tab = line.match(/^(?:>\s?)+\[(tab|accordion)\]\s*(.*)$/)
    const callout = line.match(/^(?:>\s?)+\[!(NOTE|TIP|WARNING|CAUTION|IMPORTANT)\]\s*(.*)$/)
    const isTab = !!tab && !!tab[2].trim()
    const isCallout = !!callout
    const isAccordion = !!tab && !!tab[2].trim() && tab[1] === 'accordion'

    if (isTab || isCallout) {
      const level = depthOf(line)
      // 同底深度的 tab：延续同组，吞掉挂起的结束节点（块边界不作为组边界）；
      // 无挂起时（同一引用块内的嵌套标记）直接入栈
      if (pendingEnds > 0) {
        const sameGroup = isTab && !isAccordion && level === clearedBase
        if (sameGroup) {
          pendingEnds = 0
        } else {
          flushEnds()
          stack.length = 0
        }
      }
      stack.push(level)
      pushBlank()
      out.push(
        isCallout
          ? `<div class="md-callout-marker" data-type="${callout![1].toLowerCase()}"${callout![2].trim() ? ` data-title="${callout![2].trim()}"` : ''}></div>`
          : `<div class="md-tab-title" data-kind="${tab![1]}" data-level="${level}">${tab![2].trim()}</div>`,
      )
      out.push('')
      continue
    }

    if (stack.length || pendingEnds) {
      if (line.trim() === '') {
        if (stack.length) {
          clearedBase = stack[0]
          pendingEnds += stack.length
          stack.length = 0
        }
        out.push('')
        continue
      }
      const d = depthOf(line)
      while (stack.length && d < stack[stack.length - 1]) stack.pop()
      if (stack.length && d >= stack[stack.length - 1]) {
        const n = stack[stack.length - 1]
        out.push(line.replace(new RegExp(`^(?:>\\s?){${n}}`), ''))
        continue
      }
    }

    if (pendingEnds) {
      flushEnds()
      if (line.trim() === '') continue
    }
    out.push(line)
  }
  flushEnds()

  const tail = stack.length
  const body = out.join('\n')
  return tail > 0 ? `${body}\n${endsLines(tail).join('\n')}` : body
}

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

  const html = marked.parse(transformBlocks(md), { async: false }) as string
  return { html, toc }
}
