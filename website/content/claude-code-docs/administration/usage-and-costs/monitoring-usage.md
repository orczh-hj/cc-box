> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# 监控

> 了解如何为 Claude Code 启用和配置 OpenTelemetry。

通过 OpenTelemetry (OTel) 导出遥测数据，跨组织跟踪 Claude Code 使用情况、成本和工具活动。Claude Code 通过标准指标协议导出指标作为时间序列数据，通过日志/事件协议导出事件，并可选地导出[通过 OpenTelemetry 的分布式跟踪](#traces-beta)。

> [!NOTE]
>
> 如需了解 Claude API 或 Managed Agents，请参阅 [Claude API 的 OpenTelemetry](https://platform.claude.com/docs/en/manage-claude/opentelemetry)。

## 快速开始

使用环境变量配置 OpenTelemetry：

```bash theme={null}
# 1. 启用遥测
export CLAUDE_CODE_ENABLE_TELEMETRY=1

# 2. 选择导出器（两者都是可选的 - 仅配置您需要的）
export OTEL_METRICS_EXPORTER=otlp       # 选项：otlp、prometheus、console、none
export OTEL_LOGS_EXPORTER=otlp          # 选项：otlp、console、none

# 3. 配置 OTLP 端点（用于 OTLP 导出器）
export OTEL_EXPORTER_OTLP_PROTOCOL=grpc
export OTEL_EXPORTER_OTLP_ENDPOINT=http://localhost:4317

# 4. 设置身份验证（如果需要）
export OTEL_EXPORTER_OTLP_HEADERS="Authorization=Bearer your-token"

# 5. 用于调试：减少导出间隔，并为生产使用重置它们
export OTEL_METRIC_EXPORT_INTERVAL=10000  # 10 秒（默认：60000ms）
export OTEL_LOGS_EXPORT_INTERVAL=5000     # 5 秒（默认：5000ms）

# 6. 运行 Claude Code
claude
```

要验证导出指标的设置，请检查您的后端是否有 `claude_code.session.count` 指标，Claude Code 在会话启动时会发出该指标。要验证仅日志的设置，请提交提示并检查 `claude_code.user_prompt` 事件。

如果没有任何内容到达，请使用 `claude --debug-file <path>` 启动 Claude Code 并检查它写入该路径的日志。Claude Code 将您配置的导出器的失败报告为 `[3P telemetry]` 错误，其中 3P 表示第三方。以 `[Anthropic telemetry]` 为前缀的行描述 [Anthropic 的单独操作遥测](https://code.claude.com/docs/zh-CN/data-usage.md#telemetry-services)，不表示您的设置存在问题。

有关完整配置选项，请参阅 [OpenTelemetry 规范](https://github.com/open-telemetry/opentelemetry-specification/blob/main/specification/protocol/exporter.md#configuration-options)。

## 管理员配置

管理员可以通过 [托管设置文件](https://code.claude.com/docs/zh-CN/managed-settings.md#delivery-mechanisms) 为所有用户配置 OpenTelemetry 设置。有关设置如何应用的更多信息，请参阅 [设置优先级](https://code.claude.com/docs/zh-CN/settings.md#settings-precedence)。

示例托管设置配置：

```json theme={null}
{
  "env": {
    "CLAUDE_CODE_ENABLE_TELEMETRY": "1",
    "OTEL_METRICS_EXPORTER": "otlp",
    "OTEL_LOGS_EXPORTER": "otlp",
    "OTEL_EXPORTER_OTLP_PROTOCOL": "grpc",
    "OTEL_EXPORTER_OTLP_ENDPOINT": "http://collector.example.com:4317",
    "OTEL_EXPORTER_OTLP_HEADERS": "Authorization=Bearer example-token"
  }
}
```

在 Claude Desktop 应用中，Code 标签页会话从 [到达每种 Desktop 会话的源](https://code.claude.com/docs/zh-CN/desktop.md#managed-settings) 读取这些托管设置。Cowork 在管理员控制台的 [数据和隐私设置](https://claude.ai/admin-settings/data-privacy-controls) 中的 **监控** 下的 OpenTelemetry 表单仅适用于 Cowork 会话，因此终端 CLI 和 Code 标签页都不会导出到您在那里设置的收集器。

Claude Code 忽略存储库的 `.claude/settings.json` 和 `.claude/settings.local.json` 中的 [OpenTelemetry 导出器变量](https://code.claude.com/docs/zh-CN/settings-reference.md#variables-claude-code-ignores-in-env)，因此存储库无法使用它们来打开遥测、选择其去向或捕获内容。在托管设置中设置它们，或让每个开发者在其 shell 或 `~/.claude/settings.json` 中设置它们。存储库仍然可以通过将其导出器选择器（如 `OTEL_LOGS_EXPORTER`）设置为 `none` 来关闭信号，除非托管设置、`--settings` 文件或启动 Claude Code 的环境设置了该变量。

Claude Code 不会将 `OTEL_*` 环境变量传递给它生成的子进程，包括 Bash 工具、hooks、MCP 服务器和语言服务器。通过 Bash 工具运行的已进行 OpenTelemetry 检测的应用程序不会继承 Claude Code 的导出器端点或标头，因此如果该应用程序需要导出自己的遥测，请直接在命令中设置这些变量。

### 托管设置如何锁定 OTLP 目标

当您在托管设置中设置 `OTEL_EXPORTER_OTLP_*` 变量时，Claude Code 会在启动时删除冲突的开发者设置变量，并在调试日志中记录一条警告。它删除的内容取决于您设置的变量：

* **端点**：当您设置 `OTEL_EXPORTER_OTLP_ENDPOINT` 时，Claude Code 会删除每个开发者设置的每信号端点。开发者无法将一个信号指向不同的收集器，因此您不需要在托管设置中也设置每信号端点变量。
* **协议**：当您设置 `OTEL_EXPORTER_OTLP_PROTOCOL` 时，Claude Code 会删除每个开发者设置的每信号协议。
* **凭证**：当您设置 `OTEL_EXPORTER_OTLP_HEADERS`、`OTEL_EXPORTER_OTLP_CLIENT_KEY` 或 `OTEL_EXPORTER_OTLP_CLIENT_CERTIFICATE` 时，Claude Code 会删除开发者设置的该变量的每信号版本，以及每个开发者设置的端点变量（通用或每信号），因为这些凭证否则会到达托管设置未选择的收集器。
* **导出器选择器**：`OTEL_METRICS_EXPORTER`、`OTEL_LOGS_EXPORTER` 和测试版 `OTEL_TRACES_EXPORTER` 遵循正常的每键优先级。开发者的设置仍然可以禁用信号或将其切换到控制台导出器，因此如果您需要锁定选择器，也请在托管设置中设置它们。在 [管理员源](https://code.claude.com/docs/zh-CN/managed-settings.md#precedence-within-the-managed-tier) 中，`OTEL_LOGS_EXPORTER` 遵循 [遥测单元](https://code.claude.com/docs/zh-CN/server-managed-settings.md#per-key-exceptions-across-managed-sources)，而其他两个选择器按键合并。需要 Claude Code v2.1.223 或更高版本。
* **测试版追踪端点**：当 [详细测试版追踪](#traces-beta) 处于活动状态时，Claude Code 将日志和追踪导出到 `BETA_TRACING_ENDPOINT` 而不是通过日志和追踪导出器。因此，当这些托管设置中的任何一个决定任一信号的目标时，Claude Code 会删除开发者设置的 `BETA_TRACING_ENDPOINT`：

  * 通用或日志/追踪端点或凭证
  * 一个 [`otelHeadersHelper`](https://code.claude.com/docs/zh-CN/settings-reference#otelheadershelper)
  * 日志或追踪导出器选择器设置为 `none`、`console` 或空，这些值使信号远离收集器
  * `CLAUDE_CODE_ENABLE_TELEMETRY` 关闭

  仅限指标的端点或凭证不会删除它。在 v2.1.251 之前，开发者设置的 `BETA_TRACING_ENDPOINT` 会重定向详细测试版追踪导出的日志和追踪，即使托管设置固定了收集器。

Claude Code 不会删除您在托管设置中自己设置的每信号变量，因此您可以通过在那里设置其变量来将一个信号路由到不同的收集器，如 [SIEM 示例](#send-events-to-a-siem) 所示。如果您在那里设置每信号凭证，Claude Code 会删除该信号的开发者设置端点。

此删除行为改变了遥测的传递位置，而不是 Claude Code 收集的内容。

在 v2.1.217 之前，每个变量独立遵循每键设置优先级，因此在用户设置或 shell 中设置的信号特定端点会将该信号重定向到离开托管收集器。

当桌面应用或 [自托管环境](https://code.claude.com/docs/zh-CN/self-hosted-environments) 运行器启动 Claude Code 并在其提供的环境中命名 OTLP 端点时，Claude Code 以相同的方式固定目标：启动器的遥测变量删除开发者设置的变量，就像托管设置一样。Claude Code 不会删除启动器本身设置的变量。需要 Claude Code v2.1.251 或更高版本。

## 配置详情

### 常见配置变量

这些变量为所有部署配置导出器、端点和导出行为。

如果您设置了按信号的端点或协议变量，例如 `OTEL_EXPORTER_OTLP_METRICS_ENDPOINT`，Claude Code 会使用它而不是该信号的通用变量。如果您设置了按信号的标头变量，例如 `OTEL_EXPORTER_OTLP_METRICS_HEADERS`，Claude Code 会将其与该信号的通用 `OTEL_EXPORTER_OTLP_HEADERS` 合并。

在具有托管设置的机器上，请参阅[托管设置如何锁定 OTLP 目标](#how-managed-settings-lock-the-otlp-destination)了解 Claude Code 删除的内容。

| 环境变量 | 描述 | 示例值 |
| - | - | - |
| `CLAUDE_CODE_ENABLE_TELEMETRY` | 启用遥测收集（必需） | `1` |
| `OTEL_METRICS_EXPORTER` | 指标导出器类型，以逗号分隔。使用 `none` 禁用 | `console`、`otlp`、`prometheus`、`none` |
| `OTEL_LOGS_EXPORTER` | 日志/事件导出器类型，以逗号分隔。使用 `none` 禁用 | `console`、`otlp`、`none` |
| `OTEL_EXPORTER_OTLP_PROTOCOL` | OTLP 导出器的协议，适用于所有信号。Claude Code 没有默认协议，因此请为启用的每个 `otlp` 导出器设置此变量或按信号的协议变量 | `grpc`、`http/json`、`http/protobuf` |
| `OTEL_EXPORTER_OTLP_ENDPOINT` | 所有信号的 OTLP 收集器端点 | `http://localhost:4317` |
| `OTEL_EXPORTER_OTLP_METRICS_PROTOCOL` | 指标协议，覆盖常规设置 | `grpc`、`http/json`、`http/protobuf` |
| `OTEL_EXPORTER_OTLP_METRICS_ENDPOINT` | OTLP 指标端点，覆盖常规设置 | `http://localhost:4318/v1/metrics` |
| `OTEL_EXPORTER_OTLP_LOGS_PROTOCOL` | 日志协议，覆盖常规设置 | `grpc`、`http/json`、`http/protobuf` |
| `OTEL_EXPORTER_OTLP_LOGS_ENDPOINT` | OTLP 日志端点，覆盖常规设置 | `http://localhost:4318/v1/logs` |
| `OTEL_EXPORTER_OTLP_HEADERS` | OTLP 的身份验证标头 | `Authorization=Bearer token` |
| `OTEL_EXPORTER_OTLP_METRICS_HEADERS` | 指标的身份验证标头，与常规标头合并 | `Authorization=Bearer token` |
| `OTEL_EXPORTER_OTLP_LOGS_HEADERS` | 日志的身份验证标头，与常规标头合并 | `Authorization=Bearer token` |
| `OTEL_METRIC_EXPORT_INTERVAL` | 导出间隔（毫秒）（默认值：60000） | `5000`、`60000` |
| `OTEL_LOGS_EXPORT_INTERVAL` | 日志导出间隔（毫秒）（默认值：5000） | `1000`、`10000` |
| `OTEL_LOG_USER_PROMPTS` | 启用用户提示内容的日志记录（默认值：禁用） | `1` 启用 |
| `OTEL_LOG_ASSISTANT_RESPONSES` | 在 `assistant_response` 事件上启用助手响应文本的日志记录（默认值：禁用）。未设置时，回退到 `OTEL_LOG_USER_PROMPTS` 的值 | `1` 启用，`0` 保持脱敏 |
| `OTEL_LOG_TOOL_DETAILS` | 在工具事件和跟踪跨度属性中启用工具参数和输入参数的日志记录：Bash 命令、MCP 服务器和工具名称、技能名称、用户编写的工作流名称和工具输入。还在 `user_prompt` 事件上启用自定义、插件和 MCP 命令名称，以及在[成本和令牌计数器](#cost-counter)上启用真实代理、技能、插件和 MCP 服务器和工具名称（默认值：禁用）。对于 Claude Desktop 的内置服务器，在 Claude Desktop 拥有的会话中，即使关闭标志，`mcp_server_name`/`mcp_tool_name` 也会在 `tool_decision`/`tool_result` 上发出。该异常需要 Claude Code v2.1.214 或更高版本 | `1` 启用 |
| `OTEL_LOG_TOOL_CONTENT` | 在 [`tool.output` 跨度事件](#tool-output-span-event)中启用工具内容的日志记录（默认值：禁用）。跨度属性在[其自己的门](#new-context-gates)下携带工具内容。需要[跟踪](#traces-beta)。内容在内容限制处截断（默认值 60 KB） | `1` 启用 |
| `OTEL_LOG_MANAGED_SETTINGS` | 将编辑的托管设置和编辑前设置的 SHA-256 摘要添加到[托管设置已解决](#managed-settings-resolved-event)事件（默认值：禁用）。项目或本地设置中的值不会将其打开。需要 Claude Code v2.1.274 或更高版本 | `1` 启用 |
| `OTEL_LOG_RAW_API_BODIES` | 将完整的 Anthropic Messages API 请求和响应 JSON 作为 `api_request_body` / `api_response_body` 日志事件发出（默认值：禁用）。正文包括整个对话历史记录。启用此选项意味着同意 `OTEL_LOG_USER_PROMPTS`、`OTEL_LOG_TOOL_DETAILS` 和 `OTEL_LOG_TOOL_CONTENT` 会揭示的所有内容 | `1` 表示在内容限制处截断的内联正文（默认值 60 KB），或 `file:<dir>` 表示磁盘上的未截断正文，事件中有 `body_ref` 指针 |
| `CLAUDE_CODE_OTEL_CONTENT_MAX_LENGTH` | 内容限制：内容承载属性（如模型响应、工具内容、系统提示词和原始 API 正文）的最大长度，包括截断标记，以 UTF-16 代码单位为单位（默认值：61440，即 60 KB）。默认值针对将属性值上限设为 64 KB 的后端进行了调整；仅当您的后端接受更大的值时才提高它，或降低它以减少遥测量。当设置了 OpenTelemetry SDK 属性限制 `OTEL_ATTRIBUTE_VALUE_LENGTH_LIMIT` 或其日志记录和跨度变体之一且其值更低时，Claude Code 在该较小的值处截断，以便 `[TRUNCATED ...]` 标记保持在 SDK 限制内 | `262144` |
| `OTEL_EXPORTER_OTLP_METRICS_TEMPORALITY_PREFERENCE` | 指标时间性偏好（默认值：`delta`）。如果您的后端期望累积时间性，请设置为 `cumulative` | `delta`、`cumulative` |
| `CLAUDE_CODE_OTEL_HEADERS_HELPER_DEBOUNCE_MS` | 刷新动态标头的间隔（默认值：1740000ms / 29 分钟） | `900000` |

对于 `http/protobuf` 和 `http/json` 协议，Claude Code 使用 `Content-Length` 标头发送每个导出请求。

### mTLS 身份验证

您如何为 OTLP 导出器配置客户端证书取决于用于该信号的 OTLP 协议，通过 `OTEL_EXPORTER_OTLP_PROTOCOL` 或按信号的覆盖设置。相同的配置适用于指标、日志和跟踪。

| 协议 | 客户端证书变量 | 信任收集器的 CA 使用 |
| :- | :- | :- |
| `http/protobuf`、`http/json` | `CLAUDE_CODE_CLIENT_CERT`、`CLAUDE_CODE_CLIENT_KEY` 和可选的 `CLAUDE_CODE_CLIENT_KEY_PASSPHRASE`。请参阅[网络配置](https://code.claude.com/docs/zh-CN/network-config.md#mtls-authentication) | `NODE_EXTRA_CA_CERTS` |
| `grpc` | `OTEL_EXPORTER_OTLP_CLIENT_KEY` 和 `OTEL_EXPORTER_OTLP_CLIENT_CERTIFICATE`，或按信号的变体，例如 `OTEL_EXPORTER_OTLP_METRICS_CLIENT_KEY` 以对每个信号使用不同的证书 | `OTEL_EXPORTER_OTLP_CERTIFICATE` |

对于 `grpc`，OpenTelemetry SDK 直接读取标准 OTLP 变量，因此设置按信号指标变量的现有配置继续有效。在具有托管设置的机器上，Claude Code [可能在启动时删除开发人员设置的按信号凭证和端点](#how-managed-settings-lock-the-otlp-destination)。

### 指标基数控制

以下环境变量控制指标中包含哪些属性以管理基数：

| 环境变量 | 描述 | 默认值 | 禁用示例 |
| - | - | - | - |
| `OTEL_METRICS_INCLUDE_SESSION_ID` | 在指标中包含 session.id 和云会话上的 ccr.session.id 属性 | `true` | `false` |
| `OTEL_METRICS_INCLUDE_VERSION` | 在指标中包含 app.version 属性 | `false` | `true` |
| `OTEL_METRICS_INCLUDE_ACCOUNT_UUID` | 在指标中包含 user.account\_uuid 和 user.account\_id 属性 | `true` | `false` |
| `OTEL_METRICS_INCLUDE_ENTRYPOINT` | 在指标中包含 app.entrypoint 属性 | `false` | `true` |
| `OTEL_METRICS_INCLUDE_RESOURCE_ATTRIBUTES` | 将 `OTEL_RESOURCE_ATTRIBUTES` 中的键作为属性包含在指标数据点上 | `true` | `false` |
| `OTEL_METRICS_INCLUDE_REPOSITORY` | 在指标和事件上包含 `vcs.*` [存储库身份属性](#repository-attributes)。需要 Claude Code v2.1.269 或更高版本 | `false` | `true` |

较低的基数通常意味着更好的性能和更低的存储成本，但分析的数据粒度较低。

### Traces（测试版）

分布式跟踪导出跨度，将每个用户提示链接到它触发的 API 请求和工具执行，因此您可以在跟踪后端中将完整请求视为单个跟踪。

跟踪默认关闭。要启用它，请同时设置 `CLAUDE_CODE_ENABLE_TELEMETRY=1` 和 `CLAUDE_CODE_ENHANCED_TELEMETRY_BETA=1`，然后设置 `OTEL_TRACES_EXPORTER` 以选择跨度的发送位置。跟踪重用[常见 OTLP 配置](#common-configuration-variables)用于端点、协议、标头和 [mTLS](#mtls-authentication)。在具有托管设置的机器上，Claude Code [可能在启动时删除开发人员设置的按信号凭证和端点](#how-managed-settings-lock-the-otlp-destination)。

| 环境变量 | 描述 | 示例值 |
| - | - | - |
| `CLAUDE_CODE_ENHANCED_TELEMETRY_BETA` | 启用跨度跟踪（必需）。也接受 `ENABLE_ENHANCED_TELEMETRY_BETA` | `1` |
| `OTEL_TRACES_EXPORTER` | 跟踪导出器类型，以逗号分隔。使用 `none` 禁用 | `console`、`otlp`、`none` |
| `OTEL_EXPORTER_OTLP_TRACES_PROTOCOL` | 跟踪协议，覆盖 `OTEL_EXPORTER_OTLP_PROTOCOL` | `grpc`、`http/json`、`http/protobuf` |
| `OTEL_EXPORTER_OTLP_TRACES_ENDPOINT` | OTLP 跟踪端点，覆盖 `OTEL_EXPORTER_OTLP_ENDPOINT` | `http://localhost:4318/v1/traces` |
| `OTEL_EXPORTER_OTLP_TRACES_HEADERS` | 跟踪的身份验证标头，与 `OTEL_EXPORTER_OTLP_HEADERS` 合并 | `Authorization=Bearer token` |
| `OTEL_TRACES_EXPORT_INTERVAL` | 跨度批导出间隔（毫秒）（默认值：5000） | `1000`、`10000` |

跨度默认编辑用户提示文本、工具输入详情和工具内容。设置 `OTEL_LOG_USER_PROMPTS=1`、`OTEL_LOG_TOOL_DETAILS=1` 和 `OTEL_LOG_TOOL_CONTENT=1` 以包含它们。

当跟踪处于活动状态时，Bash 和 PowerShell 子进程会自动继承包含活动工具执行跨度的 W3C 跟踪上下文的 `TRACEPARENT` 环境变量。这允许任何读取 `TRACEPARENT` 的子进程在同一跟踪下将其自己的跨度作为父级，通过 Claude 运行的脚本和命令启用端到端分布式跟踪。

当跟踪处于活动状态且 Claude Code 直接连接到 Anthropic API 时，每个模型请求都携带设置为 `claude_code.llm_request` 跨度上下文的 W3C `traceparent` 标头，API 的 `traceresponse` 标头被记录为跨度链接。这些一起通过任何兼容的中介将 Claude Code 的客户端跨度连接到服务器端跟踪。出站 HTTP MCP 请求以相同方式携带 `traceparent`。标头不会发送给第三方提供商。

默认情况下，模型和 HTTP MCP 请求上的 `traceparent` 标头仅在 `ANTHROPIC_BASE_URL` 未设置或指向 Anthropic API 时发送，因为某些代理拒绝无法识别的标头。子进程 `TRACEPARENT` 变量由相同的开关控制以保持一致性。如果您通过自定义 `ANTHROPIC_BASE_URL` 代理运行 Claude Code 并希望传播跟踪上下文，请设置 `CLAUDE_CODE_PROPAGATE_TRACEPARENT=1`。

在 Agent SDK 和使用 `-p` 启动的非交互式会话中，Claude Code 还在启动每个交互跨度时从其自己的环境中读取 `TRACEPARENT` 和 `TRACESTATE`。这允许嵌入过程将其活动 W3C 跟踪上下文传递到子进程中，以便 Claude Code 的跨度显示为调用者分布式跟踪的子级。交互式会话忽略入站 `TRACEPARENT` 以避免意外继承来自 CI 或容器环境的环境值。

入站跟踪上下文也适用于[事件](#events)。在设置了 `TRACEPARENT` 的 Agent SDK 和 `-p` 会话中，每个 OTLP 事件日志记录都携带 `trace_id` 和 `span_id` 值，将其加入您的应用程序跟踪，即使未配置跟踪导出器，您的日志后端也可以将事件与跟踪的其余部分关联。

在交互处于活动状态时发出的记录携带交互跨度的 ID，即使 Claude Code 在跨度的异步上下文之外发出它，例如在权限提示回调中或对在启动期间缓冲并稍后导出的记录。在没有活动交互跨度的情况下发出的记录直接携带入站 `TRACEPARENT` ID。

#### 跨度层次结构

每个用户提示启动一个 `claude_code.interaction` 根跨度。API 调用、工具调用和钩子执行被记录为其子级。工具跨度有两个自己的子跨度：一个用于等待权限决定的时间，一个用于执行本身。当 Agent 工具或旧版 Task 工具生成子代理时，子代理的 API 和工具跨度嵌套在父级的 `claude_code.tool` 跨度下。

```text theme={null}
claude_code.interaction
├── claude_code.llm_request
├── claude_code.hook                    (requires detailed beta tracing)
└── claude_code.tool
    ├── claude_code.tool.blocked_on_user
    ├── claude_code.tool.execution
    └── (Agent tool) subagent claude_code.llm_request / claude_code.tool spans
```

在 Agent SDK 和 `claude -p` 会话中，当在环境中设置 `TRACEPARENT` 时，`claude_code.interaction` 本身成为调用者跨度的子级。

当 `PreToolUse` 钩子[延迟工具调用](https://code.claude.com/docs/zh-CN/hooks.md#defer-a-tool-call-for-later)时，Claude Code 保存延迟它的转向的跟踪上下文。当您恢复会话并且工具重新运行时，工具的跨度加入该较早转向的跟踪作为转向的 `claude_code.interaction` 跨度的子级。

#### 跨度属性

每个跨度都携带[标准属性](#standard-attributes)加上与其名称匹配的 `span.type` 属性。下表列出了在每个跨度上设置的其他属性。`llm_request`、`tool.execution` 和 `hook` 跨度在记录失败时设置 OpenTelemetry 状态 `ERROR`；其他跨度始终以状态 `UNSET` 结束。

**`claude_code.interaction`**

| 属性 | 描述 | 门控 |
| - | - | - |
| `user_prompt` | 提示文本。除非设置了门，否则值为 `<REDACTED>` | `OTEL_LOG_USER_PROMPTS` |
| `user_prompt_length` | 提示长度（字符） | |
| `interaction.sequence` | 交互的 1 基计数器，按 Claude Code 进程而不是按会话计数，如 [`event.sequence`](#event-correlation-attributes) 所述 | |
| `parent.source` | 跨度如何获得其跟踪父级：当它在入站 `TRACEPARENT` 下作为父级时为 `env`，当它启动自己的跟踪时为 `none`。需要 Claude Code v2.1.268 或更高版本 | |
| `interaction.duration_ms` | 转向的挂钟持续时间 | |

**`claude_code.llm_request`**

| 属性 | 描述 | 门控 |
| - | - | - |
| `model` | 模型标识符 | |
| `gen_ai.system` | 始终为 `anthropic`。OpenTelemetry GenAI 语义约定 | |
| `gen_ai.request.model` | 与 `model` 相同的值。OpenTelemetry GenAI 语义约定 | |
| `query_source` | 发出请求的子系统，例如 `repl_main_thread` 或子代理名称 | `ENABLE_BETA_TRACING_DETAILED` |
| `query_source_safe` | `query_source` 的有界形式，无论是否启用详细测试版跟踪都会发出，具有 `repl_main_thread` 或 `agent.builtin.general-purpose` 等值。`:` 变为 `.`，用户命名的代理显示为 `agent.custom`。需要 Claude Code v2.1.268 或更高版本 | |
| `agent_id` | 发出请求的子代理或队友的标识符。在主会话上不存在 | |
| `parent_agent_id` | 生成此代理的代理的标识符。对于主会话和直接从其生成的代理不存在 | |
| `workflow.run_id` | 生成此代理的[工作流](https://code.claude.com/docs/zh-CN/workflows.md)工具运行的运行标识符，前缀为 `wf_`。对于不是由工作流生成的代理不存在 | |
| `workflow.name` | 生成此代理的工作流的名称。用户编写的名称被替换为 `custom`，除非设置了门 | `OTEL_LOG_TOOL_DETAILS` |
| `speed` | `fast` 或 `normal` | |
| `effort` | 应用于请求的[努力级别](https://code.claude.com/docs/zh-CN/model-config.md#adjust-effort-level)：`low`、`medium`、`high`、`xhigh` 或 `max`。当 Claude Code 不发送努力级别时不存在，例如在不支持努力的模型上。需要 Claude Code v2.1.274 或更高版本 | |
| `llm_request.context` | `interaction`、`tool` 或 `standalone`，取决于父跨度 | |
| `duration_ms` | 包括重试的挂钟持续时间 | |
| `ttft_ms` | 首个令牌的时间（毫秒） | |
| `first_content_ms` | 从请求开始到成功尝试的第一个内容块的时间（毫秒）。在回退到非流式路径的请求上不存在。需要 Claude Code v2.1.268 或更高版本 | |
| `input_tokens` | API 使用块中的输入 token 计数。不包括从提示词缓存读取或写入提示词缓存的 token，这些 token 分别在 `cache_read_tokens` 和 `cache_creation_tokens` 中报告 | |
| `output_tokens` | 输出令牌计数 | |
| `cache_read_tokens` | 从提示缓存读取的令牌 | |
| `cache_creation_tokens` | 写入提示缓存的令牌 | |
| `request_id` | API 请求 ID。与 `request_id` [事件关联属性](#event-correlation-attributes)相同的值 | |
| `gen_ai.response.id` | 与 `request_id` 相同的值。OpenTelemetry GenAI 语义约定 | |
| `client_request_id` | 最终尝试的客户端生成的 `x-client-request-id` | |
| `attempt` | 为此请求进行的总尝试次数 | |
| `success` | `true` 或 `false` | |
| `status_code` | 请求失败时的 HTTP 状态代码 | |
| `error` | 请求失败时的错误消息 | |
| `error_class` | 请求失败时的短错误类令牌，例如 `api_timeout` 或 `server_overload`。需要 Claude Code v2.1.268 或更高版本 | |
| `response.has_tool_call` | 当响应包含工具使用块时为 `true` | |
| `stop_reason` | API 响应 `stop_reason`，例如 `end_turn`、`tool_use`、`max_tokens`、`stop_sequence`、`pause_turn` 或 `refusal` | |
| `gen_ai.response.finish_reasons` | 与 `stop_reason` 相同的值，包装在字符串数组中。OpenTelemetry GenAI 语义约定 | |

每次重试尝试也被记录为具有 `attempt` 和 `client_request_id` 属性的 `gen_ai.request.attempt` 跨度事件。

**`claude_code.tool`**

| 属性 | 描述 | 门控 |
| - | - | - |
| `tool_name` | 工具名称 | |
| `tool_name_safe` | `tool_name` 的形式，不携带任何用户选择的名称。内置工具名称逐字通过。MCP 工具名称显示为 `mcp_other`，除了与几个固定形状匹配的工具名称，例如名为 `browser_*` 的 `playwright` 工具，它们逐字通过。需要 Claude Code v2.1.268 或更高版本 | |
| `bash_command_class` | 对于 Bash 工具：命令的第一个程序的类别，来自固定列表，例如 `vcs` 或 `package_manager`。`other` 表示列表外的程序，`unparsed` 表示无法解析该行。需要 Claude Code v2.1.268 或更高版本 | |
| `bash_argv0` | 对于 Bash 工具：当命令的第一个程序在同一固定列表上时，例如 `git` 或 `npm`。`other` 表示列表外的任何程序。需要 Claude Code v2.1.268 或更高版本 | |
| `duration_ms` | 包括权限等待和执行的挂钟持续时间 | |
| `result_tokens` | 工具结果的近似令牌大小 | |
| `agent_id` | 运行工具的子代理或队友的标识符。在主会话上不存在 | |
| `parent_agent_id` | 生成此代理的代理的标识符。对于主会话和直接从其生成的代理不存在 | |
| `workflow.run_id` | 生成此代理的工作流工具运行的运行标识符，前缀为 `wf_`。对于不是由工作流生成的代理不存在 | |
| `workflow.name` | 生成此代理的工作流的名称。用户编写的名称被替换为 `custom`，除非设置了门 | `OTEL_LOG_TOOL_DETAILS` |
| `tool_use_id` | 此调用的模型 `tool_use` 块 id。与[tool\_result](#tool-result-event)和[tool\_decision](#tool-decision-event)事件上的 `tool_use_id` 以及钩子有效负载中的匹配，因此您可以将跨度加入这些记录 | |
| `gen_ai.tool.call.id` | 与 `tool_use_id` 相同的值。OpenTelemetry GenAI 语义约定 | |
| `file_path` | Read、Edit 和 Write 工具的目标文件路径 | `OTEL_LOG_TOOL_DETAILS` |
| `full_command` | Bash 工具的命令字符串 | `OTEL_LOG_TOOL_DETAILS` |
| `skill_name` | Skill 工具的技能名称 | `OTEL_LOG_TOOL_DETAILS` |
| `subagent_type` | Agent 工具或旧版 Task 工具的子代理类型 | `OTEL_LOG_TOOL_DETAILS` |

**`claude_code.tool` 上的 `tool.output` 跨度事件**

如果您设置 `OTEL_LOG_TOOL_CONTENT=1`，Read 和 Bash 调用可以在 `claude_code.tool` 跨度上记录 `tool.output` 跨度事件。Edit 和 Write 调用仅在您也设置 `OTEL_LOG_TOOL_DETAILS=1` 时才记录一个。该变量不限于这两个工具，因此请检查其[配置表中的行](#common-configuration-variables)以了解它在其他地方添加的参数。

MCP 工具、WebFetch 和 WebSearch 也记录此事件，在 Claude Code v2.1.283 或更高版本上。

Claude Code 从工具调用的成功返回时写入此事件，因此引发错误的调用不记录任何内容，无论工具如何。在确实返回的调用中，它不为以下内容记录 `tool.output` 事件：

* 对除 Read、Edit、Write、Bash、WebFetch、WebSearch 和 MCP 工具之外的任何工具的调用
* 返回除文件文本之外的任何内容的 Read，例如图像、PDF 或重新读取内容未更改的文件
* Edit 或 Write 调用，除非您也设置 `OTEL_LOG_TOOL_DETAILS=1`
* Claude Code 在运行期间移到后台、以便等待中的消息能够送达 Claude 的 WebFetch 或 WebSearch 调用。稍后到达的结果也不会被记录。要了解 Claude Code 何时移动调用，对于终端请参阅[Claude Code 何时发送您排队的内容](https://code.claude.com/docs/zh-CN/interactive-mode.md#when-claude-code-sends-what-you-queued)，对于 Agent SDK 会话请参阅 [`priority` 字段](https://code.claude.com/docs/zh-CN/agent-sdk/typescript#sdkusermessage)

该事件携带这些属性，每个都在内容限制处截断（默认值 60 KB）。`门控` 命名变量一个属性需要在 `OTEL_LOG_TOOL_CONTENT=1` 之上，对于 Edit 和 Write，该变量门控事件本身而不是属性。

| 属性 | 描述 | 门控 |
| - | - | - |
| `content` | Read 工具返回的文本，或 Write 调用被要求写入的文本 | `OTEL_LOG_TOOL_DETAILS` 用于 Write 工具 |
| `output` | 对于 Bash 工具，命令的组合输出，stderr 交错到 stdout。对于 MCP 工具、WebFetch 或 WebSearch，工具返回的结果：文本块由换行符连接，图像或文档被替换为占位符，例如 `[image]` | |
| `diff` | Edit 工具应用的结构化补丁 | `OTEL_LOG_TOOL_DETAILS` |
| `file_path` | Read、Edit 和 Write 工具的目标文件路径，重复相同名称的跨度属性 | `OTEL_LOG_TOOL_DETAILS` |
| `bash_command` | Bash 工具的命令字符串 | `OTEL_LOG_TOOL_DETAILS` |

父跨度的 `tool_name` 属性告诉您事件来自哪个工具。在内容限制处切割的属性伴随 `<attribute>_truncated` 和 `<attribute>_original_length`。

**`claude_code.tool.blocked_on_user`**

| 属性 | 描述 | 门控 |
| - | - | - |
| `duration_ms` | 等待权限决定所花费的时间 | |
| `decision` | `accept`、`reject` 或 `cancelled`，其中 `cancelled` 表示 Claude Code 停止等待该工具调用的权限决定，并且未运行该工具 | |
| `source` | 对于 `accept` 或 `reject`，表示决定的来源，例如 `config`、`hook` 或 `user_temporary`，其含义见[工具决定事件](#tool-decision-event)。对于 `cancelled`，表示在做出任何决定之前结束等待的原因，例如 `user_abort` | |

`source` 为 `user_abort` 的 `cancelled` 决定表示：在权限提示或[自动模式](https://code.claude.com/docs/zh-CN/permission-modes.md#eliminate-prompts-with-auto-mode)检查尚未完成时，一条需要立即送达的消息中断了该轮次。例如，在自动模式检查期间，用户有一条排队的消息并[立即发送了它](https://code.claude.com/docs/zh-CN/interactive-mode.md#when-claude-code-sends-what-you-queued)。

**`claude_code.tool.execution`**

| 属性 | 描述 | 门控 |
| - | - | - |
| `duration_ms` | 运行工具主体所花费的时间 | |
| `tool_use_id` | 与父 `claude_code.tool` 跨度上的相同值 | |
| `gen_ai.tool.call.id` | 与 `tool_use_id` 相同的值。OpenTelemetry GenAI 语义约定 | |
| `success` | `true` 或 `false` | |
| `error` | 执行失败时的错误类别字符串，例如 `Error:ENOENT` 或 `ShellError`。当设置了门时包含完整的错误消息 | `OTEL_LOG_TOOL_DETAILS` |
| `error_class` | 标识符形式的错误类别，字母、数字和下划线之外的字符被替换为 `_`，例如 `Error_ENOENT` 或 `ShellError`。即使 `error` 携带完整消息也携带类别。需要 Claude Code v2.1.268 或更高版本 | |

**`claude_code.hook`**

此跨度仅在启用详细测试版跟踪时出现，这需要 `ENABLE_BETA_TRACING_DETAILED=1` 和 `BETA_TRACING_ENDPOINT`，一对也[改变日志和跟踪的去向](https://code.claude.com/docs/zh-CN/env-vars.md#variables)。在您的 shell、用户设置或托管设置中设置该对；两个变量都在[项目和本地设置](https://code.claude.com/docs/zh-CN/settings-reference.md#variables-claude-code-ignores-in-env)中被忽略。`CLAUDE_CODE_ENHANCED_TELEMETRY_BETA` 单独不会产生它。

在交互式 CLI 会话中，详细测试版跟踪还需要您的组织被列入该功能的白名单。Agent SDK 和非交互式 `-p` 会话不需要白名单。

| 属性 | 描述 | 门控 |
| - | - | - |
| `hook_event` | 钩子事件类型，例如 `PreToolUse` | |
| `hook_name` | 完整钩子名称，例如 `PreToolUse:Write` | |
| `num_hooks` | 执行的匹配钩子命令数 | |
| `hook_definitions` | JSON 序列化的钩子配置 | `OTEL_LOG_TOOL_DETAILS` |
| `duration_ms` | 所有匹配钩子的挂钟持续时间 | |
| `num_success` | 成功完成的钩子计数 | |
| `num_blocking` | 返回阻止决定的钩子计数 | |
| `num_non_blocking_error` | 失败而不阻止的钩子计数 | |
| `num_cancelled` | 在完成前取消的钩子计数 | |

**详细测试版跟踪下的内容属性**

> [!NOTE]
>
> 其他内容承载属性，例如 `new_context`、`system_reminders`、`system_prompt_preview`、`user_system_prompt`、`tool_input` 和 `response.model_output`，仅在启用详细测试版跟踪时发出。它们不是稳定跨度 schema 的一部分。

这些属性出现在下列跨度上，`门控` 列指出属性在详细测试版跟踪之外还需要的变量。长度超过内容限制（默认值 60 KB）的值会被截断。

| 属性 | 跨度 | 描述 | 门控 |
| - | - | - | - |
| `new_context` | `claude_code.interaction` | 用户提示词 | `OTEL_LOG_USER_PROMPTS` |
| `new_context` | `claude_code.llm_request` | 随请求发送的新用户消息和工具结果 | `OTEL_LOG_USER_PROMPTS` |
| `system_reminders` | `claude_code.llm_request` | 请求的新消息中[系统提醒](https://code.claude.com/docs/zh-CN/glossary.md#system-reminder)的文本 | `OTEL_LOG_USER_PROMPTS` |
| `system_prompt_preview` | `claude_code.llm_request` | 随请求发送的完整系统提示词的前 500 个字符 | `OTEL_LOG_USER_PROMPTS` |
| `user_system_prompt` | `claude_code.llm_request` | 仅包含您通过 `systemPrompt` SDK 选项或 `--system-prompt` 和 `--append-system-prompt` 标志提供的系统提示词文本。每个会话发出一次，而不是每个请求发出一次 | `OTEL_LOG_USER_PROMPTS` |
| `response.model_output` | `claude_code.llm_request` | 模型对该请求的响应文本 | `OTEL_LOG_USER_PROMPTS` |
| `new_context` | `claude_code.tool` | 工具调用的结果，无论是哪个工具 | `OTEL_LOG_TOOL_CONTENT` |
| `tool_input` | `claude_code.tool` | 工具调用的序列化输入 | `OTEL_LOG_TOOL_DETAILS` |

在详细测试版跟踪下且设置了 `OTEL_LOG_USER_PROMPTS=1` 时，Claude Code 还会发出一个 `claude_code.system_prompt` 事件，该事件携带完整的系统提示词，并在内容限制处截断。会话每次首次发送某个不同的系统提示词时都会发出该事件，压缩之后也会再次发出。

### 动态标头

对于需要动态身份验证的企业环境，您可以配置脚本以动态生成标头。动态标头仅适用于 `http/protobuf` 和 `http/json` 协议。使用 `grpc` 协议，Claude Code 仅使用静态标头变量 `OTEL_EXPORTER_OTLP_HEADERS` 及其按信号的变体。

#### 设置配置

添加到您的 `.claude/settings.json`，用您自己的脚本替换路径：

```json theme={null}
{
  "otelHeadersHelper": "/path/to/generate-otel-headers.sh"
}
```

该值可以是可执行文件的路径，包括包含空格的路径，或带有参数的 shell 命令行。在 Windows 上，该值始终通过 shell 运行，因此在 JSON 值内引用包含空格的路径。

#### 脚本要求

脚本必须输出有效的 JSON，其中包含代表 HTTP 标头的字符串键值对：

```bash theme={null}
#!/bin/bash
# 示例：多个标头
echo "{\"Authorization\": \"Bearer $(get-token.sh)\", \"X-API-Key\": \"$(get-api-key.sh)\"}"
```

如果助手失败或打印不符合这些要求的输出，导出失败，您的遥测后端在会话中不会收到任何内容，直到助手再次工作。Claude Code 在以下位置报告失败：

* 交互式会话中的警告通知，[`otelHeadersHelper failed; telemetry is not being exported`](https://code.claude.com/docs/zh-CN/errors#otelheadershelper-failed)，在助手首次失败时每个会话显示一次
* `/status` 输出
* 调试日志，当使用 [`--debug`](https://code.claude.com/docs/zh-CN/cli-reference#cli-flags) 运行或在会话中运行 `/debug` 后
* stderr，在使用 `-p` 启动的非交互式会话中

#### 刷新行为

标头助手脚本在启动时运行，然后定期运行以支持令牌刷新。默认情况下，脚本每 29 分钟运行一次。使用 `CLAUDE_CODE_OTEL_HEADERS_HELPER_DEBOUNCE_MS` 环境变量自定义间隔。

### 多团队组织支持

具有多个团队或部门的组织可以使用 `OTEL_RESOURCE_ATTRIBUTES` 环境变量添加自定义属性以区分不同的组：

```bash theme={null}
# 添加用于团队识别的自定义属性
export OTEL_RESOURCE_ATTRIBUTES="department=engineering,team.id=platform,cost_center=eng-123"
```

这些自定义属性包含在所有指标和事件中，允许您：

* 按团队或部门过滤指标
* 按成本中心跟踪成本
* 创建特定于团队的仪表板
* 为特定团队设置警报

Claude Code 将这些值作为属性附加到每个指标数据点和事件记录，除了在 OTLP 资源块中发送它们。因为大多数指标后端将数据点属性公开为可查询的标签，您可以直接按自定义键对指标进行分组和过滤。除了 `vcs.*` [存储库属性](#repository-attributes)，自定义键永远不会覆盖[标准属性](#standard-attributes)，例如 `user.id` 或 `session.id`：当键冲突时，Claude Code 保留内置值。

每个自定义键成为每个指标系列上的标签，因此高基数值会增加指标后端中的存储成本。要仅在资源块中发送自定义属性并从数据点标签中省略它们，请设置 `OTEL_METRICS_INCLUDE_RESOURCE_ATTRIBUTES=false`。请参阅[指标基数控制](#metrics-cardinality-control)。

> [!WARNING]
>
> `OTEL_RESOURCE_ATTRIBUTES` 环境变量使用逗号分隔的键=值对，具有严格的格式要求：
>
> * **不允许空格**：值不能包含空格。例如，`user.organizationName=My Company` 无效
> * **格式**：必须是逗号分隔的键=值对：`key1=value1,key2=value2`
> * **允许的字符**：仅限 US-ASCII 字符，不包括控制字符、空格、双引号、逗号、分号和反斜杠
> * **特殊字符**：允许范围之外的字符必须进行百分比编码
>
> 对于需要空格的值，请改用下划线或 camelCase。以下示例使用每种形式设置 `org.name`：
>
> ```bash theme={null}
> export OTEL_RESOURCE_ATTRIBUTES="org.name=Johns_Organization"
> export OTEL_RESOURCE_ATTRIBUTES="org.name=JohnsOrganization"
> ```
>
> 您可以对任何字符进行百分比编码，而不仅仅是被排除的字符。此示例对空格和撇号进行编码：
>
> ```bash theme={null}
> export OTEL_RESOURCE_ATTRIBUTES="org.name=John%27s%20Organization"
> ```
>
> 用引号包装值不会转义空格。例如，`org.name="My Company"` 导致文字值 `"My Company"`（包括引号），而不是 `My Company`。

### 将遥测归因到 Desktop SSH 会话

要查看 [Desktop SSH 会话](https://code.claude.com/docs/zh-CN/desktop.md#ssh-sessions)在哪台远程机器上运行，请在自定义属性中为每台机器命名。指标和事件不会指明会话运行所在的机器。

在每台远程机器上，将 [`OTEL_RESOURCE_ATTRIBUTES`](#multi-team-organization-support) 添加到[会话读取的托管设置文件](https://code.claude.com/docs/zh-CN/desktop.md#managed-settings)中启用遥测的 `env` 块。在每台机器的文件中写出名称。Claude Code 不会展开该值，因此 `host.name=$(hostname)` 会按这些字符原样到达。

以下示例将机器命名为 `build-7`：

```json theme={null}
{
  "env": {
    "CLAUDE_CODE_ENABLE_TELEMETRY": "1",
    "OTEL_METRICS_EXPORTER": "otlp",
    "OTEL_LOGS_EXPORTER": "otlp",
    "OTEL_EXPORTER_OTLP_PROTOCOL": "grpc",
    "OTEL_EXPORTER_OTLP_ENDPOINT": "http://collector.example.com:4317",
    "OTEL_RESOURCE_ATTRIBUTES": "host.name=build-7"
  }
}
```

当没有其他来源设置该变量时，`host.name` 会出现在资源块中，适用于 Desktop SSH 会话以及该机器上的 CLI。有关自定义属性还会出现在哪些位置，请参阅[多团队组织支持](#multi-team-organization-support)。

如果名称没有到达，请检查是否存在以下原因之一：

* **您在运行 Desktop 的计算机上设置了它**：Desktop 不会将您在那里设置的值传递给 SSH 会话
* **您在 `/etc/profile` 或其他只有登录 shell 才会读取的文件中导出了它**：从登录 shell 运行 `claude` 时名称会到达，但在 Desktop SSH 会话中不会。Desktop 不通过登录 shell 启动 Claude Code。请改为在远程机器上的[托管设置文件](https://code.claude.com/docs/zh-CN/desktop.md#managed-settings)中的 `env` 下设置该变量。
* **某个值内部包含空格**：此时 Claude Code 不会将任何键复制到事件或数据点上，也不会报告错误。[从值中去掉空格](#multi-team-organization-support)。
* **其他来源已经设置了该变量**：在桌面应用启动的会话中，启动环境中已设置的变量[优先于设置文件](https://code.claude.com/docs/zh-CN/settings-reference.md#how-env-values-interact-with-your-shell)。[调试日志](https://code.claude.com/docs/zh-CN/debug-your-config.md)会列出每个被忽略的变量。当第三方 Desktop 部署在其提供的环境中[指定 OTLP 端点](#how-managed-settings-lock-the-otlp-destination)时，该环境会带有 Desktop 自己的 `OTEL_RESOURCE_ATTRIBUTES`。

### 示例配置

在运行 `claude` 之前设置这些环境变量。下面的每个场景显示完整的配置，每个变量在[常见配置变量](#common-configuration-variables)下进行了描述。要确认配置生效，请在启动会话后检查后端中的 `claude_code.session.count` 指标；[快速入门](#quick-start)涵盖仅日志验证以及当没有内容到达时要检查的内容。

对于使用 1 秒导出间隔的控制台调试：

```bash theme={null}
export CLAUDE_CODE_ENABLE_TELEMETRY=1
export OTEL_METRICS_EXPORTER=console
export OTEL_METRIC_EXPORT_INTERVAL=1000
```

对于 OTLP over gRPC：

```bash theme={null}
export CLAUDE_CODE_ENABLE_TELEMETRY=1
export OTEL_METRICS_EXPORTER=otlp
export OTEL_EXPORTER_OTLP_PROTOCOL=grpc
export OTEL_EXPORTER_OTLP_ENDPOINT=http://localhost:4317
```

对于 Prometheus，从 `http://localhost:9464/metrics` 抓取：

```bash theme={null}
export CLAUDE_CODE_ENABLE_TELEMETRY=1
export OTEL_METRICS_EXPORTER=prometheus
```

在[自托管环境](https://code.claude.com/docs/zh-CN/self-hosted-environments-reference.md#pass-through-session-child-metrics)上，会话仅在运行器的默认容量为 1 时绑定端口 9464。在更高的容量下，运行器改为在其自己的 `/metrics` 端点上重新公开会话计数器和仪表。

要将指标发送到多个导出器：

```bash theme={null}
export CLAUDE_CODE_ENABLE_TELEMETRY=1
export OTEL_METRICS_EXPORTER=console,otlp
export OTEL_EXPORTER_OTLP_PROTOCOL=http/json
```

要将指标和日志发送到不同的端点或后端：

```bash theme={null}
export CLAUDE_CODE_ENABLE_TELEMETRY=1
export OTEL_METRICS_EXPORTER=otlp
export OTEL_LOGS_EXPORTER=otlp
export OTEL_EXPORTER_OTLP_METRICS_PROTOCOL=http/protobuf
export OTEL_EXPORTER_OTLP_METRICS_ENDPOINT=http://metrics.example.com:4318
export OTEL_EXPORTER_OTLP_LOGS_PROTOCOL=grpc
export OTEL_EXPORTER_OTLP_LOGS_ENDPOINT=http://logs.example.com:4317
```

仅导出指标，不导出事件或日志：

```bash theme={null}
export CLAUDE_CODE_ENABLE_TELEMETRY=1
export OTEL_METRICS_EXPORTER=otlp
export OTEL_EXPORTER_OTLP_PROTOCOL=grpc
export OTEL_EXPORTER_OTLP_ENDPOINT=http://localhost:4317
```

仅导出事件和日志，不导出指标：

```bash theme={null}
export CLAUDE_CODE_ENABLE_TELEMETRY=1
export OTEL_LOGS_EXPORTER=otlp
export OTEL_EXPORTER_OTLP_PROTOCOL=grpc
export OTEL_EXPORTER_OTLP_ENDPOINT=http://localhost:4317
```

## 云会话和 Claude Tag 的遥测

[云会话](https://code.claude.com/docs/zh-CN/claude-code-on-the-web.md)（包括 [Claude Tag](https://claude.com/docs/claude-tag/overview) 频道会话）在[云环境](https://code.claude.com/docs/zh-CN/cloud-environments.md)中运行，而不是在用户的设备上运行，因此设备上的托管设置文件或 shell 配置文件不会配置其遥测。对于在 Anthropic 托管环境中的会话，本部分介绍了在何处设置遥测变量、如何使收集器可从环境访问，以及如何在导出的数据中区分云会话和 Claude Tag 会话。

要从这些会话导出遥测，请使用与[管理员配置](#administrator-configuration)示例相同的密钥，在以下两个位置之一中设置 `CLAUDE_CODE_ENABLE_TELEMETRY` 和 `OTEL_*` 变量：

* **服务器管理的设置**：将它们添加到您组织的[服务器管理的设置](https://code.claude.com/docs/zh-CN/server-managed-settings.md)的 `env` 块中。Claude Code 在启动时会在[服务器管理的设置适用](https://code.claude.com/docs/zh-CN/model-config.md#surface-coverage)的任何地方获取这些设置，这包括您用户的机器和除 Claude Tag 频道会话外的云会话。Claude Tag 会话不会接收您的服务器管理的设置，因此此路由不会配置它们。
* **环境的变量**：将它们添加到云环境的[环境变量](https://code.claude.com/docs/zh-CN/cloud-environments.md#set-environment-variables)中，以仅配置在该环境中运行的会话。这是到达 Claude Tag 会话的路由。

任何使用环境的人都可以读取其变量，因此不要在其中放置凭据，例如 `OTEL_EXPORTER_OTLP_HEADERS` 中的收集器令牌。环境上的[网络密钥](https://code.claude.com/docs/zh-CN/cloud-environments.md#add-network-secrets)也无济于事，因为 Claude Code 自己的遥测导出是[从不获得该密钥的请求](https://code.claude.com/docs/zh-CN/cloud-environments.md#requests-that-never-get-the-credential)之一。如果您的收集器需要凭据，请改为通过服务器管理的设置配置整个导出，因为当您在那里设置凭据时，[Claude Code 会删除在托管设置外设置的端点变量](#how-managed-settings-lock-the-otlp-destination)。

在为云会话配置遥测时，请记住这些约束：

* **让会话到达收集器**：Claude Code 通过会话的网络发送导出，因此它是否到达您的 `OTEL_EXPORTER_OTLP_ENDPOINT` 中的主机取决于环境的[网络访问级别](https://code.claude.com/docs/zh-CN/cloud-environments.md#access-levels)。如果会话无法在您选择的级别上到达收集器的域，请[将域添加到环境的允许列表](https://code.claude.com/docs/zh-CN/cloud-environments.md#allow-specific-domains)，因为没有服务器管理的设置会将域添加到环境的网络允许列表。
* **Claude Tag 频道**：请在设置为您组织默认环境或固定到频道的[共享环境](https://code.claude.com/docs/zh-CN/cloud-environments.md#organization-shared-environments)上进行允许列表和任何环境变量更改。
* **Cowork 单独配置**：如[表面覆盖表](https://code.claude.com/docs/zh-CN/model-config.md#surface-coverage)所示，Cowork 会话不会接收服务器管理的设置，因此服务器管理的 `env` 块不会配置其遥测。

### 将遥测属性分配给云会话

默认情况下，来自云会话的指标和事件携带[标准属性](#standard-attributes)，包括 `session.id`、`ccr.session.id` 和 `organization.id`，因此您可以按会话或组织进行筛选，无需额外配置。`ccr.session.id` 值是会话的 `CLAUDE_CODE_REMOTE_SESSION_ID`。要将其转换为会话的成绩单 URL，请参阅[将输出链接回会话](https://code.claude.com/docs/zh-CN/cloud-environments.md#link-output-back-to-the-session)。

要区分不同的云端会话，请按 `ccr.session.id`（而不是 `session.id`）进行分组或筛选。

要更详细地属性化遥测，请使用这些选项：

* **识别 Claude Tag 会话**：设置 `OTEL_METRICS_INCLUDE_ENTRYPOINT=true`，如[指标基数控制](#metrics-cardinality-control)下所述。指标随后会携带 `app.entrypoint`，其值对于 Claude Tag 会话为 `claude-in-slack`。
* **添加自定义属性**：在设置这些会话的其他 `OTEL_*` 变量的同一位置设置 [`OTEL_RESOURCE_ATTRIBUTES`](#multi-team-organization-support)。如果您改为在环境的[设置脚本](https://code.claude.com/docs/zh-CN/cloud-environments.md#setup-scripts)中 `export` 它，该值不会到达 Claude Code：设置脚本是在 Claude Code 启动前运行的单独 Bash 脚本，它导出的变量随之结束。

在 Claude Tag 频道会话中，Claude 作为您组织的[共享身份](https://code.claude.com/docs/zh-CN/cloud-environments.md#set-the-environment-a-claude-tag-channel-uses)工作，而不是作为任何成员，因此不要依赖 `user.*` 属性来识别谁标记了 Claude。

## 可用的指标和事件

### 标准属性

所有指标和事件都共享以下标准属性：

| 属性 | 描述 | 控制方式 |
| - | - | - |
| `session.id` | Claude Code 自身的会话 ID，`/clear` 会替换它 | `OTEL_METRICS_INCLUDE_SESSION_ID`（默认值：true） |
| `ccr.session.id` | 云端会话标识符，即 `CLAUDE_CODE_REMOTE_SESSION_ID` 的值，仅出现在运行于[云环境](https://code.claude.com/docs/zh-CN/cloud-environments.md)中的会话上 | `OTEL_METRICS_INCLUDE_SESSION_ID`（默认值：true） |
| `app.version` | 当前 Claude Code 版本 | `OTEL_METRICS_INCLUDE_VERSION`（默认值：false） |
| `app.entrypoint` | 会话的启动方式，例如 `cli`、`sdk-cli`、`sdk-ts`、`sdk-py`、`claude-vscode`，或用于 Claude Tag 会话的 `claude-in-slack` | `OTEL_METRICS_INCLUDE_ENTRYPOINT`（默认值：false） |
| `organization.id` | 组织 UUID（已进行身份验证时） | 可用时始终包含 |
| `user.account_uuid` | 账户 UUID（已进行身份验证时） | `OTEL_METRICS_INCLUDE_ACCOUNT_UUID`（默认值：true） |
| `user.account_id` | 与 Anthropic 管理 API 匹配的带标签格式账户 ID（已进行身份验证时），例如 `user_01BWBeN28...` | `OTEL_METRICS_INCLUDE_ACCOUNT_UUID`（默认值：true） |
| `user.id` | 首次运行时生成并持久保存在 `~/.claude.json` 中的随机匿名标识符。它不包含任何个人信息，也不是从您的 Claude 账户派生的。删除该文件后，下次运行时会生成一个新的、不相关的值。 | 始终包含 |
| `user.email` | 用户电子邮件地址，来自您的登录信息，或者在[云端会话](https://code.claude.com/docs/zh-CN/claude-code-on-the-web.md)中来自该会话自身的凭据 | 可用时始终包含 |
| `terminal.type` | 终端类型，例如 `iTerm.app`、`vscode`、`cursor` 或 `tmux` | 检测到时始终包含 |
| `OTEL_RESOURCE_ATTRIBUTES` 中的键 | 您设置的自定义属性，例如 `department` 或 `team.id`。请参阅[多团队组织支持](#multi-team-organization-support) | `OTEL_METRICS_INCLUDE_RESOURCE_ATTRIBUTES`（默认值：true） |
| `vcs.repository.url.full`、`vcs.owner.name`、`vcs.repository.name`、`vcs.provider.name` | 会话仓库的身份，从其 `origin` 远程派生。请参阅[仓库属性](#repository-attributes) | `OTEL_METRICS_INCLUDE_REPOSITORY`（默认值：false）。需要 Claude Code v2.1.269 或更高版本 |

在通过 `/login` 登录到 [Claude apps 网关](https://code.claude.com/docs/zh-CN/claude-apps-gateway)的会话中，CLI 会在导出数据上标记经过身份验证的身份：`user.id` 是 IdP 主体，`user.email` 是登录的电子邮件，`user.groups` 以逗号分隔字符串的形式携带 IdP 组成员身份。每次导出还会携带 `identity.source: gateway-oidc`。网关身份最后应用，因此在这些会话中，通过 `OTEL_RESOURCE_ATTRIBUTES` 设置的 `user.*` 和 `identity.*` 键会被忽略。

> [!NOTE]
>
> Claude Code 在开发者登录之前记录的事件不携带网关身份。当 Claude Code 在未登录网关的状态下打开会话时（例如在[网关结束登录](https://code.claude.com/docs/zh-CN/errors.md#cloud-gateway-session-expired)之后），登录前记录的启动事件携带匿名的 `user.id`，且不带 `identity.source`。这些事件包括 [`managed_settings_resolved`](#managed-settings-resolved-event)、[`plugin_loaded`](#plugin-loaded-event) 和 [`mcp_server_connection`](#mcp-server-connection-event)。

有关通过网关连接的 Claude Desktop 和 Cowork 会话上的身份属性，请参阅[网关 `telemetry` 参考](https://code.claude.com/docs/zh-CN/claude-apps-gateway-config#telemetry)。

事件还额外包含以下属性。这些属性永远不会附加到指标上，因为它们会导致无界基数：

* `prompt.id`：将用户提示词与其后直到下一个提示词之前的所有事件关联起来的 UUID。请参阅[事件关联属性](#event-correlation-attributes)。
* `workspace.host_paths`：在桌面应用中选择的主机工作区目录，以字符串数组形式表示
* `workflow.run_id`：运行标识符，前缀为 `wf_`，出现在属于某次 [Workflow](https://code.claude.com/docs/zh-CN/workflows.md) 工具运行的 Agent 所发出的 API 和工具事件上。按某个 `workflow.run_id` 筛选事件，即可重建该次运行的 API 请求和工具结果。该标识符涵盖工作流脚本生成的 Agent，以及这些 Agent 进一步生成的任何 Agent（例如 skill 调用）。它与 Workflow 工具结果中报告的运行标识符一致。
* `workflow.name`：工作流的名称，即其脚本的 `meta.name`，与 `workflow.run_id` 一起发出。当运行执行未经修改的内置脚本时，内置工作流名称按原样显示。用户编写的名称（包括内置脚本的编辑副本）会被替换为 `custom`，除非设置了 `OTEL_LOG_TOOL_DETAILS=1`。
* `agent_id`：引发该事件的[子代理](https://code.claude.com/docs/zh-CN/sub-agents.md)的标识符，例如 `a3f2c1b4d5e6f7a80`。携带该属性的事件包括子代理的 API 和工具事件、其 [`assistant_response`](#assistant-response-event) 事件，以及已完成子代理的 [`subagent_completed`](#subagent-completed-event)。主会话事件和[进程内队友](https://code.claude.com/docs/zh-CN/agent-teams.md#choose-a-display-mode)事件上不包含该属性。子代理的 [span `agent_id`](#span-attributes) 与此值相同，或在开头的 `a` 之后添加一个标签，如 `acompact-3f2c1b4d5e6f7a80`。需要 Claude Code v2.1.310 或更高版本
* `parent_agent_id`：生成或最近一次恢复引发该事件的子代理的 Agent 的 `agent_id`。当该 Agent 为主会话时不包含此属性。需要 Claude Code v2.1.310 或更高版本

#### 仓库属性

设置 `OTEL_METRICS_INCLUDE_REPOSITORY=true` 可以用会话仓库的身份来标记指标和事件，以便共享收集器能够按仓库归因使用情况。需要 Claude Code v2.1.269 或更高版本。

Claude Code 每个会话从仓库的 `origin` 远程派生一次这些属性。当仓库的 HTTPS 和 SSH 远程指向相同的主机和相同的路径时（如在 GitHub、GitLab 和 Bitbucket Cloud 上），两者会产生相同的值：

| 属性 | 值 |
| - | - |
| `vcs.repository.url.full` | 不带 `.git` 的仓库浏览器 URL，例如 `https://github.com/example-org/example-repo` |
| `vcs.owner.name` | 所有者或组路径，例如 `example-org`；当远程路径只有一个段时省略 |
| `vcs.repository.name` | 纯仓库名称，例如 `example-repo` |
| `vcs.provider.name` | 当 Claude Code 将远程的主机或 URL 形式识别为 `github`、`gitlab`、`bitbucket` 或 `gitea` 之一时为相应的值；否则省略 |

这些值会转换为小写，且远程 URL 中的凭据、查询字符串和片段永远不会出现在其中。当会话没有 `origin` 远程、远程不是 URL 形式，或者唯一的外层仓库是您的主目录时，这些属性会被省略。

要从[云端会话](https://code.claude.com/docs/zh-CN/claude-code-on-the-web.md)获取这些属性，请在其[云环境](https://code.claude.com/docs/zh-CN/cloud-environments.md#set-environment-variables)中设置遥测变量，包括 `OTEL_METRICS_INCLUDE_REPOSITORY`。还需要在该环境的[网络访问](https://code.claude.com/docs/zh-CN/cloud-environments.md#network-access)中允许您的收集器域名。

您在 [`OTEL_RESOURCE_ATTRIBUTES`](#multi-team-organization-support) 中声明的 `vcs.*` 键会替换该键的派生值。如果您声明了 `vcs.repository.url.full`，Claude Code 将永远不会读取远程，并且只报告您声明的键。

如果同一仓库的 HTTPS 和 SSH 克隆报告不同的值（例如在自托管安装中，HTTPS 克隆 URL 带有 SSH URL 所没有的路径前缀），请在 `OTEL_RESOURCE_ATTRIBUTES` 中声明 `vcs.repository.url.full` 以及您希望报告的所有其他 `vcs.*` 键。之后每个克隆都会报告您声明的身份。

这些属性只会流向您自己的导出器；Anthropic 的遥测会丢弃所有 `vcs.*` 键。

### 指标

Claude Code 导出以下指标。"单位"列显示附加到每个指标的 OpenTelemetry 单位字符串；计数类指标不带单位。

| 指标名称 | 描述 | 单位 |
| - | - | - |
| `claude_code.session.count` | 已启动的 CLI 会话计数 | 无 |
| `claude_code.lines_of_code.count` | 已修改的代码行数计数 | 无 |
| `claude_code.pull_request.count` | 已创建的 Pull Request 数量 | 无 |
| `claude_code.commit.count` | 已创建的 git 提交数量 | 无 |
| `claude_code.cost.usage` | Claude Code 会话的成本 | USD |
| `claude_code.token.usage` | 使用的 token 数量 | tokens |
| `claude_code.code_edit_tool.decision` | 代码编辑工具权限决策的计数 | 无 |
| `claude_code.active_time.total` | 总活跃时间 | s |

当 `prometheus` 是 `OTEL_METRICS_EXPORTER` 中列出的唯一导出器时，Claude Code 会从导出的指标中省略 `USD`、`tokens` 和 `s` 单位，以使抓取结果保持为有效的 Prometheus 文本格式。指标名称不会改变，组合多个导出器的配置（例如 `otlp,prometheus`）会保留单位。

### 指标详情

每个指标都包含上面列出的标准属性。下面注明了带有额外上下文特定属性的指标。

#### 会话计数器

在每个会话开始时递增。

**属性**：

* 所有[标准属性](#standard-attributes)
* `start_type`：会话的启动方式。取值为 `"fresh"`、`"resume"`、`"continue"` 或 `"agents_view"` 之一。`"agents_view"` 值标识 `claude agents` 仪表板进程，这是一个由用户启动的本地 UI，而不是对话会话。在仪表板中按此值过滤，可以将 UI 进程启动与对话会话区分开来。

#### 代码行数计数器

在添加或删除代码时递增。

**属性**：

* 所有[标准属性](#standard-attributes)
* `type`：（`"added"`、`"removed"`）
* `model`：进行更改的模型的模型标识符（例如 "claude-sonnet-5"）

#### Pull Request 计数器

当 Claude Code 通过 shell 命令或 MCP 工具创建 Pull Request 或合并请求时递增。

**属性**：

* 所有[标准属性](#standard-attributes)

#### 提交计数器

通过 Claude Code 创建 git 提交时递增。

**属性**：

* 所有[标准属性](#standard-attributes)

#### 成本计数器

在每次 API 请求后递增。

`agent.name`、`skill.name`、`plugin.name`、`mcp_server.name` 和 `mcp_tool.name` 属性默认都会将某些名称编辑为 `"custom"` 或 `"third-party"` 占位符。如果您设置了 `OTEL_LOG_TOOL_DETAILS=1`，它们会改为携带真实名称。在 v2.1.273 之前，即使设置了 `OTEL_LOG_TOOL_DETAILS=1`，成本和 token 计数器以及 `api_request`、`api_error` 和 `api_refusal` 事件仍携带编辑后的值。

**属性**：

* 所有[标准属性](#standard-attributes)
* `model`：模型标识符（例如 "claude-sonnet-5"）
* `query_source`：发出请求的子系统类别。取值为 `"main"`、`"subagent"` 或 `"auxiliary"` 之一
* `speed`：当请求使用快速模式时为 `"fast"`。否则不存在
* `effort`：应用于请求的 [effort 级别](https://code.claude.com/docs/zh-CN/model-config.md#adjust-effort-level)：`"low"`、`"medium"`、`"high"`、`"xhigh"` 或 `"max"`。当 Claude Code 未发送 effort 级别时不存在，例如在不支持 effort 的模型上。
* `agent.name`：发出请求的子代理类型。内置 Agent 名称和来自官方市场插件的 Agent 按原样显示。其他用户定义的 Agent 名称会被替换为 `"custom"`。当请求不是由命名的子代理类型发出时不存在。
* `skill.name`：请求活跃的 skill，由 Skill 工具或 `/` 命令设置，或由生成的子代理继承。内置、捆绑、用户定义以及官方市场插件的 skill 名称按原样显示。第三方插件的 skill 名称会被替换为 `"third-party"`。当没有活跃的 skill 时不存在。
* `plugin.name`：当活跃的 skill 或子代理由插件提供时，为所属插件。官方市场插件名称按原样显示。第三方插件名称会被替换为 `"third-party"`。当 skill 和子代理都没有所属插件时不存在。
* `marketplace.name`：所属插件的安装来源市场。即使设置了 `OTEL_LOG_TOOL_DETAILS=1`，也仅针对官方市场插件发出。否则不存在。
* `mcp_server.name`：此请求所使用的工具结果对应的 MCP 服务器。内置、claude.ai 代理以及官方注册表的服务器名称按原样显示。用户配置的服务器名称会被替换为 `"custom"`。当请求未使用任何 MCP 工具结果时不存在。在 v2.1.222 之前，Claude Code 会在 MCP 工具调用之后的每个请求上设置此属性，而不仅仅是在使用了工具结果的请求上，因此在您升级后，聚合此属性的仪表板会显示数值下降。
* `mcp_tool.name`：此请求所使用的结果对应的 MCP 工具，其编辑和版本行为与 `mcp_server.name` 相同。当请求未使用任何 MCP 工具结果时不存在。

#### Token 计数器

在每次 API 请求后递增。

**属性**：

* 所有[标准属性](#standard-attributes)
* `type`：（`"input"`、`"output"`、`"cacheRead"`、`"cacheCreation"`）。`"input"` 类型不包括从提示缓存读取或写入提示缓存的 token，这些 token 计入 `"cacheRead"` 和 `"cacheCreation"`。`"cacheCreation"` 类型将五分钟和一小时[缓存生命周期](https://code.claude.com/docs/zh-CN/prompt-caching.md#cache-lifetime)的缓存写入合并计数。
* `model`：模型标识符（例如 "claude-sonnet-5"）
* `query_source`：发出请求的子系统类别。取值为 `"main"`、`"subagent"` 或 `"auxiliary"` 之一
* `speed`：当请求使用快速模式时为 `"fast"`。否则不存在
* `effort`：应用于请求的 [effort 级别](https://code.claude.com/docs/zh-CN/model-config.md#adjust-effort-level)。详情请参阅[成本计数器](#cost-counter)。
* `agent.name`、`skill.name`、`plugin.name`、`marketplace.name`、`mcp_server.name`、`mcp_tool.name`：请求的 skill、插件、Agent 和 MCP 归因。有关定义和编辑行为，请参阅[成本计数器](#cost-counter)。

#### 代码编辑工具决策计数器

当用户接受或拒绝 Edit、Write 或 NotebookEdit 工具的使用时递增。

**属性**：

* 所有[标准属性](#standard-attributes)
* `tool_name`：工具名称（`"Edit"`、`"Write"`、`"NotebookEdit"`）
* `decision`：用户决策（`"accept"`、`"reject"`）
* `source`：决策的来源。取值为 `"config"`、`"hook"`、`"user_permanent"`、`"user_temporary"`、`"user_abort"` 或 `"user_reject"` 之一。有关每个值的含义，请参阅[工具决策事件](#tool-decision-event)。
* `language`：被编辑文件的编程语言，例如 `"TypeScript"`、`"Python"`、`"JavaScript"` 或 `"Markdown"`。对于无法识别的文件扩展名，返回 `"unknown"`。

#### 活跃时间计数器

跟踪实际主动使用 Claude Code 的时间，不包括空闲时间。此指标在用户交互期间（例如输入和阅读回复）以及 CLI 处理期间（例如工具执行和 AI 回复生成）递增。

**属性**：

* 所有[标准属性](#standard-attributes)
* `type`：`"user"` 表示键盘交互，`"cli"` 表示工具执行和 AI 回复

### 事件

Claude Code 通过 OpenTelemetry 日志/事件导出以下事件（当配置了 `OTEL_LOGS_EXPORTER` 时）：

#### 事件关联属性

当用户提交提示词时，Claude Code 可能会进行多次 API 调用并运行多个工具。`prompt.id` 属性可以将所有这些事件关联回触发它们的单个提示词。

| 属性 | 描述 |
| - | - |
| `prompt.id` | UUID v4 标识符，关联处理单个用户提示词期间产生的所有事件 |
| `event.sequence` | 用于对事件排序的从 0 开始的计数器，按 Claude Code 进程计数，而不是按会话计数 |
| `message.uuid` | 持久保存在会话记录（`~/.claude/projects/*/*.jsonl` 文件）中的消息的 UUID。出现在 `assistant_response`、`api_response_body` 以及 `user_prompt` 上，但命令分派除外，因为命令分派可能产生零条或多条消息。在 `assistant_response` 和 `api_response_body` 上，这是该响应的最后一条会话记录条目，下一轮的 `parentUuid` 从它链接。在 `api_response_body` 上需要 Claude Code v2.1.274 或更高版本 |
| `request_id` | 服务器分配的 API 请求 ID，从 `request-id` 响应头读取，例如 `req_011...`。对于没有 `request-id` 响应头的响应（如在 [Amazon Bedrock](https://code.claude.com/docs/zh-CN/amazon-bedrock.md) 上），该值改为取自 `x-amzn-requestid` 响应头。当响应携带其中任一响应头时，出现在 `api_request`、`api_error`、`api_refusal`、`assistant_response`、`api_response_body`、[`tool_result`](#tool-result-event) 和 [`tool_decision`](#tool-decision-event) 上。与 `llm_request` 追踪 span 上的同名属性一致。`x-amzn-requestid` 来源需要 Claude Code v2.1.282 或更高版本 |
| `client_request_id` | 客户端生成的 UUID，作为 `x-client-request-id` 请求头发送。在第一方 API 连接上出现于 `api_request` 和 `api_error`；在第三方提供商后端上，以及当请求通过非流式回退方式重试时不存在。它将请求与其响应配对，并且对于超时等从未产生服务器 `request_id` 的失败仍然可用。与 `llm_request` 追踪 span 上的同名属性一致 |

要追踪由单个提示词触发的所有活动，请按特定的 `prompt.id` 值过滤您的事件。这会返回处理该提示词期间发生的 user\_prompt 事件、所有 api\_request 事件以及所有 tool\_result 事件。

`event.sequence` 在每次 Claude Code 进程启动时从 0 开始，并在该进程的生命周期内递增。它在 `/clear`（会分配新的 `session.id`）之后继续计数。如果您[在不分叉的情况下恢复会话](https://code.claude.com/docs/zh-CN/how-claude-code-works.md#resume-or-fork-sessions)，会话会保留其 `session.id`，但其 `event.sequence` 值取自恢复它的进程，因此在同一会话中，较晚的事件可能携带比较早事件更小的值，或者重复某个值。要对会话的事件进行排序，请按 `event.timestamp` 排序，并使用 `event.sequence` 对共享同一时间戳的事件进行排序。

为了进行消息级别的重建，每类事件都携带一个与会话记录中某个字段匹配的键。会话记录的条目格式[属于 Claude Code 内部实现](https://code.claude.com/docs/zh-CN/sessions.md#where-transcripts-are-stored)，并且会在版本之间变化，因此基于这些字段进行关联的管道可能在任何版本发布时失效；请将这些关联视为特定于版本的，而不是稳定的约定：

* `user_prompt`、`assistant_response` 和 `api_response_body` 上的 `message.uuid`
* API 事件上的 `request_id`，在会话记录的 assistant 条目中持久保存为 `requestId`
* `tool_result` 和 `tool_decision` 事件上的 `tool_use_id`，以及 `safety_event` 事件上的 `tool_use_id`（针对[轮次结束](#safety-event)的事件除外）

#### 用户提示词事件

在提交提示词时记录，包括 Claude Code 自行启动的轮次。

**事件名称**：`claude_code.user_prompt`

**属性**：

* 所有[标准属性](#standard-attributes)
* `event.name`：`"user_prompt"`
* `event.timestamp`：ISO 8601 时间戳
* `event.sequence`：用于对事件排序的按进程计数器，详见[事件关联属性](#event-correlation-attributes)
* `prompt_length`：提示词的长度
* `prompt`：提示词内容。默认已编辑。设置 `OTEL_LOG_USER_PROMPTS=1` 以包含它
* `prompt_text`：与 `prompt` 相同的值，在相同的条件下编辑。将带点的属性名称存储为嵌套对象的后端会将 `prompt.id` 读取为名为 `prompt` 的对象中的 `id`，从而可能丢失提示词字符串。在这种情况下请改为读取 `prompt_text`。需要 Claude Code v2.1.287 或更高版本
* `message.uuid`：生成的用户消息的 UUID，与持久保存的会话记录条目一致。在命令分派上不存在，因为命令分派可能产生零条或多条消息。
* `command_name`：当提示词调用命令时的命令名称。内置和捆绑的命令名称（例如 `compact` 或 `debug`）按原样发出；别名（例如 `reset`）按输入的形式发出，而不是规范名称。自定义、插件和 MCP 命令名称会折叠为 `custom` 或 `mcp`，除非设置了 `OTEL_LOG_TOOL_DETAILS=1`
* `command_source`：存在命令时命令的来源：`builtin`、`custom` 或 `mcp`。插件提供的命令报告为 `custom`

#### 助手回复事件

在每次从模型返回文本内容的 API 请求之后记录。仅包含响应的文本块；思考块和工具使用块被排除在外。

**事件名称**：`claude_code.assistant_response`

**属性**：

* 所有[标准属性](#standard-attributes)
* `event.name`：`"assistant_response"`
* `event.timestamp`：ISO 8601 时间戳
* `event.sequence`：用于对事件排序的按进程计数器，详见[事件关联属性](#event-correlation-attributes)
* `response_length`：回复文本的长度（以字符为单位）
* `response`：回复文本，在内容限制处截断（默认 60 KB）。默认编辑为 `<REDACTED>`。设置 `OTEL_LOG_ASSISTANT_RESPONSES=1` 以包含它。当未设置 `OTEL_LOG_ASSISTANT_RESPONSES` 时，改由 `OTEL_LOG_USER_PROMPTS` 控制，因此如果要在启用提示词日志记录的同时保持回复被编辑，请设置 `OTEL_LOG_ASSISTANT_RESPONSES=0`
* `model`：模型标识符（例如 "claude-sonnet-5"）
* `request_id`：API 请求 ID，详见[事件关联属性](#event-correlation-attributes)
* `message.uuid`：该响应最后一条会话记录条目的 UUID。API 响应按每个内容块一条会话记录条目的方式持久保存；这是最后一条，下一轮的 `parentUuid` 从它链接。
* `query_source`：发出请求的子系统，例如 `"repl_main_thread"`、`"compact"` 或子代理名称

#### 工具结果事件

在工具完成执行时记录。如果工具调用被拒绝则不会发出；有关拒绝，请参阅[工具决策事件](#tool-decision-event)。

**事件名称**：`claude_code.tool_result`

**属性**：

* 所有[标准属性](#standard-attributes)
* `event.name`：`"tool_result"`
* `event.timestamp`：ISO 8601 时间戳
* `event.sequence`：用于对事件排序的按进程计数器，详见[事件关联属性](#event-correlation-attributes)
* `tool_name`：工具的名称
* `tool_use_id`：此次工具调用的唯一标识符。与传递给 hook 的 `tool_use_id` 相匹配，从而可以在 OTel 事件和 hook 捕获的数据之间建立关联。
* `request_id`：包含此工具调用的 API 响应的 [`request_id`](#event-correlation-attributes)，以便您将该调用与该响应的 API 事件关联起来。当该响应没有请求 ID，或没有任何 API 响应包含该调用时不包含。需要 Claude Code v2.1.310 或更高版本
* `success`：`"true"` 或 `"false"`
* `duration_ms`：执行时间（以毫秒为单位）
* `error_type`：工具失败时的错误类别字符串，例如 `"Error:ENOENT"` 或 `"ShellError"`
* `error`（当 `OTEL_LOG_TOOL_DETAILS=1` 时）：工具失败时的完整错误消息
* `decision_type`：始终为 `"accept"`，因为此事件仅在工具运行后发出。被拒绝的调用不会产生工具结果
* `decision_source`：权限决策的来源。取值为 `"config"`、`"hook"`、`"user_permanent"` 或 `"user_temporary"` 之一。有关每个值的含义，请参阅[工具决策事件](#tool-decision-event)。仅用于拒绝的来源 `"user_abort"` 和 `"user_reject"` 永远不会出现在此事件上。
* `tool_input_size_bytes`：JSON 序列化后的工具输入大小（以字节为单位）
* `tool_result_size_bytes`：工具结果的大小（以字节为单位）
* `mcp_server_scope`：MCP 服务器作用域标识符（针对 MCP 工具）
* `vcs.ref.head.revision`、`vcs.ref.head.name`、`vcs.ref.head.type`（当 `OTEL_LOG_TOOL_DETAILS=1` 时）：由 Bash 或 PowerShell 工具运行的成功 `git commit` 的提交身份。`vcs.ref.head.revision` 是提交 SHA，`vcs.ref.head.name` 是提交所在的分支，`vcs.ref.head.type` 为 `branch`。当提交是在分离的 HEAD 上进行时，名称和类型会被省略。需要 Claude Code v2.1.269 或更高版本
* `tool_parameters`（当 `OTEL_LOG_TOOL_DETAILS=1` 时）：包含工具特定参数的 JSON 字符串。对于 Claude Desktop 的内置服务器，在 Claude Desktop 拥有的会话中，即使该标志关闭，也会包含 `mcp_server_name`/`mcp_tool_name` 对，这与[工具决策事件](#tool-decision-event)中由宿主定义的例外情况相同。参数因工具而异：
  * 对于 Bash 工具：包括 `bash_command`、`full_command`、`timeout`、`description` 和 `dangerouslyDisableSandbox`，当 `git commit` 命令成功时还包括 `git_commit_id` 和 `git_branch`。当该提交是会话工作目录的 HEAD 时，`git_commit_id` 是完整的提交 SHA，否则是 git 的缩写 SHA。`git_branch` 是提交所在的分支，在分离的 HEAD 上省略
  * 对于桌面应用的工作区 Bash 工具（它同样将 `tool_name` 报告为 `Bash`）：仅包括 `bash_command`、`full_command` 和 `timeout`
  * 对于 MCP 工具：包括 `mcp_server_name`、`mcp_tool_name`
  * 对于 Skill 工具：包括 `skill_name`
  * 对于 Agent 工具或旧版 Task 工具：包括 `subagent_type`
* `tool_input`（当 `OTEL_LOG_TOOL_DETAILS=1` 时）：JSON 序列化的工具参数。超过 512 个字符的单个值会被截断，完整的 JSON 数据限制在约 4 K 个字符以内。适用于所有工具，包括 MCP 工具。

#### API 请求事件

针对每个发往 Claude 的 API 请求记录。

**事件名称**：`claude_code.api_request`

**属性**：

* 所有[标准属性](#standard-attributes)
* `event.name`：`"api_request"`
* `event.timestamp`：ISO 8601 时间戳
* `event.sequence`：用于对事件排序的按进程计数器，详见[事件关联属性](#event-correlation-attributes)
* `model`：使用的模型（例如 "claude-sonnet-5"）
* `cost_usd`：以美元计的估算成本
* `cost_usd_micros`：以百万分之一美元计的估算成本，以整数形式发出
* `duration_ms`：请求持续时间（以毫秒为单位）
* `input_tokens`：输入 token 数量，不包括从提示词缓存读取或写入提示词缓存的 token
* `output_tokens`：输出 token 数量
* `cache_read_tokens`：从缓存读取的 token 数量
* `cache_creation_tokens`：用于创建缓存的 token 数量
* `cache_creation_5m_tokens`、`cache_creation_1h_tokens`：API 响应报告的以五分钟[缓存生命周期](https://code.claude.com/docs/zh-CN/prompt-caching.md#cache-lifetime)写入缓存的 token 数量，以及以一小时生命周期写入的数量。当 API 响应未报告这两项时，两者均为 `0`，即使 `cache_creation_tokens` 大于零也是如此。需要 Claude Code v2.1.300 或更高版本
* `request_id`：API 请求 ID，例如 `"req_011..."`，详见[事件关联属性](#event-correlation-attributes)。
* `client_request_id`：客户端生成的 UUID，作为 `x-client-request-id` 请求头发送；有关其出现的条件，请参阅[事件关联属性](#event-correlation-attributes)表
* `speed`：`"fast"` 或 `"normal"`，表示快速模式是否处于活跃状态
* `query_source`：发出请求的子系统，例如 `"repl_main_thread"`、`"compact"` 或子代理名称
* `effort`：应用于请求的 [effort 级别](https://code.claude.com/docs/zh-CN/model-config.md#adjust-effort-level)：`"low"`、`"medium"`、`"high"`、`"xhigh"` 或 `"max"`。当 Claude Code 未发送 effort 级别时不存在，例如在不支持 effort 的模型上。
* `agent.name`、`skill.name`、`plugin.name`、`marketplace.name`、`mcp_server.name`、`mcp_tool.name`：请求的 skill、插件、Agent 和 MCP 归因。有关定义和编辑行为，请参阅[成本计数器](#cost-counter)。

#### API 错误事件

当发往 Claude 的 API 请求失败时记录。

**事件名称**：`claude_code.api_error`

**属性**：

* 所有[标准属性](#standard-attributes)
* `event.name`：`"api_error"`
* `event.timestamp`：ISO 8601 时间戳
* `event.sequence`：用于对事件排序的按进程计数器，详见[事件关联属性](#event-correlation-attributes)
* `model`：使用的模型（例如 "claude-sonnet-5"）
* `error`：错误消息
* `status_code`：以数字表示的 HTTP 状态码。对于非 HTTP 错误（例如连接失败）不存在。
* `duration_ms`：请求持续时间（以毫秒为单位）
* `attempt`：已进行的尝试次数，包括初始请求。[检测重试耗尽](#detect-retry-exhaustion)说明了计数何时重新开始
* `request_id`：API 请求 ID，例如 `"req_011..."`，详见[事件关联属性](#event-correlation-attributes)。
* `client_request_id`：客户端生成的 UUID，作为 `x-client-request-id` 请求头发送。即使超时或连接错误等失败从未产生服务器 `request_id`，它也可用；有关其出现的条件，请参阅[事件关联属性](#event-correlation-attributes)表。
* `speed`：`"fast"` 或 `"normal"`，表示快速模式是否处于活跃状态
* `query_source`：发出请求的子系统，例如 `"repl_main_thread"`、`"compact"` 或子代理名称
* `effort`：应用于请求的 [effort 级别](https://code.claude.com/docs/zh-CN/model-config.md#adjust-effort-level)。当 Claude Code 未发送 effort 级别时不存在，例如在不支持 effort 的模型上。
* `agent.name`、`skill.name`、`plugin.name`、`marketplace.name`、`mcp_server.name`、`mcp_tool.name`：请求的 skill、插件、Agent 和 MCP 归因。有关定义和编辑行为，请参阅[成本计数器](#cost-counter)。

#### API 拒绝事件

当 API 请求返回 `stop_reason: "refusal"` 时记录。拒绝是在成功的响应流中到达的，而不是作为 HTTP 错误，因此 `api_error` 事件不会为它们触发。此事件可让您跟踪拒绝频率，并按与 `api_request` 和 `api_error` 相同的属性对拒绝进行分组。

**事件名称**：`claude_code.api_refusal`

**属性**：

* 所有[标准属性](#standard-attributes)
* `event.name`：`"api_refusal"`
* `event.timestamp`：ISO 8601 时间戳
* `event.sequence`：用于对事件排序的按进程计数器，详见[事件关联属性](#event-correlation-attributes)
* `model`：来自请求的模型标识符
* `request_id`：API 请求 ID，例如 `"req_011..."`，详见[事件关联属性](#event-correlation-attributes)。
* `query_source`：发出请求的子系统，例如 `"repl_main_thread"`、`"compact"` 或子代理名称。有关定义，请参阅 [`api_request`](#api-request-event)。
* `speed`：当[快速模式](https://code.claude.com/docs/zh-CN/fast-mode.md)处于活跃状态时为 `"fast"`，否则为 `"normal"`
* `attempt`：重试尝试次数。第一次尝试为 `1`。
* `effort`：应用于请求的 [effort 级别](https://code.claude.com/docs/zh-CN/model-config.md#adjust-effort-level)。当 Claude Code 未发送 effort 级别时不存在，例如在不支持 effort 的模型上。
* `server_fallback_hop`：当 API 的服务器端模型回退已经在另一个模型上重试了此次拒绝（因此用户没有看到这次特定的拒绝）时为 `true`。当请求以拒绝结束时为 `false`。当备用模型也拒绝时，单个轮次可以同时发出一个 `true` 跳转事件和一个随后的 `false` 最终事件。
* `has_category`：当 API 响应携带的 `stop_details.category` 为 `"cyber"`、`"bio"`、`"frontier_llm"` 或 `"reasoning_extraction"` 时为 `true`。当响应未携带类别或携带的值不在该集合中时为 `false`。当 `server_fallback_hop` 为 `true` 时不存在，因为跳转块不携带 `stop_details`。
* `has_explanation`：当 API 响应携带 `stop_details.explanation` 时为 `true`，否则为 `false`。当 `server_fallback_hop` 为 `true` 时不存在。
* `category`：来自 API 响应的 `stop_details.category` 值。取值为 `"cyber"`、`"bio"`、`"frontier_llm"` 或 `"reasoning_extraction"` 之一。仅当设置了 `OTEL_LOG_TOOL_DETAILS=1` 且 `has_category` 为 `true` 时存在。
* `agent.name`、`skill.name`、`plugin.name`、`marketplace.name`、`mcp_server.name`、`mcp_tool.name`：请求的 skill、插件、Agent 和 MCP 归因。有关定义和编辑行为，请参阅[成本计数器](#cost-counter)。

#### API 请求体事件

当设置了 `OTEL_LOG_RAW_API_BODIES` 时，针对每次 API 请求尝试记录。每次尝试发出一个事件，因此使用调整后参数进行的重试各自产生自己的事件。

**事件名称**：`claude_code.api_request_body`

**属性**：

* 所有[标准属性](#standard-attributes)
* `event.name`：`"api_request_body"`
* `event.timestamp`：ISO 8601 时间戳
* `event.sequence`：用于对事件排序的按进程计数器，详见[事件关联属性](#event-correlation-attributes)
* `body`：JSON 序列化的 Messages API 请求参数，例如系统提示词、消息和工具，在内容限制处截断（默认 60 KB）。之前 assistant 轮次中的扩展思考内容会被编辑。仅在内联模式（`OTEL_LOG_RAW_API_BODIES=1`）下发出。
* `body_ref`：包含未截断请求体的 `<dir>/<uuid>.request.json` 文件的绝对路径。仅在文件模式（`OTEL_LOG_RAW_API_BODIES=file:<dir>`）下发出。
* `body_length`：未截断的请求体长度。当 `OTEL_LOG_RAW_API_BODIES=file:<dir>` 时为 UTF-8 字节数，当 `=1` 时为 UTF-16 代码单元数
* `body_truncated`：当发生内联截断时为 `"true"`。在文件模式下以及未发生截断时不存在。
* `model`：来自请求参数的模型标识符
* `query_source`：发出请求的子系统（例如 `"compact"`）
* `request_body_id`：标识此次尝试请求体的 UUID。成功的那次尝试对应的 [`api_response_body` 事件](#api-response-body-event)携带相同的值，因此您可以将响应与产生它的确切请求配对。需要 Claude Code v2.1.274 或更高版本

#### API 响应体事件

当设置了 `OTEL_LOG_RAW_API_BODIES` 时，针对每个成功的 API 响应记录。

在文件模式（`OTEL_LOG_RAW_API_BODIES=file:<dir>`）下，Claude Code 还会针对每个成功的响应向 `<dir>/index.jsonl` 追加一行 JSON，包含字段 `timestamp`、`session_id`、`query_source`、`model`、`request_id`、`message_id`、`message_uuid`、`request_file` 和 `response_file`。读取该文件即可找到给定会话记录消息背后的请求和响应文件，而无需查询您的遥测后端。索引文件需要 Claude Code v2.1.274 或更高版本。

**事件名称**：`claude_code.api_response_body`

**属性**：

* 所有[标准属性](#standard-attributes)
* `event.name`：`"api_response_body"`
* `event.timestamp`：ISO 8601 时间戳
* `event.sequence`：用于对事件排序的按进程计数器，详见[事件关联属性](#event-correlation-attributes)
* `body`：JSON 序列化的 Messages API 响应，包括 id、内容块、用量和停止原因，在内容限制处截断（默认 60 KB）。扩展思考内容会被编辑。仅在内联模式（`OTEL_LOG_RAW_API_BODIES=1`）下发出。
* `body_ref`：包含未截断响应体的 `<dir>/<request_id>.response.json` 文件的绝对路径。仅在文件模式（`OTEL_LOG_RAW_API_BODIES=file:<dir>`）下发出。
* `body_length`：未截断的响应体长度。当 `OTEL_LOG_RAW_API_BODIES=file:<dir>` 时为 UTF-8 字节数，当 `=1` 时为 UTF-16 代码单元数
* `body_truncated`：当发生内联截断时为 `"true"`。在文件模式下以及未发生截断时不存在。
* `model`：模型标识符
* `query_source`：发出请求的子系统
* `request_id`：API 请求 ID，例如 `"req_011..."`，详见[事件关联属性](#event-correlation-attributes)。
* `request_body_id`：此响应所对应的 [`api_request_body` 事件](#api-request-body-event)的 `request_body_id`。需要 Claude Code v2.1.274 或更高版本
* `message.id`：API 为响应分配的消息 ID，即响应体的 `id` 字段。需要 Claude Code v2.1.274 或更高版本
* `message.uuid`：响应的最后一个会话记录条目的 UUID。它与 `request_body_id` 一起，将会话记录消息与其背后的请求体和响应体关联起来。需要 Claude Code v2.1.274 或更高版本

#### 工具决策事件

在做出工具权限决策（接受/拒绝）时记录。

**事件名称**：`claude_code.tool_decision`

**属性**：

* 所有[标准属性](#standard-attributes)
* `event.name`：`"tool_decision"`
* `event.timestamp`：ISO 8601 时间戳
* `event.sequence`：用于对事件排序的按进程计数器，详见[事件关联属性](#event-correlation-attributes)
* `tool_name`：工具的名称（例如 "Read"、"Edit"、"Write"、"NotebookEdit"）
* `tool_use_id`：此次工具调用的唯一标识符。与传递给 hook 的 `tool_use_id` 相匹配，从而可以在 OTel 事件和 hook 捕获的数据之间建立关联。
* `request_id`：包含此工具调用的 API 响应的 [`request_id`](#event-correlation-attributes)，以便您将该调用与该响应的 API 事件关联起来。当该响应没有请求 ID，或没有任何 API 响应包含该调用时不包含。需要 Claude Code v2.1.310 或更高版本
* `decision`：`"accept"` 或 `"reject"`
* `tool_source`：始终存在。工具的来源，是由 CLI 定义的封闭值集合：
  * `"builtin"`：CLI 自身的工具
  * `"mcp"`：一般的 MCP 服务器
  * `"sdk_host_builtin_mcp"`：内置于 Claude Desktop 本身的进程内服务器，位于 Claude Desktop 拥有的会话中。当 Claude Desktop 从其自身的某个入口点（`claude-desktop`、`claude-desktop-3p` 或 `local-agent`）启动会话，且该会话不是嵌套子会话时，Claude Desktop 拥有该会话；嵌套会话（包括 Claude Code 自身生成的会话）将这些服务器报告为 `"mcp"`
* `source`：决策的来源：
  * `"config"`：无需提示即自动决定，由以下来源之一决定：

    * 项目设置
    * 用户个人设置中的允许或拒绝规则
    * 企业托管策略
    * `--allowedTools` 或 `--disallowedTools` 标志
    * 当前的权限模式
    * 自动模式分类器，同一事件会在 `classifier_verdict` 中记录其判定结果
    * 在同一交互式 CLI 会话中较早的权限提示中授予的、在会话剩余时间内有效的授权
    * 工具本身是安全的

    `source` 值并不表明匹配的是其中哪个来源。要查明是哪个来源拒绝了调用，请查找具有相同 `tool_use_id` 的[安全事件](#safety-event)，并读取其 `category` 和 `rule_source` 属性。部分被拒绝的调用[没有安全事件](#calls-without-a-safety-event)。

    当权限提示请求本身失败时，Claude Code 也会报告 `"config"`。例如，当 Agent SDK 的 [`canUseTool`](https://code.claude.com/docs/zh-CN/agent-sdk/typescript#canusetool) 回调或 [`--permission-prompt-tool`](https://code.claude.com/docs/zh-CN/cli-reference#cli-flags) 工具返回无效结果时，或者当请求挂起期间输入流关闭时，该请求都会失败。
  * `"hook"`：由 `PreToolUse` 或 `PermissionRequest` hook 返回了该决策。
  * `"user_permanent"`：当用户在权限提示中选择 "Yes, and don't ask again for ..." 时发出，这会将一条允许规则保存到其个人设置中。在交互式 CLI 中，仅针对该选择本身发出；之后匹配已保存规则的调用会改为发出 `"config"`。在 Agent SDK 或非交互式 `-p` 会话中，初始选择和之后的规则匹配都会发出 `"user_permanent"`。视为接受。
  * `"user_temporary"`：当用户在权限提示中选择 "Yes" 进行一次性批准，或在文件编辑或读取提示中选择了在会话剩余时间内授予访问权限的选项时发出。在交互式 CLI 中，仅针对该选择本身发出；之后由该会话范围许可允许的调用会改为发出 `"config"`。在 Agent SDK 或非交互式 `-p` 会话中，该选择和之后的匹配都会发出 `"user_temporary"`。视为接受。
  * `"user_abort"`：当用户未作回答就关闭了权限提示时发出。在 Agent SDK 和非交互式 `-p` 会话中，这包括在 `canUseTool` 或 `--permission-prompt-tool` 权限请求挂起期间中断轮次。视为拒绝。
  * `"user_reject"`：当用户在提示时选择 "No" 时发出。在交互式 CLI 中，仅针对该选择本身发出；匹配用户个人设置中拒绝规则的调用会改为发出 `"config"`。在 Agent SDK 或非交互式 `-p` 会话中，匹配个人设置中拒绝规则的调用会发出 `"user_reject"`。视为拒绝。
* `classifier_verdict`：自动模式[分类器](https://code.claude.com/docs/zh-CN/permission-modes.md#how-the-classifier-evaluates-actions)对该调用的判定结果，或在没有判定结果时 Claude Code 拒绝该调用的原因。需要 Claude Code v2.1.301 或更高版本

  * `"allow"`：分类器批准了该调用
  * `"block"`：分类器阻止了该调用。中途失败的分类器检查也可能以 `"block"` 结束，因此分类器部分中断可能表现为阻止次数增多，而不是 `"unavailable"`
  * `"no_verdict"`：检查已运行，但[没有给出可用的判定结果](https://code.claude.com/docs/zh-CN/errors.md#auto-mode-cannot-determine-the-safety-of-an-action)，因此 Claude Code 拒绝了该调用。该拒绝并不意味着分类器认为该调用不安全
  * `"unavailable"`：发往分类器的请求失败，例如遇到速率限制、超时或连接错误，因此 Claude Code 拒绝了该调用。在使用[服务器端分类器审查](https://code.claude.com/docs/zh-CN/permission-modes.md#server-side-classifier-review)时，它还标记因[服务器未返回安全判定结果](https://code.claude.com/docs/zh-CN/errors.md#the-server-returned-no-safety-verdict)而被拒绝的调用

  当[分类器在计划模式下审查命令](https://code.claude.com/docs/zh-CN/permission-modes.md#analyze-before-you-edit-with-plan-mode)时，该属性也会出现在计划模式的调用上。在以下情况下该属性不存在：

  * 由允许或拒绝[权限规则](https://code.claude.com/docs/zh-CN/permissions.md#manage-permissions)决定了该调用
  * Claude Code 在自动模式下未询问分类器就批准了该调用
  * 用户回答了 Claude Code 在[多次阻止](https://code.claude.com/docs/zh-CN/permission-modes.md#repeated-block-thresholds)后显示的提示
  * 在不带 `--permission-prompt-tool` 的 `-p` 运行中，由于没有人可以回答提示，Claude Code 在多次阻止后拒绝了该调用。该事件将 `decision` 报告为 `"reject"`，将 `source` 报告为 `"config"`
* `classifier_category`（当 `OTEL_LOG_TOOL_DETAILS=1` 时）：分类器阻止该调用时指明的[内置规则](https://code.claude.com/docs/zh-CN/auto-mode-config.md#inspect-the-defaults-and-your-effective-config)名称，采用小写加下划线形式，例如 Data Exfiltration 规则对应 `"data_exfiltration"`。需要 Claude Code v2.1.301 或更高版本。当以下两个条件同时成立时，该属性存在：

  * `classifier_verdict` 为 `"block"`
  * 分类器恰好指明了一条规则，且该规则是内置规则而非您编写的规则

  在 v2.1.302 之前，对于由[服务器端分类器审查](https://code.claude.com/docs/zh-CN/permission-modes.md#server-side-classifier-review)决定的任何阻止，该属性也不存在。
* `permission_mode`：此调用的权限检查开始时处于活动状态的[权限模式](https://code.claude.com/docs/zh-CN/permission-modes.md)：`"default"`、`"plan"`、`"acceptEdits"`、`"bypassPermissions"`、`"dontAsk"` 或 `"auto"`。手动模式显示为 `"default"`。[子代理](https://code.claude.com/docs/zh-CN/sub-agents.md#permission-modes)的调用报告的是子代理的权限模式，而不是会话的权限模式。需要 Claude Code v2.1.301 或更高版本
* `tool_parameters`（当 `OTEL_LOG_TOOL_DETAILS=1` 时）：包含工具特定参数的 JSON 字符串。结构与[工具结果事件](#tool-result-event)相同，但不含 `git_commit_id` 等执行后字段。如果权限决策通过 `updatedInput` 重写了工具输入，对于被接受的调用，其值可能与 `tool_result` 不同。当 `decision` 为 `"reject"` 时，可使用此属性查看哪个命令被拒绝。
  * 对于 `"sdk_host_builtin_mcp"` 工具：即使 `OTEL_LOG_TOOL_DETAILS` 关闭，也会包含 `mcp_server_name` 和 `mcp_tool_name`，因为这些名称由宿主应用定义；如果没有它们，对这些内置服务器之一的被拒绝调用在默认数据流中将无法归属。对于用户配置的 MCP 服务器，事件的 `tool_name` 始终为字面值 `"mcp_tool"`，服务器和工具名称仅在启用该标志时出现在 `tool_parameters` 中；参数内容在任何情况下都需要该标志。
  * 对于 Bash 工具：包括 `bash_command`、`full_command`、`timeout`、`description`、`dangerouslyDisableSandbox`。桌面应用的工作区 bash 工具同样将 `tool_name` 报告为 `Bash`，但仅包括 `bash_command`、`full_command` 和 `timeout`
  * 对于 MCP 工具：包括 `mcp_server_name`、`mcp_tool_name`
  * 对于 Skill 工具：包括 `skill_name`
  * 对于 Agent 工具或旧版 Task 工具：包括 `subagent_type`

如果您将 OpenTelemetry SDK 的 `OTEL_LOGRECORD_ATTRIBUTE_COUNT_LIMIT` 降低到低于 `tool_decision` 事件上的属性数量，SDK 会丢弃 Claude Code 最后设置的属性。SDK 会先于任何其他属性按以下顺序丢弃这些属性，并跳过事件未携带的属性：

1. `parent_agent_id`
2. `agent_id`
3. `request_id`
4. `permission_mode`
5. `classifier_category`
6. `classifier_verdict`

在 Claude Code v2.1.302 或更高版本上，`permission_mode`、`classifier_category` 和 `classifier_verdict` 按此顺序丢弃。`parent_agent_id`、`agent_id` 和 `request_id` 需要 v2.1.310 或更高版本。

#### 安全事件

当某项检查拒绝工具调用或要求权限提示时记录，但[没有安全事件的调用](#calls-without-a-safety-event)除外。当[沙箱的网络代理](https://code.claude.com/docs/zh-CN/sandboxing.md#network-isolation)拒绝连接或请求时，以及当 hook 阻止结果时，也会记录。需要 Claude Code v2.1.307 或更高版本。

该事件与其他事件一起导出，没有单独的设置。该事件自身的属性不包含命令文本、文件路径、主机名或 MCP 服务器和工具名称。该记录还携带每个事件都有的[标准属性](#standard-attributes)，其中可能包括 `workspace.host_paths` 和[仓库属性](#repository-attributes)。要查看事件背后的命令或 MCP 工具，请设置 `OTEL_LOG_TOOL_DETAILS=1`，并读取匹配的 `tool_decision` 事件上的 `tool_parameters`。

**事件名称**：`claude_code.safety_event`

**属性**：

* 所有[标准属性](#standard-attributes)
* `event.name`：`"safety_event"`
* `event.timestamp`：ISO 8601 时间戳
* `event.sequence`：用于对事件排序的按进程计数器，详见[事件关联属性](#event-correlation-attributes)
* `category`：检查或结果，为一个[类别值](#safety-event-categories)
* `decision`：`"accept"`、`"reject"`，或当 hook 阻止了结果或轮次结束时为 `"sent_back"`
* `decision_source`：由谁或什么做出决定，为 [`tool_decision`](#tool-decision-event) 的某个 `source` 值或 `"other"`
* `reason_code`：检查的原因，例如 `"rule:policySettings"`、`"mode:dontAsk"`、`"hook:PreToolUse"`、`"classifier:auto-mode"`、`"bashMiss:too-complex"` 或 `"unknown"`
* `reason_type`：检查的原因类型，例如 `"rule"`、`"subcommandResults"`、`"mode"`、`"hook"`、`"classifier"` 或 `"asyncAgent"`
* `rule_source`：所涉及规则的设置来源，例如 `"policySettings"`、`"userSettings"`、`"projectSettings"`、`"localSettings"`、`"flagSettings"`、`"cliArg"` 或 `"session"`
* `grant_scope`：对于未将规则保存到设置文件即获批准的提示，为 `"once"` 或 `"session"`
* `circuit_breaker`：内置安全检查，例如 `"dangerousRemoval"`、`"outsideReadsBlocked"` 或 `"claudeSettingsFile"`
* `classifier_category`：在 `OTEL_LOG_TOOL_DETAILS=1` 时，为自动模式阻止背后的[内置规则](https://code.claude.com/docs/zh-CN/auto-mode-config.md#inspect-the-defaults-and-your-effective-config)，例如 `"data_exfiltration"`
* `hook_event`：对于 `PreToolUse` hook 为 `"pre_tool_use"`，在 `"hook_review"` 事件上为 `"post_tool_use"`、`"post_tool_use_failure"`、`"stop"` 或 `"subagent_stop"`
* `tool_class`：Bash 和 [Monitor](https://code.claude.com/docs/zh-CN/tools-reference.md#monitor-tool) 为 `"bash"`，`"powershell"`，Read、Edit、Write、NotebookEdit、Glob 和 Grep 为 `"file"`，WebFetch 和 WebSearch 为 `"web"`，`"mcp"`，或 `"other"`
* `tool_name`：工具的名称，例如内置工具为 `"Bash"`，或为 `"mcp_tool"`、`"skill_tool"`、`"dynamic_tool"` 或 `"other_tool"`
* `tool_source`：`"builtin"`、`"mcp"` 或 `"sdk_host_builtin_mcp"`，与 [`tool_decision`](#tool-decision-event) 上的相同
* `permission_mode`：检查时该调用的[权限模式](https://code.claude.com/docs/zh-CN/permission-modes.md)：`"default"`、`"plan"`、`"acceptEdits"`、`"bypassPermissions"`、`"dontAsk"` 或 `"auto"`
* `tool_use_id`：该调用的 ID，与 `tool_decision` 和 `tool_result` 上的相同
* `sandbox_violation_count`：沙箱的网络代理拒绝了来自该调用命令的多少个不同连接或请求，为 `"0"` 到 `"20"` 的字符串。针对在前台完成的沙箱化命令统计拒绝次数
* `sandbox_enabled`：如果会话启用了[沙箱隔离](https://code.claude.com/docs/zh-CN/sandboxing.md)，则为 `"true"`，否则为 `"false"`

以下是托管设置中的拒绝规则拒绝一条 Bash 命令时该事件自身的属性：

```json theme={null}
{
  "event.name": "safety_event",
  "category": "managed_policy",
  "decision": "reject",
  "decision_source": "config",
  "reason_code": "rule:policySettings",
  "rule_source": "policySettings",
  "tool_class": "bash",
  "tool_name": "Bash",
  "tool_source": "builtin",
  "permission_mode": "default",
  "tool_use_id": "toolu_01ABC123...",
  "sandbox_violation_count": "0",
  "sandbox_enabled": "false"
}
```

##### 安全事件类别

| 值 | 检查或结果 | 详情属性 |
| - | - | - |
| `"permission_rule"` | 托管设置之外的拒绝或询问[权限规则](https://code.claude.com/docs/zh-CN/permissions.md#manage-permissions) | `rule_source` |
| `"managed_policy"` | 来自[托管设置](https://code.claude.com/docs/zh-CN/permissions.md#managed-settings)的拒绝或询问规则 | `rule_source` |
| `"permission_mode"` | [`dontAsk`](https://code.claude.com/docs/zh-CN/permission-modes#allow-only-pre-approved-tools-with-dontask-mode) 或[计划](https://code.claude.com/docs/zh-CN/permission-modes.md#analyze-before-you-edit-with-plan-mode)模式，或者在无法显示提示的会话中（例如使用 [`--permission-prompts none`](https://code.claude.com/docs/zh-CN/headless#turn-off-permission-prompts-in-unattended-runs) 时）出现的[无类别](#prompts-with-no-category)提示 | `reason_code`、`reason_type` |
| `"hook"` | 某个 hook，例如 [`PreToolUse`](https://code.claude.com/docs/zh-CN/hooks#pretooluse) 或 [`PermissionRequest`](https://code.claude.com/docs/zh-CN/hooks#permissionrequest) | `hook_event`、`reason_code` |
| `"auto_mode_block"` | 自动模式[分类器](https://code.claude.com/docs/zh-CN/permission-modes.md#how-the-classifier-evaluates-actions)阻止了该调用 | `classifier_category` |
| `"auto_mode_no_verdict"` | 没有可用的[分类器判定结果](https://code.claude.com/docs/zh-CN/errors.md#auto-mode-cannot-determine-the-safety-of-an-action)，例如在请求失败之后 | `reason_code` |
| `"safety_check"` | 内置安全检查，例如针对在您的[工作目录](https://code.claude.com/docs/zh-CN/permissions.md#working-directories)之外执行的 Read、Edit 或 Write | `circuit_breaker` |
| `"static_analysis"` | Bash 或 Monitor 命令的写法，例如过于复杂而无法分析的命令 | `reason_code` |
| `"sandbox_override"` | [在沙箱之外运行命令](https://code.claude.com/docs/zh-CN/sandboxing.md#the-unsandboxed-retry-escape-hatch)的提示 | `reason_code` |
| `"user_rejected"` | 有人拒绝或关闭了[无类别](#prompts-with-no-category)的提示或仅由 hook 要求的提示，或者 `canUseTool` 回调或 `--permission-prompt-tool` 工具拒绝了此类提示 | `decision_source` |
| `"sandbox_network"` | 调用已运行，且[沙箱的网络代理](https://code.claude.com/docs/zh-CN/sandboxing.md#network-isolation)拒绝了某个连接或请求 | `sandbox_violation_count` |
| `"sandbox_other"` | 类似 `"sandbox_network"`，但没有任何被拒绝的目标可识别为主机名或 IP 地址 | `sandbox_violation_count` |
| `"hook_review"` | 某个 hook [阻止了调用的结果](https://code.claude.com/docs/zh-CN/hooks.md#posttooluse-decision-control)或[轮次的结束](https://code.claude.com/docs/zh-CN/hooks.md#stop-decision-control) | `hook_event` |
| `"other"` | 其他任何值都无法描述的拒绝 | `reason_code` |

要在任何 `category` 下查找代理拒绝，请筛选存在且不为 `"0"` 的 `sandbox_violation_count`。

##### 没有安全事件的调用

要查找因无人能回答[无类别提示](#prompts-with-no-category)而被拒绝的调用，请查找报告 `"reject"` 且 `source` 为 `"config"`、并且没有安全事件与其共享 `tool_use_id` 的 [`tool_decision`](#tool-decision-event) 事件。此类拒绝发生在既未使用 [`--permission-prompt-tool`](https://code.claude.com/docs/zh-CN/cli-reference#cli-flags) 也未使用 `--permission-prompts none` 的 `-p` 运行中。

对于以下这类调用，请检查其 `tool_decision` 事件，因为除了在其运行后发出的 [`"sandbox_network"`、`"sandbox_other"` 或 `"hook_review"` 事件](#safety-event-categories)之外，它们没有安全事件：

* 在无类别提示中获得批准的调用
* 在仅由 `PreToolUse` hook 要求的提示中，由 `canUseTool` 回调或 `--permission-prompt-tool` 工具批准的调用
* [`AskUserQuestion`](https://code.claude.com/docs/zh-CN/tools-reference#askuserquestion-tool-behavior) 或 `ExitPlanMode` 调用，无论是否被拒绝

##### 无类别的提示

当没有任何 [`category` 值](#safety-event-categories)命名某个提示的检查时，该提示即无类别，例如针对工作目录内编辑的 `default` 模式提示。

#### 权限模式变更事件

在权限模式发生变化时记录，例如通过 `Shift+Tab` 循环切换、退出计划模式或自动模式门控检查。

要按每个工具调用接受检查时所处的权限模式对其分组，请使用[工具决策事件](#tool-decision-event)上的 `permission_mode`。

**事件名称**：`claude_code.permission_mode_changed`

**属性**：

* 所有[标准属性](#standard-attributes)
* `event.name`：`"permission_mode_changed"`
* `event.timestamp`：ISO 8601 时间戳
* `event.sequence`：用于对事件排序的按进程计数器，详见[事件关联属性](#event-correlation-attributes)
* `from_mode`：先前的权限模式，例如 `"default"`、`"plan"`、`"acceptEdits"`、`"auto"` 或 `"bypassPermissions"`
* `to_mode`：新的权限模式
* `trigger`：导致变更的原因，例如 `"shift_tab"`、`"exit_plan_mode"` 或 `"auto_gate_denied"`。对于某些变更不存在，例如当 SDK 通过 [`setPermissionMode()`](https://code.claude.com/docs/zh-CN/agent-sdk/typescript#query-object) 设置权限模式时

#### 身份验证事件

在 `/login` 或 `/logout` 完成时记录。

**事件名称**：`claude_code.auth`

**属性**：

* 所有[标准属性](#standard-attributes)
* `event.name`：`"auth"`
* `event.timestamp`：ISO 8601 时间戳
* `event.sequence`：用于对事件排序的按进程计数器，详见[事件关联属性](#event-correlation-attributes)
* `action`：`"login"` 或 `"logout"`
* `success`：`"true"` 或 `"false"`
* `auth_method`：身份验证方法，例如 `"oauth"`
* `error_category`：操作失败时的错误类别。永远不会包含原始错误消息
* `status_code`：当操作因 HTTP 错误而失败时，以字符串表示的 HTTP 状态码

#### MCP 服务器连接事件

在 MCP 服务器连接、断开连接或连接失败时记录。

**事件名称**：`claude_code.mcp_server_connection`

**属性**：

* 所有[标准属性](#standard-attributes)
* `event.name`：`"mcp_server_connection"`
* `event.timestamp`：ISO 8601 时间戳
* `event.sequence`：用于对事件排序的按进程计数器，详见[事件关联属性](#event-correlation-attributes)
* `status`：`"connected"`、`"failed"` 或 `"disconnected"`
* `transport_type`：服务器传输方式，例如 `"stdio"`、`"sse"` 或 `"http"`
* `server_scope`：服务器所配置的作用域，例如 `"user"`、`"project"` 或 `"local"`
* `duration_ms`：连接尝试持续时间（以毫秒为单位）
* `error_code`：连接失败时的错误代码
* `is_plugin`：当服务器由插件提供时为 `true`，否则为 `false`
* `plugin_id_hash`（当 `is_plugin` 为 `true` 时）：插件名称和市场的稳定哈希，用于在不暴露名称的情况下按插件对事件进行分组。Claude Code 按照[插件已加载事件](#plugin-loaded-event)中描述的方式计算它
* `plugin.name`（当 `is_plugin` 为 `true` 时）：提供该服务器的插件名称。对于第三方插件，除非设置了 `OTEL_LOG_TOOL_DETAILS=1`，否则这是字面字符串 `"third-party"`；这可以防止第三方插件名称默认出现在日志中。来自 Anthropic 官方来源的插件始终以名称标识。`plugin_id_hash` 和 `plugin.name` 属性会流向您自己的监控后端，不会发送给 Anthropic
* `server_name`（当 `OTEL_LOG_TOOL_DETAILS=1` 时）：配置的服务器名称
* `error`（当 `OTEL_LOG_TOOL_DETAILS=1` 时）：连接失败时的完整错误消息

#### 内部错误事件

当 Claude Code 捕获到意外的内部错误时记录。仅记录错误类名和 errno 风格的代码。永远不会包含错误消息和堆栈跟踪。在针对 Amazon Bedrock、Google Cloud 的 Agent Platform 或 Microsoft Foundry 运行时，或者设置了 `DISABLE_ERROR_REPORTING` 时，不会发出此事件。

**事件名称**：`claude_code.internal_error`

**属性**：

* 所有[标准属性](#standard-attributes)
* `event.name`：`"internal_error"`
* `event.timestamp`：ISO 8601 时间戳
* `event.sequence`：用于对事件排序的按进程计数器，详见[事件关联属性](#event-correlation-attributes)
* `error_name`：错误类名，例如 `"TypeError"` 或 `"SyntaxError"`
* `error_code`：错误上存在时的 Node.js errno 代码，例如 `"ENOENT"`

#### 插件已安装事件

在插件完成安装时记录，来源包括 `claude plugin install` CLI 命令和交互式 `/plugin` UI。

**事件名称**：`claude_code.plugin_installed`

**属性**：

* 所有[标准属性](#standard-attributes)
* `event.name`：`"plugin_installed"`
* `event.timestamp`：ISO 8601 时间戳
* `event.sequence`：用于对事件排序的按进程计数器，详见[事件关联属性](#event-correlation-attributes)
* `marketplace.is_official`：如果市场是 Anthropic 官方市场则为 `"true"`，否则为 `"false"`
* `install.trigger`：`"cli"` 或 `"ui"`
* `plugin.name`：已安装插件的名称。对于第三方市场，仅在 `OTEL_LOG_TOOL_DETAILS=1` 时包含
* `plugin.version`：在市场条目中声明的插件版本。对于第三方市场，仅在 `OTEL_LOG_TOOL_DETAILS=1` 时包含
* `marketplace.name`：插件的安装来源市场。对于第三方市场，仅在 `OTEL_LOG_TOOL_DETAILS=1` 时包含

#### 插件已加载事件

在会话开始时针对每个已启用的插件记录一次。使用此事件可以盘点整个设备群中哪些插件处于活跃状态，作为记录安装操作本身的 `plugin_installed` 的补充。

**事件名称**：`claude_code.plugin_loaded`

**属性**：

* 所有[标准属性](#standard-attributes)
* `event.name`：`"plugin_loaded"`
* `event.timestamp`：ISO 8601 时间戳
* `event.sequence`：用于对事件排序的按进程计数器，详见[事件关联属性](#event-correlation-attributes)
* `plugin.name`：插件的名称。对于官方市场和内置捆绑包之外的插件，除非设置了 `OTEL_LOG_TOOL_DETAILS=1`，否则值为 `"third-party"`
* `marketplace.name`：插件的安装来源市场（已知时）。在与 `plugin.name` 相同的条件下编辑为 `"third-party"`
* `plugin.version`：来自插件清单的版本。仅当名称未被编辑且清单声明了版本时包含
* `plugin.scope`：插件的来源类别：`"official"`、`"community"`、`"org"`、`"user-local"` 或 `"default-bundle"`
* `enabled_via`：插件被启用的方式：`"default-enable"`、`"org-policy"`、`"admin-install"`、`"seed-mount"` 或 `"user-install"`。`"admin-install"` 值表示该插件在 [**Organization settings > Plugins & skills**](https://claude.ai/admin-settings/skills?tab=inventory) 中被设置为您组织的必需或自动安装插件。在 v2.1.246 之前，Claude Code 将这些插件报告为 `"user-install"` 或 `"seed-mount"`
* `plugin_id_hash`：插件名称和市场的确定性哈希，仅发送到您配置的导出器。使您能够在不记录名称的情况下统计整个设备群中加载的不同第三方插件的数量。对于[从 claude.ai 同步的插件](https://code.claude.com/docs/zh-CN/plugins/loading.md#synced-plugins)，Claude Code 会将插件名称与 claude.ai 为该插件报告的市场名称一起进行哈希，否则与 `synced` 一起进行哈希。在 v2.1.246 之前，Claude Code 在哈希中不使用 claude.ai 报告的市场名称
* `has_hooks`：插件是否通过 [hook 配置](https://code.claude.com/docs/zh-CN/plugins/components.md#hooks)或作为 [mod](https://code.claude.com/docs/zh-CN/plugins/mods/overview.md#how-a-mod-works) 提供 hook
* `has_mcp`：插件是否提供 MCP 服务器
* `host_owned_mcp`：当 SDK 主机管理此插件的 MCP 连接且 Claude Code 跳过了读取插件的 MCP 服务器配置时为 `true`，否则为 `false`
* `skill_path_count`：插件声明的 skill 目录数量
* `command_path_count`：插件声明的命令目录数量
* `agent_path_count`：插件声明的 Agent 目录数量
* `safe_mode`：当会话使用 [`--safe-mode`](https://code.claude.com/docs/zh-CN/cli-reference) 启动时为 `"true"`，否则为 `"false"`。在安全模式下，此事件仅报告已配置的清单；插件的命令、skill、hook 和 MCP 服务器不会加载。需要 Claude Code v2.1.169 或更高版本

#### Skill 已激活事件

在调用 skill 时记录，无论是 Claude 通过 Skill 工具调用它，还是您以 `/` 命令的形式运行它。

**事件名称**：`claude_code.skill_activated`

**属性**：

* 所有[标准属性](#standard-attributes)
* `event.name`：`"skill_activated"`
* `event.timestamp`：ISO 8601 时间戳
* `event.sequence`：用于对事件排序的按进程计数器，详见[事件关联属性](#event-correlation-attributes)
* `skill.name`：skill 的名称。对于用户定义的和第三方插件的 skill，除非设置了 `OTEL_LOG_TOOL_DETAILS=1`，否则值为占位符 `"custom_skill"`
* `invocation_trigger`：skill 的触发方式（`"user-slash"`、`"claude-proactive"` 或 `"nested-skill"`）
* `skill.source`：skill 的加载来源（例如 `"bundled"`、`"userSettings"`、`"projectSettings"`、`"plugin"`）
* `skill.kind`：当 skill 是工作流 skill 时为 `"workflow"`。否则不存在
* `plugin.name`（当 `OTEL_LOG_TOOL_DETAILS=1` 或插件来自官方市场时）：当 skill 由插件提供时，所属插件的名称
* `marketplace.name`（当 `OTEL_LOG_TOOL_DETAILS=1` 或插件来自官方市场时）：当 skill 由插件提供时，所属插件的安装来源市场

#### @ 提及事件

当 Claude Code 解析提示词中的 `@` 提及时记录。并非每个提及都会发出事件：提前退出的路径，例如权限拒绝、文件过大、PDF 引用附件以及目录列表失败，会在不记录日志的情况下返回。

每次 Claude Code 读取一个提示词时，最多记录 100 个 `mention_type` 为 `"agent"` 的事件和 100 个 `"mcp_resource"` 的事件。超过任一限制的提及仍会被解析，但不会发出事件。

**事件名称**：`claude_code.at_mention`

**属性**：

* 所有[标准属性](#standard-attributes)
* `event.name`：`"at_mention"`
* `event.timestamp`：ISO 8601 时间戳
* `event.sequence`：用于对事件排序的按进程计数器，详见[事件关联属性](#event-correlation-attributes)
* `mention_type`：提及的类型（`"file"`、`"directory"`、`"agent"`、`"mcp_resource"`、`"peer"`）。`"peer"` 值表示您提及了[您的另一个 Claude Code 会话](https://code.claude.com/docs/zh-CN/cross-session-messaging.md)。需要 Claude Code v2.1.232 或更高版本
* `success`：提及是否成功解析（`"true"` 或 `"false"`）

#### API 重试耗尽事件

当 API 请求在多次尝试后失败时记录一次。与最终的 `api_error` 事件一起发出。

**事件名称**：`claude_code.api_retries_exhausted`

**属性**：

* 所有[标准属性](#standard-attributes)
* `event.name`：`"api_retries_exhausted"`
* `event.timestamp`：ISO 8601 时间戳
* `event.sequence`：用于对事件排序的按进程计数器，详见[事件关联属性](#event-correlation-attributes)
* `model`：使用的模型
* `error`：最终错误消息
* `status_code`：以数字表示的 HTTP 状态码。对于非 HTTP 错误不存在。
* `total_attempts`：已进行的尝试总次数
* `total_retry_duration_ms`：所有尝试的总挂钟时间
* `speed`：`"fast"` 或 `"normal"`

#### Hook 已注册事件

在会话开始时针对每个已配置的 hook 记录一次。使用此事件可以盘点整个设备群中哪些 hook 处于活跃状态，作为按执行记录的 `hook_execution_start` 和 `hook_execution_complete` 事件的补充。

**事件名称**：`claude_code.hook_registered`

**属性**：

* 所有[标准属性](#standard-attributes)
* `event.name`：`"hook_registered"`
* `event.timestamp`：ISO 8601 时间戳
* `event.sequence`：用于对事件排序的按进程计数器，详见[事件关联属性](#event-correlation-attributes)
* `hook_event`：hook 事件类型，例如 `"PreToolUse"` 或 `"PostToolUse"`
* `hook_type`：hook 实现类型：`"command"`、`"prompt"`、`"mcp_tool"`、`"http"` 或 `"agent"`
* `hook_source`：hook 的定义位置：`"userSettings"`、`"projectSettings"`、`"localSettings"`、`"flagSettings"`、`"policySettings"` 或 `"pluginHook"`
* `safe_mode`：当会话使用 [`--safe-mode`](https://code.claude.com/docs/zh-CN/cli-reference) 启动时为 `"true"`，否则为 `"false"`。需要 Claude Code v2.1.169 或更高版本
* `hook_matcher`（当 `OTEL_LOG_TOOL_DETAILS=1` 时）：hook 配置中的匹配器字符串（如果设置了的话）
* `plugin.name`（当 `hook_source` 为 `"pluginHook"` 时）：提供该 hook 的插件名称。对于官方市场和内置捆绑包之外的插件，除非设置了 `OTEL_LOG_TOOL_DETAILS=1`，否则值为 `"third-party"`
* `plugin_id_hash`（当 `hook_source` 为 `"pluginHook"` 时）：插件名称和市场的确定性哈希，仅发送到您配置的导出器。使您能够在不记录名称的情况下统计提供 hook 的不同插件的数量。Claude Code 按照[插件已加载事件](#plugin-loaded-event)中描述的方式计算它

#### Hook 执行开始事件

当一个或多个 hook 开始为某个 hook 事件执行时记录。

**事件名称**：`claude_code.hook_execution_start`

**属性**：

* 所有[标准属性](#standard-attributes)
* `event.name`：`"hook_execution_start"`
* `event.timestamp`：ISO 8601 时间戳
* `event.sequence`：用于对事件排序的按进程计数器，详见[事件关联属性](#event-correlation-attributes)
* `hook_event`：hook 事件类型，例如 `"PreToolUse"` 或 `"PostToolUse"`
* `hook_name`：包含匹配器的完整 hook 名称，例如 `"PreToolUse:Write"`
* `num_hooks`：匹配的 hook 命令数量
* `managed_only`：当仅允许托管策略 hook 时为 `"true"`
* `hook_source`：`"policySettings"` 或 `"merged"`
* `safe_mode`：当会话使用 [`--safe-mode`](https://code.claude.com/docs/zh-CN/cli-reference) 启动时为 `"true"`，否则为 `"false"`。需要 Claude Code v2.1.169 或更高版本
* `hook_definitions`：JSON 序列化的 hook 配置。仅当同时启用了详细 beta 追踪和 `OTEL_LOG_TOOL_DETAILS=1` 时包含

#### Hook 执行完成事件

当某个 hook 事件的所有 hook 都已完成时记录。

**事件名称**：`claude_code.hook_execution_complete`

**属性**：

* 所有[标准属性](#standard-attributes)
* `event.name`：`"hook_execution_complete"`
* `event.timestamp`：ISO 8601 时间戳
* `event.sequence`：用于对事件排序的按进程计数器，详见[事件关联属性](#event-correlation-attributes)
* `hook_event`：hook 事件类型
* `hook_name`：包含匹配器的完整 hook 名称
* `num_hooks`：匹配的 hook 命令数量
* `num_success`：成功完成的数量
* `num_blocking`：返回阻止决策的数量
* `num_non_blocking_error`：失败但未阻止的数量
* `num_cancelled`：在完成前被取消的数量
* `total_duration_ms`：所有匹配 hook 的挂钟持续时间
* `stdout_chars`：成功执行的匹配 hook 的 stdout 总字符数。需要 Claude Code v2.1.280 或更高版本
* `additional_context_chars`：匹配 hook 返回的 `additionalContext` 总字符数。需要 Claude Code v2.1.280 或更高版本
* `system_message_chars`：匹配 hook 返回的 `systemMessage` 总字符数。需要 Claude Code v2.1.280 或更高版本
* `initial_user_message_chars`：匹配 hook 返回的 `initialUserMessage` 总字符数。需要 Claude Code v2.1.280 或更高版本
* `num_outputs_persisted`：超过 [10,000 字符上限](https://code.claude.com/docs/zh-CN/hooks.md#json-output)并被 Claude Code 保存到文件中的 hook 输出数量。需要 Claude Code v2.1.280 或更高版本
* `managed_only`：当仅允许托管策略 hook 时为 `"true"`
* `hook_source`：`"policySettings"` 或 `"merged"`
* `safe_mode`：当会话使用 [`--safe-mode`](https://code.claude.com/docs/zh-CN/cli-reference) 启动时为 `"true"`，否则为 `"false"`。需要 Claude Code v2.1.169 或更高版本
* `hook_definitions`：JSON 序列化的 hook 配置。仅当同时启用了详细 beta 追踪和 `OTEL_LOG_TOOL_DETAILS=1` 时包含

#### Hook 插件指标事件

当官方市场插件的 hook 发出每次调用的指标时记录。只有从 Anthropic 官方市场安装的插件才能发出这些指标。第三方市场插件和用户配置的 hook 不会向此事件发出数据。使用此事件可以从您自己的可观测性技术栈监控插件行为，例如发现率、成本和持续时间。

**事件名称**：`claude_code.hook_plugin_metrics`

**属性**：

* 所有[标准属性](#standard-attributes)
* `event.name`：`"hook_plugin_metrics"`
* `event.timestamp`：ISO 8601 时间戳
* `event.sequence`：用于对事件排序的按进程计数器，详见[事件关联属性](#event-correlation-attributes)
* `plugin_id`：`<name>@<marketplace>` 形式的插件标识符
* `hook_event`：发出指标的 hook 事件类型
* 最多 20 个由插件发出的指标键。名称匹配 `^[a-z][a-z0-9_]{0,39}$`。值为布尔值或数字。

#### 上下文压缩事件

在对话压缩完成时记录。

**事件名称**：`claude_code.compaction`

**属性**：

* 所有[标准属性](#standard-attributes)
* `event.name`：`"compaction"`
* `event.timestamp`：ISO 8601 时间戳
* `event.sequence`：用于对事件排序的按进程计数器，详见[事件关联属性](#event-correlation-attributes)
* `trigger`：`"auto"` 或 `"manual"`
* `success`：`"true"` 或 `"false"`
* `duration_ms`：压缩持续时间
* `pre_tokens`：压缩前的大致 token 数量
* `post_tokens`：压缩后的大致 token 数量
* `error`：压缩失败时的错误消息
* `precompute_reuse`：仅在 `trigger` 为 `"manual"` 时设置，或在交互式终端会话中进行[空闲压缩](https://code.claude.com/docs/zh-CN/settings-reference.md#idlecompaction)且 `trigger` 为 `"auto"` 时设置。自动压缩可以在上下文窗口填满之前在后台预先准备摘要，此属性记录压缩是否复用了该预先准备的摘要。`"hit"` 表示已复用；`"miss_custom_instructions"`、`"miss_hook"` 和 `"miss_not_ready"` 给出改为重新计算摘要的原因

#### 子代理完成事件

当[子代理](https://code.claude.com/docs/zh-CN/sub-agents.md)完成并将其结果返回给启动它的对话时记录。可使用此事件按子代理类型汇总工具使用情况和运行时间；如需汇总 token 或成本，请使用按 `query_source` `"subagent"` 筛选的 [token 计数器](#token-counter)和[成本计数器](#cost-counter)，因为此事件的 `total_tokens` 仅涵盖最后一个请求。`"subagent"` 类别还会统计来自基于 Agent 的 hook 的请求，这类 hook 不会发出子代理事件。

**事件名称**：`claude_code.subagent_completed`

**属性**：

* 所有[标准属性](#standard-attributes)
* `event.name`：`"subagent_completed"`
* `event.timestamp`：ISO 8601 时间戳
* `event.sequence`：用于对事件排序的按进程计数器，详见[事件关联属性](#event-correlation-attributes)
* `agent_type`：子代理类型。内置 Agent 名称以及来自官方市场插件的 Agent 会原样显示；除非设置了 `OTEL_LOG_TOOL_DETAILS=1`，否则其他 Agent 名称会被替换为 `"custom"`
* `agent.source`：Agent 定义的来源：`built-in`、`plugin`，或定义自定义 Agent 的设置来源，例如 `userSettings` 或 `projectSettings`
* `is_built_in`：该子代理是否为内置 Agent 类型
* `is_async`：该子代理是否在[后台](https://code.claude.com/docs/zh-CN/sub-agents.md#run-subagents-in-foreground-or-background)运行
* `total_tokens`：子代理最后一个 API 请求的 token 占用量：即该单个请求的输入、缓存创建、缓存读取和输出 token，大致相当于子代理完成时的上下文大小。并非整个运行过程的总和
* `total_tool_uses`：子代理在整个运行过程中进行的工具调用次数
* `duration_ms`：运行时间（毫秒）
* `model`：子代理被解析为运行的模型
* `final_model`：生成子代理最终响应的模型；在运行中途发生切换（例如回退）后，它会与 `model` 不同
* `model_swapped`：是否有多个模型处理了该子代理的请求
* `plugin_id_hash`、`plugin.name`：对于由插件提供的 Agent 存在。官方市场插件名称会原样显示；除非设置了 `OTEL_LOG_TOOL_DETAILS=1`，否则其他插件名称会被替换为 `"third-party"`

#### 反馈调查事件

在显示或回答会话质量调查时记录。有关调查收集的内容以及如何控制调查，请参阅[会话质量调查](https://code.claude.com/docs/zh-CN/data-usage.md#session-quality-surveys)。

**事件名称**：`claude_code.feedback_survey`

**属性**：

* 所有[标准属性](#standard-attributes)
* `event.name`：`"feedback_survey"`
* `event.timestamp`：ISO 8601 时间戳
* `event.sequence`：用于对事件排序的按进程计数器，详见[事件关联属性](#event-correlation-attributes)
* `event_type`：调查生命周期事件，例如 `"appeared"`、`"responded"` 或 `"transcript_prompt_appeared"`
* `appearance_id`：关联同一调查实例所发出事件的唯一 ID
* `survey_type`：生成该事件的调查。`"session"` 是 "How is Claude doing?" 评分提示
* `response`：用户在 `responded` 事件中的选择
* `enabled_via_override`：设置了 [`CLAUDE_CODE_ENABLE_FEEDBACK_SURVEY_FOR_OTEL`](https://code.claude.com/docs/zh-CN/env-vars) 时为 `true`。以布尔值而非字符串形式发出。存在于 `session` 调查事件中。可按此属性筛选，以确认该覆盖已在整个设备群中生效

#### 保留清理事件

保留清理每运行一次记录一次。该清理会删除早于 [`cleanupPeriodDays`](https://code.claude.com/docs/zh-CN/settings-reference#cleanupperioddays) 设置的[会话记录和其他应用程序数据](https://code.claude.com/docs/zh-CN/claude-directory.md#cleaned-up-automatically)。Claude Code 在后台运行清理，每个会话最多运行一次，即使某次运行未删除任何内容，也会发出该事件。如果 Claude Code 在过去 24 小时内已在同一台机器上的任何会话中运行过清理，则会将本会话的清理延迟至少 10 分钟，因此提前退出的会话不会发出任何事件。当您使用 `--bare` 运行 `claude -p` 时，Claude Code 不会运行清理，也不会发出任何事件。

与本页上的所有 OTel 事件一样，该事件仅发送到您配置的遥测后端。需要 Claude Code v2.1.227 或更高版本。

当 Claude Code 无法安全地确定保留期限时，会暂停清理，并发出 `result` 设置为 `"skipped"` 且带有 `skip_reason` 的事件。当[托管设置](https://code.claude.com/docs/zh-CN/server-managed-settings.md)设置了 `cleanupPeriodDays` 时，托管值会固定保留期限，即使较低优先级作用域中的设置文件损坏或无效，清理也会运行。当 `managed-settings.json` 本身无法读取时，Claude Code 仍会暂停清理，除非[托管层级](https://code.claude.com/docs/zh-CN/managed-settings.md#how-claude-code-combines-managed-sources)从其他位置提供了 `cleanupPeriodDays`，例如服务器托管设置，或位于损坏文件旁的 `managed-settings.d/` 附加文件。删除计数器属性仅在 `result` 为 `"complete"` 时存在。

**事件名称**：`claude_code.retention_sweep`

**属性**：

* 所有[标准属性](#standard-attributes)
* `event.name`：`"retention_sweep"`
* `event.timestamp`：ISO 8601 时间戳
* `event.sequence`：用于对事件排序的按进程计数器，详见[事件关联属性](#event-correlation-attributes)
* `result`：清理已运行时为 `"complete"`，Claude Code 暂停清理时为 `"skipped"`
* `period_days`：合并后设置中的 `cleanupPeriodDays` 值（以天为单位），如果没有任何来源设置该值，则为 `30`。在 skipped 事件中，为清理本应使用的值，根据 Claude Code 能够读取的设置来源计算得出
* `used_default`：没有可读取的设置来源设置 `cleanupPeriodDays` 时为 `"true"`，否则为 `"false"`。在 complete 事件中，`"true"` 表示应用了 30 天的默认值
* `skip_reason`：Claude Code 暂停清理的原因。仅当 `result` 为 `"skipped"` 时存在：
  * `"user_source_disabled"`：用户设置被排除（例如通过 [`--setting-sources`](https://code.claude.com/docs/zh-CN/cli-reference#cli-flags) 标志或 SDK 的 [`settingSources`](https://code.claude.com/docs/zh-CN/agent-sdk/typescript#options) 选项），且没有启用的来源提供 `cleanupPeriodDays`
  * `"settings_unknowable"`：某个设置文件无法读取或解析，因此 `cleanupPeriodDays` 或 `desktopSessionCleanupPeriodDays` 可能被设置为 Claude Code 看不到的值
  * `"settings_invalid_key_set"`：设置存在验证错误，且显式设置了 `cleanupPeriodDays` 或 `desktopSessionCleanupPeriodDays`，因此回退到默认值可能会违背该设置删除或保留文件
  * `"remote_settings_unknowable"`：第三方部署上 Claude Desktop 的 Code 选项卡中的某个会话请求了[服务器托管设置](https://code.claude.com/docs/zh-CN/server-managed-settings.md)。它没有收到可用的响应，因此 `cleanupPeriodDays` 可能被设置为 Claude Code 看不到的值
* `transcripts_deleted`：清理删除的会话记录数量，即顶层的 `~/.claude/projects/*/*.jsonl` 文件
* `transcripts_exempted_desktop`：已超过保留期限、但清理根据 [Claude Desktop 和 Cowork 规则](https://code.claude.com/docs/zh-CN/claude-directory.md#cleaned-up-automatically)予以保留的会话记录数量。这些不计入 `files_past_cutoff`。需要 Claude Code v2.1.248 或更高版本
* `session_files_deleted`：会话文件清理删除的文件数量：会话记录加上每个会话的配套文件，例如附属文件、录制文件和工具结果
* `artifacts_deleted`：清理在其覆盖的数据目录中删除的项目总数，包括会话文件。某些清理会将整个被删除的目录树计为一项，少数清理过程不会计入该计数器，因此应将该值视为下限，而非精确的文件数量
* `files_retained_fresh`：已检查并因仍在保留期限内而保留的文件。只有逐文件清理会统计这些文件，因此该值为下限；非零值是正常的稳定状态
* `files_past_cutoff`：早于保留期限、但清理未能删除的文件，例如由于权限错误或文件被占用。该计数还包括清理在 `skills/synced/` 或 `plugins/synced/` 下发现的每个过期文件夹，无论是否将该文件夹移至回收站。除这些文件夹外，大于零的值表示有文件超出了配置的保留期限仍然存在；零并不能证明没有此类文件，因为删除整个目录失败时会计入 `error_count`
* `error_count`：清理在列出或删除文件时遇到的错误数量

#### 托管设置解析事件

记录会话解析出的[托管设置](https://code.claude.com/docs/zh-CN/managed-settings.md)：在会话开始时记录一次；在会话期间托管设置或[策略辅助程序](https://code.claude.com/docs/zh-CN/managed-settings.md#compute-the-policy-with-a-helper-program)的状态发生变化时再次记录；以及当 Claude Code 因 `error.type` 属性列出的某个原因拒绝启动或结束会话时记录。
可使用此事件查找运行在意外托管来源上的机器、策略辅助程序出现故障的机器，以及机器拒绝启动的原因。
需要 Claude Code v2.1.274 或更高版本。

默认情况下，该事件包含托管来源和策略辅助程序的状态，但不包含设置本身。要添加经过脱敏的 `managed_settings.settings` 属性和 `managed_settings.resolved_sha256` 摘要，请设置 `OTEL_LOG_MANAGED_SETTINGS=1`：

* 在托管设置、用户设置或 `--settings` 的 `env` 块中设置，或在启动 Claude Code 的环境中设置。项目设置或本地设置中的值不会启用它，因为克隆的仓库可以写入这些设置。
* 服务器托管设置可以设置它，而无需显示[安全批准对话框](https://code.claude.com/docs/zh-CN/server-managed-settings.md#security-approval-dialogs)，因为该变量只是将您组织自己的脱敏策略添加到您组织已在接收的事件中。

在您尚未[信任](https://code.claude.com/docs/zh-CN/permissions.md#what-runs-before-you-trust-a-folder)的文件夹中的交互式会话里，Claude Code 不会导出拒绝事件。

**事件名称**：`claude_code.managed_settings_resolved`

**属性**：

* 所有[标准属性](#standard-attributes)
* `event.name`：`"managed_settings_resolved"`
* `event.timestamp`：ISO 8601 时间戳
* `event.sequence`：用于对事件排序的按进程计数器，详见[事件关联属性](#event-correlation-attributes)
* `managed_settings.trigger`：会话开始事件为 `"startup"`；托管设置或策略辅助程序的状态在会话后期发生变化时为 `"change"`；托管设置策略阻止会话时为 `"refused"`。仅当某个属性与上次发送的事件不同时，Claude Code 才会发送 `change` 事件；即使 `OTEL_LOG_MANAGED_SETTINGS` 处于关闭状态，设置值的变化也会计入
* `error.type`：Claude Code 停止会话的原因。仅存在于 `refused` 事件中：
  * `"helper_failed"`：[策略辅助程序运行失败](https://code.claude.com/docs/zh-CN/settings-reference.md#helper-failures)
  * `"policy_invalid"`：托管设置包含阻止 Claude Code 启动的错误，或某个管理员来源因读取被拒绝以外的原因加载失败，导致 Claude Code 无法检查组织登录或提供商强制要求
  * `"provider_not_allowed"`：会话将使用托管 [`allowedProviders`](https://code.claude.com/docs/zh-CN/settings-reference#allowedproviders) 列表不允许的 API 提供商，或将某个提供商的流量发送到该列表不允许的主机。需要 Claude Code v2.1.285 或更高版本
  * `"consent_rejected"`：用户拒绝了服务器托管设置的[安全批准对话框](https://code.claude.com/docs/zh-CN/server-managed-settings.md#security-approval-dialogs)
  * `"force_refresh_failed"`：[`forceRemoteSettingsRefresh`](https://code.claude.com/docs/zh-CN/settings-reference#forceremotesettingsrefresh) 所要求的设置获取失败
  * `"gateway_rejected"`：[Claude apps gateway](https://code.claude.com/docs/zh-CN/claude-apps-gateway) 对托管设置加载返回了 HTTP 403
  * `"version_below_minimum"`：此版本的 Claude Code 低于 [`requiredMinimumVersion`](https://code.claude.com/docs/zh-CN/settings-reference#requiredminimumversion) 或高于 [`requiredMaximumVersion`](https://code.claude.com/docs/zh-CN/settings-reference#requiredmaximumversion)
  * `"_OTHER"`：Claude apps gateway 托管设置加载因其他原因失败
* `managed_settings.sources`：提供至少一个[策略键](https://code.claude.com/docs/zh-CN/managed-settings.md#how-claude-code-combines-managed-sources)的每个托管来源，按优先级从高到低排列，包括在 `first-wins` 下其键不生效的来源。取值为：`"remote"`；`"plist"` 或 `"hklm"` 表示 MDM 或操作系统级策略；`"file"` 表示托管设置文件和附加文件；`"parent"` 表示由[嵌入宿主](https://code.claude.com/docs/zh-CN/managed-settings.md#let-an-embedding-host-add-policy)提供设置；`"hkcu"` 表示 Claude Code [读取](https://code.claude.com/docs/zh-CN/managed-settings.md#how-claude-code-combines-managed-sources)的 [Windows HKCU 注册表值](https://code.claude.com/docs/zh-CN/managed-settings.md#where-each-mechanism-stores-the-policy)。仅包含控制键的来源或 Claude Code 无法读取的来源不会列出。以字符串数组形式发出，没有托管来源提供策略键时为空
* `managed_settings.source_behavior`：Claude Code 读取到的 [`managedSourcesBehavior`](https://code.claude.com/docs/zh-CN/settings-reference#managedsourcesbehavior) 值，为 `"first-wins"` 或 `"merge"`。没有来源设置该键时为 `"first-wins"`
* `managed_settings.helper.state`：所选 MDM 或文件来源所配置的策略辅助程序的状态：
  * `"ok"`：辅助程序的输出用作托管设置
  * `"bad_path"`、`"not_a_file"`、`"exit_nonzero"`、`"timed_out"`、`"oversize"`、`"parse_failed"`、`"envelope_invalid"` 或 `"schema_rejected"`：辅助程序上次运行失败。[辅助程序故障](https://code.claude.com/docs/zh-CN/settings-reference.md#helper-failures)介绍了这些情况
  * `"none"`：未配置辅助程序，或配置辅助程序的来源不是 MDM 策略或托管设置文件
* `managed_settings.helper.applied`：辅助程序自身的输出用作托管设置时为 `"output"`，否则为 `"none"`
* `managed_settings.helper.entry`：Claude Code 选择了 [`policyHelper`](https://code.claude.com/docs/zh-CN/settings-reference#policyhelper) 时为 `"policyHelper"`。未选择辅助程序时不存在
* `managed_settings.helper.path`：辅助程序配置的 [`path`](https://code.claude.com/docs/zh-CN/settings-reference#policyhelper-path)。只要 Claude Code 选择了辅助程序即存在，无论是否设置了 `OTEL_LOG_MANAGED_SETTINGS`
* `managed_settings.resolved_sha256`（当 `OTEL_LOG_MANAGED_SETTINGS=1` 时）：脱敏前已解析托管设置的 SHA-256，序列化为键按递归排序且不含空白的 JSON。摘要相同的机器运行相同的策略。Claude Code 仅在选择启用时发送摘要，因为较短的策略可以通过对猜测值进行哈希来还原。没有解析出托管设置时不存在，在 `refused` 事件中也不存在
* `managed_settings.settings`（当 `OTEL_LOG_MANAGED_SETTINGS=1` 时）：以 JSON 字符串形式表示的已解析托管设置的名称和结构，其中的值已脱敏。在 `refused` 事件中不存在。Claude Code 根据其设置 schema 构建该值：

  * schema 声明的设置名称会被导出，schema 未声明的键会被省略
  * 布尔值、数字以及 schema 限定为固定选项集的字符串值（例如 `permissions.defaultMode`）按原样导出。`sandbox.network.httpProxyPort` 和 `sandbox.network.socksProxyPort` 导出为 `"[REDACTED]"`
  * 其他所有字符串，例如 `model`、`apiKeyHelper`、每个 `env` 值、每个 URL 以及每个命令，均导出为 `"[REDACTED]"`
  * 映射的条目名称（例如 `env` 变量名和插件 ID）按原样导出。schema 未对其条目定义类型的设置（例如 `vimInsertModeRemaps`）导出为单个 `"[REDACTED]"`，`sandbox.ignoreViolations` 导出为其路径列表的列表，不含命令模式
  * 列表保留其长度，每个条目按相同规则脱敏
  * 当工具内置于此版本的 Claude Code 中，或是 `mcp__` 引用（例如 `mcp__jira__create_issue`）时，`permissions.allow`、`permissions.deny` 或 `permissions.ask` 规则导出为其工具名称，内容脱敏，例如 `Read([REDACTED])`。其他任何规则导出为 `"[REDACTED]"`
  * hook 遵循相同的规则，因此固定选项字段和数值字段（例如 `type` 和 `timeout`）会显示，而每个命令、URL、`matcher` 和 `if` 条件均导出为 `"[REDACTED]"`

  例如，包含 `apiKeyHelper`、两个 `env` 变量和一条拒绝规则的托管设置导出为 `{"apiKeyHelper":"[REDACTED]","env":{"HTTPS_PROXY":"[REDACTED]","CLAUDE_CODE_ENABLE_TELEMETRY":"[REDACTED]"},"permissions":{"deny":["Read([REDACTED])"]}}`。

  Claude Code 会在 UTF-8 编码 8 KB 处截断该值，截断后的值不是有效的 JSON
* `managed_settings.settings_truncated`（当存在 `managed_settings.settings` 时）：Claude Code 在 8 KB 处截断了 `managed_settings.settings` 时为 `true`，否则为 `false`。以布尔值而非字符串形式发出

## 解释指标和事件数据

导出的指标和事件支持一系列分析：

### 使用情况监控

| 指标 | 分析机会 |
| - | - |
| `claude_code.token.usage` | 按 token [`type`](#token-counter)、用户、团队、模型、`skill.name`、`plugin.name` 或 `agent.name` 分解 |
| `claude_code.session.count` | 跟踪随时间推移的采用和参与度 |
| `claude_code.lines_of_code.count` | 通过跟踪代码添加和删除来衡量生产力，按模型分解 |
| `claude_code.commit.count` & `claude_code.pull_request.count` | 了解对开发工作流的影响 |

### 成本监控

`claude_code.cost.usage` 指标有助于：

* 跟踪团队或个人的使用趋势
* 识别高使用会话以进行优化
* 通过 `skill.name`、`plugin.name` 和 `agent.name` 属性将支出归属于特定技能、插件或子代理类型

> [!NOTE]
>
> 成本指标是近似值。有关官方计费数据，请参阅您的 API 提供商（Claude 控制台、Amazon Bedrock 或 Google Cloud 的 Agent Platform）。

Claude Code 将每个流式响应计入成本和 token 指标，恰好一次，包括当网关或代理在 `ANTHROPIC_BASE_URL` 后面跨多个帧逐步流式传输使用情况时。

### 警报和分段

要考虑的常见警报：

* 成本激增
* 异常的令牌消耗
* 来自特定用户的高会话量

所有指标都可以按[标准属性](#standard-attributes)进行分段。`model` 属性在 `claude_code.token.usage`、`claude_code.cost.usage` 和 `claude_code.lines_of_code.count` 上可用。

按模型的提交分解只能通过在 `session.id` 上与令牌或成本指标进行联接来近似，因为一个会话可以跨越多个模型。筛选令牌或成本端的行，使 `query_source` 为 `"main"`，以便辅助和子代理请求不会将会话的提交归属于未进行这些提交的模型。

### 检测重试耗尽

Claude Code 在内部重试失败的 API 请求，仅在放弃后才发出单个 `claude_code.api_error` 事件，因此事件本身是该请求的终端信号。中间重试尝试不会作为单独的事件记录。

事件上的 `attempt` 属性记录尝试次数。`CLAUDE_CODE_MAX_RETRIES` 默认为 10，上限为 15；如果设置了 [`CLAUDE_CODE_RETRY_WATCHDOG`](https://code.claude.com/docs/zh-CN/env-vars)，默认值会更高，并且没有上限。

当请求在瞬时错误上耗尽所有重试时，`attempt` 最多等于该有效限制加一：默认为 11。

较低的值仍可能表示重试已耗尽：每次 Claude Code 在流式传输失败后重新发出请求时，`attempt` 都会从 `1` 重新开始计数。

要区分从一个恢复的会话与停滞的会话，按 `session.id` 分组事件，并检查错误后是否存在更晚的 `api_request` 事件。

### 事件分析

事件数据提供了对 Claude Code 交互的详细见解：

**工具使用模式**：分析工具结果事件以识别：

* 最常用的工具
* 工具成功率
* 平均工具执行时间
* 按工具类型的错误模式

**性能监控**：跟踪 API 请求持续时间和工具执行时间以识别性能瓶颈。

### 将输入 token 映射到 OpenTelemetry GenAI 语义约定

Claude Code 按照 API 响应的 usage 块中的数值导出输入 token 计数，因此这些值不包括从[提示缓存](https://code.claude.com/docs/zh-CN/prompt-caching.md)读取或写入的 token：

* [`claude_code.llm_request`](#span-attributes) span 和 [`api_request`](#api-request-event) 事件上的 `input_tokens`
* [`claude_code.token.usage`](#token-counter) 指标的 `"input"` 类型

Claude Code 不设置 `gen_ai.usage.*` 属性。[OpenTelemetry GenAI 语义约定](https://github.com/open-telemetry/semantic-conventions-genai)规定 `gen_ai.usage.input_tokens` 应包括从缓存读取和写入缓存的 token。要计算该总数：

* 从 span 或事件：将 `input_tokens`、`cache_read_tokens` 和 `cache_creation_tokens` 相加
* 从 `claude_code.token.usage` 指标：将其 `"input"`、`"cacheRead"` 和 `"cacheCreation"` 类型相加

这些约定还为缓存读取和缓存写入定义了单独的属性：

* `cache_read_tokens` 映射到 `gen_ai.usage.cache_read.input_tokens`
* `cache_creation_tokens` 映射到 `gen_ai.usage.cache_write.input_tokens`。旧版本的约定将缓存写入属性命名为 `gen_ai.usage.cache_creation.input_tokens`，因此请使用您的后端所期望的名称。

## 审计安全事件

OpenTelemetry 事件是 Claude Code 活动的审计数据源。每个事件都携带身份属性，将工具调用、MCP 活动和权限决策与触发它们的用户联系起来。OTLP 日志导出器可以将这些事件传递到任何具有 OTLP 接收器的安全信息和事件管理 (SIEM) 平台，或转发到您的 SIEM 的 OpenTelemetry Collector。

### 将操作归属到用户

每个事件上的 [标准属性](#standard-attributes) 包括已认证用户的身份：`user.email`、`user.account_uuid`、`user.account_id` 和 `organization.id`（使用 Claude 账户登录时或在 [云端会话](https://code.claude.com/docs/zh-CN/claude-code-on-the-web.md) 中，当会话自己的凭据携带它们时），加上 `user.id` 和每会话的 `session.id`。`user.id` 是安装范围的标识符，除了在 [Claude apps gateway](https://code.claude.com/docs/zh-CN/claude-apps-gateway) 会话上通过 `/login` 登录时，其中它是来自网关颁发的令牌的 IdP 主体。

在开发人员启动的会话中，MCP 工具调用、Bash 命令和文件编辑因此归属于该开发人员。Claude Code 不在单独的服务账户下运行；每个事件上记录的身份是开发人员自己的 Claude 账户，或开发人员在 [Claude apps gateway](https://code.claude.com/docs/zh-CN/claude-apps-gateway) 会话上的 IdP 身份。在 Claude Tag 频道会话中，Claude 改为作为您组织的 [共享身份](https://code.claude.com/docs/zh-CN/cloud-environments.md#set-the-environment-a-claude-tag-channel-uses) 工作。

当 Claude Code 使用直接 API 密钥进行身份验证，或针对 Amazon Bedrock、Google Cloud 的 Agent Platform 或 Microsoft Foundry 进行身份验证时，会话中没有 Claude 账户，仅填充 `user.id` 和 `session.id`。在这些部署中，使用 `OTEL_RESOURCE_ATTRIBUTES` 自己附加用户身份，通过 [托管设置](#administrator-configuration) 文件或启动包装器按用户设置。Claude apps gateway 会话不需要任何这些：请参阅 [标准属性](#standard-attributes) 了解其导出携带的身份。

```bash theme={null}
export OTEL_RESOURCE_ATTRIBUTES="enduser.id=jdoe@example.com,enduser.directory_id=S-1-5-21-..."
```

### 审计 MCP 活动

要使用完整的调用详情捕获 MCP 服务器活动，启用日志导出器并设置 `OTEL_LOG_TOOL_DETAILS=1`。每个 MCP 操作然后产生结构化事件，携带服务器名称、工具名称和调用参数以及标准身份属性：

| 事件 | 它为 MCP 记录的内容 |
| - | - |
| `mcp_server_connection` | 服务器连接、断开连接和连接失败，带有 `server_name`、`transport_type`、`server_scope` 和错误详情 |
| `tool_result` | 每个 MCP 工具调用，带有 `tool_name` 和 `mcp_server_scope`，包含 `mcp_server_name` 和 `mcp_tool_name` 的 `tool_parameters` 负载，以及包含调用参数的 `tool_input` 负载 |
| `tool_decision` | 调用是否被允许或拒绝，以及决策是来自配置、hook 还是用户，以及包含 `mcp_server_name` 和 `mcp_tool_name` 的 `tool_parameters` 负载 |

没有 `OTEL_LOG_TOOL_DETAILS`，这些事件会丢弃识别详情：

* `tool_result`：保留 `mcp_server_scope` 和一个对用户配置的服务器编辑为字面值 `"mcp_tool"` 的 `tool_name`，省略参数内容。对于 Claude Desktop 的内置服务器，在 Claude Desktop 拥有的会话中，它还保留 `tool_parameters` 内的 `mcp_server_name`/`mcp_tool_name` 对，与 `tool_decision` 相同的主机编写异常
* `tool_decision`：保留 `tool_source` 和一个对用户配置的服务器编辑为字面值 `"mcp_tool"` 的 `tool_name`，省略参数内容。对于 Claude Desktop 的内置服务器，在 Claude Desktop 拥有的会话中，它还保留 `tool_parameters` 内的 `mcp_server_name`/`mcp_tool_name` 对
* `mcp_server_connection`：省略 `server_name` 和错误消息，但保留 `is_plugin`、`plugin_id_hash` 和 `plugin.name`，非 Anthropic 插件名称被编辑为字面值 `"third-party"`，因此插件提供的服务器在没有详细日志的情况下仍然可以区分

### 将安全问题映射到事件

构建检测规则时，查找您想要监控的信号并查询您的后端以获取相应的事件和属性：

| 信号 | 事件 | 关键属性 |
| - | - | - |
| 工具调用被允许或拒绝，以及决策的来源，例如配置、hook 或用户 | [`tool_decision`](#tool-decision-event) | `decision`、`source`、`classifier_verdict`、`permission_mode`、`tool_name`；设置 `OTEL_LOG_TOOL_DETAILS=1` 时还有 `tool_parameters` |
| 哪项检查拒绝了工具调用或要求对其进行权限提示 | [`safety_event`](#safety-event)，其中 `category` 指明该检查，例如 `"managed_policy"`、`"permission_rule"`、`"hook"` 或 `"auto_mode_block"`。对于未运行的调用，`decision` 为 `"reject"`。某些调用[没有安全事件](#calls-without-a-safety-event) | `category`、`decision`、`reason_code`、`rule_source`、`tool_name`、`tool_use_id` |
| 自动模式分类器允许或阻止了工具调用，或 Claude Code 在没有分类器判定的情况下拒绝了调用 | 存在 `classifier_verdict` 的 [`tool_decision`](#tool-decision-event)。对于没有判定的拒绝，`classifier_verdict` 为 `"no_verdict"` 或 `"unavailable"` | `classifier_verdict`、`permission_mode`、`tool_name`；设置 `OTEL_LOG_TOOL_DETAILS=1` 时还有 `classifier_category` |
| 权限模式升级 | `permission_mode_changed` | `from_mode`、`to_mode`、`trigger` |
| 每个工具调用在哪种权限模式下接受检查，包括子代理发起的调用 | [`tool_decision`](#tool-decision-event) | `permission_mode`、`session.id` |
| 策略 hook 阻止了操作 | `hook_execution_complete` | `hook_event`、`num_blocking` |
| hook 拒绝了哪个工具调用，以及 hook 阻止了哪个结果或轮次结束 | [`safety_event`](#safety-event)，对于被拒绝的调用，其 `category` 为 `"hook"` 且 `decision` 为 `"reject"`；对于被阻止的结果或轮次结束，其 `category` 为 `"hook_review"` | `category`、`decision`、`hook_event`；工具调用还有 `tool_use_id` |
| 沙箱的网络代理拒绝了来自 Bash 或 PowerShell 命令的连接或请求 | 存在 `sandbox_violation_count` 且其值不为 `"0"` 的 [`safety_event`](#safety-event) | `sandbox_violation_count`、`category`、`sandbox_enabled`、`tool_use_id` |
| 登录、注销和身份验证失败 | `auth` | `action`、`success`、`error_category` |
| MCP 服务器连接或失败 | `mcp_server_connection` | `status`、`server_name`、`is_plugin`、`error_code` |
| 插件已安装及其来源 | `plugin_installed` | `plugin.name`、`marketplace.name`、`marketplace.is_official` |
| 运行的命令和触及的文件 | `tool_result`（已执行）或 `tool_decision`（已拒绝），带有 `OTEL_LOG_TOOL_DETAILS=1` | `tool_parameters`；`tool_input`（仅 `tool_result`） |
| 机器使用哪些托管设置源、其策略助手是否健康，以及机器拒绝启动的原因 | `managed_settings_resolved` | `managed_settings.trigger`、`managed_settings.sources`、`managed_settings.source_behavior`、`managed_settings.helper.state`、`error.type`；`managed_settings.settings` 和 `managed_settings.resolved_sha256`，带有 `OTEL_LOG_MANAGED_SETTINGS=1` |

Claude Code 仅发出原始事件流。异常检测、基线化、跨会话关联和警报是您的 SIEM 或可观测性后端的责任。

### 将出站路径映射到托管控制项和事件

下表将可能把会话内容带出机器的路径以及本地保留，与限制它们的 [托管设置](https://code.claude.com/docs/zh-CN/managed-settings.md) 键和记录它们的事件对应起来。关于 Claude Code 本身发送给 Anthropic 的内容（例如 `/feedback` 报告），请参阅 [数据使用](https://code.claude.com/docs/zh-CN/data-usage.md)。名称链接到各自的参考条目，其中给出了取值和默认值。

| 路径 | 托管控制项 | 事件 |
| - | - | - |
| Bash 和 PowerShell 命令 | [`sandbox.enabled`](https://code.claude.com/docs/zh-CN/settings-reference#sandbox-enabled)、[`sandbox.failIfUnavailable`](https://code.claude.com/docs/zh-CN/settings-reference#sandbox-failifunavailable)、[`sandbox.allowUnsandboxedCommands`](https://code.claude.com/docs/zh-CN/settings-reference#sandbox-allowunsandboxedcommands)、[`sandbox.network.allowManagedDomainsOnly`](https://code.claude.com/docs/zh-CN/settings-reference#sandbox-network-allowmanageddomainsonly)、[`sandbox.network.allowedDomains`](https://code.claude.com/docs/zh-CN/settings-reference#sandbox-network-alloweddomains) | [`tool_decision`](#tool-decision-event)、[`tool_result`](#tool-result-event) |
| MCP 服务器 | [`allowedMcpServers`](https://code.claude.com/docs/zh-CN/settings-reference#allowedmcpservers)、[`allowManagedMcpServersOnly`](https://code.claude.com/docs/zh-CN/settings-reference#allowmanagedmcpserversonly)、[`deniedMcpServers`](https://code.claude.com/docs/zh-CN/settings-reference#deniedmcpservers)、[`managed-mcp.json`](https://code.claude.com/docs/zh-CN/managed-mcp) | [`mcp_server_connection`](#mcp-server-connection-event)、`tool_decision`、`tool_result` |
| Hook | [`allowManagedHooksOnly`](https://code.claude.com/docs/zh-CN/settings-reference#allowmanagedhooksonly)、[`allowedHttpHookUrls`](https://code.claude.com/docs/zh-CN/settings-reference#allowedhttphookurls) | [`hook_registered`](#hook-registered-event)、[`hook_execution_start`](#hook-execution-start-event)、[`hook_execution_complete`](#hook-execution-complete-event) |
| 插件 | [`strictKnownMarketplaces`](https://code.claude.com/docs/zh-CN/settings-reference#strictknownmarketplaces)、[`disableSideloadFlags`](https://code.claude.com/docs/zh-CN/settings-reference#disablesideloadflags)、[`syncClaudeAiPlugins`](https://code.claude.com/docs/zh-CN/settings-reference#syncclaudeaiplugins)、[`syncClaudeAiSkills`](https://code.claude.com/docs/zh-CN/settings-reference#syncclaudeaiskills) | [`plugin_installed`](#plugin-installed-event)、[`plugin_loaded`](#plugin-loaded-event) |
| [WebFetch](https://code.claude.com/docs/zh-CN/permissions.md#webfetch) | [`permissions.deny`](https://code.claude.com/docs/zh-CN/settings-reference#permissions-deny)、[`allowManagedPermissionRulesOnly`](https://code.claude.com/docs/zh-CN/settings-reference#allowmanagedpermissionrulesonly) | `tool_decision`、`tool_result` |
| 上传到 claude.ai 的工具，例如 Artifact | `permissions.deny`、[`enableArtifact`](https://code.claude.com/docs/zh-CN/settings-reference#enableartifact) | `tool_decision`、`tool_result` |
| Remote Control | [`disableRemoteControl`](https://code.claude.com/docs/zh-CN/settings-reference#disableremotecontrol) | 无专用事件 |
| 本地会话记录保留 | [`cleanupPeriodDays`](https://code.claude.com/docs/zh-CN/settings-reference#cleanupperioddays) | [`retention_sweep`](#retention-sweep-event) |

`allowedHttpHookUrls`、`managed-mcp.json` 和 hook 事件这几项需要的信息超出了表中所示：

* **`allowedHttpHookUrls`**：条目会在各设置文件之间合并，因此开发人员可以向空的托管列表中添加条目。由 `allowManagedHooksOnly` 决定哪些 hook 运行
* **`managed-mcp.json`**：要关闭 MCP，请参阅 [完全禁用 MCP](https://code.claude.com/docs/zh-CN/managed-mcp.md#disable-mcp-entirely)。要确认 Claude Code 读取了该文件，请参阅 [验证配置](https://code.claude.com/docs/zh-CN/managed-mcp.md#validate-the-configuration)
* **Hook 事件**：Claude Code 对每个 hook 事件记录一次 `hook_execution_start` 和 `hook_execution_complete`，涵盖所有匹配的 hook。仅靠 `OTEL_LOG_TOOL_DETAILS=1` 不会记录 HTTP hook 的 URL。hook 配置只出现在 `hook_definitions` 中，而这还需要启用详细的 beta 追踪

`OTEL_LOG_TOOL_DETAILS=1` 会向这些事件添加命令字符串、服务器和工具名称以及工具输入。这些详情可能包含与会话本身相同的敏感内容，因此仅当您的收集器获准保存此类内容时才启用它。

### 检查保留清理

要为每台机器设置相同的保留期，请在 [托管设置](https://code.claude.com/docs/zh-CN/managed-settings.md) 中设置 [`cleanupPeriodDays`](https://code.claude.com/docs/zh-CN/settings-reference#cleanupperioddays)。要检查机器是否使用该值运行清理，请收集 [`retention_sweep`](#retention-sweep-event) 事件。`period_days` 和各计数器是字符串，因此在比较之前请先将它们转换为数字。

| 机器报告的内容 | 含义 |
| - | - |
| `result` 为 `"skipped"` | Claude Code 暂停了清理。`skip_reason` 给出原因 |
| `used_default` 为 `"true"`，或 `period_days` 与您的托管值不同 | 该机器未应用您托管的 `cleanupPeriodDays` |
| `error_count` 大于零 | 清理在列出或删除文件时遇到错误，因此超过保留期的数据可能仍然存在 |
| `files_past_cutoff` 大于零 | 清理未能删除超过保留期的文件，或发现了过期的已同步 skill 和插件文件夹。请结合 `error_count` 解读 |
| 没有事件 | 本身并不代表失败 |

按预期工作的机器也可能因以下原因没有事件：

* **没有人启动 Claude Code**：不会运行清理，机器会保留其数据直到下次启动
* **会话保持打开**：Claude Code 每个会话最多运行一次清理
* **会话提前结束**：会话退出时尚未完成的清理不会发出任何事件，因意外错误而停止的清理也不会

清理并不覆盖所有路径。[保留直到您删除](https://code.claude.com/docs/zh-CN/claude-directory.md#kept-until-you-delete-them) 列出了会保留的内容，[清除本地数据](https://code.claude.com/docs/zh-CN/claude-directory.md#clear-local-data) 说明了如何删除它们。

### 将事件发送到 SIEM

将 `OTEL_EXPORTER_OTLP_LOGS_ENDPOINT` 指向您的 SIEM 的 OTLP 接收器，或指向转发到您的 SIEM 的本机摄取 API 的 OpenTelemetry Collector。以下托管设置示例仅导出事件，启用了完整的工具详情用于 MCP 和 Bash 审计：

```json theme={null}
{
  "env": {
    "CLAUDE_CODE_ENABLE_TELEMETRY": "1",
    "OTEL_LOGS_EXPORTER": "otlp",
    "OTEL_LOG_TOOL_DETAILS": "1",
    "OTEL_EXPORTER_OTLP_LOGS_PROTOCOL": "http/protobuf",
    "OTEL_EXPORTER_OTLP_LOGS_ENDPOINT": "https://siem.example.com:4318/v1/logs",
    "OTEL_EXPORTER_OTLP_HEADERS": "Authorization=Bearer your-siem-token"
  }
}
```

要确认事件到达，在运行此配置的会话中提交提示词，并在您的 SIEM 中检查 `claude_code.user_prompt` 事件。如果没有任何内容到达，使用 `claude --debug-file <path>` 启动 Claude Code 并在该日志中检查 `[3P telemetry]` 导出错误。

## 后端考虑事项

您选择的指标、日志和跟踪后端决定了您可以执行的分析类型：

### 用于指标的后端

* **时间序列数据库**：速率计算、聚合指标
* **列式存储**：复杂查询、唯一用户分析
* **全功能可观测性平台**：高级查询、可视化、警报

### 用于事件和日志的后端

* **日志聚合系统**：全文搜索、日志分析
* **列式存储**：结构化事件分析
* **全功能可观测性平台**：指标和事件之间的关联

### 用于跟踪的后端

选择支持分布式跟踪存储和 span 关联的后端：

* **分布式跟踪系统**：Span 可视化、请求瀑布、延迟分析
* **全功能可观测性平台**：跟踪搜索和与指标和日志的关联

对于需要日活跃用户/周活跃用户/月活跃用户 (DAU/WAU/MAU) 指标的组织，请考虑支持高效唯一值查询的后端。

## 服务信息

所有指标和事件都使用以下资源属性导出：

* `service.name`：终端会话为 `claude-code`，从 [Claude Desktop 应用](https://code.claude.com/docs/zh-CN/desktop.md)中的代码选项卡启动的本地会话为 `claude-code-desktop`
* `service.version`：当前 Claude Code 版本，或本地代码选项卡会话的 Desktop 应用版本
* `os.type`：操作系统类型（例如，`linux`、`darwin`、`windows`）
* `os.version`：操作系统版本字符串
* `host.arch`：主机架构（例如，`amd64`、`arm64`）
* `wsl.version`：WSL 版本号（仅在 Windows Subsystem for Linux 上运行时出现）
* 仪表名称：`com.anthropic.claude_code`

如果您的收集器管道或仪表板在 `service.name = claude-code` 上进行过滤，请将 `claude-code-desktop` 添加到过滤器中，以便也捕获来自本地代码选项卡会话的遥测数据。

## ROI 测量资源

有关测量 Claude Code 投资回报率的综合指南，包括遥测设置、成本分析、生产力指标和自动化报告，请参阅 [Claude Code ROI 测量指南](https://github.com/anthropics/claude-code-monitoring-guide)。此存储库提供了现成的 Docker Compose 配置、Prometheus 和 OpenTelemetry 设置，以及用于生成与 Linear 等工具集成的生产力报告的模板。

## 安全和隐私

* OpenTelemetry 导出到您的后端是可选的，需要显式配置。有关 Anthropic 的单独操作遥测以及如何禁用它，请参阅 [数据使用](https://code.claude.com/docs/zh-CN/data-usage.md#telemetry-services)
* 原始文件内容和代码片段不包含在指标或事件中。Trace spans 是一个单独的数据路径：请参阅下面的 `OTEL_LOG_TOOL_CONTENT` 项目符号
* 通过 OAuth 认证时，`user.email` 包含在遥测属性中，仅发送到您配置的 OTel 端点，永远不会发送到 Anthropic。如果这对您的组织是一个问题，请与您的遥测后端合作以过滤或编辑此字段
* 默认情况下不收集用户提示词内容。仅记录提示词长度。要包含提示词内容，请设置 `OTEL_LOG_USER_PROMPTS=1`。启用后：
  * `user_prompt` 事件在两个属性中携带提示词文本：`prompt` 和 [`prompt_text`](#user-prompt-event)。如果您在收集器中按属性名称删除或屏蔽该事件的提示词文本，请在规则中同时指定这两个属性

    此 OpenTelemetry Collector `attributes` 处理器会在列出它的管道中删除这两个属性：

    ```yaml theme={null}
    processors:
      attributes/drop-prompt-text:
        actions:
          - key: prompt
            action: delete
          - key: prompt_text
            action: delete
    ```

  * 启用[追踪](#traces-beta)后，`claude_code.interaction` span 在其 `user_prompt` 属性中携带提示词文本

  * 在详细的 beta 追踪下，spans 还会携带每个请求发送的新用户消息、工具结果和系统提醒、系统提示词文本以及模型输出。[详细 beta 追踪下的内容属性](#new-context-gates)列出了每个属性。`claude_code.system_prompt` 事件携带完整的系统提示词
* 默认情况下不收集助手响应文本。仅记录响应长度。要包含响应文本，请设置 `OTEL_LOG_ASSISTANT_RESPONSES=1`。与来自 Claude Code 的所有 OpenTelemetry 数据一样，响应文本仅发送到您配置的 OTel 端点，永远不会发送到 Anthropic。当此变量未设置时，会回退到 `OTEL_LOG_USER_PROMPTS`，因此如果您希望事件中包含提示词内容而不包含响应内容，请设置 `OTEL_LOG_ASSISTANT_RESPONSES=0`。在详细的 beta 追踪下，`claude_code.llm_request` span 仍会在 [`response.model_output`](#new-context-gates) 中携带模型输出，该属性遵循 `OTEL_LOG_USER_PROMPTS` 而非此变量
* 默认情况下不记录工具输入参数和参数。要包含它们，请设置 `OTEL_LOG_TOOL_DETAILS=1`。对于 Claude Desktop 的内置服务器，在 Claude Desktop 拥有的会话中，`tool_decision` 和 `tool_result` 携带 `mcp_server_name`/`mcp_tool_name` 对，即主机编写的名称而非参数内容，即使关闭该标志也是如此。此数据仅发送到您配置的 OTEL 端点，永远不会发送到 Anthropic。参数仍可能包含敏感值，因此请根据需要配置您的遥测后端以过滤或编辑这些属性。启用后：
  * `tool_result` 和 `tool_decision` 事件包含 `tool_parameters` 属性，其中包含 Bash 命令、MCP 服务器和工具名称以及 skill 名称。`full_command` 等字段以未截断的形式发出
  * `tool_result` 事件另外包含 `tool_input` 属性，其中包含文件路径、URL、搜索模式和其他参数。超过 512 个字符的单个值被截断，总数限制为约 4 K 字符
  * `user_prompt` 事件包含自定义、插件和 MCP 命令的逐字 `command_name`
  * [成本和 token 计数器](#cost-counter)以及 `api_request`、`api_error` 和 `api_refusal` 事件在其归属属性中携带真实的 Agent、skill、插件和 MCP 服务器以及工具名称
  * `claude_code.tool` span 携带输入派生属性（如 `file_path`）。在详细的 beta 追踪下，它还携带 [`tool_input`](#new-context-gates) 属性
* 默认情况下，trace spans 中不记录工具内容。要包含它，请设置 `OTEL_LOG_TOOL_CONTENT=1`。`claude_code.tool` span 随后携带一个 [`tool.output` span 事件](#tool-output-span-event)，其中包含原始文件内容、Bash 命令输出以及 MCP 工具、WebFetch 和 WebSearch 返回的内容，在内容限制处截断（默认为 60 KB）每个属性。来自 MCP 工具、WebFetch 和 WebSearch 的结果需要 Claude Code v2.1.283 或更高版本。工具内容也通过 [`new_context` 到达 spans，其门控因 span 而异](#new-context-gates)。根据需要配置您的遥测后端以过滤或编辑这些属性
* 默认情况下不记录原始 Anthropic Messages API 请求和响应主体。要包含它们，请在您的 shell、用户设置或托管设置中设置 `OTEL_LOG_RAW_API_BODIES`。在 [项目和本地设置](https://code.claude.com/docs/zh-CN/settings-reference.md#variables-claude-code-ignores-in-env) 中被忽略。主体包含完整的对话历史，包括系统提示、每个先前的用户和助手轮次以及工具结果，因此启用此选项意味着同意其他 `OTEL_LOG_*` 内容标志会揭示的所有内容。Claude Code 始终从这些主体中编辑 Claude 的扩展思考内容，无论其他设置如何。您设置的值决定了 Claude Code 如何传递主体：
  * 使用 `=1` 时，Claude Code 为每个 API 调用发出 `api_request_body` 和 `api_response_body` 日志事件。事件的 `body` 属性携带 JSON 序列化的有效负载，在内容限制处截断（默认为 60 KB）
  * 使用 `=file:<dir>` 时，Claude Code 将未截断的主体写入该目录下的 `.request.json` 和 `.response.json` 文件，事件携带 `body_ref` 路径而不是内联主体。使用日志收集器或 sidecar 传输目录，而不是通过遥测流

    对于每个成功的响应，Claude Code 还会在该目录中的 `index.jsonl` 中追加一行，将响应文件链接到生成它的请求文件以及它成为的记录消息。每行不包含任何消息内容，[API 响应主体事件](#api-response-body-event)部分列出了其字段。索引文件需要 Claude Code v2.1.274 或更高版本

## 在 Amazon Bedrock 上监控 Claude Code

有关 Amazon Bedrock 的 Claude Code 使用情况监控指南的详细信息，请参阅 [Claude Code 监控实现 (Amazon Bedrock)](https://github.com/aws-solutions-library-samples/guidance-for-claude-code-with-amazon-bedrock/blob/main/assets/docs/MONITORING.md)。
