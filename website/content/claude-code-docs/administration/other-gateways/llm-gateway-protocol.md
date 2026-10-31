> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Claude Code 网关兼容性指南

> 保持 LLM 网关与 Claude Code 兼容：它调用的端点、必须转发的标头和正文字段，以及删除它们时会破坏的功能。

本页面记录了 Claude Code 发送到网关的请求，包括它调用的端点、网关必须转发的标头和正文字段，以及不转发时停止工作的功能。本指南是为配置网关产品以与 Claude Code 兼容的运营人员编写的。

[Claude apps gateway](https://code.claude.com/docs/zh-CN/claude-apps-gateway) 是 Anthropic 的自托管网关，在 `GET /protocol` 处提供自己的端点参考，涵盖该网关的登录、推理、托管设置、模型发现和遥测端点。这是一份与本指南分开的文档。

> [!NOTE]
>
> * 要为您的组织推出现有或第三方网关，请参阅[推出 LLM 网关](https://code.claude.com/docs/zh-CN/llm-gateway-rollout.md)
> * 如果您是使用给定的凭证向网关验证 Claude Code 的个人开发者，请参阅[将 Claude Code 连接到 LLM 网关](https://code.claude.com/docs/zh-CN/llm-gateway-connect.md)

本页面涵盖：

* [API 格式](#api-formats)和每种格式要提供的端点
* [按连接方法的客户端行为](#how-the-connection-method-changes-client-behavior)：模型 ID、`anthropic-beta` 值、请求字段和默认值在格式和 Claude apps 网关登录之间的差异
* [请求标头](#request-headers)：哪些必须到达上游，哪些您的网关可以使用
* [响应标头](#response-headers)：返回什么以使停滞检测、重试和使用限制显示工作
* [系统提示属性块](#system-prompt-attribution-block)及其与提示缓存的交互方式
* [功能传递](#feature-pass-through)：删除标头或正文字段时会破坏什么
* [模型发现](#model-discovery)

本页面使用两个术语来描述您的网关对每个标头和正文字段的处理方式：

* **转发不变**：逐字节将其传递到上游
* **使用**：网关可能会读取它以进行路由、属性或跟踪，不需要转发它

任何未标记为转发不变的内容都可以由您使用或忽略。

## API 格式

网关必须向 Claude Code 客户端公开以下至少一种 API 格式。客户端选择一种格式，并通过下表"选择方式"列中的变量将 Claude Code 指向您的网关。

Google Cloud 的 Agent Platform 是 Google Cloud 的 Claude 端点，原名为 Vertex AI；其变量名保留 `VERTEX` 拼写。

| 格式 | 选择方式 | 端点 | 原样转发 |
| :- | :- | :- | :- |
| Anthropic Messages | `ANTHROPIC_BASE_URL` | `/v1/messages`, `/v1/messages/count_tokens`（可选） | `anthropic-beta` 和 `anthropic-version` 请求头 |
| Amazon Bedrock InvokeModel | `ANTHROPIC_BEDROCK_BASE_URL` 配合 `CLAUDE_CODE_USE_BEDROCK=1` | `/model/{model}/invoke`, `/model/{model}/invoke-with-response-stream`, `/model/{model}/count-tokens`（可选） | `anthropic_beta` 和 `anthropic_version` 请求体字段 |
| Google Cloud's Agent Platform rawPredict | `ANTHROPIC_VERTEX_BASE_URL` 配合 `CLAUDE_CODE_USE_VERTEX=1` | `:rawPredict`, `:streamRawPredict`, `count-tokens:rawPredict`（可选） | `anthropic-beta` 和 `anthropic-version` 请求头，以及 `anthropic_version` 请求体字段 |

### Foundry 和 AWS 上的 Claude Platform

Microsoft Foundry 和 [AWS 上的 Claude Platform](https://code.claude.com/docs/zh-CN/claude-platform-on-aws.md) 实现了 Anthropic Messages 格式。Claude Code 通过它们自己的变量 `ANTHROPIC_FOUNDRY_BASE_URL` 和 `ANTHROPIC_AWS_BASE_URL` 路由到它们，但网关在任一前面实现上述 Anthropic Messages 行。在 AWS 上的 Claude Platform 前面的网关还必须转发 `anthropic-workspace-id` 头，[该平台在每个请求上都需要](https://code.claude.com/docs/zh-CN/claude-platform-on-aws.md)。

### 可选端点和启动流量

令牌计数端点是唯一可选的：当它们不存在时，Claude Code 会回退到基于字符的上下文使用估计。

根据路径而不是完整 URL 进行匹配：

* 推理请求发送到 `/v1/messages?beta=true`
* Google Cloud 的 Agent Platform 方法后缀附加到发布者模型路径，如 `/projects/{project}/locations/{location}/publishers/anthropic/models/{model}:streamRawPredict`

网关还会看到尽力而为的启动流量，可以拒绝而不会破坏任何东西。Anthropic Messages 格式的网关接收 `HEAD /api/hello` 连接预热探针，当配置了 HTTP 代理或客户端证书时，Claude Code 会跳过该探针。Amazon Bedrock 格式的网关接收 `GET /inference-profiles?type=SYSTEM_DEFINED` 请求，以及当配置的模型是推理配置文件时，`GET /inference-profiles/{profile}` 查询。

通过网关的会话不符合 HIPAA 配置的条件。[检查开发人员如何登录和连接](https://code.claude.com/docs/zh-CN/hipaa-setup.md#check-how-developers-sign-in-and-connect)列出了哪些登录和连接方式符合条件。请参阅[网关后面的 HIPAA 配置](https://code.claude.com/docs/zh-CN/llm-gateway-rollout.md#the-hipaa-configuration-behind-a-gateway)。

[快速模式](https://code.claude.com/docs/zh-CN/fast-mode.md)可用性检查永远不会出现在网关日志中：它直接调用 `api.anthropic.com` 而不是遵循 `ANTHROPIC_BASE_URL`，因此在阻止直接出站到 `api.anthropic.com` 的网络上，快速模式可能会报告连接错误，而通过网关的推理继续工作。[WebFetch 域安全检查](https://code.claude.com/docs/zh-CN/data-usage.md#webfetch-domain-safety-check)也直接调用 `api.anthropic.com`。[在代理和 LLM 网关后面使用快速模式](https://code.claude.com/docs/zh-CN/fast-mode.md#use-fast-mode-behind-proxies-and-llm-gateways)涵盖了恢复它的变量。

### 流式传输

Claude Code 会在每个流式推理响应到达时逐个事件地读取它，因此您的网关中继流的方式会影响用户看到的内容：

* 如果您的网关将响应一直缓冲到完整为止，Claude Code 会停滞。
* Claude Code 期望按顺序收到每个响应的完整事件序列，直到最终的 `message_delta` 和 `message_stop` 事件。如果正文在内容块已启动之后、但在最终 `message_delta` 之前正常结束，Claude Code 会将该响应视为连接断开。[上述响应可能不完整](https://code.claude.com/docs/zh-CN/errors.md#the-response-above-may-be-incomplete)描述了此时用户看到的内容，[自动重试](https://code.claude.com/docs/zh-CN/errors.md#automatic-retries)则说明了 Claude Code 何时会改为重新发出请求。
* 当 Amazon Bedrock 护栏拦截回复时，Bedrock 发送的事件可能会引用其 `content_block_stop` 已经到达的内容块，而 Claude Code 依赖于按原样接收这些事件。[AWS Guardrails](https://code.claude.com/docs/zh-CN/amazon-bedrock.md#aws-guardrails) 描述了该回复如何结束。
* 一旦超过其[流式空闲超时时间](https://code.claude.com/docs/zh-CN/network-config.md#streaming-idle-watchdogs)仍没有任何字节到达，Claude Code 就会中止流式响应。在长时间的思考暂停期间，上游的 SSE `ping` 事件可能是流上唯一的字节，因此剥离或缓冲这些事件的网关可能会在响应中途触发该超时。从不发送 ping 的上游（如 Amazon Bedrock 的二进制事件流）进行转换的网关也存在同样的间隙，除非它发出自己的 `ping` 事件。
* 在 [Amazon Bedrock InvokeModel 格式](#api-formats)下，Claude Code 会将 `/model/{model}/invoke-with-response-stream` 响应作为 Bedrock 返回的二进制 `application/vnd.amazon.eventstream` 正文来读取，一旦网关将其转换为服务器发送事件或重写该 `Content-Type` 头，Claude Code 就无法解析它。[网关或代理后面的流式传输错误](https://code.claude.com/docs/zh-CN/amazon-bedrock.md#streaming-errors-behind-a-gateway-or-proxy)描述了此时用户看到的内容。

### 与上游的格式不匹配

客户端使用的格式决定了您的网关接收的内容。常见的失败模式是客户端发送到您的网关的格式与其后面的上游提供商接受的格式不匹配。

* 当客户端使用 Amazon Bedrock 或 Google Cloud 的 Agent Platform 格式时，Claude Code 仅发送这些提供商接受的完整功能集的子集
* 当客户端使用 Anthropic Messages 格式时，Claude Code 发送完整集，即使您的网关转发到 Amazon Bedrock 或 Google Cloud 的 Agent Platform 上游

弥合这种差异是您的网关的工作。[功能传递](#feature-pass-through)描述了当它不这样做时会破坏什么。

如果您的上游是 Amazon Bedrock 或 Google Cloud 的 Agent Platform，您可以通过公开该提供商的格式来避免桥接。[通过网关路由到云提供商](https://code.claude.com/docs/zh-CN/llm-gateway-connect.md#route-to-a-cloud-provider-through-a-gateway)显示了该格式的客户端配置。

## 连接方法如何改变客户端行为

开发者连接到网关的方式决定了 Claude Code 发送的模型 ID、`anthropic-beta` 值和请求字段，以及它应用的默认值。您的网关会看到以下三种客户端行为之一：

* **Amazon Bedrock 或 Agent Platform 格式**：开发者设置 `CLAUDE_CODE_USE_BEDROCK=1` 和 `ANTHROPIC_BEDROCK_BASE_URL`，或 `CLAUDE_CODE_USE_VERTEX=1` 和 `ANTHROPIC_VERTEX_BASE_URL`，指向您的网关。Claude Code 使用该提供商的模型 ID、请求字段和默认值。
* **Anthropic Messages 格式**：开发者将 `ANTHROPIC_BASE_URL` 设置为您的网关。Claude Code 将网关视为 Claude API，无法判断您转发到哪个上游。
* **Claude apps gateway 登录**：开发者登录到 [Claude apps gateway](https://code.claude.com/docs/zh-CN/claude-apps-gateway)。该网关使用 Anthropic Messages 格式，但可以路由到任何上游，因此 Claude Code 仅发送 Amazon Bedrock 和 Agent Platform 也接受的 `anthropic-beta` 值和模型能力假设。

### 按连接方法的请求和默认值

下表比较了三种连接方法，每行一个行为。它省略了 Microsoft Foundry 和 Claude Platform on AWS，它们也使用 Anthropic Messages 格式，但 Claude Code 通过它们自己的变量访问。有关这些，请参阅 [Microsoft Foundry](https://code.claude.com/docs/zh-CN/microsoft-foundry.md) 和 [Claude Platform on AWS](https://code.claude.com/docs/zh-CN/claude-platform-on-aws.md) 页面。

| 行为 | Amazon Bedrock 或 Agent Platform 格式 | Anthropic Messages 格式 | Claude apps gateway 登录 |
| :- | :- | :- | :- |
| 默认情况下请求中的模型 ID | 提供商的形式，例如 Amazon Bedrock 上的 `us.anthropic.claude-opus-4-8` | Anthropic ID，例如 `claude-opus-4-8` | Anthropic ID |
| 发送的 `anthropic-beta` 值 | Amazon Bedrock 和 Agent Platform 接受的子集 | [功能传递](#feature-pass-through)下描述的完整集合，除非开发者设置 [`CLAUDE_CODE_DISABLE_EXPERIMENTAL_BETAS`](#disable-pre-release-capabilities) | Amazon Bedrock 和 Agent Platform 接受的子集 |
| Claude Code 无法识别的模型 ID（例如网关别名）的请求字段 | 使用固定预算的思考而不是自适应推理，以及没有努力或上下文管理字段。Claude Code 仍会发送[对话中途的系统消息](#feature-pass-through) | 当前 Claude 模型在 Claude API 上接受的所有内容，包括自适应推理、努力和上下文管理，Amazon Bedrock 或 Agent Platform 上游可能会拒绝 | 与 Amazon Bedrock 或 Agent Platform 格式相同 |
| 开发者选择加入时的一小时 [提示缓存 TTL](https://code.claude.com/docs/zh-CN/prompt-caching.md#choose-the-ttl-yourself) | 通过 `cache_control` 中的 `ttl` 字段请求，没有 beta 值 | 通过 `ttl` 字段加上 `anthropic-beta` 中的 `extended-cache-ttl` 值请求，您必须转发 | 通过 `ttl` 字段请求，没有 beta 值。请参阅[一小时提示缓存](https://code.claude.com/docs/zh-CN/claude-apps-gateway-config.md#one-hour-prompt-cache) |
| [后台任务](https://code.claude.com/docs/zh-CN/costs.md#background-token-usage) 的模型，除非 `ANTHROPIC_DEFAULT_HAIKU_MODEL` 固定一个 | 默认 Sonnet 模型，或选择主模型后的主模型，如 [Amazon Bedrock](https://code.claude.com/docs/zh-CN/amazon-bedrock.md#4-pin-model-versions) 和 [Agent Platform](https://code.claude.com/docs/zh-CN/google-vertex-ai.md#5-pin-model-versions) 页面所述 | 主模型，或当 `ANTHROPIC_API_KEY` 或 `apiKeyHelper` 提供 Anthropic Console 密钥且 `ANTHROPIC_AUTH_TOKEN` 未设置时的默认 Haiku 模型 | 主模型 |

有关每个连接支持的功能以及它默认发送给 Anthropic 的遥测，请参阅 [功能可用性](https://code.claude.com/docs/zh-CN/feature-availability.md#availability-by-model-provider) 和 [按 API 提供商的默认行为](https://code.claude.com/docs/zh-CN/data-usage.md#default-behaviors-by-api-provider)。

### 未识别模型 ID 的设置

两个客户端设置改变了 Claude Code 对它无法识别的模型 ID 的假设，无论开发者使用哪种连接方法：

* **上下文窗口**：Claude Code 假设 200K，或当 ID 包含 `[1m]` 时为 1M。要声明真实窗口，请参阅 [为网关或自定义模型 ID 更正窗口](https://code.claude.com/docs/zh-CN/model-config.md#correct-the-window-for-a-gateway-or-custom-model-id)
* **能力**：要给网关别名赋予其后面模型的能力，请使用您分发的设置中的 [`modelOverrides`](https://code.claude.com/docs/zh-CN/errors#unrecognized-model-id-on-a-request) 条目将该模型的 Anthropic ID 映射到您的别名。有关 `ANTHROPIC_DEFAULT_*_MODEL_SUPPORTED_CAPABILITIES` 变量适用的位置，请参阅 [功能传递](#feature-pass-through)

## 请求标头

Claude Code 会在 API 请求中包含以下标头。标头名称在传输时不区分大小写。请原样转发 `anthropic-version` 和 `anthropic-beta`，当上游为 [Claude Platform on AWS](https://code.claude.com/docs/zh-CN/claude-platform-on-aws.md) 时还需转发 `anthropic-workspace-id`；其余标头网关可用于路由、归因和追踪，无需转发。

| 标头 | 描述 |
| :- | :- |
| `Authorization`、`x-api-key` | 开发者的网关凭据，根据开发者设置的[凭据变量](https://code.claude.com/docs/zh-CN/llm-gateway-connect.md#set-the-credential-variable)，出现在其中一个或两个标头中 |
| `anthropic-version` | API 版本，当前为 `2023-06-01`。Amazon Bedrock 格式和 Google Cloud's Agent Platform 格式的请求还会携带 `anthropic_version` 请求体字段，其值为提供商的方言字符串，而不是此标头的值 |
| `anthropic-beta` | 请求的以逗号分隔的功能值。请原样转发此标头；不要对单个值设置允许列表，因为该集合会随 Claude Code 版本发布而变化。当开发者使用 claude.ai 登录进行身份验证时（在设置了 `ANTHROPIC_BASE_URL` 但未设置网关凭据变量的情况下可能出现），此标头还会携带上游所需的 OAuth 功能值，将其移除会导致这些请求以 `401` 失败 |
| `x-claude-code-session-id` | 当前 Claude Code 会话的唯一标识符。可用它汇总来自同一会话的所有请求，而无需解析请求体 |
| `x-claude-code-agent-id` | 发出请求的[子代理](https://code.claude.com/docs/zh-CN/sub-agents.md)的标识符，仅出现在由 Claude Code 在会话内生成的 Agent 所发出的请求中。将其与会话 ID 结合使用，可将成本归因到并行运行的各个 Agent |
| `x-claude-code-parent-agent-id` | 生成发出请求的 Agent 的那个 Agent 的标识符，仅在嵌套 Agent 时出现 |

每次 Claude Code 生成子代理时，都会重新生成子代理 ID。队友 Agent（即 [agent team](https://code.claude.com/docs/zh-CN/agent-teams.md) 中的具名成员）在重新连接时会复用一个基于名称的稳定 ID。在这两种情况下，ID 标识的都是 Agent，而不是人员或设备，因此请勿将 Agent ID 标头视为用户标识符。

如果您的开发者设置了 `ANTHROPIC_CUSTOM_HEADERS`，这些标头也会出现在请求中。

### 网关提示标头

Claude Code 还可以发送路由提示：即网关或路由器可用于调度、缓存或归因请求的每个请求的相关信息。需要 Claude Code v2.1.273 或更高版本。

请求是否携带这些标头，取决于 Claude Code 将请求发送到何处：

* 直接连接 Anthropic API：默认发送
* 自定义基础 URL：默认关闭，因为拒绝未知标头的代理会导致请求失败。如需接收这些标头，请为您的开发者设置 [`CLAUDE_CODE_GATEWAY_HINT_HEADERS=1`](https://code.claude.com/docs/zh-CN/env-vars)，例如在[托管设置](https://code.claude.com/docs/zh-CN/managed-settings.md)的 `env` 块中设置
* 任何其他后端，包括 Amazon Bedrock、Google Cloud's Agent Platform、Microsoft Foundry 和 Claude Platform on AWS：仅在设置了 `CLAUDE_CODE_GATEWAY_HINT_HEADERS=1` 时发送

将 `CLAUDE_CODE_GATEWAY_HINT_HEADERS` 设置为 `0` 会在所有连接上停止发送这些标头。

这些标头仅携带下表所列的内容：固定词汇、工具名称、时长以及一个随机的提示词标识符，绝不包含提示词文本或文件内容。所有值均为可打印 ASCII 字符。

| 标头 | 描述 |
| :- | :- |
| `x-claude-code-request-class` | 请求的类型：`main` 表示主对话的一个轮次，`subagent` 表示[子代理](https://code.claude.com/docs/zh-CN/sub-agents.md)的一个轮次，`workflow` 表示在工作流中运行的 Agent，`compaction` 表示用于压缩对话的摘要请求，`auxiliary` 表示辅助请求，例如会话标题、分类器和摘要。每个请求都会发送 |
| `x-claude-code-agent-type` | 发出请求的子代理类型：内置 Agent 类型名称，例如 `Explore`、`Plan` 或 `general-purpose`；`custom` 表示用户定义的 Agent；`teammate` 表示在主导者进程中运行的 [agent team](https://code.claude.com/docs/zh-CN/agent-teams.md) 成员；`fork` 表示[分叉](https://code.claude.com/docs/zh-CN/sub-agents.md#fork-the-current-conversation)。仅出现在子代理自身的轮次中；子代理的压缩请求或辅助请求保留 Agent ID，但不携带类型。用户选择的 Agent 名称绝不会被发送 |
| `x-claude-code-compaction` | 出现在[上下文压缩](https://code.claude.com/docs/zh-CN/prompt-caching.md#compacting-the-conversation)期间对对话进行摘要的请求中。其值表示触发原因：`auto` 表示上下文窗口接近容量上限，`manual` 表示 `/compact`，`reactive` 表示 API 因请求过长而拒绝了请求。在交互式终端会话中进行的[空闲压缩](https://code.claude.com/docs/zh-CN/settings-reference.md#idlecompaction)，其值也为 `auto`。其他所有请求中均不出现，包括在压缩之前于后台写入摘要的请求 |
| `x-claude-code-context-compacted` | 在压缩后的第一个主对话请求中出现一次，取值与 `x-claude-code-compaction` 相同。此请求之前的对话前缀将不再使用，因此以其为键的缓存可以丢弃 |
| `x-claude-code-prev-tool-durations` | 此请求所携带结果对应的工具调用的实测运行时间，格式为 `<name>=<ms>;<name>=<ms>`，例如 `Bash=742;Read=9`。在一批工具调用之后，于同一对话的下一个请求中发送，适用于主会话或子代理 |
| `x-claude-code-prompt-id` | 随机 UUID，用于标识请求所服务的用户提示词。服务于同一提示词的请求共享该值，包括该提示词所启动的子代理的轮次。未归属于某个提示词的请求不包含此标头。可用它按提示词对会话中的请求进行分组。需要 Claude Code v2.1.283 或更高版本 |

在解析 `x-claude-code-prev-tool-durations` 之前，请了解 Claude Code 如何构建该值以及它会省略哪些内容：

* 条目：每个已运行的工具调用对应一个条目，按其结果被收集的顺序排列，以整毫秒为单位
* 上限：Claude Code 最多发送 32 个条目和 4 KB，保留最前面的条目
* 编码：工具名称采用百分号编码，涵盖 `%`、`;`、`=`、逗号、空格以及可打印 ASCII 范围之外的任何字符
* 解析：先按 `;` 拆分，再按 `=` 拆分，然后解码每个名称
* 缺失：压缩调用、辅助请求以及新提示词的第一个请求绝不会携带此标头。不要将缺失该标头理解为某个轮次未运行任何工具
* 时间：每个时间都不包括权限提示和 hook 的耗时，并行的工具调用各自报告自己的时间，因此各条目之和并不等于请求之间的间隔

### 作为开放列表转发

请将标头和请求体字段视为开放列表，而非封闭列表。Claude Code 会随着版本发布获得新功能，这些功能会以新的 `anthropic-beta` 值、新的请求体字段，偶尔也会以新的 `anthropic-*` 或 `x-claude-code-*` 标头的形式出现。

转发到 Anthropic 格式的上游时，请原样传递 `anthropic-*` 请求标头和请求体字段，而不是为当前看到的那些设置允许列表。固定于已观察到的列表的网关会移除下一个功能的标头或字段，并在引入该功能的版本中导致其失效。

例外情况是非 Anthropic 上游，例如 Amazon Bedrock 或 Google Cloud's Agent Platform，在这种情况下，弥合 schema 差异是网关的职责；请参阅[功能透传](#feature-pass-through)。

## 响应头

Claude Code 读取这些响应头来检测停滞的流、决定是否以及何时重试，以及显示使用限制。该表列出了每个响应头应返回的内容。同时转发错误响应体不做修改，以便 Claude Code 的[能力拒绝恢复](#automatic-retry-and-error-forwarding)可以匹配上游的错误措辞。

| 头部 | 返回内容及原因 |
| :- | :- |
| `content-type` | 在流式 Anthropic Messages 格式响应上返回 `text/event-stream`，在 Amazon Bedrock 格式响应上返回 `application/vnd.amazon.eventstream`（不做修改），其中[不同的类型会导致请求失败](https://code.claude.com/docs/zh-CN/amazon-bedrock.md#streaming-errors-behind-a-gateway-or-proxy) |
| `retry-after` | 返回整数秒而不是 HTTP 日期。Claude Code 在下一次[自动重试](https://code.claude.com/docs/zh-CN/errors.md#automatic-retries)之前至少等待该时长，在 [`CLAUDE_CODE_RETRY_WATCHDOG`](https://code.claude.com/docs/zh-CN/env-vars) 会话之外，超过 60 的值会停止重试并立即显示错误 |
| `x-should-retry` | 原样转发上游的值。Claude Code 在决定是否重试失败的请求时将此头部作为一个输入来读取：`true` 标记响应可重试，`false` 标记响应不可重试。有关重试次数、退避和 Claude Code 重试的失败情况，请参阅[自动重试](https://code.claude.com/docs/zh-CN/errors.md#automatic-retries) |
| `anthropic-ratelimit-unified-*` | 在每个响应上原样转发上游的值。Claude Code 在成功响应上读取它们以向使用 claude.ai 登录的开发人员显示针对计划限制的使用情况，在 `429` 上读取它们以区分计划限制或支出上限与临时限流；请参阅[使用限制](https://code.claude.com/docs/zh-CN/errors.md#usage-limits) |

## 系统提示归属块

Claude Code 在系统提示前面加上一个短的归属块，其中包含客户端版本和从对话派生的指纹。`api.anthropic.com` 端点在处理前删除该块，因此它不会影响第一方提示缓存。任何其他上游都会将其作为提示的一部分接收。

该删除是位置相关的，因此只有在网关原样转发 `system` 数组时才有效。要在不丢失其他系统内容的情况下将该块排除在提示之外：

* 完全按照接收的方式转发 `system` 数组，保持该块在最前面：在前面加上另一个系统块、重新排序数组或将其转换为单个字符串会破坏删除，该块随后会到达模型和提示缓存键。
* 将该块保留在其自己的数组条目中：端点将以归属标头开头的合并块视为完整的归属，并删除合并到其中的所有内容，包括系统提示的其余部分。
* 如果您的网关必须重新整形系统内容，请设置 [`CLAUDE_CODE_ATTRIBUTION_HEADER=0`](https://code.claude.com/docs/zh-CN/env-vars) 以便 Claude Code 省略该块。Anthropic 和云提供商的 Claude 端点读取该块以进行归属，因此要在客户端省略它，而不是在网关中删除或移动它。

该变量存在是为了网关和第三方缓存兼容性，而不是作为隐私控制：在直接连接上，完整请求无论如何都已经发送到 Anthropic API。当以下两个条件都成立时，Claude Code 即使在您将变量设置为 `0` 时也会在 [auto mode](https://code.claude.com/docs/zh-CN/permission-modes.md#eliminate-prompts-with-auto-mode) 分类器请求上保留该块：

* 请求发送到 `api.anthropic.com`，`ANTHROPIC_BASE_URL` 未设置或命名该主机，且未选择第三方提供商。
* 活跃凭证不是 [Anthropic 配置文件或联合凭证](https://code.claude.com/docs/zh-CN/authentication.md#anthropic-profiles-and-federation-credentials)。

分类器请求跳过 Claude Code 系统提示的其余部分，因此在这些请求上该块是请求体中唯一标识它们为 Claude Code 流量的标记。当任一条件失败时，通过 LLM 网关、在第三方提供商上或使用活跃的配置文件或联合凭证，设置 `0` 也会从分类器请求中删除该块。在 v2.1.229 之前，此例外不存在：设置 `0` 会从这些分类器请求中删除该块，当 API 拒绝未识别的请求时，auto mode 在它发送给分类器的每个操作上都会失败。

从 Claude Code v2.1.181 开始，当请求通过自定义基础 URL 路由时，该块在对话的生命周期内是稳定的，因此以完整请求体为键的网关端提示缓存可以在不禁用它的情况下工作，您的网关转发到的任何提供商都会接收稳定的提示前缀。在 v2.1.181 之前，该块包含每个请求的令牌，在系统提示的开始处改变了每个请求。在这些版本上，当您的网关执行以下任一操作时，请设置 `CLAUDE_CODE_ATTRIBUTION_HEADER=0`：

* 实现以请求体为键的提示缓存。
* 将请求转发到第三方提供商，例如 Amazon Bedrock、Microsoft Foundry 或 Google Cloud 的 Agent Platform，采用 Anthropic Messages 格式或提供商自己的格式，其中变化的前缀会减少该提供商上的提示缓存重用。

## 功能传递

Claude Code 将 `ANTHROPIC_BASE_URL` gateway 视为 Anthropic 格式端点，并向其发送它发送到 `api.anthropic.com` 的 beta 请求头和请求体字段，除了为直接连接保留的一小组诊断和默认值，例如下面涵盖的细粒度工具流式传输默认值。该集合因版本而异，因此不要依赖其内容。

细粒度工具流式传输是直接连接默认值之一：每当请求通过自定义基础 URL 路由时，它默认关闭，当开发者设置 [`CLAUDE_CODE_ENABLE_FINE_GRAINED_TOOL_STREAMING=1`](https://code.claude.com/docs/zh-CN/env-vars) 时，gateway 会接收它。

### Beta 请求头与请求体字段对

添加请求体字段的功能将它们与 beta 请求头配对，该对一起传递。删除请求头同时传递请求体的网关，或将 Anthropic 格式请求体转发到具有不同 schema 的上游，会产生 `400` 错误。当两个部分一起缺失时，功能会安静地关闭。

重写或编辑请求体以进行内容检查的网关会以与删除相同的方式破坏配对，因此请在不修改的情况下检查。该表注明了功能偏离配对的位置。

| 功能 | 请求头和请求体对 | 破坏时的症状 | 补救 |
| :- | :- | :- | :- |
| [自适应推理](https://code.claude.com/docs/zh-CN/model-config.md#adjust-effort-level) | 无 beta 请求头。Claude Code 为 Claude Opus 4.6、Claude Sonnet 4.6、Claude Haiku 5.5、Claude Fable 5 及更高版本的模型发送 `thinking: {"type": "adaptive"}`，并将它不识别的模型名称（如 gateway 别名）视为接收该字段的当前模型 | 当上游模型构建不接受它时，命名 `thinking` 字段或 `adaptive` 标签的 `400` | 升级上游。在 Opus 4.6 和 Sonnet 4.6 上，开发者可以改为设置 `CLAUDE_CODE_DISABLE_ADAPTIVE_THINKING=1` |
| [上下文管理](https://platform.claude.com/docs/en/build-with-claude/context-editing) | 上下文管理 beta 请求头与 `context_management` 请求体字段配对 | `400` 带有 `Extra inputs are not permitted`。常见于网关接受 Anthropic 格式请求但将其转发到 Amazon Bedrock 时。在 v2.1.304 或更高版本上，Claude Code 会[自行在不带该对的情况下重试](#automatic-retry-and-error-forwarding) | 转发两者，或 [`CLAUDE_CODE_DISABLE_EXPERIMENTAL_BETAS=1`](#disable-pre-release-capabilities) |
| [扩展上下文](https://platform.claude.com/docs/en/build-with-claude/context-windows#context-window-sizes-by-model)和[交错思考](https://platform.claude.com/docs/en/build-with-claude/extended-thinking#interleaved-thinking) | 仅 Beta 请求头，无请求体字段 | 当请求头被删除时无声地不可用；上游永远不会看到功能请求 | 逐字转发 `anthropic-beta` |
| Beta [工具字段](https://platform.claude.com/docs/en/agents-and-tools/tool-use/overview) | 工具相关的 beta 请求头与工具 schema 字段（如 `strict` 和 `defer_loading`）配对 | 当请求体通过而没有其请求头时，命名无法识别的工具 schema 字段的 `400`。关于 `defer_loading`，请参阅[不带工具搜索的重试](#retry-without-tool-search) | 转发两者，或 [`CLAUDE_CODE_DISABLE_EXPERIMENTAL_BETAS=1`](#disable-pre-release-capabilities) |
| [努力](https://platform.claude.com/docs/en/build-with-claude/effort)和[结构化输出](https://platform.claude.com/docs/en/build-with-claude/structured-outputs) | `output_config` 请求体字段携带努力、结构化输出格式和任务预算设置；每个都与自己的 beta 请求头配对 | 在 Amazon Bedrock 和 Google Cloud 的 Agent Platform 上游上命名 `output_config` 的 `400`，通常是 `Extra inputs are not permitted` | 一起转发该字段及其请求头，或让开发者设置 [`CLAUDE_CODE_DISABLE_EXPERIMENTAL_BETAS=1`](#disable-pre-release-capabilities)，这会移除格式和任务预算设置，但不会移除努力。若只想移除格式，开发者可以改为设置 [`CLAUDE_CODE_DISABLE_STRUCTURED_OUTPUTS=1`](https://code.claude.com/docs/zh-CN/env-vars)，这需要 v2.1.288 或更高版本 |
| 对话中途的系统消息 | 无 beta 请求头。对于 Claude Opus 4.8、在其之后发布的模型以及无法识别的模型 ID，Claude Code 会将部分提醒和附件文本作为 `messages` 中的 `role: "system"` 条目发送 | `400`，例如 `Unexpected role "system"` | Claude Code 会[在不带系统消息的情况下重试](#automatic-retry-and-error-forwarding)。若要在 Claude Opus 4.8 之前发布的模型的别名上停止系统消息，请在您分发的设置中添加 [`modelOverrides` 条目](#settings-for-unrecognized-model-ids) |
| [按轮次设置的努力](https://code.claude.com/docs/zh-CN/prompt-caching.md#changing-effort-level) | 在 Opus 5.5、Sonnet 5.5、Haiku 5.5 和 Fable 5.1 上，beta 请求头与 `role: "system"` 条目上的 `output_config.effort` 配对，这些条目可以是系统消息，也可以是内容为空的条目 | 当您的网关从空条目中删除该字段时，出现带有 `messages.N: system content must contain at least one block` 的 `400` | 转发两者，或让开发者设置 [`CLAUDE_CODE_DISABLE_EXPERIMENTAL_BETAS=1`](#disable-pre-release-capabilities) 以停止发送该字段和空条目 |
| [提示缓存](https://code.claude.com/docs/zh-CN/prompt-caching.md) | 无 beta 配对。Claude Code 将 `cache_control` 标记附加到 `system` 块和 `messages` 条目，包括在对话中途附加的 `role: "system"` 条目 | 无错误：对话在每个回合都作为未缓存的输入计费，在 `usage` 中可见为高 `input_tokens` 且缓存活动很少或没有 | 在任何地方原封不动地转发 `cache_control`，并且不要将块形式的 `system` 或消息内容转换为纯字符串 |
| [令牌计数](https://platform.claude.com/docs/en/build-with-claude/token-counting) | 无 beta 配对；使用 `count_tokens` 端点 | 无错误：Claude Code 回退到基于字符的估计，因此 `/context` 显示近似计数 | 公开该端点以获得精确的令牌计数 |

`ANTHROPIC_DEFAULT_*_MODEL_SUPPORTED_CAPABILITIES` [变量](https://code.claude.com/docs/zh-CN/model-config.md)仅在提供商配置中声明模型功能：`CLAUDE_CODE_USE_BEDROCK`、`CLAUDE_CODE_USE_VERTEX`、`CLAUDE_CODE_USE_FOUNDRY` 和 [`CLAUDE_CODE_USE_MANTLE`](https://code.claude.com/docs/zh-CN/amazon-bedrock#use-the-mantle-endpoint)。它们在 `ANTHROPIC_BASE_URL` gateway 后面没有效果。

### 自动重试和错误转发

当上游拒绝其中某项功能时，Claude Code 可以自行重试请求，并移除或更改被拒绝的部分：

* 当 `400` 表明不支持 `thinking.type` 值 `enabled` 或 `adaptive` 时，Claude Code 会改用另一种类型重试一次，并保留 `thinking` 字段。在 Claude Code 退出之前，发往该模型的后续请求会保持切换后的类型。在 v2.1.121 之前，Claude Code 不会重试，因此 [`thinking.type.enabled is not supported`](https://code.claude.com/docs/zh-CN/errors#thinking-type-enabled-is-not-supported-for-this-model) 会到达开发者
* 当上游拒绝 `messages` 中的 `role: "system"` 条目时，Claude Code 会在不带这些对话中途系统消息的情况下重试一次，并改为在用户消息中携带其文本。在开发者运行 `/clear` 或对话被压缩之前，该对话中的后续请求会保持这种形式
* 当上游仅拒绝对话中途系统消息上的 `cache_control` 标记时，Claude Code 会保留系统消息，并在移动该标记后重试一次。该标记会放在 `messages` 中最后一个非系统消息的条目上，在该次重试以及 Claude Code 退出之前的后续请求中均是如此
* 当上游拒绝[思考签名](https://platform.claude.com/docs/en/build-with-claude/extended-thinking)时，包括带有 `400` 的拒绝，其消息说该块被 `bound to a different conversation`，Claude Code 会从请求中删除早期思考块，重试，并将其排除在每个后续请求之外。新响应仍然包括思考
* 当 gateway 或其上游将[顾问工具](https://code.claude.com/docs/zh-CN/advisor.md)条目在 `tools` 中拒绝为无法识别的工具类型时，Claude Code 会重试一次请求，不包含该条目及其 `anthropic-beta` 值。对该基础 URL 的后续请求会将顾问排除在外，直到 Claude Code 退出，在该时间内 `/advisor` 对开发者不可用。Claude Code 通过 `400` 或 `422` 响应识别此拒绝，其消息在 `Input tag` 之后命名工具类型，例如 `Input tag 'advisor_20260301'`。在 v2.1.280 之前，Claude Code 没有重试此拒绝
* 当上游拒绝 `output_config.effort` 或其 `anthropic-beta` 值时，Claude Code 会重试，并在 Claude Code 退出之前，不再向发往该模型的请求中添加这两者。如果您的网关原封不动地转发上游的 `400`，且其消息满足以下条件之一，则网关无需任何更改：
  * 同时命名 `output_config.effort` 和 `Extra inputs are not permitted`
  * 说明该模型不支持 effort 参数
  * 以 `Unexpected value(s)` 开头，并针对 `anthropic-beta` 请求头列出 `effort-2025-11-24`。需要 v2.1.310 或更高版本。如果开发者的 [`ANTHROPIC_BETAS`](https://code.claude.com/docs/zh-CN/env-vars) 中列出了该值，他们会继续发送该值并收到 `400`，因此请让他们移除该条目
* 当上游拒绝 `context_management` 字段或其 `anthropic-beta` 值时，网关会再次收到不带它们的该请求。如果网关在后续请求中再次看到它们且上游再次拒绝，则会进行同样的重试。Claude Code 通过消息识别此拒绝，例如包含 `context_management: Extra inputs are not permitted` 的消息。在 v2.1.304 之前，Claude Code 没有重试此拒绝
* 当上游拒绝 `defer_loading` 以外的 beta 工具字段（例如 `strict`）时，`400` 会到达开发者。在此之前，网关可能会再次收到[不带工具搜索字段](#retry-without-tool-search)的该请求。若要停止发送此类字段，请让开发者设置 [`CLAUDE_CODE_DISABLE_EXPERIMENTAL_BETAS=1`](#disable-pre-release-capabilities)

`bound to a different conversation` 拒绝来自 API 的[保留思考](https://platform.claude.com/docs/en/build-with-claude/preserved-thinking)检查，当 `system`、`tools` 或早期 `messages` 内容与产生思考的请求不同时，该检查失败。重写任何该内容的 gateway 可能会导致拒绝本身；[库、代理和网关](https://platform.claude.com/docs/en/build-with-claude/preserved-thinking#libraries-proxies-gateways)涵盖了要原封不动地传递的内容。

大部分重试逻辑与上游的错误措辞匹配，因此请原封不动地转发错误响应体。将上游错误包装在自己的信封中的网关会破坏恢复路径，即使它保留了状态代码，除非信封的消息携带稳定的 `capability_rejected:` 标记。[Claude apps gateway 用这些标记替换云提供商的错误措辞](https://code.claude.com/docs/zh-CN/claude-apps-gateway-config.md#upstream-error-messages)，例如 `capability_rejected: prompt_too_long`。

#### 不带工具搜索的重试

在 v2.1.305 或更高版本上，携带 [`defer_loading` 字段和 `tool_reference` 块](https://code.claude.com/docs/zh-CN/mcp#tool-search-through-a-gateway)的请求，在出现 Claude Code 无法识别原因的 `400` 或 `422` 之后，可能会不带它们再次到达。重试成功后，后续请求也会省略它们。

开发者的请求无需对网关做任何更改即可继续正常工作，其 MCP 工具会预先加载。Claude Code 稍后会再次尝试工具搜索，因此网关可能会再次看到被拒绝的请求及其后的重试。若要在所有请求中都排除这些字段，请让开发者设置 `ENABLE_TOOL_SEARCH=false`。对于工具搜索本身，Claude Code 期望网关将 `defer_loading` 字段和 `tool_reference` 块连同其 `anthropic-beta` 值一起转发。

### 禁用预发布功能

当您的网关或其上游拒绝预发布的 `anthropic-beta` 值或与之配对的请求体字段，并且您无法同时转发两个部分时，请让开发者设置 `CLAUDE_CODE_DISABLE_EXPERIMENTAL_BETAS=1`。设置该变量后，Claude Code 将停止发送预发布功能以及与之配对的 `anthropic-beta` 值，包括：

* 上下文管理及其 `context_management` 请求体字段
* beta 工具 schema 字段，例如 `strict` 和 `defer_loading`。标准的 `name`、`description`、`input_schema` 和 `cache_control` 工具字段会保留
* 结构化输出 `output_config.format` 字段。需要 Claude Code v2.1.287 或更高版本
* `output_config.task_budget` 字段
* [MCP 工具搜索](https://code.claude.com/docs/zh-CN/mcp.md#scale-with-mcp-tool-search)，因此每个 MCP 工具都会预先加载，除非您的组织通过托管设置保持其开启

该变量不会删除所有 `anthropic-beta` 值。它保留的内容包括：

* 扩展上下文、交错思考和努力的 `anthropic-beta` 值，云提供商也接受这些值
* 顶层的 `output_config.effort` 字段。[自动重试和错误转发](#automatic-retry-and-error-forwarding)介绍了上游拒绝它的情况
* 自适应推理的 `thinking` 字段，它没有 beta 请求头
* 携带[对话中途系统消息](#feature-pass-through)的 `role: "system"` 条目
* 订阅身份验证所需的 OAuth `anthropic-beta` 值
* 开发者通过 [`ANTHROPIC_BETAS`](https://code.claude.com/docs/zh-CN/env-vars) 或 [`CLAUDE_CODE_EXTRA_BODY`](https://code.claude.com/docs/zh-CN/env-vars) 自行添加的请求头值和请求体字段

当嵌入 Claude Code 的主机平台设置 [`CLAUDE_CODE_PROVIDER_MANAGED_BY_HOST`](https://code.claude.com/docs/zh-CN/env-vars) 时，`CLAUDE_CODE_DISABLE_EXPERIMENTAL_BETAS` 不会阻止 Amazon Bedrock、Google Cloud 的 Agent Platform、Microsoft Foundry 或 [Claude apps gateway](https://code.claude.com/docs/zh-CN/claude-apps-gateway) 上的自动模式会话向服务器请求[分类器审查](https://code.claude.com/docs/zh-CN/permission-modes.md#server-side-classifier-review)。该审查添加了 `anthropic-beta` 值和 `safeguards` 请求字段。设置 `CLAUDE_CODE_AUTO_MODE_SERVER=0` 以在那里停止它。

在 Claude Code v2.1.227 或更高版本上，您的组织可以通过[托管设置](https://code.claude.com/docs/zh-CN/managed-settings.md)在此变量下保持 [MCP 工具搜索](https://code.claude.com/docs/zh-CN/mcp.md#scale-with-mcp-tool-search)打开。Claude Code 在该覆盖生效时发送的内容取决于您如何连接：

* 在未使用 HIPAA 配置的直接连接上，或通过设置了 `ANTHROPIC_BASE_URL` 的网关，Claude Code 继续发送工具搜索 beta 请求头、`defer_loading` 工具字段和 `tool_reference` 块
* 在云提供商上，或通过 [Claude apps gateway](https://code.claude.com/docs/zh-CN/claude-apps-gateway) 登录，覆盖没有效果
* 在[获得 HIPAA 配置](https://code.claude.com/docs/zh-CN/hipaa-setup.md#check-how-developers-sign-in-and-connect)的会话中，覆盖没有效果，工具搜索保持关闭

Claude Code 发送的功能集随版本增长。有关当前 beta 请求头字符串，请参阅 [beta 请求头参考](https://platform.claude.com/docs/en/api/beta-headers)；针对新的 Claude Code 版本测试您的 gateway，而不是固定到观察列表。

## 模型发现

当 `ANTHROPIC_BASE_URL` 指向公开 Anthropic Messages 格式的 gateway 时，Claude Code 可以在启动时查询 gateway 的 `/v1/models` 端点，并将返回的模型添加到 `/model` 选择器。如果您或您的管理员在 [`modelPicker`](https://code.claude.com/docs/zh-CN/settings-reference#modelpicker) 配置中设置了 `replaceBuiltInOptions`，Claude Code 会从选择器中隐藏发现的模型。

开发者通过在自己的环境中或通过托管设置设置 [`CLAUDE_CODE_ENABLE_GATEWAY_MODEL_DISCOVERY=1`](https://code.claude.com/docs/zh-CN/env-vars) 来启用它。发现默认关闭，以便由共享 API 密钥支持的 gateway 不会向每个用户公开密钥可以访问的每个模型。

### 发现何时运行

发现仅适用于 Anthropic Messages 格式。在以下情况下不运行：

* 设置了任何 `CLAUDE_CODE_USE_*` 提供商变量，即使也设置了 `ANTHROPIC_BASE_URL`
* `ANTHROPIC_BASE_URL` 未设置或指向 `api.anthropic.com`

当[非必要流量被禁用](https://code.claude.com/docs/zh-CN/llm-gateway-connect.md#turn-off-traffic-outside-the-gateway-path)时，发现仍然运行，因为请求仅发送到您的 gateway。在 v2.1.257 之前，非必要流量被禁用时发现不运行。

### 请求和响应

请求是 `GET /v1/models?limit=1000`，超时为 3 秒，任何重定向都被视为失败，因此凭证不会泄露到重定向目标。响应缓慢或重定向 `/v1/models` 的 gateway，即使是 `http` 到 `https`，也会无声地失败发现；在配置的基础 URL 处直接提供端点。

要给缓慢的 gateway 更长的时间，请设置 [`CLAUDE_CODE_GATEWAY_MODEL_DISCOVERY_TIMEOUT_MS`](https://code.claude.com/docs/zh-CN/env-vars#variables)。该变量需要 Claude Code v2.1.269 或更高版本。

Claude Code 使用下面两个凭证请求头发送发现请求，并省略其值无法解析的请求头。发送两个请求头需要 Claude Code v2.1.248 或更高版本。早期版本在设置了 `ANTHROPIC_AUTH_TOKEN` 时仅发送 `Authorization`，否则仅发送 `x-api-key`。

* `Authorization`：`ANTHROPIC_AUTH_TOKEN` 作为承载令牌，否则 [`apiKeyHelper`](https://code.claude.com/docs/zh-CN/llm-gateway-connect#rotate-credentials-with-apikeyhelper) 值作为承载令牌。
* `x-api-key`：Claude Code 解析的 API 密钥，例如 `ANTHROPIC_API_KEY`。当助手值是唯一的凭证时，此请求头也会携带它，因此该值会在两个请求头中到达。

Claude Code 还发送来自 `ANTHROPIC_CUSTOM_HEADERS` 的任何请求头。当自定义请求头具有非空值时，Claude Code 会发送它来代替同名的内置请求头，不区分大小写地匹配名称。

当两个凭证请求头的值都无法解析时，Claude Code 会跳过发现，并在 `claude --debug` 会话的调试日志中写入 `[gatewayDiscovery] skipped` 行。如果您仅通过 `ANTHROPIC_CUSTOM_HEADERS` 提供凭证，Claude Code 仍然会跳过发现。

Claude Code 从响应的 `data` 数组中的每个条目读取 `id`、可选的 `display_name` 和可选的 `description`：

```json theme={null}
{
  "data": [
    {
      "id": "claude-sonnet-4-6",
      "display_name": "Claude Sonnet 4.6",
      "description": "Default model for everyday coding tasks"
    },
    { "id": "claude-opus-4-8" }
  ]
}
```

Claude Code 在其 `id` 中任何位置包含 `claude` 或 `anthropic` 的条目会被保留，不区分大小写，其余的会被忽略。提供商前缀的 ID，例如 `vertex_ai/claude-sonnet-4-6` 或 `bedrock/anthropic.claude-sonnet-4-5` 会通过过滤器；不包含任何一个子字符串的 ID 则不会。在 v2.1.223 之前，Claude Code 仅在其 `id` 以 `claude` 或 `anthropic` 开头时保留条目，这隐藏了提供商前缀的 ID。

### 选择器条目和缓存

选择器是当开发者在 Claude Code 中运行 `/model` 时打开的交互式模型列表。每个发现的条目在 gateway 发送与 `id` 不同的 `display_name` 时使用 `display_name` 作为其名称。否则，当 Claude Code [识别 `id`](https://code.claude.com/docs/zh-CN/model-config#customize-pinned-model-display-and-capabilities) 时，条目显示模型的名称，当不识别时显示 `id`。例如，具有 `id` `my-gateway-claude-sonnet-4-6` 且没有 `display_name` 的条目显示为 `Sonnet 4.6`。

发现仅添加 [`availableModels` 托管设置](https://code.claude.com/docs/zh-CN/settings-reference#availablemodels) 允许的模型。

每个条目还显示模型的 `description`，折叠为一行。没有 `description` 的条目改为显示"From gateway"。在 v2.1.257 之前，每个发现的条目都显示"From gateway"。

当发现的 ID 与选择器中已有的行匹配时，它不会获得自己的行：

* 相同 ID：发现的 ID 完全匹配现有行的 ID，或两个 ID 是同一 [Fable](https://code.claude.com/docs/zh-CN/model-config.md#work-with-fable) 版本的拼写。
* 与内置别名相同的模型：当发现的显式 ID 命名内置别名当前解析到的模型时，选择器仅显示别名行。例如，当 `sonnet` 解析为 `claude-sonnet-5-5` 时，发现的 `claude-sonnet-5-5` 会折叠到 `sonnet` 行中，而发现的 `claude-sonnet-5` 仍会获得自己的行。

结果被缓存到 `~/.claude/cache/gateway-models.json`，或在 Windows 上 `%USERPROFILE%\.claude\cache\gateway-models.json`，并在每次启动时刷新。如果您设置了 [`CLAUDE_CONFIG_DIR`](https://code.claude.com/docs/zh-CN/env-vars)，缓存会改为位于该目录下。如果请求失败或 gateway 未实现 `/v1/models`，选择器会回退到上次启动的缓存列表或内置模型列表。如果您的 gateway 在不匹配发现过滤器的别名下提供 Claude 模型，开发者可以使用[模型配置](https://code.claude.com/docs/zh-CN/model-config.md)变量手动添加这些别名。

## 相关资源

有关 gateway 文档集的其余部分和基础 API 参考：

* [Gateway 概述](https://code.claude.com/docs/zh-CN/gateways.md)：什么是 gateway 以及如何在 Claude 应用 gateway 和其他产品之间进行选择
* [其他 LLM gateway](https://code.claude.com/docs/zh-CN/llm-gateway)：如何推出您的组织运行的 gateway 以及它如何与 claude.ai 订阅交互
* [为您的组织推出 LLM gateway](https://code.claude.com/docs/zh-CN/llm-gateway-rollout.md)：使用此指南的管理员检查清单
* [将 Claude Code 连接到 LLM gateway](https://code.claude.com/docs/zh-CN/llm-gateway-connect.md)：每个开发者的配置和故障排除表
* [Beta 请求头参考](https://platform.claude.com/docs/en/api/beta-headers)：当前的 `anthropic-beta` 值集
* [Messages API](https://platform.claude.com/docs/en/api/messages)：Anthropic 格式 gateway 实现的 API 格式
