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
      <div v-if="desktops.length > 0" class="gate-desktops">
        <button
          v-for="d in desktops"
          :key="d.id"
          class="desktop-item"
          :class="{ active: d.id === activeId }"
          @click="activate(d.id)"
        >
          <span class="dot" :class="dotClass(d)"></span>
          <span class="desktop-name">{{ d.name || fallbackName(d) }}</span>
          <span v-if="d.invalid" class="tag invalid">令牌无效</span>
          <span class="tag kind">{{ d.kind === 'lan' ? '局域网' : '中继' }}</span>
          <span
            class="desktop-remove"
            title="删除此设备"
            @click.stop="remove(d.id)"
          >×</span>
        </button>
      </div>

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
import { computed, onMounted, ref } from 'vue'
import { remoteTransport } from '@/api/remote-transport'
import { listDesktops, removeDesktop, type StoredDesktop } from '@/api/remote-desktops'

const emit = defineEmits<{ connected: [] }>()

const tokenInput = ref('')
const busy = ref(true)
const wasDenied = ref(false)
const connState = ref<'connecting' | 'authenticating' | 'connected' | 'reconnecting' | 'denied' | 'closed'>('closed')

// ==================== 已保存设备列表 ====================

const desktops = ref<StoredDesktop[]>([])
const activeId = ref<string | null>(remoteTransport.activeDesktopId)

function refreshDesktops(): void {
  desktops.value = listDesktops()
  // activeDesktopId 是普通 getter（非响应式）——列表刷新时机同步取值，
  // 保证激活高亮与 busy 设备名跟随
  activeId.value = remoteTransport.activeDesktopId
}

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

/** 圆点：当前条目按连接状态着色，其余条目灰色 */
function dotClass(d: StoredDesktop): string {
  if (d.id !== activeId.value) return 'unknown'
  switch (connState.value) {
    case 'connected': return 'connected'
    case 'connecting':
    case 'authenticating':
    case 'reconnecting': return 'connecting'
    case 'denied': return 'denied'
    default: return 'unknown'
  }
}

function fallbackName(d: StoredDesktop): string {
  const short = d.id.slice(-6)
  return `桌面 · ${short}`
}

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
  await remoteTransport.boot()
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
.gate-desktops {
  display: flex;
  flex-direction: column;
  gap: 6px;
  max-height: 240px;
  overflow-y: auto;
}
.desktop-item {
  display: flex;
  align-items: center;
  gap: 8px;
  padding: 10px 10px;
  background: var(--bg-primary, #fff);
  border: 1px solid var(--border-color, #d9d5cc);
  border-radius: 8px;
  font-size: 14px;
  color: var(--text-primary, #1a1816);
  text-align: left;
  cursor: pointer;
}
.desktop-item.active {
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
.desktop-name {
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
.desktop-remove {
  flex: none;
  width: 22px;
  height: 22px;
  line-height: 20px;
  border-radius: 4px;
  font-size: 15px;
  color: var(--text-secondary, #5a5550);
  text-align: center;
}
.desktop-remove:hover {
  background: #fdecea;
  color: #e74c3c;
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
