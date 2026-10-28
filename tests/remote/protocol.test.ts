/**
 * 远程协议契约测试（前端侧）
 *
 * 与 remote/protocol 的 Rust 测试共跑同一批 JSON 样例（remote/protocol/samples/），
 * 保证两端序列化不漂移。样例文件改动必须同时过 Rust 与此处测试。
 */
import { describe, it, expect } from 'vitest'
import { readFileSync, readdirSync } from 'node:fs'
import { resolve } from 'node:path'
import { parseClientMessage, parseServerMessage, PROTOCOL_VERSION } from '../../remote/protocol/protocol'

const SAMPLES_DIR = resolve(__dirname, '../../remote/protocol/samples')

function loadSample(name: string): unknown {
  return JSON.parse(readFileSync(resolve(SAMPLES_DIR, name), 'utf-8'))
}

describe('Remote_ProtocolContract_所有样例可被前端解析', () => {
  const files = readdirSync(SAMPLES_DIR).filter(f => f.endsWith('.json'))

  it('样例目录非空且文件齐全', () => {
    expect(files.length).toBeGreaterThanOrEqual(14)
  })

  for (const file of files) {
    it(`${file} 与类型定义匹配`, () => {
      const raw = loadSample(file)
      if (file.startsWith('client-')) {
        const msg = parseClientMessage(raw)
        expect(msg, `client 样例 ${file} 解析失败`).not.toBeNull()
      } else {
        const msg = parseServerMessage(raw)
        expect(msg, `server 样例 ${file} 解析失败`).not.toBeNull()
      }
    })
  }
})

describe('Remote_ProtocolContract_字段值保真', () => {
  it('hello 消息字段完整', () => {
    const msg = parseClientMessage(loadSample('client-hello.json'))
    expect(msg).toEqual({
      type: 'hello',
      ver: PROTOCOL_VERSION,
      client: 'web-1.0.0',
      token: expect.any(String),
    })
  })

  it('sessions 列表条目字段完整', () => {
    const msg = parseServerMessage(loadSample('server-sessions.json'))
    expect(msg?.type).toBe('sessions')
    if (msg?.type === 'sessions') {
      expect(msg.list.length).toBe(2)
      // 全量刷新的状态版本基线（防旧事件覆盖）
      expect(msg.revision).toBe(3)
      // claude 会话携带 sessionId（hook SessionStart 建立）与实例分组字段
      expect(msg.list[0]).toEqual({
        id: expect.any(String),
        type: 'claude',
        cwd: expect.any(String),
        working: true,
        pending: false,
        sessionId: expect.any(String),
        instanceId: expect.any(String),
        instanceName: '实例 1',
      })
      // shell 会话无 sessionId（可选字段缺失合法），归属另一实例
      expect(msg.list[1]).toEqual({
        id: expect.any(String),
        type: 'shell',
        cwd: expect.any(String),
        working: false,
        pending: false,
        instanceId: expect.any(String),
        instanceName: '实例 2',
      })
    }
  })

  it('旧桌面 sessions 无实例字段时降级（向后兼容，不分组平铺）', () => {
    const msg = parseServerMessage(loadSample('server-sessions-ungrouped.json'))
    expect(msg?.type).toBe('sessions')
    if (msg?.type === 'sessions') {
      for (const s of msg.list) {
        expect(s).not.toHaveProperty('instanceId')
        expect(s).not.toHaveProperty('instanceName')
      }
    }
  })

  it('session.create 多实例路由字段保真（instanceId）', () => {
    const msg = parseClientMessage(loadSample('client-session-create-instance.json'))
    expect(msg).toEqual({
      type: 'session.create',
      path: expect.any(String),
      instanceId: 'a3bb189e8f14ca27169d9a391f98a3ab',
    })
  })

  it('hello.ok 携带连接标识/版本基线（controller 已弃用恒 null）', () => {
    const msg = parseServerMessage(loadSample('server-hello-ok.json'))
    expect(msg).toEqual({
      type: 'hello.ok',
      ver: PROTOCOL_VERSION,
      deviceName: expect.any(String),
      connId: 7,
      controller: null,
      revision: 3,
    })
  })

  it('controller.takeover 与 attach 拒绝消息保真', () => {
    expect(parseClientMessage(loadSample('client-controller-takeover.json')))
      .toEqual({ type: 'controller.takeover' })
    const denied = parseServerMessage(loadSample('server-session-attach-denied.json'))
    expect(denied).toEqual({
      type: 'session.attach.denied',
      id: expect.any(String),
      reason: expect.any(String),
    })
  })

  it('旧桌面端 hello.ok 缺新字段时取默认值（向后兼容）', () => {
    const msg = parseServerMessage({ type: 'hello.ok', ver: 1, deviceName: 'old' })
    expect(msg).toEqual({
      type: 'hello.ok',
      ver: 1,
      deviceName: 'old',
      connId: 0,
      controller: null,
      revision: 0,
    })
  })

  it('session.kill quiet 字段保真（重启静默 kill 不触发各端删 Tab）', () => {
    // 默认（显式关闭）：无 quiet 字段，联动删 Tab 语义不变
    expect(parseClientMessage(loadSample('client-session-kill.json')))
      .toEqual({ type: 'session.kill', id: expect.any(String) })
    // quiet=true：重启场景的内部清理，不广播 remote-session-closed
    expect(parseClientMessage(loadSample('client-session-kill-quiet.json')))
      .toEqual({ type: 'session.kill', id: expect.any(String), quiet: true })
  })

  it('session.create 启动选项字段保真（skipPermissions/customArgs）', () => {
    const msg = parseClientMessage(loadSample('client-session-create-opts.json'))
    expect(msg).toEqual({
      type: 'session.create',
      path: expect.any(String),
      skipPermissions: true,
      customArgs: '--model opus',
    })
  })

  it('pty.output 数据保真（含中文与 ANSI 场景文本）', () => {
    const msg = parseServerMessage(loadSample('server-pty-output.json'))
    expect(msg?.type).toBe('pty.output')
    if (msg?.type === 'pty.output') {
      expect(msg.data).toContain('Reading file')
    }
  })
})

describe('Remote_ProtocolContract_容错', () => {
  it('未知 type 返回 null', () => {
    expect(parseClientMessage({ type: 'nonexistent' })).toBeNull()
    expect(parseServerMessage({ type: 'nonexistent' })).toBeNull()
  })

  it('缺失必填字段返回 null', () => {
    expect(parseClientMessage({ type: 'hello', ver: 1, client: 'x' })).toBeNull()
    expect(parseClientMessage({ type: 'session.subscribe' })).toBeNull()
    expect(parseServerMessage({ type: 'hello.error' })).toBeNull()
    expect(parseServerMessage({ type: 'sessions', list: [{ id: 'x' }] })).toBeNull()
  })

  it('非对象输入返回 null', () => {
    expect(parseClientMessage(null)).toBeNull()
    expect(parseClientMessage('hello')).toBeNull()
    expect(parseServerMessage(undefined)).toBeNull()
    expect(parseServerMessage(42)).toBeNull()
  })
})
