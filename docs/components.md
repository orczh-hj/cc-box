# 组件结构

## 组件树

```
App.vue
├── TitleBar.vue                    # 自定义标题栏（Windows 去除原生装饰）
├── SettingsOverlay.vue             # 全局设置浮层
├── TerminalView.vue                # 终端主视图（常驻 DOM，v-show 控制）
│   ├── IconBar.vue                 # 左侧图标栏（面板切换入口）
│   ├── SidebarPanel.vue            # 侧边栏面板容器
│   │   ├── SessionsPanel.vue       # 会话管理面板
│   │   │   ├── SessionList.vue
│   │   │   └── SessionItem.vue + SessionStatus.vue
│   │   ├── SkillsPanel.vue         # Skills 面板
│   │   ├── AgentsPanel.vue         # Agents 面板
│   │   ├── McpPanel.vue            # MCP Servers 面板
│   │   └── PluginsPanel.vue        # Plugins 面板
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
- Open Folder 按钮

### SidebarPanel.vue — 侧边栏面板

根据 `sidebarStore.activePanel` 显示对应面板内容，支持 `v-show` 切换（保留各面板状态）。

### TitleBar.vue — 自定义标题栏

Windows 平台去除原生装饰后自定义的拖拽区域 + 窗口控制按钮。

### ControllerMask.vue — 全局控制权遮罩（桌面/浏览器同构挂载）

设备互斥（同一时间只有一个设备操作——桌面与远程对等参与）的展示层：
- 挂载于 App.vue（双模式）：桌面经 Tauri 事件 `controller-changed` 驱动；浏览器经 rpc.event 驱动（payload `{id: "desktop"|连接id|空串}`）
- 显示条件：控制者标签非空且不等于本端标签（桌面 = 'desktop'，浏览器 = 连接 id）
- 本端标签（selfLabel）**响应式维护**：浏览器端每次重连服务端重新分配连接 id（旧 id 永不复用），`remote-connected` 回调同步刷新 selfLabel 与 controllerLabel——setup 一次性快照会在重连后过期，导致自己持有/接管确认时新 id ≠ 旧快照 → 自己遮自己且接管解不开
- 桌面端初始快照：`controller-changed` 只在变化时广播，桌面 UI 启动/重载会错过历史事件，挂载时经 `controller_status` command 主动查询补快照
- 「接管」按钮：桌面调 `controller_takeover` command；浏览器发 `controller.takeover` 协议消息（服务端剥离旧持有者终端并广播）
- 控制权由 Rust 后端裁决（attach 不能抢占、远程写操作被服务端拦截、桌面普通操作不自动持有），本组件只做展示
- 接管确认（controller-changed 变为自己）由 remote-transport 按"重连"语义触发全量重同步（sessions.list 重拉 + 终端重 attach）
- 层级高于终端级 taken-over 遮罩（两层并存）

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
activePanel: SidebarPanelType  // 'sessions' | 'skills' | 'agents' | 'mcp' | 'plugins' | null
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
