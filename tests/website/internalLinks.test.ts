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

  it('Resolve_NoMdSuffixMultiSegment_4：不带 .md 后缀的多段路径（正文里大量存在）也命中', () => {
    // 真实场景：mods 互链与多数正文链接无 .md 后缀（/docs/zh-CN/plugins/mods/interface）
    const url = new URL('https://code.claude.com/docs/zh-CN/plugins/mods/interface')
    expect(resolveInternalDocLink(url, manifest)?.key).toBe('plugins/mods/interface')
  })

  it('Resolve_NoMdSuffixSingleSegment_5：不带 .md 后缀的单段路径命中', () => {
    const url = new URL('https://code.claude.com/docs/en/settings')
    expect(resolveInternalDocLink(url, manifest)?.key).toBe('reference/settings')
  })

  it('Resolve_NoLocalePrefix_6：无 zh-CN/en 前缀的路径命中', () => {
    const url = new URL('https://code.claude.com/docs/plugins/mods/events.md')
    expect(resolveInternalDocLink(url, manifest)?.key).toBe('plugins/mods/events')
  })

  it('Resolve_UnknownDoc_7：清单里不存在的文档返回 null（保持外链行为）', () => {
    const url = new URL('https://code.claude.com/docs/zh-CN/some/unknown-doc.md')
    expect(resolveInternalDocLink(url, manifest)).toBeNull()
  })

  it('Resolve_NonDocPath_8：非文档路径、query、其他域名返回 null', () => {
    expect(
      resolveInternalDocLink(new URL('https://code.claude.com/docs/'), manifest),
    ).toBeNull() // 无 path
    expect(
      resolveInternalDocLink(new URL('https://code.claude.com/docs/zh-CN/settings.md?from=link'), manifest)?.key,
    ).toBe('reference/settings') // pathname 不含 query，query 被忽略、仍命中站内
    expect(
      resolveInternalDocLink(new URL('https://example.com/docs/zh-CN/settings.md'), manifest),
    ).toBeNull()
  })
})
