> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# 全屏渲染

> 启用更流畅、无闪烁的渲染模式，支持鼠标操作，在长对话中保持稳定的内存使用。

> [!NOTE]
>
> 全屏渲染是一个[研究预览](#research-preview)。无论您是[默认在全屏启动还是在经典渲染器中启动](#fullscreen-by-default)取决于您的设置。在当前对话中运行 `/tui fullscreen` 或 `/tui default` 来切换。行为可能会根据反馈而改变。

全屏渲染是 Claude Code CLI 的一种替代渲染路径，它消除了闪烁，在长对话中保持内存使用量平稳，并添加了鼠标支持。它在终端的备用屏幕缓冲区上绘制界面，就像 `vim` 或 `htop` 一样，并且只渲染当前可见的消息。这减少了每次更新时发送到终端的数据量。

在渲染吞吐量是瓶颈的终端模拟器中，如 VS Code 集成终端、tmux 和 iTerm2，差异最为明显。如果您的终端滚动位置在 Claude 工作时跳到顶部，或者工具输出流入时屏幕闪烁，此模式可以解决这些问题。

> [!NOTE]
>
> 术语"全屏"描述的是 Claude Code 如何接管终端的绘制表面，就像 `vim` 一样。它与最大化终端窗口无关，在任何窗口大小下都能工作。

## 启用全屏渲染

在任何 Claude Code 对话中运行 `/tui fullscreen`。CLI 会保存 [`tui` 设置](https://code.claude.com/docs/zh-CN/settings-reference#tui)并以您的对话完整地重新启动到全屏模式，因此您可以在会话中途切换而不会丢失上下文。运行 `/tui default` 来切换回经典渲染器，或运行不带参数的 `/tui` 来打印当前活动的渲染器。

在[屏幕阅读器模式](https://code.claude.com/docs/zh-CN/accessibility.md)中，Claude Code 始终使用经典渲染器，除了附加的[后台会话](https://code.claude.com/docs/zh-CN/agent-view.md)仍然以全屏渲染。如果您在任何其他会话中运行 `/tui fullscreen`，Claude Code 会打印说明而不是切换，并且不会更改保存的 `tui` 设置。

Claude Code 将这些内容保留到重新启动的会话中：

* 对话在屏幕上显示的样子。在 [`/rewind`](https://code.claude.com/docs/zh-CN/checkpointing#rewind-and-summarize) 之后，这意味着：
  * 如果您在会话早期倒带，Claude Code 会从倒带点而不是保存在磁盘上的较长记录中重新启动。例如，如果您倒带过了最后三条消息，重新启动的会话会在没有它们的情况下打开
  * 如果您倒带到第一条消息之前，Claude Code 会以空对话重新启动
* 您的[权限模式](https://code.claude.com/docs/zh-CN/permission-modes.md)和[努力级别](https://code.claude.com/docs/zh-CN/model-config.md#adjust-effort-level)
* 您最后用 [`/model`](https://code.claude.com/docs/zh-CN/model-config#setting-your-model) 选择的模型
* 您用 [`--allowed-tools` 或 `--disallowed-tools`](https://code.claude.com/docs/zh-CN/cli-reference#cli-flags) 传递的规则，以及您的 `--agent`、`--agents`、`--append-system-prompt` 和 `--system-prompt-snapshot` 标志
* 您启动时使用的 `--chrome` 或 `--no-chrome` 标志，以便 [Chrome 集成](https://code.claude.com/docs/zh-CN/chrome.md)在此会话中按您的选择保持开启或关闭

如果会话有一个限制条件无法传递到重新启动的进程，Claude Code 会拒绝重新启动。无法传递的限制条件包括：

* 启动标志，例如 [`--system-prompt`](https://code.claude.com/docs/zh-CN/cli-reference#cli-flags) 替换、[`--tools`](https://code.claude.com/docs/zh-CN/cli-reference#cli-flags) 允许列表或 [`--setting-sources`](https://code.claude.com/docs/zh-CN/cli-reference#cli-flags)
* [hook 或 SDK 权限更新](https://code.claude.com/docs/zh-CN/hooks.md#permission-update-entries)为仅此会话添加的拒绝或询问规则

在这种情况下，Claude Code 会打印 [`Cannot switch renderers in this session`](https://code.claude.com/docs/zh-CN/errors#cannot-switch-renderers-in-this-session)，并说明原因。它不会切换或保存任何内容。

您也可以在启动 Claude Code 之前设置 `CLAUDE_CODE_NO_FLICKER` 环境变量：

```bash theme={null}
CLAUDE_CODE_NO_FLICKER=1 claude
```

有关 [`tui`](https://code.claude.com/docs/zh-CN/settings-reference#tui) 设置和变量在两者都设置时如何组合的信息，请参阅该设置的条目。在[失败的全屏启动](#fullscreen-renderer-didnt-finish-starting)之后，Claude Code 仍然遵守该变量但不遵守该设置。`/tui` 命令从重新启动的进程中清除 `CLAUDE_CODE_NO_FLICKER`，以便它写入的设置生效。

### 默认全屏

附加的[后台会话](https://code.claude.com/docs/zh-CN/agent-view.md)以全屏渲染，[屏幕阅读器模式](https://code.claude.com/docs/zh-CN/accessibility.md)中的其他会话使用经典渲染器。否则，Claude Code 会在与您的设置匹配的此表的第一行中的渲染器中启动您：

| 您的情况 | 您启动的渲染器 |
| :- | :- |
| 您设置了 [`CLAUDE_CODE_DISABLE_ALTERNATE_SCREEN=1`](https://code.claude.com/docs/zh-CN/env-vars) 或 `CLAUDE_CODE_NO_FLICKER=0` | 经典 |
| 您设置了 `CLAUDE_CODE_NO_FLICKER=1` | 全屏 |
| Claude Code [在此机器上失败的全屏启动后关闭了全屏](#fullscreen-renderer-didnt-finish-starting) | 经典 |
| 您在 iTerm2 的 [`tmux -CC` 集成模式](#use-with-tmux)中，或您通过 SSH 连接到在 Windows 上运行的 Claude Code | 经典 |
| 您保存了 [`tui` 设置](https://code.claude.com/docs/zh-CN/settings-reference#tui) | 该设置命名的渲染器 |
| 您的会话不[从 Anthropic 获取功能标志](https://code.claude.com/docs/zh-CN/env-vars.md#features-that-need-feature-flag-fetching)，并且 Claude Code 已停止在此机器上提供启动对话框 | 经典 |
| 您的会话不从 Anthropic 获取功能标志，并且此机器的第一次 Claude Code 启动运行了 v2.1.239 或更高版本 | 全屏 |
| 您的会话从 Anthropic 获取功能标志，并且您在 2026 年 5 月 6 日或之后首次使用 Claude Code | 全屏 |
| 其他任何情况 | 经典 |

不从 Anthropic 获取功能标志的会话包括通过 [Amazon Bedrock](https://code.claude.com/docs/zh-CN/amazon-bedrock.md)、[Google Cloud 的 Agent Platform](https://code.claude.com/docs/zh-CN/google-vertex-ai.md) 或 [Microsoft Foundry](https://code.claude.com/docs/zh-CN/microsoft-foundry.md) 的会话，以及关闭了遥测的会话。

如果您在经典渲染器中启动并且尚未保存 `tui` 设置，Claude Code 可能会在启动时打开一个对话框，提供切换选项：

* 如果您接受，Claude Code 会以与 `/tui fullscreen` 相同的方式重新启动，保留相同的会话状态，并在重新启动的会话[成功启动](#fullscreen-renderer-didnt-finish-starting)后保存该设置。
* 如果您选择**稍后**，Claude Code 不会在此机器上再次提供。
* Claude Code 在显示对话框三次启动后停止提供，无论是否回答。

## 变化内容

全屏渲染改变了 CLI 绘制到终端的方式。输入框保持固定在屏幕底部，而不是在输出流入时移动。如果输入框在 Claude 工作时保持不动，则全屏渲染处于活动状态。只有可见的消息保留在渲染树中，因此无论对话长度如何，内存都保持恒定。

由于对话存在于备用屏幕缓冲区而不是终端的滚动历史中，一些事情的工作方式不同：

| 之前 | 现在 | 详情 |
| :- | :- | :- |
| `Cmd+f` 或 tmux 搜索来查找文本 | `Ctrl+o` 进入记录模式，然后 `/` 来搜索或 `[` 来写入滚动历史 | [搜索和查看对话](#search-and-review-the-conversation) |
| 终端的原生点击拖动来选择和复制 | 应用内选择，鼠标释放时自动复制 | [使用鼠标](#use-the-mouse) |
| `Cmd` 点击来打开 URL | macOS 上的 `Cmd` 点击，其他地方的 `Ctrl` 点击 | [使用鼠标](#use-the-mouse) |

如果鼠标捕获干扰您的工作流程，您可以[关闭它](#keep-native-text-selection)，同时保持无闪烁渲染。

## 使用鼠标

全屏渲染会捕获鼠标事件并在 Claude Code 内处理它们：

* **在提示输入中单击**以在您正在输入的文本中的任何位置放置光标。
* **在 `/` 命令或 `@` 文件列表中单击建议**以接受它。悬停会突出显示光标下的行。
* **在选择菜单中单击选项**以选择它。这包括权限提示、`/model`、`/config` 和其他显示选项列表的对话框。
* **在多选菜单中单击选项**以切换它，然后单击提交按钮以确认您的选择。单击自由文本行（例如多选题中的 `Other` 行）会聚焦其输入字段，以便您可以输入答案。
* **单击 `/config` 面板中的设置值**以更改它，并使用鼠标滚轮滚动设置列表。需要 Claude Code v2.1.271 或更高版本。
* **用鼠标滚轮滚动选择或多选菜单**当它显示的选项多于一次显示的选项时，例如短终端窗口中的 `/model` 列表。当指针悬停在其选项上时，滚轮会滚动列表。需要 Claude Code v2.1.280 或更高版本。
* **用其滚动条滚动溢出的列表。** 在列表面板（例如 `/skills`、`/mcp` 和 `/plugin` 的已安装列表）中，当指针悬停在列表上时，滚动条会出现在有超过适应行数的列表旁边。单击轨道以跳转到该点，或拖动滑块。需要 Claude Code v2.1.281 或更高版本。
  * 当滚动条两端带有 `↑` 和 `↓` 箭头时，单击箭头可滚动一行，按住箭头可持续滚动。箭头需要 Claude Code v2.1.286 或更高版本。
* **单击列表边缘的 `↑ N more` 或 `↓ N more` 行**以跳转到列表的该端，而不选择任何选项。需要 Claude Code v2.1.286 或更高版本。
* **单击折叠的工具结果**以展开它并查看完整输出。再次单击以折叠。工具调用及其结果一起展开。只有有更多内容要显示的消息才可点击。
  * 单击也会展开 `!` shell 命令的输出，无论是较旧的截断结果还是命令运行时的实时进度行。需要 Claude Code v2.1.257 或更高版本。
  * 当发送者是[队友](https://code.claude.com/docs/zh-CN/agent-teams.md)或在您的会话中运行的另一个 Agent 时，单击也会展开暗淡的 `Message from @<sender>` 行。
* **在 macOS 上按住 `Cmd`，或在 Linux 和 Windows 上按住 `Ctrl`，然后单击 URL 或文件路径**以打开它。纯 `http://` 和 `https://` URL 在您的浏览器中打开。当由 Claude Code 处理单击时，工具输出中的文件路径（如 Edit 或 Write 后打印的路径）会在您的文件管理器中打开并选中该文件。不带修饰符的纯单击不会打开链接，与本机终端行为相匹配。
  * 在 Linux 和 WSL 上，单击文件路径需要一个提供 `org.freedesktop.FileManager1` D-Bus 接口的文件管理器。如果没有，则不会打开任何内容。
  * Claude Code 将网络 (UNC) 路径（例如 `\\server\share\file.ts`）呈现为纯文本，没有链接，因为打开网络路径可能会将您的 Windows 凭据发送到它命名的主机。
  * 某些 macOS 终端会将 `Cmd`+单击转发给正在运行的应用程序，而不是自己打开链接，终端鼠标协议无法编码 `Cmd` 键，因此 Claude Code 收到纯单击。在 Ghostty 中，以及在 macOS 上的 Warp 中，Claude Code 检测到这一点，并让纯单击链接打开它，按住 `Cmd` 仍然有效。
  * 在 VS Code 集成终端和类似的基于 xterm.js 的终端中，Claude Code 遵循终端自己的链接处理程序，该处理程序使用相同的手势。
* **单击并拖动**以在对话中的任何位置选择文本。双击选择一个单词，与 iTerm2 的单词边界相匹配，因此文件路径作为一个单元选择。双击 URL 会选择整个 URL，包括方案。三击选择该行。
* **用鼠标滚轮滚动**以在对话中移动。

选定的文本在鼠标释放时自动复制到您的剪贴板。要关闭此功能，请在 `/config` 中切换"选择时复制"。

关闭"选择时复制"后，按 `Ctrl+Shift+c` 手动复制。在支持 kitty 键盘协议的终端上，例如 kitty、WezTerm、Ghostty 和 iTerm2，`Cmd+c` 也有效。如果您有活动选择，`Ctrl+c` 会复制而不是取消。

选择活动时，按住 `Shift` 并按箭头键从键盘扩展它。`Shift+↑` 和 `Shift+↓` 在选择到达顶部或底部边缘时滚动视口。`Shift+Home` 和 `Shift+End` 扩展到当前行的开始或结束。

在正常提示视图中，活动选择发生的情况取决于您按下的键：

* **`Esc`**：Claude Code 执行该键的常规操作，例如中断正在运行的响应或关闭打开的对话框，选择保持突出显示。
* **`PgUp`、`PgDn`、`Ctrl+Home`、`Ctrl+End` 或 `Shift`、`Alt` 或 `Option` 或 `Cmd`、`Win` 或 `Super` 与箭头、`Home` 或 `End` 键**：选择保持。
* **任何其他键，包括纯箭头键、`Enter` 和输入的字符**：Claude Code 清除选择。
* **绑定到 [`selection:clear`](https://code.claude.com/docs/zh-CN/keybindings#scroll-actions) 的键**：Claude Code 清除选择，即使该键是 `Esc` 或其他通常保持选择的键。该操作没有默认绑定。

在[会话记录模式](#search-and-review-the-conversation)中，那里列出的导航键也会保持选择。

## 滚动对话

全屏渲染处理应用内的滚动。使用这些快捷键进行导航：

| 快捷键 | 操作 |
| :- | :- |
| `PgUp` / `PgDn` | 向上或向下滚动半屏 |
| `Ctrl+Home` | 跳转到对话开始 |
| `Ctrl+End` | 跳转到最新消息并重新启用自动跟随 |
| 鼠标滚轮 | 一次滚动几行 |

如果您关闭一个已[压缩](https://code.claude.com/docs/zh-CN/context-window.md#what-survives-compaction)的会话并[恢复](https://code.claude.com/docs/zh-CN/sessions.md#what-a-resumed-session-restores)它，向上滚动会停在 `Conversation compacted` 行，而您较早的消息仍保存在磁盘上的[会话记录文件](https://code.claude.com/docs/zh-CN/sessions.md#where-transcripts-are-stored)中。要获取这些消息中的某个细节，请让 Claude 在该文件中查找。

在没有专用 `PgUp`、`PgDn`、`Home` 或 `End` 键的键盘上，例如 MacBook 键盘，按住 `Fn` 并使用箭头键：`Fn+↑` 发送 `PgUp`，`Fn+↓` 发送 `PgDn`，`Fn+←` 发送 `Home`，`Fn+→` 发送 `End`。`Ctrl+Fn+→` 在 macOS 上无法到达 Claude Code，因此 MacBook 键盘默认没有有效的跳转到底部快捷键。相反，使用以下选项之一：

* 点击[跳转到底部按钮](#auto-follow)。
* 使用鼠标滚轮滚动到底部以恢复跟随。
* 将 `scroll:bottom` 重新绑定到您的键盘可以发送的快捷键。

这些操作可重新绑定。有关完整的操作名称列表（包括没有默认绑定的半页和全页变体），请参阅[滚动操作](https://code.claude.com/docs/zh-CN/keybindings.md#scroll-actions)。

当您向上滚动时，对话顶部的一个暗淡标题行显示已滚动到视图上方的最新提示。点击该行可跳转到该提示。

### 自动跟随

向上滚动会暂停自动跟随，以便新输出不会将您拉回到底部。当您向上滚动时，一个 `Jump to bottom` 按钮会浮动在会话记录的底部边缘，当新输出到达时会显示计数，例如 `3 new messages`。点击它、按 `Ctrl+End` 或滚动到底部以恢复跟随。

当自动跟随暂停时，当响应完成流式传输时，视图也会保持在您滚动的位置。

按钮的键盘提示反映您的键盘可以发送的内容。在 macOS 上，它建议点击或 `Fn+↓` 滚动，因为 `Ctrl+End` 无法从 Mac 键盘到达 Claude Code。重新绑定 [`scroll:bottom`](https://code.claude.com/docs/zh-CN/keybindings#scroll-actions)，按钮会在每个平台上显示您的快捷键。

在终端太窄而无法显示完整标签的情况下，按钮会缩短提示，而不是换行到记录行下方。

要完全关闭自动跟随以使视图保持在您离开的位置，打开 `/config` 并将自动滚动设置为关闭。禁用自动滚动后，视图永远不会自动跳转到底部。权限提示和其他需要响应的对话框仍会根据此设置滚动到视图中。

### 鼠标滚轮滚动

鼠标滚轮滚动需要您的终端将鼠标事件转发到 Claude Code。大多数终端在应用程序请求时都会执行此操作。iTerm2 将其设置为每个配置文件的设置：如果滚轮不起作用但 `PgUp` 和 `PgDn` 有效，打开设置 → 配置文件 → 终端并启用启用鼠标报告。点击展开和文本选择也需要相同的设置。

如果鼠标滚轮滚动感觉缓慢，您的终端可能每个物理刻度发送一个滚动事件，没有乘数。某些终端（如 Ghostty 和启用更快滚动的 iTerm2）已经放大了滚轮事件。其他终端（包括 VS Code 集成终端）每个刻度发送一个事件。Claude Code 无法检测哪个。

设置 `CLAUDE_CODE_SCROLL_SPEED` 以乘以基本滚动距离：

```bash theme={null}
export CLAUDE_CODE_SCROLL_SPEED=3
```

值 `3` 与 `vim` 和类似应用程序中的默认值匹配。该设置接受任何正值，最高为 20，包括低于 1 的分数值，例如 `0.25` 以减慢已经放大滚轮事件的终端中的加速触控板和滚轮滚动。

要交互式调整滚动速度，运行 `/scroll-speed`。在对话框打开时滚动，即可立即感受变化。按 `←` 和 `→` 调整速度，按 `r` 重置为自动检测的默认值，按 `Enter` 保存。对话框以整数步长增加到 10，在支持更精细控制的终端上，它还提供四分之一步长，最低为 0.25。

该命令写入与 `CLAUDE_CODE_SCROLL_SPEED` 环境变量设置相同的值，持久化到 `~/.claude/settings.json`。对话框的最大值是 10：如果您通过环境变量设置更高的值，对话框显示 10，从对话框保存会持久化 10。该命令在 JetBrains IDE 终端中不可用。

与基本速度分开，Claude Code 在您快速旋转滚轮时加速滚动速率，因此快速旋转覆盖的距离比相同数量的慢刻度更多。要关闭加速并保持每个刻度的恒定速率，在 [`settings.json`](https://code.claude.com/docs/zh-CN/settings-reference#all-settings) 中将 `wheelScrollAccelerationEnabled` 设置为 `false`。此设置需要 Claude Code v2.1.174 或更高版本。

### 在 JetBrains IDE 终端中滚动

在 JetBrains IDE 终端中，Claude Code 应用其自己的滚动处理并忽略 `CLAUDE_CODE_SCROLL_SPEED`。终端以比其他模拟器高得多的速率发送滚动事件，因此在其他地方调整的乘数会在这里超调。

在 2025.2 中，终端还有滚动滚轮错误，会产生虚假的箭头键和错误方向的事件。Claude Code 在运行时检测这些并自动缓解它们，因此触控板和鼠标滚轮滚动无需配置即可工作。为了获得最佳滚动体验，升级到 2025.3 或更高版本。如果 Claude Code 检测到该错误，它会在您第一次滚动时显示提示。

## 搜索和查看对话

`Ctrl+o` 在普通提示和记录模式之间切换。

如需获得更清晰的视图，仅显示您的最后一个提示、工具调用的单行摘要和编辑差异统计，以及最终响应，请运行 `/focus`。该设置在会话之间保持。再次运行 `/focus` 可将其关闭。

记录模式获得 `less` 风格的导航和搜索：

| 快捷键 | 操作 |
| :- | :- |
| `/` | 打开搜索。输入以查找匹配项，按 `Enter` 接受，按 `Esc` 取消并恢复您的滚动位置 |
| `n` / `N` | 跳转到下一个或上一个匹配项。在您关闭搜索栏后有效 |
| `j` / `k` 或 `↑` / `↓` | 向下滚动一行 |
| `g` / `G` 或 `Home` / `End` | 跳转到顶部或底部 |
| `{` / `}` | 跳转到上一个或下一个提示 |
| `Ctrl+u` / `Ctrl+d` | 向下滚动半页 |
| `Ctrl+b` / `Ctrl+f` 或 `Space` / `b` | 向下滚动整页 |
| `Ctrl+o`、`Esc` 或 `q` | 退出记录模式并返回提示 |

您的终端的 `Cmd+f` 和 tmux 搜索看不到对话，因为它位于备用屏幕缓冲区中，而不是本机滚动返回。要将内容返回到您的终端，请按 `Ctrl+o` 首先进入记录模式，然后：

* **`[`**：将完整对话写入您的终端的本机滚动返回缓冲区，所有工具输出都已展开。对话现在是您的终端中的普通文本，因此 `Cmd+f`、tmux 复制模式和任何其他本机工具都可以搜索或选择它。长会话在发生这种情况时可能会暂停片刻。这会持续到您使用 `Esc` 或 `q` 退出记录模式为止，这会使您返回全屏渲染。下一个 `Ctrl+o` 重新开始。
* **`v`**：将对话写入临时文件并在 `$VISUAL` 或 `$EDITOR` 中打开它。

## 在 diff 面板中查看你的更改

在全屏渲染中，[`/diff`](https://code.claude.com/docs/zh-CN/interactive-mode#review-changes-with-%2Fdiff) 会在对话旁边打开一个面板，因此您可以在 Claude 工作时观看更改逐步累积。[Diff 面板](https://code.claude.com/docs/zh-CN/interactive-mode.md#diff-panel)涵盖了它显示的内容、它何时会自动打开、如何保持它关闭以及如何更改它比较的内容。

## 清除对话

运行 `/clear` 来开始新的对话。

如果显示看起来混乱或部分空白，按 `Ctrl+L` 来重绘屏幕。重绘会保持对话和你的输入原位。

`Cmd+K` 在你的终端将其传递给 Claude Code 时的作用与 `Ctrl+L` 相同。iTerm2 和 Terminal.app 自己处理 `Cmd+K` 并清除自己的屏幕，Claude Code 检测到清除的屏幕并重新绘制对话。在 v2.1.280 之前，从 v2.1.260 开始，按 `Ctrl+L` 或 `Cmd+K`（到达 Claude Code 的地方）会在全屏渲染中清除屏幕。在 v2.1.238 之前，在两秒内按两次 `Ctrl+L` 会运行 `/clear`。

## 与 tmux 一起使用

全屏渲染在 tmux 内部可以工作，但有三个注意事项。

鼠标滚轮滚动需要 tmux 的鼠标模式。如果你的 `~/.tmux.conf` 还没有启用它，请添加这一行并重新加载你的配置：

```bash theme={null}
set -g mouse on
```

没有鼠标模式，滚轮事件会发送到 tmux 而不是 Claude Code。使用 `PgUp` 和 `PgDn` 的键盘滚动在任何情况下都可以工作。如果 Claude Code 检测到 tmux 但鼠标模式关闭，它会在启动时打印一次性提示。

全屏渲染与 iTerm2 的 tmux 集成模式不兼容，这是你使用 `tmux -CC` 进入的模式。在集成模式中，iTerm2 将每个 tmux 窗格渲染为本地分割，而不是让 tmux 绘制到终端。备用屏幕缓冲区和鼠标跟踪在那里无法正确工作：鼠标滚轮不起作用，双击可能会破坏终端状态。不要在 `tmux -CC` 会话中启用全屏渲染。在 iTerm2 内部的常规 tmux（不带 `-CC`）可以正常工作。

tmux 3.6 系列及之前的版本不实现同步输出，所以在这些版本下，你在重绘时可能会看到比直接在终端中运行 Claude Code 时更多的闪烁。Claude Code 在启动时探测终端以获取同步输出支持，并在终端报告支持时使用它。如果你在 tmux 下看到闪烁，请升级到最新的 tmux 或在 tmux 外的自己的终端标签页中运行 Claude Code。

## 保持原生文本选择

鼠标捕获是最常见的摩擦点，特别是在 SSH 或 tmux 内部。当 Claude Code 捕获鼠标事件时，您终端的原生选择复制功能会停止工作。您通过点击和拖动进行的选择存在于 Claude Code 内部，而不是在您终端的选择缓冲区中，因此 tmux 复制模式、Kitty hints 和类似工具看不到它。

Claude Code 将选择内容写入您的系统剪贴板，它使用的路径取决于您的设置。在本地会话中，它运行原生剪贴板工具：

* **macOS**: `pbcopy`
* **Linux**: Wayland 上使用 `wl-copy`，或在 X11 上使用 `xclip` 或 `xsel`（取决于安装的是哪个）。Claude Code 同时写入剪贴板和 PRIMARY 选择，因此中键粘贴可以工作。
* **Windows 和 WSL**: PowerShell `Set-Clipboard`

在 tmux 内部，它也会写入 tmux 粘贴缓冲区。通过 SSH 时，它会回退到 OSC 52 转义序列。在 GNU screen 内部，Claude Code 也会将长选择复制到剪贴板。在 v2.1.219 之前，如果您复制的选择长度超过大约 570 个字符，GNU screen 会将 base64 文本打印到窗口中。Claude Code 在每次复制后都会打印一个提示，告诉您它使用了哪条路径。

某些终端默认阻止 OSC 52。iTerm2 会阻止它，直到您打开 Settings → General → Selection → Applications in terminal may access clipboard；在 iTerm2 中运行 [`/terminal-setup`](https://code.claude.com/docs/zh-CN/terminal-config) 会为您启用此功能。

对于一次性原生选择，要使用的键取决于您的终端：

* **Terminal.app**: `Fn`
* **iTerm2**: `Option`
* **VS Code、Cursor 和 Devin Desktop**: `Shift`，或在启用 `terminal.integrated.macOptionClickForcesSelection` 设置的 macOS 上使用 `Option`
* **大多数其他终端**: `Shift`

按住该键同时点击和拖动。您的终端自己处理选择，而不是将其传递给 Claude Code，因此复制快捷键如 `Cmd+C` 可以作用于您选择的内容。Claude Code 也会在其屏幕提示中显示正确的键。

通过 SSH 或在 tmux 内部，Claude Code 无法总是检测到您连接的终端，因此提示会列出候选键。

如果您一直依赖原生选择，设置 `CLAUDE_CODE_DISABLE_MOUSE=1` 以选择退出鼠标捕获，同时保持无闪烁渲染和平坦内存：

```bash theme={null}
CLAUDE_CODE_NO_FLICKER=1 CLAUDE_CODE_DISABLE_MOUSE=1 claude
```

禁用鼠标捕获后，使用 `PgUp`、`PgDn`、`Ctrl+Home` 和 `Ctrl+End` 的键盘滚动仍然有效，您的终端原生处理选择。您会失去点击定位光标、点击展开工具输出、URL 点击和 Claude Code 内部的滚轮滚动。

要保持滚轮滚动但关闭点击、拖动和悬停处理，请改为设置 `CLAUDE_CODE_DISABLE_MOUSE_CLICKS=1`。当两个变量都设置时，`CLAUDE_CODE_DISABLE_MOUSE` 优先。

禁用点击后，Claude Code 仍然捕获鼠标，因此滚轮和触控板滚动对话，但左键点击在 Claude Code 内部不起作用。您仍然需要按住终端的键进行原生点击和拖动选择。右键点击和中键粘贴在支持它们的终端上继续工作。

## 故障排除

### 屏幕上出现过时或错位的文本

全屏渲染仅发送帧之间更改的单元格。某些终端（最常见的是 Windows Terminal 和其他基于 ConPTY 的主机）会错误地合并这些定位写入，并在调整窗口大小之前在屏幕上留下早期输出的片段。

设置 [`CLAUDE_CODE_ALT_SCREEN_FULL_REPAINT=1`](https://code.claude.com/docs/zh-CN/env-vars) 以在每一帧上重新绘制每个单元格，而不是发送增量更新。

在 Windows PowerShell 上：

```powershell theme={null}
$env:CLAUDE_CODE_ALT_SCREEN_FULL_REPAINT = "1"
claude
```

在 macOS 或 Linux 上：

```bash theme={null}
CLAUDE_CODE_ALT_SCREEN_FULL_REPAINT=1 claude
```

在 Windows 上，Claude Code 已经为后台会话和 [agent view](https://code.claude.com/docs/zh-CN/agent-view.md) 自动启用了完整重绘，因此您只需要为直接启动的交互式全屏会话设置该变量。

### 启动时出现 `Claude Code's fullscreen renderer didn't finish starting last time`

如果此计算机上的全屏会话在成功启动之前崩溃，Claude Code 会在经典渲染器中启动您的下一个会话，并打印以下两行之一。会话在绘制其第一帧后，要么保持运行 10 秒钟，要么您使用 `/exit`、Ctrl+C 或 Ctrl+D 结束它，这样会话就已成功启动。您看到的行告诉您 Claude Code 在此会话后的操作：

* 在一次失败的启动后，您会看到 `Claude Code's fullscreen renderer didn't finish starting last time on this machine`。Claude Code 会在您启动的下一个会话中再次尝试全屏渲染
* 在两次失败的启动后，您会看到 `Claude Code's fullscreen renderer has repeatedly failed to start on this machine`。Claude Code 会继续使用经典渲染器，直到您更新 Claude Code 或运行 `/tui fullscreen`，并在之后的那些会话中不打印任何内容

要确认失败的启动是您处于经典渲染器的原因，请运行不带参数的 `/tui`。当失败的启动是原因时，`Current renderer` 行会说明这一点。

要保持经典渲染器，请运行 `/tui default`，这会保存 `tui` 设置而不重新启动。要再次尝试全屏渲染，请运行 `/tui fullscreen`。如果该会话也没有完成启动，请 [报告问题](#research-preview)。

在 v2.1.236 之前，Claude Code 在失败的启动后继续在全屏渲染中启动会话。

#### Claude Code 如何计算失败的启动

* 计数的会话：仅限因您的 `tui` 设置而在全屏渲染中启动的会话、您接受了 [启动对话框](#fullscreen-by-default)，或 Claude Code 默认以全屏启动您的会话
* `CLAUDE_CODE_NO_FLICKER=1`：如果您设置了它，Claude Code 会在失败的启动后以全屏方式渲染该会话，并且不计数
* 计数重置：Claude Code 按 Claude Code 版本计算失败的启动，成功的全屏启动会重置计数
* 启动对话框：如果您接受了对话框，重新启动的会话崩溃了，Claude Code 不会打印任何行，也不会在此 Claude Code 版本上再次显示对话框

## 研究预览

全屏渲染是一项研究预览功能。它已在常见的终端模拟器上进行了测试，但您可能会在不太常见的终端或不寻常的配置上遇到渲染问题。

如果您遇到问题，请在 Claude Code 中运行 `/feedback` 来报告，或在 [claude-code GitHub 仓库](https://github.com/anthropics/claude-code/issues)上提交问题。请包含您的终端模拟器名称和版本。

要关闭全屏渲染，请运行 `/tui default`，或如果您以这种方式启用了 `CLAUDE_CODE_NO_FLICKER`，请取消设置它。当您使用 `/tui default` 切换回去时，Claude Code 可能会首先显示一个可选的反馈提示，询问您为什么要切换。输入原因并按 `Enter` 发送，或按 `Esc` 跳过。无论哪种方式，CLI 都会重新启动到经典渲染器。要强制使用经典渲染器而不管保存的 `tui` 设置，请设置 `CLAUDE_CODE_DISABLE_ALTERNATE_SCREEN=1`。经典渲染器将对话保留在终端的原生滚动缓冲区中，因此 `Cmd+f` 和 tmux 复制模式可以照常工作。

从[代理视图](https://code.claude.com/docs/zh-CN/agent-view.md)或 `claude attach` 打开的后台会话始终使用全屏渲染。附加终端进入备用屏幕缓冲区以显示会话，经典渲染器在那里没有滚动缓冲或鼠标处理，因此 `tui` 设置和 `CLAUDE_CODE_DISABLE_ALTERNATE_SCREEN` 不适用于它们。
