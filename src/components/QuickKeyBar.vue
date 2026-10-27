<template>
  <!-- mousedown.prevent：点击按键不转移焦点——键盘开着保持开（终端 textarea
       不被 blur），关着不弹（按钮不获焦，click 仍正常触发）。:active 会被
       preventDefault 吞掉，按压高亮由 pointer 事件模拟（pressedLabel） -->
  <div
    class="quick-key-bar"
    @mousedown.prevent
    @pointerdown="onPointerDown"
    @pointerup="pressedLabel = ''"
    @pointercancel="pressedLabel = ''"
    @pointerleave="pressedLabel = ''"
  >
    <button
      v-for="k in keys"
      :key="k.key"
      class="qk-btn"
      :class="{ 'qk-action': !!k.action, 'qk-open': k.action === 'keyboard' && keyboardOpen, pressing: pressedLabel === k.label }"
      :title="k.hint"
      :aria-label="k.label"
      @click="k.action === 'select' ? $emit('select') : k.action === 'keyboard' ? $emit('toggleKeyboard') : $emit('send', k.bytes!)"
    >
      <svg
        v-if="k.icon === 'keyboard'"
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
      <template v-else>{{ k.label }}</template>
    </button>
  </div>
</template>

<script setup lang="ts">
/**
 * 快捷键工具条（手机断点）：软键盘按不出的控制键，用按钮直发终端转义序列。
 * 「键盘」是软键盘唯一显式开关（tap 终端不自动唤出——点击阅读输出时误弹
 * 键盘打扰）；「选择复制」进入终端原生选择模式（拖选 + 复制走 xterm
 * selection）；Enter 提交，Shift+Tab 模式切换，Ctrl+J 换行不提交，
 * Ctrl+U 清行，Home/End 行首/行尾。action 按钮不发字节，经 TerminalView 转发。
 */
import { computed, ref } from 'vue'
import { useI18n } from 'vue-i18n'

const props = defineProps<{ keyboardOpen?: boolean }>()
defineEmits<{ send: [bytes: string]; select: []; toggleKeyboard: [] }>()

const { t } = useI18n()

/** 按压中的按键 label（补偿 :active 被 mousedown.prevent 吞掉的高亮反馈） */
const pressedLabel = ref('')

function onPointerDown(e: PointerEvent) {
  const btn = (e.target as HTMLElement).closest('.qk-btn')
  // 图标按钮无文本：aria-label 与 k.label 同源，统一用它做按压高亮标识
  pressedLabel.value = btn?.getAttribute('aria-label') ?? btn?.textContent ?? ''
}

interface QuickKey {
  key: string
  label: string
  bytes?: string
  hint?: string
  action?: 'select' | 'keyboard'
  /** 图标按钮：渲染 SVG 而非文字（label 仅作 aria-label/按压高亮标识） */
  icon?: 'keyboard'
}

const keys = computed<QuickKey[]>(() => [
  {
    key: 'keyboard',
    label: props.keyboardOpen ? t('quickKeyKeyboardHide') : t('quickKeyKeyboard'),
    action: 'keyboard',
    icon: 'keyboard',
    hint: t('quickKeyKeyboardHint'),
  },
  { key: 'select', label: t('quickKeySelectCopy'), action: 'select' },
  { key: 'esc', label: 'Esc', bytes: '\x1b', hint: t('quickKeyEsc') },
  { key: 'enter', label: 'Enter', bytes: '\r', hint: t('quickKeyEnter') },
  { key: 'stab', label: '⇧Tab', bytes: '\x1b[Z', hint: t('quickKeyMode') },
  { key: 'ctrlj', label: 'Ctrl+J', bytes: '\x0a', hint: t('quickKeyNewline') },
  { key: 'ctrlu', label: 'Ctrl+U', bytes: '\x15', hint: t('quickKeyClearLine') },
  { key: 'up', label: '↑', bytes: '\x1b[A', hint: '' },
  { key: 'down', label: '↓', bytes: '\x1b[B', hint: '' },
  { key: 'left', label: '←', bytes: '\x1b[D', hint: '' },
  { key: 'right', label: '→', bytes: '\x1b[C', hint: '' },
  { key: 'home', label: 'Home', bytes: '\x1b[H', hint: '' },
  { key: 'end', label: 'End', bytes: '\x1b[F', hint: '' },
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

/* 动作按钮（键盘/选择复制）与字节键区分 */
.qk-btn.qk-action {
  color: var(--text-primary);
  border-color: var(--accent-color);
  white-space: nowrap;
}

.qk-btn.qk-action:active {
  color: var(--accent-color);
}

/* 键盘开启态：常亮提示当前软键盘已唤出 */
.qk-btn.qk-open {
  background: var(--selected-bg);
  color: var(--accent-gold);
}
</style>
