> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# 交互模式

> Claude Code 会话中键盘快捷键、输入模式和交互功能的完整参考。

## 快捷键

> [!NOTE]
>
> 快捷键可能因平台和终端而异。在[全屏渲染](https://code.claude.com/docs/zh-CN/fullscreen.md)中，在记录查看器中按 `?` 查看可用的快捷键。
>
> **macOS 用户**：Option/Alt 键快捷键（`Alt+B`、`Alt+F`、`Alt+D`、`Alt+Y`、`Alt+P`）需要在终端中将 Option 配置为 Meta。请参阅[在 macOS 上启用 Option 键快捷键](https://code.claude.com/docs/zh-CN/terminal-config.md#enable-option-key-shortcuts-on-macos)了解每个终端中的设置。

### 常规控制

| 快捷键 | 描述 | 上下文 |
| :- | :- | :- |
| `Ctrl+C` | 中断或清除输入 | 中断正在运行的操作。如果没有任何操作在运行，第一次按下会清除提示输入，第二次按下会退出 Claude Code。要找回被清除的草稿，请参阅[找回被清除的草稿](#bring-back-a-cleared-draft) |
| `Ctrl+X Ctrl+K` | 停止此会话中所有正在运行的[后台子代理](https://code.claude.com/docs/zh-CN/sub-agents.md#run-subagents-in-foreground-or-background)，并在该会话的剩余时间内关闭 [Artifact 自动回复](https://code.claude.com/docs/zh-CN/artifacts.md#let-claude-reply-to-comments-on-its-own)。在 3 秒内按两次以确认。后台子代理的权限提示打开时，您也可以按此快捷键 | 子代理控制 |
| `Ctrl+D` | 退出 Claude Code 会话 | 第一次按下显示确认提示，第二次在 800ms 内按下会退出。当提示有文本时，`Ctrl+D` 会删除光标后的字符 |
| `Ctrl+G` 或 `Ctrl+X Ctrl+E` | 在默认文本编辑器中打开 | 在默认文本编辑器中编辑您的提示或自定义响应。`Ctrl+X Ctrl+E` 是 readline 原生绑定。在 `/config` 中打开**在外部编辑器中显示最后一个响应**，以在您的提示上方将 Claude 的前一个回复作为 `#` 注释上下文预置；Claude Code 在您保存时会删除注释块 |
| `Ctrl+L` | 重绘屏幕 | 强制完整的终端重绘，保持输入和对话历史。如果显示变得混乱或部分空白，请使用此选项恢复。请参阅[清除对话](https://code.claude.com/docs/zh-CN/fullscreen.md#clear-the-conversation)了解全屏渲染 |
| `Ctrl+O` | 切换记录查看器 | 显示详细的工具使用和执行情况，每条助手消息上都有时间戳和使用的模型。还会展开默认折叠的行，例如 MCP 调用，显示为单个 `Called slack 3 times` 行，以及[来自您其他会话的消息](https://code.claude.com/docs/zh-CN/cross-session-messaging.md#what-a-message-looks-like)，显示为单行 `Message from @<sender>` 预览 |
| `Ctrl+R` | 反向搜索命令历史 | 交互式搜索以前的命令 |
| `Ctrl+V` 或 `Cmd+V`（iTerm2）或 `Alt+V`（Windows 和 WSL） | 从剪贴板粘贴图像 | 在光标处插入 `[Image #N]` 芯片，以便您可以在提示中按位置引用它。在 WSL 上，`Ctrl+V` 和 `Alt+V` 都被绑定；如果您的终端拦截 `Ctrl+V`，请使用 `Alt+V` |
| `Ctrl+B` | 后台运行任务 | 后台运行 Bash 命令和代理。Tmux 用户按两次 |
| `Ctrl+T` | 切换 Claude 的任务清单 | 在状态区域中显示或隐藏 [Claude 的待办事项清单](#task-list)。这不是后台任务视图；使用 [`/tasks`](https://code.claude.com/docs/zh-CN/commands) 查看运行的 shell 和子代理 |
| `Ctrl+S` | 隐藏或恢复提示 | 输入中有文本时，隐藏它并清除提示。在空提示上再次按下时，恢复隐藏的文本、光标位置、粘贴的内容和输入模式，因此隐藏的 `!` [shell 命令](#shell-mode-with-prefix)会以 shell 模式返回 |
| `Ctrl+Z` | 暂停 Claude Code | 仅限 Unix。将进程暂停到您的 shell；运行 `fg` 以恢复 |
| `Left/Right arrows` | 在对话框选项卡之间循环 | 在权限对话框和菜单中的选项卡之间导航。在选项卡式对话框中，当选项卡行获得焦点时，这些键会切换选项卡。请参阅[选项卡操作](https://code.claude.com/docs/zh-CN/keybindings.md#tabs-actions)了解焦点如何移动 |
| `Tab` | 接受自动完成建议，或向权限答案添加注释 | 当自动完成建议在提示输入中显示时，接受选定的建议。在大多数权限提示上，当**是**或**否**获得焦点时，在该选项上打开注释字段，再次按下会关闭该字段。请参阅[在回答权限提示时添加注释](https://code.claude.com/docs/zh-CN/permissions.md#add-a-comment-when-you-answer-a-permission-prompt) |
| `Up/Down arrows` 或 `Ctrl+P`/`Ctrl+N` | 移动光标或导航命令历史 | 当输入跨越多个可视行时，无论是换行还是多行，首先在提示中移动光标。一旦光标在第一行或最后一行，再次按下会导航命令历史。当您有排队的消息时，从第一行按 `Up` 会[取回它们](#take-back-what-you-queued) |
| `Esc` | 中断 Claude 或关闭对话框 | 停止当前响应或工具调用中途，以便您可以重定向。Claude 保留迄今为止所做的工作。如果您有[排队的消息](#queue-messages-while-claude-works)，Claude Code 会在下一步发送它们。当对话框打开时，`Esc` 会关闭对话框。当选中页脚项目时，例如提示下方的[子代理面板](https://code.claude.com/docs/zh-CN/sub-agents.md#run-subagents-in-foreground-or-background)中的一行，`Esc` 会[取消选择它](https://code.claude.com/docs/zh-CN/keybindings.md#footer-actions)而不是中断。在权限提示上，`Esc` 会拒绝该操作，与[**否**不带注释](https://code.claude.com/docs/zh-CN/permissions.md#add-a-comment-when-you-answer-a-permission-prompt)相同 |
| `Esc` + `Esc` | 清除输入草稿或回退 | 当提示输入包含文本时，双 `Esc` 会清除它并将草稿保存到历史记录，以便 `Up` 可以调用它。当输入为空时，双 `Esc` 会打开[回退菜单](https://code.claude.com/docs/zh-CN/checkpointing.md)以从之前的某个点恢复或总结代码和对话 |
| `Ctrl+Enter` 或 `Ctrl+X Ctrl+S` | 立即发送排队的消息 | 发送您的[排队的消息](#queue-messages-while-claude-works)和您的草稿与它们一起立即发出。[Claude Code 何时发送您排队的内容](#when-claude-code-sends-what-you-queued)涵盖了 Claude 正在处理的轮次会发生什么。在[shell 模式](#shell-mode-with-prefix)中，该键仅排队您的命令。在不报告扩展键的终端中，`Ctrl+Enter` 作为普通 `Enter` 到达；`Ctrl+X Ctrl+S` 在任何终端中都有效。需要 Claude Code v2.1.275 或更高版本 |
| `Shift+Tab` 或在 Node 或 Bun 运行时不启用 VT 输入模式时在 Windows 上使用 `Alt+M` | 循环权限模式 | 循环通过 `default`（在模式指示器中标记为 Manual）、`acceptEdits`、`plan` 和（如果可用）`bypassPermissions` 然后 `auto`。从 `auto`，第一次按下切换到 `default`。请参阅[权限模式](https://code.claude.com/docs/zh-CN/permission-modes.md)。在文件权限提示上，相同的键会关闭打开的[注释字段](https://code.claude.com/docs/zh-CN/permissions.md#add-a-comment-when-you-answer-a-permission-prompt)。如果没有字段打开，它会选择允许该操作在会话其余部分的选项，当提示提供该选项时 |
| `Option+P`（macOS）或 `Alt+P`（Windows/Linux） | 切换模型 | 在不清除提示的情况下切换模型 |
| `Option+T`（macOS）或 `Alt+T`（Windows/Linux） | 切换扩展思考 | 启用或禁用扩展思考模式。对 Opus 5.5、Sonnet 5.5、Haiku 5.5 或 Fable 模型无效，它们始终使用扩展思考。在 macOS 上无需配置 Option 为 Meta 即可工作 |
| `Option+O`（macOS）或 `Alt+O`（Windows/Linux） | 切换快速模式 | 启用或禁用[快速模式](https://code.claude.com/docs/zh-CN/fast-mode.md) |

### 文本编辑

| 快捷键 | 描述 | 上下文 |
| :- | :- | :- |
| `Ctrl+A` | 将光标移动到当前行的开始 | 在多行输入中，移动到当前逻辑行的开始 |
| `Ctrl+E` | 将光标移动到当前行的末尾 | 在多行输入中，移动到当前逻辑行的末尾 |
| `Ctrl+K` | 删除到行尾 | 存储删除的文本以供粘贴 |
| `Ctrl+U` | 从光标删除到行开始 | 存储删除的文本以供粘贴。重复以清除多行输入中的多行。在 macOS 上，包括 iTerm2 和 Terminal.app 在内的终端模拟器将 `Cmd+Backspace` 映射到此快捷键 |
| `Ctrl+W` | 删除回到上一个空格 | 存储删除的文本以供粘贴。一次按下会删除整个路径或 `--flag=value`。要仅删除上一个单词，请在 macOS 上按 `Option+Delete` 或在 Windows 上按 `Ctrl+Backspace` |
| `Ctrl+Y` | 粘贴删除的文本 | 粘贴您最后用单词或行删除快捷键（如 `Ctrl+K`、`Ctrl+U` 或 `Ctrl+W`）删除的文本 |
| `Alt+Y`（在 `Ctrl+Y` 之后） | 循环粘贴历史 | 粘贴后，循环通过之前删除的文本。在 macOS 上需要[Option 作为 Meta](#keyboard-shortcuts) |
| `Alt+B` | 将光标向后移动一个单词 | 单词导航。在 macOS 上需要[Option 作为 Meta](#keyboard-shortcuts) |
| `Alt+F` | 将光标向前移动一个单词 | 移动到当前单词的末尾，或当光标在单词之间时移动到下一个单词的末尾。在 macOS 上需要[Option 作为 Meta](#keyboard-shortcuts) |
| `Alt+D` | 删除到单词末尾 | 删除到当前单词的末尾，或当光标在单词之间时删除到下一个单词的末尾。存储删除的文本以供粘贴。在 macOS 上需要[Option 作为 Meta](#keyboard-shortcuts) |
| `Ctrl+_` 或 `Ctrl+Shift+-` | 撤销最后一次输入编辑 | 恢复上一个输入文本和光标位置 |

### 编辑快捷键中的单词边界

单词快捷键 `Alt+B`、`Alt+F`、`Alt+D`、`Option+Delete` 和 `Ctrl+Backspace` 将单词视为字母和数字的运行，因此标点符号（如 `_`、`.` 和 `/`）分隔单词。在提示中有 `src/utils/foo.ts` 时，重复按 `Alt+B` 会在 `ts`、`foo`、`utils` 和 `src` 的开始处停止。

`Ctrl+W` 不同：它忽略标点符号并删除回到上一个空格，因此一次按下会删除所有 `src/utils/foo.ts`。

在没有空格的文本中，例如中文或日文，单词快捷键仍然一次移动或删除一个单词。

这些 readline 约定适用于 Claude Code v2.1.261 及更高版本。在早期版本中打开它们的 [`keybindingFlavor`](https://code.claude.com/docs/zh-CN/settings-reference#keybindingflavor) 设置已弃用，无效。

您无法在[快捷键配置文件](https://code.claude.com/docs/zh-CN/keybindings.md)中重新映射这些快捷键，该文件没有这些操作的操作。

### 主题和显示

| 快捷键 | 描述 | 上下文 |
| :- | :- | :- |
| `Ctrl+T` | 切换代码块的语法突出显示 | 仅在 `/theme` 选择器菜单内工作。控制 Claude 响应中的代码是否使用语法着色 |

### 多行输入

| 方法 | 快捷键 | 上下文 |
| :- | :- | :- |
| 快速转义 | `\` + `Enter` | 在所有终端中工作 |
| Option 键 | `Option+Enter` | 在 macOS 上启用[Option 作为 Meta](https://code.claude.com/docs/zh-CN/terminal-config.md#enable-option-key-shortcuts-on-macos)后 |
| Shift+Enter | `Shift+Enter` | 在 iTerm2、WezTerm、Ghostty、Kitty、Warp、Apple Terminal、Windows Terminal 中原生支持。对于其他终端，请参阅[输入多行提示](https://code.claude.com/docs/zh-CN/terminal-config.md#enter-multiline-prompts) |
| 控制序列 | `Ctrl+J` | 在任何终端中无需配置即可工作 |
| 粘贴模式 | 直接粘贴 | 对于代码块、日志 |

### 快速命令

| 快捷键 | 描述 | 注释 |
| :- | :- | :- |
| `/` 在开始 | 命令或 skill | 请参阅[命令](#commands)和[skills](https://code.claude.com/docs/zh-CN/skills.md) |
| `!` 在开始 | Shell 模式 | 直接运行命令，将其输出添加到会话，并让 Claude 对其进行响应 |
| `@` | 文件路径提及 | 触发文件路径自动完成。在具有[跨会话消息传递](https://code.claude.com/docs/zh-CN/cross-session-messaging.md#message-another-session)的会话中，当您在 `@` 后键入至少一个字母时，Claude Code 也会建议您在此机器上的其他实时会话，以便您可以告诉 Claude 向您选择的会话发送消息。需要 Claude Code v2.1.232 或更高版本 |
| `:` | 表情符号简码 | 键入完整的 `:name:` 以插入表情符号，或键入两个或更多字符以获取建议。请参阅[表情符号简码](#emoji-shortcodes) |
| `?` 在空输入上 | 切换快捷键帮助面板 | 当输入已包含文本时键入 `?` 会插入该字符 |

### 记录查看器

当记录查看器打开时（使用 `Ctrl+O` 切换），这些快捷键可用。运行不带参数的 `/tui` 以检查哪个渲染器处于活动状态。`Ctrl+E` 可以通过 [`transcript:toggleShowAll`](https://code.claude.com/docs/zh-CN/keybindings) 重新绑定。

| 快捷键 | 描述 |
| :- | :- |
| `?` | 切换键盘快捷键帮助面板。需要[全屏渲染](https://code.claude.com/docs/zh-CN/fullscreen.md) |
| `{` / `}` | 跳转到上一个或下一个用户提示，如 vim 段落运动。需要[全屏渲染](https://code.claude.com/docs/zh-CN/fullscreen.md) |
| `Ctrl+E` | 切换显示所有内容。仅在经典渲染器中可用，在[全屏渲染](https://code.claude.com/docs/zh-CN/fullscreen.md)中不可用 |
| `[` | 将完整对话写入终端的原生滚动缓冲区，以便 `Cmd+F`、tmux 复制模式和其他原生工具可以搜索它。需要[全屏渲染](https://code.claude.com/docs/zh-CN/fullscreen.md#search-and-review-the-conversation) |
| `v` | 将对话写入临时文件并在 `$VISUAL` 或 `$EDITOR` 中打开它。需要[全屏渲染](https://code.claude.com/docs/zh-CN/fullscreen.md) |
| `q`、`Ctrl+C`、`Esc` | 退出记录查看。所有三个都可以通过 [`transcript:exit`](https://code.claude.com/docs/zh-CN/keybindings) 重新绑定 |

### 语音输入

| 快捷键 | 描述 | 注释 |
| :- | :- | :- |
| 按住或点击 `Space` | 语音听写 | 需要启用[语音听写](https://code.claude.com/docs/zh-CN/voice-dictation.md)。按住以录制，或运行 `/voice tap` 以进行点击切换。[可重新绑定](https://code.claude.com/docs/zh-CN/voice-dictation.md#rebind-the-dictation-key) |

## 命令

在 Claude Code 中输入 `/` 可以查看可用的命令，或输入 `/` 后跟任何字母来筛选。`/` 菜单列出了内置命令、捆绑的和用户编写的 [skills](https://code.claude.com/docs/zh-CN/skills.md)，以及由 [plugins](https://code.claude.com/docs/zh-CN/plugins/overview.md) 和 [MCP servers](https://code.claude.com/docs/zh-CN/mcp.md#use-mcp-prompts-as-commands) 贡献的命令。并非所有内置命令对每个用户都可见，因为某些命令取决于您的平台或计划，而且 [少数可用命令在设计上从菜单中隐藏](https://code.claude.com/docs/zh-CN/commands.md#how-the-command-menu-matches-what-you-type)，当您输入其全名时运行。

在 [fullscreen rendering](https://code.claude.com/docs/zh-CN/fullscreen.md#use-the-mouse) 中，`/` 命令和 `@` 文件建议列表也响应鼠标：悬停突出显示一行，点击接受它。

有关 Claude Code 中包含的命令的完整列表，请参阅 [commands reference](https://code.claude.com/docs/zh-CN/commands.md)。

### 在提示中途完成命令

命令完成也适用于提示的中途：在空格后输入 `/`，然后输入名称的前几个字母，如 `run the tests, then /com`。只有名称以这些字母开头的命令才会匹配，因此文件路径（如 `/tmp/notes.md`）不会保持列表打开。Claude Code 仅在命令 [starts your message](https://code.claude.com/docs/zh-CN/commands.md) 时自己运行命令。

* **在 [fullscreen rendering](https://code.claude.com/docs/zh-CN/fullscreen.md) 中**：匹配项在您输入时以列表形式打开，没有突出显示的行，因此 `Enter` 仍会按输入的方式发送您的提示。按 `Tab` 插入顶部匹配项，或使用箭头键和 `Enter` 选择一行。
* **在 fullscreen 之外**：顶部匹配项的其余部分在您的光标处显示为幽灵文本，当有更多命令匹配时显示 `+2` 之类的计数。按 `Tab` 插入唯一的匹配项，或在有多个匹配项时打开列表，然后使用箭头键和 `Enter` 选择一行。

在两个渲染器中，在裸露的中途 `/` 上按 `Tab` 可列出每个命令。

插件 skill 也会在其裸露名称上匹配，因此 `/deploy` 会找到名为 `myplugin:deploy-app` 的 skill。当您插入匹配项时，Claude Code 会写入完整的 `/myplugin:deploy-app`。

## Vim 编辑器模式

通过 `/config` → Editor mode 启用 vim 风格编辑。

Claude Code 在你使用 `Ctrl+O` 切换[文字记录查看器](#transcript-viewer)或打开和关闭面板（如 `/config`）时保持你的 vim 模式和光标位置。如果你在 NORMAL 模式下离开提示符，当你返回时它仍然处于 NORMAL 模式，光标位置与你离开时相同。

### 模式切换

| 命令 | 操作 | 来自模式 |
| :- | :- | :- |
| `Esc` 或 `Ctrl+[` | 进入 NORMAL 模式。在使用 Kitty 键盘协议的终端中，`Ctrl+[` 需要 v2.1.242 或更高版本 | INSERT、VISUAL |
| `i` | 在光标前插入 | NORMAL |
| `I` | 在行首插入 | NORMAL |
| `a` | 在光标后插入 | NORMAL |
| `A` | 在行尾插入 | NORMAL |
| `o` | 在下方打开新行 | NORMAL |
| `O` | 在上方打开新行 | NORMAL |
| `v` | 开始字符级可视选择 | NORMAL |
| `V` | 开始行级可视选择 | NORMAL |

### 重新映射 INSERT 模式快捷键序列

[`vimInsertModeRemaps`](https://code.claude.com/docs/zh-CN/settings-reference#viminsertmoderemaps) 设置将两个按键的 INSERT 模式序列映射到 Escape，因此像 `jj` 这样的映射会让您返回 NORMAL 模式。

以下 `~/.claude/settings.json` 示例打开 vim 模式并将 `jj` 映射到 Escape：

```json theme={null}
{
  "editorMode": "vim",
  "vimInsertModeRemaps": { "jj": "<Esc>" }
}
```

每个键恰好是按顺序输入的两个可打印字符，`"<Esc>"` 是唯一支持的目标。长度或目标不同的条目会被忽略。

输入序列的第一个字符会正常插入。在一秒内按下第二个字符会移除待处理的字符并切换到 NORMAL 模式，在你的输入中不留下任何字符。在一秒窗口之后，或者如果按下不同的键，两个字符都会保留为字面文本，因此你仍然可以通过在两个键之间暂停来输入包含该序列的单词。

Claude Code 从你的用户设置文件、`--settings` 标志和[托管设置](https://code.claude.com/docs/zh-CN/managed-settings.md)读取此设置。项目的 `.claude/settings.json` 或 `.claude/settings.local.json` 中的条目会被忽略，因此已检出的存储库无法重新映射你的按键。

### 导航（NORMAL 模式）

| 命令 | 操作 |
| :- | :- |
| `h`/`j`/`k`/`l` | 向左/向下/向上/向右移动 |
| `Space` | 向右移动 |
| `w` | 下一个单词 |
| `e` | 单词末尾 |
| `b` | 上一个单词 |
| `0` | 行首 |
| `$` | 行尾 |
| `^` | 第一个非空白字符 |
| `gg` | 输入开始 |
| `G` | 最后一行的行首 |
| `f{char}` | 跳转到当前行中该字符的下一个出现位置 |
| `F{char}` | 跳转到当前行中该字符的上一个出现位置 |
| `t{char}` | 跳转到当前行中该字符的下一个出现位置之前 |
| `T{char}` | 跳转到当前行中该字符的上一个出现位置之后 |
| `;` | 重复上一个 f/F/t/T 动作 |
| `,` | 反向重复上一个 f/F/t/T 动作 |
| `%` | 跳转到匹配的 `()`、`[]` 或 `{}` 括号；当光标不在括号上时，从当前行的下一个括号开始。需要 Claude Code v2.1.301 或更高版本 |
| `/` | 打开反向历史搜索，与 `Ctrl+R` 相同。空搜索提示显示提示：按 `Esc` 然后 `i` 然后 `/` 来打开命令菜单 |

> [!NOTE]
>
> 在 vim NORMAL 模式中，如果光标在输入的开始或结束处且无法进一步移动，`j`/`k` 和 `↑`/`↓` 会改为导航命令历史。在空输入框上按 `←` 会打开 [Agent 视图](https://code.claude.com/docs/zh-CN/agent-view.md)，在 NORMAL 模式和 INSERT 模式下均适用。

### 编辑（NORMAL 模式）

| 命令 | 操作 |
| :- | :- |
| `x` | 删除字符 |
| `r{char}` | 用 `{char}` 替换光标下的字符 |
| `dd` | 删除行 |
| `D` | 删除到行尾 |
| `dw`/`de`/`db` | 删除单词/到末尾/向后 |
| `df{char}`/`dt{char}` | 删除到当前行中该字符的下一个出现位置（包括或不包括该字符） |
| `dj`/`dk` | 删除当前行和下方或上方的行 |
| `dgg`/`dG` | 从当前行删除到第一行或最后一行 |
| `d0`/`c0`/`y0` | 从光标删除、更改或复制回行首。需要 Claude Code v2.1.281 或更高版本 |
| `d%`/`c%`/`y%` | 从光标删除、更改或复制到 `%` 跳转到的括号（包括该括号）。需要 Claude Code v2.1.301 或更高版本 |
| `cc` | 更改行 |
| `C` | 更改到行尾 |
| `cw`/`ce`/`cb` | 更改单词/到末尾/向后 |
| `s` | 替换字符：删除光标下的字符并进入 INSERT 模式 |
| `S` | 替换行：清除行并进入 INSERT 模式 |
| `yy`/`Y` | 复制行 |
| `yw`/`ye`/`yb` | 复制单词/到末尾/向后 |
| `p` | 在光标后粘贴 |
| `P` | 在光标前粘贴 |
| `>>` | 缩进行 |
| `<<` | 取消缩进行 |
| `J` | 合并行 |
| `u` | 撤销 |
| `.` | 重复上一个更改 |

### 文本对象（NORMAL 模式）

文本对象与 `d`、`c` 和 `y` 等运算符一起使用：

| 命令 | 操作 |
| :- | :- |
| `iw`/`aw` | 内部/周围单词 |
| `iW`/`aW` | 内部/周围 WORD（空白分隔） |
| `i"`/`a"` | 内部/周围双引号 |
| `i'`/`a'` | 内部/周围单引号 |
| `i(`/`a(` | 内部/周围括号 |
| `i[`/`a[` | 内部/周围方括号 |
| `i{`/`a{` | 内部/周围花括号 |

### 可视模式

按 `v` 进行字符级选择或按 `V` 进行行级选择。动作扩展选择，运算符直接作用于它。

| 命令 | 操作 |
| :- | :- |
| `d`/`x` | 删除选择 |
| `y` | 复制选择 |
| `c`/`s` | 更改选择 |
| `p` | 用寄存器内容替换选择 |
| `r{char}` | 将每个选定的字符替换为 `{char}` |
| `~`/`u`/`U` | 切换、小写或大写选择 |
| `>`/`<` | 缩进或取消缩进选定的行 |
| `J` | 合并选定的行 |
| `o` | 交换光标和锚点 |
| `iw`/`aw`/`i"`/… | 选择文本对象 |
| `v`/`V` | 在字符级和行级之间切换，或退出 |

不支持使用 `Ctrl+V` 的块级可视模式。

### 在 vim 模式下取消问题

当您在 [Claude 提出的问题](https://code.claude.com/docs/zh-CN/tools-reference.md#askuserquestion-tool-behavior)的 `Other` 行中输入自己的回答时，按两次 `Esc` 即可取消该问题。

如需改为按一次键即可取消，请[绑定另一个键来取消问题](https://code.claude.com/docs/zh-CN/keybindings.md#cancel-a-question-with-one-key-press)。

## 命令历史

Claude Code 保留了你输入的提示词的历史记录，上箭头回调可以从同一项目的过去会话中获取提示词：

* 输入历史按工作目录存储
* 运行 `/clear` 开始新会话：回调时会首先列出新会话的提示词，然后是较早会话的提示词。前一个会话的对话被保留，可以恢复。
* 连续两次提交相同的提示词只记录一个历史条目，因此按上箭头会跳到前一个不同的提示词
* 当你回调包含粘贴文本的提示词时，Claude Code 在你重新提交时会再次发送完整的粘贴内容。如果内容已被[自动清理](https://code.claude.com/docs/zh-CN/claude-directory.md#cleaned-up-automatically)，Claude Code 不会发送字面上的 `[Pasted text #N]` 字符串；有关提示词发生的情况，请参阅[粘贴大型内容](https://code.claude.com/docs/zh-CN/terminal-config.md#paste-large-content)
* 使用 `!` 的历史扩展默认被禁用

### 找回已清除的草稿

要找回使用 `Ctrl+C` 清除的草稿，请按 `Up`，可以在清除后立即按，也可以在发送消息或命令之后按：

* **清除草稿后立即找回**：在提示词输入框仍为空时按一次 `Up`。需要 Claude Code v2.1.288 或更高版本。
* **发送消息或命令之后找回**：按 `Up` 回退到该草稿。Claude Code 会为当前会话保留最近清除的十个草稿及其粘贴的文本和图像，并在您退出时将其丢弃。需要 Claude Code v2.1.292 或更高版本。

### 使用 Ctrl+R 进行反向搜索

按 `Ctrl+R` 以交互方式搜索你的命令历史。在[全屏渲染](https://code.claude.com/docs/zh-CN/fullscreen.md)中，`Ctrl+R` 打开搜索对话框：输入以过滤，按 `Up` 和 `Down` 在匹配项中移动，按 `Ctrl+S` 循环切换范围（此会话、此项目和所有项目）。按 `Enter` 或 `Tab` 将匹配项放在提示词输入中，或按 `Esc` 取消。下面的步骤描述经典渲染器的内联搜索：

1. **开始搜索**：按 `Ctrl+R` 激活反向历史搜索
2. **输入查询**：输入要在以前的命令中搜索的文本。搜索词在匹配结果中突出显示
3. **导航匹配项**：再次按 `Ctrl+R` 循环浏览较早的匹配项
4. **搜索范围**：内联搜索始终搜索来自所有项目的提示词
5. **接受匹配项**：
   * 按 `Tab` 或 `Esc` 接受当前匹配项并继续编辑
   * 按 `Enter` 接受并立即执行命令
6. **取消搜索**：
   * 按 `Ctrl+C` 取消并恢复你的原始输入
   * 在空搜索上按 `Backspace` 取消

内联搜索扫描你的完整提示词历史，最新的优先，重复项折叠到最新出现。全屏对话框在选定的范围内搜索你的整个提示词历史，最新的优先，重复项折叠到最新出现：最近的提示词立即出现，较早提示词的匹配项在 Claude Code 加载其余部分时填充。匹配的提示词显示时搜索词突出显示，因此你可以找到并重用以前的输入。

接受匹配项或取消搜索立即生效，即使 Claude Code 仍在加载历史记录。

## 后台 Bash 命令

Claude Code 支持在后台运行 Bash 命令，允许你在长时间运行的进程执行时继续工作。

### 后台运行的工作原理

当 Claude Code 在后台运行命令时，它会异步运行该命令并立即返回一个后台任务 ID。Claude Code 可以在命令继续在后台执行时响应新的提示。

要在后台运行命令，你可以：

* 提示 Claude Code 在后台运行命令
* 按 `Ctrl+B` 将常规 Bash 工具调用移到后台。Tmux 用户必须按两次 `Ctrl+B`，因为 tmux 有前缀键。

当命令在完成前达到超时时，Claude Code 会自动[将其移到后台](https://code.claude.com/docs/zh-CN/tools-reference.md#foreground-commands-that-move-to-the-background)而不是停止它，除非命令以 `sleep` 开头。如果您已通过 [`CLAUDE_CODE_DISABLE_BACKGROUND_TASKS`](https://code.claude.com/docs/zh-CN/env-vars#variables) 或以 [bare 模式](https://code.claude.com/docs/zh-CN/headless.md#start-faster-with-bare-mode)启动关闭了后台命令，命令会在超时时停止。要更改超时时间，请设置 [Bash 超时环境变量](https://code.claude.com/docs/zh-CN/tools-reference.md#timeout-and-output-limits)。

**主要功能：**

* 输出被写入文件，Claude 可以使用 Read 工具检索它
* 后台任务有唯一的 ID 用于跟踪和输出检索
* 当 Claude Code 退出时，后台任务会自动清理。在 macOS 和 Linux 上，当你从 [`/tasks`](https://code.claude.com/docs/zh-CN/commands) 停止后台任务或 Claude Code 在退出时停止它时，从任务的 shell 分离的进程（例如在 `setsid` 或 `timeout` 下启动的进程）也会停止
* 如果你将会话放在后台而不是退出，你的后台任务将继续在后台会话中运行。请参阅[将运行中的会话放在后台](https://code.claude.com/docs/zh-CN/agent-view.md#from-inside-a-session)
* 如果输出超过 5GB，后台任务会自动终止，stderr 中会有说明原因的注释
* 在 macOS 和 Linux 上，当操作系统报告严重内存压力时，Claude Code 会停止运行中的后台任务，前提是会话已空闲至少 30 分钟且没有轮次或子代理正在运行
  * [调试日志](https://code.claude.com/docs/zh-CN/debug-your-config.md)说明了为什么任务被停止，或为什么压力事件让它们继续运行
  * 将 [`CLAUDE_CODE_DISABLE_BG_SHELL_PRESSURE_REAP`](https://code.claude.com/docs/zh-CN/env-vars) 设置为 `1` 可关闭内存压力停止
* 在您通过终端、桌面应用或 VS Code 扩展进行工作的本地会话中，后台命令没有时间限制。在无人值守运行的会话中（例如 `-p` 运行或云端会话），Claude Code 会在后台命令达到其[时间限制](https://code.claude.com/docs/zh-CN/tools-reference.md#time-limit-for-background-commands)时停止它
* 由前台[子代理](https://code.claude.com/docs/zh-CN/sub-agents.md#run-subagents-in-foreground-or-background)启动的后台命令在该子代理的运行结束时结束，无论是完成、失败还是被中断；请参阅工具参考中的[后台命令何时停止](https://code.claude.com/docs/zh-CN/tools-reference.md#when-a-background-command-stops)

要关闭后台命令，请将 [`CLAUDE_CODE_DISABLE_BACKGROUND_TASKS`](https://code.claude.com/docs/zh-CN/env-vars#variables) 环境变量设置为 `1`。设置后，您还将无法使用 `Ctrl+B` 快捷键和[其他后台功能](https://code.claude.com/docs/zh-CN/env-vars.md#variables)，例如[在后台运行 skill](https://code.claude.com/docs/zh-CN/skills.md#run-skills-in-a-subagent)。以 [bare 模式](https://code.claude.com/docs/zh-CN/headless.md#start-faster-with-bare-mode)启动具有相同效果。

**常见的后台命令：**

* 构建工具（webpack、vite、make）
* 包管理器（npm、yarn、pnpm）
* 测试运行器（jest、pytest）
* 开发服务器
* 长时间运行的进程（docker、terraform）

### 使用 `!` 前缀的 Shell 模式

通过在输入前加 `!` 前缀直接运行 shell 命令，无需通过 Claude：

```bash theme={null}
! npm test
! git status
! ls -la
```

Shell 模式：

* 将命令及其输出添加到对话上下文
* 显示实时进度和输出
* 支持相同的 `Ctrl+B` 后台运行，用于长时间运行的命令
* 不需要 Claude 解释或批准命令
* 支持基于历史的自动完成：输入部分命令并按 `Tab` 从当前项目中的前面 `!` 命令完成
* 支持实时文件路径自动完成：输入包含正斜杠的 token，例如 `./src/` 或 `~/`，查看匹配文件和目录的下拉列表，然后按 `Tab` 接受。在 Windows 上也使用正斜杠；下拉列表由 `/` 触发，而不是 `\`
* 在空提示上按 `Escape`、`Backspace` 或 `Ctrl+U` 退出
* 将以 `!` 开头的文本粘贴到空提示中会自动进入 shell 模式，与输入的 `!` 行为匹配

除非您的会话是[严格沙箱模式](https://code.claude.com/docs/zh-CN/sandboxing.md#turn-off-the-retry-with-strict-sandbox-mode)下列出的会话之一，即使您已启用沙箱隔离，您在 shell 模式中输入的命令也会在[沙箱](https://code.claude.com/docs/zh-CN/sandboxing.md)外运行，因为沙箱适用于 Claude 运行的命令。

一旦命令输出出现在会话记录中，Claude 会自动对其进行回复，因此您可以运行 `! npm test` 并获得失败原因的解释，无需再发送提示词。该回复的成本与发送普通提示词相同。若要改为仅将输出添加到上下文而不回复，请在 `settings.json` 中将 [`respondToBashCommands`](https://code.claude.com/docs/zh-CN/settings-reference#respondtobashcommands) 设置为 `false`。

## 在 Claude 工作时排队消息

在 Claude 工作时输入消息并按 `Enter`。Claude Code 会将消息排队而不是中断当前轮次，并在对话中列出排队的条目，直到发送它们。您可以以相同的方式排队 `!` [shell 命令](#shell-mode-with-prefix)和大多数[命令](https://code.claude.com/docs/zh-CN/commands.md)，除了 `/status` 等 Claude Code 在您发送时立即运行的命令。

已发送和排队的消息在 Claude 开始响应之前以灰色显示，因此您可以看出 Claude 还没有开始处理哪些消息。

如果您排队的消息附带了来自[连接的 IDE](https://code.claude.com/docs/zh-CN/vs-code.md#the-built-in-ide-mcp-server)的选择，它会保留您按 `Enter` 时的选择，无论您之后选择什么。

### Claude Code 何时发送您排队的内容

排队条目何时到达 Claude 取决于您排队的内容。

* 消息：如果您在 Claude 运行工具调用时排队消息，Claude Code 会在这些工具调用完成后立即将其传递给 Claude，在同一轮次内。当轮次以仍有排队消息结束时，它们会在没有另一次按键的情况下按您输入的顺序发出
* 命令和 shell 命令：Claude Code 将其保留到轮次结束，然后逐个运行它们，保持您排队的顺序

要在不等待的情况下发送您排队的内容，请按 `Ctrl+Enter`。您排队的消息会立即发出，如果您输入了草稿，您的草稿会排在它们后面。需要 Claude Code v2.1.275 或更高版本。

如果您在消息前排队了 `!` shell 命令，该快捷键会中断轮次。否则，轮次发生的情况取决于按下快捷键时 Claude 正在做什么：

* 运行 shell 命令、子代理或其他可以移到[后台](#background-bash-commands)的工作：该工作移到后台并继续运行，Claude 在同一轮次中读取您的消息
* 仅撰写回复，或运行无法移到后台的内容：Claude Code 中断轮次并接下来发送您的消息。在 v2.1.281 之前，该快捷键在两种情况下都中断轮次

在 [shell 模式](#shell-mode-with-prefix)中，该快捷键仅排队您的命令。在不报告扩展键的终端中，`Ctrl+Enter` 作为普通 `Enter` 到达并排队草稿；`Ctrl+X Ctrl+S` 在任何终端中都有效。两个快捷键都是 [`chat:sendNow` 操作](https://code.claude.com/docs/zh-CN/keybindings#chat-actions)的绑定。

按 `Esc` 中断轮次而不提交您的草稿。Claude Code 保留您排队的内容并立即发送。

Claude Code 在您发送某些命令时立即运行它们，而不是排队它们，其中包括 `/model`、`/effort` 和 `/fast`。这三个命令各改变一个设置：模型、effort 级别或快速模式。Claude Code 是将新设置应用于 Claude 已在处理的轮次，还是仅从您的下一轮次应用，因命令而异：

* [`/model`](https://code.claude.com/docs/zh-CN/model-config#setting-your-model)：一旦您确认[缓存警告](https://code.claude.com/docs/zh-CN/prompt-caching.md#switching-models)（如果 Claude Code 显示），Claude Code 会从该轮次中它发出的下一个请求开始使用新模型
* `/effort`：您发送的下一条消息将以新级别运行。要[在 Claude 已在处理的轮次中启用新级别](https://code.claude.com/docs/zh-CN/model-config.md#when-a-new-effort-level-starts)，请在 Claude 运行工具调用时排队一条消息
* [`/fast`](https://code.claude.com/docs/zh-CN/fast-mode#toggle-fast-mode)：Claude Code 保持轮次开始时活跃的快速模式设置，因此您的速度更改从您的下一轮次应用。如果您当前的模型不支持快速模式，打开它也会[切换您的模型](https://code.claude.com/docs/zh-CN/prompt-caching.md#turning-on-fast-mode)，Claude Code 会在该轮次中从其下一个请求使用新模型

### 在有排队消息时将会话转入后台

当您[按 `←`](https://code.claude.com/docs/zh-CN/agent-view#switch-sessions-without-leaving-the-terminal) 后会话移到后台时，您排队的消息会随之移动。Claude 会在那里按您发送的顺序读取它们。

命令和 `!` shell 命令无法移动，某些消息也无法移动，例如附带了 [IDE 选择](https://code.claude.com/docs/zh-CN/vs-code.md#the-built-in-ide-mcp-server)的消息。如果您在其中任何一项处于排队状态时按 `←`，会话会保留在前台，并出现一条以 `Cannot open agents` 开头的通知。要将会话转入后台，请将该条目移出队列，然后再次按 `←`：

* **命令和 `!` shell 命令**：等待它们在轮次结束时运行，或[将其取回](#take-back-what-you-queued)并清除输入框。当有其他任何内容排队时，您无法取回 `!` shell 命令，因此请等待它们运行。
* **无法移动的消息**：等待 Claude 读取它们

### 取回您排队的内容

从输入框的第一行按 `Up` 以取回排队的消息和命令。Claude Code 将其从队列中移除并将其放在输入框中，每行一个，位于您输入的任何文本之前。编辑文本并按 `Enter` 以将其再次排队为一个条目，或清除输入框以丢弃它。

Claude Code 仅在输入框为空且您没有其他排队内容时才取回排队的 shell 命令，并在执行此操作时将输入框切换到 shell 模式。否则，它会将其保留在队列中，用其 `!` 前缀列出，并在轮次结束后运行它们。

如果您在 `←` [等待将会话转入后台](https://code.claude.com/docs/zh-CN/agent-view.md#switch-sessions-without-leaving-the-terminal)时取回排队的文本，文本会保留在输入框中，Claude Code 会取消切换。如果您恰好在会话移动的那一刻取回文本，文本会随前台屏幕一起消失：它并未被发送。您取回的每条消息都会作为单独的条目保存在[命令历史记录](#command-history)中。要恢复某条消息，请重新打开该会话，并在提示词为空且没有排队内容时按 `Up`。

## 提示建议

当你首次打开一个会话时，Claude Code 会在提示输入框中显示一个灰显的示例命令来帮助你开始。它从你的项目的 git 历史记录中选择这个示例，所以该示例反映了你最近一直在处理的文件。

在 Claude 响应后，Claude Code 可以根据你的对话历史建议你的下一个提示，例如多部分请求的后续步骤或你工作流程的自然延续。

* 按 `Tab` 或 `Right arrow` 将建议放入提示输入框，然后按 `Enter` 提交
* 开始输入以关闭它

Claude Code 使用后台请求生成这些下一个提示建议，该请求使用你的会话正在使用的同一模型。该请求计入你的计划使用限制或你的 API 成本。因为它重用对话的提示缓存，所以主要是缓存读取加上少量输出令牌，因此额外成本很小。

### 当 Claude Code 跳过建议时

在交互模式下，Claude Code 默认关闭提示建议，并在 `/config` 中隐藏 **Prompt suggestions** 切换，在[不获取功能标志的会话](https://code.claude.com/docs/zh-CN/env-vars.md#features-that-need-feature-flag-fetching)中（例如在第三方提供商上或通过 Claude 应用网关），以及在[安装或升级后的首个会话](https://code.claude.com/docs/zh-CN/env-vars.md#first-session-after-an-install-or-upgrade)中（其标志尚未到达）。

Claude Code 还在多种情况下跳过单个建议，包括：

* 提示缓存为冷状态，以避免不必要的成本
* 在对话的第一轮之后，在某些会话中
* 前一个响应以错误结束
* 当你处于 Plan Mode 时
* 您的账户接近或已达到用量限制。要使此跳过在达到限制之前不生效，请将 [`CLAUDE_CODE_ENABLE_PROMPT_SUGGESTION`](https://code.claude.com/docs/zh-CN/env-vars) 设置为 `true`。在 v2.1.238 之前，即使将该变量设置为 `true`，Claude Code 也会在接近限制时跳过建议
* 在[代理团队](https://code.claude.com/docs/zh-CN/agent-teams.md)中，默认情况下在队友的会话中。主导的会话显示建议

提示 `Showing fewer prompt suggestions · use one to bring them back` 表示由于您连续多次未使用建议，Claude Code 正在降低显示建议的频率。要恢复正常频率，请使用一个建议，或将 [`CLAUDE_CODE_ENABLE_PROMPT_SUGGESTION`](https://code.claude.com/docs/zh-CN/env-vars) 设置为 `true`。

在打印模式下，Claude Code 默认不生成建议。使用 [`--prompt-suggestions`](https://code.claude.com/docs/zh-CN/cli-reference#cli-flags) 与 `-p "<prompt>" --output-format stream-json --verbose` 一起传递，以使 Claude Code 在生成建议的每一轮之后发出 `prompt_suggestion` 消息。生成器在这里也会跳过非常短的对话和冷提示缓存，因此单个短的 `-p` 查询可能不会发出任何建议。

### 关闭提示建议

要完全禁用提示建议，请使用以下任何一种方法：

* 在 `/config` 中关闭 **Prompt suggestions**
* 在你的设置文件中将 [`promptSuggestionEnabled`](https://code.claude.com/docs/zh-CN/settings-reference#promptsuggestionenabled) 设置为 `false`
* 将 [`CLAUDE_CODE_ENABLE_PROMPT_SUGGESTION`](https://code.claude.com/docs/zh-CN/env-vars) 环境变量设置为 `false`，它优先于设置：
  ```bash theme={null}
  export CLAUDE_CODE_ENABLE_PROMPT_SUGGESTION=false
  ```

要在整个组织范围内关闭提示建议，请在[托管设置](https://code.claude.com/docs/zh-CN/managed-settings.md)中将 `promptSuggestionEnabled` 设置为 `false`。还要在托管的 [`env`](https://code.claude.com/docs/zh-CN/settings-reference#env) 键下将 `CLAUDE_CODE_ENABLE_PROMPT_SUGGESTION` 设置为 `false`，以便用户无法使用自己的环境变量重新启用它们。

## You should know 建议

"You should know" 会寻找一件您应该知道但可能遗漏了的事情，例如 Claude 做出了某项权衡却没有明确指出，或者某个结果可能有误。找到后，它会在提示词输入框上方显示一条简短的提示：

```text theme={null}
✦ Heads up · There was a change to how folder sharing starts in June, which may have bumped usage metrics artificially.
  1: Learn more   2: Knew this already   0: Dismiss
```

它是 Claude Code 的[内置 mod](https://code.claude.com/docs/zh-CN/plugins/mods/overview.md#mods-built-into-claude-code) 之一，即内置于 Claude Code 中、默认关闭的插件。需要 Claude Code v2.1.287 或更高版本。

启用后，当 Claude 处理较长的任务时，一个辅助 Agent 会不时重新阅读对话。辅助 Agent 是发送给同一模型的独立请求，且没有任何工具，因此它无法读取文件、运行命令或改变 Claude 的行为。

如果您的组织可以使用该 mod，那么当您使用 claude.ai 账户或 Console API 密钥直接连接 Claude API 时，`/plugin` 会列出它。当您通过 [LLM 网关](https://code.claude.com/docs/zh-CN/llm-gateway)或 [Claude apps 网关](https://code.claude.com/docs/zh-CN/claude-apps-gateway)连接，或者从 [`apiKeyHelper`](https://code.claude.com/docs/zh-CN/settings-reference#apikeyhelper) 获取密钥时，则不会列出。即使是直接连接，以下会话也不会列出它：

* [不获取该 mod 所依赖的功能标志](https://code.claude.com/docs/zh-CN/env-vars.md#features-that-need-feature-flag-fetching)的会话，例如设置了 `DISABLE_TELEMETRY` 的会话
* 功能标志尚未送达的[安装或升级后的首个会话](https://code.claude.com/docs/zh-CN/env-vars.md#first-session-after-an-install-or-upgrade)

要启用它，请在 Claude Code 提示符处运行：

```text theme={null}
/plugin enable cc-plugin-you-should-know@builtin
```

在 `/plugin` 未列出该 mod 的情况下，启用命令会返回 `Plugin "cc-plugin-you-should-know@builtin" is not installed in this project`。

### 检查的成本

该 mod 启用期间，检查是发送给当前会话所用同一模型的后台请求，会计入您套餐的用量限制或您的 API 费用。每次检查都会发送 Claude 上下文中已有的内容，即截至目前的对话及其工具结果，随后是该 mod 自身的指令以及您最近提示的文本。对话部分会按照与 Claude 自身下一个请求相同的条件从会话的[提示缓存](https://code.claude.com/docs/zh-CN/prompt-caching.md)中读取。指令部分（长达数千个 token）以及辅助 Agent 的回复在每次检查时都会全额计费。

要观察检查的发生过程，请在启动 Claude Code 之前在 shell 中设置 [`CLAUDE_CODE_YOU_SHOULD_KNOW_DEBUG=1`](https://code.claude.com/docs/zh-CN/env-vars)。

### 响应提示

在空的提示词输入框中单独输入选项对应的数字。提示提供以下选项：

* **Learn more**：在提示下方打开一段简短说明，该说明有自己的选项：
  * **Understood**：关闭说明并将该主题标记为已知
  * **Chat in main session**：将提示引用到您的提示词输入框中，以便您添加问题并发送给 Claude
  * **Dismiss**：关闭说明，但不将该主题标记为已知
* **Knew this already**：关闭提示并将该主题标记为已知。
* **Dismiss**：关闭提示。

除非您通过 **Chat in main session** 发送提示，否则主 Agent 看不到它。

Claude Code 会将您最近的提示以及您标记为已知的主题保存在 `~/.claude/plugins/store/` 下一个名称以 `cc-plugin-you-should-know` 开头的文件中。在本会话及之后的会话中，每次检查都会将它们列给辅助 Agent，作为不应重复的内容。当该文件超过 [`cleanupPeriodDays`](https://code.claude.com/docs/zh-CN/settings-reference#cleanupperioddays) 没有被任何会话读取或写入时，Claude Code 会将其删除。要更早清除它们，请在没有会话运行时删除该文件。

### 关闭 You should know

要关闭 You should know，可以使用以下任一方式：

* 在 Claude Code 提示符处运行 `/plugin disable cc-plugin-you-should-know@builtin`，或在 `/plugin` 的 **Installed** 标签页中将其关闭
* 在未先打开 **Learn more** 的情况下对提示选择 **Dismiss** 后，在 `Dismissed.` 行上选择 **Turn off suggestions**，该行会在 20 秒后消失

## 表情符号速记代码

在提示词输入框中输入 `:` 后跟表情符号速记代码来插入表情符号。

* 输入完整的速记代码，例如 `:heart:`，Claude Code 会在您输入结束的 `:` 时立即将其替换为 ❤️
* 输入 `:` 加上名称的至少两个字符，例如 `:hea`，打开建议弹出窗口，然后按 `Tab` 或 `Enter` 插入突出显示的表情符号

速记代码必须在输入的开头或空格后开始，因此单词或 URL 内的 `:` 不会打开建议。

要关闭此功能，请在 `settings.json` 中将 [`emojiCompletionEnabled`](https://code.claude.com/docs/zh-CN/settings-reference#emojicompletionenabled) 设置为 `false`。这会禁用建议弹出窗口和内联替换。

## 在输入时检查拼写

Claude Code 可以在您输入时在提示输入框中为拼写错误的单词加下划线。它仅检查输入框中的文本，从不检查 Claude 的回复或您的文件。它也不会在输入框处于[shell 模式](#shell-mode-with-prefix)、`Ctrl+R` 历史搜索或[语音听写](https://code.claude.com/docs/zh-CN/voice-dictation.md)时检查任何内容。

拼写检查默认关闭，Claude Code 在[屏幕阅读器模式](https://code.claude.com/docs/zh-CN/accessibility.md)下不检查任何内容。需要 Claude Code v2.1.235 或更高版本。

### 前置条件

* 安装 [aspell](https://github.com/GNUAspell/aspell)、[hunspell](https://github.com/hunspell/hunspell) 或 [ispell](https://en.wikipedia.org/wiki/Ispell)，并确保它在您的 `PATH` 中。Claude Code 在每个平台上运行它找到的前三个中的第一个，包括包管理器在 Windows 上安装的 `.cmd` shim。
* 要检查程序是否在您的 `PATH` 中，请在您的终端中运行 `aspell --version`、`hunspell --version` 或 `ispell -v`。"command not found" 错误意味着它还不在您的 `PATH` 中。

### 打开或关闭拼写检查

Claude Code 从三个地方读取 [`spellcheck`](https://code.claude.com/docs/zh-CN/settings-reference#spellcheck) 设置，并在项目的 `.claude/settings.json` 和 `.claude/settings.local.json` 中忽略它。从您使用的任何一个地方打开它：

> [tab] 用户设置
>
> 将 `spellcheck` 添加到 `~/.claude/settings.json`。它适用于您打开的每个项目，就像您的其他[用户设置](https://code.claude.com/docs/zh-CN/settings.md#where-settings-live)一样：
>
> ```json theme={null}
> {
>   "spellcheck": { "enabled": true }
> }
> ```

> [tab] 命令行
>
> 将 `spellcheck` 保存在 JSON 文件中，例如 `spellcheck.json`：
>
> ```json theme={null}
> {
>   "spellcheck": { "enabled": true }
> }
> ```
>
> 然后将文件传递给 `--settings`。它仅适用于该会话：
>
> ```bash theme={null}
> claude --settings spellcheck.json
> ```

> [tab] 托管设置
>
> 将 `spellcheck` 添加到您的组织的[托管设置源](https://code.claude.com/docs/zh-CN/permissions.md#managed-settings)之一。它适用于接收这些设置的每个用户，他们无法关闭它：
>
> ```json theme={null}
> {
>   "spellcheck": { "enabled": true }
> }
> ```

要检查拼写检查是否打开，请输入一个拼写错误的单词和一个空格。Claude Code 会为该单词加下划线。如果没有，请参阅[当 Claude Code 不加下划线时](#when-claude-code-underlines-nothing)。要再次关闭拼写检查，请在同一位置将 `enabled` 设置为 `false`，或删除 `spellcheck`。

要选择 Claude Code 运行的三个程序中的哪一个、它使用的字典或下划线颜色，请在同一位置在 `enabled` 旁边添加以下任何字段：

* `checker`：`aspell`、`hunspell` 或 `ispell`。Claude Code 不会从您命名的检查器回退，并将任何其他值视为 `auto`。
* `language`：您的检查器形式中的字典名称，例如 `en_GB`。Claude Code 忽略任何不是纯字典名称的值，例如路径或包含空格的名称，检查器使用其默认字典。
* `color`：颜色名称，例如 `yellow`，或 `#rrggbb`、`#rgb`、`rgb(r,g,b)`、`ansi256(n)` 或 `ansi:<name>` 值。Claude Code 默认使用您的主题的错误颜色，对于任何它不识别的值也是如此。

例如，此 `spellcheck` 设置运行 hunspell 及其 `en_GB` 字典，并以黄色为单词加下划线。它在 `~/.claude/settings.json`、您传递给 `--settings` 的文件和托管设置中的工作方式相同：

```json theme={null}
{
  "spellcheck": {
    "enabled": true,
    "checker": "hunspell",
    "language": "en_GB",
    "color": "yellow"
  }
}
```

如果三个地方中有多个具有 `spellcheck` 设置，Claude Code 仅使用其中一个：首先是托管设置，然后是 `--settings`，然后是用户设置。它不会组合来自两个地方的字段。例如，当 `--settings` 设置 `spellcheck` 时，您的用户设置中的 `language` 无效。

### Claude Code 加下划线的内容

在您暂停输入后不久，Claude Code 会为字典不知道的单词加下划线。它会将您仍在输入的单词单独留下，直到您越过它，并且它永远不会更改您的文本。它也会跳过看起来像代码的文本：

* 命令，例如 `/help`、`@` 提及、URL、文件路径和标志，例如 `--verbose`
* 包含数字、下划线或第一个字母后的大写字母的单词，以及反引号中的文本

Claude Code 也会跳过中文、日文、韩文、泰文、老挝文、高棉文和缅甸文文本。

Claude Code 没有自己的单词列表：当您的检查器说一个单词拼写错误时，它就是拼写错误的。要停止 Claude Code 为单词加下划线，请按照检查器自己的文档将该单词添加到您的检查器的个人字典中。Claude Code 在您重新启动它后会获取新单词。

### 当 Claude Code 不加下划线时

当 Claude Code 无法保持检查器运行时，它不加下划线：

* 未安装检查器，或您在 `checker` 中命名的检查器丢失
* 检查器连续失败两次，在启动时或会话中稍后。Claude Code 在第一次失败后重新启动它，在第二次失败后停止检查，直到您重新启动 Claude Code
* 检查器需要超过 15 秒来回答，三次。每次，Claude Code 都会将它等待的单词保持未标记；在第三次之后，它停止检查，直到您重新启动 Claude Code

要找出发生了哪种情况，请使用 `claude --debug` 启动拼写检查并输入一个单词。然后在 `~/.claude/debug/<session-id>.txt` 的调试日志中查找 `[spellcheck]` 行。一行命名 Claude Code 启动的程序，或列出它查找但未找到的程序。后面的行说明它为什么停止。那里的缺少字典错误意味着检查器没有您的 `language` 值的字典，或当 `language` 未设置时没有默认字典。安装一个，或将 `language` 设置为您拥有的字典。

## 提示词中的隐形字符

粘贴的文本可能包含 Unicode 字符，这些字符在终端中显示为空白，例如标签字符、双向控制字符和零宽空格，因此提示词可能包含您看不到的文本。为了防止复制的文本携带终端无法显示的指令，Claude Code 在您按 Enter 时会移除这些字符，然后再发送任何内容。它会清理提示词和提示词包含的任何折叠的[粘贴文本引用](https://code.claude.com/docs/zh-CN/terminal-config.md#paste-large-content)的内容。Claude Code 会保留波斯语和印度文字使用的连接符以及表情符号序列中的选择器。

如果 Claude Code 移除了任何内容，该 Enter 将不发送任何内容。清理后的提示词会返回到输入框中，并显示类似 `Removed 3 invisible characters · review and press Enter to send` 的通知，再次按 Enter 会发送显示的文本。

当您在命令行上传递提示词时，例如 `claude "fix the login bug"`，Claude Code 不会等待第二次 Enter。它会移除这些字符，显示通知，并发送清理后的提示词。如果清理后的提示词将以 `/` 开头，Claude Code 会改为将其放在输入框中供您审查和发送。

## 使用 /diff 查看更改

运行 `/diff` 可以在不离开 Claude Code 的情况下查看工作树中的更改。您可以看到 Claude 迄今为止所做的编辑以及您尚未提交的任何其他内容。`/diff` 打开的内容取决于当前启用的渲染器：

* **[全屏渲染](https://code.claude.com/docs/zh-CN/fullscreen.md)**：[diff 面板](#diff-panel)会在对话旁边打开。该面板保持打开状态，并在您继续工作时更新。
* **经典渲染器**：[diff 对话框](#diff-dialog)会在输入框上方打开，您阅读完后可以关闭它。

面板和对话框都来自 `cc-plugin-diff`，它是 [Claude Code 内置的 mod](https://code.claude.com/docs/zh-CN/plugins/mods/overview.md#mods-built-into-claude-code) 之一。如果您在 `/plugin` 中禁用该 mod，`/diff` 会改为打开 Claude Code 早期的面板和 [diff 查看器](https://code.claude.com/docs/zh-CN/keybindings.md#diff-actions)。

在 `/diff` 从 git 读取的更改中，子模块显示为单个条目，仅当它指向的提交发生更改时才会出现；对子模块内文件的编辑不会显示在那里。一旦 Claude 编辑了文件，面板和对话框还会提供每个轮次编辑的视图。这些轮次视图来自 Claude 的文件编辑而不是 git，因此 Claude 通过 shell 命令所做的更改仅显示在 `Current`（即工作树视图）下。

### Diff panel

diff 面板列出了更改的文件及其添加和删除的行数，并在列表下方显示每个文件的 diff。Claude Code 在 Claude 编辑文件或运行 shell 命令时刷新它。要关闭面板，请再次运行 `/diff` 或单击其标题中的 `✕`。

要使用该面板，您需要：

* [全屏渲染](https://code.claude.com/docs/zh-CN/fullscreen.md)
* 一个 git 仓库
* 至少 110 列宽的终端
* Claude Code v2.1.287 或更高版本

一旦 Claude 开始编辑文件，如果您的终端至少 144 列宽，该面板也会自动打开。在您自己使用 `/diff` 打开它后，后续会话会在 Claude 在任何足够宽的终端中编辑文件时立即打开它。关闭面板后，它在此会话和后续会话中保持关闭状态，直到您再次运行 `/diff`。

当面板打开时，您可以：

* **跳转到文件**：单击列表中的其行。使用鼠标滚轮滚动面板。当文件列表本身太长无法容纳时，使用 `Alt+Up` 和 `Alt+Down` 或 `Ctrl+Up` 和 `Ctrl+Down` 滚动它。
* **询问 Claude 关于某个文件的更改**：单击文件 diff 上方文件名右侧的 `ask`。Claude Code 会将该文件的 diff 附加到您的下一个提示词，在您发送该提示词之前，按钮会显示为 `asked ✓`。对第二个文件进行询问会替换第一个。
* **显示某个轮次的编辑**：单击面板标题中的 `source` 选择器，然后使用 `Up` 和 `Down` 选择一个轮次并按 `Enter`。轮次标记为 `T1`、`T2` 等。该选择器在 Claude 编辑文件后出现，选择 `Current` 可返回工作树。
* **显示面板遗漏的文件**：列表跳过测试文件和生成的文件，并将此会话之前的更改折叠为底部的一行。单击任一计数行以展开它。
* **更改面板比较的内容**：按 `Ctrl+X B` 在此会话的更改、您的未提交更改作为一个列表，以及自您的分支从默认分支分离以来的所有内容之间循环。Claude Code 为每个仓库记住该选择。

要重新绑定用于滚动文件列表或更改比较内容的快捷键，请参阅 [Diff panel actions](https://code.claude.com/docs/zh-CN/keybindings.md#diff-panel-actions)。

在未完成的合并、rebase、cherry-pick 或 revert 期间，面板会继续显示它上次从 git 读取的更改，因此不会被该操作传入的更改填满。操作结束后，面板会在第一次刷新时更新，例如在 Claude 下次编辑文件时。

### Diff dialog

diff 对话框以带边框的区块形式在输入框上方打开，列出您更改的文件及其添加和删除的行数。它将这些文件与 `HEAD` 进行比较，或者与您上次为此仓库[在 diff 面板中选择](#diff-panel)的内容进行比较。

一旦 Claude 编辑了文件，列表上方会出现一个 `source` 选择器。对于每个促使 Claude 编辑文件的提示词，它都会提供一个轮次视图，标记为 `T1`、`T2` 等。轮次视图仅显示该轮次的编辑，选择 `Current` 可返回工作树。

在对话框中使用这些快捷键：

* **上和下**：选择一个文件。
* **Enter**：打开所选文件的 diff。使用上和下或 PageUp 和 PageDown 滚动它。
* **Esc**：从文件的 diff 返回到列表，或从列表关闭对话框。
* **Tab**：移动到 `source` 选择器，然后使用上和下选择 `Current` 或某个轮次视图并按 Enter。在文件的 diff 中，Tab 会移动到 `ask` 按钮。按 Enter 可将该文件的 diff 附加到您的下一个提示词。

## 使用 /btw 提出附加问题

使用 `/btw` 提出关于当前工作的问题，而不将其添加到对话历史记录中。

```
/btw what was the name of that config file again?
```

Claude 从对话中已有的内容回答附加问题：你的消息、它的回复以及它收集的工具结果。你可以询问 Claude 已经读过的代码、它之前做出的决定，或会话中的任何其他内容。后来的附加问题也会看到你之前的附加问题：Claude Code 在每次提问时重放最新的 20 个交换，直到你清除它们。问题和答案永远不会进入对话历史记录。在终端中，它们以可关闭的覆盖层形式出现。终端将线程保存在内存中：按 `x` 清除之前的交换，退出 Claude Code 时它就消失了。

在 [VS Code 扩展](https://code.claude.com/docs/zh-CN/vs-code.md#use-the-prompt-box)的聊天面板中，`/btw` 打开一个面板而不是本节描述的覆盖层，你可以直接在面板中提出后续问题。该面板的线程在窗口重新加载后仍然存在，遵循该页面描述的保留计划。你需要 v2.1.227 或更高版本的扩展。早期的扩展版本不提供 `/btw`。

* **Claude 工作时可用**：即使 Claude 正在处理响应，你也可以运行 `/btw`。附加问题独立运行，不会中断主要回合。它可以看到到目前为止对话中的所有内容，除了 Claude 仍在编写的回复。
* **无工具访问**：附加问题仅从上下文中已有的内容回答。Claude 在回答附加问题时无法读取文件、运行命令或搜索。如果 Claude 无论如何都将工具调用写成文本，答案会以一条注释结尾，说明没有执行任何操作。
* **单一响应**：覆盖层中没有后续回合。要继续线程，请提出另一个 `/btw` 问题。要在本地会话中继续使用完整的工具访问，按 `f` 将此问题和答案分叉到 [后台子代理](https://code.claude.com/docs/zh-CN/sub-agents.md#fork-the-current-conversation)。
* **低成本**：当对话的 [prompt cache](https://code.claude.com/docs/zh-CN/prompt-caching.md) 预热时，附加问题的成本仅略高于答案本身。

你最新的五个之前的附加问题以暗淡的列表形式出现在当前答案上方，并显示任何较旧问题的计数。它们不会进入对话历史记录。

要在关闭覆盖层后返回到它，运行不带问题的 `/btw`。覆盖层在您最近的交换处重新打开。

答案出现后，覆盖层接受这些按键。

| 按键 | 操作 |
| :- | :- |
| `Space`、`Enter`、`Escape` | 关闭答案并返回到提示符 |
| `Up` / `Down` | 滚动答案 |
| `Shift+Left` / `Shift+Right` | 在此答案和你之前的 `/btw` 答案之间步进。`Shift+Left` 移动到较旧的答案，`Shift+Right` 返回到当前答案。`[` 和 `]` 执行相同操作，适用于不报告 `Shift` 与箭头键的终端。`Tab` / `Shift+Tab` 循环通过相同的答案。需要 Claude Code v2.1.257 或更高版本。在 v2.1.187 和 v2.1.256 之间，按键是普通的 `Left` / `Right` |
| `c` | 将答案作为原始 Markdown 复制到剪贴板。使用此方法而不是鼠标选择，后者会捕获硬换行的终端呈现而不是源文本 |
| `f` | 启动 [分叉的子代理](https://code.claude.com/docs/zh-CN/sub-agents.md#fork-the-current-conversation)，它继承父对话加上此问题和答案，以便它可以继续使用完整的工具访问。你留在当前会话中，并在 [提示符下方的面板](https://code.claude.com/docs/zh-CN/sub-agents.md#observe-and-steer-running-forks)中找到分叉。仅在本地会话中可用 |
| `x` | 清除当前答案上方显示的之前 `/btw` 交换的列表 |

在附加的 [后台会话](https://code.claude.com/docs/zh-CN/agent-view.md#attach-to-a-session)中，`Left` 分离并将你返回到代理视图，即使答案仍在到达。附加问题在你离开时继续运行。下次你附加到会话时，覆盖层会重新打开，显示附加问题或其答案。在 v2.1.257 之前，`Left` 在那里不分离。

`/btw` 可以看到你的完整对话，但没有工具。[子代理](https://code.claude.com/docs/zh-CN/sub-agents.md)有工具，从它接收的提示开始，或者对于 [分叉](https://code.claude.com/docs/zh-CN/sub-agents.md#fork-the-current-conversation)，从此对话的副本开始。使用 `/btw` 询问 Claude 从此会话中已知的内容；使用子代理去发现新的东西。

## 任务列表

任务列表是 Claude 的待办事项清单：Claude 创建的用于规划多步骤工作的项目，带有指示器显示待处理、进行中或已完成的状态。它与后台任务视图分开。要查看运行中的 shell 和子代理，请改用 [`/tasks`](https://code.claude.com/docs/zh-CN/commands)。

该列表仅在具有任务跟踪工具的会话中填充，Claude Code 在 [Opus 4.7、Sonnet 4.6、Haiku 4.5 及更早的模型](https://code.claude.com/docs/zh-CN/tools-reference.md#task-tool-availability) 上默认提供。在任何其他模型上，包括 Claude Code 无法识别的模型 ID，除非您使用 `CLAUDE_CODE_ENABLE_TODO_TOOLS=1` 或 [Task 工具可用性](https://code.claude.com/docs/zh-CN/tools-reference.md#task-tool-availability) 下的其他方式选择加入，否则列表保持为空。当会话具有这些工具时，任务列表的工作方式如下：

* 按 `Ctrl+T` 切换任务列表视图。显示一次最多显示五个任务。当 Claude 尚未创建任何清单项目时，切换没有可见效果，因为没有要显示的内容
* 如果您保持列表展开，Claude Code 会在下次启动仍有任务的会话时恢复展开视图，例如使用 `--resume` 或 `--continue`。当任务列表为空时，Claude Code 会将其启动为折叠状态
* 要查看所有任务或清除它们，直接询问 Claude："show me all tasks"（显示所有任务）或 "clear all tasks"（清除所有任务）
* 任务在上下文压缩中保持，帮助 Claude 在较大的项目中保持组织
* 要在会话间共享任务列表，设置 `CLAUDE_CODE_TASK_LIST_ID` 以使用 `~/.claude/tasks/` 中的命名目录：`CLAUDE_CODE_TASK_LIST_ID=my-project claude`

## 会话回顾

当你离开终端后返回时，Claude Code 会显示一行简短的回顾，说明到目前为止会话中发生了什么。一旦距离上次完成的轮次至少过了三分钟，且终端处于未聚焦状态，回顾就会在后台生成，这样当你切换回来时就已准备好。只有当会话至少有三个轮次时，回顾才会出现，且永远不会连续出现两次。

运行 `/recap` 可按需生成摘要。它仅在您亲自请求时运行。当它出现在从 Slack、Teams 或项目线程转发的消息中时，您会收到一条[通知](https://code.claude.com/docs/zh-CN/errors.md#recap-only-runs-when-you-ask-for-it-yourself)，而不是回顾。当 `/recap` 出现在云端定时任务发送的提示词中时，您也会收到该通知。

会话回顾在所有计划和提供商上默认启用。要关闭自动回顾，请打开 `/config` 并关闭**会话回顾**。自动回顾永远不会在非交互模式下出现。Claude Code 将自动回顾和 `/recap` 输出都限制在 400 个字符以内。

## 等待使用限制重置

当 claude.ai [使用限制](https://code.claude.com/docs/zh-CN/errors.md#youve-hit-your-session-limit) 在任务中途停止 Claude 时，Claude Code 会在打开的会话中等待，并在限制重置后自动继续该任务。在使用 claude.ai 订阅登录的交互式会话中，自动继续功能默认处于启用状态。需要 Claude Code v2.1.234 或更高版本。

Claude Code 等待时，会话底部的几行会显示您的限制何时重置以及 Claude 何时继续：

```text theme={null}
Usage limit reached · limit resets 3:45pm
Continuing automatically at 3:45pm · Esc to cancel
```

这两行在这些文字之后都可能带有更多内容，例如第一行中的帮助链接，或第二行中的 `/usage-credits to continue now`。当等待自动开始时，对话中也会记录一行 `Usage limit reached · continuing automatically at 3:45pm · esc to cancel`。

保持会话打开。接下来发生的情况取决于等待如何结束：

* **在重置时**：第二行变为 `Continuing shortly · Esc to cancel`。然后对话中出现 `Usage limit reset · Continuing automatically`，Claude Code 提示 Claude 从停止的地方继续任务。它不会重新发送您的最后一条消息。
* **计算机睡眠后**：如果睡眠超过约 30 分钟，并且限制在睡眠期间重置，第一行显示 `Your usage limit has reset`，第二行显示 `Press enter to continue`。按 `Enter` 继续。如果睡眠时间较短，或睡眠在重置之前结束，Claude Code 会自动继续。
* **提前**：当您使用 `/usage-credits` 完成添加 [使用额度](https://code.claude.com/docs/zh-CN/costs.md#add-usage-credits-to-your-subscription)、在 `/upgrade` 后重新登录或在等待期间使用 `/model` 切换模型时，Claude Code 会检查使用情况是否再次可用，如果可用则立即继续。它不会在您在浏览器中自行进行的升级或购买后进行检查。在 [`opusplan`](https://code.claude.com/docs/zh-CN/model-config#opusplan-model-setting) 和其他在不同模型上运行计划模式的模型设置下，Claude Code 会等待重置。

继续的任务像任何其他轮次一样运行。Claude Code 仍然照常要求 [权限](https://code.claude.com/docs/zh-CN/permissions.md)，因此任务可能在您离开时在提示处停止。如果再次达到限制，Claude Code 最多会自动重新启动等待两次，然后停止并显示 `Automatic continue stopped after repeated usage-limit hits · /rate-limit-options to try again`。

### 取消等待

在显示这些行时，于输入框为空时按 `Esc` 或按 `Ctrl+C`，或运行 [`/rate-limit-options`](https://code.claude.com/docs/zh-CN/commands#all-commands) 并选择 **Don't continue automatically**。Claude Code 会显示一行以 `Automatic continue cancelled` 开头的消息进行确认。

取消后，在您发送提示或再次从 `/rate-limit-options` 中选择以 **Wait here, then continue automatically** 开头的行之前，不会继续任何操作。Claude Code 不会为该重置窗口自动启动等待；下一个重置窗口会重新开始。

在这些情况下，等待也会在不继续任务的情况下结束：

* **您发送提示词**：Claude Code 发送您的提示词而不是等待。如果您的提示词也达到了限制，它会保留在对话中，并且 Claude Code 会重新开始等待。
* **您退出 Claude Code**：当您恢复会话时，等待不会重新启动。
* **对话转手**：您使用 `/login` 切换账户、清除或倒带对话、`/resume` 另一个会话、使用 `/teleport` 拉取一个会话、使用 `/tui` 或 [`/restart`](https://code.claude.com/docs/zh-CN/commands#all-commands) 重新启动，或将会话移至 Claude Desktop、后台会话或云端。
* **设置关闭，或重置超过 24 小时**：这仅结束 Claude Code 自动启动的等待。您从 `/rate-limit-options` 中选择的等待会继续倒计时。
* **继续被阻止**：一个阻止继续提示的 [`UserPromptSubmit` hook](https://code.claude.com/docs/zh-CN/hooks#userpromptsubmit)，或在到达模型之前的失败，会结束等待。Claude Code 会告诉您继续没有运行。发送提示以继续。

### 自己启动等待

Claude Code 在这些情况下不会自动启动等待：

* **Remote Control 和 agent team 队友会话**：该终端的人员仍然可以启动一个。
* **重置超过 24 小时**：每周限制可能在几天后重置。
* **您运行该系列之外的模型时的 Opus 或 Sonnet 限制**：您的下一轮可能不会达到该限制。[`opusplan`](https://code.claude.com/docs/zh-CN/model-config#opusplan-model-setting) 和其他在受限系列上运行计划模式的模型设置不会获得此例外。

在这些情况下，以及每当自动继续关闭时，当您在自己的终端达到限制时，Claude Code 会在每个重置窗口打开一次使用限制选项菜单。选择以 **Wait here, then continue automatically** 开头的行以启动等待。在 [Remote Control](https://code.claude.com/docs/zh-CN/remote-control.md) 或 [agent team](https://code.claude.com/docs/zh-CN/agent-teams.md) 队友会话中，自己运行 `/rate-limit-options` 以打开菜单。

Claude Code 在这些情况下根本不提供等待：

* **后台会话和 `-p` 运行**：菜单行不可用。
* **API 密钥、云提供商和基于使用情况的计费**：那里的使用情况按请求计量，因此没有重置可等待。
* **没有保存的 claude.ai 登录的 [LLM gateway](https://code.claude.com/docs/zh-CN/llm-gateway#subscriptions-and-gateways)**：Claude Code 仅在保存的 claude.ai 登录是活跃凭证时才提供等待。

### 关闭自动继续

在 `/config` 中，关闭 **Continue automatically at usage limit**，或在您的用户设置中将 [`autoContinueAtUsageLimit`](https://code.claude.com/docs/zh-CN/settings-reference#autocontinueatusagelimit) 设置为 `false`。`/config autoContinueAtUsageLimit=false` 也有效，包括使用 `-p`，但 `key=value` 形式无法将其重新打开，因为该设置授予无人值守执行。Claude Code 为此密钥读取的设置文件在 [settings reference](https://code.claude.com/docs/zh-CN/settings-reference.md#autocontinueatusagelimit) 中。

## PR 审查状态

在处理具有开放拉取请求的分支时，Claude Code 在页脚显示可点击的 PR 链接，例如"PR #446"。该链接有一个彩色下划线，指示审查状态：

* 绿色：已批准
* 黄色：待审查
* 红色：请求更改
* 灰色：草稿

拉取请求合并或关闭后，徽章消失。

`Cmd+click`（macOS）或 `Ctrl+click`（Windows/Linux）点击链接以在浏览器中打开拉取请求。

状态在 `git push` 或更改拉取请求的 `gh pr` 命令（例如 `gh pr create` 或 `gh pr merge`）在会话中成功后立即刷新。

Claude Code 将徽章呈现为超链接，即使它无法在您的终端中检测到超链接支持，这通常发生在 SSH 或 tmux 中。设置 [`FORCE_HYPERLINK=0`](https://code.claude.com/docs/zh-CN/env-vars) 以将徽章呈现为纯文本。

当您设置 [`CLAUDE_CODE_DISABLE_NONESSENTIAL_TRAFFIC`](https://code.claude.com/docs/zh-CN/env-vars) 时，Claude Code 不会检查拉取请求或合并请求状态。

> [!NOTE]
>
> GitHub 仓库的 PR 状态需要 GitHub 令牌。Claude Code 根据远程主机查找令牌：
>
> * **github.com**：`GH_TOKEN` 或 `GITHUB_TOKEN`，或由 `gh auth login` 保存的令牌。没有令牌时，当未安装 `gh` CLI 时页脚显示 `install gh for PR status`，或当已安装时显示 `gh auth login for PR status`
> * **设置为 `GH_HOST` 的 GitHub Enterprise 主机**：`GH_ENTERPRISE_TOKEN` 或 `GITHUB_ENTERPRISE_TOKEN`，或由 `gh auth login --hostname <host>` 保存的令牌。没有令牌时，页脚显示相同的提示
> * **任何其他 GitHub 主机**：由 `gh auth login --hostname <host>` 保存的令牌。没有令牌时，Claude Code 不显示徽章和提示

### GitLab 合并请求

当您在具有开放 GitLab 合并请求的分支上工作时，Claude Code 在页脚槽中显示可点击的 `MR !N` 徽章，该槽位通常保存 GitHub PR 链接。`!N` 是 GitLab 自己的合并请求编号 N 的参考语法。彩色下划线显示合并请求的状态：

* 绿色：GitLab 报告合并请求可合并
* 黄色：任何其他开放状态
* 灰色：草稿

合并请求合并或关闭后，徽章消失。

它在 `git push` 或更改合并请求的 `glab mr` 命令（例如 `glab mr create` 或 `glab mr merge`）在会话中成功后立即刷新。

要获取徽章，您需要：

* Claude Code v2.1.234 或更高版本
* 指向您的 GitLab 主机的仓库远程，可以是 gitlab.com 或自管理实例
* 您的 `PATH` 中的 [`glab` CLI](https://gitlab.com/gitlab-org/cli)，使用 `glab auth login` 进行身份验证

Claude Code 在检查状态时忽略 `glab` 的令牌环境变量（例如 `GITLAB_TOKEN`），因此您无法仅从导出的令牌获得徽章。Claude Code 还每个会话查找一次 `glab` 及其登录信息，因此在安装 `glab` 或运行 `glab auth login` 后重启 Claude Code。

## 问题参考链接

当 Claude 以 `owner/repo#123` 的形式提到一个问题时，只要您的终端支持超链接，您就可以点击该参考来打开它。如果 Claude Code 没有检测到您的终端支持超链接，请将 [`FORCE_HYPERLINK`](https://code.claude.com/docs/zh-CN/env-vars) 设置为 `1` 来启用链接，或设置为 `0` 来保持参考为纯文本。

只有两部分的 `owner/repo#123` 形式才会生成链接。以下情况保持为纯文本：

* 单独的 `#123`
* 嵌套的 GitLab 路径，例如 `group/subgroup/project#123`
* 代码跨度或代码块内的任何参考
* 长度超过约 1,000 行或 100,000 个字符的回复中的任何参考

Claude Code 根据它从您的 git remote 识别出的仓库主机来构建链接，而不是根据参考所指定的仓库：

| 您的仓库的主机 | `owner/repo#123` 链接到 |
| :- | :- |
| github.com、GitHub Enterprise 主机或下面未列出的任何主机 | `https://<host>/owner/repo/issues/123` |
| gitlab.com | `https://gitlab.com/owner/repo/-/issues/123` |
| bitbucket.org、codeberg.org 或 gitea.com | 无链接；参考保持为纯文本 |

## 另请参阅

* [Skills](https://code.claude.com/docs/zh-CN/skills.md) - 自定义提示和工作流
* [Checkpointing](https://code.claude.com/docs/zh-CN/checkpointing.md) - 回退 Claude 的编辑并恢复以前的状态
* [CLI 参考](https://code.claude.com/docs/zh-CN/cli-reference.md) - 命令行标志和选项
* [设置](https://code.claude.com/docs/zh-CN/settings.md) - 配置选项
* [内存管理](https://code.claude.com/docs/zh-CN/memory.md) - 管理 CLAUDE.md 文件
