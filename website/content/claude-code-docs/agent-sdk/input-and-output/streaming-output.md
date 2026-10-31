> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# 实时流式传输响应

> 当文本和工具调用流入时，从 Agent SDK 获取实时响应

默认情况下，Agent SDK 在 Claude 完成生成每个非空内容块（例如文本块或工具调用）后会产生完整的 `AssistantMessage`。要在文本和工具调用生成时接收增量更新，请启用部分消息流式传输。

> [!TIP]
>
> 本页面涵盖输出流式传输（实时接收令牌）。有关输入模式（如何发送消息），请参阅[向代理发送消息](https://code.claude.com/docs/zh-CN/agent-sdk/streaming-vs-single-mode.md)。您也可以[通过 CLI 使用 Agent SDK 流式传输响应](https://code.claude.com/docs/zh-CN/headless.md)。

## 启用流式输出

要启用流式传输，请在选项中将 `include_partial_messages`（Python）或 `includePartialMessages`（TypeScript）设置为 `true`。这会导致 SDK 产生包含原始 API 事件的 `StreamEvent` 消息，这些事件在到达时产生，除了通常的 `AssistantMessage` 和 `ResultMessage` 之外。

您的代码需要：

1. 检查每条消息的类型以区分 `StreamEvent` 和其他消息类型
2. 对于 `StreamEvent`，提取 `event` 字段并检查其 `type`
3. 查找 `content_block_delta` 事件，其中 `delta.type` 是 `text_delta`，这些事件包含实际的文本块

下面的示例启用流式传输并在文本块到达时打印它们。注意嵌套的类型检查：首先是 `StreamEvent`，然后是 `content_block_delta`，最后是 `text_delta`：

```python Python theme={null}
from claude_agent_sdk import query, ClaudeAgentOptions
from claude_agent_sdk.types import StreamEvent
import asyncio


async def stream_response():
    options = ClaudeAgentOptions(
        include_partial_messages=True,
        allowed_tools=["Bash", "Read"],
    )

    async for message in query(prompt="List the files in my project", options=options):
        if isinstance(message, StreamEvent):
            event = message.event
            if event.get("type") == "content_block_delta":
                delta = event.get("delta", {})
                if delta.get("type") == "text_delta":
                    print(delta.get("text", ""), end="", flush=True)


asyncio.run(stream_response())
```

```typescript TypeScript theme={null}
import { query } from "@anthropic-ai/claude-agent-sdk";

for await (const message of query({
  prompt: "List the files in my project",
  options: {
    includePartialMessages: true,
    allowedTools: ["Bash", "Read"]
  }
})) {
  if (message.type === "stream_event") {
    const event = message.event;
    if (event.type === "content_block_delta") {
      if (event.delta.type === "text_delta") {
        process.stdout.write(event.delta.text);
      }
    }
  }
}
```

## StreamEvent 参考

启用部分消息时，您会收到包装在对象中的原始 Claude API 流事件。该类型在每个 SDK 中有不同的名称：

* **Python**: [`StreamEvent`](https://code.claude.com/docs/zh-CN/agent-sdk/python#streamevent)（从 `claude_agent_sdk.types` 导入）
* **TypeScript**: [`SDKPartialAssistantMessage`](https://code.claude.com/docs/zh-CN/agent-sdk/typescript#sdkpartialassistantmessage)，其中 `type: 'stream_event'`

两者都包含原始 Claude API 事件，而不是累积的文本。您需要自己提取和累积文本增量。

`parent_tool_use_id` 字段在 Python 中始终为 `None`，在 TypeScript 中始终为 `null`。流事件仅针对主会话发出；来自子代理的令牌级增量不会被转发。要将输出归属于子代理，请使用完整消息，这些消息携带 `parent_tool_use_id`。请参阅[检测子代理调用](https://code.claude.com/docs/zh-CN/agent-sdk/subagents.md#detect-subagent-invocation)。

Claude Code 在轮次的第一个非 ping 流事件上设置 `user_message_uuid`，以及当轮次回答的消息更改时再次设置，条件在 [`user_message_uuid`](https://code.claude.com/docs/zh-CN/agent-sdk/typescript#user_message_uuid) 中。Python `StreamEvent` 不公开此字段。

`event` 字段包含来自 [Claude API](https://platform.claude.com/docs/en/build-with-claude/streaming#event-types) 的原始流事件。常见的事件类型包括：

| 事件类型 | 描述 |
| :- | :- |
| `message_start` | 新消息的开始 |
| `content_block_start` | 新内容块的开始（文本或工具使用） |
| `content_block_delta` | 内容的增量更新 |
| `content_block_stop` | 内容块的结束 |
| `message_delta` | 消息级更新（停止原因、使用情况） |
| `message_stop` | 消息的结束 |

## 消息流

Claude Code 在每个非空内容块完成时发出一个 `AssistantMessage`，因此包含文本块和工具调用的响应会产生两个 `AssistantMessage` 对象。每个对象仅携带其自己的内容块，两者共享相同的消息 ID，在 TypeScript 中读作 `message.message.id`，在 Python 中读作 `message.message_id`。启用部分消息后，每个 `AssistantMessage` 在该块的 `content_block_stop` 事件之前到达，你会按以下顺序接收消息：

```text theme={null}
StreamEvent (message_start)
StreamEvent (content_block_start) - text block
StreamEvent (content_block_delta) - text chunks...
AssistantMessage - complete text block
StreamEvent (content_block_stop)
StreamEvent (content_block_start) - tool_use block
StreamEvent (content_block_delta) - tool input chunks...
AssistantMessage - complete tool_use block
StreamEvent (content_block_stop)
StreamEvent (message_delta)
StreamEvent (message_stop)
... tool executes ...
... more streaming events for next turn ...
ResultMessage - final result
```

未启用部分消息时，你会接收除 `StreamEvent` 之外的所有消息类型。常见类型包括 `SystemMessage`（会话初始化）、`AssistantMessage`（完整内容块）、`ResultMessage`（最终结果）和一个紧凑边界消息，指示何时压缩了对话历史记录（TypeScript 中为 `SDKCompactBoundaryMessage`；Python 中为带有子类型 `"compact_boundary"` 的 `SystemMessage`）。

### 处理被中断的流

如果流在消息中途被中断，您仍会在该轮次结束前收到该消息的 `message_stop`。导致流被中断的原因包括轮次被中断、连接断开以及回退到非流式请求。在该 `message_stop` 之前，`content_block_stop` 和 `message_delta` 可能会缺失：

* **`content_block_stop`**：会为处于打开状态的块到达，除非该块是工具调用。如果 `message_stop` 到达时某个工具调用的块仍处于打开状态，请将该调用的输入视为不完整。
* **`message_delta`**：在您中断该轮次时会到达，而在连接断开或回退到非流式请求后可能会缺失，因此对于被中断的消息，请勿等待它。

在 Claude Code v2.1.290 之前，被中断的流可能会在没有 `message_stop` 的情况下结束轮次，因此您根据流事件渲染的回复可能会一直显示为进行中。TypeScript Agent SDK 从 v0.3.290 起捆绑 Claude Code v2.1.290 或更高版本，Python Agent SDK 则从 v0.2.164 起捆绑。如果轮次结束后回复仍显示为进行中，请更新 SDK。

## 流式传输工具调用

工具调用也会增量流式传输。您可以跟踪工具何时开始、在生成时接收其输入，以及查看它们何时完成。下面的示例跟踪当前被调用的工具并在流式传输时累积 JSON 输入。它使用三种事件类型：

* `content_block_start`：工具开始
* `content_block_delta`，带有 `input_json_delta`：输入块到达
* `content_block_stop`：工具调用完成

```python Python theme={null}
from claude_agent_sdk import query, ClaudeAgentOptions
from claude_agent_sdk.types import StreamEvent
import asyncio


async def stream_tool_calls():
    options = ClaudeAgentOptions(
        include_partial_messages=True,
        allowed_tools=["Read", "Bash"],
    )

    # 跟踪当前工具并累积其输入 JSON
    current_tool = None
    tool_input = ""

    async for message in query(prompt="Read the README.md file", options=options):
        if isinstance(message, StreamEvent):
            event = message.event
            event_type = event.get("type")

            if event_type == "content_block_start":
                # 新工具调用开始
                content_block = event.get("content_block", {})
                if content_block.get("type") == "tool_use":
                    current_tool = content_block.get("name")
                    tool_input = ""
                    print(f"Starting tool: {current_tool}")

            elif event_type == "content_block_delta":
                delta = event.get("delta", {})
                if delta.get("type") == "input_json_delta":
                    # 在流式传输时累积 JSON 输入
                    chunk = delta.get("partial_json", "")
                    tool_input += chunk
                    print(f"  Input chunk: {chunk}")

            elif event_type == "content_block_stop":
                # 工具调用完成 - 显示最终输入
                if current_tool:
                    print(f"Tool {current_tool} called with: {tool_input}")
                    current_tool = None


asyncio.run(stream_tool_calls())
```

```typescript TypeScript theme={null}
import { query } from "@anthropic-ai/claude-agent-sdk";

// 跟踪当前工具并累积其输入 JSON
let currentTool: string | null = null;
let toolInput = "";

for await (const message of query({
  prompt: "Read the README.md file",
  options: {
    includePartialMessages: true,
    allowedTools: ["Read", "Bash"]
  }
})) {
  if (message.type === "stream_event") {
    const event = message.event;

    if (event.type === "content_block_start") {
      // 新工具调用开始
      if (event.content_block.type === "tool_use") {
        currentTool = event.content_block.name;
        toolInput = "";
        console.log(`Starting tool: ${currentTool}`);
      }
    } else if (event.type === "content_block_delta") {
      if (event.delta.type === "input_json_delta") {
        // 在流式传输时累积 JSON 输入
        const chunk = event.delta.partial_json;
        toolInput += chunk;
        console.log(`  Input chunk: ${chunk}`);
      }
    } else if (event.type === "content_block_stop") {
      // 工具调用完成 - 显示最终输入
      if (currentTool) {
        console.log(`Tool ${currentTool} called with: ${toolInput}`);
        currentTool = null;
      }
    }
  }
}
```

## 构建流式 UI

此示例将文本和工具流式传输结合到一个有凝聚力的 UI 中。它跟踪代理当前是否正在执行工具（使用 `in_tool` 标志）以显示状态指示器，如 `[Using Read...]`，同时工具运行。当不在工具中时文本正常流式传输，工具完成会触发"完成"消息。此模式对于需要在多步骤代理任务期间显示进度的聊天界面很有用。

```python Python theme={null}
from claude_agent_sdk import query, ClaudeAgentOptions, ResultMessage
from claude_agent_sdk.types import StreamEvent
import asyncio
import sys


async def streaming_ui():
    options = ClaudeAgentOptions(
        include_partial_messages=True,
        allowed_tools=["Read", "Bash", "Grep"],
    )

    # 跟踪我们当前是否在工具调用中
    in_tool = False

    async for message in query(
        prompt="Find all TODO comments in the codebase", options=options
    ):
        if isinstance(message, StreamEvent):
            event = message.event
            event_type = event.get("type")

            if event_type == "content_block_start":
                content_block = event.get("content_block", {})
                if content_block.get("type") == "tool_use":
                    # 工具调用开始 - 显示状态指示器
                    tool_name = content_block.get("name")
                    print(f"\n[Using {tool_name}...]", end="", flush=True)
                    in_tool = True

            elif event_type == "content_block_delta":
                delta = event.get("delta", {})
                # 仅在不执行工具时流式传输文本
                if delta.get("type") == "text_delta" and not in_tool:
                    sys.stdout.write(delta.get("text", ""))
                    sys.stdout.flush()

            elif event_type == "content_block_stop":
                if in_tool:
                    # 工具调用完成
                    print(" done", flush=True)
                    in_tool = False

        elif isinstance(message, ResultMessage):
            # 代理完成所有工作
            print(f"\n\n--- Complete ---")


asyncio.run(streaming_ui())
```

```typescript TypeScript theme={null}
import { query } from "@anthropic-ai/claude-agent-sdk";

// 跟踪我们当前是否在工具调用中
let inTool = false;

for await (const message of query({
  prompt: "Find all TODO comments in the codebase",
  options: {
    includePartialMessages: true,
    allowedTools: ["Read", "Bash", "Grep"]
  }
})) {
  if (message.type === "stream_event") {
    const event = message.event;

    if (event.type === "content_block_start") {
      if (event.content_block.type === "tool_use") {
        // 工具调用开始 - 显示状态指示器
        process.stdout.write(`\n[Using ${event.content_block.name}...]`);
        inTool = true;
      }
    } else if (event.type === "content_block_delta") {
      // 仅在不执行工具时流式传输文本
      if (event.delta.type === "text_delta" && !inTool) {
        process.stdout.write(event.delta.text);
      }
    } else if (event.type === "content_block_stop") {
      if (inTool) {
        // 工具调用完成
        console.log(" done");
        inTool = false;
      }
    }
  } else if (message.type === "result") {
    // 代理完成所有工作
    console.log("\n\n--- Complete ---");
  }
}
```

## 已知限制

* **结构化输出**：启用部分消息时，JSON 作为工具调用的未验证 `input_json_delta` 块进行流式传输，只有验证后的结果才会到达最终的 `ResultMessage.structured_output`。[识别 StructuredOutput 工具调用](https://code.claude.com/docs/zh-CN/agent-sdk/structured-outputs.md#recognize-the-structuredoutput-tool-call)介绍了该调用在您的流中如何呈现。

## 后续步骤

现在您可以实时流式传输文本和工具调用，请探索这些相关主题：

* [交互式与一次性查询](https://code.claude.com/docs/zh-CN/agent-sdk/streaming-vs-single-mode.md)：为您的用例选择输入模式
* [结构化输出](https://code.claude.com/docs/zh-CN/agent-sdk/structured-outputs.md)：从代理获取类型化的 JSON 响应
* [权限](https://code.claude.com/docs/zh-CN/agent-sdk/permissions.md)：控制代理可以使用哪些工具
