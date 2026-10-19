/**
 * 平台能力统一封装（统一管理：桌面 Tauri 插件与浏览器降级实现的单一出口）
 *
 * 组件一律调用此处 API，不直接 import Tauri 插件——浏览器模式自动降级：
 * - 剪贴板：navigator.clipboard（LAN http 非安全上下文无此 API 时回退 execCommand）
 * - 外部链接/目录：window.open（浏览器无法打开本地文件管理器）
 */

import { transportEnv } from './transport'

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

/** 读取剪贴板文本（浏览器非安全上下文不可用时返回空串） */
export async function readClipboardText(): Promise<string> {
  if (transportEnv.isTauri) {
    const m = await tauriClipboard()
    return m.readText()
  }
  try {
    return (await navigator.clipboard?.readText()) ?? ''
  } catch {
    return ''
  }
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
