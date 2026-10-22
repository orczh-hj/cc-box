<template>
  <!-- 手机断点抽屉遮罩（桌面 CSS 下 display:none，点击关闭） -->
  <div v-if="visible" class="sidebar-mask" @click="$emit('close')"></div>
  <aside class="sidebar-panel" :class="{ visible }">
    <div class="panel-inner">
      <!-- Sessions 面板 -->
      <SessionsPanel
        v-show="activePanel === 'sessions'"
        @close="$emit('close')"
        @switch-session="$emit('switchSession', $event)"
        @rename-session="(id, name) => $emit('renameSession', id, name)"
        @restart-session="$emit('restartSession')"
        @new-session="$emit('newSession')"
        @resume-session="$emit('resumeSession', $event)"
        @close-tab="$emit('closeTab', $event)"
        @close-all-tabs="$emit('closeAllTabs')"
        @close-other-tabs="$emit('closeOtherTabs')"
      />

      <!-- Skills 面板 -->
      <SkillsPanel
        v-show="activePanel === 'skills'"
        @close="$emit('close')"
      />

      <!-- Agents 面板 -->
      <AgentsPanel
        v-show="activePanel === 'agents'"
        @close="$emit('close')"
      />

      <!-- MCP 面板 -->
      <McpPanel
        v-show="activePanel === 'mcp'"
        @close="$emit('close')"
      />

      <!-- Plugins 面板 -->
      <PluginsPanel
        v-show="activePanel === 'plugins'"
        @close="$emit('close')"
      />

      <!-- 文件面板（浏览器远程模式专属；桌面模式 activePanel 不会切到 'files'） -->
      <FilesPanel
        v-show="activePanel === 'files'"
        @close="$emit('close')"
      />
    </div>
  </aside>
</template>

<script setup lang="ts">
import type { SidebarPanelType } from '@/stores/sidebar'
import SessionsPanel from '../sessions/SessionsPanel.vue'
import SkillsPanel from '../skills/SkillsPanel.vue'
import AgentsPanel from '../agents/AgentsPanel.vue'
import McpPanel from '../mcp/McpPanel.vue'
import PluginsPanel from '../plugins/PluginsPanel.vue'
import FilesPanel from '../files/FilesPanel.vue'

defineProps<{
  visible: boolean
  activePanel: SidebarPanelType
}>()

defineEmits<{
  close: []
  switchSession: [tabId: string]
  renameSession: [tabId: string, name: string]
  restartSession: []
  newSession: []
  resumeSession: [sessionId: string]
  closeTab: [tabId: string]
  closeAllTabs: []
  closeOtherTabs: []
}>()
</script>

<style scoped>
/* 桌面：flex 兄弟，宽度过渡展开 */
.sidebar-panel {
  width: 0;
  background: var(--bg-secondary);
  border-right: 1px solid var(--border-color);
  transition: width 0.25s ease;
  overflow: hidden;
  display: flex;
  flex-direction: column;
  flex-shrink: 0;
}

.sidebar-panel.visible {
  width: 280px;
}

.panel-inner {
  width: 280px;
  flex: 1;
  display: flex;
  flex-direction: column;
  min-height: 0;
}

/* 手机断点遮罩（桌面恒隐藏） */
.sidebar-mask {
  display: none;
}

/* ============================================
 * 手机断点（≤768px）：flex 兄弟 → fixed 左侧抽屉 + 遮罩
 * （transform 动画替代 width 过渡，避免 flex 重排）
 * ============================================ */
@media (max-width: 768px) {
  .sidebar-mask {
    display: block;
    position: fixed;
    inset: 0;
    background: rgba(0, 0, 0, 0.4);
    z-index: 45;
  }

  .sidebar-panel {
    position: fixed;
    top: var(--titlebar-h, 0px);
    bottom: 0;
    left: 0;
    width: min(85vw, 340px);
    transform: translateX(-100%);
    transition: transform 0.25s ease;
    border-right: 1px solid var(--border-color);
    box-shadow: var(--shadow-lg);
    z-index: 46;
    overflow: hidden;
  }

  .sidebar-panel.visible {
    width: min(85vw, 340px);
    transform: translateX(0);
  }

  .panel-inner {
    width: 100%;
  }
}
</style>
