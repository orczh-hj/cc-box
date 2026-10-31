> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# 为您的组织管理 mods

> 使用托管设置控制 Claude Code mods：停止用户安装的 mods、仅允许您自己的 mods、查看 mod 可以执行的操作，以及使用您自己的 mod 强制执行策略。

[mod](https://code.claude.com/docs/zh-CN/plugins/mods/overview.md) 是在 Claude Code 内运行代码的插件，具有安装它的用户的权限。Mods 不是沙箱化的。通过[托管设置](https://code.claude.com/docs/zh-CN/managed-settings.md)，您可以决定 mods 是否在用户的机器上运行、运行哪些 mods 以及运行顺序。您还可以安装自己的 mod，用于监视或拒绝其他 mods 的操作。

本页面适用于为 Claude Code 部署托管设置的人员，无论是通过文件、MDM 还是从 claude.ai 管理控制台部署。在 Claude Code v2.1.286 及更高版本中，mod 默认处于启用状态。从与您要执行的操作相匹配的部分开始：

* **排除用户自己的 mods，有或没有您自己的 mods**：[停止用户安装的 mods 加载](#stop-user-installed-mods-from-loading)
* **查看当您不做任何更改时用户会获得什么**：[了解默认情况下会发生什么](#know-what-happens-by-default)
* **保持 mods 开启并设置其他限制**：[选择允许的程度](#choose-how-much-to-allow)

> [!NOTE]
>
> 这些情况在其他页面上有介绍：
>
> * **您之前没有部署过托管设置**：从[部署托管设置](https://code.claude.com/docs/zh-CN/managed-settings.md)开始
> * **您想控制用户可以安装哪些插件**：请参阅[为您的组织管理插件](https://code.claude.com/docs/zh-CN/plugins/org.md)

## 停止用户安装的 mod 加载

要阻止用户带来的每个 mod 运行其 hook，请在[内置守卫](#know-what-happens-by-default)上设置 `allowManagedModsOnly` 选项。内置守卫是一个策略 mod，Claude Code 会在用户安装的每个 mod 之前加载它。该选项位于托管设置的 `pluginConfigs` 下，以 `cc-plugin-sec-default@builtin` 为键：

```json managed-settings.json theme={null}
{
  "pluginConfigs": {
    "cc-plugin-sec-default@builtin": {
      "options": {
        "allowManagedModsOnly": true
      }
    }
  }
}
```

在托管设置中设置该选项后：

* **用户带来的任何 mod 都不会运行其 hook**：这包括用户安装的插件中的 mod、使用 `--plugin-dir` 加载的 mod，以及[Claude 在会话期间编写的](https://code.claude.com/docs/zh-CN/plugins/mods/create.md#ask-claude-for-a-mod) mod
* **您组织的 mod 仍然运行**：[计为您组织的](#install-your-organizations-mods) mod 不会被检查。所有其他 mod 都计为用户的 mod，并会被拒绝。这包括您从 GitHub 或其他远程市场启用的插件中的 mod，以及您的组织为其成员在 claude.ai 上启用的 mod。如果没有任何 mod 计为您组织的，则所有已安装的 mod 都不会运行其 hook。
* **用户无法撤销它**：守卫仅从托管设置读取该选项，因此用户、项目或本地设置文件中的相同条目，或使用 `--settings` 传递的文件中的条目都不会改变任何内容
* **文件或 MDM 策略涵盖每个提供商**：当您以文件形式或通过 MDM 提供该选项时，它在 Amazon Bedrock、Google Cloud 的 Agent Platform 和 Microsoft Foundry 上的工作方式相同。对于从 claude.ai 管理控制台进行的交付，请参阅[平台可用性](https://code.claude.com/docs/zh-CN/server-managed-settings.md#platform-availability)
* **用户的其他自定义保持工作**：他们[设置文件中的 hook](https://code.claude.com/docs/zh-CN/hooks.md) 和插件 `hooks/hooks.json` 中的 hook、状态栏以及 `/goal` 均不受影响
* **内置 mod 继续运行**：内置于 Claude Code 的 mod，例如 `AGENTS.md` 支持，各有[自己的开关](https://code.claude.com/docs/zh-CN/plugins/mods/overview.md#mods-built-into-claude-code)

要在用户机器上确认该选项，请在该机器上使用 `--plugin-dir` 和包含 mod 的目录路径启动 Claude Code，例如 `claude --plugin-dir ./first-mod`。该 mod 的 hook 不会运行，会话记录和调试日志中会出现[守卫的消息](https://code.claude.com/docs/zh-CN/plugins/mods/troubleshoot.md#messages-from-the-built-in-guard)，其中会指明该 mod 和 `allowManagedModsOnly`。如果没有出现该消息，请参阅[检查策略是否生效](https://code.claude.com/docs/zh-CN/managed-settings.md#check-that-a-policy-is-in-force)以及[决定选项是否生效的规则](#set-options-on-the-built-in-guard)。

如果您在早期访问期间将 `CLAUDE_CODE_ENABLE_FUNCTION_HOOKS` 设置为 `0`，请将其替换为此选项。Claude Code v2.1.287 及更高版本在任何值下都会忽略该变量，因此那里的 `0` 会使 mod 保持开启。

## 了解默认情况下会发生什么

如果您没有自己的 mod 设置，这就是您的用户会获得的：

* **Mods 处于开启状态。** 用户可以安装包含来自您的插件设置允许的任何市场的 mod 的插件，或使用 `--plugin-dir` 从目录加载一个。
* **内置守卫首先运行。** Claude Code 在用户安装的每个 mod 之前加载一个名为 `sec-default@builtin` 的内置 mod。用户无法将其关闭。`/plugin` 和调试日志将其列为 `cc-plugin-sec-default`。内置守卫在以下任一情况为真时加载：

  * 机器有托管设置
  * 会话使用 Team 或 Enterprise 计划的 [`/login` 凭据](https://code.claude.com/docs/zh-CN/authentication#authentication-precedence)运行

  使用 API 密钥进行身份验证的用户，或通过 Amazon Bedrock、Google Cloud 的 Agent Platform 或 Microsoft Foundry 进行身份验证的用户，仅在具有托管设置的机器上获得内置守卫。使用长期有效的 [`CLAUDE_CODE_OAUTH_TOKEN`](https://code.claude.com/docs/zh-CN/authentication#generate-a-long-lived-token) 运行的会话也不算作上述登录，即使机器上存储了 Team 或 Enterprise 登录信息。
* **保护程序保护您管理的内容。** 用户的 mod 无法更改您的托管 hooks 接收或决定的内容、系统提示、您的托管 `CLAUDE.md` 和其他托管说明、任何 mod 读取的设置内容，或您的托管 MCP 服务器的工具和描述。
* **在您管理的内容之外，用户的 mod 以该用户身份行事。** mod 保留该用户的权限：它仍然可以读写文件、启动进程和发出网络请求。它还可以重写工具调用和提示词、拒绝工具调用，以及在界面中绘制。
* **权限规则和 `PreToolUse` hook 优先于用户的 mod。** 加载内置守卫后，默认情况下，用户的 mod 无法批准 `deny` 规则拒绝的调用或 `ask` 规则会提示的调用，无论哪个设置文件持有该规则。mod 也无法批准任何 `PreToolUse` hook 阻止的调用。

  权限规则和 `PreToolUse` hook 适用于 Claude 的工具调用，而不适用于 mod 的 [`$.fs` 和 `$.process` 调用](https://code.claude.com/docs/zh-CN/plugins/mods/api#reach-files-processes-and-the-network)。即使 `Read(.env)` 被拒绝，mod 仍然可以使用 `$.fs.read` 读取该文件，或启动执行此操作的程序。要限制 mod 自身的 `$.fs` 和 `$.process` 调用，请[阻止该 mod 加载](#stop-user-installed-mods-from-loading)，或在[策略 mod](#enforce-a-policy-with-a-mod-of-your-own) 中处理这些调用。
* **用户的 mod 仍然可以跳过某些提示。** 在[自动模式](https://code.claude.com/docs/zh-CN/permission-modes.md#eliminate-prompts-with-auto-mode)下，mod 批准的调用运行时不进行分类器检查，除非内置守卫已加载，且会话使用 [`--permission-prompt-tool`](https://code.claude.com/docs/zh-CN/cli-reference#cli-flags) 或 Agent SDK [`canUseTool` 回调](https://code.claude.com/docs/zh-CN/agent-sdk/user-input#handle-tool-approval-requests)运行。在这种情况下，分类器会先检查该调用。对于 mod 的批准不会跳过的提示，请参阅[使用 hook 扩展权限](https://code.claude.com/docs/zh-CN/permissions.md#precedence-among-rules-hooks-and-mods)。

保护程序的源代码在 [Claude Code 存储库的 `mods/sec-default` 目录](https://github.com/anthropics/claude-code/tree/main/mods/sec-default)中是公开的。

### 了解哪些控制仍然适用

Mods 不会替换您已有的控制：

* **设置 hook 继续工作。** 设置文件和插件的 `hooks/hooks.json` 中的命令、HTTP、提示词和 Agent hook 照常运行，与 mod 一起。
* **拒绝规则在保护程序加载的地方优先。** 用户的 mod 无法批准 `deny` 规则拒绝的调用，除非您设置 [`allowModsToOverrideDenyRules`](#set-options-on-the-built-in-guard)。
* **托管 hook 首先运行。** 托管设置中的 `PreToolUse` hook 在任何 mod 的 [`tool.call`](https://code.claude.com/docs/zh-CN/plugins/mods/reference#tools) hook 之前运行，当该托管 hook 阻止某个工具调用时，不会有任何 mod 的 `tool.call` hook 针对该调用运行。如果调用继续进行且 mod 重写了它，您的托管 hook 会在重写后的调用上再次运行，因此仍然可以阻止它。

  来自其他设置文件和插件的 `PreToolUse` hook 在最后一个 mod 之后运行。如果 mod 自行应答工具调用，该工具永远不会运行，这些 hook 也不会运行。请参阅[mod 运行的顺序](https://code.claude.com/docs/zh-CN/plugins/mods/events.md#the-order-mods-run-in)。
* **网络策略涵盖 `$.http.fetch`。**

  * **您的组织策略不允许 WebFetch**：Claude Code 也会拒绝每个 mod 的 `$.http.fetch` 请求。请参阅 [WebFetch 可用性](https://code.claude.com/docs/zh-CN/tools-reference.md#webfetch-availability)。
  * **您在托管设置或任何其他设置文件中使用 [`CLAUDE_CODE_DISABLE_WEB_FETCH`](https://code.claude.com/docs/zh-CN/env-vars#variables) 或 `WebFetch` [拒绝规则](https://code.claude.com/docs/zh-CN/permissions.md#tool-specific-permission-rules)关闭了 WebFetch 工具**：mod 仍然可以发出 `$.http.fetch` 请求。

    要为您的用户安装的每个 mod 阻止这些请求，请[防止用户安装的 mod 加载](#stop-user-installed-mods-from-loading)，或在[策略 mod](#enforce-a-policy-with-a-mod-of-your-own) 中 hook `http.fetch` 并拒绝来自用户 mod 的请求。还要为该 hook 提供一个同样会拒绝请求的 [`.catch` 处理程序](https://code.claude.com/docs/zh-CN/plugins/mods/events#handle-a-hook-that-fails)，因为 Claude Code 会跳过失败的 hook，届时请求将继续进行。如果您只[限制用户可以安装的插件](https://code.claude.com/docs/zh-CN/plugins/org.md#restrict-what-users-can-install)，加载的 mod 会减少，但您允许的 mod 仍然可以发出这些请求。
  * **您设置了 [`CLAUDE_CODE_DISABLE_NONESSENTIAL_TRAFFIC`](https://code.claude.com/docs/zh-CN/env-vars)**：您或您的用户安装的 mod 仍然可以发出这些请求。该变量仅阻止 [Claude Code 内置的 mod](https://code.claude.com/docs/zh-CN/plugins/mods/overview.md#mods-built-into-claude-code)，以及任何携带会话 Anthropic 凭据的 `$.http.fetch` 请求。在 v2.1.288 之前，该变量会阻止每个 mod 的 `$.http.fetch` 请求。

  这些都不涵盖 mod 使用 `$.process.run` 启动的程序，该程序使用用户自己的访问权限访问网络。
* **插件控制涵盖 mods。** Mod 是一个插件，因此[限制用户可以安装的内容的设置](https://code.claude.com/docs/zh-CN/plugins/org.md#restrict-what-users-can-install)，例如 `strictKnownMarketplaces`，决定它是否可以被安装。
* **Mods 无法更改权限提示。** Mod 可以重新设置 Claude Code 界面的大部分样式，但不能更改权限提示，因此它无法更改提示显示的内容。Mod 仍然可以在提示出现之前批准或拒绝工具调用，如[了解默认情况下会发生什么](#know-what-happens-by-default)所述。
* **信任提示首先出现。** 在用户尚未信任的目录中的交互式会话中，在他们回答信任提示之前，没有 mod 加载。
* **`--safe-mode` 关闭已安装的 mods，包括您的。** 使用 `claude --safe-mode` 启动会话以检查 mod 是否导致了问题。

这些控制中的任何一个都不会沙箱化 mod。您允许的 mod 以用户身份运行，具有用户对文件、进程和网络的访问权限。

### 了解用户 mod 的哪些 hook 不会运行

在[内置守卫](#know-what-happens-by-default)加载的地方，用户的 mod 在以下事件上注册的 hook 永远不会运行，`/plugin` 会通过一条说明告知该用户。在 [`session.append`](https://code.claude.com/docs/zh-CN/plugins/mods/reference#session) 各行中，`door` 是该 hook 所筛选的存储行的类型。

| hook 所在事件 | 事件是什么 |
| :- | :- |
| [`classic.<Event>`](https://code.claude.com/docs/zh-CN/plugins/mods/events#hook-the-settings-hook-events)，例如 `classic.Stop` | 一个[设置 hook 事件](https://code.claude.com/docs/zh-CN/hooks.md#hook-events) |
| [`prompt.compose`](https://code.claude.com/docs/zh-CN/plugins/mods/reference#prompts-and-what-claude-reads) | 系统提示词的章节列表 |
| `prompt.section` | 系统提示词的一个章节 |
| `prompt.context` | 随对话第一条消息一起发送的上下文 |
| `prompt.attachment` | Claude Code 为 Claude 添加的消息，例如提醒 |
| `skill.prompt` | 为 Claude 展开时的 skill 文本 |
| `attribution.text` | 提交或 Pull Request 的署名文本 |
| [`session.compact`](https://code.claude.com/docs/zh-CN/plugins/mods/reference#session) | 对话的一次压缩 |
| `settings.read` | 一次设置读取，例如 mod 的 `$.settings.read` 调用 |
| `session.append{door=attachment}` | Claude Code 为 Claude 添加的消息的存储行 |
| `session.append{door=hook-context}` | 来自设置 hook 的上下文的存储行 |
| `session.append{door=compaction}` | 压缩摘要和边界的存储行 |

在其他一些情况下，守卫也会跳过用户的 hook，例如 [`tool.describe`](https://code.claude.com/docs/zh-CN/plugins/mods/reference#tools) 上您组织的工具描述，以及 skill 文本的存储行。对于这些情况，`/plugin` 不会显示说明。如果其他事件上的 hook 没有任何作用，请参阅[查明 mod 为何没有任何作用](https://code.claude.com/docs/zh-CN/plugins/mods/troubleshoot.md#find-out-why-a-mod-does-nothing)。

该说明显示在插件的详细信息中以及 `/plugin` 的 **Errors** 选项卡中。它需要 Claude Code v2.1.299 或更高版本。它会按照 mod 注册时的写法列出表中的每个 hook，如以下示例所示：

```text theme={null}
Its hooks on classic.Stop, prompt.section do not run here: where an organization manages Claude Code, these events pass over the plugins you install
Only a plugin that counts as your organization's own runs hooks on these events. An administrator sets that up in managed settings
```

要让这些 hook 运行，请通过以下两种方式之一更改托管设置：

* **将该 mod 作为您组织的 mod 运行**：[安装该 mod，使其算作您组织自己的 mod](#install-your-organizations-mods)。这样它在这些事件上的 hook 就会运行，而守卫对其他所有 mod 仍保持加载。该 mod 会为接收这些设置的每个人开启，并且必须位于您放置在每台机器上的目录中。如果您将其列在 `prependPlugins` 中，请在列表中保留 `sec-default@builtin`。
* **移除守卫**：在 `prependPlugins` 中省略 `sec-default@builtin`。一个空列表 `"prependPlugins": []` 就足够了。您失去的是整个守卫，而不仅仅是这些事件。这样用户的 mod 就可以做[守卫原本会阻止它做的事情](#know-what-happens-by-default)，并且[守卫的选项](#set-options-on-the-built-in-guard)也不再适用。

如果您通过[服务器托管设置](https://code.claude.com/docs/zh-CN/server-managed-settings.md#fetch-and-caching-behavior)下发不含守卫的列表，守卫会保持加载，直到服务器为该会话确认这些设置。要在运行 v2.1.299 或更高版本的机器上确认任一更改，请在该机器上的会话中打开 `/plugin`，并检查该说明是否已消失。

没有 `door` 筛选条件的 `session.append` hook 仍会运行且不会收到说明，但表中的存储行不会传递给它。

## 决定是否保持 mods 开启

Mod 可以做的比插件的其他部分更多，因为它在 Claude Code 内运行。它看到每个提示和工具调用，可以更改它们，并可以在权限提示出现之前允许或拒绝工具调用。

用户可以加载什么作为 mod 取决于您已有的插件控制：

| 您今天的插件控制 | 用户可以加载什么作为 mod |
| :- | :- |
| 无 | 来自任何市场的 mod、来自任何带有 `--plugin-dir` 的目录的 mod，或 Claude 在会话期间编写的 mod |
| 市场允许列表 | 来自您允许的市场的 mod，或来自任何带有 `--plugin-dir` 的目录的 mod。Claude 在会话期间编写的 mod 仅在允许列表[包含 `skills-dir`](https://code.claude.com/docs/zh-CN/plugins/org#keep-skills-directory-plugins-loading) 时加载。 |
| 市场允许列表和 `disableSideloadFlags` | 来自您允许的市场的 mod |

[为您的组织管理插件](https://code.claude.com/docs/zh-CN/plugins/org.md)列出了插件加载的每种方式以及控制每种方式的设置。

要在用户安装市场中的 mods 之前检查它们，请参阅[查看 mod 可以执行的操作](#review-what-a-mod-can-do)。要在您完成此操作之前排除用户的 mods，请参阅[停止用户安装的 mods 加载](#stop-user-installed-mods-from-loading)。

### 查看 mod 可以执行的操作

您可以看到 mod 能够做什么而无需运行它。在您的 shell 中，在插件的目录上运行 `claude plugin validate`：

```bash theme={null}
claude plugin validate ./some-mod
```

输出中的 `hooks:` 和 `calls:` 行描述了 mod 的代码：

```text theme={null}
  ❯ ./register.js hooks: session.start, tool.call, ui.render{component=Pane}
  ❯ ./register.js calls: $.fs.read, $.http.fetch, $.store.set, $.ui.open
```

`hooks:` 行列出了 mod 接收的事件。`calls:` 行列出了其代码调用的 mods API 方法。[mods API](https://code.claude.com/docs/zh-CN/plugins/mods/api.md)（在 mod 的代码中写作 `$`）是 mod 到达文件、进程和网络的方式。Claude Code 拒绝加载以此命令无法读取的方式使用 mods API 的 mod。

查看 `calls:` 行以获取这些：

| 调用 | 它的含义 |
| :- | :- |
| `$.fs.read`, `$.fs.write` | 读取或写入用户可以访问的任何地方的文件 |
| `$.process.run`, `$.process.spawn` | 以用户身份启动程序 |
| `$.http.fetch` | 发出网络请求 |
| `$.env.get`, `$.settings.read` | 读取环境变量和设置，可以保存 API 密钥。输出中的 `env reads:` 行命名每个变量。 |
| `$.env.set` | 为 Claude Code 以及它启动的每个命令和 MCP 服务器设置环境变量，可以改变这些程序运行的内容。`env writes:` 行命名每个变量。 |
| `$.mcp.call` | 在连接的 MCP 服务器上调用工具，在会话的权限规则下 |
| `$.model.complete` | 使用用户的计划或 API 密钥进行模型调用 |
| `$.prompt.submit` | 提交提示，可以将其作为用户自己的话语发送 |
| `$.session.send` | 发送另一个会话、子代理或[队友](https://code.claude.com/docs/zh-CN/agent-teams.md)的 Claude 读取的消息 |

在 `hooks:` 行中，[`tool.call`](https://code.claude.com/docs/zh-CN/plugins/mods/reference#tools) 和 [`prompt.submit`](https://code.claude.com/docs/zh-CN/plugins/mods/reference#prompts-and-what-claude-reads) 意味着 mod 看到每个工具调用和每个提示词，并可以更改它们。[`session.append`](https://code.claude.com/docs/zh-CN/plugins/mods/reference#session) 意味着 mod 可以在存储之前重写对话的每一行，但[防护机制跳过的行](#hooks-the-guard-passes-over)除外。[`ui.render{component=AskUserQuestion}`](https://code.claude.com/docs/zh-CN/plugins/mods/interface#change-what-claude-code-already-draws) 意味着 mod 可以重新绘制 Claude 用来询问用户问题的对话框。`tool.check` 意味着 mod 可以在权限提示出现之前批准或拒绝工具调用。[了解默认情况下会发生什么](#know-what-happens-by-default)列出了您的哪些规则和 hook 优先于其答案。

## 选择允许的程度

Mod 策略的范围从根本没有已安装的 mod 到用户选择的任何 mod，以及由您自己的 mod 检查其他 mod，每一种都只需几个托管设置。在第一列中找到您想要的策略，并设置第二列列出的内容。[部署托管设置](https://code.claude.com/docs/zh-CN/managed-settings.md)介绍了托管设置的存放位置。

| 您想要什么 | 设置 |
| :- | :- |
| 没有已安装的 mod 运行，设置 hook 保持不变 | 设置 [`allowManagedModsOnly`](#set-options-on-the-built-in-guard) 并且不部署您自己的 mod |
| 没有已安装的 mod，也根本没有 hook，包括您的托管 hook | 将 `disableAllHooks` 设置为 `true` |
| 仅您组织的 mod | 设置守卫的 [`allowManagedModsOnly` 选项](#stop-user-installed-mods-from-loading)，并[安装您的 mod](#install-your-organizations-mods) 以便它们计为您的 |
| 来自您批准的市场的任何 mod | 保持您的[市场限制](https://code.claude.com/docs/zh-CN/plugins/org.md#restrict-what-users-can-install)，并将 `disableSideloadFlags` 设置为 `true` |
| 任何 mod，由您自己的 mod 检查其他 mod | [安装您的 mod](#install-your-organizations-mods)，并在 `prependPlugins` 中将它与 `sec-default@builtin` 一起列出 |

每个设置的作用：

* **`allowManagedModsOnly`**：内置守卫上的一个选项。Claude Code 会拒绝用户自己的 mod，因此他们的 hook 都不会运行。用户的设置 hook、状态栏和 `/goal` 继续工作。[停止加载用户安装的 mod](#stop-user-installed-mods-from-loading)列出了它涵盖的内容。
* **`allowManagedHooksOnly`**：范围更广的设置。仅[您组织的 mod](#install-your-organizations-mods) 和内置于 Claude Code 的 mod 会加载。用户自己安装的 mod 不会加载。该设置还会阻止用户自己的设置文件中的 hook。在设置之前，请阅读[`allowManagedHooksOnly` 下运行什么](https://code.claude.com/docs/zh-CN/settings-reference#what-runs-under-allowmanagedhooksonly)。
* **`disableAllHooks`**：范围最广的设置。在托管设置中，它会停止每个已安装插件中的 mod（包括您的），并关闭设置文件中的每个 hook，因此您的托管设置中的 `PreToolUse` hook 不再阻止任何内容。自定义状态栏和 `/goal` 也会停止工作。在设置之前，请阅读[`disableAllHooks`](https://code.claude.com/docs/zh-CN/settings-reference#disableallhooks)。
* **`disableSideloadFlags`**：在启动时拒绝 `--plugin-dir` 和 `--plugin-url`，并阻止 Claude 在会话期间编写的 mod 加载。该设置还会拒绝 `--agents` 和 `--mcp-config`。在设置之前，请阅读[`disableSideloadFlags`](https://code.claude.com/docs/zh-CN/settings-reference#disablesideloadflags)。

内置于 Claude Code 的 mod，例如 `AGENTS.md` 支持，不受这些设置的影响。每个都有[自己的开关](https://code.claude.com/docs/zh-CN/plugins/mods/overview.md#mods-built-into-claude-code)。

如果用户的 mod 被拒绝或未加载，用户可以在其调试日志中找到原因。[拒绝消息](https://code.claude.com/docs/zh-CN/plugins/mods/troubleshoot.md#refusal-messages)列出了 `allowManagedHooksOnly` 和 `disableAllHooks` 对应的日志行，[来自内置守卫的消息](https://code.claude.com/docs/zh-CN/plugins/mods/troubleshoot.md#messages-from-the-built-in-guard)包含 `allowManagedModsOnly` 对应的日志行。

### 仅允许您组织的 mod

要运行您组织的 mod 并阻止用户带来的 mod，请部署[策略表](#choose-how-much-to-allow)中 **仅您组织的 mod** 一行的设置，再加上 `disableSideloadFlags`。使用以下完整的 `managed-settings.json`，Claude Code 会拒绝用户自己的 mod，因此他们的 hook 都不会运行，而您的策略 mod 会先于其他 mod 运行：

```json managed-settings.json theme={null}
{
  "extraKnownMarketplaces": {
    "acme-tools": {
      "source": { "source": "directory", "path": "/opt/acme/claude-plugins" }
    }
  },
  "enabledPlugins": { "acme-guard@acme-tools": true },
  "prependPlugins": ["acme-guard@acme-tools", "sec-default@builtin"],
  "pluginConfigs": {
    "cc-plugin-sec-default@builtin": {
      "options": { "allowManagedModsOnly": true }
    }
  },
  "disableSideloadFlags": true
}
```

每组键各负责一项工作：

* **`extraKnownMarketplaces`、`enabledPlugins` 和 `prependPlugins`**：安装您的 mod 以便它计为您的，并让它首先运行，守卫紧随其后。[安装您组织的 mod 并设置顺序](#install-your-organizations-mods)介绍了这些键所指向的目录。
* **`pluginConfigs`**：设置守卫的 `allowManagedModsOnly` 选项，因此 Claude Code 会拒绝用户自己的 mod。他们的设置 hook、状态栏和 `/goal` 继续工作。
* **`disableSideloadFlags`**：有关它在启动时拒绝的标志，请参阅 [`disableSideloadFlags`](https://code.claude.com/docs/zh-CN/settings-reference#disablesideloadflags)

要在测试机器上确认该策略，请在您的 shell 中使用 `claude --debug` 启动会话并阅读调试日志：

* **您的 mod**：其 `hooks module` 行带有 `tier prepend`
* **用户安装的 mod**：有一行显示 `refused by cc-plugin-sec-default: mods are limited to your organization's by policy (allowManagedModsOnly)`。更早的一行会显示该 mod 的 hook 模块已 `loaded`，因此请查找拒绝信息。
* **插件目录**：`claude --plugin-dir ./any-mod` 会退出，并显示以 `--plugin-dir is disabled by your organization's managed settings (disableSideloadFlags)` 开头的消息

要同时限制用户可以添加哪些市场，请将此文件与您的[市场限制](https://code.claude.com/docs/zh-CN/plugins/org.md#restrict-what-users-can-install)结合使用。

### 将您的插件控制应用于 mod

mod 就是插件，因此您[为组织管理插件](https://code.claude.com/docs/zh-CN/plugins/org.md)的方式同样适用于包含 mod 的插件：

* **查看整个设备群中加载了哪些插件**：[审计和审查](https://code.claude.com/docs/zh-CN/plugins/org.md#audit-and-review)
* **决定您审查过的插件何时可以更新**：[设置更新策略](https://code.claude.com/docs/zh-CN/plugins/org.md#set-update-policy)
* **为某个群组（例如试点群组）提供不同的策略**：[为托管设置无法强制执行的内容做好规划](https://code.claude.com/docs/zh-CN/plugins/org.md#plan-for-what-managed-settings-can’t-enforce)
* **检查哪些应用和会话类型会应用插件键**：[各使用入口何时应用插件键](https://code.claude.com/docs/zh-CN/plugins/org.md#when-each-surface-applies-the-plugin-keys)
* **设置 CI 和容器**：[为容器和 CI 预置内容](https://code.claude.com/docs/zh-CN/plugins/org.md#seed-containers-and-ci)
* **提供用户可以安装的 mod**：[托管市场](https://code.claude.com/docs/zh-CN/plugins/host-marketplace.md)。Claude Code 从 GitHub、git、URL 或 npm 源复制的 mod 计为用户的，而不是[您组织的](#install-your-organizations-mods)。

### 在内置守卫上设置选项

内置守卫接受选项。在托管设置中的 `pluginConfigs` 下设置它们，以 `cc-plugin-sec-default@builtin` 为键，如[停止加载用户安装的 mod](#stop-user-installed-mods-from-loading)中的示例所示。

该表给出了每个选项未设置和设置为 `true` 时您的用户获得的结果：

| 选项 | 未设置 | `true` |
| :- | :- | :- |
| `allowManagedModsOnly` | 用户自己的 mod 会运行 | 仅[您组织的 mod](#install-your-organizations-mods) 和内置于 Claude Code 的 mod 会运行其 hook。Claude Code 会拒绝所有其他 mod，包括用户安装的或使用 `--plugin-dir` 指定的 mod。 |
| `allowModsToOverrideDenyRules` | 拒绝规则优先于用户的 mod | 批准工具调用的用户 mod 可以批准 `deny` 规则拒绝的调用 |

这些规则决定选项是否生效：

* **id 在这里只有一种形式**：Claude Code 仅在 `cc-plugin-sec-default@builtin` 下读取选项。`prependPlugins` 也接受 `sec-default@builtin`，而 `pluginConfigs` 不接受。
* **仅托管设置有效**：用户、项目或本地设置文件中的相同条目，或使用 `--settings` 传递的文件中的相同条目，既不会设置选项，也不会放宽选项
* **守卫必须加载**：如果您设置了 `prependPlugins`，请[在列表中指定守卫](#install-your-organizations-mods)。守卫未加载时，两个选项都不适用。
* **守卫以失败即关闭方式运行**：如果守卫无法读取托管设置，它会在加载时拒绝每个用户的 mod。如果它无法针对用户的 mod 所批准的调用检查拒绝规则，它会拒绝该调用。

[来自内置守卫的消息](https://code.claude.com/docs/zh-CN/plugins/mods/troubleshoot.md#messages-from-the-built-in-guard)是任一选项生效时您的用户看到的内容。

## 运行您组织自己的 mod

您可以将自己的 mod 部署给每个用户，选择它们相对于用户 mod 的运行位置，并使用一个来强制执行策略。

### 安装您组织的 mod 并设置顺序

您组织的 mod 在用户 mod 无法加载的地方加载，并且可以在用户 mod 之前运行，因此 Claude Code 必须能够判断 mod 是否来自您。只有当以下所有条件都为真时，它才会将 mod 视为您组织的：

* 托管的 `enabledPlugins` 将 mod 的插件设置为 `true`
* 托管设置通过绝对路径将插件的[市场](https://code.claude.com/docs/zh-CN/plugins/create-marketplace.md)指定为用户机器上的目录。`extraKnownMarketplaces` 条目可以做到这一点，并且也会为用户注册该市场。
* 市场通过相对路径列出插件，因此 Claude Code [从该目录就地加载它](https://code.claude.com/docs/zh-CN/plugins/loading.md#in-place-and-copied-plugins)

为了满足这些条件，请让您的设备管理将市场目录复制到每台机器上的相同路径。使该目录及其上方的每个目录仅可由管理员写入，就像托管设置文件一样。任何可以在那里写入的人都可以重写您的 mod。您从 claude.ai 管理员控制台交付的托管设置可以携带这些键，但无法将目录放到机器上。

该目录包含市场的清单和插件：

```text theme={null}
/opt/acme/claude-plugins/
├── .claude-plugin/
│   └── marketplace.json
└── plugins/
    └── acme-guard/
        ├── .claude-plugin/
        │   └── plugin.json
        └── hooks/
            ├── hooks.json
            └── register.js
```

清单通过相对于该目录的路径列出插件：

```json /opt/acme/claude-plugins/.claude-plugin/marketplace.json theme={null}
{
  "name": "acme-tools",
  "owner": { "name": "Acme" },
  "plugins": [
    { "name": "acme-guard", "source": "./plugins/acme-guard", "description": "Acme policy mod" }
  ]
}
```

Claude Code 复制到其缓存中的插件计为用户的，即使托管的 `enabledPlugins` 启用了它。这涵盖了来自 GitHub、git、URL 或 npm 源的每个插件。其 mod 在用户 mod 中运行，`prependPlugins` 和 `appendPlugins` 会跳过它，`allowManagedModsOnly` 会拒绝它，`allowManagedHooksOnly` 会阻止它加载。用户的调试日志中有一行以插件的 id 和 `is enabled by managed settings, but` 开头。

Claude Code 每次即将采取行动（例如运行工具）时都会触发一个事件，并依次将其传递给每个 mod。计为您的 mod [在用户 mod 之前运行](https://code.claude.com/docs/zh-CN/plugins/mods/events.md#the-order-mods-run-in)，即使您没有在任何地方列出它。要设置其位置，请在两个设置之一中列出其 id。id 是插件的名称、`@` 和市场的名称，例如 `acme-guard@acme-tools`。

* **`prependPlugins`**：您的 mod 在任何用户 mod 之前看到每个事件，并在之后看到每个结果。它可以更改事件、拒绝事件或跳过用户 mod。
* **`appendPlugins`**：您的 mod 在每个用户 mod 之后运行，因此它只看到这些 mod 传递的事件，并以它们传递的形式看到

此示例在 `/opt/acme/claude-plugins` 声明 `acme-tools` 市场，启用来自它的 `acme-guard`，并首先运行该 mod，内置守卫在其后：

```json managed-settings.json theme={null}
{
  "extraKnownMarketplaces": {
    "acme-tools": {
      "source": { "source": "directory", "path": "/opt/acme/claude-plugins" }
    }
  },
  "enabledPlugins": { "acme-guard@acme-tools": true },
  "prependPlugins": ["acme-guard@acme-tools", "sec-default@builtin"]
}
```

每个键做一项工作：

* **`extraKnownMarketplaces`**：指定保存 `acme-tools` 市场的目录。`path` 是包含 `.claude-plugin/marketplace.json` 的目录的绝对路径。
* **`enabledPlugins`**：为接收这些托管设置的每个用户启用 `acme-guard`
* **`prependPlugins`**：将 `acme-guard` 放在第一位，内置守卫放在第二位，都在用户安装的任何 mod 之前。Claude Code 遵循您列出的顺序。

要确认用户的机器收到了设置，请参阅[检查策略是否生效](https://code.claude.com/docs/zh-CN/managed-settings.md#check-that-a-policy-is-in-force)。

要确认 mod 运行的位置，请在该机器上使用 `claude --debug` 启动会话，并在[调试日志](https://code.claude.com/docs/zh-CN/plugins/mods/troubleshoot.md#read-the-debug-log)中搜索 mod 的 id：

* **`hooks module acme-guard@acme-tools loaded`，带有 `tier prepend`**：mod 计为您组织的，并首先运行
* **同一行带有 `tier user`**：Claude Code 将其视为用户的 mod。第二行 `prependPlugins names acme-guard@acme-tools, which is not an enabled managed plugin with a hooks module; skipped` 表示列表跳过了它。

这些规则决定了两个列表中哪些 id 生效：

* **列表替换默认值**：当您在托管设置中设置 `prependPlugins` 时，请在其中列出 `sec-default@builtin` 以保留内置守卫。该守卫是内置的，不需要 `enabledPlugins` 条目。
* **您自己的 id 必须计为您的**：在托管设置中，Claude Code 会跳过其插件不满足组织 mod 条件的 id
* **仓库无法设置它们**：Claude Code 从托管设置读取这两个设置，从不从仓库的设置文件读取。用户可以在 `~/.claude/settings.json` 中设置它们来对自己的 mod 排序，但仅限于没有托管设置的机器，并且仅当会话的凭据不是 Team 或 Enterprise `/login` 时。在其他任何情况下，Claude Code 都会忽略用户设置中的这两个键。那里的列表既不添加也不删除内置守卫。

### 使用您自己的 mod 强制执行策略

要阻止每个用户的 mod，您不需要自己的 mod。设置 [`allowManagedModsOnly`](#stop-user-installed-mods-from-loading) 即可。当您想允许某些用户的 mod 并拒绝其他的，或记录 mod 的行为时，请编写策略 mod。

每次另一个 mod 即将加载时，您的 mod 会在名为 [`plugin.register`](https://code.claude.com/docs/zh-CN/plugins/mods/reference#other-mods) 的事件中收到 `claude plugin validate` 打印的列表。`prependPlugins` 中的 mod 可以读取该列表并拒绝该 mod。它也可以[按名称处理任何 mods API 调用](https://code.claude.com/docs/zh-CN/plugins/mods/api.md#reach-files-processes-and-the-network)，以便针对每个其他 mod 记录或拒绝该调用。名称是去掉 `$.` 的方法，因此 `fs.write` 上的 hook 会看到每个 `$.fs.write` 调用。

此策略 mod 拒绝任何自身代码调用 `$.process.run` 或 `$.process.spawn` 的用户 mod。它还保留审计日志，将每个工具调用和 mod 写入的每个文件写入调试日志。因为它首先运行，日志记录的是请求的原始内容，在任何用户 mod 更改之前。将其保存为 `acme-guard/hooks/register.js`：

```javascript acme-guard/hooks/register.js theme={null}
// The methods no user's mod may call, each spelled namespace.method
const BLOCKED_CALLS = ['process.run', 'process.spawn']

export function register(on) {
  // Runs each time another mod is about to load
  on('plugin.register', async ($, e, next) => {
    // Keep the calls in that mod's code that are on the blocked list
    const blocked = e.uses.calls.filter((call) => BLOCKED_CALLS.includes(call))
    if (e.tier === 'user' && blocked.length > 0) {
      // Returning refuse keeps the mod from loading, and the text is the reason
      return { refuse: 'Acme policy: mods may not call ' + blocked.join(', ') }
    }
    // Let every other mod load
    return next(e)
  })

  // Record each tool call, then let it go ahead unchanged
  on('tool.call', async ($, e, next) => {
    $.ui.log('audit tool.call ' + e.tool, { to: 'debug' })
    return next(e)
  })

  // Record which mod wrote a file, then the path, quoted because the mod chose it
  on('fs.write', async ($, e, next) => {
    $.ui.log('audit fs.write by ' + next.origin.plugin + ' ' + JSON.stringify(e.path), { to: 'debug' })
    return next(e)
  })
}
```

该文件注册三个 hook：

* **`plugin.register`**：决定另一个 mod 是否加载。它拒绝调用被阻止方法的用户 mod，并放行每个其他 mod。
* **`tool.call`**：为每个工具调用向调试日志写入一行，例如 `audit tool.call Bash`，并且不更改任何内容
* **`fs.write`**：为另一个 mod 发起的每个 `$.fs.write` 调用写入一行，例如 `audit fs.write by reader "/tmp/notes.md"`，并且不更改任何内容。mod 的名称在前，路径加了引号，因此 mod 选择的路径无法冒充该行的其他字段。

`plugin.register` hook 读取事件的两个字段：

* **`e.tier`**：mod 将运行的位置，为 `prepend`、`user`、`append` 或 `builtin` 之一。用户安装的每个 mod 都是 `user`。
* **`e.uses.calls`**：mod 调用的 mods API 方法，每个写作 `namespace.method`，例如 `process.run`，不带 `claude plugin validate` 打印的 `$.`

当用户安装调用 `$.process.run` 的 mod 时，该 mod 不会加载，其调试日志中有一行以 `refused by acme-guard:` 和您的原因结尾。在[热重新加载插件目录的会话](https://code.claude.com/docs/zh-CN/plugins/mods/troubleshoot.md#find-out-why-a-mod-does-nothing)中，拒绝信息也会出现在会话记录中。要在不拒绝整个 mod 的情况下阻止某个调用，请从该调用名称上的 hook 返回 `{ deny: 'your reason' }`。

要将审计行发送到调试日志以外的地方，请从相同的 hook 中调用 `$.http.fetch`。

会话可以在没有您的 mod 的情况下运行。如果运行已安装 mod 的工作线程[崩溃三次](https://code.claude.com/docs/zh-CN/plugins/mods/troubleshoot.md#mods-that-run-in-the-hooks-worker-are-off-for-this-session)，Claude Code 会卸载所有非内置的 mod（包括您的），直到用户运行 `/reload-plugins` 或启动新会话。此外，使用 `--safe-mode` 启动 Claude Code 的用户运行时不会加载已安装的 mod，包括您的。

[创建 mod](https://code.claude.com/docs/zh-CN/plugins/mods/create.md) 介绍 mod 需要的文件。[测试策略 mod](https://code.claude.com/docs/zh-CN/plugins/mods/test.md#test-a-mod-that-judges-other-mods) 提供此策略 mod 的测试文件。

#### 当您的检查失败时拒绝 mod

如果您的 `plugin.register` hook 抛出异常或超过其时间限制，Claude Code 会跳过该 hook，因此检查以放行方式失败，正在被检查的 mod 会加载。要以拒绝方式失败并拒绝用户 mod，请将检查移到命名函数中，并添加返回拒绝的 `.catch` 处理程序。此版本的文件仅显示 `plugin.register` hook，因此请在 `register` 中保留第一个版本的两个审计 hook：

```javascript acme-guard/hooks/register.js theme={null}
const BLOCKED_CALLS = ['process.run', 'process.spawn']

// The same check as before, moved into a function of its own
async function checkMod($, e, next) {
  const blocked = e.uses.calls.filter((call) => BLOCKED_CALLS.includes(call))
  if (e.tier === 'user' && blocked.length > 0) {
    return { refuse: 'Acme policy: mods may not call ' + blocked.join(', ') }
  }
  return next(e)
}

export function register(on) {
  // The handler runs only when checkMod throws or exceeds its time limit
  on('plugin.register', checkMod).catch(async ($, e, next) => {
    // Let your organization's mods and built-in mods load
    if (e.tier !== 'user') return next(e)
    // Refuse the user's mod that couldn't be checked
    return { refuse: 'Acme policy check failed, so this mod was not loaded' }
  })
}
```

处理程序就位后，如果检查在处理某个 mod 时抛出异常或超时，该 mod 不会加载，拒绝行会带有第二个原因，例如 `refused by acme-guard: Acme policy check failed, so this mod was not loaded`。处理程序将 `user` 层以外的每个 mod 传递给 `next(e)`，因此失败的检查不会阻止您组织列出的 mod。[处理失败的 hook](https://code.claude.com/docs/zh-CN/plugins/mods/events.md#handle-a-hook-that-fails) 介绍其他事件的 `.catch`。

## 后续步骤

* [插件安全](https://code.claude.com/docs/zh-CN/plugins/security.md)：任何插件可以在用户机器上执行的操作，以及如何在安装前查看一个
* [Mods 概述](https://code.claude.com/docs/zh-CN/plugins/mods/overview.md)：什么是 mod 以及它与 hooks、skills 和 MCP 服务器的比较
* [mods 运行的顺序](https://code.claude.com/docs/zh-CN/plugins/mods/events.md#the-order-mods-run-in)：`prependPlugins` 和 `appendPlugins` 如何与用户的 mods 配合
* [设置和环境变量](https://code.claude.com/docs/zh-CN/plugins/mods/reference.md#settings-and-environment-variables)：本页命名的每个设置在一个表中
