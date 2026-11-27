import { describe, expect, it } from 'vitest'
import {
  argsJoin,
  argsSplit,
  buildHookPayload,
  filterPlugins,
  hookEntryFromJson,
  hookEntryRebuild,
  hookEntryToForm,
  hookEntryToJson,
  isIconUrl,
  mcpConfigToForm,
  mcpEntryFromJson,
  mcpEntryToJson,
  orderMarkets,
  mcpFormToConfig,
  parseFrontmatter,
  type HookForm,
} from '@/components/editor/model'

// ==================== Markdown frontmatter ====================

describe('Frontmatter_Parse', () => {
  it('Parse_001_普通字段与引号', () => {
    const src = '---\nname: demo\ndescription: "a b"\n---\n正文'
    const fm = parseFrontmatter(src)!
    expect(fm.fields.name).toBe('demo')
    expect(fm.fields.description).toBe('a b')
    expect(fm.body).toBe('正文')
  })

  it('Parse_002_折叠块标量', () => {
    // YAML `>-`：多行折叠为单行，行间用空格连接
    const src = [
      '---',
      'name: nature',
      'description: >-',
      '  Search academic papers across',
      '  multiple sources and summarize.',
      'model: sonnet',
      '---',
      'body',
    ].join('\n')
    const fm = parseFrontmatter(src)!
    expect(fm.fields.description).toBe(
      'Search academic papers across multiple sources and summarize.'
    )
    // 块标量不吞掉后续同级键
    expect(fm.fields.model).toBe('sonnet')
    expect(fm.body).toBe('body')
  })

  it('Parse_003_字面量块标量保留换行', () => {
    const src = ['---', 'description: |', '  第一行', '  第二行', 'name: x', '---', 'b'].join('\n')
    const fm = parseFrontmatter(src)!
    expect(fm.fields.description).toBe('第一行\n第二行')
    expect(fm.fields.name).toBe('x')
  })

  it('Parse_004_块标量内出现分隔线不提前闭合', () => {
    const src = ['---', 'description: >-', '  a --- b', '---', 'body'].join('\n')
    const fm = parseFrontmatter(src)!
    expect(fm.fields.description).toBe('a --- b')
    expect(fm.body).toBe('body')
  })

  // YAML 语法错误必须判非法（回归：blur 无提示——tab 缩进是 YAML 非法语法）
  it('Parse_005_tab缩进判非法', () => {
    const src = ['---', 'name: demo', '\tdescription: bad', '---', 'body'].join('\n')
    expect(parseFrontmatter(src)).toBeNull()
  })

  // 引号未闭合是 YAML 非法语法
  it('Parse_006_引号未闭合判非法', () => {
    const src = '---\nname: demo\ndescription: "unterminated\n---\nbody'
    expect(parseFrontmatter(src)).toBeNull()
  })

  // 坏映射（值里再冒号）是 YAML 非法语法
  it('Parse_007_坏映射判非法', () => {
    const src = '---\nname: a: b\n---\nbody'
    expect(parseFrontmatter(src)).toBeNull()
  })
})

// ==================== MCP 表单 ↔ 配置 ====================

/** stdio 表单 → 配置：type/command 必出，args 按空白拆分且过滤空段，env 非空才输出 */
describe('McpForm_ToConfig_Stdio', () => {
  it('FormToConfig_001_完整字段', () => {
    const config = mcpFormToConfig({
      scope: 'user',
      name: 'my-db',
      type: 'stdio',
      command: 'npx',
      args: '  -y   @x/server  ', url: '',
      env: [{ key: 'K', value: 'V' }, { key: '  ', value: 'ignored' }],
      headers: [],
    })
    // 独立手写真值
    expect(config).toEqual({
      type: 'stdio',
      command: 'npx',
      args: ['-y', '@x/server'],
      env: { K: 'V' },
    })
  })

  it('FormToConfig_002_缺命令返回Null', () => {
    expect(
      mcpFormToConfig({ scope: 'user', name: 'x', type: 'stdio', command: '  ', args: '', url: '', env: [], headers: [] })
    ).toBeNull()
  })

  it('FormToConfig_003_http校验Url与Headers', () => {
    expect(
      mcpFormToConfig({ scope: 'user', name: 's', type: 'http', command: '', args: '', url: 'ftp://bad', env: [], headers: [] })
    ).toBeNull()
    expect(
      mcpFormToConfig({
        scope: 'user', name: 's', type: 'sse', command: '', args: '',
        url: ' https://mcp.sentry.dev/mcp ',
        env: [], headers: [{ key: 'Authorization', value: 'Bearer t' }],
      })
    ).toEqual({ type: 'sse', url: 'https://mcp.sentry.dev/mcp', headers: { Authorization: 'Bearer t' } })
  })

  it('FormToConfig_005_http支持env与变量引用', () => {
    // Claude Code 对 http/sse 也做 ${VAR} 扩展；env 原样保留不展开
    expect(
      mcpFormToConfig({
        scope: 'user', name: 'api', type: 'http', command: '', args: '',
        url: '${API_BASE_URL:-https://api.example.com}/mcp',
        env: [{ key: 'TRACE', value: '${TRACE_ID}' }],
        headers: [],
      })
    ).toEqual({
      type: 'http',
      url: '${API_BASE_URL:-https://api.example.com}/mcp',
      env: { TRACE: '${TRACE_ID}' },
    })
    // 空 env 不输出
    expect(
      mcpFormToConfig({
        scope: 'user', name: 'api', type: 'http', command: '', args: '',
        url: 'https://x/mcp', env: [], headers: [],
      })
    ).toEqual({ type: 'http', url: 'https://x/mcp' })
  })

  it('FormToConfig_004_缺名称返回Null', () => {
    expect(
      mcpFormToConfig({ scope: 'user', name: '  ', type: 'stdio', command: 'x', args: '', url: '', env: [], headers: [] })
    ).toBeNull()
  })

  it('FormToConfig_006_带空格路径参数不被拆断', () => {
    // 回归：Program Files 路径曾按空白拆成三个参数
    const config = mcpFormToConfig({
      scope: 'user', name: 'pw', type: 'stdio', command: 'cmd',
      args: '/c npx @playwright/mcp@latest --executable-path "C:/Program Files (x86)/Microsoft/Edge/Application/msedge.exe"',
      url: '', env: [], headers: [],
    })
    expect(config).toEqual({
      type: 'stdio',
      command: 'cmd',
      args: [
        '/c', 'npx', '@playwright/mcp@latest', '--executable-path',
        'C:/Program Files (x86)/Microsoft/Edge/Application/msedge.exe',
      ],
    })
  })

  it('ArgsJoin_含空格参数加引号且回读无损', () => {
    // join：含空格/引号的参数加双引号
    expect(argsJoin(['-c', 'C:/Program Files/x.exe', "it's"])).toBe('-c "C:/Program Files/x.exe" "it\'s"')
    expect(argsJoin([])).toBe('')
    // split：引号内空白保留；未闭合引号容错
    expect(argsSplit('a "b c" d')).toEqual(['a', 'b c', 'd'])
    expect(argsSplit('"未闭合')).toEqual(['未闭合'])
    expect(argsSplit('  ')).toEqual([])
    // 往返无损
    const args = ['/c', 'x y z', "it's", 'plain']
    expect(argsSplit(argsJoin(args))).toEqual(args)
  })
})

/** 配置 → 表单：往返转换保持字段；无法识别形态返回 null */
describe('McpConfig_ToForm_往返', () => {
  it('ConfigToForm_001_stdio往返', () => {
    const form = mcpConfigToForm(
      { type: 'stdio', command: 'npx', args: ['-y', 'srv'], env: { A: '1' } },
      { scope: 'user', name: 's' }
    )
    expect(form).toEqual({
      scope: 'user', name: 's', type: 'stdio', command: 'npx', args: '-y srv',
      url: '', env: [{ key: 'A', value: '1' }], headers: [],
    })
    // 再正向转换：语义等价
    expect(mcpFormToConfig(form!)).toEqual({ type: 'stdio', command: 'npx', args: ['-y', 'srv'], env: { A: '1' } })
  })

  it('ConfigToForm_002_无type无command返回Null', () => {
    expect(mcpConfigToForm({ url: 'https://x' }, { scope: 'user', name: 's' })).toBeNull()
  })
})

// ==================== Hook ====================

/** hook 表单 → 保存请求：matcher 空白归一为 undefined、hook 对象按类型构造、user 忽略项目路径 */
describe('Hook_Payload', () => {
  it('BuildHookPayload_001_新建user_command类型', () => {
    const payload = buildHookPayload(
      {
        scope: 'user', event: 'PostToolUse', type: 'command', matcher: '  ',
        command: 'fmt', url: '', headers: [], prompt: '', model: '', timeout: '  ',
      },
      'D:/proj'
    )
    // 独立手写真值
    expect(payload).toEqual({
      scope: 'user',
      event: 'PostToolUse',
      matcher: undefined,
      hook: { type: 'command', command: 'fmt' },
      projectPath: undefined,
      original: undefined,
    })
  })

  it('BuildHookPayload_002_项目作用域带路径与timeout', () => {
    const payload = buildHookPayload(
      {
        scope: 'project', event: 'Stop', type: 'command', matcher: 'Edit',
        command: 'c', url: '', headers: [], prompt: '', model: '', timeout: '30',
      },
      'D:/proj'
    )
    expect(payload.projectPath).toBe('D:/proj')
    expect(payload.hook).toEqual({ type: 'command', command: 'c', timeout: 30 })
    expect(payload.matcher).toBe('Edit')
  })

  it('BuildHookPayload_003_http与prompt类型对象', () => {
    const http = buildHookPayload(
      {
        scope: 'user', event: 'PostToolUse', type: 'http', matcher: '',
        command: '', url: ' http://x/hook ', headers: [{ key: 'Authorization', value: 'Bearer t' }, { key: ' ', value: 'x' }],
        prompt: '', model: '', timeout: '',
      },
      undefined
    )
    expect(http.hook).toEqual({
      type: 'http',
      url: 'http://x/hook',
      headers: { Authorization: 'Bearer t' },
    })
    const prompt = buildHookPayload(
      {
        scope: 'user', event: 'Stop', type: 'prompt', matcher: '',
        command: '', url: '', headers: [], prompt: '检查任务', model: 'haiku', timeout: '60',
      },
      undefined
    )
    expect(prompt.hook).toEqual({ type: 'prompt', prompt: '检查任务', model: 'haiku', timeout: 60 })
  })

  // 2026-11-27 审核：保存链此前走 hookFormToObject（只产受管键），JSON 模式
  // 编辑出的未知自定义键（async 等）在保存时静默丢失——prevHook 带回修复
  it('BuildHookPayload_004_prevHook保留未知自定义键', () => {
    const form = {
      scope: 'user', event: 'PostToolUse', type: 'command', matcher: 'Bash',
      command: 'new-cmd', url: '', headers: [], prompt: '', model: '', timeout: '',
    } as Parameters<typeof buildHookPayload>[0]
    // 不传 prevHook：维持旧行为（只产受管键）
    expect(buildHookPayload(form, undefined).hook).toEqual({ type: 'command', command: 'new-cmd' })
    // 传 prevHook：受管字段以表单为准，未知键从旧对象带回
    const payload = buildHookPayload(form, undefined, undefined, undefined, {
      type: 'command', command: 'old-cmd', timeout: 5, async: true, statusMessage: '运行中',
    })
    expect(payload.hook).toEqual({
      type: 'command', command: 'new-cmd', async: true, statusMessage: '运行中',
    })
    // 类型切换：旧类型字段（command）不残留
    const switched = buildHookPayload(
      { ...form, type: 'prompt', prompt: '检查', command: 'old' },
      undefined, undefined, undefined,
      { type: 'command', command: 'old-cmd', async: true }
    )
    expect(switched.hook).toEqual({ type: 'prompt', prompt: '检查', async: true })
  })
})

/** matcher 条目 JSON → 表单：command/http/prompt/agent 按类型取字段；未知类型原样保留；非法结构拒绝 */
describe('HookEntry_ToForm', () => {
  it('EntryToForm_001_command类型', () => {
    const form = hookEntryToForm(
      JSON.stringify({ matcher: 'Write', hooks: [{ type: 'command', command: 'fmt', timeout: 30 }] }),
      { scope: 'user', event: 'PreToolUse' }
    )
    expect(form).toEqual({
      scope: 'user', event: 'PreToolUse', type: 'command', matcher: 'Write',
      command: 'fmt', url: '', headers: [], prompt: '', model: '', timeout: '30',
    })
  })

  it('EntryToForm_002_http与prompt类型解析', () => {
    const http = hookEntryToForm(
      JSON.stringify({ hooks: [{ type: 'http', url: 'http://x/h', headers: { A: 'b' }, timeout: 5 }] }),
      { scope: 'user', event: 'Stop' }
    )
    expect(http?.type).toBe('http')
    expect(http?.url).toBe('http://x/h')
    expect(http?.headers).toEqual([{ key: 'A', value: 'b' }])
    expect(http?.timeout).toBe('5')

    const agent = hookEntryToForm(
      JSON.stringify({ hooks: [{ type: 'agent', prompt: '验证测试' }] }),
      { scope: 'user', event: 'Stop' }
    )
    expect(agent?.type).toBe('agent')
    expect(agent?.prompt).toBe('验证测试')
  })

  it('EntryToForm_003_未知类型原样保留', () => {
    const raw = { type: 'mcp_tool', server: 'x', tool: 'y' }
    const form = hookEntryToForm(JSON.stringify({ hooks: [raw] }), { scope: 'user', event: 'Stop' })
    expect(form?.type).toBe('mcp_tool')
    expect(form?.rawHook).toEqual(raw)
  })

  it('EntryToForm_004_非法拒绝', () => {
    expect(hookEntryToForm('not json', { scope: 'user', event: 'Stop' })).toBeNull()
    expect(hookEntryToForm('[]', { scope: 'user', event: 'Stop' })).toBeNull()
    expect(hookEntryToForm('{"matcher":"x"}', { scope: 'user', event: 'Stop' })).toBeNull()
    expect(hookEntryToForm('{"hooks":[]}', { scope: 'user', event: 'Stop' })).toBeNull()
    expect(hookEntryToForm('{"hooks":["x"]}', { scope: 'user', event: 'Stop' })).toBeNull()
  })
})

// ==================== JSON 模式（带键原生形态） ====================

/** MCP 带键 JSON：{"<名称>": 配置}——与 mcpServers 下条目形态 1:1，
 * 解析唯一键回填名称，JSON 改键 = 重命名 */
describe('McpJson_Keyed', () => {
  it('ToJson_001_名称为键包裹配置', () => {
    const json = mcpEntryToJson('srv', { type: 'stdio', command: 'npx' })
    // 独立真值：解析回手写对象
    expect(JSON.parse(json)).toEqual({ srv: { type: 'stdio', command: 'npx' } })
  })

  it('FromJson_002_单键解析与改键重命名', () => {
    const r = mcpEntryFromJson('{\n  "old-name": { "type": "http", "url": "https://x/mcp" }\n}')
    expect(r).toEqual({ ok: true, name: 'old-name', config: { type: 'http', url: 'https://x/mcp' } })
    // JSON 改键 = 重命名：名称来自键
    const renamed = mcpEntryFromJson('{ "new-name": { "type": "http", "url": "https://x/mcp" } }')
    expect(renamed).toEqual({ ok: true, name: 'new-name', config: { type: 'http', url: 'https://x/mcp' } })
  })

  it('FromJson_003_非法结构拒绝', () => {
    expect(mcpEntryFromJson('not json')).toEqual({ ok: false, reason: 'invalid' })
    expect(mcpEntryFromJson('[]')).toEqual({ ok: false, reason: 'notObject' })
    // 值不是对象（名称键下必须是配置对象）
    expect(mcpEntryFromJson('{"a": 1}')).toEqual({ ok: false, reason: 'notObject' })
    // 0 个或 2 个键
    expect(mcpEntryFromJson('{}')).toEqual({ ok: false, reason: 'multiKey' })
    expect(mcpEntryFromJson('{"a": {}, "b": {}}')).toEqual({ ok: false, reason: 'multiKey' })
  })
})

/** Hook 带键 JSON：{"<事件>": [matcher 条目]}——与 settings.json hooks 下条目形态
 * 1:1，事件取自唯一键，JSON 改键 = 跨事件移动 */
describe('HookJson_Keyed', () => {
  const entry = { matcher: 'Bash', hooks: [{ type: 'command', command: 'fmt' }] }

  it('ToJson_001_事件为键包裹条目数组', () => {
    const json = hookEntryToJson('PreToolUse', entry)
    expect(JSON.parse(json)).toEqual({ PreToolUse: [entry] })
  })

  it('FromJson_002_键为事件且回填表单', () => {
    const r = hookEntryFromJson(hookEntryToJson('PreToolUse', entry), { scope: 'user' })
    expect(r.ok).toBe(true)
    if (r.ok) {
      expect(r.event).toBe('PreToolUse')
      expect(r.entry).toEqual(entry)
      expect(r.form).toEqual({
        scope: 'user', event: 'PreToolUse', type: 'command', matcher: 'Bash',
        command: 'fmt', url: '', headers: [], prompt: '', model: '', timeout: '',
      })
    }
    // 改键 = 跨事件移动：事件跟随键；scope 取 defaults
    const moved = hookEntryFromJson(
      '{"PostToolUse": [{"hooks": [{"type": "http", "url": "http://x/h"}]}]}',
      { scope: 'project' }
    )
    expect(moved.ok).toBe(true)
    if (moved.ok) {
      expect(moved.event).toBe('PostToolUse')
      expect(moved.form.event).toBe('PostToolUse')
      expect(moved.form.scope).toBe('project')
      expect(moved.form.type).toBe('http')
      expect(moved.form.url).toBe('http://x/h')
    }
  })

  it('FromJson_003_非法结构拒绝', () => {
    expect(hookEntryFromJson('not json', { scope: 'user' })).toEqual({ ok: false, reason: 'invalid' })
    expect(hookEntryFromJson('[]', { scope: 'user' })).toEqual({ ok: false, reason: 'notObject' })
    // 0 个或 2 个事件键
    expect(hookEntryFromJson('{}', { scope: 'user' })).toEqual({ ok: false, reason: 'multiKey' })
    expect(hookEntryFromJson('{"A": {}, "B": []}', { scope: 'user' })).toEqual({ ok: false, reason: 'multiKey' })
    // 值不是数组
    expect(hookEntryFromJson('{"A": {}}', { scope: 'user' })).toEqual({ ok: false, reason: 'notArray' })
    // 数组只能有一条 matcher 条目（0 条 / 2 条）
    expect(hookEntryFromJson('{"A": []}', { scope: 'user' })).toEqual({ ok: false, reason: 'multiEntry' })
    expect(hookEntryFromJson('{"A": [1, 2]}', { scope: 'user' })).toEqual({ ok: false, reason: 'multiEntry' })
    // 条目不是对象 / hooks 缺失或为空
    expect(hookEntryFromJson('{"A": ["x"]}', { scope: 'user' })).toEqual({ ok: false, reason: 'badEntry' })
    expect(hookEntryFromJson('{"A": [{"hooks": []}]}', { scope: 'user' })).toEqual({ ok: false, reason: 'badEntry' })
  })
})

/** 表单重建 hook 对象：受管字段以表单为准，未知自定义键保留 */
describe('HookEntry_Rebuild', () => {
  // 回归：类型切换曾把旧类型字段残留进条目（command→http 后 hooks[0] 仍有 command）
  it('Rebuild_001_类型切换不残留旧字段', () => {
    const base: HookForm = {
      scope: 'user', event: 'PreToolUse', type: 'command', matcher: '',
      command: 'old-cmd', url: '', headers: [], prompt: '', model: '', timeout: '',
    }
    const next = hookEntryRebuild(
      { type: 'command', command: 'old-cmd', timeout: 30 },
      { ...base, type: 'http', url: 'http://x/hook' }
    )
    // 独立手写真值：command/timeout 随类型切换丢弃，不残留
    expect(next).toEqual({ type: 'http', url: 'http://x/hook' })
  })

  it('Rebuild_002_未知自定义键跨类型保留', () => {
    const base: HookForm = {
      scope: 'user', event: 'Stop', type: 'prompt', matcher: '',
      command: '', url: '', headers: [], prompt: '', model: '', timeout: '',
    }
    const next = hookEntryRebuild(
      { type: 'command', command: 'old', async: true, statusMessage: '运行中' },
      { ...base, prompt: '检查' }
    )
    expect(next).toEqual({ type: 'prompt', prompt: '检查', async: true, statusMessage: '运行中' })
  })
})

// ==================== Plugin 市场 ====================

/** 市场过滤：名称/描述/分类/标签不区分大小写包含 */
describe('Market_FilterPlugins', () => {
  const plugins = [
    { name: 'paper-tool', description: '论文检索', category: 'writing', tags: ['research'] },
    { name: 'git-helper', description: 'Git helpers for commits', category: 'dev', tags: ['git'] },
  ]

  it('Filter_001_空查询原样返回', () => {
    expect(filterPlugins(plugins, '  ')).toEqual(plugins)
  })

  it('Filter_002_按名称与标签命中', () => {
    expect(filterPlugins(plugins, 'PAPER')).toEqual([plugins[0]])
    expect(filterPlugins(plugins, 'git')).toEqual([plugins[1]])
    expect(filterPlugins(plugins, '不存在')).toEqual([])
  })

  it('Filter_003_按分类命中', () => {
    expect(filterPlugins(plugins, 'writing')).toEqual([plugins[0]])
  })
})

/** 市场栏工具：ccbox-market 置顶与图标渲染 */
describe('Market_Rail', () => {
  it('Order_001_ccboxMarket置顶且保持其余顺序', () => {
    // 后端已按名称排序，这里只把 ccbox-market 提到最前
    const out = orderMarkets([
      { name: 'claude-plugins-official' },
      { name: 'ccbox-market' },
      { name: 'orczh' },
    ])
    expect(out.map(m => m.name)).toEqual(['ccbox-market', 'claude-plugins-official', 'orczh'])
    // 大小写不敏感
    expect(orderMarkets([{ name: 'CCBox-Market' }, { name: 'a' }])[0].name).toBe('CCBox-Market')
    // 不修改入参
    const input = [{ name: 'a' }, { name: 'ccbox-market' }]
    orderMarkets(input)
    expect(input[0].name).toBe('a')
  })

  it('Icon_001_图片URL与emoji区分', () => {
    // 独立真值：只有 http(s) 开头按图片渲染，其余（emoji/短文本）按文本
    expect(isIconUrl('https://example.com/icon.png')).toBe(true)
    expect(isIconUrl(' HTTP://example.com/i.svg ')).toBe(true)
    expect(isIconUrl('🧪')).toBe(false)
    expect(isIconUrl('AI')).toBe(false)
    expect(isIconUrl('data:image/png;base64,xxx')).toBe(false)
  })
})
