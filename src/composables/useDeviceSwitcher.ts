/**
 * 全局设备切换弹窗开关。
 *
 * 弹窗由 App.vue 挂载一次；浏览器模式下 IconBar 与项目页共用同一入口。
 */
import { ref } from 'vue'

const visible = ref(false)

export function useDeviceSwitcher() {
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
