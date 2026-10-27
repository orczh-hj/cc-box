import { describe, it, expect } from 'vitest'
import { shouldOfferLaunchWindow } from '@/utils/remoteWindows'

describe('shouldOfferLaunchWindow', () => {
  it('RemoteWindows_BrowserZeroKnown_001', () => {
    expect(shouldOfferLaunchWindow(false, true, 0)).toBe(true)
  })

  it('RemoteWindows_NotBeforeRoster_002', () => {
    expect(shouldOfferLaunchWindow(false, false, 0)).toBe(false)
  })

  it('RemoteWindows_WithWindows_003', () => {
    expect(shouldOfferLaunchWindow(false, true, 2)).toBe(false)
  })

  it('RemoteWindows_Desktop_004', () => {
    expect(shouldOfferLaunchWindow(true, true, 0)).toBe(false)
  })
})
