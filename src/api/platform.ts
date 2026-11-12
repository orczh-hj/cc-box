/**
 * 平台能力统一封装（统一管理：桌面 Tauri 插件与浏览器降级实现的单一出口）
 *
 * 组件一律调用此处 API，不直接 import Tauri 插件——浏览器模式自动降级：
 * - 剪贴板：navigator.clipboard（LAN http 非安全上下文无此 API 时回退 execCommand）
 * - 外部链接/目录：window.open（浏览器无法打开本地文件管理器）
 */

import { transportEnv } from './transport'
import { logMessage } from './tauri'

export const isTauriEnv = transportEnv.isTauri

async function tauriClipboard() {
  const m = await import('@tauri-apps/plugin-clipboard-manager')
  return m
}

/** 复制文本到剪贴板 */
export async function copyText(text: string): Promise<void> {
  if (transportEnv.isTauri) {
    const m = await tauriClipboard()
    return m.writeText(text)
  }
  if (navigator.clipboard?.writeText) {
    try {
      await navigator.clipboard.writeText(text)
      return
    } catch {
      // fallthrough 到 execCommand
    }
  }
  // 非安全上下文（http）降级
  const ta = document.createElement('textarea')
  ta.value = text
  ta.style.position = 'fixed'
  ta.style.opacity = '0'
  document.body.appendChild(ta)
  ta.select()
  document.execCommand('copy')
  ta.remove()
}

/** 读取剪贴板文本（浏览器端三层递进，任一层拿到非空文本即返回）：
 *  1. readText()：标准 Chromium 路径
 *  2. clipboard.read()：部分手机内核 readText 是空桩但 read() 可用
 *  3. execCommand('paste')：旧内核（微信/国产 WebView）仍支持的路径
 *  全失败返回空串并落诊断日志（含 UA，真机排查用） */
export async function readClipboardText(): Promise<string> {
  if (transportEnv.isTauri) {
    const m = await tauriClipboard()
    return m.readText()
  }
  // 1) 标准 readText
  try {
    const t = await navigator.clipboard?.readText?.()
    if (t) return t
  } catch {
    // 权限拒绝等：继续降级
  }
  // 2) clipboard.read() 读 ClipboardItem
  try {
    const items = await navigator.clipboard?.read?.()
    for (const item of items ?? []) {
      // 优先纯文本：多类型 ClipboardItem 里 text/html 在前，find 顺序写反会把
      // HTML 标签粘进终端
      const type =
        item.types.find(t => t === 'text/plain') ?? item.types.find(t => t.startsWith('text/'))
      if (type) {
        const text = await item.getType(type).then(b => b.text())
        if (text) return text
      }
    }
  } catch {
    // 继续降级
  }
  // 3) 旧版 execCommand('paste')：写入隐藏 textarea（需要用户手势，本函数
  //    均在点击/按键处理器内调用）
  try {
    const ta = document.createElement('textarea')
    ta.value = ''
    ta.style.cssText = 'position:fixed;left:-9999px;top:0;opacity:0'
    document.body.appendChild(ta)
    ta.focus()
    const ok = document.execCommand('paste')
    const text = ta.value
    ta.remove()
    if (ok && text) return text
  } catch {
    // 继续到失败日志
  }
  void logMessage(
    'warn',
    `[clipboard] clipboard read failed on all paths (secureContext=${window.isSecureContext}, ua=${navigator.userAgent})`,
  )
  return ''
}

/** 打开外部 URL（浏览器无法打开本地文件管理器，仅支持 http(s)） */
export function openExternal(url: string): void {
  if (transportEnv.isTauri) {
    void import('@tauri-apps/plugin-shell').then((m) => m.open(url))
    return
  }
  if (/^https?:\/\//i.test(url)) {
    window.open(url, '_blank', 'noopener')
  }
}
