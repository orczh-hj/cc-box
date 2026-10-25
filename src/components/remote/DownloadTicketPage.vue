<template>
  <div class="ticket-page">
    <div class="ticket-card">
      <h1 class="ticket-title">CC-Box</h1>
      <p class="ticket-file">{{ scope?.fileName || t('filesTicketTitle') }}</p>
      <p v-if="scope" class="ticket-size">{{ fmtBytes(scope.size) }}</p>

      <div v-if="state === 'downloading'" class="ticket-progress">
        <div class="bar">
          <div class="fill" :style="{ width: percent + '%' }"></div>
        </div>
        <p class="ticket-hint">
          {{ fmtBytes(transferred) }} / {{ fmtBytes(scope?.size ?? 0) }}
        </p>
        <p class="ticket-hint">{{ t('filesTicketKeepForeground') }}</p>
      </div>

      <p v-else-if="state === 'done'" class="ticket-done">{{ t('filesTicketDone') }}</p>

      <!-- 微信 WebView：Blob 下载仍会失败，引导到系统浏览器 -->
      <div v-else-if="state === 'wechat'" class="ticket-error">
        <p>{{ t('filesDownloadHint') }}</p>
        <button class="ticket-btn" @click="copyLink">
          {{ copied ? t('filesCopied') : t('filesCopyLink') }}
        </button>
      </div>

      <div v-else-if="state === 'error'" class="ticket-error">
        <p>{{ error }}</p>
        <button class="ticket-btn" @click="download">{{ t('filesRetry') }}</button>
      </div>

      <p v-else class="ticket-hint">{{ t('filesPreparing') }}</p>
    </div>
  </div>
</template>

<script setup lang="ts">
/**
 * 中继票据下载页：系统浏览器打开 `/?dl=<device>.<ticket>` 后在此自动下载。
 * 独立于主 UI（受限 WS 会话只允许文件分块），断线后按已下载 offset 续传。
 */
import { computed, onMounted, onUnmounted, ref } from 'vue'
import { useI18n } from 'vue-i18n'
import { remoteTransport, type RemoteConnState } from '@/api/remote-transport'
import { downloadFileByTicket } from '@/api/fileTransfer'
import { saveBlobToFile } from '@/utils/platform'
import { copyText } from '@/api/platform'
import { isWeChat } from '@/utils/ua'
import type { DownloadScope } from '../../../remote/protocol/protocol'

const { t } = useI18n()

const wechat = isWeChat()
const scope = ref<DownloadScope | null>(remoteTransport.downloadScope)
const state = ref<'connecting' | 'downloading' | 'done' | 'error' | 'wechat'>(
  wechat ? 'wechat' : 'connecting',
)
const transferred = ref(0)
const error = ref('')
const copied = ref(false)

let cancelled = false
let started = false

const percent = computed(() => {
  const total = scope.value?.size ?? 0
  return total > 0 ? Math.floor((transferred.value / total) * 100) : 0
})

function fmtBytes(n: number): string {
  if (n < 1024) return `${n} B`
  if (n < 1024 * 1024) return `${(n / 1024).toFixed(1)} KB`
  return `${(n / 1024 / 1024).toFixed(1)} MB`
}

async function copyLink() {
  await copyText(location.href)
  copied.value = true
  setTimeout(() => { copied.value = false }, 2000)
}

async function download() {
  // 裸票据（去掉 <device_id>. 前缀）：分块命令只接受 32 hex
  const ticket = remoteTransport.downloadTicketSecret
  const s = scope.value
  if (!ticket || !s) return
  cancelled = false
  state.value = 'downloading'
  error.value = ''
  transferred.value = 0
  try {
    const { blob } = await downloadFileByTicket(ticket, s.size, {
      onProgress: (p) => { transferred.value = p.transferred },
      shouldCancel: () => cancelled,
      // 票据失效/文件变化已由函数内部快速失败；其余视为断线，等重连续传
      shouldRetry: () => remoteTransport.connState !== 'denied',
    })
    if (cancelled) return
    saveBlobToFile(blob, s.fileName)
    state.value = 'done'
  } catch (e) {
    if (cancelled) return
    state.value = 'error'
    error.value = mapTicketError(e)
  }
}

function applyScope(s: DownloadScope) {
  scope.value = s
  if (started) return
  started = true
  void download()
}

function onConnState(st: RemoteConnState) {
  if (st === 'denied') {
    state.value = 'error'
    error.value = t('filesErrTicketInvalid')
  }
}

function mapTicketError(e: unknown): string {
  const msg = e instanceof Error ? e.message : String(e)
  if (msg.includes('ticket') || msg.includes('expired')) return t('filesErrTicketInvalid')
  if (msg.includes('connection') || msg.includes('reconnecting')) return t('filesErrConnecting')
  return msg
}

/** 连接/准备超时：避免一直停在「正在生成下载链接…」 */
const connectTimer = window.setTimeout(() => {
  if (state.value === 'connecting') {
    state.value = 'error'
    error.value = t('filesErrTimeout')
  }
}, 15_000)

onMounted(async () => {
  // 微信内置浏览器：即使打开票据页，Blob 保存仍会失败——引导到系统浏览器，
  // 不建立无意义的受限连接
  if (wechat) return
  remoteTransport.onDownloadScope = applyScope
  remoteTransport.onConnState = onConnState
  if (remoteTransport.downloadScope) applyScope(remoteTransport.downloadScope)
  // 票据模式由本组件发起连接（TokenGate 不渲染）
  await remoteTransport.boot()
})

onUnmounted(() => {
  cancelled = true
  clearTimeout(connectTimer)
})
</script>

<style scoped>
.ticket-page {
  position: fixed;
  inset: 0;
  display: flex;
  align-items: center;
  justify-content: center;
  background: var(--bg-primary, #faf9f6);
  z-index: 2000;
}
.ticket-card {
  display: flex;
  flex-direction: column;
  gap: 12px;
  width: min(360px, 86vw);
  padding: 28px 24px;
  background: var(--bg-secondary, #f5f3ee);
  border: 1px solid var(--border-light, #e8e4dc);
  border-radius: 14px;
  text-align: center;
}
.ticket-title {
  font-size: 20px;
  color: var(--text-primary, #1a1816);
}
.ticket-file {
  font-size: 14px;
  font-weight: 600;
  color: var(--text-primary, #1a1816);
  word-break: break-all;
}
.ticket-size {
  font-size: 12px;
  color: var(--text-secondary, #5a5550);
}
.ticket-progress {
  display: flex;
  flex-direction: column;
  gap: 8px;
}
.bar {
  height: 6px;
  border-radius: 3px;
  background: var(--border-color, #d9d5cc);
  overflow: hidden;
}
.fill {
  height: 100%;
  background: var(--accent-primary, #1e3a5f);
  transition: width 0.2s linear;
}
.ticket-hint {
  font-size: 12px;
  line-height: 1.6;
  color: var(--text-secondary, #5a5550);
}
.ticket-done {
  font-size: 14px;
  color: var(--success-color, #2e7d32);
}
.ticket-error {
  display: flex;
  flex-direction: column;
  gap: 10px;
  font-size: 12px;
  color: #e74c3c;
}
.ticket-btn {
  padding: 10px;
  border: none;
  border-radius: 8px;
  background: var(--accent-primary, #1e3a5f);
  color: #fff;
  font-size: 14px;
  font-weight: 600;
  cursor: pointer;
}
</style>
