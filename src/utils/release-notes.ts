/**
 * 发布说明（Release notes）的极简 Markdown 渲染。
 *
 * 为什么自己写：说明内容是发布者手写的固定子集（标题 / 列表 / 加粗 / 行内代码），
 * 引入完整 Markdown 库对这点内容是浪费；自研还能保证「先转义再解析」——
 * 任何 `<` `>` `&` 都变成文本，`v-html` 不会执行注入标签。
 *
 * 支持：`#`~`######` 标题、`-`/`*` 无序列表、`1.` 有序列表、`**粗体**`、
 * `` `行内代码` ``、`[文字](链接)`（只保留文字，链接不注入，避免 WebView 内导航）。
 * 不支持：代码块、表格、图片——说明里用不到。
 */

const ESCAPES: Record<string, string> = {
  '&': '&amp;',
  '<': '&lt;',
  '>': '&gt;',
  '"': '&quot;',
  "'": '&#39;',
}

function escapeHtml(s: string): string {
  return s.replace(/[&<>"']/g, (c) => ESCAPES[c])
}

/** 行内标记（输入已转义，故只需处理标记本身） */
function inline(escaped: string): string {
  return escaped
    // 链接只保留文字：WebView 内导航会把应用页面替换掉
    .replace(/\[([^\]]+)\]\([^)]*\)/g, '$1')
    .replace(/\*\*([^*]+)\*\*/g, '<strong>$1</strong>')
    .replace(/`([^`]+)`/g, '<code>$1</code>')
}

/**
 * 把 Markdown 子集渲染成 HTML 字符串（内容已转义，可安全用于 v-html）。
 * 空输入返回空串。
 */
export function renderReleaseNotes(md: string): string {
  const lines = md.replace(/\r\n?/g, '\n').split('\n')
  const out: string[] = []
  let list: 'ul' | 'ol' | null = null
  let paragraph: string[] = []

  const flushParagraph = () => {
    if (paragraph.length) {
      out.push(`<p>${paragraph.join('<br>')}</p>`)
      paragraph = []
    }
  }
  const closeList = () => {
    if (list) {
      out.push(`</${list}>`)
      list = null
    }
  }

  for (const raw of lines) {
    const line = raw.trim()
    if (!line) {
      flushParagraph()
      closeList()
      continue
    }

    const heading = line.match(/^(#{1,6})\s+(.*)$/)
    if (heading) {
      flushParagraph()
      closeList()
      // 整体降一级：说明里常用 `###`，渲染成 h4 才不至于比卡片标题还大
      const level = Math.min(heading[1].length + 1, 6)
      out.push(`<h${level}>${inline(escapeHtml(heading[2]))}</h${level}>`)
      continue
    }

    const bullet = line.match(/^[-*+]\s+(.*)$/)
    const ordered = line.match(/^\d+\.\s+(.*)$/)
    if (bullet || ordered) {
      flushParagraph()
      const want: 'ul' | 'ol' = bullet ? 'ul' : 'ol'
      if (list !== want) {
        closeList()
        out.push(`<${want}>`)
        list = want
      }
      out.push(`<li>${inline(escapeHtml((bullet ?? ordered)![1]))}</li>`)
      continue
    }

    closeList()
    paragraph.push(inline(escapeHtml(line)))
  }

  flushParagraph()
  closeList()
  return out.join('')
}
