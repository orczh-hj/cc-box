import { describe, it, expect } from 'vitest'
import { isWeChat } from '@/utils/ua'

describe('UA 检测', () => {
  it('Ua_WeChatAndroid_1', () => {
    expect(
      isWeChat(
        'Mozilla/5.0 (Linux; Android 13) AppleWebKit/537.36 MicroMessenger/8.0.44',
      ),
    ).toBe(true)
  })

  it('Ua_WeChatIos_1', () => {
    expect(
      isWeChat(
        'Mozilla/5.0 (iPhone; CPU iPhone OS 17_0 like Mac OS X) MicroMessenger/8.0.42',
      ),
    ).toBe(true)
  })

  it('Ua_RegularBrowser_1', () => {
    expect(
      isWeChat(
        'Mozilla/5.0 (Linux; Android 13) AppleWebKit/537.36 Chrome/120.0 Mobile Safari/537.36',
      ),
    ).toBe(false)
  })
})
