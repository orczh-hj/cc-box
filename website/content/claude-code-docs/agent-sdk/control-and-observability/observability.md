> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# 使用 OpenTelemetry 进行可观测性

> 使用 OpenTelemetry 将来自 Agent SDK 的跟踪、指标和事件导出到您的可观测性后端。

在生产环境中运行代理时，您需要了解它们的行为：

* 它们调用了哪些工具
* 每个模型请求花费了多长时间
* 花费了多少个令牌
* 失败发生在哪里

Agent SDK 可以将此数据作为 OpenTelemetry 跟踪、指标和日志事件导出到任何接受 OpenTelemetry 协议 (OTLP) 的后端，无论是托管的可观测性平台还是自托管收集器。

本指南说明了 SDK 如何发出遥测数据、如何配置导出，以及如何在数据到达后端后对其进行标记和过滤。要直接从 SDK 响应流读取令牌使用情况和成本，而不是导出到后端，请参阅[跟踪成本和使用情况](https://code.claude.com/docs/zh-CN/agent-sdk/cost-tracking.md)。

> [!NOTE]
>
> 有关 Claude API 或 Managed Agents，请参阅[适用于 Claude API 的 OpenTelemetry](https://platform.claude.com/docs/en/manage-claude/opentelemetry)。

## 遥测如何从 SDK 流动

Agent SDK 将 Claude Code CLI 作为子进程运行，并通过本地管道与其通信。CLI 内置了 OpenTelemetry 检测：它在每个模型请求和工具执行周围记录跨度，为令牌和成本计数器发出指标，并为提示和工具结果发出结构化日志事件。SDK 本身不产生遥测数据。相反，它将配置传递给 CLI 进程，CLI 直接导出到您的收集器。

配置作为环境变量传递。默认情况下，子进程继承您的应用程序的环境，因此您可以在以下两个位置之一配置遥测：

* **进程环境：** 在应用程序启动前在您的 shell、容器或编排器中设置变量。每个 `query()` 调用都会自动获取它们，无需代码更改。这是生产部署的推荐方法。
* **按调用选项：** 在 `ClaudeAgentOptions.env`（Python）或 `options.env`（TypeScript）中设置变量。当同一进程中的不同代理需要不同的遥测设置时，请使用此方法。在 Python 中，`env` 合并在继承的环境之上。在 TypeScript 中，`env` 完全替换继承的环境，因此在您传递的对象中包含 `...process.env`。

CLI 导出三个独立的 OpenTelemetry 信号。每个都有自己的启用开关和自己的导出器，因此您只能打开需要的信号。

| 信号 | 包含内容 | 启用方式 |
| - | - | - |
| Metrics | 令牌、成本、会话、代码行数和工具决策的计数器 | `OTEL_METRICS_EXPORTER` |
| Log events | 每个提示、API 请求、API 错误和工具结果的结构化记录 | `OTEL_LOGS_EXPORTER` |
| Traces | 每个交互、模型请求、工具调用和 hook 的跨度（测试版） | `OTEL_TRACES_EXPORTER` 加上 `CLAUDE_CODE_ENHANCED_TELEMETRY_BETA=1` |

有关完整的指标名称、事件名称和属性列表，请参阅 Claude Code [监控](https://code.claude.com/docs/zh-CN/monitoring-usage.md)参考。Agent SDK 发出相同的数据，因为它运行相同的 CLI。跨度名称列在下面的[读取代理跟踪](#read-agent-traces)中。

## 启用遥测导出

遥测默认关闭，直到您设置 `CLAUDE_CODE_ENABLE_TELEMETRY=1` 并选择至少一个导出器。最常见的配置是通过 OTLP HTTP 将所有三个信号发送到收集器。

以下示例在字典中设置变量并通过 `options.env` 传递它们。代理运行单个任务，CLI 将跨度、指标和事件导出到 `collector.example.com` 处的收集器，同时循环消费响应流：

```python Python theme={null}
import asyncio
from claude_agent_sdk import query, ClaudeAgentOptions

OTEL_ENV = {
    "CLAUDE_CODE_ENABLE_TELEMETRY": "1",
    # 跟踪需要此项，跟踪处于测试版。指标和日志事件不需要此项。
    "CLAUDE_CODE_ENHANCED_TELEMETRY_BETA": "1",
    # 为每个信号选择一个导出器。对于 SDK 使用 otlp；请参阅下面的注意。
    "OTEL_TRACES_EXPORTER": "otlp",
    "OTEL_METRICS_EXPORTER": "otlp",
    "OTEL_LOGS_EXPORTER": "otlp",
    # 标准 OTLP 传输配置。
    "OTEL_EXPORTER_OTLP_PROTOCOL": "http/protobuf",
    "OTEL_EXPORTER_OTLP_ENDPOINT": "http://collector.example.com:4318",
    "OTEL_EXPORTER_OTLP_HEADERS": "Authorization=Bearer your-token",
}


async def main():
    options = ClaudeAgentOptions(env=OTEL_ENV)
    async for message in query(
        prompt="List the files in this directory", options=options
    ):
        print(message)


asyncio.run(main())
```

```typescript TypeScript theme={null}
import { query } from "@anthropic-ai/claude-agent-sdk";

const otelEnv = {
  CLAUDE_CODE_ENABLE_TELEMETRY: "1",
  // 跟踪需要此项，跟踪处于测试版。指标和日志事件不需要此项。
  CLAUDE_CODE_ENHANCED_TELEMETRY_BETA: "1",
  // 为每个信号选择一个导出器。对于 SDK 使用 otlp；请参阅下面的注意。
  OTEL_TRACES_EXPORTER: "otlp",
  OTEL_METRICS_EXPORTER: "otlp",
  OTEL_LOGS_EXPORTER: "otlp",
  // 标准 OTLP 传输配置。
  OTEL_EXPORTER_OTLP_PROTOCOL: "http/protobuf",
  OTEL_EXPORTER_OTLP_ENDPOINT: "http://collector.example.com:4318",
  OTEL_EXPORTER_OTLP_HEADERS: "Authorization=Bearer your-token",
};

for await (const message of query({
  prompt: "List the files in this directory",
  // env 在 TypeScript 中替换继承的环境，因此首先展开
  // process.env 以保留 PATH、ANTHROPIC_API_KEY 和其他变量。
  options: { env: { ...process.env, ...otelEnv } },
})) {
  console.log(message);
}
```

因为子进程默认继承您的应用程序的环境，您可以通过在 Dockerfile、Kubernetes 清单或 shell 配置文件中导出这些变量并完全省略 `options.env` 来实现相同的结果。

要确认导出正在工作，请在任务完成后检查您的收集器日志中是否有传入的跨度、指标和日志事件。CLI 默认在导出错误时静默失败：如果端点无法访问或拒绝数据，代理仍然正常运行，CLI 会丢弃遥测而不会在您的应用程序中显示错误。要显示导出器错误，请在导出器变量旁边设置 [`CLAUDE_CODE_OTEL_DIAG_STDERR=1`](https://code.claude.com/docs/zh-CN/env-vars)，并通过 SDK 的 `stderr` 回调（Python）或 `stderr` 选项（TypeScript）读取诊断信息。需要 Claude Code v2.1.179 或更高版本。

> [!NOTE]
>
> `console` 导出器将遥测写入标准输出，SDK 将其用作其消息通道。在通过 SDK 运行时，不要将 `console` 设置为导出器值。要在本地检查遥测，请将 `OTEL_EXPORTER_OTLP_ENDPOINT` 指向本地 OpenTelemetry 收集器。

### 从短期调用刷新遥测

CLI 批处理遥测并按间隔导出。在干净的进程退出时，它尝试刷新待处理数据，但刷新受短超时限制，因此如果收集器响应缓慢，跨度仍可能被丢弃。如果您的进程在 CLI 关闭前被杀死，批处理缓冲区中的任何内容都会丢失。降低导出间隔会减少两个时间窗口。

默认情况下，指标每 60 秒导出一次，跟踪和日志每 5 秒导出一次。以下示例缩短了所有三个间隔，以便数据在短任务仍在运行时到达收集器：

```python Python theme={null}
OTEL_ENV = {
    # ... 来自前面示例的导出器配置 ...
    "OTEL_METRIC_EXPORT_INTERVAL": "1000",
    "OTEL_LOGS_EXPORT_INTERVAL": "1000",
    "OTEL_TRACES_EXPORT_INTERVAL": "1000",
}
```

```typescript TypeScript theme={null}
const otelEnv = {
  // ... 来自前面示例的导出器配置 ...
  OTEL_METRIC_EXPORT_INTERVAL: "1000",
  OTEL_LOGS_EXPORT_INTERVAL: "1000",
  OTEL_TRACES_EXPORT_INTERVAL: "1000",
};
```

## 读取代理跟踪

跟踪为您提供了代理运行的最详细视图。设置 `CLAUDE_CODE_ENHANCED_TELEMETRY_BETA=1` 后，代理循环的每一步都会成为您可以在跟踪后端中检查的跨度：

* **`claude_code.interaction`：** 包装代理循环的单个转折，从接收提示到生成响应。
* **`claude_code.llm_request`：** 包装对 Claude API 的每个调用，具有模型名称、延迟和令牌计数作为属性。
* **`claude_code.tool`：** 包装每个工具调用，具有权限等待的子跨度（`claude_code.tool.blocked_on_user`）和执行本身（`claude_code.tool.execution`）。
* **`claude_code.hook`：** 包装每个 [hook](https://code.claude.com/docs/zh-CN/agent-sdk/hooks.md) 执行。需要详细的测试版跟踪（`ENABLE_BETA_TRACING_DETAILED=1` 和 `BETA_TRACING_ENDPOINT`），这一对还会[改变您的日志和跟踪的去向](https://code.claude.com/docs/zh-CN/env-vars.md#variables)。

`llm_request`、`tool` 和 `hook` 跨度是封闭 `claude_code.interaction` 跨度的子级。当代理通过 Agent 工具生成子代理时，子代理的 `llm_request` 和 `tool` 跨度嵌套在父代理的 `claude_code.tool` 跨度下，因此完整的委派链显示为一个跟踪。

跨度默认携带 `session.id` 属性。当您对同一[会话](https://code.claude.com/docs/zh-CN/agent-sdk/sessions.md)进行多个 `query()` 调用时，在您的后端中按 `session.id` 过滤以将它们视为一个时间线。如果您将 `OTEL_METRICS_INCLUDE_SESSION_ID` 设置为假值，Claude Code 会省略该属性。

> [!NOTE]
>
> 跟踪处于测试版。跨度名称和属性可能在版本之间更改。有关跟踪导出器配置变量，请参阅监控参考中的[跟踪（测试版）](https://code.claude.com/docs/zh-CN/monitoring-usage.md#traces-beta)。

## 将跟踪链接到您的应用程序

SDK 自动将 W3C 跟踪上下文传播到 CLI 子进程。当您在应用程序中有活跃的 OpenTelemetry 跨度时调用 `query()`，SDK 将 `TRACEPARENT` 和 `TRACESTATE` 注入到子进程环境中，CLI 读取它们，使其 `claude_code.interaction` 跨度成为您的跨度的子级。代理运行随后出现在您的应用程序的跟踪中，而不是作为断开连接的根。

OTLP 事件日志记录在运行期间发出，携带相同的跟踪上下文：设置 `TRACEPARENT` 后，每条记录的 `trace_id` 和 `span_id` 与您的应用程序的跟踪相匹配，因此您可以在后端将[事件](https://code.claude.com/docs/zh-CN/monitoring-usage.md#events)与跨度关联。在 v2.1.212 之前，在活跃跨度外发出的事件记录不携带 `trace_id` 或 `span_id`。

启用跟踪上下文传播后，CLI 还将 `TRACEPARENT` 转发到它运行的每个 Bash 和 PowerShell 命令。如果通过 Bash 工具启动的命令发出自己的 OpenTelemetry 跨度，这些跨度会嵌套在包装该命令的 `claude_code.tool.execution` 跨度下。

当您在 `options.env` 中显式设置 `TRACEPARENT` 时，自动注入被跳过，因此您可以固定特定的父上下文（如果需要）。交互式 CLI 会话完全忽略入站 `TRACEPARENT`；只有 Agent SDK 和 `claude -p` 运行遵守它。有关完整的跨度和属性参考，请参阅监控参考中的[跟踪（测试版）](https://code.claude.com/docs/zh-CN/monitoring-usage.md#traces-beta)。

## 从您的代理标记遥测

默认情况下，CLI 将 `service.name` 报告为 `claude-code`。如果您运行多个代理，或将 SDK 与导出到同一收集器的其他服务一起运行，请覆盖服务名称并添加资源属性，以便您可以在后端中按代理过滤。

以下示例重命名服务并附加部署元数据。这些值作为 OpenTelemetry 资源属性应用于代理发出的每个跨度、指标和事件：

```python Python theme={null}
options = ClaudeAgentOptions(
    env={
        # ... 来自启用遥测导出示例的导出器配置 ...
        "OTEL_SERVICE_NAME": "support-triage-agent",
        "OTEL_RESOURCE_ATTRIBUTES": "service.version=1.4.0,deployment.environment=production",
    },
)
```

```typescript TypeScript theme={null}
const options = {
  env: {
    ...process.env,
    // ... 来自启用遥测导出示例的导出器配置 ...
    OTEL_SERVICE_NAME: "support-triage-agent",
    OTEL_RESOURCE_ATTRIBUTES:
      "service.version=1.4.0,deployment.environment=production",
  },
};
```

## 将属性操作归属于您的最终用户

CLI 根据它用来调用 Anthropic 的凭证将[身份属性](https://code.claude.com/docs/zh-CN/monitoring-usage.md#standard-attributes)附加到每个事件。当您构建一个从一个部署为许多最终用户服务的应用程序时，这些属性标识您的服务的凭证，而不是代理代表其行动的最终用户。

要使工具调用和 MCP 活动可归属于您的应用程序的最终用户，请在每个 `query()` 调用上注入最终用户身份作为资源属性。在插值前对值进行百分比编码，因为 `OTEL_RESOURCE_ATTRIBUTES` [保留逗号、空格和等号](https://code.claude.com/docs/zh-CN/monitoring-usage.md#multi-team-organization-support)。以下示例将请求用户和租户附加到来自一个请求的每个跨度和事件。它假设您的 Web 框架中有一个 `request` 对象，其中包含用户和租户 ID：

```python Python theme={null}
from urllib.parse import quote

options = ClaudeAgentOptions(
    env={
        # ... 导出器配置 ...
        "OTEL_RESOURCE_ATTRIBUTES": f"enduser.id={quote(request.user_id)},tenant.id={quote(request.tenant_id)}",
    },
)
```

```typescript TypeScript theme={null}
const options = {
  env: {
    ...process.env,
    // ... 导出器配置 ...
    OTEL_RESOURCE_ATTRIBUTES: `enduser.id=${encodeURIComponent(request.userId)},tenant.id=${encodeURIComponent(request.tenantId)}`,
  },
};
```

附加最终用户身份后，这些事件将成为每个用户的审计跟踪，您可以将其转发到安全信息和事件管理 (SIEM) 平台：

* `tool_decision` 和 `tool_result`：每个工具调用、该调用是被允许还是被拒绝，以及其结果
* `safety_event`：哪项检查拒绝了某个工具调用或要求为其显示权限提示，例如权限规则或自动模式分类器。某些调用[没有安全事件](https://code.claude.com/docs/zh-CN/monitoring-usage.md#calls-without-a-safety-event)。需要 Claude Code v2.1.307 或更高版本
* `mcp_server_connection`：MCP 服务器连接和连接失败
* `permission_mode_changed`：权限模式更改

这些事件导出为以 `claude_code.` 前缀命名的日志记录，例如 `claude_code.safety_event`。有关完整的安全相关事件列表和每个事件携带的属性，请参阅监控参考中的[审计安全事件](https://code.claude.com/docs/zh-CN/monitoring-usage.md#audit-security-events)。

## 控制导出中的敏感数据

遥测在结构上是默认的。持续时间、模型名称和工具名称记录在每个跨度上；令牌计数在底层 API 请求返回使用情况数据时记录，因此失败或中止请求的跨度可能会省略它们。您的代理读取和写入的内容默认不记录。这些选择加入变量将内容添加到导出的数据：

| 变量 | 添加 |
| - | - |
| `OTEL_LOG_USER_PROMPTS=1` | `claude_code.user_prompt` 事件和 `claude_code.interaction` 跨度上的提示文本 |
| `OTEL_LOG_TOOL_DETAILS=1` | `claude_code.tool_result` 事件上的工具输入参数，例如文件路径、shell 命令和搜索模式，以及[成本和 token 指标](https://code.claude.com/docs/zh-CN/monitoring-usage.md#cost-counter)上的真实 Agent、skill、插件和 MCP 服务器名称。自动模式分类器在阻止调用时指明的内置规则，以 [`classifier_category`](https://code.claude.com/docs/zh-CN/monitoring-usage#tool-decision-event) 的形式记录在 `claude_code.tool_decision` 和 `claude_code.safety_event` 事件上 |
| `OTEL_LOG_TOOL_CONTENT=1` | `claude_code.tool` 上的[`tool.output` 跨度事件](https://code.claude.com/docs/zh-CN/monitoring-usage#tool-output-span-event)，包含文件内容、Bash 输出以及 MCP 工具、WebFetch 和 WebSearch 返回的内容，默认在 60 KB 处截断，可通过 `CLAUDE_CODE_OTEL_CONTENT_MAX_LENGTH` 配置，需要 Claude Code v2.1.214 或更高版本。来自 MCP 工具、WebFetch 和 WebSearch 的结果需要 Claude Code v2.1.283 或更高版本。需要启用[跟踪](#read-agent-traces)。跨度属性在[其自己的门控](https://code.claude.com/docs/zh-CN/monitoring-usage.md#new-context-gates)下携带工具内容 |
| `OTEL_LOG_RAW_API_BODIES` | 完整的 Anthropic Messages API 请求和响应 JSON 作为 `claude_code.api_request_body` 和 `claude_code.api_response_body` 日志事件。设置为 `1` 表示在 60 KB 处截断的内联体，或 `file:<dir>` 表示磁盘上的未截断体，事件中有 `body_ref` 路径。`CLAUDE_CODE_OTEL_CONTENT_MAX_LENGTH` 配置内联截断限制，需要 Claude Code v2.1.214 或更高版本。体包括整个对话历史记录，扩展思考内容被编辑。启用此项意味着同意上述三个变量将揭示的所有内容 |

除非您的可观测性管道被批准存储您的代理处理的数据，否则请不要设置这些。有关完整的属性列表和编辑行为，请参阅监控参考中的[安全和隐私](https://code.claude.com/docs/zh-CN/monitoring-usage.md#security-and-privacy)。

## 相关文档

这些指南涵盖了监控和部署代理的相邻主题：

* [跟踪成本和使用情况](https://code.claude.com/docs/zh-CN/agent-sdk/cost-tracking.md)：从消息流读取令牌和成本数据，无需外部后端。
* [托管 Agent SDK](https://code.claude.com/docs/zh-CN/agent-sdk/hosting.md)：在容器中部署代理，您可以在环境级别设置 OpenTelemetry 变量。
* [监控](https://code.claude.com/docs/zh-CN/monitoring-usage.md)：CLI 发出的每个环境变量、指标和事件的完整参考。
