<template>
  <div class="edit-panel">
    <div class="panel-header">
      <button class="back-btn" @click="$emit('close')">← {{ t('back') }}</button>
      <span class="panel-title">{{ t('editProviderTitle', { name: provider.name }) }}</span>
    </div>

    <div class="panel-body">
      <!-- 基本信息 -->
      <div class="config-section">
        <h3 class="section-title">{{ t('basicInfo') }}</h3>
        <div class="form-row">
          <label class="form-label">{{ t('nameLabel') }}</label>
          <input class="form-input" v-model="editName" :placeholder="t('namePlaceholder')" />
        </div>
        <div class="form-row">
          <label class="form-label">{{ t('notesLabel') }}</label>
          <input class="form-input" v-model="editNotes" :placeholder="t('notesPlaceholder')" />
        </div>
        <div class="form-row" v-if="provider.websiteUrl">
          <label class="form-label">{{ t('websiteLabel') }}</label>
          <div class="url-row">
            <span class="url-text">{{ provider.websiteUrl }}</span>
            <button class="btn-link" @click="openWebsite">{{ t('openWebsite') }}</button>
          </div>
        </div>
      </div>

      <!-- 环境变量（从 JSON 中的 env 提取，双向同步） -->
      <div class="config-section">
        <h3 class="section-title">{{ t('envVariables') }}</h3>
        <div class="env-list">
          <div v-for="key in requiredEnvKeys" :key="key" class="env-row">
            <label class="env-key" :title="key">
              {{ key }}
              <span v-if="isSensitive(key)" class="env-required">*</span>
            </label>
            <div class="env-input-wrap">
              <input
                class="env-input"
                :type="isSensitive(key) && !showSensitive[key] ? 'password' : 'text'"
                :value="getEnvValue(key)"
                @input="setEnvValue(key, ($event.target as HTMLInputElement).value)"
                :placeholder="getPlaceholder(key)"
              />
              <button
                v-if="isSensitive(key)"
                class="btn-eye"
                @click="showSensitive[key] = !showSensitive[key]"
                :title="showSensitive[key] ? t('hide') : t('show')"
              >
                {{ showSensitive[key] ? t('hide') : t('show') }}
              </button>
            </div>
          </div>
          <template v-if="customEnvKeys.length > 0">
            <button class="btn-toggle-custom" @click="showCustomEnv = !showCustomEnv">
              {{ showCustomEnv ? '▾' : '▸' }} {{ t('customVariables', { count: customEnvKeys.length }) }}
            </button>
            <template v-if="showCustomEnv">
              <div class="env-divider"></div>
              <div v-for="key in customEnvKeys" :key="key" class="env-row">
                <label class="env-key" :title="key">
                  {{ key }}
                  <span v-if="isSensitive(key)" class="env-required">*</span>
                </label>
                <div class="env-input-wrap">
                  <input
                    class="env-input"
                    :type="isSensitive(key) && !showSensitive[key] ? 'password' : 'text'"
                    :value="getEnvValue(key)"
                    @input="setEnvValue(key, ($event.target as HTMLInputElement).value)"
                    :placeholder="getPlaceholder(key)"
                  />
                  <button
                    v-if="isSensitive(key)"
                    class="btn-eye"
                    @click="showSensitive[key] = !showSensitive[key]"
                    :title="showSensitive[key] ? t('hide') : t('show')"
                  >
                    {{ showSensitive[key] ? t('hide') : t('show') }}
                  </button>
                  <button class="btn-remove" @click="removeEnvVar(key)">×</button>
                </div>
              </div>
            </template>
          </template>
          <template v-if="isAddingEnv">
            <div class="env-divider"></div>
            <div class="env-row">
              <input
                ref="newEnvKeyInput"
                class="env-input new-key-input"
                v-model="newEnvKey"
                :placeholder="t('varNamePlaceholder')"
                spellcheck="false"
                @keydown.enter="confirmAddEnv"
                @keydown.escape="cancelAddEnv"
              />
              <div class="env-input-wrap">
                <input
                  class="env-input"
                  v-model="newEnvValue"
                  :placeholder="t('varValuePlaceholder')"
                  spellcheck="false"
                  @keydown.enter="confirmAddEnv"
                  @keydown.escape="cancelAddEnv"
                />
                <button class="btn-confirm-add" @click="confirmAddEnv" :title="t('confirmBtn')">✓</button>
                <button class="btn-remove" @click="cancelAddEnv" :title="t('cancel')">×</button>
              </div>
            </div>
          </template>
        </div>
        <button v-if="!isAddingEnv" class="btn-add" @click="startAddEnv">+ {{ t('addVariable') }}</button>
      </div>

      <!-- 完整配置 JSON 编辑器（包含 env，与表单字段双向同步） -->
      <div class="config-section json-section">
        <div class="section-header">
          <h3 class="section-title">{{ t('configJson') }}</h3>
          <div class="section-actions">
            <label class="toggle-row">
              <input type="checkbox" v-model="applyCommonConfig" class="toggle-checkbox" />
              <span class="toggle-label">{{ t('applyCommonConfig') }}</span>
            </label>
            <button class="btn-secondary-sm" @click="handleEditCommon">{{ t('editCommonConfig') }}</button>
            <button class="btn-format" @click="formatJson">{{ t('format') }}</button>
          </div>
        </div>
        <div class="json-editor-wrap" :class="{ 'has-error': jsonError }">
          <codemirror
            v-model="jsonContent"
            :style="{ height: 'auto' }"
            :extensions="cmExtensions"
            @change="onJsonChange"
          />
        </div>
        <p class="json-status" v-if="jsonError">{{ jsonError }}</p>
        <p class="json-hint">{{ t('ignoredFieldsHint') }}</p>
      </div>

      </div>

    <div class="panel-footer">
      <button class="btn-cancel" @click="$emit('close')">{{ t('cancel') }}</button>
      <button class="btn-save" @click="handleSave" :disabled="!!jsonError">{{ t('save') }}</button>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, reactive, onMounted, nextTick, watch } from 'vue'
import { useI18n } from 'vue-i18n'
import { Codemirror } from 'vue-codemirror'
import { json, jsonParseLinter } from '@codemirror/lang-json'
import { oneDark } from '@codemirror/theme-one-dark'
import { linter } from '@codemirror/lint'
import { EditorView } from '@codemirror/view'
import { useAppStore } from '@/stores/app'
import type { Provider, CommonConfig } from '@/types/provider'

const { t } = useI18n()
const appStore = useAppStore()

const props = defineProps<{
  provider: Provider
  commonConfig?: CommonConfig
}>()

const emit = defineEmits<{
  close: []
  save: [provider: Provider]
  openCommonConfig: [currentJson: Record<string, any>]
}>()

// 必需的环境变量，不可删除
const REQUIRED_ENV_KEYS = [
  'ANTHROPIC_AUTH_TOKEN',
  'ANTHROPIC_BASE_URL',
  'ANTHROPIC_MODEL',
  'ANTHROPIC_DEFAULT_HAIKU_MODEL',
  'ANTHROPIC_DEFAULT_OPUS_MODEL',
  'ANTHROPIC_DEFAULT_SONNET_MODEL',
]

const DEFAULT_CONFIG: Record<string, any> = {
  env: {
    ANTHROPIC_AUTH_TOKEN: '',
    ANTHROPIC_BASE_URL: '',
    ANTHROPIC_MODEL: 'claude-sonnet-4-6',
    ANTHROPIC_DEFAULT_HAIKU_MODEL: 'claude-haiku-4-5-20251001',
    ANTHROPIC_DEFAULT_SONNET_MODEL: 'claude-sonnet-4-6',
    ANTHROPIC_DEFAULT_OPUS_MODEL: 'claude-opus-4-7',
  },
  permissions: { allow: [], deny: [] },
}

// CodeMirror 扩展
const cmExtensions = computed(() => {
  const exts: any[] = [
    json(),
    linter(jsonParseLinter()),
    EditorView.theme({
      '&': { fontSize: '14px', fontFamily: 'var(--font-mono)' },
      '.cm-content': { fontFamily: 'var(--font-mono)' },
      '.cm-gutters': { fontFamily: 'var(--font-mono)' },
    }),
  ]
  // 使用应用主题而非系统偏好
  if (appStore.theme === 'dark') {
    exts.push(oneDark)
  }
  return exts
})

const editName = ref(props.provider.name)
const editNotes = ref(props.provider.notes || '')
const applyCommonConfig = ref(props.provider.meta?.commonConfigEnabled ?? true)

// JSON 内容：完整的 settingsConfig（包含 env），与表单字段双向同步
const jsonContent = ref('')
const jsonError = ref('')
const showSensitive = reactive<Record<string, boolean>>({})
const showCustomEnv = ref(false)

// ── 深度合并（与 Rust deep_merge_json 一致） ──
function isObject(val: any): val is Record<string, any> {
  return val !== null && typeof val === 'object' && !Array.isArray(val)
}
function deepMerge(target: any, source: any): any {
  if (isObject(target) && isObject(source)) {
    const result = { ...target }
    for (const key of Object.keys(source)) {
      result[key] = key in result ? deepMerge(result[key], source[key]) : source[key]
    }
    return result
  }
  return source
}

// 缓存解析后的 JSON，避免 getEnvValue 每次都 JSON.parse
const parsedConfig = computed(() => {
  try { return JSON.parse(jsonContent.value) }
  catch { return null }
})

// 通用配置是否可合并
const hasCommonConfig = computed(() => {
  const c = props.commonConfig
  return c?.enabled && c.settings && Object.keys(c.settings).length > 0
})

// ── 勾选"应用通用配置" → 即时合并到 JSON 编辑器 ──
watch(applyCommonConfig, (newVal, oldVal) => {
  if (newVal && !oldVal && hasCommonConfig.value && parsedConfig.value) {
    const merged = deepMerge(parsedConfig.value, props.commonConfig!.settings)
    jsonContent.value = JSON.stringify(merged, null, 2)
    jsonError.value = ''
  }
})

// ── 通用配置外部修改（如从 CommonConfigPanel 保存）→ 重复合并到当前编辑内容 ──
watch(() => props.commonConfig, (newConfig) => {
  if (applyCommonConfig.value && newConfig?.enabled && newConfig.settings &&
      Object.keys(newConfig.settings).length > 0 && parsedConfig.value) {
    const merged = deepMerge(parsedConfig.value, newConfig.settings)
    jsonContent.value = JSON.stringify(merged, null, 2)
    jsonError.value = ''
  }
}, { deep: true })

// 内联添加自定义变量
const isAddingEnv = ref(false)
const newEnvKey = ref('')
const newEnvValue = ref('')
const newEnvKeyInput = ref<HTMLInputElement | null>(null)

onMounted(() => {
  const config = props.provider.settingsConfig
  const initial = config && Object.keys(config).length > 0
    ? config
    : JSON.parse(JSON.stringify(DEFAULT_CONFIG))

  // 确保必需的 env key 存在
  if (!initial.env || typeof initial.env !== 'object') {
    initial.env = {}
  }
  for (const key of REQUIRED_ENV_KEYS) {
    if (!(key in initial.env)) {
      initial.env[key] = ''
    }
  }

  jsonContent.value = JSON.stringify(initial, null, 2)
})

// 从缓存中读取 env 值
function getEnvValue(key: string): string {
  return String(parsedConfig.value?.env?.[key] ?? '')
}

// 将 env 值写回 JSON（双向同步：表单 → JSON）
function setEnvValue(key: string, value: string) {
  const config = parsedConfig.value
  if (!config) return
  if (!config.env || typeof config.env !== 'object') config.env = {}
  config.env[key] = value
  jsonContent.value = JSON.stringify(config, null, 2)
  jsonError.value = ''
}

const requiredEnvKeys = computed(() => {
  const env = parsedConfig.value?.env
  if (!env || typeof env !== 'object') return [...REQUIRED_ENV_KEYS]
  return REQUIRED_ENV_KEYS.filter(k => k in env)
})

const customEnvKeys = computed(() => {
  const env = parsedConfig.value?.env
  if (!env || typeof env !== 'object') return []
  return Object.keys(env).filter(k => !REQUIRED_ENV_KEYS.includes(k))
})

function isSensitive(key: string): boolean {
  return /TOKEN|KEY|SECRET|PASSWORD/i.test(key)
}

function getPlaceholder(key: string): string {
  const map: Record<string, string> = {
    ANTHROPIC_AUTH_TOKEN: 'sk-ant-...',
    ANTHROPIC_API_KEY: 'sk-ant-...',
    ANTHROPIC_BASE_URL: 'https://api.anthropic.com',
    ANTHROPIC_MODEL: 'claude-sonnet-4-6',
    ANTHROPIC_DEFAULT_HAIKU_MODEL: 'claude-haiku-4-5-20251001',
    ANTHROPIC_DEFAULT_SONNET_MODEL: 'claude-sonnet-4-6',
    ANTHROPIC_DEFAULT_OPUS_MODEL: 'claude-opus-4-7',
  }
  return map[key] || ''
}

function startAddEnv() {
  isAddingEnv.value = true
  newEnvKey.value = ''
  newEnvValue.value = ''
  showCustomEnv.value = true
  nextTick(() => {
    newEnvKeyInput.value?.focus()
  })
}

function confirmAddEnv() {
  const key = newEnvKey.value.trim()
  if (!key) return
  const config = parsedConfig.value
  if (!config) return
  if (!config.env) config.env = {}
  if (key in config.env) return
  config.env[key] = newEnvValue.value
  jsonContent.value = JSON.stringify(config, null, 2)
  jsonError.value = ''
  isAddingEnv.value = false
  newEnvKey.value = ''
  newEnvValue.value = ''
}

function cancelAddEnv() {
  isAddingEnv.value = false
  newEnvKey.value = ''
  newEnvValue.value = ''
}

function removeEnvVar(key: string) {
  if (REQUIRED_ENV_KEYS.includes(key)) return
  const config = parsedConfig.value
  if (!config?.env) return
  delete config.env[key]
  jsonContent.value = JSON.stringify(config, null, 2)
}

// JSON 编辑器变化
function onJsonChange(value: string) {
  jsonContent.value = value
  validateJson()
}

function validateJson() {
  try {
    JSON.parse(jsonContent.value)
    jsonError.value = ''
  } catch (e: any) {
    jsonError.value = e.message || t('jsonFormatError')
  }
}

function formatJson() {
  try {
    const parsed = JSON.parse(jsonContent.value)
    jsonContent.value = JSON.stringify(parsed, null, 2)
    jsonError.value = ''
  } catch { /* keep */ }
}

function handleEditCommon() {
  try {
    const current = JSON.parse(jsonContent.value)
    emit('openCommonConfig', current)
  } catch {
    emit('openCommonConfig', {})
  }
}

function openWebsite() {
  if (props.provider.websiteUrl) {
    window.open(props.provider.websiteUrl, '_blank')
  }
}

function handleSave() {
  if (jsonError.value) return
  try {
    const settingsConfig = JSON.parse(jsonContent.value)
    const updatedProvider: Provider = {
      ...props.provider,
      name: editName.value,
      notes: editNotes.value,
      settingsConfig,
      meta: { ...props.provider.meta, commonConfigEnabled: applyCommonConfig.value }
    }
    emit('save', updatedProvider)
  } catch {
    jsonError.value = t('jsonParseError')
  }
}
</script>

<style scoped>
.edit-panel {
  position: absolute;
  inset: 0;
  background: var(--bg-primary);
  z-index: 100;
  display: flex;
  flex-direction: column;
}

.panel-header {
  display: flex;
  align-items: center;
  gap: 16px;
  padding: 14px 20px;
  border-bottom: 1px solid var(--border-color);
  flex-shrink: 0;
}
.back-btn {
  padding: 6px 14px;
  background: transparent;
  color: var(--text-secondary);
  border: 1px solid var(--border-color);
  border-radius: 6px;
  font-size: 13px;
  cursor: pointer;
  font-family: var(--font-sans);
}
.back-btn:hover { background: var(--hover-bg); }
.panel-title {
  font-size: 15px;
  font-weight: 600;
  color: var(--text-primary);
}

.panel-body {
  flex: 1;
  display: flex;
  flex-direction: column;
  min-height: 0;
  overflow-y: auto;
  padding: 20px;
}

.config-section { margin-bottom: 20px; }
.section-title {
  font-size: 13px;
  font-weight: 600;
  color: var(--text-secondary);
  text-transform: uppercase;
  letter-spacing: 0.5px;
  margin: 0 0 14px 0;
}

.json-section {
  display: flex;
  flex-direction: column;
}
.section-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  margin-bottom: 10px;
}
.section-header .section-title { margin: 0; }
.section-actions { display: flex; align-items: center; gap: 12px; }

.form-row {
  display: flex;
  align-items: center;
  gap: 12px;
  margin-bottom: 10px;
}
.form-label {
  font-size: 14px;
  color: var(--text-secondary);
  width: 48px;
  flex-shrink: 0;
}
.form-input {
  flex: 1;
  padding: 7px 12px;
  background: var(--bg-secondary);
  border: 1px solid var(--border-color);
  border-radius: 6px;
  font-size: 14px;
  color: var(--text-primary);
  font-family: var(--font-sans);
}
.form-input:focus {
  outline: none;
  border-color: var(--accent-primary);
  box-shadow: 0 0 0 2px rgba(30, 58, 95, 0.08);
}

.url-row { display: flex; align-items: center; gap: 10px; }
.url-text { font-size: 13px; color: var(--text-secondary); }
.btn-link {
  padding: 4px 12px;
  background: var(--accent-primary);
  color: #fff;
  border: none;
  border-radius: 4px;
  font-size: 12px;
  cursor: pointer;
}
.btn-link:hover { opacity: 0.85; }

.env-list { display: flex; flex-direction: column; gap: 10px; }
.env-row { display: flex; align-items: center; gap: 12px; }
.env-key {
  width: 270px;
  flex-shrink: 0;
  font-size: 13px;
  font-family: var(--font-mono);
  color: var(--text-primary);
  text-align: right;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}
.env-required { color: var(--error-color, #c0392b); margin-left: 2px; }
.env-input-wrap {
  flex: 1;
  display: flex;
  align-items: center;
  gap: 6px;
}
.env-input {
  flex: 1;
  padding: 7px 12px;
  background: var(--bg-secondary);
  border: 1px solid var(--border-color);
  border-radius: 6px;
  font-size: 14px;
  font-family: var(--font-mono);
  color: var(--text-primary);
}
.env-input:focus {
  outline: none;
  border-color: var(--accent-primary);
  box-shadow: 0 0 0 2px rgba(30, 58, 95, 0.08);
}
.btn-eye {
  padding: 5px 10px;
  background: var(--bg-secondary);
  color: var(--text-tertiary);
  border: 1px solid var(--border-color);
  border-radius: 4px;
  font-size: 11px;
  cursor: pointer;
  font-family: var(--font-sans);
  flex-shrink: 0;
}
.btn-eye:hover { background: var(--hover-bg); color: var(--text-secondary); }
.btn-remove {
  padding: 5px 10px;
  background: transparent;
  color: var(--error-color, #c0392b);
  border: 1px solid var(--border-color);
  border-radius: 4px;
  font-size: 13px;
  cursor: pointer;
  font-family: var(--font-sans);
  flex-shrink: 0;
  line-height: 1;
}
.btn-remove:hover { background: var(--hover-bg); }
.btn-confirm-add {
  padding: 5px 10px;
  background: transparent;
  color: var(--status-success);
  border: 1px solid var(--border-color);
  border-radius: 4px;
  font-size: 13px;
  cursor: pointer;
  font-family: var(--font-sans);
  flex-shrink: 0;
  line-height: 1;
}
.btn-confirm-add:hover { background: var(--hover-bg); }
.new-key-input {
  width: 240px;
  flex-shrink: 0;
}
.env-divider {
  height: 1px;
  background: var(--border-color);
  margin: 6px 0;
}
.btn-add {
  margin-top: 12px;
  padding: 7px 14px;
  background: var(--bg-secondary);
  color: var(--text-secondary);
  border: 1px solid var(--border-color);
  border-radius: 6px;
  font-size: 13px;
  cursor: pointer;
  font-family: var(--font-sans);
}
.btn-add:hover { background: var(--hover-bg); }
.btn-toggle-custom {
  padding: 4px 0;
  background: transparent;
  border: none;
  color: var(--text-secondary);
  font-size: 12px;
  cursor: pointer;
  font-family: var(--font-sans);
}

.toggle-row {
  display: flex;
  align-items: center;
  gap: 8px;
  cursor: pointer;
}
.toggle-checkbox { width: 15px; height: 15px; cursor: pointer; }
.toggle-label { font-size: 13px; color: var(--text-primary); }
.btn-format {
  padding: 4px 12px;
  background: var(--bg-secondary);
  color: var(--text-secondary);
  border: 1px solid var(--border-color);
  border-radius: 4px;
  font-size: 12px;
  cursor: pointer;
  font-family: var(--font-sans);
}
.btn-format:hover { background: var(--hover-bg); }
.btn-secondary-sm {
  padding: 4px 12px;
  background: var(--bg-secondary);
  color: var(--text-secondary);
  border: 1px solid var(--border-color);
  border-radius: 4px;
  font-size: 12px;
  cursor: pointer;
  font-family: var(--font-sans);
}
.btn-secondary-sm:hover { background: var(--hover-bg); }

.json-editor-wrap {
  overflow: hidden;
  border: 1px solid var(--border-color);
  border-radius: 6px;
}
.json-editor-wrap.has-error {
  border-color: var(--error-color, #c0392b);
}

.json-status {
  font-size: 12px;
  padding: 6px 0 0 0;
  color: var(--error-color, #c0392b);
  flex-shrink: 0;
}

.json-hint {
  font-size: 11px;
  padding: 4px 0 0 0;
  color: var(--text-tertiary, #999);
  flex-shrink: 0;
  line-height: 1.5;
}

.panel-footer {
  display: flex;
  justify-content: flex-end;
  gap: 12px;
  padding: 14px 20px;
  border-top: 1px solid var(--border-color);
  flex-shrink: 0;
}
.btn-cancel {
  padding: 8px 20px;
  background: var(--bg-secondary);
  color: var(--text-primary);
  border: 1px solid var(--border-color);
  border-radius: 6px;
  font-size: 13px;
  cursor: pointer;
  font-family: var(--font-sans);
}
.btn-cancel:hover { background: var(--hover-bg); }
.btn-save {
  padding: 8px 20px;
  background: var(--accent-primary);
  color: #fff;
  border: none;
  border-radius: 6px;
  font-size: 13px;
  cursor: pointer;
  font-family: var(--font-sans);
}
.btn-save:hover { opacity: 0.9; }
.btn-save:disabled { opacity: 0.5; cursor: not-allowed; }
</style>
