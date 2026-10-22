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
        <p class="json-hint" v-if="commonMerged">{{ t('commonConfigMergeHint') }}</p>
        <p class="json-hint hint-warn" v-if="commonChangedWhileDirty">{{ t('commonConfigChangedWhileEditing') }}</p>
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
import {
  resolveEffective,
  diffAgainstCommon,
  computeRemovedPaths,
  isCommonActive,
  isPlainObject,
  stripEmptyCoreEnv,
} from '@/utils/provider-config'

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

// 不再使用 DEFAULT_CONFIG 兜底：schema v2 中 `settingsConfig: {}` 是合法且有意义的
// 「完全继承通用配置」，用模板替换会凭空产生空 token 覆盖通用配置。
// 缺省展示由 env 表单（getEnvValue 返回空串）负责，不会写入 JSON。

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

// JSON 内容：展示层（通用配置 + 覆盖层，或仅覆盖层），与表单字段双向同步
const jsonContent = ref('')
const jsonError = ref('')
const showSensitive = reactive<Record<string, boolean>>({})
const showCustomEnv = ref(false)

// 缓存解析后的 JSON，避免 getEnvValue 每次都 JSON.parse
const parsedConfig = computed(() => {
  try { return JSON.parse(jsonContent.value) }
  catch { return null }
})

const commonAvailable = computed(() => isCommonActive(props.commonConfig))
/** 当前编辑器视图是否叠加了通用配置 */
const commonMerged = computed(() => applyCommonConfig.value && commonAvailable.value)

// 是否有未保存修改：程序化写入（切换视图/外部同步）不置脏
const dirty = ref(false)
let suppressDirty = false
watch(jsonContent, () => {
  if (!suppressDirty) dirty.value = true
}, { flush: 'sync' })

/** 通用配置在面板打开期间被更新，但当前有未保存修改 → 不覆盖，仅提示 */
const commonChangedWhileDirty = ref(false)

/** 打开面板时的 effective 快照（保存时用于计算被删除的继承项） */
let initialEffective: Record<string, any> = {}
/** 当前覆盖层（非编辑状态下的来源，用于外部 common 变化时重新展开） */
let lastOverrides: Record<string, any> = {}
/**
 * 当前编辑器视图对应的通用配置快照。
 * dirty 期间外部 common 变化时保存仍按此快照差分，避免把旧继承值冻结成覆盖层。
 */
let viewCommonSettings: Record<string, any> = {}
/** 当前视图是否叠加了通用配置（打开/切换/非 dirty 重算时确定） */
let viewHadCommon = false

function pretty(value: Record<string, any>): string {
  return JSON.stringify(value, null, 2)
}

/** 程序化写入编辑器（不置 dirty） */
function setJsonContent(content: string) {
  suppressDirty = true
  jsonContent.value = content
  jsonError.value = ''
  queueMicrotask(() => { suppressDirty = false })
}

/**
 * 把通用配置展开到当前视图；去掉用户删除的继承项由 resolveEffective 处理。
 * 不在 JSON 里补空 env 占位：否则保存时会被当成显式覆盖，把用户 settings.json
 * 中已有的值覆盖成空串（必需项由 env 表单直接展示为空输入框）。
 */
function effectiveFrom(overrides: Record<string, any>): Record<string, any> {
  return commonMerged.value
    ? resolveEffective(props.commonConfig, overrides, props.provider.meta)
    : JSON.parse(JSON.stringify(overrides))
}

// ── 勾选/取消"应用通用配置" → 在 effective / 覆盖层两种视图间转换 ──
watch(applyCommonConfig, (now) => {
  const current = parsedConfig.value ?? {}
  // 视图切换以当前实际 common 为准，并同步快照
  viewHadCommon = now && commonAvailable.value
  viewCommonSettings = JSON.parse(JSON.stringify(props.commonConfig?.settings ?? {}))

  if (now) {
    // 当前视图是覆盖层 → 展开为 effective
    lastOverrides = JSON.parse(JSON.stringify(current))
    setJsonContent(pretty(effectiveFrom(lastOverrides)))
  } else {
    // 当前视图是 effective → 提取覆盖层（与通用配置相同的值自动继承）
    const overrides = commonAvailable.value
      ? diffAgainstCommon(current, viewCommonSettings)
      : current
    lastOverrides = JSON.parse(JSON.stringify(overrides))
    setJsonContent(pretty(JSON.parse(JSON.stringify(overrides))))
  }
  initialEffective = JSON.parse(JSON.stringify(parsedConfig.value ?? {}))
})

// ── 通用配置外部修改（如从 CommonConfigPanel 保存） ──
// 有未保存修改时不覆盖编辑器，只提示；否则按最新 common 重新展开。
// common 变为「未启用/空」时也要重算回覆盖层视图，否则旧继承值会被当成覆盖层保存。
watch(() => props.commonConfig, (newConfig) => {
  if (!applyCommonConfig.value) return
  if (dirty.value) {
    commonChangedWhileDirty.value = true
    return
  }
  viewHadCommon = isCommonActive(newConfig)
  viewCommonSettings = JSON.parse(JSON.stringify(newConfig?.settings ?? {}))
  const display = viewHadCommon
    ? resolveEffective(newConfig, lastOverrides, props.provider.meta)
    : JSON.parse(JSON.stringify(lastOverrides))
  initialEffective = JSON.parse(JSON.stringify(display))
  setJsonContent(pretty(display))
}, { deep: true })

// 内联添加自定义变量
const isAddingEnv = ref(false)
const newEnvKey = ref('')
const newEnvValue = ref('')
const newEnvKeyInput = ref<HTMLInputElement | null>(null)

onMounted(() => {
  // 已存在的 Provider 一律以真实覆盖层为起点；空对象表示「完全继承通用配置」
  const stored = JSON.parse(JSON.stringify(props.provider.settingsConfig ?? {}))
  lastOverrides = JSON.parse(JSON.stringify(stored))
  viewHadCommon = commonMerged.value
  viewCommonSettings = JSON.parse(JSON.stringify(props.commonConfig?.settings ?? {}))
  initialEffective = effectiveFrom(stored)
  setJsonContent(pretty(initialEffective))
  dirty.value = false
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

// 必需项始终展示（缺失时输入框为空，但不会写入 JSON）；
// 若按 `k in env` 过滤，{env:{}} 的预设会连输入行都消失
const requiredEnvKeys = computed(() => [...REQUIRED_ENV_KEYS])

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
    // 纯格式化不改变语义，不应把面板标记为“有未保存修改”
    setJsonContent(JSON.stringify(parsed, null, 2))
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
    const current = JSON.parse(jsonContent.value)
    // 根必须是对象：非对象会在激活时把 settings.json 根写坏
    if (!isPlainObject(current)) {
      jsonError.value = t('configRootMustBeObject')
      return
    }
    const prevRemoved = props.provider.meta?.commonConfigRemoved ?? []

    let settingsConfig: Record<string, any>
    let commonConfigRemoved: string[]

    if (viewHadCommon) {
      // 展示的是 effective → 按「打开/重算时的 common 快照」差分，
      // 避免 dirty 期间 common 变化导致旧继承值被冻结成覆盖层
      settingsConfig = diffAgainstCommon(current, viewCommonSettings)
      commonConfigRemoved = computeRemovedPaths(
        initialEffective,
        current,
        prevRemoved,
        viewCommonSettings,
        settingsConfig
      )
    } else {
      // 未叠加通用配置：编辑器内容就是覆盖层。
      // 保留 removed：取消勾选只是暂停继承，再次勾选应恢复原有屏蔽意图
      settingsConfig = current
      commonConfigRemoved = prevRemoved
    }
    // 空字符串核心 env 视为未设置，避免覆盖用户/通用配置的有效值
    settingsConfig = stripEmptyCoreEnv(settingsConfig)

    const updatedProvider: Provider = {
      ...props.provider,
      name: editName.value,
      notes: editNotes.value,
      settingsConfig,
      meta: {
        ...props.provider.meta,
        commonConfigEnabled: applyCommonConfig.value,
        commonConfigRemoved,
      }
    }
    // dirty 由父组件保存成功/面板关闭决定；这里不清，避免保存失败后丢失保护
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

.json-hint.hint-warn {
  color: var(--accent-gold, #d19a3f);
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
