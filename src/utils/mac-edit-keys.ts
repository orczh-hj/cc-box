import { isMac } from '@/utils/platform'
import { isTauriEnv } from '@/api/platform'

/**
 * macOS 桌面原生 Edit 菜单联动：菜单 key equivalent 会在按键到达 WebView
 * 前被系统拦截（这是输入框 Cmd+V 原生粘贴的来源，也是终端 Cmd+C 曾失效的
 * 根因），因此终端持焦时必须把菜单切走，输入框持焦时再切回来。
 */

/** mac 桌面才启用菜单联动（浏览器远程模式无本地菜单） */
export const nativeEditMenuSupported = isMac && isTauriEnv

/**
 * 焦点目标是否应抑制原生 Edit 菜单：仅 xterm 隐藏 textarea（终端持焦）为真，
 * xterm 的 Cmd+C/V/Z 等自定义按键处理依赖 keydown 到达 WebView
 */
export function suppressNativeEditMenu(target: EventTarget | null): boolean {
  return target instanceof HTMLElement && target.classList.contains('xterm-helper-textarea')
}
