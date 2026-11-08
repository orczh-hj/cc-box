import { describe, it, expect, vi, beforeEach } from 'vitest'
import { mount, flushPromises } from '@vue/test-utils'
import { setActivePinia, createPinia } from 'pinia'

const mockT = vi.fn((key: string) => key)
vi.mock('vue-i18n', () => ({
  useI18n: () => ({ t: mockT }),
  createI18n: () => ({ global: { locale: { value: 'en' } } }),
}))

vi.mock('@/api/tauri', () => ({
  saveMcpServer: vi.fn().mockResolvedValue(undefined),
  deleteMcpServer: vi.fn().mockResolvedValue(undefined),
  saveHook: vi.fn().mockResolvedValue(undefined),
  deleteHook: vi.fn().mockResolvedValue(undefined),
}))

vi.mock('@/api/transport', () => ({
  transport: { listen: vi.fn().mockResolvedValue(() => {}) },
}))

import McpEditor from '@/components/editor/McpEditor.vue'
import HookEditor from '@/components/editor/HookEditor.vue'
import type { McpServerInfo } from '@/types'

const HTTP_SERVER = {
  name: 'web-reader',
  sourceType: 'user',
  serverType: 'http',
  url: 'https://example.com/mcp',
  headers: {},
  env: {},
} as unknown as McpServerInfo

const STDIO_SERVER = {
  name: 'paper-tool',
  sourceType: 'user',
  serverType: 'stdio',
  command: 'node',
  args: ['server.js', '--verbose'],
  env: { API_KEY: 'abc' },
} as unknown as McpServerInfo

function mcpProps(server: McpServerInfo | null) {
  return { props: { visible: false, server }, global: { stubs: { Teleport: true, Transition: false } } }
}

function hookProps(hook: any) {
  return { props: { visible: false, hook }, global: { stubs: { Teleport: true, Transition: false } } }
}

async function open(w: ReturnType<typeof mount>) {
  await w.setProps({ visible: true })
  await flushPromises()
}

async function switchMode(w: ReturnType<typeof mount>, m: string) {
  w.findComponent({ name: 'ModeSwitch' }).vm.$emit('update:modelValue', m)
  await flushPromises()
}

function modalDirty(w: ReturnType<typeof mount>): boolean {
  return w.findComponent({ name: 'EditorModal' }).props('dirty') as boolean
}

describe('编辑器打开即脏复现', () => {
  beforeEach(() => {
    setActivePinia(createPinia())
  })

  it('McpDirty_StdioOpen_NotDirty_1', async () => {
    const w = mount(McpEditor, mcpProps(STDIO_SERVER))
    await open(w)
    expect(modalDirty(w)).toBe(false)
  })

  it('McpDirty_SwitchJsonAndBack_NotDirty_2', async () => {
    const w = mount(McpEditor, mcpProps(STDIO_SERVER))
    await open(w)
    await switchMode(w, 'json')
    expect(modalDirty(w)).toBe(false)
    await switchMode(w, 'form')
    expect(modalDirty(w)).toBe(false)
  })

  it('HookDirty_SwitchJsonAndBack_NotDirty_3', async () => {
    const hook = {
      event: 'PreToolUse',
      hookType: 'command',
      command: 'echo hi',
      key: 'PreToolUse:0',
      scope: 'user',
      source: { type: 'user' },
    }
    const w = mount(HookEditor, hookProps(hook))
    await open(w)
    expect(modalDirty(w)).toBe(false)
    await switchMode(w, 'json')
    expect(modalDirty(w)).toBe(false)
    await switchMode(w, 'form')
    expect(modalDirty(w)).toBe(false)
  })

  it('McpDirty_HttpOpen_NotDirty_4', async () => {
    const w = mount(McpEditor, mcpProps(HTTP_SERVER))
    await open(w)
    expect(modalDirty(w)).toBe(false)
  })

  // 修复不吞真实修改：json 模式改 URL 失焦提交后切回表单，仍应脏
  it('McpDirty_RealEditStaysDirty_5', async () => {
    const w = mount(McpEditor, mcpProps(HTTP_SERVER))
    await open(w)
    await switchMode(w, 'json')
    const ta = w.find('textarea')
    await ta.setValue((ta.element as HTMLTextAreaElement).value.replace('example.com', 'edited.com'))
    await ta.trigger('blur')
    await flushPromises()
    expect(modalDirty(w)).toBe(true)
    await switchMode(w, 'form')
    expect(modalDirty(w)).toBe(true)
  })
})
