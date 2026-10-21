<template>
  <div class="preset-panel">
    <div class="panel-header">
      <button class="back-btn" @click="$emit('close')">← {{ t('back') }}</button>
      <span class="panel-title">{{ t('selectPreset') }}</span>
    </div>

    <div class="panel-content">
      <div class="filter-row">
        <div class="category-filter">
          <button
            v-for="cat in categories"
            :key="cat.value"
            class="filter-btn"
            :class="{ active: selectedCategory === cat.value }"
            @click="selectedCategory = cat.value"
          >
            {{ cat.label }}
          </button>
        </div>
        <button class="custom-btn" @click="handleSelectCustom">
          <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" stroke-linecap="round" stroke-linejoin="round">
            <path d="M12 5v14M5 12h14" />
          </svg>
          <span class="custom-btn-text">{{ t('customProvider') }}</span>
        </button>
      </div>

      <div class="preset-grid">
        <div
          v-for="preset in filteredPresets"
          :key="preset.name"
          class="preset-card"
          @click="$emit('select', preset)"
        >
          <div class="preset-icon" :style="{ color: preset.iconColor || '#6366F1' }">
            <ProviderIcon :icon="preset.icon" />
          </div>
          <div class="preset-info">
            <span class="preset-name">{{ preset.name }}</span>
            <span class="preset-category">{{ getCategoryLabel(preset.category) }}</span>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed } from 'vue'
import { useI18n } from 'vue-i18n'
import { providerPresets } from '@/config/providerPresets'
import { getCategoryLabel } from '@/config/providerCategories'
import type { ProviderPreset } from '@/types/provider'
import ProviderIcon from './ProviderIcon.vue'

const emit = defineEmits<{
  close: []
  select: [preset: ProviderPreset]
}>()

const { t } = useI18n()

const presets = ref<ProviderPreset[]>(providerPresets)

const customPreset = computed(() =>
  presets.value.find(p => p.category === 'custom')
)

const categories = computed(() => [
  { value: '', label: t('categoryAll') },
  { value: 'official', label: t('categoryOfficial') },
  { value: 'cn_official', label: t('categoryCn') },
  { value: 'aggregator', label: t('categoryAggregator') },
  { value: 'cloud_provider', label: t('categoryCloud') },
  { value: 'third_party', label: t('categoryThirdParty') },
  { value: 'local', label: t('categoryLocal') },
])

const selectedCategory = ref<string>('')

const filteredPresets = computed(() => {
  if (!selectedCategory.value) {
    return presets.value.filter(p => !p.hidden && p.category !== 'custom')
  }
  return presets.value.filter(p => p.category === selectedCategory.value && !p.hidden && p.category !== 'custom')
})

function handleSelectCustom() {
  if (customPreset.value) {
    emit('select', customPreset.value)
  }
}
</script>

<style scoped>
.preset-panel {
  position: absolute;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background: var(--bg-primary);
  z-index: 100;
  display: flex;
  flex-direction: column;
}

.panel-header {
  display: flex;
  align-items: center;
  gap: 16px;
  padding: 16px 20px;
  border-bottom: 1px solid var(--border-color);
}

.back-btn {
  padding: 6px 12px;
  background: transparent;
  color: var(--text-secondary);
  border: 1px solid var(--border-color);
  border-radius: 4px;
  font-size: 13px;
  cursor: pointer;
}

.back-btn:hover {
  background: var(--hover-bg);
}

.panel-title {
  font-size: 16px;
  font-weight: 600;
  color: var(--text-primary);
}

.panel-content {
  flex: 1;
  padding: 20px;
  overflow-y: auto;
}

/* 分类行 + 自定义按钮 */
.filter-row {
  display: flex;
  align-items: center;
  justify-content: space-between;
  margin-bottom: 20px;
  gap: 12px;
}

.category-filter {
  display: flex;
  gap: 8px;
  flex-wrap: wrap;
}

.filter-btn {
  padding: 6px 12px;
  background: var(--bg-secondary);
  color: var(--text-secondary);
  border: 1px solid var(--border-color);
  border-radius: 4px;
  font-size: 12px;
  cursor: pointer;
  transition: background 0.15s;
}

.filter-btn:hover {
  background: var(--hover-bg);
}

.filter-btn.active {
  background: var(--accent-color);
  color: white;
  border-color: var(--accent-color);
}

/* 自定义 Provider 按钮 — 与 active 筛选按钮一致 */
.custom-btn {
  display: flex;
  align-items: center;
  gap: 6px;
  padding: 6px 12px;
  background: var(--accent-color);
  color: white;
  border: 1px solid var(--accent-color);
  border-radius: 4px;
  font-size: 12px;
  cursor: pointer;
  transition: opacity 0.15s;
  white-space: nowrap;
  flex-shrink: 0;
}

.custom-btn:hover {
  opacity: 0.9;
}

.custom-btn-text {
  line-height: 1;
}

/* 预设网格 */
.preset-grid {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(180px, 1fr));
  gap: 12px;
}

.preset-card {
  display: flex;
  align-items: center;
  gap: 12px;
  padding: 12px;
  background: var(--bg-secondary);
  border: 1px solid var(--border-color);
  border-radius: 8px;
  cursor: pointer;
  transition: border-color 0.15s;
}

.preset-card:hover {
  border-color: var(--accent-color);
}

.preset-icon {
  width: 36px;
  height: 36px;
  display: flex;
  align-items: center;
  justify-content: center;
  font-size: 18px;
  font-weight: 600;
  background: var(--bg-primary);
  border-radius: 6px;
}

.preset-info {
  flex: 1;
  display: flex;
  flex-direction: column;
  gap: 2px;
}

.preset-name {
  font-size: 13px;
  font-weight: 500;
  color: var(--text-primary);
}

.preset-category {
  font-size: 11px;
  color: var(--text-tertiary);
}
</style>