> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# 安装和管理插件

> 从任何使用的界面上的市场安装 Claude Code 插件，选择安装范围，以及稍后更新或删除它们。

安装插件会将其 skills、agents、hooks 和 MCP servers 添加到您机器上的 Claude Code。

本页面适用于在自己的机器或账户上使用插件的任何人，无论是在终端、桌面应用、IDE 还是云会话中：它涵盖安装、选择范围、添加市场和保持插件更新。

> [!NOTE]
>
> 这些情况在其他页面上有介绍：
>
> * **您使用 claude.ai 聊天或 Cowork，而不是 Claude Code**：请参阅 [claude.ai 和 Cowork 中的插件](https://claude.com/docs/plugins/overview)
> * **Claude Code 打印了错误**：在 [Troubleshoot plugins](https://code.claude.com/docs/zh-CN/plugins/troubleshooting.md) 中找到它

从 [Install a plugin](#install-a-plugin) 开始。如果有人发送给您的安装命令的 `@` 名称不是 `claude-plugins-official`，请先 [add that marketplace](#add-a-marketplace)。

## Install a plugin

作为示例，本节安装来自 [Anthropic 官方市场](https://code.claude.com/docs/zh-CN/plugins/anthropic-marketplaces.md) 的 [`commit-commands`](https://github.com/anthropics/claude-plugins-official/tree/main/plugins/commit-commands)，它添加了用于提交、推送和打开拉取请求的命令。

相同的步骤可以安装任何其他插件：在 `commit-commands` 和 `claude-plugins-official` 出现的地方替换其名称和其市场的名称。如果该插件来自不同的市场，请先 [add the marketplace](#add-a-marketplace)。

选择运行 Claude Code 的位置的选项卡。

> [tab] Terminal
>
> 在您的项目中使用 `claude` 启动 Claude Code，然后：
>
> **1. 使用安装命令打开插件的详细信息**
>
> 使用插件的名称和市场运行 `/plugin install`。在会话中，此命令不会立即安装：它在该插件的详细信息上打开 `/plugin` 面板，以便您可以查看它并首先选择范围。
>
> ```text theme={null}
> /plugin install commit-commands@claude-plugins-official
> ```
>
> 要浏览，请运行不带插件名称的 `/plugin`：面板在 **Discover** 选项卡上打开，该选项卡列出您的市场中的插件，您可以输入搜索，然后在插件上按 **Enter** 打开其详细信息。
>
> **2. 查看插件添加的内容**
>
> 详细信息窗格显示插件的描述。它还可以显示：
>
> * **Will install**：插件添加的命令、agents、skills、hooks 和 MCP 及 LSP servers。
> * **Last updated**：为 Anthropic 官方市场中的插件显示。
> * **Context cost**：对于 Anthropic 官方市场中的插件，有两个令牌估计。**Every turn** 是插件添加到您发送的每条消息的内容，**When invoked** 是其 skills 和 agents 在 Claude 加载它们后添加的内容。当您通过命名其市场打开插件时（如第 1 步命令所做的那样）或从 **Marketplaces** 选项卡打开时，估计会出现。从 **Discover** 列表到达的详细信息窗格不显示它们。
>
> 来自本地或自定义市场的插件可以改为显示 `Components will be discovered at installation`。
>
> 插件可以运行 hooks 和 MCP servers，因此在安装前请阅读窗格。请参阅 [Plugin security and trust](https://code.claude.com/docs/zh-CN/plugins/security.md)。
>
> **3. 选择范围**
>
> 选择三个安装选项之一：
>
> * **Install for you (user scope)**：您在此机器上的每个项目中获得该插件
> * **Install for all collaborators on this repository (project scope)**：为在此存储库中工作的每个人启用它
> * **Install for you, in this repo only (local scope)**：您仅在此存储库中获得它
>
> [Choose an install scope](#choose-an-install-scope) 说明每个选项写入哪个设置文件，以及当同一插件在多个位置设置时哪个适用。
>
> 选择范围后，Claude Code 安装插件及其声明的任何依赖项，然后打印安装摘要。
>
> **4. 读取安装摘要**
>
> 摘要的最后一句告诉您插件在此会话中是否可用：
>
> * **已激活**：`Plugin is now active.` 无需重新加载。
> * **已激活，但服务器需要设置**：`Plugin is now active.` 后跟 `Its bundled MCP server needs configuration before it can start`。在您设置其选项之前，插件的[捆绑 MCP 服务器](https://code.claude.com/docs/zh-CN/plugins/components.md#include-a-packaged-mcpb-server)无法启动。在 `/plugin` 的 **Installed** 选项卡上选择该插件，然后选择 **Configure** 以设置服务器的选项。
> * **需要重新加载**：`Run /reload-plugins to apply.` 面板会关闭，Claude Code 会为您运行该重新加载。如果重新加载会[使提示缓存失效](https://code.claude.com/docs/zh-CN/prompt-caching.md#enabling-or-disabling-a-plugin)，它会发出警告，并改为让插件保持待处理状态。运行 `/reload-plugins --force` 可强制激活它，这会产生一次未缓存的请求。
> * **加载失败**：`The plugin couldn't be loaded`。在 `/plugin` 中打开 **Errors** 选项卡查看原因，然后参阅[安装后：插件不工作](https://code.claude.com/docs/zh-CN/plugins/troubleshooting.md#plugin-installed-but-not-working)。
>
> **5. 确认插件正常工作**
>
> 输入 `/`，并在插件名称下查找其 skill，形式为 `/<plugin>:<skill>`。对于 `commit-commands`，会出现 `/commit-commands:commit`。另外还有两个地方会列出该插件：
>
> * 在 `/plugin` 中打开 **Installed** 选项卡，该选项卡列出带有其范围的插件。
> * 在您的 shell 中，运行 `claude plugin list`，它打印相同的列表，带有 `Version`、`Scope` 和 `Status` 行。
>
> 如果 `/commit-commands:commit` 没有出现，请参阅 [After install: plugin not working](https://code.claude.com/docs/zh-CN/plugins/troubleshooting.md#plugin-installed-but-not-working)。
>
> 从任何其他市场安装需要先执行一个额外步骤：[add the marketplace](#add-a-marketplace)。Claude Code 在您第一次启动交互式终端会话时为您添加 Anthropic 的官方市场，这就是为什么示例跳过该步骤。如果您在 [claude.com/marketplace](https://claude.com/marketplace) 上找到了插件，其 **Claude Code** 按钮会复制其 [shell form](#install-from-your-shell) 中的安装命令，`claude plugin install <name>@claude-plugins-official`。

> [tab] Desktop app
>
> 在桌面应用的 **Code** 选项卡中的本地或 SSH 会话中：
>
> **1. 打开插件浏览器**
>
> 单击提示框旁边的 **+** 按钮，然后选择 **Add plugins**。如果 **+** 菜单显示的是 **Plugins** 而不是 **Add plugins**，请选择 **Plugins**，然后选择 **Browse plugins**。插件浏览器打开，显示来自您的市场的插件。
>
> **2. 选择插件**
>
> 找到 `commit-commands` 并选择它。
>
> **3. 选择作用域**
>
> 选择 [作用域](#choose-an-install-scope)：您的用户账户、此项目或仅本地。
>
> 要稍后启用、禁用或卸载插件，请参阅 [Install plugins in the desktop app](https://code.claude.com/docs/zh-CN/desktop.md#install-plugins)。

> [tab] VS Code
>
> 在 VS Code 中的 Claude Code 面板中：
>
> **1. 打开 Manage plugins**
>
> 在提示框中输入 `/plugins` 以打开 **Manage plugins**。
>
> **2. 安装插件**
>
> 在 **Plugins** 选项卡上，搜索 `commit-commands` 并单击 **Install**。如果选项卡未列出任何插件，请先在 **Marketplaces** 选项卡上添加 `anthropics/claude-plugins-official`。
>
> **3. 选择范围**
>
> 选择 [scope](#choose-an-install-scope)：**Install for you**、**Install for this project** 或 **Install locally**。
>
> 您的更改应用于打开的会话，无需重新启动。请参阅 [Manage plugins in VS Code](https://code.claude.com/docs/zh-CN/vs-code.md#manage-plugins)。

> [tab] Cloud session
>
> [cloud session](https://code.claude.com/docs/zh-CN/cloud-environments.md)（包括 [the browser at claude.ai/code](https://code.claude.com/docs/zh-CN/claude-code-on-the-web.md)）没有插件浏览器，不会加载您在自己的机器上安装的插件或您的存储库的 `.claude/settings.json` 打开的插件。对于您的组织通过托管设置分发的插件，请参阅 [Manage plugins for your organization](https://code.claude.com/docs/zh-CN/plugins/org.md)。
>
> 请参阅 [which parts of your setup are also available in a cloud session](https://code.claude.com/docs/zh-CN/cloud-environments.md#what-carries-over-from-your-setup) 了解您设置的其余部分。

### Choose an install scope

插件的安装范围决定了谁获得该插件以及哪个设置文件将其记录为已启用：

* **User scope**：该插件在此机器上的每个项目中为您启用。条目进入 `~/.claude/settings.json` 中的 `enabledPlugins`。
* **Project scope**：该插件为在此存储库中工作的每个人启用。条目进入 `.claude/settings.json`，您提交它。提交该条目会为您的协作者打开插件，但不会将其下载到他们的机器上，因此每个协作者也需要运行一次 `claude plugin install <name>@<marketplace> --scope project`；请参阅 [在项目设置中启用但未安装](https://code.claude.com/docs/zh-CN/plugins/loading.md#enabled-in-project-settings-but-not-installed)。
* **Local scope**：该插件仅在此存储库中为您启用。条目进入 `.claude/settings.local.json`。

某些插件由其作者通过 [`defaultEnabled`](https://code.claude.com/docs/zh-CN/plugins/manifest-reference#defaultenabled) 字段设置为默认关闭。这样的插件已安装但保持关闭，直到您在 shell 中使用 `claude plugin enable <name>` 或从会话中 `/plugin` 的 **Installed** 选项卡打开它。

当同一插件在多个范围设置时，本地设置覆盖项目设置，项目设置覆盖用户设置。请参阅 [Find where a plugin is enabled](https://code.claude.com/docs/zh-CN/plugins/loading.md#find-where-a-plugin-is-enabled) 了解完整规则。

终端、桌面应用的本地会话和一台计算机上的 VS Code 扩展读取相同的设置文件，因此您在其中任何一个中以用户范围安装的插件在其他两个中可用。

### JetBrains、非交互式运行和 Agent SDK

您运行 Claude Code 的某些地方没有自己的插件浏览器：

* **JetBrains IDEs**：JetBrains 插件在 IDE 的终端中运行 Claude Code，因此在那里使用 **Terminal** 选项卡的步骤。
* **`claude -p` 和其他非交互式运行**：`/plugin` 不运行，Claude 回复 `/plugin isn't available in this environment.` 您已安装的插件确实会加载。使用 [`claude plugin` commands](#install-from-your-shell) 从 shell 安装和管理它们。
* **Agent SDK**：通过 SDK 的插件选项加载插件。请参阅 [Load plugins in the Agent SDK](https://code.claude.com/docs/zh-CN/agent-sdk/plugins.md)。

如果 Claude Code 报告在存储库的 `.claude/settings.json` 中启用的插件未安装，请参阅 [Enabled in project settings but not installed](https://code.claude.com/docs/zh-CN/plugins/loading.md#enabled-in-project-settings-but-not-installed)。

> [!TIP]
>
> 如果您是插件作者，正在测试磁盘上的插件副本，请从 shell 使用 `--plugin-dir` 启动 Claude Code，以便为一个会话加载它，而不是安装它。请参阅 [Flags that load a plugin for one session](https://code.claude.com/docs/zh-CN/plugins/cli-reference.md#flags-that-load-a-plugin-for-one-session)。

### Plugins from your claude.ai account

您的 claude.ai 账户是插件的单独来源，与您安装的市场并列：

* **What arrives**：您为 claude.ai 账户打开的每个插件，以及您的组织为其成员打开的每个插件。在终端会话中，每次您在使用该账户登录时启动 Claude Code 时，它们在后台同步；在 Cowork 会话中，它们在会话启动时下载。
* **Where you see them**：在 `/plugin` 和 `claude plugin list` 中，ID 为 `<name>@synced`。您可以在自己的范围关闭一个，除非您的组织要求它。
* **What doesn't go the other way**：您使用 `/plugin` 或 `claude plugin install` 安装的插件保留在此机器上，不会添加到您的 claude.ai 账户。

有关同步时间、登录要求和关闭同步，请参阅 [Plugins synced from claude.ai](https://code.claude.com/docs/zh-CN/plugins/loading.md#synced-plugins)。

### Install from your shell

在 shell 中运行 `claude plugin install` 以安装插件，而无需启动 Claude Code 会话，例如从设置脚本。

* **Scope**：默认为用户范围。传递 `--scope project` 或 `--scope local` 以更改它。
* **When the plugins load**：它安装的插件在您下次启动 Claude Code 时加载，或当您在已打开的会话中运行 `/reload-plugins` 时加载。
* **The marketplace on a new machine**：在还没有人打开过交互式 Claude Code 会话的机器上，官方市场未注册，因此从它安装的脚本在安装前运行 `claude plugin marketplace add anthropics/claude-plugins-official`。请参阅[从 shell 添加和安装](#add-and-install-from-your-shell)。

```bash theme={null}
claude plugin install formatter@your-org --scope project
```

命令完成时打印 `Successfully installed plugin: formatter@your-org (scope: project)`。

某些插件通过运行其市场命名的命令来安装，称为 [`command` source](https://code.claude.com/docs/zh-CN/plugins/marketplace-reference#command-plugin-source)。Claude Code 向您显示该命令并要求您在运行前接受它。脚本没有人来回答该提示，因此在那里传递 `--yes` 以接受它。

有关其他 `claude plugin install` 标志，请参阅 [plugin install](https://code.claude.com/docs/zh-CN/plugins/cli-reference.md#plugin-install)。

## Add a marketplace

当您想要的插件不在 Anthropic 官方市场中时，您只需要本节，例如同事发布的插件或来自 Anthropic 社区市场的插件。

市场是插件的目录，Claude Code 必须了解市场才能从中安装。您添加一次市场。之后，其插件出现在 **Discover** 选项卡上，并使用 `/plugin install <plugin>@<marketplace>` 在会话中或 `claude plugin install <plugin>@<marketplace>` 在 shell 中安装，其中 `<marketplace>` 是市场注册的名称。要在一个步骤中同时执行两者，请参阅 [Add a marketplace and install in one command](#add-a-marketplace-and-install-in-one-command)。

在 Claude Code 会话中，运行 `/plugin marketplace add` 后跟市场的来源：GitHub 存储库、任何主机上的 git 存储库、本地目录或文件，或托管的 `marketplace.json`。

| Source | What you type | Example |
| :- | :- | :- |
| GitHub 仓库 | `owner/repo`。添加 `#ref` 以固定分支或标签。 | `/plugin marketplace add anthropics/claude-plugins-official`，或 `/plugin marketplace add your-org/plugins#v1.2.0` 以固定 `v1.2.0` 标签 |
| Git repository on any host | 完整的克隆 URL。添加 `#ref` 以固定分支或标签。 | `/plugin marketplace add https://gitlab.example.com/your-group/your-marketplace.git#v1.0.0` |
| Local directory or file | 相对或绝对路径到包含 `.claude-plugin/marketplace.json` 的目录，或到 JSON 文件本身。以 `./` 或 `../` 开始相对路径，因为 Claude Code 将裸 `name/name` 读取为 GitHub 存储库。 | `/plugin marketplace add ./my-marketplace` |
| Hosted `marketplace.json` | 其 `https://` URL | `/plugin marketplace add https://example.com/marketplace.json` |

从 shell，`claude plugin marketplace add` 采用相同的来源。

> [!TIP]
>
> `/plugin market` 也可作为 `/plugin marketplace` 的较短形式。

在每个 URL 上包含 `https://` 前缀，或对 SSH 使用 `git@host:path` 形式。如果您输入裸 `gitlab.example.com/your-group/your-marketplace.git`，Claude Code 将其读取为 GitHub `owner/repo` 简写并拒绝它。

命令成功时，它打印 `Successfully added marketplace: <name>`，市场的插件在下次打开 `/plugin` 时出现在 **Discover** 选项卡上，无需重新加载。如果失败，请在 [Troubleshoot plugins](https://code.claude.com/docs/zh-CN/plugins/troubleshooting.md#add-a-marketplace) 中匹配错误消息。

### Add a marketplace and install in one command

要从您尚未添加的市场安装插件，请在安装命令中使用 `--marketplace` 命名市场来源，可以在会话中或从 shell 中执行。来源采用 [the same forms as `/plugin marketplace add`](#add-a-marketplace)，例如 GitHub `owner/repo`、git URL 或本地路径。单独给出插件名称，不带 `@marketplace` 后缀。

#### Add and install in a session

在 Claude Code 会话中运行 `/plugin install`，并指定插件和来源。需要 Claude Code v2.1.275 或更高版本。在会话中，来源不能包含空格。

```text theme={null}
/plugin install deploy-helper --marketplace your-org/plugins
```

如果您尚未添加该市场，Claude Code 显示它解析的来源并要求您在添加前确认。一旦添加了市场，插件的详细信息打开，您选择 [installation scope](#install-a-plugin)。如果来源与您已添加的市场匹配，Claude Code 跳过确认并在该市场中打开插件的详细信息。

#### Add and install from your shell

在 shell 中，无需启动会话，运行 `claude plugin install` 并指定插件和来源。需要 Claude Code v2.1.292 或更高版本。

```bash theme={null}
claude plugin install deploy-helper --marketplace your-org/plugins
```

shell 命令添加市场时没有确认步骤。您已从该来源添加的市场会被重用。新市场会经过与 `claude plugin marketplace add` 相同的 [organization policy checks](https://code.claude.com/docs/zh-CN/plugins/org.md#restrict-what-users-can-install) 后添加，并且即使您传递 `--scope project`，也会在您的用户设置中声明。

如果您尚未添加该市场，该命令会打印 `Successfully added marketplace: <name> (declared in user settings)`，然后[安装插件](#install-from-your-shell)。

### Add a private marketplace

私有市场是您需要凭证才能克隆的存储库中的市场，在 GitHub 或任何其他 git 主机上。您使用与公共市场相同的 `/plugin marketplace add` 或 `claude plugin marketplace add` 命令添加它。Claude Code 使用已在您的机器上的 git 凭证克隆它，从不提示，因此每种连接方式都有要求：

* **HTTPS**：您的 git 凭据助手适用，因此您使用 `gh auth login`、macOS Keychain 或 `git-credential-store` 设置的访问权限有效。git 的提示和 askpass 程序均被关闭，包括在 `GIT_ASKPASS` 中设置的程序，因此当 git 需要询问凭据时，克隆会直接失败而不是提示。
* **SSH**：主机必须已在您的 `known_hosts` 文件中，密钥必须在没有密码短语提示的情况下工作。如果您的 git 设置在 `GIT_SSH_COMMAND`、`GIT_SSH` 或您的 git 配置的 `core.sshCommand` 中命名 SSH 程序，Claude Code 运行该程序。
* **GitHub `owner/repo` shorthand**：Claude Code 检查您的 SSH 密钥是否向 `github.com` 认证，如果认证则通过 SSH 克隆，如果不认证则通过 HTTPS 克隆。设置 [`CLAUDE_CODE_PLUGIN_PREFER_HTTPS=1`](https://code.claude.com/docs/zh-CN/env-vars#variables) 以跳过该检查并始终通过 HTTPS 克隆。

当您运行 `/plugin install`、`/plugin marketplace update` 和 `claude plugin update` 时，相同的凭证适用。

在 GitHub Enterprise Server 主机上，请参阅 [Plugin marketplaces on GHES](https://code.claude.com/docs/zh-CN/github-enterprise-server.md#plugin-marketplaces-on-ghes) 了解每个操作需要的凭证。

如果您的组织通过托管设置为您注册市场，您不需要自己添加它。请参阅 [Pre-install and require plugins](https://code.claude.com/docs/zh-CN/plugins/org.md#pre-install-and-require-plugins)。

### Add a marketplace from claude.ai

在 [plugins sync from your claude.ai account](https://code.claude.com/docs/zh-CN/plugins/loading.md#synced-plugins) 的终端会话中，claude.ai 也可以为您列出插件市场，例如您的组织的插件库和您自己的 claude.ai 上传。您通过其名称而不是来源添加其中之一。从 claude.ai 添加市场需要 Claude Code v2.1.273 或更高版本。

从 `/plugin` 面板或从 shell 添加 claude.ai 市场：

* **Inside a session**：运行 `/plugin` 并转到 **Marketplaces** 选项卡，该选项卡列出来自 claude.ai 的市场。在那里选择一个以添加它。
* **From your shell**：运行 `claude plugin marketplace list`，它在 `From claude.ai:` 部分中打印它们。然后使用 `--claudeai` 标志和列表中显示的名称运行 `claude plugin marketplace add`。

例如，此命令添加名为 `claudeai-organization-library` 的市场：

```bash theme={null}
claude plugin marketplace add --claudeai claudeai-organization-library
```

Claude Code 在以 `claudeai-` 开头的本地名称下注册市场，该名称源自 claude.ai 列出的名称。例如，列为"Organization library"的市场变为 `claudeai-organization-library`。通过该名称安装其插件，例如使用 `claude plugin install <plugin>@claudeai-organization-library`。

如果您登出或登录到不同的 claude.ai 组织，市场保持配置但不显示插件，您已从中安装的插件继续加载。

`From claude.ai:` 部分也可以列出通过 claude.ai 共享的基于 git 的市场，并为每个市场打印来源。通过该来源添加它们，如 [Add a marketplace](#add-a-marketplace) 中所示，而不是使用 `--claudeai`。

## Manage installed plugins

`/plugin` 中的 **Installed** 选项卡列出您的插件，以及启用、禁用、更新或卸载每个插件的操作。在 Claude Code 会话中，运行 `/plugin` 并按 **Tab** 到达它，或运行 `/plugin enable`、`/plugin disable` 或 `/plugin uninstall` 以打开面板并在那里进行更改。禁用的插件在列表底部的折叠标题下分组。在列表上使用这些键：

* 输入以按名称或描述过滤。
* 按 **Space** 启用或禁用所选插件，按 **f** 将其收藏。
* 按 **Enter** 打开插件的详细信息。

插件的详细信息菜单提供 **Disable plugin** 或 **Enable plugin**、**Update now** 和 **Uninstall**。采用设置的插件还会显示两个更多项目，一个插件可以同时显示两者：

* **Configure options**：当插件的清单声明 [`userConfig` 选项](https://code.claude.com/docs/zh-CN/plugins/manifest-reference#user-configuration) 时显示。打开这些选项的对话框
* **Configure**：当插件包含 [bundled MCP server](https://code.claude.com/docs/zh-CN/plugins/components.md#include-a-packaged-mcpb-server) 时显示。设置该服务器自己的 `user_config` 设置

该选项卡也可以显示 **Managed** 范围的插件。您的组织通过 [managed settings](https://code.claude.com/docs/zh-CN/settings.md#settings-files) 安装了这些，您无法在此处启用、禁用或卸载它们。

对于您的组织在 claude.ai 上要求的同步插件，请参阅 [Manage plugins synced from claude.ai](#manage-plugins-synced-from-claude-ai)。

当您关闭 `/plugin` 面板并在其中进行待处理更改时，Claude Code 为您运行 `/reload-plugins` 以应用它们。如果重新加载会 [invalidate the prompt cache](https://code.claude.com/docs/zh-CN/prompt-caching.md#enabling-or-disabling-a-plugin)，它会警告并改为保留更改待处理。运行 `/reload-plugins --force` 以应用它们。

### Manage plugins synced from claude.ai

`/plugin` 中的 **Installed** 选项卡也列出 [plugins synced from your claude.ai account](https://code.claude.com/docs/zh-CN/plugins/loading.md#synced-plugins)，其来源为 `synced`。同步插件在 Claude Code v2.1.273 或更高版本的终端会话中出现。

* **Enable or disable**：使用 **Installed** 选项卡，除非您的组织将插件标记为必需。
* **Remove**：在 claude.ai 上关闭插件。

当 Claude Code 将添加、更新或删除的插件同步到交互式会话中时，您会看到 `Plugins changed. Run /reload-plugins to activate.` 运行 `/reload-plugins` 以在该会话中加载更改，或将其留给下次启动 Claude Code 时。

### Uninstall a plugin the project enables

当您为此存储库的 `.claude/settings.json` 启用的插件选择 **Uninstall** 时，无论是从 **Installed** 选项卡还是使用 `/plugin uninstall`，Claude Code 都会询问是为您禁用它还是为所有人卸载它：

* **Disable for me**：按 **y**。Claude Code 在您的 `.claude/settings.local.json` 中为插件写入 `false` 并为项目保留它已安装。
* **Uninstall for everyone**：按 **u**。Claude Code 从共享的 `.claude/settings.json` 中删除插件。

### See what an installed plugin adds to your sessions

在 shell 中，为已安装的插件运行 `claude plugin details <name>`。`Always-on` 行是插件添加到启用它的每个会话的令牌数，每个组件行显示哪个 skill 或 agent 贡献最多。有关完整输出和每个数字的含义，请参阅 [Measure what a plugin costs](https://code.claude.com/docs/zh-CN/plugins/measure.md#measure-what-a-plugin-costs)。

### Find plugins you no longer use

在 `/plugin` 中的 **Installed** 选项卡上，您自己安装且最近未使用的插件出现在 **Not used recently** 标题下，每个插件的详细信息显示 **Last used** 行。使用该标题和该行查找仍添加启动和上下文成本的插件，然后禁用或卸载它们。

### Plugins with dependencies

插件可以声明它依赖的其他插件。当您从市场安装、禁用或卸载这样的插件时，Claude Code 也对这些依赖项进行操作：

* **Install**：Claude Code 也在相同范围安装并启用插件的声明依赖项。成功消息列出它们。
* **Enable**：Claude Code 也启用已安装但禁用的插件的依赖项。如果声明的依赖项未安装，启用失败，消息告诉您先安装它。
* **Disable**：当另一个启用的插件仍需要您命名的插件时，Claude Code 拒绝并打印以正确顺序禁用两者的链式命令。
* **Uninstall**：自动安装的依赖项保留到您在 shell 中运行 `claude plugin prune` 为止；请参阅 [plugin prune](https://code.claude.com/docs/zh-CN/plugins/cli-reference.md#plugin-prune)。

如果您改为使用 `--plugin-dir` 加载插件，请参阅 [Test a plugin and its dependency locally](https://code.claude.com/docs/zh-CN/plugins/dependencies.md#test-a-plugin-and-its-dependency-locally)。

### Manage plugins from your shell

您也可以在不启动 Claude Code 会话的情况下管理插件。在 shell 中，运行 `claude plugin install`、`enable`、`disable` 或 `uninstall` 作为普通终端命令；它们更改 `/plugin` 面板所做的相同设置。每个都采用 `--scope` 以针对一个范围，并在您省略它时使用默认范围：

* `enable` 和 `disable` 作用于其设置已列出插件的最具体范围。
* `install` 和 `uninstall` 作用于用户范围。

例如，这些命令禁用并重新启用插件，然后在项目范围卸载它：

```bash theme={null}
claude plugin disable formatter@your-org
claude plugin enable formatter@your-org
claude plugin uninstall formatter@your-org --scope project
```

## Keep plugins updated

当插件来自的市场打开了自动更新时，插件会自动更新。会话启动后，Claude Code 刷新这些市场并更新您从中安装的插件的磁盘副本。

运行的会话保持它已加载的版本。更新后，您会看到 `Plugin updated: <name> · Run /reload-plugins to apply`，下一个会话自动加载新版本。

这些是每种市场类型的自动更新默认值：

* **On by default**：`claude-plugins-official` 和其他 [official marketplace names](https://code.claude.com/docs/zh-CN/plugins/security.md#official-marketplace-names)（除了 `knowledge-work-plugins` 和 `first-party-plugins`）以及 [marketplaces added from claude.ai](#add-from-claude-ai)。
* **Off by default**：所有其他市场，包括社区市场、第三方市场和本地开发市场。

有关自动更新何时运行、它跳过哪些插件以及关闭它的环境变量，请参阅 [When auto-update runs](https://code.claude.com/docs/zh-CN/plugins/loading.md#when-auto-update-runs)。

### Turn auto-update on or off for a marketplace

在 Claude Code 会话中，运行 `/plugin` 并转到 **Marketplaces** 选项卡。选择市场，然后选择 **Enable auto-update** 或 **Disable auto-update**。

### Update plugins now

要更新单个插件，请在会话中于 `/plugin` 的 **Installed** 选项卡上打开该插件并选择 **Update now**，或在 shell 中运行 `claude plugin update <plugin>@<marketplace>`。

没有可一次性更新所有插件的命令。要一次性更新您从某个市场安装的插件，请转到 `/plugin` 中的 **Marketplaces** 选项卡，选择该市场，然后选择 **Update marketplace**。这会刷新该市场的列表，更新您从中安装的插件，并报告留待您自行更新的任何插件。具有 [`command` source](https://code.claude.com/docs/zh-CN/plugins/marketplace-reference#command-plugin-source) 的插件，或其市场条目设置了 `headersHelper` 命令的插件，不会通过这种方式更新，因此您需要在 **Installed** 选项卡上该插件的视图中更新它，或使用 `claude plugin update <plugin>@<marketplace>` 进行更新。

如果您在 shell 中运行不带名称的 `claude plugin marketplace update`，它会刷新每个市场的列表，但会将您已安装的插件保留在当前版本。

### Auto-update from a private marketplace

对于私有市场，请参阅 [What background auto-update does with credentials](https://code.claude.com/docs/zh-CN/plugins/host-marketplace.md#what-background-auto-update-does-with-credentials) 了解后台自动更新如何通过 SSH 和 HTTPS 认证，以及 [Troubleshoot plugins](https://code.claude.com/docs/zh-CN/plugins/troubleshooting.md#add-a-marketplace) 了解您在失败时看到的消息。

## Manage marketplaces

`/plugin` 中的 **Marketplaces** 选项卡列出您注册的每个市场及其来源。选择一个以浏览其插件、更新其列表、打开或关闭自动更新，或删除它。

您也可以使用命令从 shell 或会话内列出、更新和删除市场：

| Action | In your shell | Inside a session |
| :- | :- | :- |
| List marketplaces | `claude plugin marketplace list` | `/plugin marketplace list` |
| Update a marketplace's listing | `claude plugin marketplace update <name>` | `/plugin marketplace update <name>` |
| Remove a marketplace | `claude plugin marketplace remove <name>` | `/plugin marketplace remove <name>` |

当您删除市场时，Claude Code 卸载您从中安装的每个插件，并从您的设置文件中删除其 `enabledPlugins` 条目。**Marketplaces** 选项卡在要求您确认前命名这些插件。

## Next steps

* [Anthropic 的市场](https://code.claude.com/docs/zh-CN/plugins/anthropic-marketplaces.md)：官方市场和社区市场的区别以及在哪里浏览每个市场
* [Plugin loading reference](https://code.claude.com/docs/zh-CN/plugins/loading.md)：为什么插件加载、未加载或在更新后未更改
* [Plugin security and trust](https://code.claude.com/docs/zh-CN/plugins/security.md)：在从您不认识的市场安装插件前要查看的内容
* [Troubleshoot plugins](https://code.claude.com/docs/zh-CN/plugins/troubleshooting.md)：安装和市场错误消息及其修复
* [Create a plugin](https://code.claude.com/docs/zh-CN/plugins/create.md)：构建您自己的
