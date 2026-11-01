import { beforeEach, describe, expect, it } from 'vitest'
import { useToast, clearToasts } from '@/composables/useToast'
import {
  notifyDesktopOutdated,
  resetDesktopOutdatedNotice,
  setDesktopOutdatedTextProvider,
} from '@/utils/desktop-outdated'

describe('notifyDesktopOutdated', () => {
  beforeEach(() => {
    clearToasts()
    resetDesktopOutdatedNotice()
    setDesktopOutdatedTextProvider(() => '请更新桌面端')
  })

  // 新前端撞上旧桌面：提示一次即可，连续失败不应刷屏
  it('DesktopOutdated_NotifyOnce_1', () => {
    notifyDesktopOutdated()
    notifyDesktopOutdated()
    notifyDesktopOutdated()
    const { toasts } = useToast()
    expect(toasts.value).toHaveLength(1)
    expect(toasts.value[0].type).toBe('error')
    expect(toasts.value[0].text).toBe('请更新桌面端')
  })

  it('DesktopOutdated_ResetAllowsAgain_2', () => {
    notifyDesktopOutdated()
    resetDesktopOutdatedNotice()
    notifyDesktopOutdated()
    expect(useToast().toasts.value).toHaveLength(2)
  })

  // 未注入文案时不弹空白 toast（桌面端内嵌页面正常路径）
  it('DesktopOutdated_NoProviderNoToast_3', () => {
    setDesktopOutdatedTextProvider(null)
    notifyDesktopOutdated()
    expect(useToast().toasts.value).toHaveLength(0)
  })
})
