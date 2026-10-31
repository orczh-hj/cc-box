> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# 选择沙箱环境

> 比较 Claude Code 沙箱选项：内置沙箱化 Bash 工具、沙箱运行时、开发容器、Docker 和虚拟机。为您的威胁模型选择合适的隔离方案。

隔离 Claude Code 会限制会话可以读取、写入和访问网络的内容。当您让 Claude 在较少权限提示的情况下工作、以无人值守方式运行它，或将其指向您不完全信任的代码时，这一点最为重要。

Claude Code 可以在多种隔离环境中运行，从轻量级的按命令沙箱到完全独立的虚拟机。本页面比较了它们隔离的内容和所需条件，帮助您为威胁模型选择合适的方案，并展示如何在整个组织中强制实施该选择。

> [!NOTE]
>
> 有关更广泛的安全模型，请参阅 [安全性](https://code.claude.com/docs/zh-CN/security.md)。有关 Agent SDK 部署，请参阅 [安全部署](https://code.claude.com/docs/zh-CN/agent-sdk/secure-deployment.md)。

## 比较沙箱方法

下表中的前两种方法在主机操作系统上运行，不使用容器。其余方法将 Claude Code 放在容器或虚拟机内。

| 方法 | 隔离的内容 | 需要 Docker | 设置工作量 |
| :- | :- | :- | :- |
| [Sandboxed Bash tool](#sandboxed-bash-tool) | Bash、PowerShell 和 Monitor 工具命令及其子进程 | 否 | macOS 上最少；Linux 和 WSL2 上较少 |
| [Sandbox runtime](#sandbox-runtime) | 整个 Claude Code 进程，包括文件工具、MCP 服务器和 hooks | 否 | 较少 |
| [Dev container](#dev-containers) | 完整开发环境 | 是 | 中等 |
| [Custom container](#custom-container) | 完整开发环境 | 是 | 中等到高 |
| [Virtual machine](#virtual-machine) | 完整操作系统 | 否 | 高 |
| [Cloud sessions](#cloud-sessions) | 完整操作系统，由 Anthropic 托管 | 否 | 无；需要 Claude 订阅和已连接的 GitHub 账户，除非您使用 `claude --cloud` 启动 |

[Sandboxed Bash tool](https://code.claude.com/docs/zh-CN/sandboxing.md) 内置于 Claude Code 中，仅限制 Bash 命令。内置文件工具、MCP 服务器和 hooks 仍直接在您的主机上运行。表中的所有其他方法都将整个 Claude Code 进程放在隔离边界内，因此文件工具、MCP 服务器和 hooks 也受到限制。

> [!WARNING]
>
> 沙箱隔离可以减少违规的影响，但不能消除风险。任何允许网络出站的方法仍然可能泄露代理可以读取的数据，任何以可写方式挂载项目目录的方法仍然可能修改该代码。在依赖沙箱作为硬控制之前，请查看 [security limitations](https://code.claude.com/docs/zh-CN/sandboxing.md#security-limitations)。
>
> 隔离也不会改变发送给模型的内容。您的提示和 Claude 读取的文件无论是否使用沙箱，都会传输到 Anthropic API 或您配置的提供商。有关 Claude Code 发送的内容以及如何减少它，请参阅 [Data usage](https://code.claude.com/docs/zh-CN/data-usage.md)。

## 选择一种方法

将您的目标与下面的一行匹配，然后阅读随后的详细部分。

| 您想要 | 开始使用 |
| :- | :- |
| 在您自己的机器上日常工作期间减少权限提示 | [sandboxed Bash tool](https://code.claude.com/docs/zh-CN/sandboxing.md)，使用 `/sandbox` 启用 |
| 让 Claude 使用 `--dangerously-skip-permissions` 或自动模式无人值守工作 | 预配置的 [dev container](https://code.claude.com/docs/zh-CN/devcontainer.md)、任何容器或虚拟机，或 [sandbox runtime](#sandbox-runtime) |
| 隔离 MCP 服务器和 hooks 以及 Bash，不使用 Docker | sandbox runtime |
| 在不受信任的存储库上工作 | 专用虚拟机，或 [cloud session](https://code.claude.com/docs/zh-CN/claude-code-on-the-web.md)（如果您有 Claude 订阅）；使用 `claude --cloud` 启动时不需要 GitHub |
| 在团队中标准化沙箱环境 | 预配置的 [dev container](https://code.claude.com/docs/zh-CN/devcontainer.md)，复制到您的存储库中 |
| 从没有本地设置的设备使用 Claude Code | [cloud session](https://code.claude.com/docs/zh-CN/claude-code-on-the-web.md)，需要 Claude 订阅和连接的 GitHub 账户 |
| 为组织中的每个开发人员要求隔离 | [在整个组织中强制实施隔离](#enforce-isolation-across-an-organization) |
| 在本机 Windows 主机上工作 | 容器或虚拟机，或在 WSL2 内运行 Bash 沙箱 |

### 隔离与权限模式的关系

[Permission modes](https://code.claude.com/docs/zh-CN/permission-modes.md) 决定工具调用是否运行以及是否首先提示您。隔离限制命令运行后可以访问的内容。两者协同工作：当权限模式允许操作在不询问您的情况下运行时，隔离边界限制这些操作可以到达的内容。

当您传递 `--dangerously-skip-permissions` 时，Claude 在不首先询问您的情况下行动。[任何模式都不会自动批准的操作](https://code.claude.com/docs/zh-CN/permission-modes.md#actions-no-mode-auto-approves) 仍然适用。

没有提示来捕捉错误，您选择的隔离边界是保护您的系统的东西。始终在容器、虚拟机或 [sandbox runtime](#sandbox-runtime) 内运行 `--dangerously-skip-permissions` 会话，以便文件工具、MCP 服务器和 hooks 也在边界内。在 Linux 和 macOS 上，当以 root 用户身份运行时，Claude Code 拒绝使用此标志启动，因此请以非 root 用户身份运行容器、虚拟机或 sandbox runtime。

[Auto mode](https://code.claude.com/docs/zh-CN/permission-modes.md#eliminate-prompts-with-auto-mode) 用审查操作的分类器替换提示。分类器是按操作控制，而不是隔离边界，因此隔离边界仍然为无人值守运行添加纵深防御，并且不像 `--dangerously-skip-permissions` 那样是必需的。

[sandboxed Bash tool](#sandboxed-bash-tool) 本身仅限制 shell 命令，因此对于任一模式的完全无人值守运行都不足够。您可以分层方法：在容器或虚拟机内运行沙箱化 Bash 工具可在外部环境边界之上为您提供操作系统级命令限制。有关 Bash 沙箱本身如何与权限规则和权限模式交互的信息，请参阅 [How sandboxing relates to permissions and permission modes](https://code.claude.com/docs/zh-CN/sandboxing.md#how-sandboxing-relates-to-permissions-and-permission-modes)。

## Sandboxed Bash tool

> [!NOTE]
>
> 此选项不支持本机 Windows。在 Windows 主机上，使用 WSL2 或下面的容器或虚拟机方法之一。

Sandboxed Bash tool 内置于 Claude Code 中。它使用操作系统原语来限制 Claude 运行的 Bash、PowerShell 和 Monitor 工具命令的文件系统和网络访问。

运行 `/sandbox` 命令打开沙箱面板并选择一个模式。[Sandboxing](https://code.claude.com/docs/zh-CN/sandboxing.md) 指南涵盖批准模式、默认边界以及如何扩大或缩小它。

按命令沙箱不涵盖会话中运行的所有内容：

* 其他 [built-in tools](https://code.claude.com/docs/zh-CN/tools-reference.md)（如 Read、Edit 和 WebFetch）在 Claude Code 进程内运行，不会生成任意代码。[Permission rules](https://code.claude.com/docs/zh-CN/permissions.md) 用于路径或域来控制它们。
* [MCP](https://code.claude.com/docs/zh-CN/mcp.md) 服务器、[命令 hook](https://code.claude.com/docs/zh-CN/hooks.md#command-hook-fields) 和[插件监视器](https://code.claude.com/docs/zh-CN/plugins/components.md#monitors)是在主机上无约束运行的单独进程。有关以这种方式运行的其他进程，请参阅[哪些内容在沙箱外运行](https://code.claude.com/docs/zh-CN/sandboxing.md#what-runs-outside-the-sandbox)。

要将内置工具、MCP 服务器和 hooks 都放在一个操作系统边界后面，请在 [sandbox runtime](#sandbox-runtime)、[dev container](#dev-containers) 或 [custom container](#custom-container) 内运行整个 Claude Code 进程。

## Sandbox runtime

[`@anthropic-ai/sandbox-runtime`](https://github.com/anthropics/sandbox-runtime) 包将整个进程包装在内置 Bash 沙箱使用的相同 Seatbelt 或 bubblewrap 隔离中。通过该运行时运行 Claude Code 不仅会限制 shell 命令，还会限制会话的工具、hook 和 MCP 服务器。运行时是测试版研究预览，其配置格式可能会随着包的发展而改变。

本部分涵盖您配置的内容以及运行时自身强制执行的内容。有关在 Agent SDK 应用程序中部署运行时，请参阅[安全部署指南](https://code.claude.com/docs/zh-CN/agent-sdk/secure-deployment.md#sandbox-runtime)。

### 设置和启动运行时

在 Linux 和 WSL2 上，运行时依赖于与内置沙箱相同的 `bubblewrap` 和 `socat` 包，加上 `ripgrep`，Claude Code 捆绑了它，但独立运行时从您的 PATH 中解析。按照[设置 Linux 和 WSL2](https://code.claude.com/docs/zh-CN/sandboxing.md#set-up-linux-and-wsl2) 中的说明安装 `bubblewrap` 和 `socat`，以及从您的发行版包管理器安装 `ripgrep`。在 macOS 上，您不需要任何额外的包。运行时在那里使用内置的 Seatbelt 沙箱。

默认情况下，运行时拒绝网络访问并将写入限制在一小组内置运行时路径，因此在通过它启动 Claude Code 之前配置它。将您的配置放在 `~/.srt-settings.json` 中，或在您使用 `--settings` 传递的文件中。包 [README](https://github.com/anthropics/sandbox-runtime) 记录了配置 schema。

至少允许写入访问：

* 您的项目目录。
* Claude Code 的配置路径 `~/.claude` 和 `~/.claude.json`。
* Claude Code 写入运行时文件的目录。除非您设置了 [`CLAUDE_CODE_TMPDIR`](https://code.claude.com/docs/zh-CN/env-vars)，否则该目录为：
  * **Linux 和 WSL2**：`/tmp`
  * **macOS**：`/private/tmp`。`/tmp` 是指向该目录的符号链接，而 Seatbelt 检查的是解析后的路径。

允许您的会话需要的网络域：

* `api.anthropic.com`，或您配置的提供商的端点。在第三方提供商上，也保留 `api.anthropic.com`：WebFetch 域安全检查默认仍会调用它，除非您设置 `skipWebFetchPreflight: true`。
* `claude.ai` 和 `platform.claude.com`，[OAuth 登录和令牌刷新](https://code.claude.com/docs/zh-CN/network-config.md#network-access-requirements)需要这些。使用 API 密钥进行身份验证的运行可以删除这两个。

在 Linux 和 WSL2 上，运行时仅对已存在的路径应用写入授权。在全新环境中，在首次启动前创建 Claude Code 的配置路径：

```bash theme={null}
mkdir -p ~/.claude && { [ -f ~/.claude.json ] || echo '{}' > ~/.claude.json; }
```

全局安装运行时，安装前缀应位于您允许写入的所有路径之外：

```bash theme={null}
npm install -g @anthropic-ai/sandbox-runtime
```

设置文件就位后，从您的项目目录通过运行时启动 Claude Code：

```bash theme={null}
srt claude
```

Claude Code 在沙箱内启动，具有您配置的文件系统和网络边界。相同的 `srt` 命令适用于沙箱化独立 MCP 服务器或其他辅助进程。

### 运行时自身阻止的内容

运行时在没有您任何配置的情况下阻止最高风险的写入：

* `denyWrite` 优先于 `allowWrite`。
* 在项目根目录，运行时拒绝 `.git/hooks`，拒绝 `.git/config` 除非您设置 `filesystem.allowGitConfig: true`，并拒绝 `.mcp.json`、`.claude/commands`、`.claude/agents` 和 shell 启动文件。
* 在 macOS 上，这些拒绝在写入发生时被检查，因此它们也涵盖嵌套文件和在会话期间创建的仓库。
* 在 Linux 和 WSL2 上，运行时在启动时构建拒绝列表一次。它可靠地涵盖项目根目录，对当时存在的嵌套副本进行最佳努力的浅层扫描，并不涵盖会话稍后创建的任何内容，例如 `git init`、`git clone` 或脚手架。README 的 `mandatoryDenySearchDepth` 部分描述了扫描的确切语义。
* 如果 `~/.srt-settings.json` 不存在且您没有传递 `--settings`，运行时仍然启动。它阻止网络访问并将写入限制在内置运行时路径，例如 `/tmp/claude`、`~/.npm/_logs` 和 `~/.claude/debug`。不要将干净的启动作为您的设置已加载的证明。
* 如果设置文件存在但为空、不可读或无效，运行时拒绝启动，无论是 `~/.srt-settings.json` 还是您使用 `--settings` 传递的文件。如果 `--settings` 文件不存在，它也拒绝启动。

您的写入授权仍然包括 Claude Code 加载配置的其他路径，因此使用 `denyWrite` 拒绝这些路径。可以写入它们的沙箱化会话可以持久化 hook、权限规则或 MCP 服务器，这些在您下次启动 Claude Code 时以未沙箱化的方式运行。

### 无人值守运行后

审查您保持可写的路径。在 Linux 和 WSL2 上，也审查会话创建的任何内容。

## Dev containers

Dev container 在 VS Code 或兼容编辑器管理的 Docker 容器内运行 Claude Code，您的项目挂载在其中。您可以在存储库中使用 `.devcontainer/` 目录定义自己的。

Claude Code 存储库发布了一个 [example dev container](https://code.claude.com/docs/zh-CN/devcontainer.md)，其中包含默认拒绝 iptables 防火墙作为起点。将其复制到您的存储库中，并调整防火墙允许列表、基础镜像和固定的 Claude Code 版本以适应您的环境。因为防火墙阻止未批准的出站，像这样的配置支持使用 `--dangerously-skip-permissions` 运行 Claude Code 以进行无人值守工作。

## Custom container

您可以在任何 Docker 或 OCI 容器镜像中运行 Claude Code，具有您自己的网络策略、挂载卷和 seccomp 配置文件。这是具有现有容器基础设施或 CI 运行器的组织最常见的路径。

几个托管沙箱和远程执行服务可以为您托管容器。与您操作的任何容器相同的检查清单适用：查看挂载的可写内容、容器内可访问的凭据和令牌，以及网络出站策略允许的内容。

您可以在容器内分层内置 Bash 沙箱以进行按命令限制。无特权容器需要 `enableWeakerNestedSandbox`，详见 [Bubblewrap 在容器内无法启动](https://code.claude.com/docs/zh-CN/sandboxing.md#bubblewrap-fails-to-start-inside-a-container)。

## Virtual machine

专用虚拟机提供最强的分离，具有自己的内核，在云或 microVM 部署中，还有自己的虚拟化硬件。选项包括云实例、本地虚拟机管理程序和 microVM（如 Firecracker）。当您评估不受信任的代码、您的安全策略要求代理和主机之间的内核级分离，或没有主机级方法满足您的合规要求时，使用此方法。

[Docker Sandboxes](https://docs.docker.com/ai/sandboxes/) 提供了一个具有自己的 Docker 守护程序和工作区同步的 microVM，可以在任何安装了 Docker Sandboxes 的主机上运行 Claude Code。它是来自 Docker 的免费独立产品，不需要 Docker Desktop。

## 云会话

[云会话](https://code.claude.com/docs/zh-CN/claude-code-on-the-web.md)在隔离的、由 Anthropic 管理的虚拟机中运行。网络代理强制执行默认允许列表，单独的代理在沙箱外保存您的 GitHub 令牌，同时在其内部为存储库访问发出作用域凭据。您的组织路由到[自托管环境](https://code.claude.com/docs/zh-CN/self-hosted-environments)的会话在您配置的基础设施上运行，其中隔离、出站控制和 git 凭据是您部署的责任。

当您想要完整的虚拟机隔离而无需自己配置基础设施，或当您从没有本地开发环境的设备委派任务时，使用此方法。它需要 Claude 订阅。除非您从 CLI 启动，否则您还需要一个连接的 GitHub 账户，以便沙箱可以克隆您的存储库。当您使用 `--cloud` 从 CLI 启动时，Claude Code 可以[捆绑并上传您的本地存储库](https://code.claude.com/docs/zh-CN/claude-code-on-the-web.md#send-local-repositories-without-github)。有关计划可用性和 GitHub 身份验证选项，请参阅[在云中使用 Claude Code](https://code.claude.com/docs/zh-CN/claude-code-on-the-web.md)。

## 在整个组织中强制实施隔离

个别开发人员可以选择本页面上的任何沙箱方法。组织可以强制实施什么，以及使用哪些工具，取决于该方法：

* **Built-in Bash sandbox**：Claude Code 强制实施的唯一方法。通过 [managed settings](https://code.claude.com/docs/zh-CN/managed-settings.md#delivery-mechanisms) 传递 `sandbox` 设置键，可以是由您的 MDM 管理的文件，也可以通过 Claude.ai 上的 [server-managed settings](https://code.claude.com/docs/zh-CN/server-managed-settings.md)。有关要部署的键以及如何防止开发人员扩大策略，请参阅 [Enforce sandboxing with managed settings](https://code.claude.com/docs/zh-CN/sandboxing.md#enforce-sandboxing-with-managed-settings)。
* **Dev containers**：将 [example dev container](https://code.claude.com/docs/zh-CN/devcontainer.md) 提交到您的存储库以在团队中标准化环境。这是一个约定而不是强制边界，因为 Claude Code 不需要容器。如果开发人员不应该能够在其外部运行 Claude Code，请使用您组织的设备管理或软件允许列表工具强制实施。
* **Custom containers and VMs**：通过批准的镜像分发 Claude Code，并使用您组织的设备管理或软件允许列表工具防止在其外部安装。

## 另请参阅

这些页面涵盖上述方法的配置和策略详细信息。

* [Sandboxing](https://code.claude.com/docs/zh-CN/sandboxing.md)：配置内置沙箱化 Bash 工具
* [Dev container](https://code.claude.com/docs/zh-CN/devcontainer.md)：预配置的 Docker 开发容器
* [安全性](https://code.claude.com/docs/zh-CN/security.md)：完整的 Claude Code 安全模型
* [安全部署](https://code.claude.com/docs/zh-CN/agent-sdk/secure-deployment.md)：Agent SDK 应用程序的隔离指导
* [设置](https://code.claude.com/docs/zh-CN/settings-reference.md#sandbox-settings)：所有沙箱配置键，包括托管设置传递
