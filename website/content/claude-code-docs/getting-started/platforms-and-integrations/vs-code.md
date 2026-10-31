> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# 在 VS Code 中使用 Claude Code

> 安装和配置 VS Code 的 Claude Code 扩展。获得 AI 编码协助，包括内联 diff、@-提及、计划审查和快捷键。

![VS Code 编辑器，右侧打开 Claude Code 扩展面板，显示与 Claude 的对话](https://mintcdn.com/claude-code/-YhHHmtSxwr7W8gy/images/vs-code-extension-interface.jpg?fit=max\&auto=format\&n=-YhHHmtSxwr7W8gy\&q=85\&s=300652d5678c63905e6b0ea9e50835f8)

VS Code 扩展为 Claude Code 提供了原生图形界面，直接集成到您的 IDE 中。这是在 VS Code 中使用 Claude Code 的推荐方式。

使用该扩展，您可以：

* 在接受 Claude 的计划之前审查和编辑它们
* 在进行编辑时自动接受
* 根据您的选择 @-提及具有特定行范围的文件
* 访问对话历史记录
* 在单独的选项卡或窗口中打开多个对话

## 前置条件

安装前，请确保您拥有：

* VS Code 1.94.0 或更高版本
* Anthropic 账户：任何付费 Claude 订阅（Pro、Max、Ultra、Team 或 Enterprise）或 Claude Console 账户都可以使用，无需 API 密钥。首次打开扩展时，您将[使用此账户登录](https://code.claude.com/docs/zh-CN/authentication.md#log-in-to-claude-code)。如果您通过第三方提供商（如 Amazon Bedrock 或 Google Cloud 的 Agent Platform）访问 Claude，请参阅[使用第三方提供商](#use-third-party-providers)了解设置说明。

> [!TIP]
>
> 该扩展包含其自己的 CLI（命令行界面）副本用于聊天面板。要在 VS Code 的集成终端中运行 `claude`，您还需要[独立 CLI 安装](https://code.claude.com/docs/zh-CN/setup.md)。有关详细信息，请参阅 [VS Code 扩展与 Claude Code CLI](#vs-code-extension-vs-claude-code-cli)。

## 安装扩展

点击与您的 IDE 对应的链接直接安装：

* [为 VS Code 安装](vscode:extension/anthropic.claude-code)
* [为 Cursor 安装](cursor:extension/anthropic.claude-code)

或者在 VS Code 中，按 `Cmd+Shift+X`（Mac）或 `Ctrl+Shift+X`（Windows/Linux）打开扩展视图，搜索"Claude Code"，然后点击 **Install**。

> [!NOTE]
>
> 扩展市场属于第三方软件，Anthropic 不为其背书、维护或审计。Anthropic 会将扩展提交至 Visual Studio Marketplace 和 Open VSX Registry 进行发布。

扩展的版本号即其捆绑的 Claude Code 版本。例如，需要 Claude Code v2.1.286 或更高版本的功能，需要 2.1.286 或更高版本的扩展，版本号可在扩展视图中查看。

Claude Code 还可以在其他编辑器和容器中运行：

* **其他 VS Code 分支版本**：该扩展也可以安装在 Devin Desktop 或 Kiro 等分支版本中。在编辑器的扩展视图中搜索"Claude Code"，或从 [Open VSX Registry](https://open-vsx.org/extension/Anthropic/claude-code) 安装。
* **无法安装该扩展的编辑器**：请[安装 CLI](https://code.claude.com/docs/zh-CN/quickstart.md)，然后在编辑器的集成终端中运行 `claude`。CLI 可在任何终端中使用。
* **开发容器**：要在开发容器中运行 Claude Code，请参阅[开发容器](https://code.claude.com/docs/zh-CN/devcontainer.md)。

> [!NOTE]
>
> 如果安装后扩展没有出现，请重启 VS Code，或从命令面板运行"Developer: Reload Window"。

## 开始使用

安装完成后，您可以通过 VS Code 界面开始使用 Claude Code：

**1. 打开 Claude Code 面板**

在 VS Code 中，Spark 图标代表 Claude Code：![Spark 图标](https://mintcdn.com/claude-code/c5r9_6tjPMzFdDDT/images/vs-code-spark-icon.svg?fit=max\&auto=format\&n=c5r9_6tjPMzFdDDT\&q=85\&s=3ca45e00deadec8c8f4b4f807da94505)

打开 Claude 最快的方式是点击**编辑器工具栏**（编辑器右上角）中的 Spark 图标。该图标仅在打开文件时显示。

![VS Code 编辑器，编辑器工具栏中显示 Spark 图标](https://mintcdn.com/claude-code/mfM-EyoZGnQv8JTc/images/vs-code-editor-icon.png?fit=max\&auto=format\&n=mfM-EyoZGnQv8JTc\&q=85\&s=eb4540325d94664c51776dbbfec4cf02)

打开 Claude Code 的其他方式：

* **活动栏**：点击左侧边栏中的 Spark 图标以打开会话列表。点击任意会话即可在您的[首选位置](#extension-settings)打开它，或者开始一个新会话。该图标始终显示在活动栏中。
* **命令面板**：按 `Cmd+Shift+P`（Mac）或 `Ctrl+Shift+P`（Windows/Linux），输入"Claude Code"，然后选择一个选项，例如"Open in New Tab"
* **状态栏**：点击窗口右下角的 **✻ Claude Code**。即使没有打开任何文件，此方式也可用。

您可以拖动 Claude 面板，将其放置在 VS Code 中的任意位置。详情请参阅[自定义您的工作流](#customize-your-workflow)。

**2. 登录**

首次打开面板时，会显示登录界面。点击 **Sign in**，然后在浏览器中完成授权。

如果之后看到 **Not logged in · Please run /login**，扩展会自动重新打开登录界面。如果登录界面没有出现，请在命令面板中通过 **Developer: Reload Window** 重新加载窗口。

如果您已在 shell 中设置了 `ANTHROPIC_API_KEY`，但仍看到登录提示，则可能是 VS Code 没有继承您的 shell 环境。请在终端中使用 `code .` 启动 VS Code，使其继承您的环境变量，或者改用您的 Claude 账户登录。

登录后，会显示 **Learn Claude Code** 清单。点击 **Show me** 逐项完成，或点击 X 将其关闭。如需稍后重新打开，请在 VS Code 设置的 **Extensions > Claude Code** 下取消勾选 **Hide Onboarding**。

**3. 发送提示词**

请 Claude 帮助处理您的代码或文件，无论是解释某个功能的工作原理、调试问题，还是进行修改。

> [!TIP]
>
> Claude 会自动看到您选中的文本。按 `Option+K`（Mac）/ `Alt+K`（Windows/Linux）还可以将 @ 提及引用（例如 `@file.ts#5-10`）插入到提示词中。

以下是询问文件中特定行的示例：

![VS Code 编辑器中选中了 Python 文件的第 2-3 行，Claude Code 面板显示了关于这些行的问题以及 @ 提及引用](https://mintcdn.com/claude-code/FVYz38sRY-VuoGHA/images/vs-code-send-prompt.png?fit=max\&auto=format\&n=FVYz38sRY-VuoGHA\&q=85\&s=ede3ed8d8d5f940e01c5de636d009cfd)

**4. 审查更改**

查看输入框底部显示的[权限模式](https://code.claude.com/docs/zh-CN/permission-modes.md#which-mode-a-session-starts-in)：

* 在 Auto 或 Edit automatically 模式下，Claude 会直接编辑工作区中的[大多数文件](https://code.claude.com/docs/zh-CN/permission-modes.md#protected-paths)，而不会询问。
* 在 Manual 模式下，当 Claude 想要编辑文件时，会并排显示原始内容与建议的更改，然后请求权限。您可以接受、拒绝，或告诉 Claude 改为执行其他操作。如果您在接受之前直接在 diff 视图中编辑了建议的内容，Claude 会被告知您已修改该内容，因此不会假定文件与其原始建议一致。

  ![VS Code 显示 Claude 建议更改的 diff，以及询问是否进行编辑的权限提示](https://mintcdn.com/claude-code/FVYz38sRY-VuoGHA/images/vs-code-edits.png?fit=max\&auto=format\&n=FVYz38sRY-VuoGHA\&q=85\&s=e005f9b41c541c5c7c59c082f7c4841c)

如需逐个审查建议的编辑，请使用 diff 中每处更改下方的 **Accept this change** 和 **Reject this change** 按钮。拒绝某处更改会在建议内容中将其还原；接受则会将其标记为已审查。逐处更改审查需要 Claude Code v2.1.275 或更高版本。

接受或拒绝整个文件仍会结束审查。包含超过 100 处更改的 diff 打开时不会显示逐处更改按钮，因此请将其作为整个文件进行审查。

同样的操作也可以在光标位置通过编辑器的上下文菜单使用，或在命令面板中通过 **Claude Code: Accept Change at Cursor** 和 **Claude Code: Reject Change at Cursor** 使用。

有关 Claude Code 更多用法的灵感，请参阅[常见工作流](https://code.claude.com/docs/zh-CN/common-workflows.md)。

> [!TIP]
>
> 在命令面板中运行"Claude Code: Open Walkthrough"，即可获得基础功能的引导式教程。

## 使用输入框

输入框支持以下几项功能：

* **权限模式**：点击输入框底部的模式指示器即可切换权限模式。在 Claude Code v2.1.283 或更高版本中，Auto 是内置的初始权限模式；在更早的版本中，仅 Pro、Max、Ultra 和 Team 套餐如此。请参阅[扩展如何选择初始权限模式](https://code.claude.com/docs/zh-CN/permission-modes.md#switch-permission-modes)，了解哪些因素会改变初始模式，以及指示器提供的所有权限模式。
  * **Auto**：由分类器审查大多数操作，而不是询问您。请参阅[自动模式](https://code.claude.com/docs/zh-CN/permission-modes.md#eliminate-prompts-with-auto-mode)，了解它会审查和阻止哪些操作。
  * **Manual**：Claude 在编辑文件和执行大多数 shell 命令之前会请求权限。
  * **Plan**：Claude 会描述它将要执行的操作，并在进行更改前等待批准。VS Code 会自动将计划作为完整的 Markdown 文档打开，您可以在 Claude 开始之前添加行内评论来提供反馈。

    您也可以在输入框中输入 `/plan`。需要 Claude Code v2.1.280 或更高版本。

    * `/plan`：切换到计划模式。如果已处于计划模式，则改为显示当前计划。
    * 带任务的 `/plan`，例如 `/plan fix the auth bug`：切换到计划模式并开始为该任务制定计划。
    * `/plan open`：已处于计划模式时，在编辑器中打开计划文件。
  * **Edit automatically**：Claude 直接进行编辑，不会询问。
* **模型**：从命令菜单中选择 **Switch model…**，即可在会话中途更换模型。您也可以点击输入框底部的模型名称来打开同一个选择器。在 Claude Code v2.1.284 或更高版本中，在输入框中单独输入 `/model` 也会打开该选择器。

  当前模型支持 [effort 级别](https://code.claude.com/docs/zh-CN/model-config.md#adjust-effort-level)时，选择器还会显示 **Effort** 行，模型名称按钮也会显示所选级别。当您选择 `max` 以外的级别时，Claude Code 会将其保存为当前模型的默认值，存储在用户设置的 [`modelSettings`](https://code.claude.com/docs/zh-CN/settings-reference#modelsettings) 下；`max` 仅适用于当前会话。模型名称按钮和 **Effort** 行需要 Claude Code v2.1.257 或更高版本。

  启用[动态工作流](https://code.claude.com/docs/zh-CN/workflows.md)且当前模型支持时，**Effort** 行下方会出现 **Ultracode** 开关。打开它后，Claude 会以所选 effort 级别，为本会话中的每项实质性任务规划一个[工作流](https://code.claude.com/docs/zh-CN/workflows.md#let-claude-decide-with-ultracode)。开关打开期间，模型名称按钮会在级别后显示 `· Ultracode`。该开关需要 Claude Code v2.1.284 或更高版本。
* **命令菜单**：点击 `/` 或输入 `/` 即可打开命令菜单。选项包括附加文件和切换模型。

  Customize 部分包含 MCP 服务器、命令、输出样式、hook、记忆、指令、权限和插件等条目。带有终端图标的项目会在集成终端中打开。

  * 要浏览 `/usage` 或 [`/remote-control`](https://code.claude.com/docs/zh-CN/remote-control) 等命令，请在 Customize 部分中选择 **Slash commands**。对话框会列出这些命令并提供筛选框。选择其中一个即可运行。在输入框中输入 `/` 仍会以内联方式提示命令。需要 Claude Code v2.1.257 或更高版本。

    输入 `/skills` 也会打开此对话框。每个 [skill](https://code.claude.com/docs/zh-CN/skills.md) 行都会显示其[可见性](https://code.claude.com/docs/zh-CN/skills.md#override-skill-visibility-from-settings)，例如 **On** 或 **Name only**。点击可见性即可更改，但标记为 **locked** 的行（例如插件 skill）除外。`/skills` 快捷方式和可见性控件需要 Claude Code v2.1.280 或更高版本。
  * 在 Customize 部分中选择 **Output styles**，即可选择[输出样式](https://code.claude.com/docs/zh-CN/output-styles.md)，包括您的自定义样式。需要 Claude Code v2.1.257 或更高版本。

    如需创建自定义样式，请从 **Output styles** 菜单中选择 **Build a custom style**。Claude Code 会在项目级或用户级为您编写[样式文件](https://code.claude.com/docs/zh-CN/output-styles.md#create-a-custom-output-style)。需要 Claude Code v2.1.261 或更高版本。
  * 在 Customize 部分中选择 **Hooks**，即可查看会话中已加载的 [hook](https://code.claude.com/docs/zh-CN/hooks.md)，按事件分组。您可以添加、编辑或删除保存在用户、项目和本地设置文件中的 hook。来自其他来源（例如托管设置或插件）的 hook 为只读。需要 Claude Code v2.1.269 或更高版本。
  * 在 Customize 部分中选择 **Permissions**，即可查看会话的[权限规则](https://code.claude.com/docs/zh-CN/permissions.md)，按 Allow、Ask 和 Deny 分组。您可以向用户、项目或本地设置添加规则，并删除保存在其中的规则。来自其他来源的规则（例如托管设置或仅针对本会话做出的批准）为只读。需要 Claude Code v2.1.269 或更高版本。
  * 在 Customize 部分中选择 **Memory**，即可打开或关闭[自动记忆](https://code.claude.com/docs/zh-CN/memory.md#auto-memory)。打开期间，您还可以浏览 Claude 已保存的记忆，并在文件管理器中显示存储这些记忆的文件夹。需要 Claude Code v2.1.274 或更高版本。

    点击已保存的记忆即可在对话框中阅读，您可以在其中编辑文本、删除该记忆，或在编辑器中打开其文件。在对话框中查看、编辑和删除记忆需要 Claude Code v2.1.275 或更高版本。
  * 在 Customize 部分中选择 **Instructions**，即可编辑 Claude 读取的 [CLAUDE.md 文件](https://code.claude.com/docs/zh-CN/memory.md#claude-md-files)。选择一个文件即可在编辑器中打开。如果文件尚不存在，Claude Code 会先创建它。需要 Claude Code v2.1.274 或更高版本。
  * 在 Customize 部分中选择 **Status**，或输入 `/status`，即可查看会话的 Claude Code 版本、账户、模型和 MCP 服务器详细信息。需要 Claude Code v2.1.280 或更高版本。
  * 在 Customize 部分中选择 **Sandbox**，或输入 `/sandbox`，即可查看 Claude 的 Bash 命令是否在[沙箱](https://code.claude.com/docs/zh-CN/sandboxing.md)中运行。您可以在此切换沙箱模式并添加[排除的命令](https://code.claude.com/docs/zh-CN/settings-reference.md#sandbox-excludedcommands)。需要 Claude Code v2.1.280 或更高版本。
  * 在 Customize 部分中选择 **Claude in Chrome**，或输入 `/chrome`，即可检查和管理 [Claude in Chrome](https://code.claude.com/docs/zh-CN/chrome.md) 连接。两者都需要使用 claude.ai 账户登录。需要 Claude Code v2.1.280 或更高版本。
  * 在 Context 部分中选择 **Export conversation**，或输入 `/export`，即可将对话复制为纯文本或保存到文件。添加文件名（例如 `/export notes.txt`）可跳过对话框并选择文件的保存位置。需要 Claude Code v2.1.280 或更高版本。
  * Settings 部分包含 **Enable Remote Control for all sessions**，它会设置 [`remoteControlAtStartup`](https://code.claude.com/docs/zh-CN/settings-reference#remotecontrolatstartup)，用于控制[新的交互式会话是否自动连接到 Remote Control](https://code.claude.com/docs/zh-CN/remote-control.md#enable-remote-control-for-all-sessions)。需要 Claude Code v2.1.203 或更高版本。

    在 VS Code 窗口中打开或关闭此开关时，更改不仅限于之后启动的会话，也会应用于该 VS Code 窗口中已打开的会话。如果将其关闭，已打开的会话会断开连接。在 Claude Code v2.1.261 或更高版本中，更改还会作用于您其他 VS Code 窗口中打开的会话。
  * Settings 部分还包含 **Focus view**，它会将工具调用、工具结果和思考内容隐藏在可展开的行后面，只保留您的提示词和 Claude 的回复。您可以在此处切换，也可以使用 `Ctrl+Option+F`（Mac）/ `Ctrl+Alt+F`（Windows/Linux），或在命令面板中使用 **Claude Code: Toggle Focus view**。更改会应用于所有已打开的会话，并在会话之间保持。需要 Claude Code v2.1.221 或更高版本。

    Claude 最新的待办事项列表保持可见，Claude 待回答问题所涉及的文本也保持可见；这需要 Claude Code v2.1.225 或更高版本。当 Claude 运行[子代理](https://code.claude.com/docs/zh-CN/sub-agents.md)时，显示其最新活动的实时进度行会出现在启动它们的工具调用组下方。这需要 Claude Code v2.1.269 或更高版本。
  * 要注销您的 Anthropic 账户，请在 Settings 部分中选择 **Sign out**，或输入 `/logout`。使用[第三方提供商](#use-third-party-providers)时，菜单不提供这两种方式。需要 Claude Code v2.1.277 或更高版本。
  * 要报告错误，请点击菜单底部的 **Report a problem**，或输入 `/bug` 或 `/feedback`，可附带一段描述来预填报告。当您提交报告，且通过第一方连接登录到 Anthropic 时，Claude Code 会将其发送给 Anthropic。需要 Claude Code v2.1.229 或更高版本。

    使用第三方提供商或没有 Anthropic 凭据时，不会发送任何内容。对话框会在您撰写之前说明这一点。提交时会将报告保存为[位于 `~/.claude/feedback-bundles/` 下的本地归档文件](https://code.claude.com/docs/zh-CN/data-usage#telemetry-services)，其中已知的 API 密钥和令牌模式会被遮盖。将报告保存到您的计算机需要 Claude Code v2.1.284 或更高版本。

    报告保存后，确认信息会显示文件名，并包含 **Show folder** 按钮。请将该文件发送给您的 Anthropic 客户代表，或将其附加到支持请求中。

    如果您组织的策略关闭了产品反馈，**Report a problem** 不会出现在菜单中，`/bug` 和 `/feedback` 会显示 `Feedback is turned off by your organization's policy or this environment's settings.` 通知，而不是打开报告。在 Claude Code v2.1.284 或更高版本中，如果您设置了 `DISABLE_FEEDBACK_COMMAND` 或 `CLAUDE_CODE_DISABLE_NONESSENTIAL_TRAFFIC` 环境变量，反馈同样会被关闭，打开报告时会改为显示该通知。
* **旁支问题**：输入 `/btw` 后跟一个问题，即可就您的会话提问，而[不会添加到对话中](https://code.claude.com/docs/zh-CN/interactive-mode.md#side-questions-with-%2Fbtw)。回答会在聊天旁边的面板中打开，您可以在其中继续追问。需要 Claude Code v2.1.227 或更高版本。

  该线程在窗口重新加载后依然保留。Claude Code 会保留最新的 20 轮问答，并按照 [`cleanupPeriodDays`](https://code.claude.com/docs/zh-CN/settings-reference#cleanupperioddays) 计划使已存储的线程过期，前提是 Claude Code 能够[安全地确定保留期限](https://code.claude.com/docs/zh-CN/claude-directory.md#cleaned-up-automatically)。要清除线程，请点击面板中的垃圾桶图标。
* **复制回复**：将鼠标悬停在回复上并点击 **Copy response** 即可将其复制到剪贴板，或输入 `/copy` 复制最新的回复。`/copy 2` 复制倒数第二条回复。需要 Claude Code v2.1.277 或更高版本。
* **书签**：将鼠标悬停在回复上并点击 **Bookmark response** 即可保存该回复，或在已保存的回复上点击 **Remove bookmark** 将其移除。

  要查看已保存的回复，请打开 Bookmarks 面板。您可以点击 Claude Code 面板顶部的书签图标，在命令菜单的 Context 部分中选择 **Bookmarks**，或输入 `/bookmarks`。需要 Claude Code v2.1.286 或更高版本。
* **Claude 发送给您的文件**：当会话连接到 [Remote Control](https://code.claude.com/docs/zh-CN/remote-control.md#start-a-remote-control-session)，且 Claude 使用 [`SendUserFile` 工具](https://code.claude.com/docs/zh-CN/tools-reference)向您发送文件时，对话中会显示一行，例如 **Sent report.md, chart.png**。点击文件名即可在编辑器中打开。该行在 Focus view 中保持可见。需要 Claude Code v2.1.295 或更高版本。

  当文件路径指向某个网络主机时，该文件会显示为无法点击的完整路径，例如 `\\server\share\report.md`。Claude Code 不会为该文件提供链接，因为打开它会连接该主机，并且在 Windows 上[可能会将您的 Windows 凭据发送到该主机](https://code.claude.com/docs/zh-CN/permissions.md#read-only-commands)。
* **上下文指示器**：输入框会显示您已使用了多少 Claude 上下文窗口。Claude 会在需要时自动压缩，您也可以手动运行 `/compact`。
* **提示缓存时钟**：上下文指示器旁边的时钟图标会估算对话的[提示缓存](https://code.claude.com/docs/zh-CN/prompt-caching.md)在过期前还剩多少时间。它从缓存的五分钟或一小时[生命周期](https://code.claude.com/docs/zh-CN/prompt-caching.md#cache-lifetime)开始倒计时，每次使用缓存的回复都会重新开始倒计时。除压缩外，[使缓存失效的操作](https://code.claude.com/docs/zh-CN/prompt-caching.md#actions-that-invalidate-the-cache)不会重置时钟，因此在您切换模型后它仍可能显示剩余的分钟数。
  * 在倒计时结束之前，图标会显示剩余分钟数，例如 **12m**。
  * 倒计时结束后，分钟数会消失，图标变为红色（或您主题的错误颜色），直到下一条回复。此时缓存很可能已经过期，因此在缓存重建期间，您下一条消息得到的回复会更慢、成本更高。如果五分钟的生命周期总是在您发送消息之间耗尽，请参阅[自行选择 TTL](https://code.claude.com/docs/zh-CN/prompt-caching.md#choose-the-ttl-yourself)。
  * 对话刚被[压缩](https://code.claude.com/docs/zh-CN/prompt-caching.md#compacting-the-conversation)后，图标同样会变为红色且不显示分钟数，直到下一条回复，因为缓存尚未覆盖压缩后的对话。
* **Agent 地图**：当对话包含[子代理](https://code.claude.com/docs/zh-CN/sub-agents.md)时，输入框底部会出现 Agent 计数，例如 **2 agents**。其圆点表示是否有子代理正在工作或正在等待您授予权限。

  点击 Agent 计数即可打开 Agent 地图，它以树状结构在主 Agent 下绘制对话中的子代理，每个子代理都附带其状态、已用时间和 token 数。点击某个子代理即可查看其提示词和工具调用、打开其只读会话记录，或在其运行时将其停止。需要 Claude Code v2.1.269 或更高版本。

  地图还会在 Agent 下方列出会话的其他[后台任务](https://code.claude.com/docs/zh-CN/tools-reference.md#background-commands)，例如后台 shell 命令和[监视器](https://code.claude.com/docs/zh-CN/tools-reference.md#monitor-tool)。点击某一行即可打开该任务的卡片，并在其中将其停止。

  如果没有显示 Agent 计数（例如 Claude 启动了后台 shell 但没有子代理），要打开地图，请在输入框中输入 `/tasks`。地图中的后台任务和输入的 `/tasks` 需要 Claude Code v2.1.277 或更高版本。

  在 Claude Code v2.1.286 或更高版本中，当您点击 **Stop** 或按 `Esc` 时，当前轮次会结束。后台 Agent 会继续运行，直到完成或您从地图中将其停止。
* **扩展思考**：让 Claude 花更多时间推理复杂问题。要将其关闭或重新打开，请使用命令菜单（`/`）Model 部分中的 **Thinking** 开关。在[无法关闭思考](https://code.claude.com/docs/zh-CN/model-config.md#extended-thinking)的模型上，菜单中没有 **Thinking** 开关，思考会始终保持开启。

  仅显示标签（例如 `Thought for 12s`）的思考块不包含文本，点击时也不会展开。要查看思考摘要，请在 `~/.claude/settings.json` 中将 [`showThinkingSummaries`](https://code.claude.com/docs/zh-CN/settings-reference#showthinkingsummaries) 设置为 `true`，然后从命令面板运行 **Developer: Reload Window**。

  已有的思考块仍保持为空。在重新加载之后的回复中，点击某个块或按 `Ctrl+O` 即可将它们全部展开。
* **多行输入**：按 `Shift+Enter` 可添加新行而不发送。这在问题对话框的"Other"自由文本输入中同样适用。

### 引用文件和文件夹

使用 @ 提及为 Claude 提供有关特定文件或文件夹的上下文。当您输入 `@` 后跟文件或文件夹名称时，Claude 会读取该内容，并可以回答相关问题或对其进行更改。Claude Code 支持模糊匹配，因此您可以输入部分名称来查找所需内容：

```text wrap theme={null}
Explain the logic in @auth (fuzzy matches auth.js, AuthService.ts, etc.)
What's in @src/components/ (include a trailing slash for folders)
```

### 读取 PDF 的特定页面

对于大型 PDF，您可以让 Claude 读取特定页面而不是整个文件。可以请求单个页面、诸如第 1-10 页的范围，或诸如第 3 页起的开放范围。读取特定页面需要在运行 Claude Code 的计算机上安装 [poppler-utils](https://code.claude.com/docs/zh-CN/tools-reference.md#read-tool-behavior)。

### 在提示词中包含终端输出

在提示词中使用 `@terminal:name` 引用终端输出，其中 `name` 是终端的标题。这样 Claude 无需复制粘贴即可看到命令输出、错误消息或日志。

### 附加图片和文件

* 要附加图片，请将其从剪贴板粘贴到输入框中。
* 要附加文件，请按住 `Shift` 将其拖入输入框。
* 要从上下文中移除附件，请点击附件上的 X。

### 粘贴文本

您粘贴的文本会在输入框中保持可见，而不会像[在终端中](https://code.claude.com/docs/zh-CN/terminal-config.md#paste-large-content)那样折叠为占位符。在 Claude Code [标记粘贴文本](https://code.claude.com/docs/zh-CN/terminal-config.md#how-claude-treats-pasted-text)的会话中，Claude 仍会将大段粘贴内容视为您粘贴而非输入的文本。

Claude Code 还会从您粘贴到输入框中的文本以及您发送的任何其他内容中移除[不可见 Unicode 字符](https://code.claude.com/docs/zh-CN/interactive-mode.md#invisible-characters-in-prompts)：

* 如果粘贴时出现 `Removed 3 invisible characters from the pasted text` 之类的通知，则表示文本已去除这些字符后放入输入框。
* 如果发送时出现有关已移除字符的通知，则表示没有发送任何内容。清理后的文本会回到输入框中。再次发送即可按显示的内容发送文本。

### 根据所选内容和打开的文件自动提供上下文

当您在编辑器中选择文本时，Claude 会自动看到您高亮显示的代码。输入框底部会显示已选择的行数。即使没有选择任何内容，Claude 也能看到您在编辑器中打开的是哪个文件，输入框会显示其名称。

按 `Option+K`（Mac）/ `Alt+K`（Windows/Linux）可插入包含文件路径和行号的 @ 提及，例如 `@app.ts#5-10`。

#### 不让所选内容和打开的文件进入您的消息

要阻止所选文本或打开的文件进入您的消息，请在 VS Code 的设置中关闭相应的设置项：

* **所选文本**：关闭 [Attach Selection 设置](vscode://settings/claudeCode.attachSelection)。需要 Claude Code v2.1.300 或更高版本。要移除某一次选择，请点击行数旁边的 **X**。当您选择其他文本时，行数会重新出现。
* **打开的文件**：关闭 [Attach Open File 设置](vscode://settings/claudeCode.attachOpenFile)。需要 Claude Code v2.1.271 或更高版本。除非您的消息包含您在该文件中所选的文本，否则该文件不会被包含在内。

在关闭 Attach Selection 的情况下，要将文本作为 @ 提及添加，请选择该文本，然后在 Mac 上按 `Option+K`，在 Windows 和 Linux 上按 `Alt+K`。

在 VS Code 的集成终端中运行 `claude` 时，这两个设置同样适用。

#### 来自排除或忽略文件的所选文本

在聊天面板中，扩展不会发送某些文件中的所选文本：

* 当文件位于您的工作区内且匹配您的 `files.exclude` 或 `search.exclude` 设置时，Claude 最多只会收到文件路径，而不会收到您所选的文本。
* 这同样适用于被 git 忽略的文件，前提是 VS Code 的 `search.useIgnoreFiles` 设置和扩展的 [`respectGitIgnore` 设置](#extension-settings)都已打开（默认如此）。

此筛选仅适用于聊天面板。当 Claude Code 在集成终端中运行时，Claude 可以收到您在这些文件中所选的文本。要在终端中阻止 Claude 获取文件内容，请添加 [`Read` 拒绝规则](https://code.claude.com/docs/zh-CN/permissions#read-and-edit)。

### 回复中的 LaTeX 数学公式

聊天面板会将 Claude 回复中的 LaTeX 数学公式显示为排版后的公式。数学公式渲染需要 Claude Code v2.1.301 或更高版本。在[终端模式](#switch-to-terminal-mode)下，数学公式不会被排版。面板会同时排版独立公式和行内公式：

* **独立公式**：独占一行或多行的 `$$ … $$` 或 `\[ … \]` 块，以及语言为 `math` 的围栏代码块
* **行内公式**：在同一行内开始和结束的 `$…$` 或 `\(…\)`，例如 `$E = mc^2$` 或 `\(a^2 + b^2 = c^2\)`

面板会尝试区分数学公式与相同字符的其他用法，因此 `$HOME`、`$5` 之类的价格或 `sed 's/\(foo\)/\1/'` 等文本不会被排版。代码片段内的文本，以及除 `math` 围栏代码块以外的任何代码块中的文本，也不会被排版。如果某个公式本应被排版却按原样显示，请参阅[公式显示为原始 LaTeX](#a-formula-shows-as-raw-latex)。

当您选择并复制包含已排版公式的文本时，得到的是以 `$…$` 或 `$$…$$` 分隔符包裹的公式 TeX 源码，而不是绘制出的符号。[**Copy response**](#use-the-prompt-box) 和 `/copy` 会按 Claude 所写的原样提供回复，包括 TeX 源码。

### 恢复过去的对话

点击 Claude Code 面板顶部的 **Session history** 按钮即可访问您的对话历史记录。您可以按关键字搜索或按时间浏览。

点击任意对话即可将其恢复。如果该对话已在当前窗口的另一个标签页中打开，点击它会切换到该标签页。如果通知提示该对话仍在其他位置打开，请参阅[如何在此窗口中继续该对话](#the-conversation-is-still-open-somewhere-else)。如果对话打开后没有显示任何消息，请参阅[为什么恢复的对话一开始显示为空白](#a-resumed-conversation-is-blank-at-first)。有关恢复会话的更多信息，请参阅[管理会话](https://code.claude.com/docs/zh-CN/sessions.md)。

* **会话标题**：新会话会根据您的第一条消息获得 AI 生成的标题。
* **重命名和归档**：将鼠标悬停在会话上即可显示这些操作。重命名可为其设置描述性标题，归档则会将其移至列表底部的 **Archived sessions** 组。
* **设置了 [`claudeProcessWrapper`](#extension-settings) 时对话缺失**：请将扩展更新到 2.1.304 或更高版本。之后列表会包含之前缺失的对话，以及 `claude -p` 或 [Agent SDK](https://code.claude.com/docs/zh-CN/agent-sdk/overview.md) 在此工作区中启动的会话。

默认情况下，14 天内没有活动的会话会自动移至 **Archived sessions**，除非它处于打开状态、未读或位于某个[分组](#organize-sessions-into-groups)中。自动归档需要 Claude Code v2.1.265 或更高版本。要更改期限或将其关闭，请打开 [Archive Inactive Sessions 设置](vscode://settings/claudeCode.archiveInactiveSessions)，并选择天数或 **Never**。

要恢复已归档的会话：

* **单个会话**：展开 **Archived sessions** 并点击 **Unarchive session**。
* **一次性恢复所有已归档会话**：将鼠标悬停在活动栏会话列表中的 **Archived sessions** 标题上，然后点击其取消归档图标。需要 Claude Code v2.1.277 或更高版本。

在 v2.1.257 之前，该操作是 **Delete session**，它会隐藏会话且无法恢复。升级后，您当时删除的会话会出现在 **Archived sessions** 下。

当您恢复的对话以计划模式结束时，Claude Code 会恢复计划模式。需要 Claude Code v2.1.246 或更高版本。在以下情况下，Claude Code 不会恢复计划模式：

* 扩展根据 `claudeCode.initialPermissionMode` 或从之前对话沿用的选择来[选择初始权限模式](https://code.claude.com/docs/zh-CN/permission-modes.md#switch-permission-modes)
* 您配置了 `claudeCode.claudeProcessWrapper`
* 某条[拒绝规则](https://code.claude.com/docs/zh-CN/permissions.md#manage-permissions)移除了 [`ExitPlanMode`](https://code.claude.com/docs/zh-CN/tools-reference) 工具

#### 对话仍在其他位置打开

当通知提示该对话仍在其他位置打开时，要在此窗口中继续，请先完成与您情况对应的步骤，然后点击 **Open here anyway**：

* **您没有在其他位置打开任何内容**：扩展重启之前的某个 Claude 进程可能仍打开着该对话。该进程通常会在大约 10 分钟内自行结束，因此请等待这段时间后再点击。如果在它结束之前点击，该对话会在两处同时打开，直到该进程结束。
* **您在其他位置打开了该对话**：请先在那里将其关闭，例如在终端中退出 `claude`，或在另一个 VS Code 窗口中关闭其标签页

当该对话仍在另一个 Claude Code 进程中打开时，输入框的位置会显示以下通知：

```text theme={null}
This conversation is still open somewhere else. Using it in two places at once can mix up its messages. If the extension restarted recently, a Claude process from before the restart may still be finishing. It usually ends on its own within about 10 minutes. This notice stays until you choose Open here anyway.
```

设置了 [`claudeProcessWrapper`](#extension-settings) 时，扩展会跳过此检查并直接打开对话。

### 从 claude.ai 恢复云端会话

如果您运行[云端会话](https://code.claude.com/docs/zh-CN/claude-code-on-the-web.md)，可以直接在 VS Code 中恢复它们。这需要使用 **Claude.ai Subscription** 登录，而不是 Anthropic Console。

**1. 打开会话历史记录**

点击 Claude Code 面板顶部的 **Session history** 按钮。

**2. 选择 Web 标签页**

对话框会显示两个标签页：Local 和 Web。点击 **Web** 即可查看来自 claude.ai 的会话。

**3. 选择要恢复的会话**

浏览或搜索会话。点击其中一个即可在本地继续对话。

> [!NOTE]
>
> 当您打开的文件夹是 GitHub 仓库时，Web 标签页仅显示来自该仓库的会话。
>
> 当您恢复云端会话时，扩展会下载对话历史记录的副本；所做的更改不会同步回 claude.ai。

Web 标签页还会列出您的 [Remote Control](https://code.claude.com/docs/zh-CN/remote-control.md) 会话。如果您点击的会话是在您当前打开的文件夹中运行的，扩展会直接打开该本地对话，而不是下载副本。如果已有标签页显示该对话，扩展会聚焦到该标签页。如果扩展无法排除另一个 Claude 进程正打开该对话的可能性，您将得到一份下载的副本。

如果对话的任何部分下载失败，会显示错误且不会保存副本。再次选择该会话即可重试。如果您选择的会话尚无可下载的对话，错误信息会告诉您应改为在何处继续该会话。

### 查看账户和用量

运行 `/usage` 可打开 Account & usage 对话框，其中显示您已登录的账户和用量。用量数据与您的登录方式相对应：

* **claude.ai 套餐**：显示套餐各项限额的用量条，例如当前会话和本周。

  对话框还会细分哪些因素在消耗您的套餐限额：

  * **建议**：建议进行的更改，每行一条，例如 `Trim ~/code/api/CLAUDE.md · 49k tokens per request`
  * **修复按钮**：要让 Claude Code 应用它能处理的建议，请点击建议下方的按钮，例如 **Fix both**。要了解该按钮作用于哪些行以及随后会在您的会话中发生什么，请参阅[使用修复快捷方式应用建议](https://code.claude.com/docs/zh-CN/costs.md#apply-recommendations-with-the-fix-shortcut)。需要 Claude Code v2.1.305 或更高版本。

    连接 [Remote Control](https://code.claude.com/docs/zh-CN/remote-control.md) 期间不会显示该按钮。如果在 Remote Control 连接后出现 `The usage fix wasn’t fully sent. Open usage and try again.` 警告，请先[断开 Remote Control](https://code.claude.com/docs/zh-CN/remote-control.md#check-connection-status)，然后再重试。
  * **归因**：显示每个 skill、子代理、插件和 MCP 服务器分别产生了多少用量的表格

  使用 Day 和 Week 切换可在过去 24 小时和过去 7 天之间切换。这些数据是近似值，根据本机上的本地会话计算得出，因此不包括来自其他设备或 claude.ai 的用量。
* **其他登录方式**：当套餐限额不适用于您的登录方式时（例如使用[第三方提供商](#use-third-party-providers)或 API 密钥），Usage 部分会改为显示该会话自身的费用和 token 用量。CLI 的 `/usage` 会在其 [Session 区块](https://code.claude.com/docs/zh-CN/costs.md#track-your-costs)中显示相同的总计。活动栏中的会话列表也会在其 **Account & usage** 标题下显示当前会话的总计。需要 Claude Code v2.1.277 或更高版本。

有关跟踪和减少用量的更多信息，请参阅[跟踪您的费用](https://code.claude.com/docs/zh-CN/costs.md#track-your-costs)。

### 使用检查点回退

VS Code 扩展支持检查点，它会跟踪 Claude 的文件编辑，并允许您回退到之前的状态。将鼠标悬停在任意消息上即可显示回退按钮，然后从以下三个选项中选择：

* **Fork conversation from here**：从此消息开始一个新的对话分支，同时保留所有代码更改
* **Rewind code to here**：将文件更改还原到对话中的此时间点，同时保留完整的对话历史记录
* **Fork conversation and rewind code**：开始一个新的对话分支，并将文件更改还原到此时间点

有关检查点工作原理及其限制的完整详情，请参阅[检查点功能](https://code.claude.com/docs/zh-CN/checkpointing.md)。

## 自定义您的工作流

您可以调整 Claude 面板的位置、运行多个对话、对会话列表进行分组或筛选，或切换到终端模式。

### 选择 Claude 的位置

您可以拖动 Claude 面板，将其放置在 VS Code 中的任意位置。抓住面板的标签页或标题栏并将其拖动到：

* **辅助侧边栏**：窗口右侧。编码时 Claude 始终可见。
* **主侧边栏**：左侧带有资源管理器、搜索等图标的侧边栏。
* **编辑器区域**：将 Claude 作为标签页与您的文件并排打开。适用于辅助任务。

当 Claude 在新的编辑器组中打开标签页时，扩展会锁定该组。之后在 Claude 标签页获得焦点时打开的文件会进入另一个组，而不会在其旁边打开。

若要阻止扩展锁定编辑器组，请关闭 [Lock Editor Groups 设置](vscode://settings/claudeCode.lockEditorGroups)。已锁定的组会保持锁定，直到您手动解锁。该设置需要 Claude Code v2.1.274 或更高版本。

> [!TIP]
>
> 将侧边栏用于主要的 Claude 会话，并为辅助任务打开额外的标签页。Claude 会记住您偏好的位置。活动栏中的会话列表图标与 Claude 面板是分开的。会话列表始终显示在活动栏中，而 Claude 面板图标仅在面板停靠到左侧边栏时才会出现在那里。

### 重新加载后继续对话

在运行 **Developer: Reload Window** 或重启 VS Code 后，请查看对话原先打开的位置：

* **编辑器标签页**：对话会随其标签页一起恢复。
* **侧边栏**：如果您在最近 10 分钟内在该对话中发送过消息或 Claude 在其中做出过响应，对话会恢复。如果未恢复，请从[会话历史记录](#resume-past-conversations)中恢复该对话。

如果输入框所在位置显示一条通知，提示该对话仍在其他地方打开，请参阅[如何在此处继续该对话](#the-conversation-is-still-open-somewhere-else)。如果消息起初保持空白，请参阅[消息为何空白以及何时出现](#a-resumed-conversation-is-blank-at-first)。

#### 重新加载后被中断的步骤

如果重新加载在 Claude 执行某个步骤的过程中将其中断，对话恢复后 Claude 会继续该步骤，并在聊天中显示一条通知来标记此次继续。需要 Claude Code v2.1.274 或更高版本。如果该步骤是在一小时以前被中断的，或者该会话在其他地方打开，对话恢复后将处于空闲状态。

#### 关闭重新加载后的继续执行

若要阻止 Claude 在重新加载后继续被中断的步骤，请打开 [Continue After Reload 设置](vscode://settings/claudeCode.continueAfterReload)并取消勾选。

如果您在 VS Code 的环境中或在 [`environmentVariables` 设置](#extension-settings)中设置了 [`CLAUDE_CODE_RESUME_INTERRUPTED_TURN`](https://code.claude.com/docs/zh-CN/env-vars#variables) 或任何其他 `CLAUDE_CODE_RESUME_` 变量，它在面板中不会生效。这是因为扩展在启动面板的会话之前会移除这些变量。

#### 对话未被重新打开

重新加载后，如果出现一个新聊天，且其输入框上方显示类似如下的通知，则表示面板未重新打开您的对话，因为该对话最近两次打开均未完成：

```text theme={null}
“Fix the login bug” was not reopened. It did not finish opening the last 2 times, and opening it again may stop Claude Code from responding here.
```

当面板在加载对话时崩溃或停止响应，或者您在对话打开后约三秒内重新加载或关闭窗口时，打开操作就不会完成。该对话不会发生变化，并仍保留在[会话历史记录](#resume-past-conversations)中。

* **仍要打开**：点击 **Open it anyway** 或从会话历史记录中选择该对话。对话打开完成后，之后重新加载时它会[照常恢复](#continue-conversations-after-a-reload)。
* **如果面板在加载对话时停止响应**：重新加载窗口。随后会打开一个新聊天。
* **保持关闭**：继续在新聊天中工作。发送提示词后，该通知即会消失。

### 运行多个对话

从命令面板中使用 **Open in New Tab** 或 **Open in New Window** 来启动更多对话。每个对话都维护各自的历史记录和上下文，让您可以并行处理不同的任务。

使用标签页时，火花图标上的小彩点表示状态：蓝色表示有待处理的权限请求，橙色表示 Claude 在标签页隐藏期间已完成工作。

### 将会话整理为组

在活动栏的会话列表中，您可以将相关会话收集到已命名、可折叠的组中。需要 Claude Code v2.1.229 或更高版本。

* **对会话进行分组或取消分组**：右键点击某个会话，即可基于它创建组、将其移入现有组，或将其从所在组中移除。每个会话一次只属于一个组，因此将其移入另一个组时会将其从原来的组中移除。
* **一次移动多个会话**：`Cmd`-点击（Mac）/ `Ctrl`-点击（Windows/Linux）每个会话，或 `Shift`-点击以选择一个范围，然后右键点击所选内容。
* **从标签页对会话进行分组**：从命令面板运行 **Claude Code: Add Session Tab to Group**，然后选择或创建一个组。需要 Claude Code v2.1.257 或更高版本。
* **重命名或删除组**：右键点击组标题。删除组只会移除该组本身，其中的会话会回到未分组列表。

扩展按工作区文件夹保存组，因此组在窗口重新加载后依然保留，并会出现在您打开同一文件夹的每个窗口中。当您搜索列表时，扩展会将所有组中的匹配项显示在一个平铺列表中。

### 筛选会话列表

若要缩小活动栏中较长的会话列表，请使用列表顶部的两个筛选控件。需要 Claude Code v2.1.271 或更高版本。启用任一筛选时，已归档的会话都不会显示。

* **Active**：打开此开关后，仅显示需要您输入、正在工作或未读的会话，以及您上次聚焦的 Claude 标签页中的会话。
* **按状态筛选**：点击漏斗图标，然后勾选 **Needs input**、**Working** 或 **Completed**，以显示处于这些状态中任一状态的会话。勾选 **Open** 或 **Closed**，可按会话是否处于打开状态进行筛选。当会话在此窗口中有标签页，或正在本机上的另一个 Claude Code 进程（例如终端中）运行时，该会话即视为处于打开状态。

当 **Active** 处于打开状态且您勾选了某个状态、**Open** 或 **Closed** 时，列表还会显示与您的勾选项匹配的所有会话。您设置的筛选条件在窗口重新加载后依然保留。

### 切换到终端模式

默认情况下，扩展会打开图形化聊天面板。如果您更喜欢 CLI 风格的界面，请打开 [Use Terminal 设置](vscode://settings/claudeCode.useTerminal)并勾选该复选框。

您也可以在 Mac 上按 `Cmd+,` 或在 Windows/Linux 上按 `Ctrl+,` 打开 VS Code 设置，转到 **Extensions > Claude Code**，然后勾选 **Use Terminal**。

## 管理插件

VS Code 扩展提供了一个图形界面，用于安装和管理[插件](https://code.claude.com/docs/zh-CN/plugins/overview.md)。在输入框中键入 `/plugins` 即可打开 **Manage plugins** 界面。

### 安装插件

插件对话框显示两个标签页：**Plugins** 和 **Marketplaces**。

在 Plugins 标签页中：

* **已安装的插件**显示在顶部，并带有用于启用或禁用它们的切换开关。
  * 如果您关闭了项目共享的 `.claude/settings.json` 中启用的插件，扩展会先进行询问：**Disable for me** 仅为您关闭该插件，而 **Disable for everyone** 会修改共享文件。
  * 加载失败的插件会在其所在行显示简短原因。点击该原因可查看您可以采取的措施，包括复制完整的错误消息，以便在[插件故障排除](https://code.claude.com/docs/zh-CN/plugins/troubleshooting.md)中查找。
* 来自您已配置市场的**可用插件**显示在下方
* 通过搜索可按名称或描述筛选插件
* 在任意可用插件上点击 **Install**

安装插件时，请选择安装作用域：

* **Install for you**：在您的所有项目中可用（用户作用域）
* **Install for this project**：与项目协作者共享（项目作用域）
* **Install locally**：仅对您可用，且仅在此仓库中可用（本地作用域）

安装完成后，会出现一个表单，要求填写插件尚未设置的[配置选项](https://code.claude.com/docs/zh-CN/plugins/components.md#user-configuration)。如需稍后查看或更改这些选项，请点击插件所在行的齿轮图标。

敏感文本字段会被遮盖，之前保存的密钥会显示 **(unchanged)**。将字段留空即可保留已保存的值。保存更改后，已打开的会话会重新加载其插件，对话框会显示 **Restart Claude to apply plugin changes**。

### 卸载插件

每个已安装的行都会注明其安装所在的[作用域](https://code.claude.com/docs/zh-CN/plugins/install.md#choose-an-install-scope)。要卸载该安装，请点击该行的垃圾桶图标。垃圾桶图标变暗表示该行无法从当前工作区卸载，例如由您的组织管理的插件，或为其他项目安装的插件。

扩展会在以下两种情况下先进行询问：

* **项目共享的 `.claude/settings.json` 中启用的插件**：选择 **Disable for me**，这会为您的协作者保留该插件的安装；或选择 **Uninstall for everyone**，这会使用 [`--keep-data`](https://code.claude.com/docs/zh-CN/plugins/cli-reference#what-an-uninstall-deletes-and-keeps) 移除项目的安装，因此插件保存的数据目录会保留下来。如果您已经为自己关闭了该插件，垃圾桶图标会直接移除您自己的安装，而不再询问。
* **其他情况下，带有已保存数据的插件的最后一个安装**：选择保留还是删除数据；默认为 **Keep**

### 分享插件安装链接

如需让他人直接安装某个特定插件，请向其提供扩展的 `install-plugin` URL。打开该 URL 会启动或聚焦 VS Code，打开 Claude Code 面板，并在该插件的作用域选择界面打开 **Manage plugins** 对话框。在对方选择作用域之前，不会安装任何内容。如果对方的 Claude Code 中尚未配置该插件的市场，对话框会先要求对方添加该市场。

```text theme={null}
vscode://anthropic.claude-code/install-plugin?plugin=code-review&marketplace=anthropics/claude-plugins-official
```

该 URL 接受两个查询参数：

| 参数 | 描述 |
| - | - |
| `plugin` | 插件在其市场中列出的名称。必需。 |
| `marketplace` | 市场的[来源](https://code.claude.com/docs/zh-CN/plugins/install.md#add-a-marketplace)：GitHub `owner/repo`、`https://` URL，或 git SSH 地址（例如 `git@github.com:owner/repo.git`）。省略时默认为 `anthropics/claude-plugins-official`。 |

扩展在打开任何内容之前会检查这两个值：

* **插件名称**：最多 100 个字符，以 ASCII 字母或数字开头，其余部分只能使用 ASCII 字母、数字、`.`、`_` 和 `-`。
* **市场来源**：仅限 `marketplace` 参数所列的形式，因此不能是本地路径、`http://` 地址或市场名称（例如 `claude-plugins-official`）。`https://` URL 不能包含用户名、密码或查询字符串。
* **Git ref**：要将市场固定到某个分支或标签，请在来源后追加 `%23`（`#` 的编码形式）和 ref，例如 `marketplace=owner/repo%23v1.0`。包含未编码 `#` 的链接会失败。`anthropics` GitHub 组织中的市场无法在链接中固定。

打开违反这些规则的链接时，用户会看到以 `Invalid plugin installation URL` 开头的错误。Claude Code 面板和对话框都不会打开，也不会安装任何内容。如果您的插件名称或市场无法放入链接中，请告知用户在 **Marketplaces** 标签页中添加该市场，然后从 **Plugins** 标签页安装该插件。

以下情况会在对话框中显示一条消息，而不是显示作用域选择：

* **市场中没有该名称的插件**：对话框会报告未找到该插件。请对照市场的列表检查 `plugin` 值。
* **插件已安装**：对话框会说明这一点，且不做任何更改。
* **已添加了另一个同名市场**：对话框会说明未添加链接中的市场，且不会安装任何内容。

GitHub README、issue 以及其他一些 Markdown 托管平台会移除 scheme 不是 `http` 或 `https` 的链接，因此 `vscode://` 链接在这些平台上会显示为纯文本。在这些平台上，请将 URL 放入代码块中，正如[链接显示为纯文本而不可点击](https://code.claude.com/docs/zh-CN/deep-links.md#the-link-renders-as-plain-text-instead-of-being-clickable)中针对 `claude-cli://` 链接所述。

### 管理市场

切换到 **Marketplaces** 标签页以添加或移除插件来源：

* 输入 GitHub 仓库、URL 或本地路径以添加新市场
* 点击刷新图标以更新市场的插件列表
* 点击垃圾桶图标以移除市场。移除市场会[卸载您从中安装的所有插件](https://code.claude.com/docs/zh-CN/plugins/install.md#manage-marketplaces)，因此会先弹出确认提示并列出这些插件

您在对话框中进行的插件更改会立即应用于该 VS Code 窗口中已打开的 Claude Code 会话。

如果您打开对话框时所在的会话无法重新加载其插件，对话框会提供重试或在该会话中重启 Claude 的选项。

> [!NOTE]
>
> 扩展通过运行与 CLI 相同的命令来管理插件。您在扩展中配置的插件和市场在 CLI 中同样可用，反之亦然。

有关插件系统的更多信息，请参阅[插件](https://code.claude.com/docs/zh-CN/plugins/overview.md)和[插件市场](https://code.claude.com/docs/zh-CN/plugins/overview.md)。

### 在面板中绘制内容的插件

包含 [mod](https://code.claude.com/docs/zh-CN/plugins/mods/overview.md) 的插件可以在聊天面板中绘制内容。绘制功能需要扩展版本 2.1.299 或更高版本。[不获取功能标志](https://code.claude.com/docs/zh-CN/env-vars.md#features-that-need-feature-flag-fetching)的会话（例如在 Amazon Bedrock 上运行的会话）需要 2.1.304 或更高版本。您可以从此类插件中看到：

* 对话旁边或输入框上方的窗格
* 输入框上方的 toast 通知或状态栏
* 以插件自己的方式绘制的消息或工具调用

当您安装的插件在面板中添加或更改内容时，会有一个标签注明该插件，例如以下措辞：

* **From the token-chart plugin**：该插件绘制或输出了此内容，或者此卡片是该插件在向您询问某些事项
* **Edited by the token-chart plugin**：该插件更改了 Claude Code 绘制的某些内容，例如工具调用
* **Prompt from the token-chart plugin**：该插件代表您发送了此消息

要阻止插件绘制内容，请在[插件对话框](#install-plugins)中关闭该插件。

## 使用 Chrome 自动执行浏览器任务

将 Claude 连接到您的 Chrome 浏览器，即可在不离开 VS Code 的情况下测试 Web 应用、借助控制台日志进行调试，以及自动化浏览器工作流。此功能需要 [Claude in Chrome 扩展程序](https://chromewebstore.google.com/detail/claude/fcoeoabgfenejglbffodgkkbkcdhcgfn) 1.0.36 或更高版本。

在输入框中输入 `@browser`，然后输入您希望 Claude 执行的操作：

```text wrap theme={null}
@browser go to localhost:3000 and check the console for errors
```

您也可以打开附件菜单，选择特定的浏览器工具，例如打开新标签页或读取页面内容。

Claude 会为浏览器任务打开新标签页，并共享您浏览器的登录状态，因此它可以访问您已登录的任何网站。

有关使用浏览器的更多信息，请参阅：

* [默认启用 Chrome](https://code.claude.com/docs/zh-CN/chrome.md#enable-chrome-by-default)：让每个会话在启动时自动连接到您的浏览器，无需输入 `@browser`
* [VS Code 会话中的权限提示](https://code.claude.com/docs/zh-CN/chrome.md#permission-prompts-in-vs-code-sessions)：Claude Code 何时会在执行浏览器操作前询问您
* [将 Claude Code 与 Chrome 配合使用](https://code.claude.com/docs/zh-CN/chrome.md)：设置说明、完整的功能列表以及故障排除

## 使用 git

Claude Code 与 git 集成，帮助直接在 VS Code 中进行版本控制工作流。要求 Claude 提交更改、创建拉取请求或跨分支工作。要在具有自己的文件和分支的隔离 worktree 中启动 Claude，请参阅 [使用 worktrees 运行并行会话](https://code.claude.com/docs/zh-CN/worktrees.md)。

### 创建提交和拉取请求

Claude 可以暂存更改、编写提交消息，并根据您的工作创建拉取请求：

```text wrap theme={null}
commit my changes with a descriptive message
create a pr for this feature
summarize the changes I've made to the auth module
```

创建拉取请求时，Claude 会根据实际代码更改生成描述，并可以添加有关测试或实现决策的上下文。

### 重新打开已移入 worktree 的对话

在重新打开的对话中，**This session is in worktree** 横幅表示该对话的工作目录是横幅中所指的 [worktree](https://code.claude.com/docs/zh-CN/worktrees.md)，而不是此窗口当前打开的文件夹。要在新的 VS Code 窗口中打开该 worktree，请点击横幅上的 **Open worktree**。

例如，当您[要求 Claude 在某个 worktree 中工作](https://code.claude.com/docs/zh-CN/worktrees.md#ask-claude-to-create-a-worktree)时，对话就会移入该 worktree。当您重新打开该对话，或面板[在窗口重新加载后重新打开它](#continue-conversations-after-a-reload)时，如果该 worktree 是由 Claude Code 在 `.claude/worktrees/` 下创建的且仍然存在，对话将在该 worktree 中恢复。在 Claude Code v2.1.300 之前，打开的会是一个空对话。如果您使用的是这些版本，请更新扩展。

## 在后台运行工作

将长时间运行的命令或子代理发送到后台，然后从输入框中查看其状态。

### 将正在运行的命令或子代理移至后台

当某个命令或[子代理](https://code.claude.com/docs/zh-CN/sub-agents.md)让 Claude 等待的时间超出您的预期时，请点击对话中其工具调用下方的 **Run in background**。需要 Claude Code v2.1.287 或更高版本。

* **时机**：命令运行约两秒后，或 Claude 一开始等待其从主对话中生成的[子代理](https://code.claude.com/docs/zh-CN/sub-agents.md#run-subagents-in-foreground-or-background)，该操作即会出现。
* **点击之后**：Claude 停止等待并继续当前轮次。命令或子代理会作为[后台任务](https://code.claude.com/docs/zh-CN/tools-reference.md#background-commands)继续运行，并在完成时通知 Claude。

如需在此期间查看任务状态或停止任务，请在输入框中键入 `/tasks` 以打开 [Agent 地图](#use-the-prompt-box)。子代理会在其中的 Agent 树中保持其位置，而命令会列在各 Agent 下方，其卡片上显示[最新输出](#monitor-background-processes)。

#### 子代理下方缺少 Run in background 操作

在 VS Code 中，子代理默认不显示此操作。要了解某个子代理为何不显示，请在 [Agent 地图](#use-the-prompt-box)中打开其卡片并查看 `Spawned by` 行：

* **`Spawned by the main agent`**：该子代理是在后台启动的，因此 Claude 不会等待它，您无需执行任何操作。如需让 Claude 改为等待子代理，请参阅[让 Claude 等待子代理](https://code.claude.com/docs/zh-CN/sub-agents.md#let-claude-wait-for-a-subagent)。
* **`Spawned by` 后为另一个子代理的名称**：该操作仅对主 Agent 生成的子代理显示，而不会对另一个子代理所等待的[嵌套子代理](https://code.claude.com/docs/zh-CN/sub-agents.md#let-subagents-spawn-their-own-subagents)显示。要提前结束嵌套子代理，请点击其卡片上的 **Stop agent**。等待中的子代理随后会在没有该结果的情况下继续运行。

### 监控后台进程

在输入框中键入 `/tasks` 以打开 [Agent 地图](#use-the-prompt-box)，其中列出了会话的后台任务，例如 Claude 作为后台 shell 命令保持运行的开发服务器。点击某个任务即可打开其卡片，您可以在卡片中停止该任务。需要 Claude Code v2.1.277 或更高版本。

对于后台 shell 命令，或运行命令的[监视器](https://code.claude.com/docs/zh-CN/tools-reference.md#monitor-tool)，卡片还会显示该命令的最新输出，并在命令运行期间持续刷新。

#### 查看后台子代理的工具调用

当某个子代理仅显示为一条状态栏而没有工具调用时，请在输入框中键入 `/tasks` 以打开 [Agent 地图](#use-the-prompt-box)，并在其中点击该子代理。该子代理正在[后台运行](#the-run-in-background-action-is-missing-under-a-subagent)，其卡片会显示其工具调用，以及一个用于打开其会话记录的按钮。

## VS Code 命令和快捷键

要打开命令面板，请在 Mac 上按 `Cmd+Shift+P`，在 Windows/Linux 上按 `Ctrl+Shift+P`。在其中输入"Claude Code"即可查看 Claude Code 扩展的所有可用 VS Code 命令。

某些快捷键取决于当前哪个面板处于"焦点"状态（接收键盘输入）。当光标位于代码文件中时，编辑器处于焦点状态。当光标位于 Claude 的提示词输入框中时，Claude 处于焦点状态。使用 `Cmd+Esc` / `Ctrl+Esc` 在两者之间切换。

> [!NOTE]
>
> 这些是用于控制扩展的 VS Code 命令。并非所有内置的 Claude Code 命令都可在扩展中使用。有关详细信息，请参阅 [VS Code 扩展与 Claude Code CLI 对比](#vs-code-extension-vs-claude-code-cli)。

| 命令 | 快捷键 | 描述 |
| - | - | - |
| Focus Input | `Cmd+Esc` (Mac) / `Ctrl+Esc` (Windows/Linux) | 在编辑器和 Claude 之间切换焦点 |
| Focus last message | - | 将键盘焦点移至对话中的最新消息或正在等待的权限提示，以便您从该位置使用键盘或屏幕阅读器进行阅读。在[终端模式](#switch-to-terminal-mode)下不可用。需要 Claude Code v2.1.268 或更高版本 |
| Open in Side Bar | - | 在侧边栏中打开 Claude |
| Open in Terminal | - | 以终端模式打开 Claude |
| Open in New Tab | `Cmd+Shift+Esc` (Mac) / `Ctrl+Shift+Esc` (Windows/Linux) | 以编辑器标签页的形式打开新对话 |
| Open in New Window | - | 在单独的窗口中打开新对话 |
| New Conversation | `Cmd+N` (Mac) / `Ctrl+N` (Windows/Linux) | 开始新对话。需要 Claude 处于焦点状态，并将 `enableNewConversationShortcut` 设置为 `true` |
| Reopen Closed Session | `Cmd+Shift+T` (Mac) / `Ctrl+Shift+T` (Windows/Linux) | 重新打开最近关闭的 Claude 会话标签页。如果最后关闭的标签页不是 Claude 会话，则回退到 VS Code 的常规"重新打开已关闭的编辑器"功能。可通过 `enableReopenClosedSessionShortcut` 禁用 |
| Insert @-Mention Reference | `Option+K` (Mac) / `Alt+K` (Windows/Linux) | 插入对当前文件和所选内容的引用（需要编辑器处于焦点状态） |
| Accept Change at Cursor | - | 在逐个[审阅建议的编辑](#get-started)时，接受光标处的更改。需要 Claude Code v2.1.275 或更高版本 |
| Reject Change at Cursor | - | 在逐个审阅建议的编辑时，还原光标处的更改。需要 Claude Code v2.1.275 或更高版本 |
| Toggle Focus view | `Ctrl+Option+F` (Mac) / `Ctrl+Alt+F` (Windows/Linux) | 隐藏或显示对话中的工具活动。在 Claude 面板或侧边栏可见时生效。在 Windows/Linux 上，当焦点位于调试控制台或可搜索列表（例如资源管理器）中时，`Ctrl+Alt+F` 会改为打开 VS Code 的查找功能。请将焦点移至编辑器或 Claude 面板，然后再次按下该快捷键。需要 Claude Code v2.1.221 或更高版本 |
| Rename Session Tab | - | 重命名当前活动 Claude 标签页中的会话。需要 Claude Code v2.1.257 或更高版本 |
| Add Session Tab to Group | - | 将当前活动 Claude 标签页中的会话添加到您选择或创建的[会话组](#organize-sessions-into-groups)中。需要 Claude Code v2.1.257 或更高版本 |
| Mark Session as Unread | - | 在会话列表中将当前活动 Claude 标签页中的会话标记为未读。需要 Claude Code v2.1.257 或更高版本 |
| Show Logs | - | 查看扩展调试日志 |
| Logout | - | 注销您的 Anthropic 账户 |

### 从其他工具启动 VS Code 标签页

该扩展在 `vscode://anthropic.claude-code/open` 注册了一个 URI 处理程序。您可以使用它从自己的工具中打开新的 Claude Code 标签页：shell 别名、浏览器书签小工具，或任何能够打开 URL 的脚本。如果 VS Code 尚未运行，打开该 URL 会先启动 VS Code。如果 VS Code 已在运行，该 URL 会在当前处于焦点状态的窗口中打开。

使用操作系统的 URL 打开程序来调用该处理程序。

> [tab] macOS
>
> ```bash theme={null}
> open "vscode://anthropic.claude-code/open"
> ```

> [tab] Linux
>
> ```bash theme={null}
> xdg-open "vscode://anthropic.claude-code/open"
> ```
>
> `xdg-open` 命令来自 `xdg-utils` 软件包。如果 shell 报告找不到该命令，请参阅 [Linux 上找不到 xdg-open](https://code.claude.com/docs/zh-CN/deep-links.md#xdg-open-is-not-found-on-linux)。

> [tab] Windows
>
> 在 PowerShell 中：
>
> ```powershell theme={null}
> Start-Process "vscode://anthropic.claude-code/open"
> ```
>
> 在 `cmd.exe` 中，`start` 会将其第一个带引号的参数视为窗口标题，因此请在 URL 之前传入一个空标题：
>
> ```cmd theme={null}
> start "" "vscode://anthropic.claude-code/open"
> ```

该处理程序接受两个可选的查询参数：

| 参数 | 描述 |
| - | - |
| `prompt` | 预先填入提示词输入框的文本。必须经过 URL 编码。提示词会被预先填入，但不会自动提交。 |
| `session` | 要恢复的会话 ID，该会话须来自 VS Code 中当前打开的文件夹。在 Claude Code v2.1.300 或更高版本中，来自该文件夹所在仓库的主检出目录或某个 [worktree](https://code.claude.com/docs/zh-CN/worktrees.md) 的会话也可以恢复。如果找不到该会话，则会开始一个新对话。如果该会话已在某个标签页中打开，则会聚焦到该标签页。要以编程方式获取会话 ID，请参阅[继续对话](https://code.claude.com/docs/zh-CN/headless.md#continue-conversations)。 |

例如，要打开一个预先填入"review my changes"的标签页：

```text theme={null}
vscode://anthropic.claude-code/open?prompt=review%20my%20changes
```

该扩展还处理 `vscode://anthropic.claude-code/install-plugin`，它会[针对某个插件打开插件对话框](#share-a-plugin-install-link)。要启动终端会话而不是 VS Code 标签页，请使用 CLI 的 `claude-cli://` 处理程序。请参阅[从链接启动会话](https://code.claude.com/docs/zh-CN/deep-links.md)。

## 配置设置

您可以在两个位置配置该扩展：VS Code 的扩展设置和 Claude Code 设置文件：

* VS Code 中的**扩展设置**：控制扩展在 VS Code 中的行为。使用 `Cmd+,`（Mac）或 `Ctrl+,`（Windows/Linux）打开，然后转到 **Extensions > Claude Code**。也可以输入 `/` 并选择 **General config…** 来打开设置。
* `~/.claude/settings.json` 中的 **Claude Code 设置**：由扩展和 CLI 共享。用于配置允许的命令、环境变量、hook 和 MCP 服务器。在 Claude Code v2.1.283 或更高版本中，它也是决定对话起始权限模式的输入之一；在更早的版本中，仅适用于 Pro、Max、Ultra 和 Team 计划。[切换权限模式](https://code.claude.com/docs/zh-CN/permission-modes.md#switch-permission-modes)列出了优先顺序。详情请参阅[设置](https://code.claude.com/docs/zh-CN/settings.md)。

> [!TIP]
>
> 将 `"$schema": "https://json.schemastore.org/claude-code-settings.json"` 添加到您的 `settings.json` 中，即可直接在 VS Code 中获得所有可用设置的自动补全和内联验证。

### 扩展设置

VS Code 从您的用户设置中读取 `initialPermissionMode`，并忽略工作区中的值。在 v2.1.225 之前，VS Code 将该设置默认为 `default`，并会应用工作区中的值。

| 设置 | 默认值 | 描述 |
| - | - | - |
| `useTerminal` | `false` | 以终端模式而非图形面板启动 Claude |
| `initialPermissionMode` | - | 控制新对话的批准提示：`default`、`plan`、`acceptEdits` 或 `bypassPermissions`。`manual` 是 `default` 的别名，会选择模式指示器中标记为 **Manual** 的模式。未设置时，扩展会按照[切换权限模式](https://code.claude.com/docs/zh-CN/permission-modes.md#switch-permission-modes)中所述选择起始权限模式。 |
| `preferredLocation` | `panel` | Claude 的打开位置：`sidebar`（右侧）或 `panel`（新标签页） |
| `lockEditorGroups` | `true` | [锁定 Claude 为其标签页启动的编辑器组](#choose-where-claude-lives)，这样在 Claude 标签页获得焦点时打开的文件会进入另一个编辑器组。关闭时，扩展永远不会锁定编辑器组。需要 Claude Code v2.1.274 或更高版本 |
| `autosave` | `true` | 在 Claude 读取或写入文件之前自动保存文件 |
| `attachOpenFile` | `true` | 将编辑器中打开的文件添加到您的消息中，并在输入框中显示。关闭时，仅在您选中文本时才随之添加该文件。需要 Claude Code v2.1.271 或更高版本 |
| `attachSelection` | `true` | 将您在编辑器中选中的文本添加到您的消息中，并在输入框中显示。关闭时，您仍然可以[将选区作为 @-mention 添加](#automatic-context-from-your-selection-and-open-file)。需要 Claude Code v2.1.300 或更高版本 |
| `useCtrlEnterToSend` | `false` | 使用 Ctrl/Cmd+Enter 而非 Enter 发送提示词 |
| `scrollToBottomOnSend` | `true` | 发送消息时将对话滚动到底部。关闭时，对话保持在您离开时的位置。需要 Claude Code v2.1.275 或更高版本 |
| `showMessageTimestamps` | `true` | 显示每条消息的发送时间。日期行标记日期变化的位置。需要 Claude Code v2.1.284 或更高版本。在 v2.1.290 之前，默认值为 `false` |
| `spinnerVerbs` | `{"mode": "append", "verbs": []}` | 设置在轮次运行期间对话加载动画轮换显示的动词，使用与 CLI 的 [`spinnerVerbs`](https://code.claude.com/docs/zh-CN/settings-reference#spinnerverbs) 相同的 `mode` 和 `verbs` 字段。 |
| `enableNewConversationShortcut` | `false` | 启用 Cmd/Ctrl+N 开始新对话 |
| `enableReopenClosedSessionShortcut` | `true` | 使用 Cmd/Ctrl+Shift+T 重新打开最近关闭的 Claude 会话标签页。如果最后关闭的标签页不是 Claude 会话，该快捷键会改为运行 VS Code 常规的重新打开已关闭编辑器命令。 |
| `archiveInactiveSessions` | `14` | 会话在无活动达到此天数后[自动归档](#resume-past-conversations)：`1`、`2`、`7` 或 `14`。设置为 `0` 可关闭此功能。需要 Claude Code v2.1.265 或更高版本 |
| `continueAfterReload` | `true` | 窗口重新加载后，Claude 会在恢复的会话中[继续被中断的步骤](#continue-conversations-after-a-reload)。需要 Claude Code v2.1.274 或更高版本 |
| `hideOnboarding` | `false` | 隐藏入门清单（学士帽图标） |
| `focusView` | `false` | 将工具调用、工具结果和思考内容隐藏在可展开的行中，仅保留您的提示词和 Claude 的回复。Claude 最新的待办事项列表保持可见；这需要 Claude Code v2.1.225 或更高版本。您也可以从命令菜单中切换 Focus view。需要 Claude Code v2.1.221 或更高版本 |
| `respectGitIgnore` | `true` | 在文件搜索和[选区上下文](#selected-text-from-excluded-or-ignored-files)中排除 .gitignore 模式 |
| `usePythonEnvironment` | `true` | 运行 Claude 时激活工作区的 Python 环境。需要 Python 扩展。 |
| `environmentVariables` | `[]` | 为 Claude 进程设置环境变量。如需共享配置，请改用 Claude Code 设置。[`CLAUDE_CONFIG_DIR`](https://code.claude.com/docs/zh-CN/env-vars) 条目仅在其值为绝对路径时生效；扩展不会展开 `~`，并会忽略相对路径值。 |
| `disableLoginPrompt` | `false` | 跳过身份验证提示（用于第三方提供商配置） |
| `allowDangerouslySkipPermissions` | `false` | 将 Bypass permissions 添加到模式选择器中。仅在无法访问互联网的沙箱中使用。 |
| `claudeProcessWrapper` | - | 用于启动 Claude 进程的可执行文件。如果存在捆绑的二进制文件，其路径会作为参数传入。如果扩展的构建版本未包含适用于您平台的二进制文件，请将此项设置为单独安装的 `claude` 二进制文件。 |

## 使用第三方提供商

默认情况下，Claude Code 直接连接到 Anthropic 的 API。如果您的组织使用 Amazon Bedrock、Google Cloud 的 Agent Platform 或 Microsoft Foundry 来访问 Claude，请将扩展配置为改用您的提供商：

**1. 禁用登录提示**

打开 [Disable Login Prompt 设置](vscode://settings/claudeCode.disableLoginPrompt)并勾选该复选框。

您也可以在 Mac 上按 `Cmd+,` 或在 Windows/Linux 上按 `Ctrl+,` 打开 VS Code 设置，搜索"Claude Code login"，然后勾选 **Disable Login Prompt**。

**2. 配置您的提供商**

按照适用于您的提供商的设置指南进行操作：

* [在 Amazon Bedrock 上使用 Claude Code](https://code.claude.com/docs/zh-CN/amazon-bedrock.md)
* [在 Google Cloud 的 Agent Platform 上使用 Claude Code](https://code.claude.com/docs/zh-CN/google-vertex-ai.md)
* [在 Microsoft Foundry 上使用 Claude Code](https://code.claude.com/docs/zh-CN/microsoft-foundry.md)

这些指南介绍了如何在 `~/.claude/settings.json` 中配置您的提供商，这样可以确保您的设置在 VS Code 扩展和 CLI 之间共享。

使用第三方提供商时，扩展不提供需要 claude.ai 账户的功能，例如计划用量条、[语音听写](https://code.claude.com/docs/zh-CN/voice-dictation.md)以及用于[云端会话](#resume-cloud-sessions-from-claude-ai)的 Web 标签页。有关使用这些登录方式时 Account & usage 对话框显示的内容，请参阅[查看账户和用量](#check-account-and-usage)。

之前通过 `/login` 遗留的 claude.ai 登录将保持未使用状态：扩展不会在任何请求中发送它。

## 使用屏幕阅读器

扩展的聊天面板支持屏幕阅读器。无需启用任何设置：扩展会为每位用户播报对话活动，且不会产生任何视觉变化。这与 CLI 中需手动启用的[屏幕阅读器模式](https://code.claude.com/docs/zh-CN/accessibility.md)不同，后者会调整终端界面。

聊天面板中的屏幕阅读器支持需要 Claude Code v2.1.236 或更高版本。

在对话过程中，扩展会播报以下内容：

* **Claude 的回复**：扩展会在每条回复完成时播报一次，在文本流式输出期间保持静默。屏幕阅读器会将代码块读作行数摘要，按标签读出链接，并逐个单元格读出表格；完整回复仍可在会话记录中阅读。
* **权限请求和问题**：当权限提示出现时，扩展会播报该请求，并说明 Claude 想要使用的工具名称。当 Claude 向您提问，以及 Claude 完成计划并等待您审阅时，扩展也会以同样的方式播报。
* **状态变化**：当 Claude 开始工作、Claude 准备好接收您的输入，以及 Claude Code 开始压缩对话时，扩展会进行播报。如果您在会话的 [SessionStart](https://code.claude.com/docs/zh-CN/hooks.md#sessionstart) hook 仍在运行时发送提示词，屏幕阅读器还会读出"Running start-up hooks."。
* **排队的消息**：当您在 Claude 工作期间发送消息时，扩展会针对该消息播报"Message queued."。
* **错误和模型提示**：扩展会播报对话中的错误，并在[使用额度同意提示](https://code.claude.com/docs/zh-CN/model-config.md#fable-and-usage-credits)或[标记请求提示](https://code.claude.com/docs/zh-CN/model-config.md#ask-before-switching)出现时进行播报。

在 Claude 工作期间，屏幕阅读器会读出一个文本标签，以代替进度指示器的动画。

当您重新打开会话或切换到其他会话时，扩展不会播报任何内容：恢复的历史记录、待处理的权限提示和进行中的状态都会保持静默，直到有新情况发生。

### 通过键盘使用聊天面板

会话记录中的每个轮次都以一个视觉上隐藏的标题开始，该标题以开启该轮次的提示词命名。使用屏幕阅读器的标题导航功能即可在轮次之间跳转。

在一个轮次中移动时，屏幕阅读器会播报当前所在消息的发送方：

* **您的消息**："You"
* **Claude 的消息**："Claude"
* **工具步骤**："Claude" 加上工具名称，例如 "Claude, Bash"
* **思考块**："Claude, thinking"

由于扩展将会话记录公开为一个带标签的区域，您也可以使用 `Tab` 将焦点移至会话记录本身，并按自己的节奏阅读。如果要将焦点移至最新消息或等待中的权限提示，请从[命令面板](#vs-code-commands-and-shortcuts)运行 **Claude Code: Focus last message**。

当某个权限提示选项会保存权限规则或目录访问权限时，其标签末尾会注明批准的保存位置，例如"all projects"或"this session"。要更改保存位置：

* 在该选项获得焦点时，按 `Left` 或 `Right` 方向键。移动到每个保存位置时，扩展都会进行播报。方向键功能需要 Claude Code v2.1.268 或更高版本。
* 点击标签中的保存位置。

## VS Code 扩展与 Claude Code CLI 对比

Claude Code 同时提供 VS Code 扩展（图形面板）和 CLI（终端中的命令行界面）两种形式。部分功能仅在 CLI 中提供。如果需要使用仅限 CLI 的功能，请在 VS Code 的集成终端中运行 `claude`。这需要进行[独立 CLI 安装](https://code.claude.com/docs/zh-CN/setup.md)：扩展不会将 `claude` 添加到您的 PATH 中。请参阅[在 VS Code 中运行 CLI](#run-cli-in-vs-code)。

| 功能 | CLI | VS Code 扩展 |
| - | - | - |
| 命令和 skill | [全部](https://code.claude.com/docs/zh-CN/commands.md) | 部分（输入 `/` 查看可用项） |
| MCP 服务器配置 | 是 | 是：在聊天面板中使用 `/mcp` [添加和管理服务器](#connect-to-external-tools-with-mcp) |
| 检查点 | 是 | 是 |
| `!` Bash 快捷方式 | 是 | 否 |
| Tab 补全 | 是 | 否 |

### 在 VS Code 中运行 CLI

如需在不离开 VS Code 的情况下使用 CLI，请在 Windows/Linux 上按 `` Ctrl+` ``，或在 Mac 上按 `` Cmd+` `` 打开集成终端，然后运行 `claude`。CLI 会自动与您的 IDE 集成，以支持 diff 查看和诊断信息共享等功能。如果您使用外部终端，请在 Claude Code 中运行 `/ide` 将其连接到 VS Code。

安装扩展并不会将 `claude` 添加到您的 shell PATH 中。扩展为其聊天面板捆绑了一份私有的 CLI 副本，但要在终端中输入 `claude`，需要进行[独立 CLI 安装](https://code.claude.com/docs/zh-CN/setup.md)。只需安装一次，本页中的命令（包括 `claude mcp add` 和 `claude --resume`）即可在任何终端中使用。如果安装后仍然找不到 `claude`，请[验证您的 PATH](https://code.claude.com/docs/zh-CN/troubleshoot-install.md#verify-your-path)。

### 在扩展和 CLI 之间切换

扩展和 CLI 共享相同的对话历史记录。如需在 CLI 中继续扩展中的对话，请在终端中运行 `claude --resume`。这会打开一个交互式选择器，您可以在其中搜索并选择您的对话。

## 使用 MCP 连接外部工具

MCP (Model Context Protocol) 服务器让 Claude 能够访问外部工具、数据库和 API。

要在不离开 VS Code 的情况下管理 MCP 服务器，请在聊天面板中输入 `/mcp`。在打开的对话框中，您可以：

* 添加服务器
* 移除保存在 local、user 或 project [作用域](https://code.claude.com/docs/zh-CN/mcp.md#mcp-installation-scopes)中的服务器
* 启用或禁用服务器
* 重新连接到服务器
* 管理 OAuth 身份验证

在对话框中添加和移除服务器需要 Claude Code v2.1.261 或更高版本。

您也可以在 VS Code 的集成终端（`` Ctrl+` `` 或 `` Cmd+` ``）中运行 `claude mcp add`。对话框和终端命令会保存到同一个 MCP 配置中，通过任一方式所做的更改都会在之后开始的对话中生效。下面的示例添加了 GitHub 的远程 MCP 服务器，该服务器使用以请求头形式传递的[个人访问令牌](https://github.com/settings/personal-access-tokens)进行身份验证：

```bash theme={null}
claude mcp add --transport http github https://api.githubcopilot.com/mcp/ \
  --header "Authorization: Bearer YOUR_GITHUB_PAT"
```

将 `YOUR_GITHUB_PAT` 替换为您的个人访问令牌。`claude mcp add` 命令保存配置时不会验证凭据，因此这里会接受占位值，但服务器之后将无法连接。要验证连接，请开始一个新对话，输入 `/mcp`，并确认该服务器显示 **Connected**。凭据有误的服务器会显示 **Failed**。

配置完成后，即可让 Claude 使用这些工具（例如，"Review PR #456"）。

> [!NOTE]
>
> 要查找可连接的服务器，请参阅[查找和构建 MCP 服务器](https://code.claude.com/docs/zh-CN/mcp.md#find-and-build-mcp-servers)。

## 内置 IDE MCP 服务器

扩展处于活动状态时，会运行一个本地 MCP 服务器，CLI 会自动连接到该服务器。通过此服务器，CLI 可以：

* 在 VS Code 的原生 diff 查看器中打开 diff
* 读取您当前的选中内容以用于 `@` 提及
* 在您使用 Jupyter notebook 时请求 VS Code 执行单元格

该服务器名为 `ide`，并且在 `/mcp` 中处于隐藏状态，因为它没有任何需要配置的内容。不过，如果您的组织使用 `PreToolUse` hook 来设置 MCP 工具的允许列表，则需要了解它的存在。

**选中内容和已打开文件上下文。** 连接期间，CLI 会在您发送的每条提示词中附带当前编辑器的选中内容以及活动文件的路径作为上下文。发生这种情况时，会话记录中会显示一行 `⧉ Selected N lines from <file>`。以下情况会改变该上下文：

* **关闭 [Attach Selection 设置](#keep-your-selection-and-open-file-out-of-your-messages)**：CLI 不会将您的选中内容作为上下文附带。
* **关闭 Attach Open File 设置**：只有在同时附带选中内容时，CLI 才会附带活动文件的路径。
* **排队的消息**：如果您[在 Claude 工作时将消息加入队列](https://code.claude.com/docs/zh-CN/interactive-mode.md#queue-messages-while-claude-works)，该消息会保留您按下 `Enter` 时的选中内容，无论您之后选中了什么。
* **敏感文件**：要排除 `.env` 等敏感文件，请为其路径添加 [`Read` 拒绝规则](https://code.claude.com/docs/zh-CN/permissions#read-and-edit)。匹配的拒绝规则会阻止该文件的选中文本和已打开文件通知发送给 Claude。

**传输和身份验证。** 该服务器绑定到 `127.0.0.1` 上 10000–65535 范围内的随机端口，端口不可配置。传输方式和身份验证令牌的工作方式如下：

* **传输方式**：未加密的 `ws://`。由于该套接字仅限回环地址，任何能够捕获流量的进程也都能从锁文件中读取令牌，因此 TLS 不会提供额外保护。
* **身份验证令牌**：每次激活扩展时都会生成一个新的随机身份验证令牌，并将其写入位于 `~/.claude/ide/<port>.lock` 的锁文件。CLI 必须以 `X-Claude-Code-Ide-Authorization` 标头的形式提供该令牌才能连接。
* **锁文件权限**：锁文件的权限为 `0600`，位于权限为 `0700` 的目录中，因此只有运行 VS Code 的用户才能读取它。如果设置了 `CLAUDE_CONFIG_DIR`，锁文件将改为写入 `$CLAUDE_CONFIG_DIR/ide/`。

**向模型公开的工具。** 该服务器托管了十几个工具，但只有两个对模型可见。其余工具是 CLI 用于自身 UI 的内部 RPC，例如打开 diff、读取选中内容和保存文件。这些工具会在工具列表到达 Claude 之前被过滤掉。

| 工具名称（hook 中看到的名称） | 功能 | 只读 |
| - | - | - |
| `mcp__ide__getDiagnostics` | 返回语言服务器诊断信息：即 VS Code 问题面板中的错误和警告。可选择限定为单个文件。 | 是 |
| `mcp__ide__executeCode` | 在活动 Jupyter notebook 的内核中运行 Python 代码。请参阅下方的确认流程。 | 否 |

**聊天面板中的诊断信息。** 在聊天面板中，使用 Claude Code v2.1.285 或更高版本时，Claude 会通过一个名为 `claude-vscode` 的独立内置服务器读取 VS Code 的问题面板。Claude 可以向它请求单个文件的当前错误和警告，也可以请求 VS Code 拥有诊断信息的所有文件的错误和警告。

hook 和权限规则会将聊天面板的诊断工具视为 `mcp__claude-vscode__getDiagnostics`。要同时涵盖 CLI 和聊天面板中的诊断功能，请在您的 hook 或规则中同时指定 `mcp__ide__getDiagnostics` 和 `mcp__claude-vscode__getDiagnostics`。

以下 `settings.json` 示例会拒绝这两个工具：

```json theme={null}
{
  "permissions": {
    "deny": [
      "mcp__ide__getDiagnostics",
      "mcp__claude-vscode__getDiagnostics"
    ]
  }
}
```

`Read` 拒绝规则不涵盖这两个工具中的任何一个，因此请像示例那样使用[拒绝规则](https://code.claude.com/docs/zh-CN/permissions.md#mcp)按名称阻止它们。

**Jupyter 执行始终会先询问。** `mcp__ide__executeCode` 无法静默运行任何内容。每次调用都会经过以下步骤：

1. 代码作为新单元格插入到活动 notebook 的末尾。
2. VS Code 将该单元格滚动到可见区域。
3. 原生 Quick Pick 会请您选择 **Execute** 或 **Cancel**。

取消或按 `Esc` 关闭选择器会向 Claude 返回错误，且不会运行任何内容。当没有活动 notebook、未安装 Jupyter 扩展（`ms-toolsai.jupyter`）或内核不是 Python 时，该工具也会直接拒绝执行。

> [!NOTE]
>
> Quick Pick 确认独立于 `PreToolUse` hook。`mcp__ide__executeCode` 的允许列表条目让 Claude 可以*提议*运行单元格；而 VS Code 中的 Quick Pick 才是让它*真正*运行的关键。

## 修复常见问题

登录、网络和启动错误在安装故障排除页面和错误参考页面中有各自的条目。请在下表中找到您看到的内容，然后点击对应链接。

| 您看到的内容 | 前往位置 |
| - | - |
| 登录后出现 `API Error: 403 Request not allowed` | [登录后出现 403 Forbidden](https://code.claude.com/docs/zh-CN/troubleshoot-install.md#403-forbidden-after-login) |
| 已经登录后又被要求重新登录 | [未登录或令牌已过期](https://code.claude.com/docs/zh-CN/troubleshoot-install.md#not-logged-in-or-token-expired) |
| 云提供商凭据在终端中有效，但在扩展中无效 | [Bedrock、Agent Platform 或 Foundry 凭据未加载](https://code.claude.com/docs/zh-CN/troubleshoot-install.md#bedrock-agent-platform-or-foundry-credentials-not-loading) |
| `SSL certificate verification failed` 或 `Self-signed certificate detected` | [SSL 证书错误](https://code.claude.com/docs/zh-CN/errors.md#ssl-certificate-errors) |
| `Dictation error: WebSocket error:` 后跟证书错误（例如 `unable to get local issuer certificate`），或 Web 标签页上出现 `Remote server is not connected` | [在执行 TLS 检查的代理后方，听写或 Web 标签页失败](#dictation-or-the-web-tab-fails-behind-a-tls-inspecting-proxy) |
| `Claude Code process exited with code 1` 或其他代码 | [Claude Code process exited with code N](https://code.claude.com/docs/zh-CN/errors.md#claude-code-process-exited-with-code-n) |
| `Could not locate the Claude CLI on PATH` | [Could not locate the Claude CLI on PATH](https://code.claude.com/docs/zh-CN/errors.md#could-not-locate-the-claude-cli-on-path) |
| `The connection to Claude Code ended before this message completed` | [The connection to Claude Code ended before this message completed](https://code.claude.com/docs/zh-CN/errors.md#the-connection-to-claude-code-ended-before-this-message-completed) |
| `The Claude Code program for this computer, the <build> build, could not be found` | [The Claude Code program for this computer could not be found](https://code.claude.com/docs/zh-CN/errors.md#the-claude-code-program-for-this-computer-could-not-be-found) |
| 在 VS Code 的集成终端中找不到 `claude` | [在 VS Code 中运行 CLI](#run-cli-in-vs-code) |

### 扩展无法安装

* 确保您的 VS Code 版本兼容（1.94.0 或更高版本）
* 检查 VS Code 是否具有安装扩展的权限
* 尝试直接从 [Visual Studio Marketplace](https://marketplace.visualstudio.com/items?itemName=anthropic.claude-code) 安装

### Spark 图标不可见

打开文件时，Spark 图标会显示在**编辑器工具栏**（编辑器右上角）中。如果您没有看到该图标：

1. **打开一个文件**：该图标需要有文件处于打开状态。仅打开文件夹是不够的。
2. **检查 VS Code 版本**：需要 1.94.0 或更高版本。可在 **Help > About** 下查看
3. **重启 VS Code**：从命令面板运行"Developer: Reload Window"
4. **禁用冲突的扩展**：暂时禁用其他 AI 扩展（Cline、Continue 等）
5. **检查工作区信任**：该扩展在受限模式下无法使用

或者，点击窗口右下角**状态栏**中的 **✻ Claude Code**。即使没有打开文件，此方法也有效。您也可以使用**命令面板**（`Cmd+Shift+P` / `Ctrl+Shift+P`）并输入"Claude Code"。

### 在 macOS 上按 Cmd+Esc 没有反应

在 macOS Tahoe 及更高版本中，系统的游戏叠加层快捷键默认绑定到 `Cmd+Esc`，会在按键到达 VS Code 之前将其拦截。要释放该快捷键：

1. 打开系统设置
2. 依次进入键盘、键盘快捷键、游戏控制器
3. 取消勾选游戏叠加层复选框

或者，将扩展重新绑定到其他按键：打开 VS Code [快捷键编辑器](https://code.visualstudio.com/docs/configure/keybindings)（`Cmd+K Cmd+S`），搜索 `Claude Code: Focus input`，然后分配新的快捷键。

### 按 Ctrl+J 会在输入框中添加新行

要阻止 `Ctrl+J` 在输入框中添加新行，请在 Claude Code 的 `keybindings.json` 文件中[解除该按键的绑定](https://code.claude.com/docs/zh-CN/keybindings.md#unbind-default-shortcuts)。扩展在 2.1.309 或更高版本中才会读取此文件，因此如果扩展版本较旧，请先在扩展视图中更新。

以下示例是完整的文件，其中只有一个用于解除该按键绑定的条目，即 `Chat` 块中的 `"ctrl+j": null`：

```json theme={null}
{
  "bindings": [
    {
      "context": "Chat",
      "bindings": {
        "ctrl+j": null
      }
    }
  ]
}
```

该文件位于您的 Claude Code 配置目录中，除非您已通过 `CLAUDE_CONFIG_DIR` 移动该目录，否则该目录为 `~/.claude`。请查看该目录，并根据您发现的情况执行相应步骤：

* **尚无 `keybindings.json`**：创建该文件，内容与示例完全相同
* **`keybindings.json` 中没有 `Chat` 块**：将示例中的 `Chat` 块添加到顶层 `bindings` 数组中，位置不限
* **`Chat` 块中没有 `"ctrl+j"` 行**：将 `"ctrl+j": null` 添加到该块的 `bindings` 中
* **`Chat` 块中已有 `"ctrl+j"` 行**：将该行的值更改为 `null`

保存文件后，在输入框中按 `Ctrl+J` 将不再添加新行。

如果 `Ctrl+J` 仍然在输入框中添加新行，请从命令面板按顺序运行以下两个命令：

1. **Developer: Reload Window**，然后在输入框中再次按 `Ctrl+J`
2. **Claude Code: Show Logs**，然后查找包含 `Ctrl+J in the chat input still inserts a newline` 的警告。在警告所指出的文件中更正其报告的问题，保存文件，然后再次按 `Ctrl+J`。

如果日志中没有此类警告，请检查该文件是否位于您的 Claude Code 配置目录中，以及其 `Chat` 块是否将 `"ctrl+j"` 设置为 `null`。如果 `Ctrl+J` 仍然添加新行，请[报告该问题](https://github.com/anthropics/claude-code/issues)。

### Claude Code 面板空白或无法加载

如果 Claude Code 面板已打开但完全不显示任何内容（连输入框都没有），请在扩展视图中将扩展更新到 v2.1.309 或更高版本。然后从命令面板运行 **Developer: Reload Window**。如果随后看到 `Claude Code couldn’t load` 通知，请按顺序完成其中列出的步骤。

如果面板仍然无法加载，请打开扩展的日志，并附上日志副本[报告该问题](https://github.com/anthropics/claude-code/issues)：

* **在 v2.1.309 或更高版本上仍然空白**：从命令面板运行 **Claude Code: Show Logs** 打开日志
* **通知中的步骤没有帮助**：点击通知中的 **Show logs** 打开日志

### 恢复的对话起初显示为空白

当您[恢复过去的对话](#resume-past-conversations)或某个对话[在重新加载后恢复](#continue-conversations-after-a-reload)时，在该对话的 Claude Code 进程启动期间，其消息可能保持空白。大约一秒后，空白区域会显示 `Waiting for your plugins…`。

您无需进行任何操作。一旦进程启动并确认没有插件需要先检查这些消息，消息就会显示；如果启动时间较长，则会在 20 秒后显示。

### 每次重新加载重新打开对话时面板都停止响应

如果每次重新加载并恢复对话时，整个面板都会崩溃、冻结或仅显示 `Something went wrong`，请将扩展更新到 v2.1.301 或更高版本。然后再次重新加载窗口或重启 VS Code。当对话连续两次未能完成打开后，您会得到一个新的聊天来替代它，并附有一条说明[该对话未被重新打开](#a-conversation-was-not-reopened)的通知。

### 公式显示为原始 LaTeX

如果 Claude 回复中的所有数学公式都没有被排版，请在扩展视图中检查扩展的版本，因为[数学渲染](#latex-math-in-responses)需要 2.1.301 或更高版本。如果您使用的是[终端模式](#switch-to-terminal-mode)（无论版本如何，该模式都不会排版数学公式），请取消勾选 [Use Terminal 设置](vscode://settings/claudeCode.useTerminal)，以便 Claude Code 改为在聊天面板中打开。

如果某个公式按原样显示，而其附近的其他公式已被排版，请检查以下原因：

* **该公式是行内公式，且面板未将其视为数学公式**：请让 Claude 以单独成行的 `$$` 块重新发送该公式。当 `$…$` 分隔符之间的文本符合以下情况时，面板会将行内公式按原样保留：
  * 不只是简单算术，却没有 `=`、`+` 或 `^` 等字符将其标记为数学公式，例如 `$O(n log n)$`
  * 以数字开头且没有此类字符，例如 `$30$` 和 `$1/N$`
  * 美元符号内含空格，例如 `$ x^2 $`
  * 超过 60 个字符
  * 跨两行。跨两行的 `\(…\)` 公式同样不会被排版。
* **该公式存在 TeX 语法错误**：请让 Claude 重写该公式并修正错误。
* **该公式定义了宏**：包含宏定义（例如 `\newcommand` 或 `\def`）的公式会按原样显示。请让 Claude 重写该公式，不使用宏。
* **该公式包含未转义的 `%`**：请让 Claude 重写该公式，使用 `\%` 表示百分号。
* **该公式过大**：长度超过 8,192 个字符的公式，或排版输出会远大于其 TeX 源文本的公式，会按原样显示。请让 Claude 将其拆分为多个较小的公式，或以更紧凑的方式书写。
* **该公式位于大量数学公式之后**：在回复的同一文本块中，面板会按顺序排版公式，直至达到上限。超出上限的后续公式会按原样显示。请让 Claude 单独再次发送您需要的公式。

### 在执行 TLS 检查的代理后方，听写或 Web 标签页失败

当听写或 Web 标签页因证书错误（例如 `unable to get local issuer certificate`）而失败时，请在 `~/.claude/settings.json` 的 `env` 块中设置 [`NODE_EXTRA_CA_CERTS`](https://code.claude.com/docs/zh-CN/network-config#custom-ca-certificates)。将其设置为包含代理根 CA 证书的 PEM 文件的路径，该文件可向您的 IT 团队索取。该失败会显示在麦克风按钮或 Web 标签页上，或显示在日志中：

* **听写**：麦克风按钮会在 `Dictation error: WebSocket error:` 之后显示错误文本。
* **Web 标签页**：列出[云端会话](#resume-cloud-sessions-from-claude-ai)的标签页会显示 `Remote server is not connected`，但不显示错误文本。要在日志中查看错误文本：
  1. 从[命令面板](#vs-code-commands-and-shortcuts)运行 **Claude Code: Show Logs**，打开 `Claude VSCode` 输出频道。
  2. 点击该标签页上的 **Reconnect**，等待其不再显示 `Connecting…`。
  3. 在该频道中搜索 `Error fetching remote sessions:` 或 `Failed to fetch organization UUID:`，然后在最新的匹配行中查看该前缀之后的错误文本。
* **日志警告**：提及 `an authority that Claude Code does not trust` 的警告表示出现了证书错误。在执行 TLS 检查的代理后方，请在 `~/.claude/settings.json` 的 `env` 中设置 `NODE_EXTRA_CA_CERTS`。v2.1.309 之前的版本不会记录此警告。

此修复需要扩展版本 2.1.307 或更高版本，因此请先更新扩展。然后将 `NODE_EXTRA_CA_CERTS` 设置为证书文件的完整路径，且不要使用 `~`。对于 Windows 路径，在 `settings.json` 中每个反斜杠需要写两次，例如 `C:\\certs\\example-ca.pem`。

> [!NOTE]
>
> 在连接到 WSL、SSH 主机、开发容器或其他系统的窗口中，请在该系统上（而不是在您的本地计算机上）保存证书文件并编辑 `~/.claude/settings.json`。

以下示例指定了位于 Linux 或 macOS 路径 `/etc/ssl/certs/example-ca.pem` 的证书文件：

```json ~/.claude/settings.json theme={null}
{
  "env": {
    "NODE_EXTRA_CA_CERTS": "/etc/ssl/certs/example-ca.pem"
  }
}
```

保存 `settings.json`，然后开始录音或再次点击 **Reconnect**。如果错误再次出现，请通过 **Claude Code: Show Logs** 打开输出频道，并在其中搜索 `use the CA certificates file`。此类警告会说明扩展无法使用您的文件的原因，并指出它从何处读取该路径。然后在下表中找到您设置 `NODE_EXTRA_CA_CERTS` 的每个位置，并按照对应行的说明操作：

| 设置位置 | 操作 |
| - | - |
| 启动 VS Code 时所用的环境 | 此处设置的值优先于 `~/.claude/settings.json` 中的值，即使其指定的文件无法使用也是如此。请更正或移除该值，然后重启 VS Code |
| `~/.claude/settings.json` 中的 `env` | 更正该值或其指定的文件，然后开始录音或再次点击 **Reconnect** |
| VS Code 的 [`claudeCode.environmentVariables`](#extension-settings) 设置、`~/.claude.json`、项目的 `.claude/settings.json` 或 `.claude/settings.local.json`，或托管设置 | 此处设置的值不适用于听写或 Web 标签页。请同时在 `~/.claude/settings.json` 的 `env` 中设置该变量 |

### Claude Code 始终没有响应

如果 Claude Code 没有响应您的提示词：

1. **检查您的网络连接**：确保您的网络连接稳定
2. **开始新对话**：尝试开始一个新对话，查看问题是否仍然存在
3. **尝试使用 CLI**：在终端中运行 `claude`，查看是否能获得更详细的错误消息

如果问题仍然存在，请[在 GitHub 上提交 issue](https://github.com/anthropics/claude-code/issues)，并附上错误的详细信息。

## 卸载扩展

要卸载 Claude Code 扩展：

1. 在 Mac 上按 `Cmd+Shift+X`，或在 Windows/Linux 上按 `Ctrl+Shift+X`，打开扩展视图
2. 搜索"Claude Code"
3. 点击 **Uninstall**

如果您在 VS Code 集成终端中运行 `claude`，Claude Code 会自动重新安装该扩展。要使其保持卸载状态，请在 `/config` 中关闭 **Auto-install IDE extension**，或将 [`autoInstallIdeExtension`](https://code.claude.com/docs/zh-CN/settings-reference#autoinstallideextension) 设置为 `false`。您也可以将 [`CLAUDE_CODE_IDE_SKIP_AUTO_INSTALL`](https://code.claude.com/docs/zh-CN/env-vars) 环境变量设置为 `1`。

如果还要删除扩展数据并重置所有设置，请删除您所用平台上该扩展的存储目录。

在 macOS 上：

```bash theme={null}
rm -rf ~/Library/"Application Support"/Code/User/globalStorage/anthropic.claude-code
```

在 Linux 上：

```bash theme={null}
rm -rf ~/.config/Code/User/globalStorage/anthropic.claude-code
```

在 Windows 上，于 PowerShell 中运行：

```powershell theme={null}
Remove-Item -Recurse -Force "$env:APPDATA\Code\User\globalStorage\anthropic.claude-code"
```

如需更多帮助，请参阅[故障排除指南](https://code.claude.com/docs/zh-CN/troubleshooting.md)。

## 后续步骤

现在您已在 VS Code 中设置好 Claude Code：

* [探索常见工作流](https://code.claude.com/docs/zh-CN/common-workflows.md)，充分发挥 Claude Code 的作用
* [设置 MCP 服务器](https://code.claude.com/docs/zh-CN/mcp.md)，通过外部工具扩展 Claude 的能力。可在聊天面板中使用 `/mcp` 添加和管理它们。
* [配置 Claude Code 设置](https://code.claude.com/docs/zh-CN/settings.md)，自定义允许的命令、hook 等。这些设置在扩展和 CLI 之间共享。
* 查看 Claude Code 的[安全防护措施](https://code.claude.com/docs/zh-CN/security.md)和[数据使用](https://code.claude.com/docs/zh-CN/data-usage.md)
