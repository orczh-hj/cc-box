/**
 * 传输层统一入口（统一管理：前端单一传输层）
 *
 * 桌面 Tauri 与浏览器（经远程 WS）共用同一套 API 封装（tauri.ts / provider.ts），
 * 组件零感知运行环境。环境检测在模块加载时完成（先于任何 Tauri API 调用）。
 *
 * - invoke：Tauri command 调用（浏览器端经 rpc.call 白名单转发）
 * - listen：事件订阅（payload 与 Tauri 事件同构；浏览器端经 rpc.event / 专用消息）
 * - sendClient / onServerMessage：浏览器会话层专用（pty.input / session.attach 等
 *   远程协议消息，桌面模式不可用）
 */

import type { ClientMessage, ServerMessage } from '../../remote/protocol/protocol'
import { tauriTransport } from './tauri-transport'
import { remoteTransport } from './remote-transport'
import { transportEnv } from './env'

export type Unlisten = () => void
export { transportEnv }
export type { TransportEnv } from './env'

export interface Transport {
  readonly env: TransportEnv
  invoke<T = unknown>(cmd: string, args?: Record<string, unknown>): Promise<T>
  listen<T = unknown>(event: string, cb: (payload: T) => void): Promise<Unlisten>
  /** 发送远程协议消息（仅浏览器模式；桌面模式抛错——PTY 流走 Tauri 通道） */
  sendClient(msg: ClientMessage): void
  /** 订阅远程协议服务端消息（仅浏览器模式；A5 会话层消费 pty.output / session.*） */
  onServerMessage(cb: (msg: ServerMessage) => void): Unlisten

  // ==== 会话 API（桌面 = Tauri command；浏览器 = 远程协议消息，组件零感知） ====

  /** 创建会话：桌面 pty_spawn（args 已拼好）；浏览器 session.create（结构化选项） */
  spawnSession(opts: SpawnSessionOptions): Promise<SpawnSessionResult>
  /** 输入注入 */
  sessionInput(id: string, data: string): Promise<boolean>
  /** 尺寸调整 */
  sessionResize(id: string, cols: number, rows: number): Promise<boolean>
  /** 关闭会话；quiet=true 为重启场景内部清理——不广播 remote-session-closed（各端保留 Tab 等 respawn） */
  sessionKill(id: string, quiet?: boolean): Promise<boolean>
  /** 夺回控制权（桌面 pty_claim；浏览器重新 attach 抢占，cols/rows 为当前实际尺寸） */
  sessionClaim(id: string, cols?: number, rows?: number): Promise<void>
}

export interface SpawnSessionOptions {
  cwd: string
  cols: number
  rows: number
  type: 'claude' | 'shell'
  /** 桌面：拼好的 CLI 参数（buildClaudeArgs） */
  args?: string[]
  /** 浏览器：结构化启动选项（Rust 侧 build_claude_args 按同一规则拼接） */
  resume?: string
  /** 浏览器：恢复会话的显示名（桌面建 Tab 用，防 sessionId 编码名） */
  resumeName?: string
  skipPermissions?: boolean
  customArgs?: string
}

export interface SpawnSessionResult {
  id: string
  cwd: string
}

export const transport: Transport = transportEnv.isTauri ? tauriTransport : remoteTransport
