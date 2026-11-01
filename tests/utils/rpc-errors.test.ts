import { describe, expect, it } from 'vitest'
import { isCommandNotAllowedError } from '@/utils/rpc-errors'

describe('isCommandNotAllowedError', () => {
  // 后端正本错误串：rpc_dispatch.rs `format!("command not allowed: {}", other)`
  it('RpcErrors_NotAllowedDetected_1', () => {
    expect(isCommandNotAllowedError(new Error('command not allowed: get_fancy_thing'))).toBe(true)
    // 经 WS 透传后可能是纯字符串 reject
    expect(isCommandNotAllowedError('command not allowed: spawn_new_instance')).toBe(true)
  })

  it('RpcErrors_OtherErrorsIgnored_2', () => {
    expect(isCommandNotAllowedError(new Error("rpc 'get_projects': timeout"))).toBe(false)
    expect(isCommandNotAllowedError(new Error('connection not open'))).toBe(false)
    expect(isCommandNotAllowedError(undefined)).toBe(false)
  })
})
