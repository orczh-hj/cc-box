import { describe, it, expect, beforeEach, vi } from 'vitest'
import { setActivePinia, createPinia } from 'pinia'
import { useSessionStore } from '@/stores/session'
import { useAppStore } from '@/stores/app'
import { getSessions } from '@/api/tauri'
import { randomUUID } from 'crypto'

// crypto.randomUUID polyfill for jsdom
if (typeof globalThis.crypto === 'undefined' || !globalThis.crypto.randomUUID) {
  Object.defineProperty(globalThis, 'crypto', {
    value: {
      ...globalThis.crypto,
      randomUUID: () => randomUUID(),
    },
    writable: true,
    configurable: true,
  })
}

// Mock @/api/tauri
// （session store → app store 链上的全部导出都需提供，缺导出会在 import 时抛错）
vi.mock('@/api/tauri', () => ({
  // session.ts 使用
  ptyKill: vi.fn().mockResolvedValue(true),
  getSessionCount: vi.fn().mockResolvedValue(0),
  getSessions: vi.fn().mockResolvedValue([]),
  searchSessionMessages: vi.fn().mockResolvedValue([]),
  // app.ts 使用（经 useAppStore 间接引入）
  getAppConfig: vi.fn().mockResolvedValue({}),
  updateAppConfig: vi.fn().mockResolvedValue(undefined),
  saveLastProject: vi.fn().mockResolvedValue(undefined),
  saveDefaultClaudeOptions: vi.fn().mockResolvedValue(undefined),
  getHomeData: vi.fn().mockResolvedValue({ projects: [], recentSessions: [], hasMore: false }),
  getProjects: vi.fn().mockResolvedValue([]),
  getCheckResults: vi.fn().mockResolvedValue([]),
  runChecks: vi.fn().mockResolvedValue([]),
  getSystemLocale: vi.fn().mockResolvedValue('en'),
}))

describe('session store', () => {
  beforeEach(() => {
    setActivePinia(createPinia())
  })

  // ==================== createTab ====================

  describe('createTab', () => {
    // 创建 tab 包含正确的 projectPath
    it('CreateTab_ProjectPath_001', () => {
      const store = useSessionStore()
      const tabId = store.createTab('/home/user/project-a')
      const tab = store.tabs.get(tabId)!
      expect(tab.projectPath).toBe('/home/user/project-a')
    })

    // 设置 sessionId="abc12345def" 时 name 取前 8 字符 "abc12345"
    it('CreateTab_SessionIdName_001', () => {
      const store = useSessionStore()
      const tabId = store.createTab('/project', { sessionId: 'abc12345def' })
      const tab = store.tabs.get(tabId)!
      expect(tab.name).toBe('abc12345')
    })

    // 不设 sessionId 时 name 为 "New Session"
    it('CreateTab_DefaultName_001', () => {
      const store = useSessionStore()
      const tabId = store.createTab('/project')
      const tab = store.tabs.get(tabId)!
      expect(tab.name).toBe('New Session')
    })

    // 初始 status 为 "stopped"
    it('CreateTab_InitialState_001', () => {
      const store = useSessionStore()
      const tabId = store.createTab('/project')
      const tab = store.tabs.get(tabId)!
      expect(tab.status).toBe('stopped')
      expect(tab.ptyId).toBeNull()
      expect(tab.sessionId).toBeNull()
      expect(tab.working).toBe(false)
      expect(tab.pending).toBe(false)
    })

    // 连续创建两个 tab 的 tabId 不同
    it('CreateTab_UniqueId_001', () => {
      const store = useSessionStore()
      const id1 = store.createTab('/project')
      const id2 = store.createTab('/project')
      expect(id1).not.toBe(id2)
    })
  })

  // ==================== closeTab ====================

  describe('closeTab', () => {
    // 关闭 tab 后从 store 中移除
    it('CloseTab_Remove_001', async () => {
      const store = useSessionStore()
      const tabId = store.createTab('/project')
      store.setActiveTab(tabId)
      await store.closeTab(tabId)
      expect(store.tabs.has(tabId)).toBe(false)
    })

    // 关闭活跃 tab 时 activeTabId 切换到同项目其他 tab
    it('CloseTab_SwitchActive_001', async () => {
      const store = useSessionStore()
      const id1 = store.createTab('/project-x')
      const id2 = store.createTab('/project-x')
      store.setActiveTab(id2)
      await store.closeTab(id2)
      expect(store.activeTabId).toBe(id1)
    })

    // 关闭项目中最后一个 tab 时 activeTabId 变为 null
    it('CloseTab_LastTab_001', async () => {
      const store = useSessionStore()
      const tabId = store.createTab('/project-y')
      store.setActiveTab(tabId)
      await store.closeTab(tabId)
      expect(store.activeTabId).toBeNull()
    })

    // 关闭有 PTY 的 tab 时，tab 立即删除，PTY 异步 kill 不阻塞
    it('CloseTab_ImmediateDelete_001', async () => {
      const { ptyKill } = await import('@/api/tauri')
      const mockKill = ptyKill as ReturnType<typeof vi.fn>

      const store = useSessionStore()
      const tabId = store.createTab('/project')
      store.setTabPty(tabId, 'pty-immediate')

      // closeTab 不再 await ptyKill，tab 应立即删除
      await store.closeTab(tabId)
      expect(store.tabs.has(tabId)).toBe(false)
      expect(mockKill).toHaveBeenCalledWith('pty-immediate')
    })

    // 关闭 tab 后 claimedSessionIds 释放，历史会话自动显示
    it('CloseTab_ReleaseClaim_001', async () => {
      const store = useSessionStore()
      const tabId = store.createTab('/project', { sessionId: 'sess-release' })

      expect(store.claimedSessionIds.has('sess-release')).toBe(true)
      await store.closeTab(tabId)
      expect(store.claimedSessionIds.has('sess-release')).toBe(false)
    })

    // 关闭带 sessionId 的 tab：会话信息并入首页近期会话缓存（零请求即时可见）
    it('CloseTab_MergesIntoRecent_001', async () => {
      const appStore = useAppStore()
      const store = useSessionStore()
      const tabId = store.createTab('/project', { sessionId: 'sess-recent', name: '我的会话' })

      await store.closeTab(tabId)
      const merged = appStore.cachedRecentSessions.find(s => s.sessionId === 'sess-recent')
      expect(merged).toBeDefined()
      expect(merged?.name).toBe('我的会话')
      expect(merged?.projectPath).toBe('/project')
    })

    // 关闭无 sessionId 的 tab（从未启动）：无历史记录可指，不并入缓存
    it('CloseTab_NoSessionId_NoMerge_002', async () => {
      const appStore = useAppStore()
      const store = useSessionStore()
      const tabId = store.createTab('/project')

      await store.closeTab(tabId)
      expect(appStore.cachedRecentSessions).toHaveLength(0)
    })

    // 重复关闭同 sessionId（closeTab 后 resume 再 closeTab）：按 id 去重，不产生重复条目
    it('CloseTab_MergeDedup_003', async () => {
      const appStore = useAppStore()
      const store = useSessionStore()
      const tabId = store.createTab('/project', { sessionId: 'sess-dup' })

      await store.closeTab(tabId)
      const tabId2 = store.createTab('/project', { sessionId: 'sess-dup' })
      await store.closeTab(tabId2)
      expect(appStore.cachedRecentSessions.filter(s => s.sessionId === 'sess-dup')).toHaveLength(1)
    })
  })

  // ==================== handlePtyExit ====================

  describe('handlePtyExit', () => {
    // PTY 退出后 tab 状态设为 stopped
    it('PtyExit_Status_001', () => {
      const store = useSessionStore()
      const tabId = store.createTab('/project')
      store.setTabPty(tabId, 'pty-001')
      store.handlePtyExit('pty-001')
      const tab = store.tabs.get(tabId)!
      expect(tab.status).toBe('stopped')
    })

    // PTY 退出后 ptyId 清空
    it('PtyExit_ClearPtyId_001', () => {
      const store = useSessionStore()
      const tabId = store.createTab('/project')
      store.setTabPty(tabId, 'pty-002')
      store.handlePtyExit('pty-002')
      const tab = store.tabs.get(tabId)!
      expect(tab.ptyId).toBeNull()
    })

    // PTY 退出后 working 设为 false
    it('PtyExit_ClearWorking_001', () => {
      const store = useSessionStore()
      const tabId = store.createTab('/project')
      store.setTabPty(tabId, 'pty-003')
      const tab = store.tabs.get(tabId)!
      tab.working = true
      store.handlePtyExit('pty-003')
      expect(tab.working).toBe(false)
    })
  })

  // ==================== claimedSessionIds ====================

  describe('claimedSessionIds', () => {
    // 两个 tab 分别设置 sessionId 时集合包含两个值
    it('ClaimedIds_Include_001', () => {
      const store = useSessionStore()
      const id1 = store.createTab('/project', { sessionId: 'sess-alpha' })
      const id2 = store.createTab('/project', { sessionId: 'sess-beta' })
      store.setTabSessionId(id1, 'sess-alpha')
      store.setTabSessionId(id2, 'sess-beta')
      const claimed = store.claimedSessionIds
      expect(claimed.has('sess-alpha')).toBe(true)
      expect(claimed.has('sess-beta')).toBe(true)
      expect(claimed.size).toBe(2)
    })

    // sessionId 为 null 的 tab 不出现在集合中
    it('ClaimedIds_ExcludeNull_001', () => {
      const store = useSessionStore()
      store.createTab('/project') // no sessionId
      const claimed = store.claimedSessionIds
      expect(claimed.size).toBe(0)
    })
  })

  // ==================== sessionId 演变历史（CLI fork 场景） ====================

  describe('sessionId 演变历史', () => {
    // 复现 bug：resume 传入旧 sessionId X 被 CLI fork 成新 sessionId Y 后，
    // claimedSessionIds 只认当前值 Y，旧 id X 变幽灵历史条目重复显示
    it('SessionIdHistory_ClaimedKeepsOldId_001', () => {
      const store = useSessionStore()
      const tabId = store.createTab('/project', { sessionId: 'sess-old' })
      store.setTabPty(tabId, 'pty-fork')
      store.assignSessionIdByPtyId('pty-fork', 'sess-new')
      expect(store.claimedSessionIds.has('sess-new')).toBe(true)
      expect(store.claimedSessionIds.has('sess-old')).toBe(true)
    })

    // 覆写后按旧 id 仍能找到运行中 tab（恢复入口前置匹配，防二次 spawn）
    it('SessionIdHistory_FindByOldId_001', () => {
      const store = useSessionStore()
      const tabId = store.createTab('/project', { sessionId: 'sess-old' })
      store.setTabPty(tabId, 'pty-fork')
      store.assignSessionIdByPtyId('pty-fork', 'sess-new')
      const tab = store.getTabBySessionId('sess-old')
      expect(tab?.tabId).toBe(tabId)
      expect(tab?.status).toBe('running')
    })

    // setTabSessionId 更新同样记录演变历史
    it('SessionIdHistory_SetTabSessionId_001', () => {
      const store = useSessionStore()
      const tabId = store.createTab('/project', { sessionId: 'sess-a' })
      store.setTabSessionId(tabId, 'sess-b')
      expect(store.tabs.get(tabId)!.sessionIds).toEqual(['sess-a', 'sess-b'])
    })

    // 重复赋相同值不产生重复记录
    it('SessionIdHistory_NoDuplicate_001', () => {
      const store = useSessionStore()
      const tabId = store.createTab('/project', { sessionId: 'sess-a' })
      store.setTabSessionId(tabId, 'sess-a')
      expect(store.tabs.get(tabId)!.sessionIds).toEqual(['sess-a'])
    })

    // createTab 初始化 sessionIds 数组
    it('SessionIdHistory_InitOnCreate_001', () => {
      const store = useSessionStore()
      const tabId = store.createTab('/project', { sessionId: 'sess-init' })
      expect(store.tabs.get(tabId)!.sessionIds).toEqual(['sess-init'])
      const tabId2 = store.createTab('/project')
      expect(store.tabs.get(tabId2)!.sessionIds).toEqual([])
    })
  })

  // ==================== resumeSessionTab（恢复入口统一决策：复用优先） ====================

  describe('resumeSessionTab', () => {
    // 复现 bug：恢复已有 Tab 的会话（running 或 stopped）时无条件新建 Tab，
    // 造成同一会话出现两份（一份原名 + 一份 sessionId 编码名）
    it('ResumeSessionTab_ReuseRunning_001', () => {
      const store = useSessionStore()
      const tabId = store.createTab('/project', { sessionId: 'sess-r', name: '原名' })
      store.setTabPty(tabId, 'pty-r')
      const r = store.resumeSessionTab('/project', 'sess-r')
      expect(r.reused).toBe(true)
      expect(r.tabId).toBe(tabId)
      expect(store.tabs.size).toBe(1)
      expect(store.tabs.get(tabId)!.name).toBe('原名')
    })

    // stopped Tab 同样复用（由 startTab 走 --resume 重启），名字保留
    it('ResumeSessionTab_ReuseStopped_001', () => {
      const store = useSessionStore()
      const tabId = store.createTab('/project', { sessionId: 'sess-s', name: '停用会话' })
      store.setTabPty(tabId, 'pty-s')
      store.handlePtyExit('pty-s')
      const r = store.resumeSessionTab('/project', 'sess-s')
      expect(r.reused).toBe(true)
      expect(r.tabId).toBe(tabId)
      expect(store.tabs.get(tabId)!.status).toBe('stopped')
    })

    // 覆写后的历史 id（CLI fork 前）同样命中复用
    it('ResumeSessionTab_MatchForkedHistoryId_001', () => {
      const store = useSessionStore()
      const tabId = store.createTab('/project', { sessionId: 'sess-old' })
      store.setTabPty(tabId, 'pty-f')
      store.assignSessionIdByPtyId('pty-f', 'sess-new')
      const r = store.resumeSessionTab('/project', 'sess-old')
      expect(r.reused).toBe(true)
      expect(r.tabId).toBe(tabId)
    })

    // 无已有 Tab 时新建（带名字），标记 reused=false
    it('ResumeSessionTab_CreateNew_001', () => {
      const store = useSessionStore()
      const r = store.resumeSessionTab('/project', 'sess-new-1', '会话名')
      expect(r.reused).toBe(false)
      const tab = store.tabs.get(r.tabId)!
      expect(tab.sessionId).toBe('sess-new-1')
      expect(tab.name).toBe('会话名')
    })
  })

  // ==================== isResume ====================

  describe('isResume', () => {
    // 创建 tab 时传入 sessionId，isResume 为 true
    it('IsResume_True_001', () => {
      const store = useSessionStore()
      const tabId = store.createTab('/project', { sessionId: 'sess-resume-001' })
      const tab = store.tabs.get(tabId)!
      expect(tab.isResume).toBe(true)
    })

    // 创建 tab 时不传 sessionId，isResume 为 false
    it('IsResume_False_001', () => {
      const store = useSessionStore()
      const tabId = store.createTab('/project')
      const tab = store.tabs.get(tabId)!
      expect(tab.isResume).toBe(false)
    })

    // 创建 tab 时传入 sessionId 和 name，isResume 仍为 true
    it('IsResume_WithName_001', () => {
      const store = useSessionStore()
      const tabId = store.createTab('/project', { sessionId: 'sess-123', name: 'My Session' })
      const tab = store.tabs.get(tabId)!
      expect(tab.isResume).toBe(true)
      expect(tab.name).toBe('My Session')
    })
  })

  // ==================== adoptRemoteSessions（浏览器远程模式：运行中会话恢复入口） ====================

  describe('adoptRemoteSessions', () => {
    // 空列表采纳 2 个运行中会话：各建 running Tab，绑定 ptyId，name 取目录名
    it('AdoptRemoteSessions_NewSessions_001', () => {
      const store = useSessionStore()
      store.adoptRemoteSessions([
        { id: 'pty-1', type: 'claude', cwd: 'D:\\work\\proj-a', working: true, pending: false },
        { id: 'pty-2', type: 'claude', cwd: '/home/user/proj-b', working: false, pending: true },
      ])
      expect(store.tabs.size).toBe(2)
      const tab1 = store.getTabByPtyId('pty-1')!
      expect(tab1.status).toBe('running')
      expect(tab1.projectPath).toBe('D:\\work\\proj-a')
      expect(tab1.name).toBe('proj-a')
      expect(tab1.working).toBe(true)
      const tab2 = store.getTabByPtyId('pty-2')!
      expect(tab2.name).toBe('proj-b')
      expect(tab2.pending).toBe(true)
    })

    // 重复采纳同 ptyId：不新建 Tab，仅更新 working/pending
    it('AdoptRemoteSessions_DedupByPtyId_001', () => {
      const store = useSessionStore()
      store.adoptRemoteSessions([
        { id: 'pty-1', type: 'claude', cwd: '/proj', working: false, pending: false },
      ])
      const tabId = store.getTabByPtyId('pty-1')!.tabId
      store.adoptRemoteSessions([
        { id: 'pty-1', type: 'claude', cwd: '/proj', working: true, pending: false },
      ])
      expect(store.tabs.size).toBe(1)
      const tab = store.getTabByPtyId('pty-1')!
      expect(tab.tabId).toBe(tabId)
      expect(tab.working).toBe(true)
    })

    // 采纳时历史已加载：sessionId 命中历史记录 → Tab 名取会话名（非项目名）
    it('AdoptRemoteSessions_UsesHistoryName_001', async () => {
      const mockGetSessions = vi.mocked(getSessions)
      mockGetSessions.mockResolvedValue([
        { sessionId: 'sess-a', name: '修复登录 bug', projectPath: '/p', lastActiveAt: 1 } as never,
      ])
      const store = useSessionStore()
      await store.loadHistorySessions('/p')

      store.adoptRemoteSessions([
        { id: 'pty-h1', type: 'claude', cwd: '/p', working: true, pending: false, sessionId: 'sess-a' },
      ])
      expect(store.getTabByPtyId('pty-h1')?.name).toBe('修复登录 bug')
      mockGetSessions.mockReset()
    })

    // 采纳时历史未加载（浏览器直接打开时序）：回退项目名，历史加载完成后
    // backfillTabNamesFromHistory 按 sessionId 修正为会话名
    it('AdoptRemoteSessions_BackfillAfterHistoryLoad_001', async () => {
      const store = useSessionStore()
      store.adoptRemoteSessions([
        { id: 'pty-h2', type: 'claude', cwd: '/p', working: false, pending: false, sessionId: 'sess-b' },
      ])
      expect(store.getTabByPtyId('pty-h2')?.name).toBe('p')

      const mockGetSessions = vi.mocked(getSessions)
      mockGetSessions.mockResolvedValue([
        { sessionId: 'sess-b', name: '重构远程连接', projectPath: '/p', lastActiveAt: 2 } as never,
      ])
      await store.loadHistorySessions('/p')
      store.backfillTabNamesFromHistory()

      expect(store.getTabByPtyId('pty-h2')?.name).toBe('重构远程连接')
      mockGetSessions.mockReset()
    })

    // 名字已被改过（≠ 项目名：重命名/自动命名）的 Tab，回填不覆盖
    it('AdoptRemoteSessions_BackfillKeepsRenamed_001', async () => {
      const store = useSessionStore()
      store.adoptRemoteSessions([
        { id: 'pty-h3', type: 'claude', cwd: '/p', working: false, pending: false, sessionId: 'sess-c' },
      ])
      store.getTabByPtyId('pty-h3')!.name = '我的会话'

      const mockGetSessions = vi.mocked(getSessions)
      mockGetSessions.mockResolvedValue([
        { sessionId: 'sess-c', name: '历史名', projectPath: '/p', lastActiveAt: 3 } as never,
      ])
      await store.loadHistorySessions('/p')
      store.backfillTabNamesFromHistory()

      expect(store.getTabByPtyId('pty-h3')?.name).toBe('我的会话')
      mockGetSessions.mockReset()
    })

    // 已退出的会话不在列表中返回时，已有 Tab 不受影响（等待 pty-exit 事件清理）
    it('AdoptRemoteSessions_KeepExistingTabs_001', () => {
      const store = useSessionStore()
      store.adoptRemoteSessions([
        { id: 'pty-1', type: 'claude', cwd: '/proj', working: false, pending: false },
      ])
      store.adoptRemoteSessions([])
      expect(store.tabs.size).toBe(1)
    })

    // 实例名单合并：Gateway 下发的全量名单优先（含零会话实例），Tab 兜底补齐
    it('KnownInstances_MergeRemoteRoster_001', () => {
      const store = useSessionStore()
      // 只有一个实例有运行会话（Tab 推导仅能看到它）
      store.adoptRemoteSessions([
        { id: 'pty-i1', type: 'claude', cwd: '/p', working: false, pending: false, instanceId: 'inst-1', instanceName: '实例 1' },
      ])
      expect(store.knownInstances.map(i => i.id)).toEqual(['inst-1'])

      // Gateway 名单补齐零会话实例；Tab 里未覆盖的实例也保留（旧桌面兜底）
      store.setRemoteInstances([
        { id: 'inst-1', name: '实例 1' },
        { id: 'inst-2', name: '实例 2' },
      ])
      store.adoptRemoteSessions([
        { id: 'pty-i9', type: 'claude', cwd: '/q', working: false, pending: false, instanceId: 'inst-9', instanceName: '实例 9' },
      ])
      expect(store.knownInstances).toEqual([
        { id: 'inst-1', name: '实例 1' },
        { id: 'inst-2', name: '实例 2' },
        { id: 'inst-9', name: '实例 9' },
      ])
    })

    // 零窗口判定：空名单也是有效状态；重连前必须回到“尚未加载”
    it('RemoteInstances_EmptyKnownFlag_001', () => {
      const store = useSessionStore()
      expect(store.remoteInstancesKnown).toBe(false)
      store.setRemoteInstances([])
      expect(store.remoteInstancesKnown).toBe(true)
      expect(store.remoteInstances).toEqual([])
      store.resetRemoteInstances()
      expect(store.remoteInstancesKnown).toBe(false)
    })

    // 零窗口：残留 Tab 的实例不能继续出现在选择器里
    it('KnownInstances_EmptyRosterDropsTabs_001', () => {
      const store = useSessionStore()
      store.adoptRemoteSessions([
        { id: 'pty-dead', type: 'claude', cwd: '/p', working: false, pending: false, instanceId: 'dead', instanceName: 'dead' },
      ])
      store.setRemoteInstances([])
      expect(store.knownInstances).toEqual([])
    })

    // 多实例远程：instanceId/instanceName 透传到 Tab（会话列表分组渲染依据）
    it('AdoptRemoteSessions_InstanceFields_001', () => {
      const store = useSessionStore()
      store.adoptRemoteSessions([
        { id: 'pty-i1', type: 'claude', cwd: '/proj-a', working: false, pending: false, instanceId: 'inst-1', instanceName: '实例 1' },
        { id: 'pty-i2', type: 'claude', cwd: '/proj-b', working: false, pending: false, instanceId: 'inst-2', instanceName: '实例 2' },
        // 旧桌面/单实例：无实例字段，Tab 侧为 undefined（降级平铺）
        { id: 'pty-i3', type: 'shell', cwd: '/proj-c', working: false, pending: false },
      ])
      expect(store.getTabByPtyId('pty-i1')?.instanceId).toBe('inst-1')
      expect(store.getTabByPtyId('pty-i1')?.instanceName).toBe('实例 1')
      expect(store.getTabByPtyId('pty-i2')?.instanceName).toBe('实例 2')
      expect(store.getTabByPtyId('pty-i3')?.instanceId).toBeUndefined()
    })

    // 采纳远端会话不得消费用户待用的实例选择（刷新列表不能劫持下一步新建/恢复）
    it('AdoptRemoteSessions_KeepsPendingInstance_001', () => {
      const store = useSessionStore()
      store.pendingInstanceId = 'inst-2'
      store.adoptRemoteSessions([
        { id: 'pty-i1', type: 'claude', cwd: '/proj-a', working: false, pending: false, instanceId: 'inst-1', instanceName: '实例 1' },
      ])
      // 采纳的 Tab 用自己的实例字段
      expect(store.getTabByPtyId('pty-i1')?.instanceId).toBe('inst-1')
      // 用户选择保留，下一步 createTab 仍路由到 inst-2
      expect(store.pendingInstanceId).toBe('inst-2')
      const tabId = store.createTab('/proj-new')
      expect(store.tabs.get(tabId)?.instanceId).toBe('inst-2')
      expect(store.pendingInstanceId).toBeNull()
    })

    // 已有 Tab 重复采纳时实例字段跟随更新（Gateway 重选后实例名可能变化）
    it('AdoptRemoteSessions_InstanceFieldsUpdate_001', () => {
      const store = useSessionStore()
      store.adoptRemoteSessions([
        { id: 'pty-i1', type: 'claude', cwd: '/proj-a', working: false, pending: false, instanceId: 'inst-1', instanceName: '实例 1' },
      ])
      store.adoptRemoteSessions([
        { id: 'pty-i1', type: 'claude', cwd: '/proj-a', working: false, pending: false, instanceId: 'inst-1', instanceName: '实例 1' },
        { id: 'pty-i1b', type: 'claude', cwd: '/proj-a', working: false, pending: false, instanceId: 'inst-9', instanceName: '实例 9' },
      ])
      // 原 Tab 实例字段保持；新 Tab 用新值
      expect(store.getTabByPtyId('pty-i1')?.instanceId).toBe('inst-1')
      expect(store.getTabByPtyId('pty-i1b')?.instanceId).toBe('inst-9')
    })

    // 服务端下发 sessionId 时建 Tab 绑定该 id（重连后从历史恢复不再二次 spawn 的前提）
    it('AdoptRemoteSessions_WithSessionId_001', () => {
      const store = useSessionStore()
      store.adoptRemoteSessions([
        { id: 'pty-1', type: 'claude', cwd: '/proj', working: false, pending: false, sessionId: 'sess-x' },
      ])
      const tab = store.getTabByPtyId('pty-1')!
      expect(tab.sessionId).toBe('sess-x')
      expect(store.claimedSessionIds.has('sess-x')).toBe(true)
    })

    // 已有同 ptyId Tab 时，服务端 sessionId 补写到该 Tab（演变历史）
    it('AdoptRemoteSessions_BackfillSessionId_001', () => {
      const store = useSessionStore()
      store.adoptRemoteSessions([
        { id: 'pty-1', type: 'claude', cwd: '/proj', working: false, pending: false },
      ])
      store.adoptRemoteSessions([
        { id: 'pty-1', type: 'claude', cwd: '/proj', working: false, pending: false, sessionId: 'sess-y' },
      ])
      const tab = store.getTabByPtyId('pty-1')!
      expect(tab.sessionId).toBe('sess-y')
      expect(store.claimedSessionIds.has('sess-y')).toBe(true)
    })
  })

  describe('重命名双端同步（renameKeyForTab / applyRemoteRename）', () => {
  // 复现 bug：stopped Tab 的 ptyId 被 handlePtyExit 置空后重命名不广播，
  // 另一端名字不同步——修复后 stopped Tab 以 sessionId 为广播 key
  it('RenameKey_StoppedTab用sessionId做广播key_001', () => {
    const store = useSessionStore()
    const tabId = store.createTab('/proj', { sessionId: 'sess-abc-111', name: '旧名' })
    // 模拟启动后退出：ptyId 置空、status=stopped（handlePtyExit 语义）
    store.setTabPty(tabId, 'pty-1')
    store.handlePtyExit('pty-1')
    const tab = store.tabs.get(tabId)!
    expect(tab.ptyId).toBeNull()

    // 广播 key 回退到 sessionId
    expect(store.renameKeyForTab(tabId)).toBe('sess-abc-111')
  })

  it('RenameKey_RunningTab用ptyId_002', () => {
    const store = useSessionStore()
    const tabId = store.createTab('/proj')
    store.setTabPty(tabId, 'pty-2')
    store.setTabSessionId(tabId, 'sess-2')

    expect(store.renameKeyForTab(tabId)).toBe('pty-2')
  })

  // 从未启动的 New Session：无 ptyId 无 sessionId，不广播（null）
  it('RenameKey_无标识返回null_003', () => {
    const store = useSessionStore()
    const tabId = store.createTab('/proj')

    expect(store.renameKeyForTab(tabId)).toBeNull()
  })

  // 接收端：stopped Tab 按 sessionId（含演变历史）匹配广播并改名
  it('ApplyRemoteRename_按sessionId匹配stoppedTab_004', () => {
    const store = useSessionStore()
    const tabId = store.createTab('/proj', { sessionId: 'old-id', name: '旧名' })
    // resume fork：sessionId 演变，旧 id 保留在 sessionIds
    store.setTabSessionId(tabId, 'new-id')

    expect(store.applyRemoteRename('new-id', '新名')).toBe(true)
    expect(store.tabs.get(tabId)!.name).toBe('新名')
    // 旧 id 也命中同一 Tab（演变历史匹配）
    expect(store.applyRemoteRename('old-id', '旧id命中')).toBe(true)
    expect(store.tabs.get(tabId)!.name).toBe('旧id命中')
  })

  // 接收端：running Tab 按 ptyId 匹配（原有行为不回归）
  it('ApplyRemoteRename_按ptyId匹配runningTab_005', () => {
    const store = useSessionStore()
    const tabId = store.createTab('/proj')
    store.setTabPty(tabId, 'pty-3')

    expect(store.applyRemoteRename('pty-3', '运行中新名')).toBe(true)
    expect(store.tabs.get(tabId)!.name).toBe('运行中新名')
  })

  // 无匹配（如另一端已关闭该 Tab）返回 false，静默丢弃
  it('ApplyRemoteRename_无匹配返回false_006', () => {
    const store = useSessionStore()
    expect(store.applyRemoteRename('ghost', '名字')).toBe(false)
  })
  })
})

describe('session store 远程 attach 拒绝与剥离（同类问题回归）', () => {
  beforeEach(() => {
    setActivePinia(createPinia())
  })

  /** 建立一个 running 的远程 Tab（ptyId 绑定） */
  function runningTab(): { tabId: string; ptyId: string } {
    const store = useSessionStore()
    const tabId = store.createTab('/project')
    const ptyId = 'pty-denied-1'
    store.setTabPty(tabId, ptyId)
    return { tabId, ptyId }
  }

  /**
   * attach 被拒必须按原因分支：控制权被占 → takenOver 遮罩（接管入口）；
   * 会话不存在（窗口已关）→ Tab 置 stopped——若统一显示「被接管」，
   * 用户点接管后会永久循环被拒
   */
  it('SessionStore_Attach被拒控制权分支_001', () => {
    const store = useSessionStore()
    const { tabId, ptyId } = runningTab()
    store.handleAttachDenied(ptyId, 'controller held by another device')
    const tab = store.tabs.get(tabId)!
    expect(tab.takenOver).toBe(true)
    expect(tab.status).toBe('running')
  })

  it('SessionStore_Attach被拒会话不存在分支_002', () => {
    const store = useSessionStore()
    const { tabId, ptyId } = runningTab()
    store.handleAttachDenied(ptyId, 'session not found')
    const tab = store.tabs.get(tabId)!
    expect(tab.status).toBe('stopped')
    expect(tab.takenOver).toBeUndefined()
  })

  /**
   * 宿主窗口关闭时 Broker 广播 session.detached{instance disconnected}——
   * 不处理的话 Tab 永远 running、终端冻结
   */
  it('SessionStore_远端剥离置Stopped_003', () => {
    const store = useSessionStore()
    const { tabId, ptyId } = runningTab()
    store.handleRemoteDetached(ptyId)
    expect(store.tabs.get(tabId)!.status).toBe('stopped')
  })
})
