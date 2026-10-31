import { describe, expect, it } from 'vitest'
import { transformBlocks } from '../../website/src/lib/markdown'

// Mintlify 标记转换：tab/accordion/callout 标记 → 标记节点（前后留空行防 HTML 块吞并）；
// 引用内容按标记深度剥 `>`；空行分隔的同深度 tab 延续同组；块结束发结束节点
describe('transformBlocks', () => {
  const T1 = '<div class="md-tab-title" data-kind="tab" data-level="1">'
  const T2 = '<div class="md-tab-title" data-kind="tab" data-level="2">'
  const END = '<div class="md-block-end"></div>'

  it('tab 标记 + 引用块内容：剥引用，块结束发结束节点', () => {
    const md = ['前言', '', '> [tab] 原生安装（推荐）', '>', '> **macOS：**', '>', '> ```bash', '> curl x', '> ```', '', '后续正文'].join('\n')
    expect(transformBlocks(md)).toBe(
      ['前言', '', `${T1}原生安装（推荐）</div>`, '', '', '**macOS：**', '', '```bash', 'curl x', '```', '', END, '', '后续正文'].join('\n'),
    )
  })

  it('嵌套 tab：同引用块内标记直接入栈，深度回落视为内层结束', () => {
    const md = ['> [tab] Terminal', '> CLI 正文', '> > [tab] 原生安装', '> >', '> > 安装步骤', '>', '> 外层补充'].join('\n')
    expect(transformBlocks(md)).toBe(
      [`${T1}Terminal</div>`, '', 'CLI 正文', '', `${T2}原生安装</div>`, '', '', '安装步骤', '', '外层补充', END].join('\n'),
    )
  })

  it('空行分隔的同深度 tab 延续同组（吞掉挂起结束节点）', () => {
    const md = ['> [tab] Terminal', '>', '> CLI 内容', '', '> [tab] VS Code', '>', '> 插件内容', '', '正文'].join('\n')
    expect(transformBlocks(md)).toBe(
      [`${T1}Terminal</div>`, '', '', 'CLI 内容', '', `${T1}VS Code</div>`, '', '', '插件内容', '', END, '', '正文'].join('\n'),
    )
  })

  it('callout 与 accordion：各自成块、内容去引用、结束发结束节点', () => {
    const md = ['正文', '', '> [!NOTE]', '>', '> 自动更新说明。', '', '中间正文', '', '> [accordion] 折叠标题', '>', '> 折叠内容。', '', '结尾'].join('\n')
    expect(transformBlocks(md)).toBe(
      [
        '正文',
        '',
        '<div class="md-callout-marker" data-type="note"></div>',
        '',
        '',
        '自动更新说明。',
        '',
        END,
        '',
        '中间正文',
        '',
        '<div class="md-tab-title" data-kind="accordion" data-level="1">折叠标题</div>',
        '',
        '',
        '折叠内容。',
        '',
        END,
        '',
        '结尾',
      ].join('\n'),
    )
  })

  it('非引用的 [tab] 文本与代码围栏内的行不受影响', () => {
    const md = ['```bash', '> [tab] 不是 tab', '```', '', '[tab] 裸文本不处理'].join('\n')
    expect(transformBlocks(md)).toBe(md)
  })
})
