<template>
  <div class="device-list">
    <div
      v-for="d in desktops"
      :key="d.id"
      class="device-item"
      :class="{ active: d.id === activeId, blocked: blockedId === d.id }"
      role="button"
      tabindex="0"
      :aria-current="d.id === activeId ? 'true' : undefined"
      @click="select(d)"
      @keydown.enter.self="select(d)"
      @keydown.space.prevent.self="select(d)"
    >
      <span class="dot" :class="dotClass(d)"></span>
      <span class="device-name">{{ d.name || fallbackName(d) }}</span>
      <span v-if="d.invalid && displayStatus(d) !== 'auth-failed'" class="tag invalid">{{ t('deviceInvalid') }}</span>
      <span class="tag kind">{{ d.kind === 'lan' ? t('deviceKindLan') : t('deviceKindRelay') }}</span>
      <span v-if="statusLabel(d)" class="tag status" :class="`status-${displayStatus(d)}`">
        {{ statusLabel(d) }}
      </span>
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
      <span v-if="blockedId === d.id" class="device-blocked">{{ blockedHint(d) }}</span>
    </div>
    <button v-if="desktops.length > 0" class="device-recheck" @click="$emit('recheck')">
      {{ t('deviceRecheck') }}
    </button>
    <p v-if="pendingRemove" class="device-confirm-hint">{{ t('deviceRemoveConfirm') }}</p>
  </div>
</template>

<script setup lang="ts">
import { computed, ref, watch } from 'vue'
import { useI18n } from 'vue-i18n'
import type { StoredDesktop } from '@/api/remote-desktops'
import type { RemoteConnState } from '@/api/remote-transport'
import type { ProbeStatus } from '@/api/remote-probe'
import { probeKey } from '@/composables/useDeviceProbe'

const props = defineProps<{
  desktops: StoredDesktop[]
  activeId: string | null
  /** 当前激活设备的连接状态（TokenGate / 弹窗用于着色） */
  connState?: RemoteConnState
  /** 非当前设备的连通性探测结果（key = probeKey） */
  statuses?: Record<string, ProbeStatus>
}>()

const emit = defineEmits<{
  select: [id: string]
  remove: [id: string]
  recheck: []
}>()

const { t } = useI18n()

/** 展示状态：探测结果 + 当前设备的主连接状态 */
type DisplayStatus = ProbeStatus | 'reconnecting' | 'unknown'

/** 二次确认：第一次点 × 只进入待移除态，点「移除」才真正删除配对 */
const pendingRemoveId = ref<string | null>(null)
const pendingRemove = computed(
  () => props.desktops.find(d => d.id === pendingRemoveId.value) ?? null,
)

/** 点击不可达设备：不切换，行内说明原因（删除入口仍可用） */
const blockedId = ref<string | null>(null)
const BLOCKED: DisplayStatus[] = ['auth-failed', 'offline', 'unreachable']

// 外部刷新列表（删除/切换设备）时，待确认项已不存在则清除
watch(() => props.desktops, (list) => {
  if (pendingRemoveId.value && !list.some(d => d.id === pendingRemoveId.value)) {
    pendingRemoveId.value = null
  }
  if (blockedId.value && !list.some(d => d.id === blockedId.value)) {
    blockedId.value = null
  }
})

// 待确认设备变成当前连接设备（外部切换）时取消移除
watch(() => props.activeId, (id) => {
  if (id && pendingRemoveId.value === id) pendingRemoveId.value = null
})

// 重新检测后设备恢复可达 → 收起拦截提示；检测中（checking）保持，
// 否则复测仍不可达时提示会闪没、需要用户再点一次
const blockedStillBlocked = computed(() => {
  const d = props.desktops.find(x => x.id === blockedId.value)
  if (!d) return false
  return isBlocked(d) || displayStatus(d) === 'checking'
})
watch(blockedStillBlocked, (v) => {
  if (!v) blockedId.value = null
})

function displayStatus(d: StoredDesktop): DisplayStatus {
  if (d.id === props.activeId) {
    switch (props.connState) {
      case 'connected':
        return 'online'
      case 'connecting':
      case 'authenticating':
        return 'checking'
      case 'reconnecting':
        return 'reconnecting'
      case 'denied':
        return 'auth-failed'
      default:
        return 'unknown'
    }
  }
  return props.statuses?.[probeKey(d)] ?? 'unknown'
}

function statusLabel(d: StoredDesktop): string {
  switch (displayStatus(d)) {
    case 'checking': return t('deviceStatusChecking')
    case 'online': return t('deviceStatusOnline')
    case 'offline': return t('deviceStatusOffline')
    case 'auth-failed': return t('deviceStatusAuthFailed')
    case 'unreachable': return t('deviceStatusUnreachable')
    case 'unsupported': return t('deviceStatusUnsupported')
    case 'reconnecting': return t('deviceStatusReconnecting')
    default: return ''
  }
}

function isBlocked(d: StoredDesktop): boolean {
  return d.id !== props.activeId && BLOCKED.includes(displayStatus(d))
}

function blockedHint(d: StoredDesktop): string {
  switch (displayStatus(d)) {
    case 'auth-failed': return t('deviceBlockedAuthFailed')
    case 'offline': return t('deviceBlockedOffline')
    default: return t('deviceBlockedUnreachable')
  }
}

function select(d: StoredDesktop): void {
  pendingRemoveId.value = null
  if (isBlocked(d)) {
    blockedId.value = d.id
    return
  }
  blockedId.value = null
  emit('select', d.id)
}

function confirmRemove(id: string): void {
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
  switch (displayStatus(d)) {
    case 'online':
      return 'connected'
    case 'checking':
    case 'reconnecting':
      return 'connecting'
    case 'auth-failed':
      return 'denied'
    case 'offline':
    case 'unreachable':
      return 'offline'
    default:
      return 'unknown'
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
  flex-wrap: wrap;
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
.device-item.blocked {
  border-color: #e74c3c;
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
.dot.offline { background: #95a5a6; }
@keyframes pulse {
  50% { opacity: 0.4; }
}
.device-name {
  flex: 1;
  min-width: 0;
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
.tag.status-online {
  background: #e8f5e9;
  color: #27ae60;
}
.tag.status-checking,
.tag.status-reconnecting {
  background: #fef5e7;
  color: #f39c12;
}
.tag.status-auth-failed {
  background: #fdecea;
  color: #e74c3c;
}
.tag.status-offline,
.tag.status-unreachable,
.tag.status-unsupported {
  background: var(--bg-secondary, #f5f3ee);
  color: var(--text-secondary, #5a5550);
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
.device-blocked {
  flex-basis: 100%;
  margin-top: 2px;
  font-size: 11px;
  line-height: 1.5;
  color: #e74c3c;
}
.device-recheck {
  align-self: flex-start;
  margin-top: 2px;
  padding: 4px 8px;
  border: 1px solid var(--border-color, #d9d5cc);
  border-radius: 6px;
  background: transparent;
  font-size: 11px;
  color: var(--text-secondary, #5a5550);
  cursor: pointer;
}
.device-recheck:hover {
  border-color: var(--accent-primary, #1e3a5f);
  color: var(--accent-primary, #1e3a5f);
}
.device-confirm-hint {
  margin: 2px 2px 0;
  font-size: 11px;
  line-height: 1.5;
  color: #e74c3c;
}
</style>
