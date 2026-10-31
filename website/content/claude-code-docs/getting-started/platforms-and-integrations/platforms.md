> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# 平台和集成

> 选择在哪里运行 Claude Code 以及连接什么工具。比较 CLI、Desktop、VS Code、JetBrains、Web 以及 Chrome、Slack 和 CI/CD 等集成。

Claude Code 在任何地方运行相同的底层引擎，但每个界面都针对不同的工作方式进行了优化。本页面帮助您为工作流选择合适的平台，并连接您已经使用的工具。

## 在哪里运行 Claude Code

根据您喜欢的工作方式和项目所在位置选择平台。

| 平台 | 最适合 | 您获得的功能 |
| :- | :- | :- |
| [CLI](https://code.claude.com/docs/zh-CN/quickstart.md) | 终端工作流、脚本编写、远程服务器 | 完整功能集、[Agent SDK](https://code.claude.com/docs/zh-CN/headless.md)、[计算机使用](https://code.claude.com/docs/zh-CN/computer-use.md)在 macOS 上（Pro、Max 和 Ultra）、第三方提供商 |
| [Desktop](https://code.claude.com/docs/zh-CN/desktop.md) | 视觉审查、并行会话、托管设置 | Diff 查看器、应用预览、Pro、Max 和 Ultra 上的[计算机使用](https://code.claude.com/docs/zh-CN/desktop.md#let-claude-use-your-computer)和 [Dispatch](https://code.claude.com/docs/zh-CN/desktop.md#sessions-from-dispatch) |
| [VS Code](https://code.claude.com/docs/zh-CN/vs-code.md) | 在 VS Code 内工作而无需切换到终端 | 内联 diff、集成终端、文件上下文 |
| [JetBrains](https://code.claude.com/docs/zh-CN/jetbrains.md) | 在 IntelliJ、PyCharm、WebStorm 或其他 JetBrains IDE 内工作 | Diff 查看器、选择共享、终端会话 |
| [Web](https://code.claude.com/docs/zh-CN/claude-code-on-the-web.md) | 不需要太多操作的长时间运行任务，或应该在您离线时继续的工作 | 云、Anthropic 托管（默认）；断开连接后继续运行 |
| [Mobile](https://code.claude.com/docs/zh-CN/mobile.md) | 在远离计算机时启动和监控任务 | 来自 iOS 和 Android 版 Claude 应用的云端会话、用于本地会话的 [Remote Control](https://code.claude.com/docs/zh-CN/remote-control.md)、Pro、Max 和 Ultra 上的 [Dispatch](https://code.claude.com/docs/zh-CN/desktop.md#sessions-from-dispatch) 到 Desktop |

CLI 是终端原生工作的最完整界面：脚本编写和 Agent SDK 仅限 CLI。第三方提供商也可在 [VS Code](https://code.claude.com/docs/zh-CN/vs-code.md#use-third-party-providers) 和 [JetBrains](https://code.claude.com/docs/zh-CN/feature-availability.md#features-available-on-every-provider) 中使用，后者在您的 IDE 终端中运行 CLI。企业 [Desktop](https://code.claude.com/docs/zh-CN/desktop.md) 部署支持 Google Cloud 的 Agent Platform，Desktop 支持[网关提供商](https://code.claude.com/docs/zh-CN/llm-gateway-connect.md#desktop-app)；对于 Amazon Bedrock 或 Microsoft Foundry，请使用 CLI 或 IDE 扩展，或 [Claude Desktop on 3P](https://claude.com/docs/third-party/claude-desktop/overview)，它在这些提供商上运行 Code 选项卡。Desktop 和 IDE 扩展为了视觉审查和更紧密的编辑器集成而放弃了一些仅限 CLI 的功能。Web 在云中运行，因此任务在您断开连接后继续进行。Mobile 是这些相同云会话的瘦客户端，或通过 Remote Control 进入本地会话，并可以使用 Dispatch 向 Desktop 发送任务。

您可以在同一项目上混合使用多个界面。配置、项目内存和 MCP 服务器在本地界面之间共享。

## 连接您的工具

集成让 Claude 与代码库外的服务协作。

| 集成 | 功能 | 用途 |
| :- | :- | :- |
| [Chrome](https://code.claude.com/docs/zh-CN/chrome.md) | 使用您登录的会话控制浏览器 | 测试 Web 应用、填充表单、自动化没有 API 的网站 |
| [GitHub Actions](https://code.claude.com/docs/zh-CN/github-actions.md) | 在 CI 管道中运行 Claude | 自动化 PR 审查、问题分类、计划维护 |
| [GitLab CI/CD](https://code.claude.com/docs/zh-CN/gitlab-ci-cd.md) | 与 GitHub Actions 相同，但用于 GitLab | GitLab 上的 CI 驱动自动化 |
| [Code Review](https://code.claude.com/docs/zh-CN/code-review.md) | 自动审查每个 PR | 在人工审查前捕获错误 |
| [Slack](https://code.claude.com/docs/zh-CN/slack.md) | 以您自己的账户响应频道中的 `@Claude` 提及 | 在 Pro、Max 和 Ultra 计划上，将团队聊天中的错误报告转换为 Pull Request |
| [Claude Tag](https://claude.com/docs/claude-tag) | 以您组织的共享身份运行 `@Claude`，具有管理员配置的访问权限 | Team 和 Enterprise 计划上的共享团队访问，而不是按用户的 Slack 会话 |

对于此处未列出的集成，[MCP 服务器](https://code.claude.com/docs/zh-CN/mcp.md)和[连接器](https://code.claude.com/docs/zh-CN/desktop.md#connect-external-tools)让您连接几乎任何东西：Linear、Notion、Google Drive 或您自己的内部 API。

## 远离终端时工作

Claude Code 提供了多种方式在您不在终端时进行工作。它们在触发工作的方式、Claude 运行的位置以及所需的设置量方面有所不同。

| | 触发方式 | Claude 运行位置 | 设置 | 最适合 |
| :- | :- | :- | :- | :- |
| [Dispatch](https://code.claude.com/docs/zh-CN/desktop.md#sessions-from-dispatch) | 从 Claude 移动应用发送任务消息 | 您的机器（Desktop） | [将移动应用与 Desktop 配对](https://support.claude.com/en/articles/13947068) | 在您离开时委派工作，最少设置 |
| [Remote Control](https://code.claude.com/docs/zh-CN/remote-control.md) | 从 [claude.ai/code](https://claude.ai/code) 或 Claude 移动应用驱动正在运行的会话 | 您的机器（CLI、Desktop 或 VS Code） | 运行 [`claude remote-control` 或 `/remote-control`](https://code.claude.com/docs/zh-CN/remote-control#start-a-remote-control-session) | 从另一台设备控制进行中的工作 |
| [Channels](https://code.claude.com/docs/zh-CN/channels.md) | 从聊天应用（如 Telegram 或 Discord）或您自己的服务器推送事件 | 您的机器（CLI） | [安装频道插件](https://code.claude.com/docs/zh-CN/channels.md#quickstart) 或 [构建您自己的](https://code.claude.com/docs/zh-CN/channels-reference.md) | 对外部事件（如 CI 失败或聊天消息）做出反应 |
| [Slack](https://code.claude.com/docs/zh-CN/slack.md) | 在团队频道中提及 `@Claude` | Anthropic 云 | [安装 Slack 应用](https://code.claude.com/docs/zh-CN/slack.md#setting-up-claude-code-in-slack)，启用 [Claude Code on the web](https://code.claude.com/docs/zh-CN/claude-code-on-the-web.md)，在 Pro、Max 和 Ultra 计划上 | 从团队聊天进行 PR 和审查 |
| [Self-hosted environments](https://code.claude.com/docs/zh-CN/self-hosted-environments) | 启动 [云会话](https://code.claude.com/docs/zh-CN/claude-code-on-the-web.md)并选择您组织的环境 | 您组织的基础设施 | [部署运行器](https://code.claude.com/docs/zh-CN/self-hosted-environments-quickstart.md)，在 Team 和 Enterprise 计划上 | 必须在您的网络内运行的云会话 |
| [Scheduled tasks](https://code.claude.com/docs/zh-CN/scheduled-tasks.md) | 设置计划 | [CLI](https://code.claude.com/docs/zh-CN/scheduled-tasks.md)、[Desktop](https://code.claude.com/docs/zh-CN/desktop-scheduled-tasks.md) 或 [云](https://code.claude.com/docs/zh-CN/routines.md) | 选择频率 | 定期自动化，如每日审查 |

如果您不确定从哪里开始，[安装 CLI](https://code.claude.com/docs/zh-CN/quickstart.md) 并在项目目录中运行它。如果您不想使用终端，[Desktop](https://code.claude.com/docs/zh-CN/desktop-quickstart.md) 为您提供相同的引擎和图形界面。

## 相关资源

### 平台

* [CLI 快速入门](https://code.claude.com/docs/zh-CN/quickstart.md)：在终端中安装并运行您的第一个命令
* [Desktop](https://code.claude.com/docs/zh-CN/desktop.md)：视觉 diff 审查、并行会话、计算机使用和 Dispatch
* [VS Code](https://code.claude.com/docs/zh-CN/vs-code.md)：编辑器内的 Claude Code 扩展
* [JetBrains](https://code.claude.com/docs/zh-CN/jetbrains.md)：IntelliJ、PyCharm 和其他 JetBrains IDE 的扩展
* [Web](https://code.claude.com/docs/zh-CN/claude-code-on-the-web.md)：云会话，可从浏览器访问 claude.ai/code，断开连接时继续运行
* [Projects](https://code.claude.com/docs/zh-CN/claude-projects.md)：一个对话，Claude 在其中协调许多云会话以完成一项工作并报告结果
* [Mobile](https://code.claude.com/docs/zh-CN/mobile.md)：用于在远离计算机时启动和监控任务的 [iOS](https://apps.apple.com/us/app/claude-by-anthropic/id6473753684) 和 [Android](https://play.google.com/store/apps/details?id=com.anthropic.claude) 版 Claude 应用

### 集成

* [Chrome](https://code.claude.com/docs/zh-CN/chrome.md)：使用您登录的会话自动化浏览器任务
* [计算机使用](https://code.claude.com/docs/zh-CN/computer-use.md)：让 Claude 在 macOS 上打开应用和控制您的屏幕
* [GitHub Actions](https://code.claude.com/docs/zh-CN/github-actions.md)：在 CI 管道中运行 Claude
* [GitLab CI/CD](https://code.claude.com/docs/zh-CN/gitlab-ci-cd.md)：GitLab 的相同功能
* [Code Review](https://code.claude.com/docs/zh-CN/code-review.md)：每个拉取请求上的自动审查
* [Slack](https://code.claude.com/docs/zh-CN/slack.md)：从团队聊天发送任务，获取 PR 返回，适用于 Pro、Max 和 Ultra 计划
* [Claude Tag](https://claude.com/docs/claude-tag)：在 Team 和 Enterprise 计划上以您组织的共享身份运行 `@Claude`

### 远程访问

* [Dispatch](https://code.claude.com/docs/zh-CN/desktop.md#sessions-from-dispatch)：从您的手机发送任务，它可以生成 Desktop 会话
* [Remote Control](https://code.claude.com/docs/zh-CN/remote-control.md)：从您的手机或浏览器驱动运行中的会话
* [Channels](https://code.claude.com/docs/zh-CN/channels.md)：将来自聊天应用或您自己的服务器的事件推送到会话中
* [调度选项](https://code.claude.com/docs/zh-CN/scheduled-tasks.md#compare-scheduling-options)：按定期计划运行提示词
