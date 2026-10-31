> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# 配置你的代理

> 配置 Agent SDK 会话：组合选项对象、设置模型、环境和限制，并找到每个功能选项的页面。

Agent SDK 会话从设置文件、环境变量和启动时传递的 `options` 对象读取配置。本页面展示如何组合 `options` 对象以及哪些设置文件和环境变量控制配置。

有关每个选项的类型和默认值，请参阅 [`Options`](https://code.claude.com/docs/zh-CN/agent-sdk/typescript#options)（TypeScript）和 [`ClaudeAgentOptions`](https://code.claude.com/docs/zh-CN/agent-sdk/python#claudeagentoptions)（Python）参考。

## 将选项传递给会话

每个 `query()` 调用都接受一个选项对象：TypeScript 中的 `Options`，Python 中的 `ClaudeAgentOptions`。每个字段都是可选的，使用无选项启动的会话以 SDK 的默认值运行。下面的示例配置了一个只读会话，用于总结项目的开放 TODO。对中读作 TypeScript / Python，其中拼写不同：

* **`model`**：选择模型
* **`allowedTools` / `allowed_tools`**：预先批准只读工具列表
* **`maxTurns` / `max_turns`**：限制轮次数
* **`cwd`**：设置工作目录

```typescript TypeScript theme={null}
import { query } from "@anthropic-ai/claude-agent-sdk";

for await (const message of query({
  prompt: "Summarize the open TODOs in this repo",
  options: {
    model: "claude-sonnet-5",
    allowedTools: ["Read", "Glob", "Grep"],
    maxTurns: 8,
    cwd: "/path/to/repo",
  },
})) {
  if (message.type === "result" && message.subtype === "success" && !message.is_error) {
    console.log(message.result);
  }
}
```

```python Python theme={null}
import asyncio

from claude_agent_sdk import ClaudeAgentOptions, ResultMessage, query

async def main():
    options = ClaudeAgentOptions(
        model="claude-sonnet-5",
        allowed_tools=["Read", "Glob", "Grep"],
        max_turns=8,
        cwd="/path/to/repo",
    )

    async for message in query(
        prompt="Summarize the open TODOs in this repo",
        options=options,
    ):
        if isinstance(message, ResultMessage) and not message.is_error:
            print(message.result)

asyncio.run(main())
```

将 `cwd` 指向你自己的一个项目并运行示例。该项目的开放 TODO 的摘要在结果消息到达时打印。

`allowedTools`（TypeScript）或 `allowed_tools`（Python）预先批准列出的工具，因此对它们的调用无需停止等待批准即可运行。列表外的工具保持可用。当 Claude 调用未列出的工具时，权限模式决定调用是否运行。有关更多信息，请参阅[允许和拒绝规则](https://code.claude.com/docs/zh-CN/agent-sdk/permissions.md#allow-and-deny-rules)。

## 加载设置文件

设置文件提供超出选项对象的配置。两个选项控制它们的加载方式：

* **`settingSources` / `setting_sources`**：控制加载哪些文件系统源：用户、项目和本地。设置文件和 CLAUDE.md 文件通过这些源到达。
* **`settings`**：加载设置文件路径或任一语言的内联 JSON 字符串，TypeScript 也接受设置对象。无论你传递什么形式，都会覆盖用户、项目和本地文件系统设置；只有托管策略设置排名更高。参考文档在 TypeScript 的[设置优先级](https://code.claude.com/docs/zh-CN/agent-sdk/typescript.md#settings-precedence)和 Python 的[设置优先级](https://code.claude.com/docs/zh-CN/agent-sdk/python.md#settings-precedence)下记录了完整的优先级顺序。

传递 `[]` 以禁用用户、项目和本地设置。有关更多信息，请参阅[在 SDK 中使用 Claude Code 功能](https://code.claude.com/docs/zh-CN/agent-sdk/claude-code-features.md)。

## 选择模型

除非 `model` 选项、你的设置或你的环境选择了模型，否则新会话在[Claude Code 的默认模型](https://code.claude.com/docs/zh-CN/model-config.md#default-model-setting)上启动。有关这些源的顺序，请参阅[设置你的模型](https://code.claude.com/docs/zh-CN/model-config.md#setting-your-model)。设置 `model` 以固定特定模型，或选择较小的模型以获得更快、更便宜的代理。该值采用模型别名或完整模型名称；别名及其解析到的版本列在[模型别名](https://code.claude.com/docs/zh-CN/model-config.md#model-aliases)下。

设置 `fallbackModel`（TypeScript）或 `fallback_model`（Python）以命名备份模型。当主模型过载或不可用时，会话切换到备份。在每个用户轮次开始时重试主模型，因此一旦中断通过，会话就会返回到它。

在任一语言中，该选项接受单个模型或逗号分隔的备份列表。有关顺序和链上限，请参阅[备用模型链](https://code.claude.com/docs/zh-CN/model-config.md#fallback-model-chains)。在 TypeScript 中，等于 `model` 的备用模型在启动时会抛出错误。

下面的示例显示 TypeScript 中的备用列表和 Python 中的单个备用：

```typescript TypeScript theme={null}
const options = {
  model: "claude-fable-5",
  fallbackModel: "claude-opus-5,claude-sonnet-5",
};
```

```python Python theme={null}
options = ClaudeAgentOptions(
    model="claude-fable-5",
    fallback_model="claude-opus-5",
)
```

> [!NOTE]
>
> [Messages API](https://platform.claude.com/docs/en/api/messages) 请求参数 `temperature`、`top_p` 和 `max_tokens` 在任一语言的选项对象上都没有字段。改为设置[努力级别](https://code.claude.com/docs/zh-CN/agent-sdk/agent-loop.md#effort-level)或[支出上限](#limit-turns-and-spend)，或在需要这些参数时直接调用 Messages API。

## 设置环境变量

`env` 选项为运行你的会话的 Claude Code 进程设置环境变量。你的值是替换继承的环境还是合并到它上面因语言而异：

* **TypeScript**：`env` 替换子进程环境
* **Python**：SDK 将你的值合并到继承的环境上，你的值覆盖继承的值

在 TypeScript 中，将 `process.env` 展开到 `env` 中以保留继承的变量，如 `PATH`、`HOME` 和 `ANTHROPIC_API_KEY`。当你不设置 `env` 时，子进程在两种语言中都继承你的环境。

该示例通过设置 `ANTHROPIC_BASE_URL` 将 API 流量路由通过网关。

```typescript TypeScript theme={null}
const options = {
  env: { ...process.env, ANTHROPIC_BASE_URL: "https://gateway.example.com" },
};
```

```python Python theme={null}
options = ClaudeAgentOptions(
    env={"ANTHROPIC_BASE_URL": "https://gateway.example.com"},
)
```

你传递的变量也可以配置 Claude Code 本身。有关 Claude Code 进程读取的变量，请参阅[环境变量](https://code.claude.com/docs/zh-CN/env-vars.md)。要以这种方式调整 API 超时和停滞检测，请按照[TypeScript 参考](https://code.claude.com/docs/zh-CN/agent-sdk/typescript.md#handle-slow-or-stalled-api-responses)或[Python 参考](https://code.claude.com/docs/zh-CN/agent-sdk/python.md#handle-slow-or-stalled-api-responses)中的处理缓慢或停滞的 API 响应部分进行操作。

## 设置工作目录

设置 `cwd` 以在特定目录中运行会话。当你不设置 `cwd` 时，会话在你的进程的工作目录中运行。两个 SDK 都没有 `cwd` 的设置器。要在不同目录中运行，请使用该 `cwd` 启动另一个会话。

Claude Code 读取工作目录以确定：

* **项目设置和 hooks**：哪个项目的[设置和 hooks 加载](https://code.claude.com/docs/zh-CN/agent-sdk/claude-code-features.md)
* **Skills**：[会话 skills 在哪里被发现](https://code.claude.com/docs/zh-CN/agent-sdk/skills.md)
* **会话存储**：[存储的会话属于哪个项目](https://code.claude.com/docs/zh-CN/agent-sdk/session-storage.md)

要让工具访问工作目录外的文件，请使用 `additionalDirectories`（TypeScript）或 `add_dirs`（Python）添加路径。有关该授予的范围，请参阅[其他目录授予文件访问权限，而不是配置](https://code.claude.com/docs/zh-CN/permissions.md#additional-directories-grant-file-access-not-configuration)。

## 限制轮次和支出

使用 `maxTurns` / `max_turns` 和 `maxBudgetUsd` / `max_budget_usd` 限制轮次和支出。当未设置时，两个上限都关闭。当会话达到上限时，运行以结果消息结束，其子类型命名上限，`error_max_turns` 或 `error_max_budget_usd`。接下来发生的事情因输入模式而异：

* **单次 `query()`**：SDK 产生上限结果，然后抛出，因此将循环包装在 try 块中以继续通过错误
* **流式输入**：会话在上限结果之后保持活动，最大轮次计数对每个排队的消息重新开始。预算总额在消息中累积，一旦支出达到上限，同一对话中的后续消息以相同的预算结果结束。[`/clear`](https://code.claude.com/docs/zh-CN/agent-sdk/cost-tracking) 重新开始预算

两个上限对 `0` 的处理方式不同：

* **`maxTurns` / `max_turns`**：`0` 在没有轮次限制的情况下运行会话，与不设置选项相同
* **`maxBudgetUsd` / `max_budget_usd`**：CLI 在启动时拒绝 `0` 作为无效金额，会话永远不会运行

有关两个上限的更多信息，包括子代理支出，请参阅[轮次和预算](https://code.claude.com/docs/zh-CN/agent-sdk/agent-loop.md#turns-and-budget)。

## 在会话中途更改配置

当你使用[流式输入](https://code.claude.com/docs/zh-CN/agent-sdk/streaming-vs-single-mode.md)启动会话时，你可以在它运行时切换其模型和权限模式。你调用设置器的位置因语言而异：

* **TypeScript**：`query()` 返回的对象上的方法
* **Python**：[`ClaudeSDKClient`](https://code.claude.com/docs/zh-CN/agent-sdk/python#claudesdkclient) 上的方法，因为 `query()` 返回没有控制方法的普通迭代器

两种语言都有相同的设置器：

* **`setModel()` / `set_model()`**：切换模型。不带模型调用它以切换到[Claude Code 的默认模型](https://code.claude.com/docs/zh-CN/model-config.md#default-model-setting)，而不是您在选项中传递的 `model`。如果会话处于自动模式，而您切换到自动模式不支持的模型，会话将更改为 `default` 模式。要返回自动模式，请参阅[自动模式（`auto`）](https://code.claude.com/docs/zh-CN/agent-sdk/permissions#auto-mode-auto)。
* **`setPermissionMode()` / `set_permission_mode()`**：切换权限模式

TypeScript 还有 `applyFlagSettings()` 和 `updateSettings()`：

* **`applyFlagSettings()`**：在运行时应用设置，如 `await session.applyFlagSettings({ effortLevel: "high" })`。该方法采用设置文件键而不是选项字段，因此检查[`applyFlagSettings()` 参考](https://code.claude.com/docs/zh-CN/agent-sdk/typescript#applyflagsettings)以了解架构以及哪些键在会话中途生效。
* **`updateSettings()`**：将一个允许列表中的键写入设置文件。[`updateSettings()` 参考](https://code.claude.com/docs/zh-CN/agent-sdk/typescript#updatesettings)命名每个源接受的键和版本下限。
  * 传递 `"localSettings"` 以写入项目的本地设置文件，如 `await session.updateSettings("localSettings", { outputStyle: "Explanatory" })`。写入的键在会话的下一个请求时生效，并为加载 `local` 设置的后续会话持久化。
  * 传递 `"userSettings"` 以写入 `effortLevel`，这是该源接受的唯一键。Claude Code 将其保存为会话当前模型的默认努力级别，运行中的会话的努力不会改变。

下面的示例运行一个两轮会话，在轮次之间更改配置，并打印回答每轮的模型。在 TypeScript 中，提示流保持第二条消息，直到设置器运行，第二轮在新模型上运行。

```typescript TypeScript theme={null}
import { query, type SDKUserMessage } from "@anthropic-ai/claude-agent-sdk";

function userMessage(text: string): SDKUserMessage {
  return { type: "user", message: { role: "user", content: text }, parent_tool_use_id: null };
}

// Hold the second prompt until the setters have run.
let startSecondTurn!: () => void;
const secondTurnReady = new Promise<void>((resolve) => {
  startSecondTurn = resolve;
});

async function* turnPrompts(): AsyncGenerator<SDKUserMessage, void> {
  yield userMessage("Reply with exactly: ready");
  await secondTurnReady;
  yield userMessage("Reply with exactly: done");
}

const session = query({
  prompt: turnPrompts(),
  options: {
    model: "claude-sonnet-5",
  },
});

let turnModel = "";
let completedTurns = 0;

for await (const message of session) {
  if (message.type === "assistant") {
    turnModel = message.message.model;
  } else if (message.type === "result") {
    completedTurns += 1;
    if (completedTurns === 1) {
      console.log(`First turn model: ${turnModel}`);
      await session.setModel("claude-opus-5");
      await session.setPermissionMode("acceptEdits");
      startSecondTurn();
    } else {
      console.log(`Second turn model: ${turnModel}`);
      break;
    }
  }
}
```

```python Python theme={null}
import asyncio

from claude_agent_sdk import AssistantMessage, ClaudeAgentOptions, ClaudeSDKClient

async def main():
    options = ClaudeAgentOptions(model="claude-sonnet-5")

    async with ClaudeSDKClient(options=options) as client:
        await client.query("Reply with exactly: ready")
        first_model = ""
        async for message in client.receive_response():
            if isinstance(message, AssistantMessage):
                first_model = message.model

        await client.set_model("claude-opus-5")
        await client.set_permission_mode("acceptEdits")

        await client.query("Reply with exactly: done")
        second_model = ""
        async for message in client.receive_response():
            if isinstance(message, AssistantMessage):
                second_model = message.model

    print(f"First turn model: {first_model}")
    print(f"Second turn model: {second_model}")

asyncio.run(main())
```

在 Claude API 上，程序打印 `First turn model: claude-sonnet-5`，然后在切换后打印 `Second turn model: claude-opus-5`。

> [!NOTE]
>
> 每个模型都有自己的提示缓存，因此在会话中途切换后，下一个请求以新模型的费率重新计算完整对话而不缓存。有关更多信息，请参阅[切换模型](https://code.claude.com/docs/zh-CN/prompt-caching.md#switching-models)。

## 配置特定功能

下表将每个选项映射到它配置的功能。有关本页面未涵盖的选项，请参阅[TypeScript](https://code.claude.com/docs/zh-CN/agent-sdk/typescript.md#options) 和 [Python](https://code.claude.com/docs/zh-CN/agent-sdk/python.md#claudeagentoptions) 参考。如果你知道你的目标但不知道哪个选项服务于它，请从[选择正确的功能](https://code.claude.com/docs/zh-CN/agent-sdk/claude-code-features.md#choose-the-right-feature)开始。

| TypeScript | Python | 控制 | 涵盖在 |
| - | - | - | - |
| `permissionMode` | `permission_mode` | 代理无需批准可以做什么 | [配置权限](https://code.claude.com/docs/zh-CN/agent-sdk/permissions.md) |
| `allowedTools` | `allowed_tools` | 哪些工具调用被预先批准 | [配置权限](https://code.claude.com/docs/zh-CN/agent-sdk/permissions.md) |
| `canUseTool` | `can_use_tool` | 你对工具调用的批准回调 | [处理工具批准请求](https://code.claude.com/docs/zh-CN/agent-sdk/user-input.md#handle-tool-approval-requests) |
| `systemPrompt` | `system_prompt` | 代理的指令 | [修改系统提示](https://code.claude.com/docs/zh-CN/agent-sdk/modifying-system-prompts.md) |
| `settingSources` | `setting_sources` | 加载哪些文件系统设置 | [在 SDK 中使用 Claude Code 功能](https://code.claude.com/docs/zh-CN/agent-sdk/claude-code-features.md) |
| `mcpServers` | `mcp_servers` | 外部工具服务器 | [使用 MCP 连接到外部工具](https://code.claude.com/docs/zh-CN/agent-sdk/mcp.md) |
| `agents` | `agents` | 子代理定义 | [子代理](https://code.claude.com/docs/zh-CN/agent-sdk/subagents.md) |
| `hooks` | `hooks` | 生命周期点处的回调 | [Hooks](https://code.claude.com/docs/zh-CN/agent-sdk/hooks.md) |
| `skills` | `skills` | 加载哪些 skills | [使用 skills 扩展代理](https://code.claude.com/docs/zh-CN/agent-sdk/skills.md) |
| `plugins` | `plugins` | 加载哪些 plugins | [Plugins](https://code.claude.com/docs/zh-CN/agent-sdk/plugins.md) |
| `outputFormat` | `output_format` | 结构化输出架构 | [结构化输出](https://code.claude.com/docs/zh-CN/agent-sdk/structured-outputs.md) |
| `resume` | `resume` | 继续存储的会话 | [会话](https://code.claude.com/docs/zh-CN/agent-sdk/sessions.md) |
| `forkSession` | `fork_session` | 分支会话 | [会话](https://code.claude.com/docs/zh-CN/agent-sdk/sessions.md) |
| `sessionStore` | `session_store` | 外部会话持久化 | [会话存储](https://code.claude.com/docs/zh-CN/agent-sdk/session-storage.md) |
| `enableFileCheckpointing` | `enable_file_checkpointing` | 可回退的文件编辑 | [文件检查点](https://code.claude.com/docs/zh-CN/agent-sdk/file-checkpointing.md) |
| `effort` | `effort` | Claude 在响应中投入多少工作 | [努力级别](https://code.claude.com/docs/zh-CN/agent-sdk/agent-loop.md#effort-level) |
| `sandbox` | `sandbox` | 工具执行的沙箱行为 | [TypeScript](https://code.claude.com/docs/zh-CN/agent-sdk/typescript.md#sandbox-configuration) 和 [Python](https://code.claude.com/docs/zh-CN/agent-sdk/python.md#sandbox-configuration) 参考，部署上下文在[安全部署](https://code.claude.com/docs/zh-CN/agent-sdk/secure-deployment.md)中 |

## 后续步骤

要查看配置组合成工作代理：

* **[快速入门](https://code.claude.com/docs/zh-CN/agent-sdk/quickstart.md)**：端到端构建和运行第一个代理
* **[示例](https://code.claude.com/docs/zh-CN/agent-sdk/examples.md)**：找到完整的、可运行的项目或与你想要构建的内容匹配的指导 Claude Cookbook 配方
* **[多租户隔离](https://code.claude.com/docs/zh-CN/agent-sdk/hosting.md#multi-tenant-isolation)**：使用 `settingSources` / `setting_sources`、`env` 和 `cwd` 隔离每个租户的设置和内存
