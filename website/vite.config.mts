import { fileURLToPath } from 'node:url'
import vue from '@vitejs/plugin-vue'
import { defineConfig } from 'vite'
import { resolve } from 'node:path'

// 官网独立构建：与主应用 src/ 完全解耦，部署在 OSS bucket 根路径
const root = fileURLToPath(new URL('.', import.meta.url))

export default defineConfig({
  root,
  base: '/',
  plugins: [vue()],
  build: {
    outDir: 'dist',
    emptyOutDir: true,
    assetsDir: 'assets',
    rollupOptions: {
      input: {
        main: resolve(root, 'index.html'),
        docs: resolve(root, 'docs.html'),
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
