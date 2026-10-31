> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# 自定义快捷键

> 使用快捷键配置文件在 Claude Code 中自定义快捷键。

Claude Code 支持可自定义的快捷键。运行 `/keybindings` 来创建或打开位于 `~/.claude/keybindings.json` 的配置文件。

## 配置文件

快捷键配置文件是一个包含 `bindings` 数组的对象。每个块指定一个上下文和一个按键映射到操作的映射。

> [!NOTE]
>
> 快捷键文件的更改会自动检测并应用，无需重启 Claude Code。

| 字段 | 描述 |
| :- | :- |
| `$schema` | 可选的 JSON Schema URL，用于编辑器自动完成 |
| `$docs` | 可选的文档 URL |
| `bindings` | 按上下文分组的绑定块数组 |

此示例将 `Ctrl+E` 绑定到在聊天上下文中打开外部编辑器，并取消绑定 `Ctrl+S`：

```json theme={null}
{
  "$schema": "https://www.schemastore.org/claude-code-keybindings.json",
  "$docs": "https://code.claude.com/docs/en/keybindings",
  "bindings": [
    {
      "context": "Chat",
      "bindings": {
        "ctrl+e": "chat:externalEditor",
        "ctrl+s": null
      }
    }
  ]
}
```

## 上下文

每个绑定块指定一个**上下文**，其中绑定适用：

| 上下文 | 描述 |
| :- | :- |
| `Global` | 在应用程序的任何地方应用 |
| `Chat` | 主聊天输入区域 |
| `Autocomplete` | 自动完成菜单打开 |
| `Settings` | 设置菜单 |
| `Confirmation` | 权限和确认对话框 |
| `Tabs` | 选项卡导航组件 |
| `Help` | 帮助菜单可见 |
| `Transcript` | 记录查看器 |
| `HistorySearch` | 历史记录搜索模式 (Ctrl+R) |
| `Task` | 任务在前台运行 |
| `ThemePicker` | 主题选择器对话框 |
| `Attachments` | 选择对话框中的图像附件导航 |
| `Footer` | 页脚指示器导航（任务、团队、差异、工件） |
| `MessageSelector` | 回退和总结对话框消息选择 |
| `DiffDialog` | [diff 查看器](#diff-actions)导航 |
| `DiffPanel` | [差异面板](https://code.claude.com/docs/zh-CN/interactive-mode.md#diff-panel)打开 |
| `ModelPicker` | 模型选择器工作量级别 |
| `EffortSlider` | 由 `/effort` 打开的工作量滑块 |
| `Select` | 通用选择/列表组件 |
| `Plugin` | Plugin 对话框（浏览、发现、管理） |
| `AbovePrompt` | [输入框上方的区域](#above-prompt-actions)或其中的按钮获得键盘焦点 |
| `AbovePromptInput` | 输入框上方区域或 mod 窗格中的输入字段获得键盘焦点 |
| `AbovePromptSelect` | 输入框上方区域或 mod 窗格中的选择框获得键盘焦点 |
| `Pane` | 由 [mod](https://code.claude.com/docs/zh-CN/plugins/mods/interface.md#know-which-keys-your-mod-can-receive) 绘制的窗格获得键盘焦点 |
| `PaneField` | mod 窗格中的输入字段或选择框获得键盘焦点 |
| `Agents` | [Agent 视图](https://code.claude.com/docs/zh-CN/agent-view.md)（`claude agents`） |
| `Scroll` | 对话滚动和全屏模式下的文本选择 |

在 v2.1.205 之前，`/doctor` 诊断屏幕存在 `Doctor` 上下文和 `doctor:fix` 操作。

## 可用操作

操作采用 `namespace:action` 格式，例如 `chat:submit` 用于发送消息，`app:toggleTodos` 用于显示任务列表。每个上下文都有特定的可用操作。

### 应用操作

`Global` 上下文中可用的操作：

| 操作 | 默认 | 描述 |
| :- | :- | :- |
| `app:interrupt` | Ctrl+C | 取消当前操作 |
| `app:exit` | Ctrl+D | 退出 Claude Code。在 800ms 内按两次以确认 |
| `app:redraw` | （未绑定） | 强制重绘终端 |
| `app:toggleTodos` | Ctrl+T | 切换 Claude 待办清单的显示。这不是 [`/tasks`](https://code.claude.com/docs/zh-CN/commands) 后台任务视图 |
| `app:toggleTranscript` | Ctrl+O | 切换详细会话记录 |

### 历史操作

用于浏览命令历史的操作：

| 操作 | 默认 | 描述 |
| :- | :- | :- |
| `history:search` | Ctrl+R | 打开历史搜索 |
| `history:previous` | Up | 上一个历史项 |
| `history:next` | Down | 下一个历史项 |

### 聊天操作

`Chat` 上下文中可用的操作：

| 操作 | 默认 | 描述 |
| :- | :- | :- |
| `chat:cancel` | Escape | 取消当前输入 |
| `chat:clearInput` | Ctrl+L | 强制全屏重绘，保留输入和对话 |
| `chat:clearScreen` | Cmd+K | 与 `chat:clearInput` 相同。有关 Cmd+K 在 iTerm2 和 Terminal.app 上的行为，请参阅[清除对话](https://code.claude.com/docs/zh-CN/fullscreen.md#clear-the-conversation) |
| `chat:killAgents` | Ctrl+X Ctrl+K | 停止此会话中所有正在运行的[后台子代理](https://code.claude.com/docs/zh-CN/sub-agents.md#run-subagents-in-foreground-or-background)，并在此会话剩余时间内关闭 [Artifact 自动回复](https://code.claude.com/docs/zh-CN/artifacts.md#let-claude-reply-to-comments-on-its-own)。在 3 秒内按两次快捷键以确认。在后台子代理的权限提示打开时也可以按下此快捷键 |
| `chat:cycleMode` | Shift+Tab\* | 循环切换权限模式 |
| `chat:modelPicker` | Meta+P | 打开模型选择器 |
| `chat:fastMode` | Meta+O | 切换快速模式 |
| `chat:thinkingToggle` | Meta+T | 切换扩展思考 |
| `chat:submit` | Enter | 提交消息 |
| `chat:queueSubmit` | Ctrl+X Enter | 提交消息并将其标记为排队等候：当 Claude 正在工作时，Claude Code 会[将其加入队列](https://code.claude.com/docs/zh-CN/interactive-mode.md#queue-messages-while-claude-works)，且绝不会中断当前轮次。与 `chat:submit` 不同，即使自动补全建议处于高亮状态，它也会提交草稿。需要 v2.1.247 或更高版本 |
| `chat:sendNow` | Ctrl+Enter, Ctrl+X Ctrl+S | 立即发送您[排队的消息](https://code.claude.com/docs/zh-CN/interactive-mode.md#queue-messages-while-claude-works)以及您的草稿。[Claude Code 何时发送您排队的内容](https://code.claude.com/docs/zh-CN/interactive-mode.md#when-claude-code-sends-what-you-queued)介绍了 Claude 正在进行的轮次会发生什么。当没有任何内容在运行时，该键会提交草稿；在 [shell 模式](https://code.claude.com/docs/zh-CN/interactive-mode.md#shell-mode-with-prefix)下，它只会将命令加入队列。不报告扩展键的终端会将 `Ctrl+Enter` 作为普通 `Enter` 传递，因此 `Ctrl+X Ctrl+S` 是在任何终端中都有效的绑定。需要 v2.1.275 或更高版本 |
| `chat:newline` | Ctrl+J | 插入换行而不提交 |
| `chat:undo` | Ctrl+\_, Ctrl+Shift+- | 撤销上一个操作 |
| `chat:externalEditor` | Ctrl+G, Ctrl+X Ctrl+E | 在外部编辑器中打开。[Agent 视图的 Dispatch 输入框](https://code.claude.com/docs/zh-CN/agent-view.md#keyboard-shortcuts)同样遵循此操作的单键绑定 |
| `chat:stash` | Ctrl+S | 暂存当前提示词 |
| `chat:imagePaste` | Ctrl+V（在 Windows 和 WSL 上为 Alt+V） | 从剪贴板粘贴图像。在 WSL 上，默认绑定了这两个快捷键 |
| `command:<name>` | （未绑定） | 运行[命令](https://code.claude.com/docs/zh-CN/commands.md) `/<name>`。例如，在 `Chat` 块中设置 `"meta+g": "command:compact"` 会让 `Meta+G` 运行 `/compact` |

\*在没有 VT 模式的 Windows 上（Node \<24.2.0/\<22.17.0、Bun \<1.2.23），默认为 Meta+M。

### 自动补全操作

`Autocomplete` 上下文中可用的操作：

| 操作 | 默认 | 描述 |
| :- | :- | :- |
| `autocomplete:accept` | Tab | 接受建议 |
| `autocomplete:dismiss` | Escape | 关闭菜单 |
| `autocomplete:previous` | Up | 上一个建议 |
| `autocomplete:next` | Down | 下一个建议 |

### 确认操作

`Confirmation` 上下文中可用的操作：

| 操作 | 默认 | 描述 |
| :- | :- | :- |
| `confirm:yes` | Enter | 确认操作 |
| `confirm:no` | Escape | 拒绝操作 |
| `confirm:previous` | Up | 上一个选项 |
| `confirm:next` | Down | 下一个选项 |
| `confirm:nextField` | Tab | 在 `/fast` 对话框中，打开或关闭快速模式 |
| `confirm:previousField` | （未绑定） | Claude Code 不响应此操作，引用它的 `keybindings.json` 仍然有效 |
| `confirm:toggle` | Space | 切换选择 |
| `confirm:cycleMode` | Shift+Tab\* | 在文件权限提示中，关闭已打开的[评论字段](https://code.claude.com/docs/zh-CN/permissions.md#add-a-comment-when-you-answer-a-permission-prompt)；如果没有打开的字段，且提示提供了该选项，则选择在会话剩余时间内允许该操作的选项 |

\*在没有 VT 模式的 Windows 上（Node \<24.2.0/\<22.17.0、Bun \<1.2.23），默认为 Meta+M。

在 v2.1.257 之前，`confirm:toggleExplanation` 操作（默认绑定到 `Ctrl+E`）会在 Bash 和 PowerShell 权限提示中显示由模型生成的命令说明。

即使对话框并非在询问是或否的问题，也会使用 `confirm:yes` 和 `confirm:no` 来接受和取消。如果您在此上下文中绑定单个字母（例如 `y` 或 `n`），该字母也会作用于从未将其显示为按键的对话框。将 `y` 和 `n` 显示为按键的对话框会自行读取这些字母，无需绑定。

在大多数对话框中，按两次 `Ctrl+C` 或 `Ctrl+D` 会关闭对话框，而不是退出 Claude Code。第一次按下后的提示会说明第二次按下是关闭对话框还是退出。这两个键均为[保留键](#reserved-shortcuts)，无法重新绑定。

此示例将 `y` 绑定到 `confirm:yes`，将 `n` 绑定到 `confirm:no`：

```json theme={null}
{
  "bindings": [
    {
      "context": "Confirmation",
      "bindings": {
        "y": "confirm:yes",
        "n": "confirm:no"
      }
    }
  ]
}
```

使用这些绑定时，当[文本字段](#text-fields)获得焦点时，`y` 和 `n` 仍会作为字母输入。

在 v2.1.280 之前，`y` 默认也绑定到 `confirm:yes`，`n` 默认绑定到 `confirm:no`。如果您在 v2.1.280 之前使用 `/keybindings` 创建了 `keybindings.json`，该文件会列出这两个绑定，并且在您删除这两行之前它们会一直生效。

### 权限操作

`Confirmation` 上下文中可用于权限对话框的操作：

| 操作 | 默认 | 描述 |
| :- | :- | :- |
| `permission:toggleDebug` | （未绑定） | Claude Code 不响应此操作，引用它的 `keybindings.json` 仍然有效 |

### 会话记录操作

`Transcript` 上下文中可用的操作：

| 操作 | 默认 | 描述 |
| :- | :- | :- |
| `transcript:toggleShowAll` | Ctrl+E | 切换显示全部内容 |
| `transcript:exit` | q, Ctrl+C, Escape | 退出会话记录视图 |

`transcript:toggleShowAll` 仅适用于经典渲染器；在[全屏渲染](https://code.claude.com/docs/zh-CN/fullscreen.md)中，会话记录查看器不提供显示全部的切换功能。

### 历史搜索操作

`HistorySearch` 上下文中可用的操作：

| 操作 | 默认 | 描述 |
| :- | :- | :- |
| `historySearch:next` | Ctrl+R | 下一个匹配项 |
| `historySearch:accept` | Escape, Tab | 接受选择 |
| `historySearch:cancel` | Ctrl+C | 取消搜索 |
| `historySearch:execute` | Enter | 执行所选命令 |
| `historySearch:cycleScope` | Ctrl+S | 循环切换作用域：会话、项目、全部 |

`historySearch:next`、`historySearch:accept`、`historySearch:cancel` 和 `historySearch:execute` 的默认值适用于经典渲染器中的内联历史搜索，该搜索始终搜索所有项目中的提示词。`historySearch:cycleScope` 仅在[全屏渲染](https://code.claude.com/docs/zh-CN/fullscreen.md)中生效，此时 `Ctrl+R` 会改为打开搜索对话框，`Ctrl+S` 用于循环切换其作用域。该对话框的其他按键是固定的，无法重新绑定：`Enter` 或 `Tab` 会将高亮的匹配项放入提示词输入框，`Esc` 则取消。

### 任务操作

`Task` 上下文中可用的操作：

| 操作 | 默认 | 描述 |
| :- | :- | :- |
| `task:background` | Ctrl+B, Ctrl+X Ctrl+B | 将当前任务转入后台 |

### 主题操作

`ThemePicker` 上下文中可用的操作：

| 操作 | 默认 | 描述 |
| :- | :- | :- |
| `theme:toggleSyntaxHighlighting` | Ctrl+T | 切换语法高亮 |

### 帮助操作

`Help` 上下文中可用的操作：

| 操作 | 默认 | 描述 |
| :- | :- | :- |
| `help:dismiss` | Escape | 关闭帮助菜单 |

### 标签页操作

`Tabs` 上下文中可用的操作：

| 操作 | 默认 | 描述 |
| :- | :- | :- |
| `tabs:next` | Tab, Right | 下一个标签页 |
| `tabs:previous` | Shift+Tab, Left | 上一个标签页 |

在带标签页的对话框中，当标签行获得焦点时，`tabs:next` 和 `tabs:previous` 用于切换标签页。在某些对话框中（例如 `/help` 和 `/sandbox`），切换标签页的按键在标签页内容内部也有效。

`Up` 和 `Down` 在标签行与标签页内容之间移动焦点，内容中的列表仅在获得焦点时才响应按键。

### 附件操作

`Attachments` 上下文中可用的操作：

| 操作 | 默认 | 描述 |
| :- | :- | :- |
| `attachments:next` | Right | 下一个附件 |
| `attachments:previous` | Left | 上一个附件 |
| `attachments:remove` | Backspace, Delete | 移除所选附件 |
| `attachments:exit` | Down, Escape | 退出附件导航 |

### 页脚操作

`Footer` 上下文中可用的操作：

| 操作 | 默认 | 描述 |
| :- | :- | :- |
| `footer:next` | Right | 下一个页脚项 |
| `footer:previous` | Left | 上一个页脚项 |
| `footer:up` | Up, Ctrl+P | 在页脚中向上导航（到达顶部时取消选择） |
| `footer:down` | Down, Ctrl+N | 在页脚中向下导航 |
| `footer:openSelected` | Enter | 打开所选页脚项 |
| `footer:clearSelection` | Escape | 清除页脚选择 |
| `footer:close` | x | 停止所选的 [Agent](https://code.claude.com/docs/zh-CN/sub-agents.md#observe-and-steer-running-forks) 或[工作流](https://code.claude.com/docs/zh-CN/workflows.md#manage-runs)，如果它已不再运行，则移除其所在行 |
| `footer:dismiss` | （未绑定） | 将按键绑定到此操作不会产生任何效果，引用它的 `keybindings.json` 仍然有效。在 v2.1.281 之前，Backspace 和 Delete 绑定到此操作，用于从页脚中移除所选的 Artifact 链接。 |

当选中某个页脚项（例如提示词下方 Agent 面板中的某一行）时，即使您在 `Chat` 上下文中将 `Enter` 重新绑定到 `chat:queueSubmit` 或 `chat:newline`，`Enter` 仍会打开该项。

对于 `Footer` 上下文未绑定的按键，`Chat` 绑定（例如用于 `chat:cycleMode` 的 `Shift+Tab`）在选中某项时仍然有效。

### 消息选择器操作

在[回退菜单](https://code.claude.com/docs/zh-CN/checkpointing.md)的消息列表中，您可以使用 [Select 操作](#select-actions)及其默认按键在消息间移动并选择消息。您针对这些操作设置的 `Select` 绑定在此处同样适用。`MessageSelector` 上下文本身没有任何操作或默认绑定。可使用它仅为此列表更改按键，方法是在 `MessageSelector` 块中绑定 Select 操作，例如 `select:accept`。

此示例将 `o` 绑定为在回退菜单中选择高亮的消息，而不会更改任何其他列表：

```json theme={null}
{
  "bindings": [
    {
      "context": "MessageSelector",
      "bindings": {
        "o": "select:accept"
      }
    }
  ]
}
```

在 v2.1.283 之前，此列表会忽略 `Select` 绑定，并拥有自己的操作：`messageSelector:up`、`messageSelector:down`、`messageSelector:top`、`messageSelector:bottom` 和 `messageSelector:select`。如果您的 `keybindings.json` 绑定了其中某个名称，该绑定会在此列表中继续生效，作为执行相同功能的 Select 操作。`Home` 和 `End` 跳转到列表的两端；在 v2.1.283 之前，默认由 `Shift+K` 和 `Shift+J` 等按键实现此功能。

### Diff 操作

这些操作仅作用于 Claude Code 早期的 diff 查看器，当您在 `/plugin` 中禁用 [`cc-plugin-diff` mod](https://code.claude.com/docs/zh-CN/plugins/mods/overview#mods-built-into-claude-code) 后，`/diff` 会在[全屏渲染](https://code.claude.com/docs/zh-CN/fullscreen.md)之外打开该查看器。启用该 mod 时，`/diff` 会改为打开 [diff 对话框](https://code.claude.com/docs/zh-CN/interactive-mode.md#diff-dialog)。无论哪种情况，引用这些操作的 `keybindings.json` 都能正常加载而不会报错。

`DiffDialog` 上下文中可用的操作：

| 操作 | 默认 | 描述 |
| :- | :- | :- |
| `diff:dismiss` | Escape | 关闭 diff 查看器；在详细视图中则改为返回文件列表，而不关闭查看器 |
| `diff:previousSource` | Left | 上一个 diff 来源 |
| `diff:nextSource` | Right | 下一个 diff 来源 |
| `diff:previousFile` | Up, K | 文件列表中的上一个文件；在详细视图中向上滚动一行 |
| `diff:nextFile` | Down, J | 文件列表中的下一个文件；在详细视图中向下滚动一行 |
| `diff:back` | （未绑定） | 在 diff 查看器中返回。Escape 通过 `diff:dismiss` 执行返回操作。详细视图中先前的默认键 Left 已在 v2.1.203 中移除 |

文件列表也会通过 [Select 操作](#select-actions)的默认按键以及您的 `Select` 绑定来响应这些操作。`select:previous` 和 `select:next` 分别移动到上一个和下一个文件，`Enter` 通过 `select:accept` 打开所选文件的 diff。如需仅为文件列表更改其中某个按键，请在 `DiffDialog` 块中绑定相应的 Select 操作。

在 v2.1.283 之前，文件列表会忽略 `Select` 绑定，`Enter` 通过单独的 `diff:viewDetails` 操作打开所选文件的 diff。如果您的 `keybindings.json` 绑定了 `diff:viewDetails`，该绑定会在文件列表中作为 `select:accept` 继续生效。

diff 详细视图还将分页器风格的按键绑定到标准[滚动操作](#scroll-actions)。这些绑定属于 `DiffDialog` 上下文，仅在详细视图中生效；[滚动操作](#scroll-actions)下列出的 `Scroll` 上下文默认值保持不变。

| 操作 | 默认 | 描述 |
| :- | :- | :- |
| `scroll:pageUp` | PageUp | 向上滚动半个视口 |
| `scroll:pageDown` | PageDown | 向下滚动半个视口 |
| `scroll:fullPageUp` | Shift+Space, B | 向上滚动一整个视口 |
| `scroll:fullPageDown` | Space | 向下滚动一整个视口 |
| `scroll:top` | G, Home | 跳转到顶部 |
| `scroll:bottom` | Shift+G, End | 跳转到底部 |

### Diff 面板操作

用于在全屏渲染中由 `/diff` 打开的 [diff 面板](https://code.claude.com/docs/zh-CN/interactive-mode.md#diff-panel)的操作。`app:cycleDiffBase` 位于 `DiffPanel` 上下文中，该上下文在面板打开时处于活动状态；其他操作属于 `Global`。

内置的 [`cc-plugin-diff` mod](https://code.claude.com/docs/zh-CN/plugins/mods/overview#mods-built-into-claude-code) 负责绘制此面板，并处理 `app:cycleDiffBase`、`app:diffFileListUp` 和 `app:diffFileListDown`。`app:toggleReplTab`、`app:toggleDiffNoiseFilter` 和 `app:toggleDiffPreSession` 仅作用于 Claude Code 早期的面板，当您在 `/plugin` 中禁用 `cc-plugin-diff` 后，`/diff` 会打开该面板。

| 操作 | 默认 | 描述 |
| :- | :- | :- |
| `app:toggleReplTab` | （未绑定） | 打开或关闭 diff 面板 |
| `app:cycleDiffBase` | Ctrl+X B | 循环切换面板的比较基准：本会话、未提交，然后是分支 |
| `app:diffFileListUp` | Ctrl+Up, Meta+Up | 当面板的文件列表溢出时向上滚动 |
| `app:diffFileListDown` | Ctrl+Down, Meta+Down | 当面板的文件列表溢出时向下滚动 |
| `app:toggleDiffNoiseFilter` | （未绑定） | 在面板中显示或隐藏测试文件和生成的文件 |
| `app:toggleDiffPreSession` | （未绑定） | 展开或折叠本会话之前的更改 |

### 模型选择器操作

`ModelPicker` 上下文中可用的操作：

| 操作 | 默认 | 描述 |
| :- | :- | :- |
| `modelPicker:decreaseEffort` | Left | 降低 effort 级别 |
| `modelPicker:increaseEffort` | Right | 提高 effort 级别 |
| `modelPicker:thisSessionOnly` | s | 仅将高亮的模型应用于本会话 |

### Effort 滑块操作

`EffortSlider` 上下文中可用的操作，该上下文对应不带参数运行 `/effort` 时打开的滑块。滑块的 Enter 和 Escape 键无法重新绑定。

| 操作 | 默认 | 描述 |
| :- | :- | :- |
| `effortSlider:decreaseEffort` | Left | 将滑块移动到下一个更低的 effort 级别。需要 v2.1.284 或更高版本 |
| `effortSlider:increaseEffort` | Right | 将滑块移动到下一个更高的 effort 级别。需要 v2.1.284 或更高版本 |
| `effortSlider:toggleUltracode` | Tab | 当滑块[提供该选项](https://code.claude.com/docs/zh-CN/model-config.md#when-ultracode-is-available)时，为本会话打开或关闭 [ultracode](https://code.claude.com/docs/zh-CN/workflows.md#let-claude-decide-with-ultracode)。需要 v2.1.284 或更高版本 |
| `effortSlider:thisSessionOnly` | s | 仅将当前聚焦的 [effort 级别](https://code.claude.com/docs/zh-CN/model-config.md#adjust-effort-level)应用于本会话。需要 v2.1.257 或更高版本 |

### Select 操作

`Select` 上下文中可用的操作：

| 操作 | 默认 | 描述 |
| :- | :- | :- |
| `select:next` | Down, J, Ctrl+N | 下一个选项 |
| `select:previous` | Up, K, Ctrl+P | 上一个选项 |
| `select:pageUp` | PageUp | 向上移动一页选项 |
| `select:pageDown` | PageDown | 向下移动一页选项 |
| `select:first` | Home | 第一个选项 |
| `select:last` | End | 最后一个选项 |
| `select:accept` | Enter | 接受选择 |
| `select:cancel` | Escape | 取消选择 |

在 `/skills`、`/mcp` 和 `/tasks` 等列表面板中，Claude Code 会应用您的 `select:pageUp`、`select:pageDown`、`select:first` 和 `select:last` 绑定。在大多数其他列表中（例如 `/model` 选择器），会应用您的 `select:first` 和 `select:last` 绑定。在这些列表中，无论您如何绑定，PageUp 和 PageDown 都会按页浏览选项。

在 v2.1.280 之前，这些其他列表会忽略 Home、End 以及您的 `select:first` 和 `select:last` 绑定。

在 v2.1.283 之前，无论您如何绑定，`/mcp` 工具列表都使用固定的 PageUp 和 PageDown 键进行分页。

### 插件操作

`Plugin` 上下文中可用的操作：

| 操作 | 默认 | 描述 |
| :- | :- | :- |
| `plugin:toggle` | Space | 切换插件选择 |
| `plugin:install` | I | 安装所选插件 |
| `plugin:favorite` | F | 收藏所选插件，使其在 Installed 标签页中排在靠前位置 |

### 提示词上方区域操作

用于提示词上方区域的操作，该区域是 [mod](https://code.claude.com/docs/zh-CN/plugins/mods/interface.md#pick-where-to-draw) 绘制按钮、输入字段和选择框的共享条带。`abovePrompt:toggle` 和 `abovePrompt:focus` 在 `Chat` 上下文中生效。其他操作在该区域或窗格中当前拥有键盘焦点的元素所属的[上下文](#contexts)中生效。

| 操作 | 默认 | 描述 |
| :- | :- | :- |
| `abovePrompt:toggle` | Ctrl+X Ctrl+A | 将该区域折叠为单行提示，或再次展开 |
| `abovePrompt:focus` | Ctrl+X Tab | 将键盘焦点移入该区域，然后依次移到每个打开的[窗格](#pane-actions)，并从最后一个窗格返回提示词 |
| `abovePrompt:next` | Tab | 聚焦下一个控件 |
| `abovePrompt:previous` | Shift+Tab | 聚焦上一个控件 |
| `abovePrompt:press` | Enter | 按下聚焦的按钮、提交聚焦的输入字段，或在选择框中选取高亮的选项 |
| `abovePrompt:leave` | Escape | 将键盘焦点返回到提示词 |
| `abovePrompt:highlightNext` | Down | 在聚焦的选择框中高亮下一个选项 |
| `abovePrompt:highlightPrevious` | Up | 在聚焦的选择框中高亮上一个选项 |

在 [`AbovePrompt` 上下文](#contexts)中，Right 和 Left 默认也会执行 `abovePrompt:next` 和 `abovePrompt:previous`，Space 也会执行 `abovePrompt:press`。

`AbovePrompt` 上下文还将 Up、Down、PageUp、PageDown、Home 和 End 绑定到[窗格滚动操作](#pane-actions) `pane:scrollUp` 至 `pane:bottom`，因此如需为该区域更改其中某个按键，请在 `AbovePrompt` 块中绑定相应的滚动操作。

### 窗格操作

用于由 [mod](https://code.claude.com/docs/zh-CN/plugins/mods/interface.md#know-which-keys-your-mod-can-receive) 绘制的窗格的操作。滚动、调整大小和关闭操作在 `Pane` [上下文](#contexts)中生效。`pane:close` 也在 `PaneField` 上下文中生效，因此当窗格的某个字段获得焦点时它同样有效。当打开多个窗格时，`pane:next` 和 `pane:previous` 在 `Global` 上下文中生效。

| 操作 | 默认 | 描述 |
| :- | :- | :- |
| `pane:scrollUp` | Up | 当窗格的行数超过可显示范围时向上滚动 |
| `pane:scrollDown` | Down | 当窗格的行数超过可显示范围时向下滚动 |
| `pane:pageUp` | PageUp | 将窗格向上滚动一页 |
| `pane:pageDown` | PageDown | 将窗格向下滚动一页 |
| `pane:top` | Home | 跳转到窗格顶部 |
| `pane:bottom` | End | 跳转到窗格底部 |
| `pane:grow` | Ctrl+X Left, Ctrl+X Up | 为窗格提供更多空间：位于会话记录旁边时增加宽度，位于提示词上方时增加高度 |
| `pane:shrink` | Ctrl+X Right, Ctrl+X Down | 为窗格提供更少空间：位于会话记录旁边时减少宽度，位于提示词上方时减少高度 |
| `pane:close` | Ctrl+X X | 关闭窗格 |
| `pane:next` | （未绑定） | 显示下一个打开的窗格 |
| `pane:previous` | （未绑定） | 显示上一个打开的窗格 |

`Pane` 上下文还将 Tab、Shift+Tab、Enter 和 Escape 绑定到与该区域相同的[提示词上方区域操作](#above-prompt-actions)，窗格的输入字段和选择框使用 `AbovePromptInput` 和 `AbovePromptSelect` 上下文。[键盘焦点和热键](https://code.claude.com/docs/zh-CN/plugins/mods/interface.md#know-which-keys-your-mod-can-receive)列出了每个按键在窗格中的作用。

### 设置操作

`Settings` 上下文中可用的操作。`select:accept` 和 `confirm:no` 操作复用自 [Select](#select-actions) 和 [Confirmation](#confirmation-actions) 上下文，但具有设置特有的行为：每个设置项在您更改后会立即生效，因此 Escape 会关闭面板并保存您的更改，而不是拒绝。

| 操作 | 默认 | 描述 |
| :- | :- | :- |
| `settings:search` | / | 进入搜索模式 |
| `settings:retry` | R | 出错时重试加载用量数据 |
| `settings:fixUsage` | F | 当 `/usage` 页脚提供修复时，将 Claude Code 可应用的 [`/usage` 建议](https://code.claude.com/docs/zh-CN/costs#apply-recommendations-with-the-fix-shortcut)发送到您的会话。需要 v2.1.305 或更高版本 |
| `select:accept` | Enter, Space | 更改所选设置项或打开其子菜单 |
| `confirm:no` | Escape | 关闭面板。更改已保存 |

### Agents 操作

`Agents` 上下文中可用的操作，该上下文适用于通过 `claude agents` 打开的 [Agent 视图](https://code.claude.com/docs/zh-CN/agent-view.md)。需要 v2.1.257 或更高版本。

| 操作 | 默认 | 描述 |
| :- | :- | :- |
| `agents:switchView` | Ctrl+S | 在按状态和按目录之间切换[会话分组](https://code.claude.com/docs/zh-CN/agent-view.md#organize-the-list) |
| `agents:togglePin` | Ctrl+T | [置顶或取消置顶](https://code.claude.com/docs/zh-CN/agent-view.md#organize-the-list)所选会话 |
| `agents:find` | Ctrl+F | 使用 [`n:` 筛选器](https://code.claude.com/docs/zh-CN/agent-view#filter-sessions)按名称查找会话。需要 v2.1.288 或更高版本 |
| `agents:rename` | Ctrl+R | [重命名](https://code.claude.com/docs/zh-CN/agent-view.md#organize-the-list)所选会话。需要 v2.1.288 或更高版本 |
| `agents:previousGroup` | Ctrl+Up, Meta+Up | 跳转到上一个[分组标题](https://code.claude.com/docs/zh-CN/agent-view.md#organize-the-list)。需要 v2.1.288 或更高版本 |
| `agents:nextGroup` | Ctrl+Down, Meta+Down | 跳转到下一个分组标题。需要 v2.1.288 或更高版本 |

当 Agent 视图打开时，对于 `Agents` 上下文已绑定的任何按键，Claude Code 会使用 `Agents` 绑定，并忽略同一按键上的 `Chat` 或 `Global` 绑定。例如，在 Agent 视图中按 Ctrl+S 会切换会话分组，而不是触发默认的 `chat:stash`。

Dispatch 输入框的外部编辑器快捷键不是 `Agents` 操作。Agent 视图遵循 `Chat` 上下文的 `chat:externalEditor` 绑定，默认为 Ctrl+G。

在 Agent 视图中，绑定仅在单次按键时触发，因此绑定到 `chat:externalEditor` 的 Ctrl+X Ctrl+E 组合键不会在该视图中打开编辑器。

### 语音操作

启用[语音听写](https://code.claude.com/docs/zh-CN/voice-dictation.md)时，`Chat` 上下文中可用的操作：

| 操作 | 默认 | 描述 |
| :- | :- | :- |
| `voice:pushToTalk` | Space | 听写提示词。根据 `/voice` 模式按住或轻按 |

### 滚动操作

启用[全屏渲染](https://code.claude.com/docs/zh-CN/fullscreen.md)时，`Scroll` 上下文中可用的操作：

| 操作 | 默认 | 描述 |
| :- | :- | :- |
| `scroll:lineUp` | `wheelup` | 向上滚动一行。鼠标滚轮滚动会触发此操作 |
| `scroll:lineDown` | `wheeldown` | 向下滚动一行。鼠标滚轮滚动会触发此操作 |
| `scroll:pageUp` | PageUp | 向上滚动半个视口高度 |
| `scroll:pageDown` | PageDown | 向下滚动半个视口高度 |
| `scroll:top` | Ctrl+Home | 跳转到对话开头 |
| `scroll:bottom` | Ctrl+End | 跳转到最新消息并重新启用自动跟随 |
| `scroll:halfPageUp` | （未绑定） | 向上滚动半个视口高度。行为与 `scroll:pageUp` 相同，为 vi 风格的重新绑定而提供 |
| `scroll:halfPageDown` | （未绑定） | 向下滚动半个视口高度。行为与 `scroll:pageDown` 相同，为 vi 风格的重新绑定而提供 |
| `scroll:fullPageUp` | （未绑定） | 向上滚动一整个视口高度 |
| `scroll:fullPageDown` | （未绑定） | 向下滚动一整个视口高度 |
| `selection:copy` | Ctrl+Shift+C / Cmd+C | 将所选文本复制到剪贴板 |
| `selection:clear` | （未绑定） | 清除当前的文本选择。需要 v2.1.234 或更高版本 |
| `selection:extendLeft` | Shift+Left | 将当前选择向左扩展一列 |
| `selection:extendRight` | Shift+Right | 将当前选择向右扩展一列 |
| `selection:extendUp` | Shift+Up | 将当前选择向上扩展一行。当选择到达顶部边缘时滚动视口 |
| `selection:extendDown` | Shift+Down | 将当前选择向下扩展一行。当选择到达底部边缘时滚动视口 |
| `selection:extendLineStart` | Shift+Home | 将当前选择扩展到行首 |
| `selection:extendLineEnd` | Shift+End | 将当前选择扩展到行尾 |

## 按键语法

### 修饰符

使用修饰符键和 `+` 分隔符：

* `ctrl` 或 `control` - Control 键
* `shift` - Shift 键
* `alt`、`opt`、`option` 或 `meta` - Windows 和 Linux 上的 Alt 键，macOS 上的 Option 键
* `cmd`、`command`、`super` 或 `win` - macOS 上的 Command 键，Windows 上的 Windows 键，Linux 上的 Super 键

`cmd` 组仅在报告 Super 修饰符的终端中被检测到，例如支持 Kitty 键盘协议或 xterm 的 `modifyOtherKeys` 模式的终端。大多数终端不会发送它，因此对于希望在任何地方都能工作的绑定，请使用 `ctrl` 或 `meta`。

例如：

```text theme={null}
ctrl+k          Ctrl + K
shift+tab       Shift + Tab
meta+p          macOS 上的 Option + P，其他地方的 Alt + P
ctrl+shift+c    多个修饰符
```

### 大写字母

Claude Code 不区分大小写地解析按键名称，因此 `K` 与 `k` 的绑定相同，`ctrl+K` 与 `ctrl+k` 相同。要绑定 Shift 和一个字母，请写 `shift+k`。

### 非美国键盘布局

即使您的活动键盘布局输入其他字符，也要将 Ctrl 快捷键的按键名称写成拉丁字符。

Claude Code 如何将您按下的按键与绑定匹配取决于布局的类型：

* 在非拉丁布局（如西里尔字母）下，当终端使用 Kitty 键盘协议并报告该位置时，Claude Code 通过按键的美国布局位置来匹配 Ctrl 快捷键。在这样的终端中，使用俄语布局时，按下 Ctrl 和物理 W 键会触发 `ctrl+w`。在不报告位置的终端中，Claude Code 匹配终端为按键发送的任何内容：ASCII 控制代码触发拉丁快捷键，作为西里尔字符到达的按键不匹配任何绑定
* 在重新排列拉丁字母的布局（如 AZERTY）下，Claude Code 匹配按键输入的字母，因此按下 Ctrl 和标记为 A 的按键会触发 `ctrl+a`

### 和弦

和弦是由空格分隔的按键序列：

```text theme={null}
ctrl+k ctrl+s   按 Ctrl+K，释放，然后按 Ctrl+S
```

在前一个按键的 3 秒内按下每个按键。如果您等待更长时间，Claude Code 会取消和弦并显示一条简短通知。

### 特殊键

* `escape` 或 `esc` - Escape 键
* `enter` 或 `return` - Enter 键
* `tab` - Tab 键
* `space` - 空格键
* `up`、`down`、`left`、`right` - 箭头键
* `pageup`、`pagedown` - Page Up 和 Page Down 键
* `home`、`end` - Home 和 End 键
* `backspace`、`delete` - 删除键
* `wheelup`、`wheeldown` - 鼠标滚轮滚动事件

## 取消绑定默认快捷键

将操作设置为 `null` 以取消绑定默认快捷键：

```json theme={null}
{
  "bindings": [
    {
      "context": "Chat",
      "bindings": {
        "ctrl+s": null
      }
    }
  ]
}
```

这也适用于和弦绑定。取消绑定共享前缀的每个和弦会释放该前缀以用作单键绑定。任何活跃上下文中的和弦都会保留其前缀，因此您必须在定义该和弦的上下文中取消绑定每个和弦。

Claude Code 在 `ctrl+x` 前缀上按上下文绑定以下默认和弦：

* `Chat`：`ctrl+x ctrl+k`、`ctrl+x ctrl+e`、`ctrl+x enter`、`ctrl+x ctrl+a`、`ctrl+x ctrl+s` 和 `ctrl+x tab`
* `Task`：`ctrl+x ctrl+b`
* `DiffPanel`：`ctrl+x b`
* `Pane`：`ctrl+x left`、`ctrl+x right`、`ctrl+x up`、`ctrl+x down` 和 `ctrl+x x`
* `PaneField`：`ctrl+x x`

`ctrl+x enter` 和弦需要 v2.1.247 或更高版本，`ctrl+x b`、`ctrl+x ctrl+a` 和 `ctrl+x tab` 需要 v2.1.260 或更高版本，以及 `ctrl+x ctrl+s` 需要 v2.1.275 或更高版本。

要将 `ctrl+x` 本身回收为单键绑定，请取消绑定所有这些：

```json theme={null}
{
  "bindings": [
    {
      "context": "Task",
      "bindings": {
        "ctrl+x ctrl+b": null
      }
    },
    {
      "context": "DiffPanel",
      "bindings": {
        "ctrl+x b": null
      }
    },
    {
      "context": "Pane",
      "bindings": {
        "ctrl+x left": null,
        "ctrl+x right": null,
        "ctrl+x up": null,
        "ctrl+x down": null,
        "ctrl+x x": null
      }
    },
    {
      "context": "PaneField",
      "bindings": {
        "ctrl+x x": null
      }
    },
    {
      "context": "Chat",
      "bindings": {
        "ctrl+x ctrl+k": null,
        "ctrl+x ctrl+e": null,
        "ctrl+x enter": null,
        "ctrl+x ctrl+a": null,
        "ctrl+x ctrl+s": null,
        "ctrl+x tab": null,
        "ctrl+x": "chat:newline"
      }
    }
  ]
}
```

如果您取消绑定前缀上的某些但不是全部和弦，按下前缀仍会进入和弦等待模式以处理剩余的绑定。

## 保留的快捷键

这些快捷键无法重新绑定：

| 快捷键 | 原因 |
| :- | :- |
| Ctrl+C | 硬编码的中断/取消 |
| Ctrl+D | 硬编码的退出 |
| Ctrl+M | Claude Code 始终将其接收为 Enter |
| Ctrl+\[ | Claude Code 始终将其接收为 Escape。在使用 Kitty 键盘协议的终端中，这需要 v2.1.242 或更高版本 |
| Ctrl+I | Claude Code 始终将其接收为 Tab |
| Ctrl+H | 发送 ASCII 退格字节。[Claude Code 在 Windows 上如何读取它](https://code.claude.com/docs/zh-CN/terminal-config.md#fix-backspace-deleting-a-whole-word-on-windows)取决于您的终端和 [`CLAUDE_CODE_BS_AS_CTRL_BACKSPACE`](https://code.claude.com/docs/zh-CN/env-vars) 环境变量 |
| Caps Lock | 不传递到终端应用程序 |

## 终端冲突

某些快捷键可能与终端多路复用器冲突：

| 快捷键 | 冲突 |
| :- | :- |
| Ctrl+B | tmux 前缀（按两次发送） |
| Ctrl+A | GNU screen 前缀 |
| Ctrl+Z | Unix 进程暂停（SIGTSTP） |

## 文本字段

如果你绑定一个裸字母、数字或空格，你仍然可以在对话框或面板内的文本字段中输入该字符。其中一个字段是 Claude 提出问题的 `Other` 答案。当该字段获得焦点时，你按下的不带 Ctrl、Alt 或 Cmd 的可打印键会进入该字段，Claude Code 不会根据你的绑定来匹配它。

这些键在字段获得焦点时仍然会运行其绑定：

* 不输入字符的键，例如 Enter、Escape、Tab 和箭头键
* 任何与 Ctrl、Alt 或 Cmd 一起按下的键
* 已在进行中的[和弦](#chords)的第二个按键

在主提示符处，Claude Code 根据活跃的上下文（例如 `Chat`）匹配每个键，仅当没有绑定接受该键时才输入该键。

## Vim 模式交互

当您通过 `/config` 中的编辑器模式设置启用 vim 模式时，快捷键和 vim 模式独立运行：

* **Vim 模式**在文本输入级别处理输入：光标移动、模式和动作
* **快捷键**在组件级别处理操作，例如切换待办事项或提交消息
* 在 vim 模式下，Escape 键会将 INSERT 模式切换为 NORMAL 模式，而不会触发 `chat:cancel`
* 大多数 Ctrl+key 快捷键通过 vim 模式传递到快捷键系统
* Vim 键不能通过快捷键文件重新映射。要映射两键 INSERT 模式序列（如 `jj`）到 Escape，请使用 [`vimInsertModeRemaps`](https://code.claude.com/docs/zh-CN/interactive-mode#remap-insert-mode-key-sequences) 设置
* 在 vim NORMAL 模式中，`?` 显示帮助菜单（vim 行为）
* 在 vim NORMAL 模式中，`/` 打开历史搜索，与标准模式中的 Ctrl+R 相同

在 Claude 提出的问题的 `Other` 行中，vim 模式下按 Escape 不会立即取消该问题。除[屏幕阅读器模式](https://code.claude.com/docs/zh-CN/accessibility.md)外，每次聚焦该行时，它都会以 INSERT 模式开始。在 `Other` 行中按 Escape 时，该行会切换到 NORMAL 模式或丢弃输入到一半的 vim 命令，问题仍保持打开状态。在 NORMAL 模式下再次按 Escape 即可取消该问题。

### 按一次键取消问题

要在 vim 模式下按一次键即可取消问题，请在 `Select` 上下文中将另一个键绑定到 `select:cancel`。在只能选择一个选项的问题中，您可以在 `Other` 行中以这种方式取消。在[屏幕阅读器模式](https://code.claude.com/docs/zh-CN/accessibility.md)下，或在可以选择多个选项的问题中，绑定的键不会取消问题，此时请改用 Escape 取消。

请选择一个需要配合 Ctrl 或 Alt 按下的键。单独的字母或数字会[输入到 `Other` 行](#text-fields)中，而不会触发您的绑定。例如，要绑定 `Ctrl+Q`，请将以下内容添加到 `~/.claude/keybindings.json`：

```json theme={null}
{
  "bindings": [
    {
      "context": "Select",
      "bindings": {
        "ctrl+q": "select:cancel"
      }
    }
  ]
}
```

保存文件后，在只能选择一个选项的问题的 `Other` 行中按 `Ctrl+Q`。无论该行处于 INSERT 还是 NORMAL 模式，问题都会关闭。

## 验证

Claude Code 验证您的快捷键并向调试日志写入以下警告：

* 解析错误（无效的 JSON 或结构）
* 拼写错误的修饰符，例如 `ctl+k`。Claude Code 会删除它无法识别的部分，并将绑定应用于剩余的按键，在此示例中为 `k`。
* 按键字符串中 `+` 旁边有空格，例如 `ctrl+ k`。Claude Code 会将该字符串解读为由 `ctrl+` 后跟 `k` 组成的[组合键序列](#chords)。没有任何按键能单独匹配 `ctrl+`，因此该绑定永远不会触发。请删除该空格。
* 无效的上下文名称
* 无效的操作值，例如不是字符串或 `null` 的操作
* 未知的操作名称，例如注册操作的拼写错误。Claude Code 跳过该绑定并保持该键的任何默认绑定有效。
* 保留快捷键冲突
* 同一上下文中的重复绑定

使用 [`--debug`](https://code.claude.com/docs/zh-CN/cli-reference#cli-flags) 启动 Claude Code 以查看详细信息。
