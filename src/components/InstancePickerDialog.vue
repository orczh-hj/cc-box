<template>
  <Teleport to="body">
    <Transition name="modal">
      <div v-if="visible" class="picker-overlay" @click.self="choose(undefined)">
        <div class="picker-card">
          <h3 class="picker-title">{{ t('instancePickerTitle') }}</h3>
          <p class="picker-hint">{{ t('instancePickerHint') }}</p>
          <button
            v-for="inst in instances"
            :key="inst.id"
            class="picker-item"
            @click="choose(inst.id)"
          >
            {{ inst.name }}
          </button>
          <!-- 新建窗口：拉起后自动把会话路由到新窗口 -->
          <button class="picker-item picker-new" @click="choose(NEW_INSTANCE)">
            {{ t('instancePickerNew') }}
          </button>
          <button class="picker-cancel" @click="choose(undefined)">
            {{ t('cancel') }}
          </button>
        </div>
      </div>
    </Transition>
  </Teleport>
</template>

<script setup lang="ts">
import { useI18n } from 'vue-i18n'
import { useInstancePicker, NEW_INSTANCE } from '@/composables/useInstancePicker'

const { t } = useI18n()
const { visible, instances, choose } = useInstancePicker()
</script>

<style scoped>
.picker-overlay {
  position: fixed;
  inset: 0;
  display: flex;
  align-items: center;
  justify-content: center;
  background: rgba(0, 0, 0, 0.35);
  z-index: 2100;
}
.picker-card {
  display: flex;
  flex-direction: column;
  gap: 8px;
  width: min(320px, 84vw);
  padding: 22px 20px;
  background: var(--bg-secondary, #f5f3ee);
  border: 1px solid var(--border-light, #e8e4dc);
  border-radius: 12px;
}
.picker-title {
  font-size: 16px;
  color: var(--text-primary, #1a1816);
}
.picker-hint {
  font-size: 12px;
  line-height: 1.5;
  color: var(--text-secondary, #5a5550);
}
.picker-item {
  padding: 12px;
  background: var(--bg-primary, #fff);
  border: 1px solid var(--border-color, #d9d5cc);
  border-radius: 8px;
  font-size: 14px;
  color: var(--text-primary, #1a1816);
  cursor: pointer;
  text-align: left;
}
.picker-item:active {
  border-color: var(--accent-primary, #1e3a5f);
}
/* 「＋ 新建窗口」：虚线边框弱化为动作项（非既有窗口） */
.picker-new {
  border-style: dashed;
  color: var(--text-secondary, #5a5550);
}
.picker-cancel {
  padding: 10px;
  background: transparent;
  border: none;
  font-size: 13px;
  color: var(--text-secondary, #5a5550);
  cursor: pointer;
}
</style>
