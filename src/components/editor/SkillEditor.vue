<template>
  <EditorModal
    :visible="visible"
    :title="isEdit ? t('editorSkillEditTitle', { name: originalName }) : t('editorSkillNewTitle')"
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
      <button v-if="skillPath" class="editor-btn ghost" @click="openPath">{{ openLabel }}</button>
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
        <textarea v-model="form.description" class="editor-textarea" rows="3" style="min-height: 0" :placeholder="t('editorSkillDescPlaceholder')"></textarea>
        <span class="editor-hint">{{ t('editorSkillDescHint') }}</span>
      </div>
      <div class="editor-field">
        <span class="editor-label">{{ t('editorSkillContent') }}<span class="editor-req">*</span></span>
        <textarea v-model="form.content" class="editor-textarea" rows="12" spellcheck="false"></textarea>
      </div>
    </div>

    <div v-else class="editor-field">
      <span class="editor-label">SKILL.md</span>
      <!-- 错误提示放 textarea 上方：文本超长时下方提示在滚动视口外看不见 -->
      <div v-if="sourceError" class="editor-error">{{ sourceError }}</div>
      <textarea
        v-model="sourceText"
        class="editor-textarea"
        rows="16"
        spellcheck="false"
        @blur="onSourceBlur"
      ></textarea>
      <span class="editor-hint">{{ t('editorSkillSourceHint') }}</span>
    </div>

    <div v-if="error" class="editor-error">{{ error }}</div>
  </EditorModal>
</template>

<script setup lang="ts">
import { computed, ref, watch } from 'vue'
import { useI18n } from 'vue-i18n'
import { useAppStore } from '@/stores/app'
import {
  deleteSkill,
  getSkillSource,
  saveSkill,
  openInFileManager,
  type SkillInfo,
} from '@/api/tauri'
import { copyText, isTauriEnv } from '@/api/platform'
import { uiConfirm } from '@/composables/useConfirm'
import { useEditorFormSync } from '@/composables/useEditorFormSync'
import EditorModal from './EditorModal.vue'
import ModeSwitch from './ModeSwitch.vue'
import {
  parseFrontmatter,
  buildSkillSource,
  pickExtras,
  skillKnownKeys,
} from './model'

const { t } = useI18n()
const appStore = useAppStore()

const props = defineProps<{
  visible: boolean
  /** null = 新建 */
  skill: SkillInfo | null
}>()

const emit = defineEmits<{
  'update:visible': [v: boolean]
  saved: []
}>()

const hasProject = computed(() => !!appStore.cwd)
const isEdit = computed(() => !!props.skill)
const skillPath = ref('')

const mode = ref<'form' | 'source'>('form')
const modeOptions = [
  { value: 'form', label: t('editorModeForm') },
  { value: 'source', label: t('editorModeSource') },
]

const form = ref({
  scope: 'user' as 'user' | 'project',
  name: '',
  description: '',
  content: '',
})
const originalName = ref('')
/** 编辑打开时的作用域（判断是否跨作用域移动） */
const originalScope = ref<'user' | 'project'>('user')
const loadedSource = ref('')
/** 源码模式文本（与 form 双向同步，见 mode watch） */
const sourceText = ref('')
const error = ref('')
const busy = ref(false)
/** 脏检查基线：整个 form（或源码文本）序列化 */
let baseline = ''

const dirty = computed(() =>
  mode.value === 'source' ? sourceText.value !== baseline : JSON.stringify(form.value) !== baseline
)

watch(
  () => props.visible,
  async v => {
    if (!v) return
    error.value = ''
    busy.value = false
    mode.value = 'form'
    skillPath.value = ''
    if (props.skill) {
      originalName.value = props.skill.name
      const scope = props.skill.sourceType === 'project' ? 'project' : 'user'
      originalScope.value = scope
      form.value = { scope, name: props.skill.name, description: props.skill.description ?? '', content: '' }
      try {
        const [source, path] = await getSkillSource(scope, props.skill.name, scope === 'project' ? appStore.cwd : undefined)
        loadedSource.value = source
        skillPath.value = path
        // 源码模式全量展示；表单模式剥离 frontmatter（否则保存时会嵌套两层）
        const fm = parseFrontmatter(source)
        form.value.content = fm ? fm.body : source
        const fmDesc = fm?.fields.description
        if (fmDesc !== undefined && fmDesc !== '') form.value.description = fmDesc
        extras.value = fm ? pickExtras(fm.fields, skillKnownKeys()) : {}
      } catch (e) {
        error.value = String(e)
      }
    } else {
      originalName.value = ''
      loadedSource.value = ''
      form.value = { scope: 'user', name: '', description: '', content: '' }
      extras.value = {}
    }
    sourceError.value = ''
    regenerateSource()
    baseline = JSON.stringify(form.value)
  }
)

// 表单 ↔ 源码**实时**双向同步：任一侧编辑，另一侧立即跟上（切换模式时无需再转换）。
// 打开文件时解析出的未知 frontmatter 键存 extras，生成源码时原样带回。
const extras = ref<Record<string, string>>({})
/** 源码结构非法时的提示（此时不回填表单） */
const sourceError = ref('')

const { regenerate: regenerateSource, commitFromText } = useEditorFormSync({
  form,
  text: sourceText,
  isFormMode: () => mode.value === 'form',
  build: f => buildSkillSource(f, extras.value),
  parse: (text, current) => {
    const fm = parseFrontmatter(text)
    if (!fm) return null
    extras.value = pickExtras(fm.fields, skillKnownKeys())
    // name 也从源码同步（frontmatter name 与文件名一致是后端约束）
    return {
      ...current,
      name: fm.fields.name ?? current.name,
      description: fm.fields.description ?? '',
      content: fm.body,
    }
  },
})

/** 源码 → 表单：失焦时校验并转换，非法则提示且保留上一次有效值 */
function onSourceBlur() {
  if (mode.value !== 'source') return
  sourceError.value = commitFromText() ? '' : t('editorSourceInvalid')
}

// 切换模式：两侧已同步，只需重置脏检查基线
watch(mode, m => {
  baseline = m === 'source' ? sourceText.value : JSON.stringify(form.value)
})

async function onSave() {
  error.value = ''
  // 先定稿全文（校验之前）：源码模式可能未经 blur，name 等字段要靠提交同步进表单
  const asSource = mode.value === 'source'
  let fullSource: string
  if (asSource) {
    if (!commitFromText()) {
      // 源码结构非法：拒绝保存，避免写出无法解析的 SKILL.md
      sourceError.value = t('editorSourceInvalid')
      return
    }
    fullSource = sourceText.value
  } else {
    fullSource = buildSkillSource(form.value, extras.value)
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
    error.value = t('editorSkillDescRequired')
    return
  }
  if (!asSource && !form.value.content.trim()) {
    error.value = t('editorContentRequired')
    return
  }
  busy.value = true
  try {
    // 一律整文写入（asSource=true）：未知 frontmatter 键（when_to_use 等）经
    // extras 保留在 fullSource 里，走表单重建路径会全部丢失
    await saveSkill({
      scope: form.value.scope,
      name,
      projectPath: form.value.scope === 'project' ? appStore.cwd : undefined,
      originalName: isEdit.value && name !== originalName.value ? originalName.value : undefined,
      originalScope:
        isEdit.value && form.value.scope !== originalScope.value ? originalScope.value : undefined,
      asSource: true,
      description: form.value.description.trim(),
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
  if (!props.skill) return
  if (!(await uiConfirm(t('editorSkillDeleteConfirm', { name: originalName.value }), { danger: true }))) return
  busy.value = true
  error.value = ''
  try {
    // 删除按打开时的作用域定位（用户可能已改选作用域但尚未保存）
    await deleteSkill(originalScope.value, originalName.value, originalScope.value === 'project' ? appStore.cwd : undefined)
    emit('saved')
    emit('update:visible', false)
  } catch (e) {
    error.value = String(e)
  } finally {
    busy.value = false
  }
}

/** 可选入口：打开 skill 目录（桌面）；远程降级为复制路径 */
const openLabel = computed(() => (isTauriEnv ? t('editorOpenDirectory') : t('editorCopyPath')))

async function openPath() {
  if (!skillPath.value) return
  try {
    if (isTauriEnv) {
      const dir = skillPath.value.replace(/[\\/]+SKILL\.md$/i, '')
      await openInFileManager(dir)
    } else {
      await copyText(skillPath.value)
    }
  } catch (e) {
    error.value = String(e)
  }
}
</script>
