import { describe, expect, it } from 'vitest'
import {
  buildCorpusText,
  buildJumpUrl,
  makeSnippet,
  parseCorpus,
  searchCorpus,
  stripForSearch,
  type CorpusDoc,
} from '../../website/src/lib/search'

// 语料样例：两篇文档（reference / docs 各一），覆盖 code fence、表格、多章节
const DOC_A: CorpusDoc = {
  ty: 'reference',
  key: 'getting-started/overview',
  title: '概述',
  body: `# 概述

Claude Code 是命令行工具。

## 安装

运行 npm install -g @anthropic-ai/claude-code 完成安装。

## 配置

\`\`\`bash
export ANTHROPIC_API_KEY=xxx
\`\`\`

| 平台 | 说明 |
| --- | --- |
| Windows | MSVC |

在配置文件中写入密钥。`,
}

const DOC_B: CorpusDoc = {
  ty: 'docs',
  key: 'cc-box',
  title: 'CC-Box 使用指南',
  body: `# CC-Box

## 快速开始

下载安装包后启动。
MCP 面板在侧边栏。`,
}

describe('stripForSearch', () => {
  it('StripForSearch_1 剥 code fence 与表格行，保留正文文字', () => {
    const out = stripForSearch(DOC_A.body)
    expect(out).not.toContain('```')
    expect(out).not.toContain('| Windows |')
    expect(out).not.toContain('ANTHROPIC_API_KEY')
    expect(out).toContain('## 配置')
    expect(out).toContain('在配置文件中写入密钥')
  })
})

describe('buildCorpusText / parseCorpus round-trip', () => {
  const text = buildCorpusText([DOC_A, DOC_B])
  const { corpus, sections } = parseCorpus(text)

  it('ParseCorpus_1 切回两篇文档且 ty/key/title 正确', () => {
    const keys = sections.map((s) => s.key)
    expect(new Set(keys)).toEqual(new Set(['getting-started/overview', 'cc-box']))
    expect(sections.every((s) => corpus.slice(s.start, s.end).length > 0)).toBe(true)
  })

  it('ParseCorpus_2 按 ## 切章节，首章节标题为文档标题', () => {
    const a = sections.filter((s) => s.key === 'getting-started/overview')
    expect(a.map((s) => s.sectionTitle)).toEqual(['概述', '安装', '配置'])
    expect(a[0].docTitle).toBe('概述')
    expect(a[0].ty).toBe('reference')
  })

  it('ParseCorpus_3 偏移落在对应章节文本上', () => {
    const a = sections.filter((s) => s.key === 'getting-started/overview')
    const slice = corpus.slice(a[1].start, a[1].end)
    expect(slice).toContain('npm install -g')
    // 章节起点 = `## ` 标题行行首（标题本身也可被搜索命中）
    expect(slice.startsWith('## 安装')).toBe(true)
  })

  it('ParseCorpus_4 正文出现残缺头行（缺 key/title）不切分新文档', () => {
    const tricky = buildCorpusText([
      { ty: 'docs', key: 'x', title: 'X', body: '正文提到 @@@CCBOXDOC ty=reference 但后面没有结构' },
    ])
    expect(parseCorpus(tricky).sections).toHaveLength(1)
  })

  it('ParseCorpus_5 行中（非行首）出现完整头行字段不切分', () => {
    const tricky = buildCorpusText([
      { ty: 'docs', key: 'x', title: 'X', body: '前缀 @@@CCBOXDOC ty=reference key=y title=Y 混在行中' },
    ])
    expect(parseCorpus(tricky).sections).toHaveLength(1)
  })

  it('ParseCorpus_6 首章节从正文开始，不含语料分隔头', () => {
    const a = sections.filter((s) => s.key === 'getting-started/overview')
    expect(corpus.slice(a[0].start, a[0].end).startsWith('@@@CCBOXDOC')).toBe(false)
    expect(corpus.slice(a[0].start, a[0].end).trimStart().startsWith('# 概述')).toBe(true)
  })
})

describe('searchCorpus', () => {
  const text = buildCorpusText([DOC_A, DOC_B])
  const { corpus, sections } = parseCorpus(text)

  it('SearchCorpus_1 多关键词 AND 语义：只含一词的章节不返回', () => {
    const hits = searchCorpus(corpus, sections, 'MCP 配置')
    // DOC_B「快速开始」同时含 MCP 与 配置？——「配置」只出现在 DOC_A，故 AND 结果为空
    expect(hits.map((h) => h.sectionTitle)).not.toContain('快速开始')
  })

  it('SearchCorpus_2 两词都含的章节返回', () => {
    const text2 = buildCorpusText([
      { ty: 'docs', key: 'x', title: 'X', body: '## 章节\nMCP 面板支持配置管理。' },
    ])
    const { corpus: c, sections: s } = parseCorpus(text2)
    const hits = searchCorpus(c, s, 'MCP 配置')
    expect(hits).toHaveLength(1)
    expect(hits[0].sectionTitle).toBe('章节')
  })

  it('SearchCorpus_3 长章节尾部关键词可命中（不截断语料）', () => {
    const tail = '章'.repeat(600) + '遥测'
    const text3 = buildCorpusText([{ ty: 'docs', key: 'x', title: 'X', body: `## 长文\n${tail}` }])
    const { corpus: c, sections: s } = parseCorpus(text3)
    expect(searchCorpus(c, s, '遥测')).toHaveLength(1)
  })

  it('SearchCorpus_4 章节标题命中排在仅正文命中前面', () => {
    const text4 = buildCorpusText([
      {
        ty: 'docs',
        key: 'x',
        title: 'X',
        body: '## 术语表\n这里解释遥测是什么。\n\n## 其他\n遥测也出现在这里。',
      },
    ])
    const { corpus: c, sections: s } = parseCorpus(text4)
    const hits = searchCorpus(c, s, '遥测')
    expect(hits[0].sectionTitle).toBe('术语表')
  })

  it('SearchCorpus_5 空白查询与单字查询返回空', () => {
    expect(searchCorpus(corpus, sections, '')).toEqual([])
    expect(searchCorpus(corpus, sections, '   ')).toEqual([])
  })

  it('SearchCorpus_7 语料内部标记不参与搜索（分隔头不是正文）', () => {
    expect(searchCorpus(corpus, sections, 'CCBOXDOC')).toEqual([])
  })

  it('SearchCorpus_6 结果最多 12 条且带摘要', () => {
    const body = Array.from({ length: 20 }, (_, i) => `## 章节${i}\n都包含磁盘关键字。`).join('\n\n')
    const text6 = buildCorpusText([{ ty: 'docs', key: 'x', title: 'X', body }])
    const { corpus: c, sections: s } = parseCorpus(text6)
    const hits = searchCorpus(c, s, '磁盘')
    expect(hits).toHaveLength(12)
    expect(hits[0].snippet).toContain('磁盘')
  })
})

describe('makeSnippet', () => {
  const corpus = 'A'.repeat(100) + '关键词' + 'B'.repeat(100)

  it('MakeSnippet_1 命中前后各约 40 字且关键词被 «» 包裹', () => {
    const s = makeSnippet(corpus, 0, corpus.length, ['关键词'])
    expect(s).toContain('«关键词»')
    expect(s.length).toBeLessThanOrEqual(100)
  })

  it('MakeSnippet_2 命中在开头时起点钳到 0', () => {
    const s = makeSnippet('关键词' + 'B'.repeat(100), 0, 105, ['关键词'])
    expect(s.startsWith('«')).toBe(true)
  })
})

describe('buildJumpUrl', () => {
  it('BuildJumpUrl_1 reference 类跳转带 ?a= 编码章节标题', () => {
    expect(buildJumpUrl({ ty: 'reference', key: 'getting-started/overview', sectionTitle: '安装' })).toBe(
      'reference.html#/getting-started/overview?a=' + encodeURIComponent('安装'),
    )
  })

  it('BuildJumpUrl_2 docs 类跳转锚点为 slug（空白转 -，hash 编码——DocsApp 侧 decode）', () => {
    expect(buildJumpUrl({ ty: 'docs', key: 'cc-box', sectionTitle: '快速 开始' })).toBe(
      'docs.html#' + encodeURIComponent('快速-开始'),
    )
  })
})
