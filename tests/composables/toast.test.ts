import { describe, it, expect, vi, beforeEach, afterEach } from 'vitest'
import { mount } from '@vue/test-utils'

vi.mock('vue-i18n', () => ({
  useI18n: () => ({ t: (k: string) => k }),
}))

import { useToast, clearToasts } from '@/composables/useToast'
import ToastHost from '@/components/ToastHost.vue'

const toast = useToast()

beforeEach(() => {
  clearToasts()
  vi.useFakeTimers()
})

afterEach(() => {
  vi.useRealTimers()
  clearToasts()
})

describe('useToast', () => {
  it('Toast_PushAndAutoDismiss_1', () => {
    toast.success('ok')
    expect(toast.toasts.value).toHaveLength(1)
    expect(toast.toasts.value[0]).toMatchObject({ type: 'success', text: 'ok' })
    // success 默认 4s 自动消失
    vi.advanceTimersByTime(3999)
    expect(toast.toasts.value).toHaveLength(1)
    vi.advanceTimersByTime(1)
    expect(toast.toasts.value).toHaveLength(0)
  })

  it('Toast_ErrorLongerDuration_1', () => {
    toast.error('fail')
    // error 默认 6s
    vi.advanceTimersByTime(4000)
    expect(toast.toasts.value).toHaveLength(1)
    vi.advanceTimersByTime(2000)
    expect(toast.toasts.value).toHaveLength(0)
  })

  it('Toast_CustomDuration_1', () => {
    toast.info('hint', 1000)
    vi.advanceTimersByTime(1000)
    expect(toast.toasts.value).toHaveLength(0)
  })

  it('Toast_MaxVisibleDropsOldest_1', () => {
    const id1 = toast.success('1')
    toast.success('2')
    toast.success('3')
    toast.success('4') // 超过上限 3 → 最旧移除
    expect(toast.toasts.value).toHaveLength(3)
    expect(toast.toasts.value.some((t) => t.id === id1)).toBe(false)
    expect(toast.toasts.value.map((t) => t.text)).toEqual(['2', '3', '4'])
  })

  it('Toast_RemoveById_1', () => {
    const id = toast.success('closable')
    toast.remove(id)
    expect(toast.toasts.value).toHaveLength(0)
    // 已移除的定时器不应再影响后续
    vi.advanceTimersByTime(10_000)
    expect(toast.toasts.value).toHaveLength(0)
  })
})

describe('ToastHost', () => {
  it('ToastHost_RendersTypesAndClickClose_1', async () => {
    toast.success('done')
    toast.error('oops')
    const wrapper = mount(ToastHost)

    const items = wrapper.findAll('.toast-item')
    expect(items).toHaveLength(2)
    expect(items[0].classes()).toContain('toast-success')
    expect(items[1].classes()).toContain('toast-error')
    expect(items[0].attributes('role')).toBe('status')
    // 每条均带显式关闭按钮
    expect(items[0].find('.toast-close').exists()).toBe(true)

    // 点 × 立即关闭（断言数据层：DOM 移除受 TransitionGroup 过渡时序影响）
    await items[0].find('.toast-close').trigger('click')
    expect(toast.toasts.value).toHaveLength(1)
    expect(toast.toasts.value[0].text).toBe('oops')
  })

  it('ToastHost_EmptyRendersNothing_1', () => {
    const wrapper = mount(ToastHost)
    expect(wrapper.find('.toast-host').exists()).toBe(false)
  })
})
