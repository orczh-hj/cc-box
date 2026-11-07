import { describe, it, expect } from 'vitest'
import { vkeyToBytes } from '@/utils/virtual-keys'

/**
 * 虚拟键 → ANSI 转义序列映射。
 * 期望值全部为字面量，来源：xterm ctlseqs / VT100 规范
 * （https://invisible-island.net/xterm/ctlseqs/ctlseqs.html），独立于实现。
 */
describe('vkeyToBytes（虚拟键 → ANSI 转义序列）', () => {
  // 无修饰基础键：可打印字符与控制序列
  it('vkeyToBytes_PlainKeys_001', () => {
    expect(vkeyToBytes({ key: 'a' })).toBe('a')
    expect(vkeyToBytes({ key: 'z' })).toBe('z')
    expect(vkeyToBytes({ key: '5' })).toBe('5')
    expect(vkeyToBytes({ key: 'space' })).toBe(' ')
    expect(vkeyToBytes({ key: 'enter' })).toBe('\r')
    expect(vkeyToBytes({ key: 'tab' })).toBe('\t')
    expect(vkeyToBytes({ key: 'esc' })).toBe('\x1b')
    expect(vkeyToBytes({ key: 'backspace' })).toBe('\x7f')
    expect(vkeyToBytes({ key: 'delete' })).toBe('\x1b[3~')
    expect(vkeyToBytes({ key: 'up' })).toBe('\x1b[A')
    expect(vkeyToBytes({ key: 'down' })).toBe('\x1b[B')
    expect(vkeyToBytes({ key: 'right' })).toBe('\x1b[C')
    expect(vkeyToBytes({ key: 'left' })).toBe('\x1b[D')
    expect(vkeyToBytes({ key: 'home' })).toBe('\x1b[H')
    expect(vkeyToBytes({ key: 'end' })).toBe('\x1b[F')
    expect(vkeyToBytes({ key: 'pageup' })).toBe('\x1b[5~')
    expect(vkeyToBytes({ key: 'pagedown' })).toBe('\x1b[6~')
  })

  // Shift：字母大写、数字排符号表、Shift+Tab 反向 Tab（Claude CLI 模式切换）
  it('vkeyToBytes_ShiftKeys_002', () => {
    expect(vkeyToBytes({ key: 'a', shift: true })).toBe('A')
    expect(vkeyToBytes({ key: 'q', shift: true })).toBe('Q')
    expect(vkeyToBytes({ key: '1', shift: true })).toBe('!')
    expect(vkeyToBytes({ key: '2', shift: true })).toBe('@')
    expect(vkeyToBytes({ key: '3', shift: true })).toBe('#')
    expect(vkeyToBytes({ key: '7', shift: true })).toBe('&')
    expect(vkeyToBytes({ key: '0', shift: true })).toBe(')')
    expect(vkeyToBytes({ key: '-', shift: true })).toBe('_')
    expect(vkeyToBytes({ key: '/', shift: true })).toBe('?')
    expect(vkeyToBytes({ key: 'tab', shift: true })).toBe('\x1b[Z')
  })

  // Ctrl+字母 → 控制字符 0x01-0x1A（readline/CLI 核心组合）
  it('vkeyToBytes_CtrlLetters_003', () => {
    expect(vkeyToBytes({ key: 'a', ctrl: true })).toBe('\x01')
    expect(vkeyToBytes({ key: 'c', ctrl: true })).toBe('\x03')
    expect(vkeyToBytes({ key: 'd', ctrl: true })).toBe('\x04')
    expect(vkeyToBytes({ key: 'e', ctrl: true })).toBe('\x05')
    expect(vkeyToBytes({ key: 'l', ctrl: true })).toBe('\x0c')
    expect(vkeyToBytes({ key: 'u', ctrl: true })).toBe('\x15')
    expect(vkeyToBytes({ key: 'z', ctrl: true })).toBe('\x1a')
  })

  // 方向/Home/End + 修饰 → 参数化序列（code = 1+shift+alt*2+ctrl*4）
  it('vkeyToBytes_ModifiedSequences_004', () => {
    expect(vkeyToBytes({ key: 'up', ctrl: true })).toBe('\x1b[1;5A') // 5=ctrl
    expect(vkeyToBytes({ key: 'up', shift: true })).toBe('\x1b[1;2A') // 2=shift
    expect(vkeyToBytes({ key: 'left', shift: true, ctrl: true })).toBe('\x1b[1;6D') // 6=ctrl+shift
    expect(vkeyToBytes({ key: 'home', ctrl: true })).toBe('\x1b[1;5H')
  })

  // F1-F12：F1-F4 = SS3（\x1bOP-OS），F5-F12 = CSI ~ 序列
  it('vkeyToBytes_FunctionKeys_005', () => {
    expect(vkeyToBytes({ key: 'f1' })).toBe('\x1bOP')
    expect(vkeyToBytes({ key: 'f2' })).toBe('\x1bOQ')
    expect(vkeyToBytes({ key: 'f3' })).toBe('\x1bOR')
    expect(vkeyToBytes({ key: 'f4' })).toBe('\x1bOS')
    expect(vkeyToBytes({ key: 'f5' })).toBe('\x1b[15~')
    expect(vkeyToBytes({ key: 'f6' })).toBe('\x1b[17~')
    expect(vkeyToBytes({ key: 'f12' })).toBe('\x1b[24~')
  })

  // Alt 前缀：ESC 加在完整基础结果外层（多字节序列键不丢前缀）
  it('vkeyToBytes_AltPrefix_006', () => {
    expect(vkeyToBytes({ key: 'b', alt: true })).toBe('\x1bb')
    expect(vkeyToBytes({ key: 'B', alt: true, shift: true })).toBe('\x1bB')
    expect(vkeyToBytes({ key: 'up', alt: true })).toBe('\x1b\x1b[A')
    expect(vkeyToBytes({ key: 'backspace', alt: true })).toBe('\x1b\x7f')
  })

  // 无标准映射的组合 → null（调用方忽略并释放粘滞）：
  // Ctrl+Space 无 readline 消费者；Ctrl 与 F 键/数字无标准控制字符；
  // Ctrl+Tab readline 不消费（决策见计划 Task 1）
  it('vkeyToBytes_UnmappedCombos_Null_007', () => {
    expect(vkeyToBytes({ key: 'space', ctrl: true })).toBe(null)
    expect(vkeyToBytes({ key: 'f1', ctrl: true })).toBe(null)
    expect(vkeyToBytes({ key: '1', ctrl: true })).toBe(null)
    expect(vkeyToBytes({ key: 'tab', ctrl: true })).toBe(null)
  })
})
