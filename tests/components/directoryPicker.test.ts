import { describe, it, expect, vi, beforeEach } from 'vitest'
import { mount, flushPromises } from '@vue/test-utils'
import { mockIPC } from '@tauri-apps/api/mocks'

const mockT = vi.fn((key: string) => key)
vi.mock('vue-i18n', () => ({
  useI18n: () => ({ t: mockT }),
}))

import DirectoryPicker from '@/components/DirectoryPicker.vue'

/** 根条目（盘符）与子目录条目 */
const ROOT = [
  { name: 'C:\\', path: 'C:\\' },
  { name: 'D:\\', path: 'D:\\' },
]
const SUBS = [
  { name: 'projects', path: 'C:\\projects' },
  { name: 'work', path: 'C:\\work' },
]

function mountPicker() {
  return mount(DirectoryPicker, {
    props: { visible: true },
    attachTo: document.body,
  })
}

describe('DirectoryPicker（远程端电脑目录选择器）', () => {
  beforeEach(() => {
    mockT.mockClear()
    document.body.innerHTML = ''
  })

  // 打开即从根（盘符）开始；点击目录进入下一级；选择当前目录 emit select
  it('DirectoryPicker_NavigateAndSelect_001', async () => {
    const calls: Array<string | null | undefined> = []
    mockIPC((cmd, args) => {
      if (cmd === 'list_directories') {
        calls.push((args as { path?: string | null })?.path)
        return (args as { path?: string | null })?.path ? SUBS : ROOT
      }
      return null
    })

    const wrapper = mountPicker()
    await flushPromises()

    // 根：path 为 null（Rust 侧 Option<String> 反序列化 None）
    expect(calls[0]).toBeNull()
    let items = [...document.body.querySelectorAll('.dir-item')].map(b => b.textContent || '')
    expect(items).toHaveLength(2)
    // 根路径下不能"选择当前"（必须进入具体目录）
    const selectBtn = document.body.querySelector('.primary-btn') as HTMLButtonElement
    expect(selectBtn.disabled).toBe(true)

    // 进入 C:\
    const first = document.body.querySelector('.dir-item') as HTMLButtonElement
    first.click()
    await flushPromises()
    expect(calls[1]).toBe('C:\\')
    items = [...document.body.querySelectorAll('.dir-item')].map(b => b.textContent || '')
    expect(items).toHaveLength(2)

    // 选择当前目录
    selectBtn.disabled = false
    selectBtn.click()
    await flushPromises()
    expect(wrapper.emitted('select')).toEqual([['C:\\']])
    wrapper.unmount()
  })
})
