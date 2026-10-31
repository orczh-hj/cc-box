> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Mods 概览

> 使用 mod 向 Claude Code 添加窗格、命令和工具调用规则。了解 mod 可以做什么、如何创建或安装 mod，以及 mod 在哪里运行。

Mod 是一个[插件](https://code.claude.com/docs/zh-CN/plugins/overview.md)，它改变 Claude Code 的外观和行为。它由 JavaScript 或 TypeScript 事件处理程序组成：Claude Code 在事件发生时调用一个处理程序，例如工具调用、提交的提示或界面的一部分被绘制，处理程序可以观察事件、更改事件或接管事件。使用 mod 向 Claude Code 添加自己的功能，例如一个窗格，在每个请求后显示上下文有多满。有关 mod 中的文件和完整示例，请参阅[Mod 如何工作](#how-a-mod-works)。

> [!NOTE]
>
> Claude Code 现有的 [hooks](https://code.claude.com/docs/zh-CN/hooks.md) 也在事件上运行，作为 shell 命令、HTTP 请求或在设置文件中配置的提示。Mod 的处理程序是在 Claude Code 内部运行的函数。Claude Code 调用两种类型的 hooks：在这些页面上，"hook" 指的是 mod 的处理程序，而设置文件类型是"设置 hook"。

## Mod 可以做什么

设置 hooks、skills、状态行和 MCP 服务器从 Claude Code 外部工作：每一个都运行一个脚本，或给 Claude 文本或工具。Mod 在 Claude Code 内部运行，所以它可以做他们做不了的事情：

* **绘制可以使用的界面**：在文本记录旁边的窗格或提示上方的带状区域，带有选项卡、按钮和文本字段。请参阅[在界面中绘制](https://code.claude.com/docs/zh-CN/plugins/mods/interface.md)。
* **重新绘制 Claude Code 自己的界面**：替换或重新设置 Claude Code 自己绘制的部分，例如工具调用的行、微调器或 Claude 提出问题的对话框。请参阅[更改 Claude Code 已经绘制的内容](https://code.claude.com/docs/zh-CN/plugins/mods/interface.md#change-what-claude-code-already-draws)。
* **进入工具调用或请求**：例如，在向用户提出问题时保持工具调用，在不运行工具的情况下回答问题，或将一个请求发送到不同的模型。请参阅[保护或更改工具调用](https://code.claude.com/docs/zh-CN/plugins/mods/events.md#guard-or-change-a-tool-call)和[跟随一个转折](https://code.claude.com/docs/zh-CN/plugins/mods/events.md#follow-a-turn)。
* **在命令上运行自己的代码**：一个 `/command`，立即运行你的函数，没有 Claude 转折，即使 Claude 正在工作。请参阅[添加命令或工具](https://code.claude.com/docs/zh-CN/plugins/mods/api.md#add-a-command-or-a-tool)。
* **在 hooks 之间共享数据**：mod 的 hooks 共享其文件中的变量，所以一个 hook 记录的内容，另一个可以显示。例如，一个 hook 可以计算工具调用，而另一个在微调器旁边显示计数，或者一个可以读取每个请求的令牌使用情况，而另一个在窗格中绘制它。请参阅[对事件做出反应](https://code.claude.com/docs/zh-CN/plugins/mods/events.md)。

mod 可在 Claude Code CLI、Claude Desktop 应用的 Code 选项卡以及 VS Code 扩展中工作。请参阅[mod 在哪里运行](#where-mods-run)以了解它们在其他地方的行为，例如在 `claude -p` 和云端会话中。如果设置 hook、skill 或 MCP 服务器已经能满足您的需要，请在编写 mod 之前先[比较它们](#compare-mods-settings-hooks-skills-and-mcp-servers)。要为组织管理 mod，请参阅[为您的组织管理 mod](https://code.claude.com/docs/zh-CN/plugins/mods/admin.md)。

## 获取 mod

要开始使用 mod：

* **使用您已经拥有的**：Claude Code 自身的一些功能就是 mod，例如 `/diff`。请参阅[内置于 Claude Code 的 mod](#mods-built-into-claude-code)。
* **创建一个**：在 Claude Code 会话中描述您想要的内容，Claude 会编写 mod。请参阅[向 Claude 请求 mod](https://code.claude.com/docs/zh-CN/plugins/mods/create.md#ask-claude-for-a-mod)。要了解 mod 代码如何工作，[自己编写一个](https://code.claude.com/docs/zh-CN/plugins/mods/create.md#write-a-mod-yourself)。
* **安装一个**：请参阅[安装或更新 mod](#install-or-update-a-mod)，或[试用示例 mod](#try-a-sample-mod)

### 安装或更新 mod

> [!WARNING]
>
> mod 是使用您的权限运行的代码。它可以读写您的文件、启动进程和发出网络请求。仅从您信任的作者和市场安装 mod。请参阅[决定是否信任 mod](#decide-whether-to-trust-a-mod)。

mod 作为插件从市场安装。给出插件的名称、一个 `@` 和市场的名称。这些示例从名为 `your-org` 的市场安装名为 `token-chart` 的插件：

* 在 Claude Code 会话中，运行 `/plugin install token-chart@your-org`。
* 在您的 shell 中，运行 `claude plugin install token-chart@your-org`。

[安装插件](https://code.claude.com/docs/zh-CN/plugins/install.md)涵盖市场、作用域、VS Code 扩展和桌面应用，以及[保持插件更新](https://code.claude.com/docs/zh-CN/plugins/install.md#keep-plugins-updated)，所有这些都适用于包含 mod 的插件，无需更改。

如果在会话打开时从 shell 安装或更新 mod，请在该会话中运行 `/reload-plugins` 以加载它。否则，它将在下次启动 Claude Code 时加载。

### 试用示例 mod

Anthropic 在 [`claude-code-playground` 仓库的 `claude-code/mods` 目录](https://github.com/anthropics/claude-code-playground/tree/main/claude-code/mods)中分享示例 mod。每个示例都是一个完整的插件，其 README 说明了它的构建方式。该仓库按原样分享这些示例，不提供支持。

* [`token-weather`](https://github.com/anthropics/claude-code-playground/tree/main/claude-code/mods/token-weather)：在输入框上方绘制上下文窗口的预测图
* [`blast-radius`](https://github.com/anthropics/claude-code-playground/tree/main/claude-code/mods/blast-radius)：拦截有风险的 shell 命令（例如 `rm -rf` 或强制推送），显示它将更改的内容，并提供继续或取消的按钮
* [`replay-theater`](https://github.com/anthropics/claude-code-playground/tree/main/claude-code/mods/replay-theater)：添加一个 `/replay` 命令，用于逐步浏览 Claude 在上一轮次中所做的文件编辑

示例 mod 使用您的权限运行。要在加载之前查看它的作用，请[列出其 hook 和调用](#list-what-a-mod-does-before-you-install-one)。

要试用某个示例，请克隆该仓库，并使用 `--plugin-dir` [为单个会话加载该 mod 的目录](https://code.claude.com/docs/zh-CN/plugins/create.md#load-a-directory-or-archive-for-one-session)。要确认 mod 已加载，请[检查会话加载了哪些 mod](#see-which-mods-a-session-loaded)。

要保留某个示例，请[将克隆中的 `claude-code/mods` 目录添加为市场](https://code.claude.com/docs/zh-CN/plugins/install#add-a-marketplace)，然后从 `claude-code-playground-mods` 安装该 mod。该市场指向您的克隆，因此如果您移动或删除该克隆，mod 将停止加载。

## 决定是否信任 mod

Mod 是使用您的权限在 Claude Code 内部运行的代码。仅从您信任的作者和[市场](https://code.claude.com/docs/zh-CN/plugins/security.md)安装 mod。

### Mod 可以访问什么

Mod 使用您的权限运行，所以在安装之前，请了解它可以访问什么。一旦加载，mod 可以：

* **在您的机器上以您的身份行动**：读写您的用户账户可以访问的任何位置的文件、启动程序以及发出网络请求
* **读取您的机密信息**：环境变量和设置文件，包括您保存在其中任何一处的 API 密钥
* **查看您的会话**：您发送的每个提示词和 Claude 进行的每个工具调用
* **更改您的会话**：重写提示词或工具调用、像您亲自输入一样提交提示词，或向您的另一个会话发送消息
* **在不询问您的情况下行动**：在询问您之前批准工具调用
* **消耗您的使用量**：使用您的计划或 API 密钥调用模型

Mod 不在沙箱中运行。如果您启用[沙箱隔离](https://code.claude.com/docs/zh-CN/sandboxing.md)，沙箱会隔离 Claude 运行的 Bash 命令，而 mod 启动的进程在沙箱之外运行。

当存在[托管设置](https://code.claude.com/docs/zh-CN/settings.md#check-what-your-organization-enforces)，或以 Team 或 Enterprise [`/login`](https://code.claude.com/docs/zh-CN/authentication#authentication-precedence) 作为会话凭据时，默认情况下 mod 的批准无法让调用绕过您的 `ask` 规则或 `PreToolUse` hook。如果两者都不存在，mod 可以批准 `ask` 规则会提示确认的调用，或您的 `PreToolUse` hook 已阻止的调用。要了解 mod 何时还可以批准 `deny` 规则拒绝的调用，请参阅[使用 hook 扩展权限](https://code.claude.com/docs/zh-CN/permissions.md#precedence-among-rules-hooks-and-mods)。

Mod 可以重新设置 Claude Code 界面的大部分样式，但不能重新设置权限提示。它无法改变权限提示向您显示的内容。

### 在安装 mod 之前列出它做什么

在安装 mod 之前，您可以在不运行它的情况下列出它处理哪些事件以及它要求 Claude Code 做什么，例如读取文件或发出网络请求。首先获取插件的文件，例如通过克隆其仓库。然后，在您的 shell 中，对插件的目录运行 `claude plugin validate`：

```bash theme={null}
claude plugin validate ./some-mod
```

输出中的 `hooks:` 和 `calls:` 行列出了 mod 处理的事件以及它要求 Claude Code 做什么。[查看 mod 可以做什么](https://code.claude.com/docs/zh-CN/plugins/mods/admin.md#review-what-a-mod-can-do)展示了输出内容以及需要留意的调用。

## 打开或关闭 mods

mod 默认处于打开状态。在终端中，请使用 Claude Code v2.1.287 或更高版本。Desktop 应用包含其自带的 Claude Code 副本，mod 从 v2.1.286 起即可在其中使用。请在您使用 mod 的地方检查版本：

* **终端**：在您的 shell 中运行 `claude --version`。如果您的版本较旧，请[更新 Claude Code](https://code.claude.com/docs/zh-CN/setup.md#update-claude-code)。
* **Desktop 应用**：在 Code 选项卡的本地会话中输入 `/status`，查看 **Claude Code** 一行，其中会显示 `2.1.286` 之类的版本号。如果您的版本较旧，请[更新 Desktop 应用](https://code.claude.com/docs/zh-CN/desktop.md#claude-code-version-in-the-code-tab)。

要关闭 mods，选择要停止多少个，以及停止多长时间。要重新打开它们，撤销相同的更改：

* **一个 mod**：从[`/plugin` 中的**已安装**选项卡](https://code.claude.com/docs/zh-CN/plugins/install#manage-installed-plugins)禁用或卸载其插件
* **每个已安装的 mod，对于一个会话**：使用 [`--safe-mode`](https://code.claude.com/docs/zh-CN/cli-reference#cli-flags) 启动 Claude Code，这也会禁用您的其他自定义
* **您安装的每个 mod，在每个会话中**：在 `~/.claude/settings.json` 中设置 [`"disableAllHooks": true`](https://code.claude.com/docs/zh-CN/settings-reference#disableallhooks)。您的设置 hook 和自定义状态栏也会停止。您的组织管理的内容继续运行。

如果您通过组织使用 Claude Code，管理员也可以限制哪些 mods 加载。管理员从[停止用户安装的 mods 加载](https://code.claude.com/docs/zh-CN/plugins/mods/admin.md#stop-user-installed-mods-from-loading)开始。

`disableAllHooks` 和您组织的 `allowManagedModsOnly` 会停止 mod，但保留其插件的其余部分：插件保持安装状态，其 skill、命令、Agent 和 MCP 服务器照常加载。其他设置和标志的影响范围更广。[`disableAllHooks`](https://code.claude.com/docs/zh-CN/settings-reference#disableallhooks) 和[`allowManagedHooksOnly` 下运行的内容](https://code.claude.com/docs/zh-CN/settings-reference#what-runs-under-allowmanagedhooksonly)列出了每一项对插件及其设置 hook 的影响。

要了解 mods 是否可以为您加载，请参阅[检查 mods 是否可以加载](https://code.claude.com/docs/zh-CN/plugins/mods/troubleshoot.md#check-whether-mods-can-load)。

> [!NOTE]
>
> 如果您在早期访问期间设置了 `CLAUDE_CODE_ENABLE_FUNCTION_HOOKS`，请删除它。Claude Code v2.1.287 及更高版本忽略它，所以将其设置为 `0` 不会保持 mods 关闭。

### 查看会话加载了哪些 mods

要查看终端会话加载了哪些 mods，在 Claude Code 提示符处运行 `/plugin`。选项卡下的暗色行给出计数和名称，例如 `1 mod active · first-mod`。如果您安装的 mod 没有在那里列出，请参阅[找出为什么 mod 什么都不做](https://code.claude.com/docs/zh-CN/plugins/mods/troubleshoot.md#find-out-why-a-mod-does-nothing)。

## Mod 如何工作

Mod 是一个[插件](https://code.claude.com/docs/zh-CN/plugins/overview.md)，其代码注册事件处理程序，称为 hook。Claude Code 在相应事件发生时运行 hook，例如当 Claude 调用工具或绘制微调器时。一个小 mod 有三个文件：

```text theme={null}
first-mod/
├── .claude-plugin/
│   └── plugin.json
└── hooks/
    ├── hooks.json
    └── register.js
```

* **`plugin.json`**：插件的[清单](https://code.claude.com/docs/zh-CN/plugins/manifest-reference.md)
* **`hooks.json`**：[指向您的代码文件](https://code.claude.com/docs/zh-CN/plugins/mods/reference.md#files)
* **`register.js`**：[您的代码](https://code.claude.com/docs/zh-CN/plugins/mods/create.md#write-a-mod-yourself)，称为 hooks 模块。它告诉 Claude Code 在哪些事件上运行您的函数。

这是一个完整的 `register.js`。它统计 Claude 进行的工具调用次数，并在 Claude 工作时在微调器旁边显示计数，如 `Thinking · tool calls: 3…`。

```javascript hooks/register.js theme={null}
// The count, shared by the two hooks below
let calls = 0

// Claude Code calls this once when the mod loads
export function register(on) {
  // Runs each time Claude is about to use a tool
  on('tool.call', async ($, e, next) => {
    calls += 1
    // Ask Claude Code to draw the interface again, so the new count shows
    $.ui.invalidate('ui.render')
    // Let the tool run as usual
    return next(e)
  })

  // Runs each time Claude Code draws the spinner
  on('ui.render', { component: 'Spinner' }, async ($, e, next) => {
    // Keep Claude Code's spinner, with the count added after its word
    return next({ ...e, props: { ...e.props, suffix: ' · tool calls: ' + calls + '…' } })
  })
}
```

该文件注册了两个 hook，两者都使用顶部的 `calls` 变量：

* **[`tool.call`](https://code.claude.com/docs/zh-CN/plugins/mods/reference#tools) hook** 在 Claude 每次即将使用工具时运行。它将 `calls` 加一，要求 Claude Code 再次绘制界面，并让工具照常运行。
* **[`ui.render`](https://code.claude.com/docs/zh-CN/plugins/mods/reference#interface) hook** 在 Claude Code 每次绘制微调器时运行。它保留 Claude Code 自己的微调器，并在其文字后添加计数。

这段录屏展示了 mod 的工作过程。请观察输入框上方的微调器行：当 Claude 列出目录并读取两个文件时，它显示 `Thinking · tool calls: 1…`，然后是 `2…`，然后是 `3…`。

<video autoPlay muted loop playsInline controls src="https://mintcdn.com/claude-code/dgiVO_Od1X1faduV/images/mods-overview-light.mp4?fit=max&auto=format&n=dgiVO_Od1X1faduV&q=85&s=00a18aa0743b59a700f0275ce226e6d1" aria-label="在 Claude Code 会话中，输入并发送提示词'列出此处的文件并读取 README'。当 Claude 工作时，微调器显示'Thinking · tool calls: 1'，然后是 2，然后是 3，此时 Claude 列出文件并读取其中两个。" />

### Hook 可以对事件做什么

Claude Code 在对事件采取行动之前运行您的 hook，因此由 hook 决定接下来会发生什么。它可以：

* **观察**：记录正在发生的事情并让其不变地继续，就像示例中的 `tool.call` hook 一样
* **重写**：在事件继续之前更改事件，就像 `ui.render` hook 在向微调器添加计数时所做的那样
* **回答**：自己处理事件，使通常的行为不会运行，例如拒绝某个命令

要做任何超出其自身代码的事情，例如绘制、添加命令、调用模型、读取文件、启动进程或发出网络请求，hook 需要调用 mods API。Hook 没有其他方式来做这些事情，这就是为什么 Claude Code 可以在您安装之前[列出 mod 会做什么](#list-what-a-mod-does-before-you-install-one)。

有关每种选择背后的代码，请参阅[对事件做出反应](https://code.claude.com/docs/zh-CN/plugins/mods/events.md#how-a-hook-handles-an-event)。有关 hook 可以调用什么，请参阅[使用 mods API](https://code.claude.com/docs/zh-CN/plugins/mods/api.md)。

### Mods 在哪里运行

mod 的 hook 在加载该插件的每种会话中运行，无论由哪个提供商提供模型，包括 Amazon Bedrock、Google Cloud 的 Agent Platform 和 Microsoft Foundry。绘制的范围更窄：mod 绘制的内容会出现在终端、桌面应用和 VS Code 扩展中。此表列出了您可能运行 Claude Code 的每个地方：

| 您运行 Claude Code 的地方 | Hook 是否运行 | Mod 绘制的内容是否出现 |
| :- | :- | :- |
| 终端中的 `claude`，包括编辑器的集成终端和 JetBrains 插件 | 是 | 是 |
| 桌面应用的代码选项卡，WSL 会话除外 | 是 | 是，[元素表](https://code.claude.com/docs/zh-CN/plugins/mods/reference.md#elements)标记为仅限终端的元素除外 |
| 桌面应用中的 [WSL 会话](https://code.claude.com/docs/zh-CN/desktop-wsl.md) | 否，因为插件在 WSL 会话中不可用 | 否 |
| VS Code 扩展的聊天面板 | 是 | 是，需使用扩展版本 2.1.299 或更高版本。[不获取功能标志](https://code.claude.com/docs/zh-CN/env-vars.md#features-that-need-feature-flag-fetching)的会话（例如 Amazon Bedrock 上的会话）需要 2.1.304 或更高版本。[渲染位置](https://code.claude.com/docs/zh-CN/plugins/mods/reference.md#render-sites)表和[元素](https://code.claude.com/docs/zh-CN/plugins/mods/reference.md#elements)表标出了该面板绘制的内容。 |
| `claude -p` 和 [Agent SDK](https://code.claude.com/docs/zh-CN/agent-sdk/overview.md) | 是 | 否 |
| 从 claude.ai 或移动应用使用 [Remote Control](https://code.claude.com/docs/zh-CN/remote-control.md) | 是，在您机器上的会话中 | 在您机器上的终端中 |
| [云端会话](https://code.claude.com/docs/zh-CN/claude-code-on-the-web.md) | 是，适用于[可进入云端会话](https://code.claude.com/docs/zh-CN/cloud-environments.md#what-carries-over-from-your-setup)的插件 | 否 |

会绘制内容的 mod 可以检查自己运行在哪个应用中，并在无法绘制的地方回退为会话记录中的一行或命令的文本回复。

## 为你的组织控制 mods

管理员通过[托管设置](https://code.claude.com/docs/zh-CN/managed-settings.md)决定 mods 是否运行以及哪些运行。[为你的组织管理 mods](https://code.claude.com/docs/zh-CN/plugins/mods/admin.md)涵盖默认情况下发生的事情、如何查看 mod 以及如何使用你自己的 mod 强制执行策略。

## 比较 mod、设置 hook、skill 和 MCP 服务器

Mod、设置 hook、skill 和 MCP 服务器的功能有所重叠。此表说明每一种是什么以及何时选择它。

| | Mod | 设置 hook | Skill | MCP 服务器 |
| :- | :- | :- | :- | :- |
| 它是什么 | Claude Code 在其自己的进程中调用的插件中的函数 | Claude Code 在生命周期事件上运行的 shell 命令、HTTP 请求或提示词 | Claude 读取的 `SKILL.md` 文件指令 | 为 Claude 提供工具的外部进程或服务 |
| 它可以改变什么 | 工具调用、提示词、命令、轮次和界面绘制的内容 | 工具调用或提示词是否继续、工具调用的参数和结果，以及为 Claude 添加的上下文 | Claude 知道什么和做什么 | Claude 拥有哪些工具 |
| 它可以在界面中绘制吗 | 是 | 否 | 否 | 否 |
| 您编写什么 | JavaScript 或 TypeScript | 脚本和 `settings.json` 条目 | Markdown | 任何语言的服务器 |
| 何时选择它 | 您想要一个窗格、输入框上方的带状区域、自定义命令或重写事件 | 您想用已有的脚本阻止、允许或记录事件 | 您不断将相同的指令粘贴到聊天中 | Claude 需要访问外部系统 |

其他每一种都有自己的页面：[Hooks](https://code.claude.com/docs/zh-CN/hooks.md)、[Skills](https://code.claude.com/docs/zh-CN/skills.md) 和 [MCP](https://code.claude.com/docs/zh-CN/mcp.md)。一个插件可以容纳所有这些，因此 mod 可以与 skill 和 MCP 服务器放在同一个插件中发布。

## 内置于 Claude Code 的 Mods

Claude Code 的一些自己的功能是 mods。要查看您的会话拥有的，在 Claude Code 提示符处运行 `/plugin` 并转到**已安装**选项卡，它在**内置**下列出它们。您不能更新或卸载内置 mod，表的最后一列说明如何关闭每一个。[`mods active` 行](#see-which-mods-a-session-loaded)排除了内置 mods。

此表按 `/plugin` 显示的名称列出每个条目：

| `/plugin` 中的名称 | 它做什么 | 它在哪里打开 | 如何关闭它 |
| :- | :- | :- | :- |
| `cc-plugin-agents-md` | 将 `AGENTS.md` 加载为项目指令 | 每个会话，除了[无法读取 `AGENTS.md` 的会话](https://code.claude.com/docs/zh-CN/memory#when-agents-md-support-is-unavailable) | 在 `/plugin` 中禁用它，或[选择哪些指令文件加载](https://code.claude.com/docs/zh-CN/memory.md#choose-which-instruction-files-load) |
| `cc-plugin-diff` | 接管 [`/diff`](https://code.claude.com/docs/zh-CN/interactive-mode#review-changes-with-%2Fdiff) 并绘制其窗格 | 交互式终端会话 | 在 `/plugin` 中禁用它。`/diff` 保持，Claude Code 的内置版本的命令回答它。 |
| `cc-plugin-plugin-authoring` | 给 Claude [`plugin-authoring` skill](https://code.claude.com/docs/zh-CN/plugins/mods/create#ask-claude-for-a-mod) 用于编写 mods。它持有一个 skill，没有 mod 代码。 | 除非 Anthropic 已远程关闭已安装的 mods | 在 `/plugin` 中禁用它 |
| `cc-plugin-sec-default` | 保护您的组织管理的内容免受用户安装的 mods | [保护加载的地方](https://code.claude.com/docs/zh-CN/plugins/mods/admin.md#know-what-happens-by-default) | 您不能。管理员在托管设置中[设置顺序](https://code.claude.com/docs/zh-CN/plugins/mods/admin.md#install-your-organizations-mods) |
| `cc-plugin-telemetry` | 发送 Claude Code 及其内置 mods 记录的分析记录 | 无论 Claude Code 自己的分析在哪里打开 | 在 `/plugin` 中禁用它，或关闭分析，例如使用 [`DISABLE_TELEMETRY`](https://code.claude.com/docs/zh-CN/env-vars) |
| `cc-plugin-you-should-know` | [You should know](https://code.claude.com/docs/zh-CN/interactive-mode.md#you-should-know-suggestions)：一个侧边 Agent 在发现您可能错过的内容时，会在输入框上方显示一条注释 | 默认关闭，直到您[将其打开](https://code.claude.com/docs/zh-CN/interactive-mode.md#you-should-know-suggestions) | 在 `/plugin` 中禁用它 |

停止已安装 mods 的设置和标志，例如 `disableAllHooks`、`--bare` 和 `--safe-mode`，不会停止内置 mods。

### 阅读内置 mods 的源代码

其中一些 mods 的源代码在 [Claude Code 仓库的 `mods` 目录](https://github.com/anthropics/claude-code/tree/main/mods)中是公开的。每一个都是一个完整的插件，带有其 hooks 模块和测试：

* [`diff`](https://github.com/anthropics/claude-code/tree/main/mods/diff)：`/diff` 窗格，带有绑定到键盘操作的按钮和 mod 自己处理的滚动
* [`agents-md`](https://github.com/anthropics/claude-code/tree/main/mods/agents-md)：将 `AGENTS.md` 加载为项目指令，带有 [`userConfig`](https://code.claude.com/docs/zh-CN/plugins/components#user-configuration) 选项
* [`sec-default`](https://github.com/anthropics/claude-code/tree/main/mods/sec-default)：[了解默认情况下发生的事情](https://code.claude.com/docs/zh-CN/plugins/mods/admin.md#know-what-happens-by-default)中描述的保护，一个强制执行策略的 mod 的模型
* [`telemetry`](https://github.com/anthropics/claude-code/tree/main/mods/telemetry)：添加其他 mods 可以调用的方法，并提供其类型

## 后续步骤

* [创建 mod](https://code.claude.com/docs/zh-CN/plugins/mods/create.md)：构建一个计算工具调用、在微调器旁边显示计数并添加命令的 mod，并学习编辑和重新加载循环
* [在界面中绘制](https://code.claude.com/docs/zh-CN/plugins/mods/interface.md)：窗格、输入框上方的带状区域、按钮、文本字段和状态
* [对事件做出反应](https://code.claude.com/docs/zh-CN/plugins/mods/events.md)：工具调用、提示词、轮次和 mods 运行的顺序
* [使用 mods API](https://code.claude.com/docs/zh-CN/plugins/mods/api.md)：命令、工具、模型调用、计时器和文件
* [测试 mod](https://code.claude.com/docs/zh-CN/plugins/mods/test.md)：在没有会话的情况下运行的自动化测试
* [对 mod 进行故障排除](https://code.claude.com/docs/zh-CN/plugins/mods/troubleshoot.md)：mod 什么都不做的原因和调试日志
* [为您的组织管理 mods](https://code.claude.com/docs/zh-CN/plugins/mods/admin.md)：默认值、托管设置、查看 mod 和策略 mods
* [Mods 参考](https://code.claude.com/docs/zh-CN/plugins/mods/reference.md)：事件、方法、元素和限制
