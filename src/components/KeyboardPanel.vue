<template>
  <!-- 手机键盘面板（工具条唯一入口）：三 tab 切换键盘类型。
       原生键盘 = 系统软键盘（选择即唤出并收起面板）；快捷键 = Claude Code
       主会话快捷键列表（点按直发）；电脑键盘 = 与真实 PC 键盘同布局。
       容器 mousedown.prevent：点击面板不偷终端 textarea 焦点。 -->
  <div class="keyboard-panel" @mousedown.prevent>
    <div class="kp-tabs">
      <button
        v-for="tab in tabs"
        :key="tab.id"
        class="kp-tab"
        :class="{ active: activeTab === tab.id }"
        :aria-label="tab.id"
        @click="switchTab(tab.id)"
      >{{ t(tab.labelKey) }}</button>
      <button class="kp-close" :aria-label="'close'" :title="t('vkbCloseHint')" @click="$emit('close')">
        <svg viewBox="0 0 24 24" width="14" height="14" fill="none" stroke="currentColor"
          stroke-width="2" stroke-linecap="round" aria-hidden="true">
          <path d="M6 6l12 12M18 6L6 18" />
        </svg>
      </button>
    </div>

    <!-- 快捷键：分组列表（功能名 + 键位），点按直发 -->
    <div v-if="activeTab === 'shortcuts'" class="kp-shortcuts">
      <section v-for="group in CLAUDE_SHORTCUTS" :key="group.titleKey" class="kp-group">
        <h4 class="kp-group-title">{{ t(group.titleKey) }}</h4>
        <div class="kp-grid">
          <button
            v-for="item in group.items"
            :key="item.labelKey"
            class="kp-item"
            :aria-label="item.action ?? item.bytes"
            @click="item.action === 'paste' ? $emit('paste') : $emit('send', item.bytes!)"
          >
            <span v-if="item.keys || item.keysMac" class="kp-item-keys">{{ item.keysMac && isMacTarget ? item.keysMac : item.keys }}</span>
            <span class="kp-item-label">{{ t(item.labelKey) }}</span>
          </button>
        </div>
      </section>
    </div>

    <!-- 电脑键盘：真实 PC 键盘布局 -->
    <VirtualKeyboard
      v-else-if="activeTab === 'pc'"
      :platform="platform"
      @send="$emit('send', $event)"
    />
    <!-- 原生键盘 tab 点击即收起面板并唤出系统软键盘（switchTab 直接 emit native），
         不会停在此分支；保留空占位仅为模板分支完整性 -->
  </div>
</template>

<script setup lang="ts">
/**
 * 键盘类型面板：三种模式（原生键盘 / 快捷键 / 电脑键盘）。
 * 快捷键数据源 src/config/claude-shortcuts.ts（Claude Code 主会话完整清单）。
 * 「原生键盘」tab 不放软键盘本身——软键盘与面板同时显示会挤压视口，
 * 点击即收起面板并唤出系统软键盘（由 TerminalView 执行）。
 */
import { computed } from 'vue'
import { useI18n } from 'vue-i18n'
import { CLAUDE_SHORTCUTS } from '@/config/claude-shortcuts'
import VirtualKeyboard from './VirtualKeyboard.vue'

export type KeyboardTab = 'native' | 'shortcuts' | 'pc'

const props = defineProps<{
  activeTab: KeyboardTab
  /** 目标电脑 OS（决定 Alt/Option、Win/Cmd 显示） */
  platform?: string | null
}>()

const emit = defineEmits<{
  'update:activeTab': [tab: KeyboardTab]
  send: [bytes: string]
  /** 粘贴：读系统剪贴板 + 括号粘贴（非字节直发） */
  paste: []
  native: []
  close: []
}>()

const { t } = useI18n()

const isMacTarget = computed(() => (props.platform ?? '').startsWith('mac'))

const tabs: Array<{ id: KeyboardTab; labelKey: string }> = [
  { id: 'native', labelKey: 'kbTabNative' },
  { id: 'shortcuts', labelKey: 'kbTabShortcuts' },
  { id: 'pc', labelKey: 'kbTabPc' },
]

function switchTab(tab: KeyboardTab) {
  // 原生键盘：选中即执行——收起面板 + 唤出系统软键盘（由 TerminalView 处理），
  // 不再多一步「打开系统键盘」按钮
  if (tab === 'native') {
    emit('native')
    return
  }
  emit('update:activeTab', tab)
}
</script>

<style scoped>
.keyboard-panel {
  /* 文档流内（TerminalView 的 terminal-container flex 列）：面板高度占据下方，
     终端区自动收缩 refit（ResizeObserver），输入行始终可见 */
  flex-shrink: 0;
  display: flex;
  flex-direction: column;
  background: var(--bg-secondary);
  border-top: 1px solid var(--border-color);
  padding-bottom: var(--safe-area-bottom, 0px);
  user-select: none;
  -webkit-tap-highlight-color: transparent;
}

.kp-tabs {
  display: flex;
  align-items: center;
  gap: 4px;
  padding: 6px 8px;
  border-bottom: 1px solid var(--border-color);
}

.kp-tab {
  flex: 1;
  padding: 6px 4px;
  border: 1px solid var(--border-color);
  border-radius: var(--radius-md);
  background: var(--bg-primary);
  color: var(--text-secondary);
  font-size: 12px;
  cursor: pointer;
}

.kp-tab.active {
  background: var(--selected-bg);
  color: var(--accent-gold);
  border-color: var(--accent-color);
}

.kp-close {
  flex: 0 0 auto;
  display: inline-flex;
  align-items: center;
  justify-content: center;
  width: 28px;
  height: 28px;
  border: 1px solid var(--border-color);
  border-radius: var(--radius-md);
  background: var(--bg-primary);
  color: var(--text-secondary);
  cursor: pointer;
}

/* 快捷键列表：限高滚动，避免占满屏幕 */
.kp-shortcuts {
  max-height: 42vh;
  overflow-y: auto;
  overscroll-behavior-y: contain;
  padding: 4px 8px 8px;
}

.kp-group + .kp-group {
  margin-top: 8px;
}

.kp-group-title {
  margin: 6px 0 4px;
  font-size: 11px;
  font-weight: 600;
  color: var(--text-secondary);
}

.kp-grid {
  display: grid;
  grid-template-columns: repeat(2, minmax(0, 1fr));
  gap: 4px;
}

.kp-item {
  display: flex;
  flex-direction: column;
  align-items: flex-start;
  gap: 2px;
  min-width: 0;
  padding: 7px 8px;
  border: 1px solid var(--border-color);
  border-radius: var(--radius-md);
  background: var(--bg-primary);
  color: var(--text-primary);
  cursor: pointer;
  text-align: left;
}

.kp-item:active {
  background: var(--selected-bg);
  color: var(--accent-gold);
}

/* 键位（按什么键）：等宽 + 强调色，一眼可辨 */
.kp-item-keys {
  font-family: var(--font-mono, monospace);
  font-size: 11px;
  font-weight: 600;
  color: var(--accent-gold);
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
  max-width: 100%;
}

/* 功能名（做什么）：次要色说明 */
.kp-item-label {
  font-size: 11px;
  color: var(--text-secondary);
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
  max-width: 100%;
}



</style>
