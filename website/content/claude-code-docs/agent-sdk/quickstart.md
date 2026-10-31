> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# 快速开始

> 使用 Python 或 TypeScript Agent SDK 开始构建能够自主工作的 AI 代理

使用 Agent SDK 构建一个 AI 代理，它可以读取你的代码、发现错误并修复它们，所有这一切都无需手动干预。

**你将做什么：**

1. 使用 Agent SDK 设置一个项目
2. 创建一个包含一些有缺陷代码的文件
3. 运行一个代理，自动查找并修复错误

## 前置条件

* **Node.js 18.14+** 或 **Python 3.10+**
* 一个 **Anthropic 账户**。如果您还没有账户，[在此注册](https://platform.claude.com/)。

## 设置

**1. 创建项目文件夹**

为此快速开始创建一个新目录：

```bash theme={null}
mkdir my-agent
cd my-agent
```

对于你自己的项目，你可以从任何文件夹运行 SDK；默认情况下，它将有权访问该目录及其子目录中的文件。

**2. 安装 SDK**

为你的语言安装 Agent SDK 包：

> [tab] TypeScript（新项目）
>
> ```bash theme={null}
> npm init -y
> npm pkg set type=module
> npm install @anthropic-ai/claude-agent-sdk
> npm install --save-dev tsx
> ```
>
> 在 `package.json` 中设置 `"type": "module"` 让你的代理脚本使用顶级 `await`，而 [tsx](https://tsx.hirok.io) 直接运行 TypeScript 文件。npm 在安装成功时会打印 `added N packages`。

> [tab] TypeScript（现有项目）
>
> ```bash theme={null}
> npm install @anthropic-ai/claude-agent-sdk
> npm install --save-dev tsx
> ```
>
> [tsx](https://tsx.hirok.io) 直接运行 TypeScript 文件。如果你的项目使用 CommonJS，将你的代理脚本命名为 `agent.mts` 而不是 `agent.ts`。`.mts` 扩展名使 tsx 将文件视为 ES 模块，所以顶级 `await` 可以工作，而无需将整个项目转换为 ES 模块。在本快速开始中后面的创建和运行步骤中使用 `agent.mts` 代替 `agent.ts`。

> [tab] Python（uv）
>
> [安装 uv](https://docs.astral.sh/uv/)，一个快速的 Python 包管理器，可以自动处理虚拟环境。然后初始化一个项目并添加 SDK：
>
> ```bash theme={null}
> uv init
> uv add claude-agent-sdk
> ```

> [tab] Python（pip）
>
> 创建并激活虚拟环境，然后安装该包。
>
> 在 macOS 或 Linux 上：
>
> ```bash theme={null}
> python3 -m venv .venv
> source .venv/bin/activate
> pip install claude-agent-sdk
> ```
>
> 在 Windows 上：
>
> ```powershell theme={null}
> py -m venv .venv
> .venv\Scripts\Activate.ps1
> pip install claude-agent-sdk
> ```
>
> 如果 PowerShell 因执行策略错误而阻止 `Activate.ps1`，请先运行 `Set-ExecutionPolicy -Scope Process RemoteSigned`。

> [!NOTE]
>
> TypeScript 和 Python SDK 都捆绑了一个本地 Claude Code 二进制文件，所以大多数安装不需要单独安装 Claude Code。某些安装没有捆绑的二进制文件：
>
> * 如果 pip 安装了 Python SDK 的源代码分发而不是平台 wheel，例如在 ARM64 Windows 上，则不会捆绑任何二进制文件。[本地安装 Claude Code](https://code.claude.com/docs/zh-CN/setup.md#install-claude-code)。Python SDK 在你的 `PATH` 上找到它。
> * TypeScript SDK 通过 npm 可选依赖项安装其二进制文件，所以跳过它们的安装，例如 `npm ci --omit=optional`，即使在支持的平台上也不会获得二进制文件。重新安装而不跳过可选依赖项，或 [本地安装 Claude Code](https://code.claude.com/docs/zh-CN/setup.md#install-claude-code) 并将 `pathToClaudeCodeExecutable` 设置为其路径。

**3. 设置你的 API 密钥**

从 [Claude 控制台](https://platform.claude.com/)获取 API 密钥，然后在你将运行代理的 shell 中将其设置为环境变量：

> [tab] macOS / Linux
>
> ```bash theme={null}
> export ANTHROPIC_API_KEY=your-api-key
> ```

> [tab] Windows（PowerShell）
>
> ```powershell theme={null}
> $env:ANTHROPIC_API_KEY = "your-api-key"
> ```

SDK 从运行你的代理的进程的环境中读取密钥；它不会自动加载 `.env` 文件。如果你将密钥保存在 `.env` 文件中，请自己加载它，例如使用 `dotenv` 包，然后再调用 SDK。

SDK 还支持通过第三方 API 提供商进行身份验证：

* **Amazon Bedrock**：设置 `CLAUDE_CODE_USE_BEDROCK=1` 环境变量并配置 AWS 凭证
* **Claude Platform on AWS**：设置 `CLAUDE_CODE_USE_ANTHROPIC_AWS=1` 和 `ANTHROPIC_AWS_WORKSPACE_ID`，然后配置 AWS 凭证
* **Google Cloud 的 Agent Platform**：设置 `CLAUDE_CODE_USE_VERTEX=1` 环境变量并配置 Google Cloud 凭证
* **Microsoft Foundry**：设置 `CLAUDE_CODE_USE_FOUNDRY=1` 环境变量并配置 Azure 凭证

有关详细信息，请参阅 [Amazon Bedrock](https://code.claude.com/docs/zh-CN/amazon-bedrock.md)、[Claude Platform on AWS](https://code.claude.com/docs/zh-CN/claude-platform-on-aws.md)、[Google Cloud 的 Agent Platform](https://code.claude.com/docs/zh-CN/google-vertex-ai.md) 或 [Microsoft Foundry](https://code.claude.com/docs/zh-CN/microsoft-foundry.md) 的设置指南。

> [!NOTE]
>
> 除非事先获得批准，否则 Anthropic 不允许第三方开发者为其产品（包括基于 Claude Agent SDK 构建的 Agent）提供 claude.ai 登录或速率限制。请改用本文档中描述的 API 密钥身份验证方法。

## 创建一个有缺陷的文件

此快速开始将引导你构建一个可以查找和修复代码中错误的代理。首先，你需要一个包含一些有意错误的文件供代理修复。在 `my-agent` 目录中创建 `utils.py` 并粘贴以下代码：

```python theme={null}
def calculate_average(numbers):
    total = 0
    for num in numbers:
        total += num
    return total / len(numbers)


def get_user_name(user):
    return user["name"].upper()
```

此代码有两个错误：

1. `calculate_average([])` 会因除以零而崩溃
2. `get_user_name(None)` 会因 TypeError 而崩溃

## 构建一个查找和修复错误的代理

如果你使用 Python SDK，创建 `agent.py`，或者如果使用 TypeScript，创建 `agent.ts`。如果你现有的项目使用 CommonJS，请改用 `agent.mts`：

```python Python theme={null}
import asyncio
from claude_agent_sdk import query, ClaudeAgentOptions, AssistantMessage, ResultMessage


async def main():
    # Agentic loop: streams messages as Claude works
    async for message in query(
        prompt="Review utils.py for bugs that would cause crashes. Fix any issues you find.",
        options=ClaudeAgentOptions(
            allowed_tools=["Read", "Edit", "Glob"],  # 自动批准这些工具
            permission_mode="acceptEdits",  # 自动批准文件编辑
        ),
    ):
        # 打印人类可读的输出
        if isinstance(message, AssistantMessage):
            for block in message.content:
                if hasattr(block, "text"):
                    print(block.text)  # Claude 的推理
                elif hasattr(block, "name"):
                    print(f"Tool: {block.name}")  # 正在调用的工具
        elif isinstance(message, ResultMessage):
            print(f"Done: {message.subtype}")  # 最终结果


asyncio.run(main())
```

```typescript TypeScript theme={null}
import { query } from "@anthropic-ai/claude-agent-sdk";

// Agentic loop: streams messages as Claude works
for await (const message of query({
  prompt: "Review utils.py for bugs that would cause crashes. Fix any issues you find.",
  options: {
    allowedTools: ["Read", "Edit", "Glob"], // 自动批准这些工具
    permissionMode: "acceptEdits" // 自动批准文件编辑
  }
})) {
  // 打印人类可读的输出
  if (message.type === "assistant" && message.message?.content) {
    for (const block of message.message.content) {
      if ("text" in block) {
        console.log(block.text); // Claude 的推理
      } else if ("name" in block) {
        console.log(`Tool: ${block.name}`); // 正在调用的工具
      }
    }
  } else if (message.type === "result") {
    console.log(`Done: ${message.subtype}`); // 最终结果
  }
}
```

此代码有三个主要部分：

1. **`query`**：创建 agentic 循环的主入口点。它返回一个异步迭代器，所以你使用 `async for` 来流式传输 Claude 工作时的消息。查看 [Python](https://code.claude.com/docs/zh-CN/agent-sdk/python.md#query) 或 [TypeScript](https://code.claude.com/docs/zh-CN/agent-sdk/typescript.md#query) SDK 参考中的完整 API。

2. **`prompt`**：你想让 Claude 做什么。Claude 根据任务确定要使用哪些工具。

3. **`options`**：代理的配置。此示例使用 `allowedTools` 预先批准 `Read`、`Edit` 和 `Glob`，以及 `permissionMode: "acceptEdits"` 来自动批准文件更改。其他选项包括 `systemPrompt`、`mcpServers` 等。查看 [Python](https://code.claude.com/docs/zh-CN/agent-sdk/python.md#claudeagentoptions) 或 [TypeScript](https://code.claude.com/docs/zh-CN/agent-sdk/typescript.md#options) 的所有选项。

`async for` 循环在 Claude 思考、调用工具、观察结果并决定下一步做什么时继续运行。每次迭代都会产生一条消息：Claude 的推理、工具调用、工具结果或最终结果。SDK 处理编排、工具执行、上下文管理和重试，所以你只需使用流。当 Claude 完成任务或遇到错误时，循环结束。

循环内的消息处理过滤人类可读的输出。如果没有过滤，你会看到原始消息对象，包括系统初始化和内部状态，这对调试很有用，但通常很冗长。

> [!NOTE]
>
> 此示例使用流式传输来实时显示进度。如果你不需要实时输出（例如，对于后台作业或 CI 管道），你可以一次性收集所有消息。有关详细信息，请参阅[流式传输与单轮模式](https://code.claude.com/docs/zh-CN/agent-sdk/streaming-vs-single-mode.md)。

### 运行你的代理

你的代理已准备好。使用以下命令运行它：

> [tab] TypeScript
>
> ```bash theme={null}
> npx tsx agent.ts
> ```
>
> 如果你将脚本命名为 `agent.mts`，请改为运行 `npx tsx agent.mts`。

> [tab] Python（uv）
>
> ```bash theme={null}
> uv run agent.py
> ```

> [tab] Python（pip）
>
> 使用你仍然激活的虚拟环境：
>
> ```bash theme={null}
> python agent.py
> ```

运行时，代理会打印其推理和它调用的每个工具，最后以 `Done: success` 结束。运行后，检查 `utils.py`。你会看到处理空列表和空用户的防御性代码。你的代理自主地：

1. **读取** `utils.py` 以理解代码
2. **分析**了逻辑并识别了会导致崩溃的边界情况
3. **编辑**了文件以添加适当的错误处理

这就是 Agent SDK 的与众不同之处：Claude 直接执行工具，而不是要求你实现它们。

> [!NOTE]
>
> 如果你看到身份验证错误，例如 `Not logged in` 或 `Invalid API key`，请确保你已在运行代理的 shell 中设置了 `ANTHROPIC_API_KEY` 环境变量。SDK 不会自动加载 `.env` 文件。
>
> 有关这些和其他身份验证错误的原因和修复，请参阅错误参考中的[身份验证错误](https://code.claude.com/docs/zh-CN/errors.md#authentication-errors)。

### 尝试其他提示

现在你的代理已设置好，尝试一些不同的提示：

* `"Add docstrings to all functions in utils.py"`
* `"Add type hints to all functions in utils.py"`
* `"Create a README.md documenting the functions in utils.py"`

### 自定义你的代理

你可以通过更改选项来修改代理的行为。以下是一些示例：

**添加网络搜索功能：**

```python Python theme={null}
options = ClaudeAgentOptions(
    allowed_tools=["Read", "Edit", "Glob", "WebSearch"], permission_mode="acceptEdits"
)
```

```typescript TypeScript hidelines={1,-1} theme={null}
const _ = {
  options: {
    allowedTools: ["Read", "Edit", "Glob", "WebSearch"],
    permissionMode: "acceptEdits"
  }
};
```

**给 Claude 一个自定义系统提示：**

```python Python theme={null}
options = ClaudeAgentOptions(
    allowed_tools=["Read", "Edit", "Glob"],
    permission_mode="acceptEdits",
    system_prompt="You are a senior Python developer. Always follow PEP 8 style guidelines.",
)
```

```typescript TypeScript hidelines={1,-1} theme={null}
const _ = {
  options: {
    allowedTools: ["Read", "Edit", "Glob"],
    permissionMode: "acceptEdits",
    systemPrompt: "You are a senior Python developer. Always follow PEP 8 style guidelines."
  }
};
```

**在终端中运行命令：**

```python Python theme={null}
options = ClaudeAgentOptions(
    allowed_tools=["Read", "Edit", "Glob", "Bash"], permission_mode="acceptEdits"
)
```

```typescript TypeScript hidelines={1,-1} theme={null}
const _ = {
  options: {
    allowedTools: ["Read", "Edit", "Glob", "Bash"],
    permissionMode: "acceptEdits"
  }
};
```

启用 `Bash` 后，尝试：`"Write unit tests for utils.py, run them, and fix any failures"`

这些代码片段中的每一个都在同一个选项对象上设置字段。有关更多信息，请参阅[配置你的代理](https://code.claude.com/docs/zh-CN/agent-sdk/configuration.md)。

## 关键概念

**工具**控制你的代理可以做什么：

| 工具 | 代理可以做什么 |
| - | - |
| `Read`、`Glob`、`Grep` | 只读分析 |
| `Read`、`Edit`、`Glob` | 分析和修改代码 |
| `Read`、`Edit`、`Bash`、`Glob`、`Grep` | 完全自动化 |

**权限模式**控制你想要多少人工监督。SDK 按照固定的顺序评估活跃模式以及你的允许和拒绝规则，详见[权限如何被评估](https://code.claude.com/docs/zh-CN/agent-sdk/permissions.md#how-permissions-are-evaluated)。有关完整的模式列表、它们的行为以及何时使用每种模式，请参阅[代理循环工作原理中的权限模式](https://code.claude.com/docs/zh-CN/agent-sdk/agent-loop.md#permission-mode)。

## 后续步骤

现在你已经创建了你的第一个代理，学习如何扩展其功能并将其定制到你的用例：

* **[配置你的代理](https://code.claude.com/docs/zh-CN/agent-sdk/configuration.md)**：组合选项对象并找到涵盖每个设置的页面
* **[权限](https://code.claude.com/docs/zh-CN/agent-sdk/permissions.md)**：控制你的代理可以做什么以及何时需要批准
* **[Hooks](https://code.claude.com/docs/zh-CN/agent-sdk/hooks.md)**：在工具调用之前或之后运行自定义代码
* **[会话](https://code.claude.com/docs/zh-CN/agent-sdk/sessions.md)**：构建维护上下文的多轮代理
* **[MCP 服务器](https://code.claude.com/docs/zh-CN/agent-sdk/mcp.md)**：连接到数据库、浏览器、API 和其他外部系统
* **[托管](https://code.claude.com/docs/zh-CN/agent-sdk/hosting.md)**：将代理部署到 Docker、云和 CI/CD
* **[示例代理](https://github.com/anthropics/claude-agent-sdk-demos)**：查看完整示例：电子邮件助手、研究代理等
* **[故障排除](https://code.claude.com/docs/zh-CN/agent-sdk/troubleshooting.md)**：修复 CLI 无法启动或退出时的错误，或结果到达时没有结构化输出
