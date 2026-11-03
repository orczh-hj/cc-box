/**
 * xterm 终端主题。
 *
 * 浅色主题下 ANSI white / brightWhite 必须与浅色背景保持可读对比：
 * Windows 10 的 ConPTY 无法表示 bold 与反色，会把
 * - `SGR 1`（粗体 + 默认前景）重编码为 `SGR 1;97`（brightWhite）
 * - `SGR 7`（反色，Claude 输入框光标块用）重编码为 `SGR 30;47`（黑字白底）
 * 它假设终端是深色背景。若 white / brightWhite 贴近纯白，粗体文字与光标块
 * 会在浅色背景上直接消失（Win11 的 ConPTY 不重编码，所以只在 Win10 复现）。
 *
 * 因此浅色主题把这两个槽位压到有对比的深色：brightWhite 与默认前景一致，
 * white 取中性灰，兼顾「作为前景文字」与「作为反色块背景」两种用途。
 * 详见 docs/terminal-integration.md「终端主题」。
 */

/** 浅色终端主题 */
export const lightTheme = {
  background: '#f8f9fa',
  foreground: '#1a1a2e',
  cursor: '#6c5ce7',
  cursorAccent: '#f8f9fa',
  selectionBackground: '#ede9fe',
  selectionForeground: '#1a1a2e',
  black: '#1a1a2e',
  red: '#e74c3c',
  green: '#27ae60',
  yellow: '#f39c12',
  blue: '#3498db',
  magenta: '#9b59b6',
  cyan: '#1abc9c',
  // SGR 47（ConPTY 反色块背景）与 SGR 37（白色前景）共用此槽位
  white: '#6b7280',
  brightBlack: '#2c3e50',
  brightRed: '#e74c3c',
  brightGreen: '#27ae60',
  brightYellow: '#f39c12',
  brightBlue: '#3498db',
  brightMagenta: '#9b59b6',
  brightCyan: '#1abc9c',
  // SGR 97：Win10 ConPTY 会把「bold + 默认前景」重编码到这里
  brightWhite: '#1a1a2e'
}

/** 暗色终端主题 */
export const darkTheme = {
  background: '#1e1e1e',
  foreground: '#d4d4d4',
  cursor: '#f0d4a8',
  cursorAccent: '#1e1e1e',
  selectionBackground: '#4a7aad40',
  selectionForeground: '#d4d4d4',
  black: '#1c1a17',
  red: '#e8705a',
  green: '#5dad8e',
  yellow: '#f0b460',
  blue: '#4a7aad',
  magenta: '#b8956a',
  cyan: '#5dad8e',
  white: '#c4c0b8',
  brightBlack: '#3a3734',
  brightRed: '#f0886e',
  brightGreen: '#7abd9e',
  brightYellow: '#f8ca80',
  brightBlue: '#6a9acd',
  brightMagenta: '#d0a87e',
  brightCyan: '#7abd9e',
  brightWhite: '#f8f6f3'
}
