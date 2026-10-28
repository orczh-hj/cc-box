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
            @select="select"
            @remove="remove"
          />
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
import { remoteTransport, type RemoteConnState } from '@/api/remote-transport'
import { listDesktops, removeDesktop, type StoredDesktop } from '@/api/remote-desktops'

const { t } = useI18n()
const { visible, close } = useDeviceSwitcher()

const desktops = ref<StoredDesktop[]>([])
const activeId = ref<string | null>(null)
const connState = ref<RemoteConnState>('closed')

function refresh() {
  desktops.value = listDesktops()
  activeId.value = remoteTransport.activeDesktopId
  connState.value = remoteTransport.connState
}

// 每次打开都重读 localStorage：TokenGate 配对新增的设备要立即出现在列表里
watch(visible, (open) => {
  if (open) refresh()
})

function select(id: string) {
  if (id !== activeId.value) remoteTransport.activateDesktop(id)
  close()
}

function remove(id: string) {
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
.switcher-close {
  padding: 10px;
  background: transparent;
  border: none;
  font-size: 13px;
  color: var(--text-secondary, #5a5550);
  cursor: pointer;
}
</style>
