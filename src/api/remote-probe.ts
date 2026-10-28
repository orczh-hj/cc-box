/**
 * 设备连通性探测：对已保存设备做一次真实 WS hello 握手。
 *
 * 与主连接共用协议（relay / LAN 都是同源 /ws + 首帧 hello），因此能同时
 * 判断「中继/设备在线」与「token 是否仍有效」——桌面刷新密钥后旧 token
 * 会得到 hello.error，只检查设备是否在线无法发现。探测拿到结果立即关闭，
 * 不重连、不写任何存储。
 */

import { PROTOCOL_VERSION } from '../../remote/protocol/protocol'
import type { StoredDesktop } from './remote-desktops'

export type ProbeStatus =
  | 'checking'
  | 'online'
  | 'offline'
  | 'auth-failed'
  | 'unreachable'
  | 'unsupported'

export interface ProbeOptions {
  timeoutMs?: number
  /** 当前页面协议（默认 location.protocol）：https 页面无法建 ws:// 连接 */
  pageProtocol?: string
}

const DEFAULT_TIMEOUT = 4_000

/** https 页面连 ws:// 会被混合内容策略拦截；此时不能判定设备不可达 */
export function isMixedContentBlocked(origin: string, pageProtocol: string): boolean {
  try {
    return pageProtocol === 'https:' && new URL(origin).protocol === 'http:'
  } catch {
    return false
  }
}

function wsUrlOf(origin: string): string | null {
  try {
    const u = new URL(origin)
    const proto = u.protocol === 'https:' ? 'wss' : u.protocol === 'http:' ? 'ws' : ''
    if (!proto) return null
    return `${proto}://${u.host}/ws`
  } catch {
    return null
  }
}

export function probeDesktop(
  desktop: Pick<StoredDesktop, 'origin' | 'token'>,
  opts: ProbeOptions = {},
): Promise<ProbeStatus> {
  const pageProtocol = opts.pageProtocol
    ?? (typeof location !== 'undefined' ? location.protocol : 'http:')
  if (isMixedContentBlocked(desktop.origin, pageProtocol)) {
    return Promise.resolve('unsupported')
  }
  const url = wsUrlOf(desktop.origin)
  if (!url) return Promise.resolve('unreachable')

  return new Promise<ProbeStatus>((resolve) => {
    let settled = false
    let ws: WebSocket | null = null
    const timer = setTimeout(() => finish('unreachable'), opts.timeoutMs ?? DEFAULT_TIMEOUT)

    function finish(status: ProbeStatus): void {
      if (settled) return
      settled = true
      clearTimeout(timer)
      try {
        ws?.close()
      } catch {
        // 已关闭
      }
      resolve(status)
    }

    try {
      ws = new WebSocket(url)
    } catch {
      finish('unreachable')
      return
    }

    ws.onopen = () => {
      ws?.send(
        JSON.stringify({
          type: 'hello',
          ver: PROTOCOL_VERSION,
          client: 'ccbox-probe',
          token: desktop.token,
        }),
      )
    }
    ws.onmessage = (ev) => {
      let msg: { type?: string; error?: string; online?: boolean }
      try {
        msg = JSON.parse(ev.data as string)
      } catch {
        return
      }
      if (msg.type === 'hello.ok') {
        finish('online')
      } else if (msg.type === 'hello.error') {
        // relay busy 不是 token 问题；其余 hello.error 视为令牌失效
        finish(/busy/i.test(msg.error ?? '') ? 'unreachable' : 'auth-failed')
      } else if (msg.type === 'device.status' && msg.online === false) {
        // 设备离线时 relay 只发这一帧并关闭连接
        finish('offline')
      }
      // device.status online:true 只表示设备在线，token 仍待桌面校验，继续等
    }
    ws.onerror = () => finish('unreachable')
    ws.onclose = () => finish('unreachable')
  })
}
