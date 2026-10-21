<template>
  <div class="skill-item" :class="{ expanded: isExpanded, disabled: isDisabled }">
    <!-- Skill Header -->
    <div class="skill-header" @click="toggleExpand">
      <img
        class="expand-icon"
        :class="{ expanded: isExpanded }"
        src="@/assets/icons/chevron.svg"
        alt="Toggle"
      />
      <div class="skill-info">
        <span class="skill-name">{{ skill.displayName }}</span>
        <span v-if="skill.sourceType === 'plugin'" class="skill-full-name">{{ skill.name }}</span>
      </div>
      <ToggleSwitch
        v-if="skill.sourceType === 'user'"
        :modelValue="!isDisabled"
        :title="isDisabled ? t('enable') : t('disable')"
        @update:modelValue="onToggle"
      />
      <button class="use-btn" :disabled="isDisabled" @click.stop="emitUseSkill" :title="t('useThisSkill')">
        <img src="@/assets/icons/use.svg" :alt="t('useBtn')" />
      </button>
    </div>

    <!-- 开关失败反馈（乐观更新已回滚，一行截断小字；完整原因悬停 title） -->
    <div v-if="toggleError" class="skill-toggle-error" :title="toggleError">{{ toggleError }}</div>

    <!-- Skill Details (expanded) -->
    <div v-if="isExpanded" class="skill-details">
      <div v-if="skill.description" class="skill-description-full">
        {{ skill.description }}
      </div>
      <div v-else class="skill-description-empty">
        {{ t('noDescription') }}
      </div>
      <div class="skill-invoke-format">
        <span class="invoke-label">Invoke:</span>
        <span class="invoke-value">{{ skill.invokeFormat }}</span>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onUnmounted } from 'vue'
import { useI18n } from 'vue-i18n'
import type { SkillInfo } from '@/types'
import { sendTerminalCommand } from '@/composables/useTerminalCommand'
import { useSidebarStore } from '@/stores/sidebar'
import ToggleSwitch from '@/components/common/ToggleSwitch.vue'

const { t } = useI18n()
const props = defineProps<{
  skill: SkillInfo
}>()

const sidebarStore = useSidebarStore()

const isExpanded = ref(false)
const isDisabled = computed(() => props.skill.enabled === false)

function toggleExpand() {
  isExpanded.value = !isExpanded.value
}

function emitUseSkill() {
  if (isDisabled.value) return
  sendTerminalCommand(props.skill.invokeFormat)
}

/** 开关失败提示（5s 自动消失）：后端错误原文展示（如 skill 目录被运行中的
 * Claude Code 会话锁定时提示关闭会话后重试），桌面/远程同一组件同一反馈 */
const toggleError = ref('')
let errorTimer: ReturnType<typeof setTimeout> | undefined

async function onToggle(newValue: boolean) {
  try {
    await sidebarStore.toggleSkillEnabled(props.skill.name, newValue)
    toggleError.value = ''
  } catch (err) {
    console.error('[SkillItem] toggle failed:', err)
    toggleError.value = String(err)
    clearTimeout(errorTimer)
    errorTimer = setTimeout(() => { toggleError.value = '' }, 5000)
  }
}

onUnmounted(() => clearTimeout(errorTimer))
</script>

<style scoped>
.skill-item {
  background: var(--bg-primary);
  border-radius: 8px;
  padding: 10px 12px;
  transition: background 0.15s ease;
}

.skill-item:hover {
  background: var(--hover-bg);
}

.skill-item.disabled {
  opacity: 0.55;
}

.skill-item.disabled .skill-name {
  text-decoration: line-through;
  color: var(--text-tertiary);
}

.skill-item.disabled .use-btn {
  pointer-events: none;
  opacity: 0.4;
}

.skill-header {
  display: flex;
  align-items: center;
  gap: 8px;
  cursor: pointer;
  user-select: none;
}

.skill-toggle-error {
  margin-top: 4px;
  font-size: 11px;
  color: var(--danger-color, #e5484d);
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.expand-icon {
  width: 12px;
  height: 12px;
  color: var(--text-secondary);
  flex-shrink: 0;
  transition: transform 0.15s ease;
}

.expand-icon.expanded {
  transform: rotate(90deg);
}

.skill-info {
  flex: 1;
  min-width: 0;
}

.skill-name {
  font-size: 13px;
  font-weight: 500;
  color: var(--text-primary);
}

.skill-full-name {
  display: block;
  font-size: 11px;
  color: var(--text-tertiary);
  font-family: var(--font-mono);
  margin-top: 2px;
}

.use-btn {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 24px;
  height: 24px;
  border: none;
  background: transparent;
  color: var(--text-tertiary);
  cursor: pointer;
  border-radius: 4px;
  flex-shrink: 0;
}

.use-btn img {
  width: 14px;
  height: 14px;
}

.use-btn:hover {
  color: var(--accent-color);
  background: var(--bg-tertiary);
}

.skill-details {
  margin-top: 10px;
  padding-top: 10px;
  border-top: 1px solid var(--border-color);
}

.skill-description-full {
  font-size: 12px;
  color: var(--text-secondary);
  line-height: 1.5;
  white-space: pre-wrap;
  overflow-wrap: break-word;
  word-break: break-word;
}

.skill-description-empty {
  font-size: 12px;
  color: var(--text-tertiary);
  font-style: italic;
}

.skill-invoke-format {
  margin-top: 8px;
  display: flex;
  gap: 6px;
  font-size: 11px;
}

.invoke-label {
  color: var(--text-tertiary);
}

.invoke-value {
  font-family: var(--font-mono);
  color: var(--text-primary);
  background: var(--bg-tertiary);
  padding: 2px 6px;
  border-radius: 4px;
}
</style>