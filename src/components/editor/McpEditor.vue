<template>
  <EditorModal
    :visible="visible"
    :title="isEdit ? t('editorMcpEditTitle', { name: originalName }) : t('editorMcpNewTitle')"
    :busy="busy"
    :dirty="dirty"
    :can-save="canSave"
    :deletable="isEdit && deletableSource"
    @close="$emit('update:visible', false)"
    @save="onSave"
    @delete="onDelete"
  >
    <template #header-extra>
      <ModeSwitch v-model="mode" :modes="modeOptions" />
    </template>

    <div v-if="mode === 'form'">
      <div class="editor-field-row">
        <div class="editor-field">
          <span class="editor-label">{{ t('editorName') }}<span class="editor-req">*</span></span>
          <input
            v-model="form.name"
            class="editor-input"
            :placeholder="t('editorMcpNamePlaceholder')"
          />
        </div>
        <div class="editor-field" style="max-width: 140px">
          <span class="editor-label">{{ t('editorScope') }}</span>
          <select v-model="form.scope" class="editor-select">
            <option value="user">{{ t('scopeUser') }}</option>
            <option v-if="hasProject || form.scope === 'project'" value="project">{{ t('scopeProject') }}</option>
            <option v-if="hasProject || form.scope === 'local'" value="local">{{ t('scopeLocal') }}</option>
          </select>
          <span v-if="isEdit && form.scope !== originalScope" class="editor-hint">{{ t('editorScopeMoveHint') }}</span>
        </div>
      </div>

      <div class="editor-field">
        <span class="editor-label">{{ t('editorMcpType') }}</span>
        <select v-model="form.type" class="editor-select">
          <option value="stdio">{{ t('editorMcpTypeStdio') }}</option>
          <option value="http">HTTP</option>
          <option value="sse">SSE</option>
        </select>
      </div>

      <template v-if="form.type === 'stdio'">
        <div class="editor-field">
          <span class="editor-label">{{ t('editorMcpCommand') }}<span class="editor-req">*</span></span>
          <input v-model="form.command" class="editor-input" spellcheck="false" />
          <span class="editor-hint">{{ t('editorMcpExpandHint') }}</span>
        </div>
        <div class="editor-field">
          <span class="editor-label">{{ t('editorMcpArgs') }}</span>
          <input v-model="form.args" class="editor-input" spellcheck="false" />
          <span class="editor-hint">{{ t('editorMcpArgsHint') }}</span>
        </div>
      </template>

      <template v-else>
        <div class="editor-field">
          <span class="editor-label">URL<span class="editor-req">*</span></span>
          <input v-model="form.url" class="editor-input" spellcheck="false" placeholder="https://…" />
          <span class="editor-hint">{{ t('editorMcpExpandHint') }}</span>
        </div>
        <div class="editor-field">
          <span class="editor-label">{{ t('editorMcpHeaders') }}</span>
          <div v-for="(row, i) in form.headers" :key="i" class="editor-kv-row">
            <input v-model="row.key" class="editor-input" :placeholder="t('editorKvKey')" spellcheck="false" />
            <input v-model="row.value" class="editor-input" :placeholder="t('editorKvValue')" spellcheck="false" />
            <button class="editor-kv-remove" @click="form.headers.splice(i, 1)" :title="t('remove')">×</button>
          </div>
          <button class="editor-kv-add" @click="form.headers.push({ key: '', value: '' })">＋ {{ t('editorKvAdd') }}</button>
        </div>
      </template>

      <div class="editor-field">
        <span class="editor-label">{{ t('editorMcpEnv') }}</span>
        <div v-for="(row, i) in form.env" :key="i" class="editor-kv-row">
          <input v-model="row.key" class="editor-input" :placeholder="t('editorKvKey')" spellcheck="false" />
          <input v-model="row.value" class="editor-input" :placeholder="t('editorKvValue')" spellcheck="false" />
          <button class="editor-kv-remove" @click="form.env.splice(i, 1)" :title="t('remove')">×</button>
        </div>
        <button class="editor-kv-add" @click="form.env.push({ key: '', value: '' })">＋ {{ t('editorKvAdd') }}</button>
        <span class="editor-hint">{{ t('editorMcpEnvHint') }}</span>
      </div>
    </div>

    <div v-else class="editor-field">
      <span class="editor-label">JSON</span>
      <textarea v-model="jsonText" class="editor-textarea" rows="14" spellcheck="false"></textarea>
      <span v-if="jsonError" class="editor-error" style="margin: 6px 0 0">{{ jsonError }}</span>
    </div>

    <div v-if="error" class="editor-error">{{ error }}</div>
  </EditorModal>
</template>

<script setup lang="ts">
import { computed, nextTick, ref, watch } from 'vue'
import { useI18n } from 'vue-i18n'
import { useAppStore } from '@/stores/app'
import { deleteMcpServer, saveMcpServer, type McpServerInfo } from '@/api/tauri'
import { uiConfirm } from '@/composables/useConfirm'
import EditorModal from './EditorModal.vue'
import ModeSwitch from './ModeSwitch.vue'
import { argsJoin, mcpFormToConfig, type McpForm } from './model'

const { t } = useI18n()
const appStore = useAppStore()

const props = defineProps<{
  visible: boolean
  /** null = 新建 */
  server: McpServerInfo | null
}>()

const emit = defineEmits<{
  'update:visible': [v: boolean]
  saved: []
}>()

const hasProject = computed(() => !!appStore.cwd)
const isEdit = computed(() => !!props.server)
/** plugin/managed 来源只读不可删（其生命周期归插件管） */
const deletableSource = computed(
  () => props.server?.sourceType === 'user' || props.server?.sourceType === 'project' || props.server?.sourceType === 'local'
)

const mode = ref<'form' | 'json'>('form')
const modeOptions = [
  { value: 'form', label: t('editorModeForm') },
  { value: 'json', label: 'JSON' },
]

const emptyForm = (): McpForm => ({
  scope: 'user',
  name: '',
  type: 'stdio',
  command: '',
  args: '',
  url: '',
  env: [],
  headers: [],
})

const form = ref<McpForm>(emptyForm())
const originalName = ref('')
/** 编辑打开时的作用域（跨作用域保存 = 从旧作用域移除后写入新作用域） */
const originalScope = ref<McpForm['scope']>('user')
const jsonText = ref('')
const jsonError = ref('')
const error = ref('')
const busy = ref(false)

/** 表单基线（脏检查） */
let baseline = ''

function currentConfigJson(): string | null {
  const config = mcpFormToConfig(form.value)
  return config ? JSON.stringify(config, null, 2) : null
}

const dirty = computed(() => {
  if (mode.value === 'json') return jsonText.value !== baseline
  return (currentConfigJson() ?? '') + form.value.name !== baseline
})

const canSave = computed(() => {
  if (mode.value === 'json') return !jsonError.value && !!jsonText.value.trim()
  return true
})

watch(
  () => props.visible,
  v => {
    if (!v) return
    error.value = ''
    busy.value = false
    // 程序化切模式（含未知形态的 form→json）不触发 watch(mode) 的 jsonText 生成，
    // 否则会覆盖未知形态分支精心拼装的配置（打开即丢字段 + 误报 dirty）
    restoring.value = true
    mode.value = 'form'
    jsonError.value = ''
    if (props.server) {
      originalName.value = props.server.name
      const scope = (props.server.sourceType as McpForm['scope']) ?? 'user'
      originalScope.value = scope
      const type = props.server.serverType
      if (type === 'stdio' || type === 'http' || type === 'sse') {
        // 从列表展开字段还原表单（get_all_mcp_servers 已展开 command/args/env/headers）
        form.value = {
          scope,
          name: props.server.name,
          type,
          command: props.server.command ?? '',
          args: argsJoin((props.server.args ?? []).map(String)),
          url: props.server.url ?? '',
          env: Object.entries(props.server.env ?? {}).map(([key, value]) => ({ key, value })),
          headers: Object.entries(props.server.headers ?? {}).map(([key, value]) => ({ key, value })),
        }
      } else {
        // 未知形态（plugin 内嵌 server 等）：JSON 模式展示已知字段
        form.value = { ...emptyForm(), scope, name: props.server.name }
        mode.value = 'json'
        jsonText.value = JSON.stringify(
          {
            ...(props.server.command ? { type: 'stdio', command: props.server.command } : {}),
            ...(props.server.args?.length ? { args: props.server.args } : {}),
            ...(props.server.url ? { type: props.server.serverType ?? 'http', url: props.server.url } : {}),
            ...(props.server.env && Object.keys(props.server.env).length ? { env: props.server.env } : {}),
            ...(props.server.headers && Object.keys(props.server.headers).length ? { headers: props.server.headers } : {}),
          },
          null,
          2
        )
      }
    } else {
      originalName.value = ''
      form.value = emptyForm()
    }
    // 未知形态分支已自带 jsonText（覆盖会丢配置），只在表单模式生成
    if (mode.value !== 'json') {
      jsonText.value = JSON.stringify(currentFormConfig(), null, 2)
    }
    baseline = snapshot()
    void nextTick().then(() => {
      restoring.value = false
    })
  }
)

function currentFormConfig(): Record<string, unknown> {
  return mcpFormToConfig(form.value) ?? { type: form.value.type }
}

function snapshot(): string {
  return mode.value === 'json' ? jsonText.value : (currentConfigJson() ?? '') + form.value.name
}

// 表单 → JSON 同步（用户切模式时带过去；打开时的程序化切换跳过）
const restoring = ref(false)
watch(mode, m => {
  if (restoring.value) return
  if (m === 'json') {
    jsonError.value = ''
    jsonText.value = JSON.stringify(currentFormConfig(), null, 2)
  }
})

// JSON 编辑错误提示（实时）
watch(jsonText, text => {
  if (mode.value !== 'json') return
  try {
    const parsed = JSON.parse(text)
    if (!parsed || typeof parsed !== 'object' || Array.isArray(parsed)) {
      jsonError.value = t('editorJsonMustBeObject')
    } else {
      jsonError.value = ''
    }
  } catch (e) {
    jsonError.value = String(e instanceof Error ? e.message : e)
  }
})

async function onSave() {
  error.value = ''
  let config: Record<string, unknown> | null
  if (mode.value === 'json') {
    try {
      config = JSON.parse(jsonText.value)
    } catch {
      error.value = t('editorJsonInvalid')
      return
    }
  } else {
    config = mcpFormToConfig(form.value)
    if (!config) {
      error.value = t('editorMcpIncomplete')
      return
    }
  }
  const scope = form.value.scope
  if (scope !== 'user' && !appStore.cwd) {
    error.value = t('editorScopeNeedsProject')
    return
  }
  busy.value = true
  try {
    await saveMcpServer({
      scope,
      name: form.value.name.trim(),
      config: config as Record<string, unknown>,
      projectPath: scope !== 'user' ? appStore.cwd : undefined,
      originalName: isEdit.value && form.value.name.trim() !== originalName.value ? originalName.value : undefined,
      originalScope: isEdit.value && scope !== originalScope.value ? originalScope.value : undefined,
    })
    emit('saved')
    emit('update:visible', false)
  } catch (e) {
    error.value = String(e)
  } finally {
    busy.value = false
  }
}

async function onDelete() {
  if (!props.server) return
  if (!(await uiConfirm(t('editorMcpDeleteConfirm', { name: originalName.value }), { danger: true }))) return
  busy.value = true
  error.value = ''
  try {
    await deleteMcpServer(
      props.server.sourceType,
      originalName.value,
      props.server.sourceType !== 'user' ? appStore.cwd : undefined
    )
    emit('saved')
    emit('update:visible', false)
  } catch (e) {
    error.value = String(e)
  } finally {
    busy.value = false
  }
}
</script>
