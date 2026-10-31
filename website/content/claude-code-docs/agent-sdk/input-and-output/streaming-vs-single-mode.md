> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# 流式输入

> 理解 Claude Agent SDK 的两种输入模式及何时使用每种模式

## 概述

Claude Agent SDK 支持两种不同的输入模式来与代理交互：

* **流式输入模式**：一个持久的、交互式的会话
* **单消息输入**：使用会话状态和恢复的一次性查询

## 流式输入模式（推荐）

流式输入模式是使用 Claude Agent SDK 的**首选**方式。它提供对代理功能的完全访问，并支持丰富的交互式体验。

它允许代理作为一个长期运行的进程运行，接收用户输入、处理中断、显示权限请求并处理会话管理。

### 优势

在流式输入模式中，您可以在具有以下功能的持久会话中工作：

* **图像上传**：直接将图像附加到消息中以进行视觉分析和理解
* **队列消息**：发送多条按顺序处理的消息，具有中断能力
* **工具集成**：在会话期间完全访问所有工具和自定义 MCP 服务器
* **实时反馈**：查看生成的响应，而不仅仅是最终结果
* **上下文持久性**：自然地跨多个回合维护对话上下文

### 实现示例

这些示例从工作目录读取名为 `diagram.png` 的图像。请先在那里创建一个，或更改文件名以指向您自己的图像。

```typescript TypeScript theme={null}
import { query, type SDKUserMessage } from "@anthropic-ai/claude-agent-sdk";
import { readFile } from "fs/promises";

async function* generateMessages(): AsyncGenerator<SDKUserMessage> {
  // First message
  yield {
    type: "user",
    message: {
      role: "user",
      content: "Analyze this codebase for security issues"
    },
    parent_tool_use_id: null
  };

  // Wait for conditions or user input
  await new Promise((resolve) => setTimeout(resolve, 2000));

  // Follow-up with image
  yield {
    type: "user",
    message: {
      role: "user",
      content: [
        {
          type: "text",
          text: "Review this architecture diagram"
        },
        {
          type: "image",
          source: {
            type: "base64",
            media_type: "image/png",
            data: await readFile("diagram.png", "base64")
          }
        }
      ]
    },
    parent_tool_use_id: null
  };
}

// Process streaming responses
for await (const message of query({
  prompt: generateMessages(),
  options: {
    maxTurns: 10,
    allowedTools: ["Read", "Grep"]
  }
})) {
  if (message.type === "result" && message.subtype === "success") {
    console.log(message.result);
  }
}
```

```python Python theme={null}
from claude_agent_sdk import (
    ClaudeSDKClient,
    ClaudeAgentOptions,
    AssistantMessage,
    TextBlock,
)
import asyncio
import base64


async def streaming_analysis():
    async def message_generator():
        # First message
        yield {
            "type": "user",
            "message": {
                "role": "user",
                "content": "Analyze this codebase for security issues",
            },
        }

        # Wait for conditions
        await asyncio.sleep(2)

        # Follow-up with image
        with open("diagram.png", "rb") as f:
            image_data = base64.b64encode(f.read()).decode()

        yield {
            "type": "user",
            "message": {
                "role": "user",
                "content": [
                    {"type": "text", "text": "Review this architecture diagram"},
                    {
                        "type": "image",
                        "source": {
                            "type": "base64",
                            "media_type": "image/png",
                            "data": image_data,
                        },
                    },
                ],
            },
        }

    # Use ClaudeSDKClient for streaming input
    options = ClaudeAgentOptions(max_turns=10, allowed_tools=["Read", "Grep"])

    async with ClaudeSDKClient(options) as client:
        # Send streaming input
        await client.query(message_generator())

        # Process responses
        async for message in client.receive_response():
            if isinstance(message, AssistantMessage):
                for block in message.content:
                    if isinstance(block, TextBlock):
                        print(block.text)


asyncio.run(streaming_analysis())
```

当您运行该示例时，TypeScript 版本会在每个响应完成时打印它。Python 版本的 `receive_response()` 循环在第一条结果消息处结束，因此它会打印安全分析；要读取两个响应，请使用一对 `query()` 和 `receive_response()`，如 [Python 参考中继续对话的示例](https://code.claude.com/docs/zh-CN/agent-sdk/python.md#example-continuing-a-conversation)所示。

如果图像块的 `source` 缺失或不是对象，SDK 不会报告错误。Claude Code 会向 Claude 发送一条文本说明来代替该图像，例如 `[Image could not be processed: image block has no source object]`，并且会话会继续进行。

> [!NOTE]
>
> 在 TypeScript SDK 中，如果您的消息生成器抛出异常，例如当它读取的文件丢失时，流会以一条错误消息结束，内容为 `Claude Code process aborted by user`，而不是原始错误，因此当您看到该消息时，请先检查生成器内部的代码。该错误前面可能还有一长行捆绑 SDK 源代码的缩小代码，因此请阅读输出末尾的错误文本。
>
> 在 Python SDK 中，生成器异常在调试级别被记录，会话会停滞而不会引发异常，因此如果流式会话挂起且没有输出，请启用调试日志记录并检查您的生成器。

## 单消息输入

单消息输入更简单但功能更受限。

### 何时使用单消息输入

在以下情况下使用单消息输入：

* 您需要一次性响应
* 您不需要图像附件或中间会话控制方法
* 您需要在无状态环境中运行，例如 lambda 函数

### 限制

> [!WARNING]
>
> 单消息输入模式**不**支持：
>
> * 消息中的直接图像附件
> * 动态消息队列
> * 实时中断
> * 自然的多轮对话

如果查询以错误结果结束，例如 `error_max_turns`，单个消息 `query()` 调用会抛出一个错误，该错误包含在生成最终结果消息后的失败文本，因此如果您的代码需要继续，请将循环包装在 try 块中。有关结果子类型，请参阅[处理结果](https://code.claude.com/docs/zh-CN/agent-sdk/agent-loop.md#handle-the-result)。

### 实现示例

```typescript TypeScript theme={null}
import { query } from "@anthropic-ai/claude-agent-sdk";

// Simple one-shot query
// query() throws after an error result, such as error_max_turns
try {
  for await (const message of query({
    prompt: "Explain the authentication flow",
    options: {
      maxTurns: 5,
      allowedTools: ["Read", "Grep"]
    }
  })) {
    if (message.type === "result" && message.subtype === "success") {
      console.log(message.result);
    }
  }
} catch (error) {
  console.error(`Query failed: ${error}`);
}

// Continue conversation with session management
try {
  for await (const message of query({
    prompt: "Now explain the authorization process",
    options: {
      continue: true,
      maxTurns: 5
    }
  })) {
    if (message.type === "result" && message.subtype === "success") {
      console.log(message.result);
    }
  }
} catch (error) {
  console.error(`Query failed: ${error}`);
}
```

```python Python theme={null}
from claude_agent_sdk import query, ClaudeAgentOptions, ResultMessage
import asyncio


async def single_message_example():
    # Simple one-shot query using query() function
    # query() raises ResultError after an error result, such as error_max_turns
    try:
        async for message in query(
            prompt="Explain the authentication flow",
            options=ClaudeAgentOptions(max_turns=5, allowed_tools=["Read", "Grep"]),
        ):
            if isinstance(message, ResultMessage) and message.subtype == "success":
                print(message.result)
    except Exception as e:
        print(f"Query failed: {e}")

    # Continue conversation with session management
    try:
        async for message in query(
            prompt="Now explain the authorization process",
            options=ClaudeAgentOptions(continue_conversation=True, max_turns=5),
        ):
            if isinstance(message, ResultMessage) and message.subtype == "success":
                print(message.result)
    except Exception as e:
        print(f"Query failed: {e}")


asyncio.run(single_message_example())
```

运行示例时，每个查询都会打印其最终结果文本：首先是身份验证说明，然后是授权说明。
