> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# TypeScript SDK V2 session API（已移除）

> 已移除的 V2 TypeScript Agent SDK session API 参考，具有用于多轮对话的基于会话的 send/stream 模式。

> [!WARNING]
>
> V2 session API 不再受支持。TypeScript Agent SDK 0.3.142 移除了 `unstable_v2_createSession`、`unstable_v2_resumeSession`、`unstable_v2_prompt` 以及 `SDKSession` 和 `SDKSessionOptions` 类型。
>
> 要迁移，请使用 [`query()` API](https://code.claude.com/docs/zh-CN/agent-sdk/typescript) 和它接受的 [session 选项](https://code.claude.com/docs/zh-CN/agent-sdk/sessions.md)。为多轮对话传递 `AsyncIterable<SDKUserMessage>`，或使用 `options.resume` 继续已保存的会话。如果您在 Agent SDK 0.2.x 或更早版本上维护代码，此页面保留供参考。

V2 是一个实验性的 session API，消除了对异步生成器和 yield 协调的需求。与其在各轮之间管理生成器状态，每一轮都是一个单独的 `send()`/`stream()` 周期。API 表面简化为三个概念：

* `createSession()` / `resumeSession()`：启动或继续对话
* `session.send()`：发送消息
* `session.stream()`：获取响应

## 安装

Agent SDK 0.2.x 是包含 V2 interface 的最后一个版本。包版本从 0.2.x 直接跳到 0.3.142，因此上面的移除版本和下面的安装固定版本描述的是同一个边界。要安装最后一个 V2 兼容版本，请固定主版本号和次版本号：

```bash theme={null}
npm install @anthropic-ai/claude-agent-sdk@0.2
```

> [!NOTE]
>
> SDK 为您的平台捆绑了一个本地 Claude Code 二进制文件作为可选依赖项，因此大多数安装无需单独安装 Claude Code。请参阅[快速入门的安装说明](https://code.claude.com/docs/zh-CN/agent-sdk/quickstart.md)了解需要单独安装的情况。

## 快速开始

### 单次提示

对于不需要维护会话的简单单轮查询，使用 `unstable_v2_prompt()`。此示例发送一个数学问题并记录答案：

```typescript theme={null}
import { unstable_v2_prompt } from "@anthropic-ai/claude-agent-sdk";

const result = await unstable_v2_prompt("What is 2 + 2?", {
  model: "claude-opus-4-7"
});
if (result.subtype === "success") {
  console.log(result.result);
}
```

<details>
  <summary>查看 V1 中的相同操作</summary>

  ```typescript theme={null}
  import { query } from "@anthropic-ai/claude-agent-sdk";

  const q = query({
    prompt: "What is 2 + 2?",
    options: { model: "claude-opus-4-7" }
  });

  for await (const msg of q) {
    if (msg.type === "result" && msg.subtype === "success") {
      console.log(msg.result);
    }
  }
  ```
</details>

### 基本会话

对于超出单个提示的交互，创建一个会话。V2 将发送和流式传输分为不同的步骤：

* `send()` 分派您的消息
* `stream()` 流式传输响应

这种明确的分离使得在轮次之间添加逻辑变得更容易（例如在发送后续消息之前处理响应）。

下面的示例创建一个会话，向 Claude 发送"Hello!"，并打印文本响应。它使用 [`await using`](https://www.typescriptlang.org/docs/handbook/release-notes/typescript-5-2.html#using-declarations-and-explicit-resource-management)（TypeScript 5.2+）在块退出时自动关闭会话。您也可以手动调用 `session.close()`。

```typescript theme={null}
import { unstable_v2_createSession } from "@anthropic-ai/claude-agent-sdk";

await using session = unstable_v2_createSession({
  model: "claude-opus-4-7"
});

await session.send("Hello!");
for await (const msg of session.stream()) {
  // Filter for assistant messages to get human-readable output
  if (msg.type === "assistant") {
    const text = msg.message.content
      .filter((block) => block.type === "text")
      .map((block) => block.text)
      .join("");
    console.log(text);
  }
}
```

<details>
  <summary>查看 V1 中的相同操作</summary>

  在 V1 中，输入和输出都通过单个异步生成器流动。对于基本提示，这看起来很相似，但添加多轮逻辑需要重新构造以使用输入生成器。

  ```typescript theme={null}
  import { query } from "@anthropic-ai/claude-agent-sdk";

  const q = query({
    prompt: "Hello!",
    options: { model: "claude-opus-4-7" }
  });

  for await (const msg of q) {
    if (msg.type === "assistant") {
      const text = msg.message.content
        .filter((block) => block.type === "text")
        .map((block) => block.text)
        .join("");
      console.log(text);
    }
  }
  ```
</details>

### 多轮对话

会话在多个交换中保持上下文。要继续对话，请在同一会话上再次调用 `send()`。Claude 会记住之前的轮次。

此示例提出一个数学问题，然后提出一个引用前一个答案的后续问题：

```typescript theme={null}
import { unstable_v2_createSession } from "@anthropic-ai/claude-agent-sdk";

await using session = unstable_v2_createSession({
  model: "claude-opus-4-7"
});

// Turn 1
await session.send("What is 5 + 3?");
for await (const msg of session.stream()) {
  // Filter for assistant messages to get human-readable output
  if (msg.type === "assistant") {
    const text = msg.message.content
      .filter((block) => block.type === "text")
      .map((block) => block.text)
      .join("");
    console.log(text);
  }
}

// Turn 2
await session.send("Multiply that by 2");
for await (const msg of session.stream()) {
  if (msg.type === "assistant") {
    const text = msg.message.content
      .filter((block) => block.type === "text")
      .map((block) => block.text)
      .join("");
    console.log(text);
  }
}
```

<details>
  <summary>查看 V1 中的相同操作</summary>

  ```typescript theme={null}
  import { query } from "@anthropic-ai/claude-agent-sdk";

  // Must create an async iterable to feed messages
  async function* createInputStream() {
    yield {
      type: "user",
      session_id: "",
      message: { role: "user", content: [{ type: "text", text: "What is 5 + 3?" }] },
      parent_tool_use_id: null
    };
    // Must coordinate when to yield next message
    yield {
      type: "user",
      session_id: "",
      message: { role: "user", content: [{ type: "text", text: "Multiply by 2" }] },
      parent_tool_use_id: null
    };
  }

  const q = query({
    prompt: createInputStream(),
    options: { model: "claude-opus-4-7" }
  });

  for await (const msg of q) {
    if (msg.type === "assistant") {
      const text = msg.message.content
        .filter((block) => block.type === "text")
        .map((block) => block.text)
        .join("");
      console.log(text);
    }
  }
  ```
</details>

### 会话恢复

如果您有来自之前交互的会话 ID，您可以稍后恢复它。这对于长时间运行的工作流或当您需要在应用程序重新启动时保持对话时很有用。

此示例创建一个会话，存储其 ID，关闭它，然后恢复对话：

```typescript theme={null}
import {
  unstable_v2_createSession,
  unstable_v2_resumeSession,
  type SDKMessage
} from "@anthropic-ai/claude-agent-sdk";

// Helper to extract text from assistant messages
function getAssistantText(msg: SDKMessage): string | null {
  if (msg.type !== "assistant") return null;
  return msg.message.content
    .filter((block) => block.type === "text")
    .map((block) => block.text)
    .join("");
}

// Create initial session and have a conversation
const session = unstable_v2_createSession({
  model: "claude-opus-4-7"
});

await session.send("Remember this number: 42");

// Get the session ID from any received message
let sessionId: string | undefined;
for await (const msg of session.stream()) {
  sessionId = msg.session_id;
  const text = getAssistantText(msg);
  if (text) console.log("Initial response:", text);
}

console.log("Session ID:", sessionId);
session.close();

// Later: resume the session using the stored ID
await using resumedSession = unstable_v2_resumeSession(sessionId!, {
  model: "claude-opus-4-7"
});

await resumedSession.send("What number did I ask you to remember?");
for await (const msg of resumedSession.stream()) {
  const text = getAssistantText(msg);
  if (text) console.log("Resumed response:", text);
}
```

<details>
  <summary>查看 V1 中的相同操作</summary>

  ```typescript theme={null}
  import { query } from "@anthropic-ai/claude-agent-sdk";

  // Create initial session
  const initialQuery = query({
    prompt: "Remember this number: 42",
    options: { model: "claude-opus-4-7" }
  });

  // Get session ID from any message
  let sessionId: string | undefined;
  for await (const msg of initialQuery) {
    sessionId = msg.session_id;
    if (msg.type === "assistant") {
      const text = msg.message.content
        .filter((block) => block.type === "text")
        .map((block) => block.text)
        .join("");
      console.log("Initial response:", text);
    }
  }

  console.log("Session ID:", sessionId);

  // Later: resume the session
  const resumedQuery = query({
    prompt: "What number did I ask you to remember?",
    options: {
      model: "claude-opus-4-7",
      resume: sessionId
    }
  });

  for await (const msg of resumedQuery) {
    if (msg.type === "assistant") {
      const text = msg.message.content
        .filter((block) => block.type === "text")
        .map((block) => block.text)
        .join("");
      console.log("Resumed response:", text);
    }
  }
  ```
</details>

### 清理

会话可以手动关闭或使用 [`await using`](https://www.typescriptlang.org/docs/handbook/release-notes/typescript-5-2.html#using-declarations-and-explicit-resource-management)（TypeScript 5.2+ 功能用于自动资源清理）自动关闭。如果您使用的是较旧的 TypeScript 版本或遇到兼容性问题，请改用手动清理。

下面的示例仅显示清理模式，不发送任何消息，因此运行它们不会产生任何输出。

**自动清理（TypeScript 5.2+）：**

```typescript theme={null}
import { unstable_v2_createSession } from "@anthropic-ai/claude-agent-sdk";

await using session = unstable_v2_createSession({
  model: "claude-opus-4-7"
});
// Session closes automatically when the block exits
```

**手动清理：**

```typescript theme={null}
import { unstable_v2_createSession } from "@anthropic-ai/claude-agent-sdk";

const session = unstable_v2_createSession({
  model: "claude-opus-4-7"
});
// ... use the session ...
session.close();
```

## API 参考

### `unstable_v2_createSession()`

为多轮对话创建新会话。

```typescript theme={null}
function unstable_v2_createSession(options: {
  model: string;
  // Additional options supported
}): SDKSession;
```

### `unstable_v2_resumeSession()`

按 ID 恢复现有会话。

```typescript theme={null}
function unstable_v2_resumeSession(
  sessionId: string,
  options: {
    model: string;
    // Additional options supported
  }
): SDKSession;
```

### `unstable_v2_prompt()`

用于单轮查询的单次便利函数。

```typescript theme={null}
function unstable_v2_prompt(
  prompt: string,
  options: {
    model: string;
    // Additional options supported
  }
): Promise<SDKResultMessage>;
```

### SDKSession interface

```typescript theme={null}
interface SDKSession {
  readonly sessionId: string;
  send(message: string | SDKUserMessage): Promise<void>;
  stream(): AsyncGenerator<SDKMessage, void>;
  close(): void;
}
```

## 功能可用性

V2 session API 不支持所有 V1 功能。以下功能需要使用 [V1 SDK](https://code.claude.com/docs/zh-CN/agent-sdk/typescript.md)：

* 会话分叉（`forkSession` 选项）
* 某些高级流式输入模式

## 另请参阅

* [TypeScript SDK 参考（V1）](https://code.claude.com/docs/zh-CN/agent-sdk/typescript.md) - 完整的 V1 SDK 文档
* [SDK 概述](https://code.claude.com/docs/zh-CN/agent-sdk/overview.md) - 常规 SDK 概念
* [GitHub 上的 V2 示例](https://github.com/anthropics/claude-agent-sdk-demos/tree/main/hello-world-v2) - 工作代码示例
