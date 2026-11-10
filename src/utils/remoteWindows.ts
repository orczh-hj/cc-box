/**
 * 远程（浏览器/手机）窗口拉起相关判定与等待工具。
 *
 * 桌面窗口自身就是一个成员，永远不需要“启动/新建窗口”。
 */
import { watch } from 'vue'
import { useSessionStore } from '@/stores/session'

/**
 * 零窗口启动入口判定（终端页空状态用）：
 * 只对浏览器/手机（非 Tauri）且已收到 Gateway 实例名单后启用。
 */
export function shouldOfferLaunchWindow(
  isTauri: boolean,
  instancesKnown: boolean,
  instanceCount: number,
): boolean {
  return !isTauri && instancesKnown && instanceCount === 0
}

/**
 * 新建窗口入口判定（首页 header 按钮用）：远程模式只要实例名单已知即可，
 * 不限窗口数——多窗口时也能再拉起新窗口（上限/冷却由 Broker 侧兜底）。
 */
export function canLaunchNewWindow(
  isTauri: boolean,
  instancesKnown: boolean,
): boolean {
  return !isTauri && instancesKnown
}

/**
 * 等待新实例上线：监听实例名单，出现不在 `knownIds` 中的新 id 即返回它；
 * 超时返回 null。`knownIds` 应为 spawn 前的名单快照。
 *
 * 已知限制：等待期间若他人（托盘/另一端）也拉起了新窗口，取到的是第一个
 * 未知 id——可能路由到「别人的」新窗口（id 真实有效，仅归属意图可能不符）。
 */
export async function waitForNewInstance(
  knownIds: Set<string>,
  timeoutMs = 15000,
): Promise<string | null> {
  const store = useSessionStore()
  return new Promise((resolve) => {
    let settled = false
    const stop = watch(
      () => store.remoteInstances,
      (list) => check(list),
      { deep: true },
    )
    const timer = window.setTimeout(() => finish(null), timeoutMs)

    function check(list: Array<{ id: string }>) {
      const fresh = list.find((i) => !knownIds.has(i.id))
      if (fresh) finish(fresh.id)
    }

    function finish(id: string | null) {
      if (settled) return
      settled = true
      window.clearTimeout(timer)
      stop()
      resolve(id)
    }

    // 竞态兜底：spawn 已 resolve、名单事件在 watch 注册前已到达
    check(store.remoteInstances)
  })
}
