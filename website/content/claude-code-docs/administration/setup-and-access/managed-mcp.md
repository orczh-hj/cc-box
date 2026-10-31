> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# 控制组织的 MCP 服务器访问权限

> 使用托管配置文件、托管设置、允许列表和拒绝列表，限制用户可以添加或连接的 MCP 服务器，或为每个用户提供服务器。

默认情况下，任何运行 Claude Code 的人都可以连接他们选择的任何 [MCP 服务器](https://code.claude.com/docs/zh-CN/mcp.md)。Anthropic 在将连接器添加到 [Anthropic 目录](https://claude.ai/directory)之前会根据其[列表标准](https://claude.com/docs/connectors/building/review-criteria)审查连接器，但不会对任何 MCP 服务器进行安全审计或管理。作为管理员，您可以限制在组织中运行的服务器，从部署固定的批准集到完全禁用 MCP，并且您可以为每个用户提供服务器。

这些限制涵盖 Claude Code 自身加载的服务器，包括它从 claude.ai 获取的连接器。桌面应用程序向其本地和 SSH 会话提供的连接器以进程内方式到达，并由您的 claude.ai 组织设置进行管理；[连接器如何到达 Claude Code](https://code.claude.com/docs/zh-CN/mcp.md#how-connectors-reach-claude-code) 显示了哪些控制适用于每种会话类型中的连接器，包括云会话。

本页面涵盖以下内容：

* [选择一个模式](#choose-a-pattern)，该模式与您需要的控制程度相匹配
* [使用 `managed-mcp.json` 部署固定服务器集](#exclusive-control-with-managed-mcp-json)，包括如何[完全禁用 MCP](#disable-mcp-entirely)
* [通过托管设置提供服务器](#provide-servers-through-managed-settings)，同时用户保留他们自己的服务器
* [使用允许列表和拒绝列表控制服务器](#policy-based-control-with-allowlists-and-denylists)
* [告诉用户当限制阻止服务器时会发生什么](#how-restrictions-appear-to-users)
* [监控您的组织实际使用的服务器](#monitor-mcp-usage)

> [!NOTE]
>
> [安全](https://code.claude.com/docs/zh-CN/security.md)页面涵盖 MCP 威胁模型以及如何在批准服务器之前对其进行评估。[决定要强制执行的内容](https://code.claude.com/docs/zh-CN/admin-setup.md#decide-what-to-enforce)涵盖 MCP 限制以及其他管理控制。

## 选择一个模式

Claude Code 支持一系列限制级别。每个模式使用以下一个或多个机制：用于部署固定集合的 `managed-mcp.json`、用于提供服务器以及用户添加的服务器的 `managedMcpServers` 托管设置，以及用于过滤用户配置内容的 `allowedMcpServers`/`deniedMcpServers`。

| 模式 | 功能 | 配置 |
| :- | :- | :- |
| **禁用 MCP** | 不加载任何服务器，除了[在独占控制下加载](#exclusive-control-with-managed-mcp-json)的少数几个 | 使用空服务器映射的 `managed-mcp.json` |
| **固定部署** | 每个用户获得相同的服务器，无法添加其他服务器 | 包含你想要的服务器的 `managed-mcp.json` |
| **提供的服务器** | 每个用户获得你列出的远程服务器，并保留他们自己的服务器 | 托管设置中的 `managedMcpServers` |
| **批准的目录** | 发布批准的服务器列表；用户添加他们想要的服务器，其他任何内容都被阻止 | `allowedMcpServers` + `allowManagedMcpServersOnly: true` |
| **仅插件服务器** | 用户无法通过 `~/.claude.json` 或 `.mcp.json` 添加服务器；[技能目录插件](https://code.claude.com/docs/zh-CN/plugins/loading.md#skills-directory-plugins-blocked-by-managed-settings)不再作为插件加载，来自其他插件的服务器仍然加载 | [`strictPluginOnlyCustomization`](https://code.claude.com/docs/zh-CN/settings-reference#strictpluginonlycustomization) 列表中包含 `mcp` |
| **软允许列表** | 强制执行允许列表，用户可以在他们自己的设置中扩展 | 不带 `allowManagedMcpServersOnly` 的 `allowedMcpServers` |
| **仅拒绝列表** | 阻止已知的坏服务器，允许其他所有服务器 | `deniedMcpServers` |
| **无限制** | 用户添加任何内容 | 不部署任何托管 MCP 配置 |

> [!NOTE]
>
> Claude Code 没有内置的 MCP 服务器注册表供用户浏览和安装。对于批准的目录模式，在用户会找到的地方（例如内部 wiki）共享批准的列表及其 `claude mcp add` 命令，或通过[托管插件市场](https://code.claude.com/docs/zh-CN/plugins/org.md#restrict-what-users-can-install)将服务器作为插件分发，以便用户可以从 `/plugin` 浏览和安装它们。

## 使用 managed-mcp.json 进行独占控制

当你部署 `managed-mcp.json` 文件时，Claude Code 仅加载这些 MCP 服务器：

* 该文件定义的服务器
* 你[通过 `managedMcpServers` 提供的服务器](#provide-servers-through-managed-settings)
* 启动会话的应用程序注册的进程内服务器，例如 VS Code 扩展自己的服务器或[桌面应用程序提供的连接器](https://code.claude.com/docs/zh-CN/mcp.md#how-connectors-reach-claude-code)
* 内置的[Chrome 中的 Claude](https://code.claude.com/docs/zh-CN/chrome.md) 服务器，如果你[允许它与托管集合一起使用](#allow-claude-in-chrome-alongside-the-managed-set)

用户无法添加、修改或使用任何其他 MCP 服务器，包括插件提供的服务器和通过 [`--mcp-config` CLI 标志](https://code.claude.com/docs/zh-CN/cli-reference#cli-flags)传递的服务器。该文件还会禁止 Claude Code 自身获取的 claude.ai 连接器，除非你[允许它们与托管集合一起使用](#allow-claude-ai-connectors-alongside-the-managed-set)。

### 部署 managed-mcp.json

`managed-mcp.json` 是一个独立文件，因此无法通过[服务器管理的设置](https://code.claude.com/docs/zh-CN/server-managed-settings.md)交付。要通过托管设置交付服务器而不进行独占控制，请使用 [`managedMcpServers`](#provide-servers-through-managed-settings)。

任何可以以管理员权限写入系统路径的进程都可以部署该文件。在整个机队中，这通常通过设备管理工具完成，例如 macOS 上的 Jamf 或配置文件、Windows 上的组策略或 Intune，或 Linux 上你选择的机队管理工具。Claude Code 在以下路径之一查找该文件：

| 平台 | 路径 |
| :- | :- |
| macOS | `/Library/Application Support/ClaudeCode/managed-mcp.json` |
| Linux 和 WSL | `/etc/claude-code/managed-mcp.json` |
| Windows | `C:\Program Files\ClaudeCode\managed-mcp.json` |

该文件使用与项目 [`.mcp.json`](https://code.claude.com/docs/zh-CN/mcp#project-scope) 文件相同的格式：

```json theme={null}
{
  "mcpServers": {
    "github": {
      "type": "http",
      "url": "https://api.githubcopilot.com/mcp/"
    },
    "sentry": {
      "type": "http",
      "url": "https://mcp.sentry.dev/mcp"
    },
    "company-internal": {
      "type": "stdio",
      "command": "/usr/local/bin/company-mcp-server",
      "args": ["--config", "/etc/company/mcp-config.json"],
      "env": {
        "COMPANY_API_URL": "https://internal.example.com"
      }
    }
  }
}
```

### 使用按用户凭证进行身份验证

机器上的任何用户都可以读取此文件，因此不要在 `env` 块中存储 API 密钥或其他凭证。改为使用以下方式之一传递按用户凭证：

* [`${VAR}` 扩展](https://code.claude.com/docs/zh-CN/mcp#environment-variable-expansion-in-mcp-json)从每个用户的环境中读取机密。
* [OAuth 或按用户标头](https://code.claude.com/docs/zh-CN/mcp.md#authenticate-with-remote-mcp-servers)使每个用户以自己的身份进行身份验证。
* [`headersHelper`](https://code.claude.com/docs/zh-CN/mcp#use-dynamic-headers-for-custom-authentication)在连接时生成凭证。

### 通过 `--mcp-config` 或 `--strict-mcp-config` 传递的服务器

当会话通过 `--mcp-config` 接收服务器，同时部署了 Claude Code 可以读取和解析的 `managed-mcp.json` 时，用户看到的内容在工作站和云会话之间有所不同：

* 在工作站上，Claude Code 在启动时退出，显示 `You cannot dynamically configure MCP servers when an enterprise MCP config is present`。
* 在[云会话](https://code.claude.com/docs/zh-CN/claude-code-on-the-web.md)中，在部署了该文件的主机上，例如[自托管运行器](https://code.claude.com/docs/zh-CN/self-hosted-environments-configuration.md#mcp-servers)，Claude Code 仅使用托管服务器启动，并跳过 claude.ai 连接器和云主机通过 `--mcp-config` 交付的其他服务器。会话中没有任何内容告诉用户哪些服务器被遗漏了。Claude Code 在其 stderr 上以警告的形式命名它们，自托管运行器在 `debug` 日志级别记录这些警告。

`--strict-mcp-config` 标志要求替换托管集合。如果用户在部署了这样的文件时传递它，Claude Code 在工作站和云会话中都会在启动时退出。

### 允许列表和拒绝列表如何应用于托管集合

拒绝列表可以进一步过滤 `managed-mcp.json` 中的服务器：

* `deniedMcpServers` 也适用于托管服务器，因此与条目匹配的托管服务器将不会加载。
* 用户自己的 `deniedMcpServers` 从他们的设置中合并，因此用户可以为自己阻止托管服务器。

`managed-mcp.json` 中的服务器不需要 `allowedMcpServers` 条目，但有一个例外：当服务器在 `env` 之外使用 [`${VAR}` 扩展](https://code.claude.com/docs/zh-CN/mcp#environment-variable-expansion-in-mcp-json)时，Claude Code 会针对允许列表检查该服务器。在 v2.1.305 之前，它还会检查仅在 `env` 中使用 `${VAR}` 扩展的服务器；在 v2.1.259 之前，只要设置了允许列表，它就会检查每个托管服务器。有关哪些字段触发 `${VAR}` 检查和完整检查顺序，请参阅[如何评估服务器](#how-a-server-is-evaluated)。

如果您使用 `allowedMcpServers` 来防止某些 `managed-mcp.json` 服务器加载，请在用户升级之前为它们添加 `deniedMcpServers` 条目，或为每个组部署单独的 `managed-mcp.json`。除非这些服务器在 `env` 之外使用 `${VAR}` 扩展，否则它们将在每个用户首次启动不检查它们的版本时开始加载，且没有任何提示或通知。

### 验证配置

要确认文件生效，请在托管机器上运行两项检查：

1. `claude mcp list` 仅显示 `managed-mcp.json` 中的服务器，加上你通过 `managedMcpServers` 提供的任何服务器。另外两个结果意味着出现了问题：
   * 如果用户自己的服务器仍然出现，Claude Code 没有读取该文件，因此请检查其路径和父目录的权限。
   * 如果文件的服务器没有出现，且 `MCP config diagnostics` 部分将企业配置标记为解析失败，Claude Code 无法读取或解析该文件。修复该部分命名的错误，然后让用户重新启动 Claude Code。
2. `claude mcp add --transport http test https://example.com/mcp` 失败，显示 `Cannot add MCP server: enterprise MCP configuration is active and has exclusive control over MCP servers`。URL 不需要是真实服务器，因为策略检查在联系任何内容之前拒绝该命令。

### 完全禁用 MCP

部署包含空服务器映射的 `managed-mcp.json` 以阻止除[在独占控制下加载](#exclusive-control-with-managed-mcp-json)的服务器之外的每个 MCP 服务器：

```json theme={null}
{
  "mcpServers": {}
}
```

`claude mcp add` 失败，显示上述企业策略错误。用户之前配置的服务器在下次启动会话时停止加载，没有警告说明策略是原因。你通过 `managedMcpServers` 提供的服务器以及你允许与托管集合一起使用的任何其他内容仍然在空映射下加载，因此保持这些键未设置以完全关闭 MCP。

### 允许 claude.ai 连接器与托管集合一起使用

默认情况下，部署 `managed-mcp.json` 会禁止 Claude Code 自身获取的 [claude.ai 连接器](https://code.claude.com/docs/zh-CN/mcp.md#use-mcp-servers-from-claude-ai)，包括管理员在 claude.ai 管理控制台中为组织配置的连接器。要将这些连接器与 `managed-mcp.json` 中的服务器一起加载，请在[托管设置源](https://code.claude.com/docs/zh-CN/admin-setup.md#decide-how-settings-reach-devices)中设置 `"allowAllClaudeAiMcps": true`。

启用该设置后，Claude Code 加载与未部署 `managed-mcp.json` 时相同的 claude.ai 连接器。[允许列表和拒绝列表](#policy-based-control-with-allowlists-and-denylists)仍然适用于这些连接器，因此你可以使用 `deniedMcpServers` 阻止特定的连接器。该设置仅影响 Claude Code 自身获取的 claude.ai 连接器；插件提供的服务器保持禁止。

云会话和桌面应用程序的本地和 SSH 会话以另一种方式接收连接器，如[连接器如何到达 Claude Code](https://code.claude.com/docs/zh-CN/mcp.md#how-connectors-reach-claude-code) 中所述。运行云会话的主机上的 `managed-mcp.json`，例如[自托管运行器主机](https://code.claude.com/docs/zh-CN/self-hosted-environments-configuration.md#mcp-servers)，无论你是否设置 `allowAllClaudeAiMcps`，都会禁止该会话的连接器。没有 `managed-mcp.json` 到达桌面应用程序交付给其本地和 SSH 会话的连接器。

Claude Code 仅从管理员控制的策略层读取 `allowAllClaudeAiMcps`：服务器管理的设置、MDM 部署的 plist 或 HKLM 注册表项，或系统 `managed-settings.json` 文件。将其放在用户或项目设置中无效，因此用户无法重新启用独占控制禁止的连接器。

### 允许 Chrome 中的 Claude 与托管集合一起使用

默认情况下，当您部署 `managed-mcp.json` 时，Claude Code 在终端会话中阻止内置的[Chrome 中的 Claude](https://code.claude.com/docs/zh-CN/chrome.md) 服务器。用户不会获得[扩展安装提示](https://code.claude.com/docs/zh-CN/chrome.md#install-the-extension-when-claude-asks)，而用户[默认启用 Chrome](https://code.claude.com/docs/zh-CN/chrome.md#enable-chrome-by-default) 的会话启动时不包含 Chrome 且不打印警告。如果原本可以运行 Chrome 中的 Claude 的用户启动 `claude --chrome`，Claude Code 会在启动时退出，并显示一条指明 `allowClaudeInChromeWithManagedMcp` 设置的错误。

要让用户在托管集合之外同时运行 Chrome 中的 Claude，请在设备自己的托管设置中设置 `"allowClaudeInChromeWithManagedMcp": true`。将其放在 MDM 部署的 plist、HKLM 注册表项或系统 `managed-settings.json` 文件中，以 Claude Code 在该设备上[选择](https://code.claude.com/docs/zh-CN/managed-settings.md#precedence-within-the-managed-tier)的那一个为准。需要 Claude Code v2.1.282 或更高版本。Claude Code 仅从这些设备源读取该设置，即使[服务器管理的设置](https://code.claude.com/docs/zh-CN/server-managed-settings.md)交付您的其余策略也是如此。即使启用了该设置，针对 `claude-in-chrome` 的 [`deniedMcpServers`](#policy-based-control-with-allowlists-and-denylists) 条目仍然会阻止该服务器。

## 通过托管设置提供服务器

要为每个用户提供一组远程 MCP 服务器而不独占 MCP 的控制权，请在[托管设置源](https://code.claude.com/docs/zh-CN/admin-setup.md#decide-how-settings-reach-devices)中的 `managedMcpServers` 下列出它们：服务器托管设置、[Claude 应用网关](https://code.claude.com/docs/zh-CN/claude-apps-gateway-config.md#what-goes-in-cli)策略、MDM 配置文件或注册表策略，或 `managed-settings.json`。用户保留他们自己添加的服务器，并额外接收您的服务器。需要 Claude Code v2.1.259 或更高版本。早期客户端会忽略此密钥。

该值是一个以服务器名称为键的对象。每个条目的形状与项目 [`.mcp.json`](https://code.claude.com/docs/zh-CN/mcp#project-scope) 文件中的 HTTP 或 SSE 服务器相同，包括[使用远程 MCP 服务器进行身份验证](https://code.claude.com/docs/zh-CN/mcp.md#authenticate-with-remote-mcp-servers)中描述的可选 `headers` 和 `oauth` 成员。此示例提供了一个搜索服务器，每个用户使用 OAuth 登录，以及一个记录服务器，它发送您的组织颁发的标头：

```json theme={null}
{
  "managedMcpServers": {
    "search": {
      "type": "http",
      "url": "https://search.example.com/mcp"
    },
    "records": {
      "type": "http",
      "url": "https://records.example.com/mcp",
      "headers": {
        "X-Records-Key": "key-issued-for-all-claude-code-users"
      }
    }
  }
}
```

任何能够读取机器上托管设置的人（包括用户）都可以读取您在此处设置的标头值。使用为整个受众颁发的凭证，或省略 `headers` 并让每个用户使用 OAuth 登录。

### 条目可以包含的内容

Claude Code 仅在通过以下每项检查时才加载条目。它会丢弃失败的条目，记录您可以使用 `/status` 读取的通知，并仍然加载其他条目：

* `type` 是 `http` 或 `sse`。与 `.mcp.json` 中一样，`streamable-http` 被接受为 `http` 的别名。
* `url` 是 `https://` URL。Claude Code 拒绝纯 `http://` URL，包括指向 `localhost` 的 URL。
* 该条目没有 `command`、`args`、`env` 或 `headersHelper` 成员，因此托管设置文档永远不会命名要在用户机器上运行的程序。
* 没有值包含 `${VAR}` 引用。Claude Code 不会在这些条目中展开环境变量，因此请写入字面值。
* 服务器名称仅包含字母、数字、连字符和下划线，没有键或值包含控制或不可见的格式字符。

Claude Desktop 有一个同名的托管设置，其值是不同条目形状的数组，因此不要将一个复制到另一个。Claude Code 不接受数组形式，而是记录通知而不是加载它。

Claude 应用网关在启动时运行相同的检查；请参阅[策略中的 MCP 服务器](https://code.claude.com/docs/zh-CN/claude-apps-gateway-config.md#mcp-servers-in-a-policy)。

### 提供的服务器如何加载

这些规则决定当提供的服务器与另一个服务器定义或此页面上的另一个设置重叠时会加载什么：

* 提供的服务器优先于本地、项目或用户范围内同名的服务器，以及指向相同 URL 的插件服务器或 claude.ai 连接器。
* 如果您还部署 `managed-mcp.json`，Claude Code 会加载其服务器和提供的服务器，当两者都定义一个名称时，该文件的条目优先。
* 当 [`strictPluginOnlyCustomization`](https://code.claude.com/docs/zh-CN/settings-reference#strictpluginonlycustomization) 锁定 `mcp` 表面时，提供的服务器继续加载。
* `deniedMcpServers` 适用于提供的服务器，包括来自用户自己设置的条目，因此用户可以为自己阻止一个。提供的服务器不需要 `allowedMcpServers` 条目。

当您还没有部署 `managed-mcp.json` 时，每次运行的标志保持其含义：

* 用户使用同一名称下的 `--mcp-config` 传递的服务器替换该运行的提供的服务器，并针对 `allowedMcpServers` 进行检查。
* `--strict-mcp-config` 将提供的服务器与所有其他配置的服务器一起排除。

部署 `managed-mcp.json` 后，两个标志的行为如[使用 managed-mcp.json 的独占控制](#exclusive-control-with-managed-mcp-json)所述。

### 用户可以看到和更改的内容

用户无法编辑或删除提供的服务器：

* `claude mcp remove` 报告服务器由组织提供。
* 当您还没有部署 `managed-mcp.json` 时，用户在同一名称下添加的条目会被保存但在您的条目存在时不会被使用。
* 用户仍然可以在 [`/mcp`](https://code.claude.com/docs/zh-CN/mcp#disable-a-server-without-removing-it) 中为自己关闭提供的服务器，该服务器在**托管 MCPs** 下列出提供的服务器。

`claude mcp get` 和 `/mcp` 将提供的服务器的 URL 仅显示为其主机，例如 `https://mcp.example.com/…`，`claude mcp get` 显示其标头名称而不显示其值。

### `managedMcpServers` 适用的位置

Claude Code 从它在[Claude Code 如何组合托管源](https://code.claude.com/docs/zh-CN/managed-settings.md#how-claude-code-combines-managed-sources)下选择的托管源读取 `managedMcpServers`。当该源将 [`managedSourcesBehavior`](https://code.claude.com/docs/zh-CN/settings-reference#managedsourcesbehavior) 设置为 `"merge"` 时，Claude Code 改为提供来自每个管理员源的服务器，当两个源定义相同的名称时，排名较高的源的条目整体适用。它永远不会从用户可写的 HKCU 注册表、从[嵌入主机提供的父设置](https://code.claude.com/docs/zh-CN/managed-settings.md#parent-settings-from-embedding-hosts)或从用户、项目或本地设置文件中读取密钥，它会在那里使用警告丢弃密钥。

Claude Code 不会在第三方部署中的 Claude Desktop 应用的 Code 选项卡中或在应用的 Cowork 会话中读取密钥，因为 Claude Desktop 自己提供并锁定这些会话的 MCP 服务器。当您的托管设置在那里携带密钥时，`/status` 和 `claude doctor` 会说明这一点。

### 提供的服务器何时连接

当 `managedMcpServers` 通过服务器托管设置到达时，其时序遵循[获取和缓存行为](https://code.claude.com/docs/zh-CN/server-managed-settings.md#fetch-and-caching-behavior)：

* 在具有缓存设置的机器上，Claude Code 会保留此密钥的缓存副本，直到服务器确认会话的设置，并等待该确认后再加载 MCP 服务器。如果确认失败，会话将继续而不使用提供的服务器，`/status` 会说它们被保留。
* 在机器的首次启动时，还没有缓存任何内容，在设置到达之前启动的交互式会话会在它们到达时立即连接提供的服务器，而已经启动的 `claude -p` 运行可以在没有它们的情况下完成。

使用[网关登录](https://code.claude.com/docs/zh-CN/claude-apps-gateway-config.md#precedence-with-other-managed-sources)，Claude Code 在会话启动前加载策略，因此两种情况都不会延迟或跳过提供的服务器。

已在运行的交互式会话应用您对密钥的编辑：

* **添加服务器**：Claude Code 在更新的设置到达时连接它，无需重启。
* **更改服务器的条目**：这些会话使用新定义重新连接到它。
* **删除服务器**：运行中的交互式会话在读取更改的设置后断开连接。非交互式（`-p`）运行会保留它直到结束。

## 使用允许列表和拒绝列表进行基于策略的控制

允许列表和拒绝列表过滤哪些已配置的服务器可以加载。它们不是注册表：服务器仍然必须由用户、插件或您的组织添加，然后任一列表才能应用于它。

您的组织通过 `managedMcpServers` 提供的服务器无需允许列表条目即可加载，[跳过允许列表检查的服务器](#servers-that-skip-the-allowlist-check)涵盖 `managed-mcp.json` 服务器。拒绝列表适用于每个服务器，无论它来自何处，除了进程内 `type: "sdk"` 条目。

要将服务器部署给用户，请使用 [`managed-mcp.json`](#exclusive-control-with-managed-mcp-json) 或 [`managedMcpServers`](#provide-servers-through-managed-settings)。两个列表也过滤通过 [`--mcp-config` CLI 标志](https://code.claude.com/docs/zh-CN/cli-reference#cli-flags)传递的服务器，除了进程内 `type: "sdk"` 条目；`--strict-mcp-config` 限制哪些配置文件加载，不会绕过任一列表。

要使允许列表具有权威性，请在[托管设置源](https://code.claude.com/docs/zh-CN/admin-setup.md#decide-how-settings-reach-devices)（如服务器托管设置或已部署的 `managed-settings.json` 文件）中同时设置 `allowedMcpServers` 和 `allowManagedMcpServersOnly: true`。

该锁定从每个管理员控制的托管源应用，因此已部署文件中的锁定在同时使用不提及 MCP 的服务器托管设置时仍然适用。当锁定打开时，托管允许列表来自设置该列表的最高排名管理员源。跨源读取锁定和允许列表需要 Claude Code v2.1.273 或更高版本。

[将允许列表限制为仅托管设置](#restrict-the-allowlist-to-managed-settings-only)显示配置。

如果没有 `allowManagedMcpServersOnly`，来自每个设置范围的允许列表会合并，包括用户自己的 `~/.claude/settings.json`，因此用户可以扩展您的允许列表允许的内容。拒绝列表无论如何都会从每个范围合并。

> [!NOTE]
>
> `allowManagedMcpServersOnly` 与 `allowManagedPermissionRulesOnly` 分开，后者锁定[权限规则](https://code.claude.com/docs/zh-CN/permissions.md#managed-settings)。设置该标志不会强制执行 MCP 允许列表。

### 按 URL、命令或名称匹配服务器

`allowedMcpServers` 和 `deniedMcpServers` 是条目列表。每个条目是一个对象，具有单个键，用于按 URL、命令或名称标识服务器：

| 键 | 匹配 | 用于 |
| :- | :- | :- |
| `serverUrl` | 远程服务器 URL，精确或带有 `*` 通配符 | HTTP 和 SSE 服务器 |
| `serverCommand` | 启动 stdio 服务器的确切命令和参数 | Stdio 服务器 |
| `serverName` | 用户分配的标签。仅精确匹配；通配符不展开 | 任一类型，但请参阅[`serverName` 条目如何匹配](#how-servername-entries-match) |

将 `allowedMcpServers` 保留未设置与将其设置为空数组不同：

| 设置 | 未设置（默认） | 空数组 `[]` | 已填充 |
| :- | :- | :- | :- |
| `allowedMcpServers` | 允许所有服务器 | 不允许任何服务器，除了[那些跳过允许列表检查的](#servers-that-skip-the-allowlist-check) | 仅允许匹配的服务器，除了[那些跳过允许列表检查的](#servers-that-skip-the-allowlist-check) |
| `deniedMcpServers` | 不阻止任何服务器 | 不阻止任何服务器 | 阻止匹配的服务器 |

有关条目未通过架构验证时会发生什么，请参阅[托管设置中的无效条目](https://code.claude.com/docs/zh-CN/managed-settings.md#invalid-entries-in-managed-settings)。

#### `serverName` 条目如何匹配

`serverName` 条目精确匹配用户分配的标签，不支持通配符。

> [!WARNING]
>
> 任一列表中的 `serverName` 条目不是安全控制。该名称是用户在运行 `claude mcp add` 或编辑配置文件时分配的标签，而不是底层服务器，因此用户可以将任何服务器称为 `github`。对于 claude.ai 连接器，名称是 claude.ai 返回的显示名称，可能会更改。要强制执行实际运行的服务器，请添加 `serverCommand` 或 `serverUrl` 条目。

`serverName` 验证在两个列表之间有所不同：

* 在 `deniedMcpServers` 中，`serverName` 接受任何非空字符串，因此您可以按显示名称阻止 [claude.ai 连接器](https://code.claude.com/docs/zh-CN/mcp.md#use-mcp-servers-from-claude-ai)。例如，`{ "serverName": "claude.ai Slack" }` 阻止 Slack 连接器。当您需要拒绝对重命名具有鲁棒性时，或当连接器名称冲突并获得 ` (N)` 后缀时，更倾向于使用 `serverUrl` 条目。
* 在 `allowedMcpServers` 中，`serverName` 仅限于字母、数字、连字符和下划线。使用 `serverUrl` 将 Claude Code 自身获取的 claude.ai 连接器列入允许列表；对于云主机提供给自托管会话的连接器，请改用[连接器流量离开您的网络](https://code.claude.com/docs/zh-CN/self-hosted-environments-deploy.md#connector-traffic-leaves-your-network)下列出的条目。

要关闭 Claude Code 自身获取的所有 claude.ai 连接器，请参阅 [`disableClaudeAiConnectors`](https://code.claude.com/docs/zh-CN/mcp#disable-claude-ai-connectors)。

#### `serverCommand` 条目如何匹配

`serverCommand` 条目将命令及其参数保存为一个数组，如 `{ "serverCommand": ["npx", "-y", "server"] }`。Claude Code 将该数组与服务器配置中的命令和参数进行比较：

* **命令精确匹配。** 每个参数，按顺序。`["npx", "-y", "server"]` 不匹配 `["npx", "server"]` 或 `["npx", "-y", "server", "--flag"]`。
* **不比较 `env` 块。** `["node", "server.js"]` 匹配以任何 `env` 值运行该命令的服务器。某些环境变量会改变 `node` 在启动时加载的内容。要自行设置 `env` 值，请在 [`managed-mcp.json`](#exclusive-control-with-managed-mcp-json) 中定义该服务器。

#### `serverUrl` 条目如何匹配

URL 支持 `*` 通配符，包括以 `*` 作为整个方案。主机名匹配不区分大小写，忽略尾部 FQDN 点，因此 `https://Mcp.Example.com/*` 匹配 `https://mcp.example.com/api`。路径保持区分大小写。如果未指定端口，主机名的写法决定该模式仅匹配方案的默认端口还是所有端口：

* **完整写出的主机名**：仅默认端口，`https` 为 443，`http` 为 80
* **包含 `*` 的主机名**：所有端口

下表显示常见模式允许的内容：

| 模式 | 允许 |
| :- | :- |
| `https://mcp.example.com/*` | 特定域上的所有路径，仅限端口 443 |
| `https://mcp.example.com` | 也允许该域上的所有路径，仅限端口 443。没有路径的模式匹配任何路径 |
| `https://mcp.example.com:8443/*` | 该域上的所有路径，仅限端口 8443 |
| `https://mcp.example.com:*/*` | 该域上任何端口（包括 443）的所有路径 |
| `https://*.example.com/*` | `example.com` 的任何子域，任何端口 |
| `http://localhost:*/*` | localhost 上的任何端口 |
| `*://mcp.example.com/*` | 到特定域的任何方案，每个方案仅限其默认端口 |

`deniedMcpServers` 中的条目以相同方式匹配端口，因此请根据需要阻止的端口和方案为 `staging.example.com` 选择条目：

* `https://staging.example.com/*`：仅阻止该主机上端口 443 的 `https` 服务器，因此不会阻止位于 `https://staging.example.com:8443/api` 的服务器
* `https://staging.example.com:*/*`：阻止该主机上所有端口的 `https` 服务器
* `*://staging.example.com:*/*`：阻止该主机的任何方案和任何端口

#### `serverCommand` 和 `serverUrl` 条目中的环境变量

`serverCommand` 和 `serverUrl` 值在匹配前展开。策略条目和服务器的配置值都通过 [`${VAR}` 和 `${VAR:-default}` 展开](https://code.claude.com/docs/zh-CN/mcp#environment-variable-expansion-in-mcp-json)，因此写成 `["${HOME}/bin/server"]` 的条目与使用相同引用或展开路径的服务器配置匹配。`serverName` 值按字面匹配，永不展开。

两边读取不同的环境：

* **服务器的配置值**：从实时进程环境展开，就像 `.mcp.json` 的其余部分一样
* **策略条目**：从固定环境展开，因此由项目或用户设置文件设置的变量无法更改允许列表条目的含义

因为策略条目仍然取决于启动 shell 对其引用的任何变量的值，对于您依赖的条目以进行强制执行，请使用字面 URL 和命令。

在 Windows 上，引用在那里设置的环境变量，例如 `${USERPROFILE}` 而不是 `${HOME}`。

两个列表的展开方式不同：

| 条目列表 | 展开自 | 会改变 URL 条目的方案、主机或路径范围的展开 |
| - | - | - |
| `allowedMcpServers` | Claude Code 启动时的环境，加上来自托管设置的 `env` 值 | Claude Code 忽略该条目 |
| `deniedMcpServers` | 相同，以及没有启动值且没有 `:-default` 的变量从存储库外的设置文件（如用户或托管设置）填充，这只会扩大条目匹配的内容 | 该条目仍然匹配 |

固定环境和此表中的规则需要 Claude Code v2.1.219 或更高版本。

### 服务器如何被评估

在加载服务器之前，包括来自 `managed-mcp.json` 的服务器，Claude Code 按顺序运行以下三个检查。当用户重新连接服务器或在 `/mcp` 中打开已禁用的服务器时，它会再次运行它们。进程内 `type: "sdk"` 服务器（[启动会话的应用程序注册](https://code.claude.com/docs/zh-CN/mcp.md#how-connectors-reach-claude-code)）跳过全部三个。

1. **合并列表。** 来自每个设置作用域的允许列表和拒绝列表条目合并为一个允许列表和一个拒绝列表。当 `allowManagedMcpServersOnly` 为 `true` 时，仅保留托管允许列表；拒绝列表始终从每个作用域合并。当存在多个托管源时，[从每个管理员源读取的键](https://code.claude.com/docs/zh-CN/managed-settings.md#keys-read-from-every-admin-source)说明其中哪些提供托管作用域的列表。
2. **检查拒绝列表。** 与任何拒绝列表条目匹配的服务器（按 URL、命令或名称）被阻止。没有任何东西可以覆盖拒绝列表匹配。
3. **检查允许列表。** [某些服务器跳过此检查](#servers-that-skip-the-allowlist-check)。如果 `allowedMcpServers` 未在任何地方设置，每个通过拒绝列表的服务器都会加载。如果已设置，服务器必须匹配的内容取决于其类型，如下表所示。

| 服务器类型 | 匹配时允许 |
| :- | :- |
| 远程（HTTP 或 SSE） | 一个 `serverUrl` 条目。仅当允许列表不包含 `serverUrl` 条目时，`serverName` 匹配才计数 |
| Stdio | 一个 `serverCommand` 条目。仅当允许列表不包含 `serverCommand` 条目时，`serverName` 匹配才计数 |

#### 跳过允许列表检查的服务器

除了跳过[全部三个检查](#how-a-server-is-evaluated)的进程内 `type: "sdk"` 服务器之外，还有三组服务器跳过允许列表检查：

* 组织自己的服务器：每个 `managedMcpServers` 条目，以及任何其值在 `env` 之外不使用 `${VAR}` 展开的 `managed-mcp.json` 条目。
* 内置服务器，例如 Chrome 中的 Claude、Claude Code 在运行的 VS Code 或 JetBrains IDE 中连接的 `ide` 服务器，以及 CLI 本身配置的服务器。
* [Claude Tag](https://code.claude.com/docs/zh-CN/claude-tag.md) 会话的 Slack 工具：它用来读取线程和发布回复的服务器无需允许列表条目即可加载。

在其命令、参数、URL 或标头中使用 `${VAR}` 展开的 `managed-mcp.json` 服务器仍会被检查。Claude Code 还会检查用户、插件或 claude.ai 添加的每个服务器，以及用户通过 `--mcp-config` 传递的每个服务器。有关哪些版本改变了 `managed-mcp.json` 服务器的检查范围，以及如何阻止某个服务器加载，请参阅[允许列表和拒绝列表如何应用于托管集](#how-allowlists-and-denylists-apply-to-the-managed-set)。

### 示例配置

下面的配置设置了一个硬允许列表和一个拒绝列表。突出显示的行改变了列表其余部分的评估方式，块后的标注解释了每一行：

```json {3,5,11} theme={null}
{
  "allowedMcpServers": [
    { "serverUrl": "https://api.githubcopilot.com/*" },
    { "serverUrl": "https://mcp.sentry.dev/*" },
    { "serverCommand": ["npx", "-y", "@modelcontextprotocol/server-filesystem", "."] },
    { "serverCommand": ["python", "/usr/local/bin/approved-server.py"] },
    { "serverUrl": "https://mcp.example.com/*" },
    { "serverUrl": "https://*.internal.example.com/*" }
  ],
  "deniedMcpServers": [
    { "serverName": "dangerous-server" },
    { "serverCommand": ["npx", "-y", "unapproved-package"] },
    { "serverUrl": "https://*.untrusted.example.com/*" }
  ]
}
```

* **第 3 行**：第一个 `serverUrl` 条目。一旦存在，每个远程服务器必须匹配 URL 模式，因此用户无法通过给它一个允许的名称来获得未列出的远程服务器。
* **第 5 行**：第一个 `serverCommand` 条目。对 stdio 服务器的效果相同，因此每个本地服务器必须精确匹配列出的命令。
* **第 11 行**：拒绝列表中的 `serverName` 条目。拒绝列表条目始终适用，因此任何名为 `dangerous-server` 的服务器都被阻止，无论其 URL 或命令如何。

此允许列表中的 `serverName` 条目永远不会匹配任何内容，因为两种传输类型都已有更严格的条目。

下面的手风琴演示了如何针对其他允许列表和拒绝列表组合评估服务器。

> [accordion] 仅 URL 允许列表
>
> ```json theme={null}
> {
>   "allowedMcpServers": [
>     { "serverUrl": "https://mcp.example.com/*" },
>     { "serverUrl": "https://*.internal.example.com/*" }
>   ]
> }
> ```
>
> | 服务器 | 结果 |
> | :- | :- |
> | `https://mcp.example.com/api` 处的 HTTP 服务器 | 允许：匹配 URL 模式 |
> | `https://api.internal.example.com/mcp` 处的 HTTP 服务器 | 允许：匹配通配符子域 |
> | `https://external.example.com/mcp` 处的 HTTP 服务器 | 阻止：不匹配任何 URL 模式 |
> | 具有任何命令的 Stdio 服务器 | 阻止：没有名称或命令条目可匹配 |

> [accordion] 仅命令允许列表
>
> ```json theme={null}
> {
>   "allowedMcpServers": [
>     { "serverCommand": ["npx", "-y", "approved-package"] }
>   ]
> }
> ```
>
> | 服务器 | 结果 |
> | :- | :- |
> | 具有 `["npx", "-y", "approved-package"]` 的 Stdio 服务器 | 允许：匹配命令 |
> | 具有 `["node", "server.js"]` 的 Stdio 服务器 | 阻止：不匹配命令 |
> | 名为 `my-api` 的 HTTP 服务器 | 阻止：没有名称条目可匹配 |

> [accordion] 混合名称和命令允许列表
>
> ```json theme={null}
> {
>   "allowedMcpServers": [
>     { "serverName": "github" },
>     { "serverCommand": ["npx", "-y", "approved-package"] }
>   ]
> }
> ```
>
> | 服务器 | 结果 |
> | :- | :- |
> | 名为 `local-tool` 的 Stdio 服务器，具有 `["npx", "-y", "approved-package"]` | 允许：匹配命令 |
> | 名为 `local-tool` 的 Stdio 服务器，具有 `["node", "server.js"]` | 阻止：命令条目存在但不匹配 |
> | 名为 `github` 的 Stdio 服务器，具有 `["node", "server.js"]` | 阻止：stdio 服务器在存在命令条目时必须匹配命令 |
> | 名为 `github` 的 HTTP 服务器 | 允许：匹配名称 |
> | 名为 `other-api` 的 HTTP 服务器 | 阻止：名称不匹配 |

> [accordion] 仅名称允许列表
>
> ```json theme={null}
> {
>   "allowedMcpServers": [
>     { "serverName": "github" },
>     { "serverName": "internal-tool" }
>   ]
> }
> ```
>
> | 服务器 | 结果 |
> | :- | :- |
> | 名为 `github` 的 Stdio 服务器，具有任何命令 | 允许：无命令限制 |
> | 名为 `internal-tool` 的 Stdio 服务器，具有任何命令 | 允许：无命令限制 |
> | 名为 `github` 的 HTTP 服务器 | 允许：匹配名称 |
> | 任何名为 `other` 的服务器 | 阻止：名称不匹配 |

> [accordion] 带拒绝列表覆盖的允许列表
>
> ```json theme={null}
> {
>   "allowedMcpServers": [
>     { "serverUrl": "https://*.example.com/*" }
>   ],
>   "deniedMcpServers": [
>     { "serverUrl": "https://staging.example.com/*" }
>   ]
> }
> ```
>
> | 服务器 | 结果 |
> | :- | :- |
> | `https://mcp.example.com/api` 处的 HTTP 服务器 | 允许：匹配允许列表 URL 模式，无拒绝列表匹配 |
> | `https://staging.example.com/api` 处的 HTTP 服务器 | 阻止：两者都匹配，但拒绝列表优先 |
> | `https://staging.example.com:8443/api` 处的 HTTP 服务器 | 允许：匹配允许列表 URL 模式，[此端口上无拒绝列表匹配](#how-serverurl-entries-match) |
> | `https://other.com/mcp` 处的 HTTP 服务器 | 阻止：不匹配允许列表 |

### 将允许列表限制为仅托管设置

要使托管允许列表成为唯一适用的列表，请在托管设置文件中设置 `allowManagedMcpServersOnly`：

```json theme={null}
{
  "allowManagedMcpServersOnly": true,
  "allowedMcpServers": [
    { "serverUrl": "https://api.githubcopilot.com/*" },
    { "serverUrl": "https://*.internal.example.com/*" }
  ]
}
```

当 `allowManagedMcpServersOnly` 为 `true` 时，来自用户、项目和本地设置的允许列表被忽略。拒绝列表仍然从每个设置范围合并，因此用户总是可以为自己阻止服务器。

## 限制如何向用户呈现

关于部署了 `managed-mcp.json` 且会话中同时存在 `--mcp-config` 服务器时用户在启动时看到的内容，请参阅[使用 managed-mcp.json 进行独占控制](#exclusive-control-with-managed-mcp-json)。使用下表识别其他报告，并在推出变更之前告知用户预期会看到什么：

| 限制 | 用户看到的内容 |
| :- | :- |
| 存在 `managed-mcp.json`，且用户运行 `claude mcp add` | `Cannot add MCP server: enterprise MCP configuration is active and has exclusive control over MCP servers` |
| 存在 `managed-mcp.json`，且原本可以运行 Claude in Chrome 的用户运行 `claude --chrome` | Claude Code 在启动时退出，并显示 `Claude in Chrome is blocked by your organization's managed MCP configuration (managed-mcp.json). An administrator can allow it with allowClaudeInChromeWithManagedMcp in device policy.` |
| 服务器位于拒绝列表中，且用户运行 `claude mcp add` | `Cannot add MCP server "<name>": server is explicitly blocked by enterprise policy` |
| 服务器不在允许列表中，且用户运行 `claude mcp add` | `Cannot add MCP server "<name>": not allowed by enterprise policy` |
| [`strictPluginOnlyCustomization`](https://code.claude.com/docs/zh-CN/settings-reference#strictpluginonlycustomization) 为 `true` 或包含 `mcp`，且用户运行 `claude mcp add` | [`Cannot add MCP server: your organization's managed settings allow only MCP servers that plugins provide`](https://code.claude.com/docs/zh-CN/errors#cannot-add-mcp-server-when-managed-settings-allow-only-plugin-servers) |
| 用户对来自 `managedMcpServers` 的服务器运行 `claude mcp remove` | `MCP server "<name>" is provided by your organization (managed settings) and cannot be removed locally.` |
| 先前已配置的服务器现在被策略阻止 | 该服务器从 `/mcp` 和 `claude mcp list` 中消失 |
| 服务器在会话运行期间被阻止，且用户在 `/mcp` 中选择 **Reconnect** 或将其重新启用 | [`MCP server <name> is blocked by enterprise managed policy`](https://code.claude.com/docs/zh-CN/errors#mcp-server-is-blocked-by-enterprise-managed-policy) |

当服务器静默消失时，用户不会收到任何提示表明原因是策略，因此在推出新限制时，请告知受影响的用户哪些服务器被阻止。

## 监控 MCP 使用情况

当[配置 OpenTelemetry 导出](https://code.claude.com/docs/zh-CN/monitoring-usage.md)时，Claude Code 可以记录用户调用的 MCP 服务器和工具。设置 `OTEL_LOG_TOOL_DETAILS=1` 以在工具事件和[成本和令牌计数器](https://code.claude.com/docs/zh-CN/monitoring-usage.md#cost-counter)中包含 MCP 服务器和工具名称，然后在您的收集器中聚合它们以查看用户实际连接的服务器。请参阅[监控](https://code.claude.com/docs/zh-CN/monitoring-usage.md)以设置导出器和完整的事件架构。

## 配置摘要

本页面涵盖的每个文件和设置、它控制的内容以及如何交付它：

| 表面 | 控制的内容 | 位置 | 如何交付 |
| :- | :- | :- | :- |
| `managed-mcp.json` | 固定服务器集，独占控制 | 系统路径：`/Library/Application Support/ClaudeCode/`、`/etc/claude-code/` 或 `C:\Program Files\ClaudeCode\` | MDM、GPO、舰队管理或任何具有管理员权限的进程。无法通过服务器管理的设置设置 |
| `managedMcpServers` | 提供给每个用户的远程服务器，与他们自己的服务器一起 | 仅托管设置源；该设置在其他地方无效 | 一个[托管设置源](https://code.claude.com/docs/zh-CN/admin-setup.md#decide-how-settings-reach-devices)：服务器管理的设置、网关策略、`managed-settings.json`、MDM 配置文件或 HKLM 注册表 |
| `allowedMcpServers` | 允许的服务器允许列表 | 任何[设置范围](https://code.claude.com/docs/zh-CN/settings.md#where-settings-live)；[服务器如何被评估](#how-a-server-is-evaluated)说明来自多个范围和托管源的列表如何组合 | 为了强制执行，一个[托管设置源](https://code.claude.com/docs/zh-CN/admin-setup.md#decide-how-settings-reach-devices)：服务器管理的设置、`managed-settings.json`、MDM 配置文件或注册表 |
| `deniedMcpServers` | 被阻止的服务器拒绝列表 | 任何设置范围；[服务器如何被评估](#how-a-server-is-evaluated)说明来自多个范围和托管源的列表如何组合 | 与 `allowedMcpServers` 相同 |
| `allowManagedMcpServersOnly` | 将允许列表锁定为仅托管源 | 仅托管设置源；[从每个管理源读取的密钥](https://code.claude.com/docs/zh-CN/managed-settings.md#keys-read-from-every-admin-source)说明哪些托管源可以打开它。该设置在其他范围中无效 | 与 `allowedMcpServers` 相同 |
| `allowClaudeInChromeWithManagedMcp` | 让内置的 Chrome 中的 Claude 服务器与 `managed-mcp.json` 一起运行 | 设备上的托管设置：MDM 配置文件、HKLM 注册表或 `managed-settings.json`。服务器管理的设置和用户可写源无效 | MDM、GPO、舰队管理或任何具有管理员权限的进程 |
| `allowAllClaudeAiMcps` | 加载 claude.ai 连接器，Claude Code 自身与 `managed-mcp.json` 一起获取。[在运行云会话的主机上的 `managed-mcp.json` 仍然会抑制该会话的连接器](#allow-claude-ai-connectors-alongside-the-managed-set) | 仅托管设置源；该设置在其他地方无效 | 与 `allowedMcpServers` 相同 |

## 相关资源

* [决定要强制执行的内容](https://code.claude.com/docs/zh-CN/admin-setup.md#decide-what-to-enforce)：MCP 限制以及权限规则、沙箱和其他管理控制
* [通过 MCP 将 Claude Code 连接到工具](https://code.claude.com/docs/zh-CN/mcp.md)：完整的 MCP 参考，包括传输、范围和身份验证
* [设置](https://code.claude.com/docs/zh-CN/settings.md)：设置层次结构以及托管设置如何优先
* [服务器管理的设置](https://code.claude.com/docs/zh-CN/server-managed-settings.md)：从 claude.ai 管理控制台交付 `allowedMcpServers` 和 `deniedMcpServers`
* [安全](https://code.claude.com/docs/zh-CN/security.md)：这些控制防御的威胁模型
* [Claude 企业管理员指南](https://claude.com/resources/tutorials/claude-enterprise-administrator-guide)：SSO、SCIM、座位管理和推出剧本
