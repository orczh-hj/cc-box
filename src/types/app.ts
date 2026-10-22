// App 配置相关类型定义

export interface CheckResult {
  name: string
  passed: boolean
  message: string
  detectedPath?: string
  /** 来源标记："configured"（用户配置）| "path"（PATH 检测） */
  source?: 'configured' | 'path'
  action?: string
  url?: string
}

export interface HomeData {
  projects: import('./project').Project[]
  recentSessions: import('./session').SessionInfo[]
  hasMore: boolean
}

export interface AppConfig {
  defaultContinue?: boolean
  defaultSkipPermissions?: boolean
  defaultCustomArgs?: string
  theme?: 'light' | 'dark'
  fontSize?: number
  autoConnectIde?: boolean
  hiddenProjects?: string[]
  lastOpenedProject?: string
  windowSize?: { width: number; height: number }
  claudeEnvVars?: Record<string, string>
  language?: string
  remote?: RemoteConfig
}

export interface RemoteConfig {
  enabled?: boolean
  token?: string
  port?: number
  /** 中继服务器地址（如 https://cc.orczh.top；空 = 未启用） */
  relayUrl?: string
  deviceId?: string
  deviceKey?: string
}

/** 远程连接状态（get_remote_status 返回） */
export interface RemoteStatus {
  enabled: boolean
  running: boolean
  /** 宿主制：远程由本机另一实例提供（本实例未监听），url 仍为宿主可用地址 */
  hostedElsewhere: boolean
  port: number
  token: string
  /** 手机直连 URL（含 token fragment）；token 未生成时为空 */
  url: string
  /** 中继已启用 */
  relayEnabled: boolean
  /** 中继已连接 */
  relayConnected: boolean
  /** 中继地址（如 https://cc.orczh.top） */
  relayUrl: string
  /** 当前有效配对码（5 分钟内）；无则空 */
  relayPairCode: string
  /** 中继配对二维码内容（relayUrl + #pair=code）；无效则空 */
  relayPairUrl: string
}

export interface DefaultClaudeOptions {
  skipPermissions?: boolean
  customArgs?: string
}

// Claude 启动选项（前端使用）
export interface ClaudeOptions {
  resume: string
  skipPermissions: boolean
  customArgs: string
}

// 软件更新信息
export interface PlatformAsset {
  name: string
  url: string
  size: number
}

export interface UpdateInfo {
  version: string
  currentVersion: string
  hasUpdate: boolean
  releaseNotes: string
  downloadUrl: string
  platformAsset: PlatformAsset | null
}

export interface DownloadProgress {
  downloaded: number
  total: number
  percent: number
}

// Claude CLI 更新信息
export interface ClaudeCliUpdateInfo {
  installedVersion: string | null
  latestVersion: string
  hasUpdate: boolean
  notInstalled: boolean
}

// Claude CLI 单个历史版本条目
export interface ClaudeVersionEntry {
  version: string
  releaseDate: string
  platforms: Record<string, ClaudePlatformInfo>
}

// Claude CLI 版本的平台产物信息
export interface ClaudePlatformInfo {
  url: string
  checksum: string
  size: number
}

// versions.json 顶层结构
export interface ClaudeVersions {
  latest: string
  updatedAt: string
  versions: ClaudeVersionEntry[]
}
