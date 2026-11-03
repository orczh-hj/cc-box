// reference.html 路由 hash 解析：#/<key>?a=<encodeURIComponent(章节标题)>
// 非 / 开头视为页内锚点（key=''，维持既有 getElementById 行为）

export interface RefHash {
  key: string
  anchor?: string
}

export function parseRefHash(hash: string): RefHash {
  const raw = hash.startsWith('#') ? hash.slice(1) : hash
  if (!raw.startsWith('/')) return { key: '' }
  let path = raw.slice(1)
  const q = path.indexOf('?')
  let anchor: string | undefined
  if (q !== -1) {
    const params = new URLSearchParams(path.slice(q + 1))
    const a = params.get('a')
    if (a) anchor = a
    path = path.slice(0, q)
  }
  const key = path.replace(/\/+$/, '')
  return anchor !== undefined ? { key, anchor } : { key }
}
