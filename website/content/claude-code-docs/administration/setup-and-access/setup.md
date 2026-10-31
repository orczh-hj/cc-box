> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# 高级设置

> Claude Code 的系统要求、特定平台安装、版本管理和卸载。

本页面涵盖系统要求、特定平台安装详情、更新和卸载。有关首次会话的引导式演练，请参阅[快速入门](https://code.claude.com/docs/zh-CN/quickstart.md)。如果您从未使用过终端，请参阅[终端指南](https://code.claude.com/docs/zh-CN/terminal-guide)。

## 系统要求

Claude Code 在以下平台和配置上运行：

* **操作系统**：
  * macOS 13.0+
  * Windows 10 1809+ 或 Windows Server 2019+
  * Ubuntu 20.04+
  * Debian 10+
  * Alpine Linux 3.19+
* **硬件**：4 GB+ RAM、x64 或 ARM64 处理器
* **网络**：需要互联网连接。请参阅[网络配置](https://code.claude.com/docs/zh-CN/network-config.md#network-access-requirements)。
* **Shell**：Bash、Zsh、PowerShell 或 CMD。
* **位置**：[Anthropic 支持的国家/地区](https://www.anthropic.com/supported-countries)

### 其他依赖项

* **ripgrep**：通常包含在 Claude Code 中。如果搜索失败，请参阅[搜索故障排除](https://code.claude.com/docs/zh-CN/troubleshooting.md#search-and-discovery-issues)。

## 安装 Claude Code

> [!TIP]
>
> 更喜欢图形界面？[桌面应用](https://code.claude.com/docs/zh-CN/desktop-quickstart.md)让您无需使用终端即可使用 Claude Code。下载适用于 [macOS](https://claude.ai/api/desktop/darwin/universal/dmg/latest/redirect?utm_source=claude_code\&utm_medium=docs)、[Windows](https://claude.com/download?utm_source=claude_code\&utm_medium=docs) 或 [Linux](https://code.claude.com/docs/zh-CN/desktop-linux.md) 的版本。

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

安装完成后，在您要使用的项目中打开终端并启动 Claude Code：

```bash theme={null}
claude
```

Claude Code 在您的终端中打开一个交互式会话。

如果在安装过程中遇到任何问题，请参阅[故障排除安装和登录](https://code.claude.com/docs/zh-CN/troubleshoot-install.md)。

### 在 Windows 上设置

您可以在 Windows 上原生运行 Claude Code，也可以在 WSL 中运行。根据您的项目位置和所需的功能进行选择：

| 选项 | 需要 | [沙箱](https://code.claude.com/docs/zh-CN/sandboxing.md) | 何时使用 |
| - | - | - | - |
| [原生 Windows](#install-on-native-windows) | 无；[Git for Windows](https://git-scm.com/downloads/win) 是可选的 | 不支持 | Windows 原生项目和工具 |
| [WSL 2](#install-in-wsl) | WSL 2 已启用 | 支持 | Linux 工具链或沙箱命令执行 |
| [WSL 1](#install-in-wsl) | WSL 1 已启用 | 不支持 | 如果 WSL 2 不可用 |

#### 在原生 Windows 上安装

从 PowerShell 或 CMD 运行[安装命令](#install-claude-code)。您无需以管理员身份运行。安装 [Git for Windows](https://git-scm.com/downloads/win) 是可选的。它提供 Git Bash，[Bash 工具](https://code.claude.com/docs/zh-CN/tools-reference.md#bash-tool-behavior)和 [Monitor 工具](https://code.claude.com/docs/zh-CN/tools-reference.md#monitor-tool)需要用到它。

无论您从 PowerShell 还是 CMD 安装，只会影响您运行的安装命令。您的提示在 PowerShell 中显示为 `PS C:\Users\YourName>`，在 CMD 中显示为 `C:\Users\YourName>`（不带 `PS`）。如果您是终端新手，[终端指南](https://code.claude.com/docs/zh-CN/terminal-guide#windows)会逐步讲解每个步骤。

安装后，从任何终端启动 `claude`。

* **不使用 Git for Windows**，Claude Code 通过 [PowerShell 工具](https://code.claude.com/docs/zh-CN/tools-reference.md#powershell-tool)运行 shell 命令。
* **使用 Git for Windows**，Claude Code 为 [Bash 工具](https://code.claude.com/docs/zh-CN/tools-reference.md#bash-tool-behavior)使用 Git Bash。如果 Claude Code 找不到 Git Bash，请在您的 [settings.json 文件](https://code.claude.com/docs/zh-CN/settings.md)中设置路径：

  ```json theme={null}
  {
    "env": {
      "CLAUDE_CODE_GIT_BASH_PATH": "C:\\Program Files\\Git\\bin\\bash.exe"
    }
  }
  ```

安装 Git for Windows 后，PowerShell 工具可与 Bash 一起使用：在 claude.ai 和 Console 账户上默认启用，在 Amazon Bedrock、Google Cloud 的 Agent Platform 和 Microsoft Foundry 会话中使用 `CLAUDE_CODE_USE_POWERSHELL_TOOL=1` 启用。将其设置为 `0` 以关闭该工具。有关设置和限制，请参阅 [PowerShell 工具](https://code.claude.com/docs/zh-CN/tools-reference.md#powershell-tool)。

#### 在 WSL 中安装

打开您的 WSL 发行版并按照[安装说明](#install-claude-code)运行 Linux 安装程序。您在 WSL 终端内安装和启动 `claude`，而不是从 PowerShell 或 CMD。

### Alpine Linux 和基于 musl 的发行版

在 Alpine 和其他 musl/uClibc 基础发行版上安装 Claude Code 需要 `bash` 和 `curl` 用于安装命令，以及 `libgcc`、`libstdc++` 和 `ripgrep` 用于运行时。Alpine 默认不包含 `bash` 或 `curl`，因此在您安装它们之前，文档中的安装命令会失败并显示 `not found` 错误。使用您的发行版的包管理器安装这些包，然后设置 `USE_BUILTIN_RIPGREP=0`。

此示例在 Alpine 上安装所需的包：

```bash theme={null}
apk add bash curl libgcc libstdc++ ripgrep
```

在 Alpine 上，`ripgrep` 位于社区存储库中。如果 `apk` 报告该包缺失，请将社区存储库添加到 `/etc/apk/repositories`，使用您的 Alpine 版本：

```bash theme={null}
echo "https://dl-cdn.alpinelinux.org/alpine/v3.22/community" >> /etc/apk/repositories
```

运行 `apk update` 以刷新包索引，然后重试 `apk add` 命令。

然后在您的 [`settings.json`](https://code.claude.com/docs/zh-CN/settings-reference#all-settings) 文件中将 `USE_BUILTIN_RIPGREP` 设置为 `0`：

```json theme={null}
{
  "env": {
    "USE_BUILTIN_RIPGREP": "0"
  }
}
```

## 验证您的安装

安装后，确认 Claude Code 正常工作：

```bash theme={null}
claude --version
```

正常的安装会打印版本号，例如 `2.1.211 (Claude Code)`。

如果此命令失败并显示 `command not found` 或其他错误，请参阅[排查安装和登录问题](https://code.claude.com/docs/zh-CN/troubleshoot-install.md)。

要更详细地检查您的安装和配置，请运行 [`claude doctor`](https://code.claude.com/docs/zh-CN/troubleshooting#get-more-help)：

```bash theme={null}
claude doctor
```

`claude doctor` 打印只读的安装和设置诊断信息，无需启动会话，包括安装健康状况、设置文件验证错误以及任何带有建议修复的警告。

## 身份验证

Claude Code 需要 Pro、Max、Ultra、Team、Enterprise 或 Console 账户。免费的 claude.ai 计划不包括 Claude Code 访问权限。您也可以通过第三方 API 提供商（如 [Amazon Bedrock](https://code.claude.com/docs/zh-CN/amazon-bedrock.md)、[Google Cloud's Agent Platform](https://code.claude.com/docs/zh-CN/google-vertex-ai.md) 或 [Microsoft Foundry](https://code.claude.com/docs/zh-CN/microsoft-foundry.md)）使用 Claude Code。

安装后，通过运行 `claude` 并按照浏览器提示登录。如果设置了 `ANTHROPIC_API_KEY` 环境变量，并且在 Claude Code 询问是否使用该密钥时您批准了它，Claude Code 将跳过登录提示。有关所有账户类型和团队设置选项，请参阅[身份验证](https://code.claude.com/docs/zh-CN/authentication.md)。

## 更新 Claude Code

原生安装会在后台自动更新。您可以[配置发布渠道](#configure-release-channel)来控制您是立即接收更新还是按延迟的稳定计划接收更新，或者[完全禁用自动更新](#disable-auto-updates)。Homebrew、WinGet 和[Linux 包管理器](#install-with-linux-package-managers)安装默认需要手动更新。

### 自动更新

Claude Code 在启动时和运行时定期检查更新。更新在后台下载和安装，然后在您下次启动 Claude Code 时生效。

在您的 shell 中运行 `claude doctor`，以查看 Claude Code 记录的最近一次更新尝试的结果。在原生安装上，如果它报告尝试失败，请参阅[会话页脚中的 `Auto-update failed`](https://code.claude.com/docs/zh-CN/troubleshoot-install#auto-update-failed)，了解后续操作。

在 macOS 和 Linux 上，原生安装程序将启动器作为 `~/.local/bin/claude` 处的符号链接管理到 `~/.local/share/claude/versions/`。如果您将该启动器替换为您自己的脚本或符号链接，自动更新和 `claude update` 会将其保留在原位：新版本仍然安装在 `versions/` 目录下，您的启动器决定运行哪个版本。

使用自定义启动器，Claude Code 也会在磁盘上保留每个已安装的版本，因为它无法判断启动器需要哪个版本。`claude doctor` 报告原生安装程序未创建的启动器。

要让 Claude Code 再次管理启动器，请删除 `~/.local/bin/claude` 并运行 `claude update`。

如果 npm 全局安装因为 npm 全局目录不可写而无法自动更新，Claude Code 会在启动时显示一次性通知，`claude doctor` 会列出可用的修复。有关详细信息，请参阅[安装期间的权限错误](https://code.claude.com/docs/zh-CN/troubleshoot-install.md#permission-errors-during-installation)。

> [!NOTE]
>
> Homebrew、WinGet、apt、dnf 和 apk 安装默认不会自动更新；请参阅下文以选择加入 Homebrew 和 WinGet。要手动升级 Homebrew，请运行 `brew upgrade claude-code` 或 `brew upgrade claude-code@latest`，具体取决于您安装的 cask。对于 WinGet，请运行 `winget upgrade Anthropic.ClaudeCode`。对于 Linux 包管理器，请参阅[使用 Linux 包管理器安装](#install-with-linux-package-managers)中的升级命令。
>
> 要让 Claude Code 在 Homebrew 或 WinGet 上为您运行升级命令，请将 [`CLAUDE_CODE_PACKAGE_MANAGER_AUTO_UPDATE`](https://code.claude.com/docs/zh-CN/env-vars) 设置为 `1`。Claude Code 随后会在新版本可用时在后台运行升级，并在成功时显示重启提示。升级仅针对 Claude Code 包，不会影响您已安装的其他软件。
>
> 在 WinGet 上，当 Claude Code 运行时升级可能会失败，因为 Windows 会锁定可执行文件。在这种情况下，Claude Code 会改为显示手动命令。apt、dnf 和 apk 继续需要手动升级，因为这些命令需要提升的权限。
>
> **已知问题**：Claude Code 可能会在新版本在这些包管理器中可用之前通知您有更新。如果升级失败，请稍候后重试。
>
> Homebrew 在升级后会在磁盘上保留旧版本。定期运行 `brew cleanup` 以回收磁盘空间。

### 配置发布渠道

使用 `autoUpdatesChannel` 设置控制 Claude Code 为自动更新和 `claude update` 遵循的发布渠道：

* `"latest"`，默认值：在新功能发布后立即接收
* `"stable"`：使用通常约一周前的版本，跳过有重大回归的发布

通过 `/config` → **自动更新渠道**配置此项，或将其添加到您的 [settings.json 文件](https://code.claude.com/docs/zh-CN/settings.md)：

```json theme={null}
{
  "autoUpdatesChannel": "stable"
}
```

新发布的模型可能需要比 stable 渠道所提供版本更新的 Claude Code 版本。要立即运行该模型，请切换到 latest 渠道。

对于企业部署，您可以使用[托管设置](https://code.claude.com/docs/zh-CN/managed-settings.md)在整个组织中强制执行一致的发布渠道。

Homebrew 安装通过 cask 名称而不是此设置来选择渠道：`claude-code` 跟踪稳定版，`claude-code@latest` 跟踪最新版。

从 apt、dnf 和 apk 仓库进行的安装通过仓库而不是此设置来选择渠道。要切换渠道，请按照[使用 Linux 包管理器安装](#install-with-linux-package-managers)中的说明操作。

### 固定最低版本

`minimumVersion` 设置建立了一个下限。后台自动更新和 `claude update` 拒绝安装低于此值的任何版本，因此如果您已经在较新的 `"latest"` 构建上，切换到 `"stable"` 渠道不会降级您。

通过 `/config` 从 `"latest"` 切换到 `"stable"` 会提示您选择保留当前版本或允许降级。选择保留会将 `minimumVersion` 设置为该版本。切换回 `"latest"` 会清除它。

将其添加到您的 [settings.json 文件](https://code.claude.com/docs/zh-CN/settings.md)以显式固定下限：

```json theme={null}
{
  "autoUpdatesChannel": "stable",
  "minimumVersion": "2.1.100"
}
```

在[托管设置](https://code.claude.com/docs/zh-CN/managed-settings.md)中，这会强制执行用户和项目设置无法覆盖的组织范围最低版本。

`minimumVersion` 固定仅约束更新。要使 Claude Code 拒绝在版本范围外启动，请改为使用托管设置 `requiredMinimumVersion` 和 `requiredMaximumVersion`。更新也会遵守 `requiredMaximumVersion` 上限。请参阅 [`requiredMinimumVersion`](https://code.claude.com/docs/zh-CN/settings-reference#requiredminimumversion) 和 [`requiredMaximumVersion`](https://code.claude.com/docs/zh-CN/settings-reference#requiredmaximumversion)。

### 禁用自动更新

在您的 [`settings.json`](https://code.claude.com/docs/zh-CN/settings-reference#all-settings) 文件的 `env` 键中将 `DISABLE_AUTOUPDATER` 设置为 `"1"`：

```json theme={null}
{
  "env": {
    "DISABLE_AUTOUPDATER": "1"
  }
}
```

在原生或 npm 安装上，通过运行 `claude doctor` 并检查 `Auto-updates` 行是否显示 `disabled (set by env: DISABLE_AUTOUPDATER)` 而不是 `enabled` 来确认更改已生效。

`DISABLE_AUTOUPDATER` 仅停止后台检查；`claude update` 和 `claude install` 仍然有效。要阻止所有更新路径（包括手动更新），请改为设置 [`DISABLE_UPDATES`](https://code.claude.com/docs/zh-CN/env-vars)。当您通过自己的渠道分发 Claude Code 并需要用户保持在您提供的版本上时，请使用此选项。

### 手动更新

要立即应用更新而不等待下一次后台检查，请运行：

```bash theme={null}
claude update
```

当更新安装时，该命令报告 `Successfully updated from <old version> to version <new version>`。如果您已经在最新版本上，它报告 `Claude Code is up to date (<version>)`。由 Homebrew、WinGet 或 apk 管理的安装改为报告 `Claude is up to date!`。

## 高级安装选项

这些选项用于版本固定、Linux 包管理器、npm、网络存储和验证二进制完整性。

### 安装特定版本

原生安装程序接受特定版本号或发布渠道（`latest` 或 `stable`）。您在安装时选择的渠道将成为自动更新的默认值。有关更多信息，请参阅[配置发布渠道](#configure-release-channel)。

要安装最新版本（默认）：

> [tab] macOS、Linux、WSL
>
> ```bash theme={null}
> curl -fsSL https://claude.ai/install.sh | bash
> ```

> [tab] Windows PowerShell
>
> ```powershell theme={null}
> irm https://claude.ai/install.ps1 | iex
> ```

> [tab] Windows CMD
>
> ```batch theme={null}
> curl -fsSL https://claude.ai/install.cmd -o install.cmd && install.cmd && del install.cmd
> ```

要安装稳定版本：

> [tab] macOS、Linux、WSL
>
> ```bash theme={null}
> curl -fsSL https://claude.ai/install.sh | bash -s stable
> ```

> [tab] Windows PowerShell
>
> ```powershell theme={null}
> & ([scriptblock]::Create((irm https://claude.ai/install.ps1))) stable
> ```

> [tab] Windows CMD
>
> ```batch theme={null}
> curl -fsSL https://claude.ai/install.cmd -o install.cmd && install.cmd stable && del install.cmd
> ```

要安装特定版本号：

> [tab] macOS、Linux、WSL
>
> ```bash theme={null}
> curl -fsSL https://claude.ai/install.sh | bash -s 2.1.89
> ```

> [tab] Windows PowerShell
>
> ```powershell theme={null}
> & ([scriptblock]::Create((irm https://claude.ai/install.ps1))) 2.1.89
> ```

> [tab] Windows CMD
>
> ```batch theme={null}
> curl -fsSL https://claude.ai/install.cmd -o install.cmd && install.cmd 2.1.89 && del install.cmd
> ```

要确认安装了哪个版本，请运行 `claude --version`：该命令打印您传递的确切版本，例如 `2.1.89 (Claude Code)`。

### 使用 Linux 包管理器安装

Claude Code 发布已签名的 apt、dnf 和 apk 存储库。每个存储库提供两个渠道：`stable` 提供通常约一周前的版本，跳过有重大回归的发布，`latest` 在每个发布发布时立即提供。以下命令配置 `stable` 渠道，适合大多数用户；每个选项卡还显示 `latest` 存储库 URL。包管理器安装不会通过 Claude Code 自动更新；更新通过您的正常系统升级工作流程进行。

所有存储库都使用 [Claude Code 发布签名密钥](#binary-integrity-and-code-signing)进行签名。在信任密钥之前，请按照每个选项卡中的说明验证它。

> [tab] apt
>
> 适用于 Debian 和 Ubuntu。以下安装命令使用 `curl` 下载签名密钥，并使用 `gpg` 验证它，新安装的 Debian 和 Ubuntu 可能不包含这两个命令。如果任一命令报告 `command not found`，请先安装两者：
>
> ```bash theme={null}
> sudo apt install curl gnupg
> ```
>
> 下载签名密钥：
>
> ```bash theme={null}
> sudo install -d -m 0755 /etc/apt/keyrings
> sudo curl -fsSL https://downloads.claude.ai/keys/claude-code.asc \
>   -o /etc/apt/keyrings/claude-code.asc
> ```
>
> 如果此下载失败，`apt update` 稍后会失败并显示 `NO_PUBKEY BAA929FF1A7ECACE`。在继续之前，确认密钥已下载并属于 Anthropic：
>
> ```bash theme={null}
> gpg --show-keys /etc/apt/keyrings/claude-code.asc
> ```
>
> gpg 打印的指纹应该是 `31DDDE24DDFAB679F42D7BD2BAA929FF1A7ECACE`。如果 gpg 报告文件无法打开或不包含有效的 OpenPGP 数据，则下载失败或返回了错误的内容：确认您的网络可以到达 `downloads.claude.ai`，然后重新运行下载命令。
>
> 在 `stable` 渠道上注册存储库并安装：
>
> ```bash theme={null}
> echo "deb [signed-by=/etc/apt/keyrings/claude-code.asc] https://downloads.claude.ai/claude-code/apt/stable stable main" \
>   | sudo tee /etc/apt/sources.list.d/claude-code.list
> sudo apt update
> sudo apt install claude-code
> ```
>
> 要改用 `latest` 渠道，URL 路径和套件名称都会改变。使用此 `deb` 行：
>
> ```bash theme={null}
> echo "deb [signed-by=/etc/apt/keyrings/claude-code.asc] https://downloads.claude.ai/claude-code/apt/latest latest main" \
>   | sudo tee /etc/apt/sources.list.d/claude-code.list
> ```
>
> 要稍后升级，请运行 `sudo apt update && sudo apt upgrade claude-code`。

> [tab] dnf
>
> 适用于 Fedora 和 RHEL。以下命令配置 `stable` 渠道：
>
> ```bash theme={null}
> sudo tee /etc/yum.repos.d/claude-code.repo <<'EOF'
> [claude-code]
> name=Claude Code
> baseurl=https://downloads.claude.ai/claude-code/rpm/stable
> enabled=1
> gpgcheck=1
> gpgkey=https://downloads.claude.ai/keys/claude-code.asc
> EOF
> sudo dnf install claude-code
> ```
>
> 要改用 `latest` 渠道，将 `baseurl` 设置为 `latest` 存储库：
>
> ```ini theme={null}
> baseurl=https://downloads.claude.ai/claude-code/rpm/latest
> ```
>
> dnf 在首次安装时下载密钥并提示您确认指纹。在接受之前验证它与 `31DD DE24 DDFA B679 F42D 7BD2 BAA9 29FF 1A7E CACE` 匹配。
>
> 要稍后升级，请运行 `sudo dnf upgrade claude-code`。

> [tab] apk
>
> 适用于 Alpine Linux。以下命令配置 `stable` 渠道：
>
> ```sh theme={null}
> wget -O /etc/apk/keys/claude-code.rsa.pub \
>   https://downloads.claude.ai/keys/claude-code.rsa.pub
> echo "https://downloads.claude.ai/claude-code/apk/stable" >> /etc/apk/repositories
> apk add claude-code
> ```
>
> 要切换到 `latest` 渠道，删除 `stable` 存储库行并添加 `latest` 存储库：
>
> ```sh theme={null}
> sed -i '\|downloads.claude.ai/claude-code/apk/stable|d' /etc/apk/repositories
> echo "https://downloads.claude.ai/claude-code/apk/latest" >> /etc/apk/repositories
> ```
>
> 使用 `sha256sum /etc/apk/keys/claude-code.rsa.pub` 验证下载的密钥，应该报告 `395759c1f7449ef4cdef305a42e820f3c766d6090d142634ebdb049f113168b6`。
>
> 要稍后升级，请运行 `apk update && apk upgrade claude-code`。

### 使用 npm 安装

> [!NOTE]
>
> 如果在您的 shell 中 `npm --version` 打印的版本以 `12` 开头，请改为按照[修复 npm 12 安装](https://code.claude.com/docs/zh-CN/troubleshoot-install.md#fix-an-npm-12-install)进行操作。不要使用 `sudo` 安装。Claude Code 在自我更新时运行 npm 不使用 `sudo`，因此需要 `sudo` 的安装无法自动更新。

您也可以将 Claude Code 安装为全局 npm 包。npm 包需要 [Node.js 22 或更高版本](https://nodejs.org/en/download)。在较旧的 Node.js 版本上，npm 在安装期间打印 `EBADENGINE` 警告而不是失败；安装完成，`claude` 仍然运行，因为该包下载了在运行时不使用您的 Node.js 的原生二进制文件。

```bash theme={null}
npm install -g @anthropic-ai/claude-code
```

npm 包安装与独立安装程序相同的原生二进制文件。npm 通过每个平台的可选依赖项（如 `@anthropic-ai/claude-code-darwin-arm64`）拉取二进制文件，并通过 postinstall 步骤将其链接到位。已安装的 `claude` 二进制文件本身不调用 Node。

支持的 npm 安装平台是 `darwin-arm64`、`darwin-x64`、`linux-x64`、`linux-arm64`、`linux-x64-musl`、`linux-arm64-musl`、`win32-x64` 和 `win32-arm64`。您的包管理器必须允许可选依赖项。如果安装后二进制文件丢失，请参阅[故障排除](https://code.claude.com/docs/zh-CN/troubleshoot-install.md#native-binary-not-found-after-npm-install)。

要升级 npm 安装，请运行 `npm install -g @anthropic-ai/claude-code@latest`。避免使用 `npm update -g`，因为它遵循原始安装的 semver 范围，可能不会将您移动到最新版本。

> [!WARNING]
>
> 不要使用 `sudo npm install -g`，因为这可能导致权限问题和安全风险。如果遇到权限错误，请参阅[故障排除权限错误](https://code.claude.com/docs/zh-CN/troubleshoot-install.md#permission-errors-during-installation)。

### 在网络存储上安装

正在运行的会话在工作过程中会从磁盘读取 Claude Code 可执行文件的部分内容，而不仅仅是在启动时读取。如果该文件在会话中途变得不可读，例如因为它在网络存储上被截断或删除，会话就会崩溃。在 Linux 上，您的 shell 会将此报告为 `Bus error`。

当主目录位于网络存储上时（例如挂载在多台机器上的 NFS 主目录），请合理规划安装布局，使每个会话的可执行文件在会话结束前始终保持可读：

* **安装在本地磁盘上**：将二进制文件放在每台机器的本地文件系统上，例如使用 [Linux 包管理器](#install-with-linux-package-managers)或您自己的部署工具。按用户设置的 npm 前缀和原生安装程序的默认 `~/.local/share/claude/versions/` 目录都位于主目录中。
* **将每个版本保存在各自的目录中**：使用 `npm install -g` 原地升级 npm 安装会删除之前的二进制文件。在多台机器共享的存储上，这会删除其他机器上的会话仍在运行的文件。请将每个新版本安装在旧版本旁边，然后将用户迁移到新版本。
* **仅在没有任何机器可能仍在运行旧版本时才删除它**：一台机器无法看到其他机器上运行的进程，因此在删除之前检查正在运行的进程是不够的。
* **关闭 Claude Code 自身的更新**：设置 [`DISABLE_UPDATES`](https://code.claude.com/docs/zh-CN/env-vars)，并使用您自己的工具安装新版本。否则，一台机器上 npm 安装的自动更新会执行相同的原地升级，从而删除其他机器上的会话正在运行的二进制文件。仅设置 `DISABLE_AUTOUPDATER` 是不够的，因为用户仍然可以运行 `claude update` 和 `claude install`。请参阅[禁用自动更新](#disable-auto-updates)。

原生安装程序会自行从 `~/.local/share/claude/versions/` 中删除旧版本，当该目录位于共享存储上时，这一点尤为重要。除了启动器指向的版本以及同一台机器上的会话正在运行的任何版本之外，它会保留最新的两个版本并删除其余版本。另一台机器上正在运行已删除版本的会话将失去其二进制文件。使用[自定义启动器](#auto-updates)时，Claude Code 会保留所有已安装的版本，并由您自行负责清理。

### 二进制完整性和代码签名

每个发布都发布一个 `manifest.json`，其中包含每个平台二进制文件的 SHA256 校验和。清单使用 Anthropic GPG 密钥签名，因此验证清单上的签名可以传递地验证它列出的每个二进制文件。

#### 验证清单签名

步骤 1-3 需要带有 `gpg` 和 `curl` 的 POSIX shell。在 Windows 上，在 Git Bash 或 WSL 中运行它们。步骤 4 包括 PowerShell 选项。

**1. 下载并导入公钥**

发布签名密钥发布在固定 URL。

```bash theme={null}
curl -fsSL https://downloads.claude.ai/keys/claude-code.asc | gpg --import
```

显示导入的密钥的指纹。

```bash theme={null}
gpg --fingerprint security@anthropic.com
```

确认输出包含此指纹：

```text theme={null}
31DD DE24 DDFA B679 F42D  7BD2 BAA9 29FF 1A7E CACE
```

**2. 下载清单和签名**

将 `VERSION` 设置为您要验证的发布。

```bash theme={null}
REPO=https://downloads.claude.ai/claude-code-releases
VERSION=2.1.89
curl -fsSLO "$REPO/$VERSION/manifest.json"
curl -fsSLO "$REPO/$VERSION/manifest.json.sig"
```

**3. 验证签名**

针对清单验证分离的签名。

```bash theme={null}
gpg --verify manifest.json.sig manifest.json
```

有效的结果报告 `Good signature from "Anthropic Claude Code Release Signing <security@anthropic.com>"`。

`gpg` 也会为任何新导入的密钥打印 `WARNING: This key is not certified with a trusted signature!`。这是预期的。`Good signature` 行确认密码学检查通过。第 1 步中的指纹比较确认密钥本身是真实的。

**4. 根据清单检查二进制文件**

将二进制文件的 SHA256 校验和与 `manifest.json` 中 `platforms.<platform>.checksum` 下列出的值进行比较。以下命令假设当前目录中有 `claude` 二进制文件。要验证已安装的原生二进制文件，请针对 `~/.local/share/claude/versions/VERSION` 运行命令，将 VERSION 替换为您在第 2 步中设置的发布。

> [tab] Linux
>
> ```bash theme={null}
> sha256sum claude
> ```

> [tab] macOS
>
> ```bash theme={null}
> shasum -a 256 claude
> ```

> [tab] Windows PowerShell
>
> ```powershell theme={null}
> (Get-FileHash claude.exe -Algorithm SHA256).Hash.ToLower()
> ```

> [!NOTE]
>
> 清单签名可用于 `2.1.89` 及以后的发布。较早的发布在 `manifest.json` 中发布校验和，但没有分离的签名。

#### 平台代码签名

除了签名的清单外，各个二进制文件在支持的地方还带有平台原生代码签名。

* **macOS**：由"Anthropic PBC"签名并由 Apple 公证。使用 `codesign --verify --verbose ./claude` 验证。
* **Windows**：由"Anthropic, PBC"签名。使用 `Get-AuthenticodeSignature .\claude.exe` 验证。
* **Linux**：二进制文件不单独进行代码签名。如果您直接从 `claude-code-releases` 存储桶下载或使用原生安装程序，请使用上面的清单签名验证完整性。如果您使用 [apt、dnf 或 apk](#install-with-linux-package-managers) 安装，您的包管理器会使用存储库签名密钥自动验证签名。

## 卸载 Claude Code

要删除 Claude Code，请按照您的安装方法的说明进行操作。如果之后 `claude` 仍然运行，您可能有第二个安装或来自较旧安装程序的遗留 shell 别名。请参阅[检查冲突的安装](https://code.claude.com/docs/zh-CN/troubleshoot-install.md#check-for-conflicting-installations)以查找并删除它。

### 卸载原生安装

删除 Claude Code 二进制文件和版本文件：

> [tab] macOS、Linux、WSL
>
> ```bash theme={null}
> rm -f ~/.local/bin/claude
> rm -rf ~/.local/share/claude
> ```

> [tab] Windows PowerShell
>
> ```powershell theme={null}
> Remove-Item -Path "$env:USERPROFILE\.local\bin\claude.exe" -Force
> Remove-Item -Path "$env:USERPROFILE\.local\share\claude" -Recurse -Force
> ```

### 使用 Homebrew 卸载

删除您安装的 Homebrew cask。如果您安装了稳定版 cask：

```bash theme={null}
brew uninstall --cask claude-code
```

如果您安装了最新版 cask：

```bash theme={null}
brew uninstall --cask claude-code@latest
```

### 使用 WinGet 卸载

删除 WinGet 包：

```powershell theme={null}
winget uninstall Anthropic.ClaudeCode
```

### 使用 apt、dnf 或 apk 卸载

删除包和存储库配置：

> [tab] apt
>
> ```bash theme={null}
> sudo apt remove claude-code
> sudo rm /etc/apt/sources.list.d/claude-code.list /etc/apt/keyrings/claude-code.asc
> ```

> [tab] dnf
>
> ```bash theme={null}
> sudo dnf remove claude-code
> sudo rm /etc/yum.repos.d/claude-code.repo
> ```

> [tab] apk
>
> ```sh theme={null}
> apk del claude-code
> sed -i '\|downloads.claude.ai/claude-code/apk|d' /etc/apk/repositories
> rm /etc/apk/keys/claude-code.rsa.pub
> ```

### 使用 npm 卸载

删除全局 npm 包：

```bash theme={null}
npm uninstall -g @anthropic-ai/claude-code
```

### 删除配置文件

> [!WARNING]
>
> 删除配置文件将删除您的所有设置、允许的工具、MCP 服务器配置和会话历史记录。

VS Code 扩展、JetBrains 插件和桌面应用也会写入 `~/.claude/`。如果其中任何一个仍然安装，下次运行时目录会被重新创建。要完全删除 Claude Code，请在删除这些文件之前卸载 [VS Code 扩展](https://code.claude.com/docs/zh-CN/vs-code.md#uninstall-the-extension)、JetBrains 插件和桌面应用。

要删除 Claude Code 设置和缓存数据：

> [tab] macOS、Linux、WSL
>
> ```bash theme={null}
> # 删除用户设置和状态
> rm -rf ~/.claude
> rm ~/.claude.json
>
> # 删除特定于项目的设置（从您的项目目录运行）
> rm -rf .claude
> rm -f .mcp.json
> ```

> [tab] Windows PowerShell
>
> ```powershell theme={null}
> # 删除用户设置和状态
> Remove-Item -Path "$env:USERPROFILE\.claude" -Recurse -Force
> Remove-Item -Path "$env:USERPROFILE\.claude.json" -Force
>
> # 删除特定于项目的设置（从您的项目目录运行）
> Remove-Item -Path ".claude" -Recurse -Force
> Remove-Item -Path ".mcp.json" -Force
> ```
