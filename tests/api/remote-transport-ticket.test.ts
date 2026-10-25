/**
 * 票据模式（?dl=）：URL 解析、以票据为 token 建连、不写 localStorage、
 * hello.ok 的 downloadScope 回调。
 */
import { describe, it, expect, beforeEach, vi } from 'vitest'

const DEVICE = 'a'.repeat(32)
const TICKET = 'b'.repeat(32)
window.history.replaceState({}, '', `/?dl=${DEVICE}.${TICKET}`)

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
const setItem = vi.fn()
vi.stubGlobal('localStorage', {
  getItem: () => null,
  setItem,
  removeItem: () => {},
})

const { remoteTransport } = await import('../../src/api/remote-transport')

function lastWs(): FakeWebSocket {
  return FakeWebSocket.instances[FakeWebSocket.instances.length - 1]
}

describe('remote-transport 票据模式', () => {
  beforeEach(() => {
    FakeWebSocket.instances = []
    FakeWebSocket.SENT = []
    setItem.mockClear()
  })

  it('Transport_TicketParsedFromQuery_1', () => {
    expect(remoteTransport.downloadTicket).toBe(`${DEVICE}.${TICKET}`)
    // 分块命令只接受裸票据（32 hex），不能带 device 前缀
    expect(remoteTransport.downloadTicketSecret).toBe(TICKET)
  })

  it('Transport_TicketBootUsesTicketToken_1', async () => {
    await remoteTransport.boot()
    const ws = lastWs()
    ws.serverOpen()
    const hello = FakeWebSocket.SENT.find((m) => m.type === 'hello')
    expect(hello?.token).toBe(`${DEVICE}.${TICKET}`)
    // 票据不是长期 token，禁止持久化
    expect(setItem).not.toHaveBeenCalled()
  })

  it('Transport_TicketHelloOkScope_1', async () => {
    const scopes: Array<{ fileName: string; size: number }> = []
    remoteTransport.onDownloadScope = (s) => scopes.push(s)

    await remoteTransport.boot()
    const ws = lastWs()
    ws.serverOpen()
    ws.serverSend({
      type: 'hello.ok',
      ver: 1,
      deviceName: 'CC-Box',
      connId: 1,
      revision: 0,
      downloadScope: { fileName: 'report.pdf', size: 1024 },
    })

    expect(scopes).toEqual([{ fileName: 'report.pdf', size: 1024 }])
    expect(remoteTransport.downloadScope).toEqual({
      fileName: 'report.pdf',
      size: 1024,
    })
  })
})
