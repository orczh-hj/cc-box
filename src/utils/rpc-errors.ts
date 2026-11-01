/**
 * RPC 错误分类。
 *
 * 「新前端 + 旧桌面」时（中继页面永远是最新 dist，桌面可以是旧版），
 * 新前端调用的命令可能不在旧桌面 `rpc_dispatch` 白名单里，后端返回稳定串
 * `command not allowed: <cmd>`（rpc_dispatch.rs:138）。识别它以便降级提示，
 * 而不是把原始错误直接抛给用户或让调用链异常扩散。
 */
const COMMAND_NOT_ALLOWED = 'command not allowed'

/** 是否为「桌面版本过旧、不认识该命令」 */
export function isCommandNotAllowedError(e: unknown): boolean {
  const msg = e instanceof Error ? e.message : String(e ?? '')
  return msg.includes(COMMAND_NOT_ALLOWED)
}
