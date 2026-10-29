import { describe, expect, it } from 'vitest'
import { detectDesktopPlatform } from '../../website/src/lib/ua'

// 官网下载区平台识别：移动设备（iPhone/iPad/Android）必须返回 unknown，不得误判为 macOS/linux
describe('DownloadSection_PlatformDetect', () => {
  it('Windows Chrome UA 识别为 windows', () => {
    const ua = 'Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36'
    expect(detectDesktopPlatform(ua)).toBe('windows')
  })

  it('Mac Safari UA 识别为 macos', () => {
    const ua = 'Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/17.0 Safari/605.1.15'
    expect(detectDesktopPlatform(ua)).toBe('macos')
  })

  it('iPhone UA（含 like Mac OS X）识别为 unknown 而非 macos', () => {
    const ua = 'Mozilla/5.0 (iPhone; CPU iPhone OS 17_0 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/17.0 Mobile/15E148 Safari/604.1'
    expect(detectDesktopPlatform(ua)).toBe('unknown')
  })

  it('iPad UA 识别为 unknown', () => {
    const ua = 'Mozilla/5.0 (iPad; CPU OS 16_0 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/16.0 Mobile/15E148 Safari/604.1'
    expect(detectDesktopPlatform(ua)).toBe('unknown')
  })

  it('Android UA（含 Linux 关键字）识别为 unknown 而非 linux', () => {
    const ua = 'Mozilla/5.0 (Linux; Android 13; Pixel 7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Mobile Safari/537.36'
    expect(detectDesktopPlatform(ua)).toBe('unknown')
  })

  it('Linux Firefox UA 识别为 linux', () => {
    const ua = 'Mozilla/5.0 (X11; Linux x86_64; rv:121.0) Gecko/20100101 Firefox/121.0'
    expect(detectDesktopPlatform(ua)).toBe('linux')
  })

  it('空串识别为 unknown', () => {
    expect(detectDesktopPlatform('')).toBe('unknown')
  })
})
