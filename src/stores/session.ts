import { defineStore } from 'pinia'
import { ref, computed, reactive } from 'vue'
import { uuid } from '@/utils/uuid'
import {
  getSessions,
  ptyKill,
  searchSessionMessages
} from '@/api/tauri'
import { useAppStore } from './app'
import type { SessionSearchResult } from '@/types'

// ==================== Tab-Centric 数据模型 ====================

/**
 * 终端 Tab — 跨越 PTY 生命周期的稳定 UI 单元
 *
 * - Tab 创建后一直存在，直到用户主动关闭
 * - PTY 退出后 status 变为 stopped，但 Tab 保留
 * - 有 sessionId 的 stopped Tab 可以 --resume 重启
 * - 无 sessionId 的 stopped Tab 重启时作为新会话
 */
export interface TerminalTab {
  tabId: string
  projectPath: string
  ptyId: string | null
  sessionId: string | null
  /** sessionId 演变历史（CLI --resume 会 fork 出新 id，旧 id 仍属同一对话） */
  sessionIds: string[]
  name: string
  status: 'starting' | 'running' | 'stopped'
  createdAt: number
  lastActiveAt: number
  working: boolean           // 正在工作中（用户发送消息后、响应返回前）
  model?: string
  pending: boolean           // 需要用户关注
  isResume: boolean          // 是否为恢复历史会话（创建时已有 sessionId）
  takenOver?: boolean        // 终端已被远程（手机）接管，显示遮罩
  /** 多实例远程：会话所属 CC-Box 实例（浏览器远程模式，桌面模式缺失） */
  instanceId?: string
  instanceName?: string
}

/**
 * 历史会话（从 Claude 原生数据读取，未被 Tab 占用）
 */
export interface HistorySession {
  sessionId: string
  name: string
  projectPath: string
  lastActiveAt: number
}

// ==================== Store ====================

export const useSessionStore = defineStore('session', () => {
  // ---- State ----
  const tabs = reactive(new Map<string, TerminalTab>())
  const activeTabId = ref<string | null>(null)
  /** 项目级历史会话缓存 Map */
  const historyCacheMap = reactive(new Map<string, HistorySession[]>())
  /** 当前展示历史会话的项目路径 */
  const currentHistoryProject = ref<string>('')
  /** 正在加载的项目路径（去重用） */
  const inflightLoadProject = ref<string | null>(null)
  const searchQuery = ref<string>('')
  /** 多实例远程：下一次 createTab 的实例归属（新建/恢复前的选择结果；
   *  createTab 消费后清空——桌面模式恒 null 无影响） */
  const pendingInstanceId = ref<string | null>(null)
  /** 多实例远程：Gateway 下发的全量实例名单（含零运行会话的实例；
   *  单实例/桌面为空——knownInstances 回退按 Tab 推导） */
  const remoteInstances = ref<Array<{ id: string; name: string }>>([])
  /** 是否已收到过 Gateway 的实例名单（含空名单）：用于零窗口 UI 判定，
   *  避免首帧前把“尚未加载”误判为“没有窗口” */
  const remoteInstancesKnown = ref<boolean>(false)
  const isLoading = ref<boolean>(false)
  const isLoadingMore = ref<boolean>(false)
  const messageSearchResults = ref<SessionSearchResult[]>([])
  let messageSearchTimer: ReturnType<typeof setTimeout> | null = null

  // ---- Computed ----

  const activeTab = computed<TerminalTab | null>(() => {
    if (!activeTabId.value) return null
    return tabs.get(activeTabId.value) ?? null
  })

  /** 当前项目的 Tab 列表 */
  const projectTabs = computed<TerminalTab[]>(() => {
    // 不在这里过滤，因为 projectPath 需要外部传入
    return [...tabs.values()]
  })

  /** 已知实例集合（新建/恢复时供实例选择）：Gateway 全量名单优先
   *  （含零运行会话实例），再补 Tab 上出现但名单未覆盖的实例（旧桌面/
   *  单实例场景名单为空，完全回退 Tab 推导） */
  const knownInstances = computed<Array<{ id: string; name: string }>>(() => {
    const out = new Map<string, string>()
    for (const i of remoteInstances.value) {
      if (i.id) out.set(i.id, i.name)
    }
    // 已知名单为空 = 宿主没有任何窗口：不能再用残留 Tab 推导实例，
    // 否则选择器会把已下线窗口当成可路由目标。
    const rosterEmpty = remoteInstancesKnown.value && remoteInstances.value.length === 0
    if (!rosterEmpty) {
      for (const t of tabs.values()) {
        if (t.instanceId && !out.has(t.instanceId)) {
          out.set(t.instanceId, t.instanceName || `窗口 ${out.size + 1}`)
        }
      }
    }
    return [...out].map(([id, name]) => ({ id, name }))
  })

  /** 应用 Gateway 下发的实例名单（instances-changed 事件载荷） */
  function setRemoteInstances(list: Array<{ id: string; name: string }>) {
    remoteInstances.value = list
    remoteInstancesKnown.value = true
  }

  /** 新连接/切换设备：清空名单并回到“尚未加载” */
  function resetRemoteInstances() {
    remoteInstances.value = []
    remoteInstancesKnown.value = false
  }

  /** 运行中的 Tab ID 列表 */
  const runningTabIds = computed<string[]>(() => {
    return [...tabs.values()]
      .filter(t => t.status === 'running')
      .map(t => t.tabId)
  })

  /** 已被 Tab 占用的 sessionId 集合（含演变历史——resume fork 出的新旧 id 都算占用） */
  const claimedSessionIds = computed<Set<string>>(() => {
    const ids = new Set<string>()
    for (const tab of tabs.values()) {
      for (const sid of tab.sessionIds) ids.add(sid)
    }
    return ids
  })

  /** 未被 Tab 占用的历史会话（去重 + 过滤） */
  const historySessions = computed<HistorySession[]>(() => {
    const cached = historyCacheMap.get(currentHistoryProject.value) ?? []
    const claimed = claimedSessionIds.value
    const seen = new Set<string>()
    return cached.filter(s => {
      if (claimed.has(s.sessionId) || seen.has(s.sessionId)) return false
      seen.add(s.sessionId)
      return true
    })
  })

  // ---- Tab 生命周期 ----

  /** 实例展示名：Gateway 名单优先，其次从已有 Tab 推导 */
  function instanceNameOf(instanceId: string): string | undefined {
    return remoteInstances.value.find(i => i.id === instanceId)?.name
      ?? [...tabs.values()].find(t => t.instanceId === instanceId)?.instanceName
  }

  /**
   * 创建新 Tab
   * @returns tabId
   */
  function createTab(projectPath: string, opts?: {
    sessionId?: string
    name?: string
    /** 显式归属（采纳远端会话）；缺省消费 pendingInstanceId（用户选择） */
    instanceId?: string
    instanceName?: string
  }): string {
    const tabId = uuid()
    const now = Date.now()
    const explicitInst = opts?.instanceId
    const instId = explicitInst ?? pendingInstanceId.value ?? undefined

    tabs.set(tabId, {
      tabId,
      projectPath,
      ptyId: null,
      sessionId: opts?.sessionId ?? null,
      sessionIds: opts?.sessionId ? [opts.sessionId] : [],
      name: opts?.name ?? (opts?.sessionId ? opts.sessionId.slice(0, 8) : 'New Session'),
      status: 'stopped',
      createdAt: now,
      lastActiveAt: now,
      working: false,
      pending: false,
      isResume: !!opts?.sessionId,
      // 多实例：显式归属优先，否则消费入口处的实例选择（spawn 时随 session.create 路由）
      instanceId: instId,
      instanceName: opts?.instanceName
        ?? (instId ? instanceNameOf(instId) : undefined),
    })
    // 显式归属（采纳远端会话）不消费用户选择；其余路径建 Tab 即用掉选择
    if (explicitInst === undefined) pendingInstanceId.value = null

    return tabId
  }

  /**
   * 为 Tab 启动 PTY
   * 调用者需要在拿到 ptyId 后传入
   */
  function setTabPty(tabId: string, ptyId: string) {
    const tab = tabs.get(tabId)
    if (!tab) return
    tab.ptyId = ptyId
    tab.status = 'running'
    tab.lastActiveAt = Date.now()
  }

  /**
   * 浏览器远程模式：采纳服务端运行中的会话列表（刷新/新设备连接后的恢复入口）。
   * 已有同 ptyId 的 Tab 仅更新 working/pending；新会话建 Tab 绑定 ptyId
   * （不 attach——用户点击 Tab 时才接管控制权，独占语义保持）。
   * Tab 名优先取历史会话名（sessionId 匹配）；历史未加载时回退项目名，
   * 加载完成后经 backfillTabNamesFromHistory 修正
   */
  function adoptRemoteSessions(list: Array<{ id: string; type: string; cwd: string; working: boolean; pending: boolean; sessionId?: string; instanceId?: string; instanceName?: string }>) {
    for (const s of list) {
      const existing = getTabByPtyId(s.id)
      if (existing) {
        existing.working = s.working
        existing.pending = s.pending
        existing.lastActiveAt = Date.now()
        if (s.sessionId && existing.sessionId !== s.sessionId) {
          setTabSessionId(existing.tabId, s.sessionId)
        }
        existing.instanceId = s.instanceId
        existing.instanceName = s.instanceName
        continue
      }
      const histName = s.sessionId
        ? historyCacheMap.get(s.cwd)?.find(h => h.sessionId === s.sessionId)?.name?.trim()
        : ''
      const parts = s.cwd.replace(/\\/g, '/').split('/')
      const name = histName || parts[parts.length - 1] || s.cwd
      // 显式归属：不消费 pendingInstanceId（用户选择只属于其下一步新建/恢复）
      const tabId = createTab(s.cwd, {
        name,
        sessionId: s.sessionId,
        instanceId: s.instanceId,
        instanceName: s.instanceName,
      })
      setTabPty(tabId, s.id)
      const tab = tabs.get(tabId)
      if (tab) {
        tab.working = s.working
        tab.pending = s.pending
      }
    }
  }

  /**
   * 远程 attach 被拒的单一处理（按 reason 分支）：
   * - 控制权被其他端持有 → takenOver 遮罩 + 接管入口（显式接管后重新 attach）
   * - 会话不存在（宿主窗口已关/路由丢失）→ 按退出处理置 stopped——
   *   若误显示「被接管」，用户点接管后会永久循环被拒
   */
  function handleAttachDenied(ptyId: string, reason: string) {
    const tab = getTabByPtyId(ptyId)
    if (!tab) return
    if (/session not found/i.test(reason)) {
      if (tab.status === 'running') tab.status = 'stopped'
      return
    }
    tab.takenOver = true
  }

  /**
   * 远程会话被剥离（宿主窗口断开等，服务端 session.detached）：
   * Tab 置 stopped——不处理的话 Tab 永远 running、终端冻结无输出
   */
  function handleRemoteDetached(ptyId: string) {
    const tab = getTabByPtyId(ptyId)
    if (tab && tab.status === 'running') tab.status = 'stopped'
  }

  /**
   * 远程采纳 Tab 的会话名回填：采纳时历史未加载只能用项目名，历史加载
   * 完成后按 sessionId 修正为真实会话名。名字已不是项目名的 Tab 不动
   * （可能已被用户重命名或 hook 自动命名覆盖）
   */
  function backfillTabNamesFromHistory() {
    for (const tab of tabs.values()) {
      if (!tab.sessionId) continue
      const projName = tab.projectPath.replace(/\\/g, '/').split('/').pop() || tab.projectPath
      if (tab.name !== projName) continue
      const histName = historyCacheMap.get(tab.projectPath)
        ?.find(h => h.sessionId === tab.sessionId)?.name?.trim()
      if (histName) tab.name = histName
    }
  }

  /**
   * 会话结束（用户关闭/自然退出）时把 Tab 已知信息就地并入首页「近期
   * 会话」缓存（appStore.mergeRecentSession 按 sessionId 去重）：零请求
   * 即时可见，且不重置首页项目列表分页。sessionId 为空（从未启动/CLI
   * 未上报）的 Tab 无历史记录可指，跳过
   */
  function mergeTabIntoRecentSessions(tab: TerminalTab) {
    if (!tab.sessionId) return
    useAppStore().mergeRecentSession({
      sessionId: tab.sessionId,
      name: tab.name,
      projectPath: tab.projectPath,
      lastActiveAt: Date.now(),
    })
  }

  /**
   * 处理 PTY 退出
   * 只更新状态，不删除 Tab
   */
  function handlePtyExit(ptyId: string) {
    for (const tab of tabs.values()) {
      if (tab.ptyId === ptyId) {
        tab.ptyId = null
        tab.status = 'stopped'
        tab.working = false
        tab.lastActiveAt = Date.now()
        // 会话已结束：并入近期会话缓存，首页立即可见（重启该 Tab 时运行中
        // 条目优先、缓存同 id 被排重，无副作用）
        mergeTabIntoRecentSessions(tab)
        break
      }
    }
  }

  /**
   * 关闭 Tab（用户主动操作）
   * 先删除 tab 让 UI 立即响应，再异步 kill PTY
   */
  async function closeTab(tabId: string) {
    const tab = tabs.get(tabId)
    if (!tab) return

    const projectPath = tab.projectPath
    const ptyId = tab.ptyId

    // 先删除 tab，UI 立即更新；Tab 携带的会话信息并入近期会话缓存
    tabs.delete(tabId)
    mergeTabIntoRecentSessions(tab)

    if (activeTabId.value === tabId) {
      // 关闭当前活跃标签后，聚焦到同项目的相邻标签
      const remaining = getProjectTabs(projectPath)
      if (remaining.length > 0) {
        activeTabId.value = remaining[0].tabId
      } else {
        activeTabId.value = null
      }
    }

    // 异步 kill PTY（不阻塞 UI）
    if (ptyId) {
      ptyKill(ptyId).catch(() => {})
    }
  }

  /**
   * 关闭指定项目的所有 Tab
   */
  async function closeAllTabs(projectPath: string) {
    const projectTabList = getProjectTabs(projectPath)
    const ptyIds = projectTabList
      .map(t => t.ptyId)
      .filter((id): id is string => !!id)

    tabs.forEach((tab, tabId) => {
      if (tab.projectPath === projectPath) {
        tabs.delete(tabId)
        mergeTabIntoRecentSessions(tab)
      }
    })

    if (activeTabId.value && !tabs.has(activeTabId.value)) {
      activeTabId.value = null
    }

    for (const ptyId of ptyIds) {
      ptyKill(ptyId).catch(() => {})
    }
  }

  /**
   * 关闭除指定 Tab 外的所有同项目 Tab
   */
  async function closeOtherTabs(keepTabId: string) {
    const keepTab = tabs.get(keepTabId)
    if (!keepTab) return

    const projectPath = keepTab.projectPath
    const projectTabList = getProjectTabs(projectPath)
    const ptyIds: string[] = []

    for (const tab of projectTabList) {
      if (tab.tabId !== keepTabId) {
        if (tab.ptyId) ptyIds.push(tab.ptyId)
        tabs.delete(tab.tabId)
        mergeTabIntoRecentSessions(tab)
      }
    }

    if (activeTabId.value && !tabs.has(activeTabId.value)) {
      activeTabId.value = keepTabId
    }

    for (const ptyId of ptyIds) {
      ptyKill(ptyId).catch(() => {})
    }
  }

  /**
   * 更新 Tab 名称
   */
  function updateTabName(tabId: string, name: string) {
    const tab = tabs.get(tabId)
    if (tab) {
      tab.name = name
    }
  }

  /**
   * 重命名广播 key：running 用 ptyId；stopped Tab 的 ptyId 已被 handlePtyExit
   * 置空，用 sessionId（接收端按演变历史匹配）。两者都没有（从未启动的
   * New Session）返回 null——不广播，双端各自本地命名。
   */
  function renameKeyForTab(tabId: string): string | null {
    const tab = tabs.get(tabId)
    if (!tab) return null
    return tab.ptyId ?? tab.sessionId ?? null
  }

  /**
   * 应用远端重命名广播：key 为 ptyId（running）或 sessionId（stopped，
   * 按演变历史匹配 resume fork 的新旧 id）。命中返回 true。
   */
  function applyRemoteRename(key: string, name: string): boolean {
    const tab = getTabByPtyId(key) ?? getTabBySessionId(key)
    if (!tab) return false
    tab.name = name
    return true
  }

  /**
   * 更新 Tab 的 sessionId（匹配后赋值；新值记入演变历史，旧 id 保留认领）
   */
  function setTabSessionId(tabId: string, sessionId: string, name?: string) {
    const tab = tabs.get(tabId)
    if (!tab) return
    tab.sessionId = sessionId
    if (!tab.sessionIds.includes(sessionId)) tab.sessionIds.push(sessionId)
    if (name) tab.name = name
    tab.lastActiveAt = Date.now()
  }

  /**
   * 恢复会话入口的统一决策（复用优先）：按 sessionId 演变历史找同项目已有 Tab
   * ——命中即复用（running 切换显示 / stopped 由 startTab 走 --resume 重启，
   * 原名保留），未命中才新建。所有恢复路径（App/TerminalView 历史、
   * remote-session-created）必须经此方法，防同一对话二次 spawn/重复 Tab。
   */
  function resumeSessionTab(projectPath: string, sessionId: string, name?: string): { tabId: string; reused: boolean } {
    const existing = getTabBySessionId(sessionId)
    if (existing && existing.projectPath === projectPath) {
      existing.lastActiveAt = Date.now()
      return { tabId: existing.tabId, reused: true }
    }
    return { tabId: createTab(projectPath, { sessionId, name }), reused: false }
  }

  // ---- 活跃 Tab 管理 ----

  function setActiveTab(tabId: string | null) {
    if (tabId) {
      const tab = tabs.get(tabId)
      if (tab) tab.pending = false
    }
    activeTabId.value = tabId
  }

  function getRunningTabForProject(projectPath: string): TerminalTab | null {
    for (const tab of tabs.values()) {
      if (tab.projectPath === projectPath && tab.status === 'running') {
        return tab
      }
    }
    return null
  }

  function getTabByPtyId(ptyId: string): TerminalTab | null {
    for (const tab of tabs.values()) {
      if (tab.ptyId === ptyId) return tab
    }
    return null
  }

  function getTabBySessionId(sessionId: string): TerminalTab | null {
    for (const tab of tabs.values()) {
      // 匹配演变历史：resume 旧 id 与 fork 新 id 指向同一对话
      if (tab.sessionIds.includes(sessionId)) return tab
    }
    return null
  }

  // ---- Session ID 直接分配（由 hook 事件驱动） ----

  function assignSessionIdByPtyId(ptyId: string, sessionId: string, model?: string) {
    for (const tab of tabs.values()) {
      if (tab.ptyId === ptyId) {
        setTabSessionId(tab.tabId, sessionId)
        if (model) tab.model = model
        tab.lastActiveAt = Date.now()
        return true
      }
    }
    return false
  }

  const hasPendingTabs = computed(() => {
    for (const tab of tabs.values()) {
      if (tab.pending) return true
    }
    return false
  })

  // ---- 历史会话 ----

  const BATCH_SIZE = 20

  /**
   * 加载历史会话（项目级缓存 + 去重 + 不提前清空）
   */
  async function loadHistorySessions(projectPath: string, force = false) {
    // 切换当前展示项目（即使还在加载中也立即切换，让 computed 指向正确项目）
    currentHistoryProject.value = projectPath

    // 缓存命中且非强制刷新，直接返回
    if (!force && historyCacheMap.has(projectPath)) return
    // 去重：已在加载同一项目
    if (inflightLoadProject.value === projectPath) return

    inflightLoadProject.value = projectPath
    isLoading.value = true
    isLoadingMore.value = false

    try {
      // 请求 BATCH_SIZE + 1 条来判断是否有更多
      const firstBatch = await getSessions(projectPath, BATCH_SIZE + 1, 0)
      const hasMore = firstBatch.length > BATCH_SIZE
      const firstPage = hasMore ? firstBatch.slice(0, BATCH_SIZE) : firstBatch

      const mapped = firstPage
        .map(s => ({
          sessionId: s.sessionId,
          name: s.name,
          projectPath: s.projectPath,
          lastActiveAt: s.lastActiveAt,
        }))
        .sort((a, b) => b.lastActiveAt - a.lastActiveAt)

      // 数据就绪后才写入缓存（不提前清空）
      historyCacheMap.set(projectPath, mapped)
      isLoading.value = false

      if (hasMore) {
        isLoadingMore.value = true
        let offset = BATCH_SIZE
        let allSessions = [...mapped]
        while (true) {
          const batch = await getSessions(projectPath, BATCH_SIZE, offset)
          if (batch.length === 0) break
          const more = batch
            .map(s => ({
              sessionId: s.sessionId,
              name: s.name,
              projectPath: s.projectPath,
              lastActiveAt: s.lastActiveAt,
            }))
          allSessions = [...allSessions, ...more]
            .sort((a, b) => b.lastActiveAt - a.lastActiveAt)
          offset += batch.length
          if (batch.length < BATCH_SIZE) break
        }
        historyCacheMap.set(projectPath, allSessions)
        isLoadingMore.value = false
      }
    } catch (err) {
      console.error('[SessionStore] loadHistorySessions failed:', err)
      isLoading.value = false
      isLoadingMore.value = false
    } finally {
      inflightLoadProject.value = null
    }
  }

  /** 清除指定项目或全部历史缓存 */
  function invalidateHistoryCache(projectPath?: string) {
    if (projectPath) {
      historyCacheMap.delete(projectPath)
    } else {
      historyCacheMap.clear()
    }
  }

  // ---- 搜索 ----

  function setSearchQuery(query: string) {
    searchQuery.value = query
  }

  function debouncedSearchMessages(projectPath: string, query: string) {
    if (messageSearchTimer) clearTimeout(messageSearchTimer)
    if (!query || query.length < 2) {
      messageSearchResults.value = []
      return
    }
    messageSearchTimer = setTimeout(async () => {
      try {
        messageSearchResults.value = await searchSessionMessages(projectPath, query, 10)
      } catch (err) {
        console.error('[SessionStore] searchMessages failed:', err)
        messageSearchResults.value = []
      }
    }, 400)
  }

  // ---- 清理 ----

  async function cleanupAll() {
    const ids = [...tabs.values()]
      .filter(t => t.ptyId)
      .map(t => t.ptyId!)

    for (const id of ids) {
      try { await ptyKill(id) } catch {}
    }
    tabs.clear()
    historyCacheMap.clear()
    currentHistoryProject.value = ''
    inflightLoadProject.value = null
  }

  function getProjectTabs(projectPath: string): TerminalTab[] {
    return [...tabs.values()]
      .filter(t => t.projectPath === projectPath)
      .sort((a, b) => b.lastActiveAt - a.lastActiveAt)
  }

  return {
    // State
    tabs,
    activeTabId,
    historySessions,
    searchQuery,
    isLoading,
    isLoadingMore,

    // Computed
    activeTab,
    projectTabs,
    runningTabIds,
    claimedSessionIds,
    hasPendingTabs,

    // Tab lifecycle
    createTab,
    setTabPty,
    adoptRemoteSessions,
    handleAttachDenied,
    handleRemoteDetached,
    backfillTabNamesFromHistory,
    handlePtyExit,
    closeTab,
    closeAllTabs,
    closeOtherTabs,
    updateTabName,
    renameKeyForTab,
    applyRemoteRename,
    setTabSessionId,
    resumeSessionTab,
    pendingInstanceId,
    remoteInstances,
    remoteInstancesKnown,
    setRemoteInstances,
    resetRemoteInstances,
    knownInstances,

    // Active tab
    setActiveTab,
    getRunningTabForProject,
    getTabByPtyId,
    getTabBySessionId,
    getProjectTabs,

    // Session ID assignment
    assignSessionIdByPtyId,

    // History
    loadHistorySessions,
    invalidateHistoryCache,

    // Search
    setSearchQuery,
    debouncedSearchMessages,
    messageSearchResults,

    // Cleanup
    cleanupAll,
  }
})
