> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# 连接到 MCP 服务器

> 将 MCP 服务器添加到 Claude Code，验证连接，并在磁盘上找到配置。

[Model Context Protocol (MCP)](https://modelcontextprotocol.io/introduction) 让 Claude Code 能够使用超出其内置工具集的工具，例如搜索问题跟踪器、查询数据库或控制网络浏览器。这些工具来自 MCP 服务器，这些服务器在您的机器上运行或作为托管服务运行。

本指南将引导您通过 Claude Code CLI 端到端地连接一个 MCP 服务器。完成后，您将拥有一个已连接且响应的服务器，了解其配置在磁盘上的位置，并知道如何修复最常见的连接错误。

> [!NOTE]
>
> 您也可以从其他界面添加 MCP 服务器，包括桌面应用、VS Code 和网络版。请参阅[从其他界面连接](#connect-from-other-surfaces)。

有关在 Claude Code 中连接和配置 MCP 服务器的所有方式，请参阅 [MCP 参考](https://code.claude.com/docs/zh-CN/mcp.md)。

## 开始之前

确保您拥有：

* [已安装](https://code.claude.com/docs/zh-CN/quickstart.md)并验证了 Claude Code
* 在项目目录中打开的终端。任何目录都可以，包括空目录。

## 添加并验证服务器

下面的示例连接到 [Claude Code 文档 MCP 服务器](https://code.claude.com/docs/mcp)，这是一个托管服务器，具有对 Claude Code 文档的全文搜索功能。它不需要身份验证或任何特殊配置，因此它很适合作为第一个服务器来测试设置流程。

所有服务器的步骤都相同：添加它、检查连接状态，然后在会话中使用它，最后可选择清理步骤。某些服务器会添加一个步骤，例如浏览器登录，如 [其他 MCP 服务器示例](#additional-mcp-server-examples) 中所示。有关更多要连接的服务器，请浏览 [Anthropic 目录](https://code.claude.com/docs/zh-CN/mcp.md#find-and-build-mcp-servers)。

**1. 添加 MCP 服务器**

向 Claude Code 注册服务器。在您的终端中运行此命令，而不是在 `claude` 会话内：您在启动对话之前配置服务器。

```bash theme={null}
claude mcp add --transport http claude-code-docs https://code.claude.com/docs/mcp
```

命令的各部分：

* `claude mcp add`：向 Claude Code 注册服务器。
* `--transport http`：服务器托管在 URL 上，而不是作为本地进程运行。
* `claude-code-docs`：您创建的名称。将同一服务器称为 `docs` 会完全相同。Claude Code 使用您选择的任何名称来标记 Claude 输出中的服务器工具，并在 `claude mcp remove` 等命令中引用服务器。
* `https://code.claude.com/docs/mcp`：服务器托管的 URL。

该命令打印一个确认信息，如 `Added HTTP MCP server claude-code-docs with URL: https://code.claude.com/docs/mcp to local config`，后跟一个 `File modified:` 行，显示它写入的配置文件。`local config` 部分意味着服务器已注册给您，在此项目中：如果您在不同的项目中启动 Claude Code，此服务器在那里不活跃。要为所有项目注册一次服务器，请在用户范围内添加它，详见 [更改服务器范围](#change-server-scope)。

**2. 检查连接状态**

确认服务器出现在您的服务器列表中并检查其状态：

```bash theme={null}
claude mcp list
```

服务器显示状态指示器：

| 状态 | 含义 |
| :- | :- |
| `✔ Connected` | 准备就绪。这是您应该为 `claude-code-docs` 看到的 |
| `! Connected · tools fetch failed` | 服务器已连接但无法列出其工具。运行 `claude mcp get <name>` 以获取错误详情 |
| `! Needs authentication` | 服务器可以访问但需要浏览器登录，或使用 `--header` 传递的令牌。请参阅[连接需要登录的服务器](#connect-a-server-that-requires-sign-in) |
| `✘ Failed to connect` | 服务器没有响应。请参阅 [故障排除](#troubleshooting) |
| `✘ Connection error` | 连接尝试抛出错误。请参阅 [故障排除](#troubleshooting) |
| ``⏸ Pending approval (run `claude` to approve)`` | 您尚未批准的项目范围服务器。请参阅 [直接编辑 .mcp.json](#edit-mcp-json-directly) |
| `⊘ Disabled for this project (re-enable via /mcp)` | 由项目的 `disabledMcpServers` 列表为此项目关闭的服务器。请参阅 [禁用服务器而不删除它](https://code.claude.com/docs/zh-CN/mcp.md#disable-a-server-without-removing-it) |
| `✘ Blocked by enterprise managed policy` | 您组织的某项策略阻止了此服务器，因此 Claude Code 不会连接。请询问您的管理员是哪项策略。请参阅 [服务器状态](https://code.claude.com/docs/zh-CN/mcp.md#server-status) |

某些旧版 Windows 控制台，例如 Windows 10 上的默认控制台，不支持这些 Unicode 字形，并显示 `√` 和 `×` 代替 `✔` 和 `✘`。

**3. 使用服务器**

启动会话并要求 Claude 按名称使用新服务器：

```bash theme={null}
claude
```

```text wrap theme={null}
Use the claude-code-docs server to look up what MCP_TIMEOUT does
```

> [!NOTE]
>
> 您通常不需要在提示中命名服务器，因为 Claude 会自动选择相关工具。在这里命名它可以保证演示通过新服务器而不是另一个工具（如网络获取）进行，该工具可以回答相同的问题。

如果 Claude Code 在 Claude 第一次调用服务器时要求权限，请批准它。Claude 输出中的工具调用标有服务器名称，这是您确认答案来自 MCP 服务器而不是 Claude 内置知识的方式。

**4. 删除服务器**

此步骤是可选的。完成实验后，您可以删除服务器：

```bash theme={null}
claude mcp remove claude-code-docs
```

该命令确认为 `Removed MCP server "claude-code-docs" from local config`，并显示一个 `File modified:` 行，显示它更新的文件。

> [!NOTE]
>
> 每个已连接的服务器在 [Claude 的上下文窗口](https://code.claude.com/docs/zh-CN/how-claude-code-works.md#the-context-window) 中占用一些空间，因为其工具名称和服务器说明加载到每个会话中。删除您不再使用的服务器可以保持该空间空闲。

## 服务器保存位置

`claude mcp add` 命令将服务器的详细信息写入配置文件。默认情况下，它在 `local` 范围内注册服务器：仅限您私有，仅在当前项目中活跃。传递 `--scope user` 以为所有项目注册一次，或 `--scope project` 以与队友共享。[更改服务器范围](#change-server-scope) 介绍了两者。

> [!NOTE]
>
> `claude mcp add` 在每个 shell 中的工作方式相同，包括 PowerShell 和命令提示符。在 `claude` 会话内，使用 `/mcp` 命令检查和管理您已添加的服务器。

还有其他方式添加服务器，每种都在本页后面介绍：

* [添加本地服务器](#add-a-local-server)：在您的机器上运行程序而不是连接到 URL。
* [直接编辑 `.mcp.json`](#edit-mcp-json-directly)：自己编写 JSON 条目而不是使用命令。
* [连接需要登录的服务器](#connect-a-server-that-requires-sign-in)：添加需要浏览器登录才能使其工具工作的托管服务器。

### 在磁盘上找到您的配置

`claude mcp add` 命令将服务器写入三个范围之一，根据 `--scope` 标志存储在两个文件中。您不需要直接编辑这些文件，但知道它们的位置有助于调试和版本控制。

| 范围 | 文件 | 可用于 |
| :- | :- | :- |
| `local` | `~/.claude.json`，在此项目的条目下 | 仅您，仅此项目。默认值 |
| `project` | 项目根目录中的 `.mcp.json` | 克隆项目的所有人 |
| `user` | `~/.claude.json`，在顶级 `mcpServers` 键下 | 仅您，所有项目 |

在 Windows 上，`~/.claude.json` 解析为 `%USERPROFILE%\.claude.json`，通常为 `C:\Users\YourName\.claude.json`。如果您设置了 [`CLAUDE_CONFIG_DIR`](https://code.claude.com/docs/zh-CN/env-vars)，Claude Code 会从该目录内读取 `.claude.json`。

运行 `claude mcp get claude-code-docs` 以查看哪个范围保存服务器的定义。有关当同一服务器在多个范围中定义时范围如何交互，请参阅 [MCP 安装范围](https://code.claude.com/docs/zh-CN/mcp.md#mcp-installation-scopes)。

## 更改服务器范围

服务器的范围在添加时是固定的，因此更改范围意味着删除条目并在新范围内重新添加它。下面两种情况都从删除第一个演练中的本地条目开始，因此服务器只有一个定义。如果您已在该演练结束时删除了它，请跳过此命令：

```bash theme={null}
claude mcp remove claude-code-docs --scope local
```

### 在所有项目中使用服务器

在 `user` 范围内重新添加服务器，使其在您打开的每个项目中活跃，仍然仅限您私有：

```bash theme={null}
claude mcp add --scope user --transport http claude-code-docs https://code.claude.com/docs/mcp
```

### 与您的团队共享服务器

在 `project` 范围内重新添加服务器，该范围写入项目根目录中的 `.mcp.json`：

```bash theme={null}
claude mcp add --scope project --transport http claude-code-docs https://code.claude.com/docs/mcp
```

将 `.mcp.json` 提交到版本控制。克隆存储库并启动 Claude Code 的队友会看到批准服务器的提示，然后它也会为他们连接。

## 其他 MCP 服务器示例

第一个演练使用了无需任何登录即可连接的托管服务器。下面的示例涵盖了其他两种常见形式，具有相同的添加、检查、使用流程。

### 添加本地服务器

本地 stdio 服务器是 Claude Code 在您的机器上作为子进程启动的程序，而不是它通过 URL 访问的服务。对于需要访问本地资源（如浏览器、您的文件系统或数据库套接字）的工具，请使用一个。

[Playwright MCP 服务器](https://github.com/microsoft/playwright-mcp)是一个很好的尝试：它为 Claude 提供了一个可以导航、点击和读取的浏览器，并且不需要帐户。它通过 `npx` 运行，因此需要 [Node.js](https://nodejs.org/en/download) 18 或更高版本。

**1. 添加 Playwright 服务器**

使用 Claude Code 应该运行以启动它的命令注册服务器：

```bash theme={null}
claude mcp add playwright -- npx -y @playwright/mcp@latest
```

此命令与托管示例的不同之处有三个：

* 没有 `--transport` 标志，因为本地服务器使用默认的 `stdio` 传输。
* `--` 分隔符之后的所有内容都是 Claude Code 运行以启动服务器的命令。
* `-y` 告诉 `npx` 安装包而不提示。

该命令打印一个确认信息，如 `Added stdio MCP server playwright with command: npx -y @playwright/mcp@latest to local config`，后面跟着一个 `File modified:` 行，显示它写入的配置文件。

Playwright 驱动您机器上已安装的任何 Chrome。要使用不同的浏览器，请在 `@playwright/mcp@latest` 之后附加 `--browser` 和浏览器名称，例如 `--browser firefox`。

**2. 检查连接**

`Added` 确认意味着条目已保存，而不是命令运行。检查连接：

```bash theme={null}
claude mcp list
```

第一次检查可能会在 `npx` 下载包时显示 `✘ Failed to connect`，因此请稍等片刻并再次运行。下载完成后，状态更改为 `✔ Connected`。如果在重试几次后仍然显示 `✘ Failed to connect`，请参阅[故障排除](#troubleshooting)。

**3. 使用浏览器**

给 Claude 一个需要浏览器的任务：

```text wrap theme={null}
Use playwright to open https://example.com and tell me the page title
```

浏览器窗口打开，以便您可以观看它工作，Claude 输出中的工具调用标有 `playwright` 服务器名称和操作，如 `browser_navigate`。

尝试将其指向您的本地开发服务器以检查页面在更改后是否仍然呈现，或让它逐步完成错误报告。

### 连接需要登录的服务器

托管服务（如 Sentry、Linear 和 Notion）在 OAuth 后面运行其 MCP 服务器：您添加服务器的 URL，然后通过浏览器登录。

下面的步骤以 Sentry 为例。要连接不同的服务，请替换其 URL，您可以在 [Anthropic 目录](https://code.claude.com/docs/zh-CN/mcp.md#find-and-build-mcp-servers)或服务的文档中找到。

**1. 添加服务器**

`add` 命令与文档服务器相同，使用 Sentry 的 URL：

```bash theme={null}
claude mcp add --transport http sentry https://mcp.sentry.dev/mcp
```

添加后，`claude mcp list` 显示服务器为 `! Needs authentication`。这是预期的：下一步完成登录。

**2. 在浏览器中进行身份验证**

启动 Claude Code 会话并打开 MCP 面板：

```text wrap theme={null}
/mcp
```

从列表中选择 `sentry`，按 Enter，然后选择 `Authenticate`。您的浏览器打开到 Sentry 的登录页面。在那里批准连接。

回到 Claude Code，服务器的状态更改为已连接。如果登录失败或浏览器没有打开，请参阅[故障排除](#troubleshooting)。

**3. 使用服务器**

询问 Claude 需要该服务的内容，例如 `What Sentry projects do I have access to?`，并查找在其输出中标有 `sentry` 服务器名称的工具调用。

使用静态令牌而不是 OAuth 进行身份验证的服务器在添加时使用 `--header "Authorization: Bearer <token>"` 获取令牌。有关已完成的版本，请参阅 [GitHub 示例](https://code.claude.com/docs/zh-CN/mcp.md#example-connect-to-github-for-code-reviews)。

## 直接编辑 .mcp.json

[范围表](#find-your-configuration-on-disk) 中的每个文件都对服务器条目使用相同的 JSON 格式。本部分编辑 `.mcp.json`，即项目范围文件。它最值得手工编写，因为它被检入存储库，在那里它充当团队的配置即代码。

在项目根目录中创建 `.mcp.json`。下面的示例定义了本指南中的两个服务器，通过 HTTP 访问的托管文档服务器和作为本地 `stdio` 进程的 Playwright 服务器：

```json theme={null}
{
  "mcpServers": {
    "claude-code-docs": {
      "type": "http",
      "url": "https://code.claude.com/docs/mcp"
    },
    "playwright": {
      "type": "stdio",
      "command": "npx",
      "args": ["-y", "@playwright/mcp@latest"]
    }
  }
}
```

字段因服务器类型而异：

* 对于 HTTP 服务器，`url` 是 Claude Code 连接到的端点。
* 对于 stdio 服务器，`command` 和 `args` 是它运行的程序。

保存文件后，在项目中启动新的 Claude Code 会话。Claude Code 在启动时读取 `.mcp.json`。

Claude Code 第一次看到项目范围的服务器时，它会要求您批准它。提示存在是为了防止您克隆的存储库在未经您同意的情况下在您的机器上启动进程。批准提示，或如果您错过了，运行 `/mcp` 以稍后批准。

批准后，运行 `/mcp` 并检查服务器是否显示为已连接。如果其中一个显示错误，请参阅[故障排除](#troubleshooting)。

## 从其他界面连接

本指南使用 `claude mcp` CLI 命令，但每个 Claude Code 界面都可以连接到 MCP 服务器：

* **Claude Code 桌面应用**：通过[连接器 UI](https://code.claude.com/docs/zh-CN/desktop.md#connect-external-tools)添加服务器。
* **Claude 桌面聊天应用**：与 Claude Code 不同的应用。要将其 `claude_desktop_config.json` 中的服务器复制到 CLI，请在 macOS 或 WSL 上运行 `claude mcp add-from-claude-desktop`。
* **VS Code**：请参阅[使用 MCP 连接到外部工具](https://code.claude.com/docs/zh-CN/vs-code.md#connect-to-external-tools-with-mcp)。
* **云端会话**：在[项目](https://code.claude.com/docs/zh-CN/claude-projects.md#what-threads-pick-up-from-your-repositories)的线程之外，将 `.mcp.json` 提交到您的仓库；包含一个仓库的会话会加载它。项目中的云端线程则改用您的 claude.ai 连接器。请参阅[直接编辑 .mcp.json](#edit-mcp-json-directly) 和[您的设置中保留的内容](https://code.claude.com/docs/zh-CN/cloud-environments.md#what-carries-over-from-your-setup)。
* **Claude.ai**：您在 [claude.ai/customize/connectors](https://claude.ai/customize/connectors) 添加的连接器在您使用该帐户登录 CLI 时自动加载。请参阅[从 Claude.ai 使用 MCP 服务器](https://code.claude.com/docs/zh-CN/mcp.md#use-mcp-servers-from-claude-ai)。

## 故障排除

如果服务器不连接，请在会话内使用 `/mcp` 或从 shell 使用 `claude mcp list` 检查其状态，然后匹配下面的症状。`/mcp` 面板还允许您重新连接或进行身份验证而无需离开会话。

> [accordion] /mcp 显示未配置 MCP 服务器
>
> Claude Code 没有为当前目录找到任何服务器。最常见的原因：
>
> * 您从不同的项目运行了 `claude mcp add`。本地范围的服务器与您添加它们的项目相关联：存储库根目录，或如果您不在 git 存储库中，则为确切目录。从您现在所在的项目重新添加服务器，或使用 `--scope user` 添加它，以便它不与项目相关联。
> * 您在错误的路径编辑了配置文件。正确的文件是 `~/.claude.json` 和 `<project>/.mcp.json`。Claude Code 不读取 `~/.claude/.mcp.json`、`~/.claude/config/mcp.json`、`~/.claude/mcp.json` 或 `%APPDATA%\Claude\mcp.json` 等路径。对于用户范围的服务器，运行 `claude mcp add --scope user`，它会写入 `~/.claude.json` 中的 `mcpServers` 密钥；对于项目范围的服务器，编辑项目根目录中的 `.mcp.json`。
> * 您在 `.mcp.json` 中写入了格式错误的条目。Claude Code 跳过该条目并仍然加载其他条目。从 shell 运行 `claude mcp list` 并查找解析警告，该警告会命名有问题的字段。

> [accordion] 状态显示连接失败或连接错误
>
> 两种状态都意味着服务器没有启动或 URL 没有响应。对于拒绝您在 `headers.Authorization` 中配置的令牌的 HTTP 服务器，它们也可能出现；需要您尚未配置的令牌的服务器会显示 `! Needs authentication`，在[连接需要登录的服务器](#connect-a-server-that-requires-sign-in)中介绍。
>
> 您的第一步取决于您看到的状态：
>
> * `Failed to connect`：从状态本身的失败详情开始。`claude mcp list` 和 `claude mcp get <name>` 显示 HTTP 状态或错误代码以及服务器返回的任何错误文本，这通常直接命名问题，例如缺少的标头或被拒绝的令牌。在 v2.1.219 之前，`Failed to connect` 仅显示裸状态，您需要本节后面的 curl 和命令检查来找到原因。
> * `Connection error`：Claude Code 在任何版本上都不会向此状态附加任何详情，因此直接转到本节后面的 curl 和命令检查。
>
> 如果详情指向凭据或 URL，还要检查 `claude mcp list` 输出中的警告。Claude Code 标记具有隐藏的前导或尾随空格的配置值，这是粘贴令牌后身份验证失败的常见原因。
>
> 如果 HTTP 服务器返回 `404 Not Found`，当您在 `/mcp` 中选择服务器时，Claude Code 显示 `MCP endpoint not found at <origin>. Check the URL in your MCP config.`。该消息命名 URL 的源，例如 `https://mcp.example.com`，不包括其路径，因此运行 `claude mcp get <name>` 以查看您配置的完整 URL。将其路径与服务器的文档化 MCP 端点路径进行比较，然后运行 `claude mcp remove <name>` 并使用正确的 URL 重新添加。在 v2.1.219 之前，该消息还包括 URL 的路径，在 v2.1.191 之前，`404` 显示通用的 `Error POSTing to endpoint` 消息，不包含 URL。
>
> 对于 HTTP 服务器，确认 URL 可从您的机器访问：
>
> ```bash theme={null}
> curl -I https://mcp.sentry.dev/mcp
> ```
>
> 在 PowerShell 中，使用 `curl.exe` 而不是 `curl`，以便请求转到真实的 curl 二进制文件而不是 `Invoke-WebRequest` 别名。
>
> 响应告诉您您有什么样的问题：
>
> * `404` 或 `405`：服务器已启动。许多 MCP 端点仅回答 POST 请求，因此这仍然确认 URL 可从您的机器访问。
> * `401` 或 `403`：服务器已启动，您需要进行身份验证。使用[连接需要登录的服务器](#connect-a-server-that-requires-sign-in)中的浏览器登录，或对于接受令牌的服务器（如 GitHub 的），在 `claude mcp add` 命令上使用 `--header "Authorization: Bearer <token>"` 传递它。
> * 完全没有响应：检查 URL 和您的网络。
>
> 对于 stdio 服务器，直接在终端中运行配置的命令以查看基础错误。对于本指南中的 Playwright 服务器，运行：
>
> ```bash theme={null}
> npx -y @playwright/mcp@latest
> ```
>
> 接下来发生的事情告诉您问题在哪里：
>
> * 命令启动并等待输入：服务器本身工作。
>
>   运行 `claude mcp get <name>` 并确认那里显示的命令与您刚刚运行的命令匹配。如果显示的命令与您键入的不同，您可能在服务器命令之前省略了 `--` 分隔符。删除服务器并使用 `--` 重新添加它。如果您手工编写了 `.mcp.json`，请检查其语法和位置。在 v2.1.285 之前，`claude mcp get` 对于保存时没有 `type` 字段的 stdio 条目（例如手工编写的 `.mcp.json` 条目）不打印 `Command` 行。在这些版本上，运行 `claude mcp list` 代替，它无论如何都会打印命令行。
> * 命令出错：消息命名缺少的内容，例如 Node.js 或浏览器。

> [accordion] 连接在启动时超时
>
> 服务器花费的时间超过了默认的 30 秒启动超时。stdio 服务器的第一次运行在 `npx` 下载包时可能很慢。使用 [`MCP_TIMEOUT`](https://code.claude.com/docs/zh-CN/env-vars) 环境变量（以毫秒为单位）增加限制：
>
> ```bash theme={null}
> MCP_TIMEOUT=60000 claude
> ```
>
> 在 PowerShell 中，在同一行上的命令之前设置变量：
>
> ```powershell theme={null}
> $env:MCP_TIMEOUT = "60000"; claude
> ```
>
> 在 v2.1.299 之前，某些到已启动的 HTTP 服务器的连接会在启动时超时，与 `MCP_TIMEOUT` 无关。如果您使用的是这些版本之一，并且在提高限制后已启动的 HTTP 服务器仍然超时，请[更新 Claude Code](https://code.claude.com/docs/zh-CN/setup.md#update-claude-code)。

> [accordion] 服务器已存在
>
> 您已在同一范围内添加了具有该名称的服务器。要么先删除现有条目，要么选择不同的名称：
>
> ```bash theme={null}
> claude mcp remove claude-code-docs
> ```
>
> 如果名称存在于多个范围内，`remove` 报告 `exists in multiple scopes`。传递 `--scope` 以选择要删除的副本，例如 `claude mcp remove claude-code-docs --scope local`。

> [accordion] 服务器连接但没有工具出现
>
> 在会话内运行 `/mcp` 并选择服务器以查看其工具列表。如果列表为空，服务器启动但没有注册任何工具，这通常意味着它缺少必需的环境变量，例如 API 密钥。
>
> 在 `claude mcp add` 上使用 `--env KEY=value` 传递变量，或在服务器的 `.mcp.json` 条目的 `env` 字段中传递。服务器的文档列出了它需要的变量。
>
> 在 v2.1.302 之前，对于使用[协议修订版 2026-07-28](https://code.claude.com/docs/zh-CN/mcp.md#mcp-client-runtimes) 且在其响应中省略了 `resultType`、`ttlMs` 或 `cacheScope` 的服务器，`/mcp` 会显示状态 `tools fetch failed`。如果您使用的是这些版本之一，并且服务器显示该状态，请[更新 Claude Code](https://code.claude.com/docs/zh-CN/setup.md#update-claude-code)。

> [accordion] .mcp.json 的更改不生效
>
> Claude Code 在会话启动时读取 `.mcp.json`。编辑文件后退出并重新启动会话。
>
> 如果您的服务器仍然没有出现，运行 `claude mcp list` 并查找解析警告。Claude Code 跳过格式错误的条目并在那里显示有问题的字段。
>
> 如果您之前在提示时拒绝了服务器，请重置项目批准：
>
> ```bash theme={null}
> claude mcp reset-project-choices
> ```

> [accordion] OAuth 登录失败或浏览器不打开
>
> 运行 `/mcp`，选择服务器，然后再次选择 `Authenticate`。如果浏览器没有自动打开，复制终端中显示的 URL 并手动打开它。有关固定回调端口和预配置凭据，请参阅[使用远程 MCP 服务器进行身份验证](https://code.claude.com/docs/zh-CN/mcp.md#authenticate-with-remote-mcp-servers)。

## 后续步骤

连接一个服务器后，探索 MCP 启用的其余功能：

* [查找更多 MCP 服务器](https://code.claude.com/docs/zh-CN/mcp.md#find-and-build-mcp-servers) 在 Anthropic 目录中
* [与您的团队共享服务器](https://code.claude.com/docs/zh-CN/mcp.md#mcp-installation-scopes) 使用安装范围
* [为组织管理 MCP 访问](https://code.claude.com/docs/zh-CN/managed-mcp.md) 使用托管设置和策略控制
* [在提示中引用 MCP 资源](https://code.claude.com/docs/zh-CN/mcp.md#use-mcp-resources) 使用 @ 提及
* [将 MCP 提示作为命令运行](https://code.claude.com/docs/zh-CN/mcp.md#use-mcp-prompts-as-commands) 从 `/` 菜单
* [构建您自己的服务器](https://modelcontextprotocol.io/quickstart/server) 使用 MCP SDK
