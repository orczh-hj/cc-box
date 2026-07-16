/**
 * xtermjs/xterm.js#6014 的本地 workaround。
 *
 * 多个配置相同的 Terminal 实例会共享同一个 WebGL glyph atlas（见
 * CharAtlasCache.acquireTextureAtlas）。当任一 Terminal 调用
 * clearTextureAtlas() 时，TextureAtlas.clearTexture() 会清空共享 atlas，
 * 但不会把 _requestClearModel 置位；其他共享 atlas 的 renderer 下一帧
 * beginFrame() 返回 false，沿用旧的顶点 / 纹理坐标，但这些坐标已经指向
 * 被清空或重新分配的 atlas 区域 → 渲染出乱码或空白。
 *
 * 本工具在运行时 monkey-patch WebglAddon.prototype.clearTextureAtlas，
 * 调用原方法后回调 onCleared，由调用方触发所有共享 atlas 的 Terminal
 * 全量 refresh，模拟 _requestClearModel 应有的传播。
 *
 * 设计为纯函数 + 依赖注入：proto 和 onCleared 都由调用方传入，
 * 便于在 jsdom 环境下用 mock proto 做单元测试。
 */

const PATCHED = Symbol('cc-box-clearTextureAtlas-patched')

/**
 * 为给定 prototype 打 patch。幂等：同一 prototype 重复调用只生效一次。
 *
 * @param proto  目标 prototype（通常是 WebglAddon.prototype）
 * @param onCleared  原 clearTextureAtlas 执行完毕后触发的回调，
 *                   调用方在此遍历所有共享 atlas 的 Terminal 调用 refresh
 */
export function patchClearTextureAtlas(
  proto: { clearTextureAtlas: (...args: unknown[]) => void },
  onCleared: () => void,
): void {
  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  const p = proto as any
  if (p[PATCHED]) return
  const orig = p.clearTextureAtlas
  if (typeof orig !== 'function') return
  p[PATCHED] = true
  p.clearTextureAtlas = function (this: unknown, ...args: unknown[]) {
    const result = orig.apply(this, args)
    try {
      onCleared()
    } catch {
      // 回调失败不能阻塞原方法语义
    }
    return result
  }
}
