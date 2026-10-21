import { describe, it, expect, vi, beforeEach, afterEach } from 'vitest'
import { mount } from '@vue/test-utils'

const mockT = vi.fn((key: string) => key)
vi.mock('vue-i18n', () => ({
  useI18n: () => ({ t: mockT }),
}))

const { mockCopyText } = vi.hoisted(() => ({ mockCopyText: vi.fn().mockResolvedValue(undefined) }))
vi.mock('@/api/platform', () => ({
  copyText: mockCopyText,
}))

import TerminalCopyReader from '@/components/TerminalCopyReader.vue'

const PROPS = {
  visible: true,
  smart: 'smart paragraph text',
  raw: 'smart paragraph\ntext',
  truncated: false,
}

/** 可控的 window.getSelection stub */
function stubSelection(text: string) {
  const sel = { rangeCount: text ? 1 : 0, toString: () => text }
  vi.spyOn(window, 'getSelection').mockReturnValue(sel as unknown as Selection)
}

function mountReader(props: Partial<typeof PROPS> = {}) {
  return mount(TerminalCopyReader, { props: { ...PROPS, ...props } })
}

/** 顶栏按钮（按渲染顺序：智能/原始为模式钮，其后 复制选中/复制全部/退出） */
function actionBtn(wrapper: ReturnType<typeof mountReader>, index: number) {
  const btns = wrapper.findAll('.cr-actions button')
  return btns[index]
}

describe('TerminalCopyReader（复制阅读器）', () => {
  beforeEach(() => {
    mockT.mockClear()
    mockCopyText.mockClear()
    stubSelection('')
  })
  afterEach(() => {
    vi.restoreAllMocks()
  })

  it('CopyReader_默认智能模式渲染_001：pre 显示 smart 文本', () => {
    const wrapper = mountReader()
    expect(wrapper.find('pre').text()).toBe(PROPS.smart)
  })

  it('CopyReader_模式切换_002：切原始后 pre 显示 raw 文本', async () => {
    const wrapper = mountReader()
    await wrapper.findAll('.cr-mode-btn')[1].trigger('click')
    expect(wrapper.find('pre').text()).toBe(PROPS.raw)
  })

  it('CopyReader_空选区禁用_003：无选区时复制选中禁用且不调 copyText', async () => {
    const wrapper = mountReader()
    const btn = actionBtn(wrapper, 0)
    expect(btn.attributes('disabled')).toBeDefined()
    await btn.trigger('click')
    expect(mockCopyText).not.toHaveBeenCalled()
  })

  it('CopyReader_复制选中_004：selectionchange 后可点，传选中文本', async () => {
    const wrapper = mountReader()
    stubSelection('selected text')
    document.dispatchEvent(new Event('selectionchange'))
    await wrapper.vm.$nextTick()
    const btn = actionBtn(wrapper, 0)
    expect((btn.element as HTMLButtonElement).disabled).toBe(false)
    await btn.trigger('click')
    expect(mockCopyText).toHaveBeenCalledWith('selected text')
  })

  it('CopyReader_复制全部按模式_005：smart 模式传 smart，切 raw 后传 raw', async () => {
    const wrapper = mountReader()
    await actionBtn(wrapper, 1).trigger('click')
    expect(mockCopyText).toHaveBeenCalledWith(PROPS.smart)
    mockCopyText.mockClear()
    await wrapper.findAll('.cr-mode-btn')[1].trigger('click')
    await actionBtn(wrapper, 1).trigger('click')
    expect(mockCopyText).toHaveBeenCalledWith(PROPS.raw)
  })

  it('CopyReader_复制后文案反馈_006：2 秒内显示已复制后恢复', async () => {
    vi.useFakeTimers()
    try {
      const wrapper = mountReader()
      await actionBtn(wrapper, 1).trigger('click')
      expect(actionBtn(wrapper, 1).text()).toBe('selectModeCopied')
      vi.advanceTimersByTime(2100)
      await wrapper.vm.$nextTick()
      expect(actionBtn(wrapper, 1).text()).toBe('copyReaderCopyAll')
    } finally {
      vi.useRealTimers()
    }
  })

  it('CopyReader_退出事件_007：退出按钮 emit close', async () => {
    const wrapper = mountReader()
    await actionBtn(wrapper, 2).trigger('click')
    expect(wrapper.emitted('close')).toHaveLength(1)
  })

  it('CopyReader_截断提示_008：truncated 时显示提示条', () => {
    const wrapper = mountReader({ truncated: true })
    expect(wrapper.find('.cr-truncated').exists()).toBe(true)
    expect(wrapper.find('.cr-truncated').text()).toBe('copyReaderTruncated')
  })

  it('CopyReader_Contextmenu放行_009：根元素 contextmenu 不冒泡到 window', async () => {
    const windowSpy = vi.fn()
    window.addEventListener('contextmenu', windowSpy)
    try {
      const wrapper = mountReader()
      await wrapper.find('.copy-reader').trigger('contextmenu')
      expect(windowSpy).not.toHaveBeenCalled()
    } finally {
      window.removeEventListener('contextmenu', windowSpy)
    }
  })
})
