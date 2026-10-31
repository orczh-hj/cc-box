<template>
  <div class="agent-item" :class="{ expanded: isExpanded, disabled: isDisabled }">
    <!-- Agent Header -->
    <div class="agent-header" @click="toggleExpand">
      <img
        class="expand-icon"
        :class="{ expanded: isExpanded }"
        src="@/assets/icons/chevron.svg"
        alt="Toggle"
      />
      <div class="agent-info">
        <span class="agent-name">{{ agent.displayName }}</span>
        <span v-if="agent.sourceType === 'plugin'" class="agent-full-name">{{ pluginName }}</span>
      </div>
      <span v-if="agent.model" class="agent-model">{{ agent.model }}</span>
      <ToggleSwitch
        v-if="canToggle"
        :modelValue="!isDisabled"
        :title="isDisabled ? t('enable') : t('disable')"
        @update:modelValue="onToggle"
      />
      <button v-if="canToggle" class="edit-btn" @click.stop="$emit('edit', agent)" :title="t('edit')">
        <img src="@/assets/icons/edit.svg" :alt="t('edit')" />
      </button>
      <button class="use-btn" :disabled="isDisabled" @click.stop="emitUseAgent" :title="t('useThisAgent')">
        <img src="@/assets/icons/use.svg" :alt="t('useBtn')" />
      </button>
    </div>

    <!-- 开关失败反馈（乐观更新已回滚；与 SkillItem 一致） -->
    <div v-if="toggleError" class="agent-toggle-error" :title="toggleError">{{ toggleError }}</div>

    <!-- Agent Details (expanded) -->
    <div v-if="isExpanded" class="agent-details">
      <div v-if="agent.description" class="agent-description-full">
        {{ agent.description }}
      </div>
      <div v-else class="agent-description-empty">
        {{ t('noDescription') }}
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onUnmounted } from 'vue'
import { useI18n } from 'vue-i18n'
import type { AgentInfo } from '@/types'
import { sendTerminalCommand } from '@/composables/useTerminalCommand'
import { useSidebarStore } from '@/stores/sidebar'
import ToggleSwitch from '@/components/common/ToggleSwitch.vue'

const { t } = useI18n()
const props = defineProps<{
  agent: AgentInfo
}>()

defineEmits<{
  edit: [agent: AgentInfo]
}>()

const sidebarStore = useSidebarStore()

const isExpanded = ref(false)
const isDisabled = computed(() => props.agent.enabled === false)
// plugin 条目 name 为 "plugin:agent"，第二行只展示 plugin 名（agent 名已在首行）
const pluginName = computed(() => props.agent.name.split(':')[0] ?? '')
const canToggle = computed(() => props.agent.sourceType === 'user' || props.agent.sourceType === 'project')

function toggleExpand() {
  isExpanded.value = !isExpanded.value
}

function emitUseAgent() {
  if (isDisabled.value) return
  sendTerminalCommand(props.agent.invokeFormat)
}

/** 开关失败提示（5s 自动消失）：此前只 console.error，用户看到的是"操作无效" */
const toggleError = ref('')
let errorTimer: ReturnType<typeof setTimeout> | undefined

async function onToggle(newValue: boolean) {
  try {
    await sidebarStore.toggleAgentEnabled(props.agent.name, newValue, props.agent.sourceType)
    toggleError.value = ''
  } catch (err) {
    console.error('[AgentItem] toggle failed:', err)
    toggleError.value = String(err)
    clearTimeout(errorTimer)
    errorTimer = setTimeout(() => { toggleError.value = '' }, 5000)
  }
}

onUnmounted(() => clearTimeout(errorTimer))
</script>

<style scoped>
.agent-item {
  background: var(--bg-primary);
  border-radius: 8px;
  padding: 10px 12px;
  transition: background 0.15s ease;
}

.agent-item:hover {
  background: var(--hover-bg);
}

.agent-item.disabled {
  opacity: 0.55;
}

.agent-toggle-error {
  margin-top: 4px;
  font-size: 11px;
  color: var(--danger-color, #e5484d);
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.edit-btn {
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

.edit-btn img {
  width: 13px;
  height: 13px;
}

.edit-btn:hover {
  color: var(--accent-color);
  background: var(--bg-tertiary);
}

.agent-item.disabled .agent-name {
  text-decoration: line-through;
  color: var(--text-tertiary);
}

.agent-item.disabled .use-btn {
  pointer-events: none;
  opacity: 0.4;
}

.agent-header {
  display: flex;
  align-items: center;
  gap: 8px;
  cursor: pointer;
  user-select: none;
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

.agent-info {
  flex: 1;
  min-width: 0;
}

.agent-name {
  font-size: 13px;
  font-weight: 500;
  color: var(--text-primary);
}

.agent-full-name {
  display: block;
  font-size: 11px;
  color: var(--text-tertiary);
  font-family: var(--font-mono);
  margin-top: 2px;
}

.agent-model {
  font-size: 10px;
  padding: 2px 6px;
  border-radius: 4px;
  background: #e3f2fd;
  color: #1565c0;
  flex-shrink: 0;
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

.agent-details {
  margin-top: 10px;
  padding-top: 10px;
  border-top: 1px solid var(--border-color);
}

.agent-description-full {
  font-size: 12px;
  color: var(--text-secondary);
  line-height: 1.5;
  white-space: pre-wrap;
  overflow-wrap: break-word;
  word-break: break-word;
}

.agent-description-empty {
  font-size: 12px;
  color: var(--text-tertiary);
  font-style: italic;
}
</style>