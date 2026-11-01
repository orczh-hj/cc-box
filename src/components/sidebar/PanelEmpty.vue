<!--
  侧边栏面板空状态：文案 + 说明 + 可选「新建」按钮。
  四个面板（Skills / Agents / MCP / Hooks）共用，避免样式与结构各抄一遍。
-->
<template>
  <div class="empty-state">
    <span class="empty-text">{{ text }}</span>
    <span v-if="hint" class="empty-hint">{{ hint }}</span>
    <button v-if="cta" class="empty-cta" type="button" @click="$emit('action')">
      {{ cta }}
    </button>
  </div>
</template>

<script setup lang="ts">
defineProps<{
  text: string
  hint?: string
  /** 有值时渲染新建按钮，点击触发 action */
  cta?: string
}>()

defineEmits<{ action: [] }>()
</script>

<style scoped>
.empty-state {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  padding: 24px;
  gap: 8px;
}

.empty-text {
  font-size: 13px;
  color: var(--text-secondary);
}

.empty-hint {
  font-size: 12px;
  color: var(--text-tertiary);
}

.empty-cta {
  margin-top: 6px;
  padding: 6px 16px;
  border: 1px solid var(--border-color);
  border-radius: 8px;
  background: var(--bg-primary);
  color: var(--text-primary);
  font-size: 12px;
  cursor: pointer;
}

.empty-cta:hover {
  background: var(--hover-bg);
}
</style>
