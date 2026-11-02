import { describe, expect, it } from 'vitest'
import { resolveInternalDocLink } from '../../website/src/lib/internalLinks'

// 真值来源：官方文档 URL 形态与本地 key（相对路径，斜杠分隔），
// 期望值按官方链接样例手写，非按实现逻辑反推
const manifest = [
  { key: 'getting-started/overview' },
  { key: 'reference/settings' },
  { key: 'plugins/mods/overview' },
  { key: 'plugins/mods/interface' },
  { key: 'plugins/mods/events' },
]

describe('resolveInternalDocLink', () => {
  it('Resolve_MultiSegmentZhLink_1：多段嵌套路径（zh-CN）命中本地文档', () => {
    // 真实场景：mods 文档正文里的互链是多段路径（曾因正则只允许单段而漏拦截跳外链）
    const url = new URL('https://code.claude.com/docs/zh-CN/plugins/mods/interface.md')
    expect(resolveInternalDocLink(url, manifest)?.key).toBe('plugins/mods/interface')
  })

  it('Resolve_MultiSegmentEnLinkWithAnchor_2：多段路径（en、带锚点）命中', () => {
    const url = new URL('https://code.claude.com/docs/en/plugins/mods/events.md#guard-or-change-a-tool-call')
    expect(resolveInternalDocLink(url, manifest)?.key).toBe('plugins/mods/events')
  })

  it('Resolve_SingleSegmentZhLink_3：单段路径（zh-CN）按末段命中', () => {
    const url = new URL('https://code.claude.com/docs/zh-CN/settings.md')
    expect(resolveInternalDocLink(url, manifest)?.key).toBe('reference/settings')
  })

  it('Resolve_UnknownDoc_4：清单里不存在的文档返回 null（保持外链行为）', () => {
    const url = new URL('https://code.claude.com/docs/zh-CN/some/unknown-doc.md')
    expect(resolveInternalDocLink(url, manifest)).toBeNull()
  })

  it('Resolve_NonDocHost_5：非文档路径或其他域名返回 null', () => {
    expect(
      resolveInternalDocLink(new URL('https://code.claude.com/docs/zh-CN/overview'), manifest),
    ).toBeNull() // 非 .md 结尾
    expect(
      resolveInternalDocLink(new URL('https://example.com/docs/zh-CN/settings.md'), manifest),
    ).toBeNull()
  })
})
