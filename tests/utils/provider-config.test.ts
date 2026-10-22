import { describe, it, expect } from 'vitest'
import cases from '../fixtures/provider-merge/cases.json'
import {
  resolveEffective,
  diffAgainstCommon,
  computeRemovedPaths,
  deleteAtJsonPointer,
  escapePointerToken,
  unescapePointerToken,
  deepEqual,
  stripEmptyCoreEnv,
} from '../../src/utils/provider-config'
import type { CommonConfig, ProviderMeta } from '../../src/types/provider'

// 契约样例：同一份 JSON 由 Rust 测试共同消费（tests/fixtures/provider-merge/cases.json）
describe('ProviderConfigMerge_Resolve_001', () => {
  for (const c of cases.resolve) {
    it(c.name, () => {
      const common = { enabled: c.commonEnabled, settings: c.common } as CommonConfig
      const effective = resolveEffective(common, c.overrides, c.meta as ProviderMeta)
      expect(effective).toEqual(c.effective)
    })
  }
})

describe('ProviderConfigMerge_Migration_001', () => {
  for (const c of cases.migration) {
    it(c.name, () => {
      // common 关闭时迁移跳过（保留 legacy 作为覆盖层）
      const overrides = c.commonEnabled
        ? diffAgainstCommon(c.legacy, c.common)
        : c.legacy
      expect(overrides).toEqual(c.expected)
    })
  }
})

describe('ProviderConfigMerge_Diff_001', () => {
  for (const c of cases.diff) {
    it(c.name, () => {
      expect(diffAgainstCommon(c.effective, c.common)).toEqual(c.expected)
    })
  }
})

describe('ProviderConfigMerge_RemovedPaths_001', () => {
  it('ProviderConfigMerge_RemovedPaths_Deletion_001', () => {
    const common = { env: { FOO: '1', BAR: '2' }, effortLevel: 'high' }
    const prevEffective = { env: { FOO: '1', BAR: '2' }, effortLevel: 'high' }
    // 用户在编辑器中删除了 /env/FOO
    const nextEffective = { env: { BAR: '2' }, effortLevel: 'high' }
    const overrides = diffAgainstCommon(nextEffective, common)

    expect(computeRemovedPaths(prevEffective, nextEffective, [], common, overrides))
      .toEqual(['/env/FOO'])
  })

  it('ProviderConfigMerge_RemovedPaths_Restore_001', () => {
    const common = { env: { FOO: '1' } }
    const prevEffective = { env: {} }
    const nextEffective = { env: { FOO: '2' } }
    const overrides = diffAgainstCommon(nextEffective, common)

    // 重新填写值 → 不再屏蔽；新值不同 → 成为覆盖值
    expect(computeRemovedPaths(prevEffective, nextEffective, ['/env/FOO'], common, overrides))
      .toEqual([])
    expect(overrides).toEqual({ env: { FOO: '2' } })
  })

  it('ProviderConfigMerge_RemovedPaths_OverrideCovered_001', () => {
    const common = { a: { b: 1 } }
    // 覆盖层把父路径 /a 整体替换为数组 → 屏蔽 /a/b 已无意义，自动归一化移除
    const prevEffective = { a: { b: 1 } }
    const nextEffective = { a: [] }
    expect(computeRemovedPaths(prevEffective, nextEffective, ['/a/b'], common, { a: [] }))
      .toEqual([])
  })
})

describe('ProviderConfigMerge_StripEmptyCoreEnv_001', () => {
  it('removes-empty-core-env-only', () => {
    expect(stripEmptyCoreEnv({
      env: {
        ANTHROPIC_AUTH_TOKEN: '',
        ANTHROPIC_BASE_URL: '',
        ANTHROPIC_MODEL: 'm',
        CUSTOM: '',
      },
    })).toEqual({
      env: {
        ANTHROPIC_MODEL: 'm',
        CUSTOM: '',
      },
    })
  })

  it('keeps-non-core-empty-values', () => {
    expect(stripEmptyCoreEnv({ env: { CLAUDE_CODE_SCROLL_SPEED: '' } }))
      .toEqual({ env: { CLAUDE_CODE_SCROLL_SPEED: '' } })
  })
})

describe('ProviderConfigMerge_JsonPointer_001', () => {
  it('ProviderConfigMerge_JsonPointer_Escape_001', () => {
    expect(escapePointerToken('a/b~c')).toBe('a~1b~0c')
    expect(unescapePointerToken('a~1b~0c')).toBe('a/b~c')
  })

  it('ProviderConfigMerge_JsonPointer_Delete_001', () => {
    const value = { env: { FOO: '1', BAR: '2' }, nested: { a: 1 } }
    expect(deleteAtJsonPointer(value, '/env/FOO')).toBe(true)
    expect(value).toEqual({ env: { BAR: '2' }, nested: { a: 1 } })
    expect(deleteAtJsonPointer(value, '/env/MISSING')).toBe(false)
    expect(deleteAtJsonPointer(value, 'env/FOO')).toBe(false)
  })

  it('ProviderConfigMerge_DeepEqual_001', () => {
    expect(deepEqual({ a: 1, b: [1, 2] }, { b: [1, 2], a: 1 })).toBe(true)
    expect(deepEqual({ a: 1 }, { a: 2 })).toBe(false)
    expect(deepEqual([1, 2], [1, 2, 3])).toBe(false)
  })
})
