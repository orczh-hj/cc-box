import { describe, it, expect, vi, beforeEach } from 'vitest'
import { mount, flushPromises } from '@vue/test-utils'
import { setActivePinia, createPinia } from 'pinia'

const mockT = vi.fn((key: string) => key)
vi.mock('vue-i18n', () => ({
  useI18n: () => ({ t: mockT }),
  createI18n: () => ({ global: { locale: { value: 'en' } } }),
}))

const mockSave = vi.fn()
vi.mock('@/api/tauri', () => ({
  saveHook: (...a: unknown[]) => mockSave(...a),
  deleteHook: vi.fn().mockResolvedValue(undefined),
}))

vi.mock('@/api/transport', () => ({
  transport: { listen: vi.fn().mockResolvedValue(() => {}) },
}))

import HookEditor from '@/components/editor/HookEditor.vue'
import type { HookListItem } from '@/api/tauri'

/** 列表下发条目：hook 对象与 matcher 条目层都带未知字段 */
const hookItem = {
  event: 'PreToolUse',
  matcher: 'Write',
  command: 'echo hi',
  hookType: 'command',
  hookCount: 1,
  timeout: 30,
  source: { type: 'user', label: 'user' },
  enabled: true,
  key: 'PreToolUse|Write|echo hi',
  scope: 'user',
  raw: {
    matcher: 'Write',
    hooks: [{ type: 'command', command: 'echo hi', timeout: 30, async: true }],
    customMeta: 'keep-me',
  },
} as unknown as HookListItem

describe('HookEditor 未知字段保留', () => {
  beforeEach(() => {
    setActivePinia(createPinia())
    mockSave.mockReset().mockResolvedValue(undefined)
  })

  // 编辑打开（raw 下发）→ 表单改命令 → 切 JSON：hook 对象与条目层未知字段必须保留
  it('HookRaw_UnknownFieldsSurviveFormEdit_1', async () => {
    const w = mount(HookEditor, {
      props: { visible: false, hook: hookItem },
      global: { stubs: { Teleport: true, Transition: false } },
    })
    await w.setProps({ visible: true })
    await flushPromises()
    // 表单模式改命令
    const ta = w.find('textarea')
    await ta.setValue('echo changed')
    await flushPromises()
    // 切 JSON 查看生成结果
    const ms = w.findComponent({ name: 'ModeSwitch' })
    ms.vm.$emit('update:modelValue', 'json')
    await flushPromises()
    const text = (w.find('textarea').element as HTMLTextAreaElement).value
    const parsed = JSON.parse(text)
    expect(parsed.hooks[0].command).toBe('echo changed')
    expect(parsed.hooks[0].async).toBe(true)
    expect(parsed.customMeta).toBe('keep-me')
  })

  // 未下发 raw（旧桌面端）时行为不回归：表单编辑仍可保存
  it('HookRaw_MissingRawStillSaves_2', async () => {
    const { raw: _raw, ...noRaw } = hookItem
    const w = mount(HookEditor, {
      props: { visible: false, hook: noRaw as HookListItem },
      global: { stubs: { Teleport: true, Transition: false } },
    })
    await w.setProps({ visible: true })
    await flushPromises()
    const ta = w.find('textarea')
    await ta.setValue('echo changed')
    await flushPromises()
    const saveBtn = w.findAll('button').find(b => b.text() === 'save')!
    await saveBtn.trigger('click')
    await flushPromises()
    expect(mockSave).toHaveBeenCalledTimes(1)
    expect(mockSave.mock.calls[0][0].hook.command).toBe('echo changed')
  })
})
