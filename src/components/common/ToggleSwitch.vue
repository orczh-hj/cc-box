<template>
  <button
    type="button"
    class="toggle-switch"
    :class="{ on: modelValue, disabled: disabled }"
    role="switch"
    :aria-checked="modelValue"
    :disabled="disabled"
    :title="title"
    @click.stop="onClick"
  >
    <span class="toggle-knob"></span>
  </button>
</template>

<script setup lang="ts">
const props = withDefaults(defineProps<{
  modelValue: boolean
  disabled?: boolean
  title?: string
}>(), {
  disabled: false,
  title: '',
})

const emit = defineEmits<{ (e: 'update:modelValue', v: boolean): void }>()

function onClick() {
  if (props.disabled) return
  emit('update:modelValue', !props.modelValue)
}
</script>

<style scoped>
.toggle-switch {
  width: 28px;
  height: 16px;
  border-radius: 8px;
  background: var(--bg-tertiary);
  border: 1px solid var(--border-color);
  cursor: pointer;
  position: relative;
  flex-shrink: 0;
  transition: background 0.15s ease;
  padding: 0;
  margin: 0;
  box-sizing: border-box;
}

.toggle-switch.on {
  background: var(--accent-color);
  border-color: var(--accent-color);
}

.toggle-switch.disabled {
  opacity: 0.4;
  cursor: not-allowed;
}

.toggle-switch:hover:not(.disabled) {
  opacity: 0.9;
}

.toggle-knob {
  position: absolute;
  top: 1px;
  left: 1px;
  width: 12px;
  height: 12px;
  border-radius: 50%;
  background: #fff;
  transition: transform 0.15s ease;
  box-shadow: 0 1px 2px rgba(0, 0, 0, 0.2);
}

.toggle-switch.on .toggle-knob {
  transform: translateX(12px);
}

/* 触屏：放大点击目标（远程手机端高频操作） */
@media (max-width: 768px) and (hover: none) {
  .toggle-switch {
    width: 38px;
    height: 22px;
    border-radius: 11px;
  }

  .toggle-knob {
    width: 18px;
    height: 18px;
  }

  .toggle-switch.on .toggle-knob {
    transform: translateX(16px);
  }
}
</style>
