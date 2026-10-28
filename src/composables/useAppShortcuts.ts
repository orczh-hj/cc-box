import { getCurrentWindow, currentMonitor } from '@tauri-apps/api/window'
import { LogicalSize, LogicalPosition } from '@tauri-apps/api/dpi'
import { useAppStore } from '@/stores/app'
import { useSessionStore } from '@/stores/session'
import { useSidebarStore } from '@/stores/sidebar'
import { ptyKillAll, spawnNewInstance, setNativeEditMenu } from '@/api/tauri'
import { isTauriEnv } from '@/api/platform'
import { nativeEditMenuSupported, suppressNativeEditMenu } from '@/utils/mac-edit-keys'

export async function snapWindow(side: 'left' | 'right') {
  // 窗口贴靠与新建实例为桌面专属（浏览器无窗口控制能力）
  if (!isTauriEnv) return
  try {
    const win = getCurrentWindow()
    const monitor = await currentMonitor()
    if (!monitor) return

    const scaleFactor = monitor.scaleFactor

    // 计算窗口装饰边框尺寸（标题栏 + 边框）
    const inner = await win.innerSize()
    const outer = await win.outerSize()
    const frameWidth = (outer.width - inner.width) / scaleFactor
    const frameHeight = (outer.height - inner.height) / scaleFactor

    const halfWidth = Math.floor(monitor.size.width / scaleFactor / 2)
    const height = window.screen.availHeight
    const x = side === 'left'
      ? monitor.position.x / scaleFactor
      : monitor.position.x / scaleFactor + halfWidth
    const y = monitor.position.y / scaleFactor

    await win.setPosition(new LogicalPosition(x, y))
    // setSize 设置的是 webview 内部尺寸，需要减去边框使窗口外部恰好等于半屏
    await win.setSize(new LogicalSize(halfWidth - frameWidth, height - frameHeight))
  } catch (err) {
    console.error(`Failed to snap ${side}:`, err)
  }
}

export async function openNewAppInstance() {
  if (!isTauriEnv) return
  try {
    await spawnNewInstance()
  } catch (err) {
    console.error('Failed to open new instance:', err)
  }
}

export function switchTab(direction: 'next' | 'prev') {
  const appStore = useAppStore()
  const sessionStore = useSessionStore()
  const cwd = appStore.cwd
  if (!cwd) return
  const tabs = sessionStore.getProjectTabs(cwd)
  if (tabs.length < 2) return

  const currentId = sessionStore.activeTabId
  const currentIndex = tabs.findIndex(t => t.tabId === currentId)

  let nextIndex: number
  if (currentIndex === -1) {
    nextIndex = 0
  } else if (direction === 'next') {
    nextIndex = (currentIndex + 1) % tabs.length
  } else {
    nextIndex = (currentIndex - 1 + tabs.length) % tabs.length
  }

  sessionStore.setActiveTab(tabs[nextIndex].tabId)
}

export function useAppShortcuts() {
  const appStore = useAppStore()
  const sessionStore = useSessionStore()
  const sidebarStore = useSidebarStore()

  function handleGlobalKeydown(e: KeyboardEvent) {
    const mod = e.ctrlKey || e.metaKey

    // --- 全局快捷键（所有视图生效） ---

    // Mod + , => toggle settings
    if (mod && e.key === ',') {
      e.preventDefault()
      e.stopPropagation()
      sidebarStore.toggleSettings()
      return
    }

    // Mod + Shift + N => new instance
    if (mod && e.shiftKey && e.code === 'KeyN') {
      e.preventDefault()
      e.stopPropagation()
      openNewAppInstance()
      return
    }

    // Mod + Shift + Left => snap left
    if (mod && e.shiftKey && e.key === 'ArrowLeft') {
      e.preventDefault()
      e.stopPropagation()
      snapWindow('left')
      return
    }

    // Mod + Shift + Right => snap right
    if (mod && e.shiftKey && e.key === 'ArrowRight') {
      e.preventDefault()
      e.stopPropagation()
      snapWindow('right')
      return
    }

    // Mod + Shift + R => restart app
    if (mod && e.shiftKey && e.code === 'KeyR') {
      e.preventDefault()
      e.stopPropagation()
      try { ptyKillAll() } catch { /* ignore */ }
      window.location.reload()
      return
    }

    // Mod + Shift + H => toggle home/projects
    if (mod && e.shiftKey && e.code === 'KeyH') {
      e.preventDefault()
      e.stopPropagation()
      window.dispatchEvent(new CustomEvent('app:toggleHome'))
      return
    }

    // Mod + = => font increase
    if (mod && e.key === '=' && !e.altKey) {
      e.preventDefault()
      e.stopPropagation()
      appStore.setFontSize(appStore.fontSize + 1)
      return
    }

    // Mod + - => font decrease
    if (mod && e.key === '-' && !e.shiftKey) {
      e.preventDefault()
      e.stopPropagation()
      appStore.setFontSize(appStore.fontSize - 1)
      return
    }

    // Mod + 0 => font reset
    if (mod && e.key === '0') {
      e.preventDefault()
      e.stopPropagation()
      appStore.setFontSize(12)
      return
    }

    // Mod + Shift + / => shortcuts panel
    if (mod && e.shiftKey && e.code === 'Slash') {
      e.preventDefault()
      e.stopPropagation()
      sidebarStore.openSettings('shortcuts')
      return
    }

    // Mod + Shift + S => toggle sessions panel
    if (mod && e.shiftKey && e.code === 'KeyS') {
      e.preventDefault()
      e.stopPropagation()
      sidebarStore.togglePanel('sessions')
      return
    }

    // Mod + Shift + T => toggle always on top
    if (mod && e.shiftKey && e.code === 'KeyT') {
      e.preventDefault()
      e.stopPropagation()
      appStore.toggleAlwaysOnTop()
      return
    }

    // --- 会话与标签管理（全局生效） ---

    // Alt+N => new session
    if (e.altKey && !mod && e.code === 'KeyN') {
      e.preventDefault()
      e.stopPropagation()
      window.dispatchEvent(new CustomEvent('terminal:newSession'))
      return
    }

    // Alt+R => restart session
    if (e.altKey && !mod && e.code === 'KeyR') {
      e.preventDefault()
      e.stopPropagation()
      window.dispatchEvent(new CustomEvent('terminal:restartSession'))
      return
    }

    // Alt+W => close current tab
    if (e.altKey && !mod && e.code === 'KeyW') {
      e.preventDefault()
      e.stopPropagation()
      if (sessionStore.activeTabId) {
        sessionStore.closeTab(sessionStore.activeTabId)
      }
      return
    }

    // Ctrl+Tab => switch to next tab (cyclic)
    if (e.ctrlKey && !e.altKey && !e.shiftKey && e.key === 'Tab') {
      e.preventDefault()
      e.stopPropagation()
      switchTab('next')
      return
    }

    // Ctrl+Shift+Tab => switch to previous tab (cyclic)
    if (e.ctrlKey && e.shiftKey && !e.altKey && e.key === 'Tab') {
      e.preventDefault()
      e.stopPropagation()
      switchTab('prev')
      return
    }

    // Alt+Up => switch to previous tab
    if (e.altKey && !mod && e.key === 'ArrowUp') {
      e.preventDefault()
      e.stopPropagation()
      switchTab('prev')
      return
    }

    // Alt+Down => switch to next tab
    if (e.altKey && !mod && e.key === 'ArrowDown') {
      e.preventDefault()
      e.stopPropagation()
      switchTab('next')
      return
    }
  }

  function setupShortcutListeners(): (() => void)[] {
    window.addEventListener('keydown', handleGlobalKeydown, true)
    const cleanups = [() => window.removeEventListener('keydown', handleGlobalKeydown, true)]

    // macOS：焦点驱动的原生 Edit 菜单切换——终端持焦（xterm-helper-textarea
    // 获焦）时切空菜单让 xterm 收到全部按键，其余焦点一律挂回 Edit 菜单
    if (nativeEditMenuSupported) {
      let lastActive: boolean | null = null
      const handleFocusIn = (e: FocusEvent) => {
        const active = !suppressNativeEditMenu(e.target)
        if (active === lastActive) return
        lastActive = active
        setNativeEditMenu(active).catch((err) => console.error('setNativeEditMenu failed:', err))
      }
      window.addEventListener('focusin', handleFocusIn, true)
      cleanups.push(() => window.removeEventListener('focusin', handleFocusIn, true))
    }

    return cleanups
  }

  return { setupShortcutListeners }
}
