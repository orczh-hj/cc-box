/**
 * 构建后安全网：扫描 dist 产物，阻止「esbuild 折叠未声明变量」的坏包进入发布。
 *
 * 由 `npm run build` 在 vite build 之后自动执行；Tauri beforeBuildCommand 与
 * relay 部署脚本都走该命令，因此桌面 / 手机 Web UI 共用同一道检查。
 *
 * 退出码：0 = 通过；1 = 发现坏产物或 dist 缺失。
 */
import { readdirSync, readFileSync, statSync } from 'node:fs'
import { join, relative, resolve } from 'node:path'
import { fileURLToPath } from 'node:url'
import { findUnsafeMinifiedAssignments } from './lib/dist-safety'

const ROOT = resolve(fileURLToPath(new URL('.', import.meta.url)), '..')
const DIST = join(ROOT, 'dist')

function listJsFiles(dir: string): string[] {
  const out: string[] = []
  for (const entry of readdirSync(dir)) {
    const full = join(dir, entry)
    const stat = statSync(full)
    if (stat.isDirectory()) out.push(...listJsFiles(full))
    else if (entry.endsWith('.js')) out.push(full)
  }
  return out
}

function fail(message: string): never {
  console.error(`[dist-safety] FAIL: ${message}`)
  process.exit(1)
}

let files: string[]
try {
  files = listJsFiles(DIST)
} catch (e) {
  // dist 未构建与被扫描目录的 IO 问题分开报，避免误导
  const reason = e instanceof Error ? e.message : String(e)
  fail(`cannot scan dist at ${DIST} (${reason}); run vite build first`)
}

if (files.length === 0) {
  fail('no .js chunks found under dist; refusing to pass an empty build')
}

let bad = 0
for (const file of files) {
  let code: string
  try {
    code = readFileSync(file, 'utf8')
  } catch (e) {
    const reason = e instanceof Error ? e.message : String(e)
    fail(`cannot read ${relative(ROOT, file)} (${reason})`)
  }
  const matches = findUnsafeMinifiedAssignments(code)
  if (matches.length === 0) continue
  bad += matches.length
  console.error(`[dist-safety] ${relative(ROOT, file)}: ${matches.length} unsafe assignment(s)`)
  for (const m of matches) {
    console.error(`  at ${m.index}: ${m.text}`)
    console.error(`  ...${m.context}...`)
  }
}

if (bad > 0) {
  fail(
    `${bad} unsafe minified assignment(s) found. ` +
      'This is the esbuild DCE bug that freezes xterm terminals; ' +
      'see scripts/lib/patch-xterm-enum.ts'
  )
}

console.log(`[dist-safety] OK — ${files.length} JS chunk(s) scanned`)
