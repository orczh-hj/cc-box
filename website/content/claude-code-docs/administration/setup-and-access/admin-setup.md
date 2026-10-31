> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# 为您的组织设置 Claude Code

> 针对部署 Claude Code 的管理员的决策地图，涵盖 API 提供商、托管设置、策略执行、使用情况监控和数据处理。

Claude Code 通过托管设置强制执行组织策略，这些设置优先于本地开发人员配置。您可以从 Claude 管理控制台、移动设备管理 (MDM) 系统或磁盘上的文件传递这些设置。这些设置控制 Claude 可以访问的工具、命令、服务器和网络目标。

本页按顺序介绍部署决策。每一行都链接到下面的部分和该区域的参考页面。

> [!NOTE]
>
> SSO、SCIM 预配和座位分配在 Claude 账户级别配置。有关这些步骤，请参阅 [Claude 企业管理员指南](https://claude.com/resources/tutorials/claude-enterprise-administrator-guide) 和 [座位分配](https://support.claude.com/en/articles/11845131-use-claude-code-with-your-team-or-enterprise-plan)。

| 决策 | 您的选择 | 参考 |
| :- | :- | :- |
| [选择您的 API 提供商](#choose-your-api-provider) | Claude Code 的身份验证位置和计费方式 | [Authentication](https://code.claude.com/docs/zh-CN/authentication.md)、[Amazon Bedrock](https://code.claude.com/docs/zh-CN/amazon-bedrock.md)、[Google Cloud's Agent Platform](https://code.claude.com/docs/zh-CN/google-vertex-ai.md)、[Microsoft Foundry](https://code.claude.com/docs/zh-CN/microsoft-foundry.md) |
| [决定设置如何到达设备](#decide-how-settings-reach-devices) | 托管策略如何到达开发人员机器 | [Server-managed settings](https://code.claude.com/docs/zh-CN/server-managed-settings.md)、[Delivery mechanisms](https://code.claude.com/docs/zh-CN/managed-settings.md#delivery-mechanisms) |
| [决定要强制执行的内容](#decide-what-to-enforce) | 允许哪些工具、命令和集成 | [Permissions](https://code.claude.com/docs/zh-CN/permissions.md)、[Sandboxing](https://code.claude.com/docs/zh-CN/sandboxing.md) |
| [设置使用情况可见性](#set-up-usage-visibility) | 如何跟踪支出和采用情况 | [Analytics](https://code.claude.com/docs/zh-CN/analytics.md)、[Monitoring](https://code.claude.com/docs/zh-CN/monitoring-usage.md)、[Costs](https://code.claude.com/docs/zh-CN/costs.md) |
| [审查数据处理](#review-data-handling) | 数据保留和合规性态势 | [Data usage](https://code.claude.com/docs/zh-CN/data-usage.md)、[Security](https://code.claude.com/docs/zh-CN/security.md) |

## 选择您的 API 提供商

Claude Code 通过多个 API 提供商之一连接到 Claude。您的选择会影响计费、身份验证、您继承的合规性态势，以及您的开发人员可以使用的 Claude Code 功能。

| 提供商 | 何时选择 |
| :- | :- |
| Claude for Teams / Enterprise | 您希望 Claude Code 和 claude.ai 在一个按座位订阅下，无需运行基础设施。这是默认建议。 |
| Claude Console | 您是 API 优先或希望按使用量付费 |
| Amazon Bedrock | 您希望继承现有的 AWS 合规控制和计费 |
| Google Cloud's Agent Platform | 您希望继承现有的 GCP 合规控制和计费 |
| Microsoft Foundry | 您希望继承现有的 Azure 合规控制和计费 |

某些 Claude Code 功能需要 claude.ai 账户。[云端会话](https://code.claude.com/docs/zh-CN/claude-code-on-the-web.md)、[云端定时任务](https://code.claude.com/docs/zh-CN/routines.md)、[Code Review](https://code.claude.com/docs/zh-CN/code-review.md)、[Remote Control](https://code.claude.com/docs/zh-CN/remote-control.md) 和 [Chrome 扩展](https://code.claude.com/docs/zh-CN/chrome.md) 不能仅通过 Console API 密钥或云提供商凭据使用。如果您通过 Amazon Bedrock、Google Cloud's Agent Platform 或 Microsoft Foundry 部署，请计划开发人员是否还需要 Claude for Teams 或 Enterprise 座位。每个功能页面都列出了其套餐要求。

有关涵盖身份验证、区域和功能奇偶性的完整提供商比较，请参阅 [enterprise deployment overview](https://code.claude.com/docs/zh-CN/third-party-integrations.md)。每个提供商的身份验证设置在 [Authentication](https://code.claude.com/docs/zh-CN/authentication.md) 中。

[Network configuration](https://code.claude.com/docs/zh-CN/network-config.md) 中的代理和防火墙要求适用于所有提供商。如果您想要在多个提供商前面有单个端点或集中式请求日志记录，请参阅 [LLM gateway](https://code.claude.com/docs/zh-CN/llm-gateway)。

## 决定设置如何到达设备

托管设置定义组织策略。Claude Code 按优先级顺序检查下表中的四个来源。[Claude Code 如何合并托管来源](https://code.claude.com/docs/zh-CN/managed-settings.md#precedence-within-the-managed-tier)说明其中哪些适用、策略助手更改什么，以及如何组合每个来源。该表是决策地图。

| 机制 | 传递 | 优先级 | 平台 |
| :- | :- | :- | :- |
| Server-managed | claude.ai 管理控制台，或用于网关登录的自托管 [Claude apps gateway](https://code.claude.com/docs/zh-CN/claude-apps-gateway) | 最高 | 全部 |
| plist / registry policy | macOS: `com.anthropic.claudecode` plist Windows: `HKLM\SOFTWARE\Policies\ClaudeCode` | 高 | macOS、Windows |
| File-based managed | macOS: `/Library/Application Support/ClaudeCode/managed-settings.json` Linux 和 WSL: `/etc/claude-code/managed-settings.json` Windows: `C:\Program Files\ClaudeCode\managed-settings.json` | 中 | 全部 |
| Windows user registry | `HKCU\SOFTWARE\Policies\ClaudeCode` | 最低 | 仅 Windows |

Claude Code 在启动时获取 server-managed 设置，并在会话期间每小时刷新一次，无需部署端点基础设施。通过 claude.ai 管理控制台传递需要 Claude for Teams 或 Enterprise 计划。在 Amazon Bedrock、Google Cloud 的 Agent Platform 或 Microsoft Foundry 上的部署可以通过运行 [Claude apps gateway](https://code.claude.com/docs/zh-CN/claude-apps-gateway) 获得相同的远程传递，或改用基于文件或操作系统级别的机制之一。

如果您的组织混合使用提供商，请为 claude.ai 用户配置 [server-managed settings](https://code.claude.com/docs/zh-CN/server-managed-settings.md) 加上 [file-based 或 plist/registry 回退](https://code.claude.com/docs/zh-CN/managed-settings.md#delivery-mechanisms)，以便其他用户仍然接收托管策略。

plist 和 HKLM 注册表位置适用于任何提供商，并且由于需要管理员权限才能写入，因此可以抵抗篡改。Windows 用户注册表中的 HKCU 可以在没有提升权限的情况下写入，因此将其视为便利默认值而不是执行通道。

默认情况下，WSL 仅读取 `/etc/claude-code` 处的 Linux 文件路径。要将您的 Windows 注册表和 `C:\Program Files\ClaudeCode` 策略扩展到同一台机器上的 WSL，请在这些仅限管理员的 Windows 来源之一中设置 [`wslInheritsWindowsSettings: true`](https://code.claude.com/docs/zh-CN/settings-reference#wslinheritswindowssettings)。

无论您选择哪种机制，托管值都优先于用户和项目设置，除了一些安全敏感的[例外](https://code.claude.com/docs/zh-CN/settings.md#exceptions-to-managed-settings-precedence)。数组设置（如 `permissions.allow` 和 `permissions.deny`）合并来自所有源的条目，因此开发人员可以扩展托管列表但不能从中删除。对于 `fallbackModel`、`availableModels` 和 [`modelPicker`](https://code.claude.com/docs/zh-CN/settings-reference#modelpicker)，托管值替换较低层而不是合并。

### WSL 会话在 Claude Code Desktop 中

在 Windows 上，[Claude Code Desktop 可以在 WSL 2 发行版内运行 Code 会话](https://code.claude.com/docs/zh-CN/desktop-wsl.md)。会话的 Claude Code 进程在发行版内运行，因此它通过上述 WSL 发现路径解析托管设置：除非部署了 `wslInheritsWindowsSettings: true`，否则仅限 Windows 的来源无法到达它。

Claude Desktop 在检测到设备为组织管理的设备上默认关闭 WSL 会话，例如当 `C:\Program Files\ClaudeCode\managed-settings.json` 存在时。要打开它们，请部署 Windows 注册表策略，这需要 Claude Desktop v1.19367.0 或更高版本：

* 在 `HKLM\SOFTWARE\Policies\Claude` 下创建一个名为 `disableWslSessions` 的值，并将其设置为 `REG_SZ` 字符串 `false` 或 `REG_DWORD` `0`。此值位于 Claude Desktop 策略键下，与携带托管设置的 `ClaudeCode` 键分开。在 HKLM 下部署该值，这需要管理员权限才能写入。HKCU 下的值不会启用 WSL 会话。
* 如果部署了 `C:\Program Files\ClaudeCode\managed-settings.json`，请保留它。一旦 HKLM 下的 `disableWslSessions` 为 `false`，Desktop 即使该文件存在也允许 WSL 会话。

Desktop 在每次 WSL 会话启动时读取策略，因此在部署后无需重启应用。

如果设备仍然拒绝 WSL 会话，请在该设备上的 Claude Desktop 中打开 **Help > Troubleshooting > Show Logs in File Explorer**，这会将其日志文件夹的副本保存到 Downloads。在该副本中搜索 `main.log` 中的 `[wslPolicyGate] denying WSL session`。拒绝的原因在括号中，例如 `(cli-file-present)`。

启用 WSL 会话后，将您的托管设置扩展到它们：

* 通过 HKLM 注册表或 `C:\Program Files\ClaudeCode` 文件部署 `wslInheritsWindowsSettings: true`，以便 WSL 会话继承与主机会话相同的策略。
* 通过在 WSL 会话内运行 `/status` 进行验证，并读取 `Setting sources` 行。要解释它列出的内容，请参阅[在 /status 中读取来源](https://code.claude.com/docs/zh-CN/managed-settings.md#read-the-source-in-/status)。

WSL 2 实用程序 VM 内的进程对 Windows 端端点检测传感器不可见。要观察发行版内的进程和文件活动，请查看您的端点检测供应商的 WSL 指南，了解您可以在发行版内运行的 Linux 传感器及其需要的排除项。Claude Code 的 [OpenTelemetry 工具执行遥测](https://code.claude.com/docs/zh-CN/monitoring-usage.md) 对 WSL 和本机会话的发出方式相同。

## 决定要强制执行的内容

托管设置可以锁定工具、在沙箱中执行、限制 MCP 服务器和插件来源，并控制运行哪些 hook。每一行都是一个控制面，并列出驱动它的设置键。

| 控制项 | 作用 | 关键设置 |
| :- | :- | :- |
| [权限规则](https://code.claude.com/docs/zh-CN/permissions.md) | 允许、询问或拒绝特定的工具和命令 | `permissions.allow`、`permissions.deny` |
| [权限锁定](https://code.claude.com/docs/zh-CN/permissions.md#managed-only-settings) | 使托管设置成为[权限规则的唯一设置来源](https://code.claude.com/docs/zh-CN/settings-reference.md#allowmanagedpermissionrulesonly)。禁用 `--dangerously-skip-permissions` | `allowManagedPermissionRulesOnly`、`permissions.disableBypassPermissionsMode` |
| [起始权限模式](https://code.claude.com/docs/zh-CN/permission-modes.md#which-mode-a-session-starts-in) | 选择开发人员的终端会话启动时所用的权限模式，以替代内置的起始权限模式，或移除自动模式。[切换权限模式](https://code.claude.com/docs/zh-CN/permission-modes.md#switch-permission-modes)列出了 VS Code 扩展何时读取您设置的 `defaultMode` | `permissions.defaultMode`、`permissions.disableAutoMode` |
| [沙箱隔离](https://code.claude.com/docs/zh-CN/sandboxing.md) | 操作系统级别的文件系统和网络隔离，并支持域名允许列表 | `sandbox.enabled`、`sandbox.network.allowedDomains` |
| [托管策略 CLAUDE.md](https://code.claude.com/docs/zh-CN/memory.md#deploy-organization-wide-claude-md) | 面向 Claude 的组织范围行为指导，而非由客户端强制执行的规则 | 位于托管策略路径下的文件 |
| [MCP 服务器控制](https://code.claude.com/docs/zh-CN/managed-mcp.md) | 限制用户可以添加或连接哪些 MCP 服务器、部署一组固定的服务器，或在用户自有服务器之外为每个用户提供远程服务器 | `allowedMcpServers`、`deniedMcpServers`、`allowManagedMcpServersOnly`、`managedMcpServers`，或已部署的 `managed-mcp.json` 文件 |
| [插件市场控制](https://code.claude.com/docs/zh-CN/plugins/org.md#restrict-what-users-can-install) | 限制用户可以添加和从中安装的市场来源，并拒绝在单次运行中旁加载插件、Agent 和 MCP 服务器的 CLI 标志。阻止 [`command` 插件来源](https://code.claude.com/docs/zh-CN/plugins/marketplace-reference#command-plugin-source)，并通过允许列表指定哪些市场的插件可以在终端中被推荐 | `strictKnownMarketplaces`、`blockedMarketplaces`、`disableSideloadFlags`、`disableCommandPluginSources`、`pluginSuggestionMarketplaces` |
| [自定义锁定](https://code.claude.com/docs/zh-CN/settings-reference.md#strictpluginonlycustomization) | 阻止来自用户和项目来源的 skill、Agent、hook 和 MCP 服务器，使它们只能来自插件或托管设置。锁定 skill 还会停止同步[开发人员在 claude.ai 上启用的 skill](https://code.claude.com/docs/zh-CN/skills.md#where-synced-skills-load) | `strictPluginOnlyCustomization` |
| [禁用 claude.ai 同步](https://code.claude.com/docs/zh-CN/settings-reference.md#syncclaudeaiskills) | 阻止 Claude Code 加载开发人员在 claude.ai 上启用的 [skill](https://code.claude.com/docs/zh-CN/skills.md#how-synced-skills-behave) 和[插件](https://code.claude.com/docs/zh-CN/plugins/loading.md#synced-plugins)。如果您在 claude.ai 上为组织关闭 Skills，Claude Code 会停止同步这两者，并且在 v2.1.273 或更高版本中还会移除已同步的内容。若要在不关闭 Skills 的情况下停止其中任意一项，请在托管设置中将其对应的键设置为 `false` | `syncClaudeAiSkills`、`syncClaudeAiPlugins` |
| [Hook 限制](https://code.claude.com/docs/zh-CN/settings-reference.md#allowmanagedhooksonly) | 限制运行哪些 hook 并限制 HTTP hook URL；完整的影响列表请参阅[在 `allowManagedHooksOnly` 下运行的内容](https://code.claude.com/docs/zh-CN/settings-reference#what-runs-under-allowmanagedhooksonly) | `allowManagedHooksOnly`、`allowedHttpHookUrls` |
| [登录强制](https://code.claude.com/docs/zh-CN/settings-reference.md#forceloginmethod) | 将登录限制为特定方式或特定 Anthropic 组织。方式限制适用于 VS Code 扩展、Agent SDK、`claude setup-token` 和 `/install-github-app`，而终端的交互式登录界面（通过 `/login` 或首次运行引导进入）会预先选中该方式，但不强制执行；Claude Code 会在终端、VS Code 扩展和 Agent SDK 中对 claude.ai 账户登录验证组织，但不会对 Claude Console 登录或[网关](https://code.claude.com/docs/zh-CN/claude-apps-gateway)登录检查组织。在 v2.1.212 之前，这两个键仅对终端登录生效。设置后，通过 `ANTHROPIC_API_KEY`、`ANTHROPIC_AUTH_TOKEN` 或 `apiKeyHelper` 进行身份验证的会话会在启动时被阻止；云提供商会话不受影响，除非同时存在上述凭据之一，或存在先前 Claude Console 登录时保存的 API 密钥 | `forceLoginMethod`、`forceLoginOrgUUID` |
| [提供商限制](https://code.claude.com/docs/zh-CN/settings-reference.md#allowedproviders) | 限制一台机器可以使用哪些 API 提供商。使用未列出提供商的会话会在启动时、登录时以及下次联系 API 时被拒绝。需要 Claude Code v2.1.285 或更高版本 | `allowedProviders` |
| [禁用 Agent 视图](https://code.claude.com/docs/zh-CN/agent-view.md#how-background-sessions-are-hosted) | 关闭 `claude agents`、`--bg`、`/background` 和按需启动的监督进程 | `disableAgentView` |
| [配置企业启动器](https://code.claude.com/docs/zh-CN/corporate-launcher.md) | 为[后台 Agent 监督进程](https://code.claude.com/docs/zh-CN/agent-view.md#how-background-sessions-are-hosted)、其工作进程以及[其他涵盖的后台进程](https://code.claude.com/docs/zh-CN/corporate-launcher.md#what-the-launcher-covers)添加所需的企业启动器前缀，而不是关闭 Agent 视图 | `processWrapper` |
| [模型限制](https://code.claude.com/docs/zh-CN/model-config.md#restrict-model-selection) | `availableModels` 筛选选择器中显示哪些模型。添加 `enforceAvailableModels` 还会约束自动选择的默认模型。有关此设置如何作用于 CLI、Web 和 IDE，请参阅[使用入口覆盖范围](https://code.claude.com/docs/zh-CN/model-config.md#surface-coverage) | `availableModels`、`enforceAvailableModels` |
| [Effort 上限](https://code.claude.com/docs/zh-CN/settings-reference.md#maxeffortlevel) | 在所有提供商上为所有模型或按模型限制 [effort 级别](https://code.claude.com/docs/zh-CN/model-config.md#adjust-effort-level)的上限 | `maxEffortLevel` |
| [版本下限](https://code.claude.com/docs/zh-CN/settings-reference.md#minimumversion) | 防止自动更新安装低于组织范围最低版本的版本 | `minimumVersion` |
| [必需版本范围](https://code.claude.com/docs/zh-CN/settings-reference.md#requiredminimumversion) | 当运行的版本超出组织批准的范围时完全拒绝启动。比仅阻止降级的 `minimumVersion` 更严格 | `requiredMinimumVersion`、`requiredMaximumVersion` |
| [选择退出遥测](https://code.claude.com/docs/zh-CN/data-usage.md#telemetry-services) | 在每台设备上关闭发送给 Anthropic 的使用指标、错误报告和调查 | 在 `env` 中将 `CLAUDE_CODE_DISABLE_NONESSENTIAL_TRAFFIC` 设置为 `1`；链接的章节列出了各类别对应的变量 |

如果您的成员通过 claude.ai 或 Anthropic API 登录，并且您使用的是 Claude Enterprise 计划，您还可以在组织的管理设置中管理模型，无需部署任何内容：

* [组织模型限制](https://code.claude.com/docs/zh-CN/model-config.md#organization-model-restrictions)：禁用单个模型。在服务器端强制执行。
* [组织默认模型](https://code.claude.com/docs/zh-CN/model-config.md#organization-default-model)：设置新会话启动时使用的模型。成员仍可切换模型。若要在启动时将其恢复为您的默认模型，请打开该章节中描述的覆盖选项。若要限制他们可以选择的模型，请使用[组织模型限制](https://code.claude.com/docs/zh-CN/model-config.md#organization-model-restrictions)。
* [组织 effort 限制](https://code.claude.com/docs/zh-CN/model-config.md#organization-effort-limits)：按角色限制 effort 级别上限。在服务器端强制执行。

这些控制项均不作用于 Amazon Bedrock、Google Cloud 的 Agent Platform、Microsoft Foundry 或 [Claude Platform on AWS](https://code.claude.com/docs/zh-CN/claude-platform-on-aws.md) 上的会话。在这些提供商上，请改用托管设置：用 `availableModels` 进行限制，用 `model` 设置默认模型，用 [`maxEffortLevel`](https://code.claude.com/docs/zh-CN/settings-reference#maxeffortlevel) 设置 effort 上限。

[云端会话](https://code.claude.com/docs/zh-CN/claude-code-on-the-web.md)在 claude.ai 上有自己的管理入口：

* **云环境页面**：所有者创建[组织共享环境](https://code.claude.com/docs/zh-CN/cloud-environments.md#organization-shared-environments)，为成员的云端会话设置[网络访问级别](https://code.claude.com/docs/zh-CN/cloud-environments.md#network-access)、环境变量和设置脚本。
* **默认环境**：所有者在 [claude.ai/admin-settings/claude-code](https://claude.ai/admin-settings/claude-code) 单独选择组织的默认环境。
* **Git 提供商页面**：有关关联到您组织的 GitHub 账户，请参阅[已连接的 GitHub 账户](#connected-github-accounts)。

权限规则和沙箱隔离覆盖不同的层级。拒绝 WebFetch 会阻止 Claude 的获取工具，但如果允许 Bash，`curl` 和 `wget` 仍然可以访问任何 URL。沙箱隔离通过在操作系统级别强制执行的网络域名允许列表来弥补这一缺口。

有关这些控制项所防御的威胁模型，请参阅[安全](https://code.claude.com/docs/zh-CN/security.md)。

### 已连接的 GitHub 账户

在 Team 和 Enterprise 计划中，[**Organization settings > Git providers**](https://claude.ai/admin-settings/source-control) 的 GitHub 部分列出了通过 [Claude GitHub App](https://github.com/apps/claude) 关联到您 Claude 组织的 GitHub 组织和个人账户。Claude Code、[Claude Tag](https://claude.com/docs/claude-tag/admins/configure-github) 和 Claude Security 共享该列表。打开此页面需要您在 Claude 组织中拥有管理员角色。

管理员或成员都可以关联账户：

* **管理员连接**：管理员在该部分点击 **Connect**，或在已连接账户后点击 **Add organization**，然后在 GitHub 组织上安装 Claude GitHub App。以这种方式关联组织，需要一个既是该 GitHub 组织所有者、又是您 Claude 组织管理员的人员。
* **成员连接**：当成员将其 GitHub 账户连接到 Claude 时（例如在[设置云端会话](https://code.claude.com/docs/zh-CN/web-quickstart.md#connect-github)时），Claude 会关联该成员所拥有且已安装 Claude GitHub App 的 GitHub 账户。这可能包括其个人账户以及其拥有的 GitHub 组织。

标记为 **Not linked** 的行来自您自己的 GitHub 登录。它是您在 GitHub 上可以看到且已安装 Claude GitHub App 的账户。

若要将某个账户与您的 Claude 组织取消关联，请打开该行的菜单并选择 **Unlink from this workspace**。取消关联后，Claude GitHub App 仍会保留在 GitHub 上，并且下次该账户的任一所有者将 GitHub 连接到 Claude 时，该账户会再次被关联。若要防止其再次被关联，请在 GitHub 上从该账户卸载 Claude GitHub App。

在 Enterprise 计划中，用于关联和取消关联的 [Compliance API](https://platform.claude.com/docs/en/api/compliance/activities/list) 活动类型为 `github_app_installation_linked` 和 `github_app_installation_unlinked`。

## 设置使用情况可见性

根据您需要报告的内容选择监控。仪表板、API 和支出控制在 Claude for Teams 或 Enterprise 计划与 Claude Console 组织之间有所不同，因此在围绕某项功能规划报告之前，请检查"可用性"列。

| 功能 | 您获得的内容 | 可用性 | 从何处开始 |
| :- | :- | :- | :- |
| Usage monitoring | 会话、工具和令牌的 OpenTelemetry 导出 | 所有提供商 | [Monitoring usage](https://code.claude.com/docs/zh-CN/monitoring-usage.md) |
| Analytics dashboard | Teams / Enterprise 上具有排行榜的采用和贡献指标；Console 上的每用户使用情况和支出指标 | Teams / Enterprise 在 [claude.ai/analytics](https://claude.ai/analytics/claude-code)，Console 在 [platform.claude.com/claude-code](https://platform.claude.com/claude-code) | [Analytics](https://code.claude.com/docs/zh-CN/analytics.md) |
| Programmatic reporting | 通过 API 的每用户使用情况和成本数据 | Enterprise 的 [Enterprise Analytics API](https://platform.claude.com/docs/en/api/admin/analytics)，Console 的 [Claude Code Analytics API](https://platform.claude.com/docs/en/build-with-claude/claude-code-analytics-api) | [Costs](https://code.claude.com/docs/zh-CN/costs.md#manage-costs-for-your-organization) |
| Spend controls | 支出限制和速率限制 | Teams / Enterprise 的组织设置，Console 的工作区限制；在第三方云上，云预算控制或具有每用户[支出限制](https://code.claude.com/docs/zh-CN/claude-apps-gateway-spend-limits.md)的 [Claude apps gateway](https://code.claude.com/docs/zh-CN/claude-apps-gateway) | [Costs](https://code.claude.com/docs/zh-CN/costs.md#manage-costs-for-your-organization) |

在 Teams 和 Enterprise 上，每用户使用情况和支出数字来自您组织的分析设置中的[支出报告](https://support.claude.com/en/articles/12883420-view-usage-analytics-for-team-and-enterprise-plans)，而不是分析仪表板。云提供商通过 AWS Cost Explorer、GCP Billing 或 Azure Cost Management 公开支出。有关跨 Claude chat、Claude Code 和 Cowork 规划企业预算的信息，请参阅 [Claude Enterprise consumption guide](https://support.claude.com/en/articles/14782391-claude-enterprise-consumption-guide)。

## 审查数据处理

在 Team、Enterprise、Claude API 和云提供商计划上，Anthropic 不会在您的代码或提示上训练模型。您的 API 提供商决定保留和合规性态势。

| 主题 | 需要了解的内容 | 从何处开始 |
| :- | :- | :- |
| Data usage policy | Anthropic 收集的内容、保留多长时间、永远不会用于训练的内容 | [Data usage](https://code.claude.com/docs/zh-CN/data-usage.md) |
| Zero Data Retention (ZDR) | 请求完成后不存储任何内容。适用于 Claude for Enterprise 上符合条件的账户 | [Zero data retention](https://code.claude.com/docs/zh-CN/zero-data-retention.md) |
| HIPAA configuration | 适用于已启用 HIPAA 的 Claude for Enterprise 组织。部分 Claude Code（本地模式）功能会被关闭，其他功能默认关闭 | [Set up Claude Code (local mode) for a HIPAA-ready organization](https://code.claude.com/docs/zh-CN/hipaa-setup.md) |
| Security architecture | 网络模型、加密、身份验证、审计跟踪 | [Security](https://code.claude.com/docs/zh-CN/security.md) |

如果您需要请求级别的审计日志或按数据敏感性路由流量，我们建议您在开发人员和您的提供商之间放置网关：自托管的 [Claude apps gateway](https://code.claude.com/docs/zh-CN/claude-apps-gateway) 记录带有 IdP 身份的每个请求审计日志，或者您也可以使用另一个 [LLM gateway](https://code.claude.com/docs/zh-CN/llm-gateway)。通过网关的会话不符合 HIPAA 配置的条件。[Check how developers sign in and connect](https://code.claude.com/docs/zh-CN/hipaa-setup.md#check-how-developers-sign-in-and-connect) 列出了哪些登录和连接方式符合条件。有关监管要求和认证，请参阅 [Legal and compliance](https://code.claude.com/docs/zh-CN/legal-and-compliance.md)。

## 验证和入职

配置托管设置后，让开发人员在 Claude Code 中运行 `/status`。在 **Status** 选项卡上，`Setting sources` 行显示 `Enterprise managed settings` 后跟括号中的源；[验证执行](https://code.claude.com/docs/zh-CN/managed-settings.md#verify-enforcement) 列出了标签。

分享这些资源以帮助开发人员入门：

* [快速入门](https://code.claude.com/docs/zh-CN/quickstart.md)：从安装到使用项目的首次会话演练
* [常见工作流](https://code.claude.com/docs/zh-CN/common-workflows.md)：代码审查、重构和调试等日常任务的模式
* [Claude Code 101](https://academy.claude.com/courses/claude-code-101) 和 [Claude Code in Action](https://academy.claude.com/courses/claude-code-in-action)：[Claude Academy](https://academy.claude.com/) 上的免费自定进度课程

对于登录问题，请将开发人员指向 [身份验证故障排除](https://code.claude.com/docs/zh-CN/troubleshoot-install.md#login-and-authentication)。最常见的修复是：

* 运行 `/logout` 然后 `/login` 以切换账户
* 如果缺少企业身份验证选项，运行 `claude update`
* 更新后重启终端

如果开发人员看到"您还没有被添加到您的组织"，他们的座位不包括 Claude Code 访问权限，需要在管理控制台中更新。

## 后续步骤

选择提供商和传递机制后，继续进行详细配置：

* [Server-managed settings](https://code.claude.com/docs/zh-CN/server-managed-settings.md)：从 Claude 管理控制台传递托管策略
* [All settings](https://code.claude.com/docs/zh-CN/settings-reference.md)：每个设置键、文件位置和示例
* [Which value Claude Code uses](https://code.claude.com/docs/zh-CN/settings.md#which-value-claude-code-uses)：跨托管、项目、本地和用户设置的优先级规则
* [Manage plugins for your organization](https://code.claude.com/docs/zh-CN/plugins/org.md)：通过托管设置在每台机器上安装共享插件
* [Monorepos and large repos](https://code.claude.com/docs/zh-CN/large-codebases.md)：为部署到 monorepo 的组织提供的按目录配置模式
* [Amazon Bedrock](https://code.claude.com/docs/zh-CN/amazon-bedrock.md)、[Google Cloud's Agent Platform](https://code.claude.com/docs/zh-CN/google-vertex-ai.md)、[Microsoft Foundry](https://code.claude.com/docs/zh-CN/microsoft-foundry.md)：提供商特定部署
* [Claude Enterprise Administrator Guide](https://claude.com/resources/tutorials/claude-enterprise-administrator-guide)：SSO、SCIM、座位管理和推出手册
