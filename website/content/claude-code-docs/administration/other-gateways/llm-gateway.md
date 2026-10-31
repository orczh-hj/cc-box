> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# 其他 LLM 网关

> 通过您的组织已运行的 LLM 网关路由 Claude Code。涵盖将 Claude Code 连接到网关、为您的组织部署网关以及 Claude Code 发送到网关的内容。

本部分涵盖使用您的组织已运行的网关产品，而不是 [Claude apps gateway](https://code.claude.com/docs/zh-CN/claude-apps-gateway)。有关网关是什么、它如何位于 Claude Code 和您的提供商之间，以及如何在 Claude apps gateway 和其他产品之间进行选择，请参阅[网关概述](https://code.claude.com/docs/zh-CN/gateways.md)。

> [!NOTE]
>
> * 如果您是连接到现有网关的开发人员：[将 Claude Code 连接到您的网关](https://code.claude.com/docs/zh-CN/llm-gateway-connect.md)
> * 如果您是为组织部署网关的管理员：[部署和分发网关](https://code.claude.com/docs/zh-CN/llm-gateway-rollout.md)
> * 如果您正在配置网关产品：[网关兼容性指南](https://code.claude.com/docs/zh-CN/llm-gateway-protocol.md)

任何公开[支持的 API 格式](https://code.claude.com/docs/zh-CN/llm-gateway-protocol.md#api-formats)的网关都可以工作。Anthropic 不认可、维护或审计第三方网关产品，也不支持通过任何网关将 Claude Code 路由到非 Claude 模型。按照网关自己的文档部署网关，然后使用下面的[部署步骤](#roll-out-a-gateway)完成 Claude Code 端的部署。

## 网关提供的功能

网关为您的组织提供一个地方来管理：

* **凭证**：提供商密钥保留在服务器端；开发人员改为持有网关凭证
* **使用情况跟踪**：按开发人员或团队归属使用情况，无论哪个提供商处理请求
* **成本控制**：在一个地方强制执行预算和速率限制
* **审计日志**：记录每个模型请求以实现合规性
* **提供商切换**：在网关配置中更改提供商，无需接触开发人员机器

除了提供商切换外，所有这些都适用于上游是 Anthropic 的 API 还是[云提供商](https://code.claude.com/docs/zh-CN/third-party-integrations.md)。提供商切换而无需重新配置开发人员机器也取决于网关公开单个[Anthropic 格式端点](https://code.claude.com/docs/zh-CN/llm-gateway-protocol.md#api-formats)，无论上游如何；公开提供商自己格式的网关将客户端配置与该提供商绑定，并改变[Claude Code 发送的内容以及它应用的默认值](https://code.claude.com/docs/zh-CN/llm-gateway-protocol.md#how-the-connection-method-changes-client-behavior)。

权衡是网关成为您的组织运营的基础设施。Claude Code 在每个版本中添加功能，不转发这些功能的网关会破坏相应的功能，因此网关产品需要随着 Claude Code 的发展而保持更新。[网关兼容性指南](https://code.claude.com/docs/zh-CN/llm-gateway-protocol.md)涵盖要转发的内容。

## 部署网关

当您准备好为组织部署 LLM 网关时，无论您选择哪个网关产品，顺序都是相同的：

1. 部署网关并给予它您的提供商凭证，以便它可以验证它转发的请求。
2. 为每个开发人员颁发网关凭证，以便使用情况归属于开发人员，离职时撤销一个凭证。
3. 通过[托管设置文件](https://code.claude.com/docs/zh-CN/managed-settings.md#delivery-mechanisms)和您的机密工具分发配置，以便每台机器都接收基础 URL 和凭证。当两者都分发时，开发人员无需配置任何内容。如果您没有设置分发，开发人员按照[连接页面](https://code.claude.com/docs/zh-CN/llm-gateway-connect.md)自己设置变量。
4. 让每个开发人员[检查 Claude Code 中的配置](https://code.claude.com/docs/zh-CN/llm-gateway-connect.md#check-for-an-existing-configuration)，以便分发问题在他们依赖网关之前浮出水面。

[为您的组织部署 LLM 网关](https://code.claude.com/docs/zh-CN/llm-gateway-rollout.md)逐步讲解每个步骤，并显示在每个步骤中分发的配置文件。网关是组织设置的一部分；对于策略强制执行、使用情况可见性和数据处理决策，请参阅[为您的组织设置 Claude Code](https://code.claude.com/docs/zh-CN/admin-setup.md)。

要使通过 `ANTHROPIC_BASE_URL` 访问的网关成为托管机器唯一可以使用的目标，请在同一托管设置文件中将 [`allowedProviders`](https://code.claude.com/docs/zh-CN/settings-reference#allowedproviders) 设置为 `["customEndpoint"]`，并将网关的 `ANTHROPIC_BASE_URL` 放在该文件的 `env` 块中。Claude Code 随后会拒绝指向其他任何地方的会话，包括直接指向 Anthropic 或开发人员自己的代理，并仅接受您在那里设置的值的 `ANTHROPIC_BASE_URL`。对于通过提供商特定端点变量（如 `ANTHROPIC_BEDROCK_BASE_URL`）访问的网关，`allowedProviders` 条目指定要固定的变量。需要 Claude Code v2.1.285 或更高版本。

## 订阅和网关

当[网关凭证变量](https://code.claude.com/docs/zh-CN/llm-gateway-connect.md#set-the-credential-variable)或 `apiKeyHelper` 处于活动状态时，请求会使用该凭证代替开发人员的 claude.ai 订阅登录，订阅的使用限制不适用于这些请求。Claude Code 在机器上保存了 claude.ai 登录信息，但不会在这些请求中发送它。该流量按令牌计费给拥有网关转发的凭证的人，例如您的组织的 Anthropic Console 账户，或当网关路由到那里时您的 Amazon Bedrock、Google Cloud 的 Agent Platform 或 Microsoft Foundry 账户。

[`ANTHROPIC_BASE_URL`](https://code.claude.com/docs/zh-CN/llm-gateway-connect#set-the-base-url-and-credential)是指向 Claude Code 网关的变量。仅设置该变量，不设置网关凭证，不会替换订阅。请求仍然通过网关路由，但保存的 claude.ai 登录保持活动凭证，因此其使用限制和计费适用。将此流量转发给 Anthropic 的网关必须转发 `anthropic-beta` 中的 OAuth 功能；请参阅[请求头参考](https://code.claude.com/docs/zh-CN/llm-gateway-protocol.md#request-headers)。

## 相关页面

* [网关概述](https://code.claude.com/docs/zh-CN/gateways.md)：网关如何工作以及如何在 Claude apps gateway 和其他产品之间进行选择
* [Claude apps gateway](https://code.claude.com/docs/zh-CN/claude-apps-gateway)：Anthropic 的自托管网关，具有 SSO 登录和 OTLP 遥测
* [将 Claude Code 连接到 LLM 网关](https://code.claude.com/docs/zh-CN/llm-gateway-connect.md)：在您自己的机器上设置基础 URL 和凭证，具有每个表面的配置和故障排除表
* [为您的组织部署 LLM 网关](https://code.claude.com/docs/zh-CN/llm-gateway-rollout.md)：部署网关、颁发开发人员凭证和分发托管设置的管理员检查清单
* [网关兼容性指南](https://code.claude.com/docs/zh-CN/llm-gateway-protocol.md)：Claude Code 发送到网关的内容，供配置网关的运营商使用，涵盖端点、要转发的头和功能传递
