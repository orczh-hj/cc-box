> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# 设置文件和优先级

> 更改 Claude Code 设置，选择键所属的作用域，验证更改，并了解当键在多个位置设置时 Claude Code 使用哪个值。

设置是改变 Claude Code 行为方式的 JSON 键：它启动时使用的模型、它可以在不询问的情况下运行的内容、它无法读取的文件、它在终端中的外观，以及您的组织强制执行的内容。

> [!TIP]
>
> 要查找特定键，请转到[所有设置](https://code.claude.com/docs/zh-CN/settings-reference.md)，其中列出了每个键、设置它的文件、其默认值和示例。

Claude Code 从 JSON 设置文件（如 `~/.claude/settings.json`）读取设置。它在几个位置查找它们，[它读取设置的文件决定了设置适用于谁](#settings-files-and-who-they-affect)。本页涵盖这些文件：将设置放在哪个文件中、如何更改设置并确认它已应用，以及当同一键在多个文件中设置时 Claude Code 使用哪个值。[配置权限](https://code.claude.com/docs/zh-CN/permissions.md)涵盖 Claude Code 可以在不询问的情况下运行的内容以及如何编写 `allow`、`ask` 和 `deny` 规则。

> [!NOTE]
>
> 本页涵盖在您的机器上运行的 Claude Code：终端、[VS Code](https://code.claude.com/docs/zh-CN/vs-code.md) 和 [JetBrains](https://code.claude.com/docs/zh-CN/jetbrains.md) 扩展，以及[桌面应用](https://code.claude.com/docs/zh-CN/desktop.md)，它们都读取相同的设置文件。[Claude Code on the web](https://code.claude.com/docs/zh-CN/claude-code-on-the-web.md) 上的云会话在不同的机器上运行并仅读取其中一些；请参阅[云会话中的设置](#settings-in-cloud-sessions)。

## 设置文件及其影响范围

Claude Code 从四个文件读取设置，组织也可以从 claude.ai 控制台提供托管设置。每个来源都有一个作用域：保存在其中的设置所适用的人员和项目范围，可能是仅限于您、项目中的所有人，或组织中的所有人。

| 作用域 | 文件 | 影响范围 | 用途 |
| :- | :- | :- | :- |
| 用户 | `~/.claude/settings.json` | 您在这台机器上的每个项目 | 个人偏好：主题、编辑器模式、默认模型、您自己的权限规则 |
| 共享项目 | `.claude/settings.json` | 在包含该文件的文件夹中工作的所有人。在 git 仓库中，提交该文件以便队友获得它 | 团队权限、hook、插件以及项目所需的环境变量 |
| 项目本地 | `.claude/settings.local.json` | 仅限您，且仅在这一个项目中。Claude Code 在创建该文件时会将其排除在 git 之外；如果您手动创建，请自行将其添加到 `.gitignore` | 针对单个项目的个人覆盖，以及共享前的测试 |
| 托管 | `managed-settings.json` 和其他[托管来源](https://code.claude.com/docs/zh-CN/managed-settings.md#delivery-mechanisms) | 您的组织部署到的所有人；[设置优先级](#settings-precedence)说明了哪些内容可以覆盖它 | 安全策略和合规要求 |

在"文件"列中，`~/.claude` 是您主目录中的 `.claude` 文件夹，而单独的 `.claude` 是项目内的 `.claude` 文件夹。

### 比较每个设置文件的作用域

假设您的机器上有三个项目：`website/`、`api/` 和 `acme-app/`，一位队友有自己的 `acme-app/` 克隆，并且您在 `acme-app/` 上启动了一个[云端会话](#settings-in-cloud-sessions)。

下图显示了当您从这些文件夹启动 Claude Code 时，设置会在哪些文件夹中生效。点击某个设置文件即可查看它覆盖到的文件夹。

Settings file

Your machine

A teammate’s machine

A cloud session

* **`~/.claude/settings.json`**：您机器上的每个项目，而队友的机器上和云端会话中都不会生效
* **`acme-app/.claude/settings.json`**：您的 `acme-app/`。只有当您将该文件提交到版本控制后，它才会覆盖到队友的克隆和云端会话；在此之前，它只是您磁盘上的一个普通文件，其他人都没有它
* **`acme-app/.claude/settings.local.json`**：仅您的 `acme-app/`。Claude Code 第一次写入该文件时会将其添加到您的全局 git 排除项中，因此它不会进入您的提交；如果您手动创建该文件，请[自行将其添加到 `.gitignore`](#keep-personal-settings-out-of-a-repository)
* **托管设置**，无论是 `managed-settings.json` 文件、MDM 策略，还是来自 claude.ai 控制台的[服务器托管设置](https://code.claude.com/docs/zh-CN/server-managed-settings.md)：您的组织部署到的每台机器上的每个项目，或您使用组织账户登录的每台机器上的每个项目。只有服务器托管设置会覆盖到云端会话

### 查找或创建设置文件

安装 Claude Code 不会创建任何设置文件。如果您的机器或项目中已经有设置文件，它来自以下来源之一：

* **托管**：由您的组织部署。您无需创建或编辑它。
* **共享项目**：已经使用 Claude Code 的项目可能已提交了该文件。如果没有，请在项目文件夹中的 `.claude/settings.json` 创建它。
* **用户**和**项目本地**：您可以自行创建，也可以让 Claude Code 创建。当您第一次在 `/config` 菜单中更改存储在用户设置中的选项（例如主题）时，它会写入 `~/.claude/settings.json`；当您第一次在权限提示上给予常设批准（例如对 Bash 命令选择"Yes, and don't ask again"）时，它会写入 `.claude/settings.local.json`。少数 `/config` 选项（包括 **Show tips**）会保存到 `.claude/settings.local.json`，而不是用户文件。

> [!NOTE]
>
> 在 Windows 上，`~/.claude` 表示 `%USERPROFILE%\.claude`。要将主目录中的文件保存在其他位置，请设置 [`CLAUDE_CONFIG_DIR`](https://code.claude.com/docs/zh-CN/env-vars)；Claude Code 随后会将您的设置、会话历史和插件存储在该位置。

Claude Code 还会保留第五个文件 [`~/.claude.json`](https://code.claude.com/docs/zh-CN/claude-directory#ce-claude-json)，由它自行写入；您无需编辑它。该文件保存您的账户详细信息、[MCP 服务器](https://code.claude.com/docs/zh-CN/mcp.md)配置、每个项目的状态（例如信任决定），以及 `/config` 为您写入的[全局配置键](https://code.claude.com/docs/zh-CN/settings-reference.md#global-config-settings)。

### 与团队共享设置

提交 `.claude/settings.json`，以便克隆仓库的每个人都获得相同的权限、hook 和插件。每位队友仍可以在自己的 `.claude/settings.local.json` 中为自己覆盖它，因此个人例外无需提交。有关完整的团队文件，请参阅[团队的共享设置](https://code.claude.com/docs/zh-CN/settings-example.md#a-teams-shared-settings)。

您提交的部分内容要等到每位队友[信任该文件夹](https://code.claude.com/docs/zh-CN/permissions.md#project-allow-rules-and-workspace-trust)后才会生效，还有少数键永远不会从仓库文件中生效；[排查不生效的设置](#common-cases)涵盖了这两种情况。

### 将个人设置保留在仓库之外

要在某个项目中仅为自己更改设置而不影响队友，请将其保存在项目内的 `.claude/settings.local.json` 中。Claude Code 会在已提交的 `.claude/settings.json` 之上应用该文件，因此如果团队文件设置了 `"model": "claude-sonnet-5"` 而您想使用 Opus，请在本地文件中放入 `"model": "claude-opus-5-5"`，这样只有您的会话会发生变化。

Claude Code 也会写入此文件，将其排除在您的提交之外，并且无需信任步骤即可应用其允许规则：

* **Claude Code 也会写入它。** 当 Claude 请求运行 Bash 命令的权限而您选择"Yes, and don't ask again"时，Claude Code 会将该[权限批准](https://code.claude.com/docs/zh-CN/permissions.md#permission-system)作为 `allow` 规则保存在此处。
* **您无需自行将其加入 gitignore，除非您是手动创建的。** Claude Code 第一次在尚未忽略该文件的 git 仓库中写入它时，会将 `**/.claude/settings.local.json` 添加到您的全局 git 排除文件中，因此该文件在每个仓库中都不会进入您的提交。当您的全局 git 配置将 `core.excludesFile` 设置为绝对路径或以 `~` 开头的路径时，该排除文件就是 `core.excludesFile`；否则为 `$XDG_CONFIG_HOME/git/ignore`，或在未设置 `XDG_CONFIG_HOME` 时为 `~/.config/git/ignore`。如果您手动创建了该文件且 Claude Code 尚未写入它，请自行将其添加到 `.gitignore`。
* **只要文件保持未跟踪状态，其允许规则就无需等待信任。** 由于该文件属于您而不属于仓库，Claude Code 会应用其 `allow` 规则，而无需经过已提交文件所要求的[工作区信任](https://code.claude.com/docs/zh-CN/permissions.md#project-allow-rules-and-workspace-trust)步骤。如果该文件被 git 跟踪，信任步骤同样适用于它；请参阅[本地设置文件何时需要信任](https://code.claude.com/docs/zh-CN/permissions.md#when-your-local-settings-file-needs-trust)。

#### Claude Code 在 git 仓库中保存本地文件的位置

当 Claude 请求运行 Bash 命令的权限而您选择"Yes, and don't ask again"时，Claude Code 会将该批准作为 `allow` 规则保存在 `.claude/settings.local.json` 中。如果您在 git 仓库的子目录中启动 Claude Code，它会在仓库根目录读取和写入该文件，并在整个仓库中应用该批准。在 [worktree](https://code.claude.com/docs/zh-CN/worktrees.md) 中，它使用主检出根目录下的文件。

有两条规则对根目录位置加以限定：

* **文件改为与 `.claude/settings.json` 放在一起的情况**：在 git 仓库之外、仓库根目录是您的主目录时、在 Windows 上，或者仓库根目录或其 `.git` 或 `.claude` 条目不归您的用户所有时。
* **文件中的路径不以仓库根目录为锚点**：以 `/` 开头的权限规则或相对沙箱路径改为[以会话的主工作目录为锚点](https://code.claude.com/docs/zh-CN/permissions.md#read-and-edit)。

在 v2.1.211 之前，Claude Code 将该文件保存在启动目录中。它仍会读取早期版本留在那里的文件，并与根目录文件一同读取；当两者设置了相同的键时，以根目录文件的值为准，而两个文件中的权限规则都会生效。Agent SDK 的 [`resolveSettings()`](https://code.claude.com/docs/zh-CN/agent-sdk/typescript#resolvesettings) 辅助函数始终从启动目录读取该文件。

Claude Code 从会话的[主工作目录](https://code.claude.com/docs/zh-CN/permissions.md#working-directories)读取共享的 `.claude/settings.json`，因此要使用提交在仓库根目录的文件，请从那里启动 Claude Code。在您[使用 `/cd` 移动会话](https://code.claude.com/docs/zh-CN/permissions#move-the-session-to-another-directory)后，Claude Code 会改为从新目录读取这两个项目文件，并按相同规则放置本地文件。从您移动到的目录读取它们需要 Claude Code v2.1.246 或更高版本。对于从桌面应用启动的 worktree 会话，请参阅[worktree 与主检出共享的内容](https://code.claude.com/docs/zh-CN/worktrees.md#what-worktrees-share-with-the-main-checkout)。

### 检查组织强制执行的内容

如果您的组织管理 Claude Code，某些设置已由组织为您决定，您在自己的文件中放入的任何内容都无法更改它们。要查看是哪些设置，请运行 `/status`：`Setting sources` 行会列出适用于您的托管来源。托管设置在这台机器上 Claude Code 运行的任何位置都会生效；[开发人员可以更改的内容](https://code.claude.com/docs/zh-CN/managed-settings.md#what-a-developer-can-change)介绍了本地管理员权限以及 Claude Code 以外的工具。

托管设置通过托管设置页面上介绍的[交付机制](https://code.claude.com/docs/zh-CN/managed-settings.md#delivery-mechanisms)送达您，最常见的是：

* [服务器托管设置](https://code.claude.com/docs/zh-CN/server-managed-settings.md)，由 Claude Code 从 claude.ai 管理控制台或自托管的 [Claude apps gateway](https://code.claude.com/docs/zh-CN/claude-apps-gateway) 获取
* MDM 或操作系统级别的策略，以及系统目录中的 `managed-settings.json` 文件
* 嵌入主机（例如 Claude Desktop），通过 SDK `managedSettings` 选项提供；请参阅[从嵌入主机控制策略](https://code.claude.com/docs/zh-CN/managed-settings.md#parent-settings-from-embedding-hosts)

在 Claude Desktop 应用中于您的机器上运行的 [Cowork](https://claude.com/docs/cowork/overview) 会话中，Claude Code 不会从 claude.ai 管理控制台获取服务器托管设置，并且会读取部署到您设备上的策略，除非您组织的 Claude Desktop 配置设置了 `requireCoworkFullVmSandbox`。[策略的生效位置和时间](https://code.claude.com/docs/zh-CN/managed-settings.md#where-and-when-a-policy-applies)介绍了 Cowork 和云端会话的情况。

如果您是管理员，[为组织设置 Claude Code](https://code.claude.com/docs/zh-CN/admin-setup.md) 将引导您选择要强制执行的内容，[部署托管设置](https://code.claude.com/docs/zh-CN/managed-settings.md)介绍了交付方式以及如何确认策略已生效。有关 claude.ai 管理控制台中托管设置编辑器可能显示的警告，请参阅[配置服务器托管设置](https://code.claude.com/docs/zh-CN/server-managed-settings.md#configure-server-managed-settings)。

## 更改设置

您可以从 `/config` 菜单、通过编辑设置文件或对一个会话从命令行更改设置。

Claude Code 的系统提示未发布。要给 Claude 常设指令，使用 [`CLAUDE.md` 文件](https://code.claude.com/docs/zh-CN/memory)或 `--append-system-prompt` 标志。

### 使用 /config 菜单

在 Claude Code 内运行 `/config` 并打开**配置**选项卡。它列出了一小组个人选项，如主题、编辑器模式和详细输出，而不是每个设置键。选择一个选项来更改它；Claude Code 为您保存它：

* **大多数选项**：`~/.claude/settings.json`
* **一些选项，如显示提示**：`.claude/settings.local.json`
* **[全局配置选项](https://code.claude.com/docs/zh-CN/settings-reference.md#global-config-settings)**：`~/.claude.json`

要设置一个选项而不使用菜单，传递 `key=value`，例如 `/config verbose=true`。

> [!NOTE]
>
> `/config` 是终端界面的一部分。[VS Code](https://code.claude.com/docs/zh-CN/vs-code.md) 聊天面板和[桌面应用](https://code.claude.com/docs/zh-CN/desktop.md)不打开它；通过编辑设置文件或通过这些应用自己的设置在那里更改设置。

### 编辑设置文件

在您的编辑器中打开您想要的作用域的设置文件并添加或更改键。设置文件是严格的 JSON：`//` 注释或尾部逗号是语法错误，Claude Code 在下次启动时将文件报告为[设置错误](#fix-a-broken-settings-file)。例如，要让 Claude Code 在不询问的情况下运行您的 lint 和测试命令并阻止它读取 `.env` 文件，将此添加到 `~/.claude/settings.json`：

```json ~/.claude/settings.json theme={null}
{
  "$schema": "https://json.schemastore.org/claude-code-settings.json",
  "permissions": {
    "allow": [
      "Bash(npm run lint)",
      "Bash(npm run test *)"
    ],
    "deny": [
      "Read(./.env)",
      "Read(./.env.*)"
    ]
  }
}
```

`permissions` 下的每个条目是一个命名工具及其可能做什么的规则；[配置权限](https://code.claude.com/docs/zh-CN/permissions.md)解释语法。`$schema` 行指向 Claude Code 设置的[已发布 JSON 架构](https://json.schemastore.org/claude-code-settings.json)，它在 VS Code、Cursor 和任何其他支持 JSON 架构的编辑器中为您提供自动完成和内联验证。架构可能滞后于最新的 CLI 版本，因此最近记录的键上的验证警告并不意味着您的配置无效。

保存后，在 Claude Code 内运行 `/status` 以确认文件已加载；[确认已加载的内容](#check-what-loaded)说明 `Setting sources` 行显示什么以及如何报告损坏的文件。

有关完整的个人文件、团队文件和组织文件，每个都带有每个键的注释，请参阅[示例设置文件](https://code.claude.com/docs/zh-CN/settings-example.md)。

### 为一个会话更改设置

要尝试一个值而不保存它，在启动 Claude Code 时设置它。该值适用于该会话，您的设置文件保持原样。您有三种方式做到：

* **`--settings`**：将键作为 JSON 传递，内联或作为文件路径。Claude Code 在您的用户、项目和本地文件上方以及托管设置下方应用它。它可以设置您的用户设置文件可以设置的任何键；它不能设置 `Managed` 或 `Global config` 键。
* **该键的标志**：某些键有自己的标志，如 `--model` 用于 `model` 和 `--effort` 用于 `effortLevel` 和 `modelSettings`。
* **环境变量**：在运行 `claude` 之前导出键的配对变量，如 `ANTHROPIC_MODEL` 用于 `model`。

每个键在[设置参考](https://code.claude.com/docs/zh-CN/settings-reference.md)上的条目列出其每个会话覆盖以及哪个优先，因此检查您想更改的键的条目。

您在会话内运行的命令大多保存您的选择：当您在 `/config` 中更改设置时，Claude Code 将其写入您的设置文件，`/model` 将值保存为您新会话的默认值。

如果您在 `/model` 选择器中按 `s`，Claude Code 切换模型而不将其保存为您的用户默认值。[调整努力级别](https://code.claude.com/docs/zh-CN/model-config.md#adjust-effort-level)说明哪些 `/effort` 选择 Claude Code 保存为您使用的模型的默认值，哪些仅适用于当前会话。

例如，要在 Opus 上启动一个会话而不更改您的默认值：

```bash theme={null}
claude --settings '{"model": "claude-opus-5-5"}'
```

### 编辑何时生效

Claude Code 监视您的设置文件并在它们更改时重新加载它们，因此它在运行的会话中应用大多数编辑而不需要重启，包括对 `permissions`、`hooks` 和凭证助手（如 `apiKeyHelper`）的编辑。Claude Code 也在会话中期加载您创建的设置文件，如果其文件夹在会话启动时存在。对于项目的 `.claude/` 文件夹，即使您在同一会话中创建文件夹，它也加载文件。

重新加载涵盖用户、项目、本地和托管设置，Claude Code 为每个它检测到的设置文件更改运行 [`ConfigChange` hook](https://code.claude.com/docs/zh-CN/hooks#configchange)，而不是来自 MDM 或 claude.ai 控制台的托管设置。来自 MDM 或 claude.ai 控制台的托管设置按计划而不是保存时到达运行的会话；[传递表](https://code.claude.com/docs/zh-CN/managed-settings.md#choose-a-delivery-mechanism)给出每个来源的。

Claude Code 仅在会话启动时读取某些键一次，因此对其中一个的编辑不会到达运行的会话。也等待重启的管理员端键，如 `requiredMinimumVersion`，在[策略适用的位置和时间](https://code.claude.com/docs/zh-CN/managed-settings.md#where-and-when-a-policy-applies)下列出。您最可能在会话中期编辑的：

* [`model`](https://code.claude.com/docs/zh-CN/settings-reference#model)：使用 [`/model`](https://code.claude.com/docs/zh-CN/model-config#setting-your-model) 在会话中期切换。每个模型有自己的提示缓存，因此切换后的第一个请求重新读取整个对话未缓存；请参阅[切换模型](https://code.claude.com/docs/zh-CN/prompt-caching.md#switching-models)
* [`effortLevel`](https://code.claude.com/docs/zh-CN/settings-reference#effortlevel) 和 [`modelSettings`](https://code.claude.com/docs/zh-CN/settings-reference#modelsettings)：使用 [`/effort`](https://code.claude.com/docs/zh-CN/model-config#adjust-effort-level) 在会话中期更改努力

### 确认已加载的内容

在 Claude Code 内运行 `/status` 以查看哪些设置来源处于活跃状态。**状态**选项卡包含一个 `Setting sources` 行，列出 Claude Code 为当前会话加载的每个设置文件，如 `User settings` 或 `Project local settings`。当[托管设置](https://code.claude.com/docs/zh-CN/admin-setup.md#decide-how-settings-reach-devices)生效时，托管设置条目在括号中显示它们如何到达您的机器。

该行确认 Claude Code 读取了哪些文件；它不显示哪个文件提供了每个键。要列出 Claude Code 拒绝的条目，运行 [`claude doctor`](https://code.claude.com/docs/zh-CN/debug-your-config)；对于项目或托管设置设置的模型，启动标头命名设置它的文件。`/status` 和 `/config` 在不同选项卡上打开相同的对话框，**配置**选项卡不是您的 `settings.json` 内容的视图。

### 修复损坏的设置文件

如果您拼错 JSON 或将键设置为 Claude Code 不接受的值，Claude Code 在交互式会话启动时告诉您。它显示的内容取决于文件受影响的程度：

* **设置错误**：用户、项目或本地文件有无效的 JSON 或架构拒绝的值。在交互式会话启动时，Claude Code 显示一个对话框，让您在 Claude 的帮助下修复文件、退出，或在不加载这些损坏设置的情况下继续。
* **设置警告**：仅单个条目失败，如格式错误的权限规则或未知的 hook 事件名称。Claude Code 跳过这些值并保持文件的其余部分生效。
* **托管设置**：Claude Code 继续强制执行文件的其余部分。[托管设置中的无效条目](https://code.claude.com/docs/zh-CN/managed-settings.md#invalid-entries-in-managed-settings)说明它删除什么以及哪些键回退到更严格的值，直到您修复它们。对于不是有效 JSON 的托管设置文档，请参阅[托管设置文档无法解析](https://code.claude.com/docs/zh-CN/errors.md#managed-settings-document-could-not-be-parsed)。
* **配置错误**：`~/.claude.json` 无法解析。Claude Code 将损坏的文件复制到 `~/.claude/backups/.claude.json.corrupted.<timestamp>` 并询问是否退出并手动修复它或重置为默认配置；`-p` 运行打印错误并退出。要恢复您之前的状态，复制回 `~/.claude/backups/` 中最近五个 `.claude.json.backup.<timestamp>` 文件之一，Claude Code 在写入文件前保存。

在您继续后，运行 `/status` 以查看受影响的文件，`claude doctor` 以查看每个错误的详情。

`-p` 运行显示无对话框。除非[托管设置文档无法解析](https://code.claude.com/docs/zh-CN/errors.md#managed-settings-document-could-not-be-parsed)，Claude Code 跳过损坏的文件或值并继续其余的，因此在忽略设置的 `-p` 运行后，运行 `claude doctor` 以查看它删除了什么。

## 设置优先级

当同一键出现在多个位置时，Claude Code 使用设置它的最高级别的值。下面的堆栈显示级别，最高在顶部；更高级别的键覆盖它在下面任何地方的相同键。

Settings precedence, highest first: managed settings, command line, project local, shared project, user. A key set at a higher level overrides the same key set lower down.

按顺序，最高优先级优先：

1. **托管设置**：您的组织部署的设置，通过 `managed-settings.json` 文件、MDM 策略或来自 claude.ai 控制台的[服务器管理设置](https://code.claude.com/docs/zh-CN/server-managed-settings.md)。您自己的设置文件或 `--settings` 中的任何内容都不会覆盖托管键，`--model` 等标志仅从您的组织允许的模型中选择。托管 [`model`](https://code.claude.com/docs/zh-CN/settings-reference#model) 是起始默认值，而不是锁定；锁定是 [`availableModels`](https://code.claude.com/docs/zh-CN/settings-reference#availablemodels) 和 [`deniedModels`](https://code.claude.com/docs/zh-CN/settings-reference#deniedmodels)。当您的组织传递多个托管来源时，[托管层内的优先级](https://code.claude.com/docs/zh-CN/managed-settings.md#precedence-within-the-managed-tier)的规则说明 Claude Code 从每个来源读取什么。
2. **命令行**：您在启动 `claude` 时使用 `--settings <file-or-json>` 传递的 JSON，仅用于该会话；请参阅[为一个会话更改设置](#change-a-setting-for-one-session)。您在此处设置的键会覆盖项目和用户设置文件中的相同键，省略的键保留这些文件中的值。其他标志（如 `--model`）为会话设置某一项内容，不属于此堆栈；[设置参考](https://code.claude.com/docs/zh-CN/settings-reference.md)上的键条目说明哪些标志会覆盖它。
3. **项目本地设置** (`.claude/settings.local.json`)：您对此项目的个人设置。
4. **共享项目设置** (`.claude/settings.json`)：您的团队检入源代码管理的设置。
5. **用户设置** (`~/.claude/settings.json`)：您对每个项目的个人设置。

环境变量不是此堆栈中的级别。当行为同时有 shell 变量和设置键时，哪个适用是按对决定的，而不是按级别：在您的 shell 中导出的 `ANTHROPIC_MODEL` 适用于任何文件中的 `model` 键，而 `ANTHROPIC_DEFAULT_MODEL` 仅当没有文件设置 `model` 时适用。[环境变量参考](https://code.claude.com/docs/zh-CN/env-vars.md#precedence)说哪些键有对以及 Claude Code 首先读取哪个。设置文件内的 `env` 块是普通键并遵循上面的级别。

对于少数安全敏感的键，Claude Code 尊重来自较低级别的更严格值而不是托管值；[托管设置优先级的例外](#exceptions-to-managed-settings-precedence)列出它们。

### 列表合并而不是覆盖

当您在多个文件中设置相同的列表键（如 `permissions.allow`）时，Claude Code 组合列表而不是选择一个，因此每个文件可以添加条目而不删除另一个文件的。四个保存模型列表或每个模型条目的键遵循自己的规则：

* [`fallbackModel`](https://code.claude.com/docs/zh-CN/settings-reference#fallbackmodel) 是一个有序链，其中位置具有意义，因此 Claude Code 从定义它的最高优先级文件获取整个值。
* [`modelPicker`](https://code.claude.com/docs/zh-CN/settings-reference#modelpicker) 保存一个有序的行列表加上替换标志，因此 Claude Code 永远不会合并来自两个来源的行。它从托管设置、`--settings` 和用户设置中定义它的最高获取整个值，并忽略项目和本地设置中的键。需要 Claude Code v2.1.242 或更高版本。
* [`availableModels`](https://code.claude.com/docs/zh-CN/settings-reference#availablemodels)：当 Claude Code 应用的托管设置定义它时，Claude Code 按原样应用该列表并忽略您在用户、项目或本地设置中添加的条目，除非嵌入 Claude Code 的应用提供自己的模型列表；请参阅[托管设置优先级的例外](#exceptions-to-managed-settings-precedence)。跨托管来源列表也永远不会合并；[Claude Code 如何组合托管来源](https://code.claude.com/docs/zh-CN/managed-settings.md#how-claude-code-combines-managed-sources)说哪个来源的列表适用。跨非托管作用域 Claude Code 照常合并数组。
* [`modelSettings`](https://code.claude.com/docs/zh-CN/settings-reference#modelsettings)：Claude Code 一次解决它一个模型，与 [`effortLevel`](https://code.claude.com/docs/zh-CN/settings-reference#effortlevel) 一起。`modelSettings` 条目说明哪个文件的值适用于模型。

### 优先级示例

当 Claude 工作时，Claude Code 在微调器下显示一行提示，如"使用 /config 更改您的默认权限模式（包括 Plan Mode）"。假设您想关闭这些提示，因此您在 `~/.claude/settings.json` 中将 [`spinnerTipsEnabled`](https://code.claude.com/docs/zh-CN/settings-reference#spinnertipsenabled) 设置为 `false`。下面的每个场景是可以打开它们的东西，以及您可以做什么。

#### 团队设置覆盖个人设置

您的团队的 `.claude/settings.json` 将其设置为 `true`。Claude Code 使用项目值，因为共享项目位于用户上方，因此您在该项目中看到提示，其他地方都没有。

您可以恢复您的值：在该项目中的 `.claude/settings.local.json` 中添加 `"spinnerTipsEnabled": false`。项目本地位于共享项目上方，因此您的会话停止显示提示，您队友的会话不改变。

#### 组织设置覆盖一切

您的组织的托管设置将其设置为 `true`。您在用户、项目或本地设置中放入的任何内容都不会关闭提示，`--settings` 也不会。托管是最高级别。

您无法恢复您的值。运行 `/status` 以查看哪个托管来源适用，并询问您的管理员策略是否应改变。

#### 命令行为一个会话覆盖您的文件

您使用 `claude --settings '{"spinnerTipsEnabled": true}'` 启动了会话。命令行位于除托管外的每个文件上方，因此该会话显示提示，即使您的文件说 `false`。

您在下一个会话上恢复您的值；`--settings` 持续一个会话并不写入任何文件。

#### 标志或环境变量设置相同的东西

某些键有命令行标志或环境变量，无论哪个文件设置它都覆盖设置值：`ANTHROPIC_MODEL` 覆盖 [`model`](https://code.claude.com/docs/zh-CN/settings-reference#model) 设置，`--model` 为一个会话覆盖两者。

您是否可以恢复您的值取决于键：取消设置变量或删除标志，并检查[设置参考](https://code.claude.com/docs/zh-CN/settings-reference.md)上的键条目和[环境变量参考](https://code.claude.com/docs/zh-CN/env-vars.md)上的变量行，了解 Claude Code 使用哪个。

### 排除不适用的设置

当您设置键而 Claude Code 不表现得好像您有时，从 `/status` 开始以查看它加载了哪些文件，然后在下面找到您的症状。[调试您的配置](https://code.claude.com/docs/zh-CN/debug-your-config.md)涵盖更广泛的检查，包括干净配置测试。

#### 您设置的值被忽略

其他东西设置相同的键，文件无法设置该值，或文件没有加载：

* **更高级别设置它。** 另一个设置文件、`--settings` 标志或托管来源在您的上方设置键；[堆栈](#settings-precedence)说哪个。标志或环境变量也可以自己覆盖键，按键决定；[设置参考](https://code.claude.com/docs/zh-CN/settings-reference.md)上的键条目说 Claude Code 使用哪个，[`env` 条目](https://code.claude.com/docs/zh-CN/settings-reference#env)涵盖托管 `env` 值与 shell 导出。
* **安全键保持其严格值。** 对于少数几个键 Claude Code 尊重任何文件的限制值，因此项目 `true` 用于 [`disableClaudeAiConnectors`](https://code.claude.com/docs/zh-CN/settings-reference#disableclaudeaiconnectors) 保持开启；请参阅[托管设置优先级的例外](#exceptions-to-managed-settings-precedence)。
* **文件无法设置该值。** [`permissions.defaultMode`](https://code.claude.com/docs/zh-CN/settings-reference#permissions-defaultmode) 值 `auto` 和 `bypassPermissions` 不从项目或本地设置生效；改为在用户或托管设置中设置它们，或为一个会话传递 `--permission-mode`。在 v2.1.257 之前，`bypassPermissions` 从任何文件生效。

  [`env`](https://code.claude.com/docs/zh-CN/settings-reference#env) 块中的遥测导出变量也不从项目或本地设置生效，除了少数关闭值。[Claude Code 在 `env` 中忽略的变量](https://code.claude.com/docs/zh-CN/settings-reference#variables-claude-code-ignores-in-env)列出变量和这些值。
* **文件损坏。** 无效的 JSON 或拒绝的值使 Claude Code 跳过文件或条目；请参阅[修复损坏的设置文件](#fix-a-broken-settings-file)。

#### 您在 Claude Code 中所做的更改在新会话中丢失

当您从 Claude Code 内保存新会话的选择时，如使用 `/model` 的默认模型，Claude Code 将其写入您的用户设置文件 `~/.claude/settings.json`。如果您无法写入该文件，例如因为另一个工具生成它或将其链接到只读副本，更改适用于当前会话并在下一个会话中消失。在生成文件的工具中设置键，或用您可以写入的文件替换文件。

如果您可以写入文件而更改仍然不持续，检查更改是否[仅用于一个会话](#change-a-setting-for-one-session)或[更高级别设置相同的键](#a-value-you-set-is-ignored)。对于 `model` 键，[新会话在与您选择的不同的模型上启动](https://code.claude.com/docs/zh-CN/model-config.md#a-new-session-starts-on-a-different-model-than-you-picked)列出更多原因。

#### 托管更改还没有到达您

托管来源按[传递表](https://code.claude.com/docs/zh-CN/managed-settings.md#choose-a-delivery-mechanism)中的计划到达运行的会话，因此首先重启会话。如果 `/status` 然后命名与您的管理员更改的不同的来源，更高优先级的来源适用；[Claude Code 如何组合托管来源](https://code.claude.com/docs/zh-CN/managed-settings.md#how-claude-code-combines-managed-sources)给出顺序。

#### 提交的键不到达队友

两件事阻止 `.claude/settings.json` 中的键为克隆它的每个人应用：

* **Claude Code 忽略仓库文件中的键。** 在[设置索引](https://code.claude.com/docs/zh-CN/settings-reference.md#settings-index)的作用域列中查找 `User, local, or managed`、`User or managed`、`User`、`Managed` 或 `Global config`。这些键永远不会从共享文件应用，除了少数几个仓库文件仍然可以关闭的。每个这些条目在其作用域行上说明。`Global config` 键仅从 `~/.claude.json` 应用。

  在 `env` 键内，遥测导出变量也永远不会从共享文件应用，除了少数关闭值；请参阅[Claude Code 在 `env` 中忽略的变量](https://code.claude.com/docs/zh-CN/settings-reference#variables-claude-code-ignores-in-env)。
* **键等待信任。** `permissions.allow` 规则、`permissions.additionalDirectories`、`extraKnownMarketplaces` 和大多数 [`env`](https://code.claude.com/docs/zh-CN/settings-reference#env) 值仅在每个队友[信任文件夹](https://code.claude.com/docs/zh-CN/permissions.md#project-allow-rules-and-workspace-trust)后应用。在那之前他们仍然看到提示并不从文件声明的市场获得插件。`deny` 和 `ask` 规则立即应用。

#### 权限规则组合方式与您预期不同

* **您在权限提示上选择了"Yes, and don't ask again"但仍然为相同的工具获得提示。** 该选择将 `allow` 规则保存到您的本地文件，本地的 `allow` 规则不优先于项目或托管文件中的 `ask` 规则；[权限规则如何组合](https://code.claude.com/docs/zh-CN/permissions.md#settings-precedence)解释顺序。在 VS Code 扩展中，批准卡让您选择目标文件，包括项目的共享文件，这改变了每个人的规则；在 CLI 中，Claude Code 仅写入您的本地文件。
* **您的组织的允许规则仍然与您的一起应用。** 这是预期的：Claude Code 跨作用域合并 [`permissions.allow`](https://code.claude.com/docs/zh-CN/settings-reference#permissions-allow)，除非您的组织设置 [`allowManagedPermissionRulesOnly`](https://code.claude.com/docs/zh-CN/settings-reference#allowmanagedpermissionrulesonly)。

### 托管设置优先级的例外

对于少数几个值限制会话的键，Claude Code 尊重来自否则无法覆盖托管设置的作用域的限制值。在此表中找到键以查看它尊重哪个值以及从哪里。

| 键 | Claude Code 尊重的值 | 注释 |
| :- | :- | :- |
| [`disableClaudeAiConnectors`](https://code.claude.com/docs/zh-CN/settings-reference#disableclaudeaiconnectors) | 来自任何作用域的 `true` | 即使托管来源设置 `false` 也被尊重 |
| [`enableArtifact`](https://code.claude.com/docs/zh-CN/settings-reference#enableartifact) | 来自任何作用域的 `false`，以及来自任何作用域的 `disableArtifact: true` | 即使托管来源设置 `true` 也被尊重；没有什么打开[Artifact 工具](https://code.claude.com/docs/zh-CN/artifacts.md#disable-artifacts)。需要 Claude Code v2.1.242 或更高版本 |
| [`isolatePeerMachines`](https://code.claude.com/docs/zh-CN/settings-reference#isolatepeermachines) | 来自任何作用域的 `true` | 即使托管来源设置 `false` 也被尊重 |
| [`permissions.blockReadsOutsideWorkingDirectories`](https://code.claude.com/docs/zh-CN/settings-reference#permissions-blockreadsoutsideworkingdirectories) | 来自任何作用域的 `true` | 即使托管来源设置 `false` 也被尊重。需要 Claude Code v2.1.257 或更高版本 |
| [`autoMode.classifyAllShell`](https://code.claude.com/docs/zh-CN/settings-reference#automode-classifyallshell) | 来自 `~/.claude/settings.json` 或 `--settings` 的 `true` | 即使托管来源设置 `false` 也被尊重 |
| [`remoteControlAtStartup`](https://code.claude.com/docs/zh-CN/settings-reference#remotecontrolatstartup) | 来自 `.claude/settings.json` 或 `.claude/settings.local.json` 的 `false` | 即使托管来源设置 `true` 也被尊重；项目或本地 `true` 被忽略 |
| [`crossSessionInbound`](https://code.claude.com/docs/zh-CN/settings-reference#crosssessioninbound) | 来自 `.claude/settings.json` 或 `.claude/settings.local.json` 的更严格值，在 `accept` \< `hold` \< `refuse` 梯形上 | 在托管、`--settings` 和用户值上被尊重；不是更严格的项目或本地值被忽略 |
| [`useAutoModeDuringPlan`](https://code.claude.com/docs/zh-CN/settings-reference#useautomodeduringplan) | 来自任何托管来源、`--settings`、`~/.claude/settings.json` 或 `.claude/settings.local.json` 的 `false` | 即使获胜的托管来源设置 `true` 也被尊重；`.claude/settings.json` 中的 `false` 被忽略 |
| [`syncClaudeAiSkills`](https://code.claude.com/docs/zh-CN/settings-reference#syncclaudeaiskills) | 来自任何托管来源、`--settings`、`~/.claude/settings.json` 或 `.claude/settings.local.json` 的 `false` | 即使获胜的托管来源设置 `true` 也被尊重；`.claude/settings.json` 中的 `false` 被忽略 |
| [`syncClaudeAiPlugins`](https://code.claude.com/docs/zh-CN/settings-reference#syncclaudeaiplugins) | 来自任何托管来源、`--settings`、`~/.claude/settings.json` 或 `.claude/settings.local.json` 的 `false` | 即使获胜的托管来源设置 `true` 也被尊重；`.claude/settings.json` 中的 `false` 被忽略 |
| [`maxEffortLevel`](https://code.claude.com/docs/zh-CN/settings-reference#maxeffortlevel) | 来自任何作用域（包括 `--settings`）的较低上限 | 即使 Claude Code 应用的托管设置设置更高的上限也被尊重；最低的上限适用。需要 Claude Code v2.1.267 或更高版本 |

运行 Claude Code 的应用并设置 [`CLAUDE_CODE_PROVIDER_MANAGED_BY_HOST`](https://code.claude.com/docs/zh-CN/env-vars) 也是例外。Claude Code 从该应用的模型配置优先于来自每个托管来源的 `model`、`fallbackModel`、`modelPicker` 和 `modelOverrides` 键，以及托管 `env` 块中的模型选择变量，如 `ANTHROPIC_MODEL` 和 `ANTHROPIC_DEFAULT_*_MODEL` 系列。Claude Code 保持托管 [`availableModels`](https://code.claude.com/docs/zh-CN/settings-reference#availablemodels) 允许列表生效，除非应用提供自己的。

## 云会话中的设置

[云会话](https://code.claude.com/docs/zh-CN/claude-code-on-the-web.md)在[云环境](https://code.claude.com/docs/zh-CN/cloud-environments.md)中运行在您的存储库的新克隆上，而不是在您的机器上。这改变了哪些设置到达它：

* **项目中的云端线程**：在[项目](https://code.claude.com/docs/zh-CN/claude-projects.md#what-threads-pick-up-from-your-repositories)中的云端线程里，无论该项目包含多少个仓库，仓库 `.claude/settings.json` 中的权限规则、hook 和 `env` 均不适用。[线程从您的仓库中获取哪些内容](https://code.claude.com/docs/zh-CN/claude-projects.md#what-threads-pick-up-from-your-repositories)列出了可改用的方式。
* **共享项目设置**（`.claude/settings.json`）：在项目线程之外只有一个仓库的会话中会被读取，因为该文件是克隆的一部分，且会话在其中启动。将设置提交到该文件中，即可在这些会话中应用。
* **包含多个仓库的共享项目设置**：在 Anthropic 托管的环境中，包含多个仓库的会话会在各克隆的上层目录启动，并且只从每个仓库的 `.claude/settings.json` 中读取 `enabledPlugins` 和 `extraKnownMarketplaces` 这两个键，而不会读取权限规则、hook、`env` 或其他键。这两个键所声明的市场和插件仍然[不会在云端会话中加载](https://code.claude.com/docs/zh-CN/cloud-environments.md#what-carries-over-from-your-setup)。对于自托管环境，请参阅[应用哪个仓库的设置](https://code.claude.com/docs/zh-CN/self-hosted-environments-configuration.md#repository-settings-in-sessions-with-several-repositories)。
* **用户和项目本地设置** (`~/.claude/settings.json` 和 `.claude/settings.local.json`)：不读取。两者都保持在您的机器上，本地文件不在克隆中。
* **托管设置**：您设备上的 `managed-settings.json` 文件或 MDM 配置文件不会到达云会话。您组织的[服务器管理设置](https://code.claude.com/docs/zh-CN/server-managed-settings.md)会；[表面覆盖](https://code.claude.com/docs/zh-CN/model-config.md#surface-coverage)列出哪些云会话接收它们。[自托管环境](https://code.claude.com/docs/zh-CN/self-hosted-environments)也读取其运行器镜像中的托管设置文件。[Claude Code 如何组合托管来源](https://code.claude.com/docs/zh-CN/managed-settings.md#how-claude-code-combines-managed-sources)说该文件何时适用。
* **`/config`**：在浏览器中的 claude.ai/code 上，它会打开您的 claude.ai **Code** 或 **Claude Code** 设置页面，而不是更改某个值。要为云端会话更改设置，请在环境上设置[环境变量](https://code.claude.com/docs/zh-CN/cloud-environments.md#set-environment-variables)。在项目线程之外只有一个仓库的会话中，您也可以改为将该键提交到该仓库的 `.claude/settings.json`。

[从您的设置中携带什么](https://code.claude.com/docs/zh-CN/cloud-environments.md#what-carries-over-from-your-setup)列出其余的：`CLAUDE.md`、skills、MCP 服务器、plugins 和凭证。

## 接下来是什么

* [所有设置](https://code.claude.com/docs/zh-CN/settings-reference.md)：每个键，以及您在哪里设置它和示例
* [示例设置文件](https://code.claude.com/docs/zh-CN/settings-example.md)：个人文件、团队文件和组织的托管文件
* [配置权限](https://code.claude.com/docs/zh-CN/permissions.md)：允许、询问和拒绝规则，以及 Claude Code 在不询问的情况下运行什么
* [环境变量](https://code.claude.com/docs/zh-CN/env-vars.md)：Claude Code 读取的变量和 `env` 块
* [调试您的配置](https://code.claude.com/docs/zh-CN/debug-your-config.md)：当设置不适用时
* [Claude 目录参考](https://code.claude.com/docs/zh-CN/claude-directory.md)：Claude Code 读取的每个文件，包括 subagents、MCP 服务器、插件和 `CLAUDE.md`
