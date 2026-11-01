import { createApp } from 'vue'
import { createPinia } from 'pinia'
import App from './App.vue'
import i18n from './i18n'
import { logMessage } from './api/tauri'
import { isCommandNotAllowedError } from './utils/rpc-errors'
import { notifyDesktopOutdated, setDesktopOutdatedTextProvider } from './utils/desktop-outdated'

import './styles/global.css'

// 旧桌面提示文案（transport 层不依赖 i18n，文案在此注入）
setDesktopOutdatedTextProvider(() => i18n.global.t('desktopOutdatedHint'))

// 前端全局错误捕获 → 写入 cc-box 日志文件（~/.cc-box/logs）。
// WebView 里的 JS 异常默认无处可查，转发后可在用户机器上远程定位问题
window.addEventListener('error', (e) => {
  const loc = e.filename ? ` @ ${e.filename}:${e.lineno}:${e.colno}` : ''
  logMessage('error', `[frontend] ${e.message}${loc}`)
})
window.addEventListener('unhandledrejection', (e) => {
  const reason = e.reason instanceof Error ? `${e.reason.message}\n${e.reason.stack}` : String(e.reason)
  logMessage('error', `[frontend] unhandled rejection: ${reason}`)
  // 新前端 + 旧桌面：未被捕获的「命令不被认识」也提示更新桌面端，
  // 避免界面看似故障而无任何指引（正常路径已在 transport 层提示）
  if (isCommandNotAllowedError(e.reason)) {
    notifyDesktopOutdated()
    e.preventDefault()
  }
})

const app = createApp(App)
const pinia = createPinia()

app.use(pinia)
app.use(i18n)
app.mount('#app')

// 生产环境下禁用右键菜单；普通可编辑元素放行，保留系统右键的复制/粘贴入口。
// xterm 的隐藏 textarea（xterm-helper-textarea）排除，终端右键行为保持不变
if (import.meta.env.PROD) {
  window.addEventListener('contextmenu', (e) => {
    const target = e.target as HTMLElement | null
    if (!target || target.classList.contains('xterm-helper-textarea')) {
      e.preventDefault()
      return
    }
    if (target.tagName === 'INPUT' || target.tagName === 'TEXTAREA' || target.isContentEditable) return
    e.preventDefault()
  })
}