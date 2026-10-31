> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# 通过 MCP 将 Claude Code 连接到工具

> 了解如何使用 Model Context Protocol 将 Claude Code 连接到您的工具。

Claude Code 可以通过 [Model Context Protocol (MCP)](https://modelcontextprotocol.io/introduction)（一个用于 AI 工具集成的开源标准）连接到数百个外部工具和数据源。MCP 服务器为 Claude Code 提供对您的工具、数据库和 API 的访问权限。

当您发现自己从另一个工具（如问题跟踪器或监控仪表板）复制数据到聊天中时，请连接一个服务器。连接后，Claude 可以直接读取和操作该系统，而不是从您粘贴的内容中工作。

如果您是第一次连接服务器，请从 [MCP 快速入门](https://code.claude.com/docs/zh-CN/mcp-quickstart.md) 开始，获取分步演练。本页面是完整参考。

## 使用 MCP 可以做什么

连接 MCP 服务器后，您可以要求 Claude Code：

* **从问题跟踪器实现功能**："添加 JIRA 问题 ENG-4521 中描述的功能，并在 GitHub 上创建 PR。"
* **分析监控数据**："检查 Sentry 和 Statsig 以检查 ENG-4521 中描述的功能的使用情况。"
* **查询数据库**："根据我们的 PostgreSQL 数据库，查找使用功能 ENG-4521 的 10 个随机用户的电子邮件。"
* **集成设计**："根据在 Slack 中发布的新 Figma 设计更新我们的标准电子邮件模板"
* **自动化工作流**："创建 Gmail 草稿，邀请这 10 个用户参加关于新功能的反馈会议。"
* **对外部事件做出反应**：MCP 服务器也可以充当[频道](https://code.claude.com/docs/zh-CN/channels.md)，将消息推送到您的会话中，因此当您不在时，Claude 可以对 Telegram 消息、Discord 聊天或 webhook 事件做出反应。

## 查找和构建 MCP 服务器

在 [Anthropic Directory](https://claude.ai/directory) 中浏览已审核的连接器。Directory 连接器使用与 Claude Code 相同的 MCP 基础设施，因此您可以使用 `claude mcp add` 添加列出的任何远程服务器。

> [!WARNING]
>
> 在连接每个服务器之前，请验证您信任该服务器。获取外部内容的服务器可能会使您面临 [提示注入风险](https://code.claude.com/docs/zh-CN/security.md#protect-against-prompt-injection)。

要构建您自己的服务器，请参阅 [MCP 服务器指南](https://modelcontextprotocol.io/docs/develop/build-server) 了解协议基础知识，以及 [Claude 连接器构建文档](https://claude.com/docs/connectors/building) 了解身份验证、测试和 Directory 提交。

您也可以使用官方的 [`mcp-server-dev` plugin](https://github.com/anthropics/claude-plugins-official/tree/main/plugins/mcp-server-dev) 让 Claude 为您搭建服务器。

**1. 安装插件**

在 VS Code 扩展或桌面应用中，请按照 [安装插件](https://code.claude.com/docs/zh-CN/plugins/install.md#install-a-plugin) 操作，而不是执行此步骤。在终端中，运行 `claude` 启动 Claude Code，然后在其提示符处输入以下内容：

```
/plugin install mcp-server-dev@claude-plugins-official
```

如果安装失败，请匹配 Claude Code 报告的消息：

* `Marketplace "claude-plugins-official" not found`：使用 `/plugin marketplace add anthropics/claude-plugins-official` 添加该市场，然后重试安装。
* `Plugin "<name>" not found in marketplace "claude-plugins-official"`：您本地的市场目录副本可能已过期。使用 `/plugin marketplace update claude-plugins-official` 刷新它，然后重试安装。如果重试后显示相同的消息，请参阅 [Plugin not found in marketplace](https://code.claude.com/docs/zh-CN/plugins/troubleshooting.md#plugin-not-found-in-marketplace)。

如果安装摘要报告 `Run /reload-plugins to apply.`，Claude Code 会为您运行该重新加载。如果重新加载警告您的下一条消息会重新读取对话，请运行 `/reload-plugins --force`。

**2. 运行构建 skill**

```
/mcp-server-dev:build-mcp-server
```

Claude 会询问您的用例，并搭建一个远程 HTTP 或本地 stdio 服务器。

## 安装 MCP 服务器

MCP 服务器可以根据您的需求以多种方式进行配置：

### 选项 1：添加远程 HTTP 服务器

HTTP 服务器是连接到远程 MCP 服务器的推荐选项。这是云服务最广泛支持的传输方式。

```bash theme={null}
# 基本语法
claude mcp add --transport http <name> <url>

# 真实示例：连接到 Notion
claude mcp add --transport http notion https://mcp.notion.com/mcp

# 带有 Bearer 令牌的示例
claude mcp add --transport http secure-api https://api.example.com/mcp \
  --header "Authorization: Bearer your-token"
```

通过 `.mcp.json`、`~/.claude.json` 或 `claude mcp add-json` 中的 JSON 配置 MCP 服务器时，`type` 字段接受 `streamable-http` 作为 `http` 的别名。MCP 规范对此传输使用名称 `streamable-http`，因此从服务器文档复制的配置无需修改即可工作。

具有 `url` 但没有 `type` 的 JSON 条目是配置错误，因为 Claude Code 将没有 `type` 的条目读取为 stdio 服务器。Claude Code 跳过该服务器并报告 `MCP server "<name>" has a "url" but no "type"; add "type": "http" (or "sse" / "ws") to this entry`。

只有 SDK 主机应用程序（例如 [Agent SDK](https://code.claude.com/docs/zh-CN/agent-sdk/mcp.md) 应用程序或 [桌面应用](https://code.claude.com/docs/zh-CN/desktop.md)）可以注册进程内 `"type": "sdk"` 服务器。Claude Code 跳过 `.mcp.json`、`~/.claude.json` 或设置中的 `"type": "sdk"` 条目，并报告 `Skipped — MCP server "<name>" declares type "sdk", which only an SDK host application can register`。

> [!NOTE]
>
> 在 `--output-format stream-json` 运行中，Claude Code 还会在 `system/init` 事件的 [`mcp_server_errors` 字段](https://code.claude.com/docs/zh-CN/headless#stream-responses)中报告被跳过的 `--mcp-config` 条目，以便脚本检测到该服务器从未加载。

### 选项 2：添加远程 SSE 服务器

> [!WARNING]
>
> SSE（Server-Sent Events）传输已弃用。请在可用的地方使用 HTTP 服务器。

某些服务仍然仅公开 SSE 端点。使用与 [HTTP 服务器](#option-1-add-a-remote-http-server) 相同的 `claude mcp add --transport http <name> <url>` 命令添加这些。Claude Code 首先尝试 HTTP 传输，当服务器不接受时切换到 SSE。自动切换需要 Claude Code v2.1.265 或更高版本。

在较早的版本上，或直接通过 SSE 连接，请改为传递 `--transport sse`：

```bash theme={null}
# 基本语法
claude mcp add --transport sse <name> <url>

# 真实示例：连接到 Asana
claude mcp add --transport sse asana https://mcp.asana.com/sse

# 带有身份验证标头的示例
claude mcp add --transport sse private-api https://api.company.com/sse \
  --header "X-API-Key: your-key-here"
```

### 选项 3：添加本地 stdio 服务器

Stdio 服务器作为本地进程在您的机器上运行。它们非常适合需要直接系统访问或自定义脚本的工具。

Claude Code 在生成的服务器的环境中设置 `CLAUDE_PROJECT_DIR` 为项目根目录，以便您的服务器可以解析项目相对路径，而不依赖于工作目录。这与 hook 在其 `CLAUDE_PROJECT_DIR` 变量中接收的目录相同。从服务器进程内读取它，例如 Node 中的 `process.env.CLAUDE_PROJECT_DIR` 或 Python 中的 `os.environ["CLAUDE_PROJECT_DIR"]`。

`CLAUDE_PROJECT_DIR` 是稳定的项目根目录，在会话中添加或删除工作目录时不会更改。限制自己的文件系统访问到一组允许目录的服务器应该改为实现 MCP `roots/list` 请求。Claude Code 使用会话的启动目录加上您通过 `--add-dir`、`/add-dir` 或 `additionalDirectories` 设置授予的每个 [额外工作目录](https://code.claude.com/docs/zh-CN/permissions.md#working-directories) 来回答 `roots/list`。当该集合更改时，Claude Code 发送 `notifications/roots/list_changed`。

此变量在服务器的环境中设置，而不是在 Claude Code 自己的环境中，因此通过项目作用域的 `.mcp.json` 条目或 `~/.claude.json` 中的本地或用户作用域服务器条目中的 `command` 或 `args` 中的 `${VAR}` 扩展来引用它需要默认值，例如 `${CLAUDE_PROJECT_DIR:-.}`。插件提供的 MCP 配置直接替换 `${CLAUDE_PROJECT_DIR}` 并且不需要默认值。

```bash theme={null}
# 基本语法
claude mcp add [options] <name> -- <command> [args...]

# 真实示例：添加 Airtable 服务器
claude mcp add --env AIRTABLE_API_KEY=YOUR_KEY --transport stdio airtable \
  -- npx -y airtable-mcp-server
```

> [!NOTE]
>
> 对于 stdio 服务器，`--`（双破折号）将 Claude 自己的选项（例如 `--transport`、`--env` 和 `--scope`）与运行服务器的命令和参数分开。`--` 之后的所有内容都原封不动地传递给服务器。
>
> 例如：
>
> * `claude mcp add --transport stdio myserver -- npx server` → 运行 `npx server`
> * `claude mcp add --env KEY=value --transport stdio myserver -- python server.py --port 8080` → 使用环境中的 `KEY=value` 运行 `python server.py --port 8080`
>
> 没有 `--`，Claude Code 会尝试将服务器的标志（如上面的 `--port`）解析为自己的选项。
>
> `--env` 接受多个 `KEY=value` 对。如果服务器名称直接跟在 `--env` 之后，CLI 会将该名称读取为另一对并拒绝它，因此在 `--env` 和服务器名称之间至少放置一个其他选项，例如 `--transport stdio`。

#### Claude Code 如何停止 stdio 服务器

当 Claude Code 断开 stdio 服务器时（例如您的会话结束时），它会向服务器进程发送信号，并短暂等待其退出，然后再强制停止它。如果您的服务器在关闭时保存状态或删除锁文件，请针对您的平台处理该信号，并在该时间内完成：

* **在 macOS 和 Linux 上**：Claude Code 发送 `SIGINT`，如果进程在 100 毫秒后仍在运行，则发送 `SIGTERM`，再过 400 毫秒后发送 `SIGKILL`。
* **在 Windows 上**：Claude Code 关闭服务器的 stdin，如果进程在 300 毫秒后仍在运行，则结束服务器的进程树。

在 v2.1.296 之前，Windows 上的 Claude Code 会立即结束服务器的进程树，而不先关闭 stdin 或等待。

### 选项 4：添加远程 WebSocket 服务器

WebSocket 服务器保持持久的双向连接，适合主动向 Claude 推送事件的远程 MCP 服务器。当您的服务器仅响应请求时，请改用 HTTP，因为 HTTP 支持 OAuth 和 `claude mcp add --transport` 标志，而 WebSocket 都不支持。

在 `.mcp.json` 中或使用 `claude mcp add-json` 配置 WebSocket 服务器：

```bash theme={null}
claude mcp add-json events-server \
  '{"type":"ws","url":"wss://mcp.example.com/socket","headers":{"Authorization":"Bearer YOUR_TOKEN"}}'
```

`type: "ws"` 条目接受与 `http` 相同的 `url`、`headers`、`headersHelper`、`timeout` 和 `alwaysLoad` 字段。身份验证仅限标头，因此在 `headers` 中传递静态令牌或在连接时使用 [`headersHelper`](#use-dynamic-headers-for-custom-authentication) 生成一个。`claude mcp add --transport` 标志不接受 `ws`。

### 从为另一个客户端编写的设置说明添加服务器

MCP 服务器不特定于 Claude Code，因此服务器的设置说明可能是为 Claude Desktop、Cursor 或另一个 MCP 客户端编写的，并且不提供 `claude mcp add` 命令。要添加服务器，请在这些说明中查找 URL、启动命令或 JSON 块：

* **URL**，例如 `https://mcp.example.com/mcp`：服务器是远程的。
* **启动命令**，例如 `npx -y @example/mcp-server`：服务器在您的机器上运行。
* **`mcpServers` JSON 块**：为另一个客户端的设置文件编写的配置。

每一个都是 [安装 MCP 服务器](#installing-mcp-servers) 中四个选项之一接受的输入。找到您下面拥有的形状，将其转换为 Claude Code 接受的命令。除非您添加 `--scope project` 或 `--scope user`，否则每个命令都写入 [本地作用域](#local-scope)。

#### 从 URL 添加服务器

URL 表示服务器是远程的。对于 `https://` 端点，使用 `--transport http` 添加它，或在说明说端点使用 SSE 时遵循 [选项 2](#option-2-add-a-remote-sse-server)。对于 `wss://` 端点，改为使用 [选项 4](#option-4-add-a-remote-websocket-server)，因为 `--transport` 不接受 `ws`：

```bash theme={null}
claude mcp add --transport http example https://mcp.example.com/mcp
```

如果说明还提供 API 密钥或令牌标头，请使用 `--header` 传递它，如 [选项 1](#option-1-add-a-remote-http-server) 所示。

#### 从 `npx`、`uvx` 或二进制命令添加服务器

启动命令表示服务器作为本地 stdio 进程运行。将整个命令放在 `--` 之后，以便 Claude Code 将标志（如 `-y`）传递给启动服务器的命令，而不是将它们读取为自己的选项。使用 `--env` 传递说明要求的任何环境变量，在服务器名称之后和 `--` 之前：

```bash theme={null}
claude mcp add example --env API_KEY=your-key -- npx -y @example/mcp-server
```

[选项 3](#option-3-add-a-local-stdio-server) 完整涵盖 `--` 分隔符。

#### 从 `mcpServers` JSON 块添加服务器

为另一个 MCP 客户端（例如 Claude Desktop）编写的 `mcpServers` 块使用 Claude Code 读取的包装器键和条目形状。将 `mcpServers` 内的对象（而不是包装器）传递给 `claude mcp add-json`。两种条目需要先修复：

* **`url` 没有 `type`**：添加 `"type": "http"`、`"type": "sse"` 或 `"type": "ws"` 以匹配端点。Claude Code 将没有 `type` 的条目读取为 stdio 服务器，因此没有 `type` 的 `url` 条目会失败。
* **键包含除字母、数字、连字符和下划线以外的字符**：选择仅使用这些字符的服务器名称。否则键就是服务器名称。

例如，此块：

```json theme={null}
{
  "mcpServers": {
    "example": {
      "command": "npx",
      "args": ["-y", "@example/mcp-server"]
    }
  }
}
```

变成此命令：

```bash theme={null}
claude mcp add-json example '{"command":"npx","args":["-y","@example/mcp-server"]}'
```

[从 JSON 配置添加 MCP 服务器](#add-mcp-servers-from-json-configuration) 涵盖 `add-json` 的 shell 转义和 `--scope` 标志。要与您的团队共享服务器，请改为添加 `--scope project`，或在项目根目录的 `.mcp.json` 中的 `mcpServers` 下添加条目并提交它。[项目作用域](#project-scope) 涵盖 Claude Code 如何加载和批准该文件。

每个 `claude mcp add` 和 `claude mcp add-json` 命令在成功时打印 `Added ...` 行。要检查 Claude Code 是否已连接，请运行 `claude mcp get <name>`；[服务器状态](#server-status) 涵盖它显示的状态和 `.mcp.json` 服务器的批准步骤。

### 管理您的服务器

配置后，您可以使用这些命令管理您的 MCP 服务器：

```bash theme={null}
# 列出所有配置的服务器
claude mcp list

# 获取特定服务器的详细信息
claude mcp get notion

# 删除服务器
claude mcp remove notion

# （在 Claude Code 中）检查服务器状态
/mcp
```

移除远程服务器时，Claude Code 还会删除它为该服务器存储的 OAuth 令牌和客户端注册信息。Claude Code 不会在签发这些令牌的授权服务器上撤销令牌。如果授权服务器支持撤销，要撤销令牌，请在运行 `claude mcp remove <name>` 之前，先在 shell 中运行 [`claude mcp logout <name>`](#authenticate-from-the-command-line)。

#### 服务器状态

`claude mcp add` 通过打印 `Added ...` 行确认成功添加，这意味着配置已写入。如果命令改为打印 `was not saved` 消息，请参阅 [MCP 服务器未保存或删除](https://code.claude.com/docs/zh-CN/errors.md#mcp-server-was-not-saved-or-removed)；对于 `may not have been saved` 消息，请参阅 [MCP 服务器可能未保存或删除](https://code.claude.com/docs/zh-CN/errors.md#mcp-server-may-not-have-been-saved-or-removed)。

`claude mcp list` 在它列出的每个服务器旁边显示健康状态，例如 `✔ Connected`、`! Needs authentication` 或 `✘ Failed to connect`。失败状态意味着 Claude Code 无法连接到该服务器，而不是列表命令失败。

此列表中的状态报告配置决策而不是连接尝试，因此 Claude Code 在不连接到服务器的情况下打印它们：

* ``⏸ Pending approval (run `claude` to approve)``：来自 `.mcp.json` 且您尚未批准的项目作用域服务器。Claude Code 会在 `claude mcp list` 和 `claude mcp get <name>` 中都显示它。以交互方式运行 `claude` 以审查并批准它。
* `✘ Rejected (see disabledMcpjsonServers in settings)`：由 [`disabledMcpjsonServers`](https://code.claude.com/docs/zh-CN/settings-reference#disabledmcpjsonservers) 条目拒绝的 `.mcp.json` 服务器。Claude Code 仅在 `claude mcp get <name>` 中显示它。
* `⊘ Disabled for this project (re-enable via /mcp)`：项目的 [`disabledMcpServers`](#disable-a-server-without-removing-it) 列表命名的服务器。Claude Code 在 `claude mcp list` 和 `claude mcp get <name>` 中都显示它。从 `/mcp` 面板重新打开服务器。
* `✘ Blocked by enterprise managed policy`：被您组织的某项策略拒绝的服务器。Claude Code 会在 `claude mcp list` 和 `claude mcp get <name>` 中都显示它。被[托管 MCP 配置](https://code.claude.com/docs/zh-CN/managed-mcp.md#how-restrictions-appear-to-users)中的允许列表、拒绝列表或其他限制阻止的服务器，会在这两个命令中都被隐藏，而不是以此状态显示。请询问您的管理员是哪项策略阻止了该服务器。

#### 项目服务器批准和工作区信任

`claude mcp list` 和 `claude mcp get` 仅从未签入仓库的设置文件中读取 `.mcp.json` 批准，直到您通过在其中运行 `claude` 并接受工作区信任对话框来信任工作区。克隆的仓库无法批准自己的服务器：提交到项目的 `.claude/settings.json` 的 [`enableAllProjectMcpServers`](https://code.claude.com/docs/zh-CN/settings-reference#enableallprojectmcpservers) 或 [`enabledMcpjsonServers`](https://code.claude.com/docs/zh-CN/settings-reference#enabledmcpjsonservers) 在不受信任的文件夹中被忽略，服务器保持在 `⏸ Pending approval` 而不是被连接和健康检查。

这些来源的批准仍然适用于不受信任的文件夹：

* 您的用户 `~/.claude/settings.json`
* 托管设置
* 使用 `--settings` 传递的设置

Claude Code 也应用来自未跟踪的 `.claude/settings.local.json` 的批准，但它运行 git 来检查文件是否被跟踪，并且仅在 [受信任的文件夹](https://code.claude.com/docs/zh-CN/permissions.md#project-allow-rules-and-workspace-trust) 中运行该检查。在您从未信任的文件夹中，Claude Code 等待信任对话框后才应用文件的批准，除非该文件夹是您自己的配置主目录：您的主目录，或其 `.claude` 您已设置为 [`CLAUDE_CONFIG_DIR`](https://code.claude.com/docs/zh-CN/env-vars) 的目录。在 v2.1.207 之前，Claude Code 即使在您从未信任的文件夹中也应用来自未跟踪的 `.claude/settings.local.json` 的批准。

任何设置文件中的 `disabledMcpjsonServers` 条目仍然拒绝服务器。

#### 服务器状态详情

在 `/mcp` 中（包括服务器的菜单）和 [`/plugin`](https://code.claude.com/docs/zh-CN/plugins/install) 管理器中，您之前使用过的远程 HTTP 或 SSE 服务器可以显示 `cached` 状态，例如 `cached 2h ago · connects on first use · 5 tools`。Claude Code 从发现缓存加载了服务器的工具列表，该缓存保存在上一个会话中，而不是在启动时连接，Claude Code 在 Claude 首次调用服务器的工具之一时连接服务器。工具从您的第一条消息开始可用，因此您无需执行任何操作。发现缓存及其 `cached` 状态需要 Claude Code v2.1.221 或更高版本。

发现缓存默认关闭，除非逐步推出已为您的帐户启用它。设置 [`MCP_DISCOVERY_CACHE=1`](https://code.claude.com/docs/zh-CN/env-vars) 打开它，或设置为 `0` 即使推出已启用它也保持关闭。在 v2.1.238 之前，缓存默认打开。

当您从 `/mcp` 中的服务器菜单选择 **Disable** 或 **Clear authentication** 时，Claude Code 也会丢弃该服务器的缓存条目。**Reconnect** 在已连接或失败的服务器上也会丢弃它；在 `cached` 服务器上，**Reconnect** 会立即连接服务器并保留条目。丢弃条目后，Claude Code 下次连接到服务器时，会从服务器而不是从缓存获取工具列表。

##### 连接失败与未配置的服务器

当服务器状态为 `✘ Failed to connect` 时，`claude mcp list` 会在该状态行后附加失败详情，而 `claude mcp get <name>` 会在 `Issue:` 行中显示它。详情包括 HTTP 状态或错误代码，以及服务器返回的任何错误文本。`/mcp` 中服务器的详情视图会在其 `Issue:` 行中包含同样由服务器报告的文本。

Claude Code 会从这些详情中删除类似凭据的文本，并且绝不包含展开后的服务器 URL，因为它可能携带机密信息。Claude Code 不会在 `✘ Connection error` 状态后附加任何详情，因为它在那里打印的异常文本可能嵌入该 URL。

当您从 `/mcp` 完成身份验证且连接仍然因 HTTP 状态或传输错误代码失败时，Claude Code 在尝试后打印的消息中添加该代码和服务器 URL 的来源。来源是方案和主机，加上 URL 命名的端口（如果有），例如 `https://mcp.example.com`。

* 路径和查询永远不会出现在该消息中。
* 对于本地、项目或用户 [作用域](#mcp-installation-scopes) 中的服务器或托管 MCP 配置中的服务器，来源显示该配置中写入的主机，因此主机中的 `${VAR}` 引用在消息中不会展开。
* 对于没有状态或错误代码的失败，Claude Code 显示错误文本而不显示来源。

配置为空 `url` 的远程服务器在 `/mcp`、`claude mcp list` 和 [`/plugin`](https://code.claude.com/docs/zh-CN/plugins/install) 管理器中显示为 `not configured`，Claude Code 不尝试连接到它。插件可以包含这样的占位符条目，用于您稍后配置的连接器，因此 Claude Code 不将其报告为错误或设置问题。`/mcp` 中的服务器详情视图显示 `No URL configured for this server`；设置条目的 `url` 以连接它。

#### 配置警告

Claude Code 警告以下配置问题。每个条目说明 Claude Code 检查什么以及如何清除警告：

* **隐藏的空格**：当 MCP 配置值携带隐藏的前导或尾随空格时，Claude Code 发出警告，这通常来自粘贴带有尾随换行符的令牌。Claude Code 检查 `command`、`url`、每个 `args` 条目以及 `env` 和 `headers` 下的值和键名称。Claude Code 在 `claude mcp list` 输出和 `/mcp` 中显示警告，命名受影响的字段而不回显其值，例如 `Leading or trailing whitespace in: headers.Authorization`。Claude Code 不修剪空格并完全按照写入的方式使用值，因此请编辑配置以删除它。
* **在多个作用域中使用相同名称**：如果您在多个 [作用域](#mcp-installation-scopes) 中定义相同的服务器名称，具有不同的端点，Claude Code 在 `claude mcp list` 输出和 `/mcp` 中警告冲突。Claude Code 按端点存储 OAuth 登录，因此当您在一个项目中对加载的定义进行身份验证时，您仍然需要在加载不同定义的项目中单独登录。保留您想要的端点并使用 `claude mcp remove <name> --scope <scope>` 删除其他端点。在警告中，Claude Code 按您的配置中所写的形式引用每个作用域的端点，[`${VAR}` 引用](#environment-variable-expansion-in-mcp-json) 保持未展开，因此它永远不显示已解析的值，例如 API 密钥。
* **保留名称**：Claude Code 保留其内置服务器的名称，包括 `workspace`、`claude-in-chrome`、`computer-use`、`Claude Preview` 和 `Claude Browser`。如果您的配置定义具有保留名称的服务器，Claude Code 在加载时跳过它并显示警告，要求您重命名它。`claude mcp add` 拒绝保留名称并出现错误。`Claude Preview` 和 `Claude Browser` 都命名 [Claude Code 桌面应用的预览窗格](https://code.claude.com/docs/zh-CN/desktop.md#preview-your-app) 使用的内置服务器。
* **缺少环境变量**：如果服务器配置中的 [`${VAR}` 引用](#environment-variable-expansion-in-mcp-json) 命名未设置且没有 `:-default` 的变量，Claude Code 在 `claude mcp list` 输出和 `/mcp` 中警告，命名变量，并仍然使用 `${VAR}` 文本未展开加载服务器。设置变量或添加 `${VAR:-default}` 备用值。在远程服务器的 `url` 和 `headers` 中，某些凭据变量 [读取为空](#credential-variables-that-read-as-empty) 而不显示警告。

#### 工具可用性

`/mcp` 面板在每个连接的服务器旁边显示工具计数，并标记声称工具能力但不公开工具的服务器。

如果您的请求需要来自仍在后台连接的服务器的工具，Claude 会在继续之前等待该服务器。等待的方式取决于您的配置：

* **使用 [工具搜索](#scale-with-mcp-tool-search)（默认）**：等待发生在 `ToolSearch` 调用内。
* **不使用工具搜索**：Claude 改为使用 `WaitForMcpServers` 工具。不使用工具搜索的配置包括 `ENABLE_TOOL_SEARCH=false`，以及 Google Cloud 的 Agent Platform 上早于 Claude Opus 4.5、Claude Sonnet 4.5 和 Claude Haiku 4.5 的模型。对于 `ANTHROPIC_BASE_URL` 网关，请参阅 [通过 LLM 网关使用工具搜索](#tool-search-through-a-gateway)。
* **在 Microsoft Foundry [部署托管在 Azure 上](https://platform.claude.com/docs/en/build-with-claude/claude-in-microsoft-foundry#hosting-options)**：Claude 在工具搜索路径上启动而不是使用 `WaitForMcpServers`，因为 Claude Code 仅从 API 发现部署的服务器端拒绝。Claude Code 将该部署切换到 [前期加载](#scale-with-mcp-tool-search) 后，来自完成连接的服务器的工具在 Claude 的下一个请求中变为可用。

启用工具搜索后，当服务器在 Claude 工作时完成连接时，Claude Code 在同一轮次的下一个请求中将服务器的工具名称列出给 Claude。Claude 然后可以搜索和调用这些工具，而无需等待您的下一条消息。

恢复会话后，Claude 可以在工具的 MCP 服务器仍在连接时调用已保存对话中的工具。当服务器处于首次连接尝试时，Claude Code 会将该调用保留最多 10 秒，并在工具可用后运行它。如果服务器未能及时连接，或者已在 [失败尝试后重试](#automatic-reconnection)，调用会失败并返回 `No such tool available` [工具错误](https://code.claude.com/docs/zh-CN/errors.md#no-such-tool-available)。

### 禁用服务器而不删除它

在 `/mcp` 面板中切换服务器关闭，以停止 Claude Code 连接到它，而不丢失其配置。Claude Code 仍然在 `/mcp` 中列出服务器，标记为禁用。

当您切换服务器时，Claude Code 在 `~/.claude.json` 中按项目记录您的选择，在两个涵盖不相交服务器集的列表之一中：

* `disabledMcpServers`：用户配置的服务器、插件服务器、您的组织 [通过托管设置提供](https://code.claude.com/docs/zh-CN/managed-mcp.md#provide-servers-through-managed-settings) 的服务器、Claude Code [自己获取](#how-connectors-reach-claude-code) 的 claude.ai 连接器以及默认打开的内置服务器的选择退出列表。Claude Code 不连接您在此处列出的服务器。当您使用 [禁用 claude.ai 连接器](#disable-claude-ai-connectors) 中描述的按项目 `/mcp` 切换禁用 claude.ai 连接器时，Claude Code 在此列表下使用其显示名称（例如 `claude.ai Slack`）写入它。
* `enabledMcpServers`：默认关闭的内置服务器（例如 `computer-use`）的选择加入列表。Claude Code 仅当您在此处列出它时才连接默认关闭的服务器。

Claude Code 为每个服务器查询恰好两个列表之一，因此两个列表都不会覆盖另一个。如果您将常规服务器添加到 `enabledMcpServers`，或将默认关闭的内置服务器添加到 `disabledMcpServers`，Claude Code 会忽略该条目。

`disabledMcpServers` 和 `enabledMcpServers` 与 [`enabledMcpjsonServers`](https://code.claude.com/docs/zh-CN/settings-reference#enabledmcpjsonservers) 和 [`disabledMcpjsonServers`](https://code.claude.com/docs/zh-CN/settings-reference#disabledmcpjsonservers) 无关，后者控制项目的 `.mcp.json` 文件中定义的服务器的批准。

### MCP 客户端运行时

Claude Code 通过两个客户端运行时之一连接到 MCP 服务器。v1 运行时基于 MCP TypeScript SDK 1.x。v2 运行时是 [MCP TypeScript SDK 2.0](https://ts.sdk.modelcontextprotocol.io/v2/) 上的相同代码，它添加了 MCP 协议修订版 2026-07-28。本页的其余部分适用于两个运行时，除非某个部分指明 v2 运行时。

Claude Code 在每次启动时选择一个运行时，并在您退出前保持它。在 [获取功能标志](https://code.claude.com/docs/zh-CN/env-vars.md#features-that-need-feature-flag-fetching) 的会话中，它在 Claude Code v2.1.232 或更高版本上使用 v2 运行时。

在不获取功能标志的会话中，Claude Code 在 Claude Code v2.1.274 或更高版本上默认使用 v2 运行时：

* Amazon Bedrock、Claude Platform on AWS、Google Cloud 的 Agent Platform 或 Microsoft Foundry 上的会话，除非嵌入 Claude Code 的主机平台设置 [`CLAUDE_CODE_PROVIDER_MANAGED_BY_HOST`](https://code.claude.com/docs/zh-CN/env-vars)
* 通过 [Claude 应用网关](https://code.claude.com/docs/zh-CN/claude-apps-gateway) 登录的会话
* 您关闭遥测或功能标志获取的会话，例如使用 `DISABLE_TELEMETRY`

在 v2 上，Claude Code 还会：

* 询问 HTTP、stdio 和 claude.ai 连接器服务器是否支持较新的修订版，并与支持的服务器一起使用它。这包括云端会话和 [Remote Control](https://code.claude.com/docs/zh-CN/remote-control.md) 会话的连接器。Claude Code 与 v1 一样连接到其他所有服务器。
* 通过 [它保持打开的流](#notification-streams-on-the-v2-runtime) 从使用较新修订版的服务器接收 `list_changed` 通知。
* 不注册在较新修订版上连接的 [频道](#push-messages-with-channels) 服务器，因为该修订版无法携带频道消息。
* 当授权响应指明意外的发行者时，使 [MCP OAuth 登录](#authenticate-with-remote-mcp-servers) 失败。
* 仅将 [MCP OAuth](#authenticate-with-remote-mcp-servers) 凭据发送到通过 HTTPS 或在 `localhost`、`127.0.0.1` 或 `::1` 上提供的令牌端点。对于令牌端点在其他任何地方为纯 `http://` 的服务器（例如本地网络上的设备），登录失败。请参阅 [拒绝向非 https 令牌端点发送凭据](https://code.claude.com/docs/zh-CN/errors.md#refusing-to-send-credentials-to-non-https-token-endpoint)。

Anthropic 可以通过 Claude Code 获取的功能标志，将特定服务器保持在较早的协议上，或不使用该流。

要自己选择运行时，请设置 [`MCP_SDK_GENERATION`](https://code.claude.com/docs/zh-CN/env-vars) 为 `v1` 或 `v2`。要决定 Claude Code 是否询问，请设置 [`MCP_PROTOCOL_NEGOTIATION`](https://code.claude.com/docs/zh-CN/env-vars) 为 `auto` 或 `legacy`。

### 动态工具更新

MCP 服务器可以在连接期间更改其提供的工具、提示词或资源，并发送 `list_changed` 通知。收到通知时：

* **在交互式终端会话中**，Claude Code 从该服务器获取更新后的列表，因此您无需重新连接它。
* **在使用 `-p` 标志的 [非交互模式](https://code.claude.com/docs/zh-CN/headless.md) 和 [Agent SDK](https://code.claude.com/docs/zh-CN/agent-sdk/overview.md) 中**，Claude Code 在收到这些通知时仅刷新工具列表。

如果刷新请求失败，Claude Code 保留服务器之前发现的工具、提示词和资源，直到稍后的刷新成功。

#### v2 运行时上的通知流

在 [v2 运行时](#mcp-client-runtimes) 上，Claude Code 通过它保持打开的流从使用较新协议修订版的服务器接收 `list_changed` 通知。当流关闭时，Claude Code 重新打开它，有两个限制：

* **流在 10 秒内再次关闭**：Claude Code 最多重新打开三次，然后停止该连接。
* **流保持打开超过 10 秒，然后关闭**，连接到无服务器主机的流通常如此：在一小时内五次重新打开后，Claude Code 在下一次之前等待约六小时。

在流重新打开之前，您保留服务器最后获取的工具、提示词和资源。要更快地获取其更改，请从 `/mcp` 重新连接服务器。

### 自动重新连接

Claude Code 重新连接在会话中途断开的远程服务器，并在瞬时错误后重试 HTTP 或 SSE 服务器的首次连接。对于 stdio 服务器，请参阅 [退出或无法启动的 stdio 服务器](#stdio-servers-that-exit-or-fail-to-start)。

#### 远程服务器的会话中途断开

Claude Code 首先使用指数退避尝试重新连接断开的远程服务器，最多五次，从一秒延迟开始，每次加倍：

* **在交互式会话中**：`/mcp` 在 Claude Code 重新连接时显示服务器为待处理。在五次失败尝试后，Claude Code 将服务器标记为失败，或在服务器需要再次授权时标记为需要身份验证。当它将服务器标记为失败时，您会看到 `MCP server "<name>" disconnected · open /mcp to reconnect` 通知。您可以从 `/mcp` 手动重试。
* **在 [`claude -p`](https://code.claude.com/docs/zh-CN/headless) 运行和 [Agent SDK](https://code.claude.com/docs/zh-CN/agent-sdk/overview.md) 会话中**：Claude Code 进行相同的五次尝试，没有 `/mcp` 面板显示这些尝试。如果五次尝试全部失败，运行会在没有该服务器工具的情况下继续：
  * **在瞬时错误（例如 5xx 响应或超时）之后**：Claude Code 在后台继续重试 HTTP 或 SSE 服务器。重试最多持续约半小时，当某次重试连接成功时，服务器的工具会再次可用。在此之前，Agent SDK 会话报告 [状态](https://code.claude.com/docs/zh-CN/agent-sdk/mcp.md#error-handling) `failed`，您的代码可以 [重新连接服务器](https://code.claude.com/docs/zh-CN/agent-sdk/mcp.md#error-handling)。要在 `claude -p` 中重新连接它，请重新开始运行。
  * **当服务器 [需要您重新登录](#authenticate-with-remote-mcp-servers) 时**：Claude Code 停止重试，Agent SDK 会话将服务器的状态报告为 `needs-auth`。在您的 shell 中使用 [`claude mcp login <name>`](#authenticate-from-the-command-line) 登录。然后重新开始运行，或者在 Agent SDK 会话中改为 [重新连接服务器](https://code.claude.com/docs/zh-CN/agent-sdk/mcp.md#error-handling)。

#### 失败的首次连接

当 HTTP 或 SSE 服务器的首次连接因瞬时错误（例如 5xx 响应、连接被拒绝或超时）失败时，Claude Code 最多重试三次。如果连接仍然失败，Claude Code 将服务器标记为失败。

这些重试在以下情况下不适用：

* stdio 服务器的首次连接。要再次尝试，请参阅 [退出或无法启动的 stdio 服务器](#stdio-servers-that-exit-or-fail-to-start)
* WebSocket 服务器的首次连接。要在交互式终端会话中再次尝试，请从 `/mcp` 中该服务器的菜单选择 **Reconnect**
* 身份验证或未找到错误，因为它需要配置更改来解决。当 [`headersHelper`](#use-dynamic-headers-for-custom-authentication) 是服务器唯一的 `Authorization` 标头来源时，Claude Code 仍然重试身份验证错误，因为它在每次尝试时重新运行助手并可以获取新凭据

#### 退出或无法启动的 stdio 服务器

要重新启动 stdio 服务器，请阅读与其所发生情况相符的列。根据您运行 Claude Code 的方式找到对应的行：交互式会话、`claude -p` 运行或 Agent SDK 会话。

| 您运行 Claude Code 的方式 | 服务器进程在会话中途退出 | 服务器的首次连接失败 |
| - | - | - |
| 交互式会话 | Claude Code 将服务器标记为失败。要重新启动它，请从 `/mcp` 中该服务器的菜单选择 **Reconnect** | Claude Code 将服务器标记为失败。要重新启动它，请从 `/mcp` 中该服务器的菜单选择 **Reconnect** |
| `claude -p` 运行 | Claude Code 在 Claude 下次调用该服务器的某个工具时重新启动服务器，调用会在服务器连接后运行 | 运行会在没有该服务器工具的情况下继续。要让 Claude Code 再次尝试连接，请重新开始运行。当插件提供该服务器时，新的运行会在失败后 15 分钟内跳过连接，因此请先等待这段时间，或 [更早重试](https://code.claude.com/docs/zh-CN/errors.md#skipping-connection-recent-failure-cached) |
| Agent SDK 会话 | Claude Code 在 Claude 下次调用该服务器的某个工具时重新启动服务器，调用会在服务器连接后运行 | 会话会在没有该服务器工具的情况下继续。您的代码可以 [重新连接服务器](https://code.claude.com/docs/zh-CN/agent-sdk/mcp.md#error-handling) |

#### 失败的发现请求

服务器连接后，Claude Code 向其发送能力发现请求，例如 `tools/list`、`prompts/list` 和 `resources/list`。Claude Code 在瞬时网络或服务器错误后最多重试这些请求三次，短退避。它不重试身份验证错误、4xx 响应或请求超时。

#### 自行重试失败的服务器

要重试每个失败或需要身份验证的服务器，请在交互式终端会话中的 Claude Code 输入框运行 `/mcp reconnect all`。这需要 Claude Code v2.1.284 或更高版本。较早的版本会打印 `MCP server "all" not found`。

#### Claude 如何了解服务器失败

Claude Code 是否告诉 Claude 配置的服务器未能连接取决于 [工具搜索](#scale-with-mcp-tool-search)，默认打开：

* 使用工具搜索，Claude Code 告诉 Claude 哪个服务器失败及其连接错误，因此 Claude 在其回复中报告连接失败。Claude Code 在找不到匹配工具的 `ToolSearch` 结果中包含相同的信息。
* 在任何 [不使用工具搜索的配置](#configure-tool-search) 中，Claude Code 不向 Claude 报告失败的服务器连接。

### 使用频道推送消息

MCP 服务器也可以直接将消息推送到您的会话中，以便 Claude 可以对外部事件（如 CI 结果、监控警报或聊天消息）做出反应。要启用此功能，您的服务器声明 `claude/channel` 能力，您在启动时使用 `--channels` 标志选择加入。请参阅 [频道](https://code.claude.com/docs/zh-CN/channels.md) 以使用官方支持的频道，或 [频道参考](https://code.claude.com/docs/zh-CN/channels-reference.md) 以构建您自己的。

在 [v2 运行时](#mcp-client-runtimes) 上，协商 MCP 协议修订版 2026-07-28 的频道服务器无法传递频道消息，因此 Claude Code 不将其注册为频道。不支持该修订版的频道服务器会在较早的握手上连接，并像以前一样注册。

Claude Code 默认会向 stdio 服务器询问该修订版。要将 stdio 频道服务器保持在较早的握手上，请将 [`MCP_PROTOCOL_NEGOTIATION`](https://code.claude.com/docs/zh-CN/env-vars) 设置为 `legacy`，这会将所有服务器都保持在较早的握手上。

> [!TIP]
>
> 提示：
>
> * 使用 `-s` 或 `--scope` 标志指定配置的存储位置：
>   * `local`（默认）：仅在当前项目中对您可用
>   * `project`：通过 `.mcp.json` 文件与项目中的每个人共享
>   * `user`：在所有项目中对您可用
> * 使用 `-e` 或 `--env` 标志设置环境变量（例如 `-e KEY=value`）
> * `--transport` 和 `--header` 标志也接受 `-t` 和 `-H` 短形式
> * 使用 `MCP_TIMEOUT` 环境变量配置 MCP 服务器启动超时（例如 `MCP_TIMEOUT=10000 claude` 设置 10 秒超时）
> * 通过在该服务器的 `.mcp.json` 条目中添加 `timeout` 字段（以毫秒为单位）来设置按服务器工具执行超时，例如 `"timeout": 600000` 表示十分钟。这仅对该服务器覆盖 `MCP_TOOL_TIMEOUT` 环境变量
> * 当 MCP 工具输出超过 10,000 个 token 时，Claude Code 显示警告，默认限制输出为 25,000 个 token。要更改 token 限制，请设置 `MAX_MCP_OUTPUT_TOKENS` 环境变量，例如 `MAX_MCP_OUTPUT_TOKENS=50000`。警告阈值是固定的。除非服务器提高了某个工具自身的限制，否则超过 50,000 个字符的成功文本结果会被保存到文件中，不受此变量影响。请参阅 [MCP 输出限制和警告](#mcp-output-limits-and-warnings)
> * 使用 `/mcp` 与需要 OAuth 2.0 身份验证的远程服务器进行身份验证

### MCP 工具调用的时间限制

要限制 MCP 工具调用可以运行的时长：

* **按服务器**：在该服务器的配置条目中添加以毫秒为单位的 `timeout` 字段
* **跨服务器**：设置 [`MCP_TOOL_TIMEOUT`](https://code.claude.com/docs/zh-CN/env-vars)

按服务器的 `timeout` 是每个工具调用的硬性墙钟时间限制，来自服务器的进度通知不会延长它。低于 1000 的值被忽略并回退到 `MCP_TOOL_TIMEOUT`，或在该变量未设置时回退到其约 28 小时的默认值。对于 HTTP、SSE 或 [claude.ai 连接器](https://code.claude.com/docs/zh-CN/mcp.md#use-mcp-servers-from-claude-ai) 服务器，还有第二个按请求计时器，覆盖每个请求直到服务器的第一个响应字节。Claude Code 将该计时器设置为三个值中最大的：60 秒、适用于服务器的工具超时和 `MCP_TIMEOUT`。未设置的 `MCP_TOOL_TIMEOUT` 的 28 小时默认值不参与该比较，低于 60 秒的值不会缩短计时器。Stdio 和 WebSocket 服务器没有按请求计时器。

至少 1000 的按服务器 `timeout` 也充当下面描述的空闲超时的下限：Claude Code 永远不会早于按服务器 `timeout` 因空闲而中止该服务器的工具调用。

对 MCP 服务器的工具调用，如果在空闲窗口内既没有响应也没有进度通知，会以错误中止，而不是等待墙钟时间限制。空闲超时适用于除 IDE 服务器和 SDK 进程内服务器外的每种服务器类型。空闲窗口对 HTTP、SSE、WebSocket 和 [claude.ai 连接器](#use-mcp-servers-from-claude-ai) 服务器默认为五分钟，对 stdio 服务器默认为 30 分钟。

设置 [`CLAUDE_CODE_MCP_TOOL_IDLE_TIMEOUT`](https://code.claude.com/docs/zh-CN/env-vars) 环境变量（以毫秒为单位）来更改空闲窗口，或将其设置为 `0` 以禁用检查。

这些超时限制调用可以运行多长时间，但不一定是它阻塞会话多长时间：运行超过两分钟的主对话调用会先移到后台任务。请参阅 [长工具调用的自动后台处理](#automatic-backgrounding-of-long-tool-calls)。

### 长工具调用的自动后台处理

如果某个后台调用在 [`/tasks`](https://code.claude.com/docs/zh-CN/commands#all-commands) 中的详情视图显示 `runs on the server; survives exiting this session`，请参阅[在服务器上运行的 MCP 任务](#mcp-tasks-that-run-on-the-server)。

主对话中的 MCP 工具调用在两分钟后仍在运行时会移到后台任务，而不是阻塞会话。Claude 立即收到任务 ID 并继续工作，结果在调用完成时作为任务通知到达。

任务出现在 [`/tasks`](https://code.claude.com/docs/zh-CN/commands#all-commands) 中，您也可以在其中停止它，退出会话后它不会保留。任务的条目显示服务器报告的最新进度。如果您停止任务，Claude Code 会取消调用并告知 Claude 不会有结果返回。停止并不保证服务器停止：服务器可能已经完成了部分工作，或者仍可能完成它。

调用在后台运行时，每次调用的限制仍然适用：由按服务器 `timeout` 或 [`MCP_TOOL_TIMEOUT`](https://code.claude.com/docs/zh-CN/env-vars) 设置的墙钟时间限制，以及由 [`CLAUDE_CODE_MCP_TOOL_IDLE_TIMEOUT`](https://code.claude.com/docs/zh-CN/env-vars) 设置的空闲超时。

要更改或关闭自动后台化：

* 设置以毫秒为单位的 [`CLAUDE_CODE_MCP_AUTO_BACKGROUND_MS`](https://code.claude.com/docs/zh-CN/env-vars) 环境变量以更改阈值，或设置为 `0` 以关闭自动后台化。设置为 `0` 时，Claude 在交互式会话中也会[等待 MCP 任务](#make-claude-wait-for-the-result-of-an-mcp-task)
* 将 [`CLAUDE_CODE_DISABLE_BACKGROUND_TASKS`](https://code.claude.com/docs/zh-CN/env-vars) 设置为 `1`，以关闭自动后台化和[后台命令](https://code.claude.com/docs/zh-CN/tools-reference.md#background-commands)

某些调用永远不会移到后台：

* 来自 [子代理](https://code.claude.com/docs/zh-CN/sub-agents.md) 的调用；Claude Code 仅后台处理主对话调用
* 对 IDE 服务器的调用
* [非交互模式](https://code.claude.com/docs/zh-CN/headless.md) 中的调用，除非 `CLAUDE_AUTO_BACKGROUND_TASKS` 设置为 `1`，因为一次性运行可能在结果到达前结束

等待已打开的 [引出对话框](#respond-to-mcp-elicitation-requests) 的调用在对话框打开期间不会后台处理；服务器是在等待您的输入，而不是运行缓慢，因此 Claude Code 将移动推迟到对话框关闭。

### 在服务器上运行的 MCP 任务

MCP 任务是指 MCP 服务器使用 [MCP tasks 扩展](https://modelcontextprotocol.io/extensions/tasks/overview)自行运行的长时间作业，例如报告导出或构建。服务器接受该作业后，Claude 的工具调用即返回，因此 Claude 会继续工作，除非您[让 Claude 等待](#make-claude-wait-for-the-result-of-an-mcp-task)。作业结束时，会话记录中会注明这一点，Claude 会获得结果。您可以向 Claude 询问结果。

在此期间，页脚会显示诸如 `1 MCP task` 的计数，Claude Code 会通过向服务器检查任务状态来跟踪该任务。远程服务器上的任务在您退出后会继续运行。要再次跟踪它，请[恢复同一会话](#exit-clear-or-resume-while-an-mcp-task-runs)。

> [!NOTE]
>
> MCP 任务在 Claude Code CLI 中处于公开测试版阶段，需要 Claude Code v2.1.304 或更高版本。[调用成为 MCP 任务的条件](#requirements-for-mcp-tasks)列出了可以使用该功能的会话和服务器。

#### 停止 MCP 任务或获取其结果

要停止正在运行的任务，请在 [`/tasks`](https://code.claude.com/docs/zh-CN/commands#all-commands) 中选择它并按 `x`，或者让 Claude 停止它。Claude Code 会停止跟踪该任务并请求服务器取消它，但服务器仍可能完成它。请先从 `/mcp` 重新连接已断开的服务器，否则不会发出取消请求，Claude Code 会忘记该任务且无法再次跟踪它。按 `Esc` 不会停止后台任务。

任务结束时，会话记录中会出现类似这样的一行：

```text theme={null}
MCP task <server task id> (<server>/<tool>) completed.
```

结果会发送给 Claude，而不是写入会话记录，因此请向 Claude 询问结果。如果结果到达时 Claude 处于空闲状态，Claude 会主动开始处理它。

#### 回答来自 MCP 任务的问题

正在运行的任务可以打开 [elicitation 对话框](#respond-to-mcp-elicitation-requests)向您请求输入。在 `/tasks` 打开期间，该对话框会保持隐藏，因此请关闭 `/tasks` 以作答。拒绝或按 `Esc` 也是对服务器的一种回答，服务器随后会决定任务是否继续。

在[非交互](https://code.claude.com/docs/zh-CN/headless.md)会话中，对于请求输入的任务，Claude Code 会请求服务器取消它，而不会运行您的 [`Elicitation` hook](https://code.claude.com/docs/zh-CN/hooks#elicitation)，也不会调用 Agent SDK 的 `onElicitation` 回调。

#### 在 MCP 任务运行时退出、清除或恢复

远程服务器上的任务在您退出后会继续运行。stdio 服务器的进程会在 Claude Code 退出时停止，尽管其任务在 `/tasks` 中显示 `runs on the server; survives exiting this session`。除非该服务器的文档说明工作会在进程结束后继续，否则请先让这些任务完成，或在 `/tasks` 中停止它们。

| 情况 | 任务会怎样 | 您可以做什么 |
| - | - | - |
| 您运行 `/clear` 或 `/branch`，或 `/resume` 另一个会话 | 在 Claude Code 保持打开期间，结果会送达您当前的对话 | 退出后，使用 `claude --resume` 恢复任务启动时所在的会话 |
| 您退出 Claude Code | 任何退出选项都不会取消远程服务器的任务，即使退出对话框将其列在 `The following will stop when you exit:` 下 | 先在 `/tasks` 中停止它，或稍后恢复同一会话 |
| 您以交互方式返回任务所在的会话：`claude --resume`、`claude --continue` 或 `/resume` | 服务器连接后，Claude Code 会再次跟踪该任务；如果任务在此期间已完成且服务器仍知道它，Claude 会获得结果 | 保持服务器的名称和连接设置不变 |
| 您在一次性 `claude -p "<prompt>"` 运行中使用 `--resume` 或 `--continue` 恢复任务所在的会话 | 该运行不会跟踪该任务 | 以交互方式恢复，或使用 `--input-format stream-json` 恢复 |
| 您启动新会话或使用 `--fork-session` | 新会话不会跟踪该任务 | 恢复启动该任务的会话 |

#### 调用成为 MCP 任务的条件

只有当以下所有条件都满足时，工具调用才会成为 MCP 任务：

* 服务器支持任务
* 服务器是以[协议修订版 2026-07-28](#mcp-client-runtimes) 连接的 HTTP、stdio 或 claude.ai 连接器服务器。设置 [`MCP_SDK_GENERATION=v1` 或 `MCP_PROTOCOL_NEGOTIATION=legacy`](https://code.claude.com/docs/zh-CN/env-vars) 会排除这一情况
* 会话会[获取功能标志](https://code.claude.com/docs/zh-CN/env-vars.md#features-that-need-feature-flag-fetching)，因此不能在 Amazon Bedrock 等第三方提供商上，也不能关闭遥测或标志获取
* Anthropic 已为您的会话类型启用了 MCP 任务。在 v2.1.304 或更高版本上，这包括终端 CLI 会话、[Agent SDK](https://code.claude.com/docs/zh-CN/agent-sdk/overview.md) 会话和 [GitHub Actions](https://code.claude.com/docs/zh-CN/github-actions.md) 运行。其他使用入口（例如 [VS Code 扩展](https://code.claude.com/docs/zh-CN/vs-code.md)和[桌面应用](https://code.claude.com/docs/zh-CN/desktop.md)）没有 MCP 任务
* 调用来自主对话，而不是来自[子代理](https://code.claude.com/docs/zh-CN/sub-agents.md)
* 会话是交互式的，或者 [Agent SDK](https://code.claude.com/docs/zh-CN/agent-sdk/overview.md) 会话或读取 `--input-format stream-json` 的 `claude -p` 运行满足：
  * [`CLAUDE_AUTO_BACKGROUND_TASKS`](https://code.claude.com/docs/zh-CN/env-vars) 为 `1`
  * `CLAUDE_CODE_DISABLE_MCP_TASK_BACKGROUND` 不为 `1`
  * `CLAUDE_CODE_MCP_AUTO_BACKGROUND_MS` 不为 `0`

其他任何调用都是普通工具调用，[长时间工具调用的自动后台化](#automatic-backgrounding-of-long-tool-calls)说明了其中哪些会转入后台。

请保持 Agent SDK 或 stream-json 会话的输入处于打开状态，因为输入关闭后，运行[不会等待 MCP 任务](https://code.claude.com/docs/zh-CN/headless.md#mcp-work-when-a-non-interactive-run-ends)。一次性 `claude -p "<prompt>"` 运行不会获得 MCP 任务。

#### MCP 任务的状态消息

这些消息出现在任务的 `/tasks` 条目或其结束时的会话记录行中，表示 Claude Code 无法连接到服务器或任务。当 `/tasks` 中显示其中一条消息时，作业可能仍在运行：Claude Code 不会干预它并会继续检查。`300s` 消息和配置相关消息即使在恢复联系后也会保留在条目上。

| 消息 | 发生了什么 | 您可以做什么 |
| - | - | - |
| `reconnecting…` | 自恢复以来服务器尚未应答，或状态检查没有得到应答 | 如果一直显示，请从 `/mcp` 重新连接 |
| `needs a sign-in to <server>` | 服务器拒绝了状态检查 | 从 `/mcp` 登录 |
| `status requests come back with errors; retrying…` | 服务器或其前面的代理以错误应答状态检查 | 等待 Claude Code 重试 |
| `server '<name>' did not reconnect within 300s` | 五分钟内没有得到应答的检查，例如连接中断或服务器被禁用 | 从 `/mcp` 重新连接 |
| `server '<name>' asked for a sign-in, and none was accepted within 300s` | 五分钟后服务器仍要求登录 | 从 `/mcp` 登录 |
| `status requests to server '<name>' came back with errors, and none succeeded in 300s` | 五分钟内都是错误应答 | 检查服务器自身的健康状况或日志 |
| `server '<name>' is not configured in this session; the task may still be running on it` | 会话中没有该名称的服务器，例如它已被移除、重命名或未包含在 `--mcp-config` 中，或者某个 claude.ai 连接器仍在加载 | 以相同名称恢复该服务器，然后再次恢复会话。对于正在加载的连接器，无需操作：连接器出现后 Claude Code 会跟踪该任务 |
| 以 `server '<name>' was reconfigured to a different transport since this task was created` 开头的消息 | 存在该名称的服务器，但其连接设置自任务开始后已更改，例如 URL、command、args、env、headers 或 timeout | 恢复之前的设置，然后再次恢复会话 |
| `the server rejected this task's id; asking again…` | 服务器表示不认识该任务 ID。Claude Code 会继续询问最多五分钟 | 等待：如果服务器再次识别该 ID，消息会消失，否则会出现 `outcome unknown.` 行 |
| 以 `outcome unknown.` 结尾的会话记录行，其中一种形式为 `expired on the server: outcome unknown.` | 服务器拒绝了该 ID 或以其他永久性错误拒绝，Claude Code 已停止跟踪该任务 | 第二次调用可能会重复执行该工作。再次调用该工具之前，请从服务器了解已完成的内容，例如让 Claude 使用该服务器的其他工具 |

#### MCP 任务的时间限制

默认情况下，后台任务没有 Claude Code 时间限制。[工具调用时间限制](#time-limits-on-mcp-tool-calls)（包括挂钟时间限制和空闲超时）在服务器以任务应答该调用之前适用。如果您[让 Claude 等待](#make-claude-wait-for-the-result-of-an-mcp-task)，这些限制会在整个任务期间适用。得到应答的状态检查会计为空闲超时中的活动。

#### 让 Claude 等待 MCP 任务的结果

要在交互式会话中让 Claude 等待任务的结果，请设置以下两个环境变量之一，例如在 [`settings.json`](https://code.claude.com/docs/zh-CN/settings) 的 `env` 块中：

* **[`CLAUDE_CODE_MCP_AUTO_BACKGROUND_MS`](https://code.claude.com/docs/zh-CN/env-vars) 设置为 `0`**：Claude 会等待每个 MCP 工具调用直到调用结束
* **[`CLAUDE_CODE_DISABLE_MCP_TASK_BACKGROUND`](https://code.claude.com/docs/zh-CN/env-vars) 设置为 `1`**：Claude 会等待任务的结果。当[自动后台化](#automatic-backgrounding-of-long-tool-calls)开启时，它默认会在两分钟后转移该调用，因此短任务会就地完成，长任务不会阻塞您。被转移的调用是普通后台调用，而不是 MCP 任务。它不会在退出会话后保留

Claude 等待期间，按 `Esc` 或达到[挂钟时间限制或空闲超时](#time-limits-on-mcp-tasks)会结束该调用，Claude Code 会请求服务器取消该任务。

如果您将 `CLAUDE_CODE_DISABLE_BACKGROUND_TASKS` 设置为 `1` 或以 [bare 模式](https://code.claude.com/docs/zh-CN/headless.md#start-faster-with-bare-mode)启动，仅凭这一点并不会让 Claude 等待 MCP 任务。以下示例将 `CLAUDE_CODE_MCP_AUTO_BACKGROUND_MS` 设置为 `0`，因此 Claude 会等待每个 MCP 工具调用：

```json settings.json theme={null}
{
  "env": {
    "CLAUDE_CODE_MCP_AUTO_BACKGROUND_MS": "0"
  }
}
```

### 面向服务器开发者的 MCP 任务

如果您构建 MCP 服务器，可以用任务来应答长时间运行的 `tools/call`。[MCP tasks 扩展](https://modelcontextprotocol.io/extensions/tasks/overview)定义了您要发送的消息。在[协议修订版 2026-07-28](#mcp-client-runtimes) 连接上，每个请求都会在 `params._meta["io.modelcontextprotocol/clientCapabilities"]` 中携带 Claude Code 的客户端能力，其中 `extensions` 下包含 `"io.modelcontextprotocol/tasks": {}`。[无法成为任务](#requirements-for-mcp-tasks)的 `tools/call` 请求会省略该扩展。

#### 以任务应答调用

只有当请求携带该扩展时才以任务应答，否则 Claude 会收到工具错误 `Unsupported result type 'task' for tools/call`。当缺少 `taskId`、`status`、`createdAt`、`lastUpdatedAt` 或 `ttlMs` 时，Claude 也会收到相同的错误。任务对象是您的 `tools/call` 响应的 JSON-RPC `result`，不需要 `content` 数组，此处还包含可选的 `statusMessage` 和 `pollIntervalMs`：

```json theme={null}
{
  "resultType": "task",
  "taskId": "build-7f3a9c2e",
  "status": "working",
  "statusMessage": "Compiling, 40 of 212 files done",
  "createdAt": "2026-10-22T14:03:07Z",
  "lastUpdatedAt": "2026-10-22T14:03:07Z",
  "ttlMs": null,
  "pollIntervalMs": 5000
}
```

| 字段 | Claude Code 如何处理 |
| - | - |
| `resultType` | 设置为 `"task"` 以将该应答标记为任务。如果没有它，Claude Code 会将该应答视为普通结果 |
| `taskId` | 必需。最多 128 个不含空格的可打印 ASCII 字符，否则 Claude 会收到以 `MCP server "<name>" returned a malformed task handle for tool "<tool>"` 开头的工具错误。请确保该 ID 在不同连接间都能被应答，因为恢复后 Claude Code 会在新连接上询问它 |
| `status` | 必需。取值为 `working`、`input_required`、`completed`、`failed` 或 `cancelled` 之一。其他值会产生 `Unsupported result type` 错误 |
| `createdAt`、`lastUpdatedAt`、`ttlMs` | 必需，`ttlMs` 可以为 `null`。Claude Code 不会因为任务的 `ttlMs` 已过而丢弃该任务 |
| `pollIntervalMs` | 可选，但不能为 `null`。两次 `tasks/get` 请求之间的毫秒数，限制在 250 到 30000 之间。字段缺失时 Claude Code 使用 2000，并会从每个 `tasks/get` 响应中重新读取该值 |
| `statusMessage` | 可选。`/tasks` 显示的是您最新的 `tasks/get` 响应中的值，而不是这第一个应答中的值，省略该字段的响应会清除它。[来自 Claude Code 的状态消息](#status-messages-on-an-mcp-task)可能会取代它 |

在 HTTP 或 claude.ai 连接器服务器上，任务应答必须在[按请求的计时器](#time-limits-on-mcp-tool-calls)内开始，因此请先接受作业并应答，然后再执行工作。

#### 报告状态、结果和问题

在任务应答之后，Claude Code 会立即发送带有 `{ "taskId": ... }` 的 `tasks/get`，然后每个轮询间隔重复一次。tasks 扩展要求您的响应携带 `"resultType": "complete"`，以及 `taskId`、`status`、`createdAt`、`lastUpdatedAt` 和 `ttlMs`。`ttlMs` 可以为 `null`，`statusMessage` 和 `pollIntervalMs` 是可选的。以下示例报告构建已完成：

```json theme={null}
{
  "resultType": "complete",
  "taskId": "build-7f3a9c2e",
  "status": "completed",
  "createdAt": "2026-10-22T14:03:07Z",
  "lastUpdatedAt": "2026-10-22T14:09:41Z",
  "ttlMs": null,
  "result": {
    "content": [{ "type": "text", "text": "Build succeeded: 212 files, 0 warnings" }]
  }
}
```

| 状态、方法或响应 | Claude Code 如何处理 |
| - | - |
| `working` | 继续轮询 |
| `completed` | 将顶层 `result` 对象中的普通 `tools/call` 结果交给 Claude。其中的 `isError: true` 会被报告为失败 |
| `failed` | 将顶层 `error` 对象 `{ code, message, data }` 的 `message` 告知 Claude，tasks 扩展要求失败的任务包含该对象。如果没有该 `message`，Claude 会改为收到 `statusMessage`，否则收到 `Task failed` |
| `cancelled` | 将 `statusMessage` 告知 Claude，否则告知 `Task was cancelled by the server` |
| `input_required` | 读取 `inputRequests`，这是一个将您选择的键映射到 `{ method, params }` 的对象。Claude Code 会通过带有 `{ taskId, inputResponses }` 的 `tasks/update` 对每个键应答一次，其中 `inputResponses` 将相同的键映射到该请求的结果。每个任务超过 16 个的键会被忽略 |
| `inputRequests` 中的方法 | `elicitation/create` 仅在交互式终端会话中处理，`roots/list` 在所有地方都会处理。当 Claude Code 无法处理某个请求时，它会请求服务器取消该任务 |
| 其他 `status` 值 | Claude Code 会请求服务器取消该任务，并将其作为失败报告给 Claude |
| JSON-RPC 错误 `-32602` | 视为未知的任务 ID。对于后台任务，Claude Code 会继续询问最多五分钟，然后将结果作为未知报告给 Claude |
| `tasks/cancel` | 例如，在服务器已连接时 Claude 或用户停止该任务时发送。退出时不发送 |
| `tasks/result`、`tasks/list`、`notifications/tasks` | 不使用，因此请将结果放在您的 `tasks/get` 响应中 |

### 插件提供的 MCP 服务器

[插件](https://code.claude.com/docs/zh-CN/plugins/overview.md) 可以捆绑 MCP 服务器，在您启用插件时提供工具和集成。

**插件 MCP 服务器如何工作**：

* 插件在插件根目录的 `.mcp.json` 中或内联在 `plugin.json` 中定义 MCP 服务器
* 当您启用插件时，Claude Code 自动启动其 MCP 服务器
* Claude Code 将插件 MCP 工具与手动配置的 MCP 工具一起提供
* 您通过安装或卸载插件来添加和删除插件服务器，而不是使用 `/mcp` 命令。您仍然可以在 `/mcp` 中 [切换已安装的插件服务器关闭](#disable-a-server-without-removing-it)，这会停止 Claude Code 连接到它而不删除插件

**示例插件 MCP 配置**：

在插件根目录的 `.mcp.json` 中：

```json theme={null}
{
  "mcpServers": {
    "database-tools": {
      "command": "${CLAUDE_PLUGIN_ROOT}/servers/db-server",
      "args": ["--config", "${CLAUDE_PLUGIN_ROOT}/config.json"],
      "env": {
        "DB_URL": "${DB_URL}"
      }
    }
  }
}
```

或内联在 `plugin.json` 中：

```json theme={null}
{
  "name": "my-plugin",
  "mcpServers": {
    "plugin-api": {
      "command": "${CLAUDE_PLUGIN_ROOT}/servers/api-server",
      "args": ["--port", "8080"]
    }
  }
}
```

**插件 MCP 功能**：

* **自动生命周期**：服务器在这些时间点连接和断开：
  * 在会话启动时，Claude Code 自动连接已启用插件的服务器。在 `/mcp` 中，您之前使用过的远程（HTTP 或 SSE）插件服务器可能改为显示 [`cached` 状态](#server-status-detail)；Claude Code 在 Claude 首次调用其工具之一时连接它。当 stdio 插件服务器的连接在不到 15 分钟前失败时，Claude Code 会 [跳过该连接](https://code.claude.com/docs/zh-CN/errors.md#skipping-connection-recent-failure-cached)
  * 如果您在会话期间启用或禁用插件，Claude Code 在更改生效时连接或断开其 MCP 服务器。[在不重新启动的情况下应用插件更改](https://code.claude.com/docs/zh-CN/plugins/cli-reference.md#reload-plugins) 说明了生效时间。在没有交互式终端的会话中，`/reload-plugins` 不连接或断开插件 MCP 服务器；这些更改在您的下一个会话中生效
  * 当您重新加载时，Claude Code 保留配置未更改的插件服务器的实时连接，当您从 Agent SDK [替换会话的 MCP 服务器列表](https://code.claude.com/docs/zh-CN/agent-sdk/typescript.md#mcpsetserversresult) 而未列出它们时也是如此
  * 当您在 v2.1.246 或更高版本上 [使用 `/cd` 移动会话](https://code.claude.com/docs/zh-CN/permissions#move-the-session-to-another-directory) 时，Claude Code 连接新目录的设置所启用插件的服务器，并断开不再启用的插件的服务器，因此您无需在移动后运行 `/reload-plugins`
  * 在 [云端会话](https://code.claude.com/docs/zh-CN/claude-code-on-the-web.md) 中，对尚未连接的插件服务器的 MCP 调用（例如空闲会话唤醒后）会按需启动服务器并等待它连接
* **路径占位符**：`${CLAUDE_PLUGIN_ROOT}` 解析为插件的安装目录，`${CLAUDE_PLUGIN_DATA}` 解析为其 [持久状态](https://code.claude.com/docs/zh-CN/plugins/components.md#path-variables-and-persistent-data) 目录，`${CLAUDE_PROJECT_DIR}` 解析为稳定的项目根目录。替换适用于：
  * `stdio` 服务器：`command`、`args`、`env`
  * `http`、`sse` 和 `ws` 服务器：`url`、`headers` 和 `headersHelper`
* **用户环境访问**：访问与手动配置的服务器相同的环境变量
* **多种传输类型**：支持 stdio、SSE、HTTP 和 WebSocket 传输，尽管传输支持可能因服务器而异

插件服务器在 `/mcp` 中出现，并带有表明它们来自插件的指示器。

`claude mcp get` 会隐藏为插件的 stdio 或 WebSocket 服务器配置的值，因为这些值可能携带凭据：

* **stdio 服务器**：它打印 `Command: stdio`、空的 `Args:` 行，并将每个环境变量显示为 `NAME=[REDACTED]`
* **WebSocket 服务器**：它打印 `URL: ws`，并将每个标头显示为 `<name>: [REDACTED]`，`claude mcp list` 则显示 `ws` 来代替 URL

要查看插件配置了什么，请 [在磁盘上找到插件](https://code.claude.com/docs/zh-CN/plugins/loading.md#find-plugins-on-disk) 并阅读其 `.mcp.json` 或 `plugin.json`。

**插件 MCP 工具名称**：

来自插件捆绑的 MCP 服务器的工具在其可调用名称中包含插件名称和服务器键。完整形式是 `mcp__plugin_<plugin-name>_<server-name>__<tool-name>`，其中 `A-Z`、`a-z`、`0-9`、`_` 和 `-` 之外的任何字符都被替换为 `_`。对于名为 `my-plugin` 的插件中捆绑的 `database-tools` 服务器，`query` 工具的可调用名称为：

```
mcp__plugin_my-plugin_database-tools__query
```

在 [权限规则](https://code.claude.com/docs/zh-CN/permissions.md)、skill 的 `allowed-tools` 列表、[子代理的 `tools` 字段](https://code.claude.com/docs/zh-CN/sub-agents#available-tools) 或 [hook 匹配器](https://code.claude.com/docs/zh-CN/hooks.md#match-mcp-tools) 中引用工具时使用此完整名称。针对裸服务器键编写的 hook 匹配器（例如 `mcp__database-tools__.*`）永远不会为插件捆绑的服务器触发。

服务器本身以限定名称 `plugin:<plugin-name>:<server-name>` 注册，例如 `plugin:my-plugin:database-tools`。在需要配置的服务器名称的地方使用该名称，例如 [`mcp_tool` hook 的 `server` 字段](https://code.claude.com/docs/zh-CN/hooks#mcp-tool-hook-fields)。

有关使用插件捆绑 MCP 服务器的详细信息，请参阅 [插件组件参考](https://code.claude.com/docs/zh-CN/plugins/components.md#mcp-servers)。

## MCP 安装作用域

MCP 服务器可以在三个作用域中进行配置。您选择的作用域决定了服务器在哪些项目中加载，以及配置是否与团队共享。管理员还可以通过[托管配置](#managed-mcp-configuration)为所有用户部署或提供服务器。

| 作用域 | 加载范围 | 与团队共享 | 存储位置 |
| - | - | - | - |
| [Local](#local-scope) | 仅当前项目 | 否 | `~/.claude.json` |
| [Project](#project-scope) | 仅当前项目 | 是，通过版本控制 | 项目根目录中的 `.mcp.json` |
| [User](#user-scope) | 您的所有项目 | 否 | `~/.claude.json` |

### Local 作用域

Local 作用域是默认作用域。Local 作用域的服务器仅在添加它的项目中加载，并且仅对您本人可见。Claude Code 将其存储在 `~/.claude.json` 中该项目的路径下，因此同一服务器不会出现在您的其他项目中。Local 作用域适用于个人开发服务器、实验性配置，或包含您不希望纳入版本控制的凭据的服务器。

> [!NOTE]
>
> MCP 服务器的"local 作用域"一词与一般的本地设置不同。MCP local 作用域服务器存储在 `~/.claude.json`（您的主目录）中，而一般的本地设置使用 `.claude/settings.local.json`（位于项目目录中）。有关设置文件位置的详细信息，请参阅[设置](https://code.claude.com/docs/zh-CN/settings.md#where-settings-live)。

```bash theme={null}
# Add a local-scoped server (default)
claude mcp add --transport http stripe https://mcp.stripe.com

# Explicitly specify local scope
claude mcp add --transport http stripe --scope local https://mcp.stripe.com
```

该命令会将服务器写入 `~/.claude.json` 中当前项目对应的条目。以下示例展示了从 `/path/to/your/project` 运行该命令后的结果：

```json theme={null}
{
  "projects": {
    "/path/to/your/project": {
      "mcpServers": {
        "stripe": {
          "type": "http",
          "url": "https://mcp.stripe.com"
        }
      }
    }
  }
}
```

### Project 作用域

Project 作用域的服务器通过将配置存储在项目根目录下的 `.mcp.json` 文件中来支持团队协作。添加 Project 作用域的服务器时，Claude Code 会自动创建或更新此文件，并写入相应的配置结构。请将 `.mcp.json` 提交到版本控制中，以便团队中的每个人都能获得相同的 MCP 工具和服务。

```bash theme={null}
# Add a project-scoped server
claude mcp add --transport http shared-server --scope project https://example.com/mcp
```

生成的 `.mcp.json` 文件遵循标准化格式：

```json theme={null}
{
  "mcpServers": {
    "shared-server": {
      "type": "http",
      "url": "https://example.com/mcp"
    }
  }
}
```

出于安全考虑，在交互式会话中，Claude Code 会在使用 `.mcp.json` 文件中的 Project 作用域服务器之前请求批准。要重置这些批准选择，请运行 `claude mcp reset-project-choices`。

在 `claude -p` 运行、[Agent SDK](https://code.claude.com/docs/zh-CN/headless.md) 会话以及[项目](https://code.claude.com/docs/zh-CN/claude-projects.md#what-threads-pick-up-from-your-repositories)线程之外的[云端会话](https://code.claude.com/docs/zh-CN/claude-code-on-the-web.md)中，Claude Code 无法显示该确认提示：它会直接加载 Project 作用域的服务器而不询问。如果您在用户设置或托管设置中设置了 [`skipDangerousModePermissionPrompt`](https://code.claude.com/docs/zh-CN/settings-reference#skipdangerousmodepermissionprompt)，并以 `bypassPermissions` 模式启动会话，Claude Code 也会跳过该确认提示。如果仍要排除某个服务器：

* 将其添加到 [`disabledMcpjsonServers`](https://code.claude.com/docs/zh-CN/settings-reference#disabledmcpjsonservers)，这会在所有权限模式下阻止它。
* 使用 [`--setting-sources`](https://code.claude.com/docs/zh-CN/cli-reference#cli-flags) 或 SDK 的 `settingSources` 选项完全排除项目设置。
* 使用 [`--strict-mcp-config`](https://code.claude.com/docs/zh-CN/cli-reference#cli-flags) 启动会话。此时 Claude Code 仅使用您通过 `--mcp-config` 传入的 MCP 服务器。对于 Claude Code 未加载的 Project 作用域服务器跳过批准提示，需要 Claude Code v2.1.246 或更高版本；在 v2.1.246 之前，strict 会话仍会等待对这些服务器的批准，导致后台会话在启动时处于等待状态。有关在托管 MCP 文件下该标志的作用，请参阅[使用 managed-mcp.json 进行独占控制](https://code.claude.com/docs/zh-CN/managed-mcp.md#exclusive-control-with-managed-mcp-json)。

[项目服务器批准与工作区信任](#project-server-approvals-and-workspace-trust)介绍了提交到仓库中的批准如何与工作区信任相互作用。

### User 作用域

User 作用域的服务器存储在 `~/.claude.json` 中，提供跨项目的可访问性，使其在您机器上的所有项目中均可用，同时仅对您的用户账户可见。此作用域适用于个人实用工具服务器、开发工具，或您在不同项目中经常使用的服务。

```bash theme={null}
# Add a user server
claude mcp add --transport http hubspot --scope user https://mcp.hubspot.com/anthropic
```

### 作用域层级与优先级

当同一服务器在多个位置定义时，Claude Code 只会连接一次，并使用优先级最高的来源中的定义。会使用该来源中的整个服务器条目；各作用域之间的字段不会合并。

1. Local 作用域
2. Project 作用域
3. User 作用域
4. [插件提供的服务器](https://code.claude.com/docs/zh-CN/plugins/components.md#mcp-servers)
5. [claude.ai 连接器](#use-mcp-servers-from-claude-ai)

Claude Code 在三个作用域之间按名称匹配重复项。对于插件和连接器，则按端点匹配，因此如果某个插件或连接器指向的 URL 或命令与上述某个已启用的服务器相同，则视为重复项。

如果两种 URL 写法仅在以下方面不同，则视为同一端点：scheme 或 host 的字母大小写、scheme 的默认端口（例如 `https` 上的 `:443`），或末尾的斜杠。路径、查询字符串、userinfo 或非默认端口不同，则视为两个服务器。

您的组织通过 [`managedMcpServers`](https://code.claude.com/docs/zh-CN/managed-mcp#provide-servers-through-managed-settings) 托管设置提供的服务器优先级高于上述所有来源，因此当上述某个服务器与之重复时，Claude Code 会连接组织的定义。需要 Claude Code v2.1.259 或更高版本。

如果您在 [Desktop 应用的 Code 标签页](https://code.claude.com/docs/zh-CN/desktop.md#mcp-servers-from-the-claude-desktop-chat-app)中打开本地会话，而 `~/.claude.json` 顶层（User 作用域）和 `.mcp.json` 中存在同名的 stdio 服务器，Code 标签页会使用 `~/.claude.json` 中的定义。

### `.mcp.json` 中的环境变量扩展

Claude Code 支持在 `.mcp.json` 文件中进行环境变量扩展，使团队能够共享配置，同时对特定于机器的路径和 API 密钥等敏感值保持灵活性。

您可以在服务器的 `command`、`args`、`env`、`url` 或 `headers` 值中引用环境变量，支持以下两种形式：

* `${VAR}`：扩展为环境变量 `VAR` 的值
* `${VAR:-default}`：如果已设置 `VAR`，则扩展为 `VAR` 的值；如果未设置，则扩展为 `default`

#### 变量扩展示例

在此服务器条目中，URL 是 `API_BASE_URL` 的值后接 `/mcp`，默认值为 `https://api.example.com`，API 密钥来自 `API_KEY`：

```json theme={null}
{
  "mcpServers": {
    "api-server": {
      "type": "http",
      "url": "${API_BASE_URL:-https://api.example.com}/mcp",
      "headers": {
        "Authorization": "Bearer ${API_KEY}"
      }
    }
  }
}
```

#### 未设置且无默认值的变量

如果引用的环境变量未设置且没有默认值，配置仍会加载：Claude Code 会在 `claude mcp list` 输出中针对该服务器报告缺少变量的警告，并按原样使用未扩展的 `${VAR}` 文本。请设置该变量或添加 `:-default` 回退值，以便服务器使用您预期的值启动。在远程服务器的 `url` 和 `headers` 中，某些凭据变量则会[读取为空](#credential-variables-that-read-as-empty)，且不会给出警告。

#### 设置为空字符串的变量

如果您将变量设置为空字符串，它仍被视为已设置，因此 `${VAR:-default}` 引用会扩展为空字符串，而不是 `default`。以[示例服务器](#example-with-variable-expansion)为例，如果您将 `API_BASE_URL` 设置为空字符串，其 `url` 会变为 `/mcp`，并且 `claude mcp list` 会报告 `'url' is not a valid URL`。要使用默认值，请取消设置该变量，而不是将其设置为空字符串。

#### 读取为空的凭据变量

在远程服务器的 `url` 和 `headers` 中，Claude Code 会将环境中的凭据变量读取为空，而不是扩展它们。这可以防止项目的 `.mcp.json` 或插件将您的 Claude Code 或云提供商凭据发送到其指定的服务器。如果您写入 `Bearer ${ANTHROPIC_AUTH_TOKEN}`，服务器会收到不含凭据的 `Bearer `，并拒绝该请求，通常返回 `401`。Claude Code 会将其报告为连接失败。

涵盖的名称包括：

* Claude Code 自身的凭据，例如 `ANTHROPIC_API_KEY` 和 `ANTHROPIC_AUTH_TOKEN`
* 您的云提供商凭据，例如 `AWS_BEARER_TOKEN_BEDROCK`
* 您的环境中携带的其他凭据，例如 `HTTPS_PROXY` 和 `NPM_TOKEN`

无论您是否设置了该变量，涵盖的名称都会读取为空，并且其上的 `:-default` 回退值会被忽略。`ANTHROPIC_BASE_URL` 等提供商基础 URL 仍会扩展，因此 `"url": "${ANTHROPIC_BASE_URL}/mcp"` 可以正常工作，除非该 URL 的值本身嵌入了凭据，例如用户名和密码。

此集合之外的名称（例如 `API_KEY`）会按原样扩展。要向服务器提供某个涵盖的凭据，请将其复制到您自己命名的变量中，并改为引用该名称。

当远程服务器的 `url` 或 `headers` 引用了您已设置的涵盖变量时，Claude Code 会在一行调试日志中指出该变量。要查看该行，请运行 `claude --debug-file /tmp/claude-debug.log`，并在该文件中搜索 `never expanded toward a remote server`。

#### 引用在 `/mcp` 和 CLI 输出中的显示方式

对于 local、project 或 user [作用域](#mcp-installation-scopes)中的服务器，以下使用入口会按名称显示 `${VAR}` 引用，而不是显示其解析后的值：

* 服务器 `/mcp` 详情视图中的 URL 或命令行
* `claude mcp list` 和 `claude mcp get` 的输出

在 Claude Code v2.1.268 或更高版本中，`/mcp` 详情视图会以这种方式显示引用。

对于您的组织通过 `managedMcpServers` 设置提供的服务器，这些使用入口仅显示 [URL 的主机](https://code.claude.com/docs/zh-CN/managed-mcp.md#what-users-can-see-and-change)。

要查看连接失败时 `claude mcp list`、`claude mcp get` 和 `/mcp` 显示的内容，请参阅[服务器状态详情](#server-status-detail)。

## 实际示例

### 示例：连接到 GitHub 进行代码审查

GitHub 的远程 MCP 服务器使用作为标头传递的 GitHub 个人访问令牌进行身份验证。要获取一个，请打开您的 [GitHub 令牌设置](https://github.com/settings/personal-access-tokens)，生成一个新的细粒度令牌，具有对您希望 Claude 使用的存储库的访问权限，然后添加服务器：

```bash theme={null}
claude mcp add --transport http github https://api.githubcopilot.com/mcp/ \
  --header "Authorization: Bearer YOUR_GITHUB_PAT"
```

将 `YOUR_GITHUB_PAT` 替换为您的个人访问令牌。`claude mcp add` 命令保存配置而不验证凭据，因此此处接受占位符值，但服务器稍后无法连接。要验证连接，请运行 `/mcp` 并检查该服务器所在行是否显示对勾。具有错误凭据的服务器会改为显示 ✘，其[失败详情](#server-status-detail)包括服务器返回的 HTTP 状态，例如 401。

然后使用 GitHub：

```text wrap theme={null}
审查 PR #456 并建议改进
```

```text wrap theme={null}
为我们刚发现的错误创建新问题
```

```text wrap theme={null}
显示分配给我的所有开放 PR
```

### 示例：查询您的 PostgreSQL 数据库

[DBHub](https://github.com/bytebase/dbhub)，`@bytebase/dbhub` 包，是一个 MCP 服务器，通过您在 `--dsn` 中传递的连接字符串将 Claude 连接到关系数据库。在连接字符串中使用只读数据库用户，以便 Claude 运行的查询无法修改数据：

```bash theme={null}
claude mcp add --transport stdio db -- npx -y @bytebase/dbhub \
  --dsn "postgresql://readonly:pass@prod.db.com:5432/analytics"
```

要确认服务器启动，请运行 `/mcp` 并检查 `db` 所在行是否显示对勾。

然后自然地查询您的数据库：

```text wrap theme={null}
本月我们的总收入是多少？
```

```text wrap theme={null}
显示订单表的 schema
```

```text wrap theme={null}
查找 90 天内未进行购买的客户
```

## 使用远程 MCP 服务器进行身份验证

许多基于云的 MCP 服务器需要身份验证。Claude Code 支持 OAuth 2.0 以实现安全连接。

当服务器响应 `401 Unauthorized` 或 `403 Forbidden` 时，Claude Code 会将远程服务器标记为需要身份验证。Claude Code 显示的内容取决于服务器：

* 对于您尚未登录的服务器，任一状态代码都会在 `/mcp` 中标记它，以便您可以完成 OAuth 流程。
* 对于 [claude.ai 连接器](#use-mcp-servers-from-claude-ai)，由 claude.ai 拒绝您的会话令牌导致的 `401` 不会标记连接器，因为重新授权连接器无法修复您的登录。Claude Code 改为显示 [会话令牌被拒绝状态](https://code.claude.com/docs/zh-CN/errors.md#claude-ai-rejected-the-session-token)。
* 对于您在 `headers` 中配置了 `Authorization` 标头的服务器，或通过 [`headersHelper`](#use-dynamic-headers-for-custom-authentication) 配置的服务器，连接时的 `401` 或 `403` 不会标记服务器，因为要修复的凭据是您配置的凭据。Claude Code 改为报告连接失败。如果您从 `${VAR}` 引用设置该标头，请检查该变量是否是 Claude Code [读取为空](#credential-variables-that-read-as-empty) 的变量之一。
* 对于 [传递到云端会话的连接器](#how-connectors-reach-claude-code)，Claude Code 不运行登录流程，因为会话的代理使用您在 claude.ai 中授予的授权向连接器进行身份验证。当那里的连接器需要再次授权时，请在 [claude.ai/customize/connectors](https://claude.ai/customize/connectors) 重新连接它，而不是从会话中连接。

当您已登录的 OAuth 服务器以 `401 Unauthorized` 拒绝您存储的令牌时，Claude Code 会刷新令牌、重新连接并重试请求一次。只有在该重试也失败时，它才会在 `/mcp` 中标记服务器。在 macOS 上的 `claude -p` 或 Agent SDK 运行中，Claude Code 在无法读取您存储的令牌时发送的请求会改为以 [存储凭据错误](https://code.claude.com/docs/zh-CN/errors.md#couldnt-read-the-stored-credentials-for-mcp-server) 失败。

当服务器拒绝存储的刷新令牌时，Claude Code 会立即显示一条指向 `/mcp` 的通知。打开 `/mcp` 并在服务器上选择 **Re-authenticate** 以在下一个工具调用失败之前再次登录。

返回指向其授权服务器的 `WWW-Authenticate` 标头的自定义服务器会获得与任何其他远程服务器相同的自动发现。

当一个或多个配置的服务器需要身份验证时，Claude Code 也会显示启动通知，因此您不必打开 `/mcp` 来发现哪些服务器需要登录。该通知仅计算您可以从 Claude Code 登录的服务器。在 v2.1.218 之前，它还计算了在 claude.ai 中未连接的 [claude.ai 连接器](#use-mcp-servers-from-claude-ai)，您只能从 claude.ai 设置中连接这些连接器。

该通知每次宣布每个服务器一次，并在后续启动时将其排除在计数之外，直到该服务器已连接并再次需要登录。`/mcp` 仍然列出每个需要登录的服务器。

`claude -p` 或 Agent SDK 运行没有 `/mcp` 面板，因此 Claude Code 无法在其中运行 OAuth 流程。要使用需要身份验证的服务器，请先在交互式会话中完成登录：在会话中运行 `/mcp`，或在 shell 中运行 `claude mcp login <name>`。当配置的服务器在启用了 [工具搜索](#scale-with-mcp-tool-search) 的 `claude -p` 或 Agent SDK 运行期间需要身份验证时，Claude Code 会告诉 Claude 该服务器的工具在您授权之前不可用。

如果您为服务器配置了 `headers.Authorization`，而服务器拒绝了该标头，Claude Code 会报告连接失败，而不是回退到 OAuth。检查令牌对 MCP 端点是否有效，或删除标头以使用 OAuth 流程。

**1. 添加需要身份验证的服务器**

如果您已在 [MCP 快速入门](https://code.claude.com/docs/zh-CN/mcp-quickstart.md#connect-a-server-that-requires-sign-in) 中添加了 `sentry` 服务器，请跳过此步骤：使用相同的服务器名称在相同的作用域内再次运行 `claude mcp add` 会失败，并显示 `MCP server sentry already exists in local config`。否则，运行：

```bash theme={null}
claude mcp add --transport http sentry https://mcp.sentry.dev/mcp
```

**2. 在 Claude Code 中使用 /mcp 命令**

在 Claude Code 中，使用命令：

```text wrap theme={null}
/mcp
```

然后按照浏览器中的步骤登录。

> [!TIP]
>
> 提示：
>
> * 身份验证令牌存储安全且自动刷新
> * 使用 `/mcp` 菜单中的"Clear authentication"撤销访问权限
> * 如果浏览器未自动打开，请复制提供的 URL 并手动打开
> * 如果浏览器重定向在身份验证后失败并出现连接错误，请将浏览器地址栏中的完整回调 URL 粘贴到 Claude Code 中出现的 URL 提示中
> * OAuth 身份验证适用于 HTTP 服务器

### 从命令行进行身份验证

`claude mcp login <name>` 命令直接从您的 shell 运行配置的服务器的 OAuth 流程，因此您不需要在会话内打开 `/mcp` 面板。对于 claude.ai 连接器，请按照 [从 shell 再次授权连接器](https://code.claude.com/docs/zh-CN/remote-control.md#authorize-a-connector-again-from-your-shell) 操作。

```bash theme={null}
claude mcp login sentry
```

要稍后清除存储的凭据，请运行 `claude mcp logout <name>`。

`claude mcp login` 检测何时没有本地浏览器可用，例如在 SSH 会话期间或在没有显示服务器的 Linux 上，并打印授权 URL 而不是尝试打开浏览器。在您的本地计算机上打开 URL，然后将浏览器地址栏中的完整重定向 URL 粘贴回提示符。该命令需要交互式终端来执行粘贴步骤，因此请使用 `ssh -t` 连接。传递 `--no-browser` 以强制 URL 提示，即使检测到本地浏览器。

```bash theme={null}
claude mcp login sentry --no-browser
```

### 使用固定的 OAuth 回调端口

某些 MCP 服务器需要预先注册的特定重定向 URI。默认情况下，Claude Code 为 OAuth 回调选择随机可用端口。使用 `--callback-port` 固定端口，使其与 `http://localhost:PORT/callback` 形式的预注册重定向 URI 匹配。如果在 Claude Code v2.1.229 上登录失败并出现重定向 URI 不匹配，请参阅 [使用预配置的 OAuth 凭据](#use-pre-configured-oauth-credentials) 下的版本说明。

您可以单独使用 `--callback-port`（使用动态客户端注册）或与 `--client-id` 一起使用（使用预配置的凭据）。

```bash theme={null}
# Fixed callback port with dynamic client registration
claude mcp add --transport http \
  --callback-port 8080 \
  my-server https://mcp.example.com/mcp
```

### 使用预配置的 OAuth 凭据

某些 MCP 服务器不支持通过动态客户端注册进行自动 OAuth 设置。如果您看到类似"Incompatible auth server: does not support dynamic client registration"的错误，服务器需要预配置的凭据。Claude Code 也支持使用客户端 ID 元数据文档 (CIMD) 而不是动态客户端注册的服务器，并自动发现这些服务器。如果自动发现失败，请先通过服务器的开发者门户注册 OAuth 应用，然后在添加服务器时提供凭据。

**1. 向服务器注册 OAuth 应用**

通过服务器的开发者门户创建应用并记下您的客户端 ID 和客户端密钥。

如果注册表单要求重定向 URI，请选择任何可用端口并输入 `http://localhost:PORT/callback`，其中包含该端口。您将在下一步中使用相同的端口。

在 v2.1.229 中，Claude Code 改为发送 `http://127.0.0.1:PORT/callback`，而精确匹配注册重定向 URI 的服务器会以重定向 URI 不匹配拒绝登录。Claude Code v2.1.231 恢复了 `localhost` 形式。要在 v2.1.229 上恢复，请升级 Claude Code，或临时将 `http://127.0.0.1:PORT/callback` 形式添加到服务器的注册重定向 URI。

**2. 使用您的凭据添加服务器**

这些选项卡涵盖两个命令：`claude mcp add` 将您的客户端 ID 和回调端口作为标志，`claude mcp add-json` 在 `oauth` 对象中获取它们。如果您注册了重定向 URI，请将回调端口设置为该 URI 中的端口。

> [tab] claude mcp add
>
> 使用 `--client-id` 传递您的应用的客户端 ID。`--client-secret` 标志使用掩码输入提示输入密钥：
>
> ```bash theme={null}
> claude mcp add --transport http \
>   --client-id your-client-id --client-secret --callback-port 8080 \
>   my-server https://mcp.example.com/mcp
> ```

> [tab] claude mcp add-json
>
> 在 JSON 配置中包含 `oauth` 对象，并将 `--client-secret` 作为单独的标志传递：
>
> ```bash theme={null}
> claude mcp add-json my-server \
>   '{"type":"http","url":"https://mcp.example.com/mcp","oauth":{"clientId":"your-client-id","callbackPort":8080}}' \
>   --client-secret
> ```

> [tab] claude mcp add-json (仅回调端口)
>
> 要仅固定回调端口并让 Claude Code 自动注册客户端，请单独设置 `callbackPort`：
>
> ```bash theme={null}
> claude mcp add-json my-server \
>   '{"type":"http","url":"https://mcp.example.com/mcp","oauth":{"callbackPort":8080}}'
> ```

> [tab] CI / 环境变量
>
> 通过环境变量设置密钥以跳过交互式提示：
>
> ```bash theme={null}
> MCP_CLIENT_SECRET=your-secret claude mcp add --transport http \
>   --client-id your-client-id --client-secret --callback-port 8080 \
>   my-server https://mcp.example.com/mcp
> ```

**3. 在 Claude Code 中进行身份验证**

在 Claude Code 中运行 `/mcp` 并按照浏览器登录流程。

> [!TIP]
>
> 提示：
>
> * 客户端密钥安全地存储在您的系统钥匙链 (macOS) 或凭据文件中，而不是在您的配置中
> * 您只能在添加服务器时设置客户端密钥。当您使用 `claude mcp login` 或从 `/mcp` 进行身份验证时，Claude Code 使用存储的密钥，不会提示输入密钥或读取 `MCP_CLIENT_SECRET`
> * 要稍后添加或更改密钥，请使用 `claude mcp remove <name>` 删除服务器，然后使用 `--client-secret` 和相同的 `--scope` 再次添加它
> * 如果服务器使用没有密钥的公共 OAuth 客户端，请仅使用 `--client-id` 而不使用 `--client-secret`
> * 这些标志仅适用于 HTTP 和 SSE 传输。它们对 stdio 服务器没有影响
> * 使用 `claude mcp get <name>` 验证为服务器配置了 OAuth 凭据

### 覆盖 OAuth 元数据发现

将 Claude Code 指向特定的 OAuth 授权服务器元数据 URL 以绕过默认发现链。当 MCP 服务器的标准端点出错时，或当您想通过内部代理路由发现时，设置 `authServerMetadataUrl`。默认情况下，Claude Code 首先检查 `/.well-known/oauth-protected-resource` 处的 RFC 9728 受保护资源元数据，然后回退到 `/.well-known/oauth-authorization-server` 处的 RFC 8414 授权服务器元数据。

在 `.mcp.json` 中您的服务器配置的 `oauth` 对象中设置 `authServerMetadataUrl`：

```json theme={null}
{
  "mcpServers": {
    "my-server": {
      "type": "http",
      "url": "https://mcp.example.com/mcp",
      "oauth": {
        "authServerMetadataUrl": "https://auth.example.com/.well-known/openid-configuration"
      }
    }
  }
}
```

URL 必须使用 `https://`。元数据 URL 的 `scopes_supported` 覆盖上游服务器宣传的作用域。

### 限制 OAuth 作用域

设置 `oauth.scopes` 以固定 Claude Code 在授权流程期间请求的作用域。这是当上游授权服务器宣传的作用域超过您想要授予的作用域时，将 MCP 服务器限制为安全团队批准的子集的受支持方式。该值是单个空格分隔的字符串，与 RFC 6749 §3.3 中的 `scope` 参数格式匹配。

```json theme={null}
{
  "mcpServers": {
    "slack": {
      "type": "http",
      "url": "https://mcp.slack.com/mcp",
      "oauth": {
        "scopes": "channels:read chat:write search:read"
      }
    }
  }
}
```

`oauth.scopes` 优先于 `authServerMetadataUrl` 和服务器在 `/.well-known` 处发现的作用域。将其保留为未设置以让 MCP 服务器确定请求的作用域集。

当未设置 `oauth.scopes` 时，Claude Code 不会从自动发现的授权服务器元数据中请求完整的 `scopes_supported` 目录。从配置的 `authServerMetadataUrl` 获取的元数据仍会将其 `scopes_supported` 作为请求的作用域提供。

如果授权服务器在 `scopes_supported` 中宣传 `offline_access`，Claude Code 会将其附加到固定作用域，以便可以在没有新浏览器登录的情况下刷新访问令牌。

如果服务器稍后为工具调用返回 403 `insufficient_scope`，调用会失败并显示 [`needs additional permissions`](https://code.claude.com/docs/zh-CN/errors#mcp-server-needs-you-to-sign-in-again) 消息，该消息命名服务器要求的作用域。该服务器在 `/mcp` 中显示为需要身份验证。

如果该作用域不在您的固定 `oauth.scopes` 中，请添加它，然后运行 `/mcp` 并再次对服务器进行身份验证。Claude Code 请求固定作用域而不是服务器命名的作用域，因此如果您在不添加它的情况下再次进行身份验证，您获得的令牌仍然缺少它。

### 对位于需要身份验证的代理后面的服务器使用 OAuth

如果您的 MCP 服务器在一个会检查其自身标头的代理或网关后面使用 OAuth，当 OAuth 请求到达时不带该标头，登录就会失败。通过该代理登录需要 Claude Code v2.1.299 或更高版本，并且需要同时配置该标头和登录：

* **代理的标头**：使用 [`--header`](#option-1-add-a-remote-http-server)、`headers` 字段或 [`headersHelper`](#use-dynamic-headers-for-custom-authentication)，以代理读取的名称传递它
* **OAuth 登录**：不要在配置的标头中包含 `Authorization`，然后在 shell 中使用 `claude mcp login <name>` 登录。如果您配置了 `Authorization` 标头，Claude Code 会将其用作服务器的凭据，并且不会为该服务器回退到 OAuth。

### 使用动态标头进行自定义身份验证

当 Kerberos、短期令牌或内部 SSO 等身份验证需要在连接时生成标头时，请将服务器的 `headersHelper` 设置为一个命令。Claude Code 在连接时运行该命令，并将其输出合并到它发送的标头中。此示例使用脚本：

```json theme={null}
{
  "mcpServers": {
    "internal-api": {
      "type": "http",
      "url": "https://mcp.internal.example.com",
      "headersHelper": "/opt/bin/get-mcp-auth-headers.sh"
    }
  }
}
```

该命令也可以是内联的：

```json theme={null}
{
  "mcpServers": {
    "internal-api": {
      "type": "http",
      "url": "https://mcp.internal.example.com",
      "headersHelper": "echo '{\"Authorization\": \"Bearer '\"$(get-token)\"'\"}'"
    }
  }
}
```

**要求：**

* 该命令必须将字符串键值对的 JSON 对象写入 stdout
* Claude Code 在 shell 中运行该命令，并在 10 秒后放弃
* Claude Code 根据 [您配置服务器的位置](#where-the-helper-runs) 选择命令的工作目录，因此请将脚本作为绝对路径给出或将其放在 `PATH` 上
* 动态标头覆盖任何具有相同名称的静态 `headers`

一旦 [项目和本地作用域服务器的信任规则](#trust-a-folder-before-its-headershelper-runs) 允许助手运行，Claude Code 会在每次连接时（会话启动和重新连接时）重新运行助手。它不缓存结果，因此您的脚本负责任何令牌重用。

如果工具调用返回 `401 Unauthorized` 或 `403 Forbidden`，Claude Code 会自动在相同规则下重新运行助手，使用新标头重新连接，并重试调用一次。Claude Code 仅在该重试也失败时才在 `/mcp` 中将服务器标记为需要身份验证。

当助手的输出包含 `Authorization` 标头时，Claude Code 使用该凭据作为服务器的身份验证，不会回退到服务器的 OAuth。

如果服务器在连接时拒绝助手的凭据，Claude Code 会报告连接失败，而不是将服务器标记为需要身份验证。修复您的助手返回的凭据，然后从 `/mcp` 重新连接以重新运行助手。

Claude Code 在执行助手时设置这些环境变量：

| 变量 | 值 |
| :- | :- |
| `CLAUDE_CODE_MCP_SERVER_NAME` | MCP 服务器的名称 |
| `CLAUDE_CODE_MCP_SERVER_URL` | MCP 服务器的 URL |
| `CLAUDE_PLUGIN_ROOT` | 插件的根目录。仅当 [插件](https://code.claude.com/docs/zh-CN/plugins/components.md#mcp-servers) 提供服务器时设置 |

使用这些来编写为多个 MCP 服务器服务的单个助手脚本。

插件提供的 `headersHelper` 无法引用插件的 [`${user_config.*}`](https://code.claude.com/docs/zh-CN/plugins/manifest-reference#user-configuration) 值，因为该命令通过 shell 运行。Claude Code 报告服务器配置错误并显示 [错误](https://code.claude.com/docs/zh-CN/errors.md#plugin-command-references-user-config)，不替换该值。将 `${user_config.KEY}` 放在服务器的 `headers` 字段中，该字段不是 shell 解析的，或让助手脚本从配置文件读取该值。在 v2.1.207 之前，`headersHelper` 替换了 `${user_config.*}` 值。

#### 助手运行的位置

Claude Code 从声明服务器的配置中选择 `headersHelper` 命令的工作目录。Claude 在 Bash 中运行的 `cd` 不会移动它，[`/cd`](https://code.claude.com/docs/zh-CN/permissions#move-the-session-to-another-directory) 仅对从会话主工作目录运行的服务器移动它。下面的每一行给出您的 `headersHelper` 命令中相对路径解析所依据的目录。

| 您配置服务器的位置 | 工作目录 |
| :- | :- |
| [插件](https://code.claude.com/docs/zh-CN/plugins/components.md#mcp-servers) | 插件的根目录 |
| 项目 `.mcp.json` 或 [本地作用域](#local-scope) 服务器 | 声明服务器的项目目录 |
| 您项目中的 Agent 文件、来自 SDK 的 `mcpServers` 选项或 `setMcpServers()` 方法的服务器，或 [`--mcp-config`](https://code.claude.com/docs/zh-CN/cli-reference) | 会话的 [主工作目录](https://code.claude.com/docs/zh-CN/permissions.md#working-directories) |
| [用户作用域](#user-scope)、[托管 MCP](https://code.claude.com/docs/zh-CN/managed-mcp.md)、[claude.ai 连接器](#use-mcp-servers-from-claude-ai)，或来自您项目外的 Agent 文件，包括来自 `--add-dir` 目录的 Agent 文件 | 您的配置目录，除非您设置了 [`CLAUDE_CONFIG_DIR`](https://code.claude.com/docs/zh-CN/env-vars)，否则为 `~/.claude` |

在 v2.1.238 之前，Claude Code 也从您启动它的目录运行用户作用域、托管和 claude.ai 连接器服务器的助手，以及来自您项目外的 Agent 文件的助手。

#### 助手可以读取哪些变量

仓库或插件提供的 `headersHelper` 是您没有编写的命令，因此 Claude Code 运行它时不使用来自您环境的凭据变量，例如 `ANTHROPIC_API_KEY`。您配置服务器的位置决定这是否适用：

* **已删除**：项目 `.mcp.json` 中的服务器或插件中的服务器，以及来自您项目或 `--add-dir` 目录的 Agent 文件中的内联服务器
* **未删除**：[用户](#user-scope) 或 [本地作用域](#local-scope) 的服务器、[托管 MCP](https://code.claude.com/docs/zh-CN/managed-mcp.md) 中的服务器、来自 [claude.ai 连接器](#use-mcp-servers-from-claude-ai) 的服务器、由 SDK 或 [`--mcp-config`](https://code.claude.com/docs/zh-CN/cli-reference) 提供的服务器，以及来自 `~/.claude/agents/`、来自托管设置或使用 `--agents` 传递的 Agent 文件中的内联服务器

除了 Git 的 `GIT_CONFIG_KEY_<n>` 变量外，Claude Code 会从您的环境中删除每个名称看起来像凭据的变量，例如名称中包含 `TOKEN`、`SECRET`、`PASSWORD`、`KEY` 或 `AUTH` 的名称（无论大小写），因此 `ANTHROPIC_API_KEY` 和 `MY_REGISTRY_TOKEN` 都被删除。Claude Code 也会删除一个固定的凭据变量列表，其名称不遵循该模式，例如 `ANTHROPIC_CUSTOM_HEADERS`。

当这适用于您的助手时，让脚本从文件或凭据存储读取其凭据。如果服务器的 `url` [携带这些变量之一的实时值](#environment-variable-expansion-in-mcp-json)，例如 `MY_REGISTRY_TOKEN`，助手接收的 `CLAUDE_CODE_MCP_SERVER_URL` 值也将该部分替换为 `REDACTED`。

#### 在其 headersHelper 运行之前信任文件夹

Claude Code 将 `headersHelper` 作为任意 shell 命令执行。对于项目 `.mcp.json` 中的服务器或 [本地作用域](#local-scope) 的服务器，它仅在您接受声明服务器的项目目录的 [信任对话](https://code.claude.com/docs/zh-CN/permissions.md#project-allow-rules-and-workspace-trust) 后运行助手。在 v2.1.238 之前，`claude -p` 或 SDK 会话运行这些助手而不检查信任，交互式会话在您信任父文件夹后运行它们。

* **不计算的信任**：父文件夹的信任，以及 `claude -p` 或 SDK 会话为 [设置文件中的 hook](https://code.claude.com/docs/zh-CN/permissions.md#what-runs-before-you-trust-a-folder) 获得的自动信任
* **直到您信任文件夹**：Claude Code 仅使用其静态 `headers` 连接服务器。在 `claude -p` 或 SDK 会话中，它也会向 stderr 打印每个服务器一行 [`headersHelper not run`](https://code.claude.com/docs/zh-CN/errors#headershelper-not-run)，告诉您如何授予信任。
* **无对话的信任**：在 `~/.claude.json` 中将 `projects["<path>"].hasTrustDialogAccepted` 设置为 `true`。`<path>` 是 [项目允许规则和工作区信任](https://code.claude.com/docs/zh-CN/permissions.md#project-allow-rules-and-workspace-trust) 中所述 Claude Code 用作信任键的文件夹。

对于在 [Agent 文件](https://code.claude.com/docs/zh-CN/sub-agents.md#scope-mcp-servers-to-a-subagent) 中内联声明的服务器，Claude Code 应用相同的规则，并检查该 Agent 文件的来源：您的项目（对于其 `.claude/agents/` 目录中的文件），或 `--add-dir` 目录。直到您 [信任该项目或目录本身](https://code.claude.com/docs/zh-CN/permissions.md#what-runs-before-you-trust-a-folder)，Claude Code 都不会加载该服务器，因此其助手也永远不会运行。

### 登录位于需要标头的网关后面的 MCP 服务器

当网关要求某个标头（例如 `X-API-Key`）而 Claude Code 未发送该标头时，对 MCP 服务器的 OAuth 登录会失败。要让 Claude Code 发送该标头，请将其添加到服务器的配置中，并使用 Claude Code v2.1.299 或更高版本：

* **您可以编辑其[配置](#mcp-installation-scopes)的服务器**：在 [`headers`](#example-with-variable-expansion) 中设置它，或从 [`headersHelper`](#use-dynamic-headers-for-custom-authentication) 返回它
* **由[托管 MCP](https://code.claude.com/docs/zh-CN/managed-mcp.md) 或[插件](#plugin-provided-mcp-servers)提供的服务器**：请您的管理员或插件作者将该标头添加到服务器的条目中

随后，Claude Code 会将除 `Authorization` 之外的已配置标头添加到它发送到服务器源的登录、令牌刷新和令牌撤销请求中。服务器的源是其 `url` 中的协议、主机和端口。要求该标头的网关仍可能在授权页面或位于其他源的授权服务器处阻止登录：

* **授权页面**：您在浏览器中的此页面上登录，而浏览器不会发送已配置的标头。如果您的浏览器收到的是网关的错误而不是该页面，请让网关的运维人员允许该页面在没有该标头的情况下通过。
* **位于其他源的授权服务器**：如果该服务器前面的网关也要求该标头，登录会在那里失败。请让该网关的运维人员允许 OAuth 请求在没有该标头的情况下通过。

## 从 JSON 配置添加 MCP 服务器

如果您有 MCP 服务器的 JSON 配置，您可以直接添加它：

**1. 从 JSON 添加 MCP 服务器**

```bash theme={null}
# 基本语法
claude mcp add-json <name> '<json>'

# 示例：添加带有 JSON 配置的 HTTP 服务器
claude mcp add-json weather-api '{"type":"http","url":"https://api.weather.com/mcp","headers":{"Authorization":"Bearer token"}}'

# 示例：添加带有 JSON 配置的 stdio 服务器
claude mcp add-json local-weather '{"type":"stdio","command":"/path/to/weather-cli","args":["--api-key","abc123"],"env":{"CACHE_DIR":"/tmp"}}'

# 示例：添加带有预配置 OAuth 凭据的 HTTP 服务器
claude mcp add-json my-server '{"type":"http","url":"https://mcp.example.com/mcp","oauth":{"clientId":"your-client-id","callbackPort":8080}}' --client-secret
```

**2. 验证服务器已添加**

```bash theme={null}
claude mcp get weather-api
```

> [!TIP]
>
> 提示：
>
> * 确保 JSON 在您的 shell 中正确转义
> * JSON 必须符合 MCP 服务器配置架构
> * 您可以使用 `--scope user` 将服务器添加到您的用户配置而不是项目特定的配置

## 从 Claude Desktop 导入 MCP 服务器

如果您已在 Claude Desktop 中配置了 MCP 服务器，您可以导入它们：

**1. 从 Claude Desktop 导入服务器**

```bash theme={null}
# 基本语法 
claude mcp add-from-claude-desktop 
```

**2. 选择要导入的服务器**

运行命令后，您将看到一个交互式对话框，允许您选择要导入的服务器。

**3. 验证服务器已导入**

```bash theme={null}
claude mcp list 
```

通过 `claude mcp` 命令添加的服务器名称只能包含字母、数字、连字符和下划线。Claude Desktop 不应用该限制，因此名称中包含任何其他字符（如空格）的 Claude Desktop 服务器无法导入。导入会报告它拒绝的每个名称，并仍然导入您选择的其他服务器。

> [!TIP]
>
> 提示：
>
> * 此功能仅在 macOS 和 Windows Subsystem for Linux (WSL) 上有效
> * 它从这些平台上的标准位置读取 Claude Desktop 配置文件
> * 使用 `--scope user` 标志将服务器添加到您的用户配置
> * 导入的服务器将保持与 Claude Desktop 中相同的名称，当名称仅包含字母、数字、连字符和下划线时。Claude Code 会报告名称中包含任何其他字符的服务器并跳过它
> * 如果具有相同名称的服务器已存在，它们将获得数字后缀（例如，`server_1`）

## 使用来自 claude.ai 的 MCP 服务器

如果您已使用 [claude.ai](https://claude.ai) 账户登录 Claude Code，您在 claude.ai 中添加的 MCP 服务器（称为 [connectors](https://claude.com/docs/connectors)）会自动在 Claude Code 中可用：

**1. 在 claude.ai 中配置 MCP 服务器**

在 [claude.ai/customize/connectors](https://claude.ai/customize/connectors) 添加服务器。在 Team 和 Enterprise 计划中，只有管理员可以添加服务器。

**2. 对 MCP 服务器进行身份验证**

在 claude.ai 中完成任何必需的身份验证步骤。

**3. 在 Claude Code 中查看和管理服务器**

在 Claude Code 中，使用命令：

```text wrap theme={null}
/mcp
```

来自 claude.ai 的服务器会出现在列表中，并带有指示符显示它们来自 claude.ai。

Anthropic 本身也提供一些 connectors，无需您或管理员添加。在 [Claude Docs](https://code.claude.com/docs/zh-CN/artifacts.md#write-a-document-with-claude-docs) 可用的账户上，`/mcp` 列出 `claude.ai Claude Docs`，无需设置，当您要求创建供他人使用的文档时，Claude 会使用它。要仅关闭此 connector，请参阅[禁用 claude.ai connectors](#disable-claude-ai-connectors)。

当您的组织在 claude.ai 中管理其身份验证时，Claude Code 在 `/mcp` 和 [`/plugin`](https://code.claude.com/docs/zh-CN/plugins/install) 管理器中将 connector 标记为 `managed`。Managed 状态不会改变 Claude Code 连接到 connector 的方式或应用您组织的 [工具控制](#organization-controls-on-connector-tools)。

您从未登录过的 Connectors 会折叠在 claude.ai 部分末尾的 `Show unused connectors` 行后面，因此组织预配的列表不会填满面板。选择该行以展开它们。您之前登录过的 connector 即使当前需要重新身份验证，也会保持可见。

Connectors 来自 claude.ai 仅在您的活跃 [身份验证方法](https://code.claude.com/docs/zh-CN/authentication.md#authentication-precedence) 是 claude.ai 订阅登录时才会获取。即使您之前运行过 `/login`，在以下情况下也不会加载：

* `ANTHROPIC_API_KEY`、`ANTHROPIC_AUTH_TOKEN` 或 `apiKeyHelper` 处于活跃状态
* 第三方提供商（如 Amazon Bedrock 或 Google Cloud 的 Agent Platform）处于活跃状态
* `ANTHROPIC_PROFILE`、联合变量或活跃的 [Anthropic 配置文件](https://code.claude.com/docs/zh-CN/authentication.md#anthropic-profiles-and-federation-credentials) 提供凭据
* `CLAUDE_CODE_OAUTH_TOKEN` 持有来自 [`claude setup-token`](https://code.claude.com/docs/zh-CN/authentication#generate-a-long-lived-token) 的令牌，该令牌只能进行模型请求

如果 `/mcp` 没有列出您添加的 connector，请运行 `/status` 以确认哪个身份验证方法处于活跃状态。取消设置该环境变量，删除 `apiKeyHelper` 设置，或 [关闭配置文件](https://code.claude.com/docs/zh-CN/authentication.md#anthropic-profiles-and-federation-credentials)，然后运行 `/login` 以选择您的 claude.ai 账户。

如果临时网络问题导致您的会话启动时 connector 列表无法加载，Claude Code 会在后台重试最多三次，一旦重试成功，connectors 就会出现。如果它们仍未出现，请重启 Claude Code 以再次获取列表。

如果 `/mcp` 显示 connector 为 `session token rejected`，或其详细视图显示 [`claude.ai rejected the session token`](https://code.claude.com/docs/zh-CN/errors#claude-ai-rejected-the-session-token)，则 claude.ai 拒绝了您的 Claude Code 登录中的令牌。再次授权 connector 不会清除此状态，因为被拒绝的不是 connector 在 claude.ai 中的自身授权。要清除它：

1. 运行 `/login` 以重新登录。
2. 从 `/mcp` 重新连接 connector。

在 v2.1.222 之前，Claude Code 将 connectors 标记为需要身份验证，授权它们无法解决此问题。

您在 Claude Code 中添加的服务器[优先于](#scope-hierarchy-and-precedence)指向相同 URL 的 claude.ai connector。发生这种情况时，`/mcp` 将 connector 列为隐藏，并显示如何删除重复项（如果您更希望使用 connector）。

某些 Anthropic 托管的 connectors（如 Microsoft 365、Gmail 和 Google Calendar）不支持来自 Claude Code 的本地 OAuth，因为上游身份提供商仅接受 claude.ai 注册的重定向 URL。当您使用 `claude mcp add` 或在 `.mcp.json` 中添加的服务器指向这些主机之一，并且您从 `/mcp` 或使用 `claude mcp login` 登录时，Claude Code 会显示 [`is Anthropic-hosted and doesn't support local OAuth`](https://code.claude.com/docs/zh-CN/errors#anthropic-hosted-and-doesnt-support-local-oauth)，指导您改为在 [claude.ai/customize/connectors](https://claude.ai/customize/connectors) 连接该服务。

在您使用 `claude mcp remove <name>` 删除您的条目并在 claude.ai 上连接该服务后，connector 会自动出现在 Claude Code 中。

### Connectors 如何到达 Claude Code

哪些设置管理 claude.ai 连接器取决于您的会话在哪里运行，因为只有某些会话从 claude.ai 本身获取连接器。下表中的每一行命名连接器在一种会话中的到达方式以及在那里控制它们的内容。桌面应用的 WSL 会话没有对应的行，因为连接器[仍在逐步向其推出](https://code.claude.com/docs/zh-CN/desktop-wsl.md#check-for-the-terminal-and-connectors)。

| 会话运行的位置 | Connectors 如何到达 | 什么管理它们 |
| :- | :- | :- |
| 终端、[VS Code](https://code.claude.com/docs/zh-CN/vs-code.md)、[JetBrains](https://code.claude.com/docs/zh-CN/jetbrains.md) 和 [Agent SDK](https://code.claude.com/docs/zh-CN/agent-sdk/claude-code-features.md) 会话 | Claude Code 从 claude.ai 获取它们 | 本部分中的设置和 [managed MCP 配置](https://code.claude.com/docs/zh-CN/managed-mcp.md) |
| [云端会话](https://code.claude.com/docs/zh-CN/claude-code-on-the-web.md) | 云主机传入它们 | 您的 claude.ai 组织设置，加上到达会话的 [允许列表和拒绝列表](https://code.claude.com/docs/zh-CN/managed-mcp.md#policy-based-control-with-allowlists-and-denylists) 设置以及运行它的主机上的任何 `managed-mcp.json` |
| [桌面应用](https://code.claude.com/docs/zh-CN/desktop.md) 的本地和 SSH 会话 | 桌面应用在进程中传入它们 | 您组织的 [connector 工具控制](#organization-controls-on-connector-tools) 中的 `blocked` 条目 |

[`disableClaudeAiConnectors`](#disable-claude-ai-connectors)、`ENABLE_CLAUDEAI_MCP_SERVERS` 和 [`allowAllClaudeAiMcps`](https://code.claude.com/docs/zh-CN/settings-reference#allowallclaudeaimcps) 仅作用于第一行，即 Claude Code 自身获取的 connectors。其他两行在这些方面与它不同：

* **云端会话**：到达会话的 `allowedMcpServers` 和 `deniedMcpServers` 条目（例如通过 [server-managed 设置](https://code.claude.com/docs/zh-CN/server-managed-settings.md)）也会过滤传入的 connectors。会话的代理重写每个 connector 的 URL，因此为 connector 自身 URL 编写的 `serverUrl` 模式不会匹配它。要在自托管环境中的 URL 允许列表旁边允许传入的 connectors，请添加 [Connector 流量离开您的网络](https://code.claude.com/docs/zh-CN/self-hosted-environments-deploy.md#connector-traffic-leaves-your-network) 下列出的 `serverUrl` 条目。当运行会话的主机上存在 `managed-mcp.json` 时（例如 [self-hosted runner 主机](https://code.claude.com/docs/zh-CN/self-hosted-environments-configuration.md#mcp-servers)），Claude Code 会删除传入的 connectors，无论您是否设置 `allowAllClaudeAiMcps`。
* **桌面应用本地和 SSH 会话**：桌面应用将 connectors 注册为进程内 `type: "sdk"` 服务器，没有 MCP 设置或 `managed-mcp.json` 到达它们。用户通过在 [claude.ai/customize/connectors](https://claude.ai/customize/connectors) 断开连接来将 connector 排除在自己的会话之外。组织阻止 connector 的 [工具](#organization-controls-on-connector-tools) 或完全关闭 [桌面应用中的 Claude Code](https://code.claude.com/docs/zh-CN/desktop.md#admin-console-controls)。

### 组织对 connector 工具的控制

您的组织可以在 [claude.ai connectors](https://claude.com/docs/connectors) 上设置每个工具的控制。Claude Code 在启动时读取这些设置并在本地强制执行它们，除了桌面应用的 [本地和 SSH 会话](#how-connectors-reach-claude-code)。在那里，桌面应用在传入 connector 之前扣留 `blocked` 工具，`ask` 设置不会到达 Claude Code，因此它将会话的普通 [权限规则](https://code.claude.com/docs/zh-CN/permissions.md) 应用于这些工具，而不是在每次调用时提示。在 Claude Code 自身获取 connectors 的会话中，运行 `/mcp` 以查看哪个设置适用于 connector 上的每个工具。

* **工具设置为 `ask`**：Claude Code 在每次调用时提示，原因是 `Your organization requires approval for this tool`。即使在 `acceptEdits`、`auto` 和 `bypassPermissions` [权限模式](https://code.claude.com/docs/zh-CN/permissions.md#permission-modes)中，提示也会出现。在终端中，该提示不提供记住您选择的选项。匹配该工具的 [Allow 规则](https://code.claude.com/docs/zh-CN/permissions.md)也不会跳过提示。在 `dontAsk` 模式中（从不提示），Claude Code 会改为拒绝调用。
* **工具设置为 `blocked`**：Claude Code 在 Claude 看到它之前过滤掉该工具，因此它从不出现在 Claude 的工具列表中。在 Claude Code 自身获取 connectors 的会话中，`/mcp` 工具列表仍然显示该工具，标记为 `disabled by your organization`。

桌面应用和 claude.ai 聊天应用相同的 `blocked` 设置，因此 Claude 也无法在那里使用被阻止的工具，您无法从桌面应用的会话中扣留工具，同时在聊天中保持其可用。桌面应用跳过其所有工具都被阻止的 connector。

### 禁用 claude.ai connectors

Claude Code 仅将 [`disableClaudeAiConnectors`](https://code.claude.com/docs/zh-CN/settings-reference#disableclaudeaiconnectors) 应用于它 [自身获取](#how-connectors-reach-claude-code) 的 connectors，而不是云主机或桌面应用传入的 connectors。要关闭它获取的 connectors，请在任何设置作用域中将该设置项设置为 `true`：

```json theme={null}
{
  "disableClaudeAiConnectors": true
}
```

此设置使用任何源为真的语义：任何设置源中的 `true` 优先。已检入的项目 `.claude/settings.json` 可以选择退出 Claude Code 自身获取的 connectors，但项目级别的 `false` 无法重新启用用户或策略级别的 `true` 已禁用的 connectors。通过 `--mcp-config` 显式传递的服务器不受影响。

您也可以将 `ENABLE_CLAUDEAI_MCP_SERVERS` 环境变量设置为 `false`，这对当前 shell 会话具有相同的效果：

```bash theme={null}
ENABLE_CLAUDEAI_MCP_SERVERS=false claude
```

要阻止单个 claude.ai connectors 而不是全部，请按名称或 URL 模式将它们添加到 [`deniedMcpServers`](https://code.claude.com/docs/zh-CN/managed-mcp) 中。例如，`"claude.ai Slack"` 的 `serverName` 条目会阻止 Slack connector。您也可以运行 `/mcp` 以仅为当前项目切换 Claude Code 获取的任何 connector。

## 将 Claude Code 用作 MCP 服务器

您可以将 Claude Code 本身用作 MCP 服务器，其他应用程序可以连接到它：

```bash theme={null}
# 启动 Claude 作为 stdio MCP 服务器
claude mcp serve
```

该命令启动时不会打印任何内容。stdio MCP 服务器通过 stdin 和 stdout 进行通信，因此沉默的、被阻止的终端意味着服务器正在运行并等待客户端连接。

您可以通过将此配置添加到 claude\_desktop\_config.json 在 Claude Desktop 中使用它：

```json theme={null}
{
  "mcpServers": {
    "claude-code": {
      "type": "stdio",
      "command": "claude",
      "args": ["mcp", "serve"],
      "env": {}
    }
  }
}
```

> [!WARNING]
>
> **配置可执行文件路径**：`command` 字段必须引用 Claude Code 可执行文件。如果 `claude` 命令不在您系统的 PATH 中，您需要指定可执行文件的完整路径。
>
> 要查找完整路径：
>
> ```bash theme={null}
> which claude
> ```
>
> 然后在您的配置中使用完整路径：
>
> ```json theme={null}
> {
>   "mcpServers": {
>     "claude-code": {
>       "type": "stdio",
>       "command": "/full/path/to/claude",
>       "args": ["mcp", "serve"],
>       "env": {}
>     }
>   }
> }
> ```
>
> 如果没有正确的可执行文件路径，您会遇到类似 `spawn claude ENOENT` 的错误。

> [!TIP]
>
> 提示：
>
> * 在 Claude Desktop 中，尝试要求 Claude 读取目录中的文件、进行编辑等操作。
> * 此 MCP 服务器仅向您的 MCP 客户端公开 Claude Code 的工具，因此您自己的客户端负责为各个工具调用实现用户确认。

### 作为 MCP 客户端读取后台命令的输出

当 `claude mcp serve` 返回的 Bash 或 PowerShell 结果中包含 `backgroundOutputPath` 字段时，表示该调用使其命令[在后台](https://code.claude.com/docs/zh-CN/tools-reference.md#background-commands)继续运行。当您的客户端在调用中设置 `run_in_background: true` 时，命令会在后台运行。前台命令在达到超时时也可能[转入后台](https://code.claude.com/docs/zh-CN/tools-reference.md#foreground-commands-that-move-to-the-background)。

该字段位于结果的 JSON 文本中，保存命令输出文件的路径。

* **读取截至目前的输出**：从您的客户端打开该路径下的文件，或将该路径作为 `file_path` 传递给服务器的 [`Read`](https://code.claude.com/docs/zh-CN/tools-reference#read-tool-behavior) 工具
* **判断命令何时完成**：在 `command` 值末尾附加一个输出固定字符串的 `echo`，例如 `npm test; echo DONE`。一旦该字符串出现在文件中，即表示命令已完成。如果没有该字符串，则没有任何内容标记结束，因为 Claude Code 不会向文件添加结束行，也不会通知您的客户端。
* **读取长输出的末尾部分**：仅传入 `file_path` 的 `Read` 会返回[第一页](https://code.claude.com/docs/zh-CN/tools-reference.md#large-files)，或在文件超出大小限制时失败。请同时传入 `offset` 和 `limit`，这种方式适用于任何文件大小：
  1. 调用 `Read`，将 `offset` 和 `limit` 都设置为 `1`，并从结果中获取 `totalLines`
  2. 再次调用 `Read`，将 `offset` 设置为 `totalLines` 减九，将 `limit` 设置为 `10`
  3. 在第二次调用返回的最后十行中查找您的 `echo` 字符串

`backgroundOutputPath` 字段需要 Claude Code v2.1.295 或更高版本。

## MCP 输出限制和警告

当 MCP 工具产生大量输出时，Claude Code 会帮助管理 token 使用情况，以防止压倒您的对话上下文：

* **输出警告阈值**：当任何 MCP 工具输出超过 10,000 个 token 时，Claude Code 会显示警告
* **可配置限制**：您可以使用 `MAX_MCP_OUTPUT_TOKENS` 环境变量调整允许的最大 MCP 输出 token 数
* **默认限制**：默认最大值为 25,000 个 token
* **范围**：环境变量适用于未声明自己限制的工具。设置了 [`anthropic/maxResultSizeChars`](#raise-the-limit-for-a-specific-tool) 的工具会对文本内容使用该值，而不管 `MAX_MCP_OUTPUT_TOKENS` 设置为什么。返回图像数据的工具仍然受 `MAX_MCP_OUTPUT_TOKENS` 限制
* **超过限制**：当没有图像内容的成功结果超过 token 限制时，Claude Code 会将其保存到文件中，并在对话中用一条消息替换它，该消息指定文件路径，以便 Claude 在需要内容时读取该文件。该文件位于会话的 `tool-results` 目录中，在 [`~/.claude/projects/`](https://code.claude.com/docs/zh-CN/claude-directory#cleaned-up-automatically) 下。
* **来自 HTTP 和 SSE 服务器的响应大小**：当来自 [HTTP](#option-1-add-a-remote-http-server) 或 [SSE](#option-2-add-a-remote-sse-server) 服务器的响应超过 16 MiB 时，Claude Code 会停止读取该响应，并且请求会失败。该限制在解压后适用于单个 JSON 响应体或事件流中的单个事件。如果您负责维护该服务器，请减少每个响应返回的数据量以保持在限制之内，例如对结果进行分页
* **来自 WebSocket 服务器的消息大小**：当 [WebSocket](#option-4-add-a-remote-websocket-server) 服务器发送的单条消息超过 16 MiB 时，Claude Code 会[关闭连接](#mid-session-drops-of-a-remote-server)，该服务器上正在进行的调用都会失败。如果您负责维护该服务器，请减少每条消息返回的数据量以保持在限制之内

已被 Claude Code [移至后台任务](#automatic-backgrounding-of-long-tool-calls)的调用会通过任务通知报告其结果。对于在前台完成的调用，还有另外两项限制：

* **文本结果的字符限制**：对于未声明 [`anthropic/maxResultSizeChars`](#raise-the-limit-for-a-specific-tool) 的工具，当没有图像内容的成功结果长度超过 50,000 个字符时，无论其 token 数是多少，Claude Code 都会将其保存到文件中。设置 `MAX_MCP_OUTPUT_TOKENS` 不会改变此阈值
* **错误结果**：当工具返回标记为 `isError: true` 的结果时，Claude 会将该结果的文本作为工具的错误消息接收。超过约 11,000 个字符的错误文本只会保留其前 5,000 个和后 5,000 个字符，中间带有一个标记，说明删除了多少个字符

要更改 token 限制，请在启动 Claude Code 之前在您的 shell 中设置 `MAX_MCP_OUTPUT_TOKENS`：

```bash theme={null}
export MAX_MCP_OUTPUT_TOKENS=50000
claude
```

### 为特定工具提高限制

如果您正在构建 MCP 服务器，可以通过在工具的 `tools/list` 响应条目中设置 `_meta["anthropic/maxResultSizeChars"]` 来允许单个工具返回超过默认持久化到磁盘阈值（50,000 个字符）的结果。Claude Code 会将该工具的阈值提高到注释值，最高可达 500,000 个字符的硬上限。

这对于返回本质上很大但必要的输出的工具很有用，例如数据库 schema 或完整文件树。如果没有注释，超过默认阈值的成功结果会被持久化到磁盘，并在对话中被替换为文件引用。

```json theme={null}
{
  "name": "get_schema",
  "description": "Returns the full database schema",
  "_meta": {
    "anthropic/maxResultSizeChars": 200000
  }
}
```

该注释对文本内容独立于 `MAX_MCP_OUTPUT_TOKENS` 应用，因此用户不需要为声明它的工具提高环境变量。返回图像数据的工具仍然受 token 限制。

### 工具结果中的图像

当 MCP 工具返回 PNG、JPEG、GIF 或 WebP 图像时，Claude 会在对话中内联看到该图像。内联副本可能会缩小或压缩以适应模型的图像大小限制。Claude Code 还会将原始字节保存到会话的 `tool-results` 目录中的文件中，该目录位于 [`~/.claude/projects/`](https://code.claude.com/docs/zh-CN/claude-directory#cleaned-up-automatically) 下，并向 Claude 提供路径。Claude 随后可以使用 Bash 等工具裁剪、转换或重新使用完整分辨率的文件。

如果您使用 [`--no-session-persistence`](https://code.claude.com/docs/zh-CN/cli-reference#cli-flags) 或 [`CLAUDE_CODE_SKIP_PROMPT_HISTORY`](https://code.claude.com/docs/zh-CN/env-vars) 禁用会话持久化，Claude Code 将不会写入图像文件，Claude 仅接收内联副本。

将 MCP 图像结果保存到文件需要 Claude Code v2.1.283 或更高版本。

## 具有根级组合器的工具输入模式

某些 MCP 服务器将工具的输入模式声明为 JSON Schema 联合，在模式的顶级使用 `anyOf`、`oneOf` 或 `allOf`。Claude API 不接受这些关键字在模式根部。它接受嵌套在 `properties` 内的组合器，Claude Code 会原样发送这些组合器。

具有根级组合器的工具保持可用。在将工具发送到 API 之前，Claude Code 将模式展平为单个对象，并在工具描述前面添加一句话，告诉 Claude 哪些参数组属于一起：

* `allOf`：来自每个分支的属性被合并，每个分支的 `required` 列表仍然适用
* `anyOf` 和 `oneOf`：来自每个分支的属性被合并，每个分支的 `required` 列表在工具描述中描述，而不是由模式强制执行

您的服务器接收 Claude 选择的任何参数，因此请继续在服务器端验证组合。

当 Claude Code 无法生成 API 接受的模式，或在未收到启用重写的远程配置的部署上时，它会跳过该工具，在服务器日志中记录原因，并保持服务器的其他工具可用。

## 具有无效输入架构的工具

> [!NOTE]
>
> 如果会话因按位置指明某个工具的 400 错误而失败，请按照[工具输入 schema 无效](https://code.claude.com/docs/zh-CN/errors.md#tool-input-schema-is-invalid)中的步骤操作。

如果 MCP 服务器的某个工具的输入 schema 违反了 API 强制执行的两条规则中的任意一条，Claude Code 会排除该工具。服务器的其他工具不受影响。Claude Code 在加载服务器的工具时会检查这两条规则：

* 顶级属性名称必须为 1 到 64 个字符长，并且只能使用 ASCII 字母和数字、`_`、`.` 和 `-`
* 架构必须对 JSON Schema draft 2020-12 元架构有效。Claude Code 对未声明 `$schema` 的架构和声明 draft 2020-12 的架构应用此检查。声明任何其他方言的架构会跳过此检查，尽管上面的属性名称检查仍然适用

当 Claude Code 排除一个工具时，它会在服务器的日志中记录原因，并告诉 Claude 它排除了哪些工具以及原因，这样你可以询问 Claude 为什么工具缺失。如果你修复了服务器上的架构，下次 Claude Code 加载服务器的工具时该工具就会回来。

当[功能标志获取](https://code.claude.com/docs/zh-CN/env-vars.md#features-that-need-feature-flag-fetching)处于关闭状态或标志从未到达时（例如在隔离的机器上），违反规则的工具不会被排除：

* Claude 会获得未经更改的工具，服务器的日志会为其记录 `input schema would be rejected by the Anthropic API`
* 如果 API 随后拒绝了该 schema，请参阅 [API 拒绝其输入 schema 的工具](#tools-whose-input-schema-the-api-refuses)

在 v2.1.216 之前，没有部署运行这些检查。

[根级组合器处理](#tool-input-schemas-with-a-root-level-combinator) 是独立的，当标志获取关闭或标志从未到达时保持其自己的行为。

### API 拒绝其输入 schema 的工具

如果调试日志中的某一行包含 `the API refused this tool's input schema`，则该行指明了一个在请求中被 API 拒绝其 schema 的工具。Claude Code 会重新发送该请求，并将该工具声明为不带参数。

* **如果会话因按位置指明某个工具的 400 错误而失败**：请按照[工具输入 schema 无效](https://code.claude.com/docs/zh-CN/errors.md#tool-input-schema-is-invalid)中的步骤操作
* **如果您使用该服务器**：无需执行任何操作。Claude 仍然可以调用该工具，但看不到其参数
* **如果您维护该服务器**：在服务器上修复该工具的输入 schema。该日志行以 `The API said:` 以及 API 拒绝该 schema 的原因结尾。请验证服务器收到的参数，因为 Claude 在调用该工具时看不到其参数

在 v2.1.300 之前，每个包含被拒绝 schema 的轮次都会因[按位置指明该工具的 400 错误](https://code.claude.com/docs/zh-CN/errors.md#tool-input-schema-is-invalid)而失败。

## 名称超过 128 个字符的工具

如果 `/mcp` 列出了某个 Claude 无法调用的工具，请检查该工具的完整名称是否超过 128 个字符。在会话中运行 `/mcp` 并选择该工具，即可查看其完整名称，例如 `mcp__<server>__<tool>`。当 MCP 工具的完整名称超过 128 个字符时，Claude Code 会将其从发送给 Claude 的工具中排除。Claude API 会拒绝包含此类工具的请求，因此排除该工具可以确保服务器的其他工具正常工作。

有关指明该工具的消息以及缩短其完整名称的方法，请参阅 [MCP tool name over 128 characters](https://code.claude.com/docs/zh-CN/errors.md#mcp-tool-name-over-128-characters)。

## 要求对特定工具进行批准

如果您正在构建 MCP 服务器，可以通过在工具的 `tools/list` 响应条目中将 `_meta["anthropic/requiresUserInteraction"]` 设置为 `true` 来标记工具需要在每次调用时获得明确批准。该值必须是 JSON 布尔值 `true`；任何其他值都会被忽略。

Claude Code 会在每次调用时显示该工具的权限提示，即使在 `acceptEdits`、`auto` 和 `bypassPermissions` [权限模式](https://code.claude.com/docs/zh-CN/permissions.md#permission-modes)中也是如此，并且不会为其提供"不再询问"选项。与该工具匹配的 [允许规则](https://code.claude.com/docs/zh-CN/permissions.md#permission-rule-syntax)也不会跳过提示。在 `dontAsk` 模式中（从不提示），Claude Code 会拒绝该调用。

提示必须到达一个人。在非交互模式下使用 [`--permission-prompt-tool`](https://code.claude.com/docs/zh-CN/cli-reference#cli-flags)，来自提示工具的标记工具的 `allow` 结果会被转换为拒绝，消息为 `MCP tool requires user interaction; not supported via --permission-prompt-tool`。Agent SDK 的 [`canUseTool` 回调](https://code.claude.com/docs/zh-CN/agent-sdk/permissions)确实会接收这些调用并可以批准它们，因为您的 SDK 应用程序应该将它们显示给用户。

将此用于权限提示本身就是目的的工具，例如同意或访问授予步骤，其中自动批准意味着没有人类曾经同意。来自同一服务器的其他工具保持其正常的权限行为。

以下 `tools/list` 条目将一个工具标记为始终需要批准。

```json theme={null}
{
  "name": "grant_access",
  "description": "Requests access to a protected resource",
  "_meta": {
    "anthropic/requiresUserInteraction": true
  }
}
```

某些使用入口，例如 [Remote Control](https://code.claude.com/docs/zh-CN/remote-control.md) 和基于 [Agent SDK](https://code.claude.com/docs/zh-CN/agent-sdk/overview.md) 构建的应用程序，通常允许您通过一次点击来批准工具调用。对于使用此注解标记的工具，Claude Code 会禁用一次点击操作并显示工具的完整权限提示，因此批准仍然来自回答提示的人，而不是点击。

Claude Code 对任何只有终端对话框才能完整呈现的权限请求（例如带有安全警告或远程使用入口无法显示的始终允许选项的请求）也会以相同方式禁用一次点击批准。您需要在终端对话框中回答该请求，而不是从 Remote Control 中回答。

## 标记可安全调用两次的工具

如果您构建 MCP 服务器，请标记可以安全调用两次的工具，以便 Claude Code 能够重复在连接关闭时仍处于待处理状态的调用。对于 HTTP 服务器或 [claude.ai 连接器](#use-mcp-servers-from-claude-ai)，Claude Code 可以在新连接上重复此类调用一次。Claude Code 不重复的调用可能会失败，并显示 [`connection closed before the tool call was answered`](https://code.claude.com/docs/zh-CN/errors#connection-closed-before-the-tool-call-was-answered)。

要标记某个工具，请在其 `tools/list` 条目中将以下 `annotations` 之一设置为 `true`：

* **`idempotentHint`**：该工具可以更改内容，并且使用相同参数再次调用是安全的
* **`readOnlyHint`**：该工具不更改任何内容。Claude Code 还会将该工具的调用与其他只读调用并行运行。

例如，`tools/list` 条目中的以下字段将 `set_issue_status` 工具标记为可安全重复：

```json theme={null}
{
  "name": "set_issue_status",
  "annotations": {
    "idempotentHint": true
  }
}
```

## 响应 MCP 引出请求

MCP 服务器可以在任务进行中使用引出功能向你请求结构化输入。当服务器需要无法自行获取的信息时，Claude Code 会显示一个交互式对话框，并将你的响应传回服务器。你无需进行任何配置：当服务器请求引出对话框时，它们会自动出现。

服务器可以通过两种方式请求输入：

* **表单模式**：Claude Code 显示一个对话框，其中包含由服务器定义的表单字段（例如，用户名和密码提示）。填写字段并提交。
* **URL 模式**：Claude Code 询问是否在浏览器中打开链接。服务器使用此模式处理在终端外完成的流程，例如登录。

在 URL 模式中，Claude Code 将 URL 作为命令行参数传递给系统的 URL 处理程序，并限制该参数的长度。当 URL 在为命令行转义后超过该限制时，你只能拒绝该请求。每个需要转义的字符，例如 `%` 或 `&`，都会计为上限的四倍：其自身字符加上三个转义字符。没有这些字符的 URL 在大约 8,000 个字符处达到上限。主要由百分比转义组成的 URL，其中每三个字符中有一个是 `%`，在大约 4,000 处达到上限。

要在不显示对话框的情况下自动响应引出请求，请使用 [`Elicitation` hook](https://code.claude.com/docs/zh-CN/hooks#elicitation)。

如果你正在构建使用引出功能的 MCP 服务器，请参阅 [MCP 引出规范](https://modelcontextprotocol.io/docs/learn/client-concepts#elicitation) 了解协议详情和架构示例。

### Claude Code 声明的引出模式

如果您构建 MCP 服务器，Claude Code 在连接时发送的客户端功能会告知您的服务器可以请求哪些引出模式。Claude Code 在以下连接上声明 `elicitation: {form: {}, url: {}}`，因此其中任何连接上的服务器都可以请求这两种模式：

* 使用 [protocol revision 2026-07-28](#mcp-client-runtimes) 的连接
* 在较早的协议修订版（2025-11-25 及更早版本）上，与用户 Claude Code MCP 配置中的 stdio、`sse`、`http` 和 `ws` 服务器建立的连接。需要 Claude Code v2.1.287 或更高版本

在较早修订版上的其他连接中，例如与 [claude.ai 连接器](#use-mcp-servers-from-claude-ai)的连接，Claude Code 声明的是不带内容的 `elicitation: {}`，这仅允许表单模式的引出请求。

基于 Java MCP SDK 0.17.0 或更早版本构建的服务器可能会拒绝包含 `form` 和 `url` 的声明。请参阅 [MCP server rejected the elicitation capability](https://code.claude.com/docs/zh-CN/errors.md#mcp-server-rejected-the-elicitation-capability)。

## 使用 MCP 资源

MCP 服务器可以公开资源，您可以使用 @ 提及来引用这些资源，类似于引用文件的方式。

### 引用 MCP 资源

**1. 列出可用资源**

在您的提示中键入 `@` 以查看来自所有连接的 MCP 服务器的可用资源。资源与文件一起出现在自动完成菜单中。

**2. 引用特定资源**

使用格式 `@server:protocol://resource/path` 来引用资源：

```text wrap theme={null}
Can you analyze @github:issue://123 and suggest a fix?
```

```text wrap theme={null}
Please review the API documentation at @docs:file://api/authentication
```

**3. 多个资源引用**

您可以在单个提示中引用多个资源：

```text wrap theme={null}
Compare @postgres:schema://users with @docs:file://database/user-model
```

> [!TIP]
>
> 提示：
>
> * 引用资源时，资源会自动获取并作为附件包含
> * 资源路径在 @ 提及自动完成中可进行模糊搜索
> * Claude Code 在服务器支持时自动提供列出和读取 MCP 资源的工具
> * 资源可以包含 MCP 服务器提供的任何类型的内容（文本、JSON、结构化数据等）

MCP Apps UI 资源是具有 `ui://` URI 或 `text/html;profile=mcp-app` 媒体类型的条目：这些是供主机应用程序呈现的页面，而不是供 Claude 读取的内容。它们不会出现在 `@` 建议中或资源列表工具的结果中，仅提供 UI 资源的服务器会显示空资源列表。通过其 URI 读取 UI 资源仍然有效。

## 使用 MCP 工具搜索进行扩展

工具搜索通过延迟加载工具定义直到 Claude 需要时，来保持 MCP 上下文使用量较低。只有工具名称和服务器说明在会话开始时加载，因此添加更多 MCP 服务器对您的上下文窗口的影响最小。Claude Code 不会对每个服务器施加固定的工具上限；实际限制是您的上下文窗口预算。

> [!NOTE]
>
> 工具搜索在 Microsoft Foundry [部署在 Azure 上的部署](https://platform.claude.com/docs/en/build-with-claude/claude-in-microsoft-foundry#hosting-options)上不受支持，这些部署在服务器端拒绝它：Claude Code 检测到拒绝并为该部署改为预先加载 MCP 工具。[`ENABLE_TOOL_SEARCH`](#configure-tool-search) 无法覆盖此设置，因为拒绝来自部署本身。

### 面向 MCP 服务器作者的工具搜索

如果您正在构建 MCP 服务器，启用工具搜索后，服务器说明字段会变得更加有用。服务器说明帮助 Claude 理解何时搜索您的工具，类似于 [skills](https://code.claude.com/docs/zh-CN/skills.md) 的工作方式。

添加清晰、描述性的服务器说明，说明：

* 您的工具处理的任务类别
* Claude 应该何时搜索您的工具
* 您的服务器提供的关键功能

当工具描述或服务器说明超过以下默认限制时，Claude Code 会将其截断，Claude 只能看到截断点之前的文本：

* **服务器说明，以及预先加载的工具的描述**：4,096 个字符
* **Claude 通过工具搜索加载的工具的描述**：16,384 个字符

保持描述和说明简洁，并将关键细节放在开头。当工具搜索将 Claude 的查询与工具描述进行匹配时，它只读取较短限制范围内的部分，因此也要将 Claude 会搜索的词语放在开头附近。只要工具未被延迟，它就会预先加载并沿用较短的限制，例如当[工具搜索关闭](#configure-tool-search)时、当该工具被[标记为预先加载](#per-tool-alwaysload)时，或当其服务器[豁免延迟](#exempt-a-server-from-deferral)时。

在 v2.1.296 之前，较短的限制为 2,048 个字符；在 v2.1.295 之前，该限制适用于每个工具描述，包括通过工具搜索加载的工具描述。

#### 更改描述长度限制

要更改会话中 MCP 服务器的 4,096 和 16,384 个字符的[默认限制](#for-mcp-server-authors)，请将 [`CLAUDE_CODE_MAX_MCP_DESCRIPTION_LENGTH`](https://code.claude.com/docs/zh-CN/env-vars#variables) 设置为字符数。Claude Code 会将该数值同时用于这两个限制。例如，设置为 `8192` 时，会提高服务器说明和预先加载的工具描述的 4,096 个字符限制，并降低通过工具搜索加载的工具描述的 16,384 个字符限制。此变量需要 Claude Code v2.1.280 或更高版本。

Anthropic 在 `api.anthropic.com` 托管的部分 MCP 服务器，其描述和说明会完整地传达给 Claude，设置此变量不会截断它们。

#### 将工具标记为预先加载或保持延迟

要控制服务器中某个工具的加载方式，请在该工具的 `_meta` 对象中设置 `"anthropic/alwaysLoad"`。将您的服务器添加到 Claude Code 的人也可以在其配置中为整个服务器设置 [`alwaysLoad`](#exempt-a-server-from-deferral)，并且其设置可以覆盖您的设置：

| 您的工具的值 | 结果 |
| :- | :- |
| `true` | 该工具预先加载。启动不会因为此值而等待您的服务器。配置您的服务器的人仍然可以[延迟其所有工具](#defer-a-servers-tools) |
| `false` | 当其配置设置了 `"alwaysLoad": true` 时，该工具保持延迟。这适用于通过 [`--mcp-config`](https://code.claude.com/docs/zh-CN/cli-reference#cli-flags) 传入、由 [Agent SDK 应用程序](https://code.claude.com/docs/zh-CN/agent-sdk/mcp.md#in-code)提供或由[插件](#plugin-provided-mcp-servers)提供的服务器。在其他服务器上，该工具预先加载。需要 Claude Code v2.1.285 或更高版本 |

以下 `tools/list` 条目请求预先加载一个工具：

```json theme={null}
{
  "name": "search_tickets",
  "description": "Searches the ticket tracker by keyword",
  "_meta": {
    "anthropic/alwaysLoad": true
  }
}
```

### 配置工具搜索

工具搜索默认启用：MCP 工具被延迟并按需发现。

设置 [`CLAUDE_CODE_DISABLE_EXPERIMENTAL_BETAS`](https://code.claude.com/docs/zh-CN/env-vars) 保持工具搜索关闭。您无法通过自己设置 `ENABLE_TOOL_SEARCH` 来覆盖它。在 Claude Code v2.1.227 或更高版本上，您的组织可以通过[托管设置](https://code.claude.com/docs/zh-CN/managed-settings.md)在设置此变量的情况下保持工具搜索开启。在[获得 HIPAA 配置](https://code.claude.com/docs/zh-CN/hipaa-setup.md#check-how-developers-sign-in-and-connect)的会话中，工具搜索处于关闭状态，`ENABLE_TOOL_SEARCH` 和托管设置都无法将其重新开启。[禁用预发布功能](https://code.claude.com/docs/zh-CN/llm-gateway-protocol.md#disable-pre-release-capabilities) 涵盖了覆盖应用的位置以及变量剥离的内容。

Claude Code 在 Claude Opus 4、Claude Sonnet 4、Claude Haiku 4.5 及更高版本的模型上使用工具搜索；在 Google Cloud 的 Agent Platform 上，则在 Opus 4.5、Sonnet 4.5、Haiku 4.5 及更高版本的模型上使用。在更早的模型上，Claude Code 预先加载所有 MCP 工具，且 `ENABLE_TOOL_SEARCH=true` 不会覆盖此行为。

在 Google Cloud 的 Agent Platform 上，Claude Code 按模型代数决定：

* **Claude Opus 4.5、Sonnet 4.5、Haiku 4.5 及更高版本**：工具搜索默认开启，与 Anthropic API 上相同。
* **早期 Agent Platform 模型**：Claude Code 预先加载所有 MCP 工具，因为它们的服务堆栈拒绝所需的 beta 标头。`ENABLE_TOOL_SEARCH=true` 不会覆盖此设置。

在 v2.1.221 之前，Claude Code 在 Google Cloud 的 Agent Platform 上为所有模型禁用工具搜索，除非您设置 `ENABLE_TOOL_SEARCH=true`。

使用 `ENABLE_TOOL_SEARCH` 环境变量控制工具搜索行为：

| 值 | 行为 |
| :- | :- |
| (未设置) | 所有 MCP 工具延迟并按需加载。在早于 Opus 4 和 Sonnet 4 的模型上、在[早期 Agent Platform 模型](#configure-tool-search)上，以及在部署在 Azure 上的 Microsoft Foundry 部署上，改为预先加载 |
| `true` | MCP 工具延迟，回退情况与（未设置）相同 |
| `auto` | 阈值模式。原本会被延迟的工具在其定义总计少于上下文窗口的 10% 时预先加载。达到 10% 或更多时，工具搜索会开启，即使总量之后降至 10% 以下，这些工具也可能[保持延迟](#tools-that-stay-deferred-below-the-threshold) |
| `auto:N` | 具有自定义百分比的阈值模式，其中 `N` 是 0-100。例如，`auto:5` 表示 5% |
| `false` | 所有 MCP 工具预先加载，无延迟 |

```bash theme={null}
# 使用自定义 5% 阈值
ENABLE_TOOL_SEARCH=auto:5 claude

# 完全禁用工具搜索
ENABLE_TOOL_SEARCH=false claude
```

或在您的 [settings.json `env` 字段](https://code.claude.com/docs/zh-CN/settings-reference#env) 中设置该值。

您也可以特别禁用 `ToolSearch` 工具：

```json theme={null}
{
  "permissions": {
    "deny": ["ToolSearch"]
  }
}
```

如果您拒绝 `ToolSearch` 或未将其包含在 [`--tools`](https://code.claude.com/docs/zh-CN/cli-reference#cli-flags) 中，MCP 工具会改为预先加载，但请求仍可能携带一个占位的 `defer_loading` 条目。如果网关拒绝该条目，请参阅[通过 LLM 网关使用工具搜索](#tool-search-through-a-gateway)。

#### 通过 LLM 网关使用工具搜索

> [!NOTE]
>
> 以下情况在其他地方介绍：
>
> * **您已设置 `CLAUDE_CODE_DISABLE_EXPERIMENTAL_BETAS`**：请参阅[它如何影响工具搜索](#configure-tool-search)
> * **请求失败，且错误中提到 `defer_loading` 或 `tool_reference`**：请参阅 [Extra inputs are not permitted](https://code.claude.com/docs/zh-CN/errors.md#extra-inputs-are-not-permitted)

当 `ANTHROPIC_BASE_URL` 指向网关时，工具搜索默认开启，因此请求会包含 `defer_loading` 工具字段和 `tool_reference` 块，供网关转发。工具搜索开启时，MCP 工具会被[延迟](#scale-with-mcp-tool-search)。如需改为预先加载所有工具，请在 shell 或您的 [settings `env`](https://code.claude.com/docs/zh-CN/settings-reference#env) 中设置 `ENABLE_TOOL_SEARCH=false`。在 v2.1.304 之前，在网关后面工具搜索处于关闭状态，除非 `ENABLE_TOOL_SEARCH` 为 `true`、`auto` 或 `auto:N`。

在 v2.1.305 或更高版本上，当出现原因无法识别的 `400` 或 `422` 时，Claude Code 可以[在不使用工具搜索的情况下重试请求](https://code.claude.com/docs/zh-CN/llm-gateway-protocol.md#retry-without-tool-search)。重试成功时，屏幕上不会显示任何内容，您也无需执行任何操作：MCP 工具会改为预先加载。Claude Code 稍后会再次尝试工具搜索，代价是再产生一次被拒绝的请求。要跳过这些请求，请设置 `ENABLE_TOOL_SEARCH=false`。

#### 低于 `ENABLE_TOOL_SEARCH=auto` 阈值时仍保持延迟的工具

如果 [`/context`](https://code.claude.com/docs/zh-CN/commands#all-commands) 将工具标记为 `(deferred)` 或 `(loaded on-demand)`，尽管它们的定义总计低于 `auto` 阈值，这说明工具搜索在此对话中已经开启并保持开启。总量之后可能会下降，例如在您恢复会话后 MCP 服务器重新连接期间。您无需执行任何操作：当任务需要时，Claude 会加载其中的每个工具。

### 豁免服务器不延迟

如果服务器的工具应该始终对 Claude 可见而无需搜索步骤，请在该服务器的配置中将 `alwaysLoad` 设置为 `true`。之后，该服务器的工具会加载到上下文中，无论 `ENABLE_TOOL_SEARCH` 设置如何。对于 Claude 在每一轮都需要的少量工具使用此选项，因为每个预先加载的工具会消耗本来可用于您的对话的上下文。

以下 `.mcp.json` 条目豁免一个 HTTP 服务器，同时保持其他服务器延迟：

```json theme={null}
{
  "mcpServers": {
    "core-tools": {
      "type": "http",
      "url": "https://mcp.example.com/mcp",
      "alwaysLoad": true
    }
  }
}
```

`alwaysLoad` 字段在所有服务器类型上都可用。

设置 `alwaysLoad: true` 也会使启动等待服务器的工具，上限为标准 5 秒连接超时，因为它们必须在构建第一个提示词时存在。具有有效 [`cached` 条目](#server-status-detail) 的远程服务器从缓存提供其工具而无需连接，因此它不会延迟启动。其他服务器默认在后台连接；设置 [`MCP_CONNECTION_NONBLOCKING=0`](https://code.claude.com/docs/zh-CN/env-vars) 也使启动等待它们。

### 延迟服务器的工具

要让服务器的所有工具都保持在工具搜索之后，请在 MCP 配置中该服务器的条目里设置 `"alwaysLoad": false`。这包括服务器作者已[标记为预先加载](#per-tool-alwaysload)的工具。如果您省略 `alwaysLoad`，这些已标记的工具会预先加载。需要 Claude Code v2.1.287 或更高版本。

## 将 MCP 提示用作命令

MCP 服务器可以公开提示，这些提示在 Claude Code 中作为命令可用。

来自名为 `anthropic-skills` 的服务器的提示不会出现，因为 Claude Code [保留该名称](https://code.claude.com/docs/zh-CN/skills.md#names-reserved-for-synced-skills)用于从 claude.ai 同步的 skills。该服务器的工具仍然有效。在您的 MCP 配置中重命名服务器以列出其提示。

### 执行 MCP 提示

**1. 发现可用的提示**

输入 `/` 以查看可用的命令，包括来自 MCP 服务器的命令。Claude Code 将每个 MCP 提示列为 `/servername:promptname (MCP)`。输入 `/mcp__servername__promptname` 也可以运行它。

**2. 执行没有参数的提示**

```text wrap theme={null}
/mcp__github__list_prs
```

**3. 执行带有参数的提示**

许多提示接受参数。在命令后面用空格分隔传递它们。Claude Code 在空格处分割参数，因此每个参数是单个令牌：

```text wrap theme={null}
/mcp__github__pr_review 456
```

```text wrap theme={null}
/mcp__jira__create_issue login-bug high
```

> [!TIP]
>
> 提示：
>
> * MCP 提示从连接的服务器动态发现
> * 参数根据提示的定义参数进行解析
> * 提示结果直接注入到对话中
> * 在 `/mcp__servername__promptname` 形式中，Claude Code 将服务器名称中 `A-Z`、`a-z`、`0-9`、`_` 和 `-` 之外的任何字符替换为 `_`，并使用服务器声明的提示名称

## 托管 MCP 配置

对于需要集中控制用户可以连接到哪些 MCP 服务器的组织，请参阅[托管 MCP 配置](https://code.claude.com/docs/zh-CN/managed-mcp.md)。它涵盖使用 `managed-mcp.json` 部署固定服务器集、使用 `managedMcpServers` 为每个用户提供服务器、使用 `allowedMcpServers` 和 `deniedMcpServers` 限制服务器，以及当服务器被阻止时用户看到的内容。
