<template>
  <div class="provider-card" :class="{ active: isActive }">
    <span class="drag-handle" :title="t('dragToSort')">⋮⋮</span>
    <div class="provider-icon" :style="{ color: provider.iconColor || 'var(--accent-primary)' }">
      {{ getIconChar(provider.icon) }}
    </div>
    <div class="provider-info">
      <span class="provider-name">{{ provider.name }}</span>
      <span class="provider-notes" v-if="provider.notes">{{ provider.notes }}</span>
    </div>
    <div class="card-actions">
      <span class="active-badge" v-if="isActive">{{ t('activeBadge') }}</span>
      <button
        class="action-btn primary"
        @click.stop="$emit('activate')"
        :title="isActive ? t('reactivateBtn') : t('useBtn')"
      >
        {{ isActive ? t('reactivateBtn') : t('useBtn') }}
      </button>
      <button class="action-btn" @click.stop="$emit('edit')">{{ t('editBtn') }}</button>
      <button class="action-btn" @click.stop="$emit('test')" :disabled="isTesting">
        {{ isTesting ? t('testingBtn') : t('testBtn') }}
      </button>
      <button class="action-btn danger" @click.stop="$emit('delete')">{{ t('deleteBtn') }}</button>
    </div>
  </div>
</template>

<script setup lang="ts">
import { useI18n } from 'vue-i18n'
import type { Provider } from '@/types/provider'

const { t } = useI18n()

defineProps<{
  provider: Provider
  isActive: boolean
  isTesting: boolean
}>()

defineEmits<{
  activate: []
  edit: []
  test: []
  delete: []
}>()

function getIconChar(icon?: string): string {
  if (!icon) return 'P'
  const iconChars: Record<string, string> = {
    anthropic: 'A',
    deepseek: 'D',
    zhipu: 'Z',
    baidu: 'B',
    bailian: 'B',
    kimi: 'K',
    stepfun: 'S',
    minimax: 'M',
    doubao: 'D',
    siliconflow: 'S',
    openrouter: 'O',
    gemini: 'G',
    github: 'G',
    aws: 'A',
    aihubmix: 'A',
    modelscope: 'M',
    generic: 'P',
    custom: 'C',
  }
  return iconChars[icon] || icon[0].toUpperCase()
}
</script>

<style scoped>
.provider-card {
  display: flex;
  align-items: center;
  gap: 12px;
  padding: 12px 16px;
  background: var(--bg-primary);
  border: 1px solid var(--border-light);
  border-radius: var(--radius-lg);
  transition: border-color 0.15s;
}

.provider-card:hover {
  border-color: var(--border-color);
}

.provider-card.active {
  border-color: var(--accent-gold);
  border-width: 1.5px;
  background: var(--selected-bg);
}
</style>

<!-- 非 scoped：让 SortableJS 通过类名找到 handle -->
<style>
.drag-handle {
  color: var(--text-tertiary);
  font-size: 14px;
  cursor: grab;
  user-select: none;
  flex-shrink: 0;
  letter-spacing: -2px;
  opacity: 0.3;
  transition: opacity 0.15s;
}
.drag-handle:active { cursor: grabbing; }
.provider-card:hover .drag-handle { opacity: 0.7; }

.provider-icon {
  width: 34px;
  height: 34px;
  display: flex;
  align-items: center;
  justify-content: center;
  font-size: 15px;
  font-weight: 700;
  background: var(--bg-secondary);
  border-radius: var(--radius-md);
  flex-shrink: 0;
}

.provider-info {
  flex: 1;
  display: flex;
  flex-direction: column;
  gap: 2px;
  min-width: 0;
}

.provider-name {
  font-size: 14px;
  font-weight: 500;
  color: var(--text-primary);
}

.provider-notes {
  font-size: 12px;
  color: var(--text-tertiary);
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

/* hover 才显示操作——仅指针设备（触屏 hover:none 常显，否则操作按钮无法触达） */
@media (hover: hover) {
  .card-actions {
    opacity: 0;
    transition: opacity 0.15s;
  }
  .provider-card:hover .card-actions,
  .provider-card.active .card-actions {
    opacity: 1;
  }
}

.action-btn {
  padding: 4px 12px;
  background: transparent;
  color: var(--text-secondary);
  border: 1px solid var(--border-color);
  border-radius: var(--radius-sm);
  font-size: 12px;
  cursor: pointer;
  font-family: var(--font-sans);
  transition: all 0.15s;
}
.action-btn:hover {
  background: var(--hover-bg);
}

.action-btn.primary {
  color: var(--accent-primary);
  border-color: var(--accent-primary);
}
.action-btn.primary:hover {
  background: var(--accent-primary);
  color: #fff;
}

.action-btn.danger {
  color: var(--status-error);
}
.action-btn.danger:hover {
  background: rgba(196, 92, 74, 0.08);
}

.active-badge {
  font-size: 12px;
  color: var(--accent-gold-dark);
  font-weight: 600;
  padding: 4px 12px;
}

/* 手机断点：单行放不下 4 按钮 → 操作组换到第二行右对齐；触屏无 hover 常显 */
@media (max-width: 768px) {
  .provider-card {
    flex-wrap: wrap;
  }

  .card-actions {
    opacity: 1;
    flex-basis: 100%;
    justify-content: flex-end;
  }
}
</style>
