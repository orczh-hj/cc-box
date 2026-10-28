/**
 * Tauri updater 在 latest.json 的 platforms 缺少当前运行架构时的错误特征。
 * 例：None of the fallback platforms ["darwin-x86_64-app", "darwin-x86_64"]
 * were found in the response 'platforms' object
 */
export function isMissingUpdatePlatform(message: string): boolean {
  return /None of the fallback platforms/i.test(message)
}
