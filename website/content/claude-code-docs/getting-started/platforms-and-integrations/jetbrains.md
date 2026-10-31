> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# JetBrains IDEs

> 在 JetBrains IDE（包括 IntelliJ、PyCharm、WebStorm 等）中使用 Claude Code

Claude Code 通过专用插件与 JetBrains IDE 集成，提供交互式差异查看、选择上下文共享等功能。

## 支持的 IDE

Claude Code 插件适用于大多数 JetBrains IDE，包括：

* IntelliJ IDEA
* PyCharm
* Android Studio
* WebStorm
* PhpStorm
* GoLand

## 功能

* **快速启动**：使用 `Cmd+Esc`（Mac）或 `Ctrl+Esc`（Windows/Linux）直接从编辑器打开 Claude Code，或点击 UI 中的 Claude Code 按钮
* **差异查看**：Claude Code 在 IDE 差异查看器中打开代码更改，而不是在终端中；使用 `/config` 中的 **Diff tool** 设置更改此行为
* **选择上下文**：IDE 中的当前选择或标签页会自动与 Claude Code 共享。[`Read` 拒绝规则](https://code.claude.com/docs/zh-CN/permissions#read-and-edit)会阻止对匹配文件的此共享
* **文件引用快捷方式**：使用 `Cmd+Option+K`（Mac）或 `Alt+Ctrl+K`（Linux/Windows）插入文件引用，例如 `@src/auth.ts#L1-99`
* **诊断共享**：Claude 通过调用 [`getDiagnostics` 工具](#the-built-in-ide-mcp-server)读取 IDE 的检查诊断，例如 lint 和语法错误；Claude Code 在编辑后不会自行从插件请求诊断

## 安装

该插件在您的 IDE 集成终端中运行 `claude` 命令并连接到它。它不包含自己的 CLI 副本，因此您需要安装两个部分：

**1. 安装 Claude Code CLI**

如果您还没有安装 CLI，请按照[快速入门](https://code.claude.com/docs/zh-CN/quickstart.md)进行安装。当 `claude` 不在您的 PATH 中时，插件会显示"无法启动 Claude Code"通知。

**2. 安装 JetBrains 插件**

从 JetBrains Marketplace 安装 [Claude Code 插件](https://plugins.jetbrains.com/plugin/27310-claude-code-beta-)，然后重启您的 IDE。

如果 `claude` 安装在您的 IDE 找不到的位置，请在插件的 [Claude 命令设置](#general-settings)中设置完整路径。

Claude Code 适用于任何付费 Claude 订阅（Pro、Max、Ultra、Team 或 Enterprise）或 Claude Console 账户，无需 API 密钥。首次运行 `claude` 时，系统会提示您[登录](https://code.claude.com/docs/zh-CN/authentication.md#log-in-to-claude-code)。

## 使用

### 从您的 IDE 运行 Claude Code

从 IDE 的集成终端运行 `claude`，所有集成功能都将处于活跃状态。

### 从外部终端连接

在任何外部终端中使用 `/ide` 命令将 Claude Code 连接到您的 JetBrains IDE 并激活所有功能：

```bash theme={null}
claude
```

```text theme={null}
/ide
```

当连接成功时，Claude Code 会确认一条消息，例如 `Connected to IntelliJ IDEA.`

如果您希望 Claude 能够访问与 IDE 相同的文件，请从与 IDE 项目根目录相同的目录启动 Claude Code。

## 配置

### Claude Code 设置

通过 Claude Code 的设置配置 IDE 集成：

1. 运行 `claude`
2. 输入 `/config` 命令
3. 将**差异工具**设置为 `auto` 以在 IDE 中显示差异，或设置为 `terminal` 以在终端中保留它们

**差异工具**条目仅在 Claude Code 连接到 IDE 时才会出现在 `/config` 中，因此请从 JetBrains 终端运行 `claude`，或先从外部终端运行 [`/ide`](https://code.claude.com/docs/zh-CN/commands)。有关底层设置，请参阅 [`diffTool`](https://code.claude.com/docs/zh-CN/settings-reference#difftool)。

### 插件设置

通过转到 **Settings → Tools → Claude Code \[Beta]** 配置 Claude Code 插件：

#### 常规设置

* **Claude 命令**：指定自定义命令来运行 Claude，例如 `claude`、`/usr/local/bin/claude` 或 `npx @anthropic-ai/claude-code`
* **抑制 Claude 命令未找到的通知**：跳过有关找不到 Claude 命令的通知
* **启用使用 Option+Enter 进行多行提示**：仅在 macOS 上。启用后，Option+Enter 在 Claude Code 提示中插入新行。如果 Option 键被意外捕获，请禁用。需要终端重启。
* **启用自动更新**：自动检查并安装插件更新，在重启时应用

> [!TIP]
>
> 对于 WSL 用户：将 `wsl -d Ubuntu -- bash -lic "claude"` 设置为您的 Claude 命令（将 `Ubuntu` 替换为您的 WSL 发行版名称）

#### ESC 键配置

如果 ESC 键在 JetBrains 终端中无法中断 Claude Code 操作：

1. 转到 **Settings → Tools → Terminal**
2. 执行以下任一操作：
   * 取消选中"使用 Escape 将焦点移动到编辑器"，或
   * 点击"配置终端快捷键"并删除"切换焦点到编辑器"快捷方式
3. 应用更改

这将允许 ESC 键正确中断 Claude Code 操作。

## 特殊配置

### 远程开发

> [!WARNING]
>
> 使用 JetBrains 远程开发时，您必须通过 **Settings → Plugin (Host)** 在远程主机上安装插件，而不是在您的本地客户端计算机上。

### WSL 配置

如果您在 WSL2 上使用 Claude Code 和 JetBrains IDE，并看到"未检测到可用的 IDE"，原因通常是 WSL2 的 NAT 网络或 Windows 防火墙阻止了 WSL2 和在 Windows 主机上运行的 IDE 之间的连接。WSL1 直接使用主机的网络，不受影响。

#### 允许 WSL2 流量通过 Windows 防火墙

这是推荐的修复方法，因为它保持您现有的 WSL2 网络模式。

**1. 查找您的 WSL2 IP 地址**

从您的 WSL shell 内部运行：

```bash theme={null}
hostname -I
```

记下您的子网：取地址的前两个段，然后跟上 `.0.0/16`。例如，如果地址是 `172.21.123.45`，您的子网是 `172.21.0.0/16`。

**2. 创建防火墙规则**

以管理员身份打开 PowerShell 并运行以下命令，调整 IP 范围以匹配您的子网：

```powershell theme={null}
New-NetFirewallRule -DisplayName "Allow WSL2 Internal Traffic" -Direction Inbound -Protocol TCP -Action Allow -RemoteAddress 172.21.0.0/16 -LocalAddress 172.21.0.0/16
```

**3. 重启您的 IDE 和 Claude Code**

关闭并重新打开两者，以使新规则生效。

#### 将 WSL2 切换到镜像网络

镜像网络需要 Windows 11 22H2 或更高版本。如果您使用 Windows 10，请改用上面的防火墙规则。

将以下内容添加到 Windows 用户目录中的 `.wslconfig`：

```ini theme={null}
[wsl2]
networkingMode=mirrored
```

然后从 PowerShell 使用 `wsl --shutdown` 重启 WSL。

## 故障排除

### 插件不工作

如果插件已安装但 Claude Code 功能未出现在您的 IDE 中：

* 确保您从项目根目录运行 Claude Code
* 检查 JetBrains 插件在 IDE 设置中是否已启用
* 完全重启 IDE（您可能需要多次执行此操作）
* 对于远程开发，确保插件已安装在远程主机上

### IDE 未检测到

如果 `/ide` 命令显示"未检测到可用的 IDE"：

* 验证插件已安装并启用
* 完全重启 IDE
* 如果您期望在不运行 `/ide` 的情况下自动连接，请检查您是否从 IDE 的集成终端启动了 `claude`
* 对于 WSL 用户，请参阅上面的 [WSL 配置](#wsl-configuration)

### 命令未找到

如果点击 Claude 图标显示"命令未找到"：

1. 通过在终端中运行 `claude --version` 验证 Claude Code 已安装
2. 在插件设置中配置 Claude 命令路径
3. 对于 WSL 用户，使用配置部分中提到的 WSL 命令格式

## 安全考虑

当 Claude Code 在 JetBrains IDE 中以 [`acceptEdits` 权限模式](https://code.claude.com/docs/zh-CN/permission-modes#auto-approve-file-edits-with-acceptedits-mode)运行时，它可能能够修改 IDE 配置文件，这些文件可以由您的 IDE 自动执行。这可能会增加在 `acceptEdits` 模式下运行 Claude Code 的风险，并允许绕过 Claude Code 对 Bash 执行的权限提示。

在 JetBrains IDE 中运行时，请考虑：

* 对编辑使用手动模式，因为 `acceptEdits` 和自动模式都会批准您工作目录内的编辑而不询问，除了在[受保护路径](https://code.claude.com/docs/zh-CN/permission-modes.md#protected-paths)中会询问
* 特别小心确保 Claude 仅与受信任的提示一起使用
* 了解 Claude Code 有权修改哪些文件

对于 IDE 外的 Claude Code 安装或登录问题，请参阅[排查安装和登录问题](https://code.claude.com/docs/zh-CN/troubleshoot-install.md)。

### 内置 IDE MCP 服务器

当插件处于活动状态时，它运行一个本地 MCP 服务器，CLI 会自动连接到该服务器。这是 CLI 在 IDE 的原生 diff 查看器中打开 diff、读取您当前的 `@`-提及选择以及让 Claude 读取检查诊断的方式。

服务器名为 `ide`，从 `/mcp` 中隐藏，因为没有什么需要配置。但是，如果您的组织使用 [`PreToolUse` hook](https://code.claude.com/docs/zh-CN/hooks#pretooluse) 来允许列表 MCP 工具，您需要知道它的存在。

**选择和打开文件上下文。** 连接时，CLI 会在您发送的每个提示中包含您当前的编辑器选择和活动文件的路径作为上下文。当发生这种情况时，记录会显示一行 `⧉ Selected N lines from <file>`。如果您[在 Claude 工作时排队消息](https://code.claude.com/docs/zh-CN/interactive-mode.md#queue-messages-while-claude-works)，它会保留您按下 `Enter` 时的选择，无论您之后选择什么。

要排除敏感文件（如 `.env`），请为其路径添加 [`Read` 拒绝规则](https://code.claude.com/docs/zh-CN/permissions#read-and-edit)。匹配的拒绝规则可防止该文件的选定文本和打开文件通知都到达 Claude。

**传输和身份验证。** 服务器侦听 OS 分配的临时端口，该端口不可配置。传输是未加密的 `ws://`；在环回上，任何可以捕获流量的进程也可以从锁文件中读取令牌，因此 TLS 不会对本地攻击者增加保护。每次 IDE 启动都会生成一个新的随机身份验证令牌，将其写入 `~/.claude/ide/<port>.lock` 处的锁文件，CLI 必须将其作为 `X-Claude-Code-Ide-Authorization` 标头呈现才能连接。如果设置了 `CLAUDE_CONFIG_DIR`，锁文件将改为写入 `$CLAUDE_CONFIG_DIR/ide/`。

**向模型公开的工具。** 服务器托管多个工具，但只有一个对模型可见。其余的是 CLI 用于自己的 UI 的内部 RPC，例如打开 diff 和读取选择，在工具列表到达 Claude 之前会被过滤掉。

| 工具名称（如 hooks 所见） | 它的作用 | 只读 |
| - | - | - |
| `mcp__ide__getDiagnostics` | 返回 IDE 的检查诊断，即编辑器中显示的错误和警告。每次调用涵盖一个文件：Claude 指定的文件，或如果 Claude 未指定文件则为您的活动编辑器中的文件。 | 是 |

JetBrains 插件不向模型公开代码执行工具。

**侦听接口。** 服务器绑定到哪个网络接口由**设置 → 工具 → Claude Code \[Beta] → 网络（高级）**下的**接受来自所有网络接口的连接**控制。禁用该设置时，服务器仅侦听 `127.0.0.1`，无法从其他主机访问。启用该设置时，该端口可从您的本地网络访问。该设置存在于 CLI 无法通过环回到达 IDE 的情况，例如具有默认 NAT 网络的 WSL2 或远程 IDE 设置；有关该场景，请参阅 [WSL 配置](#wsl-configuration)。

> [!WARNING]
>
> 启用**接受来自所有网络接口的连接**会使 IDE MCP 端口可从您的本地网络访问。连接仍然需要来自锁文件的身份验证令牌，但由于传输是未加密的 `ws://`，当设置打开时，会话流量和该令牌都以明文形式跨网络传输。仅在环回无法工作时才打开它。对于 WSL2，更倾向于[镜像网络](#switch-wsl2-to-mirrored-networking)，以便 Windows 环回接口与 Linux VM 共享，套接字可以保持在环回上。
