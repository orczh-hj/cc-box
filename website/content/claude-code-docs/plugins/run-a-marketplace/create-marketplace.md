> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# 创建一个 marketplace

> 从 marketplace.json 文件构建一个 plugin marketplace，并在托管之前在本地测试它。

plugin marketplace 是一个目录或仓库，包含一个 `.claude-plugin/marketplace.json` 文件，该文件列出你的 plugins 以及从哪里获取每一个。你将目录推送到 git 主机，任何有权限的人都可以用一个命令在 Claude Code 中注册它，并从中安装你的 plugins。

当你想让一个你选择的群体（例如你的团队或组织）安装你的 plugins 并继续从你控制的目录接收更新时，创建你自己的 marketplace。该仓库可以是私有的，可以列出任意数量的 plugins，管理员可以[在每台机器上要求它](https://code.claude.com/docs/zh-CN/plugins/org.md)。

> [!NOTE]
>
> 这些情况在其他页面上有介绍：
>
> * **与少数人共享一个 plugin**：将 plugin 的目录或其 `.zip` 文件发送给他们。请参阅[不使用 marketplace 共享 plugin](https://code.claude.com/docs/zh-CN/plugins/publish.md#share-a-plugin-without-a-marketplace)。
> * **向所有人提供一个 plugin**：将其提交到 Anthropic 的社区 marketplace。请参阅[提交到社区 marketplace](https://code.claude.com/docs/zh-CN/plugins/publish.md#submit-to-anthropics-directory)。
> * **自己使用一个 plugin**：使用 `--plugin-dir` 加载它或将其保存在你的 skills 目录中。请参阅[不使用 marketplace 开发](https://code.claude.com/docs/zh-CN/plugins/create.md#develop-without-a-marketplace)。

从[创建一个 marketplace](#create-a-marketplace) 开始，在你自己的机器上构建一个并从中安装一个 plugin，然后[添加更多 plugin 条目](#add-plugin-entries)。

## 创建一个 marketplace

以下步骤在你的机器上创建一个 marketplace，向其中添加一个 plugin，在 Claude Code 中注册它，并从中安装该 plugin。这是整个循环，也是你的用户一旦你在他们可以访问的地方托管 marketplace 后所经历的相同循环。从你想要创建 `my-marketplace/` 的目录在你的 shell 中运行每个命令。

你需要一个 plugin 来列出。该示例使用来自[创建你的第一个 plugin](https://code.claude.com/docs/zh-CN/plugins/create.md#create-your-first-plugin) 的 `my-first-plugin`，这是一个具有一个 skill 的 plugin，你可以将其作为 `/my-first-plugin:hello` 运行；如果你还没有 plugin，请先构建它。要使用你自己的 plugin，请在步骤说 `my-first-plugin` 的地方替换其目录和其 `name`。有关 plugin 目录可以包含的内容，请参阅[plugin 目录浏览器](https://code.claude.com/docs/zh-CN/plugins/components.md#explore-the-plugin-directory)。

**1. 设置 marketplace 目录**

marketplace 是一个包含 `.claude-plugin/marketplace.json` 文件的目录，加上它列出的 plugins。创建 marketplace 目录及其 `.claude-plugin/` 文件夹，然后在 `plugins/` 下复制你的 plugin：

```bash theme={null}
mkdir -p my-marketplace/.claude-plugin my-marketplace/plugins
cp -r my-first-plugin my-marketplace/plugins/
```

检查 plugin 现在所在的位置是否有效，以便任何后续错误都是关于 marketplace 而不是 plugin 的：

```bash theme={null}
claude plugin validate ./my-marketplace/plugins/my-first-plugin
```

输出的最后一行读作 `✔ Validation passed`。

**2. 创建 marketplace 文件**

在 `my-marketplace/.claude-plugin/marketplace.json` 处保存 `marketplace.json`。该文件需要一个 `name`、一个 `owner` 和一个 `plugins` 数组。

`plugins` 中的每个对象都是一个 plugin 条目，需要一个 `name` 和一个 `source`。将条目的 `source` 写成从 marketplace 根目录的路径。根目录是 `my-marketplace/`，即包含 `.claude-plugin/` 的目录。

```json my-marketplace/.claude-plugin/marketplace.json theme={null}
{
  "name": "my-marketplace",
  "description": "Plugins for my team",
  "owner": {
    "name": "Your Name"
  },
  "plugins": [
    {
      "name": "my-first-plugin",
      "source": "./plugins/my-first-plugin",
      "description": "A greeting plugin to learn the basics"
    }
  ]
}
```

**3. 验证 marketplace**

在 marketplace 目录上运行 `claude plugin validate` 以检查 JSON 语法、必需字段以及其 `.claude-plugin/marketplace.json` 中的每个 plugin 条目。

```bash theme={null}
claude plugin validate ./my-marketplace
```

对于在步骤 2 中编写的文件，输出的最后一行读作 `✔ Validation passed`。

**4. 添加 marketplace 并安装 plugin**

将目录注册为 marketplace。

```bash theme={null}
claude plugin marketplace add ./my-marketplace
```

该命令打印 `✔ Successfully added marketplace: my-marketplace (declared in user settings)`，这意味着 marketplace 已记录在你的用户设置文件中。

安装 plugin。安装 id 是条目的 `name`、一个 `@` 和 marketplace 的 `name`。

```bash theme={null}
claude plugin install my-first-plugin@my-marketplace
```

该命令打印 `✔ Successfully installed plugin: my-first-plugin@my-marketplace (scope: user)`。

在会话内，`/plugin marketplace add ./my-marketplace` 以相同的方式注册 marketplace。`/plugin install my-first-plugin@my-marketplace` 在 `/plugin` 面板中打开 plugin 的详细信息，你可以在其中安装它。有关该流程，请参阅[安装和管理 plugins](https://code.claude.com/docs/zh-CN/plugins/install.md)。

**5. 确认 plugin 已加载**

列出已安装的 plugins。

```bash theme={null}
claude plugin list
```

输出列出 `my-first-plugin@my-marketplace`，其 `Status: ✔ enabled`。

要查看 plugin 加载了什么，请显示其详细信息。

```bash theme={null}
claude plugin details my-first-plugin
```

`Component inventory` 部分读作 `Skills (1)  hello`。

要运行该 skill，启动一个会话并输入 `/my-first-plugin:hello`。Claude 会向你问好。该命令以 plugin 的名称作为前缀，就像每个 plugin skill 的名称一样。

## 添加 plugin 条目

你分发的每个 plugin 都是 `marketplace.json` 的 `plugins` 数组中的一个对象。要添加第二个 plugin，请添加第二个对象。这些字段涵盖了大多数条目：

* `name`：人们在安装时在 `@` 之前输入的标识符。[Plugin 条目](https://code.claude.com/docs/zh-CN/plugins/marketplace-reference.md#plugin-entries)给出了名称可以使用的字符。
* `source`：Claude Code 从哪里获取 plugin。对于 marketplace 目录内的 plugin，写一个相对路径字符串，如[演练](#create-a-marketplace)中所示，或对于目录外的 plugin，写一个源对象。请参阅[选择 plugin 源](#choose-a-plugin-source)。
* `description`：人们在 `/plugin` 中浏览你的 marketplace 时在 plugin 旁边看到的行。

有关完整的字段列表，请参阅[插件条目](https://code.claude.com/docs/zh-CN/plugins/marketplace-reference.md#plugin-entries)，其中还介绍了条目可以设置的 [`plugin.json`](https://code.claude.com/docs/zh-CN/plugins/manifest-reference) 字段以及这些字段何时生效。

## Plugin 条目的规则

来自新 marketplace 的大多数失败安装来自于从错误目录编写的相对路径，或来自与 plugin 的 `plugin.json` 中的 `name` 不同的条目名称。

### 从 marketplace 根目录编写相对路径

marketplace 根目录是包含 `.claude-plugin/` 的目录。在[演练](#create-a-marketplace)中，那是 `my-marketplace/`，所以条目的 `source` 是 `"./plugins/my-first-plugin"`。该路径不是从 `.claude-plugin/` 内部开始的，所以不要使用 `..` 来离开它。

包含 `..` 的路径和指向不存在目录的路径在不同的命令处失败：

* **包含 `..` 的路径**：`claude plugin validate` 将条目报告为无效。消息以 `Path contains "..": ./../plugins/my-first-plugin` 开头。
* **指向不存在目录的路径**：`claude plugin validate` 通过。`claude plugin install` 失败，显示 `Source path does not exist: <path>`，其中 `<path>` 是 Claude Code 检查的绝对位置。

### 保持条目名称和清单名称相同

marketplace plugin 在 `marketplace.json` 中有一个条目 `name` 和在其自己的 `plugin.json` 中有一个 `name`，称为清单名称。每个名称出现在不同的地方：

* **条目名称**：安装 id，`<entry-name>@<marketplace>`。这是人们输入来安装的内容，`claude plugin list` 显示的内容，以及 Claude Code 在他们的设置文件中的 [`enabledPlugins`](https://code.claude.com/docs/zh-CN/settings-reference#enabledplugins) 下写入的键。
* **清单名称**：plugin 的 skills 上的前缀，以及 `claude plugin details` 接受的名称。

当两个名称不同且有人按清单名称安装时，Claude Code 报告 `Plugin "<manifest-name>" not found in marketplace "<marketplace>"`。保持两个名称相同。有关 Claude Code 如何使用这两个名称的更多信息，请参阅[Plugin 加载参考](https://code.claude.com/docs/zh-CN/plugins/loading.md#find-where-a-plugin-came-from)。

### 避免将未使用的子模块拉入克隆

当有人添加您的市场时，Claude Code 会克隆该仓库，并且只获取包含插件文件的 git 子模块。需要 Claude Code v2.1.295 或更高版本。

您可以通过每个插件的放置位置来决定 Claude Code 获取哪些子模块：

* **位于子目录中的插件**：Claude Code 会获取插件目录内的子模块，以及包含该插件的子模块。将插件需要的子模块放在插件目录内，并将不需要的子模块放在目录外
* **位于市场根目录的插件**：只要插件的某个文件引用了任一子模块内的路径，Claude Code 就会获取仓库中的所有子模块。如果您的仓库中有一个仅供构建使用的大型子模块，请将插件移到子目录中

如果位于根目录的插件运行来自子模块的文件，请在插件的 hook 或服务器配置中完整写出该文件的路径，例如 `${CLAUDE_PLUGIN_ROOT}/vendor/tool/run.sh`。Claude Code 会读取这些路径来判断插件是否需要这些子模块。它无法识别脚本在运行时拼接出的路径。

有关完整规则，请参阅[市场仓库中的 Git 子模块](https://code.claude.com/docs/zh-CN/plugins/marketplace-reference.md#git-submodules-in-the-marketplace-repository)。

## 选择 plugin 源

`marketplace.json` 中的每个 plugin 条目都有一个 `source`，告诉 Claude Code 从哪里获取那个 plugin。根据 plugin 文件的存储位置选择源。该表列出了大多数 marketplace 所有者使用的源。

| 源 | 何时使用 | 最小 `source` 值 |
| :- | :- | :- |
| 相对路径 | plugin 的文件在 marketplace 目录内 | `"./plugins/my-first-plugin"` |
| `github` | plugin 是其自己的 GitHub 仓库 | `{ "source": "github", "repo": "your-org/my-first-plugin" }` |
| `git-subdir` | plugin 是某个其他仓库的子目录，例如 monorepo | `{ "source": "git-subdir", "url": "your-org/monorepo", "path": "tools/my-first-plugin" }` |

在 `git-subdir` 源中，`url` 接受 git URL 或 `owner/repo` GitHub 简写。

plugin 也可以来自以下源类型之一：

* `url`：任何主机上的 git 仓库 URL
* `archive`：通过 HTTPS 下载的 zip 文件
* `npm`：npm 包
* `command`：通过在安装 plugin 的机器上运行命令生成的目录

有关每种源类型的字段，以及将基于 git 的源固定到 `ref` 或 `sha`，请参阅[Plugin 源](https://code.claude.com/docs/zh-CN/plugins/marketplace-reference.md#plugin-sources)。

## 验证和测试

当你添加 plugins 时，在每次编辑后在你的 shell 中运行 `claude plugin validate ./my-marketplace`，并在分享之前从你自己机器上的 marketplace 安装。验证和安装会捕获不同的问题。

### 验证报告的问题

`claude plugin validate` 仅读取 marketplace 目录内的文件。它报告：

* JSON 语法错误，如 `json: Invalid JSON syntax: <reason>`
* 缺少必需字段，例如 `owner: Invalid input`
* 违反[marketplace 参考](https://code.claude.com/docs/zh-CN/plugins/marketplace-reference.md#top-level-fields)中命名规则的 marketplace 或 plugin 名称
* 包含 `..` 的相对 `source`
* 顶级或 plugin 条目中的未知字段，作为警告
* 每个相对路径 plugin 的 `plugin.json` 中的问题，如 `plugins[N] plugin.json → <field>: <message>`

有关 `validate` 可以打印的每条消息，请参阅[验证消息](https://code.claude.com/docs/zh-CN/plugins/marketplace-reference.md#validation-messages)。有关其标志和退出代码，请参阅 [`plugin validate`](https://code.claude.com/docs/zh-CN/plugins/cli-reference#plugin-validate)。

### 添加或安装时出现的问题

`claude plugin validate` 不报告的问题在你添加 marketplace 或从中安装时出现：

* **当你添加 marketplace 时**：确切的[官方 marketplace 名称](https://code.claude.com/docs/zh-CN/plugins/marketplace-reference.md#reserved-names)，例如 `claude-plugins-official`，通过验证。当你添加具有其中一个名称的 marketplace 时，Claude Code 拒绝它，消息以 `The name '<name>' is reserved for official Anthropic marketplaces` 开头。
* **当你安装一个 plugin 时**：
  * Claude Code 在你安装 plugin 时首先获取 `github`、`git-subdir` 或其他远程源，所以错误的 `repo` 或 `path` 会在那时出现。
  * 一个相对 `source` 的目录不存在也会在安装时失败，显示 `Source path does not exist: <path>`。

### 测试对 plugin 的编辑

在[演练](#create-a-marketplace)中，你从具有相对路径 `source` 的本地目录添加了 `my-marketplace`。使用该设置，Claude Code 直接从 `my-marketplace/plugins/` 读取 plugin 的文件。你的编辑在下一个会话开始时或当你在会话中运行 `/reload-plugins` 时生效，无需更改 plugin 的 `version`。

从你托管的 marketplace 安装的人会在 plugin 缓存中获得一个副本。有关他们如何接收新版本，请参阅[保持用户最新](https://code.claude.com/docs/zh-CN/plugins/host-marketplace.md#keep-users-up-to-date)。

### 删除 marketplace 以重新开始

要删除所有内容并重新开始，在你的 shell 中运行 `claude plugin marketplace remove my-marketplace`。该命令删除 marketplace 并卸载其 plugins。

## 托管你的 marketplace

当您能够像[创建市场](#create-a-marketplace)中那样，在自己的机器上从市场安装插件后，请将市场目录推送到 git 托管平台。推送之前，请为市场指定一个其他作者不太可能使用的顶层 `name`，因为用户[不能拥有两个同名的市场](https://code.claude.com/docs/zh-CN/plugins/marketplace-reference.md#marketplace-file)。

你的队友然后在他们的 shell 中为 GitHub 仓库运行 `claude plugin marketplace add <owner>/<repo>`，或使用仓库 URL 运行相同的命令。然后他们按名称安装 plugin，如[演练](#create-a-marketplace)中所示。

有关私有仓库访问、更新、版本控制以及重命名或删除条目，请参阅[托管和维护 marketplace](https://code.claude.com/docs/zh-CN/plugins/host-marketplace.md)。

## 后续步骤

* [托管和维护 marketplace](https://code.claude.com/docs/zh-CN/plugins/host-marketplace.md)：选择一个主机、保持用户最新并安全地重命名或删除 plugins
* [Marketplace 参考](https://code.claude.com/docs/zh-CN/plugins/marketplace-reference.md)：`marketplace.json` 字段和源类型
* [为你的组织管理 plugins](https://code.claude.com/docs/zh-CN/plugins/org.md)：在每台机器上要求你的 marketplace 及其 plugins
* [按相关性建议 plugins](https://code.claude.com/docs/zh-CN/plugins/relevance.md)：当会话匹配时，让 Claude Code 建议来自你的 marketplace 的 plugin
