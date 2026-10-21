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
