/**
 * 会话显示名规则（名称唯一源 = transcript JSONL，与 Claude Code CLI 同路径）：
 * Tab 名 = 官方标题 session_title（/rename、--name）> 首条 prompt 自动名 > 占位；
 * 历史/近期/搜索名一律由 Rust extract_session_name 从 JSONL 提取。
 * 本模块只放纯函数与稳定哨兵——**不 import i18n**（静态 i18n 依赖会打破
 * "只 mock vue-i18n 部分 API"的测试收集，见 desktop-outdated.ts 踩坑记录）。
 */

/** 新会话占位名（稳定哨兵：自动命名/名称重置的判定基准，不随 locale 变化） */
export const SESSION_NAME_PLACEHOLDER = 'New Session'

/** 显示截断上限（字符数），与 Rust extract_session_name / CLI extractFirstPromptFromHead 一致（200） */
export const SESSION_NAME_MAX = 200

/** 显示截断：超长加 '…'（与 Rust 侧统一，多字节安全） */
export function truncateDisplayName(name: string, max: number = SESSION_NAME_MAX): string {
  const chars = [...name]
  return chars.length > max ? chars.slice(0, max).join('') + '…' : name
}

/**
 * 是否占位类名（可被官方标题/prompt 自动命名覆盖）：
 * 空名、创建占位、sessionId 前 8 位（历史会话恢复初始名）
 */
export function isPlaceholderSessionName(name: string, sessionId?: string | null): boolean {
  if (!name || name === SESSION_NAME_PLACEHOLDER) return true
  if (sessionId && name === sessionId.slice(0, 8)) return true
  return false
}
