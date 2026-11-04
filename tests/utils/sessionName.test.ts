import { describe, it, expect } from 'vitest'
import {
  SESSION_NAME_PLACEHOLDER,
  isPlaceholderSessionName,
  truncateDisplayName,
} from '@/utils/session-name'

/** 真值来源：占位/截断规则的产品定义（与 Rust extract_session_name / CLI 同为 200 字符） */
describe('session-name 纯函数', () => {
  it('SessionName_Truncate_MultibyteSafe_001', () => {
    // 200 个中文字符不截断；第 201 个触发截断且按字符数（非 UTF-16 码元）
    const exactly200 = '中'.repeat(200)
    expect(truncateDisplayName(exactly200)).toBe(exactly200)
    const over = '中'.repeat(260)
    expect(truncateDisplayName(over)).toBe('中'.repeat(200) + '…')
    expect([...truncateDisplayName(over)].length).toBe(201)
  })

  it('SessionName_Truncate_NoEllipsisForAscii_001', () => {
    expect(truncateDisplayName('a'.repeat(200))).toBe('a'.repeat(200))
    expect(truncateDisplayName('a'.repeat(201))).toBe('a'.repeat(200) + '…')
  })

  it('SessionName_Placeholder_判定_001', () => {
    expect(isPlaceholderSessionName('')).toBe(true)
    expect(isPlaceholderSessionName(SESSION_NAME_PLACEHOLDER)).toBe(true)
    expect(isPlaceholderSessionName('abcdef12', 'abcdef123456')).toBe(true) // sessionId 前 8 位
    expect(isPlaceholderSessionName('用户起的名字')).toBe(false)
    expect(isPlaceholderSessionName('abcdef99', 'abcdef123456')).toBe(false)
    // sessionId 缺失时只判空与占位
    expect(isPlaceholderSessionName('abcdef12', null)).toBe(false)
  })
})
