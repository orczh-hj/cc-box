/**
 * 全局轻量通知（toast）：模块级单例队列，由 App.vue 挂载的 ToastHost 渲染。
 * 任意组件直接调用，无需各自的提示状态：
 *
 *   const toast = useToast()
 *   toast.success('已保存')
 *   toast.error('失败', 6000)
 *
 * 自动消失（success/info 4s，error 6s），点击可立即关闭；最多同时 3 条。
 */
import { readonly, ref } from 'vue'

export type ToastType = 'success' | 'info' | 'error'

export interface ToastItem {
  id: number
  type: ToastType
  text: string
}

const toasts = ref<ToastItem[]>([])
const timers = new Map<number, number>()
let seq = 0
const MAX_VISIBLE = 3

function remove(id: number): void {
  const timer = timers.get(id)
  if (timer !== undefined) {
    window.clearTimeout(timer)
    timers.delete(id)
  }
  toasts.value = toasts.value.filter((t) => t.id !== id)
}

function push(type: ToastType, text: string, durationMs?: number): number {
  const id = ++seq
  toasts.value.push({ id, type, text })
  // 超出上限移除最旧（remove 内部会清 timer）
  while (toasts.value.length > MAX_VISIBLE) remove(toasts.value[0].id)
  const ms = durationMs ?? (type === 'error' ? 6000 : 4000)
  timers.set(id, window.setTimeout(() => remove(id), ms))
  return id
}

export function useToast() {
  return {
    toasts: readonly(toasts),
    success: (text: string, durationMs?: number) => push('success', text, durationMs),
    info: (text: string, durationMs?: number) => push('info', text, durationMs),
    error: (text: string, durationMs?: number) => push('error', text, durationMs),
    remove,
  }
}

/** 清空全部通知（测试隔离用） */
export function clearToasts(): void {
  for (const id of [...timers.keys()]) remove(id)
  toasts.value = []
}
