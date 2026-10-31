> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# 插件依赖

> 声明你的插件所依赖的其他插件，使用版本范围如 ^1.2，并了解 Claude Code 如何安装、解析和修剪它们。

插件依赖是你的插件所依赖的另一个插件，例如你调用其 MCP 服务器或技能的插件。每个依赖都会跟踪其市场提供的最新版本，除非你声明版本约束，即你已测试过的语义版本范围，如 `^2.0` 或 `~2.1.0`。

本页面适用于在 `plugin.json` 中声明依赖的插件作者和标记发布的市场维护者。

> [!NOTE]
>
> 以下情况在其他页面中介绍：
>
> * **安装具有依赖的插件**：请参阅 [管理已安装的插件](https://code.claude.com/docs/zh-CN/plugins/install.md#manage-installed-plugins)
> * **阅读依赖错误**：请参阅 [依赖错误](https://code.claude.com/docs/zh-CN/plugins/troubleshooting.md#dependency-errors)
> * **声明你的插件自身代码所需的 npm 和 Bun 包**：请参阅 [Node.js 包依赖](https://code.claude.com/docs/zh-CN/plugins/loading.md#node-js-package-dependencies)

要添加约束，请从 [使用版本约束声明依赖](#declare-a-dependency-with-a-version-constraint) 开始。如果你维护其他人依赖的插件，请 [标记你的发布](#tag-plugin-releases-for-version-resolution) 以便他们的约束可以解析。

## 声明依赖

如果没有版本约束，依赖会在用户下次更新时移动到其市场发布的每个新版本。如果该版本重命名了你的 plugin 调用的 MCP 工具，你的 plugin 会对所有更新的用户中断。

使用约束（如来自 git 支持源的依赖上的 `~2.1.0`），安装了你的 plugin 的用户会继续接收依赖的 `2.1.x` 补丁，永远不会移动到 `2.2`。要按自己的计划升级，请针对较新的版本进行测试，然后发布你的 plugin 的新版本，使用更宽松的约束。

### 使用版本约束声明依赖

在你的 plugin 的 `.claude-plugin/plugin.json` 的 `dependencies` 数组中列出依赖。以下清单声明了一个无版本依赖和一个受约束的依赖：

```json .claude-plugin/plugin.json theme={null}
{
  "name": "deploy-kit",
  "version": "3.1.0",
  "dependencies": [
    "audit-logger",
    { "name": "secrets-vault", "version": "~2.1.0" }
  ]
}
```

一个条目可以是一个字符串：仅 plugin 名称，如此清单中的 `"audit-logger"`，或 `"name@marketplace"` 以在另一个市场中解析它。使用裸字符串，你的 plugin 依赖于该 plugin 市场提供的任何版本。

要设置版本约束，请使用具有这些字段的对象，每个字段都是字符串：

| 字段 | 描述 |
| :- | :- |
| `name` | 依赖的 plugin 名称，如其市场条目中所示。Claude Code 在与声明 plugin 相同的市场中查找它，除非你设置 `marketplace`。必需。 |
| `version` | 一个 [语义版本范围](https://github.com/npm/node-semver#ranges)，如 `~2.1.0`、`^2.0`、`>=1.4` 或 `=2.1.0`。依赖安装在满足此范围的最高 git 标签处，因此依赖的维护者必须 [标记发布版本](#tag-plugin-releases-for-version-resolution)。 |
| `marketplace` | 用于解析 `name` 的不同市场。允许列表控制跨市场依赖，详见 [依赖来自另一个市场的 plugin](#depend-on-a-plugin-from-another-marketplace)。 |

范围不匹配预发布版本，如 `2.0.0-beta.1`，除非你选择使用预发布后缀，如 `^2.0.0-0`。

### 为团队捆绑 plugin

要让工程师用一个命令安装精选的 plugin 集合，请发布一个清单包含 `name` 和 `dependencies` 数组的 plugin。Plugin 清单只需要 `name`，所以这是一个有效的 plugin，安装它会安装每个依赖。

例如，平台团队可以在内部市场中发布特定角色的捆绑包，以便工程师运行一个 `claude plugin install` 而不是分别安装每个 plugin：

```json .claude-plugin/plugin.json theme={null}
{
  "name": "backend-standard",
  "version": "1.0.0",
  "description": "Standard plugin set for backend engineers",
  "dependencies": [
    "secrets-vault",
    "deploy-kit",
    { "name": "db-migrate", "version": "^3.0" },
    "oncall-runbook"
  ]
}
```

要稍后向标准集添加 plugin，请发布新的 `backend-standard` 版本，包含额外的依赖。当市场不 [默认自动更新](https://code.claude.com/docs/zh-CN/plugins/loading.md#which-marketplaces-and-plugins-auto-update) 时，工程师要么为市场打开自动更新，要么手动更新：

* **为市场打开自动更新**：下一次自动更新会将捆绑包移动到新版本并安装它添加的任何依赖。
* **手动更新**：在 shell 中运行 `claude plugin update backend-standard`，然后在打开的会话中运行 `/reload-plugins` 以安装新添加的依赖。

有关工程师端的步骤，请参阅 [保持 plugin 更新](https://code.claude.com/docs/zh-CN/plugins/install.md#keep-plugins-updated)。

要将捆绑包部署给组织中的每个人，管理员将其添加到托管设置中的 `enabledPlugins`。请参阅 [预安装和要求 plugin](https://code.claude.com/docs/zh-CN/plugins/org.md#pre-install-and-require-plugins)。

### 依赖来自另一个市场的 plugin

默认情况下，Claude Code 不会从与声明 plugin 自身不同的市场安装依赖，除非用户已经在同一范围内安装并启用了该依赖。此默认值防止一个市场从用户未审查的源中静默安装 plugin。

要允许安装，请将目标市场的名称添加到根市场的 `marketplace.json` 中的 `allowCrossMarketplaceDependenciesOn`。根市场是托管用户正在安装的 plugin 的市场。仅根市场的允许列表适用。

以下 `marketplace.json` 允许 `deploy-kit` 依赖来自 `your-shared-marketplace` 的 plugin：

```json .claude-plugin/marketplace.json theme={null}
{
  "name": "your-marketplace",
  "owner": { "name": "Your Org" },
  "allowCrossMarketplaceDependenciesOn": ["your-shared-marketplace"],
  "plugins": [
    {
      "name": "deploy-kit",
      "source": "./deploy-kit",
      "dependencies": [
        { "name": "audit-logger", "marketplace": "your-shared-marketplace" }
      ]
    }
  ]
}
```

如果 `allowCrossMarketplaceDependenciesOn` 缺失或不包含目标市场，Claude Code 不会安装依赖。当依赖在市场条目中声明时，安装本身会被拒绝，消息以 `Dependency "audit-logger@your-shared-marketplace" (required by deploy-kit@your-marketplace) is in marketplace "your-shared-marketplace", which is not in the allowlist` 开头，并命名要设置的字段。当它在 `plugin.json` 中声明时，安装完成但没有依赖，你的 plugin 随后无法加载。

允许列表检查不适用于已启用的依赖。如果用户首先从 `your-shared-marketplace` 自己安装 `audit-logger`，在同一范围内，`deploy-kit` 随后安装时无需对允许列表进行任何更改。

### 在本地测试 plugin 及其依赖

如果你同时开发一个 plugin 和它所依赖的 plugin，请从你的 shell 启动 Claude Code 并使用 [`--plugin-dir`](https://code.claude.com/docs/zh-CN/plugins/cli-reference#flags-that-load-a-plugin-for-one-session) 加载两者：

```bash theme={null}
claude --plugin-dir ./my-dependency --plugin-dir ./my-plugin
```

依赖的本地副本满足你的 plugin 的依赖条目，所以你不需要从其市场安装依赖。

* **不需要 `version`**：依赖的本地 `plugin.json` 也不需要 `version`，因为 [版本约束](#declare-a-dependency-with-a-version-constraint) 不会针对本地副本进行检查。
* **命名市场的条目**：命名市场的条目在 Claude Code v2.1.242 或更高版本上也匹配本地副本。

在你从其市场安装依赖之前，每当本地副本被禁用或不存在时，你的 plugin 都会停止加载：

* **你禁用了本地副本**：你的 plugin 在下一次 plugin 加载时被禁用，错误以 `is disabled — enable it or remove the dependency` 结尾。当错误将依赖命名为 `<name>@inline` 时，该标识符指的是 `--plugin-dir` 副本。
* **你启动了一个没有依赖的 `--plugin-dir` 标志的会话**：错误报告依赖未安装。再次传递标志，或从其市场安装依赖。

当两个 plugin 都在一个父文件夹中时，你可以将该文件夹传递给 `--plugin-dir` 一次。如果该文件夹本身不是 plugin，Claude Code 会加载每个具有 `.claude-plugin/plugin.json` 的子文件夹。需要 Claude Code v2.1.265 或更高版本。

## 发布其他人依赖的 plugin

如果你维护其他 plugin 使用版本约束依赖的 plugin，请标记其发布版本，以便这些约束可以解析。约束针对托管 plugin 的存储库上的 git 标签进行解析。标记 plugin 的 [plugin 源](https://code.claude.com/docs/zh-CN/plugins/marketplace-reference.md#plugin-sources) 在 `marketplace.json` 中指向的存储库：

* **`github`、`url` 或 `git-subdir` 源**：plugin 自身的存储库，所以 plugin 的作者创建标签
* **相对路径，如 `./plugins/secrets-vault`**：市场存储库，所以市场维护者创建标签

### 创建发布标签

将每个发布标记为 `<plugin-name>--v<version>`，其中 `<version>` 与该提交的 `plugin.json` 中的 `version` 字段匹配。plugin-name 前缀让一个市场存储库可以托管多个具有独立版本历史的 plugin。

从 plugin 目录创建标签，配置 `origin` 远程以接收推送的标签，使用 [`claude plugin tag`](https://code.claude.com/docs/zh-CN/plugins/cli-reference#plugin-tag)：

```bash theme={null}
claude plugin tag --push
```

该命令从 plugin 的清单构建标签名称。在创建标签之前，它运行这些检查：

* 验证 plugin
* 检查 `plugin.json` 和市场条目在版本上是否一致，当 plugin 目录在市场检出内时
* 要求 plugin 目录下的工作树干净
* 如果标签已存在则拒绝

成功运行会打印 `Created tag secrets-vault--v2.1.0`。使用 `--push`，它还会打印 `Pushed to origin`。不使用 `--push`，它会打印你自己运行的 `git push` 命令。

传递 `--dry-run` 以查看计划而不创建任何内容。

[`claude plugin tag` 参考](https://code.claude.com/docs/zh-CN/plugins/cli-reference#plugin-tag) 列出了其余标志。

你也可以直接运行 `git tag secrets-vault--v2.1.0`，只要你自己保持 `plugin.json` 中的 `version` 和市场条目中的版本同步。

### 约束具有非 git 源的依赖

基于标签的解析仅适用于 git 支持的源。对于具有 `npm`、`archive` 或 `command` [plugin 源](https://code.claude.com/docs/zh-CN/plugins/marketplace-reference.md#plugin-sources) 的依赖，约束不控制获取哪个版本。它在 plugin 加载时仍会被检查，如果安装的版本不满足它，依赖的 plugin 会被禁用。

对于 `npm`、`archive` 和 `command` 源，检查的版本是依赖的 `plugin.json` 中的 `version`。在约束该依赖之前在那里设置一个，因为不设置版本的 `plugin.json` 不满足任何约束。

Claude Code 永远不会自己安装具有 `command` 源的依赖，所以用户 [首先安装它](https://code.claude.com/docs/zh-CN/plugins/marketplace-reference.md#command-plugin-source)。它也永远不会运行依赖的 [`headersHelper`](https://code.claude.com/docs/zh-CN/plugins/host-marketplace#authenticate-archive-downloads)，所以用户也在安装你的 plugin 之前安装其市场条目设置的依赖。

除了 `claude plugin install`，这些操作也会安装任何缺失的声明依赖，`command` 和 `headersHelper` 限制也适用于它们：

* `/reload-plugins`
* 依赖 plugin 市场的自动更新
* 在依赖 plugin 上重新运行 `claude plugin install`
* `claude plugin marketplace add`

## 依赖如何为你的用户表现

这些部分描述了一旦你的 plugin 与其他 plugin 一起安装，Claude Code 如何解析、检查和组合你声明的约束。

### 约束如何针对标签进行解析

当用户安装声明 `{ "name": "secrets-vault", "version": "~2.1.0" }` 的 plugin 时，依赖从满足 `~2.1.0` 的最高 `secrets-vault--v` 标签安装在托管 `secrets-vault` 的存储库上。当没有标签满足范围时，安装要么失败，要么使用市场的当前副本：

* **具有自身存储库的 plugin**：安装失败，消息包含 `Dependency "secrets-vault@your-marketplace" has no git tag satisfying`。
* **由相对路径引用的 plugin**：安装改为使用市场的当前副本，约束在 plugin 加载时被检查。如果该副本在范围之外，依赖的 plugin 保持禁用，`claude plugin list` 显示 `Requires "secrets-vault@your-marketplace" ~2.1.0, installed 3.0.0`。

对于市场通过相对路径引用的插件，当您添加为本地文件夹路径的市场所在文件夹是 git 仓库时，该市场也会针对该文件夹的 git 标签解析约束。不是 git 仓库的本地文件夹没有标签，所以 Claude Code 改为从文件夹的当前内容安装依赖。

### 确认解析的版本

要确认约束解析到哪个版本，请在你的 shell 中运行 `claude plugin list`。标签解析的依赖显示其版本，带有 12 字符的提交后缀，如 `2.1.0-8713c5b11005`。

约束检查使用标签的版本而不是 `plugin.json` 中的 `version`，即使该提交处的 `plugin.json` 滞后。

如果你强制移动标签到不同的提交，下一次安装会获取该提交的内容而不是重用陈旧的缓存副本。请参阅 [版本和更新](https://code.claude.com/docs/zh-CN/plugins/loading.md#versions-and-updates) 了解 plugin 的版本如何成为其缓存键。

### 组合来自多个 plugin 的约束

当多个已安装的 plugin 约束同一依赖时，依赖解析到满足所有范围的最高版本。常见组合解析如下：

| Plugin A 要求 | Plugin B 要求 | 结果 |
| :- | :- | :- |
| `^2.0` | `>=2.1` | 一次安装在最高 `2.x` 标签处，位于或高于 `2.1.0`。两个 plugin 都加载。 |
| `~2.1` | `~3.0` | 安装 plugin B 失败，消息为 `has conflicting version requirements`。Plugin A 和依赖保持原样。 |
| `=2.1.0` | 无 | 依赖保持在 `2.1.0`。自动更新在 plugin A 安装时跳过较新版本。 |

自动更新在满足每个已安装 plugin 范围的最高 git 标签处获取受约束的依赖，而不是在市场的最新版本处。如果已安装 plugin 的范围不重叠，自动更新将该依赖保持在其当前版本，`/plugin` **Errors** 标签页显示命名约束 plugin 的条目。如果它们重叠但没有标签落在范围内，自动更新获取市场的当前副本，当该副本的 `version` 落在任何已安装 plugin 范围之外时跳过更新。

当用户卸载最后一个约束依赖的 plugin 时，依赖不再被约束到版本范围，并在下一次更新时恢复跟踪其市场条目。

### 当您的插件从 claude.ai 加载时的依赖

当用户为其 claude.ai 账户启用您的插件时，Claude Code 将其加载为 `<name>@synced`，[不关联任何市场](https://code.claude.com/docs/zh-CN/plugins/loading.md#synced-plugins)。对于您插件的这一副本，当用户具备以下任一条件时，Claude Code 会将指定了市场的依赖条目视为已满足：

* **来自其市场的依赖**：从您的条目所指定的市场安装并已启用。Claude Code 会根据已安装副本的版本检查您条目中的 `version` 范围。
* **同名的 claude.ai 插件**：已从其 claude.ai 账户启用。Claude Code 会根据该插件的 `version` 检查您条目中的 `version` 范围，未声明 `version` 的插件会通过检查。

如果您告知用户从 claude.ai 启用您的插件，请同时告知他们也在那里启用该依赖，或从其市场安装该依赖。

只有当您的插件也从 claude.ai 加载时，claude.ai 插件才能满足该条目。

## 另请参阅

* [`claude plugin prune`](https://code.claude.com/docs/zh-CN/plugins/cli-reference#plugin-prune)：删除任何 plugin 不再需要的自动安装依赖
* [托管市场](https://code.claude.com/docs/zh-CN/plugins/host-marketplace.md)：发布渠道和推荐其他 plugin
