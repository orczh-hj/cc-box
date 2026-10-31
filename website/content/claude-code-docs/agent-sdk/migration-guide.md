> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# 迁移到 Claude Agent SDK

> 将 Claude Code TypeScript 和 Python SDK 迁移到 Claude Agent SDK 的指南

## 概述

Claude Code SDK 已更名为 **Claude Agent SDK**，其文档已重新组织。这一变化反映了该 SDK 在构建 AI Agent 方面的更广泛功能，不仅限于编码任务。

从 OpenAI Agents SDK 迁移？[OpenAI Agents SDK 迁移指南](https://platform.claude.com/cookbook/claude-agent-sdk-04-migrating-from-openai-agents-sdk)通过一个完整的示例将每个原语映射到 Claude Agent SDK。

## 有什么改变

| 方面 | 旧版 | 新版 |
| :- | :- | :- |
| **包名称 (TS/JS)** | `@anthropic-ai/claude-code` | `@anthropic-ai/claude-agent-sdk` |
| **Python 包** | `claude-code-sdk` | `claude-agent-sdk` |
| **文档位置** | Claude Code 文档 | Claude Code 文档 → 专用 [Agent SDK](https://code.claude.com/docs/zh-CN/agent-sdk/overview.md) 部分 |

## 迁移步骤

### 迁移 TypeScript 或 JavaScript 项目

**1. 卸载旧包：**

```bash theme={null}
npm uninstall @anthropic-ai/claude-code
```

**2. 安装新包：**

```bash theme={null}
npm install @anthropic-ai/claude-agent-sdk
```

**3. 更新你的导入：**

将所有导入从 `@anthropic-ai/claude-code` 更改为 `@anthropic-ai/claude-agent-sdk`：

```typescript theme={null}
// 之前
import { query, tool, createSdkMcpServer } from "@anthropic-ai/claude-code";

// 之后
import { query, tool, createSdkMcpServer } from "@anthropic-ai/claude-agent-sdk";
```

**4. 更新 package.json：**

如果 `@anthropic-ai/claude-code` 仍在你的 `package.json` 中列出，请将其替换为 `@anthropic-ai/claude-agent-sdk` 并同时更新版本范围，例如从 `"^0.0.42"` 更新到 `"^0.3.0"`。

**5. 查看[破坏性变更](#breaking-changes)**

进行任何必要的代码更改以完成迁移。

### 迁移 Python 项目

**1. 卸载旧包：**

```bash theme={null}
pip uninstall -y claude-code-sdk
```

如果未安装旧包，pip 会打印 `WARNING: Skipping claude-code-sdk as it is not installed.` 这是预期的，你可以继续下一步。

**2. 安装新包：**

```bash theme={null}
pip install claude-agent-sdk
```

如果 `claude-code-sdk` 在你的 `requirements.txt` 或 `pyproject.toml` 中列出，请将其替换为 `claude-agent-sdk`。

**3. 更新你的导入：**

将所有导入从 `claude_code_sdk` 更改为 `claude_agent_sdk`：

```python theme={null}
# 之前
from claude_code_sdk import query, ClaudeCodeOptions

# 之后
from claude_agent_sdk import query, ClaudeAgentOptions
```

**4. 查看[破坏性变更](#breaking-changes)**

进行任何必要的代码更改以完成迁移。

## 破坏性变更

> [!WARNING]
>
> 为了改进隔离和显式配置，Claude Agent SDK v0.1.0 为从 Claude Code SDK 迁移的用户引入了破坏性变更。

### Python：ClaudeCodeOptions 重命名为 ClaudeAgentOptions

**变更内容：** Python SDK 类型 `ClaudeCodeOptions` 已重命名为 `ClaudeAgentOptions`。

**迁移：**

```python theme={null}
# BEFORE (claude-code-sdk)
from claude_code_sdk import query, ClaudeCodeOptions

options = ClaudeCodeOptions(model="claude-opus-4-7", permission_mode="acceptEdits")

# AFTER (claude-agent-sdk)
from claude_agent_sdk import query, ClaudeAgentOptions

options = ClaudeAgentOptions(model="claude-opus-4-7", permission_mode="acceptEdits")
```

### 系统提示词不再是默认值

**变更内容：** SDK 不再默认使用 Claude Code 的系统提示词。

**迁移：**

```typescript TypeScript theme={null}
import { query } from "@anthropic-ai/claude-agent-sdk";

// BEFORE (v0.0.x) - 默认使用 Claude Code 的系统提示词
const before = query({ prompt: "Hello" });

// AFTER (v0.1.0) - 默认使用最小系统提示词
// 要获得旧行为，请显式请求 Claude Code 的预设：
const presetResult = query({
  prompt: "Hello",
  options: {
    systemPrompt: { type: "preset", preset: "claude_code" }
  }
});

// 或使用自定义系统提示词：
const customResult = query({
  prompt: "Hello",
  options: {
    systemPrompt: "You are a helpful coding assistant"
  }
});
```

```python Python theme={null}
from claude_agent_sdk import query, ClaudeAgentOptions
import asyncio


async def main():
    # BEFORE (v0.0.x) - 默认使用 Claude Code 的系统提示词
    async for message in query(prompt="Hello"):
        print(message)

    # AFTER (v0.1.0) - 默认使用最小系统提示词
    # 要获得旧行为，请显式请求 Claude Code 的预设：
    async for message in query(
        prompt="Hello",
        options=ClaudeAgentOptions(
            system_prompt={"type": "preset", "preset": "claude_code"}  # 使用预设
        ),
    ):
        print(message)

    # 或使用自定义系统提示词：
    async for message in query(
        prompt="Hello",
        options=ClaudeAgentOptions(system_prompt="You are a helpful coding assistant"),
    ):
        print(message)


asyncio.run(main())
```

### 设置源默认值

此默认值在 v0.1.0 中曾被短暂更改为不加载任何文件系统设置，然后已恢复，因此无需迁移操作。

**当前行为：** 在 `query()` 上省略 `settingSources` 会加载用户、项目和本地文件系统设置，与 CLI 匹配。这包括 `~/.claude/settings.json`、`.claude/settings.json`、`.claude/settings.local.json`、CLAUDE.md 文件和自定义命令。

要从文件系统设置中隔离运行，请传递 `settingSources: []`，或在 Python 中传递 `setting_sources=[]`。请参阅 [使用 settingSources 控制文件系统设置](https://code.claude.com/docs/zh-CN/agent-sdk/claude-code-features.md#control-filesystem-settings-with-settingsources) 了解每个源加载的内容。

隔离对于 CI/CD 管道、已部署的应用程序、测试环境和多租户系统特别重要，其中本地自定义不应泄露。

> [!NOTE]
>
> Python SDK 0.1.59 及更早版本将空列表视为与省略该选项相同，因此在依赖 `setting_sources=[]` 之前请升级。请参阅 [settingSources 不控制的内容](https://code.claude.com/docs/zh-CN/agent-sdk/claude-code-features.md#what-settingsources-does-not-control) 了解即使在 `settingSources` 为 `[]` 时也会读取的输入。

## 后续步骤

* 探索 [Agent SDK 概述](https://code.claude.com/docs/zh-CN/agent-sdk/overview.md) 以了解可用功能
* 查看 [TypeScript SDK 参考](https://code.claude.com/docs/zh-CN/agent-sdk/typescript.md) 以获取详细的 API 文档
* 查看 [Python SDK 参考](https://code.claude.com/docs/zh-CN/agent-sdk/python.md) 以获取 Python 特定文档
* 了解 [自定义工具](https://code.claude.com/docs/zh-CN/agent-sdk/custom-tools.md) 和 [MCP 集成](https://code.claude.com/docs/zh-CN/agent-sdk/mcp.md)
