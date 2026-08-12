# 交互规则

## 核心原则

**终端输入直接发送到 PTY，由 Claude CLI 处理；应用级快捷键由 `useAppShortcuts.ts` 通过 DOM `keydown` capturing phase 统一处理。**

```
用户按键 → window keydown (capturing phase)
                ↓
       匹配应用快捷键？
       ├─ 是 → preventDefault + stopPropagation → 执行应用操作
       └─ 否 → 正常传递到 xterm.js.onData → PTY → Claude CLI
```

## 快捷键处理架构

### 架构：DOM capturing phase → useAppShortcuts.ts

所有应用快捷键在 `src/composables/useAppShortcuts.ts` 中注册，通过 `window.addEventListener('keydown', handler, true)` 在 capturing phase 拦截：

```typescript
// useAppShortcuts.ts
function handleGlobalKeydown(e: KeyboardEvent) {
  const mod = e.ctrlKey
  // 匹配 → e.preventDefault() + e.stopPropagation()
  // 不匹配 → 传递到 xterm.js → PTY → Claude CLI
}

function setupShortcutListeners(): (() => void)[] {
  window.addEventListener('keydown', handleGlobalKeydown, true)
  return [() => window.removeEventListener('keydown', handleGlobalKeydown, true)]
}
```

### 终端视图可见性检查

部分快捷键（Alt+N/R、Alt+↑↓、Ctrl+Shift+H）仅在终端视图可见时生效：

```typescript
function isTerminalVisible(): boolean {
  const terminalView = document.querySelector('[data-terminal-view]')
  return terminalView !== null && terminalView.checkVisibility()
}
```

### 三种输入场景

| 场景 | 焦点位置 | 应用快捷键 | 终端输入 |
|------|---------|-----------|----------|
| **1. 终端聚焦** | xterm.js | DOM capturing → 执行 | xterm.js → PTY |
| **2. 标题栏点击** | 窗口框架 | DOM capturing → 执行（正常工作） | 无法输入 |
| **3. 窗口不在前台** | 其他应用 | OS 不派发 keydown → 不触发 | 正常 |

## 应用级快捷键

所有应用级快捷键在 `src/composables/useAppShortcuts.ts` 中定义：

| 快捷键 | 功能 | 作用域 |
|--------|------|--------|
| Ctrl+, | 打开设置 | 全局 |
| Ctrl+Shift+N | 新建应用实例 | 全局 |
| Ctrl+Shift+← | 窗口左移半屏 | 全局 |
| Ctrl+Shift+→ | 窗口右移半屏 | 全局 |
| Ctrl+Shift+R | 重启应用 | 全局 |
| Ctrl+Shift+H | 回到项目列表 | 全局 |
| Ctrl+= | 增大字体 | 全局 |
| Ctrl+- | 缩小字体 | 全局 |
| Ctrl+0 | 重置字体 | 全局 |
| Alt+N | 新建会话 | 终端可见时 |
| Alt+R | 重启会话 | 终端可见时 |
| Alt+↑ | 上一个标签 | 终端可见时 |
| Alt+↓ | 下一个标签 | 终端可见时 |

**Ctrl++ 特殊处理**：物理键盘上 `Ctrl++` 实际是 `Ctrl+Shift+=`，代码中 `e.key === '='` 匹配两种情况。

## 终端快捷键（Claude CLI 处理）

终端内的快捷键由 xterm.js 原生处理，通过 `onData` 发送到 PTY：

| 快捷键 | 功能 | 由谁处理 |
|--------|------|----------|
| Ctrl+C | 取消输入/生成 | xterm.js → PTY → Claude CLI |
| Ctrl+D | 退出 Claude Code | xterm.js → PTY → Claude CLI |
| Ctrl+L | 清屏 | xterm.js → PTY → Claude CLI |
| Ctrl+R | 反向搜索历史 | xterm.js → PTY → Claude CLI |
| Ctrl+B | 后台运行任务 | xterm.js → PTY → Claude CLI |
| Ctrl+W | 删除前一个单词 | xterm.js → `\x17` → PTY |
| Alt+P | 切换模型 | xterm.js → PTY → Claude CLI |
| Alt+T | 扩展思考 | xterm.js → PTY → Claude CLI |
| Ctrl+A/E | 行首/行尾 | xterm.js → PTY → Claude CLI |
| Ctrl+K/U | 删除到行尾/行首 | xterm.js → PTY → Claude CLI |
| Cmd+Z (macOS) / Ctrl+Z (Win/Linux) | Undo（readline 原生） | xterm.js 拦截 → 注入 `\x1f` → PTY → Claude CLI |

### Ctrl+W 处理

Tauri 无特殊绑定，xterm.js 原生处理：用户按 Ctrl+W → `term.onData('\x17')` → PTY → Claude CLI readline 删除前一个单词。无需额外代码。

### Ctrl+V 粘贴处理

```typescript
// src/components/XTermTerminal.vue
term.attachCustomKeyEventHandler((event: KeyboardEvent) => {
  if (event.ctrlKey && event.key === 'v') {
    event.preventDefault()
    readText().then(text => { if (text) term.paste(text) })
    return false
  }
  return true
})
```

### Undo 处理（Cmd+Z / Ctrl+Z）

Claude CLI 原生只把 `Ctrl+_`（即 `Ctrl+Shift+-`）绑给 readline undo，对用户不直观；而 `Ctrl+Z` 在 Unix 上是 SIGTSTP 挂起进程、`Cmd+Z` 在 macOS 终端里到不了 CLI——都没用作 undo。CC-Box 在 `attachCustomKeyEventHandler` 里拦截并改写：

| 平台 | 用户按 | 实际注入 PTY | 效果 |
|------|--------|-------------|------|
| macOS | `Cmd+Z` | `\x1f`（Ctrl+_） | readline undo |
| Windows / Linux | `Ctrl+Z` | `\x1f`（Ctrl+_） | readline undo |

**关键点**：必须 `return false` 阻止 xterm 默认行为，否则 Win/Linux 上 `\x1a` 会进入 PTY 触发 SIGTSTP 暂停 Claude 进程。GUI 层 Suspend 无意义（用户没有 shell 可以 `fg` 回来），屏蔽它换 undo 是净收益。

`Shift` 修饰（`Cmd+Shift+Z` / `Ctrl+Shift+Z`）当前**不命中**——预留给未来 redo。readline 本身没有 redo 接口，未来若要支持需要 CC-Box 自己维护 onData 输入镜像，复杂度高，暂不实现。

```typescript
// src/components/XTermTerminal.vue
if (isUndoShortcut(event)) {
  event.preventDefault()
  ptyInput(instance.ptyId, READLINE_UNDO)  // '\x1f'
  return false
}
```

判断逻辑抽到 `src/utils/keyboard.ts` 的 `isUndoShortcut(event, isMac)`，平台感知 + 单测覆盖。

## 视图切换

| 场景 | 触发 |
|------|------|
| 启动无收藏 | → WelcomeView |
| 启动有收藏 | → ProjectSelectView |
| 选择项目 | → TerminalView |
| 点击返回 | → ProjectSelectView |
| Ctrl+Shift+H | 终端 ↔ 项目列表 |

## 鼠标交互

- **文本选择**：xterm.js 原生支持
- **链接点击**：WebLinksAddon 处理
- **复制粘贴**：Ctrl+C/V（需聚焦终端）
- **侧边栏**：点击外部区域关闭侧边栏

## GUI 增强边界

| 增强 | 做 | 不做 |
|------|----|------|
| 终端主题 | 浅色主题 + CSS 变量 | AI 补全 |
| 多终端 | 标签切换 + 状态指示灯 | 复杂布局 |
| 会话管理 | 创建/切换/重命名/恢复 | 导出文件 |
| 信息面板 | MCP/Skills/Agents/Plugins 只读展示 | 编辑配置 |
| 搜索 | SearchAddon | 高级过滤 |
| 自动更新 | GitHub Releases 检测 + 下载安装 | 后台静默更新 |
