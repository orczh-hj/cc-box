<script setup lang="ts">
import { onBeforeUnmount, onMounted, ref } from 'vue'
import SearchBox from './SearchBox.vue'

withDefaults(defineProps<{ page?: 'home' | 'docs' | 'reference' }>(), { page: 'home' })

const GITHUB_URL = 'https://github.com/orczh-hj/cc-box'
const GITEE_URL = 'https://gitee.com/orczh/cc-box'

// ≤768px 汉堡菜单 / ≤1024px 浮层搜索（点击页面其他处收起）
const menuOpen = ref(false)
const searchOpen = ref(false)

function toggleSearch() {
  searchOpen.value = !searchOpen.value
  menuOpen.value = false
}

function toggleMenu() {
  menuOpen.value = !menuOpen.value
  searchOpen.value = false
}

function closeAll() {
  menuOpen.value = false
  searchOpen.value = false
}

// header 根节点 @click.stop 已挡住内部点击，点击页面其他处即收起浮层/菜单
onMounted(() => document.addEventListener('click', closeAll))
onBeforeUnmount(() => document.removeEventListener('click', closeAll))
</script>

<template>
  <header class="site-header" @click.stop>
    <div class="container header-inner">
      <div class="header-left">
        <a href="/index.html" class="brand">
          <img src="/logo.png" alt="CC-Box" class="brand-logo" />
          <span class="brand-name">CC-Box</span>
        </a>
        <!-- >1024px：header 内常驻搜索框 -->
        <div class="header-search">
          <SearchBox />
        </div>
      </div>
      <nav class="nav">
        <a :href="page === 'home' ? '#features' : 'index.html#features'" class="nav-link">功能</a>
        <a :href="page === 'home' ? '#download' : 'index.html#download'" class="nav-link">下载</a>
        <a href="docs.html" class="nav-link" :class="{ 'nav-link-active': page === 'docs' }"
          >文档</a
        >
        <a
          href="reference.html"
          class="nav-link"
          :class="{ 'nav-link-active': page === 'reference' }"
          >Claude Code 文档</a
        >
        <a :href="GITHUB_URL" target="_blank" rel="noopener" class="nav-link">GitHub</a>
        <a :href="GITEE_URL" target="_blank" rel="noopener" class="nav-link">Gitee</a>
      </nav>
      <!-- ≤1024px：搜索图标（展开浮层）；≤768px：汉堡按钮 -->
      <div class="header-actions">
        <button
          class="icon-btn search-toggle"
          :class="{ open: searchOpen }"
          aria-label="搜索文档"
          @click="toggleSearch"
        >
          <svg
            viewBox="0 0 24 24"
            width="17"
            height="17"
            fill="none"
            stroke="currentColor"
            stroke-width="2"
          >
            <circle cx="11" cy="11" r="7" />
            <line x1="16.5" y1="16.5" x2="21" y2="21" />
          </svg>
        </button>
        <button
          class="icon-btn menu-toggle"
          :class="{ open: menuOpen }"
          aria-label="菜单"
          @click="toggleMenu"
        >
          <svg
            v-if="!menuOpen"
            viewBox="0 0 24 24"
            width="18"
            height="18"
            fill="none"
            stroke="currentColor"
            stroke-width="2"
          >
            <line x1="4" y1="7" x2="20" y2="7" />
            <line x1="4" y1="12" x2="20" y2="12" />
            <line x1="4" y1="17" x2="20" y2="17" />
          </svg>
          <svg
            v-else
            viewBox="0 0 24 24"
            width="18"
            height="18"
            fill="none"
            stroke="currentColor"
            stroke-width="2"
          >
            <line x1="6" y1="6" x2="18" y2="18" />
            <line x1="18" y1="6" x2="6" y2="18" />
          </svg>
        </button>
      </div>
    </div>
    <!-- ≤1024px：header 下方搜索浮层 -->
    <div v-if="searchOpen" class="header-search-panel container">
      <SearchBox />
    </div>
    <!-- ≤768px：垂直菜单面板 -->
    <div v-if="menuOpen" class="header-menu-panel container">
      <nav class="menu-nav">
        <a
          :href="page === 'home' ? '#features' : 'index.html#features'"
          class="menu-link"
          @click="closeAll"
          >功能</a
        >
        <a
          :href="page === 'home' ? '#download' : 'index.html#download'"
          class="menu-link"
          @click="closeAll"
          >下载</a
        >
        <a href="docs.html" class="menu-link" @click="closeAll">文档</a>
        <a href="reference.html" class="menu-link" @click="closeAll">Claude Code 文档</a>
        <a :href="GITHUB_URL" target="_blank" rel="noopener" class="menu-link" @click="closeAll"
          >GitHub</a
        >
        <a :href="GITEE_URL" target="_blank" rel="noopener" class="menu-link" @click="closeAll"
          >Gitee</a
        >
      </nav>
      <div class="menu-search">
        <SearchBox />
      </div>
    </div>
  </header>
</template>

<style scoped>
.site-header {
  position: sticky;
  top: 0;
  z-index: 100;
  background: rgba(250, 249, 246, 0.92);
  backdrop-filter: blur(8px);
  border-bottom: 1px solid var(--border-light);
}

.header-inner {
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: 16px;
  height: 60px;
}

.header-left {
  display: flex;
  align-items: center;
  gap: 20px;
  min-width: 0;
}

.brand {
  display: flex;
  align-items: center;
  gap: 10px;
  flex-shrink: 0;
}

.brand:hover {
  text-decoration: none;
}

.brand-logo {
  width: 30px;
  height: 30px;
}

.brand-name {
  font-size: 18px;
  font-weight: 700;
  color: var(--text-primary);
}

/* >1024px：常驻搜索框 */
.header-search {
  display: none;
  width: 240px;
}

.nav {
  display: flex;
  align-items: center;
  gap: 24px;
}

.nav-link {
  font-size: 14px;
  color: var(--text-secondary);
  white-space: nowrap;
}

.nav-link:hover {
  color: var(--accent-primary);
  text-decoration: none;
}

.nav-link-active {
  color: var(--accent-primary);
  font-weight: 600;
}

/* ≤1024px 才出现的图标按钮 */
.header-actions {
  display: none;
  align-items: center;
  gap: 4px;
}

.icon-btn {
  display: inline-flex;
  align-items: center;
  justify-content: center;
  width: 36px;
  height: 36px;
  border: none;
  border-radius: var(--radius-md);
  background: none;
  color: var(--text-secondary);
  cursor: pointer;
}

.icon-btn:hover {
  background: var(--bg-hover);
  color: var(--accent-primary);
}

.icon-btn.open {
  color: var(--accent-primary);
}

.header-search-panel {
  padding-top: 4px;
  padding-bottom: 10px;
  border-bottom: 1px solid var(--border-light);
  background: rgba(250, 249, 246, 0.98);
}

.header-menu-panel {
  padding-top: 8px;
  padding-bottom: 14px;
  border-bottom: 1px solid var(--border-light);
  background: rgba(250, 249, 246, 0.98);
}

.menu-nav {
  display: flex;
  flex-direction: column;
}

.menu-link {
  padding: 10px 4px;
  font-size: 15px;
  color: var(--text-secondary);
  border-bottom: 1px solid var(--border-light);
}

.menu-link:hover {
  color: var(--accent-primary);
  text-decoration: none;
}

.menu-search {
  margin-top: 12px;
}

@media (min-width: 1025px) {
  .header-search {
    display: block;
  }
}

@media (max-width: 1024px) {
  .header-actions {
    display: flex;
  }

  /* 汉堡按钮只在 ≤768px 出现；769–1024px 只有搜索图标 */
  .menu-toggle {
    display: none;
  }
}

@media (max-width: 768px) {
  .nav {
    display: none;
  }

  .menu-toggle {
    display: inline-flex;
  }
}
</style>
