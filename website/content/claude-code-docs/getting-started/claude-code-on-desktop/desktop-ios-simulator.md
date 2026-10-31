> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# 在模拟器中测试 iOS 应用

> Claude Code Desktop 在 Claude 构建、运行或检查应用时，会在 iOS Simulator 窗格中打开你的应用，每个会话都有一个单独的模拟器。

> [!NOTE]
>
> iOS Simulator 窗格在 macOS 上的 Claude Code Desktop 中处于公开测试版。它在 Pro、Max、Ultra、Team 和 Enterprise 计划中可用，但在启用了 HIPAA 配置的 Enterprise 组织中不可用。

iOS Simulator 窗格在 Claude Code Desktop 中的对话旁边显示你的应用在 Apple 的 iOS Simulator 中运行。当 Claude 在模拟器中构建、安装、启动或检查你的应用时，该窗格会自动打开并实时流式传输设备屏幕。使用它来观看 Claude 运行和测试你的应用，或者在 Claude 继续工作时自己点击浏览应用。

模拟器窗格直接驱动模拟器，因此它不需要[计算机使用](https://code.claude.com/docs/zh-CN/desktop.md#let-claude-use-your-computer)，也不会接管你的屏幕或隐藏其他窗口。从 CLI 中，Claude 通过[计算机使用](https://code.claude.com/docs/zh-CN/computer-use.md#test-a-simulator-flow)访问 iOS Simulator，它以与你使用鼠标相同的方式控制屏幕上的模拟器。

## 要求

模拟器窗格使用 Apple 的模拟器工具，桌面应用不包含这些工具。在开始会话之前，请确保你有：

* Claude Desktop v1.24012.0 或更高版本
* 一台 Mac，因为 Apple 的 iOS Simulator 仅在 macOS 上运行
* [Xcode](https://developer.apple.com/xcode/)，其中安装了 iOS 平台，它提供模拟器设备。如果 Xcode 还没有列出任何模拟器，请参阅[模拟器窗格显示未找到模拟器](#the-simulator-pane-says-no-simulators-were-found)
  * 推荐使用 Xcode 26.x 或 Xcode 27，该窗格可与两者一起使用。如果你安装了多个 Xcode，请参阅[选择模拟器窗格使用的 Xcode](#choose-which-xcode-the-simulator-pane-uses)

> [!NOTE]
>
> 在本页上，"设备"指的是模拟的 iPhone 或 iPad，是你在 Xcode 中的**Window → Devices and Simulators** 下管理的相同模拟器设备之一，而不是物理硬件。

模拟器窗格仅在本地会话中可用。在[云](https://code.claude.com/docs/zh-CN/desktop.md#run-long-running-tasks-in-the-cloud)和 [SSH](https://code.claude.com/docs/zh-CN/desktop.md#ssh-sessions) 会话中，Claude 在无法访问 Mac 上模拟器的机器上运行。

### 选择模拟器窗格使用的 Xcode

如果你安装了多个 Xcode，例如 Xcode 26.x 与 Xcode 27 并行安装，该窗格使用 `xcode-select` 指向的任何一个。要使用不同的安装，请按其路径选择它。

例如，如果你想要的 Xcode 安装为 `/Applications/Xcode-26.4.app`，请在终端中运行以下命令：

```bash theme={null}
sudo xcode-select -s /Applications/Xcode-26.4.app
```

然后退出并重新打开 Claude Desktop，以便该窗格使用新选择的 Xcode。

要检查选择了哪个安装，请运行 `xcode-select -p`。它会打印所选安装的 Developer 文件夹，例如 `/Applications/Xcode-26.4.app/Contents/Developer`。

## 在模拟器中运行你的应用

你不需要命令或设置来打开模拟器窗格。当 Claude 在模拟器中运行你的应用时，它会打开该窗格。

**1. 打开你的 iOS 项目**

在 Claude Code Desktop 中，打开**Code** 选项卡并使用你的应用项目作为[项目文件夹](https://code.claude.com/docs/zh-CN/desktop.md#start-a-session)启动会话。任何为 iOS Simulator 构建应用的项目都可以使用。

**2. 要求 Claude 运行或测试应用**

围绕运行或验证应用来表述任务。例如：

```text theme={null}
构建应用并在模拟器中运行它以检查入门流程。
```

**3. 在模拟器窗格中观看应用**

当应用在模拟器中启动时，iOS Simulator 窗格会在对话旁边打开。Claude 第一次使用设备时，桌面应用会要求你允许它；请参阅[授予 Claude 对设备的访问权限](#grant-claude-access-to-a-device)。Claude 安装应用、点击浏览它，并读取屏幕以验证自己的更改，同时你观看。

模拟器窗格在 Claude 在会话中的任何时刻在模拟器中启动应用时打开。当你的请求是关于查看应用时，例如"新屏幕看起来对吗？"，Claude 在开始工作之前启动模拟器。在 Claude 修复错误或更改屏幕后，要求它验证更改：重新启动应用会重新打开窗格（如果它未打开）。

模拟器窗格显示应用实际启动的任何设备。要在特定设备上测试，在你的请求中命名它，例如"在 iPhone SE 模拟器上运行它"，Claude 在构建和启动时会针对该设备。

Claude 启动的设备也会出现在 Apple 的 Simulator 应用中，或在 Xcode 27 的 Device Hub 中。Claude 可以在你已经启动的设备上安装应用。

您也可以自己打开模拟器窗格。一旦会话有模拟器连接或 Claude Code 检测到 iOS 应用相关工作，会话的标题栏中就会出现 **iOS Simulator** 按钮。如果窗格还没有显示设备，请单击**Attach simulator**，或从它旁边的设备菜单中选择特定设备；选择关闭的设备会启动它。如果 Xcode 或其模拟器缺失，窗格会显示设置步骤，并在您完成每个步骤时检查它们。

## 自己控制模拟器

模拟器窗格是交互式的，不仅仅是查看器。在 Claude 工作时或任务之间，你可以：

* 通过在设备屏幕上单击和拖动来点击和滑动
* 使用与 Apple 的 Simulator 应用相同的快捷键按下硬件按钮：**Cmd+Shift+H** 表示主屏幕，**Cmd+L** 表示锁定，**Cmd+Up Arrow** 和 **Cmd+Down Arrow** 表示音量
* 使用旋转按钮或 **Cmd+Right Arrow** 将设备顺时针旋转四分之一圈
* 从设备菜单中切换窗格显示的设备，该菜单列出每个模拟器的操作系统版本以及它是否已启动
* 使用 **Cmd+S** 保存屏幕截图或使用 **Cmd+R** 保存屏幕录制，使用窗格的捕获按钮或快捷键；文件保存到你的桌面
* 通过单击**Detach simulator** 停止流式传输设备而不关闭它，这会将窗格返回到其**Attach simulator** 状态

如果窗格显示 **Display** 菜单，可以使用它来调整来自模拟器的视频流。如果窗格对您的 Mac 造成压力，请降低**Frame rate** 或**Resolution**。这两项设置改变窗格显示设备的方式，而不是应用运行的方式。

你和 Claude 驱动同一设备，因此你的点击会改变 Claude 看到的应用状态。要让 Claude 检查特定屏幕，通过点击导航到它，然后提出要求。当 Claude 驱动设备时，窗格在屏幕上方显示**Claude is using this device** 徽章；在徽章清除之前暂停点击，以便结果反映应用而不是你的输入。

## 会话如何管理设备

每个设备属于启动它的会话，因此[并行会话](https://code.claude.com/docs/zh-CN/desktop.md#work-in-parallel-with-sessions)不共享设备：你在一个会话的窗格中看到的内容反映的是该会话的工作，而不是另一个会话的工作。在侧边栏中切换会话会同时切换模拟器视图和对话，切换回来会恢复同一设备停止的地方。如果 Claude 使用多个设备，每个设备都会打开自己的窗格，每个会话最多 4 个。

Claude Code Desktop 会在模拟器不再使用时关闭它启动的模拟器：当你退出应用时、当你归档会话时，或在你从窗格中分离设备后 10 分钟。你在 Claude Code Desktop 外启动的设备，例如在 Apple 的 Simulator 应用或 Device Hub 中启动的设备，永远不会自动关闭。要立即关闭附加的设备，请使用窗格中的关闭按钮。

## 授予 Claude 对设备的访问权限

Claude 在控制设备之前要求你的同意，而构建应用或在其上打开 URL 遵循你的会话的权限模式。你或你的组织也可以完全关闭 Claude 的访问。

### 第一次允许设备

Claude 第一次使用模拟器时，桌面应用会要求你允许它。同意涵盖控制该设备和对其进行屏幕截图，你每个设备给予一次而不是每个会话一次。Claude 对设备的屏幕截图被发送到 Anthropic 并根据你的正常对话保留设置保留，因此不要在 Claude 使用的设备上登录真实账户。

在你允许设备后，Claude 对其的操作，例如点击、输入、启动应用和拍摄屏幕截图，无需进一步提示即可运行。它们具有与你在窗格中单击相同的信任，并且它们仅触及模拟设备，因此窗格不需要计算机使用所需的 macOS 辅助功能和屏幕录制权限。

如果你拒绝，设备仍会启动，窗格仍可用于你自己的点击；只有 Claude 的访问保持关闭。要稍后改变主意，请在窗格中单击**Let Claude use it**。

### 遵循你的权限模式的操作

两个操作遵循你的会话的[权限模式](https://code.claude.com/docs/zh-CN/permissions.md#permission-modes)而不是一次性同意：

* 在设备上打开 URL，例如测试深层链接或在设备的 Safari 中加载页面，因为 URL 可以将数据从设备中携带出去。
* 构建应用，因为 `xcodebuild` 在你的 Mac 上运行你的项目的构建脚本。检查已在进行的构建不会提示。

### 关闭模拟器访问

你可以在桌面应用的设置中关闭 Claude 的模拟器访问。组织有两种方式为所有人关闭它：

* `disableMobileSimulatorTools` [托管设置](https://code.claude.com/docs/zh-CN/desktop.md#managed-settings)阻止 Claude 的模拟器工具。模拟器窗格仍可用于你自己的点击，该设置无法从应用内覆盖。
* `requireCoworkFullVmSandbox` 策略密钥，它在隔离的虚拟机内而不是在你的 Mac 上运行 Claude 的工具，禁用模拟器窗格和 Claude 的模拟器工具，因此当它被设置时窗格无法连接设备。

Claude 会告诉你何时应用任一情况。

## 限制

Claude 仅驱动模拟设备，无法控制物理 iPhone 或 iPad。要在其上测试，从 Xcode 自己在其上运行应用，然后描述你看到的内容或将屏幕截图附加到对话中供 Claude 使用。

## 故障排除

### 当 Claude 运行应用时模拟器窗格不打开

Claude 可能没有识别出你想要运行或测试应用，或者模拟器工具可能缺失。检查以下内容：

* 明确说明目标，例如"在 iOS Simulator 中运行应用并点击浏览注册流程"。
* 确认 Xcode 和 iOS 模拟器已安装，并且你的 Xcode 版本符合[要求](#requirements)。
* 如果你的组织管理 Claude Code，[模拟器工具可能被策略禁用](#turn-off-simulator-access)。
* 如果你在启用了 HIPAA 配置的 Enterprise 组织中，模拟器窗格对你不可用。
* 模拟器窗格需要 Claude Desktop v1.24012.0 或更高版本。打开 **Claude > Check for Updates…**，然后重启应用。

### 模拟器窗格显示未找到模拟器

Xcode 已安装但没有 iOS 模拟器可列出。模拟器窗格显示要遵循的设置步骤，并在每个步骤完成时检查它们。要手动安装缺失的部分，从 Xcode 的设置中下载 iOS 模拟器运行时，或运行 `xcodebuild -downloadPlatform iOS`。

## 另请参阅

* [Desktop 中的计算机使用](https://code.claude.com/docs/zh-CN/desktop.md#let-claude-use-your-computer)：没有专用窗格的应用的屏幕控制
* [CLI 中的计算机使用](https://code.claude.com/docs/zh-CN/computer-use.md)：CLI 如何访问 iOS Simulator
* [在独立的 worktree 中运行会话](https://code.claude.com/docs/zh-CN/desktop.md#run-a-session-in-its-own-worktree)：会话如何隔离更改
* [开始使用 Claude Code Desktop](https://code.claude.com/docs/zh-CN/desktop-quickstart.md)
