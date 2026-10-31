import { describe, expect, it } from 'vitest'
import { resolveAutoUpdate } from '@/utils/auto-update'

describe('resolveAutoUpdate', () => {
  it('Resolve_MissingDefaultsOn_1', () => {
    // 旧 config / 字段缺失 = 默认开启
    expect(resolveAutoUpdate(undefined)).toBe(true)
  })

  it('Resolve_ExplicitValue_2', () => {
    expect(resolveAutoUpdate(true)).toBe(true)
    expect(resolveAutoUpdate(false)).toBe(false)
  })
})
