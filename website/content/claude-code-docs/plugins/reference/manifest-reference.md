> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Plugin manifest 参考

> plugin.json 的完整参考：每个字段的类型和默认值、接受的路径形式，以及 userConfig 和环境变量模式。

Plugin manifest 是 plugin 的 `.claude-plugin/` 目录中的 `plugin.json` 文件。它包含 plugin 的元数据和 Claude Code 提示用户输入的 [`userConfig`](#user-configuration) 值。它还声明任何在其[默认位置](#standard-layout)之外定义内联或保留的组件。

本参考适用于 plugin 创建者，以及将组件字段放在 marketplace 条目中的 marketplace 所有者。

> [!NOTE]
>
> 这些情况在其他页面上有介绍：
>
> * **学习构建 plugin**：从[创建 plugin](https://code.claude.com/docs/zh-CN/plugins/create.md)开始
> * **每个组件在运行时的作用**：参见[Plugin 组件](https://code.claude.com/docs/zh-CN/plugins/components.md)

从与您要查找的内容相匹配的部分开始：

* 一个字段：[字段表](#fields)给出每个字段的类型、是否必需、其默认值以及它接受的内容。[路径规则](#path-rules)涵盖 `./` 前缀和每个组件路径的包含
* 一个 `userConfig` 选项或一个 `channels` 条目：[用户配置](#user-configuration)和[频道](#channels)模式
* `${CLAUDE_PLUGIN_ROOT}` 或 plugin 可以引用的另一个变量：[环境变量](#environment-variables)
* 每个组件的文件位置：[标准布局](#standard-layout)
* 来自 `claude plugin validate` 的消息：[故障排除页面](https://code.claude.com/docs/zh-CN/plugins/troubleshooting.md)列出每条消息及其修复，并链接到本页的相关部分

## Manifest 文件

manifest 是可选的。没有它，Claude Code 会加载它在[标准布局](#standard-layout)中找到的组件。然后 plugin 名称来自 marketplace 条目，或者当您使用 `--plugin-dir` 加载 plugin 时来自目录名称。

当您想要元数据、组件在其默认目录之外、`userConfig` 或内联组件定义时，编写 manifest。

在 plugin 根目录下的 `.claude-plugin/plugin.json` 处保存 manifest。将所有其他 plugin 文件放在 plugin 根目录，而不是在 `.claude-plugin/` 内。这包括 `skills/`、`commands/` 和 `hooks/`。

以下示例设置了[字段表](#fields)中的大多数键。它在包含每个引用路径的 plugin 目录中通过验证。

```json theme={null}
{
  "name": "deploy-tools",
  "displayName": "Deploy Tools",
  "version": "1.2.0",
  "description": "Deployment commands, a review agent, and a status monitor",
  "author": {
    "name": "Example Team",
    "email": "dev@example.com",
    "url": "https://example.com"
  },
  "homepage": "https://example.com/docs/deploy-tools",
  "repository": "https://github.com/example/deploy-tools",
  "license": "MIT",
  "keywords": ["deployment", "ci"],
  "defaultEnabled": true,
  "dependencies": ["secrets-vault"],
  "metadata": { "catalogId": "cat-123" },
  "skills": ["./extra-skills/"],
  "commands": {
    "status": {
      "source": "./commands/status.md",
      "description": "Show the current deployment status"
    },
    "about": {
      "content": "Explain what the deploy-tools plugin provides.",
      "description": "Describe this plugin"
    }
  },
  "agents": ["./agents/reviewer.md"],
  "hooks": "./config/extra-hooks.json",
  "mcpServers": {
    "deploy-api": {
      "command": "node",
      "args": ["${CLAUDE_PLUGIN_ROOT}/server.js"]
    }
  },
  "lspServers": "./.lsp.json",
  "outputStyles": "./styles/",
  "experimental": {
    "themes": "./themes/",
    "monitors": "./config/monitors.json"
  },
  "userConfig": {
    "api_token": {
      "type": "string",
      "title": "API token",
      "description": "Token for the deployment API",
      "sensitive": true
    }
  }
}
```

### 无法识别的字段

无法识别的顶级键被剥离，`userConfig` 选项、`channels` 条目、`lspServers` 配置或 `monitors` 条目内的无法识别的键被拒绝：

* **顶级字段**：字段被剥离，plugin 加载。`claude plugin validate` 将每个无法识别的顶级字段报告为警告
* **严格对象**：`userConfig` 选项、`channels` 条目、`lspServers` 配置和 `monitors` 条目是严格的。其中的未知键是错误，plugin 不加载

### 验证 manifest

`claude plugin validate` 是 manifest 的权威检查。从您的 shell 针对 plugin 目录运行它：

```bash theme={null}
claude plugin validate ./my-plugin
```

该命令报告以下结果之一：

* **`Validation passed`**：manifest 加载
* **`Validation passed with warnings`**：manifest 加载，但验证器发现需要修复的内容，例如 Claude Code 剥离的未知顶级字段、不是 kebab-case 的 `name`，或缺少 `version`、`description` 或 `author`。传递 `--strict` 以在 CI 中将警告转换为失败
* **`Validation failed`**：manifest 有类型不匹配、缺失或逃逸 plugin 根目录的路径，或 `userConfig` 选项、`channels` 条目、`lspServers` 配置或 `monitors` 条目内的未知键。Claude Code 在加载 plugin 时报告相同的问题

该命令还检查 plugin 在 `.mcp.json` 中声明的每个 MCP 服务器条目、在 [`mcpServers`](#mcpservers) 命名的 `.json` 文件中，或在 `plugin.json` 中内联声明的条目。这些 MCP 检查需要 Claude Code v2.1.281 或更高版本，包括：

* **错误**：Claude Code 在加载 plugin 时会丢弃的条目、对 manifest 未声明的选项的 `${user_config.KEY}` 引用，以及不是有效绝对 URL 的远程 `url`
* **警告**：到非环回主机的 `http://` 或 `ws://` URL，以及看起来像字面凭证的标头值

## 字段

该表列出了 `plugin.json` 中的顶级键。`name` 是唯一必需的键。如果字段名称是链接，链接的部分有其完整规则。

对于组件键（如 `commands` 和 `hooks`），[组件路径形式](#component-path-forms)显示每个接受的形式及示例，每个路径遵循 `./` 前缀、扩展名和包含的[路径规则](#path-rules)。

| 字段 | 类型 | 描述 |
| :- | :- | :- |
| `$schema` | String | 用于编辑器自动完成的 JSON Schema URL。Claude Code 在加载时忽略它 |
| [`name`](#name) | String | Plugin 标识符，必需。使用 kebab-case。每个组件都在其下命名空间 |
| [`displayName`](#displayname) | String | 在 UI 中显示的名称，代替 `name` |
| [`version`](#version) | String | 版本字符串。设置它会将用户保持在该版本，直到您更改它 |
| `description` | String | plugin 提供的内容的简短说明 |
| `author` | Object | `name`（必需），加上可选的 `email` 和 `url` |
| `homepage` | String | 文档 URL。必须解析为 URL，否则 plugin 加载失败 |
| `repository` | String | 源代码库 URL。未验证 |
| `license` | String | SPDX 标识符，如 `MIT` 或 `Apache-2.0` |
| `keywords` | Array of strings | 发现标签 |
| [`metadata`](#metadata) | Object | 用于您自己数据的自由形式对象。Claude Code 不读取它 |
| [`icon`](#directory-listing-fields) | String | 插件在 Anthropic 目录中的列表所用的图标。Claude Code 不读取它 |
| [`documentationUrl`](#directory-listing-fields) | String | 插件在 Anthropic 目录中的列表所用的文档链接。Claude Code 不读取它 |
| [`supportUrl`](#directory-listing-fields) | String | 插件在 Anthropic 目录中的列表所用的支持链接。Claude Code 不读取它 |
| [`privacyPolicyUrl`](#directory-listing-fields) | String | 插件在 Anthropic 目录中的列表所用的隐私政策链接。Claude Code 不读取它 |
| [`termsOfServiceUrl`](#directory-listing-fields) | String | 插件在 Anthropic 目录中的列表所用的服务条款链接。Claude Code 不读取它 |
| [`defaultEnabled`](#defaultenabled) | Boolean | 当用户未设置时 plugin 是否在启用时启动。默认为 `true` |
| [`dependencies`](#dependencies) | Array of strings or objects | 必须为此 plugin 启用的 plugin |
| [`settings`](#settings) | Object | Claude Code 在 plugin 启用时应用的设置。仅 `agent` 和 `subagentStatusLine` 生效 |
| [`userConfig`](#user-configuration) | Object | Claude Code 在 plugin 启用时提示用户输入的值 |
| `types` | Path | 声明 [mod](https://code.claude.com/docs/zh-CN/plugins/mods/reference.md#files) 的 `$.state` 值和 `$` 名词的 `.d.ts` 文件 |
| [`channels`](#channels) | Array of objects | plugin 提供的消息频道，每个绑定到其 MCP 服务器之一 |
| `skills` | Path, or array of paths | 要扫描的目录以查找 skills，每个目录是 `<name>/SKILL.md` 文件夹或直接包含 `SKILL.md` 的一个文件夹。`"."` 命名 plugin 根目录。添加到默认 `skills/` 扫描 |
| [`commands`](#commands) | Path, array of paths, or object | 平面 `.md` 命令文件、它们的目录或命令名称到 `source` 或 `content` 的对象映射。替换默认 `commands/` 扫描 |
| `agents` | Path, or array of paths | Agent `.md` 文件。不接受目录。替换默认 `agents/` 扫描 |
| [`hooks`](#hooks) | Path, object, or array of either | `.json` hook 文件或内联 hook 配置。与 `hooks/hooks.json` 一起加载 |
| [`mcpServers`](#mcpservers) | Path, object, or array of either | `.json` MCP 配置文件、`.mcpb` 或 `.dxt` 包，或以名称为键的内联服务器配置。与 `.mcp.json` 一起加载；稍后声明的服务器名称替换较早的名称 |
| [`lspServers`](#lspservers) | Path, object, or array of either | `.json` LSP 配置文件或以名称为键的内联服务器配置。与 `.lsp.json` 一起加载 |
| `outputStyles` | Path, or array of paths | 输出样式文件或目录。替换默认 `output-styles/` 扫描 |
| `workflows` | Path, or array of paths | [Workflow](https://code.claude.com/docs/zh-CN/workflows.md#distribute-a-workflow-in-a-plugin) `.js` 文件或目录。替换默认 `workflows/` 扫描 |
| `experimental` | Object | `themes`、`monitors` 和 `evals` 的容器，其 manifest 形式可能仍会改变 |
| `experimental.themes` | Path, or array of paths | 主题文件或目录。替换默认 `themes/` 扫描。顶级 `themes` 键仍然加载，带有 `claude plugin validate` 警告 |
| [`experimental.monitors`](#monitors) | Path, or inline array | 包含 monitors 数组的 `.json` 文件，或数组本身。默认为 `monitors/monitors.json`。顶级 `monitors` 键仍然加载，带有 `claude plugin validate` 警告。Monitors [在使用 `-p` 标志时不会启动](https://code.claude.com/docs/zh-CN/plugins/components#monitors) |
| `experimental.evals` | Path, or array of paths | 当不是默认 `evals/` 时，保存 plugin 的[评估案例](https://code.claude.com/docs/zh-CN/plugin-evals.md#use-a-different-eval-directory)的目录。`claude plugin eval --eval-dir` 覆盖它 |

在"类型"列中，路径是相对于 plugin 根目录的字符串，例如 `"./custom/commands"`。

### `name`

plugin 标识符。它必须非空，没有空格、`@`、`:`、路径分隔符、控制字符或双向格式字符；使用 kebab-case。

Claude Code 在其下命名空间每个组件，因此 plugin `deploy-tools` 中的 agent `reviewer` 显示为 `deploy-tools:reviewer`。

`claude plugin validate` 还检查该名称是否通过作为 Anthropic 自己的 plugin 之一。检查忽略大小写并将任何分隔符的运行视为一个：

| 名称 | 结果 |
| :- | :- |
| 以 `claude-`、`anthropic-`、`anthropics-` 或 `cc-plugin-` 开头 | 错误 |
| 是 `claude`、`anthropic`、`anthropics`、`claude-code` 或 `claude-mods` | 错误 |
| 将 `official` 放在 `claude` 或 `anthropic` 旁边，例如 `official-claude-tools` | 错误 |
| 在其他任何地方有 `claude`、`anthropic` 或 `anthropics` 作为整个单词，例如 `mcp-for-claude` | 警告 |

错误读作 `Plugin name "<name>" is reserved: it passes as one of Anthropic's own`，警告读作 `Plugin name "<name>" reads as one of Anthropic's own`。`claude plugin init` 和 `claude plugin tag` 拒绝引发错误的名称。Claude Code 仍然安装和加载其名称被拒绝的插件。

### `displayName`

在 UI 中显示的名称，代替 `name`。它可能包含空格和任何大小写，它不用于命名空间或查找。

对于 marketplace 安装的 plugin，[marketplace 条目](https://code.claude.com/docs/zh-CN/plugins/marketplace-reference.md#plugin-entries)上的 `displayName` 优先于此值。

### `version`

版本字符串，不针对 semver 检查。设置它会将插件固定到该版本，直到您更改它；参见[版本和更新](https://code.claude.com/docs/zh-CN/plugins/loading.md#versions-and-updates)。具有[`command` 源](https://code.claude.com/docs/zh-CN/plugins/marketplace-reference)的插件、来自[托管在 claude.ai 上的市场](https://code.claude.com/docs/zh-CN/plugins/install.md#add-from-claude-ai)的插件以及从通过本地路径添加的市场[就地加载](https://code.claude.com/docs/zh-CN/plugins/loading.md#find-plugins-on-disk)的插件不由此字段固定。

### `metadata`

用于您自己数据的自由形式对象，例如目录或权利字段。Claude Code 不读取它。需要 Claude Code v2.1.222 或更高版本。

### 目录列表字段

当您[提交插件](https://code.claude.com/docs/zh-CN/plugins/publish.md#submit-to-anthropics-directory)时，Anthropic 的目录会从 `plugin.json` 中读取 `icon`、`documentationUrl`、`supportUrl`、`privacyPolicyUrl` 和 `termsOfServiceUrl` 字段，用于您插件的列表。Claude Code 在加载时忽略它们。请仅在 `plugin.json` 中设置它们。在[市场条目](#marketplace-entries-and-the-manifest)中，`claude plugin validate` 会将它们逐一报告为未知字段。

将 `icon` 设置为插件内图像文件的路径，例如 `./logo.png`，并将四个 URL 字段分别设置为 `https://` URL。

> [!NOTE]
>
> 在 Claude Code v2.1.281 或更高版本上，`claude plugin validate` 不会将 `plugin.json` 中的这些字段报告为未知字段。更早的版本会为每个字段打印一条 `Unknown field` 警告，因此在这些版本上使用 `--strict` 运行会失败。

### `defaultEnabled`

当用户未在 [`enabledPlugins`](https://code.claude.com/docs/zh-CN/settings-reference#enabledplugins) 中设置时，plugin 是否在启用时启动。默认为 `true`。启用的 plugin 依赖的 plugin 无论如何都会启用启动。marketplace 条目中的相同字段覆盖此字段。

一旦用户的 `enabledPlugins` 条目被写入，它在 plugin 更新中持续存在，因此在后续版本中更改 `defaultEnabled` 不会更改现有用户的设置。

### `dependencies`

必须为此 plugin 启用的 plugin。每个条目是 `"name"`、`"name@marketplace"` 或 `{ "name": "...", "marketplace": "...", "version": "..." }`。裸名称针对此 plugin 自己的 marketplace 解析。参见[依赖约束](https://code.claude.com/docs/zh-CN/plugins/dependencies.md)。

### `settings`

Claude Code 在 plugin 启用时应用的设置。仅 `agent` 和 `subagentStatusLine` 生效；其他键在加载时被删除。plugin 根目录处的 `settings.json` 优先于此键。参见[默认设置](https://code.claude.com/docs/zh-CN/plugins/components.md#default-settings)。

## 组件路径形式

每个组件键接受相对于 plugin 根目录的路径。`hooks`、`mcpServers`、`lspServers` 和 `experimental.monitors` 也接受内联配置，`commands` 也接受对象映射，`mcpServers` 也接受 MCP 包路径和 URL。以下示例显示每个接受的形式一次。有关每个组件在运行时的作用，参见[Plugin 组件](https://code.claude.com/docs/zh-CN/plugins/components.md)。

### 仅路径字段

`agents`、`skills`、`outputStyles`、`workflows` 和 `experimental.themes` 采用一个路径或路径数组。`agents` 条目必须是 `.md` 文件，`skills` 条目必须是目录。其他三个接受目录或文件。

```json theme={null}
{
  "agents": ["./custom-agents/reviewer.md", "./custom-agents/tester.md"],
  "skills": ["./extra-skills/", "."],
  "outputStyles": "./styles/"
}
```

### `commands`

`commands` 采用路径、路径数组或对象映射。路径命名平面 `.md` 命令文件或目录。在对象映射中，每个键在 plugin 前缀后成为命令名称。例如，plugin `deploy-tools` 中的 `"about"` 运行为 `/deploy-tools:about`。

每个值恰好设置 `source` 或 `content` 之一，设置两者或都不设置的条目验证失败。此表中的其他字段是可选的：

| 字段 | 类型 | 描述 |
| :- | :- | :- |
| `source` | string | 命令的 Markdown 文件的路径，相对于 plugin 根目录 |
| `content` | string | 命令体的内联 Markdown，而不是 `source` |
| `description` | string | 为命令显示的描述 |
| `argumentHint` | string | 在命令名称后显示的参数提示，例如 `[file]` |
| `model` | string | 命令的默认模型 |
| `allowedTools` | array of strings | 命令可以使用而无需提示的工具 |

此映射声明一个来自文件的命令和一个来自内联内容的命令：

```json theme={null}
{
  "commands": {
    "status": { "source": "./commands/status.md", "argumentHint": "[env]" },
    "about": { "content": "Explain what this plugin provides." }
  }
}
```

### `hooks`

`hooks` 采用 `.json` 文件路径、与 [`settings.json` 中的 `hooks`](https://code.claude.com/docs/zh-CN/hooks#configuration)相同形状的内联 hooks 对象，或混合两者的数组。有关 hook 事件和处理程序字段，参见[hooks 参考](https://code.claude.com/docs/zh-CN/hooks.md#hook-events)。

hooks 文件将事件映射包装在顶层 `"hooks"` 键中，即 [`hooks/hooks.json`](https://code.claude.com/docs/zh-CN/plugins/components#hooks) 使用的形状。仅包含事件映射而没有该包装的文件无法加载。内联对象就是事件映射本身，没有包装。

Claude Code 在该文件存在时将您声明的内容与 `hooks/hooks.json` 合并。此数组加载一个 hooks 文件并声明一个内联 `PostToolUse` hook：

```json theme={null}
{
  "hooks": [
    "./config/extra-hooks.json",
    {
      "PostToolUse": [
        {
          "matcher": "Write|Edit",
          "hooks": [
            { "type": "command", "command": "\"${CLAUDE_PLUGIN_ROOT}\"/scripts/format.sh" }
          ]
        }
      ]
    }
  ]
}
```

该数组指定的文件在其自身的事件映射外带有 `"hooks"` 包装：

```json config/extra-hooks.json theme={null}
{
  "hooks": {
    "PreToolUse": [
      {
        "matcher": "Bash",
        "hooks": [
          { "type": "command", "command": "\"${CLAUDE_PLUGIN_ROOT}\"/scripts/check-command.sh" }
        ]
      }
    ]
  }
}
```

### `mcpServers`

`mcpServers` 采用 `.json` 文件路径、MCP 包路径或 URL、内联映射或混合它们的数组。有关服务器配置字段，参见[plugin 提供的 MCP 服务器](https://code.claude.com/docs/zh-CN/mcp.md#plugin-provided-mcp-servers)。

Claude Code 首先加载 plugin 根目录处的 `.mcp.json`，然后按顺序加载每个声明的形式。稍后声明的服务器名称替换较早的名称。

`mcpServers` 值采用以下形式之一：

| 形式 | 示例值 | Claude Code 的作用 |
| :- | :- | :- |
| `.json` 文件路径 | `"./mcp/servers.json"` | 将文件读取为 `mcpServers` 映射 |
| MCP 包路径 | `"./bundle.mcpb"` | 将 `.mcpb` 或 `.dxt` 包提取到 plugin 根目录下的 `.mcpb-cache/` 并读取其服务器配置 |
| MCP 包 URL | `"https://example.com/server.mcpb"` | 将包下载到 `.mcpb-cache/`，然后读取它 |
| 内联映射 | `{ "deploy-api": { "command": "node", "args": ["${CLAUDE_PLUGIN_ROOT}/server.js"] } }` | 使用映射作为以名称为键的服务器配置 |

包路径或 URL 必须以 `.mcpb` 或 `.dxt` 结尾。任何其他扩展名验证失败。

### `lspServers`

`lspServers` 采用 `.json` 文件路径、服务器名称到配置的内联映射，或两者的数组。

Claude Code 首先加载 plugin 根目录处的 `.lsp.json`，然后按顺序加载每个声明的配置。稍后声明的服务器名称替换较早的名称。

每个服务器配置是具有这些字段的严格对象。未知键验证失败。

| 字段 | 必需 | 描述 |
| :- | :- | :- |
| `command` | Yes | 语言服务器二进制文件。除非值以 `/` 开头，否则没有空格；将参数放在 `args` 中 |
| `extensionToLanguage` | Yes | 文件扩展名到 LSP 语言 ID 的映射，至少一个条目。键以点开头，例如 `".go"` |
| `args` | No | 传递给服务器的参数 |
| `transport` | No | 通信传输：`stdio`（默认）或 `socket`。Claude Code 接受 `socket` 但在 stdio 上运行每个服务器，因此 stdout 协议规则适用于所有服务器 |
| `env` | No | 服务器进程的环境变量 |
| `initializationOptions` | No | 在初始化请求中发送的选项 |
| `settings` | No | 由 `workspace/didChangeConfiguration` 发送的设置 |
| `workspaceFolder` | No | 服务器的工作区文件夹路径 |
| `startupTimeout` | No | 等待启动的毫秒数，正整数 |
| `shutdownTimeout` | No | 等待正常关闭的毫秒数，正整数。当超时时间过去时，Claude Code 终止服务器进程。未设置时，不适用超时 |
| `requestTimeout` | No | 等待服务器响应请求的毫秒数，正整数。默认为 `60000`，因此服务器始终未响应的请求会在 60 秒后失败。需要 v2.1.288 或更高版本 |
| `restartOnCrash` | No | 服务器崩溃后是否重新启动。默认为 `true`。设置为 `false` 以使崩溃的服务器停止而不是重新启动 |
| `maxRestarts` | No | 放弃前的重新启动尝试，零或更多 |
| `diagnostics` | No | 编辑后是否将诊断推送到上下文。默认为 `true` |

此内联配置为 `.go` 文件运行 `gopls`：

```json theme={null}
{
  "lspServers": {
    "go": {
      "command": "gopls",
      "args": ["serve"],
      "extensionToLanguage": { ".go": "go" }
    }
  }
}
```

有关 Anthropic 作为 plugin 发布的语言服务器以及服务器在运行时的行为，参见[代码智能](https://code.claude.com/docs/zh-CN/plugins/code-intelligence.md)。

### `monitors`

`experimental.monitors` 采用 `.json` 文件路径或内联数组。当您省略该键时，Claude Code 加载 `monitors/monitors.json`（如果存在）。

每个条目是具有这些字段的严格对象。

| 字段 | 必需 | 描述 |
| :- | :- | :- |
| `name` | Yes | 在 plugin 内唯一的标识符 |
| `command` | Yes | Claude Code 在会话工作目录中作为持久后台进程运行的 shell 命令 |
| `description` | Yes | 在任务面板和通知摘要中显示的简短摘要 |
| `when` | No | 使用 `"always"`（默认），monitor 在会话启动和 plugin 重新加载时启动。使用 `"on-skill-invoke:<skill>"`，它在该 skill 首次运行时启动 |

此内联数组声明一个在 `deploy` skill 首次运行时启动的 monitor：

```json theme={null}
{
  "experimental": {
    "monitors": [
      {
        "name": "deploy-status",
        "command": "\"${CLAUDE_PLUGIN_ROOT}\"/scripts/poll-deploy.sh",
        "description": "Deployment status changes",
        "when": "on-skill-invoke:deploy"
      }
    ]
  }
}
```

monitor `command` 不能引用 `${user_config.*}`。参见[通过 shell 运行的字段](#fields-that-run-through-a-shell)。

## 路径规则

manifest 中的每个组件路径相对于 plugin 根目录，必须以 `./` 开头。路径如 `commands/foo.md` 验证失败。`skills` 和 `mcpServers` 各接受该规则之外的一种形式：

* **`skills`**：也接受 `"."`。`"."` 和 `"./"` 都表示 plugin 根目录。在 v2.1.221 之前，`"."` 验证失败，因此当 plugin 必须在较早版本上加载时使用 `"./"`
* **`mcpServers`**：也接受 `https://` 包 URL

`experimental.evals` 不是组件路径，因此本节中的规则不适用于它，而是由 `claude plugin eval` 在运行时检查该值。它指定插件根目录下的一个目录，例如 `"quality/evals"`，可以带或不带 `./` 前缀。如果是数组，则仅使用第一个条目。有关该值接受的内容以及值不可用时会发生什么，请参阅[使用不同的 eval 目录](https://code.claude.com/docs/zh-CN/plugin-evals.md#use-a-different-eval-directory)。

### 包含和存在

每个组件路径必须在 plugin 根目录内解析并且必须存在。`claude plugin validate` 检查每个组件键下的路径：

* **包含**：在 plugin 根目录外解析的路径不加载，`/plugin` **Errors** 选项卡显示 `<component> path escapes plugin directory: <path>`。包含 `..` 的路径是常见情况，`claude plugin validate` 将其报告为 `Path contains ".." which could be a path traversal attempt`
* **存在**：不存在的路径不加载，`/plugin` **Errors** 选项卡显示 `<component> path not found: <path>`。`claude plugin validate` 将其报告为 `Path not found`

对于 `outputStyles`、`lspServers`、`monitors` 和 `themes` 路径，`claude plugin validate` 检查需要 Claude Code v2.1.283 或更高版本。

### 每个键如何与其默认位置结合

每个组件键要么替换其默认位置，要么添加到它，要么与它合并：

* **替换默认值**：`commands`、`agents`、`outputStyles`、`workflows`、`experimental.themes`、`experimental.monitors`。当您设置 `commands` 时，默认 `commands/` 目录不被扫描。要保留默认值并添加更多，明确列出它：`"commands": ["./commands/", "./extras/"]`
* **添加到默认值**：`skills`。`skills/` 目录仍被扫描，列出的目录与它一起加载
* **合并**：`hooks`、`mcpServers`、`lspServers`。默认文件首先加载，manifest 声明的内容合并到它中，如[组件路径形式](#component-path-forms)下所述

如果 plugin 有默认文件夹（如 `commands/`）并且还设置了替换它的 manifest 键，Claude Code 加载 manifest 路径而不是文件夹。`claude plugin list` 和 `/plugin` 界面然后显示警告 `Default <folder>/ folder is ignored because the manifest sets "<key>"`。

要避免警告，将键设置为该文件夹内的路径：`"commands": ["./commands/deploy.md"]` 命名默认文件夹中的文件，不产生警告。

## 用户配置

`userConfig` 声明 Claude Code 在 plugin 启用时提示用户输入的值，因此用户不自己编辑 `settings.json`。

键是由字母、数字和下划线组成的标识符，不能以数字开头。

每个值是具有这些字段的严格对象。未知键验证失败。

| 字段 | 必需 | 描述 |
| :- | :- | :- |
| `type` | Yes | `string`、`number`、`boolean`、`directory` 或 `file` 之一 |
| `title` | Yes | 在配置对话框中显示的标签 |
| `description` | Yes | 在字段下方显示的帮助文本 |
| `required` | No | 如果 `true`，配置对话框不接受空值 |
| `default` | No | 当用户不提供任何内容时使用的值：字符串、数字、布尔值或字符串数组 |
| `options` | No | 对于 `string`，字段接受的值，在 `/config` 中显示为选择器。参见[将字段限制为固定选项](#limit-a-field-to-fixed-options)。需要 Claude Code v2.1.271 或更高版本 |
| `multiple` | No | 对于 `string`，允许字符串数组 |
| `sensitive` | No | 如果 `true`，掩盖输入并将值存储在安全存储中而不是 `settings.json` |
| `min` / `max` | No | `number` 的边界 |

每个启用的 plugin 的每个选项也显示为 `/config` 面板中的一行，除了 `sensitive` 选项和 `multiple` 列表。`/config` 行需要 Claude Code v2.1.269 或更高版本。

此 `userConfig` 声明端点和掩盖的令牌：

```json theme={null}
{
  "userConfig": {
    "api_endpoint": {
      "type": "string",
      "title": "API endpoint",
      "description": "Your team's API endpoint"
    },
    "api_token": {
      "type": "string",
      "title": "API token",
      "description": "API authentication token",
      "sensitive": true
    }
  }
}
```

### 将字段限制为固定选项

在 `userConfig` 字段上设置 `options` 以使用户从固定列表中选择其值。

要将 `tone` 字段限制为三个选项，在 `options` 中列出它们并将 `default` 设置为其中之一：

```json theme={null}
{
  "userConfig": {
    "tone": {
      "type": "string",
      "title": "Tone",
      "description": "Voice for generated replies",
      "options": ["neutral", "warm", "formal"],
      "default": "neutral"
    }
  }
}
```

如果您在任何字段上声明 `options`，Claude Code v2.1.271 之前版本的用户无法加载 plugin。

`options` 适用于不是 `multiple` 或 `sensitive` 的 `string` 字段。将 `default` 设置为列出的值之一，或设置 `required: true` 以便用户必须选择一个。每个选项是 1 到 64 个字符的纯标签，您在 shell 中运行的 `claude plugin validate` 报告它拒绝的任何其他内容。其 `options` 违反这些规则的 plugin 无法加载。

### 值的存储位置

非敏感值保存在用户 `settings.json` 中的 [`pluginConfigs`](https://code.claude.com/docs/zh-CN/settings-reference#pluginconfigs) 下。敏感值转到平台的安全凭证存储。[设置页面](https://code.claude.com/docs/zh-CN/settings-reference.md#pluginconfigs)列出从哪些设置文件读取 `pluginConfigs`。

### 引用保存的值

在 plugin 需要的地方引用保存的值，采用以下两种形式之一：

* **`${user_config.KEY}`**：在 MCP 服务器配置、LSP 服务器配置、[exec 形式](https://code.claude.com/docs/zh-CN/hooks.md#exec-form-and-shell-form) hook `args` 以及 skill 和 agent 内容中替换。在 skill 和 agent 内容中，仅替换非敏感值，敏感值变成占位符
* **`CLAUDE_PLUGIN_OPTION_<KEY>`**：导出到每个选项的 hook 进程，`<KEY>` 大写。shell 形式 hook 为 `api_token` 读取 `$CLAUDE_PLUGIN_OPTION_API_TOKEN`

### 通过 shell 运行的字段

Shell 形式 hook 命令、monitor 命令和 MCP [`headersHelper`](https://code.claude.com/docs/zh-CN/mcp#use-dynamic-headers-for-custom-authentication) 拒绝 `${user_config.*}`。引用它的组件在这些字段之一中失败，出现[错误](https://code.claude.com/docs/zh-CN/errors.md#plugin-command-references-user-config)而不是运行，因为字段的值被传递到会重新解析替换值的 shell。

该表显示值如何可以到达这些字段。

| 字段 | 值如何到达它 |
| :- | :- |
| Shell 形式 hook 命令 | 使用[exec 形式](https://code.claude.com/docs/zh-CN/hooks.md#exec-form-and-shell-form)与 `args`，或从 hook 的环境读取 `CLAUDE_PLUGIN_OPTION_<KEY>` |
| Monitor 命令 | 不通过 Claude Code。Monitor 进程不接收 `CLAUDE_PLUGIN_OPTION_<KEY>`，因此 monitor 脚本必须自己获取值 |
| MCP `headersHelper` | 不通过 Claude Code。helper 的环境携带 `CLAUDE_PLUGIN_ROOT`、`CLAUDE_CODE_MCP_SERVER_NAME` 和 `CLAUDE_CODE_MCP_SERVER_URL` 但没有选项值，因此 helper 脚本必须自己获取值 |

## 频道

`channels` 声明 plugin 提供的消息频道，例如到聊天应用的桥接。当您声明一个时，Claude Code 可以在 plugin 启用时提示频道的配置。有关服务器如何注入消息，参见[频道参考](https://code.claude.com/docs/zh-CN/channels-reference.md#package-as-a-plugin)。

每个条目是绑定到 plugin 的 MCP 服务器之一的严格对象，具有这些字段：

| 字段 | 必需 | 描述 |
| :- | :- | :- |
| `server` | Yes | 此 plugin 的 `mcpServers` 中频道绑定到的 MCP 服务器的键 |
| `displayName` | No | 在配置对话框标题中显示的名称。默认为服务器名称 |
| `userConfig` | No | 要提示的选项，形状与[顶级 `userConfig`](#user-configuration)相同。保存的值替换到服务器 `env` 中的 `${user_config.KEY}` 引用 |

此 manifest 将频道绑定到 plugin 的 `telegram` MCP 服务器，并提示替换到服务器 `env` 中的机器人令牌：

```json theme={null}
{
  "mcpServers": {
    "telegram": {
      "command": "node",
      "args": ["${CLAUDE_PLUGIN_ROOT}/server.js"],
      "env": { "BOT_TOKEN": "${user_config.bot_token}" }
    }
  },
  "channels": [
    {
      "server": "telegram",
      "displayName": "Telegram",
      "userConfig": {
        "bot_token": {
          "type": "string",
          "title": "Bot token",
          "description": "Telegram bot token",
          "sensitive": true
        }
      }
    }
  ]
}
```

## 环境变量

Claude Code 为 plugin 组件提供三个路径变量。在[每个变量解析的位置](#where-each-variable-resolves)下列出的字段中将它们引用为 `${NAME}`，并在接收它们的进程中将它们读取为环境变量。

| 变量 | 解析为 | 用途 |
| :- | :- | :- |
| `${CLAUDE_PLUGIN_ROOT}` | plugin 已安装版本的绝对路径 | 与 plugin 捆绑的脚本、二进制文件和配置文件 |
| `${CLAUDE_PLUGIN_DATA}` | `~/.claude/plugins/data/<id>/`，在首次引用时创建并在 plugin 更新中保留。`<id>` 是 plugin 标识符，其中除字母、数字、`_` 或 `-` 外的每个字符都被替换为 `-` | 已安装的依赖项（如 `node_modules`）、生成的代码和缓存 |
| `${CLAUDE_PROJECT_DIR}` | 项目根目录 | 项目本地脚本和配置文件 |

`${CLAUDE_PLUGIN_ROOT}` 在 plugin 更新时改变，因此不要在那里写入状态。有关根目录移动的位置和旧目录何时被清理，参见[加载页面](https://code.claude.com/docs/zh-CN/plugins/loading.md)。

默认情况下，当您从最后一个安装它的地方卸载 plugin 时，Claude Code 会删除 `${CLAUDE_PLUGIN_DATA}` 目录。有关 `--keep-data` 和其他保留它的情况，参见 [plugin 卸载](https://code.claude.com/docs/zh-CN/plugins/cli-reference.md#plugin-uninstall)。

### 每个变量解析的位置

在每个 plugin 组件中，`${...}` 引用在特定字段中内联解析，某些组件也在其进程环境中接收变量：

| Plugin 组件 | `${...}` 解析的字段 | 导出到进程 |
| :- | :- | :- |
| Hook 命令 | 在 `command` 和 `args` 中的任何地方 | `CLAUDE_PLUGIN_ROOT`、`CLAUDE_PLUGIN_DATA`、`CLAUDE_PROJECT_DIR` 和 `CLAUDE_PLUGIN_OPTION_<KEY>` |
| Monitor 命令 | 在 `command` 中的任何地方 | 未导出 |
| MCP `stdio` 服务器 | `command`、`args`、`env` | `CLAUDE_PLUGIN_ROOT`、`CLAUDE_PLUGIN_DATA` |
| MCP `http`、`sse`、`ws` 服务器 | `url`、`headers`、`headersHelper` | 不适用 |
| LSP 服务器 | `command`、`args`、`env`、`workspaceFolder` | `CLAUDE_PLUGIN_ROOT`、`CLAUDE_PLUGIN_DATA`、`CLAUDE_PROJECT_DIR` |
| Skill、command 和 agent 内容 | Markdown 体中的任何地方 | 不适用 |

变量不存在于 Claude 通过 Bash 工具在主会话或子代理中运行的命令的环境中。在 skill、command 和 agent 内容中，在 Markdown 体中写入 `${...}` 引用，Claude Code 在加载内容时内联替换路径。

### 引用和路径分隔符

保持每个替换的路径为单个参数：

* **Hook 命令**：使用[exec 形式](https://code.claude.com/docs/zh-CN/hooks.md#exec-form-and-shell-form)与 `args` 以便每个路径是一个没有引用的参数
* **Shell 形式 hooks 和 monitor 命令**：用双引号包装变量，以便带空格的路径保持为一个单词

如果您在 hooks 文件中的 shell 形式命令中将这些变量之一留在引号外，`claude plugin validate` 会发出警告，除非 hook 将 [`shell`](https://code.claude.com/docs/zh-CN/hooks#command-hook-fields) 设置为 `"powershell"`。

此 shell 形式 hook 运行与 plugin 捆绑的脚本：

```json theme={null}
{
  "hooks": {
    "PostToolUse": [
      {
        "hooks": [
          {
            "type": "command",
            "command": "\"${CLAUDE_PLUGIN_ROOT}\"/scripts/process.sh"
          }
        ]
      }
    ]
  }
}
```

在 Windows 上，大多数替换的路径使用正斜杠，例如 `C:/Users/you/...`。在 [exec 形式 hook](https://code.claude.com/docs/zh-CN/hooks.md#exec-form-and-shell-form) 以及通过 PowerShell 运行的 hook 中，它们会保留原生的反斜杠；未安装 Git Bash 时，PowerShell 是 Windows 上的默认 shell。

## 标准布局

每个组件类型在 plugin 根目录下有默认位置，当 manifest 不指向其他位置时使用。

| 组件 | 默认位置 | 内容 |
| :- | :- | :- |
| Manifest | `.claude-plugin/plugin.json` | Plugin 元数据和配置。可选 |
| Skills | `skills/` | 每个 skill 一个 `<name>/SKILL.md`。具有 `SKILL.md` 在其根目录、没有 `skills/` 和没有 `skills` 键的 plugin 加载为单个 skill |
| Commands | `commands/` | 平面 Markdown 命令文件。对于新 plugin 更喜欢 `skills/` |
| Agents | `agents/` | Agent Markdown 文件。子文件夹是[agent 名称](https://code.claude.com/docs/zh-CN/plugins/components.md#agents)的一部分 |
| Hooks | `hooks/hooks.json` | Hook 配置 |
| MCP 服务器 | `.mcp.json` | MCP 服务器定义 |
| LSP 服务器 | `.lsp.json` | LSP 服务器配置 |
| 输出样式 | `output-styles/` | 输出样式 Markdown 文件 |
| Workflows | `workflows/` | Workflow `.js` 文件 |
| 主题 | `themes/` | 主题 JSON 文件 |
| Monitors | `monitors/monitors.json` | monitors 数组 |
| 可执行文件 | `bin/` | 此处的文件在 plugin 启用时位于 Bash 工具的 `PATH` 上，因此 Claude 将它们作为裸命令运行。claude.ai 和 Cowork 不安装具有此目录的 plugin，包括您[通过 claude.ai 组织设置分发](https://claude.com/docs/plugins/org-sync#keep-executables-out-of-the-top-level-bin-directory)的 plugin |
| 设置 | `settings.json` | 在 plugin 启用时应用的 `agent` 和 `subagentStatusLine` 默认值 |

使用每个默认位置的 plugin，加上其 hooks 调用的 `scripts/` 文件夹，布局如下：

```text theme={null}
deploy-tools/
├── .claude-plugin/
│   └── plugin.json
├── skills/
│   └── deploy/
│       └── SKILL.md
├── commands/
│   └── status.md
├── agents/
│   └── reviewer.md
├── hooks/
│   └── hooks.json
├── monitors/
│   └── monitors.json
├── output-styles/
│   └── terse.md
├── themes/
│   └── dracula.json
├── workflows/
│   └── release-audit.js
├── bin/
│   └── deploy-tool
├── scripts/
│   └── format.sh
├── settings.json
├── .mcp.json
└── .lsp.json
```

要点击此布局并阅读每个文件的作用，打开[plugin 浏览器](https://code.claude.com/docs/zh-CN/plugins/components.md#explore-the-plugin-directory)。

plugin 根目录处的 `CLAUDE.md` 不作为上下文加载，`claude plugin validate` 在找到一个时发出警告。要包含加载到 Claude 上下文中的说明，将它们放在 skill 中。

## Marketplace 条目和 manifest

[市场条目](https://code.claude.com/docs/zh-CN/plugins/marketplace-reference.md)接受[其自己的字段](https://code.claude.com/docs/zh-CN/plugins/marketplace-reference.md#plugin-entries)（包括 `strict`），以及此页面上除[目录列表字段](#directory-listing-fields)之外的每个字段。

`strict` 字段决定条目是否可以向具有自己 `plugin.json` 的 plugin 添加组件。它默认为 `true`。

### 条目字段如何与 `plugin.json` 结合

条目要么充当 manifest，要么向其添加组件，要么与其冲突：

* **没有 `plugin.json`**：条目是 manifest，无论 `strict` 如何。条目 `hooks` 仅以内联对象形式加载。对于文件路径或数组，`/plugin` **Errors** 选项卡显示 `not yet supported in a marketplace entry` 错误
* **`plugin.json` 存在，`strict` 未设置或 `true`**：Claude Code 加载 manifest 并将条目的 `commands`、`agents`、`skills`、`outputStyles` 和 `themes` 附加到它。对于 `hooks`，条目对事件的匹配器替换 manifest 对该相同事件的匹配器，仅 manifest 声明的事件保留其
* **`plugin.json` 存在，`strict: false`**：声明 `commands`、`agents`、`skills`、`hooks`、`outputStyles` 或 `themes` 的条目是冲突，plugin 加载失败，出现 `Plugin <name> has conflicting manifests`

当[其 `source` 是 marketplace 根目录的 marketplace 条目](https://code.claude.com/docs/zh-CN/plugins/marketplace-reference)列出特定 `skills` 子目录时，仅这些子目录加载，plugin 的默认 `skills/` 目录不被扫描。manifest 中的 `skills` 键改为[添加到默认值](#how-each-key-combines-with-its-default-location)。

### 元数据优先级

某些元数据字段有固定的优先级，无论 `strict` 如何：

* **`defaultEnabled` 和显示字段**：条目的 `defaultEnabled` 和其[显示字段](https://code.claude.com/docs/zh-CN/plugins/marketplace-reference.md#entry-and-plugin-json)（如 `displayName`）覆盖 manifest 的
* **`version`**：manifest 的 `version` 覆盖条目的
* **`name`**：当条目在与 manifest 不同的 `name` 下列出 plugin 时，`enabledPlugins` 使用条目名称，组件在 manifest 名称下命名空间

有关完整的优先级表，参见[严格模式](https://code.claude.com/docs/zh-CN/plugins/marketplace-reference.md)。

## 后续步骤

* [向 plugin 添加组件](https://code.claude.com/docs/zh-CN/plugins/components.md)：每个组件在运行时的作用，带有验证的示例
* [Marketplace 参考](https://code.claude.com/docs/zh-CN/plugins/marketplace-reference.md)：marketplace 可以为您的 plugin 设置的条目字段
* [Plugin 命令参考](https://code.claude.com/docs/zh-CN/plugins/cli-reference.md#plugin-validate)：`claude plugin validate` 标志和输出
* [Plugin 故障排除](https://code.claude.com/docs/zh-CN/plugins/troubleshooting.md#claude-plugin-validate-reports-errors)：每条验证消息及其修复
