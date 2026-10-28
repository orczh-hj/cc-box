/**
 * 设备列表探测编排（模块级单例）。
 *
 * TokenGate 与设备切换弹窗不会同时存在，结果按 `${id}|${token}` 共享；
 * token 变化（重新配对/刷新密钥后重扫）自然产生新 key，旧结果不再命中。
 * 当前连接设备不探测——它的状态由主连接 connState 实时反映，重复建连没有收益。
 * 结果只存内存：网络状态是瞬态的，且探测失败不应改写持久化配对信息。
 */

import { reactive } from 'vue'
import { probeDesktop, type ProbeStatus } from '@/api/remote-probe'
import type { StoredDesktop } from '@/api/remote-desktops'

const statuses = reactive<Record<string, ProbeStatus>>({})
const inflight = new Set<string>()
/** 当前列表仍引用（且非当前设备）的 key：旧探测回调不得写回已移除条目 */
let currentKeys = new Set<string>()
/** 同一 key 的探测代次：recheck 后旧回调过期，结果不得覆盖新一轮 */
const probeGeneration = new Map<string, number>()
let generation = 0

export function probeKey(d: Pick<StoredDesktop, 'id' | 'token'>): string {
  return `${d.id}|${d.token}`
}

export function probeStatusOf(d: Pick<StoredDesktop, 'id' | 'token'>): ProbeStatus | undefined {
  return statuses[probeKey(d)]
}

function startProbe(d: StoredDesktop): void {
  const key = probeKey(d)
  const my = ++generation
  probeGeneration.set(key, my)
  inflight.add(key)
  statuses[key] = 'checking'
  void probeDesktop(d)
    .then((status) => {
      if (probeGeneration.get(key) !== my || !currentKeys.has(key)) return
      statuses[key] = status
    })
    .finally(() => {
      if (probeGeneration.get(key) !== my) return
      inflight.delete(key)
      probeGeneration.delete(key)
    })
}

function prune(desktops: StoredDesktop[], activeId: string | null): void {
  currentKeys = new Set(
    desktops.filter(d => d.id !== activeId).map(d => probeKey(d)),
  )
  for (const key of Object.keys(statuses)) {
    if (!currentKeys.has(key)) delete statuses[key]
  }
}

/** 并发探测列表内所有非当前设备；已在探测中的条目跳过 */
export function probeAll(desktops: StoredDesktop[], activeId: string | null): void {
  prune(desktops, activeId)
  for (const d of desktops) {
    if (d.id === activeId) continue
    const key = probeKey(d)
    if (inflight.has(key)) continue
    startProbe(d)
  }
}

/** 清空结果并强制重新探测（旧一轮在途结果由代次过滤） */
export function recheckDesktops(desktops: StoredDesktop[], activeId: string | null): void {
  prune(desktops, activeId)
  for (const d of desktops) {
    if (d.id === activeId) continue
    startProbe(d)
  }
}

export function useDeviceProbe() {
  return {
    statuses,
    probeAll,
    recheck: recheckDesktops,
    statusOf: probeStatusOf,
  }
}
