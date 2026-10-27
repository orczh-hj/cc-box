/**
 * remote-transport 控制权与 revision 机制测试
 *
 * 覆盖：hello.ok 快照（connId/controllerId/revision 基线）、controller-changed
 * 更新与接管确认（remote-connected 全量重同步语义）、state-changed 的
 * revision 单调过滤（旧事件不覆盖新状态）。
 */
import { describe, it, expect, beforeEach, vi } from 'vitest'
import type { Unlisten } from '../../src/api/transport'

// ---- WebSocket mock：收集实例供测试注入服务端消息 ----
class FakeWebSocket {
  static instances: FakeWebSocket[] = []
  static SENT: Array<Record<string, unknown>> = []
  // WebSocket 状态常量（sendClient 等实现按 WebSocket.OPEN 判定就绪）
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
    this.onclose?.()
  }
  // 测试辅助
  serverSend(msg: Record<string, unknown>) {
    this.onmessage?.({ data: JSON.stringify(msg) })
  }
  serverOpen() {
    this.readyState = 1
    this.onopen?.()
  }
}

vi.stubGlobal('WebSocket', FakeWebSocket as unknown as typeof WebSocket)
// location 同源（jsdom 默认 http://localhost/）
vi.stubGlobal('localStorage', {
  getItem: () => null,
  setItem: () => {},
  removeItem: () => {},
})

const { remoteTransport } = await import('../../src/api/remote-transport')

function lastWs(): FakeWebSocket {
  return FakeWebSocket.instances[FakeWebSocket.instances.length - 1]
}

describe('remote-transport 控制权与 revision', () => {
  beforeEach(() => {
    FakeWebSocket.instances = []
    FakeWebSocket.SENT = []
  })

  /** 建立连接并完成 hello 握手，返回注入用 ws */
  async function connect(helloOk: Record<string, unknown> = {}): Promise<FakeWebSocket> {
    const events: string[] = []
    const un = (await remoteTransport.listen('remote-connected', () => events.push('rc'))) as Unlisten
    remoteTransport.connectWithToken('test-token')
    const ws = lastWs()
    ws.serverOpen()
    ws.serverSend({ type: 'hello.ok', ver: 1, deviceName: 'D', connId: 7, controller: 7, revision: 3, ...helloOk })
    un()
    return ws
  }

  // hello.ok 快照：connId/controllerLabel 与 revision 基线
  it('RemoteTransport_HelloOk快照_001', async () => {
    const ws = await connect()
    expect(remoteTransport.connId).toBe(7)
    expect(remoteTransport.controllerLabel).toBe('7')
    // hello 后应已发出 ping 之前的事件；连接保持
    expect(ws.readyState).toBe(1)
  })

  // hello 时无人持有：controllerLabel 为 ''（无遮罩）
  it('RemoteTransport_无人持有_002', async () => {
    await connect({ controller: null, connId: 9, revision: 0 })
    expect(remoteTransport.connId).toBe(9)
    expect(remoteTransport.controllerLabel).toBe('')
  })

  // 桌面持有（controller=0 保留值）：标签归一化为 'desktop'（遮罩判定用）
  it('RemoteTransport_桌面持有标签_008', async () => {
    await connect({ controller: 0, connId: 5, revision: 1 })
    expect(remoteTransport.controllerLabel).toBe('desktop')
  })

  // controller-changed 更新控制者并分发给订阅者（遮罩判定依据）
  it('RemoteTransport_ControllerChanged更新_003', async () => {
    const ws = await connect({ controller: null, connId: 5, revision: 1 })
    const received: string[] = []
    const un = await remoteTransport.listen<{ id: string }>('controller-changed', ({ id }) => received.push(id))

    ws.serverSend({ type: 'rpc.event', name: 'controller-changed', payload: { id: '12', data: '2' } })
    expect(remoteTransport.controllerLabel).toBe('12')
    expect(received).toEqual(['12'])
    un()
  })

  // 接管确认（controller 变为自己）→ 按"重连"语义 dispatch remote-connected（全量重同步）
  it('RemoteTransport_接管确认触发全量重同步_004', async () => {
    const ws = await connect({ controller: null, connId: 5, revision: 1 })
    let resync = 0
    const un = await remoteTransport.listen('remote-connected', () => resync++)

    // 他人持有 → 自己（接管成功）
    ws.serverSend({ type: 'rpc.event', name: 'controller-changed', payload: { id: '8', data: '2' } })
    expect(resync).toBe(0)
    ws.serverSend({ type: 'rpc.event', name: 'controller-changed', payload: { id: '5', data: '3' } })
    expect(resync).toBe(1)
    expect(remoteTransport.controllerLabel).toBe('5')
    un()
  })

  // 切换设备：旧 socket 必须被摘除回调并关闭，否则其 onclose 会按新 token
  // 再开一条连接（同 token 双连接互踢），在途帧也会串到新设备
  it('RemoteTransport_切换设备关闭旧连接_009', async () => {
    const ws1 = await connect()
    remoteTransport.connectWithToken('other-token')
    const ws2 = lastWs()
    expect(ws2).not.toBe(ws1)
    expect(FakeWebSocket.instances.length).toBe(2)
    // 旧连接回调已摘除：即使触发 onclose 也不会排重连
    expect(ws1.onclose).toBeNull()
    expect(ws1.onmessage).toBeNull()
    expect(ws1.onopen).toBeNull()
  })

  // 释放（空 id）：controllerId 置 null，不触发重同步
  it('RemoteTransport_释放广播_005', async () => {
    const ws = await connect({ controller: 8, connId: 5, revision: 4 })
    let resync = 0
    const un = await remoteTransport.listen('remote-connected', () => resync++)

    ws.serverSend({ type: 'rpc.event', name: 'controller-changed', payload: { id: '', data: '5' } })
    expect(remoteTransport.controllerLabel).toBe('')
    expect(resync).toBe(0)
    un()
  })

  // revision 单调过滤：旧版本（≤ 已应用）的 state-changed 被丢弃
  it('RemoteTransport_旧状态事件被过滤_006', async () => {
    const ws = await connect({ controller: null, connId: 5, revision: 10 })
    const sections: string[] = []
    const un = await remoteTransport.listen<{ id: string }>('state-changed', ({ id }) => sections.push(id))

    // 旧事件（revision 9 < 基线 10）→ 丢弃
    ws.serverSend({ type: 'rpc.event', name: 'state-changed', payload: { id: 'skills', data: '9' } })
    expect(sections).toEqual([])
    // 重复当前版本（10 ≤ 10，回环/重放）→ 丢弃
    ws.serverSend({ type: 'rpc.event', name: 'state-changed', payload: { id: 'skills', data: '10' } })
    expect(sections).toEqual([])
    // 新版本 → 应用并推进基线
    ws.serverSend({ type: 'rpc.event', name: 'state-changed', payload: { id: 'agents', data: '11' } })
    expect(sections).toEqual(['agents'])
    un()
  })

  // 版本跳变（gap）：中间事件可能丢失 → 触发 state-resync-needed 权威重拉，且跳变事件本身不丢弃
  it('RemoteTransport_版本跳变触发重同步_009', async () => {
    const ws = await connect({ controller: null, connId: 5, revision: 10 })
    const sections: string[] = []
    let resync = 0
    const un1 = await remoteTransport.listen<{ id: string }>('state-changed', ({ id }) => sections.push(id))
    const un2 = await remoteTransport.listen('state-resync-needed', () => resync++)

    // 正常顺序（11 = 10+1）：应用，无重同步
    ws.serverSend({ type: 'rpc.event', name: 'state-changed', payload: { id: 'skills', data: '11' } })
    expect(sections).toEqual(['skills'])
    expect(resync).toBe(0)
    // 跳变（14 > 11+1）：事件本身应用 + 触发权威重同步（skills rev12/13 可能已丢）
    ws.serverSend({ type: 'rpc.event', name: 'state-changed', payload: { id: 'providers', data: '14' } })
    expect(sections).toEqual(['skills', 'providers'])
    expect(resync).toBe(1)
    un1()
    un2()
  })

  // sessions 全量应答推进 revision 基线（防其后旧事件覆盖全量结果）
  it('RemoteTransport_Sessions推进基线_007', async () => {
    const ws = await connect({ controller: null, connId: 5, revision: 1 })
    const sections: string[] = []
    const un = await remoteTransport.listen<{ id: string }>('state-changed', ({ id }) => sections.push(id))

    // 全量应答基线 20
    ws.serverSend({ type: 'sessions', revision: 20, list: [] })
    // 之后到达的 revision 15 旧事件被丢弃
    ws.serverSend({ type: 'rpc.event', name: 'state-changed', payload: { id: 'mcp', data: '15' } })
    expect(sections).toEqual([])
    // revision 21 新事件应用
    ws.serverSend({ type: 'rpc.event', name: 'state-changed', payload: { id: 'mcp', data: '21' } })
    expect(sections).toEqual(['mcp'])
    un()
  })
})

describe('remote-transport sessionKill quiet 参数', () => {
  beforeEach(() => {
    FakeWebSocket.instances = []
    FakeWebSocket.SENT = []
  })

  /** 建立连接并完成 hello 握手，返回过滤掉 hello 的已发送消息列表 */
  async function connectAndCollect(): Promise<void> {
    remoteTransport.connectWithToken('test-token')
    const ws = lastWs()
    ws.serverOpen()
    ws.serverSend({ type: 'hello.ok', ver: 1, deviceName: 'D', connId: 7, controller: 7, revision: 3 })
  }

  function sentKills(): Array<Record<string, unknown>> {
    return FakeWebSocket.SENT.filter(m => m.type === 'session.kill')
  }

  // 显式关闭（默认）：不带 quiet 字段，服务端广播 remote-session-closed 各端删 Tab
  it('RemoteTransport_SessionKill默认显式关闭_001', async () => {
    await connectAndCollect()
    await remoteTransport.sessionKill('pty-1')
    expect(sentKills()).toEqual([{ type: 'session.kill', id: 'pty-1' }])
  })

  // 重启场景内部清理（quiet=true）：带 quiet 字段，不触发各端删 Tab（保留 Tab 等 respawn）
  it('RemoteTransport_SessionKillQuiet不删Tab_002', async () => {
    await connectAndCollect()
    await remoteTransport.sessionKill('pty-1', true)
    expect(sentKills()).toEqual([{ type: 'session.kill', id: 'pty-1', quiet: true }])
  })
})

describe('remote-transport 输出序号与缺口', () => {
  beforeEach(() => {
    FakeWebSocket.instances = []
    FakeWebSocket.SENT = []
  })

  it('RemoteTransport_SeqGapMarker_010', async () => {
    remoteTransport.connectWithToken('seq-token')
    const ws = lastWs()
    ws.serverOpen()
    ws.serverSend({
      type: 'hello.ok',
      ver: 1,
      deviceName: 'D',
      connId: 1,
      controller: null,
      revision: 0,
    })

    const outputs: string[] = []
    const un = await remoteTransport.listen<{ id: string; data: string }>(
      'pty-output',
      ({ data }) => outputs.push(data),
    )

    ws.serverSend({ type: 'session.attached', id: 'p1', cols: 80, rows: 24, tail: 'base', tailSeq: 5 })
    ws.serverSend({ type: 'pty.output', id: 'p1', data: 'ok', seq: 6 })
    // 与快照重叠的旧序号：丢弃
    ws.serverSend({ type: 'pty.output', id: 'p1', data: 'dup', seq: 6 })
    // 7/8 丢失：可见缺口提示 + 后续内容继续
    ws.serverSend({ type: 'pty.output', id: 'p1', data: 'next', seq: 9 })

    const joined = outputs.join('')
    expect(joined).toContain('base')
    expect(joined).toContain('ok')
    expect(joined).not.toContain('dup')
    expect(joined).toContain('跳过 2 段')
    expect(joined).toContain('next')
    un()
  })

  it('RemoteTransport_ClaimDoesNotLowerSeqBaseline_011', async () => {
    remoteTransport.connectWithToken('seq-token')
    const ws = lastWs()
    ws.serverOpen()
    ws.serverSend({
      type: 'hello.ok',
      ver: 1,
      deviceName: 'D',
      connId: 1,
      controller: null,
      revision: 0,
    })

    const outputs: string[] = []
    const un = await remoteTransport.listen<{ id: string; data: string }>(
      'pty-output',
      ({ data }) => outputs.push(data),
    )

    ws.serverSend({ type: 'session.attached', id: 'p2', cols: 80, rows: 24, tail: 'base', tailSeq: 5 })
    ws.serverSend({ type: 'pty.output', id: 'p2', data: 'live-6', seq: 6 })
    ws.serverSend({ type: 'pty.output', id: 'p2', data: 'live-7', seq: 7 })

    // claim 重 attach 携带旧 tailSeq；不能让基线从 7 降回 5
    void remoteTransport.sessionClaim('p2')
    ws.serverSend({ type: 'session.attached', id: 'p2', cols: 80, rows: 24, tail: 'stale', tailSeq: 5 })
    ws.serverSend({ type: 'pty.output', id: 'p2', data: 'live-8', seq: 8 })

    const joined = outputs.join('')
    expect(joined).toContain('live-8')
    expect(joined).not.toContain('stale')
    expect(joined).not.toContain('输出中断')
    un()
  })
})

describe('remote-transport 设备离线与 attach 拒绝（重启实测回归）', () => {
  beforeEach(() => {
    FakeWebSocket.instances = []
    FakeWebSocket.SENT = []
  })

  /** 建立连接并完成 hello 握手，返回注入用 ws（与首个 describe 的辅助同构） */
  async function connect(helloOk: Record<string, unknown> = {}): Promise<FakeWebSocket> {
    remoteTransport.connectWithToken('test-token')
    const ws = lastWs()
    ws.serverOpen()
    ws.serverSend({ type: 'hello.ok', ver: 1, deviceName: 'D', connId: 7, controller: 7, revision: 3, ...helloOk })
    return ws
  }

  /**
   * Bug 回归（win 重启 + Broker 自启）：桌面 relay 短暂断连重连后，relay 已移除
   * 手机桥但保持 WS——前端若无动作，rpc 经僵尸连接发出即被丢弃（实测
   * spawn_new_instance 15s 超时，Broker 侧零日志），直到 ~100s 后心跳死亡才重连。
   * 期望：收到 device.status online:false 立即主动断开并按最小退避重连；
   * online:true 不触发断开。
   */
  it('RemoteTransport_设备离线主动重连_001', async () => {
    vi.useFakeTimers()
    try {
      const ws = await connect()
      expect(remoteTransport.connState).toBe('connected')

      // 桌面在线通知：连接保持
      ws.serverSend({ type: 'device.status', online: true })
      expect(remoteTransport.connState).toBe('connected')

      // 桌面离线：立即断开僵尸 WS 进入重连
      ws.serverSend({ type: 'device.status', online: false })
      expect(remoteTransport.connState).toBe('reconnecting')

      // retry 归零 → 最小退避 1s 后建立新连接，握手后恢复 connected
      await vi.advanceTimersByTimeAsync(1_100)
      const ws2 = lastWs()
      expect(ws2).not.toBe(ws)
      ws2.serverOpen()
      ws2.serverSend({ type: 'hello.ok', ver: 1, deviceName: 'D', connId: 8, controller: 8, revision: 3 })
      expect(remoteTransport.connState).toBe('connected')
    } finally {
      vi.useRealTimers()
    }
  })

  /**
   * Bug 回归（手机刷新终端白屏）：桌面持控制权时手机 attach 被 gate_write 拒绝，
   * 服务端已回 session.attach.denied——前端必须分发事件（会话层置 Tab 遮罩 +
   * 接管入口），否则 attach 悬挂、终端永久白屏。
   */
  it('RemoteTransport_attach被拒分发事件_002', async () => {
    const ws = await connect({ controller: 0, connId: 9 })
    const got: Array<{ id: string; reason: string }> = []
    const un = (await remoteTransport.listen('session-attach-denied', (p) => {
      got.push(p as { id: string; reason: string })
    })) as Unlisten
    ws.serverSend({
      type: 'session.attach.denied',
      id: 'pty-1',
      reason: 'controller held by another device',
    })
    un()
    expect(got).toEqual([{ id: 'pty-1', reason: 'controller held by another device' }])
  })
})

describe('remote-transport 会话剥离分发（同类问题回归）', () => {
  beforeEach(() => {
    FakeWebSocket.instances = []
    FakeWebSocket.SENT = []
  })

  async function connect(helloOk: Record<string, unknown> = {}): Promise<FakeWebSocket> {
    remoteTransport.connectWithToken('test-token')
    const ws = lastWs()
    ws.serverOpen()
    ws.serverSend({ type: 'hello.ok', ver: 1, deviceName: 'D', connId: 7, controller: 7, revision: 3, ...helloOk })
    return ws
  }

  /**
   * 宿主窗口关闭 → Broker 广播 session.detached：必须分发事件并清输出序号
   * 基线（否则 Tab 冻结在 running、会话层无感知）
   */
  it('RemoteTransport_会话剥离分发事件_003', async () => {
    const ws = await connect()
    // 先建立序号基线（模拟已收到输出）
    ws.serverSend({ type: 'session.attached', id: 'p9', cols: 80, rows: 24, tail: 'x', tailSeq: 4 })
    const got: Array<{ id: string; reason: string }> = []
    const un = (await remoteTransport.listen('session-detached', (p) => {
      got.push(p as { id: string; reason: string })
    })) as Unlisten
    ws.serverSend({ type: 'session.detached', id: 'p9', reason: 'instance disconnected' })
    un()
    expect(got).toEqual([{ id: 'p9', reason: 'instance disconnected' }])
  })
})
