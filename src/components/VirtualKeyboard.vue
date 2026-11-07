<template>
  <!-- 完整虚拟键盘（手机端，文档流内底部面板）：软键盘按不出的修饰键/功能键/F 键兜底。
       mousedown.prevent 同 QuickKeyBar：点击按键不偷终端 textarea 焦点。
       粘滞修饰键（RustDesk/向日葵同款）：点 Ctrl/Alt/Shift 高亮，下一键
       组合后自动释放，可叠加、再点取消。按键经 vkeyToBytes 转 ANSI 序列
       emit send，由 TerminalView 走 sendText 进 PTY。 -->
  <div
    class="virtual-keyboard"
    @mousedown.prevent
    @pointerdown="onPointerDown"
    @pointerup="pressedKey = ''"
    @pointercancel="pressedKey = ''"
    @pointerleave="pressedKey = ''"
  >
    <div class="vk-head">
      <span class="vk-title">{{ t('vkbTitle') }}</span>
      <button class="vk-close" aria-label="close" :title="t('vkbCloseHint')" @click="$emit('close')">
        <svg viewBox="0 0 24 24" width="14" height="14" fill="none" stroke="currentColor"
          stroke-width="2" stroke-linecap="round" aria-hidden="true">
          <path d="M6 6l12 12M18 6L6 18" />
        </svg>
      </button>
    </div>

    <!-- 功能排：Esc/Tab/F1-F12 横滚 -->
    <div class="vk-row vk-fn-row">
      <button
        v-for="k in fnRow"
        :key="k.key"
        class="vk-btn"
        :class="{ pressing: pressedKey === k.aria }"
        :aria-label="k.aria"
        @click="press(k)"
      >{{ k.label }}</button>
    </div>

    <div class="vk-main">
      <!-- 左侧功能列：方向键 2×2 + 导航/删除 -->
      <div class="vk-side">
        <div class="vk-arrows">
          <button class="vk-btn" aria-label="up" @click="press({ key: 'up' })">↑</button>
          <button class="vk-btn" aria-label="left" @click="press({ key: 'left' })">←</button>
          <button class="vk-btn" aria-label="down" @click="press({ key: 'down' })">↓</button>
          <button class="vk-btn" aria-label="right" @click="press({ key: 'right' })">→</button>
        </div>
        <div class="vk-nav">
          <button class="vk-btn" aria-label="home" @click="press({ key: 'home' })">Home</button>
          <button class="vk-btn" aria-label="end" @click="press({ key: 'end' })">End</button>
          <button class="vk-btn" aria-label="pageup" @click="press({ key: 'pageup' })">PgUp</button>
          <button class="vk-btn" aria-label="pagedown" @click="press({ key: 'pagedown' })">PgDn</button>
          <button class="vk-btn" aria-label="delete" @click="press({ key: 'delete' })">Del</button>
          <button class="vk-btn" aria-label="backspace" @click="press({ key: 'backspace' })">⌫</button>
        </div>
      </div>

      <!-- QWERTY 主区 -->
      <div class="vk-qwerty">
        <div class="vk-row">
          <button
            v-for="k in row0"
            :key="k.key"
            class="vk-btn"
            :class="{ pressing: pressedKey === k.aria }"
            :aria-label="k.aria"
            @click="press(k)"
          >{{ k.label }}</button>
        </div>
        <div v-for="(row, i) in letterRows" :key="i" class="vk-row">
          <button
            v-for="k in row"
            :key="k.key"
            class="vk-btn"
            :class="{ pressing: pressedKey === k.aria }"
            :aria-label="k.aria"
            @click="press(k)"
          >{{ k.label }}</button>
        </div>
        <div class="vk-row">
          <button
            class="vk-btn vk-mod"
            :class="{ 'vk-sticky': sticky.ctrl }"
            aria-label="ctrl"
            @click="toggleSticky('ctrl')"
          >Ctrl</button>
          <button
            class="vk-btn vk-mod"
            :class="{ 'vk-sticky': sticky.alt }"
            aria-label="alt"
            @click="toggleSticky('alt')"
          >Alt</button>
          <button
            class="vk-btn vk-mod"
            :class="{ 'vk-sticky': sticky.shift }"
            aria-label="shift"
            @click="toggleSticky('shift')"
          >⇧</button>
          <button class="vk-btn vk-space" aria-label="space" @click="press({ key: 'space' })">&nbsp;</button>
          <button class="vk-btn" aria-label="enter" @click="press({ key: 'enter' })">↵</button>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
/**
 * 完整虚拟键盘弹层（仅手机渲染，由 TerminalView 控制）：粘滞修饰键状态机 +
 * vkeyToBytes 映射。修饰键点击切粘滞（可叠加、再点同键取消）；普通键点击组装
 * VirtualKeyEvent → vkeyToBytes，非 null emit send，无论映射成败粘滞全部清空。
 */
import { reactive, ref } from 'vue'
import { useI18n } from 'vue-i18n'
import { vkeyToBytes, type VirtualKeyEvent } from '@/utils/virtual-keys'

const { t } = useI18n()

const emit = defineEmits<{ send: [bytes: string]; close: [] }>()

/** 按压中的按键 aria（补偿 :active 被 mousedown.prevent 吞掉的高亮反馈） */
const pressedKey = ref('')

/** 粘滞修饰键状态（下一键组合后自动清空） */
const sticky = reactive({ ctrl: false, alt: false, shift: false })

function onPointerDown(e: PointerEvent) {
  const btn = (e.target as HTMLElement).closest('.vk-btn')
  pressedKey.value = btn?.getAttribute('aria-label') ?? ''
}

function toggleSticky(mod: 'ctrl' | 'alt' | 'shift') {
  sticky[mod] = !sticky[mod]
}

interface VKey extends VirtualKeyEvent {
  /** 按钮显示文本 */
  label: string
  /** aria-label（规范键名） */
  aria: string
}

/** 数字排 */
const row0: VKey[] = '1234567890'.split('').map(c => ({ key: c, label: c, aria: c }))

/** 三排字母（QWERTY） */
const letterRows: VKey[][] = [
  'qwertyuiop'.split(''),
  'asdfghjkl'.split(''),
  'zxcvbnm'.split(''),
].map(row => row.map(c => ({ key: c, label: c, aria: c })))

/** 功能排：Esc/Tab/F1-F12 */
const fnRow: VKey[] = [
  { key: 'esc', label: 'Esc', aria: 'esc' },
  { key: 'tab', label: 'Tab', aria: 'tab' },
  ...Array.from({ length: 12 }, (_, i) => ({
    key: `f${i + 1}`, label: `F${i + 1}`, aria: `f${i + 1}`,
  })),
]

/** 普通键：组装修饰 → 映射 → 发射；无论成败粘滞清空（null 静默忽略） */
function press(k: VirtualKeyEvent) {
  const bytes = vkeyToBytes({
    key: k.key,
    ctrl: sticky.ctrl || undefined,
    alt: sticky.alt || undefined,
    shift: sticky.shift || undefined,
  })
  sticky.ctrl = false
  sticky.alt = false
  sticky.shift = false
  if (bytes !== null) emit('send', bytes)
}
</script>

<style scoped>
.virtual-keyboard {
  /* 文档流内（terminal-container flex 列、排在 QuickKeyBar 后）：终端区随弹层
     出现自动收缩 refit（ResizeObserver），输入行始终可见；不 fixed 遮挡 */
  flex-shrink: 0;
  display: flex;
  flex-direction: column;
  gap: 4px;
  padding: 6px 6px calc(var(--safe-area-bottom, 0px) + 6px);
  background: var(--bg-secondary);
  border-top: 1px solid var(--border-color);
  user-select: none;
  -webkit-tap-highlight-color: transparent;
}

.vk-head {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 0 6px;
}

.vk-title {
  font-size: 12px;
  color: var(--text-secondary);
}

.vk-close {
  display: inline-flex;
  align-items: center;
  justify-content: center;
  width: 26px;
  height: 26px;
  border: 1px solid var(--border-color);
  border-radius: var(--radius-md);
  background: var(--bg-primary);
  color: var(--text-secondary);
  cursor: pointer;
}

.vk-row {
  display: flex;
  gap: 4px;
}

.vk-fn-row {
  overflow-x: auto;
  overscroll-behavior-x: contain;
  scrollbar-width: none;
}

.vk-fn-row::-webkit-scrollbar {
  display: none;
}

.vk-fn-row .vk-btn {
  min-width: 40px;
  flex: 0 0 auto;
  font-size: 11px;
}

.vk-main {
  display: flex;
  gap: 6px;
  align-items: stretch;
}

.vk-side {
  display: flex;
  flex-direction: column;
  gap: 4px;
  flex: 0 0 auto;
}

.vk-arrows {
  display: grid;
  grid-template-columns: repeat(2, 1fr);
  gap: 4px;
}

.vk-arrows .vk-btn {
  min-width: 38px;
  padding: 8px 4px;
}

/* 方向键视觉顺序：↑ 在上排左侧空位后（↑ 独占一行更贴近实体键盘） */
.vk-arrows .vk-btn[aria-label='up'] {
  grid-column: 1 / span 2;
}

.vk-nav {
  display: grid;
  grid-template-columns: repeat(2, 1fr);
  gap: 4px;
}

.vk-nav .vk-btn {
  min-width: 38px;
  padding: 6px 4px;
  font-size: 11px;
}

.vk-qwerty {
  flex: 1;
  display: flex;
  flex-direction: column;
  gap: 4px;
  min-width: 0;
}

.vk-qwerty .vk-row {
  display: grid;
  grid-template-columns: repeat(10, 1fr);
  gap: 4px;
}

.vk-btn {
  padding: 9px 2px;
  display: inline-flex;
  align-items: center;
  justify-content: center;
  border: 1px solid var(--border-color);
  border-radius: var(--radius-md);
  background: var(--bg-primary);
  color: var(--text-primary);
  font-size: 13px;
  font-family: var(--font-mono, monospace);
  cursor: pointer;
}

.vk-btn:active,
.vk-btn.pressing {
  background: var(--selected-bg);
  color: var(--accent-gold);
}

.vk-btn.vk-mod,
.vk-btn.vk-space {
  font-weight: 600;
}

.vk-btn.vk-sticky {
  background: var(--selected-bg);
  color: var(--accent-gold);
  border-color: var(--accent-color);
}

.vk-btn.vk-space {
  grid-column: span 4;
}
</style>
