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
  getAgentSource: (...a: unknown[]) => mockGetSource(...a),
  saveAgent: (...a: unknown[]) => mockSave(...a),
  deleteAgent: vi.fn().mockResolvedValue(undefined),
  revealInFileManager: vi.fn().mockResolvedValue(undefined),
}))

vi.mock('@/api/transport', () => ({
  transport: { listen: vi.fn().mockResolvedValue(() => {}) },
}))

vi.mock('@/api/platform', () => ({
  isTauriEnv: false,
  copyText: vi.fn().mockResolvedValue(undefined),
}))

import AgentEditor from '@/components/editor/AgentEditor.vue'
import type { AgentInfo } from '@/types'

/** 粘贴样例：含未知 frontmatter 键（memory / permission 等） */
const PASTED_SOURCE = [
  '---',
  'name: research-agent',
  'description: "Research assistant for paper review."',
  'tools: Read, Glob, Grep',
  'model: sonnet',
  'memory: user',
  'permission: "ask"',
  '---',
  '',
  'Agent body.',
].join('\n')

const agent = { name: 'research-agent', description: 'd', sourceType: 'user' } as unknown as AgentInfo

function mountEditor(agentProp: AgentInfo | null) {
  return mount(AgentEditor, {
    props: { visible: false, agent: agentProp },
    global: { stubs: { Teleport: true, Transition: false } },
  })
}

describe('AgentEditor 粘贴源码保存', () => {
  beforeEach(() => {
    setActivePinia(createPinia())
    mockSave.mockReset().mockResolvedValue(undefined)
    mockGetSource.mockReset()
  })

  // 源码模式保存：整文写入，未知字段不丢
  it('AgentPaste_SourceModeSavesFullSource_1', async () => {
    mockGetSource.mockResolvedValue([PASTED_SOURCE, 'C:/x/research-agent.md'])
    const w = mountEditor(agent)
    await w.setProps({ visible: true })
    await flushPromises()
    const ms = w.findComponent({ name: 'ModeSwitch' })
    ms.vm.$emit('update:modelValue', 'source')
    await flushPromises()
    const ta = w.findAll('textarea').pop()!
    await ta.setValue(PASTED_SOURCE)
    await ta.trigger('blur')
    const saveBtn = w.findAll('button').find(b => b.text() === 'save')!
    await saveBtn.trigger('click')
    await flushPromises()
    expect(mockSave).toHaveBeenCalledTimes(1)
    const req = mockSave.mock.calls[0][0]
    expect(req.asSource).toBe(true)
    expect(req.content).toBe(PASTED_SOURCE)
    expect(req.name).toBe('research-agent')
  })

  // 表单模式保存：buildAgentSource 拼全文含未知键，round-trip 可解析
  it('AgentPaste_FormModeSavesWithExtras_2', async () => {
    mockGetSource.mockResolvedValue([PASTED_SOURCE, 'C:/x/research-agent.md'])
    const w = mountEditor(agent)
    await w.setProps({ visible: true })
    await flushPromises()
    const saveBtn = w.findAll('button').find(b => b.text() === 'save')!
    await saveBtn.trigger('click')
    await flushPromises()
    expect(mockSave).toHaveBeenCalledTimes(1)
    const req = mockSave.mock.calls[0][0]
    expect(req.asSource).toBe(true)
    expect(req.content).toContain('memory: "user"')
    expect(req.content).toContain('permission: "ask"')
    expect(req.content).toContain('tools: Read, Glob, Grep')
    const { parseFrontmatter } = await import('@/components/editor/model')
    const fm = parseFrontmatter(req.content)!
    expect(fm.fields.name).toBe('research-agent')
    expect(fm.fields.memory).toBe('user')
  })

  // 源码 frontmatter 的 name 同步进表单
  it('AgentPaste_SourceNameSyncedToForm_3', async () => {
    mockGetSource.mockResolvedValue([PASTED_SOURCE, 'C:/x/research-agent.md'])
    const w = mountEditor(null)
    await w.setProps({ visible: true })
    await flushPromises()
    const ms = w.findComponent({ name: 'ModeSwitch' })
    ms.vm.$emit('update:modelValue', 'source')
    await flushPromises()
    const ta = w.findAll('textarea').pop()!
    await ta.setValue(PASTED_SOURCE)
    await ta.trigger('blur')
    // 重新取 ModeSwitch（重渲染后旧实例引用失效）
    const ms2 = w.findComponent({ name: 'ModeSwitch' })
    ms2.vm.$emit('update:modelValue', 'form')
    await flushPromises()
    const nameInput = w.find('input.editor-input')
    expect((nameInput.element as HTMLInputElement).value).toBe('research-agent')
  })
})
