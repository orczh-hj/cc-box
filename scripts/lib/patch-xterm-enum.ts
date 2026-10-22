import type { Plugin } from 'vite'

/**
 * 修复 xterm 死枚举 IIFE 触发的 esbuild 生产压缩缺陷。
 *
 * 背景（2026-10，v0.16.4 终端卡死）：@xterm/xterm 的 lib/*.mjs 由 TS enum
 * 编译出的 IIFE 形如
 *   let r; (te => (te[te.NOT_RECOGNIZED = 0] = 'NOT_RECOGNIZED', ...))(r ||= {})
 * 枚举对象建好后从未被读取，属于死代码。生产构建 target 含 safari13，
 * esbuild 会先把 `||=` 降级为 `|| (r = {})`；随后语法压缩把未初始化的 r
 * 读到的地方折叠成 `void 0`，并因为「r 已无读取」删掉 `let r` 声明，
 * 最终产出 `(void 0 || (i = {}))` —— 对未声明标识符赋值。
 * ESM 严格模式下这行直接抛 ReferenceError。
 *
 * 该异常发生在 xterm WriteBuffer 的解析回调里（_innerWrite → parser.parse），
 * 抛错会跳过 _bufferOffset 前进与后续调度：之后 term.write() 只入队不处理，
 * 输出/回显/resize 重绘全部消失，表现为终端「卡住」。
 *
 * 改写为等价的 `|| (x = {})` 后，esbuild 不需要降级 `||=`，也就不会走到
 * 这条有缺陷的折叠路径。语义完全一致（x 在该点必然未定义）。
 *
 * 覆盖所有 @xterm/* 包（当前 xterm / addon-webgl / addon-unicode11 均含该模式）；
 * xterm 升级后是否仍需要本补丁，由 tests/scripts/xterm-bundle-safety.test.ts
 * 的命中数断言守护（依赖里不再有该模式时测试失败，提示复核并移除）。
 */

/** 死枚举实参形态：`)(<ident>||={})` */
const DEAD_ENUM_ARG_RE = /\)\(\s*([A-Za-z_$][\w$]*)\s*\|\|=\s*\{\}\s*\)/g

/** 把源码里的 `||=` 死枚举实参改写成等价的 `|| (x = {})` */
export function patchXtermDeadEnum(code: string): { code: string; count: number } {
  let count = 0
  const patched = code.replace(DEAD_ENUM_ARG_RE, (_match: string, name: string) => {
    count += 1
    return `)(${name}||(${name}={}))`
  })
  return { code: patched, count }
}

export function patchXtermDeadEnumPlugin(): Plugin {
  // 整个构建的累计命中数：为 0 说明补丁已失效（依赖形态变化），需要人工复核
  let totalPatched = 0

  return {
    name: 'cc-box:patch-xterm-dead-enum',
    // 只在生产构建生效：dev 不压缩，不会触发该缺陷
    apply: 'build',
    // 必须早于 Vite 的 esbuild 转译，插件才能看到原始 `||=`
    enforce: 'pre',
    transform(code, id) {
      const cleanId = id.split('?')[0]
      if (!/@xterm\/[^/]+\/lib\/[^/]+\.mjs$/.test(cleanId)) return null

      const { code: patched, count } = patchXtermDeadEnum(code)
      // 单个 @xterm 包无命中是正常情况（多数 addon 不含该模式）
      if (count === 0) return null
      totalPatched += count
      this.info(`[cc-box] patched ${count} dead enum IIFE(s) in ${cleanId}`)
      return { code: patched, map: null }
    },
    buildEnd() {
      if (totalPatched === 0) {
        this.warn(
          '[cc-box] xterm dead-enum workaround matched nothing in this build; ' +
            'the upstream pattern may have changed — review scripts/lib/patch-xterm-enum.ts'
        )
      }
    },
  }
}
