/**
 * TokenGate 操作指导测试
 *
 * 覆盖「远程失效时不再让用户输 token，而是按失败原因给操作指导」：
 * - 任何状态下都没有令牌输入框
 * - 标题随失败原因变化；只有令牌失效才提「刷新密钥」（它会断开所有已配对设备）
 * - 设备离线的重连循环超过 10s 后退出 spinner 给指导（阈值是设计值）
 *
 * 直接驱动真实的 remoteTransport 单例（FakeWebSocket + stub fetch）。
 */
import { describe, it, expect, beforeEach, afterEach, vi } from 'vitest'
import { mount } from '@vue/test-utils'
import TokenGate from '@/components/TokenGate.vue'

vi.mock('vue-i18n', () => ({ useI18n: () => ({ t: (key: string) => key }) }))

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

const store = new Map<string, string>()
vi.stubGlobal('localStorage', {
  getItem: (k: string) => store.get(k) ?? null,
  setItem: (k: string, v: string) => void store.set(k, v),
  removeItem: (k: string) => void store.delete(k),
})

const { remoteTransport } = await import('@/api/remote-transport')

const TOKEN = `${'a'.repeat(32)}.${'b'.repeat(32)}`
const fetchMock = vi.fn()

function lastWs(): FakeWebSocket {
  return FakeWebSocket.instances[FakeWebSocket.instances.length - 1]
}

/** 以 token 建连并完成握手 */
function connect(hello: Record<string, unknown>): FakeWebSocket {
  remoteTransport.connectWithToken(TOKEN)
  const ws = lastWs()
  ws.serverOpen()
  ws.serverSend({ type: hello.type, ver: 1, deviceName: 'D', connId: 7, revision: 0, ...hello })
  return ws
}

/** 一轮设备离线循环：1s 后重连、握手、relay 告知离线、断开 */
async function tickOffline(): Promise<void> {
  await vi.advanceTimersByTimeAsync(1_000)
  const ws = lastWs()
  ws.serverOpen()
  ws.serverSend({ type: 'device.status', online: false })
}

function stepsOf(wrapper: ReturnType<typeof mount>) {
  return wrapper.findAll('.gate-steps li').map(li => li.text())
}

describe('TokenGate 操作指导', () => {
  beforeEach(() => {
    vi.useFakeTimers()
    FakeWebSocket.instances = []
    store.clear()
    fetchMock.mockReset()
    vi.stubGlobal('fetch', fetchMock)
    history.replaceState(null, '', '/')
    remoteTransport.close()
    vi.advanceTimersByTime(0)
  })
  afterEach(() => {
    vi.useRealTimers()
  })

  // 首次进入：没有输入框，只有配对步骤
  it('TokenGate_NoTokenInput_001', () => {
    const wrapper = mount(TokenGate, { props: { autoBoot: false } })
    expect(wrapper.find('input').exists()).toBe(false)
    expect(wrapper.find('.gate-subtitle').text()).toBe('gatePairTitle')
    expect(stepsOf(wrapper)).toEqual(['gateStepOpen', 'gateStepScan', 'gateStepRotate'])
    wrapper.unmount()
  })

  // 桌面拒绝令牌：标题说明令牌失效
  it('TokenGate_AuthInvalidGuide_002', async () => {
    const wrapper = mount(TokenGate, { props: { autoBoot: false } })
    remoteTransport.connectWithToken(TOKEN)
    const ws = lastWs()
    ws.serverOpen()
    ws.serverSend({ type: 'hello.error', error: 'invalid token' })
    await vi.advanceTimersByTime(0)

    expect(wrapper.find('.gate-subtitle').text()).toBe('gateAuthInvalidTitle')
    wrapper.unmount()
  })

  // 连接码对应设备离线：码已被消费，指导必须是「更新连接码」，且不提「刷新密钥」
  it('TokenGate_PairOfflineGuide_003', async () => {
    const wrapper = mount(TokenGate, { props: { autoBoot: false } })
    fetchMock.mockResolvedValue({ ok: true, json: () => Promise.resolve({ ok: false, error: 'device offline' }) })
    history.replaceState(null, '', '/#pair=A30A3214')
    window.dispatchEvent(new Event('hashchange'))
    await vi.advanceTimersByTimeAsync(0)

    expect(wrapper.find('.gate-subtitle').text()).toBe('gateDeviceOfflineTitle')
    expect(stepsOf(wrapper)).toEqual(['gateStepOpen', 'gateStepScanFresh'])
    wrapper.unmount()
  })

  // 设备离线的重连循环：10s 内保持 spinner，超过后退出 spinner 给指导
  it('TokenGate_StallShowsGuide_004', async () => {
    const wrapper = mount(TokenGate, { props: { autoBoot: false } })
    const ws = connect({ type: 'hello.error', error: 'relay busy' })
    void ws
    for (let i = 0; i < 5; i++) await tickOffline()

    expect(wrapper.find('.gate-state').exists()).toBe(true)
    expect(wrapper.find('.gate-subtitle').text()).not.toBe('gateStalledTitle')

    for (let i = 0; i < 6; i++) await tickOffline()
    expect(wrapper.find('.gate-state').exists()).toBe(false)
    expect(wrapper.find('.gate-subtitle').text()).toBe('gateStalledTitle')
    wrapper.unmount()
  })

  // 传输层已处于 denied 时挂载（失效覆盖层）：直接显示原因
  it('TokenGate_OverlayReadsFailReason_005', async () => {
    remoteTransport.connectWithToken(TOKEN)
    const ws = lastWs()
    ws.serverOpen()
    ws.serverSend({ type: 'hello.error', error: 'invalid token' })
    await vi.advanceTimersByTime(0)

    const wrapper = mount(TokenGate, { props: { autoBoot: false } })
    expect(wrapper.find('.gate-subtitle').text()).toBe('gateAuthInvalidTitle')
    wrapper.unmount()
  })

  // 点设备行重新连接：回到 spinner，失效标题消失
  it('TokenGate_SelectDeviceClearsGuide_006', async () => {
    const wrapper = mount(TokenGate, { props: { autoBoot: false } })
    connect({ type: 'hello.error', error: 'invalid token' })
    await vi.advanceTimersByTime(0)
    expect(wrapper.find('.gate-subtitle').text()).toBe('gateAuthInvalidTitle')

    await wrapper.find('.device-item').trigger('click')
    expect(wrapper.find('.gate-state').exists()).toBe(true)
    expect(wrapper.find('.gate-subtitle').text()).not.toBe('gateAuthInvalidTitle')
    wrapper.unmount()
  })

  // 连上后通知父组件
  it('TokenGate_ConnectedEmits_007', async () => {
    const wrapper = mount(TokenGate, { props: { autoBoot: false } })
    connect({ type: 'hello.ok' })

    expect(wrapper.emitted('connected')).toHaveLength(1)
    wrapper.unmount()
  })
})
