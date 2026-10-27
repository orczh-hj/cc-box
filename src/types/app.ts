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
  /** 与后台 Broker 的 IPC 是否已注册 */
  brokerConnected: boolean
  /** 旧版 CC-Box 占用设备入口：设备级多实例控制暂不可用 */
  legacyOwned: boolean
  /** 局域网监听是否正常 */
  lanRunning: boolean
  /** 局域网不可用原因（第三方占口等） */
  lanError?: string
  /** 开机自启动（仅 Broker） */
  autostart: boolean
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
