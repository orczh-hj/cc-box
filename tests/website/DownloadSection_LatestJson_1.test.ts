import { afterEach, describe, expect, it, vi } from 'vitest'
import { fetchLatestRelease, toDownloadViewModel } from '../../website/src/lib/latest'

// 与 release.js generateLatestJson 输出结构一致的真实 fixture（v0.17.1）
const FIXTURE = {
  version: '0.17.1',
  pub_date: '2026-10-28T00:00:00.000Z',
  release_date: '2026-10-28',
  notes: '发布说明',
  release_notes_url: 'https://github.com/orczh-hj/cc-box/releases/tag/v0.17.1',
  assets: {
    windows: {
      url: 'https://cc-box.oss-cn-beijing.aliyuncs.com/cc-box/v0.17.1/CC-Box_0.17.1_x64-setup.exe',
      size: 94371840,
    },
    macos: {
      url: 'https://cc-box.oss-cn-beijing.aliyuncs.com/cc-box/v0.17.1/CC-Box_0.17.1_aarch64.dmg',
      size: 512000,
    },
    linux: {
      url: 'https://cc-box.oss-cn-beijing.aliyuncs.com/cc-box/v0.17.1/CC-Box_0.17.1_amd64.AppImage',
      size: 0,
    },
  },
}

afterEach(() => {
  vi.unstubAllGlobals()
})

describe('DownloadSection_LatestJson_toDownloadViewModel', () => {
  it('完整 fixture 转出三平台视图模型，fileName 从 url 末段提取', () => {
    const view = toDownloadViewModel(FIXTURE)
    expect(view.version).toBe('0.17.1')
    expect(view.releaseDate).toBe('2026-10-28')
    expect(view.releaseNotesUrl).toBe(FIXTURE.release_notes_url)
    expect(view.assets.windows.fileName).toBe('CC-Box_0.17.1_x64-setup.exe')
    expect(view.assets.windows.size).toBe(94371840)
    expect(view.assets.macos.fileName).toBe('CC-Box_0.17.1_aarch64.dmg')
    expect(view.assets.linux.url).toBe(FIXTURE.assets.linux.url)
  })

  it('size 为 0 视为缺失（转 undefined，由 formatBytes 显示 —）', () => {
    const view = toDownloadViewModel(FIXTURE)
    expect(view.assets.linux.size).toBeUndefined()
  })

  it('缺少 assets.macos 抛错', () => {
    const broken = { ...FIXTURE, assets: { windows: FIXTURE.assets.windows, linux: FIXTURE.assets.linux } }
    expect(() => toDownloadViewModel(broken)).toThrow()
  })

  it('缺少 version 抛错', () => {
    const broken = { ...FIXTURE, version: '' }
    expect(() => toDownloadViewModel(broken)).toThrow()
  })

  it('非对象输入抛错', () => {
    expect(() => toDownloadViewModel(null)).toThrow()
    expect(() => toDownloadViewModel('x')).toThrow()
  })
})

describe('DownloadSection_LatestJson_fetchLatestRelease', () => {
  it('fetch 返回 200 与合法 JSON 时解析成功', async () => {
    vi.stubGlobal('fetch', vi.fn().mockResolvedValue({
      ok: true,
      json: () => Promise.resolve(FIXTURE),
    }))
    const view = await fetchLatestRelease()
    expect(view?.version).toBe('0.17.1')
  })

  it('HTTP 非 200 返回 null 而不抛出', async () => {
    vi.stubGlobal('fetch', vi.fn().mockResolvedValue({ ok: false, status: 404 }))
    expect(await fetchLatestRelease()).toBeNull()
  })

  it('网络异常返回 null 而不抛出', async () => {
    vi.stubGlobal('fetch', vi.fn().mockRejectedValue(new TypeError('network error')))
    expect(await fetchLatestRelease()).toBeNull()
  })

  it('JSON 结构损坏返回 null 而不抛出', async () => {
    vi.stubGlobal('fetch', vi.fn().mockResolvedValue({
      ok: true,
      json: () => Promise.resolve({ foo: 1 }),
    }))
    expect(await fetchLatestRelease()).toBeNull()
  })
})
