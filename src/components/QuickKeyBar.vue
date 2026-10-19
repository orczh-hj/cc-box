<template>
  <div class="quick-key-bar">
    <button
      v-for="k in keys"
      :key="k.label"
      class="qk-btn"
      :title="k.hint"
      @click="$emit('send', k.bytes)"
    >{{ k.label }}</button>
  </div>
</template>

<script setup lang="ts">
/**
 * 快捷键工具条（手机断点）：软键盘按不出控制键，用按钮直发终端转义序列。
 * Shift+Tab = Claude CLI 工作模式切换，Ctrl+J = 换行不提交。
 */
import { computed } from 'vue'
import { useI18n } from 'vue-i18n'

defineEmits<{ send: [bytes: string] }>()

const { t } = useI18n()

const keys = computed(() => [
  { label: 'Esc', bytes: '\x1b', hint: t('quickKeyEsc') },
  { label: '⇧Tab', bytes: '\x1b[Z', hint: t('quickKeyMode') },
  { label: 'Ctrl+J', bytes: '\x0a', hint: t('quickKeyNewline') },
  { label: '^C', bytes: '\x03', hint: t('quickKeyInterrupt') },
  { label: '↑', bytes: '\x1b[A', hint: '' },
  { label: '↓', bytes: '\x1b[B', hint: '' },
])
</script>

<style scoped>
.quick-key-bar {
  display: flex;
  align-items: center;
  gap: 4px;
  padding: 4px 8px;
  background: var(--bg-secondary);
  border-top: 1px solid var(--border-color);
  flex-shrink: 0;
  /* 可扩展：键多时横向滚动，不压缩按钮 */
  overflow-x: auto;
  overscroll-behavior-x: contain;
  scrollbar-width: none;
}

.quick-key-bar::-webkit-scrollbar {
  display: none;
}

.qk-btn {
  flex: 0 0 auto;
  min-width: 52px;
  padding: 6px 8px;
  border: 1px solid var(--border-color);
  border-radius: var(--radius-md);
  background: var(--bg-primary);
  color: var(--text-secondary);
  font-size: 12px;
  font-family: var(--font-mono, monospace);
  cursor: pointer;
  user-select: none;
  -webkit-tap-highlight-color: transparent;
}

.qk-btn:active {
  background: var(--selected-bg);
  color: var(--accent-gold);
}
</style>
