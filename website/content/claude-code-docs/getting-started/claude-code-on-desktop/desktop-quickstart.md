> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# 开始使用桌面应用

> 安装 Claude 桌面应用，打开 Code 选项卡，并在您计算机上的项目文件夹中开始您的第一个 Claude Code 会话。

桌面应用为您提供具有图形界面的 Claude Code。您可以要求 Claude 处理计算机上文件夹中的代码，然后自行查看更改，而无需使用终端。

Claude Code 位于应用侧边栏顶部的 **Code** 选项卡中。本页面将指导您安装应用并在其中开始您的第一个会话。Claude Code 需要 [Pro、Max、Ultra、Team 或 Enterprise 订阅](https://claude.com/pricing?utm_source=claude_code\&utm_medium=docs\&utm_content=desktop_quickstart_pricing)。

> [card] [下载 macOS 版本](https://claude.ai/api/desktop/darwin/universal/dmg/latest/redirect?utm_source=claude_code\&utm_medium=docs)
>
> 适用于 Intel 和 Apple Silicon 的通用版本

> [card] [下载 Windows 版本](https://claude.ai/api/desktop/win32/x64/setup/latest/redirect?utm_source=claude_code\&utm_medium=docs)
>
> 适用于 x64 处理器

> [card] [获取 Claude for Linux（测试版）](https://code.claude.com/docs/zh-CN/desktop-linux.md)
>
> Ubuntu 和 Debian 的 apt 或 .deb

对于 Windows ARM64，请下载 [ARM64 安装程序](https://claude.ai/api/desktop/win32/arm64/setup/latest/redirect?utm_source=claude_code\&utm_medium=docs)。在 Linux 上，使用 apt 安装；请参阅 [Claude Desktop on Linux](https://code.claude.com/docs/zh-CN/desktop-linux.md)。

> [!NOTE]
>
> 这些情况在其他页面中有介绍：
>
> * **已经设置完成**：请参阅[使用 Claude Code Desktop](https://code.claude.com/docs/zh-CN/desktop.md)了解 Code 选项卡可以执行的所有操作
> * **想在您的终端中使用 `claude`**：[单独安装 CLI](https://code.claude.com/docs/zh-CN/quickstart.md)
> * **使用 Chat 或 Cowork**：请参阅 [Claude 帮助中心](https://support.claude.com/)
> * **需要了解有关安装或部署桌面应用的更多信息**：请参阅 [Claude Desktop 支持文章](https://support.claude.com/en/collections/16163169-claude-desktop)

## 安装

**1. 安装并登录**

在 macOS 和 Windows 上，从上面的链接下载安装程序并运行它。在 Linux 上，请按照 [Linux 上的 Claude Desktop](https://code.claude.com/docs/zh-CN/desktop-linux.md) 中的安装步骤进行操作。在 macOS 上从应用程序文件夹启动 Claude，在 Windows 上从开始菜单启动，或在 Linux 上从应用程序启动器启动，然后使用您的 Anthropic 账户登录。

**2. 打开 Code 选项卡**

点击侧边栏顶部的 **Code** 选项卡。如果选项卡仅显示图标，将鼠标悬停在图标上即可查看其名称。如果点击 **Code** 后出现提示或错误，请按照对应的解决方法操作：

* **提示升级**：您需要先[订阅付费计划](https://claude.com/pricing?utm_source=claude_code\&utm_medium=docs\&utm_content=desktop_quickstart_upgrade)
* **提示在线登录**：完成登录并重启应用
* **403 错误**：请参阅[身份验证故障排除](https://code.claude.com/docs/zh-CN/desktop.md#403-or-authentication-errors-in-the-code-tab)

桌面应用包含 Claude Code，因此您无需安装 Node.js 或 CLI 即可使用 Code 选项卡。

## 开始您的第一个会话

打开代码选项卡，选择一个项目，并告诉 Claude 要做什么。

**1. 选择环境和文件夹**

在[输入框下方的一行](https://code.claude.com/docs/zh-CN/desktop.md#start-a-session)中，点击第一个控件（即环境选择器），然后选择**本地**分组中的第一项。接着点击环境选择器旁边的项目文件夹控件，选择**打开文件夹…**，并选择您的项目目录。

如果新会话的输入框下方没有显示环境，说明您使用的是早期布局。请参阅[检查您使用的布局](https://code.claude.com/docs/zh-CN/desktop.md#check-which-layout-you-have)。

> [!TIP]
>
> 从一个您熟悉的小项目开始。这是查看 Claude Code 能做什么的最快方式。

您也可以选择：

* **云**：在云中运行会话，即使关闭应用也能继续。请参阅 [在云中使用 Claude Code](https://code.claude.com/docs/zh-CN/claude-code-on-the-web.md) 了解云会话的工作原理。
* **SSH**：通过 SSH 连接到远程机器，例如您自己的服务器、云虚拟机或开发容器。请先[添加连接](https://code.claude.com/docs/zh-CN/desktop.md#ssh-sessions)。桌面版在您第一次连接时会自动在远程机器上安装 Claude Code。正在运行的任务可以[在桌面版断开连接时继续运行](https://code.claude.com/docs/zh-CN/desktop.md#keep-a-task-running-while-desktop-is-disconnected)，但如果使用 Claude 账户，任务可能会在其登录过期时停止。
* **WSL**（Windows）：在 [WSL 2 发行版](https://code.claude.com/docs/zh-CN/desktop-wsl.md) 内运行会话；Claude Code、工具和 git 在 Linux 端执行，使用本机路径。

**2. 选择模型**

从输入框下方一行靠右侧的模型选择器中选择一个模型。请参阅 [模型](https://code.claude.com/docs/zh-CN/model-config.md#available-models) 以比较可用的模型。您可以稍后从模型选择器更改模型。

**3. 告诉 Claude 要做什么**

输入您想让 Claude 做的事情：

* `Find a TODO comment and fix it`
* `Add tests for the main function`
* `Create a CLAUDE.md with instructions for this codebase`

[会话](https://code.claude.com/docs/zh-CN/desktop.md#work-in-parallel-with-sessions) 是与 Claude 关于您的代码的对话。每个会话跟踪自己的上下文和更改。

**4. 审查并接受更改**

查看输入框下方一行中的[权限模式](https://code.claude.com/docs/zh-CN/desktop.md#choose-a-permission-mode)，了解 Claude 是否会等待您的批准：

* **自动或接受编辑**：Claude 应用其文件更改，并显示一个指示器（如 `+12 -1`），以便您可以在 [diff 视图](https://code.claude.com/docs/zh-CN/desktop.md#review-changes-with-diff-view)中审查它们
* **手动**：Claude 提议每项更改，并在应用前等待您的批准

在手动模式下，您将看到：

1. 一个 diff，准确显示将要发生的更改
2. 用于批准或拒绝每项更改的按钮
3. Claude 处理您的请求时的实时更新

## 接下来呢？

您已经进行了第一次编辑。有关 Desktop 可以执行的所有操作的完整参考，请参阅 [使用 Claude Code Desktop](https://code.claude.com/docs/zh-CN/desktop.md)。以下是一些可以尝试的操作。

**中断并调整方向。** 您可以随时重定向 Claude。点击停止按钮立即中断，或输入更正并按 **Enter** 发送，无需停止正在运行的操作。无论哪种方式，您都不必等待它完成或重新开始。

**为 Claude 提供更多上下文。** 在提示框中输入 `@filename` 以将特定文件拉入对话。Claude 拥有的上下文越多，结果就越好。请参阅 [添加文件和上下文](https://code.claude.com/docs/zh-CN/desktop.md#add-files-and-context-to-prompts)。

**使用 skills 处理可重复的任务。** 输入 `/` 或点击 **+** → **Slash commands** 以浏览 [内置命令](https://code.claude.com/docs/zh-CN/commands.md)、[自定义 skills](https://code.claude.com/docs/zh-CN/skills.md) 和插件 skills。Skills 是可重用的提示，您可以在需要时调用它们，例如代码审查清单或部署步骤。

**在提交前审查更改。** Claude 编辑文件后，会出现 `+12 -1` 指示符。点击它以打开 [diff 视图](https://code.claude.com/docs/zh-CN/desktop.md#review-changes-with-diff-view)，逐个文件审查修改，并对特定行进行评论。Claude 会读取您的评论并进行修订。要让 Claude 自行审查这些更改，请在提示框中输入 [`/code-review`](https://code.claude.com/docs/zh-CN/desktop#review-your-code)。

**调整您拥有的控制权。** 您的 [权限模式](https://code.claude.com/docs/zh-CN/desktop.md#choose-a-permission-mode) 设置了 Claude 在不请求批准的情况下可以执行的操作：

* **Auto**：分类器在后台审查操作，并阻止风险操作，而不是询问您。
* **Manual**：Claude 在编辑文件或运行命令前询问。
* **Accept edits**：Claude 自动接受文件编辑以加快迭代。
* **Plan**：Claude 提出一种方法而不编辑任何文件，这在大型重构前很有用。

**添加插件以获得更多功能。** 安装 [插件](https://code.claude.com/docs/zh-CN/desktop.md#install-plugins)，这些插件可添加 skills、Agent、MCP 服务器等。

**整理您的工作区。** 将聊天、diff、终端、文件和浏览器窗格拖动到您想要的任何布局中。使用 **Ctrl+\`** 打开终端以在您的会话旁边运行命令，或点击文件路径以打开它。请参阅 [整理您的工作区](https://code.claude.com/docs/zh-CN/desktop.md#arrange-your-workspace)。

**预览您的应用。** 当您在 desktop 中运行开发服务器时，您的应用会在浏览器窗格中打开，该窗格也可以 [打开外部网站](https://code.claude.com/docs/zh-CN/desktop.md#browse-external-sites)。Claude 可以查看正在运行的应用、测试端点、检查日志并对其看到的内容进行迭代。请参阅 [预览您的应用](https://code.claude.com/docs/zh-CN/desktop.md#preview-your-app)。

**跟踪您的拉取请求。** 打开 PR 后，Claude Code 会监控 CI 检查结果，并可以自动修复失败。请参阅 [监控拉取请求状态](https://code.claude.com/docs/zh-CN/desktop.md#monitor-pull-request-status)。

**将 Claude 放在日程上。** 设置 [定时任务](https://code.claude.com/docs/zh-CN/desktop-scheduled-tasks.md) 以定期自动运行 Claude：每天早上进行代码审查、每周进行依赖项审计，或从您连接的工具中提取信息的简报。

**准备好时扩展。** 从侧边栏打开 [并行会话](https://code.claude.com/docs/zh-CN/desktop.md#work-in-parallel-with-sessions) 以同时处理多个任务，可选择每个任务都在其自己的 Git worktree 中，并打开 [任务窗格](https://code.claude.com/docs/zh-CN/desktop.md#watch-background-tasks) 以观看会话正在运行的子代理和后台命令。打开 [侧边聊天](https://code.claude.com/docs/zh-CN/desktop.md#ask-a-side-question-without-derailing-the-session) 以提出问题而不偏离主线程。将 [长时间运行的工作发送到云端](https://code.claude.com/docs/zh-CN/desktop.md#run-long-running-tasks-in-the-cloud) 以便即使关闭应用也能继续，或 [将您已开始的会话移至云端](https://code.claude.com/docs/zh-CN/desktop.md#continue-in-another-surface)（如果任务花费的时间比预期长）。[连接外部工具](https://code.claude.com/docs/zh-CN/desktop.md#extend-claude-code)（如 GitHub、Slack 和 Linear）以整合您的工作流。

## 接下来

* [使用 Claude Code Desktop](https://code.claude.com/docs/zh-CN/desktop.md)：权限模式、并行会话、差异视图、连接器和企业配置
* [从 CLI 迁移过来？](https://code.claude.com/docs/zh-CN/desktop.md#coming-from-the-cli)：在同一项目上运行 Desktop 和 CLI，并比较功能、标志等效项以及 Desktop 中不可用的功能
* [故障排除](https://code.claude.com/docs/zh-CN/desktop.md#troubleshooting)：常见错误和设置问题的解决方案
* [最佳实践](https://code.claude.com/docs/zh-CN/best-practices.md)：编写有效提示和充分利用 Claude Code 的提示
* [常见工作流](https://code.claude.com/docs/zh-CN/common-workflows.md)：调试、重构、测试等教程
