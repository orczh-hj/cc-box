> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# 使用 hooks 拦截和控制代理行为

> 在代理执行的关键点使用 hooks 拦截和自定义代理行为

Hooks 是回调函数，用于响应代理事件（如工具被调用、会话启动或执行停止）运行您的代码。使用 hooks，您可以：

* **阻止危险操作**在执行前进行，如破坏性 shell 命令或未授权的文件访问
* **记录和审计**工具调用，用于合规性、调试或分析
* **转换输入和输出**以清理数据、注入凭证或重定向文件路径
* **要求人工批准**敏感操作，如数据库写入或 API 调用
* **跟踪会话生命周期**以管理状态、清理资源或发送通知

## hook 如何工作

**1. 事件触发**

Agent 执行期间发生某事，SDK 触发事件：工具即将被调用（`PreToolUse`）、工具返回结果（`PostToolUse`）、子代理启动或停止、Agent 空闲或执行完成。请参阅[完整事件列表](#available-hooks)。

**2. SDK 收集已注册的 hook**

SDK 检查为该事件类型注册的 hook。这包括您在 `options.hooks` 中传递的回调 hook 和来自设置文件的 shell 命令 hook，当相应的 [`settingSources`](https://code.claude.com/docs/zh-CN/agent-sdk/typescript#settingsource) 或 [`setting_sources`](https://code.claude.com/docs/zh-CN/agent-sdk/python#settingsource) 条目启用时（默认 `query()` 选项就是这样）。

**3. 匹配器过滤哪些 hook 运行**

如果 hook 有 [`matcher`](#matchers) 模式（如 `"Write|Edit"`），SDK 会针对事件的目标（例如工具名称）测试它。没有匹配器的 hook 对该类型的每个事件都运行。

**4. 回调函数执行**

每个匹配的 hook 的[回调函数](#callback-functions)接收有关正在发生的事情的输入：工具名称、其参数、会话 ID 和其他事件特定的详细信息。

**5. 您的回调返回决定**

执行任何操作（日志记录、API 调用、验证）后，您的回调返回一个[输出对象](#outputs)，告诉 Agent 该做什么：允许操作、阻止它、修改输入或将上下文注入到对话中。

此示例注册一个 `PreToolUse` hook（步骤 1），带有 `"Write|Edit"` 匹配器（步骤 3），因此回调会针对 Write 和 Edit 工具触发。触发时，回调接收工具的输入（步骤 4），检查文件路径是否针对 `.env` 文件，并返回 `permissionDecision: "deny"` 以阻止操作（步骤 5）：

```python Python theme={null}
import asyncio
from claude_agent_sdk import (
    AssistantMessage,
    ClaudeSDKClient,
    ClaudeAgentOptions,
    HookMatcher,
    ResultMessage,
)


# 定义一个接收工具调用详细信息的 hook 回调
async def protect_env_files(input_data, tool_use_id, context):
    # 从工具的输入参数中提取文件路径
    file_path = input_data["tool_input"].get("file_path", "")
    file_name = file_path.split("/")[-1]

    # 如果针对 .env 文件，阻止操作
    if file_name.lower() == ".env":
        return {
            "hookSpecificOutput": {
                "hookEventName": input_data["hook_event_name"],
                "permissionDecision": "deny",
                "permissionDecisionReason": "Cannot modify .env files",
            }
        }

    # 返回空对象以允许操作
    return {}


async def main():
    options = ClaudeAgentOptions(
        hooks={
            # 为 PreToolUse 事件注册 hook
            # 匹配器仅过滤 Write 和 Edit 工具调用
            "PreToolUse": [HookMatcher(matcher="Write|Edit", hooks=[protect_env_files])]
        }
    )

    async with ClaudeSDKClient(options=options) as client:
        await client.query("使用标准本地开发数据库配置创建 .env 文件")
        async for message in client.receive_response():
            # 过滤助手和结果消息
            if isinstance(message, (AssistantMessage, ResultMessage)):
                print(message)


asyncio.run(main())
```

```typescript TypeScript theme={null}
import { query, HookCallback, PreToolUseHookInput } from "@anthropic-ai/claude-agent-sdk";

// 使用 HookCallback 类型定义 hook 回调
const protectEnvFiles: HookCallback = async (input, toolUseID, { signal }) => {
  // 将输入转换为特定 hook 类型以获得类型安全
  const preInput = input as PreToolUseHookInput;

  // 转换 tool_input 以访问其属性（在 SDK 中类型为 unknown）
  const toolInput = preInput.tool_input as Record<string, unknown>;
  const filePath = toolInput?.file_path as string;
  const fileName = filePath?.split("/").pop();

  // 如果针对 .env 文件，阻止操作
  if (fileName?.toLowerCase() === ".env") {
    return {
      hookSpecificOutput: {
        hookEventName: preInput.hook_event_name,
        permissionDecision: "deny",
        permissionDecisionReason: "Cannot modify .env files"
      }
    };
  }

  // 返回空对象以允许操作
  return {};
};

for await (const message of query({
  prompt: "使用标准本地开发数据库配置创建 .env 文件",
  options: {
    hooks: {
      // 为 PreToolUse 事件注册 hook
      // 匹配器仅过滤 Write 和 Edit 工具调用
      PreToolUse: [{ matcher: "Write|Edit", hooks: [protectEnvFiles] }]
    }
  }
})) {
  // 过滤助手和结果消息
  if (message.type === "assistant" || message.type === "result") {
    console.log(message);
  }
}
```

该 hook 会拒绝 `file_path` 以 `/.env` 结尾的 Write 或 Edit 调用，不区分字母大小写。在 Windows 上，`file_path` 使用反斜杠，例如 `C:\project\.env`，此时该 hook 会允许对 `C:\project\.env` 的 Write 或 Edit 调用。

> [!WARNING]
>
> `Write|Edit` 匹配器将此 hook 限制为仅作用于 Write 和 Edit 工具。如果 Claude 通过 Bash 命令写入 `.env`，该 hook 不会运行。要在[沙箱运行](https://code.claude.com/docs/zh-CN/sandboxing.md)的情况下阻止 Bash 写入某个文件，请参阅 [Python](https://code.claude.com/docs/zh-CN/agent-sdk/python.md#sandbox-configuration) 或 [TypeScript](https://code.claude.com/docs/zh-CN/agent-sdk/typescript.md#sandbox-configuration) 的沙箱配置。

## 可用的 hooks

SDK 为代理执行的不同阶段提供 hooks。某些 hooks 在两个 SDK 中都可用，而其他 hooks 仅在 TypeScript 中可用。

| Hook 事件 | Python SDK | TypeScript SDK | 触发条件 | 示例用例 |
| - | - | - | - | - |
| `PreToolUse` | 是 | 是 | 工具调用请求（可以阻止或修改） | 阻止危险的 shell 命令 |
| `PostToolUse` | 是 | 是 | 工具执行结果 | 将文件更改记录到审计跟踪 |
| `PostToolUseFailure` | 是 | 是 | 工具执行失败 | 处理或记录工具错误 |
| `PostToolBatch` | 否 | 是 | 一整批工具调用解决，每批一次，在下一个模型调用之前 | 为整个批次注入约定 |
| [`UserPromptSubmit`](https://code.claude.com/docs/zh-CN/hooks#userpromptsubmit) | 是 | 是 | 提交提示词，包括 Claude Code 自行发起的轮次 | 将额外上下文注入到提示词中 |
| [`UserPromptExpansion`](https://code.claude.com/docs/zh-CN/hooks#userpromptexpansion) | 否 | 是 | 用户输入的命令或 MCP 提示在到达 Claude 之前扩展为提示。当 Claude 自己调用 skill 时不会触发 | 阻止命令直接调用或在输入 skill 时添加上下文 |
| `MessageDisplay` | 否 | 是 | 助手消息包含文本完成，每条消息一次，包含完整消息文本 | 编辑或重新格式化显示的文本而不改变记录 |
| `Stop` | 是 | 是 | 代理执行停止 | 在退出前保存会话状态 |
| `StopFailure` | 否 | 是 | 转换以 API 错误结束而不是正常停止 | 记录失败或发送警报 |
| `SubagentStart` | 是 | 是 | 子代理初始化 | 跟踪并行任务生成 |
| `SubagentStop` | 是 | 是 | 子代理完成 | 聚合来自并行任务的结果 |
| `PreCompact` | 是 | 是 | 对话压缩请求 | 在总结前存档完整记录 |
| `PostCompact` | 否 | 是 | 对话压缩完成 | 记录生成的摘要 |
| [`PreModelSwitch`](https://code.claude.com/docs/zh-CN/hooks#premodelswitch) | 否 | 是 | 请求的模型切换，在发生之前（可以阻止） | 阻止切换到特定模型 |
| [`PostModelSwitch`](https://code.claude.com/docs/zh-CN/hooks#postmodelswitch) | 否 | 是 | 会话的模型更改，包括自动回退 | 为新模型提供 Claude 特定的指导 |
| `PermissionRequest` | 是 | 是 | 工具调用需要权限决定 | 自定义权限处理 |
| `PermissionDenied` | 否 | 是 | 自动模式拒绝工具调用，包括没有分类器判决的拒绝 | 记录拒绝，或告诉模型它可能重试；Claude Code 对无判决拒绝忽略 `retry: true`。参见 [PermissionDenied](https://code.claude.com/docs/zh-CN/hooks.md#permissiondenied) |
| `SessionStart` | 否 | 是 | 会话初始化 | 初始化日志记录和遥测 |
| `SessionEnd` | 否 | 是 | 会话终止 | 清理临时资源 |
| `Notification` | 是 | 是 | 代理状态消息 | 将代理状态更新发送到 Slack 或 PagerDuty |
| `Setup` | 否 | 是 | 会话设置/维护 | 运行初始化任务 |
| `TeammateIdle` | 否 | 是 | 队友变为空闲 | 重新分配工作或通知 |
| `TaskCreated` | 否 | 是 | 通过 `TaskCreate` 工具创建任务 | 强制执行任务命名约定 |
| [`TaskCompleted`](https://code.claude.com/docs/zh-CN/hooks#taskcompleted) | 否 | 是 | 任务标记为已完成 | 在任务关闭前要求通过测试 |
| `Elicitation` | 否 | 是 | MCP 服务器在任务中期请求用户输入 | 以编程方式响应 MCP 输入请求 |
| `ElicitationResult` | 否 | 是 | 用户响应 MCP 引出 | 在返回到服务器之前修改或阻止响应 |
| `ConfigChange` | 否 | 是 | 配置文件更改 | 动态重新加载设置 |
| `InstructionsLoaded` | 否 | 是 | `CLAUDE.md` 或规则文件加载到上下文中 | 审计加载哪些指令文件 |
| `WorktreeCreate` | 否 | 是 | Git worktree 创建 | 跟踪隔离的工作区 |
| `WorktreeRemove` | 否 | 是 | 由 `WorktreeCreate` hook 创建的 worktree 正在被移除 | 清理工作区资源 |
| `CwdChanged` | 否 | 是 | 会话期间工作目录更改 | 按目录重新加载环境变量 |
| `FileChanged` | 否 | 是 | 监视的文件被修改、创建或删除 | 项目文件更改时重新加载配置 |
| `DirectoryAdded` | 否 | 是 | 会话期间添加工作目录 | 为中途添加的存储库安装依赖项 |

## 配置 hook

要配置 hook，请在您的 Agent 选项的 `hooks` 字段中传递它（Python 中的 `ClaudeAgentOptions`，TypeScript 中的 `options` 对象）。此代码段假设您已经定义了一个 hook 回调，例如上面示例中 Python 中的 `protect_env_files` 或 TypeScript 中的 `protectEnvFiles`：

```python Python theme={null}
options = ClaudeAgentOptions(
    hooks={"PreToolUse": [HookMatcher(matcher="Bash", hooks=[my_callback])]}
)

async with ClaudeSDKClient(options=options) as client:
    await client.query("Your prompt")
    async for message in client.receive_response():
        print(message)
```

```typescript TypeScript theme={null}
for await (const message of query({
  prompt: "Your prompt",
  options: {
    hooks: {
      PreToolUse: [{ matcher: "Bash", hooks: [myCallback] }]
    }
  }
})) {
  console.log(message);
}
```

`hooks` 选项是一个字典（Python）或对象（TypeScript），其中：

* **键**是 [hook 事件名称](#available-hooks)（例如 `'PreToolUse'`、`'PostToolUse'`、`'Stop'`）
* **值**是[匹配器](#matchers)数组，每个包含可选的过滤模式和您的[回调函数](#callback-functions)

### 匹配器

使用匹配器来过滤您的回调何时触发。`matcher` 字段根据 hook 事件类型匹配不同的值。例如，基于工具的 hook 匹配工具名称，而 `Notification` hook 匹配通知类型。

SDK 匹配器遵循与[设置文件中的匹配器](https://code.claude.com/docs/zh-CN/hooks.md#matcher-patterns)相同的规则。该部分记录了精确字符串和正则表达式评估路径、它们的版本要求以及每个事件类型的匹配器值。

| 选项 | 类型 | 默认值 | 描述 |
| - | - | - | - |
| `matcher` | `string` | `undefined` | 针对事件的过滤字段匹配的模式，遵循[设置文件中匹配器的规则](https://code.claude.com/docs/zh-CN/hooks.md#matcher-patterns)。对于工具 hook，该字段是工具名称。内置工具包括 `Bash`、`Read`、`Write`、`Edit`、`Glob`、`Grep`、`WebFetch`、`Agent` 等。请参阅[工具输入类型](https://code.claude.com/docs/zh-CN/agent-sdk/typescript.md#tool-input-types)以获取完整列表。MCP 工具使用模式 `mcp__<server>__<action>`，其中 `<server>` 是您在 `mcpServers` 配置中使用的键，其拼写方式遵循[工具命名约定](https://code.claude.com/docs/zh-CN/agent-sdk/mcp.md#tool-naming-convention)中的说明。 |
| `hooks` | `HookCallback[]` | - | 必需。当模式匹配时执行的回调函数数组 |
| `timeout` | `number` | `undefined` | 超时时间（秒）。省略时，Claude Code 应用[事件的默认超时](#hook-timeout)。您的 SDK 回调遵循 `command` hook 默认值 |

尽可能使用 `matcher` 模式来针对特定工具。带有 `'Bash'` 的匹配器仅对 Bash 命令运行，而省略模式会为事件的每次出现运行您的回调。

### 回调函数

#### 输入

每个 hook 回调接收三个参数：

* **输入数据：** 一个包含事件详细信息的类型化对象。每个 hook 类型都有自己的输入形状。例如，`PreToolUseHookInput` 包括 `tool_name` 和 `tool_input`，而 `NotificationHookInput` 包括 `message`。请参阅 [TypeScript](https://code.claude.com/docs/zh-CN/agent-sdk/typescript.md#hookinput) 和 [Python](https://code.claude.com/docs/zh-CN/agent-sdk/python.md#hookinput) SDK 参考中的完整类型定义。
  * 所有 hook 输入共享 `session_id`、`cwd` 和 `hook_event_name`。
  * 当 hook 在子代理内触发时，`agent_id` 和 `agent_type` 被填充。在 TypeScript 中，这些在基础 hook 输入上，对所有 hook 类型都可用。在 Python 中，它们是 `PreToolUse`、`PostToolUse`、`PostToolUseFailure` 和 `PermissionRequest` 上的可选字段，以及 `SubagentStart` 和 `SubagentStop` 上的必需字段。
* **工具使用 ID**（`str | None` / `string | undefined`）：关联同一工具调用的 `PreToolUse` 和 `PostToolUse` 事件。
* **上下文：** 在 TypeScript 中，包含用于取消的 `signal` 属性（`AbortSignal`）。在 Python 中，此参数保留供将来使用。

#### 输出

您的回调返回一个具有两类字段的对象：

* **顶级字段**在每个事件上被接受：`systemMessage` 向用户显示消息，`continue`（Python 中的 `continue_`）确定 Agent 在此 hook 后是否继续运行。某些事件会丢弃它们或将它们传递到其他地方。hooks 页面上每个[事件的部分](https://code.claude.com/docs/zh-CN/hooks.md#hook-events)说明了它们的去向。
* **`hookSpecificOutput`** 控制当前操作。内部的字段取决于 hook 事件类型：
  * 对于 `PreToolUse` hook，这是您设置 `permissionDecision`（`"allow"`、`"deny"`、`"ask"` 或 `"defer"`）、`permissionDecisionReason` 和 `updatedInput` 的地方。如果您返回 `"defer"`，该轮次将以一条 `stop_reason` 为 `"tool_deferred"` 的结果消息结束，以便您可以[稍后恢复该调用](https://code.claude.com/docs/zh-CN/hooks.md#defer-a-tool-call-for-later)。
  * 对于 `PostToolUse` hook，您可以设置 `additionalContext` 以将信息附加到工具结果。要在 Claude 看到之前替换工具的输出，请设置 `updatedToolOutput`，这适用于两个 SDK 中的任何工具。较旧的 `updatedMCPToolOutput` 字段仅替换 MCP 工具输出。
  * 在 TypeScript SDK 中，`PostToolUse` 回调也可以返回 `classifierContext`，这是关于工具调用结果的简短说明，用于[自动模式](https://code.claude.com/docs/zh-CN/permission-modes.md#eliminate-prompts-with-auto-mode)权限分类器。因为您的回调在您的应用程序自己的进程中运行，分类器可能会将您在说明中转达的用户声明视为用户意图。该字段需要 TypeScript Agent SDK v0.3.236 或更高版本。[为自动模式分类器注释结果](https://code.claude.com/docs/zh-CN/hooks.md#annotate-a-result-for-the-auto-mode-classifier)涵盖了长度上限、仅同步规则以及不要在说明中放入的内容。

返回 `{}` 以允许操作而不进行更改。SDK 回调 hook 使用与 [Claude Code shell 命令 hook](https://code.claude.com/docs/zh-CN/hooks.md#json-output) 相同的 JSON 输出格式，其中记录了每个字段和事件特定的选项。对于 SDK 类型定义，请参阅 [TypeScript](https://code.claude.com/docs/zh-CN/agent-sdk/typescript.md#synchookjsonoutput) 和 [Python](https://code.claude.com/docs/zh-CN/agent-sdk/python.md#synchookjsonoutput) SDK 参考。

> [!NOTE]
>
> 当多个 `PreToolUse` hook 返回不同的[决策](https://code.claude.com/docs/zh-CN/hooks.md#pretooluse-decision-control)时，`deny` 优先于 `defer`，`defer` 优先于 `ask`，`ask` 优先于 `allow`。

#### 异步输出

默认情况下，Agent 在您的 hook 返回前等待。如果您的 hook 执行副作用，例如日志记录或发送 webhook，并且不需要影响 Agent 的行为，您可以改为返回异步输出。这告诉 Agent 立即继续，而不等待 hook 完成。在此代码段中，Python 中的 `send_to_logging_service` 和 TypeScript 中的 `sendToLoggingService` 代表您定义的任何日志记录函数：

```python Python theme={null}
async def async_hook(input_data, tool_use_id, context):
    # 启动后台任务，然后立即返回
    asyncio.create_task(send_to_logging_service(input_data))
    return {"async_": True, "asyncTimeout": 30000}
```

```typescript TypeScript theme={null}
const asyncHook: HookCallback = async (input, toolUseID, { signal }) => {
  // 启动后台任务，然后立即返回
  sendToLoggingService(input).catch(console.error);
  return { async: true, asyncTimeout: 30000 };
};
```

| 字段 | 类型 | 描述 |
| - | - | - |
| `async` | `true` | 表示异步模式。Agent 继续而不等待。在 Python 中，使用 `async_` 以避免保留关键字。 |
| `asyncTimeout` | `number` | 后台操作的可选超时时间（毫秒） |

> [!NOTE]
>
> 异步输出无法阻止、修改或将上下文注入到操作中，因为 Agent 已经继续。仅将它们用于日志记录、指标或通知等副作用。

## 示例

本节中的几个示例仅显示回调函数。要运行一个示例，请在选项的 `hooks` 字段中的匹配事件下注册回调，如 [配置 hooks](#configure-hooks) 中所示。

### 修改工具输入

此示例拦截 Write 工具调用并重写 `file_path` 参数以添加 `/sandbox` 前缀。回调返回带有修改路径的 `updatedInput` 和 `permissionDecision: 'allow'` 以自动批准重写的操作：

```python Python theme={null}
async def redirect_to_sandbox(input_data, tool_use_id, context):
    if input_data["hook_event_name"] != "PreToolUse":
        return {}

    if input_data["tool_name"] == "Write":
        original_path = input_data["tool_input"].get("file_path", "")
        return {
            "hookSpecificOutput": {
                "hookEventName": input_data["hook_event_name"],
                "permissionDecision": "allow",
                "updatedInput": {
                    **input_data["tool_input"],
                    "file_path": f"/sandbox{original_path}",
                },
            }
        }
    return {}
```

```typescript TypeScript theme={null}
const redirectToSandbox: HookCallback = async (input, toolUseID, { signal }) => {
  if (input.hook_event_name !== "PreToolUse") return {};

  const preInput = input as PreToolUseHookInput;
  const toolInput = preInput.tool_input as Record<string, unknown>;
  if (preInput.tool_name === "Write") {
    const originalPath = toolInput.file_path as string;
    return {
      hookSpecificOutput: {
        hookEventName: preInput.hook_event_name,
        permissionDecision: "allow",
        updatedInput: {
          ...toolInput,
          file_path: `/sandbox${originalPath}`
        }
      }
    };
  }
  return {};
};
```

> [!NOTE]
>
> 将 `updatedInput` 与 `permissionDecision: 'allow'` 配对以自动批准修改的输入，或使用 `permissionDecision: 'ask'` 将其显示给用户。如果省略 `permissionDecision`，修改的输入仍然适用并流经正常的权限评估。使用 `'defer'` 时，`updatedInput` 会被忽略。始终返回新对象而不是改变原始 `tool_input`。

要确认重定向，请将前缀设置为您可以写入的路径，例如 `./sandbox` 或 `/tmp/sandbox`（macOS 不允许创建根级 `/sandbox` 目录），然后要求代理写入文件：Write 工具在消息流中的结果会显示带有您的沙箱前缀的路径，而不是 Claude 请求的路径。

### 添加上下文并阻止工具

此示例拒绝 `file_path` 以 `/etc` 开头的工具调用，并向模型和用户解释原因：

* `permissionDecision: 'deny'` 停止工具调用。
* `permissionDecisionReason` 告诉模型原因。
* `systemMessage` 向用户显示发生了什么。

```python Python theme={null}
async def block_etc_writes(input_data, tool_use_id, context):
    file_path = input_data["tool_input"].get("file_path", "")

    if file_path.startswith("/etc"):
        return {
            # 顶级字段：显示给用户的消息
            "systemMessage": "Remember: system directories like /etc are protected.",
            # hookSpecificOutput：阻止操作
            "hookSpecificOutput": {
                "hookEventName": input_data["hook_event_name"],
                "permissionDecision": "deny",
                "permissionDecisionReason": "Writing to /etc is not allowed",
            },
        }
    return {}
```

```typescript TypeScript theme={null}
const blockEtcWrites: HookCallback = async (input, toolUseID, { signal }) => {
  const preInput = input as PreToolUseHookInput;
  const toolInput = preInput.tool_input as Record<string, unknown>;
  const filePath = toolInput?.file_path as string;

  if (filePath?.startsWith("/etc")) {
    return {
      // 顶级字段：显示给用户的消息
      systemMessage: "Remember: system directories like /etc are protected.",
      // hookSpecificOutput：阻止操作
      hookSpecificOutput: {
        hookEventName: preInput.hook_event_name,
        permissionDecision: "deny",
        permissionDecisionReason: "Writing to /etc is not allowed"
      }
    };
  }
  return {};
};
```

要确认阻止，请在 `PreToolUse` 下注册回调，使用 `Write|Edit` 匹配器，并要求代理在 `/etc` 下创建文件：Write 工具在消息流中的结果包含 `Writing to /etc is not allowed`，并且不会创建任何文件。

### 自动批准特定工具

默认情况下，Agent 可能在使用某些工具前请求权限。此示例通过返回 `permissionDecision: 'allow'` 自动批准只读文件系统工具（Read、Glob、Grep），让它们无需用户确认即可运行（从[网络路径](https://code.claude.com/docs/zh-CN/permissions.md#network-paths)读取的操作除外），同时让所有其他工具受到正常权限检查：

```python Python theme={null}
async def auto_approve_read_only(input_data, tool_use_id, context):
    if input_data["hook_event_name"] != "PreToolUse":
        return {}

    read_only_tools = ["Read", "Glob", "Grep"]
    if input_data["tool_name"] in read_only_tools:
        return {
            "hookSpecificOutput": {
                "hookEventName": input_data["hook_event_name"],
                "permissionDecision": "allow",
                "permissionDecisionReason": "Read-only tool auto-approved",
            }
        }
    return {}
```

```typescript TypeScript theme={null}
const autoApproveReadOnly: HookCallback = async (input, toolUseID, { signal }) => {
  if (input.hook_event_name !== "PreToolUse") return {};

  const preInput = input as PreToolUseHookInput;
  const readOnlyTools = ["Read", "Glob", "Grep"];
  if (readOnlyTools.includes(preInput.tool_name)) {
    return {
      hookSpecificOutput: {
        hookEventName: preInput.hook_event_name,
        permissionDecision: "allow",
        permissionDecisionReason: "Read-only tool auto-approved"
      }
    };
  }
  return {};
};
```

### 注册多个 hooks

当事件触发时，匹配的 hook 并行运行。对于权限决策，以最严格的结果为准。任一 hook 返回的 `deny` 都会阻止工具调用。由于完成顺序是不确定的，请编写每个 hook 以独立行动，而不是依赖另一个 hook 已先运行。

下面的示例注册三个独立检查，每个检查都[省略了匹配器](https://code.claude.com/docs/zh-CN/hooks.md#matcher-patterns)。其中的 hook 名称，例如 Python 中的 `audit_logger` 或 TypeScript 中的 `auditLogger`，代表您定义的回调：

```python Python theme={null}
options = ClaudeAgentOptions(
    hooks={
        "PreToolUse": [
            HookMatcher(hooks=[authorization_check]),
            HookMatcher(hooks=[input_validator]),
            HookMatcher(hooks=[audit_logger]),
        ]
    }
)
```

```typescript TypeScript theme={null}
const options = {
  hooks: {
    PreToolUse: [
      { hooks: [authorizationCheck] },
      { hooks: [inputValidator] },
      { hooks: [auditLogger] }
    ]
  }
};
```

### 使用多工具匹配器过滤

使用多工具匹配器在相关工具间共享一个回调。此示例注册三个具有不同范围的匹配器，其中每个 hook 名称代表您定义的回调：

* 管道分隔的精确列表（`Write|Edit|NotebookEdit`）对 Write、Edit 和 NotebookEdit 工具触发 `file_security_hook`。
* 正则表达式（`^mcp__`）对任何名称以 `mcp__` 开头的 MCP 工具触发 `mcp_audit_hook`。
* 省略的匹配器触发 `global_logger`，不按工具名称过滤。

```python Python theme={null}
options = ClaudeAgentOptions(
    hooks={
        "PreToolUse": [
            # 匹配文件修改工具
            HookMatcher(matcher="Write|Edit|NotebookEdit", hooks=[file_security_hook]),
            # 匹配所有 MCP 工具
            HookMatcher(matcher="^mcp__", hooks=[mcp_audit_hook]),
            # 匹配所有内容（无匹配器）
            HookMatcher(hooks=[global_logger]),
        ]
    }
)
```

```typescript TypeScript theme={null}
const options = {
  hooks: {
    PreToolUse: [
      // 匹配文件修改工具
      { matcher: "Write|Edit|NotebookEdit", hooks: [fileSecurityHook] },

      // 匹配所有 MCP 工具
      { matcher: "^mcp__", hooks: [mcpAuditHook] },

      // 匹配所有内容（无匹配器）
      { hooks: [globalLogger] }
    ]
  }
};
```

### 跟踪子代理活动

使用 `SubagentStop` hooks 监控子代理何时完成其工作。请参阅 [TypeScript](https://code.claude.com/docs/zh-CN/agent-sdk/typescript.md#hookinput) 和 [Python](https://code.claude.com/docs/zh-CN/agent-sdk/python.md#hookinput) SDK 参考中的完整输入类型。此示例在每次子代理完成时记录摘要：

```python Python theme={null}
async def subagent_tracker(input_data, tool_use_id, context):
    # 子代理完成时记录子代理详细信息
    print(f"[SUBAGENT] Completed: {input_data['agent_id']}")
    print(f"  Transcript: {input_data['agent_transcript_path']}")
    print(f"  Tool use ID: {tool_use_id}")
    print(f"  Stop hook active: {input_data.get('stop_hook_active')}")
    return {}


options = ClaudeAgentOptions(
    hooks={"SubagentStop": [HookMatcher(hooks=[subagent_tracker])]}
)
```

```typescript TypeScript theme={null}
import { HookCallback, SubagentStopHookInput } from "@anthropic-ai/claude-agent-sdk";

const subagentTracker: HookCallback = async (input, toolUseID, { signal }) => {
  // 转换为 SubagentStopHookInput 以访问子代理特定字段
  const subInput = input as SubagentStopHookInput;

  // 子代理完成时记录子代理详细信息
  console.log(`[SUBAGENT] Completed: ${subInput.agent_id}`);
  console.log(`  Transcript: ${subInput.agent_transcript_path}`);
  console.log(`  Tool use ID: ${toolUseID}`);
  console.log(`  Stop hook active: ${subInput.stop_hook_active}`);
  return {};
};

const options = {
  hooks: {
    SubagentStop: [{ hooks: [subagentTracker] }]
  }
};
```

要确认 hook 触发，请注册回调并要求代理将小任务委派给子代理，例如列出当前目录中的文件：当子代理完成时，回调会打印 `[SUBAGENT] Completed:` 行，其中包含子代理的 ID 和脚本路径。

### 从 hooks 发出 HTTP 请求

Hooks 可以执行异步操作，如 HTTP 请求。在您的 hook 内捕获错误，而不是让它们传播。

此示例在每个工具完成后发送 webhook，记录哪个工具运行以及何时运行。hook 捕获来自失败 webhook 的错误：

```python Python theme={null}
import asyncio
import json
import urllib.request
from datetime import datetime


def _send_webhook(tool_name):
    """同步辅助函数，将工具使用数据 POST 到外部 webhook。"""
    data = json.dumps(
        {
            "tool": tool_name,
            "timestamp": datetime.now().isoformat(),
        }
    ).encode()
    req = urllib.request.Request(
        "https://api.example.com/webhook",
        data=data,
        headers={"Content-Type": "application/json"},
        method="POST",
    )
    urllib.request.urlopen(req)


async def webhook_notifier(input_data, tool_use_id, context):
    # 仅在工具完成后触发（PostToolUse），而不是之前
    if input_data["hook_event_name"] != "PostToolUse":
        return {}

    try:
        # 在线程中运行阻塞 HTTP 调用以避免阻塞事件循环
        await asyncio.to_thread(_send_webhook, input_data["tool_name"])
    except Exception as e:
        # 记录错误但不抛出
        print(f"Webhook request failed: {e}")

    return {}
```

```typescript TypeScript theme={null}
import { query, HookCallback, PostToolUseHookInput } from "@anthropic-ai/claude-agent-sdk";

const webhookNotifier: HookCallback = async (input, toolUseID, { signal }) => {
  // 仅在工具完成后触发（PostToolUse），而不是之前
  if (input.hook_event_name !== "PostToolUse") return {};

  try {
    await fetch("https://api.example.com/webhook", {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify({
        tool: (input as PostToolUseHookInput).tool_name,
        timestamp: new Date().toISOString()
      }),
      // 传递 signal 以便在 hook 超时时请求取消
      signal
    });
  } catch (error) {
    // 分别处理取消和其他错误
    if (error instanceof Error && error.name === "AbortError") {
      console.log("Webhook request cancelled");
    }
    // 不重新抛出
  }

  return {};
};

// 注册为 PostToolUse hook
for await (const message of query({
  prompt: "Refactor the auth module",
  options: {
    hooks: {
      PostToolUse: [{ hooks: [webhookNotifier] }]
    }
  }
})) {
  console.log(message);
}
```

要确认 hook 触发，请将 webhook URL 指向您可以监视的端点并发送使用工具的提示：hook 在每个工具完成后发送带有工具名称和时间戳的 POST。

### 将通知转发到 Slack

使用 `Notification` hooks 从代理接收系统通知并将其转发到外部服务。在 SDK 会话中，Claude Code 为以下通知类型运行此 hook：

* [`permission_prompt`](https://code.claude.com/docs/zh-CN/hooks#notification) 一旦权限请求在您的 [`canUseTool` 回调](https://code.claude.com/docs/zh-CN/agent-sdk/user-input) 上等待约六秒。需要 TypeScript Agent SDK v0.3.233 或更高版本，或 Python Agent SDK v0.2.139 或更高版本
* `elicitation_complete` 和 `elicitation_response` 用于用户提示引导流程
* `plugin_notification`，当 [mod](https://code.claude.com/docs/zh-CN/plugins/mods/api.md#show-something-without-starting-a-turn) 调用 `$.ui.notify` 时

Claude Code 从 SDK 会话不运行的交互式 UI 发出其他类型，例如 `idle_prompt`、`auth_success` 和 `elicitation_dialog`。

每个通知包括一个带有人类可读描述的 `message` 字段，以及可选的 `title`。

此示例将每个通知转发到 Slack 频道。它需要一个 [Slack 传入 webhook URL](https://docs.slack.dev/messaging/sending-messages-using-incoming-webhooks/)，您可以通过将应用添加到您的 Slack 工作区并启用传入 webhooks 来创建：

```python Python theme={null}
import asyncio
import json
import urllib.request

from claude_agent_sdk import ClaudeSDKClient, ClaudeAgentOptions, HookMatcher


def _send_slack_notification(message):
    """同步辅助函数，通过传入 webhook 向 Slack 发送消息。"""
    data = json.dumps({"text": f"Agent status: {message}"}).encode()
    req = urllib.request.Request(
        "https://hooks.slack.com/services/YOUR/WEBHOOK/URL",
        data=data,
        headers={"Content-Type": "application/json"},
        method="POST",
    )
    urllib.request.urlopen(req)


async def notification_handler(input_data, tool_use_id, context):
    try:
        # 在线程中运行阻塞 HTTP 调用以避免阻塞事件循环
        await asyncio.to_thread(_send_slack_notification, input_data.get("message", ""))
    except Exception as e:
        print(f"Failed to send notification: {e}")

    # 返回空对象。通知 hooks 不修改代理行为
    return {}


async def main():
    options = ClaudeAgentOptions(
        hooks={
            # 为通知事件注册 hook（不需要匹配器）
            "Notification": [HookMatcher(hooks=[notification_handler])],
        },
    )

    async with ClaudeSDKClient(options=options) as client:
        await client.query("Analyze this codebase")
        async for message in client.receive_response():
            print(message)


asyncio.run(main())
```

```typescript TypeScript theme={null}
import { query, HookCallback, NotificationHookInput } from "@anthropic-ai/claude-agent-sdk";

// 定义一个将通知发送到 Slack 的 hook 回调
const notificationHandler: HookCallback = async (input, toolUseID, { signal }) => {
  // 转换为 NotificationHookInput 以访问消息字段
  const notification = input as NotificationHookInput;

  try {
    // 将通知消息 POST 到 Slack 传入 webhook
    await fetch("https://hooks.slack.com/services/YOUR/WEBHOOK/URL", {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify({
        text: `Agent status: ${notification.message}`
      }),
      // 传递 signal 以便在 hook 超时时请求取消
      signal
    });
  } catch (error) {
    if (error instanceof Error && error.name === "AbortError") {
      console.log("Notification cancelled");
    } else {
      console.error("Failed to send notification:", error);
    }
  }

  // 返回空对象。通知 hooks 不修改代理行为
  return {};
};

// 为通知事件注册 hook（不需要匹配器）
for await (const message of query({
  prompt: "Analyze this codebase",
  options: {
    hooks: {
      Notification: [{ hooks: [notificationHandler] }]
    }
  }
})) {
  console.log(message);
}
```

当 `Notification` 事件触发时，hook 将通知的 `message` 发送到您的 webhook 目标的频道，前缀为 `Agent status:`。

## 修复常见问题

### Hook 未触发

* 验证 hook 事件名称正确且区分大小写（`PreToolUse`，而不是 `preToolUse`）
* 检查您的匹配器模式是否与工具名称完全匹配
* 确保 hook 在 `options.hooks` 中的正确事件类型下
* 对于支持匹配器的非工具 hook，如 `Notification` 和 `SubagentStop`，匹配器匹配不同的字段，而 `Stop` 完全忽略匹配器（请参阅[匹配器模式](https://code.claude.com/docs/zh-CN/hooks.md#matcher-patterns)）
* 当 Agent 达到 [`max_turns`](https://code.claude.com/docs/zh-CN/agent-sdk/python#claudeagentoptions) 限制时，hook 可能不会触发，因为会话在 hook 可以执行前结束

### 匹配器未按预期过滤

匹配器仅匹配工具名称，而不是文件路径或其他参数。要按文件路径过滤，请在您的 hook 内检查 `tool_input.file_path`：

```typescript theme={null}
const myHook: HookCallback = async (input, toolUseID, { signal }) => {
  const preInput = input as PreToolUseHookInput;
  const toolInput = preInput.tool_input as Record<string, unknown>;
  const filePath = toolInput?.file_path as string;
  if (!filePath?.endsWith(".md")) return {}; // 跳过非 markdown 文件
  // 处理 markdown 文件...
  return {};
};
```

### Hook 超时

Claude Code 运行每个回调时都有超时限制，您可以在其 `HookMatcher` 上使用 `timeout` 字段（以秒为单位）设置。当您未设置时，Claude Code 使用事件的默认值：大多数事件为 600 秒，`UserPromptSubmit`、`PreModelSwitch` 和 `PostModelSwitch` 为 30 秒，`MessageDisplay` 为 10 秒。Claude Code 在关闭期间运行 `SessionEnd` 回调时使用较短的 [SessionEnd 超时预算](https://code.claude.com/docs/zh-CN/hooks.md#sessionend-input)，默认为 1.5 秒。

当回调超过其超时时间时，Claude Code 会取消它并丢弃其输出，会话继续而不是挂起。接下来发生的情况取决于事件：

* `PreToolUse`: Claude Code 阻止该工具调用。如果其他所有 hook 均未返回任何内容或仅返回 `allow` 决定，工具结果会告知 Claude 该 hook 未在超时前响应，轮次继续。如果您的另一个 hook 已拒绝该调用，其拒绝仍会传达给 Claude：工具结果包含该 hook 的原因，超时文本可能跟在其后。

  在 v2.1.210 之前，Claude Code 将超时报告给 Claude 作为用户拒绝，这使得无人值守会话停止并等待输入。
* `PostToolUse` 和 `PostToolUseFailure`：Claude Code 保留工具结果，轮次继续。
* `UserPromptSubmit` 和 [`UserPromptExpansion`](https://code.claude.com/docs/zh-CN/hooks#userpromptexpansion)：Claude Code 使用指明 hook 和超时的消息阻止该提示词，会话继续。在 v2.1.208 之前，当这些事件上的回调超时时，Claude Code 以 `error_during_execution` 结束查询。
* `Stop` 和 `SubagentStop`：超时的回调计为不返回任何决定。Agent 或子代理停止，就像该回调已允许它一样，您在该事件上的其他 hook 的决定仍然适用。在 Claude Code v2.1.273 之前，超时的 `Stop` 或 `SubagentStop` 回调计为失败的 hook 运行，Claude Code 丢弃了您在该事件上的其他 hook 的决定。
* `SessionStart`：超时的回调计为不返回任何输出，会话继续使用您的其他 `SessionStart` hook 的输出。
* `PreModelSwitch`：Claude Code 阻止模型切换。未回答的 hook 尚未批准切换。
* 其他事件，如 `Notification`、`PreCompact` 和 `PostModelSwitch`：Claude Code 记录失败并继续。

主会话中 `Stop` 或 `SessionStart` 回调第一次超时时，Claude Code 还会向消息流添加一个 [`SDKInformationalMessage`](https://code.claude.com/docs/zh-CN/agent-sdk/typescript#sdkinformationalmessage)，说明驱动会话的应用未响应。当您的应用保持无响应时，后续超时不会重复该消息。

如果您在回调待处理时中断查询，Claude Code 会取消待处理的工具调用。在 v2.1.208 之前，如果您在待处理的 `PreToolUse` 回调期间中断，工具调用仍可能继续。

如果您的回调需要更多时间，请在其 `HookMatcher` 上设置更高的 `timeout`。在 TypeScript 中，使用第三个回调参数中的 `AbortSignal` 来在超时触发时优雅地处理取消。

### 工具意外被阻止

* 检查所有 `PreToolUse` hook 是否返回 `permissionDecision: 'deny'`
* 向您的 hook 添加日志记录以查看它们返回的 `permissionDecisionReason`
* 验证匹配器模式不会太宽泛：空匹配器匹配所有工具

### 修改的输入未应用

* 确保 `updatedInput` 在 `hookSpecificOutput` 内，而不是在顶级：

  ```typescript theme={null}
  return {
    hookSpecificOutput: {
      hookEventName: "PreToolUse",
      permissionDecision: "allow",
      updatedInput: { command: "new command" }
    }
  };
  ```

* 不要将 `updatedInput` 与 `permissionDecision: 'defer'` 配对，这会丢弃修改的输入。省略 `permissionDecision` 是可以的：修改的输入仍然通过正常权限评估应用。您也可以返回 `'allow'` 来自动批准修改的输入或 `'ask'` 来向用户显示以供批准

* 在 `hookSpecificOutput` 中包括 `hookEventName` 以识别输出针对的 hook 类型

### Python 中不可用会话 hook

`SessionStart` 和 `SessionEnd` 可以在 TypeScript 中注册为 SDK 回调 hook，但在 Python SDK 中不可用，因为其 `HookEvent` 类型省略了它们。在 Python 中，它们仅作为在设置文件（例如 `.claude/settings.json`）中定义的 [shell 命令 hook](https://code.claude.com/docs/zh-CN/hooks.md#hook-events) 可用。您的 SDK 应用程序加载哪些设置文件取决于 [`setting_sources`](https://code.claude.com/docs/zh-CN/agent-sdk/python#settingsource) 或 [`settingSources`](https://code.claude.com/docs/zh-CN/agent-sdk/typescript#settingsource)。如果您设置了该选项，请包括包含这些 hook 的设置源：

```python Python theme={null}
options = ClaudeAgentOptions(
    setting_sources=["project"],  # 加载 .claude/settings.json 包括 hooks
)
```

```typescript TypeScript theme={null}
const options = {
  settingSources: ["project"] // 加载 .claude/settings.json 包括 hooks
};
```

要改为运行初始化逻辑作为 Python SDK 回调，请使用 `client.receive_response()` 的第一条消息作为您的触发器。

### 子代理权限提示倍增

生成多个子代理时，每个子代理可能会单独请求其自身工具调用的权限。要避免重复提示，请使用 `PreToolUse` hook 自动批准特定工具，或配置权限规则，子代理[从父对话继承](https://code.claude.com/docs/zh-CN/sub-agents.md#permission-modes)这些规则。

### 子代理的递归 hook 循环

生成子代理的 `UserPromptSubmit` hook 如果这些子代理触发相同的 hook，可能会创建无限循环。要防止这种情况：

* 使用共享变量或会话状态来跟踪您是否已在子代理内
* 将 hook 限定为仅对顶级 Agent 会话运行

### systemMessage 未出现在输出中

`systemMessage` 字段向用户显示消息，而不是模型。在 Claude Code v2.1.227 或更高版本上，hook 的 `systemMessage` 可以在消息流中显示为 [`SDKInformationalMessage`](https://code.claude.com/docs/zh-CN/agent-sdk/typescript#sdkinformationalmessage)。它是否显示取决于事件。hooks 页面上每个[事件的部分](https://code.claude.com/docs/zh-CN/hooks.md#hook-events)说明输出如何显示。要改为将上下文传递给模型，请返回 [`additionalContext`](https://code.claude.com/docs/zh-CN/hooks#add-context-for-claude)。

在 v2.1.227 之前，SDK 仅在消息流中为 `SessionStart` 和 `Setup` hook 显示 hook 输出。对于任何其他事件，输出仅出现在 [`includeHookEvents`](https://code.claude.com/docs/zh-CN/agent-sdk/typescript#options)（Python 中为 `include_hook_events`）添加的生命周期事件中。该选项的条目涵盖每个 hook 事件产生的生命周期事件。

如果您需要可靠地将 hook 决定呈现给您的应用程序，请单独记录它们或使用专用输出通道。

## 相关资源

* [Claude Code hooks 参考](https://code.claude.com/docs/zh-CN/hooks.md)：完整的 JSON 输入/输出架构、事件文档和匹配器模式
* [Claude Code hooks 指南](https://code.claude.com/docs/zh-CN/hooks-guide.md)：shell 命令 hook 示例和演练
* [TypeScript SDK 参考](https://code.claude.com/docs/zh-CN/agent-sdk/typescript.md)：hook 类型、输入/输出定义和配置选项
* [Python SDK 参考](https://code.claude.com/docs/zh-CN/agent-sdk/python.md)：hook 类型、输入/输出定义和配置选项
* [权限](https://code.claude.com/docs/zh-CN/agent-sdk/permissions.md)：控制您的代理可以做什么
* [自定义工具](https://code.claude.com/docs/zh-CN/agent-sdk/custom-tools.md)：构建工具以扩展代理功能
