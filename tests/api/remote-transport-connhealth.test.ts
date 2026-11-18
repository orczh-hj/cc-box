/**
 * remote-transport 连接健康检测测试（半开连接 / 回前台校验）
 *
 * 覆盖浏览器远程「网络不佳或长时间无操作回来页面假死无提示」的修复：
 * - pong 超时（页面可见时）→ 主动断开进既有退避重连
 * - pong 正常到达不误杀；后台（hidden）不判定（防 Android 低频定时器空转）
 * - 回前台：connected 校验 ping 失败重建 / pong 到达无动作 / reconnecting 清退避立即重试
 * - denied（token 失效）回前台不重试，维持「不再重连」语义
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
  close() {
    this.readyState = 3
    this.onclose?.()
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
vi.stubGlobal('localStorage', {
  getItem: () => null,
  setItem: () => {},
  removeItem: () => {},
})

const { remoteTransport } = await import('../../src/api/remote-transport')

function lastWs(): FakeWebSocket {
  return FakeWebSocket.instances[FakeWebSocket.instances.length - 1]
}

/** jsdom 的 visibilityState 默认 'visible'；用例内按需覆写 */
function setVisibility(v: 'visible' | 'hidden'): void {
  Object.defineProperty(document, 'visibilityState', {
    configurable: true,
    get: () => v,
  })
}

function resume(): void {
  setVisibility('visible')
  document.dispatchEvent(new Event('visibilitychange'))
}

/** 建立连接并完成 hello 握手 */
async function connect(): Promise<FakeWebSocket> {
  remoteTransport.connectWithToken('health-token')
  const ws = lastWs()
  ws.serverOpen()
  ws.serverSend({ type: 'hello.ok', ver: 1, deviceName: 'D', connId: 7, revision: 3 })
  return ws
}

describe('remote-transport pong 超时检测', () => {
  beforeEach(() => {
    FakeWebSocket.instances = []
    FakeWebSocket.SENT = []
    setVisibility('visible')
  })
  afterEach(() => {
    vi.useRealTimers()
  })

  // 半开连接：pong 逾期（>45s）且页面可见 → 主动 close 进退避重连
  // （此前完全依赖 onclose，弱网/锁屏后连接死亡浏览器不感知 → 假死无提示）
  it('RemoteTransport_Pong超时主动断开重连_001', async () => {
    vi.useFakeTimers()
    const ws = await connect()
    expect(remoteTransport.connState).toBe('connected')

    // 第一个 ping 周期（30s）：pong 未逾期，正常发 ping
    await vi.advanceTimersByTimeAsync(30_000)
    expect(FakeWebSocket.SENT.filter(m => m.type === 'ping').length).toBe(1)
    expect(remoteTransport.connState).toBe('connected')

    // 第二个周期（60s，距上次 pong 已 60s > 45s）：主动断开 → 退避重连
    await vi.advanceTimersByTimeAsync(30_000)
    expect(remoteTransport.connState).toBe('reconnecting')
    // 退避 1s 后新建连接
    await vi.advanceTimersByTimeAsync(1_100)
    expect(lastWs()).not.toBe(ws)
  })

  // pong 按时到达：lastPongAt 刷新，连接保持
  it('RemoteTransport_Pong到达不误杀_002', async () => {
    vi.useFakeTimers()
    const ws = await connect()
    // 三个周期内都有 pong 回来（30s ping → 服务端 pong）
    for (let i = 0; i < 3; i++) {
      await vi.advanceTimersByTimeAsync(30_000)
      ws.serverSend({ type: 'pong' })
    }
    expect(remoteTransport.connState).toBe('connected')
    expect(FakeWebSocket.instances.length).toBe(1)
  })

  // 页面后台（hidden）：不做超时判定（防低频定时器下后台空转重连耗电）
  it('RemoteTransport_后台不判定半开_003', async () => {
    vi.useFakeTimers()
    await connect()
    setVisibility('hidden')
    await vi.advanceTimersByTimeAsync(120_000)
    expect(remoteTransport.connState).toBe('connected')
    expect(FakeWebSocket.instances.length).toBe(1)
  })
})

describe('remote-transport 回前台校验', () => {
  beforeEach(() => {
    FakeWebSocket.instances = []
    FakeWebSocket.SENT = []
    setVisibility('visible')
  })
  afterEach(() => {
    vi.useRealTimers()
  })

  // 长离回归：connected 状态发校验 ping，5s 窗口无 pong → 干净重建（旧连接摘回调防双连接）
  it('RemoteTransport_回前台校验失败重建_004', async () => {
    vi.useFakeTimers()
    const ws = await connect()
    resume()
    // 校验 ping 已发出
    expect(FakeWebSocket.SENT.some(m => m.type === 'ping')).toBe(true)
    await vi.advanceTimersByTimeAsync(5_000)
    // 旧连接回调已摘除（closeSocket 防其 onclose 触发重连），新连接已发起
    expect(ws.onclose).toBeNull()
    expect(FakeWebSocket.instances.length).toBe(2)
    expect(remoteTransport.connState).toBe('connecting')
  })

  // 校验 pong 在窗口内到达：连接健康，无任何动作
  it('RemoteTransport_回前台校验通过_005', async () => {
    vi.useFakeTimers()
    const ws = await connect()
    resume()
    ws.serverSend({ type: 'pong' })
    await vi.advanceTimersByTimeAsync(10_000)
    expect(remoteTransport.connState).toBe('connected')
    expect(FakeWebSocket.instances.length).toBe(1)
  })

  // reconnecting 态回前台：清退避立即重试（不等最长 30s 的退避定时器）
  it('RemoteTransport_回前台清退避立即重试_006', async () => {
    vi.useFakeTimers()
    const ws = await connect()
    ws.close() // 服务端关闭 → 退避重连（首次 1s）
    await vi.advanceTimersByTimeAsync(1_100)
    expect(FakeWebSocket.instances.length).toBe(2)
    const ws2 = lastWs()
    ws2.close() // 第二次失败 → 退避 2s
    expect(remoteTransport.connState).toBe('reconnecting')
    // 回前台（去重窗口 1s 已过：advance 推进 Date.now）
    await vi.advanceTimersByTimeAsync(1_100)
    resume()
    // 不等 2s 退避，立即出现新连接
    expect(FakeWebSocket.instances.length).toBe(3)
    expect(remoteTransport.connState).toBe('connecting')
  })

  // denied（token 失效）后 closedByUser=true：回前台不得绕过「不再重连」语义。
  // （denied 是瞬态：hello.error 分支随后 ws.close() → onclose 的 closedByUser
  //  路径把最终态置为 closed；本质断言是「回前台不新建连接」）
  it('RemoteTransport_回前台不重试denied_007', async () => {
    vi.useFakeTimers()
    remoteTransport.connectWithToken('health-token')
    const ws = lastWs()
    ws.serverOpen()
    ws.serverSend({ type: 'hello.error', error: 'invalid token' })
    await vi.advanceTimersByTimeAsync(2_000)
    expect(remoteTransport.connState).toBe('closed')
    resume()
    await vi.advanceTimersByTimeAsync(2_000)
    expect(remoteTransport.connState).toBe('closed')
    expect(FakeWebSocket.instances.length).toBe(1)
  })
})
