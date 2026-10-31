import { describe, expect, it } from 'vitest'
import { applyDocOrder, buildRefTree, fallbackLabel } from '../../website/src/lib/refTree'
import { stripLeadingBlockquote } from '../../website/src/lib/markdown'

// 参考文档目录树构建：三层结构（类 → 组 → 文档）、中文标签映射、根级文档归属
describe('buildRefTree', () => {
  const entries = [
    { key: 'getting-started/overview', title: '概述' },
    { key: 'getting-started/quickstart', title: '快速入门' },
    { key: 'getting-started/core-concepts/agents', title: 'Agent' },
    { key: 'reference/glossary/terms', title: '术语' },
  ]

  it('构建三层结构：顶级类 → 子目录组 → 文档', () => {
    const tree = buildRefTree(entries)
    expect(tree.map((c) => c.name)).toEqual(['getting-started', 'reference'])

    const gs = tree[0]
    expect(gs.label).toBe('入门')
    // 根级文档（两段 key）归入 __root__ 组且不显示组标题
    const root = gs.groups.find((g) => g.name === '__root__')
    expect(root?.label).toBe('')
    expect(root?.docs.map((d) => d.key)).toEqual([
      'getting-started/overview',
      'getting-started/quickstart',
    ])

    // 三段 key 进子目录组，组带中文标签
    const cc = gs.groups.find((g) => g.name === 'core-concepts')
    expect(cc?.label).toBe('核心概念')
    expect(cc?.docs).toEqual([{ key: 'getting-started/core-concepts/agents', title: 'Agent' }])
  })

  it('文档标题缺失时回退文件名', () => {
    const tree = buildRefTree([{ key: 'plugins/create-plugins/quickstart', title: '' }])
    expect(tree[0].groups[0].docs[0].title).toBe('quickstart')
  })

  it('未知类/组名回退可读标题而非原样 kebab-case', () => {
    const tree = buildRefTree([{ key: 'unknown-cat/unknown-group/doc', title: 'D' }])
    expect(tree[0].label).toBe('Unknown Cat')
    expect(tree[0].groups[0].label).toBe('Unknown Group')
  })

  it('忽略缺少类或名字段的非法条目', () => {
    expect(buildRefTree([{ key: 'no-slash', title: 'x' }])).toEqual([])
  })
})

describe('fallbackLabel', () => {
  it('kebab-case 转空格分隔首字母大写', () => {
    expect(fallbackLabel('core-concepts')).toBe('Core Concepts')
    expect(fallbackLabel('mcp')).toBe('Mcp')
  })
})

// 官方索引顺序重排：类、组、文档三层都按 order.json 首次出现顺序
describe('applyDocOrder', () => {
  it('按 order 清单重排文档与类顺序', () => {
    const entries = [
      { key: 'getting-started/overview', title: '概述' },
      { key: 'configuration/settings', title: '设置' },
      { key: 'getting-started/quickstart', title: '快速入门' },
    ]
    const ordered = applyDocOrder(entries, [
      'configuration/settings',
      'getting-started/overview',
      'getting-started/quickstart',
    ])
    expect(ordered.map((e) => e.key)).toEqual([
      'configuration/settings',
      'getting-started/overview',
      'getting-started/quickstart',
    ])
  })

  it('组顺序由组内首个登记路径决定', () => {
    const entries = [
      { key: 'getting-started/core-concepts/agents', title: 'A' },
      { key: 'getting-started/use-claude-code/memory', title: 'M' },
      { key: 'getting-started/core-concepts/context', title: 'C' },
    ]
    const ordered = applyDocOrder(entries, [
      'getting-started/use-claude-code/memory',
      'getting-started/core-concepts/context',
      'getting-started/core-concepts/agents',
    ])
    expect(ordered.map((e) => e.key)).toEqual([
      'getting-started/use-claude-code/memory',
      'getting-started/core-concepts/context',
      'getting-started/core-concepts/agents',
    ])
  })

  it('未登记条目排在所属类末尾，未登记类排在最后', () => {
    const entries = [
      { key: 'local-only/doc', title: 'L' },
      { key: 'getting-started/extra', title: 'E' },
      { key: 'getting-started/overview', title: 'O' },
    ]
    const ordered = applyDocOrder(entries, ['getting-started/overview'])
    expect(ordered.map((e) => e.key)).toEqual([
      'getting-started/overview',
      'getting-started/extra',
      'local-only/doc',
    ])
  })

  it('空 order 清单时全部按类聚集、末尾排序', () => {
    const entries = [
      { key: 'b/one', title: '1' },
      { key: 'a/two', title: '2' },
    ]
    const ordered = applyDocOrder(entries, [])
    expect(ordered.map((e) => e.key)).toEqual(['a/two', 'b/one'])
  })
})

// 官方文档开头的 Documentation Index 引用块需剥离，正文引用不受影响
describe('stripLeadingBlockquote', () => {
  const indexBlock = '> ## Documentation Index\n> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt\n> Use this file to discover all available pages before exploring further.\n'

  it('剥离开头的 Documentation Index 引用块', () => {
    const md = indexBlock + '# 快速入门\n\n正文。'
    expect(stripLeadingBlockquote(md)).toBe('# 快速入门\n\n正文。')
  })

  it('正文开头的普通引用块保留', () => {
    const md = '> 这是一段正文引言\n\n# 标题'
    expect(stripLeadingBlockquote(md)).toBe(md)
  })

  it('无引用块时原样返回', () => {
    const md = '# 标题\n正文'
    expect(stripLeadingBlockquote(md)).toBe(md)
  })
})

