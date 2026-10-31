> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# SDK 中的子代理

> 定义和调用子代理以隔离上下文、并行运行任务，以及在 Claude Agent SDK 应用程序中应用专门的指令。

子代理是您的主代理可以生成的独立代理实例，用于处理专注的子任务。
使用它们来隔离上下文、并行运行多个分析，以及应用专门的指令，而无需添加到主代理的提示中。

## 概述

您可以通过三种方式创建子代理：

* **以编程方式**：在您的 `query()` 选项中使用 `agents` 参数。请参阅 [TypeScript](https://code.claude.com/docs/zh-CN/agent-sdk/typescript.md#agentdefinition) 和 [Python](https://code.claude.com/docs/zh-CN/agent-sdk/python.md#agentdefinition) 参考文档
* **基于文件系统**：在 `.claude/agents/` 目录中将代理定义为 markdown 文件。请参阅[将子代理定义为文件](https://code.claude.com/docs/zh-CN/sub-agents.md)
* **内置通用型**：Claude 可以随时通过 Agent 工具调用内置的 `general-purpose` 子代理，无需您定义任何内容

本指南重点介绍以编程方式的方法，这是 SDK 应用程序的推荐方法。

## 使用子代理的好处

由于子代理是独立的代理实例，将工作委托给它们可以为您带来四个好处：

* **上下文隔离**：每个子代理在自己的对话中运行，除非子代理是[fork](https://code.claude.com/docs/zh-CN/sub-agents.md#fork-the-current-conversation)，否则会从头开始。无论哪种方式，中间工具调用和结果都保留在子代理内部；只有其最终消息返回到父代理。`research-assistant` 子代理可以探索数十个文件，而不会有任何内容在主对话中累积。父代理收到的是简洁的摘要，而不是子代理读取的每个文件。有关子代理上下文中的确切内容，请参阅[子代理继承的内容](#what-subagents-inherit)。
* **并行化**：多个子代理可以并发运行，因此独立的子任务在最慢的一个的时间内完成，而不是所有任务的总和。在代码审查期间，您可以同时运行 `style-checker`、`security-scanner` 和 `test-coverage` 子代理，而不是按顺序运行。
* **专门的指令和知识**：每个子代理可以有一个定制的系统提示，具有特定的专业知识、最佳实践和约束。`database-migration` 子代理可以拥有关于 SQL 最佳实践、回滚策略和数据完整性检查的详细知识，这些在主代理的指令中会是不必要的噪音。
* **工具限制**：子代理可以限制为特定工具，降低意外操作的风险。`doc-reviewer` 子代理可能只能访问 Read 和 Grep 工具，确保它可以分析但永远不会意外修改您的文档文件。

## 创建子代理

### 程序化定义（推荐）

使用 `agents` 参数直接在代码中定义子代理。Claude 通过 `Agent` 工具调用子代理。

本页面上的大多数示例仅打印最终结果。要确认 Claude 委托给了子代理而不是直接回答，请参阅[检测子代理调用](#detect-subagent-invocation)。

此示例创建两个子代理：一个具有只读访问权限的代码审查员和一个可以执行命令的测试运行器。

```python Python theme={null}
import asyncio
from claude_agent_sdk import query, ClaudeAgentOptions, AgentDefinition


async def main():
    async for message in query(
        prompt="Review the authentication module for security issues",
        options=ClaudeAgentOptions(
            # Auto-approve these tools
            allowed_tools=["Read", "Grep", "Glob", "Agent"],
            agents={
                "code-reviewer": AgentDefinition(
                    # description tells Claude when to use this subagent
                    description="Expert code review specialist. Use for quality, security, and maintainability reviews.",
                    # prompt defines the subagent's behavior and expertise
                    prompt="""You are a code review specialist with expertise in security, performance, and best practices.

When reviewing code:
- Identify security vulnerabilities
- Check for performance issues
- Verify adherence to coding standards
- Suggest specific improvements

Be thorough but concise in your feedback.""",
                    # tools restricts what the subagent can do (read-only here)
                    tools=["Read", "Grep", "Glob"],
                    # model overrides the default model for this subagent
                    model="sonnet",
                ),
                "test-runner": AgentDefinition(
                    description="Runs and analyzes test suites. Use for test execution and coverage analysis.",
                    prompt="""You are a test execution specialist. Run tests and provide clear analysis of results.

Focus on:
- Running test commands
- Analyzing test output
- Identifying failing tests
- Suggesting fixes for failures""",
                    # Bash access lets this subagent run test commands
                    tools=["Bash", "Read", "Grep"],
                ),
            },
        ),
    ):
        if hasattr(message, "result"):
            print(message.result)


asyncio.run(main())
```

```typescript TypeScript theme={null}
import { query } from "@anthropic-ai/claude-agent-sdk";

for await (const message of query({
  prompt: "Review the authentication module for security issues",
  options: {
    // Auto-approve these tools
    allowedTools: ["Read", "Grep", "Glob", "Agent"],
    agents: {
      "code-reviewer": {
        // description tells Claude when to use this subagent
        description:
          "Expert code review specialist. Use for quality, security, and maintainability reviews.",
        // prompt defines the subagent's behavior and expertise
        prompt: `You are a code review specialist with expertise in security, performance, and best practices.

When reviewing code:
- Identify security vulnerabilities
- Check for performance issues
- Verify adherence to coding standards
- Suggest specific improvements

Be thorough but concise in your feedback.`,
        // tools restricts what the subagent can do (read-only here)
        tools: ["Read", "Grep", "Glob"],
        // model overrides the default model for this subagent
        model: "sonnet"
      },
      "test-runner": {
        description:
          "Runs and analyzes test suites. Use for test execution and coverage analysis.",
        prompt: `You are a test execution specialist. Run tests and provide clear analysis of results.

Focus on:
- Running test commands
- Analyzing test output
- Identifying failing tests
- Suggesting fixes for failures`,
        // Bash access lets this subagent run test commands
        tools: ["Bash", "Read", "Grep"]
      }
    }
  }
})) {
  if ("result" in message) console.log(message.result);
}
```

### AgentDefinition 配置

| 字段 | 类型 | 必需 | 描述 |
| :- | :- | :- | :- |
| `description` | `string` | 是 | 何时使用此代理的自然语言描述 |
| `prompt` | `string` | 是 | 代理的系统提示，定义其角色和行为 |
| `tools` | `string[]` | 否 | 允许的工具名称数组。如果省略，继承[子代理可用的每个工具](https://code.claude.com/docs/zh-CN/sub-agents.md#available-tools) |
| `disallowedTools` | `string[]` | 否 | 要从代理工具集中移除的工具名称数组。也接受 MCP 服务器级别的模式：`mcp__server` 或 `mcp__server__*` 移除该服务器的每个工具，`mcp__*` 移除任何服务器的每个 MCP 工具 |
| `model` | `string` | 否 | 此代理的模型覆盖。接受别名如 `'fable'`、`'opus'`、`'sonnet'`、`'haiku'`、`'inherit'` 或完整模型 ID。`'inherit'` 使用主模型。省略时，Claude Code 选择[子代理模型顺序](https://code.claude.com/docs/zh-CN/sub-agents.md#choose-a-model)中的模型 |
| `skills` | `string[]` | 否 | 启动时预加载到 Agent 上下文中的 skill 名称列表。最多预加载列表中的[前 32 个不同名称](https://code.claude.com/docs/zh-CN/sub-agents.md#preload-skills-into-subagents)。未列出的 skill 仍可通过 Skill 工具调用 |
| `memory` | `'user' \| 'project' \| 'local'` | 否 | 此代理的内存源 |
| `mcpServers` | `(string \| object)[]` | 否 | 此代理可用的 MCP 服务器，按名称或内联配置 |
| `initialPrompt` | `string` | 否 | 当此代理作为主线程代理运行时自动提交为第一个用户轮次。当代理作为子代理调用时忽略 |
| `maxTurns` | `number` | 否 | 代理停止前的最大代理轮次数。当代理达到限制时，Claude Code 返回其输出标记为部分，您可以[恢复代理](#resume-subagents)以继续。部分标记需要 Claude Code v2.1.246 或更高版本 |
| `autoCompactWindow` | `number` | 否 | 当此 Agent 作为子代理运行时，将其[自动压缩窗口](https://code.claude.com/docs/zh-CN/sub-agents.md#subagent-auto-compact-window)降低到 `100000` 至 `1000000` 之间的 token 数。需要 TypeScript Agent SDK v0.3.296 或更高版本。Python SDK 的 [`AgentDefinition`](https://code.claude.com/docs/zh-CN/agent-sdk/python#agentdefinition) 没有此字段 |
| `background` | `boolean` | 否 | 调用时将此代理作为非阻塞后台任务运行 |
| `omitClaudeMd` | `boolean` | 否 | 将用户、项目和本地 CLAUDE.md 文件排除在此 Agent 的初始上下文之外；托管策略文件仍会加载。当该 Agent 作为主线程 Agent 运行时忽略。需要 TypeScript Agent SDK v0.3.271 或更高版本。Python SDK 的 [`AgentDefinition`](https://code.claude.com/docs/zh-CN/agent-sdk/python#agentdefinition) 没有此字段 |
| `effort` | `'low' \| 'medium' \| 'high' \| 'xhigh' \| 'max' \| number` | 否 | 此代理的推理工作量级别 |
| `permissionMode` | `PermissionMode` | 否 | 此代理内工具执行的权限模式。[子代理继承规则](https://code.claude.com/docs/zh-CN/agent-sdk/permissions.md#available-modes)决定何时应用 |

在 Python SDK 中，多词字段名称如 `disallowedTools` 和 `mcpServers` 保持其 camelCase 拼写以匹配线路格式，而不是遵循 Python 的 snake\_case 约定。有关详细信息，请参阅[`AgentDefinition` 参考](https://code.claude.com/docs/zh-CN/agent-sdk/python#agentdefinition)。

#### 选择 Claude 是否等待子代理

默认情况下，主 Agent 生成的子代理[在后台运行](https://code.claude.com/docs/zh-CN/sub-agents.md#run-subagents-in-foreground-or-background)。Agent 工具结果包含 `status: "async_launched"`，报告会在之后的轮次中送达。

当 Claude 的下一步操作需要该报告时，它会等待：调用会阻塞并返回 `status: "completed"`。要自行做出此选择：

* **在提示词中**：要求 Claude 等待，例如"run the test-runner agent and wait for it"
* **在定义中**：将 `background` 设置为 `true`，使该 Agent 保持在后台运行

### 基于文件系统的定义（替代方案）

您也可以在 `.claude/agents/` 目录中将子代理定义为 markdown 文件。有关此方法的详细信息，请参阅 [Claude Code 子代理文档](https://code.claude.com/docs/zh-CN/sub-agents.md)。程序化定义的代理优先于具有相同名称的基于文件系统的代理。

> [!NOTE]
>
> 当 Claude 调用不带 `subagent_type` 的 Agent 工具时，它获得内置的 `general-purpose` 子代理，即使您未定义任何自己的代理，Claude 也可以生成。设置 [`CLAUDE_AGENT_SDK_DISABLE_BUILTIN_AGENTS=1`](https://code.claude.com/docs/zh-CN/env-vars) 移除该默认值，此类调用失败并显示 [`subagent_type is required`](https://code.claude.com/docs/zh-CN/errors#subagent-type-is-required)。

## 子代理继承的内容

除非子代理是[分叉](https://code.claude.com/docs/zh-CN/sub-agents.md#fork-the-current-conversation)，否则其上下文窗口会重新开始，没有父对话，但也不是空的。从父代理传递给子代理的唯一内容是 Agent 工具的提示字符串，因此请直接在该提示中包含子代理需要的任何文件路径、错误消息或决策。

具有 [`SendMessage`](https://code.claude.com/docs/zh-CN/tools-reference) 工具的子代理会从会话中运行的其他命名代理列表开始，因此它知道可以向哪些名称发送消息。Claude Code 会在子代理的第一轮自动将列表添加到其中。[分叉](https://code.claude.com/docs/zh-CN/sub-agents.md#fork-the-current-conversation)不会获得该列表，因为它继承的是父对话。

子代理还继承主会话的扩展思考配置。

下表列出了非分叉子代理的上下文包含的内容以及它遗漏的内容。

| 子代理接收 | 子代理不接收 |
| :- | :- |
| 其自身的系统提示（`AgentDefinition.prompt`）和 Agent 工具的提示 | 父代理的对话历史或工具结果 |
| 项目 CLAUDE.md（通过 [`settingSources`](https://code.claude.com/docs/zh-CN/agent-sdk/claude-code-features#control-filesystem-settings-with-settingsources) 加载）。设置了 [`omitClaudeMd`](#agentdefinition-configuration) 的 Agent 启动时不包含它 | 预加载的 skill 内容，除非在 `AgentDefinition.skills` 中列出 |
| 工具定义（从父代理继承或 `tools` 中的子集，[为后台运行过滤](https://code.claude.com/docs/zh-CN/sub-agents.md#available-tools)） | 父代理的系统提示 |

> [!NOTE]
>
> 父 Agent 接收子代理的最终报告，但可能在其自己的回复中对其进行总结。要在面向用户的回复中逐字保留子代理输出，请在传递给主 `query()` 调用的提示词或 `systemPrompt` 选项中包含执行此操作的指令。
>
> 在 v2.1.210 及更高版本中，Claude Code [在父代理读取最终消息之前扫描它以查找指令形状的模式](https://code.claude.com/docs/zh-CN/sub-agents.md#subagent-output-scanning)。扫描以三种不同的方式处理三种模式：
>
> * **控制标签模仿**：Claude Code 就地中和仅由工具发出的标签，例如 `<system-reminder>` 块。它在开始角括号后插入反斜杠，不删除任何内容。
> * **权限配置提及**：Claude Code 保持对权限配置的引用，例如 `.claude/settings.json`、`bypassPermissions` 或 `--dangerously-skip-permissions`，按原样写入。
> * **轮次标记**：以 `Human:` 或 `Assistant:` 开头的行在冒号前获得反斜杠，因此消息无法模仿对话轮次边界。
>
> 对于控制标签或权限配置匹配，Claude Code 在前面加上 `[harness: ...]` 标记行，命名匹配的模式；轮次标记匹配不添加标记行。这些是扫描所做的唯一修改：它从不删除或改写子代理的文本。

提前结束子代理的 API 错误（例如速率限制）永远不会作为其结果传递。有关前台和后台行为，请参阅[子代理中的 API 错误](https://code.claude.com/docs/zh-CN/sub-agents.md#api-errors-in-subagents)。

## 调用子代理

### 自动调用

Claude 根据任务和每个子代理的 `description` 自动决定何时调用子代理。例如，如果你定义了一个 `performance-optimizer` 子代理，其描述为"用于查询调优的性能优化专家"，当你的提示中提到优化查询时，Claude 将调用它。

编写清晰、具体的描述，以便 Claude 能够将任务与正确的子代理匹配。

### 显式调用

要保证 Claude 使用特定的子代理，请在你的提示中按名称提及它：

```text theme={null}
"Use the code-reviewer agent to check the authentication module"
```

这会绕过自动匹配，直接调用指定的子代理。

### 分叉子代理

[分叉](https://code.claude.com/docs/zh-CN/sub-agents.md#fork-the-current-conversation)是一种复制当前已有对话、而非从头开始的子代理。从 Claude Code v2.1.302 开始，无需任何配置，Claude 即可在您的应用程序中启动分叉，方法是调用 Agent 工具并传入 `subagent_type: "fork"`。

要阻止 Claude 启动分叉，请添加 [`Agent(fork)` 拒绝规则](https://code.claude.com/docs/zh-CN/agent-sdk/permissions#allow-and-deny-rules)：

```python Python theme={null}
options = ClaudeAgentOptions(
    # 仅拒绝 fork 子代理类型；其他子代理仍会运行
    disallowed_tools=["Agent(fork)"],
)
```

```typescript TypeScript theme={null}
const options = {
  // 仅拒绝 fork 子代理类型；其他子代理仍会运行
  disallowedTools: ["Agent(fork)"]
};
```

### 动态代理配置

你可以根据运行时条件动态创建代理定义。此示例创建了一个安全审查器，具有不同的严格程度，对严格审查使用更强大的模型。

```python Python theme={null}
import asyncio
from claude_agent_sdk import query, ClaudeAgentOptions, AgentDefinition


# 返回 AgentDefinition 的工厂函数
# 此模式允许你根据运行时条件自定义代理
def create_security_agent(security_level: str) -> AgentDefinition:
    is_strict = security_level == "strict"
    return AgentDefinition(
        description="Security code reviewer",
        # 根据严格程度自定义提示
        prompt=f"You are a {'strict' if is_strict else 'balanced'} security reviewer...",
        tools=["Read", "Grep", "Glob"],
        # 关键见解：对高风险审查使用更强大的模型
        model="opus" if is_strict else "sonnet",
    )


async def main():
    # 代理在查询时创建，因此每个请求都可以使用不同的设置
    async for message in query(
        prompt="Review this PR for security issues",
        options=ClaudeAgentOptions(
            allowed_tools=["Read", "Grep", "Glob", "Agent"],
            agents={
                # 使用你所需的配置调用工厂
                "security-reviewer": create_security_agent("strict")
            },
        ),
    ):
        if hasattr(message, "result"):
            print(message.result)


asyncio.run(main())
```

```typescript TypeScript theme={null}
import { query, type AgentDefinition } from "@anthropic-ai/claude-agent-sdk";

// 返回 AgentDefinition 的工厂函数
// 此模式允许你根据运行时条件自定义代理
function createSecurityAgent(securityLevel: "basic" | "strict"): AgentDefinition {
  const isStrict = securityLevel === "strict";
  return {
    description: "Security code reviewer",
    // 根据严格程度自定义提示
    prompt: `You are a ${isStrict ? "strict" : "balanced"} security reviewer...`,
    tools: ["Read", "Grep", "Glob"],
    // 关键见解：对高风险审查使用更强大的模型
    model: isStrict ? "opus" : "sonnet"
  };
}

// 代理在查询时创建，因此每个请求都可以使用不同的设置
for await (const message of query({
  prompt: "Review this PR for security issues",
  options: {
    allowedTools: ["Read", "Grep", "Glob", "Agent"],
    agents: {
      // 使用你所需的配置调用工厂
      "security-reviewer": createSecurityAgent("strict")
    }
  }
})) {
  if ("result" in message) console.log(message.result);
}
```

## 检测子代理调用

Claude 通过 Agent 工具调用子代理。要检测何时调用子代理，请检查 `tool_use` 块，其中 `name` 为 `"Agent"`。

来自子代理上下文内的消息包含 `parent_tool_use_id` 字段。在 TypeScript 中，子代理生成的每条 assistant 消息和 user 消息还带有 [`agent_id`](https://code.claude.com/docs/zh-CN/agent-sdk/typescript#sdkassistantmessage)：即该子代理的[任务事件](https://code.claude.com/docs/zh-CN/agent-sdk/typescript.md#sdktaskstartedmessage)中的 `task_id`。当您[恢复子代理](#resume-subagents)时，`agent_id` 保持不变。`agent_id` 需要 TypeScript Agent SDK v0.3.292 或更高版本。

> [!NOTE]
>
> 该工具在 `tool_use` 块中显示为 `"Agent"`，但在 `system:init` 工具列表中显示为 `"Task"`。在 Claude Code v2.1.63 之前，`tool_use` 块也将其命名为 `"Task"`。为了保持检测在不同 SDK 版本中的工作，请在 `block.name` 中匹配两个值。

消息结构在不同 SDK 中有所不同。在 Python 中，您可以通过 `message.content` 直接访问内容块。在 TypeScript 中，`SDKAssistantMessage` 包装 Claude API 消息，因此您通过 `message.message.content` 访问内容。

此示例遍历流式消息，记录何时调用子代理以及后续消息来自该子代理执行上下文内的时间。TypeScript 版本还会为每条带有 `agent_id` 的子代理消息记录该 `agent_id`。

```python Python theme={null}
import asyncio
from claude_agent_sdk import query, ClaudeAgentOptions, AgentDefinition, ToolUseBlock


async def main():
    async for message in query(
        prompt="Use the code-reviewer agent to review this codebase",
        options=ClaudeAgentOptions(
            allowed_tools=["Read", "Glob", "Grep", "Agent"],
            agents={
                "code-reviewer": AgentDefinition(
                    description="Expert code reviewer.",
                    prompt="Analyze code quality and suggest improvements.",
                    tools=["Read", "Glob", "Grep"],
                )
            },
        ),
    ):
        # Check for subagent invocation. Match both names: older SDK
        # versions emitted "Task", current versions emit "Agent".
        if hasattr(message, "content") and message.content:
            for block in message.content:
                if isinstance(block, ToolUseBlock) and block.name in (
                    "Task",
                    "Agent",
                ):
                    print(f"Subagent invoked: {block.input.get('subagent_type')}")

        # Check if this message is from within a subagent's context
        if hasattr(message, "parent_tool_use_id") and message.parent_tool_use_id:
            print("  (running inside subagent)")

        if hasattr(message, "result"):
            print(message.result)


asyncio.run(main())
```

```typescript TypeScript theme={null}
import { query } from "@anthropic-ai/claude-agent-sdk";

for await (const message of query({
  prompt: "Use the code-reviewer agent to review this codebase",
  options: {
    allowedTools: ["Read", "Glob", "Grep", "Agent"],
    agents: {
      "code-reviewer": {
        description: "Expert code reviewer.",
        prompt: "Analyze code quality and suggest improvements.",
        tools: ["Read", "Glob", "Grep"]
      }
    }
  }
})) {
  const msg = message as any;

  // Check for subagent invocation. Match both names: older SDK versions
  // emitted "Task", current versions emit "Agent".
  for (const block of msg.message?.content ?? []) {
    if (block.type === "tool_use" && (block.name === "Task" || block.name === "Agent")) {
      console.log(`Subagent invoked: ${block.input.subagent_type}`);
    }
  }

  // Check if this message is from within a subagent's context
  if (msg.parent_tool_use_id) {
    console.log("  (running inside subagent)");
    // On assistant and user messages, agent_id matches the task_id
    // on that subagent's task_started and other task events
    if (msg.agent_id) {
      console.log(`  agent_id: ${msg.agent_id}`);
    }
  }

  if ("result" in message) {
    console.log(message.result);
  }
}
```

## 恢复子代理

您可以恢复子代理以继续其中断的地方，而不是重新开始。恢复的子代理保留其完整的对话历史记录，包括所有先前的工具调用、结果和推理。

当子代理在其 [`maxTurns`](#agentdefinition-configuration) 限制处停止时，Claude Code 会在 Agent 工具结果中将输出标记为部分，以便 Claude 知道运行未完成。

当子代理完成时，Agent 工具结果包含一个包含 `agentId: <id>` 的文本块。内置的 [`Explore` 和 `Plan` 代理](https://code.claude.com/docs/zh-CN/sub-agents#built-in-subagents) 是一次性的，不返回 `agentId`，因此当您需要恢复时，请使用自定义代理或 `general-purpose`。要以编程方式恢复子代理：

1. **捕获会话 ID**：从第一个查询期间的消息中提取 `session_id`
2. **提取代理 ID**：从 Agent 工具结果文本中解析 `agentId`
3. **恢复会话**：在第二个查询的选项中传递 `resume: sessionId`，并在您的提示中包含代理 ID。每个 `query()` 调用默认启动一个新会话，您必须恢复同一会话才能访问子代理的记录。

> [!NOTE]
>
> 使用自定义代理时，在两个查询的 `agents` 参数中传递相同的代理定义。

下面的示例定义了一个自定义 `endpoint-finder` 代理。第一个查询运行它并从 Agent 工具结果中捕获会话 ID 和代理 ID，然后第二个查询恢复会话以提出需要来自第一次分析的上下文的后续问题。

```python Python theme={null}
import asyncio
import re
from claude_agent_sdk import query, ClaudeAgentOptions, AgentDefinition, ToolResultBlock

AGENTS = {
    "endpoint-finder": AgentDefinition(
        description="Locates and catalogs API endpoints in a codebase.",
        prompt="You find and document API endpoints. Report each endpoint's path, method, and handler.",
        tools=["Read", "Grep", "Glob"],
    )
}


def extract_agent_id(block: ToolResultBlock) -> str | None:
    """Extract agentId from an Agent tool result's text content."""
    parts = block.content if isinstance(block.content, list) else [{"text": block.content}]
    for part in parts:
        if match := re.search(r"agentId:\s*([\w-]+)", part.get("text") or ""):
            return match.group(1)
    return None


async def main():
    agent_id = None
    session_id = None

    # First invocation - run the endpoint-finder subagent
    try:
        async for message in query(
            prompt="Use the endpoint-finder agent to find all API endpoints in this codebase",
            options=ClaudeAgentOptions(allowed_tools=["Read", "Grep", "Glob", "Agent"], agents=AGENTS),
        ):
            # Capture session_id from ResultMessage (needed to resume this session)
            if hasattr(message, "session_id"):
                session_id = message.session_id
            # Search tool results for the agentId trailer
            for block in getattr(message, "content", None) or []:
                if isinstance(block, ToolResultBlock):
                    agent_id = extract_agent_id(block) or agent_id
            # Print the final result
            if hasattr(message, "result"):
                print(message.result)
    except Exception as error:
        # A single-shot query() raises after yielding an error result,
        # so session_id and agent_id have already been captured by the loop above.
        print(f"Session ended with an error: {error}")

    # Second invocation - resume and ask follow-up
    if agent_id and session_id:
        async for message in query(
            prompt=f"Resume agent {agent_id} and list the top 3 most complex endpoints",
            options=ClaudeAgentOptions(
                allowed_tools=["Read", "Grep", "Glob", "Agent"], agents=AGENTS, resume=session_id
            ),
        ):
            if hasattr(message, "result"):
                print(message.result)
    else:
        print("No agentId found in the first query, so there is no subagent to resume.")


asyncio.run(main())
```

```typescript TypeScript theme={null}
import { query, type SDKMessage } from "@anthropic-ai/claude-agent-sdk";

const agents = {
  "endpoint-finder": {
    description: "Locates and catalogs API endpoints in a codebase.",
    prompt: "You find and document API endpoints. Report each endpoint's path, method, and handler.",
    tools: ["Read", "Grep", "Glob"]
  }
};

// Stringify content to search for agentId without traversing nested block types
function extractAgentId(message: SDKMessage): string | undefined {
  if (message.type !== "assistant" && message.type !== "user") return undefined;
  const content = JSON.stringify(message.message.content);
  const match = content.match(/agentId:\s*([\w-]+)/);
  return match?.[1];
}

let agentId: string | undefined;
let sessionId: string | undefined;

// First invocation - run the endpoint-finder subagent
try {
  for await (const message of query({
    prompt: "Use the endpoint-finder agent to find all API endpoints in this codebase",
    options: { allowedTools: ["Read", "Grep", "Glob", "Agent"], agents }
  })) {
    // Capture session_id from ResultMessage (needed to resume this session)
    if ("session_id" in message) sessionId = message.session_id;
    // Search message content for the agentId (appears in Agent tool results)
    const extractedId = extractAgentId(message);
    if (extractedId) agentId = extractedId;
    // Print the final result
    if ("result" in message) console.log(message.result);
  }
} catch (error) {
  // A single-shot query() throws after yielding an error result,
  // so sessionId and agentId have already been captured by the loop above.
  console.error(`Session ended with an error: ${error}`);
}

// Second invocation - resume and ask follow-up
if (agentId && sessionId) {
  for await (const message of query({
    prompt: `Resume agent ${agentId} and list the top 3 most complex endpoints`,
    options: { allowedTools: ["Read", "Grep", "Glob", "Agent"], agents, resume: sessionId }
  })) {
    if ("result" in message) console.log(message.result);
  }
} else {
  console.log("No agentId found in the first query, so there is no subagent to resume.");
}
```

子代理记录存储在单独的文件中，并独立于主对话而持久存在。有关压缩行为和 `cleanupPeriodDays` 清理期，请参阅 [Claude Code 中的恢复子代理](https://code.claude.com/docs/zh-CN/sub-agents.md#resume-subagents)。

## 工具限制

使用 `tools` 字段来限制子代理可以执行的操作：

* **省略 `tools`**：子代理获得[可用于子代理的每个工具](https://code.claude.com/docs/zh-CN/sub-agents.md#available-tools)
* **列出工具**：子代理仅获得这些工具。例如，不应该编辑文件的代码审查员会获得 `["Read", "Grep", "Glob"]`

你省略的工具根本不会出现在子代理的会话中：Claude 可以在没有它的情况下工作，不会出现权限提示或错误。

此示例创建了一个只读分析代理，可以检查代码但无法修改文件或运行命令。

```python Python theme={null}
import asyncio
from claude_agent_sdk import query, ClaudeAgentOptions, AgentDefinition


async def main():
    async for message in query(
        prompt="Analyze the architecture of this codebase",
        options=ClaudeAgentOptions(
            allowed_tools=["Read", "Grep", "Glob", "Agent"],
            agents={
                "code-analyzer": AgentDefinition(
                    description="Static code analysis and architecture review",
                    prompt="""You are a code architecture analyst. Analyze code structure,
identify patterns, and suggest improvements without making changes.""",
                    # Read-only tools: no Edit, Write, or Bash access
                    tools=["Read", "Grep", "Glob"],
                )
            },
        ),
    ):
        if hasattr(message, "result"):
            print(message.result)


asyncio.run(main())
```

```typescript TypeScript theme={null}
import { query } from "@anthropic-ai/claude-agent-sdk";

for await (const message of query({
  prompt: "Analyze the architecture of this codebase",
  options: {
    allowedTools: ["Read", "Grep", "Glob", "Agent"],
    agents: {
      "code-analyzer": {
        description: "Static code analysis and architecture review",
        prompt: `You are a code architecture analyst. Analyze code structure,
identify patterns, and suggest improvements without making changes.`,
        // Read-only tools: no Edit, Write, or Bash access
        tools: ["Read", "Grep", "Glob"]
      }
    }
  }
})) {
  if ("result" in message) console.log(message.result);
}
```

### 常见工具组合

| 用例 | 工具 | 描述 |
| :- | :- | :- |
| 只读分析 | `Read`、`Grep`、`Glob` | 可以检查代码但无法修改或执行 |
| 测试执行 | `Bash`、`Read`、`Grep` | 可以运行命令并分析输出 |
| 代码修改 | `Read`、`Edit`、`Write`、`Grep`、`Glob` | 完整的读/写访问权限，无命令执行 |
| 完全访问 | 所有工具 | 继承可用于子代理的工具（省略 `tools` 字段） |

## 限制子代理的深度、并发和支出

> [!NOTE]
>
> 本部分描述 TypeScript SDK v0.3.219 和 Python SDK v0.2.127 及更高版本，这些版本捆绑了 Claude Code v2.1.219 或更高版本。在较早的版本中，某些限制缺失或默认值不同，因此在依赖它们来限制运行之前，请升级。[环境变量参考](https://code.claude.com/docs/zh-CN/env-vars.md)和[轮次和预算](https://code.claude.com/docs/zh-CN/agent-sdk/agent-loop.md#turns-and-budget)记录了添加每个变量的 Claude Code 版本以及支出上限的子代理强制执行。

Claude 会自行决定何时生成子代理以及生成多少个子代理。每个子代理都会发出自己的 API 请求，这些请求计入查询的 `total_cost_usd`，而子代理可以生成自己的子代理，因此一个提示可以扩展成一个代理树。

您可以通过三种方式限制这种增长：子代理的嵌套深度、同时运行的数量以及整个查询的支出。通过 [`env`](https://code.claude.com/docs/zh-CN/agent-sdk/typescript#options) 选项将深度和并发限制设置为环境变量，并将支出限制设置为查询选项：

| 限制 | 设置方式 | 默认值 | Claude Code 在达到限制时的行为 |
| :- | :- | :- | :- |
| 深度 | [`CLAUDE_CODE_MAX_SUBAGENT_SPAWN_DEPTH`](https://code.claude.com/docs/zh-CN/env-vars) | 主代理下方的`3`层子代理。`1`会阻止您的子代理生成任何自己的子代理 | 使底层的子代理无法生成，因此它会自己完成委派的工作。请参阅[嵌套子代理](https://code.claude.com/docs/zh-CN/sub-agents.md#let-subagents-spawn-their-own-subagents) |
| 并发 | [`CLAUDE_CODE_MAX_CONCURRENT_SUBAGENTS`](https://code.claude.com/docs/zh-CN/env-vars) | `20`个子代理同时运行，计算 Claude 使用 Agent 工具生成的每个子代理 | 拒绝生成另一个子代理，返回 `Concurrent subagent limit reached`，直到运行计数降至限制以下。启用了[ultracode](https://code.claude.com/docs/zh-CN/model-config.md#adjust-effort-level)的会话永远不会被拒绝。请参阅[并发子代理限制](https://code.claude.com/docs/zh-CN/sub-agents.md#concurrent-subagent-limit) |
| 支出 | TypeScript 中的 `maxBudgetUsd`，Python 中的 `max_budget_usd` | 无限制。计算调用自身的支出，包括子代理请求 | 通过三种方式强制执行上限：拒绝生成更多子代理，返回 `Budget limit reached`，停止仍在运行的后台子代理，并以 `error_max_budget_usd` 结果子类型结束查询。关于上限在会话中的行为方式，请参阅[轮次和预算](https://code.claude.com/docs/zh-CN/agent-sdk/agent-loop.md#turns-and-budget) |

两个 SDK 对 `env` 选项的处理方式不同：TypeScript SDK 用它替换子进程环境，因此将 `process.env` 展开到其中以保留 `PATH` 等变量，而 Python SDK 将其合并到继承的环境中。此示例关闭嵌套，最多允许五个子代理同时运行，并在估计支出达到 \$5 时停止查询：

```python Python theme={null}
import asyncio
from claude_agent_sdk import query, ClaudeAgentOptions, ResultMessage


async def main():
    try:
        async for message in query(
            prompt="Audit every service in this repo for unhandled promise rejections",
            options=ClaudeAgentOptions(
                allowed_tools=["Read", "Grep", "Glob", "Agent"],
                # env is merged on top of the inherited environment
                env={
                    "CLAUDE_CODE_MAX_SUBAGENT_SPAWN_DEPTH": "1",
                    "CLAUDE_CODE_MAX_CONCURRENT_SUBAGENTS": "5",
                },
                max_budget_usd=5.0,
            ),
        ):
            if isinstance(message, ResultMessage):
                print(f"{message.subtype}: ${message.total_cost_usd}")
    except Exception as error:
        # A single-shot query() raises after yielding an error result,
        # so the budget-capped result has already been printed above.
        print(f"Session ended with an error: {error}")


asyncio.run(main())
```

```typescript TypeScript theme={null}
import { query } from "@anthropic-ai/claude-agent-sdk";

try {
  for await (const message of query({
    prompt: "Audit every service in this repo for unhandled promise rejections",
    options: {
      allowedTools: ["Read", "Grep", "Glob", "Agent"],
      // env replaces the subprocess environment, so spread process.env to keep PATH
      env: {
        ...process.env,
        CLAUDE_CODE_MAX_SUBAGENT_SPAWN_DEPTH: "1",
        CLAUDE_CODE_MAX_CONCURRENT_SUBAGENTS: "5",
      },
      maxBudgetUsd: 5,
    },
  })) {
    if (message.type === "result") {
      console.log(`${message.subtype}: $${message.total_cost_usd}`);
    }
  }
} catch (error) {
  // A single-shot query() throws after yielding an error result,
  // so the budget-capped result has already been logged above.
  console.error(`Session ended with an error: ${error}`);
}
```

您看到的内容取决于查询达到的限制（如果有的话）：

* **在支出上限以下**：您会看到 `success` 和估计成本。
* **在支出上限处**：您会看到 `error_max_budget_usd`，成本为 `5` 或以上，然后您的错误处理程序运行。
* **在并发限制处**：您会在消息流中看到一个 `tool_result` 块，其中包含 `Concurrent subagent limit reached`。Claude 收到与 Agent 工具结果相同的块。

### 使用子代理运行 Opus 5

Claude Opus 5 比早期模型更容易委派给子代理，因此[深度、并发和支出限制](#cap-subagent-depth-concurrency-and-spend)在运行 Opus 5 的查询中最为重要。[Opus 5 提示指南](https://platform.claude.com/docs/en/build-with-claude/prompt-engineering/prompting-claude-opus-5#controlling-subagent-spawning)提供了一条委派指令，您可以将其添加到任何提示中。Claude Code 是否添加自己的指令取决于您使用的[系统提示](https://code.claude.com/docs/zh-CN/agent-sdk/modifying-system-prompts.md#how-system-prompts-work)：

* **`claude_code` 预设**：当模型是 Opus 5 时，Claude Code 会在其系统提示中添加一行，告诉 Claude 除非被要求，否则不要调用 Agent 工具。Agent 工具保持可用。
* **自定义提示或无 `systemPrompt`**：Claude Code 不构建其系统提示，因此该行不存在。将提示指南的委派指令添加到您自己的提示中。

任一指令只会引导 Claude，因此也要设置限制。Claude Code 会根据 Claude 决定的委派方式强制执行它们。

## 使用动态工作流进行扩展

子代理适用于每轮委派的几个任务。对于协调数十到数百个代理的运行，请使用 `Workflow` 工具，它将编排移到运行时在对话上下文外执行的脚本中。请参阅[动态工作流](https://code.claude.com/docs/zh-CN/workflows.md)以了解工作流与逐轮子代理委派的区别。

`Workflow` 工具在 TypeScript Agent SDK v0.3.149 及更高版本中可用。在 `allowedTools` 中包含 `Workflow` 以自动批准工作流运行。工具输入和输出架构列在 [TypeScript 参考](https://code.claude.com/docs/zh-CN/agent-sdk/typescript.md#workflow)中。

## 故障排除

### Claude 不委派给子代理

如果 Claude 直接完成任务而不是委派给您的子代理：

* **使用显式提示**：在您的提示词中按名称提及子代理，例如"使用代码审查员代理来检查身份验证模块"
* **编写清晰的描述**：准确解释何时应使用子代理，以便 Claude 可以适当地匹配任务

### 基于文件系统的代理未加载

Claude Code 监视 `~/.claude/agents/` 和 `.claude/agents/`，并在几秒内拾取新的或编辑的代理文件，无需重启。如果定义从未出现，请排查这些原因：

* **新的 `agents` 目录**：监视程序仅覆盖会话启动时存在的目录，因此新目录中的第一个文件需要会话重启。这是最常见的原因。
* **无效的 frontmatter 或重复的 `name`**：检查文件的 YAML，以及现有代理是否已使用该 `name`。
* **`--disable-slash-commands`**：使用此标志启动的会话不监视这些目录，始终需要重启以加载新文件。
* **添加的目录下的文件**：Claude Code 从使用 `add_dirs`（Python）或 `additionalDirectories`（TypeScript）选项或 CLI 的 `--add-dir` 或 `/add-dir` 添加的目录加载 `.claude/agents/`，但不监视它们，因此那里的新文件或编辑文件需要会话重启。
* **具有相同名称的程序化代理**：传递给 `query()` 的 `agents` 会覆盖具有相同名称的文件系统代理。

有关文件格式，请参阅[如何编写子代理文件](https://code.claude.com/docs/zh-CN/sub-agents.md#write-subagent-files)。

## 相关文档

* [Claude Code 子代理](https://code.claude.com/docs/zh-CN/sub-agents.md)：子代理文档，包括基于文件系统的定义
* [动态工作流](https://code.claude.com/docs/zh-CN/workflows.md)：从脚本编排许多子代理，用于对话过大的工作
* [SDK 概述](https://code.claude.com/docs/zh-CN/agent-sdk/overview.md)：Claude Agent SDK 入门
