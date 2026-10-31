<template>
  <div class="agents-panel">
    <!-- Header -->
    <PanelHeader :title="t('agents')" @close="$emit('close')">
      <template #actions>
        <button class="action-btn" @click="openEditor(null)" :title="t('agentNew')">
          <img src="@/assets/icons/plus.svg" :alt="t('agentNew')" />
        </button>
        <DocsHelpButton anchor="Subagents" />
        <button class="action-btn" @click="handleRefresh" :title="t('refreshAgents')">
          <img src="@/assets/icons/refresh.svg" :alt="t('refreshAgents')" />
        </button>
      </template>
    </PanelHeader>

    <div class="panel-content">
      <!-- Concept Description -->
      <div class="panel-desc">{{ t('agentsDesc') }}</div>

      <!-- Loading -->
      <div v-if="loading" class="loading-state">
        <span class="loading-text">{{ t('loadingAgents') }}</span>
      </div>

      <!-- Error -->
      <div v-else-if="error" class="error-state">
        <span class="error-text">{{ error }}</span>
      </div>

      <!-- Empty -->
      <div v-else-if="allAgents.length === 0" class="empty-state">
        <span class="empty-text">{{ t('noAgentsAvailable') }}</span>
      </div>

      <!-- Agents List -->
      <div v-else class="agents-list">
      <!-- Project Agents -->
      <AgentGroup
        v-if="projectAgents.length > 0"
        :title="t('project')"
        :expanded="sidebarStore.agentsExpandedGroups.project"
        :count="projectAgents.length"
        :agents="projectAgents"
        @toggle="sidebarStore.toggleAgentGroup('project')"
        @edit="openEditor"
      />

      <!-- User Agents -->
      <AgentGroup
        v-if="userAgents.length > 0"
        :title="t('user')"
        :expanded="sidebarStore.agentsExpandedGroups.user"
        :count="userAgents.length"
        :agents="userAgents"
        @toggle="sidebarStore.toggleAgentGroup('user')"
        @edit="openEditor"
      />

      <!-- Plugin Agents -->
      <AgentGroup
        v-if="pluginAgents.length > 0"
        :title="t('plugin')"
        :expanded="sidebarStore.agentsExpandedGroups.plugin"
        :count="pluginAgents.length"
        :agents="pluginAgents"
        @toggle="sidebarStore.toggleAgentGroup('plugin')"
      />

      <!-- Built-in Agents -->
      <AgentGroup
        v-if="builtinAgents.length > 0"
        :title="t('builtin')"
        :expanded="sidebarStore.agentsExpandedGroups.builtin"
        :count="builtinAgents.length"
        :agents="builtinAgents"
        @toggle="sidebarStore.toggleAgentGroup('builtin')"
      />
    </div>
    </div>

    <AgentEditor v-model:visible="editorVisible" :agent="editingAgent" @saved="onSaved" />
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted } from 'vue'
import { useI18n } from 'vue-i18n'
import { useSidebarStore } from '@/stores/sidebar'
import { useAppStore } from '@/stores/app'

const { t } = useI18n()
import type { AgentInfo } from '@/types'
import AgentGroup from './AgentGroup.vue'
import AgentEditor from '../editor/AgentEditor.vue'
import PanelHeader from '../sidebar/PanelHeader.vue'
import DocsHelpButton from '../sidebar/DocsHelpButton.vue'

const sidebarStore = useSidebarStore()
const appStore = useAppStore()

const error = ref<string | null>(null)

// 使用 sidebar store 的数据（已预加载）
const agents = computed(() => sidebarStore.agents)
const loading = computed(() => sidebarStore.agentsLoading)

// 所有 agents
const allAgents = computed(() => agents.value)

// 按 sourceType 分组
const builtinAgents = computed(() =>
  agents.value.filter(a => a.sourceType === 'builtin')
)

const pluginAgents = computed(() =>
  agents.value.filter(a => a.sourceType === 'plugin')
)

const userAgents = computed(() =>
  agents.value.filter(a => a.sourceType === 'user')
)

const projectAgents = computed(() =>
  agents.value.filter(a => a.sourceType === 'project')
)

// ========== 编辑器 ==========
const editorVisible = ref(false)
const editingAgent = ref<AgentInfo | null>(null)

function openEditor(agent: AgentInfo | null) {
  editingAgent.value = agent
  editorVisible.value = true
}

function onSaved() {
  if (appStore.cwd) sidebarStore.loadAgents(appStore.cwd, { silent: true })
}

function handleRefresh() {
  if (appStore.cwd) {
    error.value = null
    sidebarStore.loadAgents(appStore.cwd)
  }
}

onMounted(() => {
  // 如果 sidebar store 还没有数据，触发加载
  if (appStore.cwd && sidebarStore.agents.length === 0) {
    sidebarStore.loadAgents(appStore.cwd)
  }
})
</script>

<style scoped>
.agents-panel {
  height: 100%;
  display: flex;
  flex-direction: column;
  background: var(--bg-secondary);
}

/* action-btn slot 样式 */
.agents-panel :deep(.action-btn) {
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

.agents-panel :deep(.action-btn img) {
  width: 16px;
  height: 16px;
}

.agents-panel :deep(.action-btn:hover) {
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

.agents-list {
  display: flex;
  flex-direction: column;
  gap: 8px;
  padding: 8px 0;
}
</style>
