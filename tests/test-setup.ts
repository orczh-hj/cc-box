import { randomFillSync } from 'crypto'

// 测试统一按 Tauri 环境运行（既有测试均以 mockIPC 为前提；src/api/env.ts 读取此标记）
;(globalThis as Record<string, unknown>).__CCBOX_TEST_FORCE_TAURI__ = true

// node 环境的测试（如构建产物安全测试，需用 esbuild）没有 window，跳过 DOM 桩
if (typeof window !== 'undefined') {
  Object.defineProperty(window, 'crypto', {
    value: {
      getRandomValues: (buffer: any) => randomFillSync(buffer),
    },
  })
}
