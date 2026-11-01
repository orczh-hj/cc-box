import { describe, it, expect, vi, beforeEach } from 'vitest'
import { mount, flushPromises } from '@vue/test-utils'
import { setActivePinia, createPinia } from 'pinia'

const mockT = vi.fn((key: string) => key)
vi.mock('vue-i18n', () => ({
  useI18n: () => ({ t: mockT }),
  createI18n: () => ({ global: { locale: { value: 'en' } } }),
}))

const mockSave = vi.fn()
const mockGetSource = vi.fn()
vi.mock('@/api/tauri', () => ({
  getSkillSource: (...a: unknown[]) => mockGetSource(...a),
  saveSkill: (...a: unknown[]) => mockSave(...a),
  deleteSkill: vi.fn().mockResolvedValue(undefined),
  openInFileManager: vi.fn().mockResolvedValue(undefined),
}))

vi.mock('@/api/transport', () => ({
  transport: { listen: vi.fn().mockResolvedValue(() => {}) },
}))

vi.mock('@/api/platform', () => ({
  isTauriEnv: false,
  copyText: vi.fn().mockResolvedValue(undefined),
}))

import SkillEditor from '@/components/editor/SkillEditor.vue'
import type { SkillInfo } from '@/types'

/** 用户粘贴样例：含多个未知 frontmatter 键（when_to_use / argument-hint / allowed-tools） */
const PASTED_SOURCE = [
  '---',
  'name: bib-search-citation',
  'description: "Search and cite from local BibTeX libraries."',
  'when_to_use: "Trigger on prompts like \\"search my references.bib\\"."',
  'metadata: ""',
  'argument-hint: "--bib library.bib --query QUERY"',
  'allowed-tools: "Read, Bash(uv *)"'.replace('uv *', 'uv *'),
  '---',
  '',
  '# Bib Search Citation',
  '',
  'Body text here.',
].join('\n')

const skill = { name: 'bib-search-citation', description: 'd', sourceType: 'user', enabled: true } as unknown as SkillInfo

function mountEditor(skillProp: SkillInfo | null) {
  return mount(SkillEditor, {
    props: { visible: false, skill: skillProp },
    global: { stubs: { Teleport: true, Transition: false } },
  })
}

/** 切到源码模式并写入文本 */
async function gotoSource(w: ReturnType<typeof mountEditor>, text: string) {
  const ms = w.findComponent({ name: 'ModeSwitch' })
  ms.vm.$emit('update:modelValue', 'source')
  await flushPromises()
  const ta = w.findAll('textarea').pop()!
  await ta.setValue(text)
  await flushPromises()
}

describe('SkillEditor 粘贴源码保存', () => {
  beforeEach(() => {
    setActivePinia(createPinia())
    mockSave.mockReset().mockResolvedValue(undefined)
    mockGetSource.mockReset()
  })

  // 源码模式保存：整文写入（asSource=true），未知字段不丢
  it('Paste_SourceModeSavesFullSource_1', async () => {
    mockGetSource.mockResolvedValue([PASTED_SOURCE, 'C:/x/SKILL.md'])
    const w = mountEditor(skill)
    await w.setProps({ visible: true })
    await flushPromises()
    await gotoSource(w, PASTED_SOURCE)
    // 触发失焦提交（点保存前浏览器会先 blur）
    await w.findAll('textarea').pop()!.trigger('blur')
    // 点保存
    const saveBtn = w.findAll('button').find(b => b.text() === 'save')!
    await saveBtn.trigger('click')
    await flushPromises()
    expect(mockSave).toHaveBeenCalledTimes(1)
    const req = mockSave.mock.calls[0][0]
    expect(req.asSource).toBe(true)
    expect(req.content).toBe(PASTED_SOURCE)
    expect(req.name).toBe('bib-search-citation')
  })

  // 表单模式保存：用 buildSkillSource 拼全文（含未知键），不丢 when_to_use 等
  it('Paste_FormModeSavesWithExtras_2', async () => {
    mockGetSource.mockResolvedValue([PASTED_SOURCE, 'C:/x/SKILL.md'])
    const w = mountEditor(skill)
    await w.setProps({ visible: true })
    await flushPromises()
    // 停留在表单模式，直接保存
    const saveBtn = w.findAll('button').find(b => b.text() === 'save')!
    await saveBtn.trigger('click')
    await flushPromises()
    expect(mockSave).toHaveBeenCalledTimes(1)
    const req = mockSave.mock.calls[0][0]
    expect(req.asSource).toBe(true)
    expect(req.content).toContain('when_to_use:')
    expect(req.content).toContain('argument-hint:')
    expect(req.content).toContain('allowed-tools:')
    // 保存后再读回 frontmatter 应能解析出全部未知键（round-trip）
    const { parseFrontmatter } = await import('@/components/editor/model')
    const fm = parseFrontmatter(req.content)!
    expect(fm.fields.name).toBe('bib-search-citation')
    expect(fm.fields['when_to_use']).toContain('search my references.bib')
    expect(fm.fields['argument-hint']).toBe('--bib library.bib --query QUERY')
    expect(fm.fields['allowed-tools']).toBe('Read, Bash(uv *)')
  })

  // 源码 frontmatter 的 name 同步进表单（blur 提交后）
  it('Paste_SourceNameSyncedToForm_3', async () => {
    mockGetSource.mockResolvedValue([PASTED_SOURCE, 'C:/x/SKILL.md'])
    // 新建场景：表单 name 初始为空
    const w = mountEditor(null)
    await w.setProps({ visible: true })
    await flushPromises()
    await gotoSource(w, PASTED_SOURCE)
    await w.findAll('textarea').pop()!.trigger('blur')
    // 切回表单模式，name 输入框应显示 frontmatter 的 name
    const ms2 = w.findComponent({ name: 'ModeSwitch' })
    ms2.vm.$emit('update:modelValue', 'form')
    await flushPromises()
    const nameInput = w.find('input.editor-input')
    expect((nameInput.element as HTMLInputElement).value).toBe('bib-search-citation')
  })
})
