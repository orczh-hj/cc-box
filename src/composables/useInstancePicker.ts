/**
 * 多实例远程：新建/恢复会话的实例选择器（全局单例状态 + App.vue 挂载的
 * InstancePickerDialog 消费）。单实例/桌面模式不弹窗直接返回——用户无感。
 */
import { ref } from 'vue'
import { isTauriEnv } from '@/api/platform'

export interface PickableInstance {
  id: string
  name: string
}

const visible = ref(false)
const instances = ref<PickableInstance[]>([])
let resolver: ((id?: string) => void) | null = null

export function useInstancePicker() {
  /** 选择目标实例：≤1 个实例（或桌面模式）直接返回不弹窗 */
  function pick(list: PickableInstance[]): Promise<string | undefined> {
    if (isTauriEnv || list.length <= 1) {
      return Promise.resolve(list[0]?.id)
    }
    instances.value = list
    visible.value = true
    return new Promise(resolve => {
      // 上一次选择尚未完成（重复触发入口）：按取消结算，避免 Promise 永挂
      resolver?.(undefined)
      resolver = resolve
    })
  }

  function choose(id?: string) {
    visible.value = false
    resolver?.(id)
    resolver = null
  }

  return { visible, instances, pick, choose }
}
