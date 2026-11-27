/**
 * PTY 输出里的「显示终端光标」序列过滤（DECTCEM set：ESC [ ? 25 h）。
 *
 * 为什么需要：Claude CLI 的输入光标是自己渲染的字符块，不走终端光标；
 * xterm 原生光标在它那里纯属多余——TUI 重绘时会跟着光标位置到处跳，
 * 表现为「随渲染点位变化四处闪烁的黑块」。
 *
 * 为什么不能只在创建时发一次 ESC[?25l：CLI 启动后会发 ESC[?25h 重新显示，
 * 一次性隐藏随即被覆盖；xterm 6.1 也没有聚焦态隐藏光标的选项
 * （cursorInactiveStyle 只管失焦）。所以必须在数据入口持续中和。
 *
 * 跨 chunk：PTY 读取边界可能切在序列中间（"\x1b[?2" + "5h"），朴素的
 * 单次替换会漏掉。尾部若是不完整的前缀则暂存，与下一块拼接后再过滤。
 */

/** 可被截断的序列前缀（按长度降序，长前缀优先匹配） */
const PARTIALS = ['\x1b[?25', '\x1b[?2', '\x1b[?', '\x1b[', '\x1b']

const SHOW_CURSOR = '\x1b[?25h'

/**
 * 过滤一段 PTY 输出中的显示光标序列。
 * 返回清洗后的文本；不完整的尾序列保存在 pending 供下次拼接。
 */
export function createCursorShowFilter(): {
  filter: (chunk: string) => string
  /** 流结束时调用：丢弃暂存的不完整前缀。流已结束即终端已停止，
   *  残留的半个序列写出去只会漏出 ESC 垃圾（或恰好就是光标显示序列） */
  flush: () => void
} {
  let pending = ''

  const filter = (chunk: string): string => {
    let text = pending + chunk
    pending = ''

    // 尾部是不完整的前缀 → 暂存（不能当普通文本写出，否则会漏出 ESC 垃圾）
    for (const p of PARTIALS) {
      if (text.length >= p.length && text.endsWith(p)) {
        pending = p
        text = text.slice(0, text.length - p.length)
        break
      }
    }

    // 逐个替换（不用正则全局：避免对含特殊字符的序列做转义假设）
    let out = ''
    let i = 0
    for (;;) {
      const hit = text.indexOf(SHOW_CURSOR, i)
      if (hit === -1) {
        out += text.slice(i)
        break
      }
      out += text.slice(i, hit)
      i = hit + SHOW_CURSOR.length
    }
    return out
  }

  const flush = (): void => {
    pending = ''
  }

  return { filter, flush }
}
