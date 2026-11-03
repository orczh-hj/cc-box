/**
 * OSC 52 剪贴板支持（xterm ClipboardAddon 的 provider 接线）。
 *
 * 背景：Claude CLI 的 TUI 自管选区，Cmd+C 时向终端发 OSC 52 转义序列
 * （ESC]52;c;<base64>），由终端解码写入系统剪贴板；xterm 默认不处理该
 * 序列，不装 addon 时复制静默失败（剪贴板为空，Mac 终端内 Cmd+C 复制
 * 不生效的根因）。
 * - 写入：委托 platform.copyText（桌面 Tauri 剪贴板插件 / 浏览器降级链）
 *   ——addon 默认 provider 用 navigator.clipboard，macOS WKWebView 下
 *   不可靠，必须替换
 * - 读取（OSC 52 '?' 查询）：恒返回空——静默把系统剪贴板回传给 PTY 内
 *   程序有隐私风险（iTerm2 等真实终端同样默认禁用），Claude 的复制只
 *   走写入方向，不影响使用
 */
import type { Terminal } from '@xterm/xterm'
import { ClipboardAddon } from '@xterm/addon-clipboard'
import { copyText } from '@/api/platform'

export function loadOsc52Clipboard(term: Terminal): void {
  term.loadAddon(new ClipboardAddon(undefined, {
    writeText(selection: string, text: string): void {
      // 仅系统剪贴板（'c'）；其他选区（如 X11 primary 'p'）不支持
      if (selection !== 'c') return
      // 复制失败静默：CLI 侧有 Shift 兜底提示
      void Promise.resolve(copyText(text)).catch(() => { /* ignore */ })
    },
    readText(_selection: string): string {
      return ''
    },
  }))
}
