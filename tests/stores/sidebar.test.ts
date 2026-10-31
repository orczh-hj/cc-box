import { describe, it, expect, beforeEach, vi } from 'vitest'
import { setActivePinia, createPinia } from 'pinia'

// Mock sidebar API（含 load 与 set 两类）
const mockGetAllSkills = vi.fn()
const mockGetAllAgents = vi.fn()
const mockGetAllMcpServers = vi.fn()
const mockGetAllPlugins = vi.fn()
const mockSetSkillEnabled = vi.fn()
const mockSetAgentEnabled = vi.fn()
const mockSetMcpServerEnabled = vi.fn()
const mockSetPluginEnabled = vi.fn()
const mockSetHookEnabled = vi.fn()

vi.mock('@/api/tauri', () => ({
  getAllSkills: (...args: unknown[]) => mockGetAllSkills(...args),
  getAllAgents: (...args: unknown[]) => mockGetAllAgents(...args),
  getAllMcpServers: (...args: unknown[]) => mockGetAllMcpServers(...args),
  getAllPlugins: (...args: unknown[]) => mockGetAllPlugins(...args),
  getHookList: vi.fn(() => Promise.resolve([])),
  setSkillEnabled: (...args: unknown[]) => mockSetSkillEnabled(...args),
  setAgentEnabled: (...args: unknown[]) => mockSetAgentEnabled(...args),
  setMcpServerEnabled: (...args: unknown[]) => mockSetMcpServerEnabled(...args),
  setPluginEnabled: (...args: unknown[]) => mockSetPluginEnabled(...args),
  setHookEnabled: (...args: unknown[]) => mockSetHookEnabled(...args),
}))

import { useSidebarStore } from '@/stores/sidebar'

describe('sidebar store', () => {
  beforeEach(() => {
    setActivePinia(createPinia())
    mockGetAllSkills.mockReset()
    mockGetAllAgents.mockReset()
    mockGetAllMcpServers.mockReset()
    mockGetAllPlugins.mockReset()
    mockSetSkillEnabled.mockReset()
    mockSetAgentEnabled.mockReset()
    mockSetMcpServerEnabled.mockReset()
    mockSetPluginEnabled.mockReset()
    mockSetHookEnabled.mockReset()
  })

  describe('togglePanel', () => {
    // panelVisible=false 时调用 togglePanel("sessions") 后 panelVisible 变为 true
    it('TogglePanel_Open_001', () => {
      const store = useSidebarStore()

      expect(store.panelVisible).toBe(false)
      expect(store.activePanel).toBeNull()

      store.togglePanel('sessions')

      expect(store.panelVisible).toBe(true)
      expect(store.activePanel).toBe('sessions')
    })

    // 面板已打开且 activePanel="sessions" 时调用 togglePanel("sessions") 后 panelVisible 变为 false
    it('TogglePanel_Close_001', () => {
      const store = useSidebarStore()

      store.activePanel = 'sessions'
      store.panelVisible = true

      store.togglePanel('sessions')

      expect(store.panelVisible).toBe(false)
    })

    // activePanel="sessions" 时调用 togglePanel("mcp") 后 activePanel 变为 "mcp"
    it('TogglePanel_Switch_001', () => {
      const store = useSidebarStore()

      store.activePanel = 'sessions'
      store.panelVisible = true

      store.togglePanel('mcp')

      expect(store.activePanel).toBe('mcp')
      expect(store.panelVisible).toBe(true)
    })

    // showSettings=true 时调用 togglePanel("sessions") 后 showSettings 变为 false
    it('TogglePanel_CloseSettings_001', () => {
      const store = useSidebarStore()

      store.showSettings = true

      store.togglePanel('sessions')

      expect(store.showSettings).toBe(false)
      expect(store.activePanel).toBe('sessions')
      expect(store.panelVisible).toBe(true)
    })
  })

  describe('updateAvailable badge', () => {
    // 无任何更新时 badge 不显示
    it('Badge_None_001', () => {
      const store = useSidebarStore()
      expect(store.updateAvailable).toBe(false)
    })

    // 仅软件更新时 badge 显示
    it('Badge_AppUpdate_001', () => {
      const store = useSidebarStore()
      store.setUpdateInfo({ version: '0.8.0', currentVersion: '0.7.0', hasUpdate: true, releaseNotes: '', downloadUrl: '', platformAsset: null })
      expect(store.updateAvailable).toBe(true)
    })

    // Claude CLI 更新不再驱动 badge（启动检查已改为只读本地版本，不对比 OSS）
    it('Badge_CliUpdate_Ignored_001', () => {
      const store = useSidebarStore()
      store.setClaudeCliUpdateInfo({ installedVersion: '1.0.30', latestVersion: '1.0.33', hasUpdate: true, notInstalled: false })
      expect(store.updateAvailable).toBe(false)
    })

    // 两者都有更新时 badge 显示
    it('Badge_Both_001', () => {
      const store = useSidebarStore()
      store.setUpdateInfo({ version: '0.8.0', currentVersion: '0.7.0', hasUpdate: true, releaseNotes: '', downloadUrl: '', platformAsset: null })
      store.setClaudeCliUpdateInfo({ installedVersion: '1.0.30', latestVersion: '1.0.33', hasUpdate: true, notInstalled: false })
      expect(store.updateAvailable).toBe(true)
    })

    // 软件无更新但 Claude CLI 无更新时 badge 不显示
    it('Badge_NoUpdate_001', () => {
      const store = useSidebarStore()
      store.setUpdateInfo({ version: '0.7.0', currentVersion: '0.7.0', hasUpdate: false, releaseNotes: '', downloadUrl: '', platformAsset: null })
      store.setClaudeCliUpdateInfo({ installedVersion: '1.0.33', latestVersion: '1.0.33', hasUpdate: false, notInstalled: false })
      expect(store.updateAvailable).toBe(false)
    })
  })

  describe('用户级资源开关 toggle 方法（乐观更新，不调 loadXxx）', () => {
    // 成功时：直接更新 store.skills 对应项的 enabled，不调 getAllSkills
    it('ToggleSkill_Success_OptimisticUpdate_001', async () => {
      const store = useSidebarStore()
      store.skills = [
        { name: 'deploy', displayName: 'deploy', sourceType: 'user', sourceLabel: 'User', invokeFormat: '/deploy', enabled: true },
      ]
      mockSetSkillEnabled.mockResolvedValue(undefined)

      await store.toggleSkillEnabled('deploy', false)

      expect(mockSetSkillEnabled).toHaveBeenCalledWith('deploy', false, undefined, undefined)
      expect(mockGetAllSkills).not.toHaveBeenCalled()
      expect(store.skills[0].enabled).toBe(false)
    })

    // 其他三个 toggle 同样不调 loadXxx
    it('ToggleAgent_NoLoad_001', async () => {
      const store = useSidebarStore()
      store.agents = [
        { name: 'reviewer', displayName: 'reviewer', sourceType: 'user', sourceLabel: 'User', invokeFormat: '@reviewer', enabled: true },
      ]
      mockSetAgentEnabled.mockResolvedValue(undefined)

      await store.toggleAgentEnabled('reviewer', false)

      expect(mockGetAllAgents).not.toHaveBeenCalled()
      expect(store.agents[0].enabled).toBe(false)
    })

    it('ToggleMcp_NoLoad_001', async () => {
      const store = useSidebarStore()
      store.mcpServers = [
        { name: 'zread', displayName: 'zread', sourceType: 'user', sourceLabel: 'User', prompts: [], enabled: true },
      ]
      mockSetMcpServerEnabled.mockResolvedValue(undefined)

      await store.toggleMcpServerEnabled('zread', false)

      expect(mockGetAllMcpServers).not.toHaveBeenCalled()
      expect(store.mcpServers[0].enabled).toBe(false)
    })

    it('TogglePlugin_NoReloadPlugins_001', async () => {
      const store = useSidebarStore()
      store.plugins = [
        { id: 'paper-tool@orczh', name: 'paper-tool', version: '1.0', scope: 'user', enabled: true, installPath: '/x' },
      ]
      ;(store as unknown as { loadedCwd: string }).loadedCwd = '/proj'
      mockSetPluginEnabled.mockResolvedValue(undefined)

      await store.togglePluginEnabled('paper-tool@orczh', false)

      // toggle 本身不重拉任何列表：plugin 乐观更新保留，子项（skills/agents/mcp）
      // 刷新统一由 state-changed 回环事件驱动（Rust 写命令广播 → App.vue 订阅 → 状态中心 sections.ts）
      expect(mockGetAllPlugins).not.toHaveBeenCalled()
      expect(mockGetAllSkills).not.toHaveBeenCalled()
      expect(mockGetAllAgents).not.toHaveBeenCalled()
      expect(mockGetAllMcpServers).not.toHaveBeenCalled()
      expect(store.plugins[0].enabled).toBe(false)
    })

    // Hook 开关：按列表下发的 key 定位，乐观更新 + 本端抑制（与其他资源一致）
    it('ToggleHook_Success_OptimisticUpdate_001', async () => {
      const store = useSidebarStore()
      store.hooks = [
        {
          event: 'PreToolUse',
          command: 'echo hi',
          hookType: 'command',
          hookCount: 1,
          source: { type: 'user', label: 'User' },
          enabled: true,
          key: 'k1',
          scope: 'user',
        },
      ]
      mockSetHookEnabled.mockResolvedValue(undefined)

      await store.toggleHookEnabled(store.hooks[0], false)

      // 独立真值：user 作用域不带 projectPath
      expect(mockSetHookEnabled).toHaveBeenCalledWith('user', 'PreToolUse', 'k1', false, undefined)
      expect(store.hooks[0].enabled).toBe(false)
      // 本端写回环已登记（state-changed 到达时跳过刷新）
      expect(store.consumeSelfWrite('hooks')).toBe(true)
    })

    it('ToggleHook_ProjectScope_PassesLoadedCwd_001', async () => {
      const store = useSidebarStore()
      ;(store as unknown as { loadedCwd: string }).loadedCwd = 'D:/proj'
      store.hooks = [
        {
          event: 'Stop',
          command: 'npm test',
          hookType: 'command',
          hookCount: 1,
          source: { type: 'project', label: 'Project' },
          enabled: true,
          key: 'k2',
          scope: 'project',
        },
      ]
      mockSetHookEnabled.mockResolvedValue(undefined)

      await store.toggleHookEnabled(store.hooks[0], false)

      expect(mockSetHookEnabled).toHaveBeenCalledWith('project', 'Stop', 'k2', false, 'D:/proj')
    })

    it('ToggleHook_Failure_Rollback_001', async () => {
      const store = useSidebarStore()
      store.hooks = [
        {
          event: 'Stop',
          command: 'npm test',
          hookType: 'command',
          hookCount: 1,
          source: { type: 'user', label: 'User' },
          enabled: true,
          key: 'k3',
          scope: 'user',
        },
      ]
      mockSetHookEnabled.mockRejectedValue(new Error('config changed externally'))

      await expect(store.toggleHookEnabled(store.hooks[0], false)).rejects.toThrow(
        'config changed externally'
      )

      expect(store.hooks[0].enabled).toBe(true) // 回滚
    })

    // loadedCwd 为空时（尚未预加载过）不触发 reload，避免无 cwd 调用
    it('TogglePlugin_NoReloadWhenNoCwd_001', async () => {
      const store = useSidebarStore()
      store.plugins = [
        { id: 'paper-tool@orczh', name: 'paper-tool', version: '1.0', scope: 'user', enabled: true, installPath: '/x' },
      ]
      mockSetPluginEnabled.mockResolvedValue(undefined)

      await store.togglePluginEnabled('paper-tool@orczh', false)

      expect(mockGetAllSkills).not.toHaveBeenCalled()
      expect(mockGetAllAgents).not.toHaveBeenCalled()
      expect(mockGetAllMcpServers).not.toHaveBeenCalled()
    })

    // 失败时：store 数据回滚到原值，错误向上抛
    it('ToggleSkill_Failure_Rollback_001', async () => {
      const store = useSidebarStore()
      store.skills = [
        { name: 'deploy', displayName: 'deploy', sourceType: 'user', sourceLabel: 'User', invokeFormat: '/deploy', enabled: true },
      ]
      mockSetSkillEnabled.mockRejectedValue(new Error('active skill not found'))

      await expect(store.toggleSkillEnabled('deploy', false)).rejects.toThrow('active skill not found')

      expect(mockGetAllSkills).not.toHaveBeenCalled()
      expect(store.skills[0].enabled).toBe(true) // 回滚
    })

    // 不存在的项：findIndex 返回 -1，still 调用 API（让后端错误反馈给用户）
    it('ToggleSkill_NotInStore_001', async () => {
      const store = useSidebarStore()
      store.skills = []
      mockSetSkillEnabled.mockResolvedValue(undefined)

      await store.toggleSkillEnabled('ghost', false)

      expect(mockSetSkillEnabled).toHaveBeenCalledWith('ghost', false, undefined, undefined)
    })

    // toggle plugin 失败时：plugin.enabled 回滚，且不触发子项 reload
    it('TogglePlugin_Failure_NoReload_001', async () => {
      const store = useSidebarStore()
      store.plugins = [
        { id: 'paper-tool@orczh', name: 'paper-tool', version: '1.0', scope: 'user', enabled: true, installPath: '/x' },
      ]
      ;(store as unknown as { loadedCwd: string }).loadedCwd = '/proj'
      mockSetPluginEnabled.mockRejectedValue(new Error('network down'))

      await expect(store.togglePluginEnabled('paper-tool@orczh', false)).rejects.toThrow('network down')

      expect(store.plugins[0].enabled).toBe(true) // 回滚
      expect(mockGetAllSkills).not.toHaveBeenCalled()
      expect(mockGetAllAgents).not.toHaveBeenCalled()
      expect(mockGetAllMcpServers).not.toHaveBeenCalled()
    })
  })

  describe('加载：静默刷新与同名精确定位', () => {
    // 静默刷新（state-changed 回环/级联）不切换 loading，避免面板卸载重建
    it('LoadSkills_Silent_KeepsLoadingFalse_001', async () => {
      const store = useSidebarStore()
      store.skills = [
        { name: 'deploy', displayName: 'deploy', sourceType: 'user', sourceLabel: 'User', invokeFormat: '/deploy', enabled: true },
      ]
      let resolveFetch: (v: unknown[]) => void = () => {}
      mockGetAllSkills.mockReturnValue(new Promise(resolve => { resolveFetch = resolve }))

      const pending = store.loadSkills('/proj', { silent: true })
      await Promise.resolve()

      expect(store.skillsLoading).toBe(false)
      expect(store.skills[0].name).toBe('deploy') // 旧数据保留

      resolveFetch([])
      await pending
      expect(store.skillsLoading).toBe(false)
      expect(store.skills).toEqual([])
    })

    // 首屏/切项目（非 silent）才显示 loading
    it('LoadSkills_NonSilent_SetsLoading_001', async () => {
      const store = useSidebarStore()
      let resolveFetch: (v: unknown[]) => void = () => {}
      mockGetAllSkills.mockReturnValue(new Promise(resolve => { resolveFetch = resolve }))

      const pending = store.loadSkills('/proj')
      await Promise.resolve()
      expect(store.skillsLoading).toBe(true)

      resolveFetch([])
      await pending
      expect(store.skillsLoading).toBe(false)
    })

    // 并发刷新乱序响应：旧请求不得覆盖新数据
    it('LoadSkills_OutOfOrderResponse_Discarded_001', async () => {
      const store = useSidebarStore()
      let resolveOld: (v: unknown[]) => void = () => {}
      let resolveNew: (v: unknown[]) => void = () => {}
      mockGetAllSkills
        .mockReturnValueOnce(new Promise(resolve => { resolveOld = resolve }))
        .mockReturnValueOnce(new Promise(resolve => { resolveNew = resolve }))
      const oldSkill = { name: 'old', displayName: 'old', sourceType: 'user', sourceLabel: 'User', invokeFormat: '/old', enabled: true }
      const newSkill = { name: 'new', displayName: 'new', sourceType: 'user', sourceLabel: 'User', invokeFormat: '/new', enabled: true }

      const first = store.loadSkills('/proj', { silent: true })
      const second = store.loadSkills('/proj', { silent: true })
      resolveNew([newSkill])
      await second
      resolveOld([oldSkill])
      await first

      expect(store.skills.map(s => s.name)).toEqual(['new'])
    })

    // 同名 project/user 并存：开关只能改 user 项（project 项没有开关，后端也只移动用户级）
    it('ToggleSkill_SameNameProjectAndUser_TargetsUser_001', async () => {
      const store = useSidebarStore()
      store.skills = [
        { name: 'deploy', displayName: 'deploy', sourceType: 'project', sourceLabel: 'Project', invokeFormat: '/deploy', enabled: true },
        { name: 'deploy', displayName: 'deploy', sourceType: 'user', sourceLabel: 'User', invokeFormat: '/deploy', enabled: true },
      ]
      mockSetSkillEnabled.mockResolvedValue(undefined)

      await store.toggleSkillEnabled('deploy', false)

      expect(store.skills[0].enabled).toBe(true) // project 项不动
      expect(store.skills[1].enabled).toBe(false) // user 项被关闭
    })

    // active/disabled 同名共存：开启操作选中禁用项，关闭操作选中启用项
    it('ToggleSkill_SameNameActiveAndDisabled_TargetsCorrect_001', async () => {
      const store = useSidebarStore()
      store.skills = [
        { name: 'deploy', displayName: 'deploy', sourceType: 'user', sourceLabel: 'User (Disabled)', invokeFormat: '/deploy', enabled: false },
      ]
      mockSetSkillEnabled.mockResolvedValue(undefined)

      await store.toggleSkillEnabled('deploy', true)

      expect(store.skills[0].enabled).toBe(true)
    })

    // agent 同理：plugin 同名项不可被误改
    it('ToggleAgent_SameNamePluginAndUser_TargetsUser_001', async () => {
      const store = useSidebarStore()
      store.agents = [
        { name: 'reviewer', displayName: 'reviewer', sourceType: 'plugin', sourceLabel: 'Plugin', invokeFormat: '@"reviewer (agent)"', enabled: true },
        { name: 'reviewer', displayName: 'reviewer', sourceType: 'user', sourceLabel: 'User', invokeFormat: '@"reviewer (agent)"', enabled: true },
      ]
      mockSetAgentEnabled.mockResolvedValue(undefined)

      await store.toggleAgentEnabled('reviewer', false)

      expect(store.agents[0].enabled).toBe(true)
      expect(store.agents[1].enabled).toBe(false)
    })

    // 开关成功登记本端写：事件回来时被消费一次，不触发列表刷新
    it('ToggleSkill_Success_MarksSelfWrite_001', async () => {
      const store = useSidebarStore()
      store.skills = [
        { name: 'deploy', displayName: 'deploy', sourceType: 'user', sourceLabel: 'User', invokeFormat: '/deploy', enabled: true },
      ]
      mockSetSkillEnabled.mockResolvedValue(undefined)

      await store.toggleSkillEnabled('deploy', false)

      expect(store.consumeSelfWrite('skills')).toBe(true)
      expect(store.consumeSelfWrite('skills')).toBe(false) // 只消费一次
    })

    // 开关失败撤销登记：后续别的设备的事件不能被误吞
    it('ToggleSkill_Failure_CancelsSelfWrite_001', async () => {
      const store = useSidebarStore()
      store.skills = [
        { name: 'deploy', displayName: 'deploy', sourceType: 'user', sourceLabel: 'User', invokeFormat: '/deploy', enabled: true },
      ]
      mockSetSkillEnabled.mockRejectedValue(new Error('boom'))

      await expect(store.toggleSkillEnabled('deploy', false)).rejects.toThrow('boom')

      expect(store.consumeSelfWrite('skills')).toBe(false)
      expect(store.skills[0].enabled).toBe(true) // 已回滚
    })

    // 失败回滚必须按 sourceLabel 区分 active/disabled 同名项（不能翻错行）
    it('ToggleAgent_Failure_SameNameRevertsClickedRow_001', async () => {
      const store = useSidebarStore()
      store.agents = [
        { name: 'reviewer', displayName: 'reviewer', sourceType: 'user', sourceLabel: 'User', invokeFormat: '@"reviewer (agent)"', enabled: true },
        { name: 'reviewer', displayName: 'reviewer', sourceType: 'user', sourceLabel: 'User (Disabled)', invokeFormat: '@"reviewer (agent)"', enabled: false },
      ]
      mockSetAgentEnabled.mockRejectedValue(new Error('conflict'))

      // 点 disabled 行（启用）→ 后端冲突 → 只回滚该行
      await expect(store.toggleAgentEnabled('reviewer', true)).rejects.toThrow('conflict')

      expect(store.agents[0].enabled).toBe(true) // active 行未被误改
      expect(store.agents[1].enabled).toBe(false) // 被点的 disabled 行已回滚
    })

    // 并发静默刷新已把数组换成权威状态时，失败回滚只能 no-op，不能翻同名行
    it('ToggleAgent_Failure_AfterRefresh_NoWrongRow_001', async () => {
      const store = useSidebarStore()
      const activeRow = { name: 'reviewer', displayName: 'reviewer', sourceType: 'user' as const, sourceLabel: 'User', invokeFormat: '@"reviewer (agent)"', enabled: true }
      const disabledRow = { name: 'reviewer', displayName: 'reviewer', sourceType: 'user' as const, sourceLabel: 'User (Disabled)', invokeFormat: '@"reviewer (agent)"', enabled: false }
      store.agents = [{ ...activeRow }, { ...disabledRow }]
      mockSetAgentEnabled.mockImplementation(async () => {
        // 模拟请求期间到达的权威刷新（disabled 行仍为 false）
        store.agents = [{ ...activeRow }, { ...disabledRow }]
        throw new Error('conflict')
      })

      await expect(store.toggleAgentEnabled('reviewer', true)).rejects.toThrow('conflict')

      expect(store.agents[0].enabled).toBe(true)
      expect(store.agents[1].enabled).toBe(false)
    })

    // 切项目先清空旧数据：新项目加载全部失败时不残留上一个项目的内容
    it('LoadAllSidebarData_ProjectSwitch_ClearsOldData_001', async () => {
      const store = useSidebarStore()
      store.skills = [
        { name: 'old', displayName: 'old', sourceType: 'user', sourceLabel: 'User', invokeFormat: '/old', enabled: true },
      ]
      store.agents = [
        { name: 'old', displayName: 'old', sourceType: 'user', sourceLabel: 'User', invokeFormat: '@"old (agent)"', enabled: true },
      ]
      store.mcpServers = [
        { name: 'old', displayName: 'old', sourceType: 'user', sourceLabel: 'User', prompts: [], enabled: true },
      ]
      store.plugins = [
        { id: 'old@x', name: 'old', version: '1', scope: 'user', enabled: true, installPath: '/x' },
      ]
      ;(store as unknown as { loadedCwd: string }).loadedCwd = '/old-proj'
      mockGetAllSkills.mockRejectedValue(new Error('cli failed'))
      mockGetAllAgents.mockRejectedValue(new Error('cli failed'))
      mockGetAllMcpServers.mockRejectedValue(new Error('cli failed'))
      mockGetAllPlugins.mockRejectedValue(new Error('cli failed'))

      await store.loadAllSidebarData('/new-proj')

      expect(store.skills).toEqual([])
      expect(store.agents).toEqual([])
      expect(store.mcpServers).toEqual([])
      expect(store.plugins).toEqual([])
    })
  })

  // state-changed 权威刷新的 section 分发/级联/暂存逻辑已迁移到状态中心，
  // 对应测试见 tests/state/sections.test.ts
})
