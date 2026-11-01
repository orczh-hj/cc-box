import { describe, expect, it } from 'vitest'
import { ref, nextTick } from 'vue'
import { useEditorFormSync } from '@/composables/useEditorFormSync'

interface Form {
  title: string
}

function setup(initialMode: 'form' | 'text') {
  const form = ref<Form>({ title: 'a' })
  const text = ref('a')
  const mode = ref<'form' | 'text'>(initialMode)
  const sync = useEditorFormSync({
    form,
    text,
    isFormMode: () => mode.value === 'form',
    build: f => f.title,
    // 空标题视为结构非法
    parse: (t, current) => (t.trim() ? { ...current, title: t } : null),
  })
  return { form, text, mode, ...sync }
}

describe('useEditorFormSync', () => {
  // 表单模式：表单改动实时生成文本
  it('EditorSync_FormRegeneratesText_1', async () => {
    const s = setup('form')
    s.form.value = { title: 'b' }
    await nextTick()
    expect(s.text.value).toBe('b')
  })

  // 文本模式：表单改动不覆盖用户正在编辑的文本
  it('EditorSync_TextModeNoOverwrite_2', async () => {
    const s = setup('text')
    s.form.value = { title: 'b' }
    await nextTick()
    expect(s.text.value).toBe('a')
  })

  // 失焦提交：合法 → 回填表单；非法 → 返回 false 且表单保持不变
  it('EditorSync_CommitValidAndInvalid_3', () => {
    const s = setup('text')
    s.text.value = 'new'
    expect(s.commitFromText()).toBe(true)
    expect(s.form.value.title).toBe('new')

    s.text.value = '   '
    expect(s.commitFromText()).toBe(false)
    expect(s.form.value.title).toBe('new')
  })

  // 提交后表单 watcher 不应反向覆盖文本（syncing 守卫）
  it('EditorSync_NoEchoAfterCommit_4', async () => {
    const s = setup('text')
    s.text.value = 'x'
    s.commitFromText()
    await nextTick()
    expect(s.text.value).toBe('x')
  })
})
