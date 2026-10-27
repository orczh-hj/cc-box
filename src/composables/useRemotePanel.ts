/**
 * 全局远程连接面板开关。
 *
 * 面板由 App.vue 挂载一次，IconBar 与托盘“远程连接”共用同一状态：
 * 无论当前停留在项目页还是终端页，都能打开面板。
 */
import { ref } from 'vue'

const visible = ref(false)

export function useRemotePanel() {
  return {
    visible,
    open() {
      visible.value = true
    },
    close() {
      visible.value = false
    },
  }
}
