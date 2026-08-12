import { isMac } from './platform'

/**
 * Undo 快捷键判断：macOS 为 Cmd+Z，Windows/Linux 为 Ctrl+Z
 *
 * 命中后由调用方注入 \x1f（Ctrl+_，readline 原生 undo）到 PTY，
 * 并阻止原始字节进入 PTY——Windows/Linux 上避免触发 SIGTSTP 暂停 Claude 进程。
 *
 * Shift 修饰（Ctrl+Shift+Z / Cmd+Shift+Z）当前未启用 redo，明确不命中。
 */
export function isUndoShortcut(event: KeyboardEvent, mac: boolean = isMac): boolean {
  const key = event.key.toLowerCase()
  if (key !== 'z') return false
  if (event.shiftKey || event.altKey) return false

  return mac
    ? event.metaKey && !event.ctrlKey
    : event.ctrlKey && !event.metaKey
}

/** readline 原生 undo 控制字符（Ctrl+_ = 0x1F） */
export const READLINE_UNDO = '\x1f'
