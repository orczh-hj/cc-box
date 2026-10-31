<template>
  <div class="plugin-item" :class="{ expanded: isExpanded, disabled: isDisabled }">
    <!-- 第一行：名称 + 开关 -->
    <div class="plugin-header" @click="toggleExpand">
      <img
        class="expand-icon"
        :class="{ expanded: isExpanded }"
        src="@/assets/icons/chevron.svg"
        alt="Toggle"
      />
      <span class="plugin-name">{{ plugin.name }}</span>
      <ToggleSwitch
        v-if="plugin.scope === 'user'"
        :modelValue="!isDisabled"
        :title="isDisabled ? t('enable') : t('disable')"
        @update:modelValue="onToggle"
      />
    </div>

    <!-- 第二行：版本（左） + 更新/卸载（右，触屏常显、桌面悬停显示） -->
    <div class="plugin-meta">
      <span class="plugin-version">v{{ plugin.version }}</span>
      <div class="plugin-actions" @click.stop>
        <button
          class="pa-btn"
          :disabled="!!actionBusy"
          :title="t('pluginUpdate')"
          @click="onUpdate"
        >
          {{ actionBusy === 'update' ? '…' : t('pluginUpdate') }}
        </button>
        <button
          class="pa-btn danger"
          :disabled="!!actionBusy"
          :title="t('pluginUninstall')"
          @click="onUninstall"
        >
          {{ t('pluginUninstall') }}
        </button>
      </div>
    </div>

    <!-- 第三行：来源市场名（无市场来源不显示） -->
    <div v-if="marketName" class="plugin-id">{{ marketName }}</div>

    <!-- Plugin Components Tags (collapsed view) -->
    <div v-if="!isExpanded && hasComponents" class="plugin-components">
      <span v-if="allSkills.length" class="component-tag skills">
        {{ allSkills.length }} Skills
      </span>
      <span v-if="plugin.agents?.length" class="component-tag agents">
        {{ plugin.agents.length }} Agents
      </span>
      <span v-if="plugin.mcpServers && Object.keys(plugin.mcpServers).length > 0" class="component-tag mcp">
        {{ Object.keys(plugin.mcpServers).length }} MCP
      </span>
      <span v-if="plugin.hooks?.length" class="component-tag hooks">
        {{ plugin.hooks.length }} Hooks
      </span>
      <span v-if="plugin.lsp?.length" class="component-tag lsp">
        {{ plugin.lsp.length }} LSP
      </span>
      <span v-if="plugin.monitors?.length" class="component-tag monitors">
        {{ plugin.monitors.length }} Monitors
      </span>
      <span v-if="plugin.themes?.length" class="component-tag themes">
        {{ plugin.themes.length }} Themes
      </span>
    </div>

    <!-- Expanded Components List -->
    <div v-if="isExpanded" class="plugin-expanded">
      <!-- Skills（commands 是老版 skill 形态，并入此处不区分） -->
      <div v-if="allSkills.length" class="component-section">
        <div class="section-title">{{ t('skills') }}</div>
        <div v-for="skill in allSkills" :key="skill.name" class="component-item" :class="{ expanded: expandedSkills[skill.name] }">
          <div class="item-header" @click="toggleSkillDetail(skill.name)">
            <img class="item-expand-icon" :class="{ expanded: expandedSkills[skill.name] }" src="@/assets/icons/chevron.svg" alt="Toggle" />
            <span class="item-name">{{ skill.name }}</span>
            <button class="item-use-btn" @click.stop="useSkill(skill.invokeFormat)" :title="t('useThisSkill')">
              <img src="@/assets/icons/skills.svg" :alt="t('skills')" class="item-icon" />
            </button>
          </div>
          <div v-if="expandedSkills[skill.name]" class="item-detail">
            <div v-if="skill.description" class="item-desc-full">{{ skill.description }}</div>
            <div v-else class="item-desc-empty">{{ t('noDescription') }}</div>
          </div>
        </div>
      </div>

      <!-- Agents -->
      <div v-if="plugin.agents?.length" class="component-section">
        <div class="section-title">{{ t('agents') }}</div>
        <div v-for="agent in plugin.agents" :key="agent.name" class="component-item" :class="{ expanded: expandedAgents[agent.name] }">
          <div class="item-header" @click="toggleAgentDetail(agent.name)">
            <img class="item-expand-icon" :class="{ expanded: expandedAgents[agent.name] }" src="@/assets/icons/chevron.svg" alt="Toggle" />
            <span class="item-name">{{ agent.name }}</span>
            <button class="item-use-btn" @click.stop="useAgent(agent.invokeFormat)" :title="t('useThisAgent')">
              <img src="@/assets/icons/agents.svg" :alt="t('agents')" class="item-icon" />
            </button>
          </div>
          <div v-if="expandedAgents[agent.name]" class="item-detail">
            <div v-if="agent.description" class="item-desc-full">{{ agent.description }}</div>
            <div v-else class="item-desc-empty">{{ t('noDescription') }}</div>
          </div>
        </div>
      </div>

      <!-- MCP Servers -->
      <div v-if="plugin.mcpServers && Object.keys(plugin.mcpServers).length > 0" class="component-section">
        <div class="section-title">{{ t('mcpServers') }}</div>
        <div v-for="(serverConfig, serverName) in plugin.mcpServers" :key="serverName" class="component-item mcp-server">
          <span class="item-name">{{ serverName }}</span>
          <span v-if="serverConfig.type" class="item-type">{{ serverConfig.type }}</span>
        </div>
      </div>

      <!-- Hooks（只读：事件 + matcher + 摘要） -->
      <div v-if="plugin.hooks?.length" class="component-section">
        <div class="section-title">{{ t('marketCompHooks') }}</div>
        <div v-for="(hook, i) in plugin.hooks" :key="`${hook.event}-${i}`" class="component-item plain-name hook-row">
          <span class="item-name">{{ hook.event }}</span>
          <span v-if="hook.matcher" class="item-type">{{ hook.matcher }}</span>
          <span v-if="hook.summary" class="hook-summary" :title="hook.summary">{{ hook.summary }}</span>
        </div>
      </div>

      <!-- LSP -->
      <div v-if="plugin.lsp?.length" class="component-section">
        <div class="section-title">{{ t('marketCompLsp') }}</div>
        <div v-for="name in plugin.lsp" :key="name" class="component-item plain-name">
          <span class="item-name">{{ name }}</span>
        </div>
      </div>

      <!-- Monitors -->
      <div v-if="plugin.monitors?.length" class="component-section">
        <div class="section-title">{{ t('marketCompMonitors') }}</div>
        <div v-for="name in plugin.monitors" :key="name" class="component-item plain-name">
          <span class="item-name">{{ name }}</span>
        </div>
      </div>

      <!-- Themes -->
      <div v-if="plugin.themes?.length" class="component-section">
        <div class="section-title">{{ t('marketCompThemes') }}</div>
        <div v-for="name in plugin.themes" :key="name" class="component-item plain-name">
          <span class="item-name">{{ name }}</span>
        </div>
      </div>

      <!-- Empty state -->
      <div v-if="!hasComponents" class="no-components">
        {{ t('noDescription') }}
      </div>
    </div>

    <!-- 操作失败提示 -->
    <div v-if="actionError" class="plugin-action-error">{{ actionError }}</div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onUnmounted } from 'vue'
import { useI18n } from 'vue-i18n'
import type { PluginInfo } from '@/types'
import { sendTerminalCommand } from '@/composables/useTerminalCommand'
import { useSidebarStore } from '@/stores/sidebar'
import { useAppStore } from '@/stores/app'
import { uiConfirm } from '@/composables/useConfirm'
import { updatePlugin, uninstallPlugin } from '@/api/tauri'
import ToggleSwitch from '@/components/common/ToggleSwitch.vue'

const { t } = useI18n()
const props = defineProps<{
  plugin: PluginInfo
}>()

const sidebarStore = useSidebarStore()
const appStore = useAppStore()

const isExpanded = ref(false)
const expandedSkills = ref<Record<string, boolean>>({})
const expandedAgents = ref<Record<string, boolean>>({})

const isDisabled = computed(() => props.plugin.enabled === false)

// 来源市场名（id = name@market；无 @ 表示本地/无市场来源，不显示）
const marketName = computed(() => {
  const idx = props.plugin.id.indexOf('@')
  return idx >= 0 ? props.plugin.id.slice(idx + 1) : ''
})

// commands 是老版 skill 形态，与 skills 合并展示（invoke 格式 /name）
const allSkills = computed(() => {
  const base = props.plugin.skills ? [...props.plugin.skills] : []
  for (const cmd of props.plugin.commands ?? []) {
    base.push({ name: cmd, invokeFormat: `/${cmd}` })
  }
  return base
})

// 更新/卸载进行中（区分动作显示省略号）与失败提示
const actionBusy = ref<'' | 'update' | 'uninstall'>('')
const actionError = ref('')
let actionErrorTimer: ReturnType<typeof setTimeout> | undefined

const hasComponents = computed(() => {
  return allSkills.value.length > 0 ||
    (props.plugin.agents && props.plugin.agents.length > 0) ||
    (props.plugin.mcpServers && Object.keys(props.plugin.mcpServers).length > 0) ||
    (props.plugin.hooks && props.plugin.hooks.length > 0) ||
    (props.plugin.lsp && props.plugin.lsp.length > 0) ||
    (props.plugin.monitors && props.plugin.monitors.length > 0) ||
    (props.plugin.themes && props.plugin.themes.length > 0)
})

function toggleExpand() {
  isExpanded.value = !isExpanded.value
}

function toggleSkillDetail(name: string) {
  expandedSkills.value[name] = !expandedSkills.value[name]
}

function toggleAgentDetail(name: string) {
  expandedAgents.value[name] = !expandedAgents.value[name]
}

function useSkill(invokeFormat: string) {
  if (isDisabled.value) return
  sendTerminalCommand(invokeFormat)
}

function useAgent(invokeFormat: string) {
  if (isDisabled.value) return
  sendTerminalCommand(invokeFormat)
}

onUnmounted(() => clearTimeout(actionErrorTimer))

async function onToggle(newValue: boolean) {
  try {
    await sidebarStore.togglePluginEnabled(props.plugin.id, newValue)
  } catch (err) {
    console.error('[PluginItem] toggle failed:', err)
  }
}

/** project 作用域 CLI 需要 cwd；user 传空（与市场浏览器同一命令） */
function lifecycleScope() {
  return {
    scope: props.plugin.scope,
    projectPath:
      props.plugin.scope === 'project'
        ? (props.plugin.projectPath ?? appStore.cwd ?? undefined)
        : undefined,
  }
}

function showActionError(msg: string) {
  actionError.value = msg
  clearTimeout(actionErrorTimer)
  actionErrorTimer = setTimeout(() => { actionError.value = '' }, 6000)
}

async function onUpdate() {
  if (actionBusy.value) return
  actionBusy.value = 'update'
  actionError.value = ''
  try {
    const { scope, projectPath } = lifecycleScope()
    await updatePlugin(props.plugin.id, scope, projectPath)
    if (appStore.cwd) sidebarStore.loadPlugins(appStore.cwd)
  } catch (err) {
    console.error('[PluginItem] update failed:', err)
    showActionError(`${t('pluginActionFailed')}: ${String(err)}`)
  } finally {
    actionBusy.value = ''
  }
}

async function onUninstall() {
  if (actionBusy.value) return
  const ok = await uiConfirm(t('pluginUninstallConfirm', { name: props.plugin.name }), {
    title: t('pluginUninstall'),
    danger: true,
  })
  if (!ok) return
  actionBusy.value = 'uninstall'
  actionError.value = ''
  try {
    const { scope, projectPath } = lifecycleScope()
    await uninstallPlugin(props.plugin.id, scope, projectPath)
    if (appStore.cwd) sidebarStore.loadPlugins(appStore.cwd)
  } catch (err) {
    console.error('[PluginItem] uninstall failed:', err)
    showActionError(`${t('pluginActionFailed')}: ${String(err)}`)
  } finally {
    actionBusy.value = ''
  }
}
</script>

<style scoped>
.plugin-item {
  background: var(--bg-primary);
  border-radius: 8px;
  padding: 10px 12px;
  transition: background 0.15s ease;
}

.plugin-item:hover {
  background: var(--hover-bg);
}

.plugin-item.disabled {
  opacity: 0.55;
}

.plugin-item.disabled .plugin-name {
  text-decoration: line-through;
  color: var(--text-tertiary);
}

.plugin-item.disabled .item-use-btn {
  pointer-events: none;
  opacity: 0.4;
}

.plugin-header {
  display: flex;
  align-items: center;
  gap: 8px;
  cursor: pointer;
  user-select: none;
}

/* 第二行：版本 + 管理按钮（右对齐，不与标题挤在一行） */
.plugin-meta {
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: 8px;
  margin-top: 4px;
  min-height: 20px;
}

/* 管理：更新/卸载——桌面悬停显示，触屏常显（避免误触占位） */
.plugin-actions {
  display: flex;
  gap: 4px;
  flex-shrink: 0;
}

@media (hover: hover) {
  .plugin-actions {
    visibility: hidden;
  }

  .plugin-item:hover .plugin-actions,
  .plugin-actions:focus-within {
    visibility: visible;
  }
}

.pa-btn {
  border: 1px solid var(--border-color);
  background: var(--bg-primary);
  color: var(--text-secondary);
  font-size: 11px;
  padding: 2px 8px;
  border-radius: 6px;
  cursor: pointer;
  line-height: 1.4;
}

.pa-btn:hover:not(:disabled) {
  color: var(--text-primary);
  background: var(--hover-bg);
}

.pa-btn:disabled {
  opacity: 0.5;
  cursor: default;
}

.pa-btn.danger {
  color: var(--danger-color, #e5484d);
}

.pa-btn.danger:hover:not(:disabled) {
  border-color: var(--danger-color, #e5484d);
}

.expand-icon {
  width: 14px;
  height: 14px;
  color: var(--text-secondary);
  flex-shrink: 0;
  transition: transform 0.15s ease;
}

.expand-icon.expanded {
  transform: rotate(90deg);
}

.plugin-name {
  font-size: 13px;
  font-weight: 500;
  color: var(--text-primary);
  flex: 1;
  min-width: 0;
  line-height: 1;
}

.plugin-version {
  margin-top: 4px;
  font-size: 11px;
  color: var(--text-tertiary);
}

.plugin-id {
  margin-top: 4px;
  font-size: 11px;
  color: var(--text-tertiary);
  font-family: var(--font-mono);
}

.plugin-components {
  margin-top: 8px;
  display: flex;
  gap: 6px;
}

.component-tag {
  font-size: 10px;
  padding: 2px 6px;
  border-radius: 4px;
}

.component-tag.mcp {
  background: #e3f2fd;
  color: #1565c0;
}

.component-tag.skills {
  background: #fff3e0;
  color: #e65100;
}

.component-tag.agents {
  background: #f3e5f5;
  color: #7b1fa2;
}

.component-tag.hooks {
  background: #ede7f6;
  color: #5e35b1;
}

.component-tag.lsp {
  background: #e0f7fa;
  color: #00838f;
}

.component-tag.monitors {
  background: #efebe9;
  color: #5d4037;
}

.component-tag.themes {
  background: #fce4ec;
  color: #ad1457;
}

/* Expanded styles */
.plugin-expanded {
  margin-top: 12px;
  padding-top: 8px;
  border-top: 1px solid var(--border-color);
}

.component-section {
  margin-bottom: 12px;
}

.section-title {
  font-size: 11px;
  font-weight: 600;
  color: var(--text-secondary);
  margin-bottom: 6px;
}

.component-item {
  background: var(--bg-secondary);
  border-radius: 6px;
  margin-bottom: 4px;
}

.item-header {
  display: flex;
  align-items: center;
  gap: 6px;
  padding: 8px;
  cursor: pointer;
  user-select: none;
}

.item-expand-icon {
  width: 10px;
  height: 10px;
  color: var(--text-tertiary);
  flex-shrink: 0;
  transition: transform 0.15s ease;
}

.item-expand-icon.expanded {
  transform: rotate(90deg);
}

.item-name {
  font-size: 12px;
  font-weight: 500;
  color: var(--text-primary);
  flex: 1;
}

.item-use-btn {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 20px;
  height: 20px;
  border: none;
  background: transparent;
  color: var(--text-tertiary);
  cursor: pointer;
  border-radius: 4px;
  flex-shrink: 0;
}

.item-use-btn:hover {
  color: var(--accent-color);
  background: var(--bg-tertiary);
}

.item-icon {
  width: 14px;
  height: 14px;
}

.item-detail {
  padding: 8px;
  padding-top: 0;
  margin-top: 6px;
  border-top: 1px solid var(--border-color);
}

.item-desc-full {
  font-size: 11px;
  color: var(--text-secondary);
  line-height: 1.5;
  white-space: pre-wrap;
}

.item-desc-empty {
  font-size: 11px;
  color: var(--text-tertiary);
  font-style: italic;
}

/* MCP Server styles */
.component-item.mcp-server {
  padding: 8px;
  cursor: default;
}

.component-item.mcp-server:hover {
  background: var(--bg-secondary);
}

.item-type {
  font-size: 10px;
  color: var(--text-tertiary);
  margin-left: 8px;
}

.no-components {
  font-size: 12px;
  color: var(--text-tertiary);
  text-align: center;
  padding: 12px;
}

.component-item.plain-name {
  padding: 8px;
  cursor: default;
}

.hook-row {
  display: flex;
  align-items: baseline;
  gap: 6px;
  min-width: 0;
}

.hook-summary {
  font-size: 10px;
  color: var(--text-tertiary);
  font-family: var(--font-mono);
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.plugin-action-error {
  margin-top: 6px;
  font-size: 11px;
  color: var(--danger-color, #e5484d);
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}
</style>