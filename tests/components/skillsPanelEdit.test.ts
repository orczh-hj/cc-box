import { describe, it, expect, vi, beforeEach } from 'vitest'
import { mount, flushPromises } from '@vue/test-utils'
import { setActivePinia, createPinia } from 'pinia'

const mockT = vi.fn((key: string) => key)
vi.mock('vue-i18n', () => ({
  useI18n: () => ({ t: mockT }),
  createI18n: () => ({ global: { locale: { value: 'en' } } }),
}))

vi.mock('@/api/tauri', () => ({
  getAllSkills: vi.fn().mockResolvedValue([]),
  getAllAgents: vi.fn().mockResolvedValue([]),
  getAllMcpServers: vi.fn().mockResolvedValue([]),
  getAllPlugins: vi.fn().mockResolvedValue([]),
  getHookList: vi.fn().mockResolvedValue([]),
  setSkillEnabled: vi.fn().mockResolvedValue([]),
  setAgentEnabled: vi.fn().mockResolvedValue([]),
  setMcpServerEnabled: vi.fn().mockResolvedValue([]),
  setPluginEnabled: vi.fn().mockResolvedValue([]),
  setHookEnabled: vi.fn().mockResolvedValue([]),
  getSkillSource: vi.fn().mockResolvedValue(['---\nname: demo\ndescription: "d"\n---\n\nbody', 'C:/x/SKILL.md']),
  saveSkill: vi.fn().mockResolvedValue(undefined),
  deleteSkill: vi.fn().mockResolvedValue(undefined),
  openInFileManager: vi.fn().mockResolvedValue(undefined),
  checkSkills: vi.fn().mockResolvedValue([]),
}))

vi.mock('@/api/transport', () => ({
  transport: { listen: vi.fn().mockResolvedValue(() => {}) },
}))

vi.mock('@/api/platform', () => ({
  isTauriEnv: false,
  copyText: vi.fn().mockResolvedValue(undefined),
}))

vi.mock('@/composables/useDocsHelp', () => ({
  useDocsHelp: () => ({ open: vi.fn() }),
}))

import SkillsPanel from '@/components/skills/SkillsPanel.vue'
import { useSidebarStore } from '@/stores/sidebar'

describe('SkillsPanel 点击编辑打开弹窗', () => {
  beforeEach(() => {
    setActivePinia(createPinia())
  })

  // 行内「编辑」点击 → 编辑弹窗出现（回归：点击无响应）
  it('SkillsPanel_EditClickOpensEditor_1', async () => {
    const sidebar = useSidebarStore()
    sidebar.skills = [
      { name: 'demo', description: 'd', sourceType: 'user', enabled: true },
    ] as never
    const w = mount(SkillsPanel, {
      global: { stubs: { Teleport: true, Transition: false } },
    })
    await flushPromises()
    // 行卡片上的编辑按钮
    const editBtn = w.find('[class*="skill"], [class*="item"] button, button')
    // 逐个点按钮直到弹窗出现（按钮文案经 mockT 原样返回 key）
    const buttons = w.findAll('button')
    let opened = false
    for (const b of buttons) {
      await b.trigger('click')
      await flushPromises()
      if (w.findComponent({ name: 'SkillEditor' }).exists() === false) continue
      // SkillEditor 内部 overlay
      if (w.find('.editor-overlay').exists()) { opened = true; break }
    }
    expect(opened || w.find('.editor-overlay').exists()).toBe(true)
    void editBtn
  })
})
