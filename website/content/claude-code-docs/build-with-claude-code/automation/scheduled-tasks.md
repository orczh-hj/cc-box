> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# 按计划运行提示词

> 使用 /loop 和 cron 调度工具在 Claude Code 会话中重复运行提示词、轮询状态或设置一次性提醒。

会话范围的定时任务让 Claude 在 Claude Code 会话中按间隔自动重新运行提示词。使用它们来轮询部署、监督 PR、检查长时间运行的构建，或在会话中稍后提醒自己做某事。

> [!NOTE]
>
> 以下情况在其他页面中介绍：
>
> * **对事件进行实时反应而不是轮询**：请参阅 [Channels](https://code.claude.com/docs/zh-CN/channels.md)。您的 CI 可以直接将失败推送到会话中。
> * **让会话逐轮次持续工作直到满足某个条件，而不是按间隔运行**：请参阅 [`/goal`](https://code.claude.com/docs/zh-CN/goal)。

任务是会话范围的。当您使用 `--resume` 或 `--continue` 恢复时，Claude Code 会恢复尚未[过期](#seven-day-expiry)的任务，除了[限制](#limitations)下列出的任务。对于独立于任何会话而存在的调度，请创建[云端定时任务](https://code.claude.com/docs/zh-CN/routines.md)、设置 [Desktop 定时任务](https://code.claude.com/docs/zh-CN/desktop-scheduled-tasks.md)，或使用 [GitHub Actions](https://code.claude.com/docs/zh-CN/github-actions.md)。

## 比较调度选项

Claude Code 提供三种方式来安排定期或一次性工作：

| | [Cloud](https://code.claude.com/docs/zh-CN/routines.md) | [Desktop](https://code.claude.com/docs/zh-CN/desktop-scheduled-tasks.md) | [`/loop`](https://code.claude.com/docs/zh-CN/scheduled-tasks) |
| :- | :- | :- | :- |
| 运行位置 | Cloud，默认由 Anthropic 管理 | 您的机器 | 您的机器 |
| 需要机器开启 | 否 | 是 | 是 |
| 需要打开会话 | 否 | 否 | 是 |
| 重启后持久化 | 是 | 是 | 在 `--resume` 上恢复，有[例外](https://code.claude.com/docs/zh-CN/scheduled-tasks.md#limitations) |
| 访问本地文件 | 否（新克隆） | 是 | 是 |
| MCP servers | 每个任务配置的连接器 | [配置文件](https://code.claude.com/docs/zh-CN/mcp.md)和连接器 | 从会话继承 |
| 权限提示 | 默认为[自动模式](https://code.claude.com/docs/zh-CN/routines.md#permission-prompts-during-a-run) | 每个任务可配置 | 从会话继承 |
| 可自定义的计划 | 是 | 是 | 是 |
| 最小间隔 | 1 小时 | 由 Claude 设置时为 1 分钟，[在表单中为 1 小时](https://code.claude.com/docs/zh-CN/desktop-scheduled-tasks.md#schedule-options) | 1 分钟 |

> [!TIP]
>
> 对于应该在没有您的机器的情况下可靠运行的工作，使用**云任务**。当您需要访问本地文件和工具时，使用**桌面任务**。对于会话期间的快速轮询，使用 **`/loop`**。

## 使用 /loop 重复运行提示词

`/loop` [随附 skill](https://code.claude.com/docs/zh-CN/commands.md) 是在会话保持打开期间重复运行提示词的最快方式。间隔和提示词都是可选的，您提供的内容决定了循环的行为方式。

| 您提供的内容 | 示例 | 会发生什么 |
| :- | :- | :- |
| 间隔和提示词 | `/loop 5m check the deploy` | 您的提示词按[固定计划](#run-on-a-fixed-interval)运行 |
| 仅提示词 | `/loop check the deploy` | 您的提示词在每次迭代时按 [Claude 选择的间隔](#let-claude-choose-the-interval)运行 |
| 仅间隔，或什么都不提供 | `/loop` | 运行[内置维护提示词](#run-the-built-in-maintenance-prompt)，如果存在 `loop.md`，则运行该文件 |

您也可以将 skill 作为提示词传入，例如 `/loop 20m /review-pr 1234`，以便在每次迭代时重新运行该 skill。定时触发只会运行 Claude [可以自行调用](https://code.claude.com/docs/zh-CN/skills.md#control-who-invokes-a-skill)的 skill。以下内容会以纯文本形式传给 Claude，而不会被执行：

* 内置命令，例如 `/permissions`、`/model` 或 `/clear`
* 标记为 [`disable-model-invocation: true`](https://code.claude.com/docs/zh-CN/skills#frontmatter-reference) 的 skill，包括随附的 `/verify` skill
* 通过 [`skillOverrides`](https://code.claude.com/docs/zh-CN/skills#override-skill-visibility-from-settings) 设置或 `Skill` [拒绝规则](https://code.claude.com/docs/zh-CN/skills.md#restrict-claude’s-skill-access)对 Claude 隐藏的 skill
* [MCP 提示词](https://code.claude.com/docs/zh-CN/mcp.md#use-mcp-prompts-as-commands)，例如 `/mcp__github__list_prs`

### 按固定间隔运行

当您提供间隔时，Claude 会将其转换为 cron 表达式，安排该任务，并确认执行频率和任务 ID。

```text theme={null}
/loop 5m check if the deployment finished and tell me what happened
```

间隔可以作为单独的标记（如 `30m`）放在提示词之前，也可以作为从句（如 `every 2 hours`）放在提示词之后。支持的单位有：`s` 表示秒，`m` 表示分钟，`h` 表示小时，`d` 表示天。

由于 cron 的粒度为一分钟，秒数会向上取整到最接近的分钟。无法映射为整齐 cron 步长的间隔（例如 `7m` 或 `90m`）会被舍入到最接近的可映射间隔，Claude 会告诉您它选择了哪个间隔。

### 让 Claude 选择间隔

当您省略间隔时，Claude 会动态选择间隔，而不是按固定的 cron 计划运行。每次迭代后，它会根据观察到的情况选择一分钟到一小时之间的延迟：在构建即将完成或 PR 处于活跃状态时等待较短时间，在没有待处理事项时等待较长时间。所选的延迟及其原因会在每次迭代结束时输出。

以下示例会检查 CI 和评审意见，一旦 PR 安静下来，Claude 会在迭代之间等待更长时间：

```text theme={null}
/loop check whether CI passed and address any review comments
```

在 [Monitor 工具可用](https://code.claude.com/docs/zh-CN/tools-reference.md#monitor-tool)的会话中，当您请求动态 `/loop` 计划时，Claude 可能会直接使用该工具。Monitor 会运行后台脚本并将每一行输出流式传回，从而完全避免轮询，并且通常比按间隔重新运行提示词更节省 token、响应更及时。

动态安排的循环会像其他任务一样出现在您的[定时任务列表](#manage-scheduled-tasks)中，因此您可以用相同的方式列出或取消它。列表将其待处理的唤醒显示为一次性任务，因此安排在 `:00` 或 `:30` 的唤醒可能会[稍早运行](#when-a-one-shot-task-runs-early)。[七天过期](#seven-day-expiry)规则同样适用于该循环。

> [!NOTE]
>
> 动态选择的间隔和[内置维护提示词](#run-the-built-in-maintenance-prompt)适用于所有提供商，并且在关闭[功能标志获取](https://code.claude.com/docs/zh-CN/env-vars.md#features-that-need-feature-flag-fetching)时同样可用。在 Amazon Bedrock、Claude Platform on AWS、Google Cloud 的 Agent Platform 和 Microsoft Foundry 上，或在关闭获取的情况下，这两项功能都需要 Claude Code v2.1.248 或更高版本。

### 运行内置维护提示词

当您省略提示词时，Claude 会使用内置维护提示词，而不是您提供的提示词。在每次迭代中，它会按顺序处理以下事项：

* 继续对话中任何未完成的工作
* 处理当前分支的 Pull Request：评审意见、失败的 CI 运行、合并冲突
* 在没有其他待处理事项时，执行清理工作，例如查找 bug 或简化代码

Claude 不会在该范围之外发起新的工作，并且推送或删除等不可逆操作只有在延续会话记录中已授权的内容时才会执行。

```text theme={null}
/loop
```

单独的 `/loop` 会以[动态选择的间隔](#let-claude-choose-the-interval)运行此提示词。添加间隔（例如 `/loop 15m`）即可改为按固定计划运行。要用您自己的默认提示词替换内置提示词，请参阅[使用 loop.md 自定义默认提示词](#customize-the-default-prompt-with-loop-md)。

### 使用 loop.md 自定义默认提示词

创建 `loop.md` 文件，即可用您自己的指令替换[内置维护提示词](#run-the-built-in-maintenance-prompt)。它为单独的 `/loop` 定义单个默认提示词，而不是一组独立的定时任务，并且每当您在命令行中提供提示词时，Claude Code 都会忽略该文件。要在其之外安排其他提示词，请使用 `/loop <prompt>` 或[直接询问 Claude](#manage-scheduled-tasks)。

Claude 会在两个位置查找该文件，并使用找到的第一个文件。

| 路径 | 作用域 |
| :- | :- |
| `.claude/loop.md` | 项目级。两个文件都存在时优先使用。 |
| `~/.claude/loop.md` | 用户级。适用于任何未定义自己的文件的项目。 |

该文件是纯 Markdown，没有必需的结构。像直接输入 `/loop` 提示词一样编写即可。以下示例用于保持发布分支的健康状态：

```markdown title=".claude/loop.md" theme={null}
Check the `release/next` PR. If CI is red, pull the failing job log,
diagnose, and push a minimal fix. If new review comments have arrived,
address each one and resolve the thread. If everything is green and
quiet, say so in one line.
```

对 `loop.md` 的编辑会在下一次迭代时生效，因此您可以在循环运行期间完善指令。当两个位置都不存在 `loop.md` 时，循环会回退到内置维护提示词。请保持文件简洁：超过 25,000 字节的内容会被截断。

### 停止循环

* **[自定节奏的循环](#let-claude-choose-the-interval)**：在循环等待下一次迭代时，按 `Esc` 即可停止它。在 Claude 空闲时，您也可以按 `Ctrl+C` 停止它，但在[您已附加到的后台会话](#detach-without-stopping-a-loop)中除外。`Pending wakeup cancelled` 通知会确认该循环不会再次触发。
* **[固定间隔的循环](#run-on-a-fixed-interval)**：要停止 `/loop 5m` 之类的循环，请让 Claude 取消它，例如 `cancel the deploy check loop`。`Esc` 和 `Ctrl+C` 无法停止它，也无法停止任何您[通过直接询问 Claude 安排的任务](#manage-scheduled-tasks)。未取消的循环会在[七天后](#seven-day-expiry)结束。

在[自定节奏模式](#let-claude-choose-the-interval)下，Claude 也可以在任务完成后自行结束循环。Claude 会使用 `stop: true` 调用 [`ScheduleWakeup` 工具](https://code.claude.com/docs/zh-CN/tools-reference)，从而立即取消待处理的唤醒。如果某次迭代结束时既没有重新安排也没有停止，Claude Code 会在大约 20 分钟后安排一次备用唤醒，如果该次迭代也没有重新安排，则结束循环。

#### 分离而不停止循环

当您[附加到后台会话](https://code.claude.com/docs/zh-CN/agent-view.md#attach-to-a-session)且 Claude 处于空闲状态时，在空输入框中按两次 `Ctrl+C` 即可分离。在那里等待下一次迭代的自定节奏循环会继续运行，且该次迭代会按计划开始。如果要停止循环，请在分离之前按 `Esc`。使用 `Ctrl+C` 分离而不停止循环需要 Claude Code v2.1.295 或更高版本。

## 设置一次性提醒

对于一次性提醒，用自然语言描述您想要的内容，而不是使用 `/loop`。Claude 计划一个单次触发的任务，该任务在运行后删除自己。

```text theme={null}
remind me at 3pm to push the release branch
```

```text theme={null}
in 45 minutes, check whether the integration tests passed
```

Claude 使用 cron 表达式将触发时间固定到特定的分钟和小时，并确认何时触发。

## 管理会话范围的定时任务

用自然语言要求 Claude 列出或取消任务，或直接引用底层工具。

```text theme={null}
what scheduled tasks do I have?
```

```text theme={null}
cancel the deploy check job
```

这些是 Claude 使用的底层工具：

| 工具 | 目的 |
| :- | :- |
| `CronCreate` | 计划新任务。接受 5 字段 cron 表达式、要运行的提示词以及是否重复或仅触发一次。 |
| `CronList` | 列出定时任务及其 ID、计划、[下次运行时间](#jitter)和提示词。 |
| `CronDelete` | 按 ID 取消任务。 |

每个计划任务都有一个 8 字符的 ID，您可以将其传递给 `CronDelete`。一个会话最多可以同时保存 50 个计划任务。

## 会话范围的定时任务如何运行

调度程序每秒检查一次到期的任务，并以低优先级将其加入队列。计划的提示词在您的回合之间触发，而不是在 Claude 正在响应时。如果 Claude 在任务到期时忙碌，提示词会等到当前回合结束。

所有时间都在您的本地时区中解释。像 `0 9 * * *` 这样的 cron 表达式意味着 9am 在您运行 Claude Code 的任何地方，而不是 UTC。

### 抖动

定时任务的实际运行时间可能与其计划时间不同。如果每个会话的任务都严格按计划运行，许多任务会在同一时刻调用 API，因此 Claude Code 会调整每个任务的运行时间。重复任务会延后运行，但每五分钟运行一次的任务（写作 `*/5 * * * *`）除外。安排在整点或半点的一次性任务会稍微提前运行。

要查看任务下次到期的时间，请查看终端中 Claude 的 `CronCreate` 调用下方的行。该行会在计划之后显示该时间，例如 `next run 10:21 AM`。之后如需再次查看，请[让 Claude 列出您的任务](#manage-scheduled-tasks)，并在 `CronList` 调用下方的行中查找 `next run`。这些行中的下次运行时间需要 Claude Code v2.1.299 或更高版本。

#### 重复任务会延后多久运行

除非每五分钟运行一次，否则重复任务的每次运行都会延后开始，延后时长最多为距下一次运行间隔的一半，且最多不超过 30 分钟。您的任务属于以下三种情况之一：

* **运行间隔均匀**：间隔相等的任务（例如每小时、每天或每 10 分钟运行一次）每次都会延后相同的分钟数运行，包括在会话空闲时。
* **运行间隔不均匀**：间隔不等的任务（例如安排在每小时 `:00` 和 `:15` 运行的任务）每次运行延后的分钟数可能不同。
* **每五分钟**：写作 `*/5 * * * *` 的任务不会被延迟。恰好在上一次运行五分钟后开始的运行，会在五分钟的[提示缓存](https://code.claude.com/docs/zh-CN/prompt-caching.md#which-ttl-each-request-gets)过期后才开始，因此每次运行都会提前 15 秒开始。

在此表的第一列中找到您的计划，即可查看其每次运行的开始时间。

| 计划 | 每次运行的开始时间 |
| :- | :- |
| 每 5 分钟，写作 `*/5 * * * *`，即 `/loop 5m` 所创建的计划 | 上一次运行后 4 分 45 秒 |
| 每 10 分钟 | 延后 0 到 5 分钟 |
| 每 30 分钟 | 延后 0 到 15 分钟 |
| 每小时，或频率更低（例如每天） | 延后 0 到 30 分钟 |

例如，`7,37 * * * *` 将任务安排在 `:07` 和 `:37` 运行，两者相隔 30 分钟，因此其延迟介于 0 到 15 分钟之间。如果该任务的延迟为 14 分钟，它会在每小时的 `:21` 和 `:51` 运行。将计划更改为其他分钟会改变运行时间，但仍会在其基础上添加延迟。

#### 一次性任务何时提前运行

安排在 `:00` 或 `:30` 的一次性任务最多会提前 90 秒运行。Claude Code 不会调整安排在其他任何分钟的一次性任务，因此当时间很重要时，请避开整点和半点进行安排：使用 `3 9 * * *` 而不是 `0 9 * * *`。

### 七天过期

重复任务在创建后 7 天自动过期。任务最后触发一次，然后删除自己。这限制了被遗忘的循环可以运行多长时间。如果您需要重复任务持续更长时间，请在过期前取消并重新创建它，或使用[云端定时任务](https://code.claude.com/docs/zh-CN/routines.md)或 [Desktop 定时任务](https://code.claude.com/docs/zh-CN/desktop-scheduled-tasks.md)进行持久调度。

## Cron 表达式参考

`CronCreate` 接受标准 5 字段 cron 表达式：`minute hour day-of-month month day-of-week`。所有字段都支持通配符 (`*`)、单个值 (`5`)、步长 (`*/15`)、范围 (`1-5`) 和逗号分隔的列表 (`1,15,30`)。

| 示例 | 含义 |
| :- | :- |
| `*/5 * * * *` | 每 5 分钟 |
| `0 * * * *` | 每小时整点 |
| `7 * * * *` | 每小时的第 7 分钟 |
| `0 9 * * *` | 每天本地时间 9am |
| `0 9 * * 1-5` | 工作日本地时间 9am |
| `30 14 15 3 *` | 3 月 15 日本地时间下午 2:30 |

星期几使用 `0` 或 `7` 表示星期日，`6` 表示星期六。不支持扩展语法如 `L`、`W`、`?` 和名称别名如 `MON` 或 `JAN`。

当月份日期和星期几都受到限制时，如果任一字段匹配，日期就匹配。这遵循标准的 vixie-cron 语义。

## 禁用会话范围的定时任务

在您的环境中设置 `CLAUDE_CODE_DISABLE_CRON=1` 以完全禁用调度程序。cron 工具和 `/loop` 变得不可用，任何已计划的任务都停止触发。有关禁用标志的完整列表，请参阅[环境变量](https://code.claude.com/docs/zh-CN/env-vars.md)。

## 限制

会话范围的调度有固有的限制：

* 任务仅在 Claude Code 运行且空闲时触发。关闭终端或让会话退出会停止它们触发。[将会话放在后台](https://code.claude.com/docs/zh-CN/agent-view.md#from-inside-a-session)会将 `/loop` 任务转移到后台会话，该会话继续运行而无需终端。
* 没有错过触发的追赶。如果任务的计划时间在 Claude 忙于长时间运行的请求时经过，它会在 Claude 变为空闲时触发一次，而不是每个错过的间隔触发一次。
* 在[后台会话](https://code.claude.com/docs/zh-CN/agent-view.md#from-inside-a-session)中，如果[自定步调的循环](#let-claude-choose-the-interval)的下一次唤醒到期时会话进程未在运行（例如发生崩溃之后），该循环会停止。当进程再次启动时，会话记录会显示一条[说明该唤醒不会触发的通知](https://code.claude.com/docs/zh-CN/errors.md#restarted-after-its-next-loop-wakeup-was-due)。要继续该循环，请[回复该会话](https://code.claude.com/docs/zh-CN/agent-view.md#peek-and-reply)并要求 Claude 保持其运行。
* 当您使用 `claude --resume` 或 `claude --continue` 恢复会话时，Claude Code 会恢复使用 `CronCreate` 调度的任务，除了已[过期](#seven-day-expiry)的重复任务和计划时间已经过去的一次性任务。[自定步调的 `/loop`](#let-claude-choose-the-interval)不会被恢复，因此请再次运行 `/loop` 以重新启动它。
* 恢复的重复任务不会为会话未运行期间错过的运行而触发。其下一次运行是您恢复后第一个到期的运行，并且[抖动](#jitter)适用于该次运行。
* 当您使用 `claude --resume` 或 `claude --continue` 恢复时，Claude Code 不会重新启动会话结束时正在运行的后台 Bash 命令和监视器。如果您仍然需要其中某一个，请让 Claude 重新启动它。
* 当[功能标志获取关闭](https://code.claude.com/docs/zh-CN/env-vars.md#features-that-need-feature-flag-fetching)时，Claude Code 会将您要求在会话间保留的任务存储在项目的 `.claude/scheduled_tasks.json` 文件中。当 `.claude` 目录或该文件是符号链接时，Claude Code 会返回错误而不是调度任务。保存的任务仅在您创建它的项目文件夹中运行。如果您将文件复制到另一个文件夹（例如新的 worktree），那里的会话会列出复制的任务但不会运行它们，因此请在该文件夹中再次创建任务。

对于需要无人值守运行的 cron 驱动自动化：

* [云端定时任务](https://code.claude.com/docs/zh-CN/routines.md)：在云上按计划运行、通过 API 调用或在 GitHub 事件上运行
* [GitHub Actions](https://code.claude.com/docs/zh-CN/github-actions.md)：在 CI 中使用 `schedule` 触发器
* [Desktop 计划任务](https://code.claude.com/docs/zh-CN/desktop-scheduled-tasks.md)：在您的机器上本地运行
