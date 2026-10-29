import type { DesktopPlatform } from './ua'

export interface AssetView {
  fileName: string
  url: string
  size?: number
}

export interface LatestView {
  version: string
  releaseDate?: string
  notes?: string
  releaseNotesUrl?: string
  assets: Record<'windows' | 'macos' | 'linux', AssetView>
}

const LATEST_URL = '/cc-box/latest.json'
const FETCH_TIMEOUT_MS = 10_000

function toAssetView(value: unknown): AssetView {
  if (typeof value !== 'object' || value === null) throw new Error('asset 不是对象')
  const { url, size } = value as { url?: unknown; size?: unknown }
  if (typeof url !== 'string' || url.length === 0) throw new Error('asset.url 缺失')
  const fileName = url.split('/').pop() ?? ''
  return {
    fileName,
    url,
    size: typeof size === 'number' && Number.isFinite(size) && size > 0 ? size : undefined,
  }
}

// latest.json → 下载区视图模型；version 或任一平台 asset 缺失视为结构损坏，抛错
export function toDownloadViewModel(json: unknown): LatestView {
  if (typeof json !== 'object' || json === null) throw new Error('latest.json 不是对象')
  const { version, release_date, notes, release_notes_url, assets } = json as Record<string, unknown>
  if (typeof version !== 'string' || version.length === 0) throw new Error('version 缺失')
  if (typeof assets !== 'object' || assets === null) throw new Error('assets 缺失')
  return {
    version,
    releaseDate: typeof release_date === 'string' ? release_date : undefined,
    notes: typeof notes === 'string' ? notes : undefined,
    releaseNotesUrl: typeof release_notes_url === 'string' ? release_notes_url : undefined,
    assets: {
      windows: toAssetView((assets as Record<string, unknown>).windows),
      macos: toAssetView((assets as Record<string, unknown>).macos),
      linux: toAssetView((assets as Record<string, unknown>).linux),
    },
  }
}

// 拉取最新版本信息；任何失败（网络/超时/非 200/结构损坏）返回 null，不抛出
export async function fetchLatestRelease(): Promise<LatestView | null> {
  const controller = new AbortController()
  const timer = setTimeout(() => controller.abort(), FETCH_TIMEOUT_MS)
  try {
    const res = await fetch(LATEST_URL, { signal: controller.signal })
    if (!res.ok) return null
    return toDownloadViewModel(await res.json())
  } catch {
    return null
  } finally {
    clearTimeout(timer)
  }
}

export const PLATFORM_ORDER: DesktopPlatform[] = ['windows', 'macos', 'linux']
