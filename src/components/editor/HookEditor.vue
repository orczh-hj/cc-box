<template>
  <EditorModal
    :visible="visible"
    :title="isEdit ? t('editorHookEditTitle') : t('editorHookNewTitle')"
    :busy="busy"
    :dirty="dirty"
    :can-save="canSave"
    :deletable="isEdit"
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
          <span class="editor-label">{{ t('editorHookEvent') }}<span class="editor-req">*</span></span>
          <select v-model="form.event" class="editor-select" :disabled="isEdit">
            <option v-for="e in HOOK_EVENTS" :key="e" :value="e">{{ e }}</option>
          </select>
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
        <span class="editor-label">{{ t('editorHookType') }}</span>
        <select v-model="form.type" class="editor-select">
          <option v-for="ty in HOOK_TYPES" :key="ty" :value="ty">{{ t('editorHookType_' + ty) }}</option>
        </select>
      </div>
      <div class="editor-field">
        <span class="editor-label">{{ t('editorHookMatcher') }}</span>
        <input v-model="form.matcher" class="editor-input" spellcheck="false" placeholder="Write, Edit, Bash" />
        <span class="editor-hint">{{ t('editorHookMatcherHint') }}</span>
      </div>

      <template v-if="form.type === 'command'">
        <div class="editor-field">
          <span class="editor-label">{{ t('editorHookCommand') }}<span class="editor-req">*</span></span>
          <textarea v-model="form.command" class="editor-textarea" rows="3" style="min-height: 0" spellcheck="false" placeholder="echo 'Hello from hook'"></textarea>
        </div>
      </template>

      <template v-else-if="form.type === 'http'">
        <div class="editor-field">
          <span class="editor-label">URL<span class="editor-req">*</span></span>
          <input v-model="form.url" class="editor-input" spellcheck="false" placeholder="http://localhost:8080/hooks/tool-use" />
          <span class="editor-hint">{{ t('editorHookHttpHint') }}</span>
        </div>
        <div class="editor-field">
          <span class="editor-label">{{ t('editorMcpHeaders') }}</span>
          <div v-for="(row, i) in form.headers" :key="i" class="editor-kv-row">
            <input v-model="row.key" class="editor-input" :placeholder="t('editorKvKey')" spellcheck="false" />
            <input v-model="row.value" class="editor-input" :placeholder="t('editorKvValue')" spellcheck="false" />
            <button class="editor-kv-remove" @click="form.headers.splice(i, 1)" :title="t('remove')">×</button>
          </div>
          <button class="editor-kv-add" @click="form.headers.push({ key: '', value: '' })">＋ {{ t('editorKvAdd') }}</button>
          <span class="editor-hint">{{ t('editorHookHeadersHint') }}</span>
        </div>
      </template>

      <template v-else-if="form.type === 'prompt' || form.type === 'agent'">
        <div class="editor-field">
          <span class="editor-label">{{ t('editorHookPrompt') }}<span class="editor-req">*</span></span>
          <textarea v-model="form.prompt" class="editor-textarea" rows="4" style="min-height: 0" spellcheck="false" :placeholder="t('editorHookPromptPlaceholder')"></textarea>
          <span class="editor-hint">{{ form.type === 'agent' ? t('editorHookAgentHint') : t('editorHookPromptHint') }}</span>
        </div>
        <div v-if="form.type === 'prompt'" class="editor-field" style="max-width: 200px">
          <span class="editor-label">{{ t('editorHookModel') }}</span>
          <input v-model="form.model" class="editor-input" spellcheck="false" placeholder="haiku" />
        </div>
      </template>

      <template v-else>
        <div class="editor-field">
          <span class="editor-hint">{{ t('editorHookMcpToolHint') }}</span>
        </div>
      </template>

      <div class="editor-field" style="max-width: 200px">
        <span class="editor-label">{{ t('editorHookTimeout') }}</span>
        <input v-model="form.timeout" class="editor-input" type="number" min="1" />
        <span class="editor-hint">{{ t('editorHookTimeoutHint') }}</span>
      </div>
    </div>

    <div v-else class="editor-field">
      <span class="editor-label">JSON</span>
      <textarea v-model="jsonText" class="editor-textarea" rows="12" spellcheck="false"></textarea>
      <span class="editor-hint">{{ t('editorHookJsonHint') }}</span>
      <span v-if="jsonError" class="editor-error" style="margin: 6px 0 0">{{ jsonError }}</span>
    </div>

    <div v-if="error" class="editor-error">{{ error }}</div>
  </EditorModal>
</template>

<script setup lang="ts">
import { computed, ref, watch } from 'vue'
import { useI18n } from 'vue-i18n'
import { useAppStore } from '@/stores/app'
import {
  deleteHook,
  saveHook,
  type HookListItem,
} from '@/api/tauri'
import { uiConfirm } from '@/composables/useConfirm'
import EditorModal from './EditorModal.vue'
import ModeSwitch from './ModeSwitch.vue'
import {
  HOOK_EVENTS,
  HOOK_TYPES,
  buildHookPayload,
  hookEntryToForm,
  hookFormError,
  hookFormToObject,
  type HookForm,
} from './model'

const { t } = useI18n()
const appStore = useAppStore()

const props = defineProps<{
  visible: boolean
  /** null = 新建 */
  hook: HookListItem | null
}>()

const emit = defineEmits<{
  'update:visible': [v: boolean]
  saved: []
}>()

const hasProject = computed(() => !!appStore.cwd)
const isEdit = computed(() => !!props.hook)

const mode = ref<'form' | 'json'>('form')
const modeOptions = [
  { value: 'form', label: t('editorModeForm') },
  { value: 'json', label: 'JSON' },
]

const emptyForm = (): HookForm => ({
  scope: 'user',
  event: 'PreToolUse',
  type: 'command',
  matcher: '',
  command: '',
  url: '',
  headers: [],
  prompt: '',
  model: '',
  timeout: '',
})

const form = ref<HookForm>(emptyForm())
const original = ref<{ event: string; matcher?: string; key: string } | undefined>(undefined)
/** 编辑打开时的作用域（跨作用域保存 = 从旧 settings 移除后写入新作用域） */
const originalScope = ref<HookForm['scope']>('user')
const jsonText = ref('')
const jsonError = ref('')
const error = ref('')
const busy = ref(false)
let baseline = ''

const dirty = computed(() => (mode.value === 'json' ? jsonText.value : snapshot()) !== baseline)

const canSave = computed(() => (mode.value === 'json' ? !jsonError.value : true))

function snapshot(): string {
  return JSON.stringify(form.value)
}

watch(jsonText, text => {
  if (mode.value !== 'json') return
  const parsed = hookEntryToForm(text, { scope: form.value.scope, event: form.value.event })
  jsonError.value = parsed ? '' : t('editorHookJsonInvalid')
  if (parsed) {
    form.value = { ...parsed, scope: form.value.scope, event: form.value.event }
  }
})

watch(
  () => props.visible,
  v => {
    if (!v) return
    error.value = ''
    busy.value = false
    mode.value = 'form'
    jsonError.value = ''
    if (props.hook) {
      form.value = formFromItem(props.hook)
      originalScope.value = scopeOf(props.hook)
      original.value = {
        event: props.hook.event,
        matcher: props.hook.matcher,
        key: props.hook.key,
      }
      // mcp_tool / 未识别类型：表单无对应字段，直接进 JSON 模式
      if (form.value.type === 'mcp_tool') mode.value = 'json'
    } else {
      form.value = emptyForm()
      original.value = undefined
    }
    jsonText.value = JSON.stringify(entryJson(form.value), null, 2)
    baseline = snapshot()
  }
)

/** 列表条目 → 表单：摘要字段（command/url/prompt 回退链）按类型归位 */
function formFromItem(h: HookListItem): HookForm {
  const type = (HOOK_TYPES as string[]).includes(h.hookType ?? 'command')
    ? ((h.hookType ?? 'command') as HookForm['type'])
    : 'mcp_tool'
  const summary = h.command ?? ''
  return {
    scope: scopeOf(h),
    event: h.event,
    type,
    matcher: h.matcher ?? '',
    command: type === 'command' ? summary : '',
    url: type === 'http' ? summary : '',
    headers: [],
    prompt: type === 'prompt' || type === 'agent' ? summary : '',
    model: '',
    timeout: h.timeout ? String(h.timeout) : '',
  }
}

function scopeOf(h: HookListItem): HookForm['scope'] {
  // 停用条目按备份记录的原始 scope（source.type 已还原为原 scope，两者一致）
  const s = h.scope ?? h.source.type
  return s === 'project' || s === 'local' ? s : 'user'
}

function entryJson(f: HookForm): Record<string, unknown> {
  const entry: Record<string, unknown> = { hooks: [hookFormToObject(f)] }
  if (f.matcher.trim()) entry.matcher = f.matcher.trim()
  return entry
}

// 表单 → JSON 同步
watch(mode, m => {
  if (m === 'json') {
    jsonError.value = ''
    jsonText.value = JSON.stringify(entryJson(form.value), null, 2)
  }
})

async function onSave() {
  error.value = ''
  if (mode.value === 'json' && jsonError.value) return
  const errKey = hookFormError(form.value)
  if (errKey) {
    error.value = t(errKey)
    return
  }
  const scope = form.value.scope
  if (scope !== 'user' && !appStore.cwd) {
    error.value = t('editorScopeNeedsProject')
    return
  }
  busy.value = true
  try {
    await saveHook(
      buildHookPayload(
        form.value,
        appStore.cwd || undefined,
        original.value,
        isEdit.value && form.value.scope !== originalScope.value ? originalScope.value : undefined
      )
    )
    emit('saved')
    emit('update:visible', false)
  } catch (e) {
    error.value = String(e)
  } finally {
    busy.value = false
  }
}

async function onDelete() {
  const h = props.hook
  if (!h) return
  if (!(await uiConfirm(t('editorHookDeleteConfirm', { event: h.event }), { danger: true }))) return
  const scope = scopeOf(h)
  busy.value = true
  error.value = ''
  try {
    await deleteHook(scope, h.event, h.key, scope !== 'user' ? appStore.cwd : undefined)
    emit('saved')
    emit('update:visible', false)
  } catch (e) {
    error.value = String(e)
  } finally {
    busy.value = false
  }
}
</script>
