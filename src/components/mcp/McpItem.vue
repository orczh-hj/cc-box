<template>
  <div class="mcp-item" :class="{ expanded: isExpanded, disabled: isDisabled }">
    <!-- Server Header -->
    <div class="server-header" @click="handleClick">
      <span class="server-name">{{ server.displayName }}</span>
      <ToggleSwitch
        v-if="server.sourceType === 'user'"
        :modelValue="!isDisabled"
        :title="isDisabled ? t('enable') : t('disable')"
        @update:modelValue="onToggle"
      />
      <button class="expand-btn" :class="{ rotated: isExpanded }" :title="t('toggleDetails')">
        <img src="@/assets/icons/chevron.svg" alt="Toggle" />
      </button>
    </div>

    <!-- Full Name (for plugin MCP) -->
    <div v-if="server.sourceType === 'plugin'" class="server-full-name">
      {{ server.name }}
    </div>

    <!-- 开关失败反馈（乐观更新已回滚；与 SkillItem 一致） -->
    <div v-if="toggleError" class="mcp-toggle-error" :title="toggleError">{{ toggleError }}</div>

    <!-- Server Type & Status -->
    <div v-if="server.serverType || server.status" class="server-meta">
      <span v-if="server.serverType" class="server-type">{{ server.serverType }}</span>
      <span v-if="server.status" class="server-status" :class="{ connected: isConnected }">
        {{ server.status }}
      </span>
    </div>

    <!-- Detail Section -->
    <div v-if="isExpanded" class="detail-section">
      <!-- Loading -->
      <div v-if="loading" class="loading-detail">
        <span class="loading-spinner"></span>
        <span class="loading-text">{{ t('fetchingDetails') }}</span>
      </div>

      <!-- Error -->
      <div v-else-if="error" class="error-detail">
        <span class="error-text">{{ error }}</span>
        <button class="retry-btn" @click="fetchDetail(true)">{{ t('retry') }}</button>
      </div>

      <!-- Detail Content -->
      <div v-else-if="detail" class="detail-content">
        <!-- Server Info -->
        <div v-if="detail.serverInfo" class="detail-group">
          <div class="group-title">{{ t('serverInfo') }}</div>
          <div class="group-content">
            <div class="info-row">
              <span class="info-label">{{ t('mcpNameLabel') }}</span>
              <span class="info-value">{{ detail.serverInfo.name }}</span>
            </div>
            <div class="info-row">
              <span class="info-label">{{ t('mcpVersionLabel') }}</span>
              <span class="info-value">{{ detail.serverInfo.version }}</span>
            </div>
          </div>
        </div>

        <!-- Tools -->
        <div v-if="detail.tools.length > 0" class="detail-group">
          <div class="group-title">{{ t('tools', { count: detail.tools.length }) }}</div>
          <div class="item-list">
            <McpSubItem
              v-for="tool in detail.tools"
              :key="tool.name"
              :name="tool.name"
              :description="tool.description"
              :type="'tool'"
              :input-schema="tool.inputSchema"
            />
          </div>
        </div>

        <!-- Prompts -->
        <div v-if="detail.prompts.length > 0" class="detail-group">
          <div class="group-title">{{ t('prompts', { count: detail.prompts.length }) }}</div>
          <div class="item-list">
            <McpSubItem
              v-for="prompt in detail.prompts"
              :key="prompt.name"
              :name="prompt.name"
              :description="prompt.description"
              :type="'prompt'"
              :arguments="prompt.arguments"
            />
          </div>
        </div>

        <!-- Resources -->
        <div v-if="detail.resources.length > 0" class="detail-group">
          <div class="group-title">{{ t('resources', { count: detail.resources.length }) }}</div>
          <div class="item-list">
            <McpSubItem
              v-for="resource in detail.resources"
              :key="resource.uri"
              :name="resource.name"
              :description="resource.description"
              :type="'resource'"
              :uri="resource.uri"
            />
          </div>
        </div>

        <!-- No Details -->
        <div v-if="!hasDetails" class="no-details">
          <span class="no-details-text">{{ t('noToolsAvailable') }}</span>
        </div>

        <!-- Refresh Button -->
        <button class="refresh-detail-btn" @click="fetchDetail(true)" :title="t('refreshDetails')">
          <img src="@/assets/icons/refresh.svg" :alt="t('refreshDetails')" />
          {{ t('refreshDetails') }}
        </button>
      </div>

      <!-- No Details -->
      <div v-if="!hasDetails && detail" class="no-details">
        <span class="no-details-text">{{ t('noToolsAvailable') }}</span>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, inject, onUnmounted } from 'vue'
import { useI18n } from 'vue-i18n'
import type { McpServerInfo, McpServerDetail } from '@/types'
import { getMcpServerDetail } from '@/api/tauri'
import { useAppStore } from '@/stores/app'
import { useSidebarStore } from '@/stores/sidebar'
import McpSubItem from './McpSubItem.vue'
import ToggleSwitch from '@/components/common/ToggleSwitch.vue'

const { t } = useI18n()

const props = defineProps<{
  server: McpServerInfo
}>()

const appStore = useAppStore()
const sidebarStore = useSidebarStore()

const isExpanded = ref(false)
const loading = ref(false)
const error = ref<string | null>(null)
const detail = ref<McpServerDetail | null>(null)

const isDisabled = computed(() => props.server.enabled === false)

// 缓存注入（从父组件）
const detailCache = inject<Map<string, McpServerDetail>>('mcpDetailCache', new Map())

/** 开关失败提示（5s 自动消失）：此前只 console.error，用户看到的是"操作无效" */
const toggleError = ref('')
let toggleErrorTimer: ReturnType<typeof setTimeout> | undefined

async function onToggle(newValue: boolean) {
  try {
    await sidebarStore.toggleMcpServerEnabled(props.server.name, newValue)
    toggleError.value = ''
  } catch (err) {
    console.error('[McpItem] toggle failed:', err)
    toggleError.value = String(err)
    clearTimeout(toggleErrorTimer)
    toggleErrorTimer = setTimeout(() => { toggleError.value = '' }, 5000)
  }
}

onUnmounted(() => clearTimeout(toggleErrorTimer))

// 是否已连接
const isConnected = computed(() => props.server.status?.includes('Connected'))

// 是否有详情内容
const hasDetails = computed(() => {
  if (!detail.value) return false
  return detail.value.tools.length > 0 ||
         detail.value.prompts.length > 0 ||
         detail.value.resources.length > 0
})

// 点击展开/折叠
function handleClick() {
  isExpanded.value = !isExpanded.value

  if (isExpanded.value && !detail.value && !loading.value) {
    fetchDetail(false)
  }
}

// 获取详情
async function fetchDetail(force: boolean) {
  if (!appStore.cwd) return

  // 检查缓存
  if (!force && detailCache.has(props.server.name)) {
    detail.value = detailCache.get(props.server.name)!
    return
  }

  loading.value = true
  error.value = null

  try {
    const result = await getMcpServerDetail(appStore.cwd, props.server.name, force)
    detail.value = result

    // 存入缓存
    if (result) {
      detailCache.set(props.server.name, result)
    }
  } catch (err) {
    error.value = err instanceof Error ? err.message : t('fetchFailed')
    console.error('[McpItem] Failed to fetch detail:', err)
  } finally {
    loading.value = false
  }
}
</script>

<style scoped>
.mcp-item {
  background: var(--bg-primary);
  border-radius: 8px;
  padding: 10px 12px;
  transition: background 0.15s ease;
}

.mcp-item:hover {
  background: var(--hover-bg);
}

.mcp-toggle-error {
  margin-top: 4px;
  font-size: 11px;
  color: var(--danger-color, #e5484d);
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.mcp-item.disabled {
  opacity: 0.55;
}

.mcp-item.disabled .server-name {
  text-decoration: line-through;
  color: var(--text-tertiary);
}

.mcp-item.expanded {
  background: var(--bg-secondary);
}

.server-header {
  display: flex;
  align-items: center;
  gap: 8px;
  cursor: pointer;
}

.server-name {
  font-size: 13px;
  font-weight: 500;
  color: var(--text-primary);
  flex: 1;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.expand-btn {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 20px;
  height: 20px;
  border: none;
  background: transparent;
  color: var(--text-tertiary);
  cursor: pointer;
  border-radius: 4px;
  flex-shrink: 0;
  transition: transform 0.2s ease;
}

.expand-btn img {
  width: 12px;
  height: 12px;
}

.expand-btn:hover {
  color: var(--text-primary);
}

.expand-btn.rotated {
  transform: rotate(90deg);
}

.server-full-name {
  margin-top: 4px;
  font-size: 11px;
  color: var(--text-tertiary);
  font-family: var(--font-mono);
}

.server-meta {
  display: flex;
  gap: 8px;
  margin-top: 4px;
  font-size: 11px;
}

.server-type {
  color: var(--text-secondary);
  padding: 1px 4px;
  background: var(--bg-tertiary);
  border-radius: 3px;
}

.server-status {
  color: var(--text-tertiary);
}

.server-status.connected {
  color: var(--success-color);
}

/* Detail Section */
.detail-section {
  margin-top: 12px;
  padding-top: 12px;
  border-top: 1px solid var(--border-color);
}

.loading-detail {
  display: flex;
  align-items: center;
  gap: 8px;
  padding: 8px;
}

.loading-spinner {
  width: 14px;
  height: 14px;
  border: 2px solid var(--border-color);
  border-top-color: var(--accent-color);
  border-radius: 50%;
  animation: spin 1s linear infinite;
}

@keyframes spin {
  to { transform: rotate(360deg); }
}

.loading-text {
  font-size: 12px;
  color: var(--text-secondary);
}

.error-detail {
  display: flex;
  align-items: center;
  gap: 8px;
  padding: 8px;
}

.error-text {
  font-size: 12px;
  color: var(--error-color);
}

.retry-btn {
  font-size: 11px;
  padding: 2px 8px;
  border: 1px solid var(--border-color);
  background: var(--bg-tertiary);
  color: var(--text-secondary);
  cursor: pointer;
  border-radius: 4px;
}

.retry-btn:hover {
  background: var(--hover-bg);
}

/* Detail Content */
.detail-content {
  display: flex;
  flex-direction: column;
  gap: 12px;
}

.detail-group {
  display: flex;
  flex-direction: column;
  gap: 6px;
}

.group-title {
  font-size: 11px;
  font-weight: 600;
  color: var(--text-secondary);
  text-transform: uppercase;
}

.group-content {
  padding: 6px 8px;
  background: var(--bg-tertiary);
  border-radius: 6px;
}

.info-row {
  display: flex;
  gap: 8px;
  font-size: 12px;
}

.info-label {
  color: var(--text-tertiary);
  min-width: 60px;
}

.info-value {
  color: var(--text-primary);
}

.capability-list {
  display: flex;
  gap: 6px;
}

.capability-badge {
  font-size: 10px;
  padding: 2px 6px;
  background: var(--accent-bg);
  color: var(--accent-color);
  border-radius: 4px;
}

.item-list {
  display: flex;
  flex-direction: column;
  gap: 4px;
}

.no-details {
  padding: 8px;
  text-align: center;
}

.no-details-text {
  font-size: 12px;
  color: var(--text-tertiary);
}

.refresh-detail-btn {
  display: flex;
  align-items: center;
  justify-content: center;
  gap: 6px;
  width: 100%;
  padding: 6px;
  margin-top: 8px;
  border: 1px solid var(--border-color);
  background: var(--bg-tertiary);
  color: var(--text-secondary);
  cursor: pointer;
  border-radius: 6px;
  font-size: 12px;
}

.refresh-detail-btn img {
  width: 14px;
  height: 14px;
}

.refresh-detail-btn:hover {
  background: var(--hover-bg);
  color: var(--text-primary);
}
</style>