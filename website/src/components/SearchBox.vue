<script setup lang="ts">
import { ref } from 'vue'
import { corpusUrl } from 'virtual:cc-docs-search'
import { buildJumpUrl, makeSnippet, parseCorpus, searchCorpus, type SearchHit } from '../lib/search'

// 站内全文搜索：首次输入才 fetch 语料（模块级缓存，同页多次开合只拉一次）
let corpusPromise: Promise<{ corpus: string; sections: ReturnType<typeof parseCorpus>['sections'] }> | null =
  null

function loadCorpus() {
  corpusPromise ??= fetch(corpusUrl)
    .then((r) => {
      if (!r.ok) throw new Error(`HTTP ${r.status}`)
      return r.text()
    })
    .then((text) => parseCorpus(text))
  return corpusPromise
}

const query = ref('')
const hits = ref<SearchHit[]>([])
const activeIndex = ref(-1)
const state = ref<'idle' | 'loading' | 'ready' | 'error'>('idle')
let debounceTimer: ReturnType<typeof setTimeout> | undefined

async function runSearch() {
  const q = query.value.trim()
  if ([...q].length < 2) {
    hits.value = []
    activeIndex.value = -1
    state.value = 'idle'
    return
  }
  state.value = 'loading'
  try {
    const { corpus, sections } = await loadCorpus()
    hits.value = searchCorpus(corpus, sections, q)
    activeIndex.value = -1
    state.value = 'ready'
  } catch {
    corpusPromise = null // 失败允许重试
    state.value = 'error'
  }
}

function onInput() {
  clearTimeout(debounceTimer)
  debounceTimer = setTimeout(runSearch, 150)
}

function moveActive(delta: number) {
  if (!hits.value.length) return
  activeIndex.value = (activeIndex.value + delta + hits.value.length) % hits.value.length
}

// 摘要中的 «关键词» 渲染为 <mark>（语料本身是纯文本，无注入面）
function renderSnippet(snippet: string) {
  const esc = (s: string) => s.replace(/&/g, '&amp;').replace(/</g, '&lt;').replace(/>/g, '&gt;')
  return esc(snippet)
    .replace(/«([^»]*)»/g, '<mark>$1</mark>')
    .replace(/\s+/g, ' ')
}

function go(hit: SearchHit) {
  location.href = buildJumpUrl(hit)
}

function onKeydown(e: KeyboardEvent) {
  if (e.key === 'ArrowDown') {
    e.preventDefault()
    moveActive(1)
  } else if (e.key === 'ArrowUp') {
    e.preventDefault()
    moveActive(-1)
  } else if (e.key === 'Enter') {
    const hit = hits.value[activeIndex.value] ?? hits.value[0]
    if (hit) go(hit)
  } else if (e.key === 'Escape') {
    // 收起结果并清空输入（浮层由 SiteHeader 的点击外部监听负责关闭）
    clearTimeout(debounceTimer)
    query.value = ''
    hits.value = []
    activeIndex.value = -1
    state.value = 'idle'
    ;(e.target as HTMLInputElement).blur()
  }
}
</script>

<template>
  <div class="search-box">
    <input
      v-model="query"
      class="search-input"
      type="search"
      placeholder="搜索文档…"
      @input="onInput"
      @keydown="onKeydown"
    />
    <div v-if="state === 'loading' && !hits.length" class="search-tip">加载搜索索引…</div>
    <div v-else-if="state === 'error'" class="search-tip search-tip-error">
      搜索暂不可用，
      <a href="#" @click.prevent="runSearch">重试</a>
    </div>
    <div v-else-if="state === 'ready' && !hits.length" class="search-tip">没有匹配结果，换个关键词试试</div>
    <ul v-else-if="hits.length" class="search-results">
      <li
        v-for="(hit, i) in hits"
        :key="hit.ty + hit.key + hit.sectionTitle"
        class="search-result"
        :class="{ active: i === activeIndex }"
        @click="go(hit)"
        @mousemove="activeIndex = i"
      >
        <div class="search-result-title">{{ hit.docTitle }} › {{ hit.sectionTitle }}</div>
        <div class="search-result-snippet" v-html="renderSnippet(hit.snippet)"></div>
      </li>
    </ul>
  </div>
</template>

<style scoped>
.search-box {
  position: relative;
  width: 100%;
}

.search-input {
  width: 100%;
  padding: 7px 12px;
  border: 1px solid var(--border-color);
  border-radius: var(--radius-lg);
  background: var(--bg-primary);
  font-size: 13.5px;
  font-family: var(--font-sans);
  color: var(--text-primary);
  outline: none;
}

.search-input:focus {
  border-color: var(--accent-secondary);
}

.search-results,
.search-tip {
  position: absolute;
  top: calc(100% + 6px);
  left: 0;
  right: 0;
  max-height: 60vh;
  overflow-y: auto;
  margin: 0;
  padding: 6px;
  list-style: none;
  background: var(--bg-primary);
  border: 1px solid var(--border-light);
  border-radius: var(--radius-lg);
  box-shadow: var(--shadow-lg);
  z-index: 200;
}

.search-tip {
  padding: 10px 12px;
  font-size: 13px;
  color: var(--text-tertiary);
}

.search-tip-error {
  color: var(--status-error);
}

.search-result {
  padding: 8px 10px;
  border-radius: var(--radius-md);
  cursor: pointer;
}

.search-result.active {
  background: var(--bg-hover);
}

.search-result-title {
  font-size: 13.5px;
  font-weight: 600;
  color: var(--text-primary);
}

.search-result-snippet {
  margin-top: 2px;
  font-size: 12.5px;
  color: var(--text-tertiary);
  line-height: 1.5;
  display: -webkit-box;
  -webkit-line-clamp: 2;
  -webkit-box-orient: vertical;
  overflow: hidden;
}

.search-result-snippet :deep(mark) {
  background: var(--accent-gold-light);
  color: inherit;
  border-radius: 2px;
  padding: 0 1px;
}
</style>
