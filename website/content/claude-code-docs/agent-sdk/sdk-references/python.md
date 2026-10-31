> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Agent SDK 参考 - Python

> Python Agent SDK 的完整 API 参考，包括所有函数、类型和类。

## 安装

在虚拟环境中安装该包。在最近的 Debian、Ubuntu 和 Homebrew Python 安装上，针对系统 Python 运行 `pip install` 会失败，出现 `error: externally-managed-environment` 错误。

```bash theme={null}
python3 -m venv .venv
source .venv/bin/activate
pip install claude-agent-sdk
```

有关 uv、Windows PowerShell 和 API 密钥设置，请参阅 [Agent SDK 快速入门中的设置](https://code.claude.com/docs/zh-CN/agent-sdk/quickstart.md#setup)。

## 在 `query()` 和 `ClaudeSDKClient` 之间选择

Python SDK 提供了两种与 Claude Code 交互的方式：

| 功能 | `query()` | `ClaudeSDKClient` |
| :- | :- | :- |
| **会话** | 默认创建新会话 | 重用同一会话 |
| **对话** | 单次交换 | 同一上下文中的多次交换 |
| **连接** | 自动管理 | 手动控制 |
| **流式输入** | ✅ 支持 | ✅ 支持 |
| **中断** | ❌ 不支持 | ✅ 支持 |
| **hooks** | ✅ 支持 | ✅ 支持 |
| **自定义工具** | ✅ 支持 | ✅ 支持 |
| **继续聊天** | 通过 `continue_conversation` 或 `resume` 手动进行 | ✅ 自动 |
| **用例** | 一次性任务 | 持续对话 |

对于交互式应用程序（如聊天界面）或当下一步操作取决于 Claude 的响应时，请使用 `ClaudeSDKClient`。

## 函数

> [!NOTE]
>
> 此页面上的签名块和裸 `async for` / `async with` 片段仅供说明之用。要运行它们，请将主体包装在 `async def main(): ...` 中并调用 `asyncio.run(main())`。

### `query()`

默认情况下，为与 Claude Code 的每次交互创建一个新会话。返回一个异步迭代器，在消息到达时产生消息。每次调用 `query()` 都会重新开始，除非您在 [`ClaudeAgentOptions`](#claudeagentoptions) 中传递 `continue_conversation=True` 或 `resume`，否则不会记住之前的交互。请参阅 [Sessions](https://code.claude.com/docs/zh-CN/agent-sdk/sessions.md)。

```python theme={null}
async def query(
    *,
    prompt: str | AsyncIterable[dict[str, Any]],
    options: ClaudeAgentOptions | None = None,
    transport: Transport | None = None
) -> AsyncIterator[Message]
```

#### 参数

| 参数 | 类型 | 描述 |
| :- | :- | :- |
| `prompt` | `str \| AsyncIterable[dict]` | 输入提示词，可以是字符串或用于流式模式的异步可迭代对象 |
| `options` | `ClaudeAgentOptions \| None` | 可选配置对象（如果为 None，默认为 `ClaudeAgentOptions()`） |
| `transport` | `Transport \| None` | 用于与 CLI 进程通信的可选自定义传输 |

#### 返回值

返回一个 `AsyncIterator[Message]`，从对话中产生消息。

#### 示例 - 带选项

```python theme={null}
import asyncio
from claude_agent_sdk import query, ClaudeAgentOptions


async def main():
    options = ClaudeAgentOptions(
        system_prompt="You are an expert Python developer",
        permission_mode="acceptEdits",
    )

    async for message in query(prompt="Create a Python web server", options=options):
        print(message)


asyncio.run(main())
```

### `tool()`

用于定义具有类型安全的 MCP 工具的装饰器。

```python theme={null}
def tool(
    name: str,
    description: str,
    input_schema: type | dict[str, Any],
    annotations: ToolAnnotations | None = None
) -> Callable[[Callable[[Any], Awaitable[dict[str, Any]]]], SdkMcpTool[Any]]
```

#### 参数

| 参数 | 类型 | 描述 |
| :- | :- | :- |
| `name` | `str` | 工具的唯一标识符 |
| `description` | `str` | 工具功能的人类可读描述 |
| `input_schema` | `type \| dict[str, Any]` | 定义工具输入参数的 schema。请参阅 [输入 schema 选项](#input-schema-options) |
| `annotations` | [`ToolAnnotations`](#toolannotations)` \| None` | 可选的 MCP 工具注解，为客户端提供行为提示 |

#### 输入 schema 选项

1. **简单类型映射**（推荐）：

   ```python theme={null}
   {"text": str, "count": int, "enabled": bool}
   ```

2. **JSON Schema 格式**（用于复杂验证）：
   ```python theme={null}
   {
       "type": "object",
       "properties": {
           "text": {"type": "string"},
           "count": {"type": "integer", "minimum": 0},
       },
       "required": ["text"],
   }
   ```

3. **TypedDict 类**：一种类型化的 schema，其中 `NotRequired` 键不会包含在 `required` 中。

   * **Python 3.11 及更高版本**：从 `typing` 导入 `TypedDict` 和 `NotRequired`。
   * **Python 3.10**：`typing` 中没有 `NotRequired`。请从 `typing_extensions` 导入 `TypedDict` 和 `NotRequired`，SDK 会在 Python 3.10 上安装该包。

   ```python theme={null}
   from typing import Annotated, Any, NotRequired, TypedDict
   from claude_agent_sdk import tool


   class ForecastArgs(TypedDict):
       latitude: Annotated[float, "Latitude coordinate"]
       hours: NotRequired[Annotated[int, "How many hours of forecast to return"]]


   @tool("get_forecast", "Get the hourly forecast for a location", ForecastArgs)
   async def get_forecast(args: dict[str, Any]) -> dict[str, Any]:
       hours = args.get("hours", 12)
       return {"content": [{"type": "text", "text": f"{hours}-hour forecast for {args['latitude']}"}]}
   ```

在简单映射和 TypedDict 形式中，将类型包装在 `Annotated[type, "description"]` 中即可设置该字段的描述。

#### 返回值

一个装饰器函数，包装工具实现并返回一个 `SdkMcpTool` 实例。

#### 示例

```python theme={null}
from claude_agent_sdk import tool
from typing import Any


@tool("greet", "Greet a user", {"name": str})
async def greet(args: dict[str, Any]) -> dict[str, Any]:
    return {"content": [{"type": "text", "text": f"Hello, {args['name']}!"}]}
```

#### `ToolAnnotations`

工具的行为提示，作为 [`tool()`](#tool) 的 `annotations` 参数传递。`ToolAnnotations` 扩展了 MCP SDK 的 `mcp.types.ToolAnnotations`，添加了 `maxResultSizeChars` 字段，您可以用 camelCase 或 snake\_case 编写每个提示：`ToolAnnotations(readOnlyHint=True)` 和 `ToolAnnotations(read_only_hint=True)` 是等效的。要从对象中读回某个提示，请使用已安装的 `mcp` 包所声明的拼写：在 `mcp` 1.x 上为 `.readOnlyHint`，在 2.x 上为 `.read_only_hint`，而 `.maxResultSizeChars` 在两者上均可使用。您也可以在 SDK 接受注解的任何地方传递普通的 `mcp.types.ToolAnnotations`。

snake\_case 名称和类型化的 `maxResultSizeChars` 字段需要 Python Agent SDK 0.2.140 或更高版本。版本 0.1.31 到 0.2.139 重新导出 `mcp.types.ToolAnnotations` 不变。在版本 0.1.55 到 0.2.139 上，您仍然可以将 `maxResultSizeChars` 作为关键字参数传递：MCP 类接受额外字段，SDK 将值转发给 Claude Code。

所有字段都是可选的。客户端不应依赖这些提示来做出安全决策。

| 字段 | 类型 | 默认值 | 描述 |
| :- | :- | :- | :- |
| `title` | `str \| None` | `None` | 工具的人类可读标题 |
| `readOnlyHint` | `bool \| None` | `False` | 如果为 `True`，工具不会修改其环境 |
| `destructiveHint` | `bool \| None` | `True` | 如果为 `True`，工具可能执行破坏性更新（仅当 `readOnlyHint` 为 `False` 时有意义） |
| `idempotentHint` | `bool \| None` | `False` | 如果为 `True`，使用相同参数的重复调用没有额外效果（仅当 `readOnlyHint` 为 `False` 时有意义） |
| `openWorldHint` | `bool \| None` | `True` | 如果为 `True`，工具与外部实体交互（例如，网络搜索）。如果为 `False`，工具的域是封闭的（例如，记忆工具） |
| `maxResultSizeChars` | `int \| None` | `None` | Claude Code 将此工具的文本结果保持内联在对话中而不是保存到文件的字符数，最多 500,000。包含图像的结果不受影响。Claude Code 设置而不是 MCP 提示：SDK 在工具的 `_meta` 中以 `anthropic/maxResultSizeChars` 的形式发送它。请参阅 [提高特定工具的限制](https://code.claude.com/docs/zh-CN/mcp.md#raise-the-limit-for-a-specific-tool) |

```python theme={null}
from claude_agent_sdk import tool, ToolAnnotations
from typing import Any


@tool(
    "search",
    "Search the web",
    {"query": str},
    annotations=ToolAnnotations(readOnlyHint=True, openWorldHint=True),
)
async def search(args: dict[str, Any]) -> dict[str, Any]:
    return {"content": [{"type": "text", "text": f"Results for: {args['query']}"}]}
```

### `create_sdk_mcp_server()`

创建在 Python 应用程序中运行的进程内 MCP 服务器。

```python theme={null}
def create_sdk_mcp_server(
    name: str,
    version: str = "1.0.0",
    tools: list[SdkMcpTool[Any]] | None = None
) -> McpSdkServerConfig
```

#### 参数

| 参数 | 类型 | 默认值 | 描述 |
| :- | :- | :- | :- |
| `name` | `str` | - | 服务器的唯一标识符 |
| `version` | `str` | `"1.0.0"` | 服务器版本字符串 |
| `tools` | `list[SdkMcpTool[Any]] \| None` | `None` | 使用 `@tool` 装饰器创建的工具函数列表 |

#### 返回值

返回一个 `McpSdkServerConfig` 对象，可以传递给 `ClaudeAgentOptions.mcp_servers`。

#### 示例

```python theme={null}
from claude_agent_sdk import tool, create_sdk_mcp_server, ClaudeAgentOptions


@tool("add", "Add two numbers", {"a": float, "b": float})
async def add(args):
    return {"content": [{"type": "text", "text": f"Sum: {args['a'] + args['b']}"}]}


@tool("multiply", "Multiply two numbers", {"a": float, "b": float})
async def multiply(args):
    return {"content": [{"type": "text", "text": f"Product: {args['a'] * args['b']}"}]}


calculator = create_sdk_mcp_server(
    name="calculator",
    version="2.0.0",
    tools=[add, multiply],  # Pass decorated functions
)

# Use with Claude
options = ClaudeAgentOptions(
    mcp_servers={"calc": calculator},
    allowed_tools=["mcp__calc__add", "mcp__calc__multiply"],
)
```

### `list_sessions()`

列出过去的会话及其元数据。按项目目录筛选或列出所有项目中的会话。同步；立即返回。

```python theme={null}
def list_sessions(
    directory: str | None = None,
    limit: int | None = None,
    offset: int = 0,
    include_worktrees: bool = True
) -> list[SDKSessionInfo]
```

#### 参数

| 参数 | 类型 | 默认值 | 描述 |
| :- | :- | :- | :- |
| `directory` | `str \| None` | `None` | 要列出会话的目录。省略时，返回所有项目中的会话 |
| `limit` | `int \| None` | `None` | 要返回的最大会话数 |
| `offset` | `int` | `0` | 从排序结果开始跳过的会话数。与 `limit` 一起用于分页 |
| `include_worktrees` | `bool` | `True` | 当 `directory` 在 git 仓库内时，包括所有 worktree 路径中的会话 |

#### 返回类型：`SDKSessionInfo`

| 属性 | 类型 | 描述 |
| :- | :- | :- |
| `session_id` | `str` | 唯一会话标识符 |
| `summary` | `str` | 显示标题：自定义标题、最近的提示词、自动生成的摘要或第一个提示词 |
| `last_modified` | `int` | 上次修改时间，以自纪元以来的毫秒为单位 |
| `file_size` | `int \| None` | 会话文件大小（以字节为单位）（远程存储后端为 `None`） |
| `custom_title` | `str \| None` | 会话标题：用户设置的标题，或未设置时的自动生成标题 |
| `first_prompt` | `str \| None` | 会话中第一个有意义的用户提示词 |
| `git_branch` | `str \| None` | 会话结束时的 Git 分支 |
| `cwd` | `str \| None` | 会话的工作目录 |
| `tag` | `str \| None` | 用户设置的会话标签（请参阅 [`tag_session()`](#tag_session)） |
| `created_at` | `int \| None` | 会话创建时间，以自纪元以来的毫秒为单位 |

#### 示例

打印项目的 10 个最近会话。结果按 `last_modified` 降序排序，因此第一项是最新的。省略 `directory` 以搜索所有项目。

```python theme={null}
from claude_agent_sdk import list_sessions

for session in list_sessions(directory="/path/to/project", limit=10):
    print(f"{session.summary} ({session.session_id})")
```

### `get_session_messages()`

从过去的会话中检索消息。同步；立即返回。

```python theme={null}
def get_session_messages(
    session_id: str,
    directory: str | None = None,
    limit: int | None = None,
    offset: int = 0
) -> list[SessionMessage]
```

#### 参数

| 参数 | 类型 | 默认值 | 描述 |
| :- | :- | :- | :- |
| `session_id` | `str` | 必需 | 要检索消息的会话 ID |
| `directory` | `str \| None` | `None` | 要查找的项目目录。省略时，搜索所有项目 |
| `limit` | `int \| None` | `None` | 要返回的最大消息数 |
| `offset` | `int` | `0` | 从开始跳过的消息数 |

#### 返回类型：`SessionMessage`

| 属性 | 类型 | 描述 |
| :- | :- | :- |
| `type` | `Literal["user", "assistant"]` | 消息角色 |
| `uuid` | `str` | 唯一消息标识符 |
| `session_id` | `str` | 会话标识符 |
| `message` | `Any` | 原始消息内容 |
| `parent_tool_use_id` | `str \| None` | 对于子代理消息，生成 `Agent` 工具使用块的 id。对于主会话消息和较旧的会话为 `None` |
| `parent_agent_id` | `str \| None` | 对于来自 [嵌套子代理](https://code.claude.com/docs/zh-CN/sub-agents.md#let-subagents-spawn-their-own-subagents) 的消息，父子代理的 Agent id。对于主会话消息、顶级子代理消息和较旧的会话为 `None`。需要 Python Agent SDK 0.2.140 或更高版本 |

#### 示例

```python theme={null}
from claude_agent_sdk import list_sessions, get_session_messages

sessions = list_sessions(limit=1)
if sessions:
    messages = get_session_messages(sessions[0].session_id)
    for msg in messages:
        print(f"[{msg.type}] {msg.uuid}")
```

### `get_session_info()`

按 ID 读取单个会话的元数据，无需扫描完整项目目录。同步；立即返回。

```python theme={null}
def get_session_info(
    session_id: str,
    directory: str | None = None,
) -> SDKSessionInfo | None
```

#### 参数

| 参数 | 类型 | 默认值 | 描述 |
| :- | :- | :- | :- |
| `session_id` | `str` | 必需 | 要查找的会话的 UUID |
| `directory` | `str \| None` | `None` | 项目目录路径。省略时，搜索所有项目目录 |

返回 [`SDKSessionInfo`](#return-type-sdksessioninfo)，如果找不到会话则返回 `None`。

#### 示例

查找单个会话的元数据，无需扫描项目目录。当您已经从之前的运行中获得会话 ID 时很有用。

```python theme={null}
from claude_agent_sdk import get_session_info

info = get_session_info("550e8400-e29b-41d4-a716-446655440000")
if info:
    print(f"{info.summary} (branch: {info.git_branch}, tag: {info.tag})")
```

### `rename_session()`

通过附加自定义标题条目来重命名会话。重复调用是安全的；最新的标题获胜。同步。

```python theme={null}
def rename_session(
    session_id: str,
    title: str,
    directory: str | None = None,
) -> None
```

#### 参数

| 参数 | 类型 | 默认值 | 描述 |
| :- | :- | :- | :- |
| `session_id` | `str` | 必需 | 要重命名的会话的 UUID |
| `title` | `str` | 必需 | 新标题。去除空格后必须非空 |
| `directory` | `str \| None` | `None` | 项目目录路径。省略时，搜索所有项目目录 |

如果 `session_id` 不是有效的 UUID 或 `title` 为空，则抛出 `ValueError`；如果找不到会话，则抛出 `FileNotFoundError`。

#### 示例

重命名最近的会话，以便稍后更容易找到。新标题在后续读取时出现在 [`SDKSessionInfo.custom_title`](#return-type-sdksessioninfo) 中。

```python theme={null}
from claude_agent_sdk import list_sessions, rename_session

sessions = list_sessions(directory="/path/to/project", limit=1)
if sessions:
    rename_session(sessions[0].session_id, "Refactor auth module")
```

### `tag_session()`

标记会话。传递 `None` 以清除标签。重复调用是安全的；最新的标签获胜。同步。

```python theme={null}
def tag_session(
    session_id: str,
    tag: str | None,
    directory: str | None = None,
) -> None
```

#### 参数

| 参数 | 类型 | 默认值 | 描述 |
| :- | :- | :- | :- |
| `session_id` | `str` | 必需 | 要标记的会话的 UUID |
| `tag` | `str \| None` | 必需 | 标签字符串，或 `None` 以清除。存储前进行 Unicode 清理 |
| `directory` | `str \| None` | `None` | 项目目录路径。省略时，搜索所有项目目录 |

如果 `session_id` 不是有效的 UUID 或 `tag` 在清理后为空，则抛出 `ValueError`；如果找不到会话，则抛出 `FileNotFoundError`。

#### 示例

标记会话，然后在稍后的读取中按该标签筛选。传递 `None` 以清除现有标签。

```python theme={null}
from claude_agent_sdk import list_sessions, tag_session

# Tag the most recent session
sessions = list_sessions(directory="/path/to/project", limit=1)
if sessions:
    tag_session(sessions[0].session_id, "needs-review")

# Later: find all sessions with that tag
for session in list_sessions(directory="/path/to/project"):
    if session.tag == "needs-review":
        print(session.summary)
```

### `fork_session()`

将会话的会话记录复制到新会话中，以便您可以将对话引向另一个方向，同时原始会话保持不变。要从对话中较早的某个点分支，请传递 `up_to_message_id`。同步。

```python theme={null}
def fork_session(
    session_id: str,
    directory: str | None = None,
    up_to_message_id: str | None = None,
    title: str | None = None,
) -> ForkSessionResult
```

#### 参数

| 参数 | 类型 | 默认值 | 描述 |
| :- | :- | :- | :- |
| `session_id` | `str` | 必需 | 要分叉的会话的 UUID |
| `directory` | `str \| None` | `None` | 项目目录路径。省略时，搜索所有项目目录 |
| `up_to_message_id` | `str \| None` | `None` | 复制会话记录直到（并包括）具有此 UUID 的消息，例如来自 [`get_session_messages()`](#get_session_messages) 的 `uuid`。省略时，复制整个会话记录 |
| `title` | `str \| None` | `None` | 分叉的标题。省略时，SDK 会从原始会话派生一个标题，后跟 `(fork)` |

返回一个 `ForkSessionResult`，其 `session_id` 是新会话的 UUID。将其作为 [`resume`](#claudeagentoptions) 传递即可继续该分叉。分叉不包含原始会话的[文件检查点](https://code.claude.com/docs/zh-CN/agent-sdk/file-checkpointing.md)，因此您无法将其回退到分叉之前捕获的检查点。

`fork_session()` 会抛出：

* `ValueError`：`session_id` 或 `up_to_message_id` 不是有效的 UUID
* `ValueError`：会话没有消息，或 `up_to_message_id` 与会话记录中的任何消息都不匹配
* `FileNotFoundError`：找不到会话

#### 示例

以新标题分叉最近的会话，然后恢复该分叉。原始会话保留其自身的历史记录。

```python theme={null}
from claude_agent_sdk import fork_session, list_sessions

sessions = list_sessions(directory="/path/to/project", limit=1)
if sessions:
    forked = fork_session(sessions[0].session_id, title="Try the OAuth approach")
    print(forked.session_id)  # pass as ClaudeAgentOptions(resume=...) to continue the fork
```

## 类

### `ClaudeSDKClient`

**在多次交换中维持对话会话。** 这是 TypeScript SDK 的 `query()` 函数内部工作方式的 Python 等价物 - 它创建一个可以继续对话的客户端对象。见[与 `query()` 的比较](#choosing-between-query-and-claudesdkclient)。

```python theme={null}
class ClaudeSDKClient:
    def __init__(self, options: ClaudeAgentOptions | None = None, transport: Transport | None = None)
    async def connect(self, prompt: str | AsyncIterable[dict] | None = None) -> None
    async def query(self, prompt: str | AsyncIterable[dict], session_id: str = "default") -> None
    async def receive_messages(self) -> AsyncIterator[Message]
    async def receive_response(self) -> AsyncIterator[Message]
    async def interrupt(self) -> None
    async def set_permission_mode(self, mode: PermissionMode) -> None
    async def set_model(self, model: str | None = None) -> None
    async def rewind_files(self, user_message_id: str) -> None
    async def get_mcp_status(self) -> McpStatusResponse
    async def get_context_usage(self) -> ContextUsageResponse
    async def reconnect_mcp_server(self, server_name: str) -> None
    async def toggle_mcp_server(self, server_name: str, enabled: bool) -> None
    async def stop_task(self, task_id: str) -> None
    async def get_server_info(self) -> dict[str, Any] | None
    async def disconnect(self) -> None
```

#### 方法

| 方法 | 描述 |
| :- | :- |
| `__init__(options)` | 使用可选配置初始化客户端 |
| `connect(prompt)` | 连接到 Claude，可选初始提示或消息流 |
| `query(prompt, session_id)` | 以流式模式发送新请求 |
| `receive_messages()` | 以异步迭代器形式接收来自 Claude 的所有消息 |
| `receive_response()` | 接收消息直到并包括 ResultMessage |
| `interrupt()` | 发送中断信号（仅在流式模式下工作） |
| `set_permission_mode(mode)` | 更改当前会话的权限模式 |
| `set_model(model)` | 更改当前会话的模型。传递 `None` 以重置为 [Claude Code 的默认模型](https://code.claude.com/docs/zh-CN/model-config.md)。如果新模型不支持自动模式，处于 `auto` 模式的会话将[切换到 `default` 模式](https://code.claude.com/docs/zh-CN/agent-sdk/permissions#auto-mode-auto) |
| `rewind_files(user_message_id)` | 将文件恢复到指定用户消息时的状态。需要 `enable_file_checkpointing=True`。见 [文件检查点](https://code.claude.com/docs/zh-CN/agent-sdk/file-checkpointing.md) |
| `get_mcp_status()` | 获取所有配置的 MCP 服务器的状态。返回 [`McpStatusResponse`](#mcpstatusresponse) |
| `get_context_usage()` | 获取按类别、skill 和工具划分的上下文窗口使用情况明细。其中包含 `/context` 在交互式会话中显示的数据。返回 [`ContextUsageResponse`](#contextusageresponse)。为了计算明细，Claude Code 会发出几个不出现在消息流中的令牌计数 API 请求；见[这些请求如何处理](#contextusageresponse) |
| `reconnect_mcp_server(server_name)` | 重试连接到失败或断开连接的 MCP 服务器 |
| `toggle_mcp_server(server_name, enabled)` | 在会话中启用或禁用 MCP 服务器。禁用会移除其工具 |
| `stop_task(task_id)` | 停止运行的后台任务。一个状态为 `"stopped"` 的 [`TaskNotificationMessage`](#tasknotificationmessage) 随后在消息流中出现 |
| `get_server_info()` | 获取服务器的初始化信息，包括可用命令和输出样式 |
| `disconnect()` | 从 Claude 断开连接 |

#### 上下文管理器支持

客户端可以用作异步上下文管理器以自动管理连接：

```python theme={null}
import asyncio
from claude_agent_sdk import ClaudeSDKClient


async def main():
    async with ClaudeSDKClient() as client:
        await client.query("Hello Claude")
        async for message in client.receive_response():
            print(message)


asyncio.run(main())
```

> **重要：** 迭代消息时，避免使用 `break` 提前退出，因为这可能导致 asyncio 清理问题。相反，让迭代自然完成或使用标志来跟踪何时找到了你需要的内容。

#### 示例 - 继续对话

```python theme={null}
import asyncio
from claude_agent_sdk import ClaudeSDKClient, AssistantMessage, TextBlock, ResultMessage


async def main():
    async with ClaudeSDKClient() as client:
        # First question
        await client.query("What's the capital of France?")

        # Process response
        async for message in client.receive_response():
            if isinstance(message, AssistantMessage):
                for block in message.content:
                    if isinstance(block, TextBlock):
                        print(f"Claude: {block.text}")

        # Follow-up question - the session retains the previous context
        await client.query("What's the population of that city?")

        async for message in client.receive_response():
            if isinstance(message, AssistantMessage):
                for block in message.content:
                    if isinstance(block, TextBlock):
                        print(f"Claude: {block.text}")

        # Another follow-up - still in the same conversation
        await client.query("What are some famous landmarks there?")

        async for message in client.receive_response():
            if isinstance(message, AssistantMessage):
                for block in message.content:
                    if isinstance(block, TextBlock):
                        print(f"Claude: {block.text}")


asyncio.run(main())
```

#### 示例 - 使用 ClaudeSDKClient 进行流式输入

`query()` 也接受用户消息字典的异步可迭代对象，因此你可以在发送时组装提示或包含内容块，如图像。Claude Code 在第一个产生的消息到达时立即开始响应，无需等待可迭代对象完成，`receive_response()` 在结束该响应的 `ResultMessage` 处停止。将 Claude 应该在回答前读取的所有内容放入一条消息中，如本生成器所做的那样，并将每个 `query()` 调用与其自己的 `receive_response()` 循环配对。

```python theme={null}
import asyncio
from claude_agent_sdk import ClaudeSDKClient


async def message_stream():
    """Assemble the prompt at send time and yield it as one user message."""
    readings = {"Temperature": "25°C", "Humidity": "60%"}
    data = ", ".join(f"{name}: {value}" for name, value in readings.items())
    yield {
        "type": "user",
        "message": {
            "role": "user",
            "content": f"Analyze the following sensor data and describe any patterns you see: {data}",
        },
    }


async def main():
    async with ClaudeSDKClient() as client:
        # Stream input to Claude
        await client.query(message_stream())

        # Process response
        async for message in client.receive_response():
            print(message)

        # Follow-up in same session
        await client.query("Should we be concerned about these readings?")

        async for message in client.receive_response():
            print(message)


asyncio.run(main())
```

#### 示例 - 使用中断

```python theme={null}
import asyncio
from claude_agent_sdk import ClaudeSDKClient, ClaudeAgentOptions, ResultMessage


async def interruptible_task():
    options = ClaudeAgentOptions(allowed_tools=["Bash"], permission_mode="acceptEdits")

    async with ClaudeSDKClient(options=options) as client:
        # Start a long-running task
        await client.query("Count from 1 to 100 slowly, using the bash sleep command")

        # Let it run for a bit
        await asyncio.sleep(2)

        # Interrupt the task
        await client.interrupt()
        print("Task interrupted!")

        # Drain the interrupted task's messages (including its ResultMessage)
        async for message in client.receive_response():
            if isinstance(message, ResultMessage):
                print(f"Interrupted task: terminal_reason={message.terminal_reason!r}")
                # terminal_reason is "aborted_streaming" or "aborted_tools"
                # for interrupted turns

        # Send a new command
        await client.query("Just say hello instead")

        # Now receive the new response
        async for message in client.receive_response():
            if isinstance(message, ResultMessage) and message.subtype == "success":
                print(f"New result: {message.result}")


asyncio.run(interruptible_task())
```

> [!NOTE]
>
> **中断后的缓冲行为：** `interrupt()` 发送停止信号但不清除消息缓冲区。被中断任务已产生的消息，包括其 `ResultMessage`，保留在流中。你必须在读取新查询的响应之前用 `receive_response()` 清空它们。如果在 `interrupt()` 之后立即发送新查询并仅调用一次 `receive_response()`，你将收到被中断任务的消息，而不是新查询的响应。

#### 示例 - 高级权限控制

```python theme={null}
import asyncio
from claude_agent_sdk import ClaudeSDKClient, ClaudeAgentOptions
from claude_agent_sdk.types import (
    PermissionResultAllow,
    PermissionResultDeny,
    ToolPermissionContext,
)


async def custom_permission_handler(
    tool_name: str, input_data: dict, context: ToolPermissionContext
) -> PermissionResultAllow | PermissionResultDeny:
    """Custom logic for tool permissions."""

    # Block writes to system directories
    if tool_name == "Write" and input_data.get("file_path", "").startswith("/system/"):
        return PermissionResultDeny(
            message="System directory write not allowed", interrupt=True
        )

    # Redirect sensitive file operations
    if tool_name in ["Write", "Edit"] and "config" in input_data.get("file_path", ""):
        safe_path = f"./sandbox/{input_data['file_path']}"
        return PermissionResultAllow(
            updated_input={**input_data, "file_path": safe_path}
        )

    # Allow everything else
    return PermissionResultAllow(updated_input=input_data)


async def main():
    # 不要同时在 allowed_tools 中列出受限工具：allow 规则在 can_use_tool 运行之前批准调用
    options = ClaudeAgentOptions(can_use_tool=custom_permission_handler)

    async with ClaudeSDKClient(options=options) as client:
        await client.query("Update the system config file")

        async for message in client.receive_response():
            # Will use sandbox path instead
            print(message)


asyncio.run(main())
```

## 类型

> [!NOTE]
>
> **`@dataclass` vs `TypedDict`：** 此 SDK 使用两种类型。用 `@dataclass` 装饰的类（如 `ResultMessage`、`AgentDefinition`、`TextBlock`）在运行时是对象实例，支持属性访问：`msg.result`。用 `TypedDict` 定义的类（如 `ThinkingConfigEnabled`、`McpStdioServerConfig`、`SyncHookJSONOutput`）在运行时是**普通字典**，需要键访问：`config["budget_tokens"]`，而不是 `config.budget_tokens`。`ClassName(field=value)` 调用语法对两者都有效，但只有数据类产生具有属性的对象。

### `SdkMcpTool`

使用 `@tool` 装饰器创建的 SDK MCP 工具的定义。

```python theme={null}
@dataclass
class SdkMcpTool(Generic[T]):
    name: str
    description: str
    input_schema: type[T] | dict[str, Any]
    handler: Callable[[T], Awaitable[dict[str, Any]]]
    annotations: ToolAnnotations | None = None
```

| 属性 | 类型 | 描述 |
| :- | :- | :- |
| `name` | `str` | 工具的唯一标识符 |
| `description` | `str` | 人类可读的描述 |
| `input_schema` | `type[T] \| dict[str, Any]` | 用于输入验证的 schema |
| `handler` | `Callable[[T], Awaitable[dict[str, Any]]]` | 处理工具执行的异步函数 |
| `annotations` | [`ToolAnnotations`](#toolannotations)` \| None` | 可选的工具注解（例如 `readOnlyHint`、`destructiveHint`、`openWorldHint`、`maxResultSizeChars`） |

### `Transport`

自定义传输实现的抽象基类。使用此类通过自定义通道与 Claude 进程通信（例如，远程连接而不是本地子进程）。

> [!WARNING]
>
> 这是一个低级内部 API。接口可能在未来版本中更改。自定义实现必须更新以匹配任何接口更改。

```python theme={null}
from abc import ABC, abstractmethod
from collections.abc import AsyncIterator
from typing import Any


class Transport(ABC):
    @abstractmethod
    async def connect(self) -> None: ...

    @abstractmethod
    async def write(self, data: str) -> None: ...

    @abstractmethod
    def read_messages(self) -> AsyncIterator[dict[str, Any]]: ...

    @abstractmethod
    async def close(self) -> None: ...

    @abstractmethod
    def is_ready(self) -> bool: ...

    @abstractmethod
    async def end_input(self) -> None: ...
```

| 方法 | 描述 |
| :- | :- |
| `connect()` | 连接传输并准备通信 |
| `write(data)` | 将原始数据（JSON + 换行符）写入传输 |
| `read_messages()` | 异步迭代器，产生解析的 JSON 消息 |
| `close()` | 关闭连接并清理资源 |
| `is_ready()` | 如果传输可以发送和接收，返回 `True` |
| `end_input()` | 关闭输入流（例如，为子进程传输关闭 stdin） |

导入：`from claude_agent_sdk import Transport`

### `ClaudeAgentOptions`

Claude Code 查询的配置数据类。

```python theme={null}
@dataclass
class ClaudeAgentOptions:
    tools: list[str] | ToolsPreset | None = None
    allowed_tools: list[str] = field(default_factory=list)
    system_prompt: str | SystemPromptPreset | SystemPromptCustom | SystemPromptFile | None = None
    mcp_servers: dict[str, McpServerConfig] | str | Path = field(default_factory=dict)
    strict_mcp_config: bool = False
    permission_mode: PermissionMode | None = None
    continue_conversation: bool = False
    resume: str | None = None
    session_id: str | None = None
    max_turns: int | None = None
    max_budget_usd: float | None = None
    disallowed_tools: list[str] = field(default_factory=list)
    model: str | None = None
    fallback_model: str | None = None
    betas: list[SdkBeta] = field(default_factory=list)
    output_format: dict[str, Any] | None = None
    permission_prompt_tool_name: str | None = None
    cwd: str | Path | None = None
    cli_path: str | Path | None = None
    settings: str | None = None
    add_dirs: list[str | Path] = field(default_factory=list)
    env: dict[str, str] = field(default_factory=dict)
    extra_args: dict[str, str | None] = field(default_factory=dict)
    max_buffer_size: int | None = None
    debug_stderr: Any = sys.stderr  # Deprecated
    stderr: Callable[[str], None] | None = None
    can_use_tool: CanUseTool | None = None
    hooks: dict[HookEvent, list[HookMatcher]] | None = None
    user: str | None = None
    include_partial_messages: bool = False
    include_hook_events: bool = False
    forward_subagent_text: bool = False
    verbatim_prompts: bool = False
    fork_session: bool = False
    resume_session_at: str | None = None
    resume_drops_turn: str | None = None
    agents: dict[str, AgentDefinition] | None = None
    setting_sources: list[SettingSource] | None = None
    skills: list[str] | Literal["all"] | None = None
    sandbox: SandboxSettings | None = None
    plugins: list[SdkPluginConfig] = field(default_factory=list)
    max_thinking_tokens: int | None = None  # Deprecated: use thinking instead
    thinking: ThinkingConfig | None = None
    effort: EffortLevel | None = None
    enable_file_checkpointing: bool = False
    session_store: SessionStore | None = None
    session_store_flush: SessionStoreFlushMode = "batched"
    load_timeout_ms: int = 60_000
    task_budget: TaskBudget | None = None
```

| 属性 | 类型 | 默认值 | 描述 |
| :- | :- | :- | :- |
| `tools` | `list[str] \| ToolsPreset \| None` | `None` | 工具配置。使用 `{"type": "preset", "preset": "claude_code"}` 获取 Claude Code 的默认工具 |
| `allowed_tools` | `list[str]` | `[]` | 无需提示即可自动批准的工具，但从[网络路径](https://code.claude.com/docs/zh-CN/permissions.md#network-paths)进行的读取除外。这不会限制 Claude 仅使用这些工具。如果您在此处列出[任务跟踪工具](https://code.claude.com/docs/zh-CN/agent-sdk/todo-tracking.md#model-availability)之一，Claude Code 也会为该会话启用它们。其他未列出的工具会交由 `permission_mode` 和 `can_use_tool` 处理。使用 `disallowed_tools` 阻止工具。见 [权限](https://code.claude.com/docs/zh-CN/agent-sdk/permissions.md#allow-and-deny-rules) |
| `system_prompt` | `str \| SystemPromptPreset \| SystemPromptCustom \| SystemPromptFile \| None` | `None` | 系统提示词配置。传递字符串以使用自定义提示词，`{"type": "preset", "preset": "claude_code"}` 以使用 Claude Code 的系统提示词（带可选 `"append"`），`{"type": "custom", "prompt": "..."}` 以使用也可以设置 `"snapshot"` 的自定义提示词，或 `{"type": "file", "path": "..."}` 从磁盘加载大型提示词。见 [`SystemPromptPreset`](#systempromptpreset)、[`SystemPromptCustom`](#systempromptcustom) 和 [`SystemPromptFile`](#systempromptfile) |
| `mcp_servers` | `dict[str, McpServerConfig] \| str \| Path` | `{}` | MCP 服务器配置或配置文件路径 |
| `strict_mcp_config` | `bool` | `False` | 当为 `True` 时，仅使用在 `mcp_servers` 中传递的服务器，忽略项目 `.mcp.json`、用户设置、插件提供的 MCP 服务器和 [claude.ai 连接器](https://code.claude.com/docs/zh-CN/mcp.md#use-mcp-servers-from-claude-ai)。映射到 CLI `--strict-mcp-config` 标志 |
| `permission_mode` | `PermissionMode \| None` | `None` | 工具使用的权限模式 |
| `continue_conversation` | `bool` | `False` | 继续最近的对话 |
| `resume` | `str \| None` | `None` | 要恢复的会话 ID |
| `session_id` | `str \| None` | `None` | 使用特定的会话 ID 而不是自动生成的。必须是有效的 UUID。不能与 `continue_conversation` 或 `resume` 结合使用，除非也设置了 `fork_session` |
| `max_turns` | `int \| None` | `None` | 最大 Agent [轮次](https://code.claude.com/docs/zh-CN/agent-sdk/agent-loop.md#turns-and-messages) |
| `max_budget_usd` | `float \| None` | `None` | 当客户端成本估计达到此 USD 值时停止查询。该估计值可能超过此值，因此请[预留余量](https://code.claude.com/docs/zh-CN/agent-sdk/agent-loop.md#budget-headroom)。仅计算调用自身的支出；从恢复的会话恢复的总数不计算。有关准确性注意事项和重置行为，见 [跟踪成本和使用](https://code.claude.com/docs/zh-CN/agent-sdk/cost-tracking.md) |
| `disallowed_tools` | `list[str]` | `[]` | 要拒绝的工具。裸名称如 `"Bash"` 从 Claude 的上下文中移除工具。限定规则如 `"Bash(rm *)"` 保持工具可用，并在每个权限模式（包括 `bypassPermissions`）中拒绝匹配的调用，针对[按书写形式](https://code.claude.com/docs/zh-CN/permissions.md#bash-rule-limits)的命令。见 [权限](https://code.claude.com/docs/zh-CN/agent-sdk/permissions.md#allow-and-deny-rules) |
| `enable_file_checkpointing` | `bool` | `False` | 启用文件更改跟踪以进行回滚。见 [文件检查点功能](https://code.claude.com/docs/zh-CN/agent-sdk/file-checkpointing.md) |
| `model` | `str \| None` | `None` | Claude 模型别名或完整模型名称。见 [接受的值和特定于提供商的 ID](https://code.claude.com/docs/zh-CN/model-config.md#available-models) |
| `fallback_model` | `str \| None` | `None` | 主模型失败时使用的备用模型。接受逗号分隔的列表。有关指导，见 [选择模型](https://code.claude.com/docs/zh-CN/agent-sdk/configuration.md#choose-a-model) |
| `betas` | `list[SdkBeta]` | `[]` | 要启用的测试功能。见 [`SdkBeta`](#sdkbeta) 了解可用选项 |
| `output_format` | `dict[str, Any] \| None` | `None` | 结构化响应的输出格式（例如 `{"type": "json_schema", "schema": {...}}`）。见 [结构化输出](https://code.claude.com/docs/zh-CN/agent-sdk/structured-outputs.md) 了解详情 |
| `permission_prompt_tool_name` | `str \| None` | `None` | 权限提示的 MCP 工具名称 |
| `cwd` | `str \| Path \| None` | `None` | 当前工作目录 |
| `cli_path` | `str \| Path \| None` | `None` | Claude Code CLI 可执行文件的自定义路径 |
| `settings` | `str \| None` | `None` | 设置文件的路径或内联 JSON 字符串 |
| `add_dirs` | `list[str \| Path]` | `[]` | Claude 可以访问的其他目录。SDK 将每个条目作为 `--add-dir` 传递给 Claude Code，因此使用 `project` 设置源时，Claude Code 也会[加载目录的 skill、命令和子代理](https://code.claude.com/docs/zh-CN/permissions.md#additional-directories-grant-file-access-not-configuration) |
| `env` | `dict[str, str]` | `{}` | 合并到继承的进程环境之上的环境变量。见 [环境变量](https://code.claude.com/docs/zh-CN/env-vars.md) 了解底层 CLI 读取的变量，以及 [处理缓慢或停滞的 API 响应](#handle-slow-or-stalled-api-responses) 了解超时相关变量。设置 `CLAUDE_AGENT_SDK_CLIENT_APP` 以在 User-Agent 标头中标识您的应用 |
| `extra_args` | `dict[str, str \| None]` | `{}` | 直接传递给 CLI 的其他 CLI 参数 |
| `max_buffer_size` | `int \| None` | `None` | 缓冲 CLI stdout 时的最大字节数 |
| `debug_stderr` | `Any` | `sys.stderr` | *已弃用* - SDK 忽略此值。使用 `stderr` 回调获取 CLI stderr 输出 |
| `stderr` | `Callable[[str], None] \| None` | `None` | CLI 中 stderr 输出的回调函数 |
| `can_use_tool` | [`CanUseTool`](#canusetool) ` \| None` | `None` | 工具权限回调，仅在[权限流程](https://code.claude.com/docs/zh-CN/agent-sdk/permissions.md#how-permissions-are-evaluated)落到提示时调用。对于由 `allowed_tools`、允许规则或 `permission_mode` 自动批准的调用不会调用。允许规则不会预批准[任何模式都不自动批准的操作](https://code.claude.com/docs/zh-CN/permission-modes.md#actions-no-mode-auto-approves)。见 [`CanUseTool`](#canusetool) 了解详情 |
| `hooks` | `dict[HookEvent, list[HookMatcher]] \| None` | `None` | 用于拦截事件的 hook 配置 |
| `user` | `str \| None` | `None` | 在 POSIX 平台上，Claude Code 子进程运行的 OS 用户账户。Claude Code 保持父进程的环境，包括 `HOME`，并在 `cwd` 中运行 |
| `include_partial_messages` | `bool` | `False` | 包括部分消息流式事件。启用时，会产生 [`StreamEvent`](#streamevent) 消息 |
| `include_hook_events` | `bool` | `False` | 在消息流中以 `HookEventMessage` 对象的形式包括 hook 生命周期事件 |
| `forward_subagent_text` | `bool` | `False` | 在消息流中转发子代理文本和思考块。没有此选项时，Claude Code 会省略在[前台](https://code.claude.com/docs/zh-CN/sub-agents.md#run-subagents-in-foreground-or-background)运行的子代理的文本和思考块。有关嵌套子代理、带有 `context: fork` 的 skill 以及各自所需的 Claude Code 版本，见 [跟踪子代理消息](https://code.claude.com/docs/zh-CN/headless.md#follow-subagent-messages)。需要 Python Agent SDK 0.2.140 或更高版本 |
| `verbatim_prompts` | `bool` | `False` | 按原样传递每个提示词。SDK 发送每条用户消息时将 `client_composed` 设置为 `True`。见 [`client_composed`](https://code.claude.com/docs/zh-CN/agent-sdk/typescript#sdkusermessage) 了解 Claude Code 在这些消息上跳过的内容。当您的提示词文本包含最终用户未输入的内容时使用此选项。如需按轮次控制，请保持关闭，并改为在单个流式消息上设置 `"client_composed": True`。启用此选项时，SDK 会覆盖您设置的任何 `client_composed` 值。需要 Python Agent SDK 0.2.158 或更高版本以及 Claude Code v2.1.248 或更高版本；这些 SDK 版本附带的 CLI 满足 Claude Code 要求 |
| `fork_session` | `bool` | `False` | 使用 `resume` 恢复时，分叉到新会话 ID 而不是继续原始会话 |
| `resume_session_at` | `str \| None` | `None` | 恢复时，仅加载对话直到并包括具有此 UUID 的消息。与 `resume` 一起使用，通常还要使用 `fork_session`，以从较早的点创建分支。需要 Python Agent SDK 0.2.137 或更高版本 |
| `resume_drops_turn` | `str \| None` | `None` | 其轮次被 `resume_session_at` 截断丢弃的用户提示词的 UUID。设置时，如果丢弃的范围包含不可归因于该轮次的条目，CLI 会拒绝恢复。需要 Python Agent SDK 0.2.137 或更高版本以及 Claude Code v2.1.223 或更高版本；这些 SDK 版本附带的 CLI 满足 Claude Code 要求 |
| `agents` | `dict[str, AgentDefinition] \| None` | `None` | 以编程方式定义的子代理 |
| `plugins` | `list[SdkPluginConfig]` | `[]` | 从本地路径加载自定义插件。见 [插件](https://code.claude.com/docs/zh-CN/agent-sdk/plugins.md) 了解详情 |
| `sandbox` | [`SandboxSettings`](#sandboxsettings) ` \| None` | `None` | 以编程方式配置沙箱行为。见 [沙箱设置](#sandboxsettings) 了解详情 |
| `setting_sources` | `list[SettingSource] \| None` | `None`（CLI 默认值：所有源） | 控制加载哪些文件系统设置。传递 `[]` 以禁用用户、项目和本地设置。如果设置了 `skills` 而未设置此字段，则仅加载用户和项目源。显式设置 `setting_sources` 以保留本地设置。端点托管策略始终会加载；当会话使用组织凭据在[符合条件的配置](https://code.claude.com/docs/zh-CN/server-managed-settings.md#platform-availability)上进行身份验证时，会获取服务器托管设置。有关无论此选项如何都会读取的输入，见 [settingSources 不控制什么](https://code.claude.com/docs/zh-CN/agent-sdk/claude-code-features.md#what-settingsources-does-not-control) |
| `skills` | `list[str] \| Literal["all"] \| None` | `None` | 会话可用的 skill。传递 `"all"` 以启用每个发现的 skill，或传递 skill 名称列表。仅传递精确名称。SDK 在启动 Claude Code 进程之前会以 `ValueError` 拒绝格式错误和通配符形式的名称；此检查需要 Python Agent SDK 0.2.129 或更高版本。设置时，SDK 会自动将 Skill 工具添加到 `allowed_tools`。如果您也传递 `tools`，请在该列表中包含 `"Skill"`。见 [Skills](https://code.claude.com/docs/zh-CN/agent-sdk/skills.md) |
| `max_thinking_tokens` | `int \| None` | `None` | *已弃用* - 思考块的最大 token 数。改用 `thinking` |
| `thinking` | [`ThinkingConfig`](#thinkingconfig) ` \| None` | `None` | 控制扩展思考行为。优先于 `max_thinking_tokens` |
| `effort` | [`EffortLevel`](#effortlevel) ` \| None` | `None` | 思考深度的 effort 级别。见 [调整 effort 级别](https://code.claude.com/docs/zh-CN/model-config.md#adjust-effort-level) |
| `session_store` | [`SessionStore`](https://code.claude.com/docs/zh-CN/agent-sdk/session-storage#the-sessionstore-interface) ` \| None` | `None` | 将会话记录镜像到外部后端，以便另一台主机可以恢复它们。见 [将会话持久化到外部存储](https://code.claude.com/docs/zh-CN/agent-sdk/session-storage.md) |
| `session_store_flush` | `Literal["batched", "eager"]` | `"batched"` | 何时将镜像的会话记录条目刷新到 `session_store`。`"batched"` 每轮刷新一次或在缓冲区填满时刷新；`"eager"` 在每帧后触发后台刷新。当 `session_store` 为 `None` 时忽略 |
| `load_timeout_ms` | `int` | `60000` | 在恢复物化期间，`session_store.load()` 和 `list_subkeys()` 的每次调用超时时间，以毫秒为单位 |
| `task_budget` | `TaskBudget \| None` | `None` | API 端 token 预算。作为 `output_config.task_budget` 随 `task-budgets-2026-03-13` 测试版标头发送。传递 `{"total": <int>}`。 |

#### 处理缓慢或停滞的 API 响应

CLI 子进程读取多个环境变量，这些变量控制 API 超时和停滞检测。通过 `ClaudeAgentOptions.env` 传递它们：

```python theme={null}
from claude_agent_sdk import ClaudeAgentOptions

options = ClaudeAgentOptions(
    env={
        "API_TIMEOUT_MS": "120000",
        "CLAUDE_CODE_MAX_RETRIES": "2",
        "CLAUDE_ASYNC_AGENT_STALL_TIMEOUT_MS": "120000",
    },
)
```

* [`API_TIMEOUT_MS`](https://code.claude.com/docs/zh-CN/env-vars)：每次请求尝试可以等待响应标头的时长，以毫秒为单位。它不限制标头到达后流式响应的持续时间。默认 `600000`。适用于主循环和所有子代理。
* `CLAUDE_CODE_MAX_RETRIES`：最大 API 重试次数。默认 `10`，上限为 `15`。每次重试都有自己的 `API_TIMEOUT_MS` 窗口。

  对于需要等待更长时间中断的无人值守运行，设置 [`CLAUDE_CODE_RETRY_WATCHDOG=1`](https://code.claude.com/docs/zh-CN/errors#tune-retry-behavior)：它会无限期重试瞬时容量错误，并且在 Claude Code v2.1.199 或更高版本上，将其他瞬时错误的默认值提高到 `300` 并移除此变量的上限。
* `CLAUDE_ASYNC_AGENT_STALL_TIMEOUT_MS`：子代理的停滞监视器。当流监视器开启时，默认值为 `CLAUDE_STREAM_IDLE_TIMEOUT_MS` 加 5 分钟，即 `600000`，除非您提高该变量。当流监视器关闭时，默认值为 `600000`。在 v2.1.257 之前，默认值始终为 `600000`。

  计时器在每个流事件时重置。停滞时，Claude Code 中止子代理并向父 Agent 报告停滞。对于后台子代理，它也会将任务标记为失败并附加任何部分结果。
* `CLAUDE_ENABLE_STREAM_WATCHDOG` 与 `CLAUDE_STREAM_IDLE_TIMEOUT_MS`：流监视器，当标头已到达但响应体停止流式传输时中止请求。监视器对所有提供商默认启用；设置 `CLAUDE_ENABLE_STREAM_WATCHDOG=0` 以禁用它。`CLAUDE_STREAM_IDLE_TIMEOUT_MS` 默认为 `300000`，且不能低于该最小值。中止后 Claude Code 会如何处理取决于响应已进行到什么程度，详见[自动重试](https://code.claude.com/docs/zh-CN/errors.md#automatic-retries)。

  当 `ANTHROPIC_BASE_URL` 后面的网关通过 keep-alive ping 保持响应打开、而监视器在等待该响应时，设置了 `include_partial_messages` 的主机会持续接收 `ping` [`StreamEvent`](#streamevent) 消息。请将这些帧视为存活信号，而不要因为静默而使会话超时。在 v2.1.257 之前，这些帧会在最后一个真实流事件后 5 分钟停止。

### `OutputFormat`

结构化输出验证的配置。将其作为 `dict` 传递给 `ClaudeAgentOptions` 上的 `output_format` 字段：

```python theme={null}
# Expected dict shape for output_format
{
    "type": "json_schema",
    "schema": {...},  # Your JSON Schema definition
}
```

| 字段 | 必需 | 描述 |
| :- | :- | :- |
| `type` | 是 | 必须是 `"json_schema"` 用于 JSON Schema 验证 |
| `schema` | 是 | 用于输出验证的 JSON Schema 定义 |

### `SystemPromptPreset`

使用 Claude Code 的预设系统提示词和可选添加内容的配置。

```python theme={null}
class SystemPromptPreset(TypedDict):
    type: Literal["preset"]
    preset: Literal["claude_code"]
    append: NotRequired[str]
    exclude_dynamic_sections: NotRequired[bool]
    snapshot: NotRequired[bool]
```

| 字段 | 必需 | 描述 |
| :- | :- | :- |
| `type` | 是 | 必须是 `"preset"` 以使用预设系统提示词 |
| `preset` | 是 | 必须是 `"claude_code"` 以使用 Claude Code 的系统提示词 |
| `append` | 否 | 要追加到预设系统提示词的其他说明 |
| `exclude_dynamic_sections` | 否 | 将每个用户的上下文（如自动记忆位置）从系统提示词移到第一条用户消息。改进跨用户和机器的提示词缓存重用。见 [修改系统提示词](https://code.claude.com/docs/zh-CN/agent-sdk/modifying-system-prompts.md#improve-prompt-caching-across-users-and-machines) |
| `snapshot` | 否 | 设置为 `False` 以在每个请求上重建系统提示词，而不是[重用会话在其第一个请求上记录的提示词](https://code.claude.com/docs/zh-CN/agent-sdk/modifying-system-prompts.md#change-the-prompt-of-an-existing-session)。需要 `claude-agent-sdk` v0.2.153 或更高版本 |

### `SystemPromptCustom`

对象形式的自定义系统提示词，等同于将字符串作为 `system_prompt` 传递，也可以设置 `snapshot`。需要 `claude-agent-sdk` v0.2.153 或更高版本。

```python theme={null}
class SystemPromptCustom(TypedDict):
    type: Literal["custom"]
    prompt: str
    snapshot: NotRequired[bool]
```

| 字段 | 必需 | 描述 |
| :- | :- | :- |
| `type` | 是 | 必须是 `"custom"` |
| `prompt` | 是 | 系统提示词文本。作为命令行参数传递给 CLI，因此[命令行长度限制](#systempromptfile)适用 |
| `snapshot` | 否 | 与 [`SystemPromptPreset.snapshot`](#systempromptpreset) 相同，应用于 `prompt` |

### `SystemPromptFile`

从文件加载自定义系统提示词而不是作为字符串传递的配置。SDK 将其映射到 CLI [`--system-prompt-file`](https://code.claude.com/docs/zh-CN/cli-reference#system-prompt-flags) 标志。当提示词很大时使用文件形式：SDK 在 CLI 子进程 argv 上传递字符串 `system_prompt`，这在 SDK 发送任何 API 请求之前就受到 OS 命令行长度限制的约束。在 Linux 上，单个参数长于大约 128 KB 会在进程生成时失败，出现 `Argument list too long`。在 Windows 上，整个命令行被限制为大约 32 KB，因此字符串形式在更低的阈值处失败。

```python theme={null}
class SystemPromptFile(TypedDict):
    type: Literal["file"]
    path: str
```

| 字段 | 必需 | 描述 |
| :- | :- | :- |
| `type` | 是 | 必须是 `"file"` 以从磁盘加载提示词 |
| `path` | 是 | 包含系统提示词的文件的路径 |

### `SettingSource`

控制 SDK 从哪些基于文件系统的配置源加载设置。

```python theme={null}
SettingSource = Literal["user", "project", "local"]
```

| 值 | 描述 | 位置 |
| :- | :- | :- |
| `"user"` | 全局用户设置 | `~/.claude/settings.json` |
| `"project"` | 共享项目设置（版本控制） | `.claude/settings.json` |
| `"local"` | 本地项目设置，当 Claude Code 将设置保存到其中时会被加入 gitignore | `.claude/settings.local.json` |

#### 默认行为

当 `setting_sources` 被省略或为 `None` 且 `skills` 未设置时，`query()` 加载与 Claude Code CLI 相同的文件系统设置：用户、项目和本地。设置了 `skills` 时，[`setting_sources`](#claudeagentoptions) 行描述当前默认值。端点托管策略在所有情况下都会加载；当会话使用组织凭据在[符合条件的配置](https://code.claude.com/docs/zh-CN/server-managed-settings.md#platform-availability)上进行身份验证时，会获取服务器托管设置。有关更多信息，见 [settingSources 不控制什么](https://code.claude.com/docs/zh-CN/agent-sdk/claude-code-features.md#what-settingsources-does-not-control)。

#### 为什么使用 setting\_sources

**禁用文件系统设置：**

```python theme={null}
# Do not load user, project, or local settings from disk
import asyncio
from claude_agent_sdk import query, ClaudeAgentOptions


async def main():
    async for message in query(
        prompt="Analyze this code",
        options=ClaudeAgentOptions(
            setting_sources=[]
        ),
    ):
        print(message)


asyncio.run(main())
```

> [!NOTE]
>
> 在 Python SDK 0.1.59 及更早版本中，空列表的处理方式与省略选项相同，因此 `setting_sources=[]` 不会禁用文件系统设置。如果您需要空列表生效，请升级到较新版本。TypeScript SDK 不受影响。

**仅加载特定设置源：**

```python theme={null}
# Load only project settings, ignore user and local
import asyncio
from claude_agent_sdk import query, ClaudeAgentOptions


async def main():
    async for message in query(
        prompt="Run CI checks",
        options=ClaudeAgentOptions(
            setting_sources=["project"]  # Only .claude/settings.json
        ),
    ):
        print(message)


asyncio.run(main())
```

**仅 SDK 应用程序：**

```python theme={null}
# Define everything programmatically.
# Pass [] to opt out of filesystem setting sources.
import asyncio
from claude_agent_sdk import AgentDefinition, ClaudeAgentOptions, query


async def main():
    async for message in query(
        prompt="Review this PR",
        options=ClaudeAgentOptions(
            setting_sources=[],
            agents={
                "code-reviewer": AgentDefinition(
                    description="Reviews code changes",
                    prompt="You are a code reviewer. Report issues in the diff.",
                ),
            },
            allowed_tools=["Read", "Grep", "Glob"],
        ),
    ):
        print(message)


asyncio.run(main())
```

要加载 CLAUDE.md 项目说明，请在 `setting_sources` 中包含 `"project"`。见 [修改系统提示词](https://code.claude.com/docs/zh-CN/agent-sdk/modifying-system-prompts.md#claude-md-files-for-project-level-instructions) 了解 CLAUDE.md 加载如何与系统提示词选项交互。

#### 设置优先级

加载多个源时，设置按此优先级合并（从高到低）：

1. 本地设置（`.claude/settings.local.json`）
2. 项目设置（`.claude/settings.json`）
3. 用户设置（`~/.claude/settings.json`）

编程选项（如 `agents`、`allowed_tools` 和 `settings`）覆盖用户、项目和本地文件系统设置。托管策略设置优先于编程选项。

### `AgentDefinition`

以编程方式定义的子代理的配置。

```python theme={null}
@dataclass
class AgentDefinition:
    description: str
    prompt: str
    tools: list[str] | None = None
    disallowedTools: list[str] | None = None
    model: str | None = None
    skills: list[str] | None = None
    memory: Literal["user", "project", "local"] | None = None
    mcpServers: list[str | dict[str, Any]] | None = None
    initialPrompt: str | None = None
    maxTurns: int | None = None
    background: bool | None = None
    effort: EffortLevel | int | None = None
    permissionMode: PermissionMode | None = None
```

| 字段 | 必需 | 描述 |
| :- | :- | :- |
| `description` | 是 | 何时使用此 Agent 的自然语言描述 |
| `prompt` | 是 | Agent 的系统提示词 |
| `tools` | 否 | 允许的工具名称数组。如果省略，继承[子代理可用的每个工具](https://code.claude.com/docs/zh-CN/sub-agents.md#available-tools) |
| `disallowedTools` | 否 | 要从 Agent 的工具集中移除的工具名称数组。也接受 MCP 服务器级别的模式：`mcp__server` 或 `mcp__server__*` 移除该服务器的每个工具，`mcp__*` 移除任何服务器的每个 MCP 工具 |
| `model` | 否 | 此 Agent 的模型覆盖。接受别名如 `"sonnet"`、`"opus"`、`"haiku"` 或 `"inherit"`，或完整模型 ID。当您省略它时，Claude Code 按[子代理模型顺序](https://code.claude.com/docs/zh-CN/sub-agents.md#choose-a-model)选择模型 |
| `skills` | 否 | 启动时预加载到 Agent 上下文中的 skill 名称列表。未列出的 skill 仍可通过 Skill 工具调用 |
| `memory` | 否 | 此 Agent 的记忆源：`"user"`、`"project"` 或 `"local"` |
| `mcpServers` | 否 | 此 Agent 可用的 MCP 服务器。每个条目是服务器名称或内联 `{name: config}` 字典 |
| `initialPrompt` | 否 | 当此 Agent 作为主线程 Agent 运行时，自动提交为第一个用户轮次 |
| `maxTurns` | 否 | Agent 停止前的最大 Agent 轮次数 |
| `background` | 否 | 调用时将此 Agent 作为非阻塞后台任务运行 |
| `effort` | 否 | 此 Agent 的推理 effort 级别。接受命名级别或整数。见 [`EffortLevel`](#effortlevel) |
| `permissionMode` | 否 | 此 Agent 内工具执行的权限模式。[子代理继承规则](https://code.claude.com/docs/zh-CN/agent-sdk/permissions.md#available-modes)决定何时应用。见 [`PermissionMode`](#permissionmode) |

> [!NOTE]
>
> `AgentDefinition` 字段名称使用 camelCase，如 `disallowedTools`、`permissionMode` 和 `maxTurns`。这些名称直接映射到与 TypeScript SDK 共享的线路格式。这与 `ClaudeAgentOptions` 不同，后者对等效的顶级字段（如 `disallowed_tools` 和 `permission_mode`）使用 Python snake\_case。因为 `AgentDefinition` 是数据类，传递 snake\_case 关键字在构造时会引发 `TypeError`。

### `PermissionMode`

用于控制工具执行的权限模式。

```python theme={null}
PermissionMode = Literal[
    "default",  # Standard permission behavior
    "acceptEdits",  # Auto-accept file edits
    "plan",  # Planning mode - explore without editing
    "dontAsk",  # Deny anything not pre-approved instead of prompting
    "bypassPermissions",  # Bypass permission checks; explicit ask rules still prompt (use with caution)
    "auto",  # A model classifier reviews actions such as shell commands and network requests
]
```

### `EffortLevel`

用于指导思考深度的 effort 级别。

```python theme={null}
EffortLevel = Literal[
    "low",  # Minimal thinking, fastest responses
    "medium",  # Moderate thinking
    "high",  # Deep reasoning
    "xhigh",  # Extended reasoning; falls back to "high" on models that don't support it
    "max",  # Maximum effort
]
```

### `CanUseTool`

工具权限回调函数的类型别名。

```python theme={null}
CanUseTool = Callable[
    [str, dict[str, Any], ToolPermissionContext], Awaitable[PermissionResult]
]
```

回调接收：

* `tool_name`：被调用的工具的名称
* `input_data`：工具的输入参数
* `context`：带有附加信息的 `ToolPermissionContext`

返回 `PermissionResult`（`PermissionResultAllow` 或 `PermissionResultDeny`）。

回调是 SDK 对交互式权限提示的替代：它仅在[权限评估流程](https://code.claude.com/docs/zh-CN/agent-sdk/permissions.md#how-permissions-are-evaluated)解析为提示时调用。已由 `allowed_tools` 条目、设置中的允许规则或权限模式（如 `acceptEdits` 或 `bypassPermissions`）批准的工具调用永远不会调用它。要把关每个工具调用，请改用 [`PreToolUse` hook](https://code.claude.com/docs/zh-CN/agent-sdk/hooks)。

允许规则不会预批准[任何模式都不自动批准的操作](https://code.claude.com/docs/zh-CN/permission-modes.md#actions-no-mode-auto-approves)；见 [权限如何被评估](https://code.claude.com/docs/zh-CN/agent-sdk/permissions.md#how-permissions-are-evaluated) 了解其中哪些会到达回调，以及在 `dontAsk` 和 `auto` 模式下会发生什么。

### `ToolPermissionContext`

传递给工具权限回调的上下文信息。

```python theme={null}
@dataclass
class ToolPermissionContext:
    signal: Any | None = None  # Future: abort signal support
    suggestions: list[PermissionUpdate] = field(default_factory=list)
    tool_use_id: str | None = None
    agent_id: str | None = None
    blocked_path: str | None = None
    decision_reason: str | None = None
    title: str | None = None
    display_name: str | None = None
    description: str | None = None
```

| 字段 | 类型 | 描述 |
| :- | :- | :- |
| `signal` | `Any \| None` | 保留供将来中止信号支持 |
| `suggestions` | `list[PermissionUpdate]` | 来自 CLI 的权限更新建议。Bash 提示包括带有 `localSettings` 目标的建议，因此在 `updated_permissions` 中返回它会将规则写入 `.claude/settings.local.json` 并在会话间持久化。 |
| `tool_use_id` | `str \| None` | 此提示所针对的特定工具调用的标识符。传递给 `can_use_tool` 时始终填充 |
| `agent_id` | `str \| None` | 当调用来自子代理时的子代理 ID；主 Agent 为 `None` |
| `blocked_path` | `str \| None` | 触发权限请求的文件路径（如适用）。例如，当 Bash 命令尝试访问允许目录外的路径时 |
| `decision_reason` | `str \| None` | 触发此权限请求的原因。当 PreToolUse hook 返回 `"ask"` 时，从该 hook 的 `permissionDecisionReason` 转发 |
| `title` | `str \| None` | 完整权限提示句子，如 `Claude wants to read foo.txt`。存在时用作主要提示文本 |
| `display_name` | `str \| None` | 工具操作的短名词短语，如 `Read file`，适合按钮标签 |
| `description` | `str \| None` | 权限 UI 的人类可读副标题 |

### `PermissionResult`

权限回调结果的联合类型。

```python theme={null}
PermissionResult = PermissionResultAllow | PermissionResultDeny
```

### `PermissionResultAllow`

指示应允许工具调用的结果。

```python theme={null}
@dataclass
class PermissionResultAllow:
    behavior: Literal["allow"] = "allow"
    updated_input: dict[str, Any] | None = None
    updated_permissions: list[PermissionUpdate] | None = None
```

| 字段 | 类型 | 默认值 | 描述 |
| :- | :- | :- | :- |
| `behavior` | `Literal["allow"]` | `"allow"` | 必须是 "allow" |
| `updated_input` | `dict[str, Any] \| None` | `None` | 要使用的修改后的输入而不是原始输入 |
| `updated_permissions` | `list[PermissionUpdate] \| None` | `None` | 要应用的权限更新 |

如果您返回的 `PermissionResultAllow` 在 `updated_permissions` 中包含超过 4,096 项，Claude Code 会将该结果视为无效并拒绝该工具调用。每个 [`PermissionUpdate`](#permissionupdate) 计为一项，其 `rules` 或 `directories` 列表中的每个条目也各计为一项。拒绝消息以 `updatedPermissions may hold at most 4096 updates, rules and directories in all.` 结尾。为避免被拒绝，请返回不超过 4,096 项。

### `PermissionResultDeny`

指示应拒绝工具调用的结果。

```python theme={null}
@dataclass
class PermissionResultDeny:
    behavior: Literal["deny"] = "deny"
    message: str = ""
    interrupt: bool = False
```

| 字段 | 类型 | 默认值 | 描述 |
| :- | :- | :- | :- |
| `behavior` | `Literal["deny"]` | `"deny"` | 必须是 "deny" |
| `message` | `str` | `""` | 解释为什么拒绝工具的消息 |
| `interrupt` | `bool` | `False` | 是否中断当前执行 |

### `PermissionUpdate`

用于以编程方式更新权限的配置。

```python theme={null}
@dataclass
class PermissionUpdate:
    type: Literal[
        "addRules",
        "replaceRules",
        "removeRules",
        "setMode",
        "addDirectories",
        "removeDirectories",
    ]
    rules: list[PermissionRuleValue] | None = None
    behavior: Literal["allow", "deny", "ask"] | None = None
    mode: PermissionMode | None = None
    directories: list[str] | None = None
    destination: (
        Literal["userSettings", "projectSettings", "localSettings", "session"] | None
    ) = None
```

| 字段 | 类型 | 描述 |
| :- | :- | :- |
| `type` | `Literal[...]` | 权限更新操作的类型 |
| `rules` | `list[PermissionRuleValue] \| None` | 用于添加/替换/移除操作的规则 |
| `behavior` | `Literal["allow", "deny", "ask"] \| None` | 基于规则的操作的行为 |
| `mode` | `PermissionMode \| None` | setMode 操作的模式 |
| `directories` | `list[str] \| None` | 用于添加/移除目录操作的目录 |
| `destination` | `Literal[...] \| None` | 应用权限更新的位置 |

### `PermissionRuleValue`

要在权限更新中添加、替换或移除的规则。

```python theme={null}
@dataclass
class PermissionRuleValue:
    tool_name: str
    rule_content: str | None = None
```

### `ToolsPreset`

使用 Claude Code 的默认工具集的预设工具配置。

```python theme={null}
class ToolsPreset(TypedDict):
    type: Literal["preset"]
    preset: Literal["claude_code"]
```

### `ThinkingConfig`

控制扩展思考行为。三种配置的联合：

```python theme={null}
ThinkingDisplay = Literal["summarized", "omitted"]


class ThinkingConfigAdaptive(TypedDict):
    type: Literal["adaptive"]
    display: NotRequired[ThinkingDisplay]


class ThinkingConfigEnabled(TypedDict):
    type: Literal["enabled"]
    budget_tokens: int
    display: NotRequired[ThinkingDisplay]


class ThinkingConfigDisabled(TypedDict):
    type: Literal["disabled"]


ThinkingConfig = ThinkingConfigAdaptive | ThinkingConfigEnabled | ThinkingConfigDisabled
```

| 变体 | 字段 | 描述 |
| :- | :- | :- |
| `adaptive` | `type`, `display` | Claude 自适应决定何时思考 |
| `enabled` | `type`, `budget_tokens`, `display` | 启用具有特定 token 预算的思考 |
| `disabled` | `type` | 禁用思考 |

可选的 `display` 字段控制思考文本以 `"summarized"` 还是 `"omitted"` 形式返回。在 Claude Opus 4.7 及更高版本上，API 默认值为 `"omitted"`，因此请设置 `"summarized"` 以在 [`ThinkingBlock`](#thinkingblock) 输出中接收思考内容。Claude Code 不会将您的 `display` 值传递给某些提供商，例如 Amazon Bedrock 和 Google Cloud 的 Agent Platform。在这些提供商上，即使您将 `display` 设置为 `"summarized"`，Opus 4.7 及更高版本也会返回空的 `ThinkingBlock` 输出。

因为这些是 `TypedDict` 类，它们在运行时是普通字典。可以将它们构造为字典字面量，也可以像调用构造函数一样调用类；两者都产生 `dict`。使用 `config["budget_tokens"]` 访问字段，而不是 `config.budget_tokens`：

```python theme={null}
from claude_agent_sdk import ClaudeAgentOptions, ThinkingConfigEnabled

# Option 1: dict literal (recommended, no import needed)
options = ClaudeAgentOptions(thinking={"type": "enabled", "budget_tokens": 20000})

# Option 2: constructor-style (returns a plain dict)
config = ThinkingConfigEnabled(type="enabled", budget_tokens=20000)
print(config["budget_tokens"])  # 20000
# config.budget_tokens would raise AttributeError
```

### `TaskBudget`

API 端任务预算（以 token 为单位），与 `ClaudeAgentOptions` 中的 `task_budget` 字段一起使用。

```python theme={null}
class TaskBudget(TypedDict):
    total: int
```

| 字段 | 类型 | 描述 |
| :- | :- | :- |
| `total` | `int` | 任务的总 token 预算 |

因为这是 `TypedDict`，请将其作为普通字典传递，如 `ClaudeAgentOptions(task_budget={"total": 50000})`。

### `SdkBeta`

SDK 测试功能的字面类型。

```python theme={null}
SdkBeta = Literal["context-1m-2025-08-07"]
```

与 `ClaudeAgentOptions` 中的 `betas` 字段一起使用以启用测试功能。

> [!WARNING]
>
> 在 Claude API 上，`context-1m-2025-08-07` 测试版已针对 Claude Sonnet 4.5 和 Claude Sonnet 4 停用。如果您在使用其中任一模型时仍传递它，超过标准 200K token 上下文窗口的请求会返回错误，因此请将其从 `betas` 中移除。要运行具有 1M token 上下文窗口的会话，请将 `model` 设置为[默认以 1M 窗口运行](https://code.claude.com/docs/zh-CN/model-config.md#extended-context)的模型，例如 `claude-sonnet-5-5` 或 `claude-opus-5-5`。对于仅通过其 `[1m]` 变体才能达到 1M 的模型，请在模型 ID 后追加该后缀，例如 `claude-opus-4-6[1m]`。

### `McpSdkServerConfig`

使用 `create_sdk_mcp_server()` 创建的 SDK MCP 服务器的配置。

```python theme={null}
class McpSdkServerConfig(TypedDict):
    type: Literal["sdk"]
    name: str
    instance: Any  # MCP Server instance
```

### `McpServerConfig`

MCP 服务器配置的联合类型。

```python theme={null}
McpServerConfig = (
    McpStdioServerConfig | McpSSEServerConfig | McpHttpServerConfig | McpSdkServerConfig
)
```

#### `McpStdioServerConfig`

```python theme={null}
class McpStdioServerConfig(TypedDict):
    type: NotRequired[Literal["stdio"]]  # Optional for backwards compatibility
    command: str
    args: NotRequired[list[str]]
    env: NotRequired[dict[str, str]]
```

#### `McpSSEServerConfig`

```python theme={null}
class McpSSEServerConfig(TypedDict):
    type: Literal["sse"]
    url: str
    headers: NotRequired[dict[str, str]]
```

#### `McpHttpServerConfig`

```python theme={null}
class McpHttpServerConfig(TypedDict):
    type: Literal["http"]
    url: str
    headers: NotRequired[dict[str, str]]
```

### `McpServerStatusConfig`

由 [`get_mcp_status()`](#methods) 报告的 MCP 服务器的配置。这是所有 [`McpServerConfig`](#mcpserverconfig) 传输变体，加上用于通过 claude.ai 代理的服务器的仅输出 `claudeai-proxy` 变体的联合。

```python theme={null}
McpServerStatusConfig = (
    McpStdioServerConfig
    | McpSSEServerConfig
    | McpHttpServerConfig
    | McpSdkServerConfigStatus
    | McpClaudeAIProxyServerConfig
)
```

`McpSdkServerConfigStatus` 是 [`McpSdkServerConfig`](#mcpsdkserverconfig) 的可序列化形式，仅包含 `type`（`"sdk"`）和 `name`（`str`）字段；进程内 `instance` 被省略。`McpClaudeAIProxyServerConfig` 具有 `type`（`"claudeai-proxy"`）、`url`（`str`）和 `id`（`str`）字段。

### `McpStatusResponse`

来自 [`ClaudeSDKClient.get_mcp_status()`](#methods) 的响应。在 `mcpServers` 键下包装服务器状态列表。

```python theme={null}
class McpStatusResponse(TypedDict):
    mcpServers: list[McpServerStatus]
```

### `McpServerStatus`

已连接的 MCP 服务器的状态，包含在 [`McpStatusResponse`](#mcpstatusresponse) 中。

```python theme={null}
class McpServerStatus(TypedDict):
    name: str
    status: McpServerConnectionStatus  # "connected" | "failed" | "needs-auth" | "pending" | "disabled"
    serverInfo: NotRequired[McpServerInfo]
    error: NotRequired[str]
    config: NotRequired[McpServerStatusConfig]
    scope: NotRequired[str]
    tools: NotRequired[list[McpToolInfo]]
```

| 字段 | 类型 | 描述 |
| :- | :- | :- |
| `name` | `str` | 服务器名称 |
| `status` | `str` | `"connected"`、`"failed"`、`"needs-auth"`、`"pending"` 或 `"disabled"` 之一 |
| `serverInfo` | `dict`（可选） | 服务器名称和版本（`{"name": str, "version": str}`） |
| `error` | `str`（可选） | 服务器连接失败时的错误消息 |
| `config` | [`McpServerStatusConfig`](#mcpserverstatusconfig)（可选） | 服务器配置。与 [`McpServerConfig`](#mcpserverconfig) 形状相同（stdio、SSE、HTTP 或 SDK），加上通过 claude.ai 连接的服务器的 `claudeai-proxy` 变体 |
| `scope` | `str`（可选） | 配置作用域 |
| `tools` | `list`（可选） | 此服务器提供的工具，每个都有 `name`、`description` 和 `annotations` 字段 |

### `ContextUsageResponse`

来自 [`ClaudeSDKClient.get_context_usage()`](#methods) 的响应。该响应包含 Claude Code 在交互式会话中为 `/context` 命令呈现的负载。除了 token 计数外，该负载还携带显示字段，如 `color` 和 `gridRows`，Claude Code 使用这些字段来绘制 `/context` 使用网格。

Claude Code 通过向 [token 计数](https://platform.claude.com/docs/en/build-with-claude/token-counting) API 发送多个请求来构建此负载。这些请求不会出现在消息流中，因此读取流的成本跟踪不会看到它们。在 Anthropic API 上，token 计数不计费。

```python theme={null}
class ContextUsageResponse(TypedDict):
    categories: list[ContextUsageCategory]
    totalTokens: int
    maxTokens: int
    rawMaxTokens: int
    percentage: float
    model: str
    isAutoCompactEnabled: bool
    memoryFiles: list[dict[str, Any]]
    mcpTools: list[dict[str, Any]]
    agents: list[dict[str, Any]]
    gridRows: list[list[dict[str, Any]]]
    autoCompactThreshold: NotRequired[int]
    deferredBuiltinTools: NotRequired[list[dict[str, Any]]]
    systemTools: NotRequired[list[dict[str, Any]]]
    systemPromptSections: NotRequired[list[dict[str, Any]]]
    slashCommands: NotRequired[dict[str, Any]]
    skills: NotRequired[dict[str, Any]]  # skill usage with frontmatter breakdown
    messageBreakdown: NotRequired[dict[str, Any]]  # message tokens by type
    apiUsage: NotRequired[dict[str, Any] | None]
```

每个 `ContextUsageCategory` 条目携带 `name`、`tokens`、`color` 和可选的 `isDeferred` 标志。`totalTokens` 是会话当前的上下文使用量，`maxTokens` 是衡量该使用量所依据的窗口。该窗口是模型的上下文窗口，或在适用时较低的自动压缩窗口，`rawMaxTokens` 携带与 `maxTokens` 相同的值。`apiUsage` 保存最新 API 响应的使用情况，而不是会话的累计总数。Claude Code 不会设置可选的 `deferredBuiltinTools`、`systemTools` 和 `systemPromptSections` 键，因此即使类型声明了它们，也应预期它们不存在。

### `SdkPluginConfig`

SDK 中加载插件的配置。

```python theme={null}
class SdkPluginConfig(TypedDict):
    type: Literal["local"]
    path: str
```

| 字段 | 类型 | 描述 |
| :- | :- | :- |
| `type` | `Literal["local"]` | 必须是 `"local"`（目前仅支持本地插件） |
| `path` | `str` | 插件目录的绝对或相对路径 |

**示例：**

```python theme={null}
plugins = [
    {"type": "local", "path": "./my-plugin"},
    {"type": "local", "path": "/absolute/path/to/plugin"},
]
```

有关创建和使用插件的完整信息，见 [插件](https://code.claude.com/docs/zh-CN/agent-sdk/plugins.md)。

## 消息类型

### `Message`

所有可能消息的联合类型。

```python theme={null}
Message = (
    UserMessage
    | AssistantMessage
    | SystemMessage
    | ResultMessage
    | StreamEvent
    | RateLimitEvent
    | ConversationResetMessage
)
```

### `UserMessage`

用户输入消息。

```python theme={null}
@dataclass
class UserMessage:
    content: str | list[ContentBlock]
    uuid: str | None = None
    parent_tool_use_id: str | None = None
    tool_use_result: dict[str, Any] | None = None
    origin: MessageOrigin | None = None
```

| 字段 | 类型 | 描述 |
| :- | :- | :- |
| `content` | `str \| list[ContentBlock]` | 消息内容为文本或内容块 |
| `uuid` | `str \| None` | 唯一消息标识符 |
| `parent_tool_use_id` | `str \| None` | 如果此消息是工具结果响应，则为工具使用 ID |
| `tool_use_result` | `dict[str, Any] \| None` | 工具结果数据（如果适用） |
| `origin` | `MessageOrigin \| None` | 此消息的来源，在注入的轮次（如任务通知和对等消息）上填充。当 CLI 未归属时为 `None`。需要 Python Agent SDK 0.2.137 或更高版本 |

SDK 从 CLI 未修改地传递 `tool_use_result`。对于外部 MCP 服务器上的工具，其结果包含 `resource_link` 块，该字典具有 `resourceLinks` 键，保存具有 TypeScript [`SDKMcpResourceLink`](https://code.claude.com/docs/zh-CN/agent-sdk/typescript#sdkmcpresourcelink) 类型键的字典列表。Claude 将每个链接作为工具结果中的一行文本接收。要呈现服务器返回的文件，请读取 `resourceLinks` 而不是解析该文本。`resourceLinks` 键需要 Python Agent SDK 0.2.150 或更高版本和 Claude Code v2.1.257 或更高版本；该 SDK 版本附带的 CLI 满足 Claude Code 要求。

当结果没有链接时，CLI 会省略该键，在来自子代理的结果上也会省略。CLI 每个结果最多保留 50 个链接，一旦列表达到 64 KiB 的序列化 JSON，就停止添加链接。使用 [`tool()`](#tool) 在进程中定义的工具永远不会产生该键，因为 SDK 在 CLI 看到结果之前将其 `resource_link` 块展平为文本。

### `AssistantMessage`

带有内容块的助手响应消息。

```python theme={null}
@dataclass
class AssistantMessage:
    content: list[ContentBlock]
    model: str
    parent_tool_use_id: str | None = None
    error: AssistantMessageError | None = None
    usage: dict[str, Any] | None = None
    message_id: str | None = None
    stop_reason: str | None = None
    session_id: str | None = None
    uuid: str | None = None
```

| 字段 | 类型 | 描述 |
| :- | :- | :- |
| `content` | `list[ContentBlock]` | 响应中的内容块列表 |
| `model` | `str` | 生成响应的模型 |
| `parent_tool_use_id` | `str \| None` | 如果这是嵌套响应，则为工具使用 ID |
| `error` | [`AssistantMessageError`](#assistantmessageerror) ` \| None` | 如果响应遇到错误，则为错误类型 |
| `usage` | `dict[str, Any] \| None` | 每条消息的 token 使用情况（与 [`ResultMessage.usage`](#resultmessage) 相同的键） |
| `message_id` | `str \| None` | API 消息 ID。来自一个轮次的多条消息共享相同的 ID |
| `stop_reason` | `str \| None` | 来自 API 的停止原因（例如 `end_turn`、`tool_use`） |
| `session_id` | `str \| None` | 此消息所属的会话 ID |
| `uuid` | `str \| None` | 会话记录中的唯一消息标识符 |

### `AssistantMessageError`

助手消息的可能错误类型。

```python theme={null}
AssistantMessageError = Literal[
    "authentication_failed",
    "billing_error",
    "rate_limit",
    "invalid_request",
    "server_error",
    "unknown",
]
```

底层 CLI 进程可以发出此 Literal 未列出的错误类型，例如 `max_output_tokens`。SDK 未修改地传递该值，因此将此列表之外的字符串视为处理 `unknown` 的方式。TypeScript [`SDKAssistantMessageError`](https://code.claude.com/docs/zh-CN/agent-sdk/typescript#sdkassistantmessage) 类型列出了 CLI 可以发出的完整值集。

### `SystemMessage`

带有元数据的系统消息。

```python theme={null}
@dataclass
class SystemMessage:
    subtype: str
    data: dict[str, Any]
```

没有自己数据类的子类型以 `SystemMessage` 的形式到达。要在轮次之间跟踪会话，请设置 [`CLAUDE_CODE_EMIT_SESSION_STATE_EVENTS=1`](https://code.claude.com/docs/zh-CN/env-vars#variables)，并在每条 `subtype` 为 `session_state_changed` 的消息上读取 `message.data["state"]`。[`SDKSessionStateChangedMessage`](https://code.claude.com/docs/zh-CN/agent-sdk/typescript#sdksessionstatechangedmessage) 列出了它可以携带的状态。使用 `receive_messages()` 迭代以读取这些消息：`receive_response()` 会在 `ResultMessage` 处停止，而 `session_state_changed` 消息可能出现在该结果之后。

### `ResultMessage`

带有成本和使用信息的最终结果消息。

```python theme={null}
@dataclass
class ResultMessage:
    subtype: str
    duration_ms: int
    duration_api_ms: int
    is_error: bool
    num_turns: int
    session_id: str
    stop_reason: str | None = None
    total_cost_usd: float | None = None
    usage: dict[str, Any] | None = None
    result: str | None = None
    structured_output: Any = None
    model_usage: dict[str, ModelUsage] | None = None
    permission_denials: list[Any] | None = None
    deferred_tool_use: DeferredToolUse | None = None
    errors: list[str] | None = None
    api_error_status: int | None = None
    uuid: str | None = None
    terminal_reason: str | None = None
    origin: MessageOrigin | None = None
```

`subtype` 字段确定填充哪些其他字段。它是 `"success"`、`"error_during_execution"`、`"error_max_turns"`、`"error_max_budget_usd"` 或 `"error_max_structured_output_retries"` 之一。Python 数据类将所有变体展平为一种形状，因此不适用于返回的子类型的字段为 `None`。

多个字段携带有关对话如何结束的诊断详情：

* `is_error`：当对话以错误状态结束时为 `True`。在 `error_*` 子类型上始终为 `True`。在 `subtype="success"` 上，有三种情况为 `True`。请检查 `stop_reason` 以区分它们：
  * `"refusal"`：模型拒绝回复。请在 `result` 中读取拒绝文本；当 `result` 为空时，在前面的 `AssistantMessage` 中读取。
  * `"tool_deferred_unavailable"`：恢复会话时，[延迟的工具](https://code.claude.com/docs/zh-CN/hooks.md#defer-a-tool-call-for-later)已不再可用，例如因为其 MCP 服务器未连接。在以下两种情况中，恢复后发送的提示词与随恢复一起传递的提示词效果相同：
    * 无提示词：该调用仍处于挂起状态。使该工具可用后，再次不带提示词地恢复，该调用会首先运行。
    * 有提示词：Claude Code 会在没有该工具结果的情况下回答它，延迟的调用将不复存在。如果仍需要调用该工具，请发送新的提示词。
  * 任何其他值：轮次因 Claude Code 报告的错误（代替模型回复）而结束，例如 API 错误。读取 `terminal_reason` 了解原因，读取 `result` 获取消息，当 API 返回 HTTP 状态时读取 `api_error_status` 获取该状态。
* `api_error_status`：终止 API 错误的 HTTP 状态代码。当轮次结束时没有错误时为 `None`。仅在 `subtype="success"` 上填充。
* `result`：在 `subtype="success"` 上为最终助手消息的文本，或在 `error_*` 子类型上为 `None`。当 `subtype="success"` 且 `is_error=True` 时，如果可用，此字段保存 API 错误字符串，但可能为空，因此请检查 `api_error_status` 和前面的 `AssistantMessage` 内容以获取详情。
* `errors`：循环级别的错误字符串，例如最大轮次消息。仅在 `error_*` 子类型上填充。
* `terminal_reason`：查询循环结束的原因，例如 `"completed"`、`"max_turns"`、`"api_error"`、`"aborted_streaming"` 或 `"aborted_tools"`。值为 `"aborted_streaming"` 或 `"aborted_tools"` 意味着轮次在完成前被中止。常见原因是 [`interrupt()`](#claudesdkclient) 和权限回调返回 [`PermissionResultDeny`](#permissionresultdeny) 且 `interrupt=True`。在早于该字段的 CLI 版本上为 `None`，在本地命令（如 `/voice` 或 `/usage`）的结果上为 `None`，这些命令绕过查询循环，或在会话严重失败时发出的合成错误结果上为 `None`。镜像 TypeScript SDK 的 [`SDKResultMessage.terminal_reason`](https://code.claude.com/docs/zh-CN/agent-sdk/typescript#sdkresultmessage)，其列出了完整的值集。
* `origin`：触发此轮次的用户消息的来源。在[流式输入模式](https://code.claude.com/docs/zh-CN/agent-sdk/streaming-vs-single-mode.md)中，检查此项以区分您自己的提示词结果（其中 `origin` 为 `None` 或 `{"kind": "human"}`）与注入的轮次（如后台任务通知）的结果。需要 Python Agent SDK 0.2.137 或更高版本。

当多个后台任务在相近时间内完成时，Claude Code 可以在一个轮次中回复它们的通知，而不是为每个通知各用一个轮次。您仍会按顺序为每个通知收到一个 `ResultMessage`，每个都带有 `kind` 为 `"task-notification"` 的 `origin`。除最后一个外，其余消息的 `num_turns` 均为 `0` 且 `result` 为空，最后一个则承载回复所有通知的那个轮次。

`usage` 字典仅涵盖主 Agent 循环，不包括子代理和其他嵌套或辅助模型调用。在[流式输入模式](https://code.claude.com/docs/zh-CN/agent-sdk/streaming-vs-single-mode.md)中，值是按轮次的。优先使用 `model_usage` 进行 token 和成本核算。`usage` 字典在存在时包含以下键：

| 键 | 类型 | 描述 |
| - | - | - |
| `input_tokens` | `int` | 顶级 Agent 循环消耗的输入 token。[子代理 token 不包括在内](https://code.claude.com/docs/zh-CN/agent-sdk/cost-tracking.md#get-the-total-cost-of-a-query)；使用 `model_usage` 进行整树核算。 |
| `output_tokens` | `int` | 顶级 Agent 循环生成的输出 token。子代理 token 不包括在内。 |
| `cache_creation_input_tokens` | `int` | 用于创建新缓存条目的 token。 |
| `cache_read_input_tokens` | `int` | 从现有缓存条目读取的 token。 |

`model_usage` 字典将模型名称映射到每个模型的使用情况。它涵盖通过查询管道进行的每个模型调用：主循环、子代理和内部调用（如压缩和 Workflow Agent）。该管道外的辅助调用（如权限分类器和 token 计数请求）从 `model_usage` 中排除。将 `model_usage` 视为估计值，而不是计费声明。

在[流式输入模式](https://code.claude.com/docs/zh-CN/agent-sdk/streaming-vs-single-mode.md)中，`model_usage` 和 `total_cost_usd` 在轮次间是累积的，因此读取最新结果而不是跨结果求和。调用恢复会话时，也会计算[从会话早期调用恢复的总计](https://code.claude.com/docs/zh-CN/agent-sdk/cost-tracking.md#accumulate-costs-across-multiple-calls)。有关重置，请参阅[在流式输入模式中跟踪成本](https://code.claude.com/docs/zh-CN/agent-sdk/cost-tracking.md#track-costs-in-streaming-input-mode)，有关清零结果，请参阅[在会话崩溃后恢复总计](https://code.claude.com/docs/zh-CN/agent-sdk/cost-tracking.md#recover-totals-after-a-session-crash)。

`model_usage` 中的每个值都是 `ModelUsage` TypedDict，通过 `from claude_agent_sdk.types import ModelUsage` 导入。其键使用 camelCase，因为 SDK 从底层 CLI 进程未修改地传递该值，匹配 TypeScript [`ModelUsage`](https://code.claude.com/docs/zh-CN/agent-sdk/typescript#modelusage) 类型：

| 键 | 类型 | 描述 |
| - | - | - |
| `inputTokens` | `int` | 此模型的输入 token。 |
| `outputTokens` | `int` | 此模型的输出 token。 |
| `cacheReadInputTokens` | `int` | 此模型的缓存读取 token。 |
| `cacheCreationInputTokens` | `int` | 此模型的缓存创建 token。 |
| `webSearchRequests` | `int` | 此模型进行的网络搜索请求。 |
| `thinkingTokens` | `int` | 此模型生成的思考 token，已计入 `outputTokens`。在轮次在记录它的 Claude Code 版本上运行之前不存在，并且未在 TypedDict 上声明，因此使用 `.get()` 读取它。需要 Python Agent SDK 0.2.150 或更高版本，其附带的 CLI 记录它。 |
| `costUSD` | `float` | 此模型的估计成本（美元），客户端计算。见[跟踪成本和使用](https://code.claude.com/docs/zh-CN/agent-sdk/cost-tracking.md)了解计费注意事项。 |
| `contextWindow` | `int` | 此模型的上下文窗口大小。 |
| `maxOutputTokens` | `int` | 此模型的最大输出 token 限制。 |
| `canonicalModel` | `str` | 用于定价查询的规范模型 ID。可能与用作条目键的原始模型字符串不同，例如特定于提供商的 ID 或别名。并非总是存在。 |
| `provider` | `str` | 提供此模型的 API 提供商，例如 `firstParty`、`bedrock`、`vertex`、`foundry`、`anthropicAws`、`mantle` 或 `gateway`。并非总是存在。 |
| `costBasis` | `str` | 为此模型最新请求定价的价格表：`list` 表示标价，`managed` 表示 [`modelPricing`](https://code.claude.com/docs/zh-CN/settings-reference#modelpricing) 表，`unknown` 表示两者都与模型 ID 不匹配。并非总是存在，并且未在 TypedDict 上声明，因此使用 `.get()` 读取它。需要 Claude Code v2.1.246 或更高版本。 |

### `StreamEvent`

流式事件，用于流式传输期间的部分消息更新。仅在 `ClaudeAgentOptions` 中 `include_partial_messages=True` 时接收。通过 `from claude_agent_sdk.types import StreamEvent` 导入。

```python theme={null}
@dataclass
class StreamEvent:
    uuid: str
    session_id: str
    event: dict[str, Any]  # The raw Claude API stream event
    parent_tool_use_id: str | None = None
```

| 字段 | 类型 | 描述 |
| :- | :- | :- |
| `uuid` | `str` | 此事件的唯一标识符 |
| `session_id` | `str` | 会话标识符 |
| `event` | `dict[str, Any]` | 原始 Claude API 流事件数据 |
| `parent_tool_use_id` | `str \| None` | 始终为 `None`。流事件仅为主会话发出。对于子代理归属，请使用完整消息，例如 [`AssistantMessage`](#assistantmessage) |

### `RateLimitEvent`

当速率限制状态更改时发出（例如，从 `"allowed"` 到 `"allowed_warning"`）。使用此来在用户达到硬限制之前警告他们，或在状态为 `"rejected"` 时退避。

```python theme={null}
@dataclass
class RateLimitEvent:
    rate_limit_info: RateLimitInfo
    uuid: str
    session_id: str
```

| 字段 | 类型 | 描述 |
| :- | :- | :- |
| `rate_limit_info` | [`RateLimitInfo`](#ratelimitinfo) | 当前速率限制状态 |
| `uuid` | `str` | 唯一事件标识符 |
| `session_id` | `str` | 会话标识符 |

### `RateLimitInfo`

由 [`RateLimitEvent`](#ratelimitevent) 携带的速率限制状态。

```python theme={null}
RateLimitStatus = Literal["allowed", "allowed_warning", "rejected"]
RateLimitType = Literal[
    "five_hour", "seven_day", "seven_day_opus", "seven_day_sonnet", "overage"
]


@dataclass
class RateLimitInfo:
    status: RateLimitStatus
    resets_at: int | None = None
    rate_limit_type: RateLimitType | None = None
    utilization: float | None = None
    overage_status: RateLimitStatus | None = None
    overage_resets_at: int | None = None
    overage_disabled_reason: str | None = None
    raw: dict[str, Any] = field(default_factory=dict)
```

| 字段 | 类型 | 描述 |
| :- | :- | :- |
| `status` | `RateLimitStatus` | 当前状态，`"allowed"`、`"allowed_warning"` 或 `"rejected"` 之一。`"allowed_warning"` 表示接近限制；`"rejected"` 表示达到限制 |
| `resets_at` | `int \| None` | 速率限制窗口重置的 Unix 时间戳 |
| `rate_limit_type` | `RateLimitType \| None` | 哪个速率限制窗口适用 |
| `utilization` | `float \| None` | 消耗的速率限制的分数（0.0 到 1.0） |
| `overage_status` | `RateLimitStatus \| None` | 按需付费超额使用的状态（如果适用） |
| `overage_resets_at` | `int \| None` | 超额窗口重置的 Unix 时间戳 |
| `overage_disabled_reason` | `str \| None` | 为什么超额不可用，如果状态为 `"rejected"` |
| `raw` | `dict[str, Any]` | 来自 CLI 的完整原始字典，包括上面未建模的字段 |

### `ConversationResetMessage`

在不结束连接的情况下替换对话时发出，例如在 `/clear` 之后。有关重置如何影响后续 `ResultMessage` 对象上的运行总计，请参阅[在流式输入模式中跟踪成本](https://code.claude.com/docs/zh-CN/agent-sdk/cost-tracking.md#track-costs-in-streaming-input-mode)。需要 Python Agent SDK 0.2.137 或更高版本。

```python theme={null}
@dataclass
class ConversationResetMessage:
    new_conversation_id: str
    uuid: str
    session_id: str
```

| 字段 | 类型 | 描述 |
| :- | :- | :- |
| `new_conversation_id` | `str` | 新对话的不透明标识符。不是后续消息的 `session_id`；从下一条消息读取 |
| `uuid` | `str` | 唯一消息标识符 |
| `session_id` | `str` | 被重置的会话的 ID。重置后的消息携带新的 `session_id` |

### `TaskStartedMessage`

当后台任务启动时发出。后台任务是在主轮次之外跟踪的任何内容：后台 Bash 命令、[Monitor](#monitor) 监视、通过 Agent 工具生成的子代理或远程 Agent。`task_type` 字段告诉您是哪一个。此命名与 `Task` 到 `Agent` 工具重命名无关。

```python theme={null}
@dataclass
class TaskStartedMessage(SystemMessage):
    task_id: str
    description: str
    uuid: str
    session_id: str
    tool_use_id: str | None = None
    task_type: str | None = None
```

| 字段 | 类型 | 描述 |
| :- | :- | :- |
| `task_id` | `str` | 任务的唯一标识符 |
| `description` | `str` | 任务的描述 |
| `uuid` | `str` | 唯一消息标识符 |
| `session_id` | `str` | 会话标识符 |
| `tool_use_id` | `str \| None` | 关联的工具使用 ID |
| `task_type` | `str \| None` | 哪种后台任务：`"local_bash"` 用于后台 Bash 和 Monitor 监视，`"local_agent"` 或 `"remote_agent"` |

### `TaskUsage`

后台任务的 token 和计时数据。

```python theme={null}
class TaskUsage(TypedDict):
    total_tokens: int
    tool_uses: int
    duration_ms: int
```

### `TaskProgressMessage`

定期为运行的后台任务发出进度更新。

```python theme={null}
@dataclass
class TaskProgressMessage(SystemMessage):
    task_id: str
    description: str
    usage: TaskUsage
    uuid: str
    session_id: str
    tool_use_id: str | None = None
    last_tool_name: str | None = None
```

| 字段 | 类型 | 描述 |
| :- | :- | :- |
| `task_id` | `str` | 任务的唯一标识符 |
| `description` | `str` | 当前状态描述 |
| `usage` | `TaskUsage` | 此任务迄今为止的 token 使用情况 |
| `uuid` | `str` | 唯一消息标识符 |
| `session_id` | `str` | 会话标识符 |
| `tool_use_id` | `str \| None` | 关联的工具使用 ID |
| `last_tool_name` | `str \| None` | 任务使用的最后一个工具的名称 |

### `TaskNotificationMessage`

当后台任务完成、失败或停止时发出。后台任务包括 `run_in_background` Bash 命令、Monitor 监视和后台子代理。

```python theme={null}
@dataclass
class TaskNotificationMessage(SystemMessage):
    task_id: str
    status: TaskNotificationStatus  # "completed" | "failed" | "stopped"
    output_file: str
    summary: str
    uuid: str
    session_id: str
    tool_use_id: str | None = None
    usage: TaskUsage | None = None
```

| 字段 | 类型 | 描述 |
| :- | :- | :- |
| `task_id` | `str` | 任务的唯一标识符 |
| `status` | `TaskNotificationStatus` | `"completed"`、`"failed"` 或 `"stopped"` 之一 |
| `output_file` | `str` | 任务输出文件的路径 |
| `summary` | `str` | 任务结果的摘要 |
| `uuid` | `str` | 唯一消息标识符 |
| `session_id` | `str` | 会话标识符 |
| `tool_use_id` | `str \| None` | 关联的工具使用 ID |
| `usage` | `TaskUsage \| None` | 任务的最终 token 使用情况 |

当 CLI [将长 MCP 工具调用移到后台](https://code.claude.com/docs/zh-CN/mcp.md#automatic-backgrounding-of-long-tool-calls)时，该调用的工具结果仅保存占位符，该调用的真实结果在此消息中到达。在此类调用的 `"completed"` 通知上，CLI 添加 `resource_links` 键，列出工具通过引用返回的文件，具有与 [`UserMessage.tool_use_result`](#usermessage) 上的 `resourceLinks` 键相同的条目和限制。`resource_links` 键需要 Python Agent SDK 0.2.150 或更高版本和 Claude Code v2.1.257 或更高版本；该 SDK 版本附带的 CLI 满足 Claude Code 要求。

数据类没有 `resource_links` 字段。从消息继承自 [`SystemMessage`](#systemmessage) 的 `data` 字典读取它：`message.data.get("resource_links")`。使用 `tool_use_id` 匹配通知与调用。当结果没有链接时，CLI 会省略该键，在不是 MCP 工具调用的任务的通知上也会省略。

## 内容块类型

### `ContentBlock`

所有内容块的联合类型。

```python theme={null}
ContentBlock = (
    TextBlock
    | ThinkingBlock
    | ToolUseBlock
    | ToolResultBlock
    | ServerToolUseBlock
    | ServerToolResultBlock
)
```

### `TextBlock`

文本内容块。

```python theme={null}
@dataclass
class TextBlock:
    text: str
```

### `ThinkingBlock`

思考内容块（用于具有思考能力的模型）。

```python theme={null}
@dataclass
class ThinkingBlock:
    thinking: str
    signature: str
```

### `ToolUseBlock`

工具使用请求块。

```python theme={null}
@dataclass
class ToolUseBlock:
    id: str
    name: str
    input: dict[str, Any]
```

### `ToolResultBlock`

工具执行结果块。

```python theme={null}
@dataclass
class ToolResultBlock:
    tool_use_id: str
    content: str | list[dict[str, Any]] | None = None
    is_error: bool | None = None
```

## 错误类型

下面的类型定义了您的代码可以捕获的内容。对于与这些类型引发的错误消息相关的条目，包括每个错误的原因和修复方法，请参阅[故障排除](https://code.claude.com/docs/zh-CN/agent-sdk/troubleshooting.md)。

### `ClaudeSDKError`

所有 SDK 错误的基础异常类。

```python theme={null}
class ClaudeSDKError(Exception):
    """Base error for Claude SDK."""
```

当单次 `query()` 以错误结果结束时，例如达到轮次限制错误，SDK 会引发 [`ResultError`](#resulterror)。

### `CLINotFoundError`

当 Claude Code CLI 未安装或找不到时引发。

```python theme={null}
class CLINotFoundError(CLIConnectionError):
    def __init__(
        self, message: str = "Claude Code not found", cli_path: str | None = None
    ):
        """
        Args:
            message: Error message (default: "Claude Code not found")
            cli_path: Optional path to the CLI that was not found
        """
```

### `CLIConnectionError`

当连接到 Claude Code 失败时引发。

```python theme={null}
class CLIConnectionError(ClaudeSDKError):
    """Failed to connect to Claude Code."""
```

### `ProcessError`

当 Claude Code 进程失败时引发。

```python theme={null}
class ProcessError(ClaudeSDKError):
    def __init__(
        self, message: str, exit_code: int | None = None, stderr: str | None = None
    ):
        self.exit_code = exit_code
        self.stderr = stderr
```

### `ResultError`

当 Claude Code 进程因运行以错误[结果消息](#resultmessage)结束而退出时引发，例如达到轮次限制错误或 API 错误。`ResultError` 是 `ProcessError` 的子类，因此现有的 `except ProcessError` 处理程序也会捕获它。其属性包含该结果消息的字段，因此您可以根据运行失败的原因进行分支，而无需解析消息文本。需要 Python Agent SDK 0.2.140 或更高版本。

```python theme={null}
class ResultError(ProcessError):
    subtype: str | None  # "error_max_turns", "error_during_execution", ...; "success" when the run ended on a failed request
    errors: list[str]  # an empty list when the result message reported none
    result: str | None
    api_error_status: int | None
    terminal_reason: str | None  # "max_turns", "api_error", ...; check after data["stop_reason"], before subtype
    session_id: str | None
    data: dict[str, Any]  # the raw result message payload
```

要区分失败，请先检查 `data["stop_reason"]`，然后检查 `terminal_reason`，最后检查 `subtype`。拒绝回答的回复的 `stop_reason` 为 `"refusal"`，`subtype` 通常为 `"success"`；其 `terminal_reason` 无法单独识别这种情况，可能为 `"completed"` 或 `"api_error"`。失败的最终请求的 `subtype` 为 `"success"`，原因在 `terminal_reason` 中，例如 API 错误对应 `"api_error"`。当运行达到您设置的限制（例如 `max_turns` 或 `max_budget_usd`）时，您会得到 `error_*` 子类型。

### `CLIJSONDecodeError`

当 JSON 解析失败时引发。

```python theme={null}
class CLIJSONDecodeError(ClaudeSDKError):
    def __init__(self, line: str, original_error: Exception):
        """
        Args:
            line: The line that failed to parse
            original_error: The original JSON decode exception
        """
        self.line = line
        self.original_error = original_error
```

## Hook 类型

有关使用 hook 的综合指南，包括示例和常见模式，见 [Hooks 指南](https://code.claude.com/docs/zh-CN/agent-sdk/hooks.md)。

### `HookEvent`

支持的 hook 事件类型。

```python theme={null}
HookEvent = Literal[
    "PreToolUse",  # Called before tool execution
    "PostToolUse",  # Called after tool execution
    "PostToolUseFailure",  # Called when a tool execution fails
    "UserPromptSubmit",  # Called when a prompt is submitted
    "Stop",  # Called when stopping execution
    "SubagentStop",  # Called when a subagent stops
    "PreCompact",  # Called before message compaction
    "Notification",  # Called for notification events
    "SubagentStart",  # Called when a subagent starts
    "PermissionRequest",  # Called when a permission decision is needed
]
```

> [!NOTE]
>
> TypeScript SDK 支持 Python 中尚未提供的其他 hook 事件。见 [hook 可用性表](https://code.claude.com/docs/zh-CN/agent-sdk/hooks.md#available-hooks) 了解每个 SDK 的支持情况。

### `HookCallback`

hook 回调函数的类型定义。

```python theme={null}
HookCallback = Callable[[HookInput, str | None, HookContext], Awaitable[HookJSONOutput]]
```

参数：

* `input`：强类型 hook 输入，具有基于 `hook_event_name` 的判别联合（见 [`HookInput`](#hookinput)）
* `tool_use_id`：可选工具使用标识符（用于工具相关的 hook）
* `context`：带有附加信息的 hook 上下文

返回 [`HookJSONOutput`](#hookjsonoutput)。

### `HookContext`

传递给 hook 回调的上下文信息。

```python theme={null}
class HookContext(TypedDict):
    signal: Any | None  # Future: abort signal support
```

### `HookMatcher`

用于将 hook 匹配到特定事件或工具的配置。

```python theme={null}
@dataclass
class HookMatcher:
    matcher: str | None = (
        None  # Tool name or pattern to match (e.g., "Bash", "Write|Edit")
    )
    hooks: list[HookCallback] = field(
        default_factory=list
    )  # List of callbacks to execute
    timeout: float | None = (
        None  # Timeout in seconds. When omitted, the per-event default applies:
        # 600 for most events, 30 for UserPromptSubmit
    )
```

### `HookInput`

所有 hook 输入类型的联合类型。实际类型取决于 `hook_event_name` 字段。

```python theme={null}
HookInput = (
    PreToolUseHookInput
    | PostToolUseHookInput
    | PostToolUseFailureHookInput
    | UserPromptSubmitHookInput
    | StopHookInput
    | SubagentStopHookInput
    | PreCompactHookInput
    | NotificationHookInput
    | SubagentStartHookInput
    | PermissionRequestHookInput
)
```

### `BaseHookInput`

所有 hook 输入类型中存在的基础字段。

```python theme={null}
class BaseHookInput(TypedDict):
    session_id: str
    transcript_path: str
    cwd: str
    permission_mode: NotRequired[str]
```

| 字段 | 类型 | 描述 |
| :- | :- | :- |
| `session_id` | `str` | 当前会话标识符 |
| `transcript_path` | `str` | 会话记录文件的路径 |
| `cwd` | `str` | 当前工作目录 |
| `permission_mode` | `str`（可选） | 当前权限模式 |

### `PreToolUseHookInput`

`PreToolUse` hook 事件的输入数据。

```python theme={null}
class PreToolUseHookInput(BaseHookInput):
    hook_event_name: Literal["PreToolUse"]
    tool_name: str
    tool_input: dict[str, Any]
    tool_use_id: str
    agent_id: NotRequired[str]
    agent_type: NotRequired[str]
```

| 字段 | 类型 | 描述 |
| :- | :- | :- |
| `hook_event_name` | `Literal["PreToolUse"]` | 始终为 "PreToolUse" |
| `tool_name` | `str` | 即将执行的工具的名称 |
| `tool_input` | `dict[str, Any]` | 工具的输入参数 |
| `tool_use_id` | `str` | 此工具使用的唯一标识符 |
| `agent_id` | `str`（可选） | 子代理标识符，当 hook 在子代理内触发时存在 |
| `agent_type` | `str`（可选） | 子代理类型，当 hook 在子代理内触发时存在 |

### `PostToolUseHookInput`

`PostToolUse` hook 事件的输入数据。

```python theme={null}
class PostToolUseHookInput(BaseHookInput):
    hook_event_name: Literal["PostToolUse"]
    tool_name: str
    tool_input: dict[str, Any]
    tool_response: Any
    tool_use_id: str
    agent_id: NotRequired[str]
    agent_type: NotRequired[str]
```

| 字段 | 类型 | 描述 |
| :- | :- | :- |
| `hook_event_name` | `Literal["PostToolUse"]` | 始终为 "PostToolUse" |
| `tool_name` | `str` | 已执行的工具的名称 |
| `tool_input` | `dict[str, Any]` | 使用的输入参数 |
| `tool_response` | `Any` | 工具执行的响应 |
| `tool_use_id` | `str` | 此工具使用的唯一标识符 |
| `agent_id` | `str`（可选） | 子代理标识符，当 hook 在子代理内触发时存在 |
| `agent_type` | `str`（可选） | 子代理类型，当 hook 在子代理内触发时存在 |

### `PostToolUseFailureHookInput`

`PostToolUseFailure` hook 事件的输入数据。当工具执行失败时调用。

```python theme={null}
class PostToolUseFailureHookInput(BaseHookInput):
    hook_event_name: Literal["PostToolUseFailure"]
    tool_name: str
    tool_input: dict[str, Any]
    tool_use_id: str
    error: str
    is_interrupt: NotRequired[bool]
    agent_id: NotRequired[str]
    agent_type: NotRequired[str]
```

| 字段 | 类型 | 描述 |
| :- | :- | :- |
| `hook_event_name` | `Literal["PostToolUseFailure"]` | 始终为 "PostToolUseFailure" |
| `tool_name` | `str` | 失败的工具的名称 |
| `tool_input` | `dict[str, Any]` | 使用的输入参数 |
| `tool_use_id` | `str` | 此工具使用的唯一标识符 |
| `error` | `str` | 失败执行的错误消息 |
| `is_interrupt` | `bool`（可选） | 当失败作为中止而不是工具报告的错误到达 Claude Code 时为真。使用 `interrupt()` 取消正在运行的工具不会触发此 hook；工具结果改为包含中断消息 |
| `agent_id` | `str`（可选） | 子代理标识符，当 hook 在子代理内触发时存在 |
| `agent_type` | `str`（可选） | 子代理类型，当 hook 在子代理内触发时存在 |

### `UserPromptSubmitHookInput`

`UserPromptSubmit` hook 事件的输入数据。

```python theme={null}
class UserPromptSubmitHookInput(BaseHookInput):
    hook_event_name: Literal["UserPromptSubmit"]
    prompt: str
```

| 字段 | 类型 | 描述 |
| :- | :- | :- |
| `hook_event_name` | `Literal["UserPromptSubmit"]` | 始终为 "UserPromptSubmit" |
| `prompt` | `str` | 提交的提示词 |

### `StopHookInput`

`Stop` hook 事件的输入数据。

```python theme={null}
class StopHookInput(BaseHookInput):
    hook_event_name: Literal["Stop"]
    stop_hook_active: bool
```

| 字段 | 类型 | 描述 |
| :- | :- | :- |
| `hook_event_name` | `Literal["Stop"]` | 始终为 "Stop" |
| `stop_hook_active` | `bool` | stop hook 是否活跃 |

### `SubagentStopHookInput`

`SubagentStop` hook 事件的输入数据。

```python theme={null}
class SubagentStopHookInput(BaseHookInput):
    hook_event_name: Literal["SubagentStop"]
    stop_hook_active: bool
    agent_id: str
    agent_transcript_path: str
    agent_type: str
```

| 字段 | 类型 | 描述 |
| :- | :- | :- |
| `hook_event_name` | `Literal["SubagentStop"]` | 始终为 "SubagentStop" |
| `stop_hook_active` | `bool` | stop hook 是否活跃 |
| `agent_id` | `str` | 子代理的唯一标识符 |
| `agent_transcript_path` | `str` | 子代理的会话记录文件路径 |
| `agent_type` | `str` | 子代理的类型 |

### `PreCompactHookInput`

`PreCompact` hook 事件的输入数据。

```python theme={null}
class PreCompactHookInput(BaseHookInput):
    hook_event_name: Literal["PreCompact"]
    trigger: Literal["manual", "auto"]
    custom_instructions: str | None
```

| 字段 | 类型 | 描述 |
| :- | :- | :- |
| `hook_event_name` | `Literal["PreCompact"]` | 始终为 "PreCompact" |
| `trigger` | `Literal["manual", "auto"]` | 什么触发了压缩 |
| `custom_instructions` | `str \| None` | 压缩的自定义说明 |

### `NotificationHookInput`

`Notification` hook 事件的输入数据。

```python theme={null}
class NotificationHookInput(BaseHookInput):
    hook_event_name: Literal["Notification"]
    message: str
    title: NotRequired[str]
    notification_type: str
```

| 字段 | 类型 | 描述 |
| :- | :- | :- |
| `hook_event_name` | `Literal["Notification"]` | 始终为 "Notification" |
| `message` | `str` | 通知消息内容 |
| `title` | `str`（可选） | 通知标题 |
| `notification_type` | `str` | 通知类型 |

### `SubagentStartHookInput`

`SubagentStart` hook 事件的输入数据。

```python theme={null}
class SubagentStartHookInput(BaseHookInput):
    hook_event_name: Literal["SubagentStart"]
    agent_id: str
    agent_type: str
```

| 字段 | 类型 | 描述 |
| :- | :- | :- |
| `hook_event_name` | `Literal["SubagentStart"]` | 始终为 "SubagentStart" |
| `agent_id` | `str` | 子代理的唯一标识符 |
| `agent_type` | `str` | 子代理的类型 |

### `PermissionRequestHookInput`

`PermissionRequest` hook 事件的输入数据。允许 hook 以编程方式处理权限决策。

```python theme={null}
class PermissionRequestHookInput(BaseHookInput):
    hook_event_name: Literal["PermissionRequest"]
    tool_name: str
    tool_input: dict[str, Any]
    permission_suggestions: NotRequired[list[Any]]
    agent_id: NotRequired[str]
    agent_type: NotRequired[str]
```

| 字段 | 类型 | 描述 |
| :- | :- | :- |
| `hook_event_name` | `Literal["PermissionRequest"]` | 始终为 "PermissionRequest" |
| `tool_name` | `str` | 请求权限的工具的名称 |
| `tool_input` | `dict[str, Any]` | 工具的输入参数 |
| `permission_suggestions` | `list[Any]`（可选） | 来自 CLI 的建议权限更新 |
| `agent_id` | `str`（可选） | 子代理标识符，当 hook 在子代理内触发时存在 |
| `agent_type` | `str`（可选） | 子代理类型，当 hook 在子代理内触发时存在 |

### `HookJSONOutput`

hook 回调返回值的联合类型。

```python theme={null}
HookJSONOutput = AsyncHookJSONOutput | SyncHookJSONOutput
```

#### `SyncHookJSONOutput`

具有控制和决策字段的同步 hook 输出。

```python theme={null}
class SyncHookJSONOutput(TypedDict):
    # Control fields
    continue_: NotRequired[bool]  # Whether to proceed (default: True)
    suppressOutput: NotRequired[bool]  # Hide stdout from transcript
    stopReason: NotRequired[str]  # Message when continue is False

    # Decision fields
    decision: NotRequired[Literal["block"]]
    systemMessage: NotRequired[str]  # Warning message for user
    reason: NotRequired[str]  # Feedback for Claude

    # Hook-specific output
    hookSpecificOutput: NotRequired[HookSpecificOutput]
```

> [!NOTE]
>
> 在 Python 代码中使用 `continue_`（带下划线）。发送到 CLI 时会自动转换为 `continue`。

#### `HookSpecificOutput`

事件特定的 `TypedDict` 输出类型的判别联合。`hookEventName` 字段确定哪些字段有效。有关每个 hook 事件的可用字段的完整详情，见 [使用 hook 控制执行](https://code.claude.com/docs/zh-CN/agent-sdk/hooks.md#outputs)。

```python theme={null}
class PreToolUseHookSpecificOutput(TypedDict):
    hookEventName: Literal["PreToolUse"]
    permissionDecision: NotRequired[Literal["allow", "deny", "ask", "defer"]]
    permissionDecisionReason: NotRequired[str]
    updatedInput: NotRequired[dict[str, Any]]
    additionalContext: NotRequired[str]


class PostToolUseHookSpecificOutput(TypedDict):
    hookEventName: Literal["PostToolUse"]
    additionalContext: NotRequired[str]
    updatedToolOutput: NotRequired[Any]
    updatedMCPToolOutput: NotRequired[Any]  # MCP tools only. Prefer updatedToolOutput, which works for all tools


class PostToolUseFailureHookSpecificOutput(TypedDict):
    hookEventName: Literal["PostToolUseFailure"]
    additionalContext: NotRequired[str]


class UserPromptSubmitHookSpecificOutput(TypedDict):
    hookEventName: Literal["UserPromptSubmit"]
    additionalContext: NotRequired[str]


class NotificationHookSpecificOutput(TypedDict):
    hookEventName: Literal["Notification"]
    additionalContext: NotRequired[str]


class SubagentStartHookSpecificOutput(TypedDict):
    hookEventName: Literal["SubagentStart"]
    additionalContext: NotRequired[str]


class PermissionRequestHookSpecificOutput(TypedDict):
    hookEventName: Literal["PermissionRequest"]
    decision: dict[str, Any]


HookSpecificOutput = (
    PreToolUseHookSpecificOutput
    | PostToolUseHookSpecificOutput
    | PostToolUseFailureHookSpecificOutput
    | UserPromptSubmitHookSpecificOutput
    | NotificationHookSpecificOutput
    | SubagentStartHookSpecificOutput
    | PermissionRequestHookSpecificOutput
)
```

#### `AsyncHookJSONOutput`

延迟 hook 执行的异步 hook 输出。

```python theme={null}
class AsyncHookJSONOutput(TypedDict):
    async_: Literal[True]  # Set to True to defer execution
    asyncTimeout: NotRequired[int]  # Timeout in milliseconds
```

> [!NOTE]
>
> 在 Python 代码中使用 `async_`（带下划线）。发送到 CLI 时会自动转换为 `async`。

### Hook 使用示例

此示例注册两个 hook：一个阻止危险的 Bash 命令（如 `rm -rf /`），另一个记录所有工具使用以进行审计。安全 hook 仅在 Bash 命令上运行（通过 `matcher`），而日志 hook 在所有工具上运行。

```python theme={null}
import asyncio
from claude_agent_sdk import query, ClaudeAgentOptions, HookMatcher, HookContext
from typing import Any


async def validate_bash_command(
    input_data: dict[str, Any], tool_use_id: str | None, context: HookContext
) -> dict[str, Any]:
    """Validate and potentially block dangerous bash commands."""
    if input_data["tool_name"] == "Bash":
        command = input_data["tool_input"].get("command", "")
        if "rm -rf /" in command:
            return {
                "hookSpecificOutput": {
                    "hookEventName": "PreToolUse",
                    "permissionDecision": "deny",
                    "permissionDecisionReason": "Dangerous command blocked",
                }
            }
    return {}


async def log_tool_use(
    input_data: dict[str, Any], tool_use_id: str | None, context: HookContext
) -> dict[str, Any]:
    """Log all tool usage for auditing."""
    print(f"Tool used: {input_data.get('tool_name')}")
    return {}


options = ClaudeAgentOptions(
    hooks={
        "PreToolUse": [
            HookMatcher(
                matcher="Bash", hooks=[validate_bash_command], timeout=120
            ),  # 2 min for validation
            HookMatcher(
                hooks=[log_tool_use]
            ),  # Applies to all tools (per-event default timeout)
        ],
        "PostToolUse": [HookMatcher(hooks=[log_tool_use])],
    }
)

async def main():
    async for message in query(prompt="Analyze this codebase", options=options):
        print(message)


asyncio.run(main())
```

## 工具输入/输出类型

内置 Claude Code 工具的输入/输出 schema 文档。虽然 Python SDK 不会将这些作为类型导出，但它们代表了消息中工具输入和输出的结构。

下面展示的每个输出都是您从该工具的 [`UserMessage.tool_use_result`](#usermessage) 中读取的值。键名与 Claude Code 发出的完全一致。标注了 `| None` 并带有"present when"或"optional"注释的键，在不适用时会被省略。

### Agent

**工具名称：** `Agent`。当此工具对会话可用时，先前的名称 `Task` 仍作为别名被接受。

为了向后兼容，init [`SystemMessage`](#systemmessage) 中的 `tools` 列表会将此工具报告为 `Task`。

**输入：**

```python theme={null}
{
    "description": str,  # 任务的简短描述（3-5 个词）
    "prompt": str,  # 要由 Agent 执行的任务
    "subagent_type": str | None,  # 要使用的专用 Agent 类型
    "model": "sonnet" | "opus" | "haiku" | "fable" | None,  # 此 Agent 的模型覆盖
    "effort": "low" | "medium" | "high" | "xhigh" | "max" | None,  # 此 Agent 的推理强度
    "run_in_background": bool | None,  # 省略时的默认行为：主 Agent 的调用会在后台运行新 Agent，子代理的调用则会等待它完成
    "name": str | None,  # 所生成 Agent 的名称
    "team_name": str | None,  # 已弃用；会被忽略
    "mode": "acceptEdits" | "auto" | "bypassPermissions" | "default" | "dontAsk" | "plan" | None,  # 已弃用；会被忽略。子代理的权限模式由子代理继承规则决定
    "isolation": "worktree" | "remote" | None,  # Agent 所做更改的隔离模式
}
```

启动一个新的 Agent 来自主处理复杂的多步骤任务。

当 Claude 在 `Agent` 调用中省略 `run_in_background` 时，默认情况下，您的主 Agent 不会等待新 Agent，而子代理会等待：

* **从您的主 Agent 调用**：新 Agent 在后台运行，结果带有 `status: "async_launched"`。如需让主 Agent 改为等待新 Agent，请参阅[选择 Claude 是否等待子代理](https://code.claude.com/docs/zh-CN/agent-sdk/subagents.md#choose-whether-claude-waits-for-subagents)。
* **从您的某个子代理调用**：发起调用的子代理会[等待新 Agent](https://code.claude.com/docs/zh-CN/sub-agents.md#let-subagents-spawn-their-own-subagents)。当新 Agent 的定义将 [`background`](#agentdefinition) 设置为 `True`，或者您通过将 `CLAUDE_CODE_FORK_SUBAGENT` 设置为 `0` 来[关闭 fork 模式](https://code.claude.com/docs/zh-CN/sub-agents.md#turn-fork-mode-on-or-off)时，新 Agent 会改为在后台运行。

在 Python SDK v0.2.169 之前，由您的子代理启动的 Agent 默认也在后台运行。该版本是首个捆绑 Claude Code v2.1.302 或更高版本的版本。

**输出（status：`"completed"`）：**

```python theme={null}
{
    "status": "completed",
    "agentId": str,  # 运行的 Agent 的 ID
    "agentType": str | None,  # 处理该任务的子代理类型
    "content": [  # 结果内容块
        {
            "type": "text",
            "text": str,
            "citations": list | None,
        }
    ],
    "resolvedModel": str | None,  # 子代理启动时使用的模型
    "modelsUsed": list[str] | None,  # 按顺序使用的模型，连续重复项已合并
    "totalToolUseCount": int,  # Agent 进行的工具调用次数
    "totalDurationMs": int,  # 执行时长（毫秒）
    "totalTokens": int,  # 来自最后一次 API 请求的 token 数，而非整个运行过程
    "usage": {  # token 使用统计
        "input_tokens": int,
        "output_tokens": int,
        "cache_creation_input_tokens": int | None,
        "cache_read_input_tokens": int | None,
        "server_tool_use": {"web_search_requests": int, "web_fetch_requests": int} | None,
        "service_tier": str | None,
        "cache_creation": {"ephemeral_1h_input_tokens": int, "ephemeral_5m_input_tokens": int} | None,
        "inference_geo": str | None,
        "speed": str | None,
        "iterations": Any | None,
        "output_tokens_details": {"thinking_tokens": int | None} | None,
        "fallback_credit": Any | None,
    },
    "toolStats": {  # 本次运行的工具活动汇总
        "readCount": int,
        "searchCount": int,
        "bashCount": int,
        "editFileCount": int,
        "linesAdded": int,
        "linesRemoved": int,
        "otherToolCount": int,
        "frameCount": int | None,
    } | None,
    "prompt": str,  # Agent 运行的提示词
    "worktreePath": str | None,  # 当 Claude Code 保留了子代理的 worktree 时存在
    "worktreeBranch": str | None,  # 当 Claude Code 使用 git 创建了该 worktree 时存在
}
```

**输出（status：`"async_launched"`）：**

```python theme={null}
{
    "status": "async_launched",
    "isAsync": bool | None,  # 后台启动时为 True
    "agentId": str,  # 已启动 Agent 的 ID
    "description": str,  # 任务描述
    "resolvedModel": str | None,  # 转入后台时正在使用的模型
    "modelsUsed": list[str] | None,  # 转入后台之前按顺序使用的模型，连续重复项已合并
    "prompt": str,  # Agent 运行的提示词
    "outputFile": str,  # 写入 Agent 输出的文件路径
    "canReadOutputFile": bool | None,  # 是否可以直接读取输出文件
}
```

**输出（status：`"remote_launched"`）：**

```python theme={null}
{
    "status": "remote_launched",
    "taskId": str,  # 已派发任务的 ID
    "sessionUrl": str,  # 指向云端会话的链接
    "description": str,  # 任务描述
    "prompt": str,  # Agent 运行的提示词
    "outputFile": str,  # 写入 Agent 输出的文件路径
}
```

返回子代理的结果。输出通过 `status` 字段进行区分：`"completed"` 表示已完成的任务，`"async_launched"` 表示后台任务，`"remote_launched"` 表示 Claude Code 派发到云端会话的任务，其中 `sessionUrl` 链接到该会话，`taskId` 用于标识该会话。如果 Claude Code [保留了子代理的隔离 worktree](https://code.claude.com/docs/zh-CN/worktrees.md#isolate-subagents-with-worktrees)，`completed` 变体上的 `worktreePath` 就是该 worktree 的位置；当 Claude Code 使用 git 创建该 worktree 时，`worktreeBranch` 是其分支。

在 `completed` 变体上，`resolvedModel` 指明子代理启动时使用的模型；当 [`availableModels`](https://code.claude.com/docs/zh-CN/model-config#restrict-model-selection) 或其他覆盖生效时，它可能与请求的 `model` 输入不同。此字段需要 Claude Code v2.1.174 或更高版本。在 `async_launched` 变体上，`resolvedModel` 指明 Agent 转入后台时正在使用的模型，因此在转入后台之前发生的模型切换会反映在其中。两种变体上的 `modelsUsed` 字段按顺序列出所使用的模型，连续重复项已合并；仅当运行中途切换了模型时才会设置该字段。`modelsUsed` 以及转入后台时的 `resolvedModel` 行为需要 Claude Code v2.1.212 或更高版本。

Claude Code 根据子代理的最后一次 API 请求（而非整个运行过程）填充 `usage` 和 `totalTokens`。如果存在，`usage` 中 `output_tokens_details` 下的 `thinking_tokens` 表示该请求的输出 token 中属于思考 token 的数量。`output_tokens_details` 键需要 Python SDK v0.2.136 或更高版本，该版本捆绑了 Claude Code v2.1.228。`fallback_credit` 键需要 Python SDK v0.2.162 或更高版本，该版本捆绑了 Claude Code v2.1.285。

### AskUserQuestion

**工具名称：** `AskUserQuestion`

在执行过程中向用户提出澄清问题。有关用法详情，请参阅[处理批准和用户输入](https://code.claude.com/docs/zh-CN/agent-sdk/user-input.md#handle-clarifying-questions)。

**输入：**

```python theme={null}
{
    "questions": [  # 要向用户提出的问题（1-4 个问题）
        {
            "question": str,  # 要向用户提出的完整问题
            "header": str,  # 以标签/徽标形式显示的极短标签（最多 12 个字符）
            "options": [  # 可用选项（2-4 个选项）
                {
                    "label": str,  # 此选项的显示文本（1-5 个词）
                    "description": str,  # 对此选项含义的说明
                    "preview": str | None,  # 选项获得焦点时渲染的预览内容
                }
            ],
            "multiSelect": bool,  # 设置为 true 以允许多选
        }
    ],
    "answers": dict[str, str] | None,
    # 由权限系统填充的用户答案。多选答案是
    # 所选标签以逗号连接而成的字符串；输入时也接受
    # 标签列表，并会被转换为该形式
    "annotations": dict[str, dict] | None,
    # 用户针对每个问题的注释，以问题文本为键。
    # 每个值可以包含 "preview"（所选选项的预览
    # 内容）和 "notes"（关于所选内容的自由文本备注）
    "metadata": dict | None,  # 分析元数据，例如 {"source": "remember"}；不会向用户显示
}
```

**输出：**

```python theme={null}
{
    "questions": [  # 已提出的问题
        {
            "question": str,
            "header": str,
            "options": [{"label": str, "description": str, "preview": str | None}],
            "multiSelect": bool,
        }
    ],
    "answers": dict[str, str],  # 将问题文本映射到答案字符串
    # 多选答案以逗号分隔
    "response": str | None,
    # 用户未回答问题而是直接输入的自由回复；设置后，
    # Claude 收到的是 "The user responded: ..." 而不是答案列表
    "annotations": dict[str, dict] | None,  # 用户所选内容中针对每个问题的 "preview" 和 "notes"
    "afkTimeoutMs": int | None,  # 当对话框因用户在这么多毫秒内无操作而自动结束时设置；用户作答时不存在
}
```

### Bash

**工具名称：** `Bash`

有关前台上限由什么决定，请参阅[超时和输出限制](https://code.claude.com/docs/zh-CN/tools-reference.md#timeout-and-output-limits)。有关后台时间限制，请参阅[后台命令的时间限制](https://code.claude.com/docs/zh-CN/tools-reference.md#time-limit-for-background-commands)。

**输入：**

```python theme={null}
{
    "command": str,  # 要执行的命令
    "timeout": int | None,  # 毫秒。前台：默认上限为 600000，更高的值会被截断。使用 run_in_background 时（Claude Code v2.1.285 或更高版本）：后台时间限制，省略时为 1800000，除非调高，否则上限为 7200000
    "description": str | None,  # 清晰、简洁的描述（5-10 个词）
    "run_in_background": bool | None,  # 设置为 true 以在后台运行
}
```

**输出：**

```python theme={null}
{
    "stdout": str,  # 命令的输出；stdout 和 stderr 合并到这一个交错的流中
    "stderr": str,  # 工具本身添加的通知，而非命令的 stderr
    "interrupted": bool,  # 命令是否被中断
    "isImage": bool | None,  # stdout 是否包含图像数据
    "backgroundTaskId": str | None,  # 命令在后台运行时的后台任务 ID
}
```

### Monitor

**工具名称：** `Monitor`

运行一个后台来源，并将每个事件传递给 Claude，使其无需轮询即可做出响应：`command` 运行一个脚本，每行 stdout 产生一个事件；`ws` 打开一个 WebSocket，每个文本帧产生一个事件。`command` 和 `ws` 必须且只能提供其中一个。

当 Monitor 运行命令时，它遵循与 Bash 相同的权限规则；WebSocket 监视会单独请求批准。`ws` 来源需要 Claude Code v2.1.195 或更高版本。有关行为和提供商可用性，请参阅 [Monitor 工具参考](https://code.claude.com/docs/zh-CN/tools-reference.md#monitor-tool)。

**输入：**

```python theme={null}
{
    "command": str | None,  # Shell 脚本；每行 stdout 是一个事件，退出即结束监视
    "ws": dict | None,  # WebSocket 来源：{"url": str, "protocols": list[str] | None}；每个文本帧是一个事件
    "description": str,  # 在通知中显示的简短描述
    "timeout_ms": int | None,  # 截止时间（毫秒）（默认 300000，最大 3600000；实际生效的截止时间最多为 1800000）
}
```

**输出：**

```python theme={null}
{
    "taskId": str,  # 后台监视任务的 ID
    "timeoutMs": int,  # 监视实际生效的截止时间（毫秒）
    "persistent": bool | None,  # False：每个监视都有截止时间
}
```

### Edit

**工具名称：** `Edit`

**输入：**

```python theme={null}
{
    "file_path": str,  # 要修改的文件的绝对路径
    "old_string": str,  # 要替换的文本
    "new_string": str,  # 用于替换的文本
    "replace_all": bool | None,  # 替换所有匹配项（默认 False）
}
```

**输出：**

```python theme={null}
{
    "filePath": str,  # 被编辑的文件
    "oldString": str,  # 被替换的文本
    "newString": str,  # 替换后的文本
    "originalFile": str | None,  # 编辑前的文件内容
    "structuredPatch": [  # 此次更改的 diff 块
        {
            "oldStart": int,
            "oldLines": int,
            "newStart": int,
            "newLines": int,
            "lines": list[str],
        }
    ],
    "userModified": bool,  # 用户在接受之前是否修改了提议的编辑
    "replaceAll": bool,  # 是否替换了所有匹配项
    "gitDiff": {  # 可选的文件 git diff 摘要
        "filename": str,
        "status": "modified" | "added",
        "additions": int,
        "deletions": int,
        "changes": int,
        "patch": str,
        "repository": str | None,  # 可用时为 GitHub owner/repo
    } | None,
}
```

### Read

**工具名称：** `Read`

**输入：**

```python theme={null}
{
    "file_path": str,  # 要读取的文件的绝对路径
    "offset": int | None,  # 开始读取的行号
    "limit": int | None,  # 要读取的行数
}
```

根据 Claude 读取的内容，输出采用以下形状之一。请检查 `type` 键来区分它们。

**输出（type：`"text"`）：**

```python theme={null}
{
    "type": "text",
    "file": {
        "filePath": str,  # 被读取的文件
        "content": str,  # 返回的内容
        "numLines": int,  # 返回内容中的行数
        "startLine": int,  # 内容起始的行号
        "totalLines": int,  # 文件的总行数
        "truncatedByTokenCap": bool | None,  # 当整个文件的读取超出 token 上限且 content 为第一页时存在，值为 True
    },
}
```

**输出（type：`"image"`）：**

```python theme={null}
{
    "type": "image",
    "file": {
        "base64": str,  # Base64 编码的图像数据
        "type": "image/jpeg" | "image/png" | "image/gif" | "image/webp",  # 图像 MIME 类型
        "originalSize": int,  # 原始文件大小（字节）
        "dimensions": {  # 用于坐标映射的可选尺寸信息
            "originalWidth": int | None,  # 可选；原始宽度（像素）
            "originalHeight": int | None,  # 可选；原始高度（像素）
            "displayWidth": int | None,  # 可选；调整大小后的宽度
            "displayHeight": int | None,  # 可选；调整大小后的高度
        } | None,
    },
}
```

**输出（type：`"notebook"`）：**

```python theme={null}
{
    "type": "notebook",
    "file": {
        "filePath": str,  # 被读取的 notebook
        "cells": list,  # Notebook 单元格
    },
}
```

**输出（type：`"pdf"`）：**

```python theme={null}
{
    "type": "pdf",
    "file": {
        "filePath": str,  # 被读取的 PDF
        "base64": str,  # Base64 编码的 PDF 数据
        "originalSize": int,  # 文件大小（字节）
    },
}
```

**输出（type：`"parts"`）：**

```python theme={null}
{
    "type": "parts",
    "file": {
        "filePath": str,  # 被读取的 PDF
        "originalSize": int,  # 文件大小（字节）
        "count": int,  # 提取为图像的页数
        "outputDir": str,  # 包含所提取页面图像的目录
    },
    "firstPage": int | None,  # 可选；第一个提取页面在文档中的页码
}
```

**输出（type：`"file_unchanged"`）：**

```python theme={null}
{
    "type": "file_unchanged",  # 自 Claude 在此会话中上次读取以来文件未发生变化，因此不会重复返回内容
    "file": {
        "filePath": str,
    },
    "source": "seeded" | None,  # 当之前的副本来自启动时加载的 CLAUDE.md 或记忆文件（而非 Read 调用）时存在
}
```

### Write

**工具名称：** `Write`

**输入：**

```python theme={null}
{
    "file_path": str,  # 要写入的文件的绝对路径
    "content": str,  # 要写入文件的内容
}
```

**输出：**

```python theme={null}
{
    "type": "create" | "update",  # 此次写入是创建了新文件还是覆盖了现有文件
    "filePath": str,  # 被写入的文件
    "content": str,  # 写入的内容
    "structuredPatch": [  # diff 块；对于新文件、内容无变化或 Claude Code 跳过 diff 时为空
        {
            "oldStart": int,
            "oldLines": int,
            "newStart": int,
            "newLines": int,
            "lines": list[str],
        }
    ],
    "originalFile": str | None,  # 之前的内容；对于新文件或之前的内容过大而无法包含时为 None
    "gitDiff": {  # 可选的文件 git diff 摘要
        "filename": str,
        "status": "modified" | "added",
        "additions": int,
        "deletions": int,
        "changes": int,
        "patch": str,
        "repository": str | None,  # 可用时为 GitHub owner/repo
    } | None,
    "userModified": bool | None,  # 可选；用户在接受之前是否编辑了提议的内容
}
```

### Glob

**工具名称：** `Glob`

**输入：**

```python theme={null}
{
    "pattern": str,  # 用于匹配文件的 glob 模式
    "path": str | None,  # 要搜索的目录（默认为 cwd）
}
```

**输出：**

```python theme={null}
{
    "durationMs": int,  # 运行搜索所用的时间（毫秒）
    "numFiles": int,  # 截断后返回的路径数量
    "filenames": list[str],  # 匹配的文件路径
    "truncated": bool,  # 结果是否因 100 个文件的限制而被截断
    "totalMatches": int | None,  # 可选；截断前匹配文件的总数；当 countIsComplete 为 False 时为下限值
    "countIsComplete": bool | None,  # 可选；totalMatches 是否精确
}
```

`totalMatches` 和 `countIsComplete` 需要 Claude Code v2.1.191 或更高版本。

### Grep

**工具名称：** `Grep`

**输入：**

```python theme={null}
{
    "pattern": str,  # 正则表达式模式
    "path": str | None,  # 要搜索的文件或目录
    "glob": str | None,  # 用于过滤文件的 glob 模式
    "type": str | None,  # 要搜索的文件类型
    "output_mode": str | None,  # "content"、"files_with_matches" 或 "count"
    "-i": bool | None,  # 不区分大小写的搜索
    "-n": bool | None,  # 显示行号
    "-B": int | None,  # 每个匹配项之前显示的行数
    "-A": int | None,  # 每个匹配项之后显示的行数
    "-C": int | None,  # 前后显示的行数
    "context": int | None,  # 前后显示的行数；-C 是其别名
    "-o": bool | None,  # 仅打印每行中匹配的部分
    "head_limit": int | None,  # 将输出限制为前 N 行/条目
    "offset": int | None,  # 在应用 head_limit 之前跳过前 N 行/条目
    "multiline": bool | None,  # 启用多行模式
}
```

**输出：**

```python theme={null}
{
    "mode": "content" | "files_with_matches" | "count" | None,  # 所使用的输出模式
    "numFiles": int,  # 结果中的文件数；在 content 模式下始终为 0
    "filenames": list[str],  # files_with_matches 模式下的匹配文件；其他模式下为空
    "content": str | None,  # content 模式下的匹配行，或 count 模式下每个文件的计数
    "numLines": int | None,  # content 中的行数，在 content 模式下存在
    "numMatches": int | None,  # 匹配总数，在 count 模式下存在
    "totalFiles": int | None,  # 可选；应用 head_limit 和 offset 之前的总数，在 files_with_matches 模式下
    "totalLines": int | None,  # 可选；应用 head_limit 和 offset 之前的总数，在 content 模式下
    "appliedLimit": int | None,  # 当 head_limit 截断了结果时存在
    "appliedOffset": int | None,  # 当应用了 offset 时存在
}
```

Grep 在每种输出模式下都返回这种 dict 形状。存在哪些可选键取决于 `output_mode`。

`totalFiles` 需要 Claude Code v2.1.208 或更高版本。`totalLines` 需要 Claude Code v2.1.210 或更高版本。

### NotebookEdit

**工具名称：** `NotebookEdit`

**输入：**

```python theme={null}
{
    "notebook_path": str,  # Jupyter notebook 的绝对路径
    "cell_id": str | None,  # 要编辑的单元格的 ID
    "new_source": str,  # 单元格的新源代码
    "cell_type": "code" | "markdown" | None,  # 单元格的类型
    "edit_mode": "replace" | "insert" | "delete" | None,  # 编辑操作类型
}
```

**输出：**

```python theme={null}
{
    "new_source": str,  # 写入单元格的源代码
    "old_source": str | None,  # 之前的单元格源代码，replace 和 delete 时存在
    "cell_id": str | None,  # 被编辑单元格的 ID（如果可用）
    "cell_type": "code" | "markdown",  # 单元格类型
    "language": str,  # notebook 的编程语言
    "edit_mode": str,  # 所使用的编辑模式
    "error": str | None,  # 操作失败时的错误消息
    "notebook_path": str,  # notebook 文件
    "original_file": str,  # 编辑前的 notebook 内容
    "updated_file": str,  # 编辑后的 notebook 内容
}
```

### WebFetch

**工具名称：** `WebFetch`

**输入：**

```python theme={null}
{
    "url": str,  # 要从中获取内容的 URL
    "prompt": str,  # 要在获取的内容上运行的提示词
    "offset": int | None,  # 从页面开头跳过的字符数。需要 Python Agent SDK 0.2.164 或更高版本
}
```

**输出：**

```python theme={null}
{
    "bytes": int,  # 获取内容的大小（字节）
    "code": int,  # HTTP 响应码
    "codeText": str,  # HTTP 响应码文本
    "result": str,  # 将提示词应用于内容后得到的处理结果
    "durationMs": int,  # 获取和处理内容所用的时间（毫秒）
    "url": str,  # 被获取的 URL
}
```

### WebSearch

**工具名称：** `WebSearch`

**输入：**

```python theme={null}
{
    "query": str,  # 要使用的搜索查询
    "allowed_domains": list[str] | None,  # 仅包含来自这些域名的结果
    "blocked_domains": list[str] | None,  # 绝不包含来自这些域名的结果
}
```

**输出：**

```python theme={null}
{
    "query": str,  # 搜索查询
    "results": list[str | {"tool_use_id": str, "content": list[{"title": str, "url": str}]}],
    "durationSeconds": float,  # 搜索时长（秒）
}
```

### TodoWrite

**工具名称：** `TodoWrite`

> [!NOTE]
>
> 以下工具仅在 Claude Opus 4.7、Claude Sonnet 4.6、Claude Haiku 4.5 及更早的模型上默认可用。在所有其他模型上，包括 Claude Code 无法识别的模型 ID，除非您选择加入，否则它们不可用：
>
> * `TodoWrite`
> * `TaskCreate`
> * `TaskGet`
> * `TaskUpdate`
> * `TaskList`
>
> 无论工具在何处可用，Claude Code 都提供四个 Task 工具，或者当你设置 `CLAUDE_CODE_ENABLE_TASKS=0` 时改为提供 `TodoWrite`。
>
> 此默认集合适用于 Claude Code v2.1.268 及更高版本，TypeScript Agent SDK 从 v0.3.268 开始捆绑此版本。
>
> 请参阅[模型可用性](https://code.claude.com/docs/zh-CN/agent-sdk/todo-tracking.md#model-availability)以选择启用。

**输入：**

```python theme={null}
{
    "todos": [
        {
            "content": str,  # 任务描述
            "status": "pending" | "in_progress" | "completed",  # 任务状态
            "activeForm": str,  # 描述的进行时形式
        }
    ]
}
```

**输出：**

```python theme={null}
{
    "oldTodos": [  # 更新前的待办事项列表
        {
            "content": str,
            "status": "pending" | "in_progress" | "completed",
            "activeForm": str,
        }
    ],
    "newTodos": [  # 更新后的待办事项列表
        {
            "content": str,
            "status": "pending" | "in_progress" | "completed",
            "activeForm": str,
        }
    ],
}
```

### TaskCreate

**工具名称：** `TaskCreate`

**输入：**

```python theme={null}
{
    "subject": str,  # 简短的任务标题
    "description": str,  # 详细的任务正文
    "activeForm": str | None,  # 进行中时显示的现在时标签
    "metadata": dict | None,  # 任意调用方元数据
}
```

**输出：**

```python theme={null}
{
    "task": {"id": str, "subject": str},  # 已创建的任务及其分配的 ID
}
```

### TaskUpdate

**工具名称：** `TaskUpdate`

**输入：**

```python theme={null}
{
    "taskId": str,  # 要修改的任务的 ID
    "status": Literal["pending", "in_progress", "completed", "deleted"] | None,
    "subject": str | None,
    "description": str | None,
    "activeForm": str | None,
    "addBlocks": list[str] | None,  # 此任务现在阻塞的任务 ID
    "addBlockedBy": list[str] | None,  # 现在阻塞此任务的任务 ID
    "owner": str | None,
    "metadata": dict | None,
}
```

**输出：**

```python theme={null}
{
    "success": bool,
    "taskId": str,
    "updatedFields": list[str],  # 发生变化的字段名称
    "error": str | None,
    "statusChange": {"from": str, "to": str} | None,
}
```

### TaskGet

**工具名称：** `TaskGet`

**输入：**

```python theme={null}
{
    "taskId": str,  # 要读取的任务的 ID
}
```

**输出：**

```python theme={null}
{
    "task": {
        "id": str,
        "subject": str,
        "description": str,
        "status": Literal["pending", "in_progress", "completed"],
        "blocks": list[str],
        "blockedBy": list[str],
    } | None,  # 未找到该 ID 时为 None
}
```

### TaskList

**工具名称：** `TaskList`

**输入：**

```python theme={null}
{}
```

**输出：**

```python theme={null}
{
    "tasks": [
        {
            "id": str,
            "subject": str,
            "status": Literal["pending", "in_progress", "completed"],
            "owner": str | None,
            "blockedBy": list[str],
        }
    ],
}
```

### TaskOutput

已在 Claude Code v2.1.277 中移除。此前用于从正在运行或已完成的后台任务中检索输出，并接受 `BashOutput` 作为别名；现在 Claude 改用 `Read` 读取后台任务的输出文件。

仍然引用这两个名称之一的 `disallowed_tools` 条目或拒绝规则会被忽略，且不会发出警告。

### TaskStop

**工具名称：** `TaskStop`。您仍然可以在权限规则、hook 匹配器以及 `tools`、`allowed_tools` 和 `disallowed_tools` 选项中使用先前的名称 `KillShell` 和 `KillBash`。

**输入：**

```python theme={null}
{
    "task_id": str | None,  # 要停止的后台任务的 ID
    "shell_id": str | None,  # 已弃用：请改用 task_id
}
```

**输出：**

```python theme={null}
{
    "message": str,  # 关于该操作的状态消息
    "task_id": str,  # 已停止任务的 ID
    "task_type": str,  # 已停止任务的类型
    "command": str | None,  # 已停止任务的命令或描述
}
```

### ExitPlanMode

**工具名称：** `ExitPlanMode`

**输入：**

```python theme={null}
{
    "plan": str  # 要呈给用户过目以获得批准的计划
}
```

**输出：**

```python theme={null}
{
    "plan": str | None,  # 呈现给用户的计划
    "isAgent": bool,  # 当子代理调用该工具时为 True
    "filePath": str | None,  # 当计划已保存到文件时存在
    "hasTaskTool": bool | None,  # 可选；Agent 工具在当前上下文中是否可用
    "planWasEdited": bool | None,  # 当用户在批准之前编辑了计划时存在，值为 True
    "awaitingLeaderApproval": bool | None,  # 当队友将计划发送给团队负责人批准时存在，值为 True
    "requestId": str | None,  # 可选；该批准请求的 ID
}
```

### ListMcpResources

**工具名称：** `ListMcpResourcesTool`

**输入：**

```python theme={null}
{
    "server": str | None  # 可选；用于过滤资源的服务器名称
}
```

结果是一个列表而不是 dict，因此对于此工具，`tool_use_result` 包含一个 `list`。

**输出：**

```python theme={null}
[  # 每个资源一个条目
    {
        "uri": str,  # 资源 URI
        "name": str,  # 资源名称
        "mimeType": str | None,  # 可选的 MIME 类型
        "description": str | None,  # 可选的描述
        "server": str,  # 提供此资源的服务器
    }
]
```

### ReadMcpResource

**工具名称：** `ReadMcpResourceTool`

**输入：**

```python theme={null}
{
    "server": str,  # MCP 服务器名称
    "uri": str,  # 要读取的资源 URI
}
```

**输出：**

```python theme={null}
{
    "contents": [
        {
            "uri": str,  # 资源 URI
            "mimeType": str | None,  # 可选的 MIME 类型
            "text": str | None,  # 文本内容，或关于二进制内容的说明
            "blobSavedTo": str | None,  # 当 Claude Code 将二进制内容保存到磁盘时存在；为所保存文件的路径
        }
    ],
    "error": str | None,  # 当服务器无法读取该资源时存在
}
```

## 构建持续对话界面

以下示例保持一个 `ClaudeSDKClient` 在多个回合中保持连接，以便 Claude 记住之前的消息。输入 `new` 可以断开连接并重新连接以获得新的会话，或输入 `exit` 结束对话。

```python theme={null}
from claude_agent_sdk import (
    ClaudeSDKClient,
    ClaudeAgentOptions,
    AssistantMessage,
    TextBlock,
)
import asyncio


class ConversationSession:
    """Maintains a single conversation session with Claude."""

    def __init__(self, options: ClaudeAgentOptions | None = None):
        self.client = ClaudeSDKClient(options)
        self.turn_count = 0

    async def start(self):
        await self.client.connect()
        print("Starting conversation session. Claude will remember context.")
        print(
            "Commands: 'exit' to quit, 'interrupt' to stop current task, 'new' for new session"
        )

        while True:
            user_input = input(f"\n[Turn {self.turn_count + 1}] You: ")

            if user_input.lower() == "exit":
                break
            elif user_input.lower() == "interrupt":
                await self.client.interrupt()
                print("Task interrupted!")
                continue
            elif user_input.lower() == "new":
                # Disconnect and reconnect for a fresh session
                await self.client.disconnect()
                await self.client.connect()
                self.turn_count = 0
                print("Started new conversation session (previous context cleared)")
                continue

            # Send message - the session retains all previous messages
            await self.client.query(user_input)
            self.turn_count += 1

            # Process response
            print(f"[Turn {self.turn_count}] Claude: ", end="")
            async for message in self.client.receive_response():
                if isinstance(message, AssistantMessage):
                    for block in message.content:
                        if isinstance(block, TextBlock):
                            print(block.text, end="")
            print()  # New line after response

        await self.client.disconnect()
        print(f"Conversation ended after {self.turn_count} turns.")


async def main():
    options = ClaudeAgentOptions(
        allowed_tools=["Read", "Write", "Bash"], permission_mode="acceptEdits"
    )
    session = ConversationSession(options)
    await session.start()


# Example conversation:
# Turn 1 - You: "Create a file called hello.py"
# Turn 1 - Claude: "I'll create a hello.py file for you..."
# Turn 2 - You: "What's in that file?"
# Turn 2 - Claude: "The hello.py file I just created contains..." (remembers!)
# Turn 3 - You: "Add a main function to it"
# Turn 3 - Claude: "I'll add a main function to hello.py..." (knows which file!)

asyncio.run(main())
```

## 错误处理

以下示例将 `query()` 调用包装在四种 [错误类型](#error-types) 的处理程序中，这些是 SDK 会抛出的错误。

此示例捕获 [`ResultError`](#resulterror)，这需要 Python Agent SDK 0.2.140 或更高版本。

```python theme={null}
import asyncio

from claude_agent_sdk import (
    query,
    CLINotFoundError,
    ProcessError,
    ResultError,
    CLIJSONDecodeError,
)


async def main():
    try:
        async for message in query(prompt="Hello"):
            print(message)
    except CLINotFoundError:
        print(
            "Claude Code CLI not found. Try reinstalling: pip install --force-reinstall claude-agent-sdk"
        )
    # Catch ResultError before ProcessError, which it subclasses. Its message
    # carries the error text. A refusal and a failed final request both usually
    # arrive with subtype "success", so check stop_reason, then terminal_reason.
    except ResultError as e:
        if e.data.get("stop_reason") == "refusal":
            print(f"Model declined to respond: {e}")
        elif e.terminal_reason == "api_error":
            print(f"API request failed: {e}")
        else:
            print(f"Query ended with an error result ({e.terminal_reason or e.subtype}): {e}")
    except ProcessError as e:
        print(f"Process failed with exit code: {e.exit_code}")
    except CLIJSONDecodeError as e:
        print(f"Failed to parse response: {e}")


asyncio.run(main())
```

## 沙箱配置

### `SandboxSettings`

沙箱行为的配置。使用此来启用命令沙箱和以编程方式配置网络限制。

```python theme={null}
class SandboxSettings(TypedDict, total=False):
    enabled: bool
    autoAllowBashIfSandboxed: bool
    excludedCommands: list[str]
    allowUnsandboxedCommands: bool
    network: SandboxNetworkConfig
    ignoreViolations: SandboxIgnoreViolations
    enableWeakerNestedSandbox: bool
```

| 属性 | 类型 | 默认值 | 描述 |
| :- | :- | :- | :- |
| `enabled` | `bool` | `False` | 为命令执行启用沙箱模式 |
| `autoAllowBashIfSandboxed` | `bool` | `True` | 启用沙箱时自动批准 bash 命令 |
| `excludedCommands` | `list[str]` | `[]` | 绕过沙箱限制的命令，例如 `["docker *"]`。这些自动运行沙箱外，无需模型参与；[`sandbox.excludedCommands`](https://code.claude.com/docs/zh-CN/settings-reference#sandbox-excludedcommands) 涵盖何时应用条目 |
| `allowUnsandboxedCommands` | `bool` | `True` | 允许模型请求在沙箱外运行命令。当为 `True` 时，模型可以在工具输入中设置 `dangerouslyDisableSandbox`，这会回退到 [权限系统](#permissions-fallback-for-unsandboxed-commands) |
| `network` | [`SandboxNetworkConfig`](#sandboxnetworkconfig) | `None` | 网络特定的沙箱配置 |
| `ignoreViolations` | [`SandboxIgnoreViolations`](#sandboxignoreviolations) | `None` | 配置要忽略的沙箱违规 |
| `enableWeakerNestedSandbox` | `bool` | `False` | 启用较弱的嵌套沙箱以实现兼容性 |

> [!NOTE]
>
> 沙箱取决于平台支持，在 Linux 上，需要 `bubblewrap` 和 `socat` 等工具。默认情况下，当 `enabled` 为 `True` 但沙箱无法启动时，命令在沙箱外运行，并在 stderr 上显示警告。此默认值与 TypeScript SDK 不同，后者中 `failIfUnavailable` 默认为 `true`。
>
> 在沙箱设置中设置 `"failIfUnavailable": True` 以改为停止。SDK 会将 `failIfUnavailable` 与已声明的 `SandboxSettings` 字段一起转发给 Claude Code。然后 `query()` 报告一个 `ResultMessage`，其 `subtype="error_during_execution"`，原因在 `errors` 中。因为这是一个单次 `query()` 调用，SDK 在生成该错误结果后会引发异常，所以请将循环包装在 try 块中以继续执行。有关错误约定，请参阅 [处理结果](https://code.claude.com/docs/zh-CN/agent-sdk/agent-loop.md#handle-the-result)。

要阻止沙箱化命令写入某个文件，请在沙箱设置中将其绝对路径列在 [`denyWrite`](https://code.claude.com/docs/zh-CN/settings-reference#sandbox-filesystem-denywrite) 下，例如 `"filesystem": {"denyWrite": ["/home/user/project/.env"]}`。SDK 会将 `filesystem` 与已声明的 `SandboxSettings` 字段一起转发给 Claude Code。沙箱启动后，会阻止对您所列的每个绝对路径的写入。

#### 示例用法

```python theme={null}
import asyncio

from claude_agent_sdk import query, ClaudeAgentOptions

sandbox_settings = {
    "enabled": True,
    "autoAllowBashIfSandboxed": True,
    "failIfUnavailable": True,
    "network": {"allowLocalBinding": True},
}


async def main():
    try:
        async for message in query(
            prompt="Build and test my project",
            options=ClaudeAgentOptions(sandbox=sandbox_settings),
        ):
            print(message)
    except Exception as error:
        # A single-shot query() raises after yielding an error result,
        # such as when failIfUnavailable is set and the sandbox can't start.
        print(f"Session ended with an error: {error}")


asyncio.run(main())
```

> [!WARNING]
>
> **Unix socket 安全性**：`allowUnixSockets` 选项可以授予对系统服务的访问权限，这些服务可能会到达沙箱外。例如，允许 `/var/run/docker.sock` 实际上通过 Docker API 授予完整的主机系统访问权限，绕过沙箱隔离。仅允许严格必要的 Unix sockets，并理解每个的安全含义。

### `SandboxNetworkConfig`

沙箱模式的网络特定配置。这些设置适用于当父 [`SandboxSettings`](#sandboxsettings) 中的 `enabled` 为 `True` 时的沙箱化 Bash 命令。它们不限制 WebFetch 工具，该工具改用 [权限规则](https://code.claude.com/docs/zh-CN/permissions.md#webfetch)。

```python theme={null}
class SandboxNetworkConfig(TypedDict, total=False):
    allowedDomains: list[str]
    deniedDomains: list[str]
    allowManagedDomainsOnly: bool
    allowUnixSockets: list[str]
    allowAllUnixSockets: bool
    allowLocalBinding: bool
    allowMachLookup: list[str]
    httpProxyPort: int
    socksProxyPort: int
```

| 属性 | 类型 | 默认值 | 描述 |
| :- | :- | :- | :- |
| `allowedDomains` | `list[str]` | `[]` | 沙箱化进程可以访问的域名 |
| `deniedDomains` | `list[str]` | `[]` | 沙箱化进程无法访问的域名。优先于 `allowedDomains` |
| `allowManagedDomainsOnly` | `bool` | `False` | 仅限托管设置：在托管设置中设置时，忽略 `allowedDomains` 和来自非托管设置源的 `WebFetch(domain:...)` 允许规则。通过 SDK 选项设置时无效 |
| `allowUnixSockets` | `list[str]` | `[]` | 仅限 macOS：进程可以访问的 Unix socket 路径，例如 Docker socket。在 Linux 上被忽略 |
| `allowAllUnixSockets` | `bool` | `False` | 允许访问所有 Unix sockets，在 Linux 上还允许访问 VM sockets（`AF_VSOCK`） |
| `allowLocalBinding` | `bool` | `False` | 允许进程绑定到本地端口（例如开发服务器） |
| `allowMachLookup` | `list[str]` | `[]` | 仅限 macOS：允许的 XPC/Mach 服务名称。支持尾部通配符 |
| `httpProxyPort` | `int` | `None` | 网络请求的 HTTP 代理端口 |
| `socksProxyPort` | `int` | `None` | 网络请求的 SOCKS 代理端口 |

> [!NOTE]
>
> 内置沙箱代理基于请求的主机名强制执行网络允许列表，不会终止或检查 TLS 流量，因此 [域名前置](https://en.wikipedia.org/wiki/Domain_fronting) 等技术可能会绕过它。有关详细信息，请参阅 [沙箱安全限制](https://code.claude.com/docs/zh-CN/sandboxing.md#security-limitations)，以及 [安全部署](https://code.claude.com/docs/zh-CN/agent-sdk/secure-deployment.md#traffic-forwarding) 以配置 TLS 终止代理。

### `SandboxIgnoreViolations`

用于忽略特定沙箱违规的配置。

```python theme={null}
class SandboxIgnoreViolations(TypedDict, total=False):
    file: list[str]
    network: list[str]
```

| 属性 | 类型 | 默认值 | 描述 |
| :- | :- | :- | :- |
| `file` | `list[str]` | `[]` | 要忽略违规的文件路径模式 |
| `network` | `list[str]` | `[]` | 要忽略违规的网络模式 |

### 沙箱外命令的权限回退

当 `allowUnsandboxedCommands` 启用时，模型可以通过在工具输入中设置 `dangerouslyDisableSandbox: True` 来请求在沙箱外运行命令。这些请求回退到现有权限系统，意味着你的 `can_use_tool` 处理程序将被调用，允许你实现自定义授权逻辑。

你的 `excludedCommands` 条目改为自动绕过沙箱，无需模型参与；[`sandbox.excludedCommands`](https://code.claude.com/docs/zh-CN/settings-reference#sandbox-excludedcommands) 涵盖何时应用条目。

以下示例记录每个沙箱外请求并拒绝它，除非你自己的授权逻辑允许它：

```python theme={null}
import asyncio
from claude_agent_sdk import (
    query,
    ClaudeAgentOptions,
    HookMatcher,
    PermissionResultAllow,
    PermissionResultDeny,
    ToolPermissionContext,
)


def is_command_authorized(command: str | None) -> bool:
    # Replace with your own authorization logic
    return False



async def can_use_tool(
    tool: str, input: dict, context: ToolPermissionContext
) -> PermissionResultAllow | PermissionResultDeny:
    # Check if the model is requesting to bypass the sandbox
    if tool == "Bash" and input.get("dangerouslyDisableSandbox"):
        # The model is requesting to run this command outside the sandbox
        print(f"Unsandboxed command requested: {input.get('command')}")

        if is_command_authorized(input.get("command")):
            return PermissionResultAllow()
        return PermissionResultDeny(
            message="Command not authorized for unsandboxed execution"
        )
    return PermissionResultAllow()


# Required: dummy hook keeps the stream open for can_use_tool
async def dummy_hook(input_data, tool_use_id, context):
    return {"continue_": True}


async def prompt_stream():
    yield {
        "type": "user",
        "message": {"role": "user", "content": "Deploy my application"},
    }


async def main():
    async for message in query(
        prompt=prompt_stream(),
        options=ClaudeAgentOptions(
            sandbox={
                "enabled": True,
                "allowUnsandboxedCommands": True,  # Model can request unsandboxed execution
            },
            permission_mode="default",
            can_use_tool=can_use_tool,
            hooks={"PreToolUse": [HookMatcher(matcher=None, hooks=[dummy_hook])]},
        ),
    ):
        print(message)


asyncio.run(main())
```

> [!WARNING]
>
> 使用 `dangerouslyDisableSandbox: True` 运行的命令具有完整的系统访问权限。确保你的 `can_use_tool` 处理程序仔细验证这些请求。
>
> 如果 `permission_mode` 设置为 `bypassPermissions` 且 `allowUnsandboxedCommands` 启用，模型可以自主执行沙箱外的命令，无需批准提示，除了 [任何模式都不会自动批准的操作](https://code.claude.com/docs/zh-CN/permission-modes.md#actions-no-mode-auto-approves) 之外。此组合实际上允许模型无声地逃离沙箱隔离。

## 另见

* [SDK 概述](https://code.claude.com/docs/zh-CN/agent-sdk/overview.md) - 一般 SDK 概念
* [TypeScript SDK 参考](https://code.claude.com/docs/zh-CN/agent-sdk/typescript.md) - TypeScript SDK 文档
* [自定义工具](https://code.claude.com/docs/zh-CN/agent-sdk/custom-tools.md) - 为 Claude 定义可调用的进程内 MCP 工具
* [CLI 参考](https://code.claude.com/docs/zh-CN/cli-reference.md) - 命令行界面
* [常见工作流](https://code.claude.com/docs/zh-CN/common-workflows.md) - 分步指南
