// 传输层统一入口（统一管理：桌面 Tauri 与浏览器远程共用同一套封装）。
// invoke/listen 签名与 @tauri-apps/api 完全一致，下方函数体零感知运行环境。
// （箭头包装而非方法解构——remoteTransport.invoke 内部依赖 this）
import { transport, type Unlisten, type InvokeOpts } from './transport';

const invoke = <T,>(cmd: string, args?: Record<string, unknown>, opts?: InvokeOpts): Promise<T> =>
  transport.invoke<T>(cmd, args, opts);
const listen = <T,>(event: string, cb: (payload: T) => void): Promise<Unlisten> =>
  transport.listen<T>(event, cb);

// 桌面专属插件 API（浏览器模式调用会 reject；调用方负责 isTauri 守卫）
import { open } from '@tauri-apps/plugin-dialog';
import { check } from '@tauri-apps/plugin-updater';
import { relaunch } from '@tauri-apps/plugin-process';

export const isTauriEnv = transport.env.isTauri;
export const remoteTransportRef = transport;

// 从统一类型目录导入
import type {
  PtySpawnResult,
  PtyOutputPayload,
  PtyExitPayload,
  PtyErrorPayload,
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
  HookStatusPayload,
  RemoteStatus,
} from '@/types';

// 重新导出类型（保持兼容性）
export type {
  PtySpawnResult,
  PtyOutputPayload,
  PtyExitPayload,
  PtyErrorPayload,
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
  /** 浏览器远程模式：结构化启动选项（桌面模式忽略，走 args） */
  resume?: string
  skipPermissions?: boolean
  customArgs?: string
}

export const ptySpawn = async (options: PtySpawnOptions): Promise<PtySpawnResult> => {
  // 统一会话创建入口：桌面 pty_spawn / 浏览器 session.create+attach（transport 分发）
  const r = await transport.spawnSession(options);
  return { id: r.id, type: options.type, cwd: r.cwd };
};

export const ptyInput = async (id: string, data: string): Promise<boolean> => {
  return transport.sessionInput(id, data);
};

export const ptyResize = async (id: string, cols: number, rows: number): Promise<boolean> => {
  return transport.sessionResize(id, cols, rows);
};

export const ptyKill = async (id: string, quiet?: boolean): Promise<boolean> => {
  return transport.sessionKill(id, quiet);
};

export const ptyKillAll = async (): Promise<void> => {
  // 浏览器模式无此命令（系统级），静默跳过
  if (isTauriEnv) {
    await invoke<void>('pty_kill_all');
  }
};

/** 桌面端/本端接管被远程持有的会话（浏览器端 = 重新 attach 抢占，传实际尺寸防重置） */
export const ptyClaim = async (id: string, cols?: number, rows?: number): Promise<void> => {
  return transport.sessionClaim(id, cols, rows);
};

/** 桌面端显式接管全局控制权（设备互斥遮罩「接管」按钮；浏览器端走
 * controller.takeover 协议消息，不经此 command） */
export const controllerTakeover = async (): Promise<void> => {
  if (isTauriEnv) {
    await invoke<void>('controller_takeover');
  } else {
    transport.sendClient({ type: 'controller.takeover' });
  }
};

/** 当前全局控制权持有者（''=无人；'desktop'=桌面；数字串=远程连接 id）。
 * 桌面 UI 启动/重载时错过历史 controller-changed 事件，挂载时查询补初始快照 */
export const controllerStatus = (): Promise<string> => invoke<string>('controller_status');

// ============================================
// Event Listeners (Tauri events)
// ============================================

export const onPtyOutput = (callback: (payload: PtyOutputPayload) => void): Promise<Unlisten> =>
  listen<PtyOutputPayload>('pty-output', callback);

export const onPtyExit = (callback: (payload: PtyExitPayload) => void): Promise<Unlisten> =>
  listen<PtyExitPayload>('pty-exit', callback);

/** PTY 启动/运行错误（stage 标明失败阶段），用于反哺 CLI 可用状态 */
export const onPtyError = (callback: (payload: PtyErrorPayload) => void): Promise<Unlisten> =>
  listen<PtyErrorPayload>('pty-error', callback);

/** 远程接管事件（payload 为 { id: ptyId }） */
export const onSessionTakenOver = (callback: (payload: { id: string }) => void): Promise<Unlisten> =>
  listen<{ id: string }>('session-taken-over', callback);

/** 远程归还事件（payload 为 { id: ptyId }） */
export const onSessionReclaimed = (callback: (payload: { id: string }) => void): Promise<Unlisten> =>
  listen<{ id: string }>('session-reclaimed', callback);

/** 远程（手机）新建会话事件（payload 为 { id: ptyId, data: cwd }），桌面建 tab 绑定该 PTY */
export const onRemoteSessionCreated = (callback: (payload: { id: string; data: string }) => void): Promise<Unlisten> =>
  listen<{ id: string; data: string }>('remote-session-created', callback);

/** 远程（手机）关闭会话事件（payload 为 { id: ptyId }），桌面关闭对应 Tab（用户显式关闭语义） */
export const onRemoteSessionClosed = (callback: (payload: { id: string }) => void): Promise<Unlisten> =>
  listen<{ id: string }>('remote-session-closed', callback);

/** 会话重命名双端同步（广播中转 command；两端收 session-renamed 应用同一结果） */
export const renameSession = (ptyId: string, name: string): Promise<void> =>
  invoke<void>('rename_session', { ptyId, name });

/** 会话重命名同步事件（payload 为 { id: ptyId, data: 新名字 }），按 ptyId 应用 updateTabName */
export const onSessionRenamed = (callback: (payload: { id: string; data: string }) => void): Promise<Unlisten> =>
  listen<{ id: string; data: string }>('session-renamed', callback);

/** 会话被某端打开查看（attach，payload 为 { id: ptyId }）：pending 视为已读，各端 Tab 状态点同步清金黄 */
export const onSessionViewed = (callback: (payload: { id: string }) => void): Promise<Unlisten> =>
  listen<{ id: string }>('session-viewed', callback);

/**
 * 写命令统一状态广播（payload 为 { id: section, data: revision }）。
 * 任意端写操作（开关/Provider/配置）落盘后全端收到，按 section 权威刷新；
 * revision 单调递增，旧事件（≤ 已应用版本）应被丢弃。
 */
export const onStateChanged = (callback: (payload: { id: string; data: string }) => void): Promise<Unlisten> =>
  listen<{ id: string; data: string }>('state-changed', callback);

// Hook 监控事件
export const onHookEvent = (callback: (payload: HookEventPayload) => void): Promise<Unlisten> =>
  listen<HookEventPayload>('hook-event', callback);

/** hook 状态事件（Rust 状态机单点计算的 working/pending，前端唯一状态来源） */
export const onHookStatus = (callback: (payload: HookStatusPayload) => void): Promise<Unlisten> =>
  listen<HookStatusPayload>('hook-status', callback);

// Menu events
export const onMenuSettings = (callback: () => void): Promise<Unlisten> =>
  listen('menu:settings', () => callback());

export const onMenuShortcuts = (callback: () => void): Promise<Unlisten> =>
  listen('menu:shortcuts', () => callback());

export const onConfigFontSize = (callback: (size: number) => void): Promise<Unlisten> =>
  listen<number>('config:fontSize', callback);

export const onTerminalRestart = (callback: (data: { cwd: string }) => void): Promise<Unlisten> =>
  listen<{ cwd: string }>('terminal:restart', callback);

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

/** 隐藏项目（首页不再主动展示；写命令，广播 state-changed(projects)） */
export const hideProject = (path: string): Promise<void> =>
  invoke<void>('hide_project', { path });

/** 取消隐藏项目（重新添加该项目时恢复展示） */
export const unhideProject = (path: string): Promise<void> =>
  invoke<void>('unhide_project', { path });

/** 目录浏览条目（远程端"添加项目"的目录选择器；仅子目录） */
export interface DirEntryLite {
  name: string
  path: string
  /** 修改时间（Unix 毫秒；取不到为 null） */
  modified: number | null
}

export const listDirectories = (path?: string): Promise<DirEntryLite[]> =>
  invoke<DirEntryLite[]>('list_directories', { path: path ?? null });

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

// ============================================
// Remote
// ============================================

export const getRemoteStatus = (): Promise<RemoteStatus> =>
  invoke<RemoteStatus>('get_remote_status');

export const setRemoteEnabled = (enabled: boolean): Promise<RemoteStatus> =>
  invoke<RemoteStatus>('set_remote_enabled', { enabled });

/** 配置/停用中继（url 为空字符串 = 停用），返回最新状态 */
export const setRelayUrl = (url: string | null): Promise<RemoteStatus> =>
  invoke<RemoteStatus>('set_relay_url', { url });

/** 手动更新中继配对码（一次性码被设备消费后为下一台设备重申），返回最新状态 */
export const refreshRelayPairCode = (): Promise<RemoteStatus> =>
  invoke<RemoteStatus>('refresh_relay_pair_code');

/** 刷新访问密钥：轮换 LAN token 与中继设备身份并踢掉一切已连接设备，
 *  所有已配对设备失效须重新扫码，返回最新状态 */
export const rotateRemoteSecret = (): Promise<RemoteStatus> =>
  invoke<RemoteStatus>('rotate_remote_secret');

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
  // 后端同步执行 `claude plugin enable/disable` 子进程，可能超过远程 rpc 默认 15s；
  // 桌面实现忽略 timeoutMs（本地 IPC 无超时）。超时会让远程先报错回滚、桌面最终成功。
  invoke<void>('set_plugin_enabled', { pluginId, enabled }, { timeoutMs: 60000 });

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
// File Transfer（远程浏览器模式上传/下载；传输逻辑层见 api/fileTransfer.ts）
// ============================================

export interface FileEntryLite {
  name: string;
  path: string;
  isDir: boolean;
  size?: number | null;
  modified?: number | null;
}

export interface TransferLimits {
  directMax: number;
  relayMax: number;
}

/** 慢命令超时（分块 rpc 在弱网下需超过默认 15s 的窗口） */
const FILE_RPC_TIMEOUT = 60_000;

export const fileTransferLimits = (): Promise<TransferLimits> =>
  invoke<TransferLimits>('file_transfer_limits', undefined, { timeoutMs: FILE_RPC_TIMEOUT });

export const fileList = (path: string): Promise<FileEntryLite[]> =>
  invoke<FileEntryLite[]>('file_list', { path }, { timeoutMs: FILE_RPC_TIMEOUT });

interface UploadBeginResult { uploadId: string; targetPath: string; relativePath: string; fileName: string }

export const fileUploadBegin = (ptyId: string, fileName: string, size: number): Promise<UploadBeginResult> =>
  invoke<UploadBeginResult>('file_upload_begin', { ptyId, fileName, size }, { timeoutMs: FILE_RPC_TIMEOUT });

export const fileUploadChunk = (uploadId: string, offset: number, dataB64: string): Promise<{ received: number }> =>
  invoke<{ received: number }>('file_upload_chunk', { uploadId, offset, dataB64 }, { timeoutMs: FILE_RPC_TIMEOUT });

export const fileUploadFinish = (uploadId: string): Promise<{ targetPath: string; relativePath: string }> =>
  invoke<{ targetPath: string; relativePath: string }>('file_upload_finish', { uploadId }, { timeoutMs: FILE_RPC_TIMEOUT });

export const fileUploadAbort = (uploadId: string): Promise<void> =>
  invoke<void>('file_upload_abort', { uploadId }, { timeoutMs: FILE_RPC_TIMEOUT });

interface DownloadBeginResult { downloadId: string; fileName: string; size: number; mtime?: number | null }

export const fileDownloadBegin = (path: string): Promise<DownloadBeginResult> =>
  invoke<DownloadBeginResult>('file_download_begin', { path }, { timeoutMs: FILE_RPC_TIMEOUT });

export const fileDownloadChunk = (downloadId: string, offset: number): Promise<{ dataB64: string; bytes: number; eof: boolean }> =>
  invoke<{ dataB64: string; bytes: number; eof: boolean }>('file_download_chunk', { downloadId, offset }, { timeoutMs: FILE_RPC_TIMEOUT });

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
  listen<InstallProgress>('download-progress', callback);

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
  listen<string>('open-directory', callback);