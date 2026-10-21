<template>
  <div class="section-content">
    <h2 class="section-heading">{{ t('startupDefaults') }}</h2>
    <p class="section-desc">{{ t('startupDefaultsDesc') }}</p>

    <div class="setting-group">
      <label class="setting-card">
        <div class="card-left">
          <input type="checkbox" v-model="skipPermissions" />
          <div class="card-info">
            <span class="setting-label">{{ t('skipPermissions') }}</span>
            <span class="setting-hint">{{ t('skipPermissionsHint') }}</span>
          </div>
        </div>
        <code class="flag-badge warning">--allow-dangerously-skip-permissions</code>
      </label>

      <div class="setting-card text-card">
        <div class="card-info">
          <span class="setting-label">{{ t('customArguments') }}</span>
          <span class="setting-hint">{{ t('customArgumentsHint') }}</span>
        </div>
        <input
          type="text"
          v-model="customArgs"
          :placeholder="t('customArgsPlaceholder')"
          class="text-input"
        />
      </div>
    </div>

    <!-- 启动环境变量 -->
    <div class="env-section">
      <div class="env-header">
        <div>
          <h2 class="section-heading" style="margin-bottom: 4px">{{ t('envVars') }}</h2>
          <p class="section-desc" style="margin-bottom: 0">
            {{ t('envVarsDesc') }}
          </p>
        </div>
        <button class="reset-btn" @click="handleReset">{{ t('resetToDefaults') }}</button>
      </div>

      <div class="env-list">
        <div v-for="(value, key) in envVars" :key="key" class="env-row">
          <input
            type="text"
            class="env-key"
            :value="key"
            @change="handleKeyChange(key, ($event.target as HTMLInputElement).value)"
            spellcheck="false"
          />
          <span class="env-eq">=</span>
          <input
            type="text"
            class="env-value"
            :value="value"
            @change="handleValueChange(key, ($event.target as HTMLInputElement).value)"
            spellcheck="false"
          />
          <button class="env-remove" @click="handleRemove(key)" :title="t('remove')">
            <svg width="14" height="14" viewBox="0 0 14 14" fill="none">
              <path d="M3.5 3.5l7 7M10.5 3.5l-7 7" stroke="currentColor" stroke-width="1.5" stroke-linecap="round"/>
            </svg>
          </button>
        </div>

        <div class="env-add-row">
          <input
            type="text"
            v-model="newKey"
            class="env-key"
            :placeholder="t('envKeyPlaceholder')"
            spellcheck="false"
            @keydown.enter="handleAdd"
          />
          <span class="env-eq">=</span>
          <input
            type="text"
            v-model="newValue"
            class="env-value"
            :placeholder="t('envValuePlaceholder')"
            spellcheck="false"
            @keydown.enter="handleAdd"
          />
          <button class="env-add-btn" @click="handleAdd" :disabled="!newKey.trim()">{{ t('add') }}</button>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, reactive, watch } from 'vue'
import { useI18n } from 'vue-i18n'
import { useAppStore, DEFAULT_CLAUDE_ENV_VARS } from '@/stores/app'

const { t } = useI18n()
const appStore = useAppStore()
const skipPermissions = ref(appStore.defaultClaudeOptions.skipPermissions)
const customArgs = ref(appStore.defaultClaudeOptions.customArgs)
const envVars = reactive<Record<string, string>>({ ...appStore.claudeEnvVars })
const newKey = ref('')
const newValue = ref('')

watch([skipPermissions, customArgs], () => {
  appStore.setDefaultClaudeOptions({
    skipPermissions: skipPermissions.value,
    customArgs: customArgs.value
  })
})

function syncEnv() {
  appStore.setClaudeEnvVars({ ...envVars })
}

function handleKeyChange(oldKey: string, newKeyVal: string) {
  const trimmed = newKeyVal.trim()
  if (!trimmed || trimmed === oldKey) return
  const value = envVars[oldKey]
  delete envVars[oldKey]
  envVars[trimmed] = value
  appStore.setClaudeEnvVars({ ...envVars })
}

function handleValueChange(key: string, val: string) {
  envVars[key] = val
  syncEnv()
}

function handleRemove(key: string) {
  delete envVars[key]
  appStore.setClaudeEnvVars({ ...envVars })
}

function handleAdd() {
  const key = newKey.value.trim()
  if (!key) return
  envVars[key] = newValue.value
  newKey.value = ''
  newValue.value = ''
  syncEnv()
}

function handleReset() {
  for (const [key, value] of Object.entries(DEFAULT_CLAUDE_ENV_VARS)) {
    envVars[key] = value
  }
  syncEnv()
}
</script>

<style scoped>
.section-content {
  padding: 8px 0;
}

.section-heading {
  font-size: 20px;
  font-weight: 600;
  color: var(--text-primary);
  margin-bottom: 8px;
}

.section-desc {
  font-size: 13px;
  color: var(--text-secondary);
  margin-bottom: 24px;
}

.setting-group {
  display: flex;
  flex-direction: column;
  gap: 12px;
}

.setting-card {
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: 12px;
  padding: 12px 16px;
  background: var(--bg-secondary);
  border: 1px solid var(--border-color);
  border-radius: 8px;
  cursor: pointer;
  transition: border-color 0.15s ease;
}

.setting-card:hover {
  border-color: var(--accent-color);
}

.card-left {
  display: flex;
  align-items: center;
  gap: 12px;
}

.card-left input[type="checkbox"] {
  width: 16px;
  height: 16px;
  accent-color: var(--accent-color);
  cursor: pointer;
}

.card-info {
  display: flex;
  flex-direction: column;
  gap: 2px;
}

.setting-label {
  font-size: 14px;
  font-weight: 500;
  color: var(--text-primary);
}

.setting-hint {
  font-size: 12px;
  color: var(--text-tertiary);
}

.flag-badge {
  font-size: 11px;
  padding: 2px 8px;
  background: var(--bg-primary);
  border: 1px solid var(--border-color);
  border-radius: 4px;
  font-family: var(--font-mono);
  color: var(--text-secondary);
  white-space: nowrap;
}

.flag-badge.warning {
  color: var(--error-color);
  border-color: rgba(239, 68, 68, 0.3);
}

.text-card {
  flex-direction: column;
  align-items: flex-start;
  cursor: default;
}

.text-input {
  width: 100%;
  margin-top: 8px;
  padding: 8px 12px;
  background: var(--bg-primary);
  border: 1px solid var(--border-color);
  border-radius: 6px;
  font-size: 13px;
  font-family: var(--font-mono);
  color: var(--text-primary);
  transition: border-color 0.15s ease;
}

.text-input:focus {
  outline: none;
  border-color: var(--accent-color);
}

.text-input::placeholder {
  color: var(--text-tertiary);
}

/* 环境变量区域 */
.env-section {
  margin-top: 32px;
}

.env-header {
  display: flex;
  align-items: flex-start;
  justify-content: space-between;
  margin-bottom: 16px;
}

.reset-btn {
  padding: 6px 14px;
  border: 1px solid var(--border-color);
  background: var(--bg-secondary);
  color: var(--text-secondary);
  border-radius: 6px;
  font-size: 12px;
  cursor: pointer;
  flex-shrink: 0;
  transition: border-color 0.15s ease, color 0.15s ease;
}

.reset-btn:hover {
  border-color: var(--accent-color);
  color: var(--text-primary);
}

.path-hint {
  font-family: var(--font-mono);
  font-size: 11px;
  padding: 1px 5px;
  background: var(--bg-secondary);
  border: 1px solid var(--border-color);
  border-radius: 3px;
  color: var(--text-secondary);
}

.env-list {
  display: flex;
  flex-direction: column;
  gap: 8px;
}

.env-row,
.env-add-row {
  display: flex;
  align-items: center;
  gap: 6px;
}

.env-key {
  width: 200px;
  padding: 6px 10px;
  background: var(--bg-primary);
  border: 1px solid var(--border-color);
  border-radius: 6px;
  font-size: 12px;
  font-family: var(--font-mono);
  color: var(--text-primary);
  transition: border-color 0.15s ease;
}

.env-key:focus {
  outline: none;
  border-color: var(--accent-color);
}

.env-key::placeholder {
  color: var(--text-tertiary);
}

.env-eq {
  font-size: 13px;
  color: var(--text-tertiary);
  font-family: var(--font-mono);
  flex-shrink: 0;
}

.env-value {
  flex: 1;
  padding: 6px 10px;
  background: var(--bg-primary);
  border: 1px solid var(--border-color);
  border-radius: 6px;
  font-size: 12px;
  font-family: var(--font-mono);
  color: var(--text-primary);
  transition: border-color 0.15s ease;
}

.env-value:focus {
  outline: none;
  border-color: var(--accent-color);
}

.env-value::placeholder {
  color: var(--text-tertiary);
}

.env-remove {
  width: 28px;
  height: 28px;
  display: flex;
  align-items: center;
  justify-content: center;
  border: none;
  background: transparent;
  color: var(--text-tertiary);
  cursor: pointer;
  border-radius: 4px;
  flex-shrink: 0;
  transition: color 0.15s ease, background 0.15s ease;
}

.env-remove:hover {
  color: var(--error-color);
  background: rgba(239, 68, 68, 0.08);
}

.env-add-btn {
  padding: 6px 14px;
  border: 1px solid var(--border-color);
  background: var(--bg-secondary);
  color: var(--text-primary);
  border-radius: 6px;
  font-size: 12px;
  cursor: pointer;
  flex-shrink: 0;
  transition: border-color 0.15s ease;
}

.env-add-btn:hover:not(:disabled) {
  border-color: var(--accent-color);
}

.env-add-btn:disabled {
  opacity: 0.4;
  cursor: not-allowed;
}

/* 手机（≤768px）：env-key 固定 200px 与 flag-badge 长参数不换行都会撑破
   视口横向溢出——key 改比例宽度、badge 允许断行 */
@media (max-width: 768px) {
  .env-key {
    width: 38%;
    flex-shrink: 0;
  }

  .flag-badge {
    white-space: normal;
    word-break: break-all;
  }

  .env-header {
    flex-wrap: wrap;
    gap: 8px;
  }
}
</style>
