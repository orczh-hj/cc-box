> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# 为 Claude 提供自定义工具

> 使用 Claude Agent SDK 的进程内 MCP 服务器定义自定义工具，以便 Claude 可以调用您的函数、访问您的 API 并执行特定领域的操作。

自定义工具通过让您定义 Claude 在对话期间可以调用的自己的函数来扩展 Agent SDK。使用 SDK 的进程内 MCP 服务器，您可以让 Claude 访问数据库、外部 API、特定领域的逻辑或应用程序需要的任何其他功能。

## 快速参考

| 如果您想... | 执行此操作 |
| :- | :- |
| 定义工具 | 使用 [`@tool`](https://code.claude.com/docs/zh-CN/agent-sdk/python#tool)（Python）或 [`tool()`](https://code.claude.com/docs/zh-CN/agent-sdk/typescript#tool)（TypeScript），包含名称、描述、架构和处理程序。请参阅[创建自定义工具](#create-a-custom-tool)。 |
| 将参数设为可选 | 在 schema 中将其声明为可选，并在处理程序中应用默认值。请参阅[将参数设为可选](#make-a-parameter-optional)。 |
| 向 Claude 注册工具 | 在 `create_sdk_mcp_server` / `createSdkMcpServer` 中包装并传递给 `query()` 中的 `mcpServers`。请参阅[调用自定义工具](#call-a-custom-tool)。 |
| 预先批准工具 | 添加到您的允许工具列表。请参阅[配置允许的工具](#configure-allowed-tools)。 |
| 要求调用某个工具前需经批准，或阻止该工具 | 在 TypeScript SDK 中，向 `createSdkMcpServer()` 返回的配置对象添加针对该工具的 `permission_policy`。请参阅[要求特定工具需经批准](#require-approval-for-specific-tools)。 |
| 从 Claude 的上下文中删除内置工具 | 传递仅列出您想要的内置工具的 `tools` 数组。请参阅[配置允许的工具](#configure-allowed-tools)。 |
| 让 Claude 并行调用工具 | 在没有副作用的工具上设置 `readOnlyHint: true`。请参阅[添加工具注释](#add-tool-annotations)。 |
| 控制 Claude 读取的错误消息 | 返回 `isError: true` 来组合消息，而不是显示原始异常。请参阅[处理错误](#handle-errors)。 |
| 返回图像或文件 | 在内容数组中使用 `image` 或 `resource` 块。请参阅[返回图像和资源](#return-images-and-resources)。 |
| 返回机器可读的 JSON 结果 | 在结果上设置 `structuredContent`。请参阅[返回结构化数据](#return-structured-data)。 |
| 扩展到许多工具 | 使用[工具搜索](https://code.claude.com/docs/zh-CN/agent-sdk/tool-search.md)按需加载工具。 |

## 创建自定义工具

工具由四个部分定义，作为参数传递给 TypeScript 中的 [`tool()`](https://code.claude.com/docs/zh-CN/agent-sdk/typescript#tool) 辅助函数或 Python 中的 [`@tool`](https://code.claude.com/docs/zh-CN/agent-sdk/python#tool) 装饰器：

* **名称：** Claude 用来调用工具的唯一标识符。
* **描述：** 工具的功能。Claude 读取此内容以决定何时调用它。
* **输入 schema：** 工具接受的参数，按语言分别声明：
  * **TypeScript**：一个 [Zod schema](https://zod.dev/)。处理程序的 `args` 从中获得类型。对字段调用 `.describe()` 可为其添加 Claude 能看到的描述。
  * **Python**：一个将名称映射到类型的字典，如 `{"latitude": float}`，SDK 会为您将其转换为 JSON Schema。将类型包装在 `Annotated` 中，如 `{"latitude": Annotated[float, "Latitude coordinate"]}`，可为字段添加 Claude 能看到的描述。当您需要枚举、范围、可选字段或嵌套对象时，装饰器还可以直接接受完整的 [JSON Schema](https://json-schema.org/understanding-json-schema/about) 字典。
* **处理程序：** 当 Claude 调用工具时运行的异步函数。它接收验证的参数，必须返回一个包含以下内容的对象：
  * `content`（必需）：结果块数组，每个块的 `type` 为 `"text"`、`"image"`、`"audio"`、`"resource"` 或 `"resource_link"`。有关非文本块，请参阅[返回图像和资源](#return-images-and-resources)。
  * `structuredContent`（可选）：包含结果作为机器可读数据的 JSON 对象，与 `content` 一起返回。请参阅[返回结构化数据](#return-structured-data)。
  * `isError`（可选）：设置为 `true` 以表示工具失败，以便 Claude 可以对其做出反应。请参阅[处理错误](#handle-errors)。

定义工具后，使用 [`createSdkMcpServer`](https://code.claude.com/docs/zh-CN/agent-sdk/typescript#createsdkmcpserver)（TypeScript）或 [`create_sdk_mcp_server`](https://code.claude.com/docs/zh-CN/agent-sdk/python#create_sdk_mcp_server)（Python）将其包装在服务器中。服务器在应用程序内部进程中运行，而不是作为单独的进程运行。

本页中发出 HTTP 请求的 Python 示例使用 [httpx](https://www.python-httpx.org/)。请使用您项目所用的包管理器添加它：

> [tab] Python (uv)
>
> ```bash theme={null}
> uv add httpx
> ```

> [tab] Python (pip)
>
> ```bash theme={null}
> pip install httpx
> ```

### 天气工具示例

此示例定义了一个 `get_temperature` 工具并将其包装在 MCP 服务器中，但没有将服务器传递给 `query`。要运行该工具，请参阅下面的[调用自定义工具](#call-a-custom-tool)。

```python Python theme={null}
from typing import Annotated, Any
import httpx
from claude_agent_sdk import tool, create_sdk_mcp_server


# Define a tool: name, description, input schema, handler
@tool(
    "get_temperature",
    "Get the current temperature at a location",
    {
        "latitude": Annotated[float, "Latitude coordinate"],
        "longitude": Annotated[float, "Longitude coordinate"],
    },
)
async def get_temperature(args: dict[str, Any]) -> dict[str, Any]:
    async with httpx.AsyncClient() as client:
        response = await client.get(
            "https://api.open-meteo.com/v1/forecast",
            params={
                "latitude": args["latitude"],
                "longitude": args["longitude"],
                "current": "temperature_2m",
                "temperature_unit": "fahrenheit",
            },
        )
        data = response.json()

    # Return a content array - Claude sees this as the tool result
    return {
        "content": [
            {
                "type": "text",
                "text": f"Temperature: {data['current']['temperature_2m']}°F",
            }
        ]
    }


# Wrap the tool in an in-process MCP server
weather_server = create_sdk_mcp_server(
    name="weather",
    version="1.0.0",
    tools=[get_temperature],
)
```

```typescript TypeScript theme={null}
import { tool, createSdkMcpServer } from "@anthropic-ai/claude-agent-sdk";
import { z } from "zod";

// Define a tool: name, description, input schema, handler
const getTemperature = tool(
  "get_temperature",
  "Get the current temperature at a location",
  {
    latitude: z.number().describe("Latitude coordinate"), // .describe() adds a field description Claude sees
    longitude: z.number().describe("Longitude coordinate")
  },
  async (args) => {
    // args is typed from the schema: { latitude: number; longitude: number }
    const response = await fetch(
      `https://api.open-meteo.com/v1/forecast?latitude=${args.latitude}&longitude=${args.longitude}&current=temperature_2m&temperature_unit=fahrenheit`
    );
    const data: any = await response.json();

    // Return a content array - Claude sees this as the tool result
    return {
      content: [{ type: "text", text: `Temperature: ${data.current.temperature_2m}°F` }]
    };
  }
);

// Wrap the tool in an in-process MCP server
const weatherServer = createSdkMcpServer({
  name: "weather",
  version: "1.0.0",
  tools: [getTemperature]
});
```

有关完整的参数详细信息，包括 JSON Schema 输入格式和返回值结构，请参阅 [`tool()`](https://code.claude.com/docs/zh-CN/agent-sdk/typescript#tool) TypeScript 参考或 [`@tool`](https://code.claude.com/docs/zh-CN/agent-sdk/python#tool) Python 参考。

### 使参数可选

要使参数可选，请在 schema 中将其声明为可选，并在处理程序中应用默认值：

* **TypeScript**：向 Zod 字段添加 `.optional()`。
* **Python**：字典 schema 要求每个键都必须提供。请使用 JSON Schema 形式，将该参数从 `required` 中省略，并使用 `args.get()` 读取它。如需带有可选键的类型化 schema，请参阅 [TypedDict 类](https://code.claude.com/docs/zh-CN/agent-sdk/python.md#input-schema-options)。

下面的 [`get_precipitation_chance` 工具](#add-more-tools)展示了这两种写法。

### 调用自定义工具

通过 `mcpServers` 选项将您创建的 MCP 服务器传递给 `query`，并在 `allowedTools` 中列出每个工具的[完全限定名称](https://code.claude.com/docs/zh-CN/agent-sdk/mcp.md#tool-naming-convention)，以便工具运行而无需权限提示。该名称为 `mcp__{server_name}__{tool_name}`，其中 `{server_name}` 是您在 `mcpServers` 中为服务器指定的键。如果该键包含 `.` 或空格等字符，请在编写 `{server_name}` 时将其替换为 `_`。

这些代码片段重用了[天气工具示例](#weather-tool-example)中的 `weatherServer`，以询问 Claude 特定位置的天气。

```python Python theme={null}
import asyncio
from claude_agent_sdk import query, ClaudeAgentOptions, ResultMessage


async def main():
    options = ClaudeAgentOptions(
        mcp_servers={"weather": weather_server},
        allowed_tools=["mcp__weather__get_temperature"],
    )

    async for message in query(
        prompt="What's the temperature in San Francisco?",
        options=options,
    ):
        # ResultMessage is the final message after all tool calls complete
        if isinstance(message, ResultMessage) and message.subtype == "success":
            print(message.result)


asyncio.run(main())
```

```typescript TypeScript theme={null}
import { query } from "@anthropic-ai/claude-agent-sdk";

for await (const message of query({
  prompt: "What's the temperature in San Francisco?",
  options: {
    mcpServers: { weather: weatherServer },
    allowedTools: ["mcp__weather__get_temperature"]
  }
})) {
  // "result" is the final message after all tool calls complete
  if (message.type === "result" && message.subtype === "success") {
    console.log(message.result);
  }
}
```

将此代码片段与[天气工具示例](#weather-tool-example)中的工具和服务器定义合并到一个文件（`weather.py` 或 `weather.ts`）中，然后在终端中运行它：

> [tab] TypeScript
>
> ```bash theme={null}
> npx tsx weather.ts
> ```

> [tab] Python (uv)
>
> ```bash theme={null}
> uv run weather.py
> ```

> [tab] Python (pip)
>
> 激活您安装了 SDK 的虚拟环境，然后运行：
>
> ```bash theme={null}
> python weather.py
> ```

Claude 调用 `get_temperature`，脚本打印一行答案，显示旧金山的当前温度。

### 添加更多工具

服务器可以容纳您在其 `tools` 数组中列出的任意数量的工具。当服务器上有多个工具时，您可以在 `allowedTools` 中单独列出每个工具，或使用通配符 `mcp__weather__*` 来覆盖服务器公开的每个工具。

下面的示例定义了第二个工具 `get_precipitation_chance`，并用列出数组中两个工具的工具替换了[天气工具示例](#weather-tool-example)中的 `weatherServer` 定义。

```python Python theme={null}
# Define a second tool for the same server
@tool(
    "get_precipitation_chance",
    "Get the hourly precipitation probability for a location",
    {
        "type": "object",
        "properties": {
            "latitude": {"type": "number"},
            "longitude": {"type": "number"},
            "hours": {
                "type": "integer",
                "minimum": 1,
                "maximum": 24,
                "description": "How many hours of forecast to return",
            },
        },
        # 'hours' is left out of required, so Claude can omit it
        "required": ["latitude", "longitude"],
    },
)
async def get_precipitation_chance(args: dict[str, Any]) -> dict[str, Any]:
    # 'hours' isn't in required - read it with .get() to fall back to a default
    hours = args.get("hours", 12)
    async with httpx.AsyncClient() as client:
        response = await client.get(
            "https://api.open-meteo.com/v1/forecast",
            params={
                "latitude": args["latitude"],
                "longitude": args["longitude"],
                "hourly": "precipitation_probability",
                "forecast_days": 1,
            },
        )
        data = response.json()
    chances = data["hourly"]["precipitation_probability"][:hours]

    return {
        "content": [
            {
                "type": "text",
                "text": f"Next {hours} hours: {'%, '.join(map(str, chances))}%",
            }
        ]
    }


# Rebuild the server with both tools in the array
weather_server = create_sdk_mcp_server(
    name="weather",
    version="1.0.0",
    tools=[get_temperature, get_precipitation_chance],
)
```

```typescript TypeScript theme={null}
// Define a second tool for the same server
const getPrecipitationChance = tool(
  "get_precipitation_chance",
  "Get the hourly precipitation probability for a location",
  {
    latitude: z.number(),
    longitude: z.number(),
    hours: z
      .number()
      .int()
      .min(1)
      .max(24)
      .optional() // .optional() lets Claude omit the parameter
      .describe("How many hours of forecast to return")
  },
  async (args) => {
    const hours = args.hours ?? 12; // Apply the default in the handler
    const response = await fetch(
      `https://api.open-meteo.com/v1/forecast?latitude=${args.latitude}&longitude=${args.longitude}&hourly=precipitation_probability&forecast_days=1`
    );
    const data: any = await response.json();
    const chances = data.hourly.precipitation_probability.slice(0, hours);

    return {
      content: [{ type: "text", text: `Next ${hours} hours: ${chances.join("%, ")}%` }]
    };
  }
);

// Rebuild the server with both tools in the array
const weatherServer = createSdkMcpServer({
  name: "weather",
  version: "1.0.0",
  tools: [getTemperature, getPrecipitationChance]
});
```

[工具搜索](https://code.claude.com/docs/zh-CN/agent-sdk/tool-search.md)默认启用，并延迟 SDK MCP 工具：Claude 在紧凑列表中看到每个工具的名称，并按需加载其完整模式。禁用工具搜索后，此数组中的每个工具在每个回合都会消耗上下文窗口空间。在 TypeScript 中，在 [`tool()`](https://code.claude.com/docs/zh-CN/agent-sdk/typescript#tool) 的 `extras` 参数或 [`createSdkMcpServer()`](https://code.claude.com/docs/zh-CN/agent-sdk/typescript#createsdkmcpserver) 的选项中传递 `alwaysLoad: true`，以在初始提示中保持工具的完整模式。

### 添加工具注释

[工具注释](https://modelcontextprotocol.io/docs/concepts/tools#tool-annotations)是描述工具行为方式的可选元数据。在 TypeScript 中将它们作为 `tool()` 辅助函数的第五个参数传递，或在 Python 中通过 `@tool` 装饰器的 `annotations` 关键字参数传递。所有提示字段都是布尔值。

| 字段 | 默认值 | 含义 |
| :- | :- | :- |
| `readOnlyHint` | `false` | 工具不修改其环境。控制工具是否可以与其他只读工具并行调用。 |
| `destructiveHint` | `true` | 工具可能执行破坏性更新。对于您[标记为 `always_ask`](#require-approval-for-specific-tools) 的任何工具，请将其显式设置为 `true`。 |
| `idempotentHint` | `false` | 使用相同参数的重复调用没有额外效果。仅供参考。 |
| `openWorldHint` | `true` | 工具到达流程外的系统。仅供参考。 |

注释是元数据，不是强制执行。标记为 `readOnlyHint: true` 的工具如果处理程序这样做，仍然可以写入磁盘。保持注释与处理程序准确一致。

此示例向[天气工具示例](#weather-tool-example)中的 `get_temperature` 工具添加 `readOnlyHint`。

```python Python theme={null}
from claude_agent_sdk import tool, ToolAnnotations


@tool(
    "get_temperature",
    "Get the current temperature at a location",
    {"latitude": float, "longitude": float},
    annotations=ToolAnnotations(
        readOnlyHint=True
    ),  # Lets Claude batch this with other read-only calls
)
async def get_temperature(args):
    return {"content": [{"type": "text", "text": "..."}]}
```

```typescript TypeScript theme={null}
import { tool } from "@anthropic-ai/claude-agent-sdk";
import { z } from "zod";

tool(
  "get_temperature",
  "Get the current temperature at a location",
  { latitude: z.number(), longitude: z.number() },
  async (args) => ({ content: [{ type: "text", text: `...` }] }),
  { annotations: { readOnlyHint: true } } // Lets Claude batch this with other read-only calls
);
```

在 [TypeScript](https://code.claude.com/docs/zh-CN/agent-sdk/typescript.md#toolannotations) 或 [Python](https://code.claude.com/docs/zh-CN/agent-sdk/python.md#toolannotations) 参考中查看 `ToolAnnotations`。

## 控制工具访问

[天气工具示例](#weather-tool-example)注册了一个服务器，并在`allowedTools`中列出了工具。本节介绍当您有多个工具或想要限制内置工具时如何限制访问范围。有关工具名称的构造方式，请参阅[调用自定义工具](#call-a-custom-tool)。

### 配置允许的工具

`tools`选项和允许/禁止列表影响两个层级：可用性（控制工具是否出现在Claude的上下文中）和权限（控制Claude尝试调用后是否批准该调用）。`tools`和裸名称`disallowedTools`条目改变可用性。`allowedTools`和作用域`disallowedTools`规则改变权限。如果您在`allowedTools`中命名其中一个[任务跟踪工具](https://code.claude.com/docs/zh-CN/agent-sdk/todo-tracking.md#model-availability)，Claude Code也会为该会话启用这些工具。

| 选项 | 层级 | 效果 |
| :- | :- | :- |
| `tools: ["Read", "Grep"]` | 可用性 | 仅列出的内置工具在Claude的上下文中。未列出的内置工具被移除。MCP工具不受影响。 |
| `tools: []` | 可用性 | 所有内置工具都被移除。Claude只能使用您的MCP工具。 |
| 允许的工具 | 权限 | 列出的工具无需权限提示即可运行。其他未列出的工具保持可用；调用通过[权限流程](https://code.claude.com/docs/zh-CN/agent-sdk/permissions.md)进行。 |
| 禁止的工具 | 两者 | 裸工具名称如`"Bash"`从Claude的上下文中移除工具，与从`tools`中省略它的效果相同。作用域规则如`"Bash(rm *)"`将工具保留在上下文中，仅拒绝与[如所写](https://code.claude.com/docs/zh-CN/permissions.md#bash-rule-limits)匹配的调用。 |

要完全移除内置工具，请从`tools`中省略它或在`disallowedTools`中列出其裸名称（Python：`disallowed_tools`）；两者都将工具保留在上下文之外，以便Claude永远不会尝试它。作用域`disallowedTools`规则阻止匹配的调用但将工具保留为可见，因此Claude可能会浪费一个回合尝试它。有关完整的评估顺序，请参阅[配置权限](https://code.claude.com/docs/zh-CN/agent-sdk/permissions.md)。

### 要求特定工具需经批准

将工具的`permission_policy`设置为`always_ask`后，`allowedTools`将不再批准该工具的调用，因此即使该工具已被列出，其调用也可以到达您的[`canUseTool`](https://code.claude.com/docs/zh-CN/agent-sdk/user-input)回调。如果改为设置`always_deny`，则[对该工具的调用会被拒绝](https://code.claude.com/docs/zh-CN/agent-sdk/permissions.md#how-permissions-are-evaluated)，但您加载的插件中的[mod](https://code.claude.com/docs/zh-CN/plugins/mods/overview.md)[仍然可以批准它](https://code.claude.com/docs/zh-CN/permissions.md#precedence-among-rules-hooks-and-mods)。该字段仅适用于TypeScript，并且需要Agent SDK v0.3.301或更高版本。

请将策略条目放在`createSdkMcpServer()`返回的配置对象的`tools`数组中，而不是放在传入该函数的`tools`选项中。以下示例展开了[天气工具示例](#weather-tool-example)中的`weatherServer`，并添加了一个`tools`数组，为`get_temperature`指定`always_ask`策略：

```typescript TypeScript theme={null}
options: {
  mcpServers: {
    weather: {
      ...weatherServer,
      tools: [{ name: "get_temperature", permission_policy: "always_ask" as const }]
    }
  },
  allowedTools: ["mcp__weather__get_temperature"],
  permissionMode: "default",
  canUseTool: async (toolName, input) => {
    console.log(`Approval requested for ${toolName}`);
    return { behavior: "allow", updatedInput: input };
  }
}
```

要试用该策略，请将[调用自定义工具](#call-a-custom-tool)脚本中的`options`替换为本示例的`options`，然后在终端中运行该脚本。脚本会打印`Approval requested for mcp__weather__get_temperature`，随后输出Claude关于温度的回答。有关完整脚本，请参阅[TypeScript参考](https://code.claude.com/docs/zh-CN/agent-sdk/typescript.md#mcpsdkserverconfigwithinstance)。

如果您使用`auto`或`plan`[权限模式](https://code.claude.com/docs/zh-CN/agent-sdk/permissions.md#permission-modes)，请在工具的[注解](#add-tool-annotations)中设置`destructiveHint: true`，以便对该工具的调用仍能到达您的回调。否则，[自动模式分类器](https://code.claude.com/docs/zh-CN/permission-modes.md#eliminate-prompts-with-auto-mode)可能会在[TypeScript参考中列出的条件](https://code.claude.com/docs/zh-CN/agent-sdk/typescript.md#permission-modes-that-can-approve-an-always_ask-call)下批准某些`always_ask`调用，而被其批准的调用不会到达您的回调。请显式设置该提示，因为对于`always_ask`调用，未设置的`destructiveHint`会被视为`false`，而不是默认为`true`。

## 处理错误

处理程序错误不会停止代理循环。SDK 的进程内 MCP 服务器捕获未捕获的异常并将其作为错误结果返回，因此您报告错误的方式决定了 Claude 读取的内容，而不是查询是否失败：

| 发生的情况 | 结果 |
| :- | :- |
| 处理程序抛出未捕获的异常 | MCP 服务器将其转换为携带原始异常消息的错误结果。Claude 看到该消息，代理循环继续。 |
| 处理程序捕获错误并返回 `isError: true` (TS) / `"is_error": True` (Python) | Claude 看到您编写的消息。您可以添加原始异常缺少的上下文，例如哪个请求失败或应该尝试什么。 |

在这两种情况下，Claude 都可以重试、尝试不同的工具或解释失败。当原始异常消息不足以让 Claude 采取行动时，请自己捕获错误。

下面的示例在处理程序内捕获两种失败，并编写 Claude 读取的错误消息。非 200 HTTP 状态从响应中捕获并作为错误结果返回。网络错误或无效的 JSON 由周围的 `try/except` (Python) 或 `try/catch` (TypeScript) 捕获，也作为错误结果返回。在这两种情况下，Claude 都会收到描述失败的消息，而不是裸异常字符串。

```python Python theme={null}
import json
import httpx
from typing import Any
from claude_agent_sdk import tool


@tool(
    "fetch_data",
    "Fetch data from an API",
    {"endpoint": str},  # Simple schema
)
async def fetch_data(args: dict[str, Any]) -> dict[str, Any]:
    try:
        async with httpx.AsyncClient() as client:
            response = await client.get(args["endpoint"])
            if response.status_code != 200:
                # Return the failure as a tool result so Claude can react to it.
                # is_error marks this as a failed call rather than odd-looking data.
                return {
                    "content": [
                        {
                            "type": "text",
                            "text": f"API error: {response.status_code} {response.reason_phrase}",
                        }
                    ],
                    "is_error": True,
                }

            data = response.json()
            return {"content": [{"type": "text", "text": json.dumps(data, indent=2)}]}
    except Exception as e:
        # Composes the message Claude reads. An uncaught exception would
        # reach Claude as the raw str(e) with no context.
        return {
            "content": [{"type": "text", "text": f"Failed to fetch data: {str(e)}"}],
            "is_error": True,
        }
```

```typescript TypeScript theme={null}
import { tool } from "@anthropic-ai/claude-agent-sdk";
import { z } from "zod";

tool(
  "fetch_data",
  "Fetch data from an API",
  {
    endpoint: z.string().url().describe("API endpoint URL")
  },
  async (args) => {
    try {
      const response = await fetch(args.endpoint);

      if (!response.ok) {
        // Return the failure as a tool result so Claude can react to it.
        // isError marks this as a failed call rather than odd-looking data.
        return {
          content: [
            {
              type: "text",
              text: `API error: ${response.status} ${response.statusText}`
            }
          ],
          isError: true
        };
      }

      const data = await response.json();
      return {
        content: [
          {
            type: "text",
            text: JSON.stringify(data, null, 2)
          }
        ]
      };
    } catch (error) {
      // Composes the message Claude reads. An uncaught throw would
      // reach Claude as the raw error message with no context.
      return {
        content: [
          {
            type: "text",
            text: `Failed to fetch data: ${error instanceof Error ? error.message : String(error)}`
          }
        ],
        isError: true
      };
    }
  }
);
```

## 返回图像和资源

工具结果中的 `content` 数组接受 `text`、`image`、`audio`、`resource` 和 `resource_link` 块。您可以在同一响应中混合使用它们。在 TypeScript 中，SDK 将音频块保存到磁盘，Claude 接收包含已保存文件路径的文本块；在 Python 中，SDK 从工具结果中删除音频块并记录警告。

Claude 将每个资源链接块作为包含链接名称、URI 和描述的文本块接收。在 TypeScript 中，您的应用程序还会在用户消息的 `tool_use_result` 上以 [`resourceLinks`](https://code.claude.com/docs/zh-CN/agent-sdk/typescript#sdkmcpresourcelink) 的形式接收链接本身；在 Python 中，SDK 在 CLI 看到结果之前将它们展平为文本，因此 Python [`resourceLinks` 键](https://code.claude.com/docs/zh-CN/agent-sdk/python#usermessage)永远不会为进程内工具生成。

### 图像

图像块以 base64 编码的方式内联携带图像字节。没有 URL 字段。要返回位于 URL 的图像，请在处理程序中获取它，读取响应字节，并在返回之前对其进行 base64 编码。PNG、JPEG、GIF 或 WebP 图像作为视觉输入到达 Claude；任何其他类型的图像都被保存到磁盘，Claude 接收其文件路径作为文本。

| 字段 | 类型 | 说明 |
| :- | :- | :- |
| `type` | `"image"` | |
| `data` | `string` | Base64 编码的字节。仅原始 base64，不带 `data:image/...;base64,` 前缀 |
| `mimeType` | `string` | 必需。例如 `image/png`、`image/jpeg`、`image/webp`、`image/gif` |

```python Python theme={null}
import base64
import httpx
from claude_agent_sdk import tool


# Define a tool that fetches an image from a URL and returns it to Claude
@tool("fetch_image", "Fetch an image from a URL and return it to Claude", {"url": str})
async def fetch_image(args):
    async with httpx.AsyncClient() as client:  # Fetch the image bytes
        response = await client.get(args["url"])

    return {
        "content": [
            {
                "type": "image",
                "data": base64.b64encode(response.content).decode(
                    "ascii"
                ),  # Base64-encode the raw bytes
                "mimeType": response.headers.get(
                    "content-type", "image/png"
                ),  # Read MIME type from the response
            }
        ]
    }
```

```typescript TypeScript theme={null}
import { tool } from "@anthropic-ai/claude-agent-sdk";
import { z } from "zod";

tool(
  "fetch_image",
  "Fetch an image from a URL and return it to Claude",
  {
    url: z.string().url()
  },
  async (args) => {
    const response = await fetch(args.url); // Fetch the image bytes
    const buffer = Buffer.from(await response.arrayBuffer()); // Read into a Buffer for base64 encoding
    const mimeType = response.headers.get("content-type") ?? "image/png";

    return {
      content: [
        {
          type: "image",
          data: buffer.toString("base64"), // Base64-encode the raw bytes
          mimeType
        }
      ]
    };
  }
);
```

### 资源

资源块嵌入由 URI 标识的内容片段。实际内容位于块的 `text` 或 `blob` 字段中。当您的工具生成文件或来自外部系统的记录时，请使用此方法。

| 字段 | 类型 | 说明 |
| :- | :- | :- |
| `type` | `"resource"` | |
| `resource.uri` | `string` | 内容的标识符。任何 URI 方案 |
| `resource.text` | `string` | 内容（如果是文本）。提供此项或 `blob`，但不能同时提供两者 |
| `resource.blob` | `string` | 内容 base64 编码（如果是二进制）。仅 TypeScript：Python SDK 从工具结果中删除二进制资源并记录警告 |
| `resource.mimeType` | `string` | 可选 |

此示例显示从工具处理程序内部返回的资源块。SDK 不会从示例的 URI `file:///tmp/report.md` 读取。

```typescript TypeScript theme={null}
return {
  content: [
    {
      type: "resource",
      resource: {
        uri: "file:///tmp/report.md", // Label for Claude to reference, not a path the SDK reads
        mimeType: "text/markdown",
        text: "# Report\n..." // The actual content, inline
      }
    }
  ]
};
```

```python Python theme={null}
return {
    "content": [
        {
            "type": "resource",
            "resource": {
                "uri": "file:///tmp/report.md",  # Not a path the SDK reads
                "mimeType": "text/markdown",
                "text": "# Report\n...",  # The actual content, inline
            },
        }
    ]
}
```

这些块形状来自 MCP `CallToolResult` 类型。有关完整定义，请参阅 [MCP 规范](https://modelcontextprotocol.io/specification/2025-06-18/server/tools#tool-result)。

## 返回结构化数据

`structuredContent` 是结果上的可选 JSON 对象，与 `content` 数组分开。使用它来返回原始值，Claude 可以将其作为精确字段读取，而不是从文本字符串或图像中解析它们。

当设置 `structuredContent` 时，Claude 会接收 JSON 以及来自 `content` 的任何图像或资源块。`content` 中的文本块不会被转发，因为假设它们会复制结构化数据。下面的示例将图表呈现为图像块，并从同一处理程序的 `structuredContent` 中返回其后面的数据点。在代码片段中，`chartPngBuffer` 是一个包含呈现的 PNG 字节的 `Buffer`。

```typescript TypeScript theme={null}
return {
  content: [
    {
      type: "image",
      data: chartPngBuffer.toString("base64"),
      mimeType: "image/png"
    }
  ],
  structuredContent: {
    series: "temperature_2m",
    unit: "fahrenheit",
    points: [62.1, 63.4, 65.0, 64.2]
  }
};
```

> [!NOTE]
>
> Python `@tool` 装饰器仅从处理程序的返回字典中转发 `content` 和 `is_error`。要从 Python 返回 `structuredContent`，请运行[独立 MCP 服务器](https://code.claude.com/docs/zh-CN/agent-sdk/mcp.md)而不是进程内 SDK 服务器。

## 示例：单位转换器

此工具在长度、温度和重量单位之间转换值。用户可以询问"将100公里转换为英里"或"72°F是多少摄氏度"，Claude会从请求中选择正确的单位类型和单位。

它演示了两种模式：

* **枚举模式：** `unit_type` 被限制为一组固定值。在 TypeScript 中，使用 `z.enum()`。在 Python 中，字典模式不支持枚举，因此需要完整的 JSON Schema 字典。
* **不支持的输入处理：** 当找不到转换对时，处理程序返回 `isError: true`，以便 Claude 可以告诉用户出了什么问题，而不是将失败视为正常结果。

```python Python theme={null}
from typing import Any
from claude_agent_sdk import tool, create_sdk_mcp_server


# z.enum() in TypeScript becomes an "enum" constraint in JSON Schema.
# The dict schema has no equivalent, so full JSON Schema is required.
@tool(
    "convert_units",
    "Convert a value from one unit to another",
    {
        "type": "object",
        "properties": {
            "unit_type": {
                "type": "string",
                "enum": ["length", "temperature", "weight"],
                "description": "Category of unit",
            },
            "from_unit": {
                "type": "string",
                "description": "Unit to convert from, e.g. kilometers, fahrenheit, pounds",
            },
            "to_unit": {"type": "string", "description": "Unit to convert to"},
            "value": {"type": "number", "description": "Value to convert"},
        },
        "required": ["unit_type", "from_unit", "to_unit", "value"],
    },
)
async def convert_units(args: dict[str, Any]) -> dict[str, Any]:
    conversions = {
        "length": {
            "kilometers_to_miles": lambda v: v * 0.621371,
            "miles_to_kilometers": lambda v: v * 1.60934,
            "meters_to_feet": lambda v: v * 3.28084,
            "feet_to_meters": lambda v: v * 0.3048,
        },
        "temperature": {
            "celsius_to_fahrenheit": lambda v: (v * 9) / 5 + 32,
            "fahrenheit_to_celsius": lambda v: (v - 32) * 5 / 9,
            "celsius_to_kelvin": lambda v: v + 273.15,
            "kelvin_to_celsius": lambda v: v - 273.15,
        },
        "weight": {
            "kilograms_to_pounds": lambda v: v * 2.20462,
            "pounds_to_kilograms": lambda v: v * 0.453592,
            "grams_to_ounces": lambda v: v * 0.035274,
            "ounces_to_grams": lambda v: v * 28.3495,
        },
    }

    key = f"{args['from_unit']}_to_{args['to_unit']}"
    fn = conversions.get(args["unit_type"], {}).get(key)

    if not fn:
        return {
            "content": [
                {
                    "type": "text",
                    "text": f"Unsupported conversion: {args['from_unit']} to {args['to_unit']}",
                }
            ],
            "is_error": True,
        }

    result = fn(args["value"])
    return {
        "content": [
            {
                "type": "text",
                "text": f"{args['value']} {args['from_unit']} = {result:.4f} {args['to_unit']}",
            }
        ]
    }


converter_server = create_sdk_mcp_server(
    name="converter",
    version="1.0.0",
    tools=[convert_units],
)
```

```typescript TypeScript theme={null}
import { tool, createSdkMcpServer } from "@anthropic-ai/claude-agent-sdk";
import { z } from "zod";

const convert = tool(
  "convert_units",
  "Convert a value from one unit to another",
  {
    unit_type: z.enum(["length", "temperature", "weight"]).describe("Category of unit"),
    from_unit: z
      .string()
      .describe("Unit to convert from, e.g. kilometers, fahrenheit, pounds"),
    to_unit: z.string().describe("Unit to convert to"),
    value: z.number().describe("Value to convert")
  },
  async (args) => {
    type Conversions = Record<string, Record<string, (v: number) => number>>;

    const conversions: Conversions = {
      length: {
        kilometers_to_miles: (v) => v * 0.621371,
        miles_to_kilometers: (v) => v * 1.60934,
        meters_to_feet: (v) => v * 3.28084,
        feet_to_meters: (v) => v * 0.3048
      },
      temperature: {
        celsius_to_fahrenheit: (v) => (v * 9) / 5 + 32,
        fahrenheit_to_celsius: (v) => ((v - 32) * 5) / 9,
        celsius_to_kelvin: (v) => v + 273.15,
        kelvin_to_celsius: (v) => v - 273.15
      },
      weight: {
        kilograms_to_pounds: (v) => v * 2.20462,
        pounds_to_kilograms: (v) => v * 0.453592,
        grams_to_ounces: (v) => v * 0.035274,
        ounces_to_grams: (v) => v * 28.3495
      }
    };

    const key = `${args.from_unit}_to_${args.to_unit}`;
    const fn = conversions[args.unit_type]?.[key];

    if (!fn) {
      return {
        content: [
          {
            type: "text",
            text: `Unsupported conversion: ${args.from_unit} to ${args.to_unit}`
          }
        ],
        isError: true
      };
    }

    const result = fn(args.value);
    return {
      content: [
        {
          type: "text",
          text: `${args.value} ${args.from_unit} = ${result.toFixed(4)} ${args.to_unit}`
        }
      ]
    };
  }
);

const converterServer = createSdkMcpServer({
  name: "converter",
  version: "1.0.0",
  tools: [convert]
});
```

定义服务器后，以与天气示例相同的方式将其传递给 `query`。此示例在循环中发送三个不同的提示，以显示同一工具处理不同单位类型的情况。对于每个响应，它检查 `AssistantMessage` 对象（包含 Claude 在该轮中进行的工具调用）并在打印最终 `ResultMessage` 文本之前打印每个 `ToolUseBlock`。这让你可以看到 Claude 何时使用工具与何时从自己的知识中回答。

因为 [tool search](https://code.claude.com/docs/zh-CN/agent-sdk/tool-search.md) 默认启用，输出也可能包括 `ToolSearch` 调用，因为 Claude 加载延迟的工具模式。

```python Python theme={null}
import asyncio
from claude_agent_sdk import (
    query,
    ClaudeAgentOptions,
    ResultMessage,
    AssistantMessage,
    ToolUseBlock,
)


async def main():
    options = ClaudeAgentOptions(
        mcp_servers={"converter": converter_server},
        allowed_tools=["mcp__converter__convert_units"],
    )

    prompts = [
        "Convert 100 kilometers to miles.",
        "What is 72°F in Celsius?",
        "How many pounds is 5 kilograms?",
    ]

    for prompt in prompts:
        try:
            async for message in query(prompt=prompt, options=options):
                if isinstance(message, AssistantMessage):
                    for block in message.content:
                        if isinstance(block, ToolUseBlock):
                            print(f"[tool call] {block.name}({block.input})")
                elif isinstance(message, ResultMessage) and message.subtype == "success":
                    print(f"Q: {prompt}\nA: {message.result}\n")
        except Exception as error:
            # A single-shot query() raises after yielding an error result. Only success
            # results are printed above, so handle the failure here and continue with
            # the next prompt.
            print(f"Call failed: {error}")


asyncio.run(main())
```

```typescript TypeScript theme={null}
import { query } from "@anthropic-ai/claude-agent-sdk";

const prompts = [
  "Convert 100 kilometers to miles.",
  "What is 72°F in Celsius?",
  "How many pounds is 5 kilograms?"
];

for (const prompt of prompts) {
  try {
    for await (const message of query({
      prompt,
      options: {
        mcpServers: { converter: converterServer },
        allowedTools: ["mcp__converter__convert_units"]
      }
    })) {
      if (message.type === "assistant") {
        for (const block of message.message.content) {
          if (block.type === "tool_use") {
            console.log(`[tool call] ${block.name}`, block.input);
          }
        }
      } else if (message.type === "result" && message.subtype === "success") {
        console.log(`Q: ${prompt}\nA: ${message.result}\n`);
      }
    }
  } catch (error) {
    // A single-shot query() throws after yielding an error result. Only success
    // results are logged above, so handle the failure here and continue with
    // the next prompt.
    console.error(`Call failed: ${error}`);
  }
}
```

## 后续步骤

您可以在同一服务器中混合使用本页面上的模式：单个服务器可以同时包含数据库工具、API 网关工具和图像渲染器。

从这里开始：

* 如果您的服务器增长到数十个工具，请参阅[工具搜索](https://code.claude.com/docs/zh-CN/agent-sdk/tool-search.md)以延迟加载它们，直到 Claude 需要它们。
* 要连接到外部 MCP 服务器（文件系统、GitHub、Slack）而不是构建自己的服务器，请参阅[连接 MCP 服务器](https://code.claude.com/docs/zh-CN/agent-sdk/mcp.md)。
* 要控制哪些工具自动运行与需要批准，请参阅[配置权限](https://code.claude.com/docs/zh-CN/agent-sdk/permissions.md)。
