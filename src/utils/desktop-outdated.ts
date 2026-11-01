/**
 * 旧桌面提示：新前端调用旧桌面不存在的 RPC 命令时（中继页面总是最新 dist），
 * 提示用户更新桌面端，而不是让原始错误在每个调用点冒出来。
 *
 * 文案由应用启动时注入（main.ts）：transport 层静态导入 i18n 会让「只 mock 了
 * vue-i18n 部分 API」的测试在收集阶段就失败（createI18n 缺失），故此处只依赖
 * toast，文案通过 provider 取用。
 *
 * 每次会话只提示一次：调用失败往往是连续的（并发初始化请求），逐条提示会
 * 刷屏并遮挡界面。
 */
import { useToast } from '@/composables/useToast'

let notified = false
let textProvider: (() => string) | null = null

/** 应用启动时注入「请更新桌面端」文案（随界面语言） */
export function setDesktopOutdatedTextProvider(fn: (() => string) | null): void {
  textProvider = fn
}

export function notifyDesktopOutdated(): void {
  if (notified) return
  notified = true
  try {
    const text = textProvider?.() ?? ''
    if (text) useToast().error(text, 8000)
  } catch {
    // 提示失败不影响主流程（绝不因提示本身抛错打断调用链）
  }
}

/** 测试隔离 / 重连后重新允许提示 */
export function resetDesktopOutdatedNotice(): void {
  notified = false
}
