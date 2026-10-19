<template>
  <div class="providers-section">
    <div class="section-header">
      <h2 class="section-title">{{ t('apiProviders') }}</h2>
      <p class="section-desc">{{ t('providersDesc') }}</p>
    </div>

    <div class="toolbar">
      <button class="btn-primary" @click="showPresetPanel = true">
        <span class="btn-icon">+</span>
        {{ t('addProvider') }}
      </button>
      <button class="btn-secondary" @click="showCommonPanel = true">
        {{ t('editCommonConfig') }}
      </button>
      <button
        v-if="providersStore.hasCcSwitchDb"
        class="btn-secondary"
        @click="handleImportCcSwitch"
      >
        {{ t('importFromCcSwitch') }}
      </button>
    </div>

    <div v-if="providersStore.isLoading" class="loading-state">
      {{ t('loading') }}
    </div>

    <div v-else-if="providersStore.providers.length === 0" class="empty-state">
      <p>{{ t('noProviders') }}</p>
      <p class="empty-hint">{{ t('noProvidersHint') }}</p>
    </div>

    <ProviderList
      v-else
      :providers="providersStore.providers"
      :active-id="providersStore.activeProviderId"
      :testing-id="testingId"
      @activate="handleActivate"
      @edit="handleEdit"
      @test="handleTest"
      @delete="handleDelete"
      @reorder="handleReorder"
    />

    <!-- 预设选择面板 -->
    <ProviderPresetPanel
      v-if="showPresetPanel"
      @close="showPresetPanel = false"
      @select="handleSelectPreset"
    />

    <!-- 通用配置面板 -->
    <CommonConfigPanel
      v-if="showCommonPanel"
      :config="providersStore.commonConfig"
      :initial-settings="commonPanelInitialSettings"
      :source-json="providerSourceJson"
      @close="handleCloseCommonPanel"
      @save="handleSaveCommon"
    />

    <!-- 编辑面板 -->
    <ProviderEditPanel
      v-if="editingProvider"
      :provider="editingProvider"
      :common-config="providersStore.commonConfig"
      @close="handleEditClose"
      @save="handleSaveProvider"
      @open-common-config="handleOpenCommonConfig"
    />

    <!-- 删除确认对话框 -->
    <div v-if="deleteConfirm" class="confirm-overlay" @click.self="deleteConfirm = null">
      <div class="confirm-dialog">
        <p class="confirm-text">{{ t('confirmDeleteProvider', { name: deleteConfirm.name }) }}</p>
        <p class="confirm-hint">{{ t('irreversible') }}</p>
        <div class="confirm-actions">
          <button class="btn-cancel" @click="deleteConfirm = null">{{ t('cancel') }}</button>
          <button class="btn-danger" @click="confirmDelete">{{ t('delete') }}</button>
        </div>
      </div>
    </div>

    <!-- 测试结果对话框 -->
    <div v-if="testResult" class="confirm-overlay" @click.self="testResult = null">
      <div class="confirm-dialog">
        <p class="confirm-text">
          <span :class="testResult.success ? 'test-success' : 'test-fail'">
            {{ testResult.success ? t('connectionSuccess') : t('connectionFailed') }}
          </span>
        </p>
        <p class="confirm-hint">{{ testResult.message }}</p>
        <p v-if="testResult.latencyMs != null" class="confirm-hint">
          {{ t('latency', { ms: testResult.latencyMs }) }}
        </p>
        <div class="confirm-actions">
          <button class="btn-cancel" @click="testResult = null">{{ t('close') }}</button>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { useI18n } from 'vue-i18n'
import { useProvidersStore } from '@/stores/providers'
import ProviderList from '../providers/ProviderList.vue'
import ProviderPresetPanel from '../providers/ProviderPresetPanel.vue'
import CommonConfigPanel from '../providers/CommonConfigPanel.vue'
import ProviderEditPanel from '../providers/ProviderEditPanel.vue'
import type { Provider, ProviderPreset } from '@/types/provider'

const { t } = useI18n()
const providersStore = useProvidersStore()

const showPresetPanel = ref(false)
const showCommonPanel = ref(false)
const editingProvider = ref<Provider | null>(null)
const isNewProvider = ref(false)
const commonPanelInitialSettings = ref<Record<string, any> | null>(null)
const providerSourceJson = ref<Record<string, any> | null>(null)
const deleteConfirm = ref<Provider | null>(null)
const testingId = ref<string | null>(null)
const testResult = ref<{ success: boolean; message: string; latencyMs: number | null } | null>(null)

onMounted(() => {
  providersStore.loadProvidersConfig()
})

async function handleActivate(id: string) {
  try {
    await providersStore.activate(id)
  } catch (err) {
    console.error('Activate failed:', err)
  }
}

function handleEdit(provider: Provider) {
  editingProvider.value = provider
  isNewProvider.value = false
}

async function handleDelete(id: string) {
  const provider = providersStore.providers.find(p => p.id === id)
  if (provider) {
    deleteConfirm.value = provider
  }
}

async function confirmDelete() {
  if (!deleteConfirm.value) return
  try {
    await providersStore.remove(deleteConfirm.value.id)
  } catch (err) {
    console.error('Delete failed:', err)
  }
  deleteConfirm.value = null
}

function handleTest(provider: Provider) {
  testingId.value = provider.id
  testResult.value = null
  providersStore.testConnection(provider.id)
    .then(result => {
      testResult.value = result
    })
    .catch(err => {
      testResult.value = { success: false, message: t('testFailed', { error: String(err) }), latencyMs: null }
    })
    .finally(() => {
      testingId.value = null
    })
}

async function handleReorder(newOrder: string[]) {
  await providersStore.reorder(newOrder)
}

async function handleSelectPreset(preset: ProviderPreset) {
  showPresetPanel.value = false
  editingProvider.value = providersStore.createLocalFromPreset(preset)
  isNewProvider.value = true
}

async function handleSaveCommon(settings: Record<string, any>) {
  try {
    await providersStore.updateCommon(settings)
    showCommonPanel.value = false
    commonPanelInitialSettings.value = null
  } catch (err) {
    console.error('Save common config failed:', err)
  }
}

function handleCloseCommonPanel() {
  showCommonPanel.value = false
  commonPanelInitialSettings.value = null
  providerSourceJson.value = null
}

function handleOpenCommonConfig(currentJson: Record<string, any>) {
  commonPanelInitialSettings.value = null
  providerSourceJson.value = currentJson
  showCommonPanel.value = true
}

function handleEditClose() {
  editingProvider.value = null
  isNewProvider.value = false
}

async function handleSaveProvider(provider: Provider) {
  try {
    if (isNewProvider.value) {
      await providersStore.create(provider)
    } else {
      await providersStore.update(provider.id, {
        name: provider.name,
        settingsConfig: provider.settingsConfig,
        notes: provider.notes,
        meta: provider.meta
      })
    }
    editingProvider.value = null
    isNewProvider.value = false
  } catch (err) {
    console.error('Save provider failed:', err)
  }
}

async function handleImportCcSwitch() {
  try {
    const result = await providersStore.importCcSwitch()
    if (result.count > 0) {
      const parts = [t('importSuccess', { count: result.count })]
      if (result.importedCommonConfig) parts.push(t('commonConfigImported'))
      if (result.activeProviderName) parts.push(t('activeProvider', { name: result.activeProviderName }))
      alert(parts.join('\n'))
    } else {
      alert(t('noProvidersToImport'))
    }
  } catch (err) {
    console.error('Import failed:', err)
    alert(t('importFailed'))
  }
}
</script>

<style scoped>
.providers-section {
  display: flex;
  flex-direction: column;
  gap: 20px;
}

.section-header {
  margin-bottom: 8px;
}

.section-title {
  font-size: 18px;
  font-weight: 600;
  color: var(--text-primary);
  margin: 0 0 8px 0;
}

.section-desc {
  font-size: 13px;
  color: var(--text-secondary);
  margin: 0;
}

.toolbar {
  display: flex;
  gap: 12px;
}

.btn-primary {
  display: flex;
  align-items: center;
  gap: 6px;
  padding: 8px 16px;
  background: var(--accent-color);
  color: white;
  border: none;
  border-radius: 6px;
  font-size: 13px;
  cursor: pointer;
  transition: opacity 0.15s;
}

.btn-primary:hover {
  opacity: 0.9;
}

.btn-icon {
  font-size: 16px;
}

.btn-secondary {
  padding: 8px 16px;
  background: var(--bg-secondary);
  color: var(--text-primary);
  border: 1px solid var(--border-color);
  border-radius: 6px;
  font-size: 13px;
  cursor: pointer;
  transition: background 0.15s;
}

.btn-secondary:hover {
  background: var(--hover-bg);
}

.loading-state,
.empty-state {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  padding: 60px 20px;
  color: var(--text-secondary);
  font-size: 14px;
}

.empty-hint {
  font-size: 12px;
  margin-top: 8px;
}

.confirm-overlay {
  position: fixed;
  inset: 0;
  background: rgba(0, 0, 0, 0.3);
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 300;
}

.confirm-dialog {
  background: var(--bg-primary);
  border: 1px solid var(--border-color);
  border-radius: var(--radius-lg);
  padding: 24px;
  /* 手机视口内收缩（100vw - 弹窗 padding - 视口边距） */
  min-width: min(320px, calc(100vw - 64px));
  box-shadow: var(--shadow-xl);
}

.confirm-text {
  font-size: 14px;
  font-weight: 500;
  color: var(--text-primary);
  margin: 0 0 6px 0;
}

.confirm-hint {
  font-size: 12px;
  color: var(--text-tertiary);
  margin: 0 0 20px 0;
}

.confirm-actions {
  display: flex;
  justify-content: flex-end;
  gap: 10px;
}

.confirm-actions .btn-cancel {
  padding: 7px 18px;
  background: var(--bg-secondary);
  color: var(--text-primary);
  border: 1px solid var(--border-color);
  border-radius: var(--radius-md);
  font-size: 13px;
  cursor: pointer;
  font-family: var(--font-sans);
}
.confirm-actions .btn-cancel:hover { background: var(--hover-bg); }

.btn-danger {
  padding: 7px 18px;
  background: var(--status-error);
  color: #fff;
  border: none;
  border-radius: var(--radius-md);
  font-size: 13px;
  cursor: pointer;
  font-family: var(--font-sans);
}
.btn-danger:hover { opacity: 0.9; }

.test-success { color: var(--status-success, #22c55e); }
.test-fail { color: var(--status-error, #ef4444); }
</style>
