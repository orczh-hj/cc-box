import { describe, it, expect, vi } from 'vitest'
import { mount } from '@vue/test-utils'

const mockT = vi.fn((key: string) => key)
vi.mock('vue-i18n', () => ({
  useI18n: () => ({ t: mockT }),
}))

import TerminalInputBox from '@/components/TerminalInputBox.vue'

// Teleport stub 原地渲染（否则内容挂到 document.body，wrapper 内查不到）
function mountBox(props: { visible: boolean; draft: string }) {
  return mount(TerminalInputBox, {
    props,
    global: { stubs: { teleport: true } },
  })
}

describe('TerminalInputBox（输入中转框）', () => {
  // 可见时渲染：textarea 呈现草稿全文（粘贴/复制由系统原生菜单接管，组件不干预）
  it('InputBox_RendersTextareaWithDraft_001', () => {
    const wrapper = mountBox({ visible: true, draft: 'hello world' })
    const ta = wrapper.find('textarea')
    expect(ta.exists()).toBe(true)
    expect((ta.element as HTMLTextAreaElement).value).toBe('hello world')
  })

  // invisible 不渲染（v-if + Teleport 由父组件控制）
  it('InputBox_HiddenWhenInvisible_002', () => {
    const wrapper = mountBox({ visible: false, draft: '' })
    expect(wrapper.find('.ib-mask').exists()).toBe(false)
  })

  // 输入 → update:draft 双向绑定（草稿由父组件持有，弹层卸载不丢）
  it('InputBox_InputEmitsUpdateDraft_003', async () => {
    const wrapper = mountBox({ visible: true, draft: '' })
    const ta = wrapper.find('textarea')
    ;(ta.element as HTMLTextAreaElement).value = 'npm run dev'
    await ta.trigger('input')
    expect(wrapper.emitted('update:draft')![0]).toEqual(['npm run dev'])
  })

  // 上屏：emit commit 且携带草稿全文；空内容不上屏（按钮禁用）
  it('InputBox_CommitEmitsFullDraft_004', async () => {
    const wrapper = mountBox({ visible: true, draft: '' })
    const commitBtn = wrapper.find('.ib-btn.primary')
    expect(commitBtn.attributes('disabled')).toBeDefined()

    await wrapper.setProps({ draft: 'line1\nline2' })
    const enabledBtn = wrapper.find('.ib-btn.primary')
    expect((enabledBtn.element as HTMLButtonElement).disabled).toBe(false)
    await enabledBtn.trigger('click')
    expect(wrapper.emitted('commit')![0]).toEqual(['line1\nline2'])
  })

  // 清空：emit update:draft 空串（草稿清除但不关闭弹层）
  it('InputBox_ClearEmitsEmptyDraft_005', async () => {
    const wrapper = mountBox({ visible: true, draft: 'to be cleared' })
    await wrapper.findAll('.ib-btn.ghost')[0].trigger('click')
    expect(wrapper.emitted('update:draft')![0]).toEqual([''])
    expect(wrapper.emitted('close')).toBeUndefined()
  })

  // 空内容点按钮不 emit commit（disabled 挡住）
  it('InputBox_EmptyDraftNoCommit_005', async () => {
    const wrapper = mountBox({ visible: true, draft: '' })
    await wrapper.find('.ib-btn.primary').trigger('click')
    expect(wrapper.emitted('commit')).toBeUndefined()
  })

  // 关闭：右上角「×」与点遮罩（click.self）都 emit close；点弹层内部不关闭
  it('InputBox_CloseEmits_006', async () => {
    const wrapper = mountBox({ visible: true, draft: '' })
    // 关闭在右上角顶栏，位于 textarea 外部
    await wrapper.find('.ib-close').trigger('click')
    expect(wrapper.emitted('close')).toHaveLength(1)

    // 点弹层内部（冒泡到 mask）不触发 .self → 不关闭
    await wrapper.find('.ib-sheet').trigger('click')
    expect(wrapper.emitted('close')).toHaveLength(1)

    await wrapper.find('.ib-mask').trigger('click')
    expect(wrapper.emitted('close')).toHaveLength(2)
  })
})
