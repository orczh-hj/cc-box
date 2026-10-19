/**
 * RFC 4122 v4 UUID。
 * 安全上下文（HTTPS / localhost / Tauri WebView）用 crypto.randomUUID；
 * 局域网 http://<IP> 访问（非安全上下文）中 randomUUID 不可用，回退
 * crypto.getRandomValues（全上下文可用）手拼 v4。
 */
export function uuid(): string {
  if (typeof crypto !== 'undefined' && typeof crypto.randomUUID === 'function') {
    return crypto.randomUUID()
  }
  const bytes = crypto.getRandomValues(new Uint8Array(16))
  bytes[6] = (bytes[6] & 0x0f) | 0x40 // version 4
  bytes[8] = (bytes[8] & 0x3f) | 0x80 // variant 10xx
  const hex = Array.from(bytes, b => b.toString(16).padStart(2, '0')).join('')
  return `${hex.slice(0, 8)}-${hex.slice(8, 12)}-${hex.slice(12, 16)}-${hex.slice(16, 20)}-${hex.slice(20)}`
}
