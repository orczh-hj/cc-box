import { describe, it, expect, vi } from 'vitest'
import { patchClearTextureAtlas } from '@/utils/webglAtlasPatch'

/**
 * 构造一个带 clearTextureAtlas 方法的 mock prototype，
 * 模拟 @xterm/addon-webgl 的 WebglAddon.prototype 结构
 */
function makeMockProto(impl: (...args: unknown[]) => unknown): any {
  return {
    clearTextureAtlas(...args: unknown[]) {
      return impl.apply(this, args)
    },
  }
}

describe('patchClearTextureAtlas', () => {
  // 验证 patch 后原方法被调用，且回调被触发（模拟多 Tab 共享 atlas 时 #6014 的修复路径）
  it('PatchClearTextureAtlas_InvokesOriginalAndCallback_001', () => {
    const original = vi.fn(() => 'orig-result')
    const proto = makeMockProto(original)
    const onCleared = vi.fn()

    patchClearTextureAtlas(proto, onCleared)

    const instance = Object.create(proto)
    // eslint-disable-next-line @typescript-eslint/no-explicit-any
    const result = (instance as any).clearTextureAtlas('a', 'b')

    expect(original).toHaveBeenCalledTimes(1)
    expect(original).toHaveBeenCalledWith('a', 'b')
    expect(onCleared).toHaveBeenCalledTimes(1)
    expect(result).toBe('orig-result')
  })

  // 验证 this 绑定：patch 后 clearTextureAtlas 内部 this 指向调用实例
  it('PatchClearTextureAtlas_PreservesThisBinding_002', () => {
    let observedThis: unknown = null
    const proto = makeMockProto(function (this: unknown) {
      observedThis = this
      return 'ok'
    })
    patchClearTextureAtlas(proto, vi.fn())

    const instance = Object.create(proto)
    // eslint-disable-next-line @typescript-eslint/no-explicit-any
    ;(instance as any).clearTextureAtlas()

    expect(observedThis).toBe(instance)
  })

  // 验证幂等：同一 proto 重复 patch 只生效一次，原方法不会被多次包装
  it('PatchClearTextureAtlas_IdempotentSameProto_003', () => {
    const original = vi.fn()
    const proto = makeMockProto(original)

    const onClearedA = vi.fn()
    const onClearedB = vi.fn()
    patchClearTextureAtlas(proto, onClearedA)
    patchClearTextureAtlas(proto, onClearedB)

    const instance = Object.create(proto)
    // eslint-disable-next-line @typescript-eslint/no-explicit-any
    ;(instance as any).clearTextureAtlas()

    expect(original).toHaveBeenCalledTimes(1)
    expect(onClearedA).toHaveBeenCalledTimes(1)
    expect(onClearedB).not.toHaveBeenCalled()
  })

  // 验证回调抛错时不污染原方法语义：返回值正常透传，不向上传播
  it('PatchClearTextureAtlas_CallbackThrows_NotPropagated_004', () => {
    const proto = makeMockProto(() => 'ok')
    patchClearTextureAtlas(proto, () => {
      throw new Error('refresh failed')
    })

    const instance = Object.create(proto)
    // eslint-disable-next-line @typescript-eslint/no-explicit-any
    const result = (instance as any).clearTextureAtlas()
    expect(result).toBe('ok')
  })

  // 验证 proto 缺少 clearTextureAtlas 方法时不抛错（防御未来 addon API 变更）
  it('PatchClearTextureAtlas_MissingMethod_Noop_005', () => {
    const proto = { foo: 'bar' }
    expect(() => patchClearTextureAtlas(proto as any, vi.fn())).not.toThrow()
  })
})
