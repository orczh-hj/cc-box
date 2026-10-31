<script setup lang="ts">
withDefaults(defineProps<{ page?: 'home' | 'docs' | 'reference' }>(), { page: 'home' })

const GITHUB_URL = 'https://github.com/orczh-hj/cc-box'
const GITEE_URL = 'https://gitee.com/orczh/cc-box'
</script>

<template>
  <header class="site-header">
    <div class="container header-inner">
      <a href="/index.html" class="brand">
        <img src="/logo.png" alt="CC-Box" class="brand-logo" />
        <span class="brand-name">CC-Box</span>
      </a>
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
  height: 60px;
}

.brand {
  display: flex;
  align-items: center;
  gap: 10px;
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

.nav {
  display: flex;
  align-items: center;
  gap: 24px;
}

.nav-link {
  font-size: 14px;
  color: var(--text-secondary);
}

.nav-link:hover {
  color: var(--accent-primary);
  text-decoration: none;
}

.nav-link-active {
  color: var(--accent-primary);
  font-weight: 600;
}

@media (max-width: 768px) {
  .nav {
    gap: 14px;
  }

  .nav-link {
    font-size: 13px;
  }
}
</style>
