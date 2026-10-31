> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# 快速入门

> 在终端中安装 Claude Code，完成登录，并使用 CLI 探索您的代码库、进行第一次代码更改。

本快速入门介绍终端中的 Claude Code：安装 CLI、在第一个会话中登录，以及在您自己的项目中使用它完成常见的开发任务。

> [!NOTE]
>
> 默认配置下，Claude Code 需要能够访问 claude.ai 和 Anthropic API 等端点才能完成安装、登录和正常使用。在中国大陆的网络环境中，这些端点可能无法直接访问。开始前，请先确认所在网络能够连通这些服务。企业代理配置以及 Amazon Bedrock 等第三方提供商的网络要求，请参阅[网络配置](https://code.claude.com/docs/zh-CN/network-config.md#network-access-requirements)。

## 开始前

确保您拥有：

* 打开的终端或命令提示符
* 一个可以使用的代码项目
* 一个 [Claude 订阅](https://claude.com/pricing?utm_source=claude_code\&utm_medium=docs\&utm_content=quickstart_prereq)（Pro、Max、Ultra、Team 或 Enterprise）、[Claude Console](https://platform.claude.com/) 账户，或通过[支持的云提供商](https://code.claude.com/docs/zh-CN/third-party-integrations.md)的访问权限

> [!NOTE]
>
> 以下情况在其他页面中介绍：
>
> * **从未使用过终端**：请从[终端指南](https://code.claude.com/docs/zh-CN/terminal-guide)开始
> * **希望在终端以外的地方使用 Claude Code**：Claude Code 也可在[网页](https://claude.ai/code)、[桌面应用](https://code.claude.com/docs/zh-CN/desktop.md)、[VS Code](https://code.claude.com/docs/zh-CN/vs-code.md) 和 [JetBrains IDE](https://code.claude.com/docs/zh-CN/jetbrains.md)、[Slack](https://code.claude.com/docs/zh-CN/slack.md) 中使用，以及通过 [GitHub Actions](https://code.claude.com/docs/zh-CN/github-actions.md) 和 [GitLab](https://code.claude.com/docs/zh-CN/gitlab-ci-cd.md) 在 CI/CD 中使用。查看[所有界面](https://code.claude.com/docs/zh-CN/overview.md#use-claude-code-everywhere)。

## 步骤 1：安装 Claude Code

要安装 Claude Code，请打开终端并运行适用于您的系统的命令。如果您之前没有使用过终端，[终端指南](https://code.claude.com/docs/zh-CN/terminal-guide)会展示如何打开终端并粘贴命令。

> [tab] 原生安装（推荐）
>
> **macOS、Linux、WSL：**
>
> ```bash theme={null}
> curl -fsSL https://claude.ai/install.sh | bash
> ```
>
> 在 Windows 上，当您在 PowerShell 中时，您的提示符显示 `PS C:\`；当您在 CMD 中时，提示符显示 `C:\`（没有 `PS`）。
>
> **Windows PowerShell：**
>
> ```powershell theme={null}
> irm https://claude.ai/install.ps1 | iex
> ```
>
> **Windows CMD：**
>
> ```batch theme={null}
> curl -fsSL https://claude.ai/install.cmd -o install.cmd && install.cmd && del install.cmd
> ```
>
> 安装命令在下载 Claude Code 期间不会显示进度。安装程序完成后，打开一个新的终端窗口并运行 `claude --version`。正常的安装会打印一个版本号。如果您的 shell 显示找不到 `claude` 或无法识别，说明安装目录还不在您的 PATH 中：请参阅[修复您的 PATH](https://code.claude.com/docs/zh-CN/troubleshoot-install.md#command-not-found-claude-after-installation)。
>
> 如果您看到 `The token '&&' is not a valid statement separator`，说明您在 PowerShell 中，而不是 CMD。如果您看到 `'irm' is not recognized as an internal or external command`，说明您在 CMD 中，而不是 PowerShell。
>
> 如果安装命令失败并显示 `syntax error near unexpected token '<'`、`403` 或其他任何错误，请参阅[排查安装问题](https://code.claude.com/docs/zh-CN/troubleshoot-install.md#find-your-error)以匹配错误并获得修复方案和替代安装方法。
>
> 建议在原生 Windows 上安装 [Git for Windows](https://git-scm.com/downloads/win)，以便 Claude Code 可以使用 Bash 工具。如果未安装 Git for Windows，Claude Code 将使用 PowerShell 作为 shell 工具。WSL 设置不需要 Git for Windows。
>
> > [!NOTE]
> >
> > 原生安装会在后台自动更新，以保持您使用最新版本。

> [tab] Homebrew
>
> ```bash theme={null}
> brew install --cask claude-code
> ```
>
> Homebrew 提供两个 casks。`claude-code` 跟踪稳定发布渠道，通常比最新版本晚约一周，并跳过有重大回归的版本。`claude-code@latest` 跟踪最新渠道，在新版本发布时立即接收。
>
> > [!NOTE]
> >
> > Homebrew 安装不会自动更新。运行 `brew upgrade claude-code` 或 `brew upgrade claude-code@latest`（取决于您安装的 cask）以获取最新功能和安全修复。

> [tab] WinGet
>
> ```powershell theme={null}
> winget install Anthropic.ClaudeCode
> ```
>
> > [!NOTE]
> >
> > WinGet 安装不会自动更新。定期运行 `winget upgrade Anthropic.ClaudeCode` 以获取最新功能和安全修复。

您也可以在 Debian、Fedora、RHEL 和 Alpine 上使用 [apt、dnf 或 apk](https://code.claude.com/docs/zh-CN/setup.md#install-with-linux-package-managers) 进行安装。

要确认安装成功，请运行：

```bash theme={null}
claude --version
```

该命令会打印一个版本号，后面跟着 `(Claude Code)`。

## 步骤 2：开始您的第一个会话

在任意项目目录中打开终端并启动 Claude Code：

```bash theme={null}
cd /path/to/your/project
claude
```

将 `/path/to/your/project` 替换为您要处理的项目路径。

首次使用时，Claude Code 会提示您登录。对于 Claude 订阅或 Console 账户，请按照提示在浏览器中完成身份验证。如果您已设置 `ANTHROPIC_API_KEY` 环境变量，并且在 Claude Code 询问是否使用该密钥时予以批准，Claude Code 将跳过登录提示。

您可以使用以下任一账户类型登录：

* [Claude Pro、Max、Ultra、Team 或 Enterprise](https://claude.com/pricing?utm_source=claude_code\&utm_medium=docs\&utm_content=quickstart_login)（推荐）
* [Claude Console](https://platform.claude.com/)（使用预付额度的 API 访问）。首次登录时，系统会在 Console 中自动创建一个"Claude Code"工作区，用于集中跟踪费用。
* [Amazon Bedrock、Google Cloud's Agent Platform 或 Microsoft Foundry](https://code.claude.com/docs/zh-CN/third-party-integrations.md)（企业云服务提供商）
* 自托管的 [Claude apps gateway](https://code.claude.com/docs/zh-CN/claude-apps-gateway)（如果您的组织部署了该网关）：管理员会预先配置网关 URL，`/login` 将直接打开 **Cloud gateway** 界面，供您使用企业 SSO 登录

登录后，您的凭据会被保存，无需再次登录。如需了解更多信息，请参阅[凭据管理](https://code.claude.com/docs/zh-CN/authentication.md#credential-management)。

随后将显示 Claude Code 提示符，其上方会显示版本、当前模型和工作目录。输入 `/help` 查看可用命令，或输入 `/resume` 继续之前的对话。如需稍后切换账户或重新进行身份验证，请在运行中的会话内输入 `/login`。

## 步骤 3：提出您的第一个问题

尝试以下命令之一：

```text wrap theme={null}
what does this project do?
```

Claude 将分析您的文件并提供摘要。您还可以提出更具体的问题：

```text wrap theme={null}
what technologies does this project use?
```

```text wrap theme={null}
where is the main entry point?
```

```text wrap theme={null}
explain the folder structure
```

您还可以询问 Claude 有关其自身功能的问题：

```text wrap theme={null}
what can Claude Code do?
```

```text wrap theme={null}
how do I create custom skills in Claude Code?
```

```text wrap theme={null}
can Claude Code work with Docker?
```

> [!NOTE]
>
> Claude Code 会根据需要读取您的项目文件。您无需手动添加上下文。

## 步骤 4：进行您的第一次代码更改

尝试一个小任务：

```text wrap theme={null}
add a hello world function to the main file
```

Claude Code 会找到合适的文件并向您展示更改。如果它在进行更改前征求确认，请选择 **Yes** 以批准。

会话的[权限模式](https://code.claude.com/docs/zh-CN/permission-modes.md)决定了 Claude 可以在不事先询问您的情况下执行哪些操作。随时按 `Shift+Tab` 即可切换当前会话的权限模式。

## 步骤 5：在 Claude Code 中使用 Git

Claude Code 让 Git 操作变得像对话一样简单：

```text wrap theme={null}
what files have I changed?
```

```text wrap theme={null}
commit my changes with a descriptive message
```

您还可以通过提示词执行更复杂的 Git 操作：

```text wrap theme={null}
create a new branch called feature/quickstart
```

```text wrap theme={null}
show me the last 5 commits
```

```text wrap theme={null}
help me resolve merge conflicts
```

## 步骤 6：修复 bug 或添加功能

用自然语言描述您想要的内容：

```text wrap theme={null}
add input validation to the user registration form
```

或修复现有问题：

```text wrap theme={null}
there's a bug where users can submit empty forms - fix it
```

## 步骤 7：试用其他常见工作流

再尝试几个提示词。您可以让 Claude 重构代码、编写测试、更新文档或审查您的更改：

```text wrap theme={null}
refactor the authentication module to use async/await instead of callbacks
```

```text wrap theme={null}
write unit tests for the calculator functions
```

```text wrap theme={null}
update the README with installation instructions
```

```text wrap theme={null}
review my changes and suggest improvements
```

> [!TIP]
>
> 像与一位乐于助人的同事交流一样与 Claude 对话。描述您想要实现的目标，它会帮助您达成。

## 基本命令

以下是日常使用中最重要的命令，按运行位置分组。

### Shell 命令

从终端运行这些命令以启动或恢复 Claude Code。

| 命令 | 功能 | 示例 |
| - | - | - |
| `claude` | 启动交互模式 | `claude` |
| `claude "task"` | 使用初始提示启动交互模式 | `claude "fix the build error"` |
| `claude -p "query"` | 运行一次性查询，然后退出 | `claude -p "explain this function"` |
| `claude -c` | 在当前目录中继续最近的对话 | `claude -c` |
| `claude -r` | 恢复之前的对话 | `claude -r` |

有关完整的 shell 命令列表，请参阅 [CLI 参考](https://code.claude.com/docs/zh-CN/cli-reference.md)。

### 会话命令

在 Claude Code 启动后，在其内部运行这些命令。

| 命令 | 功能 | 示例 |
| - | - | - |
| `/clear` | 清除对话历史 | `/clear` |
| `/help` | 显示可用命令 | `/help` |
| `/exit` 或 Ctrl+D 两次 | 退出 Claude Code | `/exit` |

有关完整的会话命令列表，请参阅 [命令参考](https://code.claude.com/docs/zh-CN/commands.md)。

## 初学者专业提示

有关更多信息，请参阅[最佳实践](https://code.claude.com/docs/zh-CN/best-practices.md)和[常见工作流](https://code.claude.com/docs/zh-CN/common-workflows.md)。

> [accordion] 对您的请求要具体
>
> 不要说："修复错误"
>
> 尝试："修复登录错误，用户输入错误凭证后看到空白屏幕"

> [accordion] 使用分步说明
>
> 将复杂任务分解为步骤：
>
> ```text wrap theme={null}
> 1. 为用户配置文件创建新的数据库表
> 2. 创建 API 端点以获取和更新用户配置文件
> 3. 构建允许用户查看和编辑其信息的网页
> ```

> [accordion] 让 Claude 先探索
>
> 在进行更改之前，让 Claude 理解您的代码：
>
> ```text wrap theme={null}
> 分析数据库架构
> ```
>
> ```text wrap theme={null}
> 构建一个仪表板，显示英国客户最常退货的产品
> ```

> [accordion] 使用快捷方式节省时间
>
> * 输入 `/` 查看所有命令和 skills
> * 使用 Tab 进行命令补全
> * 按 ↑ 查看命令历史
> * 按 `Shift+Tab` 循环切换权限模式

## 接下来呢？

现在您已经学习了基础知识，探索更多高级功能：

* [Claude Code 如何工作](https://code.claude.com/docs/zh-CN/how-claude-code-works.md)：了解智能体循环、内置工具以及 Claude Code 如何与您的项目交互
* [最佳实践](https://code.claude.com/docs/zh-CN/best-practices.md)：通过有效的提示和项目设置获得更好的结果
* [常见工作流](https://code.claude.com/docs/zh-CN/common-workflows.md)：常见任务的分步指南
* [扩展 Claude Code](https://code.claude.com/docs/zh-CN/features-overview.md)：使用 CLAUDE.md、skill、hook、MCP 等进行自定义

有关安装选项、手动更新或卸载说明，请参阅[高级设置](https://code.claude.com/docs/zh-CN/setup.md)。

## 获取帮助

* **在 Claude Code 中**：输入 `/help` 或询问"我如何..."
* **文档**：浏览本站的其他指南
* **课程**：参加 [Claude Code 101](https://academy.claude.com/courses/claude-code-101) 和 [Claude Academy](https://academy.claude.com/) 上的其他免费自学课程
* **社区**：加入 [Discord 服务器](https://www.anthropic.com/discord) 获取提示和支持
