> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# 功能可用性

> 比较 Claude Code 功能在 Anthropic 订阅计划、Anthropic Console、Amazon Bedrock、AWS 上的 Claude Platform、Google Cloud 的 Agent Platform 和 Microsoft Foundry 中的可用性。

Claude Code CLI 和所有本地运行的功能在每个提供商上的工作方式完全相同。有关每个提供商的设置说明，请参阅[企业部署概述](https://code.claude.com/docs/zh-CN/third-party-integrations.md)。要直接跳到您的提供商上缺少的功能，请参阅[按提供商汇总](#summary-by-provider)选项卡。

在下表中，✓ 表示可用，✗ 表示不可用，"See note"链接到脚注以获取部分支持。✓ 后面的限定符将可用性缩小到该子集，"Admin-enabled"表示该功能处于关闭状态，直到组织管理员将其打开。

## 按模型提供商的可用性

您的身份验证方式决定了 Claude Code 可以访问哪些功能。有关您的提供商上缺少的内容的单一列表，请参阅[按提供商汇总](#summary-by-provider)选项卡。要在表格中找到您的列：

* **Claude 订阅**：您使用 claude.ai 账户登录 Pro、Max、Ultra、Team 或 Enterprise 套餐
* **Anthropic Console**：您使用 Anthropic API 密钥进行身份验证，或者[在没有 API 密钥的情况下登录 Console 账户](https://code.claude.com/docs/zh-CN/authentication.md#sign-in-without-an-api-key)
* **Amazon Bedrock**：您从 Amazon Bedrock 模型目录中使用 Claude 模型并设置 `CLAUDE_CODE_USE_BEDROCK`。[Mantle 端点](https://code.claude.com/docs/zh-CN/amazon-bedrock.md#use-the-mantle-endpoint)（`CLAUDE_CODE_USE_MANTLE`）由此列涵盖
* **Claude Platform on AWS**：您通过 AWS Marketplace 购买了 Claude，但调用 Anthropic API，并设置 `CLAUDE_CODE_USE_ANTHROPIC_AWS`
* **Google Cloud's Agent Platform**：由 Google 运营；您设置 `CLAUDE_CODE_USE_VERTEX`
* **Microsoft Foundry**：由 Anthropic 运营；您设置 `CLAUDE_CODE_USE_FOUNDRY`

### 每个提供商都可用的功能

这些在每个提供商上都有效：

* [CLI](https://code.claude.com/docs/zh-CN/quickstart.md) 和 [Agent SDK](https://code.claude.com/docs/zh-CN/agent-sdk/overview.md)
* [VS Code](https://code.claude.com/docs/zh-CN/vs-code.md) 和 [JetBrains](https://code.claude.com/docs/zh-CN/jetbrains.md) 扩展
* [Subagents](https://code.claude.com/docs/zh-CN/sub-agents.md)、[hooks](https://code.claude.com/docs/zh-CN/hooks-guide.md)、[commands](https://code.claude.com/docs/zh-CN/commands.md) 和 [skills](https://code.claude.com/docs/zh-CN/skills.md)
* [CLAUDE.md memory](https://code.claude.com/docs/zh-CN/memory.md)、[plugins](https://code.claude.com/docs/zh-CN/plugins/overview.md) 和 [MCP servers](https://code.claude.com/docs/zh-CN/mcp.md)
* [Checkpoints](https://code.claude.com/docs/zh-CN/checkpointing.md)、[sandboxing](https://code.claude.com/docs/zh-CN/sandboxing.md) 和 [Workflows](https://code.claude.com/docs/zh-CN/workflows.md)
* [OpenTelemetry metrics](https://code.claude.com/docs/zh-CN/monitoring-usage.md) 和[托管设置文件](https://code.claude.com/docs/zh-CN/managed-settings.md#delivery-mechanisms)

这些有提供商特定的差异：

* **MCP 服务器**：[来自 claude.ai 的连接器](https://code.claude.com/docs/zh-CN/mcp.md#use-mcp-servers-from-claude-ai)仅在您的 claude.ai 订阅是活跃身份验证方法时加载。[工具搜索](https://code.claude.com/docs/zh-CN/mcp.md#configure-tool-search)在 Microsoft Foundry [部署在 Azure 上](https://platform.claude.com/docs/en/build-with-claude/claude-in-microsoft-foundry#hosting-options)时不受支持。在 Google Cloud's Agent Platform 上，早于 Claude Opus 4.5、Claude Sonnet 4.5 和 Claude Haiku 4.5 的模型不支持工具搜索
* **Subagents**：当主对话运行 Fable 时，如果使用 Claude 订阅、Anthropic Console 账户或通过 `ANTHROPIC_BASE_URL` 访问的 [LLM gateway](https://code.claude.com/docs/zh-CN/llm-gateway)，内置的 [Explore subagent](https://code.claude.com/docs/zh-CN/sub-agents.md#built-in-subagents) 会在 Opus 上运行。在其他提供商（包括 Claude Platform on AWS）上，它在 Fable 上运行
* **[Commands](https://code.claude.com/docs/zh-CN/commands.md#all-commands)**：
  * `/design-sync` 和 `/import` 及其 `claude import` 子命令形式在 Amazon Bedrock、Google Cloud's Agent Platform、Microsoft Foundry 和 Claude Platform on AWS 上不可用，以及通过 [Claude apps gateway](https://code.claude.com/docs/zh-CN/claude-apps-gateway#availability-and-limitations)
  * `/voice` 需要 claude.ai 账户
  * `/list-agents` 及其别名 `/peers` 仅在[启用了跨会话消息传递](https://code.claude.com/docs/zh-CN/cross-session-messaging.md#availability)的会话中可用

### 需要 Claude 订阅的功能

这些需要使用 claude.ai 账户登录，无法通过 Anthropic Console API 密钥或第三方提供商访问：

* [云端会话](https://code.claude.com/docs/zh-CN/claude-code-on-the-web.md)和移动设备上的 Claude Code
* [Slack 中的 Claude Code](https://code.claude.com/docs/zh-CN/slack.md)：Pro、Max 和 Ultra 套餐
* [Claude Code Desktop](https://code.claude.com/docs/zh-CN/desktop.md)
* [云端定时任务](https://code.claude.com/docs/zh-CN/routines.md)（`/schedule`）
* [Ultrareview](https://code.claude.com/docs/zh-CN/ultrareview.md)
* [Code Review](https://code.claude.com/docs/zh-CN/code-review.md)：Team 和 Enterprise 计划
* [Remote Control](https://code.claude.com/docs/zh-CN/remote-control.md)
* [Chrome 扩展](https://code.claude.com/docs/zh-CN/chrome.md)
* [Computer use](https://code.claude.com/docs/zh-CN/computer-use.md)：Pro、Max 和 Ultra 套餐
* [Artifacts](https://code.claude.com/docs/zh-CN/artifacts.md)：Pro、Max、Ultra、Team 和 Enterprise 套餐
* [Voice dictation](https://code.claude.com/docs/zh-CN/voice-dictation.md)

Desktop 是部分例外：[网关路由可以在应用中或由管理员配置](https://code.claude.com/docs/zh-CN/llm-gateway-connect.md#desktop-app)。[Claude Desktop on 3P](https://claude.com/docs/third-party/claude-desktop/overview) 还可以在 Amazon Bedrock、Google Cloud's Agent Platform、Microsoft Foundry 或自托管 LLM 网关上运行 Code 选项卡。它还可以使用 Anthropic Console API 密钥在 [Claude API](https://claude.com/docs/third-party/claude-desktop/claude-api) 上运行 Code 选项卡。有关这些功能的按计划可用性，请参阅[按订阅计划的可用性](#availability-by-subscription-plan)。

### 按提供商变化的 CLI 功能

这些功能在本地 CLI 中工作，但取决于并非每个提供商都公开的服务器端功能。

<table>
  <thead>
    <tr>
      <th>功能</th>
      <th>Claude 订阅</th>
      <th>Anthropic Console</th>
      <th>Amazon Bedrock</th>
      <th>Claude Platform on AWS</th>
      <th>Google Cloud's Agent Platform</th>
      <th>Microsoft Foundry</th>
    </tr>
  </thead>

  <tbody>
    <tr>
      <td>[Web search](https://code.claude.com/docs/zh-CN/tools-reference.md#websearch-tool-behavior)</td>
      <td>✓</td>
      <td>✓</td>
      <td>✗</td>
      <td>✓</td>
      <td>参见注释 <sup>[1](#fn1)</sup></td>
      <td>✓</td>
    </tr>

    <tr>
      <td>[Fast mode](https://code.claude.com/docs/zh-CN/fast-mode.md)</td>
      <td>✓（[所有者启用](https://code.claude.com/docs/zh-CN/fast-mode.md#enable-fast-mode-for-your-organization) Team 和 Enterprise）</td>
      <td>✓（Opus 5 和 4.8 需要预配置）</td>
      <td>✗</td>
      <td>✗</td>
      <td>✗</td>
      <td>✗</td>
    </tr>

    <tr>
      <td>[Auto mode](https://code.claude.com/docs/zh-CN/auto-mode-config.md)</td>
      <td>✓</td>
      <td>✓</td>
      <td>参见注释 <sup>[2](#fn2)</sup></td>
      <td>✓</td>
      <td>参见注释 <sup>[2](#fn2)</sup></td>
      <td>参见注释 <sup>[2](#fn2)</sup></td>
    </tr>

    <tr>
      <td>[Advisor](https://code.claude.com/docs/zh-CN/advisor.md)</td>
      <td>✓</td>
      <td>✓</td>
      <td>✗</td>
      <td>✗</td>
      <td>✗</td>
      <td>✗</td>
    </tr>

    <tr>
      <td>[Cross-session messaging](https://code.claude.com/docs/zh-CN/cross-session-messaging.md)</td>
      <td>✓ <sup>[5](#fn5)</sup></td>
      <td>✓（同一台机器）<sup>[5](#fn5)</sup></td>
      <td>✓（同一台机器）<sup>[5](#fn5)</sup></td>
      <td>✓（同一台机器）<sup>[5](#fn5)</sup></td>
      <td>✓（同一台机器）<sup>[5](#fn5)</sup></td>
      <td>✓（同一台机器）<sup>[5](#fn5)</sup></td>
    </tr>

    <tr>
      <td>[Channels](https://code.claude.com/docs/zh-CN/channels.md)</td>
      <td>✓</td>
      <td>✓</td>
      <td>✗</td>
      <td>✗</td>
      <td>✗</td>
      <td>✗</td>
    </tr>

    <tr>
      <td>[GitHub Actions](https://code.claude.com/docs/zh-CN/github-actions.md)</td>
      <td>✓</td>
      <td>✓</td>
      <td>✓</td>
      <td>✗</td>
      <td>✓</td>
      <td>✓</td>
    </tr>

    <tr>
      <td>[GitLab CI/CD](https://code.claude.com/docs/zh-CN/gitlab-ci-cd.md)</td>
      <td>✓</td>
      <td>✓</td>
      <td>✓</td>
      <td>✓</td>
      <td>✓</td>
      <td>✗</td>
    </tr>
  </tbody>
</table>

### 管理和分析

组织级别的控制和使用情况可见性。

<table>
  <thead>
    <tr>
      <th>功能</th>
      <th>Claude 订阅</th>
      <th>Anthropic Console</th>
      <th>Amazon Bedrock</th>
      <th>Claude Platform on AWS</th>
      <th>Google Cloud's Agent Platform</th>
      <th>Microsoft Foundry</th>
    </tr>
  </thead>

  <tbody>
    <tr>
      <td>[Analytics dashboard and API](https://code.claude.com/docs/zh-CN/analytics.md)</td>
      <td>✓（仪表板：Team 和 Enterprise；API：Enterprise）</td>
      <td>✓ <sup>[4](#fn4)</sup></td>
      <td>✗</td>
      <td>✗</td>
      <td>✗</td>
      <td>✗</td>
    </tr>

    <tr>
      <td>[Server-managed settings](https://code.claude.com/docs/zh-CN/server-managed-settings.md)</td>
      <td>✓（Team 和 Enterprise）</td>
      <td>参见[平台可用性](https://code.claude.com/docs/zh-CN/server-managed-settings.md#platform-availability)</td>
      <td>✗</td>
      <td>✗</td>
      <td>✗</td>
      <td>✗</td>
    </tr>

    <tr>
      <td>[Zero Data Retention](https://code.claude.com/docs/zh-CN/zero-data-retention.md)</td>
      <td>✓（合格的 Enterprise 账户）</td>
      <td>✓（合格的账户）</td>
      <td>参见注释 <sup>[3](#fn3)</sup></td>
      <td>✓（合格的账户）</td>
      <td>参见注释 <sup>[3](#fn3)</sup></td>
      <td>参见注释 <sup>[3](#fn3)</sup></td>
    </tr>
  </tbody>
</table>

<sup>1</sup> 在 Google Cloud's Agent Platform 上，web search 适用于 Opus 4、Sonnet 4、Haiku 4.5 及更高版本的模型。\

<sup>2</sup> 在这些提供商上，自动模式仅支持 Claude Sonnet 5 或更高版本、Opus 4.7 或更高版本、Haiku 5.5 以及 Fable 模型。请参阅 [Auto mode 配置](https://code.claude.com/docs/zh-CN/auto-mode-config.md)。有关会话在这些提供商上启动时所处的权限模式，请参阅[会话启动时的模式](https://code.claude.com/docs/zh-CN/permission-modes.md#which-mode-a-session-starts-in)。\

<sup>3</sup> 受您与云提供商的协议约束。\

<sup>4</sup> 仅限仪表板和 API。[贡献指标](https://code.claude.com/docs/zh-CN/analytics.md#enable-contribution-metrics)需要 claude.ai Team 或 Enterprise 组织。\

<sup>5</sup> 在 macOS 和 Linux 上需要 Claude Code v2.1.224 或更高版本，包括 WSL 2 内的 Linux。在原生 Windows 上，需要 Claude Code v2.1.234 或更高版本。使用 API 密钥身份验证时，消息传递仅限同一台机器。在 Amazon Bedrock、Claude Platform on AWS、Google Cloud's Agent Platform 和 Microsoft Foundry 上，消息传递仅限同一台机器，需要 Claude Code v2.1.248 或更高版本。Claude 只能从连接到 [Remote Control](https://code.claude.com/docs/zh-CN/remote-control.md) 的会话中找到您的[云端会话](https://code.claude.com/docs/zh-CN/claude-code-on-the-web.md)和其他机器上的会话。要连接，您需要 claude.ai 登录和其他 [Remote Control 要求](https://code.claude.com/docs/zh-CN/remote-control.md#requirements)。请参阅[在其他机器上发送消息](https://code.claude.com/docs/zh-CN/cross-session-messaging.md#message-sessions-on-other-machines)。

> [!NOTE]
>
> 如果您通过 [LLM gateway](https://code.claude.com/docs/zh-CN/llm-gateway) 进行身份验证，功能可用性与网关转发到的基础提供商相匹配，除了 Claude Code 本身关闭的功能。每当 `ANTHROPIC_BASE_URL` 指向 `api.anthropic.com` 以外的主机时，Claude Code 会关闭功能，例如 [Remote Control](https://code.claude.com/docs/zh-CN/remote-control.md#requirements) 和 [server-managed settings](https://code.claude.com/docs/zh-CN/server-managed-settings.md#platform-availability)，无论网关转发什么。某些仅限 Anthropic 的功能，例如 [Advisor](https://code.claude.com/docs/zh-CN/advisor.md)，仅在网关将请求完整转发到 Anthropic API 时才有效。
>
> 有关 Claude Code 发送的请求在 Amazon Bedrock 或 Agent Platform 格式网关、`ANTHROPIC_BASE_URL` 网关和 Claude apps gateway 登录之间如何不同，请参阅[按连接方法的客户端行为](https://code.claude.com/docs/zh-CN/llm-gateway-protocol.md#how-the-connection-method-changes-client-behavior)。

### 按提供商汇总

每个选项卡列出了该提供商上不可用或部分支持的内容，以及存在替代方案的地方。未列出的所有内容的工作方式与 Claude 订阅上相同，除了上面注明的[提供商特定的差异](#features-available-on-every-provider)。在 Amazon Bedrock、Google Cloud's Agent Platform、Microsoft Foundry 和 Claude Platform on AWS 上，错误报告和遥测到 Anthropic 默认处于关闭状态。请参阅[按 API 提供商的默认行为](https://code.claude.com/docs/zh-CN/data-usage.md#default-behaviors-by-api-provider)了解哪些流量仍然到达 Anthropic 以及如何选择退出。

> [tab] Amazon Bedrock
>
> **不可用：** 所有[需要 Claude 订阅的功能](#features-that-require-a-claude-subscription)，加上 [web search](https://code.claude.com/docs/zh-CN/tools-reference.md#websearch-tool-behavior)、[fast mode](https://code.claude.com/docs/zh-CN/fast-mode.md)、[Advisor](https://code.claude.com/docs/zh-CN/advisor.md)、[Channels](https://code.claude.com/docs/zh-CN/channels.md)、[analytics dashboard](https://code.claude.com/docs/zh-CN/analytics.md)、[server-managed settings](https://code.claude.com/docs/zh-CN/server-managed-settings.md) 和 [`/design-sync` 和 `/import` 命令](https://code.claude.com/docs/zh-CN/commands#all-commands)。
>
> **部分支持：**
>
> * [Desktop](https://code.claude.com/docs/zh-CN/desktop.md)：仅通过 [Claude Desktop on 3P](https://claude.com/docs/third-party/claude-desktop/overview)
> * [Auto mode](https://code.claude.com/docs/zh-CN/auto-mode-config.md)：仅 Sonnet 5 或更高版本、Opus 4.7 或更高版本、Haiku 5.5 以及 Fable 模型
> * [Cross-session messaging](https://code.claude.com/docs/zh-CN/cross-session-messaging.md)：仅在此机器上的您的会话之间 <sup>[5](#fn5)</sup>
> * [Zero Data Retention](https://code.claude.com/docs/zh-CN/zero-data-retention.md)：受您的 AWS 协议约束
>
> **替代方案：** 对于调度，使用 [`/loop`](https://code.claude.com/docs/zh-CN/scheduled-tasks) 而不是 `/schedule`。对于云会话，使用 [GitHub Actions](https://code.claude.com/docs/zh-CN/github-actions.md) 或 [GitLab CI/CD](https://code.claude.com/docs/zh-CN/gitlab-ci-cd.md)。对于网络查询，使用带有特定 URL 的 [WebFetch tool](https://code.claude.com/docs/zh-CN/tools-reference.md#webfetch-tool-behavior)。

> [tab] Claude Platform on AWS
>
> **不可用：** 所有[需要 Claude 订阅的功能](#features-that-require-a-claude-subscription)，加上 [fast mode](https://code.claude.com/docs/zh-CN/fast-mode.md)、[Advisor](https://code.claude.com/docs/zh-CN/advisor.md)、[Channels](https://code.claude.com/docs/zh-CN/channels.md)、[GitHub Actions](https://code.claude.com/docs/zh-CN/github-actions.md)、[analytics dashboard](https://code.claude.com/docs/zh-CN/analytics.md)、[server-managed settings](https://code.claude.com/docs/zh-CN/server-managed-settings.md) 和 [`/design-sync` 和 `/import` 命令](https://code.claude.com/docs/zh-CN/commands#all-commands)。
>
> **Amazon Bedrock 不可用的地方可用：** [web search](https://code.claude.com/docs/zh-CN/tools-reference.md#websearch-tool-behavior)。
>
> **部分支持：**
>
> * [Cross-session messaging](https://code.claude.com/docs/zh-CN/cross-session-messaging.md)：仅在此机器上的您的会话之间 <sup>[5](#fn5)</sup>
>
> **替代方案：** 对于调度，使用 [`/loop`](https://code.claude.com/docs/zh-CN/scheduled-tasks) 而不是 `/schedule`。对于云会话，使用 [GitLab CI/CD](https://code.claude.com/docs/zh-CN/gitlab-ci-cd.md)。

> [tab] Google Cloud's Agent Platform
>
> **不可用：** 所有[需要 Claude 订阅的功能](#features-that-require-a-claude-subscription)，加上 [fast mode](https://code.claude.com/docs/zh-CN/fast-mode.md)、[Advisor](https://code.claude.com/docs/zh-CN/advisor.md)、[Channels](https://code.claude.com/docs/zh-CN/channels.md)、[analytics dashboard](https://code.claude.com/docs/zh-CN/analytics.md)、[server-managed settings](https://code.claude.com/docs/zh-CN/server-managed-settings.md) 和 [`/design-sync` 和 `/import` 命令](https://code.claude.com/docs/zh-CN/commands#all-commands)。
>
> **部分支持：**
>
> * [Desktop](https://code.claude.com/docs/zh-CN/desktop.md)：通过[托管设置](https://claude.com/docs/third-party/claude-desktop/configuration)或 [Claude Desktop on 3P](https://claude.com/docs/third-party/claude-desktop/overview)
> * [Web search](https://code.claude.com/docs/zh-CN/tools-reference.md#websearch-tool-behavior)：Opus 4、Sonnet 4、Haiku 4.5 及更高版本的模型
> * [Auto mode](https://code.claude.com/docs/zh-CN/auto-mode-config.md)：仅 Sonnet 5 或更高版本、Opus 4.7 或更高版本、Haiku 5.5 以及 Fable 模型
> * [Cross-session messaging](https://code.claude.com/docs/zh-CN/cross-session-messaging.md)：仅在此机器上的您的会话之间 <sup>[5](#fn5)</sup>
> * [Zero Data Retention](https://code.claude.com/docs/zh-CN/zero-data-retention.md)：受您的 Google Cloud 协议约束
>
> **替代方案：** 对于调度，使用 [`/loop`](https://code.claude.com/docs/zh-CN/scheduled-tasks) 而不是 `/schedule`。对于云会话，使用 [GitHub Actions](https://code.claude.com/docs/zh-CN/github-actions.md) 或 [GitLab CI/CD](https://code.claude.com/docs/zh-CN/gitlab-ci-cd.md)。

> [tab] Microsoft Foundry
>
> **不可用：** 所有[需要 Claude 订阅的功能](#features-that-require-a-claude-subscription)，加上 [fast mode](https://code.claude.com/docs/zh-CN/fast-mode.md)、[Advisor](https://code.claude.com/docs/zh-CN/advisor.md)、[Channels](https://code.claude.com/docs/zh-CN/channels.md)、[GitLab CI/CD](https://code.claude.com/docs/zh-CN/gitlab-ci-cd.md)、[analytics dashboard](https://code.claude.com/docs/zh-CN/analytics.md)、[server-managed settings](https://code.claude.com/docs/zh-CN/server-managed-settings.md) 和 [`/design-sync` 和 `/import` 命令](https://code.claude.com/docs/zh-CN/commands#all-commands)。
>
> **部分支持：**
>
> * [Desktop](https://code.claude.com/docs/zh-CN/desktop.md)：仅通过 [Claude Desktop on 3P](https://claude.com/docs/third-party/claude-desktop/overview)
> * [Auto mode](https://code.claude.com/docs/zh-CN/auto-mode-config.md)：仅 Sonnet 5 或更高版本、Opus 4.7 或更高版本、Haiku 5.5 以及 Fable 模型
> * [Cross-session messaging](https://code.claude.com/docs/zh-CN/cross-session-messaging.md)：仅在此机器上的您的会话之间 <sup>[5](#fn5)</sup>
> * [Zero Data Retention](https://code.claude.com/docs/zh-CN/zero-data-retention.md)：受您的 Azure 协议约束
>
> **替代方案：** 对于调度，使用 [`/loop`](https://code.claude.com/docs/zh-CN/scheduled-tasks) 而不是 `/schedule`。对于云会话，使用 [GitHub Actions](https://code.claude.com/docs/zh-CN/github-actions.md)。

> [tab] Anthropic Console
>
> **不可用：** 所有[需要 Claude 订阅的功能](#features-that-require-a-claude-subscription)。
>
> [按提供商变化的 CLI 功能](#cli-capabilities-that-vary-by-provider)中的所有内容都可用，除了使用 Claude Opus 5 或 Claude Opus 4.8 的 [fast mode](https://code.claude.com/docs/zh-CN/fast-mode.md) 需要[预配置访问](https://code.claude.com/docs/zh-CN/fast-mode.md#enable-fast-mode-for-your-organization)。在 claude.ai Team 或 Enterprise 组织中配置的 [Server-managed settings](https://code.claude.com/docs/zh-CN/server-managed-settings.md) 不会作用于使用 Console API 密钥进行身份验证的会话。有关如何覆盖这些会话，请参阅[平台可用性](https://code.claude.com/docs/zh-CN/server-managed-settings.md#platform-availability)。
>
> **部分支持：**
>
> * [Desktop](https://code.claude.com/docs/zh-CN/desktop.md)：通过[结合 Claude API 使用的 Claude Desktop on 3P](https://claude.com/docs/third-party/claude-desktop/claude-api)

## 按订阅计划的可用性

如果您通过 Amazon Bedrock、Google Cloud 的 Agent Platform、Microsoft Foundry 或 Anthropic Console API 密钥进行身份验证，本部分不适用于您。当您使用 claude.ai 账户登录时，您的计划决定了以下哪些功能可用。

在已应用 [HIPAA 配置](https://code.claude.com/docs/zh-CN/hipaa-setup.md)的 Enterprise 组织中，此表中的部分功能会被关闭。

| 功能 | Pro | Max | Ultra | Team | Enterprise |
| :- | :- | :- | :- | :- | :- |
| [云端会话](https://code.claude.com/docs/zh-CN/claude-code-on-the-web.md) | ✓ | ✓ | ✓ | ✓ | ✓ <sup>[6](#fn6)</sup> |
| [云端定时任务](https://code.claude.com/docs/zh-CN/routines.md) | ✓ | ✓ | ✓ | ✓ | ✓ |
| [Remote Control](https://code.claude.com/docs/zh-CN/remote-control.md) | ✓ | ✓ | ✓ | Admin-enabled | Admin-enabled |
| [Channels](https://code.claude.com/docs/zh-CN/channels.md) | ✓ | ✓ | ✓ | Admin-enabled | Admin-enabled |
| [Computer use](https://code.claude.com/docs/zh-CN/computer-use.md) | ✓ | ✓ | ✓ | ✗ | ✗ |
| Dispatch ([Desktop](https://code.claude.com/docs/zh-CN/desktop.md#sessions-from-dispatch)) | ✓ | ✓ | ✓ | ✗ | ✗ |
| [Code Review](https://code.claude.com/docs/zh-CN/code-review.md) | ✗ | ✗ | ✗ | ✓ | ✓ |
| [Artifacts](https://code.claude.com/docs/zh-CN/artifacts.md) | ✓ | ✓ | ✓ | ✓ | ✓ |
| [分析仪表板和贡献指标](https://code.claude.com/docs/zh-CN/analytics.md) | ✗ | ✗ | ✗ | ✓ | ✓ |
| [Enterprise Analytics API](https://code.claude.com/docs/zh-CN/analytics.md#access-data-programmatically) | ✗ | ✗ | ✗ | ✗ | ✓ |
| [Server-managed settings](https://code.claude.com/docs/zh-CN/server-managed-settings.md) | ✗ | ✗ | ✗ | ✓ | ✓ |
| [SSO](https://support.claude.com/en/articles/9266767-what-is-the-team-plan) | ✗ | ✗ | ✗ | ✓ | ✓ |
| SCIM | ✗ | ✗ | ✗ | ✗ | ✓ |
| [Compliance API](https://platform.claude.com/docs/en/api/compliance) | ✗ | ✗ | ✗ | ✗ | ✓ |
| [Zero Data Retention](https://code.claude.com/docs/zh-CN/zero-data-retention.md) | ✗ | ✗ | ✗ | ✗ | ✓ <sup>[7](#fn7)</sup> |

<sup>6</sup> 在 Enterprise 上，需要高级座位或 Chat + Claude Code 座位。请参阅[网络上的 Claude Code](https://code.claude.com/docs/zh-CN/claude-code-on-the-web.md)。\

<sup>7</sup> 不包含在标准 Enterprise 计划中。需要 Anthropic 为合格账户单独启用。请参阅 [Zero Data Retention](https://code.claude.com/docs/zh-CN/zero-data-retention.md)。

有关定价和完整的计划比较，请参阅 [Team 计划](https://support.claude.com/en/articles/9266767-what-is-the-team-plan)和 [Enterprise 计划](https://support.claude.com/en/articles/9797531-what-is-the-enterprise-plan)。

### Ultra 套餐

Ultra 是 Max 20x 套餐的新名称，Max 是 Max 5x 的新名称。要查看您当前使用的套餐，请在 claude.ai 上查看您的订阅。

在 Claude Code 中，Max 和 Ultra 具有相同的功能，详见[可用性表](#availability-by-subscription-plan)中的 Max 和 Ultra 列。有关用量限制和定价，请参阅 [Claude 定价页面](https://claude.com/pricing)。

## 模型可用性

有关每个提供商和地区可用的 Claude 模型和上下文窗口大小，请参阅[模型配置](https://code.claude.com/docs/zh-CN/model-config.md)和[模型概述](https://platform.claude.com/docs/en/about-claude/models/overview)。Vision、PDF 输入和扩展思考是模型功能而不是 Claude Code 功能，在提供该模型的每个提供商上都有效。[Prompt caching](https://code.claude.com/docs/zh-CN/prompt-caching.md) 在大多数提供商上的工作方式相同；在 Amazon Bedrock 上，支持因模型而异。

## 相关资源

* [企业部署概述](https://code.claude.com/docs/zh-CN/third-party-integrations.md)：比较提供商之间的身份验证、计费和地区
* 提供商设置指南：[Amazon Bedrock](https://code.claude.com/docs/zh-CN/amazon-bedrock.md)、[Claude Platform on AWS](https://code.claude.com/docs/zh-CN/claude-platform-on-aws.md)、[Google Cloud's Agent Platform](https://code.claude.com/docs/zh-CN/google-vertex-ai.md)、[Microsoft Foundry](https://code.claude.com/docs/zh-CN/microsoft-foundry.md)
* [平台和集成](https://code.claude.com/docs/zh-CN/platforms.md)：Claude Code 运行的地方，包括 CLI、Desktop、IDE 扩展、网络、移动和 CI/CD
