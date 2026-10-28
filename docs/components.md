# 组件结构

## 组件树

```
App.vue
├── TitleBar.vue                    # 自定义标题栏（Windows 去除原生装饰）
├── SettingsOverlay.vue             # 全局设置浮层
├── DeviceSwitcherDialog.vue        # 浏览器模式多设备切换弹窗（remote/DeviceList.vue 复用设备列表）
├── TerminalView.vue                # 终端主视图（常驻 DOM，v-show 控制）
│   ├── IconBar.vue                 # 左侧图标栏（面板切换入口）
│   ├── SidebarPanel.vue            # 侧边栏面板容器
│   │   ├── SessionsPanel.vue       # 会话管理面板
│   │   │   ├── SessionList.vue
│   │   │   └── SessionItem.vue + SessionStatus.vue
│   │   ├── SkillsPanel.vue         # Skills 面板
│   │   ├── AgentsPanel.vue         # Agents 面板
│   │   ├── McpPanel.vue            # MCP Servers 面板
│   │   ├── PluginsPanel.vue        # Plugins 面板
│   │   └── FilesPanel.vue          # 文件传输面板（浏览器远程模式专属；上传/下载两 section）
│   ├── TerminalHeader.vue          # 终端标题栏（会话名 + 返回按钮；项目名由标签页 title 承载）
│   └── XTermTerminal.vue           # xterm.js 终端核心
│
├── WelcomeView.vue                 # 欢迎引导页（覆盖层，无收藏项目时）
└── ProjectSelectView.vue           # 项目选择页（覆盖层，有收藏项目时）
```

状态同步中心（非组件，App.vue 消费）：`src/state/sections.ts` — state-changed section → loader 注册表。登记项：skills/agents/mcp/plugins（sidebar loader，plugins 级联子项）、providers/config（对应 store loader，设置面板打开期间 defer 暂存关闭补刷）。API：`registerSectionLoader` / `applyStateSection` / `flushPendingSections` / `registeredSections`（state-resync-needed 全量重拉遍历用）。

## 组件详情

### App.vue — 视图切换 + 环境检查

- 管理三个视图：`welcome` / `projects` / `terminal`
- TerminalView 使用 `v-show` 常驻 DOM（保持 PTY 和终端实例不销毁）
- WelcomeView/ProjectSelectView 使用 `v-if` 覆盖层叠加在终端之上
- 环境检查失败时显示全屏遮罩
- 初始化：hook store、快捷键监听、自动更新检查
- state-changed 订阅只转发到状态同步中心（`src/state/sections.ts` 查表分发：section → loader、级联、暂存——新功能在注册表加一行，不改本组件）
- 浏览器模式 document.title 跟随当前项目（多标签页区分；桌面窗口标题由 Tauri 管理）

### XTermTerminal.vue — 终端核心

职责：
- 管理多个终端实例（Map<tabId, TerminalInstance>）
- 加载 FitAddon、SearchAddon、WebLinksAddon、SerializeAddon
- 双向数据绑定：onData → ptyInput，onPtyOutput → term.write
- Tab 创建/切换/重启/关闭
- Ctrl+V 粘贴处理
- 会话匹配轮询（通过 sessionStore）

Props：
- `fontSize: number` — 终端字号

Events：
- `ptyStarted(tabId, ptyId)` — PTY 启动成功

### TerminalView.vue — 终端主视图

布局：
```
┌────┬──────────────┬───────────────────────────┐
│Icon│  SidebarPanel │  TerminalHeader (38px)     │
│Bar │  (sessions/   ├───────────────────────────┤
│(40px│  skills/     │                           │
│    │  agents/      │  XTermTerminal (flex:1)   │
│    │  mcp/         │                           │
│    │  plugins)     │                           │
└────┴──────────────┴───────────────────────────┘
```

职责：
- 组合 IconBar、SidebarPanel、TerminalHeader、XTermTerminal
- 管理会话操作（新建/切换/重命名/恢复/关闭）
- 监听 cwd 变化，加载项目配置和历史会话
- 初始化 `useWindowAttention`（窗口聚焦状态）和 `useStatusMonitor`（hook 事件→Tab 状态）

### IconBar.vue — 左侧图标栏

固定宽度 40px，提供面板切换入口：
- Sessions、Skills、Agents、MCP、Plugins 图标按钮
- Settings 按钮
- Open Folder 按钮（桌面专属）
- 浏览器模式：Files 按钮（设备切换只在首页，避免手机底栏拥挤）
- 手机断点：底栏左侧面板组可横向滚动（隐藏滚动条），右侧 Files/Settings 固定不收缩；按钮宽度 36px、间距 4px 以节省横向空间

### SidebarPanel.vue — 侧边栏面板

根据 `sidebarStore.activePanel` 显示对应面板内容，支持 `v-show` 切换（保留各面板状态）。

### DeviceSwitcherDialog.vue / DeviceList.vue — 多设备切换（浏览器远程模式专属）

`DeviceSwitcherDialog` 全局挂载于 App.vue，只由**首页**（ProjectSelectView / 无项目时的 WelcomeView）右上角设备按钮打开（`useDeviceSwitcher` 单例状态）；终端页 IconBar 不放设备入口，避免手机底栏拥挤。`DeviceList` 复用同一列表 UI：名称、LAN/中继、令牌无效、当前设备高亮、移除配对（当前设备不显示移除按钮；其他设备 × 需二次确认）；TokenGate 也使用它。选择设备调用 `remoteTransport.activateDesktop`（同 origin 原地重连，跨 origin 跳转）。切换成功后 `remote-connected` 携带 `deviceChanged=true`，App 调用 `sessionStore.resetRemoteTabs()` 丢弃上一台设备的 Tabs/历史缓存（不 kill 远端 PTY）。

### FilesPanel.vue — 文件传输面板（浏览器远程模式专属，IconBar `v-if="!isTauriEnv"` 守卫）

上传（手机 → 桌面 `.cc-box-files/`）与下载（授权项目内文件 → 手机）两 section，状态机 idle/transferring（进度+取消）/done/error。**职责分层**：分块循环/重试≤2/取消在 `src/api/fileTransfer.ts`（uploadFile/downloadFile/quotePath），本组件只做状态机与展示。

- 上传完成：仅当活动终端仍是发起时的 ptyId 且 running 才 `sendTerminalCommand(quotePath(targetPath))` 插入（防串终端；路径转义三档：裸/双引号/单引号）
- 下载：路径输入 + 项目内简易浏览器（`file_list`，入口=活动 Tab 项目根，回上级到根为止；文件点选回填不自动下载）；保存走 `saveBlobToFile`（utils/platform.ts，a[download]）
- 限额：挂载时 `file_transfer_limits` 拉取，按 `remoteTransport.viaRelay` 选档展示；中继常显流量成本提示
- 错误映射：超限 / 项目外路径 / 源文件已变化 / 超时等本地化文案（控制权被占分支已随全局控制权退役删除）

### TitleBar.vue — 自定义标题栏

Windows 平台去除原生装饰后自定义的拖拽区域 + 窗口控制按钮。

## Store 结构

### app.ts — 应用状态

```typescript
cwd: string                           // 当前工作目录
theme: string                         // 主题
fontSize: number                      // 终端字号
pendingResume: PendingResume | null   // 待恢复会话信息
checkResults: CheckResult[]           // 环境检查结果
cachedProjects: Project[]             // 项目列表缓存（分页）
cachedRecentSessions: SessionInfo[]   // 近期会话缓存
defaultClaudeOptions: DefaultClaudeOptions  // 持久化默认启动参数
claudeOptions: ClaudeOptions          // 当前启动参数
```

方法：loadAppConfig、runChecks、loadCache、loadMoreProjects、setCwd、setFontSize、getClaudeArgs 等

### session.ts — 会话管理

```typescript
// Tab 数据模型（跨越 PTY 生命周期的稳定 UI 单元）
interface TerminalTab {
  tabId: string              // 稳定 ID
  projectPath: string
  ptyId: string | null       // PTY 进程 ID（停止时 null）
  sessionId: string | null   // Claude session ID（匹配后赋值）
  name: string
  status: 'starting' | 'running' | 'stopped'
  createdAt: number
  lastActiveAt: number
  working: boolean           // 正在工作中（用户发消息后、响应返回前）
  pending: boolean           // 需要用户关注（响应完成但用户未看到）
  model?: string             // 模型名
}

tabs: Map<string, TerminalTab>        // 所有 Tab
activeTabId: string | null            // 当前活跃 Tab
historySessions: HistorySession[]     // 未被 Tab 占用的历史会话
```

方法：createTab、setTabPty、handlePtyExit、closeTab、assignSessionIdByPtyId、adoptRemoteSessions（远程会话采纳：Tab 名优先取历史会话名，未加载时回退项目名）、backfillTabNamesFromHistory（历史加载后回填采纳 Tab 的会话名，已改名的不覆盖）

### sidebar.ts — 侧边栏状态

```typescript
activePanel: SidebarPanelType  // 'sessions' | 'skills' | 'agents' | 'mcp' | 'plugins' | 'files' | null
panelVisible: boolean
showSettings: boolean
// 预加载数据
skills: SkillInfo[]
agents: AgentInfo[]
mcpServers: McpServerInfo[]
plugins: PluginInfo[]
updateInfo: UpdateInfo | null
```

方法：togglePanel、loadAllSidebarData、openSettings

### config.ts — 项目配置

```typescript
projectConfig: ProjectConfigResult | null  // 当前项目 Claude 配置（只读展示）
```

方法：loadProjectConfig（带缓存）

## 色彩系统

主色调：**墨蓝 + 琥珀金**，温暖米灰基底。

```css
/* GUI 层 */
--bg-primary: #faf9f6;        /* 温暖米灰 */
--accent-primary: #1e3a5f;    /* 深邃墨蓝 */
--accent-gold: #d4a574;       /* 琥珀金 */

/* 状态语义色 */
--status-success: #3d8c6e;    /* 墨绿 */
--status-info: #2a5082;       /* 墨蓝 */
--status-warning: #c4964a;    /* 琥珀 */
--status-error: #c45c4a;      /* 赭红 */

/* 终端层 */
--terminal-bg: #f8f9fa;       /* 浅灰背景 */
--terminal-fg: #1a1816;       /* 深炭灰文字 */
```
