<template>
  <div v-if="toasts.length" class="toast-host">
    <TransitionGroup name="toast">
      <div
        v-for="item in toasts"
        :key="item.id"
        class="toast-item"
        :class="`toast-${item.type}`"
        role="status"
      >
        <span class="toast-text">{{ item.text }}</span>
        <button
          class="toast-close"
          :title="t('close')"
          :aria-label="t('close')"
          @click="remove(item.id)"
        >
          <!-- 用 SVG 而非 × 字符：字符受字体基线影响会偏下，几何描边可精确居中 -->
          <svg width="12" height="12" viewBox="0 0 24 24" fill="none"
               stroke="currentColor" stroke-width="2" stroke-linecap="round">
            <path d="M18 6 6 18M6 6l12 12" />
          </svg>
        </button>
      </div>
    </TransitionGroup>
  </div>
</template>

<script setup lang="ts">
/** 全局通知宿主（App.vue 挂载一次）：渲染 useToast 队列，右上角 × 手动关闭 */
import { useI18n } from 'vue-i18n'
import { useToast } from '@/composables/useToast'

const { t } = useI18n()
const { toasts, remove } = useToast()
</script>

<style scoped>
.toast-host {
  position: fixed;
  left: 50%;
  /* 手机底部有固定 IconBar（~56px），留出安全距离 + 刘海屏安全区 */
  bottom: calc(80px + env(safe-area-inset-bottom, 0px));
  transform: translateX(-50%);
  z-index: 4000; /* 高于全局控制权遮罩（2000）与设置浮层 */
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 8px;
  max-width: min(92vw, 420px);
  pointer-events: none;
}

.toast-item {
  pointer-events: auto;
  max-width: 100%;
  box-sizing: border-box;
  display: flex;
  align-items: center; /* 文本与 × 按钮垂直居中 */
  gap: 8px;
  padding: 10px 8px 10px 14px;
  border-radius: 8px;
  border: 1px solid transparent;
  font-size: 13px;
  line-height: 1.5;
  box-shadow: 0 6px 20px rgba(0, 0, 0, 0.18);
}

.toast-text {
  flex: 1;
  min-width: 0;
  text-align: left;
  word-break: break-word;
}

.toast-close {
  flex-shrink: 0;
  width: 22px;
  height: 22px;
  display: flex;
  align-items: center;
  justify-content: center;
  border: none;
  border-radius: 50%;
  background: transparent;
  color: currentColor;
  opacity: 0.7;
  padding: 0;
  cursor: pointer;
}

.toast-close svg {
  display: block;
}

@media (hover: hover) {
  .toast-close:hover {
    opacity: 1;
    background: rgba(127, 127, 127, 0.18);
  }
}

.toast-success {
  color: var(--status-success, #22c55e);
  background: rgba(34, 197, 94, 0.12);
  border-color: rgba(34, 197, 94, 0.35);
}

.toast-info {
  color: var(--accent-color, #4a9eff);
  background: rgba(74, 158, 255, 0.12);
  border-color: rgba(74, 158, 255, 0.35);
}

.toast-error {
  color: var(--error-color, #e5534b);
  background: rgba(229, 83, 75, 0.12);
  border-color: rgba(229, 83, 75, 0.35);
}

.toast-enter-active,
.toast-leave-active {
  transition: opacity 0.2s ease, transform 0.2s ease;
}

.toast-enter-from,
.toast-leave-to {
  opacity: 0;
  transform: translateY(6px);
}
</style>
