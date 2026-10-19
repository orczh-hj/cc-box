import { ref, onMounted, onUnmounted } from 'vue'
import { isTauriEnv } from '@/api/platform'

export function useWindowAttention() {
  // 浏览器模式无窗口概念，恒视为聚焦
  const isFocused = ref(true)
  let unlistenFocus: (() => void) | null = null
  let win: import('@tauri-apps/api/window').Window | null = null

  onMounted(async () => {
    if (!isTauriEnv) return
    const { getCurrentWindow } = await import('@tauri-apps/api/window')
    win = getCurrentWindow()

    isFocused.value = await win.isFocused()

    unlistenFocus = await win.onFocusChanged(({ payload: focused }) => {
      isFocused.value = focused
      if (focused) {
        // 聚焦时取消任务栏跳动
        win?.requestUserAttention(null).catch(() => {})
      }
    })
  })

  onUnmounted(() => {
    unlistenFocus?.()
    win?.requestUserAttention(null).catch(() => {})
  })

  return { isFocused }
}
