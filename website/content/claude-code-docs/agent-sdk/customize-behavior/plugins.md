> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# SDK 中的 Plugins

> 通过 Agent SDK 加载自定义 plugins，以向 agent 会话添加 skills、agents、hooks 和 MCP servers

Plugins 允许你使用可在项目间共享的自定义功能来扩展 Claude Code。通过 Agent SDK，你可以以编程方式从本地目录加载 plugins，以便向 agent 会话添加功能。一个 plugin 可以包括：

* **Skills**：Claude 自主调用的功能。你也可以使用 `/plugin-name:skill-name` 直接调用 plugin skill。
* **Agents**：用于特定任务的专门子 agents
* **Hooks**：响应工具使用和其他事件的事件处理程序
* **MCP servers**：通过 Model Context Protocol 的外部工具集成

有关 plugin 结构和如何创建 plugins 的完整信息，请参阅 [Plugins](https://code.claude.com/docs/zh-CN/plugins/overview.md)。

## 加载 plugins

通过在选项配置中提供本地文件系统路径来加载 plugins。`type` 字段必须是 `"local"`，这是 SDK 接受的唯一值。SDK 支持从不同位置加载多个 plugins。

要使用通过 [marketplace](https://code.claude.com/docs/zh-CN/plugins/overview.md) 或远程存储库分发的 plugin，请先下载它并提供本地目录路径。有关 plugin 需要的目录布局，请参阅下面的 [Plugin 结构参考](#plugin-structure-reference)。

```typescript TypeScript theme={null}
import { query } from "@anthropic-ai/claude-agent-sdk";

for await (const message of query({
  prompt: "Hello",
  options: {
    plugins: [
      { type: "local", path: "./my-plugin" },
      { type: "local", path: "/absolute/path/to/another-plugin" }
    ]
  }
})) {
  // Plugin commands, agents, and other features are now available
}
```

```python Python theme={null}
import asyncio
from claude_agent_sdk import query, ClaudeAgentOptions


async def main():
    async for message in query(
        prompt="Hello",
        options=ClaudeAgentOptions(
            plugins=[
                {"type": "local", "path": "./my-plugin"},
                {"type": "local", "path": "/absolute/path/to/another-plugin"},
            ]
        ),
    ):
        # Plugin commands, agents, and other features are now available
        pass


asyncio.run(main())
```

### 路径规范

Plugin 路径可以是：

* **相对路径**：相对于 `cwd` 选项解析（例如，`"./plugins/my-plugin"`）
* **绝对路径**：完整文件系统路径（例如，`"/home/user/plugins/my-plugin"`）

> [!NOTE]
>
> 路径应指向 plugin 的根目录：`skills/`、`agents/`、`hooks/`、`commands/` 或 `.claude-plugin/` 的父目录。

### 向运行中的会话添加插件

在 TypeScript SDK 中，对 `query()` 返回的 `Query` 对象调用 [`reloadPlugins()`](https://code.claude.com/docs/zh-CN/agent-sdk/typescript#load-more-plugin-folders-with-addplugins)，并在该调用的 `addPlugins` 选项中列出插件目录。每个 `addPlugins` 条目都是一个带有绝对 `path` 的 `{ type: "local", path }` 对象。需要 TypeScript Agent SDK v0.3.301 或更高版本。

在此示例中，`q` 是 `query()` 返回的 `Query` 对象。该示例向运行中的会话添加一个插件目录，并在 `reloadPlugins()` 返回后打印会话中每个插件的路径：

```typescript theme={null}
const result = await q.reloadPlugins({
  addPlugins: [{ type: "local", path: "/home/user/plugins/another-plugin" }]
});

console.log(result.plugins.map((plugin) => plugin.path));
```

如果插件已加载，打印的列表中会包含 `/home/user/plugins/another-plugin`。如果列表中缺少该路径，或者 `reloadPlugins()` 被拒绝（reject），请参阅[当 `reloadPlugins()` 未加载 `addPlugins` 文件夹时](https://code.claude.com/docs/zh-CN/agent-sdk/typescript#when-reloadplugins-doesnt-load-the-addplugins-folders)了解原因。

## 验证 plugin 安装

当 plugins 成功加载时，它们会出现在系统初始化消息中。你可以验证你的 plugins 是否可用：

```typescript TypeScript theme={null}
import { query } from "@anthropic-ai/claude-agent-sdk";

for await (const message of query({
  prompt: "Hello",
  options: {
    plugins: [{ type: "local", path: "./my-plugin" }]
  }
})) {
  if (message.type === "system" && message.subtype === "init") {
    // Check loaded plugins
    console.log("Plugins:", message.plugins);
    // Example: [{ name: "my-plugin", path: "/absolute/path/to/my-plugin" }]

    // Plugin skills appear with the plugin name as a prefix
    console.log("Skills:", message.skills);
    // Example: ["my-plugin:greet"]

    // Plugin commands use the same prefix, and skills appear here too
    console.log("Commands:", message.slash_commands);
    // Example: ["compact", "context", "my-plugin:custom-command", "my-plugin:greet"]
  }
}
```

```python Python theme={null}
import asyncio
from claude_agent_sdk import query, ClaudeAgentOptions, SystemMessage


async def main():
    async for message in query(
        prompt="Hello",
        options=ClaudeAgentOptions(
            plugins=[{"type": "local", "path": "./my-plugin"}]
        ),
    ):
        if isinstance(message, SystemMessage) and message.subtype == "init":
            # Check loaded plugins
            print("Plugins:", message.data.get("plugins"))
            # Example: [{"name": "my-plugin", "path": "/absolute/path/to/my-plugin"}]

            # Plugin skills appear with the plugin name as a prefix
            print("Skills:", message.data.get("skills"))
            # Example: ["my-plugin:greet"]

            # Plugin commands use the same prefix, and skills appear here too
            print("Commands:", message.data.get("slash_commands"))
            # Example: ["compact", "context", "my-plugin:custom-command", "my-plugin:greet"]


asyncio.run(main())
```

## 使用 plugin skills

来自 plugins 的 skills 会自动使用 plugin 名称进行命名空间划分，以避免冲突。要直接调用一个，请在提示中发送 `/plugin-name:skill-name`。

```typescript TypeScript theme={null}
import { query } from "@anthropic-ai/claude-agent-sdk";

// Load a plugin with a custom /greet skill
for await (const message of query({
  prompt: "/my-plugin:greet", // Use plugin skill with namespace
  options: {
    plugins: [{ type: "local", path: "./my-plugin" }]
  }
})) {
  // Claude executes the custom greeting skill from the plugin
  if (message.type === "assistant") {
    console.log(message.message.content);
  }
}
```

```python Python theme={null}
import asyncio
from claude_agent_sdk import query, ClaudeAgentOptions, AssistantMessage, TextBlock


async def main():
    # Load a plugin with a custom /greet skill
    async for message in query(
        prompt="/my-plugin:greet",  # Use plugin skill with namespace
        options=ClaudeAgentOptions(
            plugins=[{"type": "local", "path": "./my-plugin"}]
        ),
    ):
        # Claude executes the custom greeting skill from the plugin
        if isinstance(message, AssistantMessage):
            for block in message.content:
                if isinstance(block, TextBlock):
                    print(f"Claude: {block.text}")


asyncio.run(main())
```

> [!NOTE]
>
> 如果你通过 CLI 安装了 plugin（例如，`/plugin install my-plugin@marketplace`），你仍然可以通过提供其安装路径在 SDK 中使用它。检查 `~/.claude/plugins/` 以查找 CLI 安装的 plugins。

## 完整示例

这是一个演示 plugin 加载和使用的完整示例：

```typescript TypeScript theme={null}
import { query } from "@anthropic-ai/claude-agent-sdk";
import { fileURLToPath } from "node:url";

async function runWithPlugin() {
  const pluginPath = fileURLToPath(new URL("./plugins/my-plugin", import.meta.url));

  console.log("Loading plugin from:", pluginPath);

  for await (const message of query({
    prompt: "What custom commands do you have available?",
    options: {
      plugins: [{ type: "local", path: pluginPath }],
      maxTurns: 3
    }
  })) {
    if (message.type === "system" && message.subtype === "init") {
      console.log("Loaded plugins:", message.plugins);
      console.log("Available skills:", message.skills);
      console.log("Available commands:", message.slash_commands);
    }

    if (message.type === "assistant") {
      console.log("Assistant:", message.message.content);
    }
  }
}

runWithPlugin().catch(console.error);
```

```python Python theme={null}
#!/usr/bin/env python3
"""Example demonstrating how to use plugins with the Agent SDK."""

import asyncio
from pathlib import Path

from claude_agent_sdk import (
    AssistantMessage,
    ClaudeAgentOptions,
    SystemMessage,
    TextBlock,
    query,
)


async def run_with_plugin():
    """Example using a custom plugin."""
    plugin_path = Path(__file__).parent / "plugins" / "my-plugin"

    print(f"Loading plugin from: {plugin_path}")

    options = ClaudeAgentOptions(
        plugins=[{"type": "local", "path": str(plugin_path)}],
        max_turns=3,
    )

    async for message in query(
        prompt="What custom commands do you have available?", options=options
    ):
        if isinstance(message, SystemMessage) and message.subtype == "init":
            print(f"Loaded plugins: {message.data.get('plugins')}")
            print(f"Available skills: {message.data.get('skills')}")
            print(f"Available commands: {message.data.get('slash_commands')}")

        if isinstance(message, AssistantMessage):
            for block in message.content:
                if isinstance(block, TextBlock):
                    print(f"Assistant: {block.text}")


if __name__ == "__main__":
    asyncio.run(run_with_plugin())
```

## Plugin 结构参考

Plugin 目录通常包含一个 `.claude-plugin/plugin.json` 清单文件。清单是可选的。省略时，Claude Code 会从目录布局自动发现组件。该目录可以包括：

```text theme={null}
my-plugin/
├── .claude-plugin/
│   └── plugin.json          # Plugin 清单（可选，不需要它也能自动发现组件）
├── skills/                   # Agent Skills（自主调用或通过 /plugin-name:skill-name）
│   └── my-skill/
│       └── SKILL.md
├── commands/                 # Skills 作为平面 .md 文件
│   └── custom-cmd.md
├── agents/                   # 自定义 agents
│   └── specialist.md
├── hooks/                    # 事件处理程序
│   └── hooks.json
└── .mcp.json                # MCP 服务器定义
```

> [!NOTE]
>
> `commands/` 目录包含作为平面 Markdown 文件的 skills。对于新 plugins，请使用 `skills/`。Claude Code 支持两个位置。

## 多个 plugin 源

组合来自不同位置的 plugins：

```typescript theme={null}
import * as os from "node:os";
import * as path from "node:path";

plugins: [
  { type: "local", path: "./local-plugin" },
  {
    type: "local",
    path: path.join(os.homedir(), ".claude", "custom-plugins", "shared-plugin")
  }
];
```

> [!NOTE]
>
> SDK 不会展开波浪号路径，如 `~/plugins`。如果 plugin 路径不存在，SDK 会跳过该 plugin，会话继续，因此请检查初始化消息中的 `plugins` 列表以确认每个 plugin 已加载。

## 故障排除

### Plugin 未加载

如果你的 plugin 未出现在初始化消息的 `plugins` 列表中，请检查其 [`plugin_errors`](https://code.claude.com/docs/zh-CN/agent-sdk/typescript#sdksystemmessage) 字段以了解原因，然后按照以下检查步骤进行：

1. **检查路径**：确保路径指向 plugin 根目录，即 `skills/`、`agents/`、`hooks/`、`commands/` 或 `.claude-plugin/` 的父目录
2. **验证 plugin.json**：如果你的 plugin 包含清单文件，确保它具有有效的 JSON 语法
3. **检查文件权限**：确保 plugin 目录可读
4. **确认目录存在**：SDK 会跳过不存在的路径，plugin 不会出现在初始化消息的 `plugins` 列表中

### Skills 未出现

如果 plugin skills 不起作用：

1. **使用命名空间**：调用 plugin skills 时使用 `/plugin-name:skill-name` 格式
2. **检查初始化消息**：验证 skill 是否以正确的命名空间出现在 `skills` 列表中
3. **验证 skill 文件**：确保每个 skill 在 `skills/` 下的自己的子目录中都有一个 `SKILL.md` 文件，例如 `skills/my-skill/SKILL.md`

## 另请参阅

* [Plugins](https://code.claude.com/docs/zh-CN/plugins/overview.md) - 完整的 plugin 开发指南
* [Plugins reference](https://code.claude.com/docs/zh-CN/plugins/manifest-reference.md) - 技术规范
* [Commands](https://code.claude.com/docs/zh-CN/agent-sdk/skills.md#dispatch-commands-by-name) - 在 SDK 中调度命令
* [Subagents](https://code.claude.com/docs/zh-CN/agent-sdk/subagents.md) - 使用专门的 agents
* [Skills](https://code.claude.com/docs/zh-CN/agent-sdk/skills.md) - 使用 Agent Skills
