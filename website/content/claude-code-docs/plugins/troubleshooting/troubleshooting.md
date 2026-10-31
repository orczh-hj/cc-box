> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# 排查插件问题

> 修复 Claude Code 中的插件错误。找到您看到的确切消息，按照 /plugin 运行、安装和组织策略的阶段分组。

此页面列出了 Claude Code 插件和市场的错误消息和症状，市场是 Claude Code 安装插件的目录。每个条目都给出了原因、一个修复方法，以及修复后您会看到的内容。

如果消息中提到了插件或市场的名称，该条目会显示一个占位符，例如 `<name>`。

无论您是安装插件、构建插件、托管市场还是为组织管理插件，都可以使用此页面。

> [!NOTE]
>
> 这些情况在其他页面上有介绍：
>
> * **为什么作用域、缓存和优先级的行为方式如此**：阅读 [Plugin loading reference](https://code.claude.com/docs/zh-CN/plugins/loading.md)
> * **查找标志、字段或命令**：使用 [plugin commands reference](https://code.claude.com/docs/zh-CN/plugins/cli-reference.md)、[manifest reference](https://code.claude.com/docs/zh-CN/plugins/manifest-reference.md) 或 [marketplace reference](https://code.claude.com/docs/zh-CN/plugins/marketplace-reference.md)
> * **`hooks module not loaded` 或 `hooks module did not load` 消息**：该插件是一个 [mod](https://code.claude.com/docs/zh-CN/plugins/mods/overview.md)，请阅读 [The mod doesn't load](https://code.claude.com/docs/zh-CN/plugins/mods/troubleshoot.md#the-mod-doesn’t-load)

搜索您看到的确切消息。每条消息都列在产生它的阶段下，这不一定是您运行的命令。例如，安装可能因为市场缺失而失败，所以该消息在 [Add a marketplace](#add-a-marketplace) 下。

## 查找 `/plugin` 运行的位置

`/plugin` 是您在运行的 Claude Code 终端会话中键入的命令，它打开一个交互式面板。本节中的条目涵盖了您可以键入它但它无法运行的地方，以及不存在的命令拼写。

### `/plugin isn't available in this environment`

您在 Claude Code 终端会话之外的某个地方键入了 `/plugin`，Claude 用这一行回复而不是打开任何东西。

您会在没有终端来绘制 `/plugin` 面板的会话中收到此回复：[非交互模式](https://code.claude.com/docs/zh-CN/headless.md)，使用 `claude -p`、Agent SDK、Claude 桌面应用的 Code 选项卡、VS Code 扩展面板和浏览器上的 claude.ai/code。

在 VS Code 扩展面板中，只有 `/plugin` 行后面跟着内容（例如 `/plugin install <plugin>@<marketplace>`）才会收到此回复。单独键入 `/plugin` 或 `/plugins` 会打开 **Manage plugins** 对话框。

从您所在的表面安装插件：

* **Claude 桌面应用、本地或 SSH 会话**：使用 [安装插件](https://code.claude.com/docs/zh-CN/plugins/install.md#install-a-plugin) 下的 **Desktop app** 选项卡
* **VS Code 扩展**：使用 [安装插件](https://code.claude.com/docs/zh-CN/plugins/install.md#install-a-plugin) 下的 **VS Code** 选项卡
* **网络上的 Claude Code 或桌面云会话**：云会话没有插件浏览器。有关云会话加载的内容，请参阅 [安装插件](https://code.claude.com/docs/zh-CN/plugins/install.md#install-a-plugin) 下的 **Cloud session** 选项卡
* **您有权访问的终端**：运行 `claude` 并在那里键入 `/plugin`，或在您的 shell 中运行 `claude plugin install <plugin>@<marketplace>` 而不启动会话

当终端安装成功时，`/plugin` 会打印一个以 `✓ Installed <plugin>.` 开头的安装摘要，`claude plugin install` 会打印 `Successfully installed plugin: <plugin>@<marketplace>`。

### `zsh: no such file or directory: /plugin`

您在 shell 提示符处键入了 `/plugin ...`，shell 报告不存在名为 `/plugin` 的文件。Bash 报告 `bash: /plugin: No such file or directory`。

`/plugin` 是您在 Claude Code 会话中键入的命令，而不是在 shell 提示符处。启动一个会话并在那里键入相同的命令：

```shell theme={null}
claude
```

然后，在 Claude Code 提示符处：

```text theme={null}
/plugin install <plugin>@<marketplace>
```

成功的安装会打印一个以 `✓ Installed <plugin>.` 开头的摘要。如果安装本身随后失败，其消息在 [添加市场](#add-a-marketplace) 或 [安装插件](#install-a-plugin) 下。

要从 shell 安装而不启动会话，请改为运行 `claude plugin install <plugin>@<marketplace>`。

### `The term '/plugin' is not recognized as the name of a cmdlet`

您在 PowerShell 提示符处键入了 `/plugin ...`，`/plugin` 是 Claude Code 命令，而不是程序。Bash 和 Zsh 报告 [它们自己的这个错误形式](#zsh-no-such-file-or-directory-plugin)。

改为使用以下任一方式：

* 运行 `claude`，然后在 Claude Code 提示符处键入 `/plugin`
* 在 PowerShell 中运行 `claude plugin install <plugin>@<marketplace>` 而不启动会话

### `claude: command not found` after `claude plugin ...`

您在 shell 中运行了 `claude plugin install ...`，shell 根本找不到 `claude`。在 Windows 上，消息是 `'claude' is not recognized as the name of a cmdlet` 或 `'claude' is not recognized as an internal or external command`。

原因不是插件命令。请按照 [验证您的 PATH](https://code.claude.com/docs/zh-CN/troubleshoot-install.md#verify-your-path) 进行操作，然后重试插件命令。

### `Unknown command` 和不存在的命令拼写

您键入了在某处看到的插件命令，并在会话中收到 `Unknown command: /<name>`，或从 shell 中的 `claude` 二进制文件收到 `error: unknown command '<name>'` 或 `error: unknown option '<flag>'`。

有几种命令拼写在使用中，但 Claude Code 没有。下表将每一个映射到真实命令。[插件命令参考](https://code.claude.com/docs/zh-CN/plugins/cli-reference.md) 列出了各个子命令及其标志。

| 您键入的 | Claude Code 说什么 | 改为使用 |
| :- | :- | :- |
| `claude plugin add <source>` | `error: unknown command 'add'` | `claude plugin marketplace add <source>` 添加市场，或 `claude plugin install <plugin>@<marketplace>` 安装插件 |
| `claude plugin install <plugin> --project` | `error: unknown option '--project'` | `claude plugin install <plugin>@<marketplace> --scope project` |
| `/install <plugin>` | `Unknown command: /install` | `/plugin install <plugin>@<marketplace>` |
| `/plugin add <source>` | `/plugin` 面板在 **Discover** 选项卡上打开 | `/plugin marketplace add <source>` |
| `marketplace.anthropic.com` 作为源 | `Invalid marketplace source format. Try: owner/repo, https://..., or ./path` | `anthropics/claude-plugins-official` 用于官方市场 |

这些拼写看起来不对但有效：

* `claude plugins` 是 `claude plugin` 的别名
* `claude plugin remove` 是 `claude plugin uninstall` 的别名
* `/plugins` 和 `/marketplace` 在会话中打开与 `/plugin` 相同的面板

## 添加市场

市场是您从 git 存储库、URL 或本地路径添加到 Claude Code 的目录。这些条目涵盖了添加失败或稍后刷新失败时您收到的消息。

### `Marketplace "claude-plugins-official" not found`

您在会话中运行了 `/plugin install <plugin>@claude-plugins-official`，Claude Code 报告它没有该名称的市场。

官方市场在此机器上尚未注册。Claude Code 通常在您第一次启动交互式终端会话时自动注册它。如果您仅通过 VS Code 扩展使用 Claude Code，它还没有运行，并且它会跳过或延迟该步骤：

* 当策略阻止源时
* 当设置了 `CLAUDE_CODE_DISABLE_OFFICIAL_MARKETPLACE_AUTOINSTALL` 时
* 在等待重试的失败尝试之后

`claude plugin` shell 命令永远不会为您注册它。

添加它，然后重试安装：

```text theme={null}
/plugin marketplace add anthropics/claude-plugins-official
```

Claude Code 打印 `Successfully added marketplace: claude-plugins-official`，`/plugin marketplace list` 显示带有其源的市场。

对于此消息中的任何其他市场名称，请参阅 [`Marketplace "<name>" not found`](#marketplace-not-found)。

相同的字符串也出现在 `/plugin` **Errors** 选项卡中，即面板的加载失败列表，当您的设置中列出的插件命名您未添加的市场时。

### `Marketplace "<name>" not found`

您在会话中运行了 `/plugin install`，Claude Code 报告它没有该名称的市场。两种形式的命令会到达此消息：

* **`/plugin install <plugin>@<name>`**：安装行，通常是某人发送给您的，命名了您未添加的市场。本条目的其余部分涵盖了查找和添加它。
* **`/plugin install <source>` 带有路径、URL 或 `owner/repo`**：此形式报告消息而不是安装，即使对于您已添加的源也是如此。要在一个命令中从源安装，请参阅 [添加市场并在一个命令中安装](https://code.claude.com/docs/zh-CN/plugins/install.md#add-a-marketplace-and-install-in-one-command)。

如果名称以 `claudeai-` 开头，市场托管在 claude.ai 上，您可以从 shell 中使用 `claude plugin marketplace add --claudeai <name>` 按名称添加它。请参阅 [从 claude.ai 添加市场](https://code.claude.com/docs/zh-CN/plugins/install.md#add-from-claude-ai)。

对于任何其他名称，安装行命名市场但不说明市场托管在哪里，Claude Code 没有索引来查找市场名称。询问发送该行的人市场的源，这是 GitHub `owner/repo`、git URL 或路径。然后 [添加市场](https://code.claude.com/docs/zh-CN/plugins/install.md#add-a-marketplace) 并再次运行安装行。

某人发送给您的市场是第三方的，所以 [在安装前审查插件](https://code.claude.com/docs/zh-CN/plugins/security.md#review-a-plugin-before-you-install)。

如果您已经添加了市场，请根据 `/plugin marketplace list` 检查拼写。

### `Invalid marketplace source format`

您运行了 `/plugin marketplace add <source>`、`claude plugin marketplace add <source>` 或 `claude plugin install <plugin> --marketplace <source>`，Claude Code 回复 `Invalid marketplace source format. Try: owner/repo, https://..., or ./path`。

Claude Code 接受以下形式之一的源：

* GitHub `owner/repo` 简写
* `https://` 或 `http://` URL
* `user@host:path` SSH URL
* 以 `./`、`../`、`/` 或 `~` 开头的本地路径

裸名称（例如 `claude-plugins-official`）不匹配任何一个。裸主机名（例如 `marketplace.anthropic.com`）也不匹配。

以接受的形式之一重新键入源：

```text theme={null}
/plugin marketplace add anthropics/claude-plugins-official
```

当添加成功时，Claude Code 打印 `Successfully added marketplace: <name>`。

### `'<source>' is not a valid GitHub owner/repo shorthand`

您传递了一个包含斜杠但不是 `owner/repo` 的源，例如 `github.com/owner/repo` 或 `gitlab.example.com/group/project` 路径。Claude Code 拒绝了它，并显示了一个接受的形式列表。

`owner/repo` 简写仅限于 GitHub，必须遵循 GitHub 的命名规则，因此主机名或额外的路径段会失败。以与市场托管位置匹配的形式传递源：

* **任何主机上的存储库**：完整的克隆 URL
* **托管的 `marketplace.json`**：其 `https://` URL
* **本地检出**：`./path` 或绝对路径

例如，要通过其克隆 URL 添加官方市场，请在会话中：

```text theme={null}
/plugin marketplace add https://github.com/anthropics/claude-plugins-official.git
```

成功的添加打印 `Successfully added marketplace: <name>`。

### `Invalid git URL`

您添加了市场、安装了插件或从 git 地址运行了更新，命令失败，消息中显示 `Invalid git URL`。

Claude Code 在运行 git 之前检查每个 git 地址。它拒绝其协议不支持的地址。它也拒绝 git 可能读取为命名不同服务器或文件夹的地址。

地址后面的文本命名要更改的内容。按照消息说的重写地址并再次运行命令。

拒绝消息改为说 `is blocked by enterprise policy` 来自您组织的设置。请参阅 [市场源被企业策略阻止](#marketplace-source-is-blocked-by-enterprise-policy)。

### `Path does not exist: <path>`

您将本地路径传递给 `marketplace add`，该路径处没有任何内容。相对路径相对于您的当前目录解析。

检查消息中的已解析路径。然后从相对路径开始的目录运行命令，或将绝对路径传递给市场目录。成功的添加打印 `Successfully added marketplace: <name>`。

Claude Code 接受包含 `.claude-plugin/marketplace.json` 的目录，或指向 `.json` 文件的路径。指向任何其他文件的路径失败，显示 `File path must point to a .json file (marketplace.json)`。

### `Marketplace file not found at <path>/.claude-plugin/marketplace.json`

Claude Code 克隆或下载了市场，但在其内部的预期路径中找不到 `marketplace.json`。添加命令将其报告为 `Failed to add marketplace: Marketplace file not found at ...`。

默认位置是存储库根目录中的 `.claude-plugin/marketplace.json`，[市场参考](https://code.claude.com/docs/zh-CN/plugins/marketplace-reference.md) 列出了接受的位置。

修复因所有者和其他人而异：

* **您拥有市场**：将文件放在该位置并重新添加市场
* **其他人托管它**：向所有者询问他们发布的确切源

### `Cannot add marketplace "<name>": Claude Code cannot install plugins from a marketplace with this name`

您添加了一个市场，但其 `marketplace.json` 中的 [`name`](https://code.claude.com/docs/zh-CN/plugins/marketplace-reference#top-level-fields) 不能作为 [插件 ID](https://code.claude.com/docs/zh-CN/plugins/loading.md#find-where-a-plugin-came-from)（例如 `my-plugin@my-marketplace`）中 `@` 之后的部分。Claude Code 拒绝添加，且不注册任何内容。

消息的其余部分说明了名称的规则。在此示例中，`_internal` 以 `_` 开头，违反了该规则：

```text theme={null}
Cannot add marketplace "_internal": Claude Code cannot install plugins from a marketplace with this name. Each part of a plugin id (plugin@marketplace) may use only the letters a-z and A-Z, digits, ".", "_" and "-", and must start with a letter or digit. The name is set by "name" in the marketplace's marketplace.json; ask its maintainer to change it.
```

为市场指定符合该规则的名称，然后再次添加：

* **您拥有市场**：更改 `marketplace.json` 中的 `name`，例如改为 `internal-tools`
* **其他人托管它**：请所有者更改名称

在 v2.1.295 之前，Claude Code 会将此示例中的添加报告为成功。

### `Cannot add marketplace "<name>": Claude Code reserves this name and cannot register a marketplace under it`

您添加了一个市场，但其 `marketplace.json` 中的 [`name`](https://code.claude.com/docs/zh-CN/plugins/marketplace-reference#top-level-fields) 是每个 JavaScript 对象都具有的成员名称之一，例如 `constructor`、`toString` 或 `valueOf`。Claude Code 保留这些名称，因此它拒绝添加，且不注册任何内容。[保留名称](https://code.claude.com/docs/zh-CN/plugins/marketplace-reference.md#reserved-names) 列出了这些名称。

在此示例中，市场名为 `constructor`：

```text theme={null}
Cannot add marketplace "constructor": Claude Code reserves this name and cannot register a marketplace under it. The name is set by "name" in the marketplace's marketplace.json; ask its maintainer to change it.
```

如果设置文件在 [`extraKnownMarketplaces`](https://code.claude.com/docs/zh-CN/settings-reference#extraknownmarketplaces) 下声明了该市场，Claude Code 在启动时尝试添加它也会以相同方式失败，该消息会显示在 `/plugin` 的 **Errors** 选项卡中。

为市场指定另一个名称，然后再次添加：

* **您拥有市场**：更改 `marketplace.json` 中的 `name`
* **其他人托管它**：请所有者更改名称

在 v2.1.296 之前，添加此类市场会因内部错误而失败，而不是显示此消息。

### `SSH authentication failed` 或 `HTTPS authentication failed`

您从 git 存储库添加或更新了市场，克隆失败，显示 `Failed to clone marketplace repository:` 后跟以下行之一。

首先检查存储库本身：拼写错误的 `owner/repo`、不存在的存储库或您看不到的私有存储库也以此消息结尾。在浏览器中打开存储库 URL，或在终端中运行 `git ls-remote <url>`，以确认它存在且您有权访问。

如果仓库是正确的，原因是凭据。Claude Code 会关闭 git 的提示和 askpass 程序（包括在 `GIT_ASKPASS` 中设置的程序），并以批处理模式运行 SSH。当 git 需要提示时，克隆的错误中会包含 `fatal: Cannot prompt because user interactivity has been disabled`、`terminal prompts disabled` 或 `fatal: unable to get password from user`。只有 git 无需提示或 askpass 程序即可获取的凭据才会成功：

* **SSH**：`ssh -T git@<host>` 必须成功而不提示密码，主机必须已在 `known_hosts` 中
* **HTTPS**：您的凭据助手必须为主机保存令牌。对于 GitHub，运行 `gh auth login` 和 `gh auth setup-git`。对于另一个主机，在您的 git 凭据助手中存储个人访问令牌。要进行测试，请在终端中取消设置 `GIT_ASKPASS` 并运行 `git -c core.askPass= ls-remote <url>`。

一旦针对您所用协议的检查成功且未出现提示，再次运行添加或更新。成功的添加打印 `Successfully added marketplace: <name>`。成功的更新从您的 shell 打印 `Successfully updated marketplace: <name>`，或在会话中打印 `✔ Updated 1 marketplace`。

要使 Claude Code 为 GitHub `owner/repo` 源跳过 SSH，请设置 `CLAUDE_CODE_PLUGIN_PREFER_HTTPS=1`。没有它，当 `github.com` 的 SSH 密钥看起来已配置时，Claude Code 通过 SSH 克隆这些源，当 SSH 克隆失败时回退到 HTTPS。

有关后台自动更新可以和不能对您的凭证做什么，请参阅 [后台自动更新对凭证的处理](https://code.claude.com/docs/zh-CN/plugins/host-marketplace.md#what-background-auto-update-does-with-credentials)。

### `SSH host key is not in your known_hosts file`

您从您从未连接过的主机通过 SSH 添加了市场，克隆失败，显示此行和 `ssh -T git@<host>` 提示。对于密钥已更改的主机，消息是 `SSH host key has changed`，带有 `ssh-keygen -R <host>` 提示。

Claude Code 使用 `StrictHostKeyChecking=yes` 克隆，因此它拒绝您尚未接受其密钥的主机，而不是自动接受密钥。从您的终端连接一次以接受指纹，然后重试：

```shell theme={null}
ssh -T git@github.com
```

对于公共存储库，改为通过其 `https://` URL 添加市场以完全避免 SSH。

### `Command 'git' not found or is in an unsafe location`

在 Windows 上，您添加了市场，Claude Code 报告 `Failed to clone marketplace repository: Command 'git' not found or is in an unsafe location (current directory)`。

Claude Code 在您的 `PATH` 上查找 `git`，并拒绝运行仅在当前目录中找到的。要修复它，请安装 Git 并重试：

**1. 安装 Git for Windows**

安装 Git for Windows，以便 `git` 在您的 `PATH` 上。

**2. 打开新终端**

打开新终端，以便应用更新的 `PATH`。

**3. 确认 git 运行**

确认 `git --version` 打印版本。

**4. 重试添加**

再次运行 `marketplace add` 命令。

### `Git clone timed out after 120s`

您添加或更新了市场，它失败，显示 `Git clone timed out after 120s`，后跟设置 `CLAUDE_CODE_PLUGIN_GIT_TIMEOUT_MS` 的提示。

克隆市场和重新克隆一个以更新它，默认获得 120 秒。对于大型存储库或缓慢的连接，提高限制。该值以毫秒为单位：

> [tab] Bash 或 Zsh
>
> ```bash theme={null}
> export CLAUDE_CODE_PLUGIN_GIT_TIMEOUT_MS=300000
> ```

> [tab] PowerShell
>
> ```powershell theme={null}
> $env:CLAUDE_CODE_PLUGIN_GIT_TIMEOUT_MS = "300000"
> ```

然后在同一 shell 中重试。

如果存储库是 monorepo，使用 `claude plugin marketplace add <source> --sparse <paths>` 限制检出到您命名的目录。

如果仓库包含大型 git 子模块，请更新 Claude Code。在 v2.1.295 及更高版本中，Claude Code 仅获取[包含插件文件的子模块](https://code.claude.com/docs/zh-CN/plugins/marketplace-reference.md#git-submodules-in-the-marketplace-repository)。早期版本会获取所有子模块，仅此一项就可能耗尽这 120 秒。

### 市场更新在离线时持续失败

您在市场的 git 主机无法访问的环境中工作，每个会话都在后台重复失败的刷新。您现有的市场检出保持原位，启动不会延迟。

每个会话，对于 [启用自动更新](https://code.claude.com/docs/zh-CN/plugins/loading.md#which-marketplaces-and-plugins-auto-update) 的市场，Claude Code 在后台检查市场的 git 主机是否有新提交。当该检查无法到达主机时，它尝试再次克隆市场，离线时该克隆也失败。

设置此变量以跳过重新克隆尝试，并在检查无法到达主机时继续使用现有检出：

> [tab] Bash 或 Zsh
>
> ```bash theme={null}
> export CLAUDE_CODE_PLUGIN_KEEP_MARKETPLACE_ON_FAILURE=1
> ```

> [tab] PowerShell
>
> ```powershell theme={null}
> $env:CLAUDE_CODE_PLUGIN_KEEP_MARKETPLACE_ON_FAILURE = "1"
> ```

设置变量后，Claude Code 仅对已包含 `.claude-plugin/marketplace.json` 的检出跳过重新克隆。从未克隆或其克隆停止中途的市场仍然获得克隆尝试，因此请在在线时添加一次。

对于完全离线部署，改为在镜像构建时使用 `CLAUDE_CODE_PLUGIN_SEED_DIR` 预填充插件目录，遵循 [种子容器和 CI](https://code.claude.com/docs/zh-CN/plugins/org.md#seed-containers-and-ci)。

### 市场添加在 GitHub Enterprise Server 主机上失败

您从 GitHub Enterprise Server (GHES) URL 添加了市场并收到策略错误，或您从 claude.ai 添加了它并收到 GitHub 访问错误。

两种情况都在 GHES 页面上：

* [策略错误](https://code.claude.com/docs/zh-CN/github-enterprise-server.md#marketplace-add-fails-with-a-policy-error) 意味着您的组织限制了市场源，管理员需要为主机添加 `hostPattern`
* [claude.ai 上的 GitHub 访问错误](https://code.claude.com/docs/zh-CN/github-enterprise-server.md#marketplace-add-on-claude-ai-fails-with-a-github-access-error) 意味着您自己的 GitHub Enterprise 帐户尚未连接

## 安装插件

您添加了市场并运行了安装，安装停止并显示消息而不是安装任何东西。这些条目涵盖了这些消息。它们还涵盖了稍后出现在 `/plugin` **Errors** 选项卡中的相关消息，或当插件或其市场无法找到、读取或信任时的空 **Discover** 选项卡。

### `Plugin "<name>" not found in marketplace "<marketplace>"`

您运行了 `/plugin install <name>@<marketplace>` 或 `claude plugin install <name>@<marketplace>`，插件名称不在您机器上该市场目录的副本中。

当您根本没有添加市场时，`claude plugin install` 在您的 shell 中打印相同的消息。如果 `claude plugin marketplace update <marketplace>` 然后回答 `Marketplace '<marketplace>' not found`，[首先添加市场](#add-a-marketplace)。

#### `not found in marketplace` 带有刷新提示

提示读取 `Your local copy may be out of date — try claude plugin marketplace update <marketplace>` 或 `The marketplace couldn't be refreshed (...)`。Claude Code 在查找前没有刷新市场，例如当您离线时，所以您的目录副本可能已过时。使用市场的名称刷新，然后再次安装：

```text theme={null}
/plugin marketplace update <marketplace>
```

`claude plugin marketplace update` 打印 `Successfully updated marketplace: <name>`，`/plugin marketplace update` 显示 `✔ Updated 1 marketplace`。如果重试的安装打印相同的消息，请按照 [`not found in marketplace` 无提示](#the-message-has-no-hint) 描述检查名称。[Claude Code 何时在安装前刷新市场](https://code.claude.com/docs/zh-CN/plugins/loading.md#when-claude-code-refreshes-a-marketplace-before-an-install) 列出了刷新不运行的其他情况。

#### `not found in marketplace` 无提示

名称是最可能的问题。打开 `/plugin`，转到 **Discover**，并从列表中复制名称。

在 v2.1.232 之前，Claude Code 仅在查找失败后刷新命名的市场，并且仅当为其启用了自动更新时。

### `Plugin "<name>" not found in any marketplace`

您运行了 `/plugin install <name>` 而没有 `@marketplace`，没有注册的市场拥有该插件。`claude plugin install <name>` 报告 `Plugin "<name>" not found in any configured marketplace`。

没有市场名称，`claude plugin install` 搜索它已有的目录，不会首先刷新它们，`/plugin install` 仅刷新启用了自动更新的市场。命名市场，Claude Code 在查找插件前刷新它：

```text theme={null}
/plugin install <name>@<marketplace>
```

当安装成功时，您在会话中看到 `✓ Installed <plugin>.`，或从 `claude plugin install` 看到 `Successfully installed plugin: <plugin>@<marketplace>`。

如果您不知道哪个市场列出了插件，请运行 `/plugin marketplace list` 查看您拥有的市场，并在 `/plugin` 中浏览 **Discover** 查找插件名称。

### `Plugin '<name>@<marketplace>' is already installed globally`

您为已在用户作用域或通过托管设置安装的插件运行了 `/plugin install`，Claude Code 拒绝了 `Use '/plugin' to manage existing plugins.`。如果您键入了没有 `@<marketplace>` 的插件名称，消息会省略 `globally`。

插件已在每个项目中可用，因此没有什么可添加的。要更改其 [作用域](https://code.claude.com/docs/zh-CN/plugins/install.md)、启用或禁用它，或配置它，请打开 `/plugin` 并转到 **Installed**。

仅在项目或本地作用域安装的插件不会触发此消息。Claude Code 允许您也在用户作用域安装它，因此它在其他项目中可用。

您的 shell 中的 `claude plugin install` 打印不同的消息。对于已在目标作用域安装的插件，它打印 `Plugin "<name>@<marketplace>" is already installed (scope: user)` 并以 0 退出。如果其缓存目录缺失，相同的命令重新下载它。

### `"<plugin>" was not installed: it would share its folder with "<other>"`

您通过 `claude plugin install`、`/plugin` 或会话中的安装建议安装了插件，Claude Code 拒绝了它，显示此行或 `would share its saved data with`。

被拒绝的插件的 id 和已安装的插件的 id 映射到磁盘上的同一文件夹：一旦 `.` 和 `@` 被写成 `-`，它们就是相同的。在 macOS 和 Windows 上，仅在大小写上不同的 id 也映射到同一文件夹。安装两者都会将一个插件的文件放在另一个的文件夹中，所以 Claude Code 拒绝并且已安装的插件保留其文件。

消息命名了出路：

* **其他插件已安装**：消息说 `Only one of the two can be installed.` 并命名 `claude plugin uninstall` 命令，或 `/plugin` 中的卸载步骤，删除其他插件。运行它，然后再次安装。对于卸载删除和保留的内容，请参阅 [卸载删除和保留的内容](https://code.claude.com/docs/zh-CN/plugins/cli-reference.md#what-an-uninstall-deletes-and-keeps)。
* **两个 id 在一次安装中到达**，例如插件及其需要的依赖项：没有安装顺序有帮助。只有列出这两个插件的市场的维护者可以通过重命名其中一个来修复它。当两者来自不同的市场时，任一个的维护者都可以。

### `This plugin uses a source type your Claude Code version does not support`

您安装了一个插件，其市场条目使用此版本 Claude Code 无法获取的源类型，Claude Code 停止并显示此消息和 `Update Claude Code and try again.`

更新 Claude Code，然后重试安装。源类型在 [市场参考](https://code.claude.com/docs/zh-CN/plugins/marketplace-reference.md) 上。

### `Plugin archive integrity check failed`

您安装了作为 zip 存档分发的插件，Claude Code 拒绝了它，显示此行和 `The archive was not installed.`。插件的市场条目使用带有 `sha256` 引脚的 [`archive` 源](https://code.claude.com/docs/zh-CN/plugins/marketplace-reference)，下载文件的摘要与引脚不匹配。

完整消息如下所示：

```text theme={null}
Plugin archive integrity check failed for https://artifacts.example.com/claude-plugins/my-plugin.zip: expected sha256 6bfa50e3d2e00c052b46abe51fff89346ac803e45771f76dcf6df1ab74cca5e1, got ac52220c0914ef8ca6a602e4a7362f88d30fb021110f72a6d15b68c3fe7df2b7. The archive was not installed. Verify the sha256 in the marketplace entry, or that the URL serves the intended file.
```

修复因发布者和安装程序而异：

* **您发布插件**：重新计算 URL 提供的确切文件的摘要，并更新市场条目中的 `sha256`。使用 `shasum -a 256 my-plugin.zip`，或在 PowerShell 中使用 `Get-FileHash -Algorithm SHA256 my-plugin.zip`
* **您安装插件**：在会话中运行 `/plugin marketplace update <name>` 以刷新目录以防条目已更正，然后重试安装。如果刷新后摘要仍然不同，请在安装前询问市场所有者他们引脚了哪个文件

### `An npm plugin source must name a registry package`

市场条目使用 [`npm` 源](https://code.claude.com/docs/zh-CN/plugins/marketplace-reference#npm-plugin-source) 的插件安装、更新或加载失败，且消息中包含这句话。Claude Code 在获取任何内容之前检查了该条目的 `package` 值并拒绝了它。消息会指出该值和原因：

```text theme={null}
"github:acme/formatter" was not installed: it is not an http or https link. An npm plugin source must name a registry package (name or name@version) or link to a tarball file. For a plugin in a git repository, use a "github", "url" or "git-subdir" source.
```

市场的所有者必须更改该条目：

* **如果那是您**：将 `package` 更改为 [npm 插件源参考](https://code.claude.com/docs/zh-CN/plugins/marketplace-reference.md#npm-plugin-source) 接受的值，或将该条目切换为 `github`、`url` 或 `git-subdir` 源
* **如果不是您**：向市场所有者报告消息

### `Marketplace "<name>" is registered from an untrusted source`

您之前添加的市场停止加载，其插件也停止加载。此行出现在 `/plugin` **Errors** 选项卡中或下一次刷新时。

市场以 [为官方 Anthropic 市场保留](https://code.claude.com/docs/zh-CN/plugins/marketplace-reference.md) 的名称注册，但其注册源不是 `anthropics` GitHub 仓库。每次市场加载或刷新时都会重新检查保留名称，因此市场和从它安装的插件停止加载。

完整消息命名保留名称和修复：

```text theme={null}
Marketplace "claude-community" is registered from an untrusted source: The name 'claude-community' is reserved for official Anthropic marketplaces. Only repositories from 'github.com/anthropics/' can use this name. To fix it, remove the marketplace and re-add it from the official source.
```

修复因用户和发布者而异：

* **您使用市场**：在您的 shell 中，运行 `claude plugin marketplace remove <name>`，然后从官方 `github.com/anthropics` 仓库再次添加市场
* **您发布在其名称成为保留之前使用该名称的第三方市场**：重命名它并要求用户从您的源重新添加它

在 v2.1.205 之前，Claude Code 仅在您添加市场时检查名称，因此在其名称成为保留之前注册的条目继续加载。

### `Marketplace "<name>" is added but ignored`

该市场在 `~/.claude/plugins/known_marketplaces.json` 中有条目，但该条目未通过 Claude Code 每次读取该文件时运行的检查，因此该市场以及从它安装的插件停止加载。在您的 shell 中，`claude plugin list` 会为每个受影响的插件报告一行，指出原因和修复方法：

```text theme={null}
Marketplace team-tools is added but ignored. Its location is on a network drive, has "." or ".." in its path, or couldn't be checked. Re-add the marketplace (one added from a folder or file must be re-added from a copy on this computer), or, to trust a folder on a network drive, declare it under extraKnownMarketplaces in user or managed settings.
```

在会话中，`/plugin` **Errors** 选项卡会将市场名称放在引号中，在原因之后结束该行，并在其下一行显示修复方法。

`is added but ignored` 之后的句子指出该条目未通过的检查：

* `Its location is on a network drive, has "." or ".." in its path, or couldn't be checked`，或关于 `The folder or file it was added from` 的相同句子：市场的目录或添加它时所用的本地路径位于网络位置、路径中包含 `.` 或 `..` 段，或无法检查
* `Its git URL can't be used: <reason>` 或 `Its URL can't be read as an https:// or http:// address`：条目记录的源 URL 是 Claude Code 拒绝从中克隆或获取的 URL
* `Its source doesn't match its extraKnownMarketplaces entry in user or managed settings`：该条目与同名的 [`extraKnownMarketplaces`](https://code.claude.com/docs/zh-CN/settings-reference#extraknownmarketplaces) 声明不匹配

当 `(see the debug log)` 代替原因出现在 `is added but ignored` 之后时，Claude Code 拒绝了该市场的名称，例如 [保留名称的另一种拼写](https://code.claude.com/docs/zh-CN/errors.md#marketplace-name-is-another-spelling-of-a-reserved-name)。[调试日志](https://code.claude.com/docs/zh-CN/debug-your-config.md) 会指出该条目。

**处理方法：**

* 按照消息中的修复方法操作。在您的 shell 中，运行 `claude plugin marketplace remove <name>`，然后从受支持的源或本地路径再次添加该市场，并重新安装其插件（remove 命令会卸载这些插件）。remove 命令对被忽略的条目同样有效
* 要保留位于网络位置的市场，请在您的用户设置或托管设置中的 [`extraKnownMarketplaces`](https://code.claude.com/docs/zh-CN/settings-reference#extraknownmarketplaces) 下声明它；仓库的 `.claude/settings.json` 或 `.claude/settings.local.json` 中的声明不算数
* 对于与其设置声明不同的源，请从声明的源重新添加市场，或更改声明。`claude plugin marketplace add` 会拒绝同样的不匹配；请参阅 [对应的 `Cannot add marketplace` 条目](#cannot-add-marketplace-source-doesnt-match)
* 对于被拒绝的名称，请删除该市场；如果该行给出了 `Remove it:` 之后的命令，则使用该命令。以相同名称再次添加会再次被拒绝

在 v2.1.286 之前，无论原因是什么，`claude plugin list` 都将此类市场报告为 `Marketplace <name> not found`，而 `/plugin` **Errors** 选项卡将其报告为 `Marketplace "<name>" is registered but was refused (see the debug log)`。原因仅出现在调试日志中。在 v2.1.286 中，原因和修复句子使用了不同的措辞，例如 `Its recorded location is network-shaped or unclassifiable (never probed)`。

### `Plugin <name> has a corrupt manifest file` 或 `has an invalid manifest file`

Claude Code 获取了插件，然后无法读取其 `.claude-plugin/plugin.json`。在 shell 中，此行中的 `<name>` 可以是临时目录名称；`Failed to install plugin "<name>@<marketplace>"` 前缀携带插件的真实名称。措辞说明哪个检查失败：

* **`corrupt manifest file`，后跟 `JSON parse error:`**：文件不是有效的 JSON
* **`invalid manifest file`，后跟 `Validation errors:`**：文件可以解析但未通过 schema 校验，例如缺失必需字段时显示 `name: Invalid input`

`claude plugin install` 报告为 `Failed to install plugin "<name>@<marketplace>":` 并以代码 1 退出。

插件的作者必须修复文件，在那之前无法安装插件：

* **如果那是您**：在您的 shell 中运行 `claude plugin validate <plugin-directory>` 以查看相同的错误和违规路径，然后修复文件
* **如果不是您**：向市场所有者报告消息

### `Plugin directory not found at path: <path>`

`/plugin` 中的 **Errors** 选项卡显示这个用于启用的插件，其市场通过相对路径列出，例如 `./plugins/my-plugin`，当市场内该路径处不存在目录时。如果您维护市场，请更正条目的 `source` 路径或恢复文件夹。否则，向市场所有者报告消息。

`Marketplace directory not found at path: <path>` 意味着市场自己的目录缺失。对于您从本地路径添加的市场，该目录已移动或被删除。恢复它，或删除市场并从其新位置再次添加它。

### `No plugins available` 或 `No marketplaces configured`

您打开了 `/plugin`，**Discover** 选项卡为空，或 `claude plugin marketplace list` 打印 `No marketplaces configured`。

没有注册市场，因此没有目录可显示。在会话中，添加官方市场 `anthropics/claude-plugins-official`：

```text theme={null}
/plugin marketplace add anthropics/claude-plugins-official
```

Claude Code 打印 `Successfully added marketplace: claude-plugins-official`，**Discover** 列出其插件。[Anthropic 市场](https://code.claude.com/docs/zh-CN/plugins/anthropic-marketplaces.md) 页面列出了您可以添加的其他市场。

### `Marketplace "<name>" is already added from a different source`

您在会话中或从 shell 中，通过 [安装命令上的 `--marketplace <source>`](https://code.claude.com/docs/zh-CN/plugins/install#add-a-marketplace-and-install-in-one-command) 指定了一个新的市场源。Claude Code 从该源获取的目录与您已从不同源添加的市场具有相同的名称。Claude Code 保留现有市场而不是替换它，插件未安装。

完整消息如下所示：

```text theme={null}
Marketplace "acme-tools" is already added from a different source (github:acme/plugins). To use this source instead, remove that marketplace first with /plugin marketplace remove acme-tools.
```

选择您想要的源：

* **您已添加的市场**：使用 `/plugin install <plugin>@<name>` 按名称从它安装
* **新源**：运行 `/plugin marketplace remove <name>`，然后重试安装

### `Cannot add marketplace "<name>": its source doesn't match its extraKnownMarketplaces entry in user or managed settings`

您添加了一个市场，而其 `marketplace.json` 中的 `name` 在您的用户设置或托管设置中已有一个 [`extraKnownMarketplaces`](https://code.claude.com/docs/zh-CN/settings-reference#extraknownmarketplaces) 条目。您提供的源与该条目列出的源不同，因此 Claude Code 拒绝添加，并且不注册任何内容。

只有当两个源类型相同且每个字段的值都相同时，它们才匹配。设置了您未传递的 `ref` 的条目视为不同。当您以 `https://github.com/` URL 的形式提供仓库时，`github` 条目也视为不同，因为 Claude Code 会将该 URL 记录为 [`git` 源](https://code.claude.com/docs/zh-CN/plugins/marketplace-reference#marketplace-sources)。执行以下操作之一：

* **使用声明的源**：Claude Code 会自行[注册设置中声明的市场](https://code.claude.com/docs/zh-CN/settings-reference.md#extraknownmarketplaces)，因此请先在会话中运行 `/plugin marketplace list`。如果列表中显示该名称，则该市场已注册，无需添加。

  如果列表中未显示该名称，请按照条目的写法键入源来添加它。对于 `source` 对象为 `{ "source": "github", "repo": "acme-corp/claude-plugins", "ref": "v1.2.0" }` 的条目，请运行：

  ```text theme={null}
  /plugin marketplace add acme-corp/claude-plugins#v1.2.0
  ```

* **使用新源**：编辑或删除 `extraKnownMarketplaces` 条目，然后再次添加市场。如果托管设置声明它，请询问您的管理员

在 v2.1.287 之前，该消息为 `Cannot add marketplace "<name>": its network source differs from the one declared for it in settings (kind, target, or a fetch-shaping field such as headers / ref / path / sparsePaths)`。

### `Failed to install: <plugin> (<reason>)`

您在 `/plugin` 菜单中选择了要安装的插件，它们都没有安装，菜单关闭并显示失败的摘要。

某些原因（例如失败克隆后 git 的输出）仅显示其第一行。当这样的原因被缩短时，摘要以 `Installing a plugin from its details (Enter) in /plugin shows its full error.` 结尾。

要做什么取决于摘要是否缩短了原因：

* 修复括号中原因命名的内容
* 当原因被缩短时，运行 `/plugin`，在 **Discover** 选项卡上选择插件，然后按 **Enter** 从其详细信息安装它。如果安装在那里失败，详细信息视图显示整个错误

### `Could not move the new copy of this plugin version into <path>`

当您安装插件时，Claude Code 下载其文件的新副本并将其移动到 [插件缓存](https://code.claude.com/docs/zh-CN/plugins/loading.md#find-plugins-on-disk) 中该版本的文件夹中。此消息意味着移动失败，通常是因为另一个程序在安装运行时使用了该文件夹。文件系统代码出现在括号中：

```text theme={null}
Could not move the new copy of this plugin version into /home/user/.claude/plugins/cache/acme-tools/formatter/1.2.0: the new copy or the version folder stayed busy while the install ran (ENOTEMPTY) — usually a scanner still reading the freshly downloaded files, another program using that folder, or another process re-creating it. The previously installed copy was moved back. Run the install again once other Claude Code sessions or programs using that folder have finished.
```

消息说明了之前安装的副本发生了什么，这告诉您插件是否仍然有效：

* `The previously installed copy was moved back`：您拥有的版本仍然安装
* `had to be removed first`、`was not moved back` 或 `could not be moved back`：该插件版本在安装成功之前未安装
* 没有这样的句子：没有早期副本，因此版本尚未安装

在 Windows 上，当另一个程序持有已安装的副本本身时，消息改为说该副本 `could not be replaced` 并且 `It was not replaced and the new copy was discarded`，因此您拥有的版本仍然安装。

`Left on disk` 列表命名缓存内的搁置文件夹。稍后安装该版本或插件缓存清理会删除它们，因此您不需要删除它们。

要修复安装：

* 关闭使用插件文件夹的其他 Claude Code 会话、编辑器和终端（在 `~/.claude/plugins/cache` 下），然后再次运行安装
* 当消息说检查插件缓存文件夹的权限时，恢复您对其命名的文件夹的写入权限并释放磁盘空间，然后再次运行安装

### 依赖项错误

声明依赖项的插件在无法满足依赖项时可能无法安装或安装并保持禁用。消息在安装时或加载时到达您：

* **在安装期间**：拒绝作为安装的错误消息返回
* **当插件加载时**：问题出现在 `claude plugin list` 和 `/plugin` **Errors** 选项卡中，Claude Code 保持受影响的插件禁用，直到您解决它

表格列出了每条消息及其修复。要作为作者声明依赖项，请参阅 [插件依赖项](https://code.claude.com/docs/zh-CN/plugins/dependencies.md)。

| 消息 | 含义 | 如何解决 |
| :- | :- | :- |
| `Dependency "<dep>" is not installed` | 声明的依赖项未安装。 | 使用 `claude plugin install <dep>@<marketplace>` 在您的 shell 中安装它，或卸载插件。如果依赖项的市场尚未注册，请添加它并在您的会话中运行 `/reload-plugins`，它安装它可以解决的缺失依赖项。 |
| `"<name>@synced" is loaded from your claude.ai account but meets a marketplace dependency only for other claude.ai plugins`，位于 `is not installed` 消息中 | 该依赖项指定了一个市场，而同名插件是从您的 claude.ai 账户加载的。声明该依赖项的插件不是从 claude.ai 加载的，因此该 claude.ai 插件无法满足它。 | 在您的 shell 中运行 `claude plugin install <dep>@<marketplace>`，从其市场安装该依赖项。之后将加载市场副本，而不是 claude.ai 副本。 |
| `Dependency "<dep>" is disabled` | 依赖项已安装但关闭。 | 启用依赖项，或卸载需要它的插件。 |
| `Requires "<dep>" <range>, installed <version>` | 已安装的依赖项的版本在插件的声明范围之外。 | 将依赖项更新到范围内的版本，或卸载插件。 |
| `<Plugin or Dependency> "<name>" has conflicting version requirements` | 没有版本满足每个引脚它的范围。消息列出范围。 | 卸载或更新其中一个冲突的插件，或要求上游作者扩大其约束。 |
| `... has version requirements too complex to intersect` 或 `has an invalid version requirement` | 范围不是有效的 semver，或组合范围无法相交。 | 修复无效范围或简化长 `\|\|` 链。 |
| `... has no git tag satisfying <range>` | 依赖项的仓库在范围内没有 `<name>--v*` 标签。 | 检查上游是否使用该约定标记发布，或放宽范围。 |
| `Dependency "<dep>" (required by <plugin>) is in marketplace "<marketplace>", which is not in the allowlist` | 依赖项在不同的市场中，默认情况下跨市场解析已关闭。 | 自己在相同的作用域安装依赖项，在您的 shell 中使用 `claude plugin install <dep>@<marketplace>` 加上您安装插件的 `--scope`，然后重试。 |

要以编程方式查看这些，请在您的 shell 中运行 `claude plugin list --json`。有问题的插件携带带有消息的 `errors` 字段和带有每个 `type` 的 `errorDetails` 字段：前三行是 `dependency-unsatisfied`，第四行是 `dependency-version-unsatisfied`。

## 插件已安装但不工作

安装成功，但插件的技能、hooks 或服务器没有做任何事情。从 [插件不出现或其技能不显示](#plugin-doesnt-appear-or-its-skills-dont-show-up) 开始，它告诉您 Claude Code 在哪里报告它加载的内容，然后匹配消息。

### 插件不出现或其技能不显示

您安装了插件并键入 `/` 期望其技能，或要求 Claude 使用它，但什么都没有发生。

在更改任何内容之前检查插件的状态：

**1. 确认插件已安装并启用**

运行 `/plugin` 并打开 **Installed**。确认插件已列出并启用。您的 shell 中的 `claude plugin list` 打印相同的列表，每个插件的版本、作用域和 `Status: ✔ enabled`。

**2. 阅读 Errors 选项卡**

在同一面板中打开 **Errors** 选项卡。每个条目将消息与指导行配对。本节其余部分中的大多数消息来自该选项卡。

**3. 如果您在此会话期间安装，请重新加载**

如果插件已安装且无错误，但您在此会话期间安装了它，请运行 `/reload-plugins`。它打印 `Reloaded:` 和插件、技能、代理、hooks 和服务器的计数。当某些失败时，它添加 `N errors during load. Run /plugin for details.`

如果插件加载无错误且其技能仍然不出现，下一步因您自己的插件和其他人的而异：

* **您正在构建的插件**：请参阅 [插件加载但其技能缺失](#plugin-loads-but-its-skills-are-missing)
* **某人发布的插件**：在 `/plugin` 中打开 **Installed** 并打开插件的详细信息窗格，其中列出了插件包含的内容。在那里列出无技能的插件在您键入 `/` 时没有什么可提供的

### `Run /reload-plugins to apply.`

`/plugin` 中的安装摘要以 `Run /reload-plugins to apply.` 结尾，而不是 `Plugin is now active.`。同时，输入框上方可能会出现 `Plugins changed. Run /reload-plugins to activate.` 通知。

Claude Code 在安装期间没有激活插件，要么是因为激活它会 [使提示缓存失效](https://code.claude.com/docs/zh-CN/prompt-caching.md#enabling-or-disabling-a-plugin)，要么是因为激活尝试失败。

您不需要键入命令。面板关闭，Claude Code 为您运行 `/reload-plugins`，或将其排队直到流式传输的响应完成。

阅读该重新加载打印的内容：

* **`Reloaded:` 和插件、技能、代理、hooks 和服务器的计数**：插件现在处于活动状态。当某些加载失败时，该行添加 `N errors during load. Run /plugin for details.`
* **`This reload changes MCP tools (...) — your next message will re-read the whole conversation instead of using the cache. Run /reload-plugins --force to apply.`**：重新加载会添加或删除插件 MCP 服务器，或 `LSP` 工具，并使您的提示缓存失效。对于 LSP 情况，该行以 `This reload adds the LSP tool` 或 `This reload removes the LSP tool` 开头。运行它带有 `--force` 以激活插件，或启动新会话

在 v2.1.268 之前，在安装期间未激活的安装保持待处理状态，直到您自己运行 `/reload-plugins`。

在 v2.1.246 之前，该摘要中的技能计数仅包括插件的 `commands/` 条目，因此重新加载可以加载插件的 `SKILL.md` 技能并仍然报告 `0 skills`。

### `The packages it lists are not installed` 或 `were not installed, because ...`

当插件的依赖安装未留下 `node_modules` 目录时，`/plugin` 和 `claude plugin list` 会在该插件上显示以下注释之一。插件会加载，但需要缺失包的部分可能无法工作。

* **`are not installed`**：此插件可以运行安装，但安装没有完成，例如因为安装失败或超时。要重试安装，请在您的 shell 中运行注释给出的 `claude plugin update` 命令，或从 `/plugin` 更新插件

  ```shell theme={null}
  claude plugin update formatter@my-marketplace
  ```

  如果重试失败，输出会给出原因。当设置了 `CLAUDE_CODE_DISABLE_NONESSENTIAL_TRAFFIC` 时，`claude plugin update` 和 `/plugin` 会跳过重试，并报告插件已是最新版本。

* **`were not installed, because ...`**：此插件无法运行安装，注释会说明原因，例如 Yarn、pnpm 或 `bun.lockb` lockfile，或者其包管理器未安装在此计算机上的 lockfile。只要该原因存在，更新插件就不会安装这些包。如果原因是 lockfile，插件作者必须替换它。如果是缺少包管理器，请安装它，然后更新插件

### `Plugin "<name>" not cached at <path>`

**Errors** 选项卡显示此行，指导为 `Run /plugin to refresh the plugin cache`。Claude Code 有插件的安装记录，但记录指向的目录缺失，例如在您清除缓存后。

从您的 shell 重新安装插件。`claude plugin install <name>@<marketplace>` 重新下载安装目录缺失的插件，即使其记录存在：

```shell theme={null}
claude plugin install <name>@<marketplace>
```

然后在您的会话中运行 `/reload-plugins`。**Errors** 选项卡条目消失，插件回到 **Installed** 下。

### `installed_plugins.json holds a record under "<id>" that this version of Claude Code cannot read`

消息以这些形式出现：

* **`claude plugin list`**：将其打印为 `Note:`
* **`claude plugin install`、`uninstall` 和 `update`**：拒绝并显示 `Plugin "<name>" was not installed:`、`Plugin "<name>" was not uninstalled:` 或 `Plugin "<name>" was not updated:`，后跟相同的文本
* **这三个命令中任何一个上的 `--json`**：结果行携带相同的 `message` 和 `failureCode: "install_records_unreadable"`
* **多个这样的记录**：消息读作 `holds records under`
* **整个文件声明此版本不知道的格式**：消息读作 `installed_plugins.json is in a format (version <N>) that this version of Claude Code does not know` 而不是

命名的记录在 `installed_plugins.json` 中是有效的 JSON，在有效的插件 id 下，但其字段对此版本不解析。最可能是另一个版本的 Claude Code 写了它，也许是更新的版本。

当记录在那里时，此版本不重写文件，所以记录不会丢失。

按顺序采取消息的选项：

1. 使用 `claude update` 更新 Claude Code。
2. 如果您无法更新，请使用写入记录的 Claude Code 版本卸载命名的插件。
3. 如果两者都没有帮助，请手动从 `installed_plugins.json` 删除记录，然后重新启动 Claude Code 或运行 `/reload-plugins`。

### `installed_plugins.json could not be read and was rebuilt`

`claude plugin list` 打印此注释，带有保留文件的路径，名为 `installed_plugins.unreadable.<date>.<hash>.kept`，只要该文件位于 `installed_plugins.json` 旁边。

不是有效 JSON 的 `installed_plugins.json`，或不是插件列表的，无法说出您安装了什么。

打开 `.kept` 文件以查看旧文件记录的内容，并重新安装您缺少的插件。Claude Code 永远不会读回该文件，该文件在 [`cleanupPeriodDays`](https://code.claude.com/docs/zh-CN/settings-reference#cleanupperioddays) 计划上老化。

### `install records under names that no version of Claude Code can use were removed from installed_plugins.json`

`claude plugin list` 打印此注释，带有副本的路径，名为 `installed_plugins.set-aside.<date>.<hash>.json`，只要该副本位于 `installed_plugins.json` 旁边。注释以 `Nothing needs doing about these copies.` 结尾。

`installed_plugins.json` 中的记录位于不是有效插件 id 的键下，因此没有版本的 Claude Code 可以使用它。文件的其余部分正常加载。

Claude Code 将不可用的记录复制到 `.set-aside` 文件中并将其从列表中删除。Claude Code 永远不会读回副本，副本在 [`cleanupPeriodDays`](https://code.claude.com/docs/zh-CN/settings-reference#cleanupperioddays) 计划上老化。

### `does not load (...), so Claude Code ignores the whole file`

命令已成功执行。警告中指明的设置文件存在错误，因此在您修复之前，Claude Code 会忽略整个文件，包括命令写入其中的任何内容。

修复警告中指明的错误。对于 Claude Code 不接受的值，[修复损坏的设置文件](https://code.claude.com/docs/zh-CN/settings.md#fix-a-broken-settings-file) 说明了修复方法。然后，如果命令所做的更改已不在文件中，请再次运行该命令。

该警告出现在您的 shell 中 `claude plugin install`、`enable`、`disable` 或 `claude plugin marketplace add` 的成功行之后：

```text theme={null}
⚠ /home/user/.claude/settings.json does not load (its "permissions" is not valid), so Claude Code ignores the whole file, including anything this command wrote there. Fix the file, then run this command again if its change is missing. If a newer Claude Code wrote the file, update Claude Code instead.
```

括号中的文本指明了错误：

* **`its "<key>" is not valid`**：引号中的设置项包含 Claude Code 不接受的值。在 [设置参考](https://code.claude.com/docs/zh-CN/settings-reference.md) 中查找该设置项可接受的值。当多个值无效时，文本会指明第一个设置项并统计其他设置项的数量，例如 `its "permissions" and 1 other value are not valid`。
* **`it is not a JSON object`**：文件的顶层不是 JSON 对象，例如顶层为数组的文件。

### `Disabled in ~/.claude/settings.json but still loads`

您在 `~/.claude/settings.json` 中将插件设置为 `false`，其在 `claude plugin list` 或 `/plugin` 中的行显示此消息，后跟启用它的源，例如 `— project settings enable it, which overrides your user setting`。该更高优先级源中的 `true` 覆盖了您的用户设置。

要在您的机器上选择退出项目启用的插件，请在 `.claude/settings.local.json` 中将 id 设置为 `false`，它的优先级高于项目文件。对于消息可以命名的其他源，请参阅 [在用户设置中禁用但仍然加载](https://code.claude.com/docs/zh-CN/plugins/loading.md#disabled-in-user-settings-but-still-loads)。

如果 `claude plugin list` 改为将插件标记为 `required by your org`，则不涉及设置文件：您的组织在 claude.ai 上将该同步插件标记为必需，即使您之前禁用了它，它也会加载。请参阅 [从 claude.ai 同步的插件](https://code.claude.com/docs/zh-CN/plugins/loading.md#synced-plugins)。

### `Plugin "<name>" is enabled in project settings but isn't installed here`

**Errors** 选项卡显示此行用于您的项目的 `.claude/settings.json` 启用的插件，指导为 `Run claude plugin install <name>@<marketplace> --scope project to install it for this project`。

存储库的设置可以为打开它的每个人启用插件，但它们不安装它。当插件来自外部源（例如 GitHub 存储库或 npm 包）时，Claude Code 在您自己安装它之前不会下载它。从指导行在您的 shell 中运行命令，然后重新加载：

```shell theme={null}
claude plugin install <name>@<marketplace> --scope project
```

在您的会话中运行 `/reload-plugins` 后，**Errors** 选项卡条目消失，插件列在 **Installed** 下。

如果您的组织为您预安装插件，它通过托管设置而不是这样做。请参阅 [预安装和要求插件](https://code.claude.com/docs/zh-CN/plugins/org.md#pre-install-and-require-plugins)。

### 在 Windows 上执行 `plugin uninstall` 后插件仍保持安装

在 Windows 上，您在项目或本地作用域运行 `claude plugin uninstall` 并报告成功，但 `claude plugin list` 或 `/plugin` 仍然列出该插件。

`installed_plugins.json` 为该项目文件夹保存了该插件的两条安装记录，每条记录对文件夹路径的写法不同，而一次卸载只会删除其中一条。要进行检查，请在您的 shell 中运行 `claude plugin list --json`。该插件剩余的行中，`projectPath` 对文件夹的写法与您运行卸载的位置不同，例如 `C:\work\app` 被写为 `c:\work\app`。

在同一文件夹中，使用相同的 `--scope` 再次运行相同的卸载命令。第二次运行在其自身的路径写法下找不到记录，因此会删除另一种写法下的记录。对于项目作用域的安装：

```shell theme={null}
claude plugin uninstall <name>@<marketplace> --scope project
```

然后再次运行 `claude plugin list --json`，确认该行已消失。

在 v2.1.295 之前，第二次运行会失败并显示 `Plugin "<name>" is not installed in project scope`。请运行 `claude update`，然后再次运行卸载。

### 从 git 仓库安装的插件显示版本 `unknown`

`claude plugin list` 中的 `Version: unknown` 可能意味着 Claude Code 无法读取它用作 git 来源插件版本的提交。该插件可能有自己的仓库，也可能位于 git 托管的市场中。

在您的 shell 中运行以下命令，将 `formatter@my-marketplace` 替换为 `claude plugin list` 中该插件的 `name@marketplace` 行：

**1. 更新 Claude Code**

[将 Claude Code 更新](https://code.claude.com/docs/zh-CN/setup.md#update-claude-code)到 v2.1.302 或更高版本，该版本会以能够从中读取提交的格式进行克隆：

```shell theme={null}
claude update
```

**2. 更新市场**

更新 Claude Code 的市场副本，其中保存了市场仓库内插件的提交：

```shell theme={null}
claude plugin marketplace update my-marketplace
```

**3. 更新插件**

更新插件以重写其记录的版本：

```shell theme={null}
claude plugin update formatter@my-marketplace
```

**4. 检查版本**

检查 `Version:` 行是否显示诸如 [12 字符提交 ID](https://code.claude.com/docs/zh-CN/plugins/loading.md#how-claude-code-computes-the-version) 之类的值：

```shell theme={null}
claude plugin list
```

`claude plugin update` [每次运行只更改一个作用域的块](https://code.claude.com/docs/zh-CN/plugins/cli-reference.md#which-scope-the-command-updates)。对于仍显示 `unknown` 的块，请使用该块的 `Scope:` 值重新运行：

```shell theme={null}
claude plugin update formatter@my-marketplace --scope project
```

如果重新运行后该块仍显示 `unknown`，请参阅 [获取更多帮助](https://code.claude.com/docs/zh-CN/troubleshooting.md#get-more-help)。

### `Failed to load hooks from <path>` 和不触发的 hooks

插件的 hooks 不运行，或一个阻止了一个操作。要么 **Errors** 选项卡显示它们的加载失败，hooks 加载且您在成绩单中看到 `<Event> hook error` 通知或阻止错误，要么 hook 加载无错误且永远不触发。

#### Hooks 无法加载

**Errors** 选项卡显示以下消息之一：

* **`Failed to load hooks from <path>: <reason>`**：`hooks/hooks.json` 不是有效的 JSON 或失败 hooks 架构。原因命名解析或验证错误。修复文件。要在发布插件前在 `hooks/hooks.json` 中捕获 JSON 语法问题，请在您的 shell 中运行 `claude plugin validate <plugin-directory>`
* **`hooks path not found: <path>`**：清单的 `hooks` 字段命名在该路径相对于插件根处不存在的文件。修复路径或添加文件
* **`hooks path escapes plugin directory: <path>`**：请参阅 [Path escapes plugin directory](https://code.claude.com/docs/zh-CN/errors.md#path-escapes-plugin-directory)

#### 成绩单中的 `hook error` 通知

形式为 `... hook error: Failed with non-blocking status code: <stderr>` 的通知意味着 hook 运行且其命令失败。例如，`Stop hook error: Failed with non-blocking status code: /bin/sh: node: command not found` 意味着 Claude Code 生成的 shell 找不到 `node`。安装它，或确保它在您启动 `claude` 的终端的 `PATH` 上。

如果 stderr 显示插件的路径在空格处被截断，hook 的 shell 形式命令在引号外使用 `${CLAUDE_PLUGIN_ROOT}`，安装路径包含空格。将变量用双引号包装或使用 [exec 形式](https://code.claude.com/docs/zh-CN/hooks.md#exec-form-and-shell-form)。要找到未引用的变量，请在插件的目录上运行 `claude plugin validate` 并查找其 [引用警告](https://code.claude.com/docs/zh-CN/plugins/manifest-reference.md#quoting-and-path-separators)。

如果通知内容为 `Failed to run: Plugin directory does not exist: <path>`，请参阅 [`Plugin directory does not exist`](#plugin-directory-does-not-exist)。

对于任何其他错误，从插件目录自己运行 hook 的命令以查看完整输出，或使用 [调试日志](https://code.claude.com/docs/zh-CN/hooks.md#debug-hooks) 捕获完整 stderr。

#### 插件 hook 阻止工具调用或提示

退出代码为 2 的 hook [阻止它运行的操作](https://code.claude.com/docs/zh-CN/hooks.md#exit-code-2)。当插件的 hook 以这种方式阻止且其 stderr 是阻止消息时，错误以 `This hook comes from the <plugin> plugin.` 结尾，以便您知道要禁用或修复哪个插件。在 v2.1.281 之前，错误没有命名插件。

如果该消息显示插件的路径在空格处被截断，应用 [未引用的 `${CLAUDE_PLUGIN_ROOT}` 修复](#hook-error-notices-in-the-transcript)。

#### Hook 加载但永远不触发

如果 hook 加载无错误但永远不触发，请先在会话中运行 `/plugin` 并打开该插件的详细信息。如果其中有一条以 `Its hooks.json hooks do not run` 开头的说明，则表示另一个同名的已启用插件改为注册了它的 hook，[两个已启用插件同名时的 hook](https://code.claude.com/docs/zh-CN/plugins/loading.md#hooks-when-two-enabled-plugins-share-a-name) 说明了是哪个副本以及如何切换。否则，请检查 hook 的定义，然后观察它运行：

**1. 检查事件名称**

事件名称区分大小写，因此确认您的完全匹配，例如 `PostToolUse`。

**2. 检查匹配器**

确认 hook 的 `matcher` 匹配工具名称。

**3. 故意触发事件**

对于 `PostToolUse` hook，要求 Claude 编辑文件。

**4. 阅读调试日志**

打开 [调试日志](https://code.claude.com/docs/zh-CN/hooks.md#debug-hooks)，它记录哪些 hooks 匹配。运行的 hook 显示在那里及其退出代码。

### `Plugin directory does not exist: <path>`

即使消息提示重新安装，也请先在 Claude Code 输入框中运行 `/reload-plugins`。当您的会话加载插件 hook 所用的目录已从磁盘上消失时，插件的 hook 会失败并显示 `Failed to run: Plugin directory does not exist: <path> (<plugin> — run /plugin to reinstall)`，且该 hook 不会运行。[`Plugin directory not found at path: <path>`](#plugin-directory-not-found-at-path) 是另一条消息，与市场条目有关。

重新加载会从插件的当前目录加载其 hook。对于每个 hook 事件和命令，该失败在每个会话中只显示一次，因此 hook 不再报错并不能确认问题已修复。请改为阅读重新加载的输出：

* **`Reloaded:` 且没有错误行**：插件的 hook 不再指向缺失的目录
* **`N errors during load. Run /plugin for details.`**：在 `/plugin` 中打开 **Errors** 选项卡，并按照本页中针对其显示的消息的条目操作
* **以 `Run /reload-plugins --force to apply.` 结尾的行**：没有重新加载任何内容，hook 会继续失败。请在 Claude Code 输入框中运行 `/reload-plugins --force`

### `Invalid MCP server config for "<server>"` 和不启动的 MCP 服务器

插件捆绑了 MCP 服务器，**Errors** 选项卡显示 `Invalid MCP server config for "<server>": <error>`，或服务器已列出但 `/mcp` 永远不显示它已连接。

#### `Invalid MCP server config for "<server>": <error>`

服务器的配置通过架构检查，但 Claude Code 无法为此会话解决它。冒号后的文本命名原因并决定修复：

* **`Missing environment variables: <names>`**：在启动 Claude Code 的 shell 中设置这些变量，然后启动新会话
* **`URL is unset or invalid`**：URL 使用的 `${user_config.*}` 选项未设置。运行 `/plugin configure <plugin>` 设置它
* **`has an invalid MCP url`** 或 **`headersHelper for MCP server '<server>' references ${user_config.*}`**：插件自己的配置有问题。修复您的插件的 MCP 配置中的 `url` 或 `headersHelper`，或如果插件不是您的，向插件的作者报告。`headersHelper` 情况在 [插件命令参考 user\_config](https://code.claude.com/docs/zh-CN/errors.md#plugin-command-references-user-config) 下有其自己的条目

#### `Bundled MCP server "<name>" was not started: it needs configuration`

插件包括服务器作为 [MCPB bundle](https://code.claude.com/docs/zh-CN/plugins/components.md#include-a-packaged-mcpb-server)，声明 `user_config`，且必需的设置没有保存的值或保存的值失败 bundle 自己的验证，所以 Claude Code 跳过启动服务器。插件的其余部分工作。

在 `/plugin` 的 **Installed** 选项卡上选择插件并选择 **Configure** 以提供值。保存后，`/plugin` 显示 `Configuration saved.` 并关闭，Claude Code 重新加载插件如 [管理已安装的插件](https://code.claude.com/docs/zh-CN/plugins/install.md#manage-installed-plugins) 下所述。服务器在该重新加载应用后启动。在 v2.1.285 之前，Claude Code 跳过服务器而不显示此行。

#### 服务器已配置但永远不连接

运行 `/mcp` 查看服务器的状态。当服务器健康时，`/mcp` 将其列为已连接。

要读取服务器在启动时打印的错误，请运行 `claude --debug` 并打开 `~/.claude/debug/<session-id>.txt` 处的日志。`--debug` 标志不打印到终端。

> [!NOTE]
>
> 当与 [插件的 stdio 服务器](https://code.claude.com/docs/zh-CN/mcp.md#plugin-provided-mcp-servers) 的连接在您运行 `claude --debug` 之前不到 15 分钟内失败时，Claude Code [不会启动该服务器](https://code.claude.com/docs/zh-CN/errors.md#skipping-connection-recent-failure-cached)。在新会话中，从 `/mcp` 中该服务器的菜单选择 **Reconnect** 以启动它，然后打开日志。

`.mcp.json` 中失败架构的服务器条目不出现在 **Errors** 选项卡中。Claude Code 删除该服务器并仅在该调试日志中记录 `Invalid MCP server config for <server> in <path>`。要在不加载插件的情况下找到条目，请在插件目录上在您的 shell 中运行 `claude plugin validate`，它将其报告为错误。

在 v2.1.281 之前，`claude plugin validate` 没有检查 `.mcp.json`。

#### 服务器使用 `--plugin-dir` 工作但安装后失败

您是插件的作者，当您使用 `--plugin-dir` 从其源目录加载插件时服务器启动，但一旦插件安装就失败。

Claude Code 将已安装的插件复制到其缓存中，因此仅从源目录工作的路径会中断。使用 `${CLAUDE_PLUGIN_ROOT}` 编写插件内的路径。

对于到达插件目录外的路径，请参阅 [插件引用的文件在其目录外找不到](#files-the-plugin-references-outside-its-directory-arent-found)。

### 语言服务器不启动、使用过多内存或报告错误的诊断

您安装了 [代码智能插件](https://code.claude.com/docs/zh-CN/plugins/code-intelligence.md)，Claude 没有看到诊断，或语言服务器使用过多内存或报告不是真实的错误。

#### 语言服务器不启动

插件连接到您单独安装的语言服务器二进制文件，Claude Code 从您的 `PATH` 按命令名称生成它。

`/plugin` **Errors** 选项卡显示失败及其原因，例如 `Executable not found in $PATH: "<binary>"`，`claude --debug` 将其记录为 `LSP server <name> failed to start: <reason>`。

安装二进制文件并确认它在您启动 `claude` 的终端的 `PATH` 上，例如使用 `which typescript-language-server`。然后启动新会话。

#### 语言服务器使用过多内存

语言服务器（例如 `rust-analyzer` 和 `pyright`）索引整个项目。使用 `/plugin disable <plugin>` 在会话中禁用插件，改为依赖 Claude 的内置搜索工具。

#### monorepo 中的假阳性诊断

未为工作区配置的语言服务器可以报告内部包的未解析导入。Claude Code 端没有什么可修复的，诊断不会阻止 Claude 编辑代码。

## 构建插件

您正在开发插件，并使用 `--plugin-dir` 加载它或从本地市场安装它。这些条目涵盖了开发插件时遇到的失败。要了解每次更改后需要运行的检查，请参阅[测试和调试](https://code.claude.com/docs/zh-CN/plugins/create.md#test-and-debug)。

有两种失败也会影响插件的用户，它们的条目位于[插件已安装但无法工作](#plugin-installed-but-not-working)下：

* **未触发的 hook**：请参阅[未触发的 hook](#failed-to-load-hooks-from-and-hooks-that-dont-fire)
* **无法启动的 MCP 服务器**：请参阅[无法启动的 MCP 服务器](#invalid-mcp-server-config-for-and-mcp-servers-that-dont-start)

### `commands path not found: <path>`

**Errors** 选项卡显示 `commands path not found: <absolute path>`，并提示 `Check that the path in your manifest or marketplace config is correct`。`skills`、`agents` 和 `hooks` 也会显示相同的消息。

Claude Code 根据插件根目录解析了您的 `plugin.json` 或市场条目中的路径，但在那里找不到任何内容。消息中的路径是它检查的绝对路径，因此请将其与磁盘上的内容进行比较。修复路径或创建目录，然后运行 `/reload-plugins`。

> [!NOTE]
>
> 在 v2.1.299 之前，此消息可能会在启动时出现在带有 `synced[0]` 等标签的行上。该行对应[从 claude.ai 同步](https://code.claude.com/docs/zh-CN/plugins/loading.md#synced-plugins)的插件。当您计算机上的另一个 Claude Code 会话刚刚更新了该插件并移动了其文件时，就会出现此消息。要加载该插件，请重启 Claude Code。要避免因此原因出现该消息，请更新到 v2.1.299 或更高版本。

清单中的路径相对于插件根目录，以 `./` 开头。解析到插件根目录外的路径会被报告为 `<component> path escapes plugin directory`，并被丢弃。

### `--plugin-dir` 在市场根目录处不会加载 `plugins/` 下的插件

您启动了 `claude --plugin-dir <path>`，没有看到错误，但插件的 skill、Agent 和 hook 不存在。

`--plugin-dir` 接受插件的根目录，即包含 `.claude-plugin/plugin.json` 和 `skills/` 等组件目录的目录。如果改为指向市场根目录，Claude Code 不会读取 `marketplace.json`，因此 `plugins/` 下的插件不会加载，也看不到任何错误。在 v2.1.281 之前，Claude Code 将市场根目录作为一个以该目录命名的空插件加载。请将标志指向插件目录本身：

```shell theme={null}
claude --plugin-dir ./my-marketplace/plugins/my-plugin
```

然后在 `/plugin` 中打开 **Installed**，插件的详情窗格会列出其组件。

### 插件引用的目录外文件找不到

插件使用 `--plugin-dir` 从其源目录运行正常，但安装后失败，出现关于 `../shared-utils` 等路径的错误。

Claude Code 将已安装的插件复制到其缓存中并从那里加载，因此指向插件自身目录之外的路径在缓存中找不到任何内容。请将共享文件移到插件目录内，或通过插件目录内的符号链接引用它们。有关缓存位置和路径解析方式，请参阅[在磁盘上查找插件](https://code.claude.com/docs/zh-CN/plugins/loading.md#find-plugins-on-disk)。

### `${CLAUDE_PLUGIN_ROOT}` 在 Windows 上显示正斜杠

在 Windows 上，通过 Git Bash 运行的插件 hook 接收到的 `${CLAUDE_PLUGIN_ROOT}` 为 `C:/Users/you/...` 而不是 `C:\Users\you\...`，期望反斜杠的脚本因此出错。

安装了 Git Bash 时，[shell 形式的 hook](https://code.claude.com/docs/zh-CN/hooks.md#exec-form-and-shell-form) 默认通过 Git Bash 运行。对于通过 Git Bash 运行的 hook，Claude Code 以正斜杠形式传递路径，以免 Bash 将反斜杠解读为转义符。Bash 内置命令、MSYS 工具和本机 Windows 二进制文件都接受该形式。

如果您的脚本需要反斜杠，请将 hook 切换到保留本机路径的形式之一，如[执行形式和 shell 形式](https://code.claude.com/docs/zh-CN/hooks.md#exec-form-and-shell-form)下所述：

* 执行形式的 hook，它使用 `args` 数组直接生成进程
* 带有 `"shell": "powershell"` 的 hook

### 插件已加载但其 skill 缺失

您的插件在 **Installed** 下列出且没有错误，但输入 `/` 时不会提供其 skill。

skill 从插件根目录的 `skills/` 加载，command 从插件根目录的 `commands/` 加载。只有 `plugin.json` 应位于 `.claude-plugin/` 内，`.claude-plugin/` 内的 `skills/` 目录不会被扫描。请将这些目录移到插件根目录并运行 `/reload-plugins`。之后，`/plugin` 中插件的详情窗格会列出这些 skill，输入 `/` 也会提供它们。

每个 skill 是一个包含 `SKILL.md` 的目录。清单中指向 `SKILL.md` 文件而不是其目录的 `skills` 条目会被报告为 `path is a file; skills entries must be directories containing SKILL.md`。

### Skill 已加载但 Claude 从不调用该 skill

输入 `/<plugin>:<skill>` 命令时，您插件的 skill 会运行，但 Claude 在响应普通请求时从不调用它。

请按顺序检查以下原因：

* **skill 设置了 `disable-model-invocation: true`**：[创建您的第一个插件](https://code.claude.com/docs/zh-CN/plugins/create.md#create-your-first-plugin)中的模板 skill 设置了该字段。请从希望 Claude 自行调用的 skill 中删除该行。[控制谁调用 skill](https://code.claude.com/docs/zh-CN/skills.md#control-who-invokes-a-skill) 介绍了该字段
* **描述与人们的提问方式不匹配**：请完成[Skill 未触发](https://code.claude.com/docs/zh-CN/skills.md#skill-not-triggering)中的检查
* **描述被截断**：当安装了许多 skill 时，Claude Code 会缩短描述以适应列表的字符预算，这可能会删除 Claude 匹配请求所需的关键字。请参阅[Skill 描述被截断](https://code.claude.com/docs/zh-CN/skills.md#skill-descriptions-are-cut-short)

要衡量 skill 在真实提示词中触发的频率，而不是逐一检查，请使用 [`tool_used: Skill` grader](https://code.claude.com/docs/zh-CN/plugin-evals#create-your-first-eval-suite) 编写一个 eval 用例，并在每次更改描述后使用 `claude plugin eval` 运行它。

### `claude plugin eval init` 报告 `<directory> is not a plugin or skill folder`

您在不是插件根目录的目录中运行了 `claude plugin eval init`，例如您的主目录，或将插件保存在子目录中的仓库根目录。`init` 会在工作目录下写入套件，因此它会停止，而不是创建一个插件永远看不到的 `evals/` 目录。

请切换到插件的根目录（包含 `.claude-plugin/plugin.json` 或 skill 的 `SKILL.md` 的目录），然后再次运行命令。如果确实要在其他位置搭建套件，请传递 `--eval-dir`。请参阅[使用 evals 测试插件](https://code.claude.com/docs/zh-CN/plugin-evals.md)。

### `userConfig` 对话框从不出现

您的插件声明了 `userConfig` 选项，但安装时没有出现配置对话框。

安装时是否询问这些值取决于运行安装的位置：

* **在会话中运行 `/plugin install`，或使用 `/plugin` 中的 Discover 选项卡**：对话框是此交互式安装的一部分
* **VS Code 扩展的 Manage plugins 对话框**：在安装后以表单形式询问未设置的选项。在 v2.1.285 之前，在此处安装不会显示选项表单，因此请在终端会话中使用 `/plugin configure <plugin>@<marketplace>` 设置这些值
* **在 shell 中运行 `claude plugin install`**：从不提示输入 `userConfig` 值。它会保存您传递的任何 `--config KEY=VALUE` 值，当仍有选项未设置时，它会打印 `N userConfig options not yet set — run /plugin configure <plugin>@<marketplace> in Claude Code, or pass --config KEY=VALUE.` 当任何未设置的选项为必需项时，`not yet set` 后面会跟上 `(M required)`。

如果您从 shell 安装，请使用 `--config` 传递值，每个选项一个标志：

```shell theme={null}
claude plugin install my-plugin@my-marketplace --config api_url=https://example.com
```

当所有选项都已设置后，安装输出不会包含 `not yet set` 行。

如果想在之后打开对话框，请在会话中运行 `/plugin configure my-plugin@my-marketplace`。在 shell 中，[`claude plugin configure`](https://code.claude.com/docs/zh-CN/plugins/cli-reference#plugin-configure) 会显示哪些选项仍未设置，并保存通过 stdin 管道传入的值。它需要 Claude Code v2.1.285 或更高版本。

如果您传递了清单未声明的 `--config` 键，插件仍会安装，命令会打印 `⚠ Installed, but --config not applied: --config key "<key>" isn't declared in this plugin's userConfig.`，后跟插件已声明的键。

对于附带自身声明了 `user_config` 的[MCPB 包文件](https://code.claude.com/docs/zh-CN/plugins/components.md#include-a-packaged-mcpb-server)的插件，消息会改为 `isn't declared in this plugin's userConfig or by its bundled MCP servers.`，已知的键包括该服务器的键，写作 `<server>.<key>`。清单通过 URL 引用的包在安装时不会被读取，因此其键不会被列出，消息会提示在 `/plugin` 中进行配置。设置 `<server>.<key>` 键需要 Claude Code v2.1.285 或更高版本。

### `claude plugin validate` 报告错误

您运行了 `claude plugin validate <path>`，或在会话中运行了 `/plugin validate <path>`，它打印了 `Found N errors` 和 `Validation failed`，然后以代码 1 退出。

验证器读取您指定路径处的清单：插件目录为 `.claude-plugin/plugin.json`，市场目录为 `.claude-plugin/marketplace.json`，同时包含两者的目录则两者都读取。对于市场，它会在条目自身清单中的问题前加上条目索引，例如 `plugins[1] plugin.json → json: ...`。在 v2.1.289 之前，Claude Code 仅将同时包含两者的目录作为市场进行验证。

下表涵盖会导致验证停止的消息，以及两个警告 `No frontmatter block found` 和 `Unknown field '<key>'`，这两个警告仅在传递 `--strict` 时才会导致验证停止。其他警告（例如缺少描述）未列出。

| 消息 | 原因 | 修复 |
| :- | :- | :- |
| `File not found: <path>` | 路径没有清单，或路径不存在。 | 针对插件或市场根目录（即包含 `.claude-plugin/` 的目录）运行命令。 |
| `No manifest found in directory. Expected .claude-plugin/marketplace.json or .claude-plugin/plugin.json` | 目录没有 `.claude-plugin/` 清单。 | 创建清单，或指向正确的目录。 |
| `Invalid JSON syntax: <parse error>` | 清单或 `hooks/hooks.json` 不是有效的 JSON。 | 修复 JSON。在修复 `hooks/hooks.json` 之前，会话加载插件时不会包含该文件中的 hook。 |
| `Path not found: <path>. The runtime loader will report this as a load failure.` | 清单中的组件路径不存在。 | 修复路径或创建目录。 |
| `Path contains ".." which could be a path traversal attempt: <path>` | 组件路径超出了插件目录。 | 使用插件根目录内的路径。 |
| `Path is a file; skills entries must be directories containing SKILL.md` | `skills` 条目指向 `SKILL.md` 而不是其目录。 | 指向父目录，对于根级 `SKILL.md` 则使用 `.`。 |
| `No frontmatter block found` 或 `YAML frontmatter failed to parse: <error>` | skill、Agent 或 command 文件缺少 YAML frontmatter 或其无效。 | 在 `---` 分隔符之间添加或修复 frontmatter。在验证插件目录时报告。 |
| `Plugin name "<name>" is reserved: it passes as one of Anthropic's own` | 插件的 `name` 是[保留名称](https://code.claude.com/docs/zh-CN/plugins/manifest-reference.md#name)之一。 | 根据插件的功能重命名插件。 |
| `Unknown field '<key>'` | 清单中有一个 schema 未定义的字段。 | 删除它，或使用消息建议的名称。Claude Code 在加载时会忽略未知字段。有关 `plugin.json` 中的 `privacyPolicyUrl` 及其他目录列表字段，请参阅[目录列表字段](https://code.claude.com/docs/zh-CN/plugins/manifest-reference.md#directory-listing-fields)。 |

每次修复后再次运行命令，直到不再打印任何错误。

`plugin.json` 字段请参阅[清单参考](https://code.claude.com/docs/zh-CN/plugins/manifest-reference.md)，市场级消息请参阅[市场验证错误](#marketplace-validation-errors)。

### `Plugin <name> has conflicting manifests`

插件加载失败，显示 `Plugin <name> has conflicting manifests: both plugin.json and marketplace entry specify components.`

插件有自己的 `plugin.json`，而其市场条目设置了 `strict: false`，同时声明了 `commands`、`agents`、`skills`、`hooks`、`outputStyles` 或 `themes` 中的任何一个。请从条目中删除这些字段，或在条目中设置 `strict: true`，以便 Claude Code 将它们附加到 `plugin.json`。请参阅[严格模式](https://code.claude.com/docs/zh-CN/plugins/marketplace-reference.md#strict-mode)。

### `Warning: No commands found in plugin <name> custom directory`

插件加载时，位于 `~/.claude/debug/<session-id>.txt` 的 `claude --debug` 日志会记录 `Warning: No commands found in plugin <name> custom directory: <path>. Expected .md files or SKILL.md in subdirectories.` 会话或 **Errors** 选项卡中不会显示任何内容。

清单中的 `commands` 路径存在，但其中既没有 `.md` 文件，子目录中也没有 `SKILL.md`。请添加 command 文件，或从清单中删除该路径。

## 托管市场

您发布市场，用户报告错误，或您自己的验证失败。这些条目适用于市场所有者。

### 相对路径的插件在基于 URL 的市场中失败

用户使用 `https://example.com/marketplace.json` URL 添加了您的市场。其 `source` 是相对路径（例如 `./plugins/my-plugin`）的插件安装失败，显示 `its marketplace entry path does not stay inside the marketplace directory`。已安装的插件无法加载，显示 `Plugin source path refused`。两条消息都有 [错误参考条目](https://code.claude.com/docs/zh-CN/errors.md#marketplace-entry-path-does-not-stay-inside-the-marketplace-directory)。

当用户添加基于 URL 的市场时，Claude Code 仅下载 `marketplace.json` 文件本身。它不从该服务器通过相对路径获取插件文件，因此条目中的相对路径指向从未获取的目录。给每个条目一个 Claude Code 可以自己获取的源，例如 GitHub 存储库：

```json theme={null}
{ "name": "my-plugin", "source": { "source": "github", "repo": "owner/repo" } }
```

或者，在 git 存储库中托管市场并告诉用户使用存储库 URL 添加它。对于 git 源，Claude Code 克隆整个存储库，因此相对路径解析。源类型在 [市场参考](https://code.claude.com/docs/zh-CN/plugins/marketplace-reference.md) 上。

### 安装后根插件来自 git 子模块的文件缺失

您的插件位于市场根目录，并运行或读取来自仓库 git 子模块的文件。安装后，子模块的目录为空。在 v2.1.295 之前，Claude Code 会获取每个子模块，因此该目录中有内容。

当 Claude Code 克隆您的市场时，它仅获取[包含插件文件的子模块](https://code.claude.com/docs/zh-CN/plugins/marketplace-reference.md#git-submodules-in-the-marketplace-repository)。对于位于根目录的插件，它通过读取插件配置中的路径以及插件文件的文本来做出判断。它无法看到脚本在运行时构建的路径。

请将指向子模块的路径完整写在 Claude Code 能够读取的位置，例如 hook 命令中的 `${CLAUDE_PLUGIN_ROOT}/vendor/tool/run.sh`。这样 Claude Code 就会为该插件获取仓库中的每个子模块。

### 市场验证错误

您从市场目录运行了 `claude plugin validate .`，它在市场文件本身上报告了错误或警告。

`claude plugin validate` 也验证其 `source` 是本地路径的每个条目，并在条目的 `version` 与插件自己的清单不同时警告。

表格列出了市场级消息。条目级消息是 [`claude plugin validate` 报告错误](#claude-plugin-validate-reports-errors) 下的插件消息，前缀为 `plugins[N] plugin.json →`。

| 消息 | 类型 | 修复 |
| :- | :- | :- |
| `Duplicate plugin name "<name>" found in marketplace` | 错误 | 给每个插件一个唯一的 `name`。 |
| `Path contains "..": <path>` 在 `plugins[N].source` 下 | 错误 | 使用相对于市场根的路径，不带 `..` 段。 |
| `Marketplace name cannot contain control or bidirectional-formatting characters` | 错误 | 从名称中删除字符，例如转义或换行符。 |
| `Plugin name cannot contain control or bidirectional-formatting characters` | 错误 | 从插件 `name` 中删除字符。 |
| `Claude Code cannot install plugins from marketplace "<name>". Each part of a plugin id (plugin@marketplace) may use only the letters a-z and A-Z, digits, ".", "_" and "-", and must start with a letter or digit. Change the marketplace's "name".` | 错误 | 将市场重命名以符合消息所述的规则。 |
| `Claude Code cannot install plugin "<name>". Each part of a plugin id (plugin@marketplace) may use only the letters a-z and A-Z, digits, ".", "_" and "-", and must start with a letter or digit. Change this entry's "name".` | 错误 | 将条目重命名以符合消息所述的规则。 |
| `Marketplace has no plugins defined` | 警告 | 至少添加一个条目到 `plugins`。 |
| `No marketplace description provided` | 警告 | 添加顶级 `description`。 |
| `Plugin name "<name>" is not kebab-case` 在 `plugins[N] plugin.json → name` 下 | 警告 | 重命名为小写字母、数字和连字符；claude.ai 市场同步需要该形式。 |
| `Entry declares version "<a>" but <path>/plugin.json says "<b>"` | 警告 | 更新条目以匹配 `plugin.json`，这在安装时是权威的。 |
| `Marketplace name "<name>" is reserved in Claude Desktop` | 警告 | 重命名市场。Claude Desktop 的托管市场同步拒绝任何大小写的 `org`、`org-provisioned` 和 `unknown`。 |
| `Marketplace name "<name>" is not accepted by Claude Desktop` 或 `Plugin name "<name>" is not accepted by Claude Desktop` | 警告 | 重命名为最多 128 个字符的字母、数字、`.`、`_` 和 `-`，以字母或数字开头。 |

在 v2.1.247 之前，包含控制或双向格式化字符的市场名称仅报告为 `Marketplace name impersonates an official Anthropic/Claude marketplace`。

## 被您的组织阻止

您的组织部署了限制插件的托管设置，命令被拒绝，显示策略消息。这些条目命名每个拒绝背后的设置，以便您知道要求管理员什么。对于管理员端，请参阅 [为您的组织管理插件](https://code.claude.com/docs/zh-CN/plugins/org.md)。

### `Marketplace source '<source>' is blocked by enterprise policy`

您运行了 `/plugin marketplace add`、`update` 或安装，Claude Code 拒绝并显示此行。

您的管理员在托管设置中设置了 `blockedMarketplaces` 或 `strictKnownMarketplaces`，此源不被允许。要求您的管理员允许源，或添加消息列出的允许源之一。

将消息的其余部分与看到的内容匹配：

* **`Allowed sources: <list>`**：阻止来自 `strictKnownMarketplaces` 允许列表而不是 `blockedMarketplaces` 阻止列表
* **`No external marketplaces are allowed.`**：`strictKnownMarketplaces` 允许列表为空
* **一个 `Tip:` 说简写假设 github.com**：允许列表允许 git 主机按主机名，您传递的 `owner/repo` 简写指向 github.com。如果存储库位于您的内部主机，使用其完整 URL 再次添加它，例如 `git@your-git-host.com:owner/repo.git`

您在策略变得更严格之前添加的市场停止刷新，因为策略在每次刷新时应用。

### `Marketplace "<name>" is not in the allowed marketplace list`

**Errors** 选项卡显示此行，或 `Marketplace "<name>" is blocked by enterprise policy`，用于您已注册的市场。

相同的托管设置，阻止 [市场源](#marketplace-source-is-blocked-by-enterprise-policy)，在加载时应用。`strictKnownMarketplaces` 不包括此市场，或 `blockedMarketplaces` 命名它，因此 Claude Code 停止加载它及其插件。对于允许列表变体，指导行显示允许的源，或 `Contact your administrator to configure allowed marketplace sources`。对于阻止列表变体，它读取 `This marketplace source is explicitly blocked by your administrator`。

### `Plugin "<name>" is blocked by your organization's policy and cannot be installed`

安装被拒绝，显示此行，启用显示相同的行，以 `cannot be enabled` 结尾，或安装或更新显示命名原因的行：`Plugin "<name>" is from marketplace "<marketplace>", which is blocked by your organization's policy`，或 `Plugin "<name>" depends on "<dep>", which is blocked by your organization's policy`。

托管设置阻止此插件、其市场或它需要的依赖项。要求您的管理员哪个条目适用。阻止的依赖项意味着插件在依赖项的市场被允许之前无法安装。

### `--plugin-dir is disabled by your organization's managed settings (disableSideloadFlags)`

您使用 `--plugin-dir`、`--plugin-url`、`--agents` 或 `--mcp-config` 启动了 `claude`。Claude Code 以此消息退出并显示 `Plugins, custom agents, and MCP servers can only be loaded from sources your administrator has approved.`

您的管理员在托管设置中设置了 `disableSideloadFlags`，它关闭了从任意路径加载插件、代理和服务器的标志。改为从批准的市场加载插件，或要求您的管理员删除设置。

`/plugin` **Errors** 选项卡中的相关消息是 `--plugin-dir copy of "<name>" ignored: plugin is locked by managed settings`。托管设置按名称启用或禁用该插件，Claude Code 忽略您的 `--plugin-dir` 副本，以便标志无法覆盖策略。

### `Plugins from ~/.claude/skills/ are blocked by your organization's managed settings`

您运行了 `claude plugin init` 或 `claude plugin enable`，它停止了此行。消息命名 `strictKnownMarketplaces or blockedMarketplaces` 并要求您的管理员将 `{"source":"skills-dir"}` 添加到 `strictKnownMarketplaces` 或从 `blockedMarketplaces` 中删除它。

`skills-dir` 源代表 Claude Code 从您的 `~/.claude/skills/` 目录加载的插件。要求您的管理员进行消息命名的更改。

### `Command-sourced plugins are disabled by your organization's managed settings`

您安装或更新了具有 `command` 源的插件，它停止了此行并显示 `The plugin was not installed or updated and its command was not run.`

您的管理员设置了 `disableCommandPluginSources`，因此 Claude Code 拒绝运行市场声明的生成插件的命令。仅设置 `allowManagedHooksOnly` 在 `disableCommandPluginSources` 未设置时具有相同的效果。要求您的管理员插件是否可以从策略允许的源类型发布。

### `Marketplace '<name>' is seed-managed`

您运行了 `claude plugin marketplace update <name>`，它失败，显示 `Marketplace '<name>' is seed-managed (<dir>)` 和要求您的管理员的提示。

操作员通过 `CLAUDE_CODE_PLUGIN_SEED_DIR` 预填充了此市场，Claude Code 将种子管理的市场视为只读。批量 `marketplace update` 跳过它并更新其他。

要更改市场的内容，要求维护种子镜像的人更新它。有关该过程，请参阅 [种子容器和 CI](https://code.claude.com/docs/zh-CN/plugins/org.md#seed-containers-and-ci)。

## 后续步骤

* [插件加载参考](https://code.claude.com/docs/zh-CN/plugins/loading.md)：为什么作用域、缓存和优先级的行为方式如此
* [插件命令参考](https://code.claude.com/docs/zh-CN/plugins/cli-reference.md)：`claude plugin` 命令的标志、默认值、输出和退出代码
* [安装和管理插件](https://code.claude.com/docs/zh-CN/plugins/install.md)：从开始的安装步骤
* [为您的组织管理插件](https://code.claude.com/docs/zh-CN/plugins/org.md#troubleshoot-policy)：管理员的策略端故障排除
