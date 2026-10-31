> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# 为您的组织推出 LLM 网关

> 为 Claude Code 部署网关产品：配置它以转发 Claude Code 发送的内容，颁发开发者凭证，通过托管设置分发配置，并验证推出。

本页面指导管理员为 Claude Code 推出 LLM 网关。它假设您已部署了满足[网关要求](#gateway-requirements)的网关产品。本页面不涵盖部署或运营任何特定产品；请按照您的供应商文档部署您的产品。

> [!NOTE]
>
> * 要将您自己机器上的 Claude Code 连接到现有网关，请参阅[将 Claude Code 连接到 LLM 网关](https://code.claude.com/docs/zh-CN/llm-gateway-connect.md)
> * 有关 Claude Code 发送到网关的内容以及要转发的内容，请参阅[网关协议参考](https://code.claude.com/docs/zh-CN/llm-gateway-protocol.md)

## 前置条件

要完成推出，您需要：

* 在您的基础设施上部署的网关，在您将分发给开发者的确切地址上提供 HTTPS，而不是重定向到它的地址，并配置为将 Claude 模型名称路由到您的提供商
* 网关转发的提供商凭证：
  * 对于 Anthropic API：来自 [Claude 控制台](https://platform.claude.com/settings/keys)的 API 密钥
  * 对于云提供商：具有模型访问权限的云凭证。请参阅 [Amazon Bedrock](https://code.claude.com/docs/zh-CN/amazon-bedrock.md#prerequisites)、[Google Cloud 的 Agent Platform](https://code.claude.com/docs/zh-CN/google-vertex-ai.md#prerequisites) 或 [Microsoft Foundry](https://code.claude.com/docs/zh-CN/microsoft-foundry.md#prerequisites) 页面上的前置条件
* 一种向开发者机器交付设置文件的方式，例如 MDM 或配置管理
  * 如果您还没有，[设置如何到达设备](https://code.claude.com/docs/zh-CN/admin-setup.md#decide-how-settings-reach-devices)比较了各种选项

### 网关要求

无论哪种产品提供网关，它必须：

* **接受支持的 API 格式**：[API 格式表](https://code.claude.com/docs/zh-CN/llm-gateway-protocol.md#api-formats)中的格式之一。下面的推出步骤假设 Anthropic Messages API 位于 `POST /v1/messages`，大多数网关都提供此格式
* **流式传输响应**：按到达时传递服务器发送的事件，包括保活 ping，而不是缓冲整个响应；[流式传输](https://code.claude.com/docs/zh-CN/llm-gateway-protocol.md#streaming)涵盖了缓冲或剥离 ping 会破坏什么
* **路由 Claude 模型名称**：将开发者使用的每个名称映射到上游模型。Claude Code 在每个请求中发送模型名称，例如 `claude-sonnet-4-6`；在大多数网关产品中，映射是网关自己配置中的模型列表或路由表
* **转发标头和正文不变**：在两个方向上传递 `anthropic-beta`、`anthropic-version` 和请求正文；[功能传递表](https://code.claude.com/docs/zh-CN/llm-gateway-protocol.md#feature-pass-through)将每个映射到没有它就会中断的功能
* **返回未修改的上游错误**：Claude Code 的自动恢复与错误措辞匹配，因此在网关自己的信封中包装错误会破坏它，除非信封的消息包含 [Claude apps 网关为云提供商的错误措辞替换](https://code.claude.com/docs/zh-CN/claude-apps-gateway-config.md#upstream-error-messages)的 `capability_rejected:` 令牌之一
* **豁免路径免受请求正文 WAF 检查**：Claude Code 提示包含源代码和 XML 样式标签，与跨站脚本正文规则匹配；网关前面的 WAF 在真实会话中返回 `403`，而短测试请求通过

可选地，提供 `GET /v1/models` 以便 Claude Code 可以使用[模型发现](https://code.claude.com/docs/zh-CN/llm-gateway-protocol.md#model-discovery)从您的网关填充模型选择器。

## 推出步骤

推出分为五个步骤，每个步骤都有一个检查点：

1. [确认网关路由您的模型](#confirm-the-gateway-routes-your-models)
2. [为每个开发者颁发凭证](#issue-developer-credentials)
3. [针对网关测试 Claude Code](#test-claude-code-against-the-gateway)
4. [分发基础 URL 和凭证](#distribute-the-configuration)
5. [从开发者机器验证](#verify-the-rollout)

这些步骤涉及三个不同的凭证，检查点用占位符命名它们，以便您可以在出现问题时判断哪个有问题：

| 凭证 | 谁持有它 | 检查点中的占位符 |
| :- | :- | :- |
| 提供商凭证 | 网关，它将其转发给上游提供商 | 在网关上配置；从不出现在客户端命令中 |
| 网关管理凭证 | 您，如果您的网关产品为其管理或测试界面颁发一个 | `<gateway-key>` |
| 开发者密钥 | 每个开发者，由网关在[颁发开发者凭证](#issue-developer-credentials)中颁发 | `<developer-key>` |

### 确认网关路由您的模型

您的网关应该已经配置了您的提供商凭证，在其基础 URL 上侦听，并将请求转发到您的提供商的 API。使用最小请求测试路径是否端到端工作，替换来自您的部署的两个值：

* `<gateway-key>` 是任何让您现在调用网关的凭证：管理密钥、测试密钥或您自己的开发者密钥（如果您已经颁发了一个）。并非每个网关产品都有单独的管理凭证；如果您的没有，请先在[颁发开发者凭证](#issue-developer-credentials)中为自己颁发开发者密钥
* `model` 是您的网关配置为路由的 Claude 模型名称。示例使用 `claude-sonnet-4-6`；替换为您配置的名称

> [tab] Bash 或 Zsh
>
> ```bash theme={null}
> curl -X POST "https://llm-gateway.example.com/v1/messages" \
>   -H "Authorization: Bearer <gateway-key>" \
>   -H "anthropic-version: 2023-06-01" \
>   -H "content-type: application/json" \
>   -d '{"model": "claude-sonnet-4-6", "max_tokens": 1, "messages": [{"role": "user", "content": "."}]}'
> ```

> [tab] PowerShell
>
> ```powershell theme={null}
> Invoke-RestMethod -Method Post -Uri "https://llm-gateway.example.com/v1/messages" `
>   -Headers @{ "Authorization" = "Bearer <gateway-key>"; "anthropic-version" = "2023-06-01" } `
>   -ContentType "application/json" `
>   -Body '{"model": "claude-sonnet-4-6", "max_tokens": 1, "messages": [{"role": "user", "content": "."}]}'
> ```

**检查点**：带有 `content` 字段的 `200` 意味着网关以该模型名称到达了提供商。`404` 意味着该名称在网关处未路由；来自提供商的 `401` 意味着网关的提供商凭证错误。

对网关路由配置中的每个 Claude 模型名称重复请求一次。网关不路由的名称会向选择它的任何开发者返回 `404`，因此在推出前测试每个名称。

> [!NOTE]
>
> 避免在重定向后提供网关。重定向可能会在推理请求上丢弃请求正文或剥离凭证标头，[模型发现](https://code.claude.com/docs/zh-CN/llm-gateway-protocol.md#model-discovery)将任何重定向视为失败，因此凭证无法泄露到重定向目标。

### 颁发开发者凭证

每个开发者需要自己的网关密钥来进行身份验证。按照您的产品的凭证管理文档在网关处为每个开发者创建凭证。

使用与[确认网关路由您的模型](#confirm-the-gateway-routes-your-models)相同的请求确认新颁发的密钥对网关有效，将 `<gateway-key>` 替换为新的 `<developer-key>`：

> [tab] Bash 或 Zsh
>
> ```bash theme={null}
> curl -X POST "https://llm-gateway.example.com/v1/messages" \
>   -H "Authorization: Bearer <developer-key>" \
>   -H "anthropic-version: 2023-06-01" \
>   -H "content-type: application/json" \
>   -d '{"model": "claude-sonnet-4-6", "max_tokens": 1, "messages": [{"role": "user", "content": "."}]}'
> ```

> [tab] PowerShell
>
> ```powershell theme={null}
> Invoke-RestMethod -Method Post -Uri "https://llm-gateway.example.com/v1/messages" `
>   -Headers @{ "Authorization" = "Bearer <developer-key>"; "anthropic-version" = "2023-06-01" } `
>   -ContentType "application/json" `
>   -Body '{"model": "claude-sonnet-4-6", "max_tokens": 1, "messages": [{"role": "user", "content": "."}]}'
> ```

**检查点**：带有 `content` 字段的 `200` 意味着开发者密钥到达网关，网关转发它。当[前一步](#confirm-the-gateway-routes-your-models)成功时，这里的 `401` 意味着开发者密钥错误或尚未在网关处生效。

为每个开发者颁发一个密钥而不是共享密钥是使每个开发者使用归因和个人离职工作的原因。保存密钥的环境变量取决于网关读取的标头。对于在 `Authorization: Bearer` 标头中检查凭证的网关，开发者在 `ANTHROPIC_AUTH_TOKEN` 中设置他们的密钥。对于从 `x-api-key` 标头读取密钥的网关，开发者改为设置 `ANTHROPIC_API_KEY`；[凭证表](https://code.claude.com/docs/zh-CN/llm-gateway-connect.md#set-the-credential-variable)涵盖了映射。

### 针对网关测试 Claude Code

在分发任何内容之前，使用推出将交付的相同配置自己通过网关运行 Claude Code。直接在终端中键入这些，而不是在 `.env` 或设置文件中；它们仅在此终端会话中持续，因此关闭它会将您的机器返回到其正常配置。如果您的网关读取 `x-api-key` 标头，请使用 `ANTHROPIC_API_KEY` 而不是 `ANTHROPIC_AUTH_TOKEN`：

> [tab] Bash 或 Zsh
>
> ```bash theme={null}
> export ANTHROPIC_BASE_URL=https://llm-gateway.example.com
> export ANTHROPIC_AUTH_TOKEN="<developer-key>"
> ```

> [tab] PowerShell
>
> ```powershell theme={null}
> $env:ANTHROPIC_BASE_URL = "https://llm-gateway.example.com"
> $env:ANTHROPIC_AUTH_TOKEN = "<developer-key>"
> ```

然后通过网关发送一次性提示：

```bash theme={null}
claude -p "Reply with one word: connected"
```

**检查点**：提示返回响应，请求在网关日志中显示为对 `/v1/messages` 路径的 `POST`，状态为 `200`。Claude Code 附加查询字符串，例如 `?beta=true`，因此匹配路径，而不是完整 URL。两条失败消息指向不同的方向：

* `Not logged in`：检查网关日志以区分两个原因。如果它是空的，没有凭证到达会话，没有请求离开机器；在您测试的 shell 中重新运行导出。如果它显示在 `401` 正文中带有 `x-api-key` 的被拒绝请求，网关期望密钥在该标头中；切换到 `ANTHROPIC_API_KEY`
* `Failed to authenticate. API Error: 401` 意味着凭证被发送并被拒绝，网关日志说明了在哪里：命名 `api.anthropic.com` 或您的提供商端点的 `401` 意味着网关到达了上游但其提供商凭证被拒绝，因此开发者密钥有效，网关持有的提供商凭证错误或是占位符

错误或无法到达的基础 URL 会产生不同的症状：Claude Code [以退避方式重试连接](https://code.claude.com/docs/zh-CN/errors.md#automatic-retries)，在报告错误之前可能会坐着没有输出几分钟。如果命令似乎挂起，请检查网关日志而不是等待；没有到达的请求意味着 `ANTHROPIC_BASE_URL` 不指向网关。

### 分发配置

每个开发者机器都需要网关地址和凭证。您可以通过[托管设置](https://code.claude.com/docs/zh-CN/managed-settings.md#delivery-mechanisms)集中分发它们，以便开发者不配置任何内容，或者手动向开发者提供值以自己设置。

#### 要分发的内容

无论您选择哪条路径，都适用相同的变量集。大多数推出只需要 `ANTHROPIC_BASE_URL` 和凭证；当您的网关设置需要时包括条件行。

| 变量或设置 | 它的作用 | 包括时间 |
| :- | :- | :- |
| `ANTHROPIC_BASE_URL` | 将 Claude Code 的 API 请求发送到网关而不是 `api.anthropic.com` | 总是 |
| `apiKeyHelper`，或 `ANTHROPIC_AUTH_TOKEN` 或 `ANTHROPIC_API_KEY` 中的凭证 | 对网关的每个请求进行身份验证。助手运行命令来获取密钥；变量保存静态密钥，分别作为 `Authorization: Bearer` 和 `x-api-key` 发送 | 总是；三个中的一个 |
| `ANTHROPIC_CUSTOM_HEADERS` | 向每个 API 请求添加额外的 HTTP 标头 | 您的网关在每个请求上需要租户或路由标头 |
| `CLAUDE_CODE_GATEWAY_HINT_HEADERS` | 发送[网关提示标头](https://code.claude.com/docs/zh-CN/llm-gateway-protocol.md#gateway-hint-headers)，它们为网关处的路由和调度决策对每个请求进行分类。需要 Claude Code v2.1.273 或更高版本 | 您的网关读取提示标头 |
| `CLAUDE_CODE_ENABLE_GATEWAY_MODEL_DISCOVERY` | 在启动时查询网关的 `/v1/models` 并将返回的名称添加到 `/model` 选择器 | 您的网关提供 `/v1/models` 并且您希望开发者的选择器从中填充 |
| `CLAUDE_CODE_DISABLE_EXPERIMENTAL_BETAS` | 停止 Claude Code 发送预发布功能标头和正文字段。[禁用预发布功能](https://code.claude.com/docs/zh-CN/llm-gateway-protocol.md#disable-pre-release-capabilities)涵盖了确切的范围 | 您的网关转发到拒绝 beta 字段的 Amazon Bedrock 或 Google Cloud 的 Agent Platform 上游。请参阅[网关要求](#gateway-requirements) |
| `CLAUDE_CODE_SKIP_FAST_MODE_NETWORK_ERRORS` 或 `CLAUDE_CODE_SKIP_FAST_MODE_ORG_CHECK` | 当其可用性检查（直接调用 `api.anthropic.com` 而不是遵循 `ANTHROPIC_BASE_URL`）失败、被拦截或因缺少 Anthropic 凭证而被跳过时，恢复[快速模式](https://code.claude.com/docs/zh-CN/fast-mode.md) | 您的组织使用快速模式，开发者仅使用 `ANTHROPIC_AUTH_TOKEN` 进行身份验证，在 `ANTHROPIC_API_KEY` 中使用网关颁发的密钥或来自 `apiKeyHelper`，或您的网络阻止或拦截对 `api.anthropic.com` 的直接请求；[在代理和 LLM 网关后面使用快速模式](https://code.claude.com/docs/zh-CN/fast-mode.md#use-fast-mode-behind-proxies-and-llm-gateways)涵盖了哪两个变量中的哪一个与您的配置匹配 |
| `ANTHROPIC_MODEL` 或 [`ANTHROPIC_DEFAULT_HAIKU_MODEL`](https://code.claude.com/docs/zh-CN/model-config) | 设置 Claude Code 为主会话和后台流量请求的模型名称 | 您的网关路由与 Claude Code 默认值不匹配的模型名称，或您将[后台功能](https://code.claude.com/docs/zh-CN/costs.md#background-token-usage)路由到不同的模型。在网关处路由覆盖名称和 Claude Code 的内置模型 ID，因为某些后台子调用可以请求内置 ID，无论覆盖如何；[模型配置](https://code.claude.com/docs/zh-CN/model-config.md)涵盖了会话的每个部分使用哪个模型 |
| `ANTHROPIC_BEDROCK_BASE_URL`、`ANTHROPIC_VERTEX_BASE_URL`、`ANTHROPIC_FOUNDRY_BASE_URL` 或 `ANTHROPIC_AWS_BASE_URL` 以及[该提供商的变量](https://code.claude.com/docs/zh-CN/llm-gateway-connect.md#route-to-a-cloud-provider-through-a-gateway) | 通过网关将 Claude Code 指向网关。Amazon Bedrock 和 Google Cloud 的 Agent Platform 也切换到这些提供商的本机请求格式 | 您的网关前置 Amazon Bedrock、Google Cloud 的 Agent Platform、Microsoft Foundry 或 AWS 上的 Claude 平台；请参阅 [API 格式](https://code.claude.com/docs/zh-CN/llm-gateway-protocol.md#api-formats) |

#### 通过托管设置分发

通过[托管设置文件](https://code.claude.com/docs/zh-CN/managed-settings.md#delivery-mechanisms)的 `env` 块交付变量，由 MDM、注册表策略或配置管理推送：

```json theme={null}
{
  "env": {
    "ANTHROPIC_BASE_URL": "https://llm-gateway.example.com"
  },
  "apiKeyHelper": "/usr/local/bin/get-gateway-key"
}
```

将表中的条件变量添加到相同的 `env` 块。托管的 `ANTHROPIC_BASE_URL` 被强制执行，不能被开发者的 shell 导出覆盖，因为 Claude Code 在进程环境和较低优先级设置上应用它。

不要在托管设置中与网关凭证一起包括 `forceLoginMethod`、`forceLoginOrgUUID` 或 `forceLoginGatewayUrl`。`forceLoginMethod` 或 `forceLoginOrgUUID`，具有任何值，在启动时阻止 `ANTHROPIC_API_KEY`、`ANTHROPIC_AUTH_TOKEN` 和 `apiKeyHelper`，开发者无法继续。他们看到 `This machine's managed settings require a first-party login`，或当文件将 `forceLoginMethod` 设置为 `"gateway"` 或设置 `forceLoginGatewayUrl` 时看到 [`Administrator policy requires a Cloud gateway sign-in`](https://code.claude.com/docs/zh-CN/errors#administrator-policy-requires-a-cloud-gateway-sign-in)。

[服务器管理的设置](https://code.claude.com/docs/zh-CN/server-managed-settings.md#platform-availability)交付需要直接连接到 `api.anthropic.com`，因此它不会到达网关路由的会话。网关部署使用这个基于文件的托管设置路径，它强制执行相同的密钥。

对于凭证，在托管设置文件中分发一个 [`apiKeyHelper`](https://code.claude.com/docs/zh-CN/llm-gateway-connect#rotate-credentials-with-apikeyhelper) 命令，如上所示；该命令作为本地开发者对您的秘密存储进行身份验证，因此每台机器都接收自己的密钥。或者，通过您现有的秘密流程向每个开发者交付他们的密钥，并让他们自己设置 `ANTHROPIC_AUTH_TOKEN`。

某些环境需要单独的交付：

* 桌面应用从其第三方推理配置读取网关路由，而不是从托管设置；通过 MDM 部署该文件以及托管设置，以便桌面会话也通过网关路由。请参阅[桌面第三方配置文档](https://claude.com/docs/third-party/claude-desktop/configuration)和[桌面网关文档](https://claude.com/docs/third-party/claude-desktop/gateway)
* CI 运行器需要在[运行器的环境](https://code.claude.com/docs/zh-CN/llm-gateway-connect.md#configure-each-surface)中设置 `ANTHROPIC_BASE_URL` 和凭证
* 托管 Windows 机器上的 WSL 仅在 [`wslInheritsWindowsSettings`](https://code.claude.com/docs/zh-CN/settings-reference#wslinheritswindowssettings) 为 `true` 时读取 Windows 托管设置

#### 网关后的 HIPAA 配置

通过网关的会话不符合 HIPAA 配置的条件。[检查开发者如何登录和连接](https://code.claude.com/docs/zh-CN/hipaa-setup.md#check-how-developers-sign-in-and-connect)列出了哪些登录和连接方式符合条件。

要在托管设置文件中为这些会话限制功能，请使用[将出口路径映射到托管控制和事件](https://code.claude.com/docs/zh-CN/monitoring-usage.md#map-egress-paths-to-managed-controls-and-events)中的键。这些键不会使会话符合 HIPAA 配置的条件，也不涵盖该配置所更改的全部内容。例如：

* **云端会话**：没有任何托管键可以关闭它们。请参阅[管理控制台控制](https://code.claude.com/docs/zh-CN/desktop.md#admin-console-controls)
* **子进程中的 Anthropic 凭据**：该配置会从 Claude Code 启动的进程中[移除它们](https://code.claude.com/docs/zh-CN/hipaa-setup.md#anthropic-credentials-in-commands-hooks-and-mcp-servers)，而没有任何设置项只执行这一操作

不要使用 `CLAUDE_CODE_DISABLE_NONESSENTIAL_TRAFFIC` 代替这些键。它会关闭其中一些功能，但也会关闭自动更新程序，并且会让 WebFetch 保持开启。

#### 手动向开发者提供值以自己设置

如果您没有托管设置分发，请向每个开发者发送他们需要的内容以遵循[连接页面](https://code.claude.com/docs/zh-CN/llm-gateway-connect.md#configure-claude-code-yourself)：

* 网关 URL
* 他们的个人凭证
* **将凭证放在哪个变量中**：对于 bearer-token 网关为 `ANTHROPIC_AUTH_TOKEN`，或对于 `x-api-key` 网关为 `ANTHROPIC_API_KEY`。告诉开发者哪一个可以节省他们在[连接页面](https://code.claude.com/docs/zh-CN/llm-gateway-connect.md#set-the-credential-variable)上描述的试错
* [要分发的内容表](#what-to-distribute)中的任何条件变量，以及它们的值

[连接页面](https://code.claude.com/docs/zh-CN/llm-gateway-connect.md#configure-claude-code-yourself)指导开发者设置每一个。

**检查点**：在开发者机器上，`claude` 启动会话而不显示登录屏幕，因为分发的凭证满足身份验证。然后运行 `/status` 并打开**状态**选项卡：`Anthropic base URL` 行显示网关地址，对于托管分发，`Setting sources` 行包括托管设置。登录屏幕或缺少 `Anthropic base URL` 行意味着配置没有到达机器。

### 验证推出

从开发者机器而不是网关主机确认一切工作，以便测试涵盖开发者使用的网络路径。发送流式请求，它一次检查端点、流式传输传递和模型路由：

> [tab] Bash 或 Zsh
>
> ```bash theme={null}
> curl -N -X POST "https://llm-gateway.example.com/v1/messages" \
>   -H "Authorization: Bearer <developer-key>" \
>   -H "anthropic-version: 2023-06-01" \
>   -H "content-type: application/json" \
>   -d '{"model": "claude-sonnet-4-6", "max_tokens": 16, "stream": true, "messages": [{"role": "user", "content": "count to 3"}]}'
> ```

> [tab] PowerShell
>
> ```powershell theme={null}
> $body = '{"model": "claude-sonnet-4-6", "max_tokens": 16, "stream": true, "messages": [{"role": "user", "content": "count to 3"}]}'
> $body | curl.exe -N -X POST "https://llm-gateway.example.com/v1/messages" `
>   -H "Authorization: Bearer <developer-key>" `
>   -H "anthropic-version: 2023-06-01" `
>   -H "content-type: application/json" `
>   --data-binary '@-'
> ```

您应该看到 `data:` 行逐步到达。整个响应在暂停后一次到达意味着网关正在缓冲，这会停滞 Claude Code；`404` 意味着模型名称未路由。每个模型名称重复。

然后启动 `claude` 并发送消息。此步骤的每个症状都有一个原因：

* 登录提示意味着凭证缺口。运行 `/status` 并打开**状态**选项卡：当 `Setting sources` 行不包括托管设置时，分发没有到达机器；当它包括时，开发者凭证没有被交付，因此设置 `ANTHROPIC_AUTH_TOKEN` 或 `apiKeyHelper`
* `Failed to authenticate` 错误意味着网关拒绝请求；其日志说明了哪个凭证失败。网关自己记录的拒绝命名开发者密钥，而来自 `api.anthropic.com` 或您的提供商端点的 `401` 意味着网关持有的提供商凭证被拒绝
* 当网关期望密钥在 `x-api-key` 标头中时，在首次使用时出现一次性批准提示是预期的，设置为 `ANTHROPIC_API_KEY`。使用 `ANTHROPIC_AUTH_TOKEN`，不会出现提示，变量会无声地接管；以前保存的 claude.ai 登录对该会话无效

如果您的组织使用[快速模式](https://code.claude.com/docs/zh-CN/fast-mode.md)，也在这里运行 `/fast`：可用性检查直接调用 `api.anthropic.com` 而不是遵循网关基础 URL，因此网关路由的会话可以报告快速模式不可用或禁用，即使推理有效。[在代理和 LLM 网关后面使用快速模式](https://code.claude.com/docs/zh-CN/fast-mode.md#use-fast-mode-behind-proxies-and-llm-gateways)将每条消息映射到恢复它的变量，与[配置的其余部分](#distribute-the-configuration)一起分发。

最后，检查网关的日志以查看您发送的消息：凭证标识开发者，[`x-claude-code-session-id` 标头](https://code.claude.com/docs/zh-CN/llm-gateway-protocol#request-headers)按会话对请求进行分组。如果功能因[故障排除症状](https://code.claude.com/docs/zh-CN/llm-gateway-connect.md#troubleshoot-gateway-errors)而失败，网关正在剥离标头或重写错误；请参阅上面的[网关要求](#gateway-requirements)。

## 维护网关

推出后，三种变化会随着时间到达网关。每一种都有一个症状要观察和一个要采取的行动。

| 变化 | 当网关没有跟上时的症状 | 行动 |
| :- | :- | :- |
| 新的 Claude Code 版本添加 `anthropic-beta` 值和请求正文字段 | 开发者在更新 Claude Code 后报告 `400` 错误，命名新字段；请参阅[功能传递](https://code.claude.com/docs/zh-CN/llm-gateway-protocol.md#feature-pass-through) | 逐字转发 `anthropic-*` 标头和请求正文，而不是允许列表；在新 Claude Code 版本到达开发者之前针对网关测试它们，检查[规划 Claude Code 版本升级](#plan-claude-code-version-upgrades)中的区域 |
| 新的 Claude 模型变得可用 | 开发者选择新模型名称得到 `404`；`/model` 选择器不列出它 | 将模型名称添加到网关的路由配置，然后重新运行[路由检查](#confirm-the-gateway-routes-your-models)。如果您分发 `ANTHROPIC_MODEL` 或默认模型变量，更新托管设置 |
| 凭证过期或需要轮换 | 所有开发者请求开始从上游失败，出现 `401` | 按照自己的计划轮换网关的提供商凭证；开发者密钥在网关处轮换，[`apiKeyHelper`](https://code.claude.com/docs/zh-CN/llm-gateway-connect#rotate-credentials-with-apikeyhelper) 处理每个开发者的轮换，无需重新分发设置 |

在调整每个密钥的速率限制时，考虑客户端[重试瞬时故障](https://code.claude.com/docs/zh-CN/errors.md#automatic-retries)，包括 `429` 响应，最多 10 次，带有退避，遵守 `Retry-After`。将[兼容性指南](https://code.claude.com/docs/zh-CN/llm-gateway-protocol.md)作为每个 Claude Code 版本发送的内容的参考。

### 规划 Claude Code 版本升级

某些 Claude Code 行为内置于已安装的版本中，而不是在您的网关处设置，因此将开发者移至新版本可以改变整个部署中的行为，即使网关配置没有改变。要控制何时发生这种情况，请使用 [`requiredMaximumVersion`](https://code.claude.com/docs/zh-CN/settings-reference#requiredmaximumversion) 将开发者固定到已测试的版本，或者如果您通过自己的渠道分发 Claude Code，请使用 [`DISABLE_UPDATES`](https://code.claude.com/docs/zh-CN/setup#disable-auto-updates)。在提高固定版本之前，请阅读新版本的[更新日志](https://code.claude.com/docs/en/changelog.md)条目并[针对网关测试它](#test-claude-code-against-the-gateway)。

当您测试一个版本时，网关拒绝的新标头或请求字段显示为[维护网关](#maintain-the-gateway)中描述的 `400` 错误。下表涵盖不产生错误的版本相关变化，以及保持每个变化在升级中保持不变的设置。

| 区域 | 开发者升级时可能改变的内容 | 保持其不变的设置 |
| :- | :- | :- |
| 功能标志默认值 | [不从 Anthropic 获取功能标志](https://code.claude.com/docs/zh-CN/env-vars.md#features-that-need-feature-flag-fetching)的会话，例如云提供商上的会话或关闭遥测的会话，使用内置于已安装版本中的标志默认值。当版本改变其中一个默认值时，这些开发者的行为在他们升级后立即改变 | 版本固定本身，`requiredMaximumVersion` 或 `DISABLE_UPDATES` |
| 模型能力假设 | 已安装版本不识别的模型 ID，例如网关别名 `prod-opus`，对[自适应推理](https://code.claude.com/docs/zh-CN/model-config.md#adaptive-reasoning-and-fixed-thinking-budgets)、努力参数和[上下文窗口](https://code.claude.com/docs/zh-CN/model-config.md#correct-the-window-for-a-gateway-or-custom-model-id)运行默认假设，直到更高版本识别该 ID 或您映射它 | 在网关处路由 Anthropic 模型 ID，或添加[`modelOverrides`](https://code.claude.com/docs/zh-CN/model-config#override-model-ids-per-version)条目，将 Anthropic 模型 ID 映射到您的别名。在云提供商连接上，您可以改为[声明固定模型的能力](https://code.claude.com/docs/zh-CN/model-config.md#customize-pinned-model-display-and-capabilities) |
| 默认模型和别名 | 新会话默认启动的模型，以及别名（如 `opus` 和 `sonnet`）解析到的模型，[内置于每个版本](https://code.claude.com/docs/zh-CN/model-config.md#pin-models-for-third-party-deployments)中，开发者升级时可能改变 | [`ANTHROPIC_DEFAULT_MODEL`](https://code.claude.com/docs/zh-CN/model-config#set-a-default-model-for-new-sessions) 用于新会话启动的模型，以及 [`ANTHROPIC_DEFAULT_*_MODEL` 变量](https://code.claude.com/docs/zh-CN/model-config#environment-variables)，例如 `ANTHROPIC_DEFAULT_OPUS_MODEL`，用于每个别名解析到的内容。`ANTHROPIC_DEFAULT_MODEL` 需要 Claude Code v2.1.236 或更高版本 |

## 相关资源

* [将 Claude Code 连接到 LLM 网关](https://code.claude.com/docs/zh-CN/llm-gateway-connect.md)：面向开发者的设置步骤，具有每个表面的配置和您可以交给开发者的故障排除表
* [网关兼容性指南](https://code.claude.com/docs/zh-CN/llm-gateway-protocol.md)：网关运营商的参考指南，涵盖端点、要转发的标头以及功能传递表
* [Claude Code 使用的值](https://code.claude.com/docs/zh-CN/settings.md#which-value-claude-code-uses)：托管、项目和用户设置如何组合
* [交付机制](https://code.claude.com/docs/zh-CN/managed-settings.md#delivery-mechanisms)：托管文件在每个平台上的位置
* [为您的组织设置 Claude Code](https://code.claude.com/docs/zh-CN/admin-setup.md)：这个网关是其中一部分的更广泛推出，包括策略强制执行、使用可见性和数据处理
