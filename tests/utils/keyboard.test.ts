import { describe, it, expect } from 'vitest'
import { isUndoShortcut, READLINE_UNDO } from '@/utils/keyboard'

function mk(partial: Partial<KeyboardEvent>): KeyboardEvent {
  return {
    key: 'z',
    shiftKey: false,
    altKey: false,
    ctrlKey: false,
    metaKey: false,
    ...partial,
  } as KeyboardEvent
}

describe('isUndoShortcut', () => {
  // macOS: Cmd+Z 命中
  it('IsUndoShortcut_Mac_CmdZ_001', () => {
    expect(isUndoShortcut(mk({ metaKey: true }), true)).toBe(true)
  })

  // macOS: Ctrl+Z 不命中（保留终端 SIGTSTP 行为）
  it('IsUndoShortcut_Mac_CtrlZ_NotMatch_001', () => {
    expect(isUndoShortcut(mk({ ctrlKey: true }), true)).toBe(false)
  })

  // macOS: Cmd+Shift+Z 不命中（预留给 redo）
  it('IsUndoShortcut_Mac_CmdShiftZ_NotMatch_001', () => {
    expect(isUndoShortcut(mk({ metaKey: true, shiftKey: true }), true)).toBe(false)
  })

  // Windows: Ctrl+Z 命中
  it('IsUndoShortcut_Win_CtrlZ_001', () => {
    expect(isUndoShortcut(mk({ ctrlKey: true }), false)).toBe(true)
  })

  // Windows: Ctrl+Shift+Z 不命中
  it('IsUndoShortcut_Win_CtrlShiftZ_NotMatch_001', () => {
    expect(isUndoShortcut(mk({ ctrlKey: true, shiftKey: true }), false)).toBe(false)
  })

  // Windows: Ctrl+Alt+Z 不命中
  it('IsUndoShortcut_Win_CtrlAltZ_NotMatch_001', () => {
    expect(isUndoShortcut(mk({ ctrlKey: true, altKey: true }), false)).toBe(false)
  })

  // 非 Z 键不命中
  it('IsUndoShortcut_OtherKey_NotMatch_001', () => {
    expect(isUndoShortcut(mk({ ctrlKey: true, key: 'a' }), false)).toBe(false)
  })

  // CapsLock 开启时 key='Z' 但 shiftKey=false，应正常命中（用户仍想触发 undo）
  it('IsUndoShortcut_Win_CapsLockZ_001', () => {
    expect(isUndoShortcut(mk({ ctrlKey: true, key: 'Z' }), false)).toBe(true)
  })

  // readline undo 字节为 0x1F
  it('ReadlineUndo_Byte_001', () => {
    expect(READLINE_UNDO).toBe('\x1f')
    expect(READLINE_UNDO.charCodeAt(0)).toBe(0x1f)
  })
})
