> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# 处理批准和用户输入

> 向用户显示 Claude 的批准请求和澄清问题，然后将他们的决定返回给 SDK。

在处理任务时，Claude 有时需要与用户进行沟通。它可能需要在删除文件前获得许可，或需要询问为新项目使用哪个数据库。您的应用程序需要向用户显示这些请求，以便 Claude 可以继续使用他们的输入。

Claude 在两种情况下请求用户输入：当它需要**使用工具的权限**（如删除文件或运行命令）时，以及当它有**澄清问题**（通过 `AskUserQuestion` 工具）时。两者都会触发您的 `canUseTool` 回调，该回调会暂停执行，直到您返回响应。这与普通对话轮次不同，在普通对话轮次中 Claude 完成后等待您的下一条消息。

对于澄清问题，Claude 生成问题和选项。您的角色是向用户呈现这些问题，并返回他们的选择。您不能向此流程添加自己的问题；如果您需要自己询问用户某些内容，请在应用程序逻辑中单独进行。

回调可以无限期地保持待处理状态。执行保持暂停状态，直到您的回调返回。如果用户可能需要比您的进程能够合理保持运行的时间更长的时间来响应，请注册一个 [`PreToolUse` hook](https://code.claude.com/docs/zh-CN/agent-sdk/hooks)，它返回 [`defer` 决定](https://code.claude.com/docs/zh-CN/hooks#defer-a-tool-call-for-later)，而不是在回调中等待，以便进程可以退出并稍后从持久化会话恢复。

本指南向您展示如何检测每种类型的请求并做出适当的响应。

## 检测 Claude 何时需要输入

在您的查询选项中传递 `canUseTool` 回调。每当 Claude 需要用户输入时，回调就会触发，接收工具名称和输入作为参数：

```python Python theme={null}
from claude_agent_sdk import ClaudeAgentOptions


async def handle_tool_request(tool_name, input_data, context):
    # 提示用户并返回允许或拒绝
    ...


options = ClaudeAgentOptions(can_use_tool=handle_tool_request)
```

```typescript TypeScript theme={null}
import type { CanUseTool } from "@anthropic-ai/claude-agent-sdk";

const handleToolRequest: CanUseTool = async (toolName, input, options) => {
  // options includes { signal: AbortSignal, suggestions?: PermissionUpdate[] }
  // 在此提示用户，然后返回允许或拒绝
  return { behavior: "deny", message: "User declined" };
};

const options = { canUseTool: handleToolRequest };
```

回调在两种情况下触发：

1. **工具需要批准**：Claude 想要使用不被[权限规则](https://code.claude.com/docs/zh-CN/agent-sdk/permissions.md)或权限模式自动批准的工具。检查 `tool_name` 以获取工具（例如 `"Bash"`、`"Write"`）。
2. **Claude 提出问题**：Claude 调用 `AskUserQuestion` 工具。检查 `tool_name == "AskUserQuestion"` 以不同方式处理它。如果您指定 `tools` 数组，请包含 `AskUserQuestion` 以使其工作。有关详细信息，请参阅[处理澄清问题](#handle-clarifying-questions)。

> [!WARNING]
>
> **回调永远不会对自动批准的工具触发。** [权限评估流程](https://code.claude.com/docs/zh-CN/agent-sdk/permissions.md#how-permissions-are-evaluated)中任何较早的批准、允许规则或 `acceptEdits` 或 `bypassPermissions` 等模式会在咨询 `canUseTool` 之前解决调用。如果您在 `allowed_tools` 中列出一个工具，仅当[评估流程](https://code.claude.com/docs/zh-CN/agent-sdk/permissions.md#how-permissions-are-evaluated)将调用路由回提示（例如询问规则或 `plan` 模式）时，该工具的 `canUseTool` 检查才会运行。对于必须应用于每个工具调用的逻辑，请使用 [`PreToolUse` hook](https://code.claude.com/docs/zh-CN/agent-sdk/hooks)，它在流程的其余部分之前执行，可以允许、拒绝或修改请求。
>
> 允许规则不会预先批准[任何模式都不会自动批准的操作](https://code.claude.com/docs/zh-CN/permission-modes.md#actions-no-mode-auto-approves)；有关其中哪些到达回调以及在 `dontAsk` 和 `auto` 模式下发生的情况，请参阅[权限如何评估](https://code.claude.com/docs/zh-CN/agent-sdk/permissions.md#how-permissions-are-evaluated)。

您还可以使用 [`PermissionRequest` hook](https://code.claude.com/docs/zh-CN/agent-sdk/hooks#available-hooks) 在 Claude 等待批准时发送外部通知（Slack、电子邮件、推送）。

## 处理工具批准请求

一旦您在查询选项中传递了 `canUseTool` 回调，当 Claude 想要使用不被权限流中较早步骤批准的工具时，它就会触发。在某些配置中，例如 `dontAsk` 模式，Claude Code 不会调用它；[权限如何被评估](https://code.claude.com/docs/zh-CN/agent-sdk/permissions.md#how-permissions-are-evaluated)的最后一步列出了它们并说明了调用会发生什么。

您的回调接收三个参数：

| 参数 | 描述 |
| - | - |
| `toolName` | Claude 想要使用的工具的名称（例如 `"Bash"`、`"Write"`、`"Edit"`） |
| `input` | Claude 传递给工具的参数。内容因工具而异。 |
| `options` (TS) / `context` (Python) | 附加上下文，包括可选的 `suggestions`（建议的 `PermissionUpdate` 条目以避免重新提示）和取消信号。在 TypeScript 中，`signal` 是 `AbortSignal`；在 Python 中，信号字段保留供将来使用。有关 Python，请参阅 [`ToolPermissionContext`](https://code.claude.com/docs/zh-CN/agent-sdk/python#toolpermissioncontext)。 |

`input` 对象包含工具特定的参数。常见示例：

| 工具 | 输入字段 |
| - | - |
| `Bash` | `command`、`description`、`timeout` |
| `Write` | `file_path`、`content` |
| `Edit` | `file_path`、`old_string`、`new_string` |
| `Read` | `file_path`、`offset`、`limit` |

有关完整的输入架构，请参阅 SDK 参考：[Python](https://code.claude.com/docs/zh-CN/agent-sdk/python.md#tool-input%2Foutput-types) | [TypeScript](https://code.claude.com/docs/zh-CN/agent-sdk/typescript.md#tool-input-types)。

您可以向用户显示此信息，以便他们可以决定是否允许或拒绝该操作，然后返回适当的响应。

以下示例要求 Claude 创建和删除测试文件。当 Claude 尝试每个操作时，回调会将工具请求打印到终端并提示进行 y/n 批准。

```python Python theme={null}
import asyncio

from claude_agent_sdk import ClaudeAgentOptions, ResultMessage, query
from claude_agent_sdk.types import (
    HookMatcher,
    PermissionResultAllow,
    PermissionResultDeny,
    ToolPermissionContext,
)


async def can_use_tool(
    tool_name: str, input_data: dict, context: ToolPermissionContext
) -> PermissionResultAllow | PermissionResultDeny:
    # 显示工具请求
    print(f"\nTool: {tool_name}")
    if tool_name == "Bash":
        print(f"Command: {input_data.get('command')}")
        if input_data.get("description"):
            print(f"Description: {input_data.get('description')}")
    else:
        print(f"Input: {input_data}")

    # 获取用户批准
    response = input("Allow this action? (y/n): ")

    # 根据用户的响应返回允许或拒绝
    if response.lower() == "y":
        # 允许：工具使用原始（或修改的）输入执行
        return PermissionResultAllow(updated_input=input_data)
    else:
        # 拒绝：工具不执行，Claude 看到该消息
        return PermissionResultDeny(message="User denied this action")


# 必需的解决方法：虚拟 hook 保持流打开以供 can_use_tool 使用
async def dummy_hook(input_data, tool_use_id, context):
    return {"continue_": True}


async def prompt_stream():
    yield {
        "type": "user",
        "message": {
            "role": "user",
            "content": "Create a test file in /tmp and then delete it",
        },
    }


async def main():
    async for message in query(
        prompt=prompt_stream(),
        options=ClaudeAgentOptions(
            can_use_tool=can_use_tool,
            hooks={"PreToolUse": [HookMatcher(matcher=None, hooks=[dummy_hook])]},
        ),
    ):
        if isinstance(message, ResultMessage) and message.subtype == "success":
            print(message.result)


asyncio.run(main())
```

```typescript TypeScript theme={null}
import { query } from "@anthropic-ai/claude-agent-sdk";
import * as readline from "readline";

// 帮助程序在终端中提示用户输入
function prompt(question: string): Promise<string> {
  const rl = readline.createInterface({
    input: process.stdin,
    output: process.stdout
  });
  return new Promise((resolve) =>
    rl.question(question, (answer) => {
      rl.close();
      resolve(answer);
    })
  );
}

for await (const message of query({
  prompt: "Create a test file in /tmp and then delete it",
  options: {
    canUseTool: async (toolName, input) => {
      // 显示工具请求
      console.log(`\nTool: ${toolName}`);
      if (toolName === "Bash") {
        console.log(`Command: ${input.command}`);
        if (input.description) console.log(`Description: ${input.description}`);
      } else {
        console.log(`Input: ${JSON.stringify(input, null, 2)}`);
      }

      // 获取用户批准
      const response = await prompt("Allow this action? (y/n): ");

      // 根据用户的响应返回允许或拒绝
      if (response.toLowerCase() === "y") {
        // 允许：工具使用原始（或修改的）输入执行
        return { behavior: "allow", updatedInput: input };
      } else {
        // 拒绝：工具不执行，Claude 看到该消息
        return { behavior: "deny", message: "User denied this action" };
      }
    }
  }
})) {
  if ("result" in message) console.log(message.result);
}
```

此示例使用 y/n 流，其中除 `y` 之外的任何输入都被视为拒绝。在实践中，您可能会构建一个更丰富的 UI，让用户修改请求、提供反馈或完全重定向 Claude。有关所有响应方式，请参阅[响应工具请求](#respond-to-tool-requests)。

### 响应工具请求

您的回调返回两种响应类型之一：

| 响应 | Python | TypeScript |
| - | - | - |
| **允许** | `PermissionResultAllow(updated_input=...)` | `{ behavior: "allow", updatedInput }` |
| **拒绝** | `PermissionResultDeny(message=...)` | `{ behavior: "deny", message }` |

允许时，工具使用 Claude 请求的输入运行，除非您返回修改的输入，TypeScript 中的 `updatedInput` 或 Python 中的 `updated_input`。在 v2.1.207 之前，Claude Code 拒绝了省略 `updatedInput` 的允许结果，并以验证错误拒绝了工具调用。

拒绝时，提供说明原因的消息。Claude 会看到此消息并可能调整其方法。

除了允许或拒绝之外，您还可以修改工具的输入或提供帮助 Claude 调整其方法的上下文：

* **批准**：让工具按 Claude 请求的方式执行
* **批准并进行更改**：在执行前修改输入（例如，清理路径、添加约束）
* **批准并记住**：回显建议的权限规则，以便匹配的调用在下次跳过提示
* **拒绝**：阻止工具并告诉 Claude 原因
* **建议替代方案**：阻止但指导 Claude 朝向用户想要的方向
* **完全重定向**：使用[流输入](https://code.claude.com/docs/zh-CN/agent-sdk/streaming-vs-single-mode.md)向 Claude 发送全新指令

以下代码片段中的 `ask_user` 和 `askUser` 帮助程序代表您应用程序自己的提示 UI。

> [tab] 批准
>
> 用户按原样批准该操作。从您的回调中传递 `input` 不变，工具完全按 Claude 请求的方式执行。
>
> ```python Python theme={null}
> async def can_use_tool(tool_name, input_data, context):
>     print(f"Claude wants to use {tool_name}")
>     approved = await ask_user("Allow this action?")
>
>     if approved:
>         return PermissionResultAllow(updated_input=input_data)
>     return PermissionResultDeny(message="User declined")
> ```
>
> ```typescript TypeScript theme={null}
> canUseTool: async (toolName, input) => {
>   console.log(`Claude wants to use ${toolName}`);
>   const approved = await askUser("Allow this action?");
>
>   if (approved) {
>     return { behavior: "allow", updatedInput: input };
>   }
>   return { behavior: "deny", message: "User declined" };
> };
> ```

> [tab] 批准并进行更改
>
> 用户批准但想先修改请求。您可以在工具执行前更改输入。Claude 会看到结果，但不会被告知您更改了任何内容。对于清理参数、添加约束或限制访问范围很有用。
>
> ```python Python theme={null}
> async def can_use_tool(tool_name, input_data, context):
>     if tool_name == "Bash":
>         # 用户批准，但将所有命令限制在沙箱中
>         sandboxed_input = {**input_data}
>         sandboxed_input["command"] = input_data["command"].replace(
>             "/tmp", "/tmp/sandbox"
>         )
>         return PermissionResultAllow(updated_input=sandboxed_input)
>     return PermissionResultAllow(updated_input=input_data)
> ```
>
> ```typescript TypeScript theme={null}
> canUseTool: async (toolName, input) => {
>   if (toolName === "Bash") {
>     // 用户批准，但将所有命令限制在沙箱中
>     const sandboxedInput = {
>       ...input,
>       command: input.command.replace("/tmp", "/tmp/sandbox")
>     };
>     return { behavior: "allow", updatedInput: sandboxedInput };
>   }
>   return { behavior: "allow", updatedInput: input };
> };
> ```

> [tab] 批准并记住
>
> 用户批准并且不想再被询问此类调用。第三个回调参数携带 `suggestions`，一个现成的 [`PermissionUpdate`](https://code.claude.com/docs/zh-CN/agent-sdk/typescript#permissionupdate) 条目数组。在 `updatedPermissions` 中回显其中一个以应用它。带有 `localSettings` 目标的建议会将规则写入 `.claude/settings.local.json`，以便将来的会话跳过匹配调用的提示。
>
> 在 TypeScript 中，跳过选项携带 [`suppressAlwaysAllowRule: true`](https://code.claude.com/docs/zh-CN/agent-sdk/typescript#canusetool) 的请求的始终允许选择。该提示需要 Agent SDK v0.3.268 或更高版本，Python `context` 不携带它。
>
> Python 示例需要 `claude-agent-sdk` 0.1.80 或更高版本。
>
> ```python Python theme={null}
> async def can_use_tool(tool_name, input_data, context):
>     choice = await ask_user(f"Allow {tool_name}?", ["once", "always", "no"])
>
>     if choice == "always":
>         persist = [
>             s for s in context.suggestions if s.destination == "localSettings"
>         ]
>         return PermissionResultAllow(
>             updated_input=input_data, updated_permissions=persist
>         )
>     if choice == "once":
>         return PermissionResultAllow(updated_input=input_data)
>     return PermissionResultDeny(message="User declined")
> ```
>
> ```typescript TypeScript theme={null}
> canUseTool: async (toolName, input, { suggestions = [] }) => {
>   const choice = await askUser(`Allow ${toolName}?`, ["once", "always", "no"]);
>
>   if (choice === "always") {
>     const persist = suggestions.filter(
>       (s) => s.destination === "localSettings"
>     );
>     return {
>       behavior: "allow",
>       updatedInput: input,
>       updatedPermissions: persist
>     };
>   }
>   if (choice === "once") {
>     return { behavior: "allow", updatedInput: input };
>   }
>   return { behavior: "deny", message: "User declined" };
> };
> ```

> [tab] 拒绝
>
> 用户不希望发生此操作。阻止工具并提供说明原因的消息。Claude 会看到此消息并可能尝试不同的方法。
>
> ```python Python theme={null}
> async def can_use_tool(tool_name, input_data, context):
>     approved = await ask_user(f"Allow {tool_name}?")
>
>     if not approved:
>         return PermissionResultDeny(message="User rejected this action")
>     return PermissionResultAllow(updated_input=input_data)
> ```
>
> ```typescript TypeScript theme={null}
> canUseTool: async (toolName, input) => {
>   const approved = await askUser(`Allow ${toolName}?`);
>
>   if (!approved) {
>     return {
>       behavior: "deny",
>       message: "User rejected this action"
>     };
>   }
>   return { behavior: "allow", updatedInput: input };
> };
> ```

> [tab] 建议替代方案
>
> 用户不想要此特定操作，但有不同的想法。阻止工具并在您的消息中包含指导。Claude 将阅读此内容并根据您的反馈决定如何继续。
>
> ```python Python theme={null}
> async def can_use_tool(tool_name, input_data, context):
>     if tool_name == "Bash" and "rm" in input_data.get("command", ""):
>         # 用户不想删除，建议改为存档
>         return PermissionResultDeny(
>             message="User doesn't want to delete files. They asked if you could compress them into an archive instead."
>         )
>     return PermissionResultAllow(updated_input=input_data)
> ```
>
> ```typescript TypeScript theme={null}
> canUseTool: async (toolName, input) => {
>   if (toolName === "Bash" && input.command.includes("rm")) {
>     // 用户不想删除，建议改为存档
>     return {
>       behavior: "deny",
>       message:
>         "User doesn't want to delete files. They asked if you could compress them into an archive instead."
>     };
>   }
>   return { behavior: "allow", updatedInput: input };
> };
> ```

> [tab] 完全重定向
>
> 对于完全改变方向（不仅仅是轻推），使用[流输入](https://code.claude.com/docs/zh-CN/agent-sdk/streaming-vs-single-mode.md)向 Claude 发送新指令。这绕过当前工具请求并为 Claude 提供全新指令来遵循。

## 处理澄清问题

当 Claude 需要在具有多个有效方法的任务上获得更多指导时，它会调用 `AskUserQuestion` 工具。这会触发您的 `canUseTool` 回调，其中 `toolName` 设置为 `AskUserQuestion`。输入包含 Claude 的问题作为多选选项，您向用户显示这些问题并返回他们的选择。

> [!TIP]
>
> 澄清问题在 [`plan` 模式](https://code.claude.com/docs/zh-CN/agent-sdk/permissions#plan-mode-plan)中特别常见，其中 Claude 探索代码库并在提出计划前提出问题。这使 plan 模式非常适合交互式工作流，您希望 Claude 在进行更改前收集需求。

以下步骤显示如何处理澄清问题：

**1. 传递 canUseTool 回调**

在您的查询选项中传递 `canUseTool` 回调。默认情况下，`AskUserQuestion` 可用。如果您指定 `tools` 数组来限制 Claude 的功能（例如，仅具有 `Read`、`Glob` 和 `Grep` 的只读代理），请在该数组中包含 `AskUserQuestion`。否则，Claude 将无法提出澄清问题：

```python Python theme={null}
async for message in query(
    prompt="Analyze this codebase",
    options=ClaudeAgentOptions(
        # 在您的工具列表中包含 AskUserQuestion
        tools=["Read", "Glob", "Grep", "AskUserQuestion"],
        can_use_tool=can_use_tool,
    ),
):
    print(message)
```

```typescript TypeScript theme={null}
for await (const message of query({
  prompt: "Analyze this codebase",
  options: {
    // 在您的工具列表中包含 AskUserQuestion
    tools: ["Read", "Glob", "Grep", "AskUserQuestion"],
    canUseTool: async (toolName, input) => {
      // 批准每次调用的占位实现。"检测 AskUserQuestion"步骤会替换它。
      return { behavior: "allow", updatedInput: input };
    }
  }
})) {
  console.log(message);
}
```

**2. 检测 AskUserQuestion**

在您的回调中，检查 `toolName` 是否等于 `AskUserQuestion` 以不同方式处理它与其他工具：

```python Python theme={null}
async def can_use_tool(tool_name: str, input_data: dict, context):
    if tool_name == "AskUserQuestion":
        # 您从用户收集答案的实现
        return await handle_clarifying_questions(input_data)
    # 正常处理其他工具
    return await prompt_for_approval(tool_name, input_data)
```

```typescript TypeScript theme={null}
canUseTool: async (toolName, input) => {
  if (toolName === "AskUserQuestion") {
    // 您从用户收集答案的实现
    return handleClarifyingQuestions(input);
  }
  // 正常处理其他工具
  return promptForApproval(toolName, input);
};
```

**3. 解析问题输入**

输入包含 Claude 在 `questions` 数组中的问题。每个问题都有 `question`（要显示的文本）、`options`（选择）和 `multiSelect`（是否允许多个选择）：

```json theme={null}
{
  "questions": [
    {
      "question": "How should I format the output?",
      "header": "Format",
      "options": [
        { "label": "Summary", "description": "Brief overview" },
        { "label": "Detailed", "description": "Full explanation" }
      ],
      "multiSelect": false
    },
    {
      "question": "Which sections should I include?",
      "header": "Sections",
      "options": [
        { "label": "Introduction", "description": "Opening context" },
        { "label": "Conclusion", "description": "Final summary" }
      ],
      "multiSelect": true
    }
  ]
}
```

有关完整字段描述，请参阅[问题格式](#question-format)。

**4. 从用户收集答案**

向用户呈现问题并收集他们的选择。您如何执行此操作取决于您的应用程序：终端提示、Web 表单、移动对话框等。

**5. 将答案返回给 Claude**

将 `answers` 对象构建为记录，其中每个键是 `question` 文本，每个值是所选选项的 `label`：

| 来自问题对象 | 用作 |
| - | - |
| `question` 字段（例如 `"How should I format the output?"`） | 键 |
| 所选选项的 `label` 字段（例如 `"Summary"`） | 值 |

对于多选问题，传递标签数组或用 `", "` 连接它们。如果您[支持自由文本输入](#support-free-text-input)，使用用户的自定义文本作为值。

```python Python theme={null}
return PermissionResultAllow(
    updated_input={
        "questions": input_data.get("questions", []),
        "answers": {
            "How should I format the output?": "Summary",
            "Which sections should I include?": ["Introduction", "Conclusion"],
        },
    }
)
```

```typescript TypeScript theme={null}
return {
  behavior: "allow",
  updatedInput: {
    questions: input.questions,
    answers: {
      "How should I format the output?": "Summary",
      "Which sections should I include?": "Introduction, Conclusion"
    }
  }
};
```

### 问题格式

输入包含 Claude 在 `questions` 数组中生成的问题。每个问题都有这些字段：

| 字段 | 描述 |
| - | - |
| `question` | 要显示的完整问题文本 |
| `header` | 问题的短标签（最多 12 个字符） |
| `options` | 2-4 个选择的数组，每个都有 `label` 和 `description`。TypeScript：可选 `preview`。请参阅[选项预览](#option-previews-typescript)。 |
| `multiSelect` | 如果为 `true`，用户可以选择多个选项 |

您的回调接收的结构：

```json theme={null}
{
  "questions": [
    {
      "question": "How should I format the output?",
      "header": "Format",
      "options": [
        { "label": "Summary", "description": "Brief overview of key points" },
        { "label": "Detailed", "description": "Full explanation with examples" }
      ],
      "multiSelect": false
    }
  ]
}
```

#### 选项预览 (TypeScript)

`toolConfig.askUserQuestion.previewFormat` 向每个选项添加 `preview` 字段，以便您的应用可以在标签旁显示视觉模型。没有此设置，Claude 不会生成预览，该字段不存在。

| `previewFormat` | `preview` 包含 |
| :- | :- |
| 未设置（默认） | 字段不存在。Claude 不会生成预览。 |
| `"markdown"` | ASCII 艺术和围栏代码块 |
| `"html"` | 样式的 `<div>` 片段（SDK 在您的回调运行前拒绝 `<script>`、`<style>` 和 `<!DOCTYPE>`） |

该格式适用于会话中的所有问题。Claude 在视觉比较有帮助的选项上包含 `preview`（布局选择、配色方案），并在不会的地方省略它（是/否确认、仅文本选择）。在呈现前检查 `undefined`。

```typescript theme={null}
import { query } from "@anthropic-ai/claude-agent-sdk";

for await (const message of query({
  prompt: "Help me choose a card layout",
  options: {
    toolConfig: {
      askUserQuestion: { previewFormat: "html" }
    },
    canUseTool: async (toolName, input) => {
      // input.questions[].options[].preview 是 HTML 字符串或 undefined
      return { behavior: "allow", updatedInput: input };
    }
  }
})) {
  // ...
}
```

带有 HTML 预览的选项：

```json theme={null}
{
  "label": "Compact",
  "description": "Title and metric value only",
  "preview": "<div style=\"padding:12px;border:1px solid #ddd;border-radius:8px\"><div style=\"font-size:12px;color:#666\">Active users</div><div style=\"font-size:28px;font-weight:600\">1,284</div></div>"
}
```

### 响应格式

返回 `answers` 对象，将每个问题的 `question` 字段映射到所选选项的 `label`：

| 字段 | 描述 |
| - | - |
| `questions` | 传递原始问题数组（工具处理需要） |
| `answers` | 对象，其中键是问题文本，值是所选标签 |
| `response` | 可选的自由格式回复，用户输入的而不是回答结构化问题 |

对于多选问题，传递标签数组或用 `", "` 连接它们。对于按问题的自由文本，例如"其他"选项，将用户的文本放在 `answers[question]` 中，如[支持自由文本输入](#support-free-text-input)中所示。仅当您的 UI 让用户关闭问题卡并输入不是任何特定问题答案的一般回复时，才设置 `response`。当设置 `response` 时，Claude 会收到"用户回复：…"而不是按问题答案列表。

```jsonc theme={null}
{
  "questions": [
    // ...
  ],
  "answers": {
    "How should I format the output?": "Summary",
    "Which sections should I include?": ["Introduction", "Conclusion"]
  }
}
```

#### 支持自由文本输入

Claude 的预定义选项并不总是涵盖用户想要的内容。要让用户输入自己的答案：

* 在 Claude 的选项后显示额外的"其他"选择，接受文本输入
* 使用用户的自定义文本作为答案值（不是单词"其他"）

有关完整实现，请参阅下面的[完整示例](#complete-example)。

### 完整示例

当 Claude 需要用户输入来继续时，它会提出澄清问题。例如，当被要求帮助为移动应用程序决定技术栈时，Claude 可能会询问跨平台与原生、后端偏好或目标平台。这些问题帮助 Claude 做出与用户偏好相匹配的决定，而不是猜测。

此示例在终端应用程序中处理这些问题。以下是每个步骤发生的情况：

1. **路由请求**：`canUseTool` 回调检查工具名称是否为 `"AskUserQuestion"` 并路由到专用处理程序
2. **显示问题**：处理程序循环遍历 `questions` 数组并打印每个问题及编号选项
3. **收集输入**：用户可以输入数字来选择选项，或直接输入自由文本（例如"jquery"、"i don't know"）
4. **映射答案**：代码检查输入是数字（使用选项的标签）还是自由文本（使用文本直接）
5. **返回给 Claude**：响应包括原始 `questions` 数组和 `answers` 映射

将 TypeScript 版本保存为 `ask.ts` 并使用 `npx tsx ask.ts` 运行它，或将 Python 版本保存为 `ask.py` 并使用 `python ask.py` 运行它。

```python Python theme={null}
import asyncio

from claude_agent_sdk import ClaudeAgentOptions, ResultMessage, query
from claude_agent_sdk.types import HookMatcher, PermissionResultAllow


def parse_response(response: str, options: list) -> str:
    """将用户输入解析为选项编号或自由文本。"""
    try:
        indices = [int(s.strip()) - 1 for s in response.split(",")]
        labels = [options[i]["label"] for i in indices if 0 <= i < len(options)]
        return ", ".join(labels) if labels else response
    except ValueError:
        return response


async def handle_ask_user_question(input_data: dict) -> PermissionResultAllow:
    """显示 Claude 的问题并收集用户答案。"""
    answers = {}

    for q in input_data.get("questions", []):
        print(f"\n{q['header']}: {q['question']}")

        options = q["options"]
        for i, opt in enumerate(options):
            print(f"  {i + 1}. {opt['label']} - {opt['description']}")
        if q.get("multiSelect"):
            print("  (Enter numbers separated by commas, or type your own answer)")
        else:
            print("  (Enter a number, or type your own answer)")

        response = input("Your choice: ").strip()
        answers[q["question"]] = parse_response(response, options)

    return PermissionResultAllow(
        updated_input={
            "questions": input_data.get("questions", []),
            "answers": answers,
        }
    )


async def can_use_tool(
    tool_name: str, input_data: dict, context
) -> PermissionResultAllow:
    # 将 AskUserQuestion 路由到我们的问题处理程序
    if tool_name == "AskUserQuestion":
        return await handle_ask_user_question(input_data)
    # 为此示例自动批准其他工具
    return PermissionResultAllow(updated_input=input_data)


async def prompt_stream():
    yield {
        "type": "user",
        "message": {
            "role": "user",
            "content": "Help me decide on the tech stack for a new mobile app",
        },
    }


# 必需的解决方法：虚拟 hook 保持流打开以供 can_use_tool 使用
async def dummy_hook(input_data, tool_use_id, context):
    return {"continue_": True}


async def main():
    async for message in query(
        prompt=prompt_stream(),
        options=ClaudeAgentOptions(
            can_use_tool=can_use_tool,
            hooks={"PreToolUse": [HookMatcher(matcher=None, hooks=[dummy_hook])]},
        ),
    ):
        if isinstance(message, ResultMessage) and message.subtype == "success":
            print(message.result)


asyncio.run(main())
```

```typescript TypeScript theme={null}
import { query } from "@anthropic-ai/claude-agent-sdk";
import type { PermissionResult } from "@anthropic-ai/claude-agent-sdk";
import * as readline from "readline/promises";

// 帮助程序在终端中提示用户输入
async function prompt(question: string): Promise<string> {
  const rl = readline.createInterface({ input: process.stdin, output: process.stdout });
  const answer = await rl.question(question);
  rl.close();
  return answer;
}

// 将用户输入解析为选项编号或自由文本
function parseResponse(response: string, options: any[]): string {
  const indices = response.split(",").map((s) => parseInt(s.trim()) - 1);
  const labels = indices
    .filter((i) => !isNaN(i) && i >= 0 && i < options.length)
    .map((i) => options[i].label);
  return labels.length > 0 ? labels.join(", ") : response;
}

// 显示 Claude 的问题并收集用户答案
async function handleAskUserQuestion(input: any): Promise<PermissionResult> {
  const answers: Record<string, string> = {};

  for (const q of input.questions) {
    console.log(`\n${q.header}: ${q.question}`);

    const options = q.options;
    options.forEach((opt: any, i: number) => {
      console.log(`  ${i + 1}. ${opt.label} - ${opt.description}`);
    });
    if (q.multiSelect) {
      console.log("  (Enter numbers separated by commas, or type your own answer)");
    } else {
      console.log("  (Enter a number, or type your own answer)");
    }

    const response = (await prompt("Your choice: ")).trim();
    answers[q.question] = parseResponse(response, options);
  }

  // 将答案返回给 Claude（必须包括原始问题）
  return {
    behavior: "allow",
    updatedInput: { questions: input.questions, answers }
  };
}

async function main() {
  for await (const message of query({
    prompt: "Help me decide on the tech stack for a new mobile app",
    options: {
      canUseTool: async (toolName, input) => {
        // 将 AskUserQuestion 路由到我们的问题处理程序
        if (toolName === "AskUserQuestion") {
          return handleAskUserQuestion(input);
        }
        // 为此示例自动批准其他工具
        return { behavior: "allow", updatedInput: input };
      }
    }
  })) {
    if ("result" in message) console.log(message.result);
  }
}

main();
```

## 限制

* **子代理**：`AskUserQuestion` 目前在通过 Agent 工具生成的子代理中不可用
* **问题限制**：每个 `AskUserQuestion` 调用支持 1-4 个问题，每个 2-4 个选项

## 获取用户输入的其他方式

`canUseTool` 回调和 `AskUserQuestion` 工具涵盖了大多数批准和澄清场景，但 SDK 提供了其他从用户获取输入的方式：

### 流输入

当您需要以下情况时，使用[流输入](https://code.claude.com/docs/zh-CN/agent-sdk/streaming-vs-single-mode.md)：

* **在任务中断代理**：在 Claude 工作时发送取消信号或改变方向
* **提供额外上下文**：添加 Claude 需要的信息而无需等待它提出问题
* **构建聊天界面**：让用户在长时间运行的操作期间发送后续消息

流输入非常适合对话式 UI，用户在整个执行过程中与代理交互，而不仅仅在批准检查点。

### 自定义工具

当您需要以下情况时，使用[自定义工具](https://code.claude.com/docs/zh-CN/agent-sdk/custom-tools.md)：

* **收集结构化输入**：构建超越 `AskUserQuestion` 多选格式的表单、向导或多步工作流
* **集成外部批准系统**：连接到现有的票务、工作流或批准平台
* **实现特定领域的交互**：创建针对您的应用程序需求定制的工具，如代码审查界面或部署清单

自定义工具让您完全控制交互，但需要比使用内置 `canUseTool` 回调更多的实现工作。

## 相关资源

* [配置权限](https://code.claude.com/docs/zh-CN/agent-sdk/permissions.md)：设置权限模式和规则
* [使用 hooks 控制执行](https://code.claude.com/docs/zh-CN/agent-sdk/hooks.md)：在代理生命周期的关键点运行自定义代码
* [TypeScript SDK 参考](https://code.claude.com/docs/zh-CN/agent-sdk/typescript.md#canusetool)：完整的 canUseTool API 文档
