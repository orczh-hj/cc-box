import { describe, it, expect } from 'vitest'
import { relativizePath, normalizeWinPathSeparators } from '@/utils/path'

describe('relativizePath', () => {
  // Windows 下文件位于项目目录内 → 返回用 '/' 分隔的相对路径
  it('RelativizePath_WinInside_001', () => {
    expect(
      relativizePath('C:\\proj\\src\\main.ts', 'C:\\proj'),
    ).toBe('src/main.ts')
  })

  // Windows 下盘符大小写不同仍视为同根
  it('RelativizePath_WinDriveCase_002', () => {
    expect(
      relativizePath('c:\\proj\\src\\a.ts', 'C:\\proj'),
    ).toBe('src/a.ts')
  })

  // Windows 下目录名大小写不同仍视为同根
  it('RelativizePath_WinDirCase_003', () => {
    expect(
      relativizePath('C:\\Proj\\Src\\a.ts', 'C:\\proj'),
    ).toBe('Src/a.ts')
  })

  // Windows 下文件位于项目目录外 → 原样返回
  it('RelativizePath_WinOutside_004', () => {
    expect(
      relativizePath('D:\\other\\file.ts', 'C:\\proj'),
    ).toBe('D:\\other\\file.ts')
  })

  // Windows 下文件与项目根完全相同 → 返回 '.'
  it('RelativizePath_WinSameRoot_005', () => {
    expect(relativizePath('C:\\proj', 'C:\\proj')).toBe('.')
  })

  // Windows 下文件与项目根仅大小写不同 → 返回 '.'
  it('RelativizePath_WinSameRootCase_006', () => {
    expect(relativizePath('C:\\Proj', 'c:\\proj')).toBe('.')
  })

  // 项目路径末尾带反斜杠也应正确识别
  it('RelativizePath_WinTrailingSep_007', () => {
    expect(
      relativizePath('C:\\proj\\src\\a.ts', 'C:\\proj\\'),
    ).toBe('src/a.ts')
  })

  // 项目根名是其他项目根的前缀时不应误判（C:\proj vs C:\proj-other）
  it('RelativizePath_WinPrefixSibling_008', () => {
    expect(
      relativizePath('C:\\proj-other\\a.ts', 'C:\\proj'),
    ).toBe('C:\\proj-other\\a.ts')
  })

  // macOS 下文件位于项目目录内 → 返回相对路径
  it('RelativizePath_MacInside_001', () => {
    expect(
      relativizePath('/Users/u/proj/src/a.ts', '/Users/u/proj'),
    ).toBe('src/a.ts')
  })

  // macOS 下大小写敏感：目录名大小写不同视为不同项目
  it('RelativizePath_MacCaseSensitive_002', () => {
    expect(
      relativizePath('/Users/u/Proj/src/a.ts', '/Users/u/proj'),
    ).toBe('/Users/u/Proj/src/a.ts')
  })

  // macOS 下文件位于项目目录外 → 原样返回
  it('RelativizePath_MacOutside_003', () => {
    expect(
      relativizePath('/etc/hosts', '/Users/u/proj'),
    ).toBe('/etc/hosts')
  })

  // macOS 下文件与项目根完全相同 → 返回 '.'
  it('RelativizePath_MacSameRoot_004', () => {
    expect(relativizePath('/Users/u/proj', '/Users/u/proj')).toBe('.')
  })

  // macOS 下兄弟前缀目录不误判（/a/proj vs /a/proj-other）
  it('RelativizePath_MacPrefixSibling_005', () => {
    expect(
      relativizePath('/a/proj-other/a.ts', '/a/proj'),
    ).toBe('/a/proj-other/a.ts')
  })

  // 项目路径为空 → 原样返回
  it('RelativizePath_EmptyProject_001', () => {
    expect(relativizePath('/foo/bar.ts', '')).toBe('/foo/bar.ts')
  })

  // 文件路径为空 → 原样返回
  it('RelativizePath_EmptyFile_002', () => {
    expect(relativizePath('', '/foo')).toBe('')
  })

  // 嵌套子目录也能正确计算相对路径
  it('RelativizePath_DeepNested_001', () => {
    expect(
      relativizePath('/a/b/c/d/e.ts', '/a/b'),
    ).toBe('c/d/e.ts')
  })
})

describe('normalizeWinPathSeparators', () => {
  // Windows 盘符路径里的 '/' 统一为 '\'（DirectoryPicker goUp 的 '/' 拼接与
  // Rust entry.path() 的 '\' 追加会产生混合斜杠，落盘/展示都不该带出去）
  it('NormalizeWinPath_MixedSeparators_001', () => {
    expect(
      normalizeWinPathSeparators('D:/orczh/Documents\\Project\\my proj'),
    ).toBe('D:\\orczh\\Documents\\Project\\my proj')
  })

  // 盘符根（D:/）也统一为反斜杠
  it('NormalizeWinPath_DriveRoot_002', () => {
    expect(normalizeWinPathSeparators('D:/')).toBe('D:\\')
  })

  // 非 Windows 路径（Unix 绝对路径 / 相对路径）原样返回
  it('NormalizeWinPath_UnixUntouched_003', () => {
    expect(normalizeWinPathSeparators('/home/user/proj')).toBe('/home/user/proj')
    expect(normalizeWinPathSeparators('')).toBe('')
  })
})
