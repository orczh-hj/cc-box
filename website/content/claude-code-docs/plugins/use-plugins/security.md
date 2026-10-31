> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# 插件安全和信任

> 在安装插件之前决定是否信任它，从插件在您的机器上可以做什么，到如何审查和删除它。

您安装的 Claude Code 插件可以使用您的用户权限在您的机器上执行任意代码。

您从一个市场安装插件，市场是 Claude Code 从中获取插件的目录。某些市场名称是[为 Anthropic 自己的市场保留的](#marketplace-tiers)，其他所有市场都是第三方的。市场的名称告诉您谁发布了目录，而不是其中每个插件的功能，所以无论插件来自哪个市场，都要[在安装前审查插件](#review-a-plugin-before-you-install)。

如果您正在决定是否安装插件，或者您在审查工具以供您的团队使用，请阅读本页。

> [!NOTE]
>
> 这些情况在其他页面上有介绍：
>
> * **Claude Code 自己的安全模型**：请参阅[安全](https://code.claude.com/docs/zh-CN/security.md)
> * **为组织限制或要求插件**：请参阅[为您的组织管理插件](https://code.claude.com/docs/zh-CN/plugins/org.md)
> * **`security-guidance` 或 `claude-security` 插件**：本页不是关于它们的。请参阅[`security-guidance`](https://code.claude.com/docs/zh-CN/security-guidance) 和 [`claude-security`](https://code.claude.com/docs/zh-CN/claude-security)

首先从[插件可以做什么](#understand-what-a-plugin-can-do)和[哪些市场是 Anthropic 的](#marketplace-tiers)开始，然后[在安装前审查插件](#review-a-plugin-before-you-install)。

## 了解插件可以做什么

插件可以包含在您的机器上使用您的用户权限运行代码的内容，以及作为指令进入 Claude 上下文的内容，所以[在安装前审查插件](#review-a-plugin-before-you-install)。以下是已安装的插件可以做的事情：

* **Hooks**：插件的 [hooks](https://code.claude.com/docs/zh-CN/hooks.md) 在 Claude Code 生命周期中的特定点（例如工具调用之前或之后）作为 shell 命令运行。
* **Monitors**：插件的 [monitors](https://code.claude.com/docs/zh-CN/plugins/components.md#monitors) 作为后台 shell 命令运行，Claude Code 会在会话开始时、您重新加载插件时，或某个指定的 skill 首次运行时自行启动它们。
* **Mods**：插件的 [mod](https://code.claude.com/docs/zh-CN/plugins/mods/overview.md) 在 Claude Code 内使用您的权限运行 JavaScript。要在安装前列出 mod 的功能，请参阅[决定是否信任 mod](https://code.claude.com/docs/zh-CN/plugins/mods/overview.md#decide-whether-to-trust-a-mod)。
* **MCP 和 LSP 服务器**：Claude Code 连接到启用的插件声明的 [MCP 服务器](https://code.claude.com/docs/zh-CN/mcp.md)，并为 Claude 提供它们的工具。stdio MCP 服务器作为 Claude Code 在您的机器上启动的进程运行。Claude Code 也启动插件声明的语言服务器。
* **`bin/` 目录**：Claude Code 将每个启用的插件的 `bin/` 目录添加到 Bash 工具 shell 的 `PATH` 中，所以 Claude 的 Bash 命令可以运行那里的任何可执行文件。Claude 通过 [PowerShell 工具](https://code.claude.com/docs/zh-CN/tools-reference.md#powershell-tool)运行的命令在[沙箱](https://code.claude.com/docs/zh-CN/sandboxing.md)外运行时，其 `PATH` 中会包含这些目录；在沙箱内运行时则不包含。
* **Skills、commands 和 agents**：这些作为指令进入 Claude 的上下文，所以它们影响 Claude 对它已有的工具的使用。
* **更新**：当您安装插件的市场启用自动更新时，Claude Code 在后台更新该插件，所以您审查的文件可能会在磁盘上更改。[自动更新何时运行](https://code.claude.com/docs/zh-CN/plugins/loading.md#when-auto-update-runs)有时间安排。要按市场打开或关闭自动更新，请参阅[保持插件更新](https://code.claude.com/docs/zh-CN/plugins/install.md#keep-plugins-updated)。

Claude Code 的[权限规则](https://code.claude.com/docs/zh-CN/permissions.md)和[沙箱](https://code.claude.com/docs/zh-CN/sandboxing.md)涵盖 Claude 进行的工具调用，而不是插件自己运行的代码：

* **Hooks、monitors 和服务器进程**：命令 hook 和 monitors 是使用您的完整用户权限运行的 shell 命令。Claude Code 在沙箱外运行 hook、monitors、MCP 服务器、LSP 服务器以及 [mod](https://code.claude.com/docs/zh-CN/plugins/mods/overview.md#what-a-mod-can-reach) 启动的进程。
* **Claude 的工具调用**：对插件的 MCP 工具之一的调用，以及运行插件 `bin/` 中的可执行文件的 Bash 命令，都是工具调用，所以您的权限规则适用于它们。关于 mod 对工具调用可以做什么，请参阅[决定是否信任 mod](https://code.claude.com/docs/zh-CN/plugins/mods/overview.md#decide-whether-to-trust-a-mod)。

安装插件也会启用它，除非其清单或市场条目设置了 [`defaultEnabled: false`](https://code.claude.com/docs/zh-CN/plugins/install#choose-an-install-scope)，并且您自己没有启用它。

要删除您不再信任的插件，请参阅[删除您不再信任的插件](#remove-a-plugin-you-no-longer-trust)。

## 按名称识别 Anthropic 的市场

市场的名称将其分为三个层级之一：官方、社区或第三方。Claude Code 仅接受来自 `github.com/anthropics/` 存储库的官方和社区名称用于市场，所以第三方市场不能将自己呈现为 Anthropic 的。同事或您的组织发布的市场是第三方的。

该表列出了每个层级中的市场名称：

| 层级 | 哪些市场 |
| :- | :- |
| 官方 | [官方市场名称](#official-marketplace-names)，例如 `claude-plugins-official` |
| 社区 | `claude-community`、`claude-plugins-community` 和 `healthcare` |
| 第三方 | 所有其他市场 |

当 `claude-community` 目录将插件固定到提交 SHA 时（几乎每个条目都这样做），Claude Code 拒绝安装不同的提交。

### 官方市场名称

这些市场名称构成官方层级：

* `claude-plugins-official`
* `claude-code-marketplace`
* `claude-code-plugins`
* `anthropic-marketplace`
* `anthropic-plugins`
* `agent-skills`
* `anthropic-agent-skills`
* `life-sciences`
* `knowledge-work-plugins`
* `claude-for-legal`
* `claude-for-financial-services`
* `financial-services-plugins`
* `first-party-plugins`
* `claude-tag-plugins`

有关官方市场和社区市场的区别以及在哪里浏览每个市场列出的内容，请参阅 [Anthropic 的市场](https://code.claude.com/docs/zh-CN/plugins/anthropic-marketplaces.md)。

## 在安装前审查插件

在安装插件之前，查看它添加的内容以及它来自哪里。

**1. 检查市场的来源**

在您的 shell 中，运行 `claude plugin marketplace list` 以打印每个市场添加来源的信息，例如 GitHub 存储库或目录。

**2. 阅读详情窗格**

在 Claude Code 会话中，运行 `/plugin` 并选择插件。详情窗格显示一个**将安装**部分，列出插件的 commands、agents、skills、hooks 和 MCP 及 LSP 服务器。对于 Anthropic 没有发布组件数据的插件，该部分显示市场条目声明的内容，或一个注释：`Components will be discovered at installation` 用于存储在市场内的插件，或 `Component summary not available for remote plugin` 用于从其他地方获取的插件。

**3. 阅读插件的源代码**

在详情窗格中，选择安装选项下方的**打开主页**或**在 GitHub 上查看**。如果窗格都不提供，请打开您在第一步中找到的市场存储库。在那里找到插件的目录。**将安装**部分显示 hook 存在但不显示它运行的内容，所以请阅读插件目录中的这些文件：

* **`hooks/hooks.json`**：每个 hook 运行的命令
* **`.mcp.json`**：每个服务器的命令或 URL
* **`bin/`**：目录中的每个文件

**4. 列出插件包含的内容**

克隆保存插件目录的存储库，然后在您的 shell 中运行 `claude --plugin-dir <plugin directory> plugin details <plugin name>` 以查看 Claude Code 在其中找到的内容。该命令读取插件的文件而不启动会话，并打印一个 `Component inventory` 列出插件的 skills 和 commands、agents、带有每个 hook 事件的 hooks，以及 MCP 和 LSP 服务器。

安装插件后，在您的 shell 中运行 `claude plugin details <plugin name>` 以为 `~/.claude/plugins/cache/<marketplace>/<plugin>/<version>/` 下的已安装副本打印相同的 `Component inventory`。

### 删除您不再信任的插件

在您的 shell 中，使用您安装它的 `--scope` 运行 [`claude plugin uninstall <plugin>`](https://code.claude.com/docs/zh-CN/plugins/cli-reference#plugin-uninstall)。然后检查卸载删除了什么以及留下了什么：

* **持久数据**：默认情况下，当这是插件安装的最后一个范围时，卸载也会删除插件的持久数据目录。对于 `--keep-data` 和其他保留数据的情况，请参阅 [plugin uninstall](https://code.claude.com/docs/zh-CN/plugins/cli-reference.md#plugin-uninstall)。
* **缓存文件**：插件的文件在 `~/.claude/plugins/cache/` 下保留在磁盘上 14 天，然后[后台扫描将其删除](https://code.claude.com/docs/zh-CN/plugins/loading.md#cleanup-of-previous-versions)。卸载最后一个插件后，孤立目录保留到您安装另一个。要立即删除文件，请自己删除 `~/.claude/plugins/cache/<marketplace>/<plugin>/` 下的插件目录。
* **市场**：如果您也不信任市场的所有者，[也删除市场](https://code.claude.com/docs/zh-CN/plugins/install.md#manage-marketplaces)，这会卸载您从它安装的每个插件。

## 识别 Claude Code 何时拒绝或警告

您从 `/plugin` 中的**发现**或**市场**选项卡打开的详情窗格为每个插件显示相同的信任警告。Claude Code 在[不受信任的市场来源和失败的完整性检查](#untrusted-marketplace-sources-and-failed-integrity-checks)下的情况下拒绝而不是警告。

### 安装前的信任警告

警告读起来与插件来自的市场相同：

```text theme={null}
Make sure you trust a plugin before installing, updating, or using it. Anthropic does not control what MCP servers, files, or other software are included in plugins and cannot verify that they will work as intended or that they won't change. See each plugin's homepage for more information.
```

如果您的组织在[托管设置](https://code.claude.com/docs/zh-CN/plugins/org.md)中设置了 `pluginTrustMessage`，Claude Code 会将该文本附加到警告中。

### 不受信任的市场来源和失败的完整性检查

Claude Code 在这些情况下拒绝加载市场或安装插件，每种情况都有自己的错误消息：

* **不受信任的市场来源**：当市场使用官方或社区名称但其来源在 `github.com/anthropics/` 之外时，Claude Code 停止加载市场和您从它安装的插件。错误是[市场从不受信任的来源注册](https://code.claude.com/docs/zh-CN/errors.md#marketplace-is-registered-from-an-untrusted-source)。
* **存档完整性**：当市场条目将 [`archive` 源](https://code.claude.com/docs/zh-CN/plugins/marketplace-reference#archive-plugin-source)固定到 `sha256` 摘要，并且下载的文件的摘要与其不匹配时，Claude Code 拒绝安装。错误是[插件存档完整性检查失败](https://code.claude.com/docs/zh-CN/errors.md#plugin-archive-integrity-check-failed)。

`sha256` 固定与社区目录的提交 SHA 固定分开，后者选择要检出的 git 提交。

## 为您的组织强制执行插件控制

使用[托管设置](https://code.claude.com/docs/zh-CN/plugins/org.md)，管理员可以强制执行这些插件控制：

* 允许列表或阻止列表市场来源
* 强制启用插件
* 关闭 `--plugin-dir` 和 `--plugin-url` 标志以及 `CLAUDE_CODE_PLUGIN_DIRS` 变量
* 将 hooks 限制为来自托管设置和强制启用插件的 hooks
* 停止成员 claude.ai 账户中的插件在 Claude Code 中加载，使用 [`syncClaudeAiPlugins`](https://code.claude.com/docs/zh-CN/plugins/org#control-matrix)

[控制矩阵](https://code.claude.com/docs/zh-CN/plugins/org.md#control-matrix)说明每个键的作用和不涵盖的内容。

## 在遥测中查找插件

如果您的组织将 Claude Code 的 [OpenTelemetry 事件](https://code.claude.com/docs/zh-CN/monitoring-usage.md)导出到其自己的后端，[市场层级](#marketplace-tiers)决定哪些插件名称出现在那里：

* **[插件加载事件](https://code.claude.com/docs/zh-CN/monitoring-usage.md#plugin-loaded-event)**：事件按原样报告官方层级插件和市场名称。对于社区和第三方层级，`plugin.name` 和 `marketplace.name` 是字面字符串 `third-party`，除非您设置 `OTEL_LOG_TOOL_DETAILS=1`。
* **插件范围**：加载事件的 `plugin.scope` 仍然报告插件来自的位置，例如 `org` 用于您的托管设置启用的插件或 `user-local` 用于任何其他第三方插件。[插件加载事件](https://code.claude.com/docs/zh-CN/monitoring-usage.md#plugin-loaded-event)列出每个值。
* **[插件安装事件](https://code.claude.com/docs/zh-CN/monitoring-usage.md#plugin-installed-event)**：除非您设置 `OTEL_LOG_TOOL_DETAILS=1`，否则事件省略非官方插件的名称字段，而不是报告 `third-party`。
* **[Claude Code Analytics API](https://platform.claude.com/docs/en/api/admin/analytics/plugins/list)**：Claude Code 按名称报告来自官方和社区层级的插件，并将所有其他插件报告为 `third-party`。

## 后续步骤

* [为您的组织管理插件](https://code.claude.com/docs/zh-CN/plugins/org.md)：限制用户可以从哪些市场安装，并要求您信任的市场
* [安装和管理插件](https://code.claude.com/docs/zh-CN/plugins/install.md)：在选择范围之前审查插件的详情窗格
* [Anthropic 的市场](https://code.claude.com/docs/zh-CN/plugins/anthropic-marketplaces.md)：哪些市场名称是 Anthropic 的
* [安全](https://code.claude.com/docs/zh-CN/security.md)：Claude Code 自己的安全模型
