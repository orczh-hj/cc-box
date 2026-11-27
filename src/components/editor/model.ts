/**
 * 扩展编辑器纯函数模型：表单 ↔ 配置转换、市场列表聚合。
 * 全部无副作用，供组件与 Vitest 共用（真值对拍见 tests/editor/model.test.ts）。
 */
import { load as yamlLoad } from 'js-yaml'
import type { SaveHookRequest } from '@/api/tauri'

// ==================== Markdown frontmatter ====================

/** YAML 块标量指示符：`>` 折叠 / `|` 字面量，可带 -/+ chomping */
export function isBlockScalarIndicator(v: string): boolean {
  return /^[>|][+-]?$/.test(v.trim())
}

/** 读取块标量体：收集后续缩进行（空行属于体内），折叠模式相邻行用空格连接。
 * 返回 [值, 下一行索引]，与后端 parse_block_scalar 语义一致 */
export function parseBlockScalar(
  lines: string[],
  start: number,
  indicator: string
): [string, number] {
  const body: string[] = []
  let i = start
  while (i < lines.length) {
    const line = lines[i]
    if (line.trim() === '') {
      body.push('')
      i++
      continue
    }
    if (/^[ \t]/.test(line)) {
      body.push(line)
      i++
      continue
    }
    break
  }
  const indent = body
    .filter(l => l.trim() !== '')
    .reduce((min, l) => Math.min(min, l.length - l.trimStart().length), Number.MAX_SAFE_INTEGER)
  const cut = indent === Number.MAX_SAFE_INTEGER ? 0 : indent
  let out = ''
  if (indicator.trim().startsWith('>')) {
    let prevBlank = false
    for (const l of body) {
      const t = l.slice(cut).trimEnd()
      if (t === '') {
        prevBlank = true
        continue
      }
      if (out !== '' && !prevBlank) out += ' '
      out += t
      prevBlank = false
    }
  } else {
    for (const l of body) out += `${l.slice(cut).trimEnd()}\n`
  }
  return [out.trim(), i]
}

/** 解析 frontmatter：返回原始字段与剥离 frontmatter 的正文。
 * 非法（无 --- 闭合）返回 null。description 支持双引号转义（与后端 yaml_quote 对应）
 * 与 `>-` / `|` 块标量多行写法（与后端 parse_block_scalar 一致） */
export function parseFrontmatter(source: string): {
  fields: Record<string, string>
  body: string
} | null {
  const lines = source.split(/\r?\n/)
  if ((lines[0] ?? '').trim() !== '---') return null
  const fields: Record<string, string> = {}
  let i = 1
  for (; i < lines.length; i++) {
    if (lines[i].trim() === '---') break
    const m = lines[i].match(/^([A-Za-z_][\w-]*):\s*(.*)$/)
    if (m) {
      let v = m[2].trim()
      if (isBlockScalarIndicator(v)) {
        const [block, next] = parseBlockScalar(lines, i + 1, v)
        fields[m[1]] = block
        i = next - 1
        continue
      }
      if (v.startsWith('"') && v.endsWith('"') && v.length >= 2) {
        // JSON 转义与 YAML 双引号兼容，反向解码
        try {
          v = JSON.parse(v)
        } catch {
          v = v.slice(1, -1)
        }
      } else if (v.startsWith("'") && v.endsWith("'") && v.length >= 2) {
        v = v.slice(1, -1)
      }
      fields[m[1]] = v
    }
  }
  if (i >= lines.length) return null // 未闭合
  // YAML 语法校验：结构对但语法错（tab 缩进、引号未闭合、坏映射等）同样判非法，
  // 否则失焦/保存时静默放行写出 Claude 读不了的文件
  const block = lines.slice(1, i).join('\n')
  try {
    const doc = yamlLoad(block)
    if (doc === null || typeof doc !== 'object' || Array.isArray(doc)) return null
  } catch {
    return null
  }
  const body = lines
    .slice(i + 1)
    .join('\n')
    .replace(/^\n+/, '')
    .trimEnd()
  return { fields, body }
}

// ==================== Skill / Agent 源码生成 ====================

/** frontmatter 已知字段之外的其它键（编辑时原样保留，避免静默丢配置） */
export type ExtraFields = Record<string, string>

/** 已知字段 → 源码时保持的键顺序（其余键按原顺序追加） */
const SKILL_KNOWN_KEYS = ['name', 'description']
const AGENT_KNOWN_KEYS = ['name', 'description', 'tools', 'model']

function appendExtras(lines: string[], extras: ExtraFields, known: string[]): void {
  for (const [k, v] of Object.entries(extras)) {
    if (known.includes(k)) continue
    lines.push(`${k}: ${JSON.stringify(v)}`)
  }
}

/** 收集解析结果里已知键之外的字段 */
export function pickExtras(fields: Record<string, string>, known: string[]): ExtraFields {
  const out: ExtraFields = {}
  for (const [k, v] of Object.entries(fields)) {
    if (!known.includes(k)) out[k] = v
  }
  return out
}

export function skillKnownKeys(): string[] {
  return SKILL_KNOWN_KEYS
}

export function agentKnownKeys(): string[] {
  return AGENT_KNOWN_KEYS
}

/** 表单 → SKILL.md 全文（与后端 build_skill_md 同构；extras 保留未知 frontmatter 键） */
export function buildSkillSource(
  form: { name: string; description: string; content: string },
  extras: ExtraFields = {}
): string {
  const lines = ['---', `name: ${form.name}`, `description: ${JSON.stringify(form.description)}`]
  appendExtras(lines, extras, SKILL_KNOWN_KEYS)
  lines.push('---', '', form.content.trimEnd())
  return lines.join('\n') + '\n'
}

/** 表单 → Agent .md 全文（extras 保留未知 frontmatter 键） */
export function buildAgentSource(
  form: { name: string; description: string; tools: string; model: string; content: string },
  extras: ExtraFields = {}
): string {
  const lines = ['---', `name: ${form.name}`, `description: ${JSON.stringify(form.description)}`]
  if (form.tools.trim()) lines.push(`tools: ${form.tools.trim()}`)
  if (form.model.trim()) lines.push(`model: ${form.model.trim()}`)
  appendExtras(lines, extras, AGENT_KNOWN_KEYS)
  lines.push('---', '', form.content.trimEnd())
  return lines.join('\n') + '\n'
}

// ==================== Hook ====================

/** 常用 hook 事件（与 Claude Code HOOK_EVENTS 对齐的常用子集，均经源码核实） */
export const HOOK_EVENTS = [
  'PreToolUse',
  'PostToolUse',
  'PostToolUseFailure',
  'UserPromptSubmit',
  'Notification',
  'SessionStart',
  'SessionEnd',
  'Stop',
  'SubagentStart',
  'SubagentStop',
  'PreCompact',
  'PostCompact',
  'PermissionRequest',
] as const

/** hook 类型（Claude Code 支持 command/http/mcp_tool/prompt/agent，见官方 hooks 文档；
 * mcp_tool 字段文档未稳定，表单不提供字段，走 rawHook 原样保存） */
export type HookType = 'command' | 'http' | 'prompt' | 'agent' | 'mcp_tool'

export const HOOK_TYPES: HookType[] = ['command', 'http', 'prompt', 'agent', 'mcp_tool']

export interface HookForm {
  scope: 'user' | 'project' | 'local'
  event: string
  type: HookType
  matcher: string
  /** type=command */
  command: string
  /** type=http */
  url: string
  headers: Array<{ key: string; value: string }>
  /** type=prompt/agent */
  prompt: string
  /** type=prompt 可选模型（如 haiku） */
  model: string
  timeout: string // 空串 = 不设置
  /** 未知/mcp_tool 类型：hook 对象原样保存（表单不展开字段） */
  rawHook?: Record<string, unknown>
}

export interface HookOriginal {
  event: string
  matcher?: string
  /** 被替换条目的定位键（列表 HookListItem.key） */
  key: string
}

/** 表单 → hook 对象（SaveHookRequest.hook） */
export function hookFormToObject(form: HookForm): Record<string, unknown> {
  if (form.type === 'mcp_tool' && form.rawHook) return form.rawHook
  const hook: Record<string, unknown> = { type: form.type }
  const timeout = form.timeout.trim()
  if (timeout) hook.timeout = Number(timeout)
  if (form.type === 'command') {
    hook.command = form.command
  } else if (form.type === 'http') {
    hook.url = form.url.trim()
    const headers: Record<string, string> = {}
    for (const r of form.headers) {
      if (r.key.trim()) headers[r.key.trim()] = r.value
    }
    if (Object.keys(headers).length) hook.headers = headers
  } else {
    hook.prompt = form.prompt
    if (form.type === 'prompt' && form.model.trim()) hook.model = form.model.trim()
  }
  return hook
}

/** 表单必填校验：返回错误文案 key 或 null */
export function hookFormError(form: HookForm): string | null {
  if (form.type === 'command' && !form.command.trim()) return 'editorHookCommandRequired'
  if (form.type === 'http' && !form.url.trim()) return 'editorHookUrlRequired'
  if ((form.type === 'prompt' || form.type === 'agent') && !form.prompt.trim()) return 'editorHookPromptRequired'
  return null
}

/** hookFormToObject 可能产出的键（其余视为未知自定义键，表单流转中原样保留） */
const HOOK_MANAGED_KEYS = ['type', 'timeout', 'command', 'url', 'headers', 'prompt', 'model']

/** 表单重建 hook 对象：受管字段以表单为准（类型切换不残留旧类型字段，
 * 如 command→http 后不留 command），未知自定义键（async 等）从旧对象带回 */
export function hookEntryRebuild(
  prevHook: Record<string, unknown> | undefined,
  form: HookForm
): Record<string, unknown> {
  const next = hookFormToObject(form)
  if (prevHook) {
    for (const [k, v] of Object.entries(prevHook)) {
      if (!HOOK_MANAGED_KEYS.includes(k) && !(k in next)) next[k] = v
    }
  }
  return next
}

/** hook 表单 → 保存请求；projectPath 为当前项目（user 作用域忽略），
 * original 为编辑定位（跨事件/跨作用域编辑由后端处理）；
 * prevHook 为原始条目的 hook 对象（编辑态由调用方从 lastEntry 取）——
 * 已知类型的未知自定义键（async/statusMessage 等）经 hookEntryRebuild 带回，
 * 不传维持旧行为（只产受管键）。mcp_tool 走 rawHook，与此参数无关。 */
export function buildHookPayload(
  form: HookForm,
  projectPath: string | undefined,
  original?: HookOriginal,
  originalScope?: string,
  prevHook?: Record<string, unknown>
): SaveHookRequest {
  return {
    scope: form.scope,
    event: form.event,
    matcher: form.matcher.trim() || undefined,
    hook: prevHook ? hookEntryRebuild(prevHook, form) : hookFormToObject(form),
    projectPath: form.scope === 'user' ? undefined : projectPath,
    original,
    originalScope,
  }
}

/**
 * matcher 条目对象核心解析。形态：{ matcher?, hooks: [<hook 对象>] }，只编辑
 * 首个 hook（多命令条目由面板禁止进入编辑器）。非法结构返回 null。
 */
function hookMatcherEntryToForm(
  obj: Record<string, unknown>,
  defaults: { scope: HookForm['scope']; event: string }
): HookForm | null {
  const hooks = obj.hooks
  if (!Array.isArray(hooks) || hooks.length === 0) return null
  const first = hooks[0] as Record<string, unknown> | undefined
  if (!first || typeof first !== 'object' || Array.isArray(first)) return null
  const rawType = typeof first.type === 'string' ? first.type : 'command'
  const type = (HOOK_TYPES as string[]).includes(rawType) ? (rawType as HookType) : 'mcp_tool'
  const timeout = typeof first.timeout === 'number' ? String(first.timeout) : ''
  const matcher = typeof obj.matcher === 'string' ? obj.matcher : ''
  const form: HookForm = {
    scope: defaults.scope,
    event: defaults.event,
    type,
    matcher,
    command: typeof first.command === 'string' ? first.command : '',
    url: typeof first.url === 'string' ? first.url : '',
    headers: recordToKvRows(first.headers),
    prompt: typeof first.prompt === 'string' ? first.prompt : '',
    model: typeof first.model === 'string' ? first.model : '',
    timeout,
  }
  // 未识别类型（mcp_tool 等）：hook 对象原样保留，保存时原样写回
  if (type === 'mcp_tool') form.rawHook = first
  return form
}

/** matcher 条目 JSON（裸片段）→ 表单。非法结构返回 null。 */
export function hookEntryToForm(
  json: string,
  defaults: { scope: HookForm['scope']; event: string }
): HookForm | null {
  let parsed: unknown
  try {
    parsed = JSON.parse(json)
  } catch {
    return null
  }
  if (!parsed || typeof parsed !== 'object' || Array.isArray(parsed)) return null
  return hookMatcherEntryToForm(parsed as Record<string, unknown>, defaults)
}

// ==================== JSON 模式（带键原生形态） ====================
//
// JSON 模式与原生文件里条目的真实形态 1:1 对应：
// - MCP   → {"<名称>": { …server 配置… }}（mcpServers 下该条目）
// - Hook  → {"<事件>": [ { matcher?, hooks: [...] } ]}（hooks 下该条目）
// 作用域决定落到哪个文件，不进 JSON。单键语义：解析唯一键回填名称/事件
// （JSON 改键 = 重命名 / 跨事件移动），多键、多条数组报错拒绝。

/** MCP JSON 解析失败原因（组件映射 i18n） */
export type McpJsonFailReason = 'invalid' | 'notObject' | 'multiKey'

export type McpJsonParseResult =
  | { ok: true; name: string; config: Record<string, unknown> }
  | { ok: false; reason: McpJsonFailReason }

/** 表单 → MCP JSON 文本：{"<名称>": {…配置…}} */
export function mcpEntryToJson(name: string, config: Record<string, unknown>): string {
  return JSON.stringify({ [name.trim()]: config }, null, 2)
}

/** MCP JSON 文本 → 名称 + 配置。单键对象，其余结构拒绝。 */
export function mcpEntryFromJson(text: string): McpJsonParseResult {
  let parsed: unknown
  try {
    parsed = JSON.parse(text)
  } catch {
    return { ok: false, reason: 'invalid' }
  }
  if (!parsed || typeof parsed !== 'object' || Array.isArray(parsed)) {
    return { ok: false, reason: 'notObject' }
  }
  const obj = parsed as Record<string, unknown>
  const keys = Object.keys(obj)
  if (keys.length !== 1) return { ok: false, reason: 'multiKey' }
  const config = obj[keys[0]]
  if (!config || typeof config !== 'object' || Array.isArray(config)) {
    return { ok: false, reason: 'notObject' }
  }
  return { ok: true, name: keys[0], config: config as Record<string, unknown> }
}

/** Hook JSON 解析失败原因（组件映射 i18n） */
export type HookJsonFailReason =
  | 'invalid'
  | 'notObject'
  | 'multiKey'
  | 'notArray'
  | 'multiEntry'
  | 'badEntry'

export type HookJsonParseResult =
  | { ok: true; event: string; entry: Record<string, unknown>; form: HookForm }
  | { ok: false; reason: HookJsonFailReason }

/** 表单 → Hook JSON 文本：{"<事件>": [matcher 条目]} */
export function hookEntryToJson(event: string, entry: Record<string, unknown>): string {
  return JSON.stringify({ [event.trim()]: [entry] }, null, 2)
}

/** Hook JSON 文本 → 事件 + matcher 条目 + 表单。事件取自唯一键。 */
export function hookEntryFromJson(
  text: string,
  defaults: { scope: HookForm['scope'] }
): HookJsonParseResult {
  let parsed: unknown
  try {
    parsed = JSON.parse(text)
  } catch {
    return { ok: false, reason: 'invalid' }
  }
  if (!parsed || typeof parsed !== 'object' || Array.isArray(parsed)) {
    return { ok: false, reason: 'notObject' }
  }
  const obj = parsed as Record<string, unknown>
  const keys = Object.keys(obj)
  if (keys.length !== 1) return { ok: false, reason: 'multiKey' }
  const event = keys[0]
  const list = obj[event]
  if (!Array.isArray(list)) return { ok: false, reason: 'notArray' }
  if (list.length !== 1) return { ok: false, reason: 'multiEntry' }
  const entry = list[0]
  if (!entry || typeof entry !== 'object' || Array.isArray(entry)) {
    return { ok: false, reason: 'badEntry' }
  }
  const form = hookMatcherEntryToForm(entry as Record<string, unknown>, {
    scope: defaults.scope,
    event,
  })
  if (!form) return { ok: false, reason: 'badEntry' }
  return { ok: true, event, entry: entry as Record<string, unknown>, form }
}

// ==================== MCP ====================

export interface McpForm {
  scope: 'user' | 'project' | 'local'
  name: string
  type: 'stdio' | 'http' | 'sse'
  command: string
  args: string // 空格分隔（与 CLI 习惯一致）
  url: string
  env: Array<{ key: string; value: string }>
  headers: Array<{ key: string; value: string }>
}

/** 表单 → server 配置对象；字段非法返回 null（校验信息由组件层出） */
export function mcpFormToConfig(form: McpForm): Record<string, unknown> | null {
  if (!form.name.trim()) return null
  if (form.type === 'stdio') {
    if (!form.command.trim()) return null
    const config: Record<string, unknown> = { type: 'stdio', command: form.command.trim() }
    const args = argsSplit(form.args)
    if (args.length) config.args = args
    const env = kvRowsToRecord(form.env)
    if (env) config.env = env
    return config
  }
  // ${VAR} 引用扩展后才是真实 URL（可能以任意 scheme 开头），与字面 https? 并列放行
  if (!/^https?:\/\//.test(form.url.trim()) && !form.url.trim().startsWith('${')) return null
  const config: Record<string, unknown> = { type: form.type, url: form.url.trim() }
  const headers = kvRowsToRecord(form.headers)
  if (headers) config.headers = headers
  // http/sse 也允许 env（Claude 对 command/args/env/url/headers 统一做 ${VAR} 扩展）
  const env = kvRowsToRecord(form.env)
  if (env) config.env = env
  return config
}

/** server 配置对象 → 表单（识别不了的形态返回 null：plugin 来源等） */
export function mcpConfigToForm(
  config: Record<string, unknown> | undefined,
  base: Pick<McpForm, 'scope' | 'name'>
): McpForm | null {
  if (!config) return null
  const type = config.type === 'http' || config.type === 'sse' ? config.type : config.command ? 'stdio' : null
  if (!type) return null
  const form: McpForm = {
    ...base,
    type,
    command: typeof config.command === 'string' ? config.command : '',
    args: Array.isArray(config.args) ? argsJoin(config.args.map(String)) : '',
    url: typeof config.url === 'string' ? config.url : '',
    env: recordToKvRows(config.env),
    headers: recordToKvRows(config.headers),
  }
  return form
}

/**
 * args 数组 ↔ 空格分隔文本（无损往返）：含空格/引号的参数（如
 * `C:/Program Files (x86)/...`）在 join 时加双引号，split 时引号感知解析。
 * Windows 路径带空格是常态，裸空白 split 会把单个参数拆成多个。
 */
export function argsJoin(args: string[]): string {
  return args
    .map(a => (/\s|"|'/.test(a) ? JSON.stringify(a) : a))
    .join(' ')
}

export function argsSplit(text: string): string[] {
  const out: string[] = []
  let cur = ''
  let quote: '"' | "'" | null = null
  let hasContent = false
  for (let i = 0; i < text.length; i++) {
    const ch = text[i]
    if (quote) {
      if (ch === quote) {
        quote = null
      } else if (ch === '\\' && quote === '"' && i + 1 < text.length) {
        // JSON 风格转义（argsJoin 用 JSON.stringify 包裹）
        cur += text[i + 1]
        i++
      } else {
        cur += ch
      }
    } else if (ch === '"' || ch === "'") {
      quote = ch
      hasContent = true
    } else if (/\s/.test(ch)) {
      if (hasContent || cur) {
        out.push(cur)
        cur = ''
        hasContent = false
      }
    } else {
      cur += ch
    }
  }
  if (hasContent || cur) out.push(cur)
  return out
}

function kvRowsToRecord(rows: Array<{ key: string; value: string }>): Record<string, string> | undefined {
  const out: Record<string, string> = {}
  for (const r of rows) {
    if (r.key.trim()) out[r.key.trim()] = r.value
  }
  return Object.keys(out).length ? out : undefined
}

function recordToKvRows(record: unknown): Array<{ key: string; value: string }> {
  if (!record || typeof record !== 'object' || Array.isArray(record)) return []
  return Object.entries(record as Record<string, unknown>).map(([key, value]) => ({
    key,
    value: typeof value === 'string' ? value : JSON.stringify(value),
  }))
}

// ==================== Plugin 市场聚合 ====================

/** CC-Box 自带精选市场：列表中置顶，并作为默认选中的市场 */
export const CCBOX_MARKET = 'ccbox-market'

/** 市场列表顺序：ccbox-market 置顶，其余保持传入顺序（后端按名称排序，stable sort 保留） */
export function orderMarkets<T extends { name: string }>(markets: T[]): T[] {
  return [...markets].sort((a, b) => {
    const pa = a.name.toLowerCase() === CCBOX_MARKET ? 0 : 1
    const pb = b.name.toLowerCase() === CCBOX_MARKET ? 0 : 1
    return pa - pb
  })
}

/** 精选图标是否为图片 URL（否则按 emoji/短文本渲染） */
export function isIconUrl(icon: string): boolean {
  return /^https?:\/\//i.test(icon.trim())
}

/** 市场插件过滤：名称/描述/分类/标签不区分大小写包含 */
export function filterPlugins<T extends { name: string; description?: string; category?: string; tags: string[] }>(
  plugins: T[],
  query: string
): T[] {
  const q = query.trim().toLowerCase()
  if (!q) return plugins
  return plugins.filter(p =>
    p.name.toLowerCase().includes(q) ||
    (p.description ?? '').toLowerCase().includes(q) ||
    (p.category ?? '').toLowerCase().includes(q) ||
    p.tags.some(t => t.toLowerCase().includes(q))
  )
}
