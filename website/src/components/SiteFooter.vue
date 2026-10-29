<script setup lang="ts">
const GITHUB_URL = 'https://github.com/orczh-hj/cc-box'
const GITEE_URL = 'https://gitee.com/orczh/cc-box'
</script>

<template>
  <footer class="site-footer">
    <div class="container footer-inner">
      <p>© 2026 orczh · MIT License</p>
      <nav class="footer-nav">
        <a :href="GITHUB_URL" target="_blank" rel="noopener">GitHub</a>
        <a :href="GITEE_URL" target="_blank" rel="noopener">Gitee</a>
        <a href="mailto:orczh_hj@163.com">orczh_hj@163.com</a>
      </nav>
    </div>
  </footer>
</template>

<style scoped>
.site-footer {
  border-top: 1px solid var(--border-light);
  padding: 28px 0;
  background: var(--bg-secondary);
}

.footer-inner {
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: 16px;
  flex-wrap: wrap;
  font-size: 13px;
  color: var(--text-tertiary);
}

.footer-nav {
  display: flex;
  gap: 20px;
}

.footer-nav a {
  color: var(--text-secondary);
}
</style>
