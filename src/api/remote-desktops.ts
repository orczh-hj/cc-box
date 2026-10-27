/**
 * 已保存桌面（设备）列表：localStorage 持久化
 *
 * 多实例远程架构下手机按「设备」保存凭证（一台电脑一条目，实例分组在
 * 会话列表内呈现）。LAN 与中继两种接入形态：
 * - lan：token = 桌面静态 token，origin = `http://ip:port`（切换 = 跳转该
 *   origin 加载页面，WS 同源连对应端口）
 * - relay：token = v2 `<device_id>.<secret>`，页面在 relay 域上（切换 = 同
 *   页面换 token 重连）
 * 去重键 = origin + deviceId（token 前缀；无点格式取整串），同设备重扫覆盖。
 */

const DESKTOPS_KEY = 'ccbox-desktops'
/** 旧单值存储（v0.16.x 及之前）：保留不删，旧版前端回滚仍可读 */
export const LEGACY_TOKEN_KEY = 'ccbox-remote-token'
const MAX_DESKTOPS = 10

export type DesktopKind = 'relay' | 'lan'

export interface StoredDesktop {
  /** 去重键：`${origin}#${deviceId}` */
  id: string
  kind: DesktopKind
  token: string
  /** lan：`http://ip:port`（切换跳转用）；relay：配对时的 relay 页面 origin */
  origin: string
  /** hello.ok deviceName 回填；未连接过时缺失 */
  name?: string
  /** token 被拒（hello.error）标记；重连成功清除。UI 显示「令牌无效」角标 */
  invalid?: boolean
  savedAt: number
  lastUsedAt: number
}

function isObject(v: unknown): v is Record<string, unknown> {
  return typeof v === 'object' && v !== null && !Array.isArray(v)
}

/** token 中的设备 id：v2 取 `.` 前缀，无点格式（LAN token / v1）取整串 */
export function deviceIdOf(token: string): string {
  const dot = token.indexOf('.')
  return dot > 0 ? token.slice(0, dot) : token
}

/** v2 中继 token：`<device_id>.<secret>` 各 32 hex */
export function isRelayToken(token: string): boolean {
  return /^[0-9a-f]{32}\.[0-9a-f]{32}$/.test(token)
}

export function desktopIdOf(origin: string, token: string): string {
  const o = origin.replace(/\/+$/, '')
  return `${o}#${deviceIdOf(token)}`
}

function parseList(): StoredDesktop[] {
  try {
    const raw = localStorage.getItem(DESKTOPS_KEY)
    if (!raw) return []
    const arr = JSON.parse(raw)
    if (!Array.isArray(arr)) return []
    return arr.filter(
      (d): d is StoredDesktop =>
        isObject(d) &&
        typeof d.id === 'string' &&
        (d.kind === 'relay' || d.kind === 'lan') &&
        typeof d.token === 'string' &&
        typeof d.origin === 'string',
    )
  } catch {
    return []
  }
}

function saveList(list: StoredDesktop[]): void {
  localStorage.setItem(DESKTOPS_KEY, JSON.stringify(list))
}

/** 单调时钟：同毫秒内多次操作也能区分新旧（LRU 淘汰正确性） */
let lastTs = 0
function nextTs(): number {
  lastTs = Math.max(Date.now(), lastTs + 1)
  return lastTs
}

export function listDesktops(): StoredDesktop[] {
  return parseList().sort((a, b) => b.lastUsedAt - a.lastUsedAt)
}

export function getDesktop(id: string): StoredDesktop | undefined {
  return parseList().find(d => d.id === id)
}

/** upsert：已存在（同 id）则更新 token/name 并清 invalid，否则追加；
 * 超上限淘汰 lastUsedAt 最旧。返回落盘后的条目 */
export function addDesktop(d: {
  kind: DesktopKind
  token: string
  origin: string
  name?: string
}): StoredDesktop {
  const now = nextTs()
  const id = desktopIdOf(d.origin, d.token)
  const list = parseList()
  const idx = list.findIndex(x => x.id === id)
  if (idx >= 0) {
    const prev = list[idx]
    const merged: StoredDesktop = {
      ...prev,
      kind: d.kind,
      token: d.token,
      origin: d.origin.replace(/\/+$/, ''),
      ...(d.name ? { name: d.name } : {}),
      invalid: false,
      lastUsedAt: now,
    }
    list[idx] = merged
    saveList(list)
    return merged
  }
  const entry: StoredDesktop = {
    id,
    kind: d.kind,
    token: d.token,
    origin: d.origin.replace(/\/+$/, ''),
    ...(d.name ? { name: d.name } : {}),
    savedAt: now,
    lastUsedAt: now,
  }
  list.push(entry)
  if (list.length > MAX_DESKTOPS) {
    list.sort((a, b) => b.lastUsedAt - a.lastUsedAt)
    list.length = MAX_DESKTOPS
  }
  saveList(list)
  return entry
}

/** 连接使用时更新 lastUsedAt；可选回填 deviceName（hello.ok）与失效标记 */
export function touchDesktop(
  id: string,
  patch?: { name?: string; invalid?: boolean },
): void {
  const list = parseList()
  const idx = list.findIndex(x => x.id === id)
  if (idx < 0) return
  const prev = list[idx]
  list[idx] = {
    ...prev,
    ...(patch?.name ? { name: patch.name } : {}),
    ...(patch && 'invalid' in patch ? { invalid: patch.invalid } : {}),
    lastUsedAt: nextTs(),
  }
  saveList(list)
}

export function removeDesktop(id: string): void {
  saveList(parseList().filter(d => d.id !== id))
}

/**
 * 旧单值 token 一次性迁移为列表条目（列表为空时执行；旧 key 保留不删，
 * 旧版前端回滚仍可读）。kind 按格式判定：v2 → relay，其余（LAN token）→ lan。
 */
export function migrateLegacyToken(origin: string): StoredDesktop | null {
  const legacy = localStorage.getItem(LEGACY_TOKEN_KEY)
  if (!legacy || parseList().length > 0) return null
  return addDesktop({
    kind: isRelayToken(legacy) ? 'relay' : 'lan',
    token: legacy,
    origin,
  })
}
