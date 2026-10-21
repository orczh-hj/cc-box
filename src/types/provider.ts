/** Provider 分类 */
export type ProviderCategory =
  | 'official'       // Anthropic 官方
  | 'cn_official'    // 国内官方（DeepSeek、智谱等）
  | 'cloud_provider' // 云服务商（AWS Bedrock、Vertex AI）
  | 'aggregator'     // API 聚合平台（OpenRouter、SiliconFlow）
  | 'third_party'    // 第三方供应商
  | 'local'          // 本机推理服务（Ollama、LM Studio 等）
  | 'custom'         // 自定义
  | 'omo'            // Oh My OpenCode
  | 'omo-slim'       // Oh My OpenCode Slim

/** API Key 字段名 */
export type ClaudeApiKeyField = 'ANTHROPIC_AUTH_TOKEN' | 'ANTHROPIC_API_KEY'

/** API 格式 */
export type ClaudeApiFormat =
  | 'anthropic'        // 原生 Anthropic Messages API
  | 'openai_chat'      // OpenAI Chat Completions 格式
  | 'openai_responses' // OpenAI Responses API 格式
  | 'gemini_native'    // Gemini Native API 格式

/** Provider 元数据（完全兼容 cc-switch ProviderMeta，22 个字段） */
export interface ProviderMeta {
  commonConfigEnabled?: boolean
  endpointAutoSelect?: boolean
  apiFormat?: string
  apiKeyField?: string
  isFullUrl?: boolean
  providerType?: string
  costMultiplier?: string
  pricingModelSource?: string
  limitDailyUsd?: string
  limitMonthlyUsd?: string
  isPartner?: boolean
  partnerPromotionKey?: string
  testConfig?: Record<string, any>
  usageScript?: Record<string, any>
  authBinding?: Record<string, any>
  promptCacheKey?: string
  codexFastMode?: boolean
  liveConfigManaged?: boolean
  githubAccountId?: string
  customEndpoints?: Record<string, any>
}

/** Provider 实例（完全兼容 cc-switch Provider） */
export interface Provider {
  id: string
  name: string
  settingsConfig: Record<string, any>
  websiteUrl?: string
  category?: string                   // 字符串类型，与 cc-switch 一致
  createdAt?: number
  sortIndex?: number
  notes?: string
  meta?: ProviderMeta
  icon?: string
  iconColor?: string
  inFailoverQueue: boolean            // 非可选，默认 false
}

/** 模板变量配置（用于预设中的动态替换） */
export interface TemplateValueConfig {
  label: string
  placeholder: string
  defaultValue?: string
  editorValue: string
}

/** Provider 预设模板 */
export interface ProviderPreset {
  name: string
  nameKey?: string
  websiteUrl: string
  apiKeyUrl?: string
  settingsConfig: Record<string, any>
  isOfficial?: boolean
  isPartner?: boolean
  partnerPromotionKey?: string
  category?: ProviderCategory
  apiKeyField?: ClaudeApiKeyField
  templateValues?: Record<string, TemplateValueConfig>
  endpointCandidates?: string[]
  icon?: string
  iconColor?: string
  apiFormat?: ClaudeApiFormat
  providerType?: string
  requiresOAuth?: boolean
  hidden?: boolean
  modelsUrl?: string
}

/** 通用配置片段 */
export interface CommonConfig {
  enabled: boolean
  settings: Record<string, any>
}

/** Provider 配置文件结构 */
export interface ProvidersConfig {
  providers: Provider[]
  commonConfig: CommonConfig
  activeProviderId: string | null
}

/** 导入结果 */
export interface ImportResult {
  count: number
  importedCommonConfig: boolean
  activeProviderName: string | null
}

/** 测试连接结果 */
export interface TestConnectionResult {
  success: boolean
  message: string
  latencyMs: number | null
}
