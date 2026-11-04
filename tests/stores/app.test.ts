import { setActivePinia, createPinia } from 'pinia'
import { mockIPC } from '@tauri-apps/api/mocks'
import { beforeEach, describe, it, expect } from 'vitest'
import { useAppStore } from '@/stores/app'
import type { SessionInfo } from '@/types'
import i18n from '@/i18n'

beforeEach(() => {
  setActivePinia(createPinia())
  mockIPC(() => {})
})

describe('getClaudeArgs', () => {
  // 无选项设置时返回空数组
  it('ClaudeArgs_Empty_001', () => {
    const store = useAppStore()
    // default claudeOptions: resume='', skipPermissions=false, customArgs=''
    const args = store.getClaudeArgs()
    expect(args).toEqual([])
  })

  // 设置 resume="abc123" 时返回 ["--resume", "abc123"]
  it('ClaudeArgs_Resume_001', () => {
    const store = useAppStore()
    store.setClaudeOptions({ resume: 'abc123' })
    const args = store.getClaudeArgs()
    expect(args).toEqual(['--resume', 'abc123'])
  })

  // 设置 skipPermissions=true 时包含 "--dangerously-skip-permissions"
  it('ClaudeArgs_SkipPerm_001', () => {
    const store = useAppStore()
    store.setClaudeOptions({ skipPermissions: true })
    const args = store.getClaudeArgs()
    expect(args).toEqual(['--dangerously-skip-permissions'])
  })

  // 设置 customArgs="--flag1 --flag2" 时按空格拆分为两个元素
  it('ClaudeArgs_CustomSplit_001', () => {
    const store = useAppStore()
    store.setClaudeOptions({ customArgs: '--flag1 --flag2' })
    const args = store.getClaudeArgs()
    expect(args).toEqual(['--flag1', '--flag2'])
  })

  // 设置 customArgs="  --flag  " 时过滤空字符串
  it('ClaudeArgs_CustomTrim_001', () => {
    const store = useAppStore()
    store.setClaudeOptions({ customArgs: '  --flag  ' })
    const args = store.getClaudeArgs()
    expect(args).toEqual(['--flag'])
  })

  // 同时设置 resume、skipPermissions、customArgs 时按顺序生成全部参数
  it('ClaudeArgs_Combined_001', () => {
    const store = useAppStore()
    store.setClaudeOptions({
      resume: 'sess1',
      skipPermissions: true,
      customArgs: '--model opus --verbose'
    })
    const args = store.getClaudeArgs()
    expect(args).toEqual([
      '--resume', 'sess1',
      '--dangerously-skip-permissions',
      '--model', 'opus', '--verbose'
    ])
  })
})

describe('resetClaudeOptions', () => {
  // 启动 session 后清空 resume、skipPermissions 恢复默认值
  it('ResetOptions_ResumeAndSkipPerm_001', () => {
    const store = useAppStore()
    store.setClaudeOptions({ resume: 'sess1', skipPermissions: true })
    store.resetClaudeOptions()
    expect(store.claudeOptions.resume).toBe('')
    expect(store.claudeOptions.skipPermissions).toBe(false)
  })

  // 启动 session 后保留用户在侧栏输入的自定义参数，不重置为默认值
  it('ResetOptions_KeepCustomArgs_001', () => {
    const store = useAppStore()
    store.setDefaultClaudeOptions({ customArgs: '--model haiku' })
    store.setClaudeOptions({ customArgs: '--model sonnet --verbose' })
    store.resetClaudeOptions()
    expect(store.claudeOptions.customArgs).toBe('--model sonnet --verbose')
  })

  // 未修改自定义参数时，重置后保持默认值
  it('ResetOptions_CustomArgsDefault_001', () => {
    const store = useAppStore()
    store.setDefaultClaudeOptions({ customArgs: '--model haiku' })
    store.resetClaudeOptions()
    expect(store.claudeOptions.customArgs).toBe('--model haiku')
  })
})

describe('cliHealth', () => {
  // 检查通过（PATH 检测）→ ok + path + source
  it('CliHealth_FromChecks_Path_001', async () => {
    mockIPC((cmd) => {
      if (cmd === 'run_checks') {
        return [{
          name: 'Claude CLI', passed: true, message: 'Found',
          detectedPath: '/opt/homebrew/bin/claude', source: 'path'
        }]
      }
      return null
    })
    const store = useAppStore()
    await store.runChecks(true)
    expect(store.cliHealth).toEqual({
      status: 'ok', path: '/opt/homebrew/bin/claude', source: 'path'
    })
  })

  // 检查失败 → missing + 原始提示 + 结论码（状态条按 code 翻译展示）
  it('CliHealth_FromChecks_Missing_002', async () => {
    mockIPC((cmd) => {
      if (cmd === 'run_checks') {
        return [{ name: 'Claude CLI', passed: false, code: 'checkClaudeNotFound', message: 'not found' }]
      }
      return null
    })
    const store = useAppStore()
    await store.runChecks(true)
    expect(store.cliHealth).toEqual({ status: 'missing', message: 'not found', code: 'checkClaudeNotFound' })
  })

  // 旧版后端无 code → missing 仅带原始提示（展示层回退原文）
  it('CliHealth_FromChecks_MissingLegacy_005', async () => {
    mockIPC((cmd) => {
      if (cmd === 'run_checks') {
        return [{ name: 'Claude CLI', passed: false, message: 'Claude CLI not found.' }]
      }
      return null
    })
    const store = useAppStore()
    await store.runChecks(true)
    expect(store.cliHealth).toEqual({ status: 'missing', message: 'Claude CLI not found.' })
  })

  // 执行失败反哺 → failed；下次 spawn 成功清除
  it('CliHealth_FailureAndClear_003', () => {
    const store = useAppStore()
    store.setCliFailure('EACCES: Permission denied', 'spawn')
    expect(store.cliHealth).toEqual({
      status: 'failed', message: 'EACCES: Permission denied', stage: 'spawn'
    })

    store.clearCliFailure()
    expect(store.cliHealth).toBeNull()
  })

  // spawn 成功也能清除检查失败（missing）留下的提示
  it('CliHealth_ClearMissingOnSpawnSuccess_004', () => {
    const store = useAppStore()
    store.cliHealth = { status: 'missing', message: 'not found' }
    store.clearCliFailure()
    expect(store.cliHealth).toBeNull()
  })
})

describe('setFontSize', () => {
  // 设置 size=5 时钳位到 10
  it('FontSize_MinClamp_001', () => {
    const store = useAppStore()
    store.setFontSize(5)
    expect(store.fontSize).toBe(10)
  })

  // 设置 size=30 时钳位到 24
  it('FontSize_MaxClamp_001', () => {
    const store = useAppStore()
    store.setFontSize(30)
    expect(store.fontSize).toBe(24)
  })

  // 设置 size=14 时直接接受
  it('FontSize_InRange_001', () => {
    const store = useAppStore()
    store.setFontSize(14)
    expect(store.fontSize).toBe(14)
  })
})

describe('currentProject', () => {
  // 设置 cwd="/Users/dev/myproject" 时返回 "myproject"
  it('CurrentProject_ForwardSlash_001', () => {
    const store = useAppStore()
    store.setCwd('/Users/dev/myproject')
    expect(store.currentProject).toBe('myproject')
  })

  // 设置 cwd="C:\\Users\\dev\\myproject" 时返回 "myproject"
  it('CurrentProject_BackSlash_001', () => {
    const store = useAppStore()
    store.setCwd('C:\\Users\\dev\\myproject')
    expect(store.currentProject).toBe('myproject')
  })

  // 设置 cwd="" 时返回 null
  it('CurrentProject_Empty_001', () => {
    const store = useAppStore()
    store.setCwd('')
    expect(store.currentProject).toBeNull()
  })
})

describe('setLanguage', () => {
  // setLanguage('zh') 同时更新 store.language 和 i18n.global.locale
  it('SetLanguage_Zh_UpdatesLocale_001', () => {
    const store = useAppStore()
    store.setLanguage('zh')
    expect(store.language).toBe('zh')
    expect(i18n.global.locale.value).toBe('zh')
  })

  // setLanguage('en') 同时更新 store.language 和 i18n.global.locale
  it('SetLanguage_En_UpdatesLocale_001', () => {
    const store = useAppStore()
    store.setLanguage('en')
    expect(store.language).toBe('en')
    expect(i18n.global.locale.value).toBe('en')
  })

  // 切换后再切换回中文，locale 正确同步
  it('SetLanguage_ToggleTwice_001', () => {
    const store = useAppStore()
    store.setLanguage('zh')
    expect(i18n.global.locale.value).toBe('zh')
    store.setLanguage('en')
    expect(i18n.global.locale.value).toBe('en')
  })
})

describe('reloadRecentSessions', () => {
  const session = (id: string, over: Partial<SessionInfo> = {}): SessionInfo => ({
    sessionId: id,
    name: id,
    projectPath: '/proj/a',
    lastActiveAt: 100,
    ...over,
  })

  // 会话结束后重拉：缓存整体替换为 Rust 单一出口的数据（名称已合并 override），
  // 不再有前端 tab.name 内存合并
  it('ReloadRecentSessions_ReplacesCache_001', async () => {
    mockIPC((cmd) => {
      if (cmd === 'get_all_recent_sessions') {
        return [session('srv-1'), session('srv-2', { name: '官方标题名' })]
      }
    })
    const store = useAppStore()
    store.refreshRecentSessions([session('stale-local')])
    await store.reloadRecentSessions()
    expect(store.cachedRecentSessions.map(s => s.sessionId)).toEqual(['srv-1', 'srv-2'])
    expect(store.cachedRecentSessions[1].name).toBe('官方标题名')
  })

  // 拉取失败静默保留旧缓存（不抛出、不清空）
  it('ReloadRecentSessions_FailureKeepsCache_002', async () => {
    mockIPC(() => {
      throw new Error('backend offline')
    })
    const store = useAppStore()
    store.refreshRecentSessions([session('old-keep')])
    await store.reloadRecentSessions()
    expect(store.cachedRecentSessions.map(s => s.sessionId)).toEqual(['old-keep'])
  })
})

describe('hideProject / unhideProjectIfHidden', () => {
  // 隐藏：乐观移除项目与其近期会话（归一化匹配）+ 调后端命令
  it('HideProject_OptimisticRemove_001', async () => {
    const calls: Array<[string, unknown]> = []
    mockIPC((cmd, args) => {
      calls.push([cmd, args])
      return null
    })
    const store = useAppStore()
    store.cachedProjects = [
      { path: 'D:\\Work\\Demo', name: 'Demo' },
      { path: '/home/user/proj', name: 'proj' },
    ]
    store.refreshRecentSessions([
      { sessionId: 's1', name: 's1', projectPath: 'd:/work/demo', lastActiveAt: 1 },
      { sessionId: 's2', name: 's2', projectPath: '/home/user/proj', lastActiveAt: 2 },
    ] as SessionInfo[])

    await store.hideProject('D:\\Work\\Demo')

    expect(store.cachedProjects.map(p => p.path)).toEqual(['/home/user/proj'])
    expect(store.cachedRecentSessions.map(s => s.sessionId)).toEqual(['s2'])
    expect(store.hiddenProjects).toContain('D:\\Work\\Demo')
    expect(calls.some(([cmd, args]) => cmd === 'hide_project' && (args as { path: string }).path === 'D:\\Work\\Demo')).toBe(true)
  })

  // 未隐藏时打开项目：不发 unhide 命令（避免无谓跨端刷新）
  it('UnhideProjectIfHidden_SkipsWhenNotHidden_002', async () => {
    const calls: string[] = []
    mockIPC((cmd) => {
      calls.push(cmd)
      return null
    })
    const store = useAppStore()
    await store.unhideProjectIfHidden('/not/hidden')
    expect(calls).not.toContain('unhide_project')
  })

  // 已隐藏时打开项目：归一化匹配解除 + 调命令 + 守卫集合移除
  it('UnhideProjectIfHidden_RemovesWhenHidden_003', async () => {
    const calls: Array<[string, unknown]> = []
    mockIPC((cmd, args) => {
      calls.push([cmd, args])
      return null
    })
    const store = useAppStore()
    store.hiddenProjects = ['d:/work/demo']

    await store.unhideProjectIfHidden('D:\\Work\\Demo')

    expect(store.hiddenProjects).toHaveLength(0)
    expect(calls.some(([cmd, args]) => cmd === 'unhide_project' && (args as { path: string }).path === 'D:\\Work\\Demo')).toBe(true)
  })

  // 隐藏项目的本地缓存回填守卫：ensureProjectInList 不把隐藏项目塞回首页
  // 项目列表。近期会话的隐藏过滤已移至服务端（get_all_recent_sessions 只列
  // 已知项目，get_projects 已过滤隐藏）——前端不再本地二次过滤（单一出口）
  it('HiddenProject_LocalCacheBackfillGuard_004', async () => {
    mockIPC(() => null)
    const store = useAppStore()
    store.cachedProjects = []
    store.refreshRecentSessions([])
    store.hiddenProjects = ['D:\\Work\\Demo']

    store.ensureProjectInList('D:\\Work\\Demo')
    store.ensureProjectInList('/home/user/proj')

    expect(store.cachedProjects.map(p => p.path)).toEqual(['/home/user/proj'])
  })
})

describe('resetHomeCache', () => {
  // 浏览器切换设备：项目/近期会话/分页状态全部回到未加载（随后重拉）
  it('App_ResetHomeCache_001', () => {
    const store = useAppStore()
    store.cachedProjects = [{ path: '/a', name: 'a' }]
    store.cachedRecentSessions = [
      { sessionId: 's1', name: 's1', projectPath: '/a', lastActiveAt: 1 },
    ]
    store.cacheLoaded = true
    store.hasMoreProjects = false

    store.resetHomeCache()

    expect(store.cachedProjects).toEqual([])
    expect(store.cachedRecentSessions).toEqual([])
    expect(store.cacheLoaded).toBe(false)
    expect(store.hasMoreProjects).toBe(true)
  })
})

describe('bootstrap 启动顺序', () => {
  // 环境检查失败时 UI 停在检查遮罩；语言必须已从配置加载，否则遮罩按默认 en 渲染
  it('Bootstrap_ConfigBeforeChecks_001', async () => {
    i18n.global.locale.value = 'en'
    const calls: string[] = []
    mockIPC((cmd) => {
      calls.push(cmd)
      if (cmd === 'get_app_config') {
        return { language: 'zh', theme: 'dark', fontSize: 12, claudeEnvVars: { FOO: '1' } }
      }
      if (cmd === 'get_check_results') {
        return [{
          name: 'Claude CLI',
          passed: false,
          code: 'checkClaudeNotFound',
          message: 'Claude CLI not found. You can set a custom path below.',
        }]
      }
      return undefined
    })

    const store = useAppStore()
    await store.bootstrap()

    // 配置必须先于检查：检查失败时 UI 只显示遮罩，之后不会再有加载语言的机会
    expect(calls.indexOf('get_app_config')).toBeLessThan(calls.indexOf('get_check_results'))
    expect(store.checkFailed).toBe(true)
    expect(i18n.global.locale.value).toBe('zh')
    expect(store.theme).toBe('dark')
  })

  // 配置未指定语言时默认中文（不跟随系统 locale）
  it('App_DefaultLanguage_001', async () => {
    i18n.global.locale.value = 'en'
    mockIPC((cmd) => {
      if (cmd === 'get_app_config') return { claudeEnvVars: { FOO: '1' } }
      return undefined
    })

    const store = useAppStore()
    await store.loadAppConfig()

    expect(store.language).toBe('zh')
    expect(i18n.global.locale.value).toBe('zh')
  })
})
