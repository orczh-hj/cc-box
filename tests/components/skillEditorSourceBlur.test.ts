import { describe, it, expect, vi, beforeEach } from 'vitest'
import { mount, flushPromises } from '@vue/test-utils'
import { setActivePinia, createPinia } from 'pinia'

const mockT = vi.fn((key: string) => key)
vi.mock('vue-i18n', () => ({
  useI18n: () => ({ t: mockT }),
  createI18n: () => ({ global: { locale: { value: 'en' } } }),
}))

vi.mock('@/api/tauri', () => ({
  getSkillSource: vi.fn().mockResolvedValue(['---\nname: demo\ndescription: "d"\n---\n\nbody', 'C:/x/SKILL.md']),
  saveSkill: vi.fn().mockResolvedValue(undefined),
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

const skill = { name: 'demo', description: 'd', sourceType: 'user', enabled: true } as unknown as SkillInfo

describe('SkillEditor 源码失焦校验', () => {
  beforeEach(() => {
    setActivePinia(createPinia())
    mockT.mockClear()
  })

  // 源码格式非法时失焦必须出现错误提示（回归：blur 无提示）
  it('SkillEditor_InvalidSourceBlurShowsError_1', async () => {
    const w = mount(SkillEditor, {
      props: { visible: false, skill },
      global: { stubs: { Teleport: true, Transition: false } },
    })
    await w.setProps({ visible: true })
    await flushPromises()
    // 切到源码模式
    const modeSwitch = w.findComponent({ name: 'ModeSwitch' })
    expect(modeSwitch.exists()).toBe(true)
    modeSwitch.vm.$emit('update:modelValue', 'source')
    await w.setProps({ visible: true })
    await flushPromises()
    // 直接把内部 mode 置为 source 再改文本触发失焦
    const ta = w.find('textarea')
    expect(ta.exists()).toBe(true)
    await ta.setValue('没有 frontmatter 的非法源码')
    await ta.trigger('blur')
    await flushPromises()
    expect(w.find('.editor-error').exists()).toBe(true)
  })

  // 合法源码失焦不报错且回填表单
  it('SkillEditor_ValidSourceBlurNoError_2', async () => {
    const w = mount(SkillEditor, {
      props: { visible: false, skill },
      global: { stubs: { Teleport: true, Transition: false } },
    })
    await w.setProps({ visible: true })
    await flushPromises()
    const ta = w.find('textarea')
    // 此时仍是 form 模式，第一个 textarea 是 content；切源码后再测
    void ta
    // 通过内部状态切源码：模拟 ModeSwitch emit
    const ms = w.findComponent({ name: 'ModeSwitch' })
    ms.vm.$emit('update:modelValue', 'source')
    await flushPromises()
    const sourceTa = w.findAll('textarea').pop()!
    await sourceTa.setValue('---\nname: other\ndescription: "x"\n---\n\nhello')
    await sourceTa.trigger('blur')
    await flushPromises()
    expect(w.find('.editor-error').exists()).toBe(false)
  })
})
