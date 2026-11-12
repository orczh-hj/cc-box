/**
 * 原生键盘粘贴拦截判定（手机软键盘 / 长按菜单粘贴走 xterm textarea input 事件）
 *
 * xterm 的 _inputEvent 只处理 inputType === "insertText"，且把 data 原样发 PTY：
 * - insertFromPaste（键盘剪贴板/长按粘贴）被整体忽略 → 粘贴完全无反应
 * - insertText 带换行（整段粘贴）→ \n 不归一为 \r、无括号粘贴包裹 → 换行丢失/错乱
 * 命中此两种形态时拦截（stopImmediatePropagation）改走 term.paste（换行归一 +
 * 括号粘贴，与桌面 Ctrl+V 同路径）。
 */
export function shouldInterceptImeInput(e: InputEvent): boolean {
  if (e.isComposing) return false
  const data = e.data ?? ''
  if (
    e.inputType === 'insertFromPaste' ||
    e.inputType === 'insertReplacementText'
  ) {
    return true
  }
  return data.includes('\n')
}
