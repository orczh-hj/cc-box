> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Desktop application

> 充分利用 Claude Code Desktop：使用 Git 隔离的并行会话、拖放窗格布局、集成终端和文件编辑器、侧边聊天、计算机使用、从手机 Dispatch 会话、可视化 diff 审查、应用预览、PR 监控、连接器和企业配置。

在 Claude Desktop 应用中，Claude Code 就是 **Code** 选项卡。该应用还包含聊天和 [Cowork](https://claude.com/product/cowork)，后者负责 Dispatch 和更长的代理工作。本页是 Code 选项卡的参考。

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

安装后，启动 Claude，登录，然后点击 **Code** 选项卡。有关首次会话的演练，请参阅[快速开始指南](https://code.claude.com/docs/zh-CN/desktop-quickstart.md)。

在 Code 选项卡中，每个对话都是一个**会话**：它有自己的聊天历史和项目文件夹，独立于任何其他会话。侧边栏列出你的会话，让你可以并行运行多个会话。在一个会话中，你可以：

* [使用 diff 视图审查和评论更改](#review-changes-with-diff-view)，然后[通过 CI 监控生成的 PR](#monitor-pull-request-status)
* [在浏览器窗格中预览你的运行应用](#preview-your-app)，同时 Claude 验证自己的更改，并[在其旁边打开外部网站](#browse-external-sites)
* 在 iOS Simulator 窗格中[观看 Claude 运行和测试你的 iOS 应用](https://code.claude.com/docs/zh-CN/desktop-ios-simulator.md)
* [整理窗格](#arrange-your-workspace)，将聊天、diff、浏览器、终端和文件编辑器并排放置
* 提出[侧边问题](#ask-a-side-question-without-derailing-the-session)，使用会话的上下文而不偏离主线
* 让 Claude [检查、消息或存档你的其他会话](#work-across-sessions)
* [连接外部工具](#connect-external-tools)，如 GitHub、Slack 和 Linear
* 让 Claude [打开应用和控制你的屏幕](#let-claude-use-your-computer)
* 在你的机器上、[云中](#run-long-running-tasks-in-the-cloud)或通过 [SSH](#ssh-sessions) 运行

有关[计划的定期工作](https://code.claude.com/docs/zh-CN/desktop-scheduled-tasks.md)、[快捷键](#keyboard-shortcuts)或[从手机发送任务](#sessions-from-dispatch)，请参阅链接的页面和部分。如果你已经使用基于终端的 CLI，请参阅 [CLI 比较](#coming-from-the-cli)了解哪些内容可以继续使用。

## 启动会话

在发送第一条消息之前，请在输入框下方的一行中选择环境、项目文件夹、权限模式和模型：

* **环境**：使用环境选择器（该行中的第一个控件）选择 Claude 运行的位置。请参阅[环境配置](#environment-configuration)。
  * 如果新会话的输入框下方没有显示环境，请按照[早期布局的步骤](#check-which-layout-you-have)操作。
* **项目文件夹**：点击环境选择器旁边的项目文件夹控件，选择 Claude 工作的文件夹。
  * **云端会话**：选择[仓库](#run-long-running-tasks-in-the-cloud)而不是文件夹
  * **[已保存的本地或 SSH 环境](#create-a-saved-environment)**：该环境已包含其文件夹，因此该行不显示项目文件夹控件。要更改文件夹，请[编辑该环境](#use-edit-or-remove-a-saved-environment)。
* **权限模式**：使用[模式选择器](#choose-a-permission-mode)选择 Claude 拥有多少自主权。您可以在会话期间更改权限模式。
* **模型**：从该行靠右端的模型选择器中选择一个[模型](https://code.claude.com/docs/zh-CN/model-config.md#available-models)。您可以在会话期间更改模型。

输入你的任务并按 **Enter** 启动。每个会话独立跟踪其自己的上下文和更改。

## 使用代码

为 Claude 提供正确的上下文，控制它自主执行的工作量，并审查它所做的更改。

### 使用提示框

输入您想让 Claude 做的事情，然后按 **Enter** 发送。Claude 会读取您的项目文件，进行更改，并根据您的[权限模式](#choose-a-permission-mode)运行命令。您可以随时重定向 Claude：点击停止按钮立即中断，或输入更正并按 **Enter** 发送，无需停止正在运行的操作。Claude 会在当前操作完成后立即读取更正，并在下一步之前进行调整。

点击提示框旁边的 **+** 按钮，可以访问文件附件、[skill](#use-skills)、[连接器](#connect-external-tools) 和[插件](#install-plugins)。

### 接受建议的提示词

Claude 回复后，Code 选项卡可能会在空的输入框中以灰色文本显示建议的下一条提示词。Claude Code 通过一个简短的后台请求，根据您的对话[生成每条建议](https://code.claude.com/docs/zh-CN/interactive-mode.md#prompt-suggestions)，该请求会计入您套餐的用量限制或您的 API 费用。

* **使用建议**：按 **Tab** 或 **右箭头** 将其放入输入框，根据需要进行编辑，然后按 **Enter** 发送。在接受建议之前按 **Enter** 不会发送它。
* **编写自己的提示词**：直接开始输入。建议仅在输入框为空且没有附加文件时显示。

在 **Settings > Code** 或 **Settings > Claude Code** 中关闭 **Prompt suggestions** 即可停止显示建议。每个会话会在下次启动或恢复时停止显示建议。

### 向提示添加文件和上下文

要向 Claude 提供文件，请在输入框中 @mention 它或附加它：

* **@mention 文件**：输入 `@` 后跟文件名，将文件添加到对话上下文。
  * **在云端会话中**：`@` 只有在您发送第一条消息后才能列出文件。如果 `@` 未列出任何文件，请先发送不含该文件的第一条消息，然后在下一条消息中 @mention 该文件。
* **附加文件**：通过输入框旁边的 **+** 按钮将图像、PDF 和其他文件附加到您的提示词，或直接将文件拖放到提示词中。这对于共享错误的屏幕截图、设计模型或参考文档很有用。
  * **在 [WSL 会话](https://code.claude.com/docs/zh-CN/desktop-wsl.md)中**：如果 **+** 菜单显示 **Add image** 而不是 **Add files or photos**，则您只能附加图像。要向 Claude 提供会话文件夹中的任何其他文件，请 @mention 它。

### 选择权限模式

权限模式控制 Claude 在会话期间的自主程度：它是否在编辑文件、运行命令或两者之前询问。您可以随时使用输入框下方一行中的模式选择器切换权限模式。要自己批准每项更改，请切换到 Manual。

要为新的本地会话设置默认模式，请将 `permissions.defaultMode` 添加到您的[设置文件](https://code.claude.com/docs/zh-CN/settings.md#where-settings-live)。桌面应用读取与 CLI 相同的设置文件。您在选择器中选择的模式会被记住（按文件夹），并对该文件夹优先于 `defaultMode`，除了 Plan，它仅适用于当前会话。

| 模式 | 设置键 | 行为 |
| - | - | - |
| **Manual** | `default` | Claude 在编辑文件或运行命令之前询问。您会看到 diff，可以接受或拒绝每项更改。 |
| **Accept edits** | `acceptEdits` | Claude 自动接受文件编辑和常见的文件系统命令，如 `mkdir`、`touch` 和 `mv`，但在运行其他终端命令之前仍会询问。当您信任文件更改并希望更快迭代时，请使用此选项。 |
| **Plan** | `plan` | Claude 读取文件并运行命令进行探索，然后提出计划而不编辑您的源代码。适合复杂任务，您想先审查方法。 |
| **Auto** | `auto` | Claude 运行时不需要常规提示；在执行 shell 命令和网络请求等操作之前，后台分类器会检查它们是否与您的请求一致。当[自动模式可用](#auto-mode-availability)时出现；没有单独的设置切换。 |
| **Bypass permissions** | `bypassPermissions` | Claude 运行时不需要权限提示，除了[任何模式都不会自动批准的操作](https://code.claude.com/docs/zh-CN/permission-modes.md#actions-no-mode-auto-approves)、当 Claude [在外部网站上操作](#browse-external-sites)时的安全分类器，或删除会话等桌面操作（但[归档会话](#work-across-sessions)不在此列）。等同于 CLI 中的 `--dangerously-skip-permissions`。在 Pro、Max 和 Ultra 计划上，请在 **Settings > Code** 或 **Settings > Claude Code** 中启用"Allow bypass permissions mode"。在 Team 和 Enterprise 计划上没有设置切换，而是由组织策略控制它。仅在沙箱容器或虚拟机中使用。 |

Code 选项卡的早期版本将这些模式标记为 Ask permissions、Auto accept edits 和 Plan mode。

`dontAsk` 权限模式仅在 [CLI](https://code.claude.com/docs/zh-CN/permission-modes.md#allow-only-pre-approved-tools-with-dontask-mode) 中可用。

> [!TIP]
>
> 在 Plan 中开始复杂任务，以便 Claude 在进行更改之前制定方法。一旦您批准计划，切换到 Accept edits 或 Manual 来执行它。有关此工作流的更多信息，请参阅[计划模式](https://code.claude.com/docs/zh-CN/permission-modes.md#analyze-before-you-edit-with-plan-mode)。

云端会话支持 Accept edits、Plan 和 Auto。Accept edits 对应于 `default` 模式：云端会话预先批准文件编辑，因此选择器显示 Accept edits 而不是 Manual。Bypass permissions 在云端会话中不可用，包括[自托管环境](https://code.claude.com/docs/zh-CN/self-hosted-environments)中的会话。

Enterprise 管理员可以限制哪些权限模式可用。有关详细信息，请参阅[企业配置](#enterprise-configuration)。

#### 自动模式可用性

自动模式对 Anthropic API 上的所有用户可用，需要 Claude Opus 4.6 或更高版本、Sonnet 4.6 或更高版本、Haiku 5.5，或 [Fable 模型](https://code.claude.com/docs/zh-CN/model-config.md#work-with-fable)。组织管理员可以使用[托管设置](#managed-settings)中的 `disableAutoMode` 键关闭自动模式。

在将 Desktop 路由到 Google Cloud 的 Agent Platform 的 Enterprise 部署中，自动模式也默认可用；有关支持的模型，请参阅 [Bedrock、Agent Platform 或 Foundry 上的自动模式](https://code.claude.com/docs/zh-CN/permission-modes.md#enable-auto-mode-on-bedrock-agent-platform-or-foundry)。

### 预览您的应用

Claude 可以启动开发服务器并在浏览器窗格中打开它以验证其更改。这适用于前端 Web 应用以及后端服务器：Claude 可以测试 API 端点、查看服务器日志，并迭代它发现的问题。在大多数情况下，Claude 在编辑项目文件后自动启动服务器。您也可以随时要求 Claude 进行预览。默认情况下，Claude [自动验证](#auto-verify-changes)每次编辑后的更改。

在本地会话中，点击聊天中的 HTML、PDF 或视频路径，即可在浏览器窗格中打开它。

> [!NOTE]
>
> 在 [SSH 会话](#ssh-sessions)中，Claude 无法在浏览器窗格中打开您的应用，因为它在那里没有该窗格的工具。要自己查看应用，请[将其端口转发到您的计算机](#reach-a-port-on-the-remote-machine)。

如果在本地会话中 Claude 启动服务器后浏览器窗格没有打开，请点击聊天中该服务器卡片上的 **Open**。

从浏览器窗格，您可以：

* 直接在浏览器窗格中与运行的应用交互
* 观看 Claude 自动验证其自己的更改：它拍摄屏幕截图、检查 DOM、点击元素、填充表单，并修复它发现的问题
* 从浏览器窗格标题栏中的 **Dev servers** 菜单启动或停止服务器，或一次停止所有服务器
* 通过浏览器窗格 **⋮** 菜单中的 **Keep cookies**，选择浏览器在您退出应用后是否保留 cookie，这样您就不必在开发期间重新登录

Claude 根据您的项目创建初始服务器配置。如果您的应用使用自定义开发命令，编辑 `.claude/launch.json` 以匹配您的设置。有关完整参考，请参阅[配置预览服务器](#configure-preview-servers)。

要清除浏览器保存的数据，请在浏览器窗格的 **⋮** 菜单中选择 **Clear browsing data**。要完全关闭浏览器，请在 **Settings > Code** 或 **Settings > Claude Code** 中关闭 **Browser tools**。

### 浏览外部网站

浏览器窗格是一个选项卡式浏览器，因此您可以在运行的应用旁边打开文档、问题跟踪器或任何其他网站。要打开浏览器，在 macOS 上按 **Cmd+Shift+B** 或在 Windows 上按 **Ctrl+Shift+B**，或点击会话标题栏中的 **Browser**。您可以在窗格中登录网站，包括弹出式登录流程，例如 Google OAuth。

您第一次点击聊天中的外部链接时，会出现一个对话框，询问链接是在浏览器窗格中打开还是在您的默认浏览器中打开。要在之后更改您的选择，请使用浏览器窗格 **⋮** 菜单中的 **Open links in built-in browser**。在 macOS 上 **Cmd** 点击或在 Windows 上 **Ctrl** 点击会直接在您的默认浏览器中打开链接。

Claude 可以使用与[验证您的应用](#preview-your-app)相同的工具读取和交互外部页面，并进行两项额外的安全检查：

* 安全分类器在每个权限模式中审查 Claude 在外部页面上的写入操作，例如点击和输入。这些与[自动模式](#choose-a-permission-mode)使用的分类器相同，当它们标记操作时，无论模式如何，您都会收到权限提示。
* 在 Auto 和 Bypass permissions 以外的权限模式中，在 Claude 导航到新网站之前也会应用域名允许列表检查。

#### 批准 Claude 在网站上的操作

Claude 第一次在外部网站上操作时，会出现权限卡，Claude 等待您的选择：**Allow once**、**Always allow** 或 **Deny**。**Allow once** 批准操作而不保存任何内容。**Always allow** 在您的设备上保存该网站的批准，您可以在设置中撤销它。每个网站都需要自己的批准，包括子域。您的本地开发服务器和项目文件不需要批准，因此[自动验证](#auto-verify-changes)继续工作而不需要提示。

即使在批准的网站上，Claude 也不会在没有您的输入的情况下购买物品、创建账户或绕过 CAPTCHA。在浏览器窗格中浏览使用与 [Claude in Chrome extension](https://code.claude.com/docs/zh-CN/chrome.md) 相同的安全模型。有关 Claude 如何处理敏感网站和风险操作的信息，请参阅[安全使用 Chrome 中的 Claude](https://support.claude.com/en/articles/12902428-using-claude-in-chrome-safely)。

#### 在浏览器和 Chrome 扩展之间选择

浏览器窗格使用干净的浏览器配置文件，与您的个人浏览器分开，没有您保存的登录或历史记录。将其用于构建和测试您的应用以及不需要您的身份的网站。当您想让 Claude 在您的登录会话中以您的身份操作时，请改用 [Claude in Chrome extension](https://code.claude.com/docs/zh-CN/chrome.md)，它共享您的浏览器的登录状态。

#### 为您的组织限制外部浏览

浏览器遵循与 Claude in Chrome 扩展相同的[网站允许列表和阻止列表控制](https://support.claude.com/en/articles/13065128-claude-in-chrome-admin-controls)。如果您的组织已经为扩展配置了这些列表，浏览器会自动遵循它们。管理员也可以使用 [`browserExternalPageTools` 托管设置](#managed-settings)关闭 Claude 在外部页面上的工具。禁用工具后，用户仍然可以访问外部网站；Claude 的工具无法读取或操作它们。

要完全关闭外部浏览，请将 [`disableBrowserExternalNavigation` 托管设置](#managed-settings)设置为 `true`。这会阻止浏览器中的所有外部导航，包括您的组织允许列表上的网站；localhost 开发服务器和文件预览继续工作。使用 `browserExternalPageTools` 让用户继续浏览外部网站而不使用 Claude 的工具，使用 `disableBrowserExternalNavigation` 为用户和 Claude 阻止外部网站。

### 使用 diff 视图审查更改

Claude 对您的代码进行更改后，diff 视图让您在创建 Pull Request 之前逐个文件审查修改。要打开 diff 视图，请点击[会话详情面板](#see-session-details)的 **Changes** 卡片中的行数统计，例如 `+12 -1`。您也可以点击会话标题栏中的 **Changes**。云端会话相对于基础分支的 diff 可能[不包含尚未推送的更改](https://code.claude.com/docs/zh-CN/claude-code-on-the-web.md#troubleshoot-a-missing-file-in-the-diff-view)。

要对特定行进行评论，请将鼠标悬停在某一行上，然后点击其行号旁出现的 **+** 以打开评论框。输入您的反馈并按 **Enter** 添加评论。在多行添加评论后，一次提交所有评论：

* **macOS**：按 **Cmd+Enter**
* **Windows**：按 **Ctrl+Enter**

Claude 读取您的评论并进行请求的更改，这些更改显示为您可以审查的新 diff。

### 审查您的代码

要让 Claude 在您提交之前审查您的更改，请在[输入框](#use-the-prompt-box)中输入 `/code-review`。审查完成后，结果会出现在对话中。

在本地、[SSH](#ssh-sessions) 和 [WSL](https://code.claude.com/docs/zh-CN/desktop-wsl.md) 会话中，结果会显示为一张按文件分组的 **Code review** 卡片。使用该卡片处理这些结果：

* 点击 **Walk through in diff** 打开 diff 视图，逐条查看结果。当前 diff 中的结果会显示在其对应的行上，您可以在那里点击 **Fix this one** 或忽略它。
* 点击 **Apply fixes** 让 Claude 修复仍未处理的结果。

在任何会话中，您也可以在输入框中要求 Claude 修复审查发现的问题。有关 `/code-review` 检查的内容及其接受的参数，请参阅[使用 `/code-review` 在本地审查](https://code.claude.com/docs/zh-CN/code-review-local)。

### 监控 Pull Request 状态

要让 Claude 修复失败的检查，或让 GitHub 合并 Pull Request，请打开 **Auto-fix** 或 **Merge when ready**。打开 Pull Request 后，它会连同其 CI 检查状态一起显示在[会话详情面板](#see-session-details)的 **Changes** 卡片中。打开其标题旁边的 **⋮** 菜单：

* **Auto-fix**：在本地会话中，Claude 还会处理除您之外的其他人留下的新审查评论。评论作者必须是仓库所有者、组织成员、协作者或 GitHub App。
* **Merge when ready**：为该 PR 启用 GitHub 的自动合并。在本地会话中，除非基础分支使用合并队列，否则 Desktop 会请求 squash 合并。
  * **在 SSH 和 WSL 会话中**：**Merge when ready** 处于禁用状态，因此请在 GitHub 上为该 Pull Request 启用自动合并
  * **如果仓库不允许自动合并**：**Merge when ready** 无法打开，因此请先在您的 [GitHub 仓库设置](https://docs.github.com/en/repositories/configuring-branches-and-merges-in-your-repository/configuring-pull-request-merges/managing-auto-merge-for-pull-requests-in-your-repository)中启用自动合并
* **[早期布局](#check-which-layout-you-have)**：会话有 Pull Request 后，点击其状态栏上的 **CI**。

要在 PR 合并或关闭后自动归档会话，请在 **Settings > Code** 或 **Settings > Claude Code** 中打开[自动归档](#archive-a-session)。

> [!NOTE]
>
> PR 监控需要在您的机器上安装并认证 [GitHub CLI (`gh`)](https://cli.github.com/)。如果未安装 `gh`，Desktop 会在您第一次尝试创建 PR 时提示您安装它。
>
> 在 [SSH 会话](#ssh-sessions)中，请在点击 **Create PR** 之前在远程机器上设置好 `git` 和 `gh`，因为 Claude 会在那里推送分支并运行 `gh pr create`。请自行在那里安装 `gh`，因为 Desktop 在 SSH 会话中不会提示您安装。然后在那里检查 `git push` 是否可用，以及 `gh auth status` 是否报告您已登录。

## 整理工作区

Code 选项卡围绕可以以任何布局排列的窗格构建：聊天、diff、浏览器、终端、文件、计划、任务和子代理，以及 macOS 上的 [iOS Simulator](https://code.claude.com/docs/zh-CN/desktop-ios-simulator.md)。通过其标题拖动窗格来重新定位它，或拖动窗格边缘来调整大小。在 macOS 上按 **Cmd+\\** 或在 Windows 上按 **Ctrl+\\** 来关闭焦点窗格。

要打开其他窗格，请点击会话标题栏中对应的图标按钮。将鼠标悬停在按钮上可查看每个按钮的名称，例如 **Terminal**、用于 diff 视图的 **Changes**、**Browser** 和 **Files**。如果所需窗格在标题栏中没有按钮，请从按钮旁的 **⋮** 菜单中选择：

* **窗口较窄时**：放不下的按钮会移入该菜单
* **对于没有按钮的窗格**：**Background tasks** 和 **Plan** 等窗格会在会话有内容可在这些窗格中显示时列出

要在多个屏幕上工作，可以将 diff 或终端等窗格弹出到其自己的窗口中，完成后再停靠回来。Claude 继续在主窗口中工作。

> [!NOTE]
>
> 本部分中的窗格布局、终端、文件编辑器和视图模式需要 Claude Desktop v1.2581.0 或更高版本。在 macOS 上打开 **Claude > Check for Updates…** 或在 Windows 上打开 **Help > Check for Updates…** 来更新。

### 查看会话详细信息

会话详细信息面板是聊天旁的一组卡片，显示会话在何处运行、它更改了什么、它正在运行什么以及其上下文使用情况。该面板在您启动会话后出现，前提是聊天区域足够宽。

#### 隐藏或显示会话详细信息面板

要隐藏或显示该面板，请点击会话标题栏中或标题栏 **⋮** 菜单中的 **Hide session rail** 或 **Show session rail**。如果仍然看不到该面板，请[检查您使用的是哪种 Code 选项卡布局](#check-which-layout-you-have)。

### 在终端中运行命令

集成终端让您无需切换到另一个应用即可在会话旁运行命令。点击会话标题栏中的 **Terminal**，或在 macOS 或 Windows 上按 **Ctrl+\`**。终端在您的会话工作目录中打开，并与 Claude 共享相同的环境，因此 `npm test` 或 `git status` 等命令看到 Claude 正在编辑的相同文件。要打开第二个终端选项卡，点击终端窗格标题中的 **+** 或右键点击聊天中的文件夹来选择 **Open in terminal**。

如果 WSL 会话中没有终端，请参阅[如果终端或连接器不可用](https://code.claude.com/docs/zh-CN/desktop-wsl.md#check-for-the-terminal-and-connectors)。终端在本地和 [SSH](#ssh-sessions) 会话中可用，但在 [Windows 远程机器](#windows-remote-machines)上存在限制。

### 打开和编辑文件

要打开 Claude 提到的文件，请点击聊天中的文件路径。

要自己编辑文件，请使用[文件窗格](#arrange-your-workspace)：

* **本地会话**：您可以编辑会话工作目录中的文件，但 `.mcp.json` 和 `.claude/settings.json` 等部分文件为只读。要更改这些文件，请[在其他应用中打开](#open-files-in-other-apps)。
* **云端、SSH 和 WSL 会话**：文件窗格为只读，因此请要求 Claude 进行编辑。

#### 在文件窗格中保存编辑

默认情况下，您在文件窗格中的编辑会自动保存，因此窗格中不显示 **Save** 按钮。要自行保存编辑，请在窗格的 **⋮** 菜单中取消勾选 **Auto save**。关闭 **Auto save** 后，更改文件后点击 **Save** 来写入您的编辑，或点击 **Cancel** 来撤销编辑。

如果窗格警告文件已在磁盘上更改，请点击 **Override** 用您的编辑覆盖磁盘上的版本，或点击 **Discard** 放弃您的编辑。

### 在其他应用中打开文件

右键点击聊天、diff 查看器或文件窗格中的任何文件路径来打开上下文菜单：

* **Attach as context**：将文件添加到您的下一个提示词
* **Open in**：在已安装的编辑器（如 VS Code、Cursor 或 Zed）中打开文件
* **Show in Finder**（macOS）、**Show in Explorer**（Windows）：打开包含文件夹
* **Copy**：一个子菜单，可复制文件的 **Absolute path**、**Relative path** 或 **Filename**

### 切换视图模式

视图模式控制聊天会话记录中显示的详细程度。要切换视图模式，请从会话标题旁的插入符号打开会话菜单并选择 **Transcript view**，或在 macOS 或 Windows 上按 **Ctrl+O** 循环切换。只有在 Claude 于您正在查看的会话中产生思考后，Thinking 模式才会出现在菜单中。

| 模式 | 显示内容 |
| - | - |
| **Normal** | 工具调用折叠成摘要，带有完整文本回复 |
| **Thinking** | 工具调用折叠成摘要，加上 Claude 的思考 |
| **Verbose** | Claude 采取的每个工具调用、文件读取和中间步骤，加上 Claude 的思考 |

使用 Thinking 来跟踪 Claude 的推理，工具调用仍然折叠。在调试 Claude 为什么采取特定操作时使用 Verbose。Claude Desktop 1.46388.1 之前的版本也列出了 Summary 模式，仍然设置为 Summary 的会话在您更新后会以 Normal 打开。

### 快捷键

在 macOS 上按 **Cmd+/** 或在 Windows 上按 **Ctrl+/** 来查看 Code 选项卡中可用的所有快捷键。本部分中的表格显示的是 macOS 按键。在 Windows 和 Linux 上，请按如下方式替换这些按键：

* **Cmd+Option**：按 **Alt**
* **Cmd+Ctrl**：按 **Ctrl+Alt**
* **Cmd**：按 **Ctrl**

以 `Ctrl` 开头的快捷键在每个平台上保持不变。

| 快捷键 | 操作 |
| - | - |
| `Cmd` `/` | 显示快捷键 |
| `Cmd` `N` | 新会话 |
| `Cmd` `Option` `1`–`9` | 在[当前布局](#check-which-layout-you-have)的新会话屏幕上，选择一个环境 |
| `Cmd` `Option` `0` | 在当前布局的新会话屏幕上，打开 [**New environment** 对话框](#create-a-saved-environment) |
| `Cmd` `W` | 关闭会话 |
| `Ctrl` `Tab` / `Ctrl` `Shift` `Tab` | 在会话之间切换 |
| `Cmd` `Shift` `]` / `Cmd` `Shift` `[` | 下一个或上一个会话 |
| `Esc` | 停止 Claude 的回复 |
| `Tab` / `Right arrow` | 在空输入框中[接受建议的提示词](#accept-a-suggested-prompt) |
| `Cmd` `Shift` `D` | 切换 diff 窗格 |
| `Cmd` `Shift` `B` | 切换浏览器窗格 |
| `Cmd` `Shift` `S` | 在浏览器中选择元素 |
| `Ctrl` `` ` `` | 切换终端窗格 |
| `Cmd` `\` | 关闭焦点窗格 |
| `Cmd` `Ctrl` `W` | 将焦点会话移出[分屏视图](#work-in-parallel-with-sessions) |
| `Cmd` `;` | 打开侧边聊天 |
| `Ctrl` `O` | 循环视图模式 |
| `Cmd` `Shift` `M` | 打开权限模式菜单 |
| `Cmd` `Shift` `I` | 打开模型菜单 |
| `Cmd` `Shift` `E` | 打开工作量菜单 |
| `1`–`9` | 在打开的菜单中选择项目 |

这些快捷键适用于 Code 选项卡。在 Desktop 中，`Shift+Tab` 不会像在终端的[交互模式](https://code.claude.com/docs/zh-CN/interactive-mode.md#keyboard-shortcuts)中那样循环切换权限模式。

### 检查使用情况

要查看此会话的上下文窗口已使用了多少，请先确定您的 Code 选项卡布局：

* **当前布局**：[会话详细信息面板](#see-session-details)中 **Context** 卡片的标题以百分比形式显示该值。打开该卡片可按类别查看占用上下文窗口的内容。
* **[早期布局](#check-which-layout-you-have)**：点击模型选择器旁的使用环形图。其弹出窗口会显示您的上下文窗口使用情况。

在任一布局中，在输入框中输入 `/context`，即可在聊天中打印此会话上下文窗口的按类别细分。

两种布局的模型选择器旁都有使用环形图。点击它可查看您的计划使用情况，该使用情况在您所有的 Claude Code 使用入口之间共享。

## 让 Claude 使用您的计算机

Computer use 让 Claude 能够打开您的应用、控制您的屏幕，并像您一样直接在您的计算机上工作。您可以让 Claude 操作没有 CLI 的桌面工具，或自动化只能通过 GUI 完成的任务。对于运行和测试 iOS 应用，Desktop 会打开专用的 [iOS Simulator 窗格](https://code.claude.com/docs/zh-CN/desktop-ios-simulator.md)，而不是控制您的屏幕；该窗格无需启用 computer use 即可使用。

> [!NOTE]
>
> Computer use 是 macOS 和 Windows 上的研究预览版功能，需要 Pro、Max 或 Ultra 计划。Team 或 Enterprise 计划不提供此功能。Claude Desktop 应用必须处于运行状态。在 macOS 上，computer use 需要 macOS 14 或更高版本。

Computer use 默认处于关闭状态。在 Claude 能够控制您的屏幕之前，请先[在设置中启用它](#enable-computer-use)。在 macOS 上，您还需要授予辅助功能和屏幕录制权限。

在 macOS 上，computer use 还可以在后台运行：Claude 在您已批准的应用中工作，同时您可以继续自己的工作。

> [!WARNING]
>
> 与[沙箱隔离的 Bash 工具](https://code.claude.com/docs/zh-CN/sandboxing.md)不同，computer use 在您的实际桌面上运行，可以访问您批准的任何内容。Claude 会检查每个操作，并标记来自屏幕内容的潜在提示词注入，但信任边界有所不同。有关最佳实践，请参阅 [computer use 安全指南](https://support.claude.com/en/articles/14128542)。

### Computer use 的适用场景

Claude 有多种方式与应用或服务交互，其中 computer use 的适用范围最广，但速度最慢。Claude 会优先尝试最精确的工具：

* 如果您有某个服务的[连接器](#connect-external-tools)，Claude 会使用该连接器。
* 如果任务是 shell 命令，Claude 会使用 Bash。
* 如果任务是浏览器操作，并且您已设置 [Claude in Chrome](https://code.claude.com/docs/zh-CN/chrome.md)，Claude 会使用它。
* 如果任务是运行或测试 iOS 应用，Claude 会使用 [iOS Simulator 窗格](https://code.claude.com/docs/zh-CN/desktop-ios-simulator.md)，该窗格不使用屏幕控制。
* 如果以上都不适用，Claude 会使用 computer use。

[按应用划分的访问级别](#app-permissions)进一步强化了这一点：浏览器的权限上限为仅查看，终端和 IDE 的上限为仅点击，从而在 computer use 处于活动状态时仍引导 Claude 使用专用工具。屏幕控制仅保留给其他方式无法触及的对象，例如原生应用、硬件控制面板或没有 API 的专有工具。

### 启用 computer use

Computer use 默认处于关闭状态。如果在其关闭时您要求 Claude 执行需要它的操作，Claude 会告诉您，只要在设置中启用 computer use，它就可以完成该任务。

**1. 更新桌面应用**

请确保您使用的是最新版本的 Claude Desktop。在 macOS 和 Windows 上，请前往 [claude.com/download](https://claude.com/download) 下载或更新；在 Linux 上，请通过您的包管理器更新（[说明](https://code.claude.com/docs/zh-CN/desktop-linux.md)）。然后重启应用。

**2. 打开开关**

在桌面应用中，打开 **Settings**，然后：

* **如果 Settings 菜单中列出了 This computer**：前往 **This computer > System**
* **否则**：前往 **Capabilities**

在 **Computer use** 下，打开 **Enable computer use**：

* **Windows**：开关立即生效，设置完成
* **macOS**：继续下一步

如果您没有看到该开关，请确认您使用的是 macOS 或 Windows，并且拥有 Pro、Max 或 Ultra 计划，然后更新并重启应用。

**3. 授予 macOS 权限**

在 macOS 上，开关生效前需要授予两项系统权限：

* **辅助功能**：让 Claude 能够点击、输入和滚动
* **屏幕录制**：让 Claude 能够看到您屏幕上的内容

设置页面会显示每项权限的当前状态。如果其中任一权限被拒绝，请点击相应标识以打开对应的系统设置窗格。

### 应用权限

Claude 首次需要使用某个应用时，您的会话中会出现一个权限提示。点击 **Allow for this session** 或 **Deny**。批准在当前会话中有效，在 [Dispatch 创建的会话](#sessions-from-dispatch)中则有效 30 分钟。

该提示还会显示 Claude 对该应用获得的控制级别。这些级别按应用类别固定，无法更改：

| 级别 | Claude 可以执行的操作 | 适用于 |
| :- | :- | :- |
| 仅查看 | 在屏幕截图中查看应用 | 浏览器、交易平台 |
| 仅点击 | 点击和滚动，但不能输入或使用快捷键 | 终端、IDE |
| 完全控制 | 点击、输入、拖动和使用快捷键 | 其他所有应用 |

影响范围较广的应用（例如终端、Finder 或文件资源管理器，以及系统设置或设置）会在提示中显示额外警告，以便您了解批准它们意味着授予了什么权限。

在设置中，您[启用 computer use](#enable-computer-use) 的 **Computer use** 部分包含以下选项：

* **Denied apps**：将应用添加到此处即可直接拒绝，而无需提示。Claude 仍可能通过在已允许应用中的操作间接影响被拒绝的应用，但无法直接与被拒绝的应用交互。
* **Unhide apps when Claude finishes**：当 computer use 未在后台运行时，Claude 会在工作期间隐藏您的其他窗口，以便只与已批准的应用交互。Claude 完成后，隐藏的窗口会恢复显示，除非您关闭此设置。

## 管理会话

每个会话是一个独立的对话，拥有自己的上下文和更改。您可以并行运行多个会话、开启侧边聊天、让 Claude 查看您的其他会话并向其发送消息、将工作发送到云端，或让 Dispatch 从您的手机为您启动会话。

### 使用会话并行工作

点击侧边栏中的 **+ New**，或在 macOS 上按 **Cmd+N**、在 Windows 上按 **Ctrl+N**，即可并行处理多个任务。按 **Ctrl+Tab** 和 **Ctrl+Shift+Tab** 可在会话之间切换。

您还可以在当前会话旁边或在单独的窗口中打开第二个会话：

* **并排显示**：在 macOS 上按住 **Option** 或在 Windows 上按住 **Alt** 并点击侧边栏中的会话，或右键点击该会话并选择 **Open in**，然后选择 **Split view**
* **在单独的窗口中**：在 macOS 上按住 **Cmd** 或在 Windows 上按住 **Ctrl** 并点击会话，或右键点击该会话并选择 **Open in**，然后选择 **New window**

在分屏视图中，您最后打开或点击的窗格具有焦点，另一个窗格会变暗。要更改某个窗格显示的会话，请在该窗格中点击使其获得焦点，然后点击侧边栏中的会话。要将具有焦点的会话移出分屏视图，请在 macOS 上按 **Cmd+Ctrl+W** 或在 Windows 上按 **Ctrl+Alt+W**。

#### 在独立的 worktree 中运行会话

在属于 Git 仓库的项目中，请在启动会话前打开 **Worktree**，使会话在其自己的 [Git worktree](https://code.claude.com/docs/zh-CN/worktrees.md)（项目的隔离副本）中运行。默认情况下，**Worktree** 处于关闭状态，Claude 直接在项目文件夹中工作。在[当前布局](#check-which-layout-you-have)中，打开[输入框下方一行](#start-a-session)中的 **Worktree** 按钮。

* **使用[第三方提供商](https://code.claude.com/docs/zh-CN/third-party-integrations.md)时**：如果输入框下方一行显示了项目文件夹控件但没有 **Worktree** 按钮，请点击该控件并打开 **Worktree**
* **作为[已保存的本地或 SSH 环境](#create-a-saved-environment)的默认值**：在创建或编辑环境时设置 **Worktree**
* **[早期布局](#check-which-layout-you-have)**：选中输入框上方分支名称旁边的 **worktree** 复选框

Worktree 默认存储在 `<project-root>/.claude/worktrees/` 中。要改为将其存储在自定义目录中：

* **本地会话**：在 **Settings > Code** 或 **Settings > Claude Code** 中设置 **Worktree location**
* **SSH 会话**：在 [SSH 连接](#choose-where-ssh-session-worktrees-go)上设置 **Worktree folder**

您还可以在 **Settings > Code** 或 **Settings > Claude Code** 中设置 **Branch prefix**。Desktop 会将其添加到每个 worktree 分支名称的前面，这有助于让 Claude 创建的分支保持条理。

要在新 worktree 中包含被 gitignore 的文件（如 `.env`），请在项目根目录中创建一个 [`.worktreeinclude` 文件](https://code.claude.com/docs/zh-CN/worktrees#copy-gitignored-files-into-worktrees)。有关 worktree 会话从何处读取项目设置、hook 和 skill，请参阅 [worktree 与主检出共享的内容](https://code.claude.com/docs/zh-CN/worktrees.md#what-worktrees-share-with-the-main-checkout)。

> [!NOTE]
>
> 会话隔离需要 [Git](https://git-scm.com/downloads)。大多数 Mac 默认已包含 Git。在终端中运行 `git --version` 进行检查；如果输出了版本号，则说明 Git 已安装。如果遇到 Git 错误，请在 [Cowork](https://claude.com/product/cowork) 中请 Claude 帮助排查您的设置。

### 向会话添加文件夹

您可以在会话启动前或运行期间添加文件夹：

* **本地会话启动前**：在[创建已保存的环境](#create-a-saved-environment)或编辑已保存的环境时，在 **Folders** 下添加文件夹，然后[从该已保存的环境启动会话](#use-edit-or-remove-a-saved-environment)。
* **本地或 [SSH](#ssh-sessions) 会话运行期间**：点击输入框旁边的 **+** 并选择 **Add folder**。Claude 会直接在该文件夹中工作，不使用 [worktree](#run-a-session-in-its-own-worktree)。

### 存档会话

在存档 worktree 中有未提交更改的会话之前，请参阅[存档会话时保留未提交的更改](#keep-uncommitted-changes-when-a-session-is-archived)。要存档会话，请将鼠标悬停在侧边栏中的该会话上，打开其菜单，然后选择 **Archive**。该会话会移至您的[已存档会话](#organize-and-find-sessions)。

要让会话在其 Pull Request 合并或关闭时自动存档，请在 **Settings > Code** 或 **Settings > Claude Code** 中打开 **Auto-archive after PR merge or close**。云端会话和仍在运行的会话不会自动存档。

#### 存档会话时保留未提交的更改

会话被存档时，该会话 worktree 中未提交的更改可能会丢失，因此请先提交您想保留的内容：

* **如果 Desktop 询问这些更改**：选择 **Archive anyway** 以丢弃它们，或取消并提交它们
* **如果更改仅为未跟踪的文件**：请在 Claude 于该会话中创建的 Pull Request 合并之前，提交您想保留的文件，因为届时 Desktop 可能会在不询问的情况下删除它们
* **其他任何情况**：worktree 及其更改会保留在磁盘上。对于本地会话，Desktop 会在会话被存档或您最后一次打开它（以较晚者为准）约 30 天后将两者一并删除。要保留这些更改，请从 [**Archived** 选项卡](#organize-and-find-sessions)打开该会话并选择 **Unarchive**

### 整理和查找会话

您可以通过侧边栏会话列表中的图标对会话进行分组、排序和查找：

* **当前布局**：将鼠标悬停在会话列表中的标题上以显示 **Group and sort** 图标，然后点击它。要查找已存档的会话，请选择菜单最后一行的 **View all**，然后打开 **Archived** 选项卡。
* **[早期布局](#check-which-layout-you-have)**：**Filter** 图标无需悬停即会显示。点击它可使用 **Group by** 或 **Sort by** 排列列表。要显示已存档的会话，请选择 **Status**，然后选择 **Archived**。

要重命名会话，请打开该会话并点击标题栏中的会话标题。

### 检查并释放上下文

要检查上下文使用情况，请参阅[检查使用情况](#check-usage)。当上下文填满时，Claude 会自动总结对话并继续工作。您也可以输入 `/compact` 提前触发总结并释放上下文空间。有关压缩工作原理的详细信息，请参阅[上下文窗口](https://code.claude.com/docs/zh-CN/how-claude-code-works.md#the-context-window)。

### 在会话完成时获得通知

当 Code 会话完成任务且您当前未在查看该会话时，桌面应用会发送操作系统通知。对于属于某个[项目](https://code.claude.com/docs/zh-CN/claude-projects.md#see-what-needs-you-in-overview)的会话，您将收到该项目的通知。

### 在不偏离会话的情况下提出侧边问题

侧边聊天让您可以向 Claude 提出一个使用会话上下文的问题，但不会向主对话添加任何内容。当您想要理解一段代码、检查某个假设或探索某个想法，又不想让会话偏离方向时，可以使用它。

在 macOS 上按 **Cmd+;** 或在 Windows 上按 **Ctrl+;** 打开侧边聊天，或在输入框中输入 `/btw`。侧边聊天可以读取主线程中截至该时刻的所有内容。完成后，关闭侧边聊天，即可从中断处继续主会话。

侧边聊天可在本地、SSH 和 WSL 会话中使用。桌面应用不会将侧边聊天保存到磁盘，因此关闭应用后无法再返回之前的侧边聊天。

### 查看后台任务

任务窗格显示当前会话内正在运行的后台工作：子代理、后台 shell 命令和[动态工作流](https://code.claude.com/docs/zh-CN/workflows.md)。会话中有后台工作后，可通过标题栏 **⋮** 菜单中的 **Background tasks** 打开该窗格。

点击任意条目可在子代理窗格中查看其输出或将其停止。要查看其他会话正在做什么，请使用[侧边栏](#work-in-parallel-with-sessions)，或请 Claude [为您查看](#work-across-sessions)。

### 跨会话工作

Claude 可以列出您的其他 Code 选项卡会话，读取每个会话一直在做的工作，并在它们之间发送消息。用自然语言提问即可："哪个会话涉及了身份验证重构？"、"API 会话得出了什么结论？"或"告诉支付会话 schema 已更改"。

您也可以请 Claude 重命名或存档会话。例如，请它存档 PR 已合并的会话。

通过此使用入口，Claude 只能看到桌面应用自身运行的会话：Code 选项卡中的本地、[SSH](#ssh-sessions) 和 [WSL](https://code.claude.com/docs/zh-CN/desktop-wsl.md) 会话。Claude 看不到云端会话，也看不到您从终端 CLI 或 VS Code 扩展启动的会话，即使它们位于同一项目的 worktree 中也是如此。因此，如果打开了九个终端 worktree 和两个桌面会话，在其中一个桌面会话中回答的 Claude 只会报告另一个桌面会话。Claude 从不会列出您正在提问的那个会话。默认情况下，它可以看到最近活跃的 20 个会话，并跳过已存档的会话，除非您明确要求。[跨会话消息传递](https://code.claude.com/docs/zh-CN/cross-session-messaging.md)则另外让 Claude 可以向[您的其他 Claude Code 会话](https://code.claude.com/docs/zh-CN/cross-session-messaging.md#see-which-sessions-claude-can-reach)发送消息，包括终端会话。

当 Claude 通过此使用入口向另一个会话发送消息时，Claude Code 会在对方会话中将其显示为一张卡片，卡片标有发送会话的标题和返回链接，因此您始终可以知道消息来自何处。如果接收会话正在执行任务，Claude Code 会暂存该消息，Claude 会在当前工作完成后读取它。接收方的 Claude 可以回复，Claude Code 会通过此使用入口将回复传回。Claude 无法向已存档的会话发送消息，并会在消息未送达时告知您。

Claude Code 在跨会话时应用三项安全行为：

* 在 Claude 存档会话之前，您会看到一张批准卡片，但自动模式和 Bypass permissions 模式除外，在这两种模式下 Claude 可以在没有批准卡片的情况下存档会话。如需在存档前被询问，请切换到 Manual。
* Claude Code 会根据接收会话的[入站控制](https://code.claude.com/docs/zh-CN/cross-session-messaging.md#control-inbound-messages)检查来自此使用入口的每条消息，即使接收会话本身未启用[跨会话消息传递](https://code.claude.com/docs/zh-CN/cross-session-messaging.md#availability)。如果您在接收会话中将 [`crossSessionInbound`](https://code.claude.com/docs/zh-CN/settings-reference#crosssessioninbound) 设置为 `refuse`，Claude Code 会丢弃来自此使用入口的消息。Claude Code 会向 Claude 桌面应用报告该拒绝。在 v2.1.234 之前，对于未启用跨会话消息传递的接收会话，Claude Code 会丢弃来自此使用入口的所有消息。
* Claude Code 会引用每条传入消息并注明其来自哪个发送会话，且 Claude 在据此执行操作时仍遵循接收会话自身的权限设置。

### 启动 Claude 建议的任务

当 Claude 注意到当前任务之外值得修复的问题时，会将该工作作为任务提出建议。在[当前布局](#check-which-layout-you-have)的本地和 SSH 会话中，该建议是聊天中的一张卡片，其按钮可以启动该任务，或将其保留在此会话中：

* **Start**：在另一个会话中启动该任务。默认情况下，该任务在当前会话的环境中运行。随后卡片会显示已启动会话的状态，[侧边栏](#work-in-parallel-with-sessions)会在当前会话下方列出已启动的会话。
* **Do it here**：将该任务作为消息发送到当前会话
* **Options**：在点击 **Start** 之前，查看或更改环境、模型、[worktree](#run-a-session-in-its-own-worktree) 和权限等设置
* **Start all**：当卡片列出多个任务时，在各自的会话中启动每个任务，无需等待一个任务完成后再启动下一个

在云端会话中，以及在早期布局的本地和 SSH 会话中，该建议是一张 **Suggested task** 卡片。要按卡片主按钮所示的方式启动任务（例如 **Send to cloud** 或 **Start with worktree**），请点击该按钮。点击主按钮旁边的下拉箭头可查看启动任务的其他方式，包括 **Do it here**。

### 在云端运行长时间运行的任务

对于大型重构、测试套件、迁移或其他长时间运行的任务，请在启动会话时，在[环境选择器](#start-a-session)中选择 **Cloud** 下的某个环境。云端会话默认在 Anthropic 管理的基础设施上运行，即使您关闭应用或关闭计算机也会继续运行。您可以随时回来查看进度，或引导 Claude 转向不同的方向。您也可以从 [claude.ai/code](https://claude.ai/code) 或 [Claude 移动应用](https://code.claude.com/docs/zh-CN/mobile.md)监控云端会话。

云端会话还支持多个仓库。选择仓库的位置：

* **当前布局**：选择云端环境后，在输入框下方的一行中选择。
* **[早期布局](#check-which-layout-you-have)**：在输入框上方选择。要添加另一个仓库，请点击所选仓库旁边的 **+**。

有关云端会话工作方式的更多信息，请参阅[在云端使用 Claude Code](https://code.claude.com/docs/zh-CN/claude-code-on-the-web.md)。当一项工作需要许多云端会话时，请使用[项目](https://code.claude.com/docs/zh-CN/claude-projects.md)：Claude 会在一个对话中为您启动并跟踪这些会话。[创建项目](https://code.claude.com/docs/zh-CN/claude-projects.md#create-a-project)中介绍了具体步骤。

> [!NOTE]
>
> [SSH 会话](#ssh-sessions)中的任务也可以[在 Desktop 断开连接时继续运行](#keep-a-task-running-while-desktop-is-disconnected)，但如果使用 Claude 账户，该任务可能会在登录过期时停止。

### 在另一个使用入口继续

要在其他地方继续会话，请通过会话标题旁边的下拉箭头或侧边栏中该会话所在的行打开会话菜单，然后选择 **Open in**：

* 选择 **Cloud** 可将会话作为[云端会话](https://code.claude.com/docs/zh-CN/claude-code-on-the-web.md)继续，您的对话会以摘要形式带过去。在您确认之前，对话框会说明您的文件是否也会一并移动，以及云端会话就绪后此会话是否会被存档。通过 [SSH](#ssh-sessions) 或在 [WSL](https://code.claude.com/docs/zh-CN/desktop-wsl.md) 中运行的会话无法以这种方式移动。
* 选择已安装的编辑器或文件管理器，可在其中打开该会话在磁盘上的文件夹。

### 控制哪些会话显示在您的其他设备上

本地会话在 [Remote Control](https://code.claude.com/docs/zh-CN/remote-control.md) 将其连接后，会显示在您的其他设备上。已连接的会话会出现在 [claude.ai/code](https://claude.ai/code) 的会话列表中，以及登录了您 claude.ai 账户的设备上的 Claude 应用中。

本地会话会在您为其打开 Remote Control 时连接，或在启动时自动连接：

* **您为该会话打开它**：使用该会话的 **Remote Control** 开关，或在其输入框中输入 `/remote-control`。
* **在启动时连接**：当 **Settings > Code** 或 **Settings > Claude Code** 中的 **Connect new sessions to Remote Control** 处于打开状态时，新会话会自动连接。如果您从未更改过该设置，Desktop 会遵循您的用户设置或托管设置中的 [`remoteControlAtStartup`](https://code.claude.com/docs/zh-CN/settings-reference#remotecontrolatstartup)，其次是您组织的默认值。

要查看会话是否已连接，请查看其[会话详情面板](#see-session-details)中 **Environment** 卡片内的 **Remote Control** 开关。

要让会话不出现在您的其他设备上，请在所需的级别关闭 Remote Control：

* **单个会话**：关闭其 **Remote Control** 开关。在启动时已自动连接的会话中，输入 `/remote-control` 会保持 Remote Control 打开，并显示 `Remote Control is already on. This session connected automatically when it started.`。点击该行上的 **Turn off** 即可断开连接。
* **此计算机上的新 Desktop 会话**：在 **Settings > Code** 或 **Settings > Claude Code** 中关闭 **Connect new sessions to Remote Control**。如果它已显示为关闭，请先将其打开再关闭，以便 Desktop 保存您的选择。保存后，它优先于 `remoteControlAtStartup` 和默认值。
* **此计算机上的任何会话，包括 CLI**：在 `~/.claude/settings.json` 中将 [`disableRemoteControl`](https://code.claude.com/docs/zh-CN/settings-reference#disableremotecontrol) 设置为 `true`，以阻止会话连接。保存该文件时已连接的会话会保持连接，直到您为其关闭 Remote Control。

要隐藏已显示在您其他设备上的会话，请在 Desktop 中将其存档。Desktop 也会存档该会话的 Remote Control 副本，使其从这些设备上的默认会话列表中移除。要在那里查看或删除它，请参阅[存档会话](https://code.claude.com/docs/zh-CN/claude-code-on-the-web.md#archive-sessions)。

### 来自 Dispatch 的会话

[Dispatch](https://support.claude.com/en/articles/13947068) 是一个与 Claude 的持久对话，位于 [Cowork](https://claude.com/product/cowork) 中。您向 Dispatch 发送任务消息，由它决定如何处理。

任务可以通过两种方式成为 Code 会话：您直接提出要求，例如"打开一个 Claude Code 会话并修复登录错误"；或者 Dispatch 判断该任务属于开发工作，并自行创建一个会话。通常会路由到 Code 的任务包括修复错误、更新依赖、运行测试或创建 Pull Request。研究、文档编辑和电子表格工作则保留在 Cowork 中。

#### 跟进 Dispatch 启动的会话

Dispatch 启动的会话会与您的其他会话一起出现在 Code 选项卡的侧边栏中。使用侧边栏的[默认分组和排序](#organize-and-find-sessions)时，请在该会话运行所在文件夹的分组中查找最新的会话。当它完成或需要您批准时，您会在手机上收到推送通知。

如果您启用了[计算机使用](#let-claude-use-your-computer)，由 Dispatch 创建的 Code 会话也可以使用它。这些会话中的应用批准会在 30 分钟后过期并重新提示，而不是像常规 Code 会话那样在整个会话期间有效。

#### 设置 Dispatch

* **计划**：Dispatch 需要 Pro、Max 或 Ultra 计划，Team 或 Enterprise 计划不可用
* **设置、配对和 Dispatch 设置**：请参阅 [Dispatch 帮助文章](https://support.claude.com/en/articles/13947068)

Dispatch 是您离开终端时与 Claude 协作的几种方式之一。有关与其他选项的比较，请参阅[平台和集成](https://code.claude.com/docs/zh-CN/platforms.md#work-when-you-are-away-from-your-terminal)。

## 扩展 Claude Code

连接外部服务、添加可重用工作流、自定义 Claude 的行为并配置预览服务器。要在一个地方管理连接器、skill 和插件，请点击侧边栏中的**自定义**。桌面应用中的 [Cowork](https://claude.com/product/cowork) 从此自定义配置获取其 skill、插件和连接器，该配置通过您的 claude.ai 账户同步，而不是从 CLI 的 `~/.claude` 目录获取。

Claude Code 还会在你使用同一账户登录的终端会话中加载为你的 claude.ai 账户启用的 skills 和插件。请参阅[从 claude.ai 同步的 Skills](https://code.claude.com/docs/zh-CN/skills.md#how-synced-skills-behave) 和[从 claude.ai 同步的插件](https://code.claude.com/docs/zh-CN/plugins/loading.md#synced-plugins)。

### 连接外部工具

连接器是具有图形设置流程的 [MCP 服务器](https://code.claude.com/docs/zh-CN/mcp.md)。

在会话开始之前或期间，点击提示框旁的 **+**，即可在使用 Claude 账户的本地或 [SSH](#ssh-sessions) 会话中添加连接器。选择 **Add connectors** 打开连接器目录。如果菜单显示的是 **Connectors**，请选择它，然后选择 **Browse connectors**。

> [!NOTE]
>
> 以下情况在其他页面中介绍：
>
> * **WSL 会话中 Connectors 显示为灰色**：请参阅[如果终端或连接器不可用](https://code.claude.com/docs/zh-CN/desktop-wsl.md#check-for-the-terminal-and-connectors)
> * **云端会话**：请参阅[连接器如何接入 Claude Code](https://code.claude.com/docs/zh-CN/mcp.md#how-connectors-reach-claude-code)

连接后，Claude 可以读取您的日历、发送消息、创建问题并直接与您的工具交互。您可以询问 Claude 在您的会话中配置了哪些连接器。要管理或断开连接器，请点击侧边栏中的**自定义**并打开 **Connectors**。

对于 Connectors 中未列出的集成，通过[设置文件](https://code.claude.com/docs/zh-CN/mcp.md#installing-mcp-servers)手动添加 MCP 服务器。您也可以[创建自定义连接器](https://support.claude.com/en/articles/11175166-getting-started-with-custom-connectors-using-remote-mcp)。

### 使用 skills

[Skills](https://code.claude.com/docs/zh-CN/skills.md)扩展 Claude 可以做的事情。Claude 在相关时自动加载它们，或者你可以直接调用一个：在提示框中输入 `/` 或点击 **+** 按钮并选择 **Slash commands** 来浏览可用的内容。这包括[内置命令](https://code.claude.com/docs/zh-CN/commands.md)、你的[自定义 skills](https://code.claude.com/docs/zh-CN/skills.md#create-your-first-skill)、来自你的代码库的项目 skills 以及来自任何[已安装插件](https://code.claude.com/docs/zh-CN/plugins/install.md)的 skills。选择一个，它会在输入字段中突出显示。在它之后输入你的任务并照常发送。

您可以在 Claude 工作时发送命令，就像发送任何其他消息一样，会话会在该轮次完成后返回空闲状态。

本地会话从 `~/.claude/skills/` 加载你的个人 skills。[SSH](#ssh-sessions) 会话从远程主机的主目录读取 `~/.claude/skills/`，而不是从你的机器。

本地和云会话也加载为你的 claude.ai 账户启用的 skills，除非你的组织设置了 [`disableSideloadFlags`](https://code.claude.com/docs/zh-CN/settings-reference#disablesideloadflags)。云会话改为加载它们，而不是 `~/.claude/skills/`，如[Cowork 和云会话中的 Skills](https://code.claude.com/docs/zh-CN/skills.md#skills-in-cowork-and-cloud-sessions)所述。

### 安装插件

[Plugins](https://code.claude.com/docs/zh-CN/plugins/overview.md)是可重用的包，为 Claude Code 添加 skills、agents、hooks、MCP servers 和 LSP 配置。你可以从桌面应用安装插件，而无需使用终端。

在本地会话或 [SSH 会话](#ssh-sessions)中，点击提示框旁的 **+** 并选择 **Add plugins** 打开插件浏览器。如果菜单显示的是 **Plugins**，请选择它，然后选择 **Browse plugins**，或选择 **Manage plugins** 来启用、禁用或卸载插件。

你可以将插件限定到你的用户账户、特定项目或仅本地。如果你的组织集中管理插件，这些插件在桌面会话中的可用方式与在 CLI 中相同，除了桌面应用在 [`disableSideloadFlags`](https://code.claude.com/docs/zh-CN/settings-reference#disablesideloadflags) 下扣留的那些。

插件浏览器在云会话中不可用，从桌面应用安装的插件不可用于云会话。云会话也不会安装存储库的 `.claude/settings.json` 声明的插件，如[从你的设置中继承的内容](https://code.claude.com/docs/zh-CN/cloud-environments.md#what-carries-over-from-your-setup)所述。插件在 WSL 会话中不可用。有关完整的插件参考，包括创建你自己的插件，请参阅 [plugins](https://code.claude.com/docs/zh-CN/plugins/overview.md)。

### 配置预览服务器

Claude 自动检测你的开发服务器设置并将配置存储在启动会话时选择的文件夹根目录的 `.claude/launch.json` 中。Preview 使用此文件夹作为其工作目录，因此如果你选择了父文件夹，具有自己开发服务器的子文件夹将不会自动检测。要使用子文件夹的服务器，要么直接在该文件夹中启动会话，要么手动添加配置。

要自定义服务器的启动方式，例如使用 `yarn dev` 而不是 `npm run dev`，或更改端口，请编辑 `.claude/launch.json`。该文件支持带注释的 JSON。

```json theme={null}
{
  "version": "0.0.1",
  "configurations": [
    {
      "name": "my-app",
      "runtimeExecutable": "npm",
      "runtimeArgs": ["run", "dev"],
      "port": 3000
    }
  ]
}
```

你可以定义多个配置来从同一项目运行不同的服务器，例如前端和 API。请参阅下面的[示例](#examples)。

#### 自动验证更改

启用 `autoVerify` 时，Claude 在编辑文件后自动验证代码更改。它拍摄屏幕截图、检查错误并在完成响应之前确认更改有效。

自动验证默认开启。可通过在 `.claude/launch.json` 中添加 `"autoVerify": false` 来按项目禁用它，或在 Browser 窗格的 **⋮** 菜单中关闭 **Auto-verify changes**。

```json theme={null}
{
  "version": "0.0.1",
  "autoVerify": false,
  "configurations": [
    {
      "name": "my-app",
      "runtimeExecutable": "npm",
      "runtimeArgs": ["run", "dev"],
      "port": 3000
    }
  ]
}
```

禁用时，预览工具仍然可用，你可以随时要求 Claude 验证。自动验证使其在每次编辑后自动进行。

#### 配置字段

`configurations` 数组中的每个条目接受以下字段：

| 字段 | 类型 | 描述 |
| - | - | - |
| `name` | string | 此服务器的唯一标识符 |
| `runtimeExecutable` | string | 要运行的命令，例如 `npm`、`yarn` 或 `node` |
| `runtimeArgs` | string\[] | 传递给 `runtimeExecutable` 的参数，例如 `["run", "dev"]` |
| `port` | number | 你的服务器监听的端口。默认为 3000 |
| `cwd` | string | 相对于你的项目根目录的工作目录。默认为项目根目录。使用 `${workspaceFolder}` 显式引用项目根目录 |
| `env` | object | 其他环境变量作为键值对，例如 `{ "NODE_ENV": "development" }`。不要在这里放置秘密，因为此文件会被提交到您的仓库。有关秘密，请参阅[为开发服务器设置环境变量](#set-environment-variables-for-dev-servers)。 |
| `autoPort` | boolean | 如何处理端口冲突。请参阅[端口冲突](#port-conflicts) |
| `program` | string | 用 `node` 运行的脚本。请参阅[何时使用 `program` vs `runtimeExecutable`](#when-to-use-program-vs-runtimeexecutable) |
| `args` | string\[] | 传递给 `program` 的参数。仅在设置 `program` 时使用 |
| `url` | string | preview 打开的地址，而不是 `http://localhost:<port>`。请参阅[在特定 URL 打开 preview](#open-the-preview-at-a-specific-url) |

##### 何时使用 `program` vs `runtimeExecutable`

使用 `runtimeExecutable` 和 `runtimeArgs` 通过包管理器启动开发服务器。例如，`"runtimeExecutable": "npm"` 和 `"runtimeArgs": ["run", "dev"]` 运行 `npm run dev`。

当你有一个想用 `node` 直接运行的独立脚本时，使用 `program`。例如，`"program": "server.js"` 运行 `node server.js`。使用 `args` 传递其他标志。

##### 在特定 URL 打开 preview

默认情况下，preview 打开 `http://localhost:<port>`。当你的服务器需要不同的地址时，设置 `url`。常见情况是需要本地 HTTPS 的服务器、使用 `*.localhost` 子域的应用以及通过重定向登录你的应用。

```json theme={null}
{
  "version": "0.0.1",
  "configurations": [
    {
      "name": "my-app",
      "runtimeExecutable": "npm",
      "runtimeArgs": ["run", "dev"],
      "port": 8443,
      "url": "https://localhost:8443"
    }
  ]
}
```

Localhost 地址直接打开，完全像默认端口地址一样。这包括 `localhost`、任何 `*.localhost` 子域、`127.0.0.1` 和 `::1`。出于安全考虑，localhost `url` 必须仅是你的服务器的源 — 没有路径或查询，端口必须与条目的端口匹配。要显示特定页面，在 preview 打开后要求 Claude 导航到那里。带有路径、查询或不匹配端口的 localhost `url` 被报告为配置错误，该错误命名 url 并显示修复。

对于任何其他地址，Desktop 在 preview 首次打开它时要求你的许可，就像你在 preview 中浏览到新网站时一样。外部地址可能包括路径。选择**始终允许**以在将来跳过该网站的提示。限制 preview 中外部网站的组织策略仍然适用。

要预览你已经自己运行的服务器，设置 `url` 而不设置命令。Claude 将 preview 附加到你的运行服务器，而不是启动一个：

```json theme={null}
{
  "version": "0.0.1",
  "configurations": [
    {
      "name": "my-app",
      "url": "https://app.localhost:3000"
    }
  ]
}
```

`url` 必须是 `http` 或 `https`，且不能包含用户名或密码。

#### 端口冲突

`autoPort` 字段控制当你的首选端口已在使用时会发生什么：

* **`true`**：Claude 自动查找并使用空闲端口。适合大多数开发服务器。
* **`false`**：Claude 失败并出现错误。当你的服务器必须使用特定端口时使用此选项，例如 OAuth 回调或 CORS 允许列表。
* **未设置（默认）**：您的服务器会获得下一个空闲端口，与 `true` 相同，但下面这一种情况除外。在 Claude Desktop v2.37337.0 之前，Claude 会改为询问服务器是否需要该确切端口，然后将您的回答保存在 `launch.json` 中。

如果您的首选端口已被占用且未设置 `autoPort`，当以下条件全部成立时，您的服务器将保持停止状态：

* 您通过 `npm`、`yarn`、`pnpm` 或 `bun` 脚本启动服务器。
* 您的首选端口在该脚本中是硬编码的，或者 Claude 无法检查该脚本。
* 您没有在 `runtimeArgs` 中传递端口。

在这种情况下，请释放该端口、删除硬编码的值，或更改 `port` 以与脚本保持一致。改为设置 `"autoPort": true` 仅对读取 `PORT` 的脚本有效。

请让您的服务器从 `PORT` 环境变量读取其端口，该变量包含所分配的端口。当 Claude 将您的服务器移到其他端口时，它还会相应地重写 `runtimeArgs` 中的 `--port` 或 `-p` 值。

#### 示例

这些配置显示了不同项目类型的常见设置：

> [tab] Next.js
>
> 此配置使用 Yarn 在端口 3000 上运行 Next.js 应用：
>
> ```json theme={null}
> {
>   "version": "0.0.1",
>   "configurations": [
>     {
>       "name": "web",
>       "runtimeExecutable": "yarn",
>       "runtimeArgs": ["dev"],
>       "port": 3000
>     }
>   ]
> }
> ```

> [tab] 多个服务器
>
> 对于具有前端和 API 服务器的 monorepo，定义多个配置。前端使用 `autoPort: true`，因此如果 3000 被占用，它会选择空闲端口，而 API 服务器需要端口 8080：
>
> ```json theme={null}
> {
>   "version": "0.0.1",
>   "configurations": [
>     {
>       "name": "frontend",
>       "runtimeExecutable": "npm",
>       "runtimeArgs": ["run", "dev"],
>       "cwd": "apps/web",
>       "port": 3000,
>       "autoPort": true
>     },
>     {
>       "name": "api",
>       "runtimeExecutable": "npm",
>       "runtimeArgs": ["run", "start"],
>       "cwd": "server",
>       "port": 8080,
>       "env": { "NODE_ENV": "development" },
>       "autoPort": false
>     }
>   ]
> }
> ```

> [tab] Node.js 脚本
>
> 要直接运行 Node.js 脚本而不是使用包管理器命令，使用 `program` 字段：
>
> ```json theme={null}
> {
>   "version": "0.0.1",
>   "configurations": [
>     {
>       "name": "server",
>       "program": "server.js",
>       "args": ["--verbose"],
>       "port": 4000
>     }
>   ]
> }
> ```

## 环境配置

您在[启动会话](#start-a-session)时选择的环境决定了 Claude 在何处执行以及您如何连接：

* **Local**：在您的计算机上运行，可直接访问您的文件
* **Cloud**：默认在 Anthropic 托管的基础设施上运行。即使您关闭应用，会话也会继续。
* **SSH**：在您通过 SSH 连接的远程计算机上运行，例如您自己的服务器、云虚拟机或开发容器。正在运行的任务可以[在 Desktop 断开连接时继续运行](#keep-a-task-running-while-desktop-is-disconnected)，但使用 Claude 账户时，任务可能会在其登录过期时停止。
* **WSL**（Windows）：在您计算机上的 [WSL 2 发行版](https://code.claude.com/docs/zh-CN/desktop-wsl.md)内运行，使用该发行版的 Linux 工具链和原生路径

### 创建已保存的环境

已保存的环境是[环境选择器](#start-a-session)中的一个条目，记录 Claude 的运行位置，以便您之后在该位置启动会话。本地或 SSH 环境还会保存 Claude 工作所用的文件夹。已保存的本地和 SSH 环境属于[当前布局](#check-which-layout-you-have)，因此如果您看不到它们，请检查您使用的是哪种布局以及如何获取当前布局。

**1. 打开 New environment 对话框**

打开输入框下方的环境选择器，然后选择其菜单底部的 **New environment**。

**2. 选择 Claude 的运行位置**

选择 **Local**、**SSH** 或 [**Cloud**](#cloud-sessions) 选项卡，并填写该选项卡上的表单。在 **SSH** 选项卡上，选择一个[连接](#ssh-sessions)以及远程机器上的一个文件夹。

**3. 保存环境**

选择 **Create environment**，或在 **Cloud** 选项卡上选择 **Add environment**。

#### Desktop 为您创建的环境

如果环境选择器中列出了并非由您创建的本地或 SSH 环境，则它们是 Desktop 为您创建的。当您首次在[当前布局](#check-which-layout-you-have)中打开 Code 选项卡时，Desktop 会创建最多八个环境。这些环境来自您在过去一个月中最常使用的本地和 SSH 文件夹。您可以[编辑或删除它们](#use-edit-or-remove-a-saved-environment)。

#### 使用、编辑或移除已保存的环境

在[环境选择器](#start-a-session)中找到该环境的条目：

* **启动会话**：选择该条目。
* **编辑或删除**：将鼠标悬停在条目上并点击齿轮图标。要删除本地或 SSH 环境，请在 **Edit environment** 中点击 **Delete environment**。
* **归档云端环境**：请参阅[归档环境](https://code.claude.com/docs/zh-CN/cloud-environments.md#archive-an-environment)。

### 本地会话中的环境变量

要为本地会话设置环境变量，请[将其添加到已保存的本地环境](#set-environment-variables-for-local-sessions)，然后从该环境启动新会话。

`PATH` 和 `CLAUDE_CONFIG_DIR` 等变量有各自的说明：

| 变量 | 应改为执行的操作 |
| :- | :- |
| `PATH`，即系统搜索 `npm` 等程序的文件夹 | 请参阅[会话找不到已安装的工具](#session-not-finding-installed-tools)。在 Windows 上，Desktop 不读取 PowerShell 配置文件，因此请在用户或系统环境变量中设置 `PATH`，然后退出并重新打开 Desktop。 |
| 来自 [`claude_desktop_config.json`](#mcp-servers-from-the-claude-desktop-chat-app) 的 MCP 服务器所用的变量 | 退出 Desktop，在该服务器的 `env` 块中设置它，然后重新打开 Desktop |
| [`CLAUDE_CONFIG_DIR`](https://code.claude.com/docs/zh-CN/env-vars#variables) 或 [`CLAUDE_CODE_TMPDIR`](https://code.claude.com/docs/zh-CN/env-vars#variables) | 在[为开发服务器设置变量的位置之一](#set-environment-variables-for-dev-servers)中设置它，然后退出并重新打开 Desktop |
| `API_TIMEOUT_MS`、`ANTHROPIC_BASE_URL` 或 Desktop 自行设置的其他变量 | 请参阅 [Desktop 自行设置的环境变量](#environment-variables-desktop-sets-itself) |

在 macOS 或 Linux 上，如果您使用 `export` 设置的变量在 Claude 运行的命令中缺失或值不正确，请参阅[导出的变量在 Claude 运行的命令中缺失或不正确](#exported-variable-missing-or-wrong-in-commands-claude-runs)。

#### 为本地会话设置环境变量

将变量添加到已保存的本地环境中，并从该环境启动会话。Desktop [会自行设置某些变量](#environment-variables-desktop-sets-itself)，例如 `API_TIMEOUT_MS`，对于这些变量，您的会话将获得 Desktop 的值，而不是您的值。

> [!NOTE]
>
> 如果新会话在输入框下方没有显示环境，则说明您使用的是[早期布局](#check-which-layout-you-have)，该布局没有已保存的本地环境。请改用[本地环境编辑器](#use-the-local-environment-editor)。

**1. 打开本地环境的表单**

* **创建已保存的本地环境**：选择环境选择器菜单底部的 [**New environment**](#create-a-saved-environment)，然后选择 **Local** 选项卡
* **修改已有的环境**：在环境选择器中，将鼠标悬停在 **Local** 下的某个已保存环境上，点击齿轮图标以打开 [**Edit environment**](#use-edit-or-remove-a-saved-environment)

**2. 输入变量**

选择 **Environment variables**，每行输入一个 `KEY=value`，然后选择 **Done**。

**3. 保存环境**

选择 **Create environment** 或 **Save**。

只有从该环境启动的会话才会获得这些变量。

要为每个本地会话设置变量，请将其添加到 `~/.claude/settings.json` 文件中的 [`env` 键](https://code.claude.com/docs/zh-CN/env-vars#set-variables-in-settings-files)。如果该变量已存在于 Desktop 启动会话时所用的环境中，Claude Code 会[忽略该值](https://code.claude.com/docs/zh-CN/settings-reference.md#how-env-values-interact-with-your-shell)。有关支持的变量的完整列表，请参阅[环境变量](https://code.claude.com/docs/zh-CN/env-vars.md)。

#### 为开发服务器设置环境变量

要在不将密钥提交到 [`launch.json`](#configure-preview-servers) 的情况下为开发服务器提供密钥，请在以下位置之一进行设置，然后重启 Desktop：

* **macOS 和 Linux**：在您的 shell 配置文件中导出它
* **Windows**：将其添加到您的用户或系统环境变量中
* **任何平台**：如果您能打开[本地环境编辑器](#use-the-local-environment-editor)，请将其保存在其中

开发服务器不会获得您添加到[已保存的环境](#create-a-saved-environment)或 `~/.claude/settings.json` 中的变量。

#### 使用本地环境编辑器

要打开本地环境编辑器，请点击代表您自己计算机的条目上的齿轮图标，而不是已保存环境上的齿轮图标：

* **输入框下方的选择器**：将鼠标悬停在代表您计算机的条目上，即 **Local** 下的第一个条目。如果出现齿轮图标，请点击它。如果没有出现，则表示编辑器不可用，请通过其他方式为[本地会话](#set-environment-variables-for-local-sessions)或[开发服务器](#set-environment-variables-for-dev-servers)设置变量。
* **输入框上方的选择器**：将鼠标悬停在 **Local** 上并点击齿轮图标

每行输入一个 `KEY=value`，然后选择 **Save changes**。您在此处保存的变量会以加密方式存储在您的机器上。本地会话会获得这些变量，但 [Desktop 自行设置的](#environment-variables-desktop-sets-itself)任何变量（例如 `API_TIMEOUT_MS`）除外。重启 Desktop 即可将您的变量提供给开发服务器。

#### Claude 运行的命令中缺少导出的变量或变量值错误

如果 Claude 在 macOS 或 Linux 上运行的命令没有获得您在 `~/.zshrc` 或 `~/.bashrc` 中使用 `export` 设置的变量，或者获得了错误的值，请按顺序尝试以下操作：

**1. 启动新会话**

让 Claude 再次运行该命令。

**2. 退出并重新打开 Desktop**

然后启动新会话。

**3. 检查已保存的变量**

如果命令获得了错误的值，请检查您是否也在[已保存的环境](#use-edit-or-remove-a-saved-environment)或[本地环境编辑器](#use-the-local-environment-editor)中设置了该变量。如果是，请在那里修改该值并启动新会话。

**4. 使用已保存的环境**

将该变量添加到[已保存的本地环境](#set-environment-variables-for-local-sessions)，然后从该环境启动新会话。对于 `PATH`、`CLAUDE_CONFIG_DIR` 或 `API_TIMEOUT_MS` 等变量，请改为按照[其各自的说明](#environment-variables-in-local-sessions)操作。

在 v2.1.304 之前，Claude Code 不会将导出的变量传递给命令，因此请[检查会话运行的版本](#claude-code-version-in-the-code-tab)。

#### Desktop 自行设置的环境变量

如果您设置了 `API_TIMEOUT_MS` 或 `ANTHROPIC_BASE_URL`，但本地会话中没有任何变化，这是因为 Desktop 会自行设置这些[变量](https://code.claude.com/docs/zh-CN/env-vars.md#variables)以及其他一些变量。无论您是在 shell 配置文件、Windows 用户或系统变量、本地环境编辑器还是已保存的环境中设置自己的值，本地会话都会获得 Desktop 的值。将您的值移到设置文件中也无济于事，因为 Claude Code 在本地会话中会[忽略该值](https://code.claude.com/docs/zh-CN/settings-reference.md#how-env-values-interact-with-your-shell)。

* `API_TIMEOUT_MS`：本地会话获得的值为 `900000`，即 15 分钟。如需更长的超时时间，请[安装 Claude Code CLI](https://code.claude.com/docs/zh-CN/quickstart.md) 并使用它来代替 Desktop。[在您的 shell 中设置该变量](https://code.claude.com/docs/zh-CN/env-vars.md#set-variables-in-your-shell)，然后在终端中运行 `claude`。
* `ANTHROPIC_BASE_URL`：当您使用 Claude 账户登录时，本地会话获得的是 Anthropic 的 API 主机。要使用网关，请改为设置 Desktop 的[第三方推理配置](https://code.claude.com/docs/zh-CN/llm-gateway-connect.md#desktop-app)。

#### 关闭扩展思考

[扩展思考](https://code.claude.com/docs/zh-CN/model-config.md#extended-thinking)默认启用，它可以提升复杂推理任务的表现，但会消耗额外的 token。要关闭它，请在[已保存的本地环境](#set-environment-variables-for-local-sessions)或[本地环境编辑器](#use-the-local-environment-editor)中将 `MAX_THINKING_TOKENS` 设置为 `0`。随后，思考功能会在 Anthropic API 上关闭，但 Opus 5.5、Sonnet 5.5、Haiku 5.5 和 Fable 模型除外，这些模型始终使用扩展思考。

在 Opus 5 上，还需要在启动会话之前选择 `high` 或更低的 effort。在[当前布局](#check-which-layout-you-have)中，打开模型选择器并在 **Effort** 下选择一个级别。如果您以更高的级别启动了会话，请以 `high` 或更低的级别启动一个新会话。

#### 固定思考预算

在支持[自适应推理](https://code.claude.com/docs/zh-CN/model-config.md#adjust-effort-level)的模型上，Claude Code 会忽略正值 `MAX_THINKING_TOKENS` 中的具体数值，因为自适应推理会改为控制思考深度。在 Opus 4.6 和 Sonnet 4.6 上，将 `CLAUDE_CODE_DISABLE_ADAPTIVE_THINKING` 设置为 `1` 可使用固定的思考预算；Fable 模型、Sonnet 5 及更高版本、Haiku 5.5 以及 Opus 4.7 及更高版本始终使用自适应推理，没有固定预算模式。

### 受管设备上的本地会话

如果代表您计算机的条目在环境选择器中显示为灰色，并带有工具提示 **Disabled by your organization**，则表示您的管理员已[关闭本地会话](#managed-settings)。在 Windows 上，[WSL](https://code.claude.com/docs/zh-CN/desktop-wsl.md) 条目也会以同样的方式显示为灰色，其在受管设备上的可用性[另行管控](https://code.claude.com/docs/zh-CN/admin-setup.md#wsl-sessions-in-claude-code-desktop)。

如果已配置 SSH 连接，新会话默认使用第一个 SSH 连接。如果您尝试继续现有的本地会话，Desktop 会显示一条消息，说明此设备上无法使用本地会话。当您选择[已保存的本地环境](#create-a-saved-environment)时，会显示此错误：`Couldn’t switch environments: the Local environment isn’t available anymore.` 请改为选择 [SSH](#ssh-sessions) 或[云端](#cloud-sessions)环境，或联系您的 IT 团队。

### 云端会话

即使您关闭应用，云端会话也会在后台继续运行。用量计入您的[订阅套餐限额](https://code.claude.com/docs/zh-CN/costs.md)，不单独收取计算费用。

要添加、编辑或[归档](https://code.claude.com/docs/zh-CN/cloud-environments.md#archive-an-environment)云端环境，请打开[环境选择器](#start-a-session)：

* **添加环境**：选择选择器菜单底部的 **New environment**，然后选择 [**New environment** 对话框](#create-a-saved-environment)中的 **Cloud** 选项卡
* **编辑或归档您自己的某个环境**：将鼠标悬停在该环境上，然后点击齿轮图标

有关配置网络访问和环境变量的详细信息，请参阅[配置云端环境](https://code.claude.com/docs/zh-CN/cloud-environments.md)。

### SSH 会话

SSH 会话让您可以在远程计算机上运行 Claude Code，同时使用桌面应用作为界面。这对于处理位于云虚拟机、开发容器或具有特定硬件或依赖的服务器上的代码库非常有用。

> [!NOTE]
>
> 如果您将 Desktop 与[第三方提供商](https://code.claude.com/docs/zh-CN/third-party-integrations.md)配合使用，SSH 会话需要单独设置。请参阅 [3P 上 Claude Desktop 中的 SSH 远程会话](https://claude.com/docs/third-party/claude-desktop/ssh-remote-sessions)。

在 [**New environment** 对话框](#create-a-saved-environment)中添加 SSH 连接，然后在该连接上启动会话。如果新会话在输入框下方没有显示环境，则说明您使用的是早期布局。请参阅[检查您使用的布局](#check-which-layout-you-have)。

**1. 打开 New environment 对话框**

在启动会话之前，打开输入框下方的环境选择器，然后选择其菜单底部的 **New environment**。

**2. 添加连接**

在 **New environment** 对话框的 **SSH** 选项卡上，打开 **Host** 列表，选择 **Add SSH connection…**，然后填写连接详细信息：

* **Name**：此连接的友好标签。
* **SSH host**：`user@hostname` 或 `~/.ssh/config` 中定义的主机。
* **SSH port**：留空时默认为 22，或使用您的 SSH 配置中的端口。
* **SSH key (optional)**：您的私钥路径，例如 `~/.ssh/id_ed25519`。留空则使用您的 SSH 配置或 SSH agent。
* **Worktree folder**：远程机器上的一个文件夹，例如 `~/worktrees`，新会话会在其中创建 worktree。留空则使用[远程机器的默认值](#choose-where-ssh-session-worktrees-go)。

然后选择 **Add SSH connection**。

**3. 在该连接上启动会话**

选择远程机器上的一个文件夹，然后点击 **Create environment**。您会将该机器和文件夹作为[已保存的环境](#create-a-saved-environment)保留下来，供以后的会话使用。Desktop 会为新会话选择它，因此请输入您的任务并按 **Enter**。

远程计算机可以运行 Linux、macOS 或 [Windows](#windows-remote-machines)。首次连接时，Desktop 会自动在远程计算机上安装 Claude Code。连接后，SSH 会话支持权限模式、连接器、插件和 MCP 服务器。

#### SSH 会话的远程计算机要求

Linux 或 macOS 远程计算机需要受支持的处理器、带 SFTP 的 SSH 服务器、未以 `noexec` 挂载的安装文件夹、可正常工作的 shell 以及网络访问。请在连接之前检查每项要求，或在登录后连接失败时进行检查：

* **系统**：x86\_64 或 arm64 处理器。该计算机还必须满足 Claude Code 的[系统要求](https://code.claude.com/docs/zh-CN/setup.md#system-requirements)。在 Linux 上，Claude Code 需要 glibc 2.17 或更高版本，或 musl。若要查看该计算机使用的是哪一种，请在其上运行 `ldd --version`。在 Alpine Linux 等基于 musl 的发行版上，还需安装 [Claude Code 所需的软件包](https://code.claude.com/docs/zh-CN/setup.md#alpine-linux-and-musl-based-distributions)。
* **SFTP**：SSH 服务器必须提供 SFTP 子系统，该子系统通过其 `sshd_config` 中的 `Subsystem sftp` 行启用。若要检查，请在您计算机上的终端中使用 `sftp` 连接到相同的主机和端口。正常工作的服务器会显示 `sftp>` 提示符。
* **服务器上的 `MaxSessions 1`**：要连接到其 `sshd_config` 设置了 `MaxSessions 1` 的服务器，请执行以下操作之一：
  * **您的计算机运行 macOS 或 Linux**：更新到 Claude Desktop v3.1623.0 或更高版本
  * **您的计算机运行 Windows**：确保 Desktop [通过 OpenSSH 客户端连接](#use-your-ssh-configuration)，因为 Desktop 的内置 SSH 客户端无法连接到该服务器
  * **您可以更改服务器**：调高 `MaxSessions`，例如调到 OpenSSH 的默认值 10
* **使用 `MaxSessions 1` 时的登录**：使用无需提示即可完成的登录方式，例如由您的 SSH agent 持有的密钥。Desktop 每次连接或重新连接时至少登录两次。每个[终端](#run-commands-in-the-terminal)选项卡，以及您在其中运行以 `!` 开头的命令的每个 Desktop 会话，预计还需至少再登录一次。Claude 运行的命令则不需要额外登录。
* **使用 `MaxSessions 1` 时的一次性验证码**：如果每次登录都会提示您（例如使用一次性验证码时），`!` 命令会失败。请改为在终端选项卡中运行这些命令，终端选项卡会在其自己的窗格中提示。
* **安装文件夹**：Desktop 会在您主目录下的 `~/.claude/remote/` 中安装一个辅助程序和 Claude Code。它从该位置运行这些程序，因此该文件夹不能位于以 `noexec` 挂载的文件系统上。请在每个连接用户的主目录中预留约 1 GB 的磁盘空间。有关该文件夹包含的内容，请参阅[远程计算机上的文件和进程](#files-and-processes-on-the-remote-machine)。
* **Shell**：远程计算机上为您的账户设置的 shell 必须能运行通过 SSH 发送的命令，Bash、Zsh 和 `sh` 均可
* **网络**：Claude Code 在远程计算机上运行，并从那里发出网络请求，因此该计算机需要具备 [Claude Code 所需的网络访问](https://code.claude.com/docs/zh-CN/network-config.md#network-access-requirements)。如果该计算机无法下载 Claude Code，Desktop 会在您的计算机上下载并上传它。Claude Code 的其他请求仍从远程计算机发出。在 Linux 远程计算机上，如果这些请求需要代理，请[为 SSH 会话设置代理](#set-a-proxy-or-a-ca-certificate-for-ssh-sessions)。

#### 检查远程机器的主机密钥

主机密钥是远程机器出示的、用于证明自身身份的密钥，而指纹是该密钥的简短形式。当您信任某个密钥时，您的计算机会保存它，之后连接到出示该密钥的任何机器时都不会再询问。请在信任密钥之前先检查它，因为可能有其他方在拦截连接。

首先，根据您看到指纹的位置，确定该密钥属于哪台机器：

* **Desktop 询问 `Connect to <host> for the first time?`，或者您选择了 Review new key**：Desktop 指明的机器，即 `<host>`
* **Desktop 询问 `Connect through <jump host> for the first time?`**：跳板机，即您的连接所经过的机器，而不是您想要访问的机器
* **`ssh` 在终端中询问或发出警告**：其消息中指明的机器，这可能不是您输入的那台

从该机器本身获取其指纹，而不是通过您正在检查的连接获取：

* **该机器由他人管理**：向对方索取其主机密钥指纹
* **由您管理，且运行 Linux 或 macOS**：在该机器的控制台上，或通过您已信任的连接，在该机器上运行 `ssh-keygen -lf /etc/ssh/ssh_host_<type>_key.pub`。对于 `<type>`，请使用 Desktop 或 `ssh` 在指纹旁显示的密钥类型的小写形式：`ED25519` 或 `ssh-ed25519` 对应 `ed25519`。

然后将该机器本身的指纹与您看到的指纹进行比较：

* **两者一致**：信任该密钥。在 Desktop 中选择 **Trust and connect** 或 **Trust new key**，或在终端中回答 `yes`。
* **两者不一致，或者您无法检查**：不要连接。在 Desktop 中选择 **Cancel**，或在终端中回答 `no`。如果该机器由他人管理，请告知对方您看到的指纹。

如果您是从某个故障排除步骤跳转到此处的，无论结果如何，请返回该步骤。

#### 为 SSH 会话设置代理或 CA 证书

如果 Linux 远程计算机通过代理访问互联网或需要 CA 证书，请在该计算机上的 `~/.claude/settings.json` 中的 `env` 下添加[代理和 CA 变量](https://code.claude.com/docs/zh-CN/network-config.md)。即使您已在该计算机上的 `~/.profile`、`~/.bash_profile`、`~/.zprofile` 或 `~/.zshrc` 中导出它们，也要添加。

在 SSH 会话中，Claude Code 不会从这些文件中获取变量，即使在该机器终端中运行的 `claude` 能获取到。此时会话可能会挂起，或显示 **The connection from … to the API failed** 或 **Authentication failed**，且重新登录也无济于事。

**1. 添加变量**

对于 CA 文件，请使用 [`NODE_EXTRA_CA_CERTS`](https://code.claude.com/docs/zh-CN/network-config#custom-ca-certificates)，因为 Claude Code 不信任 `~/.claude/settings.json` 中 `SSL_CERT_FILE` 指定的 CA。此示例设置了代理和 CA 文件：

```json ~/.claude/settings.json theme={null}
{
  "env": {
    "HTTPS_PROXY": "http://proxy.example.com:8080",
    "NODE_EXTRA_CA_CERTS": "/etc/ssl/certs/example-ca.pem"
  }
}
```

**2. 重新发送您的消息**

请重新发送您的消息，因为您在该文件中添加或更改的变量会在几秒钟内生效。如果仍然失败，请启动新会话。

##### 查找远程计算机上其他位置设置的代理或 CA 变量

请检查 Linux 远程计算机是否在其他位置设置了代理或 CA 变量，因为会话会使用该值，并[忽略任何设置文件中的值](https://code.claude.com/docs/zh-CN/settings-reference.md#how-env-values-interact-with-your-shell)：

**1. 列出远程计算机设置的变量**

在您计算机上的终端中运行 `ssh devbox env`，并替换为您自己的主机。

**2. 在设置变量的位置更改它**

如果列表中有该变量，请在设置它的文件中进行更改，例如远程计算机上的 `~/.zshenv`、`/etc/environment` 或 `~/.bashrc`。

**3. 让会话获取新值**

重启远程计算机，或退出 Desktop 并结束辅助程序的进程，如[移除辅助程序](#files-and-processes-on-the-remote-machine)的前两个步骤所述。请在没有任务运行时执行此操作，因为任一方式都会中断 Claude 的工作。

#### 使用您的 SSH 配置

当 Desktop 通过您计算机上的 OpenSSH 客户端连接时，您可以在 macOS、Linux 和 Windows 上使用您的 SSH 配置：

* **macOS 和 Linux**：Desktop 通过运行您计算机上的 OpenSSH 客户端进行连接，因此会使用您的 `~/.ssh/config`。通过 OpenSSH 客户端连接需要 Claude Desktop v1.37937.0 或更高版本。
* **Windows**：当 Desktop 通过 OpenSSH 客户端连接时，它会使用您的 `%USERPROFILE%\.ssh\config`。如果您使用的是 Claude 账户而不是第三方提供商，则通过 OpenSSH 客户端连接需要满足以下所有条件：

  * Claude Desktop v2.2553.0 或更高版本。要查找您的版本，请参阅[检查您的版本](#check-your-version)。
  * Microsoft 的 OpenSSH 9.4 或更高版本
  * 使用本页顶部的下载按钮安装的 Desktop，因为来自 Microsoft Store 的 Desktop 不会通过 OpenSSH 客户端连接

  如果您无法满足这些要求，或者您使用的是第三方提供商，请参阅[在 Windows 上不使用 OpenSSH 客户端进行连接](#connect-on-windows-without-the-openssh-client)。

使用 OpenSSH 客户端时，请在 **SSH host** 字段中输入配置文件中的 `Host` 名称。随后 Desktop 会使用该 `Host` 条目中的设置，例如 `HostName`、`User`、`IdentityFile`、`CertificateFile`、`ProxyJump` 和 `ProxyCommand`。

添加连接之前，请在您计算机上的终端中检查客户端和条目：

* **macOS 和 Linux 上的客户端版本**：运行 `ssh -V`。客户端必须是 OpenSSH 7.6 或更高版本。
* **Windows 上的客户端版本**：Desktop 会首先尝试 `PATH` 上的 `ssh`，因此请运行 `ssh -V`。如果输出显示 `OpenSSH_for_Windows_` 且版本为 9.4 或更高，则该客户端符合要求。如果不符合，请检查 Desktop 接下来会尝试的两个文件夹：先在 `C:\Windows\System32\OpenSSH` 中运行 `.\ssh.exe -V`，然后在 `C:\Program Files\OpenSSH` 中运行。
* **Windows 上没有符合要求的客户端**：在 Windows 中添加 **OpenSSH Client** 可选功能并安装待处理的 Windows 更新，或者安装 [Microsoft 的较新构建版本](https://learn.microsoft.com/en-us/troubleshoot/windows-server/system-management-components/upgrade-in-box-openssh-to-latest-openssh-release)。然后退出并重新打开 Desktop。要查看 Desktop 是否找到了可用的 OpenSSH 客户端，请在 [`ssh.log`](#get-desktops-logs-for-an-ssh-problem) 中查找 `Windows OpenSSH client:` 及版本号，或查找 `no usable Windows OpenSSH client`。
* **条目**：在 Windows 上，请使用通过了版本检查的客户端。使用 `Host` 名称运行 `ssh`，例如 `ssh devbox`。如果失败，请先修复该条目。

您可以将 **SSH port** 和 **SSH key (optional)** 字段留空。如果填写其中之一，Desktop 会将其与条目一起使用：

* **SSH port**：此处的值会覆盖条目的 `Port`
* **SSH key (optional)**：使用 OpenSSH 客户端时，除了条目的 `IdentityFile` 和 SSH agent 中的密钥外，Desktop 还会提供您选择的密钥文件。在 Windows 上，请选择 OpenSSH 格式的密钥，因为 `ssh` 无法读取 PuTTY `.ppk` 文件。

##### 在 Windows 上不使用 OpenSSH 客户端进行连接

在 Windows 上，如果您的计算机不满足 [OpenSSH 客户端的要求](#use-your-ssh-configuration)，您将通过 Desktop 的内置 SSH 客户端进行连接。使用第三方提供商时，默认通过内置客户端连接，您的管理员可以更改此设置。有关该设置，请参阅 [SSH remote sessions in Claude Desktop on 3P](https://claude.com/docs/third-party/claude-desktop/ssh-remote-sessions)。

使用内置客户端时，如果您的 `PATH` 中有 `ssh` 命令，您仍可以在 **SSH host** 字段中输入 SSH 配置中的 `Host` 名称，但有以下限制：

* **条目中的设置**：Desktop 会采用 `HostName`、`User`、`Port` 和 `IdentityFile`。如果您的 `PATH` 上有 `sh` shell，它还会采用 `ProxyCommand`。
* **SSH key (optional)**：将此字段留空以保留条目的 `IdentityFile`，因为您在此处选择的密钥文件会替换 `IdentityFile`
* **`CertificateFile` 和 `ProxyJump`**：Desktop 会忽略 `CertificateFile`，如果条目设置了 `ProxyJump`，连接会失败。要使用其中任一设置，请[通过 OpenSSH 客户端连接](#use-your-ssh-configuration)。

##### 访问跳板机后面的计算机

若要访问位于跳板机（也称为堡垒机）后面的计算机，请在条目中设置 `ProxyJump` 或 `ProxyCommand`。以下条目通过 `bastion.example.com` 访问 `devbox.internal.example.com`：

```text ~/.ssh/config theme={null}
Host devbox
  HostName devbox.internal.example.com
  User alice
  ProxyJump bastion.example.com
```

在 **SSH host** 字段中输入 `devbox`。

如果您的管理员[限制了 SSH 主机](#allow-jump-hosts)，跳板机也必须在列表中。在您计算机的终端中运行 `ssh -G bastion.example.com`，并请管理员允许其输出的 `hostname`。

##### 访问远程计算机上的容器

若要在未发布 SSH 端口的容器内运行会话，请使用 `ProxyCommand` 访问它，因为 Desktop 不会运行您所连接条目的 `RemoteCommand`。如果您的条目使用 `RemoteCommand` 进入容器，会话将在该条目登录的计算机上运行，而不是在容器中运行。

**1. 添加条目**

将以下条目添加到 `~/.ssh/config` 的末尾，如果您有 `Host *` 条目，则添加到第一个 `Host *` 条目的正上方。将 Docker 容器 `devbox`、其用户 `dev` 和计算机 `build.example.com` 替换为您自己的值。该条目的命令会为每个连接在容器内启动一个 SSH 服务器：

```text ~/.ssh/config theme={null}
Host devbox-container
  User dev
  ProxyCommand ssh build.example.com docker exec -i devbox /usr/sbin/sshd -i
```

**2. 准备容器**

* 安装 OpenSSH 服务器，并使用在您重新构建镜像或重新创建容器时保持不变的主机密钥
* 为 `User` 中指定的账户提供登录方式，例如 `authorized_keys` 文件
* 检查容器是否满足[远程计算机要求](#remote-machine-requirements-for-ssh-sessions)

**3. 检查允许列表**

如果您的管理员[限制了 SSH 主机](#restrict-which-ssh-hosts-users-can-connect-to)，请在您计算机上的终端中运行 `ssh -G devbox-container`。请管理员在 `sshHostAllowlist` 中允许它输出的 `hostname`（此处为 `devbox-container`）。当他们允许该名称时，也就允许了您条目的 `ProxyCommand` 所连接的任何目标。

**4. 测试条目，然后连接**

在您计算机上的终端中运行 `ssh devbox-container`。修复它报告的问题，然后再次运行，直到它无需询问您任何内容即可在容器中打开 shell：

* **如果 `ssh` 要求输入密码、口令短语或验证码**：改用无需提示即可工作的登录方式，例如由您的 SSH agent 持有的密钥
* **如果 `ssh` 报告 `Cannot execute command-line and remote command.` 或 `Bad packet length`**：从 `build.example.com` 的条目中移除 `RemoteCommand` 和 `RequestTTY`。之后，在您计算机上的终端中运行 `ssh build.example.com` 会在该计算机上打开 shell，而不是在容器中。
* **如果 `ssh` 报告 `Connection closed by UNKNOWN port 65535`**：在同一终端中运行 `ssh build.example.com docker exec devbox /usr/sbin/sshd -t`。
  * 如果它输出 `Missing privilege separation directory: /run/sshd`，请在容器中创建该文件夹。
  * 如果它输出 `no hostkeys available` 且容器有主机密钥，请在此命令和条目中的 `exec` 后添加 `-u root`。

然后在 **SSH host** 字段中输入 `devbox-container`。

##### 在 `ProxyCommand` 中使用来自 shell 的变量

在 macOS 和 Linux 上，要在 `ProxyCommand` 或 `Match exec` 命令中使用您自己的环境变量，请在 shell 启动文件（例如 `~/.zshrc`）中导出它。然后退出并重新打开 Desktop。Desktop 启动时会读取您的登录 shell 导出的内容，并在您退出之前一直保留这些值。

这需要 Claude Desktop v2.37337.0 或更高版本，以及一个接受 `-0` 的 `env` 命令，您可以在计算机上的终端中运行 `env -0` 进行检查。

> [!NOTE]
>
> 如果您的 shell 设置了临时 AWS 凭据，请[为 AWS CLI 提供一个 profile](#use-aws-credentials-in-a-proxycommand)，而不是导出 `AWS_*` 变量。

##### 在 macOS 和 Linux 上的 `ProxyCommand` 中使用 AWS 凭据

如果您在 shell 启动文件中设置了临时 AWS 凭据，运行 AWS CLI 的 `ProxyCommand` 可能在终端中能够连接，而在 Desktop 中失败。当您的 shell 导出 `AWS_SESSION_TOKEN`、`AWS_SECURITY_TOKEN` 或 `AWS_CREDENTIAL_EXPIRATION` 时，Desktop 中的 `ssh` 和您的 `ProxyCommand` 不会获得 shell 的 AWS 凭据、`AWS_REGION` 或 `AWS_PROFILE`。

如果 AWS CLI 没有其他区域或凭据来源，它会失败并显示 `You must specify a region` 或 `Unable to locate credentials`。若要从 Desktop 连接，请为 AWS CLI 提供一个它从 `~/.aws/config` 读取的 profile：

**1. 添加包含区域和凭据来源的 profile**

此示例添加一个名为 `corp` 的 profile，它从辅助程序获取凭据：

```ini ~/.aws/config theme={null}
[profile corp]
region = us-east-1
credential_process = /full/path/to/your-credential-helper
```

* **`region`**：指定您所连接计算机的 AWS 区域
* **`credential_process`**：指定一个程序的完整路径，该程序以 [AWS 文档所述的 JSON 格式](https://docs.aws.amazon.com/cli/latest/userguide/cli-configure-sourcing-external.html)输出凭据。请选择无需您输入的程序，因为 AWS CLI 会在每次连接时运行它，包括您不在时的重新连接。
* **IAM Identity Center**：添加该 profile 的 [IAM Identity Center 设置](https://docs.aws.amazon.com/cli/latest/userguide/cli-configure-sso.html)来代替 `credential_process`。在连接之前，以及该登录失效时，在终端中运行 `aws sso login --profile corp`。

**2. 在 ProxyCommand 中指定 profile**

此示例将 `--profile` 添加到 [AWS 为 Session Manager 记录的命令](https://docs.aws.amazon.com/systems-manager/latest/userguide/session-manager-getting-started-enable-ssh-connections.html)中：

```text ~/.ssh/config theme={null}
Host i-*
  ProxyCommand sh -c "aws ssm start-session --profile corp --target %h --document-name AWS-StartSSHSession --parameters 'portNumber=%p'"
```

**3. 重新连接**

在终端中使用该主机运行 `ssh`，确认该条目在终端中仍能连接，然后从 Desktop 连接。

##### 提高 SSH 连接超时时间

如果您的 SSH 配置没有为该主机设置 `ConnectTimeout`，Desktop 会给 `ssh` 30 秒的连接时间。通过跳板机或 `ProxyCommand` 连接时，该时间包括路由建立连接所需的时间。如果您的路由需要更长时间，请在条目中添加更高的值：

```text ~/.ssh/config theme={null}
Host devbox
  HostName devbox.internal.example.com
  User alice
  ProxyJump bastion.example.com
  ConnectTimeout 120
```

`ssh` 会使用它为主机找到的第一个值，因此请将该条目放在任何设置了 `ConnectTimeout` 的 `Host *` 条目之上。若要确认，请在您计算机上的终端中运行 `ssh -G devbox`。其输出包含 `connecttimeout 120`。更改该值后，请退出并重新打开 Desktop。

如果远程计算机本身等待您在手机上批准，无论您设置的 `ConnectTimeout` 是多少，都请在 45 秒内批准。您无法提高此限制，因为 Desktop 会自行设置 `ServerAliveInterval` 和 `ServerAliveCountMax`，之后 `ssh` 会关闭远程计算机在这么长时间内未发送任何内容的登录。如果是跳板机或 `ProxyCommand` 在等待您，请在远程计算机的条目中提高 `ConnectTimeout` 以涵盖该时间。

##### 修复仅由 Desktop 应用的 `Match host` 块

如果 Desktop 使用了终端中 `ssh devbox` 不使用的设置，请查找条目 `HostName` 行上方的 `Match host` 块。在终端中，`ssh` 会在 `HostName` 行之前读取该处的块，因此它将该块与 `devbox` 进行比较。Desktop 在命令行中向 `ssh` 提供 `HostName` 值，因此 `ssh` 会将该块与该值进行比较。

若要检查，请像 Desktop 那样，在您计算机上的终端中运行 `ssh -G`，并在命令行中提供条目的 `HostName` 和 `User` 值：

```bash theme={null}
ssh -G -o HostName=devbox.internal.example.com -l alice devbox
```

将输出与 `ssh -G devbox` 的输出进行比较。如果该块的设置（例如 `proxycommand`）仅出现在第一个输出中，则该块仅在 Desktop 中生效。若要让 Desktop 也跳过它，请在其 `Match` 行中添加 `!originalhost devbox`：

```text ~/.ssh/config theme={null}
Match host *.internal.example.com !originalhost devbox
```

若要确认，请再次运行第一个命令。该块的设置将不再出现在其输出中。

##### Desktop 保持关闭的转发设置

在与远程计算机的连接上，Desktop 会保持 `ForwardAgent`、`ForwardX11`、`LocalForward`、`RemoteForward`、`DynamicForward` 和 `Tunnel` 关闭，即使您的 SSH 配置启用了它们。在[终端](#run-commands-in-the-terminal)或 `!` 命令中，您的转发设置[只在某些情况下](#limits-while-desktop-is-disconnected)生效，因为 Desktop 可能会在那里启动一个遵循这些设置的单独 `ssh`。

* **若要访问远程计算机上的端口**：[自行转发](#reach-a-port-on-the-remote-machine)
* **若要在远程计算机上使用您自己的密钥**：在远程计算机本身上设置凭据，例如用于 `git push`

若要防止 Desktop 转发您的 SSH agent 或端口，请在您计算机上的终端中运行 `ssh -G devbox`。然后按照 Desktop 可能的运行方式再次运行它，使用它输出的 `hostname` 和 `user` 值，如 `ssh -G -o HostName=devbox.internal.example.com -l alice devbox`。第二个输出可能会显示第一个输出跳过的 `Match` 块中的设置。

在两个输出中查找值不是 `no` 的 `forwardagent` 行，以及以 `localforward`、`remoteforward` 或 `dynamicforward` 开头的行。如果找到，请从您的 SSH 配置中删除该设置。对于您的 `ProxyCommand` 使用 `ssh` 访问的主机，也执行同样的操作。

#### 访问远程计算机上的端口

若要打开在远程计算机上运行的服务器（例如端口 3000 上的开发服务器），请自行转发端口。在您计算机上的终端中，使用 `-L` 和您连接的主机（例如 `devbox`）运行 `ssh`：

```bash theme={null}
ssh -N -L 3000:localhost:3000 devbox
```

如果您在连接上填写了 **SSH port** 或 **SSH key (optional)**，请添加带有相同值的 `-p` 或 `-i`。保持该命令运行，然后在您计算机上的浏览器中打开 `http://localhost:3000` 以访问远程计算机的端口 3000。如果 `ssh` 输出 `open failed: administratively prohibited`，请让该计算机的管理员允许 TCP 转发，这由 SSH 服务器 `sshd_config` 中的 `AllowTcpForwarding` 控制。

#### Windows 远程计算机

Windows 远程计算机需要 Claude Desktop v2.31226.0 或更高版本。

请使用 Microsoft 的 OpenSSH Server。该计算机可以是 x64 或 ARM64。SSH 服务器启动的 shell 可以是 `cmd.exe`、Windows PowerShell 5.1 或 Git Bash。

Windows 远程计算机的测试少于 Linux 远程计算机。在依赖它之前，请先在其上试运行您自己的工作。

在 Windows 远程计算机上，预计会有以下限制（包括但不限于）：

* **终端**：在文件夹为 Windows 路径（例如 `C:/projects/app`）的会话中，[终端](#run-commands-in-the-terminal)不会打开
* **沙箱**：Claude Code 在那里以原生 Windows 方式运行，在此环境下它运行命令时不使用沙箱。请参阅[沙箱在何处运行](https://code.claude.com/docs/zh-CN/sandboxing.md)。
* **插件**：如果插件包含 Windows 无法存储的文件或文件夹名称（例如含有 `:` 的名称），Desktop 不会复制该插件。会话会列出 Desktop 遗漏的插件。
* **MCP 服务器**：在某些 Windows 远程计算机上，Claude 对来自 `claude_desktop_config.json` 的 MCP 服务器的调用会经过 Desktop，因此 [Desktop 断开连接时](#limits-while-desktop-is-disconnected) Claude 无法调用其工具。在 Claude Desktop v2.37337.0 之前，这适用于所有 Windows 远程计算机。
* **Worktree**：worktree 位于 `<project-root>/.claude/worktrees/`。如果您[选择其他文件夹](#choose-where-ssh-session-worktrees-go)，Desktop 会忽略它。
* **早期消息**：如果 SSH 服务器的 shell 是 `cmd.exe` 或 Windows PowerShell，您可能需要先连接才能看到会话的早期消息
* **分叉**：如果 SSH 服务器的 shell 是 `cmd.exe` 或 Windows PowerShell，您只能在会话的第一条消息处分叉会话
* **断开连接时运行**：只有当 SSH 服务器允许程序在启动它的连接结束后继续存在时，任务才会[在 Desktop 断开连接时继续运行](#keep-a-task-running-while-desktop-is-disconnected)。若要确认，请[测试远程计算机](#prepare-a-long-task)。
* **残留进程**：会话结束后，请检查计算机上是否有 Claude Code 启动的程序，因为它们可能会继续运行。Desktop 安装的[后台辅助程序](#keep-a-task-running-while-desktop-is-disconnected)在那里不会自行停止。

#### 在 Desktop 断开连接时保持任务运行

如果您的计算机进入睡眠、网络中断或您退出 Desktop，任务会继续进行，直到完成、需要您处理或达到某项[限制](#limits-while-desktop-is-disconnected)。合上笔记本盖也算作睡眠。Desktop 或您计算机的重启（包括因更新而重启）算作退出。

Claude Code 作为进程在远程计算机上运行，由 Desktop 在那里安装的后台辅助程序管理，因此它无需您的 SSH 连接即可继续运行。本节所述行为需要 Claude Desktop v2.31226.0 或更高版本。

> [!WARNING]
>
> 如果您使用 Claude 账户登录 Desktop，请不要指望任务能通宵运行。Desktop 仅在自身运行且保持连接时才会为任务续期该登录，因此在您断开连接后，任务的登录可能会过期，随后任务停止。断开连接时剩余的时间不定，可能只有几分钟。
>
> 到目前为止完成的工作会保留在远程计算机上，您回来后可以发送一条消息继续。
>
> 如果任务必须通宵运行：
>
> * [保持您的计算机唤醒](https://code.claude.com/docs/zh-CN/desktop-scheduled-tasks.md#how-scheduled-tasks-run)并在线，保持笔记本盖打开并让 Desktop 运行
> * 对远程计算机以及途中的任何跳板机使用无需提示即可工作的登录方式，例如由您的 SSH agent 持有的密钥。这样，Desktop 可以在连接中断后自行重新连接。
>
> 对于必须在您的计算机睡眠或关机时运行的任务，请改为启动[云端会话](#run-long-running-tasks-in-the-cloud)。

* **睡眠或网络中断后**：如果进程[仍在运行](#limits-while-desktop-is-disconnected)且 Desktop 可以无需提示登录，您会重新获得正在运行的任务。如果无法登录，请[发送消息](#reconnect-after-sleep-or-a-network-loss)。
* **退出之后**：任务会继续运行，但除非 Desktop 能重新接入其进程，否则当会话再次打开时您会失去该任务。退出之前，请查看 [Desktop 何时可以重新接入](#reopen-a-session-after-you-quit-desktop)。如果您失去了任务，请发送消息以继续。

当您回来而任务尚未完成时，请检查是否有 Desktop 无法重新连接的提示、身份验证错误或等待中的请求：

* **Desktop 无法重新连接**：会话会说明原因，也可能会有通知说明。Desktop 可能[需要您登录](#reconnect-after-sleep-or-a-network-loss)。
* **身份验证错误**：[登录令牌或凭据](#limits-while-desktop-is-disconnected)可能已过期。使用 Claude 账户时，请发送消息以继续。使用第三方提供商时，请参阅 [3P 上 Claude Desktop 中的 SSH 远程会话](https://claude.com/docs/third-party/claude-desktop/ssh-remote-sessions)。
* **等待中的请求**：[回应该请求](#handle-requests-that-arrive-while-you-are-away)
* **以上都不是**：发送消息以继续

##### 准备长时间任务

* **先更新**：在开始长时间任务之前更新 Desktop。在 macOS 和 Windows 上，Desktop 可能会自行重启以安装已下载的更新。当 Claude 正在处理 SSH 任务时，该重启会等待。默认情况下，一旦更新已等待 72 小时，如果 Desktop 之后能够[重新加入该任务](#reopen-a-session-after-you-quit-desktop)，重启就可能在任务期间进行。
* **退出前检查**：退出或重启后，您不一定能重新获得正在运行的任务，例如在某些 Desktop 更新之后。有关具体情况，请参阅[退出 Desktop 后重新打开会话](#reopen-a-session-after-you-quit-desktop)。
* **使用旧版本时，保持 Desktop 打开**：在 Windows 上使用 v3.1623.0 之前的 Claude Desktop 时，Desktop 退出或重启后，您无法重新获得正在运行的任务。在 macOS 或 Linux 上使用 v2.42304.0 之前版本的第三方提供商时也是如此。在这两种情况下，请在任务期间保持 Desktop 打开。要查找您的版本，请参阅[检查您的版本](#check-your-version)。
* **减少请求**：选择[让任务更少等待您](#handle-requests-that-arrive-while-you-are-away)的权限模式、规则或 hook
* **检查跳板机**：如果您的跳板机要求登录（例如向手机发送推送），请改用无需提示即可工作的登录方式。否则，Desktop 每次尝试重新连接时都可能发送另一条推送。
* **测试远程计算机**：启动一个短任务，在任务仍在运行时退出 Desktop，并在任务应该完成很久之后再次打开会话。如果任务尚未完成且会话未显示身份验证错误，请检查远程计算机上[可能结束任务的因素](#limits-while-desktop-is-disconnected)。
* **不要在 Desktop 的终端中运行命令**：如果远程计算机会[注销空闲终端](#limits-while-desktop-is-disconnected)，请从 Desktop 之外的 SSH 登录运行您自己的命令

##### Desktop 断开连接时的限制

您让其继续运行的任务可能会在其登录过期时、远程计算机结束它时，或辅助程序停止或被替换时结束：

* **登录令牌或凭据**：使用 Claude 账户时，令牌过期后任务会因身份验证错误而停止。使用第三方提供商时，会过期的凭据（例如来自单点登录或凭据辅助程序的凭据）同样如此。请参阅 [3P 上 Claude Desktop 中的 SSH 远程会话](https://claude.com/docs/third-party/claude-desktop/ssh-remote-sessions)。使用 Claude 账户时，请[保持 Desktop 运行并连接](#keep-a-task-running-while-desktop-is-disconnected)，以便它能续期登录。
* **远程计算机**：如果远程计算机重启，或在您的 SSH 连接关闭时结束您的进程，正在运行的任务就会结束。在 Desktop 断开连接期间，不会有任何东西重新启动它。systemd-logind 的 `KillUserProcesses` 为 `yes` 且适用于您用户的 Linux 计算机会以这种方式结束您的进程。若要在这类计算机上保持任务运行，请让管理员豁免您的用户，例如使用 `KillExcludeUsers`。
* **空闲终端**：某些 Linux 远程计算机会注销在设定时间内无人输入的终端（systemd-logind 的 `StopIdleSessionSec`）。在 macOS 或 Linux 上的 Desktop 中，会话的[终端](#run-commands-in-the-terminal)和 `!` 命令默认共享 Desktop 的 SSH 连接。因此，该注销会在 Desktop 连接期间结束连接，正在运行的任务也可能随之结束。关闭终端窗格并不能防止这种情况，因此请[不要在该终端中运行命令](#prepare-a-long-task)。
* **Desktop 更新**：如果更新带来了新的辅助程序，Desktop 会在之后的某次连接时替换远程计算机上的辅助程序，这可能会结束仍在运行的任务
* **长时间离开**：在 Linux 或 macOS 远程计算机上，如果 Desktop 30 天内未连接，辅助程序会停止，并结束在其下运行的所有 Claude Code 进程。请在此之前连接。

在 Desktop 断开连接期间，文件编辑和某些工具可能会失败。如果任务需要它们，请保持 Desktop 连接：

* **Worktree**：在使用 [worktree](#run-a-session-in-its-own-worktree) 的会话中，Desktop 会在 Claude 进行每次文件编辑之前对其进行检查。在 Desktop 断开连接期间，每次编辑会等待约 30 秒然后失败。Shell 命令仍会运行。在 Linux 或 macOS 远程机器上，[编辑可以继续正常进行](#worktree-edits-while-disconnected)。如果您不确定，请在不使用 worktree 的情况下启动会话。
* **文件夹限制**：如果您的组织限制了会话可以使用的文件夹，无论是否使用 worktree，Desktop 都会检查每次文件读取、搜索和编辑。在 Desktop 断开连接期间，每项操作最多等待 10 分钟，然后失败。
* **连接器**：[连接器](#connect-external-tools)和 Desktop 提供的其他工具通过 Desktop 运行，因此在其断开连接期间，每次调用都会在约 30 秒后失败
* **工作流**：Desktop 会在每个新[工作流](https://code.claude.com/docs/zh-CN/workflows.md)启动前对其进行检查，因此在 Desktop 断开连接期间，新工作流最多等待 10 分钟，然后失败
* **MCP 服务器**：Claude Code 在远程计算机上启动的 MCP 服务器不通过 Desktop 运行，因此 Claude 仍可调用其工具。仅当 Desktop 未使用其[内置 SSH 客户端](#worktree-edits-while-disconnected)时，来自 `claude_desktop_config.json` 的服务器才属于这一类。否则，以及在[某些 Windows 远程计算机](#windows-remote-machines)上，Desktop 断开连接期间 Claude 无法调用其工具。

> [accordion] worktree 中的编辑何时可以继续工作
>
> 在 Linux 或 macOS 远程计算机上，该检查不经过 Desktop，因此在 Desktop 断开连接期间，worktree 内的编辑可以继续工作。在 Claude Desktop v2.37337.0 之前，这仅适用于 Linux 远程计算机。
>
> 在某些情况下编辑仍会失败，包括以下情况：
>
> * **内置 SSH 客户端**：Desktop 使用了它，例如在 Microsoft Store 安装版本中、在没有 Microsoft OpenSSH 9.4 或更高版本的 Windows 计算机上，或在使用第三方提供商的 Windows 上默认使用
> * **仓库位置**：仓库是您的主目录、其上层文件夹或 `~/.claude`
> * **macOS 路径**：在 macOS 远程计算机上，仓库位于 `/Volumes` 下，或者仓库或 worktree 路径中的某个文件夹名称包含 `~` 或非 ASCII 字符
> * **文件夹限制**：您的组织限制了会话可以使用的文件夹
> * **企业启动器**：在 Claude Desktop v2.37337.0 之前，您使用 Claude 账户登录，且您的组织通过[企业启动器](https://code.claude.com/docs/zh-CN/corporate-launcher.md)在远程计算机上启动 Claude Code
>
> 不使用 worktree 的会话没有此类检查，除非您的组织限制了会话可以使用的文件夹。

##### 处理您不在时到达的请求

如果 Claude 在 Desktop 断开连接期间需要您的批准或向您提问，Claude 会在远程计算机上等待。该等待本身没有时间限制。当 Desktop 重新接入进程时，它会向您显示该请求。如果 Desktop 改为启动新进程，您将失去该请求。

如果您回来时发现一个等待您的请求不见了，原因之一是 Desktop 自行重启了，例如为了安装更新。只有当 Desktop 能重新接入进程时，请求才能在重启后保留，因此在让任务保持运行之前，请查看 [Desktop 何时可以重新接入](#reopen-a-session-after-you-quit-desktop)。

如果您失去了请求，请发送消息以继续。

若要减少任务停下来等待您的频率，并在您不在时收到任务的消息，请在离开前设置以下任一项：

* **自动模式**：如果[您可以使用自动模式](#auto-mode-availability)，请选择[自动模式](#choose-a-permission-mode)。在多次被阻止后，[系统会再次询问您](https://code.claude.com/docs/zh-CN/permission-modes.md#when-auto-mode-falls-back)。
* **允许规则**：在远程计算机上的用户设置文件 `~/.claude/settings.json` 中，为任务所需的命令添加[允许规则](https://code.claude.com/docs/zh-CN/permissions.md#manage-permissions)。如果您的组织[仅接受来自托管设置的权限规则](https://code.claude.com/docs/zh-CN/settings-reference.md#allowmanagedpermissionrulesonly)，请让管理员将其添加到能传递到远程计算机的[托管设置](#managed-settings)中。有关项目 `.claude/settings.json` 中的允许规则，请参阅[项目允许规则和工作区信任](https://code.claude.com/docs/zh-CN/permissions.md#project-allow-rules-and-workspace-trust)。
* **自动应答的 hook**：在远程计算机上的 `~/.claude/settings.json` 中配置 [`PermissionRequest` hook](https://code.claude.com/docs/zh-CN/hooks#permissionrequest)，以代替您允许或拒绝请求
* **提醒您的 hook**：在同一文件中使用 `permission_prompt` 匹配器配置 [`Notification` hook](https://code.claude.com/docs/zh-CN/hooks#notification)，以便在有请求等待时向自己发送消息
* **在 Claude 完成时通知您的 hook**：在同一文件中配置 [`Stop` hook](https://code.claude.com/docs/zh-CN/hooks#stop)，以便在 Claude 完成响应时向自己发送消息。当 API 错误结束该轮次时，它不会运行。对于这种情况，请配置 [`StopFailure` hook](https://code.claude.com/docs/zh-CN/hooks#stopfailure)。

如果您的组织[仅运行托管 hook](https://code.claude.com/docs/zh-CN/settings-reference.md#allowmanagedhooksonly)，请让管理员将该 hook 添加到能传递到远程计算机的[托管设置](#managed-settings)中。

Claude Code 在任何权限模式（包括 Bypass permissions）下都不会自动批准[少数操作](https://code.claude.com/docs/zh-CN/permission-modes.md#actions-no-mode-auto-approves)。根据权限模式的不同，Claude Code 会拒绝此类操作或询问您，任务随后会等待您的回答。

##### 在睡眠或网络中断后重新连接

如果您回来时 Desktop 尚未重新连接，请在会话中发送消息。Desktop 仅在能够无需提示登录的情况下才会自行重新连接，例如使用由您的 SSH agent 持有的密钥。

* **记住的密码**：您让 Desktop 记住的密码也可以无需提示登录，但若长时间离开，最好使用由您的 SSH agent 持有的密钥。如果使用该密码的登录中断或挂起，Desktop 可能会停止重新连接并等待您。
* **任何其他提示**：如果远程计算机提示输入其他内容（例如一次性验证码），您可能会在计算机上收到通知。点击该通知或在会话中发送消息，Desktop 就会显示该提示。

> [accordion] 如果跳板机要求登录、中断时间较长、输出缺失或进程已不存在
>
> * **跳板机，您不在时推送停止**：当无人响应跳板机的登录（例如向手机发送的推送）时，macOS 或 Linux 上的 Desktop 可能会出现这种情况。此时，在无人使用您的计算机期间，Desktop 会停止重新连接。当您解锁或使用计算机时，您可能会收到另一条推送，因为 Desktop 可能会再次尝试。在三次无人响应的尝试后，它会等到您重试或发送消息。
> * **跳板机，推送持续不断**：在 Desktop 无法判断无人响应的情况下（例如 Windows 上的 Desktop），只要有可重新接入的进程，无论该进程是否仍在运行，它都会持续尝试。每次尝试您都可能收到另一条推送。若要结束尝试，请退出 Desktop，但这[可能会让您失去正在运行的任务](#reopen-a-session-after-you-quit-desktop)。
> * **长时间中断**：Desktop 会放慢到大约每五分钟尝试一次。若要更快重新连接，请在会话中重试或发送消息，Desktop 会在连接后投递该消息。
> * **输出缺失**：在 Desktop 断开连接期间，辅助程序会在 16 MiB 缓冲区中保留每个会话最近的 Claude Code 输出，并丢弃较早的输出。Desktop 重新连接后，会话可能会显示 `Loading recent output from <host>…`。如果 Desktop 无法加载全部输出，您会看到一行 `Some output produced while you were disconnected may be missing here.`。该行并不表示任务已停止。
> * **进程已不存在**：进程在 Desktop 断开连接期间结束了，例如远程机器重启时。Desktop 随后可以根据对话保存的历史记录启动一个新进程，并让 Claude 继续该任务。如果任务仍处于停止状态，请发送消息。使用第三方提供商时，请更新到 Claude Desktop v2.42304.0 或更高版本，因为较早的版本从不会自行继续任务。

##### 退出 Desktop 后重新打开会话

退出 Desktop 后，任务会在远程机器上继续运行，但除非 Desktop 能重新接入其进程，否则会话再次打开时任务会停止。在 Desktop 关闭期间，任务会一直运行，直到完成、需要您介入或达到某项[限制](#limits-while-desktop-is-disconnected)。

* **Windows 上的 Desktop，任务在 v3.1623.0 之前的版本中启动**：即使 Desktop 此后已更新，您也无法重新获得该任务。请仅在您预计任务已完成时再次启动 Desktop，因为会话可能会在 Desktop 启动时打开，而会话打开时任务会停止。如果您不知道任务是在哪个版本中启动的，也请等到您预计任务已完成时再启动。
* **macOS 或 Linux 上的第三方提供商，任务在 v2.42304.0 之前的版本中启动**：您无法重新获得该任务。请仅在您预计任务已完成时再次启动 Desktop。
* **任何其他任务**：启动 Desktop 并打开会话，如果 Desktop 重新加入，任务就会继续。它并不总能重新加入，例如在某些 Desktop 更新之后，因此请[在长时间任务之前更新](#prepare-a-long-task)。

当任务停止时，会话可能会在 Desktop 启动新进程期间显示 `Restarting Claude Code on <host>…`。您还会失去任何正在等待您的批准或问题。如果 Claude 没有自行继续，请发送消息，Claude 会根据对话保存的历史记录继续。

有关第三方部署，请参阅 [3P 上 Claude Desktop 中的 SSH 远程会话](https://claude.com/docs/third-party/claude-desktop/ssh-remote-sessions)。

> [!WARNING]
>
> 当对话的 Claude Code 进程仍在运行时，请勿在远程计算机上为该对话运行 [`claude --resume`](https://code.claude.com/docs/zh-CN/cli-reference#cli-flags)。Claude Code 不会阻止您，此时两个进程会写入同一个对话。如果您不确定，请添加 `--fork-session`，它会在对话的副本中继续，但两个进程仍会处理相同的文件。

#### 选择 SSH 会话 worktree 的位置

除非您的组织限制了会话可使用的文件夹，否则新的 SSH 会话会在以下已设置位置中的第一个位置创建其 [worktree](#run-a-session-in-its-own-worktree)：

1. SSH 连接上的 **Worktree folder**
2. 远程计算机上 `~/.claude/settings.json` 中的 [`worktree.location`](https://code.claude.com/docs/zh-CN/settings-reference#worktree-location)
3. `<project-root>/.claude/worktrees/`，即默认位置

每个项目都会在您设置的文件夹中拥有自己的子文件夹，因此如果设置为 `~/worktrees`，worktree 的路径为 `~/worktrees/<project>-<id>/<worktree-name>`。如果您设置的文件夹位于项目内部，Desktop 会对该项目忽略它并使用默认位置。在 [Windows 远程计算机](#windows-remote-machines)上，Desktop 始终使用默认位置。

##### 在现有连接上设置 worktree 文件夹

在您之前添加的连接或您组织管理的连接的设置中设置 **Worktree folder**。该字段需要 Claude Desktop v1.44121.0 或更高版本。如果您的组织限制了会话可使用的文件夹，Desktop 会隐藏该字段并将 worktree 保留在项目内部。

**1. 找到连接**

打开[环境选择器](#start-a-session)，在 **SSH** 下找到该连接。

**2. 打开连接的设置**

将鼠标悬停在连接上并点击齿轮图标。如果打开的对话框在 **Host** 旁边显示 **Edit connection…**，请选择 **Edit connection…**。

**3. 设置文件夹**

在 **Worktree folder** 中，输入远程机器上的一个文件夹，例如 `~/worktrees`，然后选择 **Save changes**。

#### 远程计算机上的文件和进程

Desktop 会在连接到 Linux 或 macOS 远程计算机的每个用户的主目录下的 `~/.claude/remote/` 中安装一个辅助程序和 Claude Code。辅助程序不监听任何网络端口，但 Claude Code 及其启动的程序（例如 MCP 服务器和开发服务器）可能会监听。

| 文件夹 | 内容 |
| - | - |
| `~/.claude/remote/srv/` | 辅助程序，每个版本一个文件夹 |
| `~/.claude/remote/ccd-cli/` | Claude Code，每个版本一个文件 |
| `~/.claude/remote/run/` | 辅助程序的运行时文件（例如其套接字和日志），每台连接的计算机一个文件夹 |
| `~/.claude/remote/plugins/` | Desktop 从用户计算机复制的插件 |
| [`~/.claude/uploads/`](https://code.claude.com/docs/zh-CN/claude-directory#clear-local-data) | 用户附加到消息中的文件，每个会话一个文件夹 |

若要列出某个用户的进程，请以该用户身份在远程计算机上运行 `pgrep -fl "$HOME/.claude/remote/"`，或用该用户的主目录替换 `$HOME` 后运行：

* **辅助程序**：每台连接的计算机对应一个或多个 `server` 进程
* **Claude Code**：辅助程序为该用户的会话启动的进程，每个进程都来自以其版本命名的文件，例如 `2.1.295-…`

Desktop 通过用户的 SSH 连接，经由 `~/.claude/remote/run/` 下辅助程序文件夹中的 Unix 套接字 `rpc.sock` 访问辅助程序。除 root 外，该计算机上的其他用户无法连接到该套接字，因为其模式为 600，且其所在文件夹的模式为 700。

有关辅助程序何时会自行停止，请参阅 [Desktop 断开连接时的限制](#limits-while-desktop-is-disconnected)。

若要为某个用户移除辅助程序和 Claude Code：

**1. 退出 Desktop**

在该用户用于连接的每台计算机上退出 Desktop。Desktop 下次连接时会重新安装这两个程序，并且它可能会自行重新连接。

**2. 结束进程**

以该用户身份登录远程计算机，对 [`pgrep`](#files-and-processes-on-the-remote-machine) 列出的每个 Claude Code 进程的 ID 运行 `kill`。待这些进程从列表中消失后，再对每个 `server` 进程的 ID 运行 `kill`。如果您先结束 `server` 进程，Claude Code 正在运行的命令可能会继续运行。

然后列出该用户的剩余进程（例如使用 `ps -u "$(id -un)" -f`），并结束您识别出由 Claude Code 启动的任何进程。它留在后台的程序可能会在它结束后继续运行，且不在 `pgrep` 列表中。

**3. 删除文件夹**

删除 `~/.claude/remote/`。若要移除 Claude Code 自己在 `~/.claude/` 和 `~/.claude.json` 中的文件，请参阅[移除配置文件](https://code.claude.com/docs/zh-CN/setup.md#remove-configuration-files)。

##### 结束单个会话的进程

要结束单个会话的 Claude Code 进程，请在 Desktop 连接到远程机器时，在 Desktop 中[归档该会话](#archive-a-session)。该进程在其任务完成后以及您退出 Desktop 后都会继续运行。

在归档 worktree 中有未提交更改的会话之前，请参阅[在会话被归档时保留未提交的更改](#keep-uncommitted-changes-when-a-session-is-archived)。

##### 查看进程以哪个用户身份运行

辅助程序和 Claude Code 以 Desktop 通过 SSH 登录所用的用户身份运行。安装和启动它们不需要 `sudo`。若要检查用户，请在远程计算机上使用 [`pgrep`](#files-and-processes-on-the-remote-machine) 列出的进程 ID 运行 `ps -o user= -p`。

Claude Code 启动的程序（例如 shell 命令和 MCP 服务器）也以该用户身份启动。请像审查在那里运行命令的人员一样，审查该用户在远程计算机上的权限，包括其 `sudo` 规则。

##### 查看谁连接了远程计算机

若要了解谁连接了远程计算机，请读取该计算机上 SSH 服务器的日志，其中来自 Desktop 的登录与来自终端的登录看起来相同。在 macOS 或 Linux 上的 Desktop 中，每次 Desktop 连接都会产生一条记录，而不是每个会话一条。

如果服务器设置了 `MaxSessions 1`，则每次 Desktop 连接或重新连接时至少会有两条记录。每个终端标签页，以及每个运行以 `!` 开头的命令的 Desktop 会话，还会至少再产生一条记录。

您不会在 Claude Code 的[指标和事件](https://code.claude.com/docs/zh-CN/monitoring-usage.md#available-metrics-and-events)中找到 Desktop 的 SSH 连接。要从中了解某个 SSH 会话在哪台远程机器上运行，请参阅[将遥测归因于 Desktop SSH 会话](https://code.claude.com/docs/zh-CN/monitoring-usage.md#attribute-telemetry-to-desktop-ssh-sessions)。有关 Desktop 自身的连接记录，请参阅[获取 Desktop 针对 SSH 问题的日志](#get-desktops-logs-for-an-ssh-problem)。

#### 在登录前关闭的连接

如果 SSH 服务器的日志显示有在任何登录之前就关闭的连接，它们可能是 Desktop 对 SSH 端口的测试。Desktop 会测试机器是否响应，例如在与其连接失败之后。它会打开到该端口的连接并立即关闭，不发送任何内容。预计会有一次或连续多次测试，例如约两分钟内八次。

如果没有任何机制因此类连接而封禁或拒绝地址，您无需对 Desktop 的测试采取任何措施。如果 Desktop 连接所用的地址被拒绝或封禁，请检查 [SSH 服务器本身是否拒绝该地址](#check-whether-the-ssh-server-itself-refuses-the-address)以及 [fail2ban 是否统计 Desktop 的测试](#check-whether-fail2ban-counts-desktops-tests)。

##### 在 SSH 服务器日志中识别测试

将您日志中的行与 Linux 上的 OpenSSH 服务器在默认 `LogLevel` 下为测试写入的行进行比较，请参阅与您服务器版本对应的行。如果服务器的问候消息在 Desktop 关闭连接之前到达，您看到的可能是最后一列中的行：

| OpenSSH 版本 | 测试产生的行 | 问候消息先到达时的行 |
| - | - | - |
| 6.6 至 7.9 | 一行包含 `Did not receive identification string from 203.0.113.7` 的内容 | 相同的行 |
| 8.0 至 8.2 | `error: kex_exchange_identification: Connection closed by remote host` | `error: kex_exchange_identification: read: Connection reset by peer` |
| 8.3 至 9.4 | `error: kex_exchange_identification: Connection closed by remote host`，然后是 `Connection closed by 203.0.113.7 port 51234` | `error: kex_exchange_identification: read: Connection reset by peer`，然后是 `Connection reset by 203.0.113.7 port 51234` |
| 9.5 至 10.2 | `Connection closed by 203.0.113.7 port 51234` | `error: kex_exchange_identification: read: Connection reset by peer`，然后是 `Connection reset by 203.0.113.7 port 51234` |
| 10.3 至 10.5 | `Connection closed by 203.0.113.7 port 51234 [preauth]` | `error: kex_exchange_identification: read: Connection reset by peer [preauth]`，然后是 `Connection reset by 203.0.113.7 port 51234 [preauth]` |

其他打开端口后又关闭的程序也可能留下相同的行。当某行包含地址时，请检查它是否是装有 Desktop 的计算机连接所用的地址。

8.0 至 8.2 的行不包含地址。要获取地址，请在 `sshd_config` 中设置 `LogLevel VERBOSE`，并以 root 身份运行 `sshd -T | grep -i loglevel`。如果它输出 `loglevel VERBOSE`，请重新加载 SSH 服务器。然后在测试的行之前查找包含 `Connection from 203.0.113.7 port 51234` 的行。

##### 检查 SSH 服务器本身是否拒绝该地址

如果来自许多装有 Desktop 的计算机共用的某个地址的连接被拒绝，请在 SSH 服务器的日志中查找以如下内容结尾的行：

```text theme={null}
drop connection #0 from [203.0.113.7]:51234 on [192.0.2.10]:22 penalty: connections without attempting authentication
```

在默认 `LogLevel` 下，OpenSSH 9.8 及更高版本在因连接未尝试登录即关闭（正如 Desktop 的测试那样）而拒绝某个地址时，可能会写入这样的行。在默认设置下，仅凭几次测试不会出现这样的行。

要防止拒绝，请在 `sshd_config` 中将 `PerSourcePenaltyExemptList` 设置为人们连接所用的地址，例如 `PerSourcePenaltyExemptList 203.0.113.7`。之后服务器不会对这些地址施加任何惩罚，包括针对登录失败的惩罚。要检查该文件，请以 root 身份运行 `sshd -T | grep -i persourcepenalt`。如果它输出了您的列表，请重新加载 SSH 服务器。

##### 检查 fail2ban 是否统计 Desktop 的测试

如果 fail2ban 0.10.5 或更高版本封禁了 Desktop 连接所用的地址，请在 `jail.conf`、`jail.local` 或 `jail.d/` 下找到您 jail 的模式：

* **在方括号中指定模式的 `filter` 行**，例如 `filter = sshd[mode=aggressive]`：使用该模式。
* **仅有带 `[mode=%(mode)s]` 的默认 `filter` 行**：使用 `mode` 行中的值，如果没有该行则为 `normal`。

如果模式为 `normal`，fail2ban 不会统计 Desktop 测试产生的行，因此请查找封禁的其他原因。

对于其他模式，请将您自己日志中的行复制到远程机器上的一个文件中。完整复制每一行（从日期到行尾），如果某次测试留下两行，则两行都要复制。在那里使用该文件和您 jail 的模式（例如 `aggressive`）运行 `fail2ban-regex`：

```bash theme={null}
fail2ban-regex -o ip lines.log "sshd[mode=aggressive]"
```

对于 fail2ban 统计的每次失败，该命令都会在单独一行输出地址，例如 `203.0.113.7`。在这种情况下，请[防止封禁](#prevent-a-ban-by-fail2ban)。如果它没有输出任何内容，则 fail2ban 在该模式下不会统计这些行，因此请查找封禁的其他原因。

##### 防止 fail2ban 封禁

要防止 fail2ban 因 Desktop 的测试而封禁地址，请更改 jail 的两项设置之一，每项都有代价：

* **`mode`**：在您的 jail 设置它的任何位置（`mode` 行中或诸如 `filter = sshd[mode=aggressive]` 的行的方括号中）将其设置为 `normal`。您会放弃只有其他模式才会进行的所有统计，例如对连接后未尝试登录即离开的客户端的统计。
* **`ignoreip`**：将其设置为人们连接所用的地址，例如 `ignoreip = 203.0.113.7`。之后 fail2ban 不会统计来自这些地址的任何内容，包括登录失败。

#### 使用在远程计算机上设置的 MCP 服务器

若要在 SSH 会话中使用来自 `~/.claude.json` 或项目 `.mcp.json` 的 [MCP 服务器](https://code.claude.com/docs/zh-CN/mcp.md)，请在远程计算机上的这些文件中进行设置。Claude Code 会在那里启动其 stdio 服务器，因此请在远程计算机上安装这些服务器运行的命令。有关来自 `claude_desktop_config.json` 的服务器，请参阅 [Desktop 断开连接时的限制](#limits-while-desktop-is-disconnected)。

#### 通过链接打开 SSH 会话

`claude://code/new` 链接会打开 Desktop 的新会话页面，并且可以指定 SSH 连接。将此类链接放在运维手册、仪表板或 wiki 页面中，即可打开已为正确的计算机和文件夹设置好的 Desktop。对于会去除此类链接的平台，请参阅[链接显示为纯文本而不是可点击链接](https://code.claude.com/docs/zh-CN/deep-links.md#the-link-renders-as-plain-text-instead-of-being-clickable)。

SSH 链接需要 Claude Desktop v2.110.0 或更高版本。

以下链接指定了 `build.example.com` 上的用户 `dev`、端口 2222 和文件夹 `/srv/payments`，并填入了一条提示词：

```text theme={null}
claude://code/new?ssh_host=dev%40build.example.com&ssh_port=2222&ssh_folder=/srv/payments&q=Investigate%20the%20failed%20deploy
```

SSH 链接接受以下参数，只有 `ssh_host` 是必需的：

| 参数 | 值 |
| :- | :- |
| `ssh_host` | `host` 或 `user@host`，写法与 **SSH host** 字段中相同。该值不能以 `-` 开头，且主机部分只能包含字母、数字、`.`、`_`、`:` 和 `-` |
| `ssh_port` | 1 到 65535 之间的端口号 |
| `ssh_folder` | 远程计算机上的文件夹。以 `/` 或 `~/` 开头，或使用 `~` |
| `q` | 用于输入框的 URL 编码文本 |

来自 `~/.ssh/config` 的别名只有对拥有该条目的人才能作为 `ssh_host` 使用。若要匹配用户已有的连接，请为链接提供与该连接相同的用户、主机和端口。

打开链接时，Desktop 会在选择连接之前要求您确认：

* **您已有的连接**：如果主机、用户和端口与您的某个连接匹配，Desktop 会询问是否使用它，并向您显示该连接的名称和主机，如果链接指定了文件夹，还会显示文件夹。
* **新连接**：否则，Desktop 会打开添加 SSH 连接的对话框。当您添加连接时，Desktop 会在保存任何内容之前询问是否连接，并向您显示链接中的主机，如果链接指定了端口和文件夹，还会显示它们。

在您确认之前，Desktop 不会保存链接中的主机、端口或文件夹，也不会使用它们选择或打开连接。如果已选择了某个 SSH 连接，新会话页面仍可能像没有链接时一样自行连接到该连接，即使链接指定的是同一主机。链接不能携带密钥文件、密码或命令。

任何人都可以编写链接，因此请检查它填入了什么：

* **确认之前**：检查主机和文件夹。
* **发送之前**：检查提示词和所选环境。

Desktop 会在链接打开时填入提示词，替换您尚未发送的任何文本，并且从不替您发送。它将提示词视为纯文本，因此开头的 `/` 或 `!` 以及 `@` 文件提及不会作为命令或提及生效。如果您取消，提示词会保留在输入框中，且您之前选择的环境不会改变。

链接不会绕过 [`sshHostAllowlist`](#restrict-which-ssh-hosts-users-can-connect-to)。Desktop 会在连接时检查允许列表。

如果链接打开了 Desktop 但没有出现有关连接的对话框，请查找以下原因之一：

* **您已退出登录**：登录后再次打开链接。
* **另一个对话框处于打开状态**：关闭它，然后再次打开链接。
* **链接无效**：Desktop 会显示一条消息说明需要修复的内容，并且不会填入提示词。
* **Desktop 版本早于 v2.110.0**：较早的版本会忽略 SSH 参数，仅带着提示词打开新会话页面。
* **SSH 会话已关闭**：如果您的管理员将允许列表设置为空数组，Desktop 会拒绝 SSH 链接。

#### 通过命令创建 SSH 连接

如果某个命令行工具可以按需创建您的远程机器，请将其命令添加到 [`~/.claude/settings.json`](https://code.claude.com/docs/zh-CN/settings#settings-files) 中的 `sshHostCommands`。随后，环境选择器会列出一个以您的条目命名的项，例如 **Create Dev box**。选择该项时，Desktop 会在您的计算机上运行该命令，将命令输出的主机保存为 SSH 连接，然后进行连接。

> [!NOTE]
>
> 通过命令创建的连接需要在 macOS 或 Linux 上使用 Claude Desktop v3.6556.0 或更高版本，并使用 Claude 账户登录。管理员可以[将命令分发给团队](#distribute-commands-to-your-team)。

Desktop 仅在您创建连接或[创建另一台机器](#create-another-machine)时运行该命令，而不是每次连接时都运行。每个条目对应一个连接，指向 Desktop 最近一次从该条目的命令中保存的主机。

> [!WARNING]
>
> Desktop 不会停止或删除机器。以下每一项都会持续运行，并可能持续产生费用，直到您使用自己的工具将其删除：
>
> * 在您[创建另一台](#create-another-machine)之前某个条目所指向的机器
> * 您从文件中移除的条目或更改了 `id` 的条目所对应的机器
> * 您的命令创建了但 Desktop 未保存其主机的机器，例如以下情况：
>   * 命令失败或超时
>   * Desktop 无法使用输出的主机、另一个连接已在使用该主机，或您的组织不允许该主机
>   * 在命令运行期间，您取消了命令、移除了其条目、更改了其 `command` 或 `id`，或退出了 Desktop

以下示例添加一个名为 Dev box 的连接。如果该文件不存在，请创建它。如果文件中已有设置，请将该键添加在它们旁边。命令 `acme-dev create` 代表您自己的工具，`~/project` 是 Desktop 在您的命令创建的每台机器上选择的文件夹：

```json ~/.claude/settings.json theme={null}
{
  "sshHostCommands": [
    {
      "id": "devbox",
      "name": "Dev box",
      "command": "acme-dev create",
      "startDirectory": "~/project"
    }
  ]
}
```

每个条目接受以下字段，其中 `id`、`name` 和 `command` 是必需的：

| 字段 | 值 |
| :- | :- |
| `id` | 最多 64 个 ASCII 字母、数字、`.`、`_` 和 `-`，以字母或数字开头。使用 `sshHostCommands` 或 [`sshConfigs`](#pre-configure-ssh-connections-for-your-team) 中其他条目都未使用的 `id`。Desktop 以它为键保存连接的主机，因此如果您更改它，该连接会从无主机状态重新开始，其机器会继续运行，且 Desktop [仍可连接到它们](#remove-an-entry)。 |
| `name` | 连接在环境选择器中的名称，最多 200 个字符，且为单行 |
| `command` | 一个程序及其参数，如[编写命令](#write-the-command)所述 |
| `sshIdentityFile` | 您的私钥路径，例如 `~/.ssh/id_ed25519`。省略此项则使用您的 SSH 配置或 SSH agent。 |
| `startDirectory` | 远程机器上的默认文件夹。以 `/` 或 `~/` 开头，或使用 `~`。 |

要更改或移除该连接，请在文件中编辑其条目，因为应用的 SSH 连接对话框不会为它打开。编辑文件后无需重启 Desktop。如果该条目没有显示或无法选择，请参阅[如果命令缺失或显示为灰色](#if-a-command-is-missing-or-gray)。

##### 编写命令

Desktop 直接启动程序，不经过 shell，也没有终端：

* **程序**：写出其名称，或从 `/` 开始的完整路径，不含空格。对于裸名称，Desktop 会在您的登录 shell 导出的 `PATH` 中查找。相对路径（例如 `./acme-dev`）、以 `~` 开头的路径以及 shell 别名和函数都不起作用。
* **参数**：用空格分隔普通参数。命令只能包含空格、ASCII 字母、数字以及字符 `+,-./:=@_~`，并且任何单词都不能以 `~` 开头。
* **脚本**：如果命令需要其他内容，例如引号、管道、变量或包含 `?` 或 `&` 的 URL，请将其放入脚本中。使脚本可执行，以 `#!` 行开头，并将 `command` 设置为其完整路径，例如 `/Users/you/bin/make-devbox`。在 macOS 上，请勿将脚本放在 Documents、Desktop 和 Downloads 文件夹中，macOS 可能会阻止其运行。
* **输入**：程序没有输入，也没有终端，因此无法在那里提示您。如果您的工具通过浏览器或提示让您登录，请先在终端中登录，并在每次登录失效时重新登录。
* **环境**：程序在您的主目录中运行，使用您的登录 shell 导出的变量。Desktop 在启动时读取这些变量，与[`ProxyCommand`](#use-variables-from-your-shell-in-a-proxycommand) 的处理方式相同，因此更改变量后请退出并重新打开 Desktop。
* **时间限制**：程序运行超过 10 分钟仍未结束时，Desktop 会将其停止。Desktop 的进度消息不会显示程序的输出。要更早停止程序，请选择该消息旁边的 **Cancel**。

程序必须以代码 0 退出，并且只在标准输出上打印主机，格式为以下两种之一：

* **一行**：`host` 或 `user@host`，写法与 [SSH 链接](#open-an-ssh-session-from-a-link)中的 `ssh_host` 相同。要指定端口，请使用 JSON 格式。
* **一个 JSON 对象**：`sshHost`，以及可选的数字 `sshPort` 和用于替换条目中对应值的 `startDirectory`，例如 `{"sshHost": "dev@devbox-1234.example.com", "sshPort": 2222}`

要查看 Desktop 读取的输出，请在终端中以无输入方式运行命令，并将标准输出写入文件：

```bash theme={null}
acme-dev create < /dev/null > host.txt
```

该文件必须只包含主机。运行此测试时，您的工具也会创建一台机器，因此之后请使用您的工具将其删除。

如果您的工具在主机之前打印其他行（例如进度消息），请用一个只打印主机的脚本将其包装起来。以下脚本打印您的工具输出的最后一行。如果您的工具失败，脚本会以相同的代码退出，以便 Desktop 报告该失败：

```bash theme={null}
#!/bin/bash
out=$(mktemp) || exit
trap 'rm -f "$out"' EXIT
acme-dev create > "$out" || exit
tail -n 1 "$out"
```

如果命令失败，Desktop 消息中的 **View details** 会显示命令写入标准错误的最后一行（如果有）。

##### 连接到新主机

首次连接到每个新主机时，需要回答两个问题：

* **主机密钥**：在 OpenSSH 的默认主机密钥检查下，检查密钥的指纹，然后选择是否连接。要跳过此问题，请让您的命令在打印主机之前，使用您的工具在创建机器时获得的密钥，将其添加到 `~/.ssh/known_hosts`。对于 22 以外的端口，该行以 `[host]:port` 开头。如果您的工具会重复使用主机名，也请删除旧密钥所在的行。
* **文件夹**：选择是否信任您在那里打开的文件夹

请为这些主机保持启用主机密钥检查，并为它们保留 `known_hosts` 文件。如果 `StrictHostKeyChecking` 为 `no` 或 `accept-new`，且 `UserKnownHostsFile` 也为 `/dev/null` 或 `none`，则不会记录任何密钥。之后当您的工具重复使用某个名称时，您可能会在没有任何警告的情况下连接到另一台机器。

即使计算机前没有人，复用的名称也会造成影响，因为 Desktop 会自行连接到这些主机。例如，它会在主机恢复响应后，投递您在主机无法访问时发送的消息。即使开启了主机密钥检查，在以下情况下该消息也可能送达使用该名称的新机器，因此请在删除机器之前[停止使用该机器](#stop-using-a-machine)：

* 您、您的命令或其他程序将新密钥添加到了 `known_hosts`，或移除了旧密钥所在的行，正如您的命令在添加主机密钥时所做的那样
* 您工具创建的机器共享同一个主机密钥
* 您工具创建的机器共享一个受 `known_hosts` 信任的证书颁发机构
* 您或您的管理员设置了 `KnownHostsCommand` 或 `VerifyHostKeyDNS yes`，使 SSH 接受来自 `known_hosts` 之外的密钥

##### 创建另一台机器

选择 **Create another Dev box** 后，您在之前机器上的会话仍可继续使用，新会话则使用新机器。在环境选择器中选择该项，可以创建一台全新的机器，或替换一台已不存在的机器。在新建会话页面上，提示 Desktop 无法访问主机的消息中也提供该选项。

* **之前机器上的会话**：从侧边栏继续这些会话，因为选择器只列出 Dev box 当前的机器，而不列出之前的机器。Desktop 可能会自行连接到它们的机器，例如在 Desktop 启动时，因此用完某台机器后请[停止使用该机器](#stop-using-a-machine)。
* **在 Claude Desktop v4.4212.0 之前替换的主机**：这些版本显示 **Replace Dev box's host**，并停止连接到被替换的主机。要在该主机上继续会话，请打开会话，选择 **Add SSH connection**，然后保存 Desktop 填入的连接。在替换另一个主机之前请先更新 Desktop。
* **在两台机器上新建会话**：向 [`sshHostCommands`](#create-an-ssh-connection-from-a-command) 添加第二个条目
* **尚未响应的新主机**：在新建会话页面持续尝试连接期间，将其保持打开最多 10 分钟。如果主机仍无响应，请在页面的消息中选择 **Create another Dev box**。
* **紧接着再创建一台机器**：在一小时内多次运行该命令后，每次后续运行都需要在 Desktop 打开的对话框中确认，该对话框会显示该命令

##### 停止使用某台机器

要阻止 Desktop 自行连接到由命令创建的机器，请归档该机器上的所有会话：

1. 在侧边栏中找到该机器的会话。会话的[详细信息面板](#see-session-details)会显示其运行所在的主机。
2. [归档每个会话](#archive-a-session)，并保持其归档状态。
3. 如果自创建这台机器以来，您尚未从同一个 `sshHostCommands` 条目创建另一台机器，也请[创建一台](#create-another-machine)或[移除该条目](#remove-an-entry)。在此之前，该机器是该条目已保存的连接，当您在选中该连接的情况下打开新建会话页面时，Desktop 可能会连接到它。
4. 在应用中删除指向该机器的任何其他 SSH 连接，或移除其 [`sshConfigs`](#pre-configure-ssh-connections-for-your-team) 条目。
5. 退出并重新打开 Desktop，这会关闭它与该机器的连接，并停止其访问该机器的尝试。
6. 用您的工具删除该机器。

##### 移除条目

从设置文件中移除某个条目或更改其 `id` 后，其机器上的会话仍会保留，因此 Desktop 仍可连接到这些机器。即使计算机前没有人，它也可能这样做，例如重新加入会话正在运行的进程，或投递一条等待发送的消息。要阻止这种情况，请按照[停止使用某台机器](#stop-using-a-machine)中的说明归档这些会话。

如果您是管理员并移除了一个托管条目，您用户的会话也会保留，每个用户都必须归档自己的会话。要自行阻止该条目的机器，请设置一个 [`sshHostAllowlist`](#restrict-which-ssh-hosts-users-can-connect-to)，使其匹配您的用户仍需要的主机，而不匹配这些机器的主机名。只有在某个模式能够区分两者的情况下，您才能这样做。

##### 将命令分发给团队

要为您的团队提供连接，请在每台计算机上的 Claude Code [托管设置](https://code.claude.com/docs/zh-CN/managed-settings.md)文件或 MDM 策略中设置 `sshHostCommands`，而不是在桌面应用自身的[设备管理策略](#device-management-policies)中设置。用户会看到您的条目与他们自己的条目并列，并且应用不提供任何编辑或移除条目的方式，无论是他们的还是您的。

以下托管设置文件为用户提供一个名为 Dev box 的连接，并关闭用户自己的条目，同时关闭来自用户、项目和本地设置的 Claude Code hook：

```json managed-settings.json theme={null}
{
  "sshHostCommands": [
    {
      "id": "acme-devbox",
      "name": "Dev box",
      "command": "/usr/local/bin/acme-dev create"
    }
  ],
  "strictPluginOnlyCustomization": ["hooks"]
}
```

部署该键之前，请确定以下每一项：

* **键放在哪里**：如果您针对 Claude Code 的 MDM 策略已经设置了某个[策略键](https://code.claude.com/docs/zh-CN/managed-settings.md#how-claude-code-combines-managed-sources)，请将其放在那里。否则，请将其放在托管设置文件中。默认情况下，此类策略的优先级高于该文件，此时 Desktop 会忽略文件中的条目。即使在适用[服务器托管设置](https://code.claude.com/docs/zh-CN/server-managed-settings.md)的计算机上，也请在计算机上设置它：Desktop 会忽略那里以及项目和本地设置中的该键。
* **`id`**：如果用户的 `sshHostCommands` 条目使用了您的 `id`，则只有您的条目生效。如果其他任何 SSH 连接使用了它（例如用户 `sshConfigs` 中的连接），您的条目会显示为灰色。请选择用户不太可能使用的 `id`。
* **程序**：请提供其完整路径，因为 Desktop 会在每个用户自己的 `PATH` 中查找裸名称。程序以该用户身份运行，使用其登录 shell 导出的变量。
* **用户自己的条目**：Desktop 将用户的条目视同 [hook](https://code.claude.com/docs/zh-CN/hooks.md)，因为两者都会在计算机上运行程序。要仅允许您的条目，请将 [`strictPluginOnlyCustomization`](https://code.claude.com/docs/zh-CN/settings-reference#strictpluginonlycustomization) 设置为 `["hooks"]`，或将 `"hooks"` 添加到其列表中。这样也会阻止来自用户、项目和本地设置的 Claude Code hook，且无法保留它们。如果您改为将该键或 [`allowManagedHooksOnly`](https://code.claude.com/docs/zh-CN/settings-reference#allowmanagedhooksonly) 设置为 `true`，则会阻止更多内容，具体见每个键的条目说明。
* **所有条目**：要同时关闭您的条目，请在托管设置中设置 [`disableAllHooks`](https://code.claude.com/docs/zh-CN/settings-reference#disableallhooks)。如果您设置了 [`disableDesktopLocalSessions`](https://code.claude.com/docs/zh-CN/settings-reference#disabledesktoplocalsessions)，或设置了不允许任何主机的 `sshHostAllowlist`，选择器将不会列出任何 **Create** 或 **Create another** 项。
* **锁定在哪里生效**：对于这些条目，Desktop 会从服务器托管设置以及计算机自身的文件或策略中读取 `allowManagedHooksOnly`、`strictPluginOnlyCustomization` 和 `disableAllHooks`，并应用更严格的结果。在 Desktop 检查 claude.ai 上的服务器托管设置之前，所有条目都保持灰色，即使您的组织没有任何服务器托管设置也是如此。
* **允许的主机**：如果您设置了 [`sshHostAllowlist`](#restrict-which-ssh-hosts-users-can-connect-to)，请确保有一个模式能匹配您的工具打印的每个主机名，并且模式中不要包含 `user@`。Desktop 只在命令运行后才检查打印的名称，因此被列表拒绝的机器已经存在，用户必须使用您的工具将其删除。每次连接时，解析后的 `HostName` 也必须匹配。

部署该键后，在一台测试计算机上重启 Desktop。在 Code 选项卡的环境选择器中，在 **SSH** 下或单独的 **SSH** 行中查找。您的条目会显示为 **Create** 项，或显示为灰色项，将鼠标悬停其上即可看到原因。如果它缺失，请检查您放置该键的位置，然后查看[如果命令缺失或显示为灰色](#if-a-command-is-missing-or-gray)中的各种情况。

##### 如果命令缺失或显示为灰色

在以下情况下，环境选择器不会列出任何 **Create** 或 **Create another** 项：

* **文件**：`~/.claude/settings.json` 包含[无效的 JSON 或被 schema 拒绝的值](https://code.claude.com/docs/zh-CN/settings.md#fix-a-broken-settings-file)、您拼错了键名，或者您将其放在了项目的设置中。请修复该文件。
* **平台和账户**：您使用的是 Windows，或者使用的是[第三方提供商](https://code.claude.com/docs/zh-CN/third-party-integrations.md)而非 Claude 账户。请自行将每个主机添加为 [SSH 连接](#ssh-sessions)。
* **您组织的设置**：您的管理员关闭了本地会话或 SSH 会话。请询问他们您是否可以使用命令。
* **没有 `ssh` 程序**：Desktop 无法在您的计算机上启动 OpenSSH 客户端。请安装它，或请您的管理员允许它，然后重启 Desktop。

无法选择的条目显示为灰色。将鼠标悬停其上即可查看原因：

* **条目中有错误**：修复消息中指出的字段或命令。如果 `id` 已被使用，请为该条目指定另一个。
* **设置文件损坏**：当您自己的设置文件损坏时，管理员的条目可能显示为灰色，并附带指出该文件的消息。[修复该文件](https://code.claude.com/docs/zh-CN/settings.md#fix-a-broken-settings-file)。
* **新主机**：Desktop 保存主机后的一分钟内，[**Create another**](#create-another-machine) 显示为灰色。请在一分钟后再选择它。在 Claude Desktop v4.4212.0 之前，**Replace Dev box's host** 会显示为灰色 10 分钟。
* **hook 已关闭或受限**：您自己的条目只在您自己的 hook 可以运行的地方生效。请从您的设置中移除 `disableAllHooks`，或询问您的管理员。
* **设置仍在加载**：在 Desktop 加载您组织的[服务器托管设置](https://code.claude.com/docs/zh-CN/server-managed-settings.md)之前，所有条目都显示为灰色，通常在启动后几秒钟内完成。如果条目一直是灰色，请检查您与 claude.ai 的连接。

#### 为您的团队预配置 SSH 连接

管理员可以通过在[托管设置](https://code.claude.com/docs/zh-CN/managed-settings.md)中设置 `sshConfigs`，向团队成员分发 SSH 连接。

* **用户看到的内容**：以这种方式定义的连接会自动出现在每个用户的环境选择器中。
* **用户可以更改的内容**：用户可以选择托管连接，并为其[设置自己的 **Worktree folder**](#set-the-worktree-folder-on-an-existing-connection)。他们无法在应用中编辑其其他字段或删除它。

以下示例预配置了一个连接：

```json theme={null}
{
  "sshConfigs": [
    {
      "id": "shared-dev-vm",
      "name": "Shared Dev VM",
      "sshHost": "user@dev.example.com",
      "sshPort": 22,
      "sshIdentityFile": "~/.ssh/id_ed25519"
    }
  ]
}
```

每个条目都需要 `id`、`name` 和 `sshHost`。`sshPort` 和 `sshIdentityFile` 字段是可选的。用户也可以将 `sshConfigs` 添加到自己的 `~/.claude/settings.json` 中。

如果您在 `sshHost` 中省略 `user@`，Desktop 会以每位开发者的 `~/.ssh/config` 为该主机设置的 `User` 身份连接，否则以其在自己计算机上的用户名连接。

#### 限制用户可以连接的 SSH 主机

管理员可以通过在[托管设置](https://code.claude.com/docs/zh-CN/managed-settings.md)中设置 `sshHostAllowlist`，将 Desktop 的 SSH 会话限制为一组已批准的主机。设置后，用户只能连接到解析后的主机名与其中某个模式匹配的主机。将其设置为空数组可禁用 SSH 会话。[`sshHostAllowlist` 参考条目](https://code.claude.com/docs/zh-CN/settings-reference#sshhostallowlist)说明了空数组如何与其他托管来源中的列表组合。

以下示例允许连接到 `devboxes.example.com` 下的任何主机以及一个指定的堡垒主机：

```json theme={null}
{
  "sshHostAllowlist": ["*.devboxes.example.com", "bastion.example.com"]
}
```

> [!WARNING]
>
> 如果您的组织提供[服务器托管设置](https://code.claude.com/docs/zh-CN/server-managed-settings.md)，请在其中设置 `sshHostAllowlist`。默认情况下，Desktop 仅从[提供策略键的最高优先级托管来源](https://code.claude.com/docs/zh-CN/managed-settings.md#how-claude-code-combines-managed-sources)读取该键。如果该来源未设置该键，Desktop 会忽略较低优先级的 MDM 策略或托管设置文件中的列表，并将该键视为[未设置](https://code.claude.com/docs/zh-CN/settings-reference.md#sshhostallowlist)。Desktop 不会显示任何警告。
>
> 同时，请在每个用户计算机上的最高优先级 MDM 策略或托管设置文件中保留相同的列表。Desktop 在启动时获取服务器托管设置，且不保留缓存副本，因此在获取成功之前，适用的是该计算机上的列表。

模式不区分大小写。`*` 匹配任何主机，`*.example.com` 匹配 `example.com` 及其任何子域。其他任何内容均为精确匹配。检查针对的是通过 `ssh -G` 进行 `~/.ssh/config` 解析后的主机名，因此只要 `Host` 别名解析后的 `HostName` 匹配，就允许使用该别名。会话的[跳板主机](#allow-jump-hosts)也必须匹配。

`sshHostAllowlist` 仅从托管设置中读取；用户或项目设置中的值会被忽略。只有 Claude Desktop 应用遵循此设置；Claude Code CLI 和 IDE 扩展不会读取它，并且它不会限制通过 Bash 工具运行的 `ssh` 命令。它管控的是 Desktop 应用连接到哪些主机，而不是网络出站流量，因此如果您需要硬性边界，请将其与您组织的网络或零信任控制措施结合使用。

##### 允许跳板主机

将 Desktop 检查的每台跳板主机按其自身解析后的 `HostName` 添加到 `sshHostAllowlist`。请在用户更新到 Claude Desktop v3.1623.0 或更高版本之前添加这些跳板主机，否则他们将无法通过这些主机进行连接。

> [!WARNING]
>
> 该列表只有一种条目，因此用户也可以在您列出的任何主机上启动会话。请只列出您同样允许用户在其上运行会话的跳板主机。

Desktop 会检查用户的 `~/.ssh/config` 为会话主机设置的 `ProxyJump` 中的跳板主机，或写作 `ssh -W %h:%p <host>` 的 `ProxyCommand` 中的跳板主机。如果 Desktop 在连接或重新连接会话时，所检查的某台跳板主机不在列表中，连接就会失败。当用户启动会话或发送消息时，错误详情会给出该跳板主机的 `HostName`，这就是需要添加的名称：

```text theme={null}
SSH host "bastion.example.com" is not allowed by your organization's managed settings. It is a jump host on the way to "devbox".
```

如果消息中显示 `managed settings on this computer`，请将该名称添加到用户计算机上的列表中。如果缺少多台跳板主机，错误每次只会指出一台。要在用户更新之前找出所有名称，请在用户计算机上使用终端：

**1. 查找路由**

使用用户连接的主机运行 `ssh -G`，例如 `ssh -G devbox`。查找 `proxyjump` 行，或写作 `ssh -W %h:%p <host>` 的 `proxycommand` 行。带有 `-l`、`-p`、`-J`、`-q`、`-v`、`-N`、`-T`、`-x` 或 `-a` 中任一选项的命令仍然计入，并且 `-J` 之后的主机也是跳板主机。如果 `proxycommand` 行运行的是 `ssh` 以外的程序，则无需为其添加任何内容。

**2. 查找每台跳板主机**

对该行中的每台跳板主机运行 `ssh -G`（不带任何 `:port`），并记下其输出的 `hostname`。如果该行中第一台跳板主机的输出包含上述任一类型的行，请对其重复此步骤。忽略后续跳板主机输出中的此类行。

**3. 添加名称**

将每个主机名或覆盖它的通配符，添加到您设置了列表的每个托管来源中的 `sshHostAllowlist`。然后让用户退出并重新打开 Desktop，因为它在启动时获取服务器托管设置。

#### 关闭本地会话并分发 SSH 连接

若要关闭本地会话并将 SSH 会话限制为您选择的主机，请同时部署 [`disableDesktopLocalSessions`](https://code.claude.com/docs/zh-CN/settings-reference#disabledesktoplocalsessions)、[`sshConfigs`](#pre-configure-ssh-connections-for-your-team) 和 [`sshHostAllowlist`](#restrict-which-ssh-hosts-users-can-connect-to)。

> [!NOTE]
>
> 此方案适用于使用 Claude 账户登录的用户。使用[第三方提供商](https://code.claude.com/docs/zh-CN/third-party-integrations.md)时，SSH 会话默认关闭，而设置 `sshHostAllowlist` 可以开启它们。在部署这些键之前，请阅读 [3P 上 Claude Desktop 中的 SSH 远程会话](https://claude.com/docs/third-party/claude-desktop/ssh-remote-sessions)。

以下示例关闭本地会话、添加一个连接，并将 SSH 会话限制为该连接的主机：

```json managed-settings.json theme={null}
{
  "disableDesktopLocalSessions": true,
  "sshConfigs": [
    {
      "id": "shared-dev-vm",
      "name": "Shared Dev VM",
      "sshHost": "dev.example.com"
    }
  ],
  "sshHostAllowlist": ["dev.example.com"]
}
```

这些键适用于桌面应用。它们不会改变运行 Claude Code 的其他方式，例如 CLI、Cowork 和云端会话：

* **CLI**：这些键都不会阻止开发者在自己的计算机上运行 Claude Code CLI
* **Cowork**：这些键都不适用于 [Cowork](https://claude.com/docs/cowork/overview) 会话或 Cowork 定时任务
* **云端会话**：这些键都不会改变[云端会话](#cloud-sessions)是否可用。若要关闭它们，请使用[管理控制台控制项](#admin-console-controls)下的 **Cloud sessions** 设置。

调整示例时，请检查主机名、Desktop 版本和文件权限：

* **主机名**：确保 `sshHostAllowlist` 中有一个模式与每个 `sshConfigs` 条目的主机名（不含 `user@`）匹配，否则开发者将无法连接到该主机。此外，还要添加与 Desktop 在通往该主机途中检查的每台[跳板主机](#allow-jump-hosts)相匹配的模式。有关 Desktop 如何匹配主机名，请参阅[限制用户可以连接的 SSH 主机](#restrict-which-ssh-hosts-users-can-connect-to)。
* **版本**：`disableDesktopLocalSessions` 需要 Claude Desktop v1.37937.0 或更高版本
* **文件权限**：确保计算机上每位开发者的账户都能读取托管设置文件。Desktop 不会应用账户无法读取的文件中的键，Claude Code 也会[在没有这些键的情况下启动](https://code.claude.com/docs/zh-CN/managed-settings.md#find-entries-claude-code-dropped)。

将这三个键放在一个[托管来源](https://code.claude.com/docs/zh-CN/managed-settings.md#how-claude-code-combines-managed-sources)中，即您提供的最高优先级来源。如果您提供[服务器托管设置](https://code.claude.com/docs/zh-CN/server-managed-settings.md)，还要在每台计算机上的最高优先级 MDM 策略或托管设置文件中保留这三个键的副本。Desktop 在启动时获取服务器托管设置，在获取成功之前，计算机上的副本适用。

若要确认设置已生效，请检查开发者的计算机：

**1. 检查环境选择器**

退出并重新打开 Desktop，然后打开[环境选择器](#start-a-session)。如果显示以下内容，则说明设置已生效：

* 该计算机的条目显示为灰色
* **SSH** 下的 **Shared Dev VM**。如果其中未列出，请点击单独的 **SSH** 行，或该组末尾的 **More**。

如果该计算机的条目仍然可用，请排查以下原因：

* **Desktop 版本**：如果 Desktop 版本早于 v1.37937.0，请更新 Desktop。
* **托管设置文件**：确保该文件是有效的 JSON。
* **托管来源**：确保该计算机读取的是包含这些键的[托管来源](https://code.claude.com/docs/zh-CN/managed-settings.md#how-claude-code-combines-managed-sources)。

**2. 添加不在允许列表中的主机**

使用不在允许列表中的主机名（例如 `other.example.com`）[添加 SSH 连接](#ssh-sessions)。Desktop 会保存该连接但拒绝连接，并提示您组织的设置不允许该连接。

## 企业配置

Team 或 Enterprise 计划上的组织可以通过管理员控制台控制、托管设置文件和设备管理策略来管理桌面应用行为。

### 管理员控制台控制

这些设置通过[管理员设置控制台](https://claude.ai/admin-settings/claude-code)配置：

* **Desktop**：控制您的组织中的用户是否可以在桌面应用中访问 Claude Code
* **Cloud sessions**：为您的组织启用或禁用[云端会话](https://code.claude.com/docs/zh-CN/claude-code-on-the-web.md)
* **Remote Control**：为您的组织启用或禁用 [Remote Control](https://code.claude.com/docs/zh-CN/remote-control.md)

在启用了 HIPAA 的 Enterprise 组织中，**Desktop** 开关默认关闭，[Owner](https://code.claude.com/docs/zh-CN/server-managed-settings.md#access-control) 可以将其打开。应用 [HIPAA 配置](https://code.claude.com/docs/zh-CN/hipaa-setup.md)会将其关闭（即使之前已打开），因此 Owner 之后必须重新将其打开。**Cloud sessions** 和 **Remote Control** 也默认关闭，并且一旦组织应用了 HIPAA 配置，Owner 就无法将它们打开。

> [!NOTE]
>
> Cowork 下的 OpenTelemetry 表单位于管理员控制台的[数据和隐私设置](https://claude.ai/admin-settings/data-privacy-controls)中的**监控**下，仅适用于 Cowork 会话。在此机器上的 Cowork 会话中，桌面应用将该收集器作为 `OTEL_*` 环境变量传递给 Claude Code，因此该表单生效，尽管该会话中的 Claude Code [从不获取管理员控制台设置](#managed-settings)。
>
> 要从 Code 选项卡会话导出遥测，请在 Claude Code 托管设置的 `env` 块中设置 `CLAUDE_CODE_ENABLE_TELEMETRY` 和 `OTEL_*` 变量，如[监控的管理员配置](https://code.claude.com/docs/zh-CN/monitoring-usage.md#administrator-configuration)中所示。本地、云端和 SSH 会话各自[从不同来源读取托管设置](#managed-settings)。有关云端会话可以到达的主机，请参阅[网络访问](https://code.claude.com/docs/zh-CN/cloud-environments.md#network-access)。有关 Code 选项卡会话报告的 `service.name`，请参阅[服务信息](https://code.claude.com/docs/zh-CN/monitoring-usage.md#service-information)。
>
> 要了解 SSH 会话在哪台远程机器上运行，请参阅[将遥测归因于 Desktop SSH 会话](https://code.claude.com/docs/zh-CN/monitoring-usage.md#attribute-telemetry-to-desktop-ssh-sessions)。

### 托管设置

托管设置覆盖项目和用户设置，并应用于 Desktop 中的 Claude Code 会话。您可以在您的组织的[托管设置](https://code.claude.com/docs/zh-CN/managed-settings.md)文件中设置这些键，或通过管理员控制台远程推送它们，除非某个键所在的行另有说明。

| 键 | 描述 |
| - | - |
| `permissions.disableBypassPermissionsMode` | 设置为 `"disable"` 以防止用户启用绕过权限模式。 |
| `disableAutoMode` | 设置为 `"disable"` 以从模式选择器中删除 [Auto](https://code.claude.com/docs/zh-CN/permission-modes.md#eliminate-prompts-with-auto-mode) 模式。也在 `permissions` 下接受。 |
| `autoMode` | 自定义自动模式分类器在您的组织中信任和阻止的内容。请参阅[配置自动模式](https://code.claude.com/docs/zh-CN/auto-mode-config.md)。 |
| `browserExternalPageTools` | 设置为 `"disabled"` 以防止 Claude 使用工具在[浏览器窗格](#browse-external-sites)中读取或作用于外部页面。用户仍然可以自己导航到外部网站，本地开发服务器预览不受影响。 |
| `disableMobileSimulatorTools` | 设置为 `true` 以阻止 Claude 在 [iOS Simulator 窗格](https://code.claude.com/docs/zh-CN/desktop-ios-simulator.md#turn-off-simulator-access)中控制和捕获设备的工具。该窗格仍可用于用户自己的点击；仅删除 Claude 的访问权限。该值必须是 JSON 布尔值 `true`；字符串 `"true"` 被忽略。 |
| `disableBrowserExternalNavigation` | 设置为 `true` 以完全关闭[浏览器窗格](#browse-external-sites)中的外部浏览。用户和 Claude 都无法导航到外部网站，localhost 开发服务器预览不受影响。该值必须是 JSON 布尔值 `true`；字符串 `"true"` 被忽略。 |
| `sshConfigs` | 预配置[SSH 连接](#pre-configure-ssh-connections-for-your-team)，在环境选择器中显示。用户无法删除托管连接，也无法编辑除其自己的 **Worktree folder** 之外的任何内容。 |
| `sshHostCommands` | 为用户提供[主机来自命令的 SSH 连接](#distribute-commands-to-your-team)，与用户自己的连接并存。Desktop 从计算机上的托管设置文件或 MDM 策略中读取它，并在[服务器管理的设置](https://code.claude.com/docs/zh-CN/server-managed-settings.md)中忽略它。需要 macOS 或 Linux 上的 Claude Desktop v3.6556.0 或更高版本，并使用 Claude 帐户登录。 |
| `sshHostAllowlist` | 限制 [SSH 会话](#restrict-which-ssh-hosts-users-can-connect-to)连接到已解析主机名与这些模式之一匹配的主机。仅从托管设置中读取。 |
| `disableDesktopLocalSessions` | 设置为 `true` 以关闭[在设备上运行的 Code 会话](#local-sessions-on-managed-devices)，仅保留到其他主机的 SSH 会话和云端会话可用。该值必须是 JSON 布尔值 `true`。仅从托管设置中读取。需要 Claude Desktop v1.37937.0 或更高版本。 |
| `disableSshSavedPasswords` | 设置为 `true` 以阻止 Desktop 提供记住 SSH 密码的选项，并阻止其使用或显示之前保存的密码。启用此设置不会删除这些密码。仅从托管设置中读取。需要 Claude Desktop v1.49585.0 或更高版本。 |
| `managedMcpServers` | 将 MCP 服务器配置推送到所有用户。仅在第三方 (3P) Desktop 部署中可用。在每个条目中，设置 `"http"`、`"sse"` 或 `"stdio"` 的传输、连接详细信息，以及可选的 `toolPolicy` 映射，该映射限制该服务器中用户可以调用的工具。通过托管设置文件、MDM 或 Claude apps gateway 策略的 [`desktop` 块](https://code.claude.com/docs/zh-CN/claude-apps-gateway-config#claude-desktop-overlay)提供它，因为 3P 部署不接收管理员控制台设置。要通过网关提供它，您需要网关服务器上的 Claude Code v2.1.232 或更高版本。这是桌面应用自己的键；Claude Code 读取自己的[同名托管设置](https://code.claude.com/docs/zh-CN/managed-mcp.md#provide-servers-through-managed-settings)，具有不同的条目形状。 |

哪些托管设置到达 Desktop 会话取决于该会话运行的位置。模型限制（如 [`availableModels`](https://code.claude.com/docs/zh-CN/model-config#restrict-model-selection)）在 Desktop 的 Claude Code 会话中的执行方式与在终端 CLI 中相同；请参阅[使用入口覆盖范围](https://code.claude.com/docs/zh-CN/model-config.md#surface-coverage)。

* **此机器上的本地会话**：部署到磁盘的托管设置文件适用。通过管理员控制台远程推送的托管设置也在会话使用[符合条件的登录](https://code.claude.com/docs/zh-CN/server-managed-settings.md#platform-availability)向 Anthropic 的 API 进行身份验证时到达这些会话，遵循与终端 CLI 相同的[设置优先级](https://code.claude.com/docs/zh-CN/settings.md#settings-precedence)。
* **[云端会话](#cloud-sessions)**：接收[服务器管理的设置](https://code.claude.com/docs/zh-CN/server-managed-settings.md)；设备部署的文件无法到达它们，因为它们在 Anthropic 管理的虚拟机上运行。路由到[自托管环境](https://code.claude.com/docs/zh-CN/self-hosted-environments)的会话也读取运行程序镜像中的托管设置文件。[Claude Code 如何组合托管源](https://code.claude.com/docs/zh-CN/managed-settings.md#how-claude-code-combines-managed-sources)说明该文件何时适用。
* **[SSH 会话](#ssh-sessions)**：会话从远程主机读取托管设置文件。Desktop 本身在本地机器上读取 `sshConfigs`、`sshHostAllowlist`、`disableSshSavedPasswords` 和 `disableDesktopLocalSessions`。如果您提供多个托管源，它[默认只从其中一个](https://code.claude.com/docs/zh-CN/managed-settings.md#how-claude-code-combines-managed-sources)读取。
* **[Cowork](https://claude.com/docs/cowork/overview) 会话**：在此机器上的 Cowork 会话中，Claude Code 永远不会获取管理员控制台设置，即使用户使用 Team 或 Enterprise 帐户登录，并读取部署到机器的策略，除非您的 Claude Desktop 配置设置了 `requireCoworkFullVmSandbox`。远程 Cowork 会话两者都不接收。请参阅[策略应用的位置和时间](https://code.claude.com/docs/zh-CN/managed-settings.md#where-and-when-a-policy-applies)了解哪些设备文件到达 Cowork，以及[MCP 权限规则](https://code.claude.com/docs/zh-CN/permissions.md#mcp)了解 `Bash` 和 `WebFetch` 规则如何应用于 Cowork 的工具。

在本地和 SSH 会话中，桌面应用直接将每个用户连接的 claude.ai 连接器传递给 Claude Code。无论您使用哪个设置源或文件位置，都没有 MCP 设置或 `managed-mcp.json` 到达这些连接器。要在这些会话中阻止连接器的工具，请使用您的组织的[连接器工具控制](https://code.claude.com/docs/zh-CN/mcp.md#organization-controls-on-connector-tools)。[连接器如何到达 Claude Code](https://code.claude.com/docs/zh-CN/mcp.md#how-connectors-reach-claude-code)显示在每种会话中哪些设置管理连接器。

`permissions.disableBypassPermissionsMode` 和 `disableAutoMode` 也在用户和项目设置中工作，但将它们放在托管设置中可防止用户覆盖它们。

有关仅托管源可以设置的权限、插件和交付键，请参阅[仅托管设置可以设置的键](https://code.claude.com/docs/zh-CN/managed-settings.md#managed-only-settings)。

### 设备管理策略

IT 团队可以通过 macOS 上的 MDM、Windows 上的组策略或 Linux 上的策略文件管理桌面应用。可用的策略包括启用或禁用 Claude Code 功能、在 macOS 和 Windows 上控制自动更新以及设置自定义部署 URL。

* **macOS**：通过使用 Jamf 或 Kandji 等工具的 `com.anthropic.claudefordesktop` 偏好域配置
* **Windows**：通过 `SOFTWARE\Policies\Claude` 处的注册表配置
* **Linux**：通过位于 `/etc/claude-desktop/managed-settings.json` 的 root 所有的文件配置，该文件以 JSON 对象形式保存策略键。如果除 root 之外的任何人可以写入该文件或其所在文件夹，Desktop 将拒绝使用该文件。它与 Claude Code 的[托管设置文件](https://code.claude.com/docs/zh-CN/managed-settings.md)是不同的文件。

#### 将登录限制为您的组织

要限制哪些 claude.ai 组织可以登录 Claude Desktop，请在 Claude Desktop 策略中设置 `forceLoginOrgUUID`。

该键需要 Claude Desktop v1.2581.0 或更高版本。

在 Linux 上，仅设置此键的策略文件如下：

```json /etc/claude-desktop/managed-settings.json theme={null}
{
  "forceLoginOrgUUID": "xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx"
}
```

将占位符替换为您的 claude.ai 组织 ID，该 ID 显示在 [claude.ai 管理员设置](https://claude.ai/admin-settings/organization)中。它与 Claude Code 的 [`forceLoginOrgUUID`](https://code.claude.com/docs/zh-CN/settings-reference#forceloginorguuid) 设置所采用的 ID 相同。有关设置此键的 macOS 和 Windows 示例，请参阅面向 HIPAA 就绪组织的 Cowork 指南中的[部署 Claude Desktop 策略](https://claude.com/docs/cowork/hipaa-setup#deploy-the-claude-desktop-policy)。

> [!WARNING]
>
> 部署之前，请检查 `forceLoginOrgUUID` 值。如果该 ID 不是您组织的 ID，您组织中的人员将无法使用 Claude Desktop，除非他们同时属于具有该 ID 的组织。要恢复，请部署正确的 ID，并让他们重新启动 Claude Desktop。

如果您在 Claude Code 的[托管设置](https://code.claude.com/docs/zh-CN/managed-settings.md)中设置 `forceLoginMethod` 和 `forceLoginOrgUUID`，并不会改变 Claude Desktop 的登录屏幕。

### 网络访问要求

Desktop 从 Anthropic CDN 主机加载其应用程序代码和用户内容。

```text theme={null}
anthropic.com
*.anthropic.com
claude.ai
*.claude.ai
claude.com
*.claude.com
claude.app
*.claude.app
*.claudeusercontent.com
*.claudemcpcontent.com
```

流量在端口 443 上使用 HTTPS，除非您为 [OTLP](https://code.claude.com/docs/zh-CN/monitoring-usage.md)、LLM 网关或 MCP 服务器配置自定义端口。

有关代理服务器、自定义证书颁发机构、mTLS 和独立 CLI 需要的域，请参阅[网络配置](https://code.claude.com/docs/zh-CN/network-config.md)。

要减少防火墙通配符的数量，请改为允许这些 Anthropic 主机。某些子域是动态生成的，必须保持为通配符。

```text theme={null}
anthropic.com
api.anthropic.com
a-api.anthropic.com
a-cdn.anthropic.com
s-cdn.anthropic.com
assets-proxy.anthropic.com
claude.ai
a.claude.ai
assets.claude.ai
downloads.claude.ai
*.livepreview.claude.ai
claude.com
platform.claude.com
sso.claude.com
*.livepreview.claude.app
*.claudeusercontent.com
*.claudemcpcontent.com
```

如果您的组织为 Claude 启用了[IP 允许列表](https://support.claude.com/en/articles/13200993-restrict-access-to-claude-with-ip-allowlisting)，请通过与 `claude.ai` 和 `api.anthropic.com` 相同的代理出口路由 `bridge.claudeusercontent.com`。如果您无法以这种方式路由它，请将您的代理用于该主机的出口地址添加到您的组织的 IP 允许列表，但仅当该地址专用于您的组织时：共享代理出口范围也允许代理供应商的其他客户。

Anthropic 根据连接到达的地址检查与该主机的连接是否符合您的组织的 IP 允许列表。如果您的代理通过不在该允许列表上的地址为其发送流量，Chrome 中的 Claude 和通过网桥连接的其他功能将停止工作，而应用的其余部分继续工作。

从 [Google Fonts](https://code.claude.com/docs/zh-CN/artifacts.md#improve-the-visual-design) 加载字体的 [Artifact](https://code.claude.com/docs/zh-CN/artifacts.md) 也会请求 `fonts.googleapis.com` 和 `fonts.gstatic.com`。两个主机都是可选的。如果您阻止它们，Artifact 将以备用字体呈现。使用快速拒绝而不是静默丢弃来阻止，以便字体请求立即失败，而不是延迟页面的首次呈现。

Artifact 还可以从 `cdnjs.cloudflare.com`、`cdn.jsdelivr.net`、`cdn.tailwindcss.com`、`code.jquery.com` 和 `unpkg.com` 加载 JavaScript 库（如 React 或图表包），而不能从其他任何外部主机加载。如果您阻止这些主机，Artifact 中依赖库的部分将无法工作，与被阻止的字体不同，被阻止的库没有备用方案。这里也使用快速拒绝，以便被阻止的库请求立即失败，而不是挂起直到超时。

### 身份验证和 SSO

Team 和 Enterprise 组织可以要求所有用户使用 SSO。有关计划级别的详细信息，请参阅[身份验证](https://code.claude.com/docs/zh-CN/authentication.md)，有关 SAML 配置，请参阅[设置 SSO](https://support.claude.com/en/articles/13132885-setting-up-single-sign-on-sso)；OIDC 设置在 [Claude Enterprise Administrator Guide](https://claude.com/resources/tutorials/claude-enterprise-administrator-guide) 中介绍。

### 数据处理

Claude Code 在本地会话中本地处理您的代码，或在云端会话中在 Anthropic 管理的基础设施上处理，除非您的组织将它们路由到[自托管环境](https://code.claude.com/docs/zh-CN/self-hosted-environments)。云端会话（包括在自托管环境中）将对话和代码上下文发送到 Anthropic 的 API 进行处理；本地和 SSH 会话将它们发送到您的部署配置的任何[模型提供商](#feature-comparison)，默认为 Anthropic 的 API。有关数据保留、隐私和合规性的详细信息，请参阅[数据处理](https://code.claude.com/docs/zh-CN/data-usage.md)。

### 部署

Desktop 可以通过企业部署工具分发：

* **macOS**：通过 MDM（如 Jamf 或 Kandji）使用 `.dmg` 安装程序分发
* **Windows**：通过 MSIX 包部署。有关企业部署选项（包括静默安装），请参阅[为 Windows 部署 Claude Desktop](https://support.claude.com/en/articles/12622703-deploy-claude-desktop-for-windows)

有关需要在防火墙中加入允许列表的域，请参阅上面的[网络访问要求](#network-access-requirements)。有关代理设置、自定义证书颁发机构和 LLM 网关，请参阅[网络配置](https://code.claude.com/docs/zh-CN/network-config.md)。

有关完整的企业配置参考，请参阅[企业配置指南](https://support.claude.com/en/articles/12622667-enterprise-configuration)。

## 来自 CLI？

如果您已经使用 Claude Code CLI，Desktop 运行相同的底层引擎，具有图形界面。您可以在同一机器上同时运行两者，甚至在同一项目上。每个维护单独的会话列表，您可以将 CLI 会话带入 Desktop。它们通过 CLAUDE.md 文件共享配置和项目记忆。

要将 CLI 会话移动到 Desktop，在终端中运行 `/desktop`。Claude 保存您的会话并在桌面应用中打开它，然后退出 CLI。当您使用 Claude 订阅登录时，此命令在 macOS 和 x64 Windows 上可用。它不适用于 API 密钥身份验证或 Amazon Bedrock、Google Cloud 的 Agent Platform 或 Microsoft Foundry。

从您的 shell，[`claude --desktop`](https://code.claude.com/docs/zh-CN/cli-reference#cli-flags) 直接打开 Desktop，无需启动终端会话。它需要 Claude Code v2.1.285 或更高版本，并具有与 `/desktop` 相同的平台和登录要求。没有其他参数时，它在当前目录中打开 Desktop。要在 Desktop 中打开现有的 CLI 会话，添加 `--continue` 以获取此目录中最近的对话，或使用 `--resume` 和 `/status` 显示的会话 ID：

```bash theme={null}
claude --desktop --resume <session-id>
```

Claude Code 打印 `Opening session <session-id> in Claude Desktop`，会话在应用中打开，命令退出。会话名称不能代替 ID。Claude Code 不会移动在另一个终端中打开或仍在后台运行的会话。如果未安装 Claude Desktop，命令会打印下载链接并退出。

您也可以从 Desktop 内部使用 `/resume` 选择 CLI 会话。此命令在本地会话中可用，不在 SSH、WSL 或云端会话中可用。

要在 Desktop 中继续终端会话：

1. 在终端中关闭会话。
2. 在 Desktop 输入框中，输入 `/resume`。Desktop 列出您从 CLI 在此计算机上启动的会话。按标题、文件夹或分支搜索，并预览每个会话的停止位置。
3. 选择会话。它在应用中继续，具有完整的对话和上下文。

Desktop 继续相同的会话而不是副本，所以之后在终端中 `claude --resume` 仍然可以找到它。

> [!TIP]
>
> 何时使用 Desktop vs CLI：当您想要在一个窗口中管理并行会话、并排排列窗格或可视化审查更改时，使用 Desktop。当您需要脚本、自动化或更喜欢终端工作流时，使用 CLI。

### CLI 标志等效项

此表显示了常见 CLI 标志的桌面应用等效项。未列出的标志没有桌面等效项，因为它们是为脚本或自动化设计的。

| CLI | Desktop 等效项 |
| - | - |
| `--model sonnet` | 输入框下方一行中的模型选择器 |
| `--resume`, `--continue` | 点击侧边栏中的会话，或在输入框中输入 `/resume` 来选择您从 CLI 启动的会话 |
| `--permission-mode` | 输入框下方一行中的模式选择器 |
| `--dangerously-skip-permissions` | 绕过权限模式。在 Pro、Max 和 Ultra 套餐上，在 **设置 > Code** 或 **设置 > Claude Code** 中启用"允许绕过权限模式"；在 Team 和 Enterprise 套餐上，由组织策略控制 |
| `--add-dir` | 在本地和 SSH 会话中[向会话添加文件夹](#add-folders-to-a-session)，或在云端会话中选择[多个仓库](#run-long-running-tasks-in-the-cloud) |
| `--allowedTools`, `--disallowedTools` | 无每个会话的等效项。[设置文件](https://code.claude.com/docs/zh-CN/settings.md)中的权限规则仍然适用。 |
| `--verbose` | [Verbose 视图模式](#switch-view-modes) |
| `--print`, `--output-format` | 不可用。Desktop 仅是交互式的。 |
| `ANTHROPIC_MODEL` 环境变量 | 输入框下方一行中的模型选择器 |
| `MAX_THINKING_TOKENS` 环境变量 | 请参阅[关闭扩展思考](#turn-off-extended-thinking) |

### 共享配置

Desktop 和 CLI 读取相同的配置文件，因此您的设置会沿用：

* 项目中的 **[CLAUDE.md](https://code.claude.com/docs/zh-CN/memory.md)** 和 `CLAUDE.local.md` 文件由两者共同使用
* 在 `~/.claude.json` 或 `.mcp.json` 中配置的 **[MCP 服务器](https://code.claude.com/docs/zh-CN/mcp.md)** 在两者中均可使用
* 在设置中定义的 **[hook](https://code.claude.com/docs/zh-CN/hooks.md)** 和 **[skill](https://code.claude.com/docs/zh-CN/skills.md)** 适用于两者
* `~/.claude.json` 和 `~/.claude/settings.json` 中的 **[设置](https://code.claude.com/docs/zh-CN/settings.md)** 是共享的。权限规则、允许的工具和 `settings.json` 中的其他设置适用于 Desktop 会话。
* **模型**：相同的[模型](https://code.claude.com/docs/zh-CN/model-config.md#available-models)在两者中都可用。在 Desktop 中，从输入框下方一行中的模型选择器中选择模型。您可以在会话期间从同一选择器更改模型。

#### 来自 Claude Desktop 聊天应用的 MCP 服务器

Desktop 应用会将 `claude_desktop_config.json` 中的 MCP 服务器加载到本地 Code 选项卡会话中，与来自 `~/.claude.json` 和 `.mcp.json` 的服务器一起使用。在 `claude_desktop_config.json` 中定义的服务器在 Desktop 聊天使用入口和本地 Code 选项卡会话中都可用。

如果您在 `claude_desktop_config.json` 和 `~/.claude.json` 或 `.mcp.json` 中定义相同的服务器名称，本地会话中的 Code 选项卡只连接一次并使用 `claude_desktop_config.json` 中的定义。

该应用还将来自 `~/.claude.json` 的 stdio 服务器重新传递到本地会话中的嵌入式 CLI。当 `~/.claude.json` 的顶层（用户作用域）和 `.mcp.json` 定义相同的 stdio 服务器名称时，Code 选项卡使用 `~/.claude.json` 中的定义，这与 CLI 的[作用域层次结构](https://code.claude.com/docs/zh-CN/mcp.md#scope-hierarchy-and-precedence)不同。

> [!NOTE]
>
> 独立 CLI 不读取 `claude_desktop_config.json`。在 macOS 和 WSL 上，运行 `claude mcp add-from-claude-desktop` 将这些服务器复制到 `~/.claude.json`。请参阅[从 Claude Desktop 导入 MCP 服务器](https://code.claude.com/docs/zh-CN/mcp.md#import-mcp-servers-from-claude-desktop)了解导入流程和作用域选项。

### 功能比较

此表比较了 CLI 和 Desktop 之间的核心功能。有关 CLI 标志的完整列表，请参阅 [CLI 参考](https://code.claude.com/docs/zh-CN/cli-reference.md)。

| 功能 | CLI | Desktop |
| - | - | - |
| 权限模式 | 所有模式，包括 `dontAsk` | Manual、Accept edits、Plan 和 Auto。启用后，绕过权限会出现在模式选择器中：在 Pro、Max 和 Ultra 套餐上通过设置开关启用，在 Team 和 Enterprise 套餐上通过组织策略启用 |
| [第三方提供商](https://code.claude.com/docs/zh-CN/third-party-integrations.md) | Amazon Bedrock、Google Cloud 的 Agent Platform、Microsoft Foundry | 默认使用 Anthropic 的 API。对于网关路由，请参阅[将桌面应用连接到网关](https://code.claude.com/docs/zh-CN/llm-gateway-connect.md#desktop-app)。要在 Amazon Bedrock、Google Cloud 的 Agent Platform、Microsoft Foundry 或自托管 LLM 网关上运行 Code 选项卡，请参阅 [Claude Desktop on 3P](https://claude.com/docs/third-party/claude-desktop/overview)。 |
| [MCP 服务器](https://code.claude.com/docs/zh-CN/mcp.md) | 在设置文件中配置 | 本地和 SSH 会话的连接器 UI，或设置文件 |
| [插件](https://code.claude.com/docs/zh-CN/plugins/overview.md) | `/plugin` 命令 | 插件管理器 UI |
| @mention 文件 | 基于文本 | 带自动完成，云端会话的第一条消息除外 |
| 文件附件 | 不可用 | 图像、PDF |
| 会话隔离 | [`--worktree`](https://code.claude.com/docs/zh-CN/cli-reference) 标志 | [在独立的 worktree 中运行会话](#run-a-session-in-its-own-worktree) |
| 多个会话 | 单独的终端 | 侧边栏选项卡 |
| 定期任务 | Cron 作业、CI 管道 | [定时任务](https://code.claude.com/docs/zh-CN/desktop-scheduled-tasks.md) |
| 计算机使用 | [在 macOS 上通过 `/mcp` 启用](https://code.claude.com/docs/zh-CN/computer-use) | 在 macOS 和 Windows 上的[应用和屏幕控制](#let-claude-use-your-computer) |
| iOS 模拟器 | 通过[计算机使用](https://code.claude.com/docs/zh-CN/computer-use.md#test-a-simulator-flow)驱动模拟器 | [iOS Simulator 窗格](https://code.claude.com/docs/zh-CN/desktop-ios-simulator.md)自动打开 |
| Dispatch 集成 | 不可用 | 侧边栏中的 [Dispatch 会话](#sessions-from-dispatch) |
| 脚本和自动化 | [`--print`](https://code.claude.com/docs/zh-CN/cli-reference)、[Agent SDK](https://code.claude.com/docs/zh-CN/headless.md) | 不可用 |

### Desktop 中不可用的内容

以下功能在 Desktop 中不可用，除非另有说明：

* **第三方提供商**：Desktop 默认连接到 Anthropic 的 API。要通过网关路由 Desktop，或在 Amazon Bedrock、Google Cloud 的 Agent Platform、Microsoft Foundry 或自托管 LLM 网关上运行 Code 选项卡，请按照[第三方提供商行](#feature-comparison)中的链接操作。
* **Linux (beta)**：Linux 桌面应用中尚不提供计算机使用。请参阅 [Claude Desktop on Linux](https://code.claude.com/docs/zh-CN/desktop-linux.md)。
* **内联代码建议**：Desktop 不提供自动完成风格的代码补全。它通过对话式提示词和显式代码更改工作，并可以在 Claude 回复后[建议您的下一个提示词](#accept-a-suggested-prompt)。
* **Agent team**：协调的团队（由 Claude 作为团队负责人从共享任务列表中为队友分配任务）在 [CLI](https://code.claude.com/docs/zh-CN/agent-teams.md) 中可用，不在 Desktop 中。对于一个会话内的多 Agent 工作，请使用[动态工作流](https://code.claude.com/docs/zh-CN/workflows.md)，它可在 Desktop 中运行；Claude 也可以直接[向您的其他会话发送消息并管理它们](#work-across-sessions)。
* **Terminal-dialog 命令**：在终端中打开交互式面板的内置命令，其行为在 Code 选项卡中有所不同。直接编辑[设置文件](https://code.claude.com/docs/zh-CN/settings.md)来管理权限规则和配置，或从独立 CLI 运行命令。
  * 没有参数形式的命令，例如 `/permissions`，会回复 `isn't available in this environment`。
  * `/config` 打开 **设置 > Code** 或 **设置 > Claude Code**。

## 故障排除

下面的部分涵盖特定于桌面应用的问题。对于出现在聊天中的运行时 API 错误，如 `API Error: 500`、`529 Overloaded`、`429` 或 `Prompt is too long`，请参阅[错误参考](https://code.claude.com/docs/zh-CN/errors.md)。

### 检查你的版本

要查看你运行的桌面应用版本：

* **macOS**：点击菜单栏中的 **Claude**，然后点击 **About Claude**
* **Windows**：点击 **Help**，然后点击 **About Claude**

点击版本号将其复制到你的剪贴板。

#### Code 选项卡中的 Claude Code 版本

要查看会话运行的 Claude Code 版本，请在 **Code** 选项卡的本地会话中输入 `/status`，然后查看 **Claude Code** 行，其中会显示版本号，例如 `2.1.286`。

要为本地会话获取更新的版本，请在 macOS 上打开 **Claude > Check for Updates…**，或在 Windows 上打开 **Help > Check for Updates…**，然后启动新会话。

在本地会话中，**Code** 选项卡运行其自己的 Claude Code 副本，该副本有自己的版本号。桌面应用会下载并更新该副本，因此它可能与终端中的 `claude` 命令版本不同，并且更新其中一个不会更新另一个。

### 检查您使用的布局

如果新会话在输入框上方而不是下方显示[环境](#start-a-session)，则说明您使用的是早期布局：

* **[低于 v2.37337.0 的 Desktop](#check-your-version)**：请更新以获取当前布局。在 macOS 上选择 **Claude > Check for Updates…**，或在 Windows 上选择 **Help > Check for Updates…**。
* **会话在您计算机上运行的[第三方提供商](https://code.claude.com/docs/zh-CN/third-party-integrations.md)**：当前布局需要 Claude Desktop v3.1623.0 或更高版本，因此请[检查您的版本](#check-your-version)。要进行更新，请在 macOS 上选择 **Claude > Check for Updates…**，或在 Windows 上选择 **Help > Check for Updates…**。如果您的组织[关闭了自动更新](#device-management-policies)，**Check for Updates…** 会被隐藏，此时请向管理员索取 v3.1623.0 或更高版本。
* **会话在您组织基础设施上运行的第三方提供商**：当前布局需要 Claude Desktop v4.29.0 或更高版本，因此请[检查您的版本](#check-your-version)。如果您使用的是较旧的版本，请向管理员索取 v4.29.0 或更高版本。如果您不清楚会话在哪里运行，请询问管理员。

在早期布局中，在输入框上方选择环境以及任何项目文件夹或仓库，在其下方的一行中选择权限模式和模型。早期布局没有[会话详细信息面板](#see-session-details)和[已保存的本地和 SSH 环境](#create-a-saved-environment)。

### Code 选项卡中的 403 或身份验证错误

如果在使用 Code 选项卡时看到 `Error 403: Forbidden` 或其他身份验证失败：

1. 从应用菜单中注销并重新登录。这是最常见的修复。
2. 验证您有活跃的付费订阅：Pro、Max、Ultra、Team 或 Enterprise。
3. 如果 CLI 工作但 Desktop 不工作，完全退出桌面应用，而不仅仅是关闭窗口，然后重新打开并登录。
4. 检查你的互联网连接和代理设置。

### 启动时屏幕空白或卡住

如果应用打开但显示空白或无响应的屏幕：

1. 重启应用。
2. 检查待处理的更新。在 macOS 和 Windows 上，应用在启动时自动更新；在 Linux 上，通过 apt 更新，如 [Claude Desktop on Linux](https://code.claude.com/docs/zh-CN/desktop-linux.md) 中所述。
3. 在托管网络上，确认你的防火墙允许[网络访问要求](#network-access-requirements)中的 CDN 主机。
4. 在 Windows 上，在 **Windows 日志 → 应用程序** 下的事件查看器中检查崩溃日志。

### "Failed to load session"

如果你看到 `Failed to load session`，选定的文件夹可能不再存在，Git 存储库可能需要未安装的 Git LFS，或文件权限可能阻止访问。尝试选择不同的文件夹或重启应用。

### 会话找不到已安装的工具

如果 Claude 找不到 `npm`、`node` 或其他 CLI 命令等工具，验证工具在你的常规终端中工作，检查你的 shell 配置文件是否正确设置 PATH，并重启桌面应用以重新加载环境变量。

### Git 和 Git LFS 错误

在其自己的 worktree 中运行的会话需要 Git。如果你看到"Git is required"，安装 [Git](https://git-scm.com/downloads)，或在 Windows 上安装 [Git for Windows](https://git-scm.com/downloads/win)，然后重试。在 Windows 上，1.49585.0 之前的 Claude Desktop 版本在启动任何本地会话之前要求 Git；如果你看到该提示且不使用 worktrees，请更新应用。

如果你看到"Git LFS is required by this repository but is not installed"，从 [git-lfs.com](https://git-lfs.com/) 安装 Git LFS，运行 `git lfs install`，并重启应用。

### MCP servers 在 Windows 上不工作

如果 MCP server 切换不响应或服务器在 Windows 上连接失败，检查服务器在你的设置中是否正确配置，重启应用，验证服务器进程在任务管理器中运行，并查看服务器日志以获取连接错误。

### SSH host key verification failed

以下每条消息都表示 Desktop 拒绝了某个[主机密钥](#check-a-remote-machines-host-key)：

* 在 `Couldn't connect to <host>` 卡片的 **View details** 下出现 `SSH host key verification failed` 或 `Host denied (verification failed)`
* 在输入框旁边或执行 **Test connection** 后出现 `Connection failed: Host key changed or not trusted`
* 在已断开的会话上出现 `<host>'s host key is not trusted` 或 `<host>'s host key has changed`

Desktop 拒绝密钥，通常是因为它与您计算机上保存的密钥不同，或者没有为该机器保存任何密钥。重建或重新安装过的机器通常会提供新的密钥，拦截连接的程序也会如此。再次连接之前，请先检查密钥，并从 Desktop 随消息显示的内容入手：

* **您使用的是 Windows，且 Desktop 显示 `Connection failed: Host key changed or not trusted`**：首先排除[缺少 OpenSSH 客户端](#ssh-host-key-refused-on-windows-without-an-openssh-client)的情况
* **Desktop 提示您设置 `UserKnownHostsFile`**：在 `~/.ssh/config` 中将其设置为 `~/.ssh/known_hosts`，然后[从终端检查密钥](#check-the-host-key-from-a-terminal)
* **您看到 Review new key 按钮**：选择该按钮，并将 Desktop 显示的指纹与该机器自身的指纹进行[比较](#check-a-remote-machines-host-key)。如果两者匹配，请选择 **Trust new key** 以替换已保存的密钥。否则请选择 **Cancel**。
* **您没有看到 Review new key 按钮、Desktop 显示 `Couldn't show the new key`，或者在 Trust new key 之后 Desktop 再次拒绝密钥**：[从终端检查密钥](#check-the-host-key-from-a-terminal)

#### 从终端检查主机密钥

在您的计算机上运行 `ssh <SSH host>`，使用 Desktop 中您的 [SSH 连接](#ssh-sessions)的 **SSH host** 值。如果该连接有 **SSH port** 值，请添加 `-p <SSH port>`。在 Windows 上，请使用 PowerShell，因为 Desktop 不会使用 Git Bash 或 WSL 中的 `ssh` 进行连接。如果 PowerShell 找不到 `ssh`，请[安装 Microsoft 的 OpenSSH 客户端](#ssh-host-key-refused-on-windows-without-an-openssh-client)。

留意 `ssh` 在其消息中提到的是哪台机器，它可能是跳板主机而不是 `<SSH host>`，然后按照适用的情况操作：

* **`ssh` 询问是否继续连接**：将其显示的指纹与该机器自身的指纹进行[比较](#check-a-remote-machines-host-key)。仅当两者匹配时才回答 `yes`。
* **`ssh` 警告主机密钥已更改**：将警告中的指纹与该机器自身的指纹进行[比较](#check-a-remote-machines-host-key)。仅当两者匹配时，才删除 `Offending <type> key in <file>:<line>` 所指出的那一行，然后重新运行 `ssh`。如果 `ssh` 再次就另一行发出警告，请按照同样的方式比较、删除并重新运行。
* **`ssh` 未经询问就拒绝了它不认识的密钥**：从该机器的管理人员处获取该机器的 `known_hosts` 行，不要通过您正在检查的连接获取。将该行添加到您的 `known_hosts` 文件中，并使用 `ssh` 消息中的名称作为其主机名，然后重新运行 `ssh`。
* **`ssh` 没有询问主机密钥就继续登录**：先不要输入密码。[检查 `ssh` 已信任的密钥](#check-the-keys-that-ssh-already-trusts)。

一旦 `ssh` 接受了您检查过或添加的密钥，请在 Desktop 中重试。如果 Desktop 再次拒绝该密钥，请参阅[获取 SSH 问题的 Desktop 日志](#get-desktops-logs-for-an-ssh-problem)。

##### 检查 `ssh` 已信任的密钥

添加 `-v -S none` 后重新运行相同的 `ssh` 命令。在其输出中，`Server host key:` 行会显示一个指纹，紧随其后的 `Host '<name>' is known` 行会指出该指纹所属的机器。将您找到的每个指纹与对应机器自身的指纹进行[比较](#check-a-remote-machines-host-key)。

* **跳板主机没有 `Server host key:` 行**：同时运行 `ssh -v -S none <jump host>`，并以同样的方式比较该指纹。
* **某个指纹不匹配，或者您无法检查它**：不要再次连接。删除已保存的密钥：在 `Host '<name>' is known` 下方，`Found key in <file>:<line>` 行指出了保存该密钥的文件。运行 `ssh-keygen -R "<name>" -f <file>`。如果您将主机密钥保存在多个文件中，请从每个文件中删除该密钥。如果您无法修改某个文件，请告知您计算机的管理人员。
* **所有指纹都匹配**：在 Desktop 中重试。如果 Desktop 再次拒绝该密钥，请参阅[获取 SSH 问题的 Desktop 日志](#get-desktops-logs-for-an-ssh-problem)。

### 在没有 OpenSSH 客户端的 Windows 上 SSH 主机密钥被拒绝

在没有 OpenSSH 客户端的 Windows 上，Desktop 即使面对正确的主机密钥也会拒绝，并显示 `Connection failed: Host key changed or not trusted`。它需要该客户端来查找已保存的密钥。要确认您是否已安装该客户端，请在 PowerShell 中运行 `ssh`。如果 PowerShell 无法识别 `ssh`，请安装 [Microsoft 的 OpenSSH 客户端](https://learn.microsoft.com/windows-server/administration/openssh/openssh_install_firstuse)并重启 Desktop。

然后再次连接。如果 Desktop 询问是否首次连接，请在信任之前先[检查该机器的主机密钥](#check-a-remote-machines-host-key)。如果 Desktop 仍然拒绝该密钥，请按照 [SSH host key verification failed](#ssh-host-key-verification-failed) 下的其他情况操作。

### 登录后 SSH 连接失败

在下表中找到卡片的标题或错误消息末尾的代码，然后按照最后一列的说明操作。要查看消息，请在失败卡片上选择 **View details**。

| 卡片上的标题 | 代码 | 处理方法 |
| - | - | - |
| `<host>'s SSH server has SFTP turned off` | `[SFTP_UNAVAILABLE]` | 在服务器的 `sshd_config` 中启用 SFTP，如[远程机器要求](#remote-machine-requirements-for-ssh-sessions)中所述 |
| `SFTP doesn't start on <host>` | `[SFTP_NOT_STARTED]` | 在您计算机的终端中使用 `sftp` 连接到同一主机，查看是什么阻止了它启动，然后检查 Desktop 消息中提到的设置和文件 |
| `<host> allows too few SSH sessions per connection` | `[SESSION_LIMIT]` | 更新到 Claude Desktop v3.1623.0 或更高版本，或提高服务器 `sshd_config` 中的 `MaxSessions`，例如设为 OpenSSH 的默认值 10 |
| `<host> is out of disk space` | `[ENOSPC]` 或 `[EDQUOT]` | 在远程机器上的主目录或 `/tmp` 中释放空间或配额 |
| `Free up disk space on this computer` | `[ENOSPC]` 或 `[EDQUOT]`，出现在以 `Couldn't prepare the deploy locally` 开头的消息中 | 释放您计算机上的空间 |
| `Claude Code won't start on <host>` | `[CLI_UNRESPONSIVE]` | 等待一分钟，然后重试。如果仍然失败，请在远程机器上运行消息中提到的命令，查看 Claude Code 在那里是否有响应。 |

对于任何其他消息，请查看[远程机器要求](#remote-machine-requirements-for-ssh-sessions)。

### 获取 SSH 问题的 Desktop 日志

`ssh.log` 是 Desktop 对其 SSH 连接的记录，它包含在您从 **Help** 菜单保存的 Desktop 日志 zip 文件中。要保存该文件，请打开 **Help > Troubleshooting** 并选择适用于您系统的项目：

* **macOS**：**Show Logs in Finder**
* **Windows**：**Show Logs in File Explorer**
* **Linux**：**Show Logs in File Manager**

在发送这些文件之前，请先检查其内容，因为其中可能包含来自您环境的名称和其他详细信息。`<host>` 和 `<user>` 代表 Desktop 已替换的名称。Desktop 不会替换所有名称，SSH 主机名或用户名仍可能按原样出现。如果您将 zip 文件发送给他人，请告诉对方哪个主机在何时失败，因为日志中可能不会提及该主机。

该 zip 文件不包含 Desktop 在远程机器上安装的辅助程序的日志。要同时获取该日志，请从建立连接的用户的主目录中 [`~/.claude/remote/run/`](#files-and-processes-on-the-remote-machine) 下的辅助程序文件夹中复制 `remote-server.log`。在发送之前，也请检查该文件的内容。

### 应用无法退出

* **macOS**：按 Cmd+Q。如果应用不响应，使用 Cmd+Option+Esc 强制退出，选择 Claude，然后点击强制退出。
* **Windows**：使用 Ctrl+Shift+Esc 的任务管理器来结束 Claude 进程。

### Windows 特定问题

* **安装后 PATH 未更新**：打开新的终端窗口。PATH 更新仅适用于新的终端会话。
* **并发安装错误**：如果你看到关于另一个安装正在进行的错误，但实际上没有，尝试以管理员身份运行安装程序。

### 在 CLI 中打开时"Branch doesn't exist yet"

云端会话可以创建在您的本地机器上不存在的分支。点击会话中的分支名称并选择 **Copy branch name**，然后在本地获取它：

```bash theme={null}
git fetch origin <branch-name>
git checkout <branch-name>
```

### 仍然卡住？

* 在桌面应用中打开 Help → Get Support，或直接访问 [Claude 支持中心](https://support.claude.com/)
* 对于在独立 `claude` CLI 中也能重现的问题，在 [GitHub Issues](https://github.com/anthropics/claude-code/issues) 上搜索或提交错误

提交问题时，包括你的桌面应用版本、你的操作系统、确切的错误消息和相关日志。在 macOS 上，检查 Console.app。在 Windows 上，检查事件查看器 → Windows 日志 → 应用程序。在将日志摘录发布到公开问题之前，请查看它们；它们可能包含来自你的环境的文件路径和其他详细信息。
