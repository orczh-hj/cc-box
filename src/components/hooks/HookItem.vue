<template>
  <div
    class="hook-item"
    :class="{ disabled: !item.enabled }"
    @click="canEdit && $emit('edit')"
  >
    <div class="hook-info">
      <div class="hook-main">
        <span class="hook-event">{{ item.event }}</span>
        <span v-if="item.hookType && item.hookType !== 'command'" class="hook-matcher">{{ item.hookType }}</span>
        <span v-if="item.matcher" class="hook-matcher">{{ item.matcher }}</span>
        <span v-if="item.hookCount > 1" class="hook-multi" :title="t('editorHookMultiHint')">
          ×{{ item.hookCount }}
        </span>
      </div>
      <span class="hook-command" :title="item.command">{{ item.command }}</span>
      <div v-if="toggleError" class="hook-toggle-error" :title="toggleError">{{ toggleError }}</div>
    </div>
    <ToggleSwitch
      :modelValue="item.enabled"
      :title="item.enabled ? t('disable') : t('enable')"
      @update:modelValue="onToggle"
    />
  </div>
</template>

<script setup lang="ts">
import { computed, ref, onUnmounted } from 'vue'
import { useI18n } from 'vue-i18n'
import { useSidebarStore } from '@/stores/sidebar'
import type { HookListItem } from '@/api/tauri'
import ToggleSwitch from '@/components/common/ToggleSwitch.vue'

const { t } = useI18n()
const sidebarStore = useSidebarStore()

const props = defineProps<{
  item: HookListItem
}>()

defineEmits<{
  edit: []
}>()

// 停用条目不在 settings 中（编辑定位必失败，先启用再编辑）；多命令条目整条替换会丢命令
const canEdit = computed(() => props.item.enabled && props.item.hookCount <= 1)

/** 开关失败提示（5s 自动消失），与 SkillItem 同一反馈模式 */
const toggleError = ref('')
let errorTimer: ReturnType<typeof setTimeout> | undefined

async function onToggle(enabled: boolean) {
  // 乐观更新 + 本端写回环抑制统一在 store（与其他资源开关一致）
  try {
    await sidebarStore.toggleHookEnabled(props.item, enabled)
    toggleError.value = ''
  } catch (err) {
    console.error('[HookItem] toggle failed:', err)
    toggleError.value = String(err)
    clearTimeout(errorTimer)
    errorTimer = setTimeout(() => { toggleError.value = '' }, 5000)
  }
}

onUnmounted(() => clearTimeout(errorTimer))
</script>

<style scoped>
.hook-item {
  background: var(--bg-primary);
  border-radius: 8px;
  padding: 10px 12px;
  cursor: pointer;
  user-select: none;
  display: flex;
  align-items: center;
  gap: 10px;
  transition: background 0.15s ease;
}

.hook-item:hover {
  background: var(--hover-bg);
}

.hook-item.disabled {
  opacity: 0.55;
}

.hook-item.disabled .hook-event {
  text-decoration: line-through;
  color: var(--text-tertiary);
}

.hook-info {
  flex: 1;
  min-width: 0;
}

.hook-main {
  display: flex;
  align-items: center;
  gap: 8px;
  min-width: 0;
}

.hook-event {
  font-size: 13px;
  font-weight: 500;
  color: var(--text-primary);
  font-family: var(--font-mono);
  flex-shrink: 0;
}

.hook-matcher {
  font-size: 11px;
  color: var(--text-secondary);
  background: var(--bg-tertiary);
  padding: 1px 6px;
  border-radius: 4px;
  font-family: var(--font-mono);
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.hook-multi {
  font-size: 10px;
  color: var(--text-tertiary);
  background: var(--bg-tertiary);
  padding: 1px 5px;
  border-radius: 10px;
  flex-shrink: 0;
}

.hook-command {
  margin-top: 4px;
  font-size: 11px;
  color: var(--text-tertiary);
  font-family: var(--font-mono);
  display: block;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.hook-toggle-error {
  margin-top: 4px;
  font-size: 11px;
  color: var(--danger-color, #e5484d);
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}
</style>
