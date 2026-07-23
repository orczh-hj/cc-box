<template>
<div class="app-root">
  <TitleBar />

  <!-- 环境检查提示 -->
  <div v-if="appStore.checkFailed" class="check-failed-overlay">
    <div class="check-failed-card">
      <h2>{{ t('environmentCheck') }}</h2>
      <p class="check-hint">{{ t('envCheckHint') }}</p>
      <div class="check-list">
        <div v-for="check in appStore.checkResults" :key="check.name"
          class="check-item" :class="{ passed: check.passed, failed: !check.passed }">
          <div class="check-status-icon">{{ check.passed ? '✓' : '✗' }}</div>
          <div class="check-detail">
            <div class="check-item-header">
              <span class="check-name">{{ check.name }}</span>
              <span class="check-message">{{ check.message }}</span>
            </div>
            <div v-if="check.passed && check.detectedPath" class="check-detected-path">
              {{ check.detectedPath }}
            </div>

            <!-- 失败项：自定义路径输入 + 文档链接 -->
            <template v-if="!check.passed">
              <div class="check-custom-path">
                <input
                  type="text"
                  class="custom-path-input"
                  v-model="customPathInputs[configKeyFor(check.name)]"
                  :placeholder="t('customPathPlaceholder')"
                  :disabled="isInstalling"
                />
                <button
                  class="custom-path-apply-btn"
                  @click="applyCustomPath(check.name, configKeyFor(check.name))"
                  :disabled="isInstalling"
                >
                  {{ t('customPathApply') }}
                </button>
              </div>
              <p v-if="customPathErrors[check.name]" class="custom-path-error">{{ customPathErrors[check.name] }}</p>
              <a v-if="check.url" href="javascript:void(0)" class="check-doc-link" @click="openUrl(check.url)">
                {{ check.action || t('viewDocs') }} →
              </a>
            </template>
          </div>
        </div>
      </div>

      <!-- 安装进度显示（多任务） -->
      <div v-if="isInstalling" class="install-progress-section">
        <div class="install-tasks">
          <div v-for="task in installTasks" :key="task.name" class="install-task" :class="{ 'install-task-error': task.status === 'error' || task.status === 'cancelled' }">
            <div class="install-task-header">
              <span class="install-task-name">{{ task.name }}</span>
              <span class="install-task-status" :class="{ 'status-error': task.status === 'error', 'status-cancelled': task.status === 'cancelled' }">{{ stageLabel(task.status) }}</span>
            </div>
            <div class="install-task-progress-bar">
              <div class="install-task-progress-fill" :class="{ 'progress-fill-error': task.status === 'error', 'progress-fill-cancelled': task.status === 'cancelled' }" :style="{ width: task.progress + '%' }"></div>
            </div>
            <p v-if="task.message && (task.status === 'error' || task.status === 'cancelled')" class="install-task-message">{{ task.message }}</p>
          </div>
        </div>
      </div>

      <!-- 按钮：Auto Install / Cancel / Retry -->
      <div class="check-btn-row">
        <button v-if="!isInstalling" class="check-auto-btn" @click="autoInstall">
          {{ t('autoInstall') }}
        </button>
        <button v-if="isInstalling" class="check-cancel-btn" @click="cancelInstall">
          {{ t('cancelInstall') }}
        </button>
        <button class="check-retry-btn" @click="retryChecks" :disabled="isInstalling">
          {{ t('retry') }}
        </button>
      </div>
    </div>
  </div>

  <!-- 全局设置浮层 -->
  <SettingsOverlay />

  <!-- 终端视图常驻 DOM，保持所有 PTY 和终端实例不销毁 -->
  <TerminalView
    ref="terminalViewRef"
    v-show="currentView === 'terminal'"
    :visible="currentView === 'terminal'"
    @back="handleBack"
    @select-project="handleOpenProject"
  />

  <!-- 覆盖层视图（固定定位叠加在终端之上） -->
  <Transition name="fade" mode="out-in">
    <WelcomeView
      v-if="currentView === 'welcome'"
      class="overlay-view"
      @select-project="handleSelectProject"
    />
    <ProjectSelectView
      v-else-if="currentView === 'projects'"
      class="overlay-view"
      @select-project="handleOpenProject"
      @add-project="handleSelectProject"
      @resume-session="handleResumeSession"
      @open-settings="sidebarStore.openSettings()"
    />
  </Transition>
</div>
</template>

<script setup lang="ts">
import { ref, onMounted, onUnmounted, defineAsyncComponent, nextTick, reactive } from 'vue'
import { useAppStore } from '@/stores/app'
import { useHookStore } from '@/stores/hook'
import { useSessionStore } from '@/stores/session'
import { useSidebarStore } from '@/stores/sidebar'
import { useUpdateStore } from '@/stores/update'
import { useI18n } from 'vue-i18n'
import { getCurrentWindow } from '@tauri-apps/api/window'
import { open } from '@tauri-apps/plugin-shell'
import {
  selectDirectory,
  onMenuSettings,
  onMenuShortcuts,
  onConfigFontSize,
  onTerminalRestart,
  checkForUpdates,
  getInstalledClaudeVersion,
  downloadAndInstallClaude,
  downloadAndInstallGit,
  cancelAutoInstall,
  updateAppConfig,
  onInstallProgress,
  onOpenDirectory
} from '@/api/tauri'
import { useAppShortcuts } from '@/composables/useAppShortcuts'
import WelcomeView from '@/components/WelcomeView.vue'
import ProjectSelectView from '@/components/ProjectSelectView.vue'
import TitleBar from '@/components/TitleBar.vue'

const TerminalView = defineAsyncComponent(() => import('@/components/TerminalView.vue'))
const SettingsOverlay = defineAsyncComponent(() => import('@/components/settings/SettingsOverlay.vue'))

type ViewType = 'welcome' | 'projects' | 'terminal'

const appStore = useAppStore()
const sessionStore = useSessionStore()
const sidebarStore = useSidebarStore()
const updateStore = useUpdateStore()
const { t } = useI18n()
const { setupShortcutListeners } = useAppShortcuts()
const currentView = ref<ViewType>('welcome')
const terminalViewRef = ref()

// 自动安装状态
const isInstalling = ref(false)

// 多任务进度跟踪
interface InstallTask {
  name: string       // "Claude CLI" | "Git"
  status: string     // "waiting" | "fetching" | "downloading" | "extracting" | "placing" | "verifying" | "done" | "error" | "cancelled"
  progress: number   // 0-100
  message: string
}

const installTasks = ref<InstallTask[]>([])

// 失败项的自定义路径输入（key = claudePath / gitBashPath）
const customPathInputs = reactive<Record<string, string>>({})
const customPathErrors = reactive<Record<string, string>>({})

/** check.name → app config key 的映射 */
function configKeyFor(checkName: string): string {
  if (checkName === 'Claude CLI') return 'claudePath'
  if (checkName === 'Git Bash') return 'gitBashPath'
  return ''
}

/** 把后端 stage 字符串翻译为当前语言展示 */
function stageLabel(stage: string): string {
  const map: Record<string, string> = {
    waiting: t('stageWaiting'),
    fetching: t('stageFetching'),
    downloading: t('stageDownloading'),
    extracting: t('stageExtracting'),
    placing: t('stagePlacing'),
    verifying: t('stageVerifying'),
    done: t('stageDone'),
    error: t('stageError'),
    cancelled: t('stageCancelled'),
  }
  return map[stage] ?? stage
}

/** 应用自定义路径：保存到 config 后立即重试检查 */
async function applyCustomPath(checkName: string, configKey: string) {
  const value = (customPathInputs[configKey] || '').trim()
  if (!value) {
    customPathErrors[checkName] = t('customPathInvalid')
    return
  }
  delete customPathErrors[checkName]
  try {
    await updateAppConfig({ [configKey]: value })
    customPathInputs[configKey] = ''
    await appStore.runChecks(true)
    if (!appStore.checkFailed) {
      initAfterChecks()
    }
  } catch (e) {
    customPathErrors[checkName] = String(e)
  }
}

/** 取消正在进行的自动安装 */
async function cancelInstall() {
  try {
    await cancelAutoInstall()
  } catch (e) {
    console.error('Failed to cancel auto install:', e)
  }
}

let unlistenInstallProgress: (() => void) | null = null
let unlistenOpenDir: (() => void) | null = null

// Unlisten functions for cleanup
let unlistenSettings: (() => void) | null = null
let unlistenShortcuts: (() => void) | null = null
let unlistenFontSize: (() => void) | null = null
let unlistenRestart: (() => void) | null = null
const shortcutUnlisteners: (() => void)[] = []

onMounted(async () => {
  await appStore.runChecks()
  if (appStore.checkFailed) {
    return
  }

  initAfterChecks()
})

onUnmounted(() => {
  unlistenSettings?.()
  unlistenShortcuts?.()
  unlistenFontSize?.()
  unlistenRestart?.()
  unlistenInstallProgress?.()
  unlistenOpenDir?.()
  shortcutUnlisteners.forEach(fn => fn())
  window.removeEventListener('app:toggleHome', handleToggleHome)
})

async function handleSelectProject() {
  const result = await selectDirectory()
  if (result) {
    const normalizePath = (p: string) => p.replace(/\\/g, '/').toLowerCase()
    const normalizedResult = normalizePath(result.path)
    const existingProject = appStore.cachedProjects.find(
      p => normalizePath(p.path) === normalizedResult
    )
    if (existingProject) {
      handleOpenProject(result.path)
    } else {
      appStore.setCwd(result.path)
      // 保留用户的启动参数设置，只清除 resume（新项目不需要恢复会话）
      appStore.setClaudeOptions({
        resume: '',
        // 保留 skipPermissions 和 customArgs
      })
      currentView.value = 'terminal'
      sidebarStore.loadAllSidebarData(result.path)
    }
  }
}

async function handleOpenProject(path: string) {
  appStore.setCwd(path)
  currentView.value = 'terminal'
  // 启动时自动加载 sidebar 数据
  sidebarStore.loadAllSidebarData(path)

  // 当前激活 tab 属于该项目则保持，否则切换到运行中 Tab
  const activeTab = sessionStore.activeTab
  if (activeTab && activeTab.projectPath === path) return

  await nextTick()
  const runningTab = sessionStore.getRunningTabForProject(path)
  if (runningTab) {
    sessionStore.setActiveTab(runningTab.tabId)
  }
}

async function handleResumeSession(projectPath: string, sessionId: string, sessionName?: string) {
  appStore.setCwd(projectPath)

  // 如果该 session 已在运行，直接切换到对应 tab
  const existingTab = sessionStore.getTabBySessionId(sessionId)
  if (existingTab && existingTab.status === 'running') {
    sessionStore.setActiveTab(existingTab.tabId)
    currentView.value = 'terminal'
    sidebarStore.loadAllSidebarData(projectPath)
    return
  }

  currentView.value = 'terminal'
  sidebarStore.loadAllSidebarData(projectPath)

  // 直接调用 TerminalView 方法恢复会话（不再依赖 watch）
  await nextTick()
  if (terminalViewRef.value?.startResumeSession) {
    terminalViewRef.value.startResumeSession(projectPath, sessionId, sessionName)
  } else {
    // 异步组件尚未加载的 fallback
    appStore.setClaudeOptions({ resume: sessionId })
    appStore.setPendingResume(sessionId, sessionName)
  }
}

function handleBack() {
  currentView.value = 'projects'
}

function handleToggleHome() {
  if (currentView.value === 'terminal') {
    currentView.value = 'projects'
  } else if (currentView.value === 'projects' && appStore.cwd) {
    currentView.value = 'terminal'
  }
}

function openUrl(url: string) {
  open(url)
}

async function retryChecks() {
  await appStore.runChecks(true)
  if (!appStore.checkFailed) {
    initAfterChecks()
  }
}

function initAfterChecks() {
  // 应用主题到 DOM（loadAppConfig 已在 onMounted 中调用并设置 theme 值）
  const html = document.documentElement
  html.setAttribute('data-theme', appStore.theme)
  html.classList.remove('light', 'dark')
  html.classList.add(appStore.theme)

  // 并行启动所有独立初始化任务
  appStore.loadCache().then(() => {
    if (appStore.cachedProjects.length > 0 && currentView.value === 'welcome') {
      currentView.value = 'projects'
    }
  })
  appStore.loadAppConfig()
  useHookStore().init()

  shortcutUnlisteners.push(...setupShortcutListeners())

  onMenuSettings(() => {
    sidebarStore.openSettings()
  }).then(fn => { unlistenSettings = fn })

  onMenuShortcuts(() => {
    sidebarStore.openSettings('shortcuts')
  }).then(fn => { unlistenShortcuts = fn })

  onConfigFontSize((size) => {
    appStore.setFontSize(size)
  }).then(fn => { unlistenFontSize = fn })

  onTerminalRestart((data) => {
    if (currentView.value === 'terminal') {
      appStore.setCwd(data.cwd)
    }
  }).then(fn => { unlistenRestart = fn })

  window.addEventListener('app:toggleHome', handleToggleHome)

  checkForUpdates().then(info => {
    if (info) {
      sidebarStore.setUpdateInfo(info)
      updateStore.setUpdateInfo(info)
    }
  }).catch(() => {})

  // 启动只读本地 Claude CLI 版本号，不发 HTTP 请求对比 OSS
  getInstalledClaudeVersion().then(version => {
    updateStore.setInstalledClaudeVersion(version)
  }).catch(() => {})

  // 监听右键菜单传入的目录
  onOpenDirectory((dir) => {
    if (appStore.isKnownProject(dir)) {
      handleOpenProject(dir)
    } else {
      appStore.setCwd(dir)
      appStore.setClaudeOptions({ resume: '' })
      currentView.value = 'terminal'
      sidebarStore.loadAllSidebarData(dir)
    }
  }).then(fn => { unlistenOpenDir = fn })
}

// 自动安装（并发执行）
async function autoInstall() {
  isInstalling.value = true

  // 初始化任务列表
  installTasks.value = []

  // 根据检查结果添加任务
  for (const check of appStore.checkResults) {
    if (!check.passed) {
      installTasks.value.push({
        name: check.name,
        status: 'waiting',
        progress: 0,
        message: t('installWaiting')
      })
    }
  }

  // 监听进度事件
  onInstallProgress((progress) => {
    const taskName = progress.item === 'claude' ? 'Claude CLI' : 'Git Bash'
    const task = installTasks.value.find(t => t.name === taskName)
    if (task) {
      task.status = progress.stage
      task.progress = progress.progress
      task.message = progress.message
    }
  }).then(fn => { unlistenInstallProgress = fn })

  try {
    // 并发安装所有缺失项
    const installPromises: Promise<void>[] = []

    const needsClaude = appStore.checkResults.some(c => c.name === 'Claude CLI' && !c.passed)
    if (needsClaude) {
      installPromises.push(
        downloadAndInstallClaude().then(() => {
          const task = installTasks.value.find(item => item.name === 'Claude CLI')
          if (task && task.status !== 'cancelled' && task.status !== 'error') {
            task.status = 'done'
            task.progress = 100
            task.message = t('installComplete')
          }
        })
      )
    }

    const needsGit = appStore.checkResults.some(c => c.name === 'Git Bash' && !c.passed)
    if (needsGit) {
      installPromises.push(
        downloadAndInstallGit().then(() => {
          const task = installTasks.value.find(item => item.name === 'Git Bash')
          if (task && task.status !== 'cancelled' && task.status !== 'error') {
            task.status = 'done'
            task.progress = 100
            task.message = t('installComplete')
          }
        })
      )
    }

    // 等待所有安装完成（任何一个失败/取消都会 reject）
    await Promise.all(installPromises)

    // 延迟一秒后重新检查
    await new Promise(r => setTimeout(r, 1000))

    // 重新运行检查（会自动添加 PATH）
    await appStore.runChecks(true)

    // 检查是否全部通过
    if (!appStore.checkFailed) {
      initAfterChecks()
    } else {
      // 如果仍有失败项，更新错误信息（保留 error/cancelled 任务的原始 message）
      for (const task of installTasks.value) {
        if (task.status === 'done') continue
        if (task.status === 'error' || task.status === 'cancelled') continue
        task.status = 'error'
        task.message = t('installVerifyFailed')
      }
    }
  } catch (e) {
    const errorMsg = String(e)
    // 区分取消与其他错误
    const isCancelled = errorMsg === 'cancelled'
    for (const task of installTasks.value) {
      if (task.status === 'done') continue
      // 已被 progress 事件标记为 error/cancelled 的，保留其具体 message
      if (task.status === 'error' || task.status === 'cancelled') continue
      task.status = isCancelled ? 'cancelled' : 'error'
      task.message = isCancelled ? t('stageCancelled') : (errorMsg || t('installFailed', { error: errorMsg }))
    }
    if (!isCancelled) {
      console.error('Auto install failed:', e)
    }
  } finally {
    isInstalling.value = false
    unlistenInstallProgress?.()
    unlistenInstallProgress = null
  }
}
</script>

<style scoped>
.app-root {
  display: flex;
  flex-direction: column;
  height: 100vh;
  overflow: hidden;
}

.overlay-view {
  position: fixed;
  top: 32px;
  left: 0;
  right: 0;
  bottom: 0;
  z-index: 10;
}

.fade-enter-active,
.fade-leave-active {
  transition: opacity 0.2s ease;
}

.fade-enter-from,
.fade-leave-to {
  opacity: 0;
}

.check-failed-overlay {
  position: fixed;
  top: 32px;
  left: 0;
  right: 0;
  bottom: 0;
  z-index: 100;
  display: flex;
  align-items: center;
  justify-content: center;
  background: rgba(0, 0, 0, 0.5);
}

.check-failed-card {
  background: var(--bg-primary);
  border-radius: var(--radius-lg);
  padding: 24px 32px;
  max-width: 480px;
  width: 90%;
  box-shadow: var(--shadow-lg);
  max-height: 90vh;
  overflow-y: auto;
}

.check-hint {
  font-size: 12px;
  color: var(--text-secondary);
  margin-bottom: 16px;
  line-height: 1.5;
}

.check-failed-card h2 {
  font-size: 16px;
  font-weight: 600;
  color: var(--status-error);
  margin-bottom: 16px;
}

.check-list {
  display: flex;
  flex-direction: column;
  gap: 12px;
  margin-bottom: 20px;
}

.check-item {
  display: flex;
  gap: 10px;
  padding: 10px 12px;
  background: var(--bg-secondary);
  border-radius: var(--radius-md);
}

.check-item.passed {
  border-left: 3px solid var(--status-success, #4caf50);
}

.check-item.failed {
  border-left: 3px solid var(--status-error, #ef5350);
}

.check-status-icon {
  flex-shrink: 0;
  width: 20px;
  height: 20px;
  display: flex;
  align-items: center;
  justify-content: center;
  font-size: 14px;
  font-weight: 700;
}

.check-item.passed .check-status-icon {
  color: var(--status-success, #4caf50);
}

.check-item.failed .check-status-icon {
  color: var(--status-error, #ef5350);
}

.check-detail {
  flex: 1;
  display: flex;
  flex-direction: column;
  gap: 4px;
  min-width: 0;
}

.check-item-header {
  display: flex;
  flex-direction: column;
  gap: 2px;
}

.check-name {
  font-size: 13px;
  font-weight: 600;
  color: var(--text-primary);
}

.check-message {
  font-size: 12px;
  color: var(--text-secondary);
}

.check-detected-path {
  font-size: 11px;
  color: var(--text-secondary);
  font-family: var(--font-mono);
  padding: 2px 6px;
  background: var(--bg-primary);
  border-radius: var(--radius-sm);
  word-break: break-all;
}

/* 失败项自定义路径输入 */
.check-custom-path {
  display: flex;
  gap: 6px;
  margin-top: 8px;
}

.custom-path-input {
  flex: 1;
  min-width: 0;
  padding: 6px 8px;
  font-size: 12px;
  font-family: var(--font-mono);
  background: var(--bg-primary);
  border: 1px solid var(--border-color);
  border-radius: var(--radius-sm);
  color: var(--text-primary);
}

.custom-path-input:focus {
  outline: none;
  border-color: var(--accent-primary);
}

.custom-path-input:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}

.custom-path-apply-btn {
  padding: 6px 10px;
  font-size: 12px;
  background: var(--accent-primary);
  color: var(--text-inverse);
  border: none;
  border-radius: var(--radius-sm);
  cursor: pointer;
  white-space: nowrap;
}

.custom-path-apply-btn:hover:not(:disabled) {
  opacity: 0.9;
}

.custom-path-apply-btn:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}

.custom-path-error {
  margin-top: 4px;
  font-size: 11px;
  color: var(--status-error, #ef5350);
}

.check-doc-link {
  display: inline-block;
  margin-top: 6px;
  font-size: 11px;
  color: var(--accent-primary);
  text-decoration: none;
}

.check-doc-link:hover {
  text-decoration: underline;
}

.check-btn-row {
  display: flex;
  gap: 8px;
  margin-top: 16px;
}

.check-auto-btn {
  flex: 1;
  padding: 10px;
  background: var(--status-success, #4caf50);
  color: var(--text-inverse);
  border: none;
  border-radius: var(--radius-md);
  font-size: 14px;
  font-weight: 600;
  cursor: pointer;
}

.check-auto-btn:hover:not(:disabled) {
  opacity: 0.9;
}

.check-auto-btn:disabled {
  opacity: 0.6;
  cursor: wait;
}

.check-retry-btn {
  flex: 1;
  padding: 10px;
  background: transparent;
  border: 1px solid var(--border-color);
  border-radius: var(--radius-md);
  font-size: 14px;
  font-weight: 600;
  color: var(--text-secondary);
  cursor: pointer;
}

.check-retry-btn:hover:not(:disabled) {
  border-color: var(--accent-primary);
  color: var(--accent-primary);
}

.check-retry-btn:disabled {
  opacity: 0.5;
  cursor: wait;
}

.check-cancel-btn {
  flex: 1;
  padding: 10px;
  background: transparent;
  border: 1px solid var(--status-error, #ef5350);
  color: var(--status-error, #ef5350);
  border-radius: var(--radius-md);
  font-size: 14px;
  font-weight: 600;
  cursor: pointer;
}

.check-cancel-btn:hover {
  background: var(--status-error, #ef5350);
  color: var(--text-inverse);
  opacity: 0.9;
}

/* 多任务进度 */
.install-progress-section {
  margin-top: 16px;
  padding: 16px;
  background: var(--bg-secondary);
  border-radius: var(--radius-md);
}

.install-tasks {
  display: flex;
  flex-direction: column;
  gap: 12px;
}

.install-task {
  display: flex;
  flex-direction: column;
  gap: 6px;
}

.install-task-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
}

.install-task-name {
  font-size: 13px;
  font-weight: 600;
  color: var(--text-primary);
}

.install-task-status {
  font-size: 12px;
  color: var(--text-secondary);
}

.install-task-progress-bar {
  height: 6px;
  background: var(--bg-primary);
  border-radius: 3px;
  overflow: hidden;
}

.install-task-progress-fill {
  height: 100%;
  background: var(--accent-primary);
  transition: width 0.3s ease;
}

.install-task-message {
  margin-top: 4px;
  font-size: 11px;
  color: var(--text-secondary);
  word-break: break-word;
  line-height: 1.4;
}

.status-error {
  color: var(--status-error, #ef5350);
}

.status-cancelled {
  color: var(--text-tertiary);
}

.progress-fill-error {
  background: var(--status-error, #ef5350);
}

.progress-fill-cancelled {
  background: var(--text-tertiary);
}
</style>