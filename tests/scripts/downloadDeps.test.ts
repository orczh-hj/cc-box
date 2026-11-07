// @vitest-environment node
import { describe, it, expect } from 'vitest'
// download-deps.js 是 CJS,module.exports 经 Vitest 互操作为默认导出
import depsModule from '../../scripts/download-deps.js'

const {
  parseVersionList,
  parsePlatformList,
  npmTarballName,
  npmTarballUrl,
  binaryRelPathInTarball,
  buildVersionEntryFromPlatformInfos,
  mergeVersionEntries,
} = depsModule as {
  parseVersionList: (raw: string) => string[]
  parsePlatformList: (raw: string | undefined) => string[]
  npmTarballName: (platform: string, version: string) => string
  npmTarballUrl: (platform: string, version: string) => string
  binaryRelPathInTarball: (platform: string) => string
  buildVersionEntryFromPlatformInfos: (v: string, p: any[], d: string) => any
  mergeVersionEntries: (a: any[], b: any) => any[]
}

describe('download-deps npm 源', () => {
  // 逗号/空白分隔的版本列表解析:修剪空白、去重保序、拒绝非法版本号
  it('DownloadDeps_ParseVersionList_001', () => {
    expect(parseVersionList('2.1.318, 2.1.316 ,2.1.251')).toEqual(['2.1.318', '2.1.316', '2.1.251'])
    expect(parseVersionList('2.1.318,2.1.318')).toEqual(['2.1.318'])
    expect(parseVersionList('')).toEqual([])
    // 非法版本号(字母/空段)拒绝,不是静默带上
    expect(() => parseVersionList('2.1.318,abc')).toThrow(/abc/)
    expect(() => parseVersionList('v2.1.318')).toThrow()
  })

  // 平台白名单:未知平台丢弃;缺省(undefined)=三主平台
  it('DownloadDeps_ParsePlatformList_002', () => {
    expect(parsePlatformList(undefined)).toEqual(['win32-x64', 'darwin-arm64', 'linux-x64'])
    expect(parsePlatformList('win32-x64,linux-arm64,musl-x')).toEqual(['win32-x64', 'linux-arm64'])
  })

  // npm 平台包 tarball 命名与 cdn 地址(实测 302 目标形态)
  it('DownloadDeps_NpmTarballUrl_003', () => {
    expect(npmTarballName('win32-x64', '2.1.316')).toBe('claude-code-win32-x64-2.1.316.tgz')
    expect(npmTarballUrl('win32-x64', '2.1.316')).toBe(
      'https://cdn.npmmirror.com/packages/%40anthropic-ai/claude-code-win32-x64/2.1.316/claude-code-win32-x64-2.1.316.tgz',
    )
    // darwin/linux 同形态
    expect(npmTarballUrl('darwin-arm64', '2.1.251')).toBe(
      'https://cdn.npmmirror.com/packages/%40anthropic-ai/claude-code-darwin-arm64/2.1.251/claude-code-darwin-arm64-2.1.251.tgz',
    )
  })

  // tarball 内二进制路径:win32 带 .exe(实测 2.1.316 包内仅 package/claude.exe)
  it('DownloadDeps_BinaryRelPath_004', () => {
    expect(binaryRelPathInTarball('win32-x64')).toBe('package/claude.exe')
    expect(binaryRelPathInTarball('win32-arm64')).toBe('package/claude.exe')
    expect(binaryRelPathInTarball('darwin-arm64')).toBe('package/claude')
    expect(binaryRelPathInTarball('linux-x64')).toBe('package/claude')
  })

  // 同版本补传部分平台:按平台合并,不丢其他平台条目(曾因整体替换丢过)
  it('DownloadDeps_MergeSameVersionKeepsPlatforms_006', () => {
    const existing = [{
      version: '2.1.295', release_date: '2026-10-08',
      platforms: {
        'darwin-arm64': { url: 'u1', checksum: 'c1', size: 1 },
        'linux-x64': { url: 'u2', checksum: 'c2', size: 2 },
      },
    }]
    const patch = {
      version: '2.1.295', release_date: '',
      platforms: { 'win32-x64': { url: 'u3', checksum: 'c3', size: 3 } },
    }
    const merged = mergeVersionEntries(existing, patch)
    expect(Object.keys(merged[0].platforms).sort()).toEqual(['darwin-arm64', 'linux-x64', 'win32-x64'])
    expect(merged[0].release_date).toBe('2026-10-08') // 空日期不覆盖已有值
  })

  // versions.json entry 的 release_date 取 npm 真实发布日期(而非运行当天)
  it('DownloadDeps_EntryReleaseDate_005', () => {
    const entry = buildVersionEntryFromPlatformInfos(
      '2.1.316',
      [{ platform: 'win32-x64', filename: 'claude.exe', checksum: 'ab'.repeat(32), size: 123 }],
      '2026-10-31',
    )
    expect(entry.release_date).toBe('2026-10-31')
    expect(entry.platforms['win32-x64'].url).toBe('deps/claude/2.1.316/win32-x64/claude.exe')
    expect(entry.platforms['win32-x64'].size).toBe(123)
    // 额外平台条目保留(多平台合并新版本时)
    const merged = mergeVersionEntries(
      [{ version: '2.1.179', release_date: '2026-06-17', platforms: {} }],
      entry,
    )
    expect(merged.map((e: any) => e.version)).toEqual(['2.1.316', '2.1.179'])
  })
})
