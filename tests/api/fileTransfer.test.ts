import { describe, it, expect, vi, beforeEach } from 'vitest'

// Mock 命令封装层（被测对象是 fileTransfer.ts 的分块/重试/取消逻辑）
vi.mock('@/api/tauri', () => ({
  fileUploadBegin: vi.fn(),
  fileUploadChunk: vi.fn(),
  fileUploadFinish: vi.fn(),
  fileUploadAbort: vi.fn(),
  fileDownloadBegin: vi.fn(),
  fileDownloadChunk: vi.fn(),
}))

import {
  uploadFile,
  downloadFile,
  quotePath,
  TransferCancelled,
} from '@/api/fileTransfer'
import {
  fileUploadBegin,
  fileUploadChunk,
  fileUploadFinish,
  fileUploadAbort,
  fileDownloadBegin,
  fileDownloadChunk,
} from '@/api/tauri'

/** 鸭子类型 File（只用 name/size/slice），多块场景无需构造大文件：
 * size 虚报跨块，slice 返回真实小 Blob 保证 FileReader 走真 b64 编码 */
function fakeFile(name: string, parts: Uint8Array[]): File {
  const total = parts.reduce((s, p) => s + p.length, 0)
  let offset = 0
  const ranges = parts.map((p) => {
    const r = [offset, offset + p.length]
    offset += p.length
    return r as [number, number]
  })
  return {
    name,
    size: total,
    slice: (from: number, _to: number) => {
      if (parts.length === 0) return new Blob([])
      const idx = ranges.findIndex(([s, e]) => from >= s && from < e)
      return new Blob([parts[idx] as unknown as BlobPart])
    },
  } as unknown as File
}

function b64(bytes: Uint8Array): string {
  return btoa(String.fromCharCode(...bytes))
}

/** jsdom Blob 无 arrayBuffer——经 FileReader 读回字节 */
function blobBytes(blob: Blob): Promise<Uint8Array> {
  return new Promise((resolve, reject) => {
    const r = new FileReader()
    r.onload = () => resolve(new Uint8Array(r.result as ArrayBuffer))
    r.onerror = () => reject(new Error('read blob failed'))
    r.readAsArrayBuffer(blob)
  })
}

beforeEach(() => {
  vi.mocked(fileUploadBegin).mockReset()
  vi.mocked(fileUploadChunk).mockReset()
  vi.mocked(fileUploadFinish).mockReset()
  vi.mocked(fileUploadAbort).mockReset().mockResolvedValue(undefined)
  vi.mocked(fileDownloadBegin).mockReset()
  vi.mocked(fileDownloadChunk).mockReset()
})

describe('quotePath 转义', () => {
  it('QuotePath_SafeCharsBare_1', () => {
    // 只有安全字符（字母数字 + / . _ -）裸写
    expect(quotePath('/home/me/demo/a.txt')).toBe('/home/me/demo/a.txt')
  })

  it('QuotePath_WindowsBackslashSingleQuoted_1', () => {
    // Windows 反斜杠路径必须单引号（bash 会解释 \f \b 等转义）
    expect(quotePath('D:\\proj\\.cc-box-files\\a.txt')).toBe(
      "'D:\\proj\\.cc-box-files\\a.txt'",
    )
  })

  it('QuotePath_SpaceSingleQuoted_1', () => {
    expect(quotePath('C:\\My Files\\a b.txt')).toBe("'C:\\My Files\\a b.txt'")
  })

  it('QuotePath_ShellMetacharsSingleQuoted_1', () => {
    // 所有可能被 shell 解释的字符（含注入面）都走单引号
    expect(quotePath('/tmp/a$b.txt')).toBe("'/tmp/a$b.txt'")
    expect(quotePath('/tmp/a;curl evil|sh')).toBe("'/tmp/a;curl evil|sh'")
    expect(quotePath('/tmp/a&b[1]*c?.txt')).toBe("'/tmp/a&b[1]*c?.txt'")
    // 内部单引号 → '\'' 转义
    expect(quotePath("it's a `file`")).toBe("'it'\\''s a `file`'")
  })
})

describe('uploadFile', () => {
  it('UploadFile_MultiChunk_1', async () => {
    // 3 块：slice 依次返回小 Blob（虚拟文件——offset 按实际块大小步进，
    // 与真实文件 min(CHUNK_SIZE, remaining) 语义一致）
    const parts = [new Uint8Array([1, 2]), new Uint8Array([3]), new Uint8Array([4, 5, 6])]
    const file = fakeFile('a.bin', parts)
    let sliceCall = 0
    file.slice = vi.fn(() => new Blob([parts[sliceCall++]])) as unknown as File['slice']
    vi.mocked(fileUploadBegin).mockResolvedValue({
      uploadId: 'u1',
      targetPath: 'x/a.bin',
      relativePath: '.cc-box-files/a.bin',
      fileName: 'a.bin',
    })
    vi.mocked(fileUploadChunk).mockImplementation(async (_id, offset) => ({
      received: offset + 1,
    }))
    vi.mocked(fileUploadFinish).mockResolvedValue({
      targetPath: 'x/a (1).bin',
      relativePath: '.cc-box-files/a (1).bin',
    })

    const r = await uploadFile(file as File, 'pty1')
    expect(r.targetPath).toBe('x/a (1).bin')
    expect(r.relativePath).toBe('.cc-box-files/a (1).bin')
    expect(fileUploadChunk).toHaveBeenCalledTimes(3)
    // offset 随实际块大小（2/1/3 字节）累加
    expect(vi.mocked(fileUploadChunk).mock.calls.map((c) => c[1])).toEqual([0, 2, 3])
    // chunk dataB64 与分块内容一一对应
    expect(vi.mocked(fileUploadChunk).mock.calls[0][2]).toBe(b64(new Uint8Array([1, 2])))
    expect(vi.mocked(fileUploadChunk).mock.calls[1][2]).toBe(b64(new Uint8Array([3])))
    expect(vi.mocked(fileUploadChunk).mock.calls[2][2]).toBe(b64(new Uint8Array([4, 5, 6])))
    expect(fileUploadAbort).not.toHaveBeenCalled()
  })

  it('UploadFile_ProgressCallback_1', async () => {
    const file = fakeFile('s.txt', [new Uint8Array([9, 9])])
    vi.mocked(fileUploadBegin).mockResolvedValue({
      uploadId: 'u2',
      targetPath: 'x/s.txt',
      relativePath: '.cc-box-files/s.txt',
      fileName: 's.txt',
    })
    vi.mocked(fileUploadChunk).mockResolvedValue({ received: 2 })
    vi.mocked(fileUploadFinish).mockResolvedValue({ targetPath: 'x/s.txt', relativePath: '.cc-box-files/s.txt' })

    const progress: Array<{ transferred: number; total: number }> = []
    await uploadFile(file as File, 'pty1', {
      onProgress: (p) => progress.push({ ...p }),
    })
    expect(progress).toEqual([{ transferred: 2, total: 2 }])
  })

  it('UploadFile_EmptyFile_1', async () => {
    const file = fakeFile('e.txt', [])
    Object.defineProperty(file, 'size', { value: 0 })
    vi.mocked(fileUploadBegin).mockResolvedValue({
      uploadId: 'u3',
      targetPath: 'x/e.txt',
      relativePath: '.cc-box-files/e.txt',
      fileName: 'e.txt',
    })
    vi.mocked(fileUploadChunk).mockResolvedValue({ received: 0 })
    vi.mocked(fileUploadFinish).mockResolvedValue({ targetPath: 'x/e.txt', relativePath: '.cc-box-files/e.txt' })

    await uploadFile(file as File, 'pty1')
    // 空文件走一次 b64='' 的 chunk 后 finish
    expect(fileUploadChunk).toHaveBeenCalledTimes(1)
    expect(vi.mocked(fileUploadChunk).mock.calls[0][2]).toBe('')
  })

  it('UploadFile_CancelAborts_1', async () => {
    const file = fakeFile('c.txt', [new Uint8Array([1])])
    vi.mocked(fileUploadBegin).mockResolvedValue({
      uploadId: 'u4',
      targetPath: 'x/c.txt',
      relativePath: '.cc-box-files/c.txt',
      fileName: 'c.txt',
    })
    vi.mocked(fileUploadChunk).mockResolvedValue({ received: 1 })

    await expect(
      uploadFile(file as File, 'pty1', { shouldCancel: () => true }),
    ).rejects.toBeInstanceOf(TransferCancelled)
    expect(fileUploadAbort).toHaveBeenCalledWith('u4')
    expect(fileUploadFinish).not.toHaveBeenCalled()
  })

  it('UploadFile_ChunkRetry_1', async () => {
    const file = fakeFile('r.txt', [new Uint8Array([7])])
    vi.mocked(fileUploadBegin).mockResolvedValue({
      uploadId: 'u5',
      targetPath: 'x/r.txt',
      relativePath: '.cc-box-files/r.txt',
      fileName: 'r.txt',
    })
    // 第一次超时失败、重试成功（服务端幂等保证重发安全）
    vi.mocked(fileUploadChunk)
      .mockRejectedValueOnce(new Error("rpc 'file_upload_chunk': timeout"))
      .mockResolvedValueOnce({ received: 1 })
    vi.mocked(fileUploadFinish).mockResolvedValue({ targetPath: 'x/r.txt', relativePath: '.cc-box-files/r.txt' })

    await uploadFile(file as File, 'pty1')
    expect(fileUploadChunk).toHaveBeenCalledTimes(2)
    expect(fileUploadAbort).not.toHaveBeenCalled()
  })

  it('UploadFile_FailureAborts_1', async () => {
    const file = fakeFile('f.txt', [new Uint8Array([1])])
    vi.mocked(fileUploadBegin).mockResolvedValue({
      uploadId: 'u6',
      targetPath: 'x/f.txt',
      relativePath: '.cc-box-files/f.txt',
      fileName: 'f.txt',
    })
    // 重试耗尽仍失败 → abort 清理 + 上抛
    vi.mocked(fileUploadChunk).mockRejectedValue(new Error('boom'))

    await expect(uploadFile(file as File, 'pty1')).rejects.toThrow('boom')
    expect(fileUploadChunk).toHaveBeenCalledTimes(3) // 1 + 2 重试
    expect(fileUploadAbort).toHaveBeenCalledWith('u6')
  })
})

describe('downloadFile', () => {
  it('DownloadFile_ChunksToBlob_1', async () => {
    const b1 = new Uint8Array([1, 2, 3])
    const b2 = new Uint8Array([4, 5])
    vi.mocked(fileDownloadBegin).mockResolvedValue({
      downloadId: 'd1',
      fileName: 'dl.bin',
      size: 5,
      mtime: 1,
    })
    vi.mocked(fileDownloadChunk)
      .mockResolvedValueOnce({ dataB64: b64(b1), bytes: 3, eof: false })
      .mockResolvedValueOnce({ dataB64: b64(b2), bytes: 2, eof: true })

    const r = await downloadFile('/proj/dl.bin')
    expect(r.fileName).toBe('dl.bin')
    // chunk 只传 downloadId 与累计 offset（不传 path）
    expect(fileDownloadChunk).toHaveBeenCalledWith('d1', 0)
    expect(fileDownloadChunk).toHaveBeenCalledWith('d1', 3)
    // Blob 内容 round-trip（jsdom Blob 无 arrayBuffer，走 FileReader）
    const buf = await blobBytes(r.blob)
    expect(Array.from(buf)).toEqual([1, 2, 3, 4, 5])
  })

  it('DownloadFile_ProgressAndCancel_1', async () => {
    vi.mocked(fileDownloadBegin).mockResolvedValue({
      downloadId: 'd2',
      fileName: 'x.txt',
      size: 2,
      mtime: 1,
    })
    const progress: Array<{ transferred: number; total: number }> = []
    // 第一块后取消
    vi.mocked(fileDownloadChunk).mockResolvedValueOnce({
      dataB64: b64(new Uint8Array([1])),
      bytes: 1,
      eof: false,
    })
    await expect(
      downloadFile('/p/x.txt', {
        onProgress: (p) => progress.push({ ...p }),
        shouldCancel: () => progress.length >= 1,
      }),
    ).rejects.toBeInstanceOf(TransferCancelled)
    expect(progress).toEqual([{ transferred: 1, total: 2 }])
  })

  it('DownloadFile_EmptyFile_1', async () => {
    vi.mocked(fileDownloadBegin).mockResolvedValue({
      downloadId: 'd3',
      fileName: 'e.txt',
      size: 0,
      mtime: 1,
    })
    vi.mocked(fileDownloadChunk).mockResolvedValueOnce({
      dataB64: '',
      bytes: 0,
      eof: true,
    })
    const r = await downloadFile('/p/e.txt')
    expect(r.blob.size).toBe(0)
  })
})
