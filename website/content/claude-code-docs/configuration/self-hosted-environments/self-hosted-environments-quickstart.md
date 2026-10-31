> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# 自托管环境快速入门

> 设置您的第一个自托管环境：安装 Claude Code、创建环境、启动运行器，并将会话路由到该环境。

> [!NOTE]
>
> 自托管环境在 Team 和 Enterprise 计划上处于公开测试阶段；[可用性和限制](https://code.claude.com/docs/zh-CN/self-hosted-environments#availability-and-limitations)涵盖了启用路径。本页面让您的第一个会话运行；有关它们是什么，请参阅[自托管环境](https://code.claude.com/docs/zh-CN/self-hosted-environments)，有关强化和部队配方，请参阅[部署到生产环境](https://code.claude.com/docs/zh-CN/self-hosted-environments-deploy.md)。

[自托管环境](https://code.claude.com/docs/zh-CN/self-hosted-environments)在您的组织运营的基础设施上运行 Claude Code [云会话](https://code.claude.com/docs/zh-CN/claude-code-on-the-web.md)，由您部署的运行器进程执行。本快速入门建立您的第一个环境，这是最小的可行配置：一个主机上的一个运行器，运行一个测试会话。有两个步骤：[创建环境、启动运行器并将会话路由到该环境](#set-up-an-environment-and-runner)，然后[从您的终端向该会话发送后续消息](#send-a-follow-up-message-to-a-running-session)。您将在两个界面之间切换：claude.ai 用于创建环境、检查其状态和路由会话，以及主机上的终端用于运行器执行的所有操作。

完成后，您将在[**Cloud environments** 管理页面](https://claude.ai/admin-settings/cloud-environments)上拥有一个环境、一个轮询工作的运行器，以及在您的主机上运行的会话。在连接真实存储库或内部系统之前，请完成[部署到生产环境](https://code.claude.com/docs/zh-CN/self-hosted-environments-deploy.md)，其中涵盖了安全态势、出口控制、git 凭证和编排。

## 前置条件

### 组织和角色

claude.ai 端需要：

* **Allow self-hosted environments** 由[所有者](https://code.claude.com/docs/zh-CN/cloud-environments.md#organization-shared-environments)在[**Cloud environments** 管理页面](https://claude.ai/admin-settings/cloud-environments)上启用；在启用之前，**New** 按钮不会出现。如果您不拥有该角色，拥有该角色的人可以创建环境并将其密钥交给您；本页面上的运行器和终端步骤不需要 claude.ai 角色，当步骤在管理 UI 中检查状态时，运行器自己的日志行会给您相同的信号。
* 由所有者启用的 [GitHub 连接器](https://claude.ai/admin-settings/connectors)，以及每位开发人员自己的 [GitHub 连接](https://code.claude.com/docs/zh-CN/web-quickstart.md#connect-github)，以便他们在启动会话时可以选择 github.com 仓库。

### 主机和网络

运行器主机需要：

* 一个 Linux 或 macOS 主机或容器，具有到 `api.anthropic.com` 的出站 HTTPS、到 `claude.ai` 和下面安装步骤重定向到的下载主机的出站 HTTPS，以及到您的 git 主机的出站 HTTPS 用于克隆；[网络要求表](https://code.claude.com/docs/zh-CN/self-hosted-environments-deploy.md#network-requirements)有完整列表。Windows 不支持作为运行器主机；改为在 Linux 容器中运行运行器。开发人员工作站不受影响，因为会话从浏览器中的 claude.ai 启动。
* 一个用于测试会话的仓库：可以是公共仓库，也可以是此主机已能通过其 HTTPS URL 克隆且无需提供凭据的仓库。仅有 SSH 密钥是不够的，因为运行器克隆的是仓库的 `https://` URL。要改为通过 SSH 克隆，请参阅[为私有网络重写 git URL](https://code.claude.com/docs/zh-CN/self-hosted-environments-deploy.md#rewrite-git-urls-for-private-networks)。
* 与实时同步的时钟，例如使用 NTP。

### 运行器主机上的软件

在启动之前在主机上安装：

* **Claude Code v2.1.224 或更高版本**，使用任何[标准安装方法](https://code.claude.com/docs/zh-CN/setup.md)。运行器是标准 `claude` 二进制文件的一部分，较早的版本不识别 `self-hosted-runner` 子命令。为 Claude Tag 会话提供服务的运行器需要 v2.1.229 或更高版本，并且其 `PATH` 上需要有 `python3`，因为[控制平面提供的 hook](https://code.claude.com/docs/zh-CN/self-hosted-environments-configuration.md#how-each-session’s-config-is-assembled) 会为这些会话运行 `python3`。要固定您的主机集群运行的确切版本，请参阅[安装特定版本](https://code.claude.com/docs/zh-CN/setup.md#install-a-specific-version)。对于容器镜像，请参阅[部署到生产环境](https://code.claude.com/docs/zh-CN/self-hosted-environments-deploy.md#build-the-runner-image)中的 Dockerfile。
* **Git 2.24 或更高版本**。部署页面上的某些 git 选项需要更高版本；[配置 git](https://code.claude.com/docs/zh-CN/self-hosted-environments-deploy.md#configure-git)说明了每个下限。

确认主机已准备好：

```bash theme={null}
claude self-hosted-runner --help
```

准备好的主机打印运行器的使用文本，列出诸如 `--environment-secret-file` 之类的标志。在 2.1.224 之前的版本上，该命令改为打印常规 `claude --help` 输出。如果您看到该输出，请[更新 Claude Code](https://code.claude.com/docs/zh-CN/setup.md#update-claude-code)。如果您从容器镜像运行 `claude`，请使用 2.1.224 或更高版本[重新构建镜像](https://code.claude.com/docs/zh-CN/self-hosted-environments-deploy.md#build-the-runner-image)。

## 设置环境和运行器

使用[引导式设置](#run-the-guided-setup)或[手动步骤](#set-up-manually)。引导式设置只需一条命令，它会启动一个交互式 Claude Code 会话，并引导您完成其余步骤。在无法进行交互式会话的主机上，请改用手动步骤。如果拥有所有者角色的人员已创建环境并将其密钥交给您，也请使用手动步骤，因为引导式设置需要以所有者身份登录。

### 运行引导式设置

引导式设置会引导您在管理 UI 中创建环境、使用您保存的密钥文件启动本地运行器、确认运行器已注册，并将速查表写入 `./runner-setup/CHEAT-SHEET.md`。运行之前，请确认您的登录状态和版本：

* **登录**：在您已使用拥有所有者角色的帐户通过 `claude auth login` 登录的机器上运行它。如果仅使用 API 密钥或第三方模型提供商，会话虽然会启动，但其组织检查会失败。
* **版本**：确认[版本检查](#software-on-the-runner-host)已通过。在 2.1.224 之前的版本上，此设置命令会启动一个 Claude 会话，并将这些词作为提示词，而不是启动引导式设置。

要启动引导式设置，请在 shell 中运行设置子命令并按照提示进行操作：

```bash theme={null}
claude self-hosted-runner setup
```

设置本身不会启动测试会话：它会提示您在 claude.ai/code 启动一个。设置的最后一步会停止它所启动的运行器。如果您在该步骤之前退出设置，运行器将继续运行。要在最后一步之后继续，请在 shell 中使用 `./runner-setup/CHEAT-SHEET.md` 中的命令再次启动运行器，然后[将会话路由到环境](#route-a-session)。

### 手动设置

在 claude.ai 上创建环境，从主机上的终端启动运行器，然后返回 claude.ai 确认运行器出现并将会话路由到它。如果拥有所有者角色的人员已创建环境并将其密钥交给您，请从第 2 步开始。

**1. 创建环境**

转到管理设置中的[**Cloud environments** 页面](https://claude.ai/admin-settings/cloud-environments)。在 **Self-hosted environments** 下，选择 **New**，命名环境，然后选择 **Create**。在向导的第二步，选择 **Copy environment key** 以复制环境密钥，管理 UI 将其标记为环境密钥。claude.ai 仅显示一次密钥，您之后无法检索它；它在创建后 365 天过期。环境的 `ccpool_...` ID 在其详细信息对话框中保持可见；您需要它用于[令牌验证](https://code.claude.com/docs/zh-CN/self-hosted-environments-identity.md)中的 `aud` 检查，以及用于从 CI [分派测试会话](https://code.claude.com/docs/zh-CN/self-hosted-environments-testing.md#run-the-test-loop)。

如果您丢失了密钥或需要轮换它，请从环境的 **Configuration** 选项卡创建新密钥，将新密钥推出到您的运行器，然后撤销旧密钥。持有已撤销密钥的运行器在其下一次经过身份验证的轮询时失败并退出，记录 `poll auth failed`，您的编排器使用新密钥重新启动它们。

**2. 启动运行器**

创建密钥目录。此命令和下一条命令使用 `/etc/claude`，这需要 root 权限，并且它们创建的密钥文件仅可由运行这些命令的用户读取。如果运行器将以其他用户身份运行，它会退出并显示 `error: Failed to read environment secret file <path> (EACCES: permission denied, open '<path>')`。在这种情况下，请以运行器的用户身份运行这两条命令，并使用该用户可写入的目录代替 `/etc/claude`，同时将相同的路径传递给 `--environment-secret-file`。运行器进程可以读取的任何路径都有效。

```bash theme={null}
mkdir -p /etc/claude
```

将环境密钥写入文件。下面的命令从您的终端读取，以便密钥保持在 shell 历史记录之外：粘贴您复制的值，按 Enter，然后按 Ctrl-D，子 shell 的 `umask` 使文件仅可由其所有者读取。

```bash theme={null}
(umask 077 && cat > /etc/claude/environment-secret)
```

选择一个基目录，将下面运行器命令中的 `<writable-dir>` 替换为运行器可以写入或创建的绝对路径。运行器在启动时创建目录，然后检出仓库并在其下创建每个会话的目录。没有 `--base-dir`，它使用 `/workspace`，这仅在该目录已存在且可写或您以 root 身份启动运行器时有效。

> [!WARNING]
>
> 请使用专供运行器使用的目录，而不是您的主目录或存放您自己克隆的目录。运行器会将它在 `<writable-dir>/<repo-owner>/<repo>` 处找到的任何内容视为它自己的克隆，并可能重置它以及覆盖或删除其中的文件。

如果运行器无法创建或写入基目录，它会在启动时退出并显示指明该目录的错误，而不是进行注册。请参阅[故障排除](https://code.claude.com/docs/zh-CN/self-hosted-environments-deploy.md#troubleshooting)。

然后使用 `--environment-secret-file` 和 `--base-dir` 启动运行器：

```bash theme={null}
claude self-hosted-runner --environment-secret-file '/etc/claude/environment-secret' --base-dir '<writable-dir>'
```

运行器向您的环境注册后会记录 `Registered: runner_id=<runner-id>`，然后开始轮询工作。使用此命令启动时，运行器按设计会在服务完一个会话后退出，而不是在每个轮次之后退出。要服务更多会话，请在每次运行器退出时再次启动它。有关何时会发生这种情况以及运行器会记录什么，请参阅[如果运行器退出](#if-the-runner-exits)。

**3. 验证运行器出现**

返回[**Cloud environments** 页面](https://claude.ai/admin-settings/cloud-environments)。您的环境状态在运行器启动后几秒内从 **No runners deployed** 更改为 **Healthy**；打开环境并选择 **Activity** 以查看运行器本身。如果您无权访问管理页面，上一步运行器日志中的 `Registered: runner_id=<runner-id>` 行可提供相同的信号。

**4. 将会话路由到环境**

在 claude.ai/code 启动会话，并从环境选择器中选择您的环境，其中自托管环境与 Anthropic 托管的环境一起出现。对于仓库，请选择[前提条件](#host-and-network)中的仓库：公共仓库，或此主机已可以通过其 HTTPS URL 克隆且不会被要求提供凭据的仓库。

下一个可用的运行器拾取排队的会话并记录 `Picked up session <session-id>` 以及其活跃计数和容量，因此您可以从运行器自己的输出中确认哪个主机接收了会话。在 [claude.ai/code](https://claude.ai/code) 观看会话工作并阅读 Claude 的回复。

如果会话没有开始工作，请对照您看到的情况：

* **会话保持排队状态**：请参阅[故障排除](https://code.claude.com/docs/zh-CN/self-hosted-environments-deploy.md#troubleshooting)。
* **会话因 git 错误而无法启动**：如果错误包含 git 的 `could not read Username for`，后跟您的 git 主机的 URL，则说明运行器没有该主机的 HTTPS 凭据。
  * **提供 HTTPS 凭据**：请参阅[配置 git](https://code.claude.com/docs/zh-CN/self-hosted-environments-deploy.md#configure-git)
  * **改为通过 SSH 克隆**：如果运行器主机已有 SSH 密钥，请参阅[为私有网络重写 git URL](https://code.claude.com/docs/zh-CN/self-hosted-environments-deploy.md#rewrite-git-urls-for-private-networks)
  * **完成任一修复后**：再次启动运行器并观察失败的会话，而不是启动新会话。如果该会话已失败多次，请等待几分钟，直到运行器记录 `Picked up session`。
  * **改为使用其他仓库**：[归档](https://code.claude.com/docs/zh-CN/claude-code-on-the-web.md#archive-sessions)或[删除](https://code.claude.com/docs/zh-CN/claude-code-on-the-web.md#delete-sessions)失败的会话，然后再次启动运行器并启动新会话。否则，运行器可能会拾取失败的会话而不是新会话。

### 如果运行器退出

如果运行器在本快速入门期间退出，请再次启动它。运行器可能会自行退出：

* **没有仍在运行的内容**：日志显示 `[runner:exit] account workload drained — exiting`。运行器在不再运行任何会话时会[按设计退出](https://code.claude.com/docs/zh-CN/self-hosted-environments#runner-lifecycle)，例如：
  * **您归档或删除了会话**：再次启动运行器
  * **会话启动失败**：[修复原因](#session-doesnt-start)，然后再次启动运行器
* **失去联系**：日志显示一行包含 `runner record gone server-side` 或 `poll auth failed` 的 `[runner:fatal]`。如果运行器与 Anthropic 失去联系一段时间（例如因为主机休眠），它可能会在下次连接到 Anthropic 时退出。

一个轮次结束并不会结束您的测试会话。第一轮之后，会话仍处于连接状态，运行器也仍在运行，因此您可以[向会话发送后续消息](#send-a-follow-up-message-to-a-running-session)，而无需先重新启动运行器。

对于生产，在编排器下部署运行器，该编排器在退出时重新启动它，并在运行器启动后立即继续退出时等待更长的时间再重新启动。请参阅[部署到生产环境](https://code.claude.com/docs/zh-CN/self-hosted-environments-deploy.md)和[当运行器退出时](https://code.claude.com/docs/zh-CN/self-hosted-environments-deploy.md#when-the-runner-exits)。

## 向运行中的会话发送后续消息

一旦会话在您的环境上运行，从任何您使用 `claude auth login` 登录的机器上的 `claude` CLI 向其发送后续消息；该命令不需要从启动会话的机器运行。该命令发布一条消息：

```bash theme={null}
claude -p "your message" --cloud <session-id>
```

对于 `<session-id>`，传递裸 `session_...` 或 `cse_...` ID 或会话的 claude.ai/code URL。成功发送打印 `Sent to cloud session.` 以及会话 ID 和查看链接。接受的 ID 形式、JSON 输出以及帐户和策略要求在[从 CLI 发送后续消息](https://code.claude.com/docs/zh-CN/claude-code-on-the-web.md#send-follow-ups-from-the-cli)上，因为该命令对 Anthropic 托管的会话的工作方式相同。

## 接下来的步骤

* [部署到生产环境](https://code.claude.com/docs/zh-CN/self-hosted-environments-deploy.md)：强化部署、控制出口、配置 git 凭证，并在 Kubernetes 或 Compose 下运行部队
* [自定义会话](https://code.claude.com/docs/zh-CN/self-hosted-environments-configuration.md)：包装脚本、生命周期钩子、按需运行器、MCP 服务器和权限
* [端到端测试](https://code.claude.com/docs/zh-CN/self-hosted-environments-testing.md)：一个 CI 烟雾测试，分派会话并读取 Claude 的回复
