/**
 * 生产产物安全检查：识别 esbuild 语法压缩/DCE 缺陷留下的「未声明变量赋值」。
 *
 * 典型坏产物（v0.16.4 终端卡死根因）：
 *   requestMode(t,e){var p;(v=>(...))(void 0||(i={}));...
 * esbuild 把未初始化的局部变量折叠成 `void 0` 后又删掉了它的 `let` 声明，
 * 留下 `i = {}` —— ESM 严格模式下运行时抛 ReferenceError，
 * 且异常发生在 xterm 写队列内部，会让该终端永久停止渲染。
 *
 * 见 scripts/lib/patch-xterm-enum.ts 的修复与本文件被 verify-dist-safety.ts 使用。
 */

/**
 * 匹配特征产物 `void 0 || (x = {})`（允许无括号/空白差异）。
 * 负向后顾排除 `e === void 0 || (...)` 这类合法表达式。
 *
 * 注意：这是**针对本次根因形态**的检查，不是通用「未声明变量」检测器。
 * 已知未覆盖的变体：`??=` 折叠（`void 0 ?? (x = ...)`）、折叠成
 * `Object.create(null)` / `[]` 等非 `{}` 初值。出现新变体时应扩展本正则。
 */
export const UNSAFE_MINIFIED_ASSIGN_RE =
  /(?<![=!<>])void 0\s*\|\|\s*\(?\s*[A-Za-z_$][\w$]*\s*=\s*\{\s*\}\s*\)?/g

export interface UnsafeMatch {
  index: number
  text: string
  context: string
}

export function findUnsafeMinifiedAssignments(
  code: string,
  contextRadius = 80
): UnsafeMatch[] {
  const matches: UnsafeMatch[] = []
  UNSAFE_MINIFIED_ASSIGN_RE.lastIndex = 0
  let m: RegExpExecArray | null
  while ((m = UNSAFE_MINIFIED_ASSIGN_RE.exec(code)) !== null) {
    const start = Math.max(0, m.index - contextRadius)
    const end = Math.min(code.length, m.index + m[0].length + contextRadius)
    matches.push({
      index: m.index,
      text: m[0],
      context: code.slice(start, end),
    })
    // 防御零宽匹配死循环（当前正则不会，但保持扫描器健壮）
    if (m[0].length === 0) UNSAFE_MINIFIED_ASSIGN_RE.lastIndex += 1
  }
  return matches
}
