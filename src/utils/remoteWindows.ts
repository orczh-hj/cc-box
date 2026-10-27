/**
 * 零窗口启动入口判定。
 *
 * 只对浏览器/手机（非 Tauri）且已收到 Gateway 实例名单后启用；桌面窗口
 * 自身就是一个成员，永远不需要“启动窗口”。
 */
export function shouldOfferLaunchWindow(
  isTauri: boolean,
  instancesKnown: boolean,
  instanceCount: number,
): boolean {
  return !isTauri && instancesKnown && instanceCount === 0
}
