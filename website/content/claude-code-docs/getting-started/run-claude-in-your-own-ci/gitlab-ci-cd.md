> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Claude Code GitLab CI/CD

> 了解如何将 Claude Code 集成到您的 GitLab CI/CD 开发工作流中

> [!NOTE]
>
> Claude Code for GitLab CI/CD 目前处于测试阶段。随着我们完善体验，功能和特性可能会发生变化。
>
> 此集成由 GitLab 维护。如需支持，请参阅以下 [GitLab issue](https://gitlab.com/gitlab-org/gitlab/-/issues/573776)。

> [!NOTE]
>
> 此集成基于 [Claude Code CLI and Agent SDK](https://code.claude.com/docs/zh-CN/agent-sdk/overview.md) 构建，可在您的 CI/CD 作业和自定义自动化工作流中以编程方式使用 Claude。

## 为什么在 GitLab 中使用 Claude Code？

* **即时 MR 创建**：描述您的需求，Claude 会提议一个完整的 MR，包含更改和说明
* **自动化实现**：使用单个命令或提及将问题转化为可工作的代码
* **项目感知**：Claude 遵循您的 `CLAUDE.md` 指南和现有代码模式
* **简单设置**：向 `.gitlab-ci.yml` 添加一个作业和一个掩码 CI/CD 变量
* **企业就绪**：选择 Claude API、Amazon Bedrock 或 Google Cloud 的 Agent Platform 以满足数据驻留和采购需求
* **默认安全**：在您的 GitLab runners 中运行，具有您的分支保护和批准

## 工作原理

Claude Code 使用 GitLab CI/CD 在隔离的作业中运行 AI 任务，并通过 MR 将结果提交回来：

1. **事件驱动的编排**：GitLab 监听您选择的触发器（例如，在问题、MR 或审查线程中提及 `@claude` 的评论）。该作业从线程和存储库收集上下文，从该输入构建提示，并运行 Claude Code。

2. **提供商抽象**：使用适合您环境的提供商：
   * Claude API (SaaS)
   * Amazon Bedrock（基于 IAM 的访问、跨区域选项）
   * Google Cloud 的 Agent Platform（GCP 原生、Workload Identity Federation）

3. **沙箱执行**：每次交互都在具有严格网络和文件系统规则的容器中运行。Claude Code 强制执行工作区范围的权限以限制写入。每项更改都通过 MR 流动，以便审查者可以看到差异，批准仍然适用。

选择区域端点以降低延迟并满足数据主权要求，同时使用现有的云协议。

## Claude 可以做什么？

在 GitLab 管道中，Claude Code 可以：

* 从问题描述或评论创建和更新 MR
* 分析性能回归并提出优化建议
* 直接在分支中实现功能，然后打开 MR
* 修复由测试或评论识别的错误和回归
* 回复后续评论以迭代所请求的更改

## 设置

### 快速设置

开始使用的最快方式是在 `.gitlab-ci.yml` 中添加一个最小化的任务，并将您的 API 密钥设置为掩码变量。

1. **添加掩码 CI/CD 变量**
   * 转到 **Settings** → **CI/CD** → **Variables**
   * 添加 `ANTHROPIC_API_KEY`（掩码，根据需要保护）

2. **将 Claude 任务添加到 `.gitlab-ci.yml`**

```yaml theme={null}
stages:
  - ai

claude:
  stage: ai
  image: node:24-alpine3.21
  # 调整规则以适应您想要触发任务的方式：
  # - 手动运行
  # - 合并请求事件
  # - 当评论包含 '@claude' 时的 web/API 触发
  rules:
    - if: '$CI_PIPELINE_SOURCE == "web"'
    - if: '$CI_PIPELINE_SOURCE == "merge_request_event"'
  variables:
    GIT_STRATEGY: fetch
  before_script:
    - apk update
    - apk add --no-cache git curl bash
    - curl -fsSL https://claude.ai/install.sh | bash
    # 安装程序将 claude 放在 ~/.local/bin 中，在此镜像中不在 PATH 上
    - export PATH="$HOME/.local/bin:$PATH"
  script:
    # 可选：如果您的设置提供了 GitLab MCP 服务器，请启动它
    - /bin/gitlab-mcp-server || true
    # 通过带有上下文负载的 web/API 触发时使用 AI_FLOW_* 变量
    - echo "$AI_FLOW_INPUT for $AI_FLOW_CONTEXT on $AI_FLOW_EVENT"
    - >
      claude
      -p "${AI_FLOW_INPUT:-'Review this MR and implement the requested changes'}"
      --permission-mode acceptEdits
      --allowedTools "Bash Read Edit Write mcp__gitlab"
      --debug
```

添加任务和 `ANTHROPIC_API_KEY` 变量后，通过从 **CI/CD** → **Pipelines** 手动运行任务进行测试，或从 MR 触发它以让 Claude 在分支中提议更新并在需要时打开 MR。

> [!NOTE]
>
> 要在 Amazon Bedrock 或 Google Cloud 的 Agent Platform 上运行而不是 Claude API，请参阅下面的 [Using with Amazon Bedrock and Google Cloud](#using-with-amazon-bedrock-and-google-cloud) 部分了解身份验证和环境设置。

### 手动设置（建议用于生产环境）

如果您更喜欢更受控的设置或需要企业提供商：

1. **配置提供商访问**：
   * **Claude API**：创建 `ANTHROPIC_API_KEY` 并将其存储为掩码 CI/CD 变量
   * **Amazon Bedrock**：**Configure GitLab** → **AWS OIDC** 并为 Amazon Bedrock 创建 IAM 角色
   * **Google Cloud 的 Agent Platform**：**Configure Workload Identity Federation for GitLab** → **GCP**

2. **为 GitLab API 操作添加项目凭证**：
   * 默认使用 `CI_JOB_TOKEN`，或创建具有 `api` 范围的项目访问令牌
   * 如果使用 PAT，将其存储为 `GITLAB_ACCESS_TOKEN`（掩码）

3. **将 Claude 任务添加到 `.gitlab-ci.yml`**：对于 Claude API 使用 [Quick setup](#quick-setup) 任务，或从 [Configuration examples](#configuration-examples) 使用提供商任务

4. **（可选）启用提及驱动的触发器**：
   * 为"Comments (notes)"添加项目 webhook 到您的事件监听器（如果您使用的话）
   * 当评论包含 `@claude` 时，让监听器使用 `AI_FLOW_INPUT` 和 `AI_FLOW_CONTEXT` 等变量调用管道触发 API

## 示例用例

### 将问题转换为 MR

在问题评论中：

```text wrap theme={null}
@claude implement this feature based on the issue description
```

Claude 分析问题和代码库，在分支中编写更改，并打开 MR 供审查。

### 获取实现帮助

在 MR 讨论中：

```text wrap theme={null}
@claude suggest a concrete approach to cache the results of this API call
```

Claude 提出更改建议，添加具有适当缓存的代码，并更新 MR。

### 快速修复错误

在问题或 MR 评论中：

```text wrap theme={null}
@claude fix the TypeError in the user dashboard component
```

Claude 定位错误，实现修复，并更新分支或打开新的 MR。

## 与 Amazon Bedrock 和 Google Cloud 配合使用

对于企业环境，您可以在自己的云基础设施上完全运行 Claude Code，并获得相同的开发者体验。

> [tab] Amazon Bedrock
>
> ### 前置条件
>
> 在使用 Amazon Bedrock 设置 Claude Code 之前，您需要：
>
> 1. 一个 AWS 账户，具有对所需 Claude 模型的 Amazon Bedrock 访问权限
> 2. 在 AWS IAM 中配置为 OIDC 身份提供商的 GitLab
> 3. 一个具有 Amazon Bedrock 权限的 IAM 角色和限制为您的 GitLab 项目/引用的信任策略
> 4. 用于角色假设的 GitLab CI/CD 变量：
>    * `AWS_ROLE_TO_ASSUME`（角色 ARN）
>    * `AWS_REGION`（Amazon Bedrock 区域）
>
> ### 设置说明
>
> 配置 AWS 以允许 GitLab CI 作业通过 OIDC 假设 IAM 角色（无静态密钥）。
>
> **必需的设置：**
>
> 1. 启用 Amazon Bedrock 并请求访问您的目标 Claude 模型
> 2. 为 GitLab 创建 IAM OIDC 提供商（如果尚未存在）
> 3. 创建由 GitLab OIDC 提供商信任的 IAM 角色，限制为您的项目和受保护的引用
> 4. 为 Amazon Bedrock 调用 API 附加最小权限
>
> 使用 [Amazon Bedrock 作业示例](#configuration-examples) 在运行时将作业的 OIDC 令牌交换为临时 AWS 凭证。

> [tab] Google Cloud's Agent Platform
>
> ### 前置条件
>
> 在使用 Google Cloud's Agent Platform 设置 Claude Code 之前，您需要：
>
> 1. 一个 Google Cloud 项目，具有：
>    * 启用的 Google Cloud's Agent Platform API
>    * 配置为信任 GitLab OIDC 的工作负载身份联合
> 2. 一个仅具有所需 Google Cloud's Agent Platform 角色的专用服务账户
> 3. GitLab CI/CD 变量：
>    * `GCP_WORKLOAD_IDENTITY_PROVIDER`（提供商资源名称，不包括 `//iam.googleapis.com/` 前缀，例如 `projects/123456789/locations/global/workloadIdentityPools/my-pool/providers/my-provider`）
>    * `GCP_SERVICE_ACCOUNT`（服务账户电子邮件）
>    * `GCP_PROJECT_ID`（Google Cloud 项目 ID）
>
> ### 设置说明
>
> 配置 Google Cloud 以允许 GitLab CI 作业通过工作负载身份联合模拟服务账户。
>
> **必需的设置：**
>
> 1. 启用 IAM Credentials API、STS API 和 Google Cloud's Agent Platform API
> 2. 为 GitLab OIDC 创建工作负载身份池和提供商
> 3. 创建具有 Google Cloud's Agent Platform 角色的专用服务账户
> 4. 授予 WIF 主体权限以模拟服务账户
>
> 使用 [Agent Platform 作业示例](#configuration-examples) 进行身份验证，无需存储密钥。

## 配置示例

以下是可以适配到您的管道中的现成代码片段。

### Amazon Bedrock 任务示例 (OIDC)

**前置条件：**

* Amazon Bedrock 已启用并可访问您选择的 Claude 模型
* GitLab OIDC 在 AWS 中已配置，具有信任您的 GitLab 项目和 refs 的角色
* 具有 Amazon Bedrock 权限的 IAM 角色（建议使用最小权限）

**必需的 CI/CD 变量：**

* `AWS_ROLE_TO_ASSUME`：用于 Amazon Bedrock 访问的 IAM 角色的 ARN
* `AWS_REGION`：Amazon Bedrock 区域（例如，`us-west-2`）

GitLab 从 `id_tokens:` 块中生成任务的 OIDC 令牌，并将其公开为 `GITLAB_OIDC_TOKEN`。将 `aud` 设置为您在 AWS 的 IAM OIDC 身份提供商上配置的受众值，例如您的 GitLab 实例 URL。

```yaml theme={null}
stages:
  - ai

claude-bedrock:
  stage: ai
  image: node:24-alpine3.21
  rules:
    - if: '$CI_PIPELINE_SOURCE == "web"'
  id_tokens:
    GITLAB_OIDC_TOKEN:
      aud: https://gitlab.example.com
  before_script:
    - apk add --no-cache bash curl jq git aws-cli
    - curl -fsSL https://claude.ai/install.sh | bash
    # The installer places claude in ~/.local/bin, which isn't on PATH in this image
    - export PATH="$HOME/.local/bin:$PATH"
    # Exchange the job's OIDC token for AWS credentials
    - export AWS_WEB_IDENTITY_TOKEN_FILE="/tmp/oidc_token"
    - printf "%s" "$GITLAB_OIDC_TOKEN" > "$AWS_WEB_IDENTITY_TOKEN_FILE"
    - >
      aws sts assume-role-with-web-identity
      --role-arn "$AWS_ROLE_TO_ASSUME"
      --role-session-name "gitlab-claude-$(date +%s)"
      --web-identity-token "file://$AWS_WEB_IDENTITY_TOKEN_FILE"
      --duration-seconds 3600 > /tmp/aws_creds.json
    - export AWS_ACCESS_KEY_ID="$(jq -r .Credentials.AccessKeyId /tmp/aws_creds.json)"
    - export AWS_SECRET_ACCESS_KEY="$(jq -r .Credentials.SecretAccessKey /tmp/aws_creds.json)"
    - export AWS_SESSION_TOKEN="$(jq -r .Credentials.SessionToken /tmp/aws_creds.json)"
  script:
    - /bin/gitlab-mcp-server || true
    - >
      claude
      -p "${AI_FLOW_INPUT:-'Implement the requested changes and open an MR'}"
      --permission-mode acceptEdits
      --allowedTools "Bash Read Edit Write mcp__gitlab"
      --debug
  variables:
    AWS_REGION: "us-west-2"
    CLAUDE_CODE_USE_BEDROCK: "1"
```

> [!NOTE]
>
> Amazon Bedrock 的模型 ID 包含特定于区域的前缀（例如，`us.anthropic.claude-sonnet-4-6`）。通过您的任务配置或提示传递所需的模型，如果您的工作流支持的话。

### Agent Platform 任务示例 (Workload Identity Federation)

**前置条件：**

* Google Cloud 的 Agent Platform API 在您的 GCP 项目中已启用
* Workload Identity Federation 已配置为信任 GitLab OIDC
* 具有 Google Cloud 的 Agent Platform 权限的服务账户

**必需的 CI/CD 变量：**

* `GCP_WORKLOAD_IDENTITY_PROVIDER`：提供商资源名称，不包括 `//iam.googleapis.com/` 前缀，例如 `projects/123456789/locations/global/workloadIdentityPools/my-pool/providers/my-provider`
* `GCP_SERVICE_ACCOUNT`：服务账户电子邮件
* `GCP_PROJECT_ID`：Google Cloud 项目 ID
* `CLOUD_ML_REGION`：Google Cloud 的 Agent Platform 区域（例如，`us-east5`）

GitLab 从 `id_tokens:` 块中生成任务的 OIDC 令牌，并将其公开为 `GITLAB_OIDC_TOKEN`。将 `aud` 设置为您在 Workload Identity Pool 提供商上配置的受众值，例如您的 GitLab 实例 URL。任务将令牌写入文件，凭证配置的 `credential_source` 条目告诉 Google 的身份验证库从那里读取它。将 `GOOGLE_APPLICATION_CREDENTIALS` 设置为凭证配置文件使其可通过[应用默认凭证](https://code.claude.com/docs/zh-CN/google-vertex-ai.md#3-configure-gcp-credentials)供 Claude Code 使用。

```yaml theme={null}
stages:
  - ai

claude-vertex:
  stage: ai
  image: gcr.io/google.com/cloudsdktool/google-cloud-cli:slim
  rules:
    - if: '$CI_PIPELINE_SOURCE == "web"'
  id_tokens:
    GITLAB_OIDC_TOKEN:
      aud: https://gitlab.example.com
  before_script:
    - apt-get update && apt-get install -y git && apt-get clean
    - curl -fsSL https://claude.ai/install.sh | bash
    # The installer places claude in ~/.local/bin, which isn't on PATH in this image
    - export PATH="$HOME/.local/bin:$PATH"
    # Write the job's OIDC token where credential_source expects it
    - printf "%s" "$GITLAB_OIDC_TOKEN" > /tmp/oidc_token
    # Write the WIF credential configuration to a file (no downloaded keys)
    - |
      cat > /tmp/cred.json <<EOF
      {
        "type": "external_account",
        "audience": "//iam.googleapis.com/${GCP_WORKLOAD_IDENTITY_PROVIDER}",
        "subject_token_type": "urn:ietf:params:oauth:token-type:jwt",
        "token_url": "https://sts.googleapis.com/v1/token",
        "credential_source": {
          "file": "/tmp/oidc_token"
        },
        "service_account_impersonation_url": "https://iamcredentials.googleapis.com/v1/projects/-/serviceAccounts/${GCP_SERVICE_ACCOUNT}:generateAccessToken"
      }
      EOF
    # Expose the credentials to Claude Code via Application Default Credentials
    - export GOOGLE_APPLICATION_CREDENTIALS=/tmp/cred.json
    # Authenticate the gcloud CLI with the same credential configuration
    - gcloud auth login --cred-file=/tmp/cred.json
    - gcloud config set project "$GCP_PROJECT_ID"
  script:
    - /bin/gitlab-mcp-server || true
    - >
      CLOUD_ML_REGION="${CLOUD_ML_REGION:-us-east5}"
      claude
      -p "${AI_FLOW_INPUT:-'Review and update code as requested'}"
      --permission-mode acceptEdits
      --allowedTools "Bash Read Edit Write mcp__gitlab"
      --debug
  variables:
    CLOUD_ML_REGION: "us-east5"
    CLAUDE_CODE_USE_VERTEX: "1"
    ANTHROPIC_VERTEX_PROJECT_ID: "$GCP_PROJECT_ID"
```

> [!NOTE]
>
> 使用 Workload Identity Federation，您无需存储服务账户密钥。使用特定于存储库的信任条件和最小权限服务账户。

## 最佳实践

### CLAUDE.md 配置

在存储库根目录创建一个 `CLAUDE.md` 文件，以定义编码标准、审查标准和项目特定的规则。Claude 在运行期间读取此文件，并在提议更改时遵循您的约定。

### 安全考虑

**永远不要将 API 密钥或云凭证提交到您的存储库**。始终使用 GitLab CI/CD 变量：

* 将 `ANTHROPIC_API_KEY` 添加为掩码变量（如果需要，请保护它）
* 尽可能使用提供商特定的 OIDC（无长期密钥）
* 限制作业权限和网络出口
* 像审查任何其他贡献者一样审查 Claude 的 MR

### 优化性能

* 保持 `CLAUDE.md` 专注且简洁
* 提供清晰的问题/MR 描述以减少迭代
* 在可能的情况下在运行器中缓存 npm 和包安装

### CI 成本

使用 Claude Code 与 GitLab CI/CD 时，请注意相关成本：

* **GitLab Runner 时间**：
  * Claude 在您的 GitLab 运行器上运行并消耗计算分钟数
  * 有关详细信息，请参阅您的 GitLab 计划的运行器计费

* **API 成本**：
  * 每次 Claude 交互根据提示和响应大小消耗令牌
  * 令牌使用量因任务复杂性和代码库大小而异
  * 有关详细信息，请参阅 [Anthropic 定价](https://platform.claude.com/docs/en/about-claude/pricing)

* **成本优化提示**：
  * 使用特定的 `@claude` 命令以减少不必要的轮次
  * 设置适当的 `--max-turns` 和作业 `timeout` 值
  * 限制并发以控制并行运行

## 故障排除

### Claude 不响应 @claude 命令

* 验证您的管道是否被触发（手动、MR 事件或通过注释事件监听器/webhook）
* 确保您的 `ANTHROPIC_API_KEY` 或云提供商变量存在
* 检查注释是否包含 `@claude`（不是 `/claude`）以及您的提及触发器是否已配置

### 作业无法写入注释或打开 MR

* 确保 `CI_JOB_TOKEN` 对项目具有足够的权限，或使用具有 `api` 范围的项目访问令牌
* 检查 `mcp__gitlab` 工具是否在 `--allowedTools` 中启用
* 确认作业在 MR 的上下文中运行或通过 `AI_FLOW_*` 变量具有足够的上下文

### 身份验证错误

* **对于 Claude API**：确认 `ANTHROPIC_API_KEY` 有效且未过期
* **对于 Amazon Bedrock 或 Google Cloud 的 Agent Platform**：验证 OIDC/WIF 配置、角色模拟和密钥名称；确认区域和模型可用性

## 高级配置

### 常见参数和变量

使用这些 CLI 标志、GitLab 关键字和变量来控制你的作业中的 Claude Code 运行：

* `-p`：内联提供指令，例如 `claude -p "Review this MR"`
* `--max-turns`：限制往返迭代的次数
* `timeout`：使用 GitLab 的作业级 `timeout` 关键字限制总作业执行时间，例如 `timeout: 30m`
* `ANTHROPIC_API_KEY`：Claude API 所需（不用于 Amazon Bedrock 或 Google Cloud 的 Agent Platform）
* 提供商特定环境：`AWS_REGION`、Google Cloud 的 Agent Platform 的项目/区域变量

> [!NOTE]
>
> 确切的标志和参数可能因 `@anthropic-ai/claude-code` 的版本而异。在你的作业中运行 `claude --help` 以查看支持的选项。

### 自定义 Claude 的行为

你可以通过两种主要方式指导 Claude：

1. **CLAUDE.md**：定义编码标准、安全要求和项目约定。Claude 在运行期间读取此文件并遵循你的规则。
2. **自定义提示**：通过作业中的 `-p` 传递特定任务的指令。为不同的作业使用不同的提示（例如，审查、实现、重构）。
