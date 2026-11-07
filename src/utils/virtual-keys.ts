/**
 * 虚拟键盘按键 → 终端转义序列（ANSI/VT100/xterm）纯映射模块。
 *
 * 依据 xterm ctlseqs 规范（https://invisible-island.net/xterm/ctlseqs/ctlseqs.html）：
 * - Ctrl+字母 → 控制字符 0x01-0x1A（readline 核心组合）
 * - 方向/Home/End + 修饰 → CSI 1;<code>X 参数化序列（code = 1+shift+alt*2+ctrl*4）
 * - Delete/PgUp/PgDn 等 ~ 形态 + 修饰 → CSI <n>;<code>~
 * - F1-F4 → SS3（\x1bOP-OS），F5-F12 → CSI ~ 序列（不接受修饰）
 * - Alt 任意键 → ESC 前缀加在完整基础结果外层
 * - 无标准映射的组合返回 null（调用方忽略并释放粘滞修饰）
 *
 * 消费方：VirtualKeyboard.vue（手机完整虚拟键盘弹层）。
 */
export interface VirtualKeyEvent {
  /** 规范键名：a-z、0-9、space/enter/tab/esc/backspace/delete/up/down/left/right/home/end/pageup/pagedown、f1-f12、符号 -=[];',`./\ */
  key: string
  ctrl?: boolean
  alt?: boolean
  shift?: boolean
}

/** 数字排 + 常用符号排的 Shift 符号表（美式 QWERTY） */
const SHIFT_SYMBOL: Record<string, string> = {
  '1': '!', '2': '@', '3': '#', '4': '$', '5': '%',
  '6': '^', '7': '&', '8': '*', '9': '(', '0': ')',
  '-': '_', '=': '+', '[': '{', ']': '}', '\\': '|',
  ';': ':', "'": '"', ',': '<', '.': '>', '/': '?',
  '`': '~',
}

/** 基础多字节序列（方向/Home/End 为 CSI 1 X 形态，参数化时注入修饰 code） */
const SEQ_LETTER: Record<string, string> = {
  up: 'A', down: 'B', right: 'C', left: 'D', home: 'H', end: 'F',
}

/** ~ 形态序列：键名 → CSI 参数号（delete=3、pgup=5、pgdn=6） */
const SEQ_TILDE: Record<string, number> = {
  delete: 3, pageup: 5, pagedown: 6,
}

/** F1-F4 SS3 序列（不接受修饰组合） */
const FN_SS3: Record<string, string> = {
  f1: '\x1bOP', f2: '\x1bOQ', f3: '\x1bOR', f4: '\x1bOS',
}

/** F5-F12 CSI ~ 序列参数号（不接受修饰组合） */
const FN_TILDE: Record<string, number> = {
  f5: 15, f6: 17, f7: 18, f8: 19, f9: 20, f10: 21, f11: 23, f12: 24,
}

/** 单字符基础键（space/enter/tab/esc/backspace） */
const SINGLE: Record<string, string> = {
  space: ' ', enter: '\r', tab: '\t', esc: '\x1b', backspace: '\x7f',
}

/** 修饰组合参数码：1=无，2=shift，3=alt，4=shift+alt，5=ctrl，6=ctrl+shift，7=ctrl+alt，8=ctrl+shift+alt */
function modCode(ctrl?: boolean, alt?: boolean, shift?: boolean): number {
  return 1 + (shift ? 1 : 0) + (alt ? 2 : 0) + (ctrl ? 4 : 0)
}

export function vkeyToBytes(e: VirtualKeyEvent): string | null {
  let { key, ctrl, alt, shift } = e
  // 大写字母键名归一化：等效小写 + shift（规范键名为小写，归一化保证容错）
  if (/^[A-Z]$/.test(key)) {
    key = key.toLowerCase()
    shift = true
  }

  // F 键：仅无修饰时映射
  if (FN_SS3[key]) return ctrl || alt || shift ? null : FN_SS3[key]
  if (FN_TILDE[key] !== undefined) {
    return ctrl || alt || shift ? null : `\x1b[${FN_TILDE[key]}~`
  }

  // Shift+Tab：反向 Tab（唯一带修饰的 Tab 映射；Ctrl+Tab/Alt+Tab 无 readline 消费者）
  if (key === 'tab' && shift && !ctrl && !alt) return '\x1b[Z'

  // Ctrl+字母 → 控制字符（Alt 可叠加为 ESC 前缀）
  if (ctrl && /^[a-z]$/.test(key)) {
    const base = String.fromCharCode(key.charCodeAt(0) - 96)
    return alt ? `\x1b${base}` : base
  }

  // 方向/Home/End：纯 Alt → ESC 前缀（readline 识别 \x1b\x1b[A 为 Alt+方向，
  // 与 Alt+字母语义一致）；ctrl/shift（含叠加）→ 参数化序列
  if (SEQ_LETTER[key]) {
    if (alt && !ctrl && !shift) return `\x1b\x1b[${SEQ_LETTER[key]}`
    if (ctrl || shift) {
      const code = modCode(ctrl, alt, shift)
      return `\x1b[1;${code}${SEQ_LETTER[key]}`
    }
    return `\x1b[${SEQ_LETTER[key]}`
  }

  // ~ 形态键：同上，纯 Alt 前缀 / ctrl+shift 参数化
  if (SEQ_TILDE[key] !== undefined) {
    if (alt && !ctrl && !shift) return `\x1b\x1b[${SEQ_TILDE[key]}~`
    if (ctrl || shift) {
      const code = modCode(ctrl, alt, shift)
      return `\x1b[${SEQ_TILDE[key]};${code}~`
    }
    return `\x1b[${SEQ_TILDE[key]}~`
  }

  // 单字符控制键（space/enter/tab/esc/backspace）：修饰仅支持 Alt 前缀
  if (SINGLE[key]) {
    if (ctrl) return null // Ctrl+Space 等无 readline 标准
    return alt ? `\x1b${SINGLE[key]}` : SINGLE[key]
  }

  // 可打印字符：字母/数字/符号，Shift 查表或大写
  if (/^[a-z0-9]$/.test(key) || SHIFT_SYMBOL[key] !== undefined) {
    let base: string
    if (/^[a-z]$/.test(key)) base = shift ? key.toUpperCase() : key
    else if (SHIFT_SYMBOL[key] !== undefined) base = shift ? SHIFT_SYMBOL[key] : key
    else base = key
    if (ctrl) return null // Ctrl+数字/符号无标准控制字符
    return alt ? `\x1b${base}` : base
  }

  return null
}
