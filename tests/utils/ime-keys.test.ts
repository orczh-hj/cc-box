import { describe, expect, it } from 'vitest'
import { isKeyWithIme } from '@/utils/ime-keys'

// 中文 IME 下终端 Ctrl+C/V 失效的兜底判定：IME 接管时 keydown 报
// key='Process'（真实 bug：复制变 SIGINT 出 exit 提示、粘贴静默失效）
describe('isKeyWithIme', () => {
  it('IME_Process形态下用code兜底匹配', () => {
    // 中文输入法开启时的 Ctrl+C（修复前 key === 'c' 判定失配）
    expect(isKeyWithIme({ key: 'Process', code: 'KeyC' }, 'KeyC', 'c')).toBe(true)
    expect(isKeyWithIme({ key: 'Process', code: 'KeyV' }, 'KeyV', 'v')).toBe(true)
  })

  it('普通key字面量匹配保持不变', () => {
    expect(isKeyWithIme({ key: 'c', code: 'KeyC' }, 'KeyC', 'c')).toBe(true)
    expect(isKeyWithIme({ key: 'C', code: 'KeyC' }, 'KeyC', 'C')).toBe(true)
  })

  it('code不匹配或key不同时不误判', () => {
    // Process 报的是别的物理键（如 KeyV），不能匹配 KeyC 的判定
    expect(isKeyWithIme({ key: 'Process', code: 'KeyV' }, 'KeyC', 'c')).toBe(false)
    expect(isKeyWithIme({ key: 'v', code: 'KeyV' }, 'KeyC', 'c')).toBe(false)
  })
})
