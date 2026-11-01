/**
 * 插件提供的 hooks 行（Hook 面板末尾只读分组的数据源）。
 *
 * 被禁用的插件不贡献任何子项——与 skills / agents / mcp 的后端过滤保持一致：
 * 插件开关关闭后，它的 hook 也不应继续出现在侧边栏。
 */
import type { PluginInfo } from '@/types/config'

export interface PluginHookRow {
  plugin: string
  event: string
  matcher?: string
  type?: string
  summary?: string
  timeout?: unknown
}

/** 收集 + 稳定排序：先插件名，再 hook 类型，再事件名 */
export function collectPluginHookRows(plugins: readonly PluginInfo[]): PluginHookRow[] {
  const rows: PluginHookRow[] = []
  for (const p of plugins) {
    if (!p.enabled) continue
    for (const h of p.hooks ?? []) {
      rows.push({
        plugin: p.name,
        event: h.event,
        matcher: h.matcher,
        type: h.type,
        summary: h.summary,
        timeout: h.timeout,
      })
    }
  }
  return rows.sort(
    (a, b) =>
      a.plugin.localeCompare(b.plugin) ||
      (a.type ?? '').localeCompare(b.type ?? '') ||
      a.event.localeCompare(b.event)
  )
}
