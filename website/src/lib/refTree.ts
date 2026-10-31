// Claude Code 参考文档目录树：由虚拟模块下发的扁平清单构建分层结构。
// 清单条目 key 为文档相对路径（不含 .md），如 `getting-started/quickstart`。

export interface RefDocEntry {
  key: string
  title: string
}

export interface RefDoc {
  key: string
  title: string
}

export interface RefGroup {
  /** 子目录名（如 `core-concepts`）；根级文档归入 `__root__` 组 */
  name: string
  label: string
  docs: RefDoc[]
}

export interface RefCategory {
  /** 顶级目录名（如 `getting-started`） */
  name: string
  label: string
  groups: RefGroup[]
}

const CATEGORY_LABELS: Record<string, string> = {
  'getting-started': '入门',
  configuration: '配置',
  'build-with-claude-code': '用 Claude Code 构建',
  plugins: '插件',
  reference: '参考',
  resources: '资源',
  administration: '管理',
  'agent-sdk': 'Agent SDK',
  'whats-new': '新特性',
}

const GROUP_LABELS: Record<string, string> = {
  'claude-code-in-the-cloud': '云端 Claude Code',
  'claude-code-on-desktop': '桌面版',
  'code-review-and-pull-requests': '代码审查与 PR',
  'code-review': '代码审查',
  'core-concepts': '核心概念',
  'platforms-and-integrations': '平台与集成',
  'run-claude-in-your-own-ci': '在 CI 中运行',
  'security-scans': '安全扫描',
  'use-claude-code': '使用 Claude Code',
  environments: '环境',
  interface: '界面',
  'model-and-responses': '模型与回复',
  'permissions-and-sandboxing': '权限与沙箱',
  'self-hosted-environments': '自托管环境',
  settings: '设置',
  'agents-and-parallel-work': 'Agent 与并行工作',
  artifacts: 'Artifacts',
  automation: '自动化',
  guides: '指南',
  mcp: 'MCP',
  skills: 'Skills',
  troubleshooting: '故障排查',
  'create-plugins': '创建插件',
  glossary: '术语表',
  adoption: '推广采用',
  'claude-apps-gateway': 'Claude Apps Gateway',
  deployment: '部署',
  gateways: '网关',
  'other-gateways': '其他网关',
  'security-and-data': '安全与数据',
  'setup-and-access': '设置与访问',
  'usage-and-costs': '用量与成本',
  'use-plugins': '使用插件',
  build: '构建',
  mods: 'Mods',
  'run-a-marketplace': '运营市场',
  'manage-plugins-for-your-organization': '组织管理插件',
  'test-and-troubleshoot': '测试与排障',
  'build-agents': '构建代理',
  'customize-behavior': '定制行为',
  'extend-with-tools': '扩展工具',
  'input-and-output': '输入与输出',
  'sdk-references': 'SDK 参考',
  'control-and-observability': '控制与可观测',
}

/** kebab-case 目录名 → 可读标题（无中文映射时的兜底） */
export function fallbackLabel(name: string): string {
  return name
    .split('-')
    .map((w) => (w ? w[0].toUpperCase() + w.slice(1) : w))
    .join(' ')
}

/**
 * 按官方索引顺序重排扫描清单：order 为相对路径有序清单（claude-code-docs/order.json）。
 * 类与组的顺序由其中首次出现的路径决定；未登记的条目（本地新增）按原序追加在对应类末尾。
 */
export function applyDocOrder(entries: RefDocEntry[], order: string[]): RefDocEntry[] {
  const rank = new Map(order.map((k, i) => [k, i]))
  const catRank = new Map<string, number>()
  order.forEach((k, i) => {
    const cat = k.split('/')[0]
    if (!catRank.has(cat)) catRank.set(cat, i)
  })

  return [...entries].sort((a, b) => {
    const catA = a.key.split('/')[0]
    const catB = b.key.split('/')[0]
    const ca = catRank.get(catA) ?? Number.MAX_SAFE_INTEGER
    const cb = catRank.get(catB) ?? Number.MAX_SAFE_INTEGER
    if (ca !== cb) return ca - cb
    const ra = rank.get(a.key) ?? Number.MAX_SAFE_INTEGER
    const rb = rank.get(b.key) ?? Number.MAX_SAFE_INTEGER
    if (ra !== rb) return ra - rb
    return a.key.localeCompare(b.key)
  })
}

/**
 * 扁平清单 → 三层目录树（顶级类 → 子目录组 → 文档）。
 * 组内与类内顺序保持清单顺序；类/组缺失映射时用 fallback 标题。
 */
export function buildRefTree(entries: RefDocEntry[]): RefCategory[] {
  const byCategory = new Map<string, RefCategory>()

  for (const e of entries) {
    const parts = e.key.split('/')
    const catName = parts[0]
    if (!catName || parts.length < 2) continue

    let cat = byCategory.get(catName)
    if (!cat) {
      cat = {
        name: catName,
        label: CATEGORY_LABELS[catName] ?? fallbackLabel(catName),
        groups: [],
      }
      byCategory.set(catName, cat)
    }

    // 两段 = 类下根级文档；三段及以上 = 子目录组（多余层级并入组名，文档 key 保持原样）
    const groupName = parts.length === 2 ? '__root__' : parts.slice(1, -1).join('/')
    const docName = parts[parts.length - 1]
    let group = cat.groups.find((g) => g.name === groupName)
    if (!group) {
      group = {
        name: groupName,
        label: groupName === '__root__' ? '' : GROUP_LABELS[groupName] ?? fallbackLabel(groupName),
        docs: [],
      }
      cat.groups.push(group)
    }
    group.docs.push({ key: e.key, title: e.title || docName })
  }

  return [...byCategory.values()]
}
