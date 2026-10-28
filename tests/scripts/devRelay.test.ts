// @vitest-environment node
import { describe, it, expect } from 'vitest'
import { lanIp } from '../../scripts/lib/lan-ip.mjs'
import { resolveRelayTarget } from '../../scripts/lib/relay-target.mjs'

type Addr = { family: string | number; internal: boolean; address: string }
const v4 = (address: string, extra: Partial<Addr> = {}): Addr => ({
  family: 'IPv4',
  internal: false,
  address,
  ...extra,
})

describe('lanIp（dev:relay 局域网地址探测）', () => {
  // macOS 虚拟接口（feth*）排在 en* 之前且带私网地址时，不能选中它——
  // 否则手机配对 URL 指向不可达地址
  it('DevRelay_LanIp_SkipVirtualInterface_001', () => {
    expect(
      lanIp({
        feth2024: [v4('192.168.191.8')],
        en1: [v4('192.168.5.119')],
      }),
    ).toBe('192.168.5.119')
  })

  // 多个物理接口：en* 优先于其他物理网卡
  it('DevRelay_LanIp_PreferEnInterface_001', () => {
    expect(
      lanIp({
        eth9: [v4('10.0.0.5')],
        en0: [v4('192.168.1.20')],
      }),
    ).toBe('192.168.1.20')
  })

  // en* 是公网地址时，物理网卡的私网地址优先（手机同网段才可达）
  it('DevRelay_LanIp_PreferPrivateAddress_001', () => {
    expect(
      lanIp({
        en0: [v4('203.0.113.7')],
        en5: [v4('192.168.3.9')],
      }),
    ).toBe('192.168.3.9')
  })

  // 只有虚拟接口时兜底，且优先私网段
  it('DevRelay_LanIp_FallbackVirtual_001', () => {
    expect(
      lanIp({
        utun3: [v4('100.64.0.2')],
        bridge0: [v4('192.168.64.1')],
      }),
    ).toBe('192.168.64.1')
  })

  // loopback、链路本地 169.254.*、IPv6 一律排除
  it('DevRelay_LanIp_ExcludeSpecialAddresses_001', () => {
    expect(
      lanIp({
        lo0: [v4('127.0.0.1', { internal: true })],
        en0: [v4('169.254.10.2')],
        en1: [{ family: 'IPv6', internal: false, address: 'fe80::1' }],
      }),
    ).toBeUndefined()
  })

  // Node 18 之前 family 是数字 4，需兼容
  it('DevRelay_LanIp_LegacyFamilyNumber_001', () => {
    expect(lanIp({ en0: [v4('192.168.0.8', { family: 4 })] })).toBe('192.168.0.8')
  })
})

describe('resolveRelayTarget（dev:relay 共享 relay）', () => {
  // 显式 CC_BOX_RELAY_URL：连接共享 relay，不启动本地 relay
  it('DevRelay_RelayTarget_SharedOverride_001', () => {
    expect(resolveRelayTarget({ override: '192.168.5.121:8800', ip: '', port: 8800 }))
      .toEqual({
        shared: true,
        relayUrl: '192.168.5.121:8800',
        phoneUrl: 'http://192.168.5.121:8800',
      })
  })

  // 覆盖值带 scheme：手机 URL 原样使用（https 不被改写）
  it('DevRelay_RelayTarget_SharedWithScheme_002', () => {
    expect(resolveRelayTarget({ override: 'https://relay.example.com', ip: '', port: 8800 }))
      .toEqual({
        shared: true,
        relayUrl: 'https://relay.example.com',
        phoneUrl: 'https://relay.example.com',
      })
  })

  // 无覆盖：使用探测到的局域网 IP 启动本地 relay
  it('DevRelay_RelayTarget_Local_003', () => {
    expect(resolveRelayTarget({ override: '', ip: '192.168.5.119', port: 8800 }))
      .toEqual({
        shared: false,
        relayUrl: '192.168.5.119:8800',
        phoneUrl: 'http://192.168.5.119:8800',
      })
  })

  // 无覆盖且无 IP：交给调用方报错中止
  it('DevRelay_RelayTarget_NoIp_004', () => {
    expect(resolveRelayTarget({ override: undefined, ip: undefined, port: 8800 }))
      .toEqual({ shared: false, relayUrl: '', phoneUrl: '' })
  })

  // 空白覆盖视为未设置，回落到本地 relay
  it('DevRelay_RelayTarget_WhitespaceFallsBack_005', () => {
    expect(resolveRelayTarget({ override: '   ', ip: '192.168.5.119', port: 8800 }))
      .toEqual({
        shared: false,
        relayUrl: '192.168.5.119:8800',
        phoneUrl: 'http://192.168.5.119:8800',
      })
  })

  // ws/wss 覆盖：桌面端保留原样，手机页面映射为 http/https
  it('DevRelay_RelayTarget_WsScheme_006', () => {
    expect(resolveRelayTarget({ override: 'ws://192.168.5.121:8800', ip: '', port: 8800 }))
      .toEqual({
        shared: true,
        relayUrl: 'ws://192.168.5.121:8800',
        phoneUrl: 'http://192.168.5.121:8800',
      })
    expect(resolveRelayTarget({ override: 'wss://relay.example.com', ip: '', port: 8800 }))
      .toEqual({
        shared: true,
        relayUrl: 'wss://relay.example.com',
        phoneUrl: 'https://relay.example.com',
      })
  })
})
