<template>
  <Transition name="slide-fade">
    <div v-if="sidebarStore.showSettings" class="settings-overlay">
      <SettingsView @close="sidebarStore.closeSettings()" />
    </div>
  </Transition>
</template>

<script setup lang="ts">
import { useSidebarStore } from '@/stores/sidebar'
import SettingsView from './SettingsView.vue'

const sidebarStore = useSidebarStore()
</script>

<style scoped>
.settings-overlay {
  position: fixed;
  /* 桌面模式让出自定义标题栏（app-root 的 --titlebar-h 变量，浏览器模式为 0） */
  top: var(--titlebar-h, 32px);
  left: 0;
  right: 0;
  bottom: 0;
  z-index: 1000;
  display: flex;
  background: var(--bg-primary);
}

.slide-fade-enter-active,
.slide-fade-leave-active {
  transition: all 0.25s ease;
}

.slide-fade-enter-from,
.slide-fade-leave-to {
  opacity: 0;
  transform: translateX(20px);
}
</style>