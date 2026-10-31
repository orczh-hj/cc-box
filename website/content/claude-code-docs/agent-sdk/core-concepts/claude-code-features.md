> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# 在 SDK 中使用 Claude Code 功能

> 将项目说明、skills、hooks 和其他 Claude Code 功能加载到您的 SDK 代理中。

Agent SDK 建立在与 Claude Code 相同的基础之上，这意味着您的 SDK 代理可以访问相同的基于文件系统的功能：项目说明（`CLAUDE.md` 和规则）、skills、hooks 等。

当您省略 `settingSources` 时，`query()` 读取与 Claude Code CLI 相同的文件系统设置：用户、项目和本地设置、CLAUDE.md 文件以及 `.claude/` skills、代理和命令。要在没有这些的情况下运行，请传递 `settingSources: []`，这会将代理限制为您以编程方式配置的内容。无论此选项如何，都会读取托管策略设置和全局 `~/.claude.json` 配置。请参阅 [settingSources 不控制的内容](#what-settingsources-does-not-control)。

## 使用 settingSources 控制文件系统设置

设置源选项（Python 中的 [`setting_sources`](https://code.claude.com/docs/zh-CN/agent-sdk/python#claudeagentoptions)、TypeScript 中的 [`settingSources`](https://code.claude.com/docs/zh-CN/agent-sdk/typescript#settingsource)）控制 SDK 加载哪些基于文件系统的设置。传递显式列表以选择加入特定源，或传递空数组以禁用用户、项目和本地设置。

此示例通过将 `settingSources` 设置为 `["user", "project"]` 来加载用户级和项目级设置：

```python Python theme={null}
from claude_agent_sdk import query, ClaudeAgentOptions, AssistantMessage, ResultMessage
import asyncio


async def main():
    async for message in query(
        prompt="Help me refactor the auth module",
        options=ClaudeAgentOptions(
            # "user" loads from ~/.claude/, "project" loads from ./.claude/ in cwd.
            # Together they give the agent access to CLAUDE.md, skills, hooks, and
            # permissions from both locations.
            setting_sources=["user", "project"],
            allowed_tools=["Read", "Edit", "Bash"],
        ),
    ):
        if isinstance(message, AssistantMessage):
            for block in message.content:
                if hasattr(block, "text"):
                    print(block.text)
        if isinstance(message, ResultMessage) and message.subtype == "success":
            print(f"\nResult: {message.result}")


asyncio.run(main())
```

```typescript TypeScript theme={null}
import { query } from "@anthropic-ai/claude-agent-sdk";

for await (const message of query({
  prompt: "Help me refactor the auth module",
  options: {
    // "user" loads from ~/.claude/, "project" loads from ./.claude/ in cwd.
    // Together they give the agent access to CLAUDE.md, skills, hooks, and
    // permissions from both locations.
    settingSources: ["user", "project"],
    allowedTools: ["Read", "Edit", "Bash"]
  }
})) {
  if (message.type === "assistant") {
    for (const block of message.message.content) {
      if (block.type === "text") console.log(block.text);
    }
  }
  if (message.type === "result" && message.subtype === "success") {
    console.log(`\nResult: ${message.result}`);
  }
}
```

当此代码运行时，助手的响应会打印到标准输出，然后在运行完成后打印最终结果行。

每个源从特定位置加载设置，其中 `<cwd>` 是您通过 `cwd` 选项传递的工作目录，或者如果未设置则为进程的当前目录。有关完整的类型定义，请参阅 [`SettingSource`](https://code.claude.com/docs/zh-CN/agent-sdk/typescript#settingsource)（TypeScript）或 [`SettingSource`](https://code.claude.com/docs/zh-CN/agent-sdk/python#settingsource)（Python）。

| 源 | 加载的内容 | 位置 |
| :- | :- | :- |
| `"project"` | 项目 `settings.json` 和 hooks；项目 CLAUDE.md 和 `.claude/rules/*.md`；项目 skills、commands 和 subagents | `<cwd>/.claude/` 用于 `settings.json` 和 hooks；`<cwd>` 和每个父目录用于 CLAUDE.md 和规则；`<cwd>` 和每个父目录直到存储库根目录用于 skills、commands 和 subagents，加上您通过 `additionalDirectories` 或 `add_dirs` 选项传递的每个目录的 `.claude/skills/`、`.claude/commands/` 和 `.claude/agents/` 文件夹，SDK 将其作为 [`--add-dir`](https://code.claude.com/docs/zh-CN/permissions#additional-directories-grant-file-access-not-configuration) 传递给 Claude Code |
| `"user"` | 用户 `settings.json`；用户 CLAUDE.md 和 `~/.claude/rules/*.md`；用户 skills、commands 和 subagents | `~/.claude/` 用于 `settings.json`、CLAUDE.md 和规则；`~/.claude/skills/`、`~/.claude/commands/` 和 `~/.claude/agents/` 用于 skills、commands 和 subagents |
| `"local"` | CLAUDE.local.md、`.claude/settings.local.json` | `<cwd>/.claude/` 用于 `settings.local.json`，当 `<cwd>` 位于 git 仓库内时，该文件[也可以从仓库根目录加载](https://code.claude.com/docs/zh-CN/settings.md#where-claude-code-keeps-the-local-file-in-a-git-repository)；`<cwd>` 和每个父目录用于 CLAUDE.local.md |

省略 `settingSources` 等同于 `["user", "project", "local"]`。

`cwd` 选项确定 SDK 查找项目级输入的位置。项目 `settings.json` 从 `<cwd>/.claude/` 加载，没有父目录回退。

### settingSources 不控制的内容

`settingSources` 涵盖用户、项目和本地设置。无论其值如何，都会读取一些输入：

| 输入 | 行为 | 禁用方式 |
| :- | :- | :- |
| 托管策略设置 | 端点管理的策略（例如 MDM plist、注册表策略或托管设置文件）从主机加载。[服务器管理的设置](https://code.claude.com/docs/zh-CN/server-managed-settings.md)在会话使用符合条件的凭证（例如组织 OAuth 登录、直接配置的 API 密钥或 `user_oauth` [Anthropic 配置文件](https://code.claude.com/docs/zh-CN/authentication.md#anthropic-profiles-and-federation-credentials)）进行身份验证时，在[符合条件的配置](https://code.claude.com/docs/zh-CN/server-managed-settings.md#platform-availability)上获取 | 端点策略：从主机中删除托管设置文件、plist 或注册表策略。服务器管理的设置：由您的 Claude 组织中的[所有者](https://code.claude.com/docs/zh-CN/server-managed-settings.md#access-control)控制；您无法从 SDK 禁用它们 |
| `~/.claude.json` 全局配置 | 始终读取 | 使用 `env` 中的 `CLAUDE_CONFIG_DIR` 重新定位 |
| `~/.claude/projects/<project>/memory/` 处的自动内存 | 在会话启动时加载到系统提示中。代理使用标准 `Write` 和 `Edit` 工具而不是专用内存工具在那里写入新内存，因此必须启用这些工具才能让代理保存内存 | 在设置中设置 `autoMemoryEnabled: false`，或在 `env` 中设置 `CLAUDE_CODE_DISABLE_AUTO_MEMORY=1` |
| [claude.ai MCP 连接器](https://code.claude.com/docs/zh-CN/mcp.md#use-mcp-servers-from-claude-ai) | 当会话使用您的 claude.ai 登录进行身份验证时加载。当 `CLAUDE_CODE_OAUTH_TOKEN` 持有来自 [`claude setup-token`](https://code.claude.com/docs/zh-CN/authentication#generate-a-long-lived-token) 的令牌时不加载，该令牌只能进行模型请求。传递 `mcpServers: {}` 不会抑制连接器 | 设置 `strictMcpConfig: true`、[`disableClaudeAiConnectors: true`](https://code.claude.com/docs/zh-CN/mcp#disable-claude-ai-connectors) 在设置中，或在 `env` 中设置 `ENABLE_CLAUDEAI_MCP_SERVERS=false` |
| [`sandbox.credentials`](https://code.claude.com/docs/zh-CN/sandboxing#protect-credentials) `deny` 条目和 `~/.claude/settings.json` 中的文件 `mask` 条目 | 当[命令沙箱](https://code.claude.com/docs/zh-CN/sandboxing.md)运行时，Claude Code 应用 `deny` 条目，并将 `credentials.files` `mask` 条目保持为限制，即使 `settingSources` 排除用户设置。Claude Code 仅使用这些条目来缩小沙箱命令可以访问的内容 | 从 `~/.claude/settings.json` 中删除条目 |

> [!WARNING]
>
> 不要依赖默认 `query()` 选项进行多租户隔离。因为上述输入无论 `settingSources` 如何都会被读取，SDK 进程可能会获取主机级配置和按目录内存。对于多租户部署，在自己的文件系统中运行每个租户，并设置 `settingSources: []` 加上 `env` 中的 `CLAUDE_CODE_DISABLE_AUTO_MEMORY=1`。[服务器管理的设置](https://code.claude.com/docs/zh-CN/server-managed-settings.md)在进程使用组织凭证进行身份验证时获取；文件系统隔离不会删除它们。请参阅[安全部署](https://code.claude.com/docs/zh-CN/agent-sdk/secure-deployment.md)。

## 项目说明（CLAUDE.md 和规则）

`CLAUDE.md` 文件和 `.claude/rules/*.md` 文件为您的代理提供关于您的项目的持久上下文：编码约定、构建命令、架构决策和说明。当 `settingSources` 包含 `"project"`（如 [`settingSources` 示例](#control-filesystem-settings-with-settingsources)中所示）时，SDK 在会话开始时将这些文件加载到上下文中。然后代理遵循您的项目约定，而无需在每个提示中重复它们。

### CLAUDE.md 加载位置

| 级别 | 位置 | 加载时间 |
| :- | :- | :- |
| 项目（根） | `<cwd>/CLAUDE.md` 或 `<cwd>/.claude/CLAUDE.md` | `settingSources` 包含 `"project"` |
| 项目规则 | `<cwd>/.claude/rules/*.md` 和 `.claude/rules/*.md` 在每个父目录中 | `settingSources` 包含 `"project"` |
| 项目（父目录） | `cwd` 上方目录中的 `CLAUDE.md` 文件 | `settingSources` 包含 `"project"`，在会话开始时加载 |
| 项目（子目录） | `cwd` 子目录中的 `CLAUDE.md` 文件 | `settingSources` 包含 `"project"`，[按需](https://code.claude.com/docs/zh-CN/memory.md#how-claude-md-files-load)加载 |
| 本地 | `<cwd>/CLAUDE.local.md` 和 `CLAUDE.local.md` 在每个父目录中 | `settingSources` 包含 `"local"` |
| 用户 | `~/.claude/CLAUDE.md` | `settingSources` 包含 `"user"` |
| 用户规则 | `~/.claude/rules/*.md` | `settingSources` 包含 `"user"` |

所有级别都是累加的：如果项目和用户 CLAUDE.md 文件都存在，代理会看到两者。级别之间没有硬优先级规则；如果说明冲突，结果取决于 Claude 如何解释它们。编写不冲突的规则，或在更具体的文件中明确说明优先级（"这些项目说明覆盖任何冲突的用户级默认值"）。

> [!TIP]
>
> 您也可以通过 `systemPrompt` 直接注入上下文，而无需使用 CLAUDE.md 文件。请参阅 [修改系统提示](https://code.claude.com/docs/zh-CN/agent-sdk/modifying-system-prompts.md)。当您希望在交互式 Claude Code 会话和 SDK 代理之间共享相同的上下文时，使用 CLAUDE.md。

有关如何构建和组织 CLAUDE.md 内容，请参阅 [管理 Claude 的内存](https://code.claude.com/docs/zh-CN/memory.md)。

## Skills

Skills 是 markdown 文件，为您的代理提供专业知识和可调用的工作流。与 `CLAUDE.md`（每个会话都加载）不同，skills 按需加载。代理在启动时接收 skill 描述，并在相关时加载完整内容。

Skills 通过 `settingSources` 从文件系统中发现。当 `query()` 上的 `skills` 选项被省略时，发现的用户和项目 skills 会被启用，Skill 工具可用，与 CLI 行为相匹配。要控制启用哪些 skills，请将 `skills` 作为 `"all"`、skill 名称列表或 `[]` 传递以禁用所有。当设置 `skills` 时，SDK 会自动将 Skill 工具添加到 `allowedTools`。如果您还传递了显式的 `tools` 列表，请在该列表中包含 `"Skill"`，以便 Claude 可以调用 skills。

```python Python theme={null}
from claude_agent_sdk import query, ClaudeAgentOptions, ResultMessage
import asyncio


# Skills in .claude/skills/ are discovered automatically
# when settingSources includes "project"
async def main():
    async for message in query(
        prompt="Review this PR using our code review checklist",
        options=ClaudeAgentOptions(
            setting_sources=["user", "project"],
            skills="all",
            allowed_tools=["Read", "Grep", "Glob"],
        ),
    ):
        if isinstance(message, ResultMessage) and message.subtype == "success":
            print(message.result)


asyncio.run(main())
```

```typescript TypeScript theme={null}
import { query } from "@anthropic-ai/claude-agent-sdk";

// Skills in .claude/skills/ are discovered automatically
// when settingSources includes "project"
for await (const message of query({
  prompt: "Review this PR using our code review checklist",
  options: {
    settingSources: ["user", "project"],
    skills: "all",
    allowedTools: ["Read", "Grep", "Glob"]
  }
})) {
  if (message.type === "result" && message.subtype === "success") {
    console.log(message.result);
  }
}
```

> [!NOTE]
>
> Skills 必须创建为文件系统工件（`.claude/skills/<name>/SKILL.md`）。SDK 没有用于注册 skills 的编程 API。有关完整详情，请参阅 [SDK 中的 Agent Skills](https://code.claude.com/docs/zh-CN/agent-sdk/skills.md)。

## Hooks

SDK 支持两种定义 hooks 的方式，它们并行运行：

* **文件系统 hooks：** 在 `settings.json` 中定义的 shell 命令，当 `settingSources` 包含相关源时加载。这些与您为 [交互式 Claude Code 会话](https://code.claude.com/docs/zh-CN/hooks-guide.md) 配置的 hooks 相同。
* **编程 hooks：** 直接传递给 `query()` 的回调函数。这些在您的应用程序进程中运行，可以返回结构化决策。请参阅 [使用 hooks 控制执行](https://code.claude.com/docs/zh-CN/agent-sdk/hooks.md)。

Hook 回调接收工具输入并返回决策字典。返回 `{}` 意味着允许工具继续。要阻止执行，返回一个 `hookSpecificOutput` 对象，其中包含 `permissionDecision: "deny"` 和 `permissionDecisionReason`。原因会作为工具结果发送给 Claude。请参阅 [hooks 指南](https://code.claude.com/docs/zh-CN/agent-sdk/hooks.md) 了解完整的回调签名和返回类型。

```python Python theme={null}
from claude_agent_sdk import query, ClaudeAgentOptions, HookMatcher, ResultMessage
import asyncio


# PreToolUse hook callback. Positional args:
#   input_data: HookInput dict with tool_name, tool_input, hook_event_name
#   tool_use_id: str | None, the ID of the tool call being intercepted
#   context: HookContext, reserved for future abort-signal support
async def audit_bash(input_data, tool_use_id, context):
    command = input_data.get("tool_input", {}).get("command", "")
    if "rm -rf" in command:
        return {
            "hookSpecificOutput": {
                "hookEventName": "PreToolUse",
                "permissionDecision": "deny",
                "permissionDecisionReason": "Destructive command blocked",
            }
        }
    return {}  # Empty dict: allow the tool to proceed


# Filesystem hooks from .claude/settings.json run automatically
# when settingSources loads them. You can also add programmatic hooks:
async def main():
    async for message in query(
        prompt="Refactor the auth module",
        options=ClaudeAgentOptions(
            setting_sources=["project"],  # Loads hooks from .claude/settings.json
            hooks={
                "PreToolUse": [
                    HookMatcher(matcher="Bash", hooks=[audit_bash]),
                ]
            },
        ),
    ):
        if isinstance(message, ResultMessage) and message.subtype == "success":
            print(message.result)


asyncio.run(main())
```

```typescript TypeScript theme={null}
import { query, type HookInput, type HookJSONOutput } from "@anthropic-ai/claude-agent-sdk";

// PreToolUse hook callback. HookInput is a discriminated union on
// hook_event_name, so narrowing on it gives TypeScript the right
// tool_input shape for this event.
const auditBash = async (input: HookInput): Promise<HookJSONOutput> => {
  if (input.hook_event_name !== "PreToolUse") return {};
  const toolInput = input.tool_input as { command?: string };
  if (toolInput.command?.includes("rm -rf")) {
    return {
      hookSpecificOutput: {
        hookEventName: "PreToolUse",
        permissionDecision: "deny",
        permissionDecisionReason: "Destructive command blocked",
      },
    };
  }
  return {}; // Empty object: allow the tool to proceed
};

// Filesystem hooks from .claude/settings.json run automatically
// when settingSources loads them. You can also add programmatic hooks:
for await (const message of query({
  prompt: "Refactor the auth module",
  options: {
    settingSources: ["project"], // Loads hooks from .claude/settings.json
    hooks: {
      PreToolUse: [{ matcher: "Bash", hooks: [auditBash] }]
    }
  }
})) {
  if (message.type === "result" && message.subtype === "success") {
    console.log(message.result);
  }
}
```

### 何时使用哪种 hook 类型

| Hook 类型 | 最适合 |
| :- | :- |
| **文件系统**（`settings.json`） | 在 CLI 和 SDK 会话之间共享 hooks。支持 `"command"`（shell 脚本）、`"http"`（POST 到端点）、`"mcp_tool"`（调用连接的 MCP 服务器的工具）、`"prompt"`（LLM 评估提示）和 `"agent"`（生成验证器代理）。这些在主代理和它生成的任何子代理中触发。 |
| **编程**（`query()` 中的回调） | 应用程序特定的逻辑、结构化决策和进程内集成。这些也在子代理内触发。hook 输入（回调的第一个参数）携带 `agent_id` 和 `agent_type` 字段，用于识别哪个代理触发了 hook。 |

> [!NOTE]
>
> TypeScript SDK 支持超出 Python 的其他 hook 事件，包括 `SessionStart`、`SessionEnd`、`TeammateIdle` 和 `TaskCompleted`。有关完整的事件兼容性表，请参阅 [hooks 指南](https://code.claude.com/docs/zh-CN/agent-sdk/hooks.md)。

有关编程 hooks 的完整详情，请参阅 [使用 hooks 控制执行](https://code.claude.com/docs/zh-CN/agent-sdk/hooks.md)。有关文件系统 hook 语法，请参阅 [Hooks](https://code.claude.com/docs/zh-CN/hooks.md)。

## 选择正确的功能

Agent SDK 为您提供了多种方式来扩展代理的行为。如果您不确定使用哪种，此表将常见目标映射到正确的方法。

| 您想要做什么 | 使用 | SDK 表面 |
| :- | :- | :- |
| 设置代理始终遵循的项目约定 | [CLAUDE.md](https://code.claude.com/docs/zh-CN/memory.md) | `settingSources: ["project"]` 自动加载它 |
| 为代理提供它在相关时加载的参考材料 | [Skills](https://code.claude.com/docs/zh-CN/agent-sdk/skills.md) | `settingSources` + `skills` 选项 |
| 运行可重用的工作流（部署、审查、发布） | [用户可调用的 skills](https://code.claude.com/docs/zh-CN/agent-sdk/skills.md) | `settingSources` + `skills` 选项 |
| 将隔离的子任务委托给新的上下文（研究、审查） | [子代理](https://code.claude.com/docs/zh-CN/agent-sdk/subagents.md) | `agents` 参数 + `allowedTools: ["Agent"]` |
| 协调多个 Claude Code 实例，具有共享任务列表和直接的代理间消息传递 | [代理团队](https://code.claude.com/docs/zh-CN/agent-teams.md) | 不直接通过 SDK 选项配置。代理团队是一个 CLI 功能，其中一个会话充当团队负责人，协调独立队友之间的工作 |
| 在工具调用上运行确定性逻辑（审计、阻止、转换） | [Hooks](https://code.claude.com/docs/zh-CN/agent-sdk/hooks.md) | `hooks` 参数带回调，或通过 `settingSources` 加载的 shell 脚本 |
| 为 Claude 提供对外部服务的结构化工具访问 | [MCP](https://code.claude.com/docs/zh-CN/agent-sdk/mcp.md) | `mcpServers` 参数 |

您启用的每个功能都会增加代理的上下文窗口。有关每个功能的成本以及这些功能如何分层组合，请参阅 [扩展 Claude Code](https://code.claude.com/docs/zh-CN/features-overview.md#understand-context-costs)。

## 相关资源

* [扩展 Claude Code](https://code.claude.com/docs/zh-CN/features-overview.md)：所有扩展功能的概念概述，包含比较表和上下文成本分析
* [SDK 中的 Skills](https://code.claude.com/docs/zh-CN/agent-sdk/skills.md)：使用 skills 的完整指南
* [子代理](https://code.claude.com/docs/zh-CN/agent-sdk/subagents.md)：为隔离的子任务定义和调用子代理
* [Hooks](https://code.claude.com/docs/zh-CN/agent-sdk/hooks.md)：在关键执行点拦截和控制代理行为
* [权限](https://code.claude.com/docs/zh-CN/agent-sdk/permissions.md)：使用模式、规则和回调控制工具访问
* [系统提示](https://code.claude.com/docs/zh-CN/agent-sdk/modifying-system-prompts.md)：在不使用 CLAUDE.md 文件的情况下注入上下文
