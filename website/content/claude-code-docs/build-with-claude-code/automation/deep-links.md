> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# 从链接启动会话

> 从 URL 打开 Claude Code 终端会话。在运行手册、警报和仪表板中嵌入 `claude-cli://` 链接，这样点击即可在正确的仓库中打开 Claude Code，并使用正确的提示。

深链接是一个 `claude-cli://` URL，它在新的终端窗口中打开 Claude Code。该 URL 可以携带工作目录和预填充的提示。

提示词已填充但在您按 Enter 之前不会发送。要让链接能够在某台机器上打开，请先在该机器上的交互式 Claude Code 会话中发送一条提示词，以便 Claude Code [注册其 `claude-cli://` 处理程序](#registration-and-supported-platforms)。

因为深链接是一个 URL，你可以在任何可以放置链接的地方放置它：

* 一个事件运行手册步骤，打开受影响服务的仓库并带有诊断提示
* 一个监控警报或仪表板，链接到特定指标的调查提示
* 一个 README 或 wiki 页面，打开项目并带有入职提示
* 一个 CI 失败通知，预填充失败作业的名称

本页面涵盖如何[构建链接](#build-a-link)、[在运行手册中嵌入链接或从 shell 触发](#examples)，以及[在每个平台上管理或禁用处理程序注册](#registration-and-supported-platforms)。

## 深链接的工作原理

`claude-cli://` 前缀是一个自定义 URL 方案，Claude Code 向你的操作系统注册，类似于 `mailto:` 链接打开你的电子邮件客户端的方式。当你点击一个深链接时：

1. 浏览器或应用将 URL 传递给你的操作系统。
2. 操作系统识别 `claude-cli://` 前缀并在你的机器上启动 Claude Code。
3. 一个新的终端窗口打开，Claude Code 在链接指定的目录中运行，链接的提示文本已经在输入框中。
4. 你阅读提示，如果需要可以编辑它，然后按 Enter 发送它。

链接本身可以托管在任何地方，但会话总是在你点击的计算机上本地打开。请参阅[注册和支持的平台](#registration-and-supported-platforms)了解在每个操作系统上打开哪个终端模拟器。

显示链接的平台必须允许自定义 URL 方案。关于 GitHub 如何处理它们以及解决方法，请参阅[链接呈现为纯文本而不是可点击](#the-link-renders-as-plain-text-instead-of-being-clickable)。

### 启动的会话显示什么

深链接本身永远不会执行任何操作。链接只选择一个目录并填充提示框。如果你从你不信任的页面点击链接，提示仍然是惯性的：在你阅读填充的内容并按 Enter 之前，没有任何东西到达模型。

当会话打开时，输入框下方的警告行显示 `来自外部链接的提示`，并在你发送或清除提示之前保持可见。对于超过 1,000 个字符的提示，警告包括字符计数，并告诉你在按 Enter 之前滚动并查看完整文本，因为长提示可能会将指令推出屏幕。权限规则、`CLAUDE.md` 和所选目录的信任提示的应用方式与任何其他会话相同。

## 构建链接

深链接以 `claude-cli://open` 开头，后跟可选的查询参数。最小形式在你的主目录中打开 Claude Code，带有空提示：

```text theme={null}
claude-cli://open
```

要在不将其放在页面上的情况下尝试链接，请将其粘贴到浏览器的地址栏中或[从 shell 打开它](#open-a-link-from-the-shell)。

添加参数以控制会话开始的位置和提示框包含的内容：

| 参数 | 描述 |
| - | - |
| `q` | 在提示框中预填充的文本。[URL 编码](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/encodeURIComponent)该值。在多行提示中使用 `%0A` 表示换行符。最多 5,000 个字符。 |
| `cwd` | 用作工作目录的绝对路径。网络和 UNC 路径被拒绝，包含 `..` 段或不可见或双向控制字符的路径也被拒绝。 |
| `repo` | 一个 GitHub `owner/name` slug。Claude Code 将其解析为它之前看到的本地克隆并从那里开始。如果你没有匹配的克隆，会话将在你的主目录中打开。 |

`cwd` 和 `repo` 是[设置工作目录的两种方式](#choose-between-cwd-and-repo)。如果你同时传递两者，`cwd` 优先，`repo` 被忽略，即使 `cwd` 路径不存在。

以下链接指向一个名为 `acme/payments` 的仓库，带有两行诊断提示。在构建你自己的链接时，将 `acme/payments` 替换为你的仓库的 `owner/name` slug：

```text theme={null}
claude-cli://open?repo=acme/payments&q=Investigate%20the%20failed%20deploy%20of%20payments-api.%0ACheck%20recent%20commits%20to%20main%20and%20the%20last%20successful%20build.
```

点击它会打开一个新的终端窗口，在你的 `acme/payments` 本地克隆中启动 Claude Code，并用解码的文本填充提示框：

```text theme={null}
Investigate the failed deploy of payments-api.
Check recent commits to main and the last successful build.
```

你可以在按 Enter 发送之前编辑提示。请参阅[在 `cwd` 和 `repo` 之间选择](#choose-between-cwd-and-repo)了解当你有多个克隆或 worktrees 时如何选择本地路径。

### 在 `cwd` 和 `repo` 之间选择

当点击链接的每个人都在相同的绝对路径上有项目时，使用 `cwd`，例如标准化的 devcontainer 或 VM 镜像。

当链接被共享且每个人克隆到不同位置时，使用 `repo`。Claude Code 按如下方式将 slug 解析为本地路径：

* `repo` 打开你最近运行 `claude` 的链接仓库的克隆或 worktree。每次你在 Git 仓库中运行 `claude` 时，Claude Code 都会针对仓库的 GitHub `owner/name` slug 记录该目录的路径。Claude Code 单独跟踪克隆和 worktrees。
* 链接不改变检出的分支。会话在该目录当前所处的任何状态下打开。

启动的会话显示它选择了哪个路径，所以你可以确认正确的克隆已打开。

## 示例

下面的部分展示了两种常见的使用深链接的方式：作为文档中的 Markdown 链接和作为脚本或 shell 别名中的命令。

### 在运行手册中嵌入链接

运行手册中的深链接为进行分类的人提供了一种一键式的方式，可以在正确的仓库中开始调查，并带有准备好的提示。呈现运行手册的平台必须允许自定义 URL 方案。GitHub 呈现的 Markdown 不允许 `claude-cli://`，所以 GitHub README、问题或 wiki 中的深链接只显示其标签，没有可点击的链接。请参阅[故障排除说明](#the-link-renders-as-plain-text-instead-of-being-clickable)了解解决方法。

提示是 URL 的一部分，必须进行 URL 编码。要生成编码值，请在浏览器控制台或任何 URL 编码器中通过 `encodeURIComponent` 传递你的提示文本。

下面的示例为名为 `web-gateway` 的服务的事件运行手册添加了一个调查入口点：

```markdown theme={null}
## High 5xx rate on web-gateway

1. Acknowledge the page in PagerDuty.
2. [Open Claude Code in the gateway repo](claude-cli://open?repo=acme/web-gateway&q=5xx%20rate%20is%20elevated%20on%20web-gateway.%20Check%20recent%20deploys%2C%20error%20logs%20from%20the%20last%2030%20minutes%2C%20and%20open%20incidents%20in%20Linear.)
3. Post initial findings in #incident.
```

要在您自己的运行手册中使用这个，将 `acme/web-gateway` 替换为您的服务的仓库 slug。在工程师点击第 2 步之前，他们必须已在自己的机器上[在交互式 Claude Code 会话中发送过提示词](#registration-and-supported-platforms)，并且已在其本地克隆中打开过 Claude Code。

### 从 shell 打开链接

您也可以从 shell 脚本、别名或自动化中打开深链接，而不是通过点击它。使用链接作为参数调用您的操作系统的 URL 打开命令。这些命令依赖于 Claude Code 在机器上[注册](#registration-and-supported-platforms)的处理程序。

> [tab] macOS
>
> 内置的 `open` 命令将 URL 传递给注册的 `claude-cli://` 处理程序：
>
> ```bash theme={null}
> open "claude-cli://open?repo=acme/payments&q=review%20open%20PRs"
> ```
>
> 成功时，会打开一个新的终端窗口，Claude Code 运行，提示已预填充。

> [tab] Linux
>
> 大多数桌面环境提供 `xdg-open`，它将 URL 传递给注册的处理程序：
>
> ```bash theme={null}
> xdg-open "claude-cli://open?repo=acme/payments&q=review%20open%20PRs"
> ```
>
> 成功时，会打开一个新的终端窗口，Claude Code 运行，提示已预填充。如果 shell 报告找不到 `xdg-open`，请参阅[故障排除](#xdg-open-is-not-found-on-linux)。

> [tab] Windows
>
> 在 PowerShell 中，`Start-Process` 将 URL 传递给注册的处理程序：
>
> ```powershell theme={null}
> Start-Process "claude-cli://open?repo=acme/payments&q=review%20open%20PRs"
> ```
>
> 在 `cmd.exe` 中，`start` 将其第一个带引号的参数视为窗口标题，所以在 URL 之前传递一个空标题：
>
> ```cmd theme={null}
> start "" "claude-cli://open?repo=acme/payments&q=review%20open%20PRs"
> ```
>
> 成功时，会打开一个新的终端窗口，Claude Code 运行，提示已预填充。

## 注册和支持的平台

在 macOS、Linux 和 Windows 上，当您执行以下任一操作时，Claude Code 会向您的操作系统注册 `claude-cli://` 处理程序：

* 向交互式 `claude` 会话发送任何内容，例如一条消息或 `/help`。仅启动会话而不发送任何内容不会注册处理程序。
* 在您已[信任](https://code.claude.com/docs/zh-CN/permissions.md#project-allow-rules-and-workspace-trust)的文件夹中以 [Agent 视图](https://code.claude.com/docs/zh-CN/agent-view.md)启动 Claude Code，例如使用 `claude agents`

您不需要运行单独的安装命令。注册仅写入用户级位置：

| 平台 | 处理程序位置 |
| - | - |
| macOS | `~/Applications/Claude Code URL Handler.app` |
| Linux | `claude-code-url-handler.desktop` 在 `$XDG_DATA_HOME/applications` 下，默认为 `~/.local/share/applications` |
| Windows | `HKEY_CURRENT_USER\Software\Classes\claude-cli` |

处理程序在检测到的终端模拟器中启动 Claude Code。在 macOS 上，Claude Code 记住您最近交互式会话中的终端并重复使用它，支持 iTerm2、Ghostty、kitty、Alacritty、WezTerm 和 Terminal.app。在 Linux 上，它遵守 `$TERMINAL` 环境变量，然后是 `x-terminal-emulator`，然后是常见模拟器的列表。在 Windows 上，它优先选择 Windows Terminal，然后是 PowerShell，然后是 `cmd.exe`。

要完全防止注册，在 `settings.json` 中将 [`disableDeepLinkRegistration`](https://code.claude.com/docs/zh-CN/settings-reference#disabledeeplinkregistration) 设置为 `"disable"`。要在整个组织中强制执行此操作，使用户无法重新启用它，请改为在[托管设置](https://code.claude.com/docs/zh-CN/server-managed-settings.md)中设置它。Claude Code 的 `disableDeepLinkRegistration` 设置仅适用于 `claude-cli://` 链接。

## 打开 VS Code 标签页而不是终端

VS Code 扩展在 `vscode://anthropic.claude-code/open` 注册自己的处理程序，它打开一个 Claude Code 编辑器标签页而不是终端窗口。请参阅[从其他工具启动 VS Code 标签页](https://code.claude.com/docs/zh-CN/vs-code.md#launch-a-vs-code-tab-from-other-tools)了解该 URL 的参数。

## 在 SSH 连接上打开 Claude Desktop 会话

Claude Desktop 会自行处理其 `claude://` 链接。要通过 SSH 在远程机器上打开 Desktop 会话，请参阅[通过链接打开 SSH 会话](https://code.claude.com/docs/zh-CN/desktop.md#open-an-ssh-session-from-a-link)。

## 故障排除

### 点击链接没有反应

处理程序可能还没有注册。在交互式会话中，注册发生在您发送内容时，而不是在会话启动时。在该机器上启动一个交互式 `claude` 会话，发送任意提示词，退出，然后再试一次链接。如果您在没有桌面环境的 Linux 上，`xdg-open` 可能没有东西可以分派。

### 在 Linux 上找不到 xdg-open

`xdg-open` 命令是 `xdg-utils` 包的一部分，最小服务器镜像、容器和 WSL 发行版通常会省略它。使用你的发行版的包管理器安装 `xdg-utils`，例如 `sudo apt install xdg-utils`，然后再次运行该命令。如果命令随后运行但没有打开任何内容，`xdg-open` 可能没有桌面环境可以分派；请参阅[点击链接没有反应](#clicking-the-link-does-nothing)。

### 链接呈现为纯文本而不是可点击的

某些 Markdown 渲染器只允许 `http` 和 `https` 链接，并删除其他 URL 方案。GitHub 在 README、问题、Pull Request 和 wiki 中这样做：`[label](claude-cli://...)` 呈现为仅 `label`，没有链接，URL 被删除。在这些平台上，将深链接放在代码块中，以便读者可以看到 URL 并将其粘贴到浏览器的地址栏中。

### 会话在我的主目录中打开而不是仓库

`repo` 参数只解析 Claude Code 已经看到的克隆。在克隆中运行一次 `claude` 以记录其路径，或将链接切换为使用带有绝对路径的 `cwd`。

### 链接打开了错误的终端

在 macOS 上，在你首选的终端中启动一次 `claude`，下一个深链接将使用它。在 Linux 上，将 `$TERMINAL` 环境变量设置为你首选的模拟器的命令名称。在 Windows 上，顺序是固定的：如果你想链接在那里打开而不是在 PowerShell 或 `cmd.exe` 窗口中打开，请安装 Windows Terminal。

## 了解更多

这些页面涵盖了启动或扩展 Claude Code 会话的相关方式：

* [Skills](https://code.claude.com/docs/zh-CN/skills.md)：将长运行手册提示存储为仓库中的 `/skill`，以便深链接的 `q` 参数只需命名它
* [Non-interactive mode](https://code.claude.com/docs/zh-CN/headless.md)：从脚本运行 Claude 并捕获输出而不打开终端
