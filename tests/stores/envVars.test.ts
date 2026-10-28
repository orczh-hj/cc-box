import { setActivePinia, createPinia } from 'pinia'
import { mockIPC } from '@tauri-apps/api/mocks'
import { beforeEach, describe, it, expect } from 'vitest'
import { useAppStore, DEFAULT_CLAUDE_ENV_VARS } from '@/stores/app'

// 记录 invoke 调用参数
let invokeCalls: { cmd: string; args: Record<string, unknown> }[] = []

beforeEach(() => {
  setActivePinia(createPinia())
  invokeCalls = []
  mockIPC((cmd, args) => {
    invokeCalls.push({ cmd, args: args as Record<string, unknown> })
  })
})

describe('resetClaudeEnvVars', () => {
  // resetClaudeEnvVars 恢复默认值，保留用户添加的变量
  it('EnvVars_Reset_001', async () => {
    const store = useAppStore()
    // 预置：claudeEnvVars 包含默认变量和用户添加的变量
    store.claudeEnvVars = {
      LANG: 'zh_CN.UTF-8', // 被修改的默认变量
      MY_CUSTOM_VAR: 'custom_value', // 用户添加的变量
    }
    invokeCalls = []

    await store.resetClaudeEnvVars()

    // 验证：LANG 恢复为默认值，MY_CUSTOM_VAR 保留
    expect(store.claudeEnvVars.LANG).toBe('en_US.UTF-8')
    expect(store.claudeEnvVars.MY_CUSTOM_VAR).toBe('custom_value')
    // 验证：调用 update_app_config（不再调用 sync_claude_env）
    const updateCall = invokeCalls.find(c => c.cmd === 'update_app_config')
    expect(updateCall).toBeDefined()
    const syncCall = invokeCalls.find(c => c.cmd === 'sync_claude_env')
    expect(syncCall).toBeUndefined()
  })

  // resetClaudeEnvVars 覆盖已存在的默认变量值
  it('EnvVars_Reset_Overwrite_001', async () => {
    const store = useAppStore()
    // 预置：所有默认变量都有非默认值
    store.claudeEnvVars = {
      LANG: 'zh_CN.UTF-8',
      LC_ALL: 'zh_CN.UTF-8',
      PYTHONUTF8: '0',
      CLAUDE_CODE_SCROLL_SPEED: '10',
      PYTHONIOENCODING: 'ascii',
      CLAUDE_CODE_NO_FLICKER: '0',
    }
    invokeCalls = []

    await store.resetClaudeEnvVars()

    // 验证：所有默认变量恢复为 DEFAULT_CLAUDE_ENV_VARS 中的值
    expect(store.claudeEnvVars.LANG).toBe(DEFAULT_CLAUDE_ENV_VARS.LANG)
    expect(store.claudeEnvVars.LC_ALL).toBe(DEFAULT_CLAUDE_ENV_VARS.LC_ALL)
    expect(store.claudeEnvVars.PYTHONUTF8).toBe(DEFAULT_CLAUDE_ENV_VARS.PYTHONUTF8)
    expect(store.claudeEnvVars.CLAUDE_CODE_SCROLL_SPEED).toBe(DEFAULT_CLAUDE_ENV_VARS.CLAUDE_CODE_SCROLL_SPEED)
    expect(store.claudeEnvVars.PYTHONIOENCODING).toBe(DEFAULT_CLAUDE_ENV_VARS.PYTHONIOENCODING)
    expect(store.claudeEnvVars.CLAUDE_CODE_NO_FLICKER).toBe(DEFAULT_CLAUDE_ENV_VARS.CLAUDE_CODE_NO_FLICKER)
  })
})
