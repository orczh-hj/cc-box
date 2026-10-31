> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# 让 Claude 朝着目标工作

> 使用 /goal 设置完成条件，Claude 会持续工作直到条件满足、模型判断其不可能实现或需要修复的错误清除目标。

`/goal` 命令设置一个完成条件，Claude 会在无需您逐步提示的情况下持续朝着这个目标工作。每个轮次后，模型会检查条件是否满足。如果模型判断条件尚未满足，Claude 会开始另一个轮次，而不是将控制权返回给您。一旦条件满足、模型判断条件不可能满足或轮次因[需要修复的错误](#errors-you-have-to-fix-clear-the-goal)失败时，目标会自动清除。

对于具有可验证的最终状态的实质性工作，使用目标：

* 将模块迁移到新 API，直到每个调用站点都能编译并通过测试
* 实现设计文档，直到所有验收标准都满足
* 将大文件拆分为专注的模块，直到每个模块都在大小预算内
* 处理标记的问题积压，直到队列为空

## 比较保持会话运行的方式

三种方法可以在提示之间保持当前会话运行。根据应该启动下一个回合的内容进行选择：

| 方法 | 下一个回合何时开始 | 停止条件 |
| :- | :- | :- |
| `/goal` | 前一个回合完成时，或在交互式会话中，[空闲检查](#background-work-defers-evaluation)或[自动重试](#other-errors-retry-or-pause-the-goal)到期时 | 模型确认条件已满足或判断其不可能，或回合因[你必须修复的错误](#errors-you-have-to-fix-clear-the-goal)而失败，或你运行[`/goal clear`](#clear-a-goal) |
| [`/loop`](https://code.claude.com/docs/zh-CN/scheduled-tasks#run-a-prompt-repeatedly-with-%2Floop) | 时间间隔过去时 | 你停止它，或 Claude 决定工作完成 |
| [Stop hook](https://code.claude.com/docs/zh-CN/hooks-guide.md#prompt-based-hooks) | 前一个回合完成时 | 你自己的脚本或提示决定 |

`/goal` 和 Stop hook 都在每个回合后触发。`/goal` 是一个会话范围的快捷方式：你输入一个条件，它仅在当前会话中活跃。Stop hook 存在于你的设置文件中，适用于其范围内的每个会话，可以运行脚本进行确定性检查或运行提示进行模型评估的检查。

[Auto mode](https://code.claude.com/docs/zh-CN/auto-mode-config.md) 本身在单个回合内批准工具调用，但不会启动新的回合。Claude 在判断工作完成时停止。`/goal` 添加了一个单独的评估器，在每个回合后检查你的条件，因此完成由一个新鲜的模型而不是执行工作的模型决定。这两者是互补的：auto mode 消除了每个工具的提示，`/goal` 消除了每个回合的提示。

> [!TIP]
>
> 上述方法用于保持当前会话运行。您还可以安排独立于任何打开的会话运行的工作，例如夜间测试或早间分类处理。有关云端和 Desktop 定时任务，请参阅[调度选项](https://code.claude.com/docs/zh-CN/scheduled-tasks.md#compare-scheduling-options)。

## 使用 `/goal`

每个会话可以有一个活跃的目标。同一命令根据参数设置、检查和清除它。

### 设置目标

运行 `/goal` 后跟你想要满足的条件。如果已经有一个活跃的目标，新目标会替换它。

```text theme={null}
/goal all tests in test/auth pass and the lint step is clean
```

设置目标会立即启动一个回合，条件本身作为指令。你不需要发送单独的提示。当目标活跃时，`◎ /goal active` 指示器显示目标已运行多长时间。

目标不会改变你的权限模式。要让目标回合无人值守地运行，请在[自动模式](https://code.claude.com/docs/zh-CN/auto-mode-config.md)中运行 `/goal`。在[手动模式](https://code.claude.com/docs/zh-CN/permission-modes.md)中，Claude 在进行工具调用前仍会询问，这些工具调用是你的设置不允许的，例如上面的测试命令。

当目标活跃时，记录显示评估器返回的每个判决，你可以按 Ctrl+O 查看其背后的原因。状态视图也显示最近的原因，所以你可以看到 Claude 接下来要朝着什么工作。

### 编写有效的条件

[评估器](#how-evaluation-works)根据 Claude 在对话中呈现的内容来判断你的条件。它不会独立运行命令或读取文件，所以将条件写成 Claude 自己的输出可以演示的内容。"所有 `test/auth` 中的测试都通过"之所以有效，是因为 Claude 运行测试，结果出现在记录中供评估器读取。

在许多回合中保持的条件通常具有：

* **一个可测量的最终状态**：测试结果、构建退出代码、文件计数、空队列
* **一个陈述的检查**：Claude 应该如何证明它，例如"`npm test` 退出 0"或"`git status` 是干净的"
* **重要的约束**：在此过程中必须不改变的任何内容，例如"没有其他测试文件被修改"

条件最多可以是 4,000 个字符。

要限制目标运行的时间，在条件中包含一个回合或时间子句，例如 `or stop after 20 turns`。Claude 每个回合都会针对该子句报告进度，评估器从对话中判断它。

### 检查状态

运行不带参数的 `/goal` 可以查看当前状态。

```text theme={null}
/goal
```

如果目标活跃，状态显示：

* 条件
* 已运行多长时间
* 已评估多少个回合
* 当前令牌支出
* 评估器最近的原因

回合计数和最近的原因在第一次评估运行后出现。

如果没有活跃的目标，但在会话早期实现了一个目标，状态显示已实现的条件及其持续时间、回合计数和令牌支出。

### 清除目标

运行 `/goal clear` 可以在条件满足之前移除活跃的目标。

```text theme={null}
/goal clear
```

Claude 打印 `Goal cleared:` 后跟条件以确认，或如果没有活跃的目标则打印 `No goal set`。

`stop`、`off`、`reset`、`none` 和 `cancel` 被接受为 `clear` 的别名。运行 `/clear` 启动新对话也会移除任何活跃的目标。

### 使用活跃目标恢复

当你恢复会话时，Claude Code 会恢复在会话结束时仍然活跃的目标。Claude Code 在每个恢复路由上恢复它：`--continue`、`--resume` 带有会话 ID、名称或[记录文件路径](https://code.claude.com/docs/zh-CN/sessions.md#resume-a-session)，以及[会话选择器](https://code.claude.com/docs/zh-CN/sessions.md#use-the-session-picker)。在 v2.1.239 之前，Claude Code 在除了 `claude --resume` 选择器之外的每个路由上恢复目标。

Claude Code 保留条件但重置回合计数、计时器和令牌支出基线。它不会恢复已经实现或清除的目标。

### 非交互式运行

`/goal` 在[非交互式模式](https://code.claude.com/docs/zh-CN/headless.md)、[桌面应用](https://code.claude.com/docs/zh-CN/desktop.md)中工作，并通过[远程控制](https://code.claude.com/docs/zh-CN/remote-control.md)工作。使用 `-p` 设置目标会在单个调用中运行循环至完成：

```bash theme={null}
claude -p "/goal CHANGELOG.md has an entry for every PR merged this week"
```

使用默认文本输出时，Claude 的最终回复会在循环结束时打印，因此运行许多轮次的目标可能看起来卡住了。添加 `--output-format stream-json --verbose` 以在循环运行时输出每条消息。

使用 Ctrl+C 中断进程以在条件满足之前停止非交互式目标。

## 评估如何工作

`/goal` 是会话范围的[基于提示词的 Stop hook](https://code.claude.com/docs/zh-CN/hooks.md#prompt-based-hooks)的包装器。每次 Claude 完成一个轮次时，Claude Code 会将条件和到目前为止的对话发送到您配置的[小型快速模型](https://code.claude.com/docs/zh-CN/model-config.md)。该模型返回三个判决之一，每个都带有简短的原因：

* **尚未满足**：Claude 继续工作，并将原因作为下一个回合的指导。
* **已满足**：Claude Code 清除目标并在记录中记录一个已实现的条目。
* **不可能**：评估器判断该条件永远无法满足。Claude Code 清除目标并在记录中记录一个失败的条目以及原因。你不需要自己清除它。

如果 Claude 持续回答评估器而没有取得进展（连续多个回合没有工具使用），Claude Code 会停止循环，打印警告，并将控制权返回给你，目标仍然设置。评估在你的下一个提示后恢复。[hooks 指南](https://code.claude.com/docs/zh-CN/hooks-guide.md#stop-hook-hits-the-block-cap)解释了底层机制。

### 当一个回合失败时

当一个回合失败时，如果错误是你必须修复的错误，Claude Code 会清除目标。在任何其他错误之后，目标保持设置。

#### 你必须修复的错误会清除目标

如果一个回合因为一个在你修复之前不会清除的错误而失败，Claude Code 会清除目标并打印一个警告，说明原因。警告以 `Goal cleared after an unrecoverable error` 开头，以 `Run /goal again to continue` 结尾。修复原因，然后使用 `/goal <condition>` [再次设置目标](#set-a-goal)。四种失败会清除目标：

* 身份验证失败，当 Claude Code 管理自己的凭证时。当主机为你管理凭证时，例如桌面应用或[云会话](https://code.claude.com/docs/zh-CN/claude-code-on-the-web.md)，Claude Code 会保持目标活跃，因为主机会自动恢复访问权限。
* 信用余额耗尽
* 一个[自动压缩](https://code.claude.com/docs/zh-CN/model-config.md#set-the-auto-compact-window)无法清除的上下文溢出
* 一个不可用的模型

#### 其他错误重试或暂停目标

在任何其他失败之后，目标保持设置。在 Claude Code v2.1.269 或更高版本的交互式会话中，Claude Code 也会打印一行说明原因，并自动重试或等待你：

* **重试**：在倾向于自行清除的失败之后，例如服务器过载或连接断开，一个以 `Goal still active` 开头的通知显示下一次尝试之前的等待时间。在三次自动重试之后，目标会暂停。
* **暂停**：在重试只会重复的失败之后，例如 API 速率限制、claude.ai [使用限制](https://code.claude.com/docs/zh-CN/errors.md#youve-hit-your-session-limit)或结束回合的 hook，一个以 `Goal paused` 开头的通知说明原因。如果会话[在使用限制重置时等待自动继续](https://code.claude.com/docs/zh-CN/interactive-mode.md#wait-for-a-usage-limit-to-reset)，Claude 会在那时恢复朝着目标的工作。

随时发送消息以立即开始下一个回合。要关闭自动重试，请将 [`CLAUDE_CODE_GOAL_CHECKIN_MINUTES`](https://code.claude.com/docs/zh-CN/env-vars) 设置为 `0`，这也会关闭[检查](#background-work-defers-evaluation)。

### 后台工作延迟评估

如果一个子代理或后台 shell 命令在回合结束时仍在运行，Claude Code 会跳过该回合的评估。它在下一个没有后台工作运行的回合结束时进行评估。当后台工作完成时，Claude Code 会将结果作为新回合传递给 Claude，所以你不必提示。

一旦后台工作让目标等待了 30 分钟，就应该进行检查。在检查中，Claude Code 列出正在运行的任务，并要求 Claude 读取其输出，如果它们在进行中则继续等待，并修复或停止任何卡住的任务。在第一次检查之后，Claude Code 在每次后续检查之前等待两倍的时间，最多是第一个间隔的四倍：使用默认值，第一次检查后 1 小时，然后每 2 小时。Claude Code 以两种方式之一传递应该进行的检查，包括第一个：

* **当回合结束时**：Claude Code 在下一个工作仍在运行的回合结束时传递检查。在非交互式会话中，例如使用 `-p` 启动的会话，这是 Claude Code 传递检查的唯一方式。
* **当会话空闲时**：在交互式会话中，Claude Code 也会自己启动一个回合来传递检查，而不是等待你的下一个提示。如果后台工作已停止而没有报告结果，Claude Code 会要求 Claude 继续朝着目标工作。Claude Code 在你的提示之间每个目标最多启动三个空闲检查。在第三个空闲检查中，Claude Code 会说空闲检查已暂停，直到你发送另一个提示。在 v2.1.246 之前，空闲检查是无上限的。空闲检查需要 Claude Code v2.1.236 或更高版本。

在 v2.1.239 之前，只有空闲检查以这种方式退避；在回合结束时传递的检查在第一个间隔重复。

要更改第一个间隔，请设置 [`CLAUDE_CODE_GOAL_CHECKIN_MINUTES`](https://code.claude.com/docs/zh-CN/env-vars)。Claude Code 使用你的值代替 30 分钟间隔，并相应地缩放后续间隔。将其设置为 `0` 以关闭检查和[自动重试](#other-errors-retry-or-pause-the-goal)。

检查需要 Claude Code v2.1.234 或更高版本。

### 评估模型和成本

要在不同的模型上进行评估，请设置 [`ANTHROPIC_DEFAULT_HAIKU_MODEL`](https://code.claude.com/docs/zh-CN/model-config#environment-variables)。

> [!WARNING]
>
> Claude Code 在使用小型快速模型的任何地方都会读取 `ANTHROPIC_DEFAULT_HAIKU_MODEL`，不仅仅是用于 `/goal` 评估。当您设置它时，Claude Code 也会将 [`haiku` 别名](https://code.claude.com/docs/zh-CN/model-config#model-aliases)解析为该模型，并在其上运行会话标题生成和其他一些后台任务。

评估器在你的会话配置的任何提供商上运行。它不调用工具，所以它只能判断 Claude 已经在对话中呈现的内容。

> [!NOTE]
>
> 评估令牌在为你的提供商配置的小型快速模型上计费，与主回合支出相比通常可以忽略不计。

## 要求

Claude Code 在与 [设置文件中的 hooks 相同的工作区信任规则](https://code.claude.com/docs/zh-CN/permissions.md#what-runs-before-you-trust-a-folder)下提供 `/goal`，因为评估器是 hooks 系统的一部分。当 [`disableAllHooks`](https://code.claude.com/docs/zh-CN/hooks#disable-or-remove-hooks) 为 `true`（在应用设置优先级后）或当在托管设置中设置了 [`allowManagedHooksOnly`](https://code.claude.com/docs/zh-CN/settings-reference#allowmanagedhooksonly) 时，`/goal` 也不可用。在每种情况下，该命令会告诉你原因，而不是默默地什么都不做。

## 另请参阅

* [使用 `/loop` 重复运行提示](https://code.claude.com/docs/zh-CN/scheduled-tasks#run-a-prompt-repeatedly-with-%2Floop)：按时间间隔重新运行，而不是直到条件满足
* [基于提示的 hooks](https://code.claude.com/docs/zh-CN/hooks-guide.md#prompt-based-hooks)：当你需要自定义评估逻辑时编写你自己的 Stop hook
* [自动模式](https://code.claude.com/docs/zh-CN/auto-mode-config.md)：自动批准工具调用，以便每个目标回合无人值守运行
* [调度比较](https://code.claude.com/docs/zh-CN/scheduled-tasks.md#compare-scheduling-options)：独立于任何打开的会话按计划运行工作
