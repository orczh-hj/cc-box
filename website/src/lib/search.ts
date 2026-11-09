// 官网全文搜索：构建期语料拼接 + 运行时暴力扫描（关键词 indexOf，无倒排索引）。
// 全部纯函数，构建（vite 插件）与运行时（SearchBox）共用，保证两端行为一致。

/** 单篇文档进入语料的形态（body 为剥码后的 markdown 原文） */
export interface CorpusDoc {
  ty: 'reference' | 'docs'
  key: string
  title: string
  body: string
}

/** 语料中的章节（由文档头与 `## ` 标题切出，start/end 为语料内偏移） */
export interface CorpusSection {
  ty: 'reference' | 'docs'
  key: string
  docTitle: string
  sectionTitle: string
  start: number
  end: number
}

export interface SearchHit {
  ty: 'reference' | 'docs'
  key: string
  docTitle: string
  sectionTitle: string
  snippet: string
}

/** 剥 code fence 与表格行：code/表格不是关键词目标且是体积大头 */
export function stripForSearch(md: string): string {
  return md
    .replace(/```[\s\S]*?```/g, '')
    .replace(/^\|.*\|$/gm, '')
}

// 语料分隔头：单行、行首、三段齐全才被 parseCorpus 认可（防正文撞车）
const HEADER_PREFIX = '@@@CCBOXDOC'
const HEADER_RE = /^@@@CCBOXDOC ty=(reference|docs) key=(\S+) title=(.*)$/

export function buildCorpusText(docs: CorpusDoc[]): string {
  return docs
    .map((d) => `${HEADER_PREFIX} ty=${d.ty} key=${d.key} title=${d.title}\n${d.body}`)
    .join('\n\n')
}

/** 扫头行切文档、按行首 `## ` 切章节；返回原文与带偏移的章节表（首章节标题 = 文档标题） */
export function parseCorpus(text: string): { corpus: string; sections: CorpusSection[] } {
  const lines = text.split('\n')
  const sections: CorpusSection[] = []
  // 逐行扫描重建偏移：sectionStart 指向章节首行行首（头行则指向头行）
  let doc: { ty: 'reference' | 'docs'; key: string; title: string } | null = null
  let sectionTitle = ''
  let sectionStart = 0
  let offset = 0
  const closeSection = (end: number) => {
    if (!doc) return
    sections.push({
      ty: doc.ty,
      key: doc.key,
      docTitle: doc.title,
      sectionTitle,
      start: sectionStart,
      end,
    })
  }
  for (let i = 0; i < lines.length; i++) {
    const line = lines[i]
    const m = line.match(HEADER_RE)
    if (m) {
      closeSection(offset)
      doc = { ty: m[1] as 'reference' | 'docs', key: m[2], title: m[3] }
      sectionTitle = doc.title
      // 首章节从正文开始：分隔头是语料格式，不能被搜索命中或出现在摘要里
      sectionStart = offset + line.length + 1
    } else if (doc && /^## /.test(line)) {
      closeSection(offset)
      sectionTitle = line.replace(/^## /, '').trim()
      sectionStart = offset
    }
    offset += line.length + 1 // +1 = 换行符
  }
  closeSection(text.length)
  return { corpus: text, sections }
}

function countOccurrences(haystack: string, needle: string): number {
  let n = 0
  let i = -1
  while ((i = haystack.indexOf(needle, i + 1)) !== -1) n++
  return n
}

/**
 * 关键词搜索：空白拆词、AND 语义（章节须含全部词）。
 * 排序：章节标题包含任一关键词 > 文档标题包含 > 正文命中次数；截前 12 条。
 */
export function searchCorpus(
  corpus: string,
  sections: CorpusSection[],
  query: string,
  limit = 12,
): SearchHit[] {
  const keywords = query.split(/\s+/).filter(Boolean)
  if (keywords.length === 0) return []
  // 大小写不敏感：关键词与正文/标题统一小写后比较（摘要标记在 makeSnippet 内处理）
  const lowerKws = keywords.map((k) => k.toLowerCase())
  const hits: (SearchHit & { score: number })[] = []
  for (const s of sections) {
    const text = corpus.slice(s.start, s.end).toLowerCase()
    const sectionTitle = s.sectionTitle.toLowerCase()
    const docTitle = s.docTitle.toLowerCase()
    let bodyCount = 0
    let all = true
    for (const kw of lowerKws) {
      const n = countOccurrences(text, kw)
      if (n === 0) {
        all = false
        break
      }
      bodyCount += n
    }
    if (!all) continue
    const titleHit = lowerKws.some((kw) => sectionTitle.includes(kw)) ? 2 : 0
    const docHit = lowerKws.some((kw) => docTitle.includes(kw)) ? 1 : 0
    hits.push({
      ty: s.ty,
      key: s.key,
      docTitle: s.docTitle,
      sectionTitle: s.sectionTitle,
      snippet: makeSnippet(corpus, s.start, s.end, keywords),
      score: titleHit * 1_000_000 + docHit * 100_000 + bodyCount,
    })
  }
  hits.sort((a, b) => b.score - a.score)
  return hits.slice(0, limit).map(({ score: _score, ...hit }) => hit)
}

/** 首个命中位置前后 ~40 字的摘要，关键词以 «» 包裹（前端渲染为 <mark>）；
 * 匹配不区分大小写，«» 内保留原文大小写 */
export function makeSnippet(
  corpus: string,
  start: number,
  end: number,
  keywords: string[],
): string {
  const text = corpus.slice(start, end)
  const esc = (s: string) => s.replace(/[.*+?^${}()|[\]\\]/g, '\\$&')
  const re = new RegExp(`(${keywords.map((k) => esc(k.toLowerCase())).join('|')})`, 'gi')
  const m = re.exec(text)
  if (!m) return text.slice(0, 80)
  const first = m.index
  const raw = text.slice(Math.max(0, first - 40), Math.min(text.length, first + 40 + m[1].length))
  return raw.replace(re, '«$1»')
}

/** 跳转 URL：reference 带 ?a= 编码章节标题（ReferenceApp 文字兜底）；docs 锚点 = slug（与 h2 id 规则一致） */
export function buildJumpUrl(hit: Pick<SearchHit, 'ty' | 'key' | 'sectionTitle'>): string {
  if (hit.ty === 'reference') {
    return `reference.html#/${hit.key}?a=${encodeURIComponent(hit.sectionTitle)}`
  }
  const slug = hit.sectionTitle.trim().replace(/\s+/g, '-')
  return `docs.html#${encodeURIComponent(slug)}`
}
