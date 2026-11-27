/**
 * 传输层 · 浏览器远程实现（移植自 remote/web 的 WS 客户端）
 *
 * - WS 同源 /ws，首条 hello token 认证（token 来自 URL fragment 或 localStorage）
 * - invoke → rpc.call（pending Promise 按 id resolve）
 * - listen → rpc.event 按 name 分发（payload 与 Tauri 事件同构）
 * - sendClient / onServerMessage → 远程协议消息直通（A5 会话层：pty.output / session.*）
 * - 30s 心跳 + 指数退避重连；hello.error（token 无效）停止重连，relay busy 按瞬态重试
 * - URL 凭证（#token= / #pair=）在页面加载与页面已打开时的 hashchange 都会应用
 */

import {
  PROTOCOL_VERSION,
  parseServerMessage,
  type ClientMessage,
  type DownloadScope,
  type ServerMessage,
} from '../../remote/protocol/protocol'
import { transportEnv } from './env'
import { isCommandNotAllowedError } from '@/utils/rpc-errors'
import { notifyDesktopOutdated } from '@/utils/desktop-outdated'
import type { Transport, Unlisten, InvokeOpts } from './transport'
import {
  addDesktop,
  adoptDeviceCode,
  getDesktop,
  isRelayToken,
  listDesktops,
  migrateLegacyToken,
  touchDesktop,
} from './remote-desktops'

const TOKEN_KEY = 'ccbox-remote-token'
const PING_INTERVAL = 30_000
/** pong 超过该时长未到 → 判定半开连接（浏览器未感知的死亡 WS），主动断开重连。
 *  1.5×PING：留出弱网抖动余量。最坏发现延迟约 PONG_TIMEOUT + PING_INTERVAL（75s），
 *  若连接先被反代空闲超时掐掉则走 onclose 既有重连，两条路都收敛 */
const PONG_TIMEOUT = 45_000
/** 回前台（visibilitychange/focus）校验 ping 的 pong 等待窗口 */
const RESUME_PONG_WINDOW = 5_000
/** 回前台时握手中间态（connecting/authenticating）停留超过该时长视为僵尸连接 */
const HANDSHAKE_STALE_MS = 10_000
/** visibilitychange 与 focus 连发的去重窗口 */
const RESUME_DEDUCE_MS = 1_000
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

/** 连接失败原因（TokenGate 据此给出针对性操作指导；null = 尚无失败） */
export type RemoteFailReason =
  | 'auth-invalid' // hello.error：桌面拒绝令牌（刷新密钥/设备被撤销）
  | 'identity-rotated' // relay 告知该设备身份已作废（电脑端刷新过密钥），重连无意义
  | 'pair-locked' // 配对失败次数过多，relay 临时锁定本 IP
  | 'pair-device-offline' // 连接码对应的电脑不在线
  | 'pair-rejected' // 连接码无效 / 已使用 / 已过期
  | 'pair-network' // 配对请求本身失败（断网）

/** relay `/api/pair` 的错误文案 → 失败原因（字面量以 remote/relay/src/lib.rs `api_pair` 为准） */
export function pairFailReason(
  error: unknown,
): Exclude<RemoteFailReason, 'auth-invalid' | 'identity-rotated'> {
  if (error === 'too many attempts, try later') return 'pair-locked'
  if (error === 'device offline') return 'pair-device-offline'
  return 'pair-rejected'
}

type PairResult =
  | { ok: true; token: string; deviceCode?: string }
  | { ok: false; reason: Exclude<RemoteFailReason, 'auth-invalid' | 'identity-rotated'> }

/** 是否处于断开/未就绪态（回前台校验与 UI 分层共用的判定） */
export function isDisconnectedState(s: RemoteConnState): boolean {
  return s === 'connecting' || s === 'authenticating' || s === 'reconnecting'
}

/** 票据入口：`?dl=<device_id>.<ticket>`（query 更不易被微信复制链接截断） */
function parseDownloadTicket(): string | null {
  if (typeof location === 'undefined') return null
  const fromQuery = new URLSearchParams(location.search).get('dl')
  if (fromQuery) return fromQuery
  return new URLSearchParams(location.hash.slice(1)).get('dl') || null
}

class RemoteTransportImpl implements Transport {
  readonly env = transportEnv

  private ws: WebSocket | null = null
  private token = ''
  private retry = 0
  private retryTimer: number | null = null
  private pingTimer: number | null = null
  private closedByUser = false

  /** 最近一次收到 pong 的时间（半开连接检测依据；手机后台定时器冻结时判定
   *  只由回前台校验承担，避免后台空转重连耗电） */
  private lastPongAt = 0
  /** 本次 open() 发起时间（回前台时识别僵尸握手中间态） */
  private openedAt = 0
  /** 回前台校验 ping 的超时定时器 */
  private resumeTimer: number | null = null
  /** visibilitychange/focus 去重时间戳 */
  private lastResumeAt = 0

  private nextRpcId = 1
  private pending = new Map<number, { resolve: (v: unknown) => void; reject: (e: Error) => void }>()

  /** 本连接标识（hello.ok 下发；会话遮罩归属判定依据） */
  private _connId: number | null = null
  /** 目标电脑 OS（hello.ok 下发；手机键盘平台键名依据，null=旧桌面按 Windows） */
  private _remotePlatform: string | null = null
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

  /** 每会话最后应用的输出序号（检测丢帧；tailSeq 为快照基线） */
  private lastSeq = new Map<string, number>()

  /** 认证/连接状态（TokenGate 与主 UI 的挂载时机依据） */
  onConnState: (state: RemoteConnState) => void = () => {}
  /** 连接状态订阅：主 UI 与弹窗需要同时感知 denied/connected */
  private connStateListeners = new Set<(state: RemoteConnState) => void>()

  /** 设备列表变化通知（add/touch 时触发，TokenGate 重读列表刷新） */
  onDesktopsChanged: () => void = () => {}

  /** 当前连接对应的设备条目 id（设备列表模型；null = 票据/未知来源） */
  private _activeDesktopId: string | null = null
  get activeDesktopId(): string | null {
    return this._activeDesktopId
  }
  /** 下一次 hello.ok 是否代表切换设备（同 origin 换 token 重连）；
   *  true 时上层需丢弃上一台设备的 Tabs */
  private _pendingDeviceSwitch = false

  /** 票据模式：URL 携带 dl= 时以票据为 token 建连（受限会话，不写 localStorage） */
  readonly downloadTicket: string | null = parseDownloadTicket()
  /** 裸票据（去掉 `<device_id>.` 前缀）：`file_download_ticket_chunk` 只接受 32 hex */
  get downloadTicketSecret(): string | null {
    const t = this.downloadTicket
    if (!t) return null
    const dot = t.indexOf('.')
    return dot >= 0 ? t.slice(dot + 1) : t
  }
  private _downloadScope: DownloadScope | null = null
  get downloadScope(): DownloadScope | null {
    return this._downloadScope
  }
  /** 票据认证成功（hello.ok 下发范围）后触发，DownloadTicketPage 消费 */
  onDownloadScope: (scope: DownloadScope) => void = () => {}

  get connState(): RemoteConnState {
    return this._connState
  }
  private _connState: RemoteConnState = 'closed'

  private _failReason: RemoteFailReason | null = null
  /** 最近一次连接被拒的原因；新一轮建连或连接成功时清空 */
  get failReason(): RemoteFailReason | null {
    return this._failReason
  }

  private _attemptSeq = 0
  /** 建连意图序号：每次新的建连选择（URL 凭证 / 切换设备）递增。
   *  在途的配对结果回来时序号已变 = 用户已另作选择，结果作废 */
  get attemptSeq(): number {
    return this._attemptSeq
  }

  /** 本连接唯一标识（hello.ok 下发；null = 旧桌面端未下发） */
  get connId(): number | null {
    return this._connId
  }

  /** 目标电脑 OS（windows/macos/linux；null = 旧桌面端未下发，按 Windows 处理） */
  get remotePlatform(): string | null {
    return this._remotePlatform
  }

  /** 是否经中继连接（文件传输限额选档：中继流量按量计费从严；直连走局域网宽松）。
   * 判定依据是运行时事实（配对入口走 relay 服页面 / 收到 relay 层 device.status），
   * 非 URL 猜测；直连永不收到 device.status → 保持 false */
  private _viaRelay = false
  get viaRelay(): boolean {
    return this._viaRelay
  }

  /** 状态版本过滤：state-changed 携带 revision，
   * 不大于已应用版本的事件（旧事件/回环重复）直接丢弃，防止覆盖新状态。
   * 版本跳变（> 已应用+1，中间事件可能丢失——如全量应答基线先于在途事件
   * 入队）时不丢弃，并广播 state-resync-needed 让上层权威重拉全量兜底 */
  private filterStaleEvent(name: string, payload: unknown): boolean {
    if (name !== 'state-changed') return true
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

  /** 入口：解析 URL（?dl= / #token= / #pair=）或 localStorage，发起连接 */
  async boot(): Promise<void> {
    // 票据页优先：受限会话只拉取单个文件，不进入主 UI 流程
    if (this.downloadTicket) {
      this.setConnState('connecting')
      this._viaRelay = true
      this.token = this.downloadTicket
      this.closedByUser = false
      this.open()
      return
    }
    if (await this.applyUrlCredentials()) return

    // 无 URL 参数：旧单值迁移（一次性）+ 同 origin 的最近设备自动连接。
    // 必须按 origin 过滤——当前页面属于设备 A（端口/relay 域），拿设备 B 的
    // token 来连会 hello.error 且把 B 误标「令牌无效」
    migrateLegacyToken(location.origin)
    const latest = listDesktops().find(d => d.origin.replace(/\/+$/, '') === location.origin.replace(/\/+$/, ''))
    if (latest) {
      this.connectAs(latest.token, latest.id)
    } else {
      this.setConnState('closed')
    }
  }

  /** URL fragment 凭证（`#pair=` / `#token=`）→ 入列表并建连；无凭证返回 false。
   *  页面加载（boot）与已打开页面的 hashchange 共用：地址栏打开只差 fragment 的
   *  新配对链接是同文档导航、不重载页面，只有 hashchange 能拿到它 */
  private async applyUrlCredentials(): Promise<boolean> {
    const params = new URLSearchParams(location.hash.slice(1))
    const hashToken = params.get('token')
    const pairCode = params.get('pair')
    if (!hashToken && !pairCode) return false
    // 凭证只消费一次：先从地址栏抹掉（防历史记录/分享链接泄露）再发起请求
    history.replaceState(null, '', location.pathname)
    this._attemptSeq++

    if (hashToken) {
      const d = addDesktop({
        kind: isRelayToken(hashToken) ? 'relay' : 'lan',
        token: hashToken,
        origin: location.origin,
      })
      this.onDesktopsChanged()
      this.connectAs(hashToken, d.id)
      return true
    }

    // 配对码仅在 relay 服出的页面上有效——本连接必经中继
    this._viaRelay = true
    // 当前连接可用时不打断它：配对失败只是「新设备没连上」，旧连接照常
    const keepCurrent = this._connState === 'connected'
    if (!keepCurrent) {
      this.closeSocket()
      this.retry = 0
      this.setConnState('connecting')
    }
    const seq = this._attemptSeq
    const result = await this.pair(pairCode)
    if (seq !== this._attemptSeq) return true
    if (result.ok) {
      const d = addDesktop({ kind: 'relay', token: result.token, origin: location.origin })
      // 同一台电脑重新配对（刷新密钥换了 device_id）：设备码不变，旧条目让位
      if (result.deviceCode) adoptDeviceCode(location.origin, result.deviceCode, d.id)
      this.onDesktopsChanged()
      this.connectAs(result.token, d.id)
    } else if (keepCurrent) {
      this.dispatchEvent('pair-failed', { reason: result.reason })
    } else {
      this.fail(result.reason)
    }
    return true
  }

  /** 连接被拒：记录原因并进入 denied（不再重连，直到新的建连选择） */
  private fail(reason: RemoteFailReason): void {
    this._failReason = reason
    this.closedByUser = true
    this.setConnState('denied')
  }

  /** 直接以 token 建连（UI 不再提供令牌输入框；测试与脚本用，同样入列表便于下次使用） */
  connectWithToken(token: string): void {
    if (!token.trim()) return
    const t = token.trim()
    const d = addDesktop({
      kind: isRelayToken(t) ? 'relay' : 'lan',
      token: t,
      origin: location.origin,
    })
    this.onDesktopsChanged()
    this.connectAs(t, d.id)
  }

  /** 切换设备：与当前页面同 origin（relay 页/同端口 LAN）时免重载换 token
   * 重连；不同 origin（另一端口/另一 relay 域）时跳转加载（WS 同源语义） */
  activateDesktop(id: string): void {
    const d = getDesktop(id)
    if (!d) return
    if (d.origin.replace(/\/+$/, '') !== location.origin.replace(/\/+$/, '')) {
      location.href = `${d.origin.replace(/\/+$/, '')}/#token=${encodeURIComponent(d.token)}`
      return
    }
    this.connectAs(d.token, d.id)
  }

  private connectAs(token: string, desktopId: string) {
    // 切换设备前彻底关闭旧连接：旧 socket 的 onclose 会按**新 token** 再开
    // 一条（双连接互踢），在途帧也可能串到新设备
    const previousId = this._activeDesktopId
    this._pendingDeviceSwitch = previousId !== null && previousId !== desktopId
    this._attemptSeq++
    this._failReason = null
    this.closeSocket()
    this.retry = 0
    // 旧单值 key 持续写最新值（旧版前端回滚后仍可直接连最近设备）
    localStorage.setItem(TOKEN_KEY, token)
    this._activeDesktopId = desktopId
    this.token = token
    this.closedByUser = false
    touchDesktop(desktopId)
    this.onDesktopsChanged()
    this.open()
  }

  /** 中继配对：一次性码换长期 token v2 `<device_id>.<secret>`（仅经中继 serve 的页面可用） */
  private async pair(code: string): Promise<PairResult> {
    try {
      const res = await fetch('/api/pair', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ code }),
      })
      const data = await res.json()
      if (res.ok && data && data.ok && typeof data.token === 'string') {
        return {
          ok: true,
          token: data.token as string,
          ...(typeof data.deviceCode === 'string' && data.deviceCode ? { deviceCode: data.deviceCode } : {}),
        }
      }
      return { ok: false, reason: pairFailReason(data?.error) }
    } catch {
      return { ok: false, reason: 'pair-network' }
    }
  }

  subscribeConnState(cb: (state: RemoteConnState) => void): Unlisten {
    this.connStateListeners.add(cb)
    return () => {
      this.connStateListeners.delete(cb)
    }
  }

  private setConnState(s: RemoteConnState) {
    this._connState = s
    this.onConnState(s)
    for (const cb of this.connStateListeners) cb(s)
  }

  constructor() {
    // 回前台校验：长离（锁屏/切后台）期间 WS 可能已被系统杀死但 onclose 未触发
    // （半开），回前台瞬间不能信任任何现有连接。focus 是 iOS 微信内核锁屏恢复
    // 偶发漏发 visibilitychange 的兜底；transport 是模块级单例，监听随实例常驻
    if (typeof document !== 'undefined') {
      document.addEventListener('visibilitychange', () => {
        if (document.visibilityState === 'visible') this.onResume()
      })
      window.addEventListener('focus', () => this.onResume())
      // 已打开页面出现新的配对/令牌链接（地址栏打开、书签）：fragment 变化不重载
      // 页面，只有 hashchange 能拿到。票据页是受限会话，不参与
      if (this.downloadTicket === null) {
        window.addEventListener('hashchange', () => {
          void this.applyUrlCredentials()
        })
      }
    }
  }

  /**
   * 回前台校验（可见时才动作；后台不主动判定，防 Android 低频定时器下空转重连）：
   * - connected：发校验 ping，短窗口等 pong，失败视为半开 → 干净重建连接
   * - connecting/authenticating 停留过久：hello 帧丢失的僵尸握手 → 重建
   * - reconnecting：清退避立即重试（后台排定的退避定时器由 open() 首行清掉）
   * - denied/closedByUser（token 失效/用户主动关闭）不重试，维持既有语义
   */
  private onResume(): void {
    if (Date.now() - this.lastResumeAt < RESUME_DEDUCE_MS) return
    this.lastResumeAt = Date.now()
    if (this.closedByUser || this._connState === 'denied' || this._connState === 'closed') return

    if (this._connState === 'connected') {
      const ws = this.ws
      if (!ws) return
      // 先发校验 ping，pong 分支收到即清除定时器；窗口内未回 → 连接已死
      if (this.resumeTimer !== null) window.clearTimeout(this.resumeTimer)
      this.resumeTimer = window.setTimeout(() => {
        this.resumeTimer = null
        if (this.ws !== ws || this._connState !== 'connected') return
        this.closeSocket()
        this.retry = 0
        this.open()
      }, RESUME_PONG_WINDOW)
      this.sendClient({ type: 'ping' })
      return
    }
    if (this._connState === 'connecting' || this._connState === 'authenticating') {
      if (Date.now() - this.openedAt > HANDSHAKE_STALE_MS) {
        this.closeSocket()
        this.retry = 0
        this.open()
      }
      return
    }
    if (this._connState === 'reconnecting') {
      this.retry = 0
      this.open()
    }
  }


  /** 主动关闭当前连接（切换设备）：清定时器 + 摘除回调，避免旧连接
   * 触发重连或在途帧投递 */
  private closeSocket(): void {
    this.cleanupTimers()
    const ws = this.ws
    this.ws = null
    if (!ws) return
    ws.onopen = null
    ws.onmessage = null
    ws.onclose = null
    ws.onerror = null
    try { ws.close() } catch { /* 已关闭 */ }
  }

  private open(): void {
    this.cleanupTimers()
    this.openedAt = Date.now()
    this.setConnState(this.retry === 0 ? 'connecting' : 'reconnecting')

    const proto = location.protocol === 'https:' ? 'wss' : 'ws'
    const ws = new WebSocket(`${proto}://${location.host}/ws`)
    this.ws = ws

    ws.onopen = () => {
      if (this.ws !== ws) return
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
      if (this.ws !== ws) return
      let raw: unknown
      try {
        raw = JSON.parse(ev.data as string)
      } catch {
        return
      }
      // 中继层 device.status：桌面在线状态，转成事件供 UI 订阅
      const rawObj = raw as { type?: string; online?: boolean; rotated?: boolean }
      if (rawObj?.type === 'device.status') {
        // 仅中继链路下发此帧——收到即为经中继的运行时证据
        this._viaRelay = true
        const online = rawObj.online === true
        this.dispatchEvent('device-status', { online })
        // 身份已作废（电脑端刷新过密钥）：再怎么重连也回不来，直接停止并给出重扫指导
        if (rawObj.rotated === true) {
          this.fail('identity-rotated')
          ws.close()
          return
        }
        if (!online) {
          // 设备侧已断开：relay 会移除本连接的路由但保持 WS——继续持有只会
          // 让 rpc/attach 全部黑洞（消息被 relay 丢弃，直到心跳超时才重连）。
          // 主动断开立即进入重连；retry 归零让退避从最小值起步（设备通常
          // 数秒内回来，重连一次即恢复）
          this.retry = 0
          ws.close()
        }
        return
      }
      const msg = parseServerMessage(raw)
      if (!msg) return

      if (msg.type === 'pong') {
        // 活性证据：刷新半开检测基线，并终结回前台校验窗口
        this.lastPongAt = Date.now()
        if (this.resumeTimer !== null) {
          window.clearTimeout(this.resumeTimer)
          this.resumeTimer = null
        }
        return
      }
      if (msg.type === 'hello.ok') {
        this.retry = 0
        this._failReason = null
        this.setConnState('connected')
        this.startPing()
        this._connId = msg.connId
        this._remotePlatform = msg.platform ?? null
        // 设备名回填列表条目（TokenGate 展示），并清除历史失效标记；带设备码时
        // 记到条目上（此后这台电脑刷新密钥、重新配对，旧条目能被认出并替换）
        if (this._activeDesktopId) {
          touchDesktop(this._activeDesktopId, { name: msg.deviceName, invalid: false })
          if (msg.deviceCode) adoptDeviceCode(location.origin, msg.deviceCode, this._activeDesktopId)
          this.onDesktopsChanged()
        }
        this.appliedRevision = msg.revision
        // 重连：订阅集按连接划分，旧 attach 失效；序号基线等待新的 attached
        this.lastSeq.clear()
        if (msg.downloadScope) {
          this._downloadScope = msg.downloadScope
          this.onDownloadScope(msg.downloadScope)
        }
        // 连接/重连成功：服务端订阅集按连接划分，重连 = 全部 attach 失效，
        // 广播本地事件让会话层重拉列表并重新 attach 当前 Tab。
        // deviceChanged：同 origin 切换设备 → 上层必须先丢弃旧设备的 Tabs
        const deviceChanged = this._pendingDeviceSwitch
        this._pendingDeviceSwitch = false
        this.dispatchEvent('remote-connected', { deviceChanged })
        return
      }
      if (msg.type === 'hello.error') {
        // relay busy（连接数上限）是瞬态：断开走既有退避重连，不判令牌失效
        if (/busy/i.test(msg.error ?? '')) {
          ws.close()
          return
        }
        this.fail('auth-invalid')
        this._pendingDeviceSwitch = false
        // 条目标记失效（列表显示「令牌无效」角标；不删除，密钥轮换后可重扫覆盖）
        if (this._activeDesktopId) {
          touchDesktop(this._activeDesktopId, { invalid: true })
          this.onDesktopsChanged()
        }
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
            const err = new Error(msg.error ?? 'rpc failed')
            // 新前端 + 旧桌面：命令不在旧版白名单 → 提示更新桌面端（每次会话一次）
            if (isCommandNotAllowedError(err)) notifyDesktopOutdated()
            p.reject(err)
          }
        }
        return
      }
      if (msg.type === 'rpc.event') {
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
        const seq = msg.seq ?? 0
        if (seq > 0) {
          const prev = this.lastSeq.get(msg.id) ?? 0
          // 快照与实时流重叠：旧序号丢弃
          if (prev > 0 && seq <= prev) return
          if (prev > 0 && seq > prev + 1) {
            this.dispatchEvent('pty-output', {
              id: msg.id,
              data: `\r\n\x1b[33m[输出中断：跳过 ${seq - prev - 1} 段，正在重新同步]\x1b[0m\r\n`,
            })
          }
          this.lastSeq.set(msg.id, seq)
        }
        this.dispatchEvent('pty-output', { id: msg.id, data: msg.data })
        return
      }
      if (msg.type === 'session.exit') {
        this.lastSeq.delete(msg.id)
        this.dispatchEvent('pty-exit', { id: msg.id, exitCode: 0 })
        return
      }
      if (msg.type === 'session.attached') {
        // 快照基线：后续 PtyOutput.seq <= tailSeq 属于重叠，直接丢弃。
        // claim 重 attach 时实时流可能已超过 tailSeq，只能抬高基线，不能降低。
        if (msg.tailSeq !== undefined) {
          const prev = this.lastSeq.get(msg.id) ?? 0
          this.lastSeq.set(msg.id, Math.max(prev, msg.tailSeq))
        } else {
          this.lastSeq.delete(msg.id)
        }
        // attach 快照（tail）作为首段输出写入终端；claim 重 attach 时已订阅
        // 实时流，写入 tail 会重复——跳过
        if (this.skipAttachedTailIds.delete(msg.id)) return
        this.dispatchEvent('pty-output', { id: msg.id, data: msg.tail })
        return
      }
      if (msg.type === 'session.subscribed') {
        // 只读订阅：记录序号基线，不把 tail 写入终端（避免未 attach 的重复内容）；
        // 同样只抬高，不降低已有实时流基线。
        if (msg.tailSeq !== undefined) {
          const prev = this.lastSeq.get(msg.id) ?? 0
          this.lastSeq.set(msg.id, Math.max(prev, msg.tailSeq))
        }
        return
      }
      if (msg.type === 'session.attach.denied') {
        // attach 被全局控制权拒绝（其他端持有）：分发事件，会话层置 Tab 级
        // 遮罩 + 接管入口——不处理的话 attach 悬挂、终端白屏
        this.dispatchEvent('session-attach-denied', { id: msg.id, reason: msg.reason })
        return
      }
      if (msg.type === 'session.detached') {
        // 会话被剥离（宿主窗口断开等）：分发事件（会话层置 stopped）并清
        // 输出序号基线——不处理的话 Tab 永远 running、终端冻结
        this.lastSeq.delete(msg.id)
        this.dispatchEvent('session-detached', { id: msg.id, reason: msg.reason })
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
      // 已被替换/主动关闭的旧连接不再驱动重连
      if (this.ws !== ws) return
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
    this.lastPongAt = Date.now()
    this.pingTimer = window.setInterval(() => {
      // 半开检测内联在 ping 周期里（零新增 timer）：页面可见且 pong 逾期 →
      // 连接已死但浏览器未感知。走 ws.close()（非 closeSocket）——onclose
      // 既有退避重连自然接管。后台不判定（定时器低频/冻结，回前台校验兜底）
      if (
        document.visibilityState === 'visible' &&
        Date.now() - this.lastPongAt > PONG_TIMEOUT
      ) {
        this.ws?.close()
        return
      }
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
    if (this.resumeTimer !== null) {
      clearTimeout(this.resumeTimer)
      this.resumeTimer = null
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
          ...(opts.instanceId ? { instanceId: opts.instanceId } : {}),
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
