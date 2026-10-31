> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# 使用 mod 响应事件

> 从 mod 处理 Claude Code 事件：观察、重写或回答工具调用、提示和轮次，过滤 hook 处理的事件，并为其他 mod 做计划。

hook 是一个事件处理程序：Claude Code 在命名事件发生时运行的函数。Claude Code 在即将采取行动的每个点触发事件，例如当它运行工具、提交提示、向模型发送请求或启动或结束会话时。你的 hook 在 Claude Code 采取行动之前运行，因此它可以观察事件、重写事件或代替 Claude Code 回答事件。你使用 [`on(eventName, handler)`](https://code.claude.com/docs/zh-CN/plugins/mods/reference#the-hook-function) 注册 hook。

在开始之前，请构建你的[第一个 mod](https://code.claude.com/docs/zh-CN/plugins/mods/create.md)。对于每个事件及其确切字段，请参阅[参考](https://code.claude.com/docs/zh-CN/plugins/mods/reference.md#events)或阅读[你的构建的类型](https://code.claude.com/docs/zh-CN/plugins/mods/create.md#get-the-types-for-your-build)。

## hook 如何处理事件

hook 位于事件和 Claude Code 对其采取的行动之间，因此它可以观察事件、重写事件或自己回答事件。它接收三个参数：[mods API](https://code.claude.com/docs/zh-CN/plugins/mods/api.md) 作为 `$`、事件作为 `e` 和下一个处理程序作为 `next`。事件的处理程序形成中间件链。`next(e)` 调用下一个处理程序，这是另一个 mod 的 hook 或链末端的 Claude Code 自己的行为，它解析为结果。你的 hook 对 `next` 做什么决定了它做以下三件事中的哪一件。

### 观察事件

要观察事件而不改变它，请执行你的工作并返回 `next(e)`。此 hook 记录 Claude 即将使用的每个工具：

```javascript theme={null}
on('tool.call', async ($, e, next) => {
  // 在工具运行之前运行
  $.ui.log('Claude is about to use ' + e.tool)
  // 原样传递事件
  return next(e)
})
```

在每个工具运行之前，转录中会出现一条暗线，例如 `● my-mod: Claude is about to use Bash`，其中 `my-mod` 是你的插件的名称。工具的运行方式与没有 mod 时相同。

要在事件后采取行动，请 `await next(e)`、执行你的工作并返回结果。此 hook 在每个工具运行后记录它：

```javascript theme={null}
on('tool.call', async ($, e, next) => {
  // 让工具运行，并等待其结果
  const result = await next(e)
  // 在工具运行后运行
  $.ui.log(e.tool + ' finished')
  // 原样返回结果
  return result
})
```

该行现在出现在每个工具完成后。Claude 读取相同的结果，因为 hook 返回 `next(e)` 解析的内容。

### 重写事件

要更改 Claude Code 作用的内容，例如提示词的文本，请使用修改后的事件副本调用 `next`。事件本身是不可变的：它被深度冻结，对字段赋值会抛出错误。此 hook 在发送前修剪每个提示词：

```javascript theme={null}
on('prompt.submit', async ($, e, next) => {
  // 传递事件的副本，其文本已更改
  return next({ ...e, text: e.text.trim() })
})
```

后续处理程序和 Claude Code 接收修剪后的提示，永远看不到原始提示。你也可以更改结果：`await next(e)`，然后返回替换了字段的结果副本。

### 回答事件

要自己处理事件，请返回结果而不调用 `next`。这会短路链，因此后续 mod 和 Claude Code 自己的行为不会运行。此 hook 拒绝每个 Bash 命令：

```javascript theme={null}
on('tool.call', { tool: 'Bash' }, async () => {
  // 没有调用 next，所以命令永远不会运行
  return { deny: 'Bash is turned off in this project. Use the file tools.' }
})
```

当 Claude 尝试 Bash 命令时，命令不会运行，Claude 将 `deny` 文本读作工具的结果。每个事件都有自己的结果形状，[事件参考](https://code.claude.com/docs/zh-CN/plugins/mods/reference.md#events)列出了这些。

### 过滤 hook 处理的事件

要仅为某些事件运行 hook，请将过滤器作为第二个参数传递给 `on`。Claude Code 将过滤器称为 matcher。它是一个对象，其字段与事件的字段进行比较，只有当每个字段都匹配时，hook 才会运行。字段可以是值、允许值的数组或正则表达式。

此示例中的每一行都为更窄的工具调用集合注册相同的函数 `hook`：

```javascript theme={null}
// 字符串匹配一个值：仅 Bash 调用
on('tool.call', { tool: 'Bash' }, hook)
// 数组匹配其中任何值：Edit 调用和 Write 调用
on('tool.call', { tool: ['Edit', 'Write'] }, hook)
// 正则表达式按模式匹配：来自一个 MCP 服务器的每个工具
on('tool.call', { tool: /^mcp__github__/ }, hook)
```

`hook` 为 Bash、Edit 或 Write 调用各运行一次，为名称以 `mcp__github__` 开头的工具调用运行一次。对任何其他工具（如 Read）的调用都不匹配这三个中的任何一个，因此 `hook` 不会为它运行。

事件名称可以是通配符。`'classic.*'` 匹配每个[设置 hook 事件](#hook-the-settings-hook-events)。`'*'` 匹配除[遥测事件](https://code.claude.com/docs/zh-CN/plugins/mods/reference.md#telemetry)之外的每个事件，遥测事件需使用其自身的名称和 `{ to: 'collector' }` 过滤器。

为每个 matcher 注册一次事件。如果你为 `session.start` 调用 `on` 两次而没有 matcher，模块将无法加载，错误为 `on("session.start") is registered twice without a matcher`。将你的 mod 在会话启动时执行的所有操作放在一个 hook 中。

## 对 Claude 正在执行的操作设置 hook

处理这些事件，可以在工具调用、提示词、轮次或设置读取发生时查看或更改它们。有关每个事件以及 hook 可以返回的内容，请参阅[事件参考](https://code.claude.com/docs/zh-CN/plugins/mods/reference.md#events)。

### 拦截或更改工具调用

`tool.call` hook 会看到 Claude 即将使用的每个工具，因此可以拒绝调用、更改其参数或放行。`tool.call` 在 Claude Code 即将运行某个工具时触发，包括子代理发起的调用以及对 MCP 工具的调用。`e.tool` 是工具名称，工具的参数是 `e` 的字段，例如 Bash 的 `e.command`。调用 `next(e)` 时，Claude Code 会先运行权限检查，然后运行该工具。

以下 hook 会拒绝执行强制推送的 Bash 命令，并告诉 Claude 原因：

```javascript theme={null}
// The matcher limits the hook to Bash calls, so e.command is the shell command
on('tool.call', { tool: 'Bash' }, async ($, e, next) => {
  if (/git push .*--force/.test(e.command)) {
    // Returning without calling next answers the event, so the command never runs
    return { deny: 'Force pushes are not allowed in this repository. Push to a new branch instead.' }
  }
  // Every other command goes on to the permission check and then to Bash
  return next(e)
})
```

当 Claude 尝试执行 `git push --force` 时，该命令不会运行，也不会出现权限提示，因为该 hook 从未调用 `next`。Claude 会将 `deny` 文本作为工具的结果读取，因此请将其写成 Claude 可以据此采取行动的指令。其他所有 Bash 命令的运行方式与没有该 mod 时相同。

如果此 hook 抛出异常或超时，Claude Code 会跳过它，命令会像没有该 mod 时一样继续执行。若要改为拒绝该命令，请为该 hook [添加 `.catch` 处理程序](#handle-a-hook-that-fails)。

要在工具运行之后执行操作，请 `await next(e)`，完成您的工作，然后返回 `next` 给您的结果。以下 hook 使用 [`$.ui.log`](https://code.claude.com/docs/zh-CN/plugins/mods/api#show-something-without-starting-a-turn) 记录 Claude 更改的每个 `.mdx` 文件，该方法会在会话记录中添加一行 Claude 不会读取的暗色文本：

```javascript theme={null}
on('tool.call', { tool: ['Edit', 'Write'] }, async ($, e, next) => {
  // Wait for the permission check and the tool, and keep what they produced
  const result = await next(e)
  // A refused call comes back as { deny }, and a failed one has isError set
  const changed = !result.deny && !result.isError
  if (changed && e.file_path.endsWith('.mdx')) $.ui.log('Claude changed ' + e.file_path)
  // Return the result as it came, so Claude reads what the tool returned
  return result
})
```

在 Claude 编辑或写入 `.mdx` 文件后，会话记录中会出现一行暗色文本，注明该文件名。对于其他类型的文件，或者被拒绝或失败的调用，不会记录任何内容。Claude 对该调用的认知不会改变，因为该 hook 返回的是它收到的结果。

您的 hook 还可以更改调用、重试调用、自行响应调用或扣留其结果：

* **更改调用**：将更改后的参数传给 `next`。
* **重试调用**：再次调用 `next(e)`。如果 hook 在第一次结果中看到 `isError`，可以再次运行该工具并返回那次的结果。
* **自行响应调用**：在不调用 `next` 的情况下返回一个带有 `result` 字段的对象；对于内置工具，请让 `result` 的结构与该工具自身结果在[适用于您构建版本的类型](https://code.claude.com/docs/zh-CN/plugins/mods/create.md#get-the-types-for-your-build)中的结构一致。不会出现权限提示，工具也不会运行，因此您返回的结果就是 Claude 对所发生情况的全部了解。
* **向 Claude 扣留结果**：在 `await next(e)` 之后返回 `{ deny: reason }`。Claude 读取的是您的原因，而不是 `next` 返回的内容。如果工具已运行，deny 会阻止 Claude 看到其结果，但不会撤销工具所做的任何操作。如果工具已运行并成功，原因会跟在诸如 `Bash ran, and a plugin withheld its result:` 的说明之后。

您组织的[托管设置](https://code.claude.com/docs/zh-CN/server-managed-settings.md)中的 hook 会在任何 mod 的 `tool.call` hook 之前运行。当托管 hook 阻止该调用时，任何 mod 的 `tool.call` hook 都不会针对该调用运行。

#### 暂缓工具调用，直到用户做出决定

hook 可以暂停工具调用，并在其继续之前询问用户如何处理。`tool.call` hook 可以在调用 `next` 或返回之前执行 `await`，在此之前工具调用会一直处于挂起状态。要向用户提出问题，请调用 `$.ui.ask`。它会在 Claude 向您提问时使用的对话框中，将您的问题显示在您的选项编号列表上方，并解析为用户选择的标签。在您的选项之后，对话框会添加一行用于输入其他答案，以及一行 **Chat about this**。

此示例中的 `RISKY` 模式匹配 `rm -r`、`rm -rf`、`git reset --hard` 以及带有 `--force` 的 `git push`，但不匹配其他写法，例如 `git push -f`。此模块会在运行与该模式匹配的 Bash 命令之前进行询问：

```javascript theme={null}
const RISKY = /\brm\s+-rf?\b|\bgit\s+reset\s+--hard\b|\bgit\s+push\b.*--force/

export function register(on) {
  on('tool.call', { tool: 'Bash' }, async ($, e, next) => {
    // Let every other command through without a question
    if (!RISKY.test(e.command)) return next(e)
    // Start from the safe answer, so a question nobody answers refuses the command
    let answer = 'Refuse'
    try {
      // The tool call waits here until the user picks one of the two labels
      answer = await $.ui.ask('Run this command? ' + e.command, ['Run it', 'Refuse'])
    } catch {
      // The user dismissed the question, or this is a claude -p run with nobody to ask
    }
    if (answer !== 'Run it') {
      // Answer without calling next, so the command doesn't run
      return { deny: 'The user declined this command. Ask before trying a different approach.' }
    }
    return next(e)
  })
}
```

当 Claude 尝试执行诸如 `rm -rf build` 之类的命令时，会出现包含该命令的问题，命令会等待回答：

* **用户选择 Run it**：hook 调用 `next(e)`，之后仍会运行常规的权限检查
* **用户选择 Refuse**：命令不会运行，Claude 会读取 `deny` 文本
* **用户输入答案**：`$.ui.ask` 解析为输入的文本。hook 会将其与 `Run it` 比较，因此任何其他文本都会拒绝该命令。
* **无人回答**：当用户关闭问题或选择 **Chat about this** 时，以及在 `claude -p` 运行中，`$.ui.ask` 会 reject，因此 `catch` 块会将答案保留为 `Refuse`

请将等待保持在诸如 `$.ui.ask` 之类的 mods API 调用内部，因为这段时间不计入 hook 的[时间限制](https://code.claude.com/docs/zh-CN/plugins/mods/reference.md#limits)。等待您自己的 promise 所花费的时间则会计入。

Claude Code 会跳过超时的 hook，因此被暂缓的命令将会运行。若要改为拒绝该命令，请在 `on` 返回的注册对象上链式添加[一个 `.catch` 处理程序](#handle-a-hook-that-fails)。该处理程序与 hook 内部的 `catch` 块是分开的。

#### 在询问用户之前批准或拒绝工具调用

要决定某个工具调用是否可以运行，请处理 [`tool.check`](https://code.claude.com/docs/zh-CN/plugins/mods/reference#tools)，这是 Claude Code 做出该决定的事件。

对于固定的命令或路径，请使用诸如 `Bash(npm test)` 之类的[权限规则](https://code.claude.com/docs/zh-CN/permissions.md#permission-rule-syntax)，无需编写代码。当决定取决于当下的实际情况（例如当前的 Git 分支或另一个 hook 记录的值）时，请处理 `tool.check`。

* **返回什么**：要自行决定该调用，请返回您自己的对象，例如 `{ decision: 'allow' }` 或 `{ decision: 'deny', reason }`。要保持该决定不变，请返回 `await next(e)` 给您的结果。
* **`await next(e)` 给您什么**：Claude Code 自身的决定，形式为一个对象，其 `decision` 为 `allow`、`ask` 或 `deny`。`next(e)` 运行期间不会出现权限提示。
* **参数**：`e.input` 保存工具的参数，例如 Bash 的 `command`
* **`tool.check` 何时触发**：在 [`PreToolUse` hook](https://code.claude.com/docs/zh-CN/hooks#pretooluse) 运行之后，并在权限提示、[自动模式](https://code.claude.com/docs/zh-CN/permission-modes.md#eliminate-prompts-with-auto-mode)的分类器或 [`PermissionRequest`](https://code.claude.com/docs/zh-CN/hooks#permissionrequest) hook 之前

以下 hook 会在当前分支为 `main` 时拒绝 `git push`：

```javascript theme={null}
on('tool.check', { tool: 'Bash' }, async ($, e, next) => {
  // Claude Code's own decision
  const decided = await next(e)
  if (!e.input.command.includes('git push')) return decided
  const branch = await $.process.run(['git', 'branch', '--show-current'])
  if (branch.stdout.trim() !== 'main') return decided
  return { decision: 'deny', reason: 'Push from a branch other than main' }
})
```

该 hook 会在 `main` 上拒绝 `git push`，并放行其他所有调用：

* **在 `main` 上**：即使有规则允许 `git push`，该 hook 也会返回 `deny`
* **在其他分支上以及对于其他命令**：调用会得到与没有该 mod 时相同的决定
* **hook 抛出异常或超时**：调用同样会得到与没有该 mod 时相同的决定。若要改为拒绝该调用，请[添加一个 `.catch` 处理程序](#handle-a-hook-that-fails)，使其返回 `{ decision: 'deny', reason: 'the reason' }`。

该 hook 匹配的是命令文本，因此请将其视为对 Claude 的提醒。要为所有人阻止向 `main` 推送，请在您的 Git 托管平台上保护该分支。

> [!NOTE]
>
> `tool.check` hook 也可以返回 `allow` 或 `ask`。在[计划模式](https://code.claude.com/docs/zh-CN/permissions.md#hook-approvals-in-plan-mode)下，对于非只读的调用（例如文件编辑），您的 hook 可以收紧 `next(e)` 返回的决定，但不能放宽它。有关您的 hook 的 `allow` 无法覆盖的权限规则和 `PreToolUse` hook 决定，请参阅[使用 hook 扩展权限](https://code.claude.com/docs/zh-CN/permissions.md#precedence-among-rules-hooks-and-mods)。

#### 检查工具调用是否为只读

在 [`tool.check`](#approve-or-refuse-a-tool-call-before-the-user-is-asked) hook 中，当工具将该调用报告为只读时，`e.isReadOnly` 为 `true`。否则该字段不存在，而不是 `false`。`e.isReadOnly` 需要 Claude Code v2.1.304 或更高版本。

某些工具（例如 Read、Grep、Glob 和 WebSearch）会将每次调用都报告为只读。Bash 和 MCP 工具在以下情况下将调用报告为只读：

* **Bash**：命令通过 Claude Code 内置的[只读命令检查](https://code.claude.com/docs/zh-CN/permissions.md#read-only-commands)
* **MCP 工具**：服务器将该工具的 `readOnlyHint` 注解设置为 `true`

该值是工具自身的报告，因此不要仅凭此字段批准调用。例如，WebFetch 和 Agent 调用会报告为只读，但 [WebFetch](https://code.claude.com/docs/zh-CN/tools-reference.md#webfetch-tool-behavior) 会请求一个 URL，而 [Agent](https://code.claude.com/docs/zh-CN/tools-reference.md#agent-tool-behavior) 会启动一个运行其自身工具的子代理。

以下 hook 会为每个非只读且尚未被拒绝的 `mcp__github__` 调用返回 `ask`：

```javascript theme={null}
on('tool.check', { tool: /^mcp__github__/ }, async ($, e, next) => {
  const decided = await next(e)
  // Read-only calls and denied calls pass through unchanged
  if (e.isReadOnly || decided.decision === 'deny') return decided
  return { decision: 'ask', reason: 'This tool can change the GitHub repository' }
})
```

#### 按调用是否为只读筛选 `tool.check` hook

要按 [`e.isReadOnly`](#check-whether-a-tool-call-is-read-only) 限制您的 hook 处理哪些调用，请对只读调用使用[匹配器](#filter-which-events-a-hook-handles)，对其余调用使用 hook 主体：

* **仅只读调用**：在匹配器中添加 `isReadOnly: true`，例如 `on('tool.check', { tool: 'Bash', isReadOnly: true }, hook)`
* **非只读调用**：在 hook 主体中测试 `e.isReadOnly`，如 [`mcp__github__` 示例](#check-whether-a-tool-call-is-read-only)所示。如果您在匹配器中写入 `isReadOnly: false`，您的 hook 将永远不会运行。

### 改写或补充提示词

`prompt.submit` hook 会在轮次开始之前看到每个提示词，因此可以改写文本或向其中添加内容。`e.text` 是输入的内容。

| 要执行的操作 | 返回的内容 |
| :- | :- |
| 改写提示词。会话记录和您的[提示词历史](https://code.claude.com/docs/zh-CN/interactive-mode.md#command-history)会显示新文本。 | `next({ ...e, text: newText })` |
| 在提示词之后添加只有 Claude 读取的文本 | `next({ ...e, context: [...(e.context ?? []), extraText] })` |
| 阻止发送提示词 | `{ drop: 'the reason' }` |

以下 hook 会在提示词提及 Pull Request 时，为 Claude 添加当前分支名称：

```javascript theme={null}
on('prompt.submit', async ($, e, next) => {
  // Pass on a prompt that doesn't mention a pull request as it is
  if (!/\bPR\b|pull request/i.test(e.text)) return next(e)
  const git = await $.process.run(['git', 'branch', '--show-current'])
  // Outside a git repository the command fails, so there's no branch to add
  if (git.exitCode !== 0) return next(e)
  // Keep any context an earlier hook added, and add one more line for Claude
  return next({ ...e, context: [...(e.context ?? []), 'Current branch: ' + git.stdout.trim()] })
})
```

当您发送诸如 `open a PR for this change` 之类的提示词时，您的消息在会话记录中看起来不变，而 Claude 还会在其后读取诸如 `Current branch: feature/auth` 的一行。未提及 Pull Request 的提示词会原样通过，且不会运行 `git`。

要阻止提示词，请在不调用 `next` 的情况下返回 `{ drop: 'the reason' }`。文本会返回到用户的提示词输入框中，用户会看到 `Prompt dropped by a hook:` 后跟您的原因，因此请以用户为对象撰写原因。在以下情况下，提示词仍会被发送：

* **您的 hook 在其 `next(e)` 调用已放行提示词之后返回 `drop`**：轮次仍会运行，并且该 hook 会[失败](#handle-a-hook-that-fails)，错误消息中包含 `a drop after its next() was answered`
* **您的 hook 抛出异常或超时**：Claude Code 会跳过它并发送提示词。请[添加 `.catch` 处理程序](#handle-a-hook-that-fails)，以免失败时发送了本应被该 hook 阻止的提示词。

[其他事件](https://code.claude.com/docs/zh-CN/plugins/mods/reference.md#prompts-and-what-claude-reads)涵盖了 Claude 读取的其余内容：`prompt.section` 用于系统提示词的每个部分，`prompt.context` 用于随第一条消息发送的上下文，`skill.prompt` 用于 skill 的文本。这些 hook 产生的文本如果在请求之间发生变化，会[使提示缓存失效](https://code.claude.com/docs/zh-CN/prompt-caching.md)。在 `skill.prompt` hook 中，您还可以读取[该 skill 来自哪个插件或文件](https://code.claude.com/docs/zh-CN/plugins/mods/reference.md#skill-prompt-fields)。

### 跟踪轮次

轮次是 Claude 为回应一个提示词所做的全部工作。处理 `turn.start`、`turn.step` 和 `turn.complete` 来跟踪一个轮次：

| 事件 | 触发时机 | hook 可以执行的操作 |
| :- | :- | :- |
| `turn.start` | 轮次开始 | 观察。`e.turnId` 在另外两个事件中标识该轮次。 |
| `turn.step` | Claude Code 即将向模型发送一个请求。包含工具调用的轮次会有多个请求。子代理的请求会设置 `e.agentId`。 | 读取每个请求的 token 用量、通过 `next({ ...e, model })` 将其发送到不同的模型，或在不调用模型的情况下直接响应 |
| `turn.complete` | 轮次结束，包括用户中断的轮次，此时 `e.isAborted` 为 `true`。`e.answer` 是 Claude 的最终文本，`e.durationMs` 是所用时间，`e.usage` 是该轮次的 token 总量。子代理的轮次触发该事件时会设置 `e.agentId`。 | 观察，或返回一个带有 `text` 字段的对象（例如 `{ text: 'Done in 12 seconds' }`），以在回答下方显示一行 |

请将 `turn.step` hook 编写为异步生成器，因为该事件是流式的。`yield* next(e)` 会在响应流式传输时将其转发，并求值为完成后的结果。以下 hook 会记录 Claude API 从[提示缓存](https://code.claude.com/docs/zh-CN/prompt-caching.md)中为每个请求提供了多少内容：

```javascript theme={null}
// function* makes the hook a generator, which can pass the response on piece by piece
on('turn.step', async function* ($, e, next) {
  // Send the request, forward each piece as it arrives, and keep the finished result
  const result = yield* next(e)
  // Skip a result that reports no token counts
  if (result.usage) {
    $.ui.log('cache read ' + result.usage.cache_read_input_tokens + ' · wrote ' + result.usage.cache_creation_input_tokens)
  }
  // Return the result unchanged, so the turn continues as usual
  return result
})
```

Claude 的回复会像没有该 mod 时一样以流式方式显示在屏幕上。每个请求完成后，会话记录中会出现一行暗色文本，给出从缓存读取的 token 数和写入缓存的 token 数。包含工具调用的轮次会有多个请求，因此会添加多行。

`result.usage` 保存 Claude API 为某个请求报告的 token 计数，以及作出回答的 `model`：`input_tokens`、`output_tokens`、`cache_read_input_tokens` 和 `cache_creation_input_tokens`。该 hook 也会针对子代理的请求运行，因此如果您只想处理主对话，请检查 `e.agentId`。

要查看 API 在请求期间自行运行的工具调用（例如对 [advisor 工具](https://code.claude.com/docs/zh-CN/advisor.md)的调用），请读取 `result.serverToolUses`。Claude Code 不会运行这些调用，因此不会为它们触发任何 `tool.call` 或 `tool.check` hook。当响应中没有此类调用时，该字段不存在；该字段需要 Claude Code v2.1.290 或更高版本。

#### 将请求发送到不同的模型

如果在 `model` 为别名（例如 `haiku`）时 `turn.step` 请求因模型错误而失败，请更新到 Claude Code v2.1.309 或更高版本，或使用完整的模型 ID。在 v2.1.309 之前，别名会按原样发出。

要将请求发送到不同的模型，请将更改了 `model` 的事件传给 `next`，例如 `yield* next({ ...e, model: 'haiku' })`。

### 更改或拒绝设置读取

使用 `settings.read` hook 可以更改或拒绝设置读取，无论是 mod 的读取还是 Claude Code 自身的读取：

* **mod 的读取**：某个 mod 调用 [`$.settings.read`](https://code.claude.com/docs/zh-CN/plugins/mods/api#read-one-source-or-one-key)。
* **Claude Code 自身的读取**：您的 hook 首次运行于 Claude Code 合并会话设置时，此时会话的 mod 已加载完毕。当设置或已加载的插件发生变化时，您的 hook 会再次运行。

请检查哪些 mod 会收到此事件以及您的 Claude Code 版本：

* **内置守卫**：存在托管设置时，或者以 Team 或 Enterprise [`/login`](https://code.claude.com/docs/zh-CN/authentication#authentication-precedence) 作为会话的凭据时，[内置守卫](https://code.claude.com/docs/zh-CN/plugins/mods/admin.md#know-what-happens-by-default)可能会跳过您安装或通过 `--plugin-dir` 加载的 mod 中的此 hook。要进行检查，请在[调试日志](https://code.claude.com/docs/zh-CN/plugins/mods/troubleshoot.md#read-the-debug-log)中搜索 `settings.read bypassed by cc-plugin-sec-default`。如果包含该文本的行指明了您的 mod，请让您组织的管理员[将该 mod 安装为组织的 mod 之一](https://code.claude.com/docs/zh-CN/plugins/mods/admin.md#install-your-organizations-mods)。
* **版本**：该事件在 Claude Code 自身的读取上触发，以及 `e.layers` 字段，都需要 Claude Code v2.1.304 或更高版本。

#### `settings.read` hook 接收的内容

| 字段 | 类型 | 保存的内容 |
| :- | :- | :- |
| `e.layers` | array | 每个正在使用且包含设置的来源对应一个 `{ source, settings }` 对象，[优先级](https://code.claude.com/docs/zh-CN/settings.md#settings-precedence)最低的排在最前：`plugin`、`user`、`project`、`local`、`flag`、`policy`。在 `claude plugin test` 中为空。 |
| `e.source` | string | mod 请求的单个来源，例如 `$.settings.read({ source: 'policy' })`。此时 `e.layers` 仅包含该层。在 Claude Code 自身的读取以及任何其他对合并设置的读取中不存在。 |

在 `e.layers` 中，`plugin` 保存已启用插件提供的设置，它只是一个层名称，而不是可以传给 `$.settings.read` 的 `source`。`flag` 保存通过 `--settings` 传入的设置，`policy` 保存托管设置。

#### 区分 Claude Code 自身的读取与 mod 的读取

检查 [`next.origin`](https://code.claude.com/docs/zh-CN/plugins/mods/reference#the-hook-function)：

* **Claude Code 自身的读取**：`{ plugin: 'engine', tier: 'core' }`
* **mod 的读取**：`plugin` 为发起调用的 mod 的名称

#### `settings.read` hook 可以返回的内容

原样返回 [`next(e)`](#settings-read-next) 的结果以保持读取不变，或返回以下内容之一：

| 返回 | 在 [Claude Code 自身的读取](#settings-read-hook)中 | 在 mod 的读取中 |
| :- | :- | :- |
| `{ value }` | 对于每个被更改的键，如果您的值[符合该键的 schema](https://code.claude.com/docs/zh-CN/plugins/mods/troubleshoot.md#settings-read-key-refused)，您将替换会话合并设置中该键的整个值 | 该 mod 的调用将解析为您的 `value`，或解析为从您的 `value` 中取出的它所请求的 `key`，因此请返回一个完整的对象，而不是数组 |
| `{ deny: reason }` | 会话保留在任何 hook 作出响应之前合并得到的设置，并且[调试日志会记录您的 `reason`](https://code.claude.com/docs/zh-CN/plugins/mods/troubleshoot#settings-read-refused) | 该 mod 的调用会以 `<calling mod>: $.settings.read: <reason>` reject |

在 Claude Code 自身的读取中，请基于 [`next(e)` 的 `value`](#settings-read-next) 构建您的 `{ value }`，因为您遗漏的键会变为未设置。对于 Claude Code 从每个来源分别读取的键（例如 `env`），在您的 `value` 中更改它[不会产生效果](https://code.claude.com/docs/zh-CN/plugins/mods/troubleshoot.md#settings-read-key-no-effect)。

#### `next(e)` 提供给 `settings.read` hook 的内容

`next(e)` 会解析为 `{ value }` 或 `{ deny }`：

* **`{ value }`**：`value` 是合并后的设置；当设置了 [`e.source`](#settings-read-receives) 时，则是单个来源的设置。如果[在您的 hook 之后运行](#the-order-mods-run-in)的某个 hook 返回了自己的 `{ value }`，您得到的将是该 hook 的 `value`。
* **`{ deny }`**：在您的 hook 之后运行的某个 hook 拒绝了该读取。请原样返回该 `{ deny }`。

#### 从 `settings.read` hook 更改一项设置

首先检查[哪些 mod 会收到此事件，以及从哪个版本开始](#settings-read-hook)。

要更改一个键，请展开 [`next(e)`](https://code.claude.com/docs/zh-CN/plugins/mods/reference#the-hook-function) 中的 `value` 并设置该键，如下面这个针对 [`spinnerTipsEnabled`](https://code.claude.com/docs/zh-CN/settings-reference#spinnertipsenabled) 的 hook 所示。该 hook 不检查 [`next.origin`](#settings-read-origin)，因此它会在 [Claude Code 自身的读取](#settings-read-hook)和 mod 对合并设置的读取中同样更改该键：

```javascript theme={null}
on('settings.read', async ($, e, next) => {
  const read = await next(e)
  // Pass through a read of one source, so that only the merged settings change.
  // Pass through a { deny } from a later hook too: it has no value.
  if (e.source || !read.value) return read
  // Keep every other key and change this one
  return { value: { ...read.value, spinnerTipsEnabled: false } }
})
```

在 Claude Code 自身的读取中，如果您的 hook 返回了超出该键 schema 的值（例如字符串），则仅该键会保留在任何 hook 作出响应之前合并得到的值。调试日志随后会显示 [`the answer for <key> is no value the key can hold`](https://code.claude.com/docs/zh-CN/plugins/mods/troubleshoot#settings-read-key-refused)。如果您的值符合要求但该键仍未更改，请参阅[某个键为何不生效](https://code.claude.com/docs/zh-CN/plugins/mods/troubleshoot.md#settings-read-key-no-effect)。

### 处理设置 hook 事件

设置 hook 是您在设置文件中配置的命令、HTTP、提示词和 Agent hook。每个[设置 hook 事件](https://code.claude.com/docs/zh-CN/hooks.md#hook-events)（例如 `Stop`、`SessionEnd` 或 `PostToolUse`）同时也是一个名为 `classic.` 后跟该设置 hook 事件名称的事件，例如 `classic.Stop`。`e` 是设置 hook 在 stdin 上接收的 JSON，包括 `transcript_path`。在加载了[内置守卫](https://code.claude.com/docs/zh-CN/plugins/mods/admin.md#know-what-happens-by-default)的环境中，[您安装的 mod 中针对这些事件的 hook 不会运行](https://code.claude.com/docs/zh-CN/plugins/mods/admin.md#hooks-the-guard-passes-over)。

以下 hook 使用 `Stop`（在 Claude 完成回复时触发）来记录会话的会话记录保存位置：

```javascript theme={null}
on('classic.Stop', async ($, e, next) => {
  // e has the same fields a Stop hook in a settings file reads from stdin
  $.ui.log('Transcript saved at ' + e.transcript_path)
  // Pass the event on, so Stop hooks in your settings files still run
  return next(e)
})
```

每次 Claude 完成回复时，会话记录中会出现一行暗色文本，给出会话记录文件的路径。该 hook 返回 `next(e)`，因此它只观察该事件，不会改变轮次结束的方式。

## 与其他 mod 一起运行

多个 mod 可以处理同一事件，其中任何一个都可能失败。如果您的 mod 阻止工具调用，请检查它在链中的位置以及当其 hook 失败时会发生什么。

### mod 运行的顺序

同一事件上的 hook 形成一个中间件链。每个 mod 的 `next` 调用下一个 mod 的 hook，最后的 `next` 到达 Claude Code 自己的行为。第一个 mod 是最外层的：它在其他 mod 之前看到事件，在它们之后看到结果，并决定其他 mod 是否运行。后续 mod 无法阻止早期 mod 看到事件。

Claude Code 按每个 mod 的来源对链进行排序：

1. 内置守卫 `sec-default@builtin`（一个内置于 Claude Code 的 mod，`/plugin` 将其列为 `cc-plugin-sec-default`，在[它加载](https://code.claude.com/docs/zh-CN/plugins/mods/admin.md#know-what-happens-by-default)的位置）、您的组织在 [`prependPlugins`](https://code.claude.com/docs/zh-CN/plugins/mods/admin#install-your-organizations-mods) 中列出的 mod，然后是任何其他算作您的组织的、且不在 `appendPlugins` 中的 mod
2. 您安装的 mod
3. 您的组织在 `appendPlugins` 中列出的 mod
4. 其他内置于 Claude Code 的 mod

在您安装的 mod 中，mod 在它在清单中的 `dependencies` 下列出的 mod 之前运行。在一个模块中，hook 按 `register` 调用 `on` 的顺序运行。

#### 设置 hook 在顺序中运行的位置

在设置文件中配置的 `PreToolUse` hook 也在工具调用期间运行，位于 mod 链中的固定点：

* **来自托管设置的 `PreToolUse` hook**：在第一个 mod 的 `tool.call` hook 之前运行。当托管 hook 阻止调用时，该调用不会运行任何 mod 的 `tool.call` hook。
* **来自每个其他设置文件和插件的 `hooks/hooks.json` 的 `PreToolUse` hook**：在最后一个 mod 调用 `next` 后运行，作为 Claude Code 自己的行为的一部分。回答 `tool.call` 而不调用 `next` 的 mod 会阻止它们运行，调用 `next` 的 mod 在它返回的结果中看到它们的决定。

[`tool.check`](#approve-or-refuse-a-tool-call-before-the-user-is-asked) 在 `PreToolUse` hook 运行之后触发。要了解 `tool.check` hook 的 `allow` 无法替代哪些权限规则和 `PreToolUse` hook 决定，请参阅[使用 hook 扩展权限](https://code.claude.com/docs/zh-CN/permissions.md#precedence-among-rules-hooks-and-mods)。

### 处理失败的 hook

失败的 hook 不会破坏会话，您可以决定接下来会发生什么。当没有 `.catch` 处理程序的 hook 抛出、超时或返回错误形状的结果时，接下来会发生什么取决于它是否调用了 `next`：

* **它在调用 `next` 之前失败**：Claude Code 跳过它，下一个处理程序代替运行
* **它在 `next` 解析后失败**：该结果成立，没有任何东西运行第二次

一行内容会指明 mod、事件和原因，例如 `my-mod: tool.call hook skipped: threw Error: boom`。您读取它的位置取决于会话，如[找出 mod 为什么不做任何事](https://code.claude.com/docs/zh-CN/plugins/mods/troubleshoot.md#find-out-why-a-mod-does-nothing)所列。[树未通过验证](https://code.claude.com/docs/zh-CN/plugins/mods/interface.md#build-a-tree-from-elements)的 `ui.render` hook 视为返回了错误形状结果的 hook。

要使阻止调用的 hook 失败关闭，请添加一个 `.catch` 错误处理程序来代替回答。这里，`guard` 是您的 hook 函数，处理程序检查 [`next.called`](https://code.claude.com/docs/zh-CN/plugins/mods/reference#the-hook-function) 来判断 `guard` 在失败时是否已经调用了 `next`：

```javascript theme={null}
// on 返回一个注册，.catch 将处理程序附加到该 hook
on('tool.call', { tool: 'Bash' }, guard).catch(async ($, e, next) => {
  // guard 已经调用了 next，因此返回得到的结果
  if (next.called) return next(e)
  // next.error.kind 说明调用处理程序的原因，例如 'throw' 或 'timeout'
  return { deny: 'The command guard failed, so this command was not run: ' + next.error.kind }
})
```

当 `guard` 在 Bash 调用上抛出或超时时，Claude Code 使用相同的事件调用处理程序：

* **`guard` 在调用 `next` 之前失败**：命令不会运行，Claude 读取末尾带有该类型的 `deny` 文本
* **`guard` 在调用 `next` 之后失败**：处理程序的 `next(e)` 解析为 `guard` 的调用所产生的结果，而不会再次运行命令，Claude 读取该结果

处理程序自身有更短的[时间限制](https://code.claude.com/docs/zh-CN/plugins/mods/reference.md#limits)。如果处理程序本身抛出或超时，Claude Code 会像该 hook 没有处理程序一样跳过它。如果 `guard` 尚未调用 `next`，命令随后会像没有该 mod 时一样继续执行。

同样的处理程序形式也适用于 `prompt.submit` 或 `config.set` 上的守卫。当 `next.called` 为 false 时，返回[事件参考](https://code.claude.com/docs/zh-CN/plugins/mods/reference.md#events)为该事件列出的拒绝：`prompt.submit` 使用 `{ drop: 'the reason' }`，`config.set` 使用 `{ deny: 'the reason' }`。

在 `tool.check` 和 `plugin.register` 中，在 `next` 解析后返回的拒绝仍然有效，因此无需检查 `next.called`，直接返回即可：

* **`tool.check`**：返回 `{ decision: 'deny', reason: 'the reason' }`
* **`plugin.register`**：返回 `{ refuse: 'the reason' }`，如[在检查失败时拒绝 mod](https://code.claude.com/docs/zh-CN/plugins/mods/admin.md#refuse-mods-when-your-check-fails) 所示

在 `tool.call` 中，在 `next` 解析后返回的 `deny` 会[向 Claude 隐瞒该结果](#guard-or-change-a-tool-call)。

## 后续步骤

* [使用 mods API](https://code.claude.com/docs/zh-CN/plugins/mods/api.md)：添加命令和工具、调用模型并在计时器上运行工作
* [在界面中绘制](https://code.claude.com/docs/zh-CN/plugins/mods/interface.md)：在窗格中或提示上方显示你的 hooks 收集的内容
* [测试 mod](https://code.claude.com/docs/zh-CN/plugins/mods/test.md)：从测试中触发这些事件中的任何一个
* [Mods 参考](https://code.claude.com/docs/zh-CN/plugins/mods/reference.md)：每个事件、每个 mods API 方法和限制
