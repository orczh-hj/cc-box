> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# 托管和维护一个 marketplace

> 发布一个插件 marketplace，让用户可以通过它来访问，授予对私有 marketplace 的访问权限，并在推送更新和重命名后不会破坏安装。

托管一个 marketplace 意味着将你的 `marketplace.json` 目录放在其他人可以通过 `/plugin marketplace add` 添加它的地方，安装其插件，并在你推送更改后继续接收你的更新。

本页面适用于操作 marketplace 的人员。

> [!NOTE]
>
> 这些情况在其他页面上有介绍：
>
> * **你还没有编写目录文件**：从 [创建 marketplace](https://code.claude.com/docs/zh-CN/plugins/create-marketplace.md) 开始
> * **你是一个管理员，需要在你的组织机器上要求、限制或预安装 marketplace**：阅读 [为你的组织管理插件](https://code.claude.com/docs/zh-CN/plugins/org.md)

从 [托管你的 marketplace](#host-your-marketplace) 开始选择一个主机和你的用户运行的命令。在你的第一次发布之前阅读 [保持用户更新](#keep-users-up-to-date)。在你更改插件的 `name` 之前阅读 [重命名或删除插件](#rename-or-remove-a-plugin)。

## 托管你的 marketplace

你可以在 GitHub、另一个 git 主机、托管的 `marketplace.json` URL 或共享文件系统上的目录中托管 marketplace。向你的用户发送你的主机的添加命令，并告诉他们他们的机器上需要什么：

| 主机 | 用户在 Claude Code 会话中运行 | 用户需要什么 |
| :- | :- | :- |
| GitHub | `/plugin marketplace add your-org/your-marketplace` | `git`，对于私有仓库，需要 [授予对私有 marketplace 的访问权限](#grant-access-to-a-private-marketplace) 中描述的访问权限 |
| GitLab、Bitbucket、GitHub Enterprise Server 或另一个 git 主机 | `/plugin marketplace add https://gitlab.example.com/team/plugins.git` | `git` 和从他们的机器访问主机的权限。发送完整 URL，因为 `owner/repo` 简写总是指 github.com |
| 托管的 `marketplace.json` URL | `/plugin marketplace add https://plugins.example.com/marketplace.json` | 对 URL 的 HTTPS 访问。用户不需要 `git` 来获取目录本身 |
| 共享文件系统上的目录 | `/plugin marketplace add /Volumes/shared/claude-plugins` | 对路径的读取访问权限 |

要固定 GitHub 或 git-URL marketplace 的分支或标签，告诉用户追加 `#<ref>`，如 `your-org/your-marketplace#stable`。[插件命令参考](https://code.claude.com/docs/zh-CN/plugins/cli-reference.md#plugin-marketplace-add) 列出了命令接受的每种形式。

成功添加会打印 `Successfully added marketplace: your-marketplace`。Claude Code 从你的 `marketplace.json` 中的 `name` 字段获取该名称，而不是从仓库名称。

用户随后通过其条目的 `name` 和 marketplace 的 `name` 安装插件，如 `/plugin install code-formatter@your-marketplace`。

### 为仓库中的每个人注册 marketplace

要与在一个仓库中工作的每个人共享 marketplace，请从你的 shell 在那里运行一次 `claude plugin marketplace add your-org/your-marketplace --scope project`，并提交它写入的 `.claude/settings.json`。Claude Code 随后为每个 [信任该文件夹](https://code.claude.com/docs/zh-CN/plugins/org.md#require-plugins-per-repository) 的队友注册 marketplace。

### 避免在 URL 托管的 marketplace 中使用相对路径条目

当用户将你的 marketplace 添加为裸 `marketplace.json` URL 时，Claude Code 仅下载该文件。你的 `plugins` 数组中的条目，其 `source` 是相对路径（如 `./plugins/formatter`），则在安装时会失败，出现 [`其 marketplace 条目路径不会停留在 marketplace 目录内`](https://code.claude.com/docs/zh-CN/plugins/troubleshooting#plugins-with-relative-paths-fail-in-url-based-marketplaces)。给每个条目一个可以独立获取的源，如 `github` 仓库或 `archive` URL，或在 git 仓库中托管 marketplace，以便 Claude Code 克隆整个树。

### 保持托管文件的下载限制内

当用户将你的 marketplace 添加为 `marketplace.json` URL，或安装具有 [`archive`](https://code.claude.com/docs/zh-CN/plugins/marketplace-reference#archive-plugin-source) 源的条目时，Claude Code 从你的服务器下载文件。下载超过此表中的限制会失败，因此请调整你的文件大小并配置你的服务器以保持在限制内。

| 文件 | 最大下载 | 你的服务器响应时间 | 重定向 |
| :- | :- | :- | :- |
| 来自 `url` marketplace 源的 `marketplace.json` | 5 MiB | 10 秒 | 重定向到不同源必须使用 `https://` 且不能指向环回、链路本地或云元数据主机，因此从 `https://` 重定向到 `http://` 会失败 |
| 来自 `archive` 插件源的 Zip | 256 MiB | 120 秒 | 最多五个。每个重定向目标必须使用 `https://` 且不能指向环回、链路本地或云元数据主机 |

重定向发送到不同源的请求不会携带你在 marketplace 源或插件条目上配置的任何标头。

存档下载后，当 zip 超过以下任何提取限制时，安装会失败：

* **条目**：100,000 个文件和目录
* **文件大小**：任何一个文件 512 MiB，未压缩
* **总大小**：1 GiB 未压缩
* **压缩比**：未压缩内容是 zip 大小的 50 倍

### 在共享目录上就地编辑插件

当用户从共享目录添加你的 marketplace 时，Claude Code 直接从该目录读取具有相对路径源的插件，而不是复制它们。用户在下次启动会话或运行 `/reload-plugins` 时会看到你的编辑，无需更新步骤或版本提升。

### 将插件文件保留在 Git LFS 之外

将你的插件需要的文件保留在 [Git LFS](https://git-lfs.com) 之外。当用户从 git 仓库中托管的 marketplace 添加或安装它列出的基于 git 的插件时，Claude Code 会将该 marketplace 或插件仓库克隆到他们的机器上。克隆永远不会下载 LFS 内容，因此 LFS 跟踪的文件会作为指针文件到达。

### 使用符号链接在 marketplace 内共享文件

要在你的插件和同一 marketplace 的其他部分之间共享文件，请在你的插件目录内创建符号链接。当 Claude Code 将插件复制到其缓存中时，它通过目标解析的位置处理每个符号链接：

* **在插件自己的目录内**：符号链接在缓存中被保留为相对符号链接，因此它在运行时继续解析到复制的目标。
* **在同一 marketplace 内的其他地方**：符号链接被解引用。目标的内容被复制到缓存中以代替它。这允许元插件的 `skills/` 目录链接到 marketplace 中其他插件定义的技能。
* **在 marketplace 外**：符号链接因安全原因被跳过。

对于从本地路径安装的插件，或从 [`command` 源](https://code.claude.com/docs/zh-CN/plugins/marketplace-reference#command-plugin-source)（其 `mode` 是默认 `copy`）安装的插件，Claude Code 仅保留在插件自己目录内解析的符号链接，并跳过所有其他的。

以下命令创建从 marketplace 插件内部到由兄弟插件定义的共享技能的链接。在 Windows 上，从提升的命令提示符使用 `mklink /D` 或启用开发者模式：

```bash theme={null}
ln -s ../../shared-plugin/skills/foo ./skills/foo
```

## 通过组织设置分发

在 Team 或 Enterprise 计划上，你也可以通过 claude.ai 上的 [**组织设置 > 插件和技能**](https://claude.ai/admin-settings/skills?tab=inventory) 分发 marketplace，而不是在用户自己添加的地方托管它。组织同步通过你的组织在 claude.ai 上的 GitHub 或 GitLab 连接读取仓库，因此你的用户的 git 凭证不涉及。

组织同步对仓库的要求比 `/plugin marketplace add` 更严格：

* **Marketplace 仓库**：在 github.com 和 gitlab.com 上，它必须是私有或内部的
* **插件源**：组织同步仅接受某些 [源类型](https://code.claude.com/docs/zh-CN/plugins/marketplace-reference.md#plugin-sources)
* **顶级 `bin/` 目录**：claude.ai 拒绝具有一个的插件并同步 marketplace 的其余部分。错误消息以 `Plugin contains a top-level bin/ directory` 开头。将可执行文件保留在另一个目录中，如 `scripts/`，并从你的 hooks 或 MCP 服务器配置中将它们引用为 `${CLAUDE_PLUGIN_ROOT}/scripts/<name>`

[从仓库同步你的组织的插件](https://claude.com/docs/plugins/org-sync) 在 claude.com 上列出了接受的源、GitLab 设置和 `bin/` 错误，[为你的组织管理插件](https://claude.com/docs/plugins/admin) 涵盖了管理员工作流程。

## 授予对私有 marketplace 的访问权限

当用户添加、从或更新你的 marketplace 时，Claude Code 在他们的机器上运行 `git`，交互式提示关闭，并依赖该机器已经持有的任何凭证。Claude Code 没有自己的 git 令牌，`marketplace.json` 也没有字段来存储一个。

你通过发送给用户的添加命令的形式选择克隆是通过 SSH 还是 HTTPS 运行：

* **GitHub `owner/repo`**：Claude Code 探测 `ssh -T git@github.com`，当探测成功时通过 SSH 克隆。如果探测失败，或 SSH 克隆本身失败，它通过 HTTPS 克隆。没有 GitHub SSH 密钥的机器上的用户可以设置 `CLAUDE_CODE_PLUGIN_PREFER_HTTPS=1` 来跳过探测并通过 HTTPS 克隆。
* **`git@host:path.git`**：SSH。
* **`https://example.com/repo.git`**：HTTPS。

告诉用户每个协议在他们的机器上需要什么：

* **SSH**：密钥必须在没有密码提示的情况下工作，例如因为它被加载到 `ssh-agent` 中。主机必须已经在 `known_hosts` 中。
* **HTTPS**：Claude Code 会保持用户的 git 凭据助手处于启用状态，但禁止其进行提示，并且不会运行 askpass 程序，例如在 `GIT_ASKPASS` 中设置的程序。助手已存储的凭据可以正常使用；需要助手询问才能获得的凭据则会失败。在 GitHub 上，先运行 `gh auth login`，再运行 `gh auth setup-git`，即可存储凭据。

对于 GitHub Enterprise Server 主机，用户需要从他们的机器访问该主机的 git 访问权限。有关每个 Claude Code 表面需要到达 GHES 托管的 marketplace 的内容，请参阅 [GHES 上的插件 marketplace](https://code.claude.com/docs/zh-CN/github-enterprise-server.md#plugin-marketplaces-on-ghes)。

如果你改为通过 claude.ai 上的 **组织设置 > 插件和技能** 分发，你的用户的 git 凭证不涉及。请参阅 [通过组织设置分发](#distribute-through-organization-settings)。

### 为没有 git 主机账户的用户提供服务

没有 git 主机账户的用户可以将你提供的 marketplace 添加为 `marketplace.json` URL 或从共享目录，但他们只能安装其条目源他们也可以到达的插件。指向私有 `github` 仓库的条目在安装时仍然对他们失败，因为 Claude Code 使用与 git 托管的 marketplace 相同的非交互式 `git` 获取它。

这些条目源不需要 git 账户：

* **`archive`**：通过 HTTPS 下载的 zip。用户既不需要 `git` 也不需要账户，只需要对 URL 的网络访问。需要 Claude Code v2.1.224 或更高版本。用 `sha256` 固定每个存档，以便 Claude Code 拒绝更改的下载。要随下载发送凭证，请参阅 [认证存档下载](#authenticate-archive-downloads)。
* **公共 git 仓库**：当条目给出 `https://` URL 时，Claude Code 会通过 HTTPS 克隆公共 `url` 或 `git-subdir` 源，无需凭据。对于 `github` 源，或写成 `owner/repo` 形式的 `git-subdir` 源，请告知没有 GitHub SSH 密钥的用户设置 `CLAUDE_CODE_PLUGIN_PREFER_HTTPS=1`。

即使在没有 SSH 密钥的机器上从 shell 运行 `claude plugin install` 不设置该变量也能成功，也请在您的说明中保留 `CLAUDE_CODE_PLUGIN_PREFER_HTTPS=1`。对于 `github` 源，该命令可能会自行回退到 HTTPS，并输出 `SSH not configured, cloning via HTTPS`。在会话内通过 `/plugin` 进行的安装以及插件更新不会回退，因此如果不设置该变量，没有 GitHub SSH 密钥的用户会遇到失败。

对于一个网络上的团队，共享文件系统上的 `directory` marketplace 也可以在没有 git 账户的情况下工作。用户只需要对路径的读取访问权限。

### 后台自动更新对凭证的处理

后台自动更新是 Claude Code 在会话启动后对 marketplace 和已安装插件的无人值守刷新。对于你的 marketplace，它默认关闭，直到用户或管理员打开它，如 [保持用户最新](#keep-users-up-to-date) 中所述。

当它对私有 marketplace 打开时，新提交的后台检查使用用户配置的 git 凭证助手，永远不会提示。每种远程和助手给出不同的结果：

* **SSH 远程**：加载到 `ssh-agent` 中的密钥认证检查。
* **具有存储凭证的 HTTPS 远程**：可以在不提示的情况下提供存储凭证的助手认证检查。Git Credential Manager、macOS Keychain 助手和 `git-credential-store` 一旦为主机持有凭证就以这种方式工作。
* **具有需要提示的助手的 HTTPS 远程**：助手无法在后台回答。更新失静地失败，现有检出保持原位，因此用户的插件继续从最后同步的状态工作。

检查后，Claude Code 执行以下操作之一：

* **检出是最新的**：Claude Code 保持原样。
* **检查找到新提交，或因为无法到达或认证到远程而失败**：Claude Code 再次克隆 marketplace 并用新克隆替换现有检出。如果该克隆失败，现有检出保持原位。重新克隆可以 [在大型仓库上超时](https://code.claude.com/docs/zh-CN/plugins/troubleshooting.md#git-clone-timed-out-after-120s)。

要保持私有 marketplace 最新，用户可以执行以下任一操作：

* **存储凭证**：首先登录凭证助手，以便它为主机持有凭证。对于 GitHub，运行 `gh auth login`，然后 `gh auth setup-git`。
* **在失败时保持检出**：如果用户设置 `CLAUDE_CODE_PLUGIN_KEEP_MARKETPLACE_ON_FAILURE=1`，当后台检查无法到达或认证到远程时，Claude Code 保持现有检出而不尝试重新克隆。插件继续从最后同步的状态工作。

如果用户在环境中设置 `GITHUB_TOKEN` 或另一个提供商令牌，仅这一点不会认证后台检查。令牌通过凭证助手（如 `gh` CLI 的助手）生效，该助手读取 `GH_TOKEN` 和 `GITHUB_TOKEN`。

## 向整个公司推出

向公司推出插件涉及你作为 marketplace 所有者、控制托管设置的管理员和使用 Claude Code 的每个人。你可以在没有管理员的情况下运行推出，在这种情况下每个人自己添加 marketplace 并安装插件。

| 谁 | 他们做什么 | 它在哪里被覆盖 |
| :- | :- | :- |
| 你，marketplace 所有者 | 将目录保留在只有公司可以读取的仓库中，发送你的主机的添加命令，并说明每个人的机器上需要什么 | [托管你的 marketplace](#host-your-marketplace) 和 [授予对私有 marketplace 的访问权限](#grant-access-to-a-private-marketplace) |
| 管理员 | 使用托管设置中的 `extraKnownMarketplaces` 和 `enabledPlugins` 为每个人注册 marketplace 并打开其插件，并在那里设置 `autoUpdate` | [要求一个 marketplace 及其插件](https://code.claude.com/docs/zh-CN/plugins/org.md#require-a-marketplace-and-its-plugins) 和 [设置更新策略](https://code.claude.com/docs/zh-CN/plugins/org.md#set-update-policy) |
| 每个人 | 需要对私有 git 仓库的读取访问权限，凭证已经存储在他们的机器上。没有管理员，他们也运行添加和安装命令 | [添加私有 marketplace](https://code.claude.com/docs/zh-CN/plugins/install.md#add-a-private-marketplace) |

对于没有 git 主机账户的人，这些部分各覆盖一种到达他们的方式：

* **不需要 git 账户的条目源**：[为没有 git 主机账户的用户提供服务](#serve-users-who-have-no-git-host-account)
* **预填充的插件目录**：[为容器和 CI 播种](https://code.claude.com/docs/zh-CN/plugins/org.md#seed-containers-and-ci)，也为没有 git 主机账户的用户提供服务
* **claude.ai 组织设置**：[通过组织设置分发](#distribute-through-organization-settings)，你的用户的 git 凭证不涉及

## 保持用户最新

你的更改通过后台自动更新到达用户，一旦它对你的 marketplace 打开，或当用户自己更新插件时。在两种情况下，用户只有在其计算版本改变时才获得插件的新副本，如 [发布新版本](#release-a-new-version) 中所述。

### 打开自动更新

后台自动更新默认对你的 marketplace 关闭，`marketplace.json` 没有字段来打开它。用户或管理员打开它：

* **告诉用户打开它**：每个用户进入 `/plugin` 中的 **Marketplaces**，选择你的 marketplace，并选择 **启用自动更新**。
* **要求管理员设置它**：如果管理员在托管设置中的你的 marketplace 的 `extraKnownMarketplaces` 条目上设置 `"autoUpdate": true`，它对接收这些设置的每个人都打开。请参阅 [设置更新策略](https://code.claude.com/docs/zh-CN/plugins/org.md#set-update-policy)。

没有自动更新，用户在会话中运行 `/plugin marketplace update <name>` 或在 shell 中运行 `claude plugin update <plugin>@<name>` 时接收你的更改。

对于用户在更新到达他们时看到的内容，请参阅 [自动更新何时运行](https://code.claude.com/docs/zh-CN/plugins/loading.md#when-auto-update-runs)。

### 发布新版本

要向用户发布新版本，更改插件的 `version`。用户只有在插件的计算版本与他们拥有的版本不同时才获得新副本。该版本首先来自 `plugin.json`，然后来自 marketplace 条目，根据 [版本和更新](https://code.claude.com/docs/zh-CN/plugins/loading.md#versions-and-updates)。

如果用户从通过本地路径添加的市场中[就地加载](https://code.claude.com/docs/zh-CN/plugins/loading.md#find-plugins-on-disk)插件，该插件不受 `version` 控制。无论其版本字符串是什么，它都会在每次会话启动时加载您当前的文件。

对于除了就地加载或来自 `command` 源的安装之外的每次安装，要么在每次发布时增加 `version`，要么省略它：

* **在每次发布时提升 `version`**：用户停留在他们的缓存副本上，直到字符串改变。如果你设置 `"version": "1.0.0"` 并推送新提交而不改变它，用户不会接收它们。
* **省略 `version`**：用户改为跟踪你的提交。将 `version` 保留在 `plugin.json` 和 marketplace 条目之外。

不要在 `plugin.json` 和 marketplace 条目中都设置 `version`。如果你这样做，Claude Code 使用 `plugin.json` 值而不警告，`claude plugin validate` 报告不匹配为 `Entry declares version "<a>" but <path>/plugin.json says "<b>"`。

### 将用户保持在一个版本上

一个 marketplace 一次为每个插件提供一个版本，所以你通过选择每个条目指向什么来将用户保持在一个版本上：

* **插件条目上的 `ref` 和 `sha`**：`ref` 命名分支或标签，`sha` 为 `github`、`url` 或 `git-subdir` 源命名提交。请参阅 [插件源](https://code.claude.com/docs/zh-CN/plugins/marketplace-reference.md#plugin-sources)。
* **添加命令上的 `#<ref>`**：添加 `your-org/your-marketplace#stable` 的用户获得该目录的分支或标签。对于两条发布线同时进行，请参阅 [运行发布渠道](#run-release-channels)。
* **`<plugin>--v<version>` 标签**：依赖的版本范围针对这些标签解析。请参阅 [发布其他人依赖的插件](https://code.claude.com/docs/zh-CN/plugins/dependencies.md#tag-plugin-releases-for-version-resolution)。

[发布新版本](#release-a-new-version) 说明更改的条目何时到达用户。

### 更改命令源的命令

如果你更改 [`command` 源](https://code.claude.com/docs/zh-CN/plugins/marketplace-reference#command-plugin-source) 的 `command`，或切换其 `mode`，每个用户必须在 Claude Code 运行它之前接受新命令。Claude Code 仅运行用户在安装或最后更新插件时接受的确切命令。

在用户的 marketplace 副本获取更改后，该用户看到以下内容：

* **不再有后台运行**：该用户的命令的 [每会话一次运行](https://code.claude.com/docs/zh-CN/plugins/loading.md#when-a-command-source-re-runs) 停止，因此工具的新输出不会到达他们。
* **`/plugin` 错误选项卡中的条目**：条目显示新命令和要运行的 `claude plugin update` 命令。

告诉用户在终端中运行该条目显示的 `claude plugin update` 命令。Claude Code 向他们显示新命令并要求他们接受它。

## 运行发布渠道

要提供稳定和早期访问轨道，托管两个 marketplace，其条目指向同一插件的不同 ref，并让每个用户添加他们想要的。Claude Code 没有发布渠道概念，一个 marketplace 一次为每个插件提供一个版本。

给两个 `marketplace.json` 文件不同的 `name` 值。Claude Code 通过其 `name` 识别 marketplace，所以用户一次不能有两个具有相同名称的 marketplace 注册。

使用这两个目录，添加 `stable-tools` 的用户从 `stable` 分支安装 `code-formatter`，添加 `latest-tools` 的用户从 `latest` 安装它：

```json theme={null}
{
  "name": "stable-tools",
  "owner": { "name": "Your Org" },
  "plugins": [
    { "name": "code-formatter", "source": { "source": "github", "repo": "your-org/code-formatter", "ref": "stable" } }
  ]
}
```

```json theme={null}
{
  "name": "latest-tools",
  "owner": { "name": "Your Org" },
  "plugins": [
    { "name": "code-formatter", "source": { "source": "github", "repo": "your-org/code-formatter", "ref": "latest" } }
  ]
}
```

给两个 ref 不同的 `plugin.json` 版本，或省略 `version` 以便提交 SHA 区分它们。更新通过比较版本检测，所以在没有版本改变的情况下移动的 ref 将用户留在缓存副本上。

要将渠道分配给用户组而不是让用户选择，管理员给每个组匹配的 `extraKnownMarketplaces` 条目，如 [设置更新策略](https://code.claude.com/docs/zh-CN/plugins/org.md#set-update-policy) 中所述。

## 重命名或删除插件

插件的 `name` 是其标识符。用户在 `enabledPlugins` 和 `pluginConfigs` 设置键以及 `/plugin install` 中引用它，所以改变它会破坏每次现有安装。

要更改用户在 `/plugin` 中看到的标签而不破坏任何东西，在 `plugin.json` 中设置 `displayName` 并保持 `name` 不变。

### 使用重命名映射迁移用户

当您必须更改 `name` 时，向 `marketplace.json` 添加顶级 `renames` 映射，以便 Claude Code 迁移现有用户，而不是报告 [`Plugin "<name>" not found in marketplace`](https://code.claude.com/docs/zh-CN/plugins/troubleshooting#plugin-not-found-in-marketplace)。当您从 `plugins` 中删除条目时也应这样做。

将每个前名称映射到其当前名称，或在插件消失时映射到 `null`。此 marketplace 将 `formatter` 重命名为 `code-formatter` 并记录 `legacy-linter` 被删除：

```json theme={null}
{
  "name": "your-marketplace",
  "owner": { "name": "Your Org" },
  "plugins": [
    { "name": "code-formatter", "source": "./plugins/code-formatter" }
  ],
  "renames": {
    "formatter": "code-formatter",
    "legacy-linter": null
  }
}
```

在你推送后，仍然启用旧名称的用户看到以下结果之一：

* **重命名的条目**：插件在其新名称下加载。`claude plugin list` 和 `/plugin` 下的插件详情显示 `Renamed to "code-formatter" in the "your-marketplace" marketplace` 一次，Claude Code 在用户、项目和本地设置范围中将旧键重写为 `enabledPlugins` 和 `pluginConfigs` 中的新键。
* **`null` 条目**：旧键从这些范围中删除，用户看到 `Removed from the "your-marketplace" marketplace`。
* **在托管设置中启用**：插件仍然在其新名称下加载，但 Claude Code 无法重写托管设置，所以通知重复出现，直到管理员在那里更新 `enabledPlugins`。

对于用户从 git 仓库或 URL 添加的 marketplace，重命名的插件报告 [`Plugin "<name>" not cached at <path>`](https://code.claude.com/docs/zh-CN/plugins/troubleshooting#plugin-not-cached-at)，直到用户在会话中运行 `/plugin install code-formatter@your-marketplace` 一次。

将 `renames` 视为仅追加历史。在每个人迁移后保持旧条目。当你再次重命名时，添加第二个条目而不是编辑第一个，因为 Claude Code 遵循从最旧名称的链。

在你的 shell 中，编辑映射后运行 `claude plugin validate .`。它拒绝循环或在 `null` 或 `plugins` 中的名称以外的任何地方结束的链，出现 `renames.<name>: chain does not resolve`。

### 从用户机器卸载已删除的插件

要从用户机器卸载已删除的插件而不是留下副本，在 `marketplace.json` 的顶级设置 `"forceRemoveDeletedPlugins": true`。没有该字段，已删除的插件保持安装并在会话加载它时报告 `Plugin "<name>" not found in marketplace`。有了它，Claude Code 在每次会话启动时执行以下操作：

1. 比较用户从你的 marketplace 安装的内容与条目和 `renames` 映射，并将既不列出也不重命名的任何插件视为已删除。
2. 从用户、项目和本地范围卸载每个已删除的插件。仅托管设置安装的插件保持原位。
3. 在 `/plugin` 中的 **Flagged** 标题下列出每个已删除的插件，状态为 `Removed from marketplace`。

## 认证存档下载

要认证 [`archive`](https://code.claude.com/docs/zh-CN/plugins/marketplace-reference#archive-plugin-source) 下载，如从私有注册表下载，设置 Claude Code 随其发送的 HTTP 标头。你可以在以下任一位置设置 `headers`：

* **Marketplace 的 `url` 源**：你注册 marketplace 的 `url` 源，如 [`extraKnownMarketplaces`](https://code.claude.com/docs/zh-CN/settings-reference#extraknownmarketplaces) 条目。
* **插件的条目**：在 Claude Code v2.1.238 或更高版本上，你可以改为在插件的 `marketplace.json` 条目上设置它，在 `source` 旁边。

在任一位置，当值是短期的（如你的注册表生成的令牌）时，设置 `headersHelper` 命令而不是 `headers`。Claude Code 运行命令并将其打印的 JSON 对象作为该位置的标头发送。需要 Claude Code v2.1.238 或更高版本。

[marketplace 参考](https://code.claude.com/docs/zh-CN/plugins/marketplace-reference.md#plugin-entries) 列出 `headers` 和 `headersHelper` 条目字段。

你选择的位置决定哪些下载获得标头以及 Claude Code 何时运行命令：

| 位置 | 获得标头的下载 | Claude Code 何时运行那里设置的 `headersHelper` |
| :- | :- | :- |
| Marketplace `url` 源 | 在 marketplace URL 的源上的存档下载，意味着相同的方案、主机和端口 | 在 marketplace 的 `marketplace.json` 的每次获取之前和在该源上的每次存档下载之前。Claude Code 重用一次运行的输出长达 60 秒 |
| 插件条目 | 该条目的下载仅 | 仅当用户自己安装或更新该一个插件并 [接受命令](#how-users-accept-a-headershelper-command) 时 |

当两个位置都设置相同名称的标头时，Claude Code 发送条目的值。在一个位置内，命令打印的标头覆盖相同名称列出的标头。

### 向插件条目添加 headersHelper

此条目在 `source` 旁边设置 `headersHelper`。它也设置 [`"strict": false`](https://code.claude.com/docs/zh-CN/plugins/marketplace-reference#strict-mode)，Claude Code 要求设置 `headersHelper` 的 `marketplace.json` 条目：

```json theme={null}
{
  "name": "my-plugin",
  "description": "Formatting commands for internal services",
  "strict": false,
  "source": {
    "source": "archive",
    "url": "https://registry.example.com/plugins/my-plugin-2.1.0.zip"
  },
  "headersHelper": "/opt/bin/mint-registry-token.sh"
}
```

要检查条目，在你的 shell 中运行 `claude plugin install my-plugin@your-marketplace`。Claude Code 向你显示命令和存档 URL，并在你接受后下载 zip。

### 编写 headersHelper 命令

无论你在 marketplace 的 `url` 源还是插件条目上设置 `headersHelper`，编写命令以满足这些要求：

* **命令文本**：最多 500 个可打印 ASCII 字符，没有四个或更多空格的运行。
* **输出**：在 stdout 上打印一个标头名称和字符串值的 JSON 对象，然后在 10 秒内退出 0。
* **Shell 和工作目录**：Claude Code 通过 `sh` 运行命令，或在 Windows 上通过 `cmd.exe`。工作目录是配置目录，即 `~/.claude` 或 [`CLAUDE_CONFIG_DIR`](https://code.claude.com/docs/zh-CN/env-vars#variables)。给出绝对路径或 `PATH` 上的命令，因为相对路径针对该目录解析，而不是用户的项目。
* **Claude Code 删除的变量**：当命令在 `marketplace.json` 条目中设置，或在项目的 `.claude/settings.json` 或 `.claude/settings.local.json` 中设置时，Claude Code 从环境中删除每个名称看起来像凭证的变量，通过 [它应用于 MCP `headersHelper` 的相同规则](https://code.claude.com/docs/zh-CN/mcp#which-variables-a-helper-can-read)。`ANTHROPIC_API_KEY` 和 `MY_REGISTRY_TOKEN` 都被删除，所以让命令从文件或凭证存储读取其凭证。此删除不适用于在用户设置、`--settings` 文件或托管设置中设置的命令。
* **Claude Code 设置的变量**：对于 `url` 源的命令为 `CLAUDE_CODE_MARKETPLACE_URL` 和 `CLAUDE_CODE_MARKETPLACE_NAME`，对于条目的命令为 `CLAUDE_CODE_PLUGIN_NAME` 和 `CLAUDE_CODE_PLUGIN_ARCHIVE_URL`。`CLAUDE_CODE_MARKETPLACE_NAME` 在用户通过 URL 添加 marketplace 后的第一次获取时未设置，因为该获取是提供名称的。

铸造承载令牌的命令打印像这样的对象：

```json theme={null}
{"Authorization": "Bearer eyJhbGciOiJSUzI1NiJ9"}
```

### 当 Claude Code 跳过 headersHelper 命令或删除其输出时

当以下任一情况适用时，`headersHelper` 命令不运行，或来自 `headers` 或命令输出的标头被删除：

* **命令失败**：如果命令退出非零、运行超过 10 秒或打印除 JSON 对象的字符串值以外的任何内容，命令运行的获取或下载不会发生。
* **Marketplace URL 不以 `https://` 开头**：该 `url` 源的命令不运行，请求仅携带其 `headers` 字段中列出的标头。
* **重定向离开源**：当下载被重定向离开存档 URL 的源时，重定向的请求不携带来自 marketplace `url` 源或插件条目的 `headers` 值或命令输出。
* **条目设置路由或身份标头**：Claude Code 从条目的 `headers` 和命令输出中删除请求路由和客户端身份名称，如 `Host`、`Cookie` 和 `X-Forwarded-*`，并保持认证名称，如 `Authorization`。每个 `marketplace.json` 条目都以这种方式过滤。对于设置中的内联插件条目，请参阅 [`extraKnownMarketplaces`](https://code.claude.com/docs/zh-CN/settings-reference#extraknownmarketplaces)。
* **命令在 `--add-dir` 目录的设置中设置**：命令被忽略，在 `url` 源和 [内联插件条目](https://code.claude.com/docs/zh-CN/settings-reference.md#extraknownmarketplaces) 上，仅该文件的 `headers` 被发送。
* **托管设置阻止命令**：将 [`disableCommandPluginSources`](https://code.claude.com/docs/zh-CN/settings-reference#disablecommandpluginsources) 设置为 `true` 阻止 `headersHelper` 命令，[`allowManagedHooksOnly`](https://code.claude.com/docs/zh-CN/settings-reference#allowmanagedhooksonly) 也阻止它们，除非 `disableCommandPluginSources` 明确为 `false`。在任一块下，Claude Code 仍然为托管设置本身声明的 marketplace 运行命令。

### 用户如何接受 headersHelper 命令

用户每次单独安装或更新该插件时，都需要接受插件条目的命令。Claude Code 会显示命令和存档 URL，并仅在用户接受后运行命令。

用户可以在终端中的 Claude Code 会话内、在未运行会话的 shell 中，或在 VS Code 扩展中安装或更新插件：

* **终端会话**：从 `/plugin` 中该插件自己的视图。
* **Shell**：使用 `claude plugin install` 或 `claude plugin update`。
* **VS Code 扩展**：从 [**Manage plugins** 对话框](https://code.claude.com/docs/zh-CN/vs-code.md#manage-plugins)，需要扩展版本 2.1.290 或更高版本。

在非交互式 shell 中，传递 [`--yes`](https://code.claude.com/docs/zh-CN/plugins/cli-reference#plugin-install) 以接受命令。要仅接受之前 `--json` 运行显示的命令，传递 [`--accept-command`](https://code.claude.com/docs/zh-CN/plugins/cli-reference#plugin-install) 与运行报告的 `sha256`。

Claude Code 仅针对它显示的存档 URL 运行它显示的命令。如果条目的命令或存档 URL 在此期间发生了变化，Claude Code 会拒绝安装或更新。仅查询字符串的更改不计在内，但在 VS Code 扩展中或使用 `--accept-command` 时除外。

### 拒绝命令而不是询问的安装和更新

在除单个插件安装或更新之外的任何操作上，Claude Code 既不运行条目的命令也不下载其存档。插件保持在其安装版本或保持未安装，用户看到以下结果之一：

* **一次安装多个插件、来自插件建议或作为另一个插件的依赖**：Claude Code 拒绝具有命令的插件并将用户指向 `/plugin` 中的该插件自己的视图。批量安装中的其他插件仍然安装。依赖于被拒绝插件的插件无法安装，直到用户自己安装被拒绝的插件。
* **后台自动更新，或会话启动以获取其存档从未下载的插件**：Claude Code 在 `/plugin` 错误选项卡中列出插件，以便用户知道自己安装或更新它。

### 当 marketplace `url` 源的命令运行时

你在设置文件中声明 marketplace `url` 源的 `headersHelper`，如 [`extraKnownMarketplaces`](https://code.claude.com/docs/zh-CN/settings-reference#extraknownmarketplaces) 条目，而不是在 marketplace 发布的目录中。Claude Code 因此不会在每次安装或更新时要求用户接受它。相反，声明它的设置文件决定 Claude Code 何时运行它：

| 设置文件 | Claude Code 何时运行命令 |
| :- | :- |
| 用户设置、`--settings` 文件或机器上的托管设置文件 | 不询问，包括在后台 marketplace 刷新期间 |
| 项目的 `.claude/settings.json` 或 `.claude/settings.local.json` | 仅在用户接受该文件夹本身的 [工作区信任对话](https://code.claude.com/docs/zh-CN/permissions.md#what-runs-before-you-trust-a-folder) 后。`-p` 或 SDK 会话不计为接受它，对父文件夹授予的信任也不计 |
| 服务器托管设置 | 在交互式会话中，仅在用户在 [安全批准对话](https://code.claude.com/docs/zh-CN/server-managed-settings.md#security-approval-dialogs) 中批准交付的设置后 |

对于这些文件中的 [内联插件条目](https://code.claude.com/docs/zh-CN/settings-reference.md#extraknownmarketplaces)，Claude Code 要求与该文件中 marketplace 级别命令相同的文件夹信任或设置批准，用户也在每次安装或更新时接受条目的命令。

## 依赖和推荐其他插件

条目可以声明对其他插件的依赖。

* **版本范围**：依赖可以携带 semver 范围。
* **跨 marketplace 依赖**：来自另一个 marketplace 的依赖仅在你的 marketplace 在 `allowCrossMarketplaceDependenciesOn` 中列出该 marketplace 时安装。

对于版本范围、它们解析的 `<plugin>--v<version>` git 标签约定和跨 marketplace 信任，请参阅 [插件依赖](https://code.claude.com/docs/zh-CN/plugins/dependencies.md)。

要在项目匹配时让 Claude Code 建议某个插件，请向条目添加 `relevance` 块，其中包含用于识别该项目的信号。仅当管理员在 `pluginSuggestionMarketplaces` 中列出您的市场时，用户才会看到这些建议。有关信号和启用步骤，请参阅[插件相关性](https://code.claude.com/docs/zh-CN/plugins/relevance.md)。

## 解决 marketplace 无法做的事情

一些所有者要求的东西在 `marketplace.json` 中没有字段。以下是每个的最接近选项：

* **限制用户安装的其他内容**：marketplace 允许列表是托管设置 `strictKnownMarketplaces`。请参阅 [限制用户可以安装的内容](https://code.claude.com/docs/zh-CN/plugins/org.md#restrict-what-users-can-install)。
* **在用户询问之前安装或启用插件**：没有条目字段安装插件。托管 `enabledPlugins` 为一个舰队执行此操作；请参阅 [预安装和要求插件](https://code.claude.com/docs/zh-CN/plugins/org.md#pre-install-and-require-plugins)。
* **向不同用户显示不同的条目**：条目不携带受众字段，添加 marketplace 的每个用户看到整个目录。为不同的受众托管单独的 marketplace。
* **标记插件已弃用**：没有弃用状态。选项是删除条目，在 `renames` 中将其名称映射到 `null`，并可选地设置 `forceRemoveDeletedPlugins`。
* **为你的用户打开自动更新**：每个用户在 `/plugin` 中的 **Marketplaces** 下打开它，或管理员在托管设置中设置 `autoUpdate`。请参阅 [打开自动更新](#turn-on-auto-update)。
* **携带 git 凭证**：没有 marketplace 字段持有 git 令牌。对 git 托管的 marketplace 或插件的访问遵循用户的 git 设置，根据 [授予对私有 marketplace 的访问权限](#grant-access-to-a-private-marketplace)。对于 `archive` 源，条目可以改为设置 [`headers` 或 `headersHelper`](#authenticate-archive-downloads)。

## 后续步骤

* [Marketplace 参考](https://code.claude.com/docs/zh-CN/plugins/marketplace-reference.md)：`marketplace.json` 字段、源类型和验证消息
* [为你的组织管理插件](https://code.claude.com/docs/zh-CN/plugins/org.md)：在你的组织的机器上要求、限制或播种你的 marketplace
* [插件依赖](https://code.claude.com/docs/zh-CN/plugins/dependencies.md)：标记发布，以便依赖你的插件的插件可以解析版本
* [排查插件问题](https://code.claude.com/docs/zh-CN/plugins/troubleshooting.md)：你的用户在从你的 marketplace 添加或更新时看到的错误
