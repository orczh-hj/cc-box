/**
 * 终端快照反渲染：把 xterm buffer 的"渲染行"还原成逻辑段落。
 *
 * 背景：Claude CLI 是 Ink 渲染的 TUI（内容渲染型），buffer 存屏幕帧而非线性文本——
 * 连续段落被 wrap-ansi 按终端宽度折行、对齐引入空格，直接复制会带假换行。
 * reflowSnapshot 用 separator 决定法逐对相邻行判断"行尾接什么"，还原原生句子。
 */

/** 终端快照（XTermTerminal.captureSnapshot 的返回值） */
export interface SnapshotData {
  /** 逐行文本（已 trimRight） */
  lines: string[]
  /** 每行 isWrapped（xterm 真软换行标志，shell 输出会产生） */
  wrapped: boolean[]
  /** 抓取时终端列数（满宽启发式基准） */
  cols: number
  /** 超过行数上限截头部时为 true */
  truncated: boolean
}

export interface ReflowResult {
  /** 反渲染后的逻辑段落文本 */
  smart: string
  /** 原始屏幕行（join '\n'），误拼兜底 */
  raw: string
}

/** 代码围栏行（``` 开头，允许缩进） */
function isCodeFence(s: string): boolean {
  return /^\s*```/.test(s)
}

/** markdown 块级结构 / diff 标记开头——这些行的换行是真实的 */
function startsBlock(s: string): boolean {
  return /^(\s*)(```|#{1,6}\s|[-*+]\s|\d+[.)]\s|>\s|\||\+ |@@ )/.test(s)
}

/** box-drawing 字符（表格/ASCII 图边框，拼接会破坏图形） */
function hasBoxArt(s: string): boolean {
  return /[─│╭╮╰╯├┤┈┊]/.test(s)
}

/** markdown 表格行（≥2 个 " | " 片段） */
function isTableRow(s: string): boolean {
  let idx = -1
  let count = 0
  while ((idx = s.indexOf(' | ', idx + 1)) !== -1) count++
  return count >= 2
}

/** 字符串在终端的显示宽度（CJK/全角按 2，近似 wcwidth） */
function displayWidth(s: string): number {
  let w = 0
  for (const ch of s) {
    const code = ch.codePointAt(0) ?? 0
    const wide =
      (code >= 0x1100 && code <= 0x115f) || // Hangul Jamo
      (code >= 0x2e80 && code <= 0xa4cf) || // CJK 部首 ~ Yi
      (code >= 0xac00 && code <= 0xd7a3) || // Hangul 音节
      (code >= 0xf900 && code <= 0xfaff) || // CJK 兼容表意
      (code >= 0xfe30 && code <= 0xfe4f) || // CJK 兼容形式
      (code >= 0xff00 && code <= 0xff60) || // 全角形式
      (code >= 0xffe0 && code <= 0xffe6) ||
      (code >= 0x20000 && code <= 0x3fffd) // CJK 扩展
    w += wide ? 2 : 1
  }
  return w
}

/**
 * 决定相邻行 A、B 之间的分隔符。
 * 返回 '\n' 保留换行；返回 '' 或 ' ' 表示拼接（B 是 A 的折行延续）。
 */
function sepBetween(a: string, b: string, bWrapped: boolean, inCode: boolean, cols: number): string {
  // xterm 真软换行（shell 输出超出宽度自动折行）
  if (bWrapped) return ''
  // 代码块内原样保留
  if (inCode) return '\n'
  // B 为空行 → 段落边界
  if (b.length === 0) return '\n'
  // B 是块级结构开头 → 换行是结构语义
  if (startsBlock(b)) return '\n'
  // 表格 / ASCII 图保护
  if (hasBoxArt(a) || hasBoxArt(b) || isTableRow(a) || isTableRow(b)) return '\n'
  // 满宽启发式：A 占满一行（大概率 wrap-ansi 折行，按显示宽度算）→ 拼接
  if (displayWidth(a) >= cols - 1) {
    // 英文单词中断处 wrap-ansi 会吃掉空格，补回一个；中文/标点无缝直拼
    if (/[A-Za-z0-9]$/.test(a) && /^[A-Za-z0-9]/.test(b)) return ' '
    return ''
  }
  return '\n'
}

/**
 * 反渲染快照：raw 原样 join；smart 按规则管线还原逻辑段落。
 */
export function reflowSnapshot(snap: SnapshotData): ReflowResult {
  const raw = snap.lines.join('\n')
  if (snap.lines.length === 0) return { smart: '', raw: '' }

  let out = ''
  let inCode = false
  for (let i = 0; i < snap.lines.length; i++) {
    const line = snap.lines[i]
    out += line
    if (i < snap.lines.length - 1) {
      // 围栏状态机：当前行是围栏则切换（围栏行自身仍按 inCode 变化前的状态处理分隔）
      if (isCodeFence(line)) inCode = !inCode
      out += sepBetween(line, snap.lines[i + 1], snap.wrapped[i + 1] ?? false, inCode, snap.cols)
    }
  }
  return { smart: out, raw }
}
