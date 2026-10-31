> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# 排查 mod 问题

> 了解为什么 Claude Code mod 不起作用：将症状或消息与其原因匹配，查找拒绝消息，并阅读调试日志。

当 mod 的模块或其中一个 hook 失败时，Claude Code 会跳过它，会话继续进行，因此损坏的 mod 看起来像什么都不做的 mod。首先检查 Claude Code 从 mod 读取了什么以及它在哪里报告问题，然后找到您遇到的症状或消息。

## 找出为什么 mod 不起作用

当 mod 不起作用时，请检查 Claude Code 从 mod 文件读取的内容，以及它在跳过某些内容时写入的行。对于第一项，在您的 shell 中运行 [`claude plugin validate`](https://code.claude.com/docs/zh-CN/plugins/mods/create#check-what-claude-code-reads-from-your-mod)，使用 mod 的目录，如 `claude plugin validate ./first-mod`。它可以捕获拼写错误的事件、错误的清单和 Claude Code 无法读取的模块，而无需启动会话。

当模块未加载、hook 被跳过或另一个 mod 拒绝您的 mod 时，Claude Code 会写入一行，其中命名您的 mod。您读取该行的位置取决于会话：

* **热重新加载插件目录的会话**：成绩单中的暗行。这是您使用 `--plugin-dir` 启动的交互式会话，或者是您为 Claude 编写的 mod [启用热重新加载](https://code.claude.com/docs/zh-CN/plugins/mods/create.md#ask-claude-for-a-mod) 的会话。
* **任何其他交互式会话，例如运行您从市场安装的 mod 的会话**：[调试日志](#read-the-debug-log) 仅。要获取一个，请使用 `claude --debug` 启动会话。
* **带有 `--plugin-dir` 的 `claude -p` 运行**：stderr，采用默认文本输出格式。另一个 mod 的拒绝仅进入调试日志。

## 检查 mod 是否可以加载

要检查您的设置是否允许 mod 加载，而无需安装一个，请在您的 shell 中从不包含 mod 的目录运行 `claude plugin test`。您不需要会话。它打印的消息告诉您状态：

| 消息包含 | 这意味着什么 |
| :- | :- |
| `no hooks module to load` | Mod 可以加载。该命令在此目录中找不到要测试的 mod。 |
| `hooks modules are turned off here` | 一个设置正在阻止您的 mod：您自己的设置中的 `disableAllHooks`，或您的组织的策略 |
| `hooks modules are turned off in this process: the rollout switch served off` | Anthropic 已远程关闭已安装的 mod。 |
| `hooks modules are turned off in this process: the rollout switch was saved off by an earlier session` | 该命令使用了之前某个会话保存的值，该值可能已过时。请启动一次 `claude` 以刷新该值，然后再次运行该命令。 |

组织还可以设置 `allowManagedModsOnly` 以仅允许其自己的 mod，此命令不会报告。在这种情况下，Claude Code 会拒绝您安装的 mod，并且[会有消息说明原因](https://code.claude.com/docs/zh-CN/plugins/mods/troubleshoot.md#messages-from-the-built-in-guard)。

## mod 不加载

mod 添加的任何内容都不会出现：没有命令、没有绘图，也没有行为改变。

### 您的版本过旧

请参阅[应使用哪个版本以及如何检查您的版本](https://code.claude.com/docs/zh-CN/plugins/mods/overview.md#turn-mods-on-or-off)。

### `mods active` 行不命名 mod

mod 添加的任何内容都不会出现，`/plugin` 中的 [`mods active` 行](https://code.claude.com/docs/zh-CN/plugins/mods/overview#see-which-mods-a-session-loaded) 不命名它。hooks 模块未加载。当 Claude Code 拒绝它时，调试日志有一行以 `hooks module`、mod 的名称和 `not loaded:` 开头，如 `hooks module first-mod@inline not loaded: disableAllHooks in managed settings`，用于使用 `--plugin-dir` 加载的 mod。

读取冒号后的原因。[拒绝消息](#refusal-messages) 部分列出了每一个。如果日志中没有这样的行，请逐一处理此组中的其他条目。

某些设置会阻止 mod，同时让其插件的其余部分继续工作。[启用或关闭 mod](https://code.claude.com/docs/zh-CN/plugins/mods/overview.md#turn-mods-on-or-off) 列出了这些设置。

### `claude -p` 运行打印 `hooks module not loaded`

该行以 mod 的名称开头并进入 stderr。hooks 模块被拒绝。非交互式运行没有成绩单，因此消息进入 stderr。

读取冒号后的原因。[拒绝消息](#refusal-messages) 部分列出了每一个。

### 拒绝消息

这些消息中的每一个都遵循调试日志中的 `hooks module`、mod 的名称和 `not loaded:`。

| 消息开头 | 这意味着什么 |
| :- | :- |
| `hooks modules are turned off for installed plugins in this process: the rollout switch served off` | Anthropic 已远程关闭已安装的 mod。 |
| `hooks modules are turned off for installed plugins in this process: the rollout switch was saved off by an earlier session` | 该会话使用了先前会话保存的值，该值可能已过时。请重新启动 Claude Code 以刷新该值。 |
| `disableAllHooks in managed settings` | 您的组织关闭了来自已安装插件的 hooks |
| `only managed plugins and built-in plugins run` | 设置了 `allowManagedHooksOnly`，或在托管设置以外的设置文件中设置了 `disableAllHooks` |
| `installed plugins that are not managed load no hooks module in this mode (--bare)` | 您使用 `--bare` 启动了 Claude Code |
| `another plugin of that name loads first` | 另一个已启用的插件与您的 mod 同名，并且[占用了该名称](https://code.claude.com/docs/zh-CN/plugins/loading.md#hooks-when-two-enabled-plugins-share-a-name)，因此您的 hook 模块不会加载 |

### 来自内置保护的消息

在具有托管设置的机器上，或在凭据为 Team 或 Enterprise `/login` 的会话中，[内置守卫](https://code.claude.com/docs/zh-CN/plugins/mods/admin.md#know-what-happens-by-default) 可以拒绝 mod 或其答案之一。每条消息都会指明您的组织管理员为更改该规则而设置的选项。

| 消息包含 | 这意味着什么 | 它出现在哪里 |
| :- | :- | :- |
| `mods are limited to your organization's by policy (allowManagedModsOnly)` | 您的组织仅允许 [其自己的 mod](https://code.claude.com/docs/zh-CN/plugins/mods/admin.md#install-your-organizations-mods)，因此您的 mod 被拒绝 | 调试日志，以及 [热重新加载插件目录的会话](#find-out-why-a-mod-does-nothing) 中的会话记录 |
| `tried to lift a deny rule in your settings` | 您的 mod 的 [`tool.check`](https://code.claude.com/docs/zh-CN/plugins/mods/reference#tools) hook 批准了 `deny` 规则拒绝的调用。该调用保持被拒绝。 | 成绩单和调试日志，会话中每个 mod 一次。在 `claude -p` 运行中，仅调试日志。 |
| `the deny rules in your settings could not be checked for this call, so it is refused` | 保护在检查 mod 批准的调用时失败，因此它拒绝了该调用 | Claude 为被拒绝的调用读取的原因 |

### `validate` 通过且不列出 `hooks` 行

`hooks/hooks.json` 没有 `modules` 键，或键拼写错误。

添加 `"modules": ["./register.js"]`。

### `hooks module did not load`

该行以 mod 的名称开头，然后是 `hooks module did not load:` 和一个原因，当问题在您的代码中时，它给出文件和行。Claude Code 无法加载模块，例如因为其顶级代码抛出了异常。

修复原因命名的错误。

如果原因以 `; the version loaded before is unloaded` 结尾，则该 mod 当前没有任何版本在运行。修复错误后，在会话中运行 [`/reload-plugins`](https://code.claude.com/docs/zh-CN/plugins/cli-reference#reload-plugins) 以再次加载该 mod。如果该 mod 是您安装的而非您编写的，请让其作者修复该错误。

### `options do not fit plugin.json userConfig`

该行以 mod 的名称开头，然后是 `hooks module did not load: options do not fit plugin.json userConfig:` 和一个原因。某个选项未通过其 [`userConfig`](https://code.claude.com/docs/zh-CN/plugins/components#user-configuration) 字段的验证，例如高于字段 `max` 的数字，或必需字段没有值。

设置或更改值。该行的末尾命名其在 `settings.json` 中的 `pluginConfigs` 条目。

### `code nested too deep to scan: more than 2000 scopes`

该行以 mod 的名称开头，然后是 `hooks module did not load:`、文件，以及 `code nested too deep to scan: more than 2000 scopes`。hook 模块中的文件不能将作用域（例如函数、块和循环）嵌套超过 [2,000 层](https://code.claude.com/docs/zh-CN/plugins/mods/reference.md#limits)。[`claude plugin validate`](https://code.claude.com/docs/zh-CN/plugins/mods/create#check-what-claude-code-reads-from-your-mod) 会报告相同的原因。

重写代码，减少其作用域的嵌套深度。

### `the engine ran out of stack linking the file and its imports`

hook 模块中的代码嵌套过深，或其导入链过长，导致 Claude Code 无法加载它。该行以 mod 的名称和 `hooks module did not load:` 开头，然后给出某个文件的名称以及此原因。

请在该文件以及通过其导入所到达的文件中查找嵌套最深的部分，并重写该部分以减少嵌套，或缩短导入链。如果该 mod 不是您编写的，请让其作者进行修复。

### `the parser stopped` 或 `the engine ran out of stack loading the file`

hook 模块某个文件中的代码嵌套过深，导致 Claude Code 在读取时耗尽了栈空间。该行以 mod 的名称和 `hooks module did not load:` 开头，然后给出该文件的名称以及以下原因之一：

* `code nested too deep to scan (the parser stopped: <message>)`
* `the engine ran out of stack loading the file (code nested too deep): <message>`

该原因不提供行号，也不说明嵌套的是哪种代码，因此请自行查看该文件，找出其嵌套最深的部分。重写该部分以减少嵌套。如果该 mod 是您安装的而非您编写的，请让其作者进行修改。

### 没有 mod 在您首次打开的目录中加载

您还没有回答该目录的信任提示。

使用 `claude` 在该目录中启动交互式会话，并接受它打开的信任提示。

### 没有已安装的插件加载

您使用 `--safe-mode` 启动了 Claude Code。

启动时不使用该标志。

### Claude Code 不再询问是否启用热重载

Claude 在交互式会话中编写了 mod，但没有任何内容加载，而且 Claude Code 不再询问[是否启用热重载](https://code.claude.com/docs/zh-CN/plugins/mods/create.md#ask-claude-for-a-mod)。如果该问题有三次在未选择任何答案的情况下结束，热重载将保持关闭。例如，当您设置了 [`askUserQuestionTimeout`](https://code.claude.com/docs/zh-CN/settings-reference#askuserquestiontimeout) 并且在您回答之前时间已到时，问题就会以这种方式结束。该设置在此适用，是因为 Claude Code 使用与 [`AskUserQuestion` 相同的问题对话框](https://code.claude.com/docs/zh-CN/tools-reference#question-auto-continue-timeout)进行询问。您自己关闭的问题不计入这三次。

要运行该 mod，请[将其目录从 mods 文件夹中复制出来](https://code.claude.com/docs/zh-CN/plugins/mods/create.md#use-the-mod-in-other-sessions)，然后在 shell 中使用 `--plugin-dir` 启动新会话，例如 `claude --plugin-dir ~/mods/git-branch`。

## hook 被跳过或 mod 被卸载

mod 已加载，然后 Claude Code 跳过了其中一个 hooks 或卸载了它。

### `hook skipped`

该行命名 mod 和事件，然后说 `hook skipped:` 和一个原因，如 `first-mod: tool.call hook skipped: threw Error: boom`。hook 抛出了异常、运行超过了其[时间限制](https://code.claude.com/docs/zh-CN/plugins/mods/reference.md#limits)，或返回了错误形状的结果。该行对每个事件和失败类型出现一次，直到 mod 重新加载。

修复错误。调试日志对每次出现都有一行。

### `no command.run hook answered it`

您运行了 mod 添加的命令，回复命名了 mod 和命令，如 `first-mod registered /tally but no command.run hook answered it`，然后告诉您添加一个 hook。当命令到达链的末尾而没有答案时，Claude Code 会打印该回复，这发生在两种情况下：

* **没有 hook 回答命令**：模块没有 `command.run` hook，hook 的 [filter](https://code.claude.com/docs/zh-CN/plugins/mods/events.md#filter-which-events-a-hook-handles) 命名了不同的命令，或 hook 返回了 `next(e)`
* **Claude Code 跳过了 hook**：[`hook skipped`](#hook-skipped) 列出了原因。将 `focus: false` 传递给 [`$.ui.open`](https://code.claude.com/docs/zh-CN/plugins/mods/interface#open-a-pane-at-the-right-time) 是到达那里的一种方式。

如果模块已经有回复描述的 hook，请查找命名 `command.run` 的 `hook skipped` 行，它给出了原因。运行命令的 [test](https://code.claude.com/docs/zh-CN/plugins/mods/test.md) 失败，原因相同。

### `it crashed the hooks worker`

该行以 mod 的名称开头，如 `first-mod was unloaded: it crashed the hooks worker`。已安装的 mod 共享一个工作线程。工作线程停止响应或崩溃，Claude Code 将其追踪到此 mod 并卸载了它。阻止线程的 hook（例如永不等待的循环）是一个原因。

修复 hook。

### `its session.start ran again in a fresh copy`

该行以 mod 的名称开头，并指出一个 `$.prompt.submit`、`$.command.run` 或 `$.agent.spawn` 调用，如 `first-mod: its session.start ran again in a fresh copy; the $.prompt.submit call it had already made was not made again`。Claude Code 再次加载了该 mod 的模块（例如在 hooks worker 崩溃并被替换之后），新副本的 [`session.start`](https://code.claude.com/docs/zh-CN/plugins/mods/reference#session) hook 运行了。该行指出的调用以其首次运行的结果完成，而没有再次运行，因此您的 mod 不会重复提交提示词、运行命令或启动子代理。hook 的其余部分照常运行。

无需修复。

在 v2.1.292 之前，该调用会再运行一次，因此提示词会被提交两次、命令会被运行两次，或子代理会被启动两次。

### `This mod's change didn't load` 或 `the earlier version is no longer running`

对[Claude 在本会话中编写的 mod](https://code.claude.com/docs/zh-CN/plugins/mods/create.md#ask-claude-for-a-mod) 所做的更改无法加载。会话记录中的该行以 mod 的名称开头，以原因结尾：

* `This mod's change didn't load, so the earlier version is still running: <reason>.`
* `This mod didn't load, and the earlier version is no longer running: <reason>.`

Claude 看不到此行，因此在请求 Claude 修复时，请将其粘贴到您的下一条消息中。

### `$.agent.register refused: the hooks module that made the call is no longer loaded`

该行以您的 mod 名称开头，如 `first-mod: $.agent.register refused: the hooks module that made the call is no longer loaded (it was reloaded or removed)`，且该 Agent 未被注册。您的 mod 在该调用之前被重新加载或卸载。重新加载会加载 hook 模块的新副本，而此调用来自仍在旧副本中运行的代码，例如尚未返回的 hook。

如果该 hook 未捕获此拒绝，它会失败，Claude Code 会[跳过它](#hook-skipped)。要从保持加载的副本注册该 Agent，请在您的 [`session.start`](https://code.claude.com/docs/zh-CN/plugins/mods/reference#session) hook 中进行该调用，该 hook 会在重新加载后的每个新副本中再次运行。

### `mods that run in the hooks worker are off for this session`

该行读取 `hooks: mods that run in the hooks worker are off for this session: it crashed 3 times`。工作线程停止了三次，Claude Code 无法将停止追踪到一个 mod，因此它卸载了每个不是内置的 mod，包括您的组织安装的 mod。此行在每个交互式会话中到达成绩单。

运行 `/reload-plugins` 以再次加载它们。

## 工具调用被拒绝

mod 已加载，其 hooks 运行，它接触的工具调用被拒绝。

### `a hook changed this call's input after the model wrote it`

在自动模式下，被拒绝的工具调用给出此原因。hook 在 [服务器端分类器](https://code.claude.com/docs/zh-CN/permission-modes.md#server-side-classifier-review) 审查后更改了工具调用的输入，因此该审查不涵盖将运行的内容。hook 可以是 mod 的 [`tool.call`](https://code.claude.com/docs/zh-CN/plugins/mods/reference#tools) 或 [`turn.step`](https://code.claude.com/docs/zh-CN/plugins/mods/reference#turns) hook，或 [`PreToolUse`](https://code.claude.com/docs/zh-CN/hooks#pretooluse) 设置 hook。消息不说明是哪一个。

消息告诉 Claude 再次发出记录的调用。如果也被拒绝，hook 每次都更改输入，因此关闭 mod 或 hook，或离开自动模式并自己批准调用。

### 关于您的设置中的拒绝规则的消息

`tried to lift a deny rule in your settings` 和 `the deny rules in your settings could not be checked for this call, so it is refused` 都来自内置保护。

在 [来自内置保护的消息](#messages-from-the-built-in-guard) 中查找它们。

## 停止轮次后出现提示

### `Stopped: type a message to continue; a mod cannot resume work after Stop`

此提示表示，在您停止轮次之后（例如按下 Esc），某个 mod 试图自行开始工作，而 Claude Code 没有允许它这样做。无需进行任何修复。在您发送下一条消息后，mod 可以再次开始工作，并且[其中部分工作会自行恢复](https://code.claude.com/docs/zh-CN/plugins/mods/api.md#work-your-mod-started-before-the-interruption)。如果您期望从该 mod 获得的结果没有恢复，请再次提出请求，或再次运行该 mod 的命令。

如果该 mod 是您编写的，请参阅[用户中断轮次后的调用和后台工作](https://code.claude.com/docs/zh-CN/plugins/mods/api.md#calls-and-background-work-after-the-user-interrupts-a-turn)。

## 绘图不出现或不响应

mod 已加载，其窗格、带、toast 或控件的行为不符合您的预期。

### 显示了另一个 mod 的带，而您的带没有显示

输入框上方只有一个带，所有 mod 共用它。在[您的 mod 之前](https://code.claude.com/docs/zh-CN/plugins/mods/events.md#the-order-mods-run-in)运行的 mod 如果返回的树中不包含 `next(e)` 的结果，就会替换您的带。要同时显示两个带，请修改该 mod 的 hook，或请其作者修改：

* **保留其后的带**：将 `await next(e)` 的结果放入其树中某个 `Box` 的子元素中，如[选择绘制位置](https://code.claude.com/docs/zh-CN/plugins/mods/interface.md#pick-where-to-draw)中“输入框上方的带”选项卡所示
* **保持树有效**：不要为该 `Box` 或其外层的任何 `Box` 设置 `width`、`height`、`minWidth`、`minHeight`、`position`、`top`、`left`、`right`、`bottom`、`display` 或 `overflow` prop

要找到该 mod，请[写入调试日志](#read-the-debug-log)并在其中搜索 `answered ui.render without next()`。诸如 `other-mod (user) answered ui.render without next()` 的行会指明一个返回了树但未调用 `next(e)` 的 mod。该行不会指明[渲染位置](https://code.claude.com/docs/zh-CN/plugins/mods/reference.md#render-sites)，因此仅绘制窗格的 mod 也可能出现在其中。调用了 `next(e)` 但丢弃其结果的 mod 不会产生该行，因此也请检查在您的 mod 之前运行的 hook。

当某个 mod 自身的 hook 返回的树未通过验证时，该 mod 的带也会缺失，而其他 mod 的带仍会保留。要了解是哪个 mod 以及原因，请[写入调试日志](#read-the-debug-log)并在其中搜索 [`ui.render hook skipped`](#a-pane-or-band-is-empty)。

### 窗格或带为空或显示 Claude Code 的常规内容

您的 hook 返回的[树](https://code.claude.com/docs/zh-CN/plugins/mods/interface.md#build-a-tree-from-elements)未通过验证。[会话记录或调试日志中](#find-out-why-a-mod-does-nothing)的 [`hook skipped`](#hook-skipped) 行会指明您的 mod 及原因，例如以下针对窗格中某个 `Box` 的行：

```text theme={null}
first-mod: ui.render hook skipped: returned the wrong shape (a tree Pane refuses: Box prop "flexDirection" must be one of row, column, row-reverse, column-reverse)
```

请修复原因中指出的问题。如果原因被截断，请[写入调试日志](#read-the-debug-log)并在其中搜索 `a hook returned a tree that does not validate` 以查看其余内容。

如果该行在位置名称之后紧接着显示 `refused:`，如 `ui.render (Pane) refused:`，请参阅[`ui.render` 行显示 `refused`](#ui-render-refused)。

### `ui.render` 行显示 `refused`

`ui.render (<site>) refused:`，或[调试日志](#read-the-debug-log)中的 `ui.render (<site>): the chain settled on a tree that does not validate`，表示该位置的[树](https://code.claude.com/docs/zh-CN/plugins/mods/interface.md#build-a-tree-from-elements)超出了[大小限制](https://code.claude.com/docs/zh-CN/plugins/mods/reference.md#limits)。这两行都会先指明[渲染位置](https://code.claude.com/docs/zh-CN/plugins/mods/reference.md#render-sites)，然后给出原因，原因中会指明该限制。

请修改在该位置绘制的 mod，使其 [`ui.render`](https://code.claude.com/docs/zh-CN/plugins/mods/reference#interface) hook 返回更小的树，或请其作者修改。如果会话记录行以 `hooks` 开头，则表示有多个已加载的 mod 为该位置设置了 `ui.render` hook，因此请检查每个这些 mod 的树。当多个 mod 在输入框上方的带嵌套成深度超过 32 层的树时，该行显示为：

```text theme={null}
hooks: ui.render (AbovePrompt) refused: deeper than 32; nothing was drawn
```

在 v2.1.307 之前，任何位置的 `refused:` 行都会报告任何未通过验证的树，并且对于 `Pane` 和 `AbovePrompt`，该行以 `; the engine drew its own` 结尾。在这些版本中，请修复原因中指出的问题。

### `needs a key`、`is drawn twice` 或 `key each once`

您的 `ui.render` hook 返回的树中，某个元素没有有效的 [`key`](https://code.claude.com/docs/zh-CN/plugins/mods/interface#which-controls-need-a-key)，或两个元素共用同一个 key。请找到您收到的消息：

* **`<Input> needs a key` 或 `<Select> needs a key`**：该元素没有 `key` prop，或其 `key` 为空或不是字符串，如 `key: 3`。请为其提供一个非空字符串作为 `key`。
* **`<Button> needs a key`**：为该按钮提供一个非空字符串作为其 `key`
* **`is drawn twice; each takes its own key`**：消息所指明类型的两个元素具有相同的 `key`，如 `Input "size" is drawn twice; each takes its own key`。请为每个元素提供各自的 `key`。
* **[测试](https://code.claude.com/docs/zh-CN/plugins/mods/test.md#test-a-drawing)中 `press` 报告的 `key each once`**：绘图中有多个按钮使用您传入的 key。标签相同且没有 `key` prop 的按钮会共用该标签作为其 key。请为每个按钮提供各自的 `key`。

### `ui.render` 行显示 `threw while drawn`

该行会指明[渲染位置](https://code.claude.com/docs/zh-CN/plugins/mods/reference.md#render-sites)，然后显示 `threw while drawn:` 及错误，如 `first-mod: ui.render (ToolUse) threw while drawn: <error>; the engine drew its own`。Claude Code 在绘制您的 [`ui.render`](https://code.claude.com/docs/zh-CN/plugins/mods/reference#interface) hook 返回的树时，或在根据[您的 hook 传递给 `next` 的 `props`](https://code.claude.com/docs/zh-CN/plugins/mods/interface#change-what-claude-code-already-draws) 绘制该位置时，遇到了该错误。结尾的 `the engine drew its own` 表示该位置显示 Claude Code 的常规内容。

读取该错误，并修复您的 hook 中导致该错误的值。

在 v2.1.289 之前，会话记录行中出现此错误会导致会话以 [`Claude Code exited after an unrecoverable interface error`](https://code.claude.com/docs/zh-CN/errors#exited-after-an-unrecoverable-interface-error) 结束。

### `the module failed without a message`

某个 [`Client`](https://code.claude.com/docs/zh-CN/plugins/mods/interface#when-a-client-fails) 因一个没有消息的错误而失败，例如 `throw new Error()`。在其位置显示的行类似 `my-mod: Client client/spinner.js: the module failed without a message`。

在您的 `Client` 代码中找到该 throw，并为该错误提供消息。该行随后会显示该消息。

在 v2.1.289 之前，该行改为显示 `Error` 作为原因。

### `$.ui.open` 运行且没有窗格出现

调用不是来自用户做的事情，且终端宽度小于[该窗格所需的宽度](https://code.claude.com/docs/zh-CN/plugins/mods/interface.md#when-a-pane-waits-for-a-wider-terminal)。

从命令或按钮打开窗格，或检查调用的 `isPlaced` 结果。请参阅 [在正确的时间打开窗格](https://code.claude.com/docs/zh-CN/plugins/mods/interface.md#open-a-pane-at-the-right-time)。

### toast 不出现

您的 mod 在交互式终端会话中调用了 [`$.ui.toast`](https://code.claude.com/docs/zh-CN/plugins/mods/api#show-something-without-starting-a-turn)，但您没有看到该 toast。要确认调用已运行，请在[调试日志](#read-the-debug-log)中查找包含您的 mod 名称和 toast 文本的行，如 `$.ui.toast (first-mod): build finished`。然后检查以下原因：

* **缺少该调用对应的行**：查找说明 Claude Code 拒绝该调用原因的行，如 `first-mod: $.ui.toast dropped: timeoutMs is a whole number of ms, 1 to 60000`。
* **某个窗格正在暂缓 toast**：您的 mod 或其他 mod 在打开当前显示的窗格时传递了 [`holdToasts`](https://code.claude.com/docs/zh-CN/plugins/mods/interface#hold-toasts-behind-a-dialog)。关闭该窗格即可结束暂缓。如果该窗格是您的且需要保持打开，请从其 `$.ui.open` 调用中移除 `holdToasts`，然后重新打开该窗格。
* **toast 位于输入框下方**：在[经典渲染器](https://code.claude.com/docs/zh-CN/fullscreen.md#enable-fullscreen-rendering)中，请查看输入框下方的右侧。该处的 toast 是以 mod 名称开头的一行文字，而不是右上角的方框。
* **您的 mod 发出了更新的 toast**：在经典渲染器中，来自您的 mod 的更新 toast 可能会取代正在显示或等待显示的 toast。调试日志中会有针对较早 toast 的另一行：如果该 toast 当时正在显示，该行以 `gave way, cut short` 结尾；如果它从未出现，则以 `gave way, unseen` 结尾。要同时显示两条消息，请将它们放在一个 toast 中。
* **toast 在绘制前超时**：在全屏渲染中，Claude Code 一次最多绘制三个 toast，因此 toast 可能在绘制之前就已超时。调试日志中会有针对该 toast 的另一行，以 `left the stack, never drawn` 结尾。当您的 mod 同时发出多个 toast 时，请将这些消息放在一个 toast 中。

在 v2.1.290 之前，如果某个 toast 是在 Claude Code 为您的 mod 显示上一个 toast 后两秒内发出的，Claude Code 会丢弃该 toast，且调试日志中针对被丢弃 toast 的行会显示 `within 2000ms of the last; dropped`。

### 带显示 `plugin panel hidden`

该带已[折叠](https://code.claude.com/docs/zh-CN/plugins/mods/interface.md#when-the-user-collapses-the-band)。当您按 Ctrl+X 然后 Ctrl+A（或您绑定以替代它们的按键）时，以及在[全屏渲染](https://code.claude.com/docs/zh-CN/fullscreen.md)中单击其第一行的 `[-]` 时，该带会折叠。它会保持折叠状态，直到您将其展开或再次启动 Claude Code；在此期间，任何热键或 `action` 绑定都不会按下其中的按钮。

要展开该带，请按该行所示的按键（默认为 Ctrl+X 然后 Ctrl+A），或在全屏渲染中单击该行。

### 热键不起作用

您的窗格或带没有键盘焦点、您的按钮位于已折叠的带中，或者绘图位于 VS Code 扩展中，而该扩展中的按钮没有热键。请按对应的情况操作：

* **没有键盘焦点**：按 Ctrl+X 然后 Tab，或在[全屏渲染](https://code.claude.com/docs/zh-CN/fullscreen.md)中单击窗格或带。要让窗格在打开时获得焦点，请从命令中使用 `focus: true` 打开它。
* **按钮位于已折叠的带中**：该带显示一个以 `▸ plugin panel hidden` 开头的暗色行。按该行所示的按键（默认为 Ctrl+X 然后 Ctrl+A）即可[展开它](#plugin-panel-hidden)。
* **在 VS Code 扩展中**：单击按钮，或在按钮获得焦点后按 Enter

### 绘图在终端中有效，在桌面应用或 VS Code 中无效

要么该应用不绘制该位置或元素，要么该会话属于 VS Code 扩展不为 mod 绘制任何内容的会话。该 mod 的其他 hook 仍会运行。请检查以下原因：

* **该位置或元素在该应用中不会被绘制**：在[渲染位置](https://code.claude.com/docs/zh-CN/plugins/mods/reference.md#render-sites)表中查找您的位置，并在[元素](https://code.claude.com/docs/zh-CN/plugins/mods/reference.md#elements)表中查找您的元素。对于该应用不具备的元素，请在 hook 中检查 `e.surface`，并返回该应用可以绘制的树。
* **VS Code 扩展版本低于 2.1.299**：从[扩展版本](https://code.claude.com/docs/zh-CN/vs-code.md#install-the-extension) 2.1.299 起，聊天面板才会为 mod 进行绘制。请在 VS Code 的扩展视图中检查版本，并在该处更新扩展。
* **扩展版本低于 2.1.304，且该会话不获取功能标志**：请更新扩展，或改为在 VS Code 的集成终端中运行 `claude`。在 Amazon Bedrock、Google Cloud 的 Agent Platform 或 Microsoft Foundry 上的会话会跳过此获取步骤。对于跳过此步骤的其他会话，例如 [Claude apps 网关](https://code.claude.com/docs/zh-CN/claude-apps-gateway)会话，请参阅[需要获取功能标志的功能](https://code.claude.com/docs/zh-CN/env-vars.md#features-that-need-feature-flag-fetching)。

## 编辑或值丢失

mod 运行，您所做的更改或它保留的值不存在。

### 您的编辑不生效

您正在编辑您安装的插件。Claude Code 运行已安装版本的缓存副本。

使用指向您的工作副本的 `--plugin-dir` 进行开发，如 `claude --plugin-dir ./first-mod`，它在您保存时重新加载。

### 模块重新加载时值重置

模块级变量在每次重新加载时重新初始化。

[将值保留在 `$.state` 或 `$.store` 中](https://code.claude.com/docs/zh-CN/plugins/mods/interface#keep-state)。

### 值在 `/clear`、`/resume` 或 `/branch` 后重置

值重置，或保存的值被其默认值替换。这些命令中的每一个都将 `$.state` 重置为其默认值，`session.start` 不再触发。

[在 `classic.SessionStart` hook 中再次加载保存的值](https://code.claude.com/docs/zh-CN/plugins/mods/interface#load-a-saved-value-again-after-clear)。

### `settings.read` hook 返回的某个键不生效

如果您在 [Claude Code 自身的读取](https://code.claude.com/docs/zh-CN/plugins/mods/events.md#settings-read-hook)中返回权限规则或 `env`，这些规则不会在工具调用时被检查，`env` 条目也不会成为环境变量。对这两者请使用[设置文件](https://code.claude.com/docs/zh-CN/settings.md)，或对 `env` 使用 [`$.env.set`](https://code.claude.com/docs/zh-CN/plugins/mods/api#reach-files-processes-and-the-network)。

对于其他键，请查看[哪些 mod 会收到 `settings.read` 以及从哪个版本开始](https://code.claude.com/docs/zh-CN/plugins/mods/events#settings-read-hook)，然后在[调试日志](#read-the-debug-log)中搜索 `settings.read`，并查找以下某一行：

| 日志行 | 含义和步骤 |
| :- | :- |
| `settings.read hook skipped`，位于您的 mod 名称之后 | 您的 hook 失败。[修复该行报告的错误](#hook-skipped)。 |
| `settings.read bypassed by cc-plugin-sec-default`，所在行在其前面列出了您的 mod | [内置守卫](https://code.claude.com/docs/zh-CN/plugins/mods/admin.md#know-what-happens-by-default)跳过了您的 hook。请让您组织的管理员[将该 mod 安装为组织的 mod 之一](https://code.claude.com/docs/zh-CN/plugins/mods/admin.md#install-your-organizations-mods)。 |
| `settings.read: the answer for <key> is no value the key can hold` | Claude Code 拒绝了某个 hook 为该键返回的值。该 hook 可能是您的，也可能是其他 mod 的。请按照[适用于您情况的步骤](#settings-read-key-refused)操作。 |
| `settings.read: the merge never settles for <key>` | 该行列出的键保留设置来源中的值。请在设置文件中设置它们。 |
| `answered settings.read without next()`，位于某个 mod 名称之后 | 如果所列 mod [在您的 mod 之前运行](https://code.claude.com/docs/zh-CN/plugins/mods/events.md#the-order-mods-run-in)，则您的 hook 会受到影响。此时您的 hook 在该次读取中未运行。[关闭该 mod](https://code.claude.com/docs/zh-CN/plugins/mods/overview.md#turn-mods-on-or-off)，或者如果它是由您的组织部署的，请让其管理员关闭。 |
| `settings.read: refused to the engine` | 您的值未生效，因为 Claude Code 自身的读取被拒绝。[找出拒绝它的原因](#settings-read-refused)。 |

如果没有任何一行能解释该键，请检查在您的 mod 之前运行的 mod 中是否有[更改该键](https://code.claude.com/docs/zh-CN/plugins/mods/events.md#settings-read-example)的 `settings.read` hook。关闭该 mod，或让您组织的管理员关闭。如果没有 hook 更改该键，请在设置文件中设置该键。

### `the answer for <key> is no value the key can hold`

此[调试日志](#read-the-debug-log)行表示 Claude Code 拒绝了某个 [`settings.read`](https://code.claude.com/docs/zh-CN/plugins/mods/events#settings-read-hook) hook 为所列键返回的值，且仅针对该键：

```text theme={null}
settings.read: the answer for <key> is no value the key can hold; the settings' own value stands
```

为所列键返回的值不符合该键的 schema，例如类型或范围不符。

* **您的值不符合该键的 schema**：请返回该键的[设置参考](https://code.claude.com/docs/zh-CN/settings-reference.md)条目所允许的值。
* **您的值符合要求**：不符合要求的值来自其他 mod 的 `settings.read` hook。要找出该 mod，请按照[逐个关闭 mod 的步骤](#settings-read-refused)操作。在每个新日志中搜索此行，而不是 `refused to the engine`。

在该值不符合要求期间，该键保留在任何 hook 应答之前从设置来源合并得到的值。该行将其称为设置自身的值。

### `settings.read: refused to the engine`

当 [Claude Code 自身的读取](https://code.claude.com/docs/zh-CN/plugins/mods/events.md#settings-read-hook)被拒绝时，您会在[调试日志](#read-the-debug-log)中看到此行，此时 Claude Code 会丢弃所有 hook 的值。当某个 `settings.read` hook 使用 [`{ deny: reason }`](https://code.claude.com/docs/zh-CN/plugins/mods/events#settings-read-returns) 拒绝该读取时，该行会带有该 hook 的原因：

```text theme={null}
settings.read: refused to the engine (engine: $.settings.read: <reason>); the settings' own values stand
```

在您的代码中搜索括号中的原因：

* **您的代码中有该文本**：您的 hook 拒绝了读取。当 [`next.origin`](https://code.claude.com/docs/zh-CN/plugins/mods/events#settings-read-origin) 显示为 Claude Code 自身的读取时，让它返回 `{ value }`。
* **您的代码中没有该原因**：逐个[关闭](https://code.claude.com/docs/zh-CN/plugins/mods/overview.md#turn-mods-on-or-off)您的其他 mod。如果某个 mod 是由您的组织部署的，请让其管理员将其关闭。每关闭一个后：
  1. 从 shell 使用 `claude --debug-file ./settings-read-2.log` 启动会话，后接会话启动时所用的标志。每次使用一个新路径。
  2. 在该文件中搜索 `refused to the engine`。如果该行存在，则关闭下一个 mod。如果不存在，则说明您最后关闭的 mod 拒绝了读取，请保持其关闭状态。

如果已没有其他 mod 而该行仍然存在，请在[设置文件](https://code.claude.com/docs/zh-CN/settings.md)中设置您的 hook 返回的键。在读取被拒绝期间，其中的值仍然有效。

## 阅读调试日志

调试日志对 Claude Code 加载或拒绝的每个模块、每个失败的 hook 以及它拒绝的每个结果都有一行，因此当会话记录中没有任何显示时，这是查看的地方。要写入一个，在您的 shell 中使用 `--debug` 启动 Claude Code，或使用 `--debug-file <path>` 选择它的位置：

```bash theme={null}
claude --debug-file ./mod-debug.log --plugin-dir ./first-mod
```

在另一个终端中，跟踪文件并按您的 mod 名称过滤：

```bash theme={null}
tail -f ./mod-debug.log | grep first-mod
```

已加载的 mod 有一行命名它并列出它处理的事件。使用 `--plugin-dir` 加载的 mod 出现在其名称后跟 `@inline` 下：

```text theme={null}
hooks module first-mod@inline loaded (worker, environment 2, tier user); events: session.start,tool.call,command.run,ui.render
```

要在日志中写入您自己的行，请调用 [`$.ui.log`](https://code.claude.com/docs/zh-CN/plugins/mods/api#show-something-without-starting-a-turn)，带有第二个参数，如 `$.ui.log('message', { to: 'debug' })`。没有第二个参数，`$.ui.log` 会在会话记录中添加一条暗行。

当您编辑使用 `--plugin-dir` 加载的 mod 时，会话记录为每次重新加载显示一行，命名 mod 并列出其 hook。如果保存破坏了模块，该行会显示 `reload failed, the previous version stays loaded:` 及原因，最后一个可用版本会继续运行，直到 Claude Code 下次重新加载插件，例如当您运行 `/reload-plugins` 时。

## 后续步骤

* [测试 mod](https://code.claude.com/docs/zh-CN/plugins/mods/test.md)：在问题到达会话之前捕获它们
* [排查插件问题](https://code.claude.com/docs/zh-CN/plugins/troubleshooting.md)：与安装和加载不特定于 mod 的插件相关的问题
