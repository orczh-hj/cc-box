/**
 * Claude Code 主会话快捷键清单（手机快捷键面板数据源）。
 *
 * 只收录主会话内有效的快捷键（Global / Chat / History / Autocomplete 上下文），
 * 排除仅在特定对话框内生效的键（Select / ThemePicker / DiffDialog / Plugin /
 * ModelPicker / Pane 等）。来源：
 * - website/content/claude-code-docs/reference/interactive-mode.md（常规控制、文本编辑）
 * - website/content/claude-code-docs/configuration/interface/keybindings.md（操作默认绑定、和弦）
 *
 * 排序原则：**分组与组内均按使用频率降序**——最常用的在最上面，减少滚动与寻找成本。
 * bytes 为发送到 PTY 的终端序列：Ctrl+字母 = 0x01-0x1A；Alt+字母 = ESC 前缀；
 * 和弦（如 Ctrl+X Ctrl+K）= 两个序列连发（间隔必然 <3s，CLI 按和弦识别）。
 */
export interface ShortcutItem {
  /** 功能名 i18n key（按钮显示功能而非组合键） */
  labelKey: string
  /** 键位显示文本（副标题，便于对照桌面习惯） */
  keys: string
  /** macOS 键名差异（Option=⌥ 等）；缺省同 keys */
  keysMac?: string
  /** 发送到 PTY 的序列（平台间一致：Alt 在 mac 上是 Option/Meta，字节相同）。
   *  action 项无 bytes */
  bytes?: string
  /** 本地动作（非字节发送）：paste = 读剪贴板并括号粘贴 */
  action?: 'paste'
}

export interface ShortcutGroup {
  /** 分组标题 i18n key */
  titleKey: string
  items: ShortcutItem[]
}

export const CLAUDE_SHORTCUTS: ShortcutGroup[] = [
  {
    // 最常用：打断/提交/换行/模式切换/清屏，手机端几乎每次交互都会用到
    titleKey: 'scGroupControl',
    items: [
      { labelKey: 'scCancelDialog', keys: 'Esc', bytes: '\x1b' },
      { labelKey: 'scSubmit', keys: 'Enter', bytes: '\r' },
      { labelKey: 'scNewline', keys: 'Ctrl+J', bytes: '\x0a' },
      { labelKey: 'scCycleMode', keys: 'Shift+Tab', bytes: '\x1b[Z' },
      { labelKey: 'scInterrupt', keys: 'Ctrl+C', bytes: '\x03' },
      { labelKey: 'scTranscript', keys: 'Ctrl+O', bytes: '\x0f' },
      { labelKey: 'scTodos', keys: 'Ctrl+T', bytes: '\x14' },
      { labelKey: 'scHistorySearch', keys: 'Ctrl+R', bytes: '\x12' },
      { labelKey: 'scBackgroundTask', keys: 'Ctrl+B', bytes: '\x02' },
      { labelKey: 'scRedraw', keys: 'Ctrl+L', bytes: '\x0c' },
      // Ctrl+S 收录：隐藏/恢复提示在手机上仍可用（字节是 XOFF，CLI raw mode 接收）
      { labelKey: 'scStashPrompt', keys: 'Ctrl+S', bytes: '\x13' },
      { labelKey: 'scModelPicker', keys: 'Alt+P', keysMac: '⌥P', bytes: '\x1bp' },
      { labelKey: 'scThinking', keys: 'Alt+T', keysMac: '⌥T', bytes: '\x1bt' },
      { labelKey: 'scFastMode', keys: 'Alt+O', keysMac: '⌥O', bytes: '\x1bo' },
      // 明确排除（终端范围外或手机不可用）：Ctrl+D 退出会话（易误触）、
      // Ctrl+G / Ctrl+X Ctrl+E 外部编辑器（手机无编辑器）、Ctrl+Z 暂停
      // （Unix 挂起后手机无法 fg 恢复）、Alt+V 图像粘贴（剪贴板图像链路不可靠）
    ],
  },
  {
    // 次常用：翻历史、移动光标、粘贴（手机输入场景高频）
    // 方向键/粘贴不在 Claude Code 文档绑定表内，但由终端与 CLI 直接处理
    titleKey: 'scGroupCursor',
    items: [
      { labelKey: 'scPrevHistory', keys: '↑', bytes: '\x1b[A' },
      { labelKey: 'scNextHistory', keys: '↓', bytes: '\x1b[B' },
      { labelKey: 'scMoveLeft', keys: '←', bytes: '\x1b[D' },
      { labelKey: 'scMoveRight', keys: '→', bytes: '\x1b[C' },
      { labelKey: 'scPaste', keys: '', action: 'paste' },
      { labelKey: 'scAcceptCompletion', keys: 'Tab', bytes: '\t' },
    ],
  },
  {
    // 编辑：清行/删词/跳行首行尾，长输入时使用
    titleKey: 'scGroupEdit',
    items: [
      { labelKey: 'scKillToStart', keys: 'Ctrl+U', bytes: '\x15' },
      { labelKey: 'scKillToEnd', keys: 'Ctrl+K', bytes: '\x0b' },
      { labelKey: 'scLineStart', keys: 'Ctrl+A', bytes: '\x01' },
      { labelKey: 'scLineEnd', keys: 'Ctrl+E', bytes: '\x05' },
      { labelKey: 'scKillWord', keys: 'Ctrl+W', bytes: '\x17' },
      { labelKey: 'scBackspace', keys: 'Backspace', bytes: '\x7f' },
      { labelKey: 'scDeleteChar', keys: 'Delete', bytes: '\x1b[3~' },
      { labelKey: 'scYank', keys: 'Ctrl+Y', bytes: '\x19' },
      { labelKey: 'scUndo', keys: 'Ctrl+_', bytes: '\x1f' },
      { labelKey: 'scWordBack', keys: 'Alt+B', keysMac: '⌥B', bytes: '\x1bb' },
      { labelKey: 'scWordForward', keys: 'Alt+F', keysMac: '⌥F', bytes: '\x1bf' },
      { labelKey: 'scKillWordEnd', keys: 'Alt+D', keysMac: '⌥D', bytes: '\x1bd' },
    ],
  },
  {
    // 低频：和弦组合（需要连按两段），放最后
    titleKey: 'scGroupChord',
    items: [
      { labelKey: 'scSendNow', keys: 'Ctrl+X Ctrl+S', bytes: '\x18\x13' },
      { labelKey: 'scQueueSubmit', keys: 'Ctrl+X Enter', bytes: '\x18\r' },
      { labelKey: 'scKillAgents', keys: 'Ctrl+X Ctrl+K', bytes: '\x18\x0b' },
      { labelKey: 'scToggleAbovePrompt', keys: 'Ctrl+X Ctrl+A', bytes: '\x18\x01' },
    ],
  },
]
