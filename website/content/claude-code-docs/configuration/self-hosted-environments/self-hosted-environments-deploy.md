> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# 将自托管环境部署到生产环境

> 在生产环境中运行自托管运行器：安全加固、网络出站流量控制、git 凭证、Kubernetes 和 Compose 配方以及故障排除。

> [!NOTE]
>
> 自托管环境在 Team 和 Enterprise 计划上处于公开测试阶段；[可用性和限制](https://code.claude.com/docs/zh-CN/self-hosted-environments#availability-and-limitations)涵盖启用路径。本页面涵盖在生产环境中运行队列；有关首个运行器和会话，请参阅[快速入门](https://code.claude.com/docs/zh-CN/self-hosted-environments-quickstart.md)。

[自托管环境](https://code.claude.com/docs/zh-CN/self-hosted-environments)在您部署在网络内的运行器上运行 Claude Code [云会话](https://code.claude.com/docs/zh-CN/claude-code-on-the-web.md)，在生产环境中，这些会话代表所有可以向环境分派会话的人执行模型指导的代码。本页面适用于将工作环境投入生产的操作员。它按部署顺序进行：在连接真实系统之前要锁定什么、队列需要的出站流量、会话如何向您的 git 主机进行身份验证、部署配方本身，以及会话出现故障时要检查什么。

## 加固您的部署

自托管运行器代表所有可以向其环境分派会话的人在您的基础设施上执行任意的、模型指导的代码。这是您 Anthropic 组织的任何成员，以及任何可以在所有者路由到环境的范围内启动 [Claude Tag](https://claude.com/docs/claude-tag/overview) 频道会话的人。在将环境连接到生产系统之前，请逐项完成以下操作：

* **临时的、按会话的容器**：在新容器或 VM 中运行每个运行器进程，该容器或 VM 在进程退出时被销毁，使用 `--capacity 1` 和默认的 `--drain-grace-sec 0`，以便每个容器恰好服务一个会话。在更高的容量或正的 drain grace 下，一个容器为来自同一[锁定所有者](https://code.claude.com/docs/zh-CN/self-hosted-environments#key-concepts)的多个会话服务；请参阅[运行器生命周期](https://code.claude.com/docs/zh-CN/self-hosted-environments#runner-lifecycle)。不要在运行器重启之间重用文件系统，除了在刻意的[预热检出](#reuse-a-pre-warmed-checkout)设置中，并且永远不要跨所有者。
  * 当运行器停止会话时，它不会向在其 shell 命令退出后仍在运行的进程（例如已转为守护进程的服务）发送任何信号。销毁容器或 VM 会结束该进程。
* **镜像中没有广泛的凭据**：不要包含长期的 SSH 密钥、云提供商凭据或授予超过会话需要的个人访问令牌。从您的[包装脚本](https://code.claude.com/docs/zh-CN/self-hosted-environments-configuration.md#wrapper-scripts)按会话铸造会话期间使用的凭据，例如推送或 API 令牌。初始克隆发生在包装脚本运行之前，因此请使用 [`checkout` 生命周期 hook](https://code.claude.com/docs/zh-CN/self-hosted-environments-configuration#checkout) 处理它，或者在会话的所有仓库都位于 github.com 上时使用 [`--use-anthropic-git-proxy`](#use-the-anthropic-git-proxy)。关于这两者，请参阅[配置 git](#configure-git)。
* **使主机的 GitHub 凭据远离会话**：Claude 可以使用会话能够读取的任何 GitHub 凭据，并拥有该凭据授予的全部访问权限。请确保运行器主机自身的宽范围 GitHub 凭据不出现在会话可以读取的任何位置。此类凭据可以是个人访问令牌、`gh auth login` 为您的帐户保存的令牌，或运行器环境中的 `GH_TOKEN`。
  * **使用 [Anthropic 托管的 git](#use-the-anthropic-git-proxy) 时**：有了此类凭据，Claude 会直接访问 GitHub，而不是通过 Anthropic 托管的 git。
  * **不使用 Anthropic 托管的 git 时**：如果您按照[在镜像中附带 git 配置](#ship-git-config-in-your-image)所述严格限定克隆凭据的范围，则该凭据可以保留在镜像中。
* **将环境密钥保持在运行会话的主机之外**：环境密钥可以注册运行器并获取在环境上排队的任何会话。在固定队列上，它存在于每个运行器主机上，任何会话的代码都可以读取密钥文件。优先使用[按需运行器](https://code.claude.com/docs/zh-CN/self-hosted-environments-configuration.md#on-demand-runners)，其中密钥保留在编排器主机上，该主机从不运行用户代码，每个运行器接收单次使用的工作单，恰好注册一个运行器。在固定队列上，将环境密钥文件视为可由每个会话读取，并在任何可疑会话泄露后轮换密钥。
* **默认拒绝网络出站流量**：在每个环境上限制运行器和会话容器的出站流量在您自己的网络边界；[默认拒绝出站流量](#default-deny-egress)涵盖允许什么以及原因。
* **最小权限主机 IAM**：附加到运行器主机的计算身份（例如实例配置文件或节点服务帐户）应仅授予运行器本身需要的内容。会话应通过您的包装脚本而不是继承主机的身份获取自己的凭证。
* **阻止会话访问云元数据端点**：保持会话不访问主机身份需要阻止它们访问云元数据端点，子网级出站策略不会拦截链接本地元数据流量，因此在容器本身中阻止它：

  * IMDSv2，跳数限制为 1
  * GKE Workload Identity，隐藏元数据
  * 会话容器网络命名空间中 `169.254.169.254` 的显式拒绝

  该块也适用于您的包装脚本和生命周期钩子，因为它们共享容器。使用[会话 JWT](https://code.claude.com/docs/zh-CN/self-hosted-environments-identity.md)针对您自己的令牌服务通过允许列表出站流量验证任何令牌交换，或使用基于文件的 Web 身份，例如 Amazon EKS 上的 IAM Roles for Service Accounts (IRSA)。
* **按运行器文件系统隔离**：每个运行器进程获得自己的工作目录，主机上的其他进程无法读取或写入。使 `--hooks-dir`、包装脚本和主机的 `~/.claude/` 对会话只读，无论是内置在镜像中还是以只读方式挂载。
* **分派没有按环境的访问控制**：您 Anthropic 组织的任何成员都可以向其任何环境分派会话。如果所有者[将 Claude Tag 频道路由到环境](https://code.claude.com/docs/zh-CN/cloud-environments.md#set-the-environment-a-claude-tag-channel-uses)，[Claude Tag 访问设置](https://claude.com/docs/claude-tag/admins/restrict-access#restrict-who-can-use-claude)允许的任何人都可以启动在那里运行的频道会话。默认情况下，这是连接的 Slack 工作区中的任何人，无论是否有 Claude 帐户。将每个运行器主机视为可由所有可以向其分派的人访问以执行代码，并仅在运行器主机上放置所有这些人都被允许读取的数据和凭证。[`--lock-to-account`](https://code.claude.com/docs/zh-CN/self-hosted-environments-reference#runner-cli-flags)限制给定主机执行哪个帐户的会话，但它不会缩小谁可以分派到环境中。要使自托管环境成为唯一的选择器选项，[所有者](https://code.claude.com/docs/zh-CN/cloud-environments.md#organization-shared-environments)可以从[**云环境**页面](https://claude.ai/admin-settings/cloud-environments)为整个组织隐藏 Anthropic 托管的环境。
* **强制执行 repo-settings 保护**：使用 [`--confine-repo-settings`](https://code.claude.com/docs/zh-CN/self-hosted-environments-reference#runner-cli-flags) 选择保护模式。默认的 `warn` 记录违规并仍然生成会话，`enforce` 拒绝会话，`off` 禁用扫描。运行器扫描每个存储库的提交设置以查找：

  * 在该会话自己的工作区之外解析的授予：`additionalDirectories` 条目、`permissions.allow` 中的 `Edit`、`Write` 或 `NotebookEdit` 规则，或 `sandbox.filesystem.allowWrite` 或 `allowRead` 条目
  * 非空的 `env` 块
  * 操作员态势覆盖，例如 `sandbox.enabled: false`

  无论 [`--trust-workspace`](https://code.claude.com/docs/zh-CN/self-hosted-environments-reference#runner-cli-flags) 如何，保护都会运行，并且不涵盖存储库钩子、`.mcp.json` 或 Bash 规则；请参阅[权限和工具批准](https://code.claude.com/docs/zh-CN/self-hosted-environments-configuration.md#permissions-and-tool-approval)了解这些授予的位置。

> [!NOTE]
>
> 如果您的组织启用了 [IP 允许列表](https://support.claude.com/en/articles/13200993-restrict-access-to-claude-with-ip-allowlisting)，请在启动运行器和会话容器之前，将它们的公共出站地址添加到允许列表中。如果您运行[按需运行器](https://code.claude.com/docs/zh-CN/self-hosted-environments-configuration.md#on-demand-runners)，还需添加编排器主机的地址。不要将允许列表作为运行器或会话流量的网络控制，而应在您自己的网络边界应用默认拒绝出站流量。

## 网络要求

运行器及其生成的会话子进程向以下主机进行出站连接。将会话容器出站流量限制为这些主机和会话需要到达的特定内部服务；[默认拒绝出站流量](#default-deny-egress)涵盖如何以及为什么。

这些主机始终是必需的：

| 主机 | 端口 | 用途 |
| :- | :- | :- |
| `api.anthropic.com` | 443，HTTPS；[Anthropic 托管的 git](#use-the-anthropic-git-proxy) 使用 WSS | 运行器控制平面和会话流式传输、模型推理、功能标志、产品分析、[JWKS](https://code.claude.com/docs/zh-CN/self-hosted-environments-identity.md) 密钥获取、提交签名，以及设置 `--use-anthropic-git-proxy` 时的 Anthropic 托管 git |
| 您的 git 主机，例如 `github.com` 或您的 GitHub Enterprise 主机 | 443 或 22 | 在运行器会话使用的每个 git 主机上克隆和推送仓库。对于使用 [`--use-anthropic-git-proxy`](#use-the-anthropic-git-proxy) 的运行器，请参阅[何时仍需要 `github.com` 路径](#github-com-egress-with-the-anthropic-git-proxy)。 |

使用 [`--use-anthropic-git-proxy`](#use-the-anthropic-git-proxy) 的运行器通过 `api.anthropic.com` 路由其 `github.com` git 流量，因此不需要 `github.com` 的 git 主机路径。如果您设置了 `--push-outcome-on-release` 或从 `post-session` hook 推送，则仍需要该路径。

这些主机是否需要取决于您的配置：

| 主机 | 端口 | 何时需要 |
| :- | :- | :- |
| `api.github.com` | 443 | 无论是否设置 `--use-anthropic-git-proxy`，当运行器镜像包含 [GitHub CLI](#github-api-access-with-the-github-cli) 且您为会话提供 github.com 凭据（例如 `GH_TOKEN`）时 |
| `downloads.claude.ai` | 443 | 在安装时，当您使用本机安装程序在主机上安装或更新 Claude Code 时；`install.sh` 脚本本身从 `claude.ai` 提供。在会话运行时，仅当会话从官方 Anthropic 市场安装插件时。 |
| `storage.googleapis.com` | 443 | 在会话运行时，用于 `/plugin` 中显示的插件安装计数和元数据。 |
| `code.claude.com` 和 `claude.com` | 443 | 内置 claude-code-guide 代理的文档查找和会话期间预批准的 WebFetch 请求。阻止这些主机仅影响文档查找。 |
| `*.frame.claudeusercontent.com` | 443 | 仅当[工件工具](https://code.claude.com/docs/zh-CN/artifacts.md#availability)对您组织中的会话可用时；默认值因计划而异，请参阅那里的可用性表。在运行器上设置 `CLAUDE_CODE_DISABLE_ARTIFACT=1` 以保持工具禁用，无论组织设置如何。 |
| `registry.npmjs.org` | 443 | 当会话安装插件时，用于获取 npm 源插件包和安装插件的 Node.js 依赖项，或当 `npx` 启动的 MCP 服务器运行时 |
| 您的构建所使用的软件包注册表的内部镜像，或这些注册表本身 | 443，或您镜像的端口 | 当会话安装依赖以构建或测试您的仓库时 |
| `http-intake.logs.us5.datadoghq.com` | 443 | Anthropic 操作指标。仅当设置 `CLAUDE_CODE_BYOC_ENABLE_DATADOG=1` 时；在自托管环境中默认关闭。 |
| `browser-intake-us5-datadoghq.com` | 443 | Anthropic 错误报告上传，仅在为会话帐户启用[错误报告](https://code.claude.com/docs/zh-CN/data-usage.md#telemetry-services)时发送。由 `DISABLE_ERROR_REPORTING=1` 或 `DISABLE_TELEMETRY=1` 抑制。 |
| 您的云提供商用于模型请求、模型查询和续期凭据的端点，例如 `bedrock-runtime.us-east-1.amazonaws.com` 或 `aiplatform.googleapis.com` | 443 | 仅当运行器[将模型请求发送到 Amazon Bedrock 或 Google Cloud 的 Agent Platform](https://code.claude.com/docs/zh-CN/self-hosted-environments-configuration.md#send-model-requests-to-bedrock-or-agent-platform) 时 |

您不需要为运行器或会话流量将以下主机加入允许列表：

* **`statsig.anthropic.com`、`*.sentry.io`、`claude.ai` 和 `platform.claude.com`**：这些主机出现在一些较旧的企业网络检查清单中，但运行器不会访问它们。功能标志获取转到 `api.anthropic.com`，运行器使用环境密钥而不是交互式 OAuth 进行身份验证。
* **`mcp-proxy.anthropic.com`**：自托管会话不使用它。当为您的组织启用连接器交付时，您组织的 claude.ai 连接器通过 `api.anthropic.com` 到达会话。请参阅 [MCP 服务器](https://code.claude.com/docs/zh-CN/self-hosted-environments-configuration.md#mcp-servers)。

以下主机端流程确实会访问 `claude.ai`，因此请从出站流量允许访问它的主机运行这些流程，而不是扩大会话容器出站流量：

* **单行安装程序**：在安装时从 `claude.ai` 获取 `install.sh`。
* **交互式 `claude auth login`**：通过 `claude.ai`、`claude.com` 和 `platform.claude.com` 登录。[引导设置](https://code.claude.com/docs/zh-CN/self-hosted-environments-quickstart.md#run-the-guided-setup)、`doctor` 的已登录模式和 [CI 分派](https://code.claude.com/docs/zh-CN/self-hosted-environments-testing.md#authenticate-from-ci)会使用它。您用于登录的浏览器还会从 `hcaptcha.com`、`*.hcaptcha.com` 和 `challenges.cloudflare.com` 加载 claude.ai 登录页面的浏览器检查。如果您的组织使用单点登录，该浏览器还可能被重定向经过 `sso.claude.com`。

### 默认拒绝出站流量

在网络段或命名空间中部署运行器和会话容器，其出站流量限制为[网络要求表](#network-requirements)中的主机、您的 git 主机和会话需要到达的特定内部服务。该产品无法验证或强制执行此操作，因此在每个环境的您自己的网络边界应用它。会话代码是模型指导的，可以尝试连接到任意主机；网络层的默认拒绝出站流量限制这些尝试可以到达的位置。这适用于任何权限模式：默认预批准工具集已包括 `Bash`，因此 shell 出站流量在没有[自动模式](https://code.claude.com/docs/zh-CN/self-hosted-environments-configuration.md#permissions-and-tool-approval)的情况下运行而不提示。

有关每个会话发出的遥测详情以及如何关闭它，请参阅[遥测](https://code.claude.com/docs/zh-CN/self-hosted-environments-reference.md#telemetry)。

### 向出站代理进行身份验证

某些企业出站代理在每个连接上需要 `Proxy-Authorization` 标头。该标头中的令牌通常轮换太快而无法写入您在 `HTTPS_PROXY` 中设置的代理 URL。像往常一样将 `HTTPS_PROXY` 或 `HTTP_PROXY` 设置为您的代理 URL，然后设置 `--proxy-authorization-command` 或 `--proxy-authorization-file` 以告诉运行器从何处读取标头值。两个标志都需要 Claude Code v2.1.238 或更高版本。

#### 选择 `Proxy-Authorization` 值的来源

选择与您生成 `Proxy-Authorization` 令牌的方式相匹配的标志：

* **[`--proxy-authorization-command <command>`](https://code.claude.com/docs/zh-CN/self-hosted-environments-reference#runner-cli-flags)**：为您按需生成的令牌选择此选项。运行器运行 shell 命令并使用其修剪的 stdout 作为标头值，例如 `Bearer <token>`。
* **[`--proxy-authorization-file <path>`](https://code.claude.com/docs/zh-CN/self-hosted-environments-reference#runner-cli-flags)**：为另一个进程轮换到位的令牌选择此选项。运行器读取文件并使用其修剪的内容作为标头值。

#### 运行器拒绝启动的配置

每个标志也有一个环境变量形式，在[运行器 CLI 标志参考](https://code.claude.com/docs/zh-CN/self-hosted-environments-reference.md#runner-cli-flags)中列在其旁边。在运行器联系您的代理或控制平面之前，它检查标志及其变量，并在三种情况下拒绝启动：

* **两个标志都设置**：一个标志加上另一个标志的环境变量计为设置两个。
* **没有代理 URL**：`HTTPS_PROXY` 和 `HTTP_PROXY` 都不包含 `http://` 或 `https://` URL。运行器以大写或小写读取两个变量，不查询 `ALL_PROXY`。
* **任一标志传递给编排器子命令**：`self-hosted-runner orchestrator` 不接受标志或其环境变量。改为将标志传递给编排器启动的每个运行器。

#### 设置代理授权标志时运行器更改的内容

设置任一标志后，运行器启动自己的侦听器并通过该侦听器发送来自自身、其生命周期钩子和其会话的代理流量。侦听器在到达您的代理的途中添加 `Proxy-Authorization` 标头。

* **侦听器**：侦听器是 `127.0.0.1` 上的转发代理。运行器在向控制平面注册之前启动侦听器，如果侦听器无法启动则在启动时退出。
* **代理变量**：运行器重写您设置的 `HTTPS_PROXY` 和 `HTTP_PROXY` 中的任何一个，使其指向侦听器。该重写的值到达运行器本身、其生命周期钩子和它运行的每个会话。
* **令牌轮换**：轮换的令牌无需重启即可生效。对于侦听器打开到您的代理的每个连接，运行器再次运行您的命令或读取您的文件并将结果添加为标头。
* **会话环境**：会话仅通过侦听器到达您的代理。在每个会话的环境中，运行器删除 `ALL_PROXY`，删除您未设置的 `HTTPS_PROXY` 或 `HTTP_PROXY` 的任何拼写，并将 `NO_PROXY` 固定到运行器自己的值。
* **日志**：运行器从不记录标头值。

## 配置 git

运行器管理存储库检出但默认不配置 git 身份或凭证。您控制运行器的镜像和进程环境，因此您控制 git 配置。选择两种方法之一：

* **让运行器配置 git**：使用 `--configure-git` 启动运行器，使其写入 Anthropic 托管会话使用的相同身份和提交签名配置
* **在镜像中提供 git 配置**：自己设置身份和推送凭证，例如在您自己的机器人身份下提交

对于 github.com 上的仓库，您还可以使用 [`--use-anthropic-git-proxy`](#use-the-anthropic-git-proxy) 启动运行器，或设置 `CLAUDE_RUNNER_USE_GIT_PROXY=1`，以请求 Anthropic 为运行器的会话提供 git 服务。

运行器主机上的 Git 版本下限：[`--configure-git`](#let-the-runner-configure-git) SSH 提交签名需要 Git 2.34 或更高版本，[`--use-anthropic-git-proxy`](#use-the-anthropic-git-proxy) 需要 2.32 或更高版本，从 [`--push-outcome-on-release`](https://code.claude.com/docs/zh-CN/self-hosted-environments-reference#runner-cli-flags) 推送的分支恢复会话需要 2.29 或更高版本。如果您省略所有三个并自己管理 git 身份，Git 2.24 就足够了。

### 让运行器配置 git

使用 `--configure-git` 启动运行器，或设置 `SELF_HOSTED_RUNNER_CONFIGURE_GIT=1`，使其在启动时写入全局 git 配置：

* `user.name = Claude` 和 `user.email = noreply@anthropic.com`，与 Anthropic 托管会话匹配
* SSH 格式提交和标签签名，通过运行器管理的垫片路由，使用会话自己的凭证通过 Anthropic 的签名服务签署每个提交。签名可在 GitHub 上针对 Anthropic 的已发布 SSH 签名密钥进行验证。
* `push.negotiate = true`，所以 git 在打包推送之前询问您的 git 主机它已经拥有哪些提交。需要 Claude Code v2.1.257 或更高版本。
* `core.hooksPath` 指向运行器管理的钩子目录。其 `commit-msg` 和 `prepare-commit-msg` 钩子为每个提交添加会话创建者的 `Co-authored-by:` 尾注。该尾注根据 [`CCR_SESSION_ACCOUNT_EMAIL`](https://code.claude.com/docs/zh-CN/self-hosted-environments-configuration#wrapper-scripts) 中的电子邮件构建，当该变量未设置时省略。如果您的镜像已设置 `core.hooksPath`，且运行器未使用 [Anthropic 管理的 git](#use-the-anthropic-git-proxy)，运行器会保留您的设置，跳过安装这些钩子，并打印 `[runner:git]` 警告。

提交签名需要 git 2.34 或更高版本；运行器在启动时检查并在您的 git 较旧时以错误退出。此标志不配置推送凭证，您仍然在镜像中提供。

在 v2.1.280 或更高版本的运行器上，您从 `checkout` 或 `post-session` 生命周期钩子中进行的提交也会以会话身份签名，但不带 `Co-authored-by:` 尾注。[生命周期钩子内的 Git 配置](https://code.claude.com/docs/zh-CN/self-hosted-environments-configuration.md#git-configuration-inside-lifecycle-hooks)介绍了运行器在这些钩子内固定的 git 设置。

无论是否使用 `--configure-git`，Claude Code 都会指示 Claude 在其提交信息末尾添加 `Claude-Session: <url>` 尾注，并在其 Pull Request 描述末尾添加会话的 URL。要省略两者，请在运行器主机的 [`~/.claude/settings.json`](https://code.claude.com/docs/zh-CN/self-hosted-environments-configuration#how-each-session’s-config-is-assembled) 中将 [`attribution.sessionUrl`](https://code.claude.com/docs/zh-CN/settings-reference#attribution-sessionurl) 设置为 `false`，然后重新启动运行器。

### 在镜像中提供 git 配置

git 身份对任何提交都是必需的。在您的 Dockerfile 中系统范围设置它，以便配置适用于运行器进程运行的任何用户：

```dockerfile theme={null}
RUN git config --system user.name "Claude" && \
    git config --system user.email "noreply@anthropic.com"
```

没有身份，`git commit` 失败并显示 `Please tell me who you are`，会话无法取得进展。您可以改用自己的机器人身份；运行器不会覆盖这些值。

不要将长期或广泛范围的推送凭据烘焙到共享运行器镜像中：镜像中的凭据可用于镜像运行的每个会话，无论谁启动它。相反，从您的[包装脚本](https://code.claude.com/docs/zh-CN/self-hosted-environments-configuration.md#wrapper-scripts)按会话铸造短期、最小范围的令牌，使用从会话 JWT 解码的会话创建者的身份。将其与临时的按会话容器配对，这需要 `--capacity 1`，因此没有凭据超过铸造它的会话；请参阅[加固部分](#harden-your-deployment)。

如果您必须在镜像级别配置推送凭证，例如对于只读部署密钥，请尽可能紧密地限制它们：

* SSH 部署密钥限制为一个存储库，带有 `url.<base>.insteadOf` 重写
* 返回最小范围令牌的 `credential.helper`
* `GIT_SSH_COMMAND` 指向狭义范围的密钥

您配置的任何机制都必须无需提示即可工作，因为运行器的内置克隆和获取禁用 git、SSH 和 Git Credential Manager 否则会显示的提示：

* 运行器设置 `GIT_TERMINAL_PROMPT=0`，所以 git 不要求用户名或密码。
* 运行器使用 `BatchMode=yes` 运行 SSH，如果您设置了一个，则附加到您的 `GIT_SSH_COMMAND`，所以 SSH 不要求密码短语或主机确认。
* 运行器设置 `GCM_INTERACTIVE=never`，所以 Git Credential Manager 不打开登录对话框。在 v2.1.229 之前，仓库准备可能会因 Git Credential Manager 提示而挂起。
* 运行器清除 `core.askPass`，所以如果您使用 askpass 助手，改为通过 `GIT_ASKPASS` 环境变量设置它。

如果您的 git 主机拒绝凭证，或您没有配置凭证，运行器重试几次然后失败存储库准备（当存储库是会话推送结果的存储库时）。对于会话仅从中读取的存储库，[故障排除](#troubleshooting)涵盖运行器何时改为跳过它。运行器不会将这些设置传递到会话的环境中。

保持您在 `GIT_SSH_COMMAND` 或 `GIT_ASKPASS` 中命名的任何程序，会话无法写入它，就像[加固清单](#harden-your-deployment)要求钩子目录和包装脚本的方式一样。该程序命令行上的任何密钥或文件也是如此。运行器自己的 git 在克隆或获取时运行该程序。

如果检出目录由与运行器进程不同的 uid 拥有，git 拒绝对其进行操作；添加 `safe.directory`：

```dockerfile theme={null}
RUN git config --system --add safe.directory '*'
```

### 使用 Anthropic git 代理

使用 Anthropic git 代理（也称为 Anthropic 管理的 git）时，运行器镜像无需为会话本身提供 SSH 密钥、凭据助手、`.netrc` 或其他 git 凭据。相反，运行器请求 Anthropic 为其会话提供 git 服务。对于 Anthropic 提供服务的用户会话，运行器的克隆以及会话自身的获取和推送都经过 Anthropic，Anthropic 使用为会话创建者存储的 GitHub OAuth 令牌。[Anthropic 如何为会话提供 git 服务](#how-anthropic-serves-git-for-a-session)介绍了机器人和 Agent 会话的情况。

除非您[启用它](#turn-the-anthropic-git-proxy-on)，否则 git 代理处于关闭状态。使用自身凭据访问您的 git 主机的运行器不需要它，其 git 可与任何 git 主机配合使用。

作为交换，git 代理会限制运行器支持的内容，并改变运行器的需求：

* **仅限 github.com**：只有当会话的所有仓库都在 github.com 上时，Anthropic 才会为其提供服务，并且 git 代理尚不支持 GitHub Enterprise Server。在启用 git 代理的运行器上，包含其他 git 主机上仓库的会话[无法启动](#when-anthropic-doesnt-serve-a-session)。
* **仅限会话仓库的凭据**：Anthropic 只为属于会话的仓库提供 git 凭据，而不为同一 git 主机上的其他仓库提供。私有子模块、包管理器通过 git 获取的依赖，或位于另一个仓库中的插件市场，都不会从 Anthropic 获得凭据。请让创建会话的人员在创建会话时[添加会话所需的每个仓库](https://code.claude.com/docs/zh-CN/web-quickstart.md#start-a-task)。

  [云端定时任务](https://code.claude.com/docs/zh-CN/routines.md)的每次运行都是一个包含该任务仓库的会话，因此请让每个任务的所有者[添加其运行所需的每个仓库](https://code.claude.com/docs/zh-CN/routines.md#edit-and-control-routines)。
* **仅限分支推送**：删除分支的推送会失败，推送到任何其他类型的引用（例如标签）也会失败。有关推送可以更新哪些分支，请参阅 [GitHub 代理](https://code.claude.com/docs/zh-CN/cloud-environments.md#github-proxy)。
* **已连接的 GitHub 账户**：请让每个使用仓库创建用户会话的人[连接 GitHub](https://code.claude.com/docs/zh-CN/web-quickstart.md#connect-github)。如果他们没有自己的连接，这些会话[无法启动](#creator-has-no-github-connection)。
* **`--capacity 1`**：git 代理要求每个运行器进程只有一个会话，因此请运行更多副本以获得并行性。[启用 Anthropic git 代理](#turn-the-anthropic-git-proxy-on)列出了各项要求。
* **替换全局 git 配置**：运行器会[删除并替换其运行用户的全局 git 配置](#git-proxy-replaces-global-git-config)。请以专用用户身份或在容器中运行它。
* **主机推送使用主机凭据**：运行器的 [`--push-outcome-on-release`](https://code.claude.com/docs/zh-CN/self-hosted-environments-reference#runner-cli-flags) 推送以及您的 [`post-session` 钩子](https://code.claude.com/docs/zh-CN/self-hosted-environments-configuration#post-session)进行的任何推送，仍使用运行器主机自身的 git 凭据及其[到 `github.com` 的网络路径](#github-com-egress-with-the-anthropic-git-proxy)。请将这些凭据置于[全局 git 配置](#git-proxy-replaces-global-git-config)之外，并参阅[在镜像中提供 git 配置](#ship-git-config-in-your-image)了解提供这些凭据的方法。
* **按会话决定**：Anthropic 会针对运行器上的每个会话决定是否为其提供 git 服务，未获服务的会话将无法启动。[在启用 git 代理的运行器上会话无法启动时](#when-anthropic-doesnt-serve-a-session)介绍了原因。

> [!WARNING]
>
> 设置 `--use-anthropic-git-proxy` 后，运行器会删除并替换其运行用户的全局 git 配置，且不保留备份。它会在启动时以及每个会话之前执行此操作。您保存在其中的登录或凭据助手将会丢失。[`--configure-git`](#let-the-runner-configure-git) 写入的设置会保留。请以专用用户身份或在容器中运行运行器，切勿以您自己的用户身份运行。

将非机密的 git 设置（例如身份和 `safe.directory`）保存在系统 git 配置中。

#### 启用 Anthropic git 代理

在使用 `--use-anthropic-git-proxy` 启动运行器之前，请确认运行器主机满足以下每项要求。当容量或 git 要求未满足时，运行器会拒绝启动：

* **Claude Code v2.1.267 或更高版本**：较早的版本接受该标志，但不会报告请求 Anthropic 提供 git 服务，也不会打印 `Registering as opted in` 行，因此 Anthropic 不会为其会话提供服务。
* **`--capacity 1`（默认值）**：每个运行器进程一次处理一个会话，因此请运行更多副本以获得并行性。
* **Git 2.32 或更高版本**：较旧的 git 会忽略运行器为 git 代理设置的按会话 git 配置。

> [!WARNING]
>
> 本页上的 [Kubernetes](#kubernetes) 和 [Docker Compose](#docker-compose) 配方使用 `--capacity 4`。如果您在不将容量更改为 `1` 的情况下向其中一个添加 `--use-anthropic-git-proxy` 或 `CLAUDE_RUNNER_USE_GIT_PROXY=1`，每次您的编排器重新启动它时，运行器都会在启动时退出。设置 `--capacity 1` 并运行更多副本以获得并行性。[当运行器退出](#when-the-runner-exits)显示运行器打印的行。

要启用 git 代理，请将 `--use-anthropic-git-proxy` 添加到运行器的命令中，或在运行器的环境中设置 `CLAUDE_RUNNER_USE_GIT_PROXY=1`。在运行器主机的 shell 中运行以下命令，即可启动启用了 git 代理的[快速入门](https://code.claude.com/docs/zh-CN/self-hosted-environments-quickstart.md#set-up-manually)运行器：

```bash theme={null}
claude self-hosted-runner --environment-secret-file '/etc/claude/environment-secret' --base-dir '<writable-dir>' --use-anthropic-git-proxy
```

启动时，运行器会打印 `Registering as opted in to Anthropic-managed git (--use-anthropic-git-proxy)`。随后 Anthropic 会针对该运行器上的每个会话决定是否为其提供 git 服务。对于每个获得服务的会话，运行器会记录一行包含 `governed git ACTIVE` 的 `[runner:session]` 日志。如果会话反而无法启动，请参阅[在启用 git 代理的运行器上会话无法启动时](#when-anthropic-doesnt-serve-a-session)。

#### Anthropic 如何为会话提供 git 服务

对于 Anthropic 提供服务的会话，运行器的克隆以及会话自身的获取和推送都经过 Anthropic，并使用会话自己的短期令牌进行身份验证：

* **用户会话**：Anthropic 使用为会话创建者存储的 GitHub OAuth 令牌。
* **机器人和 Agent 会话**：Anthropic 使用您组织的 GitHub App 安装令牌。
* **URL 重写**：`--git-host-rewrite` 和 `--git-ssh-rewrite` 对 git 代理提供服务的仓库无效。

#### 在启用 git 代理的运行器上会话无法启动时

在使用 `--use-anthropic-git-proxy` 启动的运行器上，当 Anthropic 不为会话提供 git 服务时，会话将无法启动。请在运行器的日志中查找提及包含 `/git_proxy/` 的 `api.anthropic.com` 地址的 git 错误。

对于每个会话，Claude Code v2.1.267 或更高版本的运行器还会记录以下两者之一：当 Anthropic 为会话提供 git 服务时，记录一行包含 `governed git ACTIVE` 的 `[runner:session]` 日志；当不提供服务时，记录一行包含 `the server withheld Anthropic-managed git for this session` 的 `[runner:warn]` 日志。在以下情况中找到您看到的行：

* **既没有 `governed git ACTIVE` 也没有 `withheld` 行**：早于 Claude Code v2.1.267 的运行器不会记录这两行中的任何一行，Anthropic 也不会为其会话提供服务。请按照[固定版本](#pin-the-version)将运行器更新到 v2.1.267 或更高版本。
* **`withheld` 行**：Anthropic 未为该会话提供服务。之前可以正常使用 git 代理的运行器，即使您这边没有任何更改，也可能以这种方式失败。
  * **某个仓库不在 github.com 上**：只要会话中有一个仓库位于其他 git 主机（例如 GitHub Enterprise Server）上，该会话就不会获得服务，其 github.com 仓库也不例外。请为该环境的运行器[关闭 Anthropic git 代理](#turn-the-anthropic-git-proxy-off)。
  * **所有仓库都在 github.com 上**：请将此失败连同 `withheld` 行中的会话 ID 一起报告给[您的 Anthropic 客户团队](#report-an-issue)。Anthropic 会在其一端记录原因。
* **包含 `remote: access denied by the git proxy` 的行**：Anthropic 提供服务的会话仍可能被拒绝，例如当组织策略拒绝该会话的 git 访问，或该会话未获得该仓库的授权时。此时运行器的日志会显示一行包含 `remote: access denied by the git proxy` 的内容，该行的其余部分说明了原因。
* **`GitHub authentication required`**：当会话的创建者在 claude.ai 上没有可用的 GitHub 连接时会出现此情况。会话的克隆失败，git 错误显示为 `GitHub authentication required. Please reconnect your GitHub account.` 请让该用户在其 claude.ai 设置中连接或重新连接 GitHub。

修复原因后，请重新启动失败的会话。

#### 关闭 Anthropic git 代理

如果某个环境中的会话使用 github.com 以外的 git 主机（例如 GitHub Enterprise Server）上的仓库，请为该环境的运行器关闭 `--use-anthropic-git-proxy`。

**1. 移除标志**

从运行器的命令中移除 `--use-anthropic-git-proxy`。如果您在运行器的环境（例如 pod spec 或 Compose 文件）中设置了 `CLAUDE_RUNNER_USE_GIT_PROXY`，请在那里将其移除。在 shell 中，取消设置它：

```bash theme={null}
unset CLAUDE_RUNNER_USE_GIT_PROXY
```

**2. 为运行器提供 git 凭据**

为运行器会话使用的每个 git 主机（包括 github.com）提供无需提示即可工作的凭据。运行器用户全局 git 配置中的任何凭据都已丢失，因为在设置 `--use-anthropic-git-proxy` 期间运行器删除了该配置。请[在镜像中提供凭据](#ship-git-config-in-your-image)或使用 [`checkout` 生命周期钩子](https://code.claude.com/docs/zh-CN/self-hosted-environments-configuration#checkout)。

**3. 打开网络路径**

允许运行器通过 443 或 22 端口访问运行器会话使用的每个 git 主机。请参阅[网络要求](#network-requirements)中的 git 主机行。

**4. 重新启动运行器**

重新启动运行器，使其在不使用 git 代理的情况下注册。然后重新启动每个失败的会话。

#### 不使用 GitHub CLI 访问 GitHub API

如果您的运行器镜像不包含 GitHub CLI，Claude Code 可以提供内置的 `gh`，因此 Claude 仍然可以创建 Pull Request、发表评论并读取 CI 结果。内置 `gh` 适用于使用 Anthropic 管理的 git 的运行器。它支持一个命令 `gh api`，用于调用 GitHub 的 REST API。需要运行器镜像中的 Claude Code v2.1.287 或更高版本。

此命令代替 `gh pr create` 创建 Pull Request。内置 `gh` 会为当前仓库填入 `{owner}` 和 `{repo}`：

```bash theme={null}
gh api repos/{owner}/{repo}/pulls -f title='Fix' -f head='my-branch' -f base='main'
```

* **凭据**：内置 `gh` 通过 Anthropic 管理的 git 发送其 REST 请求，由 Anthropic 端提供 GitHub 凭据，因此镜像无需为其准备 GitHub 令牌
* **哪些会话可以获得它**：Anthropic 按会话决定是否由 Anthropic 管理的 git 为会话的 `gh` 提供服务。如果是，运行器为该会话记录的 `[runner:session] governed git ACTIVE` 行会显示 `gh_path_shim=true`。如果不是，该会话没有 `gh`
* **`jq`**：如果您希望 `--jq` 可用，请在镜像中安装 `jq`
* **[`CLAUDE_CODE_DISABLE_NONESSENTIAL_TRAFFIC`](https://code.claude.com/docs/zh-CN/env-vars)**：如果会话环境设置了它，Claude Code 不会提供内置 `gh`，会话也就没有 `gh`

#### 使用 GitHub CLI 访问 GitHub API

如果您的运行器镜像包含供会话使用的 GitHub CLI，请让您的[包装脚本](https://code.claude.com/docs/zh-CN/self-hosted-environments-configuration.md#provision-credentials-scoped-to-the-session-creator)按会话为会话创建者铸造一个短期、最小范围的 `GH_TOKEN`。当 `gh` 位于 Claude Code 启动时所用的 `PATH` 上时，会话使用该 `gh`，而没有[内置 `gh`](#github-api-access-without-the-github-cli)。借助该令牌，该 `gh` 以令牌授予的任何访问权限、在 Anthropic 管理的 git 之外访问 GitHub，因此请允许 [`api.github.com`](#network-requirements)。

使用 Anthropic 管理的 git 时，请勿在镜像中包含 GitHub CLI，以改用内置 `gh`。

#### 使用 Anthropic 管理的 git 信任专用证书颁发机构

如果您在运行器的环境中设置 `GIT_SSL_CAINFO` 或 `GIT_SSL_NO_VERIFY`，其会话使用 Anthropic 管理的 git，本部分适用。它描述的处理需要运行器运行 Claude Code v2.1.283 或更高版本。

当运行器上的 git 必须信任专用证书颁发机构 (CA)（例如 TLS 检查代理签署的证书颁发机构）时，通常的方法如下所示：

* **系统证书存储**：在运行器主机的系统证书存储中安装您的 CA，git 无需任何变量即可信任它。
* **`GIT_SSL_CAINFO`**：将其设置为您的 CA 的 PEM 文件，例如 `GIT_SSL_CAINFO=/etc/ssl/corp-ca.pem`。
* **`GIT_SSL_NO_VERIFY`**：在重新签名代理后面没有帮助。运行器自己通过 Anthropic 管理的 git 克隆检查证书，即使设置了变量，所以克隆失败，直到 git 通过其他两种方法之一信任您的 CA。

对于将会话令牌传送到 Anthropic 管理的 git 的 git 连接，运行器应用这两个变量如下。[`command` 钩子](https://code.claude.com/docs/zh-CN/self-hosted-environments-configuration#command)以会话的环境开始，所以它获得 git 在会话内获得的内容：

* **`GIT_SSL_CAINFO`**：git 检查 Anthropic 管理的 git 的内容取决于 git 运行的位置：
  * **运行器自己的克隆和获取**：运行时不使用变量，并根据运行器写入的按会话证书文件检查 Anthropic 管理的 git。运行器根据运行器主机的系统 CA 包和您的文件中的证书构建该文件。
  * **会话内的 Git**：获得 `http.sslCAInfo` 配置，命名您的文件代替变量，加上 `http.<url>.sslCAInfo` 条目，根据按会话文件检查 Anthropic 管理的 git。
  * **`checkout` 和 `post-session` 钩子**：继承变量不变。
* **`GIT_SSL_NO_VERIFY`**：哪些证书检查保持关闭取决于 git 运行的位置：
  * **运行器自己的克隆和获取**：运行时不使用变量，并检查它们呈现的证书。
  * **会话内的 Git**：获得 `http.sslVerify=false` 配置代替变量，所以检查对其他主机保持关闭。它还获得 `http.<url>.sslVerify=true` 条目，为 Anthropic 管理的 git 保持检查打开。
  * **`checkout` 和 `post-session` 钩子**：当会话在 Anthropic 管理的 git 上有存储库时，获得 `http.sslVerify=false` 配置代替变量。它们还获得 `http.<url>.sslVerify=true` 条目，为 Anthropic 管理的 git 保持检查打开。

按会话证书文件需要在运行器主机上的 `/etc/ssl/certs/ca-certificates.crt` 或 `/etc/pki/tls/certs/ca-bundle.crt` 处的系统 CA 包。它还需要一个 `GIT_SSL_CAINFO` 文件，运行器的用户可以读取，保存 PEM `CERTIFICATE` 块，最多 1 MiB。当运行器无法构建按会话文件时，它记录一行 `[runner:warn]` 包含 `did not build the certificate file` 和原因。Git 然后按原样为 Anthropic 管理的 git 使用您的文件。修复该行命名的内容。

对于使用 Anthropic 管理的 git 的每个会话，运行器还记录一行 `[runner:warn]` 开始于 `governed git: GIT_SSL_CAINFO is set` 或 `governed git: GIT_SSL_NO_VERIFY is set`。该行说明运行器对其自己的 git、会话内的 git 和您的生命周期钩子对该变量所做的操作。它以您是否需要更改任何内容结束。

### 将 github.com 仓库添加到 Claude Tag 频道会话

如果 Claude 无法将 github.com 仓库添加到 Claude Tag 频道会话，请确保该频道有权访问该仓库，即使您的运行器可以克隆它也是如此。该访问权限来自 GitHub 上和您的 Claude 组织中的设置，而不是来自运行器的 git 凭据。

**1. 安装 Claude GitHub App**

在拥有该仓库的 GitHub 组织或账户上安装 [Claude GitHub App](https://github.com/apps/claude)，并在安装中包含该仓库。

**2. 关联 GitHub 组织或账户**

在 [**Organization settings > Git providers**](https://claude.ai/admin-settings/source-control) 中，将该组织或账户关联到您的 Claude 组织。同时也是该 GitHub 组织所有者的 Claude 组织 Owner 可以关联它。请参阅[关联您的 GitHub 组织](https://claude.com/docs/claude-tag/admins/configure-github#link-your-github-organization)。

**3. 授予仓库访问权限**

在 [Claude Tag 管理设置](https://claude.ai/admin-settings/claude-tag)中，将该仓库授予该频道，或授予包含该频道的更大作用域：其 Slack 工作区或整个 Slack。拥有 Owner 角色或 Claude Tag Admin 权限的人员可以授予它。如果频道管理员的 GitHub 账户是该仓库的管理员，频道管理员也可以将其添加到自己的频道。请参阅[授予仓库访问权限](https://claude.com/docs/claude-tag/admins/configure-github#grant-repository-access)。

然后在该频道中开始一个新会话串，并在您的第一条消息中[指明该仓库](https://claude.com/docs/claude-tag/admins/configure-github#how-granted-repositories-reach-a-session)。

如果 Claude 仍然无法添加该仓库，请参阅[如果 Claude 无法访问仓库](https://claude.com/docs/claude-tag/admins/configure-github#if-claude-can%E2%80%99t-reach-a-repository)，了解在何处检查 GitHub 关联和仓库授权。

### 为专用网络重写 git URL

仓库 URL 从控制平面作为 HTTPS 到达，带有您的 git 主机的主机名；对于 GitHub Enterprise，这是您在 claude.ai 上为 [GitHub Enterprise 集成](https://code.claude.com/docs/zh-CN/github-enterprise-server.md)配置的主机名。两个可重复的标志在克隆之前重写这些 URL：

* `--git-host-rewrite <from>=<to>`：对于分割视界 DNS，其中 Anthropic 通过外部主机名到达您的 git 主机，但运行器必须使用内部主机名
* `--git-ssh-rewrite <host>`：适用于仅接受 SSH 的 git 主机，或仅有 SSH 密钥的运行器主机。例如，要通过 SSH 克隆 github.com 仓库，请将 `--git-ssh-rewrite github.com` 添加到运行器命令中。运行器随后会将 `https://github.com/owner/repo` 重写为 `git@github.com:owner/repo`。SSH 必须[无需提示即可连接](#git-must-work-without-a-prompt)。

主机重写首先运行，因此如果您需要两者，请在 `--git-ssh-rewrite` 中列出内部主机名。为了完全控制检出，使用 [`checkout` 生命周期钩子](https://code.claude.com/docs/zh-CN/self-hosted-environments-configuration#checkout)。

### 在 GitHub 之外的仓库上启动会话

要在 GitHub 以外的 git 主机上的仓库中运行会话，请从终端在该仓库的检出中启动会话。首先检查运行器的 git 凭据、网络和 git 代理、仓库地址以及您的分支：

* **Git 凭据和网络**：为运行器克隆的地址 `https://<host>/<owner>/<name>` [提供](#ship-git-config-in-your-image)[无需提示即可工作](#git-must-work-without-a-prompt)的凭据，并按照[网络要求](#network-requirements)所列，允许运行器访问您的 git 主机。对于仅接受 SSH 的 git 主机，请参阅 [`--git-ssh-rewrite`](#rewrite-git-urls-for-private-networks)。
* **Git 代理**：如果该环境的运行器使用 Anthropic git 代理，请[将其关闭](#turn-the-anthropic-git-proxy-off)。
* **仓库地址**：检查检出的 `origin` 远程。其主机必须是域名，例如 `git.example.com`，其路径必须采用 `<owner>/<name>` 形式。诸如 `group/subgroup/name` 的嵌套路径无法通过。
* **分支**：将您当前所在的分支推送到您的 git 主机，因为运行器会克隆该分支。

在检出中，按照[从 CI 进行身份验证](https://code.claude.com/docs/zh-CN/self-hosted-environments-testing.md#authenticate-from-ci)中的说明登录 claude.ai 后，使用 [`--environment`](https://code.claude.com/docs/zh-CN/self-hosted-environments-testing#run-the-test-loop) 和您环境的 `ccpool_...` ID 运行 `claude -p`：

```bash theme={null}
claude -p "Fix the login bug" --environment ccpool_abc123 --output-format json
```

创建会话后，该命令会打印一行以 `{"ok":true` 开头的 JSON。如果 `origin` 远程不符合要求，命令会失败并显示 `requires a git source, but no GitHub remote was detected`。

在您的 git 主机上审查并合并会话[推送](https://code.claude.com/docs/zh-CN/self-hosted-environments-configuration.md#prompt-sessions-to-push-their-work)的分支。

## 构建运行器镜像

Anthropic 不发布预构建的运行器镜像。围绕 `claude` 二进制文件构建您自己的，分层您的存储库需要的任何工具链：语言运行时、编译器、包管理器和 [MCP](https://code.claude.com/docs/zh-CN/mcp.md) 边车。

下面的配方使用 `--capacity 4`，所以一个容器为来自同一锁定所有者的最多四个并发会话服务。这不提供[加固部分](#harden-your-deployment)中的按会话容器隔离：在将环境连接到生产系统之前，要么以 `--capacity 1` 运行配方，每个会话一个容器，要么使用[按需运行器](https://code.claude.com/docs/zh-CN/self-hosted-environments-configuration.md#on-demand-runners)，它也将环境密钥保持在会话运行主机之外。如果您将[Anthropic git 代理](#use-the-anthropic-git-proxy)添加到这些配方之一，也要将 `--capacity` 更改为 `1`。

这个 Dockerfile 是一个最小的起点。它安装了 `python3`，因为 [Anthropic 控制平面提供](https://code.claude.com/docs/zh-CN/self-hosted-environments-configuration.md#how-each-session’s-config-is-assembled)给 Claude Tag 会话的 hook（包括让 Claude 在 Slack 中发布回复的那个 hook）会运行 `python3`。在任何为 Claude Tag 会话提供服务的镜像中，请确保 `python3` 位于 `PATH` 中：

```dockerfile theme={null}
FROM debian:bookworm-slim
ARG CLAUDE_CODE_VERSION
RUN apt-get update && apt-get install -y --no-install-recommends git curl ca-certificates openssh-client jq python3 \
 && rm -rf /var/lib/apt/lists/*
RUN curl -fsSL "https://downloads.claude.ai/claude-code-releases/${CLAUDE_CODE_VERSION:?set with --build-arg CLAUDE_CODE_VERSION}/linux-x64/claude" \
      -o /usr/local/bin/claude && chmod +x /usr/local/bin/claude
RUN git config --system user.name "Claude" \
 && git config --system user.email "noreply@anthropic.com" \
 && git config --system --add safe.directory '*'
ENTRYPOINT ["claude"]
```

如果您的节点是 ARM，将 `linux-x64` 交换为 `linux-arm64`，或在 Alpine 等 musl 基础镜像上交换为 `linux-x64-musl` 或 `linux-arm64-musl`；请参阅 [Alpine Linux 设置](https://code.claude.com/docs/zh-CN/setup.md#alpine-linux-and-musl-based-distributions)了解 musl 镜像需要的额外包。URL 是标准 Claude Code 发布位置，因此您可以根据[二进制完整性和代码签名](https://code.claude.com/docs/zh-CN/setup.md#binary-integrity-and-code-signing)中描述的发布的已签名清单验证下载的二进制文件。运行器需要 Claude Code 版本 2.1.224 或更高版本；若要为 Claude Tag 会话提供服务，则需要 2.1.229 或更高版本。构建镜像，然后将其推送到您的注册表并在下面的配方中引用它：

```bash theme={null}
docker build \
  --build-arg CLAUDE_CODE_VERSION="$(curl -fsSL https://downloads.claude.ai/claude-code-releases/stable)" \
  -t <your-registry>/claude-runner:latest .
```

命令替换查找当前 `stable` 发布号并将其作为构建参数传递，因此在新的稳定版本发布后运行相同的命令会使用较新的二进制文件重建下载层。要为可重现的构建固定特定版本，请直接将版本号作为 `CLAUDE_CODE_VERSION` 传递。当您需要比稳定通道更新的版本（例如[新推出的模型所需的版本](https://code.claude.com/docs/zh-CN/model-config.md)）时，在查找 URL 中将 `stable` 替换为 `latest`。

## 为会话调整 CPU 和内存大小

为运行器运行的会话而不是运行器进程调整运行器的容器或主机大小。运行器本身轮询工作、准备每个会话的检出、运行您的[生命周期钩子](https://code.claude.com/docs/zh-CN/self-hosted-environments-configuration.md#lifecycle-hooks)，以及启动和监督会话进程。负载来自会话：每个都是一个 Claude Code 进程加上它启动的任何东西，例如构建、测试套件、包安装和 [MCP 服务器](https://code.claude.com/docs/zh-CN/mcp.md)。

对于一个会话，从以下值开始，表示为 Kubernetes 请求和限制或您平台的等效值，并将它们视为起点而不是要求：

* **内存**：请求和限制各 4 GiB，满足 Claude Code [系统要求](https://code.claude.com/docs/zh-CN/setup.md#system-requirements)中的 4 GB 最小值。保持两者相等，以便调度程序考虑容器的完整内存。当容器达到其内存限制时，内核杀死其中的进程，这可能会结束会话中途。
* **CPU**：请求 2 个 CPU，限制 4 个 CPU，所以会话可以在构建期间突发超过请求。内核在其 CPU 限制处限制容器，而不是杀死其中的进程，所以会话在限制处运行较慢但继续运行。

在 Kubernetes 容器规范中，使用以下 `resources` 块设置这些起始值：

```yaml theme={null}
resources:
  requests:
    cpu: "2"
    memory: 4Gi
  limits:
    cpu: "4"
    memory: 4Gi
```

构建和测试通常是会话负载中最大和最可变的部分，因此运行您的存储库的代表性构建，测量其峰值 CPU 和内存，并提高任何在该峰值之上没有为 Claude Code 进程留出空间的起始值。

运行器使用 `--capacity` 来限制它一次运行多少个会话。它不在它们之间分割 CPU 或内存，所以运行器上的会话共享容器的 CPU 和内存。为防止一个会话的内存使用导致其他会话结束，请每个运行器运行一个会话，并为其设置[独立的内存限制](#give-each-session-its-own-memory-limit)。

* **每个运行器一个会话**：给每个容器一个会话的值。在 `--capacity 1` 使用此大小，[加固部分](#harden-your-deployment)推荐，以及对于[按需运行器](https://code.claude.com/docs/zh-CN/self-hosted-environments-configuration.md#on-demand-runners)，您在您的 [`spawn-runner` 钩子](https://code.claude.com/docs/zh-CN/self-hosted-environments-configuration#the-spawn-runner-hook)提交的工作负载上设置值，例如 Kubernetes Job 的 pod 模板。
* **每个运行器多个会话**：在 `--capacity` 高于 1 时，将一个会话的值乘以容量，因为最多那么多会话可以在容器中同时运行。[Kubernetes](#kubernetes) 和 [Docker Compose](#docker-compose) 配方以 `--capacity 4` 运行，没有 CPU 或内存限制，因此添加为您运行的容量调整大小的限制。
* **一台主机上多个运行器容器**：按各运行器容量之和调整主机大小，因为每个运行器只将自己的会话计入 `--capacity`：

  * **内存**：将一个会话的内存乘以该总和，再加上主机自身进程使用的内存
  * **CPU**：将一个会话的 CPU 请求乘以该总和。若要让所有会话同时突发，则改为乘以其 CPU 限制。

  为每个运行器提供独立的容器。在每个容器中使用[相同的 `--base-dir` 路径](#keep-the-base-directory-and-capacity-identical-across-runners)，并将其放在没有其他运行器容器挂载的存储上，以便每个运行器拥有[自己的工作目录](#harden-your-deployment)。

### 为每个会话设置独立的内存限制

为防止内存耗尽时一个会话导致其他会话结束，请让每个运行器以 `--capacity 1` 运行，并置于其独立的内存限制之下，该限制设为一个会话的内存：

* **在容器中**：每个容器运行一个运行器，并设置容器的内存限制，例如 Kubernetes 上的 `limits.memory`、Docker 上的 `--memory` 或 Compose 文件中的 `mem_limit`。保持各容器限制之和低于主机内存，因为当主机本身内存耗尽时，内核可能会杀死主机上任何位置的进程。
* **在不使用容器的 systemd 下**：在运行器的 unit 上设置 `MemoryMax`，并且每台主机运行一个运行器，因为在同一文件系统上[使用相同 `--base-dir`](#keep-the-base-directory-and-capacity-identical-across-runners) 的运行器会共享克隆在那里的仓库。例如，在 `--capacity 1` 且没有 `checkout` 钩子的情况下，它们在同一仓库上的会话可能同时在同一个检出中工作。若要在一台主机上运行多个运行器，请为每个运行器提供一个容器。

当容器或 unit 达到其内存限制时，内核只杀死其内部的进程。如果您将 `--capacity` 设置为大于 1，会话将共享同一个限制。当它们达到该限制时，内核不会按会话选择要杀死的进程，因此一个会话的内存使用可能导致其他会话结束。

在 Kubernetes 上，kubelet 的 `singleProcessOOMKill` 设置无论取哪个值都无法将会话隔离开。您也无法在[包装脚本](https://code.claude.com/docs/zh-CN/self-hosted-environments-configuration.md#wrapper-scripts)中使用内存 `ulimit` 来限制一个会话的总内存。内核会对每个进程单独应用诸如 `ulimit -v` 之类的值。

### 在进程和线程限制下留出余量

要确定进程和线程限制的大小，请测量一个会话所需的数量，并乘以共享该限制的会话数。当这些会话达到限制时，内核会拒绝创建新的进程和线程，因此其中任何会话中的命令、构建和测试都可能失败。被拒绝的调用返回 `EAGAIN`，其标准消息为 `Resource temporarily unavailable`。

假设一个会话的进程和线程峰值为 300 个，且有四个会话共享该限制。它们总共需要 1,200 个，若将限制设为 2,400，则可为需要两倍数量的构建留出余量。

#### 各限制的设置位置

此表中的每个限制都同时计算线程和进程，`pids.max` 是保存 cgroup 限制的文件：

| 限制 | 设置位置 | 共享范围 |
| :- | :- | :- |
| Kubernetes 上的 `pids.max` | 运行运行器的节点的 kubelet 配置中的 `podPidsLimit`。无法在 Pod spec 中设置。 | 一个 Pod 的所有容器 |
| Docker 上的 `pids.max` | `--pids-limit`，或 Compose 文件中的 `pids_limit` | 一个容器 |
| systemd 下的 `pids.max` | 运行器 unit 上的 `TasksMax` | 一个 unit |
| `ulimit -u` | 启动运行器的 shell 或服务 | 主机上以运行器用户 ID 运行的所有内容。对 root 不强制执行。 |

当您未设置 `TasksMax` 时，systemd 会应用 `DefaultTasksMax`，其默认值为 `kernel.pid_max`、`kernel.threads-max` 和根 cgroup 的 `pids.max` 中最小值的 15%。

#### 测量一个会话的峰值

要获取您自己运行器的数值，请在其主机上测量一个会话，在 Kubernetes 上主机即为节点：

**1. 单独启动一个运行器**

以 `--capacity 1` 启动一个运行器，其容器或 systemd unit 中没有其他任何内容，并且在其主机和环境中都是唯一的：

* **其主机**：使用不运行其他运行器的主机。这样您可以在下一步中找出它的进程。
* **其环境**：使其成为该环境中唯一的运行器，例如在您[为测试创建](https://code.claude.com/docs/zh-CN/self-hosted-environments-quickstart.md#set-up-manually)的环境中。会话是路由到环境而不是路由到运行器的。

暂时不要启动会话，因为在构建开始之前您需要下一步中的目录。

**2. 找到运行器的 cgroup 目录**

在主机上（而不是在容器内）找到运行器的进程 ID，例如使用 `ps`。读取该 ID 对应的 `/proc/<pid>/cgroup`：

* **某行包含 `:pids:`**：主机使用 cgroup v1，目录为 `/sys/fs/cgroup/pids` 后接 `:pids:` 之后的文本
* **唯一的一行以 `0::` 开头**：主机使用 cgroup v2，目录为 `/sys/fs/cgroup` 后接 `0::` 之后的文本

**3. 运行代表性构建并记录峰值**

让一个会话运行一次代表性构建，并从运行器的 cgroup 目录中获取峰值：

* **目录中有 `pids.peak`**：在构建完成后读取该文件
* **目录中没有 `pids.peak`**：在构建运行期间每秒读取一次 `pids.current`，并保留最高值。两次读取之间出现的峰值无法被观测到，因此请将该值视为偏低的估计。

**4. 乘以共享该限制的会话数**

* **一个运行器的 Pod、容器或 unit**：将峰值乘以运行器的 `--capacity`
* **多个运行器共享的限制**：将峰值乘以它们容量之和

然后加上共享该限制的其他任何内容的峰值，例如 Pod 中的另一个容器。

**5. 将限制设置为高于结果**

限制越高，为更重的构建留出的余量越多；限制越低，越能更早地阻止 fork 炸弹。

#### 检查 `pids.max` 限制

* **检查是否达到了 `pids.max`**：在主机的内核日志中（例如使用 `dmesg`）搜索包含 `cgroup: fork rejected by pids controller in` 和某个 cgroup 路径的行。内核仅在第一次拒绝该 cgroup 中的进程或线程时记录此日志，因此请搜索整个日志。
* **查看生效的限制**：读取运行器 cgroup 目录及其上每一级目录中的 `pids.max`。最小值即为生效的限制，`max` 表示该级别没有限制。

## Kubernetes

运行器默认在端口 8080 上提供 `GET /healthz`，可使用 `--health-port` 配置，因此 Kubernetes 探针无需额外设置即可工作。端点在进程活着时返回 `200`，所以下面的探针检测死进程，而不是卡住的进程；要捕获停止轮询的运行器，请在 [`/metrics`](https://code.claude.com/docs/zh-CN/self-hosted-environments-reference#prometheus-metrics) 的 `last_poll_age_seconds` 系列上发出警报。下面的 Deployment 从 Kubernetes Secret 挂载环境密钥，将活跃度和就绪探针指向 `/healthz`，并设置 90 秒的终止宽限期。请参阅[关闭时序](#shutdown-timing)了解为什么宽限期很重要。

清单在运行器容器上设置没有 CPU 或内存 `resources`。添加为您运行的容量调整大小的块，如[为会话调整 CPU 和内存大小](#size-cpu-and-memory-for-sessions)所述。

```yaml theme={null}
apiVersion: apps/v1
kind: Deployment
metadata:
  name: claude-runner
  namespace: claude-runners
spec:
  replicas: 3
  selector:
    matchLabels:
      app: claude-runner
  template:
    metadata:
      labels:
        app: claude-runner
        app.kubernetes.io/part-of: claude-code-self-hosted-runner
    spec:
      terminationGracePeriodSeconds: 90
      containers:
        - name: runner
          image: <your-registry>/claude-runner:latest
          args:
            - self-hosted-runner
            - --environment-secret-file
            - /etc/claude/environment-secret
            - --capacity
            - "4"
            - --base-dir
            - /workspace
          volumeMounts:
            - name: environment-secret
              mountPath: /etc/claude
              readOnly: true
          ports:
            - name: health
              containerPort: 8080
          readinessProbe:
            httpGet:
              path: /healthz
              port: 8080
            initialDelaySeconds: 5
            periodSeconds: 10
          livenessProbe:
            httpGet:
              path: /healthz
              port: 8080
            initialDelaySeconds: 30
            periodSeconds: 30
      volumes:
        - name: environment-secret
          secret:
            secretName: claude-runner-environment-secret
```

上面的 Deployment 存在于 `claude-runners` 命名空间中。首先创建命名空间：

```bash theme={null}
kubectl create namespace claude-runners
```

从保存您在管理 UI 的 [**Copy environment key** 步骤](https://code.claude.com/docs/zh-CN/self-hosted-environments-quickstart.md#set-up-manually)中复制的值的本地文件创建支持 Secret，以便密钥永远不会出现在您的 shell 历史记录中。运行 `(umask 077 && cat > ./environment-secret)`，粘贴密钥，按 Enter，然后按 Ctrl-D。然后创建 Secret 并删除文件：

```bash theme={null}
kubectl create secret generic claude-runner-environment-secret -n claude-runners --from-file=environment-secret=./environment-secret
```

## Docker Compose

下面的 Compose 服务在运行器退出时重启它，这涵盖崩溃和正常退出后的 drain。Docker 重启策略重启同一容器及其可写层完整，所以运行器以重用的文件系统而不是[加固态势](#harden-your-deployment)推荐的新文件系统回来；为评估使用此配方，对于生产要么每次运行重新创建容器，要么使用执行此操作的编排器。

Docker 在容器不断退出时会在每次重启前等待更长时间，直到达到上限，所以在此配方下无法启动的运行器不会在紧密循环中不断重启。[当运行器退出时](#when-the-runner-exits)描述了发生这种情况时要检查的内容。

```yaml theme={null}
services:
  claude-runner:
    image: <your-registry>/claude-runner:latest
    command:
      - self-hosted-runner
      - --environment-secret-file
      - /run/secrets/environment-secret
      - --capacity
      - "4"
      - --base-dir
      - /workspace
    secrets:
      - environment-secret
    restart: always
    stop_grace_period: 90s

secrets:
  environment-secret:
    file: ./environment-secret
```

## 关闭时序

> [!NOTE]
>
> 这些时序适用于运行器。`claude self-hosted-runner orchestrator` 进程有其自己的时序：请参阅[调整 `--shutdown-drain-sec` 和编排器的停止超时时间](https://code.claude.com/docs/zh-CN/self-hosted-environments-configuration#size-shutdown-drain-sec-and-the-supervisors-stop-timeout-for-the-orchestrator)。

每个运行器会在启动时记录收到 `SIGTERM` 后干净地关闭其会话最多需要多长时间，例如在默认设置下为 `This runner needs up to 80s`。停止超时时间是指您的进程管理器在发送 `SIGTERM` 后、终止运行器之前等待的时长。请将其设置为至少该记录时间：

* **Kubernetes**：`terminationGracePeriodSeconds`。Kubernetes 默认为 30 秒，因此如果不进行此设置，它可能会在运行器完成之前停止 pod。
* **Docker Compose**：`stop_grace_period`

收到 `SIGTERM` 时，运行器停止接受新会话。随后，除非您设置了 [`--defer-shutdown-max-min`](#defer-the-drain-past-the-first-signal) 来推迟，否则它会开始一次优雅关闭，称为排空。排空包含三个步骤：

1. 运行器最多等待 [`--drain-wait-sec`](https://code.claude.com/docs/zh-CN/self-hosted-environments-reference#runner-cli-flags) 秒（默认为 `0`），以便仍在运行的轮次完成。
2. 终止每个会话的进程树，包括 Claude 仍在运行的任何命令，但不包括[在其 shell 命令退出后仍在运行的进程](#processes-a-stopped-session-leaves)。
3. 运行 [`post-session` 生命周期 hook](https://code.claude.com/docs/zh-CN/self-hosted-environments-configuration#post-session)。

在整个排空过程中，运行器持续轮询 Anthropic。这会使其会话保持分配给它，因此在您的 `post-session` hook 仍在保存未提交的工作时，其他运行器不会接管这些会话。

记录的时间是以下各值之和：

* 步骤 1 的 `--drain-wait-sec`，默认为 0 秒
* 步骤 2 的 [`--session-stop-grace-sec`](https://code.claude.com/docs/zh-CN/self-hosted-environments-reference#runner-cli-flags)，默认为 5 秒
* 步骤 3 的 [`--post-session-hook-timeout-sec`](https://code.claude.com/docs/zh-CN/self-hosted-environments-reference#runner-cli-flags)，默认为 60 秒
* 固定的 15 秒余量
* 设置 [`--push-outcome-on-release`](https://code.claude.com/docs/zh-CN/self-hosted-environments-reference#runner-cli-flags) 时额外的 30 秒

在默认设置下，总计为 0 + 5 + 60 + 15 = 80 秒。更高的 `--capacity` 不会增加该时间，因为运行器会同时排空其所有会话。

如果您设置了以下任一标志，请预留更多时间：

* **使用 [`--retire-at`](https://code.claude.com/docs/zh-CN/self-hosted-environments-reference#runner-cli-flags)**：在退休时间与主机停止时间之间留出足够的时间，以便典型轮次完成，再加上 [Runner lifecycle](https://code.claude.com/docs/zh-CN/self-hosted-environments#runner-lifecycle) 所述的后台任务等待时间，再加上记录的时间。在每次启动时计算退休时间，例如 `date +%s` 加上运行器的预期生命周期。
* **使用 [`--defer-shutdown-max-min`](#defer-the-drain-past-the-first-signal)**：停止超时时间还必须涵盖您配置的分钟数，以及排空开始前的进一步等待时间（默认设置下为 75 秒）。[Defer the drain past the first signal](#defer-the-drain-past-the-first-signal) 对这两者都有说明。运行器同样会在启动时记录这一更长的时间。

### 在重启前让正在运行的轮次完成

若要在滚动重启前让正在运行的轮次完成，请设置 `--drain-wait-sec`，并将停止超时时间提高到至少该等待时间加 80 秒。在默认等待时间为 `0` 的情况下，重启会中断任何仍在运行的轮次，会话将在另一个运行器上恢复，但不包含其[未推送的工作](#resumed-sessions-lose-unpushed-work)。

* **`--drain-wait-sec`**：大致设为您希望等待完成的轮次和后台任务所需的时长。这是一个上限：如果它们先完成，等待会提前结束。
* **您的停止超时时间**：Kubernetes 中的 `terminationGracePeriodSeconds`，Docker Compose 中的 `stop_grace_period`。例如，使用 `--drain-wait-sec 120` 时，将 [Kubernetes](#kubernetes) 或 [Docker Compose](#docker-compose) 配方中的 `90` 替换为 `200`。
* **如果您更改了其他关闭设置**：80 秒仅适用于默认设置。使用您的标志启动一个运行器，并至少采用其启动日志行 `This runner needs up to` 中的数字，该数字已包含等待时间。如果设置了 [`--defer-shutdown-max-min`](#defer-the-drain-past-the-first-signal)，请使用下一条启动日志行中 `Size the stop timeout to at least` 之后的更大数字。[Shutdown timing](#shutdown-timing) 列出了会增加这些数字的因素。

> [!WARNING]
>
> 如果停止超时时间在等待期间用完，主机会在工作仍在运行时终止运行器。其会话不会运行 `post-session` hook。运行器也不会注销，因此控制平面可能需要几分钟甚至更长时间才能重新排队这些会话。

### 在重启前发布状态消息

在进行会中断会话的重启之前，请让 [Owner](https://code.claude.com/docs/zh-CN/cloud-environments.md#organization-shared-environments) 从 [**Cloud environments** 管理页面](https://claude.ai/admin-settings/cloud-environments)在该环境上发布一条状态消息。在 [claude.ai/code](https://claude.ai/code) 中，成员会在该环境内的会话中看到这条消息。请让 Owner 在发布时设置过期时间，或在运行器恢复后移除该消息。

### 延迟排空超过第一个信号

如果你想要重启的运行器继续为其持有的会话服务长达 `n` 分钟，而不是在第一个信号上排空它们，请设置 [`--defer-shutdown-max-min <n>`](https://code.claude.com/docs/zh-CN/self-hosted-environments-reference#runner-cli-flags)。在第一个 `SIGTERM` 或 `SIGINT` 上，运行器停止接收新工作并继续为其持有的会话服务。它持续轮询，以便控制平面不会重新排队这些会话。需要 Claude Code v2.1.238 或更高版本。

#### 第一个信号后运行器持有的会话会发生什么

在信号后的前两个阶段，运行器释放会话，释放的会话在其用户发送下一条消息时在新运行器上恢复。从第一个信号开始计数，运行器经过三个阶段：

* **在前 `n` 分钟内**：运行器正常为其会话提供服务，并继续执行 `--startup-timeout-min` 和 `--kill-session-after-min`。如果您还设置了 [`--release-idle-session-min`](https://code.claude.com/docs/zh-CN/self-hosted-environments-reference#runner-cli-flags)，运行器会释放用户空闲达到该时长的任何会话，除非您[保留该会话](#keep-an-idle-session-while-its-processes-run)。如果未设置该标志，空闲会话会保留在运行器上。
* **当 `n` 分钟用完时**：运行器释放它仍然持有的每个会话，无论是否空闲。运行器等待中途轮次的轮次结束，以及最多 60 秒的轮次后台任务，然后释放该会话。
* **当发布后宽限期用完时**：运行器排空它仍然持有的任何会话，控制平面立即将每个排空的会话重新排队到另一个运行器。发布后宽限期从 `n` 分钟用完时开始，默认为 75 秒。如果你设置 `--drain-wait-sec` 超过 60 秒，发布后宽限期是 `--drain-wait-sec` 加 15 秒。

在任何阶段，运行器一旦不持有任何会话就以 0 退出。第二个信号缩短阶段：运行器立即排空，就像在没有 `--defer-shutdown-max-min` 的第一个信号上一样。一旦排空开始，下一个信号强制退出运行器。这适用于第二个信号或发布后宽限期用完是否启动了排空。

#### 调整停止超时

给您的主机停止超时至少三个部分的总和：您配置的 `n` 分钟、发布后宽限期和 [Shutdown timing](#shutdown-timing) 描述的排空。使用默认设置，发布后宽限期为 75 秒，排空最多需要 80 秒，因此允许 `n` 分钟加 155 秒。当设置 `--defer-shutdown-max-min` 时，运行器在启动时打印此总和。

如果停止超时时间在运行器完成之前用完，它仍持有的会话将不会运行 `post-session` hook。运行器不会注销，因此控制平面可能需要几分钟甚至更长时间才能重新排队这些会话。如果您无法为停止超时时间预留这一总和，请不要设置 `--defer-shutdown-max-min`，让运行器在收到第一个信号时就进行排空。

### 什么到达运行的 post-session 钩子

`post-session` 钩子和 Claude 会话子进程各自在自己的 POSIX 进程组中运行，与运行器分离，因此停止机制以不同方式到达它们：

* **运行器已在排空时的 `SIGTERM`**：立即强制退出运行器，跳过排空的任何剩余部分。没有 [`--defer-shutdown-max-min`](#defer-the-drain-past-the-first-signal)，那是运行器接收的第二个 `SIGTERM`。没有信号发送到正在运行的 `post-session` hook，因此在由初始进程收养孤儿进程的裸主机上，它会自行完成，但不受监督：其超时预算不再适用，写入关闭的日志管道可能会用 `SIGPIPE` 杀死它，因此需要在强制退出时存活的 hook 应该将其自己的输出重定向到文件。在此页面的容器配方中，运行器是容器的 PID 1，其退出会结束容器；在 systemd 的默认 `KillMode=control-group` 下，cgroup 范围的杀死也会到达 hook，如 **Cgroup-wide kills** 条目所述；在这两种情况下，请将强制退出视为对 hook 致命，并改为依赖宽限期。
* **进程组范围的信号**，例如包装脚本中的 `kill -- -<pid>`、shell 作业控制或组范围的看门狗：到达运行器和正在进行的 `checkout` 钩子子进程（故意保持组附加），但不到达正在运行的 `post-session` 钩子或会话子进程。
* **Cgroup 范围的杀死**，例如 systemd 的默认 `KillMode=control-group` 或当 `terminationGracePeriodSeconds` 过期时 Kubernetes 传递给整个容器的 `SIGKILL`：到达一切，包括 hook。进程组隔离不能防止这些，这就是为什么宽限期必须覆盖整个排空。
* **钩子自己的超时**：当钩子超过 `--post-session-hook-timeout-sec` 时，运行器向钩子的整个进程组发送 `SIGTERM`，然后两秒后发送 `SIGKILL`，因此钩子分叉的工作进程（例如 tar、rsync 或 git）与包装 shell 一起终止，而不是作为孤儿存活。运行器的监督在钩子的 stdio 关闭后结束：将其自己的输出重定向到文件并在 `SIGTERM` 阶段后存活的工作进程超出运行器的范围。

当排空开始时，以及在强制退出时，运行器记录仍在运行的 `post-session` 钩子数量，因此你可以区分安静的排空和正在进行快照的排空。

## 在运行器之间保持基目录和容量相同

如果运行器在会话中途死亡，服务器重新排队会话，环境中的另一个运行器获取它。该运行器从其自己的 `--base-dir` 和 `--capacity` 派生检出路径：`--capacity 1` 直接在 `--base-dir` 下检出，`--capacity` 高于 `1` 改用按会话 worktrees。当同一环境中的运行器对任一标志使用不同的值时，恢复的会话的工作目录更改，Agent 之前记录的绝对路径（在编辑、工具调用或其自己的笔记中）指向不再存在的位置。

在环境中的每个运行器上使用相同的 `--base-dir` 和 `--capacity`，并且不要使用按主机的值，例如实例 ID 或主机名。请像 [Kubernetes](#kubernetes) 和 [Docker Compose](#docker-compose) 示例那样显式传递这两个标志，使两个值都不依赖于默认值。

基目录默认为 `/workspace`，除了 [`--base-dir` 参考行](https://code.claude.com/docs/zh-CN/self-hosted-environments-reference#runner-cli-flags)记录的例外。运行器需要对其的写入访问。在启动时，在注册之前，运行器创建目录并确认它可以写入，当它不能时以 `cannot create or write to base directory` 退出。以 root 身份启动的运行器自己创建默认 `/workspace`。对于非 root 运行器，在启动运行器之前创建目录并给运行器的用户所有权，或将 `--base-dir` 指向该用户已拥有的目录。

## 重用预热的检出

对于大型仓库，克隆可能会主导会话启动。要跳过冷克隆，请在运行器保存其自身克隆的路径处自行提供一个克隆。在没有 [`checkout` hook](https://code.claude.com/docs/zh-CN/self-hosted-environments-configuration#checkout) 的情况下，运行器在 `<base-dir>/<repo-owner>/<repo>` 处为每个仓库保持一个规范克隆，并在会话间重用它：

* **在 `--capacity 1` 时**：运行器获取请求的引用，分离 `HEAD`，并硬重置到该引用，当变化不大时这几乎是瞬间完成的。
* **在 `--capacity` 大于 1 时**：运行器获取到该克隆中，然后从中为每个会话检出单独的 worktree。预热的克隆可以节省下载，但不能节省检出。

在镜像中或持久卷上提供克隆：

* **在镜像中克隆**：在该路径处将克隆构建到运行器镜像中。每个新容器随后都会以预热克隆启动，而无需重用磁盘。
* **在持久卷上克隆**：在使用 [`--lock-to-account`](https://code.claude.com/docs/zh-CN/self-hosted-environments-reference#runner-cli-flags) 预锁定到一个用户账户的运行器上，将 `--base-dir` 指向持久卷，这样磁盘只为该账户服务。预锁定的运行器永远不会接收 Claude Tag 频道会话，因此此选项不适用于为其服务的运行器。

重用路径的保证和不保证的内容：

* **克隆深度**：构建克隆时不要使用 `--depth`，是否使用 `--single-branch` 均可。当运行器获取到带有 `origin` 远程的克隆中时，永远不会传递 `--depth`，即使您设置了 [`CLAUDE_RUNNER_FETCH_DEPTH`](https://code.claude.com/docs/zh-CN/self-hosted-environments-reference#environment-variable-only-settings) 也是如此。使用完整克隆时，每个会话只下载克隆所缺少的提交。使用浅层克隆时，如果会话所在的分支是在克隆最旧的提交之前分叉的，该会话会下载该分支的完整历史。
* **跟踪的更改重置，未跟踪的文件保留**：在 `--capacity 1` 时，每个会话从硬重置开始，该重置会清除前一个会话的跟踪修改，但运行器永远不会运行 `git clean`，因此来自锁定所有者早期会话的未跟踪文件保留在树中。
* **按会话目录也会保留**：在检出旁边，运行器在 `<base-dir>/_sessions/` 下为其运行的每个会话创建按会话条目。会话的 Claude 配置目录保存对话记录的本地副本。在其旁边是会话的上传文件，当会话有任何文件时。会话目录也在那里：它保存会话运行时的任何按会话工作树和 `checkout` hook 检出，以及 Claude 在其中写入的任何其他内容。

  默认情况下，运行器在会话结束时将这些保留在原地，因此在持久化的磁盘上它们会累积。每个会话都以运行器自己的用户身份运行，因此该磁盘服务的任何后续会话都可以读取它们。如果保持持久的 `--base-dir`，请为该增长调整卷的大小。同样适用于在同一文件系统上重启运行器的任何设置，包括 [Docker Compose 配方](#docker-compose)。
* **使用 `--remove-session-state` 时，按会话目录不会保留**：使用 [`--remove-session-state`](https://code.claude.com/docs/zh-CN/self-hosted-environments-reference#runner-cli-flags) 启动运行器，以便在会话结束时删除每个会话的按会话目录。删除是尽力而为的：当运行器在清理运行前被杀死时，目录保留。规范克隆和会话在主机上其他地方写入的文件，例如临时目录，无论如何都会保留。
* **使用 git 代理时，重置变成检出**：使用 [`--use-anthropic-git-proxy`](#use-the-anthropic-git-proxy) 时，运行器在每个会话前清理克隆的 `.git/`。它保留对象存储和浅层状态，并删除索引。没有索引，每个会话都以完整的工作树检出开始，而不是近乎瞬间的重置。运行器仍然获取到现有克隆中，而不是再次克隆仓库。代理下不支持子模块预热。
* **使用 git 代理时，`reftable` 预热会丢失其引用**：当运行器在每个会话前清理 `.git/` 时，它会保留 `.git/refs/` 并删除 `.git/reftable/` 目录。以 git 的 `reftable` 引用格式预热的克隆会因此丢失其引用，因此请改为以 git 的 `files` 引用格式预热克隆。
* **长克隆不需要解决方法**：运行器使用 120 秒无进度监视器和 30 分钟硬上限来限制每个 git 操作，而不是平面超时，因此保持报告进度的缓慢冷克隆会完成。

### 确认运行器使用您的克隆

要确认运行器使用您的克隆，请使用 [`--log-level debug`](https://code.claude.com/docs/zh-CN/self-hosted-environments-reference#runner-cli-flags) 启动运行器，并在该仓库上启动一个会话。运行器会记录 `Prefetched repo found at` 以及克隆的路径。如果它记录的是 `No prefetched repo, fresh init at`，则说明它在该行中的路径处未找到可用的克隆。请检查您的克隆是否位于该路径、是否归运行器的用户所有，以及是否具有名为 `origin` 的远程。

如果运行器找到了您的克隆，但会话启动仍然缓慢，请检查该克隆是否为浅层克隆。

## 固定版本

默认情况下，每个会话的子进程运行运行器自己的二进制文件，并且运行器会在它生成的会话内关闭自动更新。在此默认设置下，每个会话都运行您在主机上安装或构建到镜像中的版本。主机级更新在运行器下次启动时生效。

选择您的会话运行哪个版本以及何时更改：

* **在固定版本之前**：针对您的会话使用的每个模型，检查[模型需要的 Claude Code 版本](https://code.claude.com/docs/zh-CN/model-config.md#available-models)。如果某个模型需要比您的会话所运行版本更新的版本，服务器会以 [Claude Code does not support this model](https://code.claude.com/docs/zh-CN/errors.md#claude-code-does-not-support-this-model) 拒绝对该模型的请求。
* **将队列保持在一个版本上**：使用固定版本构建镜像，或在裸主机上安装特定版本并[禁用自动更新](https://code.claude.com/docs/zh-CN/setup.md#disable-auto-updates)
* **升级固定队列**：阅读您当前版本与要安装版本之间的 [changelog](https://code.claude.com/docs/en/changelog.md) 条目，然后安装较新版本或重建镜像，并重启运行器
* **升级按需运行器**：阅读您当前版本与要安装版本之间的 [changelog](https://code.claude.com/docs/en/changelog.md) 条目，然后更改您的 [`spawn-runner` hook](https://code.claude.com/docs/zh-CN/self-hosted-environments-configuration#the-spawn-runner-hook) 启动的镜像。每个新运行器都会获得新版本。已经在运行的运行器（包括由 [`--min-idle`](https://code.claude.com/docs/zh-CN/self-hosted-environments-reference#orchestrator-cli-flags) 启动的备用运行器）会保持其版本，直到退出。不要重启它，因为它的工作指令是一次性的。
* **如果会话启动的是其他二进制文件**：如果您使用 [`--exec-path`](https://code.claude.com/docs/zh-CN/self-hosted-environments-reference#runner-cli-flags) 或 [`command` hook](https://code.claude.com/docs/zh-CN/self-hosted-environments-configuration#command) 来启动运行器自身以外的 Claude Code 二进制文件，会话将运行该二进制文件的版本。更新运行器时，请同时更新该二进制文件。
* **插件**：插件市场也不自动更新；在运行器的环境中设置 `FORCE_AUTOUPDATE_PLUGINS=1` 以让插件自动更新，同时二进制保持固定

## 扩展队列

您的编排器决定何时添加或删除运行器。由于[每个运行器一个所有者锁](https://code.claude.com/docs/zh-CN/self-hosted-environments#runner-lifecycle)，最小副本计数是您期望并发活跃的用户和 Claude Tag 代理数；`--capacity` 控制一个所有者内的并行性，而不是跨所有者。

两种扩展方法可用：

* **固定队列**：运行静态运行器副本集并在每个运行器提供的 [Prometheus 指标](https://code.claude.com/docs/zh-CN/self-hosted-environments-reference.md#prometheus-metrics)上扩展
* **按需运行器**：运行 `claude self-hosted-runner orchestrator` 子命令，它轮询 Anthropic 以查找没有可用运行器排队的会话，并调用您的 `spawn-runner` 钩子为每个会话启动一个。请参阅[按需运行器](https://code.claude.com/docs/zh-CN/self-hosted-environments-configuration.md#on-demand-runners)。

### 在固定队列上释放空闲会话

如果在每个运行器都在线的情况下，固定队列上的会话仍然处于排队状态，原因可能是空闲会话。默认情况下，会话在其轮次结束后仍会保留其槽位，其运行器也会保持锁定到该会话的[所有者](https://code.claude.com/docs/zh-CN/self-hosted-environments#key-concepts)。其他所有者的会话无法使用该运行器的空闲槽位。如果您团队的[云端定时任务](https://code.claude.com/docs/zh-CN/routines.md)每次运行都会启动一个新会话，其空闲会话可能会占满所有槽位。

要进行检查，请读取每个运行器 [Prometheus 指标](https://code.claude.com/docs/zh-CN/self-hosted-environments-reference.md#prometheus-metrics)中的 `claude_code_self_hosted_runner_session_idle_seconds`。运行器持有的每个会话都有自己的序列：会话繁忙时为 `0`，否则为自其进入空闲以来的秒数。选择您允许的空闲时间：要长于人们在任务中途的停顿时间，并且足够短，使每次运行都能在定时任务的下一次运行开始之前被释放。将序列与该时间进行比较。

* **序列高于该时间，例如 30 分钟对应 `1800`**：空闲会话占用了这些槽位，因此请设置 `--release-idle-session-min`
* **所有序列都低于该时间**：没有会话空闲那么久。请添加运行器，或针对[从不进入空闲的会话](#some-sessions-don’t-count-as-idle)设置 [`--kill-session-after-min`](https://code.claude.com/docs/zh-CN/self-hosted-environments-reference#runner-cli-flags)。
* **没有序列的运行器**：没有会话持有它。请参阅[故障排除](#troubleshooting)下的**会话保持排队**。
* **指标无响应的运行器**：其进程可能已退出。请参阅[运行器退出时](#when-the-runner-exits)下的**看起来健康的环境**。

要释放空闲会话，请使用 [`--release-idle-session-min`](https://code.claude.com/docs/zh-CN/self-hosted-environments-reference#runner-cli-flags) 及以分钟为单位的该时间启动每个运行器，例如 `--release-idle-session-min 30`。之后，运行器会释放任何空闲达到该时长的会话，无论该会话是由定时任务还是由人启动的，除非您[为其遗留运行的进程保留该会话](#keep-an-idle-session-while-its-processes-run)。

* **未推送的工作**：被释放的会话会在收到下一条消息时恢复，但不包含其尚未推送的工作。要保留其提交，请参阅[恢复的会话会丢失未推送的工作](#resumed-sessions-lose-unpushed-work)。
* **运行器退出**：释放其最后一个会话的运行器会开始[正常退出](#when-the-runner-exits)。请让您的监管程序在全新的文件系统上重新启动它，如[加固您的部署](#harden-your-deployment)所述。
* **从不进入空闲的会话**：还需设置 [`--kill-session-after-min`](https://code.claude.com/docs/zh-CN/self-hosted-environments-reference#runner-cli-flags)，因为[某些会话不计为空闲](#some-sessions-don’t-count-as-idle)

### 在进程运行期间保留空闲会话

要在会话遗留在您主机上的进程（例如 `nohup` 作业）完成之前一直保留该空闲会话，请使用 [`--hold-idle-file`](https://code.claude.com/docs/zh-CN/self-hosted-environments-reference#runner-cli-flags) 启动运行器，并从主机上持续 touch 该文件。需要 Claude Code v2.1.311 或更高版本。

以下步骤假定运行器以非 root 用户在主机上运行。如果您的运行器以 `root` 身份运行，其会话可以自行 touch 该文件，因此请先将其改为以专用用户身份运行。

**1. 为文件选择路径**

选择一个本地常规文件，例如 `/var/lib/claude-hold/hold-idle`，它应位于会话无法写入的目录中（包括其父目录），这样会话就无法写入该文件。运行器及其会话共用同一个操作系统用户，因此请先创建该目录，使其归另一个用户（例如 `root`）所有，并将模式设为 `755`。

**2. 启动运行器**

在您的 shell 中，使用 `--hold-idle-file`、`--release-idle-session-min` 和 [`--max-hold-min`](https://code.claude.com/docs/zh-CN/self-hosted-environments-reference#runner-cli-flags) 启动运行器，后者是保留的上限，从会话进入空闲时开始计算。如果不设置，上限为一天。请将其设置为高于 `--release-idle-session-min`，否则不会保留任何会话。按照[快速入门](https://code.claude.com/docs/zh-CN/self-hosted-environments-quickstart.md#set-up-manually)中的说明替换 `<writable-dir>`。此示例在 30 分钟后释放空闲会话，并在被保留的会话进入空闲后最多 4 小时将其释放：

```bash theme={null}
claude self-hosted-runner \
  --environment-secret-file /etc/claude/environment-secret \
  --base-dir '<writable-dir>' \
  --release-idle-session-min 30 \
  --hold-idle-file /var/lib/claude-hold/hold-idle \
  --max-hold-min 240
```

**3. 在进程运行期间 touch 该文件**

在进程运行期间，以该目录所有者的身份从主机上 `touch` 该文件，频率要高于每三分钟一次。第一次 `touch` 会创建该文件。例如，将以下行添加到 `root` 的 crontab 中：

```text theme={null}
* * * * * pgrep -x nightly-build > /dev/null && touch /var/lib/claude-hold/hold-idle
```

该行每分钟运行一次，并在任何名为 `nightly-build` 的进程运行期间 touch 该文件，因此启动了此类进程的会话会一直被保留，直至达到上限。请将 `pgrep` 检查替换为您的主机查找会话遗留运行内容的任何方式。

当这些进程完成时，检查必须不再匹配，否则该运行器上的每个空闲会话都会一直被保留，直至达到上限。

**4. 确认保留**

在进程运行期间，检查该文件的修改时间是否始终在三分钟以内。当会话的空闲时间耗尽时，在运行器的日志中查找 `idle release held`。如果没有出现这样的行，请读取运行器在启动时记录的 `watchdog config` 行中的 `leftover-hold=`，它会显示文件和上限，或者说明为何没有保留任何会话。

最后一次 `touch` 三分钟后，保留结束。再过大约一分钟，日志会显示 `idle release no longer held`，运行器随即释放该会话。

即使文件持续发生变化，保留也可能结束，例如以下情况：

* **超过上限**：`--max-hold-min` 耗尽。要保留更长时间，请调高该值
* **强制释放**：您设置了 [`--retire-at`](https://code.claude.com/docs/zh-CN/self-hosted-environments-reference#runner-cli-flags)、[`--defer-shutdown-max-min`](https://code.claude.com/docs/zh-CN/self-hosted-environments-reference#runner-cli-flags) 或 [`--kill-session-after-min`](https://code.claude.com/docs/zh-CN/self-hosted-environments-reference#runner-cli-flags)，并且达到了该限制。要保留更长时间，请设置更晚的限制

## 已知问题和限制

以下是此版本中的限制，其中存在解决方法。

### 连接器流量离开您的网络

Anthropic 从其自己的基础设施而不是从您的运行器调用连接器工具。连接器工具是 claude.ai 连接器，例如 GitHub、Slack 和 Linear。当 Claude 在自托管会话中使用连接器时，该流量通过 `api.anthropic.com` 而不是源自您的网络边界内。

要将连接器排除在自托管会话之外，使用 [`allowedMcpServers` 和 `deniedMcpServers` 策略设置](https://code.claude.com/docs/zh-CN/managed-mcp#policy-based-control-with-allowlists-and-denylists)过滤它。Claude Code 将这些设置应用于 Anthropic 交付的连接器以及您从运行器主机播种的服务器和用户添加的服务器，所以如果您为其他服务器部署允许列表，Claude Code 也会阻止交付的连接器。要在 URL 基础允许列表旁边保持连接器可用，添加与 Anthropic 代理路径匹配的条目以获得交付的连接器：

* `https://api.anthropic.com/v2/ccr-sessions/*`
* `https://api.anthropic.com/v1/code/sessions/*`
* `https://api.anthropic.com/v1/code/mcp/*`

如果工具流量必须保留在您的网络内，改为在运行器镜像上作为本地 MCP 服务器运行等效工具。请参阅 [MCP 服务器](https://code.claude.com/docs/zh-CN/self-hosted-environments-configuration.md#mcp-servers)。

### 某些会话不计为空闲

持有永不完成的后台任务的会话不计为空闲，所以 `--release-idle-session-min` 不会释放该会话的槽。等待从运行中工具调用内请求的批准的会话也不计为空闲。始终将 `--kill-session-after-min` 与其一起设置作为硬后挡，以便没有会话可以无限期地持有槽。

`--kill-session-after-min` 是失控会话的后挡。在 v2.1.260 或更高版本的运行器上，达到限制的会话不会立即终止。运行器给它一个宽限窗口，默认 15 分钟，您可以使用 [`SELF_HOSTED_RUNNER_MAX_LIFETIME_GRACE_MS`](https://code.claude.com/docs/zh-CN/self-hosted-environments-reference#environment-variable-only-settings) 更改：

* 如果会话等待其用户，运行器释放它。如果其转向已结束并仅持有后台任务，运行器最多等待 60 秒以完成这些任务，然后释放它。会话在其用户发送下一条消息时恢复。
* 如果转向仍在运行，运行器等待转向完成，或会话下一次等待其用户，然后释放它。
* 如果会话在宽限窗口结束时仍在运行器上，运行器终止它，任何运行转向的工作丢失。等待从运行中工具调用内请求的批准的转向是会话超过窗口的一种方式。

释放的会话从新克隆恢复，所以它未推送的工作无论如何都消失了；请参阅[恢复的会话丢失未推送的工作](#resumed-sessions-lose-unpushed-work)。在 v2.1.260 之前，运行器在限制处终止每个会话，最多等待宽限窗口以完成正在运行的轮次。

将该标志设置为高于您预期的最长会话时长，例如 `--kill-session-after-min 480` 为 8 小时。要从空闲的对话释放槽，改用 `--release-idle-session-min`。

### 恢复的会话丢失未推送的工作

在新运行器上恢复的会话会丢失其未推送的工作，因为该运行器会从其起始分支重新克隆仓库。

要保留已提交的工作，请在环境中的每个运行器上设置 [`--push-outcome-on-release`](https://code.claude.com/docs/zh-CN/self-hosted-environments-reference#runner-cli-flags)。设置了该标志的运行器会在释放之前尽力推送会话的结果分支，恢复的会话将从这些提交开始。未设置该标志的运行器会从起始分支恢复会话。推送使用运行器主机自身的 git 凭据，在使用 [Anthropic 托管 git](#use-the-anthropic-git-proxy) 的运行器上也是如此。

> [!WARNING]
>
> 设置该标志之前，请限制谁可以推送到源远程上的 `claude/*` refs。在恢复时，运行器会获取之前推送的分支，而不验证是谁推送的。

即使设置了该标志，也请通过其他方式保存以下工作：

* **未提交的更改**：这些更改仍会丢失。请参阅[提示会话推送其工作](https://code.claude.com/docs/zh-CN/self-hosted-environments-configuration.md#prompt-sessions-to-push-their-work)。
* **通过 [`checkout` 生命周期 hook](https://code.claude.com/docs/zh-CN/self-hosted-environments-configuration#checkout) 检出的仓库**：这些仓库不会被推送。请从 [`post-session` hook](https://code.claude.com/docs/zh-CN/self-hosted-environments-configuration#post-session) 对其进行快照。
* **在被某人归档或删除的会话中，或其运行器被某人从会话菜单中释放的会话中的提交**：不要依赖该标志的推送。要保存这些提交，请从 [`post-session` hook](https://code.claude.com/docs/zh-CN/self-hosted-environments-configuration#post-session) 将 `HEAD` 推送到一个救援分支。那里的示例仅在创建快照提交之后才推送，因此请让您的 hook 在没有任何未提交内容时也执行推送。

### 其他限制

* **因运行器丢失而中断的命令可能会再次运行**：当会话在另一个运行器上恢复时，Claude 会看到被中断的工具调用，其结果表明该调用的执行结果未知。
* **会话中途添加的仓库可能无法克隆**：Claude 通过 HTTPS 使用 `git clone` 克隆它。在未启用 [`--use-anthropic-git-proxy`](#use-the-anthropic-git-proxy) 的运行器上，如果主机上没有任何内容能够读取该仓库，克隆会因 git 身份验证错误而失败。如有可能，请在创建会话时选择会话所需的每个仓库。
  * **会话恢复时**：[`checkout` hook](https://code.claude.com/docs/zh-CN/self-hosted-environments-configuration#checkout) 也会为添加的仓库运行，此时 [`CLAUDE_RUNNER_REPO_REF`](https://code.claude.com/docs/zh-CN/self-hosted-environments-configuration#checkout) 为空，因此请确保您的 hook 能够处理这种情况。
* **某些连接器不出现在自托管会话中**：您在 claude.ai Settings 中尚未连接的连接器不在自托管会话中列出，会话不会提示您连接它。首先在 Settings 中连接它，然后启动新会话。向已运行的会话添加连接器也不会使其工具对 Claude 可用；启动新会话以获取新添加的连接器。

### 报告问题

对于自托管环境的问题，请联系您的 Anthropic 帐户团队。

## 故障排除

如需引导式诊断，请在运行器主机上运行 doctor 子命令。doctor 子命令启动一个交互式 Claude Code 会话，并附加运行器的日志和状态。首先在该主机上使用 `claude auth login` 登录，以便会话可以查询您的环境、其运行器和排队的会话。如果没有该登录（例如当主机使用 API 密钥进行身份验证时），它仅限于本地健康端点、指标和运行器日志，并且仅当您使用 `--log-file` 启动运行器时才读取日志。

```bash theme={null}
claude self-hosted-runner doctor
```

常见问题：

* **运行器未出现在环境中**：确认主机可以通过 HTTPS 到达 `api.anthropic.com`，并且环境密钥是最新的。运行器在身份验证失败时会记录 `[runner:fatal]` 和拒绝原因。
* **运行器在启动时退出，显示 `cannot create or write to base directory`**：运行器无法创建或写入 `--base-dir`，其默认值为 `/workspace`。修复目录的所有权或将 `--base-dir` 指向可写路径，如 [保持基础目录和容量在运行器之间相同](#keep-the-base-directory-and-capacity-identical-across-runners) 中所述。如果运行器改为记录 `[runner:fatal]` 说基础目录检查超时，则该目录位于挂起的 NFS 或 CSI 挂载上。检查挂载健康状况而不是权限。运行器在打开 `--log-file` 之前将这两个启动失败打印到 stderr，因此请在终端或您的平台的容器日志中查找它们，而不是日志文件。在 v2.1.225 之前，运行器在启动时不检查基础目录，此错误配置在拾取后失败会话。
* **运行器在启动时退出，显示 `RegisterRunner failed:` 和 TLS 证书错误**：例如，错误可能是 `self signed certificate in certificate chain`。运行器不信任其收到的证书背后的证书颁发机构（CA），在执行 TLS 检查的代理之后就会发生这种情况。在运行器的环境中将 [`NODE_EXTRA_CA_CERTS`](https://code.claude.com/docs/zh-CN/network-config#custom-ca-certificates) 设置为包含代理根 CA 的 PEM 文件。

  您无需再为会话设置它，会话会继承它。Git 不读取它：对于通过该代理进行的克隆，还需设置 [`GIT_SSL_CAINFO`](#trust-a-private-certificate-authority-with-anthropic-managed-git)。
* **您的启动脚本打印包含 `Resource temporarily unavailable` 的写入错误并退出**：您的脚本与其启动的 `claude self-hosted-runner` 或 [编排器](https://code.claude.com/docs/zh-CN/self-hosted-environments-configuration.md#on-demand-runners) 进程共享 stdout 或 stderr，而该进程已将该共享管道或套接字切换为非阻塞模式。当管道已满时，脚本的写入会因该操作系统错误而失败，而不是等待；在 `set -e` 下，脚本会退出。

  要消除该错误，请将运行器或编排器的输出发送到您脚本的用户可以创建的文件。在启动它的命令后追加 `>> "$HOME/claude-runner.log" 2>&1`，并放在任何将其置于后台运行的 `&` 之前，然后按照 [快速入门](https://code.claude.com/docs/zh-CN/self-hosted-environments-quickstart.md#set-up-manually) 中的说明替换 `<writable-dir>`：

  ```bash theme={null}
  claude self-hosted-runner --environment-secret-file /etc/claude/environment-secret --base-dir '<writable-dir>' >> "$HOME/claude-runner.log" 2>&1
  ```

  您脚本自身的输出仍然输出到原来的位置，而运行器写入 stdout 和 stderr 的内容（包括其启动错误）会写入该文件。请使用此 shell 重定向，而不是仅使用 [`--log-file`](https://code.claude.com/docs/zh-CN/self-hosted-environments-reference#runner-cli-flags)，后者会将日志镜像到文件，同时运行器仍继续写入 stdout 和 stderr。如果您已经传递了 `--log-file`，也请保留它，因为重定向不会改变该文件接收的内容。
* **编排器记录带有 TLS 证书错误的 `poll failed:` 并不断重试**：编排器不信任其收到的证书背后的 CA，在执行 TLS 检查的代理之后就会发生这种情况。在编排器的环境中将 [`NODE_EXTRA_CA_CERTS`](https://code.claude.com/docs/zh-CN/network-config#custom-ca-certificates) 设置为包含代理根 CA 的 PEM 文件。同时也为您的 `spawn-runner` hook 启动的运行器设置它。

  例如，一行可能以 `poll failed: self signed certificate in certificate chain — retrying in 1s` 结尾，冒号和破折号之间是 TLS 库给出的文字。要确认修复，请检查编排器的 [`/healthz`](https://code.claude.com/docs/zh-CN/self-hosted-environments-reference#health-endpoint) 正文中 `connected` 是否为 `true`。
* **会话保持排队**：每个在线运行器可能被锁定到不同的所有者。检查每个运行器的 `claude_code_self_hosted_runner_locked_account` [指标](https://code.claude.com/docs/zh-CN/self-hosted-environments-reference.md#prometheus-metrics) 或其 `[runner:health]` 日志行的 `locked_account` 字段，以查看谁持有它。两者仅在运行器被颁发携带 `act.email` 声明的会话令牌后才显示所有者的电子邮件，Claude Tag 代理的会话永远不会这样做。没有该声明，运行器不发出 `locked_account` 系列，并记录 `locked_account=yes`，这告诉您运行器被锁定但不知道是哪个所有者。添加副本，或等待现有运行器耗尽并重新启动。如果环境使用按需运行器，请改为检查编排器；请参阅 [按需运行器](https://code.claude.com/docs/zh-CN/self-hosted-environments-configuration.md#on-demand-runners)。
* **会话在拾取后立即失败**：在 claude.ai/code 中打开会话以查看错误。如果错误显示 git 无法进行身份验证，请检查运行器的 [git 凭据](#configure-git)。对于使用 `--use-anthropic-git-proxy` 启动的运行器，请参阅 [当会话在使用 git 代理的运行器上无法启动时](#when-anthropic-doesnt-serve-a-session)。

  不可写的基础目录会在启动时停止运行器，而不是使会话失败。请参阅此列表中的 **运行器在启动时退出，显示 `cannot create or write to base directory`** 条目。
* **会话已被拾取，但 Claude 从未回答其第一条消息**：如果您的运行器通过代理访问 `api.anthropic.com`，请将代理配置为在会话的 `/worker/events/stream` 响应到达时立即将其传递出去。提高运行器的超时时间没有帮助。要在运行器的日志中确认原因，请参阅 [会话已被拾取，但 Claude 从未回答其第一条消息](#when-a-sessions-first-message-is-never-answered)。
* **在设置了 `--use-anthropic-git-proxy` 的运行器上会话无法启动**：在运行器的日志中查找 `access denied by the git proxy`，或查找指明包含 `/git_proxy/` 的 `api.anthropic.com` 地址的 git 错误。要判断 Anthropic 是否提供了该会话并修复原因，请参阅 [当会话在使用 git 代理的运行器上无法启动时](#when-anthropic-doesnt-serve-a-session)。
* **会话无法通过身份验证出口代理到达网络**：当您使用 [`--proxy-authorization-command` 或 `--proxy-authorization-file`](#authenticate-to-an-egress-proxy) 设置的源失败、在 30 秒后超时或产生空值时，运行器以 `502 Bad Gateway` 应答该连接并记录原因。运行器在该日志中对命令的 stderr 进行脱敏，并且永远不会记录标头值。使用 `--proxy-authorization-command` 时，在主机上自己运行该命令以确认它在 stdout 上打印整个标头值。如果运行器改为在启动时退出，显示 `could not start the proxy-authorization listener`，则它无法打开其环回监听器。
* **运行器记录包含 `rejecting the malformed poll response` 的 `Poll failed` 行**：运行器收到的工作轮询响应的正文不是队列的预期 JSON。原因之一是拦截代理或强制门户用自己的页面进行了应答。如果运行器通过代理连接，请配置代理以将来自 `api.anthropic.com` 的响应原封不动地传递。

  运行器拒绝该格式错误的响应，在 `claude_code_self_hosted_runner_poll_errors_total` [指标](https://code.claude.com/docs/zh-CN/self-hosted-environments-reference.md#prometheus-metrics) 的 `transport` 类型下计数，并按 [会话生命周期](https://code.claude.com/docs/zh-CN/self-hosted-environments#session-lifecycle) 中描述的失败轮询计划重试。运行器继续为其实时会话提供服务。在 v2.1.246 之前，运行器将这样的响应读取为空工作队列，这可能会结束其实时会话或使其退出。
* **会话失败，显示 `no longer exists on the remote` 或 `couldn't find remote ref`**：运行器会将 git 的 `couldn't find remote ref` 理解为分支已被删除。但该分支可能仍然存在。请在您的 git 主机上按消息中的确切名称查找该分支，然后逐一检查符合情况的原因：

  * **分支已不存在**：请按消息的要求恢复该分支。
  * **分支存在，且您不管理该运行器**：将完整消息发送给运行器的管理员，并告知其该分支存在。
  * **分支存在，且运行器有 `checkout` hook**：您的 hook 中的某个 git 命令可能会在 hook 以非零状态退出之前将 `couldn't find remote ref` 打印到 stderr。找到该命令，并更正它请求的 ref 或它从中获取的远程。运行器会将会话请求的修订版本作为 [`CLAUDE_RUNNER_REPO_REF`](https://code.claude.com/docs/zh-CN/self-hosted-environments-configuration#checkout) 传递给您的 hook。
  * **分支存在，且运行器没有 `checkout` hook**：运行器会自行获取该分支。如果某个 [`--git-host-rewrite`](#rewrite-git-urls-for-private-networks) 适用于该仓库，请确认其重写到的目标主机上存在同名分支，如果不存在，请将该分支推送到那里。
  * **分支存在，且运行器早于 v2.1.301**：按照 [固定版本](#pin-the-version) 中的说明升级运行器，以便其他类型的失败显示其自身的错误。在 v2.1.301 之前，错误输出中包含 `couldn't find remote ref` 的其他失败也会被报告为分支已删除。

  升级运行器或修复原因后，请重试该会话或启动新会话。如果这些检查都未能找到原因，请[报告问题](#report-an-issue)。

  对于会话推送结果的源，删除的分支会导致会话失败。对于会话仅从中读取的 git 源，运行器跳过该源并继续处理其余源。当跳过会导致会话完全没有仓库时，运行器会使会话失败。在 v2.1.228 之前，这样的会话在空目录中启动。
* **Claude Tag 有时不在 Slack 中发布其回复**：回复提醒是一个[由控制平面提供的 hook](https://code.claude.com/docs/zh-CN/self-hosted-environments-configuration.md#how-each-session’s-config-is-assembled)。请使用 `claude --version` 确认运行器为 Claude Code v2.1.229 或更高版本，因为较旧的运行器会忽略所提供的 hook；确认 `python3 --version` 在运行器镜像中能够成功执行，因为这些 hook 会运行 `python3`；并确认没有任何设置文件关闭了 hook，无论是 `managed-settings.json`、运行器用户的 `~/.claude/settings.json`、仓库的 `.claude/settings.json` 或 `.claude/settings.local.json` 中的 `disableAllHooks`，还是托管设置中的 `allowManagedHooksOnly` 或涵盖 hook 的 [`strictPluginOnlyCustomization`](https://code.claude.com/docs/zh-CN/settings-reference#strictpluginonlycustomization)。
* **会话启动时缺少其中一个存储库**：在没有 [`checkout` hook](https://code.claude.com/docs/zh-CN/self-hosted-environments-configuration#checkout) 的运行器上，git 主机可能会拒绝运行器对会话仅从中读取的存储库的访问检查。运行器随后跳过该存储库，记录一条 `[runner:warn] could not access context source` 行，命名拒绝，并在其余存储库上启动会话。

  使用运行器自身的 git 凭据时，如果主机答复仓库未找到、git 找不到该主机的凭据，或身份验证失败，运行器可以跳过该仓库。

  使用 [`--use-anthropic-git-proxy`](#use-the-anthropic-git-proxy) 时，如果 git 代理以 `access denied by the git proxy` 拒绝某个仓库，运行器会跳过该仓库。此时日志行会显示 `blocked by the git proxy`。要了解需要检查的原因，请参阅 [当会话在使用 git 代理的运行器上无法启动时](#when-anthropic-doesnt-serve-a-session) 中的 `remote: access denied by the git proxy` 情况。

  网络故障、超时或任何其他 HTTP `403` 仍会导致会话启动失败，对会话推送结果的仓库的拒绝也是如此。如果跳过会导致会话完全没有仓库，运行器仍会使该会话失败。

  访问检查在每次会话在运行器上启动时再次运行，因此一旦运行器的 git 身份具有读取访问权限，下一次启动就会克隆存储库。在 v2.1.274 之前，这些拒绝中的每一个都导致会话启动失败。
* **日志显示 `could not read context source`**：与 `could not access context source` 不同，这一行表示 git 主机持续答复 HTTP `429`，或者您作为 `git` 安装的包装程序持续以 `75` 退出。运行器跳过了该仓库。要重试，请启动新会话。每次此会话在运行器上启动时，运行器也会再次尝试该仓库。

  在没有 `checkout` hook 的情况下，运行器可以跳过会话仅从中读取的仓库，除非该仓库是会话的第一个仓库。在 v2.1.311 之前，会话启动会失败。
* **会话启动失败，显示 `This runner couldn't read`**：运行器使用其 [git 凭据](#configure-git) 向 git 主机请求某个仓库，但得到了未找到的答复。该仓库不存在，或者这些凭据无法访问该仓库。
  * **如果您管理该运行器**：为其 git 凭据授予该仓库的读取权限，然后启动新会话。
  * **如果您启动了该会话**：如果该仓库不存在，请使用正确的仓库启动新会话。如果运行器无法访问该仓库，请让运行器的管理员授予访问权限，然后启动新会话。
* **会话需要数分钟才能启动或恢复**：原因之一是全新克隆。在每个运行器都从空磁盘启动、且没有 [`checkout` hook](https://code.claude.com/docs/zh-CN/self-hosted-environments-configuration#checkout) 的机群上，运行器会在启动 Claude Code 之前重新克隆会话的仓库。在空磁盘上恢复的会话会再次等待同样的克隆。

  要跳过全新克隆，请提供 [预热检出](#reuse-a-pre-warmed-checkout)。如果您已经提供了预热检出，请 [确认运行器使用了它](#confirm-that-the-runner-uses-your-clone)。如果运行器自行进行克隆并在一个会话后退出，请将 [`CLAUDE_RUNNER_FETCH_DEPTH`](https://code.claude.com/docs/zh-CN/self-hosted-environments-reference#environment-variable-only-settings) 从默认的 50 个提交调低，以获取更少的历史记录。

  如果您使用 hook，它会代替运行器的克隆运行，因此请改为检查您的 hook 执行的操作。观察 `claude_code_self_hosted_runner_session_init_duration_seconds` [指标](https://code.claude.com/docs/zh-CN/self-hosted-environments-reference.md#prometheus-metrics) 以了解启动需要多长时间。
* **轮次以 401 失败**：当轮次以来自 Anthropic API 的 401 或 403 结束时，运行器从 Anthropic 获取新的 [`CLAUDE_CODE_OAUTH_TOKEN`](https://code.claude.com/docs/zh-CN/self-hosted-environments-configuration#wrapper-scripts) 并将其传递给会话。失败的轮次不会重试。此令牌是短期的，运行器通过会话的 stdin 轮换它。

  当获取失败时，运行器记录一条 `inference_token refresh failed` 行，说明何时重试，并在会话运行期间继续重试。

  如果每个调用在会话大约 30 分钟后开始失败，包装脚本可能已断开会话的 stdin，因此令牌轮换无法到达它；请参阅 [保持 stdin 和文件描述符 3 附加](https://code.claude.com/docs/zh-CN/self-hosted-environments-configuration.md#keep-stdin-and-file-descriptor-3-attached)。

  在 v2.1.274 之前，运行器在几次尝试后停止重试失败的获取，并等待下一个计划的获取。失败的轮次不会触发获取，因此每个轮次都会失败，显示 401，直到下一个计划的获取。
* **轮次在 claude.ai/code 中失败，显示 `This model needs a newer version of Claude Code`**：您的运行器用于启动会话的 Claude Code 版本低于该会话的模型所要求的版本，因此 API [拒绝了该请求](https://code.claude.com/docs/zh-CN/errors.md#claude-code-does-not-support-this-model)。按照 [固定版本](#pin-the-version) 中的说明，将运行器升级到消息要求的版本或更高版本。在此之前，用户可以在会话中选择其他模型。
* **Pod 在耗尽中途被杀死**：将 `terminationGracePeriodSeconds` 提高到至少运行器在启动时记录的值。请参阅 [关闭时序](#shutdown-timing)。
* **运行器记录一条以 `was paused or too slow` 结尾的 `[runner:log]` 行**：运行器仍在运行，无需重新启动。在该行中指明的 stdout 或 stderr 流被再次读取之前，运行器丢弃了该流中除最新 256 KiB 之外的所有行。如果您传递了 [`--log-file`](https://code.claude.com/docs/zh-CN/self-hosted-environments-reference#runner-cli-flags)，请在该文件中查找被丢弃的行。
* **运行器记录 `Nobody can read this process's stdout anymore`**：读取运行器 stdout 的一方已不存在，因此运行器在其剩余运行期间不再向 stdout 写入，并继续运行。只要 stderr 仍处于打开状态，此行以及运行器之后的状态行（包括 `[runner:fatal]` 行）都会写入 stderr，因此请在 stderr 中读取它们。

初始化日志后，运行器将其生命周期日志（包括 `[runner:fatal]` 行）写入 stdout，将调试输出写入 stderr，全部作为纯文本行而不是 JSON。上述故障排除条目中描述的基础目录失败在该点之前打印到 stderr。使用 `--log-file` 捕获两个流，这也让 `self-hosted-runner doctor` 能够跟踪它们，或使用您的平台的日志收集。

每个会话的子进程写入单独的调试日志。失败时，运行器在 claude.ai/code 中将日志的尾部与会话一起显示。除非您使用 [`--remove-session-state`](https://code.claude.com/docs/zh-CN/self-hosted-environments-reference#runner-cli-flags) 启动了运行器，否则它也会在磁盘上保留失败会话的日志，并在运行器日志中打印其路径。

### 当运行器退出时

不要重新启动 [按需运行器](https://code.claude.com/docs/zh-CN/self-hosted-environments-configuration.md#on-demand-runners)，因为其工作订单是一次性的。在启动后立即退出的运行器需要与因任何其他原因退出的运行器不同的处理方式。

* **正常退出**：运行器完成了其会话并耗尽，达到了其退休时间，或被告知停止。重新启动它以便环境再次具有容量。[运行器生命周期](https://code.claude.com/docs/zh-CN/self-hosted-environments#runner-lifecycle) 描述了这些退出。
* **启动失败**：运行器无法使用给定的配置或主机启动，因此它在启动后几秒钟退出，并且每次重新启动时都以相同的方式退出。更快地重新启动它没有帮助。有人需要阅读其输出并修复原因。
* **失去联系**：无法连接 Anthropic 的时间超过其 [租约](https://code.claude.com/docs/zh-CN/self-hosted-environments#session-lifecycle) 的运行器（例如在其主机休眠期间）可能会被从环境中移除。被移除的运行器重新连接时会退出。其日志可能显示一条包含 `runner record gone server-side` 的 `[runner:fatal]` 行，或者在较长时间的中断之后显示 [`poll auth failed`](https://code.claude.com/docs/zh-CN/self-hosted-environments-quickstart#set-up-an-environment-and-runner)。运行器不会自行重新注册，因此请重新启动它。

配置您的监督程序在运行器退出时重新启动它，当运行器在启动后立即保持退出时等待更长时间，并在这种情况持续发生时告知某人。

对于生产环境，请让您的监督程序在全新的磁盘上、在新的容器或 VM 中重新启动运行器，正如 [加固您的部署](#harden-your-deployment) 所建议的那样。

#### 识别启动失败

当运行器无法启动时，它会打印一行说明原因，然后退出。对于大多数原因，该行包含 `[runner:fatal]`。对于某些原因，该行以 `error:` 开头，包括当运行器无法解析其标志、无法读取环境密钥或无法创建或写入基础目录时。下一行然后指向 `--help`。

大多数日志行以时间戳和 `[self-hosted-runner]` 开头，下面的示例省略了这些。例如，使用 Anthropic git 代理和容量大于 1 启动的运行器会打印如下一行：

```text theme={null}
[runner:fatal] --use-anthropic-git-proxy requires --capacity 1 (the proxy URL is per-session and linked worktrees share origin). Omit --use-anthropic-git-proxy or set --capacity 1.
```

在运行器的标准输出和标准错误、您的平台的容器日志或您使用 [`--log-file`](https://code.claude.com/docs/zh-CN/self-hosted-environments-reference#runner-cli-flags) 设置的文件中查找该行。运行器在打开日志文件之前打印 `error:` 行，因此请在终端或您的容器日志中查找它，如 [故障排除](#troubleshooting) 所述。

当您阅读启动失败时，这些也有帮助：

* **根本没有行**：主机杀死的运行器不会打印任何内容。如果输出以没有 `[runner:fatal]` 行和没有 `error:` 行结束，请检查主机或您的编排器是否停止了该进程，例如因为超过了内存限制。
* **退出代码**：运行器不会为在每次启动时重复的错误预留退出代码。它对配置错误（例如不支持的标志组合）和可以自行清除的失败（例如 API 通过运行器自己的重试保持不可达）退出相同的代码。根据运行器退出的速度快慢来决定是否等待更长时间，并阅读运行器的输出以了解原因。
* **看起来健康的环境**：某些启动步骤在运行器向您的环境注册后运行，例如 [`--configure-git`](#let-the-runner-configure-git) 和 Anthropic git 代理的凭证设置。如果其中一个步骤失败，环境可以在该进程退出后的几分钟内继续列出该运行器，并且 **Cloud environments** 页面可以读取 **Healthy**，而没有运行器拾取工作。如果会话在看起来健康的环境中保持排队，请检查您的监督程序是否在重新启动运行器。

#### 使用增长的等待时间重新启动

如何获得增长的等待时间取决于您的监督程序。

* **Kubernetes**：此页面上的 [Deployment](#kubernetes) 不需要更改。容器退出后，kubelet 默认在重新启动容器之前等待，并且等待时间在每次重新启动时增长到一个上限。一旦容器运行了一段时间而没有退出，等待就会重新开始。

  当容器仅运行很短时间时，kubelet 在正常退出后应用相同的等待。经常耗尽的运行器因此也可以显示 `CrashLoopBackOff` 状态，所以在得出运行器无法启动的结论之前请阅读输出。下面的命令从 Deployment 的一个 pod 读取最后一次运行的输出：

  ```bash theme={null}
  kubectl logs --previous -n claude-runners deploy/claude-runner
  ```

  当最后一次运行是启动失败时，`[runner:fatal]` 或 `error:` 行在输出的最后几行中。要读取另一个 pod 的最后一次运行，请在 `deploy/claude-runner` 的位置命名该 pod。
* **Docker 和 Docker Compose**：此页面上的 [Compose recipe](#docker-compose) 不需要更改。使用 `restart: always`，Docker 在保持退出的容器的每次重新启动之前等待更长时间，直到一个上限。在下面的命令中用容器的名称替换 `<container>`，该命令读取 Docker 重新启动容器的次数：

  ```bash theme={null}
  docker inspect --format '{{.RestartCount}}' <container>
  ```

  该命令打印一个数字。不断增加的数字意味着 Docker 不断重新启动运行器。
* **systemd 单元**：默认情况下，systemd 在每次重新启动之前等待相同的 `RestartSec`，并且不会延长它，因此具有 `Restart=always` 的单元以相同的间隔重新启动无法启动的运行器。当启动速度足够快以达到单元的启动速率限制（默认为 10 秒内 5 次启动）时，systemd 停止重新启动该单元。该单元保持停止状态，直到有人再次启动它，systemd 允许在速率限制的间隔已过或在 `systemctl reset-failed` 之后启动。因为 `RestartSec` 适用于每次重新启动，更长的值也会延迟正常退出后的重新启动。选择一个平衡两者的值，并对单元的重新启动计数进行警报。
* **shell 循环或您自己的监督程序**：自己应用相同的规则。从 5 秒的等待开始。在每次在一分钟内结束的运行之后，将下一次重新启动的等待加倍，最多 5 分钟。在运行了一分钟或更长时间的运行之后，回到 5 秒。

#### 检查运行器为什么保持退出

当运行器连续多次在启动后立即退出时，在再次重新启动之前停止并检查这些。

* **最后的 `[runner:fatal]` 或 `error:` 行**：它说明运行器停止的原因。[故障排除](#troubleshooting) 列出了常见原因。
* **标志的组合**：[Anthropic git 代理](#use-the-anthropic-git-proxy) 需要 `--capacity 1`。此页面上的配方使用更高的容量，因此当您将代理添加到其中一个时降低它。
* **服务的环境可以到达什么**：如果运行器手动启动并在您的监督程序下失败，请比较用户、主目录、`PATH` 和内存限制。`--configure-git` 和 Anthropic git 代理需要 `PATH` 上的 git 和可写的 `~/.gitconfig`。
* **环境密钥**：如果您撤销了密钥或输入错误，运行器会打印一行包含 `RegisterRunner auth failed`。
* **环境的 Activity 标签**：打开环境并选择 **Activity**。如果新运行器不断出现在那里，但没有拾取工作，您的监督程序正在重新启动运行器。

如需在运行器主机上进行引导式诊断，请运行 [doctor 子命令](#troubleshooting)。

### 会话已被拾取，但 Claude 从未回答其第一条消息

如果运行器拾取了会话，但 Claude 从未回答第一条消息，并且您的运行器通过代理访问 `api.anthropic.com`，则原因可能是该代理。Anthropic 通过一个保持打开的流式响应向会话发送其消息。如果您的代理缓冲该响应，或在传递任何内容之前就将其结束，第一条消息就永远不会到达。来自该会话的其他请求仍会成功。

要修复此问题，请更改代理处理此请求的方式：

```text theme={null}
GET https://api.anthropic.com/v1/code/sessions/<session-id>/worker/events/stream
```

* **不要缓冲该响应**：在响应到达时立即将其传递出去。其内容类型为 `text/event-stream`。
* **让该响应保持打开**：它最长可以保持打开一小时。如果代理有空闲超时，请将其设置为远高于 15 秒，因为默认情况下 Anthropic 至少每隔这么长时间发送一次保活消息。

然后启动一个新会话，并检查 Claude 是否回答。提高运行器的超时时间没有帮助：您只会在运行器停止会话之前等待更长时间。

#### 在运行器的日志中确认原因

这些步骤适用于 Claude Code v2.1.287 或更高版本、使用默认 `--log-level info` 的运行器。较旧的运行器会更晚记录这些行，或者根本不记录。

1. 等待 [`--startup-timeout-min`](https://code.claude.com/docs/zh-CN/self-hosted-environments-reference#runner-cli-flags) 过去，默认为 15 分钟。之后运行器会停止该会话，并记录一行包含 `no child output for` 且以 `— releasing` 结尾的内容。
2. 在运行器的日志中搜索 `SDKStartup: SSE`。这些行位于一条 `finished: interrupted` 行之后，在第 1 步中那一行之后的几行。大多数这样的行没有时间戳，因此日志收集器可能会将它们显示在其他位置。
3. 在下表中找到该行：

| 日志行包含 | 应采取的操作 |
| :- | :- |
| `SSE connect failed (no response in 30s)` 或 `SSE liveness timeout` | 检查代理是否转发该请求、是否在响应的标头和数据到达时立即将其传递，以及是否没有断开连接 |
| `SSE connect failed (HTTP` 和一个状态码 | 检查代理自身的日志。如果是代理自己应答了该请求，请让该请求通过代理。 |
| `SSE connect failed (` 和任何其他文本 | 括号中的文本就是错误。在代理自身的日志中查找同一个连接。 |
| `SSE stream ended by server`，反复出现 | 检查代理对长时间存在的响应的时间限制。单独一条这样的行是正常的。 |
| `SSE reconnected after` | 在其后查找其他行之一。这一行并不表示有任何内容到达了会话。 |

如果日志中没有这些行，或者代理的日志显示没有任何问题，请[报告问题](#report-an-issue)。

#### 阻止会话再次尝试

会话会将该消息保持为待处理状态，能够为该会话的所有者提供服务的运行器会再次拾取该会话。在您更改代理之前，该会话每次都会以相同的方式等待。要阻止这种情况，请在 claude.ai/code 中归档该会话。

## 接下来

* [自定义会话](https://code.claude.com/docs/zh-CN/self-hosted-environments-configuration.md)：包装脚本、生命周期钩子、按需运行器、MCP 服务器和权限
* [端到端测试](https://code.claude.com/docs/zh-CN/self-hosted-environments-testing.md)：在推广新运行器镜像之前从 CI 验证它
* [参考](https://code.claude.com/docs/zh-CN/self-hosted-environments-reference.md)：每个 CLI 标志、环境变量和指标
