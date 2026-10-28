<template>
  <div class="device-list">
    <div
      v-for="d in desktops"
      :key="d.id"
      class="device-item"
      :class="{ active: d.id === activeId }"
      role="button"
      tabindex="0"
      :aria-current="d.id === activeId ? 'true' : undefined"
      @click="select(d.id)"
      @keydown.enter.self="select(d.id)"
      @keydown.space.prevent.self="select(d.id)"
    >
      <span class="dot" :class="dotClass(d)"></span>
      <span class="device-name">{{ d.name || fallbackName(d) }}</span>
      <span v-if="d.invalid" class="tag invalid">{{ t('deviceInvalid') }}</span>
      <span class="tag kind">{{ d.kind === 'lan' ? t('deviceKindLan') : t('deviceKindRelay') }}</span>
      <!-- 正在连接/已连接的当前设备不可移除；denied/closed 仍可移除 -->
      <template v-if="canRemove(d)">
        <span v-if="pendingRemoveId === d.id" class="device-confirm" @click.stop>
          <button class="confirm-cancel" @click.stop="pendingRemoveId = null">{{ t('cancel') }}</button>
          <button class="confirm-ok" @click.stop="confirmRemove(d.id)">{{ t('remove') }}</button>
        </span>
        <button
          v-else
          class="device-remove"
          :title="t('deviceRemove')"
          @click.stop="pendingRemoveId = d.id"
        >×</button>
      </template>
    </div>
    <p v-if="pendingRemove" class="device-confirm-hint">{{ t('deviceRemoveConfirm') }}</p>
  </div>
</template>

<script setup lang="ts">
import { computed, ref, watch } from 'vue'
import { useI18n } from 'vue-i18n'
import type { StoredDesktop } from '@/api/remote-desktops'
import type { RemoteConnState } from '@/api/remote-transport'

const props = defineProps<{
  desktops: StoredDesktop[]
  activeId: string | null
  /** 当前激活设备的连接状态（TokenGate 用于着色；弹窗不传时按已连接处理） */
  connState?: RemoteConnState
}>()

const emit = defineEmits<{
  select: [id: string]
  remove: [id: string]
}>()

const { t } = useI18n()

/** 二次确认：第一次点 × 只进入待移除态，点「移除」才真正删除配对 */
const pendingRemoveId = ref<string | null>(null)
const pendingRemove = computed(
  () => props.desktops.find(d => d.id === pendingRemoveId.value) ?? null,
)

// 外部刷新列表（删除/切换设备）时，待确认项已不存在则清除
watch(() => props.desktops, (list) => {
  if (pendingRemoveId.value && !list.some(d => d.id === pendingRemoveId.value)) {
    pendingRemoveId.value = null
  }
})

// 待确认设备变成当前连接设备（外部切换）时取消移除
watch(() => props.activeId, (id) => {
  if (id && pendingRemoveId.value === id) pendingRemoveId.value = null
})

function select(id: string) {
  pendingRemoveId.value = null
  emit('select', id)
}

function confirmRemove(id: string) {
  pendingRemoveId.value = null
  emit('remove', id)
}

/** 当前设备在已连接/连接中不可移除；denied/closed 表示并未连上，允许移除。
 *  connState 缺省（调用方未提供）按已连接处理，保守保护当前设备。 */
function canRemove(d: StoredDesktop): boolean {
  if (d.id !== props.activeId) return true
  return props.connState === 'denied' || props.connState === 'closed'
}

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
  user-select: none;
}
.device-item.active {
  border-color: var(--accent-primary, #1e3a5f);
}
.device-item:focus-visible {
  outline: 2px solid var(--accent-primary, #1e3a5f);
  outline-offset: 1px;
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
  padding: 0;
  line-height: 20px;
  border: none;
  border-radius: 4px;
  background: transparent;
  font-size: 15px;
  color: var(--text-secondary, #5a5550);
  text-align: center;
  cursor: pointer;
}
.device-remove:hover {
  background: #fdecea;
  color: #e74c3c;
}
.device-confirm {
  flex: none;
  display: flex;
  gap: 4px;
}
.confirm-cancel,
.confirm-ok {
  padding: 4px 8px;
  border-radius: 4px;
  font-size: 11px;
  cursor: pointer;
}
.confirm-cancel {
  border: 1px solid var(--border-color, #d9d5cc);
  background: transparent;
  color: var(--text-secondary, #5a5550);
}
.confirm-ok {
  border: 1px solid #e74c3c;
  background: #e74c3c;
  color: #fff;
}
.device-confirm-hint {
  margin: 2px 2px 0;
  font-size: 11px;
  line-height: 1.5;
  color: #e74c3c;
}
</style>
