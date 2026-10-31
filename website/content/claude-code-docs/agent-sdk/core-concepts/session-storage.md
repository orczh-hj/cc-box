> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# 将会话持久化到外部存储

> 将 Agent SDK 会话记录镜像到您自己的对象存储、键值存储或数据库，以便其他主机可以恢复您的会话。

默认情况下，SDK 将会话记录写入本地文件系统上 `~/.claude/projects/` 下的 JSONL 文件。`SessionStore` 适配器让您可以将这些记录镜像到您自己的后端，例如对象存储、键值存储或数据库，这样在一个主机上创建的会话可以在另一个主机上恢复，只要工作目录相同。

使用会话存储的常见原因：

* **多主机部署。** 无服务器函数、自动扩展的工作进程和 CI 运行器不共享文件系统。共享存储让副本可以恢复彼此的会话。
* **持久性。** 本地容器是临时的。外部存储可以在重启和重新部署后继续存在。
* **合规性和审计。** 将记录保存在您已经管理的存储中，使用您自己的保留规则、加密和访问控制。

## `SessionStore` 接口

`SessionStore` 是一个对象，具有两个必需的方法 `append` 和 `load`，以及四个可选方法。SDK 调用 `append` 在查询期间写入记录条目，调用 `load` 读取它们以便恢复。

```typescript TypeScript theme={null}
// Exported from @anthropic-ai/claude-agent-sdk as
// SessionStore, SessionKey, SessionStoreEntry, SessionSummaryEntry.

type SessionKey = {
  projectKey: string;
  sessionId: string;
  subpath?: string;
};

type SessionStore = {
  // Required
  append(key: SessionKey, entries: SessionStoreEntry[]): Promise<void>;
  load(key: SessionKey): Promise<SessionStoreEntry[] | null>;

  // Optional
  listSessions?(
    projectKey: string,
  ): Promise<Array<{ sessionId: string; mtime: number }>>;
  listSessionSummaries?(projectKey: string): Promise<SessionSummaryEntry[]>;
  delete?(key: SessionKey): Promise<void>;
  listSubkeys?(key: {
    projectKey: string;
    sessionId: string;
  }): Promise<string[]>;
};

type SessionSummaryEntry = {
  sessionId: string;
  mtime: number;
  data: Record<string, unknown>;
};
```

```python Python theme={null}
# Exported from claude_agent_sdk as
# SessionStore, SessionKey, SessionStoreEntry, SessionSummaryEntry.

class SessionKey(TypedDict):
    project_key: str
    session_id: str
    subpath: NotRequired[str]

class SessionStore(Protocol):
    # Required
    async def append(
        self, key: SessionKey, entries: list[SessionStoreEntry]
    ) -> None: ...
    async def load(self, key: SessionKey) -> list[SessionStoreEntry] | None: ...

    # Optional — omit or raise NotImplementedError
    async def list_sessions(
        self, project_key: str
    ) -> list[SessionStoreListEntry]: ...
    async def list_session_summaries(
        self, project_key: str
    ) -> list[SessionSummaryEntry]: ...
    async def delete(self, key: SessionKey) -> None: ...
    async def list_subkeys(self, key: SessionListSubkeysKey) -> list[str]: ...

class SessionSummaryEntry(TypedDict):
    session_id: str
    mtime: int
    data: dict[str, Any]
```

`SessionKey` 寻址一个记录。`projectKey` 是工作目录的稳定的、文件系统安全的编码，`sessionId` 是会话 UUID，`subpath` 在条目属于子代理记录或边车文件而不是主对话时设置。

因为 `projectKey` 编码了工作目录，请从与原始运行的工作目录匹配的工作目录中从存储恢复或继续。在 TypeScript 中，如果在查询的 [`env` 选项](https://code.claude.com/docs/zh-CN/agent-sdk/typescript#options)中在 `CLAUDE_CONFIG_DIR` 旁边设置 [`CLAUDE_CODE_PROJECT_DIR_NAME`](https://code.claude.com/docs/zh-CN/sessions#name-the-project-directory-yourself)，SDK 会改用该名称作为该查询的条目及其 `resume` 和 `continue` 查找的键。因为 `listSessions` 和 `deleteSession` 等独立帮助程序不接受 `env` 并读取进程环境，也要在主机进程环境中设置 `CLAUDE_CONFIG_DIR` 和相同的名称。需要 Agent SDK v0.3.234 或更高版本。

将 `subpath` 视为不透明的键后缀；它遵循磁盘上的布局，例如 `subagents/agent-<id>`。当 `subpath` 未定义时，该键指的是主记录。

| 方法 | 必需 | 调用时机 |
| :- | :- | :- |
| `append` | 是 | 在每批记录条目本地写入后。条目是 JSON 安全的对象，本地 JSONL 中每行一个。 |
| `load` | 是 | 在子进程生成之前，当设置 `resume` 时，以及在列表从 `listSessionSummaries` 回退时每个会话一次。如果会话未知，返回 `null`。 |
| `listSessions` | 否 | 由 `listSessions({ sessionStore })` 和 `query()`/`startup()` 与 `continue: true` 调用。如果未定义，`continue: true` 会抛出异常，`listSessions({ sessionStore })` 会抛出异常，除非实现了 `listSessionSummaries`。 |
| `listSessionSummaries` | 否 | 由 `listSessions({ sessionStore })` 在一次调用中读取所有会话的元数据。在 `append` 中维护摘要。如果未定义，列表会回退到 `listSessions` 加上每个会话的 `load`。 |
| `delete` | 否 | 由 `deleteSession({ sessionStore })` 调用。删除主键（无 `subpath`）必须级联到该会话的所有子键，并且还要删除会话的摘要条目，以便删除的会话停止出现在 `listSessionSummaries` 中。如果未定义，删除是无操作的，这适合仅追加的后端。 |
| `listSubkeys` | 否 | 在恢复期间，发现子代理记录。如果未定义，仅恢复主记录。 |

在 `SessionSummaryEntry` 中，`mtime` 是边车的存储写入时间，必须与 `listSessions` 返回的 `mtime` 值共享一个时钟源。`data` 是不透明的 SDK 拥有的状态；按原样持久化它，不要解释它。

通过在 `append` 中的每个批次上调用导出的 `foldSessionSummary` 帮助程序（Python 中为 `fold_session_summary`）来构建条目。跳过其键具有 `subpath` 的批次；子代理记录不得对主会话的摘要做出贡献。折叠永远不会设置 `mtime`：在持久化时通过 TypeScript 中的 `options.mtime` 参数或通过在 Python 中覆盖返回条目上的字段来标记它。同一会话的并发 `append` 调用可能在边车上竞争，因此使用事务、比较交换或每个会话的锁来序列化读取-折叠-写入；折叠本身是纯的。

有关 SDK 对记录 `load` 返回的处理，请参阅[从存储恢复](#resume-from-the-store)。

## 快速开始

SDK 附带一个 `InMemorySessionStore` 用于开发和测试。下面的示例使用附加的存储运行查询，从结果消息中捕获会话 ID，然后在第二个 `query()` 调用中从存储恢复。第二个调用传递相同的存储实例加上 `resume`，因此 SDK 从存储而不是本地文件系统加载记录：

```typescript TypeScript theme={null}
import { query, InMemorySessionStore } from "@anthropic-ai/claude-agent-sdk";

const store = new InMemorySessionStore();

let sessionId: string | undefined;
try {
  for await (const message of query({
    prompt: "List the TypeScript files under src/",
    options: { sessionStore: store },
  })) {
    if (message.type === "result") {
      sessionId = message.session_id;
    }
  }
} catch (error) {
  // A single-shot query() throws after yielding an error result. If the
  // failure was an error result, sessionId was already captured by the loop
  // above; connection or process failures yield no result message.
  console.error(`Session ended with an error: ${error}`);
}

// Resume from the store. The agent has full context from the first call.
for await (const message of query({
  prompt: "Summarize what those files do",
  options: { sessionStore: store, resume: sessionId },
})) {
  if (message.type === "result" && message.subtype === "success") {
    console.log(message.result);
  }
}
```

```python Python theme={null}
import asyncio
from claude_agent_sdk import (
    ClaudeAgentOptions,
    InMemorySessionStore,
    ResultMessage,
    query,
)

store = InMemorySessionStore()


async def main():
    session_id = None
    try:
        async for message in query(
            prompt="List the Python files under src/",
            options=ClaudeAgentOptions(session_store=store),
        ):
            if isinstance(message, ResultMessage):
                session_id = message.session_id
    except Exception as error:
        # A single-shot query() raises after yielding an error result. If the
        # failure was an error result, session_id was already captured by the
        # loop above; connection or process failures yield no result message.
        print(f"Session ended with an error: {error}")

    # Resume from the store. The agent has full context from the first call.
    async for message in query(
        prompt="Summarize what those files do",
        options=ClaudeAgentOptions(session_store=store, resume=session_id),
    ):
        if isinstance(message, ResultMessage) and message.subtype == "success":
            print(message.result)


asyncio.run(main())
```

第二个查询打印来自第一个查询的文件摘要，这表明代理从存储中恢复了完整的上下文。

## 编写您自己的适配器

针对您的后端实现 `append` 和 `load`。如果您希望 `listSessions()`、一次调用元数据读取、`deleteSession()` 和子代理恢复针对存储工作，请添加 `listSessions`、`listSessionSummaries`、`delete` 和 `listSubkeys`。

传递给 `append` 的条目类型为 `SessionStoreEntry`（一个 `{ type: string; ... }` 对象）。将它们视为不透明的 JSON 安全值：按顺序持久化它们，并从 `load` 以相同的顺序返回它们。`load` 必须返回与追加的条目深度相等的条目；不需要字节相等的序列化，因此像重新排序对象键的二进制 JSON 列类型这样的后端是可以的。

## 参考实现

两个 SDK 存储库在 TypeScript 的 [`examples/session-stores/`](https://github.com/anthropics/claude-agent-sdk-typescript/tree/main/examples/session-stores) 和 Python 的 [`examples/session_stores/`](https://github.com/anthropics/claude-agent-sdk-python/tree/main/examples/session_stores) 下都包含可运行的参考适配器。每种存储类型都有一个适配器，每个都展示了 `append` 和 `load` 如何映射到该类型的后端。它们不作为包发布；将最接近您后端的类型的适配器复制到您的项目中，安装您后端的客户端，并进行调整。

| 存储类型 | 存储模型 | 示例适配器 |
| :- | :- | :- |
| 对象存储 | 每个 `append()` 一个部分文件；`load()` 列出部分、排序并连接。 | S3 ([TypeScript](https://github.com/anthropics/claude-agent-sdk-typescript/tree/main/examples/session-stores/s3), [Python](https://github.com/anthropics/claude-agent-sdk-python/blob/main/examples/session_stores/s3_session_store.py)) |
| 键值存储 | 每个记录一个列表，`append()` 推送到该列表，`load()` 按范围读取，加上会话的排序索引。 | Redis ([TypeScript](https://github.com/anthropics/claude-agent-sdk-typescript/tree/main/examples/session-stores/redis), [Python](https://github.com/anthropics/claude-agent-sdk-python/blob/main/examples/session_stores/redis_session_store.py)) |
| 关系数据库或文档存储 | 每个条目一行或一个文档，存储为 JSON 并按插入时分配的键排序。 | Postgres ([TypeScript](https://github.com/anthropics/claude-agent-sdk-typescript/tree/main/examples/session-stores/postgres), [Python](https://github.com/anthropics/claude-agent-sdk-python/blob/main/examples/session_stores/postgres_session_store.py)) |

每个适配器都采用预配置的客户端实例，因此您可以控制凭证、TLS、区域和池。以下示例将对象存储适配器连接到 `query()` 中，然后在另一台主机上从中恢复：

```typescript TypeScript theme={null}
import { query } from "@anthropic-ai/claude-agent-sdk";
import { S3Client } from "@aws-sdk/client-s3";
import { S3SessionStore } from "./S3SessionStore"; // copied from examples/session-stores/s3

const store = new S3SessionStore({
  bucket: "my-claude-sessions",
  prefix: "transcripts",
  client: new S3Client({ region: "us-east-1" }),
});

for await (const message of query({
  prompt: "Hello!",
  options: { sessionStore: store },
})) {
  if (message.type === "result" && message.subtype === "success") {
    console.log(message.result);
  }
}

// Later, possibly on a different host:
for await (const message of query({
  prompt: "Continue where we left off",
  options: { sessionStore: store, resume: "previous-session-id" },
})) {
  // ...
}
```

### 验证您的适配器

两个 SDK 都附带一个一致性套件，该套件断言 `append`、`load` 和可选方法必须满足的行为契约。当未实现这些方法时，可选方法的测试会自动跳过。

在 TypeScript 中，从示例目录将 [`shared/conformance.ts`](https://github.com/anthropics/claude-agent-sdk-typescript/blob/main/examples/session-stores/shared/conformance.ts) 复制到您的测试套件中。在 Python 中，该套件在包中提供。要使用 pytest 运行它（pytest 不是 SDK 依赖项），请先安装 pytest：

```bash theme={null}
pip install pytest
```

然后在测试文件中将您的适配器作为零参数工厂传递给套件，`run_session_store_conformance` 会为每个契约调用一次以构建一个新的存储：

```python Python theme={null}
import pytest
from claude_agent_sdk.testing import run_session_store_conformance


@pytest.mark.anyio
async def test_my_store_conformance():
    await run_session_store_conformance(MyRedisStore)
```

按照此示例传递 `MyRedisStore` 类本身，当构造函数不带参数时有效。对于采用预配置客户端的适配器，改为传递构造存储的 lambda。由于契约重用相同的会话键，工厂返回的每个存储必须以空存储开始，因此让 lambda 为每次调用配置隔离的后端存储，例如新的内存中伪造、唯一的键前缀或新的测试数据库。

## 行为说明

### 双写架构

Claude Code 子进程始终首先将每批会话记录条目写入本地磁盘，然后 SDK 将同一批转发到您的存储的 `append()`，因此存储是本地会话记录的镜像而不是替代品。哪个副本在运行后保留取决于运行的启动方式：

* **新会话，或当存储对该会话没有任何内容时的恢复**：您配置目录下的本地会话记录在运行后保留，存储接收一个副本。
* **从存储[恢复的运行](#resume-from-the-store)**：本地副本在运行结束时被删除，因此存储持有唯一的持久副本。

如果您不希望新会话在本地磁盘上留下会话记录，请在 `options.env` 中将 `CLAUDE_CONFIG_DIR` 设置为一个新的私有临时目录。从存储恢复的运行已经删除其本地副本，因此不需要此类设置。在 TypeScript 中，也将 `process.env` 展开到 `env` 中，因为 [`env` 选项](https://code.claude.com/docs/zh-CN/agent-sdk/typescript#options) 替换子进程环境。

如果您将 `CLAUDE_CONFIG_DIR` 设置为临时目录，并通过 [claude.ai 登录](https://code.claude.com/docs/zh-CN/authentication.md#credential-management)或用户 `settings.json` 中的 [`apiKeyHelper`](https://code.claude.com/docs/zh-CN/settings-reference#apikeyhelper) 进行登录，运行将失败并显示 `Not logged in`。要对运行进行身份验证，请设置长期令牌、设置 API 密钥或复制您的配置文件：

* **长期令牌**：如果您使用自己的 Claude 订阅亲自运行 Agent，请在 `env` 中将 `CLAUDE_CODE_OAUTH_TOKEN` 设置为通过 [`claude setup-token`](https://code.claude.com/docs/zh-CN/authentication#generate-a-long-lived-token) 获取的令牌。该令牌只能用于发出模型请求，因此您的 [claude.ai 连接器](https://code.claude.com/docs/zh-CN/mcp.md#use-mcp-servers-from-claude-ai)在该运行中不可用。
* **API 密钥**：在 `env` 中设置 `ANTHROPIC_API_KEY`。
* **配置文件**：如果您通过常用配置目录中的文件登录，例如 OAuth 凭据或用户 `settings.json` 中的 `apiKeyHelper`，请将这些文件复制到临时目录中。

两个选项与镜像冲突，如果您将其中任何一个与存储结合，SDK 会在启动时抛出异常：

* **`persistSession: false`** 在 TypeScript 中：关闭镜像所基于的本地写入。Python SDK 没有等效选项。
* **文件检查点功能**，TypeScript 中的 `enableFileCheckpointing` 或 Python 中的 `enable_file_checkpointing`：将其文件备份直接写入本地磁盘，SDK 不会将其镜像到存储。

### 从存储恢复

当您传递 `resume`，或 TypeScript 中的 `continue: true` 或 Python 中的 `continue_conversation=True`，以及存储时，SDK 在生成子进程之前向存储请求会话记录：

* **`resume`**：SDK 请求您传递的会话 ID 的会话。
* **`continue: true`** 或 **`continue_conversation=True`**：SDK 请求存储的最新会话。

当存储返回会话记录时，SDK 将其写入临时配置目录，运行子进程时 `CLAUDE_CONFIG_DIR` 指向该目录，并在运行结束时删除该目录。该运行写入的本地会话记录随之被删除，这就是为什么存储在此路径上持有唯一的持久副本的原因。

SDK 还使用来自您真实配置目录的文件为临时目录提供种子。它复制的内容因语言而异：

* **TypeScript**：凭据、`.claude.json` 和您的用户 `settings.json`。从 `settings.json` 中，它删除在临时配置目录下表现不佳的键：`enabledPlugins`、`extraKnownMarketplaces`、其 [`additionalMarketplaces`](https://code.claude.com/docs/zh-CN/settings-reference#extraknownmarketplaces) 别名，以及文件 `env` 块中的任何 `CLAUDE_CONFIG_DIR`。在 Agent SDK v0.3.232 之前，SDK 没有删除别名。在设置中配置的身份验证，例如 [`apiKeyHelper`](https://code.claude.com/docs/zh-CN/settings-reference#apikeyhelper)，在从存储恢复时有效。在 Agent SDK v0.3.222 之前，TypeScript SDK 仅复制凭据和 `.claude.json`。
* **Python**：仅凭据和 `.claude.json`，因此通过用户 `settings.json` 中的 `apiKeyHelper` 进行身份验证的应用在从存储恢复时失败并显示 `Not logged in`。托管或项目设置中的 `apiKeyHelper` 仍然有效，因为 Claude Code 从 `CLAUDE_CONFIG_DIR` 不影响的位置读取这些文件。

当存储对该会话没有任何内容时，SDK 改为在您的真实配置目录下运行，结果取决于您传递的选项：

* **`resume`**：两个 SDK 都将 ID 传递给子进程，子进程恢复本地会话记录，完全如同没有存储的 `resume` 一样。
* **TypeScript 中的 `continue: true`**：SDK 启动新会话。
* **Python 中的 `continue_conversation=True`**：SDK 从最新的本地会话继续。

### 镜像写入是尽力而为的

如果 `append()` 拒绝，SDK 会以短退避重试该批次最多两次，总共最多三次尝试。超时的调用不会重试，因为原始调用可能仍然会成功。如果批次仍然失败，SDK 记录错误，向迭代器发出 `{ type: "system", subtype: "mirror_error" }` 消息，丢弃批次，并继续查询。因为重试的批次可以重新传递已经成功的条目，请在您的 `append()` 实现中按 `entry.uuid` 进行去重。

存储中断不会中断 Agent，因为子进程首先在本地写入。如果您需要检测存储数据丢失，请监视 `mirror_error`。在[从存储恢复](#resume-from-the-store)的运行上，丢弃的批次在运行结束后没有幸存的副本。

### `getSessionMessages` 返回后压缩链

`getSessionMessages({ sessionStore })` 返回 Agent 在恢复时会看到的链接消息链。自动压缩后，早期的轮次被摘要替换，因此存储中包含 503 个原始条目的会话可能从 `getSessionMessages` 返回 18 条消息。对于完整的原始历史记录，包括压缩前的轮次和元数据条目，直接调用 `store.load(key)`。

### `forkSession` 不是字节副本

`forkSession({ sessionStore })` 读取源条目，重写每个 `sessionId` 字段并重新映射消息 UUID，然后在新键下追加转换后的条目。适配器级别的副本或 `CopyObject` 快捷方式会产生仍然引用旧会话 ID 的会话记录，因此 SDK 不使用它。

### 子代理会话记录

子代理会话记录在 `subpath: "subagents/agent-<id>"` 下镜像。`listSubagents({ sessionStore })` 要求适配器实现 `listSubkeys`；`getSubagentMessages({ sessionStore })` 在可用时使用它，但在未定义时回退到直接子路径。恢复也调用 `listSubkeys` 来恢复子代理文件；没有它，仅实现主会话记录。

### 保留

SDK 永远不会自行从您的存储中删除。保留是适配器的责任：根据您的合规要求，使用您后端的过期或生命周期机制，或运行计划清理。

`CLAUDE_CONFIG_DIR` 下的本地会话记录由 `cleanupPeriodDays` 设置独立清理，遵循[保留清理规则](https://code.claude.com/docs/zh-CN/claude-directory.md#cleaned-up-automatically)。[从存储恢复](#resume-from-the-store)的运行不留下本地会话记录，因此对于这些运行，您的存储保留是唯一的保留。

## 支持的功能

以下 TypeScript SDK 函数接受 `sessionStore` 选项，当提供时针对存储而不是本地文件系统操作：

* [`query()`](https://code.claude.com/docs/zh-CN/agent-sdk/typescript#query)
* [`startup()`](https://code.claude.com/docs/zh-CN/agent-sdk/typescript#startup)
* [`listSessions()`](https://code.claude.com/docs/zh-CN/agent-sdk/typescript#listsessions)
* [`getSessionInfo()`](https://code.claude.com/docs/zh-CN/agent-sdk/typescript#getsessioninfo)
* [`getSessionMessages()`](https://code.claude.com/docs/zh-CN/agent-sdk/typescript#getsessionmessages)
* [`renameSession()`](https://code.claude.com/docs/zh-CN/agent-sdk/typescript#renamesession)
* [`tagSession()`](https://code.claude.com/docs/zh-CN/agent-sdk/typescript#tagsession)
* [`deleteSession()`](https://code.claude.com/docs/zh-CN/agent-sdk/typescript)
* [`forkSession()`](https://code.claude.com/docs/zh-CN/agent-sdk/typescript#forksession)
* [`listSubagents()`](https://code.claude.com/docs/zh-CN/agent-sdk/typescript)
* [`getSubagentMessages()`](https://code.claude.com/docs/zh-CN/agent-sdk/typescript)

在 Python SDK 中，在 [`ClaudeAgentOptions`](https://code.claude.com/docs/zh-CN/agent-sdk/python#claudeagentoptions) 中设置 `session_store` 以针对存储运行 `query()`。其余操作各有一个接受存储作为参数的存储支持的 Python 函数：`list_sessions_from_store()`、`get_session_info_from_store()`、`get_session_messages_from_store()`、`list_subagents_from_store()`、`get_subagent_messages_from_store()`、`rename_session_via_store()`、`tag_session_via_store()`、`delete_session_via_store()` 和 `fork_session_via_store()`。`startup()` 没有 Python 等效项。[Python SDK 参考](https://code.claude.com/docs/zh-CN/agent-sdk/python.md#functions)中记录的独立函数（如 `list_sessions()`）读取本地会话文件。

## 相关资源

* [使用会话](https://code.claude.com/docs/zh-CN/agent-sdk/sessions.md)：在没有自定义存储的情况下继续、恢复和分叉
* [托管 SDK](https://code.claude.com/docs/zh-CN/agent-sdk/hosting.md)：多主机环境的部署模式
* [TypeScript `Options`](https://code.claude.com/docs/zh-CN/agent-sdk/typescript#options)：完整的选项参考
* [参考实现](#reference-implementations)：对象存储、键值存储和数据库的可运行示例适配器，在两个 SDK 存储库中
