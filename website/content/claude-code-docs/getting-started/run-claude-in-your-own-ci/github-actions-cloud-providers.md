> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# 通过云提供商使用 Claude Code GitHub Actions

> 通过 Amazon Bedrock、Google Cloud 的 Agent Platform 或 Microsoft Foundry 而不是 Claude API 运行 Claude Code GitHub Actions

[Claude Code GitHub Actions](https://code.claude.com/docs/zh-CN/github-actions.md) 默认调用 Claude API。要改为通过您自己的云账户路由推理，请设置 Claude Code GitHub Action 的 provider 输入，并配置您的云以信任工作流的 OpenID Connect (OIDC) 令牌。工作流使用该令牌进行身份验证，因此您无需在存储库中存储长期的云凭证。

> [!NOTE]
>
> 本页基于 [GitHub Actions 设置](https://code.claude.com/docs/zh-CN/github-actions.md#setup)。它假设您已经了解工作流文件和 `anthropics/claude-code-action` 步骤，仅涵盖云提供商所做的更改。

## 选择您的提供商

Claude Code GitHub Action 支持三个提供商，下面的设置步骤仅在云端配置上有所不同。使用您的组织已经拥有 Claude 模型访问权限的提供商。您通过在 `anthropics/claude-code-action` 步骤的 `with:` 块中的一个输入来告诉 Claude Code GitHub Action 使用哪个提供商：

* **Amazon Bedrock**: `use_bedrock: "true"`
* **Google Cloud 的 Agent Platform**: `use_vertex: "true"`
* **Microsoft Foundry**: `use_foundry: "true"`

[设置集成](#set-up-the-integration) 下的完整工作流示例已经包含了每个提供商的输入。

## 前置条件

在开始之前，您需要：

* 对运行 Claude Code GitHub Action 的存储库的管理员访问权限，以安装 GitHub App 并添加密钥
* 在您的云账户中创建身份资源的权限：AWS 上的 IAM 角色和 OIDC 身份提供商、Google Cloud 上的 Workload Identity Federation 资源和服务账户，或 Azure 上的 Microsoft Entra 应用程序
* 在您的提供商上拥有 Claude 模型访问权限：
  * **Amazon Bedrock**: 获得对 Claude 模型的访问权限。跨区域推理配置文件，例如本页示例中的 `us.` 模型 ID，需要在其区域组的每个区域中获得访问权限。请参阅 [Amazon Bedrock 上的 Claude Code](https://code.claude.com/docs/zh-CN/amazon-bedrock.md)
  * **Google Cloud 的 Agent Platform**: 一个启用了 Agent Platform API 的项目以及对 Claude 模型的访问权限。请参阅 [Google Cloud 的 Agent Platform 上的 Claude Code](https://code.claude.com/docs/zh-CN/google-vertex-ai.md)
  * **Microsoft Foundry**: 一个具有 Claude 模型部署的 Foundry 资源。请参阅 [Microsoft Foundry 上的 Claude Code](https://code.claude.com/docs/zh-CN/microsoft-foundry.md)

## 设置集成

除了前置条件外，您需要为 Claude Code GitHub Action 创建 GitHub 身份、云端信任配置、存储库密钥和工作流文件。下面的步骤将逐一介绍每一项。

**1. 选择 GitHub 身份**

Claude Code GitHub Action 通过 GitHub 身份推送提交和发布评论。[快速设置](https://code.claude.com/docs/zh-CN/github-actions.md#quick-setup) 为此安装了官方 Claude GitHub App。使用云提供商时，您可以自己选择身份：

* **官方 [Claude GitHub App](https://github.com/apps/claude)**: 在存储库上安装它，或如果已经安装，请跳到下一步
* **自定义 GitHub App**: 当您只想要 Claude Code GitHub Action 使用的三个权限而不是[官方应用的完整权限集](https://code.claude.com/docs/zh-CN/github-actions.md#github-app-permissions)时，创建您自己的应用
* **GitHub 的自动 `GITHUB_TOKEN`**: 无需创建或安装应用，但 GitHub 不会在使用它进行的提交上触发您的 CI 工作流

第四步中的工作流示例使用自定义应用进行身份验证。该步骤还说明了如何为其他两个选项进行更改。

要创建自定义应用，[注册一个新的 GitHub App](https://docs.github.com/en/apps/creating-github-apps/registering-a-github-app/registering-a-github-app)，禁用 webhooks，因为此集成不使用它们。授予它三个存储库权限：

* **Contents**: 读和写
* **Issues**: 读和写
* **Pull requests**: 读和写

注册应用后，生成一个私钥并保留下载的 `.pem` 文件，从应用的设置页面记下应用 ID，并在运行 Claude Code GitHub Action 的存储库上[安装应用](https://docs.github.com/en/apps/using-github-apps/installing-your-own-github-app)。您将在第三步中将密钥和 ID 添加为密钥。

**2. 配置云身份验证**

配置您的云以信任 GitHub 向工作流颁发的 OIDC 令牌，以便每个工作流运行都获得短期的云凭证。每个选项卡中的项目总结了要创建的内容，每个选项卡都链接到云供应商自己的控制台级步骤指南。

> [tab] Amazon Bedrock
>
> 在您的 AWS 账户中创建信任配置，遵循 [AWS 创建 OIDC 身份提供商指南](https://docs.aws.amazon.com/IAM/latest/UserGuide/id_roles_providers_create_oidc.html)：
>
> * 添加一个 GitHub OIDC 身份提供商，提供商 URL 为 `https://token.actions.githubusercontent.com`，受众为 `sts.amazonaws.com`
> * 创建一个由该提供商作为 Web 身份信任的 IAM 角色，并附加来自 [IAM 配置](https://code.claude.com/docs/zh-CN/amazon-bedrock.md#iam-configuration) 的作用域调用策略，该策略授予 `bedrock:InvokeModel`、`bedrock:InvokeModelWithResponseStream`、`bedrock:ListInferenceProfiles` 和 `bedrock:GetInferenceProfile`，以及两个 `aws-marketplace` 订阅操作
> * 使用主题条件（例如 `repo:your-org/your-repo:*`）将角色的信任策略限制在您的存储库。有关声明格式，请参阅 [GitHub 的 OIDC 加固指南](https://docs.github.com/en/actions/deployment/security-hardening-your-deployments/about-security-hardening-with-openid-connect)
>
> 记下角色的 ARN。您将在下一步中将其添加为密钥。

> [tab] Google Cloud 的 Agent Platform
>
> 在您的 Google Cloud 项目中创建联合资源，遵循 [Workload Identity Federation 文档](https://cloud.google.com/iam/docs/workload-identity-federation)：
>
> * 启用三个 API：IAM Credentials、Security Token Service (STS) 和 Agent Platform API，其服务名称为 `aiplatform.googleapis.com`
> * 创建一个 Workload Identity Pool，其中包含一个 GitHub OIDC 提供商，其颁发者为 `https://token.actions.githubusercontent.com`，并添加一个属性条件，将池限制在您的存储库
> * 创建一个仅具有 `Vertex AI User` 角色（即 `roles/aiplatform.user`）的专用服务账户，并允许池模拟它
>
> 记下提供商的完整资源名称和服务账户的电子邮件地址。您将在下一步中将它们添加为密钥。

> [tab] Microsoft Foundry
>
> 创建一个 Microsoft Entra 应用程序，其中包含您的存储库的联合凭证，遵循 [Microsoft 的 GitHub Actions 身份验证指南](https://learn.microsoft.com/en-us/azure/developer/github/connect-from-azure-openid-connect)：
>
> * 注册一个 Microsoft Entra 应用程序，并添加一个联合身份凭证，该凭证信任 GitHub 向您的存储库颁发的令牌。用户分配的托管身份可以代替应用程序。两者都有您在下面记下的客户端 ID
> * 在您的 Foundry 资源上为应用程序分配 `Azure AI User` 角色。有关更窄的自定义角色，请参阅 [Azure RBAC 配置](https://code.claude.com/docs/zh-CN/microsoft-foundry.md#azure-rbac-configuration)
>
> 记下应用程序的客户端 ID、您的租户 ID 和您的订阅 ID。您将在下一步中将它们添加为密钥。

**3. 添加存储库密钥**

在运行 Claude Code GitHub Action 的存储库中，为您的提供商添加密钥，如果您在第一步中创建了自定义 GitHub App，还要添加两个应用密钥。请参阅 GitHub 的 [在 GitHub Actions 中使用密钥指南](https://docs.github.com/en/actions/security-guides/using-secrets-in-github-actions)。

| 密钥 | 需要用于 | 值 |
| - | - | - |
| `AWS_ROLE_TO_ASSUME` | Amazon Bedrock | IAM 角色的 ARN |
| `GCP_WORKLOAD_IDENTITY_PROVIDER` | Google Cloud 的 Agent Platform | 提供商的完整资源名称 |
| `GCP_SERVICE_ACCOUNT` | Google Cloud 的 Agent Platform | 服务账户的电子邮件地址 |
| `AZURE_CLIENT_ID` | Microsoft Foundry | Entra 应用程序的客户端 ID |
| `AZURE_TENANT_ID` | Microsoft Foundry | 您的 Microsoft Entra 租户 ID |
| `AZURE_SUBSCRIPTION_ID` | Microsoft Foundry | 您的 Azure 订阅 ID |
| `APP_ID` | 自定义 GitHub App | GitHub App 的 ID |
| `APP_PRIVATE_KEY` | 自定义 GitHub App | `.pem` 私钥文件的内容 |

**4. 创建工作流文件**

为您的提供商创建一个工作流文件，例如 `.github/workflows/claude.yml`。每个示例都响应 `@claude` 提及，使用自定义应用向 GitHub 进行身份验证，并包含 `id-token: write` 权限，GitHub 需要此权限来颁发 OIDC 令牌，您的云提供商可以用它来交换凭证。

如果您在第一步中选择了不同的 GitHub 身份，请调整示例：

* **官方 Claude GitHub App**: 删除生成 GitHub App 令牌步骤和 `github_token` 行
* **GitHub 的自动令牌**: 删除令牌生成步骤，并将 `github_token` 行更改为 `github_token: ${{ secrets.GITHUB_TOKEN }}`

> [!WARNING]
>
> 在公共存储库上，来自任何用户的包含触发短语的评论会启动此工作流。凭证步骤在 Claude Code GitHub Action 检查评论者的写入访问权限之前运行，因此该操作仅在工作流生成应用令牌并登录到您的云提供商后才拒绝未授权用户，这会留下审计日志条目并消耗 Actions 分钟。为了避免这些运行，请添加一个步骤，在凭证步骤之前验证评论者的写入访问权限。

> [tab] Amazon Bedrock
>
> 将 `aws-region` 值替换为您自己的值。凭证步骤将其导出为 `AWS_REGION` 供作业的其余部分使用。
>
> ```yaml theme={null}
> name: Claude PR Action
>
> permissions:
>   contents: write
>   pull-requests: write
>   issues: write
>   id-token: write
>
> on:
>   issue_comment:
>     types: [created]
>   pull_request_review_comment:
>     types: [created]
>   issues:
>     types: [opened]
>
> jobs:
>   claude-pr:
>     if: |
>       (github.event_name == 'issue_comment' && contains(github.event.comment.body, '@claude')) ||
>       (github.event_name == 'pull_request_review_comment' && contains(github.event.comment.body, '@claude')) ||
>       (github.event_name == 'issues' && (contains(github.event.issue.body, '@claude') || contains(github.event.issue.title, '@claude')))
>     runs-on: ubuntu-latest
>     steps:
>       - name: Checkout repository
>         uses: actions/checkout@v6
>
>       - name: Generate GitHub App token
>         id: app-token
>         uses: actions/create-github-app-token@v2
>         with:
>           app-id: ${{ secrets.APP_ID }}
>           private-key: ${{ secrets.APP_PRIVATE_KEY }}
>
>       - name: Configure AWS Credentials (OIDC)
>         uses: aws-actions/configure-aws-credentials@v4
>         with:
>           role-to-assume: ${{ secrets.AWS_ROLE_TO_ASSUME }}
>           aws-region: us-west-2
>
>       - uses: anthropics/claude-code-action@v1
>         with:
>           github_token: ${{ steps.app-token.outputs.token }}
>           use_bedrock: "true"
>           claude_args: '--model us.anthropic.claude-sonnet-4-6'
> ```
>
> > [!TIP]
> >
> > Bedrock 模型 ID 包含跨区域推理配置文件前缀，例如 `us.`。使用您授予模型访问权限的区域组的前缀。

> [tab] Google Cloud 的 Agent Platform
>
> 将 `CLOUD_ML_REGION` 值替换为您自己的值。您无需硬编码项目 ID，因为工作流从 `auth` 步骤的输出中读取它。
>
> ```yaml theme={null}
> name: Claude PR Action
>
> permissions:
>   contents: write
>   pull-requests: write
>   issues: write
>   id-token: write
>
> on:
>   issue_comment:
>     types: [created]
>   pull_request_review_comment:
>     types: [created]
>   issues:
>     types: [opened]
>
> jobs:
>   claude-pr:
>     if: |
>       (github.event_name == 'issue_comment' && contains(github.event.comment.body, '@claude')) ||
>       (github.event_name == 'pull_request_review_comment' && contains(github.event.comment.body, '@claude')) ||
>       (github.event_name == 'issues' && (contains(github.event.issue.body, '@claude') || contains(github.event.issue.title, '@claude')))
>     runs-on: ubuntu-latest
>     steps:
>       - name: Checkout repository
>         uses: actions/checkout@v6
>
>       - name: Generate GitHub App token
>         id: app-token
>         uses: actions/create-github-app-token@v2
>         with:
>           app-id: ${{ secrets.APP_ID }}
>           private-key: ${{ secrets.APP_PRIVATE_KEY }}
>
>       - name: Authenticate to Google Cloud
>         id: auth
>         uses: google-github-actions/auth@v2
>         with:
>           workload_identity_provider: ${{ secrets.GCP_WORKLOAD_IDENTITY_PROVIDER }}
>           service_account: ${{ secrets.GCP_SERVICE_ACCOUNT }}
>
>       - uses: anthropics/claude-code-action@v1
>         with:
>           github_token: ${{ steps.app-token.outputs.token }}
>           use_vertex: "true"
>           claude_args: '--model claude-sonnet-5'
>         env:
>           ANTHROPIC_VERTEX_PROJECT_ID: ${{ steps.auth.outputs.project_id }}
>           CLOUD_ML_REGION: us-east5
> ```

> [tab] Microsoft Foundry
>
> 将 `your-resource-name` 替换为您的 Foundry 资源名称。Claude Code 从它构建端点 URL。`azure/login` 步骤使用工作流的 OIDC 令牌登录，Claude Code 通过 Azure [默认凭证链](https://learn.microsoft.com/en-us/azure/developer/javascript/sdk/authentication/credential-chains#defaultazurecredential-overview) 获取凭证。
>
> ```yaml theme={null}
> name: Claude PR Action
>
> permissions:
>   contents: write
>   pull-requests: write
>   issues: write
>   id-token: write
>
> on:
>   issue_comment:
>     types: [created]
>   pull_request_review_comment:
>     types: [created]
>   issues:
>     types: [opened]
>
> jobs:
>   claude-pr:
>     if: |
>       (github.event_name == 'issue_comment' && contains(github.event.comment.body, '@claude')) ||
>       (github.event_name == 'pull_request_review_comment' && contains(github.event.comment.body, '@claude')) ||
>       (github.event_name == 'issues' && (contains(github.event.issue.body, '@claude') || contains(github.event.issue.title, '@claude')))
>     runs-on: ubuntu-latest
>     steps:
>       - name: Checkout repository
>         uses: actions/checkout@v6
>
>       - name: Generate GitHub App token
>         id: app-token
>         uses: actions/create-github-app-token@v2
>         with:
>           app-id: ${{ secrets.APP_ID }}
>           private-key: ${{ secrets.APP_PRIVATE_KEY }}
>
>       - name: Authenticate to Azure
>         uses: azure/login@v2
>         with:
>           client-id: ${{ secrets.AZURE_CLIENT_ID }}
>           tenant-id: ${{ secrets.AZURE_TENANT_ID }}
>           subscription-id: ${{ secrets.AZURE_SUBSCRIPTION_ID }}
>
>       - uses: anthropics/claude-code-action@v1
>         with:
>           github_token: ${{ steps.app-token.outputs.token }}
>           use_foundry: "true"
>           claude_args: '--model claude-sonnet-5'
>         env:
>           ANTHROPIC_FOUNDRY_RESOURCE: your-resource-name
> ```
>
> > [!TIP]
> >
> > 使用与您的 Foundry 资源中的 Claude 部署相匹配的模型 ID。有关模型配置和版本固定，请参阅 [Microsoft Foundry 上的 Claude Code](https://code.claude.com/docs/zh-CN/microsoft-foundry.md)。

对于任何提供商，您可以通过将 `--max-turns` 添加到 `claude_args` 来限制运行长度和成本。请参阅 [管理成本](https://code.claude.com/docs/zh-CN/github-actions.md#manage-costs)。

**5. 测试设置**

在问题或 PR 评论中提及 `@claude`，然后在存储库的 Actions 选项卡中观看运行。Claude 在同一问题或 PR 上的评论中回复。

## 故障排除

失败的运行通常在以下两个地方之一中断：

* **身份验证错误**: 通常是 OIDC 配置错误。检查工作流是否包含 `id-token: write` 权限，信任配置的存储库条件是否与您的存储库完全匹配，以及工作流中的密钥名称是否与您添加的名称匹配
* **触发和 CI 问题**: 这些的行为与 Claude Code GitHub Action 调用 Claude API 时相同。请参阅主页的[故障排除部分](https://code.claude.com/docs/zh-CN/github-actions.md#troubleshooting)和 Claude Code GitHub Action 的 [FAQ](https://github.com/anthropics/claude-code-action/blob/main/docs/faq.md)

## 接下来

* [Claude Code GitHub Actions](https://code.claude.com/docs/zh-CN/github-actions.md) 获取示例、参数和最佳实践
* [Amazon Bedrock 上的 Claude Code](https://code.claude.com/docs/zh-CN/amazon-bedrock.md) 获取 Bedrock 模型 ID 和区域
* [Google Cloud 的 Agent Platform 上的 Claude Code](https://code.claude.com/docs/zh-CN/google-vertex-ai.md) 获取 Agent Platform 模型 ID 和区域
* [Microsoft Foundry 上的 Claude Code](https://code.claude.com/docs/zh-CN/microsoft-foundry.md) 获取 Foundry 模型和端点配置
