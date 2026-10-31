> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# 测量插件成本和使用情况

> 测量 Claude Code 插件的令牌成本，了解人们是否仍在使用它，并为组织范围的插件问题选择遥测事件。

启用插件的每个会话都会在 Claude 的上下文中包含其 skills、agents 和 commands 的名称和描述，这些令牌会计入用户的使用情况，无论插件是否被使用。本页面展示了如何查看插件的这个数字、如果你维护插件如何减少它，以及使用情况在哪里显示，以便你可以判断插件是否仍在被使用。

本页面适用于插件作者和维护者。如果你为组织管理 Claude Code，[跨机队测量](#measure-across-a-fleet)涵盖了每台机器上的相同问题。

> [!NOTE]
>
> 这些情况在其他页面上有介绍：
>
> * **测试插件改变 Claude 行为的可靠性**：请参阅[使用 evals 测试插件](https://code.claude.com/docs/zh-CN/plugin-evals.md)
> * **修剪你自己会话的上下文**：请参阅[管理已安装的插件](https://code.claude.com/docs/zh-CN/plugins/install.md#manage-installed-plugins)和[上下文窗口](https://code.claude.com/docs/zh-CN/context-window.md)页面

从[测量插件的成本](#measure-what-a-plugin-costs)开始。

## 测量插件的成本

要查看插件添加到 Claude 上下文的内容，请使用插件的名称运行[`claude plugin details`](https://code.claude.com/docs/zh-CN/plugins/cli-reference#plugin-details)。你在 shell 中运行它，而不是在运行的 Claude Code 会话的提示符处。插件必须被加载：已安装、在 skills 目录中，或通过同一命令中的 `--plugin-dir` 传递，如 `claude --plugin-dir ./formatter plugin details formatter`。

此示例读取一个名为 `formatter` 的已安装插件，该插件有两个 skills、一个 command、一个 agent、一个 hook 和一个 MCP 服务器：

```bash theme={null}
claude plugin details formatter
```

```text theme={null}
formatter 1.0.0
  Description: Formats and lints code on save
  Source: formatter@my-marketplace

Component inventory
  Skills (3)  format-all, format-code, lint-fix
  Agents (1)  style-reviewer
  Hooks (1)  PostToolUse  (harness-only — no model context cost)
  MCP servers (1)  formatter-tools  (tool schemas resolved at runtime; not counted)
  LSP servers (0)

Projected token cost
  Always-on:   ~146 tok   added to every session

Per-component (rounded)
  component       always-on  on-invoke
  format-code           ~40        ~30
  lint-fix              ~50        ~30
  style-reviewer        ~40        ~40
  format-all           < 20        ~30

  On-invoke cost is paid each time a skill or agent fires.
  Token counts are estimates and may differ from actual usage.
```

输出的每个部分回答了一个不同的问题：

* **Component inventory**：Claude Code 在插件中找到的内容。Commands 与 skills 一起计数，所以 `format-all` 出现在 `Skills` 下。Hooks 和 MCP 服务器没有成本估计和每个组件行；要查看插件的 MCP 工具添加的内容，请在启用插件的会话中运行 `/context`，并阅读 `MCP tools` 类别。
* **Always-on**：插件的 skills、agents 和 commands 的名称和描述添加到启用插件的每个会话中的令牌，无论是否有任何东西运行。这是每个用户携带的数字，也是要减少的数字。
* **Per-component**：每一行将一个 skill、agent 或 command 分成其 always-on 份额和其 on-invoke 成本，后者是仅在该组件运行时加载的主体。使用 always-on 列来找出哪个组件贡献最多。

### 降低 always-on 数字

如果你维护插件，这些更改会减少它添加到每个会话的内容。如果你只是使用它，你的选择是禁用或卸载它；请参阅[管理已安装的插件](https://code.claude.com/docs/zh-CN/plugins/install.md#manage-installed-plugins)。

always-on 数字计算每个组件的名称加上其 `description` 和 `when_to_use` frontmatter。要降低它：

* 缩短 skill 和 agent 描述。
* 分割大型插件，以便用户只安装他们需要的组件。

skill 的描述也是 Claude 匹配请求的内容，所以较短的描述可以阻止 skill 触发。修剪描述后，使用 eval 套件中的[`tool_used: Skill` grader](https://code.claude.com/docs/zh-CN/plugin-evals#create-your-first-eval-suite)检查触发。

有关每个组件类型的贡献，请参阅[插件组件](https://code.claude.com/docs/zh-CN/plugins/components.md)。

### 安装前向用户显示的成本

官方市场中的插件在安装前向用户显示其成本。在 `/plugin` 中，当用户浏览市场的插件列表并选择一个插件时，详细信息窗格显示一个**Context cost**部分，其中有一个 `Every turn:` 行和一个 `When invoked:` 行。当 always-on 数字为 2,000 个令牌或更多时，`Every turn:` 行显示为突出显示。

你自己市场中的插件没有**Context cost**部分。

## 检查插件是否被使用

Claude Code 不会向其作者报告插件的使用情况。使用情况记录在安装插件的每个人的机器上，所以你能学到的内容取决于你与这些人的关系：

* **你为他们的组织管理 Claude Code**：OpenTelemetry 事件和 Analytics API 计算每台机器上的安装和 skill 激活。请参阅[跨机队测量](#measure-across-a-fleet)。
* **他们是你可以询问的队友**：每个用户自己的 Claude Code 在四个地方向他们显示他们是否仍在使用插件：[`/plugin` 面板](#not-used-recently-in-/plugin)、[`/skill-doctor`](#find-skills-that-never-run)、[`/doctor`](#unused-plugins-in-/doctor)和[`/usage`](#usage-share-in-/usage)。所有四个都是用户在自己机器上的会话中在 Claude Code 提示符处运行的命令。
* **都不是**：你没有来自 Claude Code 的该插件的使用信号。

有关 Anthropic 目录中列出的插件的使用情况，请参阅 claude.com 上的[跟踪已发布的插件使用情况](https://claude.com/docs/connectors/building/after-publishing#track-published-plugin-usage)。

### `/plugin` 中最近未使用

在 `/plugin` 的**Installed**选项卡上，用户从市场安装的插件在至少 14 天和 10 个会话未使用后，会移到**Not used recently**标题下。插件的详细信息也显示一个 `Last used:` 行。有关用户对该标题和行的处理，请参阅[查找你不再使用的插件](https://code.claude.com/docs/zh-CN/plugins/install.md#find-plugins-you-no-longer-use)。

**Not used recently**标题永远不会出现在：

* 使用 `--plugin-dir` 加载或从 skills 目录加载的插件
* 通过托管设置启用或从[种子目录](https://code.claude.com/docs/zh-CN/plugins/org.md#seed-containers-and-ci)挂载的插件
* 包含主题、输出样式、监视器或工作流的插件，因为这些在没有跟踪调用的情况下使用

插件的[语言服务器](https://code.claude.com/docs/zh-CN/plugins/components.md#lsp-servers)在传递诊断或回答代码导航请求时计为已使用，所以一个 LSP 插件，其服务器在你的会话中处于活动状态，不会被列为未使用。

当用户的组织设置[`strictKnownMarketplaces`](https://code.claude.com/docs/zh-CN/plugins/org#restrict-what-users-can-install)时，标题和 `Last used:` 行都不会出现。

### 查找永远不运行的 skills

运行 `/skill-doctor` 以查看你的每个 skill 的成本以及它被使用的频率。它标记在 Claude 的 skill 列表中但从未被调用的 skills，包括来自插件的 skills。

在交互式会话中，报告在 `/plugin` 管理器的**Stats**选项卡中打开。请参阅[查找未使用的 skills](https://code.claude.com/docs/zh-CN/skills.md#find-unused-skills)了解报告涵盖的内容以及它在哪里可用。

### `/doctor` 中未使用的插件

`/doctor` 检查列出每个用户安装的 skill、MCP 服务器和插件，并建议禁用未使用的插件。请参阅[命令参考中的 `/doctor`](https://code.claude.com/docs/zh-CN/commands#all-commands)。

### `/usage` 中的使用情况份额

在 Pro、Max、Ultra、Team 或 Enterprise 计划上，`/usage` 分解将最近的使用情况归因于 skill、子代理、插件和 MCP 服务器，作为总数的份额。请参阅[使用 `/usage` 命令](https://code.claude.com/docs/zh-CN/costs#using-the-/usage-command)。

## 跨机队测量

如果你为组织管理 Claude Code，你可以从以下任一来源跨每台机器测量插件成本和使用情况：

* **OpenTelemetry 事件**：Claude Code 在你[配置导出器](https://code.claude.com/docs/zh-CN/monitoring-usage.md)后将这些导出到你自己的后端。请参阅[插件安装和使用的 OpenTelemetry 事件](#pick-the-opentelemetry-event-for-each-question)。
* **Analytics API**：来自 Anthropic 的记录，无需导出器。请参阅[查询 Analytics API](#query-the-analytics-api)。

### 插件安装和使用的 OpenTelemetry 事件

这些 OpenTelemetry 事件和属性从你的后端回答每个插件问题：

| 问题 | OpenTelemetry 事件或属性 |
| :- | :- |
| 安装了哪些插件，来自哪里 | [`claude_code.plugin_installed`](https://code.claude.com/docs/zh-CN/monitoring-usage#plugin-installed-event)，每次安装一个 |
| 哪些插件在多少个会话中处于活动状态 | [`claude_code.plugin_loaded`](https://code.claude.com/docs/zh-CN/monitoring-usage#plugin-loaded-event)，会话开始时每个启用的插件一个 |
| 哪些 skills 激活，哪个插件拥有它们 | [`claude_code.skill_activated`](https://code.claude.com/docs/zh-CN/monitoring-usage#skill-activated-event)，带有插件 skills 的 `plugin.name` 和 `marketplace.name` |
| 插件的 hooks 报告什么 | [`claude_code.hook_plugin_metrics`](https://code.claude.com/docs/zh-CN/monitoring-usage#hook-plugin-metrics-event)，仅为官方市场插件中的 hooks 发出 |
| 插件在 API 支出中的成本 | [成本计数器](https://code.claude.com/docs/zh-CN/monitoring-usage.md#cost-counter)上的 `plugin.name` 和 `marketplace.name`，在活跃 skill 或 subagent 属于插件时设置 |

### 后端中的编辑插件名称

来自官方市场的插件将其插件名称和市场名称逐字报告到你的后端。所有其他插件的名称默认被编辑或省略，包括来自你组织自己市场的插件。插件的[信任等级](https://code.claude.com/docs/zh-CN/plugins/security.md#find-plugins-in-telemetry)决定了哪个。

要在某些事件上获取真实名称，请在导出遥测的机器上将[`OTEL_LOG_TOOL_DETAILS`](https://code.claude.com/docs/zh-CN/monitoring-usage#common-configuration-variables)环境变量设置为 `1`，例如在配置导出器的同一[托管设置](https://code.claude.com/docs/zh-CN/monitoring-usage.md#administrator-configuration)的 `env` 块中：

| 事件 | 默认 | 使用 `OTEL_LOG_TOOL_DETAILS=1` |
| :- | :- | :- |
| `plugin_loaded` | `plugin.name` 和 `marketplace.name` 是字符串 `third-party` | 真实名称 |
| `plugin_installed`、`skill_activated` | `plugin.name` 和 `marketplace.name` 被省略；在 `skill_activated` 上，`skill.name` 是 `custom_skill` | 真实名称 |
| 成本计数器 | `plugin.name` 是 `third-party`；`marketplace.name` 不存在 | 真实 `plugin.name`；`marketplace.name` 仍然不存在 |

在 `plugin_loaded` 上，`plugin_id_hash` 仍然默认识别每个插件，所以你可以计算不同的第三方插件。

### 查询 Analytics API

在 Enterprise 计划上，Analytics API 从 Anthropic 的记录中回答"我的组织安装和调用哪些插件"，无需导出器。[`GET /v1/organizations/analytics/plugins`](https://platform.claude.com/docs/en/api/admin/analytics/plugins/list)返回跨 Claude Code 和 Cowork 的每个插件、每天的安装和调用计数，你可以按用户、RBAC 组或产品分组。

到达 Anthropic 而没有插件名称的插件活动出现在一个聚合 `third-party` 行中。[在遥测中查找插件](https://code.claude.com/docs/zh-CN/plugins/security.md#find-plugins-in-telemetry)说明 Claude Code 按名称报告的插件。

使用具有 `read:analytics` 范围的 API 密钥对请求进行身份验证，Primary Owner 按照[以编程方式访问数据](https://code.claude.com/docs/zh-CN/analytics.md#access-data-programmatically)中的描述创建。

有关参数和响应字段，请参阅[端点参考](https://platform.claude.com/docs/en/api/admin/analytics/plugins/list)。

## 后续步骤

* [使用 evals 测试插件](https://code.claude.com/docs/zh-CN/plugin-evals.md)：测量插件引导 Claude 的可靠性，而不仅仅是它的成本
* [降低 always-on 数字](#lower-the-always-on-figure)：在插件中更改什么以减少其每轮成本
* [插件安全和信任](https://code.claude.com/docs/zh-CN/plugins/security.md#find-plugins-in-telemetry)：哪些遥测字段携带插件名称以及何时被编辑
* [监视使用情况](https://code.claude.com/docs/zh-CN/monitoring-usage.md)：完整的 OpenTelemetry 事件参考
