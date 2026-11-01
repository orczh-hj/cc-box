<template>
  <EditorModal
    :visible="visible"
    :title="isEdit ? t('editorAgentEditTitle', { name: originalName }) : t('editorAgentNewTitle')"
    :busy="busy"
    :dirty="dirty"
    :deletable="isEdit"
    @close="$emit('update:visible', false)"
    @save="onSave"
    @delete="onDelete"
  >
    <template #header-extra>
      <ModeSwitch v-model="mode" :modes="modeOptions" />
    </template>

    <template #footer-extra>
      <button v-if="filePath" class="editor-btn ghost" @click="openPath">{{ openLabel }}</button>
    </template>

    <div v-if="mode === 'form'">
      <div class="editor-field-row">
        <div class="editor-field">
          <span class="editor-label">{{ t('editorName') }}<span class="editor-req">*</span></span>
          <input v-model="form.name" class="editor-input" spellcheck="false" />
        </div>
        <div class="editor-field" style="max-width: 140px">
          <span class="editor-label">{{ t('editorScope') }}</span>
          <select v-model="form.scope" class="editor-select">
            <option value="user">{{ t('scopeUser') }}</option>
            <option v-if="hasProject || form.scope === 'project'" value="project">{{ t('scopeProject') }}</option>
          </select>
          <span v-if="isEdit && form.scope !== originalScope" class="editor-hint">{{ t('editorScopeMoveHint') }}</span>
        </div>
      </div>
      <div class="editor-field">
        <span class="editor-label">{{ t('editorDescription') }}<span class="editor-req">*</span></span>
        <textarea v-model="form.description" class="editor-textarea" rows="3" style="min-height: 0" :placeholder="t('editorAgentDescPlaceholder')"></textarea>
        <span class="editor-hint">{{ t('editorAgentDescHint') }}</span>
      </div>
      <div class="editor-field-row">
        <div class="editor-field">
          <span class="editor-label">{{ t('editorAgentTools') }}</span>
          <input v-model="form.tools" class="editor-input" spellcheck="false" placeholder="Read, Glob, Grep" />
        </div>
        <div class="editor-field">
          <span class="editor-label">{{ t('editorAgentModel') }}</span>
          <input v-model="form.model" class="editor-input" spellcheck="false" placeholder="sonnet" />
        </div>
      </div>
      <div class="editor-field">
        <span class="editor-label">{{ t('editorAgentContent') }}<span class="editor-req">*</span></span>
        <textarea v-model="form.content" class="editor-textarea" rows="10" spellcheck="false"></textarea>
      </div>
    </div>

    <div v-else class="editor-field">
      <span class="editor-label">{{ originalName }}.md</span>
      <!-- 错误提示放 textarea 上方：文本超长时下方提示在滚动视口外看不见 -->
      <div v-if="sourceError" class="editor-error">{{ sourceError }}</div>
      <textarea
        v-model="sourceText"
        class="editor-textarea"
        rows="16"
        spellcheck="false"
        @blur="onSourceBlur"
      ></textarea>
      <span class="editor-hint">{{ t('editorAgentSourceHint') }}</span>
    </div>

    <div v-if="error" class="editor-error">{{ error }}</div>
  </EditorModal>
</template>

<script setup lang="ts">
import { computed, ref, watch } from 'vue'
import { useI18n } from 'vue-i18n'
import { useAppStore } from '@/stores/app'
import {
  deleteAgent,
  getAgentSource,
  saveAgent,
  revealInFileManager,
  type AgentInfo,
} from '@/api/tauri'
import { copyText, isTauriEnv } from '@/api/platform'
import { uiConfirm } from '@/composables/useConfirm'
import { useEditorFormSync } from '@/composables/useEditorFormSync'
import EditorModal from './EditorModal.vue'
import ModeSwitch from './ModeSwitch.vue'
import {
  parseFrontmatter,
  buildAgentSource,
  pickExtras,
  agentKnownKeys,
} from './model'

const { t } = useI18n()
const appStore = useAppStore()

const props = defineProps<{
  visible: boolean
  /** null = 新建 */
  agent: AgentInfo | null
}>()

const emit = defineEmits<{
  'update:visible': [v: boolean]
  saved: []
}>()

const hasProject = computed(() => !!appStore.cwd)
const isEdit = computed(() => !!props.agent)
const filePath = ref('')

const mode = ref<'form' | 'source'>('form')
const modeOptions = [
  { value: 'form', label: t('editorModeForm') },
  { value: 'source', label: t('editorModeSource') },
]

const form = ref({
  scope: 'user' as 'user' | 'project',
  name: '',
  description: '',
  tools: '',
  model: '',
  content: '',
})
const originalName = ref('')
/** 编辑打开时的作用域（判断是否跨作用域移动） */
const originalScope = ref<'user' | 'project'>('user')
const loadedSource = ref('')
/** 源码模式文本（与 form 双向同步） */
const sourceText = ref('')
const error = ref('')
const busy = ref(false)
/** 脏检查基线：整个 form（或源码文本）序列化 */
let baseline = ''

const dirty = computed(() =>
  mode.value === 'source' ? sourceText.value !== baseline : JSON.stringify(form.value) !== baseline
)

const openLabel = computed(() =>
  isTauriEnv ? t('editorOpenDirectory') : t('editorCopyPath')
)

watch(
  () => props.visible,
  async v => {
    if (!v) return
    error.value = ''
    busy.value = false
    mode.value = 'form'
    filePath.value = ''
    if (props.agent) {
      originalName.value = props.agent.name
      const scope = props.agent.sourceType === 'project' ? 'project' : 'user'
      originalScope.value = scope
      try {
        const [source, path] = await getAgentSource(scope, props.agent.name, scope === 'project' ? appStore.cwd : undefined)
        loadedSource.value = source
        filePath.value = path
        const fm = parseFrontmatter(source)
        form.value = {
          scope,
          name: props.agent.name,
          description: fm?.fields.description ?? props.agent.description ?? '',
          tools: fm?.fields.tools ?? '',
          model: fm?.fields.model ?? props.agent.model ?? '',
          // 表单模式只持正文（保存时后端重新拼 frontmatter，剥离防嵌套）
          content: fm ? fm.body : source,
        }
        extras.value = fm ? pickExtras(fm.fields, agentKnownKeys()) : {}
      } catch (e) {
        error.value = String(e)
      }
    } else {
      originalName.value = ''
      loadedSource.value = ''
      form.value = { scope: 'user', name: '', description: '', tools: '', model: '', content: '' }
      extras.value = {}
    }
    sourceError.value = ''
    regenerateSource()
    baseline = JSON.stringify(form.value)
  }
)

// 表单 ↔ 源码实时双向同步（同一时间只有一侧可见，另一侧随时跟上）
const extras = ref<Record<string, string>>({})
/** 源码结构非法时的提示（此时不回填表单） */
const sourceError = ref('')

const { regenerate: regenerateSource, commitFromText } = useEditorFormSync({
  form,
  text: sourceText,
  isFormMode: () => mode.value === 'form',
  build: f => buildAgentSource(f, extras.value),
  parse: (text, current) => {
    const fm = parseFrontmatter(text)
    if (!fm) return null
    extras.value = pickExtras(fm.fields, agentKnownKeys())
    // name 也从源码同步（frontmatter name 与文件名一致是后端约束）
    return {
      ...current,
      name: fm.fields.name ?? current.name,
      description: fm.fields.description ?? '',
      tools: fm.fields.tools ?? '',
      model: fm.fields.model ?? '',
      content: fm.body,
    }
  },
})

/** 源码 → 表单：失焦时校验并转换，非法则提示且保留上一次有效值 */
function onSourceBlur() {
  if (mode.value !== 'source') return
  sourceError.value = commitFromText() ? '' : t('editorSourceInvalid')
}

// 切换模式：两侧已同步，只重置脏检查基线
watch(mode, m => {
  baseline = m === 'source' ? sourceText.value : JSON.stringify(form.value)
})

/** 可选入口：打开文件（桌面 = 系统默认应用）；远程降级为复制路径 */
async function openPath() {
  if (!filePath.value) return
  try {
    if (isTauriEnv) {
      await revealInFileManager(filePath.value)
    } else {
      await copyText(filePath.value)
    }
  } catch (e) {
    error.value = String(e)
  }
}

async function onSave() {
  error.value = ''
  // 先定稿全文（校验之前）：源码模式可能未经 blur，name 等字段要靠提交同步进表单
  const asSource = mode.value === 'source'
  let fullSource: string
  if (asSource) {
    if (!commitFromText()) {
      // 源码结构非法：拒绝保存，避免写出无法解析的 .md
      sourceError.value = t('editorSourceInvalid')
      return
    }
    fullSource = sourceText.value
  } else {
    fullSource = buildAgentSource(form.value, extras.value)
  }
  const name = form.value.name.trim()
  if (!name) {
    error.value = t('editorNameRequired')
    return
  }
  if (form.value.scope === 'project' && !appStore.cwd) {
    error.value = t('editorScopeNeedsProject')
    return
  }
  if (mode.value === 'form' && !form.value.description.trim()) {
    error.value = t('editorAgentDescRequired')
    return
  }
  if (!asSource && !form.value.content.trim()) {
    error.value = t('editorContentRequired')
    return
  }
  busy.value = true
  try {
    // 一律整文写入（asSource=true）：未知 frontmatter 键经 extras 保留在 fullSource 里，
    // 走表单重建路径会全部丢失
    await saveAgent({
      scope: form.value.scope,
      name,
      projectPath: form.value.scope === 'project' ? appStore.cwd : undefined,
      originalName: isEdit.value && name !== originalName.value ? originalName.value : undefined,
      originalScope:
        isEdit.value && form.value.scope !== originalScope.value ? originalScope.value : undefined,
      asSource: true,
      description: form.value.description.trim(),
      tools: form.value.tools.trim() || undefined,
      model: form.value.model.trim() || undefined,
      content: fullSource,
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
  if (!props.agent) return
  if (!(await uiConfirm(t('editorAgentDeleteConfirm', { name: originalName.value }), { danger: true }))) return
  busy.value = true
  error.value = ''
  try {
    // 删除按打开时的作用域定位（用户可能已改选作用域但尚未保存）
    await deleteAgent(originalScope.value, originalName.value, originalScope.value === 'project' ? appStore.cwd : undefined)
    emit('saved')
    emit('update:visible', false)
  } catch (e) {
    error.value = String(e)
  } finally {
    busy.value = false
  }
}
</script>
