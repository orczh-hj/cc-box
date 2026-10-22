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
import type { Transport, Unlisten, InvokeOpts } from './transport'

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

  /** 本连接标识与当前控制者标签（hello.ok 下发；设备互斥遮罩判定依据） */
  private _connId: number | null = null
  /** 控制者标签：''=无人持有、'desktop'=桌面、数字串=远程连接 id */
  private _controllerLabel = ''
  /** 已应用的状态版本（事件单调过滤：旧事件不覆盖新状态） */
  private appliedRevision = 0

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

  /** 本连接唯一标识（hello.ok 下发；null = 旧桌面端未下发） */
  get connId(): number | null {
    return this._connId
  }

  /** 当前控制者标签（''=无人持有、'desktop'=桌面、数字串=远程连接 id） */
  get controllerLabel(): string {
    return this._controllerLabel
  }

  /** 是否经中继连接（文件传输限额选档：中继流量按量计费从严；直连走局域网宽松）。
   * 判定依据是运行时事实（配对入口走 relay 服页面 / 收到 relay 层 device.status），
   * 非 URL 猜测；直连永不收到 device.status → 保持 false */
  private _viaRelay = false
  get viaRelay(): boolean {
    return this._viaRelay
  }

  /** 状态版本过滤：state-changed/controller-changed 携带 revision，
   * 不大于已应用版本的事件（旧事件/回环重复）直接丢弃，防止覆盖新状态。
   * 版本跳变（> 已应用+1，中间事件可能丢失——如全量应答基线先于在途事件
   * 入队）时不丢弃，并广播 state-resync-needed 让上层权威重拉全量兜底 */
  private filterStaleEvent(name: string, payload: unknown): boolean {
    if (name !== 'state-changed' && name !== 'controller-changed') return true
    const rev = Number((payload as { data?: string })?.data ?? 0)
    if (!Number.isFinite(rev) || rev === 0) return true // 旧桌面端无版本：不过滤
    if (rev <= this.appliedRevision) return false
    const gap = rev > this.appliedRevision + 1
    this.appliedRevision = rev
    if (gap) {
      this.dispatchEvent('state-resync-needed', {})
    }
    return true
  }

  /** 应用控制者变更：更新内部状态并分发；接管确认（变为自己）时按
   * "重连"语义触发全量重同步（App 重拉 sessions.list、终端重 attach） */
  private applyControllerChanged(payload: unknown): void {
    const id = String((payload as { id?: string })?.id ?? '')
    this._controllerLabel = id
    this.dispatchEvent('controller-changed', { id })
    if (this._connId != null && id === String(this._connId)) {
      // 接管确认 = 以服务端状态为准的全量刷新（与重连完全同构）
      this.dispatchEvent('remote-connected', {})
    }
  }

  /** 入口：解析 URL（#token= / #pair=）或 localStorage，发起连接 */
  async boot(): Promise<void> {
    const params = new URLSearchParams(location.hash.slice(1))
    const hashToken = params.get('token')
    const pairCode = params.get('pair')

    if (pairCode) {
      history.replaceState(null, '', location.pathname)
      this.setConnState('connecting')
      // 配对码仅在 relay 服出的页面上有效——本连接必经中继
      this._viaRelay = true
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
        // 仅中继链路下发此帧——收到即为经中继的运行时证据
        this._viaRelay = true
        this.dispatchEvent('device-status', { online: rawObj.online === true })
        return
      }
      const msg = parseServerMessage(raw)
      if (!msg) return

      if (msg.type === 'hello.ok') {
        this.retry = 0
        this.setConnState('connected')
        this.startPing()
        this._connId = msg.connId
        // controller 语义：null=无人持有；0=桌面持有；>0=远程连接 id
        this._controllerLabel =
          msg.controller == null ? '' : msg.controller === 0 ? 'desktop' : String(msg.controller)
        this.appliedRevision = msg.revision
        // 连接/重连成功：服务端订阅集按连接划分，重连 = 全部 attach 失效，
        // 广播本地事件让会话层重拉列表并重新 attach 当前 Tab
        this.dispatchEvent('remote-connected', {})
        // 控制权初始快照（其他设备持有时本端直接进入全局遮罩）
        this.dispatchEvent('controller-changed', { id: this._controllerLabel })
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
        if (msg.name === 'controller-changed') {
          if (this.filterStaleEvent(msg.name, msg.payload)) {
            this.applyControllerChanged(msg.payload)
          }
          return
        }
        if (this.filterStaleEvent(msg.name, msg.payload)) {
          this.dispatchEvent(msg.name, msg.payload)
        }
        return
      }
      // 运行中会话列表 → 本地事件（App.vue 消费：刷新/新设备后的 Tabs 恢复入口）
      if (msg.type === 'sessions') {
        this.appliedRevision = Math.max(this.appliedRevision, msg.revision)
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

  async invoke<T>(cmd: string, args?: Record<string, unknown>, opts?: InvokeOpts): Promise<T> {
    // 连接未就绪时先等重连（手机切后台/文件选择器回来时 WS 尚在重连，
    // 直接 reject 会让用户操作瞬间莫名失败）
    await this.waitForOpen(opts?.connectTimeoutMs ?? 8000)
    return new Promise<T>((resolve, reject) => {
      if (!this.ws || this.ws.readyState !== WebSocket.OPEN) {
        reject(new Error(`rpc '${cmd}': connection not open`))
        return
      }
      const id = this.nextRpcId++
      // 超时兜底：rpc.result 帧可能被服务端 try_send 丢弃（通道满），
      // WS 依旧健康时 Promise 会永久悬挂——15s 超时 reject 让调用方可重试
      // （慢命令如文件分块可经 opts.timeoutMs 放宽）
      const timer = window.setTimeout(() => {
        this.pending.delete(id)
        reject(new Error(`rpc '${cmd}': timeout`))
      }, opts?.timeoutMs ?? 15000)
      this.pending.set(id, {
        resolve: (v: unknown) => { clearTimeout(timer); resolve(v as T) },
        reject: (e: Error) => { clearTimeout(timer); reject(e) },
      })
      this.ws.send(
        JSON.stringify({ type: 'rpc.call', id, cmd, args: args ?? {} }),
      )
    })
  }

  /** 等待 WS 进入 OPEN（重连窗口内），超时 reject */
  private waitForOpen(timeoutMs: number): Promise<void> {
    if (this.ws?.readyState === WebSocket.OPEN) return Promise.resolve()
    return new Promise<void>((resolve, reject) => {
      const deadline = Date.now() + timeoutMs
      const timer = window.setInterval(() => {
        if (this.ws?.readyState === WebSocket.OPEN) {
          window.clearInterval(timer)
          resolve()
        } else if (Date.now() >= deadline || this._connState === 'denied') {
          window.clearInterval(timer)
          reject(new Error('connection not open (reconnecting)'))
        }
      }, 100)
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
          ...(opts.resumeName ? { sessionName: opts.resumeName } : {}),
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

  async sessionKill(id: string, quiet?: boolean): Promise<boolean> {
    this.sendClient({ type: 'session.kill', id, ...(quiet ? { quiet } : {}) })
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
