> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# 发布和分发插件

> 通过您自己的市场或 Anthropic 的目录发布 Claude Code 插件，包括发布前检查清单以及用户如何获取更新。

发布 Claude Code 插件意味着在市场中列出它，市场是一个 JSON 目录，列出插件及其获取位置，这样其他人可以按名称安装它并接收您的更新。您可以运行自己的市场或将您的插件提交到 Anthropic 的目录。要在不发布的情况下共享插件，请将插件的目录或其 `.zip` 发送给人们以供他们自己加载。

本页面适用于已准备好共享的工作插件的作者。

> [!NOTE]
>
> 这些情况在其他页面上有介绍：
>
> * **您的插件还未完成**：从[创建插件](https://code.claude.com/docs/zh-CN/plugins/create.md)开始
> * **您维护的 CLI 或 SDK 在官方市场中有插件**：请参阅[从您的 CLI 推荐您的插件](https://code.claude.com/docs/zh-CN/plugins/cli-hints.md)

从[选择如何分发](#choose-how-to-distribute)开始，比较分发选项。如果您已经知道您的路线，请转到[为发布准备您的插件](#prepare-your-plugin-for-release)，然后按照您的路线部分了解要告诉用户什么以及他们如何接收您的更新。

## 选择如何分发

根据谁需要安装插件来选择分发选项：

| 路线 | 谁可以安装 | 您需要什么 | 用户是否自动获取您的更新？ |
| :- | :- | :- | :- |
| [无市场](#share-a-plugin-without-a-marketplace) | 您发送插件文件夹或其 `.zip` 的人 | 插件的文件夹 | 无。他们加载您发送的副本 |
| [您自己的市场](#publish-through-your-own-marketplace) | 任何可以访问存储库的人，可以是您的团队可以克隆的私有存储库 | 一个 git 存储库或其他具有列出您的插件的 `.claude-plugin/marketplace.json` 的主机 | 关闭 |
| [Anthropic 的目录](#submit-to-anthropics-directory) | 在 claude.ai 或 Cowork 中添加它的人。它也通过[账户同步](https://code.claude.com/docs/zh-CN/plugins/loading.md#synced-plugins)在他们的 Claude Code 会话中加载 | 一个包含插件的 GitHub 存储库和一个付费的 claude.ai 计划以从中提交 | 是，在您推送的版本发布后 |

自动更新是用户端的每个市场设置，在后台获取新版本。

## 为发布准备您的插件

名称、版本、验证和从市场安装决定了发布是否对安装它的人有效。在第一次发布前检查它们，以及在之后的每次发布前再次检查。

**1. 选择永久名称**

用户通过 `name@marketplace` 安装、启用和配置您的插件，因此重命名的插件对每个现有安装都是不同的插件。选择一个 kebab-case 名称，例如 `deploy-helper`，因为 `claude plugin validate` 会对其他形式发出警告，并将其视为永久的。在 `plugin.json` 中设置 `displayName` 以获取用户看到的标签。

**2. 决定如何版本化**

如果您在 `plugin.json` 中设置 `version` 并稍后推送提交而不更改它，`claude plugin update` 会打印 `<name> is already at the latest version (1.0.0).`，用户保留旧副本。要么在每次发布时增加 `version`，要么在 git 托管的市场中省略它，以便 Claude Code 改用提交 SHA。请参阅[版本和更新](https://code.claude.com/docs/zh-CN/plugins/loading.md#versions-and-updates)。

**3. 验证**

在您的 shell 中，运行 `claude plugin validate --strict ./your-plugin`。干净的运行会打印 `✔ Validation passed`。

* **在 CI 中**：保持 `--strict`，它也会因为警告（例如未知的清单字段或缺少 `version`）而以退出代码 1 失败运行。如果您在上一步中选择省略 `version`，则删除 `--strict`。
* **路径**：验证报告不以 `./` 开头的组件路径。在 hook 命令和 MCP 服务器配置中，将文件引用为 `${CLAUDE_PLUGIN_ROOT}/...`。请参阅[路径规则](https://code.claude.com/docs/zh-CN/plugins/manifest-reference.md#path-rules)。

**4. 从本地市场安装它**

在您的 shell 中，使用 `claude plugin marketplace add ./path-to-marketplace` 添加列出插件的本地市场，从中安装插件，并启动会话以确认它加载。

* 对于最小的有效市场，请参阅[创建市场](https://code.claude.com/docs/zh-CN/plugins/create-marketplace.md)。
* 要了解安装是加载您的源目录还是缓存副本，请参阅[就地和复制的插件](https://code.claude.com/docs/zh-CN/plugins/loading.md#in-place-and-copied-plugins)。

**5. 填写用户看到的元数据**

在 `plugin.json` 中设置 `description`、`author`、`homepage` 和 `repository`，并在插件根目录添加 `README.md`。`homepage` 必须解析为 URL。[清单参考](https://code.claude.com/docs/zh-CN/plugins/manifest-reference.md#fields)列出了每个字段。

**6. 运行您的 eval 套件**

如果您有 eval 套件，在您的 shell 中运行 `claude plugin eval`。它运行插件的测试用例并对结果进行评分，这在您更改插件时捕获回归。请参阅[使用 evals 测试插件](https://code.claude.com/docs/zh-CN/plugin-evals.md)。

## 不使用市场共享插件

如果插件在 git 存储库中，人们可以克隆它并加载检出，或从他们的 shell 启动 Claude Code，使用 `--plugin-url` 指向您附加到发布的 `.zip`。要获取您的下一个版本，他们拉取或再次下载。如果它不在存储库中，请将目录或其 `.zip` 发送给他们。他们可以通过以下两种方式之一加载它：

* **对于一个会话**：他们从他们的 shell 启动 Claude Code，使用 `claude --plugin-dir ./deploy-helper`，其中路径是克隆、解压的文件夹或 `.zip` 本身。请参阅[为一个会话加载插件的标志](https://code.claude.com/docs/zh-CN/plugins/cli-reference.md#flags-that-load-a-plugin-for-one-session)。
* **对于每个会话**：他们将插件目录（带有其 `.claude-plugin/plugin.json`）移到 `~/.claude/skills/` 下。此后 Claude Code 会在启动时加载它，除非他们的管理员已[阻止 skills 目录插件](https://code.claude.com/docs/zh-CN/plugins/loading.md#skills-directory-plugins-blocked-by-managed-settings)。

将 `.claude-plugin/marketplace.json` 添加到同一存储库是让人们按名称安装和使用命令更新的方式；请参阅[通过您自己的市场发布](#publish-through-your-own-marketplace)。

### 使用您自己的工具发布插件

如果您维护 CLI 或 SDK，在市场中发布插件，并让您的安装程序或安装后消息运行或打印用户需要的两个命令：`claude plugin marketplace add <source>`，然后 `claude plugin install <name>@<marketplace>`。对于当某人使用您的工具时的会话内发现，请参阅[从您的 CLI 推荐您的插件](https://code.claude.com/docs/zh-CN/plugins/cli-hints.md)。

## 通过您自己的市场发布

您自己的市场是一个 `.claude-plugin/marketplace.json` 文件，列出您的插件，添加到 git 存储库。一旦文件在存储库中，插件就会发布，无需提交表单。您可以将文件保留在插件自己的存储库中或单独的存储库中。

### 将市场文件添加到您的存储库

要从插件自己的存储库发布，请在 `.claude-plugin/` 中的 `plugin.json` 旁边保存市场文件，其中一个条目的 `source` 是 `"./"` 即存储库根目录。给条目与 `plugin.json` 相同的 `name`，根据[保持条目名称和清单名称相同](https://code.claude.com/docs/zh-CN/plugins/create-marketplace.md#keep-the-entry-name-and-the-manifest-name-the-same)：

```json .claude-plugin/marketplace.json theme={null}
{
  "name": "your-marketplace",
  "owner": { "name": "Your Name" },
  "plugins": [
    { "name": "deploy-helper", "source": "./" }
  ]
}
```

将 `your-marketplace` 替换为其他作者不太可能选用的名称，因为用户[不能拥有两个同名的市场](https://code.claude.com/docs/zh-CN/plugins/marketplace-reference.md#marketplace-file)。在您的 shell 中，推送前在仓库中运行 `claude plugin validate .`。有关该运行检查的内容，请参阅[验证目录](https://code.claude.com/docs/zh-CN/plugins/cli-reference.md#validate-a-directory)。

[创建市场](https://code.claude.com/docs/zh-CN/plugins/create-marketplace.md)涵盖了一个存储库中有多个插件的布局。

### 控制谁可以安装

任何可以克隆存储库的人都可以从中安装，因此如果存储库是私有的，市场也是私有的。对于 git 存储库以外的主机，请参阅[托管市场](https://code.claude.com/docs/zh-CN/plugins/host-marketplace.md)。要到达整个公司的每个人，包括不使用 git 的人，请参阅[向整个公司推出](https://code.claude.com/docs/zh-CN/plugins/host-marketplace.md#roll-out-to-a-whole-company)。

### 告诉用户如何安装

告诉您的用户添加市场，然后从他们的 shell 安装插件，用您的替换源和名称：

* 添加市场一次：`claude plugin marketplace add your-org/your-marketplace`，其中参数是 GitHub `owner/repo` 简写、URL 或路径
* 安装插件：`claude plugin install deploy-helper@your-marketplace`
* 或从会话内同时执行两者：`/plugin install deploy-helper --marketplace your-org/your-marketplace`。需要 Claude Code v2.1.275 或更高版本。请参阅[在一个命令中添加市场和安装](https://code.claude.com/docs/zh-CN/plugins/install.md#add-a-marketplace-and-install-in-one-command)
* 或在 shell 中用一个命令同时执行两者：`claude plugin install deploy-helper --marketplace your-org/your-marketplace`。需要 Claude Code v2.1.292 或更高版本

### 向用户发布更新

用户在请求时或为您的市场启用自动更新时接收发布：

* **按请求**：用户的 shell 中的 `claude plugin update deploy-helper@your-marketplace` 刷新市场，当您的插件版本更改时安装新副本
* **自动更新**：默认为您的市场关闭。请参阅[启用自动更新](https://code.claude.com/docs/zh-CN/plugins/host-marketplace.md#turn-on-auto-update)。启用后，它在会话启动后的延迟后执行与 `claude plugin update` 相同的操作

[安装插件](https://code.claude.com/docs/zh-CN/plugins/install.md)涵盖用户端命令，[自动更新何时运行](https://code.claude.com/docs/zh-CN/plugins/loading.md#when-auto-update-runs)涵盖时间。

## 提交到 Anthropic 的目录

[Anthropic 的目录](https://claude.ai/directory)是人们在 claude.ai 和 Cowork 中浏览以添加插件和连接器的目录。在那里的一个列表可以覆盖 claude.ai、Cowork 和 Claude Code 上的用户。您可以从开发者门户 [claude.ai/directory/manage](https://claude.ai/directory/manage) 提交，claude.com 上的 [Submit a plugin](https://claude.com/docs/plugins/submit#submit-a-plugin) 逐步介绍了门户的使用方法。

Anthropic 的官方市场 `claude-plugins-official` 不通过目录门户接受提交。如果您与 Anthropic 合作伙伴联系合作，请询问他们关于官方市场列表的信息。

要提交插件：

**1. 确认您可以提交**

提交需要付费的 claude.ai 计划。在 Pro、Max 和 Ultra 上，您可以从自己的账户提交。在 Team 和 Enterprise 上，Owner 可以提交，在 Enterprise 上，Owner 还可以通过 **Organization settings > Roles** 下的自定义角色向其他成员授予 **Directory** 权限。请参阅 [Confirm you can submit to the directory](https://claude.com/docs/directory/publish#confirm-you-can-submit-to-the-directory)。

**2. 在本地验证插件**

在您的 shell 中运行 `claude plugin validate ./your-plugin --strict`。用您的插件目录的路径替换 `./your-plugin`。该命令在本地捕获清单错误；[plugin validate](https://code.claude.com/docs/zh-CN/plugins/cli-reference.md#plugin-validate) 列出了每次运行读取的文件。门户应用了 CLI 不检查的额外目录规则，因此本地运行清晰并不保证门户验证清晰。

claude.com 上的 [plugin pre-submission checklist](https://claude.com/docs/plugins/pre-submission-checklist#run-the-checks-before-you-submit) 列出了提交前要运行和修复的检查。

**3. 检查在哪里加载**

某些插件组件仅限 Claude Code，不在 claude.ai 或 Cowork 中加载。[component support table](https://claude.com/docs/plugins/platform-support#compare-component-support-by-app) 按应用列出了每个组件，因此您知道 Claude Code 之外的用户会获得什么。

**4. 在开发者门户中提交**

打开开发者门户 [claude.ai/directory/manage](https://claude.ai/directory/manage)，并按照 claude.com 上的 [Submit a plugin](https://claude.com/docs/plugins/submit#submit-a-plugin) 操作。

流程的其余部分记录在 claude.com 上：

* [Prepare for review](https://claude.com/docs/directory/publish#prepare-for-review)：每个版本在发布前会发生什么
* [Update a published plugin](https://claude.com/docs/plugins/submit#update-a-published-plugin)：新版本如何到达已安装您插件的用户
* [Submit your plugin, and your MCP server as a connector](https://claude.com/docs/directory/publish#submit-your-plugin-and-your-mcp-server-as-a-connector)：您可以提交什么
* [Move an earlier submission to the developer portal](https://claude.com/docs/directory/publish#move-an-earlier-submission-to-the-developer-portal)：如果您通过早期提交表单之一提交了插件（在门户存在之前），该怎么办

### 列出的插件如何到达 Claude Code 用户

在 claude.ai 上从目录安装您的插件的人在他们的账户上拥有它，Claude Code 将其加载为 `<name>@synced`。[Plugins synced from claude.ai](https://code.claude.com/docs/zh-CN/plugins/loading.md#synced-plugins) 涵盖了他们看到的内容以及他们如何关闭它。

## 发布更新、重命名和删除

### 发布新版本

如果您通过您自己的市场发布，并且您的 `plugin.json` 设置了 `version`，请增加它并推送。运行 `claude plugin update` 或启用自动更新的用户然后接收新版本，如[向用户发布更新](#ship-updates-to-users)下所述。对于目录列表，请参阅[更新已发布的插件](https://claude.com/docs/plugins/submit#update-a-published-plugin)。

### 标记发布

当其他插件在您的上声明版本范围时，在 git 中标记发布，因为这些范围针对标记进行解析。否则您不需要标记。

要标记，请从插件目录在您的 shell 中运行 `claude plugin tag`。它创建一个 `{name}--v{version}` 标记。添加 `--push` 以将标记发送到 `origin`。[`plugin tag` 参考](https://code.claude.com/docs/zh-CN/plugins/cli-reference#plugin-tag)列出了其标志。

### 重命名或删除插件

永远不要更改已发布插件的 `name`。重命名后，已安装它的用户会丢失插件，因为他们的安装记录在旧名称下。您的市场文件中的 `renames` 条目会改为迁移它们。当您想要不同的标签时，更改 `displayName`。

如果重命名是不可避免的，请使用市场文件的 `renames` 映射，以便现有安装迁移而不是因为 [`Plugin "<name>" not found in marketplace`](https://code.claude.com/docs/zh-CN/plugins/troubleshooting#plugin-not-found-in-marketplace) 而失败。要从市场中删除插件，或获取完整的 `renames` 详细信息，请参阅托管页面上的[重命名或删除插件](https://code.claude.com/docs/zh-CN/plugins/host-marketplace.md#rename-or-remove-a-plugin)。[市场参考](https://code.claude.com/docs/zh-CN/plugins/marketplace-reference.md#top-level-fields)有该字段。

## 声明依赖项

如果您的插件需要来自同一市场的另一个插件被启用，请在 `plugin.json` 的 `dependencies` 数组中列出它。每个条目是一个裸名称或具有 semver `version` 范围的对象。当用户安装您的插件时，Claude Code 也会安装并启用依赖项。

[插件依赖项](https://code.claude.com/docs/zh-CN/plugins/dependencies.md)涵盖范围语法、跨市场依赖项以及用户如何修剪他们不再需要的依赖项。

## 后续步骤

* [托管和维护市场](https://code.claude.com/docs/zh-CN/plugins/host-marketplace.md)：发布新版本并让用户保持最新
* [插件依赖项](https://code.claude.com/docs/zh-CN/plugins/dependencies.md)：声明和版本化您的插件所依赖的插件
* [从您的 CLI 推荐您的插件](https://code.claude.com/docs/zh-CN/plugins/cli-hints.md)：提示您的 CLI 的 Claude Code 用户安装插件
* [测量插件成本和使用情况](https://code.claude.com/docs/zh-CN/plugins/measure.md)：查看您的插件在上下文中的成本以及人们是否使用它
