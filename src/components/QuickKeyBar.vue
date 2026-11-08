<template>
  <!-- mousedown.prevent：点击按键不转移焦点——键盘面板状态不受影响，终端
       textarea 不 blur。:active 会被 preventDefault 吞掉，按压高亮由 pointer
       事件模拟（pressedKey） -->
  <div
    class="quick-key-bar"
    @mousedown.prevent
    @pointerdown="onPointerDown"
    @pointerup="pressedKey = ''"
    @pointercancel="pressedKey = ''"
    @pointerleave="pressedKey = ''"
  >
    <button
      class="qk-btn qk-action"
      :class="{ 'qk-open': keyboardOpen }"
      :title="t('quickKeyKeyboardHint')"
      :aria-label="'keyboard'"
      @click="$emit('toggleKeyboard')"
    >
      <svg
        class="qk-icon"
        viewBox="0 0 24 24"
        width="16"
        height="16"
        fill="none"
        stroke="currentColor"
        stroke-width="1.8"
        stroke-linecap="round"
        stroke-linejoin="round"
        aria-hidden="true"
      >
        <rect x="2.5" y="6" width="19" height="12" rx="2" />
        <path d="M6 10h.5M9.5 10h.5M13 10h.5M16.5 10h.5M6 13h.5M9.5 13h.5M13 13h.5M16.5 13h.5M8 16h8" />
      </svg>
    </button>
    <button
      class="qk-btn qk-action"
      :aria-label="'select'"
      @click="$emit('select')"
    >{{ t('quickKeySelectCopy') }}</button>
    <button
      class="qk-btn qk-action"
      :aria-label="'paste'"
      @click="$emit('paste')"
    >{{ t('quickKeyPaste') }}</button>
    <!-- 高频键常驻工具条（功能名/符号为主，不写组合键） -->
    <button
      v-for="k in hotKeys"
      :key="k.key"
      class="qk-btn"
      :class="{ pressing: pressedKey === k.key }"
      :aria-label="k.key"
      @click="$emit('send', k.bytes)"
    >{{ k.label }}</button>
  </div>
</template>

<script setup lang="ts">
/**
 * 快捷键工具条（手机断点）：键盘面板唯一入口（图标）+ 选择复制 + 粘贴
 * + 高频键（Esc / Enter / 方向键）。次级与危险键（Ctrl+C 等）只在键盘面板
 * 内，工具条不放误触即打断/退出 CLI 的直发键。
 */
import { computed, ref } from 'vue'
import { useI18n } from 'vue-i18n'

defineProps<{ keyboardOpen?: boolean }>()
defineEmits<{ select: []; toggleKeyboard: []; paste: []; send: [bytes: string] }>()

const { t } = useI18n()

/** 工具条常驻高频键（标签用功能名/符号，不写组合键）。
 *  换行 = Ctrl+J（\x0a）：插入换行不提交，与 Enter 提交互补。
 *  computed 保持 i18n 文案响应式（运行中切换语言即时更新） */
const hotKeys = computed(() => [
  { key: 'esc', label: 'Esc', bytes: '\x1b' },
  { key: 'enter', label: 'Enter', bytes: '\r' },
  { key: 'newline', label: t('quickKeyNewlineShort'), bytes: '\x0a' },
  { key: 'up', label: '↑', bytes: '\x1b[A' },
  { key: 'down', label: '↓', bytes: '\x1b[B' },
  { key: 'left', label: '←', bytes: '\x1b[D' },
  { key: 'right', label: '→', bytes: '\x1b[C' },
])

/** 按压中的按键 aria（补偿 :active 被 mousedown.prevent 吞掉的高亮反馈） */
const pressedKey = ref('')

function onPointerDown(e: PointerEvent) {
  const btn = (e.target as HTMLElement).closest('.qk-btn')
  pressedKey.value = btn?.getAttribute('aria-label') ?? ''
}
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
  display: inline-flex;
  align-items: center;
  justify-content: center;
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

.qk-btn:active,
.qk-btn.pressing {
  background: var(--selected-bg);
  color: var(--accent-gold);
}

.qk-btn.qk-action {
  color: var(--text-primary);
  border-color: var(--accent-color);
  white-space: nowrap;
}

.qk-btn.qk-action:active {
  color: var(--accent-color);
}

/* 键盘面板开启态：常亮提示面板已展开 */
.qk-btn.qk-open {
  background: var(--selected-bg);
  color: var(--accent-gold);
}
</style>
