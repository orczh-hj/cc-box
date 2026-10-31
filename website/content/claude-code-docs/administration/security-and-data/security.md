> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# 安全性

> 了解 Claude Code 的安全防护措施和安全使用的最佳实践。

## 我们如何处理安全性

### 安全基础

您的代码安全至关重要。Claude Code 以安全为核心构建，按照 Anthropic 的全面安全计划开发。在 [Anthropic Trust Center](https://trust.anthropic.com) 了解更多信息并访问资源（SOC 2 Type 2 报告、ISO 27001 证书等）。

### 基于权限的架构

会话的权限模式决定了 Claude 无需先询问您即可执行哪些操作。自动模式是交互式终端和 VS Code 会话的内置初始权限模式。[会话以哪种模式启动](https://code.claude.com/docs/zh-CN/permission-modes.md#which-mode-a-session-starts-in)介绍了早期版本、其他使用入口以及可更改初始权限模式的设置。

* **自动模式**：一个单独的分类器模型代替您审查操作，并阻止它认为不安全的操作。[分类器如何评估操作](https://code.claude.com/docs/zh-CN/permission-modes.md#how-the-classifier-evaluates-actions)列出了 Claude Code 直接批准的操作、发送给分类器的操作以及 Claude Code 仍然询问您的操作。您显式设置的 ask 规则和 deny 规则仍然适用，您的组织可以[关闭自动模式](https://code.claude.com/docs/zh-CN/permission-modes.md#eliminate-prompts-with-auto-mode)
* **Manual mode**：Claude Code 以只读权限开始。当它需要编辑文件、运行测试或执行命令时，它会先询问您，您可以选择批准该操作一次或从此允许该操作。它无需询问即可运行一组内置的[只读命令](https://code.claude.com/docs/zh-CN/permissions.md#read-only-commands)，如 `ls`、`cat` 和 `git status`

您和您的组织可以直接配置这些权限。有关详细的权限配置，请参阅 [Permissions](https://code.claude.com/docs/zh-CN/permissions.md)。

### 内置保护

为了降低代理系统中的风险：

* **沙箱化 bash 工具**：使用文件系统和网络隔离的 [Sandbox](https://code.claude.com/docs/zh-CN/sandboxing.md) bash 命令，减少权限提示同时保持安全性。使用 `/sandbox` 配置以定义 Claude Code 可以自主工作的边界
* **工作目录边界**：在 Manual mode 中，Claude Code 的文件工具在读取或写入启动它的文件夹及其子文件夹之外的内容之前会先询问您。该边界是一个权限提示，因此您批准的 Bash 命令仍然可以写入您的用户账户有权写入的任何位置
  * 要在没有提示的情况下读取某个文件夹，请将其添加为[额外目录](https://code.claude.com/docs/zh-CN/permissions.md#working-directories)
  * 要在操作系统层面限制 Bash 命令，请启用[沙箱隔离](https://code.claude.com/docs/zh-CN/sandboxing.md#filesystem-isolation)
* **提示疲劳缓解**：支持按用户、按代码库或按组织的白名单常用安全命令
* **Accept Edits 模式**：自动批准文件编辑和一组固定的文件系统 Bash 命令，如 `mkdir`、`touch`、`rm`、`mv`、`cp` 和 `sed`，用于工作目录中的路径。其他 Bash 命令和超出范围的路径仍然会提示

### 用户责任

您负责在批准前审查建议的代码和命令的安全性。

## 防止提示注入

提示注入是一种攻击者试图通过插入恶意文本来覆盖或操纵 AI 助手指令的技术。Claude Code 包括针对这些攻击的多项防护措施：

### 核心保护

* **权限系统**：在手动模式下，敏感操作需要明确批准
* **网络命令批准**：从网络获取内容的命令，如 `curl` 和 `wget`，默认不会自动批准。在手动模式下，它们会像任何其他非只读 Bash 命令一样提示，因此您仍然可以批准一次或添加显式允许规则，如 `Bash(curl *)`。要阻止 Claude 运行它们，请将其添加到 [`permissions.deny`](https://code.claude.com/docs/zh-CN/permissions#tool-specific-permission-rules)。拒绝规则匹配[按照编写的命令](https://code.claude.com/docs/zh-CN/permissions.md#bash-rule-limits)；对于不依赖命令文本的网络强制执行，请参阅[沙箱网络隔离](https://code.claude.com/docs/zh-CN/sandboxing.md#network-isolation)

### 隐私保护

我们实施了多项保护措施来保护您的数据，包括：

* 敏感信息的有限保留期（请参阅 [Privacy Center](https://privacy.anthropic.com/en/articles/10023548-how-long-do-you-store-my-data) 了解更多）
* 对用户会话数据的受限访问
* 用户对数据训练偏好的控制。消费者用户可以随时更改其 [隐私设置](https://claude.ai/settings/privacy)。

有关完整详情，请查看我们的 [Commercial Terms of Service](https://www.anthropic.com/legal/commercial-terms)（适用于 Team、Enterprise 和 API 用户）或 [Consumer Terms](https://www.anthropic.com/legal/consumer-terms)（适用于 Free、Pro、Max 和 Ultra 用户）以及 [Privacy Policy](https://www.anthropic.com/legal/privacy)。

### 其他保护措施

* **网络请求批准**：在手动模式下，进行网络请求的大多数工具默认需要用户批准
* **网页摘要**：对于大多数获取操作，WebFetch 会针对页面运行一次单独的模型调用，Claude 接收的是该调用的回答，而不是原始页面。请参阅 [WebFetch 工具行为](https://code.claude.com/docs/zh-CN/tools-reference.md#webfetch-tool-behavior)
* **信任验证**：在交互式会话中，当您在尚未信任的文件夹中启动 Claude Code 时，它会显示工作区信任对话框。项目 `.mcp.json` 中的服务器有其单独的批准提示，[项目作用域](https://code.claude.com/docs/zh-CN/mcp.md#project-scope) 列出了跳过该提示的会话
  * 注意：`-p` 会话不会显示上述任何一种提示。[信任文件夹之前会运行什么](https://code.claude.com/docs/zh-CN/permissions.md#what-runs-before-you-trust-a-folder) 列出了仓库中的文件在此情况下可以运行的内容
  * 注意：当您直接在主目录中启动 Claude Code 时，信任接受仅在当前会话期间保持，不会写入磁盘，因此提示在每次启动时都会重新出现。没有设置可以持久化它。请从项目子目录启动 Claude Code，其中信任接受按目录保存
* **命令注入检测**：在手动模式下，Claude Code 在运行无法完全分析的 Bash 命令之前会先询问。针对命令一部分的允许规则（如 `Bash(git *)`）不会跳过该提示。[沙箱隔离的命令](https://code.claude.com/docs/zh-CN/permissions.md#how-permissions-interact-with-sandboxing)可以在没有该提示的情况下运行
* **故障关闭匹配**：在手动模式下，不匹配的命令默认需要批准
* **安全凭据存储**：API 密钥和令牌在可用时存储在 macOS Keychain 中。在 Linux 上，它们存储在模式为 `0600` 的文件中；在 Windows 上，它们存储在继承您用户配置文件目录访问控制的文件中。请参阅 [Credential Management](https://code.claude.com/docs/zh-CN/authentication.md#credential-management)

> [!WARNING]
>
> **Windows WebDAV 安全风险**：在 Windows 上运行 Claude Code 时，我们建议不要启用 WebDAV 或允许 Claude Code 访问可能包含 WebDAV 子目录的路径，如 `\\*`。[WebDAV 已被 Microsoft 弃用](https://learn.microsoft.com/en-us/windows/whats-new/deprecated-features#:~:text=The%20Webclient%20\(WebDAV\)%20service%20is%20deprecated)，原因是安全风险。启用 WebDAV 可能允许 Claude Code 触发对远程主机的网络请求，绕过权限系统。

**处理不受信任内容的最佳实践**：

1. 在批准前审查建议的命令
2. 避免直接将不受信任的内容通过管道传递给 Claude
3. 验证对关键文件的建议更改
4. 使用虚拟机 (VM) 运行脚本并进行工具调用，特别是在与外部 Web 服务交互时
5. 使用 `/feedback` 报告可疑行为

> [!WARNING]
>
> 虽然这些保护措施大大降低了风险，但没有系统完全免疫所有攻击。在使用任何 AI 工具时，始终保持良好的安全实践。

## MCP 安全性

您可以将 Claude Code 连接到 Model Context Protocol (MCP) 服务器。项目作用域的服务器在 `.mcp.json` 中定义，您可以将该文件检入源代码控制。[其他作用域](https://code.claude.com/docs/zh-CN/mcp.md#mcp-installation-scopes)的服务器和 [claude.ai 连接器](https://code.claude.com/docs/zh-CN/mcp.md#how-connectors-reach-claude-code)是在仓库之外配置的，插件也可以添加服务器，因此查看 `.mcp.json` 并不能看到会话可能加载的所有服务器。要限制您的组织中可以运行哪些服务器，请参阅[托管 MCP 配置](https://code.claude.com/docs/zh-CN/managed-mcp.md)。

我们鼓励编写您自己的 MCP servers 或使用来自您信任的提供商的 MCP servers。您能够为 MCP servers 配置 Claude Code 权限。Anthropic 在将连接器添加到 [Anthropic Directory](https://claude.ai/directory) 之前，会根据其 [列表标准](https://claude.com/docs/connectors/building/review-criteria) 审查连接器，但不对任何 MCP server 进行安全审计或管理。

## IDE 安全性

有关在 IDE 中运行 Claude Code 的更多信息，请参阅[内置 IDE MCP 服务器](https://code.claude.com/docs/zh-CN/vs-code.md#the-built-in-ide-mcp-server)。

## 云执行安全性

使用 [cloud sessions](https://code.claude.com/docs/zh-CN/claude-code-on-the-web.md) 时，会实施额外的安全控制。您的组织路由到 [自托管环境](https://code.claude.com/docs/zh-CN/self-hosted-environments) 的会话在您自己的基础设施上运行，其中隔离、网络出口和 git 凭证是您部署的责任。在 Anthropic 托管的环境中：

* **隔离的虚拟机**：每个云会话在隔离的、由 Anthropic 管理的 VM 中运行
* **网络访问控制**：网络访问默认受限，可以配置为禁用或仅允许特定域
* **凭证保护**：GitHub 凭证在 Anthropic 的服务器上以加密方式存储，永远不会进入会话 VM。VM 持有一个作用域限制于该会话的短期凭证，GitHub 流量通过 [Anthropic proxy](https://code.claude.com/docs/zh-CN/cloud-environments.md#github-proxy) 进行，该代理在服务器端附加 GitHub 凭证。有关如何授予访问权限，请参阅 [GitHub authentication options](https://code.claude.com/docs/zh-CN/claude-code-on-the-web.md#github-authentication-options)
* **推送限制**：[GitHub 代理](https://code.claude.com/docs/zh-CN/cloud-environments.md#github-proxy)会拒绝删除分支，以及推送分支以外的任何内容（例如标签）。GitHub 通过将您仓库的分支保护规则和规则集应用于您所连接的 GitHub 访问权限，来决定会话可以更新哪些分支。该访问权限可以绕过的规则不会阻止会话的推送
* **审计日志**：云会话中的所有操作都被记录以用于合规和审计目的
* **自动清理**：会话 VM 在一段时间不活动后被回收
* **删除**：您可以随时 [delete a session](https://code.claude.com/docs/zh-CN/claude-code-on-the-web.md#delete-sessions)。有关 Anthropic 为云会话存储的内容，请参阅 [Cloud execution data flow](https://code.claude.com/docs/zh-CN/data-usage.md#cloud-execution-data-flow-and-dependencies)

有关云执行的更多详情，请参阅 [Use Claude Code in the cloud](https://code.claude.com/docs/zh-CN/claude-code-on-the-web.md)；要为云会话配置网络访问，请参阅 [Configure cloud environments](https://code.claude.com/docs/zh-CN/cloud-environments.md#network-access)。

[Remote Control](https://code.claude.com/docs/zh-CN/remote-control.md) 会话的工作方式不同：Web 界面连接到在您本地机器上运行的 Claude Code 进程。所有代码执行和文件访问都保持本地，会话流量通过 TLS 上的 Anthropic API 传输；连接时，会话记录存储在 Anthropic 服务器上以跨设备同步对话，如 [Connection and security](https://code.claude.com/docs/zh-CN/remote-control.md#connection-and-security) 中所述。不涉及云 VM 或沙箱。连接使用多个短期的、范围狭窄的凭证，每个凭证限制于特定目的并独立过期，以限制任何单个受损凭证的影响范围。

## 安全最佳实践

### 处理敏感代码

* 在批准前审查所有建议的更改
* 为敏感存储库使用项目特定的权限设置
* 如需额外隔离，请在 [沙箱运行时](https://code.claude.com/docs/zh-CN/sandbox-environments.md#sandbox-runtime) 或 [dev container](https://code.claude.com/docs/zh-CN/devcontainer.md) 中运行整个 Claude Code（本地模式）进程
* 使用 `/permissions` 定期审计您的权限设置

### 团队安全

* 使用 [managed settings](https://code.claude.com/docs/zh-CN/settings.md#where-settings-live) 来强制执行组织标准
* 通过版本控制共享批准的权限配置
* 培训团队成员了解安全最佳实践
* 通过 [OpenTelemetry metrics](https://code.claude.com/docs/zh-CN/monitoring-usage.md) 监控 Claude Code 使用情况
* 使用 [`ConfigChange` hooks](https://code.claude.com/docs/zh-CN/hooks#configchange) 审计或阻止会话期间的设置更改

### 报告安全问题

如果您在 Claude Code 中发现安全漏洞：

1. 不要公开披露
2. 通过我们的 [HackerOne program](https://hackerone.com/4f1f16ba-10d3-4d09-9ecc-c721aad90f24/embedded_submissions/new) 报告
3. 包括详细的复现步骤
4. 在公开披露前给我们时间来解决问题

## 相关资源

* [Security guidance plugin](https://code.claude.com/docs/zh-CN/security-guidance.md)：让 Claude 在会话期间审查和修复其自身代码更改中的漏洞
* [`/security-review`](https://code.claude.com/docs/zh-CN/commands#all-commands)：对当前分支上的更改运行按需安全检查
* [Sandbox environments](https://code.claude.com/docs/zh-CN/sandbox-environments.md)：比较隔离方法并为您的威胁模型选择一个
* [Sandboxing](https://code.claude.com/docs/zh-CN/sandboxing.md)：Bash 命令的文件系统和网络隔离
* [Permissions](https://code.claude.com/docs/zh-CN/permissions.md)：配置权限和访问控制
* [Monitoring usage](https://code.claude.com/docs/zh-CN/monitoring-usage.md)：跟踪和审计 Claude Code 活动
* [Development containers](https://code.claude.com/docs/zh-CN/devcontainer.md)：安全、隔离的环境
* [Anthropic Trust Center](https://trust.anthropic.com)：安全认证和合规
* [CISO's guide to agentic AI](https://claude.com/blog/ciso-guide-to-agentic-ai)：安全领导者评估代理 AI 部署的框架
