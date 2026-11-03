import { fileURLToPath } from 'node:url'
import { createHash } from 'node:crypto'
import { existsSync, readFileSync, readdirSync, statSync } from 'node:fs'
import vue from '@vitejs/plugin-vue'
import { defineConfig, type Plugin } from 'vite'
import { resolve } from 'node:path'
import { applyDocOrder } from './src/lib/refTree'
import { buildCorpusText, stripForSearch, type CorpusDoc } from './src/lib/search'

// 官网独立构建：与主应用 src/ 完全解耦，部署在 OSS bucket 根路径
const root = fileURLToPath(new URL('.', import.meta.url))

// Claude Code 官方文档中文版（本地由 npm run sync:cc-docs 完整更新），reference.html 分层浏览
const CC_DOCS_ROOT = resolve(root, 'content/claude-code-docs')
const VIRTUAL_MANIFEST = 'virtual:cc-docs-manifest'

interface ManifestEntry {
  key: string
  title: string
}

// 扫描文档目录 → 扁平清单（key = 相对路径去掉 .md；标题取首个 h1，缺失回退文件名）。
// 存在 order.json（update.mjs 生成的官方索引顺序）时按其排序，否则回退目录名字母序。
function scanDocsManifest(): ManifestEntry[] {
  const entries: ManifestEntry[] = []

  const docTitle = (file: string): string => {
    const content = readFileSync(file, 'utf8')
    const m = content.match(/^#\s+(.+)$/m)
    return m ? m[1].trim() : ''
  }

  const walk = (dir: string, prefix: string) => {
    const mds: string[] = []
    const subdirs: string[] = []
    for (const name of readdirSync(dir).sort()) {
      if (statSync(resolve(dir, name)).isDirectory()) subdirs.push(name)
      else if (name.endsWith('.md')) mds.push(name)
    }
    for (const name of mds) {
      const key = `${prefix}${name.replace(/\.md$/, '')}`
      entries.push({ key, title: docTitle(resolve(dir, name)) })
    }
    for (const name of subdirs) {
      walk(resolve(dir, name), `${prefix}${name}/`)
    }
  }

  walk(CC_DOCS_ROOT, '')

  const orderFile = resolve(CC_DOCS_ROOT, 'order.json')
  if (existsSync(orderFile)) {
    const order: string[] = JSON.parse(readFileSync(orderFile, 'utf8'))
    return applyDocOrder(entries, order)
  }
  return entries
}

function ccDocsManifestPlugin(): Plugin {
  return {
    name: 'cc-docs-manifest',
    resolveId(id) {
      if (id === VIRTUAL_MANIFEST) return `\0${VIRTUAL_MANIFEST}`
      return null
    },
    load(id) {
      if (id !== `\0${VIRTUAL_MANIFEST}`) return null
      const manifest = scanDocsManifest()
      return `export default ${JSON.stringify(manifest)}`
    },
  }
}

// ── 全文搜索语料 ────────────────────────────────────────────────────────────
// 构建期把全部文档（reference 228 篇 + docs 2 篇）剥 code/表格后拼成单个语料：
// - build：emitFile 固定内容 hash 文件名（文档更新 → 新 URL → 旧缓存自然失效）
// - dev：configureServer 中间件按同一 buildCorpusText 生成未压缩原文（两端逐字节一致）
const VIRTUAL_SEARCH = 'virtual:cc-docs-search'

function collectSearchCorpus(): string {
  const docs: CorpusDoc[] = []

  // reference 语料：复用 manifest 的遍历与标题规则，保证与侧边栏一致
  for (const entry of scanDocsManifest()) {
    docs.push({
      ty: 'reference',
      key: entry.key,
      title: entry.title || entry.key,
      body: stripForSearch(readFileSync(resolve(CC_DOCS_ROOT, `${entry.key}.md`), 'utf8')),
    })
  }

  // docs 语料：两篇使用文档，标题取首个 h1（无则用 key）
  const docTitle = (file: string, fallback: string) => {
    const m = readFileSync(file, 'utf8').match(/^#\s+(.+)$/m)
    return m ? m[1].trim() : fallback
  }
  for (const key of ['claude-code', 'cc-box']) {
    const file = resolve(root, `content/${key}.md`)
    if (existsSync(file)) {
      docs.push({
        ty: 'docs',
        key,
        title: docTitle(file, key),
        body: stripForSearch(readFileSync(file, 'utf8')),
      })
    }
  }

  return buildCorpusText(docs)
}

function ccDocsSearchPlugin(): Plugin {
  let isServe = false
  let devCorpus: string | null = null
  let prodUrl: string | null = null
  return {
    name: 'cc-docs-search',
    configResolved(config) {
      isServe = config.command === 'serve'
    },
    // build 期在 buildStart 无条件产出语料资产：即使尚无源码 import 虚拟模块，
    // 产物也必须存在（deploy 脚本按约定检测它）
    buildStart() {
      if (isServe) return
      const corpus = collectSearchCorpus()
      const hash = createHash('sha256').update(corpus).digest('hex').slice(0, 12)
      const fileName = `assets/search-corpus-${hash}.txt`
      // fileName 自带内容 hash，可安全长缓存；文档更新 → 新 URL → 旧缓存自然失效
      this.emitFile({ type: 'asset', fileName, source: corpus })
      prodUrl = `/${fileName}`
    },
    resolveId(id) {
      if (id === VIRTUAL_SEARCH) return `\0${VIRTUAL_SEARCH}`
      return null
    },
    load(id) {
      if (id !== `\0${VIRTUAL_SEARCH}`) return null
      const url = isServe ? '/__search-corpus.txt' : prodUrl
      return `export const corpusUrl = ${JSON.stringify(url)}`
    },
    configureServer(server) {
      server.middlewares.use((req, res, next) => {
        if (!req.url || !req.url.startsWith('/__search-corpus.txt')) return next()
        devCorpus ??= collectSearchCorpus()
        res.setHeader('Content-Type', 'text/plain; charset=utf-8')
        res.end(devCorpus)
      })
    },
  }
}

export default defineConfig({
  root,
  base: '/',
  plugins: [vue(), ccDocsManifestPlugin(), ccDocsSearchPlugin()],
  build: {
    outDir: 'dist',
    emptyOutDir: true,
    assetsDir: 'assets',
    rollupOptions: {
      input: {
        main: resolve(root, 'index.html'),
        docs: resolve(root, 'docs.html'),
        reference: resolve(root, 'reference.html'),
      },
    },
  },
  server: {
    port: 4173,
    // latest.json 与官网同 bucket 同源；本地 dev 经 proxy 转发，避免跨域
    proxy: {
      '/cc-box': {
        target: 'https://cc-box.oss-cn-beijing.aliyuncs.com',
        changeOrigin: true,
      },
    },
  },
})
