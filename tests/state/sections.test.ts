import { describe, it, expect, beforeEach, vi } from 'vitest'
import { setActivePinia, createPinia } from 'pinia'
import { mockIPC } from '@tauri-apps/api/mocks'

// Mock 状态中心 loader 链路上的数据 API（import 部分覆盖，保留其余真实导出）
const mockGetAllSkills = vi.fn()
const mockGetAllAgents = vi.fn()
const mockGetAllMcpServers = vi.fn()
const mockGetAllPlugins = vi.fn()
const mockGetAppConfig = vi.fn()
const mockGetProvidersConfig = vi.fn()
const mockCheckCcSwitchDbExists = vi.fn()
const mockGetSystemLocale = vi.fn()

vi.mock('@/api/tauri', async (importOriginal) => {
  const actual = await importOriginal<typeof import('@/api/tauri')>()
  return {
    ...actual,
    getAllSkills: (...args: unknown[]) => mockGetAllSkills(...args),
    getAllAgents: (...args: unknown[]) => mockGetAllAgents(...args),
    getAllMcpServers: (...args: unknown[]) => mockGetAllMcpServers(...args),
    getAllPlugins: (...args: unknown[]) => mockGetAllPlugins(...args),
    getAppConfig: (...args: unknown[]) => mockGetAppConfig(...args),
    getSystemLocale: (...args: unknown[]) => mockGetSystemLocale(...args),
  }
})

// providers store 走 @/api/provider 封装（非 @/api/tauri）
vi.mock('@/api/provider', async (importOriginal) => {
  const actual = await importOriginal<typeof import('@/api/provider')>()
  return {
    ...actual,
    getProvidersConfig: (...args: unknown[]) => mockGetProvidersConfig(...args),
    checkCcSwitchDbExists: (...args: unknown[]) => mockCheckCcSwitchDbExists(...args),
  }
})

import { useSidebarStore } from '@/stores/sidebar'
import { applyStateSection, flushPendingSections, registeredSections } from '@/state/sections'

describe('状态同步中心（sections registry）', () => {
  beforeEach(() => {
    setActivePinia(createPinia())
    mockIPC(() => {})
    vi.clearAllMocks()
    // 清空面板状态与暂存（defer 测试间不串扰）
    const sidebar = useSidebarStore()
    sidebar.closeSettings()
    flushPendingSections()
    mockGetAllSkills.mockResolvedValue([])
    mockGetAllAgents.mockResolvedValue([])
    mockGetAllMcpServers.mockResolvedValue([])
    mockGetAllPlugins.mockResolvedValue([])
    mockGetAppConfig.mockResolvedValue({ claudeEnvVars: { X: '1' } })
    mockGetProvidersConfig.mockResolvedValue({ providers: [], commonConfig: null, activeProviderId: null })
    mockCheckCcSwitchDbExists.mockResolvedValue(true)
    mockGetSystemLocale.mockResolvedValue('zh-CN')
  })

  // 各 sidebar section 查表分发：重拉对应数据（state-changed 回环/广播到达）
  it('ApplyStateSection_LoadsCorrespondingData_001', async () => {
    const sidebar = useSidebarStore()
    ;(sidebar as unknown as { loadedCwd: string }).loadedCwd = '/proj'

    applyStateSection('skills')
    applyStateSection('agents')
    applyStateSection('mcp')
    await new Promise(r => setTimeout(r, 0))

    expect(mockGetAllSkills).toHaveBeenCalledWith('/proj')
    expect(mockGetAllAgents).toHaveBeenCalledWith('/proj')
    expect(mockGetAllMcpServers).toHaveBeenCalledWith('/proj')
    expect(mockGetAllPlugins).not.toHaveBeenCalled()
  })

  // providers/config 查表分发到对应 store loader
  it('ApplyStateSection_StoresDistributed_001', async () => {
    applyStateSection('providers')
    applyStateSection('config')
    await new Promise(r => setTimeout(r, 0))

    expect(mockGetProvidersConfig).toHaveBeenCalledTimes(1)
    expect(mockGetAppConfig).toHaveBeenCalledTimes(1)
  })

  // plugins 级联刷新子项（后端按 plugin.enabled 过滤 skills/agents/mcp）
  it('ApplyStateSection_PluginsReloadsChildren_001', async () => {
    const sidebar = useSidebarStore()
    ;(sidebar as unknown as { loadedCwd: string }).loadedCwd = '/proj'

    applyStateSection('plugins')
    await new Promise(r => setTimeout(r, 0))

    expect(mockGetAllPlugins).toHaveBeenCalledWith('/proj')
    expect(mockGetAllSkills).toHaveBeenCalledWith('/proj')
    expect(mockGetAllAgents).toHaveBeenCalledWith('/proj')
    expect(mockGetAllMcpServers).toHaveBeenCalledWith('/proj')
  })

  // loadedCwd 为空（浏览器端未打开项目）时 sidebar 系无数据可刷，跳过
  it('ApplyStateSection_SkipsWhenNoCwd_001', async () => {
    applyStateSection('skills')
    await new Promise(r => setTimeout(r, 0))

    expect(mockGetAllSkills).not.toHaveBeenCalled()
  })

  // 设置面板打开期间 defer section 暂存（防覆盖编辑中表单），关闭后补刷
  it('ApplyStateSection_DeferredWhileSettingsOpen_001', async () => {
    const sidebar = useSidebarStore()
    sidebar.openSettings()

    applyStateSection('config')
    applyStateSection('providers')
    await new Promise(r => setTimeout(r, 0))
    expect(mockGetAppConfig).not.toHaveBeenCalled()
    expect(mockGetProvidersConfig).not.toHaveBeenCalled()

    sidebar.closeSettings()
    flushPendingSections()
    await new Promise(r => setTimeout(r, 0))
    expect(mockGetAppConfig).toHaveBeenCalledTimes(1)
    expect(mockGetProvidersConfig).toHaveBeenCalledTimes(1)

    // 补刷只执行一次，再次 flush 不重复
    flushPendingSections()
    await new Promise(r => setTimeout(r, 0))
    expect(mockGetAppConfig).toHaveBeenCalledTimes(1)
  })

  // 未登记的 section 静默忽略（后端新增 section 前端未升级时不出错）
  it('ApplyStateSection_IgnoresUnknown_001', () => {
    expect(() => applyStateSection('no-such-section')).not.toThrow()
    expect(mockGetAllSkills).not.toHaveBeenCalled()
  })

  // 注册表覆盖全部已知 section（state-resync-needed 全量重拉的遍历范围）
  it('RegisteredSections_全部登记_001', () => {
    const sections = registeredSections()
    for (const s of ['skills', 'agents', 'mcp', 'plugins', 'providers', 'config']) {
      expect(sections).toContain(s)
    }
  })
})
