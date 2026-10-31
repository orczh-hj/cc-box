> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# AWS 上的 Claude Platform 中的 Claude Code

> 配置 Claude Code 以使用 Anthropic 运营的 Claude API，支持 AWS 身份验证、IAM 访问控制和 AWS Marketplace 计费。

AWS 上的 Claude Platform 是 Anthropic 运营的 Claude API，支持 AWS 身份验证、IAM 访问控制和 AWS Marketplace 计费。请求直接到达 Anthropic 的 API，因此您获得与 [Claude API](https://platform.claude.com/docs) 相同的模型和 API 功能，并遵循相同的发布计划。您使用 AWS 凭证或工作区 API 密钥进行身份验证，并通过 AWS Marketplace 付款。

Claude Code 通过 Anthropic 的功能标志服务启用的客户端功能默认处于关闭状态，[advisor 工具](https://code.claude.com/docs/zh-CN/advisor.md)不可用。有关完整列表，请参阅[功能可用性矩阵](https://code.claude.com/docs/zh-CN/feature-availability.md#summary-by-provider)。

使用本指南将 Claude Code 指向您已通过 AWS 上的 Claude Platform 配置的工作区。有关在此之前的 AWS 订阅和工作区设置，请参阅 [AWS 上的 Claude Platform 文档](https://platform.claude.com/docs/en/build-with-claude/claude-platform-on-aws)。

> [!NOTE]
>
> 通过 AWS Marketplace 订阅会配置一个与您的 AWS 账户关联的新 Anthropic 组织。此组织与您已有的任何 Anthropic 组织分开，凭证不会在它们之间转移。使用来自 AWS 关联组织的工作区 ID 和 API 密钥，而不是来自预先存在的 Claude Console 账户的凭证。

## 前置条件

在配置 Claude Code 之前，您需要：

* 通过 AWS Marketplace 的活跃 AWS 上的 Claude Platform 订阅
* 您的 AWS 关联 Anthropic 组织中的工作区及其工作区 ID
* 具有调用 Anthropic 服务权限的 IAM 主体，或限定于工作区的 API 密钥
* 您环境中的 AWS 凭证、`~/.aws/credentials` 中的凭证，或来自附加 IAM 角色的凭证（如果您想要 SigV4 身份验证）。AWS CLI 仅在 SSO 登录流程中是必需的。

## 设置

### 配置 AWS 凭证

Claude Code 支持两种 AWS 上的 Claude Platform 身份验证方法。请根据您的团队管理访问的方式选择合适的方法：

* [使用 SigV4 的 AWS 凭据](#use-aws-credentials-with-sigv4)：以 IAM 主体身份进行身份验证，凭据来自标准 AWS 凭据链
* [工作区 API 密钥](#use-a-workspace-api-key)：使用您在 AWS Console 中生成的长期有效密钥进行身份验证

#### 使用 SigV4 的 AWS 凭据

Claude Code 使用标准 AWS 凭证链使用 SigV4 对请求进行签名：环境变量、`~/.aws/credentials` 中的共享凭证、IAM 角色、AWS SSO 会话以及 AWS SDK 支持的任何其他来源。

对于本地使用，在启动 Claude Code 之前使用 AWS CLI 登录。下面的示例使用 SSO 配置文件，但任何在标准位置生成凭证的方法都可以工作。

```bash theme={null}
aws sso login --profile my-profile
export AWS_PROFILE=my-profile
```

对于 CI 和自动化，为运行程序提供具有调用 Anthropic 服务权限的 IAM 角色，并设置 `AWS_REGION`。凭证链会自动获取该角色。

如果您的 SSO 凭据在会话中途过期，请配置 [`awsAuthRefresh`](https://code.claude.com/docs/zh-CN/amazon-bedrock#advanced-credential-configuration)，以便 Claude Code 重新运行您的登录命令并重试，而不是失败。将命令添加到您的[设置文件](https://code.claude.com/docs/zh-CN/settings.md)，例如 `~/.claude/settings.json`：

```json theme={null}
{
  "awsAuthRefresh": "aws sso login --profile my-profile"
}
```

Claude Code 在启动时如果无法验证您现有的 AWS 凭据，也会运行此命令，并在 `Authentication` 面板中显示命令的输出，直到登录完成。

配置了 `awsAuthRefresh` 后，运行 `/login`，选择 **3rd-party platform**，然后在 **Using 3rd-party platforms** 下选择 **Claude Platform on AWS · refresh credentials**。Claude Code 运行配置的命令并重新读取您的 AWS 凭证，无需重启。

#### 使用工作区 API 密钥

工作区 API 密钥是一个长期有效的密钥，当您不想管理联合 AWS 凭证时很有用。在 AWS Console 中的 **Claude Platform on AWS → API keys** 下生成一个，并将其设置为 `ANTHROPIC_AWS_API_KEY`：

```bash theme={null}
export ANTHROPIC_AWS_API_KEY=sk-ant-xxxxx
```

该密钥作为 `x-api-key` 发送，优先于 SigV4，因此您环境中的任何 AWS 凭证都会被忽略。来自单独 Claude Console 组织的 API 密钥在此处不起作用。

像对待任何其他生产凭证一样对待工作区 API 密钥。[用户设置文件](https://code.claude.com/docs/zh-CN/settings.md) `env` 块是一种方便的方式，可以将密钥限定于您的机器，而无需全局导出。

> [!NOTE]
>
> `/login` 和 `/logout` 命令不会将您登录到 Claude Platform on AWS 的 claude.ai 订阅。身份验证通过您的 AWS 凭证或工作区 API 密钥运行。

### 配置 Claude Code

设置环境变量，将 Claude Code 路由通过 AWS 上的 Claude Platform，而不是默认的 Anthropic API。

```bash theme={null}
export CLAUDE_CODE_USE_ANTHROPIC_AWS=1
export ANTHROPIC_AWS_WORKSPACE_ID=wrkspc_01ABCDEFGHIJKLMN
export AWS_REGION=us-east-1
```

`ANTHROPIC_AWS_WORKSPACE_ID` 是必需的。Claude Code 在每个请求中将其作为 `anthropic-workspace-id` 标头发送。将示例 `wrkspc_01ABCDEFGHIJKLMN` 值替换为您从 AWS 上的 Claude Platform 设置中获得的工作区 ID。

Claude Code 从 AWS 区域计算基础 URL 为 `https://aws-external-anthropic.{region}.api.aws`，它使用[与 Amazon Bedrock 相同的优先级](https://code.claude.com/docs/zh-CN/amazon-bedrock.md#3-configure-claude-code)进行解析。要直接覆盖 URL，请设置 `ANTHROPIC_AWS_BASE_URL`。

即使您的环境中存在 AWS 凭证，AWS 上的 Claude Platform 也是可选的。Amazon Bedrock 和 Microsoft Foundry 在提供程序路由中优先，因此如果设置了 `CLAUDE_CODE_USE_BEDROCK` 和 `CLAUDE_CODE_USE_FOUNDRY`，请取消设置它们。

### 固定模型版本

AWS 上的 Claude Platform 使用与直接 Claude API 相同的模型 ID。

默认别名 `fable`、`opus`、`sonnet` 和 `haiku` 解析为 Claude Code 为 AWS 上的 Claude Platform 内置的默认值，这些值可能滞后于最新版本。如果没有 `ANTHROPIC_DEFAULT_OPUS_MODEL`，`opus` 别名解析为 Opus 5.5。在 v2.1.280 之前，它从 v2.1.219 开始解析为 Opus 5。

如果您将 Claude Code 部署到团队，请显式固定模型 ID，以便新版本不会一次性移动所有人：

```bash theme={null}
export ANTHROPIC_DEFAULT_FABLE_MODEL=claude-fable-5
export ANTHROPIC_DEFAULT_OPUS_MODEL=claude-opus-4-8
export ANTHROPIC_DEFAULT_SONNET_MODEL=claude-sonnet-5
export ANTHROPIC_DEFAULT_HAIKU_MODEL=claude-haiku-4-5
```

有关模型 ID 和别名的完整列表，请参阅 [Models overview](https://platform.claude.com/docs/en/about-claude/models/overview)。有关其他与模型相关的变量，请参阅 [Model configuration](https://code.claude.com/docs/zh-CN/model-config.md)。

[Prompt caching](https://code.claude.com/docs/zh-CN/prompt-caching.md) 会自动启用。要请求 1 小时缓存 TTL 而不是 5 分钟默认值，请设置 `ENABLE_PROMPT_CACHING_1H=1`。API 按更高的费率对 1 小时缓存写入进行计费。有关费率，请参阅 [prompt caching pricing](https://platform.claude.com/docs/en/build-with-claude/prompt-caching#pricing)。

要为您的主对话和 Claude Code 在其外部进行的请求设置不同的 TTL，请[自己选择 TTL](https://code.claude.com/docs/zh-CN/prompt-caching.md#choose-the-ttl-yourself)。

### 启动并验证

启动 Claude Code 并确认路由：

```bash theme={null}
claude
```

当提供程序处于活动状态时，启动横幅显示 `Claude Platform on AWS`。运行 `/status` 检查详细信息：`API provider` 行读取 `Claude Platform on AWS`，输出包括您的 `Workspace ID`、`AWS region` 和 `Claude Platform on AWS base URL`（如果您设置了覆盖）。

## 使用 Agent SDK

[Agent SDK](https://code.claude.com/docs/zh-CN/agent-sdk/overview.md) 读取与 CLI 相同的环境变量，因此任何生成 Claude Code 子进程的程序都可以通过在调用前导出 `CLAUDE_CODE_USE_ANTHROPIC_AWS`、`ANTHROPIC_AWS_WORKSPACE_ID` 和 `ANTHROPIC_AWS_API_KEY` 或 AWS 凭证来针对 AWS 上的 Claude Platform。

```typescript theme={null}
import { query } from "@anthropic-ai/claude-agent-sdk";

process.env.CLAUDE_CODE_USE_ANTHROPIC_AWS = "1";
process.env.ANTHROPIC_AWS_WORKSPACE_ID = "wrkspc_01ABCDEFGHIJKLMN";
process.env.AWS_REGION = "us-east-1";

for await (const msg of query({ prompt: "What's in this repo?" })) {
  console.log(msg);
}
```

此示例依赖于环境 AWS 凭证链进行 SigV4。要改为使用工作区 API 密钥进行身份验证，请以相同方式设置 `ANTHROPIC_AWS_API_KEY`。有关更广泛的 Agent SDK 表面，请参阅 [Agent SDK overview](https://code.claude.com/docs/zh-CN/agent-sdk/overview.md)。

## 通过企业代理路由

要通过代理或 [LLM gateway](https://code.claude.com/docs/zh-CN/llm-gateway) 路由流量，请将 `ANTHROPIC_AWS_BASE_URL` 设置为代理的地址。Claude Code 将请求发送到该 URL，并使用相同的工作区和身份验证标头，因此任何转发它们不变的网关都可以工作。

```bash theme={null}
export CLAUDE_CODE_USE_ANTHROPIC_AWS=1
export ANTHROPIC_AWS_WORKSPACE_ID=wrkspc_01ABCDEFGHIJKLMN
export ANTHROPIC_AWS_BASE_URL=https://anthropic-proxy.example.com
```

如果您的网关自己对请求进行签名，请设置 `CLAUDE_CODE_SKIP_ANTHROPIC_AWS_AUTH=1`，以便 Claude Code 发送未签名的请求，让网关在转发到 AWS 之前添加 SigV4 标头。如果网关需要自己的令牌，请在 `ANTHROPIC_AUTH_TOKEN` 中设置它。

```bash theme={null}
export CLAUDE_CODE_USE_ANTHROPIC_AWS=1
export CLAUDE_CODE_SKIP_ANTHROPIC_AWS_AUTH=1
export ANTHROPIC_AWS_WORKSPACE_ID=wrkspc_01ABCDEFGHIJKLMN
export ANTHROPIC_AWS_BASE_URL=https://anthropic-proxy.example.com
```

## 故障排除

运行 `/status` 以查看已解析的提供程序以及任何显式配置的工作区 ID、区域、基础 URL 覆盖和身份验证跳过设置。这是确认 Claude Code 是否针对 AWS 上的 Claude Platform 的最快方法。

### `403 Forbidden` 或 `AccessDenied` 在每个请求上

Claude Code 解析的 IAM 主体可能缺少在您的工作区中调用 Anthropic 服务的权限。检查附加到您的 AWS 配置文件或启动 Claude Code 的运行程序的角色，并验证它具有 [IAM action reference](https://platform.claude.com/docs/zh-CN/api/claude-platform-on-aws-iam-actions) 中记录的 `aws-external-anthropic` 操作。

如果您设置了 `ANTHROPIC_AWS_API_KEY`，该密钥优先于 SigV4，过期的密钥会产生相同的错误。在 AWS Console 中的 **Claude Platform on AWS → API keys** 下重新生成密钥，或取消设置变量以回退到您的 AWS 凭证。

### 请求失败，出现缺少工作区错误

`ANTHROPIC_AWS_WORKSPACE_ID` 可能未设置或为空。每个 Claude Platform on AWS 请求都必须包含工作区 ID。它不是由您的 AWS 凭证隐含的。在您的 Claude Platform on AWS 设置中找到 ID，并在启动 Claude Code 之前导出它。

### 请求仍然转到 `api.anthropic.com`

`CLAUDE_CODE_USE_ANTHROPIC_AWS` 可能未设置或设置为不解析为真值的值。将其设置为 `1` 并运行 `/status` 以确认已解析的提供程序。如果还设置了 `CLAUDE_CODE_USE_BEDROCK` 或 `CLAUDE_CODE_USE_FOUNDRY`，这些会优先于 AWS 上的 Claude Platform。

## 其他资源

配置 Claude Code 之前的 AWS 上的 Claude Platform 订阅、工作区和 IAM 设置在平台文档中介绍：

* [Claude Platform on AWS overview](https://platform.claude.com/docs/en/build-with-claude/claude-platform-on-aws)：订阅、工作区设置和产品参考
* [IAM action reference](https://platform.claude.com/docs/en/api/claude-platform-on-aws-iam-actions)：权限和托管策略
