import { describe, it, expect } from 'vitest'
import { attachImePositionSync, type ImeTerminalLike } from '@/utils/ime'

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

describe('attachImePositionSync', () => {
  // 挂接后触发 compositionstart，textarea 定位到光标 (5,3) → (50px, 60px)，尺寸为一个单元格
  it('ImePosition_AtCursorOnCompositionStart_001', () => {
    const { term, textarea } = createFixture({
      cols: 80, rows: 24, screenWidth: 800, screenHeight: 480, cursorX: 5, cursorY: 3,
    })
    attachImePositionSync(term)
    // 挂接本身不定位（渲染热路径零 DOM 操作）
    expect(textarea.style.left).toBe('')

    textarea.dispatchEvent(new CompositionEvent('compositionstart'))
    expect(textarea.style.left).toBe('50px')
    expect(textarea.style.top).toBe('60px')
    expect(textarea.style.width).toBe('10px')
    expect(textarea.style.height).toBe('20px')
  })

  // composition 期间光标被 TUI 重绘 park 到行尾，textarea 不再移动（无需冻结逻辑）
  it('ImePosition_NoMoveAfterCompositionStart_001', () => {
    const { term, textarea } = createFixture({
      cols: 80, rows: 24, screenWidth: 800, screenHeight: 480, cursorX: 5, cursorY: 3,
    })
    attachImePositionSync(term)
    textarea.dispatchEvent(new CompositionEvent('compositionstart'))
    expect(textarea.style.left).toBe('50px')

    term.buffer.active.cursorX = 79
    term.buffer.active.cursorY = 23
    attachImePositionSync(term) // 渲染回调再触发也不会移动
    expect(textarea.style.left).toBe('50px')
    expect(textarea.style.top).toBe('60px')
  })

  // 每次新的 compositionstart 用最新光标位置重新定位（resize/光标移动后自动新鲜）
  it('ImePosition_RelocateOnNextComposition_001', () => {
    const { term, textarea } = createFixture({
      cols: 80, rows: 24, screenWidth: 800, screenHeight: 480, cursorX: 5, cursorY: 3,
    })
    attachImePositionSync(term)
    textarea.dispatchEvent(new CompositionEvent('compositionstart'))
    textarea.dispatchEvent(new CompositionEvent('compositionend'))

    term.buffer.active.cursorX = 40
    term.buffer.active.cursorY = 10
    textarea.dispatchEvent(new CompositionEvent('compositionstart'))
    expect(textarea.style.left).toBe('400px')
    expect(textarea.style.top).toBe('200px')
  })

  // 重复调用 attach 幂等（渲染回调每帧调用，只挂一次监听）
  it('ImePosition_AttachIdempotent_001', () => {
    const { term, textarea } = createFixture({
      cols: 80, rows: 24, screenWidth: 800, screenHeight: 480, cursorX: 5, cursorY: 3,
    })
    attachImePositionSync(term)
    attachImePositionSync(term)
    // 挂两次仍只响应一次 compositionstart（第二次 attach 被短路，行为不变）
    textarea.dispatchEvent(new CompositionEvent('compositionstart'))
    expect(textarea.style.left).toBe('50px')
  })

  // open 前无 textarea → 不抛错、不挂接；textarea 出现后可正常挂接
  it('ImePosition_NoTextarea_001', () => {
    const { term, textarea } = createFixture({
      cols: 80, rows: 24, screenWidth: 800, screenHeight: 480, cursorX: 5, cursorY: 3,
    })
    term.textarea = undefined
    expect(() => attachImePositionSync(term)).not.toThrow()

    term.textarea = textarea
    attachImePositionSync(term)
    textarea.dispatchEvent(new CompositionEvent('compositionstart'))
    expect(textarea.style.left).toBe('50px')
  })

  // cols 为 0（初始化中）时 cellWidth=Infinity/NaN → 跳过不写样式
  it('ImePosition_ZeroCols_001', () => {
    const { term, textarea } = createFixture({
      cols: 0, rows: 24, screenWidth: 800, screenHeight: 480, cursorX: 5, cursorY: 3,
    })
    attachImePositionSync(term)
    expect(() => textarea.dispatchEvent(new CompositionEvent('compositionstart'))).not.toThrow()
    expect(textarea.style.left).toBe('')
  })
})
