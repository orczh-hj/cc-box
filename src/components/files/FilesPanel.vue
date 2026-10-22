<template>
  <div class="files-panel">
    <PanelHeader :title="t('files')" @close="$emit('close')" />

    <div class="panel-content">
      <!-- ==================== 上传（手机 → 桌面） ==================== -->
      <section class="section">
        <h3 class="section-title">{{ t('filesUpload') }}</h3>

        <p v-if="activeTab" class="section-desc">
          {{ t('filesUploadDesc', { dir: projectFilesDir }) }}
        </p>
        <p v-else class="section-desc warn">{{ t('filesUploadNoSession') }}</p>

        <!-- 限额提示（中继常显流量成本） -->
        <p v-if="limitHint" class="limit-hint" :class="{ relay: viaRelay }">{{ limitHint }}</p>

        <!-- idle -->
        <template v-if="upState === 'idle'">
          <input ref="fileInputRef" type="file" class="hidden-input" @change="onFileChosen" />
          <button class="primary-btn" :disabled="!activeTab?.ptyId" @click="fileInputRef?.click()">
            {{ t('filesChooseFile') }}
          </button>
        </template>

        <!-- transferring -->
        <template v-else-if="upState === 'transferring'">
          <div class="transfer-status">
            <div class="progress-row">
              <div class="progress-bar">
                <div class="progress-fill" :style="{ width: upPercent + '%' }"></div>
              </div>
              <span class="progress-text">{{ fmtBytes(upProgress.transferred) }} / {{ fmtBytes(upProgress.total) }}</span>
            </div>
            <button class="secondary-btn" @click="upCancelled = true">{{ t('filesCancel') }}</button>
          </div>
        </template>

        <!-- done -->
        <template v-else-if="upState === 'done' && upResult">
          <p class="done-text">
            {{ upResult.inserted ? t('filesPathInserted') : t('filesPathNotInserted') }}
          </p>
          <div class="path-row">
            <code class="path-code">{{ upResult.relativePath }}</code>
            <button class="secondary-btn" @click="copyUploaded">{{ copied ? t('filesCopied') : t('filesCopy') }}</button>
          </div>
          <button class="secondary-btn" @click="resetUpload">{{ t('filesUploadAnother') }}</button>
        </template>

        <!-- error -->
        <template v-else-if="upState === 'error'">
          <p class="error-text">{{ upError }}</p>
          <div class="error-actions">
            <button v-if="upErrorIsController" class="primary-btn" @click="takeover">{{ t('filesTakeover') }}</button>
            <button class="secondary-btn" @click="resetUpload">{{ t('filesRetry') }}</button>
          </div>
        </template>
      </section>

      <!-- ==================== 下载（桌面 → 手机） ==================== -->
      <section class="section">
        <h3 class="section-title">{{ t('filesDownload') }}</h3>

        <!-- 输入框独占一行（手机窄屏）；操作按钮另起一行 -->
        <div class="dl-input-wrap">
          <input
            v-model="dlPath"
            type="text"
            class="dl-input"
            :placeholder="t('filesDownloadPlaceholder')"
            :disabled="dlState === 'transferring'"
            @keydown.enter="startDownload"
          />
          <button
            v-if="dlPath"
            class="clear-btn"
            :title="t('filesClearPath')"
            @click="clearDownloadPath"
          >
            <!-- SVG 几何居中（× 字符受字体基线影响会偏下） -->
            <svg width="12" height="12" viewBox="0 0 24 24" fill="none"
                 stroke="currentColor" stroke-width="2" stroke-linecap="round">
              <path d="M18 6 6 18M6 6l12 12" />
            </svg>
          </button>
        </div>
        <div class="dl-actions">
          <button
            class="primary-btn compact dl-main-btn"
            :disabled="dlState === 'transferring' || !dlPath.trim()"
            @click="startDownload"
          >{{ t('filesDownloadBtn') }}</button>
          <button
            class="secondary-btn compact"
            :class="{ active: browserVisible }"
            :disabled="dlState === 'transferring'"
            @click="toggleBrowser"
          >{{ t('filesBrowse') }}</button>
          <button
            class="secondary-btn compact"
            :disabled="!dlPath.trim()"
            :title="t('filesCopyPath')"
            @click="copyDownloadPath"
          >{{ dlCopied ? t('filesCopied') : t('filesCopy') }}</button>
        </div>

        <!-- 项目内简易文件浏览器 -->
        <div v-if="browserVisible" class="file-browser">
          <div class="browser-path-row">
            <button class="secondary-btn compact" :disabled="!canGoUp || browserLoading" @click="browserUp">
              {{ t('filesBrowserUp') }}
            </button>
            <code class="path-code ellipsis" :title="browserDisplay">{{ browserDisplay }}</code>
          </div>
          <div class="browser-list">
            <p v-if="browserLoading" class="hint">{{ t('loading') }}</p>
            <p v-else-if="browserError" class="error-text">{{ browserError }}</p>
            <p v-else-if="browserEntries.length === 0" class="hint">{{ t('filesBrowserEmpty') }}</p>
            <template v-else>
              <button
                v-for="e in browserEntries"
                :key="e.path"
                class="browser-item"
                :class="{ selected: e.path === dlPath }"
                @click="browserPick(e)"
              >
                <svg v-if="e.isDir" width="15" height="15" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                  <path d="M22 19a2 2 0 0 1-2 2H4a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h5l2 3h9a2 2 0 0 1 2 2z"/>
                </svg>
                <svg v-else width="15" height="15" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                  <path d="M14 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V8z"/>
                  <path d="M14 2v6h6"/>
                </svg>
                <span class="item-name ellipsis">{{ e.name }}</span>
                <span v-if="!e.isDir && e.size != null" class="item-size">{{ fmtBytes(e.size) }}</span>
              </button>
            </template>
          </div>
        </div>

        <!-- transferring -->
        <template v-if="dlState === 'transferring'">
          <div class="transfer-status">
            <div class="progress-row">
              <div class="progress-bar">
                <div class="progress-fill" :style="{ width: dlPercent + '%' }"></div>
              </div>
              <span class="progress-text">{{ fmtBytes(dlProgress.transferred) }} / {{ fmtBytes(dlProgress.total) }}</span>
            </div>
            <button class="secondary-btn" @click="dlCancelled = true">{{ t('filesCancel') }}</button>
          </div>
        </template>

        <!-- done -->
        <p v-else-if="dlState === 'done' && dlResult" class="done-text">
          {{ t('filesDownloadDone', { name: dlResult }) }}
        </p>

        <!-- error -->
        <template v-else-if="dlState === 'error'">
          <p class="error-text">{{ dlError }}</p>
          <div class="error-actions">
            <button v-if="dlErrorIsController" class="primary-btn" @click="takeover">{{ t('filesTakeover') }}</button>
            <button class="secondary-btn" @click="dlState = 'idle'">{{ t('filesRetry') }}</button>
          </div>
        </template>
      </section>
    </div>
  </div>
</template>

<script setup lang="ts">
/**
 * 文件面板（仅浏览器远程模式渲染，IconBar v-if 守卫）：上传手机文件到当前
 * 会话项目 `.cc-box-files/`（完成后路径插入终端输入行）；下载桌面文件到
 * 手机（路径输入 + 项目内简易浏览器）。传输逻辑在 api/fileTransfer.ts，
 * 本组件只做状态机与展示。
 */
import { ref, computed, onMounted, onUnmounted } from 'vue'
import { useI18n } from 'vue-i18n'
import PanelHeader from '../sidebar/PanelHeader.vue'
import { useSessionStore } from '@/stores/session'
import { sendTerminalCommand } from '@/composables/useTerminalCommand'
import { copyText } from '@/api/platform'
import { transport } from '@/api/transport'
import { remoteTransport } from '@/api/remote-transport'
import { controllerTakeover, fileList, fileTransferLimits, type FileEntryLite, type TransferLimits } from '@/api/tauri'
import {
  uploadFile,
  downloadFile,
  quotePath,
  TransferCancelled,
} from '@/api/fileTransfer'
import { saveBlobToFile } from '@/utils/platform'

const { t } = useI18n()
defineEmits<{ close: [] }>()

const sessionStore = useSessionStore()
const activeTab = computed(() => sessionStore.activeTab)

/** 上传目标目录（项目根 + /.cc-box-files，正斜杠展示） */
const projectFilesDir = computed(() => {
  const root = activeTab.value?.projectPath ?? ''
  return `${root.replace(/[\\/]+$/, '').replace(/\\/g, '/')}/.cc-box-files`
})

// ==================== 限额（挂载时拉取一次；中继提示流量成本） ====================

const limits = ref<TransferLimits | null>(null)
/** 响应式镜像 viaRelay：remoteTransport 的 getter 非 ref，直连页面重开时
 * 迟到的 device.status 帧不会触发 computed 重算——用事件同步到本地 ref */
const viaRelay = ref(remoteTransport.viaRelay)
const effectiveLimit = computed(() => {
  if (!limits.value) return 0
  return viaRelay.value ? limits.value.relayMax : limits.value.directMax
})
const limitHint = computed(() => {
  if (!limits.value) return ''
  if (effectiveLimit.value === 0) return t('filesLimitDisabled')
  const size = fmtBytes(effectiveLimit.value)
  return viaRelay.value
    ? t('filesLimitRelay', { size })
    : t('filesLimitDirect', { size })
})

let unlistenDevice: (() => void) | null = null
let unlistenConnected: (() => void) | null = null

onMounted(async () => {
  try {
    limits.value = await fileTransferLimits()
  } catch {
    // 限额查询失败不阻塞面板（预检跳过，服务端仍权威校验）
  }
  const syncViaRelay = () => { viaRelay.value = remoteTransport.viaRelay }
  try {
    unlistenDevice = await transport.listen('device-status', syncViaRelay)
    unlistenConnected = await transport.listen('remote-connected', syncViaRelay)
  } catch {
    // 桌面模式无远程事件（面板本就不渲染）
  }
})

onUnmounted(() => {
  unlistenDevice?.()
  unlistenConnected?.()
})

// ==================== 上传 ====================

type TransferState = 'idle' | 'transferring' | 'done' | 'error'

const fileInputRef = ref<HTMLInputElement | null>(null)
const upState = ref<TransferState>('idle')
const upProgress = ref({ transferred: 0, total: 0 })
const upResult = ref<{ targetPath: string; relativePath: string; inserted: boolean } | null>(null)
const upError = ref('')
const upCancelled = ref(false)
const copied = ref(false)

const upPercent = computed(() =>
  upProgress.value.total > 0 ? Math.floor((upProgress.value.transferred / upProgress.value.total) * 100) : 0,
)
const upErrorIsController = computed(() => upError.value === t('filesErrController'))

function resetUpload() {
  upState.value = 'idle'
  upResult.value = null
  upError.value = ''
  upCancelled.value = false
  copied.value = false
}

async function onFileChosen(e: Event) {
  const input = e.target as HTMLInputElement
  const file = input.files?.[0]
  input.value = '' // 允许再次选择同一文件
  if (!file) return

  const beginPtyId = sessionStore.activeTab?.ptyId
  if (!beginPtyId) {
    upState.value = 'error'
    upError.value = t('filesErrNoSession')
    return
  }
  // 前端预检（限额未知时跳过——服务端 begin 权威校验；0 = 传输禁用）
  if (limits.value && (effectiveLimit.value === 0 || file.size > effectiveLimit.value)) {
    upState.value = 'error'
    upError.value =
      effectiveLimit.value === 0 ? t('filesLimitDisabled') : t('filesErrTooLarge')
    return
  }

  upCancelled.value = false
  upState.value = 'transferring'
  upProgress.value = { transferred: 0, total: file.size }
  try {
    const { targetPath, relativePath } = await uploadFile(file, beginPtyId, {
      onProgress: (p) => { upProgress.value = p },
      shouldCancel: () => upCancelled.value,
    })
    // 插入时机：仅当活动终端仍是发起上传的那个（防串终端）且在运行。
    // 插入项目内相对路径（CLI 在项目根运行），而非冗长的绝对路径
    const cur = sessionStore.activeTab
    const inserted = !!cur && cur.ptyId === beginPtyId && cur.status === 'running'
    if (inserted) sendTerminalCommand(quotePath(relativePath))
    upResult.value = { targetPath, relativePath, inserted }
    upState.value = 'done'
  } catch (err) {
    upState.value = 'error'
    upError.value = err instanceof TransferCancelled ? t('filesCancelled') : mapError(err)
  }
}

async function copyUploaded() {
  if (!upResult.value) return
  await copyText(upResult.value.relativePath)
  copied.value = true
  setTimeout(() => { copied.value = false }, 2000)
}

// ==================== 下载 ====================

const dlState = ref<TransferState>('idle')
const dlPath = ref('')
const dlProgress = ref({ transferred: 0, total: 0 })
const dlResult = ref('')
const dlError = ref('')
const dlCancelled = ref(false)
const dlCopied = ref(false)

const dlPercent = computed(() =>
  dlProgress.value.total > 0 ? Math.floor((dlProgress.value.transferred / dlProgress.value.total) * 100) : 0,
)
const dlErrorIsController = computed(() => dlError.value === t('filesErrController'))

/** 清空输入框（并复位下载状态/复制反馈） */
function clearDownloadPath() {
  dlPath.value = ''
  dlState.value = 'idle'
  dlError.value = ''
  dlCopied.value = false
}

/** 仅复制当前路径（用户只想取路径、不下载） */
async function copyDownloadPath() {
  const p = dlPath.value.trim()
  if (!p) return
  await copyText(p)
  dlCopied.value = true
  setTimeout(() => { dlCopied.value = false }, 2000)
}

async function startDownload() {
  // 输入框支持项目内相对路径或绝对路径（服务端要求绝对路径，相对路径在此解析）
  const path = resolveDownloadPath(dlPath.value)
  if (!path || dlState.value === 'transferring') return
  dlState.value = 'transferring'
  dlProgress.value = { transferred: 0, total: 0 }
  dlCancelled.value = false
  try {
    const { blob, fileName } = await downloadFile(path, {
      onProgress: (p) => { dlProgress.value = p },
      shouldCancel: () => dlCancelled.value,
    })
    saveBlobToFile(blob, fileName)
    dlResult.value = fileName
    dlState.value = 'done'
  } catch (err) {
    dlState.value = 'error'
    dlError.value = err instanceof TransferCancelled ? t('filesCancelled') : mapError(err)
  }
}

// ==================== 项目内文件浏览器 ====================

const browserVisible = ref(false)
const browserPath = ref('')
const browserEntries = ref<FileEntryLite[]>([])
const browserLoading = ref(false)
const browserError = ref('')

/** 项目根 = 活动会话项目路径（浏览锁定项目内，回上级到根为止；服务端白名单兜底） */
const projectRoot = computed(() => sessionStore.activeTab?.projectPath ?? '')

/** 浏览器当前路径的展示：相对项目根；根显示「项目根目录」 */
const browserDisplay = computed(() => {
  const rel = toRelative(browserPath.value)
  return !browserPath.value || rel === '.' ? t('filesBrowserRoot') : rel
})

function toggleBrowser() {
  browserVisible.value = !browserVisible.value
  if (!browserVisible.value) return
  const root = projectRoot.value
  if (!root) {
    browserError.value = t('filesErrNoSession')
    return
  }
  // 首次打开，或活动项目已切换（旧路径不在新项目内）→ 从新项目根重载，
  // 否则会显示上个项目的目录且「上一级」被禁用无法回到新根
  const cur = normPath(browserPath.value)
  const nr = normPath(root)
  if (!browserPath.value || (cur !== nr && !cur.startsWith(nr + '/'))) {
    void loadBrowser(root)
  }
}

async function loadBrowser(path: string) {
  browserLoading.value = true
  browserError.value = ''
  try {
    browserEntries.value = await fileList(path)
    browserPath.value = path
  } catch (e) {
    browserError.value = mapError(e)
    browserEntries.value = []
  } finally {
    browserLoading.value = false
  }
}

/** 目录 → 进入；文件 → 回填**项目内相对路径**并收起（不自动下载，防误触大文件） */
function browserPick(e: FileEntryLite) {
  if (e.isDir) {
    void loadBrowser(e.path)
  } else {
    dlPath.value = toRelative(e.path)
    browserVisible.value = false
    dlState.value = 'idle'
    // 已知大小超限时提前提示（服务端 download_begin 仍权威校验）
    if (limits.value && e.size != null) {
      if (effectiveLimit.value === 0) {
        dlState.value = 'error'
        dlError.value = t('filesLimitDisabled')
      } else if (e.size > effectiveLimit.value) {
        dlState.value = 'error'
        dlError.value = t('filesErrTooLarge')
      }
    }
  }
}

/** 剥离 Windows canonicalize 的 verbatim 前缀（`\\?\D:\` / `\\?\UNC\srv\`） */
function stripVerbatim(p: string): string {
  return p.replace(/^\\\\\?\\UNC\\/i, '\\\\').replace(/^\\\\\?\\/, '')
}

/** 路径归一（统一分隔符/去尾斜杠/小写——Windows 大小写不敏感比对） */
function normPath(p: string): string {
  return stripVerbatim(p)
    .replace(/\\/g, '/')
    .replace(/\/+$/, '')
    .toLowerCase()
}

/** 绝对路径 → 项目内相对路径（不在项目内则原样返回；项目根本身显示 '.'） */
function toRelative(abs: string): string {
  const root = stripVerbatim(projectRoot.value)
  const a = stripVerbatim(abs)
  if (!root || !a) return a
  const nr = normPath(root)
  const na = normPath(a)
  if (na === nr) return '.'
  const prefix = nr + '/'
  if (!na.startsWith(prefix)) return a
  // normPath 逐字符替换（长度不变），可用归一化长度切原始字符串
  return a.slice(prefix.length).replace(/\\/g, '/')
}

/** 下载路径解析：相对路径按项目根拼成绝对路径（服务端要求绝对）；绝对路径原样 */
function resolveDownloadPath(input: string): string {
  const p = stripVerbatim(input.trim())
  if (!p) return ''
  if (/^(?:[A-Za-z]:[\\/]|\\\\|\/)/.test(p)) return p
  const root = stripVerbatim(projectRoot.value)
  if (!root) return p
  return `${root.replace(/[\\/]+$/, '')}/${p.replace(/^\.\//, '')}`
}

const canGoUp = computed(() => {
  const cur = normPath(browserPath.value)
  const root = normPath(projectRoot.value)
  return !!cur && !!root && cur !== root && cur.startsWith(root + '/')
})

function browserUp() {
  const norm = browserPath.value.replace(/\\/g, '/').replace(/\/+$/, '')
  const idx = norm.lastIndexOf('/')
  if (idx <= 0) return
  void loadBrowser(norm.slice(0, idx))
}

// ==================== 通用 ====================

function takeover() {
  void controllerTakeover()
}

/** 服务端错误文案 → 本地化提示（未识别的原样透出便于诊断） */
function mapError(e: unknown): string {
  const msg = e instanceof Error ? e.message : String(e)
  // 先精确匹配会话类错误（避免 "transfer session not found" 误落"路径不存在"）
  if (msg.includes('transfer session not found')) return t('filesErrSessionGone')
  if (msg.includes('controller held by another device')) return t('filesErrController')
  if (msg.includes('file too large') || msg.includes('exceeds declared size')) {
    return `${t('filesErrTooLarge')}（${msg}）`
  }
  if (msg.includes('too many concurrent')) return t('filesErrTooMany')
  if (msg.includes('authorized')) return t('filesErrNotAuthorized')
  if (msg.includes('changed during download')) return t('filesErrChanged')
  if (msg.includes('not found') || msg.includes('not a file') || msg.includes('cannot read')) {
    return t('filesErrNotFound')
  }
  if (msg.includes('invalid file name')) return t('filesErrInvalidName')
  if (msg.includes('no active session')) return t('filesErrNoSession')
  if (msg.includes('connection not open') || msg.includes('reconnecting')) {
    return t('filesErrConnecting')
  }
  if (msg.includes('timeout')) return t('filesErrTimeout')
  return msg
}

function fmtBytes(n: number): string {
  if (n < 1024) return `${n} B`
  if (n < 1024 * 1024) return `${(n / 1024).toFixed(1)} KB`
  return `${(n / 1024 / 1024).toFixed(1)} MB`
}
</script>

<style scoped>
.files-panel {
  height: 100%;
  display: flex;
  flex-direction: column;
  background: var(--bg-secondary);
}

.panel-content {
  flex: 1;
  overflow-y: auto;
  padding: 10px 12px;
  display: flex;
  flex-direction: column;
  gap: 16px;
}

.section {
  display: flex;
  flex-direction: column;
  gap: 8px;
}

.section-title {
  font-size: 13px;
  font-weight: 600;
  color: var(--text-primary);
  margin: 0;
}

.section-desc {
  font-size: 12px;
  line-height: 1.5;
  color: var(--text-secondary);
  margin: 0;
  word-break: break-all;
}

.section-desc.warn {
  color: var(--text-tertiary);
}

.limit-hint {
  font-size: 11px;
  line-height: 1.5;
  padding: 6px 8px;
  border-radius: 6px;
  background: var(--bg-tertiary, rgba(127, 127, 127, 0.1));
  color: var(--text-secondary);
  margin: 0;
}

.limit-hint.relay {
  color: var(--warning-color, #d19a66);
}

.hidden-input {
  display: none;
}

.primary-btn {
  padding: 8px 14px;
  border: none;
  border-radius: 6px;
  background: var(--accent-color, #4a9eff);
  color: #fff;
  font-size: 13px;
  cursor: pointer;
}

.primary-btn:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}

.primary-btn.compact {
  padding: 6px 10px;
  white-space: nowrap;
}

.secondary-btn {
  padding: 6px 10px;
  border: 1px solid var(--border-color);
  border-radius: 6px;
  background: transparent;
  color: var(--text-primary);
  font-size: 12px;
  cursor: pointer;
  white-space: nowrap;
}

.secondary-btn:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}

.secondary-btn.compact {
  padding: 6px 8px;
}

.secondary-btn.active {
  border-color: var(--accent-color, #4a9eff);
  color: var(--accent-color, #4a9eff);
}

.transfer-status {
  display: flex;
  align-items: center;
  gap: 8px;
}

.progress-row {
  flex: 1;
  display: flex;
  align-items: center;
  gap: 8px;
  min-width: 0;
}

.progress-bar {
  flex: 1;
  height: 6px;
  border-radius: 3px;
  background: var(--bg-tertiary, rgba(127, 127, 127, 0.15));
  overflow: hidden;
}

.progress-fill {
  height: 100%;
  border-radius: 3px;
  background: var(--accent-color, #4a9eff);
  transition: width 0.2s ease;
}

.progress-text {
  font-size: 11px;
  color: var(--text-secondary);
  white-space: nowrap;
}

.done-text {
  font-size: 12px;
  color: var(--text-primary);
  margin: 0;
}

.path-row {
  display: flex;
  align-items: center;
  gap: 6px;
}

.path-code {
  flex: 1;
  font-size: 11px;
  padding: 6px 8px;
  border-radius: 6px;
  background: var(--bg-tertiary, rgba(127, 127, 127, 0.1));
  color: var(--text-primary);
  word-break: break-all;
  text-align: left;
}

.path-code.ellipsis {
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.error-text {
  font-size: 12px;
  color: var(--error-color, #e5534b);
  margin: 0;
  word-break: break-all;
}

.error-actions {
  display: flex;
  gap: 8px;
}

.dl-input-wrap {
  position: relative;
}

.dl-input {
  width: 100%;
  box-sizing: border-box;
  /* 右侧留出清空按钮位置 */
  padding: 8px 32px 8px 8px;
  border: 1px solid var(--border-color);
  border-radius: 6px;
  background: var(--bg-primary);
  color: var(--text-primary);
  font-size: 12px;
}

.clear-btn {
  position: absolute;
  right: 5px;
  top: 50%;
  transform: translateY(-50%);
  width: 22px;
  height: 22px;
  display: flex;
  align-items: center;
  justify-content: center;
  border: none;
  border-radius: 50%;
  background: transparent;
  color: var(--text-tertiary);
  padding: 0;
  cursor: pointer;
}

.clear-btn svg {
  display: block;
}

/* hover 仅指针设备（触屏无 hover，避免点击后样式粘滞） */
@media (hover: hover) {
  .clear-btn:hover {
    background: var(--bg-tertiary, rgba(127, 127, 127, 0.15));
    color: var(--text-primary);
  }
}

/* 窄屏：按钮独立一行，下载为主按钮占满剩余宽度 */
.dl-actions {
  display: flex;
  gap: 6px;
}

.dl-actions .dl-main-btn {
  flex: 1;
}

.file-browser {
  border: 1px solid var(--border-color);
  border-radius: 6px;
  display: flex;
  flex-direction: column;
  overflow: hidden;
}

.browser-path-row {
  display: flex;
  align-items: center;
  gap: 6px;
  padding: 6px 8px;
  border-bottom: 1px solid var(--border-color);
}

.browser-path-row .path-code {
  flex: 1;
  background: transparent;
  padding: 0;
}

.browser-list {
  max-height: 260px;
  overflow-y: auto;
  padding: 4px;
  display: flex;
  flex-direction: column;
  gap: 2px;
}

.browser-item {
  display: flex;
  align-items: center;
  gap: 8px;
  padding: 6px 8px;
  border: none;
  border-radius: 6px;
  background: transparent;
  color: var(--text-primary);
  font-size: 12px;
  cursor: pointer;
  text-align: left;
}

@media (hover: hover) {
  .browser-item:hover {
    background: var(--bg-tertiary, rgba(127, 127, 127, 0.1));
  }
}

.browser-item.selected {
  background: var(--accent-color, #4a9eff);
  color: #fff;
}

.item-name {
  flex: 1;
  min-width: 0;
}

.item-size {
  font-size: 11px;
  color: var(--text-tertiary);
}

.browser-item.selected .item-size {
  color: rgba(255, 255, 255, 0.8);
}

.hint {
  font-size: 12px;
  color: var(--text-secondary);
  text-align: center;
  padding: 12px 0;
  margin: 0;
}

.ellipsis {
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}
</style>
