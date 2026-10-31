> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# 创建自定义 subagents

> 在 Claude Code 中创建和使用专门的 AI subagents，用于特定任务的工作流和改进的上下文管理。

Subagents 是处理特定类型任务的专门 AI 助手。当一个辅助任务会用搜索结果、日志或文件内容充斥您的主对话，而您不会再次引用这些内容时，请使用一个 subagent：该 subagent 在自己的上下文中完成这项工作，仅返回摘要。当您不断生成相同类型的工作者并使用相同的指令时，定义一个自定义 subagent。

每个 subagent 在自己的 context window 中运行，具有自定义系统提示、特定的工具访问权限和独立的权限。它还发送自己的请求，这些请求计入与您的主对话相同的[使用限制](https://code.claude.com/docs/zh-CN/costs.md#plan-usage-breakdown)。当 Claude 遇到与 subagent 描述相匹配的任务时，它会委托给该 subagent，该 subagent 独立工作并返回结果。要在实践中看到上下文节省，[context window 可视化](https://code.claude.com/docs/zh-CN/context-window.md)演示了一个 subagent 在自己的独立窗口中处理研究的会话。

> [!NOTE]
>
> Subagents 在单个会话中工作。要在并行运行许多独立会话并从一个地方监控它们，请参阅 [background agents](https://code.claude.com/docs/zh-CN/agent-view.md)。对于相互传递消息的单独会话，请参阅 [cross-session messaging](https://code.claude.com/docs/zh-CN/cross-session-messaging.md)。对于 Claude 生成和监督的协调团队会话，请参阅 [agent teams](https://code.claude.com/docs/zh-CN/agent-teams.md)。

Subagents 帮助您：

* **保留上下文**，通过将探索和实现保持在主对话之外
* **强制执行约束**，通过限制 subagent 可以使用的工具
* **跨项目重用配置**，使用用户级 subagents
* **专门化行为**，为特定领域使用专注的系统提示
* **控制成本**，通过将任务路由到更快、更便宜的模型（如 Haiku）

Claude 使用每个 subagent 的描述来决定何时委托任务。创建 subagent 时，请编写清晰的描述，以便 Claude 知道何时使用它。

这些描述会占用上下文，因此请保持简洁。当您的 subagents（除了内置的 subagents）的组合描述超过 15,000 个 token 时，Claude Code 会在启动时显示 [包含总 token 计数的警告](https://code.claude.com/docs/zh-CN/errors.md#agent-descriptions-are-over-the-15000-token-limit)。修剪您的 subagents 的 `description` 字段，并将详细信息移到每个 subagent 的系统提示中，该提示仅在该 subagent 运行时加载。

## 内置 subagents

Claude Code 包括内置 subagents，Claude 在适当时自动使用。每个都继承父对话的权限规则；大多数运行时工具集受限。

Explore 和 Plan 启动时不加载您的 CLAUDE.md 文件和 git 状态快照，以保持研究快速且成本低廉。其他内置子代理和[自定义子代理](#configure-subagents)都会加载两者，除非您在自定义子代理的定义中设置 [`omitClaudeMd`](#supported-frontmatter-fields)，使其启动时不加载用户、项目和本地 CLAUDE.md 文件。有关到达子代理的内容的完整分解，请参阅[启动时加载的内容](#what-loads-at-startup)。

> [tab] Explore
>
> 一个快速的、只读的代理，针对搜索和分析代码库进行了优化。
>
> * **Model**: 主对话的模型。当主对话运行 Fable 时，Explore 的模型取决于您的连接方式：
>   * 使用 Claude 订阅、Anthropic Console 账户，或通过 `ANTHROPIC_BASE_URL` 访问的 [LLM 网关](https://code.claude.com/docs/zh-CN/llm-gateway)时，Explore 在 [`opus` 别名](https://code.claude.com/docs/zh-CN/model-config#model-aliases)解析到的 Opus 模型上运行。
>   * 在 Amazon Bedrock、Google Cloud 的 Agent Platform、Microsoft Foundry、[AWS 上的 Claude Platform](https://code.claude.com/docs/zh-CN/claude-platform-on-aws.md) 或 [Claude apps 网关](https://code.claude.com/docs/zh-CN/claude-apps-gateway)上，Explore 保持使用主对话的模型。
> * **Tools**: 只读工具；拒绝访问 Write 和 Edit
> * **Purpose**: 文件发现、代码搜索、代码库探索
>
> 名为 `Explore` 的[用户或项目子代理](#choose-the-subagent-scope)会覆盖内置的子代理，并保持其自己的 `model` 字段，因此可以定义一个带有 `model: haiku` 的子代理，在较低成本的模型上运行探索。要将同一个模型强制应用于每个子代理（包括 Explore），请参阅[在一个模型上运行每个子代理](#run-every-subagent-on-one-model)。
>
> 当 Claude 需要搜索或理解代码库而不进行更改时，它会委托给 Explore。这样可以将探索结果保持在主对话上下文之外。
>
> 调用 Explore 时，Claude 指定一个彻底程度级别：**quick** 用于有针对性的查找，**medium** 用于平衡的探索，或 **very thorough** 用于全面分析。

> [tab] Plan
>
> 一个研究代理，在 [plan mode](https://code.claude.com/docs/zh-CN/permission-modes.md#analyze-before-you-edit-with-plan-mode) 期间使用，以在呈现计划之前收集上下文。
>
> * **Model**: 从主对话继承，除非您设置 `CLAUDE_CODE_SUBAGENT_MODEL` 并[强制将其应用于每个 subagent](#run-every-subagent-on-one-model)
> * **Tools**: 只读工具；拒绝访问 Write 和 Edit
> * **Purpose**: 用于规划的代码库研究
>
> 当您处于 plan mode 并且 Claude 需要理解您的代码库时，它会将研究委托给 Plan subagent，以便探索输出保持在单独的上下文窗口中，而主对话保持只读。

> [tab] General-purpose
>
> 一个能够处理复杂、多步骤任务的代理，需要探索和操作。
>
> * **Model**: 如果您设置了 [`CLAUDE_CODE_SUBAGENT_MODEL`](#choose-a-model) 模型且没有其他方式分配模型，则使用该模型，否则使用主对话的模型；[选择模型](#choose-a-model) 说明了完整的顺序，[在一个模型上运行每个 subagent](#run-every-subagent-on-one-model) 展示了如何使变量覆盖这些来源
> * **Tools**: 所有[可用于 subagents 的工具](#available-tools)
> * **Purpose**: 复杂研究、多步骤操作、代码修改
>
> 当任务需要探索和修改、复杂推理来解释结果或多个依赖步骤时，Claude 会委托给 general-purpose。

> [tab] Other
>
> Claude Code 包括用于特定任务的其他辅助代理。这些通常会自动调用，因此您不需要直接使用它们。
>
> | Agent | Model | Claude 何时使用它 |
> | :- | :- | :- |
> | claude | 没有自己的；当 Claude 将其生成为 subagent 时遵循[模型顺序](#choose-a-model) | 当任务不适合更专业的代理时。一个具有所有[可用于 subagents 的工具](#available-tools)的通用代理。也是调度的[后台会话](https://code.claude.com/docs/zh-CN/agent-view.md)的默认代理；[它启动的权限模式](https://code.claude.com/docs/zh-CN/agent-view.md#permission-mode-model-and-effort)取决于会话的启动方式 |
> | statusline-setup | Sonnet | 当您运行 `/statusline` 来配置您的状态行时 |
> | claude-code-guide | Haiku | 当您提出关于 Claude Code 功能的问题时 |

内置 subagents 在交互式会话中默认被注册。要限制它们：

* 要阻止特定的内置类型，请将其添加到 `permissions.deny`，如[禁用特定 subagents](#disable-specific-subagents) 中所示。
* 要防止 Claude 委托给任何 subagent，请使用 [`permissions.deny`](https://code.claude.com/docs/zh-CN/permissions#tool-specific-permission-rules) 拒绝 `Agent` 工具本身。
* 要仅移除内置的 `Explore` 和 `Plan` subagents，请设置 [`CLAUDE_CODE_DISABLE_EXPLORE_PLAN_AGENTS=1`](https://code.claude.com/docs/zh-CN/env-vars)。Claude 直接读取和探索文件，而不是委托给它们。需要 Claude Code v2.1.198 或更高版本。
* 在[非交互模式](https://code.claude.com/docs/zh-CN/headless.md) 和 [Agent SDK](https://code.claude.com/docs/zh-CN/agent-sdk/overview.md) 中，设置 [`CLAUDE_AGENT_SDK_DISABLE_BUILTIN_AGENTS=1`](https://code.claude.com/docs/zh-CN/env-vars) 以移除所有内置类型并仅提供您自己的。

当会话没有 `general-purpose` subagent 可回退时，省略 `subagent_type` 的 Agent 工具调用会失败，并显示 [`subagent_type is required`](https://code.claude.com/docs/zh-CN/errors#subagent-type-is-required)。

除了这些内置 subagents，您可以创建自己的，具有自定义提示、工具限制、权限模式、hooks 和 skills。以下部分展示了如何开始和自定义 subagents。

## 快速入门：创建您的第一个子代理

子代理是带有 YAML frontmatter 的 Markdown 文件。要创建一个，请要求 Claude 为您编写，或者 [自己编写文件](#write-subagent-files)。

本演练创建一个用户级子代理，用于审查代码并建议改进。

**1. 要求 Claude 创建子代理**

在 Claude Code 中，描述您想要的子代理及其保存位置：

```text wrap theme={null}
Create a personal code-improver subagent in ~/.claude/agents/ that scans
files and suggests improvements for readability, performance, and best
practices. It should explain each issue, show the current code, and
provide an improved version. Make it read-only and have it use Sonnet.
```

Claude 使用 `name`、`description`、`tools` 列表、`model` 和系统提示词来编写文件。

**2. 审查文件**

打开 `~/.claude/agents/code-improver.md` 并确认 frontmatter 与您的要求相符。结果如下所示：

```markdown theme={null}
---
name: code-improver
description: Scans files and suggests improvements for readability, performance, and best practices. Use after writing or modifying code.
tools: Read, Grep, Glob
model: sonnet
---

You are a code improvement specialist. For each issue you find, explain
the problem, show the current code, and provide an improved version.
```

因为该文件位于 `~/.claude/agents/`，所以该子代理在您机器上的每个项目中都可用。要将其限定在一个项目中，请将其移动到该项目的 `.claude/agents/` 目录。[选择子代理作用域](#choose-the-subagent-scope) 比较了两者。

**3. 尝试一下**

要求 Claude 委托给新的子代理：

```text wrap theme={null}
Use the code-improver agent to suggest improvements in this project
```

Claude 委托给您的新子代理，它扫描代码库并返回改进建议。在会话记录中，委托显示为工具调用行，显示子代理的名称后跟简短的任务描述，例如 `code-improver(Suggest code improvements)`。

如果 Claude 找不到新的子代理，请重新启动 Claude Code 并重试。这仅在会话开始前 `~/.claude/agents/` 不存在时发生，因为运行中的会话不会检测到新创建的 `agents` 目录。

现在您有了一个子代理，可以在您机器上的任何项目中使用它来分析代码库并建议改进。

您也可以手动编写子代理文件、通过 CLI 标志定义它们，或通过插件分发它们。以下部分涵盖所有配置选项。

> [!NOTE]
>
> 运行 `/agents` 会打印一个提醒，提示您询问 Claude 或直接编辑 `.claude/agents/` 和 `~/.claude/agents/`。
> 在 Claude Code v2.1.197 及更早版本中，`/agents` 打开一个交互式向导，其中有一个 **Running** 选项卡列出实时子代理，以及一个 **Library** 选项卡用于创建、编辑和删除它们。

## 配置子代理

子代理文件的位置决定了谁可以使用它，其 frontmatter 决定了它能做什么。本节介绍子代理文件的存放位置以及它们支持的所有字段。

### 选择子代理作用域

根据作用域将子代理文件存放在不同位置。当多个子代理同名时，Claude Code 使用优先级较高位置中的那个。

| 位置 | 作用域 | 优先级 | 创建方式 |
| :- | :- | :- | :- |
| 托管设置 | 整个组织 | 1（最高） | 通过[托管设置](https://code.claude.com/docs/zh-CN/settings.md)部署 |
| `--agents` CLI 标志 | 当前会话 | 2 | 启动 Claude Code 时传入 JSON |
| `.claude/agents/` | 当前项目 | 3 | 让 Claude 创建，或手动创建文件 |
| `~/.claude/agents/` | 您的所有项目 | 4 | 让 Claude 创建，或手动创建文件 |
| 插件的 `agents/` 目录 | 启用插件的位置 | 5（最低） | 随[插件](https://code.claude.com/docs/zh-CN/plugins/overview.md)一起安装 |

**项目子代理**（`.claude/agents/`）非常适合特定于某个代码库的子代理。将它们提交到版本控制中，以便您的团队可以协作使用和改进它们。

项目子代理的发现方式是从当前工作目录向上遍历，因此从该目录到仓库根目录之间的每个 `.claude/agents/` 都会被扫描。当这些嵌套目录中有多个定义了相同的 `name` 时，Claude Code 使用离工作目录最近的定义。

当您使用 `--add-dir` 或 `/add-dir` 添加目录时，Claude Code 也会加载该目录的 `.claude/agents/` 文件夹，与您的项目子代理一起使用。有关还有哪些其他配置类型会从 `--add-dir` 加载，请参阅[额外目录](https://code.claude.com/docs/zh-CN/permissions.md#additional-directories-grant-file-access-not-configuration)。若要在不使用 `--add-dir` 的情况下跨项目共享子代理，请使用 `~/.claude/agents/` 或[插件](https://code.claude.com/docs/zh-CN/plugins/overview.md)。

**用户子代理**（`~/.claude/agents/`）是在您所有项目中都可用的个人子代理。

Claude Code 会递归扫描 `.claude/agents/` 和 `~/.claude/agents/`，因此您可以将定义组织到子文件夹中，例如 `agents/review/` 或 `agents/research/`。子目录路径不影响子代理的识别或调用方式，因为其身份仅来自 `name` frontmatter 字段。

请在整个目录树中保持 `name` 值唯一：如果同一个 `.claude/agents/` 目录（包括其子文件夹）下的两个文件声明了相同的名称，Claude Code 只会加载其中一个，具体由文件系统读取顺序决定，而不是由文档规定的优先级决定。在嵌套的项目目录之间，如上所述，离工作目录最近的定义优先。[`/doctor`](https://code.claude.com/docs/zh-CN/commands#all-commands) 设置检查会报告同一目录中同名的文件，并建议重命名或删除多余的文件，只保留一个。

插件的 `agents/` 目录也会被递归扫描。与项目和用户作用域不同，插件 `agents/` 目录中的子文件夹会成为[作用域标识符](#invoke-subagents-explicitly)的一部分：插件 `my-plugin` 中位于 `agents/review/security.md` 的文件注册为 `my-plugin:review:security`。

**CLI 定义的子代理**在启动 Claude Code 时以 JSON 形式传入。它们仅在该会话中存在，不会保存到磁盘，因此适用于快速测试或自动化脚本。您可以在一次 `--agents` 调用中定义多个子代理：

> [tab] macOS, Linux, WSL
>
> ```bash theme={null}
> claude --agents '{
>   "code-reviewer": {
>     "description": "Expert code reviewer. Use proactively after code changes.",
>     "prompt": "You are a senior code reviewer. Focus on code quality, security, and best practices.",
>     "tools": ["Read", "Grep", "Glob", "Bash"],
>     "model": "sonnet"
>   },
>   "debugger": {
>     "description": "Debugging specialist for errors and test failures.",
>     "prompt": "You are an expert debugger. Analyze errors, identify root causes, and provide fixes."
>   }
> }'
> ```

> [tab] Windows PowerShell
>
> ```powershell theme={null}
> claude --agents @'
> {
>   "code-reviewer": {
>     "description": "Expert code reviewer. Use proactively after code changes.",
>     "prompt": "You are a senior code reviewer. Focus on code quality, security, and best practices.",
>     "tools": ["Read", "Grep", "Glob", "Bash"],
>     "model": "sonnet"
>   },
>   "debugger": {
>     "description": "Debugging specialist for errors and test failures.",
>     "prompt": "You are an expert debugger. Analyze errors, identify root causes, and provide fixes."
>   }
> }
> '@
> ```

在[非交互模式](https://code.claude.com/docs/zh-CN/headless.md)下，`--agents` 也接受一个包含相同对象的 JSON 文件路径，适用于定义过大而无法在命令行中传入的情况。例如，`claude -p --agents ./agents.json "Review my changes"` 会从该文件读取定义。在交互式会话中，Claude Code 会拒绝文件路径。文件形式需要 Claude Code v2.1.281 或更高版本。

JSON 中的每个顶层键都是一个 Agent 的名称，其值是该 Agent 的定义。名称不要以 `-` 开头。定义包含以下字段：

* **`prompt`**：Agent 的系统提示词，等同于基于文件的子代理中的 markdown 正文。`prompt` 可以为空。如果您使用 `--agent` 将一个 `prompt` 为空且没有 `memory` 字段的 Agent 选为会话的 Agent，会话的系统提示词将保持不变。空 `prompt` 需要 Claude Code v2.1.281 或更高版本。
* **[Frontmatter 字段](#supported-frontmatter-fields)**：`description`、`tools`、`disallowedTools`、`model`、`permissionMode`、`mcpServers`、`hooks`、`maxTurns`、`autoCompactWindow`、`skills`、`initialPrompt`、`memory`、`effort`、`background`、`omitClaudeMd` 和 `isolation`。
* **被忽略的字段**：`color` 和 `experimental` 在此处不被接受，会被忽略而不是被拒绝。

有关 Claude Code 如何处理无法加载的值，以及跳过该检查的标志和环境变量，请参阅 [`Invalid --agents configuration`](https://code.claude.com/docs/zh-CN/errors#invalid-agents-configuration)。

**托管子代理**由组织管理员部署。将 markdown 文件放在[托管设置目录](https://code.claude.com/docs/zh-CN/managed-settings.md#delivery-mechanisms)内的 `.claude/agents/` 中，使用与项目和用户子代理相同的 frontmatter 格式。托管定义优先于同名的项目和用户子代理。

**插件子代理**来自您已安装的[插件](https://code.claude.com/docs/zh-CN/plugins/overview.md)。它们会与您的自定义子代理一起自动加载，并以其作用域名称出现在 @ 提及的输入提示中。有关创建插件子代理的详细信息，请参阅[插件组件参考](https://code.claude.com/docs/zh-CN/plugins/components.md#agents)。

> [!NOTE]
>
> 出于安全原因，插件子代理不支持 `hooks`、`mcpServers` 或 `permissionMode` frontmatter 字段。从插件加载 Agent 时，这些字段会被忽略。如果您需要它们，请将 Agent 文件复制到 `.claude/agents/` 或 `~/.claude/agents/` 中。您也可以在 `settings.json` 或 `settings.local.json` 中向 [`permissions.allow`](https://code.claude.com/docs/zh-CN/settings-reference#permissions-allow) 添加规则，但这些规则适用于整个会话，而不仅仅是插件子代理。
>
> 如果您是插件的作者，请改为在插件的 [`hooks/hooks.json`](https://code.claude.com/docs/zh-CN/plugins/components#hooks) 中提供 hook，在其 [`.mcp.json`](https://code.claude.com/docs/zh-CN/plugins/components#mcp-servers) 中提供 MCP 服务器。它们会在插件启用时生效，而不仅仅在子代理内部生效。

您还可以将子代理定义复用为 [agent team](https://code.claude.com/docs/zh-CN/agent-teams.md) 的队友：在请求 Claude 生成队友时指定子代理类型，Claude Code 会将该定义的部分内容应用于该队友。[将子代理定义用于队友](https://code.claude.com/docs/zh-CN/agent-teams.md#use-subagent-definitions-for-teammates)说明了在每种显示模式下适用哪些作用域和哪些部分。

### 编写子代理文件

子代理文件使用 YAML frontmatter 进行配置，随后是 Markdown 格式的系统提示词：

> [!NOTE]
>
> Claude Code 会监视 `~/.claude/agents/` 和 `.claude/agents/`。当您在磁盘上添加或编辑子代理文件，或让 Claude 为您编写一个时，Claude Code 会在几秒钟内检测到更改，下一次委派将使用更新后的定义，无需重启。
>
> 以下三种情况仍需要重启：
>
> * 监视器只覆盖会话启动时已存在的目录，因此在新的 `agents` 目录中创建某个作用域的第一个 Agent 文件后，需要重启才能加载它。
> * Claude Code 不会监视通过 `--add-dir` 或 `/add-dir` 添加的目录中的 `.claude/agents/`，因此在那里添加或编辑子代理后，需要重启才能加载更改。
> * 使用 `--disable-slash-commands` 启动的会话完全不会监视这些目录。

```markdown .claude/agents/code-reviewer.md theme={null}
---
name: code-reviewer
description: Reviews code for quality and best practices
tools: Read, Glob, Grep
model: sonnet
---

You are a code reviewer. When invoked, analyze the code and provide
specific, actionable feedback on quality, security, and best practices.
```

frontmatter 定义子代理的元数据和配置。正文成为指导子代理行为的系统提示词。子代理只接收此系统提示词以及基本的环境详细信息（如工作目录），而不接收 Claude Code 的系统提示词。

在[非交互模式](https://code.claude.com/docs/zh-CN/headless.md)下，传入 [`--append-subagent-system-prompt`](https://code.claude.com/docs/zh-CN/cli-reference#cli-flags) 可将您的文本追加到每个子代理（包括嵌套子代理）的系统提示词末尾，但[分叉子代理](#fork-the-current-conversation)除外，它复用对话自身的提示词。需要 Claude Code v2.1.205 或更高版本。如果您的文本太长而无法在命令行中传入，请将其保存到文件中，并改用 `--append-subagent-system-prompt-file` 传入路径。文件标志需要 Claude Code v2.1.261 或更高版本。

子代理在主对话的当前工作目录中启动。在子代理中，`cd` 命令不会在 Bash 或 PowerShell 工具调用之间保留，也不会影响主对话的工作目录。若要为子代理提供仓库的隔离副本，请设置 [`isolation: worktree`](#supported-frontmatter-fields)。

设置了 `isolation: worktree` 的子代理会在其 worktree 内运行 Bash 和 PowerShell 命令。如果某个命令的工作目录解析为您的主检出目录（例如因为 worktree 目录在子代理运行期间被删除），该命令会失败并报错。

此工作目录检查覆盖包含您启动 Claude Code 的目录的整个仓库。当您的会话本身运行在一个链接的 [worktree](https://code.claude.com/docs/zh-CN/worktrees.md) 中时，该检查还覆盖该 worktree 所链接的主检出目录。

对于 Bash 命令，Claude Code 还会从两个方面检查命令本身：

* 它会阻止将 git 重定向到主检出目录的命令。
* 当无法从命令文本中验证命令运行的任何 git 都停留在 worktree 内时（例如命令名称是在运行时计算的），它会拒绝该命令。

重定向途径和形式规则列在[Claude Code 如何强制执行隔离](https://code.claude.com/docs/zh-CN/worktrees.md#how-claude-code-enforces-isolation)中。PowerShell 命令只进行工作目录检查。

[Monitor](https://code.claude.com/docs/zh-CN/tools-reference.md#monitor-tool) 命令与 Bash 命令一样，会经过相同的工作目录和命令内容检查。

当主对话本身在 worktree 中隔离运行时，Claude Code 会对会话及其生成的每个子代理应用相同的检查，包括没有设置 `isolation: worktree` 的子代理；请参阅[Claude Code 如何强制执行隔离](https://code.claude.com/docs/zh-CN/worktrees.md#how-claude-code-enforces-isolation)。

### Frontmatter 参考

使用子代理文件顶部 `---` 标记之间的 YAML [frontmatter](https://code.claude.com/docs/zh-CN/glossary.md#frontmatter) 来配置子代理，并在结束的 `---` 之后以 Markdown 编写其系统提示词。只有 `name` 和 `description` 是必需的。

多词字段名使用 camelCase，例如 `maxTurns` 和 `disallowedTools`，并且必须与表格完全一致：Claude Code 会忽略无法识别的字段，且不报告错误。

* 若要在已加载的子代理中查找拼写错误的字段，请在 shell 中运行 [`claude --debug`](https://code.claude.com/docs/zh-CN/cli-reference#cli-flags)，并在 `~/.claude/debug/<session-id>.txt` 的调试日志中搜索 `does not recognize or apply`。调试日志会列出子代理文件中最多五个 Claude Code 无法识别的顶层字段，并在存在相近的受支持字段时给出可能想写的名称。需要 Claude Code v2.1.296 或更高版本。
* 若要了解子代理文件为何未加载，请参阅 [Claude Code 跳过的子代理文件](#subagent-files-claude-code-skips)。

| 字段 | 必需 | 描述 |
| :- | :- | :- |
| `name` | 是 | 最多 256 个字符的唯一标识符，例如 `code-reviewer` 或 `reviewer-v2`。[Hook](https://code.claude.com/docs/zh-CN/hooks.md#subagentstart) 以 `agent_type` 的形式接收此值。文件名不必与之匹配。名称不能包含 `:`，它保留给[插件作用域标识符](https://code.claude.com/docs/zh-CN/plugins/overview.md)，例如 `my-plugin:reviewer` |
| `description` | 是 | Claude 应在何时委派给此子代理 |
| `tools` | 否 | 子代理可以使用的[工具](#available-tools)，可以是逗号分隔的字符串（如 `Read, Grep, Bash`）或 YAML 列表。省略时继承子代理可用的所有工具。如果列表中没有任何条目能解析为工具，子代理通常会[启动失败](https://code.claude.com/docs/zh-CN/errors.md#agent-would-be-spawned-with-zero-tools)，并报告列出这些条目的错误。若要将 skill 预加载到上下文中，请使用 `skills` 字段，而不是在此处列出 `Skill` |
| `disallowedTools` | 否 | 要拒绝的工具，从继承的或指定的列表中移除。格式与 `tools` 相同。带有说明符的条目（如 `Bash(git push *)`）仍会[移除整个工具](#restrict-which-tools-a-subagent-can-use) |
| `model` | 否 | 要使用的[模型](#choose-a-model)：`sonnet`、`opus`、`haiku`、`fable`、完整模型 ID（如 `claude-opus-5-5`）或 `inherit`。省略时，Claude Code 按照[子代理模型顺序](#choose-a-model)选择模型 |
| `permissionMode` | 否 | [权限模式](#permission-modes)：`default`、`acceptEdits`、`auto`、`dontAsk`、`bypassPermissions`、`plan`，或作为 `default` 别名的 `manual`。对[插件子代理](#choose-the-subagent-scope)忽略 |
| `maxTurns` | 否 | 子代理停止前的最大 Agent 轮次数。当子代理达到限制时，Claude Code 会返回其标记为部分结果的输出，Claude 可以[恢复它](#resume-subagents)以继续。部分结果标记需要 Claude Code v2.1.246 或更高版本 |
| `autoCompactWindow` | 否 | 将此子代理的[自动压缩窗口](#subagent-auto-compact-window)降低到 `100000` 至 `1000000` 之间的 token 数。需要 Claude Code v2.1.296 或更高版本 |
| `skills` | 否 | 启动时预加载到子代理上下文中的 [skill](https://code.claude.com/docs/zh-CN/skills.md)。注入的是完整的 skill 内容，而不仅仅是描述。子代理仍可通过 Skill 工具调用未列出的项目、用户和插件 skill |
| `mcpServers` | 否 | 此子代理可用的 [MCP 服务器](https://code.claude.com/docs/zh-CN/mcp.md)。每个条目可以是引用已配置服务器的服务器名称（例如 `"slack"`），也可以是以服务器名称为键、以完整 [MCP 服务器配置](https://code.claude.com/docs/zh-CN/mcp.md#installing-mcp-servers)为值的内联定义。对[插件子代理](#choose-the-subagent-scope)忽略 |
| `hooks` | 否 | 限定于此子代理的[生命周期 hook](#define-hooks-for-subagents)。对[插件子代理](#choose-the-subagent-scope)忽略 |
| `memory` | 否 | [持久记忆作用域](#enable-persistent-memory)：`user`、`project` 或 `local`。启用跨会话学习 |
| `background` | 否 | 设置为 `true` 可使此子代理保持在后台运行，即使 Claude 要求等待它。默认情况下，Claude 从主对话生成的子代理已经会[在后台启动](#run-subagents-in-foreground-or-background) |
| `omitClaudeMd` | 否 | 设置为 `true` 可使此子代理在启动时的[初始上下文](#what-loads-at-startup)中不包含用户、项目和本地 CLAUDE.md 文件；[托管策略文件](https://code.claude.com/docs/zh-CN/memory.md#how-claude-md-files-load)仍会加载，但[托管子代理](#choose-the-subagent-scope)除外。适用于从委派提示词中获取所需全部信息的子代理。当 Agent 通过 `--agent` 或 `agent` 设置作为主会话 Agent 运行时忽略。需要 Claude Code v2.1.271 或更高版本 |
| `effort` | 否 | 此子代理活动时的 effort 级别。覆盖会话的 effort 级别，但不覆盖 [`CLAUDE_CODE_EFFORT_LEVEL`](https://code.claude.com/docs/zh-CN/env-vars#variables) 环境变量。选项：`low`、`medium`、`high`、`xhigh`、`max`；可用级别取决于模型。请参阅[选择 effort 级别](#choose-an-effort-level) |
| `isolation` | 否 | 设置为 `worktree` 可在临时 [git worktree](https://code.claude.com/docs/zh-CN/worktrees.md) 中运行子代理，为其提供仓库的隔离副本，默认从您的[默认分支](https://code.claude.com/docs/zh-CN/worktrees.md#choose-the-base-branch)而不是父会话的 `HEAD` 创建分支。如果子代理未做任何更改，worktree 会被自动清理 |
| `color` | 否 | 子代理在任务列表和会话记录中的显示颜色。接受 `red`、`blue`、`green`、`yellow`、`purple`、`orange`、`pink` 或 `cyan` |
| `initialPrompt` | 否 | 当此 Agent 作为主会话 Agent 运行时（通过 `--agent` 或 `agent` 设置），作为第一个用户轮次自动提交。会处理其中的[命令](https://code.claude.com/docs/zh-CN/commands.md)和 [skill](https://code.claude.com/docs/zh-CN/skills.md)。添加在任何用户提供的提示词之前。对[插件子代理](#choose-the-subagent-scope)忽略 |
| `experimental` | 否 | 实验性选项的映射。将其 `cacheTtl` 键设置为 `5m` 或 `1h`，可为此子代理的请求选择[提示缓存生命周期](https://code.claude.com/docs/zh-CN/prompt-caching.md#choose-the-ttl-yourself)，其优先级取决于 frontmatter 在[缓存生命周期优先级](https://code.claude.com/docs/zh-CN/prompt-caching.md#choose-the-ttl-yourself)中的位置。Claude Code 会忽略任何其他值，在您的 Claude 订阅使用使用额度时忽略 `1h`，并且只从子代理文件中读取该字段。需要 Claude Code v2.1.248 或更高版本 |

请将 `cacheTtl` 写在 `experimental` 映射内，而不是 frontmatter 的顶层。

```yaml theme={null}
---
name: repo-auditor
description: Audits a large repository and reports what it finds
experimental:
  cacheTtl: 1h
---
```

#### Claude Code 跳过的子代理文件

当项目、用户或托管 `agents` 目录中的文件，或您通过 `--add-dir` 添加的目录下的 `agents` 目录中的文件的 frontmatter 存在以下任一问题时，Claude Code 会跳过该文件，且不会在会话中报告：

* **没有 `name`**：Claude Code 将该文件视为与您的 Agent 放在一起的文档。
* **开头的 `---` 不在文件第一行**：Claude Code 认为该文件没有 frontmatter，并将其视为文档。
* **`name` 以 `-` 开头、包含 `:` 或超过 256 个字符**：Claude Code 跳过该文件，并将错误写入调试日志。
* **有 `name` 但没有 `description`**：Claude Code 跳过该文件，并将原因写入调试日志。
* **YAML 无法解析**：Claude Code 不从该文件读取任何字段，跳过它，并将解析错误写入调试日志。

若要查看调试日志，请使用 `--debug` 运行 Claude Code。

如果[插件子代理](https://code.claude.com/docs/zh-CN/plugins/components.md#agents)的 frontmatter 没有 `name` 或无法解析，它仍会以其文件名加载。

##### 在会话开始前检查 `agents` 目录

若要查找 `agents` 目录中 frontmatter 无法解析的文件，请针对该目录（例如 `.claude/agents` 或 `~/.claude/agents`）运行 `claude plugin validate`。Claude Code 只检查[您指定的目录](https://code.claude.com/docs/zh-CN/plugins/cli-reference.md#validate-a-directory)，并且不会标记 frontmatter 可解析但没有 `name` 的文件。需要 Claude Code v2.1.233 或更高版本。

### 选择模型

`model` 字段控制子代理使用哪个模型：

* **模型别名**：使用可用别名之一：`sonnet`、`opus`、`haiku` 或 `fable`
* **完整模型 ID**：使用完整模型 ID，例如 `claude-opus-5-5` 或 `claude-sonnet-5`。接受与 `--model` 标志相同的值
* **inherit**：使用与主对话相同的模型

当 Claude 调用子代理时，它还可以为该次调用传入 `model` 参数。在 Agent 工具调用中，该参数接受模型别名：`sonnet`、`opus`、`haiku` 或 `fable`。Claude 无法在该调用中传入完整模型 ID 或 `inherit`。Claude Code 按以下顺序确定子代理的模型：

1. 单次调用的 `model` 参数
2. 子代理定义中的 `model` frontmatter，其中 `inherit` 选择主对话的模型
3. [`CLAUDE_CODE_SUBAGENT_MODEL`](https://code.claude.com/docs/zh-CN/model-config#environment-variables) 环境变量（当您将其设置为模型别名或模型 ID 时）
4. 主对话的模型

如果已安装的 [mod](https://code.claude.com/docs/zh-CN/plugins/mods/overview.md) 在其 [`agent.spawn`](https://code.claude.com/docs/zh-CN/plugins/mods/reference#subagents) hook 中设置了模型，Claude Code 会使用该模型来代替单次调用的参数。

在两种情况下，单次调用参数或 frontmatter 中的系列别名（如 `opus`）会解析为主对话的模型，而不是[该别名指向的版本](https://code.claude.com/docs/zh-CN/model-config.md#model-aliases)：

* **主对话的模型属于该系列**：子代理在主对话的确切模型上运行，包括任何 `[1m]` 后缀，因此它获得与主对话相同的[扩展上下文](https://code.claude.com/docs/zh-CN/model-config.md#extended-context)窗口。
* **在 [Anthropic API 以外的提供商](https://code.claude.com/docs/zh-CN/third-party-integrations.md)上，Claude Code 无法判断主对话的模型系列**：这可能发生在 Amazon Bedrock 上使用 Claude Code 尚未解析到后端模型的[应用推理配置文件 ARN](https://code.claude.com/docs/zh-CN/amazon-bedrock.md#iam-configuration) 时。这种情况仅涵盖 `opus` 别名，并且在您设置了 [`ANTHROPIC_DEFAULT_OPUS_MODEL`](https://code.claude.com/docs/zh-CN/model-config#environment-variables) 时不适用，因为此时 `opus` 会解析为您设置的模型。

`CLAUDE_CODE_SUBAGENT_MODEL` 中的别名始终解析为该别名指向的版本，即使它指向主对话的系列。将该变量设置为 `inherit` 等同于不设置它。

仅设置 `CLAUDE_CODE_SUBAGENT_MODEL` 不会改变内置 Explore 和 Plan 子代理所运行的模型。若要更改，请参阅[让所有子代理在同一模型上运行](#run-every-subagent-on-one-model)。

在 v2.1.251 之前，`CLAUDE_CODE_SUBAGENT_MODEL` 在此顺序中排在首位，会覆盖单次调用参数和 frontmatter，包括 `model: inherit`。

Claude Code 会根据您组织的 [`availableModels`](https://code.claude.com/docs/zh-CN/model-config#restrict-model-selection) 允许列表检查单次调用参数、frontmatter 和环境变量的值。对于被阻止的值，它会替换为另一个模型：

* 当被阻止的值是系列别名（如 `opus`）时，Claude Code 会在允许列表允许的该系列最新版本上运行子代理，遵循与 `/model` 相同的[替换规则和提供商范围](https://code.claude.com/docs/zh-CN/model-config.md#restrict-model-selection)。在 v2.1.222 之前，对于被阻止的系列别名，Claude Code 同样会在继承的模型上运行子代理。
* 对于任何其他被阻止的值、在该替换不起作用的提供商上，或者当允许列表不允许该系列的任何版本时，Claude Code 会改为在继承的模型上运行子代理。如果您设置了 `CLAUDE_CODE_SUBAGENT_MODEL`，Claude Code 会按照同样的规则首先尝试该模型。

在交互式会话中，对于上述任一种替换，Claude Code 都会显示一条警告，列出请求的模型和子代理实际运行的模型。

若要查看子代理正在哪个模型上运行，请运行 [`/tasks`](https://code.claude.com/docs/zh-CN/commands)。Claude Code 会在子代理所在行显示模型名称，并附上为该子代理设置的 [effort 级别](https://code.claude.com/docs/zh-CN/model-config.md#adjust-effort-level)（如果有）。需要 Claude Code v2.1.242 或更高版本。

当子代理被[恢复或收到后续消息](#resume-subagents)时，单次调用的 `model` 参数同样适用，因此子代理会保持在该模型上。

从 v2.1.198 开始，子代理还会继承主对话的[扩展思考](https://code.claude.com/docs/zh-CN/model-config.md#extended-thinking)配置：如果您的会话中启用了思考，子代理也会启用；如果关闭，它也保持关闭。没有针对单个子代理的思考设置。在 v2.1.198 之前，无论主对话的设置如何，子代理都在禁用扩展思考的情况下运行。

#### 让所有子代理在同一模型上运行

`CLAUDE_CODE_SUBAGENT_MODEL` 是一个默认值，因此子代理的定义或 Claude 传入的模型仍优先于它。若要将同一模型应用于每个子代理、[队友](https://code.claude.com/docs/zh-CN/agent-teams.md#specify-teammates-and-models)和[工作流 Agent](https://code.claude.com/docs/zh-CN/workflows.md)，还需将 `CLAUDE_CODE_SUBAGENT_MODEL_FORCE` 设置为 `1`。需要 Claude Code v2.1.257 或更高版本。

* 如果同时设置了这两个变量，子代理会在 `CLAUDE_CODE_SUBAGENT_MODEL` 中的模型上运行。
* 如果只设置了 `CLAUDE_CODE_SUBAGENT_MODEL_FORCE`，子代理会在主对话的模型上运行，但内置 Explore 子代理除外，它在[内置子代理下为其列出的模型](#built-in-subagents)上运行。

例如，若要让所有子代理都在 Haiku 上运行，请在[设置文件](https://code.claude.com/docs/zh-CN/settings.md)的 `env` 块中同时设置这两个变量：

```json theme={null}
{
  "env": {
    "CLAUDE_CODE_SUBAGENT_MODEL": "haiku",
    "CLAUDE_CODE_SUBAGENT_MODEL_FORCE": "1"
  }
}
```

若要检查设置是否生效，请在子代理运行时运行 [`/tasks`](https://code.claude.com/docs/zh-CN/commands)。子代理所在行会显示它所运行的模型。

当 `CLAUDE_CODE_SUBAGENT_MODEL_FORCE` [启用](https://code.claude.com/docs/zh-CN/env-vars.md)时，Claude Code 会忽略子代理定义中的 `model` 字段，并且 Claude 在启动子代理时无法传入模型。以下子代理仍在主对话的模型上运行：

* [分叉](#fork-the-current-conversation)
* 设置了 `model: inherit` 的[在子代理中运行的 skill](https://code.claude.com/docs/zh-CN/skills.md#run-skills-in-a-subagent)

### 选择 effort 级别

若要让子代理以其自己的 [effort 级别](https://code.claude.com/docs/zh-CN/model-config.md#adjust-effort-level)运行，请在其定义中设置 [`effort`](#supported-frontmatter-fields) 字段。

当您要求 Claude 以特定 effort 级别运行非分叉子代理时，它还可以为该次调用传入 `effort` 参数。该参数会覆盖 `effort` 字段，并在子代理被[恢复](#resume-subagents)时保持有效。[`CLAUDE_CODE_EFFORT_LEVEL`](https://code.claude.com/docs/zh-CN/env-vars#variables) 环境变量优先于这两者。单次调用参数需要 Claude Code v2.1.292 或更高版本。

### 控制子代理的能力

您可以通过工具访问、权限模式和条件规则来控制子代理能做什么。

#### 可用工具

子代理继承主对话中可用的[内置工具](https://code.claude.com/docs/zh-CN/tools-reference.md)和 MCP 工具，并经过两个过滤器的缩减：第一个过滤器从每个子代理中移除少量工具，第二个过滤器为在[后台](#run-subagents-in-foreground-or-background)运行的子代理（默认情况）缩减内置工具集。在 macOS、Linux 和 WSL 上，当主对话没有 Glob 和 Grep 工具时，子代理也可以获得这些工具，如 [Glob 工具行为](https://code.claude.com/docs/zh-CN/tools-reference.md#glob-tool-behavior)中所述。[分叉](#fork-the-current-conversation)会跳过这两个过滤器，获得与主对话完全相同的工具池。第一个过滤器会移除以下工具，即使它们列在 `tools` 字段中：

* `Agent`，当子代理达到[深度限制](#let-subagents-spawn-their-own-subagents)时；在[分叉](#fork-the-current-conversation)中，该工具仍会列出，但会返回错误而不是生成子代理
* `AskUserQuestion`
* `EndConversation`，它只能结束主对话；请参阅 [EndConversation 工具行为](https://code.claude.com/docs/zh-CN/tools-reference.md#endconversation-tool-behavior)
* `EnterPlanMode`
* `ExitPlanMode`，除非子代理的 [`permissionMode`](#permission-modes) 为 `plan`
* `ScheduleWakeup`
* `WaitForMcpServers`
* `Workflow`

第二个过滤器适用于在后台运行的子代理。除了 `Agent` 和 `ExitPlanMode`（无论子代理在何处运行，都遵循第一个过滤器的条件）之外，后台子代理保留所有 MCP 工具，但只保留以下内置工具：`Read`、`Grep`、`Glob`、`LSP`、`Bash`、`PowerShell`、`Edit`、`Write`、`NotebookEdit`、`WebFetch`、`WebSearch`、`TodoWrite`、`Skill`、`ToolSearch`、`EnterWorktree`、`ExitWorktree`、`Monitor`、`TaskStop`、`SendMessage` 和 `Artifact`，对于通过 [`SubagentHandback`](https://code.claude.com/docs/zh-CN/tools-reference) 报告的子代理，还保留该工具。Claude Code 会从后台子代理中移除所有其他内置工具，无论是继承的还是列在 `tools` 字段中的，因此同一定义在前台和后台可能解析为不同的工具。除非移除后 `tools` 列表[解析结果为空](https://code.claude.com/docs/zh-CN/errors.md#agent-would-be-spawned-with-zero-tools)，否则移除操作不会报告错误。

在 v2.1.280 之前，后台子代理无法使用 `LSP`。

[`ListAgents`](https://code.claude.com/docs/zh-CN/cross-session-messaging) 与任何内置工具一样遵循这些过滤器：在启用了跨会话消息的会话中，前台子代理会继承它，而后台子代理不会保留它。

[agent team](https://code.claude.com/docs/zh-CN/agent-teams.md) 中的队友还会额外保留任务工具和 cron 工具：`TaskCreate`、`TaskGet`、`TaskList`、`TaskUpdate`、`CronCreate`、`CronDelete` 和 `CronList`。在[没有 Task 工具的会话](https://code.claude.com/docs/zh-CN/tools-reference.md#task-tool-availability)中，Claude Code 也不会向子代理提供任务工具，即使子代理运行的是不同的模型。进程内队友同样遵循您的会话。若要了解分割窗格中的队友是否拥有任务工具，请参阅 [Task 工具可用性](https://code.claude.com/docs/zh-CN/tools-reference.md#task-tool-availability)。

#### 限制子代理可以使用的工具

若要限制工具，请将 `tools` 字段用作允许列表，或将 `disallowedTools` 字段用作拒绝列表。如果两者都设置，会先应用 `disallowedTools`，然后根据剩余的工具池解析 `tools`。同时列在两者中的工具会被移除。

此示例使用 `tools` 只允许 Read、Grep、Glob 和 Bash。该子代理无法编辑文件、写入文件或使用任何 MCP 工具：

```yaml theme={null}
---
name: safe-researcher
description: Research agent with restricted capabilities
tools: Read, Grep, Glob, Bash
---
```

此示例使用 `disallowedTools` 继承子代理除 Write 和 Edit 之外的工具池。该子代理保留 Bash、MCP 工具及其工具池中的其余工具：

```yaml theme={null}
---
name: no-writes
description: Inherits the available tools except file writes
disallowedTools: Write, Edit
---
```

当 `tools` 列表中没有任何条目能解析为工具时（例如每个条目都拼写错误，或指向子代理不可用的工具），Claude Code 通常会拒绝启动子代理，并且 Agent 工具会返回一个列出未解析条目的错误；有关该消息以及如何修复每个条目，请参阅 [Agent would be spawned with zero tools](https://code.claude.com/docs/zh-CN/errors.md#agent-would-be-spawned-with-zero-tools)。

`tools` 和 `disallowedTools` 字段还接受针对整个 MCP 服务器的条目：

| 目标 | 字段 | 示例条目 |
| :- | :- | :- |
| 为子代理提供某个服务器的所有工具 | `tools` | `mcp__github__*` |
| 排除某个服务器的工具 | `disallowedTools` | `mcp__github` 或 `mcp__github__*` |
| 排除所有服务器的所有 MCP 工具 | `disallowedTools` | `mcp__*` |

此示例从子代理的工具池中移除 `github` MCP 服务器的所有工具，同时保留其他服务器的工具和内置工具：

```yaml theme={null}
---
name: local-only
description: Inherits every tool except those from the github MCP server
disallowedTools: mcp__github
---
```

带有说明符的 `disallowedTools` 条目（如 `Bash(git push *)`）仍会从子代理中移除整个工具，而不仅仅是匹配的命令。若要保留 Bash 并阻止特定命令，请在设置中向 `permissions.deny` 添加 [Bash 拒绝规则](https://code.claude.com/docs/zh-CN/permissions.md#bash)，例如 `Bash(git push *)`。该规则适用于主对话和子代理。

##### 名称包含双下划线的 MCP 服务器

如果子代理无法获得名称包含 `__` 的 MCP 服务器（例如 `reports__v2`）中的任何工具，请检查该子代理的 `mcp__` 条目：

* **在 `tools` 中**：写入 `mcp__`，后跟[服务器的完整名称](https://code.claude.com/docs/zh-CN/permissions.md#mcp)和 `__*`，如 `mcp__reports__v2__*`。如果没有末尾的 `__*`，`mcp__reports__v2` 不会匹配该服务器的任何工具。
* **在 `disallowedTools` 中**：查找 `mcp__reports` 或 `mcp__reports__*`，这些条目会排除名为 `reports` 的服务器。使用其中任一条目时，子代理也会失去所有 `reports__v2` 工具，因为这些工具名称同样以 `mcp__reports__` 开头。若只想排除 `reports` 服务器的工具，请改为逐一列出它们，例如 `mcp__reports__run_query`。

#### 限制可以生成的子代理

当 Agent 通过 `claude --agent` 作为主线程运行时，它可以使用 Agent 工具生成子代理。若要限制它可以生成的子代理类型，请在 `tools` 字段中使用 `Agent(agent_type)` 语法。

> [!NOTE]
>
> 在 2.1.63 版本中，Task 工具被重命名为 Agent。设置和 Agent 定义中现有的 `Task(...)` 引用仍可作为别名使用。

```yaml theme={null}
---
name: coordinator
description: Coordinates work across specialized agents
tools: Agent(worker, researcher), Read, Bash
---
```

这是一个允许列表：只能生成 `worker` 和 `researcher` 子代理。如果 Agent 尝试生成任何其他类型，请求会失败，并且 Agent 在其提示词中只能看到允许的类型。若要阻止特定 Agent 同时允许所有其他 Agent，请改用 [`permissions.deny`](#disable-specific-subagents)。

若要允许不受限制地生成任何子代理，请使用不带括号的 `Agent`：

```yaml theme={null}
tools: Agent, Read, Bash
```

如果您在 `tools` 列表中完全省略 `Agent`，该 Agent 将无法使用 Agent 工具生成任何子代理。

`Agent(agent_type)` 允许列表语法仅适用于通过 `claude --agent` 作为主线程运行的 Agent。在子代理定义中，在 `tools` 中列出 `Agent` 可让该子代理在[深度限制](#let-subagents-spawn-their-own-subagents)允许的范围内生成它自己的子代理，但括号内的任何类型列表都会被忽略。

#### 将 MCP 服务器限定于子代理

使用 `mcpServers` 字段为子代理提供对主对话中不可用的 [MCP](https://code.claude.com/docs/zh-CN/mcp.md) 服务器的访问权限。此处定义的内联服务器在子代理启动时连接（需符合[针对 Agent 文件所在文件夹的信任规则](#inline-server-trust)），并在其完成时断开连接。字符串引用共享父会话的连接。

> [!NOTE]
>
> `mcpServers` 字段在 Agent 文件可以运行的两种情境中都适用：
>
> * 作为子代理，通过 Agent 工具或 @ 提及生成
> * 作为主会话，通过 [`--agent`](#invoke-subagents-explicitly) 或 `agent` 设置启动
>
> 当 Agent 是主会话时，内联服务器定义会在启动时与来自 [`.mcp.json`](https://code.claude.com/docs/zh-CN/mcp) 和设置文件的服务器一起连接，遵循相同的[针对 Agent 文件所在文件夹的信任规则](#inline-server-trust)。在 `/mcp` 中，您以前使用过的远程（HTTP 或 SSE）服务器可能会改为显示 [`cached` 状态](https://code.claude.com/docs/zh-CN/mcp#managing-your-servers)；Claude Code 会在 Claude 首次调用其某个工具时连接它。

列表中的每个条目要么是内联服务器定义，要么是引用您会话中已配置的 MCP 服务器的字符串：

```yaml theme={null}
---
name: browser-tester
description: Tests features in a real browser using Playwright
mcpServers:
  # Inline definition: scoped to this subagent only
  - playwright:
      type: stdio
      command: npx
      args: ["-y", "@playwright/mcp@latest"]
  # Reference by name: reuses an already-configured server
  - github
---

Use the Playwright tools to navigate, screenshot, and interact with pages.
```

内联定义使用与 `.mcp.json` 服务器条目相同的 schema，以服务器名称为键，并支持 `stdio`、`http`、`sse` 和 `ws` 类型。

若要将某个 MCP 服务器完全排除在主对话之外，避免其工具描述占用主对话的上下文，请在此处以内联方式定义它，而不是在 `.mcp.json` 中定义。子代理会获得这些工具；父对话则不会。

适用于主会话的 MCP 限制同样覆盖在子代理 frontmatter 中声明的服务器：

* [`--strict-mcp-config`](https://code.claude.com/docs/zh-CN/cli-reference) 和 [`--bare`](https://code.claude.com/docs/zh-CN/cli-reference)
* [企业托管 MCP 配置](https://code.claude.com/docs/zh-CN/managed-mcp.md)
* [`allowedMcpServers` 和 `deniedMcpServers` 策略](https://code.claude.com/docs/zh-CN/managed-mcp#policy-based-control-with-allowlists-and-denylists)

当其中之一阻止某个服务器时，Claude Code 会跳过它，并显示一条列出被阻止服务器的警告。

托管设置的限制适用于每个子代理，无论其定义方式如何。`--strict-mcp-config` 不会过滤您通过 `--agents` 或 SDK `agents` 选项以内联方式传入的服务器，因为这些属于调用方的明确输入。

#### 内联 MCP 服务器所需的信任

对于您项目的 `.claude/agents/` 目录或 `--add-dir` 目录的 `.claude/agents/` 中的 Agent 文件，Claude Code 只有在您[信任该 Agent 文件所在的文件夹](https://code.claude.com/docs/zh-CN/permissions.md#what-runs-before-you-trust-a-folder)之后，才会加载其中的[内联 MCP 服务器](#scope-mcp-servers-to-a-subagent)。在 v2.1.238 之前，Claude Code 加载这些服务器时不检查信任。

* **不算数的信任**：父文件夹的信任，以及 `-p` 或 SDK 会话针对[设置文件中的 hook](https://code.claude.com/docs/zh-CN/permissions.md#what-runs-before-you-trust-a-folder) 获得的自动信任
* **在此之前**：Claude Code 会跳过该 Agent 文件中的每个内联服务器，并将 `~/.claude.json` 中确切的 `projects["<path>"].hasTrustDialogAccepted` 键写入调试日志
* **`--add-dir` 目录**：位于您受信任工作区仓库之外的目录需要其自己的信任条目，因为其 `.claude/agents/` 文件不会继承您工作区的信任

对于以下两种服务器，Claude Code 加载时不检查 Agent 文件所在文件夹的信任：

* 引用您已配置的服务器的名称
* 来自 `~/.claude/agents/` 中的 Agent 文件、您通过 `--agents` 或 SDK `agents` 选项传入的 Agent 文件，或由托管设置提供的 Agent 文件中的内联服务器

#### 权限模式

设置 `permissionMode` 以选择子代理运行时使用的权限模式。请使用模式的配置值，因此 Manual 模式为 `default`。如果不设置，子代理会继承主对话的[权限模式](https://code.claude.com/docs/zh-CN/permission-modes.md)。

主对话的权限模式决定 Claude Code 是否使用您设置的值：

* 当主对话处于 `bypassPermissions`、`acceptEdits` 或[自动模式](https://code.claude.com/docs/zh-CN/permission-modes.md#eliminate-prompts-with-auto-mode)时，子代理以相同模式运行，Claude Code 会忽略您设置的 `permissionMode`。在自动模式下，分类器使用主对话的阻止和允许规则评估子代理的工具调用。当子代理完成时，分类器还会在交付最终报告之前审查其工作和最终报告，如[自动模式如何处理子代理](https://code.claude.com/docs/zh-CN/permission-modes.md#eliminate-prompts-with-auto-mode)中所述。
* 当主对话处于 `plan` 模式时，无论您设置了什么 `permissionMode`，子代理都会保持在计划模式，除非它作为 [agent team 队友](#subagent-names)按照 [agent team 规则](https://code.claude.com/docs/zh-CN/agent-teams.md#have-teammates-plan-before-implementing)运行。如果您设置了 `dontAsk`，子代理也不会提示您：计划模式下原本会请求您批准的操作将被拒绝。
* 当主对话处于 `default` 或 `dontAsk` 模式时，子代理以您设置的权限模式运行。在以下情况下，它会改为保持主对话的权限模式：
  * 您设置了 `bypassPermissions`。`bypassPermissions` 例外需要 Claude Code v2.1.267 或更高版本。
  * 您设置了 `auto`，但子代理[无法使用自动模式](https://code.claude.com/docs/zh-CN/permission-modes.md#eliminate-prompts-with-auto-mode)，例如设置文件设置了 [`disableAutoMode`](https://code.claude.com/docs/zh-CN/settings-reference#disableautomode)，或子代理的模型不支持自动模式。

`permissionMode` 接受以下值，以及作为 `default` 别名的 `manual`：

| 模式 | 行为 |
| :- | :- |
| `default` | Manual 模式：提示请求权限 |
| `acceptEdits` | 对于工作目录或 `additionalDirectories` 中的路径，自动接受文件编辑和常见文件系统命令 |
| `auto` | [自动模式](https://code.claude.com/docs/zh-CN/permission-modes.md#eliminate-prompts-with-auto-mode)：后台分类器审查命令和对受保护目录的写入 |
| `dontAsk` | 自动拒绝会触发提示的调用，但您安装的 [mod](https://code.claude.com/docs/zh-CN/plugins/mods/overview.md) 仍可批准[其中部分调用](https://code.claude.com/docs/zh-CN/permissions.md#precedence-among-rules-hooks-and-mods)。明确允许的工具仍可使用。`AskUserQuestion`、标记为 [`requiresUserInteraction`](https://code.claude.com/docs/zh-CN/mcp#require-approval-for-a-specific-tool) 的 MCP 工具、[从网络路径读取](https://code.claude.com/docs/zh-CN/permissions.md#network-paths)，以及在该设置到达 Claude Code 的会话中[您的组织设置为 `ask`](https://code.claude.com/docs/zh-CN/mcp#organization-controls-on-connector-tools) 的连接器工具，即使您已允许也会被拒绝 |
| `bypassPermissions` | [跳过权限提示](https://code.claude.com/docs/zh-CN/permission-modes.md#skip-all-checks-with-bypasspermissions-mode)。只有当主对话处于此模式时，子代理才会以此模式运行 |
| `plan` | 计划模式（只读探索） |

#### 将 skill 预加载到子代理中

使用 `skills` 字段在启动时将 skill 内容注入子代理的上下文。这样子代理无需在执行期间发现和加载 skill 即可获得领域知识。

```yaml theme={null}
---
name: api-developer
description: Implement API endpoints following team conventions
skills:
  - api-conventions
  - error-handling-patterns
---

Implement API endpoints. Follow the conventions and patterns from the preloaded skills.
```

每个列出的 skill 的完整内容会在启动时注入子代理的上下文，最多为列表中前 32 个不同的名称。此字段控制预加载哪些 skill，而不是子代理可以访问哪些 skill：即使没有此字段，子代理在执行期间仍可通过 Skill 工具发现和调用项目、用户和插件 skill。若要完全阻止子代理调用 skill，请从 [`tools`](#available-tools) 列表中省略 `Skill`，或将其添加到 `disallowedTools`。

您无法预加载设置了 [`disable-model-invocation: true`](https://code.claude.com/docs/zh-CN/skills#control-who-invokes-a-skill) 的 skill，因为预加载取自 Claude 可以调用的同一组 skill。这包括内置的 `/verify` skill，Claude 无法自行运行它。

如果列出的某个 skill 缺失或被禁用（例如被您组织的策略禁用），Claude Code 会跳过它，并在调试日志中记录一条警告。

> [!NOTE]
>
> 这与[在子代理中运行 skill](https://code.claude.com/docs/zh-CN/skills.md#run-skills-in-a-subagent) 相反。在子代理中使用 `skills` 时，由子代理控制系统提示词并加载 skill 内容。在 skill 中使用 `context: fork` 时，skill 内容会被注入到您指定的 Agent 中。在这两种情况下，子代理启动时都不带有您的对话历史。

#### 启用持久记忆

`memory` 字段为子代理提供一个跨对话保留的持久目录。子代理使用此目录随时间积累知识，例如代码库模式、调试心得和架构决策。

```yaml theme={null}
---
name: code-reviewer
description: Reviews code for quality and best practices
memory: user
---

You are a code reviewer. As you review code, update your agent memory with
patterns, conventions, and recurring issues you discover.
```

根据记忆应适用的范围选择作用域：

| 作用域 | 位置 | 适用场景 |
| :- | :- | :- |
| `user` | `~/.claude/agent-memory/<name-of-agent>/` | 子代理应在所有项目中记住所学内容 |
| `project` | `.claude/agent-memory/<name-of-agent>/` | 子代理的知识特定于项目，并可通过版本控制共享 |
| `local` | `.claude/agent-memory-local/<name-of-agent>/` | 子代理的知识特定于项目，但不应提交到版本控制 |

子代理记忆是[自动记忆](https://code.claude.com/docs/zh-CN/memory.md#auto-memory)的一部分：如果您通过 `autoMemoryEnabled` 设置或 `CLAUDE_CODE_DISABLE_AUTO_MEMORY` 关闭自动记忆，`memory` 字段将不起作用，子代理启动时不会带有下述记忆指令或记忆工具访问权限。

启用记忆后：

* 子代理的系统提示词包含读取和写入记忆目录的指令。
* 子代理的系统提示词还包含记忆目录中 `MEMORY.md` 的前 200 行或前 25KB（以先达到者为准），并附有在 `MEMORY.md` 超出该限制时对其进行整理的指令。
* Read、Write 和 Edit 工具会自动启用，以便子代理管理其记忆文件。

##### 持久记忆提示

* `project` 是推荐的默认作用域。它使子代理的知识可以通过版本控制共享。
* 让子代理在开始工作前查阅其记忆："Review this PR, and check your memory for patterns you've seen before."
* 让子代理在完成任务后更新其记忆："Now that you're done, save what you learned to your memory." 随着时间推移，这会建立一个让子代理更高效的知识库。
* 直接在子代理的 markdown 文件中加入记忆指令，使其主动维护自己的知识库：

  ```markdown theme={null}
  Update your agent memory as you discover codepaths, patterns, library
  locations, and key architectural decisions. This builds up institutional
  knowledge across conversations. Write concise notes about what you found
  and where.
  ```

#### 使用 hook 的条件规则

若要对工具使用进行更动态的控制，请使用 `PreToolUse` hook 在操作执行前对其进行验证。当您需要允许某个工具的部分操作同时阻止其他操作时，这非常有用。

此示例创建一个只允许只读数据库查询的子代理。`PreToolUse` hook 会在每个 Bash 命令执行前运行 `command` 中指定的脚本：

```yaml theme={null}
---
name: db-reader
description: Execute read-only database queries
tools: Bash
hooks:
  PreToolUse:
    - matcher: "Bash"
      hooks:
        - type: command
          command: "./scripts/validate-readonly-query.sh"
---
```

Claude Code 通过 stdin [以 JSON 形式将 hook 输入传递](https://code.claude.com/docs/zh-CN/hooks.md#pretooluse-input)给 hook 命令。验证脚本读取此 JSON，提取 Bash 命令，并[以退出码 2 退出](https://code.claude.com/docs/zh-CN/hooks.md#exit-code-2-behavior-per-event)以阻止写入操作：

```bash theme={null}
#!/bin/bash
# ./scripts/validate-readonly-query.sh

INPUT=$(cat)
COMMAND=$(echo "$INPUT" | jq -r '.tool_input.command // empty')

# Block SQL write operations (case-insensitive)
if echo "$COMMAND" | grep -iE '\b(INSERT|UPDATE|DELETE|DROP|CREATE|ALTER|TRUNCATE)\b' > /dev/null; then
  echo "Blocked: Only SELECT queries are allowed" >&2
  exit 2
fi

exit 0
```

在 macOS 和 Linux 上，请将脚本设为可执行，否则 hook 会失败而不是阻止任何操作：

```bash theme={null}
chmod +x ./scripts/validate-readonly-query.sh
```

若要测试该规则，请让子代理运行一条 `UPDATE` 语句：脚本以退出码 2 退出，Claude Code 阻止该命令，子代理会看到 `Blocked: Only SELECT queries are allowed` 消息。

有关完整的输入 schema，请参阅 [Hook 输入](https://code.claude.com/docs/zh-CN/hooks.md#pretooluse-input)；有关退出码如何影响行为，请参阅[退出码](https://code.claude.com/docs/zh-CN/hooks.md#exit-code-output)。在 Windows 上，请使用 PowerShell 编写 hook 脚本，并按照[在 PowerShell 中运行 hook](https://code.claude.com/docs/zh-CN/hooks.md#windows-powershell-tool) 中所示，向 hook 条目添加 `shell: powershell`。

#### 禁用特定子代理

您可以将特定子代理添加到[设置](https://code.claude.com/docs/zh-CN/settings-reference.md#permission-settings)中的 `deny` 数组，以阻止 Claude 使用它们。使用 `Agent(subagent-name)` 格式，其中 `subagent-name` 与子代理的 name 字段匹配。

```json theme={null}
{
  "permissions": {
    "deny": ["Agent(Explore)", "Agent(my-custom-agent)"]
  }
}
```

这同时适用于内置子代理和自定义子代理。您也可以使用 `--disallowedTools` CLI 标志：

```bash theme={null}
claude --disallowedTools "Agent(Explore)"
```

有关权限规则的更多详细信息，请参阅[权限文档](https://code.claude.com/docs/zh-CN/permissions.md#tool-specific-permission-rules)。

### 为子代理定义 hook

子代理可以定义在子代理生命周期中运行的 [hook](https://code.claude.com/docs/zh-CN/hooks.md)。配置 hook 有两种方式：

* **在子代理的 frontmatter 中**：定义仅在该子代理活动期间运行的 hook
* **在 `settings.json` 中**：定义也会在子代理内部触发的会话级 hook。`PreToolUse` 和 `PostToolUse` 等工具事件会像在主对话中一样针对子代理的工具调用触发，而 `SubagentStart` 和 `SubagentStop` 会在子代理启动或完成时触发

来自[设置文件、托管策略设置和插件](https://code.claude.com/docs/zh-CN/hooks.md#hook-locations)的 hook 都会在子代理内部生效，因此 `settings.json` 中的 `PreToolUse` hook 也会在子代理使用每个工具之前运行。

#### 子代理 frontmatter 中的 hook

直接在子代理的 markdown 文件中定义 hook。这些 hook 仅在该特定子代理活动期间运行，并在其完成时被清理。

> [!NOTE]
>
> 当 Agent 通过 Agent 工具或 @ 提及作为子代理生成时，以及当 Agent 通过 [`--agent`](#invoke-subagents-explicitly) 或 `agent` 设置作为主会话运行时，frontmatter hook 都会触发。在主会话的情况下，它们会与 [`settings.json`](https://code.claude.com/docs/zh-CN/hooks) 中定义的任何 hook 一起运行。

若要让项目级子代理的 frontmatter hook 运行，请为包含该 Agent 文件的文件夹接受[工作区信任对话框](https://code.claude.com/docs/zh-CN/permissions.md#project-allow-rules-and-workspace-trust)。来自 `~/.claude/agents/` 中用户级子代理以及您通过 `--agents` 传入的定义的 hook 无需此步骤即可运行。如果您通过 `--add-dir` 添加了位于受信任工作区仓库之外的文件夹，请单独信任该文件夹：其 `.claude/agents/` 中的 hook 不会继承工作区的授权。

在您信任该文件夹之前，子代理仍会运行，但 Claude Code 会跳过其 frontmatter hook，并在调试日志中记录一条说明如何信任该文件夹的错误。这一规则比设置文件中 hook 的规则更严格：信任父文件夹是不够的，并且 `-p` 会话不算作受信任。[信任文件夹之前会运行什么](https://code.claude.com/docs/zh-CN/permissions.md#what-runs-before-you-trust-a-folder)对这两者进行了比较。

支持所有 [hook 事件](https://code.claude.com/docs/zh-CN/hooks.md#hook-events)。子代理最常用的事件有：

| 事件 | 匹配器输入 | 触发时机 |
| :- | :- | :- |
| `PreToolUse` | 工具名称 | 子代理使用工具之前 |
| `PostToolUse` | 工具名称 | 子代理使用工具之后 |
| `Stop` | （无） | 子代理完成时（在运行时转换为 `SubagentStop`） |

此示例使用 `PreToolUse` hook 验证 Bash 命令，并使用 `PostToolUse` 在文件编辑后运行 linter：

```yaml theme={null}
---
name: code-reviewer
description: Review code changes with automatic linting
hooks:
  PreToolUse:
    - matcher: "Bash"
      hooks:
        - type: command
          command: "./scripts/validate-command.sh"
  PostToolUse:
    - matcher: "Edit|Write"
      hooks:
        - type: command
          command: "./scripts/run-linter.sh"
---
```

当 Agent 作为子代理被调用时，frontmatter 中的 `Stop` hook 会自动转换为 `SubagentStop` 事件。

#### 针对子代理事件的项目级 hook

在 `settings.json` 中配置 hook，以响应主会话中的子代理生命周期事件。

| 事件 | 匹配器输入 | 触发时机 |
| :- | :- | :- |
| `SubagentStart` | Agent 类型名称 | 子代理开始执行时 |
| `SubagentStop` | Agent 类型名称 | 子代理完成时 |

这两个事件都支持匹配器，可按名称指定特定的 Agent 类型。对于项目级和用户级子代理，匹配器值是 Agent 的 frontmatter `name`；对于[插件子代理](https://code.claude.com/docs/zh-CN/plugins/components.md#agents)，则是插件作用域标识符，例如 `my-plugin:db-agent`。作用域名称包含冒号，因此会被作为[非锚定正则表达式](https://code.claude.com/docs/zh-CN/hooks.md#matcher-patterns)进行求值；请使用 `^` 和 `$` 对其进行锚定，如 `^my-plugin:db-agent$`，以仅匹配该 Agent。

此示例仅在 `db-agent` 子代理启动时运行设置脚本，并在任何子代理停止时运行清理脚本：

```json theme={null}
{
  "hooks": {
    "SubagentStart": [
      {
        "matcher": "db-agent",
        "hooks": [
          { "type": "command", "command": "./scripts/setup-db-connection.sh" }
        ]
      }
    ],
    "SubagentStop": [
      {
        "hooks": [
          { "type": "command", "command": "./scripts/cleanup-db-connection.sh" }
        ]
      }
    ]
  }
}
```

有关完整的 hook 配置格式，请参阅 [Hooks](https://code.claude.com/docs/zh-CN/hooks.md)。

## 使用子代理

### 理解自动委托

Claude 根据您请求中的任务描述、子代理配置中的 `description` 字段以及当前上下文自动委托任务。要鼓励主动委托，请在子代理的 description 字段中包含"use proactively"之类的短语。

请保持描述简洁：当您的子代理描述总和超过 [15,000 token 限制](https://code.claude.com/docs/zh-CN/errors.md#agent-descriptions-are-over-the-15000-token-limit)时，Claude Code 会显示启动警告，但仍会加载每个子代理。

如果子代理由[插件](https://code.claude.com/docs/zh-CN/plugins/overview.md)提供，您可以在一组贴近实际的提示词上衡量 Claude 委托给它的可靠程度，而无需逐个检查：[`claude plugin eval`](https://code.claude.com/docs/zh-CN/plugin-evals) 会在启用和不启用该插件的情况下分别运行每个提示词，并对结果进行评分。

### 显式调用子代理

当自动委托不够用时，您可以自行请求使用子代理。以下三种模式从一次性建议逐步升级到整个会话的默认设置：

* **自然语言**：在提示词中点名子代理；由 Claude 决定是否委托
* **@-mention**：保证该子代理为某一个任务运行
* **会话范围**：通过 `--agent` 标志或 `agent` 设置，让整个会话以该子代理的身份运行

使用自然语言时没有特殊语法。点名子代理，Claude 通常就会委托：

```text wrap theme={null}
Use the test-runner subagent to fix failing tests
Have the code-reviewer subagent look at my recent changes
```

**@-mention 子代理。** 输入 `@` 并从自动补全列表中选择子代理，方式与 @-mention 文件相同。这可以确保运行指定的子代理，而不是将选择权交给 Claude：

```text wrap theme={null}
@"code-reviewer (agent)" look at the auth changes
```

您的完整消息仍会发送给 Claude，由 Claude 根据您的要求为子代理编写任务提示词。@-mention 控制的是 Claude 调用哪个子代理，而不是子代理收到什么提示词。

已启用[插件](https://code.claude.com/docs/zh-CN/plugins/overview.md)提供的子代理会以其作用域名称显示在自动补全列表中，例如 `my-plugin:code-reviewer`；当插件[将 Agent 组织到子文件夹中](#choose-the-subagent-scope)时，则显示为 `my-plugin:review:security` 这样的名称。当前在会话中运行的已命名后台子代理也会出现在自动补全列表中，并在名称旁显示其状态。

您也可以不使用选择器而手动输入提及：本地子代理使用 `@agent-<name>`，插件子代理使用 `@agent-` 后跟作用域名称，例如 `@agent-my-plugin:code-reviewer`。输入这种形式时，自动补全列表显示的是匹配的文件而不是 Agent，但提交时该 Agent 提及仍会被正确解析。

**以子代理身份运行整个会话。** 传递 [`--agent <name>`](https://code.claude.com/docs/zh-CN/cli-reference) 可启动一个会话，让主线程本身采用该子代理的工具限制和模型：

```bash theme={null}
claude --agent code-reviewer
```

除非该 Agent 的[提示词为空](#choose-the-subagent-scope)，否则自定义子代理的系统提示词会完全替换默认的 Claude Code 系统提示词，效果与 [`--system-prompt`](https://code.claude.com/docs/zh-CN/cli-reference) 相同。`CLAUDE.md` 文件和项目记忆仍会通过正常的消息流加载，即使该 Agent 的定义设置了 [`omitClaudeMd`](#supported-frontmatter-fields) 也是如此。如果您使用 `-p` 以拥有自身提示词的自定义 Agent 身份运行会话，该会话启动时也不会包含 [Git 状态快照](https://code.claude.com/docs/zh-CN/settings-reference.md#includegitinstructions)。

Agent 名称会以 `@<name>` 的形式显示在启动标题中，方便您确认其已生效。

这适用于内置和自定义子代理，并且在恢复会话时该选择会保留：Claude Code 会随对话一起恢复该 Agent 的工具限制和模型。如果恢复时该 Agent 已不存在，会话将使用默认工具继续，并显示一条[指明该 Agent 的警告](https://code.claude.com/docs/zh-CN/errors.md#session-agent-no-longer-available)。关于这两种情况下的系统提示词，请参阅[已恢复对话中的系统提示词标志](https://code.claude.com/docs/zh-CN/cli-reference.md#system-prompt-flags-in-resumed-conversations)。

对于插件提供的子代理，您可以只传递 Agent 名称，Claude Code 会自行找到它：

```bash theme={null}
claude --agent security-reviewer
```

如果多个插件提供了同名的 Agent，请传递作用域名称以消除歧义：

```bash theme={null}
claude --agent my-plugin:security-reviewer
```

如果插件将 Agent 放在其 `agents/` 目录的子文件夹中，请在作用域名称中包含该子文件夹，例如 `claude --agent my-plugin:review:security`。

要使其成为项目中每个会话的默认值，请在 `.claude/settings.json` 中设置 `agent`：

```json theme={null}
{
  "agent": "code-reviewer"
}
```

如果两者同时存在，CLI 标志会覆盖该设置。

### 在前台或后台运行子代理

子代理可以在前台或后台运行：

* **前台子代理**会阻塞主对话直到完成。权限提示会在出现时传递给您。
* **后台子代理**在您继续工作时并发运行。当后台子代理遇到需要权限的工具调用时，Claude Code 会在您的主会话中显示该提示，并指明发出请求的子代理。批准即可让子代理继续；按 Esc 则拒绝该单个工具调用，而不会停止子代理。

#### 哪些子代理在后台运行

在未进行任何配置的情况下，Claude 从您的主对话生成的子代理会在后台启动。

* **`claude -p` 和 Agent SDK**：Claude 可以根据自己的判断或在您要求时等待。
* **交互式终端会话、VS Code 以及您本机上的桌面应用会话**：要让 Claude 等待子代理，请[关闭 fork 模式](#let-claude-wait-for-a-subagent)，该模式[默认开启](#turn-fork-mode-on-or-off)。

对于带有 `context: fork` 的 skill，Claude Code 改为遵循[在子代理中运行 skill](https://code.claude.com/docs/zh-CN/skills.md#run-skills-in-a-subagent) 中的规则，无论 fork 模式是否开启。

#### 让 Claude 等待子代理

1. 将 `CLAUDE_CODE_FORK_SUBAGENT` 设置为 `0` 以关闭 [fork 模式](#turn-fork-mode-on-or-off)，例如在 [`settings.json`](https://code.claude.com/docs/zh-CN/settings) 中：

   ```json theme={null}
   {
     "env": {
       "CLAUDE_CODE_FORK_SUBAGENT": "0"
     }
   }
   ```

2. 从 shell 中重新运行 `claude`，或打开一个新的 VS Code 对话。

3. 当您需要先拿到结果时，请让 Claude 等待，例如"run the reviewer subagent and wait for its report"。

要在 Claude 等待期间继续工作，请在终端中按 **Ctrl+B**，或在 VS Code 中点击 [**Run in background**](https://code.claude.com/docs/zh-CN/vs-code.md#move-a-running-command-or-subagent-to-the-background)。

如果您不关闭 fork 模式，而是将 [`CLAUDE_CODE_DISABLE_BACKGROUND_TASKS`](https://code.claude.com/docs/zh-CN/env-vars) 环境变量设置为 `1`，Claude 会等待子代理完成后再继续。后台命令也会被关闭，并且 **Ctrl+B** 不会将任何内容移到后台。

#### 后台子代理的工具、权限和结果

后台子代理使用的[内置工具集](#available-tools)比前台子代理更小，但对话 fork 和[已恢复](#resume-subagents)的前台子代理除外。

后台子代理会在您的主会话中显示每一个权限提示。当您对其中某个提示所作的选择会在该次工具调用之后继续生效（例如在会话剩余时间内持续有效的授予）时，Claude Code 会将您的答复应用于整个会话，包括您的主对话。

后台子代理可以让后台 [Bash 或 PowerShell 命令](https://code.claude.com/docs/zh-CN/tools-reference.md#background-commands)[在其轮次结束后继续运行](https://code.claude.com/docs/zh-CN/interactive-mode.md#how-backgrounding-works)。该命令结束时，Claude Code 会向该子代理发送通知。

后台子代理的结果会在之后的某个轮次中以完成通知的形式送达 Claude。Claude 会等待该通知后再报告子代理的结果；如果您先询问进度，它会告知子代理仍在运行。

#### 面板和任务列表中已结束的子代理

Claude Code 会以两种方式之一从输入框下方的子代理面板中清除后台子代理所在的行，具体取决于子代理的结束方式：

* 当子代理成功完成时，Claude Code 会立即移除其所在行，并在页脚显示 `/tasks to see subagents` 30 秒（[屏幕阅读器模式](https://code.claude.com/docs/zh-CN/accessibility.md)下除外）。在这 30 秒内，运行 [`/tasks`](https://code.claude.com/docs/zh-CN/commands) 并在该子代理上按 `Enter` 即可打开其会话记录。在 v2.1.232 之前，Claude Code 会在子代理完成后将该行保留 30 秒（与失败的子代理相同），且不显示页脚提示。
* 当子代理失败或被您停止时，Claude Code 会将其所在行保留 30 秒。要更早清除该行，请选中它并按 `x`。

已完成的后台子代理会继续列在 [`/tasks`](https://code.claude.com/docs/zh-CN/commands) 中，标记为已完成并排在正在运行的工作下方，持续时间与页脚提示相同，为 30 秒。子代理完成时，其详情视图保持打开。失败或被您停止的子代理会从列表中移除。在 v2.1.208 之前，已完成的子代理会在完成的那一刻离开列表，其详情视图也会关闭。

### 子代理名称

如果您在启用 [agent teams](https://code.claude.com/docs/zh-CN/agent-teams.md#enable-agent-teams) 的情况下启动会话，Claude 可以在 Agent 工具调用中传递 `name` 参数来为子代理命名。它可能会自行这样做，而不事先询问您。名称使子代理可被寻址：在子代理完成后，Claude 可以[按名称向其发送消息或恢复它](#resume-subagents)。

在启用了 [agent teams](https://code.claude.com/docs/zh-CN/agent-teams.md) 的交互式会话中，Claude 从主对话生成的带有 `name` 的子代理会改为以队友身份启动，除非该调用是 [fork](#fork-the-current-conversation) 或在调用本身中传递了 `isolation`。子代理 frontmatter 中的 `isolation` 值无法阻止这一点，此时该队友会在主会话的工作目录中运行。请参阅 [Claude 如何启动 agent teams](https://code.claude.com/docs/zh-CN/agent-teams.md#how-claude-starts-agent-teams)。

### 子代理中的 API 错误

当某种原因[在流式传输中途截断了子代理的响应](https://code.claude.com/docs/zh-CN/errors.md#the-response-above-may-be-incomplete)，且部分响应包含文本但没有工具调用时，Claude Code 会提示子代理继续，而不是结束运行。这在交互式会话中同样适用。只有当这些继续次数用完后，运行才会因该错误而结束。

因 API 错误（例如用量限制或反复出现的服务器错误）而结束运行的子代理会将该失败报告给 Claude。Claude 收到的内容取决于子代理的运行位置：

* **前台**：如果速率限制、过载或服务器错误截断了已经产生文本输出的子代理，Agent 工具会返回该部分输出，并附注说明子代理被截断、未完成其任务。未产生任何输出、或输出仅包含工具调用的子代理会以 [`Agent terminated early due to an API error`](https://code.claude.com/docs/zh-CN/errors#agent-terminated-early-due-to-an-api-error) 失败，后跟错误详情。
* **后台**：子代理会被标记为失败，Claude 在其结束时收到的消息会指明该 API 错误，并包含子代理的最后输出，因此部分工作不会丢失。

如果子代理的报告以下面这一行开头，说明该子代理是在备用模型上完成的：

```text theme={null}
⚠ This agent switched to <model> because its earlier model could not be used.
```

当您配置了[备用模型链](https://code.claude.com/docs/zh-CN/model-config.md#fallback-model-chains)，且子代理遇到该链所覆盖的失败（例如其模型不可用）时，Claude Code 会将子代理切换到链中第一个接受请求的模型。子代理会继续工作，而不是因错误而结束。

底层 API 错误消除后，请让 Claude 重试该任务或[恢复子代理](#resume-subagents)。

### 子代理输出扫描

Claude Code 会在 Claude 读取每个子代理的最终报告之前对其进行扫描。子代理可能读取过您从未审查过的文件、网页或命令输出，而这些来源中的文本可能携带针对主对话的指令。扫描从不删除或改写任何内容；它会做出两种您可能在报告中注意到的更改：

* **插入反斜杠**：扫描会在模仿 Claude Code 自身输出的文本中插入反斜杠，例如 `<system-reminder>` 标签，或以 `Human:` 或 `Assistant:` 开头的行，使这些模仿内容被当作普通文本读取，而不会被误认为是对话的一部分。
* **标记行**：当报告模仿了 `<system-reminder>` 之类的标签，或提及 `bypassPermissions` 或 `--dangerously-skip-permissions` 等权限设置时，扫描会在开头添加一行以 `[harness: subagent output matched instruction-shaped pattern(s):` 开头的内容。提及权限设置的报告会获得标记行，但文本本身保持原样。

扫描不会判断内容是否恶意，也不会改变报告中的指令所能产生的效果：报告促使 Claude 发起的工具调用仍然要经过会话的[权限检查](https://code.claude.com/docs/zh-CN/permissions.md)和[沙箱隔离](https://code.claude.com/docs/zh-CN/sandboxing.md)。它不能替代[限制子代理可访问的范围](#control-subagent-capabilities)。

作为子代理结果返回给 Claude 的报告还会带有一个标题，将其标记为子代理输出。该标题说明报告中的指令或批准声明是子代理的话语，并不具备来自您的任何权威。

[后台子代理的报告](#run-subagents-in-foreground-or-background)会包含在完成通知中送达，该通知被标记为自动化事件，而不是来自您的消息。

### 常见模式

#### 隔离高输出量操作

子代理最有效的用途之一是隔离会产生大量输出的操作。运行测试、获取文档或处理日志文件可能会消耗大量上下文。将这些操作委托给子代理后，冗长的输出会留在子代理的上下文中，只有相关摘要会返回到您的主对话。

```text wrap theme={null}
Use a subagent to run the test suite and report only the failing tests with their error messages
```

#### 并行研究

对于相互独立的调查，可以生成多个子代理同时工作：

```text wrap theme={null}
Research the authentication, database, and API modules in parallel using separate subagents
```

每个子代理独立探索各自的领域，然后由 Claude 综合这些发现。当各条研究路径互不依赖时，这种方式效果最好。

> [!WARNING]
>
> 子代理完成后，其结果会返回到您的主对话。运行许多各自返回详细结果的子代理可能会消耗大量上下文，而且每个子代理在运行时也会消耗自己的 token。

对于需要持续并行运行或无法容纳在一个上下文窗口中的工作，请在[单独的会话](https://code.claude.com/docs/zh-CN/agents.md)中运行，并让 Claude [在会话之间传递发现](https://code.claude.com/docs/zh-CN/cross-session-messaging.md)。

#### 串联子代理

对于多步骤工作流，可以要求 Claude 依次使用多个子代理。每个子代理完成其任务后将结果返回给 Claude，再由 Claude 将相关上下文传递给下一个子代理。

```text wrap theme={null}
Use the code-reviewer subagent to find performance issues, then use the optimizer subagent to fix them
```

### 在子代理和主对话之间选择

在以下情况下使用**主对话**：

* 任务需要频繁来回交流或迭代完善
* 多个阶段（例如规划、实现和测试）共享大量上下文
* 您要进行快速、有针对性的更改
* 延迟很重要。非 [fork](#fork-the-current-conversation) 的子代理从零开始，可能需要时间来收集上下文

在以下情况下使用**子代理**：

* 任务会产生您在主上下文中不需要的冗长输出
* 您想强制执行特定的工具限制或权限
* 工作是自成一体的，可以返回一份摘要

如果您想要可复用的提示词或工作流，并希望它们在主对话上下文中而不是隔离的子代理上下文中运行，请考虑改用 [Skills](https://code.claude.com/docs/zh-CN/skills.md)。

对于关于对话中已有内容的问题，请使用 [`/btw`](https://code.claude.com/docs/zh-CN/interactive-mode#side-questions-with-%2Fbtw) 而不是子代理。它能看到您的完整上下文，但无法使用工具，且答案不会添加到历史记录中。

### 让子代理生成自己的子代理

默认情况下，子代理可以生成自己的子代理，最多可达主对话之下三层。达到深度限制时，Claude Code 会从除 [fork](#fork-the-current-conversation) 之外的所有子代理中移除 `Agent` 工具，因此处于限制层级的子代理会自己完成委托的工作并返回一份摘要。处于限制层级的 fork 在其继承的工具列表中仍保留 `Agent`，但该工具会返回错误而不是生成子代理。

嵌套子代理适用于本身可拆分为并行子任务的委托任务，例如一个审查子代理为每项发现分派一个验证者。默认情况下，发起启动的子代理会等待每个嵌套子代理的报告：

* **交互式终端会话**：只有顶层子代理的摘要会返回给您，因为启动后台子代理的子代理会等待它们的结果后再结束。
* **`claude -p` 和 Agent SDK**：当 [fork 模式](#turn-fork-mode-on-or-off)开启时，发起启动的子代理会等待。如果您关闭 fork 模式，嵌套子代理会改为在[后台](#run-subagents-in-foreground-or-background)启动。发起启动的子代理不会等待后台嵌套子代理；如果该嵌套子代理在其启动者结束后仍在运行，它会向您的主对话报告。

要更改该限制，请将 [`CLAUDE_CODE_MAX_SUBAGENT_SPAWN_DEPTH`](https://code.claude.com/docs/zh-CN/env-vars) 设置为您希望在主对话之下允许的子代理层数。例如，[`settings.json`](https://code.claude.com/docs/zh-CN/settings) 中的以下条目将嵌套限制为两层：

```json theme={null}
{
  "env": {
    "CLAUDE_CODE_MAX_SUBAGENT_SPAWN_DEPTH": "2"
  }
}
```

使用此值时，您的子代理可以委托给它们自己的第二层子代理，而第二层无法再进一步委托。设置为 `1` 可关闭嵌套。

嵌套子代理的配置方式与顶层子代理相同，并从相同的[作用域](#choose-the-subagent-scope)中解析。要在嵌套开启时阻止某个子代理生成子代理（例如应保持只读的审查者），请从其 [`tools`](#available-tools) 列表中省略 `Agent`，或将其添加到 `disallowedTools`。

在终端中，Claude Code 会在输入框下方的子代理面板中以树形显示嵌套子代理，并为面板中仍有后代的每一行标注 `(+N)`，表示其后代数量。打开某一行即可查看该子代理的同级和直接子级，以及返回 `main` 的路径。

### 并发子代理限制

有两个限制控制子代理的使用，各自对应一个变量：本限制在运行中的子代理过多时阻止 Claude 生成更多子代理，而[深度限制](#let-subagents-spawn-their-own-subagents)则限制子代理的嵌套深度。Claude 在一个会话中可以生成的子代理总数没有限制。

默认情况下，当会话中有 20 个子代理正在运行时，使用 Agent 工具再生成一个会失败并返回 `Concurrent subagent limit reached`，且该错误会告知 Claude 不要重试。当运行数量降到限制以下时，即可再次成功生成。要更改该限制，请将 [`CLAUDE_CODE_MAX_CONCURRENT_SUBAGENTS`](https://code.claude.com/docs/zh-CN/env-vars) 设置为任意正整数。启用了 [ultracode](https://code.claude.com/docs/zh-CN/model-config.md#adjust-effort-level) 的会话不受此限制约束。需要 Claude Code v2.1.217 或更高版本。

该限制只阻止 Claude 使用 Agent 工具生成的子代理，但其他运行也会占用相同的槽位：

* 您通过 [`/subtask`](#fork-the-current-conversation) 启动的会话内 fork 在运行时会占用一个槽位，且永远不会被该限制阻止。
* [恢复](#resume-subagents)已完成的子代理会占用一个新槽位，且不检查限制，因此恢复操作可能使运行数量超过限制。

其他功能运行的 Agent，例如[工作流](https://code.claude.com/docs/zh-CN/workflows.md) Agent 和 [agent team](https://code.claude.com/docs/zh-CN/agent-teams.md) 队友，则遵循各自的限制。

### 管理子代理上下文

#### 启动时加载的内容

每个子代理都从一个全新的、隔离的上下文窗口开始。它看不到您的对话历史、您已调用的 skill 或 Claude 已读取的文件。Claude 会编写一条概括任务的委托消息，子代理以此为起点开展工作。例外是 [fork](#fork-the-current-conversation)，它会继承父对话，而不是从零开始。

非 fork 子代理的初始上下文包含：

* **系统提示词**：Agent 自身的提示词加上 Claude Code 附加的环境详情，而不是 Claude Code 系统提示词。自定义子代理在 [markdown 正文](#write-subagent-files)或 `prompt` 字段中定义其系统提示词。内置 Agent 具有预定义的提示词。
* **任务消息**：Claude 在移交工作时编写的委托提示词。
* **CLAUDE.md 文件**：主对话所加载的 [CLAUDE.md 层级结构](https://code.claude.com/docs/zh-CN/memory.md#how-claude-md-files-load)中的每一级，包括 `~/.claude/CLAUDE.md`、项目规则、`CLAUDE.local.md`、托管策略文件，以及作为项目指令加载的任何 [`AGENTS.md` 文件](https://code.claude.com/docs/zh-CN/memory#agents-md)。内置的 Explore 和 Plan Agent 会跳过这些。定义中设置了 [`omitClaudeMd`](#supported-frontmatter-fields) 的子代理只加载托管策略文件；如果该定义来自[托管设置](#choose-the-subagent-scope)，则一个也不加载。
* **Git 状态**：子代理启动时 Claude Code 从您的仓库读取的快照。对于在该仓库的[自有 worktree](https://code.claude.com/docs/zh-CN/worktrees.md#isolate-subagents-with-worktrees) 中运行的子代理，快照会显示该 worktree 的分支、状态和最近的提交。在 Git 仓库之外或快照被关闭时不存在；请参阅 [`includeGitInstructions`](https://code.claude.com/docs/zh-CN/settings-reference#includegitinstructions)。Explore 和 Plan 无论如何都会跳过它。
* **预加载的 skill**：Agent 的 [`skills` 字段](#preload-skills-into-subagents)中列出的每个 skill 的完整内容。内置 Agent 不预加载 skill。
* **同级名单**：一条[系统提醒](https://code.claude.com/docs/zh-CN/glossary.md#system-reminder)，列出 `main` 以及会话中所有其他已命名的 Agent，每一个都是 [`SendMessage`](#resume-subagents) 的有效 `to` 值。仅当子代理的工具包含 `SendMessage` 且至少有一个其他 Agent 拥有名称时才会出现该名单，无论该名称是 Claude 在生成时指定的，还是该 Agent 作为 [agent team](https://code.claude.com/docs/zh-CN/agent-teams.md) 队友运行。名单是子代理启动时拍摄的快照，因此之后命名的 Agent 不会出现在其中。

要在不加载用户、项目和本地 CLAUDE.md 文件的情况下启动您自己的某个子代理，请在其 frontmatter 或 `--agents` JSON 中设置 [`omitClaudeMd: true`](#supported-frontmatter-fields)。对于 Explore、Plan 以及设置了 `omitClaudeMd` 的子代理，当子代理处理某个子目录中的文件时，该子目录的 `CLAUDE.md` 和匹配的[路径作用域规则](https://code.claude.com/docs/zh-CN/memory.md#path-specific-rules)仍会[加载](https://code.claude.com/docs/zh-CN/memory.md#how-claude-md-files-load)。

主对话在读取这些子代理的结果时仍拥有您的完整 CLAUDE.md，因此大多数规则无需传达给子代理本身。如果某条规则必须传达，例如"忽略 `vendor/` 目录"，请在委托时给 Claude 的提示词中重新说明。

您无法更改哪些子代理会接收 Git 状态。只有 Explore 和 Plan 会跳过它。

某些主对话状态永远不会传递给非 fork 子代理：

* **输出样式**：子代理运行自己的系统提示词，因此您的[输出样式](https://code.claude.com/docs/zh-CN/output-styles.md)不会影响其回复，[fork](#fork-the-current-conversation) 除外。
* **自动记忆**：不会加载主对话的[自动记忆](https://code.claude.com/docs/zh-CN/memory.md#auto-memory)。要为子代理提供其自身的持久记忆，请使用 [`memory` 字段](#enable-persistent-memory)。
* **上下文窗口大小**：子代理的上下文窗口大小由其自身的模型决定，而不是由父级决定。委托给上下文窗口较小的模型时，该子代理获得的也是较小的窗口。

#### 恢复子代理

每次调用子代理都会创建一个新实例，而不是延续之前的实例。要继续某个现有子代理的工作而不是从头开始，请让 Claude 恢复它。

恢复的子代理会保留其完整的对话历史，包括之前所有的工具调用、结果和推理。如果该子代理生成过[自己的后台子代理](#let-subagents-spawn-their-own-subagents)，该历史还包括这些后台子代理在其运行期间交付的结果。在 v2.1.304 之前，如果 Claude 在子代理刚完成时就向其发送消息或恢复它，该子代理可能会遗忘其最近的工作。

* 子代理完成时，Claude 会收到其 Agent ID。
* 内置的 Explore 和 Plan Agent 是一次性的，不返回 Agent ID，因此 Claude 无法恢复它们。需要继续工作时，请使用 `general-purpose` 或自定义子代理。
* 当子代理因达到 [`maxTurns`](#supported-frontmatter-fields) 限制而停止时，Claude Code 会将返回的输出标记为部分结果，Claude 可以恢复该子代理以继续其工作。

Claude 使用 `SendMessage` 工具，以 Agent 的 ID 或名称作为 `to` 字段来恢复它。`SendMessage` 不要求启用 [agent teams](https://code.claude.com/docs/zh-CN/agent-teams.md)；只有 `shutdown_request` 和 `plan_approval_response` 等结构化团队协议消息才需要。除了子代理和队友之外，在启用了跨会话消息的会话中，Claude 还可以使用同一工具向[您的其他 Claude Code 会话](https://code.claude.com/docs/zh-CN/cross-session-messaging.md)发送消息，无论它们在本机还是[其他机器上](https://code.claude.com/docs/zh-CN/cross-session-messaging.md#message-sessions-on-other-machines)。

要恢复子代理，请让 Claude 继续之前的工作：

```text wrap theme={null}
Use the code-reviewer subagent to review the authentication module
[Agent completes]

Continue that code review and now analyze the authorization logic
[Claude resumes the subagent with full context from previous conversation]
```

当 Claude 使用 `SendMessage` 工具向已完成的子代理发送消息时，该子代理会在后台恢复，无需新的 `Agent` 调用。对于 Claude 使用 `TaskStop` 工具停止的子代理，在其被停止的运行退出后同样适用。恢复后的运行会保留[子代理首次运行时的工具集](#run-subagents-in-foreground-or-background)，并且可以继续读取[原始运行预热的提示缓存](https://code.claude.com/docs/zh-CN/prompt-caching.md#subagents-and-the-cache)。

拥有 `SendMessage` 工具的子代理也可以发送这种消息。在交互式会话中，被恢复的 Agent 随后会向恢复它的那个子代理汇报，而不是向您的主对话汇报。该子代理会等待结果后再完成自己的工作。当子代理向它所汇报的 Agent（例如其自身的启动者）发送消息时，Claude Code 会恢复该 Agent，且不会重定向其结果。

由您亲自停止的子代理（通过在 `/tasks` 中按 `x` 或发出 SDK `stop_task` 请求）不会自动恢复。如果 Claude 向它发送消息，该消息会被拒绝，并告知 Claude 该 Agent 已被取消。

当[该子代理所在的行仍在子代理面板中](#run-subagents-in-foreground-or-background)时，您可以在其会话记录中输入内容来亲自恢复它。此后，来自 Claude 的消息即可再次自动恢复它。

恢复会在同一 ID 下启动该 Agent 的一次新运行，因此已失败或已完成的子代理会在任务列表和 Agent SDK 的任务事件中再次显示为运行中。

`SendMessage` 会检查某个名称是否仍指向对话中先前通过该名称联系到的同一个 Agent。如果该名称已被较新的 Agent 占用（例如重新生成的后台 Agent 复用了该名称），Claude Code 会拒绝发送，而不是将消息送达错误的 Agent，并且错误会报告该名称现在指向哪个 Agent，以便 Claude 重新指定目标。要在较早的 Agent 仍在运行时联系它，Claude 会使用生成该 Agent 时收到的 Agent ID 来寻址。此检查仅限于当前对话，并会在 `/clear` 时重置。

子代理会将来自启动它的 Agent 的消息视为正常的任务指示，包括任务进行中的方向修正，并在其自身的权限设置范围内执行。无论消息由谁发送，以下两项限制始终有效：任何 Agent 发来的消息都不算作您对待处理权限提示的批准；任何 Agent 消息都无法更改子代理的权限设置、`CLAUDE.md` 或配置。只有权限系统或您本人的消息才能给予批准。

如果您想明确引用某个子代理，也可以向 Claude 询问其 Agent ID，或在 `~/.claude/projects/{project}/{sessionId}/subagents/` 下的会话记录文件中查找 ID。每份会话记录存储为 `agent-{agentId}.jsonl`。

子代理会话记录独立于主对话持久保存：

* **主对话压缩**：主对话压缩时，子代理会话记录不受影响。它们存储在单独的文件中。
* **会话持久性**：子代理会话记录在其所属会话内持久保存。重启 Claude Code 后，您可以通过恢复同一会话来[恢复子代理](#resume-subagents)。
* **自动清理**：Claude Code 会在 `cleanupPeriodDays` 保留期（默认为 30 天）过后删除子代理会话记录，遵循[保留清理规则](https://code.claude.com/docs/zh-CN/claude-directory.md#cleaned-up-automatically)。

##### 重启后按名称向子代理发送消息

> [!NOTE]
>
> 对于以队友身份启动的子代理，请改为参阅 [agent team 限制](https://code.claude.com/docs/zh-CN/agent-teams.md#limitations)。

如果在您重启 Claude Code 并恢复会话后，Claude 无法按名称向某个子代理发送消息，请改为向 Claude 提供该子代理的 Agent ID。Claude 自上次[压缩](https://code.claude.com/docs/zh-CN/context-window.md#what-survives-compaction)以来在主对话中指定的名称在重启后仍会保留，而该次压缩之前的名称则不会保留。要查找某个名称对应的 ID，请在 `~/.claude/projects/{project}/{sessionId}/subagents/` 下的 `agent-{agentId}.meta.json` 文件中查找该名称。

通过 ID 发送一条消息后，该子代理的名称即可再次使用，除非已有较新的 Agent 占用了该名称。在 v2.1.299 之前，任何名称都无法在重启后保留，即使 Agent ID 仍然有效，按名称发送也会被拒绝。

#### 上下文自动压缩

子代理支持自动压缩。压缩会在子代理自身的上下文窗口逐渐填满时触发，`CLAUDE_AUTOCOMPACT_PCT_OVERRIDE` 同样适用于子代理。有关该覆盖何时生效，请参阅[环境变量](https://code.claude.com/docs/zh-CN/env-vars.md)。

压缩事件会记录在子代理会话记录文件中：

```json theme={null}
{
  "type": "system",
  "subtype": "compact_boundary",
  "compactMetadata": {
    "trigger": "auto",
    "preTokens": 167189
  }
}
```

`preTokens` 值表示压缩发生前已使用的 token 数量。

##### 为单个子代理设置自动压缩窗口

要让某个子代理比原本更早压缩，请在其 [frontmatter](#supported-frontmatter-fields) 中将 `autoCompactWindow` 设置为 `100000` 到 `1000000` 之间的 token 数。如果您设置了 [`CLAUDE_CODE_AUTO_COMPACT_WINDOW`](https://code.claude.com/docs/zh-CN/env-vars#variables)，子代理会改用该变量的窗口，此字段不起作用，除非 Claude Code [忽略了该变量的值](https://code.claude.com/docs/zh-CN/model-config.md#claude_code_auto_compact_window-values)。需要 Claude Code v2.1.296 或更高版本。

以下 frontmatter 为一个读取长日志文件的子代理将 `autoCompactWindow` 设置为 200,000 token：

```yaml theme={null}
---
name: log-reader
description: Reads long log files and reports the errors it finds
autoCompactWindow: 200000
---
```

请将该值写为 `200000` 这样的纯整数，因为在 frontmatter 中 Claude Code 会将 `200k` 读作 200，超出了允许范围。此时子代理仍会照常加载和压缩，就像该字段不存在一样。要检查 Claude Code 是否忽略了您的值，请在 shell 中运行 `claude --debug`，并在[调试日志](#supported-frontmatter-fields)中搜索 `invalid autoCompactWindow`。

如果该值在范围内但您仍未看到任何变化，请检查以下原因：

* **该值不低于子代理原本会继承的[自动压缩窗口](https://code.claude.com/docs/zh-CN/model-config.md#context-window-and-auto-compaction)**：要更早压缩，请设置低于该窗口的值。
* **您通过 `--agent` 或 `agent` 设置以此子代理身份运行整个会话**：此时该字段不起作用。要设置会话的窗口，请参阅[设置自动压缩窗口](https://code.claude.com/docs/zh-CN/model-config.md#set-the-auto-compact-window)。

## 分叉当前对话

> [!NOTE]
>
> 当[Agent 视图被关闭](https://code.claude.com/docs/zh-CN/agent-view.md#turn-off-agent-view)时，`/subtask` 不可用，`/fork` 改为启动分叉子代理；否则 `/fork` 将整个会话复制到新的[后台会话](https://code.claude.com/docs/zh-CN/agent-view.md#from-inside-a-session)中。

分叉是一个 subagent，它继承到目前为止的整个对话，而不是从头开始。这消除了 subagents 通常提供的输入隔离：分叉看到与主会话相同的系统提示、工具、模型和消息历史，因此您可以将其交给一个辅助任务而无需重新解释情况。分叉自己的工具调用仍然保持在您的对话之外，只有其最终结果返回，因此您的主 context window 保持干净。当任何其他 subagent 需要太多背景才能有用时，或当您想从相同的起点并行尝试多种方法时，使用分叉。

分叉模式启用时（默认即为启用），Claude 可以自行启动分叉。要停止分叉，请参阅[关闭分叉模式](#turn-fork-mode-on-or-off)。

您可以使用 `/subtask` 后跟任务自己启动分叉，无论分叉模式是否启用。Claude Code 从任务的前几个单词命名分叉。以下示例分叉对话以在您继续主会话中的实现时草拟测试用例：

```text wrap theme={null}
/subtask draft unit tests for the parser changes so far
```

分叉出现在提示下方的面板中，并在您继续工作时在后台运行。完成后，其结果作为消息到达您的主对话。下一部分涵盖了在分叉运行时观察和引导它们的面板控制。

### 观察和引导运行中的分叉

运行中的分叉出现在提示输入下方的面板中，主会话有一行，每个分叉有一行。

当分叉成功完成时，Claude Code 会移除其行。Claude Code 保留失败或您停止的分叉的行 30 秒，[与任何其他后台 subagent 相同](#run-subagents-in-foreground-or-background)。在 v2.1.232 之前，Claude Code 也会将完成的分叉的行保留 30 秒。

使用这些键与面板交互：

| Key | Action |
| :- | :- |
| `↑` / `↓` | 在行之间移动 |
| `Enter` | 打开所选分叉的转录并向其发送后续消息 |
| `x` | 如果分叉正在运行，停止它；如果不再运行，关闭其行。在主会话行或您使用 `Enter` 打开其转录的分叉行上，`x` 会输入到提示中 |
| `Esc` | 将焦点返回到提示输入 |

打开分叉或子代理的会话记录后，后续消息和 [skill](https://code.claude.com/docs/zh-CN/skills.md) 会发送到该 Agent，而内置命令会发送到您的主对话，并有以下保护措施：

* `/compact`、`/clear` 和 `/rewind` 作用于主对话，因此 Claude Code 在从该视图运行其中任何一个之前会要求您确认。
* `/model` 和 `/fast` 设置的是主对话的模型和快速模式，而不是所查看 Agent 的，因此它们不会从该视图运行。一条通知会告诉您原因。

要让所查看的 Agent 在其等待的工作完成之前读取您的消息，请使用 [`Ctrl+Enter` 或 `Ctrl+X Ctrl+S`](https://code.claude.com/docs/zh-CN/keybindings#chat-actions) 发送。该 Agent 正在等待的任何可以移至[后台](https://code.claude.com/docs/zh-CN/tools-reference.md#background-commands)的 shell 命令或子代理都会移至后台并继续运行。当该 Agent 正在编写回复，或正在等待无法移至后台的工作时，它会继续进行，并在完成后读取您的消息。需要 Claude Code v2.1.286 或更高版本。

### 分叉与其他 subagents 的区别

分叉继承主会话在生成时拥有的一切。任何其他 subagent 从其定义开始。

| | 分叉 | 非分叉 subagent |
| :- | :- | :- |
| 上下文 | 完整的对话历史 | 新鲜上下文，带有您传递的提示 |
| 系统提示和工具 | 与主会话相同 | 来自 subagent 的[定义文件](#write-subagent-files)，[为后台运行过滤](#available-tools) |
| 模型 | 与主会话相同 | 来自 subagent 的 `model` 字段 |
| Prompt cache | 与主会话共享 | 单独的缓存 |

因为分叉的系统提示词和工具定义与父级相同，其第一个请求重用父级的[提示缓存](https://code.claude.com/docs/zh-CN/prompt-caching.md#subagents-and-the-cache)。由于这种重用，对于需要相同上下文的任务，分叉的成本低于新的子代理。

当 Claude 通过 Agent 工具生成分叉时，它可以传递 `isolation: "worktree"` 以便分叉的文件编辑被写入单独的 git worktree 而不是您的检出。分叉无法生成进一步的分叉。

### 关闭分叉模式

分叉模式允许 Claude 自行启动分叉。要阻止 Claude 启动分叉：

* **仅停止分叉**：在您的[设置](#disable-specific-subagents)中将 `"Agent(fork)"` 添加到 `permissions.deny`。
* **关闭分叉模式**：按照[让 Claude 等待子代理](#let-claude-wait-for-a-subagent)中的步骤，将 `CLAUDE_CODE_FORK_SUBAGENT` 设置为 `0`。

分叉模式启用时，在您本机上的交互式终端、VS Code 或桌面会话中，Claude 也无法[等待子代理](#run-subagents-in-foreground-or-background)。从 Claude Code v2.1.302 起默认启用分叉模式，而在交互式终端会话中则从 v2.1.232 起默认启用。

## 示例 subagents

这些示例演示了构建 subagents 的有效模式。将它们用作起点，或使用 Claude 生成自定义版本。

> [!TIP]
>
> **最佳实践：**
>
> * **设计专注的 subagents：** 每个 subagent 应该在一个特定任务中表现出色
> * **编写单独突出一个 subagent 的描述：** Claude 使用描述来决定何时委托。使每个描述具体到足以路由到正确的 subagent，并将组合集保持在 [15,000 令牌描述预算](#understand-automatic-delegation) 内
> * **限制工具访问：** 仅授予必要的权限以确保安全和专注
> * **检入版本控制：** 与您的团队共享项目 subagents

### 代码审查者

一个只读 subagent，审查代码而不修改它。此示例展示了如何设计一个专注的 subagent，具有有限的工具访问（排除 Edit 和 Write）和详细的提示，指定确切要查找的内容以及如何格式化输出。

```markdown theme={null}
---
name: code-reviewer
description: Expert code review specialist. Proactively reviews code for quality, security, and maintainability. Use immediately after writing or modifying code.
tools: Read, Grep, Glob, Bash
model: inherit
---

You are a senior code reviewer ensuring high standards of code quality and security.

When invoked:
1. Run git diff to see recent changes
2. Focus on modified files
3. Begin review immediately

Review checklist:
- Code is clear and readable
- Functions and variables are well-named
- No duplicated code
- Proper error handling
- No exposed secrets or API keys
- Input validation implemented
- Good test coverage
- Performance considerations addressed

Provide feedback organized by priority:
- Critical issues (must fix)
- Warnings (should fix)
- Suggestions (consider improving)

Include specific examples of how to fix issues.
```

### 调试器

一个可以分析和修复问题的 subagent。与代码审查者不同，这个包括 Edit，因为修复错误需要修改代码。提示提供了从诊断到验证的清晰工作流。

```markdown theme={null}
---
name: debugger
description: Debugging specialist for errors, test failures, and unexpected behavior. Use proactively when encountering any issues.
tools: Read, Edit, Bash, Grep, Glob
---

You are an expert debugger specializing in root cause analysis.

When invoked:
1. Capture error message and stack trace
2. Identify reproduction steps
3. Isolate the failure location
4. Implement minimal fix
5. Verify solution works

Debugging process:
- Analyze error messages and logs
- Check recent code changes
- Form and test hypotheses
- Add strategic debug logging
- Inspect variable states

For each issue, provide:
- Root cause explanation
- Evidence supporting the diagnosis
- Specific code fix
- Testing approach
- Prevention recommendations

Focus on fixing the underlying issue, not the symptoms.
```

### 数据科学家

一个用于数据分析工作的特定领域 subagent。此示例展示了如何为典型编码任务之外的专门工作流创建 subagents。它明确设置 `model: sonnet` 以获得更强大的分析能力。

```markdown theme={null}
---
name: data-scientist
description: Data analysis expert for SQL queries, BigQuery operations, and data insights. Use proactively for data analysis tasks and queries.
tools: Bash, Read, Write
model: sonnet
---

You are a data scientist specializing in SQL and BigQuery analysis.

When invoked:
1. Understand the data analysis requirement
2. Write efficient SQL queries
3. Use BigQuery command line tools (bq) when appropriate
4. Analyze and summarize results
5. Present findings clearly

Key practices:
- Write optimized SQL queries with proper filters
- Use appropriate aggregations and joins
- Include comments explaining complex logic
- Format results for readability
- Provide data-driven recommendations

For each analysis:
- Explain the query approach
- Document any assumptions
- Highlight key findings
- Suggest next steps based on data

Always ensure queries are efficient and cost-effective.
```

### 数据库查询验证器

一个允许 Bash 访问但验证命令以仅允许只读 SQL 查询的 subagent。此示例展示了当您需要比 `tools` 字段提供的更精细的控制时如何使用 `PreToolUse` hooks。

```markdown theme={null}
---
name: db-reader
description: Execute read-only database queries. Use when analyzing data or generating reports.
tools: Bash
hooks:
  PreToolUse:
    - matcher: "Bash"
      hooks:
        - type: command
          command: "./scripts/validate-readonly-query.sh"
---

You are a database analyst with read-only access. Execute SELECT queries to answer questions about the data.

When asked to analyze data:
1. Identify which tables contain the relevant data
2. Write efficient SELECT queries with appropriate filters
3. Present results clearly with context

You cannot modify data. If asked to INSERT, UPDATE, DELETE, or modify schema, explain that you only have read access.
```

Claude Code [通过 stdin 将 hook 输入作为 JSON 传递](https://code.claude.com/docs/zh-CN/hooks.md#pretooluse-input) 给 hook 命令。验证脚本读取此 JSON，提取正在执行的命令，并根据 SQL 写入操作列表检查它。如果检测到写入操作，脚本 [以代码 2 退出](https://code.claude.com/docs/zh-CN/hooks.md#exit-code-2-behavior-per-event) 以阻止执行，并通过 stderr 向 Claude 返回错误消息。

在您的项目中的任何位置创建验证脚本。路径必须与您的 hook 配置中的 `command` 字段匹配：

```bash theme={null}
#!/bin/bash
# Blocks SQL write operations, allows SELECT queries

# Read JSON input from stdin
INPUT=$(cat)

# Extract the command field from tool_input using jq
COMMAND=$(echo "$INPUT" | jq -r '.tool_input.command // empty')

if [ -z "$COMMAND" ]; then
  exit 0
fi

# Block write operations (case-insensitive)
if echo "$COMMAND" | grep -iE '\b(INSERT|UPDATE|DELETE|DROP|CREATE|ALTER|TRUNCATE|REPLACE|MERGE)\b' > /dev/null; then
  echo "Blocked: Write operations not allowed. Use SELECT queries only." >&2
  exit 2
fi

exit 0
```

在 macOS 和 Linux 上，使脚本可执行：

```bash theme={null}
chmod +x ./scripts/validate-readonly-query.sh
```

在 Windows 上，用 PowerShell 编写验证脚本，并在 hook 条目中添加 `shell: powershell`。请参阅 [在 PowerShell 中运行 hooks](https://code.claude.com/docs/zh-CN/hooks.md#windows-powershell-tool)。

Hook 通过 stdin 接收 JSON，Bash 命令在 `tool_input.command` 中。退出代码 2 阻止操作并将错误消息反馈给 Claude。有关退出代码和输出的详细信息，请参阅 [Hooks](https://code.claude.com/docs/zh-CN/hooks.md#exit-code-output)，有关完整的输入架构，请参阅 [Hook input](https://code.claude.com/docs/zh-CN/hooks.md#pretooluse-input)。

系统提示告诉 subagent 拒绝写入请求，因此 hook 是一个后备：如果 subagent 尝试写入，Claude Code 会阻止该命令，subagent 会看到 `Blocked: Write operations not allowed. Use SELECT queries only.` 消息。

## 后续步骤

现在您了解了 subagents，探索这些相关功能：

* [使用 plugins 分发 subagents](https://code.claude.com/docs/zh-CN/plugins/components.md#agents) 以在团队或项目中共享 subagents
* [以编程方式运行 Claude Code](https://code.claude.com/docs/zh-CN/headless.md)，使用 Agent SDK 进行 CI/CD 和自动化
* [使用 MCP 服务器](https://code.claude.com/docs/zh-CN/mcp.md) 为 subagents 提供对外部工具和数据的访问
