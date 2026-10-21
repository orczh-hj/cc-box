/**
 * 从本地 cc-switch 仓库快照同步 Provider 预设与图标资产。
 *
 * 用法（npm run sync:presets -- <mode>）：
 *   --zip <path>   cc-switch 仓库 zip（GitHub Code→Download ZIP，手动下载），
 *                  自动解压并同步预设 + 图标（推荐入口）
 *   --dir <path>   已解压的仓库目录（仓库根或包一层的解压目录均可，自动定位 src/）
 *   --file <path>  仅 claudeProviderPresets.ts 单文件（快速迭代转换规则，不碰图标）
 *
 * 示例：npm run sync:presets -- --zip ~/Downloads/cc-switch-main.zip
 *
 * 转换策略（见 scripts/lib/transform-presets.ts）：全量同步（排除 hidden）、
 * 剥离推广跟踪码、剔除 partner 赞助与 family 族字段。零联网。
 */
import { execSync } from 'node:child_process'
import { createHash } from 'node:crypto'
import fs from 'node:fs'
import os from 'node:os'
import path from 'node:path'
import { pathToFileURL, fileURLToPath } from 'node:url'
import { transformAll, serializePresets, type UpstreamPreset } from './lib/transform-presets'

const REPO = 'farion1231/cc-switch'
const ROOT = path.resolve(fileURLToPath(new URL('..', import.meta.url)))
const OUT_TS = path.join(ROOT, 'src/config/providerPresets.ts')
const OUT_META = path.join(ROOT, 'src/config/providerPresets.meta.json')
const ICON_DIR = path.join(ROOT, 'src/assets/icons/providers')

/** 临时工作目录（模块级：main 结束后统一清理） */
let tmp: string | null = null

function argValue(name: string): string | undefined {
  const eq = process.argv.find((a) => a.startsWith(`--${name}=`))
  if (eq) return eq.split('=').slice(1).join('=')
  const i = process.argv.indexOf(`--${name}`)
  if (i !== -1 && i + 1 < process.argv.length) return process.argv[i + 1]
  return undefined
}

/** 解压 zip（Windows: PowerShell Expand-Archive；unix: unzip） */
function extractZip(zip: string, dest: string) {
  fs.mkdirSync(dest, { recursive: true })
  if (process.platform === 'win32') {
    execSync(
      `powershell -NoProfile -Command "Expand-Archive -LiteralPath '${zip.replace(/'/g, "''")}' -DestinationPath '${dest.replace(/'/g, "''")}' -Force"`,
      { stdio: 'ignore' },
    )
  } else {
    execSync(`unzip -qo "${zip}" -d "${dest}"`, { stdio: 'ignore' })
  }
}

/** 定位仓库根（含 src/config/claudeProviderPresets.ts）；支持解压后包一层子目录 */
function locateRepoRoot(start: string): string {
  const candidates = [start, ...fs
    .readdirSync(start, { withFileTypes: true })
    .filter((d) => d.isDirectory())
    .map((d) => path.join(start, d.name))]
  for (const dir of candidates) {
    if (fs.existsSync(path.join(dir, 'src', 'config', 'claudeProviderPresets.ts'))) {
      return dir
    }
  }
  throw new Error(`在 ${start} 下未找到 src/config/claudeProviderPresets.ts——请确认是 cc-switch 仓库快照`)
}

async function main() {
  const zipArg = argValue('zip')
  const dirArg = argValue('dir')
  const fileArg = argValue('file')

  let upstreamPresets: UpstreamPreset[]
  let extractedDir: string | null = null

  if (zipArg || dirArg) {
    // 仓库快照模式：预设 + 图标全量同步
    let repoRoot: string
    if (zipArg) {
      const zip = path.resolve(zipArg)
      if (!fs.existsSync(zip)) throw new Error(`zip 不存在: ${zip}`)
      tmp = fs.mkdtempSync(path.join(os.tmpdir(), 'cc-box-sync-presets-'))
      console.log('[1/5] 解压 zip ...')
      extractZip(zip, path.join(tmp, 'x'))
      repoRoot = locateRepoRoot(path.join(tmp, 'x'))
    } else {
      repoRoot = locateRepoRoot(path.resolve(dirArg!))
    }
    const srcDir = path.join(repoRoot, 'src')
    extractedDir = path.join(srcDir, 'icons', 'extracted')
    const presetFile = path.join(srcDir, 'config', 'claudeProviderPresets.ts')
    console.log(`[2/5] 解析上游模块（${repoRoot}）...`)
    const mod = await import(pathToFileURL(presetFile).href)
    upstreamPresets = mod.providerPresets as UpstreamPreset[]
  } else if (fileArg) {
    // 单文件模式：只重跑转换（快速迭代）
    const mod = await import(pathToFileURL(path.resolve(fileArg)).href)
    upstreamPresets = mod.providerPresets as UpstreamPreset[]
  } else {
    throw new Error('缺少数据源：--zip <仓库zip> | --dir <仓库目录> | --file <claudeProviderPresets.ts>')
  }

  if (!Array.isArray(upstreamPresets) || upstreamPresets.length === 0) {
    throw new Error('上游 providerPresets 为空——数据源结构可能已变化，请人工检查')
  }
  console.log(`[3/5] 上游 ${upstreamPresets.length} 个预设，转换中 ...`)
  const presets = transformAll(upstreamPresets)

  // 图标复制：三级匹配（精确 stem → 归一化相等 → 归一化唯一前缀，上游文件
  // 名常有 -color/_icon 等后缀）；命中后重命名为 <icon>.<ext>（前端精确匹配）
  let iconCount = 0
  const iconMissing: string[] = []
  if (extractedDir && fs.existsSync(extractedDir)) {
    console.log('[4/5] 复制图标资产 ...')
    fs.mkdirSync(ICON_DIR, { recursive: true })
    // 清理旧资产（上游删掉的图标不残留）
    for (const old of fs.readdirSync(ICON_DIR)) {
      fs.rmSync(path.join(ICON_DIR, old), { recursive: true, force: true })
    }
    const norm = (s: string) => s.toLowerCase().replace(/[^a-z0-9]/g, '')
    const files = fs.readdirSync(extractedDir).filter((f) => /\.(svg|png)$/i.test(f))
    const byStem = new Map<string, string>()
    const byNorm = new Map<string, string>()
    for (const f of files) {
      const stem = f.replace(/\.(svg|png)$/i, '')
      // 同 stem 优先 svg（矢量、体积小）
      const better = !byStem.has(stem) || f.toLowerCase().endsWith('.svg')
      if (better) byStem.set(stem, f)
      if (!byNorm.has(norm(stem)) || f.toLowerCase().endsWith('.svg')) byNorm.set(norm(stem), f)
    }
    const wanted = new Set(presets.map((p) => p.icon).filter((i): i is string => !!i))
    for (const icon of wanted) {
      let src = byStem.get(icon)
      if (!src) src = byNorm.get(norm(icon))
      if (!src) {
        // 归一化前缀（候选以 icon 开头或 icon 以候选开头），唯一命中才采用
        const n = norm(icon)
        const hits = [...byNorm.entries()].filter(
          ([s]) => s.startsWith(n) || n.startsWith(s),
        )
        if (hits.length === 1) src = hits[0][1]
      }
      if (src) {
        const ext = src.toLowerCase().endsWith('.svg') ? '.svg' : '.png'
        fs.copyFileSync(path.join(extractedDir, src), path.join(ICON_DIR, icon + ext))
        iconCount++
      } else {
        iconMissing.push(icon)
      }
    }
  }

  console.log('[5/5] 生成 providerPresets.ts + meta ...')
  const header = `/**
 * Provider 预设配置模板 —— 生成物，勿手改
 * 上游：${REPO}（src/config/claudeProviderPresets.ts）
 * 生成：${new Date().toISOString()} | ${presets.length} 个条目（全量同步，排除 hidden；
 * 剥离推广跟踪码与 partner 赞助字段；CC-Box 自有 Custom 恒在尾部）
 * 重新生成：手动下载上游 zip 后 npm run sync:presets -- --zip <zip路径>
 * 转换规则：scripts/lib/transform-presets.ts
 */
`
  const body = `import type { ProviderPreset } from '@/types/provider'\n\nexport const providerPresets: ProviderPreset[] = ${serializePresets(presets)}\n`
  fs.writeFileSync(OUT_TS, header + body, 'utf-8')

  const meta = {
    source: REPO,
    generatedAt: new Date().toISOString(),
    count: presets.length,
    upstreamCount: upstreamPresets.length,
    iconCount,
    iconMissing,
    contentHash: createHash('sha256').update(body).digest('hex').slice(0, 16),
  }
  fs.writeFileSync(OUT_META, JSON.stringify(meta, null, 2) + '\n', 'utf-8')

  console.log(`完成：${presets.length} 个预设写入 src/config/providerPresets.ts`)
  console.log(`图标：${iconCount} 个复制${iconMissing.length ? `，${iconMissing.length} 个缺失回退字母头像（${iconMissing.join(', ')}）` : ''}`)
}

main()
  .catch((e) => {
    console.error('同步失败：', e)
    process.exit(1)
  })
  .finally(() => {
    if (tmp) fs.rmSync(tmp, { recursive: true, force: true })
  })
