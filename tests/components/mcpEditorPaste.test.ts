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
  saveMcpServer: (...a: unknown[]) => mockSave(...a),
  deleteMcpServer: vi.fn().mockResolvedValue(undefined),
}))

vi.mock('@/api/transport', () => ({
  transport: { listen: vi.fn().mockResolvedValue(() => {}) },
}))

import McpEditor from '@/components/editor/McpEditor.vue'
import type { McpServerInfo } from '@/types'

/** 用户粘贴样例：type 不在选项内 + 带自定义字段 */
const PASTED_JSON = JSON.stringify(
  {
    type: 'http-',
    url: 'https://open.bigmodel.cn/api/mcp/web_reader/mcp-',
    headers: { Authorization: 'Bearer d3c295BNz' },
  },
  null,
  2
)

function mountEditor(server: McpServerInfo | null) {
  return mount(McpEditor, {
    props: { visible: false, server },
    global: { stubs: { Teleport: true, Transition: false } },
  })
}

async function gotoJsonAndSet(w: ReturnType<typeof mountEditor>, text: string) {
  const ms = w.findComponent({ name: 'ModeSwitch' })
  ms.vm.$emit('update:modelValue', 'json')
  await flushPromises()
  const ta = w.find('textarea')
  await ta.setValue(text)
  await ta.trigger('blur')
  await flushPromises()
}

describe('McpEditor 粘贴未知 type 与未知字段', () => {
  beforeEach(() => {
    setActivePinia(createPinia())
    mockSave.mockReset().mockResolvedValue(undefined)
  })

  // type 不在选项内：失焦必须出现提示（回归：静默无提示），且不阻断 JSON 模式保存
  it('McpPaste_UnknownTypeShowsHint_1', async () => {
    const w = mountEditor(null)
    await w.setProps({ visible: true })
    await flushPromises()
    await gotoJsonAndSet(w, PASTED_JSON)
    // 提示出现（editor-hint，非阻断性错误）
    expect(w.find('.editor-hint').exists()).toBe(true)
    expect(w.find('.editor-error').exists()).toBe(false)
    // 提示不阻断 JSON 模式保存（配置本身合法，Claude 侧才校验 type）
    const saveBtn = w.findAll('button').find(b => b.text() === 'save')!
    expect(saveBtn.attributes('disabled')).toBeUndefined()
    await saveBtn.trigger('click')
    await flushPromises()
    expect(mockSave).toHaveBeenCalledTimes(1)
    const req = mockSave.mock.calls[0][0]
    expect(req.config.type).toBe('http-')
    expect(req.config.headers.Authorization).toBe('Bearer d3c295BNz')
  })

  // 未知字段在表单流转中保留：合法 http 配置 + 自定义字段 region，表单改 url 后 JSON 仍有 region
  it('McpPaste_UnknownFieldSurvivesFormRoundTrip_2', async () => {
    const server = {
      name: 'web-reader',
      sourceType: 'user',
      serverType: 'http',
      url: 'https://example.com/mcp',
      headers: {},
      env: {},
    } as unknown as McpServerInfo
    const w = mountEditor(server)
    await w.setProps({ visible: true })
    await flushPromises()
    // 先在 JSON 模式写入含未知字段的配置并失焦提交
    await gotoJsonAndSet(
      w,
      JSON.stringify({ type: 'http', url: 'https://example.com/mcp', region: 'cn', ttl: 60 }, null, 2)
    )
    // 切表单，改 url
    const ms2 = w.findComponent({ name: 'ModeSwitch' })
    ms2.vm.$emit('update:modelValue', 'form')
    await flushPromises()
    const urlInput = w.findAll('input.editor-input').find(i => (i.element as HTMLInputElement).value.startsWith('https'))!
    await urlInput.setValue('https://new.example.com/mcp')
    await flushPromises()
    // 切回 JSON：未知字段 region/ttl 必须还在
    const ms3 = w.findComponent({ name: 'ModeSwitch' })
    ms3.vm.$emit('update:modelValue', 'json')
    await flushPromises()
    const text = (w.find('textarea').element as HTMLTextAreaElement).value
    expect(text).toContain('"region": "cn"')
    expect(text).toContain('"ttl": 60')
    expect(text).toContain('https://new.example.com/mcp')
  })
})
