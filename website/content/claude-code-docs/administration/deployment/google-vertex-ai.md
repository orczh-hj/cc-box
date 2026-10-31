> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Google Cloud 的 Agent Platform 上的 Claude Code

> 了解如何通过 Google Cloud 的 Agent Platform（原 Vertex AI）配置 Claude Code，包括设置、IAM 配置和故障排除。

**Deploying Claude Code across your organization?** Talk to sales about enterprise plans, SSO, and centralized billing.

View plans
Contact sales

## 前置条件

在使用 Google Cloud 的 Agent Platform（原 Vertex AI）配置 Claude Code 之前，请确保您拥有：

* 启用了计费的 Google Cloud Platform (GCP) 账户
* 启用了 Google Cloud 的 Agent Platform API 的 GCP 项目
* 对所需 Claude 模型的访问权限（例如，Claude Sonnet 4.6）
* 已安装并配置的 Google Cloud SDK (`gcloud`)
* 在所需 GCP 区域中分配的配额

要使用您自己的 Google Cloud 的 Agent Platform 凭证登录，请按照下面的[使用 Google Cloud 的 Agent Platform 登录](#sign-in-with-agent-platform)进行操作。要在团队中部署 Claude Code，请使用[手动设置](#set-up-manually)步骤并在推出前[固定您的模型版本](#5-pin-model-versions)。

## 使用 Agent Platform 登录

如果您拥有 Google Cloud 凭证并想开始通过 Google Cloud 的 Agent Platform 使用 Claude Code，登录向导会引导您完成整个过程。您需要在每个项目中完成一次 GCP 端的前置条件；向导会处理 Claude Code 端的事务。

**1. 在您的 GCP 项目中启用 Claude 模型**

为您的项目[启用 Google Cloud 的 Agent Platform API](#1-enable-agent-platform-api)，然后在 [Google Cloud 的 Agent Platform Model Garden](https://console.cloud.google.com/vertex-ai/model-garden) 中请求访问您想要的 Claude 模型。有关您的账户需要的权限，请参阅 [IAM 配置](#iam-configuration)。

**2. 启动 Claude Code 并选择 Google Cloud 的 Agent Platform**

运行 `claude`。在登录提示处，选择 **3rd-party platform**，然后选择 **Google Vertex AI**，这是登录提示仍然用于 Google Cloud 的 Agent Platform 的标签。如果您已经登录，运行 `/login` 来打开相同的菜单。

**3. 按照向导提示进行操作**

选择您如何向 Google Cloud 进行身份验证：来自 `gcloud` 的应用默认凭证、服务账户密钥文件或已在您的环境中的凭证。向导会检测您的项目和区域，验证您的项目可以调用哪些 Claude 模型，并让您固定它们。它将结果保存到您的[用户设置文件](https://code.claude.com/docs/zh-CN/settings.md)的 `env` 块中，因此您无需自己导出环境变量。

登录后，您可以随时运行 `/setup-vertex` 来重新打开向导并更改您的凭证、项目、区域或模型固定。模型固定步骤从您当前固定的模型开始。向导会写入 `~/.claude/settings.json`，或在设置了 [`CLAUDE_CONFIG_DIR`](https://code.claude.com/docs/zh-CN/env-vars#variables) 时写入 `$CLAUDE_CONFIG_DIR/settings.json`。

## 区域配置

Claude Code 支持 Google Cloud 的 Agent Platform [全局](https://cloud.google.com/blog/products/ai-machine-learning/global-endpoint-for-claude-models-generally-available-on-vertex-ai)、多区域和区域端点。将 `CLOUD_ML_REGION` 设置为 `global`、多区域位置（如 `eu` 或 `us`）或特定区域（如 `us-east5`）。Claude Code 为每种形式选择正确的 Google Cloud 的 Agent Platform 主机名，包括多区域位置的 `aiplatform.eu.rep.googleapis.com` 和 `aiplatform.us.rep.googleapis.com` 主机。

> [!NOTE]
>
> Google Cloud 的 Agent Platform 可能不支持 Claude Code 默认模型在每个端点类型上。模型可用性在[特定区域](https://cloud.google.com/vertex-ai/generative-ai/docs/learn/locations#genai-partner-models)、多区域位置和[全局端点](https://cloud.google.com/vertex-ai/generative-ai/docs/partner-models/use-partner-models#supported_models)之间有所不同。您可能需要切换到支持的位置或指定支持的模型。

## 手动设置

要通过环境变量而不是向导配置 Google Cloud 的 Agent Platform，例如在 CI 或脚本化企业推出中，请按照下面的步骤进行。

### 启用 Agent Platform API

在您的 GCP 项目中启用 Google Cloud 的 Agent Platform API。将 `YOUR-PROJECT-ID` 替换为您的 GCP 项目 ID，并在下面的配置步骤中使用：

```bash theme={null}
# 设置您的项目 ID
gcloud config set project YOUR-PROJECT-ID

# 启用 Agent Platform API
gcloud services enable aiplatform.googleapis.com
```

### 请求模型访问权限

请求访问 Google Cloud 的 Agent Platform 中的 Claude 模型：

1. 导航到 [Google Cloud 的 Agent Platform Model Garden](https://console.cloud.google.com/vertex-ai/model-garden)
2. 搜索"Claude"模型
3. 请求访问所需的 Claude 模型（例如，Claude Sonnet 4.6）
4. 等待批准（可能需要 24-48 小时）

### 配置 GCP 凭证

Claude Code 使用标准的 Google Cloud 身份验证。

有关更多信息，请参阅 [Google Cloud 身份验证文档](https://cloud.google.com/docs/authentication)。

Claude Code 通过相同的应用默认凭证链支持[基于 X.509 证书的工作负载身份联合](https://cloud.google.com/iam/docs/workload-identity-federation-with-x509-certificates)。将 `GOOGLE_APPLICATION_CREDENTIALS` 设置为您的凭证配置文件的路径。

> [!NOTE]
>
> Claude Code 将 Google Cloud 的 Agent Platform 请求寻址到 `ANTHROPIC_VERTEX_PROJECT_ID` 中的项目，即使 `GCLOUD_PROJECT`、`GOOGLE_CLOUD_PROJECT` 或 `GOOGLE_APPLICATION_CREDENTIALS` 引用的凭证文件包含不同的项目。

#### 高级凭证配置

Claude Code 通过 `gcpAuthRefresh` 设置支持 GCP 的自动凭证刷新。将其添加到您的 Claude Code [设置文件](https://code.claude.com/docs/zh-CN/settings.md)，例如 `~/.claude/settings.json`。当 Claude Code 检测到您的 GCP 凭证已过期或无法加载时，它会运行配置的命令以在重试请求之前获取新凭证。

```json theme={null}
{
  "gcpAuthRefresh": "gcloud auth application-default login",
  "env": {
    "ANTHROPIC_VERTEX_PROJECT_ID": "your-project-id"
  }
}
```

在运行命令之前，Claude Code 使用您当前的凭证请求访问令牌以确认它们确实已过期，并在它们仍然有效时跳过该命令。

如果检查在五秒内未完成，Claude Code 也会跳过该命令，仅在请求因凭证错误而失败后运行它。在 v2.1.261 之前，超时的检查被视为过期的凭证，因此即使您的凭证仍然有效，该命令也可能在启动时打开您的浏览器。

Claude Code 向您显示命令的输出，但无法向命令发送交互式输入。这对于基于浏览器的身份验证流程效果很好，其中 CLI 显示 URL，您在浏览器中完成身份验证。如果身份验证未在三分钟内完成，刷新命令将超时。如果您在项目设置（如 `.claude/settings.json`）中设置 `gcpAuthRefresh`，Claude Code 会在与[设置文件中的 hooks 相同的工作区信任规则](https://code.claude.com/docs/zh-CN/permissions.md#what-runs-before-you-trust-a-folder)下运行它，其中包括您从未信任的文件夹中的 `-p` 会话。

### 配置 Claude Code

设置以下环境变量：

```bash theme={null}
# 启用 Agent Platform 集成
export CLAUDE_CODE_USE_VERTEX=1
export CLOUD_ML_REGION=global
export ANTHROPIC_VERTEX_PROJECT_ID=YOUR-PROJECT-ID

# 可选：为自定义端点或网关覆盖 Agent Platform 端点 URL
# export ANTHROPIC_VERTEX_BASE_URL=https://aiplatform.googleapis.com

# 当 CLOUD_ML_REGION=global 时，为不支持全局端点的模型覆盖区域
export VERTEX_REGION_CLAUDE_HAIKU_4_5=us-east5
export VERTEX_REGION_CLAUDE_4_6_SONNET=europe-west1
```

大多数模型版本都有对应的 `VERTEX_REGION_CLAUDE_*` 变量。有关完整列表，请参阅[环境变量参考](https://code.claude.com/docs/zh-CN/env-vars.md#variables)。检查 [Google Cloud 的 Agent Platform Model Garden](https://console.cloud.google.com/vertex-ai/model-garden) 以确定哪些模型支持全局端点与仅区域端点。

如果区域值的形状不像区域或位置名称，Claude Code 会将其视为未设置。例如，Claude Code 将包含斜杠、点或空格的值视为未设置。Claude Code 为每个变量回退到不同的源：

* `VERTEX_REGION_CLAUDE_*`：Claude Code 回退到 `CLOUD_ML_REGION`。
* `CLOUD_ML_REGION`：Claude Code 回退到 `us-east5`。

[Prompt caching](https://code.claude.com/docs/zh-CN/prompt-caching.md) 会自动启用。要禁用它，请设置 `DISABLE_PROMPT_CACHING=1`。要请求 1 小时的缓存 TTL 而不是 5 分钟的默认值，请设置 `ENABLE_PROMPT_CACHING_1H=1`；具有 1 小时 TTL 的缓存写入按更高费率计费。要为您的主对话和 Claude Code 在其外部进行的请求设置不同的 TTL，请[自己选择 TTL](https://code.claude.com/docs/zh-CN/prompt-caching.md#choose-the-ttl-yourself)。

要提高您的速率限制，请联系 Google Cloud 支持。使用 Google Cloud 的 Agent Platform 时，`/logout` 命令不可用，因为身份验证通过 Google Cloud 凭证处理。

Claude Code 在 [MCP tool search](https://code.claude.com/docs/zh-CN/mcp.md#scale-with-mcp-tool-search) 和预先加载之间进行决策，具体取决于模型生成：

* **Claude Opus 4.5、Sonnet 4.5、Haiku 4.5 及更高版本**：Claude Code 默认启用工具搜索。
* **早期模型**：Claude Code 预先加载 MCP 工具定义，因为它们的 Agent Platform 服务堆栈拒绝所需的 beta 标头。设置 `ENABLE_TOOL_SEARCH=true` 不会覆盖此行为。

设置 `ENABLE_TOOL_SEARCH=false` 以在每个模型上禁用工具搜索。在 v2.1.221 之前，Claude Code 在 Google Cloud 的 Agent Platform 上为所有模型禁用工具搜索，除非您设置 `ENABLE_TOOL_SEARCH=true`。

### 固定模型版本

> [!WARNING]
>
> 在部署到多个用户时固定特定的模型版本。如果不固定，模型别名（如 `sonnet` 和 `opus`）会解析为 Claude Code 的 Google Cloud 的 Agent Platform 内置默认值，该默认值可能滞后于最新版本，并且可能尚未在您的项目中启用。Claude Code 在启动时当默认值不可用时会[回退](#startup-model-checks)到之前的版本或更低级别的模型，但固定让您可以控制用户何时迁移到新模型。

将这些环境变量设置为特定的 Google Cloud 的 Agent Platform 模型 ID。

如果没有 `ANTHROPIC_DEFAULT_OPUS_MODEL`，Google Cloud 的 Agent Platform 上的 `opus` 别名会解析为 Opus 5.5，如果没有 `ANTHROPIC_DEFAULT_SONNET_MODEL`，`sonnet` 别名会解析为 Sonnet 4.5。此示例将每个别名固定到特定版本：

```bash theme={null}
export ANTHROPIC_DEFAULT_OPUS_MODEL='claude-opus-4-8'
export ANTHROPIC_DEFAULT_SONNET_MODEL='claude-sonnet-5'
export ANTHROPIC_DEFAULT_HAIKU_MODEL='claude-haiku-4-5@20251001'
```

有关当前和旧版模型 ID，请参阅[模型概览](https://platform.claude.com/docs/en/about-claude/models/overview)。有关完整的环境变量列表，请参阅[模型配置](https://code.claude.com/docs/zh-CN/model-config.md#pin-models-for-third-party-deployments)。

Claude Code 在未设置固定变量时使用这些默认模型：

| 模型类型 | 默认值 |
| :- | :- |
| 主模型 | `claude-opus-5-5` |
| 小型/快速模型 | `claude-sonnet-4-5@20250929` |

在 v2.1.280 之前，Google Cloud 的 Agent Platform 上的主模型默认为 Opus 5，`opus` 别名从 v2.1.219 起解析为 Opus 5。

后台任务（如会话标题生成）使用小型/快速模型，通常是 Haiku 级别的模型。在 Google Cloud 的 Agent Platform 上，Claude Code 为后台任务使用默认的 Sonnet 模型，因为 Haiku 可能不会在每个项目或区域中启用。两个选择会改变哪个模型执行这些任务：

* 当您使用 `--model`、`ANTHROPIC_MODEL` 或 `model` 设置选择主模型时，后台任务使用该模型。当 Claude Code 在您使用 [`ANTHROPIC_DEFAULT_MODEL`](https://code.claude.com/docs/zh-CN/model-config#set-a-default-model-for-new-sessions) 设置的模型上启动会话时，后台任务也使用该模型。设置 `ANTHROPIC_DEFAULT_OPUS_MODEL` 而不设置 `ANTHROPIC_DEFAULT_SONNET_MODEL` 也算作一个选择，因为内置的 Sonnet 模型可能在引导自己的 Opus 的项目中未启用。
* 要为后台任务使用 Haiku，请将 `ANTHROPIC_DEFAULT_HAIKU_MODEL` 设置为在您的项目中可用的模型 ID。

> [!WARNING]
>
> Opus 模型的每 token 价格高于 Sonnet 模型，因此如果您不固定主模型，将按 Opus 费率计费。要将 Sonnet 4.5 用作主模型，请将 `ANTHROPIC_MODEL` 设置为其完整模型 ID。使用 `ANTHROPIC_DEFAULT_SONNET_MODEL` 引导默认值且不设置 `ANTHROPIC_DEFAULT_OPUS_MODEL` 的部署会保持其引导的 Sonnet 模型作为默认值。

要进一步自定义模型：

```bash theme={null}
export ANTHROPIC_MODEL='claude-opus-4-8'
export ANTHROPIC_DEFAULT_HAIKU_MODEL='claude-haiku-4-5@20251001'
```

### 验证您的配置

启动 Claude Code 并运行 `/status` 以确认设置。`API provider` 行显示 `Google Vertex AI`，`GCP project`、`Default region` 和 `Model` 行显示您的项目 ID、区域和解析的模型。如果提供程序行缺失，环境变量未到达该进程。确认它们在您启动 `claude` 的 shell 中已导出，或在您的[设置文件](https://code.claude.com/docs/zh-CN/settings.md)的 `env` 块中设置它们。

## 启动模型检查

当 Claude Code 启动并配置了 Google Cloud 的 Agent Platform 时，它会验证它打算使用的模型在您的项目中是否可访问。

如果您固定了一个比当前 Claude Code 默认值更旧的模型版本，并且您的项目可以调用较新版本，Claude Code 会提示您更新固定。接受会将新的模型 ID 写入您的[用户设置文件](https://code.claude.com/docs/zh-CN/settings.md)并重启 Claude Code。拒绝会被记住，直到下一个默认版本更改。

如果您没有固定模型，并且当前默认值在您的项目中不可用，Claude Code 会在当前会话中回退并显示通知。它首先尝试默认模型的早期版本，当默认值是 Opus 模型且没有可用的 Opus 版本时，会回退到默认 Sonnet 模型。回退不会被持久化。在 [Model Garden](https://console.cloud.google.com/vertex-ai/model-garden) 中启用较新的模型或[固定一个版本](#5-pin-model-versions)以使选择永久化。

当您在特定的 Sonnet 或 Opus 版本上启动会话时，例如使用 `--model`、`ANTHROPIC_MODEL` 或 [`model` 设置](https://code.claude.com/docs/zh-CN/settings-reference#model)，该版本充当会话的固定默认值，用于匹配的 `sonnet` 或 `opus` 别名。Claude Code 跳过对您的模型替换的内置默认值的可用性检查，并在您配置的模型上启动，没有回退通知。

模型别名（如 `opus`）不充当固定值，Claude Code 不识别的模型 ID 也不充当固定值。

当这些检查发现您的项目无法调用的模型时，Claude Code 会在这台机器上记住该拒绝长达一天，并在此期间启动时跳过记住的模型，而不再询问 Agent Platform。Claude Code 会在距离上次检查已过十分钟后，再次检查当前默认模型的记住拒绝，因此管理员重新启用的默认值会恢复。要关闭此内存，请设置 [`CLAUDE_CODE_SKIP_MODEL_ACCESS_MEMORY=1`](https://code.claude.com/docs/zh-CN/env-vars)。

### 当您的组织强制执行模型允许列表时

如果您在托管设置中设置了 [`enforceAvailableModels`](https://code.claude.com/docs/zh-CN/model-config#enforce-the-allowlist-for-the-default-model)，启动模型检查将仅使用您的 `availableModels` 列表允许的模型。这需要 Claude Code v2.1.287 或更高版本。没有 `enforceAvailableModels` 的列表不会限制这些检查。

这些检查会将每个条目与它们将发送到 Agent Platform 的模型 ID 进行比较，因此请使用这些 ID 编写列表。此示例允许 Opus 4.8 和 Sonnet 4.5：

```json theme={null}
{
  "availableModels": ["claude-opus-4-8", "claude-sonnet-4-5@20250929"],
  "enforceAvailableModels": true
}
```

有关别名、版本前缀和 `modelOverrides` 条目，请参阅[为第三方部署固定模型](https://code.claude.com/docs/zh-CN/model-config.md#pin-models-for-third-party-deployments)。

### 当模型在会话中途被禁用时

如果您的项目失去对会话正在运行的模型的访问权限，例如因为管理员在 [Model Garden](https://console.cloud.google.com/vertex-ai/model-garden) 中禁用了它，Claude Code 会将会话切换到另一个模型，而不是让每个请求都失败，并显示 `Switched to <fallback> because <model> is not available`。它尝试与启动回退相同的模型：首先尝试同一层级的早期版本，对于没有可用 Opus 版本的 Opus 会话，则使用默认 Sonnet 模型。

切换仅适用于您未固定的层级，这与启动回退的条件相同。在您选择的特定版本上的会话保持其模型，没有回退模型链，请求会失败。在 [auto mode](https://code.claude.com/docs/zh-CN/permission-modes.md#enable-auto-mode-on-bedrock-agent-platform-or-foundry) 中，Claude Code 仅切换到 auto mode 在 Agent Platform 上支持的模型。如果这些模型都不可用，请求会失败。

您配置的[回退模型链](https://code.claude.com/docs/zh-CN/model-config.md#fallback-model-chains)会替换层级切换：在这些拒绝上，Claude Code 会切换到您配置的回退。要使被拒绝的请求失败而不是切换，请设置 [`CLAUDE_CODE_DISABLE_MODEL_ACCESS_FALLBACK=1`](https://code.claude.com/docs/zh-CN/env-vars)。您配置的回退链仍会在这些拒绝上切换；如果您希望每个被拒绝的请求都失败，也要移除该链。

## IAM 配置

分配 `roles/aiplatform.user` 角色，其中包括所需的权限：

* `aiplatform.endpoints.predict` - 模型调用和令牌计数所需

对于更严格的权限，请创建仅包含上述权限的自定义角色。

有关详细信息，请参阅 [Google Cloud 的 Agent Platform IAM 文档](https://cloud.google.com/vertex-ai/docs/general/access-control)。

> [!NOTE]
>
> 为 Claude Code 创建专用的 GCP 项目，以简化成本跟踪和访问控制。

## 1M token context window

Fable 模型、Sonnet 5 及更高版本以及 Opus 4.7 及更高版本在 Google Cloud 的 Agent Platform 上默认以 [1M token 上下文窗口](https://platform.claude.com/docs/zh-CN/build-with-claude/context-windows#context-window-sizes-by-model)运行，无需 `[1m]` 后缀。如需改为保留 200K 窗口，请设置 [`CLAUDE_CODE_DISABLE_1M_CONTEXT=1`](https://code.claude.com/docs/zh-CN/model-config#turn-off-1m-context)。

Opus 4.6 和 Sonnet 4.6 在您选择其 `[1m]` 变体时可使用 1M 窗口。[设置向导](#sign-in-with-agent-platform)在固定模型时提供 1M 上下文选项。如需改为为手动固定的模型启用它，请在模型 ID 后附加 `[1m]`。有关详细信息，请参阅[为第三方部署固定模型](https://code.claude.com/docs/zh-CN/model-config.md#pin-models-for-third-party-deployments)，包括如何在不更改固定的情况下使用 1M 窗口。

在 v2.1.287 之前，Fable 模型以及 Opus 4.7 及更高版本在 Google Cloud 的 Agent Platform 上默认以 200K 窗口运行，并通过 `[1m]` 后缀使用 1M 窗口。

## 故障排除

如果您遇到"无法加载默认凭证"错误：

* 运行 `gcloud auth application-default login` 来设置应用默认凭证
* 将 `GOOGLE_APPLICATION_CREDENTIALS` 设置为服务账户密钥文件路径
* 查看 [配置 GCP 凭证](#3-configure-gcp-credentials) 了解所有选项

如果您遇到配额问题：

* 通过 [Cloud Console](https://cloud.google.com/docs/quotas/view-manage) 检查当前配额或请求增加配额

如果您遇到"模型未找到"404 错误：

* 确认模型在 [Model Garden](https://console.cloud.google.com/vertex-ai/model-garden) 中已启用
* 验证该模型在您指定的位置可用。某些模型仅在 `global` 或多区域位置（如 `eu` 和 `us`）上提供，而不是在特定区域
* 如果使用 `CLOUD_ML_REGION=global`，请检查您的模型是否在 [Model Garden](https://console.cloud.google.com/vertex-ai/model-garden) 中的"支持的功能"下支持全局端点。对于不支持全局端点的模型，请执行以下任一操作：
  * 通过 `ANTHROPIC_MODEL` 或 `ANTHROPIC_DEFAULT_HAIKU_MODEL` 指定支持的模型，或
  * 使用该模型对应的 `VERTEX_REGION_CLAUDE_*` 变量设置区域或多区域位置，这些变量列于[环境变量参考](https://code.claude.com/docs/zh-CN/env-vars.md#variables)中

如果您遇到 429 错误：

* 对于区域端点，请确保主模型和小型/快速模型在您选择的区域中受支持
* 考虑切换到 `CLOUD_ML_REGION=global` 以获得更好的可用性

## 其他资源

* [Google Cloud 的 Agent Platform 文档](https://cloud.google.com/vertex-ai/docs)
* [Google Cloud 的 Agent Platform 定价](https://cloud.google.com/vertex-ai/pricing)
* [Google Cloud 的 Agent Platform 配额和限制](https://cloud.google.com/vertex-ai/docs/quotas)
