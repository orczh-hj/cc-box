<template>
  <div class="token-gate">
    <div class="gate-card">
      <h1 class="gate-title">CC-Box</h1>
      <h2 class="gate-subtitle">{{ headline.title }}</h2>
      <p class="gate-hint">{{ headline.hint }}</p>

      <!-- 连接中 / 认证中 / 重连未超时（列表保持可点：等待中可改连其他设备） -->
      <div v-if="inFlight && !stalled" class="gate-state">
        <span class="spinner"></span>
        <span>{{ busyText }}</span>
      </div>

      <!-- 已保存设备列表（多设备凭证：点击连接，× 删除） -->
      <DeviceList
        v-if="desktops.length > 0"
        :desktops="desktops"
        :active-id="activeId"
        :conn-state="connState"
        :statuses="statuses"
        @select="activate"
        @remove="remove"
        @recheck="recheck(desktops, activeId)"
      />

      <!-- 操作指导：未在连接，或重连停滞（自动重连仍在后台继续） -->
      <ol v-if="!inFlight || stalled" class="gate-steps">
        <li>{{ t('gateStepOpen') }}</li>
        <li>{{ t(needsFreshCode ? 'gateStepScanFresh' : 'gateStepScan') }}</li>
        <li v-if="showRotateStep">{{ t('gateStepRotate') }}</li>
      </ol>
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed, onMounted, ref, watch } from 'vue'
import { useI18n } from 'vue-i18n'
import { remoteTransport, type RemoteConnState, type RemoteFailReason } from '@/api/remote-transport'
import { listDesktops, removeDesktop, type StoredDesktop } from '@/api/remote-desktops'
import DeviceList from '@/components/remote/DeviceList.vue'
import { useDeviceProbe } from '@/composables/useDeviceProbe'
import { useConnHealth } from '@/composables/useConnHealth'

/** autoBoot=false：作为主 UI 之上的连接失效覆盖层复用，不主动重连 */
const props = withDefaults(defineProps<{
  autoBoot?: boolean
}>(), {
  autoBoot: true,
})

const emit = defineEmits<{ connected: [] }>()

const { t } = useI18n()
const { level } = useConnHealth()

const booting = ref(props.autoBoot)
const connState = ref<RemoteConnState>(remoteTransport.connState)
const reason = ref<RemoteFailReason | null>(remoteTransport.failReason)

// ==================== 已保存设备列表 ====================

const desktops = ref<StoredDesktop[]>([])
const activeId = ref<string | null>(remoteTransport.activeDesktopId)
const { statuses, probeAll, recheck } = useDeviceProbe()

function refreshDesktops(): void {
  desktops.value = listDesktops()
  // activeDesktopId 是普通 getter（非响应式）——列表刷新时机同步取值，
  // 保证激活高亮与 busy 设备名跟随
  activeId.value = remoteTransport.activeDesktopId
}

// 列表或当前设备变化即重探（当前设备由主连接 connState 展示，不重复建连）
watch(
  [() => desktops.value.map(d => `${d.id}|${d.token}`).join(','), activeId],
  () => probeAll(desktops.value, activeId.value),
  { immediate: true },
)

// ==================== 状态与指导 ====================

const stateText = computed(() => {
  switch (connState.value) {
    case 'connecting': return t('gateStateConnecting')
    case 'authenticating': return t('gateStateAuthenticating')
    case 'reconnecting': return t('gateStateReconnecting')
  }
  return ''
})

/** busy 文案带上设备名（区分多设备：知道正在连哪台） */
const busyText = computed(() => {
  const name = desktops.value.find(d => d.id === activeId.value)?.name
  return name ? `${name} · ${stateText.value}` : stateText.value
})

const inFlight = computed(() => booting.value || stateText.value !== '')
/** 断开累计超时（阈值见 useConnHealth）：退出 spinner 展示指导 */
const stalled = computed(() => inFlight.value && (level.value === 'overlay' || level.value === 'stalled'))

/** 需要新的连接码：连接码已被消费（无效/过期/设备离线时 relay 同样消费），或电脑端
 *  身份已作废——两种情况都必须在电脑端重新生成连接码再扫。
 *  pair-network 除外：请求没到 relay，码还在有效期内，原码重扫即可 */
const FRESH_CODE_REASONS: ReadonlySet<RemoteFailReason> = new Set([
  'identity-rotated',
  'pair-locked',
  'pair-device-offline',
  'pair-rejected',
])
const needsFreshCode = computed(() => reason.value !== null && FRESH_CODE_REASONS.has(reason.value))

/** 只有令牌失效才提「刷新密钥」——它会断开所有已配对设备 */
const showRotateStep = computed(() => !stalled.value && !needsFreshCode.value)

const headline = computed<{ title: string; hint: string }>(() => {
  if (stalled.value) return { title: t('gateStalledTitle'), hint: t('gateStalledHint') }
  const key = {
    'auth-invalid': 'AuthInvalid',
    'identity-rotated': 'IdentityRotated',
    'pair-rejected': 'PairRejected',
    'pair-device-offline': 'DeviceOffline',
    'pair-locked': 'PairLocked',
    'pair-network': 'PairNetwork',
  }[reason.value ?? ''] ?? 'Pair'
  return { title: t(`gate${key}Title`), hint: t(`gate${key}Hint`) }
})

function activate(id: string): void {
  remoteTransport.activateDesktop(id)
}

function remove(id: string): void {
  removeDesktop(id)
  refreshDesktops()
}

onMounted(async () => {
  remoteTransport.onDesktopsChanged = refreshDesktops
  refreshDesktops()
  remoteTransport.onConnState = (s) => {
    connState.value = s
    // 建连意图先改原因再发状态：此处读到的总是本次状态对应的原因
    reason.value = remoteTransport.failReason
    if (s === 'connected') {
      emit('connected')
      return
    }
  }
  if (props.autoBoot) {
    await remoteTransport.boot()
  }
  booting.value = false
})
</script>

<style scoped>
.token-gate {
  position: fixed;
  inset: 0;
  display: flex;
  align-items: center;
  justify-content: center;
  background: var(--bg-primary, #faf9f6);
  z-index: 2000;
}
.gate-card {
  display: flex;
  flex-direction: column;
  gap: 14px;
  width: min(360px, 86vw);
  padding: 32px 28px;
  background: var(--bg-secondary, #f5f3ee);
  border: 1px solid var(--border-light, #e8e4dc);
  border-radius: 14px;
  text-align: center;
}
.gate-title {
  font-size: 22px;
  color: var(--text-primary, #1a1816);
}
.gate-subtitle {
  font-size: 15px;
  font-weight: 600;
  color: var(--text-primary, #1a1816);
}
.gate-hint {
  font-size: 13px;
  line-height: 1.6;
  color: var(--text-secondary, #5a5550);
}
.gate-state {
  display: flex;
  align-items: center;
  justify-content: center;
  gap: 10px;
  padding: 12px 0;
  font-size: 14px;
  color: var(--text-secondary, #5a5550);
}
.gate-steps {
  display: flex;
  flex-direction: column;
  gap: 6px;
  margin-top: 4px;
  padding-left: 20px;
  font-size: 13px;
  line-height: 1.6;
  color: var(--text-secondary, #5a5550);
  text-align: left;
}
.spinner {
  width: 16px;
  height: 16px;
  border-radius: 50%;
  border: 2px solid var(--border-color, #d9d5cc);
  border-top-color: var(--accent-primary, #1e3a5f);
  animation: spin 0.9s linear infinite;
}
@keyframes spin {
  to { transform: rotate(360deg); }
}
</style>
