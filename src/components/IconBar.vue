<template>
  <aside class="icon-bar">
    <!-- 顶部图标组 -->
    <div class="icon-group top">
      <!-- Sessions -->
      <button
        class="icon-btn"
        :class="{ active: activePanel === 'sessions' }"
        @click="$emit('toggle', 'sessions')"
        :title="t('titleSessions')"
      >
        <img src="@/assets/icons/sessions.svg" alt="Sessions" />
        <span v-if="showPendingBadge" class="icon-badge pending-badge"></span>
      </button>

      <!-- Skills -->
      <button
        class="icon-btn"
        :class="{ active: activePanel === 'skills' }"
        @click="$emit('toggle', 'skills')"
        :title="t('titleSkills')"
      >
        <img src="@/assets/icons/skills.svg" alt="Skills" />
      </button>

      <!-- Agents -->
      <button
        class="icon-btn"
        :class="{ active: activePanel === 'agents' }"
        @click="$emit('toggle', 'agents')"
        :title="t('titleAgents')"
      >
        <img src="@/assets/icons/agents.svg" alt="Agents" />
      </button>

      <!-- MCP -->
      <button
          class="icon-btn"
          :class="{ active: activePanel === 'mcp' }"
          @click="$emit('toggle', 'mcp')"
          :title="t('titleMcp')"
      >
        <img src="@/assets/icons/mcp.svg" alt="MCP" />
      </button>

      <!-- Hooks -->
      <button
        class="icon-btn"
        :class="{ active: activePanel === 'hooks' }"
        @click="$emit('toggle', 'hooks')"
        :title="t('titleHooks')"
      >
        <img src="@/assets/icons/hooks.svg" alt="Hooks" />
      </button>

      <!-- Plugins -->
      <button
        class="icon-btn"
        :class="{ active: activePanel === 'plugins' }"
        @click="$emit('toggle', 'plugins')"
        :title="t('titlePlugins')"
      >
        <img src="@/assets/icons/plugins.svg" alt="Plugins" />
      </button>

    </div>

    <!-- 底部按钮组 -->
    <div class="icon-group bottom">
      <!-- 远程访问（手机扫码）——浏览器模式隐藏（已在远程端，二维码给不出） -->
      <button
        v-if="isTauriEnv"
        class="icon-btn remote-btn"
        @click="$emit('openRemote')"
        :title="t('titleRemote')"
      >
        <img src="@/assets/icons/phone.svg" alt="Remote" />
      </button>

      <!-- 文件传输——仅浏览器远程模式（桌面有系统文件管理器；与设置同组） -->
      <button
        v-if="!isTauriEnv"
        class="icon-btn"
        :class="{ active: activePanel === 'files' }"
        @click="$emit('toggle', 'files')"
        :title="t('titleFiles')"
      >
        <img src="@/assets/icons/files.svg" alt="Files" />
      </button>

      <!-- Settings -->
      <button
        class="icon-btn settings-btn"
        :class="{ active: sidebarStore.showSettings }"
        @click="handleSettingsClick"
        :title="t('titleSettings', { key: ctrl + '+,' })"
      >
        <img src="@/assets/icons/settings.svg" alt="Settings" />
        <span v-if="sidebarStore.updateAvailable" class="icon-badge update-badge"></span>
      </button>

      <!-- 文件夹按钮——浏览器模式隐藏（系统级操作不可用） -->
      <button
        v-if="isTauriEnv"
        class="icon-btn folder-btn"
        @click="$emit('openFolder')"
        :title="t('titleOpenFolder')"
      >
        <img src="@/assets/icons/folder.svg" alt="Folder" />
      </button>
    </div>
  </aside>
</template>

<script setup lang="ts">
import { computed } from 'vue'
import { useI18n } from 'vue-i18n'
import type { SidebarPanelType } from '@/stores/sidebar'
import { useSidebarStore } from '@/stores/sidebar'
import { useSessionStore } from '@/stores/session'
import { useAppStore } from '@/stores/app'
import { ctrl } from '@/utils/platform'
import { isTauriEnv } from '@/api/platform'

const { t } = useI18n()

defineProps<{
  activePanel: SidebarPanelType
}>()

defineEmits<{
  toggle: [panel: SidebarPanelType]
  toggleSettings: []
  openRemote: []
  openFolder: []
}>()

const sidebarStore = useSidebarStore()
const sessionStore = useSessionStore()
const appStore = useAppStore()

const showPendingBadge = computed(() => {
  if (sidebarStore.activePanel === 'sessions' && sidebarStore.panelVisible) return false
  const cwd = appStore.cwd
  if (!cwd) return false
  for (const tab of sessionStore.getProjectTabs(cwd)) {
    if (tab.tabId === sessionStore.activeTabId) continue
    if (tab.pending) return true
  }
  return false
})

function handleSettingsClick() {
  if (sidebarStore.updateAvailable) {
    sidebarStore.openSettings('update')
  } else {
    sidebarStore.toggleSettings()
  }
}
</script>

<style scoped>
.icon-bar {
  width: 48px;
  background: var(--bg-secondary);
  border-right: 1px solid var(--border-color);
  display: flex;
  flex-direction: column;
  padding: 8px 4px;
  gap: 4px;
  flex-shrink: 0;
}

.icon-group {
  display: flex;
  flex-direction: column;
  gap: 4px;
}

.icon-group.bottom {
  margin-top: auto;
}

.icon-btn {
  position: relative;
  width: 40px;
  height: 40px;
  margin: 0 auto;
  display: flex;
  align-items: center;
  justify-content: center;
  border: none;
  background: transparent;
  color: var(--text-secondary);
  cursor: pointer;
  border-radius: var(--radius-md);
  transition: all 0.15s ease;
}

.icon-btn img {
  width: 20px;
  height: 20px;
  flex-shrink: 0;
  opacity: 0.85;
  transition: opacity 0.15s ease;
}

.icon-btn:hover {
  background: var(--hover-bg);
  color: var(--text-primary);
}

.icon-btn:hover img {
  opacity: 1;
}

.icon-btn.active {
  background: var(--selected-bg);
  color: var(--accent-gold);
}

.icon-btn.active img {
  opacity: 1;
}

.icon-btn.active::before {
  content: '';
  position: absolute;
  left: -4px;
  top: 8px;
  bottom: 8px;
  width: 3px;
  background: var(--accent-gold);
  border-radius: 0 2px 2px 0;
}

.settings-btn {
  position: relative;
}

.icon-badge {
  position: absolute;
  top: 6px;
  right: 6px;
  width: 8px;
  height: 8px;
  border-radius: 50%;
  border: 2px solid var(--bg-secondary);
  animation: pulse 2s ease-in-out infinite;
}

.update-badge {
  background: var(--status-error);
}

.pending-badge {
  background: var(--accent-gold);
}

@keyframes pulse {
  0%, 100% { opacity: 1; }
  50% { opacity: 0.6; }
}

/* ============================================
 * 手机断点（≤768px 且触屏）：左竖栏 → 底部横栏
 * 纯样式分支（结构不变），与 useIsMobile 断点一致
 * ============================================ */
@media (max-width: 768px) and (hover: none) {
  .icon-bar {
    position: fixed;
    left: 0;
    right: 0;
    bottom: 0;
    width: 100%;
    flex-direction: row;
    align-items: center;
    justify-content: flex-start;
    gap: 4px;
    padding: 6px 8px calc(6px + var(--safe-area-bottom));
    border-right: none;
    border-top: 1px solid var(--border-color);
    box-shadow: 0 -2px 8px rgba(26, 24, 22, 0.06);
    z-index: 40;
  }

  .icon-group {
    flex-direction: row;
    gap: 4px;
  }

  /* 左侧面板组：空间不足时横向滚动（隐藏滚动条），不挤压右侧 */
  .icon-group.top {
    flex: 1 1 auto;
    min-width: 0;
    flex-wrap: nowrap;
    overflow-x: auto;
    overflow-y: hidden;
    scrollbar-width: none;
    -webkit-overflow-scrolling: touch;
    overscroll-behavior-x: contain;
  }

  .icon-group.top::-webkit-scrollbar {
    display: none;
  }

  .icon-group.top .icon-btn {
    flex: 0 0 auto;
  }

  /* 右侧 Files/Settings：固定宽度，不参与收缩 */
  .icon-group.bottom {
    flex: 0 0 auto;
    margin-top: 0;
    margin-left: auto;
  }

  /* 手机端收窄按钮（高度保持 40px 触控目标），减少横向占用 */
  .icon-btn {
    width: 36px;
    height: 40px;
    margin: 0;
  }

  /* 激活指示条：左侧竖条 → 底部横条 */
  .icon-btn.active::before {
    left: 6px;
    right: 6px;
    top: auto;
    bottom: 1px;
    width: auto;
    height: 3px;
    border-radius: 2px 2px 0 0;
  }
}
</style>