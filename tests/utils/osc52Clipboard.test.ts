import { describe, it, expect, vi, beforeEach } from 'vitest'

// 只 mock 传输出口：写入必须落到 platform.copyText（Tauri 插件/浏览器降级链）
const platformMock = vi.hoisted(() => ({ copyText: vi.fn() }))
vi.mock('@/api/platform', () => ({
  isTauriEnv: true,
  copyText: platformMock.copyText,
  readClipboardText: vi.fn(),
}))

// 组件接入点经真实 ClipboardAddon 驱动（addon 不 mock，验证真实 OSC 52
// 解析 + 我们的 provider 接线；Terminal 用最小桩捕获 parser handler）
import { loadOsc52Clipboard } from '@/utils/osc52-clipboard'

function stubTerminal() {
  let osc52Handler: ((data: string) => boolean | Promise<boolean>) | null = null
  return {
    input: vi.fn(),
    loadAddon(addon: { activate(t: unknown): void }) {
      addon.activate(this)
    },
    parser: {
      registerOscHandler(_id: number, cb: (data: string) => boolean | Promise<boolean>) {
        osc52Handler = cb
        return { dispose() {} }
      },
    },
    fire(data: string) {
      return osc52Handler?.(data)
    },
  }
}

describe('OSC 52 剪贴板支持', () => {
  beforeEach(() => {
    platformMock.copyText.mockReset()
  })

  // Claude CLI 的复制路径：OSC 52 写入 'c' 选区，base64 解码后必须进 copyText。
  // 真值独立手算：'hello' → aGVsbG8=；中文「你好」 → 5L2g5aW9
  it('Osc52_WriteDecodesToClipboard_001', async () => {
    const term = stubTerminal()
    loadOsc52Clipboard(term as never)

    expect(term.fire('c;aGVsbG8=')).toBe(true)
    await Promise.resolve()
    expect(platformMock.copyText).toHaveBeenCalledWith('hello')

    expect(term.fire('c;5L2g5aW9')).toBe(true)
    await Promise.resolve()
    expect(platformMock.copyText).toHaveBeenCalledWith('你好')
  })

  // X11 primary 选区（'p'）不支持：不得写入系统剪贴板
  it('Osc52_IgnoresPrimarySelection_002', async () => {
    const term = stubTerminal()
    loadOsc52Clipboard(term as never)

    expect(term.fire('p;aGVsbG8=')).toBe(true)
    await Promise.resolve()
    expect(platformMock.copyText).not.toHaveBeenCalled()
  })

  // 读取查询（'?'）：隐私考虑恒返回空，addon 以空 base64 回发，不触碰剪贴板读取
  it('Osc52_ReadQueryReturnsEmpty_003', async () => {
    const term = stubTerminal()
    loadOsc52Clipboard(term as never)

    const result = term.fire('c;?')
    await Promise.resolve(result ?? Promise.resolve())
    // 回发的响应是空串的 base64（''），绝不是系统剪贴板内容
    expect(term.input).toHaveBeenCalled()
    const seq = term.input.mock.calls[0][0] as string
    expect(seq).toBe('\x1b]52;c;\x07')
    expect(platformMock.copyText).not.toHaveBeenCalled()
  })
})
