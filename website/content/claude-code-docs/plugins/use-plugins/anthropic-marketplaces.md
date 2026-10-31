> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Anthropic 的插件市场

> Anthropic 为 Claude Code 提供的官方和社区插件市场：它们的名称、仓库、如何添加每个市场，以及在哪里浏览它们的插件。

Anthropic 为 Claude Code 发布了两个通用插件市场：[官方](https://github.com/anthropics/claude-plugins-official)和[社区](https://github.com/anthropics/claude-plugins-community)。每个都是其自己的 GitHub 仓库中的插件目录。当您在 Claude Code 会话中从其中一个安装插件时，需要在 `@` 后面输入市场的名称，如 `/plugin install commit-commands@claude-plugins-official`。

使用此页面来区分这两个市场，并找到检查官方市场是否包含给定插件的位置。

> [!NOTE]
>
> 这些情况在其他页面上有介绍：
>
> * **如何安装插件**：请参阅[安装插件](https://code.claude.com/docs/zh-CN/plugins/install.md)
> * **安装失败**：请参阅[插件故障排除](https://code.claude.com/docs/zh-CN/plugins/troubleshooting.md)

转到你需要的页面部分：

* 要按仓库、市场名称和获取方式来区分这两个市场，请参阅 [Anthropic 的插件市场](#anthropic’s-marketplaces)。
* 要在官方市场中查找插件，请参阅[在官方市场中查找插件](#find-plugins-in-the-official-marketplace)。

## Anthropic 的插件市场

市场是一个插件目录，由仓库在其 `.claude-plugin/marketplace.json` 文件中定义。官方市场和社区市场各自来自自己的 GitHub 仓库。Anthropic 还发布主题特定的市场，例如 `anthropics/skills` 和 `anthropics/knowledge-work-plugins`，您可以在 Claude Code 会话中使用 `/plugin marketplace add <owner>/<repo>` 添加。

此表给出了每个市场的存储库和市场名称，这是你从该市场安装插件时在 `@` 后面输入的内容。社区市场的名称是 `claude-community`，而不是其存储库名称。

| | 官方 | 社区 |
| :- | :- | :- |
| 仓库 | [`anthropics/claude-plugins-official`](https://github.com/anthropics/claude-plugins-official) | [`anthropics/claude-plugins-community`](https://github.com/anthropics/claude-plugins-community) |
| 市场名称 | `claude-plugins-official` | `claude-community` |
| 其中包含的内容 | Anthropic 维护的插件，加上来自合作伙伴和其他作者的插件 | 第三方插件，由其作者提交给 Anthropic |
| 获取方式 | Claude Code 在您首次启动交互式终端会话时添加它，除非[托管策略](https://code.claude.com/docs/zh-CN/plugins/org.md#allow-the-official-marketplace-and-your-own)或 `CLAUDE_CODE_DISABLE_OFFICIAL_MARKETPLACE_AUTOINSTALL` 阻止它。如果缺少，请参阅[市场 `claude-plugins-official` 未找到](https://code.claude.com/docs/zh-CN/plugins/troubleshooting#marketplace-claude-plugins-official-not-found) | 您在 Claude Code 会话中使用 `/plugin marketplace add anthropics/claude-plugins-community` 添加它 |

如果你编写了插件并希望其他人安装它，请参阅[发布插件](https://code.claude.com/docs/zh-CN/plugins/publish.md)，其中涵盖了你自己的市场和提交到 Anthropic 的目录。

### 已弃用的 `claude-code-plugins` 市场

> [!WARNING]
>
> `anthropics/claude-code` 中的 `claude-code-plugins` 市场已弃用。要添加插件，请在 shell 中运行 `claude plugin install <plugin>@claude-plugins-official`，并跳过任何运行 `/plugin marketplace add anthropics/claude-code` 的教程步骤。

除非您安装了下面两个插件之一，否则可以保持您的 `claude-code-plugins` 插件和该市场不变。它们会从 `claude-plugins-official` 仓库进行更新。

* **`ralph-wiggum`**：此插件现在名为 `ralph-loop`，其命令以 `/ralph-loop:` 开头。如果这些命令不可用，请在 shell 中运行 `claude plugin install ralph-loop@claude-plugins-official`。
* **`claude-opus-4-5-migration`**：此插件已停用，请在 shell 中运行 `claude plugin uninstall claude-opus-4-5-migration@claude-code-plugins` 将其移除。

如果有任何其他插件停止工作，请参阅[插件已安装但无法工作](https://code.claude.com/docs/zh-CN/plugins/troubleshooting.md#plugin-installed-but-not-working)。

## 在官方市场中查找插件

默认情况下，Claude Code 为你添加了官方市场 `claude-plugins-official`。它列出的大部分内容来自合作伙伴和其他作者，而不是来自 Anthropic：工具供应商发布连接 Claude Code 到其服务的插件，Anthropic 维护一个较小的自己的插件集，例如 `commit-commands`、`code-review`、`feature-dev` 和[语言服务器插件](https://code.claude.com/docs/zh-CN/plugins/code-intelligence.md)。目录经常变化，所以此页面不列出它。

要查看其中的内容，请在 Claude Code 会话中使用 `/plugin` 的 **Discover** 选项卡（你可以搜索），或在网络上浏览 [Claude Marketplace](https://claude.com/marketplace/plugins)。

## 从 Anthropic 的市场浏览和安装

你可以在 Claude Code、网络或 GitHub 上搜索 Anthropic 的市场中的插件：

* **在 Claude Code 中，通过浏览**：在交互式会话中运行 `/plugin`。其 **Discover** 选项卡列出了你的市场中的插件。
* **在 Claude Code 中，按名称**：在会话中运行 `/plugin install <name>`，它会在你的市场中查找该名称。如果插件在其中一个中，其详细信息会在 `/plugin` 面板中打开，在你选择[安装范围](https://code.claude.com/docs/zh-CN/plugins/install.md#install-a-plugin)并在那里确认之前，不会安装任何内容。如果不在，你会看到 `Plugin "<name>" not found in any marketplace`。
* **在网络上**：在 [Claude Marketplace](https://claude.com/marketplace/plugins) 上搜索完整目录，它显示安装计数并标记一些插件为 **Anthropic verified**。
* **在 GitHub 上**：打开市场存储库中的 `.claude-plugin/marketplace.json`，例如 [`anthropics/claude-plugins-official`](https://github.com/anthropics/claude-plugins-official)。该文件就是目录本身。

Anthropic 的目录与这些市场分开。该目录是 claude.ai 上的目录。你从 claude.ai 上的目录添加的插件通过[账户同步](https://code.claude.com/docs/zh-CN/plugins/loading.md#synced-plugins)到达 Claude Code。要在那里列出你自己的插件，请参阅[提交到 Anthropic 的目录](https://code.claude.com/docs/zh-CN/plugins/publish.md#submit-to-anthropics-directory)。

要从桌面应用或脚本安装，或查看云会话加载的内容，请参阅[安装插件](https://code.claude.com/docs/zh-CN/plugins/install.md)。

### 添加社区市场

社区市场在您添加之前不会注册。在 Claude Code 会话中运行 `/plugin marketplace add anthropics/claude-plugins-community`，然后使用 `@claude-community` 后缀安装。

如果 `claude-plugins-official` 不在 `/plugin` 的 **Marketplaces** 选项卡上，用 `/plugin marketplace add anthropics/claude-plugins-official` 以相同的方式添加它。

对于 `not found` 错误和无法添加的市场，请参阅[插件故障排除](https://code.claude.com/docs/zh-CN/plugins/troubleshooting.md#install-a-plugin)。

## 第三方市场

许多流行的插件不在任何 Anthropic 市场中。它们在其作者自己的市场中，通常是一个 GitHub 存储库，其根目录中有 `.claude-plugin/marketplace.json`。

Anthropic 不审查第三方市场，所以在添加一个之前，请阅读[插件安全和信任](https://code.claude.com/docs/zh-CN/plugins/security.md)。

要使用第三方市场，在 Claude Code 会话中使用 `/plugin marketplace add <owner>/<repo>` 添加其存储库，然后使用 `/plugin install <plugin>@<marketplace-name>` 安装。市场名称是该 `marketplace.json` 的 `name` 字段，Claude Code 在添加市场后会打印它。

有关添加市场的其他方式，请参阅[添加市场](https://code.claude.com/docs/zh-CN/plugins/install.md#add-a-marketplace)。

## 后续步骤

* [安装和管理插件](https://code.claude.com/docs/zh-CN/plugins/install.md)：从这些市场之一安装插件并选择范围
* [插件安全和信任](https://code.claude.com/docs/zh-CN/plugins/security.md)：插件可以在你的机器上做什么，以及在安装前如何审查插件
* [代码智能插件](https://code.claude.com/docs/zh-CN/plugins/code-intelligence.md)：安装官方市场的语言服务器插件之一
* [创建市场](https://code.claude.com/docs/zh-CN/plugins/create-marketplace.md)：在 Anthropic 的市场旁边运行你自己的市场
