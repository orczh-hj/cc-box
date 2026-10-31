> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Claude Code Desktop 在 WSL 中

> 在 Windows 上的 WSL 2 发行版内运行 Code 会话

在 Windows 上，Code 选项卡可以在 WSL 2 发行版内运行会话，而不是在 Windows 本身上运行。会话的 Claude Code 进程、其工具和 git 都在发行版内执行，使用其 Linux 工具链和本机 Linux 路径，与您的项目所针对的环境相同。

当您的存储库位于发行版的文件系统内时，请使用 WSL 会话。从 Windows 处理这些文件会通过网络文件系统进行，这很慢并且会破坏文件监视；在发行版内运行会话可以避免两者。

## 要求

* Windows 10 或 11，带有 [WSL 2](https://learn.microsoft.com/windows/wsl/install)。不支持 WSL 1。
* 至少安装一个发行版（例如 Ubuntu）。
* 在发行版内安装了 `git`。

## 启动 WSL 会话

**1. 选择发行版**

在 Code 选项卡中启动新会话，然后打开输入框下方（在[早期布局](https://code.claude.com/docs/zh-CN/desktop.md#check-which-layout-you-have)中则位于输入框上方）的[环境选择器](https://code.claude.com/docs/zh-CN/desktop.md#start-a-session)。您安装的 WSL 2 发行版会出现在 **WSL** 部分中。选择一个。

**2. 选择文件夹**

会话在发行版的主目录中启动。使用文件夹选择器选择项目文件夹。浏览在发行版内进行，使用 Linux 路径，如 `/home/you/project`。

**3. 信任文件夹**

文件夹中的第一个会话会显示工作区信任对话框。信任按发行版和文件夹授予。在一个发行版中信任的文件夹，在另一个发行版中或在 Windows 上的相同路径下不会被信任。

发行版中的第一个会话需要稍长时间，因为 Claude 需要在其中进行设置。您也可以从普通文件夹选择器打开 `\\wsl.localhost\...` 文件夹，它会在该发行版内重新打开。

## WSL 会话中的工作内容

并行会话、侧边聊天、可视化差异审查、分支和拉取请求状态以及 worktrees 都可以工作，由发行版内的 git 和工具链支持。"在编辑器中打开"会打开通过 [Remote - WSL](https://code.visualstudio.com/docs/remote/wsl) 连接到发行版的 VS Code。

如果某项 Desktop 功能在 WSL 会话中不可用或受限，请检查它是否属于以下情况之一：

* **终端和连接器**：您的会话可能缺少[集成终端](https://code.claude.com/docs/zh-CN/desktop.md#run-commands-in-the-terminal)、[连接器](https://code.claude.com/docs/zh-CN/desktop.md#connect-external-tools)，或两者都缺少。请参阅[如果终端或连接器不可用](#check-for-the-terminal-and-connectors)。
* **[插件](https://code.claude.com/docs/zh-CN/desktop.md#install-plugins)**：在 WSL 会话中不可用
* **文件编辑**：您可以在[文件窗格](https://code.claude.com/docs/zh-CN/desktop.md#open-and-edit-files)中查看文件，但无法在其中编辑，因此请让 Claude 进行更改
* **附件**：如果 **+** 菜单显示的是 **Add image** 而不是 **Add files or photos**，则您只能附加图片。请参阅[向提示词添加文件和上下文](https://code.claude.com/docs/zh-CN/desktop.md#add-files-and-context-to-prompts)。

### 如果终端或连接器不可用

您无法自行为 WSL 会话启用集成终端或连接器：

* **Claude 账户**：这两项功能正在分别推出。请选择 **Help > Check for Updates…**，如果更新后某项功能仍然缺失，则无需进行任何操作。
* **[第三方提供商](https://code.claude.com/docs/zh-CN/third-party-integrations.md)**：WSL 会话不提供终端或连接器，更新 Desktop 也不会改变这一点

## 受管设备

在由组织管理的设备上，WSL 会话可能不可用。如果会话启动失败并显示设备受管的消息，这由您的管理员控制。管理员：请参阅部署指南中的[设置如何到达设备](https://code.claude.com/docs/zh-CN/admin-setup.md#decide-how-settings-reach-devices)。
