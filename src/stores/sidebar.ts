import { defineStore } from 'pinia'
import { ref, computed, type Ref } from 'vue'
import { getAllAgents, getAllSkills, getAllMcpServers, getAllPlugins, setSkillEnabled, setAgentEnabled, setMcpServerEnabled, setPluginEnabled } from '@/api/tauri'
import type { AgentInfo, SkillInfo, McpServerInfo, PluginInfo, UpdateInfo, ClaudeCliUpdateInfo } from '@/types'

export type SidebarPanelType = 'sessions' | 'skills' | 'agents' | 'mcp' | 'plugins' | 'files' | null

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

  /**
   * 构造一个列表加载器：
   * - **静默刷新**（silent）：只替换数据、不切换 loading 态。state-changed 回环与
   *   级联刷新走这里——面板用 `v-if="loading"` 渲染，切 loading 会让整块列表
   *   卸载重建（展开态、滚动位置全丢），这正是开关后"全量刷新"体验的根源。
   * - **请求序号**：并发刷新（自身回环 + 跨端广播 + 切项目）时丢弃乱序响应，
   *   旧响应不得覆盖新数据。
   */
  function createLoader<T>(
    list: Ref<T[]>,
    label: string,
    fetch: (cwd: string) => Promise<T[]>,
    loading: Ref<boolean>,
  ) {
    let latestReq = 0
    let loadingOwner = 0
    return async function load(cwd: string, opts?: { silent?: boolean }) {
      const reqId = ++latestReq
      if (!opts?.silent) {
        loadingOwner = reqId
        loading.value = true
      }
      try {
        const data = await fetch(cwd)
        if (reqId === latestReq) list.value = data
      } catch (err) {
        console.error(`[SidebarStore] Failed to load ${label}:`, err)
      } finally {
        // 只有本次请求持有的 loading 才负责关闭（被静默请求取代时仍由原请求收尾）
        if (reqId === loadingOwner) loading.value = false
      }
    }
  }

  const loadSkills = createLoader(skills, 'skills', getAllSkills, skillsLoading)
  const loadAgents = createLoader(agents, 'agents', getAllAgents, agentsLoading)
  const loadMcpServers = createLoader(mcpServers, 'mcp servers', getAllMcpServers, mcpServersLoading)
  const loadPlugins = createLoader(plugins, 'plugins', getAllPlugins, pluginsLoading)

  // ========== 本端写回环抑制 ==========
  // 开关成功后后端会广播 state-changed（桌面与远程都会把事件回环给发起方）。
  // 发起方已经做了乐观更新，再刷新会让列表替换/重新排序——这里按 section 登记
  // "本端写"，事件到达时消耗一次并跳过该 section 的刷新：条目原地响应、顺序不变。
  // 其他设备没有登记，照常静默刷新（跨端同步不受影响）。
  // TTL 兜底：广播丢失时计数不会永久吞掉后续跨端事件。
  const SELF_WRITE_TTL_MS = 5000
  const selfWrites = new Map<string, { count: number; expiresAt: number }>()

  /** 发起写操作前登记（广播回来时消费） */
  function markSelfWrite(section: string) {
    const now = Date.now()
    const entry = selfWrites.get(section)
    if (entry && entry.expiresAt > now) {
      entry.count += 1
      entry.expiresAt = now + SELF_WRITE_TTL_MS
    } else {
      selfWrites.set(section, { count: 1, expiresAt: now + SELF_WRITE_TTL_MS })
    }
  }

  /** 写操作失败：撤销登记（不会有对应广播，避免误吞别的设备的事件） */
  function cancelSelfWrite(section: string) {
    const entry = selfWrites.get(section)
    if (!entry) return
    entry.count -= 1
    if (entry.count <= 0) selfWrites.delete(section)
  }

  /** 消费一次本端写登记；true = 这次 state-changed 是自身回环，应跳过刷新 */
  function consumeSelfWrite(section: string): boolean {
    const entry = selfWrites.get(section)
    if (!entry) return false
    if (Date.now() > entry.expiresAt) {
      selfWrites.delete(section)
      return false
    }
    entry.count -= 1
    if (entry.count <= 0) selfWrites.delete(section)
    return true
  }

  /** 丢弃某 section 的全部待消费登记（权威全量重拉前调用，避免吞掉兜底刷新） */
  function clearSelfWrites(section: string) {
    selfWrites.delete(section)
  }

  // 加载所有 sidebar 数据
  async function loadAllSidebarData(cwd: string) {
    if (loadedCwd.value === cwd) return // 已加载过

    loadedCwd.value = cwd
    selfWrites.clear() // 切项目：旧项目的待消费回环登记作废

    // 切项目先清空旧数据：加载失败时不会把上一个项目的内容留在面板里
    skills.value = []
    agents.value = []
    mcpServers.value = []
    plugins.value = []

    // 并行加载所有数据
    await Promise.all([
      loadSkills(cwd),
      loadAgents(cwd),
      loadMcpServers(cwd),
      loadPlugins(cwd),
    ])
  }

  // ========== 用户级资源开关 ==========
  // 设计要点：
  // - 开关本身只做乐观更新 + 调后端；列表刷新由 `state-changed` 回环统一驱动
  //   （sections.ts 以 silent 模式重拉，不闪 loading）
  // - 乐观更新：直接改 store 中对应项的 enabled 字段，Vue 响应式刷新单项 ToggleSwitch 视觉
  // - 失败回滚：API 失败时按身份重新定位并还原 enabled，错误向上抛由组件 catch
  // - 数据安全由后端原子操作 + 冲突检测保证（同名检测、路径穿越防御）
  // - 多窗口 stale 场景：B 操作已被 A 改过的项时后端返回错误，store 回滚，B 的 UI 短暂闪回原状态

  /**
   * 定位开关要修改的条目。用户级开关只作用于 **user 级** 资源（后端只移动
   * 用户级目录/配置），不能因同名而改到 project/plugin 项（列表里 project 项
   * 排在前面，裸 name 匹配会误改它，造成"点了没反应"）。
   * active/disabled 同名共存时才按"当前状态与目标相反"精确匹配。
   */
  function findToggleIndex<T extends { name: string; sourceType: string; enabled?: boolean }>(
    list: T[],
    name: string,
    targetEnabled: boolean,
  ): number {
    const isUserLevel = (item: T) => item.sourceType === 'user'
    const matchesTarget = (item: T) => (item.enabled !== false) !== targetEnabled
    const precise = list.findIndex(
      item => item.name === name && isUserLevel(item) && matchesTarget(item),
    )
    if (precise >= 0) return precise
    return list.findIndex(item => item.name === name && isUserLevel(item))
  }

  /**
   * 失败回滚：刷新可能已替换整个数组，必须按身份重新定位（旧索引不可信）。
   * 身份包含 `sourceLabel`——active 与 disabled 同名共存时，两行的 name/sourceType
   * 相同，只靠 enabled 会回滚到另一行（把用户没点的那行翻过去）。
   */
  function revertToggle<T extends { name: string; sourceType: string; sourceLabel?: string; enabled?: boolean }>(
    list: T[],
    identity: { name: string; sourceType: string; sourceLabel?: string },
    optimisticEnabled: boolean,
  ) {
    const isOptimistic = (item: T) => (item.enabled !== false) === optimisticEnabled
    // 有 sourceLabel 时严格按标签匹配：并发静默刷新可能已把数组换成权威状态，
    // 此时找不到就 no-op（权威状态已落地），绝不能退化成按 enabled 匹配——
    // 那会把同名的 active 行翻掉。
    if (identity.sourceLabel !== undefined) {
      const idx = list.findIndex(
        item =>
          item.name === identity.name &&
          item.sourceType === identity.sourceType &&
          item.sourceLabel === identity.sourceLabel &&
          isOptimistic(item),
      )
      if (idx >= 0) list[idx] = { ...list[idx], enabled: !optimisticEnabled }
      return
    }
    // 物品不在列表中（乐观更新前就没找到）时，只按 name + sourceType 兜底
    const idx = list.findIndex(
      item =>
        item.name === identity.name &&
        item.sourceType === identity.sourceType &&
        isOptimistic(item),
    )
    if (idx >= 0) list[idx] = { ...list[idx], enabled: !optimisticEnabled }
  }

  async function toggleSkillEnabled(name: string, enabled: boolean) {
    const idx = findToggleIndex(skills.value, name, enabled)
    const identity = idx >= 0
      ? { name: skills.value[idx].name, sourceType: skills.value[idx].sourceType, sourceLabel: skills.value[idx].sourceLabel }
      : { name, sourceType: 'user' }
    if (idx >= 0) skills.value[idx] = { ...skills.value[idx], enabled }
    markSelfWrite('skills')
    try {
      await setSkillEnabled(name, enabled)
    } catch (err) {
      cancelSelfWrite('skills')
      revertToggle(skills.value, identity, enabled)
      throw err
    }
  }

  async function toggleAgentEnabled(name: string, enabled: boolean) {
    const idx = findToggleIndex(agents.value, name, enabled)
    const identity = idx >= 0
      ? { name: agents.value[idx].name, sourceType: agents.value[idx].sourceType, sourceLabel: agents.value[idx].sourceLabel }
      : { name, sourceType: 'user' }
    if (idx >= 0) agents.value[idx] = { ...agents.value[idx], enabled }
    markSelfWrite('agents')
    try {
      await setAgentEnabled(name, enabled)
    } catch (err) {
      cancelSelfWrite('agents')
      revertToggle(agents.value, identity, enabled)
      throw err
    }
  }

  async function toggleMcpServerEnabled(name: string, enabled: boolean) {
    const idx = findToggleIndex(mcpServers.value, name, enabled)
    const identity = idx >= 0
      ? { name: mcpServers.value[idx].name, sourceType: mcpServers.value[idx].sourceType, sourceLabel: mcpServers.value[idx].sourceLabel }
      : { name, sourceType: 'user' }
    if (idx >= 0) mcpServers.value[idx] = { ...mcpServers.value[idx], enabled }
    markSelfWrite('mcp')
    try {
      await setMcpServerEnabled(name, enabled)
    } catch (err) {
      cancelSelfWrite('mcp')
      revertToggle(mcpServers.value, identity, enabled)
      throw err
    }
  }

  async function togglePluginEnabled(pluginId: string, enabled: boolean) {
    const idx = plugins.value.findIndex(p => p.id === pluginId)
    const old = idx >= 0 ? { ...plugins.value[idx] } : null
    if (idx >= 0) plugins.value[idx] = { ...plugins.value[idx], enabled }
    // 本端回环跳过 plugins 刷新（卡片原地翻转）；级联的 skills/agents/mcp
    // 仍由状态中心照常刷新（插件子项需要出现/消失）
    markSelfWrite('plugins')
    try {
      await setPluginEnabled(pluginId, enabled)
    } catch (err) {
      cancelSelfWrite('plugins')
      // 刷新可能替换过数组：按 id（唯一）重新定位，不用过期索引
      const i = plugins.value.findIndex(p => p.id === pluginId)
      if (i >= 0 && old) plugins.value[i] = old
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
    // 本端写回环登记（状态中心消费；测试可直接模拟）
    markSelfWrite,
    cancelSelfWrite,
    consumeSelfWrite,
    clearSelfWrites,
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