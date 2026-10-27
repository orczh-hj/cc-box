<template>
  <div class="section-content">
    <h2 class="section-heading">{{ t('ccBoxUpdate') }}</h2>

    <div class="update-card">
      <div class="version-row">
        <div class="version-info">
          <span class="version-label">CC-Box</span>
          <span class="version-value">v{{ currentVersion }}</span>
        </div>
        <button
          class="check-btn"
          :disabled="checking"
          @click="handleCheckUpdate"
        >
          <img v-if="checking" src="@/assets/icons/refresh.svg" class="spinning" alt="" />
          <span>{{ checking ? t('checking') : t('checkForUpdates') }}</span>
        </button>
      </div>

      <div v-if="error" class="update-message error">
        <span>{{ t('checkFailed', { error: errorMessage }) }}</span>
      </div>

      <div v-if="updateStore.updateInfo && !updateStore.updateInfo.hasUpdate" class="update-message success">
        <span>{{ t('upToDate') }}</span>
      </div>

      <template v-if="updateStore.updateInfo && updateStore.updateInfo.hasUpdate">
        <div class="update-available">
          <div class="update-banner">
            <span class="update-icon">🆕</span>
            <div>
              <span class="update-version">{{ t('versionAvailable', { version: updateStore.updateInfo.version }) }}</span>
              <span class="update-hint">{{ t('yourVersion') }} v{{ updateStore.updateInfo.currentVersion }}</span>
            </div>
          </div>

          <div v-if="updateStore.updateInfo.releaseNotes" class="release-notes">
            <h4>{{ t('whatsNew') }}</h4>
            <div class="notes-content" v-html="renderedNotes"></div>
          </div>

          <!-- 下载/安装状态 -->
          <div class="update-actions">
            <!-- 初始状态：显示下载并安装按钮 -->
            <template v-if="updateStore.downloadState === 'idle'">
              <div class="action-row">
                <button class="action-btn primary" @click="handleDownloadAndInstall">
                  {{ t('downloadAndInstall') }}
                </button>
                <button class="action-btn secondary" @click="openReleases">
                  {{ t('manualDownload') }}
                </button>
              </div>
            </template>

            <!-- 下载中 -->
            <template v-if="updateStore.downloadState === 'downloading'">
              <div class="progress-section">
                <div class="progress-header">
                  <span>{{ t('downloadingUpdate') }}</span>
                </div>
                <div class="progress-bar">
                  <div class="progress-fill" :style="{ width: updateStore.downloadProgress.percent + '%' }"></div>
                </div>
                <div class="progress-info">
                  <span>{{ updateStore.downloadProgress.percent.toFixed(0) }}%</span>
                  <span class="progress-size">
                    {{ formatSize(updateStore.downloadProgress.downloaded) }} / {{ formatSize(updateStore.downloadProgress.total) }}
                  </span>
                </div>
              </div>
            </template>

            <!-- 安装中 -->
            <template v-if="updateStore.downloadState === 'installing'">
              <div class="installing-message">
                <span class="spinning-text">{{ t('installingUpdate') }}</span>
                <span class="installing-hint">{{ t('willRestartAuto') }}</span>
              </div>
            </template>

            <!-- 错误 -->
            <div v-if="updateStore.downloadState === 'error'" class="update-message error">
              <span>{{ updateStore.downloadError }}</span>
              <div class="error-actions">
                <button class="retry-link" @click="handleRetry">{{ t('retry') }}</button>
              </div>
            </div>
          </div>
        </div>
      </template>
    </div>

    <!-- Claude CLI 历史版本卡片 -->
    <div class="update-card claude-cli-card">
      <div class="version-row">
        <div class="version-info">
          <span class="version-label">Claude CLI</span>
          <span class="version-value" v-if="updateStore.installedClaudeVersion">
            v{{ updateStore.installedClaudeVersion }}
          </span>
          <span class="version-value not-installed" v-else>
            {{ t('notInstalledShort') }}
          </span>
        </div>
        <button
          class="check-btn"
          :disabled="updateStore.claudeVersionListLoading"
          @click="handleRefreshVersions"
        >
          <img v-if="updateStore.claudeVersionListLoading" src="@/assets/icons/refresh.svg" class="spinning" alt="" />
          <span>{{ updateStore.claudeVersionListLoading ? t('checking') : t('refreshVersions') }}</span>
        </button>
      </div>

      <div v-if="updateStore.claudeVersionListError" class="update-message error">
        <span>{{ t('loadVersionsFailed') }}: {{ updateStore.claudeVersionListError }}</span>
      </div>

      <div class="version-list-wrapper">
        <div class="version-list-header">
          <span class="version-list-title">{{ t('versionHistory') }}</span>
          <span v-if="updateStore.claudeVersionList.length > 0" class="version-list-count">
            {{ t('versionCount', { count: updateStore.claudeVersionList.length, latest: updateStore.claudeVersionLatest }) }}
          </span>
        </div>
        <div class="version-list-hint">{{ t('versionHistoryHint') }}</div>

        <div v-if="updateStore.claudeVersionList.length === 0 && !updateStore.claudeVersionListLoading" class="version-empty">
          {{ t('noVersionsAvailable') }}
        </div>

        <div v-else class="version-list">
          <div v-for="entry in updateStore.claudeVersionList" :key="entry.version" class="version-item">
            <div class="version-item-info">
              <span class="version-item-number">v{{ entry.version }}</span>
              <span class="version-item-date">{{ entry.releaseDate }}</span>
              <span v-if="entry.version === updateStore.installedClaudeVersion" class="version-tag installed-tag">
                {{ t('installed') }}
              </span>
              <span v-else-if="entry.version === updateStore.claudeVersionLatest" class="version-tag latest-tag">
                {{ t('latest') }}
              </span>
            </div>

            <div class="version-item-actions">
              <span
                v-if="!isVersionAvailableForCurrentPlatform(entry)"
                class="version-item-disabled"
                :title="t('versionNotAvailableForPlatform')"
              >
                {{ t('versionNotAvailableForPlatform') }}
              </span>

              <div v-else-if="getDownload(entry.version)?.state === 'downloading'" class="version-progress">
                <div class="version-progress-bar">
                  <div class="version-progress-fill" :style="{ width: (getDownload(entry.version)?.progress || 0) + '%' }"></div>
                </div>
                <span class="version-progress-text">{{ (getDownload(entry.version)?.progress || 0).toFixed(0) }}%</span>
                <button
                  class="action-btn secondary small cancel-btn"
                  @click="handleCancelDownload(entry.version)"
                >
                  {{ t('cancelInstall') }}
                </button>
              </div>

              <span
                v-else-if="getDownload(entry.version)?.state === 'installing'"
                class="version-item-installing"
              >
                {{ t('installingVersion', { version: entry.version }) }}
              </span>

              <template v-else-if="getDownload(entry.version)?.state === 'error'">
                <span class="version-item-error" :title="getDownload(entry.version)?.error">
                  {{ t('downloadFailed', { error: '' }) }}
                </span>
                <button class="action-btn primary small" @click="handleDownloadVersion(entry.version)">
                  {{ t('retry') }}
                </button>
              </template>

              <template v-else-if="getDownload(entry.version)?.state === 'cancelled'">
                <span class="version-item-cancelled">{{ t('installCancelled') }}</span>
                <button class="action-btn primary small" @click="handleDownloadVersion(entry.version)">
                  {{ t('download') }}
                </button>
              </template>

              <!-- 使用中版本（idle 或 done）：只显示「重装」 -->
              <button
                v-else-if="entry.version === updateStore.installedClaudeVersion"
                class="action-btn secondary small"
                @click="handleDownloadVersion(entry.version)"
              >
                {{ t('reinstall') }}
              </button>

              <!-- 非使用中 + done：显示「在文件夹中显示」 -->
              <button
                v-else-if="getDownload(entry.version)?.state === 'done'"
                class="action-btn secondary small"
                @click="openDownloadedFolder(entry.version)"
              >
                {{ t('openFolder') }}
              </button>

              <!-- 非使用中 + idle：显示「安装」 -->
              <button
                v-else
                class="action-btn primary small"
                @click="handleDownloadVersion(entry.version)"
              >
                {{ t('download') }}
              </button>
            </div>
          </div>
        </div>
      </div>
    </div>

    <!-- CC-Box 更新确认对话框 -->
    <div v-if="showConfirm" class="confirm-overlay" @click.self="showConfirm = false">
      <div class="confirm-dialog">
        <p class="confirm-text">{{ t('updateConfirmActivePtys') }}</p>
        <div class="confirm-actions">
          <button class="btn-cancel" @click="showConfirm = false">{{ t('cancel') }}</button>
          <button class="btn-primary" @click="confirmUpdate">{{ t('downloadAndInstall') }}</button>
        </div>
      </div>
    </div>

    <!-- Claude CLI 历史版本安装：检测到 Claude 运行的确认弹窗 -->
    <div v-if="claudeRunningConfirm" class="confirm-overlay" @click.self="dismissClaudeRunning">
      <div class="confirm-dialog">
        <p class="confirm-text">{{ t('claudeRunningTitle') }}</p>
        <p class="confirm-hint">{{ t('claudeRunningHint') }}</p>
        <div class="confirm-actions">
          <button class="btn-cancel" @click="dismissClaudeRunning">{{ t('cancel') }}</button>
          <button class="btn-primary" @click="confirmKillAndInstall">{{ t('killAndInstall') }}</button>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted, onUnmounted } from 'vue'
import { useI18n } from 'vue-i18n'
import { check, relaunch, prepareForUpdate, resumeBrokerConnection } from '@/api/tauri'
import { checkForUpdates, downloadClaudeVersion, cancelClaudeVersionDownload, installClaudeVersion, killClaudeProcesses, getInstalledClaudeVersion, onInstallProgress } from '@/api/tauri'
import { openExternal } from '@/api/platform'
import { useSidebarStore } from '@/stores/sidebar'
import { useSessionStore } from '@/stores/session'
import { useUpdateStore } from '@/stores/update'
import { getClaudePlatformKey } from '@/utils/platform'
import type { ClaudeVersionEntry } from '@/types'

const { t } = useI18n()
const sidebarStore = useSidebarStore()
const sessionStore = useSessionStore()
const updateStore = useUpdateStore()
const currentVersion = __APP_VERSION__
const checking = ref(false)
const error = ref(false)
const errorMessage = ref('')
const showConfirm = ref(false)

// Claude 运行时的安装确认弹窗：记录待安装的版本与下载路径
const claudeRunningConfirm = ref<{ version: string; savedPath: string } | null>(null)

const currentPlatformKey = getClaudePlatformKey()

// 监听 Claude CLI 历史版本下载进度
let unlistenHistoryProgress: (() => void) | null = null

const renderedNotes = computed(() => {
  if (!updateStore.updateInfo?.releaseNotes) return ''
  return updateStore.updateInfo.releaseNotes
    .replace(/\n/g, '<br>')
    .replace(/#{1,3}\s(.+)/g, '<strong>$1</strong>')
})

function formatSize(bytes: number): string {
  if (bytes === 0) return '0 B'
  const units = ['B', 'KB', 'MB', 'GB']
  const i = Math.floor(Math.log(bytes) / Math.log(1024))
  return `${(bytes / Math.pow(1024, i)).toFixed(1)} ${units[i]}`
}

async function handleCheckUpdate() {
  checking.value = true
  error.value = false
  errorMessage.value = ''
  try {
    const info = await checkForUpdates()
    if (info) {
      updateStore.setUpdateInfo(info)
      sidebarStore.setUpdateInfo(info)
    }
  } catch (err) {
    error.value = true
    errorMessage.value = String(err)
  } finally {
    checking.value = false
  }
}

function hasActivePtys(): boolean {
  return sessionStore.runningTabIds.length > 0
}

function openReleases() {
  openExternal('https://github.com/orczh-hj/cc-box/releases')
}

async function handleDownloadAndInstall() {
  if (hasActivePtys()) {
    showConfirm.value = true
    return
  }
  startDownload()
}

async function confirmUpdate() {
  showConfirm.value = false
  await startDownload()
}

async function startDownload() {
  updateStore.setDownloadState('downloading')
  updateStore.clearError()
  updateStore.setDownloadProgress({ downloaded: 0, total: 0, percent: 0 })

  try {
    const update = await check()
    if (!update) {
      updateStore.setDownloadError(t('noUpdateAvailable'))
      updateStore.setDownloadState('error')
      return
    }

    let downloaded = 0
    let contentLength = 0

    // 先下载：下载期间 Broker 继续提供远程。只有真正安装前才让它退出，
    // 避免慢下载/下载失败导致远程长时间离线。
    await update.download((event) => {
      switch (event.event) {
        case 'Started':
          contentLength = event.data.contentLength ?? 0
          updateStore.setDownloadProgress({ downloaded: 0, total: contentLength, percent: 0 })
          break
        case 'Progress':
          downloaded += event.data.chunkLength
          const percent = contentLength > 0 ? (downloaded / contentLength) * 100 : 0
          updateStore.setDownloadProgress({ downloaded, total: contentLength, percent })
          break
        case 'Finished':
          break
      }
    })

    updateStore.setDownloadState('installing')
    // Broker 与本进程共用可执行文件：安装前让它退出并写 update_in_progress，
    // 避免 Windows 上无法替换正在运行的 exe，也避免 GUI 立即把它重新拉起。
    try {
      await prepareForUpdate()
    } catch {
      // Broker 可能已退出（响应未及返回）——继续安装
    }
    try {
      await update.install()
      await relaunch()
    } catch (err) {
      // 安装/重启失败：清除 update_in_progress 与 SUPPRESSED，恢复 Broker，
      // 否则本次进程会永久处于远程/托盘不可用状态。
      await resumeBrokerConnection().catch(() => {})
      throw err
    }
  } catch (err) {
    updateStore.setDownloadError(t('updateFailed', { error: String(err) }))
    updateStore.setDownloadState('error')
  }
}

async function handleRetry() {
  updateStore.resetDownload()
  await startDownload()
}

// ============ Claude CLI 历史版本 ============

async function handleRefreshVersions() {
  await updateStore.loadClaudeVersionList(true)
}

function isVersionAvailableForCurrentPlatform(entry: ClaudeVersionEntry): boolean {
  return !!entry.platforms?.[currentPlatformKey]
}

function getDownload(version: string) {
  return updateStore.getHistoryDownload(version)
}

async function handleDownloadVersion(version: string) {
  updateStore.setHistoryDownload(version, {
    state: 'downloading',
    progress: 0,
    message: t('downloadingVersion', { version }),
    error: '',
    savedPath: '',
  })

  try {
    const savedPath = await downloadClaudeVersion(version)
    // 下载完成（或本地缓存复用）后立即尝试安装
    await performInstall(version, savedPath)
  } catch (err) {
    const errStr = String(err)
    // 后端取消返回 'cancelled' 字符串，识别后展示为已取消状态
    if (errStr.includes('cancelled')) {
      updateStore.setHistoryDownload(version, {
        state: 'cancelled',
        progress: 0,
        message: t('installCancelled'),
      })
    } else {
      updateStore.setHistoryDownload(version, {
        state: 'error',
        error: errStr,
      })
    }
  }
}

// 执行覆盖安装：若 claude 在运行 → 弹窗；否则直接复制覆盖
async function performInstall(version: string, savedPath: string) {
  updateStore.setHistoryDownload(version, {
    state: 'installing',
    progress: 0,
    message: t('installingVersion', { version }),
    savedPath,
  })

  try {
    await installClaudeVersion(savedPath, version)
    // 刷新本地版本号
    try {
      const v = await getInstalledClaudeVersion()
      updateStore.setInstalledClaudeVersion(v)
    } catch { /* 读取失败不影响整体流程 */ }
    updateStore.setHistoryDownload(version, {
      state: 'done',
      progress: 100,
      message: t('installComplete'),
    })
  } catch (err) {
    const errStr = String(err)
    if (errStr.includes('claude-running')) {
      // 弹窗让用户决定是否 kill 后重试
      claudeRunningConfirm.value = { version, savedPath }
      // 状态保留在 installing，等用户决定
      return
    }
    updateStore.setHistoryDownload(version, {
      state: 'error',
      error: t('installFailed', { error: errStr }),
    })
  }
}

function dismissClaudeRunning() {
  const info = claudeRunningConfirm.value
  claudeRunningConfirm.value = null
  if (info) {
    // 用户取消安装，回退到 done 状态（文件仍保留在下载目录）
    updateStore.setHistoryDownload(info.version, {
      state: 'done',
      progress: 100,
      message: t('downloadComplete'),
      savedPath: info.savedPath,
    })
  }
}

async function confirmKillAndInstall() {
  const info = claudeRunningConfirm.value
  if (!info) return
  claudeRunningConfirm.value = null

  try {
    await killClaudeProcesses()
  } catch (err) {
    updateStore.setHistoryDownload(info.version, {
      state: 'error',
      error: t('killClaudeFailed', { error: String(err) }),
    })
    return
  }

  // kill 后再尝试安装
  await performInstall(info.version, info.savedPath)
}

async function handleCancelDownload(version: string) {
  try {
    await cancelClaudeVersionDownload(version)
    // 标记为 cancelled；后端也会异步 emit 'cancelled' 进度，但前端 invoke 会被 reject
    // 这里乐观更新，避免等后端 reject 期间按钮一直显示"取消中"
    updateStore.setHistoryDownload(version, {
      state: 'cancelled',
      progress: 0,
      message: t('installCancelled'),
    })
  } catch (err) {
    console.error('Failed to cancel download:', err)
    updateStore.setHistoryDownload(version, {
      state: 'error',
      error: t('installCancelFailed', { error: String(err) }),
    })
  }
}

async function openDownloadedFolder(version: string) {
  const download = updateStore.getHistoryDownload(version)
  if (!download?.savedPath) return
  // 从保存路径取父目录
  const path = download.savedPath
  const lastSep = Math.max(path.lastIndexOf('/'), path.lastIndexOf('\\'))
  const dir = lastSep >= 0 ? path.substring(0, lastSep) : path
  try {
    openExternal(dir)
  } catch (err) {
    console.error('Failed to open folder:', err)
  }
}

onMounted(async () => {
  // 自动检查 CC-Box 更新（未检查过时）：托盘「更新到新版本」跳转进来即可看到结果
  if (!updateStore.updateInfo) void handleCheckUpdate()

  // 自动加载版本列表（带缓存）
  await updateStore.loadClaudeVersionList()

  // 监听历史版本下载进度事件
  unlistenHistoryProgress = await onInstallProgress((progress) => {
    if (progress.item !== 'claude-history') return
    // 找到当前正在下载的版本（state === 'downloading'）
    for (const [version, info] of updateStore.historyDownloads) {
      if (info.state === 'downloading') {
        if (progress.stage === 'done') {
          // done 由 handleDownloadVersion 的 await 返回值统一处理
        } else if (progress.stage === 'error') {
          updateStore.setHistoryDownload(version, {
            state: 'error',
            error: progress.message,
          })
        } else {
          updateStore.setHistoryDownload(version, {
            progress: progress.progress,
            message: progress.message,
          })
        }
        break
      }
    }
  })
})

onUnmounted(() => {
  unlistenHistoryProgress?.()
})
</script>

<style scoped>
.section-content {
  padding: 8px 0;
}

.section-heading {
  font-size: 20px;
  font-weight: 600;
  color: var(--text-primary);
  margin-bottom: 24px;
}

.update-card {
  background: var(--bg-secondary);
  border: 1px solid var(--border-color);
  border-radius: 8px;
  padding: 20px;
}

.claude-cli-card {
  margin-top: 20px;
}

.version-row {
  display: flex;
  align-items: center;
  justify-content: space-between;
}

.version-info {
  display: flex;
  flex-direction: column;
  gap: 2px;
}

.version-label {
  font-size: 12px;
  color: var(--text-tertiary);
}

.version-value {
  font-size: 16px;
  font-weight: 600;
  color: var(--text-primary);
  font-variant-numeric: tabular-nums;
}

.not-installed {
  color: var(--text-tertiary);
  font-style: italic;
  font-weight: 400;
}

.check-btn {
  display: flex;
  align-items: center;
  gap: 6px;
  padding: 8px 16px;
  background: var(--accent-color);
  color: white;
  border: none;
  border-radius: 6px;
  font-size: 13px;
  font-weight: 500;
  cursor: pointer;
  transition: opacity 0.15s ease;
}

.check-btn:hover:not(:disabled) {
  opacity: 0.9;
}

.check-btn:disabled {
  opacity: 0.6;
  cursor: not-allowed;
}

.check-btn img {
  width: 14px;
  height: 14px;
}

.spinning {
  animation: spin 1s linear infinite;
}

@keyframes spin {
  from { transform: rotate(0deg); }
  to { transform: rotate(360deg); }
}

.update-message {
  margin-top: 16px;
  padding: 10px 14px;
  border-radius: 6px;
  font-size: 13px;
}

.update-message.error {
  background: rgba(239, 68, 68, 0.1);
  color: var(--error-color);
}

.update-message.success {
  background: rgba(34, 197, 94, 0.1);
  color: var(--success-color);
}

.update-available {
  margin-top: 16px;
}

.update-banner {
  display: flex;
  align-items: center;
  gap: 12px;
  padding: 12px 16px;
  background: var(--accent-light);
  border-radius: 6px;
  margin-bottom: 16px;
}

.update-icon {
  font-size: 20px;
}

.update-version {
  font-size: 14px;
  font-weight: 600;
  color: var(--text-primary);
  display: block;
}

.update-hint {
  font-size: 12px;
  color: var(--text-secondary);
}

.release-notes {
  margin-bottom: 16px;
}

.release-notes h4 {
  font-size: 13px;
  font-weight: 600;
  color: var(--text-secondary);
  margin-bottom: 8px;
}

.notes-content {
  font-size: 13px;
  color: var(--text-primary);
  line-height: 1.6;
  max-height: 200px;
  overflow-y: auto;
  padding: 12px;
  background: var(--bg-primary);
  border-radius: 6px;
  border: 1px solid var(--border-color);
}

.update-actions {
  display: flex;
  flex-direction: column;
  gap: 10px;
}

.action-row {
  display: flex;
  gap: 12px;
  align-items: center;
}

.action-btn {
  display: inline-flex;
  align-items: center;
  justify-content: center;
  padding: 10px 20px;
  border-radius: 6px;
  font-size: 14px;
  font-weight: 500;
  cursor: pointer;
  transition: opacity 0.15s ease;
  text-decoration: none;
  border: none;
}

.action-btn:hover {
  opacity: 0.9;
}

.action-btn.primary {
  background: var(--accent-color);
  color: white;
}

.action-btn.secondary {
  background: transparent;
  border: 1px solid var(--border-color);
  color: var(--text-secondary);
}

.action-btn.secondary:hover {
  border-color: var(--accent-color);
  color: var(--accent-color);
}

.progress-section {
  display: flex;
  flex-direction: column;
  gap: 8px;
}

.progress-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  font-size: 13px;
  color: var(--text-secondary);
}

.progress-bar {
  height: 8px;
  background: var(--bg-primary);
  border-radius: 4px;
  overflow: hidden;
  border: 1px solid var(--border-color);
}

.progress-fill {
  height: 100%;
  background: var(--accent-color);
  border-radius: 4px;
  transition: width 0.3s ease;
}

.progress-info {
  display: flex;
  justify-content: space-between;
  font-size: 12px;
  color: var(--text-secondary);
}

.progress-size {
  color: var(--text-tertiary);
}

.installing-message {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 8px;
  padding: 20px;
  background: var(--accent-light);
  border-radius: 6px;
}

.spinning-text {
  font-size: 14px;
  font-weight: 500;
  color: var(--text-primary);
  animation: pulse-text 1.5s ease-in-out infinite;
}

.installing-hint {
  font-size: 12px;
  color: var(--text-secondary);
}

@keyframes pulse-text {
  0%, 100% { opacity: 1; }
  50% { opacity: 0.5; }
}

.error-actions {
  display: flex;
  gap: 12px;
  margin-top: 8px;
}

.retry-link {
  background: none;
  border: none;
  color: var(--accent-color);
  cursor: pointer;
  font-size: 13px;
  text-decoration: underline;
  padding: 0;
}

.confirm-overlay {
  position: fixed;
  inset: 0;
  background: rgba(0, 0, 0, 0.3);
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 300;
}

.confirm-dialog {
  background: var(--bg-primary);
  border: 1px solid var(--border-color);
  border-radius: var(--radius-lg);
  padding: 24px;
  /* 手机视口内收缩（100vw - 弹窗 padding - 视口边距） */
  min-width: min(320px, calc(100vw - 64px));
  box-shadow: var(--shadow-xl);
}

.confirm-text {
  font-size: 14px;
  font-weight: 500;
  color: var(--text-primary);
  margin: 0 0 20px 0;
}

.confirm-actions {
  display: flex;
  justify-content: flex-end;
  gap: 10px;
}

.confirm-actions .btn-cancel {
  padding: 7px 18px;
  background: var(--bg-secondary);
  color: var(--text-primary);
  border: 1px solid var(--border-color);
  border-radius: var(--radius-md);
  font-size: 13px;
  cursor: pointer;
  font-family: var(--font-sans);
}

.confirm-actions .btn-cancel:hover {
  background: var(--hover-bg);
}

.confirm-actions .btn-primary {
  padding: 7px 18px;
  background: var(--accent-color);
  color: #fff;
  border: none;
  border-radius: var(--radius-md);
  font-size: 13px;
  cursor: pointer;
  font-family: var(--font-sans);
}

.confirm-actions .btn-primary:hover {
  opacity: 0.9;
}

/* ============ Claude CLI 版本列表 ============ */

.version-list-wrapper {
  margin-top: 16px;
}

.version-list-header {
  display: flex;
  align-items: baseline;
  justify-content: space-between;
  gap: 12px;
  margin-bottom: 4px;
}

.version-list-title {
  font-size: 13px;
  font-weight: 600;
  color: var(--text-secondary);
}

.version-list-count {
  font-size: 12px;
  color: var(--text-tertiary);
  font-variant-numeric: tabular-nums;
}

.version-list-hint {
  font-size: 12px;
  color: var(--text-tertiary);
  margin-bottom: 12px;
}

.version-empty {
  padding: 24px 12px;
  text-align: center;
  font-size: 13px;
  color: var(--text-tertiary);
  background: var(--bg-primary);
  border-radius: 6px;
  border: 1px dashed var(--border-color);
}

.version-list {
  max-height: 320px;
  overflow-y: auto;
  border: 1px solid var(--border-color);
  border-radius: 6px;
  background: var(--bg-primary);
}

.version-item {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 10px 14px;
  border-bottom: 1px solid var(--border-color);
  gap: 12px;
}

.version-item:last-child {
  border-bottom: none;
}

.version-item-info {
  display: flex;
  align-items: center;
  gap: 8px;
  flex: 1;
  min-width: 0;
}

.version-item-number {
  font-size: 13px;
  font-weight: 600;
  color: var(--text-primary);
  font-variant-numeric: tabular-nums;
}

.version-item-date {
  font-size: 12px;
  color: var(--text-tertiary);
  font-variant-numeric: tabular-nums;
}

.version-tag {
  font-size: 11px;
  padding: 2px 8px;
  border-radius: 10px;
  font-weight: 500;
}

.installed-tag {
  background: rgba(34, 197, 94, 0.15);
  color: var(--success-color);
}

.latest-tag {
  background: var(--accent-light);
  color: var(--accent-color);
}

.version-item-actions {
  display: flex;
  align-items: center;
  gap: 8px;
  flex-shrink: 0;
}

.version-item-disabled {
  font-size: 12px;
  color: var(--text-tertiary);
  font-style: italic;
}

.version-item-error {
  font-size: 12px;
  color: var(--error-color);
  max-width: 120px;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.version-item-cancelled {
  font-size: 12px;
  color: var(--text-tertiary);
  font-style: italic;
}

.version-item-installing {
  font-size: 12px;
  color: var(--accent-color);
  animation: pulse-text 1.5s ease-in-out infinite;
}

.cancel-btn {
  padding: 3px 10px;
  font-size: 11px;
}

.confirm-hint {
  font-size: 12px;
  color: var(--text-secondary);
  margin: 0 0 16px 0;
  line-height: 1.5;
}

.version-progress {
  display: flex;
  align-items: center;
  gap: 8px;
  min-width: 140px;
}

.version-progress-bar {
  flex: 1;
  height: 6px;
  background: var(--bg-secondary);
  border-radius: 3px;
  overflow: hidden;
  border: 1px solid var(--border-color);
}

.version-progress-fill {
  height: 100%;
  background: var(--accent-color);
  border-radius: 3px;
  transition: width 0.3s ease;
}

.version-progress-text {
  font-size: 12px;
  color: var(--text-secondary);
  font-variant-numeric: tabular-nums;
  min-width: 32px;
  text-align: right;
}

.action-btn.small {
  padding: 5px 12px;
  font-size: 12px;
}
</style>
