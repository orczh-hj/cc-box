import os from 'node:os'

/**
 * 局域网 IPv4 探测（dev:relay 用）。
 *
 * macOS 上除物理网卡外还挂着大量虚拟接口（feth= FakeEthernet、utun= VPN、
 * bridge / awdl / llw = AirDrop、vmenet / vmnet = 虚拟机/Docker、anpi = Apple
 * 私有接口）。它们常带 192.168.x.x 地址且枚举顺序在 en0 之前，直接取「第一个
 * 私网 IPv4」会把手机配对 URL 指到手机不可达的地址。因此按接口名分层：
 * en 物理接口优先，虚拟接口只在完全没有其他候选时兜底。
 */

/** 虚拟/非物理接口前缀：即使带私网地址也不参与优先排序 */
const VIRTUAL_PREFIXES = [
  'lo',
  'feth',
  'utun',
  'gif',
  'stf',
  'bridge',
  'awdl',
  'llw',
  'vmenet',
  'vmnet',
  'docker',
  'tun',
  'tap',
  'anpi',
]

function isVirtual(name) {
  return VIRTUAL_PREFIXES.some((p) => name.startsWith(p))
}

function isPrivate(ip) {
  return ip.startsWith('192.168.') || ip.startsWith('10.') || ip.startsWith('172.')
}

/**
 * 从 networkInterfaces() 结果里选一个局域网 IPv4。
 * 返回 undefined 表示没有可用地址（调用方负责提示并中止）。
 */
export function lanIp(interfaces = os.networkInterfaces()) {
  const physical = []
  const virtual = []
  for (const [name, addrs] of Object.entries(interfaces)) {
    for (const a of addrs ?? []) {
      // a.family 在 Node 18+ 为 'IPv4'，旧版为 4；两种都认
      if (a.family !== 'IPv4' && a.family !== 4) continue
      if (a.internal || a.address.startsWith('169.254.')) continue
      const entry = { name, address: a.address }
      if (isVirtual(name)) virtual.push(entry)
      else physical.push(entry)
    }
  }
  // en*+私网 > 其他物理+私网 > en* 非私网 > 其他物理非私网
  const rank = (e) => (e.name.startsWith('en') ? 0 : 2) + (isPrivate(e.address) ? 0 : 1)
  physical.sort((a, b) => rank(a) - rank(b))
  if (physical.length > 0) return physical[0].address
  // 仅剩虚拟接口：优先带私网段的那条，尽量让地址可用
  virtual.sort((a, b) => Number(!isPrivate(a.address)) - Number(!isPrivate(b.address)))
  return virtual[0]?.address
}
