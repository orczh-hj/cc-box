import { describe, it, expect, vi, beforeEach } from 'vitest'
import { mount, flushPromises } from '@vue/test-utils'
import { setActivePinia, createPinia } from 'pinia'

const mockT = vi.fn((key: string) => key)
vi.mock('vue-i18n', () => ({
  useI18n: () => ({ t: mockT }),
  createI18n: () => ({ global: { locale: { value: 'en' } } }),
}))

// getSkillSource 返回合法 frontmatter 源码
const mockGetSource = vi.fn()
vi.mock('@/api/tauri', () => ({
  getSkillSource: (...a: unknown[]) => mockGetSource(...a),
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

const skill: SkillInfo = {
  name: 'demo',
  description: 'demo skill',
  sourceType: 'user',
  enabled: true,
} as unknown as SkillInfo

describe('SkillEditor 打开编辑弹窗', () => {
  beforeEach(() => {
    setActivePinia(createPinia())
    mockGetSource.mockReset()
  })

  // 点击编辑后弹窗必须出现：visible 置 true 且源码加载完成后 overlay 渲染
  it('SkillEditor_OpenShowsOverlay_1', async () => {
    mockGetSource.mockResolvedValue(['---\nname: demo\ndescription: "demo skill"\n---\n\nbody', 'C:/x/SKILL.md'])
    const w = mount(SkillEditor, {
      props: { visible: false, skill },
      global: { stubs: { Teleport: true, Transition: false } },
    })
    await w.setProps({ visible: true })
    await flushPromises()
    expect(w.find('.editor-overlay').exists()).toBe(true)
    // 源码以编辑对象为参数请求
    expect(mockGetSource).toHaveBeenCalledWith('user', 'demo', undefined)
  })

  // 新建（skill=null）同样必须出现弹窗
  it('SkillEditor_OpenNewShowsOverlay_2', async () => {
    const w = mount(SkillEditor, {
      props: { visible: false, skill: null },
      global: { stubs: { Teleport: true, Transition: false } },
    })
    await w.setProps({ visible: true })
    await flushPromises()
    expect(w.find('.editor-overlay').exists()).toBe(true)
  })
})
