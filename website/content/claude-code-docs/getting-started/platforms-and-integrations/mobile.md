> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Claude Code 移动版

> 从您的手机使用 Claude 应用程序启动、监控和指导 Claude Code 任务，支持 iOS 和 Android。

Claude [iOS](https://apps.apple.com/us/app/claude-by-anthropic/id6473753684) 和 [Android](https://play.google.com/store/apps/details?id=com.anthropic.claude) 应用是 Claude Code 会话的客户端，而不是代码运行的地方。从您的手机，您可以访问云中的[云会话](#start-and-monitor-cloud-sessions)和[项目](https://code.claude.com/docs/zh-CN/claude-projects.md)，通过[远程控制](#continue-a-local-session-with-remote-control)运行在您自己机器上的会话，或通过 [Dispatch](https://code.claude.com/docs/zh-CN/desktop.md#sessions-from-dispatch) 访问桌面应用。

> [!NOTE]
>
> Claude Code 没有单独的移动应用：云会话和远程控制都位于 Claude 应用中的 **Code** 选项卡中，Dispatch 是您在应用中向其发送消息的任务。

## 获取应用程序

**1. 下载 Claude 应用程序**

为 [iOS](https://apps.apple.com/us/app/claude-by-anthropic/id6473753684) 或 [Android](https://play.google.com/store/apps/details?id=com.anthropic.claude) 安装 Claude 应用程序。在 iPad 上，安装相同的 iOS 应用程序。

> [!TIP]
>
> 在 Claude Code 会话中运行 `/mobile` 以显示 [claude.ai/mobile](https://claude.ai/mobile) 的二维码，该二维码会打开适合您手机的应用商店。`/ios` 和 `/android` 执行相同的操作。

**2. 登录**

使用您用于 Claude Code 的相同 claude.ai 账户和组织登录。云会话和远程控制需要 claude.ai 账户，因此无法通过 Anthropic Console API 密钥或来自 Amazon Bedrock 等第三方提供商的方式访问。

**3. 打开 Code 选项卡**

在应用程序的导航中点击 **Code** 以访问您的会话，或在您的手机上打开 [claude.ai/code/new](https://claude.ai/code/new) 以在应用程序中启动新的 Code 会话。如果您看不到 Code 选项卡，您的计划或组织可能不包括这些功能；请参阅[按订阅计划的可用性](https://code.claude.com/docs/zh-CN/feature-availability.md#availability-by-subscription-plan)。

## 从您的手机工作

从应用程序中，您可以启动云会话、打开项目、驱动在您的计算机上运行的 Claude Code 会话，或向 Dispatch 消息传递任务。应用程序对所有这些都是相同的；它们在工作发生的位置上有所不同。

| 功能 | 您连接到的内容 | 何时使用 |
| :- | :- | :- |
| [云会话](https://code.claude.com/docs/zh-CN/claude-code-on-the-web.md) | 云基础设施上的会话，默认由 Anthropic 托管 | 您的存储库在 GitHub 上，任务应在您放下手机后继续运行。请参阅[云快速入门](https://code.claude.com/docs/zh-CN/web-quickstart.md)进行设置。 |
| [项目](https://code.claude.com/docs/zh-CN/claude-projects.md) | Claude 协调平行工作线程并报告回复的对话 | 您有一系列相关工作而不是一个任务，并且想要查看哪些线程已完成或需要您。 |
| [远程控制](https://code.claude.com/docs/zh-CN/remote-control.md) | 在您的计算机上运行的 Claude Code 会话 | 工作需要您的本地文件系统、工具或 MCP 服务器。 |
| [Dispatch](https://code.claude.com/docs/zh-CN/desktop.md#sessions-from-dispatch) | 您计算机上的桌面应用程序 | 您想消息传递一个任务，让 Dispatch 决定如何运行它。需要 Pro、Max 或 Ultra 计划。 |

如果您的计算机将关闭，请使用云会话或项目，它们在云中运行，并在您的笔记本电脑关闭后继续运行。远程控制和 Dispatch 驱动您自己的机器，因此它需要保持打开状态并运行 Claude Code 或桌面应用程序。如果您的机器在远程控制会话期间进入睡眠状态，Claude Code 会在机器重新上线时重新连接。

有关更完整的比较，请参阅[当您远离终端时工作](https://code.claude.com/docs/zh-CN/platforms.md#work-when-you-are-away-from-your-terminal)。

云会话和远程控制从 **Code** 选项卡运行。对于 Dispatch（您在应用程序中作为任务消息传递），请参阅[来自 Dispatch 的会话](https://code.claude.com/docs/zh-CN/desktop.md#sessions-from-dispatch)。

### 启动和监控云会话

云会话在云基础设施上运行任务，默认由 Anthropic 托管，因此会话在您放下手机后继续进行。从 Code 选项卡中，选择一个存储库和分支，描述任务，然后提交。会话在设备之间持久化：您在笔记本电脑上启动的任务已准备好从您的手机进行审查，您从手机启动的任务在您回到办公桌时正在等待。

在应用程序中打开会话以检查进度、回答 Claude 的问题或将其引导到新的方向。您也可以告诉 Claude [监视拉取请求](https://code.claude.com/docs/zh-CN/claude-code-on-the-web.md#auto-fix-pull-requests)并在 CI 失败或审查评论到达时修复它们。要连接 GitHub 并设置您的环境，请按照[云快速入门](https://code.claude.com/docs/zh-CN/web-quickstart.md)进行操作，并查看[在云中使用 Claude Code](https://code.claude.com/docs/zh-CN/claude-code-on-the-web.md) 了解云会话可以执行的所有操作。

### 使用远程控制继续本地会话

远程控制将 Claude 应用程序连接到在您的机器上运行的 Claude Code 会话，因此代码执行和文件系统访问保持本地，而您从手机驱动会话。在您的计算机上使用 `claude remote-control` 启动会话，或在已打开的会话中运行 `/remote-control`。然后扫描终端可以显示的会话二维码，或打开 Claude 应用程序，点击 **Code**，然后从列表中选择会话。有关每个选项，请参阅[从另一个设备连接](https://code.claude.com/docs/zh-CN/remote-control.md#connect-from-another-device)。

当您在 Claude 应用程序中添加附件时，它也会到达本地会话：

* **照片**：Claude 直接将附加的照片视为您消息的一部分。Claude Code 还会将每张照片保存在 `~/.claude/uploads/` 下，并告诉 Claude 保存的文件路径，以便 Claude 可以将图像复制到它创建的文件中。
* **其他文件**：Claude Code 将它们下载到您的机器，并将它们作为 `@` 文件引用传递给 Claude。

有关要求、调用模式和故障排除，请参阅[远程控制概述](https://code.claude.com/docs/zh-CN/remote-control.md)。

### 获取推送通知

当远程控制处于活动状态时，Claude 可以向您的手机发送推送通知，通常在长时间运行的任务完成或需要您做出决定时。您也可以在提示中请求一个，例如 `notify me when the tests finish`。有关两个 `/config` 切换和交付故障排除，请参阅[移动推送通知](https://code.claude.com/docs/zh-CN/remote-control.md#mobile-push-notifications)。

Dispatch 在其生成的 Code 会话完成或需要您的批准时发送自己的通知，如[来自 Dispatch 的会话](https://code.claude.com/docs/zh-CN/desktop.md#sessions-from-dispatch)中所述。

## 限制

移动客户端涵盖了会话需要的大部分内容，但有一些限制：

* **仅限本地命令**：仅在终端界面中运行的命令，例如 `/plugin` 和 `/resume`，无法从应用程序中工作。[远程控制限制](https://code.claude.com/docs/zh-CN/remote-control.md#limitations)列出了从移动设备工作的命令以及它们的行为如何不同。
* **权限模式**：云端会话提供接受编辑、Plan 和 Auto，Remote Control 会话提供 Manual、接受编辑、Plan 和 Auto。在任何一种情况下，您都无法从应用程序中选择 Bypass permissions。有关 Auto 何时可用，请参阅[切换权限模式](https://code.claude.com/docs/zh-CN/permission-modes.md#switch-permission-modes)。
* **Dispatch 计划**：Dispatch 需要 Pro、Max 或 Ultra 计划，在 Team 或 Enterprise 上不可用。

## 相关资源

* [平台和集成](https://code.claude.com/docs/zh-CN/platforms.md)：比较 Claude Code 运行的每个表面
* [Claude Code 网页版](https://code.claude.com/docs/zh-CN/claude-code-on-the-web.md)：云会话如何运行以及如何在您的终端之间移动工作
* [配置云环境](https://code.claude.com/docs/zh-CN/cloud-environments.md)：云会话的网络访问级别、环境变量和设置脚本
* [远程控制](https://code.claude.com/docs/zh-CN/remote-control.md)：从任何设备继续本地会话
* [来自 Dispatch 的会话](https://code.claude.com/docs/zh-CN/desktop.md#sessions-from-dispatch)：Dispatch 任务如何在桌面应用程序中成为 Code 会话
* [Channels](https://code.claude.com/docs/zh-CN/channels.md)：通过 Telegram、Discord 或 iMessage 从您的手机询问 Claude 一些事情，同时工作在您的机器上运行
* [Slack 中的 Claude Code](https://code.claude.com/docs/zh-CN/slack.md)：通过提及 `@Claude` 从您的 Slack 工作区委派编码任务
