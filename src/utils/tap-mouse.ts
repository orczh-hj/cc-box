export interface TapMouseInit {
  bubbles: true
  cancelable: true
  detail: number
  clientX: number
  clientY: number
  button: 0
}

/**
 * TAP 手势坐标 → 合成 MouseEvent 参数（page→client 坐标换算 + 双击 detail）。
 * 坐标无效（惯性等场景缺失）返回 null，调用方跳过合成。
 */
export function buildTapMouseInit(
  pageX: number | undefined,
  pageY: number | undefined,
  tapCount: number | undefined,
  scrollX: number,
  scrollY: number
): TapMouseInit | null {
  const isFiniteNum = (v: number | undefined): v is number =>
    typeof v === 'number' && Number.isFinite(v)
  if (!isFiniteNum(pageX) || !isFiniteNum(pageY)) return null
  return {
    bubbles: true,
    cancelable: true,
    detail: tapCount === 2 ? 2 : 1,
    clientX: pageX - scrollX,
    clientY: pageY - scrollY,
    button: 0,
  }
}
