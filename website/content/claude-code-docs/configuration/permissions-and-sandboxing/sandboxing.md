> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# 配置沙箱化的 Bash 工具

> 使用内置沙箱限制 Claude Code 的 shell 命令可以访问的文件和网络主机。启用沙箱、设置边界，并修复它导致的问题。

Bash 沙箱是操作系统围绕 Claude 在您的计算机上运行的 shell 命令强制执行的边界。您可以设置这些命令可以访问哪些文件和网络域，这些限制适用于 Bash、PowerShell 和 Monitor 命令及其启动的进程。由于操作系统会在命令运行时应用这些限制，Claude Code 可以[运行沙箱化命令而无需询问您](#sandbox-modes)逐一批准。

沙箱仅涵盖 shell 命令。Claude 的文件工具、MCP 服务器和 hook [在沙箱之外运行](#what-runs-outside-the-sandbox)。

沙箱可在 macOS、Linux 和 WSL2 上运行。在原生 Windows 上，Claude Code 以非沙箱方式运行命令。要在 Windows 计算机上使用沙箱，请在 WSL2 发行版中运行 Claude Code。

> [!NOTE]
>
> 本页介绍您自己计算机上围绕 shell 命令的沙箱。其他页面涵盖相关问题：
>
> * 有关云端会话如何隔离，请参阅[安全与隔离](https://code.claude.com/docs/zh-CN/claude-code-on-the-web.md#security-and-isolation)
> * 要比较开发容器、自定义容器和虚拟机等其他隔离方法，请参阅[沙箱环境](https://code.claude.com/docs/zh-CN/sandbox-environments.md)
> * 要减少 Bash 以外工具的权限提示，请参阅[权限模式](https://code.claude.com/docs/zh-CN/permission-modes.md)

## 沙箱限制的内容

沙箱启用时，Claude 运行的 shell 命令会在其边界内启动，这些命令启动的进程也是如此。沙箱默认处于关闭状态。要启用它，请按照[入门](#get-started)中的说明在会话中运行 `/sandbox`，或在 `~/.claude/settings.json` 等[设置文件](https://code.claude.com/docs/zh-CN/settings.md)中将 [`sandbox.enabled`](https://code.claude.com/docs/zh-CN/settings-reference#sandbox-enabled) 设置为 `true`。

下表列出了沙箱化命令默认可以访问的内容，以及可更改各项默认值的设置。

| 访问 | 默认值 | 更改方式 |
| :- | :- | :- |
| 写入 | 工作目录、每个用户独立的临时目录，以及[您添加的目录](https://code.claude.com/docs/zh-CN/permissions.md#additional-directories-grant-file-access-not-configuration)。[受保护路径](#protected-paths)始终禁止写入 | [`filesystem.allowWrite`](https://code.claude.com/docs/zh-CN/settings-reference#sandbox-filesystem-allowwrite)、[`filesystem.denyWrite`](https://code.claude.com/docs/zh-CN/settings-reference#sandbox-filesystem-denywrite) |
| 读取 | 机器上的大部分内容，包括 `~/.ssh` 和 `~/.aws/credentials` 等凭据文件 | [`filesystem.denyRead`](https://code.claude.com/docs/zh-CN/settings-reference#sandbox-filesystem-denyread)、[`credentials`](#protect-credentials) |
| 网络 | 没有直接的出站路由。连接会经过您机器上的代理，该代理会根据您允许的域名检查每个主机，允许的域名列表初始为空。您的权限模式决定了[其他主机的处理方式](#hosts-outside-your-allowed-domains) | [`network.allowedDomains`](https://code.claude.com/docs/zh-CN/settings-reference#sandbox-network-alloweddomains)、[`network.deniedDomains`](https://code.claude.com/docs/zh-CN/settings-reference#sandbox-network-denieddomains) |
| 环境变量 | 继承自 Claude Code，包括其环境中的任何机密信息 | [`credentials`](#protect-credentials)、[`CLAUDE_CODE_SUBPROCESS_ENV_SCRUB`](https://code.claude.com/docs/zh-CN/env-vars) |

Claude Code 基于开源的 [`@anthropic-ai/sandbox-runtime`](https://github.com/anthropics/sandbox-runtime) 包构建沙箱。

### 在沙箱之外运行的内容

沙箱封装的是 shell 命令。以下工具和进程在沙箱之外运行：

* **内置文件和 Web 工具**：Read、Edit、Write、WebFetch 和 WebSearch 等工具改为遵循[权限规则](https://code.claude.com/docs/zh-CN/permissions.md)。`denyRead` 条目不会阻止 Read 工具，`allowedDomains` 也不会限制 WebFetch
* **Claude Code 启动的其他进程**：命令 [hook](https://code.claude.com/docs/zh-CN/hooks.md)、本地 [MCP 服务器](https://code.claude.com/docs/zh-CN/mcp.md)、[插件监视器](https://code.claude.com/docs/zh-CN/plugins/components.md#monitors)、[LSP 服务器](https://code.claude.com/docs/zh-CN/tools-reference.md#lsp-tool-behavior)，以及您的[状态栏](https://code.claude.com/docs/zh-CN/statusline.md)命令和 `apiKeyHelper` 等辅助命令，都以您的完整访问权限运行

根据您的设置，某些 shell 命令也会在沙箱之外运行：

* **您自己输入的命令**：在大多数会话中，您在 [`!` shell 模式提示符](https://code.claude.com/docs/zh-CN/interactive-mode#shell-mode-with-prefix)处输入的命令不在沙箱中运行。[严格沙箱模式](#turn-off-the-retry-with-strict-sandbox-mode)列出了您输入的命令会在沙箱中运行的会话
* **排除的命令**：与 [`excludedCommands`](#run-commands-outside-the-sandbox-with-excludedcommands) 匹配的命令不在沙箱中运行
* **非沙箱重试**：Claude 可以[请求在沙箱之外运行命令](#the-unsandboxed-retry-escape-hatch)，通常是在命令于沙箱中运行失败之后

要将本节中的工具、进程和命令置于同一边界之内，请在[容器、虚拟机或沙箱运行时](https://code.claude.com/docs/zh-CN/sandbox-environments.md)中运行 Claude Code 进程本身。

## 开始使用

沙箱内置于 Claude Code 中。需要安装的内容取决于您的平台：

* **macOS**：沙箱隔离使用内置的 Seatbelt 框架，因此可以直接按照以下步骤操作
* **Linux 和 WSL2**：沙箱依赖 `bubblewrap` 和 `socat`，详见[设置 Linux 和 WSL2](#set-up-linux-and-wsl2)。即使尚未安装它们，也可以先运行 `/sandbox`，因为其面板会显示是否缺少任何内容

**1. 运行 /sandbox**

启动 Claude Code 会话并运行 `/sandbox` 命令：

```text theme={null}
/sandbox
```

这会打开包含三个选项卡的沙箱面板；在 Linux 上，如果缺少可选的 seccomp 过滤器，还会额外显示一个 Dependencies 选项卡：

* **Mode**：选择沙箱命令的批准方式，详见下一步
* **Overrides**：选择在沙箱中失败的命令是否可以回退到在沙箱外运行。这对应 [`allowUnsandboxedCommands`](https://code.claude.com/docs/zh-CN/settings-reference#sandbox-allowunsandboxedcommands) 设置
* **Config**：查看解析后的沙箱设置

如果面板只显示 Dependencies 选项卡，则表示缺少必需的软件包。请按照[设置 Linux 和 WSL2](#set-up-linux-and-wsl2) 中的说明进行安装，重启 Claude Code，然后再次运行 `/sandbox`。

**2. 选择模式**

在 Mode 选项卡中，选择自动允许或常规权限。自动允许会在不提示的情况下运行沙箱命令，而常规权限即使在命令处于沙箱中时也会保留常规权限提示。有关在自动允许模式下仍会提示的命令，请参阅[沙箱模式](#sandbox-modes)。

**3. 运行 Bash 命令**

让 Claude 运行一个命令，例如构建或测试套件。默认情况下，沙箱内的命令可以写入工作目录、[每用户临时目录](https://code.claude.com/docs/zh-CN/env-vars.md)，以及通过 `--add-dir`、`/add-dir` 或 `permissions.additionalDirectories` [添加的任何目录](https://code.claude.com/docs/zh-CN/permissions.md#additional-directories-grant-file-access-not-configuration)。

当命令首次需要访问新的网络域名时，Claude Code 会请求批准；在[自动模式](https://code.claude.com/docs/zh-CN/permission-modes.md#eliminate-prompts-with-auto-mode)下，Claude 则会[在命令本身上](#per-command-allowed-domains-in-auto-mode)列出该命令所需的主机，供分类器连同命令一起审查。

要扩大或缩小沙箱允许的范围，请参阅[配置沙箱隔离](#configure-sandboxing)。

如果沙箱命令在容器内失败并显示 `Operation not permitted`，请参阅 [Bubblewrap 在容器内无法启动](#bubblewrap-fails-to-start-inside-a-container)。

当您在面板中选择模式时，Claude Code 会将其保存到项目的本地设置 `.claude/settings.local.json` 中，该设置适用于当前项目。Claude Code 在该文件中保存设置时，会将其添加到您的全局 gitignore 中。要在所有项目中启用沙箱，请在用户设置 `~/.claude/settings.json` 中将 [`sandbox.enabled`](https://code.claude.com/docs/zh-CN/settings-reference#sandbox-enabled) 设置为 `true`。要为组织中的每位开发者强制启用沙箱隔离，请使用[托管设置](#enforce-sandboxing-with-managed-settings)。

要在不写入设置文件的情况下为单个会话更改沙箱，请使用 [`--settings`](https://code.claude.com/docs/zh-CN/settings#change-a-setting-for-one-session) 启动 Claude Code。例如，以下命令会启动一个沙箱会话，在该会话中 Claude 无法在沙箱外重试被阻止的命令：

```bash theme={null}
claude --settings '{"sandbox": {"enabled": true, "allowUnsandboxedCommands": false}}'
```

> [!WARNING]
>
> 默认情况下，如果由于缺少依赖或平台不受支持而导致沙箱无法启动，Claude Code 会在不使用沙箱的情况下运行命令。要让 Claude Code 改为在启动时退出，请将 [`sandbox.failIfUnavailable`](https://code.claude.com/docs/zh-CN/settings-reference#sandbox-failifunavailable) 设置为 `true`。需要将沙箱隔离作为安全关卡的托管部署可以使用此设置。

### 确认命令在沙箱内运行

要检查沙箱是否正常工作，请让 Claude 运行表中的每一行。您在 [`!` 提示符](#what-runs-outside-the-sandbox)处输入的内容通常在沙箱外运行，因此自己输入这些命令无法起到测试作用。

| 命令 | 在沙箱内的结果 |
| :- | :- |
| `touch ~/sandbox-probe` | 在 macOS 上失败并显示 `Operation not permitted`，在 Linux 和 WSL2 上失败并显示 `Read-only file system` |
| `curl --noproxy '*' https://example.com` | 失败并显示 `Could not resolve host`，因为该命令没有绕过沙箱代理的路由 |

如果 Claude 请求在沙箱外重试失败的命令，请拒绝该重试。如果 `touch` 成功，而您的主目录并不在沙箱允许命令写入的目录之列，请删除 `~/sandbox-probe`。然后运行 `/sandbox`，检查沙箱是否已开启以及其依赖是否已安装。

### 设置 Linux 和 WSL2

在 Linux 和 WSL2 上，沙箱依赖以下软件包：

* [`bubblewrap`](https://github.com/containers/bubblewrap)：用于强制执行文件系统隔离的非特权沙箱隔离工具
* [`socat`](http://www.dest-unreach.org/socat/)：用于将网络流量通过沙箱代理进行路由的中继

使用您的发行版的包管理器安装它们：

> [tab] Ubuntu/Debian
>
> ```bash theme={null}
> sudo apt-get install bubblewrap socat
> ```

> [tab] Fedora
>
> ```bash theme={null}
> sudo dnf install bubblewrap socat
> ```

当缺少依赖时，`/sandbox` 中的 Dependencies 选项卡会列出您的平台缺少 `ripgrep`、`bubblewrap`、`socat` 和 seccomp 过滤器中的哪些。如果在安装并重启 Claude Code 后没有看到该选项卡，则说明所有依赖都已就绪。

Ripgrep 已随原生 Claude Code 二进制文件捆绑提供。seccomp 过滤器是可选的，用于增加对 Unix 域套接字的阻止。如果缺少该过滤器，请使用 `npm install -g @anthropic-ai/sandbox-runtime` 进行安装。

当缺少必需的依赖时，在安装之前 Dependencies 选项卡是唯一显示的选项卡。当仅缺少可选的 seccomp 过滤器时，Dependencies 选项卡会与其他选项卡一起显示。依赖检查在启动时运行，因此安装软件包后请重启 Claude Code，以便 `/sandbox` 检测到它们。

> [accordion] Ubuntu 24.04 及更高版本：允许 bubblewrap 创建用户命名空间
>
> 在 Ubuntu 24.04 及更高版本上，默认的 AppArmor 策略会阻止 bubblewrap 创建其隔离所需的用户命名空间。
>
> 要检查您的环境（包括 WSL2 内部）是否强制执行此限制，请运行 `sysctl kernel.apparmor_restrict_unprivileged_userns`。如果该命令返回 `0`，请跳过此步骤。如果输出 `No such file or directory` 错误，则表示该键不存在，也可以跳过此步骤。如果返回 `1`，请添加一个授予 `bwrap` 此能力的 AppArmor 配置文件：
>
> ```bash theme={null}
> sudo tee /etc/apparmor.d/bwrap > /dev/null <<'EOF'
> abi <abi/4.0>,
> include <tunables/global>
>
> profile bwrap /usr/bin/bwrap flags=(unconfined) {
>   userns,
>   include if exists <local/bwrap>
> }
> EOF
> ```
>
> 该配置文件仅适用于 `bwrap` 本身，而不适用于它在沙箱内运行的命令。重新加载 AppArmor 以使其生效：
>
> ```bash theme={null}
> sudo systemctl reload apparmor
> ```

> [accordion] WSL2 注意事项
>
> 在 PowerShell 中使用 `wsl -l -v` 检查您的 WSL 版本。如果看到 `Sandboxing requires WSL2`，则说明您的发行版运行的是 WSL1。请将其升级到 WSL2，或在不使用沙箱隔离的情况下运行 Claude Code。
>
> 在 WSL2 上，WSL 会通过 Unix 套接字将 Windows 二进制文件（例如 `cmd.exe`、`powershell.exe` 或 `/mnt/c/` 下的任何内容）的启动交给 Windows 主机处理，因此沙箱命令能否启动这些程序取决于沙箱的 [Unix 套接字设置](https://code.claude.com/docs/zh-CN/settings-reference.md#sandbox-network-allowunixsockets)：必须先安装可选的 seccomp 过滤器，才能阻止该套接字。要允许这些启动，请设置 `allowAllUnixSockets`，这会向沙箱命令开放所有 Unix 套接字。

### 沙箱模式

Claude Code 提供两种沙箱模式。在这两种模式下，沙箱都会强制执行相同的文件系统和网络限制；区别仅在于沙箱命令是自动批准还是需要明确的权限。

#### 自动允许模式

当命令在沙箱内运行时，Claude Code 会自动批准该命令，不会提示。当命令因匹配 [`excludedCommands`](#run-commands-outside-the-sandbox-with-excludedcommands) 或因 Claude [在沙箱外重试](#the-unsandboxed-retry-escape-hatch)而在沙箱外运行时，它会经过常规的[权限流程](https://code.claude.com/docs/zh-CN/permissions.md)。

连接到您未允许的主机的沙箱命令仍会留在沙箱中。[允许域名之外的主机](#hosts-outside-your-allowed-domains)介绍了由谁决定是否放行该连接。

即使在自动允许模式下，以下规则仍然适用：

* 采用 [`Bash` 拒绝规则所匹配的形式](https://code.claude.com/docs/zh-CN/permissions#bash-rule-limits)的命令仍会被阻止
* 被识别为针对[关键路径](https://code.claude.com/docs/zh-CN/permission-modes.md#which-paths-are-critical)的 `rm` 或 `rmdir` 命令仍会经过常规权限流程
* 限定内容范围的[询问规则](https://code.claude.com/docs/zh-CN/permissions.md)（如 `Bash(git push *)`）即使对沙箱命令也仍会强制提示
* 单独的 `Bash` 询问规则或等效的 `Bash(*)` 形式，对于在沙箱中运行的命令会被跳过；对于在沙箱外运行的命令仍然适用。在[计划模式](https://code.claude.com/docs/zh-CN/permission-modes.md#analyze-before-you-edit-with-plan-mode)下，该规则不会被跳过：它同样会对沙箱命令进行提示，包括只读命令
* [Monitor 工具](https://code.claude.com/docs/zh-CN/tools-reference.md#monitor-tool)的命令不会被自动批准，但仍会在沙箱中运行。要跳过提示，请添加与该命令匹配的[允许规则](https://code.claude.com/docs/zh-CN/permissions.md#bash)，例如 `Bash(npm run *)`

某些沙箱命令也会经过常规的[权限流程](https://code.claude.com/docs/zh-CN/permissions.md)，而不是被自动批准。这种情况会出现在诸如 `FOO=bar npm test` 之类的命令上（在命令前设置的变量可能会改变实际运行的程序），以及 `make CC=clang` 或 `eval "ls"` 上。此时由您的允许规则和权限模式决定是否显示提示，而命令仍在沙箱内运行。

对于这三个命令，与命令匹配的[允许规则](https://code.claude.com/docs/zh-CN/permissions.md#bash)可以避免反复出现提示。以下规则展示了这种模式：

* `Bash(make *)` 会批准 `make CC=clang`
* `Bash(FOO=bar npm *)` 会批准 `FOO=bar npm test`。当命令以变量赋值开头时，请在规则中包含该赋值
* `Bash(eval "ls")` 会批准 `eval "ls"`。诸如 `Bash(eval *)` 之类的通配符规则无法避免此提示，因此请编写与整个命令匹配的规则

与命令匹配的允许规则也会批准该命令的[沙箱外重试](#the-unsandboxed-retry-escape-hatch)，因此该命令可以在沙箱外运行且不提示。

> [!NOTE]
>
> 自动允许模式独立于您的权限模式设置运行，但有三个例外：[计划模式](https://code.claude.com/docs/zh-CN/permission-modes.md#analyze-before-you-edit-with-plan-mode)、带有[每命令允许域名](#per-command-allowed-domains-in-auto-mode)的自动模式命令，以及自动模式下对沙箱命令的[服务器端分类器审查](https://code.claude.com/docs/zh-CN/permission-modes.md#how-the-classifier-evaluates-actions)。即使您未处于"接受编辑"模式，启用自动允许后，沙箱 Bash 命令也会自动运行。这意味着，即使在 Manual 模式下（此时文件编辑工具会提示），在沙箱边界内修改文件的 Bash 命令也会在不提示的情况下执行。
>
> 在计划模式下，自动允许不会扩大批准范围；有关 Claude Code 在您制定计划时如何管控命令，请参阅[计划模式](https://code.claude.com/docs/zh-CN/permission-modes.md#analyze-before-you-edit-with-plan-mode)。

#### 常规权限模式

所有 Bash 命令都会经过常规权限流程，即使在沙箱中也是如此。这提供了更多控制，但需要更多批准。

#### 沙箱外重试逃生通道

沙箱外重试是为在沙箱内失败的命令（例如与沙箱不兼容的工具）提供的逃生通道。当沙箱阻止网络连接时，Claude Code 会在命令结果中指明被拒绝的主机，从而让 Claude 了解被阻止的内容。Claude 会分析失败原因，并可能使用 `dangerouslyDisableSandbox` 参数重试该命令。

重试的命令在沙箱外运行。在交互式终端会话中，由谁批准取决于您的权限模式：

* **`bypassPermissions` 模式**：重试在不提示的情况下运行
* **Manual 模式和 `acceptEdits` 模式**：您会收到标题为"Bash command (unsandboxed)"的提示
* **[自动模式](https://code.claude.com/docs/zh-CN/permission-modes.md#eliminate-prompts-with-auto-mode)**：由一个独立的分类器模型评估底层命令
* **`dontAsk` 模式**：Claude Code 拒绝该重试
* **计划模式**：请参阅 [Claude Code 在您制定计划时如何管控命令](https://code.claude.com/docs/zh-CN/permission-modes.md#analyze-before-you-edit-with-plan-mode)

以下规则和设置会改变由谁批准重试：

* **匹配的允许规则**：如果诸如 `Bash(curl *)` 之类的允许规则与命令匹配，它也会批准重试，因此该命令会在沙箱外运行且不提示
* **针对该参数的询问规则**：为 `Bash(dangerouslyDisableSandbox:true)` 添加[询问规则](https://code.claude.com/docs/zh-CN/permissions.md#match-by-input-parameter)，即可在 Bash 重试时收到提示。在自动模式和 `bypassPermissions` 模式下您也会收到提示，并且该规则优先于匹配的允许规则
* **[`permissions.blockReadsOutsideWorkingDirectories`](https://code.claude.com/docs/zh-CN/settings-reference#permissions-blockreadsoutsideworkingdirectories)**：[任何模式都不会自动批准的操作](https://code.claude.com/docs/zh-CN/permission-modes.md#actions-no-mode-auto-approves)介绍了在启用该设置时会提示的重试

当命令因 [`Sandbox is required` 消息](#commands-refused-with-sandbox-is-required)而被拒绝时，Claude Code 会忽略 `dangerouslyDisableSandbox` 参数，并同样拒绝该重试。

#### 使用严格沙箱模式关闭重试

您可以在[沙箱设置](https://code.claude.com/docs/zh-CN/settings-reference.md#sandbox-settings)中设置 `"allowUnsandboxedCommands": false` 来禁用沙箱外重试。禁用重试后，Claude Code 会忽略 `dangerouslyDisableSandbox` 参数。此后，在沙箱运行期间，Claude 运行的命令都会在沙箱中执行，除非它们匹配 `excludedCommands` 条目。要防止 Claude Code 在沙箱无法启动时在沙箱外运行命令，还需设置 [`failIfUnavailable`](https://code.claude.com/docs/zh-CN/settings-reference#sandbox-failifunavailable)。`/sandbox` 的 **Overrides** 选项卡将此设置显示为 **Strict sandbox mode**。

即使项目设置中设置了 `true`，在您的用户设置、`--settings` 或托管设置中设置的 `false` 仍然有效。用户设置中的 `false` 不会使沙箱成为管理员强制要求的，因此项目的其他沙箱设置仍然适用。在 v2.1.285 之前，项目的 `true` 会覆盖您用户设置中的 `false`。

如果您或您的管理员在托管设置中或通过 `--settings` 标志禁用了重试，沙箱将成为管理员强制要求的。此时 Claude Code 会忽略仓库文件中放宽沙箱的设置，包括 `excludedCommands` 条目。[管理员强制沙箱下的仓库设置](#repository-settings-under-an-admin-required-sandbox)列出了这些设置。

严格沙箱模式适用于 Claude 运行的命令。您自己在 [`!` shell 模式提示符](https://code.claude.com/docs/zh-CN/interactive-mode#shell-mode-with-prefix)处输入的命令会在沙箱外运行，除非会话属于以下情况之一：

* **[后台会话](https://code.claude.com/docs/zh-CN/agent-view.md)**：严格沙箱模式也涵盖 shell 模式命令
* **设置了 [`CLAUDE_CODE_SUBPROCESS_ENV_SCRUB`](https://code.claude.com/docs/zh-CN/env-vars#variables) 的 Linux 会话**：所有命令都在沙箱中运行，包括 shell 模式命令

在 v2.1.260 之前，严格沙箱模式会在每个会话中对 shell 模式命令进行沙箱隔离。

#### 临时目录

默认情况下，除工作目录外，每用户临时目录在沙箱内也是可写的。除非您[禁用文件系统隔离](#disable-filesystem-isolation)，否则 Claude Code 会为沙箱命令将 `$TMPDIR` 设置为此目录，因此写入临时文件的工具无需额外配置即可正常工作。

沙箱外命令在您的 shell 设置了 `$TMPDIR` 时会继承该值，因此在启用文件系统隔离时，沙箱命令和沙箱外命令会将 `$TMPDIR` 解析为不同的目录。如果您的 shell 未设置 `$TMPDIR` 或将其设为空，引用 `$TMPDIR` 的沙箱外命令会获得您的 [`CLAUDE_CODE_TMPDIR`](https://code.claude.com/docs/zh-CN/env-vars) 覆盖值；如果您未设置该覆盖值或覆盖值是一个较长的路径，则会获得操作系统的临时目录，因此该变量不会展开为空字符串。要在两者之间传递临时文件，请改为将其写入工作目录下。

## 配置沙箱隔离

通过 `settings.json` 文件自定义沙箱行为。完整的配置参考请参阅[设置](https://code.claude.com/docs/zh-CN/settings-reference.md#sandbox-settings)。

默认情况下，沙箱中的命令可以写入当前工作目录、每用户临时目录，以及通过 `--add-dir`、`/add-dir` 或 `permissions.additionalDirectories` [添加的任何目录](https://code.claude.com/docs/zh-CN/permissions.md#additional-directories-grant-file-access-not-configuration)。如果 `kubectl`、`terraform` 或 `npm` 等子进程命令需要写入这些目录之外的位置，请使用 `sandbox.filesystem.allowWrite` 授予对特定路径的访问权限：

```json theme={null}
{
  "sandbox": {
    "enabled": true,
    "filesystem": {
      "allowWrite": ["~/.kube", "/tmp/build"]
    }
  }
}
```

这些路径在操作系统层面强制执行，因此在沙箱内运行的所有命令（包括其子进程）都会遵守这些限制。当某个工具需要对特定位置的写入权限时，推荐使用这种方法，而不是使用 `excludedCommands` 将该工具完全排除在沙箱之外。

当您在多个[设置作用域](https://code.claude.com/docs/zh-CN/settings.md#settings-precedence)中定义同一个文件系统数组时，Claude Code 会将它们合并，组合来自每个作用域的路径，而不是用一个作用域的数组替换另一个作用域的数组。当某个条目受到[防止开发者放宽策略](#keep-developers-from-widening-the-policy)中的锁定约束时，Claude Code 会将该条目排除在合并之外。

如果您在 CLI 上使用 [`--setting-sources`](https://code.claude.com/docs/zh-CN/cli-reference) 或在 Agent SDK 中使用 [`settingSources`](https://code.claude.com/docs/zh-CN/agent-sdk/claude-code-features#control-filesystem-settings-with-settingsources) 排除了某个来源，Claude Code 在构建沙箱配置时会忽略该来源的 `sandbox.filesystem` 条目、`Edit` 权限规则以及 `Read` 拒绝规则。需要 Claude Code v2.1.246 或更高版本。

当您在会话期间编辑这些文件系统列表时，Claude Code 会[将更改应用到正在运行的会话](https://code.claude.com/docs/zh-CN/settings.md#when-edits-take-effect)，因此下一条沙箱命令将在新路径下运行。

沙箱文件系统路径使用标准约定：`/tmp/build` 是绝对路径，`~/.kube` 相对于您的主目录。这与 [Read 和 Edit 权限规则](https://code.claude.com/docs/zh-CN/permissions.md#read-and-edit)不同，后者使用 `//path` 表示绝对路径，使用 `/path` 表示相对于项目的路径。有关相对路径、末尾斜杠和通配符，请参阅[沙箱路径前缀](https://code.claude.com/docs/zh-CN/settings-reference.md#sandbox-path-prefixes)。

您还可以使用 `sandbox.filesystem.denyWrite` 和 `sandbox.filesystem.denyRead` 拒绝写入或读取访问，并使用 `sandbox.filesystem.allowRead` 在被拒绝的区域内重新允许特定路径。当读取规则重叠时，路径范围更窄的规则生效：

| 示例规则 | 结果 |
| :- | :- |
| `"denyRead": ["~/"]` 搭配 `"allowRead": ["~/projects"]` | `~/projects` 可读，主目录的其余部分仍被阻止。范围更窄的允许规则重新开放了被拒绝区域中的这一部分 |
| `"allowRead": ["~/"]` 搭配 `"denyRead": ["~/.env"]` | `~/.env` 仍被阻止，主目录的其余部分可读。拒绝规则在更宽的允许规则内依然有效，因此宽泛的允许规则不会悄无声息地重新暴露机密 |
| `"allowRead": ["~/"]` 搭配 `"denyRead": ["~/**/.env"]` | 主目录下的每个 `.env` 都仍被阻止，其余部分可读。[通配符拒绝规则](https://code.claude.com/docs/zh-CN/settings-reference.md#sandbox-path-prefixes)在更宽的允许规则内的效果与精确路径相同 |

下面的示例阻止读取整个主目录，同时仍允许读取当前项目。请将其放在项目的 `.claude/settings.json` 中，因为只有当配置位于项目设置中时，相对路径 `.` 才会解析为项目根目录：

```json theme={null}
{
  "sandbox": {
    "enabled": true,
    "filesystem": {
      "denyRead": ["~/"],
      "allowRead": ["."]
    }
  }
}
```

如果您将相同的配置放在 `~/.claude/settings.json` 中，`.` 将解析为 `~/.claude`，项目文件将仍被 `denyRead` 规则阻止。

要拒绝沙箱命令对主目录和挂载卷的读取访问，同时保持工作目录可读，请设置 [`permissions.blockReadsOutsideWorkingDirectories`](https://code.claude.com/docs/zh-CN/settings-reference#permissions-blockreadsoutsideworkingdirectories)，而不是编写路径规则。

### 使用 `excludedCommands` 在沙箱外运行命令

在 [`sandbox.excludedCommands`](https://code.claude.com/docs/zh-CN/settings-reference#sandbox-excludedcommands) 中列出命令模式，即可在沙箱外运行匹配的命令，这意味着没有文件系统限制，也没有网络代理。请将其用于无法在沙箱内工作、且您信任其拥有您全部访问权限的工具。如果某个工具只是需要多一个目录或多一个主机，可以尝试使用 `allowWrite` 或 `allowedDomains`，这样命令仍会保持在沙箱中。

此示例将 `docker compose` 命令移出沙箱。将其保存在 `~/.claude/settings.json` 中即可应用于您的所有项目：

```json theme={null}
{
  "sandbox": {
    "enabled": true,
    "excludedCommands": ["docker compose *"]
  }
}
```

Claude Code 会将您的条目与每次 Bash 和 Monitor 调用进行比对。一次调用是 Claude 发送的整个命令行，其中可以串联多条命令。以下规则决定一次调用是否离开沙箱：

* **以 ` *` 结尾模式**：条目使用与 `Bash(...)` [权限规则](https://code.claude.com/docs/zh-CN/permissions.md#permission-rule-syntax)相同的语法，不含通配符的模式为精确匹配。`docker` 只匹配不带参数的 `docker`。`docker *` 匹配带或不带参数的 `docker`
* **调用中的每条命令都必须匹配**：`npm ci && docker compose build` 会保持在沙箱中，除非有另一个条目覆盖 `npm ci`
* **Claude Code 匹配的是调用的文本**：在内部调用 `docker` 的脚本或 `make` 目标不会匹配，`/usr/local/bin/docker` 也不会匹配
* **某些调用会保持在沙箱中**：重定向到文件、`cd` 或诸如 `$(...)` 的命令替换会使整个调用保持在沙箱中。[参考条目](https://code.claude.com/docs/zh-CN/settings-reference.md#sandbox-excludedcommands)列出了更多会保持在沙箱中的调用
* **`git clone` 需要使用相对路径**：如果克隆指定了绝对路径、以 `~` 开头的路径或包含 `..` 段的路径，您的条目不会覆盖该克隆。如果克隆可以放在您的工作目录内，请让 Claude 使用不含 `..` 段的相对路径，例如 `vendor/lib`
* **条目的保存位置可能很重要**：当沙箱为[管理员强制要求](#repository-settings-under-an-admin-required-sandbox)时，Claude Code 会忽略 `.claude/settings.json` 和 `.claude/settings.local.json` 中的条目

被排除的命令会经过常规的权限流程：

* [只读命令](https://code.claude.com/docs/zh-CN/permissions.md#read-only-commands)以及您的允许规则所覆盖的命令无需确认提示即可运行
* 在自动模式下，分类器会审查其他被排除的命令
* 在 `bypassPermissions` 模式下，被排除的命令无需确认提示即可运行，除非有询问规则与之匹配

要确认某个条目是否匹配，请切换到 Manual 模式，并让 Claude 运行一条会更改内容的匹配命令，例如 `docker compose up -d`。权限提示的标题为“Bash command (unsandboxed)”。

> [!WARNING]
>
> 被排除的命令以您的全部访问权限运行。诸如 `docker *` 这样宽泛的条目涵盖了该工具能做的一切。如果您编写的模式涵盖了解释器、工作目录中的脚本，或者作用于该目录中某个文件的工具（正如 `docker compose` 作用于其 compose 文件那样），Claude 就可以写入该文件，然后在沙箱外运行它。范围更窄的模式会减少 Claude 可以在沙箱外运行的内容。

### 禁用文件系统隔离

将 `sandbox.filesystem.disabled` 设置为 `true` 可跳过文件系统隔离，同时保留网络隔离。下面的示例关闭了文件系统隔离，同时保留网络域名的允许列表：

```json theme={null}
{
  "sandbox": {
    "enabled": true,
    "filesystem": {
      "disabled": true
    },
    "network": {
      "allowedDomains": ["github.com", "*.npmjs.org"]
    }
  }
}
```

沙箱有两个相互独立的层：[文件系统隔离](#filesystem-isolation)控制沙箱命令可以读写哪些路径，[网络隔离](#network-isolation)控制它们可以访问哪些域名。关闭文件系统层后，沙箱命令将获得对主机文件系统不受限制的读写访问权限，而其网络出站流量仍限于您允许的域名。当您使用沙箱的目的是控制命令连接到哪里，而不是控制它们写入什么时，请关闭该层。

`sandbox.filesystem.disabled` 默认为 `false`。

> [!WARNING]
>
> 在文件系统隔离关闭且命令自动允许的情况下，沙箱命令可以写入后续命令会运行或读取的文件，例如 shell 启动文件、`$PATH` 上的可执行文件或 `~/.claude/settings.json`，并利用它们在下一次运行时扩大自身的访问权限。仅对您信任不会自行提升访问权限的工作负载将 `filesystem.disabled` 设置为 `true`。使用 [`allowManagedDomainsOnly`](#keep-developers-from-widening-the-policy) 锁定网络域名可以降低风险，但无法消除风险，因为该锁定仅适用于在沙箱内运行的命令。

#### 哪些设置可以禁用它

由于关闭文件系统隔离会扩大沙箱命令的能力范围，Claude Code 仅接受来自以下设置来源的 `filesystem.disabled`：

* 用户设置、托管设置和 `--settings` CLI 标志可以设置它。`.claude/settings.json` 和 `.claude/settings.local.json` 中的项目设置不能设置它，因此检出的项目无法关闭文件系统隔离。
* 当托管设置配置了任何 `sandbox.filesystem` 内容，或列出了任何 `"mode": "deny"` 的 `sandbox.credentials.files` 条目时，只有托管设置可以设置该键。这可防止开发者在此类部署中关闭文件系统隔离；要为所有人关闭它，请在托管设置中设置 `"disabled": true`。
* 当设置了 [`CLAUDE_CODE_SUBPROCESS_ENV_SCRUB`](https://code.claude.com/docs/zh-CN/env-vars) 时，Claude Code 会忽略来自所有来源（包括托管设置）的 `filesystem.disabled`，并保持文件系统隔离开启。

[有效的](https://code.claude.com/docs/zh-CN/settings-reference.md#invalid-credential-entries-in-managed-settings) `mask` 条目不会锁定该键，即使 Claude Code 在启动时对其[回退为 `deny`](#mask-credential-files) 也是如此。请将无法掩码的路径（例如凭据目录）在托管设置中列为显式的 `deny` 条目，这样会锁定该键。

#### 关闭文件系统隔离后的变化

设置 `filesystem.disabled` 会解除文件系统层本身强制执行的保护。由其他层强制执行的保护仍然有效：

| 保护 | 文件系统隔离关闭时 |
| - | - |
| `filesystem.denyRead` 和 [`credentials.files`](#protect-credentials) `deny` 读取阻止 | 不强制执行。两者都由文件系统层应用 |
| `credentials.envVars` `deny` 和 `mask` 条目 | 强制执行。环境变量清除独立于文件系统层 |
| 以掩码方式应用的 [`credentials.files` `mask` 条目](#mask-credential-files) | 强制执行：掩码独立于文件系统层。[已回退为 `deny`](#mask-credential-files) 的条目与任何 `deny` 条目一样不强制执行 |

另外还有两项变化：

* 沙箱命令会继承您 shell 的 `$TMPDIR`，而不是每用户临时目录，因为所有临时目录都可写，Claude Code 不再将命令重定向到每用户临时目录。

  在 Linux 上，父 shell 中通常未设置该变量。Bash 工具指引会告诉 Claude 使用 `mktemp -d` 创建临时工作目录，而不是依赖 `$TMPDIR`。
* [`autoAllowBashIfSandboxed`](https://code.claude.com/docs/zh-CN/settings-reference#sandbox-autoallowbashifsandboxed) 默认仍为 `true`，因此沙箱命令会继续在无确认提示的情况下运行。将其设置为 `false` 可对沙箱命令进行确认提示。

### 保护凭据

`sandbox.credentials` 设置声明需要防范沙箱命令访问的凭据文件和环境变量。每个条目指定一个文件路径或一个环境变量以及一个 `mode`。专用的 `credentials` 块使凭据规则集中在一起，并与通用文件系统规则分开。

对于 `"mode": "deny"` 的条目，文件路径在沙箱内被拒绝读取（与 `filesystem.denyRead` 施加的限制相同），环境变量则在每条沙箱命令运行前被取消设置。文件保护属于文件系统层，因此如果您[禁用文件系统隔离](#disable-filesystem-isolation)，它将不再生效；而环境变量保护仍然有效。

下面的示例阻止读取 AWS 凭据文件和 SSH 目录，并从沙箱命令的环境中移除 `GITHUB_TOKEN` 和 `NPM_TOKEN`：

```json theme={null}
{
  "sandbox": {
    "enabled": true,
    "credentials": {
      "files": [
        { "path": "~/.aws/credentials", "mode": "deny" },
        { "path": "~/.ssh", "mode": "deny" }
      ],
      "envVars": [
        { "name": "GITHUB_TOKEN", "mode": "deny" },
        { "name": "NPM_TOKEN", "mode": "deny" }
      ]
    }
  }
}
```

环境变量条目和文件条目也接受 `"mode": "mask"`，详见[掩码凭据](#mask-credentials)。

文件路径遵循与 `sandbox.filesystem.*` 设置相同的[前缀规则](https://code.claude.com/docs/zh-CN/settings-reference.md#sandbox-path-prefixes)。

Claude Code 会合并会话所加载的每个[设置作用域](https://code.claude.com/docs/zh-CN/settings.md#settings-precedence)中的 `deny` 条目。`deny` 条目只会收窄访问权限，因此任何作用域都可以添加此类条目，但任何作用域都无法移除其他作用域添加的条目。

当您[排除某个设置来源](#configure-sandboxing)时：

* **项目或本地设置**：Claude Code 不会应用其中的任何 `credentials` 条目。需要 Claude Code v2.1.246 或更高版本。
* **用户设置**：Claude Code 仍会应用 `~/.claude/settings.json` 中的 `deny` 条目，并将其[文件 `mask` 条目](#mask-credential-files)作为限制保留（这些限制不再授权代理替换真实值），但会丢弃其[环境变量 `mask` 条目](#mask-environment-variables)。

没有内置的凭据拒绝列表，因此只有您列出的文件和变量会受到限制。

`sandbox.credentials` 仅影响沙箱中的 Bash 命令。要从所有子进程中清除凭据（无论是否使用沙箱），请设置 [`CLAUDE_CODE_SUBPROCESS_ENV_SCRUB`](https://code.claude.com/docs/zh-CN/env-vars)。

### 掩码凭据

当您对凭据进行掩码时，Claude Code 会向沙箱命令显示一个每个会话的占位符（称为哨兵值），并由[沙箱代理](#network-isolation)在发往您允许的主机的出站请求中替换为真实值。而[保护凭据](#protect-credentials)中的 `deny` 条目则会阻止该凭据。对于 macOS 上的文件，Claude Code 会[阻止该文件](#mask-credential-files)，而不是对其进行掩码。[`sandbox.credentials`](https://code.claude.com/docs/zh-CN/settings-reference#sandbox-credentials) 参考列出了所有字段。

掩码需要满足以下条件：

* **TLS 终止**：代理在请求内容中替换真实值，因此它必须能够看到请求内容。请设置 [`network.tlsTerminate`](https://code.claude.com/docs/zh-CN/settings-reference#sandbox-network-tlsterminate)，使代理自行终止 TLS。如果不设置，掩码会失败，但不会暴露任何内容：命令仍然只能看到哨兵值，但哨兵值会原样到达服务器，导致身份验证失败。要检查此错误配置，请在终端中运行 `claude doctor`，并查看是否有 `TLS termination is unavailable` 警告。
* **允许的目标**：每个 `mask` 条目可以列出 `injectHosts`，即允许真实值到达的主机。代理只在[域名允许列表](#network-isolation)所允许的连接上注入凭据，因此每个 `injectHosts` 主机还必须能够通过 `network.allowedDomains` 访问。对于没有 `injectHosts` 的 `mask` 条目，代理会在发往 `network.allowedDomains` 中每个主机的请求中替换真实值。
* **受信任的设置作用域**：掩码会授权代理将您的真实凭据发送到某处，因此 Claude Code 只接受来自用户设置、托管设置和 `--settings` 标志的 `mask` 条目、`network.tlsTerminate`、[`credentials.allowPlaintextInject`](https://code.claude.com/docs/zh-CN/settings-reference#sandbox-credentials-allowplaintextinject)、`awsPairs` 和 `sigv4`。它会忽略仓库的 `.claude/settings.json` 或 `.claude/settings.local.json` 中的这些设置。当您的管理员通过服务器托管设置下发 `mask` 条目、`network.tlsTerminate` 或 `credentials.allowPlaintextInject` 时，它们属于[需要批准的设置](https://code.claude.com/docs/zh-CN/server-managed-settings.md#security-approval-dialogs)。

#### 掩码环境变量

要对环境变量进行掩码，请在其 `credentials.envVars` 条目上设置 `"mode": "mask"`。命令及其记录的任何日志都不会持有真实凭据，但其请求仍能通过身份验证。当同一变量在任何作用域中以 `deny` 列出时，`deny` 优先。

下面的示例对两个令牌进行掩码。`GH_TOKEN` 仅在发往 `api.github.com` 的请求中被替换，而 `NPM_TOKEN` 没有 `injectHosts`，因此会在发往 `network.allowedDomains` 中每个主机的请求中被替换：

```json theme={null}
{
  "sandbox": {
    "enabled": true,
    "network": {
      "tlsTerminate": {},
      "allowedDomains": ["*.github.com", "registry.npmjs.org"]
    },
    "credentials": {
      "envVars": [
        { "name": "GH_TOKEN", "mode": "mask", "injectHosts": ["api.github.com"] },
        { "name": "NPM_TOKEN", "mode": "mask" }
      ]
    }
  }
}
```

默认情况下，掩码会替换整个值。对于具有结构的值（例如 `DATABASE_URL` 连接字符串或 JWT），请使用 [`extract`、`decode`、`maskClaims` 和 `onExtractNoMatch` 字段](https://code.claude.com/docs/zh-CN/settings-reference#sandbox-credentials-envvars)，使解析该值的工具继续正常工作。

对于 IPv6 目标，在两个列表中的地址写法不同：

* **`network.allowedDomains`**：使用方括号形式，例如 `"[::1]"`
* **`injectHosts`**：使用规范压缩形式的裸地址，例如 `"::1"`

代理将每个 `injectHosts` 条目与连接的裸目标地址进行匹配，忽略端口，因此带方括号、带区域 ID 或采用其他压缩方式的写法永远不会匹配。`claude doctor` 会使用警告 `Sandbox credential injectHosts entries can never match their destination` 标记永远无法匹配的条目。此检查需要 Claude Code v2.1.229 或更高版本。

#### 重新签名 AWS 请求

AWS 请求携带基于请求内容的 SigV4 签名，因此请同时对 `AWS_ACCESS_KEY_ID` 和 `AWS_SECRET_ACCESS_KEY` 进行掩码。代理通过访问密钥的[哨兵值](#mask-credentials)识别 SigV4 请求，并使用真实值对请求重新签名，这需要 Claude Code v2.1.221 或更高版本。如果仅对私有密钥进行掩码，请求会使用代理无法识别的占位符签名，因此它们会在 AWS 处失败。

当您对常规的 `AWS_ACCESS_KEY_ID`、`AWS_SECRET_ACCESS_KEY` 和 `AWS_SESSION_TOKEN` 变量的整个值进行掩码时，Claude Code 会自动将它们关联为一个凭据。如果您的 AWS 凭据存储在其他名称的变量中，请使用 [`credentials.awsPairs`](https://code.claude.com/docs/zh-CN/settings-reference#sandbox-credentials-awspairs) 对它们进行分组，这需要 Claude Code v2.1.224 或更高版本。

流式上传、预签名 URL 和 SigV4A 请求携带代理无法重新计算的签名。当此类请求使用已掩码凭据对的占位符签名时，代理会使其失败，而不是转发损坏的签名。使用未掩码凭据签名的请求不受影响。请使用 [`credentials.sigv4`](https://code.claude.com/docs/zh-CN/settings-reference#sandbox-credentials-sigv4)（需要 Claude Code v2.1.224 或更高版本）来转发这些请求形式之一。AWS 仍会拒绝该请求，因此调用工具会收到 AWS 自身的拒绝响应，而不是代理错误。

#### 掩码凭据文件

要对凭据文件进行掩码，请在其 `credentials.files` 条目上设置 `"mode": "mask"`。掩码文件需要 Claude Code v2.1.221 或更高版本。沙箱命令看到的内容取决于平台：

* **Linux 和 WSL2**：沙箱命令读取的是该文件的[哨兵](#mask-credentials)副本，代理会在出站请求中替换为真实值。
* **macOS**：沙箱命令完全无法读取该文件。Claude Code 不会构建哨兵副本，因此使用该文件进行身份验证的工具无法在沙箱内工作，效果与 `deny` 相同。即使您[禁用文件系统隔离](#disable-filesystem-isolation)，该读取阻止仍然有效。

下面的示例对存储在 `~/.config/gh/hosts.yml` 中的 GitHub 令牌进行掩码。`extract` 模式标记文件的哪一部分是机密，因此在 Linux 和 WSL2 上，`gh` 仍能解析其配置的其余部分：

```json theme={null}
{
  "sandbox": {
    "enabled": true,
    "network": {
      "tlsTerminate": {},
      "allowedDomains": ["*.github.com"]
    },
    "credentials": {
      "files": [
        {
          "path": "~/.config/gh/hosts.yml",
          "mode": "mask",
          "extract": "oauth_token:\\s*(\\S+)",
          "injectHosts": ["api.github.com"]
        }
      ]
    }
  }
}
```

要确认掩码是否生效，请让 Claude 在沙箱命令中运行 `cat ~/.config/gh/hosts.yml`。在 Linux 和 WSL2 上，输出会显示一个替代令牌的哨兵值；在 macOS 上，读取则会失败。

如果不使用 `extract` 或 `decode`，Claude Code 会用一个哨兵值替换整个文件，这适用于只包含单个裸机密的文件。请使用 [`extract`、`decode`、`maskClaims`、`onExtractNoMatch` 和 `maskDuplicates` 字段](https://code.claude.com/docs/zh-CN/settings-reference#sandbox-credentials-files)来控制部分掩码，以及模式未匹配到任何内容时的处理方式。

> [!WARNING]
>
> 当匹配未找到任何可掩码的内容时，默认的 `onExtractNoMatch` 值 `warn` 会跳过该条目，因此沙箱命令可以读取未掩码的真实文件。在 macOS 上，只要文件系统隔离开启，Claude Code 就会在模式运行之前将 `mask` 条目作为 `deny` 应用，因此未匹配处理结果仅在[文件系统隔离关闭](#disable-filesystem-isolation)时才在 macOS 上生效。默认值适用于可能合法缺失的凭据。如果机密可能存在但模式可能漏匹配，请使用 [`deny`](https://code.claude.com/docs/zh-CN/settings-reference#mask-fields-for-files)。

`mask` 仅适用于单个文件，因此请逐个列出每个凭据文件。对于无法安全掩码的 `mask` 条目，Claude Code 会回退为 `deny`：目录路径、glob 模式、大于 8 MiB 的文件，或非 UTF-8 文本的文件。

## 沙箱隔离的工作原理

### 文件系统隔离

沙箱化的 Bash 工具将文件系统访问限制在特定目录内：

* **默认写入行为**：对当前工作目录及其子目录、通过 `--add-dir`、`/add-dir` 或 [`permissions.additionalDirectories`](https://code.claude.com/docs/zh-CN/settings-reference#permissions-additionaldirectories) 添加的任何目录，以及 `$TMPDIR` 所指向的每用户临时目录拥有读写权限
* **默认读取行为**：对整台计算机拥有读取权限，但某些被拒绝的目录除外。此默认设置仍允许读取凭据文件，因此请[保护凭据](#protect-credentials)，防止命令读取您不希望其读取的凭据。
* **读取阻止**：启用 [`permissions.blockReadsOutsideWorkingDirectories`](https://code.claude.com/docs/zh-CN/settings-reference#permissions-blockreadsoutsideworkingdirectories) 后，沙箱化的命令也会失去对您的主目录以及其他存放用户文件的目录的读取权限，[阻止状态下的沙箱化命令](https://code.claude.com/docs/zh-CN/settings-reference.md#sandboxed-commands-under-the-block)中列出的路径除外。该部分还说明了此部分阻止在何种情况下不适用。
* **Git worktree**：当工作目录是[链接的 git worktree](https://code.claude.com/docs/zh-CN/worktrees.md) 时，沙箱还允许写入主仓库共享的 `.git` 目录，以便 `git commit` 等命令能够更新 ref 和索引。对该目录内 `hooks/` 和 `config` 的写入仍被拒绝。

要完全跳过文件系统隔离而保留网络隔离，请设置 [`sandbox.filesystem.disabled`](#disable-filesystem-isolation)。

### 受保护的路径

在沙箱化命令可写入的目录中，沙箱仍会拒绝写入 Claude Code 从中加载配置和代码的文件。能够编辑这些文件的命令可能会为自己授予权限，或添加由 Claude Code 在沙箱之外运行的 hook 或 MCP 服务器。权限系统有其自己的[受保护的路径](https://code.claude.com/docs/zh-CN/permission-modes.md#protected-paths)，用于控制 Claude Code 在工具运行之前批准的内容；而沙箱的列表适用于已经在运行的命令。它涵盖四组路径：

* **在您的工作目录及其上级目录中**：`.claude` 设置文件，`.claude/skills`、`.claude/agents`、`.claude/commands` 和 `.claude/hooks` 目录，`.mcp.json`，以及 Claude Code 自行运行的文件，例如 `.claude/workflows` 和 `.claude/scheduled_tasks.json`
* **仅在您的工作目录中**：shell 启动文件（例如 `.bashrc` 和 `.zshrc`）、`.gitconfig`、`.vscode` 和 `.idea` 目录，以及 `.git` 内的 `hooks` 和 `config`
* **会将您的工作目录变成裸 git 仓库的文件**：顶层的 `HEAD`、`objects` 和 `refs`，以及当旁边存在 `HEAD` 文件或符号链接时，该处已有的 `config` 和 `hooks` 条目。即使没有 `HEAD`，名为 `config` 的文件也会被拒绝。[之后出现的 `HEAD`、`objects` 或 `refs` 可能会被删除](#when-the-sandbox-deletes-head-objects-or-refs)。在 Linux 和 WSL2 上，沙箱化命令可以在其不存在的位置创建一个。
* **在 `~/.claude` 或 `CLAUDE_CONFIG_DIR` 所指向的目录中**：其中的大部分内容，以及 `~/.claude.json` 和 `.credentials.json` 凭据存储

如果会话期间在受保护设置文件的路径上出现符号链接，沙箱还会从下一条命令开始拒绝写入该符号链接所指向的文件。

如果您添加了覆盖这些路径之一的 `allowWrite` 条目或 `Edit` 允许规则，保护仍然适用。要关闭此保护，请设置 [`filesystem.disabled`](#disable-filesystem-isolation)，这也会关闭文件系统隔离的其余部分。

> [!WARNING]
>
> 如果您在 Linux 或 WSL2 上将 [`allowNestedUserNamespaces`](https://code.claude.com/docs/zh-CN/settings-reference#sandbox-allownestedusernamespaces) 设为 `true`，请不要依赖受保护的路径：来自沙箱化命令的代码最终可能会在沙箱之外运行。

要查看其中大部分路径在您的计算机上解析后的结果，请运行 `/sandbox` 并打开 **Config** 标签页，其中会在 **Denied within allowed** 下列出这些路径，并与您自己的 `denyWrite` 条目混合显示。

如果 `git merge` 或 `git checkout` 在这些路径之一上因 `unable to unlink old` 而失败，请参阅[git 命令因 `unable to unlink old` 而失败](#a-git-command-fails-with-unable-to-unlink-old)。

#### 沙箱何时删除 `HEAD`、`objects` 或 `refs`

沙箱化命令结束后，工作目录中较新的顶层 `HEAD`、`objects` 或 `refs` 条目会被删除，除非它是包含内容的文件夹。"较新"是指沙箱上次构建其配置时该条目并不存在：即会话开始时，或在 [`/cd`](https://code.claude.com/docs/zh-CN/commands) 等更改之后。

如果沙箱上次构建其配置时存在 `HEAD` 文件或符号链接，那么较新的包含内容的 `objects` 或 `refs` 文件夹也会连同其内容一起被删除。要保留某个文件夹，请参阅[命令执行后名为 `objects` 或 `refs` 的文件夹消失](#a-folder-named-objects-or-refs-disappears-after-a-command)。

#### 经过符号链接的被拒绝路径

在 macOS 上，当被拒绝的路径经过符号链接时（例如 `~/.claude` 是一个指向 dotfiles 仓库的链接），沙箱也会拒绝该路径的真实位置。这涵盖沙箱自身的拒绝，以及您按字面写出的 `denyWrite` 和 `denyRead` 条目。

对于包含 `*`、`?` 或 `[` 的 `denyWrite` 条目，当这些字符中第一个出现之前的某个文件夹是符号链接时，同样适用。例如，`~/.claude/*.json` 在 `~/.claude` 的真实位置下也会被拒绝。

### 网络隔离

沙箱化的命令没有直接通往网络的路径：

* **Linux 和 WSL2**：命令在一个独立的网络命名空间中运行，该命名空间与您的网络没有连接
* **macOS**：Seatbelt 沙箱框架默认阻止除连接到沙箱代理之外的其他连接

Claude Code 在您的计算机上、沙箱之外运行沙箱代理，并通过 `HTTP_PROXY`、`HTTPS_PROXY`、`ALL_PROXY` 及相关环境变量将命令引导至该代理。代理会根据您允许和拒绝的域名检查每个连接的主机名。

工具能够访问的内容取决于它是否使用代理：

* **读取代理变量的工具**：`curl`、`npm`、基于 HTTPS 的 `git` 以及类似工具在其主机被允许后即可连接。不带端口的 `allowedDomains` 条目会允许该主机上的所有端口
* **忽略代理变量的工具**：普通的 `ssh`、大多数数据库驱动程序以及类似工具无法连接，即使是连接到被允许的主机也不行。请参阅[数据库客户端或其他非 HTTP 工具无法访问被允许的主机](#a-database-client-or-other-non-http-tool-fails-to-reach-an-allowed-host)
* **任何非 TCP 的流量**：UDP、基于 QUIC 的 HTTP/3 以及 `ping` 等 ICMP 工具无法离开沙箱

以下设置和行为控制代理允许哪些主机：

* **域名限制**：您允许的域名初始为空。[允许域名之外的主机](#hosts-outside-your-allowed-domains)介绍了命令首次需要新域名时会发生什么。
* **批准选项**：如果您在提示时选择 Yes，Claude Code 会在当前会话的剩余时间内允许该主机。如果您选择"Yes, and don't ask again"，Claude Code 会将一条 `WebFetch(domain:...)` 允许规则保存到您的[本地设置](https://code.claude.com/docs/zh-CN/permissions.md#permission-system)中，使该主机在以后的会话中保持允许状态。当沙箱处于[管理员要求](#repository-settings-under-an-admin-required-sandbox)状态时，Claude Code 会将该规则保存到您的用户设置中，使其在每个项目中都生效。
* **预先允许的域名**：使用 [`allowedDomains`](https://code.claude.com/docs/zh-CN/settings-reference#sandbox-network-alloweddomains) 预先允许域名，以完全避免提示。Claude Code 还会预先允许来自 `WebFetch(domain:...)` 允许规则的域名，如[权限规则](#permission-rules)中所述。
* **严格允许列表**：如果您在用户设置、托管设置或 CLI `--settings` 设置中将 [`strictAllowlist`](https://code.claude.com/docs/zh-CN/settings-reference#sandbox-network-strictallowlist) 设为 `true`，Claude Code 会拒绝沙箱化命令访问允许列表之外的任何主机，而不是进行提示。允许列表由 `allowedDomains` 加上来自 `WebFetch(domain:...)` 允许规则的域名组成；当设置了 `allowManagedDomainsOnly` 时，则仅包含托管设置中的条目。[无需管理员要求沙箱即可生效的锁定](#locks-that-apply-without-an-admin-required-sandbox)介绍了仓库中的条目。Claude Code 仅对沙箱化命令强制执行此设置；`WebFetch` 等进程内工具仍遵循其[权限规则](#permission-rules)。在仓库的 `.claude/settings.json` 或 `.claude/settings.local.json` 中设置此项无效。需要 Claude Code v2.1.219 或更高版本。
* **托管锁定**：如果在托管设置中设置了 [`allowManagedDomainsOnly`](https://code.claude.com/docs/zh-CN/settings-reference#sandbox-network-allowmanageddomainsonly)，未被允许的域名会被自动阻止而不会提示，并且只有托管设置中的 `allowedDomains` 和 `WebFetch(domain:...)` 允许规则会生效。
* **企业代理**：当您的网络要求出站流量通过企业代理时，请按照[代理配置](https://code.claude.com/docs/zh-CN/network-config.md#proxy-configuration)中的说明设置 `HTTPS_PROXY`、`HTTP_PROXY` 和 `NO_PROXY`，可以设置在您设置的 `env` 块中，以便[后台 Agent](https://code.claude.com/docs/zh-CN/network-config.md#set-network-variables-in-settings-not-the-shell) 也能获取它们，也可以设置在您启动 Claude Code 的环境中。Claude Code 会强制执行域名允许列表，然后通过该上游代理隧道传输被允许的连接。`http://` 和 `https://` 代理 URL 均可使用，如有需要，可在 URL 中包含基本身份验证信息。

在 `WebFetch(domain:...)` 规则中，沙箱支持两种通配符形式：前导 `*.`（例如 `*.example.com`）和单独的 `*`。单独的 `*` 形式需要 Claude Code v2.1.186 或更高版本。位于其他位置的通配符（例如 `WebFetch(domain:example.*)`）仍可匹配抓取请求，但对沙箱化命令没有任何作用。

> [!NOTE]
>
> 内置代理根据请求的主机名强制执行允许列表，默认情况下不会终止或检查 TLS 流量。实验性的 [`network.tlsTerminate`](https://code.claude.com/docs/zh-CN/settings-reference#sandbox-network-tlsterminate) 设置会让内置代理自行终止 TLS，这是 [`mask` 凭据条目](#mask-credentials)所必需的。有关默认行为的影响，请参阅[安全限制](#security-limitations)；如果您的威胁模型要求进行 TLS 检查，请参阅[自定义代理配置](#custom-proxy-configuration)。

#### 允许域名之外的主机

当沙箱化命令连接到不在您允许域名中的主机时，该命令会留在沙箱中并等待决定。在交互式终端会话中，该决定取决于您的权限模式：

| 权限模式 | 连接会发生什么 |
| :- | :- |
| `bypassPermissions` 模式，以及[可绕过权限](https://code.claude.com/docs/zh-CN/permission-modes.md#skip-all-checks-with-bypasspermissions-mode)时的计划模式 | 无需提示即被允许 |
| 手动模式、`acceptEdits` 模式，以及其他情况下的计划模式 | 您会收到提示 |
| 自动模式 | 被拒绝，除非命令[列出了该主机](#per-command-allowed-domains-in-auto-mode)且分类器批准了该列表 |
| `dontAsk` 模式 | 被拒绝 |

启用 [`strictAllowlist`](https://code.claude.com/docs/zh-CN/settings-reference#sandbox-network-strictallowlist) 或 [`allowManagedDomainsOnly`](https://code.claude.com/docs/zh-CN/settings-reference#sandbox-network-allowmanageddomainsonly) 后，内置沙箱代理会在所有权限模式下拒绝该连接。在 `bypassPermissions` 模式下，除非启用了其中之一，否则允许域名之外的主机会被允许。[非沙箱重试逃生通道](#the-unsandboxed-retry-escape-hatch)介绍了在该模式下命令何时可以离开沙箱。连接到 [`deniedDomains`](https://code.claude.com/docs/zh-CN/settings-reference#sandbox-network-denieddomains) 中的主机也会在所有权限模式下被拒绝。

#### 解析为本地地址的主机名

主机名通过允许列表后，沙箱代理会对其进行解析，当该名称仅解析为本地地址时拒绝连接。本地地址包括 `127.0.0.1` 等环回地址、`169.254.169.254` 云元数据端点等链路本地地址，以及分配给您自己计算机的地址。名称 `localhost` 和 `*.localhost` 允许解析为环回地址。

解析为 `10.0.0.0/8` 等私有地址范围的被允许内网主机名可以连接。要让某个名称解析为被拒绝的地址，请将该 IP 地址添加到 `allowedDomains`，例如 `"127.0.0.1:8080"`。

此检查适用于主机名。对 IP 地址的连接由您允许的域名和权限模式决定。对于通过上游企业代理发送的连接，代理也会跳过此检查，因为该名称由上游代理解析。

#### 自动模式下的逐命令允许域名

在启用沙箱隔离的[自动模式](https://code.claude.com/docs/zh-CN/permission-modes.md#eliminate-prompts-with-auto-mode)下，Claude 会在命令本身上指明该命令所需的主机，而不是为每个连接触发网络批准。在沙箱中运行的每个 Bash、PowerShell 或 [Monitor](https://code.claude.com/docs/zh-CN/tools-reference.md#monitor-tool) 命令都可以携带一个超出沙箱允许列表的主机列表：可以是 `registry.npmjs.org` 等域名、`*.pythonhosted.org` 等通配符或 IP 地址，每项均可带有可选的 `:port`。分类器会将这些主机与命令一起审查。需要 Claude Code v2.1.271 或更高版本。

经批准的列表仅对该单条命令开放这些主机，并在其运行期间有效。不会向会话允许的主机或您的设置中添加任何内容；下一条命令会指明它自己的主机。

携带主机的命令会交给分类器处理，而不是由权限规则或沙箱的[自动允许模式](#sandbox-modes)批准。如果某条[询问规则](https://code.claude.com/docs/zh-CN/permissions.md#manage-permissions)强制对该命令进行提示，终端中的权限对话框会在命令旁列出这些主机，在此处批准即同时涵盖两者。

逐命令列表只会放宽沙箱默认拒绝的内容。[`deniedDomains`](https://code.claude.com/docs/zh-CN/settings-reference#sandbox-network-denieddomains) 条目仍会阻止访问。当 [`strictAllowlist`](https://code.claude.com/docs/zh-CN/settings-reference#sandbox-network-strictallowlist) 或 [`allowManagedDomainsOnly`](https://code.claude.com/docs/zh-CN/settings-reference#sandbox-network-allowmanageddomainsonly) 锁定允许列表时，Claude Code 会拒绝逐命令列表。

在逐命令列表生效期间，Claude Code 会拒绝连接到任何未被已批准命令列出的主机，既不提示也不经过分类器检查。拒绝信息会在命令结果中指明该主机，Claude 会在添加该主机后重新运行命令。

#### 域名列表中的 IPv6 地址

要在 `allowedDomains`、`deniedDomains` 或 `WebFetch(domain:...)` 规则中匹配 IPv6 地址，请将地址写在方括号中：`"[::1]"` 匹配该地址的所有端口，`"[::1]:443"` 仅匹配其 443 端口。方括号形式需要 Claude Code v2.1.229 或更高版本。

不带方括号的条目（例如 `::1:443`）存在歧义，既可能是一个地址，也可能是一个地址加端口：

* **拒绝列表**：Claude Code 会拒绝该条目可解析出的每一种含义，因此无论您指的是哪种含义都会被阻止。对于无法解析出任何含义的条目，Claude Code 不会阻止任何内容
* **允许列表**：Claude Code 绝不会允许超出您所写的内容。当主机加端口的含义能够被清晰解析时，它会将有歧义的条目改写为该含义；它也可能会完全丢弃该条目，而不是扩大允许列表

要查找有歧义的条目，请在终端中运行 `claude doctor` 并查看 `Sandbox network domain entries have unreliable spellings` 警告。将每个有歧义的条目改写为方括号形式。

### 操作系统级强制执行

沙箱化的 Bash 工具使用操作系统安全原语：

* **macOS**：使用 Seatbelt 进行沙箱强制执行
* **Linux**：使用 [bubblewrap](https://github.com/containers/bubblewrap) 进行隔离
* **WSL2**：使用 bubblewrap，与 Linux 相同

您也可以单独运行 [`@anthropic-ai/sandbox-runtime`](https://github.com/anthropics/sandbox-runtime) 包来包装 Claude Code 进程。请参阅[沙箱运行时](https://code.claude.com/docs/zh-CN/sandbox-environments.md#sandbox-runtime)。

## 沙箱隔离与权限和权限模式的关系

沙箱隔离、[权限规则](https://code.claude.com/docs/zh-CN/permissions.md)和[权限模式](https://code.claude.com/docs/zh-CN/permission-modes.md)是互补的层级。下面的部分涵盖了沙箱隔离如何与每一个交互。

### 权限规则

权限规则和沙箱隔离控制不同的事项：

* **权限规则**控制 Claude Code 可以使用哪些工具，并在任何工具运行之前进行评估。它们适用于每个工具：Bash、Read、Edit、WebFetch、MCP 和其他工具，除了拒绝或询问规则无法阻止 [`EndConversation`](https://code.claude.com/docs/zh-CN/tools-reference#endconversation-tool-behavior)，而任何其他工具仍然存在。
* **沙箱隔离**提供操作系统级别的强制执行，限制 shell 命令在文件系统和网络级别可以访问的内容。它适用于 Bash、PowerShell 和 [Monitor](https://code.claude.com/docs/zh-CN/tools-reference.md#monitor-tool) 工具的命令及其子进程。

这两个层级在强制执行方式上也有所不同。Claude Code 在命令运行之前根据命令字符串和在自动模式下单独分类器对命令是否安全的判断来评估权限决策。操作系统在运行的进程上强制执行沙箱边界，因此无论模型选择运行什么，即使允许的命令执行的操作超出其名称所示，它也会保持有效。

文件系统和网络限制通过沙箱设置和权限规则进行配置：

| 设置或规则 | 作用 |
| :- | :- |
| `sandbox.filesystem.allowWrite` | 授予子进程对工作目录外路径的写入访问权限 |
| `sandbox.filesystem.denyWrite` 和 `sandbox.filesystem.denyRead` | 阻止子进程访问特定路径 |
| `sandbox.filesystem.allowRead` | 重新允许读取 `denyRead` 区域内的特定路径 |
| [`sandbox.filesystem.disabled`](#disable-filesystem-isolation) | 完全关闭文件系统层，同时保持网络隔离 |
| `Edit` 允许规则 | 授予对特定路径的写入访问权限，与 `sandbox.filesystem.allowWrite` 的方式相同 |
| `Read` 和 `Edit` 拒绝规则 | 阻止访问特定文件或目录 |
| `WebFetch(domain:...)` 允许和拒绝规则 | 控制域访问 |
| 沙箱 `allowedDomains` | 控制 Bash 命令可以访问哪些域 |
| 沙箱 `deniedDomains` | 阻止特定域，即使更广泛的 `allowedDomains` 通配符本来会允许它们 |

来自沙箱设置和权限规则的路径和域被合并到最终的沙箱配置中。

[claude-code 存储库的示例目录](https://github.com/anthropics/claude-code/tree/main/examples/settings)包含常见部署场景的启动设置配置，包括沙箱特定的示例。使用这些作为起点，并根据您的需求进行调整。

### 权限模式

`/sandbox` 不是[权限模式](https://code.claude.com/docs/zh-CN/permission-modes.md)。权限模式决定工具调用是否运行以及是否首先提示您，而沙箱限制 Bash 命令运行后可以访问的内容。它们在控制的内容和替代每个操作提示的内容上有所不同：

| | 控制的内容 | 替代提示的内容 |
| :- | :- | :- |
| `/sandbox` | Bash 命令运行后可以访问的内容 | 沙箱边界本身，在[自动允许模式](#sandbox-modes)中 |
| [自动模式](https://code.claude.com/docs/zh-CN/permission-modes.md#eliminate-prompts-with-auto-mode) | 每个工具调用是否运行 | 审查操作的分类器 |
| `--dangerously-skip-permissions` | 每个工具调用是否运行 | 无。[受保护路径](https://code.claude.com/docs/zh-CN/permission-modes.md#protected-paths)检查也被跳过；[任何模式都不会自动批准的操作](https://code.claude.com/docs/zh-CN/permission-modes.md#actions-no-mode-auto-approves)仍然适用 |

沙箱的[自动允许模式](#sandbox-modes)与[自动模式](https://code.claude.com/docs/zh-CN/permission-modes.md#eliminate-prompts-with-auto-mode)分开：自动允许批准 Bash 命令是因为沙箱边界包含它们，而自动模式使用分类器来审查操作。这两者独立工作，可以组合，但[沙箱模式](#sandbox-modes)下列出的例外除外。要为无人值守运行选择隔离边界，请参阅[沙箱环境](https://code.claude.com/docs/zh-CN/sandbox-environments.md#how-isolation-relates-to-permission-modes)。有关常见权限模式和沙箱配对及启动每个配对的标志的表格，请参阅[常见设置](https://code.claude.com/docs/zh-CN/permission-modes.md#common-setups)。

## 为您的组织配置沙箱

管理员可以为每个用户强制要求沙箱隔离，防止开发者扩大策略，并通过公司代理路由沙箱流量。

### 使用托管设置强制执行沙箱隔离

要为每个开发者强制要求沙箱，请通过[托管设置](https://code.claude.com/docs/zh-CN/managed-settings.md#delivery-mechanisms)下发 `sandbox` 设置项，可以是由您的 MDM 管理的文件，也可以是通过 claude.ai 上的[服务器托管设置](https://code.claude.com/docs/zh-CN/server-managed-settings.md)。

以下托管设置配置会启用沙箱，在平台不受支持或缺少依赖时拒绝启动 Claude Code，并防止模型在沙箱外重试命令：

```json theme={null}
{
  "sandbox": {
    "enabled": true,
    "failIfUnavailable": true,
    "allowUnsandboxedCommands": false
  }
}
```

除 `enabled` 之外的两个设置项控制沙箱无法运行命令时会发生什么：

* **`failIfUnavailable`**：缺少依赖（例如 Linux 上的 bubblewrap）时会阻止 Claude Code 启动，而不是回退到非沙箱化执行
* **`allowUnsandboxedCommands: false`**：Claude Code 忽略 `dangerouslyDisableSandbox` 逃生舱，因此当命令在沙箱下失败时，Claude 无法在沙箱外重试该命令

请考虑同时添加以下内容：

* 为任何必须在没有隔离的情况下运行的组织批准的工具添加 `excludedCommands`，因为此配置会[阻止仓库的设置将命令移出沙箱](#repository-settings-under-an-admin-required-sandbox)
* 为凭据目录（例如 `~/.aws` 和 `~/.ssh`）以及机密环境变量添加 [`sandbox.credentials`](#protect-credentials) 条目，因为默认读取策略仍允许访问这些内容

此配置对 Claude 运行的命令进行沙箱化。开发者仍然可以在 [`!` shell 模式提示符](https://code.claude.com/docs/zh-CN/interactive-mode#shell-mode-with-prefix)处键入命令并在沙箱外运行，其访问权限与他们在 Claude Code 之外的任何终端中已有的权限相同。有关键入的命令也在沙箱中运行的会话，请参阅[严格沙箱模式](#turn-off-the-retry-with-strict-sandbox-mode)。

沙箱无法在原生 Windows 上运行，因此设置 `failIfUnavailable` 后，Claude Code 会在这些机器上于启动时退出。如果您的设备群包含 Windows 主机，您可以：

* **按操作系统下发配置**：仅在 macOS 和 Linux 机器上通过您的 MDM 或作为[托管设置文件](https://code.claude.com/docs/zh-CN/managed-settings.md#delivery-mechanisms)进行部署。[服务器托管设置](https://code.claude.com/docs/zh-CN/server-managed-settings.md#current-limitations)适用于组织中的所有用户
* **将 Windows 用户迁移到受支持的环境**：让他们在 WSL2 或容器内运行 Claude Code

### 防止开发者扩大策略

当托管设置设定了布尔设置项（例如 `enabled` 或 `failIfUnavailable`）时，Claude Code 使用托管值并忽略开发者在本地设置的任何内容。对于数组设置项（例如 `allowRead`），Claude Code 会合并来自会话加载的各个作用域的条目，因此除非有锁定覆盖该设置项，否则开发者可以追加扩大策略的条目。

除非托管设置已设定，否则开发者的用户设置或 `--settings` 可以启用以下设置项。仓库的 `.claude/settings.json` 也可以启用，除非沙箱是[管理员强制要求的](#repository-settings-under-an-admin-required-sandbox)。其中每一项都会削弱沙箱，因此如果您不希望使用它们，请在托管设置中将其设置为 `false`：

* [`enableWeakerNestedSandbox`](https://code.claude.com/docs/zh-CN/settings-reference#sandbox-enableweakernestedsandbox)
* [`enableWeakerNetworkIsolation`](https://code.claude.com/docs/zh-CN/settings-reference#sandbox-enableweakernetworkisolation)
* [`network.allowAllUnixSockets`](https://code.claude.com/docs/zh-CN/settings-reference#sandbox-network-allowallunixsockets)
* [`network.allowLocalBinding`](https://code.claude.com/docs/zh-CN/settings-reference#sandbox-network-allowlocalbinding)
* [`allowAppleEvents`](https://code.claude.com/docs/zh-CN/settings-reference#sandbox-allowappleevents)，仓库无法启用此项
* [`allowNestedUserNamespaces`](https://code.claude.com/docs/zh-CN/settings-reference#sandbox-allownestedusernamespaces)，仓库无法启用此项

在托管设置中将 `allowManagedReadPathsOnly` 设置为 `true`，以便仅采用来自托管设置的 `allowRead` 条目。这可以防止开发者将读取访问权限扩大到组织批准的路径之外。要以相同的方式将网络域锁定为托管值，请设置 [`allowManagedDomainsOnly`](https://code.claude.com/docs/zh-CN/settings-reference#sandbox-network-allowmanageddomainsonly)。启用 `allowManagedDomainsOnly` 后，只有托管设置可以设置[代理端口](#custom-proxy-configuration)。

当托管设置配置了 `sandbox.filesystem` 或列出任何带有 `"mode": "deny"` 的 `sandbox.credentials.files` 条目时，仅托管设置可以设置 [`filesystem.disabled`](#disable-filesystem-isolation)，因此开发者无法使用该设置项关闭文件系统隔离。[有效的](https://code.claude.com/docs/zh-CN/settings-reference.md#invalid-credential-entries-in-managed-settings) `mask` 条目不会锁定该设置项。请参阅[哪些设置可以禁用它](#which-settings-can-disable-it)。

#### 管理员强制要求沙箱时的仓库设置

当以下任一设置生效时，沙箱即为管理员强制要求的：

* [`allowUnsandboxedCommands`](https://code.claude.com/docs/zh-CN/settings-reference#sandbox-allowunsandboxedcommands) 在托管设置中设置为 `false`，或通过 `--settings` 标志设置为 `false`（除非托管设置将其设置为 `true`）
* [`allowManagedDomainsOnly`](https://code.claude.com/docs/zh-CN/settings-reference#sandbox-network-allowmanageddomainsonly) 在托管设置中设置为 `true`

这些设置不会启用沙箱，因此还需设置 `enabled`。

当沙箱为管理员强制要求时，Claude Code 仅从托管设置、`--settings` 标志以及每个开发者的 `~/.claude/settings.json` 中读取放宽沙箱的设置。它会忽略仓库的 `.claude/settings.json` 和 `.claude/settings.local.json` 中的以下设置：

| 仓库设置 | Claude Code 忽略的内容 |
| :- | :- |
| `excludedCommands`、`ignoreViolations`、`network.allowedDomains`、`network.allowUnixSockets`、`network.allowMachLookup`、`network.httpProxyPort`、`network.socksProxyPort` | 所有条目 |
| `filesystem.allowWrite`、`Edit(...)` 允许规则、`permissions.additionalDirectories` | 每个条目为沙箱化命令授予的写入权限。Claude 的文件工具仍遵循 `Edit(...)` 规则和附加目录 |
| `WebFetch(domain:...)` 允许规则 | 每条规则添加到沙箱允许列表中的主机。WebFetch 工具仍遵循该规则 |
| `enableWeakerNestedSandbox`、`enableWeakerNetworkIsolation`、`network.allowAllUnixSockets`、`network.allowLocalBinding` | `true`。`false` 仍然生效 |
| `enabled`、`failIfUnavailable` | 当开发者的 `~/.claude/settings.json` 设置为 `true` 时的 `false` |
| `filesystem.allowRead` | 位于托管设置、`--settings` 或用户设置拒绝读取的路径处或其下的条目，或可能匹配此类路径的 glob |

当沙箱为管理员强制要求时，以下设置仍然生效：

* **在仓库的文件中**：拒绝条目和 `autoAllowBashIfSandboxed` 值。在托管设置中设置该设置项可防止仓库更改它
* **在开发者自己的设置中**：表中的设置在 `~/.claude/settings.json` 或 `--settings` 中仍然生效，除非有仅托管锁定（例如 `allowManagedDomainsOnly`）覆盖它们。其中大多数（例如 `excludedCommands` 和 `filesystem.allowWrite`）没有仅托管锁定

[使用托管设置强制执行沙箱隔离](#enforce-sandboxing-with-managed-settings)下的配置会使沙箱成为管理员强制要求的。请将您批准的工具所需的 `excludedCommands`、`allowWrite` 和套接字条目添加到托管设置中，因为仓库无法提供它们。

需要 Claude Code v2.1.285 或更高版本。在 v2.1.282 到 v2.1.284 中，相同的设置会使 Claude Code 忽略仓库的 `excludedCommands` 条目。

#### 无需管理员强制要求沙箱即可生效的锁定

即使沙箱不是管理员强制要求的，某些设置也会使 Claude Code 忽略直接覆盖某项限制的仓库设置项。每项设置仅在您于其所在行列出的文件中设置时才有此效果，仓库的其他沙箱设置仍然生效。需要 Claude Code v2.1.285 或更高版本。

| 设置 | 设置位置 | Claude Code 在仓库设置中忽略的内容 |
| :- | :- | :- |
| `network.deniedDomains` 或 `WebFetch(domain:...)` 拒绝规则 | 托管设置、`--settings` | `httpProxyPort` 和 `socksProxyPort` |
| `network.strictAllowlist` | 托管设置、`--settings`、用户设置 | 代理端口、`allowedDomains` 和 `WebFetch(domain:...)` 允许规则 |
| `filesystem.denyRead`、`Read(...)` 拒绝规则或 `credentials.files` 条目 | 托管设置、`--settings` | 位于托管设置、`--settings` 或用户设置拒绝读取的路径处或其下的 `allowRead`、`allowWrite`、`Edit(...)` 允许或 `additionalDirectories` 条目，或可能匹配此类路径的 glob |

这些锁定改变的是沙箱化命令可以访问的内容。WebFetch 工具和 Claude 的文件工具仍遵循仓库的规则和附加目录。

### 自定义代理配置

要使用您自己的工具检查、过滤或记录沙箱流量，请将内置的沙箱代理替换为您在同一台机器上运行的代理。

要通过网络中其他位置的公司代理路由沙箱流量，请改为设置 `HTTPS_PROXY`，如[网络隔离](#network-isolation)下的**公司代理**条目所述。这样，Claude Code 的允许列表仍然适用。

要将沙箱化命令定向到您的代理，请在[沙箱设置](https://code.claude.com/docs/zh-CN/settings-reference.md#sandbox-settings)中设置代理监听的 localhost 端口：

```json theme={null}
{
  "sandbox": {
    "network": {
      "httpProxyPort": 8080,
      "socksProxyPort": 8081
    }
  }
}
```

如果您设置了端口，同时也设置了 `HTTPS_PROXY` 或 `HTTP_PROXY`，Claude Code 不会将沙箱化命令发送到您的代理的内容转发到这些变量指定的代理。要访问公司代理，请配置您自己的代理转发到该代理。

哪些文件可以设置端口取决于您的其他沙箱设置。以第一个匹配的情况为准：

* **已启用 `allowManagedDomainsOnly`**：仅托管设置
* **沙箱为[管理员强制要求的](#repository-settings-under-an-admin-required-sandbox)，或适用[更窄的网络锁定](#locks-that-apply-without-an-admin-required-sandbox)**：托管设置、`--settings` 和用户设置
* **其他情况**：任何设置文件

Claude Code 会忽略在其他任何位置设置的端口。在 v2.1.285 之前，任何设置文件都可以设置端口。

> [!WARNING]
>
> 一旦任一端口生效，您的代理就负责过滤发送给它的所有内容。Claude Code 自身的网络控制（例如 `allowedDomains`、`deniedDomains`、`strictAllowlist`、批准提示和[本地地址检查](#hostnames-that-resolve-to-local-addresses)）将不再适用于该流量。沙箱化命令可以连接到任一代理，因此如果您只设置了一个端口，另一个代理上的 Claude Code 域列表无法限制该命令通过您的代理访问的内容。

## 故障排除

某些命令在沙箱内失败，即使它们在沙箱外可以正常工作。请找到与您的症状或错误消息相符的标题。

如果您的组织的沙箱是[管理员强制要求的](#repository-settings-under-an-admin-required-sandbox)，Claude Code 会忽略项目设置文件中这些修复所提到的设置，因此请将它们保存在 `~/.claude/settings.json` 中，这样它们会在每个项目中生效。如果某个修复仍然没有效果，可能是您的组织的托管设置设置了该键。

添加 `excludedCommands` 模式的修复会让该模式匹配的命令脱离沙箱。请参阅[被排除的命令可以做什么](#run-commands-outside-the-sandbox-with-excludedcommands)。

### 命令因主机不允许错误而失败

许多 CLI 工具需要访问特定的主机。在出现提示时批准该主机，或将其添加到 [`allowedDomains`](https://code.claude.com/docs/zh-CN/settings-reference#sandbox-network-alloweddomains)。如果您的组织使用 `allowManagedDomainsOnly` 锁定了允许列表，则不会出现提示，因此请让您的管理员添加该主机。

### `jest` 挂起或失败

`watchman` 与沙箱不兼容。改为运行 `jest --no-watchman`。

### 基于 Go 的 CLI 在 macOS 上 TLS 验证失败

`gh`、`gcloud` 和 `terraform` 等工具在 [Seatbelt](#os-level-enforcement) 下可能无法通过 TLS 验证。要在沙箱外运行这些工具，请为每个工具向 [`excludedCommands`](#run-commands-outside-the-sandbox-with-excludedcommands) 添加一个模式，例如 `gh *`。这样该工具将以您的完整访问权限及其存储的凭据运行。如果您将 `httpProxyPort` 与 MITM 代理和自定义 CA 一起使用，请改为将 [`enableWeakerNetworkIsolation`](https://code.claude.com/docs/zh-CN/settings-reference#sandbox-enableweakernetworkisolation) 设置为 `true`。

### `open`、`osascript` 或基于浏览器的身份验证流程在 macOS 上因错误 `-600` 失败

沙箱默认阻止 Apple Events。在您的用户、托管或 CLI 设置中将 [`allowAppleEvents`](https://code.claude.com/docs/zh-CN/settings-reference#sandbox-allowappleevents) 设置为 `true` 以允许它们。Claude Code 会忽略项目设置中的此键。

启用 `allowAppleEvents` 会移除代码执行隔离，因为沙箱化命令随后可以在没有用户提示的情况下启动其他未沙箱化的应用程序，并向正在运行的应用程序发送 AppleScript 命令，但受 macOS 自动化同意提示 (TCC) 的约束。或者，向 [`excludedCommands`](#run-commands-outside-the-sandbox-with-excludedcommands) 添加一个模式，例如 `open *`。这样每次 `open` 调用都会经过权限流程，而 `open` 可以启动任何文件或应用，包括 Claude 编写的文件或应用。

### `docker` 命令失败

`docker` 与沙箱不兼容。使用 `excludedCommands` 模式（例如 `docker compose *`）将您需要的 `docker` 命令移出沙箱。[使用 `excludedCommands` 在沙箱外运行命令](#run-commands-outside-the-sandbox-with-excludedcommands)说明了被排除的 `docker` 命令可以访问什么。模式越窄，移出沙箱的命令就越少。

### Chromium 或 rootless Podman 在 Linux 上的沙箱内失败

在 Linux 和 WSL2 上，沙箱中的命令默认无法创建用户命名空间，因此依赖用户命名空间的工具会失败：

* Chromium 退出并显示 `No usable sandbox!`
* `unshare -U true` 打印 `unshare: unshare failed: Operation not permitted`。启用 [`allowAllUnixSockets`](https://code.claude.com/docs/zh-CN/settings-reference#sandbox-network-allowallunixsockets) 时，它可能改为打印 `No space left on device`
* Rootless Podman 打印 `cannot clone: Operation not permitted`，或者更早地在其自身的某个文件夹上因 `read-only file system` 而停止

在将 [`CLAUDE_CODE_SUBPROCESS_ENV_SCRUB`](https://code.claude.com/docs/zh-CN/env-vars#variables) 设置为 `1` 的会话中，请使用 `--no-sandbox` 启动 Chromium 或 Electron，并在 Claude Code 之外自行运行任何其他工具。在该会话中，[每条命令都在沙箱中运行](#turn-off-the-retry-with-strict-sandbox-mode)，下面其他修复所提到的设置不起作用。

每种修复都适用于其所提到的工具，并会牺牲部分隔离：

* 对于 Chromium 或 Electron，请使用 `--no-sandbox` 启动浏览器。Claude Code 沙箱保持不变，但浏览器会在没有其自身沙箱的情况下运行网页内容。
* 对于 rootless Podman、Buildah，或运行浏览器或 `unshare` 的脚本或测试命令，请向 [`sandbox.excludedCommands`](#run-commands-outside-the-sandbox-with-excludedcommands) 添加一个模式，例如 `podman *`。匹配的命令会以您的完整访问权限在沙箱外运行，并且 Claude 可以在运行之前编辑您工作目录中的脚本。该模式必须匹配 Claude 运行的命令，因此对于浏览器，应匹配测试命令而不是浏览器。
* 要保留 Chromium 或 Electron 自身的沙箱，或运行嵌套的 bubblewrap 或 `unshare`，请在 `~/.claude/settings.json`（而不是项目的 `.claude/settings.json`）中将 [`sandbox.allowNestedUserNamespaces`](https://code.claude.com/docs/zh-CN/settings-reference#sandbox-allownestedusernamespaces) 设置为 `true`。需要 Claude Code v2.1.305 或更高版本。以 `unshare` 开头的命令无论您列出什么 `excludedCommands` 模式都会留在沙箱中，因为 `unshare` 可以启动任何其他程序。仅靠此设置不足以运行 rootless Podman 或 Buildah，因为沙箱还会以其他方式阻止它们。

> [!WARNING]
>
> 如果您将 `allowNestedUserNamespaces` 设置为 `true`，就会关闭一道安全边界：来自沙箱化命令的代码最终可能在沙箱外运行。
>
> 不要依赖沙箱的任何限制（包括[受保护路径](#protected-paths)和 [`credentials.files`](https://code.claude.com/docs/zh-CN/settings-reference#sandbox-credentials-files) 条目）来阻止恶意命令。在[自动允许模式](#auto-allow-mode)下，该命令可以在没有权限提示的情况下运行。如果您的组织有安全团队，请在启用此设置之前咨询他们。

### `pbcopy`、`xclip` 或 `wl-copy` 不更新剪贴板

`pbcopy`、`xclip` 和 `wl-copy` 剪贴板实用程序可能无法从沙箱内访问系统剪贴板，在这种情况下，通过管道传给它们的文本不会到达剪贴板。

要将 Claude 的输出放到您的剪贴板上，请让 Claude 在其回复中打印出来，然后运行 [`/copy`](https://code.claude.com/docs/zh-CN/commands)。`/copy` 从 Claude Code 进程而不是从沙箱化命令写入剪贴板。

当 Claude 将文本通过管道传给这些工具之一时，将该工具添加到 [`excludedCommands`](https://code.claude.com/docs/zh-CN/settings-reference#sandbox-excludedcommands) 本身并不会将该调用移出沙箱。

### git 命令因 `unable to unlink old` 失败

`git merge`、`git checkout` 和类似命令在需要替换沙箱拒绝写入的文件时会因 `unable to unlink old` 失败。在 Linux 和 WSL2 上，错误以 `Read-only file system` 结尾。该文件可能位于以下位置之一：

* 位于[受保护路径](#protected-paths)（如 `.claude/skills`）下
* 位于您的某个 `denyWrite` 条目下
* 根本位于沙箱允许命令写入的目录之外

失败后，Claude 可能会[提议在沙箱外重新运行该命令](#the-unsandboxed-retry-escape-hatch)。批准该重试，或在另一个终端中自行运行该 git 命令。如果您已将 `allowUnsandboxedCommands` 设置为 `false`，Claude 无法提供重试，因此请自行运行该命令。

### 名为 `objects` 或 `refs` 的文件夹在命令执行后消失

会话期间出现的顶层 `objects` 或 `refs` 文件夹可能会[在沙箱化命令结束后被删除](#when-the-sandbox-deletes-head-objects-or-refs)。无论它是由命令、Claude 还是您创建的，都是如此。要保留某个文件夹，请在其中放入一些内容。如果会话期间顶层曾存在 `HEAD` 文件或符号链接，请将其移走，然后重新启动在该位置打开的每个 Claude Code 会话。

在 v2.1.307 之前，即使文件夹中有内容且不存在 `HEAD`，该文件夹也会被删除。如果您使用的是更早的版本，请[更新 Claude Code](https://code.claude.com/docs/zh-CN/setup.md#update-claude-code)。

### Bubblewrap 在容器内启动失败

在无特权容器中，[bubblewrap](#os-level-enforcement) 无法挂载新的 `/proc` 文件系统，因此沙箱化命令会因 `bwrap` 错误（如 `Can't mount proc on /newroot/proc: Operation not permitted`）而失败。将 [`enableWeakerNestedSandbox`](https://code.claude.com/docs/zh-CN/settings-reference#sandbox-enableweakernestedsandbox) 设置为 `true`，以便沙箱改为绑定挂载容器现有的 `/proc`。仅当外部容器已提供您需要的隔离边界时才使用此设置，因为该设置会向沙箱化命令公开进程信息，而新的 `/proc` 挂载会隐藏这些信息。

### 0 字节只读文件出现在 `.claude` 设置路径中，且"是的，不要再问"无法保存

在 Linux 和 WSL2 上，沙箱化命令运行期间，沙箱通过在尚不存在的文件位置创建 0 字节只读占位符来保持对该文件的写入拒绝。沙箱随后会移除占位符。

在以下情况下，占位符会残留下来：

* 会话在清理运行之前被终止，例如通过 SIGKILL
* 会话在沙箱化命令仍在运行时退出，例如后台命令

后续会话每次启动时都会再次将这些占位符以只读方式绑定，因此在占位符所在的位置，设置写入（如保存权限选择）会失败。

在终端中运行 `claude doctor` 以列出残留的占位符文件。[`Sandbox mask files left behind at write-deny paths`](https://code.claude.com/docs/zh-CN/errors#stale-sandbox-mask-files-left-by-a-killed-session) 警告会列出其中部分文件的名称，并统计其余文件的数量。在该项目中没有其他 Claude Code 会话运行时，使用 `rm` 删除每个文件。在 v2.1.257 之前，会话被终止时 Claude Code 会留下相同的占位符，而不对其进行标记。

### 启用沙箱时通过 SSH 使用 `git` 失败

在 macOS 上，即使主机已被允许，针对 SSH 远程仓库的 `git fetch`、`git pull` 和 `git push` 在沙箱内也会失败。在 Linux 和 WSL2 上，只要主机被允许，它们就能正常工作。Claude Code 通过[沙箱代理](#network-isolation)隧道传输 git 的 SSH 连接，而 macOS 上的隧道无法向该代理进行身份验证。

在 Linux 和 WSL2 上，如果连接仍然失败，请检查以下几点：

* **主机在端口 22 上被允许**：不带端口的 `allowedDomains` 条目（例如 `"git.example.com"`）即可涵盖
* **您的企业代理允许端口 22**：如果您的网络需要上游代理，隧道也会经过该代理
* **密钥可以作为文件读取**：沙箱可能会阻止 `ssh-agent` 套接字，而针对 `~/.ssh` 的 `denyRead` 或 `credentials` 条目会隐藏您的密钥文件

在 macOS 上，将远程仓库切换为 HTTPS，这需要 HTTPS 凭据，例如个人访问令牌：

```bash theme={null}
git remote set-url origin https://git.example.com/example-org/example-repo.git
```

如果您必须保留 SSH 远程仓库，请使用 [`excludedCommands`](#run-commands-outside-the-sandbox-with-excludedcommands) 将 git 的网络命令移出沙箱：

```json theme={null}
{
  "sandbox": {
    "excludedCommands": ["git fetch *", "git pull *", "git push *"]
  }
}
```

这些条目匹配 `git push origin main`。添加了 `cd`、使用 `git -C` 或包含命令替换的调用仍会留在沙箱中。被排除的 git 命令可以访问任何主机，而不仅仅是 `allowedDomains` 中的主机。

通过 SSH 使用的普通 `ssh`、`scp` 和 `rsync` 会失败，原因见[数据库客户端条目](#a-database-client-or-other-non-http-tool-fails-to-reach-an-allowed-host)。

### 数据库客户端或其他非 HTTP 工具无法访问允许的主机

忽略代理环境变量的工具无法从沙箱内建立连接，即使目标是 `allowedDomains` 中的主机也是如此。沙箱化命令[没有直接访问网络的路由](#network-isolation)，因此自行建立连接的工具会失败。大多数数据库驱动程序、普通 `ssh` 以及使用 UDP 的工具都属于这种情况。

失败表现为网络或名称解析错误：

* **macOS**：`Operation not permitted`，或名称解析错误，例如 `Could not resolve host`
* **Linux 和 WSL2**：`Network is unreachable`，或名称解析错误，例如 `Temporary failure in name resolution`

使用代理的工具在其主机未被允许时会以不同方式失败。您会收到网络提示，或者该工具会收到来自代理的 `403` 响应。

要让该工具能够连接，请使用 [`excludedCommands`](#run-commands-outside-the-sandbox-with-excludedcommands) 在沙箱外运行需要它的命令。此示例排除了一个脚本，并添加了一条 [ask 规则](https://code.claude.com/docs/zh-CN/permissions.md)，以便您批准每次运行：

```json theme={null}
{
  "sandbox": {
    "excludedCommands": ["python scripts/load_orders.py *"]
  },
  "permissions": {
    "ask": ["Bash(python scripts/load_orders.py *)"]
  }
}
```

该脚本以您的完整访问权限运行，而 Claude 可以编辑位于您工作目录内的脚本，因此请在出现提示时审查它。

### 命令无法访问 localhost 上的服务器

默认情况下，沙箱化命令无法直接连接到在您的机器上、沙箱外运行的服务器，例如开发服务器或容器中的数据库。您可以更改的内容取决于您的平台：

* **macOS**：将 [`network.allowLocalBinding`](https://code.claude.com/docs/zh-CN/settings-reference#sandbox-network-allowlocalbinding) 设置为 `true`。这样沙箱化命令就可以监听网络端口并连接到 localhost 上的任何端口，包括在那里监听的所有其他服务。不需要身份验证的 localhost 服务（例如调试器）随后可以代表该命令在沙箱外执行操作，而在非回环地址上监听的命令会接受来自其他机器的连接
* **Linux 和 WSL2**：沙箱化命令的 `localhost` 是该命令私有的。该命令可以监听端口并访问它自己启动的服务器。直接连接到 `localhost` 或 `127.0.0.1` 无法访问主机上的服务器，且 `allowLocalBinding` 不起作用。使用 [`excludedCommands`](#run-commands-outside-the-sandbox-with-excludedcommands) 在沙箱外运行需要访问主机服务器的命令，在那里它不受任何文件系统或网络限制。对于经过沙箱代理的连接，请参阅[解析为本地地址的主机名](#hostnames-that-resolve-to-local-addresses)

此示例在 macOS 上启用该设置：

```json theme={null}
{
  "sandbox": {
    "network": {
      "allowLocalBinding": true
    }
  }
}
```

针对 `localhost` 的 `allowedDomains` 条目适用于经过代理的连接，因此它不会改变直接连接。Claude Code 为沙箱化命令设置了 `NO_PROXY`，以便它们直接连接到 `localhost`，而不经过代理。该条目还会将您机器 localhost 上的每个端口暴露给确实使用代理的命令。对于指向 `127.0.0.1` 的开发主机名，请参阅[允许的主机名因 `resolved to a loopback address` 被拒绝](#an-allowed-hostname-is-refused-with-resolved-to-a-loopback-address)。

### 允许的主机名因 `resolved to a loopback address` 被拒绝

沙箱代理会拒绝[解析为本地地址](#hostnames-that-resolve-to-local-addresses)的允许主机名，这会影响指向 `127.0.0.1` 的开发名称，例如 `myapp.test`。命令会收到一个 `403` 响应，其正文会指明地址类型，例如 `Connection to myapp.test blocked: resolved to a loopback address`。

在 `allowedDomains` 中将该名称解析到的 IP 地址与主机名一起添加，并分别附上您的服务器监听的端口：

```json theme={null}
{
  "sandbox": {
    "network": {
      "allowedDomains": ["myapp.test:3000", "127.0.0.1:3000"]
    }
  }
}
```

不带端口的 IP 地址条目会让沙箱化命令能够访问在该地址上监听的所有服务。

在 v2.1.284 之前，代理会连接到允许的主机名所解析到的任何地址。

### 命令因 `Sandbox is required` 被拒绝

如果您通过 [`sandbox.failIfUnavailable`](https://code.claude.com/docs/zh-CN/settings-reference#sandbox-failifunavailable) 要求使用沙箱，而沙箱在 Claude Code 启动后以下列方式之一失败，则 Claude 运行的每条沙箱化命令都会被拒绝，并显示相应的消息。会话会继续运行，被拒绝的命令不会在沙箱外运行：

* **`Sandbox is required but failed to initialize: <reason>`**：Claude Code 已启动，但沙箱随后初始化失败。请执行消息中给出的步骤：`Restart to retry.` 或 `Fix the sandbox settings to retry`。
* **`Sandbox is required (sandbox.failIfUnavailable) and stopped being available in this session: <reason>`**：在 Linux 或 WSL2 上，沙箱已启动，但 Claude Code 随后发现缺少某个依赖项。请修复原因中指出的问题，例如[安装缺失的依赖项](#set-up-linux-and-wsl2)，然后重新启动 Claude Code。

被排除的命令和您自行输入的命令会继续运行，且 Claude 无法在沙箱外重试被拒绝的命令：

* **被排除的命令**：匹配 [`excludedCommands`](#run-commands-outside-the-sandbox-with-excludedcommands) 条目的命令会继续在沙箱外运行。当其中某条命令写入您工作目录之外的 [`filesystem.allowWrite`](https://code.claude.com/docs/zh-CN/settings-reference#sandbox-filesystem-allowwrite) 路径时，该写入会经过常规权限流程。在[管理员强制要求的沙箱](#repository-settings-under-an-admin-required-sandbox)下，Claude Code 会忽略来自仓库设置的条目
* **您在 [`!` shell 模式提示符](https://code.claude.com/docs/zh-CN/interactive-mode#shell-mode-with-prefix)处输入的命令**：继续在沙箱外运行。在[后台会话](https://code.claude.com/docs/zh-CN/agent-view.md)中，它们也会被拒绝
* **在沙箱外重试**：Claude Code 会忽略[在沙箱外重试](#the-unsandboxed-retry-escape-hatch)的 `dangerouslyDisableSandbox` 参数

在设置了 `CLAUDE_CODE_SUBPROCESS_ENV_SCRUB` 的 Linux 会话中，每条命令都在沙箱中运行，因此被排除的命令和您输入的命令也会被拒绝。

### `/sandbox` 因 `Sandbox settings are overridden by a higher-priority configuration` 失败

当更高的[设置级别](https://code.claude.com/docs/zh-CN/settings.md#settings-precedence)设置了 `sandbox.enabled`、`sandbox.autoAllowBashIfSandboxed` 或 `sandbox.allowUnsandboxedCommands` 时，`/sandbox` 会打印 `Error: Sandbox settings are overridden by a higher-priority configuration and cannot be changed locally.`，而不是打开其面板。该面板会将您的选择保存到 `.claude/settings.local.json`，而保存在那里的值无法覆盖这些级别。

托管设置和 `--settings` 的优先级高于本地设置。要查看此会话加载了其中哪些，请运行 `/status` 并查看 `Setting sources` 行：

* **`Command line arguments`**：如果您使用 [`--settings`](https://code.claude.com/docs/zh-CN/settings#change-a-setting-for-one-session) 启动了 Claude Code，请检查您传入的文件或 JSON 是否设置了上述某个键。如果是，请在那里更改该值，或在不带这些键的情况下重新启动 Claude Code。
* **`Enterprise managed settings`**：已加载您的组织的托管设置。如果它们设置了上述某个键，您无法通过 `/sandbox` 或您控制的任何设置文件更改该键，因此请联系您的管理员。

### `/sandbox` 打印 `Sandbox settings not changed`

此行表示 `/sandbox` 无法将您在其面板中所做的选择保存到项目的 `.claude/settings.local.json`：

```text theme={null}
✗ Sandbox settings not changed: /home/you/project/.claude/settings.local.json couldn't be updated (EACCES). Start Claude Code in a project folder you can write to.
```

如果括号中的原因是 `EACCES`、`EPERM` 或 `EROFS`，请在您可以写入的项目文件夹中启动 Claude Code，然后在该会话中再次运行 `/sandbox`。如果原因以 `Invalid JSON syntax in settings file` 开头，请更正项目的 `.claude/settings.local.json` 中的 JSON，然后再次运行 `/sandbox`。

如果您没有可写入的项目文件夹，请改为从用户设置中启用沙箱。将以下内容添加到 `~/.claude/settings.json`：

```json theme={null}
{
  "sandbox": {
    "enabled": true
  }
}
```

这会启用[自动允许模式](#sandbox-modes)。如需使用常规权限模式，还需将 [`sandbox.autoAllowBashIfSandboxed`](https://code.claude.com/docs/zh-CN/settings-reference#sandbox-autoallowbashifsandboxed) 设置为 `false`。该更改会在 Claude 运行下一条 shell 命令时在您正在运行的会话中生效，随后 `/sandbox` 的 **Mode** 选项卡会标记当前生效的模式。

## 限制

沙箱降低风险，但不是完整的隔离边界。在依赖它作为硬安全控制之前，请查看下面的限制。

### 安全限制

* **网络过滤**：沙箱限制进程可以连接的域。默认情况下，内置代理不会终止或检查出站流量上的 TLS，因此不会检查加密连接的内容。实验性的 [`network.tlsTerminate`](https://code.claude.com/docs/zh-CN/settings-reference#sandbox-network-tlsterminate) 设置在代理处终止 TLS 以进行 [`mask` 凭据替换](#mask-credentials)，但不添加内容过滤。您负责确保策略中只允许受信任的域。

> [!WARNING]
>
> 允许广泛的域名（例如 `github.com`）可能会为数据泄露创建路径。因为代理从客户端提供的主机名做出允许决定而不检查 TLS，在沙箱内运行的代码可能会使用 [domain fronting](https://en.wikipedia.org/wiki/Domain_fronting) 或类似技术来到达允许列表外的主机。如果您的威胁模型需要更强的保证，请配置一个 [custom proxy](#custom-proxy-configuration)，它终止 TLS 并检查流量，并在沙箱内安装其 CA 证书。更强的 TLS 感知网络隔离是一个活跃的开发领域。

* **通过 Unix 套接字的权限提升**：`allowUnixSockets` 配置可能会无意中授予对可能导致沙箱绕过的系统服务的访问权限。例如，允许访问 `/var/run/docker.sock` 有效地通过 Docker 套接字授予对主机系统的访问权限。请仔细考虑通过沙箱允许的任何 Unix 套接字。
* **文件系统权限提升**：过于宽泛的文件系统写入权限可能导致权限提升攻击。允许写入包含 `$PATH` 中的可执行文件、系统配置目录或用户 shell 配置文件（例如 `.bashrc` 或 `.zshrc`）的目录可能导致当其他用户或系统进程访问这些文件时在不同的安全上下文中执行代码。
* **Linux 沙箱强度**：Linux 实现提供强大的文件系统和网络隔离，但包括一个 `enableWeakerNestedSandbox` 模式，使其能够在 Docker 环境中工作而无需特权命名空间。此选项大大削弱了安全性，应仅在其他隔离被强制执行时使用。
* **macOS 上的 Apple Events**：macOS 沙箱默认阻止 Apple Events。`allowAppleEvents` 设置解除此限制，以便 `open` 和 `osascript` 等工具可以工作，但它移除了代码执行隔离：沙箱化命令可以在没有用户提示的情况下启动其他未沙箱化的应用程序，并可以向运行的应用程序发送 AppleScript 命令，受限于每个应用程序的 macOS 自动化同意提示 (TCC)。它仅从用户、托管或 CLI 设置中被遵守。项目设置无法启用它。
* **Linux 和 WSL2 上的用户命名空间**：默认情况下，沙箱化命令无法创建用户命名空间。如果您将 [`allowNestedUserNamespaces`](https://code.claude.com/docs/zh-CN/settings-reference#sandbox-allownestedusernamespaces) 设置为 `true`，就会关闭一道安全边界：来自沙箱化命令的代码最终可能在沙箱外运行。在启用此设置之前，请阅读 [Linux 上 Chromium 或 rootless Podman 在沙箱内运行失败](#chromium-or-rootless-podman-fails-inside-the-sandbox-on-linux)中的警告。

### 范围

沙箱隔离 shell 命令及其子进程。[在沙箱外运行的内容](#what-runs-outside-the-sandbox)列出了它不涵盖的工具和辅助进程。计算机使用和子代理与沙箱的关系如下：

* **计算机使用**：当 Claude 打开应用程序并控制您的屏幕时，它在您的实际桌面上运行，而不是在隔离的环境中。每个应用程序的权限提示控制每个应用程序。请参阅 [CLI 中的计算机使用](https://code.claude.com/docs/zh-CN/computer-use.md) 或 [Desktop 中的计算机使用](https://code.claude.com/docs/zh-CN/desktop.md#let-claude-use-your-computer)。
* **子代理**：[子代理](https://code.claude.com/docs/zh-CN/sub-agents.md) 在与父会话相同的进程中运行，并使用相同的沙箱配置。当在父会话中启用沙箱隔离时，子代理内的 Bash 命令被沙箱化。
* **后台会话**：[后台会话](https://code.claude.com/docs/zh-CN/agent-view.md) 在其自己的进程中运行，当[其设置](https://code.claude.com/docs/zh-CN/agent-view.md#settings-and-provider)启用沙箱隔离时，其 Bash 命令会被沙箱化。
* **围绕整个进程的边界**：若要将[在沙箱外运行的内容](#what-runs-outside-the-sandbox)中的进程也置于边界之后，请在 [sandbox runtime](https://code.claude.com/docs/zh-CN/sandbox-environments.md#sandbox-runtime) 中运行 Claude Code（本地模式），并将网络允许列表限定为您批准的主机；或者在带有防火墙脚本的 [dev container](https://code.claude.com/docs/zh-CN/devcontainer.md) 中运行。对于后台服务及其托管的会话，请参阅[在企业启动器后运行 Claude Code](https://code.claude.com/docs/zh-CN/corporate-launcher.md)。
* **Mods**：[mod](https://code.claude.com/docs/zh-CN/plugins/mods/overview.md) 是一种在 Claude Code 内运行自身代码的插件，由 mod 启动的进程在沙箱外运行。请参阅 [mod 可以访问的内容](https://code.claude.com/docs/zh-CN/plugins/mods/overview.md#what-a-mod-can-reach)。

> [!WARNING]
>
> 有效的沙箱隔离需要同时进行文件系统和网络隔离。没有网络隔离，被入侵的 Agent 可能会泄露敏感文件，如 SSH 密钥。没有文件系统隔离，无论是来自宽泛的策略还是来自 [disabling the filesystem layer](#disable-filesystem-isolation)，被入侵的 Agent 可能会在系统资源中植入后门以获得网络访问权限。当您扩大默认值时，请检查 `allowWrite` 路径、广泛的 `allowedDomains` 条目或 `excludedCommands` 例外是否会撤销另一侧的限制。

## 另请参阅

* [Sandbox environments](https://code.claude.com/docs/zh-CN/sandbox-environments.md)：比较内置沙箱与开发容器、容器和虚拟机
* [Security](https://code.claude.com/docs/zh-CN/security.md)：安全功能和最佳实践
* [Permissions](https://code.claude.com/docs/zh-CN/permissions.md)：权限配置和访问控制
* [All settings](https://code.claude.com/docs/zh-CN/settings-reference.md)：每个设置键
* [CLI reference](https://code.claude.com/docs/zh-CN/cli-reference.md)：命令行选项
