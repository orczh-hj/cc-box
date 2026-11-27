<template>
  <Teleport to="body">
    <Transition name="modal">
      <div v-if="visible" class="conn-overlay" :class="{ stalled: isStalled }">
        <div class="conn-card">
          <div class="spinner" aria-hidden="true"></div>
          <h3 class="conn-title">{{ t(isStalled ? 'connStalledTitle' : 'connLostTitle') }}</h3>
          <p class="conn-hint">{{ hint }}</p>
          <div v-if="isStalled" class="conn-actions">
            <button class="conn-switch-btn" @click="openSwitcher">
              {{ t('connSwitchDevice') }}
            </button>
            <button class="conn-reload-btn" @click="reloadPage">
              {{ t('connReloadPage') }}
            </button>
          </div>
        </div>
      </div>
    </Transition>
  </Teleport>
</template>

<script setup lang="ts">
/**
 * 浏览器远程模式：断开连接的页面级提示（App.vue 挂载一份）。
 *
 * 断开超 10s 弹出半透明覆盖（自动重连进行中，恢复即自动关闭、零用户操作）；
 * 超 60s 升级为强提示并给「切换设备」出口——自动重连仍在后台继续，出口只是
 * 兜底而非恢复的唯一路径。不提供「强制刷新」：刷新不解决传输层问题且丢失
 * 终端现场。桌面 Tauri / 票据模式不挂载本组件。
 *
 * 另承接 pair-failed：当前连接可用时收到失败的新配对，连接不打断，只提示。
 */
import { computed, onMounted, onUnmounted } from 'vue'
import { useI18n } from 'vue-i18n'
import { transport } from '@/api/transport'
import { useConnHealth } from '@/composables/useConnHealth'
import { useDeviceSwitcher } from '@/composables/useDeviceSwitcher'
import { useToast } from '@/composables/useToast'

const { t } = useI18n()
const { level, disconnectedMs } = useConnHealth()
const { open } = useDeviceSwitcher()
const toast = useToast()

let unlistenPairFailed: (() => void) | null = null
onMounted(async () => {
  unlistenPairFailed = await transport.listen('pair-failed', () => {
    toast.error(t('gatePairFailedToast'))
  })
})
onUnmounted(() => {
  unlistenPairFailed?.()
})

const visible = computed(() => level.value === 'overlay' || level.value === 'stalled')
const isStalled = computed(() => level.value === 'stalled')

const hint = computed(() => {
  const seconds = Math.floor(disconnectedMs.value / 1000)
  return isStalled.value
    ? t('connStalledHint', { seconds })
    : t('connLostHint', { seconds })
})

function openSwitcher(): void {
  open()
}

/** 刷新兜底：自动重连失败过久时给用户整页重置的出口（终端会话在桌面端不受影响） */
function reloadPage(): void {
  location.reload()
}
</script>

<style scoped>
.conn-overlay {
  position: fixed;
  inset: 0;
  display: flex;
  align-items: center;
  justify-content: center;
  background: rgba(0, 0, 0, 0.35);
  z-index: 1900;
}
.conn-overlay.stalled {
  background: rgba(0, 0, 0, 0.55);
}
.conn-card {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 10px;
  width: min(320px, 84vw);
  padding: 24px 20px;
  background: var(--bg-secondary, #f5f3ee);
  border: 1px solid var(--border-light, #e8e4dc);
  border-radius: 12px;
  text-align: center;
}
.spinner {
  width: 26px;
  height: 26px;
  border: 3px solid var(--border-light, #e8e4dc);
  border-top-color: var(--accent-color, #4a9eff);
  border-radius: 50%;
  animation: conn-spin 0.9s linear infinite;
}
@keyframes conn-spin {
  to { transform: rotate(360deg); }
}
.conn-title {
  margin: 0;
  font-size: 15px;
  color: var(--text-primary, #1a1816);
}
.conn-hint {
  margin: 0;
  font-size: 12px;
  line-height: 1.6;
  color: var(--text-secondary, #5a5550);
}
.conn-actions {
  display: flex;
  gap: 10px;
  margin-top: 4px;
}

.conn-switch-btn {
  padding: 8px 18px;
  border: none;
  border-radius: 8px;
  background: var(--accent-color, #4a9eff);
  color: #fff;
  font-size: 13px;
  cursor: pointer;
}

.conn-reload-btn {
  padding: 8px 18px;
  border: 1px solid var(--border-light, #e8e4dc);
  border-radius: 8px;
  background: transparent;
  color: var(--text-primary, #1a1816);
  font-size: 13px;
  cursor: pointer;
}
</style>
