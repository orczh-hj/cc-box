/**
 * 测试环境说明：transport 按 Tauri 环境选择（统一管理单一事实来源）
 *
 * 既有 API/store 测试均以"Tauri 环境 + mockIPC"为前提编写；
 * jsdom 下 __TAURI_INTERNALS__ 不存在，transport 会误选浏览器远程实现。
 * 由 tests/test-setup.ts 设置 __CCBOX_TEST_FORCE_TAURI__，
 * src/api/env.ts 读取该标记强制 Tauri 分支。
 */
export const TEST_FORCE_TAURI = '__CCBOX_TEST_FORCE_TAURI__'
