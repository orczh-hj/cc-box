import { describe, it, expect, vi, beforeEach } from 'vitest'
import { mount, flushPromises } from '@vue/test-utils'
import { setActivePinia, createPinia } from 'pinia'

const mockT = vi.fn((key: string) => key)
vi.mock('vue-i18n', () => ({
  useI18n: () => ({ t: mockT }),
  createI18n: () => ({ global: { locale: { value: 'en' } } }),
}))

// 传输层 mock：uploadFile/downloadFile/getDownloadTicket 可控；quotePath/TransferCancelled 用真实现
const mockUpload = vi.fn()
const mockDownload = vi.fn()
const mockTicket = vi.fn()
vi.mock('@/api/fileTransfer', async (importOriginal) => {
  const actual = await importOriginal<typeof import('@/api/fileTransfer')>()
  return {
    ...actual,
    uploadFile: (...a: unknown[]) => mockUpload(...a),
    downloadFile: (...a: unknown[]) => mockDownload(...a),
    getDownloadTicket: (...a: unknown[]) => mockTicket(...a),
  }
})

vi.mock('@/api/tauri', () => ({
  fileTransferLimits: vi.fn().mockResolvedValue({ directMax: 200 * 1024 * 1024, relayMax: 20 * 1024 * 1024 }),
  fileList: vi.fn(),
  saveLastProject: vi.fn().mockResolvedValue(undefined),
}))

// 面板挂载时订阅 device-status / remote-connected 同步 viaRelay
vi.mock('@/api/transport', () => ({
  transport: { listen: vi.fn().mockResolvedValue(() => {}) },
}))

vi.mock('@/composables/useTerminalCommand', () => ({
  sendTerminalCommand: vi.fn(),
}))

vi.mock('@/api/platform', () => ({
  isTauriEnv: false,
  copyText: vi.fn().mockResolvedValue(undefined),
}))

// viaRelay 按用例切换
const mockRelay = vi.hoisted(() => ({ viaRelay: false }))
vi.mock('@/api/remote-transport', () => ({
  remoteTransport: {
    get viaRelay() { return mockRelay.viaRelay },
  },
}))

const mockSave = vi.fn()
vi.mock('@/utils/platform', () => ({
  saveBlobToFile: (...a: unknown[]) => mockSave(...a),
}))

import FilesPanel from '@/components/files/FilesPanel.vue'
import { useSessionStore } from '@/stores/session'
import { useAppStore } from '@/stores/app'
import { sendTerminalCommand } from '@/composables/useTerminalCommand'

function seedTab(ptyId: string | null) {
  const sessionStore = useSessionStore()
  sessionStore.tabs.set('t1', {
    tabId: 't1', projectPath: 'D:/demo', ptyId, sessionId: null,
    name: 'demo', status: 'running', createdAt: 0, lastActiveAt: 0,
    working: false, pending: false, isResume: false, sessionIds: [],
  })
  sessionStore.activeTabId = 't1'
  return sessionStore
}

function seedSecondTab() {
  const sessionStore = useSessionStore()
  sessionStore.tabs.set('t2', {
    tabId: 't2', projectPath: 'D:/other', ptyId: 'pty-2', sessionId: null,
    name: 'other', status: 'running', createdAt: 0, lastActiveAt: 0,
    working: false, pending: false, isResume: false, sessionIds: [],
  })
}

/** 触发文件选择（jsdom input.files 只读——defineProperty 覆盖实例属性） */
async function chooseFile(wrapper: ReturnType<typeof mount>, file: File) {
  const input = wrapper.find('input[type="file"]').element as HTMLInputElement
  Object.defineProperty(input, 'files', { value: [file] })
  input.value = ''
  input.dispatchEvent(new Event('change'))
  await flushPromises()
}

beforeEach(() => {
  setActivePinia(createPinia())
  vi.clearAllMocks()
  mockRelay.viaRelay = false
  mockUpload.mockReset()
  mockDownload.mockReset()
  mockSave.mockReset()
  // 默认模拟旧桌面（无 file_download_ticket）→ 走 Blob 分块降级
  mockTicket.mockReset()
  mockTicket.mockRejectedValue(new Error('command not allowed: file_download_ticket'))
})

describe('FilesPanel 上传', () => {
  it('FilesPanel_UploadInsertsRelativePath_1', async () => {
    seedTab('pty-1')
    mockUpload.mockResolvedValue({
      targetPath: 'D:/demo/.cc-box-files/a b.txt',
      relativePath: '.cc-box-files/a b.txt',
    })
    const wrapper = mount(FilesPanel)
    await flushPromises()

    await chooseFile(wrapper, new File(['hi'], 'a b.txt'))
    await flushPromises()

    // 插入项目内相对路径（含空格 → 单引号包裹；纯插入不回车）
    expect(mockUpload).toHaveBeenCalledTimes(1)
    expect(sendTerminalCommand).toHaveBeenCalledWith("'.cc-box-files/a b.txt'")
    // done 视图展示相对路径（不展示冗长绝对路径）
    expect(wrapper.text()).toContain('.cc-box-files/a b.txt')
  })

  it('FilesPanel_UploadNoInsertOnSwitchedTab_1', async () => {
    const sessionStore = seedTab('pty-1')
    // 受控 Promise：传输期间用户切换终端
    let resolveUpload!: (v: { targetPath: string; relativePath: string }) => void
    mockUpload.mockReturnValue(
      new Promise((r) => { resolveUpload = r }),
    )
    const wrapper = mount(FilesPanel)
    await flushPromises()

    const input = wrapper.find('input[type="file"]').element as HTMLInputElement
    Object.defineProperty(input, 'files', { value: [new File(['hi'], 'x.txt')] })
    input.dispatchEvent(new Event('change'))
    await flushPromises()

    // 传输在途：切到另一终端
    seedSecondTab()
    sessionStore.activeTabId = 't2'
    resolveUpload({ targetPath: 'D:/demo/.cc-box-files/x.txt', relativePath: '.cc-box-files/x.txt' })
    await flushPromises()

    // 不向已切换的终端插入路径；显示可复制路径
    expect(sendTerminalCommand).not.toHaveBeenCalled()
    expect(wrapper.text()).toContain('.cc-box-files/x.txt')
  })

  it('FilesPanel_UploadPrecheckRelayLimit_1', async () => {
    seedTab('pty-1')
    mockRelay.viaRelay = true
    const wrapper = mount(FilesPanel)
    await flushPromises()

    // 中继档 20MB：30MB 文件前端预检拒绝，不发起 begin
    const big = new File([new Uint8Array(8)], 'big.bin')
    Object.defineProperty(big, 'size', { value: 30 * 1024 * 1024 })
    await chooseFile(wrapper, big)

    expect(mockUpload).not.toHaveBeenCalled()
    expect(wrapper.text()).toContain('filesErrTooLarge')
  })

  it('FilesPanel_UploadNoActiveSession_1', async () => {
    seedTab(null) // stopped / 无 pty
    const wrapper = mount(FilesPanel)
    await flushPromises()

    // 选择按钮禁用；直接触发 change 也被 ptyId 守卫拦下
    const btn = wrapper.find('.primary-btn')
    expect(btn.attributes('disabled')).toBeDefined()
    await chooseFile(wrapper, new File(['hi'], 'a.txt'))
    expect(mockUpload).not.toHaveBeenCalled()
  })

  it('FilesPanel_LimitsUnavailable_UploadsAnyway_1', async () => {
    seedTab('pty-1')
    // 限额查询失败：跳过前端预检，交给服务端权威校验（不得误判为超限）
    const { fileTransferLimits } = await import('@/api/tauri')
    vi.mocked(fileTransferLimits).mockRejectedValueOnce(new Error('rpc timeout'))
    mockUpload.mockResolvedValue({
      targetPath: '/demo/.cc-box-files/a.txt',
      relativePath: '.cc-box-files/a.txt',
    })
    const wrapper = mount(FilesPanel)
    await flushPromises()

    await chooseFile(wrapper, new File(['hi'], 'a.txt'))
    await flushPromises()
    expect(mockUpload).toHaveBeenCalledTimes(1)
  })
})

describe('FilesPanel 下载', () => {
  it('FilesPanel_DownloadSavesBlob_1', async () => {
    seedTab('pty-1')
    const blob = new Blob(['data'])
    mockDownload.mockResolvedValue({ blob, fileName: 'out.txt' })
    const wrapper = mount(FilesPanel)
    await flushPromises()

    const input = wrapper.find('.dl-input')
    await input.setValue('D:/demo/out.txt')
    await wrapper.findAll('button').find(b => b.text() === 'filesDownloadBtn')!.trigger('click')
    await flushPromises()

    expect(mockDownload).toHaveBeenCalledWith('D:/demo/out.txt', expect.anything())
    expect(mockSave).toHaveBeenCalledWith(blob, 'out.txt')
    // done 视图（mock t 不做插值，文件名断言由 mockSave 覆盖）
    expect(wrapper.text()).toContain('filesDownloadDone')
  })

  it('FilesPanel_BrowserCanGoUpAfterEnteringDir_1', async () => {
    seedTab('pty-1')
    const { fileList } = await import('@/api/tauri')
    const mocked = vi.mocked(fileList)
    // 服务端返回带 verbatim 前缀的路径（旧版本）——前端 normPath 应剥离，
    // 保证进入子目录后仍能识别「仍在项目内」→ 上一级可用
    mocked.mockResolvedValueOnce([
      { name: 'sub', path: '\\\\?\\D:\\demo\\sub', isDir: true },
    ])
    mocked.mockResolvedValueOnce([
      { name: 'a.txt', path: '\\\\?\\D:\\demo\\sub\\a.txt', isDir: false, size: 1 },
    ])
    const wrapper = mount(FilesPanel)
    await flushPromises()

    await wrapper.findAll('button').find(b => b.text() === 'filesBrowse')!.trigger('click')
    await flushPromises()
    await wrapper.find('.browser-item').trigger('click')
    await flushPromises()

    // 进入子目录后「上一级」应可用（不再因 verbatim 前缀误判为项目外）
    const upBtn = wrapper.find('.browser-path-row .secondary-btn')
    expect(upBtn.attributes('disabled')).toBeUndefined()
    // 返回的是项目内的子目录
    expect(mocked).toHaveBeenLastCalledWith('\\\\?\\D:\\demo\\sub')
  })

  it('FilesPanel_BrowserPickFileFillsPath_1', async () => {
    seedTab('pty-1')
    const { fileList } = await import('@/api/tauri')
    vi.mocked(fileList).mockResolvedValue([
      { name: 'sub', path: 'D:/demo/sub', isDir: true },
      { name: 'a.txt', path: 'D:/demo/a.txt', isDir: false, size: 3 },
    ])
    const wrapper = mount(FilesPanel)
    await flushPromises()

    // 打开浏览器：入口 = 活动会话项目根
    await wrapper.findAll('button').find(b => b.text() === 'filesBrowse')!.trigger('click')
    await flushPromises()
    expect(fileList).toHaveBeenCalledWith('D:/demo')

    // 点选文件 → 回填**相对项目根**路径并收起（不自动下载）
    await wrapper.findAll('.browser-item').find(b => b.text().includes('a.txt'))!.trigger('click')
    const input = wrapper.find('.dl-input').element as HTMLInputElement
    expect(input.value).toBe('a.txt')
    expect(wrapper.find('.file-browser').exists()).toBe(false)
    expect(mockDownload).not.toHaveBeenCalled()
  })

  it('FilesPanel_ClearPathButton_1', async () => {
    seedTab('pty-1')
    const wrapper = mount(FilesPanel)
    await flushPromises()

    // 空输入时无清空按钮
    expect(wrapper.find('.clear-btn').exists()).toBe(false)
    await wrapper.find('.dl-input').setValue('.cc-box-files/a.jpg')
    expect(wrapper.find('.clear-btn').exists()).toBe(true)

    // 点击 × 清空输入并隐藏按钮
    await wrapper.find('.clear-btn').trigger('click')
    expect((wrapper.find('.dl-input').element as HTMLInputElement).value).toBe('')
    expect(wrapper.find('.clear-btn').exists()).toBe(false)
  })

  it('FilesPanel_CopyPathButton_1', async () => {
    seedTab('pty-1')
    const { copyText } = await import('@/api/platform')
    const wrapper = mount(FilesPanel)
    await flushPromises()

    const copyBtn = () => wrapper.findAll('button').find(b => b.text() === 'filesCopyPath' || b.text() === 'filesCopied')!
    // 空路径时禁用
    expect(copyBtn().attributes('disabled')).toBeDefined()

    await wrapper.find('.dl-input').setValue('.cc-box-files/a.jpg')
    await copyBtn().trigger('click')
    await flushPromises()

    // 只复制路径，不发起下载
    expect(copyText).toHaveBeenCalledWith('.cc-box-files/a.jpg')
    expect(mockDownload).not.toHaveBeenCalled()
    expect(copyBtn().text()).toBe('filesCopied')
  })

  it('FilesPanel_BrowserPickOversizePrecheck_1', async () => {
    seedTab('pty-1')
    mockRelay.viaRelay = true // 中继档 20MB
    const { fileList } = await import('@/api/tauri')
    vi.mocked(fileList).mockResolvedValueOnce([
      { name: 'big.bin', path: 'D:/demo/big.bin', isDir: false, size: 30 * 1024 * 1024 },
    ])
    const wrapper = mount(FilesPanel)
    await flushPromises()

    await wrapper.findAll('button').find(b => b.text() === 'filesBrowse')!.trigger('click')
    await flushPromises()
    await wrapper.find('.browser-item').trigger('click')
    await flushPromises()

    // 点选已知超限文件：提前提示，不发起下载
    expect(wrapper.text()).toContain('filesErrTooLarge')
    expect(mockDownload).not.toHaveBeenCalled()
  })

  it('FilesPanel_DownloadRelativeInputResolved_1', async () => {
    seedTab('pty-1')
    mockDownload.mockResolvedValue({ blob: new Blob(['x']), fileName: 'a.jpg' })
    const wrapper = mount(FilesPanel)
    await flushPromises()

    // 相对路径按项目根解析为绝对路径交给服务端
    await wrapper.find('.dl-input').setValue('.cc-box-files/a.jpg')
    await wrapper.findAll('button').find(b => b.text() === 'filesDownloadBtn')!.trigger('click')
    await flushPromises()

    expect(mockDownload).toHaveBeenCalledWith('D:/demo/.cc-box-files/a.jpg', expect.anything())
  })
})

describe('FilesPanel 当前项目（未启动终端）', () => {
  it('FilesPanel_BrowserUsesCurrentProjectWithoutTerminal_1', async () => {
    seedTab('pty-1') // 旧项目的终端仍然活动
    useAppStore().setCwd('D:/other') // 已打开新项目，但尚未启动终端
    const { fileList } = await import('@/api/tauri')
    const mocked = vi.mocked(fileList)
    mocked.mockImplementation(async (p: string) => [
      p === 'D:/other'
        ? { name: 'new.txt', path: 'D:/other/new.txt', isDir: false, size: 1 }
        : { name: 'old.txt', path: 'D:/demo/old.txt', isDir: false, size: 1 },
    ])
    const wrapper = mount(FilesPanel)
    await flushPromises()

    await wrapper.findAll('button').find(b => b.text() === 'filesBrowse')!.trigger('click')
    await flushPromises()

    // 文件上下文跟随当前项目，而非旧项目的活动终端
    expect(mocked).toHaveBeenLastCalledWith('D:/other')
    expect(wrapper.text()).toContain('new.txt')
  })

  it('FilesPanel_DownloadRelativeUsesCurrentProjectWithoutTerminal_1', async () => {
    seedTab('pty-1')
    useAppStore().setCwd('D:/other')
    mockDownload.mockResolvedValue({ blob: new Blob(['x']), fileName: 'a.jpg' })
    const wrapper = mount(FilesPanel)
    await flushPromises()

    await wrapper.find('.dl-input').setValue('.cc-box-files/a.jpg')
    await wrapper.findAll('button').find(b => b.text() === 'filesDownloadBtn')!.trigger('click')
    await flushPromises()

    expect(mockDownload).toHaveBeenCalledWith('D:/other/.cc-box-files/a.jpg', expect.anything())
  })

  it('FilesPanel_UploadIgnoresOtherProjectTerminal_1', async () => {
    seedTab('pty-1')
    useAppStore().setCwd('D:/other') // 新项目无终端；旧项目终端不属于当前项目
    const wrapper = mount(FilesPanel)
    await flushPromises()

    await chooseFile(wrapper, new File(['hi'], 'a.txt'))

    // 不得把文件上传到旧项目的终端/目录
    expect(mockUpload).not.toHaveBeenCalled()
  })
})

describe('FilesPanel 切换项目', () => {
  it('FilesPanel_SwitchProjectReloadsBrowser_1', async () => {
    const sessionStore = seedTab('pty-1')
    const { fileList } = await import('@/api/tauri')
    const mocked = vi.mocked(fileList)
    // 按根返回不同文件：用于确认列表确实换成了新项目
    mocked.mockImplementation(async (p: string) => [
      p === 'D:/demo'
        ? { name: 'old.txt', path: 'D:/demo/old.txt', isDir: false, size: 1 }
        : { name: 'new.txt', path: 'D:/other/new.txt', isDir: false, size: 1 },
    ])
    const wrapper = mount(FilesPanel)
    await flushPromises()

    // 打开浏览器（旧项目）并进入旧目录列表
    await wrapper.findAll('button').find(b => b.text() === 'filesBrowse')!.trigger('click')
    await flushPromises()
    expect(mocked).toHaveBeenLastCalledWith('D:/demo')
    expect(wrapper.text()).toContain('old.txt')

    // 切到另一个项目：浏览列表必须立即改为新项目根
    seedSecondTab()
    sessionStore.activeTabId = 't2'
    await flushPromises()

    expect(mocked).toHaveBeenLastCalledWith('D:/other')
    expect(wrapper.text()).toContain('new.txt')
    expect(wrapper.text()).not.toContain('old.txt')
  })

  it('FilesPanel_SwitchProjectClearsRelativeInput_1', async () => {
    const sessionStore = seedTab('pty-1')
    const wrapper = mount(FilesPanel)
    await flushPromises()

    // 浏览器点选的路径是相对旧项目根的；切项目后沿用会指向错误目录
    await wrapper.find('.dl-input').setValue('.cc-box-files/a.jpg')

    seedSecondTab()
    sessionStore.activeTabId = 't2'
    await flushPromises()

    expect((wrapper.find('.dl-input').element as HTMLInputElement).value).toBe('')
  })

  it('FilesPanel_SwitchProjectKeepsAbsoluteInput_1', async () => {
    const sessionStore = seedTab('pty-1')
    const wrapper = mount(FilesPanel)
    await flushPromises()

    // 绝对路径含义明确，与项目无关，切项目后保留
    await wrapper.find('.dl-input').setValue('D:/demo/a.jpg')

    seedSecondTab()
    sessionStore.activeTabId = 't2'
    await flushPromises()

    expect((wrapper.find('.dl-input').element as HTMLInputElement).value).toBe('D:/demo/a.jpg')
  })

  it('FilesPanel_SwitchSameProjectKeepsBrowser_1', async () => {
    const sessionStore = seedTab('pty-1')
    const { fileList } = await import('@/api/tauri')
    const mocked = vi.mocked(fileList)
    mocked.mockResolvedValue([
      { name: 'a.txt', path: 'D:/demo/a.txt', isDir: false, size: 1 },
    ])
    const wrapper = mount(FilesPanel)
    await flushPromises()

    await wrapper.findAll('button').find(b => b.text() === 'filesBrowse')!.trigger('click')
    await flushPromises()
    expect(mocked).toHaveBeenCalledTimes(1)

    // 同项目的另一个会话：目录上下文相同，不重置、不重复加载
    sessionStore.tabs.set('t3', {
      tabId: 't3', projectPath: 'D:/demo', ptyId: 'pty-3', sessionId: null,
      name: 'demo-2', status: 'running', createdAt: 0, lastActiveAt: 0,
      working: false, pending: false, isResume: false, sessionIds: [],
    })
    sessionStore.activeTabId = 't3'
    await flushPromises()

    expect(mocked).toHaveBeenCalledTimes(1)
  })
})
