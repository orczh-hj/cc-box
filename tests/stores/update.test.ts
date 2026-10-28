import { describe, it, expect, beforeEach } from 'vitest'
import { setActivePinia, createPinia } from 'pinia'
import { useUpdateStore } from '@/stores/update'

describe('update store', () => {
  beforeEach(() => {
    setActivePinia(createPinia())
  })

  describe('resetDownload', () => {
    // resetDownload 将所有下载状态恢复到初始值
    it('Reset_FullRestore_001', () => {
      const store = useUpdateStore()
      store.setDownloadState('downloading')
      store.setDownloadProgress({ downloaded: 800000, total: 1000000, percent: 80 })
      store.setDownloadError('some error')

      store.resetDownload()

      expect(store.downloadState).toBe('idle')
      expect(store.downloadProgress).toEqual({ downloaded: 0, total: 0, percent: 0 })
      expect(store.downloadError).toBe('')
    })
  })

  describe('historyDownloads', () => {
    // setHistoryDownload 增量更新
    it('HistoryDownload_Patch_001', () => {
      const store = useUpdateStore()
      store.setHistoryDownload('1.0.17', { state: 'downloading', progress: 0, message: '', savedPath: '', error: '' })
      store.setHistoryDownload('1.0.17', { progress: 80 })
      const d = store.getHistoryDownload('1.0.17')
      expect(d?.progress).toBe(80)
      expect(d?.state).toBe('downloading')
    })

    // resetHistoryDownload 删除条目
    it('HistoryDownload_Reset_001', () => {
      const store = useUpdateStore()
      store.setHistoryDownload('1.0.17', { state: 'done', progress: 100, savedPath: 'p', message: '', error: '' })
      store.resetHistoryDownload('1.0.17')
      expect(store.getHistoryDownload('1.0.17')).toBeUndefined()
    })

    // 安装中状态：downloading → installing，savedPath 保留
    it('HistoryDownload_Installing_001', () => {
      const store = useUpdateStore()
      store.setHistoryDownload('1.0.17', { state: 'downloading', progress: 100, savedPath: 'C:\\Downloads\\c.exe', message: '', error: '' })
      store.setHistoryDownload('1.0.17', { state: 'installing', progress: 0, message: '正在安装' })
      const d = store.getHistoryDownload('1.0.17')
      expect(d?.state).toBe('installing')
      expect(d?.savedPath).toBe('C:\\Downloads\\c.exe')
    })

    // 安装完成：installing → done，savedPath 保留
    it('HistoryDownload_Installing_To_Done_001', () => {
      const store = useUpdateStore()
      store.setHistoryDownload('1.0.17', { state: 'installing', progress: 0, savedPath: 'p', message: '', error: '' })
      store.setHistoryDownload('1.0.17', { state: 'done', progress: 100, message: '安装完成' })
      const d = store.getHistoryDownload('1.0.17')
      expect(d?.state).toBe('done')
      expect(d?.savedPath).toBe('p')
    })
  })
})
