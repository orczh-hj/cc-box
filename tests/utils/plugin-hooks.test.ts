import { describe, expect, it } from 'vitest'
import { collectPluginHookRows } from '@/utils/plugin-hooks'
import type { PluginInfo } from '@/types/config'

function plugin(name: string, enabled: boolean, hooks: { event: string; type?: string }[]): PluginInfo {
  return {
    id: `${name}@x`,
    name,
    version: '1.0',
    scope: 'user',
    enabled,
    installPath: `/p/${name}`,
    hooks: hooks as PluginInfo['hooks'],
  }
}

describe('collectPluginHookRows', () => {
  // 被禁用的插件，其 hooks 不应进入侧边栏展示
  it('PluginHooks_SkipsDisabledPlugin_1', () => {
    const rows = collectPluginHookRows([
      plugin('on', true, [{ event: 'Stop', type: 'command' }]),
      plugin('off', false, [{ event: 'PreToolUse', type: 'command' }, { event: 'Stop' }]),
    ])
    expect(rows.map((r) => r.event)).toEqual(['Stop'])
    expect(rows.every((r) => r.plugin === 'on')).toBe(true)
  })

  // 排序稳定：先插件名，再类型，再事件
  it('PluginHooks_SortStable_2', () => {
    const rows = collectPluginHookRows([
      plugin('b', true, [{ event: 'Stop', type: 'http' }]),
      plugin('a', true, [{ event: 'Stop', type: 'http' }, { event: 'Notice', type: 'command' }]),
    ])
    expect(rows.map((r) => `${r.plugin}/${r.type}/${r.event}`)).toEqual([
      'a/command/Notice',
      'a/http/Stop',
      'b/http/Stop',
    ])
  })
})
