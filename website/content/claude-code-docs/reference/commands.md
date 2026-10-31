> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Commands

> Claude Code 中可用命令的完整参考，包括内置命令和捆绑的 skills。

Commands 从会话内部控制 Claude Code。它们提供了一种快速方式来切换模型、管理权限、清除上下文、运行工作流等。

输入 `/` 查看可用的命令，或输入 `/` 后跟字母来筛选。[命令菜单如何匹配你输入的内容](#how-the-command-menu-matches-what-you-type)涵盖了高亮显示、拼写错误以及 Claude Code 在你输入完整名称之前隐藏的少数命令。

命令只在您的消息开头被识别。命令名称后面的文本成为其参数。在终端提示符处，您可以在一条消息的开头使用多个 [skill](https://code.claude.com/docs/zh-CN/skills.md#stack-several-skills-in-one-message)：输入 `/skill-a /skill-b do XYZ` 会加载这两个 skill，并将尾部文本作为参数传递给每个 skill。最多可以链接六个 skill。

如果你在 Claude 响应时发送命令，Claude Code 会将其排队，并在当前轮次完成后运行。Claude Code 会立即运行某些命令而不中断响应，例如 `/status`、`/tasks` 和 `/usage`。在[全屏渲染](https://code.claude.com/docs/zh-CN/fullscreen.md)中，Claude Code 也会立即打开对话命令，例如 `/theme` 和 `/help`。在 v2.1.234 之前，Claude Code 会将这些对话排队直到轮次完成。

## 典型工作流程中的命令

大多数命令在会话的特定阶段很有用，从设置项目到发布更改。

**首次在仓库中的会话。** 运行 `/init` 生成一个启动 `CLAUDE.md`，然后运行 `/memory` 来完善它。使用 `/mcp` 设置项目需要的任何服务器，要求 Claude 创建你想要的任何 [subagents](https://code.claude.com/docs/zh-CN/sub-agents.md)，并运行 `/permissions` 来设置你的批准规则。

**执行任务期间。** `/plan` 在大型更改前切换到 plan mode。`/model` 和 `/effort` 调整你使用的模型以及它应用多少推理。当对话变得很长时，`/context` 显示什么在填充窗口，`/compact` 总结它以释放空间。使用 `/btw` 提出不应添加到对话历史的附带问题。

**并行运行工作。** Claude 将附带任务委派给 [subagents](https://code.claude.com/docs/zh-CN/sub-agents.md)，`/tasks` 列出当前会话的后台工作，包括已完成的 subagents。`/background` 分离整个会话以继续作为 [background agent](https://code.claude.com/docs/zh-CN/agent-view.md) 运行并释放你的终端。对于跨越代码库的大型更改，`/batch` 将其分解为独立单元并在各自的 [worktree](https://code.claude.com/docs/zh-CN/worktrees.md) 中运行每个单元。请参阅 [Run agents in parallel](https://code.claude.com/docs/zh-CN/agents.md) 了解这些方法如何相关。

**在你发布之前。** `/diff` 显示更改的内容。`/code-review` 检查当前 diff 是否存在正确性错误和清理，并可以使用 `--fix` 应用发现的问题；传递 PR 号码，例如 `/code-review high 1234`，以改为审查拉取请求。`/review` 是一个别名。`/code-review ultra` 在云中运行多代理审查。`/security-review` 检查 diff 是否存在安全漏洞。

**会话之间。** `/clear` 在保持项目记忆的同时开始新任务。`/resume` 返回到较早的对话，`/branch` 分支当前对话以尝试不同的方向，`/fork` 将其复制到新的 [后台会话](https://code.claude.com/docs/zh-CN/agent-view.md)。`/teleport` 将云端会话拉入此终端，`/remote-control` 让你从另一台设备继续此本地会话。

**当出现问题时。** `/rewind` 将代码和对话回滚到检查点，或总结对话的一部分。`/doctor` 运行设置检查以诊断安装和配置问题并可以修复它们，`/debug` 诊断运行时问题，`/feedback` 报告附加会话上下文的错误。

## 所有命令

下表列出了 Claude Code 中包含的所有命令。大多数是内置命令，其行为编码在 CLI 中。有两类条目带有标记：

* **[Skill](https://code.claude.com/docs/zh-CN/skills.md#bundled-skills)**：随附 skill。它的工作方式与您自己编写的 skill 相同：是交给 Claude 的提示词。
  * `/verify` 仅在您调用时运行。在 v2.1.215 之前，Claude 也可以自行运行 `/verify`。
* **[工作流](https://code.claude.com/docs/zh-CN/workflows.md#bundled-workflows)**：随附的[动态工作流](https://code.claude.com/docs/zh-CN/workflows.md)，会将工作分发到多个子代理并在后台运行。
  * `/deep-research` 仅在您调用时运行。

要添加您自己的命令，请参阅 [skill](https://code.claude.com/docs/zh-CN/skills.md)。

在下表中，`<arg>` 表示必需参数，`[arg]` 表示可选参数。

> [!NOTE]
>
> 并非每个命令都会对每个用户显示。可用性取决于您的平台、套餐和环境。例如，`/desktop` 仅在使用 Claude 订阅登录时才会在 macOS 和 x64 Windows 上显示，而 `/upgrade` 不会在 Enterprise 套餐上显示。

| 命令 | 用途 |
| :- | :- |
| `/add-dir <path>` | 添加一个工作目录，以便在当前会话期间访问文件。输入部分路径可查看匹配的目录建议；按 `Tab` 接受其中一个。大多数 `.claude/` 配置[不会从添加的目录中被发现](https://code.claude.com/docs/zh-CN/permissions.md#additional-directories-grant-file-access-not-configuration)。您无法添加大多数[网络路径](https://code.claude.com/docs/zh-CN/errors.md#working-directory-is-a-network-path)，例如 `\\server\share`。成功添加后，您的 [`DirectoryAdded` hook](https://code.claude.com/docs/zh-CN/hooks#directoryadded) 会运行。如果在 Claude 回复时运行此命令，Claude Code 会立即要求您确认该目录，确认后，Claude 在同一轮次中的下一次工具调用即可访问该目录。在 v2.1.234 之前，Claude Code 会将该命令排队，直到该轮次结束 |
| `/advisor [model\|off]` | 启用或禁用[顾问工具](https://code.claude.com/docs/zh-CN/advisor.md)，它会在任务的关键时刻咨询第二个模型以获取指导。接受 `fable`、`opus`、`sonnet` 或完整的模型 ID。`fable` 需要 [Fable 访问权限](https://code.claude.com/docs/zh-CN/advisor.md#choose-an-advisor-model)。不带参数时，打开选择器。在没有交互式终端的会话中，或通过 [Remote Control](https://code.claude.com/docs/zh-CN/remote-control.md#limitations) 使用时，请将模型或 `off` 作为参数传递；在这些情况下不带参数时，该命令会以文本形式输出当前顾问。这些形式需要 Claude Code v2.1.260 或更高版本 |
| `/agents` | 输出一条提醒，提示您请 Claude 创建或管理[子代理](https://code.claude.com/docs/zh-CN/sub-agents.md)，或直接编辑 `.claude/agents/` 或 `~/.claude/agents/`。在 v2.1.197 及更早版本中，打开用于创建和管理子代理配置的交互式界面 |
| `/artifact-capabilities` | **[Skill](https://code.claude.com/docs/zh-CN/skills.md#bundled-skills)。** 加载已发布的 [Artifact](https://code.claude.com/docs/zh-CN/artifacts.md) 可使用的运行时功能参考，例如[调用您的连接器](https://code.claude.com/docs/zh-CN/artifacts.md#pull-live-data-with-mcp-connectors)或[提供文件下载](https://code.claude.com/docs/zh-CN/artifacts.md#offer-a-file-download)，包括您的账户拥有哪些功能。Claude 通常会在构建使用其中某项功能的页面之前自行加载它。在 [Artifact 可用](https://code.claude.com/docs/zh-CN/artifacts.md#availability)的地方可用 |
| `/artifact-diagramming` | **[Skill](https://code.claude.com/docs/zh-CN/skills.md#bundled-skills)。** 加载供 Claude 在 [Artifact](https://code.claude.com/docs/zh-CN/artifacts.md) 中遵循的绘图指导：何时使用图表有帮助、绘制什么，以及如何编写在浅色和深色主题下都清晰可读的内联 SVG。需要 Claude Code v2.1.221 或更高版本 |
| `/artifacts` | 列出您拥有的或与您共享的 [Artifact](https://code.claude.com/docs/zh-CN/artifacts.md#find-an-artifact-again)，然后将其中一个附加到会话、在浏览器中打开，或复制其链接。在 [Artifact 可用](https://code.claude.com/docs/zh-CN/artifacts.md#availability)的地方可用 |
| `/auto-mode-setup` | 根据您的项目和最近的会话[起草 `autoMode.environment` 条目](https://code.claude.com/docs/zh-CN/auto-mode-config#generate-environment-entries)，然后审阅草稿并将其保存到您的用户设置中。在 Pro、Max、Ultra 或 Team 套餐上，需要 Claude Code v2.1.228 或更高版本，在原生 Windows 上则需要 v2.1.233 或更高版本。在 Enterprise 套餐上或使用 Claude Console API 密钥时，需要 v2.1.283 或更高版本。如果该命令不存在，请参阅 [`/auto-mode-setup` 返回 "Unknown command"](https://code.claude.com/docs/zh-CN/auto-mode-config#auto-mode-setup-returns-unknown-command) |
| `/autocompact [auto\|<tokens>]` | 设置自动压缩窗口：即上下文窗口填充到多满时 Claude Code 会自动压缩。传递大小（例如 `500k`），或传递 `auto` 以恢复为针对您的模型调整的窗口。Claude Code 会将该值保存到用户设置并应用于当前会话。有关接受的值以及哪些设置会覆盖它，请参阅[设置自动压缩窗口](https://code.claude.com/docs/zh-CN/model-config.md#set-the-auto-compact-window)。不带参数时，打开显示当前窗口的对话框。需要 Claude Code v2.1.221 或更高版本 |
| `/autofix-pr [prompt]` | 生成一个[云端会话](https://code.claude.com/docs/zh-CN/claude-code-on-the-web.md#auto-fix-pull-requests)，监视当前分支的 PR，并在 CI 失败或审阅者留下评论时推送修复。通过 `gh pr view` 从您检出的分支检测打开的 PR；要监视其他 PR，请先检出其分支。默认情况下，云端会话会被指示修复每个 CI 失败和审阅评论；传递提示词可给出不同的指令，例如 `/autofix-pr only fix lint and type errors`。需要 `gh` CLI 以及[云端会话](https://code.claude.com/docs/zh-CN/claude-code-on-the-web.md)访问权限 |
| `/background [prompt]` | 将当前会话分离，作为[后台 Agent](https://code.claude.com/docs/zh-CN/agent-view.md) 运行，并释放此终端。传递提示词可在分离前再发送一条指令。使用 `claude agents` 监视该会话。要将对话复制到新的后台会话中，同时让当前会话继续运行，请使用 `/fork`。别名：`/bg` |
| `/batch <instruction>` | **[Skill](https://code.claude.com/docs/zh-CN/skills.md#bundled-skills)。** 在代码库中并行编排大规模更改。研究代码库，将工作分解为 5 到 30 个独立单元，并呈现计划。获得批准后，为每个单元在隔离的 [worktree](https://code.claude.com/docs/zh-CN/worktrees.md) 中生成一个[后台子代理](https://code.claude.com/docs/zh-CN/sub-agents.md#run-subagents-in-foreground-or-background)。每个子代理实现其单元、运行测试并发布其更改。需要 git 仓库，或一个用于创建 worktree 的 [`WorktreeCreate` hook](https://code.claude.com/docs/zh-CN/worktrees#non-git-version-control)。在 git 仓库之外，`/batch` 需要 Claude Code v2.1.281 或更高版本。示例：`/batch migrate src/ from JavaScript to TypeScript` |
| `/branch [name]` | 在此处创建当前对话的分支，以便您可以尝试不同的方向而不丢失当前的对话。会将您切换到该分支并保留原始对话，您可以使用 `/resume` 返回原始对话。要将副本作为单独的[后台会话](https://code.claude.com/docs/zh-CN/agent-view.md)运行而不切换进去，请使用 `/fork`；要将附带任务交给会向此对话汇报结果的[子代理](https://code.claude.com/docs/zh-CN/sub-agents.md)，请使用 `/subtask` |
| `/btw [question]` | 询问有关当前会话的[附带问题](https://code.claude.com/docs/zh-CN/interactive-mode.md#side-questions-with-%2Fbtw)，而不将其添加到对话中。如果不带问题运行 `/btw`，Claude Code 会显示您最近的附带问题，以便您浏览之前的回答；如果您还没有提问过，Claude Code 会输出一行用法说明 |
| `/bug [report]` | 报告 bug 或分享您的对话。您可以选择包含多少会话历史，并在发送任何内容之前在同意屏幕上确认。当您通过第一方连接登录 Anthropic 时，报告会发送给 Anthropic；在第三方提供商上，或没有 Anthropic 凭据时，Claude Code 会将报告写入[位于 `~/.claude/feedback-bundles/` 下的本地归档](https://code.claude.com/docs/zh-CN/data-usage#telemetry-services)，由您自行转发。在 [VS Code 扩展](https://code.claude.com/docs/zh-CN/vs-code.md#use-the-prompt-box)中，`/bug` 会改为打开扩展自己的反馈对话框；需要 Claude Code v2.1.229 或更高版本。如果在 Claude 回复时运行此命令，Claude Code 会立即打开对话框。在 v2.1.232 之前，Claude Code 会将该命令排队，直到该轮次结束。别名：`/share`。在 v2.1.212 之前，`/bug` 和 `/share` 是 `/feedback` 的别名 |
| `/cd <path>` | 将此会话移动到新的工作目录，同时保留对话。输入部分路径可查看匹配的目录建议；按 `Tab` 接受其中一个。有关移动后 Claude Code 会立即从新目录应用哪些内容，以及 `/cd` 与 `/add-dir` 的区别，请参阅[将会话移动到另一个目录](https://code.claude.com/docs/zh-CN/permissions.md#move-the-session-to-another-directory) |
| `/chrome` | 配置 [Claude in Chrome](https://code.claude.com/docs/zh-CN/chrome.md) 设置 |
| `/claude-api [migrate\|upgrade\|api-version\|managed-agents-onboard\|managed-agents-secure\|prompt-audit\|cost-optimize\|build-eval\|hillclimb\|preserved-thinking-migration]` | **[Skill](https://code.claude.com/docs/zh-CN/skills.md#bundled-skills)。** 加载适用于您项目语言的 [Claude API](https://platform.claude.com/docs/en/api/overview) 和 [Managed Agents](https://platform.claude.com/docs/en/managed-agents/overview) 参考资料。当您的代码导入 `anthropic` 或 `@anthropic-ai/sdk` 时也会自动激活。有关每个子命令的作用及其所需版本，请参阅[处理 Claude API 项目](https://code.claude.com/docs/zh-CN/skills.md#work-on-claude-api-projects) |
| `/claude-in-chrome [task]` | **[Skill](https://code.claude.com/docs/zh-CN/skills.md#bundled-skills)。** 让 Claude 通过 [Claude in Chrome](https://code.claude.com/docs/zh-CN/chrome.md) 在您的浏览器中执行任务，例如测试页面、填写表单或读取控制台日志。当会话启用了 Chrome 集成时可用（例如使用 `claude --chrome`），或者当 Claude Code 可以提议[安装扩展](https://code.claude.com/docs/zh-CN/chrome.md#install-the-extension-when-claude-asks)时可用 |
| `/clear [name]` | 使用空上下文开始新对话。传递名称可在 `/resume` 选择器中为之前的对话添加标签。要在继续同一对话的同时释放上下文，请改用 `/compact`。使用 `/resume` 恢复之前的对话，或者在同一 Claude Code 进程中，从[回退菜单的上一个会话条目](https://code.claude.com/docs/zh-CN/checkpointing.md#rewind-past-a-cleared-conversation)恢复。别名：`/reset`、`/new` |
| `/code-review [low\|medium\|high\|xhigh\|max\|ultra] [--fix] [--comment] [--max-findings n\|all\|default] [pr#\|branch\|path]` | **[Skill](https://code.claude.com/docs/zh-CN/skills.md#bundled-skills)。** 审查当前 diff，或您传递的 PR 编号、分支或路径，查找正确性 bug。根据您的模型和 effort 级别，审查还会涵盖清理机会。传递 `--fix` 可让 Claude 应用发现的问题，传递 `--comment` 可将其发布到 GitHub PR 或 GitLab Merge Request 上，传递 `ultra` 可运行深度[云端审查](https://code.claude.com/docs/zh-CN/ultrareview.md)。发布到 GitLab Merge Request 需要 Claude Code v2.1.257 或更高版本。在以 `github.com` PR 为目标使用 `ultra` 时，传递 `--post` 可在启动对话框中预先选择[将完成的发现发布到 PR](https://code.claude.com/docs/zh-CN/ultrareview.md#post-findings-to-the-pull-request)；`--post` 需要 Claude Code v2.1.227 或更高版本。有关 effort 级别、目标指定方式以及它与 `/simplify` 的关系，请参阅[使用 `/code-review` 在本地审查](https://code.claude.com/docs/zh-CN/code-review-local)。别名：`/review` |
| `/color [color\|default]` | 设置当前会话的提示栏颜色。可用颜色：`red`、`blue`、`green`、`yellow`、`purple`、`orange`、`pink`、`cyan`。使用 `default` 重置，或不带参数运行以随机选择一种颜色。连接 [Remote Control](https://code.claude.com/docs/zh-CN/remote-control.md) 时，颜色会同步到 claude.ai/code。也可在非交互模式（`-p`）中使用 |
| `/compact [instructions]` | 通过总结到目前为止的对话来释放上下文。可选择传递摘要的重点指令。请参阅[压缩如何处理规则、skill 和记忆文件](https://code.claude.com/docs/zh-CN/context-window.md#what-survives-compaction) |
| `/config [key=value ...]` | 打开[设置](https://code.claude.com/docs/zh-CN/settings.md)界面，以调整主题、模型、[输出样式](https://code.claude.com/docs/zh-CN/output-styles.md)和其他偏好。传递一个或多个 `key=value` 对可直接设置某项设置而无需打开界面，例如 `/config thinking=false`、`/config theme=dark` 或 `/config model=sonnet`。`key=value` 形式也适用于非交互模式（`-p`），以及通过 [Remote Control](https://code.claude.com/docs/zh-CN/remote-control.md) 从 Claude 移动应用使用。`key=value` 形式无法开启需要您在面板中确认的设置，例如 [`autoContinueAtUsageLimit`](https://code.claude.com/docs/zh-CN/interactive-mode#turn-automatic-continue-off)，但可以将其关闭。运行 `/config --help` 可列出其接受的键。别名：`/settings` |
| `/context [all]` | 以彩色网格形式可视化当前上下文使用情况。显示针对占用大量上下文的工具、记忆膨胀的优化建议以及容量警告。当对话超出上下文窗口时，输出会包含一条[警告](https://code.claude.com/docs/zh-CN/errors.md#context-exceeds-the-token-limit)，显示超出限制多少以及哪个命令可以释放空间。在[全屏模式](https://code.claude.com/docs/zh-CN/fullscreen.md)下，`/context` 会折叠逐项明细以保持网格可见。传递 `all` 可将其展开 |
| `/copy [N]` | 将最后一条助手回复复制到剪贴板。传递数字 `N` 可复制倒数第 N 条回复：`/copy 2` 复制倒数第二条。当存在代码块或引用块时，会显示交互式选择器，以选择单个块或完整回复。在选择器中按 `w` 可将所选内容写入文件而不是剪贴板，这在通过 SSH 使用时很有用 |
| `/cost` | `/usage` 的别名 |
| `/dataviz [request]` | **[Skill](https://code.claude.com/docs/zh-CN/skills.md#bundled-skills)。** 图表、图形和仪表板的设计指导。Claude 会为数据选择图表形式，按角色分配颜色，使用随附脚本验证调色板的色盲安全性和对比度，并应用标记、交互和无障碍规则。使用品牌中立的占位调色板，供您替换为自己的调色板 |
| `/debug [description]` | **[Skill](https://code.claude.com/docs/zh-CN/skills.md#bundled-skills)。** 为当前会话启用调试日志记录，并通过读取会话调试日志来排除问题。除非您使用 `claude --debug` 启动，否则调试日志记录默认关闭，因此在会话中途运行 `/debug` 会从该时刻开始捕获日志。可选择描述问题以聚焦分析 |
| `/deep-research <question>` | **[工作流](https://code.claude.com/docs/zh-CN/workflows.md#bundled-workflows)。** 针对一个问题分散进行网络搜索，获取并交叉核对来源，并综合生成带引用的报告 |
| `/design [brief]` | **[Skill](https://code.claude.com/docs/zh-CN/skills.md#bundled-skills)。** 在一个画布上以画板形式起草 UI 模型、屏幕流程、着陆页或海报，并发布为 Claude Design [Artifact](https://code.claude.com/docs/zh-CN/artifacts.md#draft-a-design-canvas)，例如 `/design a settings screen for a mobile banking app`。您可以在桌面浏览器中编辑画板，编辑内容会自动保存。您可以将每个画板导出为 PNG 或 PDF。需要 Claude Code v2.1.265 或更高版本、[Artifact 可用](https://code.claude.com/docs/zh-CN/artifacts.md#availability)的会话，以及 [Design 模板可用](https://code.claude.com/docs/zh-CN/artifacts.md#start-from-a-slides-design-or-docs-template)的账户；如果您的组织已关闭该模板，`/design` 不会起草设计。可在 Anthropic API 上使用。在 Amazon Bedrock、Google Cloud's Agent Platform、Microsoft Foundry 和 Claude Platform on AWS 上，Artifact 不可用，因此该命令在这些平台上不可用 |
| `/design consent`, `/design revoke` | 授予 Claude 访问 [Claude Design](https://claude.ai/design) 的权限，或撤销该权限。当 Claude 的回复要求您运行 `/design consent` 时，请自行在 Claude Code 输入框中输入该命令 |
| `/design-login` | 使用您的 claude.ai 账户为 `/design-sync` 授权设计系统访问。每当 `/design-sync` 或 `/design consent` 提示您登录时，请使用 `/design-login` 登录。此登录用于位于 claude.ai/design 的独立 Claude Design 站点，该站点将于 2026 年 12 月 14 日前关闭；请参阅[从独立 Claude Design 迁移到 Claude](https://support.claude.com/en/articles/17440474-migrate-from-standalone-claude-design-to-claude) |
| `/design-sync [hint]` | **[Skill](https://code.claude.com/docs/zh-CN/skills.md#bundled-skills)。** 转换您仓库中的 React 设计系统并将其上传到 [Claude Design](https://claude.ai/design)，使其生成的设计使用您的真实组件。可选择为设计系统命名，例如 `/design-sync Acme DS`。首次同步会验证每个组件，在大型仓库上可能需要几个小时。可在 Anthropic API 上使用。它需要 claude.ai，而 CLI 在 Amazon Bedrock、Google Cloud's Agent Platform、Microsoft Foundry 或 Claude Platform on AWS 上，或通过 [Claude apps gateway](https://code.claude.com/docs/zh-CN/claude-apps-gateway#availability-and-limitations) 使用时不会连接 claude.ai。该命令在这些情况下不可用。此命令会上传到独立的 claude.ai/design 站点，该站点将于 2026 年 12 月 14 日前关闭；请参阅[从独立 Claude Design 迁移到 Claude](https://support.claude.com/en/articles/17440474-migrate-from-standalone-claude-design-to-claude) |
| `/desktop` | 在 Claude Code Desktop 应用中继续当前会话。需要 macOS 或 x64 Windows 以及 Claude 订阅。别名：`/app` |
| `/diff` | 审查工作树中的更改，包括 Claude 到目前为止所做的编辑。请参阅[使用 /diff 审查更改](https://code.claude.com/docs/zh-CN/interactive-mode.md#review-changes-with-%2Fdiff) |
| `/doctor [prompt-audit [path]]` | **[Skill](https://code.claude.com/docs/zh-CN/skills.md#bundled-skills)。** 运行设置检查，诊断安装、设置、扩展和 `CLAUDE.md` 方面的问题，并提出修复建议，Claude 会在您确认后应用这些修复。有关检查涵盖的内容，或要改用 `prompt-audit` 审计您的指令，请参阅[使用 `/doctor` 检查您的设置](https://code.claude.com/docs/zh-CN/skills#check-your-setup-with-/doctor)。`prompt-audit` 子命令需要 Claude Code v2.1.283 或更高版本。别名：`/checkup` |
| `/effort [level\|auto\|status\|ultracode [on\|off]]` | 设置 [effort 级别](https://code.claude.com/docs/zh-CN/model-config.md#adjust-effort-level)：`low` 到 `xhigh`、`max` 或 `auto`；`status` 会输出当前级别。`ultracode` 或 `ultracode on` 会为会话开启 [ultracode](https://code.claude.com/docs/zh-CN/workflows.md#let-claude-decide-with-ultracode)，`ultracode off` 会将其关闭；[`ultracode`](https://code.claude.com/docs/zh-CN/settings-reference#ultracode) 键会持久保存。`max` 仅对当前会话有效。`on` 和 `off` 参数需要 Claude Code v2.1.284 或更高版本。如果您在 Claude 工作时运行 `/effort`，请参阅[新级别何时生效](https://code.claude.com/docs/zh-CN/model-config.md#when-a-new-effort-level-starts)。在 v2.1.205 或更高版本中可在 `-p` 中使用 |
| `/exit` | 退出 CLI。在已附加的[后台会话](https://code.claude.com/docs/zh-CN/agent-view.md#attach-to-a-session)中，此操作会分离，会话继续运行。别名：`/quit` |
| `/export [filename]` | 将当前对话导出为纯文本。提供文件名时，直接写入该文件。不提供时，打开对话框以复制到剪贴板或保存到文件 |
| `/fast [on\|off]` | 开启或关闭[快速模式](https://code.claude.com/docs/zh-CN/fast-mode.md)。在 Claude 回复时运行此命令，Claude Code 会切换快速模式而无需等待轮次结束，但正在运行的轮次仍会以其原始速度完成。在 v2.1.242 之前，Claude Code 会根据从 Anthropic 获取的功能标志来决定是在轮次中途运行该命令，还是将其排队直到轮次结束，并且在不[获取功能标志](https://code.claude.com/docs/zh-CN/env-vars.md#features-that-need-feature-flag-fetching)的会话中始终将其排队。在使用 `-p` 的非交互模式中可用性有限；请参阅[切换快速模式](https://code.claude.com/docs/zh-CN/fast-mode.md#toggle-fast-mode) |
| `/feedback [report]` | 发送有关 Claude Code 的产品反馈。打开与 [`/bug`](#all-commands) 相同的对话框，具有相同的同意步骤、发送规则和轮次中途行为。在具有 [Claude 起草的反馈](https://code.claude.com/docs/zh-CN/tools-reference.md#sendfeedback-tool-behavior)的会话中，不带参数的 `/feedback` 会改为打开草稿队列，您可以在其中审阅、编辑、发送或丢弃 Claude 排入队列的草稿；该队列包含一个在对话框中撰写新报告的选项。带参数时，以及对于 `/bug` 始终如此，对话框会直接打开 |
| `/fewer-permission-prompts` | **[Skill](https://code.claude.com/docs/zh-CN/skills.md#bundled-skills)。** 扫描您的会话记录，查找常见的只读 Bash 和 MCP 工具调用，然后将按优先级排序的允许列表添加到项目的 `.claude/settings.json` 中，以减少权限提示 |
| `/focus` | 切换专注视图，该视图仅显示您的最后一条提示词、带有编辑 diffstat 的单行工具调用摘要以及最终回复。工具调用摘要还会统计该轮次中启动的子代理数量，并将已完成的后台任务通知合并为单个计数。该选择会跨会话保持；在设置中设置 [`viewMode`](https://code.claude.com/docs/zh-CN/settings-reference#viewmode) 可覆盖它。仅在[全屏渲染](https://code.claude.com/docs/zh-CN/fullscreen.md)中可用。在 [Remote Control](https://code.claude.com/docs/zh-CN/remote-control.md) 客户端中，运行 `/focus [on\|off]` 可仅为当前会话开启或关闭专注视图，而不更改您保存的选择；这需要 Claude Code v2.1.281 或更高版本。[VS Code 扩展](https://code.claude.com/docs/zh-CN/vs-code.md#use-the-prompt-box)提供其自己的 Focus 视图，作为命令菜单中的开关，存储为扩展设置，独立于 `viewMode` |
| `/fork [prompt]` | [将当前对话复制](https://code.claude.com/docs/zh-CN/agent-view.md#copy-the-session-with-%2Ffork)到新的后台会话中，并在此处继续工作。传递提示词后，副本会立即开始处理；不传递时，副本会在 Agent 视图中等待其第一条提示词。除非副本[就地编辑](https://code.claude.com/docs/zh-CN/agent-view.md#how-file-edits-are-isolated)，否则 Claude Code 会指示它在进行代码更改之前创建自己的 worktree；该隔离指令需要 Claude Code v2.1.221 或更高版本。要将附带任务交给结果会返回到此对话的子代理，请使用 `/subtask`；要自己切换到副本中，请使用 `/branch`。当[关闭 Agent 视图](https://code.claude.com/docs/zh-CN/agent-view.md#turn-off-agent-view)时，`/fork` 会改为启动[分叉子代理](https://code.claude.com/docs/zh-CN/sub-agents.md#fork-the-current-conversation) |
| `/goal [condition\|clear]` | 设置[目标](https://code.claude.com/docs/zh-CN/goal.md)：Claude 会跨轮次持续工作，直到满足条件或目标[因其他原因被清除](https://code.claude.com/docs/zh-CN/goal.md#how-evaluation-works)。不带参数时，显示当前目标或最近达成的目标。`clear`、`stop`、`off`、`reset`、`none` 或 `cancel` 可提前移除活动目标 |
| `/heapdump` | 将 JavaScript 堆快照和内存明细写入 `~/Desktop`（在没有 Desktop 文件夹的 Linux 上则写入主目录），用于诊断高内存使用率。报告内存问题时只附加 `-diagnostics.json` 文件；`.heapsnapshot` 包含您的完整对话和凭据，因此请勿分享它。[在命令菜单中隐藏](#how-the-command-menu-matches-what-you-type)；需要完整输入。请参阅[如何处理输出](https://code.claude.com/docs/zh-CN/troubleshooting.md#high-cpu-or-memory-usage) |
| `/help` | 显示帮助和可用命令 |
| `/hooks` | 查看 [hook](https://code.claude.com/docs/zh-CN/hooks.md#the-%2Fhooks-menu) 配置 |
| `/ide` | 管理 IDE 集成并显示状态 |
| `/import [codex\|gemini\|cursor] [--dry-run] [--yes]` | 将您计算机上 OpenAI Codex、Google Gemini CLI 或 Cursor 中的配置导入 Claude Code，包括指令文件、MCP 服务器、命令、子代理和 skill。在使用 `-p` 的[非交互模式](https://code.claude.com/docs/zh-CN/headless.md)中，`/import` 会列出它找到的内容，并给出用于确认导入的命令。添加 `--dry-run` 可预览而不写入任何内容，或添加 `--yes` 以跳过交互式选择器。在 Amazon Bedrock、Google Cloud's Agent Platform、Microsoft Foundry 或 Claude Platform on AWS 上，或通过 [Claude apps gateway](https://code.claude.com/docs/zh-CN/claude-apps-gateway#availability-and-limitations) 使用时不可用。当您关闭[功能标志获取](https://code.claude.com/docs/zh-CN/env-vars.md#features-that-need-feature-flag-fetching)时也不可用。从 Cursor 导入需要 v2.1.265 或更高版本 |
| `/init` | 使用 `CLAUDE.md` 指南初始化项目。设置 `CLAUDE_CODE_NEW_INIT=1` 可使用交互式流程，该流程还会引导您完成 skill、hook 和个人记忆文件的设置。如果 `/init` 发现 OpenAI Codex 或 Google Gemini CLI 配置，它会提议使用 `/import` 将其迁移过来 |
| `/insights` | 生成 HTML 报告，分析您在此计算机上的最近会话：您在哪些项目中工作、如何使用 Claude Code、哪里出现问题，以及值得尝试的功能。在[云端会话](https://code.claude.com/docs/zh-CN/claude-code-on-the-web.md)中不可用。有关报告位置、保留期限和开销，请参阅[分析您的使用模式](https://code.claude.com/docs/zh-CN/costs.md#analyze-your-usage-patterns) |
| `/install-github-app` | 为仓库安装 Claude GitHub App，并可选择设置 [GitHub Actions](https://code.claude.com/docs/zh-CN/github-actions.md) 工作流和密钥。引导您选择仓库并配置集成。仅适用于 github.com 仓库。当您仓库的 git 远程位于 gitlab.com 或 bitbucket.org 上时，该命令会输出一条通知并退出，而不是开始设置。要从 GitLab 流水线运行 Claude Code，请参阅 [GitLab CI/CD](https://code.claude.com/docs/zh-CN/gitlab-ci-cd.md) |
| `/install-slack-app` | 安装 Claude Slack 应用。打开浏览器以完成 OAuth 流程 |
| `/keybindings` | 打开您的[快捷键](https://code.claude.com/docs/zh-CN/keybindings.md)文件 |
| `/list-agents` | 列出 Claude 可以向其发送消息的子代理、[agent team](https://code.claude.com/docs/zh-CN/agent-teams.md) 队友以及其他 Claude Code 会话，并显示每一项应使用的名称。请参阅[跨会话消息传递](https://code.claude.com/docs/zh-CN/cross-session-messaging.md)。也可作为 `/peers` 使用。需要 Claude Code v2.1.224 或更高版本；更早版本会报告 `Unknown command: /list-agents`。队友行以及显示此会话自身名称的第一行需要 v2.1.239 或更高版本。仅在[已启用跨会话消息传递](https://code.claude.com/docs/zh-CN/cross-session-messaging.md#availability)的会话中可用 |
| `/login` | 登录您的 Anthropic 账户 |
| `/logout` | 从您的 Anthropic 账户注销 |
| `/loop [interval] [prompt]` | **[Skill](https://code.claude.com/docs/zh-CN/skills.md#bundled-skills)。** 在会话保持打开期间重复运行提示词。省略间隔时，Claude 会[自行决定迭代之间的节奏](https://code.claude.com/docs/zh-CN/scheduled-tasks.md#let-claude-choose-the-interval)。省略提示词时，Claude 会运行[内置维护提示词](https://code.claude.com/docs/zh-CN/scheduled-tasks.md#run-the-built-in-maintenance-prompt)或您的 [`loop.md`](https://code.claude.com/docs/zh-CN/scheduled-tasks#customize-the-default-prompt-with-loop-md)。示例：`/loop 5m check if the deploy finished`。请参阅[按计划运行提示词](https://code.claude.com/docs/zh-CN/scheduled-tasks.md)。别名：`/proactive` |
| `/mcp [reconnect (<server>\|all)\|enable\|disable [<server>\|all]]` | 管理 MCP 服务器连接和 OAuth 身份验证。不带参数运行可打开交互式列表，或传递 `reconnect`、`enable` 或 `disable` 以及服务器名称或 `all`，可在不打开列表的情况下更改连接状态。`reconnect all` 会[重试每个失败或需要身份验证的服务器](https://code.claude.com/docs/zh-CN/mcp.md#retry-failed-servers-yourself)。在非交互模式（`-p`）中，不带参数运行会输出服务器状态的文本摘要，而不是打开列表 |
| `/memory` | 编辑 `CLAUDE.md` 文件，启用或禁用[自动记忆](https://code.claude.com/docs/zh-CN/memory.md#auto-memory)，并查看自动记忆条目 |
| `/mobile` | 显示用于下载 Claude 移动应用的二维码。别名：`/ios`、`/android` |
| `/model [model]` | 切换 AI 模型并将其保存为新会话的默认模型。对于支持的模型，使用左/右箭头[调整 effort 级别](https://code.claude.com/docs/zh-CN/model-config.md#adjust-effort-level)。不带参数时，打开选择器；在某一行上按 `s` 可仅为当前会话切换。请参阅[新模型何时生效](https://code.claude.com/docs/zh-CN/interactive-mode.md#when-claude-code-sends-what-you-queued)。在 v2.1.242 之前，切换可能需要等待。使用 `-p` 时，请传递模型参数，该参数不会保存为默认模型；需要 v2.1.205 或更高版本 |
| `/output-style [style]` | 列出[输出样式](https://code.claude.com/docs/zh-CN/output-styles.md)或切换到其中一种，例如 `/output-style concise`。请参阅[更改输出样式](https://code.claude.com/docs/zh-CN/output-styles.md#change-your-output-style)。需要 Claude Code v2.1.269 或更高版本 |
| `/passes` | 与朋友分享一周免费的 Claude Code。仅在您的账户符合条件时可见 |
| `/permissions` | 管理工具权限的允许、询问和拒绝规则。打开交互式对话框，您可以在其中按作用域查看规则、添加或删除规则、管理工作目录，以及查看[最近的自动模式拒绝记录](https://code.claude.com/docs/zh-CN/auto-mode-config.md#review-denials)。您还可以从对话框的 **Auto mode** 选项卡查看和编辑[自动模式分类器规则](https://code.claude.com/docs/zh-CN/auto-mode-config.md#edit-rules-from-permissions)。如果在 Claude 回复时运行此命令，Claude Code 会立即打开对话框，并从 Claude 在同一轮次中的下一次工具调用开始应用您的更改。在 v2.1.234 之前，Claude Code 会将该命令排队，直到该轮次结束。别名：`/allowed-tools` |
| `/plan [description]` | 直接从输入框进入计划模式。传递可选描述可进入计划模式并立即开始处理该任务，例如 `/plan fix the auth bug` |
| `/plugin [subcommand]` | 管理 Claude Code [插件](https://code.claude.com/docs/zh-CN/plugins/overview.md)。不带参数运行可打开插件菜单，或传递 `list`、`install`、`enable` 或 `disable` 等子命令以直接执行操作。Claude Code 可以在安装期间激活插件；[安装摘要](https://code.claude.com/docs/zh-CN/plugins/install.md#install-a-plugin)会告诉您插件是否已激活，或是否需要运行 `/reload-plugins` |
| `/plugin-authoring` | 加载 Claude 用于[编写 mod](https://code.claude.com/docs/zh-CN/plugins/mods/create.md#ask-claude-for-a-mod) 的参考资料。当您请求 mod 时，Claude 可以自行加载它。这是一个来自[内置插件](https://code.claude.com/docs/zh-CN/plugins/mods/overview.md#mods-built-into-claude-code)的 skill，您可以在 `/plugin` 中将其关闭。需要 Claude Code v2.1.287 或更高版本 |
| `/powerup` | 通过带有动画演示的快速交互式课程了解 Claude Code 功能 |
| `/pr-comments [PR]` | 已在 v2.1.91 中移除。请改为直接让 Claude 查看 Pull Request 评论。在更早版本中，获取并显示 GitHub Pull Request 中的评论；自动检测当前分支的 PR，或传递 PR URL 或编号。需要 `gh` CLI |
| `/privacy-settings` | 查看和更新您的隐私设置。仅适用于 Pro、Max 和 Ultra 套餐订阅者 |
| `/radio` | 在浏览器中打开 Claude FM lo-fi 电台。没有可用浏览器时输出流 URL |
| `/rate-limit-options` | 显示当 claude.ai 用量限制阻止请求时继续工作的方法：等待并[在限制重置时自动继续](https://code.claude.com/docs/zh-CN/interactive-mode.md#wait-for-a-usage-limit-to-reset)、添加[使用额度](https://code.claude.com/docs/zh-CN/costs.md#add-usage-credits-to-your-subscription)，或升级您的套餐。当您在自己的终端中遇到限制时，Claude Code 也可能会自行打开此菜单。请参阅[关闭自动继续](https://code.claude.com/docs/zh-CN/interactive-mode.md#turn-automatic-continue-off)。需要 claude.ai 订阅。等待并继续的选项行需要 Claude Code v2.1.234 或更高版本 |
| `/recap` | 按需生成当前会话的单行摘要。有关您离开一段时间后出现的自动回顾，请参阅[会话回顾](https://code.claude.com/docs/zh-CN/interactive-mode.md#session-recap) |
| `/release-notes` | 在交互式版本选择器中查看更新日志。选择特定版本以查看其发布说明，或选择显示所有版本。这些说明会出现在您的会话记录中，但不会进入 Claude 看到的对话 |
| `/reload-plugins [--force]` | 重新加载所有活动[插件](https://code.claude.com/docs/zh-CN/plugins/overview.md)以应用待处理的更改，无需重启。报告每个重新加载组件的数量，并标记任何加载错误。当重新加载会改变已加载的 MCP 工具并使提示缓存失效时，该命令会发出警告并跳过，除非您传递 `--force`。也可在非交互模式（`-p`）、Agent SDK 和桌面应用中使用，在这些环境中它仅对直接输入到会话中的内容运行，并且不会应用插件 MCP 服务器的更改；需要 Claude Code v2.1.260 或更高版本。请参阅[无需重启即可应用插件更改](https://code.claude.com/docs/zh-CN/plugins/cli-reference.md#reload-plugins) |
| `/reload-skills` | 重新扫描 [skill](https://code.claude.com/docs/zh-CN/skills.md) 和命令目录，使会话期间在磁盘上添加或更改的 skill 无需重启即可使用。报告可用的 skill 数量以及添加或移除的数量 |
| `/remote-control` | 使此会话可通过 claude.ai 进行 [Remote Control](https://code.claude.com/docs/zh-CN/remote-control.md)。在未登录时运行，会输出 Remote Control 需要 claude.ai 订阅的提示，并告诉您如何登录。别名：`/rc` |
| `/remote-env` | 为您从 CLI 启动的云端会话选择默认[云环境](https://code.claude.com/docs/zh-CN/cloud-environments.md#select-an-environment-from-the-cli) |
| `/rename [name]` | 重命名当前会话并在提示栏上显示名称。不提供名称时，根据对话历史自动生成一个名称。也可在非交互模式（`-p`）中使用。在所有重命名入口（包括 claude.ai 和桌面应用）中，Claude Code 都会将新名称中的控制字符和不可见字符替换为空格，并将名称长度上限设为 200 个字符。如果移除不可见字符后名称为空，Claude Code 会拒绝该名称并显示 `That name is empty once invisible characters are removed. Usage: /rename <name>`。字符替换和长度上限需要 Claude Code v2.1.221 或更高版本 |
| `/restart` | 重启 Claude Code 并恢复此会话，从而应用已安装的更新。在更新安装后页脚提供此选项时运行它。需要 Pro、Max 或 Ultra 套餐，以及在 macOS 或 Linux（非 WSL）上的原生安装。需要 Claude Code v2.1.288 或更高版本。别名：`/update` |
| `/resume [session]` | 通过 ID 或名称恢复对话，或打开会话选择器。[后台会话](https://code.claude.com/docs/zh-CN/agent-view.md)在选择器中以 `bg` 标记显示。恢复仍在运行的后台会话时（无论是从选择器还是通过 ID 或名称），会[打开该会话](https://code.claude.com/docs/zh-CN/sessions.md#resume-a-running-background-session)：您当前的对话会移到后台，此终端会附加到正在运行的会话。在空输入框中按 `←` 可返回 Agent 视图，其中也会列出您离开的对话。在 v2.1.285 之前，Claude Code 会拒绝并告诉您使用 `claude attach` 打开该会话，或先停止它。别名：`/continue` |
| `/review [low\|medium\|high\|xhigh\|max\|ultra] [--fix] [--comment] [--max-findings n\|all\|default] [pr#\|branch\|path]` | [`/code-review`](https://code.claude.com/docs/zh-CN/code-review-local) 的别名：审查当前 diff，或您传递的 PR 编号、分支或路径（例如 `/review 1234`），并接受相同的 effort 级别和标志。未指定级别时，审查会沿用您上次输入的 `low` 到 `max` 级别；有关确切规则，请参阅[使用 `/code-review` 在本地审查](https://code.claude.com/docs/zh-CN/code-review-local)。要进行深度云端审查，请使用 [`/code-review ultra`](https://code.claude.com/docs/zh-CN/ultrareview)。在 v2.1.223 之前，`/review` 是一个单独的命令，按编号对 GitHub Pull Request 运行单次只读审查，不带参数运行时会列出打开的 PR 供选择；从 v2.1.186 到 v2.1.201，它运行与 `/code-review medium` 相同的多 Agent 引擎 |
| `/rewind` | 将对话和/或代码回退到之前的某个时间点，或从选定的消息开始总结。请参阅[检查点功能](https://code.claude.com/docs/zh-CN/checkpointing.md)。别名：`/checkpoint`、`/undo` |
| `/run` | **[Skill](https://code.claude.com/docs/zh-CN/skills.md#bundled-skills)。** 启动并操作您项目的应用，以查看更改是否实际生效，而不仅仅是通过测试。请参阅[运行并验证您的应用](https://code.claude.com/docs/zh-CN/skills.md#run-and-verify-your-app) |
| `/run-skill-generator` | **[Skill](https://code.claude.com/docs/zh-CN/skills.md#bundled-skills)。** 通过编写一个项目专属的 [skill](https://code.claude.com/docs/zh-CN/skills.md#run-and-verify-your-app)，教会 `/run` 和 `/verify` 如何在干净的环境中构建、启动和操作您项目的应用 |
| `/sandbox` | 切换[沙箱模式](https://code.claude.com/docs/zh-CN/sandboxing.md)。仅在支持的平台上可用 |
| `/schedule [description]` | 创建、更新、列出或运行[云端定时任务](https://code.claude.com/docs/zh-CN/routines.md)（以前称为 Routine）。Claude 会以对话方式引导您完成设置。您还可以询问[定时任务的最近运行情况](https://code.claude.com/docs/zh-CN/routines.md#manage-scheduled-tasks-from-the-cli)。别名：`/routines` |
| `/scroll-speed` | 以交互方式调整鼠标滚轮[滚动速度](https://code.claude.com/docs/zh-CN/fullscreen.md#mouse-wheel-scrolling)。仅在[全屏渲染](https://code.claude.com/docs/zh-CN/fullscreen.md)中可用，在 JetBrains IDE 终端中不可用 |
| `/security-review` | 分析当前分支上的更改是否存在安全漏洞。审查您的分支与 origin 默认分支之间的 diff，识别注入、身份验证问题和数据泄露等风险。需要 `origin` 远程；如果审查因 `ambiguous argument` 错误而失败，请参阅[错误参考](https://code.claude.com/docs/zh-CN/errors.md#security-review-fails-without-origin-head) |
| `/setup-bedrock` | 通过交互式向导配置 [Amazon Bedrock](https://code.claude.com/docs/zh-CN/amazon-bedrock.md) 身份验证、区域和模型固定。在设置 `CLAUDE_CODE_USE_BEDROCK=1` 之前[在命令菜单中隐藏](#how-the-command-menu-matches-what-you-type)；需要完整输入。首次使用 Amazon Bedrock 的用户也可以从登录屏幕访问此向导 |
| `/setup-vertex` | 通过交互式向导配置 [Google Cloud's Agent Platform](https://code.claude.com/docs/zh-CN/google-vertex-ai.md) 身份验证、项目、区域和模型固定。在设置 `CLAUDE_CODE_USE_VERTEX=1` 之前[在命令菜单中隐藏](#how-the-command-menu-matches-what-you-type)；需要完整输入。首次使用 Google Cloud's Agent Platform 的用户也可以从登录屏幕访问此向导 |
| `/simplify [target]` | **[Skill](https://code.claude.com/docs/zh-CN/skills.md#bundled-skills)。** 审查已更改的代码以寻找清理机会并应用修复。四个审查 [Agent](https://code.claude.com/docs/zh-CN/sub-agents.md) 并行运行，涵盖现有辅助函数的复用、简化、效率，以及更改是否处于合适的抽象层级。该审查不查找正确性 bug。使用 `/code-review` 查找 bug。传递路径或 PR 引用可审查特定目标 |
| `/skill-doctor` | 显示您的每个 [skill](https://code.claude.com/docs/zh-CN/skills.md) 在上下文中的开销以及使用频率，以便您[找到可以关闭的 skill](https://code.claude.com/docs/zh-CN/skills.md#find-unused-skills)。需要 Claude Code v2.1.252 或更高版本以及[功能标志获取](https://code.claude.com/docs/zh-CN/env-vars.md#features-that-need-feature-flag-fetching) |
| `/skills` | 列出可用的 [skill](https://code.claude.com/docs/zh-CN/skills.md)。输入内容可按名称、描述或来源筛选列表。按 `t` 按 token 数排序，按 `Space` 或 `Enter` [循环切换 skill 对 Claude 和 `/` 菜单的可见性](https://code.claude.com/docs/zh-CN/skills#override-skill-visibility-from-settings)，按 `Esc` 保存并关闭。您无法循环切换插件 skill、frontmatter 设置了 `disable-model-invocation: true` 的 skill，或在托管设置或 `--settings` 标志中具有 `skillOverrides` 条目的 skill |
| `/slides [brief]` | **[Skill](https://code.claude.com/docs/zh-CN/skills.md#bundled-skills)。** 根据您的简介创建一个新的演示文稿，作为 Claude Slides [Artifact](https://code.claude.com/docs/zh-CN/artifacts.md#make-a-slide-deck)，例如 `/slides a quarterly review of the platform team`。需要 Claude Code v2.1.265 或更高版本、[Artifact 可用](https://code.claude.com/docs/zh-CN/artifacts.md#availability)的会话，以及 [Slides 模板可用](https://code.claude.com/docs/zh-CN/artifacts.md#start-from-a-slides-design-or-docs-template)的账户；否则该命令不会出现。可在 Anthropic API 上使用。在 Amazon Bedrock、Google Cloud's Agent Platform、Microsoft Foundry 和 Claude Platform on AWS 上，Artifact 不可用，因此该命令在这些平台上不可用 |
| `/stats` | `/usage` 的别名。在 Stats 选项卡上打开 |
| `/status` | 在 Status 选项卡上打开设置界面，显示版本、模型、账户和连接状态。在[后台会话](https://code.claude.com/docs/zh-CN/agent-view.md)中，`Session kind` 行会显示 `background job · attached` 或 `background job · unattended`（取决于是否附加了终端），在其他任何会话中显示 `interactive`。在 v2.1.221 之前，`/status` 不显示此行。在 Claude 回复时也可使用 |
| `/statusline` | 配置 Claude Code 的[状态栏](https://code.claude.com/docs/zh-CN/statusline.md)。描述您想要的内容，或不带参数运行以根据您的 shell 提示符自动配置 |
| `/stickers` | 订购 Claude Code 贴纸 |
| `/stop` | 停止您已附加到的[后台会话](https://code.claude.com/docs/zh-CN/agent-view.md)，或您以[窥视回复](https://code.claude.com/docs/zh-CN/agent-view.md#peek-and-reply)方式发送此命令的目标会话；会话记录和任何 worktree 都会保留。要分离而不停止，请使用 `/exit` 或按 `←` |
| `/subtask <task>` | 生成一个[分叉子代理](https://code.claude.com/docs/zh-CN/sub-agents.md#fork-the-current-conversation)：一个继承完整对话的后台子代理，在您继续工作的同时处理该任务。完成后，其结果会返回到此对话。要改为将对话复制到单独的后台会话中，请使用 `/fork`。当[关闭 Agent 视图](https://code.claude.com/docs/zh-CN/agent-view.md#turn-off-agent-view)时，`/subtask` 不可用，`/fork` 会改为生成分叉子代理 |
| `/tasks` | 查看和管理当前会话中的后台工作，包括已完成的子代理。也可作为 `/bashes` 使用 |
| `/team-onboarding` | 根据您的 Claude Code 使用历史生成团队入门指南。Claude 会分析您过去 30 天的会话、命令和 MCP 服务器使用情况，并生成一份 markdown 指南，队友可以将其作为第一条消息粘贴以快速完成设置。对于 Team 和 Enterprise 套餐的 claude.ai 订阅者，还会返回一个分享链接，队友可以直接在 Claude Code 中打开 |
| `/teleport` | 将[云端会话](https://code.claude.com/docs/zh-CN/claude-code-on-the-web.md#from-cloud-to-terminal)拉取到此终端。打开选择器，然后获取分支和对话。也可作为 `/tp` 使用。需要 claude.ai 订阅 |
| `/terminal-setup` | 在 VS Code、Cursor、Devin Desktop、Alacritty 或 Zed 中[安装用于换行的 Shift+Enter 快捷键](https://code.claude.com/docs/zh-CN/terminal-config.md#enter-multiline-prompts)。在 Apple Terminal 中，则改为[启用 Option+Enter 换行并关闭提示音](https://code.claude.com/docs/zh-CN/terminal-config.md#enable-option-key-shortcuts-on-macos)。在 iTerm2 中，[开启剪贴板访问，使 `/copy` 能够正常工作](https://code.claude.com/docs/zh-CN/terminal-config#enable-option-key-shortcuts-on-macos) |
| `/theme` | 更改颜色主题。包括与终端浅色或深色背景匹配的 `auto` 选项、浅色和深色变体、色盲友好（daltonized）主题、使用终端调色板的 ANSI 主题，以及来自 `~/.claude/themes/` 或插件的任何[自定义主题](https://code.claude.com/docs/zh-CN/terminal-config.md#create-a-custom-theme)。选择 **New custom theme…** 可创建一个主题 |
| `/tui [default\|fullscreen]` | 设置终端 UI 渲染器，并在保持对话完整的情况下重新启动到该渲染器。`fullscreen` 会启用[无闪烁备用屏幕渲染器](https://code.claude.com/docs/zh-CN/fullscreen.md)。不带参数时，输出当前活动的渲染器 |
| `/ultraplan <prompt>` | 已移除。请改用[计划模式](https://code.claude.com/docs/zh-CN/permission-modes.md#analyze-before-you-edit-with-plan-mode)。以前会将规划任务发送到[云端会话](https://code.claude.com/docs/zh-CN/claude-code-on-the-web.md)，以便在浏览器中审阅 |
| `/ultrareview [PR or branch]` | 使用 [ultrareview](https://code.claude.com/docs/zh-CN/ultrareview.md) 在云端沙箱中运行深度多 Agent 代码审查。传递 PR 引用可审查该 Pull Request，或传递基准分支或提交以更改比较基准。首选调用方式是 `/code-review ultra`，`/ultrareview` 是其别名。Pro、Max 和 Ultra 包含 3 次免费运行，之后需要[使用额度](https://support.claude.com/en/articles/12429409-extra-usage-for-paid-claude-plans) |
| `/update-config [request]` | **[Skill](https://code.claude.com/docs/zh-CN/skills.md#bundled-skills)。** 描述一项设置更改，例如允许某个命令、设置环境变量或添加 [hook](https://code.claude.com/docs/zh-CN/hooks.md)，Claude 会编辑相应的 [`settings.json`](https://code.claude.com/docs/zh-CN/settings) 文件。对于主题和模型等选项，请改用 `/config` |
| `/upgrade` | 在浏览器中打开升级页面，以切换到更高的套餐级别。当浏览器无法打开时，该命令会显示登录提示，但不会输出 URL |
| `/usage` | 显示会话开销、套餐用量限制和活动统计信息。在 Pro、Max、Ultra、Team 或 Enterprise 套餐上，包括[计入套餐限制的内容明细](https://code.claude.com/docs/zh-CN/costs.md#plan-usage-breakdown)。`/cost` 和 `/stats` 是其别名 |
| `/usage-credits` | 在达到限制时配置使用额度，或向管理员申请使用额度。在浏览器中打开您的[使用额度计费设置](https://code.claude.com/docs/zh-CN/costs.md#add-usage-credits-to-your-subscription)；但没有计费访问权限的 Team 和 Enterprise 成员会在对话框中确认该请求会通知其管理员后，改为从 CLI 向管理员发送使用额度请求。当无法打开浏览器访问计费页面时（例如通过 SSH），该命令会改为输出要访问的 URL；这需要 Claude Code v2.1.205 或更高版本，更早版本在这种情况下不显示任何内容。以前为 `/extra-usage` |
| `/verify` | **[Skill](https://code.claude.com/docs/zh-CN/skills.md#bundled-skills)。** 通过构建、运行您项目的应用并观察结果来确认代码更改是否按预期工作，而不是依赖测试或类型检查。请参阅[运行并验证您的应用](https://code.claude.com/docs/zh-CN/skills.md#run-and-verify-your-app) |
| `/vim` | 已在 v2.1.92 中移除。要在 Vim 和 Normal 编辑模式之间切换，请使用 `/config` → Editor mode |
| `/voice [hold\|tap\|off]` | 切换[语音听写](https://code.claude.com/docs/zh-CN/voice-dictation.md)，或以特定模式启用它。需要 Claude.ai 账户 |
| `/web-setup` | 使用本地 `gh` CLI 凭据为[云端会话](https://code.claude.com/docs/zh-CN/web-quickstart.md#connect-from-your-terminal)连接您的 GitHub 账户 |
| `/workflow-authoring` | **[Skill](https://code.claude.com/docs/zh-CN/skills.md#bundled-skills)。** 加载编写[动态工作流](https://code.claude.com/docs/zh-CN/workflows.md)脚本的参考：脚本 API、恢复行为、质量模式和完整示例。Claude 通常会在编写脚本之前自行加载它；在[手动编辑已保存的脚本](https://code.claude.com/docs/zh-CN/workflows.md#edit-a-saved-script)之前，请自行运行它。在启用动态工作流时可用，需要 Claude Code v2.1.248 或更高版本 |
| `/workflows` | 打开[工作流](https://code.claude.com/docs/zh-CN/workflows.md#watch-the-run)进度视图，以监视、暂停、恢复或保存正在运行和已完成的工作流 |

## 命令菜单如何匹配你输入的内容

Claude Code 在你输入时过滤 `/` 菜单。下面的每个要点涵盖了你在过滤时可能注意到的一件事：

* **高亮显示**：Claude Code 仅当 `/` 后面的字母与命令的名称或别名匹配时才高亮显示顶部建议，匹配可以从名称的开头或名称内的某个单词开始，忽略 `:`、`_` 和 `-` 分隔符。输入 `/adddir` 会高亮显示 `/add-dir`，输入 `/new` 会通过其别名高亮显示 `/clear`。按 `Enter` 运行高亮显示的建议。这些高亮显示规则需要 Claude Code v2.1.236 或更高版本。
* **打字错误后**：Claude Code 不高亮显示任何内容。接近的匹配项保持列出，你可以用 `Tab` 或箭头键选择一个，但 `Enter` 会按原样提交你的文本并报告[未知命令](https://code.claude.com/docs/zh-CN/errors.md#unknown-command)。
* **你无法使用的命令**：Claude Code 将它们排除在菜单之外。当没有任何内容匹配时，Claude Code 显示 `No commands match "/name"`。大多数不可用的命令在你提交时会返回[未知命令](https://code.claude.com/docs/zh-CN/errors.md#unknown-command)；少数几个，例如[`/schedule` 在 Console API 密钥上](https://code.claude.com/docs/zh-CN/routines#schedule-returns-unknown-command)，会改为返回自己的可用性消息。当你的组织的策略禁用某些命令时，这些命令也会返回自己的消息。
* **隐藏命令**：Claude Code 故意将一些可用命令（例如 `/heapdump`）排除在菜单之外。部分名称永远不会将隐藏命令带入菜单：如果部分匹配不到任何可见内容，Claude Code 会显示相同的不匹配消息。Claude Code 仅在你输入完整名称后才列出该命令，提交完整名称会运行它。

## MCP prompts

MCP 服务器可以公开显示为命令的提示。有关详细信息，请参阅 [MCP prompts](https://code.claude.com/docs/zh-CN/mcp.md#use-mcp-prompts-as-commands)。

## 另请参阅

* [Skills](https://code.claude.com/docs/zh-CN/skills.md)：创建您自己的命令
* [Interactive mode](https://code.claude.com/docs/zh-CN/interactive-mode.md)：快捷键、Vim 模式和命令历史记录
* [CLI reference](https://code.claude.com/docs/zh-CN/cli-reference.md)：启动时标志
