// @vitest-environment node
import { describe, it, expect } from 'vitest'
import { lanIp } from '../../scripts/lib/lan-ip.mjs'

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
