<template>
  <div class="quick-key-bar">
    <button
      v-for="k in keys"
      :key="k.label"
      class="qk-btn"
      :class="{ 'qk-action': !!k.action }"
      :title="k.hint"
      @click="k.action === 'select' ? $emit('select') : $emit('send', k.bytes!)"
    >{{ k.label }}</button>
  </div>
</template>

<script setup lang="ts">
/**
 * 快捷键工具条（手机断点）：软键盘按不出的控制键，用按钮直发终端转义序列。
 * 软键盘由点击终端区域唤出；「选择复制」进入终端原生选择模式（拖选 + 复制走
 * xterm selection）；Shift+Tab 模式切换，Ctrl+J 换行，Ctrl+U 清行，
 * Home/End 行首/行尾。action 按钮不发字节，经 TerminalView 转发。
 */
import { computed } from 'vue'
import { useI18n } from 'vue-i18n'

defineEmits<{ send: [bytes: string]; select: [] }>()

const { t } = useI18n()

interface QuickKey {
  label: string
  bytes?: string
  hint?: string
  action?: 'select'
}

const keys = computed<QuickKey[]>(() => [
  { label: t('quickKeySelectCopy'), action: 'select' },
  { label: 'Esc', bytes: '\x1b', hint: t('quickKeyEsc') },
  { label: '⇧Tab', bytes: '\x1b[Z', hint: t('quickKeyMode') },
  { label: 'Ctrl+J', bytes: '\x0a', hint: t('quickKeyNewline') },
  { label: 'Ctrl+U', bytes: '\x15', hint: t('quickKeyClearLine') },
  { label: '↑', bytes: '\x1b[A', hint: '' },
  { label: '↓', bytes: '\x1b[B', hint: '' },
  { label: '←', bytes: '\x1b[D', hint: '' },
  { label: '→', bytes: '\x1b[C', hint: '' },
  { label: 'Home', bytes: '\x1b[H', hint: '' },
  { label: 'End', bytes: '\x1b[F', hint: '' },
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

/* 动作按钮（键盘/选择复制）与字节键区分 */
.qk-btn.qk-action {
  color: var(--text-primary);
  border-color: var(--accent-color);
  white-space: nowrap;
}

.qk-btn.qk-action:active {
  color: var(--accent-color);
}
</style>
