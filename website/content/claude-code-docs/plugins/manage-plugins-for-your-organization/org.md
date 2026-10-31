> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# 为您的组织管理 Claude Code plugins

> 通过托管设置控制 Claude Code 在组织中每台机器上安装和允许的 plugins。

托管设置让您决定 Claude Code 在组织中每台机器上安装和允许的 plugins。用户无法覆盖这些设置。您可以从 claude.ai 管理员控制台以[服务器托管设置](https://code.claude.com/docs/zh-CN/server-managed-settings.md)的形式或通过 MDM 或 `managed-settings.json` 文件以端点托管设置的形式提供这些设置。此页面上的大多数控制仅从托管设置生效。

此页面适用于管理员，此处的设置管理 Claude Code。

> [!NOTE]
>
> 这些情况在其他页面上有介绍：
>
> * **为自己安装 plugins**：从[安装 plugins](https://code.claude.com/docs/zh-CN/plugins/install.md)开始
> * **控制成员在 claude.ai 和 Cowork 中可以使用的 plugins**：请参阅 claude.com 上的[为您的组织管理 plugins](https://claude.com/docs/plugins/admin)
> * **将一个 plugin 同时推出到 claude.ai、Cowork 和 Claude Code**：请参阅 claude.com 上的[选择推出路线](https://claude.com/docs/plugins/org-rollout#choose-a-rollout-route)
> * **claude.ai 管理员设置中的 plugins 页面**：[**组织设置 > Plugins & skills**](https://claude.ai/admin-settings/skills?tab=inventory)为成员的 claude.ai 账户启用 plugins，这些会作为[同步的 plugins](https://code.claude.com/docs/zh-CN/plugins/loading.md#synced-plugins)到达 Claude Code。它不设置此页面上的任何键

这些部分遵循大多数推出采取的顺序：为所有人或按存储库[要求 plugins](#pre-install-and-require-plugins)，[为容器和 CI 提供种子](#seed-containers-and-ci)，[限制](#restrict-what-users-can-install)用户可以自己添加的内容，[设置更新策略](#set-update-policy)，然后[审计](#audit-and-review)已安装的内容。要在一个地方查看每个策略键，请参阅[控制矩阵](#control-matrix)。

## 预安装和要求插件

市场是 Claude Code 从 git 存储库、URL 或本地路径获取的插件目录。在机器上注册市场后，Claude Code 可以从中安装插件。

要为整个车队安装插件，请在[托管设置](https://code.claude.com/docs/zh-CN/managed-settings.md)、策略文件或组织中每台机器读取的服务器交付策略中一起设置两个键：`extraKnownMarketplaces` 在每台机器上注册市场，`enabledPlugins` 命名要从中安装和启用的插件。[选择交付机制](#choose-a-delivery-mechanism)涵盖托管设置如何到达每台机器。

### 选择交付机制

托管设置通过以下三种交付机制之一到达机器：

* **服务器托管设置**：在[**组织设置 > Claude Code > 托管设置**](https://claude.ai/admin-settings/claude-code)处将插件键设置为 JSON。需要在您的 Claude 组织中具有[所有者角色](https://code.claude.com/docs/zh-CN/server-managed-settings.md#access-control)。云会话在安装插件之前获取这些设置。
* **MDM 策略**：在 macOS 上，交付一个 plist，其顶级键是设置键。在 Windows 上，将整个 JSON 文档作为字符串存储在注册表值中。plist 域和注册表键在[每个机制存储策略的位置](https://code.claude.com/docs/zh-CN/managed-settings.md#where-each-mechanism-stores-the-policy)中。
* **托管设置文件**：在平台的系统路径处放置 `managed-settings.json`。您也可以将文件添加到其旁边的 `managed-settings.d/` 放入目录。每个平台的文件路径在[每个机制存储策略的位置](https://code.claude.com/docs/zh-CN/managed-settings.md#where-each-mechanism-stores-the-policy)中，放入合并规则在[跨团队拆分基于文件的策略](https://code.claude.com/docs/zh-CN/managed-settings.md#split-a-file-based-policy-across-teams)中。

如果您在 claude.ai 上有 Claude for Teams 或 Enterprise 组织，并且您的设备不都在 MDM 下，请使用服务器托管设置。否则使用 MDM 策略或托管设置文件。有关权衡，请参阅[在服务器托管和端点托管设置之间选择](https://code.claude.com/docs/zh-CN/server-managed-settings.md#choose-between-server-managed-and-endpoint-managed-settings)。

#### 哪个托管源在机器上应用

默认情况下，这三个源中只有一个在机器上应用。Claude Code 使用首先交付策略键的源，首先检查服务器托管设置，然后是 MDM 策略，然后是托管设置文件。如果服务器托管设置交付甚至一个不相关的策略键，Claude Code 会忽略该机器上 MDM 策略或托管设置文件中的插件键，除了[它从每个源读取的键](https://code.claude.com/docs/zh-CN/managed-settings.md#keys-read-from-every-admin-source)。

要改为应用每个源，请将[`managedSourcesBehavior`](https://code.claude.com/docs/zh-CN/managed-settings#compose-every-managed-source)设置为 `"merge"`。

[Claude Code 如何组合托管源](https://code.claude.com/docs/zh-CN/managed-settings.md#how-claude-code-combines-managed-sources)也在两种模式中列出 Claude Code 从每个源读取的键。

### 要求市场及其插件

在 `extraKnownMarketplaces` 下添加市场，使用市场自己的 `marketplace.json` 中的 `name` 作为键。然后在 `enabledPlugins` 下添加每个插件作为 `plugin-name@marketplace-name`。每个市场条目都带有一个 `source` 对象，其中 `source` 字段命名类型，例如 `github`。此托管设置示例注册一个组织市场并强制启用来自它的两个插件：

```json theme={null}
{
  "extraKnownMarketplaces": {
    "your-marketplace": {
      "source": { "source": "github", "repo": "your-org/your-marketplace" },
      "autoUpdate": true
    }
  },
  "enabledPlugins": {
    "code-formatter@your-marketplace": true,
    "deploy-helper@your-marketplace": true
  }
}
```

设置到达机器后，Claude Code 注册市场并在用户下一个会话开始时安装两个插件。用户在 `/plugin` 中看到它们，在自己的范围内禁用一个不会阻止它加载，因为托管设置优先于每个其他范围。

要在每个范围内阻止插件并将其从市场列表中隐藏，请在托管 `enabledPlugins` 中将其设置为 `false`。

为您的市场调整 `autoUpdate` 和 `source` 字段：

* **`autoUpdate`**：`true` 使市场及其插件在后台刷新，`false` 关闭它。请参阅[设置更新策略](#set-update-policy)。
* **`source`**：`github` 是几种源类型之一。`git` 源为 GitLab 或内部主机采用 `url`，`url` 源采用托管 `marketplace.json` 的地址。每个源形状在[市场参考](https://code.claude.com/docs/zh-CN/plugins/marketplace-reference.md)中。

如果市场是私有 git 存储库，每个用户都需要对其有读取权限。git 市场的克隆在用户的机器上使用 git 运行，使用存储的凭证且无提示。对于没有 git 主机账户的用户，请改用[播种](#seed-containers-and-ci)。

托管条目也会覆盖来自另一个源的同名市场条目或 `--plugin-dir` 副本：

* **市场**：托管市场条目替换具有相同名称的较低优先级条目，两个条目的字段不合并。
* **`--plugin-dir` 副本**：`--plugin-dir` 为一个会话从本地目录加载插件。有关当该副本的名称与您的托管 `enabledPlugins` 命名的插件匹配时会发生什么，请参阅[名称冲突](https://code.claude.com/docs/zh-CN/plugins/loading.md#name-conflicts)。

Anthropic 的官方市场 `claude-plugins-official` 当 `enabledPlugins` 将其一个插件设置为 `true` 时不需要 `extraKnownMarketplaces` 条目。该 `name@claude-plugins-official` 条目在这些键应用的任何地方声明市场。如果您不启用其任何插件但仍想在每台机器上注册它，请给它一个显式条目，如[允许官方市场和您自己的](#allow-the-official-marketplace-and-your-own)所做的那样。

### 按存储库要求插件

要覆盖一个存储库的贡献者而不是整个车队，请在该存储库的 `.claude/settings.json` 中设置 `extraKnownMarketplaces` 和 `enabledPlugins`。`extraKnownMarketplaces` 条目仅在贡献者已信任的文件夹中应用，在不受信任的文件夹中 Claude Code 会无声地忽略它们：

* **交互式会话**：Claude Code 仅在贡献者接受该文件夹的[工作区信任对话](https://code.claude.com/docs/zh-CN/permissions.md#what-runs-before-you-trust-a-folder)后注册市场。
* **[非交互式 `-p` 运行](https://code.claude.com/docs/zh-CN/headless)** ：条目仅在贡献者已交互式接受其信任的文件夹中应用，或您在 `~/.claude.json` 中设置其 `hasTrustDialogAccepted` 标志的文件夹中应用。

市场按相对路径列出的插件在存储库的 `extraKnownMarketplaces` 条目应用后从市场副本加载。市场条目指向外部源（例如插件自己的 GitHub 存储库）的插件不会仅从存储库的设置安装。每个贡献者看到 `Plugin "<name>" is enabled in project settings but isn't installed`，直到他们运行 `claude plugin install <name>@<marketplace> --scope project`，如[安装插件](https://code.claude.com/docs/zh-CN/plugins/install.md)所述。

如果您使用带有相对路径的本地 `directory` 或 `file` 源，路径相对于您的存储库的主检出解析。当您从 git worktree 运行 Claude Code 时，路径仍指向主检出，因此所有 worktree 共享相同的市场位置。

要推出具有依赖关系的插件包，请将包插件放在 `enabledPlugins` 中，如[插件依赖关系](https://code.claude.com/docs/zh-CN/plugins/dependencies.md)所述。

### 每个表面何时应用插件键

该表显示每种 Claude Code 会话何时从托管设置和存储库的 `.claude/settings.json` 应用 `extraKnownMarketplaces` 和 `enabledPlugins`。对于 Desktop 应用和 IDE 扩展，请参阅[安装插件](https://code.claude.com/docs/zh-CN/plugins/install.md#install-a-plugin)。

| 表面 | 托管 `extraKnownMarketplaces` 和 `enabledPlugins` | 存储库 `.claude/settings.json` |
| :- | :- | :- |
| 终端，交互式 | 在接收设置的每台机器上的会话开始时应用 | `extraKnownMarketplaces` 在信任后应用；`enabledPlugins` 在会话开始时应用 |
| `-p` 和 CI | 在会话开始时应用，安装在后台运行 | 仅在受信任的文件夹中的 `extraKnownMarketplaces`；`enabledPlugins` 应用 |
| 云会话 | 在 Anthropic 托管的环境中，仅服务器托管设置到达会话，它在安装插件之前等待它们。MDM 策略和托管设置文件保留在用户的机器上。对于自托管环境，请参阅[策略应用的位置和时间](https://code.claude.com/docs/zh-CN/managed-settings.md#where-and-when-a-policy-applies) | 请参阅[安装插件](https://code.claude.com/docs/zh-CN/plugins/install.md#install-a-plugin)下的**云会话**选项卡 |

在 `-p` 或 CI 运行中，市场和插件在后台安装，因此插件可能在第一轮中丢失。设置 `CLAUDE_CODE_SYNC_PLUGIN_INSTALL=1` 使运行在其第一个查询之前等待安装。

### 确认推出

检查市场和插件是否到达机器或 CI 运行：

* **在一台机器上**：启动 Claude Code 并运行 `/plugin`。市场和插件被列出。
* **在 CI 中**：使用 `--output-format stream-json --verbose` 运行 `claude -p`。`init` 事件在 `plugins` 下列出加载的插件。

## 为容器和 CI 播种

对于无法在运行时克隆的容器镜像和 CI 运行器，在构建时预填充插件目录并在 `CLAUDE_CODE_PLUGIN_SEED_DIR` 处指向它。Claude Code 在启动时注册播种的市场并从播种加载插件缓存，无需克隆。

播种也为没有 git 主机账户的用户服务。

> [!NOTE]
>
> 在 CI/CD 环境中，在从私有存储库安装插件之前配置 git 凭证助手。在 GitHub Actions 上，导出具有市场存储库读取权限的令牌作为 `GH_TOKEN`，然后运行 `gh auth setup-git`。默认工作流令牌只能访问工作流自己的存储库，因此另一个存储库中的私有市场需要个人访问令牌或应用令牌。

**1. 在构建时安装到播种中**

设置 `CLAUDE_CODE_PLUGIN_CACHE_DIR` 为播种路径，以便市场和插件安装在那里而不是 `~/.claude/plugins`：

```bash theme={null}
CLAUDE_CODE_PLUGIN_CACHE_DIR=/opt/claude-seed claude plugin marketplace add your-org/your-marketplace
CLAUDE_CODE_PLUGIN_CACHE_DIR=/opt/claude-seed claude plugin install code-formatter@your-marketplace
```

播种的布局与 `~/.claude/plugins` 相同：`known_marketplaces.json`、`marketplaces/<name>/` 和 `cache/<marketplace>/<plugin>/<version>/`。您可以在与构建它不同的路径处挂载播种。

**2. 在运行时指向播种**

在容器的环境中设置 `CLAUDE_CODE_PLUGIN_SEED_DIR=/opt/claude-seed`。要使用多个播种，在 Unix 上用 `:` 分隔其路径，在 Windows 上用 `;` 分隔。Claude Code 使用包含给定市场或插件缓存的第一个播种。

**3. 启用插件**

播种中的插件不会自动启用。为每个要加载的播种插件在托管设置或存储库的 `.claude/settings.json` 中设置 `enabledPlugins`。

要验证播种，在镜像中使用 `--output-format stream-json --verbose` 运行 `claude -p`。在 `init` 事件的 `plugins` 列表中，每个加载的插件的 `path` 在播种下，例如 `/opt/claude-seed/cache/your-marketplace/code-formatter/1.0.0`。

播种市场遵循这些规则：

* **只读**：Claude Code 从不写入播种并为播种市场强制 `autoUpdate` 关闭。
* **播种条目优先**：在每次启动时，播种中声明的市场覆盖用户的同名条目。用户使用 `claude plugin disable` 选择退出播种插件，而不是通过删除市场。
* **更新和删除失败**：`claude plugin marketplace update <name>` 和在播种市场上不带 `--scope` 的 `remove` 失败，并显示命名播种目录的消息。
* **策略仍然适用**：[允许列表和阻止列表](#restrict-what-users-can-install)也检查播种市场的记录源。允许您构建播种的源。

对于没有出站 git 访问的车队，将播种与共享挂载上的 `directory` 或 `file` 市场源结合。也设置 `CLAUDE_CODE_DISABLE_NONESSENTIAL_TRAFFIC=1`，这也关闭[插件自动更新](https://code.claude.com/docs/zh-CN/plugins/loading.md#when-auto-update-runs)。如果代理可用，请参阅[代理配置](https://code.claude.com/docs/zh-CN/network-config.md#proxy-configuration)了解要设置的变量。

## 限制用户可以安装的内容

托管 `strictKnownMarketplaces` 允许列表和 `blockedMarketplaces` 阻止列表决定插件可能来自哪些市场源。市场的源是 Claude Code 从中获取它的 git 存储库、URL 或本地路径。两个列表都匹配插件来自的市场的源，而不是该市场内的插件自己的条目。

对于常见的锁定，允许官方市场和您自己的，请参阅[允许官方市场和您自己的](#allow-the-official-marketplace-and-your-own)。将其与[`disableSideloadFlags`](#control-matrix)配对，以便用户无法从本地目录或 URL 加载插件。

两个列表在任何下载之前和会话开始时应用：

* **在下载之前**：当用户添加市场以及在每次安装、更新、刷新和自动更新时应用列表。
* **在会话开始时**：列表再次应用于已安装的插件，因此已安装的插件其市场源不再匹配不加载。`/plugin` 将其列为 `Marketplace "<name>" is not in the allowed marketplace list` 或 `Marketplace "<name>" is blocked by enterprise policy`。

两个列表的执行位置取决于您在哪里设置它们：

* **claude.ai 管理控制台**：Claude Code 在[读取服务器托管设置](https://code.claude.com/docs/zh-CN/managed-settings.md#where-and-when-a-policy-applies)的会话中执行两个列表。claude.ai 也在您组织中的任何人从 git 存储库在 claude.ai 上添加新市场，或从 Claude Desktop 应用中 Code 选项卡之外的**自定义**添加新市场时检查它们。这涵盖成员为自己的账户添加的市场和在[**组织设置 > 插件**](https://claude.ai/admin-settings/plugins)下为整个组织添加的市场。claude.ai 拒绝允许列表不允许或阻止列表命名的存储库。它不重新检查在您设置列表之前在任一位置添加的市场，也不检查上传的插件。
* **托管设置文件、OS 级策略或其他托管源**：Claude Code 在读取该源的地方执行两个列表。claude.ai 不读取它。

虽然设置了任何允许列表，或阻止列表命名除[`skills-dir`](#blocklist-with-blockedmarketplaces)之外的任何源，Claude Code 找不到的市场的插件不加载。`/plugin` 为其显示策略错误而不是未找到错误。常见情况是市场的陈旧 `enabledPlugins` 条目，没有人注册。

### 控制矩阵

该表列出每个插件策略键、它执行的内容以及它无法做的内容。

| 键 | 它执行的内容 | 它无法做的内容 |
| :- | :- | :- |
| `strictKnownMarketplaces` | 市场源的允许列表。`[]` 阻止每个源，包括官方市场。别名：`allowedMarketplaces` | 不注册市场、限制允许市场内的条目或阻止 `--plugin-dir` |
| `blockedMarketplaces` | 市场源的阻止列表，在允许列表之前检查 | 不阻止已从不匹配的源注册的市场 |
| `syncClaudeAiPlugins` | 设置 `false` 以停止 Claude Code 下载和加载为每个用户账户[从 claude.ai 同步](https://code.claude.com/docs/zh-CN/plugins/loading.md#synced-plugins)的插件。需要 Claude Code v2.1.273 或更高版本 | 不关闭一个同步插件。为此，在[`enabledPlugins`](https://code.claude.com/docs/zh-CN/settings-reference#enabledplugins)中设置 `"<name>@synced": false` |
| `enabledPlugins` | `true` 强制启用，`false` 在每个范围内阻止并隐藏插件 | 不安装其市场未注册或不允许的插件 |
| `disableSideloadFlags` | 在启动时拒绝 `--plugin-dir`、`--plugin-url`、`--agents`、Agent SDK `plugins` 选项以及 [`CLAUDE_CODE_PLUGIN_DIRS`](https://code.claude.com/docs/zh-CN/env-vars#variables) 变量中命名的文件夹，并且也拒绝 `--mcp-config`，除非其中的每个服务器都是进程内 `type: "sdk"` 条目。拒绝 Agent SDK 应用程序在会话中途通过 [`addPlugins`](https://code.claude.com/docs/zh-CN/agent-sdk/typescript#load-more-plugin-folders-with-addplugins) 添加的插件文件夹 | 不限制 `.mcp.json`、`claude mcp add` 或 SDK 提供的服务器。将其与[`allowedMcpServers`](https://code.claude.com/docs/zh-CN/managed-mcp)配对 |
| `disableCommandPluginSources` | 阻止具有 `command` 源的插件安装、更新或加载。`command` 源是其插件目录通过在机器上运行命令产生的源。未设置时，它采用 `allowManagedHooksOnly` 的值 | 不影响其他源类型 |
| `allowManagedHooksOnly` | 限制哪些 hooks 运行。请参阅[`allowManagedHooksOnly`](https://code.claude.com/docs/zh-CN/settings-reference#allowmanagedhooksonly) | 不信任用户自己启用的插件中的 hooks |
| `strictPluginOnlyCustomization` | 阻止不来自插件、托管设置或 Claude Code 内置的 skill、Agent、hook 和 MCP 服务器。它还会阻止 [skill 目录插件](#keep-skills-directory-plugins-loading)加载。设置 `true` 以覆盖所有四种类型，或 `skills`、`agents`、`hooks` 和 `mcp` 值的数组（例如 `["skills", "hooks"]`）以覆盖某些 | 不限制用户安装哪些插件。将其与 `strictKnownMarketplaces` 配对 |
| `pluginSuggestionMarketplaces` | 其插件可能在终端中显示为安装建议的市场。请参阅[推荐插件](#recommend-plugins) | 不影响内置提示 |
| `pluginTrustMessage` | 将您的文本附加到 `/plugin` 在插件安装之前显示的信任警告 | 不改变警告自己的文本 |
| `allowedChannelPlugins` | 替换允许推送频道消息的默认插件列表。需要 `channelsEnabled: true` | 请参阅[限制哪些频道插件可以运行](https://code.claude.com/docs/zh-CN/channels.md#restrict-which-channel-plugins-can-run) |
| [`CLAUDE_CODE_DISABLE_OFFICIAL_MARKETPLACE_AUTOINSTALL=1`](https://code.claude.com/docs/zh-CN/env-vars) | 停止交互式终端会话自动注册官方市场 | 不删除已注册的市场。允许列表和阻止列表在没有它的情况下门控相同的自动注册。在设置它的情况下启动一次的机器在您取消设置它后不会恢复自动注册 |
| [`allowManagedModsOnly`](https://code.claude.com/docs/zh-CN/plugins/mods/admin#stop-user-installed-mods-from-loading) | 阻止每个不[算作您组织所有](https://code.claude.com/docs/zh-CN/plugins/mods/admin.md#install-your-organizations-mods)的已安装 [mod](https://code.claude.com/docs/zh-CN/plugins/mods/overview.md) 运行其 hook | 不停止包含 mod 的插件安装。为此，使用此表中的市场键 |

表中的每个键都是托管设置，除了 `enabledPlugins`、`syncClaudeAiPlugins`、`CLAUDE_CODE_DISABLE_OFFICIAL_MARKETPLACE_AUTOINSTALL` 和 `allowManagedModsOnly`：

* **`enabledPlugins`**：您可以在任何范围内设置它，托管设置锁定它。
* **`syncClaudeAiPlugins`**：每个用户也可以在自己的用户或本地设置中设置它。请参阅其[设置参考中的范围](https://code.claude.com/docs/zh-CN/settings-reference.md#syncclaudeaiplugins)。
* **`CLAUDE_CODE_DISABLE_OFFICIAL_MARKETPLACE_AUTOINSTALL`**：这是一个环境变量，您通过[关闭整个车队的更新](#turn-updates-off-for-the-whole-fleet)下显示的托管 `env` 块交付。
* **`allowManagedModsOnly`**：这是一个内置插件上的选项，您在托管设置中的 `pluginConfigs` 下设置。请参阅[停止用户安装的 mods 从加载](https://code.claude.com/docs/zh-CN/plugins/mods/admin.md#stop-user-installed-mods-from-loading)。

此处的每个设置键在[设置参考](https://code.claude.com/docs/zh-CN/settings-reference.md)中都有条目。

#### 市场键的别名

`strictKnownMarketplaces` 也可以拼写为 `allowedMarketplaces`，`extraKnownMarketplaces` 也可以拼写为 `additionalMarketplaces`。

* **版本**：别名需要 Claude Code v2.1.232 或更高版本，较旧的客户端忽略它们。在混合车队读取的文件中，保持规范名称。
* **两个拼写都设置**：当文件设置两个拼写时，规范键的值应用。

### 使用 `strictKnownMarketplaces` 的允许列表

将允许列表设置为这些源对象的列表。大多数条目完全匹配，`hostPattern` 和 `pathPattern` 条目作为正则表达式匹配，`github` 所有者通配符按所有者匹配：

* **`github`**：`{ "source": "github", "repo": "your-org/approved-plugins" }`，带可选的 `ref` 和 `path`。
* **`github` 所有者通配符**：`{ "source": "github", "repo": "your-org/*" }` 匹配该所有者下的每个存储库。`*` 必须代表整个存储库名称。Claude Code 忽略诸如 `*/plugins` 和 `your-org/tools-*` 之类的条目作为无效，因此它们不匹配任何内容。需要 Claude Code v2.1.223 或更高版本。
* **`git`**：`{ "source": "git", "url": "https://gitlab.example.com/tools/plugins.git" }`，带可选的 `ref` 和 `path`。
* **`url`**：`{ "source": "url", "url": "https://plugins.example.com/marketplace.json" }`，带可选的 `headers`。
* **`file` 和 `directory`**：`{ "source": "file", "path": "/opt/marketplace/marketplace.json" }` 或 `{ "source": "directory", "path": "/opt/marketplace/plugins" }`，带绝对路径。
* **`hostPattern`**：`{ "source": "hostPattern", "hostPattern": "^github\\.example\\.com$" }`，与 `github`、`git` 和 `url` 源的主机匹配。该模式在主机名中的任何地方匹配，因此如所示用 `^` 和 `$` 锚定以匹配整个主机。`github` 源始终计为 `github.com`。对于开发人员创建自己的市场的 GitHub Enterprise Server 或 GitLab 主机，使用 `hostPattern` 条目。[GHES 页面](https://code.claude.com/docs/zh-CN/github-enterprise-server.md#allowlist-ghes-marketplaces-in-managed-settings)有工作示例。
* **`pathPattern`**：`{ "source": "pathPattern", "pathPattern": "^/opt/approved/" }`，与 `file` 和 `directory` 源的 `path` 匹配。该模式在路径中的任何地方匹配，因此以 `^` 开头以固定目录前缀。`".*"` 允许每个本地路径。
* **`skills-dir`**：`{ "source": "skills-dir" }` 不匹配任何市场。有关其作用，请参阅[保持 skill 目录插件加载](#keep-skills-directory-plugins-loading)。

#### 条目如何匹配

`url` 条目在其 `url` 值上匹配；`headers` 不被比较。对于 `github` 和 `git` 条目，`repo` 或 `url`、`ref` 和 `path` 必须都匹配，或在两侧都不存在：

* 没有 `ref` 的条目不覆盖具有 `ref: "main"` 的源。
* `your-org/your-marketplace` 的条目不覆盖克隆相同存储库的 `git` URL。
* 尾部斜杠、`.git` 后缀或 `ssh://` 代替 `https://` 是不同的值。当市场可以通过多个 URL 克隆时，更喜欢 `hostPattern` 条目。

所有者通配符条目遵循 `ref` 的确切规则，并匹配存储库内的任何 `path`，除非条目固定一个。通配符匹配在允许列表上区分大小写。

#### 保持技能目录插件加载

skill 目录插件是用户在 `~/.claude/skills/` 或项目的 `.claude/skills/` 下保持的插件，在带有 `.claude-plugin/plugin.json` 的文件夹中。如果您设置任何没有 `{ "source": "skills-dir" }` 条目的允许列表，它们停止加载。如果您使用 [`strictPluginOnlyCustomization`](https://code.claude.com/docs/zh-CN/settings-reference#strictpluginonlycustomization) 锁定任何自定义类型，无论该条目是否存在，它们都会停止加载。允许列表不适用于普通 [skill](https://code.claude.com/docs/zh-CN/skills.md)，即没有该清单的 `SKILL.md`。

#### 在 claude.ai 上托管的市场

允许列表和阻止列表通过其主机匹配[在 claude.ai 上托管的市场](https://code.claude.com/docs/zh-CN/plugins/install.md#add-from-claude-ai)。要允许或阻止一个，将与 `claude.ai` 匹配的 `hostPattern` 条目添加到 `strictKnownMarketplaces` 或 `blockedMarketplaces`。在允许列表上，这样的条目允许您组织的 claude.ai 市场和 claude.ai 默认市场，但不允许由成员自己的 claude.ai 上传组成的市场或其范围 claude.ai 未声明的市场。需要 Claude Code v2.1.273 或更高版本。

#### 锁定每个源

空允许列表 `[]` 锁定每个市场源，包括官方市场。

此锁定不覆盖[从 claude.ai 同步](https://code.claude.com/docs/zh-CN/plugins/loading.md#synced-plugins)的插件，Claude Code 从每个用户的账户而不是从市场下载。要也停止那些，请在托管设置中将[`syncClaudeAiPlugins`](https://code.claude.com/docs/zh-CN/settings-reference#syncclaudeaiplugins)设置为 `false`，或在 claude.ai 上为您的组织关闭技能。

### 使用 `blockedMarketplaces` 的阻止列表

`blockedMarketplaces` 采用与[`strictKnownMarketplaces`](#allowlist-with-strictknownmarketplaces)相同的源对象，并首先检查，因此两个列表上的源被阻止。阻止列表匹配比允许列表匹配更宽：

* Git URL 被规范化，因此一个 `github.com` 存储库的 `git@` 和 `https://` 形式、`.git` 后缀和尾部斜杠都匹配相同的条目。
* `github` 条目也阻止等效的 `git` URL，反之亦然。
* 对于 `owner/*` 条目，所有者比较不区分大小写。
* 没有 `ref` 或 `path` 的条目阻止它匹配的存储库的每个 ref 和 path。

此条目阻止一个 GitHub 所有者下的每个存储库：

```json theme={null}
{
  "blockedMarketplaces": [
    { "source": "github", "repo": "untrusted-org/*" }
  ]
}
```

`blockedMarketplaces` 中的 `url` 条目也在用户添加 Claude Code [克隆而不是获取](https://code.claude.com/docs/zh-CN/plugins/cli-reference.md#plugin-marketplace-add)的 `https://` 存储库 URL 时应用，例如裸 `github.com` 或 `gitlab.com` 存储库 URL。如果条目命名该 URL，用户无法添加它。匹配忽略 `.git` 后缀和用户在 `#` 后附加的任何 ref。需要 Claude Code v2.1.232 或更高版本。

此处的 `{ "source": "skills-dir" }` 条目停止[技能目录插件](#keep-skills-directory-plugins-loading)从 `~/.claude/skills/` 和项目的 `.claude/skills/` 加载。

仅命名该条目的阻止列表不计为活跃限制，因此它不[停止 Claude Code 找不到其市场的插件](#restrict-what-users-can-install)加载。

### 允许官方市场和您自己的

大多数组织允许官方市场和他们自己的，并注册两者，以便每台机器都有它们。此托管设置策略允许两个市场，注册两者，强制启用两个插件，并拒绝 `--plugin-dir`：

```json theme={null}
{
  "strictKnownMarketplaces": [
    { "source": "github", "repo": "anthropics/claude-plugins-official" },
    { "source": "github", "repo": "your-org/*" },
    { "source": "skills-dir" }
  ],
  "extraKnownMarketplaces": {
    "claude-plugins-official": {
      "source": { "source": "github", "repo": "anthropics/claude-plugins-official" }
    },
    "your-marketplace": {
      "source": { "source": "github", "repo": "your-org/your-marketplace" }
    }
  },
  "enabledPlugins": {
    "code-formatter@your-marketplace": true,
    "deploy-helper@your-marketplace": true
  },
  "disableSideloadFlags": true
}
```

在具有此策略的机器上，添加列表外的任何源，例如 `/plugin marketplace add https://example.com/other-marketplace.git`，失败，消息包含 `is blocked by enterprise policy` 后跟允许的源。`claude --plugin-dir ./x` 以命名 `disableSideloadFlags` 的消息退出。

如果您从此允许列表中删除 `{ "source": "skills-dir" }` 条目，[skill 目录插件](#keep-skills-directory-plugins-loading)将停止加载。

使用显式 `extraKnownMarketplaces` 条目注册两个市场，如此策略所做的那样，而不是依赖允许列表或官方市场注册自己：

* **允许列表不注册任何内容**：`extraKnownMarketplaces` 条目注册，它本身必须通过允许列表。Claude Code 拒绝注册其源允许列表不匹配的托管市场。
* **官方市场仅在交互式终端会话中注册自己**：即使在那里，它也仅在允许列表允许时注册。`-p` 运行或附加到云会话的终端从不注册它。
* **阻止的尝试被记住**：如果机器曾在阻止官方市场的策略下运行，Claude Code 记录阻止的尝试，并在策略更改后不重试。`[]` 锁定是一个这样的策略。该机器仅通过 `extraKnownMarketplaces` 条目（例如此策略中的条目）、其一个插件的 `enabledPlugins` 条目或手动 `/plugin marketplace add` 再次注册它。

## 设置更新策略

您可以按市场、整个车队或通过发布频道按用户组设置更新策略。

### 按市场打开或关闭自动更新

插件自动更新在启动后在后台为打开它的市场运行。有关默认打开哪些市场，请参阅[自动更新何时运行](https://code.claude.com/docs/zh-CN/plugins/loading.md#when-auto-update-runs)。要为车队决定，在托管 `extraKnownMarketplaces` 条目上设置 `"autoUpdate": true` 或 `false`：

* 如果托管条目设置字段，Claude Code 拒绝用户的 `/plugin` 切换，错误以 `Auto-update for '<name>' is set by` 开头。
* 如果托管条目保留字段未设置，用户的切换持续。

### 关闭整个车队的更新

要关闭每个市场的插件自动更新，在托管 `env` 块中设置 `DISABLE_AUTOUPDATER`，如此示例所做的那样。相同的变量也停止 Claude Code 自己的更新：

```json theme={null}
{
  "env": {
    "DISABLE_AUTOUPDATER": "1"
  }
}
```

要停止 Claude Code 自己的更新但保持插件自动更新，将 `"FORCE_AUTOUPDATE_PLUGINS": "1"` 添加到相同的块。其他[停止插件自动更新的环境变量](https://code.claude.com/docs/zh-CN/plugins/loading.md#when-auto-update-runs)以相同的方式工作。

`DISABLE_AUTOUPDATER` 不覆盖具有[`command` 源](https://code.claude.com/docs/zh-CN/plugins/marketplace-reference#command-plugin-source)的插件。Claude Code 每个会话重新运行每个启用的命令，并在其更改时安装输出。有关停止那些运行的内容，请参阅[命令源何时重新运行](https://code.claude.com/docs/zh-CN/plugins/loading.md#when-a-command-source-re-runs)。

### 为用户组分配发布频道

要运行稳定和早期访问频道，托管两个指向相同插件的不同 ref 的市场。然后通过单独的端点托管设置或网关策略为每个用户组提供自己的市场。来自管理控制台的服务器托管设置[应用于组织中的每个用户](https://code.claude.com/docs/zh-CN/server-managed-settings.md#current-limitations)，因此它们无法为不同的组分配不同的设置。

* 将单独的[端点托管设置](https://code.claude.com/docs/zh-CN/managed-settings.md#delivery-mechanisms)（例如托管设置文件或 MDM 配置文件）部署到每个组的设备。要检查按组文件或配置文件是否在也有组织范围源的设备上应用，请参阅[Claude Code 如何组合托管源](https://code.claude.com/docs/zh-CN/managed-settings.md#precedence-within-the-managed-tier)。
* 为每个组定义一个[Claude 应用网关策略](https://code.claude.com/docs/zh-CN/claude-apps-gateway-config.md#managed)。网关应用第一个匹配规则适合用户的策略，因此订购策略以便每个用户到达其组的策略。该策略的 `extraKnownMarketplaces` 映射不与任何其他策略的合并，因此在其中列出组需要的每个市场，而不仅仅是其频道市场。

使用任一机制，稳定组接收此配置：

```json theme={null}
{
  "extraKnownMarketplaces": {
    "stable-tools": {
      "source": { "source": "github", "repo": "your-org/stable-tools" }
    }
  }
}
```

早期访问组接收 `latest-tools` 代替。要设置两个市场，请参阅[运行发布频道](https://code.claude.com/docs/zh-CN/plugins/host-marketplace.md#run-release-channels)。

## 推荐插件

市场所有者可以将 `relevance` 信号附加到条目，以便 Claude Code 在项目匹配时建议插件。

来自市场的建议仅在其在用户的机器上注册、您在托管设置中的 `pluginSuggestionMarketplaces` 中列出其名称，并且您在相同策略中声明其源时出现。声明源作为市场的 `extraKnownMarketplaces` 条目或允许列表条目。官方市场仅需要名称。请参阅[在托管设置中启用建议](https://code.claude.com/docs/zh-CN/plugins/relevance.md#enable-suggestions-in-managed-settings)。

> [!NOTE]
>
> 在云端会话的对话中，无论您是否设置了 `pluginSuggestionMarketplaces`，Claude 都可以建议插件。要关闭这些建议，请参阅[关闭插件建议](https://code.claude.com/docs/zh-CN/claude-code-on-the-web.md#turn-off-plugin-suggestions)。

## 审计和审查

OpenTelemetry 事件和 Analytics API 告诉您您的车队安装和运行什么。

有关插件可以在机器上运行什么以及每个信任层允许什么，在批准市场之前阅读[插件安全](https://code.claude.com/docs/zh-CN/plugins/security.md)。

### OpenTelemetry 事件

`claude_code.plugin_installed` 记录每次安装，`claude_code.plugin_loaded` 记录每个启用的插件在会话开始时。除非您设置 `OTEL_LOG_TOOL_DETAILS=1`，否则两个事件都删除或省略第三方插件和市场名称，如[您的后端中的删除插件名称](https://code.claude.com/docs/zh-CN/plugins/measure.md#redacted-plugin-names-in-your-backend)所示。字段列表在[插件已安装事件](https://code.claude.com/docs/zh-CN/monitoring-usage.md#plugin-installed-event)和[插件已加载事件](https://code.claude.com/docs/zh-CN/monitoring-usage.md#plugin-loaded-event)下。

### Analytics API

在 Enterprise 计划上，`GET /v1/organizations/analytics/plugins` 返回跨 Claude Code 和 Cowork 的每个插件、每天的安装和调用计数。您可以按用户或 RBAC 组对计数进行分组。到达 Anthropic 而没有插件名称的插件活动出现在一个聚合 `third-party` 行中。请参阅[端点参考](https://platform.claude.com/docs/en/api/admin/analytics/plugins/list)和[以编程方式访问数据](https://code.claude.com/docs/zh-CN/analytics.md#access-data-programmatically)了解它需要的键。

## 规划托管设置无法执行的内容

这些来自安全审查的请求在当前设置架构中没有专用键。最接近的现有控制是：

* **按用户或按组目标**：每个插件键应用于接收设置的每个用户。服务器托管设置为每个组织交付一个配置。对于按组策略，使用单独的端点托管设置或网关策略，如[为用户组分配发布频道](#assign-release-channels-to-user-groups)下所述。
* **限制允许市场内的条目**：允许列表匹配市场源。要从允许的市场阻止一个插件，在托管 `enabledPlugins` 中将其设置为 `false`。
* **隐藏 `/plugin`**：没有键禁用命令。最接近的等效项结合仅命名您的市场的允许列表、您提供的插件的托管 `enabledPlugins` 条目和 `disableSideloadFlags`。
* **通过允许列表门控 `--plugin-dir`**：允许列表不覆盖 `--plugin-dir`。`disableSideloadFlags` 覆盖。
* **通过这些键执行 claude.ai 插件切换**：[**组织设置 > 插件和技能**](https://claude.ai/admin-settings/skills?tab=inventory)不设置此页面上的键。成员和您的组织在那里打开的内容作为[同步插件](https://code.claude.com/docs/zh-CN/plugins/loading.md#synced-plugins)到达 CLI，它们有自己的控制。

## 策略故障排除

如果插件策略在机器上的行为不符合预期，首先检查这些症状：

* **托管文件未解析**：当 `managed-settings.json` 不是有效的 JSON 时，Claude Code 拒绝启动并打印[命名文件的错误](https://code.claude.com/docs/zh-CN/errors.md#managed-settings-document-could-not-be-parsed)。解析但有一个无效条目的文件保持其策略的其余部分。请参阅[托管设置中的无效条目](https://code.claude.com/docs/zh-CN/managed-settings.md#invalid-entries-in-managed-settings)。
* **托管源未加载**：运行 `/status` 并在 `Setting sources` 行中查找 `Enterprise managed settings`。如果缺少，源未加载。
* **用户报告 `blocked by enterprise policy`**：消息命名市场或其源。对于允许列表，它也列出允许的源。面向用户的条目在[插件故障排除](https://code.claude.com/docs/zh-CN/plugins/troubleshooting.md)上。
* **用户在 `~/.claude/settings.json` 中禁用的插件仍然加载**：另一个设置源重新启用它，例如强制启用它的托管 `enabledPlugins` 条目。`/plugin` 和 `claude plugin list` 显示 `Disabled in ~/.claude/settings.json but still loads` 与该设置源。

## 后续步骤

* [市场参考](https://code.claude.com/docs/zh-CN/plugins/marketplace-reference.md#marketplace-sources)：`extraKnownMarketplaces`、`strictKnownMarketplaces` 和 `blockedMarketplaces` 接受的 `source` 值
* [托管和维护市场](https://code.claude.com/docs/zh-CN/plugins/host-marketplace.md)：运行您的策略指向的市场
* [插件安全和信任](https://code.claude.com/docs/zh-CN/plugins/security.md)：插件可以在机器上做什么以及在安装前如何审查一个
* [管理您的组织的 mods](https://code.claude.com/docs/zh-CN/plugins/mods/admin.md)：关闭或限制 mods，在 Claude Code 内运行 JavaScript 的插件
* [服务器托管设置](https://code.claude.com/docs/zh-CN/server-managed-settings.md)：从 claude.ai 管理控制台交付这些键
* [插件故障排除](https://code.claude.com/docs/zh-CN/plugins/troubleshooting.md#blocked-by-your-organization)：策略阻止用户时看到的消息
