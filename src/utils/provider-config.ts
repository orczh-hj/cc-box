import { deepMergeSettings } from './json'
import type { CommonConfig, ProviderMeta } from '@/types/provider'

/**
 * Provider 配置分层合成（与 Rust src-tauri/src/providers.rs 同一语义）。
 *
 * 层级（高 → 低）：Provider 覆盖层 > 通用配置默认层 > 用户 settings.json 现状。
 * 通用配置不再物化进 Provider；编辑面板展示 effective，保存时存回覆盖层。
 * 改动本文件时必须同步修改 Rust 实现与 tests/fixtures/provider-merge/cases.json。
 */

/** Claude Code 自管状态字段：不参与 Provider/通用配置，激活时由用户现状保留 */
export const CC_STATE_KEYS = [
  'enabledPlugins',
  'disabledPlugins',
  'extraKnownMarketplaces',
  'hooks',
] as const

/** Provider 专属的核心 env：空字符串视为「未设置」，不得覆盖下层/用户现有值 */
export const CORE_ENV_KEYS = [
  'ANTHROPIC_AUTH_TOKEN',
  'ANTHROPIC_BASE_URL',
  'ANTHROPIC_DEFAULT_HAIKU_MODEL',
  'ANTHROPIC_DEFAULT_OPUS_MODEL',
  'ANTHROPIC_DEFAULT_SONNET_MODEL',
  'ANTHROPIC_MODEL',
] as const

export function isPlainObject(value: unknown): value is Record<string, any> {
  return value !== null && typeof value === 'object' && !Array.isArray(value)
}

export function cloneJson<T>(value: T): T {
  return value === undefined ? value : (JSON.parse(JSON.stringify(value)) as T)
}

export function stripCcStateFields(settings: Record<string, any> | null | undefined): Record<string, any> {
  const result: Record<string, any> = isPlainObject(settings) ? cloneJson(settings) : {}
  for (const key of CC_STATE_KEYS) delete result[key]
  return result
}

/**
 * 移除核心 env 中的空字符串：
 * 预设里大量 `ANTHROPIC_AUTH_TOKEN: ''`，若原样落盘会在激活时把用户/通用配置的
 * 有效值覆盖成空串。空串按「未设置」处理（需要清空请删键或在更低层处理）。
 */
export function stripEmptyCoreEnv(settings: Record<string, any> | null | undefined): Record<string, any> {
  const result = cloneJson(isPlainObject(settings) ? settings : {})
  if (!isPlainObject(result.env)) return result
  for (const key of CORE_ENV_KEYS) {
    if (result.env[key] === '') delete result.env[key]
  }
  return result
}

/** JSON 深度相等（对象键序无关） */
export function deepEqual(a: any, b: any): boolean {
  if (a === b) return true
  if (Array.isArray(a) && Array.isArray(b)) {
    return a.length === b.length && a.every((item, i) => deepEqual(item, b[i]))
  }
  if (isPlainObject(a) && isPlainObject(b)) {
    const ak = Object.keys(a)
    const bk = Object.keys(b)
    return ak.length === bk.length && ak.every(k => k in b && deepEqual(a[k], b[k]))
  }
  return false
}

// ---------- JSON Pointer（RFC 6901） ----------

export function escapePointerToken(token: string): string {
  return token.replace(/~/g, '~0').replace(/\//g, '~1')
}

export function unescapePointerToken(token: string): string {
  return token.replace(/~1/g, '/').replace(/~0/g, '~')
}

/** 按 JSON Pointer 删除对象叶子；数组在设计中是叶子，不遍历内部 */
export function deleteAtJsonPointer(root: any, pointer: string): boolean {
  if (typeof pointer !== 'string' || !pointer.startsWith('/') || pointer === '/') return false
  const tokens = pointer.slice(1).split('/').map(unescapePointerToken)

  let current: any = root
  for (let i = 0; i < tokens.length - 1; i++) {
    if (!isPlainObject(current)) return false
    current = current[tokens[i]]
    if (current === undefined) return false
  }

  if (!isPlainObject(current)) return false
  const last = tokens[tokens.length - 1]
  if (!(last in current)) return false
  delete current[last]
  return true
}

/** 收集叶子路径（对象递归；数组/标量视作叶子） */
export function collectLeafPaths(value: any, prefix = ''): string[] {
  if (!isPlainObject(value)) return prefix ? [prefix] : []
  const out: string[] = []
  for (const key of Object.keys(value)) {
    out.push(...collectLeafPaths(value[key], `${prefix}/${escapePointerToken(key)}`))
  }
  return out.sort()
}

// ---------- 分层合成 ----------

export function isCommonActive(common?: CommonConfig | null): boolean {
  return !!common?.enabled && isPlainObject(common.settings) && Object.keys(common.settings).length > 0
}

export function providerCommonEnabled(meta?: ProviderMeta): boolean {
  return meta?.commonConfigEnabled ?? true
}

/**
 * effective = deepMerge(通用配置 - removed 路径, Provider 覆盖层)
 */
export function resolveEffective(
  common: CommonConfig | null | undefined,
  overrides: Record<string, any> | null | undefined,
  meta?: ProviderMeta
): Record<string, any> {
  // 核心 env 空串视为未设置（与 Rust resolve_effective 一致）
  const cleanOverrides = stripEmptyCoreEnv(stripCcStateFields(overrides))
  if (!common?.enabled || !providerCommonEnabled(meta)) return cleanOverrides

  const base = stripEmptyCoreEnv(stripCcStateFields(common.settings))
  for (const pointer of meta?.commonConfigRemoved ?? []) {
    deleteAtJsonPointer(base, pointer)
  }
  return deepMergeSettings(base, cleanOverrides)
}

/**
 * 从 value 中移除与 base 相同的叶子（递归）。
 * 返回 undefined 表示该子树与 base 完全一致、可以整体丢弃。
 */
export function subtractEqual(value: any, base: any): any | undefined {
  if (deepEqual(value, base)) return undefined
  if (isPlainObject(value) && isPlainObject(base)) {
    const out: Record<string, any> = {}
    for (const key of Object.keys(value)) {
      if (key in base) {
        const reduced = subtractEqual(value[key], base[key])
        if (reduced !== undefined) out[key] = reduced
      } else {
        out[key] = cloneJson(value[key])
      }
    }
    return Object.keys(out).length > 0 ? out : undefined
  }
  return cloneJson(value)
}

/** effective → 覆盖层（与通用配置相同的值不落盘，自动继承） */
export function diffAgainstCommon(
  effective: Record<string, any> | null | undefined,
  commonSettings: Record<string, any> | null | undefined
): Record<string, any> {
  const reduced = subtractEqual(
    stripEmptyCoreEnv(stripCcStateFields(effective)),
    stripEmptyCoreEnv(stripCcStateFields(commonSettings))
  )
  return isPlainObject(reduced) ? reduced : {}
}

function pathPrefixes(path: string): string[] {
  const tokens = path.split('/').filter(Boolean)
  const out: string[] = []
  let current = ''
  for (const token of tokens) {
    current += `/${token}`
    out.push(current)
  }
  return out
}

/**
 * 计算显式屏蔽路径集合：
 * - 旧 effective 有、新 effective 没有、且来自通用配置的路径 → 加入；
 * - 已在新 effective 出现 / 被覆盖层包含 / 通用配置已不存在的路径 → 移除。
 */
export function computeRemovedPaths(
  prevEffective: Record<string, any> | null | undefined,
  nextEffective: Record<string, any> | null | undefined,
  prevRemoved: string[] | null | undefined,
  commonSettings: Record<string, any> | null | undefined,
  overrides: Record<string, any> | null | undefined
): string[] {
  const commonPaths = new Set(collectLeafPaths(stripCcStateFields(commonSettings)))
  const prevPaths = collectLeafPaths(stripCcStateFields(prevEffective))
  const nextPaths = new Set(collectLeafPaths(stripCcStateFields(nextEffective)))
  const overridePaths = new Set(collectLeafPaths(stripCcStateFields(overrides)))

  const out = new Set(prevRemoved ?? [])
  for (const path of prevPaths) {
    if (!nextPaths.has(path) && commonPaths.has(path)) out.add(path)
  }

  for (const path of [...out]) {
    if (nextPaths.has(path) || !commonPaths.has(path)) {
      out.delete(path)
      continue
    }
    if (pathPrefixes(path).some(prefix => overridePaths.has(prefix))) {
      out.delete(path)
    }
  }

  return [...out].sort()
}
