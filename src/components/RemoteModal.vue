<template>
  <Teleport to="body">
    <Transition name="modal">
      <div v-if="visible" class="modal-overlay" @click.self="$emit('close')">
        <div class="remote-modal">
          <header class="modal-header">
            <h2>{{ t('remoteTitle') }}</h2>
            <button class="close-btn" @click="$emit('close')">
              <img src="@/assets/icons/close.svg" :alt="t('close')" />
            </button>
          </header>

          <div class="modal-body">
            <p v-if="loading" class="hint">{{ t('loading') }}</p>

            <template v-else-if="status?.running || status?.hostedElsewhere">
              <!-- 宿主制：远程由本机另一实例提供，扫码连接的是该实例 -->
              <p v-if="status.hostedElsewhere" class="hint">{{ t('remoteHostedElsewhere') }}</p>

              <!-- 模式切换：默认公网中继，勾选切局域直连（宿主态无中继，强制局域） -->
              <label v-if="!status.hostedElsewhere" class="lan-toggle">
                <input type="checkbox" v-model="lanOnly" />
                <span>{{ t('remoteLanOnly') }}</span>
              </label>

              <!-- 局域直连 -->
              <template v-if="lanOnly">
                <p class="hint">{{ t('remoteQrHint') }}</p>
                <div class="qr-wrap">
                  <img v-if="qrDataUrl" :src="qrDataUrl" alt="QR" />
                </div>
                <div class="url-row">
                  <code class="url-text">{{ status.url }}</code>
                  <button class="copy-btn" @click="copyUrl">
                    {{ copied ? t('remoteCopied') : t('remoteCopyUrl') }}
                  </button>
                </div>
              </template>

              <!-- 公网中继（默认） -->
              <template v-else>
                <p class="hint">{{ t('remoteRelayPairHint') }}</p>
                <template v-if="status.relayPairUrl">
                  <div class="qr-wrap">
                    <img v-if="relayQrDataUrl" :src="relayQrDataUrl" alt="QR" />
                  </div>
                  <div class="url-row">
                    <code class="url-text">{{ status.relayPairUrl }}</code>
                    <button class="copy-btn" @click="copyRelayUrl">
                      {{ copiedRelay ? t('remoteCopied') : t('remoteCopyUrl') }}
                    </button>
                  </div>
                </template>
                <!-- 未配置中继地址：提示去启动面板配置 -->
                <p v-else-if="!status.relayEnabled" class="hint">{{ t('remoteRelayNotConfigured') }}</p>
                <div v-else class="relay-status">
                  <span class="dot pulsing"></span>
                  <span>{{ t('remoteRelayConnecting') }}</span>
                </div>
              </template>

              <!-- 停止按钮仅宿主实例显示（非宿主停不了宿主的服务） -->
              <button v-if="!status.hostedElsewhere" class="stop-btn" :disabled="stopping" @click="stopRemote">
                {{ t('remoteStop') }}
              </button>
            </template>

            <!-- 未开启：不自动启动（安全阻断由用户显式操作），点按钮开启 -->
            <div v-else-if="!error" class="enable-block">
              <p class="hint">{{ t('remoteEnableHint') }}</p>
              <button class="enable-btn" :disabled="starting" @click="enableRemote">
                {{ starting ? t('loading') : t('remoteEnable') }}
              </button>
            </div>

            <p v-if="error" class="error-text">{{ error }}</p>
          </div>
        </div>
      </div>
    </Transition>
  </Teleport>
</template>

<script setup lang="ts">
import { ref, watch, onUnmounted } from 'vue'
import { useI18n } from 'vue-i18n'
import QRCode from 'qrcode'
import { copyText } from '@/api/platform'
import { getRemoteStatus, setRemoteEnabled } from '@/api/tauri'
import type { RemoteStatus } from '@/types'

const props = defineProps<{ visible: boolean }>()
const emit = defineEmits<{ close: [] }>()

const { t } = useI18n()

const status = ref<RemoteStatus | null>(null)
const qrDataUrl = ref('')
const relayQrDataUrl = ref('')
const loading = ref(false)
const starting = ref(false)
const stopping = ref(false)
const copied = ref(false)
const copiedRelay = ref(false)
const error = ref('')
/** 局域直连模式（默认公网中继） */
const lanOnly = ref(false)
/** 中继状态轮询（连接/配对码异步就绪） */
let relayPollTimer: number | null = null

watch(() => props.visible, (v) => {
  if (v) open()
  else stopRelayPoll()
}, { immediate: true })

// 组件卸载兜底清轮询（watch 之外的最后一道清理）
onUnmounted(stopRelayPoll)

function stopRelayPoll() {
  if (relayPollTimer !== null) {
    clearInterval(relayPollTimer)
    relayPollTimer = null
  }
}

async function refreshRelayQr() {
  if (!status.value?.relayPairUrl) return
  relayQrDataUrl.value = await QRCode.toDataURL(status.value.relayPairUrl, {
    width: 220,
    margin: 1,
    color: { dark: '#1e1e1e', light: '#ffffff' },
  })
}

function startRelayPoll() {
  stopRelayPoll()
  // 面板打开期间持续轮询（get_remote_status 轻量）：既等配对码就绪，
  // 也在码过期后触发按需重申（pair.created 到达即刷新二维码）
  relayPollTimer = window.setInterval(async () => {
    if (!status.value?.relayEnabled) return
    try {
      const s = await getRemoteStatus()
      const prev = status.value
      status.value = s
      if (s.relayPairUrl && s.relayPairUrl !== prev?.relayPairUrl) {
        await refreshRelayQr()
      }
    } catch {
      // 忽略轮询失败
    }
  }, 2000)
}

/** 依据状态生成二维码并启动中继轮询（open 与 enableRemote 共用） */
async function applyStatus(s: RemoteStatus) {
  status.value = s
  // 中继地址由环境变量 CC_BOX_RELAY_URL 配置（后台调试入口，UI 不暴露）；
  // 未配置中继、或配对码无效（连接中/已过期，一次性消费后需中继重连才有新码）
  // 时默认切到局域视图——面板始终有可扫的 URL/二维码
  lanOnly.value = !s.relayEnabled || !s.relayPairUrl
  if (s.url) {
    qrDataUrl.value = await QRCode.toDataURL(s.url, {
      width: 220,
      margin: 1,
      color: { dark: '#1e1e1e', light: '#ffffff' },
    })
  }
  if (s.relayPairUrl) {
    await refreshRelayQr()
  }
  if (s.relayEnabled) {
    startRelayPoll()
  }
}

/** 打开面板只查状态——不自动开启（远程通断由用户显式控制，安全可控） */
async function open() {
  loading.value = true
  error.value = ''
  qrDataUrl.value = ''
  relayQrDataUrl.value = ''
  try {
    const s = await getRemoteStatus()
    await applyStatus(s)
  } catch (e) {
    error.value = String(e)
  } finally {
    loading.value = false
  }
}

/** 显式开启远程（生成 token 并启动服务器） */
async function enableRemote() {
  if (starting.value) return
  starting.value = true
  error.value = ''
  try {
    const s = await setRemoteEnabled(true)
    await applyStatus(s)
  } catch (e) {
    error.value = String(e)
  } finally {
    starting.value = false
  }
}

async function copyRelayUrl() {
  if (!status.value?.relayPairUrl) return
  try {
    await copyText(status.value.relayPairUrl)
    copiedRelay.value = true
    setTimeout(() => { copiedRelay.value = false }, 2000)
  } catch {
    // 忽略复制失败
  }
}

async function stopRemote() {
  if (stopping.value) return
  stopping.value = true
  try {
    await setRemoteEnabled(false)
    emit('close')
  } catch (e) {
    error.value = String(e)
  } finally {
    stopping.value = false
  }
}

async function copyUrl() {
  if (!status.value?.url) return
  try {
    await copyText(status.value.url)
    copied.value = true
    setTimeout(() => { copied.value = false }, 2000)
  } catch {
    // 忽略复制失败
  }
}
</script>

<style scoped>
.modal-overlay {
  position: fixed;
  inset: 0;
  background: rgba(0, 0, 0, 0.45);
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 1000;
}

.remote-modal {
  width: 360px;
  max-width: calc(100vw - 48px);
  background: var(--bg-primary);
  border: 1px solid var(--border-color);
  border-radius: 12px;
  box-shadow: 0 8px 32px rgba(0, 0, 0, 0.25);
  overflow: hidden;
}

.modal-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 14px 18px;
  border-bottom: 1px solid var(--border-color);
}

.modal-header h2 {
  font-size: 15px;
  font-weight: 600;
  color: var(--text-primary);
}

.close-btn {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 24px;
  height: 24px;
  border: none;
  background: transparent;
  cursor: pointer;
  border-radius: 4px;
}

.close-btn img {
  width: 16px;
  height: 16px;
}

.close-btn:hover {
  background: var(--hover-bg);
}

.modal-body {
  padding: 20px 18px;
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 14px;
}

.hint {
  font-size: 13px;
  color: var(--text-secondary);
  text-align: center;
}

.qr-wrap {
  background: #fff;
  padding: 8px;
  border-radius: 8px;
}

.qr-wrap img {
  display: block;
  width: 220px;
  height: 220px;
}

.url-row {
  display: flex;
  align-items: center;
  gap: 8px;
  width: 100%;
}

.url-text {
  flex: 1;
  min-width: 0;
  font-size: 11px;
  color: var(--text-secondary);
  background: var(--bg-secondary);
  border: 1px solid var(--border-color);
  border-radius: 6px;
  padding: 7px 10px;
  word-break: break-all;
  user-select: all;
}

.copy-btn {
  flex-shrink: 0;
  padding: 7px 12px;
  border: none;
  border-radius: 6px;
  background: var(--accent-color);
  color: #fff;
  font-size: 12px;
  cursor: pointer;
}

.copy-btn:hover {
  opacity: 0.9;
}

.stop-btn {
  border: none;
  border-radius: 6px;
  background: transparent;
  color: var(--error-color);
  font-size: 12px;
  cursor: pointer;
  padding: 6px 10px;
}

.stop-btn:hover {
  background: var(--hover-bg);
}

.stop-btn:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}

/* 未开启状态：显式开启入口（安全阻断——远程默认不通） */
.enable-block {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 14px;
  padding: 8px 0;
}

.enable-btn {
  padding: 10px 28px;
  border: none;
  border-radius: 6px;
  background: var(--accent-color);
  color: #fff;
  font-size: 13px;
  cursor: pointer;
}

.enable-btn:hover {
  opacity: 0.9;
}

.enable-btn:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}

.error-text {
  font-size: 12px;
  color: var(--error-color);
  text-align: center;
  word-break: break-all;
}

/* 模式切换（默认公网中继 ↔ 局域直连） */
.lan-toggle {
  display: flex;
  align-items: center;
  gap: 6px;
  font-size: 13px;
  color: var(--text-secondary);
  cursor: pointer;
  user-select: none;
}

.lan-toggle input {
  width: 14px;
  height: 14px;
  accent-color: var(--accent-color);
  cursor: pointer;
}

/* 中继连接中状态 */
.relay-status {
  display: flex;
  align-items: center;
  gap: 6px;
  font-size: 12px;
  color: var(--text-secondary);
  padding: 8px 0;
}

.relay-status .dot {
  width: 8px;
  height: 8px;
  border-radius: 50%;
}

.relay-status .dot.pulsing {
  background: #f0b460;
  animation: relay-pulse 1.5s ease-in-out infinite;
}

@keyframes relay-pulse {
  50% { opacity: 0.4; }
}

.modal-enter-active,
.modal-leave-active {
  transition: opacity 0.15s ease;
}

.modal-enter-from,
.modal-leave-to {
  opacity: 0;
}
</style>
