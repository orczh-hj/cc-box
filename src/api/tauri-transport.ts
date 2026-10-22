/**
 * 传输层 · 桌面 Tauri 实现：直接转发 invoke / listen（payload 解包与 Tauri 事件一致）
 */

import { invoke as tauriInvoke } from '@tauri-apps/api/core'
import { listen as tauriListen } from '@tauri-apps/api/event'
import type { ClientMessage, ServerMessage } from '../../remote/protocol/protocol'
import { transportEnv } from './env'
import type {
  Transport,
  Unlisten,
  InvokeOpts,
  SpawnSessionOptions,
  SpawnSessionResult,
} from './transport'

export const tauriTransport: Transport = {
  env: transportEnv,
  invoke<T>(cmd: string, args?: Record<string, unknown>, _opts?: InvokeOpts): Promise<T> {
    // 本地 IPC 无慢命令，忽略 timeoutMs（仅浏览器远程 rpc 有 15s 默认超时）
    return tauriInvoke<T>(cmd, args)
  },
  listen<T>(event: string, cb: (payload: T) => void): Promise<Unlisten> {
    return tauriListen<T>(event, (e) => cb(e.payload))
  },
  sendClient(_msg: ClientMessage): void {
    // 桌面模式 PTY 流走 Tauri 通道，远程协议消息不存在
    throw new Error('sendClient is only available in browser remote mode')
  },
  onServerMessage(_cb: (msg: ServerMessage) => void): Unlisten {
    throw new Error('onServerMessage is only available in browser remote mode')
  },

  async spawnSession(opts: SpawnSessionOptions): Promise<SpawnSessionResult> {
    // 桌面：pty_spawn 返回完整 PtySpawnResult（id/cwd 等），此处收窄为公共结构
    const r = await tauriInvoke<{ id: string; cwd?: string }>('pty_spawn', {
      options: {
        cwd: opts.cwd,
        cols: opts.cols,
        rows: opts.rows,
        type: opts.type,
        args: opts.args,
      },
    })
    return { id: r.id, cwd: r.cwd ?? opts.cwd }
  },
  sessionInput(id, data) {
    return tauriInvoke<boolean>('pty_input', { id, data })
  },
  sessionResize(id, cols, rows) {
    return tauriInvoke<boolean>('pty_resize', { id, cols, rows })
  },
  sessionKill(id, quiet) {
    return tauriInvoke<boolean>('pty_kill', { id, quiet })
  },
  async sessionClaim(id) {
    await tauriInvoke<void>('pty_claim', { id })
  },
}
