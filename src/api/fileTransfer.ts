/**
 * 远程文件传输 · 传输逻辑层（分块循环 / 重试 / 取消；FilesPanel 仅负责 UI）
 *
 * - 上传：begin → 512KB 串行 chunk（等 ack 再发下一块，天然背压）→ finish；
 *   失败/取消调 abort（服务端删 .part 临时文件）
 * - 下载：begin（服务端校验白名单/限额并固化 size/mtime）→ 循环拉 chunk
 *   直到 eof → 拼装 Blob；取消 = 停止拉取（服务端会话由 TTL 自动回收）
 * - chunk 失败重试 ≤2 次：服务端幂等（重复 offset 不重写），重发安全
 * - 仅浏览器远程模式可用（桌面路径 conn_id=None 服务端直接拒绝）
 */

import {
  fileUploadBegin,
  fileUploadChunk,
  fileUploadFinish,
  fileUploadAbort,
  fileDownloadBegin,
  fileDownloadChunk,
  fileDownloadTicket,
  fileDownloadTicketChunk,
} from './tauri'

/** 与服务端 file_transfer.rs CHUNK_SIZE 对齐（512KB；base64 后 ~683KB < relay 4MiB） */
export const CHUNK_SIZE = 512 * 1024

/** chunk 失败重试次数（幂等保证安全；总尝试 = 1 + RETRIES） */
const CHUNK_RETRIES = 2

export interface TransferProgress {
  transferred: number
  total: number
}

export interface TransferOpts {
  onProgress?(p: TransferProgress): void
  /** 轮询取消标记；返回 true 后上传会 abort、下载停止拉取 */
  shouldCancel?(): boolean
}

/** 取消信号（uploadFile/downloadFile 以此 reject，UI 据此区分错误与用户取消） */
export class TransferCancelled extends Error {
  constructor() {
    super('transfer cancelled')
    this.name = 'TransferCancelled'
  }
}

async function withRetry<T>(
  fn: () => Promise<T>,
  cancelled: () => boolean,
): Promise<T> {
  let lastErr: unknown
  for (let i = 0; i <= CHUNK_RETRIES; i++) {
    if (cancelled()) throw new TransferCancelled()
    try {
      return await fn()
    } catch (e) {
      lastErr = e
    }
  }
  throw lastErr
}

/** Blob → 纯 base64（FileReader.readAsDataURL 去掉 data: 前缀；jsdom 可 mock） */
function blobToBase64(blob: Blob): Promise<string> {
  return new Promise((resolve, reject) => {
    const r = new FileReader()
    r.onload = () => {
      const s = r.result
      resolve(typeof s === 'string' ? (s.split(',')[1] ?? '') : '')
    }
    r.onerror = () => reject(new Error('read local file failed'))
    r.readAsDataURL(blob)
  })
}

/**
 * 上传文件到当前会话项目的 `.cc-box-files/` 目录。
 * @returns 落盘路径：`relativePath` 为项目内相对路径（`.cc-box-files/<name>`，
 * 插入终端/展示用；冲突改名后为最终名），`targetPath` 为绝对路径
 */
export async function uploadFile(
  file: File,
  ptyId: string,
  opts: TransferOpts = {},
): Promise<{ targetPath: string; relativePath: string }> {
  const begin = await fileUploadBegin(ptyId, file.name, file.size)
  const cancelled = () => opts.shouldCancel?.() === true
  try {
    let offset = 0
    // 空文件也走一次 chunk（服务端记录 received=0，finish 直接提交空文件）
    do {
      if (cancelled()) throw new TransferCancelled()
      const blob = file.slice(offset, offset + CHUNK_SIZE)
      // 零进展保护：非空文件却读到 0 字节（异常 File 实现）会死循环
      if (blob.size === 0 && file.size > 0) {
        throw new Error('zero progress: failed to read local file chunk')
      }
      const dataB64 = blob.size > 0 ? await blobToBase64(blob) : ''
      const r = await withRetry(
        () => fileUploadChunk(begin.uploadId, offset, dataB64),
        cancelled,
      )
      offset += blob.size
      opts.onProgress?.({ transferred: r.received, total: file.size })
    } while (offset < file.size)
    if (cancelled()) throw new TransferCancelled()
    const fin = await fileUploadFinish(begin.uploadId)
    return { targetPath: fin.targetPath, relativePath: fin.relativePath }
  } catch (e) {
    // 任何失败（含取消）都让服务端立即清理 .part（不等 TTL）
    try {
      await fileUploadAbort(begin.uploadId)
    } catch {
      // 会话可能已被清理（如服务端已判定失败），忽略
    }
    throw e
  }
}

/**
 * 下载桌面文件（路径须在授权项目内，服务端校验）。
 * @returns 拼装好的 Blob 与文件名（调用方走 saveBlobToFile 保存）
 */
export async function downloadFile(
  path: string,
  opts: TransferOpts = {},
): Promise<{ blob: Blob; fileName: string }> {
  const begin = await fileDownloadBegin(path)
  const cancelled = () => opts.shouldCancel?.() === true
  const chunks: BlobPart[] = []
  let transferred = 0
  let eof = false
  while (!eof) {
    if (cancelled()) throw new TransferCancelled()
    const c = await withRetry(
      () => fileDownloadChunk(begin.downloadId, transferred),
      cancelled,
    )
    // 零进展保护：非 eof 却拿不到数据会死循环
    if (c.bytes === 0 && !c.eof) {
      throw new Error('zero progress: empty chunk before eof')
    }
    if (c.bytes > 0) {
      // atob → Uint8Array（512KB 逐字节拷贝，毫秒级）
      const bin = atob(c.dataB64)
      const arr = new Uint8Array(bin.length)
      for (let i = 0; i < bin.length; i++) arr[i] = bin.charCodeAt(i)
      chunks.push(arr as unknown as BlobPart)
      transferred += c.bytes
    }
    eof = c.eof
    opts.onProgress?.({ transferred, total: begin.size })
  }
  return { blob: new Blob(chunks), fileName: begin.fileName }
}

// ==================== 一次性票据直链（所有浏览器统一路径） ====================

export interface DownloadTicket {
  ticket: string
  url: string
  fileName: string
  size: number
  expiresAt: number
}

/**
 * 签发票据并拼出完整下载地址。
 * urlPath 由服务端按连接类型给出（直连 /dl/…、中继 /?dl=…），前端只拼 origin。
 */
export async function getDownloadTicket(path: string): Promise<DownloadTicket> {
  const t = await fileDownloadTicket(path)
  return {
    ticket: t.ticket,
    url: new URL(t.urlPath, location.origin).href,
    fileName: t.fileName,
    size: t.size,
    expiresAt: t.expiresAt,
  }
}

/**
 * 能力探测：旧 relay/旧桌面对 /dl/ 会返回 200 + index.html（静态 fallback），
 * 必须确认响应确实是文件，否则会下载到一个 HTML。HEAD 不消耗票据次数。
 */
export async function probeDownloadUrl(url: string, size: number): Promise<boolean> {
  try {
    const res = await fetch(url, { method: 'HEAD', cache: 'no-store' })
    if (!res.ok) return false
    const disposition = res.headers.get('content-disposition') ?? ''
    const contentType = res.headers.get('content-type') ?? ''
    const length = Number(res.headers.get('content-length'))
    if (!disposition.includes('attachment')) return false
    if (contentType.includes('text/html')) return false
    return size === 0 || length === size
  } catch {
    return false
  }
}

/** 触发浏览器原生下载（同源 download 属性 + 服务端 attachment 双保险） */
export function triggerDownload(url: string, fileName: string): void {
  const a = document.createElement('a')
  a.href = url
  a.download = fileName
  a.rel = 'noopener'
  a.style.display = 'none'
  document.body.appendChild(a)
  a.click()
  a.remove()
}

/**
 * 票据分块拉取（中继模式受限下载页）：断线后调用方按已下载 offset 重试即可续传
 * （服务端按 offset 幂等读取）。
 */
export async function downloadFileByTicket(
  ticket: string,
  size: number,
  opts: TransferOpts & { shouldRetry?(err: unknown): boolean } = {},
): Promise<{ blob: Blob; uploaded: number }> {
  const chunks: BlobPart[] = []
  const cancelled = () => opts.shouldCancel?.() === true
  let transferred = 0
  let eof = false
  let failures = 0
  while (!eof) {
    if (cancelled()) throw new TransferCancelled()
    let c: { dataB64: string; bytes: number; eof: boolean }
    try {
      // offset 幂等：断线重连后重拉同一块即可续传
      c = await fileDownloadTicketChunk(ticket, transferred)
      failures = 0
    } catch (e) {
      const msg = e instanceof Error ? e.message : String(e)
      // 票据失效/源文件变化/命令不可用：重试无意义，立即失败
      if (
        msg.includes('ticket') ||
        msg.includes('changed during download') ||
        msg.includes('offset beyond end') ||
        msg.includes('cannot read') ||
        msg.includes('not allowed')
      ) {
        throw e
      }
      // 网络抖动/重连中：等待恢复后按原 offset 继续（上限约 60s）
      failures++
      if (failures > 60 || (opts.shouldRetry && !opts.shouldRetry(e))) throw e
      await new Promise((r) => setTimeout(r, 1000))
      continue
    }
    if (c.bytes === 0 && !c.eof) {
      throw new Error('zero progress: empty chunk before eof')
    }
    if (c.bytes > 0) {
      const bin = atob(c.dataB64)
      const arr = new Uint8Array(bin.length)
      for (let i = 0; i < bin.length; i++) arr[i] = bin.charCodeAt(i)
      chunks.push(arr as unknown as BlobPart)
      transferred += c.bytes
    }
    eof = c.eof
    opts.onProgress?.({ transferred, total: size })
  }
  return { blob: new Blob(chunks), uploaded: transferred }
}

/** 纯安全字符可直接裸写（字母数字 + `/ . _ -`）——不含 Windows 反斜杠、
 * 空格与任何 shell 元字符 */
const SAFE_PATH = /^[A-Za-z0-9/._-]+$/

/** 终端输入路径转义（跨 shell 安全）：仅 SAFE_PATH 裸写，其余一律单引号
 * 包裹（内部 `'` → `'\''`）。Windows 反斜杠路径、空格及 `& ; | $ ( ) [ ] * ?`
 * 等元字符全走单引号——否则 bash 会解释反斜杠转义，且路径可被拼入命令时
 * 构成注入面（单引号内一切原样） */
export function quotePath(p: string): string {
  if (SAFE_PATH.test(p)) return p
  return `'${p.replace(/'/g, `'\\''`)}'`
}
