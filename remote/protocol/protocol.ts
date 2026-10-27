/**
 * CC-Box 远程连接协议正本（TypeScript 侧，与 Rust crate remote/protocol 保持一致）
 *
 * JSON 固定样例在 remote/protocol/samples/，由 Rust 测试与本文件对应的
 * Vitest 契约测试共同执行，保证两端序列化不漂移。
 * 修改协议时同步四处：共享 crate 定义 → protocol.ts → JSON 样例与契约测试 → remote/docs/protocol.md。
 */

export const PROTOCOL_VERSION = 1

/** 手机 → 桌面消息 */
export type ClientMessage =
  | { type: 'hello'; ver: number; client: string; token: string }
  | { type: 'sessions.list' }
  | { type: 'projects.list' }
  | { type: 'session.create'; path: string; sessionId?: string; skipPermissions?: boolean; customArgs?: string; sessionName?: string; instanceId?: string }
  | { type: 'session.kill'; id: string; quiet?: boolean }
  | { type: 'sessions.history'; path: string }
  | { type: 'session.subscribe'; id: string }
  | { type: 'session.unsubscribe'; id: string }
  | { type: 'session.attach'; id: string; cols: number; rows: number }
  | { type: 'session.detach'; id: string }
  | { type: 'pty.input'; id: string; data: string }
  | { type: 'pty.resize'; id: string; cols: number; rows: number }
  | { type: 'ping' }
  | { type: 'rpc.call'; id: number; cmd: string; args: Record<string, unknown> }
  /** 抢占全局控制权（设备互斥：他端持有时唯一获取方式；attach 不能抢占） */
  | { type: 'controller.takeover' }

/** 票据连接的下载范围（受限会话：只能拉取这一个文件的分块） */
export interface DownloadScope {
  fileName: string
  size: number
}

/** 远程可见的会话信息 */
export interface RemoteSession {
  id: string
  /** "claude" | "shell" */
  type: string
  cwd: string
  working: boolean
  pending: boolean
  /** Claude 会话 id（hook SessionStart 建立；shell 会话与旧版桌面端缺失） */
  sessionId?: string
  /** 会话所属 CC-Box 实例 id（设备级 Gateway 多实例聚合；单实例/旧版缺失） */
  instanceId?: string
  /** 实例展示名（如「实例 2」；缺失时手机端降级为不分组平铺） */
  instanceName?: string
}

/** 远程可见的项目（新建会话用） */
export interface RemoteProject {
  path: string
  name: string | null
}

/** 历史会话条目（恢复用） */
export interface RemoteHistorySession {
  id: string
  name: string | null
  lastActiveAt: number
}

/** 桌面 → 手机消息 */
export type ServerMessage =
  | {
      type: 'hello.ok'
      ver: number
      deviceName: string
      /** 本连接唯一标识（前端自判"当前控制者是不是自己"）；旧桌面端缺省 0 */
      connId: number
      /** 当前全局控制者（null=无人持有；0=桌面持有——conn_id 从 1 起，0 保留）；旧桌面端缺省 null */
      controller: number | null
      /** 服务端状态版本基线；旧桌面端缺省 0 */
      revision: number
      /** 票据连接：只能拉取该文件的下载分块；普通连接缺省（旧桌面端不下发） */
      downloadScope?: DownloadScope
    }
  | { type: 'hello.error'; error: string }
  | { type: 'sessions'; list: RemoteSession[]; revision: number }
  | { type: 'projects'; list: RemoteProject[] }
  | { type: 'session.created'; id: string; path: string }
  | { type: 'session.create.failed'; error: string }
  | { type: 'history.sessions'; list: RemoteHistorySession[] }
  | { type: 'session.subscribed'; id: string; tail: string; tailSeq?: number }
  | { type: 'session.unsubscribed'; id: string }
  | { type: 'session.attached'; id: string; cols: number; rows: number; tail: string; tailSeq?: number }
  /** attach 被拒：全局控制权被其他设备持有（先 controller.takeover 接管） */
  | { type: 'session.attach.denied'; id: string; reason: string }
  | { type: 'session.detached'; id: string; reason: string }
  | { type: 'session.status'; id: string; working: boolean; pending: boolean }
  | { type: 'session.exit'; id: string }
  | { type: 'pty.output'; id: string; data: string; seq?: number }
  | { type: 'pong' }
  | { type: 'rpc.result'; id: number; ok: boolean; data?: unknown; error?: string }
  | { type: 'rpc.event'; name: string; payload: unknown }

function isObject(v: unknown): v is Record<string, unknown> {
  return typeof v === 'object' && v !== null && !Array.isArray(v)
}

function str(v: unknown): v is string {
  return typeof v === 'string'
}

function num(v: unknown): v is number {
  return typeof v === 'number' && Number.isFinite(v)
}

function bool(v: unknown): v is boolean {
  return typeof v === 'boolean'
}

/** 解析手机 → 桌面消息；结构不符返回 null（调用方忽略，协议容错） */
export function parseClientMessage(raw: unknown): ClientMessage | null {
  if (!isObject(raw) || !str(raw.type)) return null
  switch (raw.type) {
    case 'hello':
      if (num(raw.ver) && str(raw.client) && str(raw.token)) {
        return { type: 'hello', ver: raw.ver, client: raw.client, token: raw.token }
      }
      return null
    case 'sessions.list':
      return { type: 'sessions.list' }
    case 'projects.list':
      return { type: 'projects.list' }
    case 'session.create':
      if (str(raw.path)) {
        const sessionId = raw.sessionId
        const skipPermissions = raw.skipPermissions
        const customArgs = raw.customArgs
        const sessionName = raw.sessionName
        const instanceId = raw.instanceId
        return {
          type: 'session.create',
          path: raw.path,
          ...(str(sessionId) ? { sessionId } : {}),
          ...(typeof skipPermissions === 'boolean' ? { skipPermissions } : {}),
          ...(str(customArgs) ? { customArgs } : {}),
          ...(str(sessionName) ? { sessionName } : {}),
          ...(str(instanceId) ? { instanceId } : {}),
        }
      }
      return null
    case 'session.kill':
      if (str(raw.id)) {
        const quiet = raw.quiet
        return {
          type: 'session.kill',
          id: raw.id,
          ...(typeof quiet === 'boolean' ? { quiet } : {}),
        }
      }
      return null
    case 'sessions.history':
      if (str(raw.path)) return { type: 'sessions.history', path: raw.path }
      return null
    case 'session.subscribe':
      if (str(raw.id)) return { type: 'session.subscribe', id: raw.id }
      return null
    case 'session.unsubscribe':
      if (str(raw.id)) return { type: 'session.unsubscribe', id: raw.id }
      return null
    case 'session.attach':
      if (str(raw.id) && num(raw.cols) && num(raw.rows)) {
        return { type: 'session.attach', id: raw.id, cols: raw.cols, rows: raw.rows }
      }
      return null
    case 'session.detach':
      if (str(raw.id)) return { type: 'session.detach', id: raw.id }
      return null
    case 'pty.input':
      if (str(raw.id) && str(raw.data)) return { type: 'pty.input', id: raw.id, data: raw.data }
      return null
    case 'pty.resize':
      if (str(raw.id) && num(raw.cols) && num(raw.rows)) {
        return { type: 'pty.resize', id: raw.id, cols: raw.cols, rows: raw.rows }
      }
      return null
    case 'ping':
      return { type: 'ping' }
    case 'rpc.call':
      if (num(raw.id) && str(raw.cmd)) {
        return {
          type: 'rpc.call',
          id: raw.id,
          cmd: raw.cmd,
          args: isObject(raw.args) ? raw.args : {},
        }
      }
      return null
    case 'controller.takeover':
      return { type: 'controller.takeover' }
    default:
      return null
  }
}

/** 解析桌面 → 手机消息；结构不符返回 null（调用方忽略，协议容错） */
export function parseServerMessage(raw: unknown): ServerMessage | null {
  if (!isObject(raw) || !str(raw.type)) return null
  switch (raw.type) {
    case 'hello.ok':
      if (num(raw.ver) && str(raw.deviceName)) {
        return {
          type: 'hello.ok',
          ver: raw.ver,
          deviceName: raw.deviceName,
          connId: num(raw.connId) ? raw.connId : 0,
          controller: num(raw.controller) ? raw.controller : null,
          revision: num(raw.revision) ? raw.revision : 0,
          ...(isObject(raw.downloadScope) &&
          str(raw.downloadScope.fileName) &&
          num(raw.downloadScope.size)
            ? {
                downloadScope: {
                  fileName: raw.downloadScope.fileName,
                  size: raw.downloadScope.size,
                },
              }
            : {}),
        }
      }
      return null
    case 'hello.error':
      if (str(raw.error)) return { type: 'hello.error', error: raw.error }
      return null
    case 'sessions':
      if (Array.isArray(raw.list)) {
        const list: RemoteSession[] = []
        for (const item of raw.list) {
          if (
            isObject(item) && str(item.id) && str(item.type) &&
            str(item.cwd) && bool(item.working) && bool(item.pending) &&
            (item.sessionId === undefined || str(item.sessionId)) &&
            (item.instanceId === undefined || str(item.instanceId)) &&
            (item.instanceName === undefined || str(item.instanceName))
          ) {
            list.push({
              id: item.id,
              type: item.type,
              cwd: item.cwd,
              working: item.working,
              pending: item.pending,
              ...(item.sessionId !== undefined ? { sessionId: item.sessionId } : {}),
              ...(item.instanceId !== undefined ? { instanceId: item.instanceId } : {}),
              ...(item.instanceName !== undefined ? { instanceName: item.instanceName } : {}),
            })
          } else {
            return null
          }
        }
        return { type: 'sessions', list, revision: num(raw.revision) ? raw.revision : 0 }
      }
      return null
    case 'projects':
      if (Array.isArray(raw.list)) {
        const list: RemoteProject[] = []
        for (const item of raw.list) {
          if (isObject(item) && str(item.path) && (item.name === null || str(item.name))) {
            list.push({ path: item.path, name: item.name ?? null })
          } else {
            return null
          }
        }
        return { type: 'projects', list }
      }
      return null
    case 'session.created':
      if (str(raw.id) && str(raw.path)) return { type: 'session.created', id: raw.id, path: raw.path }
      return null
    case 'session.create.failed':
      if (str(raw.error)) return { type: 'session.create.failed', error: raw.error }
      return null
    case 'history.sessions':
      if (Array.isArray(raw.list)) {
        const list: RemoteHistorySession[] = []
        for (const item of raw.list) {
          if (
            isObject(item) && str(item.id) && num(item.lastActiveAt) &&
            (item.name === null || item.name === undefined || str(item.name))
          ) {
            list.push({ id: item.id, name: item.name ?? null, lastActiveAt: item.lastActiveAt })
          } else {
            return null
          }
        }
        return { type: 'history.sessions', list }
      }
      return null
    case 'session.subscribed':
      if (str(raw.id) && str(raw.tail)) {
        const tailSeq = num(raw.tailSeq) ? raw.tailSeq : undefined
        return {
          type: 'session.subscribed',
          id: raw.id,
          tail: raw.tail,
          ...(tailSeq !== undefined ? { tailSeq } : {}),
        }
      }
      return null
    case 'session.unsubscribed':
      if (str(raw.id)) return { type: 'session.unsubscribed', id: raw.id }
      return null
    case 'session.attached':
      if (str(raw.id) && num(raw.cols) && num(raw.rows) && str(raw.tail)) {
        const tailSeq = num(raw.tailSeq) ? raw.tailSeq : undefined
        return {
          type: 'session.attached',
          id: raw.id,
          cols: raw.cols,
          rows: raw.rows,
          tail: raw.tail,
          ...(tailSeq !== undefined ? { tailSeq } : {}),
        }
      }
      return null
    case 'session.attach.denied':
      if (str(raw.id) && str(raw.reason)) {
        return { type: 'session.attach.denied', id: raw.id, reason: raw.reason }
      }
      return null
    case 'session.detached':
      if (str(raw.id) && str(raw.reason)) {
        return { type: 'session.detached', id: raw.id, reason: raw.reason }
      }
      return null
    case 'session.status':
      if (str(raw.id) && bool(raw.working) && bool(raw.pending)) {
        return { type: 'session.status', id: raw.id, working: raw.working, pending: raw.pending }
      }
      return null
    case 'session.exit':
      if (str(raw.id)) return { type: 'session.exit', id: raw.id }
      return null
    case 'pty.output':
      if (str(raw.id) && str(raw.data)) {
        const seq = num(raw.seq) ? raw.seq : undefined
        return {
          type: 'pty.output',
          id: raw.id,
          data: raw.data,
          ...(seq !== undefined ? { seq } : {}),
        }
      }
      return null
    case 'pong':
      return { type: 'pong' }
    case 'rpc.result': {
      if (!num(raw.id) || typeof raw.ok !== 'boolean') return null
      const base = { type: 'rpc.result' as const, id: raw.id, ok: raw.ok }
      if (raw.ok) {
        return { ...base, ...(raw.data !== undefined ? { data: raw.data } : {}) }
      }
      return { ...base, ...(str(raw.error) ? { error: raw.error } : { error: 'unknown error' }) }
    }
    case 'rpc.event':
      if (str(raw.name) && raw.payload !== undefined) {
        return { type: 'rpc.event', name: raw.name, payload: raw.payload }
      }
      return null
    default:
      return null
  }
}
