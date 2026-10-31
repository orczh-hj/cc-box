> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Marketplace 参考

> marketplace.json 字段、插件条目和插件及 marketplace 源对象的完整参考，包括每个字段的有效位置。

`marketplace.json` 是定义插件 marketplace 的文件。它包含 marketplace 的名称、所有者和每个插件的一个条目。每个条目的插件源说明 Claude Code 从哪里获取该插件。

marketplace 源是一个单独的对象，说明 Claude Code 从哪里获取 marketplace 文件本身。你在设置中编写一个，或者当你运行 `claude plugin marketplace add` 时 Claude Code 会构建一个。

本参考适用于需要确切字段名称或值的 marketplace 维护者，以及需要了解哪些 `source` 值在 [`extraKnownMarketplaces`](https://code.claude.com/docs/zh-CN/settings-reference#extraknownmarketplaces)、[`strictKnownMarketplaces`](https://code.claude.com/docs/zh-CN/settings-reference#strictknownmarketplaces) 和 [`blockedMarketplaces`](https://code.claude.com/docs/zh-CN/plugins/org#restrict-what-users-can-install) 中有效的管理员。

> [!NOTE]
>
> 这些情况在其他页面上有介绍：
>
> * **构建或托管 marketplace**：请参阅 [创建 marketplace](https://code.claude.com/docs/zh-CN/plugins/create-marketplace.md) 和 [托管和维护 marketplace](https://code.claude.com/docs/zh-CN/plugins/host-marketplace.md)
> * **允许列表和阻止列表配方**：请参阅 [为你的组织管理插件](https://code.claude.com/docs/zh-CN/plugins/org.md)

查找你正在编写或读取的内容的部分：

* **marketplace 文件**：[顶级字段](#top-level-fields) 和 [插件条目](#plugin-entries)
* **条目的 `source`**：[插件源](#plugin-sources)
* **设置中的 `source` 对象**：[Marketplace 源](#marketplace-sources)
* **来自 [`claude plugin validate <path>`](https://code.claude.com/docs/zh-CN/plugins/cli-reference) 的输出**：[验证消息](#validation-messages)，它将每条消息映射到它命名的字段

## Marketplace 文件

将 marketplace 文件保存在 marketplace 目录中的 `.claude-plugin/marketplace.json`。如果你将文件保存在存储库中的其他位置，用户必须在 [`extraKnownMarketplaces`](https://code.claude.com/docs/zh-CN/settings-reference#extraknownmarketplaces) 中声明 marketplace，并在其源上设置 `path`，因为 `claude plugin marketplace add` 没有该选项。

包含 `.claude-plugin/` 的目录称为 marketplace 根目录，每个相对插件源都从它解析，而不是从 `.claude-plugin/`。

每个用户为每个 `name` 注册一个 marketplace，因此用户不能同时注册两个同名的 marketplace。

Claude Code 忽略未知的顶级键或插件条目键，而不是拒绝它，因此拼写错误会静默加载。`claude plugin validate` 将每个未知键报告为警告。

### 保留名称

你不能给你的 marketplace 以下任何名称：

* **官方 marketplace 名称**：`claude-code-marketplace`、`claude-code-plugins`、`claude-plugins-official`、`anthropic-marketplace`、`anthropic-plugins`、`agent-skills`、`anthropic-agent-skills`、`life-sciences`、`knowledge-work-plugins`、`claude-for-legal`、`claude-for-financial-services`、`financial-services-plugins`、`first-party-plugins` 和 `claude-tag-plugins`。除非 marketplace 来自 `github.com/anthropics/` 下的 `github` 或 `git` [marketplace 源](#marketplace-sources)，否则保留。
* **社区 marketplace 名称**：`claude-community`、`claude-plugins-community` 和 `healthcare`。保留规则与官方名称相同。
* **插件目录名称**：`anthropic-plugin-directory` 和 `claude-plugin-directory`。保留规则与官方名称相同。
* **冒充官方 marketplace 的名称**：名称如 `official-claude-plugins` 或 `claude-plugins-v2`，以及任何包含非 ASCII 字符的名称。错误是 `Marketplace name impersonates an official Anthropic/Claude marketplace`。名称中的控制或双向格式化字符也会报告 `Marketplace name cannot contain control or bidirectional-formatting characters`。已在这样的名称下注册的 marketplace 停止加载，连同其插件。
* **保留名称的另一种拼写**：仅因尾部的点，或因用除下划线以外的符号代替连字符，而与保留名称不同的名称，因此 `claude.code.plugins` 计为 `claude-code-plugins`。添加 marketplace 失败，错误为 [`is another spelling of "<reserved>", a reserved marketplace name`](https://code.claude.com/docs/zh-CN/errors#marketplace-name-is-another-spelling-of-a-reserved-name)，已使用此类名称注册的 marketplace 将停止加载。此检查需要 Claude Code v2.1.280 或更高版本。
* **Claude Code 用于不来自 marketplace 的插件的名称**：`inline` 用于使用 [`--plugin-dir`](https://code.claude.com/docs/zh-CN/cli-reference) 加载的插件，`builtin` 用于内置插件，`skills-dir` 用于从 [`.claude/skills/`](https://code.claude.com/docs/zh-CN/skills) 自动加载的插件，`synced` 用于从你的 claude.ai 账户同步的插件。`claude-plugin-test` 也被保留。`skills-dir` 也显示为 `{"source": "skills-dir"}`，在 `strictKnownMarketplaces` 和 `blockedMarketplaces` 中，如 [仅在策略列表中有效的源值](#source-values-valid-only-in-policy-lists) 下所述。
* **`npm`、`pip`、`uv`、`cargo`、`github` 和 `gh`**：以任何大小写保留。此检查需要 Claude Code v2.1.275 或更高版本。
* **每个 JavaScript 对象都具有的成员名称**：`constructor`、`hasOwnProperty`、`isPrototypeOf`、`propertyIsEnumerable`、`toLocaleString`、`toString` 和 `valueOf`。`claude plugin marketplace add` 会拒绝使用其中之一的市场，错误为 [`Claude Code reserves this name and cannot register a marketplace under it`](https://code.claude.com/docs/zh-CN/plugins/troubleshooting#claude-code-reserves-this-name)。此检查需要 Claude Code v2.1.296 或更高版本。
* **以 `claudeai-` 开头的名称**：为托管在 claude.ai 上的 marketplace 保留。`claude plugin marketplace add` 拒绝任何其他使用一个的 marketplace，错误为 `Cannot add marketplace "<name>": names starting with "claudeai-" are reserved for marketplaces hosted on claude.ai`。
* **已注册 GitHub 市场的下载文件夹名称 `<owner>-<repo>`**：对于从 `github` 源（例如 `acme/x-tools`）添加的市场，无论该市场自身的 `name` 是什么，Claude Code 都会通过名为 `acme-x-tools` 的文件夹下载它。当该市场以 `acme-x-tools` 以外的名称注册时，`claude plugin marketplace add` 会在下载另一个名为 `acme-x-tools` 的市场后拒绝它，并报告 `Can't use the marketplace name "acme-x-tools"`。此检查需要 Claude Code v2.1.290 或更高版本。

当已注册的市场因其名称模仿官方市场而停止加载时，`claude plugin list` 和 `/plugin` 会报告 `Claude Code refuses the marketplace name "<name>"`。该消息会提示您删除该市场。删除它也会卸载其插件，并删除这些插件的数据、选项和密钥。此命名拒绝消息需要 Claude Code v2.1.282 或更高版本。

如果某个设置文件以内联方式声明了该市场，该消息会提示您在删除旧名称之前先重命名其条目。有关步骤，请参阅 [Claude Code refuses the marketplace name](https://code.claude.com/docs/zh-CN/errors.md#claude-code-refuses-the-marketplace-name)。该消息在 Claude Code v2.1.292 或更高版本中会列出这些步骤。

## 顶级字段

该表列出 Claude Code 从 `marketplace.json` 读取的每个键。`name`、`owner` 和 `plugins` 是必需的。

| 字段 | 类型 | 描述 |
| :- | :- | :- |
| `name` | string | 市场标识符：字母、数字、`.`、`_` 和 `-`，以字母或数字开头，且不含 `..`。`claude plugin validate` 会使任何其他名称验证失败，因为 Claude Code [无法从使用此类名称的市场安装插件](https://code.claude.com/docs/zh-CN/plugins/troubleshooting.md#cannot-install-plugins-from-a-marketplace-with-this-name)。用户安装插件时，会在 [插件 ID](https://code.claude.com/docs/zh-CN/plugins/loading.md#find-where-a-plugin-came-from)（例如 `my-plugin@my-marketplace`）中的 `@` 之后输入该名称。请参阅 [保留名称](#reserved-names) |
| `owner` | object | 维护者信息。`name` 是必需的；`email` 和 `url` 是可选的 |
| `plugins` | array | [插件条目](#plugin-entries)。每个条目单独验证，因此一个无效条目不会导致 marketplace 失败 |
| `$schema` | string | JSON Schema URL 用于编辑器自动完成。在加载时忽略 |
| `description` | string | 向用户显示的 marketplace 描述。`claude plugin validate` 在缺少时警告 |
| `version` | string | Marketplace 清单版本 |
| `metadata.description`、`metadata.version` | string | `description` 和 `version` 的备用位置 |
| `metadata.pluginRoot` | string | 裸插件源名称解析的目录。请参阅 [相对路径插件源](#relative-path-plugin-source)。需要 Claude Code v2.1.239 或更高版本 |
| `forceRemoveDeletedPlugins` | boolean | 当为 `true` 时，从 `plugins` 中删除的插件会在用户的机器上卸载。请参阅 [托管和维护 marketplace](https://code.claude.com/docs/zh-CN/plugins/host-marketplace.md) |
| `allowCrossMarketplaceDependenciesOn` | array of strings | 其插件可作为此 marketplace 插件的依赖项安装的 marketplace 名称。安装插件时，仅适用该插件自己的 marketplace 中的列表，用于其整个依赖链。请参阅 [插件依赖项](https://code.claude.com/docs/zh-CN/plugins/dependencies.md) |
| `renames` | object | 从插件以前的 `name` 映射到其当前名称；对于已删除的插件，则映射到 `null`。请参阅 [托管和维护市场](https://code.claude.com/docs/zh-CN/plugins/host-marketplace.md) |

## 插件条目

`marketplace.json` 的顶级 `plugins` 数组中的每个对象命名一个插件并说明从哪里获取它。`name` 和 `source` 是必需的。

除 [目录列表字段](https://code.claude.com/docs/zh-CN/plugins/manifest-reference.md#directory-listing-fields) 外，条目也接受每个 [`plugin.json` 字段](https://code.claude.com/docs/zh-CN/plugins/manifest-reference)，如 `description`、`version`、`author`、`commands` 和 `hooks`。有关这些字段何时适用，请参阅 [条目如何与 plugin.json 结合](#entry-and-plugin-json)。

该表列出条目自己的字段和清单字段，其含义在条目中改变。

| 字段 | 类型 | 描述 |
| :- | :- | :- |
| `name` | string | 插件标识符：字母、数字、`.`、`_` 和 `-`，以字母或数字开头。`claude plugin validate` 会拒绝其他名称，Claude Code 无法安装。用户在安装时在 `@` 前键入它，即使插件自己的 `plugin.json` 设置了不同的 `name` |
| `source` | string or object | 从哪里获取插件。请参阅 [插件源](#plugin-sources) |
| `description` | string | 在 [`/plugin`](https://code.claude.com/docs/zh-CN/plugins/install) 列表和详情中显示 |
| `version` | string | 插件的版本字符串。当 `plugin.json` 也设置 `version` 时，`plugin.json` 优先，`claude plugin validate` 警告。请参阅 [插件加载参考](https://code.claude.com/docs/zh-CN/plugins/loading.md) |
| `category` | string | 用于组织目录的自由格式类别 |
| `tags` | array of strings | 用于搜索的自由格式标签 |
| `strict` | boolean | 默认 `true`。`plugin.json` 是否是插件组件的权威来源。请参阅 [严格模式](#strict-mode) |
| `relevance` | object | 告诉 Claude Code 何时建议插件的信号。请参阅 [为您的组织推荐插件](https://code.claude.com/docs/zh-CN/plugins/relevance.md) |
| `dependencies` | array | 必须为此插件启用的插件。每个项是 `"name"`、`"name@marketplace"` 或对象。请参阅 [插件依赖项](https://code.claude.com/docs/zh-CN/plugins/dependencies.md) |
| `defaultEnabled` | boolean | 默认 `true`。当用户未在 [`enabledPlugins`](https://code.claude.com/docs/zh-CN/settings-reference#enabledplugins) 中设置时，插件是否启动时启用。条目值优先于 `plugin.json` |
| `displayName` | string | 在 UI 中显示的人类可读名称。当条目和插件的 `plugin.json` 都未设置时，用户看到插件的 `name` |
| `metadata` | object | 用于您自己字段的自由格式对象。Claude Code 不读取它。需要 Claude Code v2.1.222 或更高版本 |
| `headers` | object | Claude Code 在下载此条目的 [archive](#archive-plugin-source) 时发送的 HTTP 标头。此处设置的标头替换市场源的 [`headers`](#fields-by-type) 中同名的标头。需要 Claude Code v2.1.238 或更高版本 |
| `headersHelper` | string | 打印此条目的 archive 下载标头的命令，作为一个 JSON 对象，用于会过期的凭据。条目还必须设置 [`"strict": false`](#strict-mode)。需要 Claude Code v2.1.238 或更高版本。请参阅 [验证 archive 下载](https://code.claude.com/docs/zh-CN/plugins/host-marketplace.md#authenticate-archive-downloads) |

### 条目如何与 plugin.json 结合

条目的字段对获取的具有自己的 `.claude-plugin/plugin.json` 的插件和没有的插件的应用方式不同：

* **没有 `plugin.json`**：条目是清单，无论 `strict` 如何。条目中的每个清单字段都适用，包括 [`mcpServers`、`lspServers`、`userConfig` 和 `channels`](https://code.claude.com/docs/zh-CN/plugins/manifest-reference)。
* **`plugin.json` 存在**：`plugin.json` 是清单。[严格模式](#strict-mode) 决定条目的六个组件字段 `commands`、`agents`、`skills`、`hooks`、`outputStyles` 和 `themes` 是与其结合还是作为冲突被拒绝。条目 `mcpServers`、`lspServers`、`userConfig` 和 `channels` 不适用。在 `plugin.json` 中声明它们。

#### 条目中的 Hooks

将条目 `hooks` 写成内联对象，将 hook 事件名称映射到匹配器数组。如果您写文件路径或数组，`claude plugin validate` 会通过。这些 hook 永远不会运行，Claude Code 为插件报告 `not yet supported in a marketplace entry` 错误。将基于文件的 hook 放在插件自己的 [`hooks/hooks.json`](https://code.claude.com/docs/zh-CN/plugins/components) 或 `plugin.json` 中。

#### 显示字段

条目和插件自己的 `plugin.json` 都可以设置显示字段 `displayName`、`description`、`author`、`homepage`、`repository`、`license` 和 `keywords`。用户在插件列表和详情中看到这些值，在安装前后：

* 对于您在条目上设置的字段，用户看到条目的值，即使 `plugin.json` 设置了不同的值。
* 对于条目未设置的字段，用户看到 `plugin.json` 值。

在安装前，Claude Code 只能为具有 [相对路径源](#relative-path-plugin-source) 的条目读取 `plugin.json`，其插件文件在市场内。对于具有任何其他源类型的条目，用户在安装插件之前只看到条目自己的字段。

### 严格模式

`strict` 决定当获取的插件具有自己的 `plugin.json` 且条目也声明任何 [组件字段](#entry-and-plugin-json) 时会发生什么：`commands`、`agents`、`skills`、`hooks`、`outputStyles` 或 `themes`。使用 `strict: true`（默认值），Claude Code 将条目的组件字段附加到 `plugin.json`，除了 `hooks`，其匹配器替换清单的每个事件。使用 `strict: false`，声明任何组件字段的条目是冲突，插件加载失败。该表显示 `strict`、`plugin.json` 和条目的组件字段的每个组合。

| `strict` | `plugin.json` | 条目组件字段 | 结果 |
| :- | :- | :- | :- |
| any | absent | any | 条目是清单 |
| `true`，默认值 | present | any | `plugin.json` 是权威。Claude Code 将条目的组件字段附加到它，除了 `hooks`，其匹配器 [替换清单的每个事件](https://code.claude.com/docs/zh-CN/plugins/manifest-reference.md#how-entry-fields-combine-with-plugin-json) |
| `false` | present | none | `plugin.json` 是清单，与 `true` 相同 |
| `false` | present | one or more | 冲突。插件加载失败，错误为 `Plugin <name> has conflicting manifests: both plugin.json and marketplace entry specify components` |

## Plugin sources

一个插件条目的 `source` 说明 Claude Code 从哪里获取该插件。它要么是一个相对路径字符串，要么是一个对象，其自身的 `source` 键命名类型，所以一个条目看起来像 `"source": { "source": "github", "repo": "your-org/formatter" }`。

该表列出了每种插件源类型及其字段。

| 类型 | 字段 | 说明 |
| :- | :- | :- |
| 相对路径 | 字符串本身 | marketplace 内的一个目录，从 marketplace 根目录解析。必须以 `./` 开头，除非你在 [`metadata.pluginRoot` 下写一个裸名](#relative-path-plugin-source)。`"."` 本身表示根目录 |
| `github` | `repo`, `ref`, `sha` | GitHub 仓库，格式为 `owner/repo` |
| `url` | `url`, `ref`, `sha` | 任何 git 仓库的 URL |
| `git-subdir` | `url`, `path`, `ref`, `sha` | git 仓库的一个子目录，使用稀疏检出获取 |
| `npm` | `package`, `version`, `registry` | npm registry 包或 tarball 链接，使用您的 npm 客户端获取并解包，不运行安装脚本 |
| `archive` | `url`, `sha256` | HTTPS 上的 Zip 存档。需要 Claude Code v2.1.224 或更高版本 |
| `command` | `command`, `timeout`, `mode` | 由 Claude Code 在用户机器上运行的命令打印的目录。需要 Claude Code v2.1.229 或更高版本 |

名称 `url` 和 `github` 也是[marketplace 源](#marketplace-sources)类型，其中 `url` 表示直接链接到 `marketplace.json` 文件而不是 git 仓库。`git` 仅作为 marketplace 源存在，`npm` 既作为 marketplace 源也作为插件源存在。`git-subdir`、`archive` 和 `command` 仅作为插件源存在。

对于 marketplace 仓库本身的子目录中的插件，使用相对路径。对于其他仓库的子目录，使用 `git-subdir`。

`github`、`url` 和 `git-subdir` 源共享 `ref` 和 `sha` 字段：

* **`ref`**：一个分支或标签。默认为仓库的默认分支。
* **`sha`**：一个完整的 40 字符小写提交 SHA。当你同时设置 `ref` 和 `sha` 时，Claude Code 检出 `sha`。在大多数 git 主机上，包括 GitHub、GitLab 和 Bitbucket，这意味着即使上游的分支或标签已被删除，只要提交仍然可从仓库到达，安装就会成功。某些服务器（如 AWS CodeCommit）不支持按 SHA 获取提交。在这些服务器上，`ref` 必须仍然存在，固定的提交必须可从它到达。

有关每种类型如何获取、缓存和版本化的信息，请参阅 [Plugin loading reference](https://code.claude.com/docs/zh-CN/plugins/loading.md)。

### Relative path plugin source

路径从 marketplace 根目录解析。`./plugins/formatter` 是 `<root>/plugins/formatter`，即使 marketplace 文件在 `<root>/.claude-plugin/` 中。

包含 `..` 的路径会验证失败。在 macOS 和 Linux 上，Claude Code 拒绝条目路径在前导 `./` 之后的任何地方包含反斜杠，所以用正斜杠写路径。

```json theme={null}
{ "name": "formatter", "source": "./plugins/formatter" }
```

相对路径仅在 Claude Code 拥有 marketplace 文件时才能解析，所以检查 [marketplace 源](#marketplace-sources)类型：

* **`github`、`git`、`file` 和 `directory`**：Claude Code 拥有 marketplace 的文件。
* **`url`**：Claude Code 仅获取 `marketplace.json`，所以相对路径无法解析。给每个插件一个对象源，如 `github` 或 `git-subdir`。
* **`settings`**：相对路径被直接拒绝。

#### Git submodules in the marketplace repository

当 Claude Code 克隆市场仓库时，它只获取包含插件文件的 git 子模块。需要 Claude Code v2.1.295 或更高版本。

早期版本会获取所有子模块，包括嵌套的子模块。如果仓库中有大型子模块，Claude Code 在获取它时可能会超出[克隆超时](https://code.claude.com/docs/zh-CN/plugins/troubleshooting.md#git-clone-timed-out-after-120s)。

对于位于子目录中的插件（如 `./plugins/formatter`），Claude Code 在以下情况下获取子模块：

* **插件位于子模块内**：子模块是插件的目录或其上级目录
* **子模块位于插件内**：子模块位于插件目录下的任意位置
* **符号链接指向子模块内部**：提交到仓库的链接的目标位于子模块中

位于市场根目录的插件（`"source": "./"`）要么获取仓库中的所有子模块，要么一个都不获取。当以下任一项指向任何子模块内部时，Claude Code 获取所有子模块：

* **组件目录**：根目录下存放插件组件的目录，如 `skills/`、`hooks/` 或 `bin/`
* **插件配置中的路径**：插件的市场条目、其 `plugin.json` 或配置文件（如 `.mcp.json` 或 `hooks/hooks.json`）中的路径
* **符号链接**：提交到仓库的链接
* **插件文件中的文本**：写在配置文件中或组件目录下某个文件中的子模块路径，如 `vendor/tool`

当以上各项都不指向子模块时，Claude Code 不为该插件获取任何子模块。既是项目又是插件的仓库通常含有仅供项目构建使用的子模块，此规则会将它们排除在外。

在无法判断哪些子模块包含插件文件时，Claude Code 也会获取所有子模块，例如：

* **根目录下的 `SKILL.md`**：此时整个仓库就是一个 skill 的目录
* **Claude Code 无法读取的文件**：市场文件、插件的某个配置文件或 `.gitmodules`

如果您使用 `--sparse` 添加市场，或其源设置了 `sparsePaths`，Claude Code 只检出其中指定的目录，不获取任何子模块。早期版本也是如此。

要将插件未使用的子模块排除在克隆之外，请参阅[将未使用的子模块排除在克隆之外](https://code.claude.com/docs/zh-CN/plugins/create-marketplace.md#keep-unused-submodules-out-of-the-clone)。

#### Bare names under pluginRoot

裸名是一个没有 `/` 的单个目录名，如 `"formatter"`。要写裸名而不是 `./` 路径，设置 [`metadata.pluginRoot`](#top-level-fields) 为它们解析的目录。使用 `"pluginRoot": "./plugins"`，`"source": "formatter"` 解析为 `./plugins/formatter`。需要 Claude Code v2.1.239 或更高版本。

`metadata.pluginRoot` 有这些限制：

* 它本身必须是 marketplace 内的相对路径。
* 它对已经以 `./` 开头的源没有影响。
* 包含 `/` 的源，如 `team-a/formatter`，不是裸名，即使设置了 `metadata.pluginRoot` 也仍然需要 `./` 前缀。

### github plugin source

`repo` 采用 `owner/repo` 格式。`ref` 和 `sha` 是可选的。

```json theme={null}
{
  "name": "formatter",
  "source": {
    "source": "github",
    "repo": "your-org/formatter",
    "ref": "v2.0.0",
    "sha": "a1b2c3d4e5f6a7b8c9d0e1f2a3b4c5d6e7f8a9b0"
  }
}
```

### url plugin source

`url` 是一个完整的 git URL：`https://`、`http://`、`file://` 或 `git@`。不需要 `.git` 后缀，所以 Azure DevOps 和 AWS CodeCommit URL 可以按原样工作。此类型不采用 `owner/repo` 简写。

```json theme={null}
{
  "name": "formatter",
  "source": {
    "source": "url",
    "url": "https://gitlab.example.com/your-group/formatter.git",
    "ref": "main"
  }
}
```

### git-subdir plugin source

`url` 接受完整的 git URL 或 GitHub `owner/repo` 简写。`path` 是保存插件的子目录，Claude Code 仅检出该子目录。通过 `https` 或 SSH URL 时，Claude Code 会向服务器请求部分克隆，因此从支持部分克隆的主机获取时，大型 monorepo 中的插件无需下载仓库的其余部分即可安装。

```json theme={null}
{
  "name": "formatter",
  "source": {
    "source": "git-subdir",
    "url": "https://github.com/your-org/monorepo.git",
    "path": "tools/formatter"
  }
}
```

### npm plugin source

一个 `npm` 源采用这些字段：

* `package`：一个 registry 包名，如 `@your-org/formatter`；一个附加了版本的名称，如 `@your-org/formatter@2.0.0`；或一个指向包 tarball 文件的 `https` 链接
* `version`：一个版本、一个 semver 范围或一个 dist-tag，在 `package` 是未附加版本的包名时使用。省略它则获取 `latest`
* `registry`：一个不在默认 registry 上的包的 registry URL

Claude Code 使用你的 npm 客户端获取包。包的安装脚本，如 `preinstall` 或 `postinstall`，永远不会运行，其依赖项在获取期间不会被安装。如果包在其 `package.json` 旁边有一个支持的 lockfile，Claude Code 在单独的步骤中安装这些 [Node.js 包依赖项](https://code.claude.com/docs/zh-CN/plugins/loading.md#node-js-package-dependencies)，也禁用脚本。

Claude Code 在获取任何内容之前会检查 `package` 值。被拒绝的值会导致安装失败，并显示一条指明该值及原因的消息。被拒绝的值包括：

* **git 地址、文件夹或 `file:` 路径，或 `npm:` 别名**：对于 git 仓库，请使用 [`github`、`url` 或 `git-subdir` 源](#plugin-sources)；对于市场中的文件夹，请使用相对路径；对于别名，请使用包自身的名称
* **位于 github.com、gist.github.com、gitlab.com、bitbucket.org 或 git.sr.ht 上的 tarball 链接**：即使该链接是 GitHub release 下载链接也会被拒绝，除非它是 `gitlab.com/api/v4/` 下的 GitLab npm registry 链接
* **通过 `http` 的 tarball 链接**：除非它指向安装用户自己的默认 npm registry，否则会被拒绝

`registry` URL 必须使用 `https`，除非它是安装用户自己的默认 npm registry。对于任何其他 `http` registry，安装会在 npm 与其联系之前失败。

```json theme={null}
{
  "name": "formatter",
  "source": {
    "source": "npm",
    "package": "@your-org/formatter",
    "version": "^2.0.0",
    "registry": "https://npm.example.com"
  }
}
```

### archive plugin source

`url` 必须使用 `https://`，不能指向环回、链接本地或云元数据主机。有关下载的大小、超时、重定向和提取限制，请参阅[保持在托管文件的下载限制内](https://code.claude.com/docs/zh-CN/plugins/host-marketplace.md#stay-within-the-download-limits-for-hosted-files)。

插件根可能在 zip 的顶部或下一个目录。

`sha256` 是存档的摘要，为 64 个十六进制字符，大写或小写。当你设置它时，Claude Code 拒绝不匹配的下载。

```json theme={null}
{
  "name": "formatter",
  "source": {
    "source": "archive",
    "url": "https://artifacts.example.com/formatter-2.0.0.zip",
    "sha256": "6bfa50e3d2e00c052b46abe51fff89346ac803e45771f76dcf6df1ab74cca5e1"
  }
}
```

### command plugin source

当安装在用户机器上的工具生成插件目录时，使用 `command` 源，如一个为用户选择的工具链呈现其插件的 IDE。Claude Code 在用户安装或更新插件时运行该命令，并[每个会话再运行一次](https://code.claude.com/docs/zh-CN/plugins/loading.md#when-a-command-source-re-runs)，所以用户无需重新安装就能获得工具的更改输出。

一个 `command` 源采用这些字段：

* `command`：一个 shell 命令，打印插件目录的绝对路径作为一行并退出 0。Claude Code 在运行前向用户显示整个字符串以供审查。将其写为可打印的 ASCII，最多 500 个字符，没有四个或更多空格的连续。
* `timeout`：从 1 到 600 的整数秒数。默认为 60。
* `mode`：`copy`（默认）或 `link`。参见 [Copy mode and link mode](#copy-mode-and-link-mode)。

```json theme={null}
{
  "name": "formatter",
  "source": {
    "source": "command",
    "command": "my-tool claude-plugin-path",
    "timeout": 120
  }
}
```

有关用户如何接受命令的信息，请参阅 [Install from your shell](https://code.claude.com/docs/zh-CN/plugins/install.md#install-from-your-shell)。有关你更改它后用户看到的内容，请参阅 [Change the command of a command source](https://code.claude.com/docs/zh-CN/plugins/host-marketplace.md#change-the-command-of-a-command-source)。管理员使用 [`disableCommandPluginSources`](https://code.claude.com/docs/zh-CN/settings-reference#disablecommandpluginsources) 关闭命令源。

#### What the command must do

编写命令以满足这些要求：

* **Shell 和工作目录**：Claude Code 通过 `sh` 运行命令，或在 Windows 上通过 `cmd.exe`，从用户的主目录。给出绝对路径或 `PATH` 上的命令。
* **输出**：在 stdout 上打印恰好一行，插件目录的绝对路径，并在 `timeout` 秒内退出 0。
* **目录内容**：该目录在命令退出时保存完整的插件。路径可能因运行而异。

#### Output that fails the install or update

当命令退出非零、运行时间超过 `timeout` 或打印除一个绝对路径之外的任何内容时，安装或更新失败。当打印的目录是以下之一时，它也会失败：

* **没有插件内容**：打印的目录在其顶级没有插件内容，如 `.claude-plugin/` 目录或 `skills/`、`commands/`、`agents/` 或 `hooks/` 目录。
* **会话自己的目录**：打印的目录是 Claude Code 启动的目录，或其父目录之一。
* **网络路径**：在 Windows 上，打印的路径是 UNC 路径。
* **太大而无法复制**：在复制模式下，目录大于 256 MiB 或有超过 20,000 个条目。

#### Copy mode and link mode

`mode` 决定 Claude Code 是复制打印的目录还是就地使用它：

* **`copy`**：Claude Code 将目录复制到插件缓存中，并从复制文件的哈希值派生[插件版本](https://code.claude.com/docs/zh-CN/plugins/loading.md#how-claude-code-computes-the-version)。你的工具可以在命令退出后删除或重写目录。产生相同文件的重新运行计为最新。
* **`link`**：Claude Code 用指向打印目录的每个顶级条目的链接填充插件的缓存条目，并就地加载文件。不复制任何内容，文件内容不被哈希，大小限制不适用。对于太大而无法复制的目录（如呈现的 SDK 导出），使用它。

链接模式插件有这些要求：

* **保持目录就位**：Claude Code 在每次启动时通过链接加载插件，所以打印的目录必须保持在原位，只要插件保持安装。
* **打印不同的路径以表示新内容**：版本来自打印目录的真实路径及其顶级条目，而不是其中的文件。
* **保持顶级符号链接在目录内**：如果顶级条目是指向打印目录外的符号链接，安装失败。
* **包含 `node_modules`**：Claude Code 跳过链接模式插件的 [Node.js 包依赖项安装](https://code.claude.com/docs/zh-CN/plugins/loading.md#node-js-package-dependencies)，所以打印一个已经包含插件需要的包的目录。
* **在目录内启动的会话**：在打印目录或其下方任何地方启动的会话不加载插件。
* **不在 Windows 上**：Claude Code 拒绝在 Windows 上安装链接模式插件。在那里声明 `"mode": "copy"`。

## Marketplace 源

marketplace 源说明 Claude Code 从哪里获取 `marketplace.json`。CLI 在你添加 marketplace 时为你构建一个，你在设置中自己编写一个：

* **[`claude plugin marketplace add`](https://code.claude.com/docs/zh-CN/plugins/cli-reference)**：Claude Code 从你传递的字符串构建源。
* **[`extraKnownMarketplaces`](https://code.claude.com/docs/zh-CN/settings-reference#extraknownmarketplaces)**：你自己将源写成 `source` 对象。
* **[`strictKnownMarketplaces`](https://code.claude.com/docs/zh-CN/settings-reference#strictknownmarketplaces) 和 [`blockedMarketplaces`](https://code.claude.com/docs/zh-CN/plugins/org#restrict-what-users-can-install)**：管理员在这两个策略列表中编写源。`strictKnownMarketplaces` 是允许列表，`blockedMarketplaces` 是阻止列表。

类型名称 `url`、`git` 和 `github` 在 marketplace 源中的含义与在 [插件源](#plugin-sources) 中不同：

| 类型名称 | 作为 marketplace 源 | 作为插件源 |
| :- | :- | :- |
| `url` | 直接链接到 `marketplace.json` 文件，字段为 `url`、`headers` 和 `headersHelper` | 要克隆的 git 存储库，字段为 `url`、`ref` 和 `sha` |
| `git` | 要克隆的 git 存储库，字段为 `url`、`ref`、`path` 和 `sparsePaths` | 不存在 |
| `github` | GitHub 存储库，字段为 `repo`、`ref`、`path` 和 `sparsePaths` | GitHub 存储库，字段为 `repo`、`ref` 和 `sha`，没有 `path` |

该表列出每个 marketplace 源类型及其字段、产生它的 `claude plugin marketplace add` 输入，以及它在三个设置键中的作用。

| 类型 | 字段 | `marketplace add` 输入 | `extraKnownMarketplaces` | `strictKnownMarketplaces` | `blockedMarketplaces` |
| :- | :- | :- | :- | :- | :- |
| `url` | `url`、`headers`、`headersHelper` | 不匹配 git 形式的 `http://` 或 `https://` URL | 加载 | 允许相同的 URL | 阻止相同的 URL |
| `github` | `repo`、`ref`、`path`、`sparsePaths` | `owner/repo`、`owner/repo@ref` 或 `owner/repo#ref` | 加载 | 允许相同的 `repo`、`ref` 和 `path`。`repo` 可能是 `owner/*` | 阻止相同的，以及到相同存储库的 `git` URL |
| `git` | `url`、`ref`、`path`、`sparsePaths` | `user@host:path` URL，或以 `.git` 结尾、包含 `/_git/` 或命名 github.com 或 gitlab.com 存储库的 `http://` 或 `https://` URL。`#ref` 固定 ref | 加载 | 允许相同的 URL、`ref` 和 `path` | 阻止相同的，以及相同 github.com 存储库的其他拼写 |
| `npm` | `package` | 未产生 | 加载失败：`NPM marketplace sources not yet implemented` | 解析但不匹配任何内容，因为没有任何内容注册 `npm` marketplace | 解析但不匹配任何内容 |
| `file` | `path` | `.json` 文件的路径 | 加载 | 允许相同的路径 | 阻止相同的路径 |
| `directory` | `path` | 目录的路径 | 加载 | 允许相同的路径 | 阻止相同的路径 |
| `settings` | `name`、`plugins`、`owner` | 未产生 | 加载 | 允许具有相同 `name` 和相同 `plugins` 的条目 | 阻止相同的 `name` |
| `skills-dir` | none | 不会产生 | 加载失败：`Unsupported marketplace source type` | 允许 [skills-directory 插件](https://code.claude.com/docs/zh-CN/plugins/org.md#keep-skills-directory-plugins-loading)。请参阅[仅在策略列表中有效的源值](#source-values-valid-only-in-policy-lists) | 阻止 skills-directory 插件加载 |
| `hostPattern` | `hostPattern` | 未产生 | 加载失败：`Unsupported marketplace source type` | 允许主机匹配的 `github`、`git` 和 `url` 源 | 阻止这些源 |
| `pathPattern` | `pathPattern` | 未产生 | 加载失败：`Unsupported marketplace source type` | 允许 `path` 匹配的 `file` 和 `directory` 源 | 阻止这些源 |

### 按类型的字段

该表列出每个具有默认值、约束或特定于其类型的含义的 marketplace 源字段。

| 字段 | 类型 | 描述 |
| :- | :- | :- |
| `url` | `url` | 指向 `marketplace.json` 文件的链接。Claude Code 仅下载该文件，因此 marketplace 的插件不能使用 [相对路径源](#relative-path-plugin-source)。请参阅 [保持在托管文件的下载限制内](https://code.claude.com/docs/zh-CN/plugins/host-marketplace.md#stay-within-the-download-limits-for-hosted-files) 了解大小、超时和重定向限制 |
| `url` | `git` | 要克隆的 git 存储库 |
| `headers` | `url` | Claude Code 随获取发送的 HTTP 标头映射，用于经过身份验证的主机 |
| `headersHelper` | `url` | 打印标头的命令，其值太短暂而无法在 `headers` 中列出。需要 Claude Code v2.1.238 或更高版本。请参阅 [验证 archive 下载](https://code.claude.com/docs/zh-CN/plugins/host-marketplace.md#authenticate-archive-downloads) |
| `repo` | `github` | 在 `marketplace add` 和 `extraKnownMarketplaces` 中，`repo` 必须命名一个存储库。`marketplace add` 拒绝 `owner/*` 作为无效的 `owner/repo` 简写；在 `extraKnownMarketplaces` 中 Claude Code 按字面意思取它，克隆失败 |
| `ref` | `github`、`git` | 分支或标签。默认为存储库的默认分支 |
| `path` | `github`、`git` | marketplace 文件在存储库内的路径。默认为 `.claude-plugin/marketplace.json` |
| `path` | `file` | marketplace 文件本身。Claude Code 就地读取它，并将上两级的目录作为 marketplace 根目录，因此将文件保持在 `<root>/.claude-plugin/marketplace.json` |
| `path` | `directory` | marketplace 根目录，包含 `.claude-plugin/marketplace.json` 的目录 |
| `sparsePaths` | `github`、`git` | 用于稀疏检出的目录数组，如 `[".claude-plugin", "plugins"]`。`claude plugin marketplace add --sparse` 设置它 |
| `skipLfs` | `github`、`git` | 接受且无效果。请参阅 [保持插件文件不在 Git LFS 中](https://code.claude.com/docs/zh-CN/plugins/host-marketplace.md#keep-plugin-files-out-of-git-lfs) |
| `name` | `settings` | 必须等于 `extraKnownMarketplaces` 键，不能是 [保留名称](#reserved-names) |
| `plugins` | `settings` | 内联目录，没有托管文件。每个项采用 `name`、`source`、`description`、`version`、`strict`、`headers` 和 `headersHelper`。将每个项的 `source` 写成对象类型，因为相对路径没有存储库来解析 |

### 仅在策略列表中有效的源值

`hostPattern`、`pathPattern`、`skills-dir` 和 `repo` 的 `owner/*` 形式仅在两个策略列表中有效，`strictKnownMarketplaces` 和 `blockedMarketplaces`：

* **`hostPattern` 和 `pathPattern`**：Claude Code 在获取前针对源测试的正则表达式。
* **`skills-dir`**：不是源。如果您设置了 `strictKnownMarketplaces` 但其中没有 `{"source": "skills-dir"}` 条目，[skills-directory 插件](https://code.claude.com/docs/zh-CN/plugins/org.md#keep-skills-directory-plugins-loading)将停止加载。如果您使用 [`strictPluginOnlyCustomization`](https://code.claude.com/docs/zh-CN/settings-reference#strictpluginonlycustomization) 锁定了任何自定义类型，无论该条目是否存在，它们都会停止加载。
* **`owner/*`**：作为 `github` `repo` 值，匹配恰好该 GitHub 所有者下的每个存储库。需要 Claude Code v2.1.223 或更高版本。

有关匹配顺序、精确 `ref` 语义和配方，请参阅 [为你的组织管理插件](https://code.claude.com/docs/zh-CN/plugins/org.md)。

### 设置中的源对象

`extraKnownMarketplaces` 值是从 marketplace 名称到具有 `source` 的对象的映射。此条目从其 `main` 分支的 git 存储库注册 marketplace：

```json theme={null}
{
  "extraKnownMarketplaces": {
    "your-marketplace": {
      "source": {
        "source": "git",
        "url": "https://git.example.com/your-org/your-marketplace.git",
        "ref": "main"
      }
    }
  }
}
```

`strictKnownMarketplaces` 和 `blockedMarketplaces` 是源对象的数组。此允许列表允许一个 GitHub 所有者和一个内部主机：

```json theme={null}
{
  "strictKnownMarketplaces": [
    { "source": "github", "repo": "your-org/*" },
    { "source": "hostPattern", "hostPattern": "^git\\.example\\.com$" }
  ]
}
```

## 验证消息

`claude plugin validate <path>` 接受 marketplace 根目录或 marketplace 文件本身。它打印错误和警告。有关退出代码和 `--strict`，请参阅 [plugin validate](https://code.claude.com/docs/zh-CN/plugins/cli-reference.md#plugin-validate)。

消息通过索引命名插件条目，写作 `plugins.1.source` 或 `plugins[1].source`。

以条目索引和 `plugin.json →` 为前缀的消息，例如 `plugins[2] plugin.json →`，涉及该插件自己的文件。[`claude plugin validate` 报告错误](https://code.claude.com/docs/zh-CN/plugins/troubleshooting#claude-plugin-validate-reports-errors) 列出这些消息及其修复。

提及 Claude Desktop 标志名称的警告，这些名称 Claude Desktop 拒绝。

该表将 marketplace 级别的消息映射到每个消息所涉及的字段。

| 消息 | 级别 | 字段 |
| :- | :- | :- |
| `Marketplace must have a name` | 错误 | `name` 为空 |
| `Marketplace name cannot contain spaces. Use kebab-case (e.g., "my-marketplace")` | 错误 | `name` |
| `Marketplace name cannot contain path separators (/ or \), ".." sequences, or be "."` | 错误 | `name` |
| `Marketplace name impersonates an official Anthropic/Claude marketplace` | 错误 | `name`。请参阅 [保留名称](#reserved-names) |
| `Marketplace name cannot contain control or bidirectional-formatting characters` | 错误 | `name` 包含控制字符，例如转义或换行符，或 Unicode 双向格式化字符 |
| `Marketplace name "inline" is reserved for --plugin-dir session plugins`, and the `builtin`, `skills-dir`, `synced`, `claude-plugin-test`, `npm`, `pip`, `uv`, `cargo`, `github`, and `gh` variants | 错误 | `name` |
| `Author name cannot be empty` | 错误 | `owner.name` |
| `Plugin name cannot contain spaces. Use kebab-case (e.g., "my-plugin")` | 错误 | `plugins[i].name` |
| `Plugin name cannot contain control or bidirectional-formatting characters` | 错误 | `plugins[i].name` |
| `Plugin name "x" is reserved: it passes as one of Anthropic's own` | 错误 | `plugins[i].name`。请参阅 manifest 的 [`name`](https://code.claude.com/docs/zh-CN/plugins/manifest-reference#name) 以了解保留名称 |
| `Plugin name "x" reads as one of Anthropic's own` | 警告 | `plugins[i].name` |
| `Claude Code cannot install plugins from marketplace "x". Each part of a plugin id (plugin@marketplace) may use only the letters a-z and A-Z, digits, ".", "_" and "-", and must start with a letter or digit. Change the marketplace's "name".` | 错误 | `name` |
| `Claude Code cannot install plugin "x". Each part of a plugin id (plugin@marketplace) may use only the letters a-z and A-Z, digits, ".", "_" and "-", and must start with a letter or digit. Change this entry's "name".` | 错误 | `plugins[i].name` |
| `Duplicate plugin name "x" found in marketplace` | 错误 | 两个条目共享一个 `name` |
| `plugins.i.source: Invalid input` | 错误 | 该条目的 `source` 与任何类型都不匹配。请参阅 [Invalid input on a source](#invalid-input-on-a-source) |
| `plugins.i.source: Invalid string: must start with "./"` | 错误 | 缺少开头 `./` 的相对路径 `source`。在 v2.1.285 之前，此错误会改为打印 `Invalid input` |
| `plugins[i].source: Path contains "..": <path>` | 错误 | 转义 marketplace 根目录的相对 `source` |
| `source.source: 'unsupported' is a parse-time placeholder and cannot be authored` | 错误 | `plugins[i].source` |
| `Plugin "x" sets headersHelper but is not "strict": false` | 错误 | `plugins[i].headersHelper`，在 `archive` 条目上 |
| `chain does not resolve (<reason>) — target must be a name in plugins[], a key in renames, or null` | 错误 | `renames.<old>` |
| `target "x" is not a valid plugin name (PluginIdSchema)` | 错误 | `renames.<old>` |
| `Unknown field 'x'. Claude Code ignores it at load time.` | 警告 | 顶级、`metadata` 下、条目中或条目的 `relevance` 下的命名键 |
| `Marketplace has no plugins defined` | 警告 | `plugins` 为空 |
| `Plugin "x" sets headers/headersHelper, which only apply to "archive" sources; they have no effect on this entry.` | 警告 | `plugins[i].headers` 或 `plugins[i].headersHelper`，在 `source` 不是 `archive` 的条目上 |
| `Plugin "x" fetches its archive with a headersHelper but sets no sha256 pin` | 警告 | `plugins[i].source.sha256` |
| `Header "x" is a request-routing/identity header that catalog entries may not set; Claude Code drops it at download time.` | 警告 | `plugins[i].headers.<name>` |
| `Local source "x" is or traverses a symlink, so <path> was not read` | 警告 | `plugins[i].source` |
| `No marketplace description provided. Adding a description helps users understand what this marketplace offers` | 警告 | `description` |
| `Entry declares version "x" but <path>/plugin.json says "y". At install time, plugin.json wins` | 警告 | `plugins[i].version`，在相对路径条目上 |
| `'relevance' must be an object containing topic and signals; got <type>. It will be ignored at load time.` | 警告 | `plugins[i].relevance` |
| `'metadata' must be a free-form object; got <type>. It will be ignored at load time.` | 警告 | `plugins[i].metadata` |
| `'experimental' must be an object containing component declarations; got <type>. It will be ignored at load time.` | 警告 | `plugins[i].experimental` |
| `Marketplace name "x" is reserved in Claude Desktop` | 警告 | `name` 是 `org`、`org-provisioned` 或 `unknown`。Claude Desktop 拒绝该 marketplace |
| `Marketplace name "x" is not accepted by Claude Desktop (letters, digits, ".", "_", "-"; must start alphanumeric; max 128 chars)` | 警告 | `name`。Claude Desktop 拒绝该 marketplace |
| `Plugin name "x" is not accepted by Claude Desktop (letters, digits, ".", "_", "-"; must start alphanumeric; max 128 chars)` | 警告 | `plugins[i].name`。Claude Desktop 删除该条目 |

### Invalid input on a source

`source` 上的 `Invalid input` 意味着该对象与任何源类型都不匹配。检查这些原因：

* 包含 `..` 的 `npm` `package`
* 不是 [plugin sources](#plugin-sources) 之一的 `source` 类型
* 已知类型缺少必需字段或字段类型错误，例如没有 `repo` 的 `github`

不以 `./` 开头的相对路径（`"."` 或 [`metadata.pluginRoot` 下的裸名称](#relative-path-plugin-source) 除外）会失败并显示 `Invalid string: must start with "./"`。在 v2.1.285 之前，它会像上述原因一样打印 `Invalid input`。

### 验证未捕获的失败

`claude plugin validate` 不会报告每个失败。写作文件路径或数组的条目 `hooks` 通过验证，错误仅在插件加载时出现，如 [Hooks in an entry](#hooks-in-an-entry) 所述。获取 `source` 的错误也仅在安装后出现，不在验证中出现。

[`claude plugin list`](https://code.claude.com/docs/zh-CN/plugins/cli-reference) 显示加载失败的插件及其错误，[Troubleshoot plugins](https://code.claude.com/docs/zh-CN/plugins/troubleshooting.md) 涵盖加载时字符串。

## 后续步骤

* [创建 marketplace](https://code.claude.com/docs/zh-CN/plugins/create-marketplace.md)：从这些字段构建 marketplace 并在本地安装
* [托管和维护 marketplace](https://code.claude.com/docs/zh-CN/plugins/host-marketplace.md)：放置文件的位置以及用户如何接收更改
* [插件清单参考](https://code.claude.com/docs/zh-CN/plugins/manifest-reference.md)：条目可以覆盖的 `plugin.json` 字段
* [为你的组织管理插件](https://code.claude.com/docs/zh-CN/plugins/org.md)：使用这些源值的允许列表和阻止列表配方
