import { defineStore } from 'pinia'
import { ref, computed, reactive } from 'vue'
import { uuid } from '@/utils/uuid'
import {
  getSessions,
  ptyKill,
  searchSessionMessages
} from '@/api/tauri'
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
  name: string
  status: 'starting' | 'running' | 'stopped'
  createdAt: number
  lastActiveAt: number
  working: boolean           // 正在工作中（用户发送消息后、响应返回前）
  model?: string
  pending: boolean           // 需要用户关注
  isResume: boolean          // 是否为恢复历史会话（创建时已有 sessionId）
  takenOver?: boolean        // 终端已被远程（手机）接管，显示遮罩
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

  /** 运行中的 Tab ID 列表 */
  const runningTabIds = computed<string[]>(() => {
    return [...tabs.values()]
      .filter(t => t.status === 'running')
      .map(t => t.tabId)
  })

  /** 已被 Tab 占用的 sessionId 集合 */
  const claimedSessionIds = computed<Set<string>>(() => {
    const ids = new Set<string>()
    for (const tab of tabs.values()) {
      if (tab.sessionId) ids.add(tab.sessionId)
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

  /**
   * 创建新 Tab
   * @returns tabId
   */
  function createTab(projectPath: string, opts?: {
    sessionId?: string
    name?: string
  }): string {
    const tabId = uuid()
    const now = Date.now()

    tabs.set(tabId, {
      tabId,
      projectPath,
      ptyId: null,
      sessionId: opts?.sessionId ?? null,
      name: opts?.name ?? (opts?.sessionId ? opts.sessionId.slice(0, 8) : 'New Session'),
      status: 'stopped',
      createdAt: now,
      lastActiveAt: now,
      working: false,
      pending: false,
      isResume: !!opts?.sessionId,
    })

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
   * （不 attach——用户点击 Tab 时才接管控制权，独占语义保持）
   */
  function adoptRemoteSessions(list: Array<{ id: string; type: string; cwd: string; working: boolean; pending: boolean }>) {
    for (const s of list) {
      const existing = getTabByPtyId(s.id)
      if (existing) {
        existing.working = s.working
        existing.pending = s.pending
        existing.lastActiveAt = Date.now()
        continue
      }
      const parts = s.cwd.replace(/\\/g, '/').split('/')
      const name = parts[parts.length - 1] || s.cwd
      const tabId = createTab(s.cwd, { name })
      setTabPty(tabId, s.id)
      const tab = tabs.get(tabId)
      if (tab) {
        tab.working = s.working
        tab.pending = s.pending
      }
    }
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

    // 先删除 tab，UI 立即更新（历史列表通过 computed 自动显示释放的 sessionId）
    tabs.delete(tabId)

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
   * 更新 Tab 的 sessionId（匹配后赋值）
   */
  function setTabSessionId(tabId: string, sessionId: string, name?: string) {
    const tab = tabs.get(tabId)
    if (!tab) return
    tab.sessionId = sessionId
    if (name) tab.name = name
    tab.lastActiveAt = Date.now()
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
      if (tab.sessionId === sessionId) return tab
    }
    return null
  }

  // ---- Session ID 直接分配（由 hook 事件驱动） ----

  function assignSessionIdByPtyId(ptyId: string, sessionId: string, model?: string) {
    for (const tab of tabs.values()) {
      if (tab.ptyId === ptyId) {
        tab.sessionId = sessionId
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
    handlePtyExit,
    closeTab,
    closeAllTabs,
    closeOtherTabs,
    updateTabName,
    setTabSessionId,

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
