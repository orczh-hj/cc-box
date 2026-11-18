/**
 * 手机软键盘输入协调（xterm textarea 的 input 事件接管判定）
 *
 * 背景：xterm 的发送逻辑对 InputEvent.data 的语义有「桌面浏览器」假设——
 * 只认 insertText 且当 data 是增量。手机内核（微信 X5、各类安卓输入法）的
 * input 事件形态是开放集合：data 可能是增量、可能是插入后的全量 value、
 * paste/replacement 形态被整体忽略……逐形态打补丁不可穷举。
 *
 * 治本模型（对 data 语义零假设）：手机模式下前端差分是唯一发送者，xterm 的
 * 三条发送路径全部旁路（inputEvent / keydown-229 定时差分 / composition
 * finalize——第三条由我方在 compositionend 时先发送组合文本并清空
 * textarea.value，使 xterm finalize 的 substring 得空串）。差分只看
 * textarea.value 的变化，任何 inputType / data 语义的内核都自动正确。
 *
 * 职责边界：
 * - 非 229 可打印 keydown 仍由 xterm 发送（该路径可靠），差分剥离该字符防双发
 * - 后缀删除换算为终端退格（imeBackspaceDelta）；替换/重写无法在终端流中表达，吞掉
 * - 追加量含换行 → term.paste（换行归一 + 括号粘贴，与桌面 Ctrl+V 同路径）
 * - 桌面模式不接管（保留显式粘贴形态拦截），行为零改动
 */

/**
 * 纯插入的 value 差分（LCP/LCS，任意位置）：返回应发送的插入段；
 * null = 无变化或含删除成分（替换/重写——终端流不可撤销，交由上层吞掉）。
 * 中间插入必须支持：自动补全双引号后光标在 '“|”' 中间，后续字符都是中间插入。
 */
export function imeAppendDelta(before: string, after: string): string | null {
  const min = Math.min(before.length, after.length)
  let p = 0
  while (p < min && before[p] === after[p]) p++
  // 最长公共后缀（不得越过前缀，防重叠计数）
  let s = 0
  while (
    s < before.length - p &&
    s < after.length - p &&
    before[before.length - 1 - s] === after[after.length - 1 - s]
  ) s++
  const removed = before.length - p - s
  const inserted = after.length - p - s
  if (removed !== 0 || inserted === 0) return null
  return after.slice(p, after.length - s)
}

/**
 * 后缀删除换算：返回终端退格串（每个 code point 一个 \x7f）；
 * null = 非删除/非后缀（中间删除、替换无法在终端流中表达，重置基线吞掉）。
 * 手机端删除路径由差分统一接管（xterm 的 keydown-229 定时差分已被拦截）。
 */
export function imeBackspaceDelta(before: string, after: string): string | null {
  if (after.length >= before.length) return null
  if (!before.startsWith(after)) return null
  const removed = [...before].length - [...after].length
  return removed > 0 ? '\x7f'.repeat(removed) : null
}

/** 原生键盘粘贴拦截判定（差分模型的补充：显式粘贴形态也走 term.paste 归一） */
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
