> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# 输出样式

> 通过内置输出样式（如简洁或解释性）或自定义样式来改变 Claude Code 的角色、语气和响应格式。

输出样式是一组指令，为会话中的每个响应设置 Claude 的角色、语气和响应格式。Claude Code 包括四种内置样式（除了默认样式），你也可以编写自己的样式。

使用输出样式来改变 Claude 在整个会话中的响应和工作方式，这样你就不需要在每个提示中重复请求。例如，内置样式可以使响应更短、为每个更改添加解释，或让 Claude 在不提出常规问题的情况下开始工作。自定义样式也可以将 Claude 转变为软件工程师以外的角色，例如写作助手或数据分析师。

* 要使用内置样式，请从[内置输出样式](#built-in-output-styles)中选择一个，然后[切换到它](#change-your-output-style)。
* 要编写自己的指令，请[创建自定义输出样式](#create-a-custom-output-style)。

> [!NOTE]
>
> 输出样式为 Claude 提供要遵循的指令。它不保证某些事情总是发生或永远不会发生。某些需求适合不同的功能：
>
> * 对于 Claude 应该了解的关于你的项目的内容，请使用 [CLAUDE.md](https://code.claude.com/docs/zh-CN/memory.md)。
> * 对于必须每次都发生的事情，例如每次编辑后的格式化或阻止命令，请使用 [hook](https://code.claude.com/docs/zh-CN/hooks-guide.md)。
> * 对于技能、子代理和其他选项，请参阅[在输出样式和其他功能之间选择](#choose-between-an-output-style-and-other-features)。

## 内置输出样式

Claude Code 从[**默认**](#default)样式开始，这是其完成软件工程任务的标准指令。其他四种内置样式保留这些指令并添加各自的特定内容。

此表显示每种样式改变了什么以及何时适合使用：

| 样式 | 改变内容 | 何时使用 |
| :- | :- | :- |
| [Proactive](#proactive) | Claude 立即开始工作，对常规决策做出合理的假设，而不是询问 | 你希望 Claude 通过常规决策继续工作，如果假设有误，你可以纠正方向 |
| [Concise](#concise) | 响应以结果开头，省略前言、叙述和总结 | 默认响应比你想要的要长 |
| [Explanatory](#explanatory) | Claude 添加简短的 `Insight` 块，解释其编写代码背后的选择 | 你正在了解一个代码库或想要随着更改一起获得推理 |
| [Learning](#learning) | Claude 解释其选择，并留下小段代码供你自己编写 | 你想要在完成任务的同时获得实践编码经验 |

### 默认

默认意味着未选择任何输出样式。Claude Code 不添加样式指令，Claude 从 Claude Code 的标准系统提示工作，该提示是为软件工程任务编写的。

`default` 出现在 `/output-style` 列表中与其他样式一起，所以你[以相同的方式选择它](#change-your-output-style)。

### Proactive

在 Proactive 样式中，Claude 在你发送任务后立即开始实现。它对常规决策做出合理的假设，而不是停下来询问，除非你要求制定计划，否则不会切换到计划模式。你可以在任何时刻重定向它。

该样式的指令还告诉 Claude 在删除数据或更改共享或生产系统的操作之前在对话中与你核实。该核实是 Claude 遵循的指令，与权限提示分开。

切换到 Proactive 样式不会改变你的[权限模式](https://code.claude.com/docs/zh-CN/permission-modes.md)。你的权限模式仍然决定哪些工具调用在不询问你的情况下运行，所以权限提示的显示方式与你切换之前相同。

### Concise

在 Concise 样式中，响应的第一句陈述发生了什么或答案是什么。Claude 省略了引言、逐步叙述和结尾总结，并用一到三句话回答简单问题。它以与默认样式相同的彻底程度完成工程工作。需要 Claude Code v2.1.237 或更高版本。

Claude 在以下情况下仍然会完整编写：

* **你要求的任何内容**：当你要求解释或更多细节时，Claude 会完整回答。
* **你安全行动所需的任何内容**：错误报告、失败的测试输出、安全警告和破坏性操作的确认保留其完整内容。

### Explanatory

在 Explanatory 样式中，Claude 以与默认样式相同的方式完成任务，并添加关于其做出选择原因的简短解释。每个解释出现在对话中，在其相关代码之前或之后，在标记为 `Insight` 的块中。这些解释不会作为注释写入你的文件中。

`Insight` 块包含关于你的代码库或 Claude 编写的代码的两到三个要点，例如添加 API 端点后的这个：

```text theme={null}
★ Insight ─────────────────────────────────────
- Every route in this repo goes through the withAuth wrapper, so the new endpoint gets session checks without its own middleware.
- Rate limits are set per route in limits.ts, which is why this change adds an entry there rather than a global default.
─────────────────────────────────────────────────
```

### Learning

在 Learning 样式中，Claude 添加与 [Explanatory 样式](#explanatory)相同的 `Insight` 块，并且还要求你编写一些代码。Claude 自己处理常规实现。当它到达具有真实设计决策的部分时，例如错误处理、数据结构或具有多个有效方法的业务逻辑，它会为你留下几行代码。

Claude 用文件中的 `TODO(human)` 注释标记该位置，然后发送一个请求，说明已经构建的内容、要编写的内容以及要权衡的内容：

```text theme={null}
● Learn by Doing

Context: The upload form is in place and calls validateFile() before accepting a file. Size and type checks work for images, but the switch statement has no handling for documents yet.

Your Task: In upload.js, implement the case "document" branch inside validateFile(). Look for TODO(human).

Guidance: Decide on a size limit for documents and whether the file extension has to match the MIME type. Return {valid: boolean, error?: string}.
```

Claude 然后停止并等待。在 `TODO(human)` 注释处编写你的代码，并告诉 Claude 你已完成。Claude 用一个关于你的代码的 `Insight` 进行响应并继续该任务。

## 更改你的输出样式

通过命令、菜单或设置文件选择样式。命令和两个菜单都会将你的选择保存到[本地项目级别](https://code.claude.com/docs/zh-CN/settings.md)的 `.claude/settings.local.json`。

* **`/output-style` 命令**：运行 `/output-style <style>` 来切换，例如 `/output-style concise`。不带参数时，该命令列出你可以选择的样式并标记当前样式。

  该命令也适用于[非交互模式](https://code.claude.com/docs/zh-CN/headless.md)和 Agent SDK 会话，以及来自移动应用或网页的[远程控制](https://code.claude.com/docs/zh-CN/remote-control.md#limitations)，其中你只能列出和选择[内置样式](#built-in-output-styles)。需要 Claude Code v2.1.269 或更高版本。
* **Terminal 菜单**：运行 `/config` 并选择**输出样式**从菜单中选择一种样式。
* **VS Code extension**：使用 `/` 打开[命令菜单](https://code.claude.com/docs/zh-CN/vs-code.md#use-the-prompt-box)并选择**输出样式**来选择一种样式，包括你的自定义样式。需要 Claude Code v2.1.257 或更高版本。
* **桌面应用**：在设置文件中设置 `outputStyle` 字段，例如 `.claude/settings.local.json`，这是终端菜单写入的文件。当您在桌面应用中运行 `/config` 时，Claude Code 会[打开**设置 > Code** 或**设置 > Claude Code**](https://code.claude.com/docs/zh-CN/desktop.md#what’s-not-available-in-desktop)，而不是菜单。

要在不使用菜单的情况下设置样式，直接编辑设置文件中的 `outputStyle` 字段：

```json theme={null}
{
  "outputStyle": "Explanatory"
}
```

该值区分大小写，因此请将内置名称写为 `Proactive`、`Concise`、`Explanatory` 和 `Learning`。与样式名称不完全匹配的值（例如 `explanatory`）会给你默认样式。`/output-style` 命令忽略大小写。

要在项目间将样式设置为默认值，请在 `~/.claude/settings.json` 中设置 `outputStyle`。项目自己的设置文件[优先于](https://code.claude.com/docs/zh-CN/settings.md#settings-precedence)该值。

当你在会话中途切换样式时，Claude 从你的下一条消息开始使用新样式。关于该第一条消息在 prompt caching 中的成本，请参阅[更改输出样式](https://code.claude.com/docs/zh-CN/prompt-caching.md#changing-output-style)。在 v2.1.251 之前，新样式仅在你运行 `/clear` 或开始新会话后才会应用。

## 创建自定义输出样式

自定义输出样式是一个 Markdown 文件：frontmatter 用于元数据，然后是 Claude 的说明。

在 VS Code 扩展中，你也可以从[**输出样式**菜单](https://code.claude.com/docs/zh-CN/vs-code.md#use-the-prompt-box)创建文件，而不是手动编写。这需要 Claude Code v2.1.261 或更高版本。

**1. 创建一个 Markdown 文件**

在三个级别之一保存它。文件名成为样式名称，除非你在 frontmatter 中设置 `name`。

* 用户：`~/.claude/output-styles`
* 项目：`.claude/output-styles`
* 托管策略：[托管设置目录](https://code.claude.com/docs/zh-CN/managed-settings.md#delivery-mechanisms)内的 `.claude/output-styles`

项目输出样式从工作目录和仓库根目录之间的每个 `.claude/output-styles/` 加载。当多个这样的嵌套目录定义了同名样式时，Claude Code 使用最接近工作目录的那个。

**2. 添加 frontmatter 和说明**

决定是否保留 Claude Code 的软件工程说明。如果你改变 Claude 的通信方式但仍希望它以相同的方式编码，请设置 `keep-coding-instructions: true`。如果 Claude 不会进行软件工程，请省略它。

此示例在保留 Claude 编码行为的同时，在每个解释前面加上一个图表：

```markdown theme={null}
---
name: Diagrams first
description: Lead every explanation with a diagram
keep-coding-instructions: true
---

When explaining code, architecture, or data flow, start with a Mermaid diagram showing the structure, then explain in prose.

## Diagram conventions

Use `flowchart TD` for control flow and `sequenceDiagram` for request paths. Keep diagrams under 15 nodes.
```

**3. 切换到你的样式**

在终端中运行 `/output-style <style>`，或运行 `/config` 并在**输出样式**下选择你的样式。Claude 从你的下一条消息开始使用新样式。在终端中，Claude Code 在启动时读取样式文件，所以如果你在运行会话期间创建或编辑一个样式文件，请重启 Claude Code 以获取更改。

[Plugins](https://code.claude.com/docs/zh-CN/plugins/manifest-reference.md) 也可以在 `output-styles/` 目录中提供输出样式。

### Frontmatter 参考

使用位于文件顶部 `---` 标记之间的 YAML [frontmatter](https://code.claude.com/docs/zh-CN/glossary.md#frontmatter) 配置输出样式。所有字段都是可选的，字段名称使用由连字符分隔的小写单词。拼写错误的字段会被忽略而不会出现错误。如果 YAML 无法解析，样式仍会以其文件名加载，且不设置任何字段；运行 `claude --debug` 以查看解析错误。

| 字段 | 必需 | 描述 |
| :- | :- | :- |
| `name` | 否 | 输出样式的名称，在 `/config` 选择器中显示。默认值：文件名 |
| `description` | 否 | 输出样式的描述，在 `/config` 选择器中显示 |
| `keep-coding-instructions` | 否 | 设置为 `true` 以在您的样式之外保留 Claude Code 的内置软件工程说明部分（只有完整的系统提示词包含该部分）。请参阅[输出样式的工作原理](#how-output-styles-work)。默认值：`false` |
| `force-for-plugin` | 否 | 仅限 Plugin 输出样式。设置为 `true` 以在启用 plugin 时自动应用此样式，无需要求用户选择它。覆盖用户的 `outputStyle` 设置。如果多个启用的 plugin 设置了此项，Claude Code 使用第一个加载的。默认值：`false` |

## 在输出样式和其他功能之间选择

输出样式适用于会话中的每个响应。这是 Claude 遵循的指令，所以没有任何东西强制执行它。当你想要的内容比每个响应更狭窄，或者必须无一例外地发生时，另一个功能更合适。

此表将你想要的内容与执行该操作的功能相匹配：

| 你想要 | 使用 | 为什么合适 |
| :- | :- | :- |
| 每个响应都采用特定的语气、长度或格式，或 Claude 采用不同的角色 | 输出样式 | 它适用于整个会话，你可以用一个命令切换样式 |
| Claude 了解你的项目的约定、命令和结构 | [CLAUDE.md](https://code.claude.com/docs/zh-CN/memory.md) | 它保存了 Claude 应该了解的关于代码库的内容，无论你选择哪种样式，它都保持加载状态 |
| 针对一种任务的指令，例如发布清单或审查程序 | 一个 [skill](https://code.claude.com/docs/zh-CN/skills.md) | Claude 仅在你调用它或任务匹配时加载它，所以它不会影响无关的响应 |
| 每次都无一例外地发生的事情，例如每次编辑后的格式化或阻止命令 | 一个 [hook](https://code.claude.com/docs/zh-CN/hooks-guide.md) | Claude Code 在生命周期事件中自己运行 hook，所以它不依赖于 Claude 遵循指令 |
| 一个具有自己的指令、模型和工具的助手，用于专注任务 | 一个 [subagent](https://code.claude.com/docs/zh-CN/sub-agents.md) | 它在具有自己的系统提示的单独上下文中运行，并将摘要返回到你的对话 |
| 你启动 Claude Code 时传递的对 Claude 指令的补充 | [`--append-system-prompt`](https://code.claude.com/docs/zh-CN/cli-reference#system-prompt-flags) | 它附加到系统提示而不删除任何内容 |

这些功能可以组合使用。例如，你可以使用 CLAUDE.md 来说明 Claude 应该了解的内容，使用输出样式来说明它如何响应，以及使用 hook 来保证任何必须保证的事情。[扩展 Claude Code](https://code.claude.com/docs/zh-CN/features-overview.md) 比较了其余的扩展功能。

## 输出样式的工作原理

输出样式改变 Claude Code 给予 Claude 的指令。

* Claude Code 在每个请求中发送活跃样式的指令。
* 在完整系统提示词中，自定义输出样式会省略 Claude Code 的内置软件工程指令部分，例如如何限定更改范围、编写注释和验证工作，除非 `keep-coding-instructions` 设置为 `true`。较短的系统提示词不包含该部分，因此该字段在那里不起作用。若要依赖该字段，请将 [`CLAUDE_CODE_SIMPLE_SYSTEM_PROMPT`](https://code.claude.com/docs/zh-CN/env-vars#variables) 设置为 `0`，这样在任何模型上都会选用完整提示词。

输出样式适用于主对话和[分支](https://code.claude.com/docs/zh-CN/sub-agents.md#fork-the-current-conversation)，分支继承父级的完整对话和系统提示词。其他[子代理运行自己的系统提示词](https://code.claude.com/docs/zh-CN/sub-agents.md#what-loads-at-startup)，因此样式不会改变它们的回复方式。

令牌使用情况取决于样式。样式的指令会增加输入令牌，尽管提示缓存在会话中的第一个请求之后会降低这个成本。

内置的 Explanatory 和 Learning 样式按设计会产生比 Default 更长的响应，这会增加输出令牌。Concise 样式则相反，通过指示 Claude 默认保持响应简短来实现。对于自定义样式，输出令牌使用情况取决于你的指令告诉 Claude 生成什么。

## 相关资源

* [Settings](https://code.claude.com/docs/zh-CN/settings.md)：`outputStyle` 字段所在的位置以及设置优先级的工作原理
* [Permission modes](https://code.claude.com/docs/zh-CN/permission-modes.md)：Proactive 样式与自动模式的比较方式
* [Plugins](https://code.claude.com/docs/zh-CN/plugins/overview.md)：打包和分发输出样式以及 skills、hooks 和 agents
* [Debug your configuration](https://code.claude.com/docs/zh-CN/debug-your-config.md)：诊断为什么输出样式没有生效
