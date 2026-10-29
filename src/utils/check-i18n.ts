// 环境检查文案渲染：后端 CheckResult 返回稳定 code（i18n 键），
// 前端按当前语言翻译；无 code 或未知 code（旧版桌面后端）回退原始文案。
import i18n from '@/i18n'

/** code 命中翻译表则返回当前语言文案，否则回退 fallback */
export function checkCodeText(code: string | undefined, fallback: string): string {
  if (code && i18n.global.te(code)) return i18n.global.t(code)
  return fallback
}

/** action 是 i18n 键则翻译；旧后端的英文动作文本原样保留；缺失时用默认标签 */
export function checkActionText(action: string | undefined, defaultLabel: string): string {
  if (!action) return defaultLabel
  return i18n.global.te(action) ? i18n.global.t(action) : action
}
