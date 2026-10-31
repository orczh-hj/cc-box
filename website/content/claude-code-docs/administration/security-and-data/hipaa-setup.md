> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# 为符合 HIPAA 要求的组织设置 Claude Code（本地模式）

> 为开发人员的计算机做好准备，以便在 HIPAA 配置下运行 Claude Code（本地模式）。涵盖版本、网络访问、托管设置和本地数据。

HIPAA 配置是 Claude Enterprise 计划中的一项组织设置，适用于处理受保护健康信息（PHI）并与 Anthropic 签订了[业务伙伴协议（BAA）](https://support.claude.com/en/articles/8114513-business-associate-agreements-baa-for-commercial-customers)的组织。它适用于 Claude Code（本地模式）和 Cowork（本地模式），并会限制这两款产品中的功能。

> [!NOTE]
>
> "（本地模式）"指的是本地会话，而不是[云端会话](https://code.claude.com/docs/zh-CN/claude-code-on-the-web.md)。本地会话在以下位置之一运行：
>
> * 终端中的 Claude Code
> * Claude Desktop 的 Code 标签页中的 Claude Code
> * Claude Desktop 中的 Cowork
>
> 适用于 VS Code 和 JetBrains 的 Claude Code 扩展不属于（本地模式）。应用 HIPAA 配置后，这些扩展仍可继续使用，但您的 BAA 不涵盖它们。有关合格服务的完整列表，请参阅[实施指南](https://trust.anthropic.com/resources?s=l1wrssd9hsbi4gak0tp5a6\&name=%5Banthropic%5D-hipaa-ready-offering-implementation-guide)。

本页面面向负责为开发人员准备计算机的 IT 或安全管理员。配置本身由您的 Claude 组织的主要所有者（Primary Owner）应用。[在符合 HIPAA 要求的 Enterprise 计划中使用 Claude Code（本地模式）和 Cowork（本地模式）](https://support.claude.com/en/articles/17318731)说明了您的 BAA 包含的内容、配置的应用方式，以及如何安排应用配置的日期。

如果您组织的成员也使用 Cowork，请同时按照[为符合 HIPAA 要求的组织设置 Cowork（本地模式）](https://claude.com/docs/cowork/hipaa-setup)进行操作。该页面涵盖 Claude Desktop 策略和 Cowork 的本地数据。

下表列出了设置的各个部分应在何时完成：

| 时间 | 操作内容 |
| :- | :- |
| 应用配置之前 | [准备计算机](#prepare-computers-before-the-hipaa-configuration-is-applied)：检查开发人员的连接方式、更新应用、允许网络访问、部署托管设置，并限制 Claude Desktop 登录 |
| 应用配置之后 | Code 标签页处于关闭状态，直到所有者将其重新打开。[在计算机上确认配置](#confirm-the-configuration-on-a-computer) |
| 持续进行 | [管理本地会话数据](#manage-local-session-data) |

## 在应用 HIPAA 配置之前准备计算机

我们建议您从本节中的任务开始，并在应用配置之前完成这些任务。

### 检查开发人员的登录和连接方式

只有当开发人员使用 Claude Enterprise 账户登录且 Claude Code 直接连接到 Claude API 时，HIPAA 配置才会在会话中生效。通过其他任何连接方式，开发人员仍可继续使用 Claude Code，但不会应用 [HIPAA 配置](#what-developers-see-in-claude-code)。

下表显示了哪些连接方式符合条件。要了解您的 BAA 是否涵盖"否"行中的会话，请参阅[在符合 HIPAA 要求的 Enterprise 计划中使用 Claude Code（本地模式）和 Cowork（本地模式）](https://support.claude.com/en/articles/17318731)。

| Claude Code 的连接方式 | 是否符合 HIPAA 配置的条件 |
| :- | :- |
| Claude Enterprise 账户，直接连接到 Claude API | 是 |
| Amazon Bedrock、Google Cloud 的 Agent Platform、Microsoft Foundry、Claude Platform on AWS 或 [Claude apps 网关](https://code.claude.com/docs/zh-CN/claude-apps-gateway) | 否 |
| [LLM 网关](https://code.claude.com/docs/zh-CN/llm-gateway)或任何其他自定义 `ANTHROPIC_BASE_URL` | 否 |
| 在未登录 Claude Enterprise 的计算机上使用 `ANTHROPIC_AUTH_TOKEN` 或 `apiKeyHelper` | 否 |
| Claude Console API 密钥或[联合凭据](https://code.claude.com/docs/zh-CN/authentication.md#anthropic-profiles-and-federation-credentials) | 否。这些会话属于 Claude Console 组织，该组织有自己的协议和设置 |

#### 检查计算机的连接方式

在计算机上打开终端，运行 `claude`，然后在输入框中输入 `/status`。**Status** 标签页会显示以下各行：

| 行 | 出现时机 |
| :- | :- |
| `Login method` 和 `Organization` | 会话使用 claude.ai 账户登录。对于 Claude Enterprise 账户，`Login method` 显示为 `Claude Enterprise account`，`Organization` 显示您的组织 |
| `API provider` | 仅当会话使用云提供商或 Claude apps 网关时 |
| `Anthropic base URL` | 仅当设置了 `ANTHROPIC_BASE_URL` 时 |

如果计算机使用的连接方式不符合 HIPAA 配置的条件，您可以使用[托管设置](#deploy-managed-settings)来阻止云提供商、网关以及通过 `ANTHROPIC_API_KEY`、`ANTHROPIC_AUTH_TOKEN` 或 `apiKeyHelper` 设置的凭据。

### 更新 Claude Code 和 Claude Desktop

HIPAA 配置要求 Claude Code v2.1.285 或更高版本，以及 Claude Desktop v2.19675.0 或更高版本。如果您的组织同时使用终端和 [Claude Desktop 应用](https://code.claude.com/docs/zh-CN/desktop.md)，请同时更新两者。

要查看已安装的 Claude Code 版本，请在终端中运行以下命令。该命令在 Bash、Zsh 和 PowerShell 中相同：

```bash theme={null}
claude --version
```

受支持的安装会输出 `2.1.285 (Claude Code)` 或更高的版本号。

要查看已安装的 Claude Desktop 版本，请参阅[检查您的版本](https://code.claude.com/docs/zh-CN/desktop.md#check-your-version)。

#### 开发人员在旧版本上会看到什么

对于具有 HIPAA 配置的组织，Anthropic 的服务器会拒绝来自低于最低版本的请求。Anthropic 会随时间提高最低版本，您无需进行任何配置。

| 应用 | 开发人员在旧版本上会看到的内容 |
| :- | :- |
| Claude Code | 每个请求都会失败，并显示 [`API Error`](https://code.claude.com/docs/zh-CN/errors#claude-code-does-not-support-this-model)，说明该版本低于您组织的策略所要求的最低版本 |
| Claude Desktop | 一个 **Update required** 对话框，提示开发人员更新 Claude Desktop 才能继续使用 **Code** 标签页 |

要让开发人员保持使用受支持的版本，请[保持 Claude Code 为最新版本](https://code.claude.com/docs/zh-CN/setup.md#update-claude-code)。对于 Claude Desktop，请参阅[更新 Claude Desktop](https://claude.com/docs/cowork/hipaa-setup#update-claude-desktop)。

### 允许网络访问

请通过您的代理和防火墙允许下表中的主机，使用 HTTPS 端口 443。请允许整个主机，而不是单个路径。

| 主机 | 用途 |
| :- | :- |
| `api.anthropic.com` | Claude API 请求、遥测，以及告知 Claude Code HIPAA 配置已开启的组织策略 |
| `claude.ai`、`claude.com`、`platform.claude.com` | 登录和令牌刷新 |
| `sso.claude.com` | 登录（当您的组织使用单点登录时） |
| `downloads.claude.ai` | 原生安装程序及其更新 |
| `mcp-proxy.anthropic.com` | [来自 claude.ai 的连接器](https://code.claude.com/docs/zh-CN/mcp.md#use-mcp-servers-from-claude-ai) |

此表列出了终端中原生安装的 Claude Code 进行登录、运行和更新所需的主机。其余主机列在以下页面中：

* **其他安装方式和可选功能**：[网络访问要求](https://code.claude.com/docs/zh-CN/network-config.md#network-access-requirements)列出了 npm 和 Homebrew 安装检查更新时使用的主机，以及插件安装等功能所需的主机
* **Code 标签页和 Cowork**：[Desktop 网络访问要求](https://code.claude.com/docs/zh-CN/desktop.md#network-access-requirements)列出了 Claude Desktop 所需的其他主机
* **检查 TLS 的代理**：[自定义 CA 证书](https://code.claude.com/docs/zh-CN/network-config.md#custom-ca-certificates)说明了如何信任您代理的证书

只要代理能够访问表中的主机，通过企业 HTTPS 代理的会话仍然符合 HIPAA 配置的条件。

Claude Code 通过从 `api.anthropic.com` 获取您组织的策略来得知您的组织具有 HIPAA 配置，获取时机为启动时，以及会话使用期间大约每小时一次。该策略记录了您组织的 HIPAA 状态及由此产生的功能限制。

要检查某台计算机是否已获取策略，请参阅[在计算机上确认配置](#confirm-the-configuration-on-a-computer)。

### 部署托管设置

您可以使用[托管设置](https://code.claude.com/docs/zh-CN/managed-settings.md)引导开发人员使用 Claude Enterprise 账户登录、阻止云提供商和网关，并设置每台计算机保留本地会话数据的天数。无论 HIPAA 配置是否生效，这些设置都会生效。

本节中的设置是我们推荐作为起点的示例。您的组织有责任确定其自身环境的需求，并确认其配置满足这些需求。

以下示例设置了四个键，您可以将它们添加到组织部署的[托管设置](https://code.claude.com/docs/zh-CN/managed-settings.md#choose-a-delivery-mechanism)中：

```json theme={null}
{
  "forceLoginMethod": "claudeai",
  "forceLoginOrgUUID": "xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx",
  "allowedProviders": ["anthropic"],
  "cleanupPeriodDays": 30
}
```

如需包含沙箱隔离、网络允许列表、凭据保护和本地数据保留的更完整 `managed-settings.json`，请参阅[设置示例仓库](https://github.com/anthropics/claude-code/tree/main/examples/settings)中的 `settings-hipaa.json` 和 `README-hipaa.md`。

#### 每个键的作用

下表显示了每个键应设置的值，以及 Claude Code 对其强制执行的内容。

| 键 | 设置值 | Claude Code 强制执行的内容 |
| :- | :- | :- |
| [`forceLoginMethod`](https://code.claude.com/docs/zh-CN/settings-reference#forceloginmethod) | `"claudeai"` | Claude Code 引导开发人员使用 claude.ai 登录，而不是 Claude Console |
| [`forceLoginOrgUUID`](https://code.claude.com/docs/zh-CN/settings-reference#forceloginorguuid) | 您的组织 ID，[所有者](https://code.claude.com/docs/zh-CN/server-managed-settings.md#access-control)可以从 [claude.ai 管理设置](https://claude.ai/admin-settings/organization)中复制 | 当 claude.ai 登录属于其他组织时，Claude Code 会在启动时退出 |
| [`allowedProviders`](https://code.claude.com/docs/zh-CN/settings-reference#allowedproviders) | `["anthropic"]` | Claude Code 拒绝在云提供商或网关上启动 |
| [`cleanupPeriodDays`](https://code.claude.com/docs/zh-CN/settings-reference#cleanupperioddays) | 您的记录策略允许计算机保留会话数据的天数 | 每台计算机在相同天数后删除旧的会话数据 |

> [!WARNING]
>
> 部署前请检查 `forceLoginOrgUUID` 的值。如果它与您的组织 ID 不匹配，则每个使用 claude.ai 账户登录的开发人员在启动 Claude Code 时都会退出。

HIPAA 配置不限制 `cleanupPeriodDays`，因此开发人员可以在自己的设置中提高该值。当您在托管设置中设置它时，Claude Code 会忽略开发人员的值。

设置 `forceLoginMethod` 或 `forceLoginOrgUUID` 后，Claude Code 还会拒绝使用 `ANTHROPIC_API_KEY`、`ANTHROPIC_AUTH_TOKEN` 或 `apiKeyHelper` 进行身份验证的会话。

#### 确认设置已加载

在已具有这些设置的计算机上，运行 `claude`，使用 Claude Enterprise 账户登录，然后输入 `/status`。`Setting sources` 行会列出 `Enterprise managed settings`，后跟括号中的来源，例如 `(file)`，并且 `Allowed providers` 行显示为 `Anthropic API (managed allowedProviders)`。如果 `Setting sources` 中未列出它，或者缺少 `Allowed providers` 行，请参阅[检查策略是否生效](https://code.claude.com/docs/zh-CN/managed-settings.md#check-that-a-policy-is-in-force)。

#### 托管设置键无法阻止的会话

即使部署了这些键，某些会话仍可能在没有 HIPAA 配置的情况下运行：

* **Claude Console 登录和联合凭据**：`forceLoginOrgUUID` 仅检查 claude.ai 登录。[将登录限制为您的组织](https://code.claude.com/docs/zh-CN/authentication.md#restrict-login-to-your-organization)列出了 Claude Code 对每种登录路径和凭据检查的内容。
* **服务器托管设置**：如果您的组织还使用[服务器托管设置](https://code.claude.com/docs/zh-CN/server-managed-settings.md)，请让所有者在其中添加相同的键。[Claude Code 如何合并托管来源](https://code.claude.com/docs/zh-CN/managed-settings.md#how-claude-code-combines-managed-sources)说明了哪个来源生效。
* **低于 v2.1.285 的版本**：这些版本会忽略 `allowedProviders`，因此仍可能在云提供商或网关上启动。要让 v2.1.163 至 v2.1.284 拒绝启动，您可以在与[示例键](#deploy-managed-settings)相同的托管设置中添加值为 `"2.1.285"` 的 [`requiredMinimumVersion`](https://code.claude.com/docs/zh-CN/settings-reference#requiredminimumversion)。v2.1.163 之前的版本既会忽略 `allowedProviders`，也会忽略 `requiredMinimumVersion`，因此请[更新这些计算机](#update-claude-code-and-claude-desktop)。

要了解您的 BAA 是否涵盖在没有 HIPAA 配置的情况下运行的会话，请参阅[在符合 HIPAA 要求的 Enterprise 计划中使用 Claude Code（本地模式）和 Cowork（本地模式）](https://support.claude.com/en/articles/17318731)。

### 限制 Claude Desktop 登录

如果开发人员使用 Claude Desktop 的 Code 标签页，还需按照[将登录限制为您的组织](https://code.claude.com/docs/zh-CN/desktop.md#restrict-sign-in-to-your-organization)中的说明，在 Claude Desktop 策略中设置 `forceLoginOrgUUID`。在托管设置中部署 `forceLoginMethod` 和 `forceLoginOrgUUID` 不会改变 Claude Desktop 的登录界面。

## 在计算机上确认配置

在为您的组织应用配置后，请在一台托管计算机上运行此检查。

**1. 重新启动 Claude Code**

退出所有正在运行的会话，打开终端，然后运行 `claude`。正在使用中的会话无需重新启动，会在大约一小时内获取配置。重新启动时，Claude Code 会立即获取配置。

**2. 检查启动通知**

确认 Claude Code 启动时输出 `Per your organization's policy, some features are limited · /status for details`。

**3. 检查页脚**

确认输入框下方页脚的右侧出现 `HIPAA configured` 标签。在 v2.1.286 之前，该标签显示为 `HIPAA`。

**4. 运行 /status**

在输入框中输入 `/status`。确认 **Status** 标签页的 `Organization configuration` 行中列出了 `HIPAA`。

**5. 检查 Claude Desktop**

应用 HIPAA 配置会为您的组织关闭 Code 标签页。如果您的组织使用它，请让所有者前往 [**Organization settings > Claude Code**](https://claude.ai/admin-settings/claude-code) 并打开 **Desktop** 开关。对于 Cowork，请参阅[在 Claude Desktop 中确认 HIPAA 配置](https://claude.com/docs/cowork/hipaa-setup#confirm-the-hipaa-configuration-in-claude-desktop)。

重新加载 Claude Desktop 或重新登录。确认标题栏显示 **HIPAA configured** 标签。在 Mac 上，需要打开侧边栏才能看到它。

如果 `/status` 中缺少 `HIPAA`，请按顺序检查以下原因：

1. **账户或连接方式错误**：确认 `/status` 在 `Organization` 行显示您的组织，并且未显示 `API provider` 或 `Anthropic base URL` 行。[检查开发人员的登录和连接方式](#check-how-developers-sign-in-and-connect)列出了哪些登录和连接方式不符合配置条件。
2. **策略获取被阻止**：在 `/status` 中查找 `Organization policy` 行，该行会给出原因。在会话之外，运行 `claude doctor` 并查看同一行，该行会说明 Claude Code 从何处加载了策略或策略未加载的原因。请通过您的代理允许 `api.anthropic.com`，然后重新启动 Claude Code。
3. **配置尚未应用**：询问主要所有者是否已应用配置。

## 开发人员在 Claude Code 中会看到什么

应用 HIPAA 配置后，终端中的某些 Claude Code 功能会被关闭或行为有所不同。下表列出了开发人员最有可能向您询问的变化。[HIPAA 功能可用性表](https://support.claude.com/en/articles/8114513-business-associate-agreements-baa-for-commercial-customers)列出了所有 Claude Code 和 Cowork 功能，包括所有者可以重新打开的功能。

| 开发人员注意到的情况 | 原因 |
| :- | :- |
| WebFetch 工具不可用 | WebFetch 已关闭。网页搜索仍然可用 |
| `--cloud`、`/teleport` 和 [Remote Control](https://code.claude.com/docs/zh-CN/remote-control.md) 被拒绝 | [云端会话](https://code.claude.com/docs/zh-CN/claude-code-on-the-web.md)和 Remote Control 已关闭 |
| `/feedback` 和 `/bug` 不可用 | 反馈提交已关闭 |
| Claude 无法发布 [Artifact](https://code.claude.com/docs/zh-CN/artifacts.md) | Artifact 发布已关闭 |
| MCP 工具会预先加载到上下文中 | [MCP 工具搜索](https://code.claude.com/docs/zh-CN/mcp.md#configure-tool-search)已关闭。`ENABLE_TOOL_SEARCH` 和托管设置都无法重新打开它 |
| 读取 `ANTHROPIC_API_KEY` 的 MCP 服务器或 hook 无法再进行身份验证 | Claude Code 会从其启动的进程中[移除 Anthropic 凭据](#anthropic-credentials-in-commands-hooks-and-mcp-servers) |
| 通过 `/login` 切换到其他组织后限制仍然存在 | HIPAA 状态会一直持续到 Claude Code 重新启动 |

### 命令、hook 和 MCP 服务器中的 Anthropic 凭据

应用 HIPAA 配置后，Claude Code 会从其启动的 shell 命令、hook 和 MCP 服务器的环境中移除其用于访问 Anthropic 的凭据，例如 `ANTHROPIC_API_KEY` 和 `ANTHROPIC_AUTH_TOKEN`。

HIPAA 配置不会移除云提供商或 GitHub 凭据，因此推送到 GitHub 或调用其他服务的命令仍然可以使用该开发人员的访问权限正常工作。您与 Anthropic 签订的 BAA 不涵盖发送到这些位置的数据。有关合格服务的完整列表，请参阅[实施指南](https://trust.anthropic.com/resources?s=l1wrssd9hsbi4gak0tp5a6\&name=%5Banthropic%5D-hipaa-ready-offering-implementation-guide)。

要限制 Claude 可以使用的命令和主机，请参阅[权限规则](https://code.claude.com/docs/zh-CN/permissions.md)和[沙箱](https://code.claude.com/docs/zh-CN/sandboxing.md)。

## 管理本地会话数据

Claude Code（本地模式）和 Cowork（本地模式）会在每位开发人员的计算机上存储会话数据。保护和删除这些数据是您组织的责任。

### Claude Code 数据

[应用程序数据](https://code.claude.com/docs/zh-CN/claude-directory.md#application-data)列出了 Claude Code（本地模式）在计算机上存储的内容、其保留清理在 `cleanupPeriodDays` 之后删除的内容，以及在有人删除之前一直保留的内容。该页面还说明了在已应用 HIPAA 配置的组织中有何不同。

保留清理仅在有人启动 Claude Code 时运行，因此无人启动 Claude Code 的计算机会保留其数据。

### Code 标签页数据

Code 标签页将数据存储在以下位置：

* **会话记录**：位于 `~/.claude/projects/`，与终端会话记录存放在一起。[自动清理](https://code.claude.com/docs/zh-CN/claude-directory.md#cleaned-up-automatically)说明了保留清理何时删除它们。
* **Claude Desktop 数据文件夹**：在 macOS 上为 `~/Library/Application Support/Claude`。在 Windows 上为 `%APPDATA%\Claude`，对于从 Anthropic 下载的安装程序则为 `%LOCALAPPDATA%\Packages\Claude_pzs8sxrjxfjjc\LocalCache\Roaming\Claude`，因此请同时检查两者。应用 HIPAA 配置后，Claude Desktop 会删除不活动时间超过 `cleanupPeriodDays` 的本地 Code 标签页会话，包括已加星标的会话。它仅在运行时执行删除。Claude Desktop 会以下列方式之一处理已删除会话的 worktree：
  * **没有未提交的更改、会话未加星标或固定，且没有其他会话正在使用该 worktree**：Claude Desktop 会移除该 worktree
  * **其他任何情况**：worktree 保留在计算机上

在 Windows 上，`~` 表示 `%USERPROFILE%`。

### Cowork 数据

[管理每台计算机上的 Cowork 数据](https://claude.com/docs/cowork/hipaa-setup#manage-cowork-data-on-each-computer)列出了 Cowork（本地模式）存储数据的位置以及 Claude Desktop 删除的内容。

### 立即删除会话数据

如果您的组织需要在保留清理删除之前移除某位开发人员的会话数据，您可以使用一条命令移除其中的大部分数据。以该开发人员的身份登录计算机，打开任意 shell，然后运行与已安装的 Claude Code 版本对应的命令。

在 Claude Code v2.1.288 或更高版本上，运行 `claude purge`：

```bash theme={null}
claude purge --all --yes
```

在 v2.1.126 至 v2.1.287 上，运行 `claude project purge`，它接受相同的标志：

```bash theme={null}
claude project purge --all --yes
```

这两个命令都会删除每个项目的会话记录和自动记忆、`tasks/`、`debug/` 和 `file-history/` 中的条目、`history.jsonl`，以及 `~/.claude.json` 中的项目条目。如果不加 `--yes`，它会先输出计划并进行询问。

清除操作会保留其他可能包含会话内容的路径，例如 `paste-cache/` 中粘贴的文本。[清除本地数据](https://code.claude.com/docs/zh-CN/claude-directory.md#clear-local-data)列出了您可以手动删除的路径。要彻底清理一台计算机（例如在重新分配之前），请[擦除它](#offboard-a-developer)。

### 开发人员离职处理

移除开发人员的席位或账户不会删除其计算机上的任何内容，`/logout` 也不会删除会话数据。要移除所有数据，您可以使用设备管理工具擦除计算机。

## 相关资源

* [为符合 HIPAA 要求的组织设置 Cowork（本地模式）](https://claude.com/docs/cowork/hipaa-setup)
* [部署托管设置](https://code.claude.com/docs/zh-CN/managed-settings.md)
* [HIPAA 设置示例](https://github.com/anthropics/claude-code/tree/main/examples/settings)
* [企业网络配置](https://code.claude.com/docs/zh-CN/network-config.md)
* [零数据保留](https://code.claude.com/docs/zh-CN/zero-data-retention.md)
* [法律与合规](https://code.claude.com/docs/zh-CN/legal-and-compliance.md)
* [数据使用](https://code.claude.com/docs/zh-CN/data-usage.md)
