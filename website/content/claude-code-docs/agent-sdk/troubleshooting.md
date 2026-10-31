> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Agent SDK 故障排除

> 当 Claude Code CLI 无法启动、CLI 进程退出或成功结果到达但没有结构化输出时，修复 Agent SDK 错误。

本页面涵盖 CLI 启动、CLI 进程退出和结构化输出中的 Agent SDK 错误。本页面上的条目按您看到的错误进行分类。每个条目说明了原因和解决方法。

与特定功能相关的症状，例如 hook 未触发或 skill 未被使用，在该功能的页面上有故障排除部分。该表列出了涵盖每个症状的部分或页面：

| 症状 | 转到 |
| :- | :- |
| 找不到 Skills、skill 未被使用、`Invalid skill name` 错误 | [Skills 故障排除](https://code.claude.com/docs/zh-CN/agent-sdk/skills.md#troubleshooting) |
| MCP 服务器显示 `failed` 状态、工具未被调用、SDK MCP 服务器中缺少某个工具、连接超时、工具输出超过允许的最大 token 数 | [MCP 故障排除](https://code.claude.com/docs/zh-CN/agent-sdk/mcp.md#troubleshooting) |
| Plugin 未加载、plugin skills 未出现 | [Plugins 故障排除](https://code.claude.com/docs/zh-CN/agent-sdk/plugins.md#troubleshooting) |
| Claude 未委派给子代理、基于文件系统的代理未加载 | [Subagents 故障排除](https://code.claude.com/docs/zh-CN/agent-sdk/subagents.md#troubleshooting) |
| Checkpointing 选项未被识别、没有 UUID 的用户消息、`No file checkpoint found`、`File rewinding is not enabled`、`ProcessTransport is not ready for writing` | [文件 checkpointing 故障排除](https://code.claude.com/docs/zh-CN/agent-sdk/file-checkpointing.md#troubleshooting) |
| Hook 未触发、matcher 未按预期过滤、hook 超时、工具被意外阻止、修改的输入未应用、Python 中不可用的会话 hooks、子代理权限提示倍增、与子代理的递归 hook 循环、`systemMessage` 未出现在输出中 | [修复常见问题](https://code.claude.com/docs/zh-CN/agent-sdk/hooks.md#fix-common-issues)（在 hooks 页面上） |
| 在您的机器上工作的代理在已部署的服务或容器中失败 | [故障排除部署失败](https://code.claude.com/docs/zh-CN/agent-sdk/hosting.md#troubleshoot-deployment-failures) |
| `Not logged in`、`Invalid API key`、`API Error`、`429`、`There's an issue with the selected model` | [错误参考](https://code.claude.com/docs/zh-CN/errors.md#find-your-error) |
| `CLINotFoundError`、`CLIConnectionError`、`ProcessError`、`Claude Code process exited with code N`、`Claude Code returned an error result`、`structured_output` 为 `None` | 本页面上的 [CLI 启动](#cli-startup)、[CLI 进程退出](#cli-process-exit) 和 [结构化输出](#structured-outputs) |

## CLI 启动

### CLINotFoundError: Claude Code not found

Python SDK 将 Claude Code CLI 作为子进程启动。当它找不到 `claude` 可执行文件时，连接会失败并显示 `CLINotFoundError`：

```
Claude Code not found at: /your/configured/path
```

当您设置 `ClaudeAgentOptions(cli_path=...)` 并且它指向一个不存在的文件时，消息会包含配置的路径。如果没有 `cli_path`，SDK 会搜索您的 `PATH` 和常见安装位置，消息会包含您平台的安装说明。

要修复它：

* 如果尚未安装 Claude Code，请安装它。请参阅 [安装 Claude Code](https://code.claude.com/docs/zh-CN/setup.md#install-claude-code) 了解您平台上的命令。
* 如果您设置了 `cli_path`，请确认该文件存在且是 `claude` 可执行文件。
* 如果您依赖 `PATH` 解析，请确认 `claude --version` 在您的应用程序运行的同一环境中有效。您从 IDE 或服务管理器等外部启动的进程通常使用不同的 `PATH`。

TypeScript SDK 在其捆绑的平台包和您在 `pathToClaudeCodeExecutable` 中设置的路径中查找 CLI。匹配您看到的消息：

* `Native CLI binary for <platform>-<arch> not found`：捆绑的平台包缺失，最常见的原因是安装跳过了可选依赖项。重新安装 `@anthropic-ai/claude-agent-sdk` 而不跳过可选依赖项，或将 `pathToClaudeCodeExecutable` 指向 [原生安装](https://code.claude.com/docs/zh-CN/setup.md#install-claude-code)。在使用 `bun build --compile` 构建的单文件可执行文件中，同一消息有不同的原因和修复方法。请参阅 [编译为单个可执行文件](https://code.claude.com/docs/zh-CN/agent-sdk/typescript.md#compile-to-a-single-executable)。
* `Claude Code native binary not found at <path>` 或 `Claude Code executable not found at <path>. Is options.pathToClaudeCodeExecutable set?`：已解析路径处的文件缺失，或进程无法访问它。确认该文件存在于该路径处，并且进程可以访问它。

### CLIConnectionError: Refusing to execute batch script

在 Windows 上，当 Python SDK 使用的 CLI 路径是 `.bat` 或 `.cmd` 批处理脚本（包括 npm 安装创建的 `claude.cmd` 垫片）时，连接会失败并显示 `CLIConnectionError`：

```
Refusing to execute batch script 'C:\\Users\\you\\AppData\\Roaming\\npm\\claude.cmd': Windows runs .bat/.cmd files via cmd.exe, which can execute commands injected through CLI arguments, and no reliable escaping for cmd.exe exists. Use a native claude executable instead: install Claude Code natively (irm https://claude.ai/install.ps1 | iex), point ClaudeAgentOptions(cli_path=...) at a claude.exe, or install the claude-agent-sdk wheel for a platform that bundles claude.exe (e.g. Windows x64).
```

这种拒绝是有意的安全加固，而不是破损的安装。Windows 通过将生成重写为 `cmd.exe /c` 调用来运行批处理脚本，而 `cmd.exe` 在执行时重新解析整个命令行，因此参数值可以执行注入的命令。

大多数 Windows 安装永远不会遇到此错误。Windows x64 版本的 `claude-agent-sdk` 捆绑了 `claude.exe`，SDK 优先使用捆绑的 CLI，然后是它可以发现的任何原生 `claude.exe`，最后才回退到批处理垫片。您在两种情况下会看到拒绝：

* 您将 `ClaudeAgentOptions(cli_path=...)` 设置为 `.bat` 或 `.cmd` 文件，例如 npm 的 `claude.cmd` 垫片。
* 您的安装没有捆绑或原生 `claude.exe`，例如 ARM64 Windows 上的源代码安装，其中您的 `PATH` 上唯一的 `claude` 是 npm 垫片。

要修复它，请给 SDK 一个原生可执行文件而不是批处理脚本：

* 如果您设置了 `ClaudeAgentOptions(cli_path=...)`，请将其指向 `claude.exe` 或删除该选项。当设置了 `cli_path` 时，SDK 会跳过发现，因此仅原生安装无法生效。
* 在 PowerShell 中原生安装 Claude Code：`irm https://claude.ai/install.ps1 | iex`
* 在 x64 Windows 上，安装捆绑 `claude.exe` 的 `claude-agent-sdk` wheel。

在 `claude-agent-sdk` 0.2.124 之前，Python SDK 通过 `cmd.exe` 生成批处理脚本而没有此检查。

### CLIConnectionError: Failed to start Claude Code

SDK 在已解析的路径处找到了一个文件，但无法启动它。Python 将这些失败作为 `CLIConnectionError` 引发。TypeScript 拒绝消息迭代并显示没有 SDK 类的错误。下表将每条消息映射到它告诉您的内容。匹配您看到的消息：

| 消息 | SDK | 它告诉您什么 |
| - | - | - |
| `Failed to start Claude Code: <detail>` | Python | 消息的其余部分是操作系统自己的错误 |
| `Claude Code executable at <path> exists but failed to launch` | TypeScript | 配置路径处的脚本无法运行 |
| `Claude Code native binary at <path> exists but failed to launch` | TypeScript | 二进制文件无法运行，消息后附加了 libc 建议 |
| `Failed to spawn Claude Code process: <detail>` | TypeScript | 任何其他启动失败 |

在两个 SDK 中，通常的原因是已解析的路径指向无法运行的内容，例如文本文件、目录或没有执行权限的文件。将原生二进制消息的 libc 建议作为一个可能的原因来阅读。

要在任一 SDK 中修复它：

* 确认配置的路径指向 `claude` 可执行文件本身，并且该文件具有执行权限。
* 如果您不需要自定义路径，请在 Python 中删除 `cli_path` 或在 TypeScript 中删除 `pathToClaudeCodeExecutable`，以便 SDK 自己找到 CLI，优先使用其捆绑的副本。
* 当失败的二进制文件是容器镜像中 SDK 的捆绑副本时，在镜像构建期间重新安装 SDK，以便捆绑的二进制文件与容器的平台匹配，或为其运行的架构重建镜像。通常的原因是与容器架构或 libc 不匹配的二进制文件，或在镜像构建中失去执行权限的二进制文件。

### CLIConnectionError: Not connected

在客户端连接之前或断开连接之后，在 Python 中调用 `ClaudeSDKClient` 方法会引发带有此消息的 `CLIConnectionError`：

```
Not connected. Call connect() first.
```

按照消息所说的做。要么在任何其他客户端方法之前调用 `await client.connect()`，要么使用 `async with ClaudeSDKClient() as client:` 打开客户端，它在进入时连接。

## CLI 进程退出

本部分中的条目意味着 Claude Code 进程在您的应用程序使用它时结束。您看到的错误取决于 SDK 语言以及 CLI 在退出前是否报告了错误结果。

### ProcessError: Command failed with exit code

当 Claude Code 进程以非零代码退出时，Python SDK 会引发 `ProcessError`：

```
Command failed with exit code 1 (exit code: 1)
Error output: Check stderr output for details
```

消息两次说明了退出代码，`Error output` 行是固定文本而不是您进程的错误输出。相同的固定文本填充异常的 `stderr` 属性。异常的 `exit_code` 属性携带代码。要捕获 CLI 实际写入 stderr 的内容，请在 `ClaudeAgentOptions` 中传递 `stderr` 回调并记录它接收的内容。

裸 `ProcessError` 意味着 CLI 退出时没有报告错误结果。当 CLI 确实报告了一个时，SDK 会改为引发 [`ResultError`](https://code.claude.com/docs/zh-CN/agent-sdk/python#resulterror)，在 [Claude Code returned an error result](#claude-code-returned-an-error-result) 中介绍。`ResultError` 是 `ProcessError` 的子类，所以 `except ProcessError` 会捕获两者。要以不同方式处理它们，请先放置 `except ResultError` 子句。

在 `claude-agent-sdk` 0.2.140 之前，Python SDK 将错误结果退出作为普通 `Exception` 而不是 `ResultError` 引发。

### Claude Code process exited with code N

IDE 包装器也会打印此消息，[错误参考](https://code.claude.com/docs/zh-CN/errors.md#claude-code-process-exited-with-code-n) 为 VS Code 和其他启动器介绍了它。此条目介绍了您的 TypeScript SDK 代码接收的内容。SDK 将非零 CLI 退出作为普通 `Error` 显示，该错误拒绝 `query()` 消息上的 `for await` 循环。没有 SDK 错误类可以捕获，所以将循环包装在 `try`/`catch` 中并匹配消息：

```
Claude Code process exited with code 1. stderr: <tail of the CLI's stderr>
```

当 CLI 写入 stderr 时，消息以其尾部结尾。要捕获完整流，请在查询选项中传递 `stderr` 回调。被信号杀死的进程以相同的形式报告 `Claude Code process terminated by signal <name>`。

### Claude Code returned an error result

当 CLI 在退出前报告错误结果时，两个 SDK 都用此消息替换进程退出错误：

```
Claude Code returned an error result: <the CLI's own error report>
```

冒号后的文本是 CLI 对出错原因的报告，所以从那里开始而不是从退出本身开始。Python 将其作为 [`ResultError`](https://code.claude.com/docs/zh-CN/agent-sdk/python#resulterror) 引发，其 `data` 属性携带完整的错误结果。TypeScript 拒绝消息循环并显示携带相同消息形状的普通 `Error`。

## 结构化输出

### structured\_output is None but the result says success

结果消息可以以 `subtype: "success"` 结尾，而在 Python 中 `structured_output` 是 `None` 或在 TypeScript 中是 `undefined`。运行完成，但不存在经过验证的输出。

在应用程序代码中将此结果视为失败。在使用 `structured_output` 之前，检查 `subtype` 是否为 `success` 以及 `structured_output` 是否存在。[错误处理](https://code.claude.com/docs/zh-CN/agent-sdk/structured-outputs.md#error-handling) 部分为两个 SDK 显示了此模式。然后检查以下原因：

* **Claude 拒绝的请求**：要在代码中检测这种情况，请检查结果消息的 `stop_reason` 是否为 `"refusal"`。有关接下来可以尝试的操作，请参阅[使用政策拒绝](https://code.claude.com/docs/zh-CN/errors.md#usage-policy-refusal)。
* **任何输出都无法满足的 schema**：例如，一个 `minLength` 大于其 `maxLength` 的字符串。如果在使用您认为正确的 schema 时 `structured_output` 反复缺失，请查找相互矛盾的约束。如果没有找到，请简化 schema 直到输出通过验证，然后一次重新引入一个约束。

## 报告新问题

如果您的错误未在此处介绍，请检查开放问题或在 SDK 存储库中提交新问题：[claude-agent-sdk-typescript](https://github.com/anthropics/claude-agent-sdk-typescript/issues) 或 [claude-agent-sdk-python](https://github.com/anthropics/claude-agent-sdk-python/issues)。包括完整的错误文本和您的 SDK 版本。
