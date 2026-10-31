> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# 故障排除

> 修复 Claude Code 中的高 CPU 或内存使用、挂起、自动压缩抖动和搜索问题，并找到其他问题的正确页面。

本页涵盖 Claude Code 运行后的性能、稳定性和搜索问题。对于其他问题，请从与您遇到的问题相匹配的页面开始：

| 症状 | 转到 |
| :- | :- |
| `command not found`、安装失败、PATH 问题、`EACCES`、TLS 错误 | [故障排除安装和登录](https://code.claude.com/docs/zh-CN/troubleshoot-install.md) |
| 更新或安装下载失败，显示 `The connection dropped while downloading the update` 或 `aborted` | [故障排除安装和登录](https://code.claude.com/docs/zh-CN/troubleshoot-install.md#the-connection-dropped-while-downloading-the-update) |
| 更新或安装失败，显示 `Release verification failed for Claude Code` | [故障排除安装和登录](https://code.claude.com/docs/zh-CN/troubleshoot-install.md#release-verification-failed) |
| 会话页脚以红色显示 `Auto-update failed · Run claude doctor` | [故障排除安装和登录](https://code.claude.com/docs/zh-CN/troubleshoot-install.md#auto-update-failed) |
| 登录循环、OAuth 错误、`403 Forbidden`、"organization disabled"、Amazon Bedrock、Google Cloud 的 Agent Platform 或 Microsoft Foundry 凭据 | [故障排除安装和登录](https://code.claude.com/docs/zh-CN/troubleshoot-install.md#login-and-authentication) |
| 设置未应用、hooks 未触发、MCP 服务器未加载 | [调试您的配置](https://code.claude.com/docs/zh-CN/debug-your-config.md) |
| 会话以自动模式启动，或 Claude 编辑文件并运行命令而不询问 | [会话启动的模式](https://code.claude.com/docs/zh-CN/permission-modes.md#which-mode-a-session-starts-in) |
| 更新 Claude Code 或 Agent SDK 后，在未传递 `--permission-mode`、`permissionMode` 或 `permission_mode` 的 `claude -p` 或 SDK 运行中，工具调用被拒绝 | [`claude -p` 和 Agent SDK 运行中的工具调用被拒绝](https://code.claude.com/docs/zh-CN/permission-modes#which-mode-a-non-interactive-run-starts-in) |
| `API Error: 5xx`、`529 Overloaded`、`429`、请求验证错误 | [错误参考](https://code.claude.com/docs/zh-CN/errors.md) |
| `model not found` 或 `you may not have access to it` | [错误参考](https://code.claude.com/docs/zh-CN/errors.md#theres-an-issue-with-the-selected-model) |
| Claude 运行的命令失败，显示 `Disk full:`、`Disk quota used up:`、`Nearly out of inodes:`、`The command was not started:`、`Your disk quota is full`、`is full (ENOSPC)` 或 `Command output was lost` | [错误参考](https://code.claude.com/docs/zh-CN/errors.md#disk-quota-or-temp-filesystem-is-full) |
| VS Code 扩展未连接或未检测到 Claude | [VS Code 集成](https://code.claude.com/docs/zh-CN/vs-code.md#fix-common-issues) |
| VS Code 或 SDK 应用中出现 `Claude Code process exited with code 1` | [错误参考](https://code.claude.com/docs/zh-CN/errors.md#claude-code-process-exited-with-code-n) |
| JetBrains 插件或 IDE 未检测到 | [JetBrains 集成](https://code.claude.com/docs/zh-CN/jetbrains.md#troubleshooting) |
| 高 CPU 或内存、响应缓慢、挂起、搜索找不到文件 | [性能和稳定性](#performance-and-stability)下方 |

如果您不确定哪个适用，请在 Claude Code 内运行 `/doctor` 以自动检查您的安装、设置、扩展和上下文使用情况；它会提议可以在您确认后应用的修复。如果 `claude` 根本无法启动，请从您的 shell 运行 `claude doctor`。运行 `/mcp` 以检查 MCP 服务器状态。

## 性能和稳定性

这些部分涵盖与资源使用、响应性和搜索行为相关的问题。

### 高 CPU 或内存使用

Claude Code 设计用于大多数开发环境，但在处理大型代码库时可能消耗大量资源。如果您遇到性能问题：

1. 定期使用 `/compact` 以减少上下文大小。如果它返回 `Not enough messages to compact.`，则对话的轮次太少而无法总结；即使上下文已满，单个大粘贴填充它时也可能发生这种情况
2. 在主要任务之间关闭并重启 Claude Code
3. 考虑将大型构建目录添加到您的 `.gitignore` 文件
4. 使用 [`claude --safe-mode`](https://code.claude.com/docs/zh-CN/cli-reference#cli-flags) 重启以检查插件、MCP 服务器或 hook 是否是源头。它禁用会话的所有自定义；如果使用量下降，请参阅[调试您的配置](https://code.claude.com/docs/zh-CN/debug-your-config.md#test-against-a-clean-configuration)以找出是哪一个

如果会话的堆内存超过 2.5GB，会出现严重内存使用警告。要释放内存，请重启 Claude Code 并运行 [`claude --continue`](https://code.claude.com/docs/zh-CN/cli-reference#cli-flags) 以在新进程中恢复对话。

在[全屏渲染](https://code.claude.com/docs/zh-CN/fullscreen.md)之外，运行 `/compact` 也会释放内存。一旦内存使用量降至 2.5GB 以下，警告就会消失。

如果内存使用在这些步骤后仍然很高，请运行 `/heapdump` 以将两个文件写入 `~/Desktop`：一个名为 `<session-id>.heapsnapshot` 的 JavaScript 堆快照和一个名为 `<session-id>-diagnostics.json` 的内存分解。Claude Code [从命令菜单中隐藏该命令](https://code.claude.com/docs/zh-CN/commands.md#how-the-command-menu-matches-what-you-type)；请完整输入它。在没有 Desktop 文件夹的 Linux 上，文件被写入您的主目录。

> [!WARNING]
>
> `.heapsnapshot` 文件包含进程中的每个字符串，包括您的完整对话和凭证。不要将其附加到公开问题或共享。

该命令还在对话中打印摘要，显示进程的总内存、其中有多少在 JS 堆中，以及有多少在堆外。摘要还列出任何泄漏指示器，例如高内存增长率或异常高的打开句柄数。摘要说明大部分内存是在 JS 堆中（快照捕获），还是在本机内存中（它不捕获）。

对输出执行以下两项操作之一：

* **报告它**：打开 [GitHub 问题](https://github.com/anthropics/claude-code/issues)并仅附加 `-diagnostics.json` 文件，该文件包含打印摘要背后的统计信息，不包含任何对话内容或凭证
* **自己调查它**：如果摘要说大部分内存是 JS 堆，请在 Chrome DevTools 中的 Memory → Load 下打开 `.heapsnapshot` 文件，并按保留大小排序以查看什么在保留内存

如果摘要说大部分内存是本机内存，快照无法显示它；请在您的报告中包含摘要的泄漏指示器。

### 大型表格在终端中被截断

超过 200 行的 Markdown 表格呈现其前 200 行，后跟 `… N more rows not shown` 行。仅显示被限制：完整表格保留在对话中，[`/copy`](https://code.claude.com/docs/zh-CN/commands) 复制每一行。对于在终端中太大而无法读取的表格，请要求 Claude 将其写入文件。在 v2.1.208 之前，Claude Code 呈现每一行，因此恢复包含非常大表格的会话可能会在重新呈现时停滞。

### 自动压缩停止并出现抖动错误

如果您看到 `Autocompact is thrashing: the context refilled to the limit...`，自动压缩成功，但文件或工具输出立即多次将上下文窗口重新填充到限制。Claude Code 停止重试以避免在没有取得进展的循环上浪费 API 调用。

要恢复：

1. 要求 Claude 以较小的块读取超大文件，例如特定行范围或函数，而不是整个文件
2. 运行 `/compact`，重点是删除大输出，例如 `/compact keep only the plan and the diff`
3. 将大文件工作移到 [subagent](https://code.claude.com/docs/zh-CN/sub-agents.md)，以便它在单独的上下文窗口中运行
4. 如果早期对话不再需要，运行 `/clear`

如果在 `/clear` 之后错误再次出现，请运行 [`/context`](https://code.claude.com/docs/zh-CN/debug-your-config) 并将 `Messages` 行与其上方的行进行比较：

* **`Messages` 是最大的行**：新对话中的文件或工具输出正在重新填满窗口，因此请再次执行步骤 1 到 3
* **其他行加起来更大**：会话启动时加载的内容留下的工作空间太少，因此请[精简启动时加载的内容](https://code.claude.com/docs/zh-CN/errors.md#prompt-is-too-long)

### 命令挂起或冻结

如果 Claude Code 似乎无响应：

1. 按 Ctrl+C 尝试取消当前操作
2. 如果无响应，您可能需要关闭终端并重新启动

重新启动不会丢失您的对话。在同一目录中运行 `claude --resume` 以继续会话。

### 编辑器集成终端中的文本乱码或损坏

如果在 VS Code、Cursor 或 Devin Desktop 集成终端中运行 Claude Code 时字符呈现为方框、涂抹或错误的字形，终端的 GPU 渲染器可能是原因。在 Claude Code 中运行 `/terminal-setup` 以将 `terminal.integrated.gpuAcceleration` 设置为 `"off"`，或在编辑器设置中手动设置并重新加载窗口。有关 `/terminal-setup` 写入的其他设置，请参阅[终端配置](https://code.claude.com/docs/zh-CN/terminal-config.md)。

### 启动时出现多余的 `>0q` 或退出后出现 `>4m`

这些字符由基于 0.53 之前版本 VTE 库构建的终端打印，是两个 Claude Code 控制序列的末尾部分。要消除这些文本，请将 Claude Code [更新](https://code.claude.com/docs/zh-CN/setup.md#update-claude-code)到 v2.1.301 或更高版本，该版本在识别出此类终端时会跳过这两个序列。

如果文本仍然存在，请在终端的 shell 中打印 VTE 版本：

```bash theme={null}
echo $VTE_VERSION
```

在基于 VTE 的终端上，结果低于 `5300` 表示库版本早于 0.53：请更新或更换终端以获取 VTE 0.53 或更高版本。否则，请参阅[获取更多帮助](#get-more-help)。

### 全屏渲染中鼠标滚轮一次滚动一行

在[全屏渲染](https://code.claude.com/docs/zh-CN/fullscreen.md)中，Claude Code 滚动对话本身，而不是将其留给您的终端。如果每个滚轮缺口移动的行数少于您想要的，请运行 `/scroll-speed` 以提高每个缺口的行数并保存它，或设置 `CLAUDE_CODE_SCROLL_SPEED` 环境变量，除了在 JetBrains IDE 终端中，Claude Code 应用其自己的滚动处理，两者都不起作用。有关每个接受的值，请参阅[鼠标滚轮滚动](https://code.claude.com/docs/zh-CN/fullscreen.md#mouse-wheel-scrolling)。

要在不改变速度的情况下移动得更快，请按 `PgUp` 和 `PgDn` 一次滚动半屏。要将滚动交还给您的终端的本机滚回，请运行 `/tui default` 以切换到经典渲染器。

### 沙箱内的剪贴板命令（如 `pbcopy`）失败

当[沙箱](https://code.claude.com/docs/zh-CN/sandboxing.md)打开时，剪贴板实用程序（如 `pbcopy`、`xclip` 和 `wl-copy`）可能无法从沙箱化 Bash 命令内部到达系统剪贴板，在 Claude 将文本管道到它们后保持您的剪贴板不变。

要将 Claude 的输出放在您的剪贴板上，请要求 Claude 在其响应中打印内容，然后运行 [`/copy`](https://code.claude.com/docs/zh-CN/commands)。`/copy` 从 Claude Code 进程本身而不是从沙箱化命令写入剪贴板，因此沙箱不会阻止它。它可以复制单个代码块而不是整个响应，它还将复制的内容写入文件并打印路径，这在剪贴板写入无法到达您的终端时提供回退，例如通过 SSH。

当 Claude 将文本管道到这些工具之一时，将 `pbcopy *`、`wl-copy *` 或 `xclip *` 添加到 [`excludedCommands`](https://code.claude.com/docs/zh-CN/settings-reference#sandbox-excludedcommands) 本身不会将该调用从沙箱中取出。

### 复制的文本在 SSH 上无法到达您的本地剪贴板

当 Claude Code 通过 SSH 在远程机器上运行时，它无法在您的本地机器上运行剪贴板工具。在 tmux 外，当您在[全屏渲染](https://code.claude.com/docs/zh-CN/fullscreen.md)中选择文本或运行 `/copy` 时，Claude Code 会将文本作为 OSC 52 转义序列发送到您的终端。您的终端决定是否将其放在您的剪贴板上。`/copy` 报告 `Copied to clipboard`，无论文本是否到达，在 tmux 外，选择通知读取 `sent N chars via OSC 52`。

某些终端不对 OSC 52 进行操作。iTerm2 忽略它，直到您打开**Settings > General > Selection > Applications in terminal may access clipboard**，macOS Terminal.app 不支持它。

要在没有 OSC 52 的情况下获取文本：

* 按住您的终端的本机选择键同时拖动，然后使用您的终端的常用快捷方式复制，例如 `Cmd+C`。该键在 Terminal.app 中是 `Fn`，在 iTerm2 中是 `Option`。[保持本机文本选择](https://code.claude.com/docs/zh-CN/fullscreen.md#keep-native-text-selection)为其他终端列出了它。
* 在远程机器上设置 [`CLAUDE_CODE_DISABLE_MOUSE=1`](https://code.claude.com/docs/zh-CN/env-vars)，以便您的终端为整个会话处理选择。

### 搜索和发现问题

如果搜索工具、`@file` 提及、自定义代理或自定义 skills 找不到文件，捆绑的 `ripgrep` 二进制文件可能无法在您的系统上运行。安装您平台的 `ripgrep` 包并告诉 Claude Code 改用它：

> [tab] macOS
>
> ```bash theme={null}
> brew install ripgrep
> ```

> [tab] Ubuntu/Debian
>
> ```bash theme={null}
> sudo apt install ripgrep
> ```

> [tab] Alpine
>
> ```bash theme={null}
> apk add ripgrep
> ```
>
> `ripgrep` 在 Alpine 的社区存储库中。如果 `apk` 报告包缺失，请参阅 [Alpine Linux 设置](https://code.claude.com/docs/zh-CN/setup.md#alpine-linux-and-musl-based-distributions)。

> [tab] Arch
>
> ```bash theme={null}
> pacman -S ripgrep
> ```

> [tab] Windows
>
> ```powershell theme={null}
> winget install BurntSushi.ripgrep.MSVC
> ```

然后在您的 shell [环境](https://code.claude.com/docs/zh-CN/env-vars.md)中或在您的 [`settings.json`](https://code.claude.com/docs/zh-CN/settings-reference#all-settings) 的 `env` 块中将 `USE_BUILTIN_RIPGREP` 设置为 `0`：

```json theme={null}
{
  "env": {
    "USE_BUILTIN_RIPGREP": "0"
  }
}
```

要确认切换生效，请在您的终端中运行 `claude doctor` 并检查搜索行显示您的系统 ripgrep 的路径而不是 `OK (bundled)`。

### WSL 上的搜索速度缓慢或结果不完整

在 WSL 上[跨文件系统工作](https://learn.microsoft.com/en-us/windows/wsl/filesystems)时的磁盘读取性能损失可能导致使用 Claude Code 在 WSL 上时搜索匹配数少于预期。搜索仍然有效，但返回的结果少于本机文件系统。

> [!NOTE]
>
> `claude doctor` 在这种情况下将搜索显示为正常。

**解决方案：**

1. **提交更具体的搜索**：通过指定目录或文件类型来减少搜索的文件数："在 auth-service 包中搜索 JWT 验证逻辑"或"在 JS 文件中查找 md5 哈希的使用"。

2. **将项目移到 Linux 文件系统**：如果可能，确保您的项目位于 Linux 文件系统（`/home/`）而不是 Windows 文件系统（`/mnt/c/`）。

3. **改用本机 Windows**：考虑在 Windows 上本机运行 Claude Code 而不是通过 WSL，以获得更好的文件系统性能。

## 获取更多帮助

如果您遇到此处未涵盖的问题：

1. 运行 `/doctor` 进行设置检查，运行 `/mcp` 检查 MCP 服务器状态
2. 在 Claude Code 中使用 `/feedback` 命令直接向 Anthropic 报告问题
3. 检查 [GitHub 存储库](https://github.com/anthropics/claude-code) 以了解已知问题
4. 直接向 Claude 询问其功能和特性。Claude 可以内置访问其文档。

如有账户、账单或订阅问题，请改为联系 Anthropic 支持：登录 [claude.ai](https://claude.ai)（Console 用户：[platform.claude.com](https://platform.claude.com)），点击左下角的您的首字母缩写，然后选择**获取帮助**。请参阅[如何获取支持](https://support.claude.com/en/articles/9015913-how-to-get-support)了解完整流程，包括每个计划中谁可以联系人工代理。
