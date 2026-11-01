/**
 * 编辑器「表单 ↔ 源码/JSON」同步（Skill / Agent / MCP / Hook 四个编辑器共用）。
 *
 * 规则：
 * - **表单 → 文本**：表单是结构化的，随时可生成合法文本，逐键实时同步
 * - **文本 → 表单**：结构可能写坏，**只在失焦时**解析（逐键解析会让半成品
 *   输入反复报错）；解析失败返回 null，调用方提示并保留上一次有效值
 * - 两个方向都加 syncing 守卫，避免 watcher 回环
 */
import { watch, type Ref } from 'vue'

export interface EditorFormSyncOptions<F> {
  form: Ref<F>
  text: Ref<string>
  /** 当前是否处于表单模式（决定表单改动是否实时生成文本） */
  isFormMode: () => boolean
  /** 表单 → 文本 */
  build: (form: F) => string
  /** 文本 → 表单；结构非法返回 null */
  parse: (text: string, current: F) => F | null
}

export function useEditorFormSync<F>(opts: EditorFormSyncOptions<F>) {
  /** 程序化写入时置位，watcher 见到即跳过 */
  let syncing = false

  /** 由表单重新生成文本（打开编辑器 / 表单改动时调用） */
  function regenerate(): void {
    syncing = true
    opts.text.value = opts.build(opts.form.value)
    syncing = false
  }

  watch(
    opts.form,
    () => {
      if (syncing || !opts.isFormMode()) return
      regenerate()
    },
    { deep: true }
  )

  /** 失焦时调用：解析成功则回填表单并返回 true，结构非法返回 false */
  function commitFromText(): boolean {
    if (syncing) return true
    const next = opts.parse(opts.text.value, opts.form.value)
    if (!next) return false
    syncing = true
    opts.form.value = next
    syncing = false
    return true
  }

  return { regenerate, commitFromText }
}
