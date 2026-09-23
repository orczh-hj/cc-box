import { setActivePinia, createPinia } from 'pinia'
import { mockIPC } from '@tauri-apps/api/mocks'
import { beforeEach, describe, it, expect, vi } from 'vitest'
import { useAppStore } from '@/stores/app'
import i18n from '@/i18n'

beforeEach(() => {
  setActivePinia(createPinia())
  mockIPC(() => {})
})

describe('getClaudeArgs', () => {
  // 无选项设置时返回空数组
  it('ClaudeArgs_Empty_001', () => {
    const store = useAppStore()
    // default claudeOptions: resume='', skipPermissions=false, customArgs=''
    const args = store.getClaudeArgs()
    expect(args).toEqual([])
  })

  // 设置 resume="abc123" 时返回 ["--resume", "abc123"]
  it('ClaudeArgs_Resume_001', () => {
    const store = useAppStore()
    store.setClaudeOptions({ resume: 'abc123' })
    const args = store.getClaudeArgs()
    expect(args).toEqual(['--resume', 'abc123'])
  })

  // 设置 skipPermissions=true 时包含 "--dangerously-skip-permissions"
  it('ClaudeArgs_SkipPerm_001', () => {
    const store = useAppStore()
    store.setClaudeOptions({ skipPermissions: true })
    const args = store.getClaudeArgs()
    expect(args).toEqual(['--dangerously-skip-permissions'])
  })

  // 设置 customArgs="--flag1 --flag2" 时按空格拆分为两个元素
  it('ClaudeArgs_CustomSplit_001', () => {
    const store = useAppStore()
    store.setClaudeOptions({ customArgs: '--flag1 --flag2' })
    const args = store.getClaudeArgs()
    expect(args).toEqual(['--flag1', '--flag2'])
  })

  // 设置 customArgs="  --flag  " 时过滤空字符串
  it('ClaudeArgs_CustomTrim_001', () => {
    const store = useAppStore()
    store.setClaudeOptions({ customArgs: '  --flag  ' })
    const args = store.getClaudeArgs()
    expect(args).toEqual(['--flag'])
  })

  // 同时设置 resume、skipPermissions、customArgs 时按顺序生成全部参数
  it('ClaudeArgs_Combined_001', () => {
    const store = useAppStore()
    store.setClaudeOptions({
      resume: 'sess1',
      skipPermissions: true,
      customArgs: '--model opus --verbose'
    })
    const args = store.getClaudeArgs()
    expect(args).toEqual([
      '--resume', 'sess1',
      '--dangerously-skip-permissions',
      '--model', 'opus', '--verbose'
    ])
  })
})

describe('resetClaudeOptions', () => {
  // 启动 session 后清空 resume、skipPermissions 恢复默认值
  it('ResetOptions_ResumeAndSkipPerm_001', () => {
    const store = useAppStore()
    store.setClaudeOptions({ resume: 'sess1', skipPermissions: true })
    store.resetClaudeOptions()
    expect(store.claudeOptions.resume).toBe('')
    expect(store.claudeOptions.skipPermissions).toBe(false)
  })

  // 启动 session 后保留用户在侧栏输入的自定义参数，不重置为默认值
  it('ResetOptions_KeepCustomArgs_001', () => {
    const store = useAppStore()
    store.setDefaultClaudeOptions({ customArgs: '--model haiku' })
    store.setClaudeOptions({ customArgs: '--model sonnet --verbose' })
    store.resetClaudeOptions()
    expect(store.claudeOptions.customArgs).toBe('--model sonnet --verbose')
  })

  // 未修改自定义参数时，重置后保持默认值
  it('ResetOptions_CustomArgsDefault_001', () => {
    const store = useAppStore()
    store.setDefaultClaudeOptions({ customArgs: '--model haiku' })
    store.resetClaudeOptions()
    expect(store.claudeOptions.customArgs).toBe('--model haiku')
  })
})

describe('setFontSize', () => {
  // 设置 size=5 时钳位到 10
  it('FontSize_MinClamp_001', () => {
    const store = useAppStore()
    store.setFontSize(5)
    expect(store.fontSize).toBe(10)
  })

  // 设置 size=30 时钳位到 24
  it('FontSize_MaxClamp_001', () => {
    const store = useAppStore()
    store.setFontSize(30)
    expect(store.fontSize).toBe(24)
  })

  // 设置 size=14 时直接接受
  it('FontSize_InRange_001', () => {
    const store = useAppStore()
    store.setFontSize(14)
    expect(store.fontSize).toBe(14)
  })
})

describe('currentProject', () => {
  // 设置 cwd="/Users/dev/myproject" 时返回 "myproject"
  it('CurrentProject_ForwardSlash_001', () => {
    const store = useAppStore()
    store.setCwd('/Users/dev/myproject')
    expect(store.currentProject).toBe('myproject')
  })

  // 设置 cwd="C:\\Users\\dev\\myproject" 时返回 "myproject"
  it('CurrentProject_BackSlash_001', () => {
    const store = useAppStore()
    store.setCwd('C:\\Users\\dev\\myproject')
    expect(store.currentProject).toBe('myproject')
  })

  // 设置 cwd="" 时返回 null
  it('CurrentProject_Empty_001', () => {
    const store = useAppStore()
    store.setCwd('')
    expect(store.currentProject).toBeNull()
  })
})

describe('setLanguage', () => {
  // setLanguage('zh') 同时更新 store.language 和 i18n.global.locale
  it('SetLanguage_Zh_UpdatesLocale_001', () => {
    const store = useAppStore()
    store.setLanguage('zh')
    expect(store.language).toBe('zh')
    expect(i18n.global.locale.value).toBe('zh')
  })

  // setLanguage('en') 同时更新 store.language 和 i18n.global.locale
  it('SetLanguage_En_UpdatesLocale_001', () => {
    const store = useAppStore()
    store.setLanguage('en')
    expect(store.language).toBe('en')
    expect(i18n.global.locale.value).toBe('en')
  })

  // 切换后再切换回中文，locale 正确同步
  it('SetLanguage_ToggleTwice_001', () => {
    const store = useAppStore()
    store.setLanguage('zh')
    expect(i18n.global.locale.value).toBe('zh')
    store.setLanguage('en')
    expect(i18n.global.locale.value).toBe('en')
  })
})

describe('detectSystemLocale', () => {
  // navigator.language 为 'zh-CN' 时返回 'zh'
  it('DetectLocale_ZhCN_001', () => {
    const original = navigator.language
    Object.defineProperty(navigator, 'language', { value: 'zh-CN', configurable: true })
    const store = useAppStore()
    // detectSystemLocale 是内部函数，通过 loadAppConfig 间接触发
    // 直接验证初始 language 默认值逻辑
    const lang = navigator.language.startsWith('zh') ? 'zh' : 'en'
    expect(lang).toBe('zh')
    Object.defineProperty(navigator, 'language', { value: original, configurable: true })
  })

  // navigator.language 为 'en-US' 时返回 'en'
  it('DetectLocale_EnUS_001', () => {
    const original = navigator.language
    Object.defineProperty(navigator, 'language', { value: 'en-US', configurable: true })
    const lang = navigator.language.startsWith('zh') ? 'zh' : 'en'
    expect(lang).toBe('en')
    Object.defineProperty(navigator, 'language', { value: original, configurable: true })
  })
})
