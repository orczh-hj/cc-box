<template>
  <div class="settings-view">
    <!-- 桌面：左侧竖导航；手机：顶部横向滚动 tab（结构分支，共享 navItems 数据） -->
    <nav v-if="!isMobile" class="settings-nav">
      <div class="settings-nav-header">
        <span class="nav-title">{{ t('settings') }}</span>
        <button class="close-btn" @click="$emit('close')">
          <img src="@/assets/icons/close.svg" :alt="t('close')" />
        </button>
      </div>
      <div class="nav-items">
        <button
          v-for="item in navItems"
          :key="item.id"
          class="nav-item"
          :class="{ active: sidebarStore.activeSettingsSection === item.id }"
          @click="sidebarStore.activeSettingsSection = item.id"
        >
          <span class="nav-label">{{ item.label }}</span>
          <span v-if="item.id === 'update' && sidebarStore.updateAvailable" class="nav-badge"></span>
        </button>
      </div>
      <div class="nav-footer">
        <span class="footer-version">CC-Box v{{ currentVersion }}</span>
      </div>
    </nav>

    <nav v-else class="settings-nav mobile">
      <div class="settings-nav-header">
        <span class="nav-title">{{ t('settings') }}</span>
        <button class="close-btn" @click="$emit('close')">
          <img src="@/assets/icons/close.svg" :alt="t('close')" />
        </button>
      </div>
      <div class="nav-items horizontal">
        <button
          v-for="item in navItems"
          :key="item.id"
          class="nav-item"
          :class="{ active: sidebarStore.activeSettingsSection === item.id }"
          @click="sidebarStore.activeSettingsSection = item.id"
        >
          <span class="nav-label">{{ item.label }}</span>
          <span v-if="item.id === 'update' && sidebarStore.updateAvailable" class="nav-badge"></span>
        </button>
      </div>
    </nav>

    <!-- 右侧内容 -->
    <div class="settings-content">
      <AppearanceSection v-if="sidebarStore.activeSettingsSection === 'appearance'" />
      <StartupSection v-if="sidebarStore.activeSettingsSection === 'startup'" />
      <ProvidersSection v-if="sidebarStore.activeSettingsSection === 'providers'" />
      <ShortcutsSection v-if="sidebarStore.activeSettingsSection === 'shortcuts'" />
      <UpdateSection v-if="sidebarStore.activeSettingsSection === 'update'" />
      <AboutSection v-if="sidebarStore.activeSettingsSection === 'about'" />
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue'
import { useI18n } from 'vue-i18n'
import { useSidebarStore } from '@/stores/sidebar'
import { isTauriEnv } from '@/api/platform'
import { useIsMobile } from '@/composables/useMediaQuery'
import AppearanceSection from './sections/AppearanceSection.vue'
import StartupSection from './sections/StartupSection.vue'
import ProvidersSection from './sections/ProvidersSection.vue'
import ShortcutsSection from './sections/ShortcutsSection.vue'
import UpdateSection from './sections/UpdateSection.vue'
import AboutSection from './sections/AboutSection.vue'

const { t } = useI18n()
const currentVersion = __APP_VERSION__
const isMobile = useIsMobile()

defineEmits<{ close: [] }>()

const sidebarStore = useSidebarStore()

const navItems = computed(() => [
  { id: 'appearance', label: t('appearance') },
  { id: 'startup', label: t('startup') },
  { id: 'providers', label: t('providers') },
  { id: 'shortcuts', label: t('shortcuts') },
  // 更新/安装器为系统级操作，仅桌面可用
  ...(isTauriEnv ? [{ id: 'update', label: t('update') }] : []),
  { id: 'about', label: t('about') },
])
</script>

<style scoped>
.settings-view {
  display: flex;
  flex: 1;
  height: 100%;
  background: var(--bg-primary);
}

/* 左侧导航 */
.settings-nav {
  width: 180px;
  background: var(--bg-secondary);
  border-right: 1px solid var(--border-color);
  display: flex;
  flex-direction: column;
  flex-shrink: 0;
}

.settings-nav-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 12px 14px;
  border-bottom: 1px solid var(--border-color);
}

.nav-title {
  font-size: 14px;
  font-weight: 600;
  color: var(--text-primary);
}

.close-btn {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 24px;
  height: 24px;
  border: none;
  background: transparent;
  color: var(--text-secondary);
  cursor: pointer;
  border-radius: 4px;
}

.close-btn img {
  width: 16px;
  height: 16px;
}

.close-btn:hover {
  background: var(--hover-bg);
  color: var(--text-primary);
}

.nav-items {
  flex: 1;
  padding: 8px 8px;
  display: flex;
  flex-direction: column;
  gap: 2px;
}

.nav-item {
  position: relative;
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 8px 10px;
  border: none;
  background: transparent;
  color: var(--text-secondary);
  font-size: 13px;
  cursor: pointer;
  border-radius: 6px;
  text-align: left;
  transition: all 0.15s ease;
}

.nav-item:hover {
  background: var(--hover-bg);
  color: var(--text-primary);
}

.nav-item.active {
  background: var(--selected-bg);
  color: var(--accent-color);
  font-weight: 500;
}

.nav-item.active::before {
  content: '';
  position: absolute;
  left: -8px;
  top: 6px;
  bottom: 6px;
  width: 3px;
  background: var(--accent-color);
  border-radius: 0 2px 2px 0;
}

.nav-badge {
  width: 8px;
  height: 8px;
  background: var(--error-color);
  border-radius: 50%;
  flex-shrink: 0;
}

.nav-footer {
  padding: 12px 14px;
  border-top: 1px solid var(--border-color);
}

.footer-version {
  font-size: 11px;
  color: var(--text-tertiary);
  font-variant-numeric: tabular-nums;
}

/* 右侧内容 */
.settings-content {
  flex: 1;
  overflow-y: auto;
  padding: 28px 36px;
}

/* ============================================
 * 手机断点（isMobile 结构分支的样式侧）：
 * 竖导航 → 顶部横向滚动 tab；内容区缩小留白
 * ============================================ */
.settings-nav.mobile {
  width: 100%;
  border-right: none;
  border-bottom: 1px solid var(--border-color);
}

.settings-nav.mobile .nav-items {
  flex-direction: row;
  flex-wrap: nowrap;
  overflow-x: auto;
  -webkit-overflow-scrolling: touch;
  padding: 8px 10px;
  gap: 6px;
  scrollbar-width: none;
}

.settings-nav.mobile .nav-items::-webkit-scrollbar {
  display: none;
}

.settings-nav.mobile .nav-item {
  flex-shrink: 0;
  white-space: nowrap;
  justify-content: center;
  padding: 6px 12px;
}

/* 激活指示条：左侧竖条 → 底部横条 */
.settings-nav.mobile .nav-item.active::before {
  left: 10px;
  right: 10px;
  top: auto;
  bottom: 2px;
  width: auto;
  height: 3px;
  border-radius: 2px 2px 0 0;
}

@media (max-width: 768px) {
  /* 手机整体变纵向布局：导航在上、内容在下 */
  .settings-view {
    flex-direction: column;
  }

  .settings-content {
    padding: 16px 16px calc(16px + var(--safe-area-bottom));
  }
}
</style>
