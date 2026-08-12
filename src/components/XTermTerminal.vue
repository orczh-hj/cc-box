<template>
  <div ref="containerRef" class="xterm-container" :class="{ 'drag-over': isDragOver }">
    <!-- 动态渲染每个 Tab 的终端容器 -->
    <div
      v-for="[tabId] in terminalInstances"
      :key="tabId"
      :ref="(el: Element | ComponentPublicInstance | null) => setTerminalEl(tabId, el as HTMLElement | null)"
      :data-tab="tabId"
      class="terminal-wrapper"
      :class="{ active: tabId === currentDisplayTabId }"
    ></div>
  </div>
</template>

<script setup lang="ts">
import { ref, reactive, watch, onMounted, onUnmounted, nextTick, type ComponentPublicInstance } from 'vue'
import { Terminal } from '@xterm/xterm'
import { FitAddon } from '@xterm/addon-fit'
import { WebglAddon } from '@xterm/addon-webgl'
import { Unicode11Addon } from '@xterm/addon-unicode11'
import { debounce } from 'lodash-es'
import '@xterm/xterm/css/xterm.css'
import { useAppStore } from '@/stores/app'
import { useSessionStore } from '@/stores/session'
import { useHookStore } from '@/stores/hook'
import { isMac } from '@/utils/platform'
import { isUndoShortcut, READLINE_UNDO } from '@/utils/keyboard'
import {
  ptySpawn,
  ptyInput,
  ptyResize,
  ptyKill,
  onPtyOutput,
  onPtyExit,
  logMessage,
} from '@/api/tauri'
import { registerTerminalCommand } from '@/composables/useTerminalCommand'
import { safeDispose } from '@/utils/dispose'
import { relativizePath } from '@/utils/path'
import { readText, writeText } from '@tauri-apps/plugin-clipboard-manager'
import { getCurrentWindow } from '@tauri-apps/api/window'
import { getCurrentWebview } from '@tauri-apps/api/webview'

const props = defineProps<{
  fontSize?: number
}>()

const emit = defineEmits<{
  ptyStarted: [tabId: string, ptyId: string]
}>()

const appStore = useAppStore()
const sessionStore = useSessionStore()
const hookStore = useHookStore()
const containerRef = ref<HTMLElement>()
const isDragOver = ref(false)

// 等待 DOM 元素可用
async function waitForElement(tabId: string, timeout = 10000): Promise<HTMLElement | null> {
  const start = Date.now()
  while (Date.now() - start < timeout) {
    let el = terminalEls.get(tabId)
    if (el) return el

    el = containerRef.value?.querySelector(
      `.terminal-wrapper[data-tab="${tabId}"]`
    ) as HTMLElement
    if (el) {
      terminalEls.set(tabId, el)
      return el
    }

    await new Promise(r => requestAnimationFrame(r))
  }
  return null
}

// 每个 Tab 独立的 Terminal 实例（key 为 tabId）
const terminalInstances = reactive(new Map<string, {
  term: Terminal
  fitAddon: FitAddon
  ptyId: string
}>())

// Terminal DOM 元素引用
const terminalEls = reactive(new Map<string, HTMLElement | null>())

// 当前显示的 Tab ID
const currentDisplayTabId = ref<string | null>(null)

// 是否正在启动 PTY（防止并发）
const isPtyStarting = ref<boolean>(false)

// macOS 原生 Copy 事件：Tauri MenuBuilder 注册了 Copy 菜单项后，
// Cmd+C 会派发 copy 事件到 WebView，此处将 xterm 选中文本写入剪贴板
function handleNativeCopy(e: ClipboardEvent) {
  const tabId = currentDisplayTabId.value
  if (!tabId) return
  const instance = terminalInstances.get(tabId)
  if (!instance) return
  const selection = instance.term.getSelection()
  if (selection) {
    e.preventDefault()
    writeText(selection).catch(() => {})
  }
}

// Unlisten functions
let unlistenPtyOutput: (() => void) | null = null
let unlistenPtyExit: (() => void) | null = null
let unlistenDragDrop: (() => void) | null = null
let unlistenWindowResized: (() => void) | null = null

// ResizeObserver
let resizeObserver: ResizeObserver | null = null

// 窗口最小化状态（最小化期间跳过 fit，恢复后主动刷新）
let isMinimized = false

// 浅色终端主题
const lightTheme = {
  background: '#f8f9fa',
  foreground: '#1a1a2e',
  cursor: '#6c5ce7',
  cursorAccent: '#f8f9fa',
  selectionBackground: '#ede9fe',
  selectionForeground: '#1a1a2e',
  black: '#1a1a2e',
  red: '#e74c3c',
  green: '#27ae60',
  yellow: '#f39c12',
  blue: '#3498db',
  magenta: '#9b59b6',
  cyan: '#1abc9c',
  white: '#ecf0f1',
  brightBlack: '#2c3e50',
  brightRed: '#e74c3c',
  brightGreen: '#27ae60',
  brightYellow: '#f39c12',
  brightBlue: '#3498db',
  brightMagenta: '#9b59b6',
  brightCyan: '#1abc9c',
  brightWhite: '#ffffff'
}

// 暗色终端主题
const darkTheme = {
  background: '#1e1e1e',
  foreground: '#d4d4d4',
  cursor: '#f0d4a8',
  cursorAccent: '#1e1e1e',
  selectionBackground: '#4a7aad40',
  selectionForeground: '#d4d4d4',
  black: '#1c1a17',
  red: '#e8705a',
  green: '#5dad8e',
  yellow: '#f0b460',
  blue: '#4a7aad',
  magenta: '#b8956a',
  cyan: '#5dad8e',
  white: '#c4c0b8',
  brightBlack: '#3a3734',
  brightRed: '#f0886e',
  brightGreen: '#7abd9e',
  brightYellow: '#f8ca80',
  brightBlue: '#6a9acd',
  brightMagenta: '#d0a87e',
  brightCyan: '#7abd9e',
  brightWhite: '#f8f6f3'
}

// 设置 Terminal DOM 元素引用
function setTerminalEl(tabId: string, el: HTMLElement | null) {
  if (el) {
    terminalEls.set(tabId, el)
    const instance = terminalInstances.get(tabId)
    if (instance && !instance.term.element) {
      instance.term.open(el)
      loadRendererAddons(instance.term)
      if (tabId === currentDisplayTabId.value) {
        requestAnimationFrame(() => instance.fitAddon.fit())
      }
    }
  }
}

// Fit 当前显示的终端（防抖，频繁调用时只有最后一次生效，最小化期间跳过）
const fitCurrentTerminal = debounce(() => {
  if (isMinimized || !currentDisplayTabId.value) return
  const instance = terminalInstances.get(currentDisplayTabId.value)
  if (instance) {
    requestAnimationFrame(() => instance.fitAddon.fit())
  }
}, 50)

// 按平台选择字体：CJK 用等宽字体（Microsoft YaHei / Noto Sans CJK），
// emoji 在主字体中缺失时回退到系统 emoji 字体。把 emoji 字体放在 monospace 前，
// 确保渲染层能找到 emoji 字形（实际宽度由 Unicode 11 wcwidth 决定，与字体回退无关）
function pickFontFamily(): string {
  if (isMac) {
    return '"Cascadia Code", "Fira Code", "JetBrains Mono", Consolas, "Apple Color Emoji", monospace'
  }
  return '"Cascadia Code", "Fira Code", "JetBrains Mono", Consolas, "Microsoft YaHei", "Noto Sans CJK SC", "Segoe UI Emoji", monospace'
}

// 在 term.open(el) 之后加载 Unicode 11 + WebGL addon
//
// @xterm/addon-webgl@0.19.0 / 0.20.0-beta 在长会话 + 多 Tab 共享 atlas 下会出现
// glyph atlas corruption(xtermjs/xterm.js#6038),上游尚未修复。
//
// 本项目的应对:每次切到 Tab 时主动 reload 该 Tab 的 WebGL addon(见 reloadWebgl)。
// reload 是唯一能消除已发生 corruption 的方式;放在「Tab 切入」时机执行,
// 切换动画遮盖了 reload 的几十毫秒重建,用户无感。
//
// 详见 docs/webgl-corruption-fix.md
function loadRendererAddons(term: Terminal) {
  try {
    const unicode11 = new Unicode11Addon()
    term.loadAddon(unicode11)
    term.unicode.activeVersion = '11'
  } catch (err) {
    console.warn('[XTerm] Unicode 11 addon unavailable, fallback to default:', err)
  }

  attachWebgl(term)
}

// 跟踪每个 Terminal 的 WebGL addon 实例(用于 reload)
const webglAddons = new WeakMap<Terminal, WebglAddon>()

function attachWebgl(term: Terminal) {
  try {
    const webglAddon = new WebglAddon()
    webglAddon.onContextLoss(() => {
      try { webglAddon.dispose() } catch { /* 已 dispose */ }
      webglAddons.delete(term)
      setTimeout(() => attachWebgl(term), 0)
    })
    term.loadAddon(webglAddon)
    webglAddons.set(term, webglAddon)
  } catch (err) {
    console.warn('[XTerm] WebGL init failed, fallback to DOM renderer:', err)
  }
}

/**
 * Reload Terminal 的 WebGL addon。
 * 用途:清除 glyph atlas corruption(详见 docs/webgl-corruption-fix.md)。
 *
 * 副作用:reload 时会触发 atlas page merge,可能短暂污染共享 atlas 的其他 Tab。
 * 因此只在「Tab 即将成为活跃 Tab」时调用 —— 此时其他 Tab 处于不可见状态,
 * 即使被污染用户也看不见。
 */
function reloadWebgl(term: Terminal) {
  const old = webglAddons.get(term)
  if (old) {
    try { old.dispose() } catch { /* ignore */ }
    webglAddons.delete(term)
  }
  attachWebgl(term)
}

// 统一清理 terminal
async function disposeTerminal(term: Terminal, context: string) {
  webglAddons.delete(term)
  await safeDispose(term, context)
}

// 创建新的 Terminal 实例
function createTerminal(tabId: string): Terminal {
  const term = new Terminal({
    fontFamily: pickFontFamily(),
    fontSize: props.fontSize ?? 12,
    lineHeight: 1.2,
    cursorBlink: true,
    cursorStyle: 'bar',
    theme: appStore.theme === 'dark' ? darkTheme : lightTheme,
    allowProposedApi: true,
    macOptionIsMeta: true,
    scrollback: 10000,
  })

  const fitAddon = new FitAddon()
  term.loadAddon(fitAddon)

  // 用户输入 → 发送到对应的 PTY
  term.onData(data => {
    const instance = terminalInstances.get(tabId)
    if (instance) {
      ptyInput(instance.ptyId, data)

      // Escape 按键：Claude 的 Stop hook 不在用户中断时触发，立即清除 working
      if (data === '\x1b') {
        const tab = sessionStore.tabs.get(tabId)
        if (tab?.working) tab.working = false
      }
    }
  })

  // 终端尺寸变化 → resize 对应的 PTY
  term.onResize(({ cols, rows }) => {
    const instance = terminalInstances.get(tabId)
    if (instance) {
      ptyResize(instance.ptyId, cols, rows)
    }
  })

  // 复制粘贴处理
  term.attachCustomKeyEventHandler((event: KeyboardEvent) => {
    if (event.type !== 'keydown') return true

    // Cmd+C (macOS) 复制选中内容
    if (event.metaKey && !event.ctrlKey && event.key === 'c') {
      const selection = term.getSelection()
      if (selection) {
        event.preventDefault()
        writeText(selection).catch(() => {})
        return false
      }
      return true
    }

    // Ctrl+C 复制（有选中）或 SIGINT（无选中）
    if (event.ctrlKey && !event.metaKey && event.key === 'c' && !event.shiftKey) {
      const selection = term.getSelection()
      if (selection) {
        event.preventDefault()
        writeText(selection).catch(() => {})
        return false
      }
      return true
    }

    // Ctrl+Shift+C 强制复制
    if (event.ctrlKey && event.shiftKey && event.key === 'C') {
      event.preventDefault()
      const selection = term.getSelection()
      if (selection) {
        writeText(selection).catch(() => {})
      }
      return false
    }

    // Ctrl+V / Cmd+V 粘贴
    if ((event.ctrlKey || event.metaKey) && event.key === 'v') {
      event.preventDefault()
      readText().then(text => {
        if (text) term.paste(text)
      }).catch(() => {})
      return false
    }

    // Shift+Enter => 插入换行（模拟 \ + Enter）
    if (event.shiftKey && event.key === 'Enter') {
      event.preventDefault()
      const instance = terminalInstances.get(tabId)
      if (instance) {
        ptyInput(instance.ptyId, '\\\r')
      }
      return false
    }

    // Undo: macOS Cmd+Z / Win+Linux Ctrl+Z → 注入 readline 原生 undo（Ctrl+_ = 0x1F）
    // 必须拦截原始字节：Win/Linux 上 \x1a 会触发 SIGTSTP 暂停 Claude 进程
    if (isUndoShortcut(event)) {
      event.preventDefault()
      const instance = terminalInstances.get(tabId)
      if (instance) {
        ptyInput(instance.ptyId, READLINE_UNDO)
      }
      return false
    }

    return true
  })

  return term
}

onMounted(async () => {
  await setupEventListeners()
  window.addEventListener('copy', handleNativeCopy)

  if (containerRef.value) {
    resizeObserver = new ResizeObserver(() => fitCurrentTerminal())
    resizeObserver.observe(containerRef.value)
  }

  // 监听窗口 resize → 追踪最小化状态，恢复时主动刷新
  const win = getCurrentWindow()
  unlistenWindowResized = await win.onResized(async () => {
    const minimized = await win.isMinimized()
    if (isMinimized && !minimized) {
      // 从最小化恢复 → fit + 刷新渲染 + 滚动到底部
      isMinimized = false
      const tabId = currentDisplayTabId.value
      if (tabId) {
        const instance = terminalInstances.get(tabId)
        if (instance) {
          await nextTick()
          instance.fitAddon.fit()
          instance.term.refresh(0, instance.term.rows - 1)
          instance.term.scrollToBottom()
        }
      }
    } else {
      isMinimized = minimized
    }
  })

  registerTerminalCommand(sendText)
})

// 设置事件监听器
async function setupEventListeners() {
  // 文件拖放 → 将路径输入终端
  // 项目内文件转换为相对路径，便于 Claude 直接引用
  unlistenDragDrop = await getCurrentWebview().onDragDropEvent((event) => {
    if (event.payload.type === 'drop') {
      isDragOver.value = false
      const paths = event.payload.paths
      if (paths.length > 0) {
        const projectPath = sessionStore.tabs.get(currentDisplayTabId.value ?? '')?.projectPath ?? ''
        const text = paths
          .map(p => relativizePath(p, projectPath))
          .map(p => p.includes(' ') ? `"${p}"` : p)
          .join(' ')
        sendText(text)
      }
    } else if (event.payload.type === 'enter' || event.payload.type === 'over') {
      isDragOver.value = true
    } else {
      isDragOver.value = false
    }
  })

  // PTY 输出 → 写入对应 Tab 的 Terminal
  unlistenPtyOutput = await onPtyOutput(({ id, data }) => {
    for (const [_tabId, instance] of terminalInstances) {
      if (instance.ptyId === id) {
        instance.term.write(data)
        break
      }
    }
  })

  // PTY 退出 → 更新 Tab 状态（不删除 Tab）
  unlistenPtyExit = await onPtyExit(({ id }) => {
    for (const [tabId, instance] of terminalInstances) {
      if (instance.ptyId === id) {
        // 更新 store（Tab 保留，状态变 stopped）
        sessionStore.handlePtyExit(id)

        // 清理 hook 状态（防止残留旧数据）
        hookStore.clearSession(id)

        // 销毁 Terminal 实例（释放资源）
        void disposeTerminal(instance.term, `onPtyExit(tabId=${tabId})`)
        terminalInstances.delete(tabId)
        terminalEls.delete(tabId)
        break
      }
    }
  })
}

// 监听 fontSize 变化
watch(() => props.fontSize, (newSize) => {
  if (newSize) {
    for (const instance of terminalInstances.values()) {
      instance.term.options.fontSize = newSize
      instance.fitAddon.fit()
    }
  }
})

// 监听主题变化，更新所有终端实例
watch(() => appStore.theme, (newTheme) => {
  const themeConfig = newTheme === 'dark' ? darkTheme : lightTheme
  for (const instance of terminalInstances.values()) {
    instance.term.options.theme = themeConfig
  }
})

// 监听活跃 Tab 变化 → 切换显示
watch(() => sessionStore.activeTabId, async (newTabId, oldTabId) => {
  if (!newTabId) return

  if (newTabId === oldTabId) {
    fitCurrentTerminal()
    return
  }

  if (isPtyStarting.value) return

  currentDisplayTabId.value = newTabId

  await nextTick()

  const existingInstance = terminalInstances.get(newTabId)

  if (existingInstance) {
    // ⚡ 切到的 Tab 主动 reload WebGL,清除可能的 atlas corruption
    // 详见 docs/webgl-corruption-fix.md
    reloadWebgl(existingInstance.term)

    const buf = existingInstance.term.buffer.active
    existingInstance.term.refresh(0, Math.max(buf.length - 1, 0))
    requestAnimationFrame(() => existingInstance.fitAddon.fit())
  } else {
    const tab = sessionStore.tabs.get(newTabId)
    if (!tab) return

    if (tab.ptyId && tab.status === 'running') {
      // PTY 在运行但没有 Terminal 实例 → 创建
      await createTerminalForTab(newTabId, tab.ptyId)
    } else if (tab.status === 'stopped') {
      // 已停止的 Tab：不自动启动，等用户操作
    }
  }
})

// 为已有 PTY 的 Tab 创建 Terminal 实例
async function createTerminalForTab(tabId: string, ptyId: string) {
  if (terminalInstances.has(tabId)) return

  const term = createTerminal(tabId)
  const fitAddon = new FitAddon()
  term.loadAddon(fitAddon)

  terminalInstances.set(tabId, { term, fitAddon, ptyId })

  const el = await waitForElement(tabId)
  if (el) {
    // display:none 时 xterm 无法获取尺寸，临时显示
    const isActive = tabId === currentDisplayTabId.value
    if (!isActive) el.style.display = 'block'

    term.open(el)
    loadRendererAddons(term)

    if (!isActive) {
      requestAnimationFrame(() => {
        fitAddon.fit()
        el.style.display = ''
      })
    } else {
      requestAnimationFrame(() => fitAddon.fit())
    }
  }
}

// ==================== Tab 操作（外部调用） ====================

/**
 * 启动 Tab 的 PTY
 * 由 TerminalView 调用，传入已创建好的 tabId
 */
async function startTab(tabId: string) {
  const tab = sessionStore.tabs.get(tabId)
  if (!tab) {
    logMessage('warn', `startTab: tab not found, tabId=${tabId}`)
    return
  }
  if (isPtyStarting.value) {
    logMessage('warn', `startTab: blocked by isPtyStarting, tabId=${tabId}`)
    return
  }

  // 已有运行中的 PTY
  if (tab.ptyId && tab.status === 'running') {
    if (!terminalInstances.has(tabId)) {
      await createTerminalForTab(tabId, tab.ptyId)
    }
    return
  }

  isPtyStarting.value = true

  try {
    const args = buildClaudeArgs(tab)
    const cwd = tab.projectPath

    const term = createTerminal(tabId)
    const fitAddon = new FitAddon()
    term.loadAddon(fitAddon)

    const info = await ptySpawn({
      cwd,
      cols: 80,
      rows: 24,
      type: 'claude',
      args,
    })

    if (info) {
      terminalInstances.set(tabId, { term, fitAddon, ptyId: info.id })
      sessionStore.setTabPty(tabId, info.id)
      sessionStore.setActiveTab(tabId)
      currentDisplayTabId.value = tabId

      const el = await waitForElement(tabId)
      if (el) {
        term.open(el)
        loadRendererAddons(term)
        requestAnimationFrame(() => fitAddon.fit())
      }

      emit('ptyStarted', tabId, info.id)
    }
  } catch (err) {
    console.error('[XTerm] startTab ERROR:', err)
    logMessage('error', `startTab failed, tabId=${tabId}: ${err}`)
  } finally {
    isPtyStarting.value = false
  }
}

/**
 * 重启 Tab（停止的 Tab 恢复运行）
 */
async function restartTab(tabId: string) {
  const tab = sessionStore.tabs.get(tabId)
  if (!tab || isPtyStarting.value) return

  isPtyStarting.value = true

  try {
    // 如果有旧 PTY 在运行，先 kill
    if (tab.ptyId) {
      try { await ptyKill(tab.ptyId) } catch {}
      hookStore.clearSession(tab.ptyId)
    }

    // 清理旧 Terminal 实例（dispose 失败不阻断重启流程，仍需清理 Map 引用）
    const oldInstance = terminalInstances.get(tabId)
    if (oldInstance) {
      await disposeTerminal(oldInstance.term, `restartTab(old term, tabId=${tabId})`)
      terminalInstances.delete(tabId)
      terminalEls.delete(tabId)
    }

    const args = buildClaudeArgs(tab)
    const cwd = tab.projectPath

    const term = createTerminal(tabId)
    const fitAddon = new FitAddon()
    term.loadAddon(fitAddon)

    const info = await ptySpawn({
      cwd,
      cols: 80,
      rows: 24,
      type: 'claude',
      args,
    })

    if (info) {
      terminalInstances.set(tabId, { term, fitAddon, ptyId: info.id })
      sessionStore.setTabPty(tabId, info.id)
      sessionStore.setActiveTab(tabId)
      currentDisplayTabId.value = tabId

      const el = await waitForElement(tabId)
      if (el) {
        term.open(el)
        loadRendererAddons(term)
        requestAnimationFrame(() => fitAddon.fit())
      }

      emit('ptyStarted', tabId, info.id)
    }

    appStore.resetClaudeOptions()
  } catch (err) {
    console.error('[XTerm] restartTab ERROR:', err)
    logMessage('error', `restartTab failed, tabId=${tabId}: ${err}`)
  } finally {
    isPtyStarting.value = false
  }
}

/**
 * 根据 Tab 状态构建 Claude CLI 参数
 */
function buildClaudeArgs(tab: { sessionId: string | null }): string[] {
  const args: string[] = []
  const opts = appStore.claudeOptions

  // 有 sessionId → --resume；无 → 新建会话（不带 --resume）
  if (tab.sessionId) {
    args.push('--resume', tab.sessionId)
  }

  if (opts.skipPermissions) args.push('--dangerously-skip-permissions')
  if (opts.customArgs) {
    const custom = opts.customArgs.trim().split(/\s+/).filter(Boolean)
    args.push(...custom)
  }

  return args
}

/**
 * 使用启动选项创建并启动 Tab（兼容旧的 startWithOptions 入口）
 */
async function startWithOptions(cwd: string, opts: {
  resume?: string
  skipPermissions?: boolean
  customArgs?: string
}) {
  if (isPtyStarting.value) return

  const tabId = sessionStore.createTab(cwd, {
    sessionId: opts.resume || undefined,
  })

  sessionStore.setActiveTab(tabId)
  await startTab(tabId)
  appStore.resetClaudeOptions()
}

/**
 * 新建会话（无参数模式）
 */
async function startNewSession(cwd: string) {
  if (isPtyStarting.value) {
    logMessage('warn', `startNewSession: blocked by isPtyStarting, cwd=${cwd}`)
    return
  }

  const tabId = sessionStore.createTab(cwd)
  sessionStore.setActiveTab(tabId)
  await startTab(tabId)
  appStore.resetClaudeOptions()
}

// 清理所有 PTY
async function cleanup() {
  for (const [tabId, instance] of terminalInstances.entries()) {
    await disposeTerminal(instance.term, `cleanup(tabId=${tabId})`)
  }
  terminalInstances.clear()
  terminalEls.clear()
  await sessionStore.cleanupAll()
}

// 向活跃终端发送文字并聚焦
function sendText(text: string) {
  const tabId = currentDisplayTabId.value
  if (!tabId) return
  const instance = terminalInstances.get(tabId)
  if (instance) {
    ptyInput(instance.ptyId, text)
    instance.term.focus()
  }
}

// 聚焦活跃终端
function focus() {
  const tabId = currentDisplayTabId.value
  if (!tabId) return
  const instance = terminalInstances.get(tabId)
  if (instance) instance.term.focus()
}

// 兼容：重启当前活跃 Tab
async function restartCurrentPty() {
  if (sessionStore.activeTabId) {
    await restartTab(sessionStore.activeTabId)
  }
}

onUnmounted(() => {
  fitCurrentTerminal.cancel()
  resizeObserver?.disconnect()
  window.removeEventListener('copy', handleNativeCopy)
  unlistenPtyOutput?.()
  unlistenPtyExit?.()
  unlistenDragDrop?.()
  unlistenWindowResized?.()
  for (const [tabId, instance] of terminalInstances.entries()) {
    void disposeTerminal(instance.term, `onUnmounted(tabId=${tabId})`)
  }
})

defineExpose({
  startTab,
  restartTab,
  startWithOptions,
  startNewSession,
  restartCurrentPty,
  cleanup,
  sendText,
  focus,
  fitCurrentTerminal,
})
</script>

<style scoped>
.xterm-container {
  width: 100%;
  height: 100%;
  box-sizing: border-box;
  background: var(--terminal-bg);
  border-radius: 8px;
  position: relative;
  overflow: hidden;
  transition: box-shadow 0.15s ease;
}

.xterm-container.drag-over {
  box-shadow: inset 0 0 0 2px var(--accent-gold);
}

.terminal-wrapper {
  position: absolute;
  top: 0;
  left: 0;
  width: 100%;
  height: 100%;
  padding: 0 12px;
  box-sizing: border-box;
  display: none;
}

.terminal-wrapper.active {
  display: block;
}

.terminal-wrapper :deep(.xterm) {
  height: 100%;
}

.terminal-wrapper :deep(.xterm-viewport) {
  border-radius: 4px;
}

.terminal-wrapper :deep(.xterm-viewport::-webkit-scrollbar) {
  width: 8px;
}

.terminal-wrapper :deep(.xterm-viewport::-webkit-scrollbar-thumb) {
  background: var(--border-dark);
  border-radius: 4px;
}

.terminal-wrapper :deep(.xterm-viewport::-webkit-scrollbar-track) {
  background: transparent;
}
</style>
