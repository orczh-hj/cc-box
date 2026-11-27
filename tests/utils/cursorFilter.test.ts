import { describe, it, expect } from 'vitest'
import { createCursorShowFilter } from '@/utils/cursor-filter'

// DECTCEM（DEC 私有模式 25）：ESC [ ? 25 h = 显示光标，ESC [ ? 25 l = 隐藏光标
const SHOW = '\x1b[?25h'
const HIDE = '\x1b[?25l'

describe('createCursorShowFilter：显示光标序列中和', () => {
  // 完整序列在单块内 → 被移除，前后普通文本原样保留
  it('CursorFilter_移除完整序列_001', () => {
    const f = createCursorShowFilter()
    expect(f.filter(`hello${SHOW}world`)).toBe('helloworld')
  })

  // 隐藏序列（ESC[?25l）必须原样放行——它是我们想要的方向，误删会让光标回来
  it('CursorFilter_隐藏序列放行_002', () => {
    const f = createCursorShowFilter()
    expect(f.filter(`a${HIDE}b`)).toBe(`a${HIDE}b`)
  })

  // 其它私有模式序列不受影响（?25 之外：如 ?2004 括号粘贴、?1000 鼠标）
  it('CursorFilter_其它私有模式放行_003', () => {
    const f = createCursorShowFilter()
    const other = '\x1b[?2004h\x1b[?1000h\x1b[?1049h'
    expect(f.filter(other)).toBe(other)
  })

  // 序列跨 chunk 拆成 "\x1b[?2" + "5h" → 仍被移除（朴素单次替换会漏，光标复现）
  it('CursorFilter_跨chunk拆分_004', () => {
    const f = createCursorShowFilter()
    expect(f.filter('A\x1b[?2')).toBe('A')
    expect(f.filter('5hB')).toBe('B')
  })

  // 拆在 ESC 之后："\x1b" + "[?25h"
  it('CursorFilter_跨chunk拆在ESC后_005', () => {
    const f = createCursorShowFilter()
    expect(f.filter('X\x1b')).toBe('X')
    expect(f.filter('[?25hY')).toBe('Y')
  })

  // 尾部前缀不是光标序列（?2 + 别的终结符，如 ESC[?2J 清屏）→ 拼接后原样写出
  it('CursorFilter_相似前缀不是光标序列_006', () => {
    const f = createCursorShowFilter()
    expect(f.filter('P\x1b[?2')).toBe('P')
    // ESC[?2J = 清屏，必须完整保留
    expect(f.filter('JQ')).toBe('\x1b[?2JQ')
  })

  // 一块内多个显示序列全部移除
  it('CursorFilter_多序列全移除_007', () => {
    const f = createCursorShowFilter()
    expect(f.filter(`${SHOW}a${SHOW}b${SHOW}`)).toBe('ab')
  })

  // 暂存期间后块以普通字符续接（ESC 后跟字母 = 单字符转义，非 CSI）
  it('CursorFilter_暂存后非CSI续接_008', () => {
    const f = createCursorShowFilter()
    expect(f.filter('Z\x1b')).toBe('Z')
    // ESC c = 终端重置，应完整放行
    expect(f.filter('cW')).toBe('\x1bcW')
  })

  // 流结束：丢弃暂存的半个序列（终端已停，写出去只会漏 ESC 垃圾）
  it('CursorFilter_flush丢弃残留_009', () => {
    const f = createCursorShowFilter()
    expect(f.filter('end\x1b[?25')).toBe('end')
    f.flush()
    // flush 后状态干净：下一块不带上一次的残留
    expect(f.filter('next')).toBe('next')
  })
})
