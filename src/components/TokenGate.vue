<template>
  <div class="token-gate">
    <div class="gate-card">
      <h1 class="gate-title">CC-Box</h1>
      <p class="gate-hint">{{ hint }}</p>

      <!-- 连接中 / 认证中（列表保持可点：等待中可改连其他设备） -->
      <div v-if="busy" class="gate-state">
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

      <!-- 需要输入 token（closed 无 token / denied 且无可用列表） -->
      <template v-if="!busy">
        <input
          v-model="tokenInput"
          class="gate-input"
          type="text"
          placeholder="token"
          autocapitalize="off"
          autocomplete="off"
          spellcheck="false"
          @keydown.enter="connect"
        >
        <button class="gate-btn" :disabled="!tokenInput.trim()" @click="connect">连接</button>
        <p v-if="wasDenied" class="gate-error">令牌无效，请检查后重试</p>
      </template>
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed, onMounted, ref, watch } from 'vue'
import { remoteTransport } from '@/api/remote-transport'
import { listDesktops, removeDesktop, type StoredDesktop } from '@/api/remote-desktops'
import DeviceList from '@/components/remote/DeviceList.vue'
import { useDeviceProbe } from '@/composables/useDeviceProbe'

/** autoBoot=false：作为主 UI 之上的连接失效覆盖层复用，不主动重连
 *  initialDenied：覆盖层进入时即展示「令牌失效」提示 */
const props = withDefaults(defineProps<{
  autoBoot?: boolean
  initialDenied?: boolean
}>(), {
  autoBoot: true,
  initialDenied: false,
})

const emit = defineEmits<{ connected: [] }>()

const tokenInput = ref('')
const busy = ref(props.autoBoot)
const wasDenied = ref(props.initialDenied)
const connState = ref<'connecting' | 'authenticating' | 'connected' | 'reconnecting' | 'denied' | 'closed'>('closed')

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

const stateText = computed(() => {
  switch (connState.value) {
    case 'connecting': return '连接中…'
    case 'authenticating': return '认证中…'
    case 'reconnecting': return '重连中…'
  }
  return ''
})

/** busy 文案带上设备名（区分多设备：知道正在连哪台） */
const busyText = computed(() => {
  const name = desktops.value.find(d => d.id === activeId.value)?.name
  return name ? `${name} · ${stateText.value}` : stateText.value
})

function activate(id: string): void {
  busy.value = true
  wasDenied.value = false
  remoteTransport.activateDesktop(id)
}

function remove(id: string): void {
  removeDesktop(id)
  refreshDesktops()
}

const hint = computed(() =>
  wasDenied.value
    ? '访问令牌无效或已过期（中继升级或桌面刷新密钥后需重新配对），请重新扫码'
    : '在桌面端 CC-Box 点击左侧「远程」按钮，扫码或粘贴访问令牌',
)

onMounted(async () => {
  remoteTransport.onDesktopsChanged = refreshDesktops
  refreshDesktops()
  remoteTransport.onConnState = (s) => {
    connState.value = s
    if (s === 'connected') {
      emit('connected')
      return
    }
    if (s === 'denied') {
      wasDenied.value = true
      busy.value = false
      return
    }
    // closed（无 token 可用）需要输入
    if (s === 'closed') {
      busy.value = false
    }
  }
  if (props.autoBoot) {
    await remoteTransport.boot()
  } else {
    busy.value = false
  }
})

function connect() {
  const t = tokenInput.value.trim()
  if (!t) return
  busy.value = true
  wasDenied.value = false
  remoteTransport.connectWithToken(t)
}
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
.gate-input {
  background: var(--bg-primary, #fff);
  border: 1px solid var(--border-color, #d9d5cc);
  border-radius: 8px;
  color: var(--text-primary, #1a1816);
  padding: 12px;
  font-size: 14px;
}
.gate-input:focus {
  outline: none;
  border-color: var(--accent-primary, #1e3a5f);
}
.gate-btn {
  background: var(--accent-primary, #1e3a5f);
  border: none;
  border-radius: 8px;
  color: #fff;
  padding: 12px;
  font-size: 15px;
  font-weight: 600;
}
.gate-btn:disabled {
  opacity: 0.4;
}
.gate-error {
  font-size: 12px;
  color: #e74c3c;
}
</style>
