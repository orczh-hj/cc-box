import { describe, it, expect, vi, beforeEach } from 'vitest'

// Mock @tauri-apps/api/core 的 invoke
const mockInvoke = vi.fn()
vi.mock('@tauri-apps/api/core', () => ({
  invoke: (...args: unknown[]) => mockInvoke(...args),
}))

import { setSkillEnabled, setPluginEnabled } from '@/api/tauri'

describe('setEnabled 系列 IPC 契约', () => {
  beforeEach(() => {
    mockInvoke.mockReset()
  })

  // setPluginEnabled → 命令名 set_plugin_enabled，参数名 pluginId（camelCase，与默认 snake_case 不同）
  it('SetPluginEnabled_Call_001', async () => {
    mockInvoke.mockResolvedValue(undefined)
    await setPluginEnabled('paper-tool@orczh', false)
    expect(mockInvoke).toHaveBeenCalledWith('set_plugin_enabled', {
      pluginId: 'paper-tool@orczh',
      enabled: false,
    })
  })

  // 后端返回错误时，前端 promise reject 含错误字符串
  it('SetSkillEnabled_ErrorPropagated_001', async () => {
    mockInvoke.mockRejectedValue('Skill not found')
    await expect(setSkillEnabled('ghost', false)).rejects.toBe('Skill not found')
  })
})
