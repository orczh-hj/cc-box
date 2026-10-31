> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# 使用工具搜索扩展到多个工具

> 通过动态发现和按需加载，将您的代理扩展到数千个工具。

工具搜索使您的代理能够通过动态发现和按需加载来处理数百或数千个工具。代理不是将所有工具定义预先加载到上下文窗口中，而是搜索您的工具目录并仅加载它需要的工具。

当工具库扩展时，这种方法解决了两个挑战：

* **上下文效率：** 工具定义可能会消耗上下文窗口的大部分（50个工具可能使用10-20K个令牌），留下较少的空间用于实际工作。
* **工具选择准确性：** 一次加载超过30-50个工具时，工具选择准确性会下降。

## 工具搜索的工作原理

工具搜索默认处于启用状态，[配置工具搜索](#configure-tool-search)中列出的情况除外。

当工具搜索处于活动状态时，工具定义会从上下文窗口中隐藏。代理会收到可用工具的摘要，并在任务需要尚未加载的功能时搜索相关工具。最多五个最相关的工具被默认加载到上下文中，在后续轮次中保持可用，直到SDK压缩代理发现它们的消息。在该压缩之后，代理在下次需要这些工具时会再次搜索它们。

工具搜索在Claude每次搜索工具时增加一个额外的往返，但对于大型工具集，这被每个轮次中较小的上下文所抵消。对于少于约10个工具且其定义能够舒适地放入上下文窗口的情况，预先加载所有工具通常更快。

有关底层API机制的详细信息，请参阅[API中的工具搜索](https://platform.claude.com/docs/zh-CN/agents-and-tools/tool-use/tool-search-tool)。

> [!NOTE]
>
> Microsoft Foundry上的工具搜索不受支持，[部署在Azure上](https://platform.claude.com/docs/en/build-with-claude/claude-in-microsoft-foundry#hosting-options)的部署会在服务器端拒绝它：SDK检测到拒绝并为该部署改为预先加载工具定义。[`ENABLE_TOOL_SEARCH`](#configure-tool-search)无法覆盖此行为，因为拒绝来自部署本身。

## 配置工具搜索

工具搜索默认启用。对于SDK的不支持模型列表中的模型，SDK会改为预先加载工具定义，任何`ENABLE_TOOL_SEARCH`值都无法覆盖此行为。在Google Cloud的Agent Platform上，SDK根据模型代数决定：

* **Claude Opus 4.5、Sonnet 4.5、Haiku 4.5及更高版本**：工具搜索默认启用。
* **早期Agent Platform模型**：SDK预先加载工具定义，因为它们的服务堆栈拒绝所需的beta标头。`ENABLE_TOOL_SEARCH`无法覆盖此行为。

在Claude Code v2.1.221之前，SDK会为Google Cloud的Agent Platform上的所有模型禁用工具搜索，除非您设置`ENABLE_TOOL_SEARCH`。

> [!NOTE]
>
> 在`ANTHROPIC_BASE_URL`网关后方，工具搜索默认启用，无需任何设置。如果网关拒绝它，请参阅[通过LLM网关使用工具搜索](https://code.claude.com/docs/zh-CN/mcp.md#tool-search-through-a-gateway)。在Claude Code v2.1.304之前，除非`ENABLE_TOOL_SEARCH`为`true`、`auto`或`auto:N`，否则SDK会在此类网关后方禁用工具搜索。

您可以使用`ENABLE_TOOL_SEARCH`环境变量控制是否以及何时延迟工具定义，该变量接受以下值：

| 值 | 行为 |
| :- | :- |
| （未设置） | 工具搜索启用，工具定义被延迟并按需发现。在早于Opus 4和Sonnet 4的模型、[早期Agent Platform模型](#configure-tool-search)以及托管在Azure上的Microsoft Foundry部署上回退到预先加载。 |
| `true` | 工具搜索启用，回退情况与（未设置）相同。 |
| `auto` | 工具搜索关闭，工具定义预先加载，直到工具搜索可以延迟的定义达到模型上下文窗口的10%。随后工具搜索激活，并且即使这些定义之后降至10%以下，也可以[在同一对话中保持启用](https://code.claude.com/docs/zh-CN/mcp.md#tools-that-stay-deferred-below-the-threshold)。 |
| `auto:N` | 与`auto`相同，但具有自定义百分比。`auto:5`在这些定义达到上下文窗口的5%时激活。较低的值更早激活。 |
| `false` | 工具搜索关闭。所有工具定义在每个轮次上都加载到上下文中。 |

设置[`CLAUDE_CODE_DISABLE_EXPERIMENTAL_BETAS`](https://code.claude.com/docs/zh-CN/env-vars)会保持工具搜索关闭。您无法通过自己设置`ENABLE_TOOL_SEARCH`来覆盖它。在Claude Code v2.1.227或更高版本上，您的组织可以通过[托管设置](https://code.claude.com/docs/zh-CN/managed-settings.md)在设置了此变量的情况下保持工具搜索启用。在[获得HIPAA配置](https://code.claude.com/docs/zh-CN/hipaa-setup.md#check-how-developers-sign-in-and-connect)的会话中，工具搜索处于关闭状态，`ENABLE_TOOL_SEARCH`和托管设置都无法将其重新启用。[禁用预发布功能](https://code.claude.com/docs/zh-CN/llm-gateway-protocol.md#disable-pre-release-capabilities)涵盖了覆盖应用的位置以及变量删除的内容。

工具搜索适用于所有已注册的工具，无论它们来自远程MCP 服务器还是[自定义SDK MCP 服务器](https://code.claude.com/docs/zh-CN/agent-sdk/custom-tools.md)。当您使用`auto`时，SDK会计算工具搜索可以延迟的每个定义，针对一个组合阈值：来自任何服务器的未标记为[`alwaysLoad`](https://code.claude.com/docs/zh-CN/mcp#exempt-a-server-from-deferral)的每个MCP工具，加上按需加载的内置工具。SDK会预先加载核心内置工具（如Bash、Read和Edit），不会将其计入阈值。您的某个插件中的[mod](https://code.claude.com/docs/zh-CN/plugins/mods/reference.md#tools)也可以延迟某个工具或预先加载它，这会改变该计数。

在`query()`上的`env`选项中设置值。在TypeScript中，`env`替换子进程环境，因此展开`...process.env`以保持继承的变量。在Python中，`env`合并到继承的环境之上。此示例连接到公开许多工具的远程MCP 服务器，使用通配符预先批准所有工具，并使用`auto:5`，以便当工具搜索可以延迟的定义达到上下文窗口的5%时激活工具搜索：

```typescript TypeScript theme={null}
import { query } from "@anthropic-ai/claude-agent-sdk";

try {
  for await (const message of query({
    prompt: "Find and run the appropriate database query",
    options: {
      mcpServers: {
        "enterprise-tools": {
          // Connect to a remote MCP server
          type: "http",
          url: "https://tools.example.com/mcp"
        }
      },
      allowedTools: ["mcp__enterprise-tools__*"], // Wildcard pre-approves all tools from this server
      env: {
        ...process.env, // env replaces the subprocess environment, so keep inherited variables
        ENABLE_TOOL_SEARCH: "auto:5" // Activate tool search when deferrable definitions reach 5% of context
      }
    }
  })) {
    if (message.type === "result" && message.subtype === "success") {
      console.log(message.result);
    }
  }
} catch (error) {
  // A single-shot query() throws after yielding an error result
  console.log(`Session ended with an error: ${error}`);
}
```

```python Python theme={null}
import asyncio
from claude_agent_sdk import query, ClaudeAgentOptions, ResultMessage


async def main():
    options = ClaudeAgentOptions(
        mcp_servers={
            "enterprise-tools": {
                "type": "http",
                "url": "https://tools.example.com/mcp",
            }
        },
        allowed_tools=[
            "mcp__enterprise-tools__*"
        ],  # Wildcard pre-approves all tools from this server
        env={
            "ENABLE_TOOL_SEARCH": "auto:5"  # Activate tool search when deferrable definitions reach 5% of context
        },
    )

    try:
        async for message in query(
            prompt="Find and run the appropriate database query",
            options=options,
        ):
            if isinstance(message, ResultMessage) and message.subtype == "success":
                print(message.result)
    except Exception as error:
        # A single-shot query() raises after yielding an error result
        print(f"Session ended with an error: {error}")


asyncio.run(main())
```

要运行此示例，请将`https://tools.example.com/mcp`替换为您自己的MCP 服务器的URL。成功时，结果文本会打印到控制台。

因为这是一个单次`query()`调用，SDK在产生错误结果后会抛出异常，所以该示例将循环包装在try块中。要查看运行失败的原因，请检查循环内结果消息的`subtype`，例如`error_during_execution`。有关结果消息的更多信息，请参阅[处理结果](https://code.claude.com/docs/zh-CN/agent-sdk/agent-loop.md#handle-the-result)。

## 优化工具发现

搜索机制将查询与工具名称和描述进行匹配。像`search_slack_messages`这样的名称比`query_slack`更广泛地出现在各种请求中。具有特定关键字的描述（"按关键字、频道或日期范围搜索Slack消息"）比通用描述（"查询Slack"）匹配更多查询。

您还可以添加一个系统提示部分，列出可用的工具类别。这为代理提供了关于可以搜索什么类型工具的上下文。通过 TypeScript 中的 `systemPrompt` 选项或 Python 中的 `system_prompt` 传递文本，使用带有 `append` 的 `claude_code` 预设，这会将您的文本添加到预设的提示中，而不是替换它：

```typescript TypeScript theme={null}
options: {
  systemPrompt: {
    type: "preset",
    preset: "claude_code",
    append: "You can search for tools to interact with Slack, GitHub, and Jira."
  }
}
```

```python Python theme={null}
options = ClaudeAgentOptions(
    system_prompt={
        "type": "preset",
        "preset": "claude_code",
        "append": "You can search for tools to interact with Slack, GitHub, and Jira.",
    }
)
```

有关完整的系统提示选项集，请参阅[修改系统提示](https://code.claude.com/docs/zh-CN/agent-sdk/modifying-system-prompts.md)。

## 限制

* **最大工具数：** 您的目录中最多10,000个工具
* **搜索结果：** 默认每次搜索返回最多五个最相关的工具
* **模型支持：** Opus 4、Sonnet 4、Haiku 4.5及更新的模型；在Google Cloud的Agent Platform上为Opus 4.5、Sonnet 4.5、Haiku 4.5及更新的模型。在更早的模型上，SDK会预先加载工具定义，如[配置工具搜索](#configure-tool-search)中所述。

## 相关文档

* [API中的工具搜索](https://platform.claude.com/docs/zh-CN/agents-and-tools/tool-use/tool-search-tool)：工具搜索的完整API文档，包括自定义实现
* [连接MCP服务器](https://code.claude.com/docs/zh-CN/agent-sdk/mcp.md)：通过MCP服务器连接到外部工具
* [自定义工具](https://code.claude.com/docs/zh-CN/agent-sdk/custom-tools.md)：使用SDK MCP服务器构建您自己的工具
* [TypeScript SDK参考](https://code.claude.com/docs/zh-CN/agent-sdk/typescript.md)：完整API参考
* [Python SDK参考](https://code.claude.com/docs/zh-CN/agent-sdk/python.md)：完整API参考
