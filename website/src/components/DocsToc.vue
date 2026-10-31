<script setup lang="ts">
export interface TocItem {
  id: string
  label: string
}

defineProps<{ groups: Array<{ title: string; items: TocItem[] }> }>()
</script>

<template>
  <nav class="toc" aria-label="目录">
    <div v-for="g in groups" :key="g.title" class="toc-group">
      <p class="toc-group-title">{{ g.title }}</p>
      <a
        v-for="item in g.items"
        :key="item.id"
        :href="`#${item.id}`"
        class="toc-link"
        :data-toc="item.id"
        >{{ item.label }}</a
      >
    </div>
  </nav>
</template>

<style scoped>
.toc {
  position: sticky;
  top: 84px;
  align-self: start;
  max-height: calc(100vh - 104px);
  overflow-y: auto;
  font-size: 13.5px;
  line-height: 2;
  scrollbar-width: thin;
  scrollbar-color: var(--border-light) transparent;
  padding-right: 4px;
}

/* 纤细滚动条：默认近乎隐形，悬停才显色（与参考文档页侧栏一致） */
.toc::-webkit-scrollbar {
  width: 4px;
}

.toc::-webkit-scrollbar-track {
  background: transparent;
}

.toc::-webkit-scrollbar-thumb {
  background: transparent;
  border-radius: 2px;
}

.toc:hover::-webkit-scrollbar-thumb {
  background: var(--border-light);
}

.toc:hover::-webkit-scrollbar-thumb:hover {
  background: var(--text-tertiary);
}

.toc-group {
  margin-bottom: 20px;
}

.toc-group-title {
  font-size: 12px;
  font-weight: 700;
  color: var(--text-tertiary);
  letter-spacing: 0.5px;
  margin-bottom: 4px;
}

.toc-link {
  display: block;
  color: var(--text-secondary);
  padding-left: 12px;
  border-left: 2px solid var(--border-light);
}

.toc-link:hover {
  color: var(--accent-primary);
  text-decoration: none;
}

.toc-link-active {
  color: var(--accent-primary);
  font-weight: 600;
  border-left-color: var(--accent-gold);
}
</style>
