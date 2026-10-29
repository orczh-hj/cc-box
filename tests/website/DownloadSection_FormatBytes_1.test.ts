import { describe, expect, it } from 'vitest'
import { formatBytes } from '../../website/src/lib/format'

// 期望值全部手算：1024 进制，MB 一位小数
describe('DownloadSection_FormatBytes', () => {
  it('0、负数、NaN、非数字显示 —', () => {
    expect(formatBytes(0)).toBe('—')
    expect(formatBytes(-1024)).toBe('—')
    expect(formatBytes(Number.NaN)).toBe('—')
    expect(formatBytes('8.9 MB')).toBe('—')
  })

  it('500KB（512000 字节）显示 500 KB', () => {
    expect(formatBytes(500 * 1024)).toBe('500 KB')
  })

  it('915MB（959447040 字节）显示 915.0 MB', () => {
    expect(formatBytes(915 * 1024 * 1024)).toBe('915.0 MB')
  })

  it('90MB（94371840 字节）显示 90.0 MB', () => {
    expect(formatBytes(90 * 1024 * 1024)).toBe('90.0 MB')
  })

  it('2GB（2147483648 字节）显示 2.0 GB', () => {
    expect(formatBytes(2 * 1024 * 1024 * 1024)).toBe('2.0 GB')
  })

  it('不足 1KB 的值向下取整为 1 KB 以内的整数 KB', () => {
    expect(formatBytes(512)).toBe('1 KB')
    expect(formatBytes(100)).toBe('0 KB')
  })
})
