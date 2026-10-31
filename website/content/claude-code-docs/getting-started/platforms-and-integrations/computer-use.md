> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# 让 Claude 从 CLI 使用您的计算机

> 在 Claude Code CLI 中启用 computer use，使 Claude 能够在 macOS 上打开应用、点击、输入和查看您的屏幕。测试原生应用、调试视觉问题，以及自动化仅限 GUI 的工具，无需离开您的终端。

> [!NOTE]
>
> Computer use 是 macOS 上的研究预览版，需要 Pro、Max 或 Ultra 计划。它在 Team 或 Enterprise 计划上不可用。它需要交互式会话，因此在使用 `-p` 标志的非交互模式下不可用。

Computer use 让 Claude 能够打开应用、控制您的屏幕，并以您的方式在您的机器上工作。从 CLI 中，Claude 可以编译 Swift 应用、启动它、点击每个按钮，并截图结果，所有这些都在编写代码的同一对话中进行。

本页面介绍 computer use 在 CLI 中的工作原理。对于 macOS 或 Windows 上的 Desktop 应用，请参阅 [Desktop 中的 computer use](https://code.claude.com/docs/zh-CN/desktop.md#let-claude-use-your-computer)。

## 您可以用 computer use 做什么

Computer use 处理需要 GUI 的任务：任何您通常必须离开终端并手动完成的事情。

* **构建和验证原生应用**：要求 Claude 构建 macOS 菜单栏应用。Claude 编写 Swift、编译它、启动它，并点击每个控件来验证它是否有效，然后您才打开它。
* **端到端 UI 测试**：将 Claude 指向本地 Electron 应用并说"测试入门流程"。Claude 打开应用、点击注册，并截图每一步。无需 Playwright 配置，无需测试工具。
* **调试视觉和布局问题**：告诉 Claude"模态框在小窗口上被裁剪"。Claude 调整窗口大小、重现错误、截图、修补 CSS，并验证修复。Claude 看到您看到的内容。
* **驱动仅限 GUI 的工具**：与设计工具、硬件控制面板、iOS 模拟器或没有 CLI 或 API 的专有应用交互。

## Computer use 何时适用

Claude 有多种方式与应用或服务交互。Computer use 是最广泛和最慢的，所以 Claude 首先尝试最精确的工具：

* 如果您有该服务的 [MCP server](https://code.claude.com/docs/zh-CN/mcp.md)，Claude 会使用它。
* 如果任务是 shell 命令，Claude 会使用 Bash。
* 如果任务是浏览器工作且您已设置 [Claude in Chrome](https://code.claude.com/docs/zh-CN/chrome.md)，Claude 会使用它。
* 如果以上都不适用，Claude 会使用 computer use。

屏幕控制保留用于其他工具无法到达的事物：原生应用、模拟器（如 iOS Simulator）和没有 API 的工具。

## 启用 computer use

Computer use 作为称为 `computer-use` 的内置 MCP server 可用。默认情况下它是关闭的，直到您启用它。

**1. 打开 MCP 菜单**

在交互式 Claude Code 会话中，运行：

```text theme={null}
/mcp
```

在服务器列表中找到 `computer-use`。它显示为已禁用。

**2. 启用服务器**

选择 `computer-use` 并选择**启用**。该设置按项目持久化，因此您只需为每个想要 computer use 的项目执行一次此操作。

**3. 授予 macOS 权限**

Claude 第一次尝试使用您的计算机时，您会看到一个提示来授予两个 macOS 权限：

* **Accessibility**：让 Claude 点击、输入和滚动
* **Screen Recording**：让 Claude 看到您屏幕上的内容

该提示包括打开相关系统设置窗格的链接。授予两者，然后在提示中选择**重试**。授予 Screen Recording 后，macOS 可能需要您重启 Claude Code。

设置后，要求 Claude 做需要 GUI 的事情：

```text theme={null}
构建应用目标、启动它，并点击每个选项卡以确保
没有任何内容崩溃。截图您找到的任何错误状态。
```

## 按会话批准应用

启用 `computer-use` 服务器并不会授予 Claude 访问您机器上所有应用的权限。在一个会话中，Claude 首次需要使用某个特定应用时，终端中会出现一个权限提示，显示：

* Claude 想要控制哪些应用
* 请求的任何额外权限，例如剪贴板访问
* Claude 工作期间将隐藏多少个其他应用

选择 **Allow for this session** 或 **Deny**。批准仅在当前会话中有效。当 Claude 同时请求多个应用时，您可以一次性批准它们。

影响范围较广的应用会在权限提示中显示额外警告，以便您了解批准它们意味着授予哪些权限：

| 警告 | 适用于 |
| :- | :- |
| 等同于 shell 访问 | Terminal、iTerm、VS Code、Warp 以及其他终端和 IDE |
| 可以读取或写入任何文件 | Finder |
| 可以更改系统设置 | System Settings |

这些应用并不会被阻止。该警告让您能够判断任务是否需要这种级别的访问权限。

批准 Finder 后，Claude 即可点击桌面、Dock 或 Finder 窗口。

Claude 的控制级别也因应用类别而异：浏览器和交易平台仅可查看，终端和 IDE 仅可点击，其他所有应用则获得完全控制。有关完整的层级划分，请参阅 [Desktop 中的应用权限](https://code.claude.com/docs/zh-CN/desktop.md#app-permissions)。

## Claude 如何在您的屏幕上工作

理解流程有助于您预期 Claude 将做什么以及如何干预。

### 一次一个会话

一次只有一个会话可以使用您的计算机。会话在其第一个计算机使用操作时获取锁，并在会话退出时释放它，而不是在任务完成时释放。第二个会话的计算机使用会失败并显示一条错误消息，说明哪个会话持有该锁。首先退出该会话。

### Claude 工作时应用被隐藏

当 Claude 开始控制您的屏幕时，其他可见应用被隐藏，以便 Claude 仅与批准的应用交互。您的终端窗口保持可见并被排除在屏幕截图之外，因此您可以观看会话，Claude 永远看不到自己的输出。

当 Claude 完成轮次时，隐藏的应用会自动恢复。

### 屏幕截图自动缩小

Claude Code 在将每个屏幕截图发送到模型之前会缩小它。您不需要降低显示分辨率或在 Retina 或其他高分辨率显示器上调整窗口大小。16 英寸 MacBook Pro 以原生 Retina 分辨率捕获 3456×2234，并缩小到大约 1372×887，保持宽高比。

没有设置可以更改目标大小。如果屏幕上的文本或控件在缩小后对 Claude 来说太小而无法读取，请增加应用中的大小，而不是更改显示分辨率。

### 随时停止

Claude 在每个轮次中首次使用您的计算机时，会出现 macOS 通知："Claude is using your computer · press Esc to stop"。在任何地方按 `Esc` 立即中止当前操作，或在终端中按 `Ctrl+C`。无论哪种方式，Claude 都会停止、取消隐藏您的应用，并将控制权返回给您。会话保持 [computer use 锁](#one-session-at-a-time)，直到它退出。

当 Claude 完成时，会出现第二个通知。

## 安全性和信任边界

> [!WARNING]
>
> 与 [sandboxed Bash tool](https://code.claude.com/docs/zh-CN/sandboxing.md) 不同，computer use 在您的实际桌面上运行，可以访问您批准的应用。Claude 检查每个操作并标记来自屏幕内容的潜在提示注入，但信任边界是不同的。有关最佳实践，请参阅 [computer use 安全指南](https://support.claude.com/en/articles/14128542)。

内置的护栏在不需要配置的情况下降低风险：

* **按应用批准**：Claude 只能控制您在当前会话中批准的应用。
* **哨兵警告**：授予 shell、文件系统或系统设置访问权限的应用在您批准之前被标记。
* **终端被排除在屏幕截图之外**：Claude 永远看不到您的终端窗口，因此您会话中的屏幕提示无法反馈到模型中。
* **全局转义**：`Esc` 键从任何地方中止 computer use，并且按键被消耗，因此提示注入无法使用它来关闭对话框。
* **锁文件**：一次只有一个会话可以控制您的机器。

## 示例工作流

这些示例展示了将 computer use 与编码任务结合的常见方式。

### 验证原生构建

对 macOS 或 iOS 应用进行更改后，让 Claude 在一次通过中编译和验证：

```text theme={null}
构建 MenuBarStats 目标、启动它、打开首选项窗口，
并验证间隔滑块更新标签。完成后截图首选项窗口。
```

Claude 运行 `xcodebuild`、启动应用、与 UI 交互，并报告它发现的内容。

### 重现布局错误

当视觉错误仅在某些窗口大小下出现时，让 Claude 找到它：

```text theme={null}
设置模态框在窄窗口上裁剪其页脚。调整应用窗口大小
直到您可以重现它、截图裁剪状态，然后检查模态框容器的 CSS。
```

Claude 调整窗口大小、捕获损坏的状态，并读取相关的样式表。

### 测试模拟器流程

无需编写 XCTest 即可驱动 iOS 模拟器：

```text theme={null}
打开 iOS 模拟器、启动应用、点击入门屏幕，
并告诉我是否有任何屏幕加载时间超过一秒。
```

Claude 以您使用鼠标的方式控制模拟器。此流程适用于 CLI；在 Desktop 应用中，相同的请求会打开 [iOS Simulator 窗格](https://code.claude.com/docs/zh-CN/desktop-ios-simulator.md)，而不是屏幕控制。

## 与 Desktop 应用的差异

CLI 和 Desktop 表面共享相同的 computer use 引擎，有一些差异：

| 功能 | Desktop | CLI |
| :- | :- | :- |
| 平台 | macOS 和 Windows | 仅 macOS |
| 启用 | [设置](https://code.claude.com/docs/zh-CN/desktop.md#enable-computer-use) 中的切换 | 在 `/mcp` 中启用 `computer-use` |
| 拒绝应用列表 | 在设置中可配置 | 尚不可用 |
| 自动取消隐藏切换 | 可选 | 始终开启 |
| Dispatch 集成 | Dispatch 生成的会话可以使用 computer use | 不适用 |

## 故障排除

### "Computer use is in use by another Claude session"

另一个 Claude Code 会话持有锁，它会一直保持到该会话退出。退出该会话。如果另一个会话崩溃，当 Claude 检测到该进程不再运行时，锁会自动释放。

### macOS 权限提示不断重新出现

授予 Screen Recording 后，macOS 有时需要重启请求进程。完全退出 Claude Code 并启动新会话。如果提示仍然存在，打开 **System Settings > Privacy & Security > Screen Recording** 并确认您的终端应用已列出并启用。

### `computer-use` 不出现在 `/mcp` 中

服务器仅在符合条件的设置上出现。检查：

* 您在 macOS 上。Computer use 在 CLI 中在 Linux 或 Windows 上不可用。在 Windows 上，改用 [Desktop 中的 computer use](https://code.claude.com/docs/zh-CN/desktop.md#let-claude-use-your-computer)。
* 您使用的是 Pro、Max 或 Ultra 套餐。运行 `/status` 来确认您的订阅。
* 您通过 claude.ai 进行身份验证。Computer use 不适用于第三方提供商，如 Amazon Bedrock、Google Cloud 的 Agent Platform 或 Microsoft Foundry。如果您仅通过第三方提供商访问 Claude，您需要单独的 claude.ai 账户来使用此功能。
* 您在交互式会话中。Computer use 在使用 `-p` 标志的非交互式模式下不可用。

## 另请参阅

* [Desktop 中的 Computer use](https://code.claude.com/docs/zh-CN/desktop.md#let-claude-use-your-computer)：具有图形设置页面的相同功能
* [Claude in Chrome](https://code.claude.com/docs/zh-CN/chrome.md)：用于基于网络的任务的浏览器自动化
* [MCP](https://code.claude.com/docs/zh-CN/mcp.md)：将 Claude 连接到结构化工具和 API
* [Sandboxing](https://code.claude.com/docs/zh-CN/sandboxing.md)：Claude 的 Bash 工具如何隔离文件系统和网络访问
* [Computer use 安全指南](https://support.claude.com/en/articles/14128542)：安全 computer use 的最佳实践
