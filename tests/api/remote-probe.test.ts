import { describe, it, expect, beforeEach, afterEach, vi } from 'vitest'
import { probeDesktop, isMixedContentBlocked } from '@/api/remote-probe'

// ---- WebSocket mock：探测只关心首帧判定与关闭 ----
class FakeWebSocket {
  static instances: FakeWebSocket[] = []
  readyState = 0
  onopen: (() => void) | null = null
  onmessage: ((ev: { data: string }) => void) | null = null
  onclose: (() => void) | null = null
  onerror: (() => void) | null = null
  closed = false
  sent: string[] = []

  constructor(public url: string) {
    FakeWebSocket.instances.push(this)
  }
  send(data: string) { this.sent.push(data) }
  close() {
    this.closed = true
    this.onclose?.()
  }
  serverOpen() {
    this.readyState = 1
    this.onopen?.()
  }
  serverSend(msg: unknown) {
    this.onmessage?.({ data: JSON.stringify(msg) })
  }
  serverError() {
    this.onerror?.()
  }
}

vi.stubGlobal('WebSocket', FakeWebSocket as unknown as typeof WebSocket)

const desktop = { origin: 'http://192.168.1.20:8747', token: 'lan-token' }

describe('remote-probe 设备连通性探测', () => {
  beforeEach(() => {
    FakeWebSocket.instances = []
  })
  afterEach(() => {
    vi.useRealTimers()
  })

  // hello.ok → 在线；探测连接在拿到结论后立即关闭
  it('RemoteProbe_HelloOkOnline_001', async () => {
    const p = probeDesktop(desktop)
    const ws = FakeWebSocket.instances[0]
    expect(ws.url).toBe('ws://192.168.1.20:8747/ws')

    ws.serverOpen()
    const hello = JSON.parse(ws.sent[0])
    expect(hello).toMatchObject({ type: 'hello', token: 'lan-token' })

    ws.serverSend({ type: 'hello.ok', ver: 1, deviceName: 'D', connId: 1 })
    await expect(p).resolves.toBe('online')
    expect(ws.closed).toBe(true)
  })

  // 桌面刷新密钥：旧 token 被桌面拒绝 → 令牌失效
  it('RemoteProbe_HelloErrorAuthFailed_002', async () => {
    const p = probeDesktop(desktop)
    const ws = FakeWebSocket.instances[0]
    ws.serverOpen()
    ws.serverSend({ type: 'hello.error', error: 'invalid token' })
    await expect(p).resolves.toBe('auth-failed')
  })

  // relay busy 不是 token 问题，按不可达处理
  it('RemoteProbe_HelloErrorBusyUnreachable_003', async () => {
    const p = probeDesktop(desktop)
    FakeWebSocket.instances[0].serverOpen()
    FakeWebSocket.instances[0].serverSend({ type: 'hello.error', error: 'relay busy' })
    await expect(p).resolves.toBe('unreachable')
  })

  // relay 在线但桌面离线：只下发 device.status online:false
  it('RemoteProbe_DeviceOffline_004', async () => {
    const p = probeDesktop(desktop)
    const ws = FakeWebSocket.instances[0]
    ws.serverOpen()
    ws.serverSend({ type: 'device.status', online: false })
    await expect(p).resolves.toBe('offline')
  })

  // device.status online:true 只说明设备在线，仍需等桌面校验 token
  it('RemoteProbe_DeviceOnlineWaitsHello_005', async () => {
    const p = probeDesktop(desktop)
    const ws = FakeWebSocket.instances[0]
    ws.serverOpen()
    ws.serverSend({ type: 'device.status', online: true })
    ws.serverSend({ type: 'hello.ok', ver: 1, deviceName: 'D', connId: 2 })
    await expect(p).resolves.toBe('online')
  })

  // 无响应超时 → 不可达
  it('RemoteProbe_TimeoutUnreachable_006', async () => {
    vi.useFakeTimers()
    const p = probeDesktop(desktop, { timeoutMs: 100 })
    FakeWebSocket.instances[0].serverOpen()
    vi.advanceTimersByTime(100)
    await expect(p).resolves.toBe('unreachable')
  })

  // 连接错误 → 不可达
  it('RemoteProbe_ErrorUnreachable_007', async () => {
    const p = probeDesktop(desktop)
    FakeWebSocket.instances[0].serverError()
    await expect(p).resolves.toBe('unreachable')
  })

  // https 页面无法建 ws:// 连接（混合内容）：标 unsupported，不能误判为设备不可达
  it('RemoteProbe_MixedContentUnsupported_008', async () => {
    await expect(probeDesktop(desktop, { pageProtocol: 'https:' })).resolves.toBe('unsupported')
    expect(FakeWebSocket.instances).toHaveLength(0)
    expect(isMixedContentBlocked(desktop.origin, 'https:')).toBe(true)
    expect(isMixedContentBlocked(desktop.origin, 'http:')).toBe(false)
  })

  // https 目标映射为 wss
  it('RemoteProbe_SecureOriginUsesWss_009', async () => {
    const p = probeDesktop({ origin: 'https://relay.example:8800', token: 'x' })
    expect(FakeWebSocket.instances[0].url).toBe('wss://relay.example:8800/ws')
    FakeWebSocket.instances[0].serverError()
    await expect(p).resolves.toBe('unreachable')
  })
})
