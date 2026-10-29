import { describe, it, expect, beforeEach } from 'vitest'
import i18n from '@/i18n'
import { checkCodeText, checkActionText } from '@/utils/check-i18n'

describe('check-i18n 环境检查文案渲染', () => {
  beforeEach(() => {
    i18n.global.locale.value = 'zh'
  })

  // code 命中翻译表 → 当前语言文案（期望值来自 zh.ts 字面量）
  it('CheckI18n_CodeZh_001', () => {
    expect(checkCodeText('checkClaudeNotFound', 'fallback')).toBe('未找到 Claude CLI，可在下方设置自定义路径')
    expect(checkCodeText('checkFoundConfigured', 'fallback')).toBe('已找到（用户配置）')
  })

  // 同一 code 在英文 locale 下返回英文文案
  it('CheckI18n_CodeEn_002', () => {
    i18n.global.locale.value = 'en'
    expect(checkCodeText('checkClaudeNotFound', 'fallback')).toBe('Claude CLI not found. You can set a custom path below.')
  })

  // 未知 code（未来后端新增、前端未更新）→ 回退原始文案
  it('CheckI18n_UnknownCodeFallback_003', () => {
    expect(checkCodeText('checkSomethingNew', 'raw message')).toBe('raw message')
  })

  // 无 code（旧版桌面后端）→ 回退原始文案
  it('CheckI18n_NoCodeFallback_004', () => {
    expect(checkCodeText(undefined, 'Claude CLI not found.')).toBe('Claude CLI not found.')
  })

  // action 为 i18n 键 → 翻译
  it('CheckI18n_ActionKey_005', () => {
    expect(checkActionText('installGitForWindows', '默认标签')).toBe('下载安装 Git for Windows')
    expect(checkActionText('viewInstallationGuide', '默认标签')).toBe('查看安装指南')
  })

  // 旧后端的英文动作文本（非 i18n 键）→ 原样保留，不用默认标签覆盖
  it('CheckI18n_ActionLegacyText_006', () => {
    expect(checkActionText('View installation guide', '默认标签')).toBe('View installation guide')
  })

  // action 缺失 → 默认标签
  it('CheckI18n_ActionMissing_007', () => {
    expect(checkActionText(undefined, '查看文档')).toBe('查看文档')
  })
})
