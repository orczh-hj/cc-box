import { describe, it, expect, vi, beforeEach } from 'vitest'

// Mock @tauri-apps/plugin-updater 的 check 函数
const mockCheck = vi.fn()
vi.mock('@tauri-apps/plugin-updater', () => ({
  check: (...args: unknown[]) => mockCheck(...args),
}))

// 模拟 __APP_VERSION__
vi.stubGlobal('__APP_VERSION__', '0.8.0')

import { checkForUpdates } from '@/api/tauri'

describe('checkForUpdates 返回值', () => {
  beforeEach(() => {
    mockCheck.mockReset()
  })

  // check() 返回 null 时，返回 hasUpdate: false 的完整对象
  it('CheckForUpdates_NoUpdate_001', async () => {
    mockCheck.mockResolvedValue(null)
    const result = await checkForUpdates()
    expect(result.hasUpdate).toBe(false)
    expect(result.version).toBe('0.8.0')
    expect(result.currentVersion).toBe('0.8.0')
    expect(result.releaseNotes).toBe('')
    expect(result.platformAsset).toBeNull()
  })

  // check() 返回 update 对象时，hasUpdate 为 true，字段正确映射
  it('CheckForUpdates_HasUpdate_001', async () => {
    mockCheck.mockResolvedValue({
      version: '0.9.0',
      body: '### Features\n- New feature',
      currentVersion: '0.8.0',
    })
    const result = await checkForUpdates()
    expect(result.hasUpdate).toBe(true)
  })

  // update.body 为 undefined 时，releaseNotes 降级为空字符串
  it('CheckForUpdates_NoBody_001', async () => {
    mockCheck.mockResolvedValue({
      version: '1.0.0',
      body: undefined,
      currentVersion: '0.8.0',
    })
    const result = await checkForUpdates()
    expect(result.hasUpdate).toBe(true)
    expect(result.releaseNotes).toBe('')
  })
})
