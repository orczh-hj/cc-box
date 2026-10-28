import { describe, it, expect } from 'vitest'
import { isMissingUpdatePlatform } from '@/utils/update-error'

describe('isMissingUpdatePlatform', () => {
  // Tauri updater 缺少当前平台键时的真实报错
  it('UpdateError_MissingPlatform_001', () => {
    expect(
      isMissingUpdatePlatform(
        'None of the fallback platforms ["darwin-x86_64-app", "darwin-x86_64"] ' +
        "were found in the response 'platforms' object",
      ),
    ).toBe(true)
  })

  // 其他网络/签名错误不应被误判
  it('UpdateError_OtherError_002', () => {
    expect(isMissingUpdatePlatform('network error')).toBe(false)
    expect(isMissingUpdatePlatform('signature verification failed')).toBe(false)
  })
})
