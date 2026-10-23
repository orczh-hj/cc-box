/**
 * 状态同步中心：state-changed section → 数据加载器 注册表
 *
 * 后端写命令落盘后广播 state-changed(section)（见 src-tauri rpc_dispatch 的
 * registry! 与 #[cc_command] 宏），本中心是前端的统一消费端：
 * - **查表分发**：App.vue 只订阅事件并调 applyStateSection(id)，section 与
 *   loader 的映射集中在此登记——新功能加一行，不再改 App.vue/各 store
 * - **暂存防覆盖**：deferWhileSettingsOpen 的 section（providers/config）
 *   在设置面板打开期间暂存，面板关闭后补刷（防覆盖编辑中表单）
 * - **级联刷新**：alsoLoad 声明连带 section（plugins 开关影响其
 *   skills/agents/mcp 子项展示）
 *
 * 时序：登记表是模块级 Map（构建 import 即完成，先于任何事件订阅）；
 * loader 内部才 useStore()（Pinia 惰性实例化，注册时零副作用）。
 */
import { useSidebarStore } from '@/stores/sidebar'
import { useAppStore } from '@/stores/app'
import { useProvidersStore } from '@/stores/providers'

interface SectionLoader {
  /** 权威重拉该 section 数据（签名不强制统一，内部自行适配参数/去重） */
  load: () => Promise<void> | void
  /** 设置面板打开期间暂存、面板关闭后补刷（防覆盖编辑中表单） */
  deferWhileSettingsOpen?: boolean
  /** 级联刷新的连带 section（本 section 变更影响其展示） */
  alsoLoad?: string[]
}

const registry = new Map<string, SectionLoader>()
const pendingSections = new Set<string>()

/** 登记 section 加载器（重复登记覆盖——测试/热替换用） */
export function registerSectionLoader(section: string, loader: SectionLoader): void {
  registry.set(section, loader)
}

/** 已登记的全部 section（state-resync-needed 全量重拉遍历用） */
export function registeredSections(): string[] {
  return [...registry.keys()]
}

/** 设置面板是否打开（defer 判定用；面板状态单点在 sidebar store） */
function settingsOpen(): boolean {
  return useSidebarStore().showSettings
}

/**
 * 按 section 权威刷新（App.vue 的 state-changed 订阅回调）。
 * 未登记的 section 静默忽略（后端新增 section 前端未升级时不出错）。
 */
export function applyStateSection(
  section: string,
  opts?: { resync?: boolean },
): void {
  const loader = registry.get(section)
  if (!loader) return
  if (opts?.resync) {
    // 权威全量重拉是兜底路径：丢弃本端写登记，确保不被回环抑制吞掉
    useSidebarStore().clearSelfWrites(section)
  } else if (useSidebarStore().consumeSelfWrite(section)) {
    // 本端写回环：发起方已乐观更新，跳过该 section 的刷新（条目原地响应、
    // 列表顺序不变）；级联 section 仍照常刷新（plugin 开关影响子项展示）。
    for (const dep of loader.alsoLoad ?? []) {
      void registry.get(dep)?.load()
    }
    return
  }
  if (loader.deferWhileSettingsOpen && settingsOpen()) {
    pendingSections.add(section)
    return
  }
  void loader.load()
  // 级联：连带 section 同批刷新（不再走 alsoLoad 的 defer 语义——主 section
  // 已判定可刷，连带数据同属一个权威状态）
  for (const dep of loader.alsoLoad ?? []) {
    void registry.get(dep)?.load()
  }
}

/** 补刷设置面板打开期间暂存的 section（App.vue watch 面板关闭时调用） */
export function flushPendingSections(): void {
  if (pendingSections.size === 0) return
  const sections = [...pendingSections]
  pendingSections.clear()
  for (const s of sections) {
    void registry.get(s)?.load()
  }
}

// ===== section 登记（新功能在此加一行） =====

/** sidebar 系 loader 共享守卫：未加载过项目（浏览器端尚未打开项目）时无数据可刷 */
function withLoadedCwd(reload: (cwd: string) => Promise<void>) {
  return () => {
    const sidebar = useSidebarStore()
    if (!sidebar.loadedCwd) return
    return reload(sidebar.loadedCwd)
  }
}

// sidebar 系一律静默刷新：保留旧数据与 loading 态，避免面板卸载重建
// （展开态/滚动位置丢失）；首屏与切项目仍走非 silent 加载
registerSectionLoader('skills', {
  load: withLoadedCwd((cwd) => useSidebarStore().loadSkills(cwd, { silent: true })),
})
registerSectionLoader('agents', {
  load: withLoadedCwd((cwd) => useSidebarStore().loadAgents(cwd, { silent: true })),
})
registerSectionLoader('mcp', {
  load: withLoadedCwd((cwd) => useSidebarStore().loadMcpServers(cwd, { silent: true })),
})
// plugin 开关影响子项展示（后端按 plugin.enabled 过滤），连带刷新
registerSectionLoader('plugins', {
  load: withLoadedCwd((cwd) => useSidebarStore().loadPlugins(cwd, { silent: true })),
  alsoLoad: ['skills', 'agents', 'mcp'],
})
registerSectionLoader('providers', {
  load: () => useProvidersStore().loadProvidersConfig(),
  deferWhileSettingsOpen: true,
})
registerSectionLoader('config', {
  load: () => useAppStore().loadAppConfig(),
  deferWhileSettingsOpen: true,
})
// 隐藏/恢复项目（hide_project/unhide_project）：首页项目列表与近期会话权威
// 重拉；hiddenProjects 存于 config，一并静默同步守卫集合（不走 config loader
// ——loadAppConfig 会重置 claudeOptions 等编辑中状态）
registerSectionLoader('projects', {
  load: async () => {
    await useAppStore().refreshCache()
    void useAppStore().syncHiddenProjects()
  },
})
