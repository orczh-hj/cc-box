import { describe, it, expect } from 'vitest'
import { syncImeTextareaPosition, type ImeTerminalLike } from '@/utils/ime'

interface FixtureOptions {
  cols: number
  rows: number
  screenWidth: number
  screenHeight: number
  cursorX: number
  cursorY: number
}

function createFixture(opts: FixtureOptions) {
  const root = document.createElement('div')
  const screen = document.createElement('div')
  screen.className = 'xterm-screen'
  root.appendChild(screen)
  const textarea = document.createElement('textarea')
  root.appendChild(textarea)

  // jsdom 不做布局，clientWidth/clientHeight 恒为 0，需显式模拟
  Object.defineProperty(screen, 'clientWidth', { value: opts.screenWidth })
  Object.defineProperty(screen, 'clientHeight', { value: opts.screenHeight })

  const term: ImeTerminalLike = {
    textarea,
    element: root,
    cols: opts.cols,
    rows: opts.rows,
    buffer: { active: { cursorX: opts.cursorX, cursorY: opts.cursorY } },
  }
  return { term, textarea }
}

describe('syncImeTextareaPosition', () => {
  // 光标在 (5,3)，单元格 10x20 → textarea 定位到 (50px, 60px)，尺寸为一个单元格
  it('ImePosition_AtCursor_001', () => {
    const { term, textarea } = createFixture({
      cols: 80, rows: 24, screenWidth: 800, screenHeight: 480, cursorX: 5, cursorY: 3,
    })
    syncImeTextareaPosition(term)
    expect(textarea.style.left).toBe('50px')
    expect(textarea.style.top).toBe('60px')
    expect(textarea.style.width).toBe('10px')
    expect(textarea.style.height).toBe('20px')
  })

  // 光标在左上角 (0,0) → textarea 定位到 (0px, 0px)
  it('ImePosition_Origin_001', () => {
    const { term, textarea } = createFixture({
      cols: 100, rows: 30, screenWidth: 900, screenHeight: 600, cursorX: 0, cursorY: 0,
    })
    syncImeTextareaPosition(term)
    expect(textarea.style.left).toBe('0px')
    expect(textarea.style.top).toBe('0px')
  })

  // 单元格宽高非整数时尺寸向上取整，避免 caret 区域为零
  it('ImePosition_CeilCellSize_001', () => {
    const { term, textarea } = createFixture({
      cols: 80, rows: 24, screenWidth: 700, screenHeight: 500, cursorX: 0, cursorY: 0,
    })
    syncImeTextareaPosition(term)
    expect(textarea.style.width).toBe('9px')
    expect(textarea.style.height).toBe('21px')
  })

  // 输入法组合期间位置冻结：compositionstart 后移动光标再 sync，textarea 不动；
  // compositionend 后恢复跟随
  it('ImePosition_FreezeDuringComposition_001', () => {
    const { term, textarea } = createFixture({
      cols: 80, rows: 24, screenWidth: 800, screenHeight: 480, cursorX: 5, cursorY: 3,
    })
    syncImeTextareaPosition(term)
    expect(textarea.style.left).toBe('50px')
    expect(textarea.style.top).toBe('60px')

    textarea.dispatchEvent(new CompositionEvent('compositionstart'))
    term.buffer.active.cursorX = 79 // 光标被 TUI 重绘 park 到行尾
    term.buffer.active.cursorY = 23
    syncImeTextareaPosition(term)
    expect(textarea.style.left).toBe('50px')
    expect(textarea.style.top).toBe('60px')

    textarea.dispatchEvent(new CompositionEvent('compositionend'))
    syncImeTextareaPosition(term)
    expect(textarea.style.left).toBe('790px')
    expect(textarea.style.top).toBe('460px')
  })

  // open 前无 textarea → 不抛错
  it('ImePosition_NoTextarea_001', () => {
    const { term } = createFixture({
      cols: 80, rows: 24, screenWidth: 800, screenHeight: 480, cursorX: 5, cursorY: 3,
    })
    term.textarea = undefined
    expect(() => syncImeTextareaPosition(term)).not.toThrow()
  })

  // element 中无 .xterm-screen → 不抛错且不修改 textarea
  it('ImePosition_NoScreen_001', () => {
    const { term, textarea } = createFixture({
      cols: 80, rows: 24, screenWidth: 800, screenHeight: 480, cursorX: 5, cursorY: 3,
    })
    term.element = document.createElement('div')
    syncImeTextareaPosition(term)
    expect(textarea.style.left).toBe('')
  })
})
