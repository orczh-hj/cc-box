<template>
  <div class="cli-output-wrap">
    <!-- 收起态：一行低调条目（末行预览 + 行数），点击展开——不抢界面戏 -->
    <button v-if="collapsed" class="cli-toggle collapsed" @click="collapsed = false">
      <span class="chevron">▸</span>
      <span class="toggle-label">{{ t('marketCliOutput') }}</span>
      <span v-if="lines.length" class="preview">{{ lines[lines.length - 1] }}</span>
    </button>
    <!-- 展开态：头部（收起钮）+ 输出区（自动滚底、可选中复制） -->
    <template v-else>
      <button class="cli-toggle expanded" @click="collapsed = true">
        <span class="chevron">▾</span>
        <span class="toggle-label">{{ t('marketCliOutput') }}</span>
        <span v-if="lines.length" class="line-count">{{ lines.length }}</span>
      </button>
      <div ref="boxEl" class="cli-output" @scroll="onScroll">
        <div v-for="(line, i) in lines" :key="i" class="cli-line">{{ line }}</div>
      </div>
    </template>
  </div>
</template>

<script setup lang="ts">
/**
 * 插件市场 CLI 输出区：展示 plugin-cli-output 事件上抛的实时行。
 * 行合并/替换逻辑在 MarketBrowser 侧完成；本组件负责：
 * - 收起/展开（收起 = 一行低调预览条，点击展开；新动作开始时自动复位展开）
 * - 自动滚底（用户手动上滚查看历史时不打断，滚回底部则跟随）
 * - 文本可选中复制（全局 body user-select:none，这里显式开回）
 */
import { nextTick, ref, watch } from 'vue'
import { useI18n } from 'vue-i18n'

const { t } = useI18n()

const props = defineProps<{
  lines: string[]
}>()

const collapsed = ref(false)
const boxEl = ref<HTMLElement | null>(null)
/** 用户是否上滚离开底部（true = 停止自动跟随） */
let scrolledUp = false

// 新动作开始（外部清空 lines）→ 复位为展开，让用户直接看到新一轮输出
watch(
  () => props.lines.length,
  (n, old) => {
    if (n > 0 && (old ?? 0) === 0) {
      collapsed.value = false
    }
    if (collapsed.value || scrolledUp) return
    void nextTick(() => {
      const el = boxEl.value
      if (el) el.scrollTop = el.scrollHeight
    })
  }
)

function onScroll() {
  const el = boxEl.value
  if (!el) return
  scrolledUp = el.scrollTop + el.clientHeight < el.scrollHeight - 16
}
</script>

<style scoped>
/* 收起/展开头部：无边框背景的细条，tertiary 弱化色，不与 busy-bar 抢视觉 */
.cli-toggle {
  display: flex;
  align-items: center;
  gap: 6px;
  width: calc(100% - 32px);
  margin: 6px 16px 0;
  padding: 3px 4px;
  border: none;
  background: transparent;
  color: var(--text-tertiary);
  font-size: 11px;
  font-family: ui-monospace, 'Cascadia Mono', Consolas, monospace;
  cursor: pointer;
  text-align: left;
}

.cli-toggle:hover {
  color: var(--text-secondary);
}

.chevron {
  flex-shrink: 0;
  font-size: 9px;
}

.toggle-label {
  flex-shrink: 0;
}

/* 收起态末行预览：单行截断，弱色但仍是真实输出（不可选——整条是按钮） */
.preview {
  flex: 1;
  min-width: 0;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
  color: var(--text-tertiary);
}

/* 展开态行数徽标 */
.line-count {
  flex-shrink: 0;
  padding: 0 5px;
  border-radius: 7px;
  background: var(--bg-tertiary, var(--hover-bg));
  font-size: 10px;
}

/* 输出区：可选中复制（全局 body user-select:none，显式开回） */
.cli-output {
  margin: 4px 16px 8px;
  padding: 10px 12px;
  max-height: 180px;
  overflow-y: auto;
  background: var(--bg-primary);
  border: 1px solid var(--border-color);
  border-radius: 8px;
  font-family: ui-monospace, 'Cascadia Mono', Consolas, monospace;
  font-size: 12px;
  line-height: 1.55;
  color: var(--text-primary);
  white-space: pre-wrap;
  word-break: break-all;
  user-select: text;
  cursor: text;
}

.cli-line {
  min-height: 1em;
}

.cli-output::-webkit-scrollbar {
  width: 6px;
}

.cli-output::-webkit-scrollbar-thumb {
  background: var(--border-color);
  border-radius: 3px;
}
</style>
