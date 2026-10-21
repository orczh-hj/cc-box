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

vi.mock('@/api/tauri', () => ({
  getAllSkills: (...args: unknown[]) => mockGetAllSkills(...args),
  getAllAgents: (...args: unknown[]) => mockGetAllAgents(...args),
  getAllMcpServers: (...args: unknown[]) => mockGetAllMcpServers(...args),
  getAllPlugins: (...args: unknown[]) => mockGetAllPlugins(...args),
  setSkillEnabled: (...args: unknown[]) => mockSetSkillEnabled(...args),
  setAgentEnabled: (...args: unknown[]) => mockSetAgentEnabled(...args),
  setMcpServerEnabled: (...args: unknown[]) => mockSetMcpServerEnabled(...args),
  setPluginEnabled: (...args: unknown[]) => mockSetPluginEnabled(...args),
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

      expect(mockSetSkillEnabled).toHaveBeenCalledWith('deploy', false)
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

      expect(mockSetSkillEnabled).toHaveBeenCalledWith('ghost', false)
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

  // state-changed 权威刷新的 section 分发/级联/暂存逻辑已迁移到状态中心，
  // 对应测试见 tests/state/sections.test.ts
})
