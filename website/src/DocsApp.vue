<script setup lang="ts">
import { onMounted, onUnmounted } from 'vue'
import ccBoxMd from '../content/cc-box.md?raw'
import claudeCodeMd from '../content/claude-code.md?raw'
import DocsToc from './components/DocsToc.vue'
import SiteFooter from './components/SiteFooter.vue'
import SiteHeader from './components/SiteHeader.vue'
import { renderMarkdown } from './lib/markdown'
import { setupScrollSpy } from './lib/toc'

// 文档正文存于 website/content/*.md，此处渲染并从 h2 生成目录
const docs = [
  { title: 'Claude Code', ...renderMarkdown(claudeCodeMd) },
  { title: 'CC-Box', ...renderMarkdown(ccBoxMd) },
]

const tocGroups = docs.map((d) => ({ title: d.title, items: d.toc }))

let teardown: (() => void) | undefined

onMounted(() => {
  teardown = setupScrollSpy(tocGroups.flatMap((g) => g.items.map((i) => i.id)))

  // 内容为客户端渲染，浏览器加载时的 hash 定位会因元素尚不存在而失效，挂载后补一次
  const hash = decodeURIComponent(location.hash.slice(1))
  if (hash) {
    document.getElementById(hash)?.scrollIntoView({ behavior: 'auto', block: 'start' })
  }
})

onUnmounted(() => teardown?.())
</script>

<template>
  <SiteHeader page="docs" />
  <main class="section-alt">
    <div class="container doc-page">
      <h1 class="doc-title">使用文档</h1>
      <p class="doc-lead">Claude Code 基本用法与 CC-Box 使用指南。</p>
      <div class="doc-layout">
        <DocsToc :groups="tocGroups" />
        <div class="doc-content">
          <div v-for="d in docs" :key="d.title" class="md-body" v-html="d.html"></div>
        </div>
      </div>
    </div>
  </main>
  <SiteFooter />
</template>

<style scoped>
.doc-title {
  font-size: 32px;
  font-weight: 800;
  text-align: center;
  margin-bottom: 8px;
}

.doc-lead {
  text-align: center;
  font-size: 15px;
  color: var(--text-secondary);
  margin-bottom: 48px;
}

.doc-layout {
  display: grid;
  grid-template-columns: 200px minmax(0, 1fr);
  gap: 48px;
}

.doc-content {
  max-width: 780px;
  min-width: 0;
}

@media (max-width: 900px) {
  .doc-layout {
    grid-template-columns: minmax(0, 1fr);
  }

  .doc-layout > nav {
    display: none;
  }
}

@media (max-width: 768px) {
  .doc-page {
    padding-top: 32px;
    padding-bottom: 56px;
  }

  .doc-title {
    font-size: 26px;
  }

  .doc-lead {
    font-size: 14px;
    margin-bottom: 32px;
  }
}
</style>
