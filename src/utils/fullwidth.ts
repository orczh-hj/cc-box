/**
 * 半角 → 全角字符映射（手机电脑键盘的「中/英（全/半角）」输出模式）。
 *
 * 只转换可打印 ASCII：0x21-0x7E → U+FF01-FF5E（+0xFEE0，Unicode Halfwidth
 * and Fullwidth Forms 块），空格 0x20 → U+3000（表意空格）。
 * 控制字符（PTY 转义序列，如 \x03 / \x1b[A）原样返回——全角模式只影响
 * 可见字符输入，不影响控制键与功能键。
 *
 * 消费方：VirtualKeyboard.vue（电脑键盘全角模式）。
 */

/** 可打印 ASCII 范围（不含空格） */
const ASCII_PRINTABLE_MIN = 0x21
const ASCII_PRINTABLE_MAX = 0x7e

/** 半角 → 全角；非可打印 ASCII 原样返回 */
export function toFullWidth(ch: string): string {
  if (ch.length !== 1) return ch
  const code = ch.charCodeAt(0)
  if (code === 0x20) return '　'
  if (code >= ASCII_PRINTABLE_MIN && code <= ASCII_PRINTABLE_MAX) {
    return String.fromCharCode(code + 0xfee0)
  }
  return ch
}

/** 整个字符串是否都是可打印 ASCII（决定全角模式是否转换该输出） */
export function isPrintableAscii(s: string): boolean {
  if (!s) return false
  for (let i = 0; i < s.length; i++) {
    const code = s.charCodeAt(i)
    if (code < 0x20 || code > 0x7e) return false
  }
  return true
}
