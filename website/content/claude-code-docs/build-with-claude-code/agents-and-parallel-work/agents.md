> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# 并行运行代理

> 比较 Claude Code 同时处理多个任务的方式：子代理、代理视图、代理团队、动态工作流和项目。

Claude Code 有五种方式可以同时处理多个任务：[子代理](https://code.claude.com/docs/zh-CN/sub-agents.md)、[代理视图](https://code.claude.com/docs/zh-CN/agent-view.md)、[代理团队](https://code.claude.com/docs/zh-CN/agent-teams.md)、[动态工作流](https://code.claude.com/docs/zh-CN/workflows.md) 和 [项目](https://code.claude.com/docs/zh-CN/claude-projects.md)。它们在您保持参与的程度上有所不同，从自己指导每个对话到让 Claude 协调一组工作人员，以及工作是在您的机器上运行还是在云中运行。

| 方法 | 它提供什么 | 何时使用 |
| :- | :- | :- |
| [子代理](https://code.claude.com/docs/zh-CN/sub-agents.md) | 在一个会话内的委派工作人员，在自己的上下文中执行辅助任务并返回摘要 | 辅助任务会用搜索结果、日志或文件内容淹没您的主对话，而您不会再次引用这些内容 |
| [代理视图](https://code.claude.com/docs/zh-CN/agent-view.md) | 一个屏幕来分派和监控在后台运行的会话，使用 `claude agents` 打开。研究预览 | 您有多个独立任务，想要交付它们，一目了然地检查状态，并仅在需要时介入 |
| [代理团队](https://code.claude.com/docs/zh-CN/agent-teams.md) | 多个协调的会话，具有共享任务列表和代理间消息传递，由主导者管理。实验性功能，默认禁用 | 您希望 Claude 将项目分成多个部分、分配它们，并保持工作人员同步 |
| [项目](https://code.claude.com/docs/zh-CN/claude-projects.md) | 在 claude.ai/code 或桌面应用中进行的一个持续对话。Claude 启动称为线程的并行会话，在云中运行，或在您要求时通过 Remote Control 在您的计算机上运行。它为每个会话提供项目的说明，并向您显示哪些需要您处理。Pro、Max 和 Ultra 上的公开测试版 | 工作跨越多个任务，持续数天或数周，应在您的机器关闭时继续运行，并且您宁愿描述一次而不是分派和跟踪每个会话 |
| [动态工作流](https://code.claude.com/docs/zh-CN/workflows.md) | 一个脚本，运行许多子代理并交叉检查其结果，用于一个太大而无法一次协调的工作或需要多次处理的工作 | 一个任务对于少数几个子代理来说太大了，或者您想要对结果进行相互验证：代码库范围的审计、500 个文件的迁移、交叉检查的研究或从多个角度起草的计划 |

在每种方法中，工作人员都是 Claude 会话。要涉及不同的工具，请将其作为 [MCP server](https://code.claude.com/docs/zh-CN/mcp.md) 公开给 Claude。

三个更多的工具支持这项工作，但它们本身不是运行代理的方式：

* [Worktree](https://code.claude.com/docs/zh-CN/worktrees.md) 为每个会话提供单独的 git 检出，因此并行会话各自编辑自己的文件副本。将它们用于您自己运行的会话。从 Agent 视图分派的会话会 [在编辑文件之前移到自己的 worktree 中](https://code.claude.com/docs/zh-CN/agent-view.md#how-file-edits-are-isolated)，您生成的子代理也可以各自获得一个。
* [跨会话消息传递](https://code.claude.com/docs/zh-CN/cross-session-messaging.md) 让 Claude 列出并消息传递您在这台机器上、另一台机器上或 [云中](https://code.claude.com/docs/zh-CN/claude-code-on-the-web.md) 的其他 Claude Code 会话，因此您自己运行的会话可以在彼此之间传递发现和状态。
* [`/batch`](https://code.claude.com/docs/zh-CN/commands) 是一个 [skill](https://code.claude.com/docs/zh-CN/skills.md)，它让 Claude 将一个大型更改分成 5 到 30 个 worktree 隔离的子代理。它是子代理和 worktrees 的打包使用，不是一个单独的协调风格。

还有一些其他功能在没有您驱动每一步的情况下运行 Claude，但它们解决的问题与在代理之间分割工作不同：

* [后台 bash 命令](https://code.claude.com/docs/zh-CN/interactive-mode.md#background-bash-commands) 运行一个 shell 命令而不阻止对话。它不会生成代理。
* [分叉子代理](https://code.claude.com/docs/zh-CN/sub-agents.md#fork-the-current-conversation) 是一个继承您完整对话上下文而不是从头开始的子代理。它是生成子代理的一种方式，不是一个单独的界面。使用 `/subtask` 启动一个。Claude 也会在 [fork mode](https://code.claude.com/docs/zh-CN/sub-agents.md#turn-fork-mode-on-or-off) 打开时自己生成一个。要将整个会话复制到一个新的 [后台会话](https://code.claude.com/docs/zh-CN/agent-view.md#from-inside-a-session) 中，该会话与其并行运行，请使用 `/fork`。当 [agent view 关闭](https://code.claude.com/docs/zh-CN/agent-view.md#turn-off-agent-view) 时，分叉子代理命令是 `/fork` 而不是 `/subtask` 不可用。
* [云端定时任务](https://code.claude.com/docs/zh-CN/routines.md) 在云中按计划运行会话，而不是在您的机器上并行运行。

> [!NOTE]
>
> 同时运行多个会话或子代理会成倍增加 token 使用量。有关使用情况和速率限制详情，请参阅 [Costs](https://code.claude.com/docs/zh-CN/costs.md)。

## 选择一种方法

正确的方法取决于谁协调工作、工作人员是否需要通信以及他们是否编辑相同的文件：

* **谁协调工作？**
  * Claude 在一个对话中委派和收集结果：[子代理](https://code.claude.com/docs/zh-CN/sub-agents.md)
  * 您交付独立任务并稍后检查：[代理视图](https://code.claude.com/docs/zh-CN/agent-view.md)
  * Claude 计划、分配和监督一组工作人员：[代理团队](https://code.claude.com/docs/zh-CN/agent-teams.md)，实验性功能，默认禁用
  * 脚本而不是 Claude 的逐轮判断来保持协调：[动态工作流](https://code.claude.com/docs/zh-CN/workflows.md)。请参阅[工作流与子代理和 skills 的比较](https://code.claude.com/docs/zh-CN/workflows.md#when-to-use-a-workflow)
* **工作人员需要相互交谈吗？** Claude 可以通过[跨会话消息传递](https://code.claude.com/docs/zh-CN/cross-session-messaging.md)在您自己运行的会话之间传递发现，包括您从代理视图分派的会话。子代理将结果报告回生成它们的对话，代理视图会话仅向您报告结果。代理团队中的队友直接相互发送消息，当他们[拥有 Task tools](https://code.claude.com/docs/zh-CN/tools-reference.md#task-tool-availability) 时，共享任务列表。
* **任务是否接触相同的文件？** 使用 [worktrees](https://code.claude.com/docs/zh-CN/worktrees.md) 隔离工作。子代理和您自己运行的会话可以各自使用单独的 worktree。代理团队不会在 worktrees 中隔离队友，因此[分区工作](https://code.claude.com/docs/zh-CN/agent-teams.md#avoid-file-conflicts)，以便每个队友拥有不同的文件集。

## 检查运行中的工作

检查运行中工作的命令取决于您使用的方法：

* 对于后台会话，`claude agents` 打开 [代理视图](https://code.claude.com/docs/zh-CN/agent-view.md)：一个屏幕显示每个会话、其状态以及哪些需要您的输入。
* 对于当前会话中的子代理，命名的后台子代理出现在 @-mention 类型提前中，显示其状态。`/agents` 命令会打印一条通知，指向子代理文件位置。要 [创建和编辑自定义子代理](https://code.claude.com/docs/zh-CN/sub-agents.md#configure-subagents)，请询问 Claude 或直接编辑文件。尽管名称相似，`/agents` 与 `claude agents` 是分开的。
* 对于当前会话后台运行的任何内容，`/tasks` 列出每个项目，让您检查、附加到或停止它。该列表还包括已完成的子代理。
* 对于动态工作流，`/workflows` 列出运行和已完成的运行、每个运行所处的阶段以及有多少代理已完成。

有关所有会话的桌面视图，请参阅 [桌面应用中的并行会话](https://code.claude.com/docs/zh-CN/desktop.md#work-in-parallel-with-sessions)。

## 了解更多

下面的每个指南涵盖一种方法的设置和配置：

* [创建自定义子代理](https://code.claude.com/docs/zh-CN/sub-agents.md)：定义可重用的专家并控制他们可以使用的工具。
* [使用代理视图管理代理](https://code.claude.com/docs/zh-CN/agent-view.md)：分派会话、观察其状态，并在需要时附加。
* [编排代理团队](https://code.claude.com/docs/zh-CN/agent-teams.md)：设置主导者和队友、分配任务并审查他们的工作。
* [编排动态工作流](https://code.claude.com/docs/zh-CN/workflows.md)：运行捆绑的工作流或让 Claude 编写一个运行许多子代理并相互验证其发现的工作流。
* [使用 worktrees 运行并行会话](https://code.claude.com/docs/zh-CN/worktrees.md)：在隔离的检出中启动 Claude、控制复制的内容并在之后清理。
