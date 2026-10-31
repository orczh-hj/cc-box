<template>
  <Teleport to="body">
    <Transition name="confirm-fade">
      <div v-if="confirmState.visible" class="confirm-overlay" @click.self="settleConfirm(false)">
        <div class="confirm-card" role="alertdialog" aria-modal="true">
          <h3 v-if="confirmState.title" class="confirm-title">{{ confirmState.title }}</h3>
          <p class="confirm-message">{{ confirmState.message }}</p>
          <div class="confirm-actions">
            <button class="editor-btn" ref="cancelRef" @click="settleConfirm(false)">
              {{ t('cancel') }}
            </button>
            <button
              class="editor-btn"
              :class="confirmState.danger ? 'danger' : 'primary'"
              @click="settleConfirm(true)"
            >
              {{ confirmState.okText || t('confirmOk') }}
            </button>
          </div>
        </div>
      </div>
    </Transition>
  </Teleport>
</template>

<script setup lang="ts">
import { nextTick, ref, watch, onUnmounted } from 'vue'
import { useI18n } from 'vue-i18n'
import { confirmState, settleConfirm } from '@/composables/useConfirm'

const { t } = useI18n()
const cancelRef = ref<HTMLButtonElement | null>(null)

function onKeydown(e: KeyboardEvent) {
  if (!confirmState.visible) return
  if (e.key === 'Escape') {
    e.stopPropagation()
    settleConfirm(false)
  } else if (e.key === 'Enter') {
    e.stopPropagation()
    settleConfirm(true)
  }
}

watch(
  () => confirmState.visible,
  v => {
    if (v) {
      window.addEventListener('keydown', onKeydown, true)
      // 焦点落在取消上：危险操作默认不选中确认
      void nextTick(() => cancelRef.value?.focus())
    } else {
      window.removeEventListener('keydown', onKeydown, true)
    }
  }
)
onUnmounted(() => window.removeEventListener('keydown', onKeydown, true))
</script>

<style scoped>
.confirm-overlay {
  position: fixed;
  inset: 0;
  background: rgba(26, 24, 22, 0.42);
  z-index: 1800;
  display: flex;
  align-items: center;
  justify-content: center;
  padding: 24px;
}

.confirm-card {
  width: min(360px, 92vw);
  box-sizing: border-box;
  background: var(--bg-primary);
  border: 1px solid var(--border-color);
  border-radius: 12px;
  box-shadow: 0 16px 48px rgba(26, 24, 22, 0.22);
  padding: 18px 18px 14px;
}

.confirm-title {
  margin: 0 0 6px;
  font-size: 14px;
  font-weight: 600;
  color: var(--text-primary);
}

.confirm-message {
  margin: 0 0 14px;
  font-size: 13px;
  line-height: 1.65;
  color: var(--text-secondary);
  white-space: pre-line;
  overflow-wrap: anywhere;
}

.confirm-actions {
  display: flex;
  justify-content: flex-end;
  gap: 8px;
}

.confirm-fade-enter-active,
.confirm-fade-leave-active {
  transition: opacity 0.15s ease;
}

.confirm-fade-enter-active .confirm-card,
.confirm-fade-leave-active .confirm-card {
  transition: transform 0.15s ease;
}

.confirm-fade-enter-from,
.confirm-fade-leave-to {
  opacity: 0;
}

.confirm-fade-enter-from .confirm-card,
.confirm-fade-leave-to .confirm-card {
  transform: translateY(4px);
}
</style>
