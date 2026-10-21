<template>
  <div
    ref="containerRef"
    class="xterm-container"
    :class="{ 'drag-over': isDragOver }"
    :style="termBgStyle"
    @touchstart.capture="handleTouchStartCapture"
    @touchmove.capture="handleTouchMoveCapture"
  >
    <!-- 动态渲染每个 Tab 的终端容器 -->
    <div
      v-for="[tabId] in terminalInstances"
      :key="tabId"
      :ref="(el: Element | ComponentPublicInstance | null) => setTerminalEl(tabId, el as HTMLElement | null)"
      :data-tab="tabId"
      class="terminal-wrapper"
      :class="{ active: tabId === currentDisplayTabId }"
    >
      <!-- 远程（手机）接管遮罩：显示层独占，数据层继续写入 -->
      <div v-if="isTabTakenOver(tabId)" class="taken-over-overlay">
        <div class="taken-over-card">
          <p class="taken-over-title">{{ t('remoteTakenOver') }}</p>
          <p class="taken-over-hint">{{ t('remoteTakenOverHint') }}</p>
          <button class="taken-over-btn" @click="handleClaim(tabId)">{{ t('remoteClaimBack') }}</button>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, reactive, computed, watch, onMounted, onUnmounted, nextTick, markRaw, type ComponentPublicInstance } from 'vue'
import { Terminal } from '@xterm/xterm'
import { FitAddon } from '@xterm/addon-fit'
import { WebglAddon } from '@xterm/addon-webgl'
import { Unicode11Addon } from '@xterm/addon-unicode11'
import { debounce } from 'lodash-es'
import '@xterm/xterm/css/xterm.css'
// Cascadia Code webfont：终端符号（box-drawing/箭头/spinner）跨端一致——
// 手机无雅黑/Consolas 等桌面字体，缺 webfont 时回退系统字体导致符号错位
import '@fontsource/cascadia-code/400.css'
import { useAppStore } from '@/stores/app'
import { useSessionStore } from '@/stores/session'
import { useI18n } from 'vue-i18n'
import { isMac } from '@/utils/platform'
import { isUndoShortcut, READLINE_UNDO } from '@/utils/keyboard'
import { attachImePositionSync } from '@/utils/ime'
import {
  ptySpawn,
  ptyInput,
  ptyResize,
  ptyKill,
  ptyClaim,
  onPtyOutput,
  onPtyExit,
  onSessionTakenOver,
  onSessionReclaimed,
  logMessage,
} from '@/api/tauri'
import { registerTerminalCommand } from '@/composables/useTerminalCommand'
import { useIsMobile } from '@/composables/useMediaQuery'
import { safeDispose } from '@/utils/dispose'
import { relativizePath } from '@/utils/path'
import { mapTerminalGlyphs } from '@/utils/terminal-glyphs'
import type { SnapshotData } from '@/utils/terminal-reflow'
import { copyText as platformCopyText, readClipboardText as platformReadText } from '@/api/platform'
import { transport } from '@/api/transport'
import { getCurrentWindow } from '@tauri-apps/api/window'
import { isTauriEnv } from '@/api/platform'
import { getCurrentWebview } from '@tauri-apps/api/webview'

const props = defineProps<{
  fontSize?: number
}>()

const emit = defineEmits<{
  ptyStarted: [tabId: string, ptyId: string]
}>()

const appStore = useAppStore()
const sessionStore = useSessionStore()
const isMobile = useIsMobile()

const { t } = useI18n()
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

// 每个 Tab 独立的 Terminal 实例（key 为 tabId）。
// 创建时 markRaw：Terminal 是高频访问对象（每输出块多次属性读写），
// 深 reactive Proxy 会让 xterm 内部全部走 trap——输出热路径显著变慢
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
    void platformCopyText(selection)
  }
}

// Unlisten functions
let unlistenPtyOutput: (() => void) | null = null
let unlistenTakenOver: (() => void) | null = null
let unlistenReclaimed: (() => void) | null = null
let unlistenPtyExit: (() => void) | null = null
let unlistenDragDrop: (() => void) | null = null
let unlistenWindowResized: (() => void) | null = null
let unlistenRemoteConn: (() => void) | null = null

// 浏览器远程模式：已 attach 的 ptyId 集合（attach = 接管输出流 + 控制权）。
// 重连后服务端订阅清空，标记失效需清空重 attach
const attachedPtyIds = new Set<string>()

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

// 终端背景色注入为 CSS 变量：xterm.css 给 .xterm-viewport 兜底 #000，
// 容器高度与行高不整除时底部露出的黑条由此而来；覆盖为主题背景色后缝隙不可见
const termBgStyle = computed(() => ({
  '--term-bg': appStore.theme === 'dark' ? darkTheme.background : lightTheme.background
}))

// 设置 Terminal DOM 元素引用
function setTerminalEl(tabId: string, el: HTMLElement | null) {
  if (el) {
    terminalEls.set(tabId, el)
    const instance = terminalInstances.get(tabId)
    if (instance && !instance.term.element) {
      instance.term.open(el)
      loadRendererAddons(instance.term)
      ensureBrowserAttached(tabId)
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

// 按平台选择字体：等宽主字体（Cascadia Code 已打包为 webfont，全端一致命中，
// 桌面/手机符号字形统一）+ 各端系统 CJK 回退（PingFang=iOS/macOS、雅黑=Windows、
// Noto CJK=Android）。emoji 字体放在 monospace 前，确保渲染层能找到 emoji 字形
// （实际宽度由 Unicode 11 wcwidth 决定，与字体回退无关）
function pickFontFamily(): string {
  if (isMac) {
    return '"Cascadia Code", "Fira Code", "JetBrains Mono", Consolas, "PingFang SC", "Apple Color Emoji", monospace'
  }
  return '"Cascadia Code", "Fira Code", "JetBrains Mono", Consolas, "PingFang SC", "Microsoft YaHei", "Noto Sans CJK SC", "Noto Sans SC", "Segoe UI Emoji", monospace'
}

/** 手机断点字号 = 桌面设置 × 0.9（手机物理宽度小，缩小字号使每行列数接近桌面） */
function effectiveFontSize(size: number | undefined): number {
  const base = size ?? 12
  return isMobile.value ? Math.max(6, Math.round(base * 0.9)) : base
}

// 在 term.open(el) 之后加载 Unicode 11 + WebGL addon
//
// glyph atlas corruption(xtermjs/xterm.js#6038)已由上游修复:
// PR #6042(共享 atlas 页合并后 renderer 状态过期)+ PR #6043(atlas 页数超容量),
// 包含在 @xterm/xterm@6.1.0-beta.303 / @xterm/addon-webgl@0.20.0-beta.299。
//
// 每次切到 Tab 时主动 reload 该 Tab 的 WebGL addon(见 reloadWebgl)——保留作兜底:
// beta 线仍可能出现回归,reload 是唯一能消除已发生 corruption 的方式;
// 放在「Tab 切入」时机执行,切换动画遮盖 reload 的几十毫秒重建,用户无感。
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
    fontSize: effectiveFontSize(props.fontSize),
    lineHeight: 1.2,
    cursorBlink: true,
    cursorStyle: 'bar',
    theme: appStore.theme === 'dark' ? darkTheme : lightTheme,
    allowProposedApi: true,
    macOptionIsMeta: true,
    scrollback: 10000,
    // 隐藏 xterm 6 自绘滚动条：cc-box 中滚动由 Claude CLI 内部实现，
    // 鼠标滚轮滚动 scrollback 不受影响（滚动条仅视觉隐藏）
    scrollbar: { showScrollbar: false },
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

  // 输入法候选框跟随光标：compositionstart 时把 textarea 定位到光标处一次。
  // 此回调仅做 dataset 幂等检查（挂接后零 DOM 操作），不干扰渲染管线
  term.onRender(() => attachImePositionSync(term))

  // 复制粘贴处理
  term.attachCustomKeyEventHandler((event: KeyboardEvent) => {
    if (event.type !== 'keydown') return true

    // Cmd+C (macOS) 复制选中内容
    if (event.metaKey && !event.ctrlKey && event.key === 'c') {
      const selection = term.getSelection()
      if (selection) {
        event.preventDefault()
        void platformCopyText(selection)
        return false
      }
      return true
    }

    // Ctrl+C 复制（有选中）或 SIGINT（无选中）
    if (event.ctrlKey && !event.metaKey && event.key === 'c' && !event.shiftKey) {
      const selection = term.getSelection()
      if (selection) {
        event.preventDefault()
        void platformCopyText(selection)
        return false
      }
      return true
    }

    // Ctrl+Shift+C 强制复制
    if (event.ctrlKey && event.shiftKey && event.key === 'C') {
      event.preventDefault()
      const selection = term.getSelection()
      if (selection) {
        void platformCopyText(selection)
      }
      return false
    }

    // Ctrl+V / Cmd+V 粘贴
    if ((event.ctrlKey || event.metaKey) && event.key === 'v') {
      event.preventDefault()
      platformReadText().then(text => {
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

  // webfont 迟到兜底：Cascadia Code woff2 加载完成晚于 term.open 时，
  // 清掉按 fallback 字体缓存的 glyph atlas 并重新测量/重绘
  document.fonts.ready.then(() => {
    for (const instance of terminalInstances.values()) {
      try {
        instance.term.clearTextureAtlas()
        instance.term.refresh(0, instance.term.rows - 1)
      } catch { /* 实例可能已销毁 */ }
    }
    fitCurrentTerminal()
  })

  if (containerRef.value) {
    let lastContainerHeight = 0
    resizeObserver = new ResizeObserver((entries) => {
      const h = entries[0]?.contentRect.height ?? 0
      // 容器高骤降（软键盘弹出特征）：立即 fit 并滚到底，保证光标/输入行可见
      if (lastContainerHeight - h > 120) {
        const instance = terminalInstances.get(currentDisplayTabId.value)
        if (instance) {
          requestAnimationFrame(() => {
            instance.fitAddon.fit()
            instance.term.scrollToBottom()
          })
          lastContainerHeight = h
          return
        }
      }
      lastContainerHeight = h
      fitCurrentTerminal()
    })
    resizeObserver.observe(containerRef.value)
  }

  // 监听窗口 resize → 追踪最小化状态，恢复时主动刷新（桌面专属）
  if (isTauriEnv) {
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
  }

  registerTerminalCommand(sendText)

  // 浏览器远程模式：重连后（remote-connected）订阅已失效——
  // 清 attach 标记并重新 attach 当前 Tab（会话列表拉取由 App.vue 常驻处理）
  if (!isTauriEnv) {
    unlistenRemoteConn = await transport.listen('remote-connected', () => {
      attachedPtyIds.clear()
      const tabId = currentDisplayTabId.value
      if (tabId) ensureBrowserAttached(tabId)
    })
  }
})

// 设置事件监听器
async function setupEventListeners() {
  // 文件拖放 → 将路径输入终端（桌面专属；浏览器可后续用 HTML5 drag events 补充）
  // 项目内文件转换为相对路径，便于 Claude 直接引用
  if (isTauriEnv) {
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
  }

  // PTY 输出 → 写入对应 Tab 的 Terminal
  unlistenPtyOutput = await onPtyOutput(({ id, data }) => {
    for (const [_tabId, instance] of terminalInstances) {
      if (instance.ptyId === id) {
        // 浏览器远程模式：移动端字体缺媒体控制符字形（⏵⏵/⏸ 渲染为 tofu），
        // 写入前兜底替换为等义通用符号；桌面端有系统字体回退，原样写入
        instance.term.write(isTauriEnv ? data : mapTerminalGlyphs(data))
        break
      }
    }
  })

  // 远程（手机）接管 → 该 Tab 显示遮罩（数据层继续写入，接管回来零缺失）
  unlistenTakenOver = await onSessionTakenOver(({ id }) => {
    const tab = sessionStore.getTabByPtyId(id)
    if (tab) tab.takenOver = true
  })

  // 远程归还（手机断连/主动 detach/桌面 claim）→ 解除遮罩并触发一次 fit 恢复尺寸
  unlistenReclaimed = await onSessionReclaimed(({ id }) => {
    const tab = sessionStore.getTabByPtyId(id)
    if (tab) tab.takenOver = false
    for (const [tabId, instance] of terminalInstances) {
      if (instance.ptyId === id && tabId === currentDisplayTabId.value) {
        instance.fitAddon.fit()
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

        // 销毁 Terminal 实例（释放资源）
        void disposeTerminal(instance.term, `onPtyExit(tabId=${tabId})`)
        terminalInstances.delete(tabId)
        terminalEls.delete(tabId)
        attachedPtyIds.delete(id)
        break
      }
    }
  })
}

// 监听 fontSize 变化
watch(() => props.fontSize, (newSize) => {
  if (newSize) {
    for (const instance of terminalInstances.values()) {
      instance.term.options.fontSize = effectiveFontSize(newSize)
      instance.fitAddon.fit()
    }
  }
})

/** 手机断点跨越（窗口缩放/旋转）时重算所有实例字号（×0.9 缩放） */
watch(isMobile, () => {
  for (const instance of terminalInstances.values()) {
    instance.term.options.fontSize = effectiveFontSize(props.fontSize)
    instance.fitAddon.fit()
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

  terminalInstances.set(tabId, { term: markRaw(term), fitAddon: markRaw(fitAddon), ptyId })

  const el = await waitForElement(tabId)
  if (el) {
    // display:none 时 xterm 无法获取尺寸，临时显示
    const isActive = tabId === currentDisplayTabId.value
    if (!isActive) el.style.display = 'block'

    term.open(el)
    loadRendererAddons(term)
    // 浏览器端 attach 输出流（刷新恢复/重新显示 Tab 场景；已 attach 时幂等跳过）
    ensureBrowserAttached(tabId)

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

  let term: Terminal | null = null
  let fitAddon: FitAddon | null = null

  try {
    const args = buildClaudeArgs(tab)
    const cwd = tab.projectPath

    term = createTerminal(tabId)
    fitAddon = new FitAddon()
    term.loadAddon(fitAddon)

    const info = await ptySpawn({
      cwd,
      cols: 80,
      rows: 24,
      type: 'claude',
      args,
      // 浏览器远程模式：结构化启动选项（Rust 侧按同一规则拼接 CLI 参数）；
      // resumeName 带上有意义的显示名（历史会话名/prompt 名），桌面建 Tab 防编码名
      resume: tab.sessionId ?? undefined,
      resumeName: tab.name && tab.name !== 'New Session' && tab.name !== tab.sessionId?.slice(0, 8)
        ? tab.name
        : undefined,
      skipPermissions: appStore.claudeOptions.skipPermissions || undefined,
      customArgs: appStore.claudeOptions.customArgs || undefined,
    })

    if (info) {
      // 浏览器端 spawnSession 内部已 create+attach，记录标记防重复 attach
      attachedPtyIds.add(info.id)
      terminalInstances.set(tabId, { term: markRaw(term), fitAddon: markRaw(fitAddon), ptyId: info.id })
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
    // spawn 失败（工作目录不存在/CLI 未检出/exec 失败）必须打开终端写入
    // 错误，否则表现为静默白屏（Mac 打开历史会话白屏的教训）
    if (term && fitAddon) await showSpawnError(tabId, term, fitAddon, err)
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

  let term: Terminal | null = null
  let fitAddon: FitAddon | null = null

  try {
    // 如果有旧 PTY 在运行，先 kill
    if (tab.ptyId) {
      try { await ptyKill(tab.ptyId) } catch {}
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

    term = createTerminal(tabId)
    fitAddon = new FitAddon()
    term.loadAddon(fitAddon)

    const info = await ptySpawn({
      cwd,
      cols: 80,
      rows: 24,
      type: 'claude',
      args,
      // 浏览器远程模式：结构化启动选项（与 buildClaudeArgs 同一规则）
      resume: tab.sessionId ?? undefined,
      skipPermissions: appStore.claudeOptions.skipPermissions || undefined,
      customArgs: appStore.claudeOptions.customArgs || undefined,
    })

    if (info) {
      // 浏览器端 spawnSession 内部已 create+attach，记录标记防重复 attach
      attachedPtyIds.add(info.id)
      terminalInstances.set(tabId, { term: markRaw(term), fitAddon: markRaw(fitAddon), ptyId: info.id })
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
    // 同 startTab：失败必须可见（写入终端），不允许静默白屏
    if (term && fitAddon) await showSpawnError(tabId, term, fitAddon, err)
  } finally {
    isPtyStarting.value = false
  }
}

/**
 * spawn 失败反馈：打开终端并写入错误（startTab/restartTab 的 catch 调用）。
 * 失败时 term 尚未 open，错误只进 console 的话终端区一片空白、无任何提示
 * （Mac 打开历史会话"白屏"的直接表象）——错误文本即 Rust spawn_pty_core
 * 的 Err 消息（如 "Working directory does not exist: ..."）
 */
async function showSpawnError(tabId: string, term: Terminal, fitAddon: FitAddon, err: unknown) {
  const el = await waitForElement(tabId)
  if (el && !term.element) {
    term.open(el)
    loadRendererAddons(term)
    requestAnimationFrame(() => fitAddon.fit())
  }
  term.write(`\r\n\x1b[31m[CC-Box] 启动 Claude CLI 失败: ${String(err)}\x1b[0m\r\n\r\n`)
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

// 聚焦活跃终端（唤起软键盘）：iOS/Android 对 xterm 内部 focus
// （preventScroll:true）不弹键盘——直接聚焦裸 textarea（工具栏
// 「键盘」按钮手势内调用，属于用户激活态，键盘可靠唤出）
function focus() {
  const instance = currentInstance()
  if (!instance) return
  const ta = instance.term.textarea
  if (ta) ta.focus()
  else instance.term.focus()
}

// 浏览器远程模式：确保 Tab 已 attach（输出流 + 控制权；后 attach 优先抢占，
// 从桌面/其他远程端接管回来即走此路径）。桌面 Tauri 模式 PTY 输出全局广播，无需 attach
function ensureBrowserAttached(tabId: string) {
  if (isTauriEnv) return
  const tab = sessionStore.tabs.get(tabId)
  if (!tab?.ptyId || tab.status !== 'running') return
  if (attachedPtyIds.has(tab.ptyId)) return
  const instance = terminalInstances.get(tabId)
  if (!instance) return
  attachedPtyIds.add(tab.ptyId)
  transport.sendClient({
    type: 'session.attach',
    id: tab.ptyId,
    cols: instance.term.cols,
    rows: instance.term.rows,
  })
}

// 浏览器远程模式：拉取服务端运行中的会话列表 → 重建 Tabs（刷新/新设备后的
// 恢复入口；用户点击 Tab 时 attach 接管，不自动抢当前控制权）
function refreshRemoteSessions() {
  if (isTauriEnv) return
  transport.sendClient({ type: 'sessions.list' })
}

// 当前显示终端实例（触摸/选择/键盘的统一取用）
function currentInstance() {
  const tabId = currentDisplayTabId.value
  return tabId ? terminalInstances.get(tabId) : undefined
}

// ===== 快照抓取（手机复制部分内容）=====
// 抓取当前 buffer 全量（scrollback + viewport）供复制阅读器反渲染展示；
// 终端不暂停，快照为静态视图，退出阅读器后重进可重抓最新

/** 快照行数上限：超出截头部（保留最新），防超大 scrollback 拖垮 DOM */
const MAX_SNAPSHOT_LINES = 5000

function captureSnapshot(): SnapshotData {
  const instance = currentInstance()
  if (!instance) return { lines: [], wrapped: [], cols: 80, truncated: false }
  const buf = instance.term.buffer.active
  const total = buf.length
  if (total === 0) return { lines: [], wrapped: [], cols: instance.term.cols, truncated: false }
  const start = Math.max(0, total - MAX_SNAPSHOT_LINES)
  const lines: string[] = []
  const wrapped: boolean[] = []
  for (let i = start; i < total; i++) {
    const line = buf.getLine(i)
    // trimRight：剥离行尾渲染填充空格（跨行拼接质量由 reflowSnapshot 保证）
    lines.push(line?.translateToString(true) ?? '')
    wrapped.push(line?.isWrapped ?? false)
  }
  return { lines, wrapped, cols: instance.term.cols, truncated: start > 0 }
}

/** 终端渲染区（.xterm-screen）矩形 */
function screenRect(): DOMRect | null {
  const instance = currentInstance()
  const el = instance?.term.element?.querySelector('.xterm-screen')
  return el?.getBoundingClientRect() ?? null
}

function cellHeight(): number {
  const r = screenRect()
  const instance = currentInstance()
  if (!r || !instance || instance.term.rows <= 0) return 20
  return r.height / instance.term.rows
}

// ===== 普通模式触摸：滚动自管（capture 阶段接管） =====
// stopPropagation 断开 xterm 内部 touch 处理（其 touchmove 在应用鼠标模式
// 下转序列时坐标异常，产生 "aN;NaNM" 垃圾输入——根因）。滚动的正确通道是
// 合成 wheel 派发给 xterm：CLI 开 mouse reporting 时 xterm 编码滚轮序列发给
// CLI（TUI 内部滚动）；未开时 xterm 滚 scrollback——两条路都通，坐标由我们
// 提供永不为 NaN。tap（位移 <10px）不 preventDefault，合成 click/mousedown
// 照常透传给 xterm（光标跳转等原生响应）
let touchStartY = 0
let touchLastY = 0
let touchScrolling = false
let wheelAccum = 0

/** 拖动量转为合成 wheel 派发给 xterm（deltaY 像素，触点为有效坐标） */
function dispatchWheelToXterm(deltaY: number, x: number, y: number) {
  const instance = currentInstance()
  const el = instance?.term.element
  if (!el) return
  el.dispatchEvent(new WheelEvent('wheel', {
    bubbles: true,
    cancelable: true,
    clientX: x,
    clientY: y,
    deltaX: 0,
    deltaY,
    deltaZ: 0,
    deltaMode: WheelEvent.DOM_DELTA_PIXEL,
  }))
}

function handleTouchStartCapture(e: TouchEvent) {
  if (!isMobile.value) return
  e.stopPropagation()
  touchStartY = touchLastY = e.touches[0].clientY
  touchScrolling = false
  wheelAccum = 0
}
function handleTouchMoveCapture(e: TouchEvent) {
  if (!isMobile.value) return
  e.stopPropagation()
  const t = e.touches[0]
  const y = t.clientY
  if (!touchScrolling && Math.abs(y - touchStartY) >= 10) {
    touchScrolling = true
    // 滚动手势：收起已弹出的软键盘（键盘不挡翻阅；高度回升只 fit 不滚底）
    const ta = currentInstance()?.term.textarea
    if (ta && document.activeElement === ta) ta.blur()
  }
  if (touchScrolling) {
    e.preventDefault()
    // 手指上滑（y 减小）→ 内容上移看新内容 → wheel deltaY 为正（下滚方向）
    const dy = touchLastY - y
    touchLastY = y
    wheelAccum += dy
    const ch = cellHeight()
    const steps = Math.trunc(wheelAccum / ch)
    if (steps !== 0) {
      wheelAccum -= steps * ch
      dispatchWheelToXterm(steps * ch, t.clientX, t.clientY)
    }
  }
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
  unlistenTakenOver?.()
  unlistenReclaimed?.()
  unlistenDragDrop?.()
  unlistenWindowResized?.()
  unlistenRemoteConn?.()
  for (const [tabId, instance] of terminalInstances.entries()) {
    void disposeTerminal(instance.term, `onUnmounted(tabId=${tabId})`)
  }
})

/** 桌面端接管被远程持有的终端 */
async function handleClaim(tabId: string) {
  const instance = terminalInstances.get(tabId)
  if (instance?.ptyId) {
    // 浏览器端 claim = 重新 attach（sessionClaim），记录标记防重复；
    // 传当前实际尺寸（attach 会 resize PTY，默认值会把终端打回 80×24）
    attachedPtyIds.add(instance.ptyId)
    await ptyClaim(instance.ptyId, instance.term.cols, instance.term.rows)
    // claim 后 Rust 会 emit session-reclaimed 解除遮罩；主动 fit 恢复尺寸。
    // 接管者自身的遮罩没有对应的解除事件（taken-over 只发给被抢方），本地解除
    const tab = sessionStore.getTabByPtyId(instance.ptyId)
    if (tab) tab.takenOver = false
    instance.fitAddon.fit()
    instance.term.focus()
  }
}

function isTabTakenOver(tabId: string): boolean {
  return sessionStore.tabs.get(tabId)?.takenOver ?? false
}

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
  refreshRemoteSessions,
  captureSnapshot,
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

/* 远程（手机）接管遮罩 */
.taken-over-overlay {
  position: absolute;
  inset: 0;
  z-index: 10;
  background: var(--bg-primary);
  display: flex;
  align-items: center;
  justify-content: center;
}

.taken-over-card {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 10px;
  padding: 28px 32px;
}

.taken-over-title {
  font-size: 14px;
  font-weight: 600;
  color: var(--text-primary);
}

.taken-over-hint {
  font-size: 12px;
  color: var(--text-tertiary);
}

.taken-over-btn {
  margin-top: 6px;
  padding: 8px 18px;
  border: none;
  border-radius: 6px;
  background: var(--accent-color);
  color: #fff;
  font-size: 13px;
  cursor: pointer;
}

.taken-over-btn:hover {
  opacity: 0.9;
}

.xterm-container.drag-over {
  box-shadow: inset 0 0 0 2px var(--accent-gold);
}

/* 左右留白用 left/right 定位而非 padding：FitAddon 量的是本元素
   getComputedStyle().width（border-box 语义含 padding），padding 会让
   cols 按含留白的宽度计算 → 终端网格比可视区宽，右侧内容被裁 */
.terminal-wrapper {
  position: absolute;
  top: 0;
  left: 12px;
  right: 12px;
  height: 100%;
  box-sizing: border-box;
  display: none;
}

.terminal-wrapper.active {
  display: block;
}

.terminal-wrapper :deep(.xterm) {
  height: 100%;
}

/* xterm 6 自绘滚动条已通过 scrollbar: { showScrollbar: false } 隐藏（滚动由 Claude CLI 内部实现） */

/* 覆盖 xterm.css 的 .xterm:not(.allow-transparency) .xterm-viewport { background-color: #000 }：
   行高不整除的底部缝隙会露出该背景，改用主题终端背景色（--term-bg 由根元素注入） */
.terminal-wrapper :deep(.xterm-viewport) {
  background-color: var(--term-bg, transparent) !important;
}
</style>
