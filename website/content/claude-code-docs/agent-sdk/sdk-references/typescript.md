> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Agent SDK 参考 - TypeScript

> TypeScript Agent SDK 的完整 API 参考，包括所有函数、类型和接口。

## 安装

```bash theme={null}
npm install @anthropic-ai/claude-agent-sdk
```

> [!NOTE]
>
> SDK 为您的平台捆绑了一个本地 Claude Code 二进制文件，作为可选依赖项，例如 `@anthropic-ai/claude-agent-sdk-darwin-arm64`。大多数安装无需单独安装 Claude Code。SDK 版本跟踪捆绑的 Claude Code 版本。SDK v0.3.191 捆绑 Claude Code v2.1.191，因此本页面上需要特定 Claude Code 版本的功能需要具有相同补丁号或更高版本的 SDK 版本。如果您的包管理器跳过可选依赖项，SDK 会抛出 `Native CLI binary for <platform>-<arch> not found`；改为将 [`pathToClaudeCodeExecutable`](#options) 设置为单独安装的 `claude` 二进制文件。
>
> 如果您的包管理器不应用 npm 的 `libc` 字段（如 Yarn 1.x 不应用），您会在 Linux 上同时获得 glibc 和 musl 平台包，大约使安装大小翻倍。在 Agent SDK v0.2.141 或更高版本上，SDK 仍然会启动正确的变体。要在容器镜像中回收空间，请删除与您的应用运行的 libc 不匹配的平台包；对于 x64 上的 glibc 运行时，即 `rm -rf node_modules/@anthropic-ai/claude-agent-sdk-linux-x64-musl`。在开发机器上删除是临时的，因为 Yarn 会在下一次依赖项更改时重新安装该包。

### 编译为单个可执行文件

当您使用 `bun build --compile` 将应用程序编译为单文件可执行文件时，SDK 无法在运行时解析捆绑的 CLI 二进制文件。`require.resolve` 在编译后的可执行文件的 `$bunfs` 虚拟文件系统内不起作用，因此 SDK 会抛出 `Native CLI binary for <platform>-<arch> not found`。

要解决此问题，请将平台二进制文件作为文件资产嵌入，在启动时使用 `extractFromBunfs()` 将其提取到真实路径，然后将该路径传递给 [`pathToClaudeCodeExecutable`](#options)。

`extractFromBunfs()` 辅助函数需要 `@anthropic-ai/claude-agent-sdk` v0.3.144 或更高版本。下面的示例为 Apple Silicon 上的 macOS 构建：

```typescript theme={null}
import binPath from "@anthropic-ai/claude-agent-sdk-darwin-arm64/claude" with { type: "file" };
import { extractFromBunfs } from "@anthropic-ai/claude-agent-sdk/extract";
import { query } from "@anthropic-ai/claude-agent-sdk";

const cliPath = extractFromBunfs(binPath);

for await (const message of query({
  prompt: "Hello",
  options: { pathToClaudeCodeExecutable: cliPath },
})) {
  console.log(message);
}
```

`extractFromBunfs()` 将嵌入的二进制文件从编译后的可执行文件的虚拟文件系统复制到每个用户的临时目录，并返回真实路径。在编译后的可执行文件之外，它返回输入路径不变，因此相同的代码在开发中无需修改即可运行。

每个编译后的可执行文件都嵌入了单个平台的二进制文件。将导入中的平台包与您的 `--target` 匹配：

* 要进行交叉编译，请安装不匹配的平台包，例如 `npm install @anthropic-ai/claude-agent-sdk-linux-x64 --force`。
* 在 Windows 上，二进制文件子路径是 `claude.exe`，例如 `@anthropic-ai/claude-agent-sdk-win32-x64/claude.exe`。

### 在捆绑 Agent SDK 时导入 `/core` 入口

如果您的应用程序将 Agent SDK 与其自己的依赖项一起捆绑，请从 `@anthropic-ai/claude-agent-sdk/core` 而不是包根目录导入。`/core` 入口需要 TypeScript Agent SDK v0.3.282 或更高版本，其类型需要 TypeScript 5.0 或更高版本。

`/core` 入口导出与根入口相同的 `query()`、`startup()`、`tool()`、`createSdkMcpServer()` 和 `resolveSettings()`，以及重命名、标记和删除会话的函数、`AbortError`、运行时常量和每种类型。它不添加自己的名称。为了保持应用程序加载的代码较小，`/core` 省略了一些根导出，包括 `prewarm()`、`InMemorySessionStore` 类以及列出、读取、分叉、导入和总结会话的辅助函数。如果您需要其中之一，请改用根入口。

根入口内联了自己的 `zod` 和 `@modelcontextprotocol/sdk` 副本。`/core` 入口从您的 `node_modules` 按照 Agent SDK 的 `peerDependencies` 声明的范围导入它们，因此已经包含它们的捆绑包不会携带第二个副本。在给定的进程中从根或 `/core` 导入，而不是两者：它们是单独的捆绑包，加载两者会给您两个 Agent SDK 类和状态的副本。

## 函数

### `query()`

与 Claude Code 交互的主要函数。创建一个异步生成器，在消息到达时以流式方式输出消息。

```typescript theme={null}
function query({
  prompt,
  options
}: {
  prompt: string | AsyncIterable<SDKUserMessage>;
  options?: Options;
}): Query;
```

#### 参数

| 参数 | 类型 | 描述 |
| :- | :- | :- |
| `prompt` | `string \| AsyncIterable<`[`SDKUserMessage`](#sdkusermessage)`>` | 输入提示词，可以是字符串，也可以是用于流式模式的异步可迭代对象 |
| `options` | [`Options`](#options) | 可选的配置对象（参见下文的 Options 类型） |

#### 返回值

返回一个 [`Query`](#query-object) 对象，该对象扩展了 `AsyncGenerator<`[`SDKMessage`](#sdkmessage)`, void>` 并提供了额外的方法。

### `startup()`

在提示词可用之前，通过启动 CLI 子进程并完成初始化握手来预热该子进程。返回的 [`WarmQuery`](#warmquery) 句柄可在稍后接收提示词，并将其写入已就绪的进程，因此第一次 `query()` 调用在解析时无需在调用路径中承担子进程启动和初始化的开销。如果您尚不知道会话的工作目录，请改用 [`prewarm()`](#prewarm)。

```typescript theme={null}
function startup(params?: {
  options?: Options;
  initializeTimeoutMs?: number;
}): Promise<WarmQuery>;
```

#### 参数

| 参数 | 类型 | 描述 |
| :- | :- | :- |
| `options` | [`Options`](#options) | 可选的配置对象。与 `query()` 的 `options` 参数相同 |
| `initializeTimeoutMs` | `number` | 等待子进程初始化的最长时间（毫秒）。默认为 `60000`。如果初始化未能在此时间内完成，promise 将以超时错误被拒绝 |

#### 返回值

返回一个 `Promise<`[`WarmQuery`](#warmquery)`>`，在子进程启动并完成初始化握手后解析。

对于 `SessionStart` 或 `Setup` hook 在握手完成之前发出的 [`hook_progress`](#sdkhookprogressmessage) 消息，您只会收到最新的一条，其中包含该 hook 截至当时的全部输出。`startup()` 会暂存该消息、该 hook 的 [`hook_started`](#sdkhookstartedmessage) 消息，以及子进程在握手完成之前发出的所有其他内容。当您对 `WarmQuery` 调用 `.query()` 并迭代结果时，这些暂存的消息才会传递给您。

#### 示例

尽早调用 `startup()`（例如在应用程序启动时），然后在提示词就绪后对返回的句柄调用 `.query()`。这样可以将子进程的启动和初始化移出关键路径。

```typescript theme={null}
import { startup } from "@anthropic-ai/claude-agent-sdk";

// Pay startup cost upfront
const warm = await startup({ options: { maxTurns: 3 } });

// Later, when a prompt is ready, this is immediate
for await (const message of warm.query("What files are here?")) {
  console.log(message);
}
```

### `prewarm()`

*Alpha。* 在您知道某个 Claude Code 进程将服务于哪个会话之前，先将其作为备用进程启动，以便稍后通过 [`claim()`](#spareprocess) 将其绑定到会话。适用于在用户选择文件夹之前就已启动的应用程序。需要 TypeScript Agent SDK v0.3.282 或更高版本。

`prewarm()` 完成与 [`startup()`](#startup) 相同的初始化握手；如果您设置了 `options.cwd`，进程将在该目录中等待，否则将在您的 Claude Code 配置目录下的一个私有临时目录中等待。会话的工作目录、其 `SessionStart` hook、其 stdio MCP 服务器以及其 CLAUDE.md 和 git 上下文都要等到认领时才会确定。备用进程在等待期间大约占用 230 到 260 MB 内存。如果您的 [`spawnClaudeCodeProcess`](#options) 在另一台机器或容器中运行 Claude Code，请将 `options.cwd` 设置为该环境中存在的目录，供备用进程在其中等待。

```typescript theme={null}
function prewarm(params?: {
  options?: Options;
  initializeTimeoutMs?: number;
}): Promise<SpareProcess>;
```

`options` 和 `initializeTimeoutMs` 的含义与 `startup()` 中相同，只是 `options.cwd` 仅设置备用进程等待所在的目录。进程完成初始化握手后，promise 将解析为一个 [`SpareProcess`](#spareprocess)。如果 `options` 设置了 `resume`、`continue` 或 `forkSession`，`prewarm()` 会抛出错误，因为备用进程尚无会话。认领时无法设置的所有内容，例如 `mcpServers`、`hooks`、`canUseTool`、`settingSources`、`systemPrompt` 和 `plugins`，在备用进程的整个生命周期内都是固定的，因此请为每组不同的此类选项保留一个备用进程，并在这些选项发生变化时重新预热。

#### 示例

在应用程序启动时预热，然后在用户开始会话时认领备用进程：

```typescript theme={null}
import { prewarm } from "@anthropic-ai/claude-agent-sdk";

// On application boot, before the session's folder is known
const spare = await prewarm({ options: { maxTurns: 3 } });

// Later, when the user starts a session in a folder
const claimedQuery = spare.claim({
  prompt: "What files are here?",
  options: { cwd: "/path/to/project" },
});

spare.claimed.catch((error: Error) => {
  // Unless the message starts with "option_not_applied", the prompt didn't run:
  // start this session with query() instead
  console.error("Claim failed:", error.message);
});

try {
  for await (const message of claimedQuery) {
    console.log(message);
  }
} catch (error) {
  // After a refused claim, the claimed query throws once it has yielded the error result
  console.error(`Session ended with an error: ${error}`);
}
```

### `tool()`

创建类型安全的 MCP 工具定义，供 SDK MCP 服务器使用。

```typescript theme={null}
function tool<Schema extends AnyZodRawShape>(
  name: string,
  description: string,
  inputSchema: Schema,
  handler: (args: InferShape<Schema>, extra: unknown) => Promise<CallToolResult>,
  extras?: { annotations?: ToolAnnotations; searchHint?: string; alwaysLoad?: boolean }
): SdkMcpToolDefinition<Schema>;
```

#### 参数

| 参数 | 类型 | 描述 |
| :- | :- | :- |
| `name` | `string` | 工具的名称 |
| `description` | `string` | 对工具功能的描述 |
| `inputSchema` | `Schema extends AnyZodRawShape` | 定义工具输入参数的 Zod schema（同时支持 Zod 3 和 Zod 4） |
| `handler` | `(args, extra) => Promise<`[`CallToolResult`](#calltoolresult)`>` | 执行工具逻辑的异步函数 |
| `extras` | `{ annotations?: `[`ToolAnnotations`](#toolannotations)`; searchHint?: string; alwaysLoad?: boolean }` | 可选的附加项。`annotations` 为客户端提供 MCP 行为提示。`searchHint` 是一个单行的能力描述短语，在启用 [工具搜索](https://code.claude.com/docs/zh-CN/agent-sdk/tool-search.md) 时显示在延迟加载工具列表中。`alwaysLoad: true` 会将此工具的完整 schema 保留在初始提示词中，而不是延迟加载 |

#### `ToolAnnotations`

定义于 `@modelcontextprotocol/sdk/types.js`。所有字段都是可选的提示；客户端不应依赖它们做出安全决策。

| 字段 | 类型 | 默认值 | 描述 |
| :- | :- | :- | :- |
| `title` | `string` | `undefined` | 工具的人类可读标题 |
| `readOnlyHint` | `boolean` | `false` | 如果为 `true`，则该工具不会修改其环境 |
| `destructiveHint` | `boolean` | `true` | 如果为 `true`，则该工具可能执行破坏性更新。对于您标记为 [`always_ask`](#mcpservertoolpolicy) 的任何工具，请显式设置 `destructiveHint: true`，否则自动模式分类器可能会代替您的 `canUseTool` 回调批准该工具的调用。仅将 `destructiveHint` 保持未设置是不够的，因为 Claude Code 在处理 `always_ask` 调用时会将未设置的 `destructiveHint` 视为 `false`，尽管 MCP 规范中的默认值为 `true` |
| `idempotentHint` | `boolean` | `false` | 如果为 `true`，则使用相同参数重复调用不会产生额外效果（仅当 `readOnlyHint` 为 `false` 时有意义） |
| `openWorldHint` | `boolean` | `true` | 如果为 `true`，则该工具会与外部实体交互（例如网络搜索）。如果为 `false`，则该工具的作用域是封闭的（例如记忆工具） |

```typescript theme={null}
import { tool } from "@anthropic-ai/claude-agent-sdk";
import { z } from "zod";

const searchTool = tool(
  "search",
  "Search the web",
  { query: z.string() },
  async ({ query }) => {
    return { content: [{ type: "text", text: `Results for: ${query}` }] };
  },
  { annotations: { readOnlyHint: true, openWorldHint: true } }
);
```

### `createSdkMcpServer()`

创建一个与您的应用程序在同一进程中运行的 MCP 服务器实例。

```typescript theme={null}
function createSdkMcpServer(options: {
  name: string;
  version?: string;
  instructions?: string;
  tools?: Array<SdkMcpToolDefinition<any>>;
  alwaysLoad?: boolean;
  timeout?: number;
}): McpSdkServerConfigWithInstance;
```

#### 参数

| 参数 | 类型 | 描述 |
| :- | :- | :- |
| `options.name` | `string` | MCP 服务器的名称 |
| `options.version` | `string` | 可选的版本字符串 |
| `options.instructions` | `string` | 可选的服务器指令，由 `initialize` 返回，并作为 MCP 指令块呈现给模型 |
| `options.tools` | `Array<SdkMcpToolDefinition>` | 使用 [`tool()`](#tool) 创建的工具定义数组 |
| `options.alwaysLoad` | `boolean` | 为 `true` 时，此服务器的工具会保留在初始提示词中，而不会延迟到 [工具搜索](https://code.claude.com/docs/zh-CN/agent-sdk/tool-search.md) 之后加载。可与 [`tool()`](#tool) 中针对单个工具的 `alwaysLoad` 组合使用 |
| `options.timeout` | `number` | 此服务器工具调用的超时时间（毫秒）。Claude Code 会将其应用于此服务器，以取代 [`MCP_TOOL_TIMEOUT`](https://code.claude.com/docs/zh-CN/env-vars)。请传入不小于 1000 的整数。Claude Code 会忽略其他值。需要 TypeScript Agent SDK v0.3.248 或更高版本 |

### `listSessions()`

发现并列出过去的会话及其轻量元数据。可按项目目录筛选，或列出所有项目中的会话。

```typescript theme={null}
function listSessions(options?: ListSessionsOptions): Promise<SDKSessionInfo[]>;
```

#### 参数

| 参数 | 类型 | 默认值 | 描述 |
| :- | :- | :- | :- |
| `options.dir` | `string` | `undefined` | 要列出会话的目录。省略时返回所有项目中的会话 |
| `options.limit` | `number` | `undefined` | 要返回的最大会话数 |
| `options.includeWorktrees` | `boolean` | `true` | 当 `dir` 位于 git 仓库内时，包含来自所有 worktree 路径的会话 |

#### 返回类型：`SDKSessionInfo`

| 属性 | 类型 | 描述 |
| :- | :- | :- |
| `sessionId` | `string` | 唯一会话标识符（UUID） |
| `summary` | `string` | 显示标题：自定义标题、最近的提示词、自动生成的摘要或第一个提示词 |
| `lastModified` | `number` | 最后修改时间，自纪元以来的毫秒数 |
| `fileSize` | `number \| undefined` | 会话文件大小（字节）。仅在本地 JSONL 存储时填充 |
| `customTitle` | `string \| undefined` | 会话已设置的自定义标题，例如通过 `--name`、`/rename`、hook 的 `sessionTitle` 输出或 [`renameSession()`](#renamesession) 设置的标题。否则为 AI 生成的会话标题（如果会话有的话） |
| `firstPrompt` | `string \| undefined` | 会话中第一个有意义的用户提示词 |
| `gitBranch` | `string \| undefined` | 会话结束时的 Git 分支 |
| `cwd` | `string \| undefined` | 会话的工作目录 |
| `tag` | `string \| undefined` | 用户设置的会话标签（参见 [`tagSession()`](#tagsession)） |
| `createdAt` | `number \| undefined` | 创建时间，自纪元以来的毫秒数，取自第一个条目的时间戳 |

#### 示例

打印某个项目最近的 10 个会话。结果按 `lastModified` 降序排列，因此第一项是最新的。省略 `dir` 可在所有项目中搜索。

```typescript theme={null}
import { listSessions } from "@anthropic-ai/claude-agent-sdk";

const sessions = await listSessions({ dir: "/path/to/project", limit: 10 });

for (const session of sessions) {
  console.log(`${session.summary} (${session.sessionId})`);
}
```

### `getSessionMessages()`

从过去的会话记录中读取用户和助手消息。

```typescript theme={null}
function getSessionMessages(
  sessionId: string,
  options?: GetSessionMessagesOptions
): Promise<SessionMessage[]>;
```

#### 参数

| 参数 | 类型 | 默认值 | 描述 |
| :- | :- | :- | :- |
| `sessionId` | `string` | 必需 | 要读取的会话 UUID（参见 `listSessions()`） |
| `options.dir` | `string` | `undefined` | 用于查找会话的项目目录。省略时搜索所有项目 |
| `options.limit` | `number` | `undefined` | 要返回的最大消息数 |
| `options.offset` | `number` | `undefined` | 从开头跳过的消息数 |
| `options.includeSystemMessages` | `boolean` | `false` | 为 `true` 时，除用户和助手消息外，还返回系统消息，例如压缩边界 |

#### 返回类型：`SessionMessage`

| 属性 | 类型 | 描述 |
| :- | :- | :- |
| `type` | `"user" \| "assistant" \| "system"` | 消息角色。仅当 `options.includeSystemMessages` 为 `true` 时才会出现 `"system"` |
| `uuid` | `string` | 唯一消息标识符 |
| `session_id` | `string` | 此消息所属的会话 |
| `message` | `unknown` | 来自会话记录的原始消息负载 |
| `parent_tool_use_id` | `string \| null` | 对于子代理消息，为启动该子代理的 `Agent` 或 `Skill` 工具调用的 `tool_use_id`。对于主会话消息和较旧的会话为 `null` |
| `parent_agent_id` | `string \| null` | 对于来自 [嵌套子代理](https://code.claude.com/docs/zh-CN/sub-agents.md#let-subagents-spawn-their-own-subagents) 的消息，为生成它的子代理的 `agentId`。对于主会话消息、来自顶层子代理的消息以及较旧的会话为 `null`。需要 Claude Code v2.1.202 或更高版本 |

#### 示例

```typescript theme={null}
import { listSessions, getSessionMessages } from "@anthropic-ai/claude-agent-sdk";

const [latest] = await listSessions({ dir: "/path/to/project", limit: 1 });

if (latest) {
  const messages = await getSessionMessages(latest.sessionId, {
    dir: "/path/to/project",
    limit: 20
  });

  for (const msg of messages) {
    console.log(`[${msg.type}] ${msg.uuid}`);
  }
}
```

### `getSessionInfo()`

按 ID 读取单个会话的元数据，无需扫描整个项目目录。

```typescript theme={null}
function getSessionInfo(
  sessionId: string,
  options?: GetSessionInfoOptions
): Promise<SDKSessionInfo | undefined>;
```

#### 参数

| 参数 | 类型 | 默认值 | 描述 |
| :- | :- | :- | :- |
| `sessionId` | `string` | 必需 | 要查找的会话的 UUID |
| `options.dir` | `string` | `undefined` | 项目目录路径。省略时搜索所有项目目录 |

返回 [`SDKSessionInfo`](#return-type-sdksessioninfo)，如果未找到会话则返回 `undefined`。

### `renameSession()`

通过追加一个自定义标题条目来重命名会话。重复调用是安全的；以最近的标题为准。

```typescript theme={null}
function renameSession(
  sessionId: string,
  title: string,
  options?: SessionMutationOptions
): Promise<void>;
```

#### 参数

| 参数 | 类型 | 默认值 | 描述 |
| :- | :- | :- | :- |
| `sessionId` | `string` | 必需 | 要重命名的会话的 UUID |
| `title` | `string` | 必需 | 新标题。去除首尾空白后不得为空 |
| `options.dir` | `string` | `undefined` | 项目目录路径。省略时搜索所有项目目录 |

### `tagSession()`

为会话添加标签。传入 `null` 可清除标签。重复调用是安全的；以最近的标签为准。

```typescript theme={null}
function tagSession(
  sessionId: string,
  tag: string | null,
  options?: SessionMutationOptions
): Promise<void>;
```

#### 参数

| 参数 | 类型 | 默认值 | 描述 |
| :- | :- | :- | :- |
| `sessionId` | `string` | 必需 | 要添加标签的会话的 UUID |
| `tag` | `string \| null` | 必需 | 标签字符串，或传入 `null` 以清除 |
| `options.dir` | `string` | `undefined` | 项目目录路径。省略时搜索所有项目目录 |

### `forkSession()`

将会话的会话记录复制到一个新会话中，以便您在保持原始会话不变的同时，将对话引向另一个方向。要从对话中较早的某个位置分叉，请传入 `upToMessageId`。

```typescript theme={null}
function forkSession(
  sessionId: string,
  options?: ForkSessionOptions
): Promise<ForkSessionResult>;
```

#### 参数

| 参数 | 类型 | 默认值 | 描述 |
| :- | :- | :- | :- |
| `sessionId` | `string` | 必需 | 要分叉的会话的 UUID |
| `options.dir` | `string` | `undefined` | 项目目录路径。省略时搜索所有项目目录 |
| `options.upToMessageId` | `string` | `undefined` | 复制会话记录，直到并包括具有此 `uuid` 的消息：该值可以来自 [`getSessionMessages()`](#getsessionmessages)，也可以是您在流式传入的 [`SDKUserMessage`](#sdkusermessage) 上设置的 `uuid`。省略时复制整个会话记录 |
| `options.title` | `string` | `undefined` | 分叉的标题。省略时，SDK 会根据原始会话生成一个标题，并在其后附加 `(fork)` |

返回 `{ sessionId }`，即新会话的 UUID。将其作为 [`resume`](#options) 传入即可继续该分叉。分叉不包含原始会话的 [文件检查点](https://code.claude.com/docs/zh-CN/agent-sdk/file-checkpointing.md)，因此您无法将其回退到分叉之前捕获的检查点。

在以下情况下，`forkSession()` 会抛出错误：

* `sessionId` 不是 UUID
* 找不到该会话，或该会话没有消息
* `upToMessageId` 与会话记录中的任何消息都不匹配

### `resolveSettings()`

使用与 CLI 相同的合并引擎，解析给定目录的有效 Claude Code 设置，而无需启动 Claude CLI。可在调用 `query()` 之前，用它检查该调用将会看到哪些配置。

> [!NOTE]
>
> 此函数处于 alpha 阶段，其 API 在稳定之前可能会发生变化。

该快照与实时 `query()` 会话实际应用的内容存在以下差异：

* **`policyHelper`**：`resolveSettings()` 会读取 MDM 来源，包括 macOS plist 和 Windows HKLM/HKCU，但不会执行管理员配置的 `policyHelper` 子进程。
* **服务器托管设置**：`resolveSettings()` 不会获取 [服务器托管设置](https://code.claude.com/docs/zh-CN/server-managed-settings.md#fetch-and-caching-behavior)。如需包含这些设置，请将其作为 `options.serverManagedSettings` 传入。
* **`defaultMode`**：快照会按原样返回每个层级中的 `permissions.defaultMode`，因此其中可能包含来自项目设置和本地设置的 `'auto'` 和 `'bypassPermissions'` 值，而 [实时会话会忽略这些值](https://code.claude.com/docs/zh-CN/permission-modes.md#which-mode-a-session-starts-in)。

```typescript theme={null}
function resolveSettings(
  options?: ResolveSettingsOptions
): Promise<ResolvedSettings>;
```

#### 参数

`resolveSettings()` 接受单个选项对象。所有字段都是可选的。

| 参数 | 类型 | 默认值 | 描述 |
| :- | :- | :- | :- |
| `options.cwd` | `string` | `process.cwd()` | 解析项目设置和本地设置时所相对的目录 |
| `options.settingSources` | [`SettingSource`](#settingsource)`[]` | 所有来源 | 要加载哪些文件系统来源。传入 `[]` 可跳过用户设置、项目设置和本地设置。[端点托管策略](https://code.claude.com/docs/zh-CN/managed-settings.md#delivery-mechanisms) 在所有情况下都会加载。仅当您传入 `options.serverManagedSettings` 时，`resolveSettings()` 才会包含服务器托管设置 |
| `options.managedSettings` | `Settings` | `undefined` | 由嵌入宿主提供的策略层级设置。遵循与 [`Options` 中的 `managedSettings`](#options) 相同的规则，不同之处在于 `resolveSettings()` 不会执行已配置的 [`policyHelper`](https://code.claude.com/docs/zh-CN/settings-reference#policyhelper)，因此快照中可能包含实时会话会丢弃的设置 |
| `options.serverManagedSettings` | `Settings` | `undefined` | 来自 `/api/claude_code/settings` 的服务器托管设置负载。非限制性键将不经过滤直接传递 |

#### 返回类型：`ResolvedSettings`

`resolveSettings()` 返回一个对象，描述合并后的设置以及提供每个键的来源。

| 属性 | 类型 | 描述 |
| :- | :- | :- |
| `effective` | `Settings` | 按优先级顺序应用所有已启用来源后的合并设置 |
| `provenance` | `Partial<Record<keyof Settings, ProvenanceEntry>>` | 对于 `effective` 中的每个顶层键，指明由哪个来源提供了该值 |
| `sources` | `Array<{ source, settings, path?, policyOrigin? }>` | 每个来源的原始设置，按优先级从低到高排列 |

#### 示例

下面的示例解析某个项目目录的设置，并打印控制清理周期的来源。在没有任何设置文件设置 `cleanupPeriodDays` 的机器上，打印的两行中该值都显示为 `undefined`，这是预期的输出，而不是错误。

```typescript theme={null}
import { resolveSettings } from "@anthropic-ai/claude-agent-sdk";

const { effective, provenance } = await resolveSettings({
  cwd: "/path/to/project",
  settingSources: ["user", "project", "local"],
});

console.log(`Cleanup period: ${effective.cleanupPeriodDays} days`);
console.log(`Set by: ${provenance.cleanupPeriodDays?.source}`);
```

## 类型

### `Options`

`query()` 函数的配置对象。

| 属性 | 类型 | 默认值 | 描述 |
| :- | :- | :- | :- |
| `abortController` | `AbortController` | `new AbortController()` | 用于取消操作的控制器 |
| `additionalDirectories` | `string[]` | `[]` | Claude 可以访问的其他目录。SDK 会将每个条目作为 `--add-dir` 传递给 Claude Code，因此在使用 `project` 设置源时，Claude Code 还会[加载该目录的 skill、命令和子代理](https://code.claude.com/docs/zh-CN/permissions.md#additional-directories-grant-file-access-not-configuration) |
| `agent` | `string` | `undefined` | 主线程的 Agent 名称。该 Agent 必须在 `agents` 选项或设置中定义 |
| `agents` | `Record<string, [`AgentDefinition`](#agentdefinition)>` | `undefined` | 以编程方式定义子代理 |
| `agentProgressSummaries` | `boolean` | `false` | 为 `true` 时，为子代理生成单行进度摘要，并通过 `summary` 字段在 [`task_progress`](#sdktaskprogressmessage) 事件中转发。适用于前台和后台子代理 |
| `allowDangerouslySkipPermissions` | `boolean` | `false` | 启用绕过权限。使用 `permissionMode: 'bypassPermissions'` 时必需，无论是在启动时还是之后通过 `setPermissionMode()` 设置。有关它与 `permissionMode: 'plan'` 的交互方式，请参阅[计划模式](https://code.claude.com/docs/zh-CN/agent-sdk/permissions.md#plan-mode-plan) |
| `allowedTools` | `string[]` | `[]` | 无需提示即可自动批准的工具，从[网络路径](https://code.claude.com/docs/zh-CN/permissions.md#network-paths)读取的操作除外。这并不会将 Claude 限制为仅使用这些工具。如果您在此处指定了某个[任务跟踪工具](https://code.claude.com/docs/zh-CN/agent-sdk/todo-tracking.md#model-availability)，Claude Code 也会为该会话启用它。其他未列出的工具将交由 `permissionMode` 和 `canUseTool` 处理。使用 `disallowedTools` 来阻止工具。请参阅[权限](https://code.claude.com/docs/zh-CN/agent-sdk/permissions.md#allow-and-deny-rules) |
| `betas` | [`SdkBeta`](#sdkbeta)`[]` | `[]` | 启用 beta 功能 |
| `canUseTool` | [`CanUseTool`](#canusetool) | `undefined` | 自定义权限函数，仅在[权限流程](https://code.claude.com/docs/zh-CN/agent-sdk/permissions.md#how-permissions-are-evaluated)落到提示环节时调用。对于由 `allowedTools`、允许规则或 `permissionMode` 自动批准的调用，不会调用此函数。允许规则不会预先批准[任何模式都不会自动批准的操作](https://code.claude.com/docs/zh-CN/permission-modes.md#actions-no-mode-auto-approves)。详情请参阅 [`CanUseTool`](#canusetool) |
| `continue` | `boolean` | `false` | 继续最近的对话 |
| `cwd` | `string` | `process.cwd()` | 当前工作目录 |
| `debug` | `boolean` | `false` | 为 Claude Code 进程启用调试模式 |
| `debugFile` | `string` | `undefined` | 将调试日志写入指定的文件路径。会隐式启用调试模式 |
| `disallowedTools` | `string[]` | `[]` | 要拒绝的工具。像 `"Bash"` 这样的裸名称会将该工具从 Claude 的上下文中移除。像 `"Bash(rm *)"` 这样的限定规则会保留该工具可用，但在所有权限模式（包括 `bypassPermissions`）下拒绝匹配的调用，匹配基于[命令的书写形式](https://code.claude.com/docs/zh-CN/permissions.md#bash-rule-limits)。请参阅[权限](https://code.claude.com/docs/zh-CN/agent-sdk/permissions.md#allow-and-deny-rules) |
| `effort` | `'low' \| 'medium' \| 'high' \| 'xhigh' \| 'max'` | `undefined` | 控制 Claude 在回复中投入的 effort。与自适应思考配合使用以引导思考深度。请参阅[调整 effort 级别](https://code.claude.com/docs/zh-CN/model-config.md#adjust-effort-level) |
| `enableFileCheckpointing` | `boolean` | `false` | 启用文件更改跟踪以便回退。请参阅[文件检查点功能](https://code.claude.com/docs/zh-CN/agent-sdk/file-checkpointing.md) |
| `env` | `Record<string, string \| undefined>` | `process.env` | 环境变量。设置后，它会替换子进程环境，而不是与 `process.env` 合并，因此请传递 `{ ...process.env, YOUR_VAR: 'value' }` 以保留 `PATH` 等继承的变量。有关此模式的示例，请参阅[处理缓慢或停滞的 API 响应](#handle-slow-or-stalled-api-responses)；有关底层 CLI 读取的变量，请参阅[环境变量](https://code.claude.com/docs/zh-CN/env-vars.md)。设置 `CLAUDE_AGENT_SDK_CLIENT_APP` 可在 User-Agent 标头中标识您的应用 |
| `executable` | `'bun' \| 'deno' \| 'node'` | 自动检测 | 要使用的 JavaScript 运行时 |
| `executableArgs` | `string[]` | `[]` | 传递给可执行文件的参数 |
| `extraArgs` | `Record<string, string \| null>` | `{}` | 附加参数 |
| `fallbackModel` | `string` | `undefined` | 主模型失败时使用的模型。接受以逗号分隔的列表。有关顺序和上限，请参阅[备用模型链](https://code.claude.com/docs/zh-CN/model-config.md#fallback-model-chains)。有关指导，请参阅[选择模型](https://code.claude.com/docs/zh-CN/agent-sdk/configuration.md#choose-a-model) |
| `forkSession` | `boolean` | `false` | 使用 `resume` 恢复时，分叉到新的会话 ID，而不是继续原始会话 |
| `forwardSubagentText` | `boolean` | `false` | 将子代理的文本和思考块作为设置了 `parent_tool_use_id` 的 assistant 和 user 消息转发，以便使用方渲染嵌套的会话记录。如果不使用此选项，Claude Code 会省略在[前台](https://code.claude.com/docs/zh-CN/sub-agents.md#run-subagents-in-foreground-or-background)运行的子代理的文本和思考块。有关嵌套子代理、带有 `context: fork` 的 skill 以及各自所需的 Claude Code 版本，请参阅[跟踪子代理消息](https://code.claude.com/docs/zh-CN/headless.md#follow-subagent-messages) |
| `hooks` | `Partial<Record<`[`HookEvent`](#hookevent)`, `[`HookCallbackMatcher`](#hookcallbackmatcher)`[]>>` | `{}` | 事件的 hook 回调 |
| `includeHookEvents` | `boolean` | `false` | 在消息流中以 [`SDKHookStartedMessage`](#sdkhookstartedmessage)、[`SDKHookProgressMessage`](#sdkhookprogressmessage) 和 [`SDKHookResponseMessage`](#sdkhookresponsemessage) 的形式包含 hook 生命周期事件。`SessionStart` 和 `Setup` hook 的生命周期事件始终会包含，无需此选项。某些 hook 事件（例如 `Notification`、`SessionEnd`、`PreCompact` 和 `PostCompact`）即使启用此选项也永远不会产生 `SDKHookStartedMessage`。对于这些事件，当运行超过一秒的命令 hook 产生输出时，Claude Code 仍会发出 `SDKHookProgressMessage`，并且仅在[在后台运行的](https://code.claude.com/docs/zh-CN/hooks.md#run-hooks-in-the-background) hook 完成时才发出 `SDKHookResponseMessage` |
| `includePartialMessages` | `boolean` | `false` | 包含部分消息事件 |
| `loadTimeoutMs` | `number` | `60000` | *Alpha。* 在恢复物化期间，每次 `sessionStore.load()` 和 `sessionStore.listSubkeys()` 调用的超时时间（毫秒）。如果适配器在此时间窗口内未完成，查询将失败而不是挂起。未设置 `sessionStore` 时忽略 |
| `managedSettings` | `Settings` | `undefined` | 由您的宿主进程提供给所生成会话的策略层级设置。在部署了管理员托管设置的机器上，除非管理员优先级最高的托管源设置了 `parentSettingsBehavior: 'merge'`，否则 Claude Code 会忽略这些设置；并且当 [`policyHelper`](https://code.claude.com/docs/zh-CN/settings-reference#policyhelper) 提供托管设置时，永远不会合并它们。合并的值会经过一个仅限收紧的过滤器；[限制父级设置](https://code.claude.com/docs/zh-CN/claude-apps-gateway#restrict-parent-settings)介绍了过滤器允许的内容以及 `allowManaged*Only` 锁定。设置了 [`CLAUDE_CODE_PROVIDER_MANAGED_BY_HOST`](https://code.claude.com/docs/zh-CN/env-vars) 的宿主会改为直接从此负载中读取三个键：在 Claude Code v2.1.222 或更高版本中读取其[模型配置](https://code.claude.com/docs/zh-CN/model-config.md#restrict-model-selection)；在 v2.1.246 或更高版本中，当没有托管源设置 [`modelPricing`](https://code.claude.com/docs/zh-CN/settings-reference#modelpricing) 时读取该键；在 v2.1.247 或更高版本中读取其 `ENABLE_TOOL_SEARCH` env 条目 |
| `maxBudgetUsd` | `number` | `undefined` | 当客户端成本估算达到此美元值时停止查询。估算值可能超过此值，因此请[留出余量](https://code.claude.com/docs/zh-CN/agent-sdk/agent-loop.md#budget-headroom)。仅计算本次调用自身的花费；从恢复的会话中还原的总计不计入。有关准确性注意事项和重置行为，请参阅[跟踪成本和用量](https://code.claude.com/docs/zh-CN/agent-sdk/cost-tracking.md) |
| `maxThinkingTokens` | `number` | `undefined` | *已弃用：* 请改用 `thinking`。思考过程的最大 token 数 |
| `maxTurns` | `number` | `undefined` | 最大 Agent [轮次](https://code.claude.com/docs/zh-CN/agent-sdk/agent-loop.md#turns-and-messages) |
| `mcpServers` | `Record<string, [`McpServerConfig`](#mcpserverconfig)>` | `{}` | MCP 服务器配置 |
| `model` | `string` | 来自 CLI 的默认值 | Claude 模型别名或完整模型名称。请参阅[可接受的值和特定于提供商的 ID](https://code.claude.com/docs/zh-CN/model-config.md#available-models) |
| `onElicitation` | `(request: ElicitationRequest, options: { signal: AbortSignal }) => Promise<ElicitationResult>` | `undefined` | 用于处理 MCP elicitation 请求的回调。当 MCP 服务器请求用户输入且没有 hook 先行处理时调用。未提供时，未处理的 elicitation 请求会被自动拒绝 |
| `outputFormat` | `{ type: 'json_schema', schema: JSONSchema }` | `undefined` | 定义 Agent 结果的输出格式。详情请参阅[结构化输出](https://code.claude.com/docs/zh-CN/agent-sdk/structured-outputs.md) |
| `outputStyle` | `string` | `undefined` | 不是 `Options` 字段。请改为在内联 [`settings`](https://code.claude.com/docs/zh-CN/settings) 对象或设置文件中设置 `outputStyle`。请参阅[激活输出样式](https://code.claude.com/docs/zh-CN/agent-sdk/modifying-system-prompts.md#activate-an-output-style) |
| `pathToClaudeCodeExecutable` | `string` | 从捆绑的原生二进制文件自动解析 | Claude Code 可执行文件的路径。仅当安装期间跳过了可选依赖或您的平台不在支持范围内时才需要 |
| `permissionMode` | [`PermissionMode`](#permissionmode) | `undefined` | 会话的权限模式。如果省略它且没有其他内容设置权限模式（例如 [`permissions.defaultMode`](https://code.claude.com/docs/zh-CN/settings-reference#permissions-defaultmode) 设置），会话将以 `default` 启动。请参阅[权限模式](https://code.claude.com/docs/zh-CN/agent-sdk/permissions.md#permission-modes) |
| `permissionPromptToolName` | `string` | `undefined` | 用于权限提示的 MCP 工具名称 |
| `permissionPrompts` | `'host' \| 'none'` | `'host'` | 由谁回答权限提示：`'host'` 将其路由到您的 [`canUseTool`](#canusetool) 回调或 `permissionPromptToolName` 工具，`'none'` 则[拒绝本应触发提示的调用](https://code.claude.com/docs/zh-CN/agent-sdk/permissions.md#how-permissions-are-evaluated)。需要 Claude Code v2.1.259 或更高版本 |
| `persistSession` | `boolean` | `true` | 为 `false` 时，禁用会话持久化到磁盘。会话之后将无法恢复 |
| `planModeInstructions` | `string` | `undefined` | 计划模式的自定义工作流指令。当 `permissionMode` 为 `'plan'` 时，此字符串会替换默认的计划模式工作流正文。CLI 仍会用只读强制前言和 ExitPlanMode 协议尾注将其包裹 |
| `plugins` | [`SdkPluginConfig`](#sdkpluginconfig)`[]` | `[]` | 从本地路径加载自定义插件。详情请参阅[插件](https://code.claude.com/docs/zh-CN/agent-sdk/plugins.md) |
| `projectConfigRoot` | `string` | `undefined` | `cwd` 作为其 worktree 的受信任检出目录的绝对路径。Claude Code 会从此目录而不是 `cwd` 读取项目设置、`.mcp.json` 以及项目 `.claude/` 中的命令、Agent、skill、工作流、Routine 和输出样式，并将 `CLAUDE_PROJECT_DIR` 设置为此目录。hook、`apiKeyHelper` 等辅助脚本以及 stdio MCP 服务器会以此目录作为工作目录启动。`CLAUDE.md` 文件和 `.claude/rules/` 仍从 `cwd` 加载。需要 Claude Code v2.1.275 或更高版本 |
| `promptSuggestions` | `boolean` | `false` | 启用提示词建议。每轮结束后，Claude Code 会发出一条 `prompt_suggestion` 消息，其中包含预测的下一条用户提示词。对于某些轮次，Claude Code 不会生成建议，例如当您的账户接近或已达到用量限制时。请参阅[Claude Code 何时跳过建议](https://code.claude.com/docs/zh-CN/interactive-mode.md#when-claude-code-skips-suggestions) |
| `resume` | `string` | `undefined` | 要恢复的会话 ID |
| `resumeDropsTurn` | `string` | `undefined` | 与 `resumeSessionAt` 配合使用：截断式恢复打算丢弃的轮次的提示词 UUID。当被丢弃的范围包含任何无法归属于该轮次的内容（例如已吸收的排队消息或任务通知）时，Claude Code 会拒绝恢复，并在拒绝消息中指明 `--resume-drops-turn` 标志。只有 Agent SDK 和 print 模式的恢复会读取这对参数。需要 Claude Code v2.1.223 或更高版本 |
| `resumeSessionAt` | `string` | `undefined` | 在指定的消息 UUID 处恢复会话 |
| `sandbox` | [`SandboxSettings`](#sandboxsettings) | `undefined` | 以编程方式配置沙箱行为。详情请参阅[沙箱设置](#sandboxsettings) |
| `sessionId` | `string` | 自动生成 | 为会话使用指定的 UUID，而不是自动生成 |
| `sessionStore` | [`SessionStore`](https://code.claude.com/docs/zh-CN/agent-sdk/session-storage#the-sessionstore-interface) | `undefined` | 将会话记录镜像到外部后端，以便其他主机可以恢复它们。请参阅[将会话持久化到外部存储](https://code.claude.com/docs/zh-CN/agent-sdk/session-storage.md) |
| `sessionStoreFlush` | `'batched' \| 'eager'` | `'batched'` | *Alpha。* `sessionStore` 的刷新模式。未设置 `sessionStore` 时忽略 |
| `settings` | `string \| Settings` | `undefined` | 内联[设置](https://code.claude.com/docs/zh-CN/settings.md)对象、设置文件路径或内联 JSON 字符串。填充[优先级顺序](https://code.claude.com/docs/zh-CN/settings.md#settings-precedence)中的标志设置层。可在运行时通过 [`applyFlagSettings()`](#applyflagsettings) 更改 |
| `settingSources` | [`SettingSource`](#settingsource)`[]` | CLI 默认值（所有来源） | 控制要加载哪些文件系统设置。传递 `[]` 可禁用用户、项目和本地设置。[端点托管策略](https://code.claude.com/docs/zh-CN/managed-settings.md#delivery-mechanisms)无论如何都会加载；当会话在[符合条件的配置](https://code.claude.com/docs/zh-CN/server-managed-settings.md#platform-availability)上使用组织凭据进行身份验证时，会获取服务器托管设置。请参阅[使用 Claude Code 功能](https://code.claude.com/docs/zh-CN/agent-sdk/claude-code-features.md#what-settingsources-does-not-control) |
| `skills` | `string[] \| 'all'` | `undefined` | 会话可用的 skill。传递 `'all'` 以启用所有发现的 skill，或传递 skill 名称列表。仅传递精确名称。在 Agent SDK v0.3.221 或更高版本中，SDK 会在启动 Claude Code 进程之前以错误拒绝格式错误和通配符形式的名称。设置后，SDK 会自动将 Skill 工具添加到 `allowedTools`。如果您同时传递了 `tools`，请在该列表中包含 `'Skill'`。请参阅 [Skills](https://code.claude.com/docs/zh-CN/agent-sdk/skills.md) |
| `spawnClaudeCodeProcess` | `(options: SpawnOptions) => SpawnedProcess` | `undefined` | 用于生成 Claude Code 进程的自定义函数。用于在虚拟机、容器或远程环境中运行 Claude Code |
| `stderr` | `(data: string) => void` | `undefined` | stderr 输出的回调 |
| `strictMcpConfig` | `boolean` | `false` | 仅使用通过 `mcpServers` 传递的服务器，忽略项目 `.mcp.json`、用户设置、插件提供的 MCP 服务器以及 [claude.ai 连接器](https://code.claude.com/docs/zh-CN/mcp.md#use-mcp-servers-from-claude-ai) |
| `systemPrompt` | `string \| string[] \| { type: 'custom'; prompt: string \| string[]; snapshot?: boolean } \| { type: 'preset'; preset: 'claude_code'; append?: string; excludeDynamicSections?: boolean; snapshot?: boolean }` | `undefined`（最简提示词） | 系统提示词配置。传递字符串以使用自定义提示词，或传递 `{ type: 'preset', preset: 'claude_code' }` 以使用 Claude Code 的系统提示词。传递字符串数组，并在静态部分与每请求部分之间放置导出的 `SYSTEM_PROMPT_DYNAMIC_BOUNDARY` 常量，即可[缓存自定义提示词的静态部分](https://code.claude.com/docs/zh-CN/agent-sdk/modifying-system-prompts.md#cache-the-static-part-of-a-custom-prompt)。使用预设对象形式时，添加 `append` 以附加额外指令进行扩展，并设置 `excludeDynamicSections: true` 将每会话上下文移到第一条用户消息中，以[在不同机器间更好地复用提示缓存](https://code.claude.com/docs/zh-CN/agent-sdk/modifying-system-prompts.md#improve-prompt-caching-across-users-and-machines)。设置 `snapshot: false` 可在每次请求时重新构建提示词，而不是[复用会话在首次请求时记录的提示词](https://code.claude.com/docs/zh-CN/agent-sdk/modifying-system-prompts.md#change-the-prompt-of-an-existing-session)。要在自定义提示词上设置 `snapshot`，请使用 `{ type: 'custom', prompt }` 形式。`{ type: 'custom' }` 形式和 `snapshot` 字段需要 TypeScript Agent SDK v0.3.257 或更高版本 |
| `taskBudget` | `{ total: number }` | `undefined` | *Alpha。* API 端的任务预算（以 token 计）。设置后，模型会被告知其剩余的 token 预算，以便控制工具使用节奏并在达到限制前收尾 |
| `thinking` | [`ThinkingConfig`](#thinkingconfig) | 对支持的模型为 `{ type: 'adaptive' }` | 控制 Claude 的思考/推理行为。有关选项，请参阅 [`ThinkingConfig`](#thinkingconfig) |
| `title` | `string` | `undefined` | 会话的显示标题。通过 `resume` 或 `continue` 恢复时，被恢复会话已持久化的标题优先；使用 [`renameSession()`](#renamesession) 为现有会话重新命名 |
| `toolAliases` | `Record<string, string>` | `undefined` | 将内置工具名称映射到 MCP 工具名称，使 Claude 调用您的 MCP 实现来代替内置工具。例如 `{ Bash: 'mcp__workspace__bash' }` |
| `toolConfig` | [`ToolConfig`](#toolconfig) | `undefined` | 内置工具行为的配置。详情请参阅 [`ToolConfig`](#toolconfig) |
| `tools` | `string[] \| { type: 'preset'; preset: 'claude_code' }` | `undefined` | 工具配置。传递工具名称数组，或使用预设以获取 Claude Code 的默认工具 |
| `verbatimPrompts` | `boolean` | `false` | 按原样传递每条提示词。SDK 会以 `client_composed: true` 发送每条用户消息。有关 Claude Code 对这些消息跳过的处理，请参阅 [`client_composed`](#sdkusermessage)。当您的提示词文本包含并非最终用户输入的内容时，请使用此选项。如需按轮次控制，请保持其关闭，并改为在单条流式消息上设置 `client_composed`。需要 TypeScript Agent SDK v0.3.280 或更高版本以及 Claude Code v2.1.248 或更高版本；这些 SDK 版本捆绑的 Claude Code 版本满足 Claude Code 的版本要求 |

#### 处理缓慢或停滞的 API 响应

CLI 子进程会读取若干控制 API 超时和停滞检测的环境变量。请通过 `env` 选项传递它们：

```typescript theme={null}
import { query } from "@anthropic-ai/claude-agent-sdk";

const result = query({
  prompt: "Analyze this code",
  options: {
    env: {
      ...process.env,
      API_TIMEOUT_MS: "120000",
      CLAUDE_CODE_MAX_RETRIES: "2",
      CLAUDE_ASYNC_AGENT_STALL_TIMEOUT_MS: "120000",
    },
  },
});
```

* [`API_TIMEOUT_MS`](https://code.claude.com/docs/zh-CN/env-vars)：每次请求尝试等待响应标头的时长（毫秒）。它不限制标头到达后流式响应持续的时间。默认值为 `600000`。适用于主循环和所有子代理。
* `CLAUDE_CODE_MAX_RETRIES`：最大 API 重试次数。默认值为 `10`，上限为 `15`。每次重试都有各自的 `API_TIMEOUT_MS` 时间窗口。

  对于需要挺过较长中断的无人值守运行，请设置 [`CLAUDE_CODE_RETRY_WATCHDOG=1`](https://code.claude.com/docs/zh-CN/errors#tune-retry-behavior)：它会无限期重试临时性容量错误，并且在 Claude Code v2.1.199 或更高版本上，会将其他临时性错误的默认重试次数提高到 `300`，并取消此变量的上限。
* `CLAUDE_ASYNC_AGENT_STALL_TIMEOUT_MS`：子代理的停滞看门狗。当流看门狗开启时，默认值为 `CLAUDE_STREAM_IDLE_TIMEOUT_MS` 加 5 分钟，除非您调高该变量，否则合计为 `600000`。当流看门狗关闭时，默认值为 `600000`。在 v2.1.257 之前，默认值始终为 `600000`。

  每个流事件都会重置计时器。发生停滞时，Claude Code 会中止该子代理并向父级报告停滞。对于后台子代理，它还会将该任务标记为失败并附上任何部分结果。
* `CLAUDE_ENABLE_STREAM_WATCHDOG` 与 `CLAUDE_STREAM_IDLE_TIMEOUT_MS`：流看门狗，在响应标头已到达但响应体停止流式传输时中止请求。看门狗对所有提供商默认开启；设置 `CLAUDE_ENABLE_STREAM_WATCHDOG=0` 可将其禁用。`CLAUDE_STREAM_IDLE_TIMEOUT_MS` 默认值为 `300000`，且最小值被限制为该值。中止之后，Claude Code 会根据响应已进行到的程度采取相应操作，详见[自动重试](https://code.claude.com/docs/zh-CN/errors.md#automatic-retries)。

  当看门狗等待一个由 `ANTHROPIC_BASE_URL` 背后的网关通过 keep-alive ping 保持打开的响应时，设置了 `includePartialMessages` 的宿主会持续收到 `ping` [流事件](#sdkpartialassistantmessage)，因此请将这些帧视为存活信号，而不是因为静默而使会话超时。在 v2.1.257 之前，这些帧会在最后一个真实流事件 5 分钟后停止。

### `Query` 对象

`query()` 函数返回的接口。

```typescript theme={null}
interface Query extends AsyncGenerator<SDKMessage, void> {
  interrupt(): Promise<SDKControlInterruptResponse | undefined>;
  rewindFiles(
    userMessageId: string,
    options?: { dryRun?: boolean }
  ): Promise<RewindFilesResult>;
  setPermissionMode(mode: PermissionMode): Promise<void>;
  setModel(model?: string): Promise<void>;
  setMaxThinkingTokens(maxThinkingTokens: number | null): Promise<void>;
  applyFlagSettings(settings: {
    [K in keyof Settings]?: K extends 'effortLevel'
      ? 'low' | 'medium' | 'high' | 'xhigh' | 'max' | null
      : Settings[K] | null;
  }): Promise<void>;
  updateSettings(
    source: 'localSettings' | 'userSettings',
    settings: Record<string, unknown>,
  ): Promise<void>;
  initializationResult(): Promise<SDKControlInitializeResponse>;
  reinitialize(): Promise<SDKControlInitializeResponse>;
  supportedCommands(): Promise<SlashCommand[]>;
  supportedModels(): Promise<ModelInfo[]>;
  supportedAgents(): Promise<AgentInfo[]>;
  mcpServerStatus(): Promise<McpServerStatus[]>;
  getContextUsage(opts?: {
    detail?: 'summary' | 'full';
  }): Promise<SDKControlGetContextUsageResponse>;
  readFile(
    path: string,
    options?: { maxBytes?: number; encoding?: 'utf-8' | 'base64' }
  ): Promise<SDKControlReadFileResponse | null>;
  reloadPlugins(options?: {
    holdOnCacheImpact?: boolean;
    addPlugins?: SdkPluginConfig[];
  }): Promise<SDKControlReloadPluginsResponse>;
  reloadSkills(): Promise<SDKControlReloadSkillsResponse>;
  reloadOutputStyles(): Promise<SDKControlReloadOutputStylesResponse>;
  seedReadState(path: string, mtime: number): Promise<void>;
  accountInfo(): Promise<AccountInfo>;
  reconnectMcpServer(serverName: string): Promise<void>;
  toggleMcpServer(serverName: string, enabled: boolean): Promise<void>;
  setMcpServers(servers: Record<string, McpServerConfig>): Promise<McpSetServersResult>;
  setMcpPermissionModeOverride(
    serverName: string,
    mode: 'default' | 'auto' | null
  ): Promise<{ warning?: string }>;
  readMcpResource(serverName: string, uri: string): Promise<SDKControlMcpReadResourceResponse>;
  streamInput(stream: AsyncIterable<SDKUserMessage>): Promise<void>;
  stopTask(taskId: string): Promise<void>;
  backgroundTasks(toolUseId?: string): Promise<boolean>;
  close(): void;
}
```

#### 方法

| 方法 | 描述 |
| :- | :- |
| `interrupt()` | 中断查询。仅在流式输入模式下可用。当 CLI 在 [`SDKSystemMessage.capabilities`](#sdksystemmessage) 中声明了 `interrupt_receipt_v1` 能力时，会以一个 [`SDKControlInterruptResponse`](#sdkcontrolinterruptresponse) 完成，其中列出中断到达时仍处于待处理状态的消息。在 v2.1.205 之前的 CLI 上以 `undefined` 完成 |
| `rewindFiles(userMessageId, options?)` | 将文件恢复到指定用户消息时的状态。传递 `{ dryRun: true }` 可预览更改。需要 `enableFileCheckpointing: true`。请参阅[文件检查点功能](https://code.claude.com/docs/zh-CN/agent-sdk/file-checkpointing.md) |
| `setPermissionMode()` | 更改权限模式（仅在流式输入模式下可用） |
| `setModel()` | 更改模型（仅在流式输入模式下可用）。传递 `undefined` 或字符串 `"default"` 会重置为 [Claude Code 的默认模型](https://code.claude.com/docs/zh-CN/model-config.md)。如果新模型不支持自动模式，处于 `auto` 模式的会话会[切换到 `default` 模式](https://code.claude.com/docs/zh-CN/agent-sdk/permissions#auto-mode-auto) |
| `setMaxThinkingTokens()` | *已弃用：* 请改用 `thinking` 选项。更改最大思考 token 数。传递 `null` 会将思考重置为会话默认值：会话中途的覆盖会被清除，对于禁用了思考的会话，思考保持关闭 |
| `applyFlagSettings(settings)` | 在运行时将设置合并到会话的标志设置层（仅在流式输入模式下可用）。请参阅 [`applyFlagSettings()`](#applyflagsettings) |
| `updateSettings(source, settings)` | 将一个允许列表中的键写入项目的本地设置文件或您的用户设置文件，使该值在之后的会话中保持有效。请参阅 [`updateSettings()`](#updatesettings)。需要 TypeScript SDK v0.3.257 或更高版本，其捆绑了 Claude Code v2.1.257 |
| `initializationResult()` | 返回完整的初始化结果，包括支持的命令、模型、账户信息和输出样式配置 |
| `reinitialize()` | 向正在运行的 CLI 重新发送 `initialize` 控制请求，并返回新的结果，而不是首次连接时缓存的结果。在传输中断之后（例如断开连接后重新附加到会话）使用它，以便待处理的权限请求再次到达您的 `canUseTool` 回调。请让回调对每个请求 ID 保持幂等，因为响应丢失的请求会被再次分派。需要 Claude Code v2.1.195 或更高版本 |
| `supportedCommands()` | 返回可用的命令。从 Agent SDK v0.3.216 起，该列表会反映会话中途的命令变更；请参阅 [`SDKCommandsChangedMessage`](#sdkcommandschangedmessage) |
| `supportedModels()` | 返回可用的模型及其显示信息 |
| `supportedAgents()` | 以 [`AgentInfo`](#agentinfo)`[]` 形式返回可用的子代理 |
| `mcpServerStatus()` | 以 [`McpServerStatus`](#mcpserverstatus)`[]` 形式返回已连接 MCP 服务器的状态 |
| `getContextUsage(opts?)` | 返回一个 [`SDKControlGetContextUsageResponse`](#sdkcontrolgetcontextusageresponse)，按类别、skill 和工具细分会话的上下文窗口使用情况。使用默认的 `detail` 时，它包含 `/context` 在交互式会话中显示的数据，这些数据通过不会出现在消息流中的 token 计数 API 请求计算得出；请参阅[这些请求的处理方式](#sdkcontrolgetcontextusageresponse)。[`detail` 选项](#sdkcontrolgetcontextusageresponse)需要 Agent SDK v0.3.257 或更高版本 |
| `readFile(path, options?)` | 从会话的文件系统中读取文件。Claude Code 会相对于 `cwd` 解析路径；[`readFile()` 可以读取的内容](#what-readfile-can-read)列出了它可提供的文件。传递 `{ maxBytes }` 可更改读取上限（默认 1 MB，最大 10 MB），对于图像等二进制文件传递 `{ encoding: 'base64' }`。以 [`SDKControlReadFileResponse`](#sdkcontrolreadfileresponse) 完成；在权限被拒绝、文件不存在或传输错误时以 `null` 完成。需要 TypeScript SDK v0.2.121 或更高版本 |
| `reloadPlugins(options?)` | 从磁盘重新加载插件，使您在会话中途安装或编辑的插件能够作用于正在运行的会话。以 [`SDKControlReloadPluginsResponse`](#sdkcontrolreloadpluginsresponse) 解析，其中列出会话的命令、子代理、插件和 MCP 服务器状态。需要 Agent SDK v0.2.85 或更高版本。[`holdOnCacheImpact` 选项](#sdkcontrolreloadpluginsresponse)需要 Agent SDK v0.3.268 或更高版本。[`addPlugins` 选项](#load-more-plugin-folders-with-addplugins)需要 Agent SDK v0.3.301 或更高版本 |
| `reloadSkills()` | 从磁盘重新加载 skill，使您在会话中途添加或编辑的 skill 可供正在运行的会话使用。以 [`SDKControlReloadSkillsResponse`](#sdkcontrolreloadskillsresponse) 完成，其中列出重新加载后可用的 skill。需要 Agent SDK v0.3.163 或更高版本 |
| `reloadOutputStyles()` | 从磁盘重新读取[输出样式](https://code.claude.com/docs/zh-CN/output-styles.md)，使您在会话中途添加或编辑的样式文件可供正在运行的会话使用。以 [`SDKControlReloadOutputStylesResponse`](#sdkcontrolreloadoutputstylesresponse) 完成，其中列出重新加载后可用的样式名称。需要 Agent SDK v0.3.261 或更高版本 |
| `seedReadState(path, mtime)` | 将文件标记为已读取，使之后的 Edit 不会因 `File has not been read yet` 而失败。请参阅 [`seedReadState()`](#seedreadstate)。需要 TypeScript SDK v0.2.83 或更高版本 |
| `accountInfo()` | 返回账户信息 |
| `reconnectMcpServer(serverName)` | 按名称重新连接 MCP 服务器。如果该名称同时匹配 `.mcp.json` 或 `~/.claude.json` 等设置文件中的条目，Claude Code 会重新连接您通过 [`mcpServers`](#options) 或 `setMcpServers()` 配置的服务器，而不是设置文件中的条目。该解析顺序需要 Claude Code v2.1.257 或更高版本 |
| `toggleMcpServer(serverName, enabled)` | 按名称启用或禁用 MCP 服务器，名称解析方式与 `reconnectMcpServer()` 相同。禁用服务器会断开其连接并移除其工具。有关每种服务器所需的 Claude Code 版本，请参阅 [`toggleMcpServer()`](#togglemcpserver) |
| `setMcpServers(servers)` | 替换此方法管理的 MCP 服务器：通过此方法添加的服务器以及[进程内 SDK 服务器](#createsdkmcpserver)。以 [`McpSetServersResult`](#mcpsetserversresult) 完成，其中指明添加和移除了哪些服务器以及任何错误；该部分说明了哪些其他服务器会保持连接 |
| `setMcpPermissionModeOverride(serverName, mode)` | 用比会话更严格的模式覆盖某个 MCP 服务器工具的权限模式，或使用 `null` 清除覆盖。仅在流式输入模式下可用。请参阅 [`setMcpPermissionModeOverride()`](#setmcppermissionmodeoverride)。需要 TypeScript SDK v0.3.187 或更高版本 |
| `readMcpResource(serverName, uri)` | *Alpha。* 从已连接的 MCP 服务器读取一个 MCP Apps `ui://` 资源，以便您的应用渲染工具的小组件。以 [`SDKControlMcpReadResourceResponse`](#sdkcontrolmcpreadresourceresponse) 完成。需要 TypeScript Agent SDK v0.3.280 或更高版本 |
| `streamInput(stream)` | 向查询流式传输输入消息，用于多轮对话 |
| `stopTask(taskId)` | 按 ID 停止正在运行的后台任务 |
| `backgroundTasks(toolUseId?)` | 将当前轮次正在等待的运行中 Bash 命令和子代理移到后台，效果与在交互式终端中按 Ctrl+B 相同。请参阅 [`backgroundTasks()`](#backgroundtasks)。需要 TypeScript SDK v0.2.139 或更高版本 |
| `close()` | 关闭查询并终止底层进程。强制结束查询并清理所有资源 |

#### `setMcpPermissionModeOverride()`

用比会话更严格的模式覆盖某个 MCP 服务器工具的权限模式，或使用 `null` 清除覆盖。当会话运行在跳过逐项操作提示的模式下，但某个服务器的工具仍应经过审查时，请使用它。需要 TypeScript SDK v0.3.187 或更高版本。

将以下三个值之一作为 `mode` 传递：

* `'default'`：Claude Code 以 [Manual 模式](https://code.claude.com/docs/zh-CN/permission-modes.md#available-modes)的方式评估该服务器的工具调用。
* `'auto'`：Claude Code 将该服务器的工具调用交由[自动模式](https://code.claude.com/docs/zh-CN/permission-modes.md#eliminate-prompts-with-auto-mode)分类器处理。当该分类器对会话不可用时，Claude Code 会拒绝此值。
* `null`：清除覆盖。

覆盖仅在会话处于跳过逐项操作提示的模式（`bypassPermissions` 或 `auto`）时生效。在任何其他会话模式下，Claude Code 会忽略该覆盖，因此它永远不会放宽会话允许的范围。

当尚不知道有该名称的服务器时，promise 会以 `{ warning }` 解析。Claude Code 仍会存储该覆盖，并在具有该确切名称的服务器连接后应用它，因此该警告只是拼写检查，而不是失败。

仅在流式输入模式下可用，与 `setModel()` 和 `setPermissionMode()` 的限制相同。

#### `seedReadState()`

将文件标记为已读取，使之后的 Edit 不会因 `File has not been read yet` 而失败。当您的应用已从对话上下文中丢弃了先前的 `Read` 结果，而 Claude 接下来要编辑该文件时，请使用它。需要 TypeScript SDK v0.2.83 或更高版本。

将 `mtime` 传递为先前那次读取时文件的修改时间，以自纪元起的整毫秒数表示，例如 `Math.floor(stat.mtimeMs)`。如果磁盘上的文件在该时间之后发生了变化，Claude Code 会跳过标记。

无论文件是否被标记，promise 都会解析，因此 promise 解析并不能告诉您实际发生了哪种情况。

#### `backgroundTasks()`

将当前轮次正在等待的运行中 Bash 命令和子代理移到后台，效果与在交互式终端中按 Ctrl+B 相同。传递某个工具调用的 `toolUseId` 可仅将该调用启动的 Bash 命令或子代理移到后台，省略它则将所有前台任务移到后台。需要 TypeScript SDK v0.2.139 或更高版本。

每个被移到后台的工具调用会立即向 Claude 返回一个"正在后台运行"的结果，当前轮次继续进行，并且在任务完成时 Claude Code 会发出一个 [`SDKTaskNotificationMessage`](#sdktasknotificationmessage)。

在启用了后台任务的会话中，promise 会解析为一个布尔值：

* **`true`**：有任务被移到后台，或者您省略了 `toolUseId`。
* **`false`**：您传递的 `toolUseId` 未匹配任何前台任务。

如果会话禁用了后台任务（例如通过 [`CLAUDE_CODE_DISABLE_BACKGROUND_TASKS`](https://code.claude.com/docs/zh-CN/env-vars)），promise 会被拒绝，并且不会有任何任务被移到后台。

#### `applyFlagSettings()`

在不重启查询的情况下更改正在运行的会话的[设置](https://code.claude.com/docs/zh-CN/settings.md)。当某个没有专用设置方法的设置需要在会话中途更改时使用它，例如在 Agent 读取不受信任的输入后收紧 `permissions`。`setModel()` 和 `setPermissionMode()` 是针对这两个键的专用设置方法；`applyFlagSettings()` 是通用形式，接受设置键的任意子集，在此处传递 `model` 的行为与 `setModel()` 相同。

只有部分键会在会话中途生效：

* **在下一轮生效**：`ultracode`、`permissions`、`hooks`、`skillOverrides`、`fastMode`、`agent`。切换 `agent` 还会在下一轮应用该 Agent 的模型覆盖和 hook。其系统提示词在下一轮生效。在[复用已记录系统提示词](https://code.claude.com/docs/zh-CN/agent-sdk/modifying-system-prompts.md#change-the-prompt-of-an-existing-session)的会话中，它会在会话被压缩后生效。
* **在当前轮次生效**：`model`。如果您在 Claude 处理某一轮时切换 `model`，Claude 正在生成的回复会使用旧模型完成，而该轮的其余部分（从 Claude Code 对模型发起的下一次调用开始）会使用新模型。子代理保留各自的模型。在 v2.1.212 之前，轮次中途的切换会等到下一轮才生效。
* **最迟在下一轮生效**：`effortLevel`。该级别从 Claude Code 对模型的下一次调用开始生效。如果[更改 effort 会保留提示缓存](https://code.claude.com/docs/zh-CN/prompt-caching.md#changing-effort-level)（例如在使用 API 密钥或 Claude 订阅的 Claude Opus 5.5 上），则预计会在下一轮生效。
* **会话中途无效**：系统提示词选项。这些选项在启动时解析一次，因此即使调用成功，正在运行的会话也会保留原始值。要更改它们，请启动新会话。

`effortLevel` 接受一个 [effort 级别](https://code.claude.com/docs/zh-CN/model-config.md#adjust-effort-level)名称。它还接受 `"ultracode"`，表示请求 `xhigh` effort 并开启 [ultracode](https://code.claude.com/docs/zh-CN/workflows.md#let-claude-decide-with-ultracode)。`applyFlagSettings()` 声明的 `effortLevel` 不包含该值，因此在 TypeScript 中请传递 `{ ultracode: true, effortLevel: "xhigh" }` 以获得相同效果，或者单独传递 [`ultracode`](https://code.claude.com/docs/zh-CN/settings-reference#ultracode) 键，以在会话当前的 effort 级别下开启 ultracode。`ultracode` 值需要 Claude Code v2.1.203 或更高版本，并且只被 `applyFlagSettings()` 接受，设置文件中的 `effortLevel` 键不接受该值。在 v2.1.284 之前，单独传递 `ultracode` 键也会将级别设置为 `xhigh`。

这些值会写入标志设置层，合并覆盖在启动时 `query()` 的内联 `settings` 选项所设置的内容之上。这与[本页优先级章节](#settings-precedence)中所称的编程选项属于同一层级。`effortLevel` 是个例外：如 [effort 解析顺序](https://code.claude.com/docs/zh-CN/model-config.md#adjust-effort-level)所示，您传递的级别会替换来自 `--effort` 或设置（包括托管设置）的级别。

连续调用会对顶层键进行浅合并。第二次调用 `{ permissions: {...} }` 会替换先前调用中的整个 `permissions` 对象，而不是深度合并到其中。

要清除通过 `applyFlagSettings()` 设置的键，请为该键传递 `null`。大多数键随后会先回退到 `query()` 的 `settings` 选项在启动时设置的值，然后回退到优先级更低的来源。被清除的 `model` 会重置为 [Claude Code 的默认模型](https://code.claude.com/docs/zh-CN/model-config.md)，即使设置文件设置了 `model` 也是如此。传递 `undefined` 没有效果，因为 JSON 序列化会将其丢弃。

除 `model` 外，还有三个键会重置会话状态，而不是回退：

* `effortLevel: null` 会将会话恢复为模型的默认 effort 级别，而不是 `query()` 的 `effort` 选项或设置文件中的 `effortLevel`。
* `agent: null` 会从下一轮开始在不使用任何 Agent 的情况下运行主线程，而不是恢复 `query()` 的 `agent` 选项或设置文件中的 `agent`。如果被清除的 Agent 曾应用自己的模型，会话会恢复为启动时解析的模型。
* `ultracode: null` 会关闭 ultracode（与 `false` 的效果相同），而不是恢复设置文件中的 `ultracode` 值。会话保留其当前的 effort 级别，因此如需更改，请在同一次调用中传递 `effortLevel`。

仅在流式输入模式下可用，与 `setModel()` 和 `setPermissionMode()` 的限制相同。

下面的示例在会话中途切换活动模型，然后清除覆盖，使模型重置为 [Claude Code 的默认模型](https://code.claude.com/docs/zh-CN/model-config.md)。

```typescript theme={null}
import { query } from "@anthropic-ai/claude-agent-sdk";

const q = query({ prompt: messageStream });

// Override the model for the rest of the session
await q.applyFlagSettings({ model: "claude-opus-4-6" });

// Later: clear the override; the model resets to Claude Code's default
await q.applyFlagSettings({ model: null });
```

> [!NOTE]
>
> `applyFlagSettings()` 仅适用于 TypeScript。Python SDK 未提供等效方法。

#### `updateSettings()`

将一个允许列表中的键写入磁盘上的设置文件，使该值在之后加载该来源的会话中保持有效。每个来源接受一个键，值为字符串：

* **`"localSettings"`**：接受 `outputStyle`，并将其合并到项目的本地设置文件 `.claude/settings.local.json` 中。新样式会在会话的下一次请求时生效。
* **`"userSettings"`**：接受 `effortLevel`，并将其保存为会话当前模型的默认 [effort 级别](https://code.claude.com/docs/zh-CN/model-config.md#adjust-effort-level)，位于您用户设置文件的 [`modelSettings`](https://code.claude.com/docs/zh-CN/settings-reference#modelsettings) 下。传递 `max` 不会写入任何内容，因为 `max` 仅限会话使用。无论哪种情况，正在运行的会话都会保留其当前的 effort 级别，因此如果您也想更改它，请调用 [`applyFlagSettings()`](#applyflagsettings)。此来源需要 TypeScript SDK v0.3.277 或更高版本，其捆绑了 Claude Code v2.1.277。

当请求携带任何其他键、会话通过远程传输运行，或会话的 [`settingSources`](#options) 排除了您指定的来源时，调用会被拒绝。不支持删除键。

#### `toggleMcpServer()`

禁用服务器会断开其连接并从会话中移除其工具。对于您在会话中途添加的服务器和进程内服务器，这取决于您的 Claude Code 版本：

* 您在会话中途通过 `setMcpServers()` 添加的 stdio、SSE 或 HTTP 服务器：移除其工具需要 Claude Code v2.1.285 或更高版本。
* 您通过 [`createSdkMcpServer()`](#createsdkmcpserver) 创建的进程内服务器，无论是通过 `mcpServers` 还是 `setMcpServers()` 传递：断开其连接并移除其工具需要 Claude Code v2.1.286 或更高版本。禁用此类服务器还会使其仍在运行的工具调用失败，因此 Claude 会立即收到每个调用的错误结果，而无需等待您的处理程序返回。

### `WarmQuery`

由 [`startup()`](#startup) 返回的句柄。子进程已经生成并完成初始化，因此在此句柄上调用 `query()` 会将提示词直接写入已就绪的进程，没有启动延迟。

```typescript theme={null}
interface WarmQuery extends AsyncDisposable {
  query(prompt: string | AsyncIterable<SDKUserMessage>): Query;
  close(): void;
}
```

#### 方法

| 方法 | 描述 |
| :- | :- |
| `query(prompt)` | 向预热的子进程发送提示词并返回一个 [`Query`](#query-object)。每个 `WarmQuery` 只能调用一次 |
| `close()` | 关闭子进程而不发送提示词。用于丢弃不再需要的预热查询 |

`WarmQuery` 实现了 `AsyncDisposable`，因此可以与 `await using` 一起使用以实现自动清理。

### `SpareProcess`

*Alpha。* 由 [`prewarm()`](#prewarm) 返回的句柄：一个已启动但尚未绑定到会话的 Claude Code 进程，可以被认领一次。需要 TypeScript Agent SDK v0.3.282 或更高版本。

```typescript theme={null}
interface SpareProcess extends AsyncDisposable {
  claim(params: {
    prompt: string | AsyncIterable<SDKUserMessage>;
    options: ClaimOptions;
  }): Query;
  readonly claimed: Promise<{ cwd: string; sessionId: string; parkedMs?: number; sdkMcpSettled: boolean }>;
  readonly exited: Promise<void>;
  close(): void;
}
```

#### 成员

| 成员 | 描述 |
| :- | :- |
| `claim({ prompt, options })` | 将备用进程绑定到 `options.cwd` 中的会话并发送其第一条消息。与 `query()` 一样同步返回一个 [`Query`](#query-object)。只能调用一次 |
| `claimed` | 在 Claude Code 接受认领后，以会话的工作目录和 ID 完成。在以下情况下拒绝：Claude Code 拒绝认领；进程先已退出或被关闭；以及会话在未使用您所请求的 `model` 或 `maxThinkingTokens` 的情况下运行（此时消息以 `option_not_applied` 开头） |
| `exited` | 在进程退出时完成，无论是否已被认领。请替换在您认领之前就已退出的备用进程 |
| `close()` | 终止进程。在认领之前调用会丢弃该备用进程并拒绝 `claimed` |

`options.cwd` 为必填项。认领还可以设置 `additionalDirectories`、`model`、`permissionMode`、`maxThinkingTokens`、`settings` 中的标志设置覆盖层、`appendSystemPrompt`、`title`、`agents`，以及 `env` 中的每会话令牌。

Claude Code 可能会拒绝认领，例如针对不存在的文件夹，或其项目设置设置了 `env`、`agent` 或 `model` 的文件夹。被拒绝后，`claim()` 已发送的提示词会收到一个文本以 `not_claimed` 开头的错误结果，随后返回的查询会抛出异常。请将查询的循环包裹在 try 块中，以便在抛出异常后继续。当 `claimed` 以 `option_not_applied` 开头的消息拒绝时，表示会话在未使用您所请求的 `model` 或 `maxThinkingTokens` 的情况下运行。在任何其他拒绝之后，您的提示词都尚未运行，因此请改用 `query()` 启动会话。

### `SDKControlInitializeResponse`

`initializationResult()` 的返回类型。包含会话初始化数据。

```typescript theme={null}
type SDKControlInitializeResponse = {
  commands: SlashCommand[];
  agents: AgentInfo[];
  output_style: string;
  available_output_styles: string[];
  models: ModelInfo[];
  account: AccountInfo;
  fast_mode_state?: "off" | "cooldown" | "on";
  fast_mode_disabled_reason?: FastModeDisabledReason;
  hooks_applied?: boolean;
  sdk_mcp_manifests_parked?: Record<
    string,
    | "parked"
    | "already_connected"
    | "protocol_version_mismatch"
    | "malformed"
    | "not_honoured"
  >;
  claude_code_version?: string;
};
```

`hooks_applied` 报告 Claude Code 是否注册了 `initialize` 请求所携带的 `hooks`。SDK 会在会话启动时发送一次该请求，并在每次调用 [`reinitialize()`](#query-object) 时再次发送。该字段需要 Agent SDK v0.3.238 或更高版本。

当请求未携带 hook 时，Claude Code 会省略该字段。当请求携带了 hook 时，其值取决于该请求是否为会话的首次 initialize，以及对于重复的 initialize，它是如何到达会话的：

* `true`：Claude Code 注册了这些 hook。会话的首次 initialize 返回此值。通过 CLI 的 stdin 发送的重复 initialize 也返回 `true`。在这种情况下，新请求中的 hook 会替换先前注册的 hook。
* `false`：Claude Code 忽略了这些 hook。发送到远程会话的重复 initialize 返回此值，因此加入会话的第二个客户端无法替换第一个客户端注册的 hook。

在 Agent SDK v0.3.238 之前，响应从不携带该字段，并且 Claude Code 在每次重复 initialize 时都会忽略 `hooks`。

请求的 `sdkMcpServerManifests` 字段和响应的 `sdk_mcp_manifests_parked` 字段用于您通过 [`createSdkMcpServer()`](#createsdkmcpserver) 创建的进程内 [SDK MCP 服务器](https://code.claude.com/docs/zh-CN/agent-sdk/custom-tools.md)。您的应用不需要设置或读取这两个字段。

`claude_code_version` 是运行会话轮次的进程的 Claude Code 版本。它与 [`SDKSystemMessage`](#sdksystemmessage) init 消息携带的值相同，因此您可以在第一轮之前读取 CLI 版本。

* 该字段需要 Agent SDK v0.3.296 或更高版本，该版本捆绑了 Claude Code v2.1.296。
* 如果该字段为 `undefined` 且您设置了 `pathToClaudeCodeExecutable`，请将其指向 Claude Code v2.1.296 或更高版本。

响应始终会报告 `fast_mode_state`，并且当有因素阻止[快速模式](https://code.claude.com/docs/zh-CN/fast-mode.md)时，`fast_mode_disabled_reason` 会随之携带原因代码，因此您可以解释被阻止的状态，而无需重新推断可用性。这两种行为都需要 Claude Code v2.1.219 或更高版本。在 v2.1.219 之前，当快速模式不可用时，响应会省略 `fast_mode_state`，并且从不携带原因。有关原因代码及其含义，请参阅结果消息上的 [`fast_mode_disabled_reason`](#sdkresultmessage)。

成功的 `initialize` 的控制响应包装器还携带一个 `pending_permission_requests` 数组。该字段位于响应包装器本身，而不在上述 `SDKControlInitializeResponse` 负载中。每个条目都是一条完整的 `control_request` 消息，其 `{ type: "control_request", request_id, request }` 结构与会话运行期间为权限请求流式发送的结构相同。

该数组列出此 Claude Code 进程已发出但尚未解决的权限请求。SDK 会为您读取该数组，并将每个条目分派到您的 [`canUseTool`](#canusetool) 回调，这与 [`reinitialize()`](#query-object) 在传输中断后触发的重新投递相同。请以幂等方式处理重复的请求 ID，因为某个条目可能重复了回调在连接断开前已收到的请求。

该数组在成功的 `initialize` 响应中始终存在，当此进程没有未解决的权限请求时为空。需要 Claude Code v2.1.268 或更高版本。更早的版本可能会省略该字段，因此如果您自行解析线路协议，请将缺失该字段视为较旧的 CLI，而不是视为没有待处理请求的证据。

### `SDKControlInterruptResponse`

中断回执：在 [`SDKSystemMessage.capabilities`](#sdksystemmessage) 中声明了 `interrupt_receipt_v1` 能力的 CLI 上，[`interrupt()`](#query-object) 完成时返回的值。需要 Claude Code v2.1.205 或更高版本。更早的 CLI 会以空的成功负载响应中断，因此 `interrupt()` 会以 `undefined` 完成。

```typescript theme={null}
type SDKControlInterruptResponse = {
  still_queued: string[];
  cancelled?: string[];
};
```

`still_queued` 列出中断到达时仍处于待处理状态的用户消息的 UUID：仍在队列中的消息，以及 Claude Code 已从队列中取出准备用于下一轮的消息。一旦会话的第一轮已经开始，除非您先取消，否则 Claude Code 会在中断之后处理列出的消息，并且可能将多条消息合并为一轮。如果您在第一轮开始之前中断，Claude Code 会在该轮开始后立即中止它，该轮中列出的消息将不会得到回复。

请使用回执来决定是否需要重新发送任何内容。未被您取消的列出消息无论是否得到回复都会进入对话，因此重新发送会使其两次传递给 Claude。

解读该列表时请注意以下事项：

* 只有带 UUID 入队的消息才会出现。空数组并不意味着不会再运行其他内容。
* 只列出主线程消息。发送给子代理的消息不在范围内。
* 该列表可能包含您的客户端从未发送过的 UUID，例如[会话作用域的定时任务](https://code.claude.com/docs/zh-CN/scheduled-tasks.md)触发器。请忽略您无法识别的 UUID，而不是将其视为错误。

直接驱动 CLI 控制协议（而不是通过 `interrupt()`）的客户端可以在 `interrupt` 控制请求上设置 `cancel_queued: true`。Claude Code v2.1.219 及更高版本通过 [`SDKSystemMessage.capabilities`](#sdksystemmessage) 中的 `interrupt_cancel_queued_v1` 能力声明支持；较旧的 CLI 会忽略该字段，并照常运行排队的消息。这样的中断还会取消原本会列在 `still_queued` 下的每条消息：回执改为将它们列在 `cancelled` 下，`still_queued` 为空，并且这些消息都不会运行。

`cancelled` 列表的注意事项与 `still_queued` 相同。`interrupt()` 方法从不发送 `cancel_queued`，因此它完成时返回的回执不携带 `cancelled`。

回执是在处理中断时拍摄的快照，在干净的中断中，它会在被中断轮次的 [`SDKResultMessage`](#sdkresultmessage) 之前到达。请读取回执，而不是在该结果之后检查队列：循环会立即开始下一个排队的轮次，因此您在结果之后检查的队列已经发生了变化。

### `SDKControlGetContextUsageResponse`

[`getContextUsage()`](#query-object) 的返回类型。使用默认的 `detail` 时，响应包含 Claude Code 在交互式会话中为 `/context` 命令渲染的负载。除 token 计数外，该负载还携带 `color` 和 `gridRows` 等显示字段，Claude Code 用它们绘制 `/context` 用量网格。

该方法的可选 `detail` 参数决定 Claude Code 如何统计每个类别。`detail` 参数需要 Agent SDK v0.3.257 或更高版本。

* **`'full'`**：默认值。Claude Code 使用 [token 计数](https://platform.claude.com/docs/en/build-with-claude/token-counting) API 请求统计每个类别。这些请求不会出现在消息流中，因此读取消息流的成本跟踪不会看到它们。在 Anthropic API 上，token 计数不收费。
* **`'summary'`**：传递 `{ detail: 'summary' }` 以改为根据上一次响应的用量和本地估算得出结果。不会发出任何 token 计数请求，各类别的数值为近似值。

当您将 `/context` 作为提示词发送而不是调用该方法时，Claude Code 会将一个 [`SDKContextUsage`](#sdkcontextusage) 负载附加到传递结果的 assistant 消息的 `context_usage` 字段。该字段需要 Agent SDK v0.3.232 或更高版本。

在 `SDKControlGetContextUsageResponse` 中，`categories[].id`、`mcpInstructions`，以及 `agents` 和 `skills.skillFrontmatter` 条目的 `pluginId` 和 `path` 字段需要 Agent SDK v0.3.304 或更高版本。

```typescript theme={null}
type SDKControlGetContextUsageResponse = {
  categories: {
    id?:
      | "system_prompt"
      | "system_tools"
      | "mcp_tools"
      | "mcp_instructions"
      | "mcp_tools_deferred"
      | "system_tools_deferred"
      | "custom_agents"
      | "memory_files"
      | "skills"
      | "messages"
      | "autocompact_buffer"
      | "compact_buffer"
      | "free_space";
    name: string;
    tokens: number;
    color: string;
    isDeferred?: boolean;
    kind: "used" | "free" | "buffer" | "deferred";
  }[];
  totalTokens: number;
  maxTokens: number;
  rawMaxTokens: number;
  percentage: number;
  gridRows: {
    color: string;
    isFilled: boolean;
    categoryName: string;
    tokens: number;
    percentage: number;
    squareFullness: number;
  }[][];
  model: string;
  memoryFiles: {
    path: string;
    type: string;
    tokens: number;
  }[];
  mcpTools: {
    name: string;
    serverName: string;
    tokens: number;
    isLoaded?: boolean;
  }[];
  mcpInstructions?: {
    serverName: string;
    tokens: number;
  }[];
  deferredBuiltinTools?: {
    name: string;
    tokens: number;
    isLoaded: boolean;
  }[];
  systemTools?: {
    name: string;
    tokens: number;
  }[];
  systemPromptSections?: {
    name: string;
    tokens: number;
  }[];
  agents: {
    agentType: string;
    source: string;
    pluginId?: string;
    path?: string;
    tokens: number;
  }[];
  slashCommands?: {
    totalCommands: number;
    includedCommands: number;
    tokens: number;
  };
  skills?: {
    totalSkills: number;
    includedSkills: number;
    tokens: number;
    skillFrontmatter: {
      name: string;
      source: string;
      pluginName?: string;
      pluginId?: string;
      path?: string;
      tokens: number;
    }[];
  };
  autoCompactThreshold?: number;
  isAutoCompactEnabled: boolean;
  messageBreakdown?: {
    toolCallTokens: number;
    toolResultTokens: number;
    attachmentTokens: number;
    assistantMessageTokens: number;
    userMessageTokens: number;
    redirectedContextTokens: number;
    unattributedTokens: number;
    toolCallsByType: {
      name: string;
      callTokens: number;
      resultTokens: number;
    }[];
    attachmentsByType: {
      name: string;
      tokens: number;
    }[];
  };
  apiUsage: {
    input_tokens: number;
    output_tokens: number;
    cache_creation_input_tokens: number;
    cache_read_input_tokens: number;
  } | null;
};
```

从集合字段中读取 token 归属：

* `categories` 保存各类别的总计。每个条目的 `kind` 使用与 [`SDKContextUsageCategory`](#sdkcontextusagecategory) 相同的值对该行进行分类。请依据它而不是显示用的 `name` 来对行进行分类。该字段需要 Agent SDK v0.3.268 或更高版本。
* `mcpTools` 和 `agents` 将 token 归属到各个 MCP 工具和子代理。
* `memoryFiles` 列出每个已加载的记忆文件及其开销。
* `skills.skillFrontmatter` 将 skill 列表的 token 归属到每个被包含的 skill。每个 skill 的计数衡量的是 Claude Code 实际发送的该 skill 列表条目，可能比该 skill 的完整 frontmatter 更短。比较 `skills.totalSkills` 与 `skills.includedSkills`，可查看是否每个发现的 skill 都进入了列表。

`totalTokens` 是会话当前的上下文用量，`maxTokens` 是衡量该用量所依据的窗口。该窗口是模型的上下文窗口，或者在适用时为更低的自动压缩窗口。`rawMaxTokens` 与 `maxTokens` 的值相同，`percentage` 是 `totalTokens` 占该窗口的百分比（四舍五入）。`apiUsage` 保存最近一次 API 响应的用量，而不是会话的累计总量。

Claude Code 不会设置可选的 `deferredBuiltinTools`、`systemTools` 和 `systemPromptSections` 诊断字段，因此即使类型中声明了它们，也应预期它们不存在。

#### 按 ID 查找行、服务器的指令 token 或条目的插件和文件

MCP 服务器指令是已连接服务器提供、由 Claude Code 添加到上下文中的使用说明，`categories` 中的 `mcp_instructions` 行统计它们的 token。使用以下 [`getContextUsage()`](#query-object) 字段，可以按 `id` 挑出某一 `categories` 行、按服务器拆分 `mcp_instructions` 的 token，或查找某个 Agent 或 skill 条目背后的插件和文件：

| 字段 | 所在位置 | 包含内容 |
| :- | :- | :- |
| `id` | 每个 `categories` 行 | 该行的标识符，为[响应类型](#sdkcontrolgetcontextusageresponse)所列值之一，例如 `mcp_instructions`。按 `id` 匹配可挑出某一行，按 `kind` 匹配可对行进行分组。较新的 Claude Code 版本可能会发送您已安装的类型中未列出的值，因此请将无法识别的 `id` 视为不存在 |
| `mcpInstructions` | 响应 | 每个提供指令的已连接 MCP 服务器对应一个 `{ serverName, tokens }` 条目；若没有服务器提供指令则为空数组。`tokens` 值是估算值，其总和等于 `mcp_instructions` 行的 `tokens`。`serverName` 是 [`mcpServerStatus()`](#query-object) 为该服务器报告的 `name` |
| `pluginId` | `agents` 和 `skills.skillFrontmatter` 条目 | 以 `name@marketplace` 形式的 ID 指明条目来自的插件；对于不来自插件的条目则不存在。当插件并非来自市场时，`@` 之后的部分会改为指明[其来源](https://code.claude.com/docs/zh-CN/plugins/loading.md#find-where-a-plugin-came-from)，例如对于您通过 [`plugins`](#options) 选项加载的插件为 `inline` |
| `path` | `agents` 和 `skills.skillFrontmatter` 条目 | 定义该 Agent 或 skill 的文件在运行 Claude Code 的机器上的绝对路径。当没有文件定义该条目时，该字段不存在，例如来自 [`agents`](#options) 选项的 Agent 或 Claude Code 内置的 skill |

作为按 `id` 匹配的示例，以下代码查找 `mcp_instructions` 行：

```typescript theme={null}
// q is the Query object that query() returned
const usage = await q.getContextUsage();
// undefined if no connected server supplies instructions
const row = usage.categories.find((c) => c.id === "mcp_instructions");
```

### `SDKControlReadFileResponse`

[`readFile()`](#query-object) 的返回类型。

```typescript theme={null}
type SDKControlReadFileResponse = {
  contents: string;
  absPath: string;
  truncated?: boolean;
  encoding?: 'base64';
};
```

`contents` 保存文件文本；当您请求 `encoding: 'base64'` 时则保存 base64 数据，此时响应的 `encoding` 字段会被设置为 `'base64'`。`absPath` 是解析后的绝对路径。当文件长度超过 `maxBytes` 上限且内容在该限制处被截断时，会设置 `truncated`。

#### `readFile()` 可以读取的内容

`readFile()` 提供的文件范围比 Read 工具更窄：

* 位于会话某个工作目录（例如 `cwd` 和 `additionalDirectories`）内的常规文件
* Claude Code 为该会话保存的少量自身文件，例如工具结果

`Read` 的拒绝和询问规则仍会阻止匹配的路径，而宽泛的 `Read` 允许规则不会向 `readFile()` 开放文件系统的其余部分。对于其他任何内容，调用都会以 `null` 完成。

### `SDKControlReloadPluginsResponse`

[`reloadPlugins()`](#query-object) 的返回类型。

```typescript theme={null}
type SDKControlReloadPluginsResponse = {
  commands: SlashCommand[];
  agents: AgentInfo[];
  plugins: {
    name: string;
    path: string;
    source?: string;
    version?: string;
  }[];
  mcpServers: McpServerStatus[];
  error_count: number;
  held?: boolean;
  cache_impact?: {
    mcp_servers_added: string[];
    mcp_servers_removed: string[];
    lsp_tool_change: ("adds" | "may-add" | "removes" | "may-remove") | null;
  };
};
```

集合字段描述调用之后的会话：

* `commands`、`agents` 和 `mcpServers`：会话的命令、子代理和 MCP 服务器状态，其结构与 `supportedCommands()`、`supportedAgents()` 和 `mcpServerStatus()` 返回的结构相同。`supportedAgents()` 会继续返回初始化时捕获的列表，因此要获取重新加载后的集合，请在此处读取 `agents`
* `plugins`：每个已加载的插件及其 `name` 和安装 `path`。`version` 复述插件清单中声明的内容，由插件作者控制，因此在信任之前请先验证。当清单未声明版本时会省略该字段
* `error_count`：加载插件时产生的错误数量

向 `reloadPlugins()` 传递 `{ holdOnCacheImpact: true }`，可在重新加载会使对话的提示缓存失效时将其挂起而不应用。Claude Code 会运行交互式 `/reload-plugins` 命令在[警告缓存开销](https://code.claude.com/docs/zh-CN/prompt-caching.md#enabling-or-disabling-a-plugin)之前所做的检查。该选项需要 Agent SDK v0.3.268 或更高版本。早于 v2.1.268 的 Claude Code 可执行文件（例如您通过 `pathToClaudeCodeExecutable` 指向的可执行文件）会忽略该选项并应用重新加载。

传递该选项后，请读取 `held` 以了解发生了什么：

* `true`：重新加载未被应用，集合字段描述的是会话的当前状态。`cache_impact` 说明应用后会发生哪些变化。如果仍要应用，请在不带该选项的情况下再次调用 `reloadPlugins()`。
* `false`：检查未发现缓存影响，重新加载已被应用。
* 不存在：您没有传递该选项，或者 Claude Code 可执行文件早于 v2.1.268 并已应用重新加载。

`cache_impact` 仅与 `held: true` 一同出现。`mcp_servers_added` 和 `mcp_servers_removed` 以限定的 `plugin:<plugin>:<server>` 名称列出重新加载将注册或移除的插件 MCP 服务器。这些名称由插件作者编写，因此在显示之前请先验证。`lsp_tool_change` 说明应用后是否会添加或移除 LSP 工具，如果两者都不会发生则为 `null`。`may-` 形式表示检查无法完全看到待处理的插件集合。

#### 使用 `addPlugins` 加载更多插件文件夹

要加载启动时未包含在 [`plugins`](#options) 选项中的插件文件夹，请将它们作为 `addPlugins` 传给 `reloadPlugins()`，这是一个由 [`SdkPluginConfig`](#sdkpluginconfig) 条目组成的列表，其中的 `path` 必须是绝对路径。Claude Code 会在一次处理中加载新文件夹并重新加载会话中已有的插件。每个新增插件的 MCP 服务器也会连接，除非您在该文件夹的条目上设置了 [`skipMcpDiscovery`](#sdkpluginconfig)。需要 Agent SDK v0.3.301 或更高版本以及 Claude Code v2.1.301 或更高版本。

以下示例向正在运行的查询 `q` 添加一个文件夹，并在调用返回时打印会话已加载的每个插件的安装路径：

```typescript theme={null}
const result = await q.reloadPlugins({
  addPlugins: [{ type: "local", path: "/opt/my-app/plugins/reports" }]
});

console.log(result.plugins.map((plugin) => plugin.path));
```

如果该文件夹已加载，打印的列表中会包含 `/opt/my-app/plugins/reports`，并且该插件的命令和子代理会出现在 `result.commands` 和 `result.agents` 中。

新增的文件夹会一直作为会话的一部分。当您再次调用 `reloadPlugins()` 时，Claude Code 会将其与其他插件一起重新加载，您无需再次在 `addPlugins` 中传入它。

##### 当 `reloadPlugins()` 未加载 `addPlugins` 文件夹时

如果 `reloadPlugins()` 被拒绝，`addPlugins` 中的文件夹都不会被添加，因此重试时请再次在 `addPlugins` 中传入它们。当拒绝由以下某个原因导致时，会话会保留其原有的插件：

* **相对 `path` 或格式错误的条目**：错误消息为 `add_plugins must be a list of { type: "local", path } entries with absolute paths`。只要有一个错误条目，Claude Code 就会拒绝整个列表，因此请将每个条目都传为 `{ type: "local", path }`，并使用绝对 `path`
* **管理员在您的托管设置中启用了 [`disableSideloadFlags`](https://code.claude.com/docs/zh-CN/settings-reference#disablesideloadflags)**：消息为 `reload_plugins add_plugins is disabled by managed settings (disableSideloadFlags)`。要加载该文件夹，请联系该管理员移除此设置
* **无法重建[沙箱](https://code.claude.com/docs/zh-CN/sandboxing.md)**：启用[沙箱隔离](#sandboxsettings)时，Claude Code 会重建沙箱配置，使沙箱中的 shell 命令无法写入您添加的文件夹。消息为 `the sandbox could not be rebuilt to keep shell commands out of this folder`。请检查您的[沙箱设置](#sandboxsettings)是否仍构成有效配置

如果您传入了 `holdOnCacheImpact`，且调用以 [`held: true`](#sdkcontrolreloadpluginsresponse) 解析，则 `addPlugins` 中的文件夹不会被添加。如需仍然添加，请使用相同的 `addPlugins` 列表、不带 `holdOnCacheImpact` 再次调用 `reloadPlugins()`。

如果调用未以 `held: true` 解析，但某个文件夹仍未出现在 `result.plugins` 中，请先排查以下原因：

* **文件夹不存在或其插件加载失败**：`result.error_count` 为 `1` 或更高。即便如此，该文件夹仍是会话的一部分。修复文件夹后，再次调用 `reloadPlugins()` 即可加载它，这次无需在 `addPlugins` 中列出它
* **Claude Code 可执行文件早于 v2.1.301**：早于 v2.1.301 的可执行文件会忽略 `addPlugins`，只重新加载会话中已有的插件。请检查 [init 消息](#sdksystemmessage)中的 `claude_code_version`，并运行 v2.1.301 或更高版本的可执行文件

### `SDKControlReloadSkillsResponse`

[`reloadSkills()`](#query-object) 的返回类型。

```typescript theme={null}
type SDKControlReloadSkillsResponse = {
  skills: SlashCommand[];
};
```

`skills` 以与 `supportedCommands()` 返回的相同 [`SlashCommand`](#slashcommand) 结构列出重新加载后可用的 skill。

### `SDKControlReloadOutputStylesResponse`

[`reloadOutputStyles()`](#query-object) 的返回类型。

```typescript theme={null}
type SDKControlReloadOutputStylesResponse = {
  available_output_styles: string[];
};
```

`available_output_styles` 列出重新加载后可用的内置和自定义输出样式的名称。

### `SDKControlMcpReadResourceResponse`

[`readMcpResource()`](#query-object) 的返回类型，携带 MCP 服务器的 `resources/read` 结果。需要 TypeScript Agent SDK v0.3.280 或更高版本。

```typescript theme={null}
type SDKControlMcpReadResourceResponse = {
  contents: {
    uri: string;
    mimeType?: string;
    text?: string;
    blob?: string;
    _meta?: Record<string, unknown>;
  }[];
};
```

向 `readMcpResource()` 传递 `mcpServerStatus()` 所报告的服务器名称以及一个 `ui://` URI，例如工具在其 [`_meta`](#mcpserverstatus) 中声明的 `ui.resourceUri`。对于任何其他 URI 方案、您的应用自行托管的 [SDK MCP 服务器](#createsdkmcpserver)以及未连接的服务器，调用都会被拒绝。当 init 消息的 [`capabilities`](#sdksystemmessage) 包含 `mcp_read_resource_v1` 时可用。

每个 `contents` 条目都是服务器发送的一个内容项，但会去掉 `com.anthropic/` 前缀下的任何 `_meta` 键，该前缀保留给 Claude Code 使用。`blob` 保存二进制项的 base64 数据，`_meta` 是该项自身的 `_meta`，MCP Apps 服务器会在其中放置资源的 `ui.csp` 和 `ui.permissions`。

这些内容是不受信任的第三方 HTML，因此请在沙箱中渲染它们。

### `AgentDefinition`

以编程方式定义的子代理的配置。

```typescript theme={null}
type AgentDefinition = {
  description: string;
  tools?: string[];
  disallowedTools?: string[];
  prompt: string;
  model?: string;
  mcpServers?: AgentMcpServerSpec[];
  skills?: string[];
  initialPrompt?: string;
  maxTurns?: number;
  autoCompactWindow?: number;
  background?: boolean;
  omitClaudeMd?: boolean;
  memory?: "user" | "project" | "local";
  effort?: "low" | "medium" | "high" | "xhigh" | "max" | number;
  permissionMode?: PermissionMode;
  criticalSystemReminder_EXPERIMENTAL?: string;
};
```

| 字段 | 必需 | 描述 |
| :- | :- | :- |
| `description` | 是 | 用自然语言描述何时使用此 Agent |
| `tools` | 否 | 允许的工具名称数组。如果省略，则继承所有[子代理可用的工具](https://code.claude.com/docs/zh-CN/sub-agents.md#available-tools)。要将 Skills 预加载到 Agent 的上下文中，请使用 `skills` 字段，而不是在此处列出 `'Skill'` |
| `disallowedTools` | 否 | 要为此 Agent 明确禁止的工具名称数组。也接受 MCP 服务器级别的模式：`mcp__server` 或 `mcp__server__*` 会移除该服务器的所有工具，`mcp__*` 会移除任何服务器的所有 MCP 工具 |
| `prompt` | 是 | Agent 的系统提示词 |
| `model` | 否 | 此 Agent 的模型覆盖。接受 `'fable'`、`'opus'`、`'sonnet'`、`'haiku'`、`'inherit'` 等别名，或完整的模型 ID。`'inherit'` 使用主模型。省略时，Claude Code 会按照[子代理模型顺序](https://code.claude.com/docs/zh-CN/sub-agents.md#choose-a-model)选择模型 |
| `mcpServers` | 否 | 此 Agent 的 MCP 服务器规范 |
| `skills` | 否 | 要预加载到 Agent 上下文中的 skill 名称数组 |
| `initialPrompt` | 否 | 当此 Agent 作为主线程 Agent 运行时，作为第一轮用户输入自动提交 |
| `maxTurns` | 否 | 停止前的最大 agentic 轮次数（API 往返次数） |
| `autoCompactWindow` | 否 | 当此 Agent 作为子代理运行时，将其[自动压缩窗口](https://code.claude.com/docs/zh-CN/sub-agents.md#subagent-auto-compact-window)降低到 `100000` 至 `1000000` 之间的 token 数。需要 TypeScript Agent SDK v0.3.296 或更高版本 |
| `background` | 否 | 被调用时将此 Agent 作为非阻塞的后台任务运行 |
| `omitClaudeMd` | 否 | 不将用户、项目和本地 CLAUDE.md 文件纳入此 Agent 的初始上下文；托管策略文件仍会加载。适用于从 Agent 工具提示词中获取所需全部信息的 Agent。当此 Agent 作为主线程 Agent 运行时忽略。需要 TypeScript Agent SDK v0.3.271 或更高版本 |
| `memory` | 否 | 此 Agent 的记忆来源：`'user'`、`'project'` 或 `'local'` |
| `effort` | 否 | 此 Agent 的推理 effort 级别。接受命名级别或整数 |
| `permissionMode` | 否 | 此 Agent 内工具执行的权限模式。[子代理继承规则](https://code.claude.com/docs/zh-CN/agent-sdk/permissions.md#available-modes)决定其何时生效。请参阅 [`PermissionMode`](#permissionmode) |
| `criticalSystemReminder_EXPERIMENTAL` | 否 | 实验性：添加到系统提示词中的关键提醒 |

### `AgentMcpServerSpec`

指定子代理可用的 MCP 服务器。可以是服务器名称（引用父级 `mcpServers` 配置中某个服务器的字符串），也可以是将服务器名称映射到配置的内联服务器配置记录。

```typescript theme={null}
type AgentMcpServerSpec = string | Record<string, McpServerConfigForProcessTransport>;
```

其中 `McpServerConfigForProcessTransport` 为 `McpStdioServerConfig | McpSSEServerConfig | McpHttpServerConfig | McpSdkServerConfig`。

### `SettingSource`

控制 SDK 从哪些基于文件系统的配置来源加载设置。

```typescript theme={null}
type SettingSource = "user" | "project" | "local";
```

| 值 | 描述 | 位置 |
| :- | :- | :- |
| `'user'` | 全局用户设置 | `~/.claude/settings.json` |
| `'project'` | 共享项目设置（纳入版本控制） | `.claude/settings.json` |
| `'local'` | 本地项目设置，当 Claude Code 向其中保存设置时会被加入 gitignore | `.claude/settings.local.json` |

#### 默认行为

当省略 `settingSources` 或其为 `undefined` 时，`query()` 会加载与 Claude Code CLI 相同的文件系统设置：用户、项目和本地设置。有关无论此选项如何都会读取的输入以及如何禁用它们，请参阅 [settingSources 不控制的内容](https://code.claude.com/docs/zh-CN/agent-sdk/claude-code-features.md#what-settingsources-does-not-control)。

#### 为什么使用 settingSources

**禁用文件系统设置：**

```typescript theme={null}
import { query } from "@anthropic-ai/claude-agent-sdk";

// Do not load user, project, or local settings from disk
const result = query({
  prompt: "Analyze this code",
  options: { settingSources: [] }
});
```

**仅加载特定的设置来源：**

```typescript theme={null}
import { query } from "@anthropic-ai/claude-agent-sdk";

// Load only project settings, ignore user and local
const result = query({
  prompt: "Run CI checks",
  options: {
    settingSources: ["project"] // Only .claude/settings.json
  }
});
```

要加载 CLAUDE.md 项目指令，请在 `settingSources` 中包含 `"project"`。有关 CLAUDE.md 加载如何与系统提示词选项交互，请参阅[修改系统提示词](https://code.claude.com/docs/zh-CN/agent-sdk/modifying-system-prompts.md#claude-md-files-for-project-level-instructions)。

#### 设置优先级

当加载多个来源时，设置按以下优先级合并（从高到低）：

1. 本地设置（`.claude/settings.local.json`）
2. 项目设置（`.claude/settings.json`）
3. 用户设置（`~/.claude/settings.json`）

`agents`、`allowedTools` 和 `settings` 等编程选项会覆盖用户、项目和本地文件系统设置。如果托管策略设置和编程选项设置了相同的键，则以托管值为准，但存在例外，例如传给 [`applyFlagSettings()`](#applyflagsettings) 的 `effortLevel`。

### `PermissionMode`

```typescript theme={null}
type PermissionMode =
  | "default" // Standard permission behavior
  | "acceptEdits" // Auto-accept file edits
  | "bypassPermissions" // Bypass permission checks; explicit ask rules still prompt
  | "plan" // Planning mode - explore without editing
  | "dontAsk" // Don't prompt for permissions, deny if not pre-approved
  | "auto"; // A model classifier reviews actions such as shell commands and network requests
```

### `CanUseTool`

用于控制工具使用的自定义权限函数类型。

该函数是 SDK 中交互式权限提示的替代方案：仅当[权限评估流程](https://code.claude.com/docs/zh-CN/agent-sdk/permissions.md#how-permissions-are-evaluated)的结果为提示时才会调用它。已被 `allowedTools` 条目、设置中的允许规则或权限模式（例如 `acceptEdits` 或 `bypassPermissions`）批准的工具调用永远不会调用它。要对每个工具调用进行把关，请改用 [`PreToolUse` hook](https://code.claude.com/docs/zh-CN/agent-sdk/hooks)。

允许规则不会预先批准[任何模式都不会自动批准的操作](https://code.claude.com/docs/zh-CN/permission-modes.md#actions-no-mode-auto-approves)；有关其中哪些操作会到达回调，以及在 `dontAsk` 和 `auto` 模式下会发生什么，请参阅[权限的评估方式](https://code.claude.com/docs/zh-CN/agent-sdk/permissions.md#how-permissions-are-evaluated)。

```typescript theme={null}
type CanUseTool = (
  toolName: string,
  input: Record<string, unknown>,
  options: {
    signal: AbortSignal;
    suggestions?: PermissionUpdate[];
    blockedPath?: string;
    mcpServer?: { name: string; source: string };
    decisionReason?: string;
    defaultToNo?: boolean;
    suppressAlwaysAllowRule?: boolean;
    toolUseID: string;
    agentID?: string;
    requestId: string;
  }
) => Promise<PermissionResult | null>;
```

| 选项 | 类型 | 描述 |
| :- | :- | :- |
| `signal` | `AbortSignal` | 当操作应被中止时发出信号 |
| `suggestions` | [`PermissionUpdate`](#permissionupdate)`[]` | 建议的权限更新，使用户不会因此工具再次收到提示。Bash 提示包含一个[目标](#permissionupdatedestination)为 `localSettings` 的建议，因此在 `updatedPermissions` 中返回它会将规则写入 `.claude/settings.local.json`，并在会话之间持久保留。 |
| `blockedPath` | `string` | 触发权限请求的文件路径（如适用） |
| `mcpServer` | `{ name: string; source: string }` | 对于 `mcp__*` 工具，表示提供该工具的 MCP 服务器以及该服务器定义的来源，字段与 [`McpServerProvenance`](#mcpserverprovenance) 相同。其他工具不包含此项。需要 Agent SDK v0.3.274 或更高版本 |
| `decisionReason` | `string` | 说明触发此权限请求的原因 |
| `defaultToNo` | `boolean` | 为 `true` 时，一次误按键不得批准此请求：打开提示时应定位在拒绝选项上，不要预先选中批准，也不要提供单键批准快捷方式。需要 Agent SDK v0.3.268 或更高版本 |
| `suppressAlwaysAllowRule` | `boolean` | 为 `true` 时，不要为此请求提供持久的"始终允许"选项。需要 Agent SDK v0.3.268 或更高版本 |
| `toolUseID` | `string` | 此特定工具调用在助手消息中的唯一标识符 |
| `agentID` | `string` | 如果在子代理中运行，则为该子代理的 ID |
| `requestId` | `string` | `control_request` 信封的 `request_id`。您的应用程序在 SDK 之外发送的 `control_response`（例如签名的 HTTP POST）必须回传此值，以便 Claude Code 进程能够将回复与请求匹配 |

回调通常通过返回 [`PermissionResult`](#permissionresult) 来处理请求，SDK 会将其作为 `control_response` 通过其传输通道写回。仅当您的应用程序已通过自己的通道为此请求发送了 `control_response`（并回传了 `requestId`）时，才返回 `null`；此时 SDK 会跳过向其传输通道写入响应。在任何其他情况下返回 `null` 都会使工具调用无限期阻塞，因为永远不会发送 `control_response`，而权限提示不会超时。

`requestId` 选项和 `null` 返回值需要 Claude Code v2.1.199 或更高版本。

### `PermissionResult`

权限检查的结果。

```typescript theme={null}
type PermissionResult =
  | {
      behavior: "allow";
      updatedInput?: Record<string, unknown>;
      updatedPermissions?: PermissionUpdate[];
      toolUseID?: string;
    }
  | {
      behavior: "deny";
      message: string;
      interrupt?: boolean;
      toolUseID?: string;
    };
```

如果您返回 `allow` 且 `updatedPermissions` 中的项超过 4,096 个，Claude Code 会将该结果视为无效并拒绝工具调用。每个更新计为一项，其 `rules` 或 `directories` 列表中的每个条目也各计为一项。拒绝消息以 `updatedPermissions may hold at most 4096 updates, rules and directories in all.` 结尾。要使调用被允许，请返回不超过 4,096 项。

### `ToolConfig`

内置工具行为的配置。

```typescript theme={null}
type ToolConfig = {
  askUserQuestion?: {
    previewFormat?: "markdown" | "html";
  };
};
```

| 字段 | 类型 | 描述 |
| :- | :- | :- |
| `askUserQuestion.previewFormat` | `'markdown' \| 'html'` | 启用 [`AskUserQuestion`](https://code.claude.com/docs/zh-CN/agent-sdk/user-input#question-format) 选项上的 `preview` 字段并设置其内容格式。未设置时，Claude 不会生成预览 |

### `McpServerConfig`

MCP 服务器的配置。

```typescript theme={null}
type McpServerConfig =
  | McpStdioServerConfig
  | McpSSEServerConfig
  | McpHttpServerConfig
  | McpSdkServerConfigWithInstance;
```

#### `McpStdioServerConfig`

```typescript theme={null}
type McpStdioServerConfig = {
  type?: "stdio";
  command: string;
  args?: string[];
  env?: Record<string, string>;
};
```

#### `McpSSEServerConfig`

```typescript theme={null}
type McpSSEServerConfig = {
  type: "sse";
  url: string;
  headers?: Record<string, string>;
};
```

#### `McpHttpServerConfig`

```typescript theme={null}
type McpHttpServerConfig = {
  type: "http";
  url: string;
  headers?: Record<string, string>;
};
```

#### `McpSdkServerConfigWithInstance`

进程内 MCP 服务器的配置。它是 [`createSdkMcpServer()`](#createsdkmcpserver) 的返回值，外加一个由您添加的可选 `tools` 列表，其中包含 [`McpServerToolPolicy`](#mcpservertoolpolicy) 条目。

```typescript theme={null}
type McpSdkServerConfigWithInstance = {
  type: "sdk";
  name: string;
  timeout?: number;
  tools?: McpServerToolPolicy[];
  instance: McpServer;
};
```

要要求对服务器某个工具的调用进行批准或拒绝这些调用，请添加一个指明该工具的策略条目。将 `createSdkMcpServer()` 的返回值展开到 `mcpServers` 下的一个对象中，并将条目放在该对象的 `tools` 列表中，因为该函数的 `tools` 选项接受的是工具定义而非策略条目。策略列表需要 Agent SDK v0.3.301 或更高版本，[`McpServerToolPolicy`](#mcpservertoolpolicy) 说明了每个 `permission_policy` 值的作用。

在此脚本中，`allowedTools` 通配符预先批准了 `records` 服务器上的所有工具，但 `delete_record` 被标记为 `always_ask`，因此对它的调用仍会交给 `canUseTool`。借助 `destructiveHint: true` 注解，即使会话在 `auto` 模式下运行，调用仍会到达回调，否则自动模式[分类器](https://code.claude.com/docs/zh-CN/permission-modes.md#eliminate-prompts-with-auto-mode)可能会批准它。

```typescript theme={null}
import { query, tool, createSdkMcpServer } from "@anthropic-ai/claude-agent-sdk";
import { z } from "zod";

const deleteRecord = tool(
  "delete_record",
  "Delete a customer record by ID",
  { id: z.string() },
  async ({ id }) => ({ content: [{ type: "text", text: `Deleted record ${id}` }] }),
  { annotations: { destructiveHint: true } }
);

const records = createSdkMcpServer({ name: "records", tools: [deleteRecord] });

for await (const message of query({
  prompt: "Delete customer record 42",
  options: {
    mcpServers: {
      // Spread the returned config and add the policy list beside it
      records: {
        ...records,
        tools: [{ name: "delete_record", permission_policy: "always_ask" as const }]
      }
    },
    allowedTools: ["mcp__records__*"],
    canUseTool: async (toolName, input) => {
      console.log(`Approval requested for ${toolName}`);
      return { behavior: "allow", updatedInput: input };
    }
  }
})) {
  if (message.type === "result" && message.subtype === "success") {
    console.log(message.result);
  }
}
```

当 Claude 调用 `delete_record` 时，脚本会打印 `Approval requested for mcp__records__delete_record`。

要在会话运行期间更改或解除某个工具的策略，请将带有新 `tools` 值的服务器配置传给 [`setMcpServers()`](#mcpsetserversresult)。您传入的 `tools` 值从 Claude 下一次调用该服务器的工具时开始生效：

* **包含条目的列表**：这些条目替换当前策略
* **空列表**：清除该服务器上的所有策略
* **没有 `tools` 键**：保留当前策略

请在该调用中包含您的其他进程内服务器以及先前 `setMcpServers()` 调用添加的任何服务器，否则该调用会断开它们的连接。

#### `McpServerToolPolicy`

[`McpSdkServerConfigWithInstance`](#mcpsdkserverconfigwithinstance) 的 `tools` 列表中的一个条目，用于指定服务器的某个工具并设置对该工具调用的权限策略。该列表需要 Agent SDK v0.3.301 或更高版本以及 Claude Code v2.1.301 或更高版本。较旧的 Claude Code 可执行文件（例如您通过 `pathToClaudeCodeExecutable` 指定的可执行文件）会忽略该列表。此时 `always_ask` 和 `always_deny` 不会被强制执行，每个调用都会经过[权限流程](https://code.claude.com/docs/zh-CN/agent-sdk/permissions.md#how-permissions-are-evaluated)。

```typescript theme={null}
type McpServerToolPolicy = {
  name: string;
  permission_policy?: "always_allow" | "always_ask" | "always_deny";
  org_max_permission?: "allow" | "ask" | "blocked";
};
```

| 字段 | 类型 | 描述 |
| :- | :- | :- |
| `name` | `string` | 传给 [`tool()`](#tool) 的工具裸名称，不带 `mcp__<server>__` 前缀 |
| `permission_policy` | `'always_allow' \| 'always_ask' \| 'always_deny'` | 该工具调用的批准策略，即 [`always_ask` 或 `always_deny`](#permission-policy-values)。`always_allow` 在进程内服务器上会被忽略：要预先批准某个工具，请改为在 [`allowedTools`](#options) 中列出它 |
| `org_max_permission` | `'allow' \| 'ask' \| 'blocked'` | 由组织管理员设置的单个工具上限。在进程内服务器上会被忽略，因此请勿设置 |

##### `permission_policy` 值

选择 `always_ask` 以亲自批准每个调用，或选择 `always_deny` 以直接阻止该工具：

* **`always_ask`**：对该工具的每个调用都[需要批准](https://code.claude.com/docs/zh-CN/agent-sdk/permissions.md#how-permissions-are-evaluated)，即使有 `allowedTools` 条目或 allow 规则覆盖它。该调用会转到您的 `canUseTool` 回调或 [`permissionPromptToolName`](#options) 工具，除非它已先被处理。您加载的插件中的 [mod](https://code.claude.com/docs/zh-CN/plugins/mods/overview.md) [可以先批准它](https://code.claude.com/docs/zh-CN/permissions.md#precedence-among-rules-hooks-and-mods)。否则，在 `dontAsk` 模式下它会被拒绝，而在 [`auto`、`plan` 和 `bypassPermissions` 模式](#permission-modes-that-can-approve-an-always_ask-call)下，它可能在到达您的回调之前就被批准
* **`always_deny`**：在所有权限模式下（包括 `bypassPermissions`），对该工具的调用都[会被拒绝](https://code.claude.com/docs/zh-CN/agent-sdk/permissions.md#how-permissions-are-evaluated)，但您加载的插件中的 mod [仍然可以批准它](https://code.claude.com/docs/zh-CN/permissions.md#precedence-among-rules-hooks-and-mods)

如果您既未传入 `canUseTool` 也未传入 `permissionPromptToolName`，或将 [`permissionPrompts`](#options) 设为 `'none'`，则未被先行处理的 `always_ask` 调用会被拒绝，除非 [`PermissionRequest` hook](https://code.claude.com/docs/zh-CN/hooks#permissionrequest) 允许它。

##### 可以批准 `always_ask` 调用的权限模式

在 `auto`、`plan` 和 `bypassPermissions` 模式下，对标记为 [`always_ask`](#permission-policy-values) 的工具的调用可能在到达您的 `canUseTool` 回调之前被批准。

* **`auto`**：自动模式[分类器](https://code.claude.com/docs/zh-CN/permission-modes.md#eliminate-prompts-with-auto-mode)会审查该调用并可能允许它。要让每个调用在 `auto` 模式下都到达您的回调，请在工具的[注解](#toolannotations)中设置 `destructiveHint: true`
* **`plan`**：当会话可使用[自动模式](https://code.claude.com/docs/zh-CN/permission-modes.md#eliminate-prompts-with-auto-mode)时，对注解为 `readOnlyHint: true` 的 `always_ask` 工具的调用会转到自动模式分类器，分类器可能允许它们。对其他 `always_ask` 工具的调用会到达您的回调。要让只读工具的调用也到达您的回调，请在该工具的注解中于 `readOnlyHint: true` 之外添加 `destructiveHint: true`。或者，将 [`useAutoModeDuringPlan`](https://code.claude.com/docs/zh-CN/settings-reference#useautomodeduringplan) 设为 `false`，这会在规划期间关闭分类器
* **`bypassPermissions`**：当 [`CLAUDE_CODE_REMOTE`](https://code.claude.com/docs/zh-CN/env-vars#variables) 环境变量为 `true` 时（在[云端会话](https://code.claude.com/docs/zh-CN/claude-code-on-the-web.md)中即是如此），无论工具的注解如何，该调用都会在不经过您回调的情况下被批准。在其他任何地方，该调用都会到达您的回调。要让它在云端会话中也到达您的回调，请改为在 `default` 或 `acceptEdits` 模式下运行会话

在 `auto` 和 `plan` 模式下，请显式地将 `destructiveHint` 设为 `true`。对于 `always_ask` 调用，Claude Code 会将未设置的 `destructiveHint` 视为 `false`，尽管 [`ToolAnnotations`](#toolannotations) 下列出的默认值为 `true`。

#### `McpClaudeAIProxyServerConfig`

```typescript theme={null}
type McpClaudeAIProxyServerConfig = {
  type: "claudeai-proxy";
  url: string;
  id: string;
};
```

### `SdkPluginConfig`

在 SDK 中加载插件的配置。

```typescript theme={null}
type SdkPluginConfig = {
  type: "local";
  path: string;
  skipMcpDiscovery?: boolean;
};
```

| 字段 | 类型 | 描述 |
| :- | :- | :- |
| `type` | `'local'` | 必须为 `'local'`（目前仅支持本地插件） |
| `path` | `string` | 插件目录的路径。相对路径会在会话启动时相对于 [`cwd`](#options) 选项解析。使用 [`addPlugins`](#load-more-plugin-folders-with-addplugins) 在会话中途加载插件时，路径必须为绝对路径 |
| `skipMcpDiscovery` | `boolean` | 为 `true` 时，SDK 会从此插件加载 skill、hook、Agent 和命令，但不会读取其 `.mcp.json` 或清单中的 `mcpServers`。当您的应用程序自行管理插件的 MCP 连接时，请设置此项。 |

**示例：**

```typescript theme={null}
plugins: [
  { type: "local", path: "./my-plugin" },
  { type: "local", path: "/absolute/path/to/plugin" }
];
```

有关创建和使用插件的完整信息，请参阅[插件](https://code.claude.com/docs/zh-CN/agent-sdk/plugins.md)。

## 消息类型

### `SDKMessage`

查询返回的所有可能消息的联合类型。

```typescript theme={null}
type SDKMessage =
  | SDKAssistantMessage
  | SDKUserMessage
  | SDKUserMessageReplay
  | SDKResultMessage
  | SDKSystemMessage
  | SDKPartialAssistantMessage
  | SDKCompactBoundaryMessage
  | SDKStatusMessage
  | SDKLocalCommandOutputMessage
  | SDKHookStartedMessage
  | SDKHookProgressMessage
  | SDKHookResponseMessage
  | SDKPluginInstallMessage
  | SDKToolProgressMessage
  | SDKAuthStatusMessage
  | SDKTaskNotificationMessage
  | SDKTaskStartedMessage
  | SDKTaskProgressMessage
  | SDKTaskUpdatedMessage
  | SDKBackgroundTasksChangedMessage
  | SDKThinkingTokensMessage
  | SDKSessionStateChangedMessage
  | SDKWorkerShuttingDownMessage
  | SDKCommandsChangedMessage
  | SDKNotificationMessage
  | SDKFilesPersistedEvent
  | SDKToolUseSummaryMessage
  | SDKMemoryRecallMessage
  | SDKRateLimitEvent
  | SDKElicitationCompleteMessage
  | SDKPermissionDeniedMessage
  | SDKPromptSuggestionMessage
  | SDKAPIRetryMessage
  | SDKControlRequestProgressMessage
  | SDKModelRefusalFallbackMessage
  | SDKModelRefusalNoFallbackMessage
  | SDKMirrorErrorMessage
  | SDKInformationalMessage
  | SDKConversationResetMessage;
```

此联合类型中有三种消息用于报告控制请求进度和模型拒绝。它们都带有 `type: "system"`，因此请根据各自列出的 `subtype` 加以区分：

* `SDKControlRequestProgressMessage`，`subtype: "control_request_progress"`：报告客户端发送的长时间运行的控制请求的进度。`request_id` 将该消息与对应请求关联，`status` 为 `"started"` 或 `"api_retry"`。需要 Agent SDK v0.3.203 或更高版本。
* `SDKModelRefusalFallbackMessage`，`subtype: "model_refusal_fallback"`：当模型以拒绝结束流式输出，且该轮次在备用模型上重试时发出。携带 `original_model` 和 `fallback_model`。需要 Agent SDK v0.3.170 或更高版本。
* `SDKModelRefusalNoFallbackMessage`，`subtype: "model_refusal_no_fallback"`：当模型以拒绝结束流式输出且未进行重试时发出，例如因为未配置备用模型。携带 `original_model`。需要 Agent SDK v0.3.193 或更高版本。

### `SDKAssistantMessage`

助手响应消息。

```typescript theme={null}
type SDKAssistantMessage = {
  type: "assistant";
  uuid: UUID;
  session_id: string;
  message: BetaMessage; // From Anthropic SDK
  parent_tool_use_id: string | null;
  error?: SDKAssistantMessageError;
  aborted?: true;
  agent_id?: string;
  timestamp?: string;
  context_usage?: SDKContextUsage;
  usage_report?: SDKUsageReport;
  user_message_uuid?: string;
  user_message_uuids?: string[];
  resume_reason?: string;
};
```

`message` 字段是来自 Anthropic SDK 的 [`BetaMessage`](https://platform.claude.com/docs/en/api/messages/create)。它包含 `id`、`content`、`model`、`stop_reason` 和 `usage` 等字段。

`SDKAssistantMessageError` 是以下之一：`'authentication_failed'`、`'oauth_org_not_allowed'`、`'account_on_hold'`、`'billing_error'`、`'rate_limit'`、`'overloaded'`、`'invalid_request'`、`'model_not_found'`、`'server_error'`、`'max_output_tokens'`、`'cloud_credential_error'` 或 `'unknown'`。其中四个值的含义超出了它们的名称：

* `'model_not_found'`：选定的模型不存在或对您的账户或部署不可用
* `'overloaded'`：API 返回 529，因为服务器处于容量限制，与 `'rate_limit'` 不同，后者是针对您配额的 429
* `'account_on_hold'`：[您的账户被冻结](https://code.claude.com/docs/zh-CN/errors.md#your-account-is-on-hold)
* `'cloud_credential_error'`：Claude Code 无法在其运行的机器上获取可用的 AWS 或 Google Cloud 凭据，因此没有请求到达云提供商。通常原因是云登录已过期或从未在该机器上完成，但暂时无法访问的凭据服务会报告相同的值。请参阅[无法加载 AWS 或 Google Cloud 凭据](https://code.claude.com/docs/zh-CN/errors.md#could-not-load-aws-or-google-cloud-credentials)。需要 TypeScript Agent SDK v0.3.267 或更高版本，其中包含 Claude Code v2.1.267

当中断或中止在流完成前截断助手消息时，`aborted` 为 `true`：消息没有 `stop_reason`，内容可能在单词中间结束。该字段在正常完成的消息上不存在。它需要 Agent SDK v0.3.214 或更高版本。

`agent_id` 标识产生该消息的子代理，在主线程消息上不存在。其值等于该子代理的 [`task_started`](#sdktaskstartedmessage) 及其他任务事件上的 `task_id`，并且在子代理被[恢复](https://code.claude.com/docs/zh-CN/agent-sdk/subagents.md#resume-subagents)时保持不变。对于来自[嵌套子代理](https://code.claude.com/docs/zh-CN/sub-agents.md#let-subagents-spawn-their-own-subagents)的消息，它指向该嵌套子代理，而不是生成它的子代理。该字段需要 Agent SDK v0.3.292 或更高版本。

请通过 `agent_id` 将子代理的消息与其任务事件进行匹配，而不是将消息的 `parent_tool_use_id` 与任务事件的 `tool_use_id` 配对。当某个工具调用恢复子代理时，任务事件携带的是该调用的 `tool_use_id`，而消息保留的是最初启动该子代理的工具调用的 `parent_tool_use_id`，因此两者不再匹配。

Claude Code 会在满足 [`user_message_uuid`](#user_message_uuid) 中所述条件时，在该轮次的第一条助手消息上设置 `user_message_uuid` 和 `user_message_uuids`。当该轮次是继续一个被重启中断的轮次时，携带这些字段的助手消息还会携带 [`resume_reason`](#resume_reason)。

`timestamp` 是消息内容在生成它的进程上完成生成的 ISO 8601 时间。该值来自该机器的时钟，因此仅用于显示，不要按它排序消息。一个 API 轮可以产生多条共享 `message.id` 的助手消息，每条都有自己的 `timestamp`。当字段不存在时，回退到您收到消息的时间。

`context_usage` 是 `/context` 报告的结构化副本，类型为 [`SDKContextUsage`](#sdkcontextusage)，需要 Agent SDK v0.3.232 或更高版本。当您发送 `/context` 作为提示词时，Claude Code 将报告作为助手消息传递，其 `message.content` 包含 markdown 表格，并将 `context_usage` 附加到同一消息。Claude Code 不在任何其他助手消息上设置该字段，早期版本在没有它的情况下传递 `/context` 表格，因此当字段存在时从字段读取分解，当不存在时回退到 markdown 文本。

`usage_report` 是 `/usage` 报告的结构化副本，类型为 [`SDKUsageReport`](#sdkusagereport)，需要 Agent SDK v0.3.273 或更高版本。当您将 `/usage` 作为提示词发送时，Claude Code 会以一条助手消息的形式交付报告，其 `message.content` 包含文本。仅当会话满足以下所有条件时，它才会将 `usage_report` 附加到同一条消息上：

* 会话使用 claude.ai 凭据进行身份验证
* 凭据显示已知的套餐类型，或携带 `user:profile` 作用域
* 账户未采用按用量计费

以 `CLAUDE_CODE_OAUTH_TOKEN` 传入的 `claude setup-token` 令牌默认不符合条件，因为它只携带 `user:inference` 作用域。其他会话（例如使用 API 密钥的会话）交付的文本不带该字段，较早的版本也是如此。当该字段存在时请从中读取报告，不存在时再回退到文本。

### `SDKUserMessage`

用户输入消息。

```typescript theme={null}
type SDKUserMessage = {
  type: "user";
  uuid?: UUID;
  session_id?: string;
  agent_id?: string;
  message: MessageParam; // From Anthropic SDK
  pasted_content?: MessageParam["content"][];
  parent_tool_use_id: string | null;
  isSynthetic?: boolean;
  shouldQuery?: boolean;
  client_composed?: true;
  tool_use_result?: unknown;
  priority?: "now" | "next" | "later";
  origin?: SDKMessageOrigin;
  inline_pastes?: string[];
};
```

设置 `pasted_content` 以发送用户粘贴到您的提示词 UI 中而不是输入的内容，每个粘贴一个条目，每个条目是字符串或内容块数组。Claude Code 按顺序在输入的文本后追加每个条目的文本，并可能将每个粘贴包装在 `<pasted_content>` 标签中。除文本外的块被忽略，因此在 `message.content` 中发送图像和文档。需要 Agent SDK v0.3.277 或更高版本。

设置 `inline_pastes` 可告知 Claude Code `message.content` 中哪些部分是用户粘贴的而非键入的，每次粘贴对应一个字符串。提示词文本保留在用户放置的位置。Claude Code 可能会在原位用 `<pasted_content>` 标签包裹每个列出的粘贴内容，以便 Claude 区分粘贴的材料和用户自己的话。只有提示词最后一个文本块中的粘贴内容会被包裹。需要 TypeScript Agent SDK v0.3.280 或更高版本。

每个粘贴字段都有大小限制：

* `pasted_content`：如果条目数加上其中的内容块数超过 1,000，Claude Code 会忽略整个字段。
* `inline_pastes`：Claude Code 使用前 100 个非空条目，并忽略其余条目。

设置 `shouldQuery`、`client_composed` 或 `priority` 可以改变 Claude Code 处理您所发送消息的方式：

* `shouldQuery`：设置为 `false` 以将消息附加到会话记录中而不触发助手轮。消息被保留并合并到下一条触发轮的用户消息中。使用此方法注入上下文，例如您在带外运行的命令的输出，而无需在模型调用上花费。
* `client_composed`：设置为 `true` 以让 Claude Code 按原样传递消息文本。Claude Code 然后不展开 `@path` 或 [`@server:resource`](https://code.claude.com/docs/zh-CN/mcp#use-mcp-resources) 提及，也不运行以 `/` 开头的文本作为命令。当 [`verbatimPrompts`](#options) 选项打开时，SDK 在每条消息上设置该字段。需要 TypeScript Agent SDK v0.3.280 或更高版本和 Claude Code v2.1.248 或更高版本。
* `priority`：控制您在轮次运行期间发送的消息何时到达 Claude：
  * `'next'` 或未设置 `priority` 字段：Claude 会在同一轮次中读取该消息，时机是其正在运行的工具调用一完成之时。如果轮次先结束，该消息将开启下一轮次。
  * `'later'`：Claude Code 会暂存该消息直到轮次结束，然后将其作为新轮次发送。
  * 带有 [`origin: { kind: "human" }`](#sdkmessageorigin) 的 `'now'`：在 Claude Code v2.1.286 或更高版本上，可以在后台继续的工作会被移到后台，Claude 在同一轮次中读取该消息。可移动的工作包括 shell 命令、子代理和 MCP 工具调用。在 v2.1.287 或更高版本上，还包括 WebFetch 和 WebSearch 调用。当 Claude 只是在撰写回复，或其正在运行的工作无法移动时，Claude Code 会改为中断该轮次，Claude 接下来读取该消息。
  * 不带该 origin 的 `'now'`：Claude Code 中断该轮次，Claude 接下来读取该消息。

以下消息在轮次运行期间发送，要求 Claude 改变方向，同时不丢失仍在运行的 shell 命令：

```typescript theme={null}
const message: SDKUserMessage = {
  type: "user",
  message: { role: "user", content: "Skip the integration tests and summarize what you have so far" },
  parent_tool_use_id: null,
  priority: "now",
  origin: { kind: "human" },
};
```

由子代理生成的用户消息（例如其自身某个工具调用的 `tool_result`）会携带 `agent_id`。请参阅 [`SDKAssistantMessage`](#sdkassistantmessage)，其中定义了该字段及其版本要求。

在携带 `tool_result` 块的消息上，`tool_use_result` 是工具的结构化输出对象，而不是发送给模型的文本。其结构取决于对应 `tool_use` 块所指定的工具，因此该字段的类型为 `unknown`；内置结构列在[工具输出类型](#tool-output-types)下。以下结果需要超出其所列结构的额外处理：

* `Agent` 工具：`tool_use_result` 为 [`AgentOutput`](#agent-2)。请据此进行渲染，而不是解析 `tool_result` 文本。`completed` 结果的 `content` 包含子代理的报告；对于通过 `SubagentHandback` 工具调用提交报告的子代理，则包含一条关于该交接的简短说明来代替报告。在 Claude Code v2.1.271 或更高版本的[自动模式](https://code.claude.com/docs/zh-CN/permission-modes.md#eliminate-prompts-with-auto-mode)下，每个产生 `completed` 结果的子代理都以这种方式报告（[fork](https://code.claude.com/docs/zh-CN/sub-agents.md#fork-the-current-conversation) 除外），Claude 会以来自子代理的单独消息接收报告。
* Claude Code 为交付 `'now'` 消息而移到后台的 WebFetch 或 WebSearch 调用：携带该调用 `tool_result` 的用户消息的 `tool_use_result` 被设为 `{ detachedToolCall: true }`。该调用仍在运行，Claude 会在其完成后收到结果。该 `tool_use_id` 之后不会再有第二个 `tool_result`，因此如果您的应用为每个工具调用绘制一行，请在收到此消息时将该行标记为已移到后台。需要 Claude Code v2.1.287 或更高版本。
* 结果包含 `resource_link` 块的 MCP 工具：`tool_use_result` 是一个对象，其 `resourceLinks` 数组由 [`SDKMcpResourceLink`](#sdkmcpresourcelink) 条目组成。Claude 会在 `tool_result` 块中以一行文本的形式接收每个链接，因此请读取 `resourceLinks` 来渲染服务器返回的文件，而不是解析该文本。当结果中没有链接时以及对于来自子代理的结果，Claude Code 会省略 `resourceLinks`；每个结果最多保留 50 个链接，并在数组序列化后的 JSON 达到 64 KiB 时停止添加链接。`resourceLinks` 需要 Agent SDK v0.3.257 或更高版本。
* 返回 [`structuredContent`](#calltoolresult) 的 MCP 工具：`tool_use_result` 是一个对象，其 `structuredContent` 成员包含服务器发送的内容，`content` 成员包含 [`McpOutput`](#mcpoutput) 值。来自子代理的结果不携带 `structuredContent`。
* `structuredContent` 序列化后超过 1,048,576 个 JSON 字符的 MCP 工具：Claude Code 会从 `tool_use_result` 中去掉 `structuredContent`，并在其位置设置 `structuredContentOmitted: true`，以便您的应用能够区分被丢弃的对象与根本未发送该对象的工具。其他成员（例如 `content` 和 `resourceLinks`）会保留，Claude 收到的内容也不会改变。Claude Code v2.1.287 或更高版本会应用此上限。有两类工具有所不同：
  * 来自[进程内 SDK 服务器](https://code.claude.com/docs/zh-CN/agent-sdk/custom-tools.md)的工具不受此限制，会完整交付该对象。
  * 在 `tools/list` 条目中声明了 [MCP Apps `ui://` 资源](#mcpserverstatus)的工具，在 Claude Code v2.1.295 或更高版本上的上限为 8,388,608 个字符，而 v2.1.295 之前的版本对其不设限制。

### `SDKUserMessageReplay`

带有必需 UUID 的重放用户消息。

```typescript theme={null}
type SDKUserMessageReplay = {
  type: "user";
  uuid: UUID;
  session_id: string;
  message: MessageParam;
  parent_tool_use_id: string | null;
  isSynthetic?: boolean;
  client_composed?: true;
  tool_use_result?: unknown;
  origin?: SDKMessageOrigin;
  isReplay: true;
};
```

从会话外部注入的用户轮，其 [`origin`](#sdkmessageorigin) 类型为 `peer` 或 `channel`，无论是在活跃轮期间传递还是在会话空闲时启动新轮，都作为重放到达流。在 v2.1.207 之前，在会话空闲时传递的注入轮在流上不产生消息，仅在您重新读取会话记录时出现。

### `SDKResultMessage`

最终结果消息。

```typescript theme={null}
type SDKResultMessage =
  | {
      type: "result";
      subtype: "success";
      uuid: UUID;
      session_id: string;
      duration_ms: number;
      duration_api_ms: number;
      is_error: boolean;
      api_error_status?: number | null;
      num_turns: number;
      result: string;
      stop_reason: string | null;
      ttft_ms?: number;
      ttft_stream_ms?: number;
      time_to_request_ms?: number;
      user_message_uuid?: string;
      user_message_uuids?: string[];
      resume_reason?: string;
      local_command?: string;
      request_sent_wall_ms?: number;
      first_content_frame_ms?: number;
      first_stream_post_ms?: number;
      first_stream_post_ack_ms?: number;
      first_stream_post_queue_wait_ms?: number;
      first_stream_post_queued_behind?: "durable_post" | "ephemeral_post" | "retry_backoff" | "hold" | "none";
      first_stream_post_wall_ms?: number;
      first_text_post_ms?: number;
      first_text_post_queue_wait_ms?: number;
      first_text_post_queued_behind?: "durable_post" | "ephemeral_post" | "retry_backoff" | "hold" | "none";
      first_text_post_wall_ms?: number;
      total_cost_usd: number;
      usage: NonNullableUsage;
      modelUsage: { [modelName: string]: ModelUsage };
      permission_denials: SDKPermissionDenial[];
      queued_turn_count?: number;
      structured_output?: unknown;
      deferred_tool_use?: { id: string; name: string; input: Record<string, unknown> };
      terminal_reason?: TerminalReason;
      result_index?: number;
      fast_mode_state?: FastModeState;
      fast_mode_disabled_reason?: FastModeDisabledReason;
      origin?: SDKMessageOrigin;
    }
  | {
      type: "result";
      subtype:
        | "error_max_turns"
        | "error_during_execution"
        | "error_max_budget_usd"
        | "error_max_structured_output_retries";
      uuid: UUID;
      session_id: string;
      duration_ms: number;
      duration_api_ms: number;
      is_error: boolean;
      num_turns: number;
      stop_reason: string | null;
      total_cost_usd: number;
      usage: NonNullableUsage;
      modelUsage: { [modelName: string]: ModelUsage };
      permission_denials: SDKPermissionDenial[];
      queued_turn_count?: number;
      errors: string[];
      startup_failure_reason?: SDKStartupFailureReason;
      user_message_uuid?: string;
      user_message_uuids?: string[];
      resume_reason?: string;
      terminal_reason?: TerminalReason;
      result_index?: number;
      fast_mode_state?: FastModeState;
      fast_mode_disabled_reason?: FastModeDisabledReason;
      origin?: SDKMessageOrigin;
    };
```

结果上的多个字段除了 `subtype` 之外还提供诊断详情：

* `api_error_status`：终止对话的 API 错误的 HTTP 状态码。当轮在没有 API 错误的情况下结束时不存在或为 `null`。
* `ttft_ms`：首个 token 的时间（毫秒），在第一条完整助手消息到达时测量。仅在成功分支上存在。
* `ttft_stream_ms`：直到第一个 `message_start` 流事件（响应流打开时）的时间（毫秒）。低于 `ttft_ms`；两者之间的差距是流式传输第一条消息所花费的时间。仅在成功分支上存在。
* `time_to_request_ms`：从轮次开始到 Claude Code 发送 API 请求的时间（毫秒）。仅在成功分支上存在。需要 Claude Code v2.1.162 或更高版本。
* `user_message_uuid`：您发送的消息的 `uuid`，该轮回答了该消息。请参阅 [`user_message_uuid`](#user_message_uuid) 了解哪些结果携带它。
* `user_message_uuids`：您发送的每条消息的 `uuid`，Claude Code 在该轮中回答了这些消息。请参阅 [`user_message_uuids`](#user_message_uuids)。
* `resume_reason`：本轮次为何是继续一个被重启中断的轮次。在两个分支上都存在。请参阅 [`resume_reason`](#resume_reason)。
* `local_command`：轮分派的命令的名称，在轮由命令完成而不进入 Agent 循环的成功结果上，例如 `/compact`。名称折叠为小写字母和下划线，因此 `/reload-plugins` 报告 `reload_plugins`。MCP 服务器提供的命令和内置 `/mcp` 报告 `mcp`。您自己定义的命令报告 `custom`。参数从不包含。在进入 Agent 循环的每个轮上不存在，在运行无命令的发送上不存在。需要 Agent SDK v0.3.268 或更高版本。
* `request_sent_wall_ms`：Claude Code 分派 API 请求的纪元毫秒，用于与服务器端时间戳的连接。仅与 [`user_message_uuid`](#user_message_uuid) 一起存在，在成功结果上，其中 `is_error` 为 false，且轮发送了 API 请求。
* `first_content_frame_ms`：直到第一个 `content_block_start` 或 `content_block_delta` 流事件的时间（毫秒），计算思考块作为内容。仅在成功分支上存在，当 `is_error` 为 false 时。需要 Agent SDK v0.3.260 或更高版本。
* `first_stream_post_ms`、`first_stream_post_ack_ms`、`first_stream_post_wall_ms`：上传轮的第一个流事件的时间。Claude Code 仅在它流式传输到 claude.ai 的会话中记录它们，例如[云端会话](https://code.claude.com/docs/zh-CN/claude-code-on-the-web.md)，`query()` 产生的结果不携带它们。需要 Agent SDK v0.3.260 或更高版本。
* `usage`：仅主 Agent 循环。排除子代理和辅助模型调用，在流式输入会话中按轮计算。对于 token/成本核算，优先使用 `modelUsage`。
* `modelUsage`：在此 `query()` 调用期间通过查询管道进行的每个模型调用的每模型总计，包括主循环、子代理和内部调用（如压缩和 Workflow Agent）。该管道外的辅助调用（如权限分类器和 token 计数请求）被排除。恢复会话的调用也计算[从会话早期调用恢复的每模型总计](https://code.claude.com/docs/zh-CN/agent-sdk/cost-tracking.md#accumulate-costs-across-multiple-calls)。在流式输入会话中，总计在轮中是累积的，因此读取最新结果而不是跨结果求和。请参阅[在流式输入模式中跟踪成本](https://code.claude.com/docs/zh-CN/agent-sdk/cost-tracking.md#track-costs-in-streaming-input-mode)了解重置，以及[在会话崩溃后恢复总计](https://code.claude.com/docs/zh-CN/agent-sdk/cost-tracking.md#recover-totals-after-a-session-crash)了解零化结果。
* `total_cost_usd`：累积估计成本（美元），涵盖与 `modelUsage` 相同的调用并在相同点重置。恢复会话的调用也计算[从会话早期调用恢复的总计](https://code.claude.com/docs/zh-CN/agent-sdk/cost-tracking.md#accumulate-costs-across-multiple-calls)。这是一个估计值，不是账单声明。请参阅[跟踪成本和使用情况](https://code.claude.com/docs/zh-CN/agent-sdk/cost-tracking.md)了解准确性注意事项。
* `queued_turn_count`：您发送的带有 `origin: { kind: "human" }` 的消息数，在 Claude Code 产生结果时仍在等待。请参阅 [`queued_turn_count`](#queued_turn_count) 了解 `0` 和缺失字段告诉您什么。
* `result_index`：此结果在运行的传递顺序中的位置，从 0 开始计算，跨越进程写入的每个结果。在两个分支上存在。写入失败的结果仍然消耗其编号，因此序列中的间隙意味着结果丢失。需要 Agent SDK v0.3.268 或更高版本。
* `startup_failure_reason`：Claude Code 拒绝启动的原因，在它在已知启动失败时退出前写入的 `error_during_execution` 结果上。请参阅 [`startup_failure_reason`](#startup_failure_reason) 了解值以及哪些失败携带它。需要 Agent SDK v0.3.274 或更高版本。
* `terminal_reason`：循环结束的原因。`"completed"`、`"max_turns"`、`"tool_deferred"`、`"aborted_streaming"`、`"aborted_tools"`、`"hook_stopped"`、`"stop_hook_prevented"`、`"background_requested"`、`"blocking_limit"`、`"rapid_refill_breaker"`、`"prompt_too_long"`、`"image_error"`、`"model_error"`、`"api_error"`、`"malformed_tool_use_exhausted"`、`"budget_exhausted"`、`"structured_output_retry_exhausted"`、`"tool_deferred_unavailable"` 或 `"turn_setup_failed"` 之一。
* `fast_mode_state`：`"on"`、`"off"` 或 `"cooldown"` 之一。
* `fast_mode_disabled_reason`：[快速模式](https://code.claude.com/docs/zh-CN/fast-mode.md)现在不可用的原因。当没有任何东西阻止快速模式时不存在，尽管请求仍可能以标准速度运行。在快速模式速率限制后的冷却期间，Claude Code 报告 `fast_mode_state: "cooldown"` 且没有原因代码，并在冷却期过期时重新启用快速模式。需要 Claude Code v2.1.219 或更高版本。

使用原因代码在您自己的 UI 中解释为什么快速模式关闭，而不是重新推导可用性。每个代码命名阻止快速模式的检查：

| 原因代码 | 含义 |
| - | - |
| `free` | 账户没有快速模式需要的付费订阅或使用额度 |
| `preference` | 组织已禁用快速模式 |
| `extra_usage_disabled` | 账户的使用额度已关闭 |
| `network_error` | [可用性检查](https://code.claude.com/docs/zh-CN/fast-mode.md#use-fast-mode-behind-proxies-and-llm-gateways)无法到达 `api.anthropic.com` |
| `unknown` | Claude Code 无法确定可用性 |
| `not_first_party` | 会话使用 Anthropic API 以外的提供商 |
| `disabled_by_env` | [`CLAUDE_CODE_DISABLE_FAST_MODE`](https://code.claude.com/docs/zh-CN/env-vars) 已设置 |
| `model_not_allowed` | 快速模式 Opus 模型不在组织的 [`availableModels`](https://code.claude.com/docs/zh-CN/model-config#restrict-model-selection) 允许列表中 |
| `sdk_opt_in_required` | 会话未选择加入快速模式：在 [`settings`](#options) 选项中或通过 [`applyFlagSettings()`](#applyflagsettings) 传递 `fastMode: true` |
| `pending` | 可用性检查尚未完成 |

相同的字段对出现在 [`SDKSystemMessage`](#sdksystemmessage) 和 [`SDKControlInitializeResponse`](#sdkcontrolinitializeresponse) 上，因此您可以在第一轮之前读取快速模式状态。

`origin` 字段转发触发此结果的用户消息的 [`SDKMessageOrigin`](#sdkmessageorigin)。携带 `origin: { kind: "task-notification" }` 的结果包括：

* **合成的后续轮次**：当 SDK 注入一个合成的后续轮次时（例如针对已完成的后台任务），所产生的 `SDKResultMessage` 会携带此 kind
* **已触发的定时任务以及来自其他会话的消息**：触发器已触发的云端定时任务，以及来自您其他会话且经服务器验证的消息，也会以此 kind 到达，各自带有[任务通知 subkind](#task-notification-subkinds) 中所述的 `subkind`

在路由或抑制结果之前，请检查 `kind` 以区分回答您提示词的结果和注入的后续结果。如果您的应用程序[声明了计划运行](#declare-a-scheduled-run)，其结果也会携带 `kind: "task-notification"`，因此不要仅凭 `kind` 进行抑制。

当多个后台任务完成一起排队时，Claude Code 可以在一轮中回答它们，而不是每个一轮。每个完成仍然产生自己的结果与此来源。Claude Code 一起回答的完成中除最后一个外的所有完成产生空结果，其中 `num_turns: 0`，按顺序，最后一个的结果携带回答它们全部的轮。

该字段在任何用户轮之前发出的结果上不存在，例如启动错误。

当 `PreToolUse` hook 返回 `permissionDecision: "defer"` 时，结果具有 `stop_reason: "tool_deferred"` 和 `deferred_tool_use` 携带待处理工具的 `id`、`name` 和 `input`。读取此字段以在您自己的 UI 中显示请求，然后使用相同的 `session_id` 恢复以继续。请参阅[延迟工具调用以供稍后使用](https://code.claude.com/docs/zh-CN/hooks.md#defer-a-tool-call-for-later)了解完整往返。

#### `user_message_uuid`

轮回答的 [`SDKUserMessage`](#sdkusermessage) 的 `uuid`，回显以便您可以将 Claude Code 的回复与您发送的消息匹配。Claude Code 仅在您在消息上设置 uuid 时回显 `uuid`。该字段在 `SDKUserMessage` 上是可选的，传递给 `query()` 的字符串提示词不携带任何。

轮回答的消息取决于轮如何启动：

* **您发送的常规消息**，意思是没有 `isSynthetic: true` 的消息：轮在其整个运行中回答该消息。当您一起发送多条消息时，Claude Code 可以将它们合并为一轮，该字段然后仅携带最后一条消息的 `uuid`。要将回复与任何合并的消息匹配，请使用 [`user_message_uuids`](#user_message_uuids)。
* **您发送的带有 `isSynthetic: true` 的消息**：轮最初回答该消息。如果 Claude Code 在工具调用之间拾取您的常规消息，轮从那时起回答拾取的消息。回显合成消息的 `uuid` 需要 Agent SDK v0.3.265 或更高版本；早期版本在合成轮上不回显任何内容。
* **Claude Code 在 [`CLAUDE_CODE_RESUME_INTERRUPTED_TURN`](https://code.claude.com/docs/zh-CN/env-vars) 下为继续被中断的轮次而生成的提示词**：当被中断轮次的最后一个提示词是您发送的常规消息时（无论它是开启了该轮次，还是 Claude Code 在轮次期间接收的），继续的轮次起初回答该消息。[`resume_reason`](#resume_reason) 可将继续轮次的帧与被中断尝试的帧区分开来。当最后一个提示词不是您的常规消息时，继续的轮次起初不回答您的任何消息。如果 Claude Code 在工具调用之间接收了您的一条常规消息，则从那时起该轮次回答被接收的消息。回显被中断轮次的提示词需要 Agent SDK v0.3.268 或更高版本。
* **Claude Code 自己生成的任何其他提示词**：轮最初不回答您的任何消息，其帧不携带回显。如果 Claude Code 在工具调用之间拾取您的常规消息，轮从那时起回答该消息。拾取回显需要 Agent SDK v0.3.265 或更高版本；早期版本在这些轮上不回显任何内容。

Claude Code 在三种帧上回显回答的消息的 `uuid`：

* **结果**：回答您发送的消息的轮的每个结果。在 Agent SDK v0.3.265 或更高版本上，每个这样的结果都携带它。在 v0.3.265 之前，常规消息启动的轮的成功结果在轮未发送 API 请求或以延迟工具调用结束时缺少它。在 v0.3.246 之前，错误结果也缺少它，在 v0.3.216 之前每个结果都缺少它。
* **轮的第一个回复**：第一条[助手消息](#sdkassistantmessage)，以及在使用 `includePartialMessages` 时第一条 `event.type` 不是 `ping` 的[流事件](#sdkpartialassistantmessage)，因此您可以在结果到达之前绑定回复。第一个回复回显需要 Agent SDK v0.3.246 或更高版本。在 v0.3.269 之前，使用 `includePartialMessages` 时，Claude Code 仅在该第一条流事件上设置它，或者在轮未流式传输任何内容时在第一条助手消息上设置它。当轮回答的消息在轮中间改变时，改变后的第一个回复也携带该字段，在 Agent SDK v0.3.265 或更高版本上；早期版本在每轮一个回复帧上设置它。
* **轮的每个 [`thinking_tokens`](#sdkthinkingtokensmessage) 帧**：以便您可以将思考进度归因于您发送的消息，而无需等待轮的第一个回复。需要 Agent SDK v0.3.260 或更高版本。

Claude Code 在这些情况下省略该字段：

* 除了那些第一个回复之外的回复帧
* 子代理帧
* 不回答您任何消息的轮，或回答您发送的没有 `uuid` 的消息的轮
* 不回答您发送的任何消息的结果，例如崩溃的工作进程后的零化结果

当 [`UserPromptSubmit`](https://code.claude.com/docs/zh-CN/hooks#userpromptsubmit) hook 阻止了一条开启自身轮次的消息，或以 `continue: false` 停止它时，您仍会收到该消息的结果。它是 `is_error: false` 且 `num_turns: 0` 的成功结果，其 `result` 为[阻止消息](https://code.claude.com/docs/zh-CN/hooks.md#what-a-blocked-prompt-leaves-behind)或[停止消息](https://code.claude.com/docs/zh-CN/hooks.md#json-output)，并按照与其他结果相同的规则回显该消息的 `uuid`。该结果没有 `terminal_reason`，除 `result` 文本外，结果上没有任何内容表明是 hook 结束了该轮次。在另一个轮次运行期间被 hook 阻止的消息可能不会得到结果：请参阅 [`user_message_uuids`](#user_message_uuids)。

#### `user_message_uuids`

Claude Code 在该轮中回答的您发送的每条消息的 `uuid`。当您一起发送多条消息时，Claude Code 可以将它们合并为一轮，`user_message_uuid` 然后仅命名其中的最后一个。要将回复与任何合并的消息匹配，在此列表中的任何位置查找该消息的 `uuid`。需要 Agent SDK v0.3.259 或更高版本。

Claude Code 在携带 `user_message_uuid` 的每个回复帧和结果上，与该字段一起设置此列表。有关回显所回答消息 `uuid` 的完整轮帧集合以及每种帧所需的版本，请参阅 [`user_message_uuid`](#user_message_uuid)。列表始终包含 `user_message_uuid` 并最多包含 64 个条目。

当 Claude Code 接收您在轮次运行期间发送的常规消息时，会将该消息的 `uuid` 添加到结果的列表中。当 [`UserPromptSubmit`](https://code.claude.com/docs/zh-CN/hooks#userpromptsubmit) hook 在接收时阻止了该消息，Claude Code 会改为丢弃它：该消息永远不会到达 Claude，也不会为后续轮次保留。您不会收到它的结果，它的 `uuid` 也不会出现在任何结果的列表中。如果 hook 可能阻止消息，请为等待列出您在轮次运行期间所发送消息的结果设置超时。

以上情况适用于 hook 的输出仅为阻止时。如果输出设置了 `continue: false`（即使同时带有 `decision: "block"`），或者该 hook 是一个回答 [`ok: false`](https://code.claude.com/docs/zh-CN/hooks#response-schema) 且未设置 `continueOnBlock: true` 的提示词 hook，Claude Code 会停止该消息而不是丢弃它。该消息会等待自己的轮次，您会收到它的结果，其文本保留在对话中，因此 Claude 会在下一轮次看到它。

当第一个回复或结果携带 `user_message_uuid` 而没有列表时，它来自早期的 Claude Code 版本，因此回退到单个字段。

#### `resume_reason`

本轮次为何是继续一个被重启中断的轮次。Claude Code 会在 [`CLAUDE_CODE_RESUME_INTERRUPTED_TURN`](https://code.claude.com/docs/zh-CN/env-vars) 下继续被中断轮次的轮次上设置此字段，以便您将继续轮次的回复和结果与被中断尝试的回复和结果区分开来。需要 Agent SDK v0.3.268 或更高版本。

Claude Code 在两种帧上设置该字段：

* **继续轮次的结果**：在 success 和 error 分支上均设置，无论结果是否携带 `user_message_uuid`。
* **继续轮次的回复帧**：携带 [`user_message_uuid`](#user_message_uuid) 的那些帧。

其值是一个简短的小写标记，例如 `interrupted_turn`。

#### `queued_turn_count`

您发送的带有 [`origin: { kind: "human" }`](#sdkmessageorigin) 的消息数，在 Claude Code 产生结果时仍在命令队列中等待。需要 Agent SDK v0.3.242 或更高版本。

`0` 和缺失字段告诉您什么：

* **`0`**：Claude Code 不计算您发送的没有该 `origin` 的消息，也不计算任务通知，因此轮仍然可以跟随。
* **缺失**：Claude Code 在崩溃或致命启动错误后发出的最终结果省略该字段，并[可能携带零化总计](https://code.claude.com/docs/zh-CN/agent-sdk/cost-tracking.md#recover-totals-after-a-session-crash)。

#### `startup_failure_reason`

Claude Code 拒绝启动的原因，以便您的应用程序可以提供修复而不是重试。Claude Code 在它在已知启动失败时退出前写入的 `error_during_execution` 结果上设置它。该结果携带零化总计，其 `errors` 数组携带与 stderr 相同的文本。该字段在所有其他结果上不存在。需要 Agent SDK v0.3.274 或更高版本。

在 [`env`](#options) 中设置 `CLAUDE_CODE_STARTUP_FAILURE_RESULTS` 为 `1` 以接收每个 `SDKStartupFailureReason` 值的此结果。没有该变量，Claude Code 仅为这些失败写入结果，其余的以 stderr 输出、非零退出和无结果消息结束：

* Claude Code 停止的恢复，因为它[无法将会话返回到其 worktree](https://code.claude.com/docs/zh-CN/worktrees.md#the-session-resumes-outside-its-worktree)，带有 `worktree_unverified` 或 `worktree_resume_refused`。该部分说明哪个错误携带哪个值。
* 拒绝后台会话持有的对话的 [`continue`](#options)，带有 `session_held_by_background`。对于这样的对话的拒绝 [`resume`](#options)，Claude Code 仅在设置了变量时写入结果。

```typescript theme={null}
type SDKStartupFailureReason =
  | "org_pin_api_key_conflict"
  | "provider_not_allowed"
  | "org_verify_failed"
  | "org_pin_mismatch"
  | "managed_settings_invalid"
  | "remote_settings_required_unavailable"
  | "gateway_signin_required"
  | "gateway_access_denied"
  | "proxy_invalid"
  | "temp_dir_unusable"
  | "cwd_unavailable"
  | "shell_tool_missing"
  | "session_held_by_background"
  | "worktree_resume_refused"
  | "worktree_unverified"
  | "cli_version_too_old"
  | "bypass_root"
  | "org_config_required_unavailable"
  | "org_config_refused";
```

每个值命名一个拒绝：

| 值 | 停止会话的原因 |
| :- | :- |
| `org_pin_api_key_conflict` | 托管设置[需要第一方或 Cloud 网关登录](https://code.claude.com/docs/zh-CN/authentication.md#restrict-login-to-your-organization)，并配置了 Anthropic API 密钥、身份验证令牌或 `apiKeyHelper` |
| `provider_not_allowed` | 托管设置[列出此机器可能使用的 API 提供商](https://code.claude.com/docs/zh-CN/settings-reference.md#allowedproviders)，会话设置为不在列表中的提供商，或设置未固定的端点。需要 Claude Code v2.1.285 或更高版本 |
| `org_verify_failed` | 登录的组织无法针对 pin 进行验证，例如由于网络故障或已撤销的令牌 |
| `org_pin_mismatch` | 登录属于 pin 不允许的组织 |
| `managed_settings_invalid` | 无法读取托管策略设置，pin 未命名任何组织，或[托管模型限制](https://code.claude.com/docs/zh-CN/errors.md#managed-settings-block-the-default-model)为默认选项留下没有允许的模型 |
| `remote_settings_required_unavailable` | 组织需要的托管设置无法加载 |
| `gateway_signin_required` | [Cloud 网关](https://code.claude.com/docs/zh-CN/claude-apps-gateway)结束了此登录 |
| `gateway_access_denied` | 对 Cloud 网关的托管设置请求返回 403，网关的[故障排除表](https://code.claude.com/docs/zh-CN/claude-apps-gateway-deploy.md#troubleshooting)涵盖了这一点 |
| `proxy_invalid` | 代理设置不是完整的 URL |
| `temp_dir_unusable` | 每用户临时目录不安全或无法创建 |
| `cwd_unavailable` | 工作目录被删除、移动或无法读取 |
| `shell_tool_missing` | 在 Windows 上，没有可用的 shell 工具：Git Bash 缺失，PowerShell 缺失或使用 `CLAUDE_CODE_USE_POWERSHELL_TOOL` 关闭 |
| `session_held_by_background` | 要恢复或继续的对话作为[后台会话](https://code.claude.com/docs/zh-CN/agent-view.md)运行 |
| `worktree_resume_refused` | 会话的 worktree 未通过其安全检查，或恢复是从其内部启动的。`errors` 说明运行相同恢复是否继续而不使用 worktree |
| `worktree_unverified` | 会话的 worktree 现在无法验证，重试可能成功 |
| `cli_version_too_old` | 此 Claude Code 版本低于 Anthropic 需要的最低版本 |
| `bypass_root` | 在以 root 身份运行时请求了绕过权限模式 |
| `org_config_required_unavailable` | 会话在启动前需要组织的策略和托管设置，但无法加载它们，例如由于网络故障或 Anthropic 服务器错误。需要 Agent SDK v0.3.293 或更高版本 |
| `org_config_refused` | Anthropic 拒绝为此次登录提供组织的策略和托管设置，例如因为登录已过期或被吊销，或组织不允许此账户使用 Claude Code。需要 Agent SDK v0.3.293 或更高版本 |

### `SDKSystemMessage`

系统初始化消息。

```typescript theme={null}
type SDKSystemMessage = {
  type: "system";
  subtype: "init";
  uuid: UUID;
  session_id: string;
  agents?: string[];
  apiKeySource: ApiKeySource;
  betas?: string[];
  claude_code_version: string;
  cwd: string;
  tools: string[];
  mcp_servers: {
    name: string;
    status: string;
    source?: string;
  }[];
  model: string;
  permissionMode: PermissionMode;
  slash_commands: string[];
  terminal_slash_commands?: string[];
  output_style: string;
  skills: string[];
  plugins: { name: string; path: string }[];
  plugin_errors?: {
    plugin: string;
    type: string;
    message: string;
    path?: string;
  }[];
  fast_mode_state?: FastModeState;
  fast_mode_disabled_reason?: FastModeDisabledReason;
  effort?: "low" | "medium" | "high" | "xhigh" | "max" | null;
  capabilities?: string[];
};
```

`fast_mode_state` 报告会话的[快速模式](https://code.claude.com/docs/zh-CN/fast-mode.md)状态。当某些东西阻止快速模式时，`fast_mode_disabled_reason` 命名阻止它的检查；该字段需要 Claude Code v2.1.219 或更高版本。对于原因代码及其含义，请参阅结果消息上的 [`fast_mode_disabled_reason`](#sdkresultmessage)。

`terminal_slash_commands` 命名 `slash_commands` 中的条目，其接口绑定到本地终端，例如 `exit`。您可以像 `slash_commands` 中的任何其他条目一样发送它们；该字段存在以便远程或移动客户端可以从其命令菜单中隐藏它们。该字段仅在非空时存在，需要 Agent SDK v0.3.229 或更高版本。

* `source` 在每个 `mcp_servers` 条目上：服务器定义的来源，与 [`McpServerStatus`](#mcpserverstatus) 的 `source` 值相同。需要 Agent SDK v0.3.274 或更高版本。
* `effort`：为会话下一轮次设置的 [effort 级别](https://code.claude.com/docs/zh-CN/model-config.md#adjust-effort-level)，当 Claude Code 不发送任何级别时为 `null`。Claude Code 仅在发送给 [Remote Control](https://code.claude.com/docs/zh-CN/remote-control.md) 客户端的 init 消息上设置该字段，并在您的应用读取的 init 消息中省略它。需要 Agent SDK v0.3.234 或更高版本。

`capabilities` 数组命名此 CLI 实现的协议行为，因此您可以进行功能检测而不是比较 `claude_code_version` 字符串。这是一个开放集：忽略您不认识的值，并检查您依赖其行为的特定功能。该字段需要 Claude Code v2.1.205 或更高版本，在早期 CLI 上不存在。

| 功能 | 含义 |
| - | - |
| `interrupt_receipt_v1` | [`interrupt()`](#query-object) 使用 [`SDKControlInterruptResponse`](#sdkcontrolinterruptresponse) 收据解析，列出中断到达时待处理的消息 |
| `interrupt_cancel_queued_v1` | `interrupt` 控制请求尊重 `cancel_queued: true`，取消收据在 `still_queued` 下列出的消息，并改为在 `cancelled` 下列出它们。请参阅 [`SDKControlInterruptResponse`](#sdkcontrolinterruptresponse)。需要 Claude Code v2.1.219 或更高版本 |
| `sdk_mcp_manifests` | `initialize` 控制请求接受 `sdkMcpServerManifests`，即从您的进程内 [SDK MCP 服务器](https://code.claude.com/docs/zh-CN/agent-sdk/custom-tools.md)捕获的 MCP 握手结果。Claude Code 在 v2.1.286 或更高版本中公布此能力 |
| `sdk_mcp_tools_list_changed` | 来自 [SDK MCP 服务器](https://code.claude.com/docs/zh-CN/agent-sdk/custom-tools.md)的 `tools/list_changed` 通知会使 Claude Code 重新列出该服务器的工具，因此服务器在会话中途添加的工具能够到达 Claude。Claude Code 在 v2.1.286 或更高版本中公布此能力 |

`plugin_errors` 数组列出插件加载失败。一个条目描述要么是未加载的插件且在 `plugins` 中不存在，要么是加载但没有其部分之一（例如其 hooks 文件）的插件。当没有任何东西失败时，该键被省略。`SDKSystemMessage` 在 Agent SDK v0.3.283 或更高版本中声明 `plugin_errors`。

当您的 [`plugins` 选项](#options)中的目录或存档本身无法加载时，条目的 `plugin` 字段保存位置标签（例如 `inline[0]`）而不是插件名称。例如，当路径不存在或清单无效时会发生这种情况。通过其 `path` 字段将这样的条目与您的选项匹配。

下表列出了每个 `plugin_errors` 条目的字段。

| 字段 | 类型 | 描述 |
| - | - | - |
| `plugin` | `string` | 失败插件的 ID，或位置标签（例如 `inline[0]`），当插件目录或存档本身无法加载时 |
| `type` | `string` | 来自开放集的错误类别，例如 `path-not-found` 或 `manifest-validation-error`。将您不认识的值视为通用失败 |
| `message` | `string` | 描述失败的显示文本 |
| `path` | `string` | 仅当插件目录或存档本身无法加载时存在。其绝对路径，相对路径从您的 `plugins` 选项针对 [`cwd`](#options) 选项解析 |

### `SDKPartialAssistantMessage`

流式部分消息（仅当 `includePartialMessages` 为 true 时）。

`parent_tool_use_id` 字段始终为 `null`：流事件仅针对主会话发出。要进行子代理归属，请使用完整消息（它们携带 [`agent_id`](#sdkassistantmessage) 和 `parent_tool_use_id`），或启用 [`forwardSubagentText`](#options) 以完整消息的形式接收子代理的文本和思考内容。

```typescript theme={null}
type SDKPartialAssistantMessage = {
  type: "stream_event";
  event: BetaRawMessageStreamEvent; // From Anthropic SDK
  parent_tool_use_id: string | null;
  uuid: UUID;
  session_id: string;
  ttft_ms?: number; // Time to first token in ms, present only on message_start events
  user_message_uuid?: string;
  user_message_uuids?: string[];
  resume_reason?: string;
};
```

Claude Code 会在满足 [`user_message_uuid`](#user_message_uuid) 中所述条件时，在该轮次的第一个非 ping 流事件上设置 `user_message_uuid` 和 `user_message_uuids`，并在轮次所回答的消息发生变化时再次设置。当该轮次是继续一个被重启中断的轮次时，携带这些字段的流事件还会携带 [`resume_reason`](#resume_reason)。

### `SDKCompactBoundaryMessage`

指示对话压缩边界的消息。

```typescript theme={null}
type SDKCompactBoundaryMessage = {
  type: "system";
  subtype: "compact_boundary";
  uuid: UUID;
  session_id: string;
  compact_metadata: {
    trigger: "manual" | "auto";
    pre_tokens: number;
  };
};
```

### `SDKInformationalMessage`

循环发出的通用文本横幅。携带 Claude Code 引发的警告、通知和其他非错误状态行，以及 hook 反馈，例如 `UserPromptSubmit` hook 的阻止原因。

在 Claude Code v2.1.227 或更高版本上，hook 的 [`systemMessage`](https://code.claude.com/docs/zh-CN/hooks#json-output) 可以作为此消息到达，每行以 hook 的名称为前缀，例如 `PostToolUse:Bash says:`。hooks 页面上每个[事件的部分](https://code.claude.com/docs/zh-CN/hooks.md#hook-events)说明输出如何显示。

将 `content` 呈现为给定 `level` 的纯文本。

```typescript theme={null}
type SDKInformationalMessage = {
  type: "system";
  subtype: "informational";
  content: string;
  level: "info" | "notice" | "suggestion" | "warning";
  tool_use_id?: string;
  prevent_continuation?: boolean;
  uuid: UUID;
  session_id: string;
};
```

### `SDKWorkerShuttingDownMessage`

在优雅的工作进程拆卸上发出，以便远程客户端可以显示工作进程退出的原因，而不是等待心跳超时。`reason` 是由主机 CLI 设置的短 snake\_case 字符串，例如 `"host_exit"` 或 `"remote_control_disabled"`。仅在实时流式传输时对此采取行动。恢复的会话重放此消息的过去实例，因此在这种情况下忽略它们。

```typescript theme={null}
type SDKWorkerShuttingDownMessage = {
  type: "system";
  subtype: "worker_shutting_down";
  reason: string;
  uuid: UUID;
  session_id: string;
};
```

### `SDKPluginInstallMessage`

插件安装进度事件。在设置 [`CLAUDE_CODE_SYNC_PLUGIN_INSTALL`](https://code.claude.com/docs/zh-CN/env-vars) 时发出，以便您的 Agent SDK 应用程序可以在第一轮之前跟踪市场插件安装。`started` 和 `completed` 状态括住整体安装。`installed` 和 `failed` 状态报告单个市场并包含 `name`。

```typescript theme={null}
type SDKPluginInstallMessage = {
  type: "system";
  subtype: "plugin_install";
  status: "started" | "installed" | "failed" | "completed";
  name?: string;
  error?: string;
  uuid: UUID;
  session_id: string;
};
```

### `SDKPermissionDeniedMessage`

当权限系统在没有交互式提示的情况下拒绝工具调用时发出的流事件。使用它在您的 UI 中实时呈现拒绝，而不是仅观察随后的 `is_error` 工具结果。它报告的拒绝取决于运行如何处理权限提示：

* **使用 [`canUseTool`](#canusetool) 回调**和默认 [`permissionPrompts: 'host'`](#options)：权限提示转到您的回调，此事件报告 Claude Code 自己决定的拒绝，而不调用它。
* **两者都不使用**：裸 `-p` 运行，或既未设置 `canUseTool` 也未设置 `permissionPromptToolName` 的 `query()`，会拒绝任何本应触发提示的工具调用，除非 [`PermissionRequest` hook](https://code.claude.com/docs/zh-CN/hooks-guide#limitations) 允许它；此事件会报告这些拒绝以及 Claude Code 自行决定的拒绝。在 v2.1.223 之前，Claude Code 在没有回调的运行中不会发出此事件。
* **使用 MCP 提示工具**，使用 `permissionPromptToolName` 或 [`--permission-prompt-tool`](https://code.claude.com/docs/zh-CN/cli-reference#cli-flags) 标志设置，和默认 `permissionPrompts: 'host'`：Claude Code 根本不发出此事件，甚至不发出它自己决定的规则拒绝。
* **使用 [`permissionPrompts: 'none'`](#options)**：Claude Code 拒绝会提示的调用，即使也设置了 `canUseTool` 或 MCP 提示工具，此事件也报告这些拒绝以及 Claude Code 自己决定的拒绝。需要 Claude Code v2.1.259 或更高版本。

在每个配置中，此事件跳过在 `PreToolUse` hook 路径上决定的任何拒绝，无论 hook 本身拒绝了调用还是拒绝规则覆盖了 hook 的允许或询问决定。该事件也是尽力而为的：偶尔 Claude Code 记录拒绝而不发出此事件，因此[结果消息](#sdkresultmessage)上的 `permission_denials` 是权威记录。

```typescript theme={null}
type SDKPermissionDeniedMessage = {
  type: "system";
  subtype: "permission_denied";
  tool_name: string;
  tool_use_id: string;
  agent_id?: string;
  decision_reason_type?: string;
  decision_reason?: string;
  message: string;
  uuid: UUID;
  session_id: string;
};
```

| 字段 | 类型 | 描述 |
| - | - | - |
| `tool_name` | `string` | 被拒绝的工具的名称 |
| `tool_use_id` | `string` | 此拒绝回答的 `tool_use` 块的 ID |
| `agent_id` | `string` | 当拒绝的调用源自子代理内部时的子代理 ID。镜像主机端路由的 `can_use_tool` 上的字段 |
| `decision_reason_type` | `string` | 决定组件的鉴别器，例如 `"rule"`、`"mode"`、`"classifier"` 或 `"asyncAgent"` |
| `decision_reason` | `string` | 来自决定组件的人类可读原因，当可用时 |
| `message` | `string` | 在 `tool_result` 中返回给模型的拒绝消息 |

### `SDKPermissionDenial`

关于被拒绝的工具使用的信息。

```typescript theme={null}
type SDKPermissionDenial = {
  tool_name: string;
  tool_use_id: string;
  tool_input: Record<string, unknown>;
};
```

### `SDKContextUsage`

`/context` 报告的结构化形式，作为 `context_usage` 在传递 `/context` 结果的 [`SDKAssistantMessage`](#sdkassistantmessage) 上携带。Agent SDK v0.3.232 及更高版本导出该类型。与 [`SDKControlGetContextUsageResponse`](#sdkcontrolgetcontextusageresponse) 不同，它仅携带呈现使用情况分解所需的数据，不包含 `color` 和 `gridRows` 等显示字段。Claude Code 使用不出现在消息流中的 token 计数 API 请求计算报告；请参阅[这些请求如何处理](#sdkcontrolgetcontextusageresponse)。

```typescript theme={null}
type SDKContextUsage = {
  model: string;
  total_tokens: number;
  raw_max_tokens: number;
  percentage: number;
  over_limit?: {
    tokens_over: number;
    kind: "hard_limit" | "compaction_window";
  };
  categories: SDKContextUsageCategory[];
  mcp_tools: {
    name: string;
    server_name: string;
    tokens: number;
  }[];
  memory_files: {
    path: string;
    type: string;
    tokens: number;
  }[];
  agents: {
    agent_type: string;
    source: string;
    tokens: number;
  }[];
  skills?: {
    name: string;
    source: string;
    plugin_name?: string;
    tokens: number;
  }[];
};
```

该表列出了 Claude Code 在每个字段中放入的内容。从 `model` 到 `over_limit` 的字段描述整个会话，集合字段将 token 归因于单个项目。

| 字段 | 类型 | 描述 |
| - | - | - |
| `model` | `string` | Claude Code 计算使用情况的主循环的模型，不是子代理的 |
| `total_tokens` | `number` | Claude Code 对使用中 token 的估计。未限制在窗口，因此当会话超过限制时可以超过 `raw_max_tokens` |
| `raw_max_tokens` | `number` | 模型的上下文窗口，或较低的[自动压缩窗口](https://code.claude.com/docs/zh-CN/model-config.md#context-window-and-auto-compaction)（当适用时），例如您设置的或 Claude Code 应用于某些具有 1M token 窗口的模型的 200K 边界。Claude Code 针对此窗口测量 `total_tokens` |
| `percentage` | `number` | `total_tokens` 作为 `raw_max_tokens` 的四舍五入百分比，因此当会话超过限制时可以超过 100 |
| `over_limit` | `object` | 仅当 `total_tokens` 超过 `raw_max_tokens` 时存在。`tokens_over` 是超过的数量，`kind` 说明 Claude Code 如何解决窗口 |
| `categories` | [`SDKContextUsageCategory`](#sdkcontextusagecategory)`[]` | 使用情况按类别分解的每一行一个条目 |
| `mcp_tools` | `object[]` | 归因于每个 MCP 工具的 token，带有其线路名称（例如 `mcp__linear__create_issue`）和其 `server_name` |
| `memory_files` | `object[]` | 归因于每个加载的记忆文件的 token，带有其 `path` 和源标签（例如 `Project` 或 `User`）在 `type` 中 |
| `agents` | `object[]` | 归因于每个自定义子代理定义的 token，带有源标识符，例如 `projectSettings`、`userSettings` 或 `plugin`。内置子代理未列出 |
| `skills` | `object[]` | 归因于 skill 列表中每个 skill 的 token，带有源标识符，对于插件 skill，插件的名称在 `plugin_name` 中。当没有 skill 贡献 token 时不存在 |

`over_limit.kind` 记录 Claude Code 如何解决窗口，而不是 API 是否接受下一个请求：

* `hard_limit`：窗口是 Claude Code 认为是模型自己的限制，超过该限制 API 拒绝请求
* `compaction_window`：窗口是压缩策略窗口，可能与模型的限制一致，也可能不一致

Claude Code 以加法方式演进该类型，添加新数据作为可选字段而不是重塑现有字段。读取您知道的字段并忽略您不认识的任何字段。

### `SDKContextUsageCategory`

`/context` 使用情况按类别分解的一行。

```typescript theme={null}
type SDKContextUsageCategory = {
  name: string;
  tokens: number;
  kind: "used" | "free" | "buffer" | "deferred";
};
```

该表列出了 Claude Code 在行的每个字段中放入的内容。

| 字段 | 类型 | 描述 |
| - | - | - |
| `name` | `string` | 行的显示名称，如 `/context` 打印的那样，例如 `Messages`。按 `kind` 分类行，而不是按名称 |
| `tokens` | `number` | 行的 token 计数。行可以携带零 token |
| `kind` | `string` | 行代表什么：`used`、`free`、`buffer` 或 `deferred` |

每个 `kind` 值说明行的 token 是什么：

* `used`：占据上下文窗口的内容
* `free`：剩余窗口
* `buffer`：压缩保留
* `deferred`：Claude Code 保留在窗口外的工具 schema，从使用情况计算中排除，列出以供了解

### `SDKUsageReport`

`/usage` 报告的结构化形式，作为 `usage_report` 携带在交付 `/usage` 结果的 [`SDKAssistantMessage`](#sdkassistantmessage) 上。Agent SDK v0.3.273 及更高版本导出此类型。此类型为实验性的：其结构可能会发生变化。

```typescript theme={null}
type SDKUsageReport = {
  session: {
    total_cost_usd: number;
    total_api_duration_ms: number;
    total_duration_ms: number;
    total_lines_added: number;
    total_lines_removed: number;
    model_usage: { [modelName: string]: ModelUsage };
  };
  rate_limits: {
    limits:
      | {
          kind: string;
          group: string;
          percent: number;
          resets_at: string | null;
          scope?: {
            model?: { display_name: string } | null;
            surface?: { display_name: string } | null;
          } | null;
          severity: string;
          is_active: boolean;
        }[]
      | null;
    extra_usage?: {
      is_enabled: boolean;
      monthly_limit: number | null;
      used_credits: number | null;
      utilization: number | null;
      currency?: string | null;
    } | null;
  } | null;
};
```

顶层字段为 `session` 和 `rate_limits`：

* `session`：Claude Code 的累计成本和用量汇总，读取自与 [`SDKResultMessage`](#sdkresultmessage) 上的 `total_cost_usd` 和 `modelUsage` 相同的账目。每个 `model_usage` 条目都是一个 [`ModelUsage`](#modelusage)。
* `rate_limits`：`limits` 中为套餐的用量行，`extra_usage` 中为使用额度的支出。当 Claude Code 无法获取套餐用量时（例如会话的 OAuth 令牌缺少 `user:profile` 作用域），该字段为 `null`。

Claude Code 根据 token 数在本地计算 `session.total_cost_usd`，因此它是估算值，而不是您的套餐实际收取的费用。服务器报告的使用额度支出是单独的 `extra_usage` 块。关于准确性注意事项，请参阅[跟踪成本和用量](https://code.claude.com/docs/zh-CN/agent-sdk/cost-tracking.md)。

`limits` 按服务器发送的原样保存服务器的用量行：适用哪些计量器、其作用域、标签、严重程度和顺序都由服务器决定，因此请按原样渲染这些行。

* 空数组表示服务器未报告任何计量器。
* `null` 表示 Claude Code 没有可报告的行。

`limits` 的每一行描述一个用量计量器：

| 字段 | 类型 | 描述 |
| - | - | - |
| `kind` | `string` | 服务器的计量器类型，例如 `session`、`weekly_all` 或 `weekly_scoped`。请以此对行进行分类，切勿依据标签 |
| `group` | `string` | 服务器的行分组，例如 `session` 或 `weekly`。行按服务器的顺序分组渲染在其下 |
| `percent` | `number` | 窗口已使用的比例，0-100 |
| `resets_at` | `string \| null` | 窗口重置时的 ISO 8601 时间戳 |
| `scope` | `object \| null` | 可选。限定行所针对的对象（模型或使用入口），附带服务器的显示标签 |
| `severity` | `string` | 服务器对该行的判定，用于计量器的颜色，例如 `normal`、`warning` 或 `critical` |
| `is_active` | `boolean` | 在服务器选定用于单值指示器显示的行上为 `true` |

在 Agent SDK v0.3.277 之前，该类型将 `severity` 和 `is_active` 声明为可选且可为 null，行到达时可能不带这两个字段。

`extra_usage` 是服务器报告的计费周期内的[使用额度](https://support.claude.com/en/articles/12429409-extra-usage-for-paid-claude-plans)支出和上限，当套餐拥有使用额度时存在。金额以 `currency` 的最小单位表示，美元为美分。

* 当此账户没有自己的支出上限时，`monthly_limit` 为 `null`。在 Team 和 Enterprise 套餐上，不要将 `null` 渲染为无限制。
* 当使用额度无法用于支付请求时，`is_enabled` 为 `false`。

### `SDKMessageOrigin`

用户角色消息的来源。这在 [`SDKUserMessage`](#sdkusermessage) 上显示为 `origin`，并转发到相应的 [`SDKResultMessage`](#sdkresultmessage)，以便您可以告诉什么触发了给定的轮。

```typescript theme={null}
type SDKMessageOrigin =
  | { kind: "human" }
  | { kind: "channel"; server: string }
  | {
      kind: "peer";
      from: string;
      fromMode?: "bypass" | "prompting";
      name?: string;
      fromSession?: string;
      senderTaskId?: string;
      body?: string;
      verifiedPeerPid?: number;
    }
  | {
      kind: "task-notification";
      subkind?: "scheduled-trigger" | "peer-send-message";
      fireReason?: string;
      runId?: string;
    }
  | { kind: "coordinator" }
  | { kind: "auto-continuation" }
  | { kind: "unclassified" };
```

| `kind` | 含义 |
| - | - |
| `human` | 来自最终用户的直接输入。如果您的应用程序将用户输入的内容转发为用户消息，显式将其 `origin` 设置为 `{ kind: "human" }`：Claude Code 将没有 `origin` 的用户消息视为未归因，并且需要人类输入的提示词的检查（例如 [`ultracode` 工作流关键字](https://code.claude.com/docs/zh-CN/workflows#ask-for-a-workflow-in-your-prompt)）不接受它。在 v2.1.210 之前，Claude Code 将用户消息上缺失的 `origin` 视为人类输入。 |
| `channel` | 在[频道](https://code.claude.com/docs/zh-CN/channels.md)上到达的消息。`server` 是源 MCP 服务器名称。 |
| `peer` | 来自另一个 Agent 的消息：进程内[队友](https://code.claude.com/docs/zh-CN/agent-teams.md)或[跨会话对等体](https://code.claude.com/docs/zh-CN/cross-session-messaging.md)，您的另一个 Claude Code 会话。请参阅[对等体来源字段](#peer-origin-fields)了解每个字段的语义和信任模型。 |
| `task-notification` | 为没有新鲜用户提示词的传递注入的合成轮，例如完成的后台任务；请参阅 [`SDKTaskNotificationMessage`](#sdktasknotificationmessage) 了解该分支。您的应用程序[声明为定时运行](#declare-a-scheduled-run)的提示词也携带此类型。可选的 `subkind` 标记引发通知的原因。请参阅[任务通知子类型](#task-notification-subkinds)。 |
| `coordinator` | 来自 [agent team](https://code.claude.com/docs/zh-CN/agent-teams.md) 中的团队协调员的消息。 |
| `auto-continuation` | 当会话在没有新鲜用户输入的情况下继续时注入的合成轮，例如触发后续提示词的命令结果。 |
| `unclassified` | 无法确定来源的注入轮。需要 Claude Code v2.1.223 或更高版本。当 Claude Code 接收带有 `isSynthetic: true` 的 [`SDKUserMessage`](#sdkusermessage) 并无法将其分类为任何其他 `kind` 时，它在消息到达时设置此类型，并将轮作为非用户来源呈现给模型，而不是将其视为人类输入。您的应用程序不应设置此值。 |

### 任务通知子类型

当 Claude Code 将任务通知传递到会话中时，如果 Anthropic 服务器验证了该通知的来源，它会在通知的 `origin` 上设置 `subkind`。当您的应用程序[自己声明消息为定时运行](#declare-a-scheduled-run)时，它也设置 `subkind`，这需要 TypeScript Agent SDK v0.3.280 或更高版本。`subkind` 需要 Claude Code v2.1.213 或更高版本，它采用两个值之一：

* `scheduled-trigger`：该通知是某个[云端定时任务](https://code.claude.com/docs/zh-CN/routines.md)存储的提示词，因该任务的某个触发器被触发而投递：其计划、其 [API 触发器](https://code.claude.com/docs/zh-CN/routines.md#add-an-api-trigger)、其 [GitHub 触发器](https://code.claude.com/docs/zh-CN/routines.md#add-a-github-trigger)或 **Run now**。您的应用程序[声明为定时运行](#declare-a-scheduled-run)的提示词也带有此值。Claude Code 会将这些通知作为会话被分配的任务呈现给模型，所附的说明与[其他任务通知所附的说明](#sdktasknotificationmessage)不同。
* `peer-send-message`：该通知是您的另一个会话通过服务器端 `send_message` 工具发送的消息（[云端会话](https://code.claude.com/docs/zh-CN/claude-code-on-the-web.md)之间使用该工具互相发送消息），而不是[跨会话 `SendMessage` 工具](https://code.claude.com/docs/zh-CN/cross-session-messaging)，并且 Anthropic 服务器已验证两个会话属于同一个私有会话组。需要 Claude Code v2.1.224 或更高版本。服务器未以此方式验证的 `send_message` 投递不会获得子类型。

每个其他任务通知都没有 `subkind`。这包括[PR 活动](https://code.claude.com/docs/zh-CN/claude-code-on-the-web.md#how-claude-responds-to-pr-activity)传递到会话和后台事件，例如完成的任务。来自[跨会话 `SendMessage` 工具](https://code.claude.com/docs/zh-CN/cross-session-messaging)的消息根本不是任务通知：无论它们来自同一机器上的会话还是通过 Anthropic 服务器来自另一台机器，Claude Code 都给它们 `kind: "peer"` 和[对等体来源字段](#peer-origin-fields)。

`fireReason` 说明 `scheduled-trigger` 通知被触发的原因，以简短的小写标记表示，例如 `scheduled`、`manual`、`retry`、`catch_up` 或 `api`。Anthropic 服务器会在[云端定时任务](https://code.claude.com/docs/zh-CN/routines.md)的投递上设置该字段，您的应用程序在声明定时运行时也会设置它。当两者都未发送时，该字段不存在。需要 TypeScript Agent SDK v0.3.280 或更高版本。

一个后台任务可以在同一个 `task_id` 下运行多次，例如当 Claude [恢复子代理](https://code.claude.com/docs/zh-CN/agent-sdk/subagents.md#resume-subagents)时。每次运行都有自己的 [`run_id`](#sdktaskstartedmessage)。当某次运行的通知作为用户消息到达您的会话时，该消息 `task-notification` origin 上的 `runId` 持有相同的值。通知可能在任务的下一次运行开始后才到达，但它仍然带有较早那次运行的 ID。要判断某个轮次属于哪次运行，请比较 `runId`，而不是 `task_id`。需要 TypeScript Agent SDK v0.3.292 或更高版本。

#### 声明定时运行

如果您的应用程序按自己的计划运行提示词，声明每个运行，以便 Claude Code 将轮作为定时任务而不是来自用户的实时输入呈现给模型。使用 [`env`](#options) 中设置为 `1` 的 `CLAUDE_CODE_HOST_SCHEDULED_RUN` 启动会话，然后发送运行的 [`SDKUserMessage`](#sdkusermessage)，其中 `origin: { kind: "task-notification", subkind: "scheduled-trigger", fireReason: "scheduled" }` 且没有 `isSynthetic`。Claude Code 忽略在没有该变量启动的进程中的声明。它也在进程的环境携带 [`CLAUDECODE`](https://code.claude.com/docs/zh-CN/env-vars) 或 `CLAUDE_CODE_CHILD_SESSION` 时忽略它。Claude Code 仅在值为 1 到 32 个小写字母或下划线时保留 `fireReason`。需要 TypeScript Agent SDK v0.3.280 或更高版本。

### 对等体来源字段

`peer` 来源标识发送消息的 Agent：使用 `SendMessage` 向 `main` 发送消息的进程内[队友](https://code.claude.com/docs/zh-CN/agent-teams.md)，或[跨会话对等方](https://code.claude.com/docs/zh-CN/cross-session-messaging.md)（即您的另一个 Claude Code 会话）。在 macOS 和 Linux 上，跨会话对等方需要 Claude Code v2.1.224 或更高版本；有关原生 Windows 的要求，请参阅[跨会话消息的可用性](https://code.claude.com/docs/zh-CN/cross-session-messaging.md#availability)。跨会话对等方可以在同一台机器上运行，也可以在[您的另一台机器](https://code.claude.com/docs/zh-CN/cross-session-messaging.md#message-sessions-on-other-machines)上运行，此时其消息通过 [Remote Control](https://code.claude.com/docs/zh-CN/remote-control.md) 到达。

* `from`：队友的名称，或跨会话对等体的发送者地址。对于[单向跨机器消息](https://code.claude.com/docs/zh-CN/cross-session-messaging.md#message-sessions-on-other-machines)，发送者没有回复地址，`from` 是 `"unknown"`。该值由发送者创作；`verifiedPeerPid` 是验证的身份。
* `fromMode`：发送会话的权限类，`bypass` 或 `prompting`，由在您的会话之间中继对等消息的主机声明，例如[桌面应用](https://code.claude.com/docs/zh-CN/desktop.md#work-across-sessions)。Claude Code 在接收会话中应用[入站控制](https://code.claude.com/docs/zh-CN/cross-session-messaging.md#control-inbound-messages)时读取它。需要 Agent SDK v0.3.234 或更高版本。
* `senderTaskId`：队友的任务 ID。对于跨会话对等体不存在。
* `name`：发送者的显示名称，由 Claude Code 规范化：它删除 Unicode 控制、格式、代理项和行或段落分隔符代码点，然后修剪结果并将其限制为 64 个代码点，带有省略号。需要 Claude Code v2.1.205 或更高版本。
* `body`：去除对等方信封后的已解码消息正文，与模型所见内容逐字节一致。对于单条队友消息，该字段存在；当多条消息被合并时，该字段不存在；对于跨会话对等方，仅当该轮次恰好是由 Claude Code 构成的一个对等方信封时才存在。请渲染 `name` 和 `body`，而不要重新解析消息文本。在 v2.1.299 之前，合并的消息会保留第一条消息的 `body`。需要 Claude Code v2.1.205 或更高版本。
* `fromSession`：发送者的主机可打开会话 ID，由发送者的主机设置，以便您的 UI 可以链接回发送会话。像 `from` 一样，它是发送者声称的：仅将其用作导航目标，不要将其视为发送者身份的证明。需要 Claude Code v2.1.216 或更高版本。
* `verifiedPeerPid`：连接到此会话的跨会话消息传递套接字的进程的进程 ID，由内核验证并从连接本身读取，从不从负载读取。使用它，而不是 `from`，来标识发送者：`from` 可由任何同用户进程伪造。当 Claude Code 无法验证它时，该字段不存在，例如在 Windows 或非套接字入口上，因此缺失值意味着发送者未验证。对于中继流量，它标识中继而不是消息的作者，进程 ID 是可回收的，因此将其视为来源而不是身份验证令牌。需要 Claude Code v2.1.216 或更高版本。

## Hook 类型

有关使用 hook 的指南，包括示例和常见模式，请参阅 [Hooks 指南](https://code.claude.com/docs/zh-CN/agent-sdk/hooks.md)。

### `HookEvent`

可用的 hook 事件。

```typescript theme={null}
type HookEvent =
  | "PreToolUse"
  | "PostToolUse"
  | "PostToolUseFailure"
  | "PostToolBatch"
  | "Notification"
  | "UserPromptSubmit"
  | "UserPromptExpansion"
  | "SessionStart"
  | "SessionEnd"
  | "Stop"
  | "StopFailure"
  | "SubagentStart"
  | "SubagentStop"
  | "PreCompact"
  | "PostCompact"
  | "PreModelSwitch"
  | "PostModelSwitch"
  | "PermissionRequest"
  | "PermissionDenied"
  | "Setup"
  | "TeammateIdle"
  | "TaskCreated"
  | "TaskCompleted"
  | "Elicitation"
  | "ElicitationResult"
  | "ConfigChange"
  | "DirectoryAdded"
  | "WorktreeCreate"
  | "WorktreeRemove"
  | "InstructionsLoaded"
  | "CwdChanged"
  | "FileChanged"
  | "MessageDisplay";
```

### `HookCallback`

Hook 回调函数类型。

```typescript theme={null}
type HookCallback = (
  input: HookInput, // 所有 hook 输入类型的联合
  toolUseID: string | undefined,
  options: { signal: AbortSignal }
) => Promise<HookJSONOutput>;
```

### `HookCallbackMatcher`

带有可选匹配器的 Hook 配置。

```typescript theme={null}
interface HookCallbackMatcher {
  matcher?: string;
  hooks: HookCallback[];
  timeout?: number; // 此匹配器中所有 hooks 的超时时间（秒）
}
```

### `HookInput`

所有 hook 输入类型的联合类型。

```typescript theme={null}
type HookInput =
  | PreToolUseHookInput
  | PostToolUseHookInput
  | PostToolUseFailureHookInput
  | PostToolBatchHookInput
  | PermissionDeniedHookInput
  | NotificationHookInput
  | UserPromptSubmitHookInput
  | UserPromptExpansionHookInput
  | SessionStartHookInput
  | SessionEndHookInput
  | StopHookInput
  | StopFailureHookInput
  | SubagentStartHookInput
  | SubagentStopHookInput
  | PreCompactHookInput
  | PostCompactHookInput
  | PreModelSwitchHookInput
  | PostModelSwitchHookInput
  | PermissionRequestHookInput
  | SetupHookInput
  | TeammateIdleHookInput
  | TaskCreatedHookInput
  | TaskCompletedHookInput
  | ElicitationHookInput
  | ElicitationResultHookInput
  | ConfigChangeHookInput
  | InstructionsLoadedHookInput
  | DirectoryAddedHookInput
  | WorktreeCreateHookInput
  | WorktreeRemoveHookInput
  | CwdChangedHookInput
  | FileChangedHookInput
  | MessageDisplayHookInput;
```

### `BaseHookInput`

所有 hook 输入类型扩展的基本接口。

```typescript theme={null}
type BaseHookInput = {
  session_id: string;
  transcript_path: string;
  cwd: string;
  prompt_id?: string;
  permission_mode?: string;
  effort?: { level: string };
  agent_id?: string;
  agent_type?: string;
};
```

`prompt_id` 字段是一个 UUID，用于标识当前正在处理的用户提示。它与 [OpenTelemetry 事件上的 `prompt.id` 属性](https://code.claude.com/docs/zh-CN/monitoring-usage#event-correlation-attributes)匹配，在第一个用户输入之前不存在。需要 Claude Code v2.1.196 或更高版本。

#### `PreToolUseHookInput`

```typescript theme={null}
type PreToolUseHookInput = BaseHookInput & {
  hook_event_name: "PreToolUse";
  tool_name: string;
  tool_input: unknown;
  tool_use_id: string;
  mcp_server?: McpServerProvenance;
};
```

当工具来自 MCP 服务器时，`mcp_server` 存在；请参阅 [`McpServerProvenance`](#mcpserverprovenance)。`PostToolUse`、`PostToolUseFailure`、`PermissionRequest` 和 `PermissionDenied` 输入携带相同的字段。该字段需要 Agent SDK v0.3.274 或更高版本。

#### `PostToolUseHookInput`

```typescript theme={null}
type PostToolUseHookInput = BaseHookInput & {
  hook_event_name: "PostToolUse";
  tool_name: string;
  tool_input: unknown;
  tool_response: unknown;
  tool_use_id: string;
  duration_ms?: number;
  mcp_server?: McpServerProvenance;
};
```

#### `PostToolUseFailureHookInput`

```typescript theme={null}
type PostToolUseFailureHookInput = BaseHookInput & {
  hook_event_name: "PostToolUseFailure";
  tool_name: string;
  tool_input: unknown;
  tool_use_id: string;
  error: string;
  is_interrupt?: boolean;
  duration_ms?: number;
  mcp_server?: McpServerProvenance;
};
```

#### `PostToolBatchHookInput`

在批处理中的每个工具调用都已解决后触发一次，在下一个模型请求之前。`tool_response` 携带序列化的 `tool_result` 内容，模型会看到该内容；其形状与 `PostToolUseHookInput` 的结构化 `Output` 对象不同。

```typescript theme={null}
type PostToolBatchHookInput = BaseHookInput & {
  hook_event_name: "PostToolBatch";
  tool_calls: PostToolBatchToolCall[];
};

type PostToolBatchToolCall = {
  tool_name: string;
  tool_input: unknown;
  tool_use_id: string;
  tool_response?: unknown;
};
```

#### `PermissionDeniedHookInput`

```typescript theme={null}
type PermissionDeniedHookInput = BaseHookInput & {
  hook_event_name: "PermissionDenied";
  tool_name: string;
  tool_input: unknown;
  tool_use_id: string;
  reason: string;
  mcp_server?: McpServerProvenance;
};
```

#### `NotificationHookInput`

```typescript theme={null}
type NotificationHookInput = BaseHookInput & {
  hook_event_name: "Notification";
  message: string;
  title?: string;
  notification_type: string;
};
```

#### `UserPromptSubmitHookInput`

```typescript theme={null}
type UserPromptSubmitHookInput = BaseHookInput & {
  hook_event_name: "UserPromptSubmit";
  prompt: string;
  session_title?: string;
};
```

#### `UserPromptExpansionHookInput`

```typescript theme={null}
type UserPromptExpansionHookInput = BaseHookInput & {
  hook_event_name: "UserPromptExpansion";
  expansion_type: "slash_command" | "mcp_prompt";
  command_name: string;
  command_args: string;
  command_source?: string;
  prompt: string;
};
```

#### `SessionStartHookInput`

```typescript theme={null}
type SessionStartHookInput = BaseHookInput & {
  hook_event_name: "SessionStart";
  source: "startup" | "resume" | "clear" | "compact" | "fork";
  agent_type?: string;
  model?: string;
  session_title?: string;
};
```

#### `SessionEndHookInput`

```typescript theme={null}
type SessionEndHookInput = BaseHookInput & {
  hook_event_name: "SessionEnd";
  reason: ExitReason; // EXIT_REASONS 数组中的字符串
};
```

#### `StopHookInput`

```typescript theme={null}
type StopHookInput = BaseHookInput & {
  hook_event_name: "Stop";
  stop_hook_active: boolean;
  last_assistant_message?: string;
  background_tasks?: BackgroundTaskSummary[];
  session_crons?: SessionCronSummary[];
};
```

#### `StopFailureHookInput`

```typescript theme={null}
type StopFailureHookInput = BaseHookInput & {
  hook_event_name: "StopFailure";
  error: SDKAssistantMessageError;
  error_details?: string;
  last_assistant_message?: string;
};
```

#### `SubagentStartHookInput`

```typescript theme={null}
type SubagentStartHookInput = BaseHookInput & {
  hook_event_name: "SubagentStart";
  agent_id: string;
  agent_type: string;
};
```

#### `SubagentStopHookInput`

```typescript theme={null}
type SubagentStopHookInput = BaseHookInput & {
  hook_event_name: "SubagentStop";
  stop_hook_active: boolean;
  agent_id: string;
  agent_transcript_path: string;
  agent_type: string;
  last_assistant_message?: string;
  background_tasks?: BackgroundTaskSummary[];
  session_crons?: SessionCronSummary[];
};

type BackgroundTaskSummary = {
  id: string;
  type: string;
  status: string;
  description: string;
  command?: string;
  agent_type?: string;
  server?: string;
  tool?: string;
  name?: string;
};

type SessionCronSummary = {
  id: string;
  schedule: string;
  recurring: boolean;
  prompt: string;
};
```

#### `PreCompactHookInput`

```typescript theme={null}
type PreCompactHookInput = BaseHookInput & {
  hook_event_name: "PreCompact";
  trigger: "manual" | "auto";
  custom_instructions: string | null;
};
```

#### `PostCompactHookInput`

```typescript theme={null}
type PostCompactHookInput = BaseHookInput & {
  hook_event_name: "PostCompact";
  trigger: "manual" | "auto";
  compact_summary: string;
};
```

#### `PreModelSwitchHookInput`

在请求的模型切换生效之前触发。`context_tokens` 和之后的字段估计向新模型重新发送对话的成本。有关完整的字段描述和阻止语义，请参阅 [PreModelSwitch](https://code.claude.com/docs/zh-CN/hooks.md#premodelswitch)。

```typescript theme={null}
type PreModelSwitchHookInput = BaseHookInput & {
  hook_event_name: "PreModelSwitch";
  from_model: string;
  to_model: string;
  requested_model: string | null;
  source: "command" | "picker" | "sdk";
  context_tokens: number;
  prompt_cache_warm: boolean;
  cache_ttl: "5m" | "1h";
  estimated_cache_write_usd: number;
  pricing: "configured" | "catalog" | "default";
};
```

#### `PostModelSwitchHookInput`

在会话的模型更改后触发。它携带与 `PreModelSwitchHookInput` 相同的字段，另外还有两个 `source` 值。请参阅 [PostModelSwitch](https://code.claude.com/docs/zh-CN/hooks.md#postmodelswitch)。

```typescript theme={null}
type PostModelSwitchHookInput = BaseHookInput & {
  hook_event_name: "PostModelSwitch";
  from_model: string;
  to_model: string;
  requested_model: string | null;
  source: "command" | "picker" | "sdk" | "auto" | "resume";
  context_tokens: number;
  prompt_cache_warm: boolean;
  cache_ttl: "5m" | "1h";
  estimated_cache_write_usd: number;
  pricing: "configured" | "catalog" | "default";
};
```

#### `PermissionRequestHookInput`

```typescript theme={null}
type PermissionRequestHookInput = BaseHookInput & {
  hook_event_name: "PermissionRequest";
  tool_name: string;
  tool_input: unknown;
  permission_suggestions?: PermissionUpdate[];
  mcp_server?: McpServerProvenance;
};
```

#### `SetupHookInput`

```typescript theme={null}
type SetupHookInput = BaseHookInput & {
  hook_event_name: "Setup";
  trigger: "init" | "maintenance";
};
```

#### `TeammateIdleHookInput`

```typescript theme={null}
type TeammateIdleHookInput = BaseHookInput & {
  hook_event_name: "TeammateIdle";
  teammate_name: string;
  /** @deprecated 自 v2.1.178 起已弃用。携带会话派生的团队名称；将被移除。 */
  team_name: string;
};
```

#### `TaskCreatedHookInput`

```typescript theme={null}
type TaskCreatedHookInput = BaseHookInput & {
  hook_event_name: "TaskCreated";
  task_id: string;
  task_subject: string;
  task_description?: string;
  teammate_name?: string;
  /** @deprecated 自 v2.1.178 起已弃用。携带会话派生的团队名称；将被移除。 */
  team_name?: string;
};
```

#### `TaskCompletedHookInput`

```typescript theme={null}
type TaskCompletedHookInput = BaseHookInput & {
  hook_event_name: "TaskCompleted";
  task_id: string;
  task_subject: string;
  task_description?: string;
  teammate_name?: string;
  /** @deprecated 自 v2.1.178 起已弃用。携带会话派生的团队名称；将被移除。 */
  team_name?: string;
};
```

#### `ElicitationHookInput`

```typescript theme={null}
type ElicitationHookInput = BaseHookInput & {
  hook_event_name: "Elicitation";
  mcp_server_name: string;
  message: string;
  mode?: "form" | "url";
  url?: string;
  elicitation_id?: string;
  requested_schema?: Record<string, unknown>;
};
```

#### `ElicitationResultHookInput`

```typescript theme={null}
type ElicitationResultHookInput = BaseHookInput & {
  hook_event_name: "ElicitationResult";
  mcp_server_name: string;
  elicitation_id?: string;
  mode?: "form" | "url";
  action: "accept" | "decline" | "cancel";
  content?: Record<string, unknown>;
};
```

#### `ConfigChangeHookInput`

```typescript theme={null}
type ConfigChangeHookInput = BaseHookInput & {
  hook_event_name: "ConfigChange";
  source:
    | "user_settings"
    | "project_settings"
    | "local_settings"
    | "policy_settings"
    | "skills";
  file_path?: string;
};
```

#### `InstructionsLoadedHookInput`

```typescript theme={null}
type InstructionsLoadedHookInput = BaseHookInput & {
  hook_event_name: "InstructionsLoaded";
  file_path: string;
  memory_type: "User" | "Project" | "Local" | "Managed";
  load_reason:
    | "session_start"
    | "nested_traversal"
    | "path_glob_match"
    | "include"
    | "compact";
  globs?: string[];
  trigger_file_path?: string;
  parent_file_path?: string;
};
```

#### `DirectoryAddedHookInput`

```typescript theme={null}
type DirectoryAddedHookInput = BaseHookInput & {
  hook_event_name: "DirectoryAdded";
  directory: string;
  source: "slash_command" | "register_repo_root";
};
```

`directory` 是被添加的目录的绝对路径。当 `/add-dir` 添加它时，`source` 是 `"slash_command"`，当 SDK 控制请求添加它时，`source` 是 `"register_repo_root"`。

#### `WorktreeCreateHookInput`

```typescript theme={null}
type WorktreeCreateHookInput = BaseHookInput & {
  hook_event_name: "WorktreeCreate";
  name: string;
};
```

#### `WorktreeRemoveHookInput`

```typescript theme={null}
type WorktreeRemoveHookInput = BaseHookInput & {
  hook_event_name: "WorktreeRemove";
  worktree_path: string;
};
```

#### `CwdChangedHookInput`

```typescript theme={null}
type CwdChangedHookInput = BaseHookInput & {
  hook_event_name: "CwdChanged";
  old_cwd: string;
  new_cwd: string;
};
```

#### `FileChangedHookInput`

```typescript theme={null}
type FileChangedHookInput = BaseHookInput & {
  hook_event_name: "FileChanged";
  file_path: string;
  event: "change" | "add" | "unlink";
};
```

#### `MessageDisplayHookInput`

```typescript theme={null}
type MessageDisplayHookInput = BaseHookInput & {
  hook_event_name: "MessageDisplay";
  turn_id: string;
  message_id: string;
  index: number;
  final: boolean;
  delta: string;
};
```

### `HookJSONOutput`

Hook 返回值。

```typescript theme={null}
type HookJSONOutput = AsyncHookJSONOutput | SyncHookJSONOutput;
```

#### `AsyncHookJSONOutput`

```typescript theme={null}
type AsyncHookJSONOutput = {
  async: true;
  asyncTimeout?: number;
};
```

#### `SyncHookJSONOutput`

```typescript theme={null}
type SyncHookJSONOutput = {
  continue?: boolean;
  suppressOutput?: boolean;
  stopReason?: string;
  decision?: "approve" | "block";
  systemMessage?: string;
  /**
   * 一个终端转义序列（例如 OSC 9 / OSC 777 desktop-notification）
   * 供 Claude Code 代表您发出。仅允许通知/标题 OSCs
   * （0、1、2、9、99、777）和 BEL；包含任何其他内容的值
   * 将被整体忽略。仅交互式 CLI 会发出它；SDK 忽略该字段。
   */
  terminalSequence?: string;
  reason?: string;
  hookSpecificOutput?:
    | {
        hookEventName: "PreToolUse";
        permissionDecision?: "allow" | "deny" | "ask" | "defer";
        permissionDecisionReason?: string;
        updatedInput?: Record<string, unknown>;
        additionalContext?: string;
      }
    | {
        hookEventName: "UserPromptSubmit";
        additionalContext?: string;
        sessionTitle?: string;
        /** 当 decision 为 "block" 时，从阻止消息中省略原始提示。 */
        suppressOriginalPrompt?: boolean;
      }
    | {
        hookEventName: "UserPromptExpansion";
        additionalContext?: string;
      }
    | {
        hookEventName: "SessionStart";
        additionalContext?: string;
        initialUserMessage?: string;
        sessionTitle?: string;
        watchPaths?: string[];
        /**
         * SessionStart hooks 完成后重新扫描 skill 和命令目录，
         * 以便 hook 安装的 skills 在同一会话中可用。
         */
        reloadSkills?: boolean;
      }
    | {
        hookEventName: "Setup";
        additionalContext?: string;
      }
    | {
        hookEventName: "PreModelSwitch";
        /**
         * 与 PreToolUse 相同的约定："allow" 继续，"deny" 取消
         * 切换，"ask" 要求用户确认。仅交互式会话中的 /model
         * 显示该提示；其他所有表面，包括 set_model 请求，
         * 将 "ask" 视为拒绝。
         */
        permissionDecision?: "allow" | "deny" | "ask";
        permissionDecisionReason?: string;
      }
    | {
        hookEventName: "PostModelSwitch";
        /** 通过新模型服务的下一个请求到达模型。 */
        additionalContext?: string;
      }
    | {
        hookEventName: "SubagentStart";
        additionalContext?: string;
      }
    | {
        hookEventName: "PostToolUse";
        additionalContext?: string;
        /**
         * 关于此工具调用结果的简短说明，用于自动模式
         * 权限分类器。限制为 2000 个字符，在响应同一调用的
         * 所有 hooks 之间共享；仅在同步 hook 响应上被接受。
         * 不要将不受信任的工具输出复制到其中。
         */
        classifierContext?: string;
        updatedToolOutput?: unknown;
        /** @deprecated 使用 `updatedToolOutput`，它适用于所有工具。 */
        updatedMCPToolOutput?: unknown;
      }
    | {
        hookEventName: "PostToolUseFailure";
        additionalContext?: string;
      }
    | {
        hookEventName: "PostToolBatch";
        additionalContext?: string;
      }
    | {
        hookEventName: "Stop";
        additionalContext?: string;
      }
    | {
        hookEventName: "SubagentStop";
        additionalContext?: string;
      }
    | {
        hookEventName: "PermissionDenied";
        retry?: boolean;
      }
    | {
        hookEventName: "Notification";
        additionalContext?: string;
      }
    | {
        hookEventName: "PermissionRequest";
        decision:
          | {
              behavior: "allow";
              updatedInput?: Record<string, unknown>;
              updatedPermissions?: PermissionUpdate[];
            }
          | {
              behavior: "deny";
              message?: string;
              interrupt?: boolean;
            };
      }
    | {
        hookEventName: "Elicitation";
        action?: "accept" | "decline" | "cancel";
        content?: Record<string, unknown>;
      }
    | {
        hookEventName: "ElicitationResult";
        action?: "accept" | "decline" | "cancel";
        content?: Record<string, unknown>;
      }
    | {
        hookEventName: "CwdChanged";
        watchPaths?: string[];
      }
    | {
        hookEventName: "FileChanged";
        watchPaths?: string[];
      }
    | {
        hookEventName: "WorktreeCreate";
        worktreePath: string;
      }
    | {
        hookEventName: "MessageDisplay";
        /** 用来代替 delta 显示的文本。省略（或返回 delta 不变）以显示原始内容。 */
        displayContent?: string;
      };
};
```

## 工具输入类型

内置 Claude Code 工具的输入 schema 文档。这些类型从 `@anthropic-ai/claude-agent-sdk/sdk-tools` 导出，可用于类型安全的工具交互。

`StructuredOutput` 工具在此处没有类型：Claude Code 根据您传递给 `outputFormat` 的 schema 构建其输入 schema，因此没有固定的内容可以导出。请参阅[识别 StructuredOutput 工具调用](https://code.claude.com/docs/zh-CN/agent-sdk/structured-outputs.md#recognize-the-structuredoutput-tool-call)，了解该调用在您的消息流中的样子。

### `ToolInputSchemas`

从 `@anthropic-ai/claude-agent-sdk/sdk-tools` 导出的工具输入类型的联合；成员包括：

```typescript theme={null}
type ToolInputSchemas =
  | AgentInput
  | ArtifactInput
  | AskUserQuestionInput
  | BashInput
  | CronCreateInput
  | CronDeleteInput
  | CronListInput
  | EnterPlanModeInput
  | EnterWorktreeInput
  | ExitPlanModeInput
  | ExitWorktreeInput
  | FileEditInput
  | FileReadInput
  | FileWriteInput
  | GlobInput
  | GrepInput
  | ListMcpResourcesInput
  | McpInput
  | MonitorInput
  | NotebookEditInput
  | ProjectsInput
  | PushNotificationInput
  | ReadMcpResourceDirInput
  | ReadMcpResourceInput
  | RefreshMcpToolsInput
  | RemoteTriggerInput
  | ReportFindingsInput
  | ScheduleWakeupInput
  | ShowOnboardingRolePickerInput
  | TaskCreateInput
  | TaskGetInput
  | TaskListInput
  | TaskStopInput
  | TaskUpdateInput
  | TodoWriteInput
  | WebFetchInput
  | WebSearchInput
  | WorkflowInput;
```

### Agent

**工具名称：** `Agent`。当此工具对会话可用时，之前的名称 `Task` 仍被接受作为别名。

[`SDKSystemMessage`](#sdksystemmessage) 初始化消息中的 `tools` 数组为了向后兼容将此工具列为 `Task`。

> [!NOTE]
>
> 在 Claude Code v2.1.212 或更高版本上，`mode` 字段已弃用且被忽略。子代理在父会话的权限模式或其定义的 [`permissionMode`](#agentdefinition) 中运行，[子代理继承规则](https://code.claude.com/docs/zh-CN/agent-sdk/permissions.md#available-modes)决定使用哪一个。

```typescript theme={null}
type AgentInput = {
  description: string;
  prompt: string;
  subagent_type?: string;
  model?: "sonnet" | "opus" | "haiku" | "fable";
  effort?: "low" | "medium" | "high" | "xhigh" | "max";
  run_in_background?: boolean;
  name?: string;
  team_name?: string; // 已弃用；被忽略
  mode?: "acceptEdits" | "auto" | "bypassPermissions" | "default" | "dontAsk" | "plan"; // 已弃用；被忽略。子代理继承规则决定子代理的权限模式
  isolation?: "worktree" | "remote";
};
```

启动新的 Agent 以自主处理复杂的多步骤任务。

### AskUserQuestion

**工具名称：** `AskUserQuestion`

```typescript theme={null}
type AskUserQuestionInput = {
  questions: Array<{
    question: string;
    header: string;
    options: Array<{ label: string; description: string; preview?: string }>;
    multiSelect: boolean;
  }>;
  answers?: Record<string, string>;
  annotations?: Record<string, { preview?: string; notes?: string }>;
  metadata?: { source?: string };
};
```

在执行期间向用户提出澄清问题。请参阅[处理批准和用户输入](https://code.claude.com/docs/zh-CN/agent-sdk/user-input.md#handle-clarifying-questions)了解使用详情。

### Bash

**工具名称：** `Bash`

```typescript theme={null}
type BashInput = {
  command: string;
  timeout?: number; // 毫秒。前台：默认上限为 600000，更高的值会被限制。使用 run_in_background（Claude Code v2.1.285 或更高版本）：后台时间限制，省略时为 1800000，上限为 7200000，除非提高
  description?: string;
  run_in_background?: boolean;
  dangerouslyDisableSandbox?: boolean;
};
```

执行 Bash 命令，支持可选超时和后台执行。工作目录在命令之间保持不变，包括多轮会话后续轮次中运行的命令；shell 状态（如导出的环境变量）不保持。有关哪些目录更改会保持的限制，请参阅[命令之间保持什么](https://code.claude.com/docs/zh-CN/tools-reference.md#what-persists-between-commands)。有关设置前台上限的内容，请参阅[超时和输出限制](https://code.claude.com/docs/zh-CN/tools-reference.md#timeout-and-output-limits)。有关后台时间限制，请参阅[后台命令的时间限制](https://code.claude.com/docs/zh-CN/tools-reference.md#time-limit-for-background-commands)。

### Monitor

**工具名称：** `Monitor`

```typescript theme={null}
type MonitorInput = {
  description: string;
  timeout_ms: number;
  command?: string;
  ws?: {
    url: string;
    protocols?: string[];
  };
};
```

运行后台源并将每个事件传递给 Claude，以便它可以做出反应而无需轮询：`command` 运行脚本并为每个 stdout 行发出一个事件，`ws` 打开 WebSocket 并为每个文本帧发出一个事件。恰好提供 `command` 或 `ws` 之一。`ws` 源需要 Claude Code v2.1.195 或更高版本。

`timeout_ms` 是监视的截止时间（以毫秒为单位）。它默认为 300000，接受最高 3600000 的值。有效截止时间最多为 1800000，即 30 分钟，因此更大的接受值会被缩短到该值。在截止时间，监视结束，Claude 收到一个通知，以便在仍需要时可以启动新的监视。

导出的类型将 `timeout_ms` 标记为必需，因为 schema 填充了默认值；省略它的调用会验证通过。

当 Monitor 运行命令时，它遵循与 Bash 相同的权限规则；WebSocket 监视会单独提示批准。请参阅 [Monitor 工具参考](https://code.claude.com/docs/zh-CN/tools-reference.md#monitor-tool)了解行为和提供商可用性。

### TaskOutput

在 Claude Code v2.1.277 中移除，连同其 `TaskOutputInput` 类型一起。之前从运行中或已完成的后台任务检索输出；Claude 改为使用 `Read` 读取后台任务的输出文件。

仍然命名 `TaskOutput` 的 `disallowedTools` 条目或拒绝规则会被忽略，不会发出警告。

### Edit

**工具名称：** `Edit`

```typescript theme={null}
type FileEditInput = {
  file_path: string;
  old_string: string;
  new_string: string;
  replace_all?: boolean;
};
```

在文件中执行精确字符串替换。

### Read

**工具名称：** `Read`

```typescript theme={null}
type FileReadInput = {
  file_path: string;
  offset?: number;
  limit?: number;
  pages?: string;
  allow_large?: boolean;
};
```

从本地文件系统读取文件，包括文本、图像、PDF 和 Jupyter 笔记本。

* 对 PDF 页面范围使用 `pages`（例如，`"1-5"`）。
* `allow_large` 允许 Claude 在会话的上下文窗口有空间时读取超过[通常大小限制](https://code.claude.com/docs/zh-CN/tools-reference.md#large-files)的文本文件。它需要 Agent SDK v0.3.296 或更高版本，该版本捆绑了 Claude Code v2.1.296。

对于 PDF，Claude 在 Read 调用的 `tool_result` 内容中接收文件的内容。返回 `pdf` [输出](#tool-output-types)的读取操作包含一个摘要 `text` 块，后跟一个 `document` 块。返回 `parts` 输出的读取操作包含摘要 `text` 块，后跟每个提取页面的一个块：一个 `image` 块，或当 Claude Code 无法将其呈现为图像时命名该页面的 `text` 块。在 Agent SDK v0.3.242 之前，Claude Code 在工具结果后作为单独的 `user` 消息传递文件的内容。

### Write

**工具名称：** `Write`

```typescript theme={null}
type FileWriteInput = {
  file_path: string;
  content: string;
};
```

将文件写入本地文件系统，如果存在则覆盖。

### Glob

**工具名称：** `Glob`

```typescript theme={null}
type GlobInput = {
  pattern: string;
  path?: string;
};
```

快速文件模式匹配，适用于任何代码库大小。

### Grep

**工具名称：** `Grep`

```typescript theme={null}
type GrepInput = {
  pattern: string;
  path?: string;
  glob?: string;
  type?: string;
  output_mode?: "content" | "files_with_matches" | "count";
  "-i"?: boolean;
  "-o"?: boolean; // 仅打印每行的匹配部分；需要 output_mode: "content"
  "-n"?: boolean;
  "-B"?: number;
  "-A"?: number;
  "-C"?: number;
  context?: number;
  head_limit?: number;
  offset?: number;
  multiline?: boolean;
};
```

基于 ripgrep 的搜索工具，支持正则表达式。

### TaskStop

**工具名称：** `TaskStop`

```typescript theme={null}
type TaskStopInput = {
  task_id?: string;
  shell_id?: string; // 已弃用：使用 task_id
};
```

按 ID 停止运行的后台任务或 shell。自 v2.1.198 起，`task_id` 也接受 agent team 队友，或按 Agent ID 或名称指定的命名后台 Agent。

### NotebookEdit

**工具名称：** `NotebookEdit`

```typescript theme={null}
type NotebookEditInput = {
  notebook_path: string;
  cell_id?: string;
  new_source: string;
  cell_type?: "code" | "markdown";
  edit_mode?: "replace" | "insert" | "delete";
};
```

编辑 Jupyter 笔记本文件中的单元格。

### WebFetch

**工具名称：** `WebFetch`

```typescript theme={null}
type WebFetchInput = {
  url: string;
  prompt: string;
  offset?: number;
};
```

从 URL 获取内容并使用 AI 模型处理它。

`offset` 是从页面开头跳过的字符数。Claude 设置它以继续读取较长的页面。该字段需要 Agent SDK v0.3.290 或更高版本。

### WebSearch

**工具名称：** `WebSearch`

```typescript theme={null}
type WebSearchInput = {
  query: string;
  allowed_domains?: string[];
  blocked_domains?: string[];
};
```

搜索网络并返回格式化的结果。

### Workflow

**工具名称：** `Workflow`

```typescript theme={null}
type WorkflowInput = {
  script?: string;
  name?: string;
  scriptPath?: string;
  args?: unknown; // 任何 JSON 值；发布的类型将其呈现为对象映射
  resumeFromRunId?: string;
  title?: string; // 被忽略；脚本的 meta 块设置标题
  description?: string; // 被忽略；脚本的 meta 块设置描述
};
```

运行[动态工作流](https://code.claude.com/docs/zh-CN/workflows.md)：一个脚本，在后台协调许多子代理并返回一个统一的结果。`Workflow` 工具在 Agent SDK v0.3.149 及更高版本中可用。至少需要 `script`、`name` 或 `scriptPath` 之一。

| 字段 | 类型 | 描述 |
| - | - | - |
| `script` | `string` | 内联工作流脚本。必须以 `export const meta = { name, description }` 作为字面量开头，后跟使用 `agent()`、`parallel()`、`pipeline()` 和 `phase()` 的脚本主体。`meta` 中的可选 `phases` 数组在进度视图中将 Agent 分组到命名阶段下 |
| `name` | `string` | 内置工作流的名称或保存在 `.claude/workflows/` 中的工作流名称。解析为脚本 |
| `scriptPath` | `string` | 磁盘上工作流脚本文件的路径，例如先前运行返回的 `scriptPath`。优先于 `script` 和 `name`。当会话的工具不包含 `Read` 时，Claude Code 会以错误拒绝 `scriptPath` |
| `args` | `unknown` | 输入值，作为全局 `args` 暴露给脚本，用于参数化的命名工作流，例如研究问题或文件路径列表。将数组和对象作为实际 JSON 值传递，而不是作为 JSON 编码的字符串 |
| `resumeFromRunId` | `string` | 要恢复的先前 `Workflow` 调用的运行 ID。具有未更改输入的已完成 `agent()` 调用通常返回缓存的结果；其余的实时运行。[暂停后恢复](https://code.claude.com/docs/zh-CN/workflows.md#resume-after-a-pause)涵盖哪些已完成的调用会重新运行。仅限同一会话 |
| `title` | `string` | 被忽略；脚本的 `meta` 块设置标题 |
| `description` | `string` | 被忽略；脚本的 `meta` 块设置描述 |

### TodoWrite

**工具名称：** `TodoWrite`

```typescript theme={null}
type TodoWriteInput = {
  todos: Array<{
    content: string;
    status: "pending" | "in_progress" | "completed";
    activeForm: string;
  }>;
};
```

创建和管理结构化任务列表以跟踪进度。

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
> 请参阅[模型可用性](https://code.claude.com/docs/zh-CN/agent-sdk/todo-tracking.md#model-availability)以选择加入。

### TaskCreate

**工具名称：** `TaskCreate`

```typescript theme={null}
type TaskCreateInput = {
  subject: string;
  description: string;
  activeForm?: string;
  metadata?: Record<string, unknown>;
};
```

创建单个任务并返回其分配的 ID。

### TaskUpdate

**工具名称：** `TaskUpdate`

```typescript theme={null}
type TaskUpdateInput = {
  taskId: string;
  status?: "pending" | "in_progress" | "completed" | "deleted";
  subject?: string;
  description?: string;
  activeForm?: string;
  addBlocks?: string[];
  addBlockedBy?: string[];
  owner?: string;
  metadata?: Record<string, unknown>;
};
```

按 ID 修补一个任务。将 `status` 设置为 `"deleted"` 以删除它。

### TaskGet

**工具名称：** `TaskGet`

```typescript theme={null}
type TaskGetInput = {
  taskId: string;
};
```

返回一个任务的完整详情，或在找不到 ID 时返回 `null`。

### TaskList

**工具名称：** `TaskList`

```typescript theme={null}
type TaskListInput = {};
```

返回当前列表中所有任务的快照。

### ExitPlanMode

**工具名称：** `ExitPlanMode`

```typescript theme={null}
type ExitPlanModeInput = {
  /** 已弃用：不再使用。 */
  allowedPrompts?: Array<{
    tool: "Bash";
    prompt: string;
  }>;
  [k: string]: unknown;
};
```

退出计划模式。`allowedPrompts` 字段已弃用且被忽略；Claude Code 仍然接受它，以便现有调用者和会话记录能够通过验证。在 v2.1.205 之前，它请求基于提示词的 Bash 权限以实现计划。

### ListMcpResources

**工具名称：** `ListMcpResourcesTool`

```typescript theme={null}
type ListMcpResourcesInput = {
  server?: string;
};
```

列出来自连接服务器的可用 MCP 资源。

### ReadMcpResource

**工具名称：** `ReadMcpResourceTool`

```typescript theme={null}
type ReadMcpResourceInput = {
  server: string;
  uri: string;
};
```

从服务器读取特定的 MCP 资源。

### EnterWorktree

**工具名称：** `EnterWorktree`

```typescript theme={null}
type EnterWorktreeInput = {
  name?: string;
  path?: string;
};
```

创建并进入临时 git worktree 以进行隔离工作。传递 `path` 以切换到现有 worktree 而不是创建新的。在首次进入时，目标必须是当前仓库的已注册 worktree，或在多仓库工作区中，必须是嵌套在其中的仓库的已注册 worktree；从 worktree 会话内进入时，必须在会话仓库的 `.claude/worktrees/` 下。`name` 和 `path` 互斥。

### ExitWorktree

**工具名称：** `ExitWorktree`

```typescript theme={null}
type ExitWorktreeInput = {
  action: "keep" | "remove";
  discard_changes?: boolean;
};
```

退出当前 git worktree 并返回到原始工作目录。`keep` 操作将 worktree 和分支保留在磁盘上，而 `remove` 删除两者。当删除具有未提交文件或未合并提交的 worktree 时，`discard_changes` 必须为 `true`。

### EnterPlanMode

**工具名称：** `EnterPlanMode`

```typescript theme={null}
type EnterPlanModeInput = {};
```

进入计划模式，Claude 在其中研究并呈现计划，然后再进行更改。

### CronCreate

**工具名称：** `CronCreate`

```typescript theme={null}
type CronCreateInput = {
  cron: string;
  prompt: string;
  recurring?: boolean;
  durable?: boolean;
};
```

按本地时间的 5 字段 cron 计划安排提示词运行。将 `recurring` 设置为 `false` 以在下一个匹配时仅触发一次。作业默认限定于会话，使用 `--resume` 或 `--continue` 恢复时会还原尚未过期的作业。请参阅[限定于会话的定时任务](https://code.claude.com/docs/zh-CN/scheduled-tasks.md)。

将 `durable` 设置为 `true` 请求持久化到 `.claude/scheduled_tasks.json`，以便作业在重启后继续存在。持久化调度并非在每个会话中都可用：当不可用时，Claude Code 接受 `durable: true` 但创建仅会话的作业。读取输出的 `durable` 字段以查看作业是否已持久化。

### CronDelete

**工具名称：** `CronDelete`

```typescript theme={null}
type CronDeleteInput = {
  id: string;
};
```

按从 `CronCreate` 返回的 ID 删除计划的 cron 作业。

### CronList

**工具名称：** `CronList`

```typescript theme={null}
type CronListInput = {};
```

列出计划的 cron 作业：来自 `.claude/scheduled_tasks.json` 的持久化作业和来自当前会话的仅会话作业。

### ScheduleWakeup

**工具名称：** `ScheduleWakeup`

```typescript theme={null}
type ScheduleWakeupInput = {
  delaySeconds?: number;
  reason?: string;
  prompt?: string;
  noop?: boolean;
  stop?: boolean;
};
```

安排一次性唤醒，在延迟后触发给定的提示词。此工具支持自定步调的 `/loop` 命令。运行时将 `delaySeconds` 限制在 60 到 3600 秒之间。除非 `stop` 为 true，否则 `delaySeconds`、`reason`、`prompt` 和 `noop` 字段是必需的。`noop: true` 报告没有任何更改的唤醒。设置 `stop: true` 取消待处理的唤醒并结束自定步调的 `/loop`。`stop` 字段需要 Claude Code v2.1.202 或更高版本。请参阅[工具参考中的 ScheduleWakeup 行](https://code.claude.com/docs/zh-CN/tools-reference.md)。

### RemoteTrigger

**工具名称：** `RemoteTrigger`

```typescript theme={null}
type RemoteTriggerInput = {
  action:
    | "list"
    | "get"
    | "create"
    | "update"
    | "run"
    | "create_webhook_trigger"
    | "list_runs"
    | "get_run_log";
  trigger_id?: string;
  session_id?: string;
  cursor?: string;
  body?: {
    [k: string]: unknown;
  };
};
```

管理[云端定时任务](https://code.claude.com/docs/zh-CN/routines.md)（前身为 Routine）：即在云端托管的按计划和按触发运行的 Claude Code 任务。此工具支持 `/schedule` 命令。`trigger_id` 对于 `get`、`update`、`run` 和 `list_runs` 操作是必需的。`body` 对于 `create`、`update` 和 `create_webhook_trigger` 是必需的，对于 `run` 是可选的。

`create_webhook_trigger`、`list_runs` 和 `get_run_log` 各自需要最低的 Claude Code 版本：

* **`create_webhook_trigger`**：将事件源附加到现有定时任务，例如触发它的 [GitHub 事件](https://code.claude.com/docs/zh-CN/routines.md#add-a-github-trigger)。`body` 命名源、事件和要触发的任务。需要 Claude Code v2.1.225 或更高版本。
* **`list_runs` 和 `get_run_log`**：`list_runs` 列出定时任务的最近运行，`get_run_log` 读取一次运行的日志。`session_id` 从 `list_runs` 结果中指定要读取的运行，`cursor` 分页浏览任一操作的结果。两个操作都需要 Claude Code v2.1.227 或更高版本。

此工具仅在会话使用启用了云端定时任务的计划的 claude.ai 账户进行身份验证时可用。当您的组织的策略禁用[云端会话](https://code.claude.com/docs/zh-CN/claude-code-on-the-web.md)时，该工具不存在。在 Claude Code v2.1.227 或更高版本上，当所有者[为组织关闭定时任务](https://code.claude.com/docs/zh-CN/routines.md#routines-are-disabled-by-your-organizations-policy)时，该工具也不存在。

### PushNotification

**工具名称：** `PushNotification`

```typescript theme={null}
type PushNotificationInput = {
  message: string;
  status: "proactive";
};
```

向用户发送主动推送通知。将 `message` 保持在 200 个字符以下，因为移动操作系统会截断较长的文本。请参阅[工具参考中的 PushNotification 行](https://code.claude.com/docs/zh-CN/tools-reference.md)了解提供商可用性；推送传递通过 Anthropic 托管的基础设施进行，该基础设施无法从 Amazon Bedrock、AWS 上的 Claude Platform、Google Cloud 的 Agent Platform 或 Microsoft Foundry 访问。

### REPL

在 v2.1.275 中移除。在 v2.1.274 及之前，可以通过在 [`env` 选项](#options)中设置 `CLAUDE_CODE_REPL=1` 来打开实验性 `REPL` 工具。

### ReportFindings

**工具名称：** `ReportFindings`

```typescript theme={null}
type ReportFindingsInput = {
  level?: "low" | "medium" | "high" | "xhigh" | "max";
  findings: Array<{
    file: string;
    line?: number;
    summary: string;
    failure_scenario: string;
    short_summary?: string;
    category?: string;
    verdict?: "CONFIRMED" | "PLAUSIBLE";
    outcome?: "fixed" | "skipped" | "no_change_needed";
  }>;
};
```

将代码审查发现报告为结构化列表，以便 Claude Code 可以呈现它们而不是将其打印为文本。发现按最严重优先排序，每次调用最多 32 个，当没有发现保留下来时数组为空。需要 Claude Code v2.1.196 或更高版本。

`level` 是可选的，包含 Claude 为该审查报告的 effort 级别。Claude Code 不会将其与审查实际运行时的级别进行比较，因此两者可能不同。

每个发现包含这些字段：

* `file`：发现所在的仓库相对路径。可选的 `line` 是它锚定到的 1 索引行。
* `summary`：缺陷的单句陈述。`failure_scenario` 描述导致错误输出或崩溃的具体输入和状态。
* `short_summary`：可选的最多 60 个字符的压缩标签，用于紧凑显示。需要 Claude Code v2.1.212 或更高版本。
* `category`：可选的发现类型的短 kebab-case slug，例如 `correctness` 或 `test-coverage`。需要 Claude Code v2.1.199 或更高版本。
* `verdict`：在验证通过运行时设置；在仅内联审查中不存在。
* `outcome`：仅在应用修复后重新报告时设置。

### Artifact

**工具名称：** `Artifact`

```typescript theme={null}
type ArtifactInput = {
  action?: "publish" | "list";
  file_path?: string;
  favicon?: string;
  icon?: string;
  limit?: number;
  scope?: "mine" | "shared" | "all";
  title?: string;
  description?: string;
  label?: string;
  url?: string;
  force?: boolean;
  capabilities?: Record<string, unknown>;
  contract?: "latest" | string;
};
```

将本地 `.html` 或 `.md` 文件发布为托管的 Artifact 页面，或列出用户发布的 Artifact。省略 `action` 或传递 `"publish"` 以发布 `file_path`，这对于发布操作是必需的。下面的每个字段适用于发布：

* `icon`：Artifact 浏览器标签图标的一个短通用词，例如 `chart` 或 `map`。Claude 在首次发布时包含它，在更新时省略它，这会保留 Artifact 的存储图标。
* `favicon`：已弃用，Claude 会省略它。
* `title`：当 HTML 文件没有 `<title>` 标签时，在浏览器标签和库中命名发布的页面。
* `url`：针对现有 Artifact 以就地更新，而不是创建新的。

`force` 是最后手段的覆盖，丢弃另一个会话发布的较新版本。在冲突时，失败的发布返回较新的内容；Claude 将其更改合并到该内容上，或重新读取 Artifact，然后再次发布。仅当用户明确要求丢弃该版本时才传递 `force`。

传递 `"list"` 以枚举用户发布的 Artifact；仅 `limit` 和 `scope` 可以伴随它。`scope` 默认为 `"mine"`，列出用户拥有的 Artifact；`"shared"` 列出其他人与用户共享的 Artifact，`"all"` 列出两者。

`limit` 设置列表返回的 Artifact 最大数量，范围为 1 到 200。大于 50 的 `limit` 需要 Agent SDK v0.3.292 或更高版本。不指定 `limit` 时，列表最多返回 25 个。

* `capabilities`：发布的页面使用的运行时功能，以功能名称为键，例如[页面可能调用的连接器](https://code.claude.com/docs/zh-CN/artifacts.md#pull-live-data-with-mcp-connectors)。Artifact 服务验证声明，并拒绝命名了账户无法使用的功能或为某功能提供无效配置的发布。传递 `{}` 以清除存储的声明，在重新部署时省略该字段以保留它。需要 Agent SDK v0.3.235 或更高版本。
* `contract`：发布的页面运行所基于的运行时版本。省略它以保留 Artifact 的当前版本，传递 `"latest"` 以升级，或传递特定版本以固定或回滚。需要 Agent SDK v0.3.235 或更高版本。

这些类型已导出，但该工具在 Agent SDK 会话中默认处于关闭状态。发布还需要满足 [Artifact 可用性表](https://code.claude.com/docs/zh-CN/artifacts.md#availability)中的每个条件，使用 API 密钥进行身份验证的会话不满足这些条件。

### Projects

**工具名称：** `Projects`

```typescript theme={null}
type ProjectsInput = {
  method:
    | "project_info"
    | "project_read"
    | "project_search"
    | "project_write"
    | "project_delete";
  path?: string;
  content?: string;
  local_path?: string;
  present_to_user?: boolean;
  query?: string;
  n?: number;
};
```

读取和写入附加到会话的 claude.ai Project。根据 `method` 分派：

* `project_info`：返回项目元数据和文档列表。
* `project_read`：按 `path` 读取一个文档。
* `project_search`：使用 `query` 查询项目的知识库。`n` 限制命中数并默认为 `5`。
* `project_write`：从 `content`（包含内联文本）或 `local_path`（命名工作目录内的文件）中的恰好一个在 `path` 处创建或替换文档。`present_to_user: true` 将写入的文档标记为用户需要看到的可交付成果。
* `project_delete`：按 `path` 删除文档。

### ReadMcpResourceDir

**工具名称：** `ReadMcpResourceDirTool`

```typescript theme={null}
type ReadMcpResourceDirInput = {
  server: string;
  uri: string;
};
```

列出 MCP 服务器上目录资源的直接子项。仅可用于已声明支持目录列表的服务器；列表不是递归的。目录列表并非在每个会话中都启用：当关闭时，调用返回空的 `resources` 列表，`error` 字段报告目录列表未启用。

### RefreshMcpTools

**工具名称：** `RefreshMcpTools`

```typescript theme={null}
type RefreshMcpToolsInput = {
  server?: string; // 仅刷新此服务器；省略以刷新所有连接的服务器
};
```

重新查询连接的 MCP 服务器的工具列表并应用任何更改。这些类型已导出，但 Claude Code 仅在您在 [`env` 选项](#options)中设置 `CLAUDE_CODE_ENABLE_REFRESH_MCP_TOOLS=1` 时注册该工具，并且仅在至少有一个 MCP 服务器的会话中。需要 Claude Code v2.1.211 或更高版本。

### ShowOnboardingRolePicker

**工具名称：** `ShowOnboardingRolePicker`

```typescript theme={null}
type ShowOnboardingRolePickerInput = {};
```

在 Cowork 入职期间呈现可点击的角色选择器芯片行，以便用户可以选择其角色并安装匹配的插件。不需要参数；角色列表由客户端定义。调用会阻塞直到用户响应。

### McpInput

**工具名称：** 形式为 `mcp__<server>__<tool>` 的动态 MCP 工具名称

```typescript theme={null}
type McpInput = {
  [k: string]: unknown;
};
```

MCP 工具参数是开放对象：每个服务器定义自己的参数，因此类型对字段名称或值不施加任何约束。请查阅服务器自己的工具 schema 以了解特定工具接受的字段。

## 工具输出类型

内置 Claude Code 工具的输出 schema 文档。这些类型从 `@anthropic-ai/claude-agent-sdk/sdk-tools` 导出，代表每个工具返回的实际响应数据。

### `ToolOutputSchemas`

从 `@anthropic-ai/claude-agent-sdk/sdk-tools` 导出的工具输出类型的联合；成员包括：

```typescript theme={null}
type ToolOutputSchemas =
  | AgentOutput
  | ArtifactOutput
  | AskUserQuestionOutput
  | BashOutput
  | CronCreateOutput
  | CronDeleteOutput
  | CronListOutput
  | EnterPlanModeOutput
  | EnterWorktreeOutput
  | ExitPlanModeOutput
  | ExitWorktreeOutput
  | FileEditOutput
  | FileReadOutput
  | FileWriteOutput
  | GlobOutput
  | GrepOutput
  | ListMcpResourcesOutput
  | McpOutput
  | MonitorOutput
  | NotebookEditOutput
  | ProjectsOutput
  | PushNotificationOutput
  | ReadMcpResourceDirOutput
  | ReadMcpResourceOutput
  | RefreshMcpToolsOutput
  | RemoteTriggerOutput
  | ReportFindingsOutput
  | ScheduleWakeupOutput
  | ShowOnboardingRolePickerOutput
  | TaskCreateOutput
  | TaskGetOutput
  | TaskListOutput
  | TaskStopOutput
  | TaskUpdateOutput
  | TodoWriteOutput
  | WebFetchOutput
  | WebSearchOutput
  | WorkflowOutput;
```

### Agent

**工具名称：** `Agent`。当此工具对会话可用时，之前的名称 `Task` 仍然被接受作为别名。

```typescript theme={null}
type AgentOutput =
  | {
      status: "completed";
      agentId: string;
      agentType?: string;
      content: Array<{ type: "text"; text: string; citations?: unknown[] | null }>;
      resolvedModel?: string;
      modelsUsed?: string[];
      totalToolUseCount: number;
      totalDurationMs: number;
      totalTokens: number;
      usage: {
        input_tokens: number;
        output_tokens: number;
        cache_creation_input_tokens: number | null;
        cache_read_input_tokens: number | null;
        server_tool_use: {
          web_search_requests: number;
          web_fetch_requests: number;
        } | null;
        service_tier: string | null;
        cache_creation: {
          ephemeral_1h_input_tokens: number;
          ephemeral_5m_input_tokens: number;
        } | null;
        inference_geo?: string | null;
        speed?: string | null;
        iterations?: unknown;
        output_tokens_details?: {
          thinking_tokens?: number | null;
        } | null;
        fallback_credit?: unknown;
      };
      toolStats?: {
        readCount: number;
        searchCount: number;
        bashCount: number;
        editFileCount: number;
        linesAdded: number;
        linesRemoved: number;
        otherToolCount: number;
        frameCount?: number;
      };
      prompt: string;
      worktreePath?: string;
      worktreeBranch?: string;
    }
  | {
      status: "async_launched";
      isAsync?: true;
      agentId: string;
      description: string;
      resolvedModel?: string;
      modelsUsed?: string[];
      prompt: string;
      outputFile: string;
      canReadOutputFile?: boolean;
    }
  | {
      status: "remote_launched";
      taskId: string;
      sessionUrl: string;
      description: string;
      prompt: string;
      outputFile: string;
    };
```

返回来自子代理的结果。在 `status` 字段上进行区分：`"completed"` 表示已完成的任务，`"async_launched"` 表示后台任务，`"remote_launched"` 表示 Claude Code 分派到云端会话的任务，其中 `sessionUrl` 链接到该会话，`taskId` 标识它。

在 `completed` 变体上，`resolvedModel` 命名子代理启动时所用的模型，当应用 [`availableModels`](https://code.claude.com/docs/zh-CN/model-config#restrict-model-selection) 或其他覆盖时，该模型可能与请求的 `model` 输入不同。此字段需要 Claude Code v2.1.174 或更高版本。在 `async_launched` 上，它命名任务移至后台时使用的模型。

`modelsUsed` 列出子代理使用的模型，按顺序。该字段仅在发生中途交换时出现，当运行交换回某个模型时，该模型会再次出现。在 `async_launched` 上，该列表涵盖后台处理前使用的模型。`modelsUsed` 和 `resolvedModel` 的后台处理行为都需要 Claude Code v2.1.212 或更高版本。

如果 Claude Code [保留了子代理的隔离 worktree](https://code.claude.com/docs/zh-CN/worktrees.md#isolate-subagents-with-worktrees)，`completed` 结果上的 `worktreePath` 是找到它的位置。`worktreeBranch` 是其分支，当 Claude Code 使用 git 创建 worktree 时出现。

Claude Code 从子代理的最终 API 请求而不是整个运行中填充 `usage` 和 `totalTokens`，因此 `usage.service_tier` 是 API 在该请求上报告的服务层字符串。当存在时，`usage.output_tokens_details.thinking_tokens` 是该请求的输出 token 中属于思考 token 的数量。`output_tokens_details` 字段需要 TypeScript SDK v0.3.228 或更高版本，该版本包含 Claude Code v2.1.228。`fallback_credit` 字段需要 TypeScript SDK v0.3.285 或更高版本，该版本包含 Claude Code v2.1.285。

`usage.output_tokens_details` 在含义上与 [`Usage.output_tokens_details`](#usage) 匹配，范围限于该最终请求，但其每个级别都是可选的。保护对象和字段，例如 `usage.output_tokens_details?.thinking_tokens ?? 0`，而不是直接读取它。

在 v2.1.207 之前，发布的类型更窄。它省略了 `worktreePath`、`worktreeBranch`、`citations`、`toolStats.frameCount` 和 `inference_geo`、`speed` 和 `iterations` 使用字段，并将 `service_tier` 类型化为 `"standard" | "priority" | "batch"`。类型标记为可选的字段可能在早期版本记录的结果中不存在。

### AskUserQuestion

**工具名称：** `AskUserQuestion`

```typescript theme={null}
type AskUserQuestionOutput = {
  questions: Array<{
    question: string;
    header: string;
    options: Array<{ label: string; description: string; preview?: string }>;
    multiSelect: boolean;
  }>;
  answers: Record<string, string>;
  response?: string;
  annotations?: Record<string, { preview?: string; notes?: string }>;
  afkTimeoutMs?: number;
};
```

返回提出的问题和用户的答案。当用户输入自由形式的回复而不是回答结构化问题时，`response` 被设置；当存在时，Claude 会收到"用户回复：…"而不是每个问题的答案列表。

### Bash

**工具名称：** `Bash`

```typescript theme={null}
type BashOutput = {
  stdout: string;
  stderr: string;
  rawOutputPath?: string;
  interrupted: boolean;
  isImage?: boolean;
  backgroundTaskId?: string;
  backgroundedByUser?: boolean;
  timedOutAfterMs?: number;
  backgroundCwdHint?: string;
  backgroundEndsWithFinalResponse?: true;
  dangerouslyDisableSandbox?: boolean;
  returnCodeInterpretation?: string;
  noOutputExpected?: boolean;
  structuredContent?: unknown[];
  persistedOutputPath?: string;
  persistedOutputSize?: number;
  staleReadFileStateHint?: string;
  ghRateLimitHint?: string;
  gitOperation?: {
    commit?: { sha: string; kind: "committed" | "amended" | "cherry-picked"; branch?: string };
    push?: { branch: string };
    branch?: { ref: string; action: "merged" | "rebased" };
    pr?: {
      number: number;
      url?: string;
      action: "created" | "edited" | "merged" | "commented" | "closed" | "reopened" | "ready" | "draft" | "auto-merge-enabled" | "auto-merge-disabled";
    };
  };
};
```

`stdout`、`stderr` 和 `backgroundTaskId` 字段携带：

| 字段 | 它携带的内容 |
| - | - |
| `stdout` | 命令的 stdout 和 stderr，合并为一个交错流 |
| `stderr` | 工具本身添加的通知，例如 shell 工作目录重置，不是命令的 stderr |
| `backgroundTaskId` | 对于后台命令存在 |

`timedOutAfterMs` 是超时时间（以毫秒为单位），当命令达到其超时并移至后台而不是显式启动时设置。`backgroundCwdHint` 在后台命令包含目录更改内置命令（如 `cd`、`pushd`、`popd` 或 `chdir`）时设置，并注意会话工作目录未更改。两个字段都需要 Claude Code v2.1.210 或更高版本。

当在前台运行的子代理拥有后台命令时，该命令[在该子代理的运行结束时结束](https://code.claude.com/docs/zh-CN/tools-reference.md#when-a-background-command-stops)。Claude Code 在此类命令上将 `backgroundEndsWithFinalResponse` 设置为 `true`，并在命令存活该轮时省略该字段，如主对话或后台子代理启动的命令那样。该字段需要 Claude Code v2.1.227 或更高版本。

Claude Code 将 `gitOperation.commit.branch` 设置为 git 提交摘要行中命名的分支，对于在分离 HEAD 上进行的提交则省略它。该字段需要 Agent SDK v0.3.227 或更高版本。Claude Code 将 `gh pr reopen` 命令报告为 `reopened` PR 操作，这需要 Agent SDK v0.3.234 或更高版本。

### Monitor

**工具名称：** `Monitor`

```typescript theme={null}
type MonitorOutput = {
  taskId: string;
  timeoutMs: number;
  persistent?: boolean;
};
```

返回运行监视器的后台任务 ID。使用此 ID 与 `TaskStop` 一起提前取消监视。

### Edit

**工具名称：** `Edit`

```typescript theme={null}
type FileEditOutput = {
  filePath: string;
  oldString: string;
  newString: string;
  originalFile: string | null;
  structuredPatch: Array<{
    oldStart: number;
    oldLines: number;
    newStart: number;
    newLines: number;
    lines: string[];
  }>;
  userModified: boolean;
  replaceAll: boolean;
  gitDiff?: {
    filename: string;
    status: "modified" | "added";
    additions: number;
    deletions: number;
    changes: number;
    patch: string;
    repository?: string | null;
  };
};
```

返回编辑操作的结构化 diff。

### Read

**工具名称：** `Read`

```typescript theme={null}
type FileReadOutput =
  | {
      type: "text";
      file: {
        filePath: string;
        content: string;
        numLines: number;
        startLine: number;
        totalLines: number;
        /** True when a whole-file read was auto-paginated because it exceeded the token cap (the content is a partial first page). */
        truncatedByTokenCap?: boolean;
      };
    }
  | {
      type: "image";
      file: {
        base64: string;
        type: "image/jpeg" | "image/png" | "image/gif" | "image/webp";
        originalSize: number;
        dimensions?: {
          originalWidth?: number;
          originalHeight?: number;
          displayWidth?: number;
          displayHeight?: number;
        };
      };
    }
  | {
      type: "notebook";
      file: {
        filePath: string;
        cells: unknown[];
      };
    }
  | {
      type: "pdf";
      file: {
        filePath: string;
        base64: string;
        originalSize: number;
      };
    }
  | {
      type: "parts";
      file: {
        filePath: string;
        originalSize: number;
        count: number;
        outputDir: string;
      };
      /** Document page number of the first extracted page; labels the page images in the tool_result content. */
      firstPage?: number;
      /** In-process only: the page-image bytes are delivered as image blocks in the tool_result content and aren't retained on the emitted tool_use_result, so this key is absent there. */
      pages?: {
        base64: string;
        mediaType: "image/jpeg" | "image/png" | "image/gif" | "image/webp";
        error?: string;
      }[];
    }
  | {
      type: "file_unchanged";
      file: {
        filePath: string;
      };
      /** Set when the dedup matched a startup-seeded entry (CLAUDE.md / nested memory) rather than a prior Read tool_result. */
      source?: "seeded";
    };
```

返回适合文件类型的格式的文件内容。在 `type` 字段上进行区分。

### Write

**工具名称：** `Write`

```typescript theme={null}
type FileWriteOutput = {
  type: "create" | "update";
  filePath: string;
  content: string;
  structuredPatch: Array<{
    oldStart: number;
    oldLines: number;
    newStart: number;
    newLines: number;
    lines: string[];
  }>;
  originalFile: string | null;
  gitDiff?: {
    filename: string;
    status: "modified" | "added";
    additions: number;
    deletions: number;
    changes: number;
    patch: string;
    repository?: string | null;
  };
  userModified?: boolean;
};
```

返回写入结果，包含结构化 diff 信息。`originalFile` 和 `structuredPatch` 持有的内容取决于写入：

* 对于新创建的文件，`originalFile` 为 null，`structuredPatch` 为空
* 在覆盖时，`originalFile` 携带之前的内容，除非该内容大于约 10 MB：Claude Code 则跳过 diff 并返回 `originalFile` null 和 `structuredPatch` 空
* 当写入未更改任何内容或 diff 超时时，`structuredPatch` 也为空

### Glob

**工具名称：** `Glob`

```typescript theme={null}
type GlobOutput = {
  durationMs: number;
  numFiles: number;
  filenames: string[];
  truncated: boolean;
  totalMatches?: number;
  countIsComplete?: boolean;
};
```

返回与 glob 模式匹配的文件路径，按修改时间排序。

`totalMatches` 和 `countIsComplete` 需要 Claude Code v2.1.191 或更高版本。`totalMatches` 报告截断前的匹配文件数。当 `countIsComplete` 为 false 时，`totalMatches` 是一个下界，因为底层搜索截断了其自己的输出。

### Grep

**工具名称：** `Grep`

```typescript theme={null}
type GrepOutput = {
  mode?: "content" | "files_with_matches" | "count";
  numFiles: number;
  filenames: string[];
  content?: string;
  numLines?: number;
  numMatches?: number;
  totalFiles?: number;
  totalLines?: number;
  appliedLimit?: number;
  appliedOffset?: number;
};
```

返回搜索结果。形状因 `mode` 而异：文件列表、带匹配的内容或匹配计数。在 `count` 模式下，`numFiles` 和 `numMatches` 是完整结果集上的总计，不是分页切片。在 v2.1.208 之前，截断列出条目的 `head_limit` 或 `offset` 也会截断这些总计。

`totalFiles` 需要 Claude Code v2.1.208 或更高版本，并在 `files_with_matches` 模式下报告 `head_limit` 和 `offset` 分页前的总结果数。`totalLines` 需要 Claude Code v2.1.210 或更高版本，并在 `content` 模式下报告分页前的总行数。

### TaskStop

**工具名称：** `TaskStop`

```typescript theme={null}
type TaskStopOutput = {
  message: string;
  task_id: string;
  task_type: string;
  command?: string;
};
```

停止后台任务后返回确认。

### NotebookEdit

**工具名称：** `NotebookEdit`

```typescript theme={null}
type NotebookEditOutput = {
  new_source: string;
  old_source?: string;
  cell_id?: string;
  cell_type: "code" | "markdown";
  language: string;
  edit_mode: string;
  error?: string;
  notebook_path: string;
  original_file: string;
  updated_file: string;
};
```

返回笔记本编辑的结果，包含原始和更新的文件内容。

### WebFetch

**工具名称：** `WebFetch`

```typescript theme={null}
type WebFetchOutput = {
  bytes: number;
  code: number;
  codeText: string;
  result: string;
  durationMs: number;
  url: string;
  artifactRead?: {
    slug: string;
    ver?: string;
    seeded?: false;
  };
};
```

返回获取的内容，包含 HTTP 状态和元数据。

`artifactRead` 是 Claude Code 自己的 Artifact 读取记录，仅当 Claude 获取会话可以发布的 Artifact 时出现。Claude Code 在会话恢复时读取它回来，以便稍后的发布基于正确的版本；您的代码不需要对其采取行动。`slug` 命名 Artifact，`ver` 是读取记录的版本，当它未记录任何内容时不存在，`seeded: false` 标记其完整源未到达 Claude 的读取。`seeded` 字段需要 Agent SDK v0.3.239 或更高版本。

### WebSearch

**工具名称：** `WebSearch`

```typescript theme={null}
type WebSearchOutput = {
  query: string;
  results: Array<
    | {
        tool_use_id: string;
        content: Array<{ title: string; url: string }>;
      }
    | string
  >;
  durationSeconds: number;
  searchCount?: number;
};
```

返回来自网络的搜索结果。

### Workflow

**工具名称：** `Workflow`

```typescript theme={null}
type WorkflowOutput = {
  status: "async_launched" | "remote_launched";
  taskId: string;
  taskType?: "local_workflow" | "remote_agent";
  workflowName?: string;
  runId?: string;
  summary?: string;
  transcriptDir?: string;
  scriptPath?: string;
  sessionUrl?: string; // set when the workflow launched as a cloud session
  warning?: string;
  error?: string;
};
```

在工具接受调用后立即返回。最终结果稍后作为任务完成到达。在将运行视为已启动之前检查 `error`：脚本如果语法检查失败，会返回 `status: "async_launched"` 并设置 `error`，且永远不会运行。

| 字段 | 类型 | 描述 |
| - | - | - |
| `status` | `"async_launched" \| "remote_launched"` | 工具接受了调用。`"async_launched"` 用于进程内运行，`"remote_launched"` 用于分派到云端会话而不是在进程内运行的运行 |
| `taskId` | `string` | 运行的后台任务标识符 |
| `taskType` | `"local_workflow" \| "remote_agent"` | 已注册后台任务的任务类型，与 `status` 分支匹配 |
| `workflowName` | `string` | 工作流脚本中的 `meta.name` |
| `runId` | `string` | 工作流运行标识符，用于在后续调用中作为 `resumeFromRunId` 传递。对于 `remote_launched` 运行不存在，其中云端会话 URL 是恢复句柄 |
| `summary` | `string` | 工作流功能的单行描述 |
| `transcriptDir` | `string` | 执行期间写入子代理会话记录的目录 |
| `scriptPath` | `string` | 此运行的持久化工作流脚本的路径。编辑它并作为 `scriptPath` 传回以重新运行而无需重新发送脚本 |
| `sessionUrl` | `string` | 云端会话 URL，当 `status` 为 `"remote_launched"` 时设置 |
| `warning` | `string` | 非阻塞性提示，例如本地 git 状态与云端会话将克隆的推送分支不同 |
| `error` | `string` | 当脚本语法检查失败时设置。存在时，尽管启动状态，运行未启动 |

### TodoWrite

**工具名称：** `TodoWrite`

```typescript theme={null}
type TodoWriteOutput = {
  oldTodos: Array<{
    content: string;
    status: "pending" | "in_progress" | "completed";
    activeForm: string;
  }>;
  newTodos: Array<{
    content: string;
    status: "pending" | "in_progress" | "completed";
    activeForm: string;
  }>;
};
```

返回之前和更新的任务列表。

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
> 请参阅[模型可用性](https://code.claude.com/docs/zh-CN/agent-sdk/todo-tracking.md#model-availability)以选择加入。

### TaskCreate

**工具名称：** `TaskCreate`

```typescript theme={null}
type TaskCreateOutput = {
  task: {
    id: string;
    subject: string;
  };
};
```

返回创建的任务及其分配的 ID。

### TaskUpdate

**工具名称：** `TaskUpdate`

```typescript theme={null}
type TaskUpdateOutput = {
  success: boolean;
  taskId: string;
  updatedFields: string[];
  error?: string;
  statusChange?: {
    from: string;
    to: string;
  };
};
```

返回更新结果，包括哪些字段已更改。

### TaskGet

**工具名称：** `TaskGet`

```typescript theme={null}
type TaskGetOutput = {
  task: {
    id: string;
    subject: string;
    description: string;
    status: "pending" | "in_progress" | "completed";
    blocks: string[];
    blockedBy: string[];
  } | null;
};
```

返回完整的任务记录，或在找不到 ID 时返回 `null`。

### TaskList

**工具名称：** `TaskList`

```typescript theme={null}
type TaskListOutput = {
  tasks: Array<{
    id: string;
    subject: string;
    status: "pending" | "in_progress" | "completed";
    owner?: string;
    blockedBy: string[];
  }>;
};
```

返回当前列表中所有任务的快照。

### ExitPlanMode

**工具名称：** `ExitPlanMode`

```typescript theme={null}
type ExitPlanModeOutput = {
  plan: string | null;
  isAgent: boolean;
  filePath?: string;
  hasTaskTool?: boolean;
  planWasEdited?: boolean;
  awaitingLeaderApproval?: boolean;
  requestId?: string;
};
```

返回退出计划模式后的计划状态。

### ListMcpResources

**工具名称：** `ListMcpResourcesTool`

```typescript theme={null}
type ListMcpResourcesOutput = Array<{
  uri: string;
  name: string;
  mimeType?: string;
  description?: string;
  server: string;
}>;
```

返回可用 MCP 资源的数组。

### ReadMcpResource

**工具名称：** `ReadMcpResourceTool`

```typescript theme={null}
type ReadMcpResourceOutput = {
  contents: Array<{
    uri: string;
    mimeType?: string;
    text?: string;
    blobSavedTo?: string;
  }>;
  error?: string;
};
```

返回请求的 MCP 资源的内容。

### EnterWorktree

**工具名称：** `EnterWorktree`

```typescript theme={null}
type EnterWorktreeOutput = {
  worktreePath: string;
  worktreeBranch?: string;
  message: string;
};
```

返回有关 git worktree 的信息。

### ExitWorktree

**工具名称：** `ExitWorktree`

```typescript theme={null}
type ExitWorktreeOutput = {
  action: "keep" | "remove";
  originalCwd: string;
  worktreePath: string;
  worktreeBranch?: string;
  tmuxSessionName?: string;
  discardedFiles?: number;
  discardedCommits?: number;
  message: string;
};
```

返回采取的操作和有关退出的 worktree 的详细信息。

### EnterPlanMode

**工具名称：** `EnterPlanMode`

```typescript theme={null}
type EnterPlanModeOutput = {
  message: string;
};
```

返回进入计划模式的确认。

### CronCreate

**工具名称：** `CronCreate`

```typescript theme={null}
type CronCreateOutput = {
  id: string;
  humanSchedule: string;
  recurring: boolean;
  durable?: boolean; // true when persisted to .claude/scheduled_tasks.json; false when session-only
  nextFireAt?: number;
};
```

返回作业 ID、计划的人类可读描述以及下次运行时间。`nextFireAt` 以纪元毫秒给出该时间，且已应用[抖动](https://code.claude.com/docs/zh-CN/scheduled-tasks.md#jitter)，因此它就是作业被设定的运行时间。该字段需要 TypeScript Agent SDK v0.3.299 或更高版本以及 Claude Code v2.1.299 或更高版本。

### CronDelete

**工具名称：** `CronDelete`

```typescript theme={null}
type CronDeleteOutput = {
  id: string;
};
```

返回已删除作业的 ID。

### CronList

**工具名称：** `CronList`

```typescript theme={null}
type CronListOutput = {
  jobs: {
    id: string;
    cron: string;
    humanSchedule: string;
    prompt: string;
    recurring?: boolean;
    durable?: boolean;
    nextFireAt?: number;
  }[];
};
```

返回计划的 cron 作业：来自 `.claude/scheduled_tasks.json` 的持久作业和来自当前会话的仅会话作业。仅会话作业携带 `durable: false`；从磁盘读取的作业省略该字段。

`nextFireAt` 是作业的下次运行时间（以纪元毫秒表示），已包含[抖动](https://code.claude.com/docs/zh-CN/scheduled-tasks.md#jitter)。任何值都不会早于 `CronList` 调用的时间：对于运行时间[已过但未运行](https://code.claude.com/docs/zh-CN/scheduled-tasks.md#how-scheduled-tasks-run)的作业，该字段携带调用时的时间。该字段需要 TypeScript Agent SDK v0.3.299 或更高版本以及 Claude Code v2.1.299 或更高版本。

### ScheduleWakeup

**工具名称：** `ScheduleWakeup`

```typescript theme={null}
type ScheduleWakeupOutput = {
  scheduledFor: number;
  clampedDelaySeconds: number;
  wasClamped: boolean;
  stopped?: boolean;
  cancelledWakeups?: number;
};
```

返回唤醒将触发的时间作为纪元毫秒时间戳、实际使用的延迟以及请求的延迟是否被限制。`stopped` 字段在调用以 `stop: true` 结束循环时为 `true`。它需要 Claude Code v2.1.202 或更高版本。`cancelledWakeups` 字段计算 `stop: true` 调用取消了多少待处理唤醒。值为 0 表示没有待处理，重复 `/loop` cron 不会被 `stop: true` 取消。它需要 Claude Code v2.1.206 或更高版本。

### RemoteTrigger

**工具名称：** `RemoteTrigger`

```typescript theme={null}
type RemoteTriggerOutput = {
  status: number;
  json: string;
  summary?: string;
};
```

返回触发操作的 API 响应状态和正文。

### PushNotification

**工具名称：** `PushNotification`

```typescript theme={null}
type PushNotificationOutput = {
  message: string;
  pushSent?: boolean;
  localSent?: boolean;
  disabledReason?: "config_off" | "user_present" | "no_transport";
  sentAt?: string;
};
```

返回传递详细信息，包括是否发送了推送或本地通知以及跳过传递的原因。

### ReportFindings

**工具名称：** `ReportFindings`

```typescript theme={null}
type ReportFindingsOutput = {
  count: number;
  level?: "low" | "medium" | "high" | "xhigh" | "max";
  findings: Array<{
    file: string;
    line?: number;
    summary: string;
    failure_scenario: string;
    short_summary?: string;
    category?: string;
    verdict?: "CONFIRMED" | "PLAUSIBLE";
    outcome?: "fixed" | "skipped" | "no_change_needed";
  }>;
};
```

返回报告的发现数、Claude 传入的 `level` 值以及为结果正文回显的发现。需要 Claude Code v2.1.196 或更高版本。回显的 `short_summary` 字段需要 Claude Code v2.1.212 或更高版本。

### Artifact

**工具名称：** `Artifact`

```typescript theme={null}
type ArtifactOutput =
  | {
      url: string;
      path: string;
      title?: string;
      version?: string;
      capabilities?: unknown;
      stored?: {
        contract: string;
        capabilities?: Record<string, unknown>;
      };
      warnings?: string[];
      contract?: string;
      updated?: boolean;
      liveSubscription?: string;
    }
  | {
      artifacts: Array<{
        title: string;
        url: string;
        updatedAt?: string;
        rel?: "mine" | "shared";
      }>;
      truncated?: boolean;
      total?: number;
      total_at_least?: true;
      scope?: "shared" | "all";
    };
```

返回已发布页面的 `url` 和为发布操作发布的本地 `path`，当发布重新部署现有 Artifact 时 `updated` 设置为 true，`warnings` 携带任何发布时建议。列表操作返回 `artifacts` 行，当存在比请求限制更多的 Artifact 时 `truncated` 设置。在作用域不是 `"mine"` 的列表上，每行携带 `rel` 标记用户是否拥有该 Artifact 或该 Artifact 是与他们共享的，输出的 `scope` 记录哪个非默认作用域产生了列表；两者在默认列表上不存在。

列表结果还会报告 `total`，即与所列作用域匹配的 Artifact 数量，包括超出 `limit` 的部分。设置 `total_at_least` 时，该数字是下界，可能还存在更多 Artifact。这两个字段都需要 Agent SDK v0.3.292 或更高版本。

### Projects

**工具名称：** `Projects`

```typescript theme={null}
type ProjectsOutput =
  | {
      method: "project_info";
      notice?: string;
      name: string;
      description: string;
      instructions: string;
      docs: Array<{ path: string; created_at: string | null }>;
      files?: Array<{
        path: string;
        file_kind: string;
        created_at: string | null;
      }>;
      sync_sources?: Array<{
        type: string | null;
        config: Record<string, unknown>;
      }>;
      knowledge: {
        knowledge_size: number;
        max_knowledge_size: number;
      };
    }
  | {
      method: "project_read";
      notice?: string;
      path: string;
      file_kind?: string;
      content?: string;
      local_file?: string;
      created_at: string | null;
    }
  | {
      method: "project_search";
      notice?: string;
      rag: boolean;
      hits?: Array<{ name?: string; doc_uuid?: string; text?: string }>;
      docs?: string[];
    }
  | {
      method: "project_write";
      notice?: string;
      path: string;
      doc_uuid: string;
      replaced: boolean;
      present_to_user?: boolean;
      local_path?: string;
    }
  | {
      method: "project_delete";
      notice?: string;
      path: string;
      deleted: boolean;
    };
```

在 `method` 字段上进行区分，镜像输入。`project_read` 在 `content` 中内联返回小文本文档，并将较大的文档写入 `local_file` 路径；`project_search` 当项目的索引可用时返回 RAG `hits` 且 `rag: true`，否则回退到 `docs` 路径列表。

### ReadMcpResourceDir

**工具名称：** `ReadMcpResourceDirTool`

```typescript theme={null}
type ReadMcpResourceDirOutput = {
  resources: Array<{
    uri: string;
    name: string;
    mimeType?: string;
  }>;
  error?: string;
};
```

返回目录资源的直接子项。子目录显示为 mimeType `"inode/directory"`；`error` 在服务器无法列出目录时携带人类可读的消息。

### RefreshMcpTools

**工具名称：** `RefreshMcpTools`

```typescript theme={null}
type RefreshMcpToolsOutput = Array<{
  server: string;
  status: "refreshed" | "error" | "not_connected";
  toolCount?: number; // tools now available from this server
  added?: string[]; // tool names this refresh added
  removed?: string[]; // tool names this refresh removed
  error?: string; // why the refresh failed or the server was unavailable
}>;
```

返回每个服务器一个条目：`refreshed` 表示重新查询的工具列表已应用，`error` 表示重新查询失败且保留了之前的工具集，`not_connected` 表示服务器没有实时连接来查询。

### ShowOnboardingRolePicker

**工具名称：** `ShowOnboardingRolePicker`

```typescript theme={null}
type ShowOnboardingRolePickerOutput = {
  role?: string;
  dismissed?: boolean;
};
```

返回用户的选择：当他们选择角色芯片或输入一个时为 `role`，当他们关闭选择器时为 `dismissed: true`。空对象表示用户批准了调用而未选择角色。

### McpOutput

**工具名称：** 形式为 `mcp__<server>__<tool>` 的动态 MCP 工具名称

```typescript theme={null}
type McpOutput =
  | string
  | {
      type: string;
      [k: string]: unknown;
    }[]
  | {
      [k: string]: unknown;
    };
```

MCP 工具结果作为字符串或内容块数组返回，取决于服务器。导出类型中的尾部纯对象分支是 schema 生成过程的产物。对于同时携带 `structuredContent` 或资源链接的结果，请参阅 [`tool_use_result`](#sdkusermessage)，它在其 `content` 成员中保存此值。在运行时值也可能是 `undefined`，尽管导出的类型不对此建模。

## 权限类型

### `PermissionUpdate`

用于更新权限的操作。

```typescript theme={null}
type PermissionUpdate =
  | {
      type: "addRules";
      rules: PermissionRuleValue[];
      behavior: PermissionBehavior;
      destination: PermissionUpdateDestination;
    }
  | {
      type: "replaceRules";
      rules: PermissionRuleValue[];
      behavior: PermissionBehavior;
      destination: PermissionUpdateDestination;
    }
  | {
      type: "removeRules";
      rules: PermissionRuleValue[];
      behavior: PermissionBehavior;
      destination: PermissionUpdateDestination;
    }
  | {
      type: "setMode";
      mode: PermissionMode;
      destination: PermissionUpdateDestination;
    }
  | {
      type: "addDirectories";
      directories: string[];
      destination: PermissionUpdateDestination;
    }
  | {
      type: "removeDirectories";
      directories: string[];
      destination: PermissionUpdateDestination;
    };
```

### `PermissionBehavior`

```typescript theme={null}
type PermissionBehavior = "allow" | "deny" | "ask";
```

### `PermissionUpdateDestination`

```typescript theme={null}
type PermissionUpdateDestination =
  | "userSettings" // 全局用户设置
  | "projectSettings" // 每个目录的项目设置
  | "localSettings" // 本地项目设置
  | "session" // 仅当前会话
  | "cliArg"; // CLI 参数
```

### `PermissionRuleValue`

```typescript theme={null}
type PermissionRuleValue = {
  toolName: string;
  ruleContent?: string;
};
```

## 其他类型

### `ApiKeySource`

会话请求所用 API 密钥的来源，在 [`SDKSystemMessage`](#sdksystemmessage) init 消息中以 `apiKeySource` 报告。

```typescript theme={null}
type ApiKeySource =
  | "ANTHROPIC_API_KEY"
  | "apiKeyHelper"
  | "/login managed key"
  | "none"
  | "user"
  | "project"
  | "org"
  | "temporary"
  | "oauth";
```

Claude Code 会报告以下四个值之一：

| 值 | 使用的密钥 |
| - | - |
| `ANTHROPIC_API_KEY` | `ANTHROPIC_API_KEY` 环境变量中的密钥 |
| `apiKeyHelper` | 您的 [`apiKeyHelper`](https://code.claude.com/docs/zh-CN/settings-reference#apikeyhelper) 命令返回的密钥 |
| `/login managed key` | 您使用 [Claude Console 账户](https://code.claude.com/docs/zh-CN/authentication.md#claude-console-authentication)登录时 Claude Code 存储的密钥 |
| `none` | 没有 API 密钥。会话通过其他方式进行身份验证，例如 claude.ai 登录、bearer 令牌或云提供商 |

Agent SDK v0.3.234 及更高版本在该类型中列出这四个值。该类型还保留了 `user`、`project`、`org`、`temporary` 和 `oauth`，以便旧代码仍能编译，但 Claude Code 不会报告它们。

### `SdkBeta`

可通过 `betas` 选项启用的 beta 功能。有关更多信息，请参阅 [Beta headers](https://platform.claude.com/docs/en/api/beta-headers)。

```typescript theme={null}
type SdkBeta = "context-1m-2025-08-07";
```

> [!WARNING]
>
> 在 Claude API 上，`context-1m-2025-08-07` beta 已针对 Claude Sonnet 4.5 和 Claude Sonnet 4 停用。如果您仍在这两个模型中的任一个上传递它，超出标准 200K token 上下文窗口的请求将返回错误，因此请将其从 `betas` 中移除。要以 1M token 上下文窗口运行会话，请将 `model` 设置为[默认以 1M 窗口运行](https://code.claude.com/docs/zh-CN/model-config.md#extended-context)的模型，例如 `claude-sonnet-5-5` 或 `claude-opus-5-5`。对于只能通过其 `[1m]` 变体达到 1M 的模型，请在模型 ID 后附加该后缀，例如 `claude-opus-4-6[1m]`。

### `SlashCommand`

有关可用命令的信息。

```typescript theme={null}
type SlashCommand = {
  name: string;
  description: string;
  argumentHint: string;
  aliases?: string[];
  builtin?: boolean;
};
```

当某条命令是 Claude Code 自身的命令且输入 `/name` 即可运行它时，该行的 `builtin` 为 `true`。对于由用户、项目、插件或 MCP 服务器定义的命令，以及被上述某一来源[按名称替换](https://code.claude.com/docs/zh-CN/skills.md#resolve-skills-that-share-a-name)的内置命令，该字段不存在。需要 Agent SDK v0.3.277 或更高版本。

### `ModelInfo`

有关可用模型的信息。

```typescript theme={null}
type ModelInfo = {
  value: string;
  resolvedModel?: string;
  displayName: string;
  description: string;
  supportsEffort?: boolean;
  supportedEffortLevels?: ("low" | "medium" | "high" | "xhigh" | "max")[];
  supportsAdaptiveThinking?: boolean;
  supportsFastMode?: boolean;
  supportsAutoMode?: boolean;
};
```

| 字段 | 类型 | 描述 |
| :- | :- | :- |
| `value` | `string` | 在 API 调用中传递的模型标识符 |
| `resolvedModel` | `string \| undefined` | 此条目的 `value` 解析到的模型 ID，例如 `sonnet` 别名条目对应 `claude-sonnet-5-5`。需要 Claude Code v2.1.197 或更高版本。 |
| `displayName` | `string` | 人类可读的显示名称 |
| `description` | `string` | 模型能力的描述 |
| `supportsEffort` | `boolean \| undefined` | 此模型是否支持 effort 级别 |
| `supportedEffortLevels` | `("low" \| "medium" \| "high" \| "xhigh" \| "max")[] \| undefined` | 此模型接受的 effort 级别 |
| `supportsAdaptiveThinking` | `boolean \| undefined` | 此模型是否支持自适应思考，即由 Claude 决定何时思考以及思考多少 |
| `supportsFastMode` | `boolean \| undefined` | 此模型是否支持快速模式 |
| `supportsAutoMode` | `boolean \| undefined` | 此模型是否支持自动模式 |

### `AgentInfo`

有关可通过 Agent 工具调用的可用子代理的信息。

```typescript theme={null}
type AgentInfo = {
  name: string;
  description: string;
  model?: string;
};
```

| 字段 | 类型 | 描述 |
| :- | :- | :- |
| `name` | `string` | Agent 类型标识符（例如 `"Explore"`、`"general-purpose"`） |
| `description` | `string` | 何时使用此 Agent 的描述 |
| `model` | `string \| undefined` | 此 Agent 使用的模型：别名或模型 ID，或 `'inherit'` 表示使用父级的模型。当其为 `undefined` 时，Claude Code 按照[子代理模型顺序](https://code.claude.com/docs/zh-CN/sub-agents.md#choose-a-model)选择模型 |

### `McpServerProvenance`

提供某个 `mcp__*` 工具的 MCP 服务器，以及该服务器定义的来源。[`PreToolUse`](#pretoolusehookinput)、`PostToolUse`、`PostToolUseFailure`、`PermissionRequest` 和 `PermissionDenied` hook 输入以 `mcp_server` 携带它，[`CanUseTool`](#canusetool) 选项以 `mcpServer` 携带它。对于并非来自 MCP 服务器的工具，两者都会省略它。

```typescript theme={null}
type McpServerProvenance = {
  name: string;
  source: string;
};
```

| 字段 | 类型 | 描述 |
| :- | :- | :- |
| `name` | `string` | 服务器注册时使用的名称，与 [`mcpServerStatus()`](#query-object) 为其报告的值相同 |
| `source` | `string` | 服务器定义的来源：`sdk`、`plugin` 或某个配置作用域 |

`source` 取以下值之一。该集合是开放的，因此请将无法识别的值视为已配置的来源，绝不要视为 `sdk`：

* **`sdk`**：由您的应用程序注册的进程内服务器。只有 SDK 宿主应用程序才能注册此类服务器，因此已配置的服务器无论名称为何，都不会报告 `sdk`。
* **`plugin`**：由[插件](https://code.claude.com/docs/zh-CN/agent-sdk/plugins.md)提供的服务器。其 `name` 为[插件提供的 MCP 服务器](https://code.claude.com/docs/zh-CN/mcp.md#plugin-provided-mcp-servers)中描述的限定形式 `plugin:<plugin-name>:<server-name>`。
* **配置作用域**：`user`、`project`、`local`、`dynamic`、`managed`、`enterprise`、`claudeai` 或 `agent`。`.mcp.json` 服务器报告 `project`，[MCP 安装作用域](https://code.claude.com/docs/zh-CN/mcp.md#mcp-installation-scopes)定义了 `local`、`project` 和 `user`。您的应用程序通过 [`mcpServers` 选项](#options)传入的服务器（进程内 SDK 服务器除外）报告 `dynamic`。

请基于 `source` 做出信任决策，而不是基于 `name` 或 `mcp__<server>__` 工具名前缀。对于 `sdk` 以外的任何来源，`name` 都是不可信文本：在显示之前请对其进行转义。

`McpServerProvenance` 及携带它的字段需要 Agent SDK v0.3.274 或更高版本。

### `McpServerStatus`

已连接 MCP 服务器的状态。

```typescript theme={null}
type McpServerStatus = {
  name: string;
  status: "connected" | "failed" | "needs-auth" | "pending" | "disabled";
  serverInfo?: {
    name: string;
    version: string;
  };
  error?: string;
  config?: McpServerStatusConfig;
  scope?: string;
  source?: string;
  tools?: {
    name: string;
    description?: string;
    annotations?: {
      readOnly?: boolean;
      destructive?: boolean;
      openWorld?: boolean;
    };
    _meta?: Record<string, unknown>;
  }[];
};
```

使用 `source` 决定对服务器的信任程度，并使用每个 `tools` 条目上的 `_meta` 查找服务器附加到该工具的 UI 资源：

* **`source`**：服务器定义的来源，取值与 [`McpServerProvenance`](#mcpserverprovenance) 上的 `source` 相同。请基于 `source` 而不是 `name` 做出信任决策，因为只有您自己的进程内服务器才会报告 `sdk`，无论任何其他服务器以什么名称注册。如果您无法识别某个 `source` 值，请像对待任何 `source` 不是 `sdk` 的服务器一样对待该服务器。需要 Agent SDK v0.3.274 或更高版本
* **`_meta`**：服务器为该工具声明的 `_meta` 中的 `ui` 对象和 `ui/resourceUri` 字符串。Claude Code 会扣留所有其他 `_meta` 键。`ui/resourceUri` 是 `ui.resourceUri` 的一种较旧的顶层写法。当服务器为该工具附加了 [MCP Apps UI 资源](https://code.claude.com/docs/zh-CN/mcp.md#reference-mcp-resources)时，`ui.resourceUri` 或 `ui/resourceUri` 保存该资源的 `ui://` URI。使用 [`readMcpResource()`](#query-object) 获取该 URI，并在您的应用程序中渲染该资源。需要 Agent SDK v0.3.280 或更高版本

仅当 subtype 为 `init` 的 `system` 消息的 [`capabilities`](#sdksystemmessage) 数组列出 `mcp_tool_ui_meta_v1` 时，Claude Code 才会发送 `_meta`。您无需自行启用 `mcp_tool_ui_meta_v1`。

#### 当工具的 `_meta.ui` 仅包含 `visibility` 时

如果您的服务器为某个工具声明了 `ui.resourceUri`，但您的应用程序收到的 `_meta.ui` 仅包含 `visibility`，则说明服务器发送的 `ui` 对象未通过 Claude Code 的某项检查。请将服务器 `tools/list` 响应中的 `_meta` 与每个键的检查项进行比对：

* **`ui`**：如果以下三项检查全部通过，则原样到达您的应用程序，包括 `resourceUri` 和 `visibility` 以外的任何成员：

  * `resourceUri` 不存在，或者是以 `ui://` 开头的字符串
  * `visibility` 不存在，或者仅列出 `"model"` 和 `"app"`
  * 整个对象序列化为 JSON 后最多 4,096 个字符

  如果任何一项检查未通过，您的应用程序最多只会收到一个 `ui`，其唯一成员是由服务器所列出的 `"model"` 和 `"app"` 条目组成的 `visibility` 数组。
* **`ui/resourceUri`**：如果它是以 `ui://` 开头且最多 4,096 个字符的字符串，则会到达您的应用程序。否则 Claude Code 会将其丢弃

当 Claude Code 既没有可发送的 `ui` 成员，也没有可发送的 `ui/resourceUri` 字符串时，该 `tools` 条目完全不包含 `_meta`。

### `McpServerStatusConfig`

由 `mcpServerStatus()` 报告的 MCP 服务器配置。这是所有 MCP 服务器传输类型的联合类型。

```typescript theme={null}
type McpServerStatusConfig =
  | McpStdioServerConfig
  | McpSSEServerConfig
  | McpHttpServerConfig
  | McpSdkServerConfig
  | McpClaudeAIProxyServerConfig;
```

有关每种传输类型的详细信息，请参阅 [`McpServerConfig`](#mcpserverconfig)。

### `AccountInfo`

已通过身份验证用户的账户信息。

```typescript theme={null}
type AccountInfo = {
  email?: string;
  organization?: string;
  subscriptionType?: string;
  tokenSource?: string;
  apiKeySource?: string;
};
```

### `ModelUsage`

在结果消息中返回的按模型统计的使用情况。`costUSD` 值是客户端估算值。有关计费注意事项，请参阅[跟踪成本和使用情况](https://code.claude.com/docs/zh-CN/agent-sdk/cost-tracking.md)。

```typescript theme={null}
type ModelUsage = {
  inputTokens: number;
  outputTokens: number;
  thinkingTokens?: number;
  cacheReadInputTokens: number;
  cacheCreationInputTokens: number;
  webSearchRequests: number;
  costUSD: number;
  contextWindow: number;
  maxOutputTokens: number;
  canonicalModel?: string;
  provider?: string;
  costBasis?: 'list' | 'managed' | 'unknown';
};
```

`thinkingTokens` 统计此模型生成的思考 token。`outputTokens` 已包含它们，因此不要将两者相加。在某个轮次于记录该字段的 Claude Code 版本上运行之前，该字段不存在，因此在更早版本上开始的恢复会话会报告部分计数。`thinkingTokens` 需要 Agent SDK v0.3.257 或更高版本。

`canonicalModel` 和 `provider` 字段需要 Claude Code v2.1.218 或更高版本。`canonicalModel` 是价格查询所使用的规范模型 ID；它可能与作为条目键的原始模型字符串不同，例如当该字符串是特定于提供商的 ID 或别名时。

`provider` 指明提供该模型的 API 后端，例如 `firstParty`、`bedrock`、`vertex`、`foundry`、`anthropicAws`、`mantle` 或 `gateway`。

`costBasis` 指明为该模型最近一次请求定价所使用的价格表：`list` 表示标价，`managed` 表示 [`modelPricing`](https://code.claude.com/docs/zh-CN/settings-reference#modelpricing) 表，`unknown` 表示两者都未匹配该模型 ID。该字段需要 Claude Code v2.1.246 或更高版本。

### `ConfigScope`

```typescript theme={null}
type ConfigScope = "local" | "user" | "project";
```

### `NonNullableUsage`

[`Usage`](#usage) 的一个版本，其中除 `fallback_credit`（仍可为 `null`）外，所有可为空的字段都变为不可为空。

```typescript theme={null}
type NonNullableUsage = {
  [K in keyof Usage]: K extends "fallback_credit"
    ? Usage[K]
    : NonNullable<Usage[K]>;
};
```

### `Usage`

token 使用统计信息。这是来自 `@anthropic-ai/sdk` 的 `BetaUsage` 类型。

```typescript theme={null}
type Usage = {
  input_tokens: number;
  output_tokens: number;
  cache_creation_input_tokens: number | null;
  cache_read_input_tokens: number | null;
  cache_creation: {
    ephemeral_5m_input_tokens: number;
    ephemeral_1h_input_tokens: number;
  } | null;
  server_tool_use: BetaServerToolUsage | null;
  service_tier: "standard" | "priority" | "batch" | null;
  speed: "standard" | "fast" | null;
  inference_geo: string | null;
  iterations: BetaIterationsUsage | null;
  output_tokens_details: BetaOutputTokensDetails | null;
  fallback_credit: BetaFallbackCreditUsage | null;
};
```

`BetaServerToolUsage`、`BetaIterationsUsage`、`BetaOutputTokensDetails` 和 `BetaFallbackCreditUsage` 在 `@anthropic-ai/sdk` 中定义。

`output_tokens_details` 按类别细分计费的输出。它目前包含一个字段 `thinking_tokens: number`，统计模型作为内部推理生成的输出 token，包括思考块分隔符。`output_tokens_details` 字段需要 TypeScript SDK v0.3.228 或更高版本，该版本捆绑了 Claude Code v2.1.228。

* **计费**：请将此细分用于可观测性，而不是计费。`output_tokens` 仍是权威总数，`output_tokens - thinking_tokens` 近似于非推理输出。
* **计数涵盖的内容**：模型产生的原始推理，可能比响应体中返回的思考文本更长。API 通过对该原始文本重新进行 token 化来计算它，因此可能与模型的确切生成计数相差几个 token。
* **流式**：在流式的助手消息上，此细分与 `output_tokens` 一样是 `message_start` 占位符，不携带真实计数，因此请按照[从结果消息中读取输出 token](https://code.claude.com/docs/zh-CN/agent-sdk/cost-tracking.md#read-output-tokens-from-the-result-message) 的说明，从结果消息的 `usage` 中读取。在结果消息上，当模型或提供商未报告细分时，`thinking_tokens` 为 `0`。
* **`null` 情况**：在 Claude Code 合成的助手消息（例如 API 错误消息）上，`output_tokens_details` 本身为 `null`。

`Usage` 是否携带 `fallback_credit` 取决于您安装的 `@anthropic-ai/sdk`，该字段在 0.115.0 中添加。

### `CallToolResult`

MCP 工具结果类型（来自 `@modelcontextprotocol/sdk/types.js`）。`structuredContent` 是一个 JSON 对象，可以与 `content`（包括图像块）一起返回。请参阅[返回结构化数据](https://code.claude.com/docs/zh-CN/agent-sdk/custom-tools.md#return-structured-data)。

```typescript theme={null}
type CallToolResult = {
  content: Array<{
    type: "text" | "image" | "audio" | "resource" | "resource_link";
    // Additional fields vary by type
  }>;
  structuredContent?: Record<string, unknown>;
  isError?: boolean;
};
```

### `SDKMcpResourceLink`

MCP 工具以引用方式返回的一个文件。Claude Code 根据工具结果中的 `resource_link` 块构建每个条目，并将列表作为 [`SDKUserMessage.tool_use_result`](#sdkusermessage) 上的 `resourceLinks` 传递；当调用在后台完成时，则作为 [`SDKTaskNotificationMessage`](#sdktasknotificationmessage) 上的 `resource_links` 传递。需要 Agent SDK v0.3.257 或更高版本。

```typescript theme={null}
type SDKMcpResourceLink = {
  uri: string;
  name: string;
  title?: string;
  description?: string;
  mimeType?: string;
  size?: number;
  annotations?: Record<string, unknown>;
};
```

Claude Code 会丢弃 `uri` 或 `name` 不是字符串的块，并省略值不属于所列类型的可选字段。

| 字段 | 类型 | 描述 |
| :- | :- | :- |
| `uri` | `string` | 资源的 URI，与服务器返回的一致 |
| `name` | `string` | 服务器为资源指定的名称 |
| `title` | `string \| undefined` | 显示标题，当服务器设置了时 |
| `description` | `string \| undefined` | 描述，当服务器设置了时 |
| `mimeType` | `string \| undefined` | MIME 类型，当服务器设置了时 |
| `size` | `number \| undefined` | 以字节为单位的大小，当服务器设置了时 |
| `annotations` | `Record<string, unknown> \| undefined` | 该块的 MCP annotations 对象，当服务器设置了时 |

### `ThinkingConfig`

控制 Claude 的思考/推理行为。优先于已弃用的 `maxThinkingTokens`。

```typescript theme={null}
type ThinkingDisplay = "summarized" | "omitted";

type ThinkingConfig =
  | { type: "adaptive"; display?: ThinkingDisplay } // The model determines when and how much to reason (Opus 4.6+)
  | { type: "enabled"; budgetTokens?: number; display?: ThinkingDisplay } // Fixed thinking token budget
  | { type: "disabled" }; // No extended thinking
```

可选的 `display` 字段控制思考文本以 `"summarized"` 还是 `"omitted"` 方式返回。在 Claude Opus 4.7 及更高版本上，API 默认值为 `"omitted"`，因此请设置 `"summarized"` 以在 `thinking` 块中接收思考内容。Claude Code 不会将您的 `display` 值传递给某些提供商，例如 Amazon Bedrock 和 Google Cloud 的 Agent Platform。在这些提供商上，即使您将 `display` 设置为 `"summarized"`，Opus 4.7 及更高版本也会返回空的 `thinking` 块。

### `SpawnedProcess`

用于自定义进程生成的接口（与 `spawnClaudeCodeProcess` 选项一起使用）。`ChildProcess` 已满足此接口。

```typescript theme={null}
interface SpawnedProcess {
  stdin: Writable;
  stdout: Readable;
  readonly killed: boolean;
  readonly exitCode: number | null;
  kill(signal: NodeJS.Signals): boolean;
  on(
    event: "exit",
    listener: (code: number | null, signal: NodeJS.Signals | null) => void
  ): void;
  on(event: "error", listener: (error: Error) => void): void;
  once(
    event: "exit",
    listener: (code: number | null, signal: NodeJS.Signals | null) => void
  ): void;
  once(event: "error", listener: (error: Error) => void): void;
  off(
    event: "exit",
    listener: (code: number | null, signal: NodeJS.Signals | null) => void
  ): void;
  off(event: "error", listener: (error: Error) => void): void;
}
```

### `SpawnOptions`

传递给自定义 spawn 函数的选项。

```typescript theme={null}
interface SpawnOptions {
  command: string;
  args: string[];
  cwd?: string;
  env: Record<string, string | undefined>;
  signal: AbortSignal;
}
```

> [!NOTE]
>
> `signal` 字段告诉您的 spawn 函数何时拆除进程。请将其作为 `signal` 选项传递给 Node 的 `spawn()`，或将其传递给您的 VM 或容器拆除处理程序。
>
> 此信号不会在 [`Options.abortController`](#options) 中止的瞬间触发。SDK 会先关闭进程的 stdin 并等待约两秒，以便 CLI 能够干净地关闭，然后再中止此信号。如果要在调用方中止的那一刻立即作出反应，请监听您自己的 `Options.abortController.signal`，您的 spawn 函数可以从其外围作用域引用它。

### `McpSetServersResult`

`setMcpServers()` 操作的结果。

```typescript theme={null}
type McpSetServersResult = {
  added: string[];
  removed: string[];
  errors: Record<string, string>;
};
```

`setMcpServers()` 返回的 Promise 会在新添加的 stdio、HTTP 和 SSE 服务器连接成功或失败后 resolve，因此已连接服务器的工具在下一轮次即可使用。

调用 `setMcpServers()` 时，Claude Code 会应用以下规则：

* **调用未指定的服务器**：在[云端会话](https://code.claude.com/docs/zh-CN/claude-code-on-the-web.md)之外，Claude Code 会断开先前 `setMcpServers()` 调用添加的服务器以及进程内 SDK 服务器的连接，并在 `removed` 中列出它们。其他服务器会继续运行，且不会列在 `removed` 中，其中包括来自 [`mcpServers`](#options) 选项的 stdio、HTTP 和 SSE 服务器、来自设置文件的服务器以及插件提供的服务器。
* **先前 `setMcpServers()` 调用添加的 stdio、HTTP 和 SSE 服务器**：在云端会话之外，如果您以相同的配置再次指定某个服务器，它会保留其连接和工具，既不会出现在 `added` 中，也不会出现在 `removed` 中。如果您传入更改后的配置，Claude Code 会使用该配置重新连接该服务器，并将其列在 `added` 中，而不是 `removed` 中。
* **调用指定的、此前没有任何调用添加过的 stdio、HTTP 和 SSE 服务器**：在云端会话之外，Claude Code 会添加该服务器并将其列在 `added` 中；如果拒绝该服务器，则将其列在 `errors` 中。
* **调用指定的进程内 SDK 服务器**：已通过 `mcpServers` 或先前调用注册的服务器会保留其正在运行的实例和超时时间。如果您为其传入 [`tools` 策略列表](#mcpsdkserverconfigwithinstance)（即使是空列表），则会替换其策略；否则策略保持不变。要在云端会话之外运行新实例，请在一次调用中省略该服务器，然后在下一次调用中指定它。
* **CLI 在启动时启动的内置服务器**：如果调用指定了其中之一，Claude Code 会丢弃该条目并在 `errors` 中报告它。
* **与 MCP 安装作用域（例如 `.mcp.json`）中的服务器同名的 stdio、HTTP 或 SSE 服务器**：在该名称下，会话使用您的服务器的工具，而不是[安装作用域服务器](https://code.claude.com/docs/zh-CN/mcp.md#mcp-installation-scopes)的工具。在 Claude Code v2.1.305 之前，当您的应用[重新加载插件](#query-object)时，Claude Code 可能会断开您的服务器的连接并改为连接安装作用域服务器。

`added` 列出 Claude Code 添加或替换的服务器，无论它们是否已连接。连接失败的服务器会同时出现在 `added` 和 `errors` 中，失败文本位于 `errors` 下，并在 [`mcpServerStatus()`](#methods) 中有一行 `failed`。在 Claude Code v2.1.257 之前，连接尝试抛出异常的服务器只会在 `errors` 下报告。

以 `Blocked by enterprise` 开头的 `errors` 值表示策略拒绝了该服务器，而不是连接失败：

* `Blocked by enterprise policy (allowedMcpServers/deniedMcpServers)`：该服务器不在 `added` 中。某个 [`deniedMcpServers`](https://code.claude.com/docs/zh-CN/managed-mcp#policy-based-control-with-allowlists-and-denylists) 条目与其匹配，或者设置了 `allowedMcpServers` 列表但其中未包含该服务器。如果该列表位于您通过 [`settingSources`](#options) 加载的设置文件中，请编辑该文件。如果它位于托管设置中，请让您的管理员进行更改。
* `Blocked by enterprise managed policy`：该服务器在 `added` 中，并在 `mcpServerStatus()` 中有一行 `failed`。请询问管理员是哪项策略阻止了它。

### `RewindFilesResult`

`rewindFiles()` 操作的结果。

```typescript theme={null}
type RewindFilesResult = {
  canRewind: boolean;
  error?: string;
  filesChanged?: string[];
  insertions?: number;
  deletions?: number;
  skippedLinks?: number;
};
```

`skippedLinks` 统计回退出于链接安全考虑而拒绝恢复或删除的被跟踪路径：被跟踪路径上的符号链接、硬链接或其他非常规文件，不再解析到创建检查点时所指向位置的父目录，或无法安全读取的备份。该字段需要 Claude Code v2.1.216 或更高版本。使用 `rewindFiles(userMessageId, { dryRun: true })` 的预览调用永远不会设置它。

### `SDKStatusMessage`

状态更新消息（例如压缩中）。

```typescript theme={null}
type SDKStatusMessage = {
  type: "system";
  subtype: "status";
  status: "compacting" | null;
  permissionMode?: PermissionMode;
  uuid: UUID;
  session_id: string;
};
```

### `SDKTaskNotificationMessage`

后台任务完成、失败或被停止时的通知。后台任务包括 `run_in_background` Bash 命令、[Monitor](#monitor) 监视以及后台子代理。有关 `run_id` 和 `ambient` 字段，请参阅 [`SDKTaskStartedMessage`](#sdktaskstartedmessage)，其中定义了这些字段及其版本要求。

```typescript theme={null}
type SDKTaskNotificationMessage = {
  type: "system";
  subtype: "task_notification";
  task_id: string;
  run_id?: string;
  tool_use_id?: string;
  status: "completed" | "failed" | "stopped";
  reason?: "worker_restart";
  output_file: string;
  summary: string;
  ambient?: boolean;
  usage?: {
    total_tokens: number;
    tool_uses: number;
    duration_ms: number;
  };
  resource_links?: SDKMcpResourceLink[];
  uuid: UUID;
  session_id: string;
};
```

当任务因其自身完成、失败或停止以外的原因结束时，会设置 `reason`，该字段需要 Agent SDK v0.3.273 或更高版本。Claude Code 仅在通过 claude.ai 连接的会话中设置它：云端会话（包括在自托管运行器上运行的云端会话）以及 Remote Control 会话。本地 `query()` 调用永远不会设置它。其唯一的值 `worker_restart` 表示运行该任务的 Claude Code 进程已重启。该通知携带状态 `"stopped"`，因此请将该任务视为既未完成也未失败。

当 Claude Code [将耗时较长的 MCP 工具调用移到后台](https://code.claude.com/docs/zh-CN/mcp.md#automatic-backgrounding-of-long-tool-calls)时，该调用的 `tool_result` 块只包含一个占位符，调用的真实结果会在此通知中到达。请使用 `tool_use_id` 将通知与调用匹配。在 `completed` 通知上，`resource_links` 以 [`SDKMcpResourceLink`](#sdkmcpresourcelink) 条目的形式列出工具以引用方式返回的文件，其 50 个链接和 64 KiB 的限制与 [`tool_use_result.resourceLinks`](#sdkusermessage) 相同。当结果没有链接时，以及在非 MCP 工具调用任务的通知上，Claude Code 会省略 `resource_links`。`resource_links` 需要 Agent SDK v0.3.257 或更高版本。

Claude Code 会在其发送给模型的每个任务通知前添加一条提示，但带有 [`scheduled-trigger` subkind](#task-notification-subkinds) 标记的投递除外，这类投递改为携带分配任务的框架说明。该提示声明没有发生任何人工输入，因此模型不会将通知视为用户指令或批准。

要检测任务通知轮次，请检查 [`SDKUserMessage`](#sdkusermessage) 或 [`SDKResultMessage`](#sdkresultmessage) 上的 `origin.kind === "task-notification"`，而不是匹配提示文本。如果需要知道是什么触发了它，请从同一字段读取 `subkind`。在 v2.1.205 之前，Claude Code 不会在会话空闲时到达的通知上添加该提示。

### `SDKToolUseSummaryMessage`

对话中工具使用情况的摘要。

```typescript theme={null}
type SDKToolUseSummaryMessage = {
  type: "tool_use_summary";
  summary: string;
  preceding_tool_use_ids: string[];
  uuid: UUID;
  session_id: string;
};
```

### `SDKHookStartedMessage`

在 hook 开始执行时发出。

Claude Code 会立即将此消息、[`SDKHookProgressMessage`](#sdkhookprogressmessage) 和 [`SDKHookResponseMessage`](#sdkhookresponsemessage) 传递到消息流，包括在会话启动期间 `SessionStart` 或 `Setup` hook 仍在运行时。Claude Code v2.1.169 至 v2.1.203 会在 `SessionStart` 或 `Setup` hook 完成后一次性批量传递这些消息；v2.1.204 恢复了实时传递。

如果您使用 [`startup()`](#startup) 启动会话，则这些消息会在您调用 `.query()` 并迭代其结果后到达。

```typescript theme={null}
type SDKHookStartedMessage = {
  type: "system";
  subtype: "hook_started";
  hook_id: string;
  hook_name: string;
  hook_event: string;
  uuid: UUID;
  session_id: string;
};
```

### `SDKHookProgressMessage`

在 hook 运行期间发出，包含 stdout/stderr 输出。

```typescript theme={null}
type SDKHookProgressMessage = {
  type: "system";
  subtype: "hook_progress";
  hook_id: string;
  hook_name: string;
  hook_event: string;
  stdout: string;
  stderr: string;
  output: string;
  uuid: UUID;
  session_id: string;
};
```

### `SDKHookResponseMessage`

在 hook 执行完成时发出。

```typescript theme={null}
type SDKHookResponseMessage = {
  type: "system";
  subtype: "hook_response";
  hook_id: string;
  hook_name: string;
  hook_event: string;
  output: string;
  stdout: string;
  stderr: string;
  exit_code?: number;
  outcome: "success" | "error" | "cancelled";
  uuid: UUID;
  session_id: string;
};
```

### `SDKToolProgressMessage`

在工具执行期间定期发出，以指示进度。

```typescript theme={null}
type SDKToolProgressMessage = {
  type: "tool_progress";
  tool_use_id: string;
  tool_name: string;
  parent_tool_use_id: string | null;
  elapsed_time_seconds: number;
  task_id?: string;
  heartbeat?: boolean;
  subagent_type?: string;
  subagent_retry?: {
    agent_id: string;
    attempt: number;
    max_retries: number;
    retry_delay_ms: number;
    error_status: number | null;
    error_category: string;
  };
  uuid: UUID;
  session_id: string;
};
```

当工具调用在主对话中运行时，Claude Code 每 30 秒发出一条带有 `heartbeat: true` 的 `tool_progress` 消息。每个心跳都携带工具名称和已用秒数，因此您可以区分长时间运行的调用和停滞的会话。Claude Code 不会为子代理内部的工具调用发出心跳。`heartbeat` 字段需要 Agent SDK v0.3.214 或更高版本。在 v2.1.257 之前，Claude Code 也不会为前台的 Agent 工具调用发出心跳。

在 Agent 工具的非心跳 `tool_progress` 消息上，`subagent_type` 指明正在运行的子代理类型，例如 `general-purpose`。当该子代理因 API 错误（例如速率限制或过载）而等待退避时，会出现 `subagent_retry`，每次重试尝试对应一条消息。这两个字段都需要 Agent SDK v0.3.214 或更高版本。

要根据 `subagent_retry` 渲染重试指示器：

* 按 `parent_tool_use_id` 跟踪指示器，它对每个子代理都是唯一的。同一助手轮次中的并行子代理共享 `tool_use_id`，因此按它跟踪会导致一个子代理的更新清除另一个子代理的指示器。
* 当同一 `parent_tool_use_id` 的后续 `tool_progress` 到达且既没有 `subagent_retry` 也没有 `heartbeat: true` 时，或当工具的结果消息到达时，清除指示器。带有 `heartbeat: true` 的帧仅报告存活状态，因此在收到此类帧时请保留指示器。在持续重试下，`attempt` 可能超过 `max_retries`，因此不要根据计数器来判断是否清除。
* 将 `error_category` 视为用于选择您自己的消息文本的标记，而不是显示文本。其取值为 `rate_limit`、`overloaded`、`authentication_failed`、`server_error`、`cloud_credential_error` 和 `unknown`。请以处理 `unknown` 的方式处理无法识别的值，因为后续版本可能会添加新值。

### `SDKAuthStatusMessage`

在身份验证流程中发出。

```typescript theme={null}
type SDKAuthStatusMessage = {
  type: "auth_status";
  isAuthenticating: boolean;
  output: string[];
  error?: string;
  uuid: UUID;
  session_id: string;
};
```

### `SDKTaskStartedMessage`

在任务开始时发出。对于 Bash 命令和 [Monitor](#monitor) 监视，`task_type` 字段为 `"local_bash"`；对于子代理为 `"local_agent"`；否则为 `"remote_agent"`。

```typescript theme={null}
type SDKTaskStartedMessage = {
  type: "system";
  subtype: "task_started";
  task_id: string;
  run_id?: string;
  tool_use_id?: string;
  description: string;
  task_type?: string;
  is_backgrounded?: boolean;
  spawn_depth?: number;
  parent_task_id?: string;
  ambient?: boolean;
  uuid: UUID;
  session_id: string;
};
```

`run_id` 标识任务的一次运行。当任务以相同的 `task_id` 再次运行时（例如 [Claude 恢复的子代理](https://code.claude.com/docs/zh-CN/agent-sdk/subagents.md#resume-subagents)），新的运行会获得新的 `run_id`。该字段需要 Agent SDK v0.3.292 或更高版本。使用它来区分同一任务的各次运行：

* **将消息与某次运行匹配**：同一次运行的 `task_started`、[`task_progress`](#sdktaskprogressmessage)、[`task_updated`](#sdktaskupdatedmessage) 和 [`task_notification`](#sdktasknotificationmessage) 消息携带相同的 `run_id`。每条 [`SDKBackgroundTasksChangedMessage`](#sdkbackgroundtaskschangedmessage) 中该运行的条目也携带它，传递该运行通知的用户消息的 origin 上的 [`runId`](#task-notification-subkinds) 也是如此。
* **检测迟到的消息**：按普通字符串比较时，同一任务的各个运行 ID 按运行开始的顺序排序，因此如果某条消息的 `run_id` 排在您正在显示的运行之前，则它来自更早的运行。除该比较外，请将该值视为不透明值。
* **回退到 `task_id`**：当消息不携带 `run_id` 时（例如恢复的会话为关闭先前进程启动的任务而发出的 `task_notification`），请按 `task_id` 确定其归属。

例如，以下三条消息（已精简为其标识字段）来自一个被 Claude 恢复过一次的后台子代理。两条 `task_started` 消息共享同一个 `task_id`，结束第一次运行的通知携带第一次运行的 ID：

```json theme={null}
{"subtype": "task_started", "task_id": "a5c2e8f41b09d7a36", "run_id": "0mubbs7i8-6f0c2d41"}
{"subtype": "task_notification", "task_id": "a5c2e8f41b09d7a36", "run_id": "0mubbs7i8-6f0c2d41", "status": "completed"}
{"subtype": "task_started", "task_id": "a5c2e8f41b09d7a36", "run_id": "0mubc18m1-b7719e05"}
```

对于不属于会话工作的任务（例如 Claude Code 为其自身运行而执行的任务），`ambient` 为 `true`。实时更新监视器也属于 ambient，包括用户要求的监视器。请将 ambient 任务从活动指示器中排除。该字段需要 Agent SDK v0.3.247 或更高版本。

`ambient` 也出现在 [`SDKTaskNotificationMessage`](#sdktasknotificationmessage) 以及 [`SDKBackgroundTasksChangedMessage`](#sdkbackgroundtaskschangedmessage) 的条目上。

`is_backgrounded` 和 `spawn_depth` 描述 Claude Code 如何启动该任务。这两个字段都需要 Agent SDK v0.3.238 或更高版本。

* `is_backgrounded`：Claude Code 在 `"local_agent"` 和 `"local_bash"` 任务上设置它。`true` 表示任务在后台运行。`false` 表示任务在前台运行，启动它的工具调用会一直阻塞，直到任务完成或移到后台。
* `spawn_depth`：Claude Code 仅在 `"local_agent"` 任务上设置它。由主线程生成的子代理深度为 `1`。由深度为 `1` 的子代理生成的子代理深度为 `2`，依此类推。

[恢复的子代理](https://code.claude.com/docs/zh-CN/agent-sdk/subagents.md#resume-subagents)始终报告 `is_backgrounded: true`，因为 Claude Code 会在后台运行每个恢复的子代理。当前台任务稍后移到后台时，Claude Code 会在 [`task_updated`](#sdktaskupdatedmessage) 消息中报告新的 `is_backgrounded` 值，而不是发送第二条 `task_started`。

`parent_task_id` 保存启动此任务的子代理的 `task_id`。使用它将每个任务归组到启动它的子代理之下。Claude Code 会在子代理、Bash 和 [Monitor](#monitor) 任务上设置它。该字段需要 Agent SDK v0.3.292 或更高版本。在以下情况下该字段不存在：

* 任务由主线程启动
* Claude Code 不再跟踪父任务
* 任务由 [teammate](https://code.claude.com/docs/zh-CN/agent-teams.md) 或工作流中的 Agent 启动

父任务可能是前台任务，也可能是已经结束的任务，因此请将无法识别的 ID 视为没有父任务。

### `SDKTaskProgressMessage`

在子代理或后台任务运行期间定期发出。有关 `run_id` 字段，请参阅 [`SDKTaskStartedMessage`](#sdktaskstartedmessage)，其中定义了该字段及其版本要求。

对于子代理任务，`summary` 字段携带模型生成的进度摘要，仅在启用 [`agentProgressSummaries`](#options) 时填充。对于[已移到后台的 MCP 工具调用](https://code.claude.com/docs/zh-CN/mcp.md#automatic-backgrounding-of-long-tool-calls)，`summary` 携带 MCP 服务器最近报告的进度，不依赖于该选项。

```typescript theme={null}
type SDKTaskProgressMessage = {
  type: "system";
  subtype: "task_progress";
  task_id: string;
  run_id?: string;
  tool_use_id?: string;
  description: string;
  subagent_type?: string;
  usage: {
    total_tokens: number;
    tool_uses: number;
    duration_ms: number;
  };
  last_tool_name?: string;
  summary?: string;
  uuid: UUID;
  session_id: string;
};
```

### `SDKTaskUpdatedMessage`

在后台任务状态发生变化时发出，例如从 `running` 转换为 `completed` 时。请将 `patch` 合并到以 `task_id` 为键的本地任务映射中。`end_time` 字段是以毫秒为单位的 Unix 纪元时间戳，可与 `Date.now()` 比较。有关 `run_id` 字段，请参阅 [`SDKTaskStartedMessage`](#sdktaskstartedmessage)，其中定义了该字段及其版本要求。

```typescript theme={null}
type SDKTaskUpdatedMessage = {
  type: "system";
  subtype: "task_updated";
  task_id: string;
  run_id?: string;
  patch: {
    status?: "pending" | "running" | "completed" | "failed" | "killed";
    description?: string;
    end_time?: number;
    total_paused_ms?: number;
    error?: string;
    is_backgrounded?: boolean;
  };
  uuid: UUID;
  session_id: string;
};
```

### `SDKBackgroundTasksChangedMessage`

每当活动后台任务集合发生变化时发出：任务启动、完成、被终止、前台 Agent 被移到后台，或任务的 `description`、`ambient` 或 `parent_task_id` 字段发生变化。有关每个条目上的 `run_id` 和 `parent_task_id` 字段，请参阅 [`SDKTaskStartedMessage`](#sdktaskstartedmessage)，其中定义了这些字段及其版本要求。

`tasks` 数组是完整的活动集合。请用每次的负载替换任何缓存的集合，而不是对 `task_started` 和 `task_notification` 事件进行配对，这样下一次成员变化就会纠正您错过的任何事件。

当任务结束时，其 [`task_updated`](#sdktaskupdatedmessage) 和 [`task_notification`](#sdktasknotificationmessage) 会先于将其从列表中移除的 `background_tasks_changed` 到达。除此之外，相对于这些逐任务事件的顺序是未指定的。

启动时不会发出任何内容。每当会话的 CLI 进程启动或重启时，请重置为空集合，并由下一次成员变化重新填充。

当您向正在运行的会话发送重复的 `initialize` 控制请求时（例如在传输中断后使用 [`reinitialize()`](#query-object)），Claude Code 会在响应之后发送当前活动集合的快照，即使该集合为空。因此，重新连接的宿主无需等待下一次成员变化即可了解正在运行的内容。在 Agent SDK v0.3.239 之前，Claude Code 在重复的 `initialize` 之后不发送快照。

需要 Claude Code v2.1.203 或更高版本。

```typescript theme={null}
type SDKBackgroundTasksChangedMessage = {
  type: "system";
  subtype: "background_tasks_changed";
  tasks: {
    task_id: string;
    run_id?: string;
    task_type: string;
    subagent_type?: string;
    description: string;
    parent_task_id?: string;
    ambient?: boolean;
  }[];
  uuid: UUID;
  session_id: string;
};
```

在 [`task_type`](#sdktaskstartedmessage) 为 `"local_agent"` 的条目上，`subagent_type` 指明子代理类型，例如 `general-purpose` 或自定义子代理的名称。该字段需要 Agent SDK v0.3.293 或更高版本。

### `SDKThinkingTokensMessage`

在 Claude 生成思考块（包括经过编辑隐去的思考块）期间发出。`estimated_tokens` 是当前块中迄今为止生成的思考 token 的累计估算值，`estimated_tokens_delta` 是此帧携带的增量。请将这些估算值用于进度显示。

当模型或提供商报告细分时，顶层 Agent 循环的最终计数是结果消息的 [`usage.output_tokens_details.thinking_tokens`](#usage)，它[不包括子代理的 token](https://code.claude.com/docs/zh-CN/agent-sdk/cost-tracking.md#get-the-total-cost-of-a-query)。

需要 Claude Code v2.1.153 或更高版本。

```typescript theme={null}
type SDKThinkingTokensMessage = {
  type: "system";
  subtype: "thinking_tokens";
  estimated_tokens: number;
  estimated_tokens_delta: number;
  user_message_uuid?: string;
  uuid: UUID;
  session_id: string;
};
```

### `SDKSessionStateChangedMessage`

在 Claude Code 报告会话状态时发出。要接收这些消息，请设置 [`CLAUDE_CODE_EMIT_SESSION_STATE_EVENTS=1`](https://code.claude.com/docs/zh-CN/env-vars#variables)。Claude Code 可能会多次报告相同的状态，因此请将消息理解为会话的当前状态，而不是状态转换。

`state` 字段携带以下值之一：

* `running`：会话正在工作。
* `idle`：Claude Code 正在等待您的下一个提示词。
* `requires_action`：会话因等待对其发送给宿主的请求（例如权限提示）的答复而被阻塞。

一个轮次的 `idle` 消息与其 `result` 消息可能以任意顺序到达。要更改 `idle` 是否等待后台工作（例如后台子代理或[工作流](https://code.claude.com/docs/zh-CN/workflows.md)运行），请参阅 [`CLAUDE_CODE_BG_TASKS_REPORT_RUNNING`](https://code.claude.com/docs/zh-CN/env-vars#variables)。

```typescript theme={null}
type SDKSessionStateChangedMessage = {
  type: "system";
  subtype: "session_state_changed";
  state: "idle" | "running" | "requires_action";
  uuid: UUID;
  session_id: string;
};
```

### `SDKFilesPersistedEvent`

在文件检查点持久化到磁盘时发出。

```typescript theme={null}
type SDKFilesPersistedEvent = {
  type: "system";
  subtype: "files_persisted";
  files: { filename: string; file_id: string }[];
  failed: { filename: string; error: string }[];
  processed_at: string;
  uuid: UUID;
  session_id: string;
};
```

### `SDKRateLimitEvent`

在会话遇到速率限制时发出。

```typescript theme={null}
type SDKRateLimitEvent = {
  type: "rate_limit_event";
  rate_limit_info: {
    status: "allowed" | "allowed_warning" | "rejected";
    resetsAt?: number;
    utilization?: number;
    errorCode?: "credits_required";
    canUserPurchaseCredits?: boolean;
    hasChargeableSavedPaymentMethod?: boolean;
  };
  uuid: UUID;
  session_id: string;
};
```

当 `errorCode` 为 `"credits_required"` 时，表示拒绝来自已用尽所含用量的 claude.ai 订阅，在用户购买使用额度之前，会话无法继续。`canUserPurchaseCredits` 表示已通过身份验证的用户是否可以为该账户购买额度，`hasChargeableSavedPaymentMethod` 表示是否存有已保存的付款方式。在不属于 credits-required 拒绝的速率限制事件上，这三个字段都不存在。需要 Claude Code v2.1.181 或更高版本。

### `SDKLocalCommandOutputMessage`

Claude Code 不会发出此消息类型。当您将 `/context` 或 `/usage` 等命令作为提示词发送时，其输出会以 [`SDKAssistantMessage`](#sdkassistantmessage) 的形式到达。

```typescript theme={null}
type SDKLocalCommandOutputMessage = {
  type: "system";
  subtype: "local_command_output";
  content: string;
  uuid: UUID;
  session_id: string;
};
```

### `SDKCommandsChangedMessage`

在会话中途可用命令集合发生变化时发出，例如当 Agent 进入子目录时 Claude Code 发现了 skill。`commands` 数组是完整的更新列表，因此请用此负载替换任何缓存的命令列表。在此消息之后调用 [`supportedCommands()`](#query-object) 会返回相同的更新列表，因为该方法会跟踪最新的推送；这需要 Agent SDK v0.3.216 或更高版本。在更早的 SDK 版本中，`supportedCommands()` 返回初始化时捕获的快照，永远不会反映会话中途的变化。

当 MCP 服务器的[提示词](https://code.claude.com/docs/zh-CN/mcp.md#use-mcp-prompts-as-commands)加入或离开列表时（例如服务器在会话开始后才完成连接），Claude Code 也会发出此消息。这需要 Claude Code v2.1.281 或更高版本。

```typescript theme={null}
type SDKCommandsChangedMessage = {
  type: "system";
  subtype: "commands_changed";
  commands: SlashCommand[];
  uuid: UUID;
  session_id: string;
};
```

### `SDKPromptSuggestionMessage`

在启用 [`promptSuggestions`](#options) 且 Claude Code 为某个轮次生成了建议时，于该轮次之后发出。包含预测的下一个用户提示词。有关不会获得建议的轮次，请参阅 [Claude Code 何时跳过建议](https://code.claude.com/docs/zh-CN/interactive-mode.md#when-claude-code-skips-suggestions)。

```typescript theme={null}
type SDKPromptSuggestionMessage = {
  type: "prompt_suggestion";
  suggestion: string;
  uuid: UUID;
  session_id: string;
};
```

### `SDKConversationResetMessage`

在会话的对话被替换但会话未结束时发出。在 `query()` 调用中，只有 `/clear` 及其别名会产生此消息。请在 `new_conversation_id` 下挂载一个空的会话记录，并丢弃任何缓存的会话标题。

```typescript theme={null}
type SDKConversationResetMessage = {
  type: "conversation_reset";
  new_conversation_id: UUID;
  uuid: UUID;
  session_id: string;
  trigger?: "clear" | "plan_mode_exit" | "fresh_session" | "onboarding";
  user_message_uuid?: string;
  timestamp?: string;
};
```

可选字段描述此次重置：

* `trigger`：是什么丢弃了对话。请在每条 `conversation_reset` 消息上重置您的会话记录，包括该字段不存在或携带无法识别的值的消息。
* `user_message_uuid`：携带 `/clear` 的用户消息的 `uuid`。使用它将重置与该消息匹配。
* `timestamp`：重置发生的时间，为 UTC 的 ISO 8601 字符串。请将其用于显示，而不是用于消息排序。

`trigger`、`user_message_uuid` 和 `timestamp` 字段需要 Claude Code v2.1.281 或更高版本。

在 Claude Code v2.1.203 及更高版本中，SDK 发布的类型定义声明了 `SDKConversationResetMessage`。在 v2.1.203 之前，`SDKMessage` 引用了该类型却未声明它，因此在禁用 `skipLibCheck` 时，基于 `type === "conversation_reset"` 的类型收窄无法通过类型检查。

### `BUILTIN_TOOL_NAMES` 和 `LEGACY_TOOL_NAME_ALIASES`

使用这两个常量来检查您的应用程序用户输入的工具名称。两者中每个名称的写法都与您在权限规则、hook 匹配器以及 `tools` 和 `disallowedTools` 选项中的写法一致。请从 `@anthropic-ai/claude-agent-sdk` 或 `@anthropic-ai/claude-agent-sdk/core` 导入它们。它们需要 Agent SDK v0.3.301 或更高版本，并且处于 alpha 阶段。

SDK 的类型声明为 `BUILTIN_TOOL_NAMES` 提供了只读元组类型，为 `LEGACY_TOOL_NAME_ALIASES` 提供了字符串到字符串的记录类型：

```typescript theme={null}
const BUILTIN_TOOL_NAMES: readonly ["Agent", /* ...more names... */ "Write"];
const LEGACY_TOOL_NAME_ALIASES: Readonly<Record<string, string>>;
```

* **`BUILTIN_TOOL_NAMES`**：按字母顺序排列的 Claude Code 内置工具名称，例如 `Agent`、`Bash` 和 `Read`。不包括 MCP 工具名称。由于该类型是字符串字面量元组，`(typeof BUILTIN_TOOL_NAMES)[number]` 会为您提供这些名称的联合类型
* **`LEGACY_TOOL_NAME_ALIASES`**：权限规则、hook 匹配器以及 `tools` 和 `disallowedTools` 选项仍然接受的早期工具名称。每个键是一个早期名称，其值是当前名称：例如，`Task` 映射到 `Agent`，`KillShell` 和 `KillBash` 映射到 `TaskStop`

您可以通过以下两种检测中的任意一种来确认某个名称：`BUILTIN_TOOL_NAMES` 包含它，或者 `Object.hasOwn(LEGACY_TOOL_NAME_ALIASES, name)` 返回 `true`。两种检测都未通过的名称仍可能是有效的，因为 `BUILTIN_TOOL_NAMES` 不包括 MCP 工具和部分内置工具。如果您基于 `BUILTIN_TOOL_NAMES` 构建拒绝列表，Claude 仍然可以使用该常量未包含的工具。要拒绝所有工具，请改为设置 [`disallowedTools: ["*"]`](https://code.claude.com/docs/zh-CN/agent-sdk/permissions#allow-and-deny-rules)。

在 `BUILTIN_TOOL_NAMES` 中找到某个名称并不代表会话拥有该工具：`TodoWrite` 虽然[默认处于禁用状态](https://code.claude.com/docs/zh-CN/tools-reference.md)，但仍列在其中。要查看会话拥有哪些工具，请读取其 [`SDKSystemMessage`](#sdksystemmessage) 的 `tools` 数组。该数组包含 MCP 工具，并将 Agent 工具列为 `Task`，因此在与 `BUILTIN_TOOL_NAMES` 比较之前，请先通过 `LEGACY_TOOL_NAME_ALIASES` 映射每个名称。

### `AbortError`

用于中止操作的自定义错误类。

```typescript theme={null}
class AbortError extends Error {}
```

`AbortError` 是 SDK 类型化 API 中唯一的错误类。其他失败（例如 Claude Code 进程退出或启动失败）会以不带任何可匹配 SDK 类的错误拒绝消息迭代。[故障排除](https://code.claude.com/docs/zh-CN/agent-sdk/troubleshooting.md)按消息列出了这些错误，并给出了每种错误的原因和修复方法。

## 沙箱配置

### `SandboxSettings`

沙箱行为的配置。使用它以编程方式启用命令沙箱并配置网络限制。

```typescript theme={null}
type SandboxSettings = {
  enabled?: boolean;
  failIfUnavailable?: boolean;
  autoAllowBashIfSandboxed?: boolean;
  excludedCommands?: string[];
  allowUnsandboxedCommands?: boolean;
  network?: SandboxNetworkConfig;
  filesystem?: SandboxFilesystemConfig;
  ignoreViolations?: Record<string, string[]>;
  enableWeakerNestedSandbox?: boolean;
  allowNestedUserNamespaces?: boolean;
  ripgrep?: { command: string; args?: string[] };
};
```

| 属性 | 类型 | 默认值 | 描述 |
| :- | :- | :- | :- |
| `enabled` | `boolean` | `false` | 为命令执行启用沙箱模式 |
| `failIfUnavailable` | `boolean` | `true` | 如果 `enabled` 为 `true` 但沙箱无法启动，则在启动时停止。设置为 `false` 可回退到非沙箱执行，并在 stderr 上输出警告 |
| `autoAllowBashIfSandboxed` | `boolean` | `true` | 启用沙箱时自动批准 Bash 命令 |
| `excludedCommands` | `string[]` | `[]` | 绕过沙箱限制的命令，例如 `['docker *']`。这些命令会自动在沙箱外运行，无需模型参与；有关条目何时适用，请参阅 [`sandbox.excludedCommands`](https://code.claude.com/docs/zh-CN/settings-reference#sandbox-excludedcommands) |
| `allowUnsandboxedCommands` | `boolean` | `true` | 允许模型请求在沙箱外运行命令。为 `true` 时，模型可以在工具输入中设置 `dangerouslyDisableSandbox`，此时将回退到[权限系统](#permissions-fallback-for-unsandboxed-commands) |
| `network` | [`SandboxNetworkConfig`](#sandboxnetworkconfig) | `undefined` | 网络相关的沙箱配置 |
| `filesystem` | [`SandboxFilesystemConfig`](#sandboxfilesystemconfig) | `undefined` | 用于读/写限制的文件系统相关沙箱配置 |
| `ignoreViolations` | `Record<string, string[]>` | `undefined` | 从命令子字符串（或表示所有命令的 `*`）到要忽略的违规文本子字符串的映射，例如 `{ "*": ['/etc/hosts'] }`；请参阅 [`sandbox.ignoreViolations`](https://code.claude.com/docs/zh-CN/settings-reference#sandbox-ignoreviolations) |
| `enableWeakerNestedSandbox` | `boolean` | `false` | 为兼容性启用较弱的嵌套沙箱 |
| `allowNestedUserNamespaces` | `boolean` | `false` | 允许 Linux 和 WSL2 上在沙箱中运行的命令创建用户命名空间。在将其设置为 `true` 之前，请阅读 [`sandbox.allowNestedUserNamespaces`](https://code.claude.com/docs/zh-CN/settings-reference#sandbox-allownestedusernamespaces)，其中介绍了需要此设置的工具以及您将关闭的安全边界。需要 Claude Code v2.1.305 或更高版本 |
| `ripgrep` | `{ command: string; args?: string[] }` | `undefined` | 用于沙箱环境的自定义 ripgrep 二进制文件配置 |

> [!NOTE]
>
> 沙箱依赖于平台支持，在 Linux 上还依赖 `bubblewrap` 和 `socat` 等工具。当 `enabled` 为 `true` 且沙箱无法启动时，`query()` 会报告一条 `subtype: "error_during_execution"` 的 `result` 消息，并在 `errors` 中给出原因。对于单消息 `query()` 调用，SDK 会在产出该错误结果后抛出异常，因此请将循环包裹在 try 块中以便继续执行。有关错误约定，请参阅[处理结果](https://code.claude.com/docs/zh-CN/agent-sdk/agent-loop.md#handle-the-result)。
>
> 如需改为在沙箱外运行，请设置 `failIfUnavailable: false`。

#### 使用示例

```typescript theme={null}
import { query } from "@anthropic-ai/claude-agent-sdk";

try {
  for await (const message of query({
    prompt: "Build and test my project",
    options: {
      sandbox: {
        enabled: true,
        autoAllowBashIfSandboxed: true,
        network: {
          allowLocalBinding: true
        }
      }
    }
  })) {
    if ("result" in message) console.log(message.result);
  }
} catch (error) {
  // A single-shot query() throws after yielding an error result,
  // such as when the sandbox can't start (failIfUnavailable defaults to true).
  console.log(`Session ended with an error: ${error}`);
}
```

> [!WARNING]
>
> **Unix 套接字安全：** `allowUnixSockets` 选项可能会授予对可触及沙箱外部的系统服务的访问权限。例如，允许 `/var/run/docker.sock` 实际上会通过 Docker API 授予对主机系统的完全访问权限，从而绕过沙箱隔离。请仅允许确实必要的 Unix 套接字，并了解每个套接字的安全影响。

### `SandboxNetworkConfig`

沙箱模式的网络相关配置。当父级 [`SandboxSettings`](#sandboxsettings) 中的 `enabled` 为 `true` 时，这些设置适用于在沙箱中运行的 Bash 命令。它们不会限制 WebFetch 工具，该工具改为使用[权限规则](https://code.claude.com/docs/zh-CN/permissions.md#webfetch)。

```typescript theme={null}
type SandboxNetworkConfig = {
  allowedDomains?: string[];
  deniedDomains?: string[];
  strictAllowlist?: boolean;
  allowManagedDomainsOnly?: boolean;
  allowLocalBinding?: boolean;
  allowUnixSockets?: string[];
  allowAllUnixSockets?: boolean;
  httpProxyPort?: number;
  socksProxyPort?: number;
};
```

| 属性 | 类型 | 默认值 | 描述 |
| :- | :- | :- | :- |
| `allowedDomains` | `string[]` | `[]` | 沙箱内进程可以访问的域名 |
| `deniedDomains` | `string[]` | `[]` | 沙箱内进程无法访问的域名。优先于 `allowedDomains` |
| `strictAllowlist` | `boolean` | `false` | 拒绝沙箱内命令访问[网络允许列表](https://code.claude.com/docs/zh-CN/sandboxing.md#network-isolation)之外的主机，而不是进行提示。仅对沙箱内命令强制执行；WebFetch 等进程内工具不受其限制。仅在用户设置、托管设置或 CLI `--settings` 设置中生效；项目设置中的该项会被忽略。需要 Claude Code v2.1.219 或更高版本 |
| `allowManagedDomainsOnly` | `boolean` | `false` | 仅限托管设置。在[托管设置](https://code.claude.com/docs/zh-CN/managed-settings.md)中设置时，仅采用托管设置中的 `allowedDomains` 条目和 `WebFetch(domain:...)` 允许规则，而用户、项目或本地设置中的允许条目将被忽略。在 SDK 中，请通过 [`managedSettings`](#options) 选项传入 |
| `allowLocalBinding` | `boolean` | `false` | 允许进程绑定到本地端口（例如用于开发服务器） |
| `allowUnixSockets` | `string[]` | `[]` | 进程可以访问的 Unix 套接字路径（例如 Docker 套接字） |
| `allowAllUnixSockets` | `boolean` | `false` | 允许访问所有 Unix 套接字，在 Linux 上还允许访问 VM 套接字（`AF_VSOCK`） |
| `httpProxyPort` | `number` | `undefined` | 用于网络请求的 HTTP 代理端口 |
| `socksProxyPort` | `number` | `undefined` | 用于网络请求的 SOCKS 代理端口 |

> [!NOTE]
>
> 内置沙箱代理根据请求的主机名强制执行 `allowedDomains`，不会终止或检查 TLS 流量，因此[域前置](https://en.wikipedia.org/wiki/Domain_fronting)等技术可能会绕过它。有关详细信息，请参阅[沙箱隔离的安全限制](https://code.claude.com/docs/zh-CN/sandboxing.md#security-limitations)；有关配置 TLS 终止代理，请参阅[安全部署](https://code.claude.com/docs/zh-CN/agent-sdk/secure-deployment.md#traffic-forwarding)。

### `SandboxFilesystemConfig`

沙箱模式的文件系统相关配置。

```typescript theme={null}
type SandboxFilesystemConfig = {
  allowWrite?: string[];
  denyWrite?: string[];
  denyRead?: string[];
};
```

| 属性 | 类型 | 默认值 | 描述 |
| :- | :- | :- | :- |
| `allowWrite` | `string[]` | `[]` | 允许写入访问的文件路径模式 |
| `denyWrite` | `string[]` | `[]` | 拒绝写入访问的文件路径模式。要阻止沙箱内对会话工作目录中某个文件的写入，请列出其绝对路径。 |
| `denyRead` | `string[]` | `[]` | 拒绝读取访问的文件路径模式 |

### 非沙箱命令的权限回退

启用 `allowUnsandboxedCommands` 后，模型可以通过在工具输入中设置 `dangerouslyDisableSandbox: true` 来请求在沙箱外运行命令。这些请求会回退到现有的权限系统，这意味着您的 `canUseTool` 处理程序会被调用，从而允许您实现自定义授权逻辑。

而您的 `excludedCommands` 条目会在无需模型参与的情况下将调用移出沙箱；有关条目何时适用，请参阅 [`sandbox.excludedCommands`](https://code.claude.com/docs/zh-CN/settings-reference#sandbox-excludedcommands)。

在下面的示例中，`isCommandAuthorized` 代表您自行定义的授权检查。

```typescript theme={null}
import { query } from "@anthropic-ai/claude-agent-sdk";

for await (const message of query({
  prompt: "Deploy my application",
  options: {
    sandbox: {
      enabled: true,
      allowUnsandboxedCommands: true // Model can request unsandboxed execution
    },
    permissionMode: "default",
    canUseTool: async (tool, input) => {
      // Check if the model is requesting to bypass the sandbox
      if (tool === "Bash" && input.dangerouslyDisableSandbox) {
        // The model is requesting to run this command outside the sandbox
        console.log(`Unsandboxed command requested: ${input.command}`);

        if (isCommandAuthorized(input.command)) {
          return { behavior: "allow" as const, updatedInput: input };
        }
        return {
          behavior: "deny" as const,
          message: "Command not authorized for unsandboxed execution"
        };
      }
      return { behavior: "allow" as const, updatedInput: input };
    }
  }
})) {
  if ("result" in message) console.log(message.result);
}
```

> [!WARNING]
>
> 以 `dangerouslyDisableSandbox: true` 运行的命令拥有完全的系统访问权限。请确保您的 `canUseTool` 处理程序仔细验证这些请求。
>
> 如果 `permissionMode` 设置为 `bypassPermissions` 且启用了 `allowUnsandboxedCommands`，模型可以在没有批准提示的情况下自主执行沙箱外的命令，[任何模式都不会自动批准的操作](https://code.claude.com/docs/zh-CN/permission-modes.md#actions-no-mode-auto-approves)除外。这种组合实际上允许模型悄无声息地逃逸沙箱隔离。

## 另请参阅

* [SDK 概述](https://code.claude.com/docs/zh-CN/agent-sdk/overview.md) - 常规 SDK 概念
* [Python SDK 参考](https://code.claude.com/docs/zh-CN/agent-sdk/python.md) - Python SDK 文档
* [CLI 参考](https://code.claude.com/docs/zh-CN/cli-reference.md) - 命令行界面
* [常见工作流](https://code.claude.com/docs/zh-CN/common-workflows.md) - 分步指南
