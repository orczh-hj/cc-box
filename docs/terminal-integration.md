# 终端集成架构

## 架构概述

使用 **Tauri 2 + xterm.js + portable-pty** 直接运行 Claude CLI，实现完全的原生终端体验。

### 核心依赖

| 包名 | 版本 | 用途 |
|------|------|------|
| @xterm/xterm | 6.1.0-beta.303 | 终端 UI 渲染 |
| @xterm/addon-fit | 0.12.0-beta.300 | 自动适应容器大小 |
| @xterm/addon-webgl | 0.20.0-beta.299 | WebGL 渲染器（含 atlas corruption 修复） |
| @xterm/addon-unicode11 | 0.10.0-beta.300 | Unicode 11 宽度支持（CJK/emoji） |
| @xterm/addon-search | 0.17.0-beta.300 | 终端内搜索 |
| @xterm/addon-web-links | 0.13.0-beta.300 | 链接点击 |
| @xterm/addon-serialize | 0.15.0-beta.300 | 会话序列化 |
| portable-pty | ^0.8 | 伪终端进程管理（Rust） |
| tauri | ^2 | 应用框架 |

> xterm 6.1.0-beta 线包含 WebGL glyph atlas corruption 上游修复（见 [webgl-corruption-fix.md](webgl-corruption-fix.md)）；beta 依赖使用精确版本号锁定。

## Rust 后端 PTY 模块 (src-tauri/src/pty.rs)

### 核心功能

```rust
pub fn spawn(&self, cwd: String, cols: u16, rows: u16, args: Option<Vec<String>>) -> Result<PtyInfo>
pub fn write(&self, id: &str, data: &str) -> Result<()>
pub fn resize(&self, id: &str, cols: u16, rows: u16) -> Result<()>
pub fn kill(&self, id: &str) -> Result<()>
pub fn kill_all(&self)
```

### Claude 进程启动

Claude CLI 有多种安装方式，启动命令构建需要智能检测启动类型：

#### 安装方式与文件特征

| 安装方式 | 命令 | 文件类型 | Windows路径示例 | Mac/Linux路径示例 |
|---------|------|---------|-----------------|-------------------|
| Native Install | `curl -fsSL https://claude.ai/install.sh \| bash` | 编译后可执行文件 | `~/.local/bin/claude.exe` | `~/.local/bin/claude` |
| npm | `npm install -g @anthropic-ai/claude-code` | Node.js脚本 | `...\node_global\node_modules\@anthropic-ai\claude-code\cli.js` | `.../node_modules/@anthropic-ai/claude-code/cli.js` |
| Homebrew | `brew install --cask claude-code` | 编译后可执行文件 | - | `/usr/local/bin/claude` |
| WinGet | `winget install Anthropic.ClaudeCode` | 编译后可执行文件 | 系统PATH | - |

#### 启动类型检测逻辑

检测优先级（检测结果保存到配置避免重复检测）：

```
1. 检查扩展名
   path.ends_with(".js") → node
2. 检查文件内容（前5行）
   "#!/usr/bin/env node" → node
   "// (c) Anthropic" + "Version:" → node (cli.js特征)
3. Mac/Linux: 解析符号链接
   canonicalize(path).ends_with(".js") → node
   真实文件内容检测 → node
否则 → direct
```

#### 命令构建逻辑

```rust
// Windows
if ends_with(".exe") { CommandBuilder::new(&claude_path) }
else if needs_node_launcher() { CommandBuilder::new("node").arg(&claude_path) }
else { CommandBuilder::new("cmd.exe").arg("/C").arg(&claude_path) }

// Mac/Linux
if needs_node_launcher() { CommandBuilder::new("node").arg(&claude_path) }
else { CommandBuilder::new(&claude_path) }
```

#### 配置存储

检测结果保存到 `~/.cc-box/config.json`：`{ "claudeLauncherType": "node" | "direct" }`

PTY 启动时优先从配置读取，无值时检测并保存。

### 数据流

```
PTY reader → PtyDecoder::decode()（跨 read 状态 + 贪心解码）→ emit('pty-output', { id, data }) → frontend
PTY exit → PtyDecoder::flush() 刷出残留 → emit('pty-exit', { id, exit_code }) → frontend
```

**输出编码处理**（`src-tauri/src/pty.rs::read_output_loop` + `src-tauri/src/pty_decoder.rs`）：

PTY 是字节流，子进程可能输出 UTF-8 或 GBK（Windows 中文 cmd.exe、某些 git 输出）。两层抽象协同：

1. **`PtyDecoder`**（有状态流式解码器，跨 read 边界）
   - 每次 `reader.read()` 的字节通过 `decoder.decode(&buf[..n])` 处理
   - 内部维护 `pending: Vec<u8>`，把末尾潜在不完整字符（UTF-8 续字节不足 / GBK 首字节缺次字节）保留到下次拼接
   - `find_safe_boundary` 贪心扫描：合法 UTF-8 序列前进对应字节、合法 GBK 双字节前进 2、末尾孤立 GBK 首字节保留、单字节非法前进 1
   - EOF 时调用 `decoder.flush()` 强制刷出残留（不完整 UTF-8 按 GBK 兜底，优于丢失）

2. **`decode_output`**（无状态字节 → 字符串，`src-tauri/src/platform.rs:117`）
   - 贪心扫描：ASCII 直解、合法 UTF-8 多字节序列优先、否则尝试 GBK 双字节、最后兜底 `U+FFFD`
   - 保证 UTF-8 与 GBK 混合输出（Claude CLI UTF-8 + cmd.exe GBK）各自正确解码，互不污染
   - 同时被 `installer.rs` 等一次性处理子进程 stdout 的场景复用

历史教训：
- v0.12.2 之前用 `String::from_utf8_lossy`，Windows 中文子进程的 GBK 字节变成黑色方块乱码
- v0.12.2 改为 UTF-8 优先 + 整体回退 GBK，但混合输出时整体 GBK 解码污染 UTF-8 内容
- v0.12.3 改为贪心扫描 + PtyDecoder 状态机，三个根源（整体 GBK 污染、carry 中间非法字节、GBK 跨 read 损坏）一并解决

回归测试：`src-tauri/src/tests/pty_decoder.rs`（PtyDecoder 行为）、`src-tauri/src/tests/pty.rs::PtyDecode_*`（decode_output 行为）。

## IPC 通道 (Tauri Commands)

### PTY 操作命令

| 命令 | 方向 | 参数 | 返回 |
|------|------|------|------|
| pty_spawn | renderer→main | { cwd, cols, rows, type, args } | PtySpawnResult |
| pty_input | renderer→main | { id, data } | boolean |
| pty_resize | renderer→main | { id, cols, rows } | boolean |
| pty_kill | renderer→main | { id } | boolean |
| pty_kill_all | renderer→main | - | void |

### PTY 事件

| 事件 | 方向 | 数据 |
|------|------|------|
| pty:output | main→renderer | { id, data } |
| pty:exit | main→renderer | { id, exit_code, signal? } |

## 前端 API (src/api/tauri.ts)

### PTY 操作

```typescript
interface PtySpawnOptions {
  cwd: string
  cols: number
  rows: number
  type: 'claude' | 'shell'
  args?: string[]
}

ptySpawn(options: PtySpawnOptions): Promise<PtySpawnResult>
ptyInput(id: string, data: string): Promise<boolean>
ptyResize(id: string, cols: number, rows: number): Promise<boolean>
ptyKill(id: string): Promise<boolean>
ptyKillAll(): Promise<void>
```

### PTY 事件监听

```typescript
onPtyOutput((payload) => { ... }): Promise<UnlistenFn>
onPtyExit((payload) => { ... }): Promise<UnlistenFn>
```

## 渲染进程终端组件 (XTermTerminal.vue)

### xterm.js 配置

```typescript
const term = new Terminal({
  fontFamily: 'Cascadia Code, Fira Code, Consolas, monospace',
  fontSize: 12,
  lineHeight: 1.2,
  cursorBlink: true,
  cursorStyle: 'bar',
  theme: lightTheme,
  allowProposedApi: true,
  macOptionIsMeta: true,
})
```

### 数据绑定

```typescript
// 用户输入 → PTY
term.onData(data => { ptyInput(instance.ptyId, data) })

// PTY 输出 → Terminal
onPtyOutput(({ id, data }) => { instance.term.write(data) })

// resize 同步
term.onResize(({ cols, rows }) => { ptyResize(instance.ptyId, cols, rows) })
```

### Ctrl+V 粘贴处理

```typescript
term.attachCustomKeyEventHandler((event: KeyboardEvent) => {
  if (event.ctrlKey && event.key === 'v') {
    event.preventDefault()
    readText().then(text => { if (text) term.paste(text) })
    return false
  }
  return true
})
```

## 终端缩放与布局刷新

### 三层架构

```
容器尺寸变化（flexbox / 窗口缩放）
  → ResizeObserver 检测 .xterm-container 尺寸变化
    → fitCurrentTerminal()（debounce 50ms，仅 trailing）
      → requestAnimationFrame → FitAddon.fit()
        → xterm.js 计算新的 cols/rows
          → term.onResize → ptyResize(ptyId, cols, rows)
```

### 核心：fitCurrentTerminal（debounce）

```typescript
const fitCurrentTerminal = debounce(() => {
  if (isMinimized || !currentDisplayTabId.value) return
  const instance = terminalInstances.get(currentDisplayTabId.value)
  if (instance) {
    requestAnimationFrame(() => instance.fitAddon.fit())
  }
}, 50) // 仅 trailing，频繁调用时只有最后一次生效
```

- **防抖**：侧边栏 CSS transition（250ms）期间连续触发只生效一次
- **最小化守卫**：`isMinimized` 为 true 时跳过，避免无意义 fit
- **仅当前 tab**：`fitCurrentTerminal` 只处理活跃 tab

### 10 个触发源

| # | 触发源 | fit 路径 | rAF | 范围 |
|---|--------|---------|-----|------|
| 1 | **ResizeObserver** | `fitCurrentTerminal()` (debounced) | 是 | 仅当前 tab |
| 2 | **侧边栏开关** | 经由 ResizeObserver（flex 布局重算） | 是 | 仅当前 tab |
| 3 | **Tab 切换** | 直接 `fitAddon.fit()` | 是 | 目标 tab |
| 4 | **字体大小改变** | watcher 直接 `fitAddon.fit()` | 否（同步） | **所有实例** |
| 5 | **视图可见性恢复** | `fitCurrentTerminal()` via nextTick | 是 | 仅当前 tab |
| 6 | **窗口缩放/最大化/半屏** | 经由 ResizeObserver | 是 | 仅当前 tab |
| 7 | **新建 Tab** | `fitAddon.fit()` after `term.open()` | 是 | 新 tab |
| 8 | **重启 Tab** | `fitAddon.fit()` after `term.open()` | 是 | 新 tab |
| 9 | **Vue ref 回调** | `fitAddon.fit()` | 是 | 对应 tab |
| 10 | **同 tab 重选** | `fitCurrentTerminal()` | 是 | 当前 tab |

### 最小化/恢复处理

```typescript
win.onResized(async () => {
  const minimized = await win.isMinimized()
  if (isMinimized && !minimized) {
    // 从最小化恢复
    isMinimized = false
    await nextTick()
    instance.fitAddon.fit()
    instance.term.refresh(0, instance.term.rows - 1) // 刷新渲染
    instance.term.scrollToBottom()                     // 滚动到底部
  } else {
    isMinimized = minimized
  }
})
```

恢复时三步操作保证布局无变化：`fit()` 重算尺寸 → `refresh()` 刷新脏区域 → `scrollToBottom()` 保持滚动位置。

### PTY 初始尺寸

`startTab` / `restartTab` 以 `cols: 80, rows: 24` 创建 PTY，随后 `fitAddon.fit()` 修正为实际容器尺寸。存在短暂窗口，PTY 可能在 resize 到达前输出内容。

## 快捷键处理机制

**应用级快捷键由 `useAppShortcuts.ts` 通过 DOM `keydown` capturing phase 统一处理；终端快捷键由 xterm.js + PTY 原生处理。**

- 应用快捷键：`window.addEventListener('keydown', handler, true)` → capturing phase 拦截 → 匹配后 `preventDefault` + `stopPropagation`
- 终端快捷键：xterm.js 通过 `onData` 发送到 PTY，由 Claude CLI 处理
- 终端视图可见性检查：`document.querySelector('[data-terminal-view]').checkVisibility()`

### 应用级快捷键

| 快捷键 | 功能 |
|--------|------|
| Ctrl+, | 打开设置 |
| Ctrl+Shift+N | 新建应用实例 |
| Ctrl+Shift+←/→ | 窗口左移/右移半屏 |
| Ctrl+Shift+R | 重启应用 |
| Ctrl+Shift+H | 回到项目列表 |
| Ctrl+=/- | 增大/减小字体 |
| Ctrl+0 | 重置字体 |
| Alt+N | 新建会话（终端可见时） |
| Alt+R | 重启会话（终端可见时） |
| Alt+↑/↓ | 切换标签（终端可见时） |

详细快捷键架构 → [docs/interaction.md](interaction.md)

## 进程生命周期

```
启动 → 环境检查 → PTY spawn → Claude CLI 运行
                              ↓
                       用户交互（双向数据流）
                              ↓
关闭窗口 → kill_all() → PTY 进程清理 → Claude CLI 退出
```
