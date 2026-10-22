// @vitest-environment node
// 本文件要用 esbuild 做真实压缩；jsdom 环境的 TextEncoder 与 Node 不一致会触发 esbuild 不变量
import { describe, it, expect } from 'vitest'
import { readFileSync } from 'node:fs'
import { resolve } from 'node:path'
import { transformWithEsbuild } from 'vite'
import { patchXtermDeadEnum } from '../../scripts/lib/patch-xterm-enum'
import { findUnsafeMinifiedAssignments } from '../../scripts/lib/dist-safety'

// 与 vite.config.ts 的生产压缩配置保持一致（safari13 是触发 esbuild 降级的关键）
const BUILD_OPTIONS = {
  loader: 'js',
  target: ['es2021', 'chrome100', 'safari13'],
  minify: true,
  treeShaking: true,
  format: 'esm',
} as const

/** 生产构建同款压缩（Vite 内部即走 esbuild transform） */
async function minifyLikeProduction(code: string): Promise<string> {
  const result = await transformWithEsbuild(code, 'xterm.mjs', BUILD_OPTIONS as never)
  return result.code
}

const XTERM_MJS = resolve('node_modules/@xterm/xterm/lib/xterm.mjs')
const ADDON_WEBGL_MJS = resolve('node_modules/@xterm/addon-webgl/lib/addon-webgl.mjs')

// 函数作用域 + 变量后续不再读取，才会触发 esbuild 删 let 声明的折叠路径；
// 顶层声明或导出的函数不会复现（导出保证 treeShaking 不删掉整个函数）
const DEAD_ENUM_FIXTURE = 'export function f(){let r;(x=>(x[x.A=0]="A"))(r||={});}'

describe('patchXtermDeadEnum（||= 死枚举改写）', () => {
  it('XtermDeadEnumPatch_Fixture_001', async () => {
    // 未改写：safari13 降级 + 语法压缩后产生对未声明变量的赋值
    const unpatched = await minifyLikeProduction(DEAD_ENUM_FIXTURE)
    expect(findUnsafeMinifiedAssignments(unpatched).length).toBe(1)

    // 改写后：||= 变成等价的 || (x = {})，不再触发折叠
    const { code, count } = patchXtermDeadEnum(DEAD_ENUM_FIXTURE)
    expect(count).toBe(1)
    expect(code).toContain('(r||(r={}))')
    expect(code).not.toContain('||=')

    const patched = await minifyLikeProduction(code)
    expect(findUnsafeMinifiedAssignments(patched)).toEqual([])
  })

  it('XtermDeadEnumPatch_Idempotent_001', () => {
    // 已改写源码上重复执行不应再次命中
    const once = patchXtermDeadEnum(DEAD_ENUM_FIXTURE)
    const twice = patchXtermDeadEnum(once.code)
    expect(twice.count).toBe(0)
    expect(twice.code).toBe(once.code)
  })
})

describe('patchXtermDeadEnum（真实依赖）', () => {
  it('XtermDeadEnumPatch_RealDeps_001', async () => {
    for (const file of [XTERM_MJS, ADDON_WEBGL_MJS]) {
      const source = readFileSync(file, 'utf8')
      const { code, count } = patchXtermDeadEnum(source)
      // xterm 升级后若移除该模式，这里会失败：提示复核补丁是否还需要
      expect(count, `${file} 中未找到死枚举 IIFE，请复核补丁`).toBeGreaterThan(0)
      expect(code).not.toContain('||={}')

      const minified = await minifyLikeProduction(code)
      expect(findUnsafeMinifiedAssignments(minified), `${file} 压缩后仍有坏赋值`).toEqual([])
    }
  })

  it('XtermDeadEnumPatch_RealDeps_002', async () => {
    // 当前 xterm 版本未改写时确实会产生坏赋值（守护本补丁的真实必要性）
    const minified = await minifyLikeProduction(readFileSync(XTERM_MJS, 'utf8'))
    expect(findUnsafeMinifiedAssignments(minified).length).toBeGreaterThan(0)
  })
})

describe('findUnsafeMinifiedAssignments（产物安全扫描）', () => {
  it('DistSafety_Detector_001', () => {
    expect(findUnsafeMinifiedAssignments(')))(void 0||(i={}))').length).toBe(1)
    expect(findUnsafeMinifiedAssignments('a=1,void 0||(y={})').length).toBe(1)
    expect(findUnsafeMinifiedAssignments('void 0||(z={})').length).toBe(1)
  })

  it('DistSafety_Detector_002', () => {
    // 合法表达式：=== void 0 后的短路，不应误报
    expect(findUnsafeMinifiedAssignments('if(e===void 0||(K=this._search(e),K===-1)){}')).toEqual([])
    expect(findUnsafeMinifiedAssignments('o!==void 0||(o=TA(o))')).toEqual([])
    // 对象初始化也要被负向后顾挡住：否则删除 lookbehind 时上面的用例仍会通过
    expect(findUnsafeMinifiedAssignments('if(e!==void 0||(K={})){}')).toEqual([])
    expect(findUnsafeMinifiedAssignments('(r||(r={}))')).toEqual([])
  })
})
