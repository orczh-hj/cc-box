> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# 创建一个 mod

> 让 Claude 从描述中编写一个 Claude Code mod，或者自己编写一个来计算工具调用并添加命令。学习重新加载和验证循环。

Mod 是一个 Claude Code [插件](https://code.claude.com/docs/zh-CN/plugins/overview.md)，具有一个入口文件，称为 hooks 模块：一个 JavaScript 或 TypeScript 文件，其函数在事件发生时由 Claude Code 调用。创建方式如下：

* **让 Claude 编写它**：在 Claude Code 会话中[描述你想要的内容](#ask-claude-for-a-mod)
* **自己编写**：[按照教程](#write-a-mod-yourself)学习 mod 代码的工作原理。你不需要 Node.js、打包工具或构建步骤，因为 Claude Code 直接加载 `.js` 和 `.ts` 文件。

如果你还没有决定 mod 是否是合适的工具，请先阅读[概述中的比较](https://code.claude.com/docs/zh-CN/plugins/mods/overview.md#compare-mods-settings-hooks-skills-and-mcp-servers)。

> [!NOTE]
>
> 请使用 Claude Code v2.1.287 或更高版本。在 shell 中运行 `claude --version` 进行检查。要查看 mod 是否可以为您加载，请参阅[检查 mod 是否可以加载](https://code.claude.com/docs/zh-CN/plugins/mods/troubleshoot.md#check-whether-mods-can-load)。

## 让 Claude 为你编写一个 mod

在交互式 Claude Code 会话中描述你想要的 mod，Claude 会编写它。Claude 使用一个名为 `plugin-authoring` 的内置[技能](https://code.claude.com/docs/zh-CN/skills.md)，它告诉 Claude 在哪里编写 mod、你的版本有哪些事件和方法，以及 mod 如何被加载。当你请求一个 mod 时，Claude 可以加载该技能，或者你可以通过在 Claude Code 提示符处运行 `/plugin-authoring` 来自己加载它。

一旦你批准了 mod，它就会运行，除了在[某些会话中 Claude 编写的 mod 无法加载](#sessions-that-skip-the-approval)的情况。

**1. 描述 mod**

用你自己的话请求 mod，例如 `make a mod that shows the current git branch above the prompt`。Claude 在会话的 mods 文件夹中的自己的目录中编写 mod，该文件夹是 `~/.claude/dev-mods/` 后跟会话的 ID。mod 的完整路径看起来像 `~/.claude/dev-mods/3f2a9c1e-5b7d-4e8a-9c21-6d0f4b8a7e13/git-branch/`。

> [!NOTE]
>
> 在 `default` 和 `acceptEdits` [权限模式](https://code.claude.com/docs/zh-CN/permission-modes.md)中，Claude Code 会在 Claude 创建 mod 的每个文件之前请求您批准，因为 mods 文件夹是一个[受保护的路径](https://code.claude.com/docs/zh-CN/permission-modes.md#protected-paths)。请在每个文件出现时批准它。

**2. 批准 mod**

当 Claude 保存第一个文件时，Claude Code 会询问是否为会话启用热重新加载。热重新加载运行此会话中 Claude 编写的 mod，并在每个后续更改它们的转折处选择每个更改。

选择以下答案之一：

* **为此会话启用**：会话的 mods 文件夹中的 mod 在转折结束时加载，并在每个更改它们的转折结束时重新加载。你的答案在整个会话中持续，包括在你恢复它之后。
* **暂时不**：现在什么都不加载。文件保留在 Claude 编写的位置，mod 在该会话下次启动时加载。要防止 mod 加载，请删除其目录。

**3. 检查 mod 是否已加载**

在 Claude Code 提示符处运行 `/plugin`，然后按 Tab 直到选中**已安装**选项卡。它列出了 mod，你可以在那里关闭它。

**4. 尝试 mod**

使用您请求的功能。对于示例提示词，当前分支名称会显示在输入框上方。如果 mod 没有按您的预期工作，请告诉 Claude 需要更改什么。mod 会在每个更改其文件的轮次结束时重新加载，因此 Claude 完成后您就可以立即尝试该更改。

如果会话记录中以该 mod 名称开头的某一行显示[更改未加载](https://code.claude.com/docs/zh-CN/plugins/mods/troubleshoot.md#this-mods-change-didnt-load)，请在向 Claude 请求修复时将该行粘贴到您的下一条消息中。除非您粘贴，否则 Claude 看不到该行。

### 在其他会话中使用 mod

Claude 编写的 mod 仅在创建它的会话中加载，一旦 Claude Code 的会话比 [`cleanupPeriodDays`](https://code.claude.com/docs/zh-CN/settings-reference#cleanupperioddays) 更旧，它就会删除该会话的 mods 文件夹。要保留 mod，请将其目录从 mods 文件夹复制到你自己的位置，例如 `~/mods/git-branch`。然后选择如何加载它：

* **在你启动的会话中**：在你的 shell 中，运行 `claude --plugin-dir ~/mods/git-branch`
* **对于其他人**：[将其添加到市场](#share-your-mod)，以便他们可以安装它

### Claude 编写的 mod 无法加载的会话

Claude 编写的 mod 仅在你批准它后加载，在允许 mod 运行的受信任工作区中。在这些会话中它不会加载：

* **没有人在那里批准**：会话无法向你显示提示，如在 `claude -p` 运行或 [`dontAsk` 模式](https://code.claude.com/docs/zh-CN/permission-modes)中
* **工作区不受信任**：你还没有接受目录的信任提示
* **mod 已被禁用**：您使用 `--safe-mode` 或 `--bare` 启动，您设置了 `disableAllHooks`，或者您组织的[托管设置阻止了它](https://code.claude.com/docs/zh-CN/plugins/mods/admin.md#choose-how-much-to-allow)

## 自己编写一个 mod

在本教程中，你构建一个名为 `first-mod` 的 mod，它计算 Claude 进行的工具调用，在 Claude 工作时在微调器旁边显示计数，并添加一个 `/tally` 命令来打印它。然后你读取 Claude Code 在你的 mod 旁边写入的类型声明，并运行 `claude plugin validate`。它们一起向你展示你的版本提供的事件和方法，以及 Claude Code 从你的代码中读取的内容。

这个录制显示了完成的 mod。微调器计算工具调用，`/tally` 打印计数，对代码的编辑在会话运行时生效：

<video autoPlay muted loop playsInline controls src="https://mintcdn.com/claude-code/dgiVO_Od1X1faduV/images/mods-first-mod-light.mp4?fit=max&auto=format&n=dgiVO_Od1X1faduV&q=85&s=eb561134afa90375777408453ba51c77" aria-label="In a Claude Code session, the prompt 'list the files here and read the README' is typed and sent. The spinner reads 'Thinking · tool calls: 1' and the count rises as Claude works. The /tally command prints 'first-mod: Claude has made 3 tool calls since this mod loaded'. A line says first-mod reloaded and lists its four hooks. On the next prompt the spinner reads 'Thinking · tools used: 1'." />

你编写三个文件：

```text theme={null}
first-mod/
├── .claude-plugin/
│   └── plugin.json
└── hooks/
    ├── hooks.json
    └── register.js
```

* **`plugin.json`**：插件的[清单](https://code.claude.com/docs/zh-CN/plugins/manifest-reference.md)
* **`hooks.json`**：[指向你的代码文件](https://code.claude.com/docs/zh-CN/plugins/mods/reference.md#files)
* **`register.js`**：你的代码，称为 hooks 模块

**1. 创建插件目录**

创建保存文件的两个目录：

> [tab] Bash or Zsh
>
> ```bash theme={null}
> mkdir -p first-mod/.claude-plugin first-mod/hooks
> ```

> [tab] PowerShell
>
> ```powershell theme={null}
> New-Item -ItemType Directory -Force first-mod\.claude-plugin, first-mod\hooks
> ```

**2. 编写清单**

Mod 是一个插件，mod 需要一个[清单](https://code.claude.com/docs/zh-CN/plugins/manifest-reference.md)。这个 mod 的清单没有特殊字段。将其保存为 `first-mod/.claude-plugin/plugin.json`：

```json first-mod/.claude-plugin/plugin.json theme={null}
{
  "name": "first-mod",
  "version": "0.1.0",
  "description": "Counts Claude's tool calls, shows the count beside the spinner, and adds a /tally command",
  "author": { "name": "Your Name" }
}
```

**3. 告诉 Claude Code 你的代码在哪里**

当 Claude Code 加载一个插件时，它读取插件的 `hooks/hooks.json`。该文件中的 `modules` 键给出你的代码的路径，拥有它是使插件成为 mod 的原因。列出一个路径，相对于 `hooks.json`。这里它指向 `register.js`，你在下一步中编写。

将其保存为 `first-mod/hooks/hooks.json`：

```json first-mod/hooks/hooks.json theme={null}
{
  "description": "The first-mod hooks module",
  "modules": ["./register.js"]
}
```

**4. 编写代码**

这个文件是 mod 的代码，称为 hooks 模块。当 mod 加载时，Claude Code 调用文件导出的 `register` 函数，并传递一个名为 [`on`](https://code.claude.com/docs/zh-CN/plugins/mods/reference#the-hook-function) 的函数。每次调用 `on` 都会为它命名的事件注册一个事件处理程序，称为 hook。

将其保存为 `first-mod/hooks/register.js`：

```javascript first-mod/hooks/register.js theme={null}
// The count, shared by the hooks below
let calls = 0

// Claude Code calls this once when the mod loads
export function register(on) {
  // Runs when the session starts, before your first prompt
  on('session.start', async ($, e, next) => {
    // Add the /tally command
    await $.command.register({
      name: 'tally',
      description: 'Show how many tool calls Claude has made',
    })
    // Let the session start as usual
    return next(e)
  })

  // Runs each time Claude is about to use a tool
  on('tool.call', async ($, e, next) => {
    calls += 1
    // Ask Claude Code to draw the interface again, so the new count shows
    $.ui.invalidate('ui.render')
    // Let the tool run as usual
    return next(e)
  })

  // Runs when you type /tally, and only then, because of the matcher
  on('command.run', { command: 'tally' }, async () => {
    // The text to print in the transcript
    return { text: 'Claude has made ' + calls + ' tool calls since this mod loaded' }
  })

  // Runs each time Claude Code draws the spinner
  on('ui.render', { component: 'Spinner' }, async ($, e, next) => {
    // Keep Claude Code's spinner, with the count added after its word
    return next({ ...e, props: { ...e.props, suffix: ' · tool calls: ' + calls + '…' } })
  })
}
```

该文件在 `calls` 中保持计数，并注册四个 hook：

* **[`session.start`](https://code.claude.com/docs/zh-CN/plugins/mods/reference#session)** 在会话启动时运行，在你的第一个提示之前，以及每次 mod 重新加载时。它将 `/tally` 命令添加到 Claude Code。
* **[`tool.call`](https://code.claude.com/docs/zh-CN/plugins/mods/reference#tools)** 每次 Claude 即将使用工具时运行。它将 1 添加到 `calls` 并要求 Claude Code 再次绘制界面。
* **[`command.run`](https://code.claude.com/docs/zh-CN/plugins/mods/reference#commands-and-configuration)** 当你键入 `/tally` 时运行。它返回要打印的文本。
* **[`ui.render`](https://code.claude.com/docs/zh-CN/plugins/mods/reference#interface)** 每次 Claude Code 绘制微调器时运行。它在微调器的单词后添加计数。

[示例 mod 如何工作](#how-the-example-mod-works)解释了每个 hook 采用的三个参数以及每个参数返回的内容。

**5. 加载 mod**

使用 `--plugin-dir` 标志启动 Claude Code，它为一个会话加载一个插件目录而不安装它：

```bash theme={null}
claude --plugin-dir ./first-mod
```

**6. 尝试 mod**

要求 Claude 做一些需要几个工具调用的事情，例如 `list the files here and read the README`。当 Claude 工作时，微调器的单词后跟一个上升的计数，如 `Thinking · tool calls: 2…`。当 Claude 完成时，键入 `/tally` 并按 Enter。转录显示 `first-mod: Claude has made 2 tool calls since this mod loaded`，带有你自己的计数。Claude Code 将插件的名称放在命令的文本前面。

要在非交互模式下检查命令，请运行它：

```bash theme={null}
claude -p "/tally" --plugin-dir ./first-mod
```

```text theme={null}
first-mod: Claude has made 0 tool calls since this mod loaded
```

如果 `/tally` 不在命令列表中，则模块未加载。请参阅[找出为什么 mod 什么都不做](https://code.claude.com/docs/zh-CN/plugins/mods/troubleshoot.md#find-out-why-a-mod-does-nothing)。

**7. 在会话运行时更改代码**

保持会话打开。在 `register.js` 中，在 `ui.render` hook 中将 `' · tool calls: '` 更改为 `' · tools used: '` 并保存。突出显示的行是更改的行：

```javascript first-mod/hooks/register.js {4} theme={null}
  // Runs each time Claude Code draws the spinner
  on('ui.render', { component: 'Spinner' }, async ($, e, next) => {
    // Keep Claude Code's spinner, with the count added after its word
    return next({ ...e, props: { ...e.props, suffix: ' · tools used: ' + calls + '…' } })
  })
```

转录中的一行说 `first-mod` 重新加载并列出其 hook，下一个微调器使用新文本，如 `Thinking · tools used: 1…`。

### 示例 mod 如何工作

你传递给 `on` 的每个函数都是一个 hook，它是一个事件处理程序。Claude Code 将相同的三个参数传递给每个 hook：

* **Mods API**，名为 `$`：mod 可以调用的每个方法来到达自身之外，在[命名空间](https://code.claude.com/docs/zh-CN/plugins/mods/reference.md#mods-api-methods)中，例如 `$.ui` 和 `$.command`
* **事件**，名为 `e`：[事件的输入](https://code.claude.com/docs/zh-CN/plugins/mods/reference.md#events)作为纯数据，例如工具调用的名称和参数
* **下一个处理程序**，名为 [`next`](https://code.claude.com/docs/zh-CN/plugins/mods/events#how-a-hook-handles-an-event)：一个函数，将事件传递给其他 mod，然后传递给 Claude Code 自己的行为，并返回结果

`first-mod` 中的 hook 以下列方式处理各自的事件：

* **观察**：`session.start` hook 注册命令，`tool.call` hook 计算调用并要求重新绘制。两者都返回 `next(e)`，所以会话启动，工具照常运行。
* **回答**：`command.run` hook 返回自己的结果，从不调用 `next`。`on` 的第二个参数 `{ command: 'tally' }` 是一个过滤器，称为[匹配器](https://code.claude.com/docs/zh-CN/plugins/mods/events.md#filter-which-events-a-hook-handles)，所以 hook 仅对 `/tally` 运行。
* **重写**：`ui.render` hook 使用 `e` 的副本调用 `next`，其 `suffix` 保持计数，所以 Claude Code 绘制其通常的微调器，你的文本在单词后面

Claude Code 监视使用 `--plugin-dir` 加载的目录，并在其中的文件更改时热重新加载 hooks 模块。每次重新加载都会再次运行 `register`，所以 `calls` 回到 `0`，`/tally` 开始重新计数。要在重新加载中保持值，请参阅[保持状态](https://code.claude.com/docs/zh-CN/plugins/mods/interface.md#keep-state)。

## 继续处理 mod

一旦 mod 加载，你可以让 Claude 更改它，根据你的版本的类型定义检查你的代码，列出 Claude Code 在其中找到的事件和调用，并测试它。

### 使用 Claude 更改 mod

要更改你已有的 mod，使用 `--plugin-dir` 指向 mod 的目录启动会话，以便 Claude 编写的内容在同一会话中加载：

```bash theme={null}
claude --plugin-dir ./first-mod
```

然后请求更改，例如 `add a /tally-reset command to this mod that sets the tally back to zero`。Claude 编辑 hooks 模块，运行 `claude plugin validate`，并修复它报告的内容。你使用 `--plugin-dir` 加载的目录是一个[受保护的路径](https://code.claude.com/docs/zh-CN/permission-modes.md#protected-paths)，所以在 `default` 和 `acceptEdits` 模式中，你被要求批准 Claude 对 mod 的每个编辑。受保护的路径表给出其他权限模式的结果。

Claude 在其转折期间保存的文件在转折结束时重新加载，所以你可以在 Claude 完成后立即尝试 `/tally-reset`。

### 获取你的版本的类型定义

当 Claude Code 在交互式会话中从 `--plugin-dir` 加载 mod，或加载 [Claude 为您编写](#ask-claude-for-a-mod)的 mod 时，它会将 TypeScript 声明文件写入 mod 的 `.claude-plugin/types/` 目录。这些文件描述了您正在运行的 Claude Code 版本中的确切事件、mods API 方法和元素，因此您的编辑器可以对您的 hook 进行自动补全和类型检查。该目录包含以下文件：

| 路径 | 它声明的内容 |
| :- | :- |
| `claude-code/index.d.ts` | 每个事件及其输入和结果，每个 mods API 命名空间和方法，以及每个表面可以绘制的元素 |
| `claude-code-tools/index.d.ts` | 内置工具的输入和结果，以便检查 `e.tool === 'Bash'` 缩小 `e` |
| `claude-code-mcp/index.d.ts` | 上次在 mod 中保存文件时连接的 MCP 工具的输入 |
| 以插件命名的目录中的 `index.d.ts` | 该插件添加到 mods API 的内容。你的 `plugin.json` 在 `dependencies` 下列出的每个插件都有一个目录。 |
| `tsconfig.json` | 适合 hooks 模块的编译器选项 |

如果你的 mod 没有自己的 `tsconfig.json`，Claude Code 会在 mod 的根目录添加一个，扩展生成的那个，所以你的编辑器和 `tsc -p ./first-mod` 类型检查 mod 而无需更多设置。

事件和方法可以在版本之间更改，所以当它们不同意时，相信这些文件而不是任何页面，包括这个。

`claude-code/index.d.ts` 是你的构建的最完整的参考，每个 mods API 方法都有注释和示例。要查找某些内容，请在文件中搜索其名称，例如 `'tool.call'`。

### 检查 Claude Code 从你的 mod 中读取的内容

要以 Claude Code 看到的方式查看你的 mod，而不运行你的代码或启动会话，请使用 `claude plugin validate`。它检查清单并对 hooks 模块的源运行相同的静态分析，Claude Code 在加载 mod 时运行。在你的 shell 中，在 mod 的目录上运行它：

```bash theme={null}
claude plugin validate ./first-mod
```

对于 `first-mod`，输出包括这些行。

```text theme={null}
  ❯ ./register.js hooks: session.start, tool.call, command.run{command=tally}, ui.render{component=Spinner}
  ❯ ./register.js calls: $.command.register, $.ui.invalidate

✔ Validation passed
```

在 `hooks:` 行中查看您的模块 hook 的事件，每个事件的过滤器都显示在大括号中；在 `calls:` 行中查看它调用的每个 mods API 方法。如果您的模块读取或设置环境变量，还请查看 `env reads:` 和 `env writes:` 行；如果它使用 [`$.state`](https://code.claude.com/docs/zh-CN/plugins/mods/interface#keep-state)，还请查看 `state reads:` 和 `state writes:` 行。对于每个可以拒绝操作的 hook，您还会看到一行，例如 `gating hook without .catch: tool.call`，它表明该 hook 是否有 [`.catch` 处理程序](https://code.claude.com/docs/zh-CN/plugins/mods/events#handle-a-hook-that-fails)。

如果您打算处理的某个事件未出现在第一行中，Claude Code 也不会调用该 hook。常见原因是事件名称拼写错误，该命令会将其报告为错误，例如 `"tool.calls" is not an event`。

遵循这些规则，以便静态分析可以找到每个 hook 和调用：

* 完整写出每个 mods API 调用：`$`、命名空间，然后是方法，例如 `$.store.get('notes')`。您可以将 `$` 传递给在同一文件顶层声明的函数；对于名为 `loadNotes` 的自定义函数，`calls:` 行会显示 `$.store.get (via loadNotes)`。将 `$` 传递给方法、在 hook 内部定义的函数或从您的其他文件导入的函数会导致验证失败。[`$.state`](https://code.claude.com/docs/zh-CN/plugins/mods/interface#keep-state) 使用的 `read` 和 `update` 函数是可以接收它的导入。不要将 `$` 或其某个命名空间赋值给变量、对其解构，或使用计算名称对其进行索引。`const ui = $.ui` 会失败并报告 `$.ui is used as a value`。
* 在每个 `on` 调用中将事件名称写为字符串文字，例如 `'tool.call'`。变量或循环遍历名称列表会失败，出现 `the event name passed to on() is not a string literal`。
* 在 `register` 内，不要声明第二个名为 `on` 的变量或参数。验证失败，出现 `"on" is declared again (shadowed)`。
* 仅从插件目录内的文件导入，通过相对路径。唯一允许的裸导入是 `claude-code`，用于类型和一些帮助程序。
* 在文件顶部使用 `import` 声明，如 `import { name } from './file.js'`。动态 `import()` 失败，出现 `a dynamic import(); a hooks module imports its own files with an import declaration`。
* 将每个文件写为 ES 模块，使用 `import` 而不是 `require`。[参考](https://code.claude.com/docs/zh-CN/plugins/mods/reference.md#files)列出 Claude Code 加载的文件扩展名。

### 测试 mod

您可以为 mod 编写自动化测试，并在 shell 中使用 `claude plugin test` 运行，无需会话、登录或网络。测试会触发您的 hook 所处理的事件，并检查 hook 执行了哪些操作。

以下测试触发两次工具调用，运行 `/tally`，并检查回复是否统计了这两次调用。将其保存为 `first-mod/tests/first-mod.test.ts`：

```typescript first-mod/tests/first-mod.test.ts theme={null}
import { expect, test } from 'claude-code/testing'

test('/tally reports the tool calls the mod has seen', async ($, on) => {
  // Answer each tool call in Claude Code's place, so no tool runs
  on('tool.call', () => ({ result: 'ok' }))

  // Fire two tool calls, which the mod's tool.call hook counts
  await $.tool.call({ tool: 'Bash', command: 'ls' })
  await $.tool.call({ tool: 'Read', file_path: 'README.md' })

  // Run /tally and check the text its hook returns
  const answer = await $.command.run({ command: 'tally', args: '' })
  expect(answer.text).toBe('Claude has made 2 tool calls since this mod loaded')
})
```

在你的 shell 中，从 `first-mod` 目录运行测试：

```bash theme={null}
claude plugin test
```

输出命名每个测试及其是否通过，时间从运行到运行变化：

```text theme={null}
tests/first-mod.test.ts:
(pass) /tally reports the tool calls the mod has seen [22.87ms]

 1 pass
 0 fail
Ran 1 test across 1 file. [0.19s]
```

[测试 mod](https://code.claude.com/docs/zh-CN/plugins/mods/test.md)涵盖存根模型调用或存储，以及测试计时器和绘图。

## 分享你的 mod

mod 是一个插件，所以您在清单中对其进行版本控制，人们使用 `/plugin` 命令安装和更新它。如何分享取决于分享对象：

* **少数几个人**：将插件的目录或其 `.zip` 文件发送给他们。请参阅[在没有市场的情况下分享插件](https://code.claude.com/docs/zh-CN/plugins/publish.md#share-a-plugin-without-a-marketplace)
* **您的团队**：将其列入[您自己的市场](https://code.claude.com/docs/zh-CN/plugins/publish.md#publish-through-your-own-marketplace)，例如每个插件各占一个目录的私有仓库。要为在某个仓库中工作的所有人添加该市场，请[在仓库的设置中注册它](https://code.claude.com/docs/zh-CN/plugins/host-marketplace.md#register-the-marketplace-for-everyone-in-a-repository)
* **您的整个组织**：管理员可以通过托管设置[安装您组织的 mod](https://code.claude.com/docs/zh-CN/plugins/mods/admin.md#install-your-organizations-mods)
* **任何人**：将您的市场仓库设为公开，或[将插件提交到 Anthropic 的目录](https://code.claude.com/docs/zh-CN/plugins/publish.md#submit-to-anthropics-directory)

在你这样做之前，检查插件的 `name`：`claude plugin validate` 失败一个[看起来像 Anthropic 自己的](https://code.claude.com/docs/zh-CN/plugins/manifest-reference.md#name)名称，例如以 `claude-` 开头的名称。事件和方法可以在版本之间更改，所以你的 README 是说明你测试的 Claude Code 版本的地方。

继续针对目录使用 `--plugin-dir` 进行开发，而不是针对已安装的副本。Claude Code 按版本缓存已安装的插件，所以在您提高版本并再次安装之前，您的编辑不会到达已安装的副本。

## 后续步骤

* [在界面中绘制](https://code.claude.com/docs/zh-CN/plugins/mods/interface.md)：打开一个窗格，在提示上方绘制，并添加按钮和文本字段
* [对事件做出反应](https://code.claude.com/docs/zh-CN/plugins/mods/events.md)：hook 工具调用、提示和转折
* [使用 mods API](https://code.claude.com/docs/zh-CN/plugins/mods/api.md)：添加命令和工具、调用模型、在计时器上运行工作
* [测试 mod](https://code.claude.com/docs/zh-CN/plugins/mods/test.md)：存根 Claude Code 会回答的内容，以及测试计时器和绘图
* [排查 mod 故障](https://code.claude.com/docs/zh-CN/plugins/mods/troubleshoot.md)：mod 什么都不做的原因和调试日志
* [阅读内置 mod 的源代码](https://code.claude.com/docs/zh-CN/plugins/mods/overview.md#read-the-source-of-built-in-mods)：完整的插件，每个都有其 hooks 模块和测试
