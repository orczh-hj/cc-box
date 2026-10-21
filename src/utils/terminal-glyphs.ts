/**
 * 终端符号兜底替换（浏览器远程模式专用）
 *
 * Claude CLI 输出中的媒体控制符（"⏵⏵" 提示、⏸ 暂停等，Unicode
 * Miscellaneous Technical 块 U+23Ex–U+23Fx）在移动端系统字体中普遍缺字形，
 * xterm 渲染为 tofu 空心方块。桌面端有 Segoe UI Symbol 等系统字体回退不受
 * 影响；浏览器端在写入 xterm 前替换为字形覆盖面广的等义符号（Geometric
 * Shapes / 常用标点，Android/iOS/桌面浏览器均有字形）。
 *
 * 宽度安全性：替换前后均为 1 列（xterm Unicode 11 下 U+23F4–U+23FA 与
 * U+25B6 等均为窄宽度），不影响终端布局与 TUI 对齐。
 */
const GLYPH_FALLBACKS: Record<string, string> = {
  '\u23EF': '\u25B6', // ⏯ 播放/暂停 → ▶
  '\u23F4': '\u25C0', // ⏴ 左三角   → ◀
  '\u23F5': '\u25B6', // ⏵ 右三角   → ▶（Claude CLI "⏵⏵" 提示符）
  '\u23F6': '\u25B2', // ⏶ 上三角   → ▲
  '\u23F7': '\u25BC', // ⏷ 下三角   → ▼
  '\u23F8': '\u2016', // ⏸ 暂停     → ‖
  '\u23F9': '\u25A0', // ⏹ 停止     → ■
  '\u23FA': '\u25CF', // ⏺ 录制     → ●
}

const GLYPH_RE = /[\u23EF\u23F4-\u23FA]/gu

/** 替换移动端字体缺失的终端符号；无命中时原样返回 */
export function mapTerminalGlyphs(data: string): string {
  return data.replace(GLYPH_RE, ch => GLYPH_FALLBACKS[ch] ?? ch)
}
