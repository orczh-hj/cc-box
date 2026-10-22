export type Platform = 'macos' | 'windows' | 'linux' | 'unknown'

export function detectPlatform(): Platform {
  const ua = navigator.userAgent.toLowerCase()
  if (ua.includes('mac')) return 'macos'
  if (ua.includes('win')) return 'windows'
  if (ua.includes('linux')) return 'linux'
  return 'unknown'
}

export const platform = detectPlatform()
export const isMac = platform === 'macos'
export const isWindows = platform === 'windows'

export const ctrl = isMac ? 'Control' : 'Ctrl'
export const alt = isMac ? 'Option' : 'Alt'
export const cmd = isMac ? 'Cmd' : 'Ctrl'

// 推断当前平台对应的 Claude CLI OSS 平台 key（与 scripts/download-deps.js 的 CLAUDE_PLATFORMS 对齐）
export function getClaudePlatformKey(): string {
  if (isWindows) return 'win32-x64'
  if (isMac) {
    // Apple Silicon 检测：userAgent 含 ARM 或平台为 MacIntel 之外的
    // 主流 M1/M2 默认 arm64，Intel 老机器保留 x64
    const ua = navigator.userAgent.toLowerCase()
    if (ua.includes('arm')) return 'darwin-arm64'
    // 兜底：现代 Mac 默认 arm64
    return 'darwin-arm64'
  }
  if (platform === 'linux') {
    const ua = navigator.userAgent.toLowerCase()
    if (ua.includes('aarch64') || ua.includes('arm64')) return 'linux-arm64'
    return 'linux-x64'
  }
  return 'win32-x64'
}

/** 浏览器保存 Blob 到下载目录（a[download]；http 非安全上下文同可用——
 * download 属性不受安全上下文限制；Tauri 环境不会走到此分支） */
export function saveBlobToFile(blob: Blob, fileName: string): void {
  const url = URL.createObjectURL(blob)
  const a = document.createElement('a')
  a.href = url
  a.download = fileName
  a.click()
  // 释放延迟：部分移动浏览器立即 revoke 会中断下载
  setTimeout(() => URL.revokeObjectURL(url), 10_000)
}

