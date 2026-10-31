> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Amazon Bedrock 上的 Claude Code

> 了解如何通过 Amazon Bedrock 配置 Claude Code，包括设置、IAM 配置和故障排除。

**Deploying Claude Code across your organization?** Talk to sales about enterprise plans, SSO, and centralized billing.

View plans
Contact sales

## 前置条件

在使用 Amazon Bedrock 配置 Claude Code 之前，请确保您拥有：

* 启用了 Amazon Bedrock 访问权限的 AWS 账户
* 在 Amazon Bedrock 中访问所需的 Claude 模型（例如 Claude Sonnet 4.6）
* 已安装并配置 AWS CLI（可选 - 仅在您没有其他获取凭证的机制时需要）
* 适当的 IAM 权限

要使用您自己的 Amazon Bedrock 凭证登录，请按照下面的[使用 Amazon Bedrock 登录](#sign-in-with-bedrock)进行操作。要在团队中部署 Claude Code，请使用[手动设置](#set-up-manually)步骤并在推出前[固定您的模型版本](#4-pin-model-versions)。

## 使用 Bedrock 登录

如果您拥有 AWS 凭据，并希望通过 Amazon Bedrock 开始使用 Claude Code，登录向导会引导您完成整个过程。每个账户只需完成一次 AWS 端的前提条件；Claude Code 端的设置由向导处理。

**1. 在您的 AWS 账户中启用 Anthropic 模型**

在 [Amazon Bedrock 控制台](https://console.aws.amazon.com/bedrock/)中，打开 Model catalog，选择一个 Anthropic 模型，然后提交用例表单。提交后即可立即获得访问权限。有关 AWS Organizations，请参阅[提交用例详细信息](#1-submit-use-case-details)；有关您的角色所需的权限，请参阅 [IAM 配置](#iam-configuration)。

**2. 启动 Claude Code 并选择 Amazon Bedrock**

运行 `claude`。在登录提示中，选择 **3rd-party platform**，然后选择 **Amazon Bedrock**。如果您已经登录并看到的是聊天输入框，请运行 `/setup-bedrock` 打开向导。在设置 `CLAUDE_CODE_USE_BEDROCK=1` 之前，Claude Code 会[在命令菜单中隐藏该命令](https://code.claude.com/docs/zh-CN/commands.md#how-the-command-menu-matches-what-you-type)；请完整输入该命令。

**3. 按照向导提示操作**

选择您向 AWS 进行身份验证的方式：从您的 `~/.aws` 目录中检测到的 AWS 配置文件、Amazon Bedrock API 密钥、访问密钥和密钥，或环境中已有的凭据。向导会询问您的区域，验证您的账户可以调用哪些 Claude 模型，并允许您固定这些模型。它会将结果保存到您的[用户设置文件](https://code.claude.com/docs/zh-CN/settings.md)的 `env` 块中，因此您无需自行导出环境变量。

登录后，您可以随时运行 `/setup-bedrock` 重新打开向导，以更改您的凭据、区域或固定的模型。模型固定步骤会从您当前固定的模型开始。向导会写入 `~/.claude/settings.json`；如果设置了 [`CLAUDE_CONFIG_DIR`](https://code.claude.com/docs/zh-CN/env-vars#variables)，则写入 `$CLAUDE_CONFIG_DIR/settings.json`。

## 手动设置

要通过环境变量而不是向导来配置 Amazon Bedrock，例如在 CI 或脚本化企业部署中，请按照以下步骤操作。

### 提交用例详情

在首次调用 Anthropic 模型之前，请提交用例详情。每个 AWS 账户只需执行一次。

1. 确保您拥有下面描述的正确 IAM 权限
2. 导航到 [Amazon Bedrock 控制台](https://console.aws.amazon.com/bedrock/)
3. 从**模型目录**中选择 Anthropic 模型
4. 完成用例表单。提交后立即授予访问权限。

如果您使用 AWS Organizations，可以使用 [`PutUseCaseForModelAccess` API](https://docs.aws.amazon.com/bedrock/latest/APIReference/API_PutUseCaseForModelAccess.html) 从管理账户提交一次表单。此调用需要 `bedrock:PutUseCaseForModelAccess` IAM 权限。批准会自动扩展到子账户。

### 配置 AWS 凭证

Claude Code 使用默认 AWS SDK 凭据链。如果机器已经向该链提供凭据，例如 Amazon EC2 实例配置文件或 Amazon ECS 任务凭据，请直接跳到[第 3 步](#3-configure-claude-code)。

AWS [不建议在开发专用软件或处理真实数据时使用 IAM 用户的访问密钥](https://docs.aws.amazon.com/cli/latest/userguide/cli-authentication-user.html)。使用以下方法之一设置您的凭据：

* [`aws configure`](#use-aws-configure)：将 IAM 用户的访问密钥保存到 `~/.aws` 目录中的配置文件
* [访问密钥环境变量](#export-an-access-key)：仅在当前 shell 中设置访问密钥，或带会话令牌的临时凭据
* [SSO 配置文件](#use-an-sso-profile)：在浏览器中通过 IAM Identity Center 登录并获取临时凭据。如果您通过 IAM Identity Center 访问 AWS 账户，请使用此方法。
* [AWS 管理控制台凭据](#use-aws-management-console-credentials)：在浏览器中使用您的 AWS 管理控制台凭据登录并获取临时凭据。如果您以根用户、IAM 用户身份或通过 IAM 联合身份访问 AWS 账户，AWS [推荐此方法](https://docs.aws.amazon.com/signin/latest/userguide/command-line-sign-in.html)。
* [Amazon Bedrock API 密钥](#use-an-amazon-bedrock-api-key)：使用仅适用于 Amazon Bedrock 的持有者令牌进行身份验证，而不是使用 AWS 凭据

#### 使用 `aws configure`

运行 `aws configure`，并在提示时输入您的访问密钥 ID、秘密访问密钥和默认区域：

```bash theme={null}
aws configure
```

AWS CLI 会将密钥保存到 `~/.aws/credentials` 中的 `default` 配置文件，凭据链会从那里读取它。

#### 导出访问密钥

将您的访问密钥导出为环境变量。`AWS_SESSION_TOKEN` 仅在使用临时凭据时需要，因此如果您的访问密钥属于 IAM 用户，请省略该行：

```bash theme={null}
export AWS_ACCESS_KEY_ID=your-access-key-id
export AWS_SECRET_ACCESS_KEY=your-secret-access-key
export AWS_SESSION_TOKEN=your-session-token
```

#### 使用 SSO 配置文件

如果您还没有配置文件，请使用 `aws configure sso` 创建一个。然后登录 IAM Identity Center 并设置 `AWS_PROFILE`，使凭据链使用该配置文件。在运行这些命令之前，将 `your-profile-name` 替换为您的 AWS 配置文件的名称。

```bash theme={null}
aws sso login --profile=your-profile-name

export AWS_PROFILE=your-profile-name
```

Claude Code 从配置文件的 `sso_region` 命名的 IAM Identity Center 区域请求角色凭据，这不需要与您运行 Amazon Bedrock 的区域匹配。

#### 使用 AWS 管理控制台凭据

`aws login` 命令需要 AWS CLI 2.32.0 或更高版本。有关您的身份所需的 IAM 策略，请参阅 [AWS 关于 `aws login` 的说明](https://docs.aws.amazon.com/signin/latest/userguide/command-line-sign-in.html)。

运行以下命令，在浏览器中使用您的 AWS 管理控制台凭据登录：

```bash theme={null}
aws login
```

会话最长有效 12 小时，之后需要再次运行 `aws login`。

#### 使用 Amazon Bedrock API 密钥

Amazon Bedrock API 密钥是一种持有者令牌，可代替 AWS 凭据对您的请求进行身份验证。AWS 提供[两种类型的密钥](https://docs.aws.amazon.com/bedrock/latest/userguide/api-keys.html)：

* **短期密钥**：最长有效 12 小时。对于生产环境，AWS 更推荐使用短期密钥而非长期密钥。
* **长期密钥**：在您设置的到期日期之前一直有效。AWS 仅建议将其用于探索。

将密钥导出为 `AWS_BEARER_TOKEN_BEDROCK`：

```bash theme={null}
export AWS_BEARER_TOKEN_BEDROCK=your-bedrock-api-key
```

设置 `AWS_BEARER_TOKEN_BEDROCK` 后，Claude Code 会使用该密钥进行身份验证，并且不会解析凭据链，即使存在其他 AWS 凭据也是如此。[了解更多关于 Amazon Bedrock API 密钥](https://aws.amazon.com/blogs/machine-learning/accelerate-ai-development-with-amazon-bedrock-api-keys/)。

#### 凭证缓存和解析超时

Claude Code 解析 AWS 默认凭据提供程序链一次，并将解析的凭据保存在内存中。它重复使用这些凭据，直到它们过期前五分钟，或在没有过期时间时使用一小时，因此 SSO 支持的配置文件大约每个凭据生命周期从 IAM Identity Center 请求一次凭据。来自 API 的凭据错误会清除缓存，重试会解析新凭据。

缓存涵盖此步骤开头列出的所有凭据方法，但 Amazon Bedrock API 密钥除外，它不使用提供程序链。要在每个请求上解析链，请改为设置 [`CLAUDE_CODE_SKIP_AWS_CRED_CACHE=1`](https://code.claude.com/docs/zh-CN/env-vars)。

填充缓存的解析在 60 秒后超时。如果链中的某个步骤停滞，例如等待无法接收的输入的 `credential_process` 帮助程序，请求会失败并显示 [`AWS default-chain credential resolve timed out`](https://code.claude.com/docs/zh-CN/errors#aws-default-chain-credential-resolve-timed-out)。如果您的链运行合法需要更长时间的交互式登录，例如通过 `aws-vault` 等包装器进行基于浏览器的 SSO 和 MFA，请使用 [`CLAUDE_CODE_AWS_CHAIN_RESOLVE_TIMEOUT_MS`](https://code.claude.com/docs/zh-CN/env-vars) 以毫秒为单位提高限制。设置 `CLAUDE_CODE_SKIP_AWS_CRED_CACHE=1` 后，每个 API 请求解析链时不受此限制。

除了使用 Amazon Bedrock API 密钥进行身份验证外，[设置向导](#sign-in-with-bedrock)对它在验证您的凭证时进行的每个 AWS 调用以及每个模型检查之前的凭证查找应用相同的限制。在凭证验证期间，超过限制的检查会失败并显示 [`Timed out after 60s waiting for AWS`](https://code.claude.com/docs/zh-CN/errors#bedrock-setup-verification-timed-out-waiting-for-aws)。

#### 高级凭证配置

Claude Code 支持 AWS SSO 和企业身份提供程序的自动凭证刷新。将这些设置添加到您的 Claude Code 设置文件（有关文件位置，请参阅[设置](https://code.claude.com/docs/zh-CN/settings.md)）。

这两个设置有不同的触发条件：

* **`awsAuthRefresh`**：仅当 Claude Code 检测到您的 AWS 凭证已过期时运行，可以基于其时间戳在本地检测，或者当 API 返回凭证错误时，然后使用刷新的凭证重试请求。
* **`awsCredentialExport`**：在会话启动和每次凭证重新加载时运行，即使 AWS 默认凭证提供程序链中的凭证仍然有效。当您的 Amazon Bedrock 账户需要与默认提供程序链将解析的凭证不同的跨账户凭证时，请使用此选项。

在运行 `awsAuthRefresh` 命令之前，Claude Code 会进行 STS `GetCallerIdentity` 调用以确认您的凭证确实已过期，并在凭证仍然有效时跳过该命令。Claude Code 通过您的[代理配置](https://code.claude.com/docs/zh-CN/network-config.md#proxy-configuration)发送此检查，遵守 `HTTPS_PROXY` 和 `NO_PROXY`。在 v2.1.239 之前，Claude Code 直接发送此检查，在仅允许通过代理出站的网络上在启动时挂起。

##### 示例配置

```json theme={null}
{
  "awsAuthRefresh": "aws sso login --profile myprofile",
  "env": {
    "AWS_PROFILE": "myprofile"
  }
}
```

##### 配置设置说明

**`awsAuthRefresh`**：用于修改 `.aws` 目录的命令，例如更新凭证、SSO 缓存或配置文件。命令的输出显示给用户，但不支持交互式输入。这适用于基于浏览器的 SSO 流，其中 CLI 显示 URL 或代码，您在浏览器中完成身份验证。

**`awsCredentialExport`**：仅当您无法修改 `.aws` 且必须直接返回凭证时才使用此选项。输出被静默捕获，不显示给用户。命令必须输出以下格式的 JSON：

```json theme={null}
{
  "Credentials": {
    "AccessKeyId": "value",
    "SecretAccessKey": "value",
    "SessionToken": "value",
    "Expiration": "2026-01-01T00:00:00Z"
  }
}
```

来自 `aws configure export-credentials --format process` 的平面输出也被接受，在顶级而不是嵌套在 `Credentials` 下具有相同的密钥。

`Expiration` 是可选的。当命令返回有效的 ISO 8601 `Expiration` 时，Claude Code 会缓存凭证直到该时间前五分钟。没有它，凭证被缓存一小时。

当您配置 `awsCredentialExport` 而不配置 `awsAuthRefresh` 时，Claude Code 直接使用导出的凭据，不在启动时重新解析 AWS 默认凭据提供程序链。

### 配置 Claude Code

设置以下环境变量以启用 Amazon Bedrock：

```bash theme={null}
# Enable Bedrock integration
export CLAUDE_CODE_USE_BEDROCK=1
export AWS_REGION=us-east-1  # optional if your AWS profile already sets a region

# Optional: Override the AWS region for the small/fast model (Bedrock and Mantle).
# On Bedrock, has no effect without ANTHROPIC_DEFAULT_HAIKU_MODEL
# or the deprecated ANTHROPIC_SMALL_FAST_MODEL set.
export ANTHROPIC_SMALL_FAST_MODEL_AWS_REGION=us-west-2

# Optional: Override the Bedrock endpoint URL for custom endpoints or gateways
# export ANTHROPIC_BEDROCK_BASE_URL=https://bedrock-runtime.us-east-1.amazonaws.com
```

为 Claude Code 启用 Amazon Bedrock 时，请记住以下几点：

* 您只需设置 `AWS_REGION` 来覆盖您的 AWS 配置文件的区域或当您的配置文件没有区域时。Claude Code 按以下顺序解析区域：

  * `AWS_REGION`
  * `AWS_DEFAULT_REGION`
  * 在您的活跃 AWS 配置文件上设置的 `region`，首先从 AWS 共享凭证文件读取，然后从共享配置文件读取，匹配 AWS SDK 优先级
  * `us-east-1`

  如果来自任何这些源的值看起来不像区域名称，Claude Code 会将其视为未设置并继续按顺序进行。例如，Claude Code 将包含斜杠、点或空格的值视为未设置。

  活跃配置文件是 `AWS_PROFILE`（如果已设置），否则为 `default`。设置 `AWS_SHARED_CREDENTIALS_FILE` 或 `AWS_CONFIG_FILE` 以指向非默认文件路径。

  运行 `/status` 以查看解析的区域。当区域来自您的 AWS 配置文件或默认回退时，Claude Code 也会在 `/status` 输出中注明源。
* 使用 Amazon Bedrock 时，`/logout` 命令不可用，因为身份验证通过 AWS 凭证处理。
* WebSearch 工具在 Amazon Bedrock 上不可用。请参阅 [WebSearch 工具行为](https://code.claude.com/docs/zh-CN/tools-reference.md#websearch-tool-behavior)。
* 您可以对不想泄露给其他进程的环境变量（如 `AWS_PROFILE`）使用设置文件。有关更多信息，请参阅[设置](https://code.claude.com/docs/zh-CN/settings.md)。

### 固定模型版本

> [!WARNING]
>
> 在部署到多个用户时固定特定模型版本。不固定的话，模型别名（如 `sonnet` 和 `opus`）会解析为 Claude Code 对 Amazon Bedrock 的内置默认值，这可能滞后于最新版本，并且可能在您的账户中尚不可用。Claude Code 在启动时当默认值不可用时会[回退](#startup-model-checks)到较早或较低级别的模型，但固定让您可以控制用户何时迁移到新模型。

将这些环境变量设置为特定的 Amazon Bedrock 模型 ID。

没有 `ANTHROPIC_DEFAULT_OPUS_MODEL` 时，Amazon Bedrock 上的 `opus` 别名解析为 Opus 5.5，没有 `ANTHROPIC_DEFAULT_SONNET_MODEL` 时，`sonnet` 别名解析为 Sonnet 4.5。此示例将每个别名固定到特定版本：

```bash theme={null}
export ANTHROPIC_DEFAULT_OPUS_MODEL='us.anthropic.claude-opus-4-8'
export ANTHROPIC_DEFAULT_SONNET_MODEL='us.anthropic.claude-sonnet-4-6'
export ANTHROPIC_DEFAULT_HAIKU_MODEL='us.anthropic.claude-haiku-4-5-20251001-v1:0'
```

这些 ID 使用 `us.` 跨区域推理配置文件前缀。如果您使用不同的区域前缀或应用程序推理配置文件，请相应调整。在 AWS GovCloud 区域中，使用 `us-gov.` 前缀。

要保留内置默认模型并仅更改其首选前缀，请改为设置 [`ANTHROPIC_BEDROCK_REGION_PREFIX`](#cross-region-inference-profile-prefixes)。差异显示在 `opus` 别名解析为什么：

| 您设置 | `opus` 别名解析为 |
| :- | :- |
| `ANTHROPIC_DEFAULT_OPUS_MODEL='us.anthropic.claude-opus-4-8'` | `us.anthropic.claude-opus-4-8`，您固定的确切 ID |
| `ANTHROPIC_BEDROCK_REGION_PREFIX=eu` | `eu.anthropic.claude-opus-5-5`，具有您首选前缀的内置默认值 |

有关当前和旧版模型 ID，请参阅[模型概览](https://platform.claude.com/docs/en/about-claude/models/overview)。有关完整的固定环境变量列表，请参阅[模型配置](https://code.claude.com/docs/zh-CN/model-config.md#pin-models-for-third-party-deployments)。

未设置固定变量时，Claude Code 使用这些默认模型：

| 模型类型 | 默认模型 |
| :- | :- |
| 主要模型 | Opus 5.5，例如 `us-*` 区域中的 `us.anthropic.claude-opus-5-5` |
| 小型/快速模型 | Sonnet 4.5，例如 `us-*` 区域中的 `us.anthropic.claude-sonnet-4-5-20250929-v1:0` |

在 v2.1.280 之前，Amazon Bedrock 上的主要模型默认为 Opus 5，`opus` 别名从 v2.1.219 起解析为 Opus 5。

后台任务（如会话标题生成）使用小型/快速模型，通常是 Haiku 级别的模型。在 Amazon Bedrock 上，Claude Code 为后台任务使用默认 Sonnet 模型，因为 Haiku 可能不会在每个账户或区域中启用。两个选择改变哪个模型执行它们：

* 当您使用 `--model`、`ANTHROPIC_MODEL` 或 `model` 设置选择主要模型时，后台任务使用该模型。当 Claude Code 在您使用 [`ANTHROPIC_DEFAULT_MODEL`](https://code.claude.com/docs/zh-CN/model-config#set-a-default-model-for-new-sessions) 设置的模型上启动会话时，后台任务也使用该模型。设置 `ANTHROPIC_DEFAULT_OPUS_MODEL` 而不设置 `ANTHROPIC_DEFAULT_SONNET_MODEL` 也算作选择，因为内置 Sonnet 模型可能在引导自己的 Opus 的账户中不启用。
* 要为后台任务使用 Haiku，请将 `ANTHROPIC_DEFAULT_HAIKU_MODEL` 设置为在您的账户中可用的模型 ID。

> [!WARNING]
>
> Opus 模型的每 token 价格高于 Sonnet 模型，因此如果您不固定主要模型，将按 Opus 费率计费。要将 Sonnet 4.5 用作主要模型，请将 `ANTHROPIC_MODEL` 设置为其完整模型 ID。使用 `ANTHROPIC_DEFAULT_SONNET_MODEL` 引导默认值且不设置 `ANTHROPIC_DEFAULT_OPUS_MODEL` 的部署保留其引导的 Sonnet 模型作为默认值。

要进一步自定义模型，请使用以下方法之一：

```bash theme={null}
# 使用推理配置文件 ID
export ANTHROPIC_MODEL='us.anthropic.claude-sonnet-4-6'
export ANTHROPIC_DEFAULT_HAIKU_MODEL='us.anthropic.claude-haiku-4-5-20251001-v1:0'

# 使用应用程序推理配置文件 ARN
export ANTHROPIC_MODEL='arn:aws:bedrock:us-east-2:your-account-id:application-inference-profile/your-model-id'

# 可选：如果需要禁用提示缓存
# export DISABLE_PROMPT_CACHING=1

# 可选：请求 1 小时提示缓存 TTL 而不是 5 分钟默认值
# export ENABLE_PROMPT_CACHING_1H=1
```

1 小时缓存 TTL 的计费费率高于 5 分钟默认值。请参阅[缓存生命周期](https://code.claude.com/docs/zh-CN/prompt-caching.md#cache-lifetime)。要为您的主要对话和 Claude Code 在其外部进行的请求设置不同的 TTL，请[自己选择 TTL](https://code.claude.com/docs/zh-CN/prompt-caching.md#choose-the-ttl-yourself)。

> [!NOTE]
>
> 提示缓存可能不在所有 Amazon Bedrock 区域中可用。如果缓存令牌计数保持为零，请检查 Amazon Bedrock 文档中的[支持的模型、区域和限制](https://docs.aws.amazon.com/bedrock/latest/userguide/prompt-caching.html#prompt-caching-models)。

#### 将每个模型版本映射到推理配置文件

`ANTHROPIC_DEFAULT_*_MODEL` 环境变量为每个模型系列配置一个推理配置文件。如果您的组织需要在 `/model` 选择器中公开同一系列的多个版本，每个版本路由到其自己的应用程序推理配置文件 ARN，请改为在您的[设置文件](https://code.claude.com/docs/zh-CN/settings.md#where-settings-live)中使用 `modelOverrides` 设置。

此示例将四个 Opus 版本映射到不同的 ARN，以便用户可以在它们之间切换，而无需绕过您的组织的推理配置文件：

```json theme={null}
{
  "modelOverrides": {
    "claude-opus-4-7": "arn:aws:bedrock:us-east-2:123456789012:application-inference-profile/opus-47-prod",
    "claude-opus-4-6": "arn:aws:bedrock:us-east-2:123456789012:application-inference-profile/opus-46-prod",
    "claude-opus-4-5-20251101": "arn:aws:bedrock:us-east-2:123456789012:application-inference-profile/opus-45-prod",
    "claude-opus-4-1-20250805": "arn:aws:bedrock:us-east-2:123456789012:application-inference-profile/opus-41-prod"
  }
}
```

当用户在 `/model` 中选择这些版本之一时，Claude Code 使用映射的 ARN 调用 Amazon Bedrock。当您通过 `--model` 或 `ANTHROPIC_MODEL` 直接传递 Anthropic 模型 ID 时，相同的映射也适用。没有覆盖的版本回退到内置 Amazon Bedrock 模型 ID 或在启动时发现的任何匹配推理配置文件。在 v2.1.200 之前，`--model` 和 `ANTHROPIC_MODEL` 值直接到达 Amazon Bedrock，不经过覆盖映射。有关覆盖如何与 `availableModels` 和其他模型设置交互的详细信息，请参阅[按版本覆盖模型 ID](https://code.claude.com/docs/zh-CN/model-config.md#override-model-ids-per-version)。

## 启动模型检查

当 Claude Code 配置了 Amazon Bedrock 时，它会验证其打算使用的模型在您的账户中是否可访问。

如果您已固定的模型版本比 Claude Code 当前默认版本更旧，且您的账户可以调用较新版本，Claude Code 会提示您更新该固定版本。接受会将新的模型 ID 写入您的[用户设置文件](https://code.claude.com/docs/zh-CN/settings.md)并重启 Claude Code。拒绝会被记住，直到下一次默认版本更改。指向[应用推理配置文件 ARN](#map-each-model-version-to-an-inference-profile) 的固定版本会被跳过，因为这些由您的管理员管理。

如果您未固定模型且当前默认模型在您的账户中不可用，Claude Code 会为当前会话进行回退并显示通知。它首先尝试默认模型的早期版本，当默认模型是 Opus 模型且没有可用的 Opus 版本时，会回退到默认 Sonnet 模型。回退不会被保留。在您的 Amazon Bedrock 账户中启用较新的模型或[固定一个版本](#4-pin-model-versions)以使选择永久生效。

当您在特定 Sonnet 或 Opus 版本上启动会话时，例如使用 `--model`、`ANTHROPIC_MODEL` 或[`model` 设置](https://code.claude.com/docs/zh-CN/settings-reference#model)，该版本将作为会话针对匹配的 `sonnet` 或 `opus` 别名的固定默认版本。Claude Code 会跳过对您的模型替换的内置默认版本的可用性检查，并在您配置的模型上启动，不显示回退通知。

模型别名（如 `opus`）不充当固定版本，Claude Code 无法识别的模型 ID（如应用推理配置文件 ARN）也不充当固定版本。

当这些检查发现您的账户无法调用的模型时，Claude Code 会在这台机器上记住该拒绝长达一天，在此期间启动时会跳过记住的模型，而不再询问 Amazon Bedrock。Claude Code 会在距离上次检查已过十分钟后再次检查当前默认模型的记住拒绝，因此您的管理员重新启用的默认模型会恢复。要关闭此记忆功能，请设置 [`CLAUDE_CODE_SKIP_MODEL_ACCESS_MEMORY=1`](https://code.claude.com/docs/zh-CN/env-vars)。

### 当您的组织强制执行模型允许列表时

如果您在托管设置中设置了 [`enforceAvailableModels`](https://code.claude.com/docs/zh-CN/model-config#enforce-the-allowlist-for-the-default-model)，启动模型检查将仅使用您的 `availableModels` 列表允许的模型。这适用于 Amazon Bedrock Invoke API，并且需要 Claude Code v2.1.287 或更高版本。未设置 `enforceAvailableModels` 的列表不会限制这些检查。

检查会将每个条目与其将要发送的推理配置文件 ID（包括其[区域前缀](#cross-region-inference-profile-prefixes)）进行比较，因此请使用这些 ID 编写列表。此示例为模型解析为 `us.` 配置文件的部署允许 Opus 4.8 和 Sonnet 4.5：

```json theme={null}
{
  "availableModels": ["us.anthropic.claude-opus-4-8", "us.anthropic.claude-sonnet-4-5-20250929-v1:0"],
  "enforceAvailableModels": true
}
```

有关别名、版本前缀和 `modelOverrides` 条目，请参阅[为第三方部署固定模型](https://code.claude.com/docs/zh-CN/model-config.md#pin-models-for-third-party-deployments)。

### 当模型在会话中被禁用时

如果您的账户失去对会话正在运行的模型的访问权限，例如因为管理员在您的 Amazon Bedrock 账户中禁用了它，Claude Code 会将会话切换到另一个模型，而不是使每个请求都失败，并显示 `Switched to <fallback> because <model> is not available`。它尝试与启动回退相同的模型：首先尝试同一层级的早期版本，对于没有可用 Opus 版本的 Opus 会话，则使用默认 Sonnet 模型。

切换仅适用于您未固定的层级，这与启动回退的条件相同。在您选择的特定版本上的会话，或在[应用推理配置文件 ARN](#map-each-model-version-to-an-inference-profile) 上的会话，会保持其模型；如果没有备用模型链，请求会改为失败。在[自动模式](https://code.claude.com/docs/zh-CN/permission-modes.md#enable-auto-mode-on-bedrock-agent-platform-or-foundry)中，Claude Code 仅切换到自动模式在 Amazon Bedrock 上支持的模型。如果这些模型中也没有可用的，请求会失败并显示 [AWS 身份验证失败](https://code.claude.com/docs/zh-CN/errors.md#aws-authentication-failed)，并提示启用该模型。

您配置的[备用模型链](https://code.claude.com/docs/zh-CN/model-config.md#fallback-model-chains)会替换层级切换：在这些拒绝上，Claude Code 会切换到您配置的备用模型。要使被拒绝的请求失败而不是切换，请设置 [`CLAUDE_CODE_DISABLE_MODEL_ACCESS_FALLBACK=1`](https://code.claude.com/docs/zh-CN/env-vars)。您配置的备用链仍会在这些拒绝上切换；如果您希望每个被拒绝的请求都失败，也请移除该链。

## 跨区域推理配置文件前缀

在 Amazon Bedrock [Invoke API](https://docs.aws.amazon.com/bedrock/latest/APIReference/API_runtime_InvokeModelWithResponseStream.html) 上，Claude Code 将其内置默认模型解析为[跨区域推理配置文件](https://docs.aws.amazon.com/bedrock/latest/userguide/inference-profiles-support.html) ID；要通过您自己的推理配置文件路由模型版本，请参阅[将每个模型版本映射到推理配置文件](#map-each-model-version-to-an-inference-profile)。此表显示了 Claude Code 对每个已解析的 AWS 区域首选的前缀：

| AWS 区域 | 前缀 |
| :- | :- |
| `us-gov-*` (AWS GovCloud) | `us-gov.` |
| `us-*` | `us.` |
| `eu-*` | `eu.` |
| `ap-*` | `apac.` |
| 所有其他区域 | `global.` |

设置 `ANTHROPIC_BEDROCK_REGION_PREFIX` 以选择 Claude Code 首先尝试的前缀；当 Claude Code 可以检查配置文件可用性并且找不到模型的匹配配置文件时，它会按照下面的解析顺序进行回退。有效值为 `us`、`eu`、`apac`、`jp`、`au` 和 `global`。例如，当您的账户启用了 `global.` 配置文件但 Claude Code 会从您的 AWS 区域派生地理位置特定的配置文件时，将其设置为 `global`。需要 Claude Code v2.1.224 或更高版本。

此示例通过 `global.` 配置文件路由默认模型：

```bash theme={null}
export ANTHROPIC_BEDROCK_REGION_PREFIX=global
# 在 us-* 区域中，主模型现在解析为
# global.anthropic.claude-opus-5-5 而不是 us.anthropic.claude-opus-5-5
```

首选前缀是一个偏好，而不是保证，无论它来自您的区域还是来自变量。Claude Code 如何应用它取决于它是否可以检查您账户中的配置文件可用性：

* 当 Claude Code 可以[列出您账户中的推理配置文件](#iam-configuration)时，它按以下顺序解析每个模型：
  1. 具有您首选前缀的配置文件。
  2. 任何匹配的配置文件，用于没有该前缀的配置文件的模型。
  3. 具有您首选前缀的内置模型 ID，用于根本没有匹配配置文件的模型。Claude Code 在此步骤中应用此 ID 而不检查可用性；[启动模型检查](#startup-model-checks)仍然涵盖会话的默认模型。
* 当配置文件发现不可用时，Claude Code 应用前缀而不检查可用性。如果您的账户没有启用该前缀的推理配置文件，请求将失败并显示 400 错误。

Claude Code 不会重写您自己配置的 Amazon Bedrock 推理配置文件 ID 或 ARN，或 [`modelOverrides`](#map-each-model-version-to-an-inference-profile) 值；Anthropic 格式的模型 ID 通过[与 `/model` 选择器相同的映射](#map-each-model-version-to-an-inference-profile)解析。Claude Code 在两种情况下也会忽略该变量：

* 在 AWS GovCloud 区域中，Claude Code 始终使用 `us-gov.`，这是在 GovCloud 分区内路由的唯一前缀。
* 当您设置的值不是有效值之一时，Claude Code 会回退到区域派生的首选前缀。

## IAM 配置

创建具有 Claude Code 所需权限的 IAM 策略：

```json theme={null}
{
  "Version": "2012-10-17",
  "Statement": [
    {
      "Sid": "AllowModelAndInferenceProfileAccess",
      "Effect": "Allow",
      "Action": [
        "bedrock:InvokeModel",
        "bedrock:InvokeModelWithResponseStream",
        "bedrock:ListInferenceProfiles",
        "bedrock:GetInferenceProfile"
      ],
      "Resource": [
        "arn:aws:bedrock:*:*:inference-profile/*",
        "arn:aws:bedrock:*:*:application-inference-profile/*",
        "arn:aws:bedrock:*:*:foundation-model/*"
      ]
    },
    {
      "Sid": "AllowMarketplaceSubscription",
      "Effect": "Allow",
      "Action": [
        "aws-marketplace:ViewSubscriptions",
        "aws-marketplace:Subscribe"
      ],
      "Resource": "*",
      "Condition": {
        "StringEquals": {
          "aws:CalledViaLast": "bedrock.amazonaws.com"
        }
      }
    }
  ]
}
```

为了获得更严格的权限，您可以将资源限制为特定的推理配置文件 ARN。

`bedrock:GetInferenceProfile` 让 Claude Code 能够将[应用推理配置文件 ARN](#map-each-model-version-to-an-inference-profile) 解析为其支持的基础模型，该模型用于为该模型选择正确的请求形状。

如果令牌缺少此权限，Claude Code 会通过使用备用形状重试一次来自动恢复，因此请求仍然会成功，但每个新模型都会增加一个额外的往返。授予该权限可以避免重试。这最常适用于 `AWS_BEARER_TOKEN_BEDROCK` 部署，其中令牌的策略通常比完整的 IAM 角色更窄。

有关详情，请参阅 [Amazon Bedrock IAM 文档](https://docs.aws.amazon.com/bedrock/latest/userguide/security-iam.html)。

> [!NOTE]
>
> 为 Claude Code 创建一个专用的 AWS 账户，以简化成本跟踪和访问控制。

## 1M 令牌上下文窗口

Fable 模型、Sonnet 5 及更高版本，以及 Opus 4.7 及更高版本在 Amazon Bedrock 上默认以 [1M token 上下文窗口](https://platform.claude.com/docs/en/build-with-claude/context-windows#context-window-sizes-by-model)运行，Invoke API 和 [Mantle 端点](#use-the-mantle-endpoint)均是如此，无需 `[1m]` 后缀。当 [`modelOverrides`](#map-each-model-version-to-an-inference-profile) 条目将其模型映射到应用程序推理配置文件 ARN 时，该 ARN 将获得 1M 窗口。如需改为保留 200K 窗口，请设置 [`CLAUDE_CODE_DISABLE_1M_CONTEXT=1`](https://code.claude.com/docs/zh-CN/model-config#turn-off-1m-context)。

在 Invoke API 上，当您选择 Opus 4.6 和 Sonnet 4.6 的 `[1m]` 变体时，它们将使用 1M 窗口。[设置向导](#sign-in-with-bedrock)在固定模型时提供 1M 上下文选项。如需改为为手动固定的模型启用它，请在模型 ID 后附加 `[1m]`。请参阅[为第三方部署固定模型](https://code.claude.com/docs/zh-CN/model-config.md#pin-models-for-third-party-deployments)了解详情，包括如何在不更改固定的情况下使用 1M 窗口。

在 v2.1.287 之前，Fable 模型以及 Opus 4.7 及更高版本在 Invoke API 上默认以 200K 窗口运行，需通过 `[1m]` 后缀才能在其上使用 1M 窗口。

## 服务层级

[Amazon Bedrock 服务层级](https://docs.aws.amazon.com/bedrock/latest/userguide/service-tiers-inference.html)让您在成本和延迟之间进行权衡。将 `ANTHROPIC_BEDROCK_SERVICE_TIER` 设置为 `default`、`flex` 或 `priority`：

```bash theme={null}
export ANTHROPIC_BEDROCK_SERVICE_TIER=priority
```

Claude Code 在每个请求上将此作为 `X-Amzn-Bedrock-Service-Tier` 标头发送。层级可用性因模型和区域而异。预留容量使用[预配吞吐量](https://docs.aws.amazon.com/bedrock/latest/userguide/prov-throughput.html) ARN 作为模型 ID，而不是此设置。

## AWS Guardrails

[Amazon Bedrock Guardrails](https://docs.aws.amazon.com/bedrock/latest/userguide/guardrails.html) 让您为 Claude Code 实现内容过滤。在 [Amazon Bedrock 控制台](https://console.aws.amazon.com/bedrock/)中创建 Guardrail，发布一个版本，然后将 Guardrail 标头添加到您的[设置文件](https://code.claude.com/docs/zh-CN/settings.md)。如果您使用跨区域推理配置文件，请在您的 Guardrail 上启用跨区域推理。

示例配置：

```json theme={null}
{
  "env": {
    "ANTHROPIC_CUSTOM_HEADERS": "X-Amzn-Bedrock-GuardrailIdentifier: your-guardrail-id\nX-Amzn-Bedrock-GuardrailVersion: 1"
  }
}
```

如果您的组织通过 [Claude apps gateway](https://code.claude.com/docs/zh-CN/claude-apps-gateway) 策略传递 guardrail 标头，它们将被视为[需要批准的设置](https://code.claude.com/docs/zh-CN/server-managed-settings.md#environment-variables-and-the-approval-dialog)。

当 guardrail 在中途阻止响应时，已流式传输的文本会保留，并且回复将以 guardrail 上为被阻止响应配置的消息结尾。

## 使用 Mantle 端点

Mantle 是一个 Amazon Bedrock 端点，通过原生 Anthropic API 形状而不是 Amazon Bedrock Invoke API 提供 Claude 模型。它使用相同的 [AWS 凭证](#2-configure-aws-credentials) 和 [`awsAuthRefresh` 配置](#advanced-credential-configuration)。

Mantle 在 `bedrock-mantle:` 前缀下有自己的 IAM 操作，因此 [IAM 配置](#iam-configuration) 中的 `bedrock:` 操作不涵盖它。为推理授予您的 IAM 身份 `bedrock-mantle:CreateInference`，为令牌计数授予 `bedrock-mantle:CountTokens`。请参阅 AWS 文档中的[进行推理请求](https://docs.aws.amazon.com/bedrock/latest/userguide/inference.html)和[计数令牌](https://docs.aws.amazon.com/bedrock/latest/userguide/count-tokens.html)，以及[服务授权参考](https://docs.aws.amazon.com/service-authorization/latest/reference/list_amazonbedrockpoweredbyawsmantle.html)了解每个 Mantle 操作。

### 启用 Mantle

配置了 AWS 凭证后，设置 `CLAUDE_CODE_USE_MANTLE` 以将请求路由到 Mantle 端点：

```bash theme={null}
export CLAUDE_CODE_USE_MANTLE=1
export AWS_REGION=us-east-1
```

Claude Code 从 AWS 区域构造端点 URL，使用与[上面的 Amazon Bedrock](#3-configure-claude-code) 相同的优先级解析。要为自定义端点或网关覆盖 URL，请设置 `ANTHROPIC_BEDROCK_MANTLE_BASE_URL`。

在 Claude Code 内运行 `/status` 来确认。当 Mantle 处于活动状态时，提供者行显示 `Amazon Bedrock (Mantle)`。

### 选择 Mantle 模型

Mantle 使用以 `anthropic.` 为前缀且没有版本后缀的模型 ID，例如 `anthropic.claude-sonnet-5` 或 `anthropic.claude-haiku-4-5`。您的账户可用的模型取决于您的组织被授予的内容；其他模型 ID 列在您来自 AWS 的入职材料中。联系您的 AWS 账户团队以请求访问允许列表中的模型。

使用 `--model` 标志或在 Claude Code 内使用 `/model` 设置模型：

```bash theme={null}
claude --model anthropic.claude-haiku-4-5
```

### 在 Invoke API 旁边运行 Mantle

您在 Mantle 上可用的模型可能不包括您今天使用的每个模型。设置 `CLAUDE_CODE_USE_BEDROCK` 和 `CLAUDE_CODE_USE_MANTLE` 让 Claude Code 从同一会话调用两个端点。与 Mantle 格式匹配的模型 ID 被路由到 Mantle，所有其他模型 ID 转到 Amazon Bedrock Invoke API。

```bash theme={null}
export CLAUDE_CODE_USE_BEDROCK=1
export CLAUDE_CODE_USE_MANTLE=1
```

要在 `/model` 选择器中显示 Mantle 模型，请在您的[设置文件](https://code.claude.com/docs/zh-CN/settings.md)中的 `availableModels` 中列出其 ID。此设置也将选择器限制为列出的条目。列出 `anthropic.claude-haiku-4-5` 会从选择器中移除裸 `haiku` 别名，因此也要列出版本前缀或您想保持可选择的版本的完整 ID。Mantle ID 和 `haiku` 别名解析为相同的模型族，因此合并仅保留更具体的条目。请参阅[合并行为](https://code.claude.com/docs/zh-CN/model-config.md#merge-behavior)：

```json theme={null}
{
  "availableModels": ["opus", "sonnet", "claude-haiku-4-5", "anthropic.claude-haiku-4-5"]
}
```

带有 `anthropic.` 前缀的条目被添加为自定义选择器选项，其中与 Mantle 格式匹配的条目被路由到 Mantle。将 `anthropic.claude-haiku-4-5` 替换为您的账户被授予的模型 ID。请参阅[限制模型选择](https://code.claude.com/docs/zh-CN/model-config.md#restrict-model-selection)了解 `availableModels` 如何与其他模型设置交互。

当两个提供商都处于活动状态时，`/status` 显示 `Amazon Bedrock + Amazon Bedrock (Mantle)`。

### 通过网关路由 Mantle

如果您的组织通过集中式 [LLM 网关](https://code.claude.com/docs/zh-CN/llm-gateway)路由模型流量，该网关在服务器端注入 AWS 凭证，请禁用客户端身份验证，以便 Claude Code 发送没有 SigV4 签名或 `x-api-key` 标头的请求：

```bash theme={null}
export CLAUDE_CODE_USE_MANTLE=1
export CLAUDE_CODE_SKIP_MANTLE_AUTH=1
export ANTHROPIC_BEDROCK_MANTLE_BASE_URL=https://your-gateway.example.com
```

### Mantle 环境变量

这些变量特定于 Mantle 端点。请参阅[环境变量](https://code.claude.com/docs/zh-CN/env-vars.md)了解完整列表。

| 变量 | 目的 |
| :- | :- |
| `CLAUDE_CODE_USE_MANTLE` | 启用 Mantle 端点。设置为 `1` 或 `true`。 |
| `ANTHROPIC_BEDROCK_MANTLE_BASE_URL` | 覆盖默认 Mantle 端点 URL |
| `CLAUDE_CODE_SKIP_MANTLE_AUTH` | 跳过客户端身份验证以用于代理设置 |
| `ANTHROPIC_SMALL_FAST_MODEL_AWS_REGION` | 覆盖 Haiku 类模型的 AWS 区域（与 Amazon Bedrock 共享） |

## 故障排除

### 使用 SSO 和企业代理的身份验证循环

如果在使用 AWS SSO 时浏览器登录标签页不断打开，请从您的[设置文件](https://code.claude.com/docs/zh-CN/settings.md)中删除 `awsAuthRefresh` 设置。

当企业 VPN 或 TLS 检查代理中断 SSO 浏览器流程时，可能会出现此循环。Claude Code 将中断的连接视为身份验证失败。当后续请求发现凭据仍处于过期状态时，Claude Code 会重新运行 `awsAuthRefresh`，从而打开另一个标签页。

如果您的网络环境干扰自动基于浏览器的 SSO 流，请在启动 Claude Code 之前手动使用 `aws sso login`，而不是依赖 `awsAuthRefresh`。

### TLS 检查代理后的证书错误

Claude Code 将您的[CA 证书存储](https://code.claude.com/docs/zh-CN/network-config.md#ca-certificate-store)配置应用于其对 AWS 的请求，包括：

* 模型发现
* 令牌计数
* 解析您的 AWS 凭证的 STS 和 SSO 角色凭证调用
* [设置向导](#sign-in-with-bedrock)的凭证验证和模型检查

对于这些请求，您的 OS 信任存储或 `NODE_EXTRA_CA_CERTS` 包中的企业根证书不需要特定于 Amazon Bedrock 的设置。

在 v2.1.260 之前，Claude Code 仅在这些请求通过配置的代理时才将您的 CA 配置应用于这些请求，在直接连接上，它们仅信任运行时的默认证书存储。

在 v2.1.261 之前，使用**使用我的环境中已有的凭证**选项的设置向导的模型检查后的凭证查找仍然仅信任运行时的默认证书存储。在 TLS 检查代理后面，其根证书仅在 OS 存储中，受影响的请求失败，显示 `unable to get local issuer certificate`，或向导将模型显示为 `unreachable`，而推理请求成功。更新到 v2.1.261 或更高版本。

### 区域问题

如果您遇到区域问题：

* 检查模型可用性：`aws bedrock list-inference-profiles --region your-region`
* 切换到支持的区域：`export AWS_REGION=us-east-1`
* 考虑使用推理配置文件进行跨区域访问

如果您收到错误"不支持按需吞吐量"：

* 将模型指定为[推理配置文件](https://docs.aws.amazon.com/bedrock/latest/userguide/inference-profiles-support.html) ID

Claude Code 使用 Amazon Bedrock [Invoke API](https://docs.aws.amazon.com/bedrock/latest/APIReference/API_runtime_InvokeModelWithResponseStream.html)，不支持 Converse API。

### 网关或代理后的流式传输错误

Amazon Bedrock 以二进制事件流格式流式传输 `InvokeModelWithResponseStream` 响应，标头为 `Content-Type: application/vnd.amazon.eventstream`。Claude Code 和 Amazon Bedrock 之间的网关或代理必须转发响应正文及其标头，包括 `Content-Type`，就像 Amazon Bedrock 发送的那样。

如果网关将 `Content-Type` 重写为另一个值，Claude Code 会拒绝响应，错误以 `Bedrock streaming response has content-type` 开头，命名它收到的值。常见的重写是 `text/event-stream`，来自将流重新发出为服务器发送事件的集成。有关错误消息中提到的 `CLAUDE_CODE_DISABLE_BEDROCK_CONTENT_TYPE_GUARD` 变量，请参阅 [Bedrock streaming response has an unexpected content-type](https://code.claude.com/docs/zh-CN/errors.md#bedrock-streaming-response-has-an-unexpected-content-type)。

如果网关删除或清空标头，Claude Code 会假设正文是 Amazon Bedrock 的事件流并对其进行解码，因此网关未修改地通过的正文会继续流式传输。

如果删除标头的网关也将流重新发出为服务器发送事件，Claude Code 无法解码正文，并在每个回合上回退到较慢的非流式传输路径：每个响应仅在完成后才出现，而不是流式传输。在这种情况下，设置 [`CLAUDE_CODE_DISABLE_BEDROCK_CONTENT_TYPE_DEFAULT=1`](https://code.claude.com/docs/zh-CN/env-vars)，以便 Claude Code 将正文读取为服务器发送事件。

要修复错误或回退，请配置网关以未修改地转发 `InvokeModelWithResponseStream` 响应正文及其 `Content-Type` 标头。

将流转换为服务器发送事件的网关不再提供 Amazon Bedrock API。如果它也接受 Anthropic Messages API 请求，请使用 `ANTHROPIC_BASE_URL` 而不是 `CLAUDE_CODE_USE_BEDROCK` 将其作为 [LLM 网关](https://code.claude.com/docs/zh-CN/llm-gateway-connect.md)连接。

### /context 中的零令牌计数

`/context` 命令通过将工具架构发送到 Amazon Bedrock count-tokens API 来计算每个工具组的令牌。在 Claude Code v2.1.196 之前的版本中，Amazon Bedrock 拒绝了该请求，因为架构包含其 count-tokens API 不接受的字段，因此每个工具组显示 0 个令牌。分解中的其他行（如消息和内存文件）不受影响。

更新到 v2.1.196 或更高版本。

### Mantle 端点错误

如果在设置 `CLAUDE_CODE_USE_MANTLE` 后 `/status` 没有显示 `Amazon Bedrock (Mantle)`，则该变量没有到达进程。确认它在您启动 `claude` 的 shell 中被导出，或在您的[设置文件](https://code.claude.com/docs/zh-CN/settings.md)的 `env` 块中设置它。

来自 Mantle 端点的 `403` 的含义取决于错误是否命名了 IAM 操作：

* 如果错误命名了 `bedrock-mantle:` 操作，请为您的 IAM 身份授予该操作。
* 如果错误没有命名任何操作且您的凭证有效，您的 AWS 账户没有被授予访问您请求的模型的权限。联系您的 AWS 账户团队以请求访问。

命名模型 ID 的 `400` 意味着该模型不在 Mantle 上提供。Mantle 有其自己的模型阵容，与标准 Amazon Bedrock 目录分开，因此推理配置文件 ID（如 `us.anthropic.claude-sonnet-4-6`）将不起作用。使用 Mantle 格式的 ID，或启用[两个端点](#run-mantle-alongside-the-invoke-api)，以便 Claude Code 将每个请求路由到模型可用的端点。

## 其他资源

* [Amazon Bedrock 文档](https://docs.aws.amazon.com/bedrock/)
* [Amazon Bedrock 定价](https://aws.amazon.com/bedrock/pricing/)
* [Amazon Bedrock 推理配置文件](https://docs.aws.amazon.com/bedrock/latest/userguide/inference-profiles-support.html)
* [Amazon Bedrock 令牌消耗和配额](https://docs.aws.amazon.com/bedrock/latest/userguide/quotas-token-burndown.html)
* [Amazon Bedrock 上的 Claude Code：快速设置指南](https://builder.aws.com/content/2tXkZKrZzlrlu0KfH8gST5Dkppq/claude-code-on-amazon-bedrock-quick-setup-guide)
* [Claude Code 监控实现 (Amazon Bedrock)](https://github.com/aws-solutions-library-samples/guidance-for-claude-code-with-amazon-bedrock/blob/main/assets/docs/MONITORING.md)
