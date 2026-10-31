> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# 术语表

> Claude Code 术语定义。了解 agentic loop、compaction、CLAUDE.md、hooks、subagents、MCP 和其他核心概念的含义。

本术语表定义了 Claude Code 术语。每个条目都链接到深入讨论该概念的页面。对于模型级概念（如 tokens、temperature 和 RAG），请参阅[平台术语表](https://platform.claude.com/docs/zh-CN/about-claude/glossary)。对于 Claude Desktop 术语（如 desktop extension、MCPB 和 DXT），请参阅 [Claude 帮助中心](https://support.claude.com/)。

## A

### AGENTS.md

您为 AI 编码代理编写的项目说明的 markdown 文件。如果您的存储库有一个且没有 [CLAUDE.md](#claude-md)，Claude 会将其作为您的项目说明读取，而无需添加第二个文件。您可以在 `/config` 中更改**项目说明**设置，以让 Claude 同时读取两个文件或仅读取 `CLAUDE.md`。直接读取 `AGENTS.md` 需要 Claude Code v2.1.277 或更高版本。在某些会话中 Claude [无法读取 `AGENTS.md`](https://code.claude.com/docs/zh-CN/memory#when-agents-md-support-is-unavailable)，因此请改为[从 `CLAUDE.md` 中导入它](https://code.claude.com/docs/zh-CN/memory#share-one-file-with-other-coding-tools)。

了解更多：[AGENTS.md](https://code.claude.com/docs/zh-CN/memory.md#agents-md)

### Agent teams

由团队负责人协调的多个独立 Claude Code 会话，具有共享任务列表和点对点消息传递。与在单个会话中运行且仅向父级报告的 [subagents](#subagent) 不同，团队成员各自拥有自己的上下文窗口，您可以直接与任何一个交互。Agent teams 是实验性的，默认情况下被禁用；请参阅[启用 agent teams](https://code.claude.com/docs/zh-CN/agent-teams.md#enable-agent-teams)。

了解更多：[运行 agent teams](https://code.claude.com/docs/zh-CN/agent-teams.md)

### Agentic coding

一种工作流程，其中 AI 可以自主读取文件、运行命令和进行更改，而您可以观看、重定向或离开，与只能用文本响应的基于聊天的助手相反，您必须自己应用这些响应。Claude Code 是 agentic 的，因为它拥有允许它采取行动而不仅仅是建议的[工具](#tool)。

了解更多：[Claude Code 如何工作](https://code.claude.com/docs/zh-CN/how-claude-code-works.md)

### Agentic harness

将语言模型转变为能力强大的编码代理的工具、上下文管理和执行环境。Claude Code 是 harness；Claude 是其中的模型。Harness 提供文件访问、shell 执行、权限控制、内存加载以及链接操作的循环。

了解更多：[Claude Code 如何工作](https://code.claude.com/docs/zh-CN/how-claude-code-works.md)

### Agentic loop

Claude 为每个任务所经历的循环：收集上下文、采取行动、验证结果并重复直到完成。每个工具使用都会返回信息，为下一步提供信息。您可以随时中断循环进行重定向。大多数扩展点，包括 [hooks](#hook)、[skills](#skill) 和 [MCP](#mcp-model-context-protocol)，都插入到此循环的特定阶段。

了解更多：[Claude Code 如何工作](https://code.claude.com/docs/zh-CN/how-claude-code-works.md#the-agentic-loop)

### Artifact

Claude Code 从您的会话发布到 claude.ai 上私有 URL 的实时交互式网页，因此您可以直观地查看输出或共享它，而不是阅读终端文本。当会话重新发布时，页面会就地更新。您从 Claude Code 创建的 Artifacts 出现在与 claude.ai 对话中创建的 artifacts 相同的库中。共享选项取决于您的计划：请参阅[共享 artifact](https://code.claude.com/docs/zh-CN/artifacts.md#share-an-artifact)。

了解更多：[将会话输出共享为 artifacts](https://code.claude.com/docs/zh-CN/artifacts.md)

### Auto memory

Claude 根据您的更正和偏好为自己编写的笔记，按 git 存储库存储在 `~/.claude/projects/` 下。同一存储库的所有 worktrees 共享一个 auto memory 目录。`MEMORY.md` 索引的前 200 行或 25 KB 在每个会话开始时加载。Auto memory 是 Claude 编写的对应物，与您编写的 [CLAUDE.md](#claude-md) 相对。

了解更多：[Auto memory](https://code.claude.com/docs/zh-CN/memory.md#auto-memory)

### Auto mode

一种[权限模式](#permission-mode)，其中单独的分类器模型审查操作而不是您，因此 Claude Code 可以在不询问您的情况下运行大多数操作。Claude Code 仍然会在您的显式 ask 规则匹配的操作之前询问您。分类器阻止范围升级、不受信任的基础设施和[提示词注入](#prompt-injection)。工具结果从它看到的内容中被剥离，因此文件或网页中的恶意内容无法直接操纵它。

* **Claude Code v2.1.283 或更高版本**：自动模式是交互式终端和 VS Code 会话的[内置起始权限模式](https://code.claude.com/docs/zh-CN/permission-modes.md#which-mode-a-session-starts-in)
* **早期版本**：自动模式仅在 Pro、Max、Ultra 和 Team 计划上是内置起始权限模式

了解更多：[使用 auto mode 消除提示](https://code.claude.com/docs/zh-CN/permission-modes.md#eliminate-prompts-with-auto-mode)

## B

### Bare mode

使用 `--bare`，Claude Code 启动时不加载 hooks、skills、custom commands、subagents、installed plugins、MCP servers、auto memory 或 CLAUDE.md，除了您通过 `--add-dir` 传递的目录中的 skills。建议用于 CI 和脚本调用，其中您需要在每台机器上获得相同的结果。

了解更多：[使用 bare mode 更快启动](https://code.claude.com/docs/zh-CN/headless.md#start-faster-with-bare-mode)

### Bundled skills

包含在 Claude Code 中的基于提示的 playbooks，例如 `/batch`、`/code-review`、`/debug` 和 `/loop`。与执行固定逻辑的内置命令不同，bundled skills 为 Claude 提供详细的提示并让它编排工作，因此它们可以生成代理、读取文件并适应您的代码库。

了解更多：[Bundled skills](https://code.claude.com/docs/zh-CN/skills.md#bundled-skills)

## C

### Channel

一个 [MCP server](#mcp-model-context-protocol)，将事件推送到您正在运行的会话中，以便 Claude 可以对您离开终端时发生的事情做出反应。Channels 可以是双向的：Claude 读取入站事件并通过同一 channel 回复。Telegram、Discord 和 iMessage 包含在研究预览中。

了解更多：[Channels](https://code.claude.com/docs/zh-CN/channels.md)

### Checkpoint

在您发送的每个开启新轮次的提示词处创建的还原点。Claude Code 在每次编辑之前对文件进行快照，以便检查点可以恢复它们。按两次 `Esc` 或运行 `/rewind` 将代码、对话或两者恢复到较早的点，或从选定的消息总结对话的一部分。

检查点会与对话一起保存，因此恢复的会话仍然可以 `/rewind` 到它们。回退不会撤销 Bash 命令对文件所做的更改，除非 Claude 已在您的对话中对该文件使用过其文件编辑工具。

了解更多：[Checkpointing](https://code.claude.com/docs/zh-CN/checkpointing.md)

### `.claude` directory

Claude Code 读取项目范围配置的目录：settings、hooks、skills、subagents、rules 和 auto memory。项目在其根目录有 `.claude/`；您的用户级默认值在 `~/.claude/`。

了解更多：[The `.claude` directory](https://code.claude.com/docs/zh-CN/claude-directory)

### CLAUDE.md

一个 markdown 文件，包含您为 Claude 编写的持久指令，在每个会话开始时作为系统提示后的用户消息加载。在此处放置项目约定、架构笔记和"始终执行 X"规则。Project-root CLAUDE.md 在 [compaction](#compaction) 期间保留，之后从磁盘重新读取。

您可以在项目作用域内的 `./CLAUDE.md` 或 `./.claude/CLAUDE.md`、用户作用域内的 `~/.claude/CLAUDE.md` 或作为组织的[托管策略](#managed-settings)放置 CLAUDE.md。所有发现的文件都被连接到上下文中，而不是相互覆盖，按从最广泛的作用域到最具体的作用域排序。Claude Code 也可以加载项目的 [AGENTS.md](#agents-md) 文件来代替 CLAUDE.md。

了解更多：[CLAUDE.md files](https://code.claude.com/docs/zh-CN/memory.md#claude-md-files)

### Cloud session

一个 Claude Code 会话，在您关闭笔记本电脑后继续运行，因为它在云基础设施上运行而不是在您的机器上。该基础设施默认由 Anthropic 管理，也可以是您的组织运营的[自托管环境](https://code.claude.com/docs/zh-CN/self-hosted-environments)。您可以从 claude.ai/code、Claude 移动应用、选择了 **Cloud** 的 Desktop 应用、`claude --cloud` 或[云端定时任务](https://code.claude.com/docs/zh-CN/routines.md)启动一个云端会话。

在您的终端、IDE 或选择了 **Local** 的 Desktop 应用中的会话是本地会话；要从另一台设备访问本地会话，请使用 [Remote Control](#remote-control)。

了解更多：[Use Claude Code in the cloud](https://code.claude.com/docs/zh-CN/claude-code-on-the-web.md)

### Command

一个可重用的指令，您可以通过在提示中键入 `/name` 来调用。内置命令（如 `/clear`、`/model` 和 `/compact`）控制会话。您可以在 `.claude/commands/` 中将自己的命令定义为文件，或从 [plugin](#plugin) 安装它们。[Skills](#skill) 是打包多步骤命令的推荐方式。

这个词的另外两种用法是无关的：`claude` CLI 子命令，如 `claude mcp add`，列在 [CLI 参考](https://code.claude.com/docs/zh-CN/cli-reference.md#cli-commands) 中，以及 stdio [MCP server](#mcp-server) 条目的 `command` 字段，它指定 Claude Code 启动以启动服务器的可执行文件。

了解更多：[Commands](https://code.claude.com/docs/zh-CN/commands.md) · [Skills](https://code.claude.com/docs/zh-CN/skills.md)

### Compaction

当[上下文窗口](#context-window)接近其限制时，自动总结您的对话。项目根目录的 CLAUDE.md 和自动记忆在压缩后仍会保留，但仅在对话中给出的指令可能不会保留。运行 `/compact` 手动触发，可选择指定焦点，如 `/compact focus on the API changes`。

了解更多：[压缩后保留的内容](https://code.claude.com/docs/zh-CN/context-window.md#what-survives-compaction) · [上下文填满时](https://code.claude.com/docs/zh-CN/how-claude-code-works.md#when-context-fills-up) · [Claude Code 何时压缩您闲置的对话](https://code.claude.com/docs/zh-CN/settings-reference.md#idlecompaction)

### Connector

一个 [MCP server](#mcp-server)，添加到您的 claude.ai 账户而不是在 Claude Code 中配置。当您使用该账户登录 Claude Code 时，您的 connectors 会在 `/mcp` 中与您本地添加的服务器一起出现。组织也可以配置 connectors 并对其设置按工具控制。

了解更多：[Use MCP servers from claude.ai](https://code.claude.com/docs/zh-CN/mcp.md#use-mcp-servers-from-claude-ai)

### Context window

会话的工作内存，保存对话历史、文件内容、命令输出、CLAUDE.md、auto memory、加载的 skills 和系统指令。当您工作时，上下文会填满直到 [compaction](#compaction) 总结它。运行 `/context` 查看什么在使用空间。对于底层模型概念，请参阅[平台术语表](https://platform.claude.com/docs/zh-CN/about-claude/glossary#context-window)。

了解更多：[Explore the context window](https://code.claude.com/docs/zh-CN/context-window.md)

## D

### Dispatch

一个电话启动的任务路由器，当您从 Claude 移动应用发送编码任务时，在 Desktop 应用中生成 Claude Code 会话。您的提示自动路由到正确的工具。在 Pro、Max 和 Ultra 计划上可用。

了解更多：[来自 Dispatch 的会话](https://code.claude.com/docs/zh-CN/desktop.md#sessions-from-dispatch)

## E

### Effort level

一个设置，控制自适应推理，让模型决定是否以及在每一步上进行多少思考。更高的 effort 意味着更多的思考 token 和更深入的推理；更低的 effort 更快且更便宜。Effort 在 Fable 模型、Opus 4.6 及更高版本、Sonnet 4.6 及更高版本以及 Haiku 5.5 上受支持。

了解更多：[调整 effort level](https://code.claude.com/docs/zh-CN/model-config.md#adjust-effort-level)

### Extended thinking

模型在响应前执行的可见逐步推理。您可以使用 [effort level](#effort-level) 调整它，或使用 `MAX_THINKING_TOKENS` 在具有固定思考预算的模型上限制思考 tokens。思考在终端中以灰色斜体文本显示。

了解更多：[使用 extended thinking](https://code.claude.com/docs/zh-CN/model-config.md#extended-thinking)

## F

### Frontmatter

Markdown 文件最顶部的一个 YAML 设置块，位于开头的 `---` 行和结尾的 `---` 行之间。Skills、subagents、output styles 和 rules 各自从 frontmatter 读取其配置，例如 skill 的 `description` 或 subagent 的 `tools`，并将结尾 `---` 之后的所有内容视为说明。开头的 `---` 必须是文件的第一行。每种文件类型都接受其自己的一组字段。

了解更多：[Skill frontmatter](https://code.claude.com/docs/zh-CN/skills.md#frontmatter-reference)、[Subagent frontmatter](https://code.claude.com/docs/zh-CN/sub-agents.md#supported-frontmatter-fields)、[Output style frontmatter](https://code.claude.com/docs/zh-CN/output-styles.md#frontmatter)、[Rule frontmatter](https://code.claude.com/docs/zh-CN/memory.md#rules-frontmatter-reference)

## H

### Hook

一个用户定义的处理程序，在 Claude Code 生命周期中的特定点自动执行，例如在工具运行之前、文件编辑之后或会话开始时。处理程序可以是 shell 命令、HTTP 端点、MCP 工具、LLM 提示或 subagent。Hooks 是确定性的：它们在固定的生命周期点触发，而不是由模型自行决定。

Hook 配置有三个级别：

* **Hook event**：生命周期点
* **Matcher**：过滤哪些事件触发它
* **Hook handler**：运行什么

了解更多：[开始使用 hooks](https://code.claude.com/docs/zh-CN/hooks-guide.md) · [Hooks 参考](https://code.claude.com/docs/zh-CN/hooks.md)

## M

### Managed settings

由 IT 或 DevOps 在组织范围内强制执行的设置，通过管理员控制台从 Anthropic 的服务器交付，或部署到 `~/.claude` 之外的操作系统级路径。用户和项目设置无法覆盖托管设置。服务器管理的交付适用于[符合条件的配置](https://code.claude.com/docs/zh-CN/server-managed-settings.md#platform-availability)；请参阅[安全考虑](https://code.claude.com/docs/zh-CN/server-managed-settings.md#security-considerations)。使用此功能可实现安全策略、合规要求或跨一个群体的标准化工具。

了解更多：[服务器管理的设置](https://code.claude.com/docs/zh-CN/server-managed-settings.md) · [设置文件](https://code.claude.com/docs/zh-CN/settings.md#where-settings-live)

### MCP (Model Context Protocol)

一个开放标准，用于将 AI 工具连接到外部数据源和服务。MCP servers 为 Claude 提供 Slack、Jira、数据库、浏览器和数百个其他集成的新工具。您可以通过 `/mcp` 连接服务器或将它们添加到 `.mcp.json`。对于协议本身，请参阅[平台术语表](https://platform.claude.com/docs/zh-CN/about-claude/glossary#mcp-model-context-protocol)。

了解更多：[Model Context Protocol](https://code.claude.com/docs/zh-CN/mcp.md)

### MCP server

一个为 Claude 提供工具、提示或资源的程序，通过 [MCP](#mcp-model-context-protocol)。您可以使用 `claude mcp add` 添加服务器，在 `.mcp.json` 中添加，通过 [plugin](#plugin) 添加，或作为 claude.ai [connector](#connector) 添加。本地 stdio 服务器作为一个进程运行，Claude Code 从其配置的 `command` 和 `args` 字段启动该进程，这与您在提示符处键入的 [commands](#command) 无关。

了解更多：[Model Context Protocol](https://code.claude.com/docs/zh-CN/mcp.md)

### MCP Tool Search

一个上下文节省机制，延迟 MCP 工具 schemas 直到需要。只有工具名称和服务器指令在启动时加载；Claude 在决定使用特定工具时按需获取完整 schema。这使空闲 MCP servers 不会消耗太多上下文。

了解更多：[使用 MCP Tool Search 扩展](https://code.claude.com/docs/zh-CN/mcp.md#scale-with-mcp-tool-search)

## N

### Non-interactive mode

一种执行单个提示并退出而不进行对话会话的模式，使用 `-p` 或 `--print` 调用。用于 CI、脚本和管道。该运行仍然保存为可恢复的会话，除非您传递 `--no-session-persistence`。[Agent SDK](https://code.claude.com/docs/zh-CN/agent-sdk/overview.md) 是 Python 和 TypeScript 等效项。以前称为 headless mode。

了解更多：[以编程方式运行 Claude Code](https://code.claude.com/docs/zh-CN/headless.md)

## O

### Output style

一个配置，改变 Claude Code 给予 Claude 的指令，以设置回复行为、语气或格式。与 [CLAUDE.md](#claude-md) 不同，后者在 Claude Code 的默认指令旁添加项目上下文，自定义输出样式会添加自己的指令，并且可以省略默认的软件工程指令。

了解更多：[Output styles](https://code.claude.com/docs/zh-CN/output-styles.md)

## P

### Permission mode

会话的基线批准行为。在 CLI 中使用 `Shift+Tab` 循环或在 VS Code、Desktop 和 claude.ai 中使用模式选择器。可用模式为 `default`、`acceptEdits`、`plan`、`auto`、`dontAsk` 和 `bypassPermissions`。

`default` 模式在 CLI 中标记为 Manual，在 VS Code 和 JetBrains 扩展中也标记为 Manual，在桌面应用中也标记为 Manual，Claude Code 接受 `manual` 作为该值的别名。

了解更多：[选择权限模式](https://code.claude.com/docs/zh-CN/permission-modes.md)

### Permission rule

一个设置条目，根据工具名称和参数模式允许、询问或拒绝工具调用。规则按 deny→ask→allow 顺序评估，首先匹配获胜。Permission rules 是分层在更广泛的 [permission mode](#permission-mode) 之上的细粒度控制。

了解更多：[配置权限](https://code.claude.com/docs/zh-CN/permissions.md)

### Plan mode

一种 [permission mode](#permission-mode)，其中 Claude 研究并提议更改而不编辑您的源文件。它可以读取、搜索和运行探索命令，然后在触及任何内容之前提出批准计划。使用 `/plan` 或按 `Shift+Tab` 进入 plan mode。

了解更多：[使用 plan mode 分析后再编辑](https://code.claude.com/docs/zh-CN/permission-modes.md#analyze-before-you-edit-with-plan-mode)

### Plugin

一个 skills、hooks、subagents 和 MCP servers 的包，打包为单个可安装单元。Plugin skills 命名为 `plugin-name:skill-name`，以便多个 plugins 共存。通过[市场](https://code.claude.com/docs/zh-CN/plugins/overview.md)跨团队分发 plugins。

了解更多：[Plugins](https://code.claude.com/docs/zh-CN/plugins/overview.md)

### Project trust

一个对话框，在 Claude Code 加载其配置之前接受目录。接受情况按项目目录保存，除了您的主目录，其中信任仅在当前会话中保持，并在每次启动时重新显示提示。在您信任目录之前，Claude Code 会暂不加载其存储库提供的某些内容，例如来自 `.claude/settings.json` 的项目允许规则和市场。[在您信任文件夹之前运行的内容](https://code.claude.com/docs/zh-CN/permissions.md#what-runs-before-you-trust-a-folder)列出了每种内容，包括 `-p` 会话在没有对话框的情况下运行的内容。

了解更多：[`.claude` directory](https://code.claude.com/docs/zh-CN/claude-directory)

### Prompt injection

嵌入在文件、网页或工具结果中的恶意指令，试图将 Claude 重定向到您从未要求的操作。Claude Code 的防御包括权限系统、命令注入检测和信任验证。[Auto mode](#auto-mode) 添加了一个服务器端探针，扫描工具结果中的可疑内容，以及一个分类器，用于审查剥离工具结果的操作，因此注入的文本无法直接操纵它。

了解更多：[防止提示注入](https://code.claude.com/docs/zh-CN/security.md#protect-against-prompt-injection)

## R

### Remote Control

一种通过 claude.ai 从您的手机或浏览器继续本地 Claude Code 会话的方式。您的代码执行和文件保留在您的机器上；界面是远程的。与[云会话](https://code.claude.com/docs/zh-CN/claude-code-on-the-web.md)不同，后者在云沙箱中运行。

了解更多：[Remote Control](https://code.claude.com/docs/zh-CN/remote-control.md)

### Rules

`.claude/rules/` 中的模块化指令文件，与 CLAUDE.md 一起加载。规则可以使用 YAML `paths:` frontmatter 进行路径范围限定，因此它仅在 Claude 读取、写入或编辑匹配文件时加载，保持上下文精简直到相关。

了解更多：[使用 `.claude/rules/` 组织规则](https://code.claude.com/docs/zh-CN/memory#organize-rules-with-claude/rules/)

## S

### Sandboxing

Bash 工具的操作系统级文件系统和网络隔离。命令在您预先定义的边界内运行，因此 Claude 可以在其中自由工作，无需每个命令的批准提示。沙箱隔离是与 [permission rules](#permission-rule) 分开的一层。

了解更多：[Sandboxing](https://code.claude.com/docs/zh-CN/sandboxing.md)

### Session

与您当前目录相关的对话，具有自己独立的 [上下文窗口](#context-window)。会话可以使用 `claude -c` 恢复，使用 `--fork-session` 分叉以在新会话 ID 下保留历史，或在终端中并行运行。运行 `/clear` 启动新会话；前一个会话保持存储并可通过 `/resume` 获得。每个会话的会话记录存储在 `~/.claude/projects/` 下。

了解更多：[使用会话](https://code.claude.com/docs/zh-CN/how-claude-code-works.md#work-with-sessions)

### Settings layers

Claude Code 读取配置的层次结构，按优先级顺序从最高到最低：[managed policy](#managed-settings)、通过 `--settings` 标志传递的设置、`.claude/settings.local.json` 处的本地设置、`.claude/settings.json` 处的项目设置，然后是 `~/.claude/settings.json` 处的用户设置。数组跨层合并；更高层的标量覆盖较低的。请参阅 [Settings precedence](https://code.claude.com/docs/zh-CN/settings.md#settings-precedence)。

了解更多：[Settings files](https://code.claude.com/docs/zh-CN/settings.md#where-settings-live)

### Skill

一个 `SKILL.md` 文件，包含 Claude 添加到其工具包中的指令、知识或工作流。Claude 在相关时自动加载 skill，或您可以使用 `/skill-name` 直接调用它。Skills 遵循 Agent Skills 开放标准；Claude Code 使用调用控制和子代理执行扩展它。

Skills 是自定义命令的推荐后继。`.claude/commands/deploy.md` 处的文件和 `.claude/skills/deploy/SKILL.md` 处的文件都创建 `/deploy` 并以相同方式工作；现有命令文件继续工作。

了解更多：[使用 skills 扩展 Claude](https://code.claude.com/docs/zh-CN/skills.md)

### Subagent

一个专门的 AI 助手，在其自己的上下文窗口中运行，具有自定义系统提示词、特定工具访问和独立权限。它处理委派任务并向主对话返回摘要。使用子代理将大型探索保留在主上下文之外或运行并行研究。子代理保持在生成它的会话内。要在您自己运行的单独会话之间传递发现，请使用 [cross-session messaging](https://code.claude.com/docs/zh-CN/cross-session-messaging.md)。

内置子代理包括 Explore、Plan 和通用目的。

了解更多：[创建自定义子代理](https://code.claude.com/docs/zh-CN/sub-agents.md)

### Surface

您访问 Claude Code 的任何地方：CLI、VS Code、JetBrains、Desktop 或 claude.ai。所有使用入口共享相同的引擎。您机器上的会话读取您的本地 CLAUDE.md、设置和 skills；[云端会话](https://code.claude.com/docs/zh-CN/cloud-environments.md#what-carries-over-from-your-setup) 从您的仓库的新克隆开始，不读取您机器上的 `~/.claude/`。Slack 和 Chrome 扩展是连接到使用入口的集成，而不是使用入口本身。

了解更多：[平台和集成](https://code.claude.com/docs/zh-CN/platforms.md)

### System prompt

Claude Code 在每个请求之前发送给您的对话的指令，涵盖 Claude 如何使用工具、安全行为和格式化回复。您可以使用 `--append-system-prompt` 添加到系统提示词或使用 `--system-prompt` 替换它。系统提示词是 [prompt cache](https://code.claude.com/docs/zh-CN/prompt-caching.md#how-the-cache-is-organized) 的第一层。

您的 [CLAUDE.md](#claude-md) 文件和您的 [output style](#output-style) 的指令不是系统提示词的一部分。Claude Code 在对话中将它们作为 [system reminders](#system-reminder) 传递。

了解更多：[System prompt flags](https://code.claude.com/docs/zh-CN/cli-reference.md#system-prompt-flags)

### System reminder

Claude Code 作为 [harness](#agentic-harness) 添加到对话中的消息，为 Claude 提供上下文。您不会自己发送系统提醒。Claude Code 在会话运行时插入它们，例如当会话启动时、当 hook 返回文本时或当文件在磁盘上更改时。Claude 与您的消息一起读取它们。以下所有内容都作为系统提醒到达 Claude：

* 您的 [CLAUDE.md](#claude-md) 文件
* 您的 [output style](#output-style) 的指令
* [hook](#hook) 作为 `additionalContext` 返回的文本
* 可用 [skills](#skill) 的列表
* Claude 之前读取的文件已在磁盘上更改的注记
* 提交和拉取请求的归属行

在记录的 API 请求中，系统提醒出现在用户消息内的 `<system-reminder>` 标签中，或在某些模型上作为具有 `system` 角色的单独消息。

了解更多：[Claude Code 在系统提示词之外添加的上下文](https://code.claude.com/docs/zh-CN/agent-sdk/modifying-system-prompts.md#context-claude-code-adds-outside-the-system-prompt)

## T

### Teleport

一个命令 `/teleport`，将云 Claude Code 会话拉入您的本地终端。Claude 获取分支、加载对话历史并从云会话的最后状态恢复。反向方向是 `--cloud`，它将本地任务发送到云上运行。

了解更多：[在终端中继续云端会话](https://code.claude.com/docs/zh-CN/claude-code-on-the-web.md#from-cloud-to-terminal)

### Tool

Claude 可以采取的操作：读取文件、编辑代码、运行 shell 命令、搜索 web、生成 subagent。Tools 是使 Claude Code agentic 的原因。没有它们，Claude 只能用文本响应。每个工具使用都会返回一个结果，为 [agentic loop](#agentic-loop) 中 Claude 的下一个决策提供信息。

了解更多：[Claude 可用的工具](https://code.claude.com/docs/zh-CN/tools-reference.md)

### Transcript

[session](#session) 的存储记录。对话是您和 Claude 之间的交换；transcript 是将该对话保存为文件，默认位置为 `~/.claude/projects/<project>/<session-id>.jsonl`。Claude Code 在您恢复时读取该文件，这就是会话结束后对话如何继续的方式。有关同一对话的屏幕视图，请参阅 [transcript viewer](https://code.claude.com/docs/zh-CN/interactive-mode.md#transcript-viewer)。

了解更多：[Transcripts 存储位置](https://code.claude.com/docs/zh-CN/sessions.md#where-transcripts-are-stored)

### Turn

Claude 在一个 [session](#session) 中的一个完整响应。一个 turn 从您发送消息开始，到 Claude 完成响应结束，中间可能有任意数量的 [tool](#tool) 调用。[Stop hooks](#hook) 在每个 turn 的末尾触发。一个 session 由许多 turn 组成，[agentic loop](#agentic-loop) 描述了在一个 turn 内发生的情况。

了解更多：[Claude Code 如何工作](https://code.claude.com/docs/zh-CN/how-claude-code-works.md#the-agentic-loop)

## V

### Verification loop

一个会话知道工作实际完成而不仅仅是看起来合理的方式。您给 Claude 一个它可以运行的检查，例如测试套件、构建或屏幕截图比较，Claude 迭代直到检查通过，而不是在一次尝试后停止。验证循环是 [`/goal`](https://code.claude.com/docs/zh-CN/goal)、无人值守运行和[动态工作流](https://code.claude.com/docs/zh-CN/workflows.md)的前提条件：没有它，唯一决定 Agent 是否完成的就是 Agent 本身。

了解更多：[给 Claude 一个可以运行的检查](https://code.claude.com/docs/zh-CN/best-practices.md#give-claude-a-check-to-run)

## W

### Worktree isolation

一个隔离模式，在 `.claude/worktrees/` 下的单独 git worktree 中运行 Claude，使用 `-w` 标志或子代理配置中的 `isolation: worktree` 启用。更改保留在单独目录中的单独分支上，因此每个并行 Agent 都各自编辑自己的文件副本。

了解更多：[使用 git worktrees 运行并行会话](https://code.claude.com/docs/zh-CN/worktrees.md)

***

## 已弃用和重命名的术语

这些术语出现在较旧的文档、博客文章和社区内容中。搜索此网站时使用当前名称。

| 旧术语 | 现在称为 | 注释 |
| - | - | - |
| Headless mode | [Non-interactive mode](#non-interactive-mode) | 相同的 `-p` 标志，相同的行为 |
| Web session; "Claude Code on the web" as the name for any cloud session | [Cloud session](#cloud-session) | "Claude Code on the web" 现在仅命名 claude.ai/code 处的浏览器界面 |
| Custom commands | [Skills](#skill) | `.claude/commands/` 文件仍然有效 |
| Slash commands | Commands | "Slash"从产品副本中删除 |
