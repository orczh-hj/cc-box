import { defineStore } from 'pinia'
import { ref, computed } from 'vue'
import { getAllAgents, getAllSkills, getAllMcpServers, getAllPlugins, setSkillEnabled, setAgentEnabled, setMcpServerEnabled, setPluginEnabled } from '@/api/tauri'
import type { AgentInfo, SkillInfo, McpServerInfo, PluginInfo, UpdateInfo, ClaudeCliUpdateInfo } from '@/types'

export type SidebarPanelType = 'sessions' | 'skills' | 'agents' | 'mcp' | 'plugins' | null

export const useSidebarStore = defineStore('sidebar', () => {
  const activePanel = ref<SidebarPanelType>(null)
  const panelVisible = ref(false)

  // 设置模式
  const showSettings = ref(false)
  const activeSettingsSection = ref<string>('appearance')
  const updateInfo = ref<UpdateInfo | null>(null)
  const claudeCliUpdateInfo = ref<ClaudeCliUpdateInfo | null>(null)
  const updateAvailable = computed(() => {
    // 仅由 CC-Box 自身更新驱动（启动不再检测 Claude CLI 更新）
    return updateInfo.value?.hasUpdate ?? false
  })

  function setUpdateInfo(info: UpdateInfo) {
    updateInfo.value = info
  }

  function setClaudeCliUpdateInfo(info: ClaudeCliUpdateInfo) {
    claudeCliUpdateInfo.value = info
  }

  // Skills 面板折叠状态（按来源分组）
  const skillsExpandedGroups = ref({
    project: true,
    user: true,
    plugin: true
  })

  // Agents 面板折叠状态
  const agentsExpandedGroups = ref({
    builtin: true,
    plugin: true,
    user: true,
    project: true
  })

  // MCP 面板折叠状态
  const mcpExpandedGroups = ref({
    plugin: true,
    user: true,
    project: true
  })

  // Plugins 面板折叠状态
  const pluginsExpandedGroups = ref({
    user: true,
    project: true
  })

  // ========== 预加载数据 ==========

  // 数据
  const skills = ref<SkillInfo[]>([])
  const agents = ref<AgentInfo[]>([])
  const mcpServers = ref<McpServerInfo[]>([])
  const plugins = ref<PluginInfo[]>([])

  // 加载状态
  const skillsLoading = ref(false)
  const agentsLoading = ref(false)
  const mcpServersLoading = ref(false)
  const pluginsLoading = ref(false)

  // 已加载的 cwd（用于判断是否需要重新加载）
  const loadedCwd = ref<string | null>(null)

  // 加载所有 sidebar 数据
  async function loadAllSidebarData(cwd: string) {
    if (loadedCwd.value === cwd) return // 已加载过

    loadedCwd.value = cwd

    // 并行加载所有数据
    await Promise.all([
      loadSkills(cwd),
      loadAgents(cwd),
      loadMcpServers(cwd),
      loadPlugins(cwd),
    ])
  }

  async function loadSkills(cwd: string) {
    skillsLoading.value = true
    try {
      skills.value = await getAllSkills(cwd)
    } catch (err) {
      console.error('[SidebarStore] Failed to load skills:', err)
    } finally {
      skillsLoading.value = false
    }
  }

  async function loadAgents(cwd: string) {
    agentsLoading.value = true
    try {
      agents.value = await getAllAgents(cwd)
    } catch (err) {
      console.error('[SidebarStore] Failed to load agents:', err)
    } finally {
      agentsLoading.value = false
    }
  }

  async function loadMcpServers(cwd: string) {
    mcpServersLoading.value = true
    try {
      mcpServers.value = await getAllMcpServers(cwd)
    } catch (err) {
      console.error('[SidebarStore] Failed to load mcp servers:', err)
    } finally {
      mcpServersLoading.value = false
    }
  }

  async function loadPlugins(cwd: string) {
    pluginsLoading.value = true
    try {
      plugins.value = await getAllPlugins(cwd)
    } catch (err) {
      console.error('[SidebarStore] Failed to load plugins:', err)
    } finally {
      pluginsLoading.value = false
    }
  }

  // ========== 用户级资源开关 ==========
  // 设计要点：
  // - 多窗口之间不同步（不监听焦点、不调 loadXxx），避免效率低/闪烁/延迟
  // - 乐观更新：直接改 store 中对应项的 enabled 字段，Vue 响应式刷新单项 ToggleSwitch 视觉
  // - 失败回滚：API 失败时还原 enabled，错误向上抛由组件 catch（仅 console.error）
  // - 数据安全由后端原子操作 + 冲突检测保证（同名检测、路径穿越防御）
  // - 多窗口 stale 场景：B 操作已被 A 改过的项时后端返回错误，store 回滚，B 的 UI 短暂闪回原状态

  async function toggleSkillEnabled(name: string, enabled: boolean) {
    const idx = skills.value.findIndex(s => s.name === name)
    const old = idx >= 0 ? { ...skills.value[idx] } : null
    if (idx >= 0) skills.value[idx] = { ...skills.value[idx], enabled }
    try {
      await setSkillEnabled(name, enabled)
    } catch (err) {
      if (idx >= 0 && old) skills.value[idx] = old
      throw err
    }
  }

  async function toggleAgentEnabled(name: string, enabled: boolean) {
    const idx = agents.value.findIndex(a => a.name === name)
    const old = idx >= 0 ? { ...agents.value[idx] } : null
    if (idx >= 0) agents.value[idx] = { ...agents.value[idx], enabled }
    try {
      await setAgentEnabled(name, enabled)
    } catch (err) {
      if (idx >= 0 && old) agents.value[idx] = old
      throw err
    }
  }

  async function toggleMcpServerEnabled(name: string, enabled: boolean) {
    const idx = mcpServers.value.findIndex(m => m.name === name)
    const old = idx >= 0 ? { ...mcpServers.value[idx] } : null
    if (idx >= 0) mcpServers.value[idx] = { ...mcpServers.value[idx], enabled }
    try {
      await setMcpServerEnabled(name, enabled)
    } catch (err) {
      if (idx >= 0 && old) mcpServers.value[idx] = old
      throw err
    }
  }

  async function togglePluginEnabled(pluginId: string, enabled: boolean) {
    const idx = plugins.value.findIndex(p => p.id === pluginId)
    const old = idx >= 0 ? { ...plugins.value[idx] } : null
    if (idx >= 0) plugins.value[idx] = { ...plugins.value[idx], enabled }
    try {
      await setPluginEnabled(pluginId, enabled)
      // plugin 开关影响其 skills/agents/mcp 是否展示——子项刷新统一由
      // state-changed 回环事件驱动（App.vue 订阅 → 状态中心 sections.ts），不再手动 reload
    } catch (err) {
      if (idx >= 0 && old) plugins.value[idx] = old
      throw err
    }
  }

  // 切换面板
  function togglePanel(panel: SidebarPanelType) {
    // 如果设置打开，先关闭设置再打开面板
    if (showSettings.value) {
      showSettings.value = false
      activePanel.value = panel
      panelVisible.value = true
      return
    }

    if (activePanel.value === panel && panelVisible.value) {
      closePanel()
    } else {
      activePanel.value = panel
      panelVisible.value = true
    }
  }

  // 设置模式
  function openSettings(section?: string) {
    panelVisible.value = false
    activePanel.value = null
    showSettings.value = true
    if (section) activeSettingsSection.value = section
  }

  function closeSettings() {
    showSettings.value = false
  }

  function toggleSettings() {
    if (showSettings.value) {
      closeSettings()
    } else {
      openSettings()
    }
  }

  // 关闭面板
  function closePanel() {
    panelVisible.value = false
    setTimeout(() => {
      activePanel.value = null
    }, 250)
  }

  // 切换 Skill 分组折叠
  function toggleSkillGroup(group: keyof typeof skillsExpandedGroups.value) {
    skillsExpandedGroups.value[group] = !skillsExpandedGroups.value[group]
  }

  // 切换 Agent 分组折叠
  function toggleAgentGroup(group: keyof typeof agentsExpandedGroups.value) {
    agentsExpandedGroups.value[group] = !agentsExpandedGroups.value[group]
  }

  // 切换 MCP 分组折叠
  function toggleMcpGroup(group: keyof typeof mcpExpandedGroups.value) {
    mcpExpandedGroups.value[group] = !mcpExpandedGroups.value[group]
  }

  // 切换 Plugin 分组折叠
  function togglePluginGroup(group: keyof typeof pluginsExpandedGroups.value) {
    pluginsExpandedGroups.value[group] = !pluginsExpandedGroups.value[group]
  }

  return {
    activePanel,
    panelVisible,
    showSettings,
    activeSettingsSection,
    updateInfo,
    claudeCliUpdateInfo,
    updateAvailable,
    setUpdateInfo,
    setClaudeCliUpdateInfo,
    skillsExpandedGroups,
    agentsExpandedGroups,
    mcpExpandedGroups,
    pluginsExpandedGroups,
    // 预加载数据
    skills,
    agents,
    mcpServers,
    plugins,
    skillsLoading,
    agentsLoading,
    mcpServersLoading,
    pluginsLoading,
    loadedCwd,
    loadAllSidebarData,
    loadSkills,
    loadAgents,
    loadMcpServers,
    loadPlugins,
    // 操作函数
    togglePanel,
    closePanel,
    openSettings,
    closeSettings,
    toggleSettings,
    toggleSkillGroup,
    toggleAgentGroup,
    toggleMcpGroup,
    togglePluginGroup,
    // 用户级资源开关
    toggleSkillEnabled,
    toggleAgentEnabled,
    toggleMcpServerEnabled,
    togglePluginEnabled
  }
})