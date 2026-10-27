/**
 * 设备列表存储（remote-desktops.ts）单元测试
 *
 * 多实例远程架构：手机按「设备」保存凭证（一台电脑一条目）。
 * 覆盖：token 形态判定、去重键、upsert 更新、失效标记回填、LRU 上限、
 * 旧单值 key 一次性迁移（保留不删，回滚兼容）。
 */
import { describe, it, expect, beforeEach } from 'vitest'
import {
  addDesktop,
  deviceIdOf,
  desktopIdOf,
  getDesktop,
  isRelayToken,
  listDesktops,
  migrateLegacyToken,
  removeDesktop,
  touchDesktop,
  LEGACY_TOKEN_KEY,
} from '@/api/remote-desktops'

const RELAY_TOKEN = 'f81d4fae7dec11d0a76500a0c91e6bf6.a3bb189e8f14ca27169d9a391f98a3ab'
const LAN_TOKEN = '4d5e6f7a8b9c0d1e2f3a4b5c6d7e8f9a0b1c2d3e4f5a6b7c8d9e0f1a2b3c4d5e'

describe('StoredDesktops 设备列表存储', () => {
  beforeEach(() => {
    localStorage.clear()
  })

  // ==================== token 形态判定 ====================

  it('StoredDesktops_RelayToken判定_001', () => {
    // v2 中继 token：两段 32 hex
    expect(isRelayToken(RELAY_TOKEN)).toBe(true)
    // LAN 静态 token（64 hex 无点）/ v1 历史格式 / 任意串
    expect(isRelayToken(LAN_TOKEN)).toBe(false)
    expect(isRelayToken('abc')).toBe(false)
    expect(isRelayToken('x.y')).toBe(false)
  })

  it('StoredDesktops_DeviceId提取_002', () => {
    // v2 取 `.` 前缀；无点格式取整串
    expect(deviceIdOf(RELAY_TOKEN)).toBe('f81d4fae7dec11d0a76500a0c91e6bf6')
    expect(deviceIdOf(LAN_TOKEN)).toBe(LAN_TOKEN)
  })

  it('StoredDesktops_去重键_003', () => {
    // 同设备同端口重扫（secret 轮换）→ 同键（键取 deviceId 前缀）；不同端口 → 不同键
    const rotated = 'f81d4fae7dec11d0a76500a0c91e6bf6.0123456789abcdef0123456789abcdef'
    const a = desktopIdOf('http://192.168.1.5:8747', RELAY_TOKEN)
    const b = desktopIdOf('http://192.168.1.5:8747/', rotated)
    const c = desktopIdOf('http://192.168.1.5:8748', RELAY_TOKEN)
    expect(a).toBe(b)
    expect(a).not.toBe(c)
  })

  // ==================== addDesktop upsert ====================

  it('StoredDesktops_新增与去重更新_004', () => {
    const d1 = addDesktop({ kind: 'relay', token: RELAY_TOKEN, origin: 'https://orczh.top:8800' })
    expect(listDesktops()).toHaveLength(1)
    expect(d1.kind).toBe('relay')

    // 同设备重扫（secret 轮换）：token 更新、条目不重复、invalid 清除
    touchDesktop(d1.id, { invalid: true })
    const newToken = 'f81d4fae7dec11d0a76500a0c91e6bf6.0123456789abcdef0123456789abcdef'
    const d2 = addDesktop({ kind: 'relay', token: newToken, origin: 'https://orczh.top:8800/' })
    expect(d2.id).toBe(d1.id)
    expect(d2.token).toBe(newToken)
    expect(d2.invalid).toBeFalsy()
    expect(listDesktops()).toHaveLength(1)
  })

  it('StoredDesktops_hello回填名称与失效标记_005', () => {
    const d = addDesktop({ kind: 'lan', token: LAN_TOKEN, origin: 'http://192.168.1.5:8747' })
    touchDesktop(d.id, { name: 'DESKTOP-ABC' })
    touchDesktop(d.id, { invalid: true })
    expect(getDesktop(d.id)?.name).toBe('DESKTOP-ABC')
    expect(getDesktop(d.id)?.invalid).toBe(true)
    // 重连成功清除失效标记
    touchDesktop(d.id, { invalid: false })
    expect(getDesktop(d.id)?.invalid).toBe(false)
  })

  it('StoredDesktops_LRU上限淘汰最旧_006', () => {
    for (let i = 0; i < 12; i++) {
      addDesktop({ kind: 'lan', token: `${LAN_TOKEN.slice(0, 60)}${i}`, origin: `http://192.168.1.5:874${i}` })
    }
    expect(listDesktops().length).toBeLessThanOrEqual(10)
    // 最后加入的（最新使用）必然在列
    const ids = listDesktops().map(d => d.id)
    expect(ids).toContain(desktopIdOf('http://192.168.1.5:87411', `${LAN_TOKEN.slice(0, 60)}11`))
  })

  it('StoredDesktops_删除条目_007', () => {
    const d = addDesktop({ kind: 'relay', token: RELAY_TOKEN, origin: 'https://orczh.top:8800' })
    removeDesktop(d.id)
    expect(listDesktops()).toHaveLength(0)
    // 幂等
    removeDesktop(d.id)
  })

  it('StoredDesktops_损坏数据容错_008', () => {
    localStorage.setItem('ccbox-desktops', '{broken json')
    expect(listDesktops()).toEqual([])
    // 容错后可正常写入
    addDesktop({ kind: 'lan', token: LAN_TOKEN, origin: 'http://192.168.1.5:8747' })
    expect(listDesktops()).toHaveLength(1)
  })

  // ==================== 旧单值迁移 ====================

  it('StoredDesktops_旧token迁移_009', () => {
    localStorage.setItem(LEGACY_TOKEN_KEY, RELAY_TOKEN)
    const d = migrateLegacyToken('https://orczh.top:8800')
    expect(d?.kind).toBe('relay')
    expect(d?.token).toBe(RELAY_TOKEN)
    // 旧 key 保留不删（回滚兼容：旧版前端仍可读）
    expect(localStorage.getItem(LEGACY_TOKEN_KEY)).toBe(RELAY_TOKEN)
    // 二次迁移不重复（列表非空）
    expect(migrateLegacyToken('https://orczh.top:8800')).toBeNull()
    expect(listDesktops()).toHaveLength(1)
  })

  it('StoredDesktops_旧LANtoken迁移判lan_010', () => {
    localStorage.setItem(LEGACY_TOKEN_KEY, LAN_TOKEN)
    const d = migrateLegacyToken('http://192.168.1.5:8747')
    expect(d?.kind).toBe('lan')
    expect(d?.origin).toBe('http://192.168.1.5:8747')
  })
})
