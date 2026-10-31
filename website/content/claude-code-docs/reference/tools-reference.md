> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# 工具参考

> Claude Code 可以使用的工具的完整参考，包括权限要求和每个工具的行为。

Claude Code 可以访问一组内置工具，帮助它理解和修改您的代码库。工具名称是您在[权限规则](https://code.claude.com/docs/zh-CN/permissions.md#tool-specific-permission-rules)、[子代理工具列表](https://code.claude.com/docs/zh-CN/sub-agents.md)和[hook 匹配器](https://code.claude.com/docs/zh-CN/hooks.md)中使用的确切字符串。

要控制 Claude 可以使用哪些工具以及何时首先询问，请在您的设置、[hooks](https://code.claude.com/docs/zh-CN/hooks.md) 或[子代理的工具列表](https://code.claude.com/docs/zh-CN/sub-agents.md#supported-frontmatter-fields)中配置[权限规则](https://code.claude.com/docs/zh-CN/permissions.md#tool-specific-permission-rules)。有关接受工具名称的每个位置，请参阅[使用权限规则和 hooks 配置工具](#configure-tools-with-permission-rules-and-hooks)。

要添加自定义工具，请连接一个 [MCP 服务器](https://code.claude.com/docs/zh-CN/mcp.md)。要使用可重用的基于提示的工作流扩展 Claude，请编写一个[skill](https://code.claude.com/docs/zh-CN/skills.md)，它通过现有的 `Skill` 工具运行，而不是添加新的工具条目。

> [!NOTE]
>
> 在[自动模式](https://code.claude.com/docs/zh-CN/permission-modes.md#eliminate-prompts-with-auto-mode)中，分类器决定大多数权限提示，而不是您。`Permission required` 列显示工具是否在[手动模式](https://code.claude.com/docs/zh-CN/permission-modes.md)中为工作目录内的路径提示。标记为"否"的文件访问工具，包括 `Read`、`Grep` 和 `Glob`，仍然会为[工作目录和其他目录](https://code.claude.com/docs/zh-CN/permissions.md#working-directories)之外的路径提示。`Bash` 标记为"是"，但运行内置的[只读命令](https://code.claude.com/docs/zh-CN/permissions.md#read-only-commands)而不提示。

| 工具 | 描述 | 需要权限 |
| :- | :- | :- |
| `Agent` | 生成一个[子代理](https://code.claude.com/docs/zh-CN/sub-agents.md)，具有自己的上下文窗口来处理任务。启用[代理团队](https://code.claude.com/docs/zh-CN/agent-teams.md)后，携带 `name` 的调用可以启动一个[队友](https://code.claude.com/docs/zh-CN/agent-teams.md#how-claude-starts-agent-teams)。请参阅 [Agent 工具行为](#agent-tool-behavior) | 否 |
| `Artifact` | 将 HTML 或 Markdown 文件作为 [Artifact](https://code.claude.com/docs/zh-CN/artifacts.md) 发布到 claude.ai：一个私有的交互式页面。您可以通过公开链接共享它，或在 Team 和 Enterprise 计划上在您的组织内共享，其中公开共享需要所有者[启用它](https://code.claude.com/docs/zh-CN/artifacts.md#control-public-sharing)。需要 Pro、Max、Ultra、Team 或 Enterprise 计划和 `/login` 身份验证；请参阅[可用性](https://code.claude.com/docs/zh-CN/artifacts.md#availability) | 是 |
| `AskUserQuestion` | 提出多选问题以收集要求或澄清歧义。问题默认保持打开状态，直到您回答。请参阅 [AskUserQuestion 工具行为](#askuserquestion-tool-behavior) | 否 |
| `Bash` | 在您的环境中执行 shell 命令。请参阅 [Bash 工具行为](#bash-tool-behavior) | 是 |
| `CronCreate` | 在当前会话中安排重复或一次性提示词。任务的范围是会话级别，在 `--resume` 或 `--continue` 时恢复（如果未过期）。请参阅[会话级定时任务](https://code.claude.com/docs/zh-CN/scheduled-tasks.md) | 否 |
| `CronDelete` | 按 ID 取消会话级定时任务 | 否 |
| `CronList` | 列出所有会话级定时任务 | 否 |
| `Edit` | 对特定文件进行有针对性的编辑。请参阅 [Edit 工具行为](#edit-tool-behavior) | 是 |
| `EndConversation` | 结束会话，在持续滥用输入的罕见情况下或当您要求 Claude 演示该工具时。请参阅 [EndConversation 工具行为](#endconversation-tool-behavior) | 否 |
| `EnterPlanMode` | 切换到 Plan Mode 以在编码前设计方法 | 否 |
| `EnterWorktree` | 创建一个隔离的 [git worktree](https://code.claude.com/docs/zh-CN/worktrees.md) 并切换到它。传递 `path` 以切换到现有 worktree，而不是创建新的。首次进入时，目标可能是当前存储库的 worktree，或在多存储库工作区中，是嵌套在其中的存储库的 worktree。在 v2.1.203 之前，嵌套存储库的 worktree 被拒绝。`.claude/worktrees/` 之外的 `path` 会在进入前提示您的批准，因为它会移动会话的工作目录和对该位置的写入访问权限。新 worktree 创建和 `.claude/worktrees/` 下的路径不会提示。在 v2.1.206 之前，Claude 进入 `.claude/worktrees/` 之外的路径而不提示。从 worktree 会话内，或从具有固定工作目录的子代理（例如 [`isolation: worktree`](https://code.claude.com/docs/zh-CN/sub-agents#supported-frontmatter-fields)），只有 `path` 形式可用，目标必须在会话存储库的 `.claude/worktrees/` 下 | 是 |
| `ExitPlanMode` | 呈现计划以供批准并退出 Plan Mode | 是 |
| `ExitWorktree` | 退出 worktree 会话并返回到原始目录。不适用于已在自己的工作目录中运行的子代理，例如 [`isolation: worktree`](https://code.claude.com/docs/zh-CN/sub-agents#supported-frontmatter-fields) | 否 |
| `Glob` | 基于模式匹配查找文件。在 macOS、Linux 和 WSL 上默认不存在。请参阅 [Glob 工具行为](#glob-tool-behavior) | 否 |
| `Grep` | 在文件内容中搜索模式。在 macOS、Linux 和 WSL 上默认不存在。请参阅 [Grep 工具行为](#grep-tool-behavior) | 否 |
| `ListAgents` | 列出 Claude 可以使用 `SendMessage` 消息的代理：会话中的子代理、[代理团队](https://code.claude.com/docs/zh-CN/agent-teams.md)队友、您的其他本地 Claude Code 会话，以及当此会话连接到[远程控制](https://code.claude.com/docs/zh-CN/remote-control.md)时，您的[云会话](https://code.claude.com/docs/zh-CN/claude-code-on-the-web.md)和您在其他机器上的远程控制会话。支持 `/list-agents` 命令。请参阅[跨会话消息传递](https://code.claude.com/docs/zh-CN/cross-session-messaging.md)。需要 Claude Code v2.1.224 或更高版本，仅在[启用跨会话消息传递](https://code.claude.com/docs/zh-CN/cross-session-messaging.md#availability)的会话中出现。队友行和显示此会话自己名称的第一行需要 v2.1.239 或更高版本 | 否 |
| `ListMcpResourcesTool` | 列出连接的 [MCP 服务器](https://code.claude.com/docs/zh-CN/mcp.md)公开的资源，不包括 [MCP Apps UI 资源](https://code.claude.com/docs/zh-CN/mcp.md#reference-mcp-resources)，这些是主机应用程序要呈现的页面 | 否 |
| `LSP` | 通过语言服务器的代码智能：跳转到定义、查找引用、报告类型错误和警告。请参阅 [LSP 工具行为](#lsp-tool-behavior) | 否 |
| `Monitor` | 在后台运行命令并将每个输出行反馈给 Claude，以便它可以对日志条目、文件更改或轮询状态做出反应。还可以打开 WebSocket 并将每条传入消息视为事件。请参阅 [Monitor 工具](#monitor-tool) | 是 |
| `NotebookEdit` | 修改 Jupyter notebook 单元格。请参阅 [NotebookEdit 工具行为](#notebookedit-tool-behavior) | 是 |
| `PowerShell` | 本地执行 PowerShell 命令。请参阅 [PowerShell 工具](#powershell-tool)了解可用性 | 是 |
| `PushNotification` | 发送桌面通知，以及当 [Remote Control](https://code.claude.com/docs/zh-CN/remote-control.md) 连接时的手机推送，以便长时间运行的任务或[会话级定时任务](https://code.claude.com/docs/zh-CN/scheduled-tasks.md)可以在您离开时联系您。推送传递通过 Anthropic 托管的基础设施运行，无法从 Amazon Bedrock、AWS 上的 Claude Platform、Google Cloud 的 Agent Platform 或 Microsoft Foundry 访问 | 否 |
| `Read` | 读取文件的内容。请参阅 [Read 工具行为](#read-tool-behavior) | 否 |
| `ReadMcpResourceTool` | 按 URI 读取特定 MCP 资源 | 否 |
| `RemoteTrigger` | 在 claude.ai 上创建、更新、运行和列出[云端定时任务](https://code.claude.com/docs/zh-CN/routines.md)（以前称为 Routine）。支持 `/schedule` 命令。[`RemoteTrigger` 输入参考](https://code.claude.com/docs/zh-CN/agent-sdk/typescript#remotetrigger)记录了每个操作和删除该工具的组织策略。云端定时任务位于 claude.ai 上，需要 Pro、Max、Ultra、Team 或 Enterprise 计划。因此，此工具无法从 Amazon Bedrock、AWS 上的 Claude Platform、Google Cloud 的 Agent Platform 或 Microsoft Foundry 访问 | 否 |
| `ReportFindings` | 将代码审查发现报告为结构化列表，每个发现都有文件、摘要和失败场景，以便 Claude Code 可以呈现它们而不是将其打印为文本。当活跃的代码审查指令要求它这样做时，Claude 会调用它。发现还可以携带可选的 `category` 段，例如 `correctness` 或 `test-coverage`，显示在呈现列表中的文件位置旁边 | 否 |
| `ScheduleWakeup` | 重新安排[自定步调 `/loop`](https://code.claude.com/docs/zh-CN/scheduled-tasks#let-claude-choose-the-interval)的下一次迭代。Claude 在每次迭代结束时调用此工具以选择下一次运行的时间，在一分钟到一小时之间；您不直接调用它。要改为结束循环，Claude 使用 `stop: true` 调用它，这会取消待处理的唤醒。待处理的唤醒出现在 [Stop hook 输入](https://code.claude.com/docs/zh-CN/hooks.md#stop-input)中的 `session_crons` 中 | 否 |
| `SendFeedback` | 起草关于 Claude Code 的反馈报告，涵盖产品问题或 Claude 在会话中的自身行为，并将其排队在您的机器上供您审查。Claude Code 在您选择发送草稿之前不会发送任何内容。请参阅 [SendFeedback 工具行为](#sendfeedback-tool-behavior)。需要 Claude Code v2.1.238 或更高版本 | 否 |
| `SendMessage` | 向另一个代理发送消息：[代理团队](https://code.claude.com/docs/zh-CN/agent-teams.md)队友、[通过代理 ID 或名称恢复的子代理](https://code.claude.com/docs/zh-CN/sub-agents.md#resume-subagents)，或您的其他 Claude Code 会话之一，在此机器上或超越它。消息传递其他会话需要 Claude Code v2.1.224 或更高版本。[跨会话消息传递](https://code.claude.com/docs/zh-CN/cross-session-messaging.md)涵盖 Claude 可以到达的会话、[消息到达时的样子](https://code.claude.com/docs/zh-CN/cross-session-messaging.md#what-a-message-looks-like)以及[Claude 如何在另一个会话空闲时获得通知](https://code.claude.com/docs/zh-CN/cross-session-messaging.md#get-a-notice-when-another-session-goes-idle)。Claude 可以包含可选的 `summary` 输入，通常为 5-10 个单词，Claude Code 显示为单行预览。当 Claude 在[纯文本消息](https://code.claude.com/docs/zh-CN/cross-session-messaging.md#limitations)上省略它时，Claude Code 使用消息的第一行作为摘要。Claude Code 使用省略号截断长于 200 个字符的摘要 | 否 |
| `SendUserFile` | 从会话向您发送文件，带有可选标题，以便生成的报告、图表、屏幕截图或构建产物到达您的设备，而不仅仅在会话记录中提及。可选的 `display` 输入控制呈现方式：`render` 在客户端中内联打开文件，`attach` 仅显示下载卡，未设置时客户端按文件类型决定。在连接 [Remote Control](https://code.claude.com/docs/zh-CN/remote-control.md) 客户端或在[云端会话](https://code.claude.com/docs/zh-CN/claude-code-on-the-web.md)中时可用。传递通过 Anthropic 托管的基础设施运行，因此该工具在 Amazon Bedrock、Google Cloud 的 Agent Platform 或 Microsoft Foundry 上不可用 | 否 |
| `ShareOnboardingGuide` | 上传 `ONBOARDING.md` 并返回队友可以在 Claude Code 中打开的共享链接。在编写指南后从 `/team-onboarding` 调用。适用于 Team 和 Enterprise 计划上的 claude.ai 订阅者 | 是 |
| `Skill` | 在主对话中执行[skill](https://code.claude.com/docs/zh-CN/skills.md#control-who-invokes-a-skill) | 是 |
| `SubagentHandback` | 将子代理的最终报告传递给接收该子代理结果的任何对话。仅在[自动模式](https://code.claude.com/docs/zh-CN/permission-modes.md#eliminate-prompts-with-auto-mode)中提供，给 Agent 工具在本地运行的子代理，除了[分叉](https://code.claude.com/docs/zh-CN/sub-agents.md#fork-the-current-conversation)，并在终端 CLI、IDE 扩展、网络版会话和 Agent SDK 中可用；分类器在传递报告前审查它。需要 Claude Code v2.1.271 或更高版本 | 否 |
| `TaskCreate` | 在任务列表中创建新任务。仅在[任务工具可用性](#task-tool-availability)下列出的模型上默认提供，在其他模型上当您选择加入时提供 | 否 |
| `TaskGet` | 检索特定任务的完整详细信息。仅在[任务工具可用性](#task-tool-availability)下列出的模型上默认提供，在其他模型上当您选择加入时提供 | 否 |
| `TaskList` | 列出所有任务及其当前状态。仅在[任务工具可用性](#task-tool-availability)下列出的模型上默认提供，在其他模型上当您选择加入时提供 | 否 |
| `TaskOutput` | 已在 v2.1.277 中移除。之前用于从后台任务检索输出；Claude 改为使用 `Read` 读取后台任务的[输出文件](https://code.claude.com/docs/zh-CN/interactive-mode.md#how-backgrounding-works) | 否 |
| `TaskStop` | 按 ID 停止运行中的后台任务。它还接受 [agent team 队友](https://code.claude.com/docs/zh-CN/agent-teams.md)或按 Agent ID 或名称指定的命名后台 Agent。当没有任务与 ID 匹配时，错误按 ID 和描述列出运行中的后台 Agent，包括由另一个 Agent 生成的 Agent | 否 |
| `TaskUpdate` | 更新任务状态、依赖项、详细信息或删除任务。仅在[任务工具可用性](#task-tool-availability)下列出的模型上默认提供，在其他模型上当您选择加入时提供 | 否 |
| `TodoWrite` | 管理会话任务清单。默认禁用，改为使用 `TaskCreate`、`TaskGet`、`TaskList` 和 `TaskUpdate`。设置 `CLAUDE_CODE_ENABLE_TASKS=0` 以在[具有任务跟踪工具的会话](#task-tool-availability)中重新启用它 | 否 |
| `ToolSearch` | 当[工具搜索](https://code.claude.com/docs/zh-CN/mcp.md#scale-with-mcp-tool-search)启用时，搜索并加载延迟工具 | 否 |
| `WaitForMcpServers` | 等待一个或多个仍在后台连接的 [MCP 服务器](https://code.claude.com/docs/zh-CN/mcp.md)，以便请求可以使用它们的工具而无需重启会话。当所需的服务器尚未连接时，Claude 会调用它。仅在[工具搜索](https://code.claude.com/docs/zh-CN/mcp.md#scale-with-mcp-tool-search)禁用时出现，因为启用时 `ToolSearch` 处理等待 | 否 |
| `WebFetch` | 从指定的 URL 获取内容。请参阅 [WebFetch 工具行为](#webfetch-tool-behavior) | 是 |
| `WebSearch` | 执行网络搜索。请参阅 [WebSearch 工具行为](#websearch-tool-behavior) | 是 |
| `Workflow` | 运行[动态工作流](https://code.claude.com/docs/zh-CN/workflows.md)：在后台编排许多子代理并返回一个合并结果的脚本 | 是 |
| `Write` | 创建或覆盖文件。请参阅 [Write 工具行为](#write-tool-behavior) | 是 |

## 使用权限规则和 hooks 配置工具

在大多数情况下，Claude 会决定何时使用这些工具，您在与 Claude 交互时不需要自己命名它们。在定义权限和其他配置时，您直接引用工具名称：

* 在设置中的 [`permissions.allow`](https://code.claude.com/docs/zh-CN/settings-reference#permissions-allow) 和 [`permissions.deny`](https://code.claude.com/docs/zh-CN/settings-reference#permissions-deny)，以及 `/permissions` 界面
* 在 [`--allowedTools` 和 `--disallowedTools`](https://code.claude.com/docs/zh-CN/cli-reference) CLI 标志中
* 在 Agent SDK 的 [`allowedTools` 和 `disallowedTools`](https://code.claude.com/docs/zh-CN/agent-sdk/permissions#allow-and-deny-rules) 选项中
* 在 [skill 的 `allowed-tools`](https://code.claude.com/docs/zh-CN/skills#frontmatter-reference) frontmatter 中
* 在 hook 的 [`if` 条件](https://code.claude.com/docs/zh-CN/hooks-guide#filter-by-tool-name-and-arguments-with-the-if-field)中

所有这些都接受相同的规则格式 `ToolName(specifier)`。specifier 取决于工具，多个工具共享一种格式：

| 规则格式 | 适用于 | 详情 |
| :- | :- | :- |
| `Bash(npm run *)` | Bash, Monitor | [命令模式匹配](https://code.claude.com/docs/zh-CN/permissions.md#bash) |
| `PowerShell(Get-ChildItem *)` | PowerShell | [命令模式匹配](https://code.claude.com/docs/zh-CN/permissions.md#powershell) |
| `Read(~/secrets/**)` | Read, Grep, Glob, LSP | [路径模式匹配](https://code.claude.com/docs/zh-CN/permissions.md#read-and-edit) |
| `Edit(/src/**)` | Edit, Write, NotebookEdit | [路径模式匹配](https://code.claude.com/docs/zh-CN/permissions.md#read-and-edit) |
| `Skill(deploy *)` | Skill | [Skill 名称匹配](https://code.claude.com/docs/zh-CN/skills.md#restrict-claude%E2%80%99s-skill-access) |
| `Agent(Explore)` | Agent | [Subagent 类型匹配](https://code.claude.com/docs/zh-CN/permissions.md#agent-subagents) |
| `WebFetch(domain:example.com)` | WebFetch | [域名匹配](https://code.claude.com/docs/zh-CN/permissions.md#webfetch) |
| `WebSearch` | WebSearch | 无 specifier；允许或拒绝整个工具 |

此处未列出的工具，例如 `ExitPlanMode` 或 `ShareOnboardingGuide`，仅接受不带 specifier 的裸工具名称。

`Edit(...)` 允许规则也授予对相同路径的读取访问权限，因此您不需要匹配的 `Read(...)` 规则。`Read(...)` 拒绝规则也会阻止同一路径上的 Edit 和 Write 工具，包括在该处创建新文件，因为两个工具都会更改 Claude 必须能够读回的内容。对于 Write 工具，`Read` 拒绝检查需要 Claude Code v2.1.228 或更高版本。

Hook `matcher` 字段使用裸工具名称，而不是带括号的规则格式。有关匹配规则，请参阅 [matcher 模式](https://code.claude.com/docs/zh-CN/hooks.md#matcher-patterns)。有关每个工具在 hooks 中传递给 `tool_input` 的字段名称，请参阅 [PreToolUse 输入参考](https://code.claude.com/docs/zh-CN/hooks.md#pretooluse-input)。

## Agent tool 行为

Agent tool 在单独的上下文窗口中生成一个子代理。子代理自主地完成其任务，然后向父对话返回其结果。父对话看不到子代理的中间 tool 调用或输出，只能看到最终结果。同一个 tool 也可以启动 teammate 或 fork：

* **Teammate**：启用 [agent teams](https://code.claude.com/docs/zh-CN/agent-teams.md) 后，携带 `name` 的调用可以改为启动一个 [teammate](https://code.claude.com/docs/zh-CN/agent-teams.md#how-claude-starts-agent-teams)，它通过团队消息而不是返回结果来报告
* **Fork**：在 [fork mode](https://code.claude.com/docs/zh-CN/sub-agents.md#turn-fork-mode-on-or-off) 打开的任何地方，该 tool 都可以启动一个 [forked subagent](https://code.claude.com/docs/zh-CN/sub-agents.md#fork-the-current-conversation)，它继承完整的父对话，而不是从头开始

要限制子代理运行的轮数，请在 [subagent definition](https://code.claude.com/docs/zh-CN/sub-agents.md#supported-frontmatter-fields) 中设置 `maxTurns`。当子代理达到限制时，Claude Code 将返回的结果标记为部分输出，Claude 可以 [resume the subagent](https://code.claude.com/docs/zh-CN/sub-agents.md#resume-subagents) 来继续。

非 fork 子代理可以使用哪些 tools 取决于 [subagent definition](https://code.claude.com/docs/zh-CN/sub-agents.md) 中的 `tools` 和 `disallowedTools` 字段：

* **两个字段都未设置**：子代理继承每个 [tool available to subagents](https://code.claude.com/docs/zh-CN/sub-agents.md#available-tools)。
* **仅 `tools`**：子代理仅获得列出的 tools。
* **仅 `disallowedTools`**：子代理获得除列出的 tools 之外的每个父 tool。
* **两者都设置**：`disallowedTools` 优先。同时列在两者中的 tool 被移除。

在任何情况下，解析的集合都限于 [tools available to subagents](https://code.claude.com/docs/zh-CN/sub-agents.md#available-tools)：不可用于子代理的 tool 永远不会被授予，即使在 `tools` 中列出。在 `SubagentHandback` tools-table 条目中的条件成立的地方，Claude Code 也会给子代理该 tool，即使您将其排除在 `tools` 之外或在 `disallowedTools` 中列出它。

如果子代理的 `tools` 列表中的每个条目都无法匹配可用的 tool，Agent tool 通常会返回一个错误，命名这些条目而不是启动子代理；请参阅 [Agent would be spawned with zero tools](https://code.claude.com/docs/zh-CN/errors.md#agent-would-be-spawned-with-zero-tools) 了解消息以及如何修复每个条目。

启动子代理本身不会提示权限。Claude Code 在运行时根据您的权限规则检查子代理自己的 tool 调用。

您看到子代理权限提示的位置取决于它是在前台还是后台运行。Claude Code 默认在后台运行子代理，除了 [cases that run in the foreground](https://code.claude.com/docs/zh-CN/sub-agents.md#run-subagents-in-foreground-or-background)。

* **前台子代理**显示您在主对话中会看到的相同权限提示，在每个 tool 调用发生时。
* **后台子代理**在您的主会话中显示权限提示。提示会注明是哪个子代理在请求，按 Esc 会拒绝该单个 tool 调用而不停止子代理。

要 [limit what a subagent can reach](https://code.claude.com/docs/zh-CN/sub-agents.md#control-subagent-capabilities)，首先缩小其 `tools` 字段，例如通过将 Bash 排除在列表之外，或在您的设置中设置拒绝规则。

## AskUserQuestion 工具行为

Claude 使用 `AskUserQuestion` 在需要决策或澄清时向您提出多选题。通过选择一个选项来回答，或通过 `Other` 行或备注字段输入您自己的文本。

当您通过输入自己的文本来回答时，Claude Code 会用中立的措辞转达答案，以便 Claude 遵循您写的内容，包括等待或先解释的请求。

### 问题自动继续超时

问题保持打开状态，直到您回答。如果您想让一个未回答的问题最终关闭并让 Claude 在没有您的情况下继续，请在您的用户 `settings.json` 中或从 `/config` 中的 **Question auto-continue timeout** 行设置 [`askUserQuestionTimeout`](https://code.claude.com/docs/zh-CN/settings-reference#askuserquestiontimeout) 为 `60s`、`5m` 或 `10m`。

问题在没有输入的情况下保持该长时间后，对话框会自动关闭：它会提交您已经选择的任何选项，并告诉 Claude 您可能离开了键盘，因此 Claude 会根据自己的判断继续进行，稍后可以重新提问。您会看到最后 20 秒的倒计时。按任意键可重启计时器。当终端报告其窗口处于焦点状态时，计时器不会倒计时。

对于 Claude 在[后台会话](https://code.claude.com/docs/zh-CN/agent-view.md)中、在[屏幕阅读器模式](https://code.claude.com/docs/zh-CN/accessibility.md)下，或在会话连接到 [Remote Control](https://code.claude.com/docs/zh-CN/remote-control.md) 时提出的问题，计时器永远不会启动。这些问题会一直等待，直到您回答。超时仅适用于 `AskUserQuestion` 的多选题；权限提示（包括计划批准）在空闲时永远不会自动解决。

## Bash 工具行为

Bash 工具在单独的进程中运行每个命令。

### 命令之间保留的内容

* 当 Claude 在主会话中运行 `cd` 时，新的工作目录会传递到后续的 Bash 命令，只要它保持在项目目录内或使用 `--add-dir`、`/add-dir` 或 settings 中的 `additionalDirectories` 添加的[额外工作目录](https://code.claude.com/docs/zh-CN/permissions.md#working-directories)内。这包括 Claude 响应您后续消息时运行的命令。
  * 子代理会话永远不会传递工作目录更改。
  * 如果 `cd` 落在这些目录之外，Claude Code 会重置为项目目录，并将 `Shell cwd was reset to <dir>` 附加到工具结果中。
  * 要禁用此传递，使每个 Bash 命令都在项目目录中启动，请设置 `CLAUDE_BASH_MAINTAIN_PROJECT_WORKING_DIR=1`。
* 环境变量不会保留。一个命令中的 `export` 在下一个命令中不可用。
* 在您的 shell 启动文件中定义的别名和 shell 函数可用。在会话启动时，Claude Code 会获取 `~/.zshrc`、`~/.bashrc` 或 `~/.profile`（取决于您的 shell），捕获生成的别名、函数和 shell 选项，并将它们应用于每个 Bash 命令。

在启动 Claude Code 之前激活您的 virtualenv 或 conda 环境。要使环境变量在 Bash 命令之间保留，请在启动 Claude Code 之前将 [`CLAUDE_ENV_FILE`](https://code.claude.com/docs/zh-CN/env-vars) 设置为 shell 脚本，或使用 [SessionStart hook](https://code.claude.com/docs/zh-CN/hooks.md#persist-environment-variables) 动态填充它。

### 超时和输出限制

每个命令在超时下运行，Claude 管理它：当它需要比命令的默认值更长的时间时，它会传递 `timeout` 参数进行该调用。您永远不会设置每个命令的超时。

两个[环境变量](https://code.claude.com/docs/zh-CN/env-vars.md)控制 Claude 获得的内容，用于在前台运行的命令：

* `BASH_DEFAULT_TIMEOUT_MS` — 当 Claude 不传递超时时的默认值；开箱即用为两分钟
* `BASH_MAX_TIMEOUT_MS` — 使用默认值，设置上限以限制 Claude 请求的任何内容：有效上限是两者中较大的，开箱即用为十分钟

在具有[后台命令时间限制](#time-limit-for-background-commands)的会话中，对于 Claude 在后台启动的命令，`timeout` 改为设置命令在那里可以运行多长时间，并使用该限制单独的默认值和最大值。[PowerShell 工具](#powershell-tool)遵循相同的超时规则并读取相同的两个变量。

#### 输出限制

Claude Code 在命令运行时将命令的输出流式传输到工作文件；输出超过 5 GB 的命令会被杀死。命令完成后，Claude Code 从该文件读取输出，最多读取下面描述的读回窗口。输出中有多少到达 Claude 取决于 Claude Code 是否将结果视为失败：

| 结果 | Claude 获得的内容 |
| :- | :- |
| 有效 | 内联最多约 30,000 个字符（默认）；超过该值，为保存到会话目录的文件的路径（文件超过 64 MiB 的部分会被截断），加上最多前 2,000 个字符的预览，Claude 在需要其余部分时读取或搜索该文件 |
| 失败 | 内联最多约 10,000 个字符；超过该值，从读回窗口中切割的该大小的头尾摘录，没有文件路径 |

退出码为 1 的命令仅当 Claude Code 识别退出码 1 为该命令的良性结果时，才计为 Bash 工具的有效结果：`grep`、`rg`、`egrep`、`fgrep`、`find`、`diff`、`test` 和 `[`，加上 `git diff` 和 `git grep`。退出码为 1 的所有其他命令都计为失败，即使退出 1 是良性信息结果：`pgrep` 和 `jq -e` 没有匹配项，`cmp` 的文件不同。

[`BASH_MAX_OUTPUT_LENGTH`](https://code.claude.com/docs/zh-CN/env-vars) 设置 Claude Code 从工作文件读回到命令结果中的输出字符数：默认 30,000，最多 150,000。当您的命令经常溢出该窗口时（例如详细的构建或完整的测试套件日志），请提高它。提高它会扩大读回窗口，这也是失败命令的摘录被切割的窗口。它不会提高内联上限：超过内联上限的有效结果作为文件路径加预览到达，无论此变量如何。

要更改有效结果中 Claude 接收的内联内容量，请改为设置 [`bashOutputMaxChars`](https://code.claude.com/docs/zh-CN/settings-reference#bashoutputmaxchars) 设置，最多 128,000 个字符。它同时调整内联上限和读回窗口的大小，Claude Code 然后忽略 `BASH_MAX_OUTPUT_LENGTH`。需要 Claude Code v2.1.261 或更高版本。

### 后台命令

对于长时间运行的进程（例如开发服务器或监视构建），Claude 可以设置 `run_in_background: true` 以将命令作为后台任务启动并在其运行时继续工作。使用 `/tasks` 列出和停止后台任务。在您从那里停止一个后，或从连接的客户端（例如桌面应用）停止，Claude 继续而不是等待。如果子代理启动了命令，则是该子代理继续。

#### 后台命令何时停止

[前台子代理](https://code.claude.com/docs/zh-CN/sub-agents.md#run-subagents-in-foreground-or-background)启动的命令在该子代理的运行结束时停止，无论它是完成、失败还是被中断。主对话或后台子代理启动的命令在最终回复后继续运行，直到它退出、被停止或达到其[时间限制](#time-limit-for-background-commands)。

当主对话启动的命令仍在运行时，使用 `-p` 标志的非交互模式运行会[在其结果之后保持打开](https://code.claude.com/docs/zh-CN/headless.md#background-tasks-at-exit)，直到该命令退出或达到其时间限制。后台子代理启动的命令会在运行退出时被停止。

#### 后台命令的时间限制

在无人值守运行的会话中，例如使用 `-p` 标志的运行、Agent SDK 应用程序、CI 作业或云端会话，后台 Bash 和 PowerShell 命令有时间限制。您在终端、桌面应用或 VS Code 扩展中使用的本地会话对后台命令没有时间限制。

时间限制需要 Claude Code v2.1.285 或更高版本。在 v2.1.288 之前，它适用于每个会话。

时间限制从命令进入后台的时刻开始计算：

* Claude 在后台启动的命令获得 30 分钟，或 Claude 使用 `run_in_background` 传递的 `timeout`，最多 2 小时
* 在前台启动然后移到后台的命令，例如在其超时时，从移动时获得 30 分钟

在使用 `-p` 标志且以文本形式（而不是使用 `--input-format stream-json`）传递提示词的运行中，两个默认值都是 10 分钟而不是 30 分钟，因为该运行会[在其结果之后等待后台命令](https://code.claude.com/docs/zh-CN/headless.md#background-tasks-at-exit)。

当后台命令达到其时间限制时，Claude Code 停止它并告诉 Claude 原因，Claude 可以使用更长的 `timeout` 重新启动命令，如果工作仍然需要的话。停止通知读作 `Background command "<description>" was stopped after reaching its background time limit`。

#### 提高后台命令的时间限制

两个[环境变量](https://code.claude.com/docs/zh-CN/env-vars.md)提高这些限制，对于 Bash 和 PowerShell 命令都是如此。两者都采用毫秒，都不能缩短限制：较低的值保留默认值和 2 小时的最大值。

* 将 `BASH_DEFAULT_TIMEOUT_MS` 设置为高于 `1800000` 以用该值替换 30 分钟的默认值，既适用于 Claude 启动的没有 `timeout` 的命令，也适用于移动的命令。在以文本形式传递提示词的 `-p` 运行中，任何高于 `600000` 的值都会替换其 10 分钟的默认值
* 将 `BASH_MAX_TIMEOUT_MS` 设置为高于 `7200000` 以将 2 小时的最大值提高到该值。将 `BASH_DEFAULT_TIMEOUT_MS` 设置为高于 `7200000` 以相同方式提高最大值

#### 移到后台的前台命令

当前台命令在完成前达到其超时时，Claude Code 会将其移到后台而不是停止它，除非命令以 `sleep` 开头。移动的命令的[时间限制](#time-limit-for-background-commands)从移动时开始计算，前台子代理的移动命令仍然在该子代理的运行结束时停止。

设置 [`CLAUDE_CODE_DISABLE_BACKGROUND_TASKS=1`](https://code.claude.com/docs/zh-CN/env-vars#variables) 或在 [bare 模式](https://code.claude.com/docs/zh-CN/headless.md#start-faster-with-bare-mode)下运行会禁用自动后台处理，因此达到超时的命令会改为停止。

移到后台的命令的结果说明发生了什么：

* 当超时触发移动时，结果明确报告：`Command did not complete within its 120s timeout and was moved to the background`，秒数与应用的超时匹配，后跟任务 ID 和输出被写入的文件路径。
* 移到后台的命令内的 `cd`、`pushd`、`popd` 或 `chdir` 永远不会传递：结果说明 `Session cwd remains <dir>; directory changes made by the backgrounded command do not apply to subsequent commands.`，所以 Claude 不会对没有发生的目录更改进行操作。

### Linux 和 WSL 上的内存限制

在 Linux 和 WSL 上，设置 [`CLAUDE_CODE_TOOL_MEMORY_LIMIT`](https://code.claude.com/docs/zh-CN/env-vars#variables) 为大小（例如 `4G`）以限制 Bash、PowerShell 和 [Monitor](#monitor-tool) 工具命令可以使用的内存，这样一个失控的构建就不会占用会话其余部分需要的内存。需要 Claude Code v2.1.233 或更高版本。在 v2.1.246 之前，Monitor 工具命令在上限之外运行。

* 将大小写为字节数或带有 `K`、`M`、`G` 或 `T` 后缀。设置 `0`、`off`、`false`、`no` 或 `none` 以关闭上限。Claude Code 忽略任何其他无法读取为大小的值，例如 `4e9`。
* Claude Code 将会话的所有 Bash、PowerShell 和 Monitor 命令计入一个上限，而不是每个命令各自计入。
* Claude Code 使用内存 cgroup 应用上限。当它无法设置 cgroup 时，命令在没有上限的情况下运行，来自 `claude --debug` 的调试日志说明原因。
* 在 Claude Code 启动的第一个进程打开上限后，或因为关闭值或失败的 cgroup 设置而关闭它后，Claude Code 保持该结果直到您重新启动。要应用更改或删除的值，或修复后的设置，再次启动 `claude`。
* 当命令无法保持在上限以下时，内核杀死命令，其结果中没有任何内容命名上限。

Claude Code 也可以将它启动的其他类型的进程计入同一限制。设置 [`CLAUDE_CODE_TOOL_MEMORY_CGROUP_EXCLUDE`](https://code.claude.com/docs/zh-CN/env-vars#variables) 为逗号分隔的类型列表以豁免上限；Claude Code 对不在您列表中的每种类型应用上限。设置为 `none` 以限制每种类型，或设置为 `all-new` 以仅限制 Bash、PowerShell 和 Monitor 工具命令。需要 Claude Code v2.1.246 或更高版本。您可以命名的类型：

* `mcp`: 本地 [MCP 服务器](https://code.claude.com/docs/zh-CN/mcp.md)
* `lsp`: [language servers](#lsp-tool-behavior)
* `hooks`: [hook](https://code.claude.com/docs/zh-CN/hooks.md) 命令
* `plugin`: [插件](https://code.claude.com/docs/zh-CN/plugins/overview.md)运行的命令
* `helper`: Claude Code 自己的辅助命令，例如 `git`
* `agent`: 子 Claude Code 进程，例如 [agent teammates](https://code.claude.com/docs/zh-CN/agent-teams.md)

无论您列出什么，这些规则适用：

* **未知名称**：Claude Code 忽略它不识别的名称
* **变量未设置**：Claude Code 从 Anthropic 从服务器传递的配置中获取其他限制类型的集合，该集合可能随时间变化，因此当您需要不变的集合时设置变量
* **权限门控 hook**：即使每种类型都受限，Claude Code 也会从上限中排除可以阻止或更改操作结果的 hook，以及任何此类 hook 调用的 MCP 服务器，因此内核杀死权限门控 hook 不能允许它阻止的操作

## Edit 工具行为

Edit 工具执行精确字符串替换。它接受一个 `old_string` 和一个 `new_string`，并用后者替换前者。它不使用正则表达式或模糊匹配。

编辑应用必须通过以下检查。在任何检查之前，与 [`Read` 拒绝规则](https://code.claude.com/docs/zh-CN/permissions#tool-specific-permission-rules)匹配的路径会被拒绝，包括在该路径创建新文件。

* **编辑前读取**：Claude 在编辑文件前在当前对话中读取该文件，并且以 [`PARTIAL view` 通知](#large-files)中断的读取不计数。Claude Opus 4.6、Claude Haiku 4.5 和更早的模型始终需要读取。较新的模型可以在读取不需要权限提示且 Read 工具可用时编辑未读文件。
* **匹配**：`old_string` 必须在文件中完全按照编写的方式出现。单个空格或缩进差异足以导致不匹配。
* **唯一性**：`old_string` 必须恰好出现一次。当它出现多次时，Claude 要么提供一个更长的字符串，其周围上下文足以确定一个出现位置，要么设置 `replace_all: true` 来替换所有出现位置。

当 `old_string` 与当前内容完全匹配且明确无误，且 Claude Code 可以在不提示的情况下读取文件时，在 Claude 最后读取后在磁盘上更改的文件仍然可以编辑。针对文件的当前内容进行匹配可以保持安全，结果会注明该文件包含其他更改，以便 Claude 在依赖周围内容的编辑前重新读取它。在任何其他情况下，例如过时的 `old_string` 或在没有 `replace_all` 的情况下匹配多次的情况，Claude 在编辑前再次读取文件。

Claude 使用 `cat` 或 `grep` 等 Bash 命令查看文件后，也可以在不单独执行 Read 的情况下编辑该文件。这些命令是 `cat`、`nl`、`bat`、`batcat`、`head`、`tail`、`sed -n 'X,Yp'`、`grep`、`egrep`、`fgrep` 和 `rg`，且每个命令都需在单个文件上运行，没有管道或重定向。如果命令没有输出该文件的任何文本（例如搜索没有匹配到任何内容时），编辑前读取检查仍然适用。

当 Claude 以这种方式查看文件时，Claude Code 还会加载适用于该文件的任何[子目录 `CLAUDE.md`](https://code.claude.com/docs/zh-CN/memory#how-claude-md-files-load) 和[路径范围规则](https://code.claude.com/docs/zh-CN/memory.md#path-specific-rules)。请参阅 [Read 和 Edit 权限规则](https://code.claude.com/docs/zh-CN/permissions.md#read-and-edit)，了解您的 `Read` 和 `Edit` 拒绝规则涵盖哪些 Bash 命令。

### 非 UTF-8 文件

Claude 无法对不是有效 UTF-8 的文件使用 [NotebookEdit](#notebookedit-tool-behavior)。Edit 也是如此，除非该文件以小端 UTF-16 字节顺序标记开头。当 Claude 尝试时，工具会拒绝更改，并保持文件不变。被拒绝的文件包括以 Windows-1252 或 Shift-JIS 等旧编码保存的非 ASCII 文本、二进制文件，以及包含无效字节序列的 UTF-8 文件。

这些工具之所以拒绝，是因为它们会将整个文件以 UTF-8 保存回去，这会把所有无法解码的字节都替换为替换字符 `U+FFFD`。相反，[Claude 收到的错误](https://code.claude.com/docs/zh-CN/errors.md#file-is-not-valid-utf-8)会告诉它使用一条保留文件编码的 shell 命令来进行更改，或者先询问您是否将文件转换为 UTF-8。

Claude 仍然可以使用 Write 替换此类文件，除非新内容包含 `U+FFFD`，即 Read 在无法解码的字节位置向 Claude 显示的字符。这一防护措施可防止 Claude 将其读取到的乱码文本写回。当 Write 确实替换文件时，它会将新内容保存为 UTF-8，因此文件的原始编码会丢失。

## EndConversation 工具行为

EndConversation 工具结束当前会话。Claude 仅在两种情况下使用它：

* 作为对持续辱骂性输入的最后手段，在尝试重定向对话失败且在之前的消息中发出明确警告之后
* 当你明确要求查看该工具的演示并确认你想要结束会话时

一般性的沮丧、粗言秽语或任务进行不顺利都不符合条件，对有害内容的请求也不符合，Claude 会拒绝这些请求而不是结束会话。Claude Code 遵循与 claude.ai 相同的方法，后者可以[结束少数聊天](https://www.anthropic.com/research/end-subset-conversations)。

Claude 结束交互式会话后，会话被锁定。新提示和大多数命令返回 `Claude ended this conversation. Start a new session (or /clear) to continue.`，只有 `/clear`、`/resume`、`/help`、`/exit` 和 `/feedback` 仍然可以运行。Claude Code 在会话的记录中记录结束，因此恢复已结束的会话会恢复锁定；会话的历史记录不会被删除。

在[非交互模式](https://code.claude.com/docs/zh-CN/headless.md)中使用 `-p` 标志恢复已结束的会话会出错并以代码 1 退出，因此脚本不会将已结束的运行读取为成功。

该工具从不提示权限，[PreToolUse hooks](https://code.claude.com/docs/zh-CN/hooks.md#pretooluse) 也不会为其运行。虽然任何其他工具仍然存在，你也无法阻止它：命名 `EndConversation` 的[拒绝和询问规则](https://code.claude.com/docs/zh-CN/permissions.md#tool-specific-permission-rules)无效，`--disallowedTools` 和 `--tools` 列表都无法将其删除。这个豁免是有意的：该工具除了结束对话外什么都不做，从不读取或修改文件或数据，这种保护措施只有在应用它的会话无法将其关闭时才能有效。当你的拒绝规则删除所有其他工具并且也匹配 `EndConversation` 时，如 `"*"` 所做的那样，Claude Code 也会将其删除，而不是将其作为唯一工具保留，除非允许规则明确命名 `EndConversation`。删除所有其他工具但不匹配 `EndConversation` 的拒绝列表会将其保留在原位。

[子代理](https://code.claude.com/docs/zh-CN/sub-agents.md)永远不会获得该工具。共享主对话工具列表的后台任务会看到它，但在那里调用它不会结束任何内容。

该工具仅在以下所有条件都满足时出现：

* **模型**：会话的模型是 Claude Opus 4.8、Claude Sonnet 5、Claude Fable 5 或这些系列之一的更高版本。
* **界面**：交互式终端会话，包括 IDE 集成终端中的 `claude` 会话，这是[JetBrains 插件](https://code.claude.com/docs/zh-CN/jetbrains.md)运行它的方式。其他界面不包括该工具，例如：
  * 非交互式 `-p` 运行
  * 通过 [Agent SDK](https://code.claude.com/docs/zh-CN/agent-sdk/overview.md) TypeScript 和 Python 包的会话
  * [VS Code 扩展](https://code.claude.com/docs/zh-CN/vs-code.md)面板，它捆绑了自己的 CLI
  * [GitHub Actions](https://code.claude.com/docs/zh-CN/github-actions.md)
  * [云会话](https://code.claude.com/docs/zh-CN/claude-code-on-the-web.md)
* **启动模式**：不是 [`--bare`](https://code.claude.com/docs/zh-CN/headless#start-faster-with-bare-mode) 会话。裸模式仅加载 shell 和文件工具，因此该工具从不在那里注册。
* **提供商**：在 [Amazon Bedrock](https://code.claude.com/docs/zh-CN/amazon-bedrock.md)、[Claude Platform on AWS](https://code.claude.com/docs/zh-CN/claude-platform-on-aws.md)、[Google Cloud's Agent Platform](https://code.claude.com/docs/zh-CN/google-vertex-ai.md) 或 [Microsoft Foundry](https://code.claude.com/docs/zh-CN/microsoft-foundry.md) 上不可用，或在通过[云网关](https://code.claude.com/docs/zh-CN/claude-apps-gateway)登录的会话上不可用。

## Glob 工具行为

Glob 工具通过名称模式查找文件。在 Windows 上，它是默认工具集的一部分。在 macOS、Linux 和 WSL 上，Claude Code 将 Glob 和 [Grep](#grep-tool-behavior) 排除在默认工具集之外，Claude 改为通过 Bash 工具使用 `find` 和 `grep` 进行搜索。在 Claude 的 shell 中，这两个命令运行 `bfs` 和 `ugrep` 的嵌入式版本，搜索以 `Bash` 调用的形式到达您的 hook 和权限规则。

在 macOS、Linux 和 WSL 上，在以下情况下可以恢复 Glob 和 Grep 工具：

* 您在启动会话时在 [`--tools` 或 `--allowedTools`](https://code.claude.com/docs/zh-CN/cli-reference#cli-flags) 中指定 `Glob` 或 `Grep`，或在等效的 [Agent SDK](https://code.claude.com/docs/zh-CN/agent-sdk/overview.md) 选项中指定。使用 `--tools` 时，您会获得列出的工具；在 `--allowedTools` 中指定任一工具会恢复两者。设置文件中的允许规则没有这种效果。
* 权限 [拒绝规则](https://code.claude.com/docs/zh-CN/permissions.md#match-all-uses-of-a-tool)、`--disallowedTools` 标志或 [`--restricted`](https://code.claude.com/docs/zh-CN/cli-reference#cli-flags) 从会话中移除 `Bash`。
* [子代理](https://code.claude.com/docs/zh-CN/sub-agents.md#available-tools) 在其 `tools` 字段中列出 `Glob` 或 `Grep` 并排除 `Bash`。列出的工具仅对该子代理恢复；当它通过 [`--agent`](https://code.claude.com/docs/zh-CN/sub-agents#invoke-subagents-explicitly) 或 `agent` 设置作为主会话 Agent 运行时，则对整个会话恢复。

Glob 支持标准 glob 语法，包括用于递归目录匹配的 `**`：

* `**/*.js` 匹配任何深度的所有 `.js` 文件
* `src/**/*.ts` 匹配 `src/` 下的所有 `.ts` 文件
* `*.{json,yaml}` 匹配当前目录中的 `.json` 和 `.yaml` 文件

结果按修改时间排序，最多限制 100 个文件。如果达到上限，Claude 会在结果中看到截断标志，可以缩小模式范围。

Glob 默认不遵守 `.gitignore`，因此它会找到被 gitignore 的文件和跟踪的文件。这与 [Grep](#grep-tool-behavior) 不同，后者会跳过被 gitignore 的文件。要使 Glob 遵守 `.gitignore`，请在启动 Claude Code 前设置 `CLAUDE_CODE_GLOB_NO_IGNORE=false`。

Claude Code 在检查搜索目录是否存在之前决定 Glob 调用的权限。它仍然对 [工作目录](https://code.claude.com/docs/zh-CN/permissions.md#working-directories) 之外的缺失 `path` 运行读取权限检查，因此针对某个路径的权限提示并不意味着该路径存在。

包含空字节的 `pattern` 或 `path` 值会返回错误，要求 Claude 将其删除。

## Grep 工具行为

Grep 工具在文件内容中搜索模式。[Glob](#glob-tool-behavior) 按名称查找文件，而 Grep 在文件内部查找行。在 macOS、Linux 和 WSL 上，Grep 在与 Glob 相同的条件下默认不可用。有关两个工具何时可用的信息，请参阅 [Glob 工具行为](#glob-tool-behavior)。

Grep 基于 [ripgrep](https://github.com/BurntSushi/ripgrep) 构建，使用 ripgrep 的正则表达式语法，而不是 POSIX grep。包含正则表达式元字符的模式需要转义。例如，在 Go 代码中查找 `interface{}` 需要使用模式 `interface\{\}`。

ripgrep 拒绝的模式、glob 或文件类型会返回一个包含 ripgrep 诊断信息的错误，以便 Claude 可以更正输入并重新搜索。在 v2.1.208 之前，Claude Code 将被拒绝的输入报告为 `No files found`，而不是错误，即使搜索的文本存在于目标文件中。

三种输出模式控制返回的内容：

* `files_with_matches`：仅文件路径，无行内容。这是默认值。
* `content`：匹配的行及其文件和行号。当工具的 `offset` 参数指向某个有匹配项的模式的最后一个匹配项之后时，Grep 返回 `No entries at this offset`，因此 Claude 会扩大或重置偏移量，而不是得出模式不匹配的结论。
* `count`：每个文件的匹配计数，后跟所有匹配文件的总计数。即使工具的 `head_limit` 或 `offset` 参数截断了列出的每个文件的条目，总计仍覆盖每个匹配项。

Claude 可以使用 `glob` 参数（如 `**/*.tsx`）按文件范围限制结果，或使用 `type` 参数（如 `py` 或 `rust`）按语言限制结果。默认情况下，模式在单行内匹配。Claude 可以设置 `multiline: true` 以跨行边界匹配。

Grep 遵守 `.gitignore`，因此被 gitignore 的文件会被跳过。要搜索被 gitignore 的文件，Claude 直接传递其路径。

Claude Code 在检查搜索 `path` 是否存在之前决定 Grep 调用的权限。它仍然对 [工作目录](https://code.claude.com/docs/zh-CN/permissions.md#working-directories) 之外的缺失 `path` 运行读取权限检查，因此路径的权限提示并不意味着该路径存在。

## LSP tool behavior

LSP tool 从运行的语言服务器为 Claude 提供代码智能。在每次文件编辑后，它会自动报告类型错误和警告，以便 Claude 可以在没有单独构建步骤的情况下修复问题。Claude 也可以直接调用它来导航代码：

* 跳转到符号的定义
* 查找对符号的所有引用
* 获取位置处的类型信息
* 列出文件中的符号
* 在工作区中按名称搜索符号
* 查找接口的实现
* 追踪调用层次结构

Claude Code 会保持该工具处于非活动状态，直到您为您的语言安装 [code intelligence plugin](https://code.claude.com/docs/zh-CN/plugins/code-intelligence.md)。在[云会话](https://code.claude.com/docs/zh-CN/claude-code-on-the-web.md)中，Claude Code 不会启动 plugin 语言服务器，因此 LSP tool 在那里保持非活动状态。Claude Code 从 plugin 获取语言服务器的配置，您需要自己安装服务器二进制文件。

Claude Code 对于无法启动其语言服务器的文件上的每个 LSP 调用都会返回错误结果。

## Monitor 工具

Monitor 工具让 Claude 在后台监视某些内容，并在其发生变化时做出反应，而无需暂停对话。您可以要求 Claude：

* 跟踪日志文件并在错误出现时标记
* 轮询 PR 或 CI 作业并在其状态更改时报告
* 监视目录中的文件更改
* 跟踪您指向的任何长时间运行脚本的输出
* 连接到 WebSocket 源并在每条消息到达时报告

对于大多数监视，Claude 会编写一个小脚本，在后台运行它，并在每行输出到达时接收。对于已经推送事件的服务器，Claude 可以打开 [WebSocket](#websocket-source) 而不是运行脚本。

您可以在同一会话中继续工作，Claude 在事件到达时进行插入。

每个 Claude 启动的监视都有一个截止时间：默认为 5 分钟，最多 30 分钟，在 [非交互式](https://code.claude.com/docs/zh-CN/headless.md) 运行中使用单个提示和 `-p` 时最多 10 分钟。

在截止时间时，监视结束。Claude 会收到一个通知，因此如果仍然需要，它可以重新启动监视。

通过要求 Claude 取消监视或结束会话来停止监视。当您停止启动了监视的 [子代理](https://code.claude.com/docs/zh-CN/sub-agents.md)（例如来自 `/tasks`）时，这些监视会随之停止。

当 Monitor 运行命令时，它使用与 Bash 相同的 [权限规则](https://code.claude.com/docs/zh-CN/permissions.md#tool-specific-permission-rules)，因此您为 Bash 设置的 `allow` 和 `deny` 模式也适用于此处。当 [自动模式](https://code.claude.com/docs/zh-CN/permission-modes.md#eliminate-prompts-with-auto-mode) 处于活动状态时，Claude Code 会搁置命名 `Monitor` 本身的允许规则，以及它删除的其他 [广泛允许规则](https://code.claude.com/docs/zh-CN/permission-modes.md#how-the-classifier-evaluates-actions)，因此分类器以与审查 Bash 命令相同的方式审查 Monitor 命令。

[WebSocket 源](#websocket-source) 有其自己的批准提示，分类器也在自动模式下决定。

在以下情况下，Monitor 需要最低版本或 Git Bash：

* **最低版本**：在 Amazon Bedrock、Claude Platform on AWS、Google Cloud 的 Agent Platform 或 Microsoft Foundry 上，或者设置了 `DISABLE_TELEMETRY` 或 `CLAUDE_CODE_DISABLE_NONESSENTIAL_TRAFFIC` 时，Monitor 需要 Claude Code v2.1.310 或更高版本。此要求在 [功能标志获取](https://code.claude.com/docs/zh-CN/env-vars.md#features-that-need-feature-flag-fetching) 关闭时适用
* **Windows**：仅当安装了 [Git Bash](https://code.claude.com/docs/zh-CN/setup.md#set-up-on-windows) 时该工具才可用

插件可以声明在插件处于活动状态时自动启动的监视，而不是要求 Claude 启动它们。请参阅 [插件监视器](https://code.claude.com/docs/zh-CN/plugins/components.md#monitors)。

### WebSocket 源

当服务器已经通过 WebSocket 推送事件时，Claude 可以直接连接到它，而不是编写轮询脚本。每种套接字活动要么成为一个事件，要么结束监视：

* **文本消息**：每条消息都成为一个事件，即使消息跨越多行。
* **二进制消息**：不通过。Claude 接收一个占位符行，例如 `[binary frame, 512 bytes]`。
* **大于 1 MiB 的消息**：监视结束，因此请订阅存在的过滤源。
* **套接字关闭**：监视结束，Claude 接收关闭代码。

WebSocket 监视采用 `ws` 输入代替 `command`，单个 Monitor 调用不能将两者结合。`ws` 输入有两个字段：

| 字段 | 必需 | 描述 |
| :- | :- | :- |
| `url` | 是 | 要连接的端点。必须是 `ws://` 或 `wss://` URL，不包含嵌入的凭据或空格，仅使用 ASCII 字符 |
| `protocols` | 否 | 在握手期间提供的 WebSocket 子协议名称。每个条目必须是有效的子协议令牌，列表不能包含重复项 |

`timeout_ms` 截止时间也适用于 WebSocket 监视：监视在截止时间结束，`TaskStop` 会提前取消它。

打开 WebSocket 会提示批准；在 [自动模式](https://code.claude.com/docs/zh-CN/permission-modes.md#eliminate-prompts-with-auto-mode) 中，分类器会决定。该提示不提供跳过同一主机的未来提示的选项。

Claude Code 拒绝指向私有、链接本地或云元数据地址的 URL，包括解析为其中之一的主机名。它还拒绝 `sandbox.network.deniedDomains` 中的主机，以及当在托管设置中设置了 [`allowManagedDomainsOnly`](https://code.claude.com/docs/zh-CN/settings-reference#sandbox-network-allowmanageddomainsonly) 时，任何在托管允许列表之外的主机。

## NotebookEdit 工具行为

NotebookEdit 一次修改一个 Jupyter notebook 单元格，按其 `cell_id` 定位单元格。它不像 [Edit](#edit-tool-behavior) 在纯文本文件上那样在整个 notebook 中执行字符串替换。

三种编辑模式控制目标单元格发生的情况：

* `replace`：覆盖单元格的源。这是默认值。
* `insert`：在目标后添加新单元格。没有 `cell_id` 时，新单元格位于 notebook 的开始。需要 `cell_type` 设置为 `code` 或 `markdown`。
* `delete`：删除目标单元格。

对于无法按 UTF-8 解码的 notebook 文件，NotebookEdit 会依照[与 Edit 相同的规则](#non-utf-8-files)拒绝处理，且不写入任何内容。

权限规则使用 `Edit(...)` 路径格式。像 `Edit(notebooks/**)` 这样的规则涵盖该目录中的 NotebookEdit 调用。

## PowerShell 工具

PowerShell 工具让 Claude 能够原生运行 PowerShell 命令。在 Windows 上，这意味着命令在 PowerShell 中运行，而不是通过 Git Bash 路由。该工具的可用性取决于您的平台：

* **Windows 不带 Git Bash**：该工具自动启用。
* **Windows 安装了 Git Bash**：对于 claude.ai 和 Console 账户，该工具默认启用；在 Amazon Bedrock、Google Cloud 的 Agent Platform 和 Microsoft Foundry 会话中，设置 `CLAUDE_CODE_USE_POWERSHELL_TOOL=1` 以启用它，或设置 `0` 以关闭它。
* **Linux、macOS 和 WSL**：该工具是可选的。

您的 [PreToolUse hooks](https://code.claude.com/docs/zh-CN/hooks.md#powershell) 在 `tool_input.command` 中接收该工具的命令字符串，字段与 Bash 工具相同。

在检查 shell 命令的 hooks 中匹配 `Bash|PowerShell`；[PowerShell hook 输入部分](https://code.claude.com/docs/zh-CN/hooks.md#powershell)解释了为什么仅匹配 `Bash` 是不够的。

### 启用 PowerShell 工具

在您的环境或 `settings.json` 中设置 `CLAUDE_CODE_USE_POWERSHELL_TOOL=1`：

```json theme={null}
{
  "env": {
    "CLAUDE_CODE_USE_POWERSHELL_TOOL": "1"
  }
}
```

在 Windows 上，将变量设置为 `0` 以关闭该工具。在 Linux、macOS 和 WSL 上，该工具需要 PowerShell 7 或更高版本：安装 `pwsh` 并确保它在您的 `PATH` 中。

在 Windows 上，Claude Code 自动检测 PowerShell 7+ 的 `pwsh.exe`，如果没有则回退到 PowerShell 5.1 的 `powershell.exe`。启用该工具后，Claude 将 PowerShell 视为主 shell。当安装了 Git Bash 时，Bash 工具仍可用于 POSIX 脚本。

Claude Code 使用 `-ExecutionPolicy Bypass` 仅在进程范围内生成 PowerShell，因此 `.ps1` 脚本和模块导入可以在默认 Windows 安装上工作，无需更改机器的策略。进程范围的绕过不会覆盖组策略 `MachinePolicy` 或 `UserPolicy`，因此企业策略仍然适用。要改为遵守机器的有效执行策略，请设置 `CLAUDE_CODE_POWERSHELL_RESPECT_EXECUTION_POLICY=1`。

### Bash 拒绝规则也会关闭 PowerShell 工具

在安装了 Git Bash 的 Windows 上，拒绝 Bash 也会在该会话中关闭 PowerShell 工具。这既适用于限定范围的规则（例如 `Bash(git push *)`），也适用于单独的 `Bash`，并且适用于来自您某个设置文件或 `--disallowedTools` 的规则。Claude Code 这样做是因为 `Bash` 规则不会限制 PowerShell 工具，后者有[自己的权限规则](https://code.claude.com/docs/zh-CN/permissions.md#powershell)。如果 PowerShell 保持开启，Claude 可能会在其中运行您的规则在 Bash 中所拒绝的内容。

要在存在 Bash 拒绝规则的同时保持 PowerShell 工具开启，请执行以下任一操作：

* 在您的环境或设置文件的 `env` 块中设置 `CLAUDE_CODE_USE_POWERSHELL_TOOL=1`，如[启用 PowerShell 工具](#enable-the-powershell-tool)中所示。
* 向设置文件添加限定范围的 [`PowerShell` 权限规则](https://code.claude.com/docs/zh-CN/permissions#powershell)，例如 `PowerShell(git push *)` 拒绝规则。

如果没有其中任何一项，限定范围的 Bash 拒绝规则会保留 Bash 工具可用，而 Claude Code 会在不发出警告的情况下关闭 PowerShell。移除整个 Bash 工具的规则会使 Claude 在该会话中没有任何 shell 工具。

### 设置、hooks 和 skills 中的 shell 选择

三个额外的设置控制 PowerShell 的使用位置：

* [`settings.json`](https://code.claude.com/docs/zh-CN/settings-reference#all-settings) 中的 `"defaultShell": "powershell"`：通过 PowerShell 路由交互式 `!` 命令。需要启用 PowerShell 工具。
* 单个 [command hooks](https://code.claude.com/docs/zh-CN/hooks.md#command-hook-fields) 上的 `"shell": "powershell"`：在 PowerShell 中运行该 hook。Hooks 直接生成 PowerShell，因此无论 `CLAUDE_CODE_USE_POWERSHELL_TOOL` 如何，这都有效。
* [skill frontmatter](https://code.claude.com/docs/zh-CN/skills.md#frontmatter-reference) 中的 `shell: powershell`：在 PowerShell 中运行 `` !`command` `` 块。需要启用 PowerShell 工具。

PowerShell 命令遵循与 Bash 命令[相同的主会话工作目录重置行为](#what-persists-between-commands)，包括 `CLAUDE_BASH_MAINTAIN_PROJECT_WORKING_DIR` 环境变量。

PowerShell 命令还会接收 hook 通过 `CLAUDE_ENV_FILE` 持久化的变量，具体条件请参阅 [PowerShell 命令中的持久化变量](https://code.claude.com/docs/zh-CN/hooks.md#persisted-variables-in-powershell-commands)。需要 Claude Code v2.1.296 或更高版本。

### PowerShell 中搜索和 `git diff` 的退出码 1

如果 Claude 在 PowerShell 工具中运行的 `grep` 或 `rg` 搜索没有匹配到任何内容并显示 `Error: Exit code 1`，则该搜索并未失败。您无需执行任何操作。当搜索是较长命令的一部分时，可能会出现此错误。例如，当 `rg` 以退出码 1 退出时，`rg foo | Out-Null` 会显示该错误，而 `rg foo` 不会。

PowerShell 工具可以将来自 `grep`、`rg`、`egrep`、`fgrep`、`findstr`、`git grep` 和 `git diff` 的退出码 1 视为正常结果。这些搜索使用退出码 1 表示没有匹配。对于 `git diff`，退出码 1 表示文件存在差异。在带有 `shell: powershell` 的 skill 中，如果 Claude 运行某条命令时显示该错误，则该命令[被视为失败](https://code.claude.com/docs/zh-CN/skills.md#when-an-injected-command-fails)。

### Windows 编码和退出代码

* 使用 `>` 和 `>>` 的重定向在 PowerShell 5.1 上写入 UTF-8 文件
* Claude Code 将管道传输到本机命令标准输入的文本编码为 UTF-8
* Claude Code 捕获没有 ANSI 转义序列的错误输出
* 其子进程等待标准输入的命令接收文件结束而不是挂起
* 如果 `where.exe` 未找到匹配项，或者 `fc.exe` 或 `diff.exe` 发现文件不同，结果可能会显示 `Error: Exit code 1`，但命令并未失败。您无需执行任何操作。
* 如果 `robocopy` 的退出码 1 到 7 显示为错误，则复制并未失败，您无需执行任何操作。`robocopy` 以退出码 0 到 7 报告结果（例如已复制的文件），失败时以 8 或更高的退出码退出。

### 预览限制

PowerShell 工具在预览期间有以下已知限制：

* PowerShell 配置文件未加载
* 在 Windows 上，不支持沙箱化

## Read 工具行为

Read 工具接收文件路径并返回带有行号的文件内容。Claude 被指示始终传递绝对路径。

读取空文件会返回一个通知，说明文件存在但内容为空，而 `offset` 超过最后一行会返回一个通知，给出文件的行数。

Read 处理多种文件类型，不仅仅是纯文本：

* **图像**：PNG、JPEG、GIF 和 WebP 文件作为 Claude 可以看到的视觉内容返回，而不是原始字节。Claude Code 在发送大型图像之前会调整大小并重新压缩，以适应模型的图像大小限制，因此 Claude 可能会看到大型屏幕截图的缩小版本。

  如果 Claude 在大型图像中遗漏了细微的像素级细节，请要求它裁剪您关注的区域，例如通过 Bash 使用 ImageMagick，然后读取裁剪后的图像。

  Read 会拒绝读取大于 128 MB 的图像文件，并返回一个错误，告诉 Claude 调整图像大小或导出一个较小的副本，然后读取该副本。如果 Claude 在遇到该错误后停止而没有这样做，请参阅 [File is too large to read as an image](https://code.claude.com/docs/zh-CN/errors.md#file-is-too-large-to-read-as-an-image)。
* **PDF**：Claude 完整读取短 `.pdf` 文件。对于超过 10 页的 PDF，它使用 `pages` 参数按范围读取，例如 `"1-5"`，一次最多 20 页。页面范围读取使用 poppler-utils 中的 `pdftoppm` 呈现页面，因此在 macOS 上使用 `brew install poppler` 安装，在 Debian 和 Ubuntu 上使用 `apt-get install poppler-utils` 安装。在 Windows 和其他平台上，安装一个将 `pdftoppm` 放在 `PATH` 上的 poppler 构建。没有它，页面范围读取会失败并显示 `pdftoppm is not installed`。
* **Jupyter 笔记本**：`.ipynb` 文件返回所有单元格及其输出，包括代码、markdown 和可视化。如果笔记本的单元格总量超过 256 KB，或超过 [token 限制](#large-files)，则会返回错误。Claude Code 拒绝读取超过 100 MB 的笔记本文件；错误会告诉 Claude 如何改为读取笔记本的一部分，例如使用 shell 命令读取单元格的一个切片。

Read 仅读取文件，不读取目录。Claude 使用 shell 命令（如 `ls`）列出目录内容。

Read 会拒绝读取其视为二进制扩展名的文件，例如 `.xlsx`、`.parquet`、`.heic` 或 `.zip`。被拒绝后 Claude 会继续工作：如果有适用于该文件类型的 skill，错误会建议使用该 skill，否则会建议使用能够读取该格式的 shell 命令或脚本。如果您 [`@` 提及](https://code.claude.com/docs/zh-CN/common-workflows#reference-files-and-directories)一个被 Read 拒绝的文件，Claude 会收到该文件的路径，但不会收到其内容。

### 大型文件

Claude 可以读取大于单次 Read 调用所能返回内容的文本文件。默认情况下，单次调用最多返回 25,000 个 token，或您在 [`CLAUDE_CODE_FILE_READ_MAX_OUTPUT_TOKENS`](https://code.claude.com/docs/zh-CN/env-vars) 中设置的值，并且会拒绝读取超过 256 KB 的整个文件，因此 Claude 会使用 `offset` 和 `limit` 分页读取较大的文件。在 Claude Code v2.1.296 或更高版本中，Claude 可以在需要时（例如因为您要求读取整个文件）通过设置 `allow_large: true`，在一次调用中读取整个文件或较长的行范围。此类读取的大小依据会话[上下文窗口](https://code.claude.com/docs/zh-CN/context-window.md)中的剩余空间来确定，而不是依据默认限制。图像、PDF 和笔记本仍保留各自的限制。

当读取超过默认限制时，Claude 收到的内容：

* **整个文件超过 token 限制**：文件的第一页，并附带 `PARTIAL view` 通知，说明它接收了多少文件内容以及如何使用 `offset` 和 `limit` 读取更多内容
* **整个文件超过 256 KB，或使用 `offset` 或 `limit` 的读取超过 token 限制**：一个错误，告诉它使用 `offset` 和 `limit` 读取一部分内容，或改用 [Grep](#grep-tool-behavior) 搜索特定内容

## SendFeedback 工具行为

Claude 起草的反馈是 Claude 为您撰写的关于 Claude Code 的反馈报告。它需要 Claude Code v2.1.238 或更高版本。Claude Code 将每份草稿保存在您的机器上的 `~/.claude/feedback/drafts/` 下，在您发送之前，任何内容都不会到达 Anthropic。Claude 在以下情况下使用 SendFeedback 工具起草一份：

* 工具或命令持续失败
* 它无法帮助您要求的事情
* 您指出它犯的错误，或它注意到一个错误
* 您要求它提交反馈

### 当 Claude 起草时您看到的内容

Claude 将草稿加入队列后，您会在提示上方看到一张卡片，显示草稿的标题。按 `1` 查看草稿，按 `2` 两次按原样发送，或按 `0` 关闭它。被关闭的草稿保留在您的队列中。关闭卡片后，Claude Code 会询问是否关闭 Claude 起草的反馈。一旦您拒绝两次，它就会停止询问。

默认情况下，您在一个会话中最多看到三张卡片；Anthropic 可以从服务器调整该限制，无需发布。超过限制后，以及每当您将 [`feedbackDrafts`](https://code.claude.com/docs/zh-CN/settings-reference#feedbackdrafts) 设置为 `quiet` 时，您只会在提示页脚中看到排队草稿的计数。

### 查看和编辑草稿

运行不带参数的 `/feedback` 来打开您的队列。它列出来自所有会话的每份排队草稿，包括您关闭或从未看到其卡片的草稿。选择一份草稿来打开它进行查看，您可以：

* 编辑标题、区域和详情
* 将 **Send transcript** 设置为 `yes` 或 `no`。当 Claude 将草稿加入队列的会话中的记录仍然可用时，它以 `yes` 开始，这会将该对话发送给 Anthropic；`no` 仅发送报告
* 发送草稿、丢弃它或将其留在队列中以供稍后使用

要自己写一份报告，请按 `w` 打开标准反馈对话框。`/feedback` 后跟文本和 `/bug` 直接打开该对话框。

### 发送草稿

当您发送草稿时，Claude Code 以与 `/feedback` 报告相同的方式提交它，具有相同的 [保留期](https://code.claude.com/docs/zh-CN/data-usage.md#feedback-using-the-%2Ffeedback-command)，并从您的机器中删除草稿。当您从卡片发送时，它显示 `✓ Sent`；当您从队列发送时，它关闭并显示收据 ID。

报告包含：

* 您的标题、区域和详情
* 环境信息，例如您的 Claude Code 版本、操作系统和模型
* 最近 API 请求的 ID
* 对话记录，当您在查看屏幕中将 **Send transcript** 保留为 `yes` 时。从卡片发送永远不包括记录

Claude Code 在本地草稿中保留您的工作目录，以便它可以找到记录，并且不发送目录。

在 [零数据保留的组织](https://code.claude.com/docs/zh-CN/zero-data-retention.md#features-disabled-under-zdr) 中，以及在已应用 [HIPAA 配置](https://code.claude.com/docs/zh-CN/hipaa-setup.md) 的组织中，Claude Code 会在您的会话中省略 SendFeedback 工具。如果零数据保留组织中的会话仍然提供该工具，草稿保留在您的机器上，发送失败并显示 `Feedback collection is not available for organizations with custom data retention policies.`

### 丢弃或保留草稿

当您丢弃草稿时，Claude Code 从您的机器中删除它。您留在队列中的草稿在 30 天后过期，或在 [`cleanupPeriodDays`](https://code.claude.com/docs/zh-CN/settings-reference#cleanupperioddays) 更短时过期。队列在所有会话中最多保留 10 份草稿，当 Claude 将第十一份加入队列时，Claude Code 删除最旧的。当您运行 `/exit` 且会话中的草稿仍在队列中时，Claude Code 会询问您是否在退出前查看或丢弃它们。

### 关闭 Claude 起草的反馈

在 `/config` 中将 **Claude-drafted feedback** 设置为 `off`，这会写入 [`feedbackDrafts`](https://code.claude.com/docs/zh-CN/settings-reference#feedbackdrafts) 设置，或为一个会话设置 [`CLAUDE_CODE_SEND_FEEDBACK=0`](https://code.claude.com/docs/zh-CN/env-vars)。使用任一方式，Claude 都无法将草稿加入队列。要在没有卡片的情况下继续起草，请改为将 `feedbackDrafts` 设置为 `quiet`。管理员可以在 [托管设置](https://code.claude.com/docs/zh-CN/managed-settings.md) 中设置 `feedbackDrafts`，这优先于您自己的设置。

### 没有 Claude 起草反馈的会话

Claude Code 在使用 Claude API 而不是云提供商的您自己机器上的交互式终端会话中包含该工具。它在以下情况下省略该工具：

* 非交互式 `-p` 运行和 [Agent SDK](https://code.claude.com/docs/zh-CN/agent-sdk/overview.md) 会话，这些没有屏幕来查看队列
* [云会话](https://code.claude.com/docs/zh-CN/claude-code-on-the-web.md)，无法在您的机器上写入队列
* [Amazon Bedrock](https://code.claude.com/docs/zh-CN/amazon-bedrock.md)、[Claude Platform on AWS](https://code.claude.com/docs/zh-CN/claude-platform-on-aws.md)、[Google Cloud's Agent Platform](https://code.claude.com/docs/zh-CN/google-vertex-ai.md) 或 [Microsoft Foundry](https://code.claude.com/docs/zh-CN/microsoft-foundry.md) 上的会话
* 您设置 [`CLAUDE_CODE_SEND_FEEDBACK=0`](https://code.claude.com/docs/zh-CN/env-vars) 或 [`DISABLE_FEEDBACK_COMMAND=1`](https://code.claude.com/docs/zh-CN/env-vars) 的会话，将 `CLAUDE_CODE_DISABLE_NONESSENTIAL_TRAFFIC` 设置为任何非空值，或关闭 [功能标志获取](https://code.claude.com/docs/zh-CN/env-vars.md#features-that-need-feature-flag-fetching)
* 已关闭产品反馈的组织、[零数据保留的组织](https://code.claude.com/docs/zh-CN/zero-data-retention.md#features-disabled-under-zdr)，以及已应用 [HIPAA 配置](https://code.claude.com/docs/zh-CN/hipaa-setup.md) 的组织

## Task 工具可用性

Task 跟踪工具 `TaskCreate`、`TaskGet`、`TaskUpdate`、`TaskList` 和 `TodoWrite` 默认仅在 Opus 4.7、Sonnet 4.6、Haiku 4.5 及更早的模型上可用。只要这些工具可用，您就会获得四个 Task 工具，或者当您设置 [`CLAUDE_CODE_ENABLE_TASKS=0`](https://code.claude.com/docs/zh-CN/env-vars) 时改为获得 `TodoWrite`。

在所有其他模型上，Claude Code 会排除这些工具，除非您选择加入。这同样适用于 Claude Code 无法识别的模型 ID，例如通过 [LLM 网关](https://code.claude.com/docs/zh-CN/llm-gateway)提供的自定义模型名称。在较新的模型上，Claude 可以在没有书面清单的情况下跟踪多步骤工作，而这些工具的定义和提醒会占用上下文。没有这些工具，Claude 在工作时不会向[任务列表](https://code.claude.com/docs/zh-CN/interactive-mode.md#task-list)添加任何内容。

如果您想在默认情况下没有这些工具的模型上使用它们，请执行以下操作之一：

* 在启动 Claude Code 之前导出 [`CLAUDE_CODE_ENABLE_TODO_TOOLS=1`](https://code.claude.com/docs/zh-CN/env-vars)，例如 `CLAUDE_CODE_ENABLE_TODO_TOOLS=1 claude`。Claude Code 随后会在每个模型和每个提供商上提供相同的工具
* 在 [`--allowedTools`](https://code.claude.com/docs/zh-CN/cli-reference#cli-flags) 中命名其中一个工具，例如 `claude --allowedTools TaskCreate`
* 在 [`--tools`](https://code.claude.com/docs/zh-CN/cli-reference#cli-flags) 中列出这些工具，这会将会话的内置工具限制为它命名的工具。将您想要的工具与您使用的其他内置工具一起包括
* 在 Agent SDK 中，[`allowedTools` 和 `tools` 选项](https://code.claude.com/docs/zh-CN/agent-sdk/todo-tracking#model-availability)的工作方式与这两个标志相同

在[后台会话](https://code.claude.com/docs/zh-CN/agent-view.md)和[云会话](https://code.claude.com/docs/zh-CN/claude-code-on-the-web.md)中，Claude Code 在每个模型上提供相同的工具，无论是否列出。

此处描述的默认集合适用于 Claude Code v2.1.268 及更高版本。

### 子代理和 agent team 中的 Task 跟踪工具

子代理仅在您的会话拥有 Task 跟踪工具时才拥有这些工具，无论子代理运行什么模型。进程内 [agent team](https://code.claude.com/docs/zh-CN/agent-teams.md) 队友在您的会话拥有 Task 跟踪工具时也拥有这些工具。[分割窗格](https://code.claude.com/docs/zh-CN/agent-teams.md#choose-a-display-mode)队友是一个单独的 Claude Code 进程，它[继承](https://code.claude.com/docs/zh-CN/agent-teams.md#permissions)您的 `--tools` 列表和拒绝规则，但不继承 `--allowedTools`，因此请检查您的设置：

* **没有 `--tools` 列表或拒绝规则，且未为队友设置 `CLAUDE_CODE_ENABLE_TODO_TOOLS`**：队友拥有其自身模型默认具备的 Task 跟踪工具。
* **在您的 shell 中导出了 `CLAUDE_CODE_ENABLE_TODO_TOOLS`**：Claude Code 会向队友传递一组固定的环境变量，其中不包括您导出的那个变量。若要同时为队友设置该变量，请在 shell 中运行 `claude --settings '{"env":{"CLAUDE_CODE_ENABLE_TODO_TOOLS":"1"}}'` 来启动会话。队友也会获得您的 [`--settings`](https://code.claude.com/docs/zh-CN/settings#change-a-setting-for-one-session) 值。
* **`--tools` 中命名了 Task 跟踪工具**：队友在任何模型上都拥有这些工具。
* **某个 Task 跟踪工具未包含在 `--tools` 中或在拒绝规则中被命名**：队友不具备该工具。

没有 Task 工具时，Agent 通过消息而不是[共享任务列表](https://code.claude.com/docs/zh-CN/agent-teams.md#assign-and-claim-tasks)与其团队协调。

## WebFetch 工具行为

WebFetch 接收一个 URL 和一个描述要提取内容的提示词。它会获取页面，并在服务器返回 HTML 时将响应转换为 Markdown。对于大多数获取操作，它随后会在单独的模型调用中针对内容运行该提示词，Claude 接收到的是该调用的结果，而不是原始页面。转换步骤不可配置。

这使得 WebFetch 在设计上就是有损的。提取提示词决定了哪些内容会到达 Claude，因此如果结果表明页面没有提到某些内容，可能仅仅意味着提示词没有询问这一点。可以让 Claude 使用更具体的提示词重新获取，或通过 Bash 使用 `curl` 获取未经处理的页面。

以下几种行为会影响 Claude 接收到的响应：

* WebFetch 会在发出请求之前拒绝 `localhost` 以及任何不含点的主机名，例如单纯的内网名称。它返回的[错误](https://code.claude.com/docs/zh-CN/errors.md#webfetch-cannot-fetch-localhost)会告诉 Claude 改为通过 Bash 使用 `curl` 访问本地服务器。
* HTTP URL 会自动升级为 HTTPS。
* WebFetch 每次调用最多读取页面内容的 100,000 个字符。在 Claude Code v2.1.290 或更高版本中，对于较长的页面，结果会告诉 Claude 有多少内容未被读取，以便 Claude 获取下一部分。
* WebFetch 默认将每个响应缓存 15 分钟，因此重复获取同一 URL 会很快返回。在 Claude Code v2.1.233 或更高版本中，可设置 [`CLAUDE_CODE_WEBFETCH_CACHE_TTL_MS`](https://code.claude.com/docs/zh-CN/env-vars#variables) 来更改 WebFetch 保留每个响应的时长。
* 如果页面在五分钟内（包括 WebFetch 跟随的任何重定向）未完成下载，则会因截止时间错误而失败。在 Claude Code v2.1.268 或更高版本中，可设置 [`CLAUDE_CODE_WEBFETCH_DEADLINE_MS`](https://code.claude.com/docs/zh-CN/env-vars#variables) 来更改该限制，或将其设为 `0` 以移除限制。
* 当 URL 重定向到不同的主机时，WebFetch 不会跟随重定向，而是返回一个文本结果，其中列出原始 URL 和重定向目标。Claude 随后会通过第二次 WebFetch 调用获取新的 URL。
* 当提取步骤遇到 API 过载时，Claude Code 会以退避方式重试；如果获取仍然失败，则返回错误结果。

在 Manual 和 `acceptEdits` [权限模式](https://code.claude.com/docs/zh-CN/permission-modes.md)下，WebFetch 会在获取之前进行提示，但您的[权限规则](https://code.claude.com/docs/zh-CN/permissions.md#manage-permissions)已允许或拒绝的域名，以及一组内置的预先批准的文档域名除外，这些域名无需提示即可获取。无论您的规则允许什么，获取操作都会先通过 [WebFetch 域名安全检查](https://code.claude.com/docs/zh-CN/data-usage.md#webfetch-domain-safety-check)；该部分介绍了检查会发送的内容以及跳过检查的设置。提示提供三个选项：

* **Yes**：仅批准本次获取。下一次 WebFetch 调用会再次提示，即使是同一域名。
* **Yes, and don't ask again for `<domain>`**：批准本次获取，并将该域名的 `WebFetch(domain:...)` 允许规则保存到该仓库的 `.claude/settings.local.json` 中。请参阅[已保存的批准如何持久保留](https://code.claude.com/docs/zh-CN/permissions.md#permission-system)。当您的组织设置了 [`allowManagedPermissionRulesOnly`](https://code.claude.com/docs/zh-CN/permissions#managed-only-settings) 时，Claude Code 会隐藏此选项。
* **No, and tell Claude what to do differently**：拒绝本次获取。

要预先允许某个域名而无需提示，请添加类似 `WebFetch(domain:example.com)` 的允许规则；`WebFetch(domain:*)` 允许所有域名。`auto` 和 `bypassPermissions` [权限模式](https://code.claude.com/docs/zh-CN/permissions.md#permission-modes)会跳过提示，但显式 `ask` 规则匹配的域名除外。

`deny`、`ask` 或 `allow` 中的显式 `WebFetch(domain:...)` 规则优先于预先批准的集合，因此您可以阻止某个预先批准的域名，或要求对其进行提示。

当 URL 是 claude.ai [Artifact](https://code.claude.com/docs/zh-CN/artifacts.md) 链接时，Claude Code 还可能请求批准以读取该 Artifact 本身。有关会请求批准的情况，请参阅[读取与您共享的 Artifact](https://code.claude.com/docs/zh-CN/artifacts.md#read-an-artifact-shared-with-you)。

WebFetch 会设置以 `Claude-User` 开头的 `User-Agent` 标头，以及优先选择 Markdown 而非 HTML 的 `Accept` 标头，以便支持内容协商的服务器可以直接返回 Markdown。

沙箱中的命令不会继承 WebFetch 内置的预先批准文档域名集合。要让沙箱中的命令无需提示即可访问某个域名，请将该域名添加到 [`allowedDomains`](https://code.claude.com/docs/zh-CN/settings-reference#sandbox-network-alloweddomains)，或使用 `WebFetch(domain:...)` 规则允许它，[沙箱也会遵循](https://code.claude.com/docs/zh-CN/sandboxing.md#network-isolation)该规则。反过来，WebFetch 从不读取沙箱允许列表，因此将域名添加到沙箱或组织网络允许列表并不会阻止 WebFetch 对其进行提示。

### WebFetch 可用性

在 Claude Code v2.1.285 或更高版本中，将 [`CLAUDE_CODE_DISABLE_WEB_FETCH`](https://code.claude.com/docs/zh-CN/env-vars#variables) 设置为 `1` 即可关闭 WebFetch。

如果您使用 Team 或 Enterprise claude.ai 账户登录，并且未通过 [LLM 网关](https://code.claude.com/docs/zh-CN/llm-gateway)连接，WebFetch 还取决于您组织的策略，Claude Code 会在会话启动时从 `api.anthropic.com` 请求该策略。对于 Claude Code 无法确定其套餐的会话，例如在其他应用提供的 claude.ai 令牌下运行的会话，情况也是如此。

如果会话中缺少 WebFetch，请在会话中运行 `/status`。如果其 `Organization policy` 行报告策略未加载，并在等待该策略的功能中列出了 web fetch，则表示 Claude Code 在确认您的组织允许 WebFetch 之前会暂不提供该工具。在会话之外，`claude doctor` 会自行发出请求并打印相同的行。一旦允许 WebFetch 的策略加载完成，该工具就会恢复可用，无需重启。

## WebSearch 工具行为

WebSearch 针对 Anthropic 的[网络搜索](https://platform.claude.com/docs/en/agents-and-tools/tool-use/web-search-tool)后端运行查询，并返回结果标题和 URL。它不会获取结果页面。要读取 Claude 在搜索结果中找到的页面，它会跟进使用 [WebFetch](#webfetch-tool-behavior)。

该工具每次调用最多可以发出八个后端搜索，在返回结果之前在内部优化搜索。Claude 可以使用 `allowed_domains` 限制结果范围以仅包含某些主机，或使用 `blocked_domains` 排除它们。这两个列表不能在单次调用中组合。

当搜索请求命中过载的 API 时，Claude Code 会使用退避策略重试；仍然失败的调用会返回错误结果。

WebSearch 权限规则不需要指定符。`allow` 或 `deny` 中的单独 `WebSearch` 条目是唯一的形式。

搜索后端不可配置。要使用不同的提供商进行搜索，请添加一个[MCP 服务器](https://code.claude.com/docs/zh-CN/mcp.md)来公开搜索工具。

> [!NOTE]
>
> WebSearch 在 Claude API、[AWS 上的 Claude Platform](https://code.claude.com/docs/zh-CN/claude-platform-on-aws.md) 和 Microsoft Foundry 上可用。在 Google Cloud 的 Agent Platform 上，它适用于 Opus 4、Sonnet 4、Haiku 4.5 及更高版本的模型。Amazon Bedrock 不公开服务器端网络搜索工具。

### 会话搜索限制

一个交互式终端会话最多可以进行 200 次 WebSearch 调用。来自主对话和[子代理](https://code.claude.com/docs/zh-CN/sub-agents.md)的搜索（例如并行研究扇出）计入同一限制。

当会话达到限制时，搜索在对话中显示为未执行任何操作的调用。Claude 会收到一个通知，告诉它继续使用已经收集的信息，在回答中说明这一点，并且如果需要更多搜索，则要求您提高限制。

要获得更多搜索次数，请提高上限、等待限制恢复，或开始新的对话：

* **提高上限**：将 [`CLAUDE_CODE_MAX_WEB_SEARCHES_PER_SESSION`](https://code.claude.com/docs/zh-CN/env-vars#variables) 环境变量设置为正整数，例如 `500`。上限可以提高但不能关闭。
* **等待恢复**：在 Claude Code v2.1.290 或更高版本中，交互式终端会话的限制大约每小时恢复 100 次调用。要更改该速率，请将 [`CLAUDE_CODE_WEB_SEARCH_REFILLS_PER_HOUR`](https://code.claude.com/docs/zh-CN/env-vars#variables) 设置为每小时的调用次数，例如 `50`。
* **开始新的对话**：在 Claude Code 提示符处运行 [`/clear`](https://code.claude.com/docs/zh-CN/commands#all-commands) 也会重置计数。如果仍然可以生成子代理的工作（例如正在运行的工作流）在清除后继续存在，计数会改为继续累计。

在使用 `-p` 的[非交互模式](https://code.claude.com/docs/zh-CN/headless.md)下以及在 [Agent SDK](https://code.claude.com/docs/zh-CN/agent-sdk/overview.md) 中，该限制按轮次而非按会话计算。一个轮次从您发送一条消息开始，直到 Claude 完成响应为止。当您在此之后发送下一条消息或开始新的 `claude -p` 运行时，限制会重置，即使使用 [`--continue` 或 `--resume`](https://code.claude.com/docs/zh-CN/headless#continue-conversations) 也是如此。如需更多搜索次数，请提高 [`CLAUDE_CODE_MAX_WEB_SEARCHES_PER_SESSION`](https://code.claude.com/docs/zh-CN/env-vars#variables)，或使用 [`CLAUDE_CODE_WEB_SEARCH_REFILLS_PER_HOUR`](https://code.claude.com/docs/zh-CN/env-vars#variables) 设置恢复速率。

## Write tool 行为

Write tool 创建一个新文件或用提供的完整内容覆盖现有文件。它不会追加或合并。对于字节无法解码的现有文件，Write 同样会将其覆盖，并将新内容保存为 UTF-8，详见[非 UTF-8 文件](#non-utf-8-files)。

Claude 是否必须在当前对话中读取现有文件后才能覆盖它取决于模型和文件：

* Claude Opus 4.6、Claude Haiku 4.5 和更早的模型始终需要读取，因此对未读的现有文件进行 Write 操作会失败并显示错误。
* 较新的模型可以在与[读取前编辑](#edit-tool-behavior)相同的条件下覆盖他们在此会话中从未读过的文件：读取它不需要权限提示，且 Read tool 可用。
* Jupyter notebooks 和 Claude 仅部分读取的文件（带有[`PARTIAL view` 通知](#large-files)）在每个模型上都需要读取。

此约束不适用于新文件。在 v2.1.228 之前，每个模型都需要在覆盖现有文件前进行读取。

使用 Bash 查看文件也满足此要求，遵循[编辑 tool 行为](#edit-tool-behavior)中描述的相同规则。

对于对现有文件的部分更改，Claude 使用 Edit 而不是 Write。

## 检查哪些工具可用

您的确切工具集取决于您的提供商、平台和设置。要检查在运行中的会话中加载了什么，请直接询问 Claude：

```text theme={null}
What tools do you have access to?
```

Claude 提供对话摘要。对于确切的 MCP 工具名称，请运行 `/mcp`。

> [!NOTE]
>
> [advisor tool](https://code.claude.com/docs/zh-CN/advisor.md) 是一个 [server tool](https://platform.claude.com/docs/en/agents-and-tools/tool-use/advisor-tool)，由 API 运行，而不是 Claude Code 实现的工具。它没有您可以在权限规则或 hook 匹配器中引用的名称。

## 另请参阅

* [MCP servers](https://code.claude.com/docs/zh-CN/mcp.md)：通过连接外部服务器添加自定义工具
* [权限](https://code.claude.com/docs/zh-CN/permissions.md)：权限系统、规则语法和工具特定模式
* [Subagents](https://code.claude.com/docs/zh-CN/sub-agents.md)：为 subagents 配置工具访问
* [Hooks](https://code.claude.com/docs/zh-CN/hooks-guide.md)：在工具执行前后运行自定义命令
