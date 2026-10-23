import { defineStore } from 'pinia'
import { ref, computed } from 'vue'
import { getCurrentWindow } from '@tauri-apps/api/window'
import {
  getAppConfig,
  updateAppConfig,
  saveLastProject,
  saveDefaultClaudeOptions,
  getHomeData,
  getProjects,
  getCheckResults,
  runChecks,
  getSystemLocale,
  hideProject as hideProjectApi,
  unhideProject as unhideProjectApi,
} from '@/api/tauri'
import i18n from '@/i18n'
import { useSidebarStore } from '@/stores/sidebar'

import type { ClaudeOptions, DefaultClaudeOptions, CheckResult, Project, SessionInfo } from '@/types'

const PAGE_SIZE = 12

/** 默认环境变量（代码中定义，用户可重置） */
const DEFAULT_CLAUDE_ENV_VARS: Record<string, string> = {
  LANG: 'en_US.UTF-8',
  LC_ALL: 'en_US.UTF-8',
  PYTHONUTF8: '1',
  CLAUDE_CODE_SCROLL_SPEED: '5',
  PYTHONIOENCODING: 'utf-8',
  CLAUDE_CODE_NO_FLICKER: '1',
}

export { DEFAULT_CLAUDE_ENV_VARS }

export interface PendingResume {
  sessionId: string
  sessionName?: string
}

/**
 * Claude CLI 可用状态（运行态，不落盘）。
 * 唯一状态源：启动检查结果 + pty-error + spawn 成功共同维护。
 */
export type CliHealth =
  | { status: 'ok'; path: string; source: 'configured' | 'path' }
  | { status: 'missing'; message: string }
  | { status: 'failed'; message: string; stage: string }

export const useAppStore = defineStore('app', () => {
  const cwd = ref<string>('')
  const theme = ref<string>('light')
  const fontSize = ref<number>(12)
  const language = ref<string>('en')
  const alwaysOnTop = ref<boolean>(false)
  const claudeEnvVars = ref<Record<string, string>>({})

  // 启动控制
  const pendingResume = ref<PendingResume | null>(null)
  const shouldAutoOpenSessions = ref(false)

  // 环境检查
  const checkResults = ref<CheckResult[]>([])
  const checkFailed = ref(false)
  const cliHealth = ref<CliHealth | null>(null)

  // 缓存：项目列表（分页）和近期会话
  const cachedProjects = ref<Project[]>([])
  const cachedRecentSessions = ref<SessionInfo[]>([])
  const cacheLoaded = ref(false)
  const openedProjectPaths = ref<Set<string>>(new Set())
  const projectsPage = ref(0)
  const hasMoreProjects = ref(true)
  const isLoadingProjects = ref(false)

  // 隐藏项目路径（config.hiddenProjects 镜像；判断"重新添加时解除隐藏"用）
  const hiddenProjects = ref<string[]>([])

  // Claude 默认启动参数（持久化，Settings 绑定）
  const defaultClaudeOptions = ref<DefaultClaudeOptions>({
    skipPermissions: false,
    customArgs: ''
  })

  // Claude 当前使用启动参数（SessionsPanel/ProjectSelectView 绑定）
  const claudeOptions = ref<ClaudeOptions>({
    resume: '',
    skipPermissions: false,
    customArgs: ''
  })

  const currentProject = computed(() => {
    if (!cwd.value) return null
    const parts = cwd.value.replace(/\\/g, '/').split('/')
    return parts[parts.length - 1] || cwd.value
  })

  const failedChecks = computed(() => checkResults.value.filter(c => !c.passed))

  async function loadAppConfig() {
    try {
      const config = await getAppConfig()
      theme.value = config.theme || 'light'
      fontSize.value = config.fontSize || 12
      language.value = config.language || (await detectSystemLocale())
      i18n.global.locale.value = language.value
      hiddenProjects.value = config.hiddenProjects ?? []

      // 加载环境变量：非首次纯读；仅配置为空（首次使用）时持久化默认值。
      // 不能每次加载都写回（doSyncEnv）——写命令会广播 state-changed(config)，
      // 各端事件驱动刷新又调 loadAppConfig，形成 写→广播→读→写 死循环
      // （风暴塞满通道后 RpcResult 丢失、且页面加载即抢全局控制权）
      if (Object.keys(config.claudeEnvVars ?? {}).length > 0) {
        claudeEnvVars.value = config.claudeEnvVars!
      } else {
        claudeEnvVars.value = { ...DEFAULT_CLAUDE_ENV_VARS }
        await doSyncEnv()
      }

      defaultClaudeOptions.value = {
        skipPermissions: config.defaultSkipPermissions ?? false,
        customArgs: config.defaultCustomArgs ?? ''
      }
      claudeOptions.value = {
        resume: '',
        skipPermissions: defaultClaudeOptions.value.skipPermissions,
        customArgs: defaultClaudeOptions.value.customArgs
      }
    } catch (err) {
      console.error('Failed to load app config:', err)
    }
  }

  async function doChecks(force = false) {
    try {
      checkResults.value = force ? await runChecks() : await getCheckResults()
      checkFailed.value = checkResults.value.some(c => !c.passed)
      syncCliHealthFromChecks(checkResults.value)
    } catch (err) {
      console.error('Failed to run checks:', err)
    }
  }

  /** 检查结果 → CLI 运行态（检查与执行共用的唯一状态源） */
  function syncCliHealthFromChecks(results: CheckResult[]) {
    const cli = results.find(c => c.name === 'Claude CLI')
    if (!cli) return
    cliHealth.value = cli.passed
      ? {
          status: 'ok',
          path: cli.detectedPath ?? '',
          source: cli.source === 'configured' ? 'configured' : 'path'
        }
      : { status: 'missing', message: cli.message }
  }

  /** spawn 失败反哺：CLI 不可用（执行与检查结论不一致时以执行结果为准） */
  function setCliFailure(message: string, stage: string) {
    cliHealth.value = { status: 'failed', message, stage }
  }

  /** spawn 成功：CLI 已被证明可用，清除所有非 ok 的旧提示 */
  function clearCliFailure() {
    if (cliHealth.value && cliHealth.value.status !== 'ok') cliHealth.value = null
  }

  async function loadCache() {
    if (cacheLoaded.value) return
    try {
      const data = await getHomeData(PAGE_SIZE, 20)
      cachedProjects.value = data.projects
      cachedRecentSessions.value = data.recentSessions
      projectsPage.value = 1
      hasMoreProjects.value = data.hasMore
      cacheLoaded.value = true
    } catch (err) {
      console.error('Failed to load cache:', err)
    }
  }

  async function loadMoreProjects() {
    if (isLoadingProjects.value || !hasMoreProjects.value) return
    isLoadingProjects.value = true
    try {
      const offset = projectsPage.value * PAGE_SIZE
      const projs = await getProjects(PAGE_SIZE, offset)
      cachedProjects.value.push(...projs)
      projectsPage.value++
      hasMoreProjects.value = projs.length === PAGE_SIZE
    } catch (err) {
      console.error('Failed to load more projects:', err)
    } finally {
      isLoadingProjects.value = false
    }
  }

  async function refreshCache() {
    cacheLoaded.value = false
    await loadCache()
  }

  /** 路径归一化（与后端 normalize_project_key 一致：Windows 大小写不敏感） */
  function normalizeProjectPath(p: string): string {
    return p.replace(/\\/g, '/').replace(/\/+$/, '').toLowerCase()
  }

  /** 隐藏项目：乐观从首页缓存移除（条目原地消失）；跨端经
   *  state-changed(projects) 权威刷新，本端写登记抑制回环整列表重建 */
  async function hideProject(path: string) {
    const key = normalizeProjectPath(path)
    cachedProjects.value = cachedProjects.value.filter(p => normalizeProjectPath(p.path) !== key)
    cachedRecentSessions.value = cachedRecentSessions.value.filter(s => normalizeProjectPath(s.projectPath) !== key)
    if (!hiddenProjects.value.some(p => normalizeProjectPath(p) === key)) {
      hiddenProjects.value = [...hiddenProjects.value, path]
    }
    useSidebarStore().markSelfWrite('projects')
    try {
      await hideProjectApi(path)
    } catch (err) {
      console.error('Failed to hide project:', err)
      // 失败回滚：撤销登记（无对应广播，防误吞其它端事件）+ 以服务端状态为准重拉
      useSidebarStore().cancelSelfWrite('projects')
      void syncHiddenProjects()
      void refreshCache()
    }
  }

  /** 重新打开项目时解除隐藏（未隐藏时不发写命令，避免无谓的跨端刷新） */
  async function unhideProjectIfHidden(path: string) {
    const key = normalizeProjectPath(path)
    if (!hiddenProjects.value.some(p => normalizeProjectPath(p) === key)) return
    hiddenProjects.value = hiddenProjects.value.filter(p => normalizeProjectPath(p) !== key)
    useSidebarStore().markSelfWrite('projects')
    try {
      await unhideProjectApi(path)
    } catch (err) {
      console.error('Failed to unhide project:', err)
      useSidebarStore().cancelSelfWrite('projects')
      void syncHiddenProjects()
    }
  }

  /** 仅同步 hiddenProjects 镜像（projects section 跨端刷新时补拉，不动其余配置态） */
  async function syncHiddenProjects() {
    try {
      hiddenProjects.value = (await getAppConfig()).hiddenProjects ?? []
    } catch {
      // 读配置失败保持现状
    }
  }

  /** 路径是否在隐藏清单中（归一化比对；乐观更新与本地缓存回填的统一守卫） */
  function isPathHidden(path: string): boolean {
    return hiddenProjects.value.some(p => normalizeProjectPath(p) === normalizeProjectPath(path))
  }

  function ensureProjectInList(projectPath: string) {
    // 隐藏项目不回填首页（运行中 Tab 的会话启动路径可能触发此处）
    if (isPathHidden(projectPath)) return
    const normalize = (p: string) => p.replace(/\\/g, '/').toLowerCase()
    const normalized = normalize(projectPath)
    if (cachedProjects.value.some(p => normalize(p.path) === normalized)) return

    const parts = projectPath.replace(/\\/g, '/').split('/')
    cachedProjects.value.unshift({
      path: projectPath,
      name: parts[parts.length - 1] || projectPath,
      lastDuration: Date.now(),
    })
  }

  /** 检查路径是否为已知项目（归一化后匹配 cachedProjects） */
  function isKnownProject(projectPath: string): boolean {
    const normalize = (p: string) => p.replace(/\\/g, '/').toLowerCase()
    const normalized = normalize(projectPath)
    return cachedProjects.value.some(p => normalize(p.path) === normalized)
  }

  function refreshRecentSessions(sessions: SessionInfo[]) {
    cachedRecentSessions.value = sessions
  }

  /** 就地合并一条近期会话（按 sessionId 去重）：会话关闭/自然结束时把
   *  Tab 上已有的信息（sessionId/名称/项目路径）直接并入缓存，首页
   *  「近期会话」零请求即时可见——比重拉 getHomeData 高效，且不重置
   *  项目列表分页状态。缓存条目代表已结束的会话，working/pending 清 false */
  function mergeRecentSession(session: SessionInfo) {
    // 隐藏项目的会话不并入近期会话（Tab 关闭路径；否则隐藏被本地缓存绕过）
    if (isPathHidden(session.projectPath)) return
    const existing = cachedRecentSessions.value.find(s => s.sessionId === session.sessionId)
    if (existing) {
      Object.assign(existing, session, { working: false, pending: false })
    } else {
      cachedRecentSessions.value.unshift({ ...session, working: false, pending: false })
    }
  }

  function setCwd(path: string) {
    cwd.value = path
    openedProjectPaths.value.add(path)
    saveLastProject(path)
  }

  function setTheme(newTheme: string) {
    theme.value = newTheme
    // 同步应用到 DOM
    const html = document.documentElement
    html.setAttribute('data-theme', newTheme)
    html.classList.remove('light', 'dark')
    html.classList.add(newTheme)
    // 持久化
    updateAppConfig({ theme: newTheme })
  }

  function setFontSize(size: number) {
    fontSize.value = Math.max(10, Math.min(24, size))
    updateAppConfig({ fontSize: size })
  }

  /** 通过 Tauri 命令获取系统 locale，回退到 navigator.language */
  async function detectSystemLocale(): Promise<'en' | 'zh'> {
    try {
      const locale = await getSystemLocale()
      if (locale && locale.toLowerCase().startsWith('zh')) return 'zh'
      if (locale) return 'en'
    } catch {
      // 命令失败时回退到浏览器语言
    }
    const browserLang = navigator.language || 'en'
    return browserLang.toLowerCase().startsWith('zh') ? 'zh' : 'en'
  }

  function setLanguage(lang: string) {
    language.value = lang
    updateAppConfig({ language: lang })
    i18n.global.locale.value = lang
  }

  /** 同步当前 claudeEnvVars 到 cc-box config */
  async function doSyncEnv() {
    await updateAppConfig({ claudeEnvVars: claudeEnvVars.value })
  }

  /** 更新环境变量并同步 */
  async function setClaudeEnvVars(vars: Record<string, string>) {
    claudeEnvVars.value = vars
    await doSyncEnv()
  }

  /** 将默认变量恢复为代码默认值，保留用户添加的变量 */
  async function resetClaudeEnvVars() {
    const updated = { ...claudeEnvVars.value }
    for (const [key, value] of Object.entries(DEFAULT_CLAUDE_ENV_VARS)) {
      updated[key] = value
    }
    claudeEnvVars.value = updated
    await doSyncEnv()
  }

  function setClaudeOptions(options: Partial<ClaudeOptions>) {
    claudeOptions.value = { ...claudeOptions.value, ...options }
  }

  function resetClaudeOptions() {
    claudeOptions.value = {
      resume: '',
      skipPermissions: defaultClaudeOptions.value.skipPermissions,
      // 自定义参数是用户的持续意图（如 --model sonnet），启动 session 后保留
      customArgs: claudeOptions.value.customArgs
    }
  }

  async function setDefaultClaudeOptions(opts: Partial<DefaultClaudeOptions>) {
    defaultClaudeOptions.value = { ...defaultClaudeOptions.value, ...opts }
    claudeOptions.value = {
      resume: claudeOptions.value.resume,
      skipPermissions: defaultClaudeOptions.value.skipPermissions,
      customArgs: defaultClaudeOptions.value.customArgs
    }
    await saveDefaultClaudeOptions(defaultClaudeOptions.value)
  }

  async function saveAsDefault(): Promise<boolean> {
    try {
      const opts = {
        skipPermissions: claudeOptions.value.skipPermissions,
        customArgs: claudeOptions.value.customArgs
      }
      defaultClaudeOptions.value = opts
      await saveDefaultClaudeOptions(opts)
      return true
    } catch (err) {
      console.error('Failed to save default options:', err)
      return false
    }
  }

  function getClaudeArgs(): string[] {
    const opts = claudeOptions.value
    const args: string[] = []

    if (opts.resume) args.push('--resume', opts.resume)
    if (opts.skipPermissions) args.push('--dangerously-skip-permissions')
    if (opts.customArgs) {
      const custom = opts.customArgs.trim().split(/\s+/).filter(Boolean)
      args.push(...custom)
    }

    return args
  }

  function setPendingResume(sessionId: string, sessionName?: string) {
    pendingResume.value = { sessionId, sessionName }
  }

  function clearPendingResume() {
    pendingResume.value = null
  }

  function setAutoOpenSessions(val: boolean) {
    shouldAutoOpenSessions.value = val
  }

  async function toggleAlwaysOnTop() {
    try {
      const win = getCurrentWindow()
      const newState = !alwaysOnTop.value
      await win.setAlwaysOnTop(newState)
      alwaysOnTop.value = newState
    } catch (err) {
      console.error('Failed to toggle always on top:', err)
    }
  }

  return {
    cwd,
    theme,
    fontSize,
    language,
    claudeEnvVars,
    defaultClaudeOptions,
    claudeOptions,
    currentProject,
    pendingResume,
    shouldAutoOpenSessions,
    checkResults,
    checkFailed,
    failedChecks,
    cliHealth,
    setCliFailure,
    clearCliFailure,
    cachedProjects,
    cachedRecentSessions,
    cacheLoaded,
    openedProjectPaths,
    hasMoreProjects,
    isLoadingProjects,
    hiddenProjects,
    loadAppConfig,
    runChecks: doChecks,
    loadCache,
    loadMoreProjects,
    refreshCache,
    hideProject,
    unhideProjectIfHidden,
    syncHiddenProjects,
    isPathHidden,
    mergeRecentSession,
    ensureProjectInList,
    isKnownProject,
    refreshRecentSessions,
    setCwd,
    setTheme,
    setFontSize,
    setLanguage,
    setClaudeEnvVars,
    resetClaudeEnvVars,
    setClaudeOptions,
    setDefaultClaudeOptions,
    resetClaudeOptions,
    saveAsDefault,
    getClaudeArgs,
    setPendingResume,
    clearPendingResume,
    setAutoOpenSessions,
    alwaysOnTop,
    toggleAlwaysOnTop
  }
})
