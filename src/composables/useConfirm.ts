import { reactive } from 'vue'

/**
 * 统一前端确认框（替代 window.confirm）：
 * 浏览器原生 confirm 在远程 WebView 上样式突兀、不可定制、部分内嵌浏览器被禁用。
 * App.vue 挂载一次 <ConfirmDialog />，各处 `await uiConfirm(message, { danger: true })`。
 */
export interface ConfirmOptions {
  title?: string
  /** 危险操作（删除/卸载）：确认按钮红色 */
  danger?: boolean
  okText?: string
}

interface ConfirmState {
  visible: boolean
  title: string
  message: string
  danger: boolean
  okText: string
  resolve: (v: boolean) => void
}

export const confirmState = reactive<ConfirmState>({
  visible: false,
  title: '',
  message: '',
  danger: false,
  okText: '',
  resolve: () => {},
})

/** 弹出确认框；resolve(true) = 确认，resolve(false) = 取消/Esc/遮罩 */
export function uiConfirm(message: string, opts?: ConfirmOptions): Promise<boolean> {
  // 前一个未关闭时先取消，避免其 resolve 永远挂起
  if (confirmState.visible) confirmState.resolve(false)
  return new Promise(resolve => {
    confirmState.message = message
    confirmState.title = opts?.title ?? ''
    confirmState.danger = opts?.danger ?? false
    confirmState.okText = opts?.okText ?? ''
    confirmState.resolve = resolve
    confirmState.visible = true
  })
}

/** 仅由 ConfirmDialog 调用 */
export function settleConfirm(v: boolean) {
  if (!confirmState.visible) return
  confirmState.visible = false
  confirmState.resolve(v)
}
