import { describe, it, expect, vi, beforeEach } from 'vitest'
import { mount, flushPromises } from '@vue/test-utils'

const mockT = vi.fn((key: string) => key)
vi.mock('vue-i18n', () => ({
  useI18n: () => ({ t: mockT }),
}))

const mockSetBrokerAutostart = vi.fn()
const mockGetRemoteStatus = vi.fn()

vi.mock('@/api/tauri', () => ({
  getRemoteStatus: (...args: unknown[]) => mockGetRemoteStatus(...args),
  setBrokerAutostart: (...args: unknown[]) => mockSetBrokerAutostart(...args),
}))

vi.mock('@/stores/app', () => ({
  DEFAULT_CLAUDE_ENV_VARS: {},
  useAppStore: () => ({
    defaultClaudeOptions: { skipPermissions: false, customArgs: '' },
    claudeEnvVars: {},
    setDefaultClaudeOptions: vi.fn(),
    setClaudeEnvVars: vi.fn(),
  }),
}))

import StartupSection from '@/components/settings/sections/StartupSection.vue'

describe('StartupSection 后台服务自启动', () => {
  beforeEach(() => {
    mockGetRemoteStatus.mockReset()
    mockSetBrokerAutostart.mockReset()
  })

  it('Startup_BrokerAutostartToggle_001', async () => {
    mockGetRemoteStatus.mockResolvedValue({ autostart: false, brokerConnected: true })
    mockSetBrokerAutostart.mockResolvedValue({ autostart: true, brokerConnected: true })

    const wrapper = mount(StartupSection, { attachTo: document.body })
    await flushPromises()

    const input = wrapper.find('.broker-autostart-card input').element as HTMLInputElement
    expect(input.checked).toBe(false)
    expect(input.disabled).toBe(false)

    input.checked = true
    await wrapper.find('.broker-autostart-card input').trigger('change')
    await flushPromises()

    expect(mockSetBrokerAutostart).toHaveBeenCalledWith(true)
    expect((wrapper.find('.broker-autostart-card input').element as HTMLInputElement).checked).toBe(true)
    wrapper.unmount()
  })

  it('Startup_BrokerUnavailable_002', async () => {
    mockGetRemoteStatus.mockResolvedValue({ autostart: false, brokerConnected: false })

    const wrapper = mount(StartupSection, { attachTo: document.body })
    await flushPromises()

    const input = wrapper.find('.broker-autostart-card input').element as HTMLInputElement
    expect(input.disabled).toBe(true)
    wrapper.unmount()
  })
})
