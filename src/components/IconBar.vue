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
    justify-content: space-between;
    padding: 6px 12px calc(6px + var(--safe-area-bottom));
    border-right: none;
    border-top: 1px solid var(--border-color);
    box-shadow: 0 -2px 8px rgba(26, 24, 22, 0.06);
    z-index: 40;
  }

  .icon-group {
    flex-direction: row;
    gap: 6px;
  }

  .icon-group.bottom {
    margin-top: 0;
  }

  /* 激活指示条：左侧竖条 → 底部横条 */
  .icon-btn.active::before {
    left: 8px;
    right: 8px;
    top: auto;
    bottom: 1px;
    width: auto;
    height: 3px;
    border-radius: 2px 2px 0 0;
  }
}
</style>