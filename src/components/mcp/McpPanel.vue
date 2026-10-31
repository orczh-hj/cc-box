<template>
  <div class="mcp-panel">
    <!-- Header -->
    <PanelHeader :title="t('mcpServers')" @close="$emit('close')">
      <template #actions>
        <button class="action-btn" @click="openEditor(null)" :title="t('mcpNew')">
          <img src="@/assets/icons/plus.svg" :alt="t('mcpNew')" />
        </button>
        <DocsHelpButton anchor="MCP" />
        <button class="action-btn" @click="handleRefresh" :title="t('refreshMcp')">
          <img src="@/assets/icons/refresh.svg" :alt="t('refreshMcp')" />
        </button>
      </template>
    </PanelHeader>

    <div class="panel-content">
      <!-- Concept Description -->
      <div class="panel-desc">{{ t('mcpDesc') }}</div>

      <!-- Loading -->
      <div v-if="loading" class="loading-state">
        <span class="loading-text">{{ t('loadingMcp') }}</span>
      </div>

      <!-- Error -->
      <div v-else-if="error" class="error-state">
        <span class="error-text">{{ error }}</span>
      </div>

      <!-- Empty -->
      <div v-else-if="allServers.length === 0" class="empty-state">
        <span class="empty-text">{{ t('noMcpConfigured') }}</span>
        <span class="empty-hint">{{ t('addMcpHint') }}</span>
      </div>

      <!-- Servers List -->
      <div v-else class="servers-list">
      <!-- Project Servers -->
      <McpGroup
        v-if="projectServers.length > 0"
        :title="t('project')"
        :expanded="sidebarStore.mcpExpandedGroups.project"
        :count="projectServers.length"
        :servers="projectServers"
        @toggle="sidebarStore.toggleMcpGroup('project')"
        @edit="openEditor"
      />

      <!-- User Servers -->
      <McpGroup
        v-if="userServers.length > 0"
        :title="t('user')"
        :expanded="sidebarStore.mcpExpandedGroups.user"
        :count="userServers.length"
        :servers="userServers"
        @toggle="sidebarStore.toggleMcpGroup('user')"
        @edit="openEditor"
      />

      <!-- Local Servers -->
      <McpGroup
        v-if="localServers.length > 0"
        :title="t('local')"
        :expanded="sidebarStore.mcpExpandedGroups.local"
        :count="localServers.length"
        :servers="localServers"
        @toggle="sidebarStore.toggleMcpGroup('local')"
        @edit="openEditor"
      />

      <!-- Plugin Servers -->
      <McpGroup
        v-if="pluginServers.length > 0"
        :title="t('plugin')"
        :expanded="sidebarStore.mcpExpandedGroups.plugin"
        :count="pluginServers.length"
        :servers="pluginServers"
        @toggle="sidebarStore.toggleMcpGroup('plugin')"
      />
    </div>
    </div>

    <McpEditor v-model:visible="editorVisible" :server="editingServer" @saved="onSaved" />
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted, provide } from 'vue'
import { useI18n } from 'vue-i18n'
import { useSidebarStore } from '@/stores/sidebar'
import { useAppStore } from '@/stores/app'

const { t } = useI18n()
import type { McpServerInfo, McpServerDetail } from '@/types'
import McpGroup from './McpGroup.vue'
import McpEditor from '../editor/McpEditor.vue'
import PanelHeader from '../sidebar/PanelHeader.vue'
import DocsHelpButton from '../sidebar/DocsHelpButton.vue'

const sidebarStore = useSidebarStore()
const appStore = useAppStore()

const error = ref<string | null>(null)

// MCP Detail 缓存（提供给子组件）
const detailCache = new Map<string, McpServerDetail>()
provide('mcpDetailCache', detailCache)

// 使用 sidebar store 的数据（已预加载）
const servers = computed(() => sidebarStore.mcpServers)
const loading = computed(() => sidebarStore.mcpServersLoading)

// 所有 servers
const allServers = computed(() => servers.value)

// 按 sourceType 分组
const pluginServers = computed(() =>
  servers.value.filter(s => s.sourceType === 'plugin')
)

const userServers = computed(() =>
  servers.value.filter(s => s.sourceType === 'user')
)

const localServers = computed(() =>
  servers.value.filter(s => s.sourceType === 'local')
)

const projectServers = computed(() =>
  servers.value.filter(s => s.sourceType === 'project')
)

// ========== 编辑器 ==========
const editorVisible = ref(false)
const editingServer = ref<McpServerInfo | null>(null)

function openEditor(server: McpServerInfo | null) {
  editingServer.value = server
  editorVisible.value = true
}

function onSaved() {
  if (appStore.cwd) sidebarStore.loadMcpServers(appStore.cwd, { silent: true })
}

function handleRefresh() {
  if (appStore.cwd) {
    error.value = null
    sidebarStore.loadMcpServers(appStore.cwd)
  }
}

onMounted(() => {
  // 如果 sidebar store 还没有数据，触发加载
  if (appStore.cwd && sidebarStore.mcpServers.length === 0) {
    sidebarStore.loadMcpServers(appStore.cwd)
  }
})
</script>

<style scoped>
.mcp-panel {
  height: 100%;
  display: flex;
  flex-direction: column;
  background: var(--bg-secondary);
}

/* action-btn slot 样式 */
.mcp-panel :deep(.action-btn) {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 24px;
  height: 24px;
  border: none;
  background: transparent;
  color: var(--text-secondary);
  cursor: pointer;
  padding: 0;
}

.mcp-panel :deep(.action-btn img) {
  width: 16px;
  height: 16px;
}

.mcp-panel :deep(.action-btn:hover) {
  color: var(--text-primary);
}

.panel-desc {
  padding: 8px 12px;
  font-size: 12px;
  line-height: 1.5;
  color: var(--text-secondary);
}

.panel-content {
  flex: 1;
  overflow-y: auto;
  display: flex;
  flex-direction: column;
}

.loading-state,
.error-state,
.empty-state {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  padding: 24px;
  gap: 8px;
}

.loading-text,
.error-text,
.empty-text {
  font-size: 13px;
  color: var(--text-secondary);
}

.error-text {
  color: var(--error-color);
}

.empty-hint {
  font-size: 12px;
  color: var(--text-tertiary);
}

.servers-list {
  display: flex;
  flex-direction: column;
  gap: 8px;
  padding: 8px 0;
}
</style>
