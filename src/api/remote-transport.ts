/**
 * 传输层 · 浏览器远程实现（移植自 remote/web 的 WS 客户端）
 *
 * - WS 同源 /ws，首条 hello token 认证（token 来自 URL fragment 或 localStorage）
 * - invoke → rpc.call（pending Promise 按 id resolve）
 * - listen → rpc.event 按 name 分发（payload 与 Tauri 事件同构）
 * - sendClient / onServerMessage → 远程协议消息直通（A5 会话层：pty.output / session.*）
 * - 30s 心跳 + 指数退避重连；hello.error（token 无效）停止重连
 */

import {
  PROTOCOL_VERSION,
  parseServerMessage,
  type ClientMessage,
  type ServerMessage,
} from '../../remote/protocol/protocol'
import { transportEnv } from './env'
import type { Transport, Unlisten } from './transport'

const TOKEN_KEY = 'ccbox-remote-token'
const PING_INTERVAL = 30_000
const RETRY_MIN = 1_000
const RETRY_MAX = 30_000

export type RemoteConnState =
  | 'connecting'
  | 'authenticating'
  | 'connected'
  | 'reconnecting'
  | 'denied'
  | 'closed'

type EventHandler = (payload: unknown) => void

class RemoteTransportImpl implements Transport {
  readonly env = transportEnv

  private ws: WebSocket | null = null
  private token = ''
  private retry = 0
  private retryTimer: number | null = null
  private pingTimer: number | null = null
  private closedByUser = false

  private nextRpcId = 1
  private pending = new Map<number, { resolve: (v: unknown) => void; reject: (e: Error) => void }>()

  private eventHandlers = new Map<string, Set<EventHandler>>()
  private serverMessageHandlers = new Set<(msg: ServerMessage) => void>()

  /** session.create 的等待队列（与 session.created/create.failed 按序配对） */
  private createWaiters: Array<
    (created: { id: string; path: string } | null, error: string | null) => void
  > = []

  /** claim 重 attach 的 ptyId：已有实时输出流，跳过 attached 的 tail 快照防重复写入 */
  private skipAttachedTailIds = new Set<string>()

  /** 认证/连接状态（TokenGate 与主 UI 的挂载时机依据） */
  onConnState: (state: RemoteConnState) => void = () => {}

  get connState(): RemoteConnState {
    return this._connState
  }
  private _connState: RemoteConnState = 'closed'

  /** 入口：解析 URL（#token= / #pair=）或 localStorage，发起连接 */
  async boot(): Promise<void> {
    const params = new URLSearchParams(location.hash.slice(1))
    const hashToken = params.get('token')
    const pairCode = params.get('pair')

    if (pairCode) {
      history.replaceState(null, '', location.pathname)
      this.setConnState('connecting')
      const token = await this.pair(pairCode)
      if (token) {
        this.storeAndConnect(token)
      } else {
        this.setConnState('denied')
      }
      return
    }

    if (hashToken) {
      history.replaceState(null, '', location.pathname)
      this.storeAndConnect(hashToken)
      return
    }

    const saved = localStorage.getItem(TOKEN_KEY) ?? ''
    if (saved) {
      this.storeAndConnect(saved)
    } else {
      this.setConnState('closed')
    }
  }

  /** 手动输入 token 连接（TokenGate 输入框） */
  connectWithToken(token: string): void {
    if (!token.trim()) return
    this.storeAndConnect(token.trim())
  }

  private storeAndConnect(token: string) {
    localStorage.setItem(TOKEN_KEY, token)
    this.token = token
    this.closedByUser = false
    this.open()
  }

  /** 中继配对：一次性码换长期 access_token（仅经中继 serve 的页面可用） */
  private async pair(code: string): Promise<string | null> {
    try {
      const res = await fetch('/api/pair', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ code }),
      })
      const data = await res.json()
      if (data && data.ok && typeof data.token === 'string') {
        return data.token as string
      }
      return null
    } catch {
      return null
    }
  }

  private setConnState(s: RemoteConnState) {
    this._connState = s
    this.onConnState(s)
  }

  private open(): void {
    this.cleanupTimers()
    this.setConnState(this.retry === 0 ? 'connecting' : 'reconnecting')

    const proto = location.protocol === 'https:' ? 'wss' : 'ws'
    const ws = new WebSocket(`${proto}://${location.host}/ws`)
    this.ws = ws

    ws.onopen = () => {
      this.setConnState('authenticating')
      ws.send(
        JSON.stringify({
          type: 'hello',
          ver: PROTOCOL_VERSION,
          client: 'web-1.0.0',
          token: this.token,
        }),
      )
    }

    ws.onmessage = (ev) => {
      let raw: unknown
      try {
        raw = JSON.parse(ev.data as string)
      } catch {
        return
      }
      // 中继层 device.status：桌面在线状态，转成事件供 UI 订阅
      const rawObj = raw as { type?: string; online?: boolean }
      if (rawObj?.type === 'device.status') {
        this.dispatchEvent('device-status', { online: rawObj.online === true })
        return
      }
      const msg = parseServerMessage(raw)
      if (!msg) return

      if (msg.type === 'hello.ok') {
        this.retry = 0
        this.setConnState('connected')
        this.startPing()
        // 连接/重连成功：服务端订阅集按连接划分，重连 = 全部 attach 失效，
        // 广播本地事件让会话层重拉列表并重新 attach 当前 Tab
        this.dispatchEvent('remote-connected', {})
        return
      }
      if (msg.type === 'hello.error') {
        this.setConnState('denied')
        this.closedByUser = true // token 无效不再重连
        ws.close()
        return
      }
      // rpc.result → resolve pending invoke；rpc.event → 事件分发；其余 → 会话层
      if (msg.type === 'rpc.result') {
        const p = this.pending.get(msg.id)
        if (p) {
          this.pending.delete(msg.id)
          if (msg.ok) {
            p.resolve(msg.data)
          } else {
            p.reject(new Error(msg.error ?? 'rpc failed'))
          }
        }
        return
      }
      if (msg.type === 'rpc.event') {
        this.dispatchEvent(msg.name, msg.payload)
        return
      }
      // 运行中会话列表 → 本地事件（App.vue 消费：刷新/新设备后的 Tabs 恢复入口）
      if (msg.type === 'sessions') {
        this.dispatchEvent('remote-sessions', msg.list)
        return
      }
      // 远程协议消息 → 映射为与桌面 Tauri 事件同构的 pty-output / pty-exit
      // （XTermTerminal 等组件经 transport.listen 订阅，零感知运行环境）
      if (msg.type === 'pty.output') {
        this.dispatchEvent('pty-output', { id: msg.id, data: msg.data })
        return
      }
      if (msg.type === 'session.exit') {
        this.dispatchEvent('pty-exit', { id: msg.id, exitCode: 0 })
        return
      }
      if (msg.type === 'session.attached') {
        // attach 快照（tail）作为首段输出写入终端；claim 重 attach 时已订阅
        // 实时流，写入 tail 会重复——跳过
        if (this.skipAttachedTailIds.delete(msg.id)) return
        this.dispatchEvent('pty-output', { id: msg.id, data: msg.tail })
        return
      }
      if (msg.type === 'session.create.failed') {
        const cb = this.createWaiters.shift()
        cb?.(null, msg.error)
        return
      }
      if (msg.type === 'session.created') {
        const cb = this.createWaiters.shift()
        cb?.(msg, null)
        return
      }
      for (const h of this.serverMessageHandlers) {
        h(msg) // 收包即派发，不 await（保序且不阻塞读循环）
      }
    }

    ws.onclose = () => {
      this.cleanupTimers()
      this.rejectAllPending('connection closed')
      if (this.closedByUser) {
        this.setConnState('closed')
        return
      }
      this.retry += 1
      const delay = Math.min(RETRY_MIN * 2 ** (this.retry - 1), RETRY_MAX)
      this.retryTimer = window.setTimeout(() => this.open(), delay)
      this.setConnState('reconnecting')
    }

    ws.onerror = () => {
      // onclose 随后触发，重连逻辑在那里
    }
  }

  private startPing(): void {
    this.pingTimer = window.setInterval(() => {
      this.sendClient({ type: 'ping' })
    }, PING_INTERVAL)
  }

  private cleanupTimers(): void {
    if (this.pingTimer !== null) {
      clearInterval(this.pingTimer)
      this.pingTimer = null
    }
    if (this.retryTimer !== null) {
      clearTimeout(this.retryTimer)
      this.retryTimer = null
    }
  }

  private rejectAllPending(reason: string): void {
    for (const p of this.pending.values()) {
      p.reject(new Error(reason))
    }
    this.pending.clear()
  }

  private dispatchEvent(name: string, payload: unknown): void {
    const handlers = this.eventHandlers.get(name)
    if (handlers) {
      for (const h of handlers) h(payload)
    }
  }

  // ==================== Transport 接口 ====================

  invoke<T>(cmd: string, args?: Record<string, unknown>): Promise<T> {
    return new Promise<T>((resolve, reject) => {
      if (!this.ws || this.ws.readyState !== WebSocket.OPEN) {
        reject(new Error(`rpc '${cmd}': connection not open`))
        return
      }
      const id = this.nextRpcId++
      // 超时兜底：rpc.result 帧可能被服务端 try_send 丢弃（通道满），
      // WS 依旧健康时 Promise 会永久悬挂——15s 超时 reject 让调用方可重试
      const timer = window.setTimeout(() => {
        this.pending.delete(id)
        reject(new Error(`rpc '${cmd}': timeout`))
      }, 15000)
      this.pending.set(id, {
        resolve: (v: unknown) => { clearTimeout(timer); resolve(v as T) },
        reject: (e: Error) => { clearTimeout(timer); reject(e) },
      })
      this.ws.send(
        JSON.stringify({ type: 'rpc.call', id, cmd, args: args ?? {} }),
      )
    })
  }

  listen<T>(event: string, cb: (payload: T) => void): Promise<Unlisten> {
    let set = this.eventHandlers.get(event)
    if (!set) {
      set = new Set()
      this.eventHandlers.set(event, set)
    }
    const handler = cb as EventHandler
    set.add(handler)
    return Promise.resolve(() => {
      set?.delete(handler)
    })
  }

  sendClient(msg: ClientMessage): void {
    if (this.ws?.readyState === WebSocket.OPEN) {
      this.ws.send(JSON.stringify(msg))
    }
  }

  onServerMessage(cb: (msg: ServerMessage) => void): Unlisten {
    this.serverMessageHandlers.add(cb)
    return () => {
      this.serverMessageHandlers.delete(cb)
    }
  }

  // ==================== 会话 API（远程协议消息） ====================

  async spawnSession(opts: import('./transport').SpawnSessionOptions): Promise<import('./transport').SpawnSessionResult> {
    // session.create 带项目白名单与并发上限；成功后立即 attach（输出流 + 控制权）
    const created = await new Promise<{ id: string; path: string }>(
      (resolve, reject) => {
        const timer = window.setTimeout(() => {
          const idx = this.createWaiters.indexOf(waiter)
          if (idx >= 0) this.createWaiters.splice(idx, 1)
          reject(new Error('session create timeout'))
        }, 30_000)
        const waiter = (c: { id: string; path: string } | null, error: string | null) => {
          window.clearTimeout(timer)
          if (c) resolve(c)
          else reject(new Error(error ?? 'session create failed'))
        }
        this.createWaiters.push(waiter)
        this.sendClient({
          type: 'session.create',
          path: opts.cwd,
          ...(opts.resume ? { sessionId: opts.resume } : {}),
          ...(opts.skipPermissions ? { skipPermissions: true } : {}),
          ...(opts.customArgs?.trim() ? { customArgs: opts.customArgs.trim() } : {}),
        })
      },
    )
    // attach（tail 快照经 session.attached 映射为 pty-output 事件写入终端）
    this.sendClient({ type: 'session.attach', id: created.id, cols: opts.cols, rows: opts.rows })
    return { id: created.id, cwd: created.path }
  }

  async sessionInput(id: string, data: string): Promise<boolean> {
    this.sendClient({ type: 'pty.input', id, data })
    return true
  }

  async sessionResize(id: string, cols: number, rows: number): Promise<boolean> {
    this.sendClient({ type: 'pty.resize', id, cols, rows })
    return true
  }

  async sessionKill(id: string): Promise<boolean> {
    this.sendClient({ type: 'session.kill', id })
    return true
  }

  async sessionClaim(id: string, cols = 80, rows = 24): Promise<void> {
    // 浏览器端夺回 = 重新 attach（后 attach 优先抢占）；attach 会 resize PTY，
    // 必须传当前实际尺寸，否则终端被重置回默认 80×24
    this.skipAttachedTailIds.add(id)
    this.sendClient({ type: 'session.attach', id, cols, rows })
  }

  close(): void {
    this.closedByUser = true
    this.cleanupTimers()
    this.ws?.close()
    this.ws = null
  }
}

export const remoteTransport = new RemoteTransportImpl()
