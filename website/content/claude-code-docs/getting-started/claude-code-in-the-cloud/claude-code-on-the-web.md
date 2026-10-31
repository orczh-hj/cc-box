> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# 在云端使用 Claude Code

> 从浏览器、手机、桌面应用或终端在云端运行 Claude Code 会话，使用 `--cloud` 和 `--teleport` 移动会话，以及自动修复 Pull Request。

> [!NOTE]
>
> 云端会话在 Pro、Max、Ultra 和 Team 计划上可用，以及拥有高级席位或 Chat + Claude Code 席位的 Enterprise 用户。如果[自定义角色](https://support.claude.com/en/articles/13930452)管理您的访问权限，您还需要一个包含云端会话权限的角色，才能从浏览器、应用或终端在 Anthropic 的基础设施上启动会话。请让 Owner 在 [**Organization settings > Roles**](https://claude.ai/admin-settings/roles) 中将该权限添加到您的某个角色。

云端会话是在云基础设施上运行的 Claude Code 会话，而不是在您的机器上运行。默认情况下，它在 Anthropic 管理的基础设施上运行，或在路由到您的组织的[自托管环境](https://code.claude.com/docs/zh-CN/self-hosted-environments)时在那里运行。即使关闭笔记本电脑后，会话也会继续运行，您可以从任何设备检查或控制它。它会与您的其他 Claude 和 Claude Code 使用量一起计入您计划的用量限制，云端 VM 不会单独收费。

要让云端会话从 GitHub 克隆代码并推送分支，请使用其中一种 [GitHub 连接方式](#github-authentication-options)连接 GitHub。如果您的仓库位于 GitLab、Bitbucket 或其他托管平台上，请参阅[平台限制](#limitations)了解哪些功能可用。

你可以从以下任何界面启动云会话：

* **浏览器**：[claude.ai/code](https://claude.ai/code)，也称为网络上的 Claude Code
* **移动设备**：[Claude 应用](https://code.claude.com/docs/zh-CN/mobile.md)中的 **Code** 标签页
* **桌面应用**：在[启动会话](https://code.claude.com/docs/zh-CN/desktop.md#run-long-running-tasks-in-the-cloud)时，在环境选择器中选择 **Cloud** 下的某个环境
* **终端**：[`claude --cloud`](#from-terminal-to-cloud)
* **云端定时任务**：[定时运行和触发运行](https://code.claude.com/docs/zh-CN/routines.md)每次都作为云端会话运行

完成设置后，可使用本页在终端和云端之间移动工作、管理和共享会话、为 Pull Request 启用自动修复，以及进行故障排除。

> [!NOTE]
>
> 以下情况在其他页面中介绍：
>
> * **启动您的第一个云端会话**：[云端会话入门](https://code.claude.com/docs/zh-CN/web-quickstart.md)介绍如何连接 GitHub，并引导您在浏览器中完成一个任务
> * **为一项工作使用多个云端会话**：[项目](https://code.claude.com/docs/zh-CN/claude-projects.md)可让 Claude 为您启动并跟踪这些会话
> * **从其他设备控制本地会话**：在终端、IDE 或选择了 **Local** 的桌面应用中的会话在您的机器上运行，[Remote Control](https://code.claude.com/docs/zh-CN/remote-control.md) 让您可以从手机或浏览器访问这些会话

## 云环境

每个云端会话都在一个[云环境](https://code.claude.com/docs/zh-CN/cloud-environments.md)中运行，这是一个保存的配置，控制网络访问、环境变量和设置脚本。

* **您的第一个环境**：如果您还没有环境，入门流程会设置一个具有[**受信任**网络访问](https://code.claude.com/docs/zh-CN/cloud-environments.md#access-levels)的**默认**环境，要么为您创建它，要么要求您创建它。请参阅[默认环境](https://code.claude.com/docs/zh-CN/cloud-environments.md#the-default-environment)，了解在您的计划上会发生哪种情况
* **会话使用哪个环境**：请参阅[默认环境](https://code.claude.com/docs/zh-CN/cloud-environments.md#the-default-environment)，了解当您有多个环境时会话如何选择环境
* **更改会话可以访问的内容或在启动时运行的内容**：请参阅[配置云环境](https://code.claude.com/docs/zh-CN/cloud-environments.md)
* **无需任何配置即已安装的内容**：请参阅[已安装的工具](https://code.claude.com/docs/zh-CN/cloud-environments.md#installed-tools)

## GitHub 身份验证选项

云会话需要访问你的 GitHub 存储库来克隆代码和推送分支。你可以通过两种方式授予访问权限：

| 方法 | 如何连接 | 会话可以访问的存储库 | 最适合 |
| :- | :- | :- | :- |
| **GitHub App** | 在[网络快速入门](https://code.claude.com/docs/zh-CN/web-quickstart.md)期间授权 Claude GitHub App | 任何公开存储库，以及安装了 Claude GitHub App 的私有存储库 | 浏览器入门；想要[自动修复](#auto-fix-pull-requests)的团队 |
| **`/web-setup`** | 在终端中运行 `/web-setup` 以将本地 `gh` CLI 令牌发送到你的 Claude 账户 | 你的 `gh` 令牌可以访问的任何存储库，无论是否安装了 Claude GitHub App | 已经使用 `gh` 的个人开发者 |

以下功能依赖于仓库上已安装 Claude GitHub App：

* **自动修复**：在仓库上安装 Claude GitHub App 也会为其中的 Pull Request 启用[自动修复](#auto-fix-pull-requests)
* **项目**：[项目](https://code.claude.com/docs/zh-CN/claude-projects.md)中的线程需要在其克隆的每个仓库上安装 Claude GitHub App，无论您使用哪种方法连接。请参阅[设置 GitHub 访问权限](https://code.claude.com/docs/zh-CN/claude-projects.md#set-up-github-access)

在 Anthropic 托管的环境中，你的 GitHub 凭证在 Anthropic 的服务器上保持加密状态，永远不会进入会话的虚拟机。来自虚拟机的 GitHub 操作通过 [GitHub 代理](https://code.claude.com/docs/zh-CN/cloud-environments.md#github-proxy)进行，它在服务器端附加凭证。

有关 `/web-setup` 的分步说明（包括 `/web-setup` 存储的内容以及如何删除它），请参阅[从终端连接](https://code.claude.com/docs/zh-CN/web-quickstart.md#connect-from-your-terminal)。

> [!NOTE]
>
> 启用了[零数据保留](https://code.claude.com/docs/zh-CN/zero-data-retention.md)或应用了 [HIPAA 配置](https://code.claude.com/docs/zh-CN/hipaa-setup.md)的组织无法使用 `/web-setup` 或其他云端会话功能。使用[客户管理的加密密钥](https://platform.claude.com/docs/en/manage-claude/cmek)（CMEK）的组织也无法启动云端会话。

### 面向 Team 和 Enterprise 的快速设置

快速设置是一项组织设置，可减少成员在 GitHub 和环境设置中的步骤。在 Team 和 Enterprise 计划中，该设置默认处于关闭状态。

启用后，成员会看到以下变化：

* **`/web-setup`**：成员可以使用 `/web-setup` 连接 GitHub。该设置关闭时，此命令处于隐藏状态
* **GitHub App 提示**：浏览器入门引导会跳过 Claude GitHub App 安装提示
* **第一个环境**：浏览器入门引导会为成员创建 [**Default** 环境](https://code.claude.com/docs/zh-CN/cloud-environments.md#the-default-environment)，而不是显示环境表单

[所有者](https://code.claude.com/docs/zh-CN/server-managed-settings.md#access-control)可以在 [**Organization settings > Claude Code**](https://claude.ai/admin-settings/claude-code) 处通过 **Quick setup** 开关启用该设置。

## 在终端和云之间移动任务

这些工作流需要 [Claude Code CLI](https://code.claude.com/docs/zh-CN/quickstart.md) 登录到同一个 claude.ai 账户。您可以从终端启动新的云会话，或将云会话拉入终端以继续本地工作。云会话即使在您关闭笔记本电脑后也会持续存在，您可以从任何地方（包括 Claude 移动应用）监控它们。

> [!NOTE]
>
> 从 CLI 来看，会话切换是单向的：您可以使用 `--teleport` 将云端会话拉入终端，但无法将现有的终端会话推送到云端。带有任务描述的 `--cloud` 标志会为您当前的仓库创建新的云端会话；带有 `-p` 和会话 ID 或 claude.ai/code URL 时，它会改为[将消息排队到该现有会话](https://code.claude.com/docs/zh-CN/claude-code-on-the-web.md#send-follow-ups-from-the-cli)。[Desktop 应用](https://code.claude.com/docs/zh-CN/desktop.md#continue-in-another-surface)可以通过其 **Open in** 菜单，将其 Code 标签页中的本地会话发送到云端。

### 从终端启动云端会话

使用 `--cloud` 标志从命令行启动云会话：

```bash theme={null}
claude --cloud "Fix the authentication bug in src/auth/login.ts"
```

这会在 claude.ai 上创建新的云会话。云 VM 会在您当前分支克隆您当前目录的 GitHub 远程，而不是您的本地检出，因此如果您有本地提交，请先推送。有关 Claude Code 上传本地仓库而不是克隆的情况，请参阅 [发送没有 GitHub 的本地仓库](#send-local-repositories-without-github)。如果终端打印 `Branch <name> is not on GitHub`，请参阅 [Branch is not on GitHub](#branch-is-not-on-github)。

`--cloud` 一次只能与单个存储库配合使用。任务在云中运行，而您继续在本地工作。较旧的 `--remote` 拼写仍然可以作为 `--cloud` 的已弃用别名使用。

当云容器启动时，CLI 显示设置步骤的实时清单，例如克隆存储库和运行您的 [设置脚本](https://code.claude.com/docs/zh-CN/cloud-environments.md#setup-scripts)。它会将您在配置期间键入的消息排队，并在会话准备好后发送它们。

> [!NOTE]
>
> `--cloud` 创建云会话。`--remote-control` 无关：它让您能够从 claude.ai 或 Claude 应用监控和引导本地 CLI 会话。请参阅 [Remote Control](https://code.claude.com/docs/zh-CN/remote-control.md)。

在 claude.ai 或 Claude 移动应用上打开会话以检查进度或直接交互。从那里您可以引导 Claude、提供反馈或像在任何其他对话中一样回答问题。

如果 Claude 提出问题且会话处于空闲状态，您仍然可以在返回时回答，直到 [环境过期](#environment-expired)，会话会从您的答案继续。

#### 云任务提示

**在本地规划，在云中执行**：对于复杂任务，启动 Claude 处于规划模式以协作制定方法，然后将工作发送到云：

```bash theme={null}
claude --permission-mode plan
```

在规划模式下，Claude 读取文件、运行命令进行探索，并提出计划而不编辑源代码。一旦您满意，将计划保存到存储库、提交并推送，以便云 VM 可以克隆它。然后启动云会话以进行自主执行：

```bash theme={null}
claude --cloud "Execute the migration plan in docs/migration-plan.md"
```

**并行运行任务**：每个 `--cloud` 命令创建自己的云会话，独立运行。您可以启动多个任务，它们都会在单独的会话中同时运行：

```bash theme={null}
claude --cloud "Fix the flaky test in auth.spec.ts"
claude --cloud "Update the API documentation"
claude --cloud "Refactor the logger to use structured output"
```

当会话完成时，您可以从 claude.ai/code 创建 PR，或 [teleport](#from-cloud-to-terminal) 会话到您的终端以继续工作。

#### 发送没有 GitHub 的本地存储库

当您从没有 git 远程的仓库运行 `claude --cloud` 时，或从未安装 Claude GitHub App 的 github.com 仓库运行时，Claude Code 会将您的本地仓库打包并直接上传到云端会话。即使您使用 `/web-setup` 连接了 GitHub，这也适用。

对于完整克隆，该捆绑包包括您在所有分支上的仓库历史记录，加上对已跟踪文件的未提交更改。

敏感文件中的未提交更改如何处理取决于您的平台：

* **macOS、Linux 和 WSL**：Claude Code 会将名称类似于凭据或密钥的文件的未提交更改排除在上传之外。这涵盖 `.env` 文件、Terraform `*.tfvars` 文件以及密钥文件，例如 `id_rsa` 和 `*.pem`。它还会排除由 git 过滤器（例如 Git LFS）管理的文件的未提交更改。`Left on this machine:` 通知会列出被排除的文件，会话以每个文件的已提交版本启动，如果该文件没有任何已提交版本，则不包含该文件。
* **原生 Windows**：对已跟踪文件的未提交更改会按原样上传，无论文件名称如何。在启动云端会话之前，请先 stash 或还原您不希望出现在云端会话中的编辑。

要在 Claude Code 会克隆远程时上传捆绑包，请设置 `CCR_FORCE_BUNDLE=1`：

```bash theme={null}
CCR_FORCE_BUNDLE=1 claude --cloud "Run the test suite and fix any failures"
```

捆绑的存储库必须满足这些限制：

* 目录必须是具有至少一个提交的 git 存储库
* 捆绑的存储库必须小于 100 MB。较大的存储库会回退到仅捆绑当前分支，然后回退到工作树的单个压缩快照，如果快照仍然太大则失败
* 未跟踪的文件不包括在内；对您希望云会话看到的文件运行 `git add`
* 在 macOS、Linux 和 WSL 上，当 Claude Code 无法遵循影响哪些属性规则适用于您的文件的 git 设置时，它会拒绝上传，例如在包含的配置文件中设置的 `core.attributesFile`。[拒绝消息](https://code.claude.com/docs/zh-CN/errors.md#the-repository-upload-cant-follow-a-git-setting) 命名该设置和修复
* 从捆绑创建的会话只有在您的 [GitHub 连接](#github-authentication-options) 对该存储库具有推送访问权限时，才能推送回 GitHub 远程

在 macOS、Linux 和 WSL 上，上传还需要 git 2.31 或更高版本以及受支持的检出布局，而在原生 Windows 上，Claude Code 上传时不进行这两项检查。当检出不满足这些要求时，Claude Code 不会启动会话。它会打印一条包含 `Not uploading this working tree:` 的错误，指明原因并说明需要更改的内容。以下是常见原因：

* **较旧的 git**：已安装的 git 早于 2.31。请更新 git，然后重试。
* **上传不支持的检出布局**：您在子模块内启动、在使用 `git clone --separate-git-dir`、`--shared` 或 `--reference` 创建的克隆中启动、在设置了 `core.worktree` 的检出中启动，或在以 reftable 格式保存 refs 的仓库中启动。请改为从使用普通 `git clone` 创建的克隆的主检出启动。
* **带有稀疏检出的链接 worktree**：`git sparse-checkout` 会将设置写入该 worktree 自己的 `config.worktree` 文件，而上传不接受该文件，因此具有这些设置的 worktree 不会被上传，Claude Code 使用 [`worktree.sparsePaths`](https://code.claude.com/docs/zh-CN/settings-reference#worktree-sparsepaths) 创建的 worktree 也不会被上传。请改为从仓库的主检出启动。
* **保存在工作树内的 git 配置**：您的 git 配置包含位于检出内部的文件，例如指向仓库内部的 `include.path` 条目。请将该文件移到工作树之外或删除该 include，然后重试。

在 macOS、Linux 和 WSL 上，使用 `git clone --filter` 创建的部分克隆会作为其工作树的快照上传，不包含历史记录，前提是该克隆在本地拥有每个已跟踪文件。

对于 `claude --cloud`，如果仓库位于 GitHub 上，您可以避免上传及其要求：推送您的分支，在仓库上安装 Claude GitHub App，然后再次启动会话，使其从 GitHub 克隆。

### 从 CLI 发送后续消息

一旦云端会话运行，无论它在哪里执行，都可以从任何您使用 `claude auth login` 登录的机器上的 `claude` CLI 向它发送后续消息。CLI 使用您的 Anthropic 账户凭据进行身份验证，不发送任何本地会话状态，因此该命令不需要从启动会话的机器运行。

该命令发布一条消息并退出：

```bash theme={null}
claude -p "your message" --cloud <session-id>
```

CLI 将消息排队到会话中并退出，无需等待回复。使用它来引导长时间运行的会话、在当前步骤仍在完成时排队下一步，或从 [CI 脚本](https://code.claude.com/docs/zh-CN/self-hosted-environments-testing.md#run-the-test-loop) 发送后续消息。您也可以在 stdin 上管道消息，而不是将其作为参数传递：`echo "your message" | claude -p --cloud <session-id>`。

对于 `<session-id>`，传递裸 ID，例如 `session_...` 或 `cse_...`，或会话的 `claude.ai/code/<id>` URL，带或不带方案或查询字符串。在 claude.ai/code 的会话列表中找到 ID。

> [!NOTE]
>
> `--cloud` 需要 Anthropic 账户。当 Claude Code 为 Amazon Bedrock、Google Cloud 的 Agent Platform 或其他第三方提供商配置时，它不可用。仅通过 `ANTHROPIC_BASE_URL` 配置的 [LLM 网关](https://code.claude.com/docs/zh-CN/llm-gateway) 不算作此检查的第三方提供商，但您仍需要使用 `claude auth login` 登录。您组织的 `allow_remote_sessions` 策略也必须启用。所有者可以在 claude.ai/admin-settings/claude-code 的 Claude Code 管理设置中打开它。

#### 输出

成功时，该命令打印会话 ID 和查看会话的链接：

```
Sent to cloud session.
Session ID: session_01DiUkqY2kzbUbDmW1w96rfi
View: https://claude.ai/code/session_01DiUkqY2kzbUbDmW1w96rfi?from=cli&m=0
```

传递 `--output-format json` 以获得机器可读的结果：成功时为 `{ok, session_id, url}`，或发送失败时为 `{ok: false, session_id, error}`，例如当会话缺失或已存档时。配置错误（例如不支持的提供商或禁用的组织策略）打印到 stderr，不带 JSON。`--output-format stream-json` 不支持 `--cloud <session-id>`。

如果发送失败，请参阅[发送到云端会话时的错误](#errors-when-sending-to-a-cloud-session)。

### 在终端中继续云端会话

使用以下任何方法将云会话拉入您的终端：

* **使用 `--teleport`**：从命令行运行 `claude --teleport` 以获得交互式会话选择器，或 `claude --teleport <session-id>` 以直接恢复特定会话。如果您有未提交的更改，系统会提示您先隐藏它们。[`--resume`](https://code.claude.com/docs/zh-CN/sessions#resume-a-session) 从此机器的本地历史记录重新打开对话，不列出云端会话。
* **使用 `/teleport`**：在现有 CLI 会话内，运行 `/teleport` 或 `/tp` 以打开相同的会话选择器，无需重启 Claude Code。
* **从 `/tasks`**：运行 `/tasks` 以查看您的后台会话，然后按 `t` 以 teleport 到其中一个。
* **从 claude.ai/code**：从会话菜单中选择 **Open in > Terminal** 以复制可以粘贴到终端的命令。
* **从云会话内部**：键入 `/teleport`，Claude Code 会回复该会话的确切 `claude --teleport <session-id>` 命令，准备从存储库的检出运行。需要会话环境中的 Claude Code v2.1.223 或更高版本。

当您 teleport 会话时，Claude 验证您在正确的仓库中，从云端会话获取并检出分支，并将对话加载到您的终端。

> [!NOTE]
>
> 如果 Claude Code 已对云端会话进行了[压缩](https://code.claude.com/docs/zh-CN/context-window.md#what-survives-compaction)（在较长的会话中会自动进行），则被其汇总的较早消息不会复制到您的终端。Claude 会从摘要继续。要阅读原始消息，请在 claude.ai 或 Claude 移动应用的云端会话中向上滚动。

终端获得会话的自己的副本：那里的新工作保持本地，不会出现在 claude.ai 上的云端会话或 Claude 移动应用中。要在 teleport 后继续从您的手机引导，请在本地会话中启动 [`/remote-control`](https://code.claude.com/docs/zh-CN/remote-control)。

#### Teleport 要求

Teleport 在恢复会话前检查这些要求。如果任何要求未满足，您会看到错误或被提示解决问题。

| 要求 | 详情 |
| - | - |
| 干净的 git 状态 | 您的工作目录必须没有未提交的更改。如果需要，Teleport 会提示您隐藏更改。 |
| 正确的仓库 | 您必须从同一仓库的检出运行 `--teleport`，而不是 fork。如果您从不同仓库的检出运行它，Claude Code 会显示一个错误，指明会话的仓库和您的检出的仓库。如果 Claude Code 无法将您的远程解析为主机名，例如 SSH 主机别名如 `git@work:owner/repo.git`，它会要求您确认，并在远程的所有者和仓库名称与会话的仓库匹配时接受检出。 |
| 分支可用 | 来自云会话的分支必须已推送到远程。Teleport 会自动获取并检出它。 |
| 相同账户 | 您必须使用云会话中使用的相同 claude.ai 账户进行身份验证。 |

当 teleport 获取会话的分支时，获取操作永远不会在您的终端中等待输入。如果 git 或 ssh 需要询问密码、密钥口令或确认新的 SSH 主机，获取就会失败，此时只有当您的本地克隆已包含该分支时，检出才能成功。对于两种 SSH 情况，请将您的密钥加载到 `ssh-agent` 中，并先手动运行一次 `git fetch` 以记录该主机。

#### `--teleport` 不可用

Teleport 需要 claude.ai 订阅身份验证。请找到与您相符的情况：

* **您使用 API 密钥或 Console 账户登录**：在您的 shell 中运行 `claude auth login`，然后使用 claude.ai 登录
* **错误指明了您的提供商**：云端会话无法通过第三方提供商使用。请参阅[错误表](#errors-when-sending-to-a-cloud-session)
* **您已通过 claude.ai 登录**：您的组织可能已禁用云端会话

## 处理会话

会话出现在 claude.ai/code 的侧边栏中。从那里你可以审查更改、与队友共享、归档完成的工作或永久删除会话。

### 云端会话中的权限模式

无论是在创建任务时还是在会话运行期间，您都可以从[模式下拉菜单](https://code.claude.com/docs/zh-CN/permission-modes.md#switch-permission-modes)为云端会话选择[权限模式](https://code.claude.com/docs/zh-CN/permission-modes.md)。

在以下任一情况下，Claude Code 会以会话原先所处的权限模式恢复该会话：

* 重新打开 Anthropic 托管的[环境已过期](#environment-expired)的会话
* 向自托管运行程序[在空闲时释放](https://code.claude.com/docs/zh-CN/self-hosted-environments-reference.md#runner-cli-flags)的会话发送消息

### 审查更改

Claude 将其更改推送到分支后，聊天旁面板中的 **Changes** 卡片会显示添加和删除的行数，例如 `+42 -18`。点击 **Changes** 卡片上的行数即可打开 diff 视图。

* 您也可以点击会话标题栏中的 **Changes**，或标题栏 **⋮** 菜单中的该选项。
* 面板隐藏时，要显示面板，请点击会话标题栏中的 **Show session rail** 图标，或标题栏 **⋮** 菜单中的该选项。

在 diff 视图中，您可以对行添加评论，并与其他分支进行比较：

* **评论**：在特定行上留下内联评论，然后随下一条消息将这些评论发送给 Claude
* **比较**：diff 视图默认将会话的更改与其基础分支进行比较。要与仓库中的任何其他分支进行比较，请选择 **Compare against** 并选择一个分支

Claude Code 根据原始 git blob 内容计算这些 diff，因此仓库中配置的 diff 驱动程序和 `textconv` 过滤器不适用。

以下步骤在其他文档中介绍：

* **完整演练（包括 PR 创建）**：请参阅[审查和迭代](https://code.claude.com/docs/zh-CN/web-quickstart.md#review-and-iterate)
* **让 Claude 自动监控 PR 的 CI 失败和审查评论**：请参阅[自动修复 Pull Request](#auto-fix-pull-requests)

#### 排查 diff 视图中缺失的文件

当 diff 视图与基础分支进行比较时，无论会话是否已推送更改，您预期的某个文件都可能未出现在 diff 视图的文件列表中：

* **尚未推送更改**：列表包含 Claude 自己所做的编辑，但不包含 Claude 运行的命令（例如格式化工具）所做的更改
* **已推送更改**：列表包含已推送的提交，但不包含上次推送之后所做的更改

要将缺失的更改添加到列表中，请让 Claude 提交并推送。

### 管理上下文

云会话支持产生文本输出的[内置命令](https://code.claude.com/docs/zh-CN/commands.md)。仅在终端界面中运行的命令，如 `/plugin` 或 `/resume`，不可用。在云会话中打开选择器或面板的命令表现不同：

* **`/model`、`/effort`、`/color` 和 `/rename`**：将值作为参数传递，例如 `/model sonnet`，而不是打开终端选择器或滑块。参数形式遵循每个命令的[可用性说明](https://code.claude.com/docs/zh-CN/commands.md#all-commands)。
* **`/fast`**：当快速模式在[你的账户上可用](https://code.claude.com/docs/zh-CN/fast-mode.md#requirements)时，为会话切换[快速模式](https://code.claude.com/docs/zh-CN/fast-mode.md#use-fast-mode-in-cloud-sessions)。需要会话环境中的 Claude Code v2.1.271 或更高版本。
* **`/config`**：在浏览器中访问 claude.ai/code 时，此命令会打开 **Settings > Code** 或 **Settings > Claude Code**，而不是设置某个值。要更改云端会话的设置，请在环境中设置[环境变量](https://code.claude.com/docs/zh-CN/cloud-environments.md#set-environment-variables)。在[项目](https://code.claude.com/docs/zh-CN/claude-projects.md)的线程之外且仅包含一个仓库的会话中，请改为将该键提交到该仓库的 `.claude/settings.json` 中。[云端会话中的设置](https://code.claude.com/docs/zh-CN/settings.md#settings-in-cloud-sessions)列出了每个会话读取的内容。

对于[项目线程](https://code.claude.com/docs/zh-CN/claude-projects.md#choose-models-and-let-claude-manage-context)之外的云端会话中的上下文管理：

| 命令 | 在云会话中工作 | 注释 |
| :- | :- | :- |
| `/compact` | 是 | 总结对话以释放上下文。接受可选的焦点指令，如 `/compact keep the test output` |
| `/context` | 是 | 显示当前在上下文窗口中的内容 |
| `/clear` | 是 | 在同一云端会话中以空上下文开始新的对话 |

自动压缩在上下文窗口接近容量时自动运行。云会话自己设置 [`CLAUDE_AUTOCOMPACT_PCT_OVERRIDE`](https://code.claude.com/docs/zh-CN/env-vars)，所以压缩在[自动压缩窗口](https://code.claude.com/docs/zh-CN/model-config.md#set-the-auto-compact-window)的中途触发，而不是当窗口填满时。该值覆盖你在[环境变量](https://code.claude.com/docs/zh-CN/cloud-environments.md#set-environment-variables)中添加的值，所以在那里添加变量不会改变压缩何时触发。

要改为更改自动压缩窗口，请在你的环境变量中设置 [`CLAUDE_CODE_AUTO_COMPACT_WINDOW`](https://code.claude.com/docs/zh-CN/env-vars)，或在变量未设置的会话中运行带有令牌计数的 [`/autocompact`](https://code.claude.com/docs/zh-CN/commands#all-commands)。

[子代理](https://code.claude.com/docs/zh-CN/sub-agents.md)的工作方式与本地相同。Claude 可以使用 Agent 工具生成子代理，将研究或并行工作分流到单独的上下文窗口中，使主对话更轻量。在[项目](https://code.claude.com/docs/zh-CN/claude-projects.md#what-threads-pick-up-from-your-repositories)的线程之外，仓库 `.claude/agents/` 中定义的子代理会自动加载。

[Agent teams](https://code.claude.com/docs/zh-CN/agent-teams.md) 默认关闭，但可以通过将 `CLAUDE_CODE_EXPERIMENTAL_AGENT_TEAMS=1` 添加到你的[环境变量](https://code.claude.com/docs/zh-CN/cloud-environments.md#set-environment-variables)来启用。

### 对话中的插件建议

如果对话中出现一张列出一个或多个[插件](https://code.claude.com/docs/zh-CN/plugins/overview.md)并带有 **Add** 按钮的卡片，则表示 Claude 正在为您的任务建议插件。Claude 可能会在您未提出请求的情况下给出这些建议。您无需回应，无论如何 Claude 都会继续执行任务。

#### 添加或移除建议的插件

在插件上选择 **Add**，即可为您的 claude.ai 账户启用该插件，会话结束后该插件仍保持启用。要撤销，请从卡片中打开已添加的插件，选择 **Manage** 进入该插件在 claude.ai 上的页面，然后从该页面移除插件。

#### 关闭插件建议

要阻止 Claude 建议插件，请使用[权限规则](https://code.claude.com/docs/zh-CN/permissions.md#manage-permissions)拒绝 `SearchPlugins` 和 `SuggestPluginInstall` 工具。在[项目](https://code.claude.com/docs/zh-CN/claude-projects.md)之外且仅包含一个仓库的会话中，请将这些规则提交到该仓库的 `.claude/settings.json`，并在 `permissions.deny` 下列出这两个工具名称：

```json theme={null}
{
  "permissions": {
    "deny": ["SearchPlugins", "SuggestPluginInstall"]
  }
}
```

当适用于该会话的[托管设置](https://code.claude.com/docs/zh-CN/managed-settings.md)设置了 [`allowManagedPermissionRulesOnly`](https://code.claude.com/docs/zh-CN/settings-reference#allowmanagedpermissionrulesonly) 时，Claude Code 会忽略仓库 `.claude/settings.json` 中的权限规则，包括这些拒绝规则。在这种情况下，请改为将规则放入这些托管设置中。对于其他配置，例如包含多个仓库的会话或项目中的云端线程，请参阅哪些[设置会作用于云端会话](https://code.claude.com/docs/zh-CN/settings.md#settings-in-cloud-sessions)。

### 取回已排队的消息

如果您在 Claude 工作时发送消息，该消息会排队，直到 Claude 读取它。要取回已排队的消息，请点击消息上的 ✕。文本会返回到消息框，以便您编辑或发送其他内容。

如果 Claude 已经读取了该消息，它会保留在对话中。

### 共享会话

要共享会话，请根据下面的账户类型切换其可见性。之后，按原样共享会话链接。打开链接时，收件人会看到最新状态，但他们的视图不会实时更新。

#### 从 Enterprise 或 Team 账户共享

Enterprise 和 Team 账户的共享方式如下：

* **可见性选项**：**私有**和**团队**。团队可见性使会话对您的 claude.ai 组织中的其他成员可见
* **仓库访问权限**：默认启用验证，基于收件人账户所连接的 GitHub 账户
* **您的名称**：您账户的显示名称对所有有访问权限的收件人可见
* **Slack 会话**：[Claude in Slack](https://code.claude.com/docs/zh-CN/slack.md) 会话会自动以团队可见性共享

#### 从 Max、Ultra 或 Pro 账户共享

* **可见性选项**：**私有**和**公开**。公开可见性使会话对任何登录 claude.ai 的用户可见
* **仓库访问权限**：默认不启用验证
* **敏感内容**：共享前请检查您的会话。会话可能包含来自私有 GitHub 仓库的代码和凭据

要要求收件人拥有仓库访问权限，或从共享会话中隐藏您的名称，请在 claude.ai 上打开 Settings，然后：

* **如果 Settings 菜单显示的是 Code 而不是 Claude Code**：前往 [**Privacy**](https://claude.ai/settings/data-privacy-controls)，其中包含这些设置
* **否则**：前往 [**Claude Code > Sharing settings**](https://claude.ai/settings/claude-code)

### 归档会话

归档会话可将其从侧边栏的会话列表中移除。要归档会话，请在侧边栏中将鼠标悬停在该会话上，打开其 **⋯** 菜单，然后选择 **Archive**。

#### 查找已归档的会话

要查看已归档的会话，请前往 [claude.ai/code/sessions](https://claude.ai/code/sessions) 并打开 **Archived** 标签页。要将某个会话放回侧边栏的会话列表，请在该标签页上将鼠标悬停在该会话上，打开其 **⋮** 菜单，然后选择 **Unarchive**。

### 删除会话

删除会话会永久删除会话及其数据。此操作无法撤销。您可以从侧边栏、会话菜单删除会话；如果会话已归档，也可以从 **Archived** 标签页删除：

* **从侧边栏**：将鼠标悬停在会话上，打开其 **⋯** 菜单，然后选择 **Delete**
* **从会话菜单**：打开会话，选择会话标题旁的下拉菜单，然后选择**删除**
* **从 Archived 标签页**：在 [**Archived** 标签页](#find-an-archived-session)上将鼠标悬停在会话上，打开其 **⋮** 菜单，然后选择 **Delete**

删除会话前会要求你确认。

## 自动修复拉取请求

Claude 可以监视拉取请求并自动响应 CI 失败和审查评论。Claude 订阅 PR 上的 GitHub 活动，当检查失败或审查者留下评论时，Claude 会调查并推送修复（如果有明确的修复）。

> [!NOTE]
>
> 自动修复需要在你的存储库上安装 Claude GitHub App。如果你还没有，请从 [GitHub App 页面](https://github.com/apps/claude) 安装它。

根据 PR 来自何处以及你使用的设备，有几种方法可以打开自动修复：

* **在云端会话中创建的 PR**：打开 claude.ai/code 处的会话，在 [**Changes** 卡片](#review-changes)中找到该拉取请求。打开拉取请求标题旁边的 **⋮** 菜单并启用 **Auto-fix**
* **从终端**：在 PR 的分支上运行 [`/autofix-pr`](https://code.claude.com/docs/zh-CN/commands)。Claude Code 使用 `gh` 检测打开的 PR，生成云会话，并一步启用自动修复
* **从移动应用**：告诉 Claude 自动修复 PR，例如"watch this PR and fix any CI failures or review comments"
* **任何现有 PR**：将 PR URL 粘贴到会话中并告诉 Claude 自动修复它

要停止监视，请在会话的 [**Changes** 卡片](#review-changes)中，通过拉取请求的 **⋮** 菜单关闭 **Auto-fix**，或告诉 Claude 停止监视该 PR。

### Claude 如何响应 PR 活动

当自动修复处于活动状态时，Claude 接收 PR 的 GitHub 事件，包括新的审查评论和 CI 检查失败。对于每个事件，Claude 调查并决定如何进行：

* **明确的修复**：如果 Claude 对修复有信心且不与早期指令冲突，Claude 会进行更改、推送它，并在会话中解释所做的工作
* **模糊的请求**：如果审查者的评论可以以多种方式解释或涉及架构上重要的内容，Claude 会在采取行动前询问你
* **重复或无操作事件**：如果事件是重复的或不需要更改，Claude 会在会话中记录它并继续

GitHub 不会在基础分支推进并创建合并冲突时发出 webhook，因此自动修复无法自行对冲突做出反应。要解决冲突，请打开会话并要求 Claude 进行变基。

Claude 可能会作为解决审查评论线程的一部分在 GitHub 上回复它们。这些回复使用你的 GitHub 账户发布，所以它们出现在你的用户名下，但每个回复都标记为来自 Claude Code，以便审查者知道它是由代理编写的，而不是由你直接编写的。

> [!WARNING]
>
> 如果你的存储库使用注释触发的自动化，例如 Atlantis、Terraform Cloud 或在 `issue_comment` 事件上运行的自定义 GitHub Actions，请注意 Claude 可以代表你回复，这可能会触发这些工作流。在启用自动修复之前审查你的存储库的自动化，并考虑为可能部署基础设施或运行特权操作的 PR 注释的存储库禁用自动修复。

## 安全和隔离

每个云端会话通过多个层与您的机器和其他会话分离：

* **隔离的虚拟机**：每个会话在隔离的、Anthropic 管理的 VM 中运行。您的组织路由到[自托管环境](https://code.claude.com/docs/zh-CN/self-hosted-environments)的会话改为在您自己的基础设施上运行，其中隔离是您的部署的责任
* **网络访问控制**：在 Anthropic 托管的环境中，网络访问默认受限，可以禁用。请参阅[网络访问](https://code.claude.com/docs/zh-CN/cloud-environments.md#network-access)了解访问级别、[默认允许的域](https://code.claude.com/docs/zh-CN/cloud-environments.md#default-allowed-domains)和不通过允许列表的流量。在自托管环境中，您在自己的网络边界处限制会话出口。当在禁用网络访问的情况下运行时，Claude Code 仍然可以与 Anthropic API 通信，这可能允许数据从 VM 中退出。
* **凭据保护**：在 Anthropic 托管的环境中，git 凭据和签名密钥保持在沙箱外，代理使用作用域凭据代表会话进行身份验证。在自托管环境中，您的部署提供 git 凭据；请参阅[配置 git](https://code.claude.com/docs/zh-CN/self-hosted-environments-deploy.md#configure-git)
* **网络密钥**：在 Anthropic 托管的环境中，您[添加到云环境](https://code.claude.com/docs/zh-CN/cloud-environments.md#add-network-secrets)的密钥以相同的方式保持在沙箱外，在请求离开会话后附加到匹配的请求。自托管环境没有网络密钥
* **安全分析**：代码在会话的隔离环境内分析和修改，然后创建 PR

## 故障排除

对于出现在对话中的运行时 API 错误，如 `API Error: 500`、`529 Overloaded`、`429` 或 `Prompt is too long`，请参阅[错误参考](https://code.claude.com/docs/zh-CN/errors.md)。这些错误及其修复与 CLI 和 Desktop 应用共享。下面的部分涵盖特定于云会话的问题。

### 会话创建失败

如果新会话无法启动，显示 `Session creation failed` 或在配置时停滞，Claude Code 无法为会话分配 VM。

* 检查 [status.claude.com](https://status.claude.com) 以了解云会话事件
* 一分钟后重试，因为容量是按需配置的
* 通过遵循[连接 GitHub 后没有存储库出现](https://code.claude.com/docs/zh-CN/web-quickstart.md#no-repositories-appear-after-connecting-github)来确认你的 GitHub 连接可以访问存储库

### Your new session couldn't start

您的提示词已返回到您发送它时所用的输入框中。

* **连接中断**：请检查您的网络连接以及您使用的任何 VPN 或代理，然后重新发送您的提示词
* **服务器错误**：请稍等片刻，然后重新发送您的提示词。如果该通知再次出现，请检查 [status.claude.com](https://status.claude.com)

### Can't rewind to this message

当您的会话在 Claude Code 版本早于 v2.1.246 的[自托管运行器](https://code.claude.com/docs/zh-CN/self-hosted-environments)上运行时，对除最新消息之外的任何消息执行 **Rewind to here** 都会因此错误而失败。请让负责运行您组织运行器的人员[升级它们](https://code.claude.com/docs/zh-CN/self-hosted-environments-deploy.md#pin-the-version)。

### 无法获取组织 UUID

`claude --cloud` 和 `claude --teleport` 需要使用 claude.ai 账户登录。如果您改为使用 API 密钥或 Console 登录进行身份验证，或者存储的账户详情已过期，您会看到以下消息之一：

* `Unable to get organization UUID`
* ``Cloud sessions need a claude.ai sign-in. Run `claude auth login` (or /login in a local session), then try again.``
* ``Cloud sessions aren't available with this Console sign-in. They need a claude.ai sign-in. Run `claude auth login` (or /login in a local session), then try again.``
* 在不带会话 ID 运行 `claude --teleport` 时，会话选择器中显示 `Error loading Claude Code sessions`

在 shell 中运行 [`claude auth login`](https://code.claude.com/docs/zh-CN/cli-reference#cli-commands) 以使用您的 claude.ai 账户登录，然后重试该命令。在正在运行的会话中，`/login` 的作用相同。如果错误中提到的是您的提供商，请参阅[错误表](#errors-when-sending-to-a-cloud-session)：云端会话无法通过第三方提供商使用。

在 v2.1.274 至 v2.1.289 版本中，登录消息为 `Claude Code cloud sessions require authentication with a Claude.ai account. API key authentication is not sufficient. Please run /login to authenticate, or check your authentication status with /status.`

### 分支不在 GitHub 上

当 `claude --cloud` 打印以 `Branch <name> is not on GitHub` 开头的一行时，云端会话仍会启动，但不是从您的分支启动。该行会指出会话改为从何处启动：您分支历史中一个已推送的提交，或仓库的默认分支。仅存在于您本机上的提交以及未提交的更改不会出现在会话中。

例如，当您从未推送该分支，或者 GitHub 在其 Pull Request 合并后删除了该分支时，您会看到这一行。如果该行指出的起点包含您的任务所需的内容，请直接使用该会话。否则，请推送该分支并启动一个新会话，使其克隆您的分支。

### Remote Control 会话已过期或访问被拒绝

`--teleport` 通过与云会话使用的相同 Remote Control 会话基础设施连接，所以身份验证和会话过期错误会显示 Remote Control 措辞。你可能会看到 `Remote Control session expired` 或 `Access denied`。连接令牌是短期的，并限定于你的账户。

* 在本地运行 `/login` 以刷新你的凭证，然后重新连接
* 确认你已登录到拥有会话的相同账户
* 如果你看到 `Remote Control may not be available for this organization`，所有者尚未为你的组织启用云会话

### 向云端会话发送消息时的错误

这些错误来自使用 [`--cloud <session-id>`](#send-follow-ups-from-the-cli) 运行 `claude`，无论是否带有 `-p`。CLI 会在错误前加上 `Error: ` 前缀。发送失败时会包装为 `failed to send message to cloud session <id>: <reason>`。

| 消息 | 含义 |
| - | - |
| `Cloud sessions aren't available with <provider>. They run on Anthropic's infrastructure and require an Anthropic account.` | Claude Code 配置为使用第三方提供商。消息会使用您的配置中的标签显示提供商名称，例如 `Amazon Bedrock` 或 `Google Vertex AI`。请移除该提供商的配置（例如取消设置 `CLAUDE_CODE_USE_BEDROCK`），并使用 Anthropic 账户登录（`claude auth login`）。 |
| `Cloud sessions are disabled by your organization's policy. Contact your organization admin to enable them.` | 您组织的 `allow_remote_sessions` 策略已关闭；请参阅[Cloud sessions are disabled by your organization's policy](https://code.claude.com/docs/zh-CN/errors.md#cloud-sessions-are-disabled-by-your-organizations-policy)。如果消息中提到的是此会话正在使用的某个凭据，请参阅[云端会话使用您的 claude.ai 登录](https://code.claude.com/docs/zh-CN/errors.md#cloud-sessions-use-your-claude-ai-login)。 |
| `Couldn't verify your organization's policy for cloud sessions. Check your network connection and try again.` | Claude Code 尚未完成加载您组织的策略，因此拒绝发送，而不是假定云端会话已被允许。请检查网络连接并重试。 |
| `Cloud sessions aren't available:` 后跟原因 | 加载您组织的策略失败，消息的其余部分说明了失败方式。请根据其中指出的原因采取相应措施；请参阅[Couldn't verify your organization's policy for cloud sessions](https://code.claude.com/docs/zh-CN/errors.md#couldnt-verify-your-organizations-policy-for-cloud-sessions)。 |
| `Attaching to an existing cloud session is not enabled for your account.` | 您在运行 `--cloud <session-id>` 时未带 `-p`。请使用 `claude -p "your message" --cloud <session-id>` 发送消息。 |
| `Session not found: <id>` | 该 ID 或 URL 与您可以访问的任何会话都不匹配。请对照该会话的 claude.ai/code URL 进行检查。 |
| `cloud session <id> is archived and cannot accept new messages` | 该会话已归档。请改为启动一个新会话。 |

### 环境已过期

云会话在不活动一段时间后停止，会话的 VM 被回收。会话在等待你批准 [MCP 连接器](https://code.claude.com/docs/zh-CN/cloud-environments.md#network-access)工具调用或登录到 MCP 服务器时计为不活动，它可以在该等待期间过期。

从 [claude.ai/code](https://claude.ai/code) 重新打开会话以预配新的 VM：

* **会恢复**：您的对话历史
* **不会恢复**：VM 被回收时仍在运行或待处理的后台工作，包括：
  * **Shell 命令**：Claude Code 会告知 Claude 哪些命令已停止。请让 Claude 重新运行您仍需要的命令。
  * **子代理**：Claude Code 会告知 Claude 哪些子代理已停止，以及其中哪些可以[恢复](https://code.claude.com/docs/zh-CN/sub-agents.md#resume-subagents)而无需从头开始。请让 Claude 继续它们的工作。
  * **会话范围的定时任务**：您[在会话中安排的任务](https://code.claude.com/docs/zh-CN/scheduled-tasks.md)以及正在等待下一次迭代的 `/loop` 不会延续到重新打开的会话中。请让 Claude 重新安排这些任务，或再次运行 `/loop` 以重新启动循环。

## 限制

在依赖云会话进行工作流之前，请考虑这些约束：

* **速率限制**：云端会话与您账户内所有其他 Claude 和 Claude Code 使用共享速率限制。并行运行多个任务会按比例消耗更多速率限制。
* **时间限制**：Claude 运行的命令和 SessionStart hooks 有你可以更改的默认超时，设置脚本仅在大约五分钟内完成时才被缓存。请参阅[时间限制](https://code.claude.com/docs/zh-CN/cloud-environments.md#time-limits)
* **存储库身份验证**：你只能在认证到相同账户时将云会话拉入你的终端
* **平台限制**：在 Anthropic 托管环境中克隆仓库以及创建 Pull Request 需要 GitHub。Team 和 Enterprise 计划支持自托管的 [GitHub Enterprise Server](https://code.claude.com/docs/zh-CN/github-enterprise-server.md) 实例。您可以通过设置 `CCR_FORCE_BUNDLE=1` 将 GitLab、Bitbucket 或其他非 GitHub 仓库作为[本地捆绑](#send-local-repositories-without-github)发送到云端会话，但会话无法将结果推送回该远程
  * **自托管环境**：对于托管在其他 git 平台上的仓库，请参阅[使用 `--environment` 启动会话的要求](https://code.claude.com/docs/zh-CN/self-hosted-environments-deploy#start-a-session-on-a-repository-outside-github)。
* **组织 IP 允许列表**：云端会话从 Anthropic 管理的基础设施（而非您的网络）调用 Anthropic API，而[自托管环境](https://code.claude.com/docs/zh-CN/self-hosted-environments)中的会话则从您自己的网络调用它。如果您的组织启用了 [IP 允许列表](https://support.claude.com/en/articles/13200993-restrict-access-to-claude-with-ip-allowlisting)，每个 Anthropic 托管的云端会话都会因身份验证错误而失败。
  * **Code Review 和定时任务**：这同样适用于 [Code Review](https://code.claude.com/docs/zh-CN/code-review.md) 以及在 Anthropic 托管环境中运行的[云端定时任务](https://code.claude.com/docs/zh-CN/routines.md)。路由到自托管环境的定时任务会从您自己的网络调用 API。
  * **获取豁免**：请联系 [Anthropic 支持](https://support.claude.com/)，将 Anthropic 托管的服务从您组织的 IP 允许列表中豁免。

## 相关资源

* [云环境](https://code.claude.com/docs/zh-CN/cloud-environments.md)：为云会话配置网络访问、环境变量和设置脚本
* [Projects](https://code.claude.com/docs/zh-CN/claude-projects.md)：一个对话，Claude 在其中协调您的存储库上的并行云会话并报告结果
* [Ultrareview](https://code.claude.com/docs/zh-CN/ultrareview.md)：在云沙箱中运行深度多代理代码审查
* [云端定时任务](https://code.claude.com/docs/zh-CN/routines.md)：按计划、通过 API 调用或响应 GitHub 事件自动化工作
* [Hooks 配置](https://code.claude.com/docs/zh-CN/hooks.md)：在会话生命周期事件处运行脚本
* [所有设置](https://code.claude.com/docs/zh-CN/settings-reference.md)：所有配置选项
* [安全](https://code.claude.com/docs/zh-CN/security.md)：隔离保证和数据处理
* [数据使用](https://code.claude.com/docs/zh-CN/data-usage.md)：Anthropic 从云会话保留的内容
* [Claude Tag](https://claude.com/docs/claude-tag/overview)：在 Slack 中由组织管理的 @Claude，运行在相同的云基础设施上
