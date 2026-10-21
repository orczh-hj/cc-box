<template>
  <div v-if="visible" class="copy-reader" @contextmenu.stop>
    <!-- 顶栏：模式切换 + 复制操作（user-select:none 保证选区只在正文） -->
    <div class="cr-topbar">
      <div class="cr-modes">
        <button
          class="cr-mode-btn"
          :class="{ active: mode === 'smart' }"
          @click="mode = 'smart'"
        >{{ t('copyReaderSmart') }}</button>
        <button
          class="cr-mode-btn"
          :class="{ active: mode === 'raw' }"
          @click="mode = 'raw'"
        >{{ t('copyReaderRaw') }}</button>
      </div>
      <div class="cr-actions">
        <button
          class="cr-btn primary"
          :disabled="!selectionText"
          @click="copySelected"
        >{{ copiedKind === 'selected' ? t('selectModeCopied') : t('selectModeCopy') }}</button>
        <button class="cr-btn" @click="copyAll">
          {{ copiedKind === 'all' ? t('selectModeCopied') : t('copyReaderCopyAll') }}
        </button>
        <button class="cr-btn exit" @click="emit('close')">{{ t('selectModeExit') }}</button>
      </div>
    </div>
    <div v-if="truncated" class="cr-truncated">{{ t('copyReaderTruncated') }}</div>
    <div class="cr-hint">{{ t('selectModeHint') }}</div>

    <!-- 正文：单文本节点 pre，系统原生长按选择手柄接管 -->
    <div ref="scrollRef" class="cr-content">
      <pre>{{ currentText }}</pre>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, watch, nextTick, onBeforeUnmount } from 'vue'
import { useI18n } from 'vue-i18n'
import { copyText } from '@/api/platform'

const props = defineProps<{
  visible: boolean
  smart: string
  raw: string
  truncated: boolean
}>()

const emit = defineEmits<{ close: [] }>()

const { t } = useI18n()

const mode = ref<'smart' | 'raw'>('smart')
const selectionText = ref('')
const copiedKind = ref<'' | 'selected' | 'all'>('')
const scrollRef = ref<HTMLElement | null>(null)
let copiedTimer: number | null = null

const currentText = computed(() => (mode.value === 'smart' ? props.smart : props.raw))

/** 选区实时跟踪（顶栏禁用态） */
function refreshSelection() {
  const sel = window.getSelection()
  selectionText.value = sel && sel.rangeCount > 0 ? sel.toString() : ''
}

function flashCopied(kind: 'selected' | 'all') {
  copiedKind.value = kind
  if (copiedTimer) window.clearTimeout(copiedTimer)
  copiedTimer = window.setTimeout(() => (copiedKind.value = ''), 2000)
}

async function copySelected() {
  if (!selectionText.value) return
  await copyText(selectionText.value)
  flashCopied('selected')
}

async function copyAll() {
  await copyText(currentText.value)
  flashCopied('all')
}

// 打开：收软键盘 + 滚到底（最新输出在底部）
watch(
  () => props.visible,
  async (v) => {
    if (!v) return
    mode.value = 'smart'
    copiedKind.value = ''
    selectionText.value = ''
    ;(document.activeElement as HTMLElement | null)?.blur()
    await nextTick()
    if (scrollRef.value) scrollRef.value.scrollTop = scrollRef.value.scrollHeight
  },
  { immediate: true }
)

document.addEventListener('selectionchange', refreshSelection)
onBeforeUnmount(() => {
  document.removeEventListener('selectionchange', refreshSelection)
  if (copiedTimer) window.clearTimeout(copiedTimer)
})
</script>

<style scoped>
.copy-reader {
  position: fixed;
  inset: 0;
  z-index: 1000;
  display: flex;
  flex-direction: column;
  background: var(--bg-primary);
  /* Android 长按选择菜单依赖 contextmenu（模板 @contextmenu.stop 已阻隔全局拦截） */
  -webkit-user-select: none;
  user-select: none;
}

.cr-topbar {
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: 8px;
  flex-wrap: wrap;
  padding: 8px 12px;
  padding-top: calc(8px + env(safe-area-inset-top));
  background: var(--bg-secondary);
  border-bottom: 1px solid var(--border-color);
  -webkit-user-select: none;
  user-select: none;
}

.cr-modes {
  display: flex;
  border: 1px solid var(--border-color);
  border-radius: 6px;
  overflow: hidden;
}

.cr-mode-btn {
  padding: 6px 12px;
  border: none;
  background: transparent;
  color: var(--text-secondary);
  font-size: 12px;
  cursor: pointer;
}

.cr-mode-btn.active {
  background: var(--accent-color);
  color: var(--bg-primary);
}

.cr-actions {
  display: flex;
  gap: 6px;
  flex-wrap: wrap;
}

.cr-btn {
  padding: 6px 10px;
  border: 1px solid var(--border-color);
  border-radius: 6px;
  background: var(--bg-primary);
  color: var(--text-primary);
  font-size: 12px;
  cursor: pointer;
}

.cr-btn.primary {
  border-color: var(--accent-color);
  color: var(--accent-color);
}

.cr-btn:disabled {
  opacity: 0.45;
  cursor: default;
}

.cr-btn.exit {
  color: var(--text-secondary);
}

.cr-truncated {
  padding: 6px 12px;
  background: var(--warning-bg, rgba(204, 153, 0, 0.12));
  color: var(--warning, #b8860b);
  font-size: 12px;
}

.cr-hint {
  padding: 4px 12px;
  color: var(--text-secondary);
  font-size: 11px;
  -webkit-user-select: none;
  user-select: none;
}

.cr-content {
  flex: 1;
  overflow: auto;
  overscroll-behavior: contain;
  -webkit-overflow-scrolling: touch;
  padding: 8px 12px;
  /* 不设 touch-action：让浏览器原生滚动 + 长按选择生效 */
}

.cr-content pre {
  margin: 0;
  font-family: 'Cascadia Code', 'Cascadia Mono', Consolas, 'Courier New', monospace;
  font-size: 13px;
  line-height: 1.45;
  white-space: pre; /* 保横向滚动，勿 word-wrap */
  /* 系统原生选择手柄（iOS 放大镜 / Android 双锚点） */
  -webkit-user-select: text;
  user-select: text;
  -webkit-touch-callout: default;
  color: var(--text-primary);
}
</style>
