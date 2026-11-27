/**
 * useConnHealth 分层映射测试
 *
 * 分层提示方案的 UI 侧核心：断开持续时长 → 提示层级。阈值是设计常量
 * （spinner <10s < overlay <60s < stalled），断言直接对照阈值边界；
 * 组合行为验证「恢复（connected）自动复位 ok，无需用户操作」。
 */
import { describe, it, expect, beforeEach, afterEach, vi } from 'vitest'
import { effectScope } from 'vue'

class FakeWebSocket {
  static instances: FakeWebSocket[] = []
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
  send() {}
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
const { connHealthLevel, useConnHealth, OVERLAY_AFTER_MS, STALLED_AFTER_MS } = await import(
  '../../src/composables/useConnHealth'
)

/** 在 effectScope 内使用 composable（onUnmounted 需要活跃 scope） */
function scoped(): ReturnType<typeof useConnHealth> {
  const scope = effectScope()
  return scope.run(() => useConnHealth())!
}

function connect(): FakeWebSocket {
  remoteTransport.connectWithToken('health-token')
  const ws = FakeWebSocket.instances[FakeWebSocket.instances.length - 1]
  ws.serverOpen()
  ws.serverSend({ type: 'hello.ok', ver: 1, deviceName: 'D', connId: 1, revision: 0 })
  return ws
}

describe('useConnHealth 阈值映射（纯函数）', () => {
  // 阈值边界：设计值 10s / 60s（硬编码字面量，勿引用实现常量——同源断言测不出阈值错误）
  it('UseConnHealth_阈值边界_001', () => {
    expect(connHealthLevel(0)).toBe('spinner')
    expect(connHealthLevel(9_999)).toBe('spinner')
    expect(connHealthLevel(10_000)).toBe('overlay')
    expect(connHealthLevel(59_999)).toBe('overlay')
    expect(connHealthLevel(60_000)).toBe('stalled')
    // 实现常量与设计值一致性（防止实现改常量而测试不知情）
    expect(OVERLAY_AFTER_MS).toBe(10_000)
    expect(STALLED_AFTER_MS).toBe(60_000)
  })
})

describe('useConnHealth 组合行为', () => {
  beforeEach(() => {
    FakeWebSocket.instances = []
  })
  afterEach(() => {
    vi.useRealTimers()
  })

  // 断开时长驱动分层升级；恢复（hello.ok）自动复位 ok——零用户操作
  it('UseConnHealth_断开分层与自动复位_002', async () => {
    vi.useFakeTimers()
    const health = scoped()
    const ws = connect()
    expect(health.level.value).toBe('ok')

    // 连接断开：立即 spinner
    ws.close()
    expect(health.level.value).toBe('spinner')

    // 10s：overlay（字面量驱动时间，勿引用实现常量）
    await vi.advanceTimersByTimeAsync(10_000)
    expect(health.level.value).toBe('overlay')

    // 累计 60s：stalled
    await vi.advanceTimersByTimeAsync(50_000)
    expect(health.level.value).toBe('stalled')

    // 自动重连成功：复位 ok
    await vi.advanceTimersByTimeAsync(1_100)
    const ws2 = FakeWebSocket.instances[FakeWebSocket.instances.length - 1]
    ws2.serverOpen()
    ws2.serverSend({ type: 'hello.ok', ver: 1, deviceName: 'D', connId: 2, revision: 0 })
    expect(health.level.value).toBe('ok')
  })

  /** 推进一轮离线循环：1s 后重连、握手、relay 告知设备离线、断开 */
  async function tickOffline(): Promise<void> {
    await vi.advanceTimersByTimeAsync(1_000)
    const ws = FakeWebSocket.instances[FakeWebSocket.instances.length - 1]
    ws.serverOpen()
    ws.serverSend({ type: 'device.status', online: false })
  }

  // 设备离线时状态每秒在 reconnecting/authenticating 间切换：断开时长照常累计
  it('UseConnHealth_离线循环照常累计_004', async () => {
    vi.useFakeTimers()
    const health = scoped()
    const ws = connect()
    ws.serverSend({ type: 'device.status', online: false })

    for (let i = 0; i < 12; i++) await tickOffline()
    expect(health.level.value).toBe('overlay')
  })

  // 用户发起新的建连（换设备/换码）：停滞重新计时，先给新一轮 10s 机会
  it('UseConnHealth_新一次建连重新计时_003', async () => {
    vi.useFakeTimers()
    const health = scoped()
    const ws = connect()
    ws.serverSend({ type: 'device.status', online: false })
    for (let i = 0; i < 12; i++) await tickOffline()
    expect(health.level.value).toBe('overlay')

    remoteTransport.connectWithToken('other-token')
    expect(health.level.value).toBe('spinner')
  })
})
