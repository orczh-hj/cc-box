<template>
  <div class="hooks-panel">
    <!-- Header -->
    <PanelHeader :title="t('titleHooks')" @close="$emit('close')">
      <template #actions>
        <button class="action-btn" @click="openEditor(null)" :title="t('hookNew')">
          <img src="@/assets/icons/plus.svg" :alt="t('hookNew')" />
        </button>
        <DocsHelpButton anchor="Hooks" />
        <button class="action-btn" @click="handleRefresh" :title="t('refreshHooks')">
          <img src="@/assets/icons/refresh.svg" :alt="t('refreshHooks')" />
        </button>
      </template>
    </PanelHeader>

    <div class="panel-content">
      <div class="panel-desc">{{ t('hooksDesc') }}</div>

      <div v-if="loading" class="loading-state">
        <span class="loading-text">{{ t('loadingHooks') }}</span>
      </div>

      <div v-else-if="error" class="error-state">
        <span class="error-text">{{ error }}</span>
      </div>

      <div v-else-if="hooks.length === 0 && pluginHookRows.length === 0" class="empty-state">
        <span class="empty-text">{{ t('noHooksConfigured') }}</span>
        <span class="empty-hint">{{ t('addHooksHint') }}</span>
        <button class="empty-cta" @click="openEditor(null)">{{ t('hookNew') }}</button>
      </div>

      <div v-else class="hooks-list">
        <template v-if="hooks.length > 0">
          <HookGroup
            v-for="g in groups"
            :key="g.key"
            :title="g.title"
            :expanded="expanded[g.key] ?? true"
            :count="g.items.length"
            :items="g.items"
            @toggle="toggleGroup(g.key)"
            @edit="openEditor"
          />
        </template>

        <!-- Plugin 提供的 hooks（只读，随插件启停；点击行展开详情） -->
        <div v-if="pluginHookRows.length > 0" class="hook-group">
          <div class="group-header" @click="toggleGroup('plugin')">
            <img
              class="expand-icon"
              :class="{ expanded: expanded['plugin'] ?? true }"
              src="@/assets/icons/chevron.svg"
              alt="Toggle"
            />
            <span class="group-title">{{ t('hookGroup_plugin') }}</span>
            <span class="group-count">{{ pluginHookRows.length }}</span>
          </div>
          <div v-if="expanded['plugin'] ?? true" class="hooks-container">
            <div
              v-for="(row, i) in pluginHookRows"
              :key="`${row.plugin}:${i}`"
              class="plugin-hook-item"
              @click="togglePluginDetail(i)"
            >
              <div class="phi-info">
                <div class="phi-main">
                  <span class="ph-event">{{ row.event }}</span>
                  <span class="ph-plugin">{{ row.plugin }}</span>
                </div>
                <div v-if="row.summary && !pluginExpanded[i]" class="ph-summary">{{ row.summary }}</div>
                <div v-if="pluginExpanded[i]" class="phi-detail">
                  <div class="phi-line">
                    <span class="phi-label">plugin</span>
                    <span class="phi-value">{{ row.plugin }}</span>
                  </div>
                  <div v-if="row.matcher" class="phi-line">
                    <span class="phi-label">matcher</span>
                    <span class="phi-value">{{ row.matcher }}</span>
                  </div>
                  <div v-if="row.type" class="phi-line">
                    <span class="phi-label">type</span>
                    <span class="phi-value">{{ row.type }}</span>
                  </div>
                  <div v-if="row.timeout !== undefined" class="phi-line">
                    <span class="phi-label">timeout</span>
                    <span class="phi-value">{{ row.timeout }}</span>
                  </div>
                  <div v-if="row.summary" class="phi-line">
                    <span class="phi-label">cmd</span>
                    <span class="phi-value wrap">{{ row.summary }}</span>
                  </div>
                </div>
              </div>
            </div>
          </div>
        </div>
      </div>
    </div>

    <HookEditor
      v-model:visible="editorVisible"
      :hook="editingHook"
      @saved="onSaved"
    />
  </div>
</template>

<script setup lang="ts">
import { computed, onMounted, ref } from 'vue'
import { useI18n } from 'vue-i18n'
import { useSidebarStore } from '@/stores/sidebar'
import { useAppStore } from '@/stores/app'
import type { HookListItem } from '@/api/tauri'
import HookGroup from './HookGroup.vue'
import HookEditor from '../editor/HookEditor.vue'
import PanelHeader from '../sidebar/PanelHeader.vue'
import DocsHelpButton from '../sidebar/DocsHelpButton.vue'

const { t } = useI18n()
const sidebarStore = useSidebarStore()
const appStore = useAppStore()

const error = ref<string | null>(null)

const hooks = computed(() => sidebarStore.hooks)
const loading = computed(() => sidebarStore.hooksLoading)
const expanded = computed(() => sidebarStore.hooksExpandedGroups)

interface Group {
  key: 'user' | 'project' | 'local' | 'disabled'
  title: string
  items: HookListItem[]
}

const groups = computed<Group[]>(() => {
  const by: Record<string, HookListItem[]> = { user: [], project: [], local: [], disabled: [] }
  for (const h of hooks.value) {
    const type = h.enabled ? h.source.type : 'disabled'
    ;(by[type] ??= []).push(h)
  }
  const out: Group[] = []
  for (const key of ['user', 'project', 'local', 'disabled'] as const) {
    if (by[key].length) {
      out.push({ key, title: t(`hookGroup_${key}`), items: by[key] })
    }
  }
  return out
})

function toggleGroup(key: string) {
  sidebarStore.toggleHookGroup(key as keyof typeof sidebarStore.hooksExpandedGroups)
}

// Plugin 提供的 hooks（来自已安装插件，只读展示 + 点击展开详情）
interface PluginHookRow {
  plugin: string
  event: string
  matcher?: string
  type?: string
  summary?: string
  timeout?: unknown
}

const pluginHookRows = computed<PluginHookRow[]>(() => {
  const rows: PluginHookRow[] = []
  for (const p of sidebarStore.plugins) {
    for (const h of p.hooks ?? []) {
      rows.push({
        plugin: p.name,
        event: h.event,
        matcher: h.matcher,
        type: h.type,
        summary: h.summary,
        timeout: h.timeout,
      })
    }
  }
  // 排序：先 plugin 名称，再 hook 类型（command/http/prompt/agent），同类型内按事件名稳定排序
  return rows.sort(
    (a, b) =>
      a.plugin.localeCompare(b.plugin) ||
      (a.type ?? '').localeCompare(b.type ?? '') ||
      a.event.localeCompare(b.event)
  )
})

const pluginExpanded = ref<Record<number, boolean>>({})

function togglePluginDetail(i: number) {
  pluginExpanded.value[i] = !pluginExpanded.value[i]
}

// ========== 编辑器 ==========
const editorVisible = ref(false)
const editingHook = ref<HookListItem | null>(null)

function openEditor(hook: HookListItem | null) {
  editingHook.value = hook
  editorVisible.value = true
}

function onSaved() {
  // 刷新由 state-changed(hooks) 统一驱动（写命令出口自动广播）；
  // 这里不再手动重拉，避免与广播叠加成双刷新
}

function handleRefresh() {
  if (appStore.cwd) {
    error.value = null
    sidebarStore.loadHooks(appStore.cwd)
  }
}

onMounted(() => {
  if (appStore.cwd && sidebarStore.hooks.length === 0) {
    sidebarStore.loadHooks(appStore.cwd)
  }
  // 插件 hooks 分组依赖已安装插件列表
  if (appStore.cwd && sidebarStore.plugins.length === 0) {
    sidebarStore.loadPlugins(appStore.cwd)
  }
})
</script>

<style scoped>
.hooks-panel {
  height: 100%;
  display: flex;
  flex-direction: column;
  background: var(--bg-secondary);
}

.hooks-panel :deep(.action-btn) {
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

.hooks-panel :deep(.action-btn img) {
  width: 16px;
  height: 16px;
}

.hooks-panel :deep(.action-btn:hover) {
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
  color: var(--danger-color, #e5484d);
}

.empty-hint {
  font-size: 12px;
  color: var(--text-tertiary);
}

.empty-cta {
  margin-top: 6px;
  padding: 6px 16px;
  border: 1px solid var(--border-color);
  border-radius: 8px;
  background: var(--bg-primary);
  color: var(--text-primary);
  font-size: 12px;
  cursor: pointer;
}

.empty-cta:hover {
  background: var(--hover-bg);
}

.hooks-list {
  display: flex;
  flex-direction: column;
  gap: 8px;
  padding: 8px 0;
}

/* 插件 hooks 分组：结构与 HookGroup 完全一致（头部 + 指示器 + 容器内边距/间距） */
.hook-group {
  display: flex;
  flex-direction: column;
}

.expand-icon {
  width: 14px;
  height: 14px;
  color: var(--text-secondary);
  flex-shrink: 0;
  transition: transform 0.2s ease;
}

.expand-icon.expanded {
  transform: rotate(90deg);
}

.group-header {
  display: flex;
  align-items: center;
  gap: 8px;
  padding: 8px 16px;
  cursor: pointer;
  user-select: none;
}

.group-header:hover {
  background: var(--hover-bg);
}

.group-title {
  font-size: 13px;
  font-weight: 500;
  color: var(--text-primary);
}

.group-count {
  font-size: 11px;
  color: var(--text-tertiary);
  background: var(--bg-tertiary);
  padding: 2px 6px;
  border-radius: 10px;
}

.hooks-container {
  display: flex;
  flex-direction: column;
  gap: 4px;
  padding: 4px 12px 8px 12px;
}

/* 行：与 HookItem 同视觉（bg-primary 卡片 + 事件 mono + 徽标），点击展开只读详情 */
.plugin-hook-item {
  background: var(--bg-primary);
  border-radius: 8px;
  padding: 10px 12px;
  cursor: pointer;
  user-select: none;
  display: flex;
  align-items: center;
  gap: 10px;
  transition: background 0.15s ease;
  min-width: 0;
}

.plugin-hook-item:hover {
  background: var(--hover-bg);
}

.phi-info {
  flex: 1;
  min-width: 0;
}

.phi-main {
  display: flex;
  align-items: center;
  gap: 8px;
  min-width: 0;
}

.ph-plugin {
  font-size: 11px;
  color: var(--text-secondary);
  background: var(--bg-tertiary);
  padding: 1px 6px;
  border-radius: 4px;
  font-family: var(--font-mono);
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
  flex-shrink: 0;
  max-width: 120px;
}

.ph-event {
  font-size: 13px;
  font-weight: 500;
  color: var(--text-primary);
  font-family: var(--font-mono);
}

/* 折叠态第二行：hook 内容摘要，与 HookItem 的 hook-command 同视觉（末尾省略，展开后由完整详情替代） */
.ph-summary {
  margin-top: 4px;
  font-size: 11px;
  color: var(--text-tertiary);
  font-family: var(--font-mono);
  display: block;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.phi-detail {
  margin-top: 8px;
  padding-top: 6px;
  border-top: 1px solid var(--border-color);
  display: flex;
  flex-direction: column;
  gap: 4px;
}

.phi-line {
  display: flex;
  gap: 8px;
  font-size: 11px;
  min-width: 0;
}

.phi-label {
  color: var(--text-tertiary);
  flex-shrink: 0;
  width: 52px;
}

.phi-value {
  font-family: var(--font-mono);
  color: var(--text-secondary);
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.phi-value.wrap {
  white-space: pre-wrap;
  word-break: break-all;
}
</style>
