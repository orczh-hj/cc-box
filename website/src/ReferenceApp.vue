<script setup lang="ts">
import { computed, nextTick, onMounted, onUnmounted, ref, watch } from 'vue'
import manifest from 'virtual:cc-docs-manifest'
import SiteFooter from './components/SiteFooter.vue'
import SiteHeader from './components/SiteHeader.vue'
import { renderMarkdown, stripLeadingBlockquote, type TocEntry } from './lib/markdown'
import { buildRefTree, type RefCategory } from './lib/refTree'
import { enhanceBlocks } from './lib/tabs'

// 文档正文按需懒加载：每篇 md 一个独立 chunk，首屏只带清单
const rawDocs = import.meta.glob('../content/claude-code-docs/**/*.md', {
  query: '?raw',
  import: 'default',
}) as Record<string, () => Promise<string>>

const tree = buildRefTree(manifest)

const DEFAULT_DOC = 'getting-started/overview'
const firstKey = manifest[0]?.key ?? ''
const defaultKey = manifest.some((e) => e.key === DEFAULT_DOC) ? DEFAULT_DOC : firstKey

const currentKey = ref(defaultKey)
const html = ref('')
const toc = ref<TocEntry[]>([])
const loading = ref(false)
const loadError = ref('')
const mdBodyEl = ref<HTMLElement | null>(null)

// 目录树展开状态：类可折叠；组（三级索引）始终展开
const expandedCats = ref<Set<string>>(new Set())
const sidebarOpen = ref(false)

const currentTitle = computed(
  () => manifest.find((e) => e.key === currentKey.value)?.title ?? currentKey.value,
)

function openDoc(key: string, anchor?: { id: string; text: string }) {
  pendingAnchor = anchor
  if (key !== currentKey.value) location.hash = `#/${key}`
  else scrollToAnchor(anchor)
  sidebarOpen.value = false
}

let pendingAnchor: { id: string; text: string } | undefined

// 官方中文文档的站内锚点是英文 slug（如 #work-effectively-with-claude-code），
// 而标题渲染出的是中文 id——直接 getElementById 必然落空。
// 兜底：用链接文字与各级标题做归一化包含匹配（官方链接文字通常就是标题原文，可能带前后缀）。
function resolveAnchor(anchor: string, linkText?: string): HTMLElement | null {
  const byId = document.getElementById(anchor)
  if (byId) return byId
  if (!linkText) return null

  const norm = (s: string) =>
    s
      .toLowerCase()
      .replace(/[\s\d\p{P}\p{S}]+/gu, '')
  const target = norm(linkText)
  if (!target) return null

  const headings = [...document.querySelectorAll<HTMLElement>('.md-body h1, .md-body h2, .md-body h3')]
  let best: HTMLElement | null = null
  let bestLen = 0
  for (const h of headings) {
    const t = norm(h.textContent ?? '')
    if (!t) continue
    if ((target.includes(t) || t.includes(target)) && t.length > bestLen) {
      best = h
      bestLen = t.length
    }
  }
  return best
}

function scrollToAnchor(anchor?: { id: string; text: string }) {
  if (!anchor) {
    window.scrollTo({ top: 0 })
    return
  }
  const el = resolveAnchor(anchor.id, anchor.text)
  if (el) el.scrollIntoView({ behavior: 'smooth', block: 'start' })
}

function expandTo(key: string) {
  expandedCats.value.add(key.split('/')[0])
}

function toggleCat(cat: RefCategory) {
  const set = new Set(expandedCats.value)
  set.has(cat.name) ? set.delete(cat.name) : set.add(cat.name)
  expandedCats.value = set
}

// 站内链接内部导航：拦截 code.claude.com/docs/(zh-CN|en)/<slug>.md 形式的链接，
// 按末段 slug 在清单中定位本地文档并跳转（保留 #锚点，带链接文字做兜底匹配）；
// 同页锚点链接（#xxx，官方用英文 slug）也拦截并兜底；其余外链新标签打开
function onContentClick(e: MouseEvent) {
  const a = (e.target as HTMLElement).closest('a')
  if (!a) return
  const url = new URL(a.href, location.origin)
  const anchorId = decodeURIComponent(url.hash.slice(1))
  const anchor = anchorId ? { id: anchorId, text: a.textContent ?? '' } : undefined
  if (url.hostname === 'code.claude.com') {
    const m = url.pathname.match(/^\/docs\/(?:zh-CN\/|en\/)?([a-z0-9-]+)\.md$/)
    const hit = m && manifest.find((en) => en.key === m[1] || en.key.endsWith(`/${m[1]}`))
    if (hit) {
      e.preventDefault()
      openDoc(hit.key, anchor)
    }
  } else if (url.hostname === location.hostname && url.pathname === location.pathname && anchor) {
    // 同页锚点：默认跳转只认 id，英文 slug 会静默失效，统一走 resolveAnchor
    e.preventDefault()
    scrollToAnchor(anchor)
  } else if (url.hostname !== location.hostname) {
    a.target = '_blank'
    a.rel = 'noopener'
  }
}

let loadSeq = 0

async function loadDoc(key: string) {
  const loader = rawDocs[`../content/claude-code-docs/${key}.md`]
  if (!loader) {
    html.value = ''
    toc.value = []
    loadError.value = '文档不存在'
    return
  }

  const seq = ++loadSeq
  loading.value = true
  loadError.value = ''
  try {
    const md = stripLeadingBlockquote(await loader())
    if (seq !== loadSeq) return
    const rendered = renderMarkdown(md)
    html.value = rendered.html
    toc.value = rendered.toc
    document.title = `${currentTitle.value} — Claude Code 文档 — CC-Box`
  } catch (err) {
    if (seq !== loadSeq) return
    loadError.value = err instanceof Error ? err.message : String(err)
  } finally {
    if (seq === loadSeq) loading.value = false
  }

  // 必须等 loading 清除、md-body 进入 DOM 后才能折叠 tabs/callout/accordion
  // （loading=true 时渲染的是「加载中」分支，mdBodyEl 尚为 null）
  if (seq === loadSeq && !loadError.value) {
    await nextTick()
    if (seq === loadSeq && mdBodyEl.value) enhanceBlocks(mdBodyEl.value)
  }
}

function syncFromHash() {
  const hash = decodeURIComponent(location.hash.slice(1))
  if (hash.startsWith('/')) {
    const key = hash.slice(1).replace(/\/+$/, '')
    if (key && key !== currentKey.value) {
      currentKey.value = key
    }
  }
}

function onHashChange() {
  const hash = decodeURIComponent(location.hash.slice(1))
  if (hash.startsWith('/')) {
    syncFromHash()
  } else if (hash) {
    // 非路由 hash = 页内锚点（中文 id），挂载后内容已就绪则直接跳
    document.getElementById(hash)?.scrollIntoView({ behavior: 'smooth', block: 'start' })
  }
}

watch(currentKey, (key) => {
  expandTo(key)
  loadDoc(key).then(() => {
    const anchor = pendingAnchor
    pendingAnchor = undefined
    nextTick(() => scrollToAnchor(anchor))
  })
})

onMounted(() => {
  syncFromHash()
  expandTo(currentKey.value)
  loadDoc(currentKey.value)
  window.addEventListener('hashchange', onHashChange)
})

onUnmounted(() => window.removeEventListener('hashchange', onHashChange))
</script>

<template>
  <SiteHeader page="reference" />
  <main class="section-alt">
    <div class="container ref-page">
      <div class="ref-head">
        <h1 class="ref-title">Claude Code 文档</h1>
        <p class="ref-lead">
          Claude Code 官方文档中文版 ·<a
            class="ref-origin-link"
            href="https://code.claude.com/docs/en/sub-agents"
            target="_blank"
            rel="noopener"
            >原文链接</a
          >
        </p>
        <button class="ref-nav-toggle" @click="sidebarOpen = !sidebarOpen">
          {{ sidebarOpen ? '收起目录' : '展开目录' }}
        </button>
      </div>
      <div class="ref-layout" :class="{ 'sidebar-open': sidebarOpen }">
        <nav class="ref-sidebar">
          <div v-for="cat in tree" :key="cat.name" class="ref-cat">
            <button class="ref-cat-header" @click="toggleCat(cat)">
              <span class="ref-caret" :class="{ open: expandedCats.has(cat.name) }">▸</span>
              <span>{{ cat.label }}</span>
            </button>
            <div v-if="expandedCats.has(cat.name)" class="ref-cat-body">
              <template v-for="group in cat.groups" :key="group.name">
                <div v-if="group.label" class="ref-group-label">{{ group.label }}</div>
                <button
                  v-for="doc in group.docs"
                  :key="doc.key"
                  class="ref-doc-link"
                  :class="{ active: doc.key === currentKey, grouped: !!group.label }"
                  @click="openDoc(doc.key)"
                >
                  {{ doc.title }}
                </button>
              </template>
            </div>
          </div>
        </nav>
        <div class="ref-content" @click="onContentClick">
          <div v-if="loading" class="ref-loading">加载中…</div>
          <div v-else-if="loadError" class="ref-error">加载失败：{{ loadError }}</div>
          <template v-else>
            <div v-if="toc.length" class="ref-page-toc">
              <span class="ref-page-toc-label">本页：</span>
              <a
                v-for="item in toc"
                :key="item.id"
                :href="'#' + encodeURIComponent(item.id)"
                class="ref-page-toc-item"
              >
                {{ item.label }}
              </a>
            </div>
            <div ref="mdBodyEl" class="md-body" v-html="html"></div>
          </template>
        </div>
      </div>
    </div>
  </main>
  <SiteFooter />
</template>

<style scoped>
.ref-page {
  padding-top: 40px;
  padding-bottom: 64px;
}

.ref-head {
  text-align: center;
  margin-bottom: 36px;
}

.ref-title {
  font-size: 32px;
  font-weight: 800;
  margin-bottom: 8px;
}

.ref-lead {
  font-size: 15px;
  color: var(--text-secondary);
}

.ref-origin-link {
  margin-left: 4px;
  color: var(--accent-primary);
}

.ref-nav-toggle {
  display: none;
}

.ref-layout {
  display: grid;
  grid-template-columns: 250px minmax(0, 1fr);
  gap: 48px;
}

.ref-sidebar {
  position: sticky;
  top: 76px;
  align-self: start;
  max-height: calc(100vh - 96px);
  overflow-y: auto;
  font-size: 13px;
  padding-right: 8px;
  scrollbar-width: thin;
  scrollbar-color: var(--border-light) transparent;
}

/* 纤细滚动条：默认近乎隐形，悬停才显色，不与索引文字抢视觉 */
.ref-sidebar::-webkit-scrollbar {
  width: 4px;
}

.ref-sidebar::-webkit-scrollbar-track {
  background: transparent;
}

.ref-sidebar::-webkit-scrollbar-thumb {
  background: transparent;
  border-radius: 2px;
}

.ref-sidebar:hover::-webkit-scrollbar-thumb {
  background: var(--border-light);
}

.ref-sidebar:hover::-webkit-scrollbar-thumb:hover {
  background: var(--text-tertiary);
}

.ref-cat:not(:last-child) {
  margin-bottom: 20px;
}

/* 一级：类标题——与「文档」页 toc-group-title 同款（小号粗体弱色 + 字距），可折叠 */
.ref-cat-header {
  display: flex;
  align-items: center;
  gap: 6px;
  width: 100%;
  padding: 0 0 4px;
  border: none;
  background: none;
  font-size: 12px;
  font-weight: 700;
  color: var(--text-tertiary);
  letter-spacing: 0.5px;
  text-align: left;
  cursor: pointer;
}

.ref-cat-header:hover {
  color: var(--text-secondary);
}

.ref-caret {
  display: inline-block;
  font-size: 10px;
  color: var(--text-tertiary);
  transition: transform 0.15s;
}

.ref-caret.open {
  transform: rotate(90deg);
}

/* 二级：组标签——次级小标题，缩进与文档链接竖线对齐，始终展开 */
.ref-group-label {
  margin: 8px 0 2px;
  padding-left: 12px;
  font-size: 12.5px;
  font-weight: 600;
  color: var(--text-secondary);
}

/* 三级：文档链接——与「文档」页 toc-link 同款（左侧竖线，选中金色） */
.ref-doc-link {
  display: block;
  width: 100%;
  padding: 1px 6px 1px 12px;
  border: none;
  border-left: 2px solid var(--border-light);
  background: none;
  font-size: 13.5px;
  color: var(--text-secondary);
  text-align: left;
  cursor: pointer;
  line-height: 1.9;
}

.ref-doc-link.grouped {
  margin-left: 12px;
}

.ref-doc-link:hover {
  color: var(--accent-primary);
}

.ref-doc-link.active {
  color: var(--accent-primary);
  font-weight: 600;
  border-left-color: var(--accent-gold);
}

.ref-content {
  max-width: 780px;
  min-width: 0;
}

.ref-loading,
.ref-error {
  color: var(--text-secondary);
  font-size: 14px;
  padding: 40px 0;
}

.ref-page-toc {
  display: flex;
  flex-wrap: wrap;
  gap: 8px;
  align-items: baseline;
  margin-bottom: 24px;
  padding: 12px 14px;
  background: var(--bg-secondary, rgba(0, 0, 0, 0.03));
  border-radius: 8px;
  font-size: 12px;
}

.ref-page-toc-label {
  color: var(--text-tertiary);
}

.ref-page-toc-item {
  color: var(--text-secondary);
}

.ref-page-toc-item:hover {
  color: var(--accent-primary);
}

@media (max-width: 900px) {
  .ref-nav-toggle {
    display: inline-block;
    margin-top: 12px;
    padding: 6px 14px;
    font-size: 13px;
    border: 1px solid var(--border-light);
    border-radius: 6px;
    background: none;
    color: var(--text-secondary);
    cursor: pointer;
  }

  .ref-layout {
    grid-template-columns: minmax(0, 1fr);
  }

  .ref-sidebar {
    display: none;
    position: static;
    max-height: none;
    margin-bottom: 24px;
    padding: 12px;
    border: 1px solid var(--border-light);
    border-radius: 8px;
  }

  .ref-layout.sidebar-open .ref-sidebar {
    display: block;
  }
}

@media (max-width: 768px) {
  .ref-page {
    padding-top: 28px;
    padding-bottom: 48px;
  }

  .ref-title {
    font-size: 26px;
  }

  .ref-lead {
    font-size: 14px;
  }
}
</style>
