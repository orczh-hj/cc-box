// 字节数 → 人类可读大小；非法值（非有限数、<=0）显示 '—'
export function formatBytes(bytes: unknown): string {
  if (typeof bytes !== 'number' || !Number.isFinite(bytes) || bytes <= 0) return '—'
  const KB = 1024
  const MB = KB * 1024
  const GB = MB * 1024
  if (bytes < MB) return `${Math.round(bytes / KB)} KB`
  if (bytes < GB) return `${(bytes / MB).toFixed(1)} MB`
  return `${(bytes / GB).toFixed(1)} GB`
}
