import { describe, it, expect, vi, beforeEach } from 'vitest'
import { mount } from '@vue/test-utils'

const mockT = vi.fn((key: string) => key)
vi.mock('vue-i18n', () => ({
  useI18n: () => ({ t: mockT }),
}))

const openExternal = vi.fn()
vi.mock('@/api/platform', () => ({
  openExternal: (...args: unknown[]) => openExternal(...args),
}))

import DocsHelpButton from '@/components/sidebar/DocsHelpButton.vue'

// 五个侧边栏面板 → 官网 docs.html（claude-code.md 文档）章节锚点；
// 锚点 = claude-code.md 中 h2 标题原文（markdown.ts slug 规则），改章节名须同步这里
describe('DocsHelpButton（侧边栏面板帮助链接）', () => {
  beforeEach(() => {
    mockT.mockClear()
    openExternal.mockClear()
  })

  it('DocsHelpButton_BuildsAnchorUrl_001', () => {
    const wrapper = mount(DocsHelpButton, { props: { anchor: 'Skills' } })
    expect(wrapper.find('button').attributes('title')).toBe('viewDocs')
    // 未点击不打开链接
    expect(openExternal).not.toHaveBeenCalled()
  })

  it('DocsHelpButton_OpensDocsOnClick_002', async () => {
    const wrapper = mount(DocsHelpButton, { props: { anchor: 'Subagents' } })
    await wrapper.find('button').trigger('click')
    expect(openExternal).toHaveBeenCalledWith('https://ccbox.orczh.top/docs.html#Subagents')
  })

  // 官网域名固定为正式域名 ccbox.orczh.top（非 OSS 默认域名，后者对网页强制下载）
  it('DocsHelpButton_UsesCanonicalDomain_003', async () => {
    const wrapper = mount(DocsHelpButton, { props: { anchor: 'MCP' } })
    await wrapper.find('button').trigger('click')
    const url = openExternal.mock.calls[0][0] as string
    expect(url.startsWith('https://ccbox.orczh.top/docs.html#')).toBe(true)
  })
})
