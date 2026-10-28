<template>
  <Teleport to="body">
    <Transition name="modal">
      <div v-if="visible" class="switcher-overlay" @click.self="close">
        <div class="switcher-card">
          <h3 class="switcher-title">{{ t('deviceSwitchTitle') }}</h3>
          <p v-if="desktops.length === 0" class="switcher-hint">{{ t('deviceSwitchEmpty') }}</p>
          <DeviceList
            v-else
            :desktops="desktops"
            :active-id="activeId"
            :conn-state="connState"
            :statuses="statuses"
            @select="select"
            @remove="remove"
            @recheck="recheck(desktops, activeId)"
          />
          <p v-if="connectingId" class="switcher-hint">{{ t('deviceConnecting') }}</p>
          <p v-if="connectError" class="switcher-error">{{ connectError }}</p>
          <p v-if="desktops.length === 1" class="switcher-hint">{{ t('deviceSwitchSameOriginHint') }}</p>
          <button class="switcher-close" @click="close">{{ t('close') }}</button>
        </div>
      </div>
    </Transition>
  </Teleport>
</template>

<script setup lang="ts">
import { ref, watch } from 'vue'
import { useI18n } from 'vue-i18n'
import DeviceList from './DeviceList.vue'
import { useDeviceSwitcher } from '@/composables/useDeviceSwitcher'
import { useDeviceProbe } from '@/composables/useDeviceProbe'
import { remoteTransport, type RemoteConnState } from '@/api/remote-transport'
import { listDesktops, removeDesktop, type StoredDesktop } from '@/api/remote-desktops'

const { t } = useI18n()
const { visible, close } = useDeviceSwitcher()
const { statuses, probeAll, recheck } = useDeviceProbe()

const desktops = ref<StoredDesktop[]>([])
const activeId = ref<string | null>(null)
const connState = ref<RemoteConnState>('closed')

/** 同 origin 切换的等待态：探测与真正建连之间设备仍可能失效 */
const connectingId = ref<string | null>(null)
const connectError = ref('')
let waitTimer: number | null = null
let unsubConnState: (() => void) | null = null

function clearWait() {
  if (waitTimer !== null) {
    window.clearTimeout(waitTimer)
    waitTimer = null
  }
  unsubConnState?.()
  unsubConnState = null
  connectingId.value = null
}

function refresh() {
  desktops.value = listDesktops()
  activeId.value = remoteTransport.activeDesktopId
  connState.value = remoteTransport.connState
}

// 每次打开都重读 localStorage：TokenGate 配对新增的设备要立即出现在列表里
// 并重新探测连通性（设备/中继状态可能已变化）
watch(visible, (open) => {
  if (open) {
    refresh()
    probeAll(desktops.value, activeId.value)
  } else {
    clearWait()
    connectError.value = ''
  }
})

function select(id: string) {
  clearWait()
  connectError.value = ''
  if (id === activeId.value) {
    close()
    return
  }
  const d = desktops.value.find(x => x.id === id)
  if (!d) return

  // 不同 origin 只能整页跳转；失败反馈由新页面的 TokenGate 提供
  if (d.origin.replace(/\/+$/, '') !== location.origin.replace(/\/+$/, '')) {
    close()
    remoteTransport.activateDesktop(id)
    return
  }

  // 同 origin：保持弹窗等连接结论，避免「点了没反馈」
  connectingId.value = id
  unsubConnState = remoteTransport.subscribeConnState((s) => {
    if (s === 'connected') {
      clearWait()
      close()
      return
    }
    if (s === 'denied') {
      connectError.value = t('deviceSwitchFailedInvalid')
      clearWait()
    }
  })
  waitTimer = window.setTimeout(() => {
    if (connectingId.value !== id) return
    connectError.value = t('deviceSwitchFailedTimeout')
    clearWait()
  }, 8_000)
  remoteTransport.activateDesktop(id)
}

function remove(id: string) {
  // 删除正在等待连接的设备：停止后台尝试，避免连上一台已被移除的配对
  if (id === connectingId.value) {
    remoteTransport.close()
    clearWait()
  }
  removeDesktop(id)
  refresh()
}
</script>

<style scoped>
.switcher-overlay {
  position: fixed;
  inset: 0;
  display: flex;
  align-items: center;
  justify-content: center;
  background: rgba(0, 0, 0, 0.35);
  z-index: 2100;
}
.switcher-card {
  display: flex;
  flex-direction: column;
  gap: 10px;
  width: min(360px, 86vw);
  padding: 22px 20px;
  background: var(--bg-secondary, #f5f3ee);
  border: 1px solid var(--border-light, #e8e4dc);
  border-radius: 12px;
}
.switcher-title {
  font-size: 16px;
  color: var(--text-primary, #1a1816);
}
.switcher-hint {
  font-size: 12px;
  line-height: 1.5;
  color: var(--text-secondary, #5a5550);
}
.switcher-error {
  font-size: 12px;
  line-height: 1.5;
  color: #e74c3c;
}
.switcher-close {
  padding: 10px;
  background: transparent;
  border: none;
  font-size: 13px;
  color: var(--text-secondary, #5a5550);
  cursor: pointer;
}
</style>
