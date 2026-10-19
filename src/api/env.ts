/**
 * 运行环境检测（独立模块，打破 transport ↔ 实现的循环依赖）
 *
 * 顶层立即执行，先于任何 Tauri API 调用；__TAURI_INTERNALS__ 由 Tauri 注入。
 */

export interface TransportEnv {
  /** 桌面 Tauri 环境 */
  isTauri: boolean
  /** 浏览器远程模式（非 Tauri 环境一律视为远程 WS 模式） */
  isBrowserRemote: boolean
}

const isTauri =
  // 测试钩子：tests/test-setup.ts 强制 Tauri 分支（jsdom 无 __TAURI_INTERNALS__，
  // 而既有测试均以"Tauri 环境 + mockIPC"为前提）
  (typeof globalThis !== 'undefined' &&
    (globalThis as Record<string, unknown>).__CCBOX_TEST_FORCE_TAURI__ === true) ||
  (typeof window !== 'undefined' && '__TAURI_INTERNALS__' in window)

export const transportEnv: TransportEnv = {
  isTauri,
  isBrowserRemote: !isTauri,
}
