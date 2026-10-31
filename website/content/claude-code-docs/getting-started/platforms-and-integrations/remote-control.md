> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# 使用 Remote Control 从任何设备继续本地会话

> 使用 Remote Control 从您的手机、平板电脑或任何浏览器继续本地 Claude Code 会话。适用于 claude.ai/code 和 Claude 移动应用。

Remote Control 将 [claude.ai/code](https://claude.ai/code) 或 Claude 应用（[iOS](https://apps.apple.com/us/app/claude-by-anthropic/id6473753684) 和 [Android](https://play.google.com/store/apps/details?id=com.anthropic.claude)）连接到在您的机器上运行的 Claude Code 会话。在您的办公桌上启动一个任务，然后从沙发上的手机或另一台计算机上的浏览器继续。

当您在机器上启动 Remote Control 会话时，Claude 始终在本地运行，因此您的代码执行和文件系统访问保留在您的机器上。使用 Remote Control，您可以：

* **远程使用您的完整本地环境**：您的文件系统、[MCP servers](https://code.claude.com/docs/zh-CN/mcp.md)、工具和项目配置都保持可用，输入 `@` 会自动完成本地项目中的文件路径。
* **同时从两个界面工作**：对话和 [subagents](https://code.claude.com/docs/zh-CN/sub-agents.md) 和 [dynamic workflows](https://code.claude.com/docs/zh-CN/workflows.md) 的进度在所有连接的设备上保持同步，因此您可以从终端、浏览器和手机交替发送消息。
* **从您的手机或浏览器发送图像和文件**：在 Claude 应用或 claude.ai/code 中附加照片或文件，可以带有或不带有标题。Claude 直接将附加的照片视为您消息的一部分。Claude Code 将其他文件下载到您的机器，并将其作为 `@` 文件引用传递给 Claude。
* **在中断后恢复**：如果您的笔记本电脑进入睡眠状态或网络断开，当您的机器重新上线时，Claude Code 会自动重新连接。

与[网络上的 Claude Code](https://code.claude.com/docs/zh-CN/claude-code-on-the-web.md)（在云基础设施上运行）不同，Remote Control 会话直接在您的机器上运行并与您的本地文件系统交互。网络和移动界面只是该本地会话的一个窗口，因此您的计算机必须保持开启状态，`claude` 进程必须继续运行。

> [!NOTE]
>
> Remote Control 是 Claude Code 的一项功能。有关其他 Claude 产品中的对话，请参阅 [Claude 帮助中心](https://support.claude.com)。

## 要求

在使用 Remote Control 之前，请确认您的环境满足以下条件：

* **订阅**：在 Pro、Max、Ultra、Team 和 Enterprise 套餐中可用。不支持 API 密钥。在 Team 和 Enterprise 上，Owner 必须首先在 [Claude Code 管理员设置](https://claude.ai/admin-settings/claude-code)中启用 Remote Control 切换。
* **自定义角色**：如果您的 Enterprise 组织使用[自定义角色](https://support.claude.com/en/articles/13930452)管理您的访问权限，您的角色必须同时授予您云端会话权限和 **Claude Code remote control** 权限。如果缺少其中任何一项，启动 Remote Control 时会显示 [`Remote Control is disabled by your organization's policy`](#remote-control-is-disabled-by-your-organizations-policy)。请让 Owner 在 [**Organization settings > Roles**](https://claude.ai/admin-settings/roles) 中检查您的角色并添加缺少的权限。
* **身份验证**：运行 `claude` 并使用 `/login` 通过 claude.ai 登录（如果您还没有登录）。如果没有符合条件的登录，`claude remote-control` 会以错误退出，而 `claude --remote-control` 仍会启动交互式会话，并在启动后不久显示 Remote Control 失败通知。
* **API 端点**：在以下任何配置中都不可用：
  * 您使用 Amazon Bedrock、Google Cloud 的 Agent Platform 或 Microsoft Foundry。
  * 您将 [`ANTHROPIC_BASE_URL`](https://code.claude.com/docs/zh-CN/env-vars) 指向 `api.anthropic.com` 以外的主机，例如 [LLM gateway](https://code.claude.com/docs/zh-CN/llm-gateway) 或代理。取消设置该变量以使用 Remote Control。
  * 您通过企业 [Claude apps gateway](https://code.claude.com/docs/zh-CN/claude-apps-gateway) 登录。
* **功能标志评估**：如果您设置了[关闭功能标志评估的环境变量](https://code.claude.com/docs/zh-CN/env-vars.md#features-that-need-feature-flag-fetching)，Remote Control 是否可用取决于您设置的是哪一个：
  * 如果您设置了 `CLAUDE_CODE_DISABLE_NONESSENTIAL_TRAFFIC` 或 `DISABLE_GROWTHBOOK`，Remote Control 不可用。在您的 shell 环境或 [`settings.json` 文件](https://code.claude.com/docs/zh-CN/settings-reference#all-settings)的 `env` 块中取消设置该变量，以使用 Remote Control。
  * 如果您仅设置了 `DISABLE_TELEMETRY` 或 `DO_NOT_TRACK`，Remote Control 保持可用，除非您的组织需要[受信任的设备](#trusted-devices)。如果需要，取消设置该变量以使用 Remote Control。使用设置了任一变量的 Remote Control 需要 Claude Code v2.1.283 或更高版本，在 Claude Desktop 应用中则需要 v2.1.304 或更高版本。
* **工作区信任**：在您尚未信任的目录中，`claude remote-control` 会打印信任该目录会启用的功能，并在启动前询问 `Trust <directory>? [y/N]`。回答 `y` 会保存该选择，但在您的主目录中除外，在主目录中信任永远不会被保存，每次运行时都会返回该问题。当其标准输入或输出不是终端时，该命令无法询问并以 [`Workspace not trusted`](https://code.claude.com/docs/zh-CN/errors#workspace-not-trusted-when-starting-remote-control) 错误退出。

## 启动 Remote Control 会话

您可以从 CLI、[Claude Desktop 应用](https://code.claude.com/docs/zh-CN/desktop.md)或 VS Code 扩展启动 Remote Control 会话。CLI 提供三种调用模式；Desktop 应用和 VS Code 使用 `/remote-control` 命令。

> [tab] 服务器模式
>
> 在项目目录中运行：
>
> ```bash theme={null}
> claude remote-control
> ```
>
> 在您接受 Remote Control 的一次性确认之前，`claude remote-control` 会先说明其作用，并在启动服务器前询问 `Enable Remote Control? (y/n)`。回答 `y` 即可接受并启动服务器。如果您拒绝，Claude Code 会退出而不启动服务器，并在您下次运行该命令时再次询问。
>
> 在服务器模式下，该进程会在终端中持续运行，等待远程连接。它会显示一个会话 URL，您可以用它[从另一台设备连接](#connect-from-another-device)，还可以按空格键显示二维码，以便从手机快速访问。远程会话处于活动状态时，终端会显示连接状态和工具活动。
>
> 在 `remote-control` 之后传入以下任意标志：
>
> | 标志 | 描述 |
> | - | - |
> | `--name "My Project"` | 设置自定义会话标题，该标题会显示在 claude.ai/code 的会话列表中。 |
> | `--remote-control-session-name-prefix <prefix>` | 未设置显式名称时，自动生成的会话名称所用的前缀。默认为您机器的主机名，生成类似 `myhost-graceful-unicorn` 的名称。设置 `CLAUDE_REMOTE_CONTROL_SESSION_NAME_PREFIX` 可达到相同效果。 |
> | `-c`, `--continue` | 恢复此目录中上一个服务器启动时使用的会话，而不是创建新会话。请参阅[停止服务器后恢复会话](#resume-sessions-after-stopping-the-server)。不能与 `--session-id`、`--spawn`、`--capacity` 或 `--create-session-in-dir` 组合使用。需要 Claude Code v2.1.200 或更高版本。 |
> | `--session-id <id>` | 按 ID 恢复某个会话。请参阅[停止服务器后恢复会话](#resume-sessions-after-stopping-the-server)。不能与 `--continue`、`--spawn`、`--capacity` 或 `--create-session-in-dir` 组合使用。需要 Claude Code v2.1.200 或更高版本。 |
> | `--spawn <mode>` | 服务器创建会话的方式。 • `same-dir`（未保存模式时的默认值）：所有会话共享当前工作目录，因此编辑相同文件时可能会发生冲突。 • `worktree`：每个按需创建的会话都有自己的 [git worktree](https://code.claude.com/docs/zh-CN/worktrees.md)。需要 git 仓库或 [`WorktreeCreate` hook](https://code.claude.com/docs/zh-CN/hooks#worktreecreate)。 • `session`：单会话模式。仅服务一个会话并拒绝其他连接。只能在启动时设置。 在运行时按 `w` 可在 `same-dir` 和 `worktree` 之间切换。您切换到的模式会成为[此项目的默认模式](#set-the-default-spawn-mode-for-a-project)。 |
> | `--capacity <N>` | 最大并发会话数。默认为 32。不能与 `--spawn=session` 一起使用。 |
> | `--[no-]create-session-in-dir` | 服务器启动时在当前目录中预先创建一个会话，以便您可以立即开始输入。在 `worktree` 模式下，此会话保留在当前目录中，而按需创建的会话则获得独立的 worktree。默认开启。如果您传入 `--no-create-session-in-dir` 以不带任何会话启动，Claude Code 会在您停止服务器时归档服务器的会话，因此将没有可[恢复](#resume-sessions-after-stopping-the-server)的会话。 |
> | `--permission-mode <mode>` | 为服务器的会话设置初始[权限模式](https://code.claude.com/docs/zh-CN/permission-modes.md)，例如 `acceptEdits`。接受 `manual` 作为 `default` 的别名；无法识别的模式会使服务器在启动时停止并列出有效模式。 |
> | `--chrome` / `--no-chrome` | 在服务器创建的会话中打开或关闭 [Chrome 集成](https://code.claude.com/docs/zh-CN/chrome.md)，以便您在另一台设备上工作时 Claude 可以使用您机器上的 Chrome。如果不带任一标志，服务器预先创建的会话以及您自己从 claude.ai/code 或 Claude 应用启动的任何会话都会以关闭 Chrome 的状态开始，即使您已[默认启用 Chrome](https://code.claude.com/docs/zh-CN/chrome.md#enable-chrome-by-default)。服务器为您的某个[项目](https://code.claude.com/docs/zh-CN/claude-projects.md)线程启动的会话则遵循该设置，但 `bypassPermissions` 模式除外。需要 Claude Code v2.1.273 或更高版本。 |
> | `-d`, `--debug[=<filter>]` | 为服务器开启调试日志，可选择按类别过滤。只能以 `=` 形式传入过滤器，例如 `--debug=api,hooks`。需要 Claude Code v2.1.282 或更高版本。 |
> | `--debug-file <path>` | 将调试日志写入指定文件。 |
> | `--verbose` | 显示详细的连接和会话日志。 |
> | `--sandbox`, `--no-sandbox` | 已在 v2.1.300 中移除。如果您传入任一标志，服务器会[报错退出](https://code.claude.com/docs/zh-CN/errors.md#sandbox-is-not-supported-by-claude-remote-control)，而不是启动。 |
>
> 如果您在 `remote-control` 之前传入全局 `claude` 标志，或者包装脚本添加了此类标志，Claude Code 不会将该标志传递给服务器创建的会话。只有在确知丢弃该标志不会改变这些会话的能力时，Claude Code 才会放行该标志，例如 `--verbose` 或 `--model`。对于任何其他标志，例如 `--settings`，Claude Code 会[拒绝启动](https://code.claude.com/docs/zh-CN/errors.md#not-carried-over-to-the-sessions-remote-control-starts)并指出需要移除的标志。
>
> 要对服务器启动的会话进行沙箱隔离，请在设置文件中开启[沙箱隔离](https://code.claude.com/docs/zh-CN/sandboxing.md)。
>
> 要在终端中打印 `remote-control` 的标志，请运行 `claude remote-control --help`。在 v2.1.302 之前，如果没有符合条件的登录，该命令会返回[故障排除](#troubleshooting)中列出的某个错误。

> [tab] 交互式会话
>
> 要启动启用了 Remote Control 的普通交互式 Claude Code 会话，请使用 `--remote-control` 标志（或 `--rc`）：
>
> ```bash theme={null}
> claude --remote-control
> ```
>
> 可以选择为会话传入一个名称：
>
> ```bash theme={null}
> claude --remote-control "My Project"
> ```
>
> 这会在终端中为您提供一个完整的交互式会话，您还可以从 claude.ai 或 Claude 应用控制它。与 `claude remote-control`（服务器模式）不同，您可以在本地输入消息，同时该会话也可供远程使用。

> [tab] 从现有会话
>
> 如果您已经在 Claude Code 会话中，并希望远程继续该会话，请使用 `/remote-control`（或 `/rc`）命令：
>
> ```text theme={null}
> /remote-control
> ```
>
> 传入名称作为参数以设置自定义会话标题：
>
> ```text theme={null}
> /remote-control My Project
> ```
>
> 这会启动一个 Remote Control 会话，并沿用您当前的对话历史。
>
> 在您接受 Remote Control 的一次性确认之前，`/remote-control` 连接前会出现一个对话框。选择 **Enable Remote Control** 即可接受并连接。如果您选择 **Never mind** 或按 Esc，Claude Code 不会连接，并会在您下次运行 `/remote-control` 时再次询问。
>
> 此命令不支持 `--verbose` 标志。

> [tab] VS Code
>
> 在 [Claude Code VS Code 扩展](https://code.claude.com/docs/zh-CN/vs-code.md)中，在输入框中输入 `/remote-control` 或 `/rc`。
>
> ```text theme={null}
> /remote-control
> ```
>
> Remote Control 开启期间，Claude Code 会在输入框底部显示 **Remote Control** 指示器。会话连接后，点击该指示器即可直接进入会话，或在 [claude.ai/code](https://claude.ai/code) 的会话列表中找到它。Claude Code 还会在对话中发布会话 URL。要断开连接，请再次运行 `/remote-control`。
>
> 与 CLI 不同，VS Code 命令不接受名称参数，也不显示二维码。会话标题来自您的对话历史或第一条提示词。

> [tab] Desktop 应用
>
> 在 [Claude Desktop 应用](https://code.claude.com/docs/zh-CN/desktop.md) Code 标签页的本地会话中，在输入框中输入 `/remote-control` 或 `/rc`。
>
> ```text theme={null}
> /remote-control
> ```
>
> 会话连接后，在 [claude.ai/code](https://claude.ai/code) 的会话列表中找到它。要断开连接，请再次运行 `/remote-control`。
>
> 如果要改为默认为每个会话开启 Remote Control，请参阅[为所有会话启用 Remote Control](#enable-remote-control-for-all-sessions)。

### 检查连接状态

在交互式会话中，Remote Control 已连接时，终端会显示一个 `/rc active` 指示器，该指示器链接到 claude.ai 上的会话。当终端太窄无法容纳时，该指示器会被隐藏。要查看会话 URL 和用于[从另一台设备连接](#connect-from-another-device)的二维码，请再次运行 `/remote-control` 以打开状态面板。您还可以在该面板中断开 Remote Control，同时本地会话继续运行。

如果交互式会话中的连接失败，指示器会变为显示失败状态，Claude Code 会在通知中显示原因并将其添加到对话中。运行 `/remote-control` 即可重新连接，除非原因表明会话已在其他地方发生变化：

* **Another connection took over this session**：另一台设备或另一个 Claude Code 会话现在持有该会话。仅当您想夺回它时才运行 `/remote-control`。
* **This session was ended or archived from another device or app**：仅当您想恢复该会话时才运行 `/remote-control`。Claude Code 会重新打开已归档的会话。
* **The server no longer reports this session**：该会话可能已从另一台设备或应用中删除。

### 从另一台设备连接

Remote Control 会话处于活动状态后，您可以通过以下几种方式从另一台设备连接：

* 在任意浏览器中**打开会话 URL**，直接进入 [claude.ai/code](https://claude.ai/code) 上的会话。
* **扫描**会话 URL 旁显示的**二维码**，直接在 Claude 应用中打开它。使用 `claude remote-control` 时，按空格键可切换二维码显示。
* **打开 [claude.ai/code](https://claude.ai/code) 或 Claude 应用**，在会话列表中按名称找到该会话。在 Claude 移动应用中，点击导航中的 **Code** 进入会话列表。Remote Control 会话在线时会显示带有绿色状态点的电脑图标。

连接后，设备会显示该会话已在后台运行的所有子代理和工作流。从设备上停止其中一个，Claude Code 就会在您的机器上停止该任务。

远程会话标题按以下顺序选择：

1. 您传给 `--name`、`--remote-control` 或 `/remote-control` 的名称
2. 您使用 `/rename` 设置的标题
3. 现有对话历史中最后一条有意义的消息
4. 自动生成的名称，例如 `myhost-graceful-unicorn`，其中 `myhost` 是您机器的主机名或您通过 `--remote-control-session-name-prefix` 设置的前缀

如果您没有设置显式名称，Claude Code 会在您发送提示词后更新标题以反映该提示词。当您从 claude.ai 或 Claude 应用重命名会话时，Claude Code 也会更新 `claude --resume` 中显示的本地标题。

如果您还没有 Claude 应用，请在 Claude Code 中运行 `/mobile` 以显示 [claude.ai/mobile](https://claude.ai/mobile) 的二维码，它会为您的手机打开相应的应用商店。

### 已连接设备看到的内容

已连接的设备会实时显示终端中的对话。以下情况超出了普通消息的范畴：

* **压缩和 `/clear`**：在 Claude Code [压缩对话](https://code.claude.com/docs/zh-CN/context-window.md#what-survives-compaction)时，已连接的设备会显示进度，然后显示对话被压缩的位置。当您运行 `/clear` 时，已连接设备上的对话也会重置。
* **使用 `/resume` 切换对话**：已连接的设备不会收到切换到的对话的标题或早期历史，但双向的新消息都会发往或来自终端中当前打开的对话。要再次从设备上处理原来的对话，请在终端中运行 `/resume` 并切换回该对话。
* **使用 `/teleport` 拉取会话**：当您使用 `/teleport` 将[云端会话](https://code.claude.com/docs/zh-CN/claude-code-on-the-web.md#from-cloud-to-terminal)拉取到终端时，已连接的设备不会收到被拉取对话的早期历史。双向的新消息都会发往或来自被拉取的对话，该对话现在就是终端中打开的对话。
* **来自您其他会话的消息**：借助[跨会话消息](https://code.claude.com/docs/zh-CN/cross-session-messaging.md)，同一连接会在您位于不同机器上的各个会话之间传递消息。
* **您的更改的 diff**：当会话的目录位于 git 仓库中时，已连接设备的 diff 面板会显示您的更改。在领先于仓库默认分支的分支上，该面板会显示自该分支从默认分支分出以来的更改，包括您未提交的编辑。在默认分支本身或不领先于默认分支的分支上，该面板仅显示您未提交的更改。
* **模型**：当您从已连接的设备选择[模型](https://code.claude.com/docs/zh-CN/model-config.md)时，Claude Code 会使用该模型运行会话。需要 Claude Code v2.1.238 或更高版本。从设备的模型控件中选择的模型仅适用于当前会话。当您从设备向交互式会话发送 `/model <name>` 时，Claude Code 还会设置新会话的默认模型。
* **Effort 级别**：当您从已连接的设备使用 `/effort` 或设备的 effort 控件设置 [effort 级别](https://code.claude.com/docs/zh-CN/model-config.md#adjust-effort-level)时，Claude Code 会将其应用到您机器上的会话。如果您已使用 `CLAUDE_CODE_EFFORT_LEVEL` 固定了级别，会话将保持该级别，Claude Code 会拒绝从 effort 控件中选择的其他级别。从 effort 控件中选择级别需要您机器上的 Claude Code 为 v2.1.234 或更高版本。
* **连接失败后重新连接**：运行 `/remote-control` 即可重新连接。如果在此期间压缩重写了对话，或者您使用 `/resume` 切换了对话，Claude Code 会归档它一直在使用的服务器会话，而不是将其留在会话列表中。您仍然可以在 claude.ai/code 的[已归档会话](https://code.claude.com/docs/zh-CN/claude-code-on-the-web.md#find-an-archived-session)中找到该会话。在设备仍处于连接状态时切换对话不会归档该会话。

### 为所有会话启用 Remote Control

除非开启了自动连接，否则只有在您显式运行 `claude remote-control`、`claude --remote-control` 或 `/remote-control` 时，Remote Control 才会激活。要为每个交互式会话开启自动连接，请在 Claude Code 中运行 `/config` 并设置 **Enable Remote Control for all sessions**。该开关有三个值：

* **`true`**：交互式会话启动时自动连接。
* **`false`**：关闭自动连接，但[托管设置](https://code.claude.com/docs/zh-CN/managed-settings.md)中的 `true` 优先级更高，因为 Claude Code 会将此选择保存到您的用户设置中。项目或本地设置（`.claude/settings.json`、`.claude/settings.local.json`）中的 `false` 即使在托管设置为 `true` 时也会关闭自动连接。
* **`default`**：清除您的选择，如果您的组织设置了管理员默认值，则遵循该默认值，否则遵循 Claude Code 的当前默认值。如果您设置了 `DISABLE_TELEMETRY` 或 `DO_NOT_TRACK`，请参阅[设置了 `DISABLE_TELEMETRY` 或 `DO_NOT_TRACK` 时的自动连接](#auto-connect-with-disable-telemetry-or-do-not-track)。

VS Code 扩展和 Desktop 应用也有自动连接开关：

* **VS Code 扩展**：[命令菜单](https://code.claude.com/docs/zh-CN/vs-code.md#use-the-prompt-box) Settings 部分中的 **Enable Remote Control for all sessions**。
* **Desktop 应用**：**Settings > Code** 或 **Settings > Claude Code** 中的 **Connect new sessions to Remote Control**。请参阅[控制哪些会话显示在您的其他设备上](https://code.claude.com/docs/zh-CN/desktop.md#control-which-sessions-appear-on-your-other-devices)。

如果要改为通过设置文件开启自动连接，请在您的用户 `~/.claude/settings.json` 或[托管设置](https://code.claude.com/docs/zh-CN/managed-settings.md)中将 [`remoteControlAtStartup`](https://code.claude.com/docs/zh-CN/settings-reference#remotecontrolatstartup) 设为 `true`。在项目或本地设置（`.claude/settings.json`、`.claude/settings.local.json`）中，Claude Code 会遵循 `false` 并为该仓库关闭自动连接，但会忽略 `true`，因此签入仓库的文件无法为每个打开该仓库的人开启 Remote Control。

自动连接使用您自己的 claude.ai 账户登录，因此它启动的会话仅显示在您自己账户的 Claude 应用中，不会向任何其他人授予访问权限。

开启此设置后，每个交互式 Claude Code 进程都会注册一个远程会话。如果您运行多个实例，每个实例都会获得自己的远程会话。要从单个进程运行多个并发会话，请改用[服务器模式](#start-a-remote-control-session)。

#### 设置了 `DISABLE_TELEMETRY` 或 `DO_NOT_TRACK` 时的自动连接

如果仅设置了 `DISABLE_TELEMETRY` 或 `DO_NOT_TRACK`，您的会话不会通过组织的管理员默认值或 Claude Code 自身的默认值自动连接。如果是其他原因导致您无法使用 Remote Control，例如 `DISABLE_GROWTHBOOK` 或 LLM 网关，请改为从[要求](#requirements)开始排查。

在仅设置了 `DISABLE_TELEMETRY` 或 `DO_NOT_TRACK` 的情况下开启自动连接之前，请确认您的 Claude Code 版本满足[这些变量的要求](#requirements)。然后检查您的组织是否要求 [Trusted Devices](#trusted-devices)：

* **不要求**：在 `/config` 中将 **Enable Remote Control for all sessions** 设为 `true`。
* **要求**：设置了 `DISABLE_TELEMETRY` 或 `DO_NOT_TRACK` 时无法使用 Remote Control。请取消设置这两个变量中您设置的那个。如果您的会话仍未自动连接，请在 `/config` 中将 **Enable Remote Control for all sessions** 设为 `true`。

### 停止服务器后恢复会话

当您使用 Ctrl+C 停止 `claude remote-control` 时，它所服务的会话将不再响应来自您手机或浏览器的操作。只要您没有在同一目录中运行另一个 `claude remote-control`，并且启动此服务器时没有使用 `--no-create-session-in-dir`，Claude Code 就不会归档这些会话。要恢复它们，请在同一目录中运行以下命令之一：

* **`claude remote-control`**：恢复服务器所服务的所有会话。
* **`claude remote-control --continue`**：仅恢复服务器启动时使用的会话，并在该会话结束时退出。如果此目录中没有记录，Claude Code 会使用此仓库其他 git worktree 中最新的记录。
* **`claude remote-control --session-id <id>`**：仅恢复您传入 ID 的会话，并在该会话结束时退出。该 ID 是 claude.ai/code 上会话 URL 中位于 `/code/` 与任何 `?` 之间的部分。

这些命令在服务器停止后约四小时内有效。之后，请运行 `claude remote-control` 启动新会话。如果您在此期间归档了某个会话，在 Claude Code v2.1.228 或更高版本上，`--continue` 和 `--session-id` 会取消归档该会话。

要恢复您使用 `claude --remote-control` 或 `/remote-control` 启动的会话，请使用 `claude --continue` 或 `claude --resume` 恢复对话。关于恢复的对话以何种权限模式启动，请参阅[恢复时的权限模式](https://code.claude.com/docs/zh-CN/sessions.md#permission-mode-on-resume)。如果 Remote Control 没有重新连接，请参阅[无法重新连接到您的 Remote Control 会话](#couldnt-reconnect-to-your-remote-control-session)。

如果您在第二个终端中恢复对话，而第一个终端仍开启着 Remote Control，Claude Code 会在第二个终端中打印 `Remote Control not started here` 通知，并在那里保持 Remote Control 关闭，而不是从第一个终端夺走该会话。在第二个终端中运行 `/remote-control` 即可将 Remote Control 转移到该终端。

当您在 Claude Desktop 或 IDE 扩展中恢复一个曾开启 Remote Control 的对话时，Claude Code 会将其重新关联到现有的 claude.ai 会话，而不是在会话列表中新增一个会话。

### 为项目设置默认 spawn 模式

如果 `claude remote-control` 在启动时打印 `Spawn mode: <mode> (saved for this folder; press w to change)`，则表示服务器正在以之前为此项目保存的 spawn 模式启动。

保存的模式来自您在自己终端中做出的选择。如果 `claude remote-control` 在那里询问过 `Spawn mode for this project:`，Claude Code 会保留您的回答：`1` 表示 `same-dir`，`2` 表示 `worktree`。如果您在其运行时按了 `w`，Claude Code 会保留您切换到的模式，即使该次运行是使用 `--spawn same-dir` 或 `--spawn worktree` 启动的。

* 当服务器以 `same-dir` 或 `worktree` 模式运行时，在服务器的终端中按 `w` 即可切换到另一种模式。切换需要 git 仓库或 `WorktreeCreate` hook。
* 要在某一次运行中使用不同的模式，请传入 `--spawn <mode>`。

## 连接和安全

您的本地 Claude Code 会话仅发出出站 HTTPS 请求，从不在您的机器上打开入站端口。当您启动 Remote Control 时，它向 Anthropic API 注册并轮询工作。当您从另一个设备连接时，服务器通过流连接在网络或移动客户端和您的本地会话之间路由消息。

所有流量都通过 Anthropic API 通过 TLS 传输，与任何 Claude Code 会话的传输安全相同。连接使用多个短期凭证，每个凭证的范围限定为单一目的并独立过期。

Remote Control 连接时，会话记录（包括您的消息、Claude 的响应和工具活动）存储在 Anthropic 服务器上。存储的记录保持您的设备之间的对话同步，并让会话在网络中断后重新连接。执行和文件系统访问保留在您的机器上，存储的记录根据[数据使用](https://code.claude.com/docs/zh-CN/data-usage.md)政策保留。

要完全关闭 Remote Control，请使用 [`disableRemoteControl`](https://code.claude.com/docs/zh-CN/settings-reference#disableremotecontrol) 设置。启用了[零数据保留](https://code.claude.com/docs/zh-CN/zero-data-retention.md)或应用了 [HIPAA 配置](https://code.claude.com/docs/zh-CN/hipaa-setup.md)的组织无法启用 Remote Control。

## 受信任的设备

> [!NOTE]
>
> 受信任的设备目前处于测试阶段。功能和特性可能会随着体验的完善而演变。
>
> 受信任的设备在 Pro、Max、Ultra、Team 和 Enterprise 计划中可用，默认处于关闭状态。在 Team 和 Enterprise 计划中，所有者为组织启用它。在 Pro、Max 和 Ultra 计划中，您可以在设置中的 Cowork 或 Account 页面上自行启用**需要受信任的设备**。

受信任的设备要求您在从 claude.ai、Claude 移动应用或 Claude Desktop 查看或控制 Remote Control 会话之前拥有经过验证的设备。该要求适用于您组织的每个成员，或在 Pro、Max 或 Ultra 计划上仅适用于您自己。它将 Remote Control 访问权限与已知设备和最近的身份验证绑定，而不仅仅是已登录的账户。

当设置打开时，与 Remote Control 会话交互需要以下两项：

* **已注册的设备**：成员用于 Remote Control 的每个浏览器、手机或桌面应用都会注册自己的凭证。注册仅在完整登录后不久提供，因此设备作为真实身份验证的一部分加入受信任列表，而不是在后台静默加入。
* **最近的登录**：成员的登录不能超过 18 小时。成员不需要每天重新登录，而是使用 Face ID、Touch ID、Windows Hello 或通行密钥确认存在。此生物识别步骤立即刷新会话。

生物识别检查通过操作系统或浏览器在设备上运行，与通行密钥登录的机制相同。Anthropic 从不接收或存储指纹、面部数据或任何其他生物识别信息。仅存储设备的公钥和基本元数据，如显示名称、平台和注册时间。

该设置适用于 Claude Code 中的 Remote Control 以及 [Cowork](https://claude.com/docs/cowork/overview) 中的计算机访问。本页介绍 Claude Code 方面的内容。常规 Claude 聊天、终端中的 Claude Code 和 API 使用不受影响。

### 为 Team 或 Enterprise 组织启用受信任的设备

所有者从 claude.ai 组织设置启用该设置。

**1. 转到 Capabilities 页面**

转到 [**Organization settings > Capabilities > Remote sessions**](https://claude.ai/admin-settings/capabilities)。**需要受信任的设备**切换出现在该部分。

**2. 打开需要受信任的设备**

该设置适用于组织的每个成员以及在您启用它后启动的 Remote Control 会话。在切换打开之前已经运行的会话不会被追溯保护，并继续运行而不需要设备要求，直到它们结束。不提供按团队或按项目的范围。

**3. 告诉成员期望什么**

在启用该设置后，成员第一次从浏览器、手机或桌面应用查看或控制新的 Remote Control 会话时，系统会提示他们注册该设备。提前告知他们可以避免混淆。

### 成员看到什么

注册是每个设备的一次性步骤。之后，唯一可见的变化是偶尔的生物识别提示。

* **首次在每个设备上使用**：成员被要求注册。如果他们的登录不是最近的，他们首先通过您的正常流程登录，包括配置的 SSO，然后确认注册。
* **日常使用**：拥有已注册设备和最近登录的成员看不到任何提示。当登录超过 18 小时时，下一次 Remote Control 交互会显示单个 Face ID、Touch ID、Windows Hello 或通行密钥提示。
* **未注册的设备**：Remote Control 会话无法查看或控制，直到设备被注册。该设备上的常规 Claude 聊天不受影响。
* **没有平台身份验证器**：在没有 Face ID、Touch ID 或 Windows Hello 的机器上的成员可以使用硬件安全密钥，或重新登录而不是升级。
* **在终端中**：运行 Claude Code 的机器在开发人员登录到 CLI 时自动接收自己的凭证。终端中没有单独的注册步骤。

### 管理已注册的设备

成员可以从账户设置中查看和撤销自己的设备。

打开 [claude.ai/settings/account](https://claude.ai/settings/account#trusted-devices) 并找到**受信任的设备**部分，查看每个已注册设备及其名称、平台和注册日期。删除设备会立即撤销其凭证，设备可以在新登录后重新注册。凭证如果不续期也会自动过期，因此未使用的设备会自动从受信任列表中删除。

对于丢失或被盗的设备，成员从此页面删除它。如果成员无法登录，管理员可以在管理员控制台中使用**到处登出**为该成员撤销每个会话和已注册设备，之后成员重新注册他们仍然持有的设备。

## 远程控制与云会话

远程控制和[云会话](https://code.claude.com/docs/zh-CN/claude-code-on-the-web.md)都使用 claude.ai/code 界面。关键区别在于会话运行的位置：远程控制在您的机器上执行，因此您的本地 MCP 服务器、工具和项目配置保持可用。云会话在云基础设施上执行，默认由 Anthropic 管理。

当您在进行本地工作中途，想要从另一台设备继续工作时，使用远程控制。当您想要在没有任何本地设置的情况下启动任务、处理您没有克隆的仓库，或并行运行多个任务时，使用云会话。[项目](https://code.claude.com/docs/zh-CN/claude-projects.md)结合了两者：其线程在云中运行，当您在那里请求时，它使用远程控制来[在您的计算机上运行线程](https://code.claude.com/docs/zh-CN/claude-projects.md#run-a-thread-on-your-own-computer)。

Claude Code 提供了多种方式在您不在终端时进行工作。它们在触发工作的方式、Claude 运行的位置以及所需的设置量方面有所不同。

| | 触发方式 | Claude 运行位置 | 设置 | 最适合 |
| :- | :- | :- | :- | :- |
| [Dispatch](https://code.claude.com/docs/zh-CN/desktop.md#sessions-from-dispatch) | 从 Claude 移动应用发送任务消息 | 您的机器（Desktop） | [将移动应用与 Desktop 配对](https://support.claude.com/en/articles/13947068) | 在您离开时委派工作，最少设置 |
| [Remote Control](https://code.claude.com/docs/zh-CN/remote-control.md) | 从 [claude.ai/code](https://claude.ai/code) 或 Claude 移动应用驱动正在运行的会话 | 您的机器（CLI、Desktop 或 VS Code） | 运行 [`claude remote-control` 或 `/remote-control`](https://code.claude.com/docs/zh-CN/remote-control#start-a-remote-control-session) | 从另一台设备控制进行中的工作 |
| [Channels](https://code.claude.com/docs/zh-CN/channels.md) | 从聊天应用（如 Telegram 或 Discord）或您自己的服务器推送事件 | 您的机器（CLI） | [安装频道插件](https://code.claude.com/docs/zh-CN/channels.md#quickstart) 或 [构建您自己的](https://code.claude.com/docs/zh-CN/channels-reference.md) | 对外部事件（如 CI 失败或聊天消息）做出反应 |
| [Slack](https://code.claude.com/docs/zh-CN/slack.md) | 在团队频道中提及 `@Claude` | Anthropic 云 | [安装 Slack 应用](https://code.claude.com/docs/zh-CN/slack.md#setting-up-claude-code-in-slack)，启用 [Claude Code on the web](https://code.claude.com/docs/zh-CN/claude-code-on-the-web.md)，在 Pro、Max 和 Ultra 计划上 | 从团队聊天进行 PR 和审查 |
| [Self-hosted environments](https://code.claude.com/docs/zh-CN/self-hosted-environments) | 启动 [云会话](https://code.claude.com/docs/zh-CN/claude-code-on-the-web.md)并选择您组织的环境 | 您组织的基础设施 | [部署运行器](https://code.claude.com/docs/zh-CN/self-hosted-environments-quickstart.md)，在 Team 和 Enterprise 计划上 | 必须在您的网络内运行的云会话 |
| [Scheduled tasks](https://code.claude.com/docs/zh-CN/scheduled-tasks.md) | 设置计划 | [CLI](https://code.claude.com/docs/zh-CN/scheduled-tasks.md)、[Desktop](https://code.claude.com/docs/zh-CN/desktop-scheduled-tasks.md) 或 [云](https://code.claude.com/docs/zh-CN/routines.md) | 选择频率 | 定期自动化，如每日审查 |

## 移动推送通知

当远程控制处于活跃状态时，Claude 可以向您的手机发送推送通知。

Claude 决定何时推送。它通常在长时间运行的任务完成时或需要您做出决定以继续时发送一条通知。您也可以在提示中请求推送，例如 `notify me when the tests finish`。除了下面的两个开/关切换外，没有按事件配置。

要设置移动推送通知：

**1. 安装 Claude 移动应用**

下载 Claude 应用，适用于 [iOS](https://apps.apple.com/us/app/claude-by-anthropic/id6473753684) 或 [Android](https://play.google.com/store/apps/details?id=com.anthropic.claude)。

**2. 使用您的 Claude Code 账户登录**

使用您在终端中用于 Claude Code 的相同账户和组织。

**3. 允许通知**

接受来自操作系统的通知权限提示。

**4. 在 Claude Code 中启用推送**

在您的终端中，运行 `/config` 并启用 **Push when Claude decides**（当 Claude 决定时推送）以获得主动通知，**Push when actions required**（当需要操作时推送）以获得权限提示和问题，或两者都启用。

如果通知未送达：

* 如果 `/config` 显示 **No mobile registered**（未注册移动设备），请在您的手机上打开 Claude 应用，以便它可以刷新其推送令牌。下次远程控制连接时，警告将清除。
* 在 iOS 上，焦点模式和通知摘要可能会抑制或延迟推送。检查设置 → 通知 → Claude。
* 在 Android 上，激进的电池优化可能会延迟传递。在系统设置中将 Claude 应用从电池优化中豁免。

当您在连接的终端中输入或专注时，Claude Code 会跳过移动推送通知。要将其扩展到您在机器上的任何时间，即使在另一个窗口中，请将 [`CLAUDE_CLIENT_PRESENCE_FILE`](https://code.claude.com/docs/zh-CN/env-vars) 设置为标记文件路径：当文件存在时，通知会被跳过。配置屏幕锁定侦听器或类似工具，以在屏幕解锁时创建文件，在屏幕锁定时删除文件。

## 限制

* **每个交互式进程仅支持一个远程会话**：在服务器模式之外，每个 Claude Code 实例一次仅支持一个远程会话。使用[服务器模式](#start-a-remote-control-session)可从单个进程运行多个并发会话。
* **本地进程必须保持运行**：Remote Control 作为本地进程运行。如果您关闭终端、退出在您的机器上运行该会话的 Desktop 应用或 VS Code，或以其他方式停止 `claude` 进程，会话将离线，直到您[将其恢复](#resume-sessions-after-stopping-the-server)。如果您在远程机器的终端中运行 `claude`，请在 `tmux` 或 `screen` 中启动它，以便在断开 SSH 连接后会话仍保持运行。
* **服务器模式下崩溃的会话**：如果由 `claude remote-control` 提供服务的会话崩溃，请从已连接的设备向其发送一条消息。Claude Code 会重新为其提供服务。您无需重启服务器。需要 Claude Code v2.1.238 或更高版本。
* **已连接会话上的 HTTP 403 拒绝**：交互式会话连接后，如果您的机器与 Anthropic 服务器之间的某个环节以 HTTP 403 响应（例如在 VPN 或网络变更之后可能发生），Claude Code 会持续重试最多三分钟。如果拒绝持续时间更长，Claude Code 将断开连接，并在原因中指明拒绝的来源：网络边缘，或您自身网络上的代理、VPN 或防火墙。
* **长时间网络中断**：如果您的机器处于唤醒状态但无法访问网络，后续操作取决于所用模式：
  * **服务器模式**：Claude Code 大约 10 分钟后放弃，`claude remote-control` 进程退出。再次运行 `claude remote-control` 以启动新会话。
  * **交互式会话**：继续在本地工作。Claude Code 会在中断期间持续重试，并在网络恢复时自动重新连接。
* **无法下载的附件**：如果您从手机或浏览器附加的文件无法下载到您的机器，Claude 仍会收到您的消息以及已下载的文件。Claude Code 会在消息中添加一条说明（例如 `[1 of 3 attachments did not arrive]`）来代替缺失的文件。
* **在线状态心跳失败**：如果交互式会话因 `could not reach the Remote Control server for about 30 minutes` 而断开连接，请运行 `/remote-control` 重新连接。
* **转发的对话框会过期**：Claude Code 会保持权限提示和 `AskUserQuestion` 问题处于打开状态，直到您作答。当 Claude Code 将其他类型的对话框转发到远程会话时（例如安全拒绝后显示的模型选择提示），默认会等待五分钟，然后关闭该对话框并按对话框的无操作默认值继续。设置 [`dialogExpiry`](https://code.claude.com/docs/zh-CN/settings-reference#dialogexpiry) 可调整或禁用该期限。需要 Claude Code v2.1.224 或更高版本。
* **Fable 使用额度同意提示不会被转发**：Claude Code 仅在会话运行的位置显示会话中途的 [Fable 使用额度同意提示](https://code.claude.com/docs/zh-CN/model-config.md#fable-and-usage-credits)，而不会在您的设备上显示。当会话在终端中运行，且在 Claude Code 关闭该提示之前无人在终端作答时，该轮次将结束而不发送请求；请参阅[确认提示未得到回应](https://code.claude.com/docs/zh-CN/errors.md#the-prompt-to-confirm-went-unanswered)。
* **部分命令仅限本地使用**：仅在终端界面中运行的命令（例如 `/plugin` 或 `/resume`）只能从本地 CLI 使用，无论是否传递参数。从移动端或 Web 端输入 `/claude-api` 时同样不可用。Claude 在那里仍可以[自行加载该 skill](https://code.claude.com/docs/zh-CN/skills.md#work-on-claude-api-projects)。以下命令可在移动端和 Web 端使用：
  * 文本输出命令：`/compact`、`/clear`、`/context`、`/usage`、`/exit`、`/usage-credits`、`/recap` 和 `/reload-plugins`。`/usage-credits` 会打印计费 URL，而不是打开浏览器。`/reload-plugins` 仅在会话运行于交互式终端中时有效；没有交互式终端的会话会拒绝执行该命令。
  * `/model`、`/effort`、`/fast`、`/color` 和 `/rename`：将值作为参数传递，例如 `/model sonnet` 或 `/effort high`。在移动端和 Web 端，`/model` 和 `/effort` 接受参数，以代替终端中的选择器或滑块。
  * `/mcp`：在移动应用中，返回服务器状态的文本摘要，而不是打开选择器。在 Web 端，单独使用 `/mcp` 会打开 [claude.ai 连接器](https://code.claude.com/docs/zh-CN/mcp.md#use-mcp-servers-from-claude-ai)目录，而不是返回摘要。当会话运行于交互式终端中时，`reconnect`、`enable` 和 `disable` [子命令](https://code.claude.com/docs/zh-CN/commands.md#all-commands)在两端均可使用。不带服务器名称的 `/mcp reconnect` 会重试所有失败或需要身份验证的服务器。要在不使用选择器的情况下授权 claude.ai 连接器，请参阅[从 shell 重新授权连接器](#authorize-a-connector-again-from-your-shell)。
  * `/config`：在移动应用中，传递 `key=value` 可更改设置项，或不带参数运行以列出可设置的键。在 Web 端，`/config` 会改为打开 **Settings > Code** 或 **Settings > Claude Code**。
  * 在 Team 和 Enterprise 版本中，从移动端或 Web 端运行 `/usage-credits` 不会[向您的管理员发送使用额度请求](https://code.claude.com/docs/zh-CN/costs.md#add-usage-credits-to-your-subscription)。发送请求需要一个仅在交互式 CLI 中出现的确认，因此该命令会提示您改为在 CLI 中运行。
  * `/autocompact`，自 v2.1.221 起：将窗口大小作为参数传递，例如 `/autocompact 500k`。不带参数时，它会以文本形式打印当前窗口大小，而不是打开该命令在终端会话中显示的对话框。
  * `/advisor`，自 v2.1.260 起：将模型作为参数传递，例如 `/advisor opus`，或传递 `off` 以关闭 advisor。两种形式都仅应用于当前会话，不会更改您保存的默认值。不带参数时，它会以文本形式打印当前的 advisor，而不是打开选择器。
  * `/output-style`，自 v2.1.269 起：将样式名称作为参数传递，例如 `/output-style concise`，或不带参数运行以列出样式。在移动端和 Web 端，您只能列出和选择[内置样式](https://code.claude.com/docs/zh-CN/output-styles.md#built-in-output-styles)。要使用[自定义样式](https://code.claude.com/docs/zh-CN/output-styles.md#create-a-custom-output-style)，请在会话本身中选择它。
  * `/focus`，自 v2.1.281 起：将 `on` 或 `off` 作为参数传递，例如 `/focus on`，或不带参数运行以切换[专注视图](https://code.claude.com/docs/zh-CN/commands.md#all-commands)。两种形式都仅应用于当前会话，不会更改您保存的选择。

## 从 shell 重新授权连接器

当您通过 Remote Control 操作的会话中某个 claude.ai 连接器需要身份验证时，移动应用或网页端无法使用 `/mcp` 面板。请在运行该会话的机器上通过终端获取授权链接，然后在您正在使用的设备上打开该链接。您无法从移动应用或网页端运行该命令。从那里发送的以 `!` 开头的内容会作为消息发送给 Claude，而不会在 [shell 模式](https://code.claude.com/docs/zh-CN/interactive-mode.md#shell-mode-with-prefix)中运行。

**1. 获取授权链接**

在运行该会话的机器上的终端中（例如通过 SSH），运行 `claude mcp login` 并附上用引号括起的连接器名称。连接器名称以 `claude.ai` 开头，例如 Slack 连接器的名称为 `claude.ai Slack`。以下命令获取 Slack 连接器的链接：

```bash theme={null}
claude mcp login "claude.ai Slack" --no-browser
```

该命令会输出一个 claude.ai 链接后退出。在浏览器中打开该链接，并在 claude.ai 上完成授权。`--no-browser` 可防止该命令在那台机器上打开浏览器，因为那台机器可能并不是您正在使用的设备。

**2. 在会话中使用连接器**

启动一个新会话，或在已运行的会话中重新连接该连接器：

* **新会话**：授权后启动的会话会自动连接到该连接器，无需任何额外步骤。
* **正在运行的会话**：在 Claude Code 提示符处，或从移动应用或网页端，运行 `/mcp reconnect` 并附上相同的名称（不加引号）。要确认您的会话是否接受从那里发送的该命令，请参阅[哪些命令可在移动端和网页端使用](#limitations)中的 `/mcp` 条目。

以下命令重新连接 Slack 连接器：

```text theme={null}
/mcp reconnect claude.ai Slack
```

在终端中，Claude Code 会输出 `Successfully reconnected to claude.ai Slack`。在移动应用或网页端，回复为 `Reconnected "claude.ai Slack".`

## 故障排除

### "Remote Control disconnected" or "Can't reach"

在 [claude.ai/code](https://claude.ai/code) 或 Desktop 应用中，**Remote Control disconnected** 或 **Can't reach** 消息表示您的浏览器或应用与您计算机上的 Claude Code 失去了联系。这两条消息都无法告诉您 Claude 或您正在查看的子代理是否仍在您的计算机上工作。在该计算机上，检查 Claude Code 所处的状态，并按照相应的步骤重新连接：

* **交互式会话显示 `Remote Control disconnected`**：按照[该行给出的原因所对应的重新连接步骤](#session-ended-elsewhere)操作
* **Claude Code 正在运行且未显示任何故障**：一旦计算机处于唤醒状态并已联网，交互式会话会自动重新连接，因此您无需执行任何操作。如果您使用的是 `claude remote-control`，请参阅[限制](#limitations)下的 **长时间网络中断**
* **Claude Code 未在运行**：[重新启动它并恢复会话](#resume-sessions-after-stopping-the-server)

### "Remote Control requires a claude.ai subscription"

您未使用 claude.ai 账户登录，或者其他凭据优先于您的登录。该消息采用以下形式之一：

* 已登出，来自 `/remote-control` 或 `--remote-control`：`Remote Control requires a claude.ai subscription.` 或 `/remote-control requires a claude.ai subscription.`
* 已登出，来自 `claude remote-control`：`You must be logged in to use Remote Control. Remote Control is only available with claude.ai subscriptions.`
* 已登入，但正在使用 API 密钥或令牌：`Remote Control requires claude.ai subscription auth.` 后跟正在使用的凭据，例如 `ANTHROPIC_API_KEY is set, so this session is using API-key auth`。`apiKeyHelper` 设置和 `ANTHROPIC_AUTH_TOKEN` 的命名方式相同。

运行 `claude auth login` 并选择 claude.ai 选项。如果消息中提到 `ANTHROPIC_API_KEY` 或 `ANTHROPIC_AUTH_TOKEN`，请在设置它的任何地方删除它：您的 shell 环境或[设置文件](https://code.claude.com/docs/zh-CN/settings-reference.md#env)的 `env` 块。如果提到 `apiKeyHelper`，请删除该设置。

### "Remote Control requires a full-scope login token"

您使用来自 `claude setup-token` 或 `CLAUDE_CODE_OAUTH_TOKEN` 环境变量的长期令牌进行了身份验证。这些令牌只能发出模型请求，因此无法建立 Remote Control 会话。运行 `claude auth login` 以改用完整范围的会话令牌进行身份验证。

### "Unable to determine your organization for Remote Control eligibility"

您的缓存账户信息已过期或不完整。运行 `claude auth login` 以刷新它。

### "Remote Control isn't enabled for this account"

Claude Code 检查了您登录的账户的 Remote Control 可用性，检查结果为关闭。通常原因是在计划更改后过期的缓存权利。运行 `claude auth logout` 然后 `claude auth login` 以刷新它们，如果您使用的是旧版本，请更新 Claude Code。

运行 `claude doctor` 以查看哪个单独的资格检查失败。环境变量冲突、无法访问的检查和您的组织的 Remote Control 设置各自产生自己的消息，因此此错误意味着账户级别的检查本身。

在 v2.1.239 之前，此消息读作"Remote Control is not yet enabled for your account"。

### "Couldn't verify Remote Control eligibility"

Claude Code 无法访问功能标志服务来检查是否为您的账户启用了 Remote Control，通常是因为您离线或代理阻止了请求。一旦您有网络访问权限，请重试，或运行 `claude doctor` 以获取详细信息。相关消息"Couldn't verify your organization's Remote Control policy"意味着 Claude Code 在读取该策略时遇到错误，具有相同的修复方法。

### "Remote Control requires feature-flag evaluation"

设置了[环境变量](https://code.claude.com/docs/zh-CN/env-vars.md#features-that-need-feature-flag-fetching)来关闭功能标志评估，完整消息命名了 Claude Code 找到的变量。在 2.1.154 之前的版本上，相同的配置会产生"Remote Control is not yet enabled for your account"。要做什么取决于消息命名的变量：

* **`CLAUDE_CODE_DISABLE_NONESSENTIAL_TRAFFIC` 或 `DISABLE_GROWTHBOOK`**：在设置它的任何地方取消设置变量，在您的 shell 环境或[`settings.json` 文件](https://code.claude.com/docs/zh-CN/settings-reference#all-settings)的 `env` 块中。
* **`DISABLE_TELEMETRY` 或 `DO_NOT_TRACK`**：在 Pro、Max、Ultra、Team 或 Enterprise 套餐上，且 `DISABLE_GROWTHBOOK` 未设置，这些变量会保留 Remote Control 可用，除非您的组织需要[可信设备](#trusted-devices)。如果需要，在设置它的任何地方取消设置变量以使用 Remote Control。在 v2.1.154 到 v2.1.282 上，设置任一变量都会看到此消息，因此请将 Claude Code 更新到满足[这些变量的要求](#requirements)的版本。

### "Remote Control is only available when using Claude via api.anthropic.com"

会话不是直接与 Anthropic API 通信，Remote Control 需要这样做。这发生在 Amazon Bedrock、Google Cloud 的 Agent Platform 和 Microsoft Foundry 上。当 [`ANTHROPIC_BASE_URL`](https://code.claude.com/docs/zh-CN/env-vars) 指向 `api.anthropic.com` 以外的主机时，例如 [LLM 网关](https://code.claude.com/docs/zh-CN/llm-gateway)或代理，即使您使用 claude.ai 登录，也会发生这种情况。有关完整原因列表，请参阅[错误参考](https://code.claude.com/docs/zh-CN/errors.md#remote-control-requires-the-anthropic-api)。

消息命名了将会话路由离开 Anthropic API 的内容，例如 `CLAUDE_CODE_USE_BEDROCK` 或自定义 `ANTHROPIC_BASE_URL`。如果您有符合条件的 claude.ai 登录，请取消设置命名的变量，如果您在[设置](https://code.claude.com/docs/zh-CN/settings.md)中设置了它，请从 `env` 键中删除它，然后重新启动会话。

### "Remote Control is disabled by your organization's policy"

策略阻止了 Remote Control。按顺序检查这些原因：

* **错误提到 `disableRemoteControl`**：您的 IT 管理员已通过[托管设置](https://code.claude.com/docs/zh-CN/managed-settings.md)在此设备上禁用了 Remote Control，独立于组织范围的切换和您的登录方式。
* **您的 claude.ai 套餐是 Pro、Max 或 Ultra**：Claude Code 仍然以来自较早登录的 Team 或 Enterprise 组织身份登录，因此它检查该组织的 Remote Control 策略。运行 `/status` 以查看您的登录使用的套餐和组织。运行 `claude auth logout` 然后 `claude auth login` 以在您当前的套餐下重新登录。
* **消息未说联系您的组织管理员**：您的组织应用了 [HIPAA 配置](https://code.claude.com/docs/zh-CN/hipaa-setup.md)，该配置会关闭 Remote Control。要确认这一点，请运行 `/status` 并在 `Organization configuration` 行中查找 `HIPAA`。所有者会在[管理设置](https://claude.ai/admin-settings/claude-code)中看到 **Remote Control** 切换呈灰显状态，且无法将其打开。如果您对该配置有疑问，请询问所有者，所有者可以联系您组织的 Anthropic 客户团队。在 v2.1.267 之前，此情况显示"Remote Control isn't available for your organization due to its compliance policy"。
* **Remote Control 对组织中的其他人有效，但对您无效**：如果您的 Enterprise 组织使用[自定义角色](https://support.claude.com/en/articles/13930452)管理您的访问权限，则原因可能在于您的角色。Remote Control 同时需要云端会话权限和 **Claude Code remote control** 权限。请让所有者检查您的角色是否同时包含这两项权限，并在 [**Organization settings > Roles**](https://claude.ai/admin-settings/roles) 中添加缺少的权限。
* **否则，所有者尚未为您的组织启用它**：Remote Control 在 Team 和 Enterprise 计划上默认关闭。所有者可以在 [claude.ai/admin-settings/claude-code](https://claude.ai/admin-settings/claude-code) 通过打开 **Remote Control** 切换来启用它。此切换是服务器端组织设置。

在 v2.1.281 之前，当 Claude Code 未在此计算机上加载您的组织策略时，此消息也会出现，例如在离线启动后。更高版本将该状态报告为[`Couldn't verify your organization's policy for remote control`](#couldnt-verify-your-organizations-policy-for-remote-control)。

### "Remote Control was turned off by your organization's policy"

在会话已连接期间，您的组织策略不再允许 Remote Control，因此 Claude Code 断开了该会话的连接。会话之后的状态取决于您启动 Remote Control 的方式：

* **使用 `/remote-control`、`claude --remote-control` 或[自动连接](#enable-remote-control-for-all-sessions)**：会话继续运行但不再使用 Remote Control，Claude Code 会在 claude.ai 上将其归档
* **使用 `claude remote-control`**：服务器停止并归档其所服务的会话，然后退出

要再次找到这些会话之一，请在 claude.ai/code 打开您的[已归档会话](https://code.claude.com/docs/zh-CN/claude-code-on-the-web.md#find-an-archived-session)。

Remote Control 不会自动重新连接。要在您的组织重新允许后再次启用它，请在会话中运行 `/remote-control`，或在 shell 中运行 `claude remote-control`。在此计算机上的 Claude Code 获取到已更改的策略之前，这两个命令都会失败并显示 [`Remote Control is disabled by your organization's policy`](#remote-control-is-disabled-by-your-organizations-policy)。打开的会话大约每小时获取一次策略。要找出阻止 Remote Control 的原因，请将命令输出的完整文本与该条目进行对照。

### "Couldn't verify your organization's policy for remote control"

Claude Code 无法获取您的组织策略，并且此计算机上没有保存的副本可供使用，因此它会保持 Remote Control 关闭，直到它可以确认您的组织允许它。这通常发生在您离线启动 Claude Code 或在 VPN 连接之前，或当代理干扰请求时。在慢速连接上，当第一个请求仍在进行中时，它也可能出现。

消息采用以下形式之一：

* 来自 `/remote-control`、`claude remote-control` 或 `claude --remote-control`：`Couldn't verify your organization's policy for remote control. Check your network connection and try again.`
* 来自[自动连接](#enable-remote-control-for-all-sessions)当会话启动时：`couldn't verify your organization's policy — check your network connection and try again`，在通知中以 `Remote Control failed` 为前缀，在对话中以 `Remote Control disconnected` 为前缀。会话随后保持 Remote Control 关闭。

恢复您的网络连接，然后运行 `/remote-control` 或再次运行该命令。每次尝试都会再次检查策略，因此您无需重新启动 Claude Code。如果消息继续出现，请运行 `claude doctor` 并阅读其 `Organization policy` 行，该行说明策略未加载的原因。

在 v2.1.281 之前，此状态显示 `Remote Control is disabled by your organization's policy`。

### "Remote credentials fetch failed"

Claude Code 无法从 Anthropic API 获取短期凭据来建立连接。使用 `--verbose` 重新运行以查看完整错误：

```bash theme={null}
claude remote-control --verbose
```

常见原因：

* 未登录：运行 `claude` 并使用 `/login` 以您的 claude.ai 账户进行身份验证。Remote Control 不支持 API 密钥身份验证。
* 网络或代理问题：防火墙或代理可能阻止了出站 HTTPS 请求。Remote Control 需要访问端口 443 上的 Anthropic API。
* 会话创建失败：如果您还看到 `Session creation failed — see debug log`，失败发生在设置的早期。检查您的订阅是否有效。

### "Couldn't reconnect to your Remote Control session"

当您使用 `claude --resume` 或 `claude --continue` 恢复对话时，Claude Code 会重新连接到该对话中记录的 Remote Control 会话。此消息意味着重新连接因可能是临时的原因（例如网络中断或服务器错误）而失败，因此 Claude Code 无法确认远程会话是否仍然存在。

运行 `/remote-control` 以重试连接，或使用 `claude --remote-control` 启动新会话以创建新的 Remote Control 会话。您的本地会话在此期间继续运行而不使用 Remote Control。

### "Previous session is unavailable — run /remote-control to start a new one"

Claude Code 无法恢复之前的 Remote Control 会话，而是停止了，而不是自动启动新会话。您可以在使用 `claude --resume` 或 `claude --continue` 恢复对话后看到此消息，或在 Claude Code [在断开连接后自动重新连接](https://code.claude.com/docs/zh-CN/errors.md#remote-control-couldnt-refresh-your-login)后看到此消息。

运行 `/remote-control` 以在当前登录下启动新的 Remote Control 会话；您的本地会话在此期间继续运行而不使用 Remote Control。相关消息 `Remote Control could not verify the signed-in account — run /remote-control to reconnect` 具有相同的修复方法。如果您在 `Previous session is unavailable` 之后运行 `/remote-control` 而不首先重新启动 Claude Code，Claude Code 会将对话的早期消息排除在新会话之外。

### "Remote Control got an unexpected server response"

Remote Control 服务器接受了请求但以此版本的 Claude Code 无法读取的形式回复，同时创建远程会话或获取其凭据。在同一版本上重试会以相同方式失败。运行 `claude update`，然后运行 `/remote-control` 以重新连接。

### "Your organization requires Trusted Devices for Remote Control, but this device is not enrolled"

您的组织已启用[可信设备](#trusted-devices)，此计算机尚未注册。在 Claude Code 中运行 `/login`。注册作为登录的一部分进行，没有单独的注册命令。

### "session expired for trusted-device check"

您的登录已超过 18 小时。在 Claude Code 中运行 `/login`，或在 claude.ai 或移动应用提示您时使用 Face ID、Touch ID、Windows Hello 或通行密钥进行确认。请参阅[可信设备](#trusted-devices)。

## 相关资源

* [网络上的 Claude Code](https://code.claude.com/docs/zh-CN/claude-code-on-the-web.md)：在云中运行会话而不是在您的机器上，通过[云环境](https://code.claude.com/docs/zh-CN/cloud-environments.md)配置
* [跨会话消息传递](https://code.claude.com/docs/zh-CN/cross-session-messaging.md)：让 Claude 在其他机器上或在[云会话](https://code.claude.com/docs/zh-CN/claude-code-on-the-web.md)上向您的会话发送消息
* [Channels](https://code.claude.com/docs/zh-CN/channels.md)：将 Telegram、Discord 或 iMessage 转发到会话中，以便 Claude 在您离开时对消息做出反应
* [Dispatch](https://code.claude.com/docs/zh-CN/desktop.md#sessions-from-dispatch)：从您的手机发送任务消息，它可以生成 Desktop 会话来处理它
* [身份验证](https://code.claude.com/docs/zh-CN/authentication.md)：设置 `/login` 并管理 claude.ai 的凭证
* [CLI 参考](https://code.claude.com/docs/zh-CN/cli-reference.md)：包括 `claude remote-control` 的标志和命令的完整列表
* [安全](https://code.claude.com/docs/zh-CN/security.md)：Remote Control 会话如何适应 Claude Code 安全模型
* [数据使用](https://code.claude.com/docs/zh-CN/data-usage.md)：在本地、Remote Control 和云会话期间通过 Anthropic API 流动的数据
