import { invoke } from '@tauri-apps/api/core';
import { listen, type Unlisten } from '@tauri-apps/api/event';
import { open } from '@tauri-apps/plugin-dialog';
import { check } from '@tauri-apps/plugin-updater';
import { relaunch } from '@tauri-apps/plugin-process';

// 从统一类型目录导入
import type {
  PtySpawnResult,
  PtyOutputPayload,
  PtyExitPayload,
  Project,
  SessionInfo,
  SessionDetails,
  SessionSearchResult,
  AppConfig,
  DefaultClaudeOptions,
  ProjectConfigResult,
  AgentInfo,
  McpServerInfo,
  McpServerDetail,
  PluginInfo,
  SkillInfo,
  UpdateInfo,
  DownloadProgress,
  ClaudeCliUpdateInfo,
  ClaudeVersionEntry,
  ClaudeVersions,
  HomeData,
  CheckResult,
  HookEventPayload,
} from '@/types';

// 重新导出类型（保持兼容性）
export type {
  PtySpawnResult,
  PtyOutputPayload,
  PtyExitPayload,
  Project,
  SessionInfo,
  SessionDetails,
  SessionSearchResult,
  AppConfig,
  DefaultClaudeOptions,
  ProjectConfigResult,
  AgentInfo,
  McpServerInfo,
  McpServerDetail,
  PluginInfo,
  SkillInfo,
  UpdateInfo,
  DownloadProgress,
  ClaudeCliUpdateInfo,
  ClaudeVersionEntry,
  ClaudeVersions,
};

// ============================================
// PTY Operations
// ============================================

interface PtySpawnOptions {
  cwd: string
  cols: number
  rows: number
  type: 'claude' | 'shell'
  args?: string[]
}

export const ptySpawn = async (options: PtySpawnOptions): Promise<PtySpawnResult> => {
  return invoke<PtySpawnResult>('pty_spawn', { options });
};

export const ptyInput = async (id: string, data: string): Promise<boolean> => {
  return invoke<boolean>('pty_input', { id, data });
};

export const ptyResize = async (id: string, cols: number, rows: number): Promise<boolean> => {
  return invoke<boolean>('pty_resize', { id, cols, rows });
};

export const ptyKill = async (id: string): Promise<boolean> => {
  return invoke<boolean>('pty_kill', { id });
};

export const ptyKillAll = async (): Promise<void> => {
  return invoke<void>('pty_kill_all');
};

// ============================================
// Event Listeners (Tauri events)
// ============================================

export const onPtyOutput = (callback: (payload: PtyOutputPayload) => void): Promise<Unlisten> =>
  listen<PtyOutputPayload>('pty-output', (event) => callback(event.payload));

export const onPtyExit = (callback: (payload: PtyExitPayload) => void): Promise<Unlisten> =>
  listen<PtyExitPayload>('pty-exit', (event) => callback(event.payload));

// Hook 监控事件
export const onHookEvent = (callback: (payload: HookEventPayload) => void): Promise<Unlisten> =>
  listen<HookEventPayload>('hook-event', (event) => callback(event.payload));

// Menu events
export const onMenuSettings = (callback: () => void): Promise<Unlisten> =>
  listen('menu:settings', () => callback());

export const onMenuShortcuts = (callback: () => void): Promise<Unlisten> =>
  listen('menu:shortcuts', () => callback());

export const onConfigFontSize = (callback: (size: number) => void): Promise<Unlisten> =>
  listen<number>('config:fontSize', (event) => callback(event.payload));

export const onTerminalRestart = (callback: (data: { cwd: string }) => void): Promise<Unlisten> =>
  listen<{ cwd: string }>('terminal:restart', (event) => callback(event.payload));

// ============================================
// Projects and Sessions
// ============================================

export const getCheckResults = (): Promise<CheckResult[]> =>
  invoke<CheckResult[]>('get_check_results');

export const runChecks = (): Promise<CheckResult[]> =>
  invoke<CheckResult[]>('run_checks');

export const getHomeData = (projectLimit?: number, sessionLimit?: number): Promise<HomeData> =>
  invoke<HomeData>('get_home_data', { projectLimit, sessionLimit });

export const getProjects = (limit?: number, offset?: number): Promise<Project[]> =>
  invoke<Project[]>('get_projects', { limit, offset });

export const getProjectInfo = (path: string): Promise<Project | null> =>
  invoke<Project | null>('get_project_info', { path });

export const getSessions = (projectPath: string, limit?: number, offset?: number): Promise<SessionInfo[]> =>
  invoke<SessionInfo[]>('get_sessions', { projectPath, limit, offset });

export const getSessionCount = (projectPath: string): Promise<number> =>
  invoke<number>('get_session_count', { projectPath });

export const getAllRecentSessions = (limit?: number): Promise<SessionInfo[]> =>
  invoke<SessionInfo[]>('get_all_recent_sessions', { limit });

export const getSessionDetails = (projectPath: string, sessionId: string): Promise<SessionDetails | null> =>
  invoke<SessionDetails | null>('get_session_details', { projectPath, sessionId });

export const searchSessionMessages = (
  projectPath: string,
  query: string,
  limit?: number
): Promise<SessionSearchResult[]> =>
  invoke<SessionSearchResult[]>('search_session_messages', { projectPath, query, limit });

// ============================================
// Configuration
// ============================================

export const getAppConfig = (): Promise<AppConfig> =>
  invoke<AppConfig>('get_app_config');

export const updateAppConfig = (updates: Record<string, unknown>): Promise<void> =>
  invoke<void>('update_app_config', { updates });

export const getDefaultClaudeOptions = (): Promise<DefaultClaudeOptions> =>
  invoke<DefaultClaudeOptions>('get_default_claude_options');

export const saveDefaultClaudeOptions = (options: Partial<DefaultClaudeOptions>): Promise<void> =>
  invoke<void>('save_default_claude_options', { options });

export const saveLastProject = (path: string): Promise<void> =>
  invoke<void>('save_last_project', { path });

export const getProjectConfig = (projectPath: string): Promise<ProjectConfigResult> =>
  invoke<ProjectConfigResult>('get_project_config', { projectPath });

export const getAllAgents = (projectPath: string): Promise<AgentInfo[]> =>
  invoke<AgentInfo[]>('get_all_agents', { projectPath });

export const getAllSkills = (projectPath: string): Promise<SkillInfo[]> =>
  invoke<SkillInfo[]>('get_all_skills', { projectPath });

export const getAllMcpServers = (projectPath: string): Promise<McpServerInfo[]> =>
  invoke<McpServerInfo[]>('get_all_mcp_servers', { projectPath });

export const getAllPlugins = (projectPath: string): Promise<PluginInfo[]> =>
  invoke<PluginInfo[]>('get_all_plugins', { projectPath });

// 切换用户级 skill/agent/mcp/plugin 启用状态
export const setSkillEnabled = (name: string, enabled: boolean): Promise<void> =>
  invoke<void>('set_skill_enabled', { name, enabled });

export const setAgentEnabled = (name: string, enabled: boolean): Promise<void> =>
  invoke<void>('set_agent_enabled', { name, enabled });

export const setMcpServerEnabled = (name: string, enabled: boolean): Promise<void> =>
  invoke<void>('set_mcp_server_enabled', { name, enabled });

export const setPluginEnabled = (pluginId: string, enabled: boolean): Promise<void> =>
  invoke<void>('set_plugin_enabled', { pluginId, enabled });

export const getMcpServerDetail = (
  projectPath: string,
  serverName: string,
  forceRefresh: boolean = false
): Promise<McpServerDetail | null> =>
  invoke<McpServerDetail | null>('get_mcp_server_detail', { projectPath, serverName, forceRefresh });

// ============================================
// File Management
// ============================================

export const openInFileManager = (path: string): Promise<void> =>
  invoke<void>('open_in_file_manager', { path });

// ============================================
// Updater (Tauri official plugin)
// ============================================

export type { Update } from '@tauri-apps/plugin-updater';
export { check, relaunch };

export const checkForUpdates = async (): Promise<UpdateInfo> => {
  const update = await check();
  if (!update) {
    return {
      version: __APP_VERSION__,
      currentVersion: __APP_VERSION__,
      hasUpdate: false,
      releaseNotes: '',
      downloadUrl: '',
      platformAsset: null,
    };
  }
  return {
    version: update.version,
    currentVersion: __APP_VERSION__,
    hasUpdate: true,
    releaseNotes: update.body || '',
    downloadUrl: '',
    platformAsset: null,
  };
};

// ============================================
// App Instance
// ============================================

export const getAppPath = (): Promise<string> =>
  invoke<string>('get_app_path');

export const spawnNewInstance = (): Promise<void> =>
  invoke<void>('spawn_new_instance');

// ============================================
// Logging
// ============================================

export const logMessage = (level: 'error' | 'warn' | 'info' | 'debug', message: string): Promise<void> =>
  invoke<void>('log_message', { level, message });

// ============================================
// Dialog (Tauri dialog plugin)
// ============================================

export const selectDirectory = async (): Promise<{ path: string } | null> => {
  const result = await open({
    directory: true,
    multiple: false,
    title: 'Select Project Directory'
  } as any);
  if (result && typeof result === 'string') {
    return { path: result };
  }
  return null;
};

// ============================================
// Dependency Installation
// ============================================

export interface ClaudeLatestInfo {
  version: string
  releaseDate: string
  platforms: Record<string, PlatformInfo>
}

export interface PlatformInfo {
  url: string
  checksum: string
  size: number
}

export interface GitLatestInfo {
  version: string
  releaseDate: string
  file: string
  url: string
  size: number
}

export interface LatestVersions {
  claude: ClaudeLatestInfo
  git?: GitLatestInfo
}

export interface InstallProgress {
  item: string        // "claude" | "git"
  stage: string       // "fetching" | "downloading" | "extracting" | "placing" | "done" | "error"
  progress: number    // 0-100
  message: string
}

export const getLatestVersions = (): Promise<LatestVersions> =>
  invoke<LatestVersions>('get_latest_versions');

export const checkInstalledVersions = (): Promise<Record<string, boolean>> =>
  invoke<Record<string, boolean>>('check_installed_versions');

export const downloadAndInstallClaude = (): Promise<void> =>
  invoke<void>('download_and_install_claude');

export const downloadAndInstallGit = (): Promise<void> =>
  invoke<void>('download_and_install_git');

// 取消正在进行的自动安装（首次启动 Auto Install 触发的）
export const cancelAutoInstall = (): Promise<boolean> =>
  invoke<boolean>('cancel_auto_install');

// 获取系统 locale（BCP-47 标签，如 "zh-CN"、"en-US"）
export const getSystemLocale = (): Promise<string> =>
  invoke<string>('get_system_locale');

export const onInstallProgress = (callback: (progress: InstallProgress) => void): Promise<Unlisten> =>
  listen<InstallProgress>('download-progress', (event) => callback(event.payload));

// ============================================
// Claude CLI Update Check
// ============================================

export const checkClaudeCliUpdate = (): Promise<ClaudeCliUpdateInfo> =>
  invoke<ClaudeCliUpdateInfo>('check_claude_cli_update');

// 获取本地已安装 Claude CLI 版本号（无 HTTP）
export const getInstalledClaudeVersion = (): Promise<string | null> =>
  invoke<string | null>('get_installed_claude_version');

// 拉取所有支持的 Claude CLI 历史版本
export const listClaudeVersions = (): Promise<ClaudeVersions> =>
  invoke<ClaudeVersions>('list_claude_versions');

// 下载指定历史版本的 Claude CLI（返回本地保存路径）
export const downloadClaudeVersion = (version: string): Promise<string> =>
  invoke<string>('download_claude_version', { version });

// 取消指定历史版本的下载（返回是否找到活动下载并标记取消）
export const cancelClaudeVersionDownload = (version: string): Promise<boolean> =>
  invoke<boolean>('cancel_claude_download', { version });

// 把本地下载好的 Claude CLI 覆盖安装到 ~/.local/bin/
// 如果 claude 进程在运行，promise 会 reject 一个 'claude-running' 字符串
export const installClaudeVersion = (sourcePath: string, version: string): Promise<string> =>
  invoke<string>('install_claude_version', { sourcePath, version });

export const checkClaudeRunning = (): Promise<boolean> =>
  invoke<boolean>('check_claude_running');

export const killClaudeProcesses = (): Promise<void> =>
  invoke<void>('kill_claude_processes');

// 右键菜单打开目录
export const onOpenDirectory = (callback: (dir: string) => void): Promise<Unlisten> =>
  listen<string>('open-directory', (event) => callback(event.payload));