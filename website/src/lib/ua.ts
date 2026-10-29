export type DesktopPlatform = 'windows' | 'macos' | 'linux' | 'unknown'

// 从 UA 判断桌面平台；移动设备（iPhone/iPad/iPod/Android 一律含平板）返回 unknown
export function detectDesktopPlatform(ua: string): DesktopPlatform {
  if (!ua) return 'unknown'
  if (/iPhone|iPad|iPod|Android/i.test(ua)) return 'unknown'
  if (/Windows/i.test(ua)) return 'windows'
  if (/Macintosh|Mac OS X/i.test(ua)) return 'macos'
  if (/Linux|X11/i.test(ua)) return 'linux'
  return 'unknown'
}
