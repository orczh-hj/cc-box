> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# 常见工作流程

> 使用 Claude Code 探索代码库、修复错误、重构、测试和其他日常任务的分步指南。

本页收集了日常开发的简短工作流程。有关提示和上下文管理的更高级指导，请参阅[最佳实践](https://code.claude.com/docs/zh-CN/best-practices.md)。

本页涵盖：

* [提示工作流程](#prompt-recipes)，用于探索代码、修复错误、重构、测试、PR 和文档
* [恢复以前的对话](#resume-previous-conversations)，以便任务可以跨越多个会话
* [使用 worktrees 运行并行会话](#run-parallel-sessions-with-worktrees)，以便并发编辑不会冲突
* [编辑前规划](#plan-before-editing)，以在更改接触磁盘前审查更改
* [将研究委派给 subagents](#delegate-research-to-subagents)，以保持主上下文清洁
* [将 Claude 管道输入脚本](#pipe-claude-into-scripts)，用于 CI 和批处理

## 提示工作流程

这些是日常任务的提示模式，如探索陌生代码、调试、重构、编写测试和创建 PR。每个都可以在任何 Claude Code 界面中工作；根据您的项目调整措辞。

### 理解新的代码库

有关在 monorepo 或大型代码库中配置 Claude Code 的信息，请参阅 [Monorepos 和大型存储库](https://code.claude.com/docs/zh-CN/large-codebases.md)。

#### 快速获取代码库概览

假设您刚加入一个新项目，需要快速了解其结构。

**1. 导航到项目根目录**

```bash theme={null}
cd /path/to/project 
```

将 `/path/to/project` 替换为您项目的路径。

**2. 启动 Claude Code**

```bash theme={null}
claude 
```

**3. 请求高级概览**

```text wrap theme={null}
give me an overview of this codebase
```

**4. 深入了解特定组件**

```text wrap theme={null}
explain the main architecture patterns used here
```

```text wrap theme={null}
what are the key data models?
```

```text wrap theme={null}
how is authentication handled?
```

> [!TIP]
>
> 提示：
>
> * 从广泛的问题开始，然后缩小到特定领域
> * 询问项目中使用的编码约定和模式
> * 请求项目特定术语的词汇表

#### 查找相关代码

假设您需要定位与特定功能相关的代码。

**1. 要求 Claude 查找相关文件**

```text wrap theme={null}
find the files that handle user authentication
```

**2. 获取有关组件如何交互的上下文**

```text wrap theme={null}
how do these authentication files work together?
```

**3. 理解执行流程**

```text wrap theme={null}
trace the login process from front-end to database
```

> [!TIP]
>
> 提示：
>
> * 明确说明您要查找的内容
> * 使用项目中的领域语言
> * 为您的语言安装 [code intelligence plugin](https://code.claude.com/docs/zh-CN/plugins/code-intelligence.md)，以便 Claude 能够精确地进行"转到定义"和"查找引用"导航

***

### 高效修复错误

假设您遇到了错误消息，需要找到并修复其来源。

**1. 与 Claude 分享错误**

```text wrap theme={null}
I'm seeing an error when I run npm test
```

**2. 请求修复建议**

```text wrap theme={null}
suggest a few ways to fix the @ts-ignore in user.ts
```

**3. 应用修复**

```text wrap theme={null}
update user.ts to add the null check you suggested
```

> [!TIP]
>
> 提示：
>
> * 告诉 Claude 重现问题的命令并获取堆栈跟踪
> * 提及重现错误的任何步骤
> * 让 Claude 知道错误是间歇性的还是持续的

***

### 重构代码

假设您需要更新旧代码以使用现代模式和实践。

有关将整个代码库迁移到新语言的信息，请参阅博客上的 [Anthropic 如何使用 Claude Code 运行大规模代码迁移](https://claude.com/blog/ai-code-migration)。

**1. 识别用于重构的遗留代码**

```text wrap theme={null}
find deprecated API usage in our codebase
```

**2. 获取重构建议**

```text wrap theme={null}
suggest how to refactor utils.js to use modern JavaScript features
```

**3. 安全地应用更改**

```text wrap theme={null}
refactor utils.js to use ES2024 features while maintaining the same behavior
```

**4. 验证重构**

```text wrap theme={null}
run tests for the refactored code
```

> [!TIP]
>
> 提示：
>
> * 要求 Claude 解释现代方法的优势
> * 在需要时请求更改保持向后兼容性
> * 以小的、可测试的增量进行重构

***

### 使用测试

假设您需要为未覆盖的代码添加测试。

**1. 识别未测试的代码**

```text wrap theme={null}
find functions in NotificationsService.swift that are not covered by tests
```

**2. 生成测试脚手架**

```text wrap theme={null}
add tests for the notification service
```

**3. 添加有意义的测试用例**

```text wrap theme={null}
add test cases for edge conditions in the notification service
```

**4. 运行并验证测试**

```text wrap theme={null}
run the new tests and fix any failures
```

Claude 可以生成遵循您项目现有模式和约定的测试。请求测试时，请明确说明您想验证的行为。Claude 检查您现有的测试文件以匹配已在使用的样式、框架和断言模式。

要提高覆盖率，请要求 Claude 识别您可能遗漏的边界情况。Claude 可以分析您的代码路径并建议测试错误条件、边界值和容易被忽视的意外输入。

***

### 创建拉取请求

您可以通过直接要求 Claude 创建拉取请求（"create a pr for my changes"），或逐步指导 Claude：

**1. 总结您的更改**

```text wrap theme={null}
summarize the changes I've made to the authentication module
```

**2. 生成拉取请求**

```text wrap theme={null}
create a pr
```

**3. 审查和细化**

```text wrap theme={null}
enhance the PR description with more context about the security improvements
```

要稍后找到会话，请运行 `claude --from-pr 1234`，将 1234 替换为您自己的 PR 编号，这会打开会话选择器，筛选链接到该 PR 的会话，或将 PR URL 粘贴到 [`/resume` 选择器](https://code.claude.com/docs/zh-CN/sessions#use-the-session-picker) 搜索中。当 Claude 使用 `gh pr create` 或 `glab mr create` 创建拉取请求时，Claude Code 会将会话链接到 PR，以及当 Claude [处理现有 PR](https://code.claude.com/docs/zh-CN/agent-view.md#pull-request-status) 时。

> [!TIP]
>
> 在提交前审查 Claude 生成的 PR，并要求 Claude 突出显示潜在的风险或注意事项。

### 处理文档

假设您需要为代码添加或更新文档。

**1. 识别未记录的代码**

```text wrap theme={null}
find functions without proper JSDoc comments in the auth module
```

**2. 生成文档**

```text wrap theme={null}
add JSDoc comments to the undocumented functions in auth.js
```

**3. 审查和增强**

```text wrap theme={null}
improve the generated documentation with more context and examples
```

**4. 验证文档**

```text wrap theme={null}
check if the documentation follows our project standards
```

> [!TIP]
>
> 提示：
>
> * 指定您想要的文档样式（JSDoc、docstrings 等）
> * 请求文档中的示例
> * 请求公共 API、接口和复杂逻辑的文档

***

### 在笔记和非代码文件夹中工作

Claude Code 可以在任何目录中工作。在笔记库、文档文件夹或任何 markdown 文件集合中运行它，以搜索、编辑和重新组织内容，就像处理代码一样。

`.claude/` 目录和 `CLAUDE.md` 与其他工具的配置目录并排存在，不会产生冲突。Claude 在每次工具调用时都会重新读取文件，因此它会在下次读取该文件时看到您在另一个应用程序中所做的编辑。

***

### 使用图像

假设您需要在代码库中使用图像，并希望 Claude 帮助分析图像内容。

**1. 将图像添加到对话中**

您可以使用以下任何方法：

1. 将图像拖放到 Claude Code 窗口中
2. 复制图像并使用 `Ctrl+V` 将其粘贴到 CLI 中，或在 [Windows 和 WSL 上使用 `Alt+V`](https://code.claude.com/docs/zh-CN/interactive-mode#general-controls)
3. 向 Claude 提供图像路径，例如"Analyze this image: /path/to/your/image.png"

**2. 要求 Claude 分析图像**

```text wrap theme={null}
What does this image show?
```

```text wrap theme={null}
Describe the UI elements in this screenshot
```

```text wrap theme={null}
Are there any problematic elements in this diagram?
```

**3. 使用图像获取上下文**

```text wrap theme={null}
Here's a screenshot of the error. What's causing it?
```

```text wrap theme={null}
This is our current database schema. How should we modify it for the new feature?
```

**4. 从视觉内容获取代码建议**

```text wrap theme={null}
Generate CSS to match this design mockup
```

```text wrap theme={null}
What HTML structure would recreate this component?
```

> [!TIP]
>
> 提示：
>
> * 当文本描述不清楚或繁琐时使用图像
> * 包含错误、UI 设计或图表的屏幕截图以获得更好的上下文
> * 您可以在对话中使用多个图像
> * 图像分析适用于图表、屏幕截图、模型等
> * 当 Claude 引用图像时（例如，`[Image #1]`），`Cmd+Click`（Mac）或 `Ctrl+Click`（Windows/Linux）链接以在默认查看器中打开图像

***

### 引用文件和目录

使用 @ 快速包含文件或目录，无需等待 Claude 读取它们。

**1. 引用单个文件**

```text wrap theme={null}
Explain the logic in @src/utils/auth.js
```

当文件符合 [Read 工具](https://code.claude.com/docs/zh-CN/tools-reference.md#read-tool-behavior)的 token 限制（默认为 25,000 个 token）时，这会将文件内容包含在对话中。大于 256KB 的文本文件不会被包含。

**2. 引用目录**

```text wrap theme={null}
What's the structure of @src/components?
```

**3. 引用 MCP 资源**

```text wrap theme={null}
Show me the data from @github:repos/owner/repo/issues
```

这使用 @server:resource 格式从连接的 MCP 服务器获取数据。有关详细信息，请参阅 [MCP 资源](https://code.claude.com/docs/zh-CN/mcp.md#use-mcp-resources)。

> [!TIP]
>
> 提示：
>
> * 文件路径可以是相对的或绝对的
> * 键入 `@` 打开路径建议菜单，然后按 Enter 或 Tab 接受突出显示的路径，再按 Enter 发送消息
> * @ 文件引用在文件的目录和父目录中添加 `CLAUDE.md` 到上下文
> * 目录引用显示文件列表，而不是内容
> * 您可以在单个消息中引用多个文件（例如，"@file1.js and @file2.js"）

若要阻止 `@` 引用包含文件内容和目录列表，请在 shell 中，或在用户设置或[托管设置](https://code.claude.com/docs/zh-CN/managed-settings.md)的 [`env` 块](https://code.claude.com/docs/zh-CN/env-vars#set-variables-in-settings-files)中，将 [`CLAUDE_CODE_DISABLE_FILE_MENTIONS`](https://code.claude.com/docs/zh-CN/env-vars#variables) 设置为 `1`。随后 Claude 会收到您引用的路径，并附带一条说明，提示它使用其文件工具读取这些路径。该变量需要 Claude Code v2.1.299 或更高版本，并且 Claude Code 会在项目设置和本地设置中忽略它。

当 Claude Code 包含您使用 `@` 引用的文件时，您的 [`PreToolUse` hook](https://code.claude.com/docs/zh-CN/hooks#pretooluse) 不会运行，因为此过程不涉及工具调用。如果您依赖这些 hook 来控制 Claude 读取的内容，请阻止特定路径，或阻止 `@` 引用包含文件：

* **阻止特定路径**：为这些路径添加 [`Read` 拒绝规则](https://code.claude.com/docs/zh-CN/permissions#read-and-edit)。Claude Code 不会包含与该规则匹配的文件。
* **阻止 `@` 引用包含文件**：设置 `CLAUDE_CODE_DISABLE_FILE_MENTIONS=1`，这会阻止消息中的 `@` 文件和目录引用包含内容，即使是您的 hook 不检查的路径也是如此。随后，在 Claude 使用 `Read` 等工具打开被引用的路径之前，您的匹配 hook 会运行。

***

### 按计划运行 Claude

假设您想让 Claude 自动定期处理任务，如每天早上审查开放的 PR、每周审计依赖项或在夜间检查 CI 失败。

根据您希望任务运行的位置选择调度选项：

| 选项 | 运行位置 | 最适合 |
| :- | :- | :- |
| [云端定时任务](https://code.claude.com/docs/zh-CN/routines.md) | 云端，默认由 Anthropic 管理 | 即使您的计算机关闭也应该运行的任务。也可以在 API 调用或 GitHub 事件上触发，除了计划。在 [claude.ai/code/routines](https://claude.ai/code/routines) 配置。 |
| [桌面计划任务](https://code.claude.com/docs/zh-CN/desktop-scheduled-tasks.md) | 您的机器，通过桌面应用 | 需要直接访问本地文件、工具或未提交更改的任务。 |
| [GitHub Actions](https://code.claude.com/docs/zh-CN/github-actions.md) | 您的 CI 管道 | 与存储库事件（如打开的 PR）相关的任务，或应该与工作流配置一起存在的 cron 计划。 |
| [`/loop`](https://code.claude.com/docs/zh-CN/scheduled-tasks) | 当前 CLI 会话 | 会话打开时的快速轮询。`--resume` 和 `--continue` 恢复未过期的固定间隔循环。 |

> [!TIP]
>
> 为计划任务编写提示时，明确说明成功是什么样的以及如何处理结果。任务自主运行，所以它不能提出澄清问题。例如："审查标记为 `needs-review` 的开放 PR，对任何问题留下内联评论，并在 `#eng-reviews` Slack 频道中发布摘要。"

***

### 询问 Claude 关于其功能

Claude 可以回答关于其自身功能和限制的问题。它会在最新的 Claude Code 文档中查找答案，因此答案不局限于您正在运行的版本。

#### 示例问题

```text wrap theme={null}
can Claude Code create pull requests?
```

```text wrap theme={null}
how does Claude Code handle permissions?
```

```text wrap theme={null}
what skills are available?
```

```text wrap theme={null}
how do I use MCP with Claude Code?
```

```text wrap theme={null}
how do I configure Claude Code for Amazon Bedrock?
```

```text wrap theme={null}
what are the limitations of Claude Code?
```

> [!NOTE]
>
> Claude 基于文档提供对这些问题的答案。有关可执行示例和实际演示，请运行 `/powerup` 以获得带有动画演示的交互式课程，或参考上面的特定工作流程部分。

> [!TIP]
>
> 提示：
>
> * 提出具体问题以获得详细答案
> * Claude 可以解释复杂的功能，如 MCP 集成、企业配置和高级工作流程

***

## 恢复以前的对话

当任务跨越多个会话时，从您离开的地方继续，而不是重新解释上下文。Claude Code 在本地保存每个对话。

```bash theme={null}
claude --continue
```

这会恢复当前目录中最近的会话；如果还没有，它会打印 `No conversation found to continue` 并退出。使用 `claude --resume` 从列表中选择，或从运行中的会话内使用 `/resume`。有关命名、分支和完整选择器参考，请参阅[管理会话](https://code.claude.com/docs/zh-CN/sessions.md)。

## 使用 worktrees 运行并行会话

在一个终端中处理功能，同时 Claude 在另一个终端中修复错误，而不会编辑冲突。每个 [git worktree](https://git-scm.com/docs/git-worktree) 是其自己分支上的单独检出，从现有提交创建，因此存储库首先需要至少一个提交。

```bash theme={null}
claude --worktree feature-auth
```

在第二个终端中使用不同的名称运行相同的命令以启动隔离的并行会话。在没有提交的存储库中，该命令失败并显示 `Failed to resolve base branch "HEAD": git rev-parse failed`。有关清理、`.worktreeinclude` 和非 git VCS 支持，请参阅 [Worktrees](https://code.claude.com/docs/zh-CN/worktrees.md)。要从一个屏幕而不是单独的终端监视并行会话，请参阅[后台代理](https://code.claude.com/docs/zh-CN/agent-view.md)。

## 编辑前规划

对于您想在接触磁盘前审查的更改，切换到 plan mode。Claude 读取文件并提出计划，但在您批准前不进行任何编辑。状态栏在 plan mode 激活时显示 `⏸ plan mode on`。

```bash theme={null}
claude --permission-mode plan
```

您也可以在会话中按 `Shift+Tab` 直到状态栏显示 `⏸ plan mode on` 来切换到 plan mode。有关批准流程和在文本编辑器中编辑计划，请参阅 [Plan mode](https://code.claude.com/docs/zh-CN/permission-modes.md#analyze-before-you-edit-with-plan-mode)。

## 将研究委派给 subagents

探索大型代码库会用文件读取填充您的上下文。委派探索，以便只有发现结果返回。

```text wrap theme={null}
use a subagent to investigate how our auth system handles token refresh
```

subagent 在其自己的上下文窗口中读取文件并报告摘要。有关定义具有自己工具和提示的自定义代理，请参阅 [Subagents](https://code.claude.com/docs/zh-CN/sub-agents.md)。

## 将 Claude 管道输入脚本

以非交互方式运行 Claude 用于 CI、预提交钩子或批处理。stdin 和 stdout 像任何 Unix 工具一样工作。

```bash theme={null}
git log --oneline -20 | claude -p "summarize these recent commits"
```

有关输出格式、权限标志和扇出模式，请参阅[非交互模式](https://code.claude.com/docs/zh-CN/headless.md)。

## 后续步骤

> [card] [最佳实践](https://code.claude.com/docs/zh-CN/best-practices.md)
>
> 充分利用 Claude Code 的模式

> [card] [管理会话](https://code.claude.com/docs/zh-CN/sessions.md)
>
> 恢复、命名和分支对话

> [card] [Worktrees](https://code.claude.com/docs/zh-CN/worktrees.md)
>
> 运行隔离的并行会话

> [card] [扩展 Claude Code](https://code.claude.com/docs/zh-CN/features-overview.md)
>
> 添加 skills、hooks、MCP、subagents 和插件
