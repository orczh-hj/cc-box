import { describe, it, expect, vi, afterEach } from 'vitest'
import { uuid } from '@/utils/uuid'

const V4_RE = /^[0-9a-f]{8}-[0-9a-f]{4}-4[0-9a-f]{3}-[89ab][0-9a-f]{3}-[0-9a-f]{12}$/

describe('uuid（非安全上下文回退）', () => {
  afterEach(() => {
    vi.unstubAllGlobals()
  })

  // 安全上下文：randomUUID 可用时直接使用
  it('Uuid_SecureContext_001', () => {
    const spy = vi.fn(() => 'aaaaaaaa-bbbb-4ccc-8ddd-eeeeeeeeeeee')
    vi.stubGlobal('crypto', { randomUUID: spy, getRandomValues: vi.fn() })
    expect(uuid()).toBe('aaaaaaaa-bbbb-4ccc-8ddd-eeeeeeeeeeee')
    expect(spy).toHaveBeenCalledOnce()
  })

  // 非安全上下文（局域网 http://IP）：randomUUID 不存在 → getRandomValues 回退，格式仍为 v4
  it('Uuid_InsecureContextFallback_002', () => {
    const realGetRandomValues = crypto.getRandomValues.bind(crypto)
    vi.stubGlobal('crypto', { getRandomValues: realGetRandomValues })
    const id = uuid()
    expect(id).toMatch(V4_RE)
  })

  // 回退路径随机性：多次生成不重复
  it('Uuid_FallbackUnique_003', () => {
    const realGetRandomValues = crypto.getRandomValues.bind(crypto)
    vi.stubGlobal('crypto', { getRandomValues: realGetRandomValues })
    const ids = new Set(Array.from({ length: 100 }, () => uuid()))
    expect(ids.size).toBe(100)
  })
})
