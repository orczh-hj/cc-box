<template>
  <div class="plugins-panel">
    <!-- Header -->
    <PanelHeader :title="t('plugins')" @close="$emit('close')">
      <template #actions>
        <button class="action-btn" @click="marketVisible = true" :title="t('marketOpen')">
          <img src="@/assets/icons/market.svg" :alt="t('marketOpen')" />
        </button>
        <DocsHelpButton anchor="Plugins" />
        <button class="action-btn" @click="handleRefresh" :title="t('refreshPlugins')">
          <img src="@/assets/icons/refresh.svg" :alt="t('refreshPlugins')" />
        </button>
      </template>
    </PanelHeader>

    <div class="panel-content">
      <!-- Concept Description -->
      <div class="panel-desc">{{ t('pluginsDesc') }}</div>

      <!-- Loading -->
      <div v-if="loading" class="loading-state">
        <span class="loading-text">{{ t('loadingPlugins') }}</span>
      </div>

      <!-- Error -->
      <div v-else-if="error" class="error-state">
        <span class="error-text">{{ error }}</span>
      </div>

      <!-- Empty -->
      <div v-else-if="userPlugins.length === 0 && projectPlugins.length === 0" class="empty-state">
        <span class="empty-text">{{ t('noPluginsInstalled') }}</span>
        <span class="empty-hint">{{ t('installPluginsHint') }}</span>
        <button class="market-cta" @click="marketVisible = true">{{ t('marketOpen') }}</button>
      </div>

      <!-- Plugins List -->
      <div v-else class="plugins-list">
      <!-- Project Plugins -->
      <PluginGroup
        v-if="projectPlugins.length > 0"
        :title="t('projectPlugins')"
        :expanded="sidebarStore.pluginsExpandedGroups.project"
        :count="projectPlugins.length"
        :plugins="projectPlugins"
        @toggle="sidebarStore.togglePluginGroup('project')"
      />

      <!-- User Plugins -->
      <PluginGroup
        v-if="userPlugins.length > 0"
        :title="t('userPlugins')"
        :expanded="sidebarStore.pluginsExpandedGroups.user"
        :count="userPlugins.length"
        :plugins="userPlugins"
        @toggle="sidebarStore.togglePluginGroup('user')"
      />
    </div>
    </div>

    <MarketBrowser v-model:visible="marketVisible" />
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted } from 'vue'
import { useI18n } from 'vue-i18n'
import { useSidebarStore } from '@/stores/sidebar'
import { useAppStore } from '@/stores/app'

const { t } = useI18n()
import type { PluginInfo } from '@/types'
import PluginGroup from './PluginGroup.vue'
import MarketBrowser from './MarketBrowser.vue'
import PanelHeader from '../sidebar/PanelHeader.vue'
import DocsHelpButton from '../sidebar/DocsHelpButton.vue'

const sidebarStore = useSidebarStore()
const appStore = useAppStore()

const error = ref<string | null>(null)

const marketVisible = ref(false)

// 使用 sidebar store 的数据（已预加载）
const plugins = computed(() => sidebarStore.plugins)
const loading = computed(() => sidebarStore.pluginsLoading)

// 按 scope 分组（保留 disabled 项，由 PluginItem 原位灰显 + ToggleSwitch 切换启用）
const userPlugins = computed(() => {
  return plugins.value.filter(p => p.scope === 'user')
})

const projectPlugins = computed(() => {
  return plugins.value.filter(p => p.scope === 'project')
})

function handleRefresh() {
  if (appStore.cwd) {
    error.value = null
    sidebarStore.loadPlugins(appStore.cwd)
  }
}

onMounted(() => {
  // 如果 sidebar store 还没有数据，触发加载
  if (appStore.cwd && sidebarStore.plugins.length === 0) {
    sidebarStore.loadPlugins(appStore.cwd)
  }
})
</script>

<style scoped>
.plugins-panel {
  height: 100%;
  display: flex;
  flex-direction: column;
  background: var(--bg-secondary);
}

/* action-btn slot 样式 */
.plugins-panel :deep(.action-btn) {
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

.plugins-panel :deep(.action-btn img) {
  width: 16px;
  height: 16px;
}

.plugins-panel :deep(.action-btn:hover) {
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

.market-cta {
  margin-top: 6px;
  border: 1px solid var(--border-color);
  background: var(--bg-primary);
  color: var(--text-primary);
  font-size: 12px;
  padding: 6px 14px;
  border-radius: 8px;
  cursor: pointer;
}

.market-cta:hover {
  background: var(--hover-bg);
}

.plugins-list {
  display: flex;
  flex-direction: column;
  gap: 8px;
  padding: 8px 0;
}
</style>
