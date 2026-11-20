/**
 * keydown 按键匹配（兼容中文 IME 的 'Process' 形态）。
 *
 * 中文输入法开启时，WebView2/Chromium 对组合键的 keydown 可能报
 * `event.key === 'Process'`（IME 接管按键），字面量 `key` 判定全部失配——
 * 实测中文 IME 下终端 Ctrl+C 变成 SIGINT（出现 exit 提示）、Ctrl+Shift+C
 * 复制与 Ctrl+V 粘贴静默失效，切英文输入法即恢复。Ctrl/⌘ 等修饰键组合
 * 不是 IME 组合输入的目标键，此时以物理键 `event.code` 兜底判定。
 */
export function isKeyWithIme(
  ev: Pick<KeyboardEvent, 'key' | 'code'>,
  code: string,
  ...keys: string[]
): boolean {
  return keys.includes(ev.key) || (ev.key === 'Process' && ev.code === code)
}
