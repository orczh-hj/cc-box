import { fileURLToPath } from 'node:url'
import { existsSync, readFileSync, readdirSync, statSync } from 'node:fs'
import vue from '@vitejs/plugin-vue'
import { defineConfig, type Plugin } from 'vite'
import { resolve } from 'node:path'
import { applyDocOrder } from './src/lib/refTree'

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

export default defineConfig({
  root,
  base: '/',
  plugins: [vue(), ccDocsManifestPlugin()],
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
