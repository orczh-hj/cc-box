/** syncImeTextareaPosition 所需的最小 Terminal 形状（便于单测） */
export interface ImeTerminalLike {
  textarea: HTMLTextAreaElement | undefined
  element: HTMLElement | undefined
  cols: number
  rows: number
  buffer: {
    active: {
      cursorX: number
      cursorY: number
    }
  }
}

// 正在输入法组合中的 textarea 集合（composition 期间冻结位置）
const composing = new WeakSet<HTMLTextAreaElement>()

function hookCompositionEvents(textarea: HTMLTextAreaElement): void {
  if (textarea.dataset.imeHooked) return
  textarea.dataset.imeHooked = '1'
  textarea.addEventListener('compositionstart', () => composing.add(textarea))
  textarea.addEventListener('compositionend', () => composing.delete(textarea))
}

/**
 * 将 xterm 隐藏 textarea 定位到终端光标处。
 *
 * xterm 默认把 helper textarea 放在屏幕外（left: -9999em），而输入法候选框
 * 跟随 textarea 的 caret 定位，导致候选框远离终端光标。把 textarea 移到
 * 光标所在单元格，候选框即可出现在光标附近。
 *
 * 坐标系：textarea 的 offsetParent（.xterm-helpers）挂在 .xterm-screen 内，
 * 用 screen 尺寸 ÷ cols/rows 换算单元格宽高。
 *
 * 输入法组合期间（compositionstart → compositionend）冻结位置：TUI 应用
 * （如 Claude CLI）重绘/spinner 时会把光标 park 到行尾，若此时移动 textarea，
 * 正在输入的候选框会被拉到屏幕角落。
 *
 * term.open() 之前 textarea 不存在，内部自判跳过。
 */
export function syncImeTextareaPosition(term: ImeTerminalLike): void {
  const textarea = term.textarea
  const screen = term.element?.querySelector('.xterm-screen') as HTMLElement | null
  if (!textarea || !screen) return

  hookCompositionEvents(textarea)
  if (composing.has(textarea)) return

  const cellWidth = screen.clientWidth / term.cols
  const cellHeight = screen.clientHeight / term.rows

  const left = term.buffer.active.cursorX * cellWidth
  const top = term.buffer.active.cursorY * cellHeight

  textarea.style.left = `${left}px`
  textarea.style.top = `${top}px`
  textarea.style.width = `${Math.ceil(cellWidth)}px`
  textarea.style.height = `${Math.ceil(cellHeight)}px`
}
