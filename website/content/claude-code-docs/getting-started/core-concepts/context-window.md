> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# 探索上下文窗口

> Claude Code 上下文窗口在会话期间如何填充的交互式模拟。查看自动加载的内容、每个文件读取的成本以及规则和 hooks 何时触发。

Claude Code 的上下文窗口包含 Claude 在您的会话中了解的所有内容：您的指令、它读取的文件、它自己的响应以及从不在您的终端中出现的内容。下面的时间线展示了从启动到压缩的完整会话：在您输入之前加载的内容、每个文件读取、规则和 hook 在 Claude 工作时添加的内容，以及子代理如何将大型读取保留在您的上下文之外。有关相同内容的列表形式，请参阅[书面分解](#what-the-timeline-shows)。

## 时间线显示的内容

该会话通过具有代表性的令牌计数演示了一个现实的流程：

* **在您输入任何内容之前**：CLAUDE.md、自动记忆、MCP 工具名称和 skill 描述都加载到上下文中。[AGENTS.md 文件](https://code.claude.com/docs/zh-CN/memory.md#agents-md)可以代替 CLAUDE.md 加载。您自己的设置可能会在此处添加更多内容，例如[输出样式](https://code.claude.com/docs/zh-CN/output-styles.md)或来自 [`--append-system-prompt`](https://code.claude.com/docs/zh-CN/cli-reference) 的文本。
* **当 Claude 工作时**：每个文件读取都会添加到上下文中，[路径范围的规则](https://code.claude.com/docs/zh-CN/memory.md#path-specific-rules)会自动与匹配的文件一起加载，并且[PostToolUse hook](https://code.claude.com/docs/zh-CN/hooks-guide.md)在每次编辑后触发。
* **后续提示**：[子代理](https://code.claude.com/docs/zh-CN/sub-agents.md)在其自己的单独上下文窗口中处理研究，因此大文件读取不会进入您的窗口。只有摘要和一个小的元数据预告片返回。
* **在演练结束时**：您运行 `/compact`，它用结构化摘要替换对话。大多数启动内容会自动重新加载；下表显示了每个机制会发生什么。

## 压缩后保留的内容

当长会话压缩时，Claude Code 会总结对话历史以适应上下文窗口。从 v2.1.198 开始，总结请求继承您会话的[扩展思考](https://code.claude.com/docs/zh-CN/model-config.md#extended-thinking)配置，因此当您的会话启用思考时，它会在启用思考的情况下进行推理，否则保持关闭。思考仅影响摘要的生成方式；您的会话设置之后保持不变。每种内容的处理方式取决于其加载方式：

| 机制 | 压缩后 |
| :- | :- |
| 系统提示词和输出样式 | 两者仍然适用 |
| 项目根目录 CLAUDE.md 和无范围规则 | 从磁盘重新注入 |
| 自动记忆 | 从磁盘重新注入 |
| skill 描述 | 根据您当前的 [skill](https://code.claude.com/docs/zh-CN/skills.md) 重新构建并重新注入。需要 Claude Code v2.1.304 或更高版本 |
| [Git 状态快照](https://code.claude.com/docs/zh-CN/settings-reference.md#includegitinstructions) | 如果会话开始时包含快照，Claude Code 会从您的仓库读取一个新的 |
| Claude 在[计划模式](https://code.claude.com/docs/zh-CN/permission-modes.md#analyze-before-you-edit-with-plan-mode)中编写的计划 | 从磁盘重新注入 |
| 带有 `paths:` frontmatter 的规则 | Claude Code [按需](https://code.claude.com/docs/zh-CN/memory.md#path-specific-rules)重新加载它们 |
| 子目录中的嵌套 CLAUDE.md | Claude Code [按需](https://code.claude.com/docs/zh-CN/memory.md#how-claude-md-files-load)重新加载它们 |
| Claude 读取或编辑的文件 | Claude Code 重新读取最多五个，最近修改的优先 |
| 调用的 skill 主体 | 重新注入，每个 skill 上限为 5,000 个 token，总计 25,000 个 token；最旧的首先删除 |
| [后台命令](https://code.claude.com/docs/zh-CN/interactive-mode.md#background-bash-commands)和后台[子代理](https://code.claude.com/docs/zh-CN/sub-agents.md#run-subagents-in-foreground-or-background) | 继续运行。Claude Code 提醒 Claude 哪些仍在运行，以便它不会启动重复的 |
| hook 先前添加的上下文 | 与对话的其余部分一起总结 |
| 匹配 `compact` 源的 [SessionStart hooks](https://code.claude.com/docs/zh-CN/hooks-guide.md#re-inject-context-after-compaction) | Claude Code 运行它们并将其输出添加到压缩的上下文中 |

压缩后立即，Claude Code 重新读取最多五个 Claude 在会话中读取或编辑的文件，选择最近修改的文件。超过 5,000 个 token 的文件作为路径引用返回，不包含其内容，显示为 `Referenced file` 而不是 `Read`。

路径范围的规则和嵌套的 CLAUDE.md 文件在 Claude 读取、写入或编辑其触发文件时加载到消息历史中，因此压缩会将它们与其他所有内容一起总结。如果规则必须在压缩过程中保持不变，请删除 `paths:` frontmatter 或将其移动到项目根目录 CLAUDE.md。

skill 主体在压缩后重新注入，但大型 skill 会被截断以适应每个 skill 的上限，一旦超过总预算，最旧的已调用 skill 就会被删除。截断保留文件的开头，因此请将最重要的指令放在 `SKILL.md` 的顶部附近。

## 当上下文填满时

Claude Code 会在接近上限时自动压缩，因此上下文窗口填满并不会结束您的会话。自动压缩的工作方式与时间线中的 `/compact` 步骤相同。有关它会保留哪些内容，请参阅[当上下文填满时](https://code.claude.com/docs/zh-CN/how-claude-code-works.md#when-context-fills-up)。关于 Claude Code 何时也会压缩您闲置的对话，请参阅[空闲压缩](https://code.claude.com/docs/zh-CN/settings-reference.md#idlecompaction)。

您也可以在自动压缩运行之前采取行动：

* **带重点进行压缩**：在开始一项较长的新任务之前，运行带有指令的 `/compact`，例如 `/compact focus on the auth bug fix`。摘要会保留您选择的内容，而不是自动压缩所推测的重要内容。
* **压缩部分对话**：运行 `/rewind`，选择一条消息，然后选择 **Summarize from here** 或 **Summarize up to here**。有关每个选项会保留哪些内容以及如何引导摘要，请参阅[回退与摘要](https://code.claude.com/docs/zh-CN/checkpointing.md#rewind-and-summarize)。
* **更早进行压缩**：运行带有 token 数量的 [`/autocompact`](https://code.claude.com/docs/zh-CN/commands#all-commands)，例如 `/autocompact 500k`，以设置上下文窗口填充到多满时运行自动压缩。有关可接受的值和覆盖方式，请参阅[设置自动压缩窗口](https://code.claude.com/docs/zh-CN/model-config.md#set-the-auto-compact-window)。
* **在任务之间清除**：切换到不相关的工作时运行 `/clear`。旧的对话会挤占您接下来需要的文件，并且在每条消息上都会消耗 token。
* **委派大量读取**：将研究工作交给[子代理](https://code.claude.com/docs/zh-CN/sub-agents.md)，这样文件内容会保留在它的上下文窗口中，而不是您的上下文窗口中。

如果您需要更大的窗口而不是更短的对话，Fable 模型、Sonnet 5 及更高版本、Haiku 5.5、Opus 4.6 及更高版本以及 Sonnet 4.6 支持 100 万 token 的上下文窗口。有关各计划的可用性以及如何选择 `[1m]` 模型变体，请参阅[扩展上下文](https://code.claude.com/docs/zh-CN/model-config.md#extended-context)。在更大的上限下，压缩的工作方式相同。

自动压缩的运行时机取决于您的模型和配置。有关每个模型的边界，请参阅[默认自动压缩阈值](https://code.claude.com/docs/zh-CN/model-config.md#default-auto-compact-thresholds)；如果 Claude Code 为您的模型 ID（例如 [LLM 网关](https://code.claude.com/docs/zh-CN/llm-gateway)别名）假定了错误的窗口，请参阅[为网关或自定义模型 ID 更正窗口](https://code.claude.com/docs/zh-CN/model-config.md#correct-the-window-for-a-gateway-or-custom-model-id)。

## 检查您自己的会话

该可视化使用代表性数字。要在任何时刻查看您的实际上下文使用情况，请运行 `/context` 以获取按类别的实时分解和优化建议，包括加载了哪些 CLAUDE.md 和自动内存文件。运行 `/memory` 以打开和编辑这些文件。

## 相关资源

有关时间线中显示的功能的更深入覆盖，请参阅这些页面：

* [扩展 Claude Code](https://code.claude.com/docs/zh-CN/features-overview.md)：何时使用 CLAUDE.md 与技能与规则与 hooks 与 MCP
* [存储指令和内存](https://code.claude.com/docs/zh-CN/memory.md)：CLAUDE.md 层次结构和自动内存
* [子代理](https://code.claude.com/docs/zh-CN/sub-agents.md)：将研究委托给单独的上下文窗口
* [最佳实践](https://code.claude.com/docs/zh-CN/best-practices.md#keep-cost-and-context-down)：保持会话专注并降低成本
* [提示缓存](https://code.claude.com/docs/zh-CN/prompt-caching.md)：哪些操作会使缓存的前缀失效
* [减少令牌使用](https://code.claude.com/docs/zh-CN/costs.md#reduce-token-usage)：保持上下文使用低的策略
