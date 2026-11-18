/**
 * 浏览器远程模式的连接健康度分层（UI 提示级别派生）。
 *
 * transport 层负责检测与恢复（pong 超时 / 回前台校验，见 remote-transport.ts）；
 * 本 composable 只做呈现分层：订阅 connState 记录断开起始时间，按持续时长
 * 派生三级提示。恢复（connected）时自动复位到 ok，无需任何用户操作。
 *
 * 分层（浏览器远程「卡住无提示」问题的分层提示方案）：
 * - spinner：断开即现（header 旋转图标，非阻断）
 * - overlay：断开超 10s（页面半透明覆盖提示「正在自动重连」）
 * - stalled：断开超 60s（强提示 + 切换设备出口）
 */
import { ref, onUnmounted } from 'vue'
import { remoteTransport, isDisconnectedState } from '@/api/remote-transport'

export type ConnHealthLevel = 'ok' | 'spinner' | 'overlay' | 'stalled'

export const OVERLAY_AFTER_MS = 10_000
export const STALLED_AFTER_MS = 60_000

/** 纯函数：断开持续时长 → 提示层级（独立于订阅逻辑，可单测） */
export function connHealthLevel(disconnectedMs: number): ConnHealthLevel {
  if (disconnectedMs >= STALLED_AFTER_MS) return 'stalled'
  if (disconnectedMs >= OVERLAY_AFTER_MS) return 'overlay'
  return 'spinner'
}

export function useConnHealth() {
  const level = ref<ConnHealthLevel>('ok')
  const disconnectedMs = ref(0)
  let since: number | null = null

  function compute(): void {
    if (!isDisconnectedState(remoteTransport.connState)) {
      since = null
      disconnectedMs.value = 0
      level.value = 'ok'
      return
    }
    if (since === null) since = Date.now()
    disconnectedMs.value = Date.now() - since
    level.value = connHealthLevel(disconnectedMs.value)
  }

  const unlisten = remoteTransport.subscribeConnState(compute)
  // 每秒推进分层（断开 10s/60s 的升级由时间驱动，不由事件驱动）
  const timer = window.setInterval(compute, 1_000)
  onUnmounted(() => {
    unlisten()
    window.clearInterval(timer)
  })
  // 挂载时可能已处于断开态（如覆盖层/图标随视图切换重建）
  compute()

  return { level, disconnectedMs }
}
