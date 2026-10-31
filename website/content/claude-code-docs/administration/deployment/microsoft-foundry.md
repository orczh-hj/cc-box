> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Microsoft Foundry 上的 Claude Code

> 了解如何通过 Microsoft Foundry 配置 Claude Code，包括设置、配置和故障排除。

**Deploying Claude Code across your organization?** Talk to sales about enterprise plans, SSO, and centralized billing.

View plans
Contact sales

## 前置条件

在使用 Microsoft Foundry 配置 Claude Code 之前，请确保您拥有：

* 具有 Microsoft Foundry 访问权限的 Azure 订阅
* 创建 Microsoft Foundry 资源和部署的 RBAC 权限
* 已安装并配置 Azure CLI（可选 - 仅在您没有其他获取凭证机制时需要）

> [!NOTE]
>
> 如果您要将 Claude Code 部署给多个用户，请[固定您的模型版本](#4-pin-model-versions)以防止在 Anthropic 发布新模型时出现破损。

## 设置

### 配置 Microsoft Foundry 资源

首先，在 Azure 中创建 Claude 资源：

1. 导航到 [Microsoft Foundry 门户](https://ai.azure.com/)
2. 创建新资源，记下您的资源名称
3. 为 Claude 模型创建部署，记下您为每个模型指定的部署名称；您将在第 4 步中将这些名称设置为模型变量：

   * Claude Opus
   * Claude Sonnet
   * Claude Haiku

   配置部署时，您还需要选择其[托管选项](https://platform.claude.com/docs/en/build-with-claude/claude-in-microsoft-foundry#hosting-options)，这决定了推理是在 Azure 上运行还是在 Anthropic 基础设施上运行。

### 配置 Azure 凭据

Claude Code 支持三种 Microsoft Foundry 身份验证方法。选择最适合您安全要求的方法：

* [API 密钥](#use-an-api-key)：从 Microsoft Foundry 门户复制密钥，并将其设置为 `ANTHROPIC_FOUNDRY_API_KEY`
* [Microsoft Entra ID](#use-microsoft-entra-id)：Claude Code 通过 Azure SDK 默认凭据链获取令牌，例如从 `az login` 会话中获取，因此无需存储 API 密钥
* [Bearer 令牌](#use-a-bearer-token)：由另一个进程获取 Microsoft Entra ID 访问令牌，然后您通过 `ANTHROPIC_FOUNDRY_AUTH_TOKEN` 传入

> [!NOTE]
>
> 使用 Microsoft Foundry 时，`/logout` 命令不可用，因为身份验证通过 Azure 凭据处理。

#### 使用 API 密钥

从 Microsoft Foundry 门户复制密钥，然后将其设置为环境变量：

1. 在 Microsoft Foundry 门户中导航到您的资源
2. 打开**端点和密钥**部分
3. 复制 **API 密钥**
4. 设置环境变量，将 `your-azure-api-key` 替换为您复制的密钥：

```bash theme={null}
export ANTHROPIC_FOUNDRY_API_KEY=your-azure-api-key
```

#### 使用 Microsoft Entra ID

不要设置 `ANTHROPIC_FOUNDRY_API_KEY` 和 `ANTHROPIC_FOUNDRY_AUTH_TOKEN`。这样 Claude Code 就会使用 Azure SDK [默认凭据链](https://learn.microsoft.com/en-us/azure/developer/javascript/sdk/authentication/credential-chains#defaultazurecredential-overview)。
这支持多种方法来验证本地和远程工作负载。

在本地计算机上，使用 Azure CLI 登录：

```bash theme={null}
az login
```

有关您的身份所需的角色，请参阅 [Azure RBAC 配置](#azure-rbac-configuration)。

#### 使用 Bearer 令牌

Claude Code 在每个请求中将 `ANTHROPIC_FOUNDRY_AUTH_TOKEN` 的值作为 `Authorization: Bearer` 标头发送。当另一个进程（例如主机应用程序或登录脚本）已经为您获取了访问令牌时，请使用此选项。

将变量设置为 Microsoft Entra ID 为您的资源颁发的 Bearer 令牌：

```bash theme={null}
export ANTHROPIC_FOUNDRY_AUTH_TOKEN=your-entra-access-token
```

`ANTHROPIC_FOUNDRY_AUTH_TOKEN` 优先于 `ANTHROPIC_FOUNDRY_API_KEY` 和默认凭据链。

### 配置 Claude Code

设置以下环境变量以启用 Microsoft Foundry：

```bash theme={null}
# 启用 Microsoft Foundry 集成
export CLAUDE_CODE_USE_FOUNDRY=1

# Azure 资源名称（将 {resource} 替换为您的资源名称）
export ANTHROPIC_FOUNDRY_RESOURCE={resource}
# 或提供完整的基础 URL：
# export ANTHROPIC_FOUNDRY_BASE_URL=https://{resource}.services.ai.azure.com/anthropic
```

将 `ANTHROPIC_FOUNDRY_RESOURCE` 仅设置为资源名称，例如 `my-resource`。如果设置为 URL 或主机名，Claude Code 会在您发送消息时[拒绝该值](https://code.claude.com/docs/zh-CN/errors.md#anthropic-foundry-resource-must-be-a-foundry-resource-name)。

### 固定模型版本

> [!WARNING]
>
> 为每个部署固定特定的模型版本。如果不固定版本，模型别名（如 `sonnet` 和 `opus`）会解析为 Claude Code 为 Microsoft Foundry 内置的默认值，这可能滞后于最新版本，并且可能在您的账户中尚不可用。Microsoft Foundry 没有启动模型检查，因此当默认值不可用时请求会失败。创建 Azure 部署时，请选择特定的模型版本而不是"自动更新到最新版本"。

设置模型变量以匹配您在第 1 步中创建的部署名称。

如果没有 `ANTHROPIC_DEFAULT_OPUS_MODEL`，Microsoft Foundry 上的 `opus` 别名会解析为 Opus 4.6。将其设置为 Opus 4.8 等更新的 Opus 模型的 ID：

```bash theme={null}
export ANTHROPIC_DEFAULT_OPUS_MODEL='claude-opus-4-8'
export ANTHROPIC_DEFAULT_SONNET_MODEL='claude-sonnet-5'
export ANTHROPIC_DEFAULT_HAIKU_MODEL='claude-haiku-4-5'
```

后台任务（如会话标题生成）使用小型/快速模型，通常是 Haiku 级别的模型。在 Microsoft Foundry 上，Claude Code 默认使用主模型，因为并非每个账户都有 Haiku 部署。要为后台任务使用 Haiku，请将 `ANTHROPIC_DEFAULT_HAIKU_MODEL` 设置为您账户中可用的 Haiku 部署，如上所示。

有关当前和旧版模型 ID，请参阅[模型概览](https://platform.claude.com/docs/en/about-claude/models/overview)。有关完整的环境变量列表，请参阅[模型配置](https://code.claude.com/docs/zh-CN/model-config.md#pin-models-for-third-party-deployments)。

[提示缓存](https://code.claude.com/docs/zh-CN/prompt-caching.md)会自动启用。要请求 1 小时的缓存 TTL 而不是 5 分钟的默认值，请设置以下变量；具有 1 小时 TTL 的缓存写入按更高的费率计费：

```bash theme={null}
export ENABLE_PROMPT_CACHING_1H=1
```

要为您的主对话和 Claude Code 在其外部进行的请求设置不同的 TTL，请[自己选择 TTL](https://code.claude.com/docs/zh-CN/prompt-caching.md#choose-the-ttl-yourself)。

### 运行 Claude Code

设置环境变量后，从您的项目目录启动 Claude Code：

```bash theme={null}
claude
```

Claude Code 从环境中读取 `CLAUDE_CODE_USE_FOUNDRY` 和其他 Microsoft Foundry 变量，并在第一个提示词时连接到您的 Azure 资源。与 Amazon Bedrock 和 Google Cloud 的 Agent Platform 不同，Microsoft Foundry 没有交互式设置向导，因此第 3 和第 4 步中的环境变量是唯一的配置路径。

要验证您的设置，请在 Claude Code 中运行 `/status`。API 提供商行显示 `Microsoft Foundry`，以及您配置的资源名称或基础 URL。

## Azure RBAC 配置

`Azure AI User` 和 `Cognitive Services User` 默认角色包括调用 Claude 模型所需的所有权限。

对于更严格的权限，请创建具有以下内容的自定义角色：

```json theme={null}
{
  "permissions": [
    {
      "dataActions": [
        "Microsoft.CognitiveServices/accounts/providers/*"
      ]
    }
  ]
}
```

有关详情，请参阅 [Microsoft Foundry RBAC 文档](https://learn.microsoft.com/en-us/azure/ai-foundry/concepts/rbac-azure-ai-foundry)。

## 1M token 上下文窗口

在 Microsoft Foundry 上，当 Claude Code 能够识别您的部署所服务的模型时，Fable 模型、Sonnet 5 及更高版本以及 Opus 4.7 及更高版本默认使用 [1M token 上下文窗口](https://platform.claude.com/docs/en/build-with-claude/context-windows#context-window-sizes-by-model)，无需添加 `[1m]` 后缀。Claude Code 从模型变量中的部署名称读取模型。请使用模型 ID 命名每个部署，例如 `claude-opus-4-8`，或者使用 [`modelOverrides`](https://code.claude.com/docs/zh-CN/model-config#override-model-ids-per-version) 将模型映射到您的部署名称。对于无法匹配到模型的部署名称，Claude Code 会假定使用 200K 窗口，除非您[声明其他窗口大小](https://code.claude.com/docs/zh-CN/model-config.md#correct-the-window-for-a-gateway-or-custom-model-id)。

以下 `settings.json` 条目告知 Claude Code，名为 `team-opus-prod` 的部署服务于 Opus 4.8：

```json theme={null}
{
  "modelOverrides": {
    "claude-opus-4-8": "team-opus-prod"
  }
}
```

如需改为保持 200K 窗口，请设置 [`CLAUDE_CODE_DISABLE_1M_CONTEXT=1`](https://code.claude.com/docs/zh-CN/model-config#turn-off-1m-context)。

当您在 `ANTHROPIC_DEFAULT_OPUS_MODEL` 或 `ANTHROPIC_DEFAULT_SONNET_MODEL` 中的部署名称后附加 `[1m]` 时，Opus 4.6 和 Sonnet 4.6 可使用 1M 窗口，如[为第三方部署固定模型](https://code.claude.com/docs/zh-CN/model-config.md#pin-models-for-third-party-deployments)中所述。在 v2.1.287 之前，Fable 模型以及 Opus 4.7 及更高版本在 Microsoft Foundry 上同样需要该后缀，否则默认使用 200K 窗口。

## 故障排除

如果您收到错误"Failed to get token from azureADTokenProvider: ChainedTokenCredential authentication failed"：

* 在环境中配置 Entra ID，或设置 `ANTHROPIC_FOUNDRY_API_KEY`。

如果请求在第一个提示上反复出现连接错误而失败：

* 检查 `ANTHROPIC_FOUNDRY_RESOURCE` 是否设置为您的实际资源名称，而不是占位符。Claude Code 从此值构建端点 URL，因此不正确的名称会指向不存在的主机。

## 其他资源

* [Microsoft Foundry 文档](https://learn.microsoft.com/en-us/azure/ai-foundry/what-is-azure-ai-foundry)
* [Microsoft Foundry 模型](https://ai.azure.com/explore/models)
* [Microsoft Foundry 定价](https://azure.microsoft.com/en-us/pricing/details/ai-foundry/)
