/** 本模块所需的最小 Terminal 形状（便于单测） */
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

/**
 * 输入法候选框跟随光标。
 *
 * xterm 默认把 helper textarea 放在屏幕外（left: -9999em），输入法候选框
 * 跟随 textarea 的 caret 定位，导致候选框远离终端光标。
 *
 * 只在 compositionstart（用户开始输入法输入）那一刻把 textarea 定位到
 * 当前光标处，之后直到输入结束都不再移动——IME 弹窗锚定后自行跟随，
 * 无需持续同步。这样渲染热路径上没有任何 DOM 读写：曾因每帧渲染后
 * 移动 textarea（读取 clientWidth 强制布局 + 写 style）被怀疑导致部分
 * 机器上渲染视口停止更新。
 *
 * 坐标系：textarea 的 offsetParent（.xterm-helpers）挂在 .xterm-screen
 * 内，两者同以终端左上角为原点，用 screen 尺寸 ÷ cols/rows 换算单元格宽高。
 */
export function attachImePositionSync(term: ImeTerminalLike): void {
  const textarea = term.textarea
  if (!textarea || textarea.dataset.imeHooked) return
  textarea.dataset.imeHooked = '1'
  textarea.addEventListener('compositionstart', () => {
    positionTextareaAtCursor(term)
  })
}

/** 把 textarea 定位到终端光标所在单元格（composition 期间保持不动） */
function positionTextareaAtCursor(term: ImeTerminalLike): void {
  const textarea = term.textarea
  const screen = term.element?.querySelector('.xterm-screen') as HTMLElement | null
  if (!textarea || !screen) return

  const cellWidth = screen.clientWidth / term.cols
  const cellHeight = screen.clientHeight / term.rows
  if (!isFinite(cellWidth) || !isFinite(cellHeight) || cellWidth <= 0 || cellHeight <= 0) return

  textarea.style.left = `${term.buffer.active.cursorX * cellWidth}px`
  textarea.style.top = `${term.buffer.active.cursorY * cellHeight}px`
  textarea.style.width = `${Math.ceil(cellWidth)}px`
  textarea.style.height = `${Math.ceil(cellHeight)}px`
}
