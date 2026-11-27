/**
 * remote-transport 配对入口测试（URL fragment 凭证）
 *
 * 覆盖浏览器远程「已打开页面用带新连接码的链接再进入会失败」的修复：
 * - #pair= / #token= 不只在首次加载读一次，页面已打开时的 hashchange 同样生效
 *   （地址栏粘贴只差 fragment 的链接是同文档导航，不重载页面）
 * - 配对失败保留真实原因；已连接时失败不打断现有连接
 * - relay busy 是瞬态，走退避重连而不是判令牌失效
 *
 * 真值来源：relay 的错误文案字面量（remote/relay/src/lib.rs api_pair / ws_client）、
 * URL 与 stub 返回的 token 字面量。
 */
import { describe, it, expect, beforeEach, afterEach, vi } from 'vitest'

// ---- WebSocket mock：收集实例供测试注入服务端消息 ----
class FakeWebSocket {
  static instances: FakeWebSocket[] = []
  static SENT: Array<Record<string, unknown>> = []
  static readonly CONNECTING = 0
  static readonly OPEN = 1
  static readonly CLOSING = 2
  static readonly CLOSED = 3
  readyState = 0
  onopen: (() => void) | null = null
  onmessage: ((ev: { data: string }) => void) | null = null
  onclose: (() => void) | null = null
  onerror: (() => void) | null = null
  constructor() {
    FakeWebSocket.instances.push(this)
  }
  send(data: string) {
    FakeWebSocket.SENT.push(JSON.parse(data))
  }
  // 真实浏览器的 close 异步触发 onclose：同步触发会把 hello.error 刚置的 denied 覆盖成 closed
  close() {
    this.readyState = 3
    queueMicrotask(() => this.onclose?.())
  }
  serverSend(msg: Record<string, unknown>) {
    this.onmessage?.({ data: JSON.stringify(msg) })
  }
  serverOpen() {
    this.readyState = 1
    this.onopen?.()
  }
}

vi.stubGlobal('WebSocket', FakeWebSocket as unknown as typeof WebSocket)

// 可读写的内存 localStorage：配对结果经 addDesktop 落盘后才建连
const store = new Map<string, string>()
vi.stubGlobal('localStorage', {
  getItem: (k: string) => store.get(k) ?? null,
  setItem: (k: string, v: string) => void store.set(k, v),
  removeItem: (k: string) => void store.delete(k),
})

const { remoteTransport } = await import('../../src/api/remote-transport')

function lastWs(): FakeWebSocket {
  return FakeWebSocket.instances[FakeWebSocket.instances.length - 1]
}

/** token v2：`<device_id>.<secret>` 各 32 hex */
const OLD_TOKEN = `${'a'.repeat(32)}.${'b'.repeat(32)}`
const NEW_TOKEN = `${'c'.repeat(32)}.${'d'.repeat(32)}`
const PAIR_CODE = 'A30A3214'

const fetchMock = vi.fn()

/** 模拟同文档导航：改 fragment 后派发 hashchange（浏览器不重载页面） */
function navigate(hash: string): void {
  history.replaceState(null, '', hash)
  window.dispatchEvent(new Event('hashchange'))
}

function pairResponse(body: unknown): void {
  fetchMock.mockResolvedValue({ ok: true, json: () => Promise.resolve(body) })
}

/** 以 token 建连并完成握手（hello.ok） */
function connect(token: string): FakeWebSocket {
  remoteTransport.connectWithToken(token)
  const ws = lastWs()
  ws.serverOpen()
  ws.serverSend({ type: 'hello.ok', ver: 1, deviceName: 'D', connId: 7, revision: 0 })
  return ws
}

/** 已发出 hello 帧里出现过的 token */
function helloTokens(): string[] {
  return FakeWebSocket.SENT.filter(m => m.type === 'hello').map(m => String(m.token))
}

describe('remote-transport 配对入口（hashchange）', () => {
  beforeEach(() => {
    FakeWebSocket.instances = []
    FakeWebSocket.SENT = []
    store.clear()
    fetchMock.mockReset()
    vi.stubGlobal('fetch', fetchMock)
    history.replaceState(null, '', '/')
    remoteTransport.close()
    vi.useRealTimers()
  })
  afterEach(() => {
    vi.useRealTimers()
  })

  // 已打开页面出现带新连接码的链接：应发起配对并用换到的 token 建连
  //（修复前：fragment 只在首次加载读一次，hashchange 被忽略）
  it('RemoteTransport_HashChangeAppliesPair_001', async () => {
    pairResponse({ ok: true, token: NEW_TOKEN })
    const before = FakeWebSocket.instances.length

    navigate(`/#pair=${PAIR_CODE}`)
    await new Promise(r => setTimeout(r, 0))

    expect(fetchMock).toHaveBeenCalledTimes(1)
    const [url, init] = fetchMock.mock.calls[0] as unknown as [string, RequestInit]
    expect(url).toBe('/api/pair')
    expect(JSON.parse(String(init.body))).toEqual({ code: PAIR_CODE })
    // 凭证只消费一次：不从地址栏残留
    expect(location.hash).toBe('')
    expect(FakeWebSocket.instances.length).toBe(before + 1)
    lastWs().serverOpen()
    expect(helloTokens()).toEqual([NEW_TOKEN])
  })

  // 主 UI 已挂载、旧 token 的设备在 relay 上离线（刷新密钥换了 device_id）：
  // 换新码后应切到新身份，并停止用旧 token 重连
  it('RemoteTransport_HashChangeRecoversFromOfflineLoop_002', async () => {
    vi.useFakeTimers()
    pairResponse({ ok: true, token: NEW_TOKEN })
    const ws = connect(OLD_TOKEN)
    // relay：设备离线 → 前端断开进入重连循环
    ws.serverSend({ type: 'device.status', online: false })

    const events: unknown[] = []
    const un = await remoteTransport.listen('remote-connected', e => events.push(e))
    navigate(`/#pair=${PAIR_CODE}`)
    await vi.advanceTimersByTimeAsync(0)
    lastWs().serverOpen()
    lastWs().serverSend({ type: 'hello.ok', ver: 1, deviceName: 'D', connId: 8, revision: 0 })
    un()

    expect(events).toEqual([{ deviceChanged: true }])
    FakeWebSocket.SENT.length = 0
    await vi.advanceTimersByTimeAsync(5_000)
    expect(helloTokens()).not.toContain(OLD_TOKEN)
  })

  // 失效覆盖层（denied）状态下换新码：应恢复连接并清空失败原因
  it('RemoteTransport_HashChangeRecoversFromDenied_003', async () => {
    pairResponse({ ok: true, token: NEW_TOKEN })
    const ws = connect(OLD_TOKEN)
    ws.serverSend({ type: 'hello.error', error: 'invalid token' })
    expect(remoteTransport.connState).toBe('denied')

    navigate(`/#pair=${PAIR_CODE}`)
    await new Promise(r => setTimeout(r, 0))
    lastWs().serverOpen()
    lastWs().serverSend({ type: 'hello.ok', ver: 1, deviceName: 'D', connId: 9, revision: 0 })

    expect(remoteTransport.connState).toBe('connected')
    expect(remoteTransport.failReason).toBeNull()
  })

  // #token= 不经配对接口，直接入列表建连
  it('RemoteTransport_HashChangeToken_004', async () => {
    const before = FakeWebSocket.instances.length
    navigate('/#token=lan-static-token')
    await new Promise(r => setTimeout(r, 0))

    expect(fetchMock).not.toHaveBeenCalled()
    expect(location.hash).toBe('')
    expect(FakeWebSocket.instances.length).toBe(before + 1)
    lastWs().serverOpen()
    expect(helloTokens()).toEqual(['lan-static-token'])
  })

  // 不含凭证的 fragment 变化：什么都不做（不能落进「自动连最近设备」分支）
  it('RemoteTransport_HashChangeNoCredentialIgnored_005', async () => {
    const before = FakeWebSocket.instances.length
    const stateBefore = remoteTransport.connState
    navigate('/#section')
    navigate('/')
    await new Promise(r => setTimeout(r, 0))

    expect(fetchMock).not.toHaveBeenCalled()
    expect(FakeWebSocket.instances.length).toBe(before)
    expect(remoteTransport.connState).toBe(stateBefore)
  })

  // relay 文案「device offline」：码已被消费，原因归设备离线
  it('RemoteTransport_PairOfflineReason_006', async () => {
    pairResponse({ ok: false, error: 'device offline' })
    navigate(`/#pair=${PAIR_CODE}`)
    await new Promise(r => setTimeout(r, 0))

    expect(remoteTransport.connState).toBe('denied')
    expect(remoteTransport.failReason).toBe('pair-device-offline')
  })

  // relay 文案「too many attempts, try later」：同 IP 连续失败被临时锁定
  it('RemoteTransport_PairLockedReason_007', async () => {
    pairResponse({ ok: false, error: 'too many attempts, try later' })
    navigate(`/#pair=${PAIR_CODE}`)
    await new Promise(r => setTimeout(r, 0))

    expect(remoteTransport.failReason).toBe('pair-locked')
  })

  // relay 文案「invalid or expired code」：码无效 / 已使用 / 已过期
  it('RemoteTransport_PairExpiredReason_008', async () => {
    pairResponse({ ok: false, error: 'invalid or expired code' })
    navigate(`/#pair=${PAIR_CODE}`)
    await new Promise(r => setTimeout(r, 0))

    expect(remoteTransport.failReason).toBe('pair-rejected')
  })

  // 配对请求本身失败（断网）：不是码的问题
  it('RemoteTransport_PairNetworkReason_009', async () => {
    fetchMock.mockRejectedValue(new TypeError('network error'))
    navigate(`/#pair=${PAIR_CODE}`)
    await new Promise(r => setTimeout(r, 0))

    expect(remoteTransport.failReason).toBe('pair-network')
  })

  // 当前连接可用时收到失败的新配对：不能把好端端的连接打成失效
  it('RemoteTransport_PairFailKeepsConnected_010', async () => {
    const ws = connect(OLD_TOKEN)
    const events: unknown[] = []
    const un = await remoteTransport.listen('pair-failed', e => events.push(e))
    pairResponse({ ok: false, error: 'invalid or expired code' })

    navigate(`/#pair=${PAIR_CODE}`)
    await new Promise(r => setTimeout(r, 0))
    un()

    expect(events).toEqual([{ reason: 'pair-rejected' }])
    expect(remoteTransport.connState).toBe('connected')
    expect(lastWs()).toBe(ws)
  })

  // 配对在途时用户手动选了别的设备：旧配对结果回来后不得再建连
  it('RemoteTransport_StalePairResultDropped_011', async () => {
    let resolve!: (v: unknown) => void
    fetchMock.mockReturnValue(new Promise(r => (resolve = r)))
    navigate(`/#pair=${PAIR_CODE}`)

    connect('manual-token')
    resolve({ ok: true, json: () => Promise.resolve({ ok: true, token: NEW_TOKEN }) })
    await new Promise(r => setTimeout(r, 0))

    expect(FakeWebSocket.instances.length).toBe(1)
    expect(helloTokens()).not.toContain(NEW_TOKEN)
  })

  // relay 文案「relay busy」（连接数上限）：瞬态，退避重连，不判令牌失效
  it('RemoteTransport_RelayBusyRetries_012', async () => {
    vi.useFakeTimers()
    const ws = connect(OLD_TOKEN)
    ws.serverSend({ type: 'hello.error', error: 'relay busy' })

    expect(remoteTransport.connState).not.toBe('denied')
    const saved = JSON.parse(store.get('ccbox-desktops') ?? '[]') as Array<{ invalid?: boolean }>
    expect(saved.some(d => d.invalid === true)).toBe(false)

    await vi.advanceTimersByTimeAsync(1_100)
    expect(FakeWebSocket.instances.length).toBe(2)
  })

  // 桌面拒绝令牌：原因记为令牌失效
  it('RemoteTransport_HelloErrorFailReason_013', async () => {
    const ws = connect(OLD_TOKEN)
    ws.serverSend({ type: 'hello.error', error: 'invalid token' })

    expect(remoteTransport.connState).toBe('denied')
    expect(remoteTransport.failReason).toBe('auth-invalid')
  })

  // 中继告知身份已作废（电脑刷新过密钥，上报了旧 device_id）：不再重连，
  // 原因记为身份已作废（否则会一直空转重连，用户只看到「重连中」）
  it('RemoteTransport_RetiredIdentityStopsRetrying_014', async () => {
    vi.useFakeTimers()
    const ws = connect(OLD_TOKEN)
    ws.serverSend({ type: 'device.status', online: false, rotated: true })

    expect(remoteTransport.failReason).toBe('identity-rotated')
    await vi.advanceTimersByTimeAsync(5_000)
    expect(FakeWebSocket.instances.length).toBe(1)
  })

  // 同一台电脑刷新密钥后重新配对：device_id 变了，但配对应答里的设备码不变 →
  // 旧条目被新条目取代，列表里不再出现两台「主机」
  it('RemoteTransport_RePairReplacesOldEntryByDeviceCode_015', async () => {
    const code = 'abcdef0123456789abcdef0123456789'
    const ws = connect(OLD_TOKEN)
    ws.serverSend({ type: 'hello.ok', ver: 1, deviceName: 'D', connId: 7, revision: 0, deviceCode: code })
    pairResponse({ ok: true, token: NEW_TOKEN, deviceCode: code })

    navigate(`/#pair=${PAIR_CODE}`)
    await new Promise(r => setTimeout(r, 0))

    const saved = JSON.parse(store.get('ccbox-desktops') ?? '[]') as Array<{ token: string }>
    expect(saved.map(d => d.token)).toEqual([NEW_TOKEN])
  })
})
