<template>
  <div class="terminal-view" data-terminal-view>
    <!-- 图标栏（常驻） -->
    <IconBar
      :active-panel="sidebarStore.activePanel"
      @toggle="handleTogglePanel"
      @toggle-settings="sidebarStore.toggleSettings()"
      @open-folder="handleOpenFolder"
      @open-remote="showRemoteModal = true"
    />

    <!-- 远程访问弹窗（二维码 + 复制地址） -->
    <RemoteModal :visible="showRemoteModal" @close="showRemoteModal = false" />

    <!-- 侧边栏 + 终端 -->
    <SidebarPanel
      :visible="sidebarStore.panelVisible"
      :active-panel="sidebarStore.activePanel"
      @close="sidebarStore.closePanel"
      @switch-session="handleSwitchSession"
      @rename-session="handleRenameSession"
      @restart-session="handleRestartSession"
      @new-session="handleNewSession"
      @resume-session="handleResumeSession"
      @close-tab="handleCloseTab"
      @close-all-tabs="handleCloseAllTabs"
      @close-other-tabs="handleCloseOtherTabs"
    />

    <!-- 主内容区 -->
    <div class="main-content">
      <TerminalHeader
        @back="handleBack"
      />
      <!-- CLI 不可用状态条：检查失败或 spawn 失败统一在此展示与重试 -->
      <div v-if="cliHealthBar" class="cli-health-bar">
        <span class="cli-health-text">{{ cliHealthText }}</span>
        <button class="cli-health-btn" @click="handleRetryChecks">{{ t('retry') }}</button>
      </div>
      <div class="terminal-container">
        <!-- 空状态提示：没有任何 session -->
        <div v-if="showEmptyState" class="empty-state-overlay">
          <div class="empty-state-content">
            <p class="empty-state-text">{{ t('startNewSession') }}</p>
            <button class="empty-state-btn" @click="handleNewSession">
              <img src="@/assets/icons/plus.svg" alt="New" />
              {{ t('newSession') }}
            </button>
          </div>
        </div>
        <XTermTerminal
          ref="terminalRef"
          :font-size="appStore.fontSize"
          @pty-started="handlePtyStarted"
        />
        <!-- 快捷键工具条（手机断点）：键盘唤出/选择复制 + 软键盘按不出的控制键 -->
        <QuickKeyBar v-if="isMobile" @send="handleQuickKey" @select="handleSelectCopy" />
        <!-- 复制阅读器（手机复制部分内容）：buffer 快照 + 反渲染，系统原生长按选择 -->
        <TerminalCopyReader
          v-if="copyReaderData"
          :visible="true"
          :smart="copyReaderData.smart"
          :raw="copyReaderData.raw"
          :truncated="copyReaderData.truncated"
          @close="copyReaderData = null"
        />
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, onUnmounted, watch, nextTick, computed } from 'vue'
import { useI18n } from 'vue-i18n'
import { useAppStore } from '@/stores/app'
import { useSessionStore } from '@/stores/session'
import { useSidebarStore, type SidebarPanelType } from '@/stores/sidebar'
import { useConfigStore } from '@/stores/config'
import { openInFileManager, logMessage, onRemoteSessionCreated, onRemoteSessionClosed, onSessionRenamed, renameSession } from '@/api/tauri'
import { sendTerminalCommand } from '@/composables/useTerminalCommand'
import { useWindowAttention } from '@/composables/useWindowAttention'
import { useStatusMonitor } from '@/composables/useStatusMonitor'
import { getCurrentWindow } from '@tauri-apps/api/window'
import { isTauriEnv } from '@/api/platform'
import TerminalHeader from './TerminalHeader.vue'
import XTermTerminal from './XTermTerminal.vue'
import IconBar from './IconBar.vue'
import QuickKeyBar from './QuickKeyBar.vue'
import TerminalCopyReader from './TerminalCopyReader.vue'
import { reflowSnapshot } from '@/utils/terminal-reflow'
import RemoteModal from './RemoteModal.vue'
import SidebarPanel from './sidebar/SidebarPanel.vue'
import { useIsMobile } from '@/composables/useMediaQuery'

const emit = defineEmits<{
  back: []
}>()

const props = defineProps<{
  visible?: boolean
}>()

const appStore = useAppStore()

// 远程访问弹窗
const showRemoteModal = ref(false)
// 远程新建会话监听清理函数
let unlistenRemoteCreated: (() => void) | null = null
// 远程关闭会话监听清理函数
let unlistenRemoteClosed: (() => void) | null = null
// 会话重命名同步监听清理函数
let unlistenSessionRenamed: (() => void) | null = null
const sessionStore = useSessionStore()
const sidebarStore = useSidebarStore()
const configStore = useConfigStore()
const terminalRef = ref()

const isMobile = useIsMobile()

// 复制阅读器数据（null = 关闭；v-if 挂载即新快照，不常驻 DOM）
const copyReaderData = ref<{ smart: string; raw: string; truncated: boolean } | null>(null)

/** 快捷键工具条：直发控制序列到当前显示终端（移动端不聚焦由 sendText
 *  内部 isMobile 守卫处理——点按键不召唤软键盘；桌面回焦让键盘输入立即可用） */
function handleQuickKey(bytes: string) {
  terminalRef.value?.sendText(bytes)
}

/** 「选择复制」按钮：抓 buffer 快照 → 反渲染 → 打开复制阅读器（系统原生长按选择） */
function handleSelectCopy() {
  const snap = terminalRef.value?.captureSnapshot?.()
  if (!snap || snap.lines.length === 0) return
  copyReaderData.value = reflowSnapshot(snap)
}

const { t } = useI18n()

// CLI 不可用状态条（检查失败或 spawn 失败的统一出口）
const cliHealthBar = computed(() => {
  const health = appStore.cliHealth
  return health && health.status !== 'ok' ? health : null
})

const cliHealthText = computed(() => {
  const health = cliHealthBar.value
  if (!health) return ''
  return health.status === 'missing'
    ? `${t('cliUnavailable')}: ${health.message}`
    : `${t('cliStartFailed')}: ${health.message}`
})

async function handleRetryChecks() {
  // failed = 执行失败，检查可能早已通过：重跑检查确认 CLI 可用后，
  // 还要重启当前 Tab 才算真正恢复（否则状态条消失而终端仍是错误页）
  const wasFailed = appStore.cliHealth?.status === 'failed'
  await appStore.runChecks(true)
  if (wasFailed && appStore.cliHealth?.status === 'ok') {
    await terminalRef.value?.restartCurrentPty?.()
  }
}

const { isFocused } = useWindowAttention()
const isTerminalVisible = computed(() => props.visible ?? false)
useStatusMonitor({ isFocused, isTerminalVisible })

// 空状态：当前项目没有任何 tab
const showEmptyState = computed(() => {
  const cwd = appStore.cwd
  if (!cwd) return false
  const projectTabs = sessionStore.getProjectTabs(cwd)
  return projectTabs.length === 0
})

// 会话面板打开时刷新服务端运行中会话列表（浏览器远程模式：
// 其他端新建/关闭的会话经此同步到本地 Tabs）
watch(() => sidebarStore.panelVisible && sidebarStore.activePanel === 'sessions', (open) => {
  if (open) terminalRef.value?.refreshRemoteSessions?.()
})

function updateWindowTitle(cwd: string) {
  if (!isTauriEnv) return
  const parts = cwd.replace(/\\/g, '/').split('/')
  const folderName = parts[parts.length - 1] || 'CC-Box'
  getCurrentWindow().setTitle(folderName).catch(() => {})
}

async function startResumeSession(projectPath: string, sessionId: string, sessionName?: string) {
  // 复用优先（resumeSessionTab）：已有该对话的 Tab 切换/重启，未命中才新建
  const { tabId } = sessionStore.resumeSessionTab(projectPath, sessionId, sessionName)
  sessionStore.setActiveTab(tabId)
  await nextTick()
  if (terminalRef.value?.startTab) {
    terminalRef.value.startTab(tabId)
  }
}

defineExpose({
  startResumeSession
})

// 初始化
onMounted(async () => {
  // 监听快捷键事件（必须在 cwd 检查之前设置）
  window.addEventListener('terminal:newSession', handleNewSession)
  window.addEventListener('terminal:restartSession', handleRestartSession)

  // 远程（手机）新建/恢复会话：建 tab 绑定 Rust 侧已 spawn 的 PTY
  // data 为 JSON：{cwd, resume, name}（resume 为空串表示新建；JSON 避免 cwd 含 \t 时截断；
  // name 为恢复会话的显示名——历史列表提供，防 Tab 名变 sessionId 编码）
  unlistenRemoteCreated = await onRemoteSessionCreated(({ id, data }) => {
    if (!data) return
    let cwd = ''
    let resume = ''
    let name = ''
    try {
      const parsed = JSON.parse(data)
      cwd = parsed.cwd ?? ''
      resume = parsed.resume ?? ''
      name = parsed.name ?? ''
    } catch {
      return
    }
    if (!cwd) return
    // 复用优先：resume 命中已有 Tab（手机从历史恢复、桌面已开着该对话）→
    // 重绑新 ptyId、保留原名，不新建（防重复 Tab 与 sessionId 编码名）
    const tabId = resume
      ? sessionStore.resumeSessionTab(cwd, resume, name || undefined).tabId
      : sessionStore.createTab(cwd)
    sessionStore.setTabPty(tabId, id)
    sessionStore.setActiveTab(tabId)
    void nextTick(() => terminalRef.value?.startTab(tabId))
  })

  // 远程（手机）显式关闭会话：桌面同步关闭对应 Tab（pty-exit 只置 stopped
  // 保留 Tab，远程关闭场景用户预期是关闭——与桌面 closeTab 语义对称）
  unlistenRemoteClosed = await onRemoteSessionClosed(({ id }) => {
    const tab = sessionStore.getTabByPtyId(id)
    if (tab) void sessionStore.closeTab(tab.tabId)
  })

  // 会话重命名双端同步：另一端（或本端回环，同名幂等）发起的重命名按
  // key（running=ptyId / stopped=sessionId 演变历史）应用到本地 Tab
  unlistenSessionRenamed = await onSessionRenamed(({ id, data }) => {
    sessionStore.applyRemoteRename(id, data)
  })

  const cwd = appStore.cwd
  if (!cwd) return

  updateWindowTitle(cwd)

  try {
    await sessionStore.loadHistorySessions(cwd)
    // 历史加载完成：回填远程采纳 Tab 的会话名（采纳时历史未到只能用项目名）
    sessionStore.backfillTabNamesFromHistory()
    configStore.loadProjectConfig(cwd)

    // 检查是否已有该项目的运行中 Tab
    const runningTab = sessionStore.getRunningTabForProject(cwd)

    if (appStore.pendingResume) {
      const { sessionId, sessionName } = appStore.pendingResume
      appStore.clearPendingResume()
      await startResumeSession(cwd, sessionId, sessionName)
    } else if (runningTab) {
      sessionStore.setActiveTab(runningTab.tabId)
    }
  } catch (err) {
    console.error('[TerminalView] onMounted ERROR:', err)
  }
})

onUnmounted(() => {
  window.removeEventListener('terminal:newSession', handleNewSession)
  window.removeEventListener('terminal:restartSession', handleRestartSession)
  unlistenRemoteCreated?.()
  unlistenRemoteClosed?.()
  unlistenSessionRenamed?.()
})

// KeepAlive 激活 → 改为 visible watcher（v-show 常驻 DOM）
watch(() => props.visible, async (isVisible) => {
  if (isVisible) {
    const cwd = appStore.cwd
    if (cwd) {
      updateWindowTitle(cwd)
      await nextTick()
      terminalRef.value?.fitCurrentTerminal?.()
      configStore.loadProjectConfig(cwd)
    }
  }
})

// 监听 cwd 变化
watch(() => appStore.cwd, async (newCwd, oldCwd) => {
  if (newCwd && newCwd !== oldCwd) {
    updateWindowTitle(newCwd)

    try {
      await sessionStore.loadHistorySessions(newCwd)
      configStore.loadProjectConfig(newCwd)
    } catch (err) {
      console.error('[TerminalView] cwd watch ERROR:', err)
    }
  }
})

function handleBack() {
  emit('back')
}

function handleTogglePanel(panel: SidebarPanelType) {
  sidebarStore.togglePanel(panel)
}

function handleOpenFolder() {
  if (appStore.cwd) openInFileManager(appStore.cwd)
}

// 切换到已有 Tab
function handleSwitchSession(tabId: string) {
  sessionStore.setActiveTab(tabId)
}

// 重命名会话（本地应用 + 广播双端同步）
function handleRenameSession(tabId: string, name: string) {
  const tab = sessionStore.tabs.get(tabId)
  if (tab?.status === 'running' && tab.ptyId) {
    sendTerminalCommand(`/rename ${name}\r`)
  }
  sessionStore.updateTabName(tabId, name)
  // 双端同步：广播 key = ptyId（running）或 sessionId（stopped，ptyId 已空——
  // 修复：stopped Tab 此前不广播导致另一端名字不同步）。回环幂等。
  // 手机端同一份代码走 rpc.call → rename_session → 同一广播
  const key = sessionStore.renameKeyForTab(tabId)
  if (key) {
    void renameSession(key, name).catch((e) => {
      logMessage('warn', `rename broadcast failed: ${e}`)
    })
  }
}

// 新建会话
function handleNewSession() {
  const cwd = appStore.cwd
  if (!cwd) {
    logMessage('warn', 'handleNewSession: no cwd')
    return
  }
  if (!terminalRef.value) {
    logMessage('warn', 'handleNewSession: no terminalRef')
    return
  }
  terminalRef.value.startNewSession(cwd)
}

// 重启会话
function handleRestartSession() {
  if (sessionStore.activeTabId && terminalRef.value) {
    terminalRef.value.restartTab(sessionStore.activeTabId)
  }
}

// 恢复历史会话（复用优先：已有 Tab 切换/重启，未命中才新建 Tab --resume）
function handleResumeSession(sessionId: string) {
  const cwd = appStore.cwd
  if (!cwd || !terminalRef.value) return

  // 查找历史会话的名称
  const historySession = sessionStore.historySessions.find(s => s.sessionId === sessionId)
  const name = historySession?.name

  const { tabId } = sessionStore.resumeSessionTab(cwd, sessionId, name)
  sessionStore.setActiveTab(tabId)

  terminalRef.value.startTab(tabId)
}

// 关闭 Tab
function handleCloseTab(tabId: string) {
  sessionStore.closeTab(tabId)
}

// 关闭所有 Tab
function handleCloseAllTabs() {
  const cwd = appStore.cwd
  if (cwd) {
    sessionStore.closeAllTabs(cwd)
  }
}

// 关闭其他 Tab
function handleCloseOtherTabs() {
  const activeId = sessionStore.activeTabId
  if (activeId) {
    sessionStore.closeOtherTabs(activeId)
  }
}

// PTY 启动回调
function handlePtyStarted(tabId: string, _ptyId: string) {
  const cwd = appStore.cwd
  if (cwd) {
    appStore.ensureProjectInList(cwd)
    // 恢复历史会话时不需要重载（claimedSessionIds 已自动过滤），仅新建会话时刷新
    const tab = sessionStore.tabs.get(tabId)
    if (!tab?.isResume) {
      sessionStore.loadHistorySessions(cwd, true)
    }
  }
}
</script>

<style scoped>
.terminal-view {
  display: flex;
  flex: 1;
  min-height: 0;
  background: var(--bg-primary);
}

/* 手机断点：IconBar 变 fixed 底部横栏，主内容让出空间 */
@media (max-width: 768px) {
  .terminal-view {
    padding-bottom: calc(52px + var(--safe-area-bottom));
  }
}

.main-content {
  flex: 1;
  display: flex;
  flex-direction: column;
  min-width: 0;
}

/* CLI 不可用状态条：不占终端高度预算（flex-shrink: 0），终端 fit 自动让位 */
.cli-health-bar {
  flex-shrink: 0;
  display: flex;
  align-items: center;
  gap: 8px;
  padding: 6px 12px;
  background: rgba(220, 80, 80, 0.12);
  border-bottom: 1px solid rgba(220, 80, 80, 0.35);
  color: var(--text-primary);
  font-size: 12px;
}

.cli-health-text {
  flex: 1;
  min-width: 0;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.cli-health-btn {
  flex-shrink: 0;
  padding: 2px 10px;
  border: 1px solid var(--border-color);
  border-radius: 4px;
  background: var(--bg-secondary);
  color: var(--text-primary);
  font-size: 12px;
  cursor: pointer;
}

.cli-health-btn:hover {
  border-color: var(--accent-gold);
}

.terminal-container {
  flex: 1;
  min-height: 0;
  overflow: hidden;
  position: relative;
  background: var(--terminal-bg);
}

/* 手机断点：终端 + 快捷键工具条纵向排布（工具条占高，终端 fit 随之收缩） */
@media (max-width: 768px) {
  .terminal-container {
    display: flex;
    flex-direction: column;
  }

  .terminal-container :deep(.xterm-container) {
    flex: 1;
    min-height: 0;
    height: auto;
  }
}

.empty-state-overlay {
  position: absolute;
  inset: 0;
  display: flex;
  align-items: center;
  justify-content: center;
  background: var(--terminal-bg);
  z-index: 10;
}

.empty-state-content {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 16px;
}

.empty-state-text {
  font-size: 14px;
  color: var(--text-secondary);
}

.empty-state-btn {
  display: flex;
  align-items: center;
  gap: 8px;
  padding: 12px 24px;
  border: 1px solid var(--accent-primary);
  background: var(--accent-primary);
  color: #fff;
  cursor: pointer;
  border-radius: var(--radius-lg);
  font-size: 14px;
  font-weight: 500;
  transition: all 0.15s ease;
  box-shadow: var(--shadow-md);
}

.empty-state-btn img {
  width: 16px;
  height: 16px;
  filter: brightness(0) invert(1);
}

.empty-state-btn:hover {
  background: var(--accent-secondary);
  transform: translateY(-1px);
  box-shadow: var(--shadow-lg);
}
</style>
