> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# 使用动态工作流大规模编排子代理

> 动态工作流从 Claude 编写的脚本中编排许多子代理，您可以重新运行。用于代码库审计、大型迁移和交叉检查研究。

> [!NOTE]
>
> 动态工作流在所有付费计划上可用，具有 Anthropic API 访问权限，以及在 Amazon Bedrock、Google Cloud 的 Agent Platform 和 Microsoft Foundry 上可用。在 Pro 上，从 `/config` 中的 Dynamic workflows 行启用它们。

动态工作流是一个 JavaScript 脚本，可大规模编排许多[子代理](https://code.claude.com/docs/zh-CN/sub-agents.md)。Claude 为您描述的任务编写脚本，运行时在后台执行它，同时您的会话保持响应。

当任务需要比一个对话能协调的更多代理时，或当您想将编排编纂为可以读取和重新运行的脚本时，请使用工作流。示例包括代码库范围的错误扫描、500 文件迁移、需要相互交叉检查来源的研究问题，以及在提交一个之前值得从多个独立角度起草的困难计划。

## 何时使用工作流

[子代理](https://code.claude.com/docs/zh-CN/sub-agents.md)、[skills](https://code.claude.com/docs/zh-CN/skills.md)、[agent teams](https://code.claude.com/docs/zh-CN/agent-teams.md) 和工作流都可以运行多步骤任务。区别在于谁掌握计划：

| | 子代理 | Skills | Agent teams | 工作流 |
| :- | :- | :- | :- | :- |
| 它是什么 | Claude 生成的工作者 | Claude 遵循的指令 | 监督对等会话的主导代理 | 运行时执行的脚本 |
| 谁决定接下来运行什么 | Claude，逐轮 | Claude，遵循提示 | 主导代理，逐轮 | 脚本 |
| 中间结果在哪里 | Claude 的上下文窗口 | Claude 的上下文窗口 | 共享任务列表 | 脚本变量 |
| 什么是可重复的 | 工作者定义 | 指令 | 团队定义 | 编排本身 |
| 规模 | 每轮几个委派任务 | 与子代理相同 | 少数几个长期运行的对等体 | 每次运行数十到数百个代理 |
| 中断 | 重启轮次 | 重启轮次 | 队友继续运行 | 在同一会话中可恢复 |

工作流将计划移入代码。使用子代理、skills 和 agent teams，Claude 是编排者：它逐轮决定接下来生成或分配什么，每个结果都进入上下文窗口。工作流脚本持有循环、分支和中间结果本身，所以 Claude 的上下文只持有最终答案。

将计划移入代码也让工作流应用可重复的质量模式，而不仅仅是运行更多代理：它可以让独立代理在报告之前对彼此的发现进行对抗性审查，或从多个角度起草计划并相互权衡，所以您获得比单次通过更可信的结果。

## 运行捆绑工作流

查看工作流运行的最快方式是运行 `/deep-research`，这是 Claude Code 包含的[内置工作流](#bundled-workflows)，用于跨许多来源调查问题。您将看到代理在后台通过一组阶段工作，同时您的会话保持空闲，最后获得一份报告而不是逐轮记录。

**1. 运行工作流**

使用您想要调查的问题运行 `/deep-research`。它在多个角度上扇出网络搜索，获取并交叉检查它找到的来源，并综合一份引用的报告。

```text wrap theme={null}
/deep-research What changed in the Node.js permission model between v20 and v22?
```

**2. 允许工作流**

Claude Code 询问是否允许工作流。选择**是**继续。确切的提示取决于您的权限模式。有关每种模式的选项，请参阅[在运行前批准计划](#approve-the-plan-before-it-runs)。

**3. 观看进度**

运行在后台启动。运行 `/workflows` 打开其进度视图：

```text wrap theme={null}
/workflows
```

如果 `/workflows` 显示的是运行列表，请选择您刚启动的运行并按 Enter。该视图显示每个阶段及其 Agent 计数。深入任何阶段以查看其 Agent 及每个 Agent 发现的内容。有关完整的控制集，请参阅[观看运行](#watch-the-run)。

您也可以从输入框下方的任务面板观看：运行进行时会出现一行进度摘要。按向下箭头聚焦它，然后按 Enter 展开。

**4. 阅读报告**

运行完成后，报告进入您的会话。它引用每个声明来自的来源，未通过交叉检查的声明已被过滤掉。

当验证代理无法检查声明时，例如在速率限制或 API 错误之后，报告将该声明列为未验证，而不是计为被驳回。

要为您自己的任务运行工作流，[让 Claude 编写一个](#have-claude-write-a-workflow)，一旦运行完成您想要的操作，您可以[保存它](#save-the-workflow-for-reuse)作为您自己的命令。

### 捆绑工作流

Claude Code 包含 `/deep-research` 作为内置工作流：

| 命令 | 它做什么 |
| :- | :- |
| `/deep-research <question>` | 在多个角度上扇出网络搜索问题，获取并交叉检查它找到的来源，对每个声明投票，并返回一份引用的报告，其中未通过交叉检查的声明已被过滤掉。需要[WebSearch 工具](https://code.claude.com/docs/zh-CN/tools-reference.md#websearch-tool-behavior)可用 |

`/deep-research` 仅在您调用它时运行。

[您自己保存的工作流](#save-the-workflow-for-reuse)以相同方式成为命令，并在 `/` 自动完成中与捆绑的工作流一起出现。

### 观看运行

工作流在后台运行，所以会话在 Agent 工作时保持响应。随时运行 `/workflows` 列出运行中和已完成的工作流，然后选择一个打开其进度视图。当会话只有一个运行时，`/workflows` 会跳过列表并直接打开该运行。要从列表中停止运行中的工作流而不打开它，请选择它并按 `x`。

进度视图显示每个阶段及其 Agent 计数。页脚列出每个操作的键：

| 键 | 操作 |
| :- | :- |
| `↑` / `↓` | 选择一个阶段或代理 |
| `Enter` 或 `→` | 深入选定的阶段，然后进入代理的详情。在详情中，`Enter` 展开或折叠它 |
| `Esc` 或 `←` | 返回一个级别 |
| `j` / `k` | 当代理详情溢出时在其中滚动 |
| `f` | 按状态过滤选定阶段中的代理列表。再次按下以循环 |
| `p` | 暂停或恢复运行 |
| `x` | 停止选定的代理，或当焦点在运行上时停止整个工作流 |
| `r` | 重启选定的运行中代理 |
| `s` | [保存](#save-the-workflow-for-reuse)运行的脚本作为命令 |

代理详情列出代理的提示、其最近的工具调用和其结果。每个调用显示其状态，例如仍在运行或失败。当代理保持自己的任务列表时，详情显示它，以及每个任务的状态。

按 `Enter` 展开详情。提示和结果然后完整显示，每个列出的调用显示其输入和其结果的开始。

## 让 Claude 编写工作流

您可以通过两种方式让 Claude 为您的任务编写工作流：

* 在提示词中[请求工作流](#ask-for-a-workflow-in-your-prompt)，可以用您自己的话表述，也可以包含关键字 `ultracode`，Claude 会为该任务编写一个工作流。
* [使用 ultracode 让 Claude 决定](#let-claude-decide-with-ultracode)：设置 `/effort ultracode`，Claude 会为会话中的每个实质性任务规划工作流。

您也可以运行已存在的工作流命令：例如 `/deep-research` 这样的[内置工作流](#bundled-workflows)，或您[已保存](#save-the-workflow-for-reuse)的工作流。

### 在提示词中请求工作流

要在不更改会话 effort 级别的情况下以工作流方式运行单个任务，请在提示词中包含关键字 `ultracode`。用您自己的话请求也同样有效，例如"use a workflow"或"run a workflow"：Claude 会将直接请求视为同样的选择加入。

```text wrap theme={null}
ultracode: audit every API endpoint under src/routes/ for missing auth checks
```

Claude Code 会在您的输入中高亮显示该关键字，Claude 会为该任务编写工作流脚本，而不是逐轮次地处理任务。该关键字只决定 Claude 如何组织工作：Agent 的工具调用与会话中的任何其他工具调用一样，接受相同的权限检查和[沙箱隔离](https://code.claude.com/docs/zh-CN/sandboxing.md)。

如果运行结果符合您的预期，您可以在之后[将其保存为命令](#save-the-workflow-for-reuse)。如果您已经用其他方式构建了编排器，例如一个存放子代理提示词的文件夹，或一个分发工作的 skill，您可以让 Claude 参考它，并请求编写一个实现相同功能的工作流。

#### 取消或关闭关键字

如果您并非有意启动工作流，请在 macOS 上按 `Option+W`，在 Windows 和 Linux 上按 `Alt+W`，以取消本次提示词的高亮；或者在光标紧跟高亮关键字时按退格键。要完全阻止该关键字触发，请在 `/config` 中关闭 Ultracode keyword trigger。

#### 关键字的生效范围

只有在您亲自输入的提示词中，该关键字才会作为选择加入：包括交互式输入框、IDE 扩展面板、[Remote Control](https://code.claude.com/docs/zh-CN/remote-control.md) 客户端，或将您的键盘输入的 [`origin`](https://code.claude.com/docs/zh-CN/agent-sdk/typescript#sdkmessageorigin) 标记为 `{ kind: "human" }` 的 Agent SDK 应用程序。当它通过其他方式到达会话时，不会启动工作流：

* 通过 `-p` 传入的提示词
* Agent SDK 应用程序发送但未标记为人工输入的提示词
* 定时任务的提示词
* 转发到对话中的 webhook 负载或 Pull Request 评论

### 使用 ultracode 让 Claude 决定

Ultracode 是 Claude Code 的一项设置，可为会话启用自动工作流编排，无论会话运行在哪个 [effort 级别](https://code.claude.com/docs/zh-CN/model-config.md#adjust-effort-level)。启用后，Claude 会为每个实质性任务规划工作流，而不是等待您提出请求。在 Claude Code 输入框中启用它：

```text wrap theme={null}
/effort ultracode
```

要在启动会话时就启用 ultracode，请使用 `claude --effort ultracode` 启动，这还会将 effort 级别设置为 `xhigh`。

要从 `/effort` 滑块启用，请按 `Tab` 切换 **Ultracode** 开关，然后按 `Enter` 应用。[调整 effort 级别](https://code.claude.com/docs/zh-CN/model-config.md#adjust-effort-level)列出了启用 ultracode 的各种途径。

启用 ultracode 后，Claude 会决定任务何时需要工作流。单个请求可能会变成连续的多个工作流：一个用于理解代码，一个用于进行更改，一个用于验证更改。这适用于会话中的每个任务，因此每个请求都会比不使用工作流的相同请求消耗更多 token、耗时更长。在订阅套餐中，这些 token 会计入您的用量限制，因此启用 ultracode 的会话会比关闭时完成相同工作更早达到会话或每周限制。

启用 ultracode 本身就意味着您已选择加入大型运行，因此启用期间以下检查不适用：

* 工作流运行时不会出现 [`Large workflow` 警告](#cost)
* 对于 Claude 使用 Agent 工具生成的子代理，不强制执行会话的[并发子代理限制](https://code.claude.com/docs/zh-CN/sub-agents.md#concurrent-subagent-limit)
* 在自动权限模式下，不会要求您[批准首次工作流启动](#approve-the-plan-before-it-runs)

`/effort ultracode` 仅在当前会话中有效；要让每个会话启动时都启用它，请设置 [`ultracode`](https://code.claude.com/docs/zh-CN/settings-reference#ultracode) 设置项。回到日常工作时，使用 `/effort ultracode off` 将其关闭。只有[在 ultracode 可用时](https://code.claude.com/docs/zh-CN/model-config.md#when-ultracode-is-available)，`/effort` 滑块才会提供该开关。在 v2.1.284 之前，`/effort ultracode` 会将会话设置为 `xhigh`，而 `/effort ultracode off` 会失败并显示 `Invalid argument`。

### 在运行前批准计划

在 CLI 中，每次运行的确认提示会显示计划的阶段以及以下选项：

* **Yes, run it**：开始运行
* **Yes, and don't ask again for `<name>` in `<path>`**：开始运行，并且此后在此项目中对该工作流跳过此提示。当您按名称运行内置、已保存或插件工作流时，Claude Code 会提供此选项；对于 Claude 为当前任务编写的脚本则不提供。
* **View raw script**：在决定之前阅读脚本
* **No**：取消

`Ctrl+G` 会在您的编辑器中打开脚本。选中 **Yes, run it** 或 **No** 时，按 `Tab` 可为您的回答[添加评论](https://code.claude.com/docs/zh-CN/permissions.md#add-a-comment-when-you-answer-a-permission-prompt)。

是否会看到此提示取决于您的[权限模式](https://code.claude.com/docs/zh-CN/permission-modes.md)：

| 权限模式 | 何时提示 |
| :- | :- |
| 自动 | 仅首次启动。任何 **Yes** 都会在您的用户设置中记录同意，之后的启动不再提示。启用 ultracode 时完全跳过 |
| 手动、接受编辑 | 每次运行，除非您已在此项目中为该工作流选择了 **Yes, and don't ask again** |
| 绕过权限 | Claude Code 不会提示您。运行立即开始 |
| `claude -p`、Agent SDK | Claude Code 不会提示您 |

在 `claude -p` 和 Agent SDK 中，Claude Code 从不显示此提示。它会对 Workflow 工具调用执行与会话其余部分相同的[权限评估](https://code.claude.com/docs/zh-CN/agent-sdk/permissions.md#how-permissions-are-evaluated)，因此拒绝规则、询问规则和 `dontAsk` 模式对启动的作用与对每个工具调用的作用相同。要允许工作流在这些运行中启动，请使用以下方式之一：

* **权限规则**：将 `Workflow` 添加到允许规则中以批准所有工作流，或添加 `Workflow(<name>)` 以按名称批准某个已保存的工作流。
* **自动权限模式**：在[自动模式](https://code.claude.com/docs/zh-CN/permission-modes.md#eliminate-prompts-with-auto-mode)下运行，分类器会审查该调用并可以批准它。
* **绕过权限模式**：在绕过权限模式下运行。
* **hook**：从 [`PreToolUse`](https://code.claude.com/docs/zh-CN/hooks#pretooluse) hook 为该调用返回 `"allow"`。在[计划模式](https://code.claude.com/docs/zh-CN/permission-modes.md#analyze-before-you-edit-with-plan-mode)下，仅凭 hook 的批准工作流不会启动，因此请改用权限规则或您的宿主程序的批准。
* **您的宿主程序**：由 [`--permission-prompt-tool`](https://code.claude.com/docs/zh-CN/cli-reference#cli-flags)，或在 Agent SDK 中由 [`canUseTool`](https://code.claude.com/docs/zh-CN/agent-sdk/permissions) 回调批准。

在桌面应用中，批准卡片会显示工作流名称、阶段列表和 token 用量提醒，并提供 **Once**、**Always** 和 **Deny** 操作。进度视图显示在 Background tasks 侧边窗格中。

工作流生成的子代理使用您的[权限规则](https://code.claude.com/docs/zh-CN/settings-reference.md#permission-settings)，Claude Code 会按照[子代理在哪种权限模式下运行](https://code.claude.com/docs/zh-CN/sub-agents.md#permission-modes)中的规则为其选择权限模式。为避免长时间运行中出现提示，请在开始前将 Agent 所需的工具添加到允许规则中。

### 保存工作流以便重用

当 Claude 为您会重复执行的任务编写工作流时，您可以将该次运行的脚本保存为命令。这样，像在每个分支上运行的审查这类流程，每次都会执行相同的编排。

运行 `/workflows`，选择要保留的运行，然后按 `s`。在保存对话框中，Tab 可在两个保存位置之间切换：

* 项目中的 `.claude/workflows/`：与所有克隆该仓库的人共享
* 主目录中的 `~/.claude/workflows/`：在每个项目中可用，仅您可见。如果您设置了 [`CLAUDE_CONFIG_DIR`](https://code.claude.com/docs/zh-CN/env-vars)，此位置为该路径下的 `workflows/` 目录。

保存对话框会显示个人位置的解析后路径。按 Enter 保存。无论保存在哪个位置，该工作流在之后的会话中都以 `/<name>` 运行。

Claude Code 在写入前会检查保存位置是否存在符号链接，如果存在，会显示错误而不是通过符号链接写入：

* 项目位置：如果 `.claude`、`.claude/workflows` 或目标文件是符号链接，Claude Code 会拒绝写入。
* 个人位置：仅当目标文件本身是符号链接时 Claude Code 才会拒绝，因此由 dotfiles 工具管理的 `~/.claude` 目录仍然可以正常使用。

在包含多个 `.claude/` 目录的 monorepo 中，您可以将工作流放在其所适用的包旁边。保存到项目位置时，会写入工作目录与仓库根目录之间已存在的最近的 `.claude/workflows/` 目录；如果尚不存在，则写入仓库根目录。项目工作流也会从该路径上的每个 `.claude/workflows/` 加载，当多个目录定义了同名工作流时，Claude Code 会运行离工作目录最近的那个。

如果项目工作流和个人工作流同名，则运行项目工作流。

### 通过插件分发工作流

要在团队或仓库之间共享工作流，请将其包含在[插件](https://code.claude.com/docs/zh-CN/plugins/overview.md)中。将脚本放在插件根目录的 `workflows/` 目录中，或通过 [`workflows` 清单字段](https://code.claude.com/docs/zh-CN/plugins/manifest-reference#fields)指向其他位置。

插件工作流以插件名称作为命名空间。名为 `acme-tools` 的插件中，如果某个脚本的 `meta.name` 为 `release-audit`，它将以 `/acme-tools:release-audit` 运行。

### 向已保存的工作流传递输入

已保存的工作流可以通过 `args` 参数接收输入。脚本会将其作为名为 `args` 的全局变量读取。您可以借此在调用时提供研究问题、目标路径列表或配置对象，而无需为每次运行编辑脚本。

以下提示词使用一组 issue 编号运行已保存的工作流：

```text wrap theme={null}
Run /triage-issues on issues 1024, 1025, and 1030
```

Claude 会以结构化数据的形式传递该列表，因此脚本可以直接对 `args` 调用数组和对象方法，而无需先解析。如果省略 `args`，该全局变量在脚本中为 `undefined`。

## 工作流提示示例

工作流最适合当任务大于一个代理可以在上下文中容纳的情况，或当相同的步骤需要在许多项目中运行时。下面的提示显示常见的形状。每一个都要求 Claude 为该任务编写并运行工作流；您不自己编写脚本。

### 审计许多文件以查找相同问题

扇出每个文件一个代理，然后收集并验证发现。

```text wrap theme={null}
use a workflow to audit every route handler under src/routes/ for missing authentication checks, and adversarially verify each finding before reporting it
```

### 保持修复直到检查通过

运行检查器，修复失败的内容，然后重复直到通过或停止取得进展。

```text wrap theme={null}
use a workflow to run npx tsc --noEmit and keep fixing the reported errors until the type check passes or two rounds in a row make no progress
```

### 并行迁移许多文件

发现要迁移的文件，在隔离的副本中转换每个文件，以便编辑不会冲突，并验证每个结果。

```text wrap theme={null}
use a workflow to migrate every component under src/components/ from JavaScript to TypeScript, working on each file in its own isolated copy
```

### 审查每个更改的文件并写一份摘要

为每个文件运行审查者，然后将所有发现交给一个代理，该代理对它们进行排名和去重。

```text wrap theme={null}
use a workflow to review every file changed in this PR for correctness issues, then merge the per-file findings into one ranked summary
```

### 跨许多来源研究一个主题

在更改日志、问题和文档中扇出读者，然后综合。捆绑的 `/deep-research` 工作流执行此操作；您也可以描述一个更狭窄的版本。

```text wrap theme={null}
use a workflow to research how our three competitors handle rate limiting: read their public docs and recent changelog entries in parallel, then compare the approaches
```

### 查找问题直到列表停止增长

保持分轮搜索并在新轮次找不到任何新内容时停止。

```text wrap theme={null}
use a workflow to find flaky tests in this repo: run the suite repeatedly, record which tests fail intermittently, and stop once two rounds in a row find nothing new
```

### 保存的脚本看起来像什么

当您[保存工作流](#save-the-workflow-for-reuse)时，`.claude/workflows/` 中的文件包含一个 `meta` 块，后跟编排子代理的脚本体。您通常不需要编辑它，但这是一个小的形状，所以您可以识别 Claude 生成的内容：

```javascript theme={null}
export const meta = {
  name: 'audit-routes',
  description: 'Audit every route handler for missing auth checks',
}

const found = await agent('List every .ts file under src/routes/.', {
  schema: { type: 'object', required: ['files'], properties: { files: { type: 'array', items: { type: 'string' } } } },
})

const audits = await pipeline(found.files, file =>
  agent(`Audit ${file} for missing authentication checks.`, { label: file }),
)

return audits.filter(Boolean)
```

主体是带有顶级 `await` 的纯 JavaScript。`agent()` 生成一个子代理，`pipeline()` 为列表中的每个项目运行一个，`parallel()` 同时运行一组代理任务并等待所有任务完成。

如果您在运行中途停止 `agent()` 调用或它遇到不可恢复的 API 错误，则 `agent()` 调用解析为 `null`。`pipeline()` 在结果数组中保留每个 `null`，这就是为什么示例以 `.filter(Boolean)` 结尾以删除这些条目，其中包括[每次尝试都停滞的 Agent](#when-an-agent-stalls-and-restarts) 所占的位置。

在[自动模式](https://code.claude.com/docs/zh-CN/permission-modes.md#eliminate-prompts-with-auto-mode)中，您的脚本传递给 `agent()` 的提示不会计为您的请求，当分类器审查该子代理的操作时，因为 Claude Code 将其标记为脚本计算的文本。

如果您在 `agent()` 调用上传递 `schema`，该子代理将返回与形状匹配的 JSON 而不是散文。Claude Code 在启动子代理之前检查架构：当它可以证明架构自相矛盾时，调用失败并显示一个错误，命名矛盾，子代理永远不会启动。它可以证明的一个矛盾是 `additionalProperties: false` 排除的 `required` 键。

如果子代理的输出在五次尝试后仍然无法通过验证，调用将失败并显示一个错误，其中包括最后的验证失败。要更改尝试次数，请设置 [`MAX_STRUCTURED_OUTPUT_RETRIES`](https://code.claude.com/docs/zh-CN/env-vars)。

### 编辑保存的脚本

要更改[您保存的工作流](#save-the-workflow-for-reuse)，编辑其 `.js` 文件或要求 Claude 进行更改。在编辑或要求之前，运行 `/workflow-authoring` [捆绑技能](https://code.claude.com/docs/zh-CN/skills.md#bundled-skills)以加载 Claude 工作的脚本编写参考。该技能需要 Claude Code v2.1.248 或更高版本。

要在当前会话中运行编辑后的版本，运行 [`/reload-skills`](https://code.claude.com/docs/zh-CN/commands#all-commands) 以重新读取工作流目录，然后再次运行 `/<name>`。

Claude Code 在加载和运行脚本时对文件的每个部分应用这些规则：

* **`meta` 块**：将 `export const meta` 保持为第一个语句，并将其保持为带有 `name` 和 `description` 的纯对象字面量。如果它包含除字面值之外的任何内容，例如变量、函数调用或展开，Claude Code 会从 `/` 自动完成中删除 `/<name>`。
* **主体**：除了 `agent()`、`pipeline()` 和 `parallel()` 之外，您可以调用 `phase()` 在进度视图中对以下代理进行分组并添加标题，调用 `log()` 在阶段上方显示消息，并读取 [`args`](#pass-input-to-a-saved-workflow) 全局变量。如果主体有语法错误，Claude Code 会在您运行工作流时报告它。
* **`phases`**：如果您在 `meta` 中列出它们，为每个条目提供您传递给 `phase()` 的确切标题。没有条目的 `phase()` 标题会获得自己的进度组。
* **时间戳和随机性**：Claude Code 使脚本内的 `Date.now()`、`Math.random()` 和无参数 `new Date()` 抛出异常，以便[重新启动的运行](#resume-after-a-pause)重复相同的 `agent()` 调用。改为通过 `args` 传入时间戳。

您也可以编辑[单个运行的脚本](#how-a-workflow-runs)而不是保存的副本。[在暂停后恢复](#resume-after-a-pause)涵盖当您重新启动编辑的脚本时哪些代理再次运行。有关 Workflow 工具的输入，请参阅其在 [Agent SDK 参考](https://code.claude.com/docs/zh-CN/agent-sdk/typescript.md#workflow)中的条目。

## 工作流如何运行

工作流运行时在隔离的环境中执行脚本，与您的对话分离。中间结果保留在脚本变量中，而不是进入 Claude 的上下文。

除非 Claude 从脚本文件的路径启动运行，否则 Claude Code 会将该次运行的脚本写入您会话目录下 `~/.claude/projects/` 中的文件。Claude 在运行开始时接收脚本的路径，因此您可以要求它提供该路径。您可以打开该文件来读取 Claude 编写的编排，将其与之前运行的脚本进行对比，或编辑它并要求 Claude 从编辑后的版本重新启动。

Claude 只能从会话已允许读取的脚本文件启动工作流。要运行保存在工作目录外的脚本，请先使用 [`/add-dir`](https://code.claude.com/docs/zh-CN/permissions#working-directories) 或 [Read allow rule](https://code.claude.com/docs/zh-CN/permissions.md#read-and-edit) 添加其目录。

运行时在运行进行时跟踪每个代理的结果，这正是使运行在同一会话内 [可恢复](#resume-after-a-pause) 的原因。

### 扇出中的 prompt caching

同一运行中的代理可以读取彼此的 [prompt cache](https://code.claude.com/docs/zh-CN/prompt-caching.md#subagents-and-the-cache)。两个使用相同模型、努力级别、代理类型、工具、输出架构和工作目录运行的代理会构建相同的工具和系统提示前缀，因此在匹配的兄弟代理响应开始后启动的代理会在其第一个请求中读取该兄弟代理的缓存。

工作流代理的请求落在主对话的 [cache TTL bucket](https://code.claude.com/docs/zh-CN/prompt-caching.md#which-ttl-each-request-gets) 之外，因此其缓存默认保持五分钟，包括在 Claude 订阅上。要将其保持一小时，请将 [`subagentPromptCacheTtl`](https://code.claude.com/docs/zh-CN/settings-reference#subagentpromptcachettl) 设置为 `1h`。API 以更高的费率计费 1 小时缓存写入。

当扇出同时启动多个匹配的代理时，Claude Code 会保留除第一个之外的所有代理，直到第一个代理的响应开始，然后一起释放保留的代理，以便它们的第一个请求读取共享前缀，而不是每个都未缓存地处理它。Claude Code 将保留限制在 [`CLAUDE_CODE_WORKFLOW_PREFIX_STAGGER_MS`](https://code.claude.com/docs/zh-CN/env-vars) 毫秒，默认为 `5000`。将其设置为 `0` 以禁用保留。

### 行为和限制

运行时应用以下约束：

| 约束 | 原因 |
| :- | :- |
| 无中途用户输入 | 运行仅在代理权限提示和 [使用限制等待](#when-a-run-hits-your-usage-limit) 时暂停。对于阶段之间的签署，将每个阶段作为其自己的工作流运行 |
| 工作流本身无直接文件系统或 shell 访问 | 代理读取、写入和运行命令。脚本协调代理 |
| 无模块加载：包含 `import()` 的脚本在运行开始前失败 | 脚本体是纯 JavaScript。将需要库的工作放在代理的任务中 |
| 最多 16 个并发代理，当 Claude Code 可用的 CPU 较少时更少，包括在 CPU 受限的容器内。要更改限制，请将 [`CLAUDE_CODE_WORKFLOW_MAX_CONCURRENT_AGENTS`](https://code.claude.com/docs/zh-CN/env-vars#variables) 设置为 1 到 256 之间的值，这需要 Claude Code v2.1.269 或更高版本 | 限制本地资源使用 |
| 在扇出中，共享第一个代理的 prompt-cache 前缀的代理最多在其后 5 秒启动，默认情况下 | 除第一个外的所有代理都读取 [第一个代理缓存的前缀](#prompt-caching-in-a-fan-out)，而不是每个都未缓存地处理它 |
| 单个 `parallel()` 或 `pipeline()` 调用中最多 4,096 个项目：运行时以错误拒绝更长的列表 | 无声上限会在不告知脚本的情况下丢弃部分工作负载 |
| 每次运行总共 1,000 个代理 | 防止失控循环 |

## 管理运行

运行开始后，您可以在 `/workflows` 视图中管理它，也可以展开输入框下方任务面板中该运行的进度行进行管理。

停止运行后，只要其任何 Agent 的进程仍在运行，该运行就会一直保留在任务面板中。如果您再次停止它，Claude Code 会重新向这些进程发送信号。

### 暂停后恢复

要恢复已暂停的运行，请在 `/workflows` 中选中它并按 `p`。对于您已停止的运行，请让 Claude 使用同一脚本重新启动该工作流。如果已停止运行中的 Agent 尚未退出，Claude Code 会拒绝重新启动，直到它们退出为止，以免这些 Agent 的第二个副本与它们同时运行。

#### 哪些 Agent 会再次运行

Claude Code 按 Agent 的启动顺序重放运行，每个 Agent 要么返回其保存的结果，要么再次运行：

* **已完成**：返回其保存的结果。第一个提示词与上一次运行不同的 Agent（因为您编辑了脚本，或者前面的某个 Agent 返回了不同的内容）会再次运行，其后的每个 Agent 也会再次运行，即使它们已经完成。

  在 v2.1.305 之前，如果您重新启动一个未经编辑的脚本，而该脚本的 `pipeline()` 项或 `parallel()` 任务各自运行了多个 Agent，已完成的 Agent 也可能再次运行。要让新的运行不再出现这类重复运行，请更新 Claude Code。
* **停止时仍在运行**：从头开始。停止整个运行不会将任何 Agent 计为失败。
* **失败**：再次运行，脚本在其之后启动的每个 Agent 也会再次运行，即使它们已经完成。单独停止某一个 Agent（在 [`/workflows`](#watch-the-run) 中选中它并按 `x`）会被计为失败。

最后一种情况意味着，扇出过程中途的失败会导致已完成的工作重新运行。如果一个脚本按顺序启动 A、B、C 和 D，而 B 失败了，重新启动时会从缓存返回 A，并再次运行 B、C 和 D。

如果您编辑一个原本启动 A、B、C 和 D 的脚本，使其改为启动 A、C、B 和 D，那么第二个启动的 Agent 现在是 C，而上一次运行中是 B。重新启动时会从缓存返回 A，并再次运行 C、B 和 D。

#### 离开会话后恢复

如果您在工作流运行进行期间将会话转入后台或退出 Claude Code，该运行不会自行继续。只有当 Claude 通过 `Workflow` 调用重新启动它时，运行才会继续，这会[根据保存的结果重放运行](#which-agents-run-again)。

* **您将会话转入了后台，或选择了 [`Move to background and exit`](https://code.claude.com/docs/zh-CN/agent-view#from-inside-a-session)**：如果您查看[后台会话](https://code.claude.com/docs/zh-CN/agent-view.md#what-carries-over-when-you-background)时运行尚未继续，请在该会话中让 Claude 重新启动工作流。根据您的[权限模式](#approve-the-plan-before-it-runs)，Claude Code 会先请您批准此次重新启动。如果该会话在 agent view 中显示在 `Needs input` 下，请[连接到该会话](https://code.claude.com/docs/zh-CN/agent-view.md#attach-to-a-session)以回应批准提示。
* **在后台会话中重新启动失败并显示 `nothing to resume`**：请让 Claude 将工作流作为新运行从头开始。如果 Claude 是从 `~/.claude/projects/` 之外的脚本文件路径（例如工作目录中的脚本）启动该运行的，Claude 无法在后台会话中重新启动它。
* **您选择了 `Exit and stop tasks`，或者没有出现 `Move to background and exit` 选项**：运行已随会话一起停止，Claude Code 会将该运行保存的结果保留在 `~/.claude/projects/` 中该会话的目录下。请在 shell 中使用 `claude --resume` 恢复该会话，然后让 Claude 重新启动工作流。在全新的会话中，Claude 没有可以重新启动的先前运行，会从头开始工作流。

在[云端会话](https://code.claude.com/docs/zh-CN/claude-code-on-the-web.md)中，Claude Code 还会将运行的结果与会话的对话历史一起保存，即使会话的 VM 被回收，这些结果也会保留。当您[重新打开此类会话](https://code.claude.com/docs/zh-CN/claude-code-on-the-web.md#environment-expired)并让 Claude 重新启动工作流时，已完成的 Agent 仍会返回其保存的结果。

无论是本地会话还是云端会话，当 Claude 重新启动先前的运行而 Claude Code 完全找不到该运行保存的结果时，重新启动会失败并显示 `nothing to resume` 错误，而不会自行从头开始运行。请让 Claude 将工作流作为新运行从头开始。

### 运行达到用量限制时

当某个 Agent 达到您的 claude.ai [用量限制](https://code.claude.com/docs/zh-CN/interactive-mode.md#wait-for-a-usage-limit-to-reset)时，运行会暂停，而不是让该 Agent 失败：达到限制的 Agent 会等待重置，并且不会启动新的 Agent。限制重置后不久，等待中的 Agent 会再次运行，运行会自行继续。需要 Claude Code v2.1.271 或更高版本；在更早的版本中，受影响的 Agent 会失败。

运行等待期间，任务面板中的进度行和 [`/workflows`](#watch-the-run) 标题会显示限制的重置时间。

只有当以下所有条件都满足时，运行才会暂停；任一条件不满足时，受影响的 Agent 会失败：

* 会话是交互式的，并且已使用 claude.ai 订阅登录。在使用 `claude -p` 的[非交互模式](https://code.claude.com/docs/zh-CN/headless.md)或 [Agent SDK](https://code.claude.com/docs/zh-CN/agent-sdk/overview.md) 中、在[后台会话](https://code.claude.com/docs/zh-CN/agent-view.md)中，或在 [Remote Control](https://code.claude.com/docs/zh-CN/remote-control.md) 或 [agent team](https://code.claude.com/docs/zh-CN/agent-teams.md) 队友会话中，运行不会暂停。
* [`autoContinueAtUsageLimit`](https://code.claude.com/docs/zh-CN/settings-reference#autocontinueatusagelimit) 已开启，这也是让会话本身能够[等待用量限制重置](https://code.claude.com/docs/zh-CN/interactive-mode.md#wait-for-a-usage-limit-to-reset)的同一设置。如果您在等待期间将其关闭，等待会结束，等待中的 Agent 会失败。
* 限制在 24 小时内重置。每周限制的重置时间可能更远。
* 运行尚未等待过两次。当第三次达到限制时，该 Agent 会失败。

### Agent 停滞并重新启动时

如果某个 Agent 的输出停止到达的时间足够长，它会使用相同的提示词从头开始。在 [`/workflows`](#watch-the-run) 中，其名称会加上 `(retry 1)` 后缀，详情中显示 `attempt 2 (stalled)`。重新启动是自动的，您无需进行任何操作。

新的尝试启动时不带有停滞尝试的会话记录。停滞尝试已经更改的文件会保持更改状态，其消耗的 token 也会计入运行的总量。停滞窗口是 Claude Code 在结束尝试之前等待 Agent 输出的时长。Agent 等待自身工具调用或[用量限制重置](#when-a-run-hits-your-usage-limit)所花的时间不计入停滞窗口。

一个 Agent 最多重新启动五次，包括您通过 `r` 要求的任何重新启动。如果第六次尝试也停滞，`agent()` 调用会失败，错误开头会说明原因：

* `agent stalled on all 6 attempts`：每次尝试在整个窗口内都没有输出。如果 Agent 的工作会让它保持沉默这么长时间，请延长窗口
* `agent lost its reply on all 6 attempts`：每次尝试的响应流都变得沉默，Claude Code 放弃了等待。延长停滞窗口没有帮助，因为[流式空闲看门狗](https://code.claude.com/docs/zh-CN/network-config.md#streaming-idle-watchdogs)先结束了响应，而 `CLAUDE_STREAM_IDLE_TIMEOUT_MS` 用于设置该看门狗的超时时间
* `agent abandoned after 6 attempts`：各次尝试以不同方式结束，错误会按顺序列出这些方式

要在窗口结束前给 Agent 更多时间产生输出：

* **单个 Agent**：在其 `agent()` 调用中以毫秒为单位传入 `stallMs`，例如 `agent(prompt, { stallMs: 1800000 })` 表示 30 分钟
* **所有 Agent**：设置 [`CLAUDE_ASYNC_AGENT_STALL_TIMEOUT_MS`](https://code.claude.com/docs/zh-CN/env-vars#variables)，它也适用于工作流之外的子代理

失败后运行是否继续，取决于您的脚本如何调用该 Agent：

* **在 [`parallel()` 或 `pipeline()`](#what-the-saved-script-looks-like) 内部**：运行会继续，以 `null` 代替该 Agent 的结果
* **直接 await**：运行会随错误结束

要重试，请让 Claude 重新启动工作流。[暂停后恢复](#resume-after-a-pause)介绍了哪些内容会再次运行。

### 成本

工作流会生成许多 Agent，因此单次运行使用的 token 可能明显多于在对话中完成同一任务。运行会计入您套餐的用量和速率限制。

要在投入大型任务之前估算开销，请先在一小部分内容上运行工作流：一个目录而不是整个仓库，或者一个具体的问题而不是宽泛的问题。`/workflows` 视图会在运行过程中显示每个 Agent 的 token 用量，您可以随时在那里停止运行，通常不会丢失已完成的工作。[暂停后恢复](#resume-after-a-pause)介绍了已停止的运行会保留哪些内容。运行时的 [Agent 上限](#behavior-and-limits)限制了单次运行可以生成的 Agent 数量，从而限定了失控脚本的成本。要让运行使用更少的 Agent，请选择 `small` [规模指导](#set-a-size-guideline)。

Claude Code 还会标记规模异常大的运行。当工作流安排的 Agent 超过 25 个，或其预计的 token 总量超过 150 万时，输入框下方任务面板中的进度行会显示 `Large workflow` 警告。该警告会引导您前往 [`/workflows`](#watch-the-run)，您可以在那里停止运行。

该警告仅作提示：它不会暂停或限制运行。有两项设置会改变您看到它的时机：

* 如果您自行选择了[规模指导](#set-a-size-guideline)，其 Agent 数量会取代 25 个 Agent 的阈值。内置的默认指导会将阈值保持为 25。
* 开启了 [ultracode](#let-claude-decide-with-ultracode) 的会话不会显示该警告，因为开启 ultracode 就已表示您选择接受大型运行。

Claude Code 为每个工作流 Agent 选择模型的顺序与[为子代理选择模型的顺序](https://code.claude.com/docs/zh-CN/sub-agents.md#choose-a-model)相同。脚本为某个阶段指定的模型在该顺序中算作按调用指定的模型。当没有其他来源指定模型时，Agent 会使用您会话的模型运行。

要控制模型成本：

* 如果您通常会为日常工作切换到较小的模型，请在大型运行之前检查 `/model`
* 在描述任务时，让 Claude 为不需要最强模型的阶段使用较小的模型

当您组织的 [`availableModels` 允许列表](https://code.claude.com/docs/zh-CN/model-config#restrict-model-selection)阻止了脚本为某个 Agent 请求的模型时，该 Agent 会改用替代模型运行，遵循与[子代理相同的替代规则](https://code.claude.com/docs/zh-CN/sub-agents.md#choose-a-model)。

### 设置规模指导

规模指导告诉 Claude 在编写动态工作流时以多少个 Agent 为目标。Claude Code 将该指导作为建议而不是上限发送给 Claude，因此要求不同规模的提示词仍会覆盖它。

每个值对应一个 Agent 数量：

| 值 | Claude 的目标 Agent 数量 |
| :- | :- |
| `unrestricted` | 无指导：Claude 根据任务确定工作流规模 |
| `small` | 少于 5 个 Agent |
| `medium` | 少于 10 个 Agent |
| `large` | 少于 50 个 Agent |

默认值为 `medium`，如果您使用 Pro 套餐登录且 Claude Code 为 v2.1.271 或更高版本，则默认值为 `small`。在您选择某个值之前，`/config` 中的该行会将该值标记为默认值，工作流的 `Running in background` 行会显示当前生效的规模。

要更改指导，请在 `/config` 中为 Dynamic workflow size 设置选择一个值，或运行 `/config workflowSizeGuideline=small`。您还可以在任意设置文件中设置 [`workflowSizeGuideline` 键](https://code.claude.com/docs/zh-CN/settings-reference#workflowsizeguideline)；该值优先于 `/config`，并且当设置文件提供了该值时，Claude Code 会隐藏 `/config` 中的该行。

更改会在下一个提示词时生效。无论该设置如何，[运行时 Agent 上限](#behavior-and-limits)仍然适用。

### 关闭工作流

工作流可在 CLI、Desktop 应用、IDE 扩展、使用 `claude -p` 的[非交互模式](https://code.claude.com/docs/zh-CN/headless.md)以及 [Agent SDK](https://code.claude.com/docs/zh-CN/agent-sdk/overview.md) 中使用。相同的禁用设置适用于每个使用入口。

要为您自己关闭工作流：

* 在 `/config` 中关闭 Dynamic workflows。在会话之间保持有效。
* 在 `~/.claude/settings.json` 中设置 `"disableWorkflows": true`。在会话之间保持有效。
* 设置 `CLAUDE_CODE_DISABLE_WORKFLOWS=1`。在启动时读取，因此在您设置它的任何位置都适用。

要为整个组织关闭工作流，请在[托管设置](https://code.claude.com/docs/zh-CN/server-managed-settings.md)中设置 `"disableWorkflows": true`，或使用 [Claude Code 管理设置](https://claude.ai/admin-settings/claude-code)页面上的开关。

工作流被禁用时：

* `/workflows`、工作流命令以及 `/workflow-authoring` skill 不可用
* `ultracode` 关键字不再触发运行，并且 **Ultracode** 开关会从 `/effort` 中移除

已在进行中的运行会继续进行。

关闭工作流也会使 [ultracode](#let-claude-decide-with-ultracode) 不可用。没有任何托管设置可以单独禁止 ultracode：在 ultracode [可用](https://code.claude.com/docs/zh-CN/model-config.md#when-ultracode-is-available)的任何地方，用户都可以使用 `/effort ultracode` 开启它。[effort 上限](https://code.claude.com/docs/zh-CN/model-config.md#organization-effort-limits)会降低开启了 ultracode 的会话运行时所用的 effort 级别，但不会关闭 ultracode。

## 相关资源

* [并行运行代理](https://code.claude.com/docs/zh-CN/agents.md)：比较子代理、代理视图、代理团队和工作流
* [创建自定义子代理](https://code.claude.com/docs/zh-CN/sub-agents.md)：工作流编排的工作者原语
* [管理成本](https://code.claude.com/docs/zh-CN/costs.md)：多代理运行如何计入使用限制
