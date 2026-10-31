> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# 使用 skills 扩展 agents

> 控制 Claude 在 Claude Agent SDK 会话中可以调用哪些 skills，按名称分派命令，以及编写会话发现的 skills

Agent Skills 通过专业能力扩展 Claude，Claude 会在相关时自动调用这些能力。Skills 被打包为 `SKILL.md` 文件，包含说明、描述和可选的支持资源。本页还涵盖了 [Agent SDK 会话中的命令](#commands-in-agent-sdk-sessions)。

有关 skill 的更多信息，包括优势、架构和编写指南，请参阅 [Agent Skills 概述](https://platform.claude.com/docs/en/agents-and-tools/agent-skills/overview)。

## Skills 如何与 Agent SDK 配合使用

使用 Claude Agent SDK 时，skills 的工作方式如下：

* **定义为文件系统工件**：你在自己的目录中创建每个 skill 作为 `SKILL.md` 文件，例如 `.claude/skills/<name>/SKILL.md`
* **从文件系统加载**：SDK 从由 `settingSources`（TypeScript）或 `setting_sources`（Python）管理的文件系统位置加载 skills
* **自动发现**：加载文件系统设置后，SDK 在启动时从用户和项目目录发现 skill 元数据，当 Claude 调用 skill 时加载完整内容
* **由模型调用**：Claude 根据上下文自动选择何时使用它们
* **由用户调用**：你可以通过在提示中发送 `/<name>` 直接分派 skill。请参阅 [Agent SDK 会话中的命令](#commands-in-agent-sdk-sessions)
* **通过 `skills` 选项进行范围限制**：发现的 skills 默认启用。传递 skill 名称列表、`"all"` 或 `[]` 来控制 Claude 可以调用哪些 skills

与子代理不同，子代理可以在 [`agents` 选项](https://code.claude.com/docs/zh-CN/agent-sdk/subagents#programmatic-definition-recommended) 中定义，你创建 skills 作为磁盘上的文件。SDK 不提供用于注册它们的编程 API。

> [!NOTE]
>
> Skills 通过文件系统设置源发现。使用默认 `query()` 选项时，SDK 加载用户和项目源，因此 `~/.claude/skills/`、`<cwd>/.claude/skills/` 和 `<cwd>` 到存储库根目录之间任何父目录中的 `.claude/skills/` 中的 skills 可用。项目源还涵盖你通过 `additionalDirectories`（TypeScript）或 `add_dirs`（Python）传递的每个目录中的 `<dir>/.claude/skills/`，因为 SDK 将这些目录作为 [`--add-dir`](https://code.claude.com/docs/zh-CN/skills#skills-from-additional-directories) 传递给 Claude Code。如果你显式设置 `settingSources`，请包含 `'project'` 以保持项目和添加目录 skills，包含 `'user'` 以保持你的个人 skills，或使用 [`plugins` 选项](https://code.claude.com/docs/zh-CN/agent-sdk/plugins) 从特定路径加载 skills。

## 在 Agent SDK 中使用 skill

在 `query()` 上设置 `skills` 选项以控制会话中 Claude 可以调用哪些 skill。省略时，发现的 skill 启用且 Skill 工具可用，与 CLI 行为匹配。传递 `"all"` 以让 Claude 调用每个发现的 skill，传递 skill 名称列表以仅允许那些，或传递 `[]` 以让 Claude 不调用任何 skill。

例如，要让 Claude 仅调用两个命名的 skill：

```python Python theme={null}
options = ClaudeAgentOptions(skills=["pdf", "docx"])
```

```typescript TypeScript theme={null}
const options = { skills: ["pdf", "docx"] };
```

### 在会话中设置 skill

当您设置 `skills` 时，SDK 自动将 Skill 工具添加到 `allowedTools`。如果您还传递显式 `tools` 列表，请在该列表中包含 `"Skill"`，以便 Claude 可以调用 skill。

配置后，Claude 自动从文件系统发现 skill 并在与用户请求相关时调用它们。

以下示例在会话中启用每个发现的 skill，并预先批准 skill 通常需要的工具。该示例将 `cwd` 设置为进程的当前工作目录，因此请在当前目录或直到仓库根目录的任何父目录中具有 `.claude/skills/` 目录的项目内运行它：

```python Python theme={null}
import asyncio
import os

from claude_agent_sdk import query, ClaudeAgentOptions


async def main():
    options = ClaudeAgentOptions(
        cwd=os.getcwd(),  # .claude/skills/ here or in a parent directory
        setting_sources=["user", "project"],  # Load skills from filesystem
        skills="all",  # Let Claude invoke every discovered skill
        allowed_tools=["Read", "Write", "Bash"],
    )

    async for message in query(
        prompt="Help me process this PDF document", options=options
    ):
        print(message)


asyncio.run(main())
```

```typescript TypeScript theme={null}
import { query } from "@anthropic-ai/claude-agent-sdk";

for await (const message of query({
  prompt: "Help me process this PDF document",
  options: {
    cwd: process.cwd(), // .claude/skills/ here or in a parent directory
    settingSources: ["user", "project"], // Load skills from filesystem
    skills: "all", // Let Claude invoke every discovered skill
    allowedTools: ["Read", "Write", "Bash"]
  }
})) {
  console.log(message);
}
```

### 确认 skill 已加载

在流的开始附近，SDK 产生一个子类型为 `init` 的系统消息。检查其 `skills` 数组以在 Claude 开始工作前确认您的 skill 已加载。该数组包括您通过 `description` 或 `when_to_use` frontmatter 字段定义的用户可调用 skill，以及 [Claude Code 随附 skill](https://code.claude.com/docs/zh-CN/skills.md#bundled-skills)。

该数组仅列出用户可调用的 skill。在其 frontmatter 中具有 [`user-invocable: false`](https://code.claude.com/docs/zh-CN/skills#control-who-invokes-a-skill) 的 skill 会加载并保持对 Claude 可用，但不会出现在数组中。该数组列出相同的 skill，无论它们是否在您的 `skills` 列表中。

### 仅允许特定 skill

要让 Claude 仅调用特定 skill，请在 `skills` 列表中传递它们的名称：

* **您自己的 skill**：skill 的目录名称
* **插件 skill**：`plugin:skill`
* **`additionalDirectories` 或 `add_dirs` 中的名称冲突**：当这些目录之一中的 skill 与企业、个人或项目 skill，或与先前添加的目录中的 skill 名称冲突时，它会获得一个[带前缀的名称](https://code.claude.com/docs/zh-CN/skills.md#how-a-skill-gets-its-command-name)。请按该名称列出它。如果 `../api` 和 `../web` 中各有一个 `verify` skill，按此顺序添加，且没有其他 `verify` skill，则将它们列为 `verify` 和 `web:verify`。需要 Claude Code v2.1.309 或更高版本

该列表仅接受确切的 skill 名称。如果一个条目不能作为确切名称工作，`query()` 会在会话开始前拒绝该列表。请参阅 [Invalid skill name error](#invalid-skill-name-error) 了解名称规则和每个 SDK 引发的错误。

模型看不到未列出的 skill，Skill 工具会拒绝它们，而它们的文件保留在磁盘上，可通过 Read 和 Bash 访问。限制列表不会限制 [按名称分派](#dispatch-commands-by-name)。

要让 Claude 调用每个发现的 skill，传递 `skills: "all"` 而不是通配符。

## Agent SDK 会话中的命令

本部分是 SDK 的命令文档。命令是你通过在提示中发送 `/<name>` 来运行的任何内容。命令表面上的条目在支持它们的内容上有所不同：

* **内置命令**：执行编码到 SDK 运行的 Claude Code 进程中的逻辑，例如 `/compact`
* **捆绑 skills**：Claude Code 包含的提示工件，例如 `/code-review`
* **你的 skills**：你编写的提示工件，每个都是一个包含 `SKILL.md` 文件的目录。用户可调用 skill 的名称自动加入表面，因此分派你自己的 `/security-check` 和运行内置的工作方式相同
* **自定义命令文件**：一种较旧的工件形式，具有相同的行为，`.claude/commands/` 中的平面 Markdown 文件，其文件名成为命令名称。Skills 是它们推荐的后继者

默认情况下，你和 Claude 都可以调用任何 skill。你可以通过 skill 的 [frontmatter](https://code.claude.com/docs/zh-CN/skills.md#control-who-invokes-a-skill) 限制任一路径。有关命令和 skill 的定义，请参阅词汇表的 [Command](https://code.claude.com/docs/zh-CN/glossary.md#command) 和 [Skill](https://code.claude.com/docs/zh-CN/glossary.md#skill) 条目。请参阅 [Claude Code 中的命令](https://code.claude.com/docs/zh-CN/commands.md) 了解每个内置命令，以及 [使用 skills 扩展 Claude](https://code.claude.com/docs/zh-CN/skills.md) 了解两种工件形式的完整指南。

### 发现可用命令

你可以通过 SDK 分派不需要交互式终端的命令。`system/init` 消息在其 `slash_commands` 字段中列出会话中可用的命令。需要交互式终端的命令，例如 `/theme` 和 `/terminal-setup`，不会出现在列表中。在会话开始时访问该字段：

```typescript TypeScript theme={null}
import { query } from "@anthropic-ai/claude-agent-sdk";

for await (const message of query({
  prompt: "Hello Claude",
  options: { maxTurns: 1 }
})) {
  if (message.type === "system" && message.subtype === "init") {
    console.log("Available commands:", message.slash_commands);
  }
}
```

```python Python theme={null}
import asyncio
from claude_agent_sdk import query, ClaudeAgentOptions, SystemMessage


async def main():
    async for message in query(prompt="Hello Claude", options=ClaudeAgentOptions(max_turns=1)):
        if isinstance(message, SystemMessage) and message.subtype == "init":
            print("Available commands:", message.data["slash_commands"])


asyncio.run(main())
```

打印的列表混合了内置命令、捆绑 skills、你的用户可调用 skills 和 `.claude/commands/` 文件：

```text theme={null}
Available commands: ["clear", "compact", "context", "usage", "code-review", "verify", "security-check", ...]
```

在其 frontmatter 中具有 [`user-invocable: false`](https://code.claude.com/docs/zh-CN/skills#control-who-invokes-a-skill) 的 skill 不会出现在此列表或 [确认 skills 已加载](#confirm-skills-loaded) 中的 `skills` 数组中。配置 [MCP servers](https://code.claude.com/docs/zh-CN/agent-sdk/mcp.md) 的会话也可以公开 [MCP prompts 作为命令](https://code.claude.com/docs/zh-CN/mcp.md#use-mcp-prompts-as-commands)。

### 按名称分派命令

通过在提示字符串中包含命令来发送命令，就像发送常规文本一样。分派不依赖于 `skills` 选项。发送 `/<name>` 会运行用户可调用的 skill，即使你的 `skills` 列表省略了它。作用于对话历史的命令，例如 `/compact`，需要先前的消息才能工作。

一个 `/<name>` 既不匹配会话中的命令也不匹配内置 Claude Code 命令不会导致查询失败。Claude Code 将提示作为普通消息发送给 Claude，并附注该命令未运行，因此查询花费一个模型轮次并返回 Claude 的回复。在 v2.1.274 之前，匹配不到任何内容的 `/<name>` 返回 `Unknown command: /<name>` 作为结果，不花费模型轮次。

一个 `/<name>` 匹配在会话中不可用的内置 Claude Code 命令，例如 `/theme`，返回 `/theme isn't available in this environment.` 作为结果，不花费模型轮次。

> [!NOTE]
>
> 命令可以像任何其他提示一样触及 `maxTurns` / `max_turns` 限制，以错误结果而不是 `success` 结束查询。有关错误结果合约，请参阅 [处理结果](https://code.claude.com/docs/zh-CN/agent-sdk/agent-loop.md#handle-the-result)。如果你的命令可能触及限制，请在 TypeScript 中用 `try`/`catch` 或在 Python 中用 `try`/`except` 包装循环，如 [单消息输入](https://code.claude.com/docs/zh-CN/agent-sdk/streaming-vs-single-mode.md#single-message-input) 中所示，或设置 `maxTurns` 足够高以完成工作。

### 使用 `/compact` 压缩历史

`/compact` 命令通过总结较旧的消息同时保留重要上下文来减少对话历史的大小。压缩需要现有对话和足够的先前消息来总结。此示例首先进行对话，然后压缩它并读取报告结果的 `compact_boundary` 系统消息：

```typescript TypeScript theme={null}
import { query } from "@anthropic-ai/claude-agent-sdk";

// Compaction needs existing history, so have a conversation first
try {
  for await (const message of query({
    prompt: "Explain what this project does",
    options: { maxTurns: 2 }
  })) {
    if (message.type === "result" && message.subtype === "success") {
      console.log(message.result);
    }
  }
} catch (error) {
  // A single-shot query() throws after yielding an error result,
  // so the follow-up query below still runs.
  console.error(`Session ended with an error: ${error}`);
}

// Compact the same conversation
for await (const message of query({
  prompt: "/compact",
  options: { continue: true, maxTurns: 1 }
})) {
  if (message.type === "system" && message.subtype === "compact_boundary") {
    console.log("Compaction completed");
    console.log("Pre-compaction tokens:", message.compact_metadata.pre_tokens);
    console.log("Trigger:", message.compact_metadata.trigger);
    // Example output:
    // Compaction completed
    // Pre-compaction tokens: 1842
    // Trigger: manual
  }
}
```

```python Python theme={null}
import asyncio
from claude_agent_sdk import query, ClaudeAgentOptions, ResultMessage, SystemMessage


async def main():
    # Compaction needs existing history, so have a conversation first
    try:
        async for message in query(
            prompt="Explain what this project does",
            options=ClaudeAgentOptions(max_turns=2),
        ):
            if isinstance(message, ResultMessage) and message.subtype == "success":
                print(message.result)
    except Exception as error:
        # A single-shot query() raises after yielding an error result,
        # so the follow-up query below still runs.
        print(f"Session ended with an error: {error}")

    # Compact the same conversation
    async for message in query(
        prompt="/compact",
        options=ClaudeAgentOptions(continue_conversation=True, max_turns=1),
    ):
        if isinstance(message, SystemMessage) and message.subtype == "compact_boundary":
            print("Compaction completed")
            print("Pre-compaction tokens:", message.data["compact_metadata"]["pre_tokens"])
            print("Trigger:", message.data["compact_metadata"]["trigger"])
            # Example output:
            # Compaction completed
            # Pre-compaction tokens: 1842
            # Trigger: manual


asyncio.run(main())
```

> [!NOTE]
>
> `compact_boundary` 消息仅在压缩运行时到达。当继续的会话有消息但 `/compact` 无法总结任何内容时，运行仍以 `success` 结果结束而不是错误，并且没有 `compact_boundary` 消息到达。结果文本随后会携带原因，例如当会话仅包含提示但尚未收到 Claude 的回复时 `Not enough messages to compact.`。新的单次 `query()` 调用以空上下文开始，因此在具有先前轮次的会话中使用此模式，例如在 [流式输入模式](https://code.claude.com/docs/zh-CN/agent-sdk/streaming-vs-single-mode.md) 中或恢复会话时。

### 使用 `/clear` 重置上下文

`/clear` 命令将对话重置为空上下文，因此后续提示以没有先前对话历史开始。先前的对话保留在磁盘上。你可以通过将其会话 ID 传递给 [`resume` 选项](https://code.claude.com/docs/zh-CN/agent-sdk/sessions#resume-by-id) 返回到该对话。

`/clear` 在 [流式输入模式](https://code.claude.com/docs/zh-CN/agent-sdk/streaming-vs-single-mode.md) 中很有用，你在单个连接上发送多个提示。对于单次 `query()` 调用，每个调用已经以空上下文开始，因此发送 `/clear` 没有实际效果。改为启动新的 `query()`。

## 创建 skills

将每个 skill 创建为包含带有 YAML frontmatter 和 Markdown 内容的 `SKILL.md` 文件的目录。`description` 字段确定 Claude 何时调用你的 skill。

**示例目录结构**：

```text theme={null}
.claude/skills/security-check/
└── SKILL.md
```

### 选择发现级别

在两个最常见的 [发现级别](https://code.claude.com/docs/zh-CN/skills.md#where-skills-live) 中的任一个保存 skills：

* **项目 skills**：`.claude/skills/`，仅在当前项目中可用
* **个人 skills**：`~/.claude/skills/`，在所有项目中可用

如果你在 `.claude/commands/` 中有现有的自定义命令文件，它们会继续工作。`.claude/commands/deploy.md` 中的命令文件创建 `/deploy` 并以与 `.claude/skills/deploy/SKILL.md` 中的 skill 相同的方式工作。如果命令文件和 skill 共享名称，请参阅 [解决共享名称的 skills](https://code.claude.com/docs/zh-CN/skills.md#resolve-skills-that-share-a-name) 了解哪一个运行。SDK 从与 skills 相同的两个范围加载 `.claude/commands/` 和 `~/.claude/commands/` 文件。请参阅 [使用 skills 扩展 Claude](https://code.claude.com/docs/zh-CN/skills.md) 了解两种工件形式的完整指南。

### 创建并分派你的第一个 skill

要查看完整流程，创建 `.claude/skills/security-check/SKILL.md`：

```markdown theme={null}
---
name: security-check
description: Run a security vulnerability scan
---

Analyze the codebase for security vulnerabilities including:
- SQL injection risks
- XSS vulnerabilities
- Exposed credentials
- Insecure configurations
```

文件存在后，skill 可通过 SDK 使用。当请求与其描述匹配时 Claude 调用它，你可以直接分派它：

```typescript TypeScript theme={null}
import { query } from "@anthropic-ai/claude-agent-sdk";

for await (const message of query({
  prompt: "/security-check",
  options: { maxTurns: 10 }
})) {
  if (message.type === "result" && message.subtype === "success") {
    console.log(message.result);
  }
}
```

```python Python theme={null}
import asyncio
from claude_agent_sdk import query, ClaudeAgentOptions, ResultMessage


async def main():
    async for message in query(
        prompt="/security-check", options=ClaudeAgentOptions(max_turns=10)
    ):
        if isinstance(message, ResultMessage) and message.subtype == "success":
            print(message.result)


asyncio.run(main())
```

成功的运行以 `success` 结果结束，其文本携带扫描发现。针对具有植入问题的小型 Express 应用，结果文本开始于：

```text theme={null}
**Security scan of `app.js` — 4 findings (most severe first):**

1. **SQL Injection** (line 8) — `req.query.name` is concatenated directly into the SQL string. Trivially exploitable (`' OR '1'='1`, `'; DROP TABLE users;--`). **Fix:** use parameterized queries, e.g. `db.query("SELECT * FROM users WHERE name = ?", [req.query.name], cb)`.
...
```

skill 的名称也出现在 init 消息的 `slash_commands` 数组中。

> [!NOTE]
>
> Claude Code 包含捆绑的 `code-review` 和 `verify` skills。如果你在 `.claude/commands/` 中命名一个文件为其中之一，例如 `.claude/commands/code-review.md`，文件的命令会遮蔽捆绑的 skill，`slash_commands` 列出名称一次。

## 为 skills 预先批准工具

> [!NOTE]
>
> 在 SDK 会话中，您可以通过 skill 的 [`allowed-tools`](https://code.claude.com/docs/zh-CN/skills#pre-approve-tools-for-a-skill) frontmatter，或通过查询配置中的 `allowedTools` 选项（Python 中的 `allowed_tools`），为项目或个人 skill 预先批准工具。如果您的组织在托管设置中设置了 [`allowManagedPermissionRulesOnly`](https://code.claude.com/docs/zh-CN/settings-reference#allowmanagedpermissionrulesonly)，Claude Code 会忽略这两者。[从 claude.ai 同步的](https://code.claude.com/docs/zh-CN/skills.md#how-claude-code-handles-the-frontmatter-of-a-synced-skill) skill 遵循其自身的 frontmatter 规则。

Skills 使用会话的工具运行。下面的示例使用 `allowedTools`（Python 中的 `allowed_tools`）预先批准 `Read`、`Grep` 和 `Glob`，因此 Claude 可以在运行 [security-check skill](#create-and-dispatch-your-first-skill) 时检查文件，而无需停止以获得批准：

```python Python theme={null}
import asyncio

from claude_agent_sdk import query, ClaudeAgentOptions

options = ClaudeAgentOptions(
    setting_sources=["user", "project"],  # Load skills from filesystem
    skills="all",
    allowed_tools=["Read", "Grep", "Glob"],
)


async def main():
    async for message in query(prompt="Check this project for security issues", options=options):
        print(message)


asyncio.run(main())
```

```typescript TypeScript theme={null}
import { query } from "@anthropic-ai/claude-agent-sdk";

for await (const message of query({
  prompt: "Check this project for security issues",
  options: {
    settingSources: ["user", "project"], // Load skills from filesystem
    skills: "all",
    allowedTools: ["Read", "Grep", "Glob"]
  }
})) {
  console.log(message);
}
```

在流中，skill 调用显示为 Skill 工具使用，然后是对项目文件的 Read 调用。运行以 `success` 结果结束，其文本携带发现。

该列表预先批准命名的工具而不是限制其他工具。有关完整权限流程，包括权限模式和 `canUseTool` 回调，请参阅 [权限](https://code.claude.com/docs/zh-CN/agent-sdk/permissions.md)。

## 故障排除

### 找不到 Skills

**检查 settingSources 配置**：SDK 通过 `user` 和 `project` 设置源发现 skills。如果你显式设置 `settingSources`/`setting_sources` 并省略这些源，SDK 不会加载 skills：

```python Python theme={null}
# Skills not loaded: setting_sources excludes user and project
options = ClaudeAgentOptions(setting_sources=[], skills="all")

# Skills loaded: user and project sources included
options = ClaudeAgentOptions(
    setting_sources=["user", "project"],
    skills="all",
)
```

```typescript TypeScript theme={null}
// Skills not loaded: settingSources excludes user and project
const optionsWithoutSkills = {
  settingSources: [],
  skills: "all"
};

// Skills loaded: user and project sources included
const optionsWithSkills = {
  settingSources: ["user", "project"],
  skills: "all"
};
```

有关每个源加载哪些 skill 目录，请参阅 [文件系统源表](https://code.claude.com/docs/zh-CN/agent-sdk/claude-code-features.md#control-filesystem-settings-with-settingsources)。有关 `settingSources`/`setting_sources` 的更多详情，请参阅 [TypeScript SDK 参考](https://code.claude.com/docs/zh-CN/agent-sdk/typescript.md#settingsource) 或 [Python SDK 参考](https://code.claude.com/docs/zh-CN/agent-sdk/python.md#settingsource)。

**检查工作目录**：SDK 从 `cwd` 选项中的 `.claude/skills/` 以及直到存储库根目录的每个父目录加载 skills。确保 `cwd` 指向包含 `.claude/skills/` 的目录或其下方目录，且在同一存储库内：

```python Python theme={null}
# Ensure your cwd points to the directory containing .claude/skills/
options = ClaudeAgentOptions(
    cwd="/path/to/project",  # .claude/skills/ here or in a parent directory
    setting_sources=["user", "project"],  # Loads skills from these sources
    skills="all",
)
```

```typescript TypeScript theme={null}
// Ensure your cwd points to the directory containing .claude/skills/
const options = {
  cwd: "/path/to/project", // .claude/skills/ here or in a parent directory
  settingSources: ["user", "project"], // Loads skills from these sources
  skills: "all"
};
```

请参阅 [在 Agent SDK 中使用 skills](#use-skills-with-the-agent-sdk) 了解完整模式。

**验证文件系统位置**：

```bash theme={null}
# Check project skills
ls .claude/skills/*/SKILL.md

# Check personal skills
ls ~/.claude/skills/*/SKILL.md
```

### Skill 未被使用

**检查 `skills` 选项**：如果你传递了 `skills` 列表，确认 skill 的名称已包含。当 Claude 尝试调用未列出的 skill 时，Skill 工具返回 `Skill <name> is not in this session's skills allowlist`。将名称添加到你的列表，或通过在提示中发送 `/<name>` 直接分派 skill，这在不列出的情况下也能工作。

**检查描述**：确保它具体且包含相关关键字。请参阅 [Agent Skills 最佳实践](https://platform.claude.com/docs/en/agents-and-tools/agent-skills/best-practices#writing-effective-descriptions) 了解编写有效描述的指导。

### Invalid skill name error

当你的 `skills` 列表中的名称不能作为确切 skill 名称工作时，`query()` 会在启动 Claude Code 进程前拒绝该列表。触发拒绝的名称包括：

* 空名称
* 包含括号、逗号或控制字符的名称
* 用空格填充的名称
* 通配符形式，例如裸 `*` 或 `:*` 后缀

每个 SDK 以不同的方式呈现拒绝：

> [tab] TypeScript
>
> TypeScript SDK 抛出一个 `Error`，说明条目破坏的规则。例如，`skills: ["docs:*"]` 抛出：
>
> ```text theme={null}
> Invalid skill name "docs:*": wildcard-suffix names are not allowed; list each skill by its exact name.
> ```
>
> 空名称报告 `Skill names must be non-empty strings.`
>
> 在 TypeScript Agent SDK 0.3.221 之前，SDK 没有运行此检查。

> [tab] Python
>
> Python SDK 引发 `ValueError`，说明条目破坏的规则。例如，`skills=["docs:*"]` 引发：
>
> ```text theme={null}
> ValueError: Invalid skill name 'docs:*': wildcard-suffix names are not allowed; list each skill by its exact name.
> ```
>
> 空名称报告 `Skill names must be non-empty strings`。
>
> 在 Python Agent SDK 0.2.129 之前，SDK 没有运行此检查。

### 其他故障排除

有关一般 skills 故障排除，例如 YAML 语法错误和调试，请参阅 [Claude Code skills 故障排除部分](https://code.claude.com/docs/zh-CN/skills.md#troubleshooting)。

## 后续步骤

[Claude Code skills 指南](https://code.claude.com/docs/zh-CN/skills.md) 深入涵盖编写。其指导适用于 SDK 会话。从这些部分开始：

* [Frontmatter 参考](https://code.claude.com/docs/zh-CN/skills.md#frontmatter-reference)：每个支持的字段
* [将参数传递给 skill](https://code.claude.com/docs/zh-CN/skills.md#pass-arguments-to-skills)：`$ARGUMENTS`、`$0` 和 `$1`。[完整替换表](https://code.claude.com/docs/zh-CN/skills.md#available-string-substitutions) 添加命名参数和 `${CLAUDE_*}` 变量
* [注入动态上下文](https://code.claude.com/docs/zh-CN/skills.md#inject-dynamic-context)：`` !`command` `` 行在 Claude 看到 skill 内容前运行
* [选择 skills 加载的位置](https://code.claude.com/docs/zh-CN/skills.md#where-skills-live)：每个 skill 位置、插件命名空间和两个共享名称时哪一个运行

## 相关资源

* [Claude Code 中的命令](https://code.claude.com/docs/zh-CN/commands.md)：完整命令表面，包括每个内置命令
* [Agent Skills 概述](https://platform.claude.com/docs/en/agents-and-tools/agent-skills/overview)：概念概述、优势和架构
* [Agent Skills 最佳实践](https://platform.claude.com/docs/en/agents-and-tools/agent-skills/best-practices)：有效 skills 的编写指南
* [Agent Skills 食谱](https://platform.claude.com/cookbook/skills-notebooks-01-skills-introduction)：示例 skills 和模板
* [SDK 中的子代理](https://code.claude.com/docs/zh-CN/agent-sdk/subagents.md)：具有编程选项的类似文件系统代理
* [SDK 概述](https://code.claude.com/docs/zh-CN/agent-sdk/overview.md)：常规 SDK 概念
* [TypeScript SDK 参考](https://code.claude.com/docs/zh-CN/agent-sdk/typescript.md)：完整 API 文档
* [Python SDK 参考](https://code.claude.com/docs/zh-CN/agent-sdk/python.md)：完整 API 文档
