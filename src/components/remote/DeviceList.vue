<template>
  <div class="device-list">
    <button
      v-for="d in desktops"
      :key="d.id"
      class="device-item"
      :class="{ active: d.id === activeId }"
      @click="$emit('select', d.id)"
    >
      <span class="dot" :class="dotClass(d)"></span>
      <span class="device-name">{{ d.name || fallbackName(d) }}</span>
      <span v-if="d.invalid" class="tag invalid">{{ t('deviceInvalid') }}</span>
      <span class="tag kind">{{ d.kind === 'lan' ? t('deviceKindLan') : t('deviceKindRelay') }}</span>
      <span
        class="device-remove"
        :title="t('deviceRemove')"
        @click.stop="$emit('remove', d.id)"
      >×</span>
    </button>
  </div>
</template>

<script setup lang="ts">
import { useI18n } from 'vue-i18n'
import type { StoredDesktop } from '@/api/remote-desktops'
import type { RemoteConnState } from '@/api/remote-transport'

const props = defineProps<{
  desktops: StoredDesktop[]
  activeId: string | null
  /** 当前激活设备的连接状态（TokenGate 用于着色；弹窗不传时按已连接处理） */
  connState?: RemoteConnState
}>()

defineEmits<{
  select: [id: string]
  remove: [id: string]
}>()

const { t } = useI18n()

function fallbackName(d: StoredDesktop): string {
  return t('deviceFallback', { id: d.id.slice(-6) })
}

function dotClass(d: StoredDesktop): string {
  if (d.invalid) return 'denied'
  if (d.id !== props.activeId) return 'unknown'
  switch (props.connState) {
    case 'connecting':
    case 'authenticating':
    case 'reconnecting':
      return 'connecting'
    case 'denied':
      return 'denied'
    case 'closed':
      return 'unknown'
    default:
      return 'connected'
  }
}
</script>

<style scoped>
.device-list {
  display: flex;
  flex-direction: column;
  gap: 6px;
  max-height: 240px;
  overflow-y: auto;
}
.device-item {
  display: flex;
  align-items: center;
  gap: 8px;
  padding: 10px;
  background: var(--bg-primary, #fff);
  border: 1px solid var(--border-color, #d9d5cc);
  border-radius: 8px;
  font-size: 14px;
  color: var(--text-primary, #1a1816);
  text-align: left;
  cursor: pointer;
}
.device-item.active {
  border-color: var(--accent-primary, #1e3a5f);
}
.dot {
  flex: none;
  width: 8px;
  height: 8px;
  border-radius: 50%;
  background: var(--border-color, #d9d5cc);
}
.dot.connected { background: #27ae60; }
.dot.connecting {
  background: #f39c12;
  animation: pulse 1s ease-in-out infinite;
}
.dot.denied { background: #e74c3c; }
@keyframes pulse {
  50% { opacity: 0.4; }
}
.device-name {
  flex: 1;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}
.tag {
  flex: none;
  font-size: 11px;
  padding: 1px 6px;
  border-radius: 4px;
  background: var(--bg-secondary, #f5f3ee);
  color: var(--text-secondary, #5a5550);
}
.tag.invalid {
  background: #fdecea;
  color: #e74c3c;
}
.device-remove {
  flex: none;
  width: 22px;
  height: 22px;
  line-height: 20px;
  border-radius: 4px;
  font-size: 15px;
  color: var(--text-secondary, #5a5550);
  text-align: center;
}
.device-remove:hover {
  background: #fdecea;
  color: #e74c3c;
}
</style>
