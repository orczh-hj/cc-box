<template>
  <div class="mode-switch" role="tablist">
    <button
      v-for="m in modes"
      :key="m.value"
      type="button"
      class="mode-btn"
      :class="{ active: modelValue === m.value }"
      role="tab"
      :aria-selected="modelValue === m.value"
      @click="$emit('update:modelValue', m.value)"
    >
      {{ m.label }}
    </button>
  </div>
</template>

<script setup lang="ts">
defineProps<{
  modelValue: string
  modes: Array<{ value: string; label: string }>
}>()

defineEmits<{ (e: 'update:modelValue', v: string): void }>()
</script>

<style scoped>
.mode-switch {
  display: inline-flex;
  background: var(--bg-tertiary);
  border-radius: 8px;
  padding: 2px;
  gap: 2px;
  flex-shrink: 0;
}

.mode-btn {
  border: none;
  background: transparent;
  color: var(--text-secondary);
  font-size: 12px;
  padding: 4px 12px;
  border-radius: 6px;
  cursor: pointer;
  transition: all 0.15s ease;
  white-space: nowrap;
}

.mode-btn.active {
  background: var(--bg-primary);
  color: var(--text-primary);
  box-shadow: 0 1px 2px rgba(0, 0, 0, 0.08);
}
</style>
