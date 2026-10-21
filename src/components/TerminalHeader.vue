<template>
  <header class="terminal-header">
    <div class="title-area">
      <!-- 统一只显示会话名：项目名由 document.title 承载（浏览器端标题跟随
           项目，桌面端 Tauri 窗口标题自管），header 不再重复拼接 -->
      <span v-if="sessionName" class="header-title">{{ sessionName }}</span>
    </div>
    <div class="header-right">
      <!-- 窗口管理按钮（置顶/半屏停靠）：仅桌面 Tauri，浏览器模式（任意宽度）无窗口操作 -->
      <template v-if="isTauriEnv">
        <button class="header-btn window-btn" :class="{ active: appStore.alwaysOnTop }" @click="appStore.toggleAlwaysOnTop()" :title="t('toggleAlwaysOnTop', { key: cmd + '+Shift+T' })">
          <img v-if="appStore.alwaysOnTop" src="@/assets/icons/pin-active.svg" alt="Pin" />
          <img v-else src="@/assets/icons/pin.svg" alt="Pin" />
        </button>
        <button class="header-btn window-btn" @click="snapWindow('left')" :title="t('snapLeft', { key: ctrl + '+Shift+←' })">
          <img src="@/assets/icons/half-left.svg" alt="Snap left" />
        </button>
        <button class="header-btn window-btn" @click="snapWindow('right')" :title="t('snapRight', { key: ctrl + '+Shift+→' })">
          <img src="@/assets/icons/half-right.svg" alt="Snap right" />
        </button>
      </template>
      <button class="header-btn home-btn" @click="$emit('back')" :title="t('backToProjects', { key: ctrl + '+Shift+H' })">
        <img src="@/assets/icons/home.svg" alt="Home" />
      </button>
    </div>
  </header>
</template>

<script setup lang="ts">
import { computed } from 'vue'
import { useI18n } from 'vue-i18n'
import { snapWindow } from '@/composables/useAppShortcuts'
import { useSessionStore } from '@/stores/session'
import { useAppStore } from '@/stores/app'
import { ctrl, cmd } from '@/utils/platform'
import { isTauriEnv } from '@/api/platform'

const { t } = useI18n()

defineEmits<{
  back: []
  sidebar: []
}>()

const sessionStore = useSessionStore()
const appStore = useAppStore()

const sessionName = computed(() => sessionStore.activeTab?.name || '')
</script>

<style scoped>
.terminal-header {
  display: flex;
  align-items: center;
  height: 38px;
  padding: 0 12px;
  background: var(--bg-secondary);
  border-bottom: 1px solid var(--border-color);
}

.title-area {
  flex: 1;
  min-width: 0;
  text-align: center;
  padding: 0 8px;
  display: flex;
  align-items: baseline;
  justify-content: center;
  gap: 8px;
}

.header-title {
  font-size: 13px;
  font-weight: 500;
  color: var(--text-primary);
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
  flex-shrink: 1;
  min-width: 0;
}

.header-right {
  display: flex;
  align-items: center;
  gap: 2px;
  flex-shrink: 0;
}

.header-btn {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 32px;
  height: 32px;
  border: none;
  background: transparent;
  color: var(--text-secondary);
  cursor: pointer;
  border-radius: 6px;
  transition: all 0.15s ease;
}

.header-btn img {
  width: 18px;
  height: 18px;
}

.header-btn:hover {
  background: var(--hover-bg);
  color: var(--text-primary);
}

.header-btn.active {
  color: var(--accent-color);
}
</style>
