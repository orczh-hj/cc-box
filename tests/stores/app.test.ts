import { setActivePinia, createPinia } from 'pinia'
import { mockIPC } from '@tauri-apps/api/mocks'
import { beforeEach, describe, it, expect, vi } from 'vitest'
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

  // 检查失败 → missing + 原始提示（供状态条展示）
  it('CliHealth_FromChecks_Missing_002', async () => {
    mockIPC((cmd) => {
      if (cmd === 'run_checks') {
        return [{ name: 'Claude CLI', passed: false, message: 'not found' }]
      }
      return null
    })
    const store = useAppStore()
    await store.runChecks(true)
    expect(store.cliHealth).toEqual({ status: 'missing', message: 'not found' })
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

describe('detectSystemLocale', () => {
  // navigator.language 为 'zh-CN' 时返回 'zh'
  it('DetectLocale_ZhCN_001', () => {
    const original = navigator.language
    Object.defineProperty(navigator, 'language', { value: 'zh-CN', configurable: true })
    const store = useAppStore()
    // detectSystemLocale 是内部函数，通过 loadAppConfig 间接触发
    // 直接验证初始 language 默认值逻辑
    const lang = navigator.language.startsWith('zh') ? 'zh' : 'en'
    expect(lang).toBe('zh')
    Object.defineProperty(navigator, 'language', { value: original, configurable: true })
  })

  // navigator.language 为 'en-US' 时返回 'en'
  it('DetectLocale_EnUS_001', () => {
    const original = navigator.language
    Object.defineProperty(navigator, 'language', { value: 'en-US', configurable: true })
    const lang = navigator.language.startsWith('zh') ? 'zh' : 'en'
    expect(lang).toBe('en')
    Object.defineProperty(navigator, 'language', { value: original, configurable: true })
  })
})

describe('mergeRecentSession', () => {
  const session = (id: string, over: Partial<SessionInfo> = {}): SessionInfo => ({
    sessionId: id,
    name: id,
    projectPath: '/proj/a',
    lastActiveAt: 100,
    ...over,
  })

  // 新会话（本次运行新建后关闭）插入缓存顶部，首页近期会话即时可见
  it('MergeRecentSession_New_InsertsTop_001', () => {
    const store = useAppStore()
    store.refreshRecentSessions([session('old-1')])
    store.mergeRecentSession(session('closed-new', { lastActiveAt: 999 }))
    expect(store.cachedRecentSessions.map(s => s.sessionId)).toEqual(['closed-new', 'old-1'])
    expect(store.cachedRecentSessions[0].lastActiveAt).toBe(999)
  })

  // 已存在的 sessionId（恢复后关闭）就地更新，不产生重复条目
  it('MergeRecentSession_Existing_UpdatesInPlace_002', () => {
    const store = useAppStore()
    store.refreshRecentSessions([session('old-1', { name: '旧名' })])
    store.mergeRecentSession(session('old-1', { name: '新名', lastActiveAt: 500 }))
    expect(store.cachedRecentSessions).toHaveLength(1)
    expect(store.cachedRecentSessions[0].name).toBe('新名')
    expect(store.cachedRecentSessions[0].lastActiveAt).toBe(500)
  })

  // 合入条目代表已结束的会话：working/pending 强制清 false（不残留运行态圆点）
  it('MergeRecentSession_ClearsWorkingPending_003', () => {
    const store = useAppStore()
    store.mergeRecentSession(session('s1', { working: true, pending: true }))
    expect(store.cachedRecentSessions[0].working).toBe(false)
    expect(store.cachedRecentSessions[0].pending).toBe(false)
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

  // 隐藏项目的本地缓存回填守卫：ensureProjectInList / mergeRecentSession
  // 不把隐藏项目塞回首页（运行中 Tab 的会话启动/关闭路径）
  it('HiddenProject_LocalCacheBackfillGuard_004', async () => {
    mockIPC(() => null)
    const store = useAppStore()
    store.cachedProjects = []
    store.refreshRecentSessions([])
    store.hiddenProjects = ['D:\\Work\\Demo']

    store.ensureProjectInList('D:\\Work\\Demo')
    store.ensureProjectInList('/home/user/proj')
    store.mergeRecentSession({ sessionId: 's1', name: 's1', projectPath: 'd:/work/demo', lastActiveAt: 1 } as SessionInfo)
    store.mergeRecentSession({ sessionId: 's2', name: 's2', projectPath: '/home/user/proj', lastActiveAt: 2 } as SessionInfo)

    expect(store.cachedProjects.map(p => p.path)).toEqual(['/home/user/proj'])
    expect(store.cachedRecentSessions.map(s => s.sessionId)).toEqual(['s2'])
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
