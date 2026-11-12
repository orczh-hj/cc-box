import { describe, it, expect, beforeEach, vi } from 'vitest'

const logMock = vi.fn()
vi.mock('@/api/tauri', () => ({
  logMessage: (...args: unknown[]) => logMock(...args),
}))
vi.mock('@/api/transport', () => ({
  transportEnv: { isTauri: false, isBrowserRemote: true },
}))

type Mod = typeof import('@/api/platform')

/** 模拟手机浏览器剪贴板实现：各层可独立注入。
 *  getType 返回带 text() 的普通对象（源码只调用 blob.text()；
 *  jsdom 的 Blob 缺 text()，生产浏览器一定有） */
function installClipboard(impl: {
  readText?: () => Promise<string>
  read?: () => Promise<{ types: string[]; getType(t: string): Promise<{ text(): Promise<string> }> }[]>
}) {
  Object.defineProperty(navigator, 'clipboard', {
    configurable: true,
    value: {
      readText: impl.readText ?? (() => Promise.resolve('')),
      ...(impl.read ? { read: () => impl.read!() } : {}),
    },
  })
}

/** 模拟旧版 execCommand('paste')：往当前聚焦元素写入剪贴板文本（jsdom 无此 API，直接挂 mock） */
function installExecCommandPaste(text: string | null) {
  Object.defineProperty(document, 'execCommand', {
    configurable: true,
    value: () => {
      if (text === null) return false
      const el = document.activeElement as HTMLTextAreaElement | null
      if (el) el.value = text
      return true
    },
  })
}

describe('readClipboardText 浏览器三层降级读取', () => {
  let mod: Mod
  beforeEach(async () => {
    vi.resetModules()
    logMock.mockReset()
    mod = await import('@/api/platform')
    installExecCommandPaste(null)
  })

  // 标准 Chromium：readText 直接返回文本 → 不触碰后两层
  it('ClipboardBrowser_ReadTextFirst_001', async () => {
    const read = vi.fn(() => Promise.resolve('hello'))
    installClipboard({ readText: read })
    await expect(mod.readClipboardText()).resolves.toBe('hello')
    expect(read).toHaveBeenCalledTimes(1)
  })

  // 手机浏览器空桩（真实 bug：微信/国产内核 readText 恒返空串不报错）
  // → 降级 clipboard.read() 读 ClipboardItem
  it('ClipboardBrowser_FallbackToRead_002', async () => {
    installClipboard({
      readText: () => Promise.resolve(''),
      read: () =>
        Promise.resolve([
          {
            types: ['text/html', 'text/plain'],
            getType: (t: string) =>
              Promise.resolve({ text: () => Promise.resolve(t === 'text/plain' ? 'FROM_READ' : '<b>x</b>') }),
          },
        ]),
    })
    await expect(mod.readClipboardText()).resolves.toBe('FROM_READ')
  })

  // read 也是空桩 → 降级 execCommand('paste')（旧内核可用路径）
  it('ClipboardBrowser_FallbackToExecCommand_003', async () => {
    installClipboard({
      readText: () => Promise.resolve(''),
      read: () => Promise.resolve([]),
    })
    installExecCommandPaste('FROM_EXEC')
    await expect(mod.readClipboardText()).resolves.toBe('FROM_EXEC')
  })

  // 三层全失败 → 返回空串并落诊断日志（含 UA，供真机排查）
  it('ClipboardBrowser_AllFailLogs_004', async () => {
    installClipboard({
      readText: () => Promise.resolve(''),
      read: () => Promise.reject(new DOMException('denied', 'NotAllowedError')),
    })
    await expect(mod.readClipboardText()).resolves.toBe('')
    expect(logMock).toHaveBeenCalledWith(
      'warn',
      expect.stringContaining('clipboard read failed on all paths'),
    )
  })

  // readText 抛错（权限拒绝）也进入降级链而非直接失败
  it('ClipboardBrowser_RejectFallsThrough_005', async () => {
    installClipboard({
      readText: () => Promise.reject(new DOMException('denied', 'NotAllowedError')),
    })
    installExecCommandPaste('EXEC_AFTER_REJECT')
    await expect(mod.readClipboardText()).resolves.toBe('EXEC_AFTER_REJECT')
  })
})
