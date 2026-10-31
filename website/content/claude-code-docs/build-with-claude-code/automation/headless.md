> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# 以编程方式运行 Claude Code

> 使用 Agent SDK 从 CLI、Python 或 TypeScript 以编程方式运行 Claude Code。

[Agent SDK](https://code.claude.com/docs/zh-CN/agent-sdk/overview.md) 为您提供了与 Claude Code 相同的工具、agent 循环和上下文管理。它可作为 CLI 用于脚本和 CI/CD，或作为 [Python](https://code.claude.com/docs/zh-CN/agent-sdk/python.md) 和 [TypeScript](https://code.claude.com/docs/zh-CN/agent-sdk/typescript.md) 包供完整的编程控制。

要以非交互模式运行 Claude Code，请使用 `-p` 传递您的提示和任何 [CLI 选项](https://code.claude.com/docs/zh-CN/cli-reference.md)：

```bash theme={null}
claude -p "Find and fix the bug in auth.py" --allowedTools "Read,Edit,Bash"
```

本页面涵盖通过 CLI (`claude -p`) 使用 Agent SDK。对于具有结构化输出、工具批准回调和原生消息对象的 Python 和 TypeScript SDK 包，请参阅 [完整 Agent SDK 文档](https://code.claude.com/docs/zh-CN/agent-sdk/overview.md)。

## 基本用法

在任何 `claude` 命令中添加 `-p`（或 `--print`）标志以非交互方式运行它。并非每个 [CLI 选项](https://code.claude.com/docs/zh-CN/cli-reference.md) 都能与 `-p` 组合使用。Claude Code 会拒绝 `--bg`，也会拒绝带有任务描述的 `--cloud`，并报错说明冲突；而 `--cloud` 与会话 ID 和 `-p` 一起使用时，会 [将消息排队到该云端会话](https://code.claude.com/docs/zh-CN/claude-code-on-the-web.md#send-follow-ups-from-the-cli) 并退出。经常与 `-p` 结合使用的选项包括：

* `--continue` 用于 [继续对话](#continue-conversations)
* `--allowedTools` 用于 [自动批准工具](#auto-approve-tools)
* `--output-format` 用于 [获取结构化输出](#get-structured-output)

此示例向 Claude 询问有关您的代码库的问题并打印回复：

```bash theme={null}
claude -p "What does the auth module do?"
```

Claude Code 在成功时以代码 0 退出，在运行失败时以非零代码退出，因此您的脚本可以根据退出状态进行分支。如果您传递无效标志，Claude Code 会在运行开始前向 stderr 报告错误。当运行内部发生故障（例如缺少身份验证）时，Claude Code 会将故障作为结果打印到 stdout。

### 使用 bare 模式更快启动

添加 `--bare` 可以跳过 hook、skill、自定义命令、[子代理](https://code.claude.com/docs/zh-CN/sub-agents.md)、已安装的插件、MCP 服务器、自动记忆和 CLAUDE.md 的自动发现，从而减少启动时间。如果不使用它，`claude -p` 会像交互式会话一样发现并加载这些内容，包括在工作目录或 `~/.claude` 中配置的任何内容。

bare 模式适用于需要在每台机器上获得相同结果的 CI 和脚本。队友的 `~/.claude` 中的 hook 或项目的 `.mcp.json` 中的 MCP 服务器不会运行，因为 bare 模式从不读取它们。您用 `--add-dir` 指定的目录是一个部分例外：bare 模式会从其 `.claude/skills/` 文件夹加载 skill，但仍然跳过其 `.claude/commands/` 和 `.claude/agents/` 文件夹。[来自其他目录的 Skills](https://code.claude.com/docs/zh-CN/skills.md#skills-from-additional-directories) 介绍了哪些内容会加载、哪些不会加载。

如果不使用 `--bare`，`-p` 会话会运行项目的 `.claude/settings.json` 中的 hook 并连接其 `.mcp.json` 中的服务器，即使在您从未信任过的文件夹中也是如此。`-p` 会话不会显示工作区信任对话框，也不会显示逐个服务器的批准提示。[在您信任文件夹之前运行的内容](https://code.claude.com/docs/zh-CN/permissions.md#what-runs-before-you-trust-a-folder) 介绍了 `-p` 下每种仓库内容以及如何将其排除在外。

此示例在 bare 模式下运行一次性摘要任务，并预先批准 Read 工具，以便调用无需权限提示即可完成。运行前请设置 `ANTHROPIC_API_KEY`，因为 bare 模式不使用您的订阅登录：

```bash theme={null}
claude --bare -p "Summarize README.md" --allowedTools "Read"
```

在 bare 模式下，Claude Code 从不读取 OAuth 凭据或系统密钥链。对于 Anthropic API，请在环境中设置 `ANTHROPIC_API_KEY`（使用在 [Claude Console](https://platform.claude.com) 中创建的密钥），或在 `--settings` JSON 中提供 `apiKeyHelper`。Amazon Bedrock、Google Cloud 的 Agent Platform 和 Microsoft Foundry 会继续照常读取各自的提供商凭据。

在 bare 模式下，Claude 可以使用 Bash、文件读取和文件编辑工具。请使用标志传递您需要的任何上下文：

| 要加载的内容 | 使用 |
| - | - |
| 系统提示词附加内容 | `--append-system-prompt`, `--append-system-prompt-file` |
| 设置 | `--settings <file-or-json>` |
| MCP 服务器 | `--mcp-config <file-or-json>` |
| [自定义 Agent](https://code.claude.com/docs/zh-CN/sub-agents.md#choose-the-subagent-scope) | `--agents <file-or-json>` |
| 插件 | `--plugin-dir <path>`, `--plugin-url <url>` |

bare 模式还会限制会话运行期间发生的事情：

* **MCP 服务器**：只有在命令行中提供的服务器才会连接，例如通过 `--mcp-config`。在交互式会话中，除非您传递 `--ide`，否则 Claude Code 还会跳过自动 IDE 连接。
* **系统提醒**：Claude 会收到您的提示词和工具结果，但不会收到 Claude Code 原本会随之添加的 [系统提醒](https://code.claude.com/docs/zh-CN/glossary.md#system-reminder)。例如，当 Claude 之前读取过的文件在磁盘上发生更改时，Claude 不会收到通知，也不会获得可用 skill 的列表，包括来自 `--add-dir` 文件夹的 skill。
* **后台任务**：Claude 无法运行后台命令或后台子代理，并且 [`CLAUDE_CODE_DISABLE_BACKGROUND_TASKS`](https://code.claude.com/docs/zh-CN/env-vars) 所关闭的其他功能也同样处于关闭状态。达到 [超时](https://code.claude.com/docs/zh-CN/tools-reference.md#timeout-and-output-limits) 的命令会停止，而不是 [转入后台](https://code.claude.com/docs/zh-CN/tools-reference.md#background-commands)。

在 v2.1.286 之前，这些限制只部分生效：交互式 `--bare` 会话会连接普通会话会连接的 MCP 服务器，每个 `--bare` 会话都会发送系统提醒，并且后台任务仍然可用。

> [!NOTE]
>
> `--bare` 是脚本和 SDK 调用的推荐模式，将在未来版本中成为 `-p` 的默认值。

### 退出时的后台任务

在 Claude 完成其轮次且 stdin 关闭后，`claude -p` 运行可以保持打开状态，以等待 Claude 启动的后台工作。

除非主对话启动的后台命令仍在运行，否则默认情况下，Claude Code 会在连续空闲等待 10 分钟后停止仍在运行的任何内容，并丢弃其部分结果。要更改 10 分钟上限，请设置 [`CLAUDE_CODE_PRINT_BG_WAIT_CEILING_MS`](https://code.claude.com/docs/zh-CN/env-vars)，或将其设置为 `0` 以在没有上限的情况下等待。

运行会等待后台工作，例如后台命令、子代理和工作流、Monitor 监视以及待处理的 `/loop` 唤醒：

* **[后台命令](https://code.claude.com/docs/zh-CN/tools-reference.md#background-commands)**：对于主对话启动的命令（例如开发服务器或监视构建），运行会等待，直到该命令退出或达到其 [时间限制](https://code.claude.com/docs/zh-CN/tools-reference.md#time-limit-for-background-commands)。随后 Claude 会根据结果再进行一轮。在命令运行期间，10 分钟上限不会结束等待。
* **后台[子代理](https://code.claude.com/docs/zh-CN/sub-agents.md)和工作流**：运行会保持打开状态，直到该工作完成，因为其结果是最终输出的一部分。
* **[Monitor](https://code.claude.com/docs/zh-CN/tools-reference.md#monitor-tool) 监视**：运行会等待，直到监视超时或 10 分钟上限结束等待，以先发生者为准。在等待期间，Claude 会继续响应监视报告的内容。默认情况下，监视在 Claude 启动后五分钟超时。
* **待处理的唤醒**：在以文本形式而非通过 `--input-format stream-json` 传递提示词的运行中，如果 Claude 已安排 [自定节奏的 `/loop` 唤醒](https://code.claude.com/docs/zh-CN/scheduled-tasks#let-claude-choose-the-interval)，运行会等待每次唤醒触发并执行其迭代，直到 [循环结束](https://code.claude.com/docs/zh-CN/scheduled-tasks.md#stop-a-loop)，即使超过 10 分钟上限也是如此。

当 stderr 是终端且运行已等待五秒时，Claude Code 会向 stderr 打印一行以 `Waiting for background work to finish` 开头的内容，并列出正在等待的工作。使用 [`json` 或 `stream-json` 输出](#get-structured-output) 时，该行仅在 stdout 不是终端时才会打印，因此您的脚本读取的 JSON 中永远不会包含它。

如果运行达到其 [`--max-budget-usd`](https://code.claude.com/docs/zh-CN/cli-reference#cli-flags) 上限，Claude Code 会停止剩余的后台工作，而不是继续等待。

当后台工作启动新的轮次时，使用默认的 `text` 输出时运行会打印每一轮的结果，使用 `json` 输出时则打印最后一轮的结果。在 v2.1.295 之前，使用 `text` 输出时运行也只打印最后一轮的结果。

#### 非交互式运行结束时的 MCP 工作

`claude -p` 运行不会等待在后台进行的 MCP 工作：

* **[MCP 任务](https://code.claude.com/docs/zh-CN/mcp.md#mcp-tasks-that-run-on-the-server)**：在远程服务器上，该作业会继续进行。要再次跟进它，请 [恢复该会话](https://code.claude.com/docs/zh-CN/mcp.md#exit-clear-or-resume-while-an-mcp-task-runs)，而不是以一次性 `claude -p "<prompt>"` 运行的方式。
* **被 [自动转入后台](https://code.claude.com/docs/zh-CN/mcp.md#automatic-backgrounding-of-long-tool-calls) 的 MCP 调用**：该调用的结果会丢失。

### 使用 SIGTERM 停止运行

如果您使用 SIGTERM 停止 `claude -p` 运行（例如使用 `kill` 或通过进程监督程序），Claude Code 会以代码 143 退出。Claude Code 会让正在进行的轮次保持未完成状态，并且不为其记录任何结果。如果要结束该轮次，请在停止进程之前发送 SIGINT，或调用 Agent SDK 的 `interrupt()`。

收到 SIGTERM 时，Claude Code 会终止仍在运行的任何 Bash 命令的进程树。然后 Claude Code 运行 [`SessionEnd` hook](https://code.claude.com/docs/zh-CN/hooks#sessionend) 并退出。在退出过程中，Claude Code 不会启动新的工具调用，不会发送新的模型请求，也不会运行除 `SessionEnd` 之外的任何 hook。如果信号到达时运行正在执行命令或等待权限提示，Claude Code 会按如下方式处理该步骤：

* **正在运行命令**：Claude Code 会在会话中将该命令记录为已终止。
* **正在等待权限提示的答复**：如果您向进程发送 SIGTERM，Claude Code 会让该提示保持未答复状态。如果您的程序通过 Agent SDK 关闭会话，SDK 会在发送任何信号之前结束 Claude Code 的输入，而 Claude Code 会在输入结束后立即取消该提示。

当您 [恢复会话](#continue-conversations) 时，Claude Code 会保留被中断的轮次原样，由您的下一个提示词推动对话。如果希望 Claude Code 在恢复时继续被中断的轮次，请设置 [`CLAUDE_CODE_RESUME_INTERRUPTED_TURN=1`](https://code.claude.com/docs/zh-CN/env-vars)。

### 如果工作目录被删除

如果 `claude -p` 或 Agent SDK 会话的工作目录在会话进行中被删除，会话会继续运行。当某个轮次在目录缺失时开始，Claude Code 会在 `stream-json` 输出中发出 [警告消息](https://code.claude.com/docs/zh-CN/agent-sdk/typescript.md#sdkinformationalmessage)，并且 shell 命令会失败，直到该目录重新存在。

## 示例

这些示例突出了常见的 CLI 模式。对于命名文件（如 `auth.py` 或 `build-error.txt`）的命令，请替换为您自己项目中的文件。在 CI 或其他脚本环境中，添加 [`--bare`](#start-faster-with-bare-mode)，以便 Claude Code 启动时不加载主机的 hook、插件、自动记忆或 `CLAUDE.md`。

### 通过 Claude 管道传输数据

非交互模式读取 stdin，因此您可以像任何其他命令行工具一样管道传输数据并重定向响应。

此示例将构建日志管道传输到 Claude 并将说明写入文件：

```bash theme={null}
cat build-error.txt | claude -p 'concisely explain the root cause of this build error' > output.txt
```

使用 `--output-format json` 时，响应的 JSON 数据包括 `total_cost_usd` 和按模型的成本分解，因此脚本调用者可以跟踪支出而无需查询 [使用情况仪表板](https://code.claude.com/docs/zh-CN/costs.md)。当您使用 `--continue` 或 `--resume` 继续较早的对话时，运行报告对话的整体总计，[包括较早运行的支出](https://code.claude.com/docs/zh-CN/agent-sdk/cost-tracking.md#accumulate-costs-across-multiple-calls)。两个数字都是 [客户端估计](https://code.claude.com/docs/zh-CN/agent-sdk/cost-tracking.md)，可能与您的实际账单不同。

> [!NOTE]
>
> 管道 stdin 的上限为 10MB。如果超过上限，Claude Code 会以清晰的错误和非零状态退出。要处理更大的输入，请将内容写入文件并在提示词中引用文件路径，而不是管道传输它。

如果 Claude Code 无法读取 stdin，例如因为启动它的进程断开了其端点，Claude Code 会向 stderr 打印警告并继续使用命令行中的提示词。

### 将 Claude 添加到构建脚本

您可以在脚本中包装非交互调用，以将 Claude 用作项目特定的 linter 或审查者。

此 `package.json` 脚本将针对 `main` 的 diff 管道传输到 Claude，并要求它报告拼写错误。管道传输 diff 意味着 Claude 不需要 Bash 权限来读取它，转义的双引号使脚本可移植到 Windows：

```json theme={null}
{
  "scripts": {
    "lint:claude": "git diff main | claude -p \"you are a typo linter. for each typo in this diff, report filename:line on one line and the issue on the next. return nothing else.\""
  }
}
```

使用 `npm run lint:claude` 运行它。

### 获取结构化输出

使用 `--output-format` 控制响应的返回方式：

* `text`（默认）：纯文本输出
* `json`：包含结果、会话 ID 和元数据的结构化 JSON
* `stream-json`：用于实时流式传输的换行符分隔的 JSON

此示例以 JSON 格式返回项目摘要以及会话元数据，文本结果在 `result` 字段中：

```bash theme={null}
claude -p "Summarize this project" --output-format json
```

要获得符合特定 schema 的输出，请使用 `--output-format json` 与 `--json-schema` 和 [JSON Schema](https://json-schema.org/) 定义。响应包括关于请求的元数据（会话 ID、使用情况等），结构化输出在 `structured_output` 字段中。

此示例从 auth.py 中提取函数名称并将其作为字符串数组返回：

```bash theme={null}
claude -p "Extract the main function names from auth.py" \
  --output-format json \
  --json-schema '{"type":"object","properties":{"functions":{"type":"array","items":{"type":"string"}}},"required":["functions"]}'
```

如果该值不是有效的 JSON Schema，`claude` 会以 `Error: --json-schema is not a valid JSON Schema` 退出，后跟验证器的诊断。Claude Code 接受使用 `format` 关键字的 schema，例如 `"format": "email"`，但将 `format` 视为注释，不强制执行它。

如果运行结束时没有有效的结构化输出，例如因为 Claude 拒绝了该请求，则响应中没有 `structured_output` 字段。在脚本使用 `structured_output` 之前，请检查该字段是否存在。

> [!TIP]
>
> 使用 [jq](https://jqlang.org/) 之类的工具来解析响应并提取特定字段：
>
> ```bash theme={null}
> # Extract the text result
> claude -p "Summarize this project" --output-format json | jq -r '.result'
>
> # Extract structured output
> claude -p "Extract function names from auth.py" \
>   --output-format json \
>   --json-schema '{"type":"object","properties":{"functions":{"type":"array","items":{"type":"string"}}},"required":["functions"]}' \
>   | jq '.structured_output'
> ```

### 流式传输响应

使用 `--output-format stream-json` 与 `--verbose` 和 `--include-partial-messages` 来接收生成的 token。每一行都是代表一个事件的 JSON 对象：

```bash theme={null}
claude -p "Explain recursion" --output-format stream-json --verbose --include-partial-messages
```

流的最后一行是包含最终响应文本、成本和会话元数据的 `result` 消息。

如果您的消费者缓慢读取流，Claude Code 会等待队列中的输出排空后再退出，根据仍然队列中的数量缩放等待时间，上限为 30 秒。

以下示例使用 [jq](https://jqlang.org/) 过滤文本增量，仅显示流式文本。`-r` 标志输出原始字符串（无引号），`-j` 不带换行符地连接，以便 token 连续流式输出：

```bash theme={null}
claude -p "Write a poem" --output-format stream-json --verbose --include-partial-messages | \
  jq -rj 'select(.type == "stream_event" and .event.delta.type? == "text_delta") | .event.delta.text'
```

对于具有回调和消息对象的编程流式传输，请参阅 Agent SDK 文档中的 [实时流式传输响应](https://code.claude.com/docs/zh-CN/agent-sdk/streaming-output.md)。

#### 跟踪子代理消息

来自[子代理](https://code.claude.com/docs/zh-CN/sub-agents.md)以及[在子代理中运行](https://code.claude.com/docs/zh-CN/skills.md#run-skills-in-a-subagent)的 skill 的消息在流中显示为 `assistant` 和 `user` 消息。其 `parent_tool_use_id` 字段表明每条消息属于哪次运行。来自主对话的消息在该字段中为 `null`。

来自 forked skill 或在[前台](https://code.claude.com/docs/zh-CN/sub-agents.md#run-subagents-in-foreground-or-background)运行的子代理的第一条消息是一条 `user` 消息，携带驱动它的提示词或 skill 内容。在该第一条消息之后，Claude Code 发出：

* **默认情况下**：该次运行的 `tool_use` 和 `tool_result` 块。
* **使用 [`--forward-subagent-text`](https://code.claude.com/docs/zh-CN/cli-reference#cli-flags) 或 [`CLAUDE_CODE_FORWARD_SUBAGENT_TEXT`](https://code.claude.com/docs/zh-CN/env-vars) 时**：还包括该次运行的文本和思考块，因此您可以重建每次运行的会话记录。

当您启用任一选项时，Claude Code 会转发来自[每个嵌套深度的子代理](https://code.claude.com/docs/zh-CN/sub-agents.md#let-subagents-spawn-their-own-subagents)的消息，无论每个子代理是使用 Agent 工具生成的还是作为 forked skill 启动的。在 `parent_tool_use_id` 中，嵌套子代理的消息携带启动它的 Agent 或 Skill 工具调用的 ID，因此您可以通过跟踪这些 ID 来重建完整的嵌套树。

由 Claude 通过工具调用启动的运行携带该工具调用的 ID。通过将 `/<skill-name>` 作为提示词传递而启动的 forked skill 没有工具调用，因此其消息改为携带 `forked-command-` 值，并在其完成后到达。请在第一列中查找运行的启动方式：

| 运行的启动方式 | `parent_tool_use_id` | 其消息何时到达 |
| :- | :- | :- |
| Claude 从主对话调用 Agent 工具 | 该 Agent `tool_use` 块的 ID | 在子代理工作期间 |
| Claude 从主对话为 forked skill 调用 Skill 工具 | 该 Skill `tool_use` 块的 ID | 在 forked skill 工作期间 |
| 您将 `/<skill-name>` 作为提示词传递 | 以 `forked-command-` 开头的值 | 在 forked skill 完成后一起按顺序到达 |

对于从提示词启动的 forked skill，请按 `forked-command-` 前缀匹配 `parent_tool_use_id`，因为其后的名称可能与您输入的名称不同。

如果流中缺少其中某些消息，请将您的 Claude Code 版本与以下最低版本进行对照：

* **Claude 从主对话使用 Skill 工具启动的 forked skill**：其 `tool_use` 和 `tool_result` 块需要 v2.1.86 或更高版本，其第一条 `user` 消息以及文本和思考块需要 v2.1.265 或更高版本
* **forked skill 生成的子代理的消息，以及在子代理或另一个 forked skill 内启动的 forked skill 的消息**：v2.1.275 或更高版本
* **通过将 `/<skill-name>` 作为提示词传递而启动的 forked skill 的消息**：v2.1.287 或更高版本

#### 处理 API 重试

当 API 请求因可重试错误而失败时，Claude Code 在重试前发出 `system/api_retry` 事件。在 v2.1.246 或更高版本上，当 `401` 或 `403` 拒绝 [`apiKeyHelper`](https://code.claude.com/docs/zh-CN/settings-reference#apikeyhelper) 凭据时，Claude Code 会静默进行前两次重试，没有事件，然后从第三次连续重试开始照常发出事件。静默重试仍然计入 `attempt`。您可以使用该事件在您自己的界面中显示重试进度。

| 字段 | 类型 | 描述 |
| - | - | - |
| `type` | `"system"` | 消息类型 |
| `subtype` | `"api_retry"` | 将其标识为重试事件 |
| `attempt` | 整数 | 当前尝试次数，从 1 开始 |
| `max_retries` | 整数 | 针对此失败原因允许的总重试次数 |
| `retry_delay_ms` | 整数 | 距下一次尝试的毫秒数 |
| `error_status` | 整数或 null | 失败尝试的 HTTP 状态代码，当尝试未从 API 获得 HTTP 响应时为 `null` |
| `no_response` | 对象，可选 | 仅当失败的尝试 [未及时获得响应头](https://code.claude.com/docs/zh-CN/errors.md#no-response-from-api) 时存在。`waited_ms` 是该尝试等待的时间，`retry_wait_ms` 是重试将等待的时间。需要 Claude Code v2.1.261 或更高版本 |
| `error` | 字符串 | 错误类别：`authentication_failed`、`oauth_org_not_allowed`、`account_on_hold`、`billing_error`、`rate_limit`、`overloaded`、`invalid_request`、`model_not_found`、`server_error`、`max_output_tokens`、`cloud_credential_error` 或 `unknown` |
| `uuid` | 字符串 | 唯一事件标识符 |
| `session_id` | 字符串 | 事件所属的会话 |

#### 读取会话元数据

`system/init` 事件报告会话元数据，包括模型、工具、MCP 服务器和加载的插件。它是流中的第一个事件，除非启动事件在其之前：

* `plugin_install` 事件，当设置了 [`CLAUDE_CODE_SYNC_PLUGIN_INSTALL`](https://code.claude.com/docs/zh-CN/env-vars) 时。
* [`hook_started`、`hook_progress` 和 `hook_response` 事件](https://code.claude.com/docs/zh-CN/agent-sdk/typescript#sdkhookstartedmessage)，当配置的 [`SessionStart`](https://code.claude.com/docs/zh-CN/hooks#sessionstart) 或 [`Setup`](https://code.claude.com/docs/zh-CN/hooks#setup) hook 运行时。这些事件在 hook 生成时流式传输。

该事件还携带一个可选的 `capabilities` 字符串数组，命名此 Claude Code 版本实现的协议行为，例如 `interrupt_receipt_v1` 或 `interrupt_cancel_queued_v1`。检查它以进行功能检测，而不是比较版本字符串，并忽略您不认识的值。有关功能列表，请参阅 [`SDKSystemMessage`](https://code.claude.com/docs/zh-CN/agent-sdk/typescript#sdksystemmessage)。

#### 当插件或 MCP 服务器未加载时使 CI 失败

使用 `system/init` 事件中的插件字段来捕获未加载的插件：

| 字段 | 类型 | 描述 |
| - | - | - |
| `plugins` | 数组 | 成功加载的插件，每个都有 `name` 和 `path` |
| `plugin_errors` | 数组 | 插件加载时错误，每个都有 `plugin`、`type` 和 `message`。包括不满足的依赖版本和 `--plugin-dir` 加载失败，例如缺失路径或无效存档。未加载的插件不会出现在 `plugins` 中。当没有错误时，该键被省略 |

当 `--plugin-dir` 目录或存档本身加载失败时，其 `plugin_errors` 条目包括解析的绝对路径作为 `path`。使用它来判断哪个 `--plugin-dir` 值失败。`path` 字段需要 Claude Code v2.1.283 或更高版本。

以相同的方式使用 MCP 服务器字段。当您使用 `-p` 传递 [`--mcp-config`](https://code.claude.com/docs/zh-CN/cli-reference#cli-flags) 时，Claude Code 在运行第一轮之前等待仍然待处理的服务器，最多等待 [`MCP_TIMEOUT`](https://code.claude.com/docs/zh-CN/env-vars) 启动超时时间，默认为 30 秒。具有 [缓存工具列表](https://code.claude.com/docs/zh-CN/agent-sdk/mcp.md#connection-timing) 的远程服务器跳过等待，在 `system/init` 中显示 `pending`，并在其第一次工具调用时连接。在 [自托管环境](https://code.claude.com/docs/zh-CN/self-hosted-environments-configuration.md#connection-timing) 中，改为适用较短的等待时间。等待需要 Claude Code v2.1.221 或更高版本。

Claude Code 在启动时验证每个 `--mcp-config` 条目并跳过验证失败的条目，例如没有 `type` 的 `url` 条目。运行继续并干净地退出，因此检查这些字段以捕获从未加载的服务器：

| 字段 | 类型 | 描述 |
| - | - | - |
| `mcp_servers` | 数组 | 会话中的 MCP 服务器，每个都有 `name` 和 `status` |
| `mcp_server_errors` | 数组 | 被配置验证跳过的 `--mcp-config` 条目，每个都有 `name`、`type` 和 `message`。`type` 是跳过类别，例如 `unknown_type`、`url_missing_type`、`invalid_config` 或 `reserved_name`；将您不认识的值视为通用跳过。受影响的服务器不会出现在 `mcp_servers` 中。当没有错误时，该键被省略，因此 CI 门可以在数组非空时失败 |

当您在终端中手动运行命令时，Claude Code 也会向 stderr 打印启动警告，例如 `Warning: 1 MCP server skipped due to invalid config:`，后跟每个跳过条目的原因。

当您重定向 stderr，或当 CI 运行器或 SDK 主机等程序捕获它时，Claude Code 不打印警告。它仅在 `mcp_server_errors` 字段中报告跳过的条目。

#### 跟踪插件安装

当设置了 [`CLAUDE_CODE_SYNC_PLUGIN_INSTALL`](https://code.claude.com/docs/zh-CN/env-vars) 时，Claude Code 在第一轮之前安装市场插件时发出 `system/plugin_install` 事件。使用这些事件在您自己的 UI 中显示安装进度。

| 字段 | 类型 | 描述 |
| - | - | - |
| `type` | `"system"` | 消息类型 |
| `subtype` | `"plugin_install"` | 将其标识为插件安装事件 |
| `status` | `"started"`、`"installed"`、`"failed"` 或 `"completed"` | `started` 和 `completed` 标记整体安装的开始和结束；`installed` 和 `failed` 报告单个市场 |
| `name` | 字符串，可选 | 市场名称，在 `installed` 和 `failed` 上存在 |
| `error` | 字符串，可选 | 失败消息，在 `failed` 上存在 |
| `uuid` | 字符串 | 唯一事件标识符 |
| `session_id` | 字符串 | 事件所属的会话 |

### 自动批准工具

使用 `--allowedTools` 让 Claude 使用某些工具而无需提示。列出 `Read` 和 `Edit` 让 Claude 读取和编辑文件而无需请求权限，但从 [网络路径](https://code.claude.com/docs/zh-CN/permissions.md#network-paths) 读取除外。列出 `Bash` 对 shell 命令执行相同操作，但在 [自动模式](https://code.claude.com/docs/zh-CN/permission-modes.md#how-auto-mode-evaluates-actions) 中启动的运行除外，此时 Claude Code 会将裸 `Bash` 条目作为宽泛的允许规则丢弃，改由自动模式评估每个命令。此示例运行测试套件并修复失败，列出了这三个工具：

```bash theme={null}
claude -p "Run the test suite and fix any failures" \
  --allowedTools "Bash,Read,Edit"
```

对未列出工具的调用如果无需批准，则会运行。在 [Manual 模式](https://code.claude.com/docs/zh-CN/permission-modes.md#available-modes) 的 `claude -p` 运行中，需要批准的调用会被拒绝，除非有机制批准它，例如 [`PermissionRequest` hook](https://code.claude.com/docs/zh-CN/hooks#permissionrequest) 或 [`--permission-prompt-tool`](https://code.claude.com/docs/zh-CN/cli-reference#cli-flags)。在 Claude Code v2.1.302 或更高版本中，当 [没有任何设置指定权限模式](https://code.claude.com/docs/zh-CN/permission-modes.md#which-mode-a-non-interactive-run-starts-in) 时，`claude -p` 运行以 Manual 模式启动。

要为整个会话设置基线而不是列出单个工具，请使用 `--permission-mode` 传递 [权限模式](https://code.claude.com/docs/zh-CN/permission-modes.md)，例如 `auto`、`dontAsk` 或 `acceptEdits`：

* **`auto`**：传递 `--permission-mode auto` 以让分类器代替您审查大多数操作
* **`dontAsk`**：Claude Code 拒绝每个原本会提示的调用，这对于锁定的 CI 运行很有用，但您安装的 [mod](https://code.claude.com/docs/zh-CN/plugins/mods/overview.md) 仍然可以批准其中某些调用。要查看 mod 可以批准哪些调用，请参阅 [使用 hook 扩展权限](https://code.claude.com/docs/zh-CN/permissions.md#precedence-among-rules-hooks-and-mods)。

  在 Manual 模式下无需批准的操作仍会运行，例如工作目录中的文件读取和 [只读命令集](https://code.claude.com/docs/zh-CN/permissions.md#read-only-commands)。您的 `--allowedTools` 条目或 `permissions.allow` 规则涵盖的操作也会运行。`AskUserQuestion`、[您的组织设置为 `ask`](https://code.claude.com/docs/zh-CN/mcp#organization-controls-on-connector-tools) 的连接器工具、标记为 [`requiresUserInteraction`](https://code.claude.com/docs/zh-CN/mcp#require-approval-for-a-specific-tool) 的 MCP 工具以及 [从网络路径读取](https://code.claude.com/docs/zh-CN/permissions.md#network-paths) 即使在允许规则匹配时也会被拒绝
* **`acceptEdits`**：Claude 写入文件而无需提示，Claude Code 自动批准常见的文件系统命令，例如 `mkdir`、`touch`、`mv` 和 `cp`。[任何模式都不自动批准的操作](https://code.claude.com/docs/zh-CN/permission-modes.md#actions-no-mode-auto-approves) 仍然适用。除了只读命令集，其他 shell 命令和网络请求仍然需要 `--allowedTools` 条目或 `permissions.allow` 规则。有关完整列表，请参阅 [`acceptEdits` 自动批准的内容](https://code.claude.com/docs/zh-CN/permission-modes#auto-approve-file-edits-with-acceptedits-mode)

此示例使用 `acceptEdits` 作为基线应用 lint 修复：

```bash theme={null}
claude -p "Apply the lint fixes" --permission-mode acceptEdits
```

### 在无人值守运行中关闭权限提示

当没有人可用来回答权限提示时，传递 `--permission-prompts none`，例如在计划的作业中。当您的运行有权限主机时，该标志最重要：具有 [`canUseTool` 回调](https://code.claude.com/docs/zh-CN/agent-sdk/user-input) 的 Agent SDK 应用，或您使用 [`--permission-prompt-tool`](https://code.claude.com/docs/zh-CN/cli-reference#cli-flags) 传递的 MCP 工具。没有该标志，您的运行会等待该主机回答每个权限请求。

使用该标志，您的运行不会查询主机或等待它。任何会提示的内容都被拒绝，除非 `PermissionRequest` hook 允许它，Claude 被告知没有人可以批准请求且不要重试它，运行继续。在没有主机的 `-p` 运行中，这些请求无论如何都被拒绝，该标志也告诉 Claude 不要重试它们。权限规则、[`PermissionRequest` hook](https://code.claude.com/docs/zh-CN/hooks#permissionrequest) 和您设置的权限模式仍然首先决定每个调用；Claude Code 仅拒绝其他任何机制都无法解决的请求。

此示例在 [自动模式](https://code.claude.com/docs/zh-CN/permission-modes.md#eliminate-prompts-with-auto-mode) 中运行无人值守任务。分类器照常审查每个操作，Claude Code 拒绝任何会回退到提示的内容：

```bash theme={null}
claude -p "Update the dependency pins and run the tests" --permission-mode auto --permission-prompts none
```

使用 `--permission-prompts none`，Claude Code 移除需要来自人的答案的工具，例如 [`AskUserQuestion`](https://code.claude.com/docs/zh-CN/tools-reference#askuserquestion-tool-behavior)，因此 Claude 无法调用它们。任何没有 [`Elicitation` hook](https://code.claude.com/docs/zh-CN/hooks#elicitation) 回答的 [MCP 引出请求](https://code.claude.com/docs/zh-CN/mcp.md#respond-to-mcp-elicitation-requests) 都被取消。

使用 `--output-format stream-json`，拒绝显示为 `permission_denied` 系统消息，最终结果消息在 `permission_denials` 中列出它们。

> [!NOTE]
>
> `--permission-prompts` 标志需要 Claude Code v2.1.259 或更高版本。早期版本以未知选项错误拒绝它。

### 创建提交

此示例审查暂存的更改并创建具有适当提交信息的提交：

```bash theme={null}
claude -p "Look at my staged changes and create an appropriate commit" \
  --allowedTools "Bash(git diff *),Bash(git log *),Bash(git status *),Bash(git commit *)"
```

`--allowedTools` 标志使用 [权限规则语法](https://code.claude.com/docs/zh-CN/settings-reference.md#permission-rule-syntax)。尾部的 ` *` 启用前缀匹配，因此 `Bash(git diff *)` 允许任何以 `git diff` 开头的命令。`*` 之前的空格很重要：没有它，`Bash(git diff*)` 也会匹配 `git diff-index`。

> [!NOTE]
>
> 命令支持在 `-p` 模式下有所不同：
>
> * 用户调用的 [skill](https://code.claude.com/docs/zh-CN/skills.md) 和自定义命令可以使用。在提示词字符串中包含 `/skill-name`，Claude Code 会在运行前展开它。
> * 仅在终端界面中运行的内置命令，例如 `/login`，不可用。
> * `/model`、`/effort`、`/fast`、`/color` 和 `/rename` 接受该值作为参数，例如 `/model sonnet`，`/mcp` 不带参数时打印服务器状态的文本摘要。这些形式遵循每个命令的 [可用性说明](https://code.claude.com/docs/zh-CN/commands.md#all-commands)。
> * 要更改设置，请将 `key=value` 传递给 `/config`，例如 `/config thinking=false`。
> * `/output-style <style>` 切换 [输出样式](https://code.claude.com/docs/zh-CN/output-styles.md)，单独使用 `/output-style` 会列出它们。需要 Claude Code v2.1.269 或更高版本。

### 自定义系统提示词

使用 `--append-system-prompt` 添加指令同时保持 Claude Code 的默认行为。此示例将 PR diff 传递给 Claude 并指示它审查安全漏洞。将其保存为 shell 脚本，例如 `review.sh`：

```bash theme={null}
gh pr diff "$1" | claude -p \
  --append-system-prompt "You are a security engineer. Review for vulnerabilities." \
  --output-format json
```

在脚本中，`"$1"` 代表您在命令行上传递的第一个参数。运行 `bash review.sh 123`，shell 将 `"$1"` 替换为 `123`，因此脚本获取 PR 123 的 diff。Claude Code 将审查打印为 JSON，文本在 `result` 字段中。

有关更多选项（包括用于完全替换默认提示词的 `--system-prompt`），请参阅 [系统提示词标志](https://code.claude.com/docs/zh-CN/cli-reference.md#system-prompt-flags)。如果替换默认提示词，运行启动时也不会包含 [git 状态快照](https://code.claude.com/docs/zh-CN/settings-reference.md#includegitinstructions)。

### 继续对话

使用 `--continue` 继续最近的对话，或使用 `--resume` 与会话 ID 继续特定对话。在 Claude Code v2.1.257 或更高版本上，当您传递 `--continue` 时，Claude Code 会打开已完成的 [后台会话](https://code.claude.com/docs/zh-CN/sessions.md#where-the-session-picker-looks)，但不会打开仍在运行的后台会话。此示例运行审查，然后发送后续提示词：

```bash theme={null}
# First request
claude -p "Review this codebase for performance issues"

# Continue the most recent conversation
claude -p "Now focus on the database queries" --continue
claude -p "Generate a summary of all issues found" --continue
```

如果您运行多个对话，请捕获会话 ID 以恢复特定对话：

```bash theme={null}
session_id=$(claude -p "Start a review" --output-format json | jq -r '.session_id')
claude -p "Continue that review" --resume "$session_id"
```

您可以从不同的目录运行这两个命令：Claude Code 会在此机器上的任何项目中 [按其 ID 查找会话](https://code.claude.com/docs/zh-CN/sessions.md#where-the-session-picker-looks)。

您可以向 `--resume` 传递会话的 `.jsonl` [会话记录文件](https://code.claude.com/docs/zh-CN/sessions.md#where-transcripts-are-stored) 的绝对路径来代替会话 ID，Claude Code 会继续存储在该文件中的对话。

## 后续步骤

* [Agent SDK 快速入门](https://code.claude.com/docs/zh-CN/agent-sdk/quickstart.md)：使用 Python 或 TypeScript 构建您的第一个 agent
* [CLI 参考](https://code.claude.com/docs/zh-CN/cli-reference.md)：所有 CLI 标志和选项
* [GitHub Actions](https://code.claude.com/docs/zh-CN/github-actions.md)：在 GitHub 工作流中使用 Agent SDK
* [GitLab CI/CD](https://code.claude.com/docs/zh-CN/gitlab-ci-cd.md)：在 GitLab 管道中使用 Agent SDK
