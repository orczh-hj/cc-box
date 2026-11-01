import { describe, it, expect, vi, beforeEach } from 'vitest'
import { mount, flushPromises } from '@vue/test-utils'

const mockT = vi.fn((key: string) => key)
vi.mock('vue-i18n', () => ({
  useI18n: () => ({ t: mockT }),
}))

const mockSetBrokerAutostart = vi.fn()
const mockGetRemoteStatus = vi.fn()
const mockGetCheckResults = vi.fn()
const mockGetInstalledClaudeVersion = vi.fn()
const mockGetClaudeConfigDir = vi.fn()

vi.mock('@/api/tauri', () => ({
  getRemoteStatus: (...args: unknown[]) => mockGetRemoteStatus(...args),
  setBrokerAutostart: (...args: unknown[]) => mockSetBrokerAutostart(...args),
  getCheckResults: (...args: unknown[]) => mockGetCheckResults(...args),
  getInstalledClaudeVersion: (...args: unknown[]) => mockGetInstalledClaudeVersion(...args),
  getClaudeConfigDir: (...args: unknown[]) => mockGetClaudeConfigDir(...args),
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
    mockGetCheckResults.mockReset()
    mockGetInstalledClaudeVersion.mockReset()
    mockGetClaudeConfigDir.mockReset()
    // 默认：无检查结果、无版本、无目录（不影响自启动用例）
    mockGetCheckResults.mockResolvedValue([])
    mockGetInstalledClaudeVersion.mockResolvedValue(null)
    mockGetClaudeConfigDir.mockResolvedValue('')
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

  it('Startup_RuntimeInfoReadOnly_002', async () => {
    mockGetRemoteStatus.mockResolvedValue({ autostart: false, brokerConnected: false })
    mockGetCheckResults.mockResolvedValue([
      { name: 'Git Bash', passed: true },
      { name: 'Claude CLI', passed: true, detectedPath: 'C:/tools/claude.exe' },
    ])
    mockGetInstalledClaudeVersion.mockResolvedValue('2.1.179')
    mockGetClaudeConfigDir.mockResolvedValue('D:/custom-claude-config')

    const wrapper = mount(StartupSection, { attachTo: document.body })
    await flushPromises()

    const rows = wrapper.findAll('.runtime-row .runtime-value')
    expect(rows).toHaveLength(3)
    expect(rows[0].text()).toBe('C:/tools/claude.exe')
    expect(rows[1].text()).toBe('v2.1.179')
    expect(rows[2].text()).toBe('D:/custom-claude-config')
    // 只读：呈现为 code 文本，不含输入控件
    expect(wrapper.findAll('.runtime-card input')).toHaveLength(0)
  })
})
