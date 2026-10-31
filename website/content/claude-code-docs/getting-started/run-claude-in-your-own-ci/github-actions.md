> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Claude Code GitHub Actions

> 在 GitHub Actions 工作流中运行 Claude Code，响应 @claude 提及、自动化任务并将 issue 转换为拉取请求

[Claude Code GitHub Actions](https://github.com/anthropics/claude-code-action) 是一个 GitHub Action，在您的仓库工作流中运行 Claude Code。在拉取请求或 issue 评论中提及 `@claude`，让 Claude 分析代码、实现更改并推送提交。您也可以给 Claude Code GitHub Action 一个提示，让它在任何 GitHub 事件上自动运行。使用它将 issue 转换为拉取请求、从评论中修复错误或自动化重复任务。

多个产品共享 Claude Code 名称。本页涵盖 `claude-code-action` 工作流集成，您可以使用仓库中的工作流文件进行配置。对于相关产品，请参阅：

* [Code Review](https://code.claude.com/docs/zh-CN/code-review.md)：在每个拉取请求上自动审查，无需编写工作流
* [Claude Code in the cloud](https://code.claude.com/docs/zh-CN/claude-code-on-the-web.md)：在云基础设施上运行的 Claude Code 会话，而不是在您的机器上
* [Claude Agent SDK](https://code.claude.com/docs/zh-CN/agent-sdk/overview.md)：GitHub Actions 之外的自定义自动化。Claude Code GitHub Action 建立在 SDK 之上
* [GitHub Enterprise Server](https://code.claude.com/docs/zh-CN/github-enterprise-server.md)：带有自托管 GitHub 的 Claude Code

## 设置

您可以通过以下两种方式之一设置 Claude Code GitHub Action：

* **快速设置**：在 Claude Code 中运行 `/install-github-app`。Claude Code 会为您安装 GitHub App、添加身份验证密钥，并准备好工作流 Pull Request
* **手动设置**：自行安装应用、添加密钥，并将工作流文件复制到仓库中。如果您不在本地运行 Claude Code、该命令执行失败，或者您希望完全掌控工作流文件，请使用此方式

无论采用哪种方式，您都需要拥有该仓库的管理员权限。

### 快速设置

`/install-github-app` 仅适用于 github.com 仓库。如果仓库的 git 远程地址位于 gitlab.com 或 bitbucket.org，该命令会输出提示并退出，而不会开始设置。如需在 GitLab 流水线中运行 Claude Code，请参阅 [Claude Code GitLab CI/CD](https://code.claude.com/docs/zh-CN/gitlab-ci-cd.md)。

开始之前，请安装 [GitHub CLI](https://cli.github.com) 并使用 `gh auth login` 进行身份验证。Claude Code 会检查该工具，如果缺失会发出警告。

在要连接的仓库中打开 `claude`，运行 `/install-github-app`，然后按照提示操作。Claude Code 会安装 Claude GitHub App，然后为工作流设置身份验证密钥：

* 如果 Claude Code 已有 API 密钥，则会复用该密钥；如果仓库已设置 `ANTHROPIC_API_KEY` 密钥，还会询问是否保留现有密钥
* 否则，您可以选择使用 Claude 订阅创建长期令牌，或粘贴 API 密钥

Claude Code 会将凭据保存为仓库密钥：API 密钥命名为 `ANTHROPIC_API_KEY`，订阅令牌命名为 `CLAUDE_CODE_OAUTH_TOKEN`。

随后，Claude Code 会推送一个包含您所选工作流文件的分支，这些文件已配置为使用该密钥，并在浏览器中打开 GitHub，其中已准备好待创建的 Pull Request。创建并合并该 Pull Request 后，即可在仓库中使用 `@claude`。

如需在设置中途停止，请按 Esc。正在进行的步骤会完成，后续步骤不会再开始。结束消息会列出仓库中已经发生的操作，例如已推送的分支或已保存的密钥。

如果您选择了审查工作流，Claude 会将每次审查直接发布在 Pull Request 上：对发现的每个问题发布行内评论，若未发现问题则发布一条摘要评论。Claude 会跳过某些 Pull Request，例如草稿。[审查工作流示例](#run-a-skill)使用相同的 skill 并列出了这些情况。在 v2.1.229 之前，Claude 仅将审查结果写入工作流运行日志。

如需更新由早期版本生成的审查工作流，请执行以下任一操作：

* 再次运行 `/install-github-app`。当仓库中已有 `claude.yml` 时，选择 **Update workflow file with latest version**。Claude Code 会将最新的工作流文件推送到新分支并打开 Pull Request，与首次安装时相同。
* 自行将 [审查工作流示例](#run-a-skill)中的 `--comment` 参数和 `claude_args` 行添加到已提交的文件中，这样可以保留您对该文件所做的其他修改。

安装 GitHub App 后，Claude Code 会询问是否继续进行 GitHub Actions 设置。选择 **Skip for now** 将在仅安装 GitHub App 的状态下停止。之后可再次运行 `/install-github-app` 来完成工作流和密钥步骤。

> [!NOTE]
>
> * 安装 GitHub App 时，您会授予它多项权限。完整列表请参阅 [GitHub App 权限](#github-app-permissions)
> * 快速设置适用于 Claude API 和 Claude 订阅。如果您使用 Amazon Bedrock、Google Cloud 的 Agent Platform 或 Microsoft Foundry，请参阅[在云服务提供商上使用 Claude Code GitHub Actions](https://code.claude.com/docs/zh-CN/github-actions-cloud-providers.md)

### 手动设置

如需在不运行 `/install-github-app` 的情况下配置 Claude Code GitHub Action，请自行安装应用、添加密钥并复制工作流文件：

**1. 安装 Claude GitHub App**

将 [Claude GitHub App](https://github.com/apps/claude) 安装到您的仓库。Claude Code GitHub Action 依赖该应用的以下三项权限：

* **Contents**：读写，以便 Claude 可以修改仓库文件
* **Issues**：读写，以便 Claude 可以回复 issue
* **Pull requests**：读写，以便 Claude 可以创建 PR 并推送更改

安装过程中，您还会授予其他 Claude 功能所使用的权限。完整列表请参阅 [GitHub App 权限](#github-app-permissions)。

**2. 添加身份验证密钥**

根据您的身份验证方式，向仓库添加以下密钥之一。请参阅 GitHub 的[在 GitHub Actions 中使用密钥](https://docs.github.com/en/actions/security-guides/using-secrets-in-github-actions)指南。

* `ANTHROPIC_API_KEY`：来自 [Claude Console](https://platform.claude.com) 的 Claude API 密钥
* `CLAUDE_CODE_OAUTH_TOKEN`：使用您的 Claude 订阅进行身份验证的 OAuth 令牌，适用于 Pro、Max、Ultra、Team 和 Enterprise 套餐。在本地运行 `claude setup-token` 即可生成。请参阅[生成长期令牌](https://code.claude.com/docs/zh-CN/authentication.md#generate-a-long-lived-token)

在工作流文件中，将密钥传递给对应的输入：API 密钥使用 `anthropic_api_key`，OAuth 令牌使用 `claude_code_oauth_token`。

**3. 复制工作流文件**

将 [examples/claude.yml](https://github.com/anthropics/claude-code-action/blob/main/examples/claude.yml) 复制到仓库的 `.github/workflows/` 目录中。该文件是一个可直接使用的工作流，而不仅仅是示例。按提交时的状态，每当有人在 issue 或 Pull Request 中提及 `@claude` 时，Claude 都会做出回应，并使用 `ANTHROPIC_API_KEY` 密钥进行身份验证。如果您添加的是 `CLAUDE_CODE_OAUTH_TOKEN`，请将工作流中的 `anthropic_api_key` 行改为 `claude_code_oauth_token: ${{ secrets.CLAUDE_CODE_OAUTH_TOKEN }}`。

> [!TIP]
>
> 设置完成后，在 issue 或 PR 评论中标记 `@claude` 来测试 Claude Code GitHub Action。

### 为组织进行设置

无论是快速设置还是手动设置，每次只能配置一个仓库。如需在整个组织中推广 Claude Code GitHub Action：

* 在组织级别安装一次 [Claude GitHub App](https://github.com/apps/claude)，选择所有仓库或指定的仓库列表
* 将身份验证密钥存储为组织级别的 Actions 密钥，这样每个仓库就无需单独保存副本
* 将工作流文件添加到每个需要运行 Claude Code GitHub Action 的仓库，或将该作业定义为一个[可复用工作流](https://docs.github.com/en/actions/using-workflows/reusing-workflows)，供各仓库调用

对于跨仓库共享的密钥，请使用来自 [Claude Console](https://platform.claude.com) 的 API 密钥进行身份验证，而不是 OAuth 令牌，因为 OAuth 令牌与运行 `claude setup-token` 的用户的订阅绑定。

如需完全避免存储长期密钥，请通过工作负载身份联合进行身份验证：Claude Code GitHub Action 会通过 Claude Console 服务账号，将工作流的 GitHub OpenID Connect (OIDC) 令牌交换为 Claude API 访问权限。请设置以下输入：

* `anthropic_federation_rule_id`：联合规则 ID，`fdrl_...`
* `anthropic_organization_id`：您的 Anthropic 组织 ID
* `anthropic_service_account_id`：服务账号 ID，`svac_...`。可选，因为您在 Console 中创建的联合规则已指向某个服务账号
* `anthropic_workspace_id`：工作区 ID，`wrkspc_...`。当联合规则指向单个工作区时为可选

为工作流授予 `id-token: write` 权限。即使您传入了自己的 `github_token`，Claude Code GitHub Action 也需要此权限来完成联合交换。Console 端的配置请参阅 [Claude Code GitHub Action 设置指南](https://github.com/anthropics/claude-code-action/blob/main/docs/setup.md)。

关于安全审查中涉及的数据处理和保留问题，请参阅[数据使用](https://code.claude.com/docs/zh-CN/data-usage.md)和[安全](https://code.claude.com/docs/zh-CN/security.md)。

### 卸载

如需移除 Claude Code GitHub Action，请撤销您的安装中适用的每一项设置：

* **工作流文件**：从 `.github/workflows/` 中删除使用 `anthropics/claude-code-action` 的工作流。如果您使用的是快速设置，请查找 `claude.yml`，如果选择了审查工作流，还需查找 `claude-code-review.yml`。删除工作流后，Claude Code GitHub Action 将不再运行
* **密钥**：从仓库中删除 `ANTHROPIC_API_KEY` 或 `CLAUDE_CODE_OAUTH_TOKEN` 密钥；如果您已[跨仓库共享该密钥](#set-up-for-an-organization)，还需从组织级别的 Actions 密钥中删除。删除密钥后，其中保存的凭据仍然有效。如需彻底停用 API 密钥，还需在 [Claude Console](https://platform.claude.com) 中删除该密钥
* **GitHub App**：在仓库或组织设置的 GitHub Apps 下卸载 Claude GitHub App，但前提是您没有将其用于其他 Claude 功能，例如 Code Review 或网页版自动修复

如果您配置了[云服务提供商](https://code.claude.com/docs/zh-CN/github-actions-cloud-providers.md)，还需删除提供商密钥，例如 `AWS_ROLE_TO_ASSUME`、`GCP_*` 密钥或 `AZURE_*` 密钥，并卸载自定义 GitHub App 及其 `APP_ID` 和 `APP_PRIVATE_KEY` 密钥。

### GitHub App 权限

[Claude GitHub App](https://github.com/apps/claude) 由所有与 GitHub 集成的 Claude 功能共享，包括 Claude Code GitHub Action、[Code Review](https://code.claude.com/docs/zh-CN/code-review.md) 以及云端会话中的 [Pull Request 自动修复](https://code.claude.com/docs/zh-CN/claude-code-on-the-web.md#auto-fix-pull-requests)。一个 GitHub App 只有一套涵盖其所有功能的权限，因此其中包含一些 Claude Code GitHub Action 并不使用的权限。

安装该应用时，您会授予以下权限：

| 权限 | 访问级别 |
| - | - |
| Actions | 读写 |
| Administration | 读取 |
| Checks | 读写 |
| Contents | 读写 |
| Discussions | 读写 |
| Issues | 读写 |
| Members | 读取 |
| Merge queues | 读取 |
| Metadata | 读取 |
| Pull requests | 读写 |
| Repository hooks | 读写 |
| Statuses | 读取 |
| Workflows | 读写 |

权限集也可能先于使用这些权限的功能而发生变化。当应用请求之前没有的权限时，GitHub 会提示账号所有者（对于组织安装则为组织所有者）进行批准，在其批准之前，安装将保留原有权限。例如，当 Actions 访问权限从读取变为读写时，应用将能够重新运行工作流，而不仅仅是查看运行记录和日志，因此 GitHub 会请求所有者批准此更改。

安装该应用即表示您接受其完整的权限集。GitHub 不允许只接受其中一部分。如果您的组织要求仅授予 Claude Code GitHub Action 所使用的权限，请按照 [Claude Code GitHub Action 设置指南](https://github.com/anthropics/claude-code-action/blob/main/docs/setup.md)创建一个仅包含 Contents、Issues 和 Pull requests 权限的自定义 GitHub App。自定义应用仅适用于 Claude Code GitHub Action。Code Review 和网页版自动修复仍需使用官方应用。

关于 Claude Code GitHub Action 如何限制 Claude 使用这些权限的详细信息，请参阅[安全文档](https://github.com/anthropics/claude-code-action/blob/main/docs/security.md)。

## 交互和自动化模式

Claude Code GitHub Action 从您的工作流配置中检测如何运行：

* **交互模式**：当工作流不提供 `prompt` 输入时，Claude 等待触发短语 `@claude`（默认），在 issue 或拉取请求评论、拉取请求审查或新打开的 issue 的正文或标题中，然后响应该请求。进度和结果显示为触发 issue 或 PR 上的评论。
* **自动化模式**：当工作流提供 `prompt` 输入时，Claude 运行而不等待提及，仅受[谁可以触发运行](#who-can-trigger-runs)的检查约束。默认情况下，结果显示在工作流运行日志中而不是评论中。Claude 可以在提示指导它并且它有可以发布的工具时发布到 issue 或拉取请求，如[代码审查示例](#run-a-skill)中所示。

### 谁可以触发运行

在两种模式中，Claude Code GitHub Action 在 Claude 开始之前对触发参与者运行两个检查，当任一检查拒绝它时运行失败：

* **写入访问**：在 issue 和拉取请求事件上，触发用户必须对仓库具有写入访问权限。要允许没有写入访问权限的特定用户，请设置 `allowed_non_write_users` 并传递您自己的 `github_token` 输入。没有用户创建的事件，例如 `schedule` 触发器，会跳过此检查。
* **人类参与者**：在每个事件上，Claude Code GitHub Action 拒绝机器人参与者，除非您在 `allowed_bots` 中列出它，这可以防止机器人在循环中触发 Claude。此检查也适用于计划运行，GitHub 将其归因于仓库用户，通常是最后更改工作流 `cron` 计划的用户。如果该用户是机器人，请在 `allowed_bots` 中列出它。

## 示例用例

[examples 目录](https://github.com/anthropics/claude-code-action/tree/main/examples)包含针对不同场景的现成工作流。

本页上的示例显示 API 密钥身份验证。如果您使用 Claude 订阅进行身份验证，请将任何示例中的 `anthropic_api_key` 行替换为 `claude_code_oauth_token: ${{ secrets.CLAUDE_CODE_OAUTH_TOKEN }}`。

### 响应 @claude 提及

此工作流在交互模式下运行 Claude Code GitHub Action，因此每当有人在 issue 或 PR 评论中提及 `@claude` 时，Claude 都会响应。

```yaml theme={null}
name: Claude Code
on:
  issue_comment:
    types: [created]
  pull_request_review_comment:
    types: [created]
jobs:
  claude:
    if: contains(github.event.comment.body, '@claude')
    runs-on: ubuntu-latest
    permissions:
      contents: write
      pull-requests: write
      issues: write
      id-token: write
      actions: read
    steps:
      - uses: actions/checkout@v6
        with:
          fetch-depth: 1
      - uses: anthropics/claude-code-action@v1
        with:
          anthropic_api_key: ${{ secrets.ANTHROPIC_API_KEY }}
```

此工作流中不是样板的部分：

* `id-token: write`：Claude Code GitHub Action 的默认 GitHub App 身份验证所需
* `actions: read`：让 Claude 读取 PR 上的 CI 结果
* `actions/checkout`：给 Claude 一个本地仓库副本来工作
* `if`：防止运行器在不提及 `@claude` 的评论上启动。Claude Code GitHub Action 也在响应之前检查触发短语本身

工作流就位后，在任何 issue 或 PR 评论中提及 `@claude` 并提出请求：

```text wrap theme={null}
@claude implement this feature based on the issue description
@claude how should I implement user authentication for this endpoint?
@claude fix the TypeError in the user dashboard component
```

Claude 在同一 issue 或 PR 上的评论中回复并在工作时更新它。

### 运行 skill

`prompt` 输入接受 [skill](https://code.claude.com/docs/zh-CN/skills.md) 调用以及纯文本：

* 对于仓库的 `.claude/skills/` 目录中的 skill，在 `anthropics/claude-code-action` 步骤之前运行 `actions/checkout`，以便 skill 文件在运行器上可用，然后将 `/skill-name` 作为 `prompt` 传递。
* 对于打包在[插件](https://code.claude.com/docs/zh-CN/plugins/overview.md)中的 skill，使用 `plugin_marketplaces` 和 `plugins` 输入安装插件，然后将命名空间的 `/plugin-name:skill-name` 作为 `prompt` 传递。`plugins` 输入采用 `plugin-name@marketplace-name`，其中市场名称来自市场自己的清单而不是其仓库 URL。

以下工作流安装 `code-review` 插件并在拉取请求打开、更新、重新打开或标记为准备审查时运行其 skill。它运行与快速设置中的审查工作流相同的插件。当您想自己控制提示、模型和触发器时，请使用这样的工作流。对于无需维护工作流文件的自动审查，请参阅 [Code Review](https://code.claude.com/docs/zh-CN/code-review.md)。在公共仓库上，GitHub 从 fork 拉取请求触发的运行中扣留密钥，因此审查仅在来自同一仓库中分支的拉取请求上运行。

```yaml theme={null}
name: Code Review
on:
  pull_request:
    types: [opened, synchronize, ready_for_review, reopened]
jobs:
  review:
    runs-on: ubuntu-latest
    permissions:
      contents: read
      pull-requests: read
      issues: read
      id-token: write
    steps:
      - uses: actions/checkout@v6
        with:
          fetch-depth: 1
      - uses: anthropics/claude-code-action@v1
        with:
          anthropic_api_key: ${{ secrets.ANTHROPIC_API_KEY }}
          plugin_marketplaces: "https://github.com/anthropics/claude-plugins-official.git"
          plugins: "code-review@claude-plugins-official"
          prompt: "/code-review:code-review --comment ${{ github.repository }}/pull/${{ github.event.pull_request.number }}"
          claude_args: '--allowedTools "mcp__github_inline_comment__create_inline_comment"'
```

此工作流中的两行控制审查的去向：

* **`--comment`**：Claude 在拉取请求上发布其审查，作为它发现的每个问题的内联评论或在它没有发现任何问题时作为一个摘要评论。没有它，Claude 不发布任何内容，您在工作流运行日志中读取发现。
* **`claude_args`**：即使 skill 自己的 `allowed-tools` frontmatter 命名相同的工具，也要保留此行，因为 Claude Code GitHub Action 仅在 `claude_args` 中的 `--allowedTools` 命名它时才启动发布内联评论的 MCP 服务器。

Claude 跳过草稿和已关闭的拉取请求、它判断不需要审查的拉取请求（例如自动化或琐碎的拉取请求）以及已经有来自 Claude 的评论的拉取请求。

### 按计划运行

使用 `prompt` 输入，Claude Code GitHub Action 在任何 GitHub 事件上以自动化模式运行，包括 cron 计划。对于纯文本提示，Claude 没有 shell 或 GitHub API 访问权限，直到您授予提示需要的工具，使用 `claude_args` 中的 `--allowedTools` 或 `settings` 输入中的 [`permissions.allow` 规则](https://code.claude.com/docs/zh-CN/permissions#permission-rule-syntax)。如果您改为调用 skill，Claude 可以使用其 [`allowed-tools` frontmatter](https://code.claude.com/docs/zh-CN/skills#pre-approve-tools-for-a-skill) 授予的工具。GitHub 仅从默认分支运行计划工作流，在公共仓库中，在 60 天没有仓库活动后禁用计划。

此工作流在每天 09:00 UTC 在工作流运行日志中生成报告。其 `claude_args` 行[传递 CLI 参数](#pass-cli-arguments)，选择模型并允许两个 GitHub MCP 工具。Claude 通过这些工具使用 GitHub API 读取提交和 issue，因此您可以省略 checkout 步骤：

```yaml theme={null}
name: Daily Report
on:
  schedule:
    - cron: "0 9 * * *"
jobs:
  report:
    runs-on: ubuntu-latest
    permissions:
      contents: read
      issues: read
      id-token: write
    steps:
      - uses: anthropics/claude-code-action@v1
        with:
          anthropic_api_key: ${{ secrets.ANTHROPIC_API_KEY }}
          prompt: "Generate a summary of yesterday's commits and open issues"
          claude_args: |
            --model claude-opus-5-5
            --allowedTools "mcp__github__list_commits,mcp__github__list_issues"
```

## 最佳实践

### 在 CLAUDE.md 中定义项目标准

在您的仓库根目录创建一个 `CLAUDE.md` 文件来定义代码风格指南、审查标准、项目特定规则和首选模式。Claude 在创建 PR 和响应请求时遵循这些指南。有关详细信息，请参阅[内存文档](https://code.claude.com/docs/zh-CN/memory.md)。

### 保护您的凭证

> [!WARNING]
>
> 永远不要直接将 API 密钥或 OAuth 令牌提交到您的仓库。始终将它们存储为 GitHub Secrets 并在工作流中引用它们，例如 `anthropic_api_key: ${{ secrets.ANTHROPIC_API_KEY }}`。

仅授予工作流所需的权限，并在合并前审查 Claude 的更改。

有关安全指导，包括权限和身份验证，请参阅 [Claude Code Action 安全文档](https://github.com/anthropics/claude-code-action/blob/main/docs/security.md)。

### 管理成本

每次运行消耗两种资源：

* **GitHub Actions 分钟**：Claude Code GitHub Action 在 GitHub 托管的运行器上运行，这会消耗您的 GitHub Actions 分钟。有关定价和分钟限制，请参阅 [GitHub 的计费文档](https://docs.github.com/en/billing/managing-billing-for-your-products/managing-billing-for-github-actions/about-billing-for-github-actions)。
* **API 令牌**：每次交互根据提示和响应的长度、任务复杂性和代码库大小消耗令牌。有关当前令牌费率，请参阅 [Claude 的定价页面](https://claude.com/platform/api)。如果您使用 OAuth 令牌进行身份验证，运行使用您的 Claude 订阅而不是 API 计费。

您可以通过给 Claude 更清晰的上下文和限制每次运行可以做多少工作来降低两种成本：

* 编写特定的 `@claude` 请求，以便 Claude 需要更少的轮次来完成
* 使用 issue 模板提前提供上下文
* 保持您的 `CLAUDE.md` 简洁，因为 Claude 在每次运行时都会读取它
* 在 `claude_args` 中设置 `--max-turns` 以限制迭代
* 设置工作流级别的超时以避免失控的作业
* 使用 GitHub 的并发控制来限制并行运行

有关跨组织的使用跟踪，请参阅[分析仪表板](https://code.claude.com/docs/zh-CN/analytics.md)和[监控](https://code.claude.com/docs/zh-CN/monitoring-usage.md)。有关如何测量和计费使用情况，请参阅[成本](https://code.claude.com/docs/zh-CN/costs.md)。

## 使用云提供商

默认情况下，Claude Code GitHub Action 使用您的 API 密钥或 OAuth 令牌直接调用 Claude API。要通过您自己的云账户路由推理，请设置您的提供商的输入并按照[将 Claude Code GitHub Actions 与云提供商一起使用](https://code.claude.com/docs/zh-CN/github-actions-cloud-providers.md)：

* **Amazon Bedrock**：`use_bedrock: "true"`
* **Google Cloud 的 Agent Platform**：`use_vertex: "true"`
* **Microsoft Foundry**：`use_foundry: "true"`

对于所有三个提供商，您通过 OIDC 身份联合进行身份验证，而不是 Claude API 密钥，因此您不在仓库中存储静态云凭证。

## 故障排除

### Claude 不响应 @claude 命令

* 验证 GitHub App 是否安装在仓库上
* 检查工作流是否为仓库启用
* 确保您的 API 密钥或 OAuth 令牌在仓库密钥中设置
* 确认评论包含 `@claude` 作为完整单词，而不是 `/claude` 或 `@claude-bot`
* 确认评论用户对仓库具有写入访问权限。有关例外，请参阅[谁可以触发运行](#who-can-trigger-runs)

### CI 不在 Claude 的提交上运行

* GitHub 不会在使用默认 `GITHUB_TOKEN` 进行的提交上触发工作流。如果您将 `github_token: ${{ secrets.GITHUB_TOKEN }}` 传递给 Claude Code GitHub Action，请删除它以便它作为 Claude GitHub App 进行身份验证，或改为传递自定义应用令牌
* 检查您的 CI 工作流的触发器是否包括 Claude 的推送产生的事件，例如 `push` 或 `pull_request`

### 身份验证错误

* 通过在本地使用 `claude` 测试 API 密钥或 OAuth 令牌来确认它有效，然后再调试工作流
* 对于 Bedrock、Agent Platform 和 Foundry，请参阅云提供商页面的[故障排除部分](https://code.claude.com/docs/zh-CN/github-actions-cloud-providers.md#troubleshooting)

有关更多解决方案，请参阅 Claude Code GitHub Action 的 [FAQ](https://github.com/anthropics/claude-code-action/blob/main/docs/faq.md)。

## 高级配置

### Action 参数

这些是最常用的输入。每个都映射到 `anthropics/claude-code-action` 步骤中的 `with:` 键。

| 参数 | 描述 | 必需 |
| - | - | - |
| `prompt` | Claude 的说明，作为纯文本或 [skill](https://code.claude.com/docs/zh-CN/skills.md) 调用。省略时，Claude 改为响应[触发短语](#interactive-and-automation-modes) | 否 |
| `claude_args` | 传递给 Claude Code 的 CLI 参数 | 否 |
| `anthropic_api_key` | Claude API 密钥 | 对于 Claude API，除非您使用 `claude_code_oauth_token` 或[工作负载身份联合](#set-up-for-an-organization)。不用于 Bedrock、Agent Platform 或 Foundry |
| `claude_code_oauth_token` | 用于使用 Claude 订阅进行身份验证的 OAuth 令牌，使用 `claude setup-token` 生成 | 否 |
| `github_token` | 用于 GitHub 操作的令牌。省略时，Claude Code GitHub Action 作为 Claude GitHub App 进行身份验证 | 否 |
| `plugin_marketplaces` | 插件市场 Git URL 的换行符分隔列表 | 否 |
| `plugins` | 执行前要安装的插件名称的换行符分隔列表 | 否 |
| `settings` | Claude Code 设置，作为 JSON 字符串或设置 JSON 文件的路径 | 否 |
| `trigger_phrase` | Claude 响应的触发短语。默认：`@claude` | 否 |
| `use_bedrock` | 使用 Amazon Bedrock 而不是 Claude API | 否 |
| `use_vertex` | 使用 Google Cloud 的 Agent Platform 而不是 Claude API | 否 |
| `use_foundry` | 使用 Microsoft Foundry 而不是 Claude API | 否 |

有关完整的输入列表，请参阅 Claude Code GitHub Action 的[配置参考](https://github.com/anthropics/claude-code-action/blob/main/docs/usage.md#inputs)。

### 传递 CLI 参数

`claude_args` 参数接受任何 [Claude Code CLI 参数](https://code.claude.com/docs/zh-CN/cli-reference.md)：

```yaml theme={null}
claude_args: "--max-turns 5 --model claude-sonnet-5 --mcp-config /path/to/config.json"
```

常见参数：

* `--max-turns`：限制对话轮数
* `--model`：要使用的模型，例如 `claude-sonnet-5`。没有此参数，Claude Code GitHub Action 使用 Claude Code [默认模型](https://code.claude.com/docs/zh-CN/model-config.md)
* `--mcp-config`：[MCP 配置](https://code.claude.com/docs/zh-CN/mcp.md)的路径
* `--allowedTools`：允许的工具的逗号分隔列表。`--allowed-tools` 别名也可以使用
* `--debug`：启用调试输出

## 从 beta 升级

如果您的工作流仍然引用 `anthropics/claude-code-action@beta`，请将它们更新为 v1：

1. 在 `uses` 行中将 `@beta` 更改为 `@v1`
2. 删除 `mode` 输入，因为 Claude Code GitHub Action 现在[自动检测模式](#interactive-and-automation-modes)
3. 将 `direct_prompt` 替换为 `prompt`
4. 将 CLI 选项（如 `max_turns` 和 `model`）移到 `claude_args` 中。`custom_instructions` 没有同名标志，变成 `--append-system-prompt`

有关完整的输入映射和前后示例，请参阅[迁移指南](https://github.com/anthropics/claude-code-action/blob/main/docs/migration-guide.md)。

## 接下来

* [将 Claude Code GitHub Actions 与云提供商一起使用](https://code.claude.com/docs/zh-CN/github-actions-cloud-providers.md)：通过 Amazon Bedrock、Google Cloud 的 Agent Platform 或 Microsoft Foundry 路由推理
* [配置参考](https://github.com/anthropics/claude-code-action/blob/main/docs/usage.md#inputs)：action 输入的完整列表
* [Examples 目录](https://github.com/anthropics/claude-code-action/tree/main/examples)：更多场景的现成工作流
* [Code Review](https://code.claude.com/docs/zh-CN/code-review.md)：无需维护工作流文件的自动拉取请求审查
