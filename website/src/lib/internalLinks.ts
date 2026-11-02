// 参考文档正文里 code.claude.com 文档链接 → 本地文档 key 的解析（纯函数，便于测试）
export interface DocEntryLike {
  key: string
}

// 官方文档 URL 形如 https://code.claude.com/docs/(zh-CN/|en/)<path>(.md)，
// path 可能是单段（settings.md）或多段嵌套（plugins/mods/interface.md），
// 且正文里大量链接不带 .md 后缀（如 /docs/zh-CN/advisor）——后缀可选；
// 多段路径与本地 key（plugins/mods/interface）逐段一致
export function resolveInternalDocLink<T extends DocEntryLike>(
  url: URL,
  manifest: T[],
): T | null {
  if (url.hostname !== 'code.claude.com') return null
  const m = url.pathname.match(/^\/docs\/(?:zh-CN\/|en\/)?([a-z0-9-]+(?:\/[a-z0-9-]+)*)(?:\.md)?$/)
  if (!m) return null
  const path = m[1]
  return manifest.find((en) => en.key === path || en.key.endsWith(`/${path}`)) ?? null
}
