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
        :project-name="appStore.currentProject || t('claudeCode')"
        @back="handleBack"
      />
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
        <!-- 快捷键工具条（手机断点）：软键盘按不出的控制键（模式切换/换行等） -->
        <QuickKeyBar v-if="isMobile" @send="handleQuickKey" />
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
import { openInFileManager, logMessage, onRemoteSessionCreated } from '@/api/tauri'
import { sendTerminalCommand } from '@/composables/useTerminalCommand'
import { useWindowAttention } from '@/composables/useWindowAttention'
import { useStatusMonitor } from '@/composables/useStatusMonitor'
import { getCurrentWindow } from '@tauri-apps/api/window'
import { isTauriEnv } from '@/api/platform'
import TerminalHeader from './TerminalHeader.vue'
import XTermTerminal from './XTermTerminal.vue'
import IconBar from './IconBar.vue'
import QuickKeyBar from './QuickKeyBar.vue'
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
const sessionStore = useSessionStore()
const sidebarStore = useSidebarStore()
const configStore = useConfigStore()
const terminalRef = ref()

const isMobile = useIsMobile()

/** 快捷键工具条：直发控制序列到当前显示终端（sendText 含重新聚焦） */
function handleQuickKey(bytes: string) {
  terminalRef.value?.sendText(bytes)
}

const { t } = useI18n()

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
  const tabId = sessionStore.createTab(projectPath, { sessionId, name: sessionName })
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
  // data 为 JSON：{cwd, resume}（resume 为空串表示新建；JSON 避免 cwd 含 \t 时截断）
  unlistenRemoteCreated = await onRemoteSessionCreated(({ id, data }) => {
    if (!data) return
    let cwd = ''
    let resume = ''
    try {
      const parsed = JSON.parse(data)
      cwd = parsed.cwd ?? ''
      resume = parsed.resume ?? ''
    } catch {
      return
    }
    if (!cwd) return
    const tabId = sessionStore.createTab(cwd, resume ? { sessionId: resume } : undefined)
    sessionStore.setTabPty(tabId, id)
    sessionStore.setActiveTab(tabId)
    void nextTick(() => terminalRef.value?.startTab(tabId))
  })

  const cwd = appStore.cwd
  if (!cwd) return

  updateWindowTitle(cwd)

  try {
    await sessionStore.loadHistorySessions(cwd)
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

// 重命名会话
function handleRenameSession(tabId: string, name: string) {
  const tab = sessionStore.tabs.get(tabId)
  if (tab?.status === 'running' && tab.ptyId) {
    sendTerminalCommand(`/rename ${name}\r`)
  }
  sessionStore.updateTabName(tabId, name)
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

// 恢复历史会话（在新 Tab 中 --resume）
function handleResumeSession(sessionId: string) {
  const cwd = appStore.cwd
  if (!cwd || !terminalRef.value) return

  // 查找历史会话的名称
  const historySession = sessionStore.historySessions.find(s => s.sessionId === sessionId)
  const name = historySession?.name

  const tabId = sessionStore.createTab(cwd, { sessionId, name })
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
