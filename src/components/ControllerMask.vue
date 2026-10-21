<template>
  <Transition name="controller-mask-fade">
    <div v-if="masked" class="controller-mask">
      <div class="controller-mask-card">
        <div class="controller-mask-icon">⌨</div>
        <div class="controller-mask-title">{{ t('controllerMaskTitle') }}</div>
        <div class="controller-mask-hint">{{ t('controllerMaskHint') }}</div>
        <button class="controller-mask-btn" @click="handleTakeover">
          {{ t('controllerTakeover') }}
        </button>
      </div>
    </div>
  </Transition>
</template>

<script setup lang="ts">
/**
 * 全局控制权遮罩（设备互斥：桌面与所有远程设备对等参与）。
 * 其他设备（含桌面）持有控制权时整界面遮罩 + 「接管」按钮：
 * - 桌面端：遮罩经 Tauri 事件 controller-changed 驱动；接管调
 *   controller_takeover command（桌面普通操作不抢占远程）
 * - 浏览器端：遮罩经 rpc.event controller-changed 驱动；接管发
 *   controller.takeover 协议消息；接管确认按"重连"语义全量重同步
 * 控制权由 Rust 后端裁决（attach 不能抢占、写操作被服务端拦截），本组件只做展示
 */
import { ref, computed, onMounted, onUnmounted } from 'vue'
import { useI18n } from 'vue-i18n'
import { transport } from '@/api/transport'
import { remoteTransport } from '@/api/remote-transport'
import { controllerTakeover, controllerStatus } from '@/api/tauri'
import { isTauriEnv } from '@/api/platform'

const { t } = useI18n()

/** 本端控制者标签：桌面 = 'desktop'；浏览器 = 连接 id。
 * 响应式维护：重连后服务端重新分配连接 id（旧 id 永不复用），setup 快照
 * 会过期——自己持有/接管确认时新 id ≠ 旧快照 → 自己遮自己且接管解不开 */
const selfLabel = ref(isTauriEnv ? 'desktop' : String(remoteTransport.connId ?? ''))

/** 当前控制者标签（''=无人持有）；hello.ok 初始快照 + controller-changed 维护 */
const controllerLabel = ref(isTauriEnv ? '' : remoteTransport.controllerLabel)

let unlistenController: (() => void) | null = null
let unlistenConnected: (() => void) | null = null

/** 遮罩条件：控制者是其他设备（''=无人持有不遮罩） */
const masked = computed(() => {
  if (controllerLabel.value === '' || controllerLabel.value === selfLabel.value) return false
  return true
})

/** 接管：桌面调 command（显式接管）；浏览器发协议消息（服务端剥离旧持有者并广播） */
function handleTakeover() {
  void controllerTakeover()
}

onMounted(async () => {
  // 桌面端补初始快照：controller-changed 只在变化时广播，桌面 UI 启动/重载
  // （如切 Tab reload）会错过历史事件——挂载时主动查询当前持有者
  if (isTauriEnv) {
    try {
      controllerLabel.value = await controllerStatus()
    } catch {
      // 查询失败不阻塞：后续 controller-changed 事件仍可纠偏
    }
  }
  unlistenController = await transport.listen<{ id: string }>('controller-changed', ({ id }) => {
    controllerLabel.value = id ?? ''
  })
  // 浏览器端重连/接管确认后 hello.ok 重新分配快照，刷新本地缓存
  // （connId 与 controllerLabel 都要刷新：connId 每次重连重新分配）
  unlistenConnected = await transport.listen('remote-connected', () => {
    if (!isTauriEnv) {
      selfLabel.value = String(remoteTransport.connId ?? '')
      controllerLabel.value = remoteTransport.controllerLabel
    }
  })
})

onUnmounted(() => {
  unlistenController?.()
  unlistenConnected?.()
})
</script>

<style scoped>
.controller-mask {
  position: fixed;
  inset: 0;
  z-index: 2000; /* 高于终端级 taken-over 遮罩（两层并存，全局优先） */
  display: flex;
  align-items: center;
  justify-content: center;
  background: color-mix(in srgb, var(--bg-primary) 78%, transparent);
  backdrop-filter: blur(2px);
}

.controller-mask-card {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 10px;
  padding: 28px 36px;
  background: var(--bg-secondary);
  border: 1px solid var(--border-color);
  border-radius: var(--radius-lg, 10px);
  box-shadow: 0 8px 32px rgba(0, 0, 0, 0.35);
  max-width: 320px;
  text-align: center;
}

.controller-mask-icon {
  font-size: 28px;
  opacity: 0.7;
}

.controller-mask-title {
  font-size: 15px;
  font-weight: 600;
  color: var(--text-primary);
}

.controller-mask-hint {
  font-size: 12px;
  color: var(--text-secondary);
  line-height: 1.5;
}

.controller-mask-btn {
  margin-top: 6px;
  padding: 8px 28px;
  border: 1px solid var(--accent-color);
  border-radius: var(--radius-md, 6px);
  background: var(--bg-primary);
  color: var(--accent-color);
  font-size: 13px;
  font-weight: 500;
  cursor: pointer;
  transition: all 0.15s ease;
}

.controller-mask-btn:hover {
  background: var(--accent-color);
  color: var(--bg-primary);
}

.controller-mask-btn:active {
  transform: scale(0.97);
}

.controller-mask-fade-enter-active,
.controller-mask-fade-leave-active {
  transition: opacity 0.18s ease;
}

.controller-mask-fade-enter-from,
.controller-mask-fade-leave-to {
  opacity: 0;
}
</style>
