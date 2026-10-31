<template>
  <Teleport to="body">
    <Transition name="editor-modal">
      <div v-if="visible" class="editor-overlay" @click.self="requestClose">
        <div class="editor-card" :class="{ sheet: isMobile }" role="dialog" :aria-label="title">
          <!-- 头部：标题 + 关闭 -->
          <div class="editor-header">
            <span class="editor-title">{{ title }}</span>
            <slot name="header-extra" />
            <button class="editor-close" @click="requestClose" :title="t('close')">
              <img src="@/assets/icons/close.svg" :alt="t('close')" />
            </button>
          </div>

          <!-- 内容（表单/源码由各编辑器自组织） -->
          <div class="editor-body">
            <slot />
          </div>

          <!-- 底部：删除（左） + 取消/保存（右） -->
          <div class="editor-footer">
            <button
              v-if="deletable"
              class="editor-delete"
              :disabled="busy"
              @click="$emit('delete')"
            >
              <img src="@/assets/icons/close.svg" :alt="t('delete')" />
              <span>{{ t('delete') }}</span>
            </button>
            <div class="editor-footer-spacer"></div>
            <slot name="footer-extra" />
            <button class="editor-btn" :disabled="busy" @click="requestClose">{{ t('cancel') }}</button>
            <button class="editor-btn primary" :disabled="busy || !canSave" @click="$emit('save')">
              {{ busy ? t('editorSaving') : t('save') }}
            </button>
          </div>
        </div>
      </div>
    </Transition>
  </Teleport>
</template>

<script setup lang="ts">
import { watch, onUnmounted } from 'vue'
import { useI18n } from 'vue-i18n'
import { useIsMobile } from '@/composables/useMediaQuery'
import { uiConfirm } from '@/composables/useConfirm'

const { t } = useI18n()

const props = withDefaults(defineProps<{
  visible: boolean
  title: string
  busy?: boolean
  /** 内容有未保存修改：Esc/遮罩关闭时确认 */
  dirty?: boolean
  canSave?: boolean
  deletable?: boolean
}>(), {
  busy: false,
  dirty: false,
  canSave: true,
  deletable: false,
})

const emit = defineEmits<{
  close: []
  save: []
  delete: []
}>()

const isMobile = useIsMobile()

/** Esc/遮罩/取消统一走确认：有未保存修改时二次确认丢弃 */
async function requestClose() {
  if (props.busy) return
  if (props.dirty && !(await uiConfirm(t('editorDiscardConfirm')))) return
  emit('close')
}

function onKeydown(e: KeyboardEvent) {
  if (e.key === 'Escape' && props.visible) requestClose()
}

watch(
  () => props.visible,
  v => {
    if (v) window.addEventListener('keydown', onKeydown)
    else window.removeEventListener('keydown', onKeydown)
  },
  { immediate: true }
)
onUnmounted(() => window.removeEventListener('keydown', onKeydown))
</script>

<style scoped>
.editor-overlay {
  position: fixed;
  inset: 0;
  background: rgba(0, 0, 0, 0.45);
  z-index: 1500;
  display: flex;
  align-items: center;
  justify-content: center;
  padding: 24px;
}

.editor-card {
  width: min(720px, 100%);
  max-height: min(85vh, 720px);
  box-sizing: border-box;
  background: var(--bg-primary);
  border: 1px solid var(--border-color);
  border-radius: 12px;
  box-shadow: var(--shadow-lg);
  display: flex;
  flex-direction: column;
  overflow: hidden;
}

.editor-header {
  display: flex;
  align-items: center;
  gap: 10px;
  padding: 14px 18px;
  border-bottom: 1px solid var(--border-color);
  flex-shrink: 0;
}

.editor-title {
  flex: 1;
  min-width: 0;
  font-size: 15px;
  font-weight: 600;
  color: var(--text-primary);
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.editor-close {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 26px;
  height: 26px;
  border: none;
  background: transparent;
  color: var(--text-secondary);
  cursor: pointer;
  border-radius: 6px;
  flex-shrink: 0;
}

.editor-close img {
  width: 14px;
  height: 14px;
}

.editor-close:hover {
  background: var(--hover-bg);
  color: var(--text-primary);
}

.editor-body {
  flex: 1;
  min-height: 0;
  overflow-y: auto;
  padding: 16px 18px;
}

.editor-footer {
  display: flex;
  align-items: center;
  gap: 10px;
  padding: 12px 18px;
  border-top: 1px solid var(--border-color);
  flex-shrink: 0;
}

.editor-footer-spacer {
  flex: 1;
}

.editor-delete {
  display: flex;
  align-items: center;
  gap: 5px;
  border: none;
  background: transparent;
  color: var(--danger-color, #e5484d);
  font-size: 13px;
  cursor: pointer;
  padding: 6px 8px;
  border-radius: 6px;
}

.editor-delete img {
  width: 13px;
  height: 13px;
}

.editor-delete:hover:not(:disabled) {
  background: rgba(229, 72, 77, 0.08);
}

.editor-delete:disabled {
  opacity: 0.5;
  cursor: default;
}

/* .editor-btn 家族在 global.css（非 scoped）：slot 内按钮与 MarketBrowser 共用 */

/* 进出场 */
.editor-modal-enter-active,
.editor-modal-leave-active {
  transition: opacity 0.18s ease;
}

.editor-modal-enter-active .editor-card,
.editor-modal-leave-active .editor-card {
  transition: transform 0.18s ease;
}

.editor-modal-enter-from,
.editor-modal-leave-to {
  opacity: 0;
}

.editor-modal-enter-from .editor-card,
.editor-modal-leave-to .editor-card {
  transform: translateY(8px);
}

/* ============================================
   手机断点（≤768px 且触屏）：居中卡片 → 底部抽屉
   （结构分支用 useIsMobile 挂 sheet class，样式用同一断点）
   ============================================ */
@media (max-width: 768px) and (hover: none) {
  .editor-overlay {
    padding: 0;
    align-items: flex-end;
  }

  .editor-card,
  .editor-card.sheet {
    width: 100%;
    max-height: 88vh;
    border-radius: 14px 14px 0 0;
    border-left: none;
    border-right: none;
    border-bottom: none;
    padding-bottom: var(--safe-area-bottom);
  }
}
</style>
