<template>
  <div class="sessions-panel">
    <!-- Header -->
    <PanelHeader :title="t('sessions')" @close="$emit('close')">
      <template #actions>
        <button class="action-btn" @click="handleRefresh" :title="t('refreshSessions')">
          <img src="@/assets/icons/refresh.svg" :alt="t('refreshSessions')" />
        </button>
      </template>
    </PanelHeader>

    <!-- 搜索框 -->
    <div class="search-box">
      <img class="search-icon" src="@/assets/icons/search.svg" alt="Search" />
      <input
        v-model="searchQuery"
        type="text"
        :placeholder="t('searchSessions')"
        class="search-input"
      />
      <button
        v-if="searchQuery"
        class="search-clear-btn"
        @click="searchQuery = ''"
        :title="t('clearSearch')"
      >
        <img src="@/assets/icons/close.svg" :alt="t('clearSearch')" />
      </button>
    </div>

    <!-- 会话列表 -->
    <div class="panel-content" ref="scrollContainer">
      <!-- 搜索结果模式 -->
      <template v-if="searchQuery.trim()">
        <div class="section">
          <div class="section-title">{{ t('searchResults') }}</div>

          <!-- 来自 Open Tabs 的命中 -->
          <div v-if="matchedTabs.length > 0" class="subsection">
            <div class="subsection-title">{{ t('openTabs') }} · {{ matchedTabs.length }}</div>
            <SessionList
              :tabs="matchedTabs"
              :active-id="sessionStore.activeTabId"
              closable
              @switch="(id) => $emit('switchSession', id)"
              @rename="(id, name) => $emit('renameSession', id, name)"
              @restart="() => $emit('restartSession')"
              @close="(id) => $emit('closeTab', id)"
            />
          </div>

          <!-- 来自 History 的命中 -->
          <div v-if="filteredHistory.length > 0" class="subsection">
            <div class="subsection-title">{{ t('history') }} · {{ filteredHistory.length }}</div>
            <SessionList
              :history="filteredHistory"
              :active-id="null"
              :snippet-map="snippetMap"
              @switch="(id) => $emit('resumeSession', id)"
            />
          </div>

          <!-- 空状态 -->
          <div v-if="matchedTabs.length === 0 && filteredHistory.length === 0 && !sessionStore.isLoading" class="empty-hint">
            {{ t('noSessionsFound') }}
          </div>

          <div v-if="sessionStore.isLoading" class="loading-indicator">
            {{ t('loading') }}
          </div>
        </div>
      </template>

      <!-- 正常模式 -->
      <template v-else>
        <!-- Open Tabs -->
        <div v-if="projectTabs.length > 0" class="section">
          <div class="section-title-row">
            <span class="section-title">{{ t('openTabs') }}</span>
            <div v-if="projectTabs.length > 1" class="section-actions">
              <button class="section-action-btn" @click="$emit('closeOtherTabs')" :title="t('closeOtherTabs')">
                {{ t('closeOtherTabs') }}
              </button>
              <button class="section-action-btn" @click="$emit('closeAllTabs')" :title="t('closeAllTabs')">
                {{ t('closeAllTabs') }}
              </button>
            </div>
          </div>
          <SessionList
            :tabs="projectTabs"
            :active-id="sessionStore.activeTabId"
            closable
            @switch="(id) => $emit('switchSession', id)"
            @rename="(id, name) => $emit('renameSession', id, name)"
            @restart="() => $emit('restartSession')"
            @close="(id) => $emit('closeTab', id)"
          />
        </div>

        <!-- History -->
        <div v-if="filteredHistory.length > 0 || sessionStore.isLoading" class="section">
          <div class="section-title">{{ t('history') }}</div>
          <SessionList
            :history="filteredHistory"
            :active-id="null"
            :snippet-map="snippetMap"
            @switch="(id) => $emit('resumeSession', id)"
          />
        </div>

        <!-- 空状态 -->
        <div v-if="projectTabs.length === 0 && filteredHistory.length === 0 && !sessionStore.isLoading" class="empty-hint">
          {{ t('noSessionsFound') }}
        </div>

        <div v-if="sessionStore.isLoading" class="loading-indicator">
          {{ t('loading') }}
        </div>

        <div v-if="sessionStore.isLoadingMore" class="loading-indicator">
          {{ t('loadingMore') }}
        </div>
      </template>
    </div>

    <!-- 底部操作区 -->
    <footer class="panel-footer">
      <div class="action-buttons">
        <button
          class="action-btn with-hint"
          @click="$emit('newSession')"
          :title="t('newSessionTitle')"
        >
          <div class="btn-content">
            <img src="@/assets/icons/plus.svg" :alt="t('newBtn')" />
            <span class="btn-label">{{ t('newBtn') }}</span>
          </div>
          <span class="btn-hint">{{ alt }}+N</span>
        </button>
        <button
          class="action-btn with-hint"
          @click="$emit('restartSession')"
          :disabled="!sessionStore.activeTab"
          :title="t('restartSessionTitle')"
        >
          <div class="btn-content">
            <img src="@/assets/icons/refresh.svg" :alt="t('restartBtn')" />
            <span class="btn-label">{{ t('restartBtn') }}</span>
          </div>
          <span class="btn-hint">{{ alt }}+R</span>
        </button>
      </div>

      <div class="options-content">
        <label class="option-item">
          <input type="checkbox" v-model="appStore.claudeOptions.skipPermissions" />
          <span class="option-label">{{ t('allow') }}</span>
          <code class="option-flag warning">skip-permissions</code>
        </label>

        <div class="option-item text-option">
          <span class="option-label">{{ t('customArgs') }}</span>
          <input type="text" v-model="appStore.claudeOptions.customArgs" :placeholder="t('customArgsPlaceholder')" />
        </div>
      </div>
    </footer>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted, onUnmounted, watch } from 'vue'
import { useI18n } from 'vue-i18n'
import { useSessionStore } from '@/stores/session'
import { useAppStore } from '@/stores/app'

const { t } = useI18n()
import { alt } from '@/utils/platform'
import SessionList from './SessionList.vue'
import PanelHeader from '../sidebar/PanelHeader.vue'

const emit = defineEmits<{
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

const sessionStore = useSessionStore()
const appStore = useAppStore()
const scrollContainer = ref<HTMLElement>()
const searchQuery = ref('')

const projectTabs = computed(() => {
  const cwd = appStore.cwd
  if (!cwd) return []
  return sessionStore.getProjectTabs(cwd)
})

const matchedTabs = computed(() => {
  const q = searchQuery.value.toLowerCase().trim()
  if (!q) return []
  return projectTabs.value.filter(t =>
    t.name.toLowerCase().includes(q)
    || (t.sessionId?.toLowerCase().includes(q) ?? false)
    || t.tabId.toLowerCase().includes(q)
  )
})

const filteredHistory = computed(() => {
  const query = searchQuery.value.toLowerCase()
  if (!query) return sessionStore.historySessions

  const byName = sessionStore.historySessions.filter(s =>
    s.name.toLowerCase().includes(query)
  )

  const msgResults = sessionStore.messageSearchResults
  const byNameIds = new Set(byName.map(s => s.sessionId))

  return [
    ...byName,
    ...msgResults
      .filter(r => !byNameIds.has(r.sessionId))
      .map(r => ({
        sessionId: r.sessionId,
        name: r.name,
        projectPath: r.projectPath,
        lastActiveAt: r.lastActiveAt,
      }))
  ]
})

const snippetMap = computed(() => {
  const map = new Map<string, string>()
  for (const r of sessionStore.messageSearchResults) {
    map.set(r.sessionId, r.snippet)
  }
  return map
})

watch(searchQuery, (query) => {
  if (appStore.cwd) {
    sessionStore.debouncedSearchMessages(appStore.cwd, query)
  }
})

function handleRefresh() {
  if (appStore.cwd) {
    sessionStore.loadHistorySessions(appStore.cwd, true)
  }
}

function handleKeydown(e: KeyboardEvent) {
  if (e.key === 'Escape') emit('close')
}

onMounted(() => {
  window.addEventListener('keydown', handleKeydown)
})

onUnmounted(() => {
  window.removeEventListener('keydown', handleKeydown)
})
</script>

<style scoped>
.sessions-panel {
  display: flex;
  flex-direction: column;
  height: 100%;
}

.sessions-panel :deep(.action-btn:not(.with-hint)) {
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

.sessions-panel :deep(.action-btn:not(.with-hint) img) {
  width: 16px;
  height: 16px;
}

.sessions-panel :deep(.action-btn:not(.with-hint):hover) {
  color: var(--text-primary);
}

.search-box {
  display: flex;
  align-items: center;
  gap: 8px;
  padding: 5px 10px;
  background: var(--bg-primary);
  border: 1px solid var(--border-color);
  border-radius: 6px;
  margin: 8px 12px;
}

.search-icon {
  width: 14px;
  height: 14px;
  opacity: 0.6;
}

.search-clear-btn {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 18px;
  height: 18px;
  border: none;
  background: transparent;
  color: var(--text-tertiary);
  cursor: pointer;
  padding: 0;
  border-radius: 4px;
  flex-shrink: 0;
}

.search-clear-btn img {
  width: 12px;
  height: 12px;
}

.search-clear-btn:hover {
  color: var(--text-primary);
  background: var(--bg-tertiary);
}

.search-input {
  flex: 1;
  border: none;
  background: transparent;
  font-size: 13px;
  color: var(--text-primary);
  outline: none;
}

.search-input::placeholder {
  color: var(--text-tertiary);
}

.panel-content {
  flex: 1;
  overflow-y: auto;
  padding: 0 12px;
  min-height: 0;
}

.section {
  margin-bottom: 12px;
}

.subsection {
  margin-top: 8px;
}

.subsection-title {
  font-size: 10px;
  font-weight: 500;
  color: var(--text-tertiary);
  padding: 0 4px 4px;
  text-transform: uppercase;
  letter-spacing: 0.3px;
}

.section-title {
  font-size: 11px;
  font-weight: 600;
  color: var(--text-tertiary);
  text-transform: uppercase;
  padding: 0 4px;
}

.section-title-row {
  display: flex;
  align-items: center;
  justify-content: space-between;
  margin-bottom: 6px;
}

.section-actions {
  display: flex;
  gap: 4px;
}

.section-action-btn {
  font-size: 10px;
  color: var(--text-tertiary);
  background: none;
  border: none;
  cursor: pointer;
  padding: 2px 6px;
  border-radius: 3px;
  white-space: nowrap;
}

.section-action-btn:hover {
  color: var(--text-primary);
  background: var(--bg-secondary);
}

.loading-indicator,
.empty-hint {
  text-align: center;
  padding: 12px;
  font-size: 12px;
  color: var(--text-secondary);
}

.panel-footer {
  border-top: 1px solid var(--border-color);
  padding: 8px 12px;
}

.action-buttons {
  display: flex;
  gap: 8px;
  margin-bottom: 6px;
}

.action-btn {
  flex: 1;
  display: flex;
  align-items: center;
  justify-content: center;
  gap: 6px;
  padding: 8px 12px;
  border: 1px solid var(--border-color);
  background: var(--bg-primary);
  color: var(--text-primary);
  cursor: pointer;
  border-radius: 6px;
  font-size: 13px;
  transition: all 0.15s ease;
}

.action-btn img {
  width: 14px;
  height: 14px;
}

.action-btn:hover:not(:disabled) {
  border-color: var(--accent-color);
  color: var(--accent-color);
}

.action-btn:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}

.action-btn.with-hint {
  flex-direction: column;
  padding: 4px 12px;
  gap: 2px;
}

.btn-content {
  display: flex;
  align-items: center;
  justify-content: center;
  gap: 6px;
}

.btn-label {
  font-size: 13px;
}

.btn-hint {
  font-size: 10px;
  color: var(--text-tertiary);
  font-family: var(--font-mono);
}

.options-content {
  padding: 6px 8px;
  background: var(--bg-primary);
  border-radius: 6px;
  border: 1px solid var(--border-color);
}

.option-item {
  display: flex;
  align-items: center;
  gap: 8px;
  margin-bottom: 4px;
  cursor: pointer;
}

.option-item:last-child {
  margin-bottom: 0;
}

.option-item input[type="checkbox"] {
  width: 14px;
  height: 14px;
  accent-color: var(--accent-color);
}

.option-label {
  font-size: 12px;
  color: var(--text-primary);
}

.option-flag {
  font-size: 10px;
  padding: 1px 5px;
  background: var(--bg-secondary);
  border: 1px solid var(--border-color);
  border-radius: 3px;
  font-family: 'Consolas', 'Monaco', monospace;
  color: var(--text-secondary);
}

.option-flag.warning {
  color: var(--status-error);
  border-color: rgba(232, 112, 90, 0.3);
}

.text-option {
  flex-direction: column;
  align-items: flex-start;
  gap: 4px;
}

.text-option input[type="text"] {
  width: 100%;
  padding: 5px 8px;
  background: var(--bg-secondary);
  border: 1px solid var(--border-color);
  border-radius: 4px;
  font-size: 12px;
  color: var(--text-primary);
}

.text-option input[type="text"]:focus {
  outline: none;
  border-color: var(--accent-color);
}
</style>
