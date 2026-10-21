import { describe, it, expect } from 'vitest'
// release.js 是 CJS,module.exports 经 Vitest 互操作为默认导出
import releaseModule from '../../scripts/release.js'

const { generateLatestJson, changelogHasEntry, bumpVersion } = releaseModule as {
  generateLatestJson: (p: Record<string, unknown>) => Record<string, any>
  changelogHasEntry: (content: string, version: string) => boolean
  bumpVersion: (v: string, t: string) => string
}

describe('changelogHasEntry（CHANGELOG 幂等查重,格式无 v 前缀）', () => {
  const content = '# Changelog\n\n## [0.16.0] - 2026-10-21\n\n### Features\n- X\n'

  // 已含版本段 → true(断点续跑跳过依据)
  it('ReleaseScript_ChangelogIdempotent_001', () => {
    expect(changelogHasEntry(content, '0.16.0')).toBe(true)
  })

  // 未含版本 → false;带 v 前缀的错误格式不误命中(写入格式无 v)
  it('ReleaseScript_ChangelogIdempotent_002', () => {
    expect(changelogHasEntry(content, '0.17.0')).toBe(false)
    expect(changelogHasEntry('# Changelog\n\n## [v0.16.0] - 2026-10-21\n', '0.16.0')).toBe(false)
  })
})

describe('bumpVersion', () => {
  it('ReleaseScript_BumpVersion_001', () => {
    expect(bumpVersion('0.15.9', 'patch')).toBe('0.15.10')
    expect(bumpVersion('0.15.9', 'minor')).toBe('0.16.0')
    expect(bumpVersion('0.15.9', 'major')).toBe('1.0.0')
  })
})

describe('generateLatestJson（双格式 updater 清单）', () => {
  const files = [
    { name: 'CC-Box_0.16.0_x64-setup.exe', path: 'a.exe' },
    { name: 'CC-Box_0.16.0_x64-setup.exe.sig', path: 'a.exe.sig' },
    { name: 'CC-Box.app.tar.gz', path: 't.tar.gz' },
    { name: 'CC-Box.app.tar.gz.sig', path: 't.sig' },
    { name: 'CC-Box_0.16.0_aarch64.dmg', path: 'm.dmg' },
    { name: 'CC-Box_0.16.0_amd64.AppImage', path: 'l.AppImage' },
    { name: 'CC-Box_0.16.0_amd64.AppImage.sig', path: 'l.sig' },
  ]
  const json = generateLatestJson({
    version: 'v0.16.0',
    releaseNotes: 'notes here',
    files,
    bucketName: 'cc-box',
    endpoint: 'oss-cn-beijing.aliyuncs.com',
    readFileContent: (p: string) => (p ? `sig(${p})` : ''),
    getFileSize: (p: string) => (p ? 123 : 0),
  })

  // version 字段无 v 前缀（Tauri 官方 updater 格式;OSS 幂等校验依赖此格式）
  it('ReleaseScript_LatestJsonVersion_001', () => {
    expect(json.version).toBe('0.16.0')
    expect(json.notes).toBe('notes here')
  })

  // macOS 架构从 dmg 文件名推断;三平台 platforms 完整
  it('ReleaseScript_LatestJsonPlatforms_002', () => {
    expect(json.platforms['darwin-aarch64']).toBeTruthy()
    expect(json.platforms['windows-x86_64'].url).toContain('cc-box/v0.16.0/CC-Box_0.16.0_x64-setup.exe')
    expect(json.platforms['linux-x86_64'].signature).toBe('sig(l.sig)')
  })

  // 旧版本兼容格式 assets 三平台齐全
  it('ReleaseScript_LatestJsonLegacyAssets_003', () => {
    expect(json.assets.windows.url).toContain('-setup.exe')
    expect(json.assets.macos.url).toContain('.dmg')
    expect(json.assets.linux.size).toBe(123)
  })
})
