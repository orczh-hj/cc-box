> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Agent SDK 概览

> 使用 Claude Code 作为库构建生产级 AI 代理

代理是一个应用程序，它通过规划自己的步骤并调用读取文件、运行命令或编辑代码的工具来完成任务。Agent SDK 为您提供了与 Claude Code 相同的工具、[代理循环](https://code.claude.com/docs/zh-CN/agent-sdk/agent-loop.md)和上下文管理，可在 Python 和 TypeScript 中编程。

## 将 Agent SDK 与其他 Claude 工具进行比较

Agent SDK、CLI、Client SDK 和 Managed Agents 在谁运行代理、内置功能以及如何访问方面有所不同。找到与您想要构建和运行的方式相匹配的行。

| 您想要 | 使用 | 您获得 |
| - | - | - |
| 在您自己操作的 Python 或 TypeScript 应用程序中嵌入 Claude Code 的代理 | **Agent SDK** | 一个运行 Claude Code 二进制文件的库，具有 Claude Code 的[功能](#capabilities)，例如内置工具、权限、会话和 hooks。 |
| 进行交互式开发或从终端运行一次性任务 | [**Claude Code CLI**](https://code.claude.com/docs/zh-CN/overview.md) | 终端界面，为日常交互使用而构建。 |
| 直接从您自己的代码调用 Claude API | [**Client SDK**](https://platform.claude.com/docs/en/cli-sdks-libraries/overview) | 从任何客户端 SDK 语言直接访问 Claude API。您自己编写工具循环，或让客户端 SDK 的测试版[工具运行器](https://platform.claude.com/docs/en/agents-and-tools/tool-use/tool-runner)驱动它。 |
| 让 Anthropic 托管代理，通过 Claude API 配置 | [**Managed Agents**](https://platform.claude.com/docs/en/managed-agents/overview) | 一个托管代理工具，运行代理循环，会话在 Anthropic 管理的云沙箱或您自己的基础设施上的[自托管沙箱](https://platform.claude.com/docs/en/managed-agents/self-hosted-sandboxes)中。从您语言的 [SDK](https://platform.claude.com/docs/en/managed-agents/quickstart#install-the-sdk)、`ant` CLI 或 REST API 使用它。 |

要从 Python 或 TypeScript 以外的语言驱动相同的代理循环，请[以子进程的形式运行 CLI](https://code.claude.com/docs/zh-CN/headless.md)，使用 `-p` 标志和 `--output-format json`。

## 功能

这些 Claude Code 功能在 SDK 中可用：

| 功能 | 功能说明 | 了解更多 |
| - | - | - |
| 内置工具 | 读取、写入、编辑文件，运行命令，搜索网络 | [工具参考](https://code.claude.com/docs/zh-CN/tools-reference.md) |
| Hooks | 在代理生命周期的关键点运行自定义代码 | [Hooks](https://code.claude.com/docs/zh-CN/agent-sdk/hooks.md) |
| Subagents | 生成专门的代理来处理专注的子任务 | [Subagents](https://code.claude.com/docs/zh-CN/agent-sdk/subagents.md) |
| MCP | 通过 Model Context Protocol 连接外部工具和数据源 | [MCP](https://code.claude.com/docs/zh-CN/agent-sdk/mcp.md) |
| 权限 | 控制哪些工具自动运行，哪些需要批准 | [权限](https://code.claude.com/docs/zh-CN/agent-sdk/permissions.md) |
| 会话 | 在多次交换中保持上下文，稍后恢复或分叉 | [会话](https://code.claude.com/docs/zh-CN/agent-sdk/sessions.md) |
| Skills、命令和内存 | 从您的项目的 `.claude/` 和 `~/.claude/` 自动加载，与 Claude Code 相同 | [Skills](https://code.claude.com/docs/zh-CN/agent-sdk/skills.md)、[命令](https://code.claude.com/docs/zh-CN/agent-sdk/skills.md#commands-in-agent-sdk-sessions)、[内存](https://code.claude.com/docs/zh-CN/agent-sdk/modifying-system-prompts.md)、[配置加载](https://code.claude.com/docs/zh-CN/agent-sdk/claude-code-features.md) |
| Plugins | 打包 skills、代理、hooks 和 MCP 服务器，并按本地路径加载它们 | [Plugins](https://code.claude.com/docs/zh-CN/agent-sdk/plugins.md) |

## 开始使用

按照 [快速入门](https://code.claude.com/docs/zh-CN/agent-sdk/quickstart.md) 安装 SDK、设置您的 API 密钥，并构建您的第一个代理，该代理可以查找并修复现有代码中的错误。

> [!NOTE]
>
> 除非事先获得批准，否则 Anthropic 不允许第三方开发者为其产品（包括基于 Claude Agent SDK 构建的代理）提供 claude.ai 登录或速率限制。请改用 [快速入门](https://code.claude.com/docs/zh-CN/agent-sdk/quickstart.md) 中描述的 API 密钥身份验证方法。

## 更新日志

查看完整的更新日志以了解 SDK 更新、bug 修复和新功能：

* **TypeScript SDK**：[查看 CHANGELOG.md](https://github.com/anthropics/claude-agent-sdk-typescript/blob/main/CHANGELOG.md)
* **Python SDK**：[查看 CHANGELOG.md](https://github.com/anthropics/claude-agent-sdk-python/blob/main/CHANGELOG.md)

## 报告 bug

如果您在 Agent SDK 中遇到 bug 或问题：

* **TypeScript SDK**：[在 GitHub 上报告问题](https://github.com/anthropics/claude-agent-sdk-typescript/issues)
* **Python SDK**：[在 GitHub 上报告问题](https://github.com/anthropics/claude-agent-sdk-python/issues)

## 品牌指南

对于集成 Claude Agent SDK 的合作伙伴，使用 Claude 品牌是可选的。在您的产品中引用 Claude 时：

**允许：**

* "Claude Agent"，首选用于下拉菜单
* "Claude"，当已在标记为"Agents"的菜单中时
* "\{YourAgentName} Powered by Claude"，如果您有现有的代理名称

**不允许：**

* "Claude Code" 或 "Claude Code Agent"
* Claude Code 品牌的 ASCII 艺术或模仿 Claude Code 的视觉元素

您的产品应保持自己的品牌，不应显示为 Claude Code 或任何 Anthropic 产品。如有关于品牌合规性的问题，请联系 Anthropic [销售团队](https://www.anthropic.com/contact-sales)。

## 许可证和条款

Claude Agent SDK 的使用受 [Anthropic 商业服务条款](https://www.anthropic.com/legal/commercial-terms)管制，包括当您使用它为您自己的客户和最终用户提供的产品和服务时，除非特定组件或依赖项由该组件的 LICENSE 文件中指示的不同许可证覆盖。

## 后续步骤

这些资源涵盖了使用 Agent SDK 构建的更深层次的技术细节和示例项目。

* [快速入门](https://code.claude.com/docs/zh-CN/agent-sdk/quickstart.md)：构建你的第一个查找和修复 bug 的代理
* [迁移指南](https://code.claude.com/docs/zh-CN/agent-sdk/migration-guide.md)：从 Claude Code SDK 包迁移到 Agent SDK
* [Agent 循环](https://code.claude.com/docs/zh-CN/agent-sdk/agent-loop.md)：Claude 如何规划、调用工具以及决定任务何时完成
* [示例代理](https://github.com/anthropics/claude-agent-sdk-demos)：用于本地开发的演示应用
* [TypeScript SDK](https://code.claude.com/docs/zh-CN/agent-sdk/typescript.md)：完整的 TypeScript API 参考和示例
* [Python SDK](https://code.claude.com/docs/zh-CN/agent-sdk/python.md)：完整的 Python API 参考和示例
* [Agent 工具设计](https://claude.com/blog/a-harness-for-every-task-dynamic-workflows-in-claude-code)：Claude Code 团队如何使用动态工作流来同时编排许多子代理
