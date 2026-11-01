import { describe, expect, it } from 'vitest'
import { renderReleaseNotes } from '@/utils/release-notes'

describe('renderReleaseNotes', () => {
  // 标题、列表、加粗是发布说明最常用的三种结构
  it('ReleaseNotes_HeadingListBold_1', () => {
    const html = renderReleaseNotes('### Features\n- 新增自动更新\n- **重要**：零窗口自动安装')
    expect(html).toContain('<h4')
    expect(html).toContain('Features')
    expect(html).toContain('<ul')
    expect(html.match(/<li/g)).toHaveLength(2)
    expect(html).toContain('<strong>重要</strong>')
  })

  // 备注里出现 HTML 必须转义，不能当作标签执行
  it('ReleaseNotes_EscapesHtml_2', () => {
    const html = renderReleaseNotes('- <script>alert(1)</script>\n- a & b < c')
    expect(html).not.toContain('<script>')
    expect(html).toContain('&lt;script&gt;')
    expect(html).toContain('a &amp; b &lt; c')
  })

  // 空行分段、行内代码、链接只保留文字
  it('ReleaseNotes_ParagraphCodeLink_3', () => {
    const html = renderReleaseNotes('第一段\n\n第二段用 `cc-box -h` 查看\n\n见 [官网](https://ccbox.orczh.top/)')
    expect(html.match(/<p/g)?.length).toBe(3)
    expect(html).toContain('<code>cc-box -h</code>')
    expect(html).toContain('官网')
    expect(html).not.toContain('https://ccbox.orczh.top')
  })

  // 空输入不产生内容
  it('ReleaseNotes_Empty_4', () => {
    expect(renderReleaseNotes('')).toBe('')
    expect(renderReleaseNotes('   \n  ')).toBe('')
  })
})
