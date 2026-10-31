// Config 相关类型定义

export interface ConfigSource {
  type: 'project' | 'user' | 'local' | 'managed' | 'builtin' | 'plugin'
  label: string
  path?: string
}

export interface BasicConfigItem {
  model?: string
  theme?: string
  editorMode?: string
  autoConnectIde?: boolean
  permissions?: {
    allow?: string[]
    deny?: string[]
  }
  env?: Record<string, string>
  source: ConfigSource
}

export interface McpServerItem {
  name: string
  type?: 'stdio' | 'http' | 'sse'
  command?: string
  args?: string[]
  env?: Record<string, string>
  url?: string
  source: ConfigSource
}

export interface SkillItem {
  name: string
  description: string
  path: string
  source: ConfigSource
}

export interface AgentItem {
  name: string
  description: string
  path: string
  source: ConfigSource
}

export interface HookItem {
  event: string
  matcher?: string
  command?: string
  type?: string
  source: ConfigSource
}

export interface ProjectConfigResult {
  basic: BasicConfigItem[]
  mcp: McpServerItem[]
  skills: SkillItem[]
  agents: AgentItem[]
  hooks: HookItem[]
}

// Skill 信息（用于面板显示）
export interface SkillInfo {
  name: string           // Skill 名称（如 "deploy" 或 "paper-tool:paper-search"）
  displayName: string    // 显示名称（去除前缀）
  description?: string
  sourceType: 'project' | 'user' | 'plugin'
  sourceLabel: string
  invokeFormat: string   // 调用格式: /skill 或 /plugin:skill
  enabled?: boolean      // 是否启用（仅 user 源可能为 false）
}

// Agent 信息（用于面板显示）
export interface AgentInfo {
  name: string
  displayName: string
  description?: string
  sourceType: 'builtin' | 'plugin' | 'user' | 'project'
  sourceLabel: string
  model?: string
  invokeFormat: string
  enabled?: boolean      // 是否启用（仅 user 源可能为 false）
}

// MCP Server 信息（用于面板显示）
export interface McpServerInfo {
  name: string
  displayName: string
  description?: string
  sourceType: 'plugin' | 'user' | 'project' | 'local' | 'managed'
  sourceLabel: string
  serverType?: string
  status?: string
  url?: string      // HTTP/SSE server URL
  command?: string  // stdio server command
  args?: string[]   // stdio server arguments
  env?: Record<string, string>  // stdio server environment variables
  headers?: Record<string, string>  // HTTP headers for authentication
  prompts: McpPromptInfo[]
  enabled?: boolean      // 是否启用（仅 user 源可能为 false）
}

// MCP Server 详情（通过 MCP 协议获取）
export interface McpServerDetail {
  name: string
  serverInfo?: ServerInfo
  capabilities?: ServerCapabilities
  tools: McpToolInfo[]
  prompts: McpPromptDetailInfo[]
  resources: McpResourceInfo[]
  cachedAt?: number
}

export interface ServerInfo {
  name: string
  version: string
}

export interface ServerCapabilities {
  tools: boolean
  prompts: boolean
  resources: boolean
}

export interface McpToolInfo {
  name: string
  description?: string
  inputSchema?: Record<string, unknown>
}

export interface McpPromptDetailInfo {
  name: string
  description?: string
  arguments?: PromptArgument[]
}

export interface PromptArgument {
  name: string
  description?: string
  required: boolean
}

export interface McpResourceInfo {
  uri: string
  name: string
  description?: string
  mimeType?: string
}

// MCP Prompt 信息
export interface McpPromptInfo {
  name: string
  description?: string
  invokeFormat: string
}

// Plugin 内部 Skill 信息
export interface PluginSkill {
  name: string           // Skill 名称（不含 plugin 前缀）
  description?: string
  invokeFormat: string   // /plugin-name:skill-name
}

// Plugin 内部 Agent 信息
export interface PluginAgent {
  name: string           // Agent 名称
  description?: string
  invokeFormat: string   // @"plugin-name:agent-name (agent)"
}

// Plugin 提供的单条 hook（只读展示）
export interface PluginHookEntry {
  event: string
  matcher?: string
  type?: string
  /** 摘要：command → url → prompt 回退 */
  summary?: string
  timeout?: unknown
}

// Plugin 信息（用于面板显示）
export interface PluginInfo {
  id: string
  name: string
  version: string
  scope: 'user' | 'project'
  enabled: boolean
  installPath: string
  installedAt?: string
  lastUpdated?: string
  projectPath?: string
  // Plugin 提供的组件（详细列表）
  skills?: PluginSkill[]
  agents?: PluginAgent[]
  mcpServers?: Record<string, {
    type?: string
    command?: string
    args?: string[]
  }>
  /** Plugin 提供的 commands（.md 文件名，展示时并入 skills） */
  commands?: string[]
  /** Plugin 提供的 hook 明细（事件 + matcher + 摘要） */
  hooks?: PluginHookEntry[]
  /** Plugin 提供的 LSP 服务器名 */
  lsp?: string[]
  /** Plugin 提供的 monitor 名 */
  monitors?: string[]
  /** Plugin 提供的主题名 */
  themes?: string[]
}