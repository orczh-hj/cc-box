/**
 * dev:relay 目标解析。
 *
 * 多设备联调时设备凭证按浏览器 origin 隔离，两台机器必须连接同一个 relay：
 * 显式 CC_BOX_RELAY_URL 覆盖时只启动桌面端（连接共享 relay）；否则用探测到
 * 的局域网 IP + 端口启动本地 relay。
 */
/** 桌面端接受的 relay 地址可能是 ws(s)://，手机页面需要对应的 http(s):// */
function toPhoneUrl(url) {
  if (/^wss:\/\//i.test(url)) return url.replace(/^wss:/i, 'https:')
  if (/^ws:\/\//i.test(url)) return url.replace(/^ws:/i, 'http:')
  if (/^https?:\/\//i.test(url)) return url
  return `http://${url}`
}

export function resolveRelayTarget({ override, ip, port }) {
  const shared = typeof override === 'string' ? override.trim() : ''
  if (shared) {
    return {
      shared: true,
      relayUrl: shared,
      phoneUrl: toPhoneUrl(shared),
    }
  }
  return {
    shared: false,
    relayUrl: ip ? `${ip}:${port}` : '',
    phoneUrl: ip ? `http://${ip}:${port}` : '',
  }
}
