/** 微信内置浏览器（Android X5 / iOS WKWebView）检测。
 * 仅用于追加「复制链接到系统浏览器」提示——下载路径本身对所有浏览器一致。 */
export function isWeChat(ua: string = navigator.userAgent): boolean {
  return /MicroMessenger/i.test(ua)
}
