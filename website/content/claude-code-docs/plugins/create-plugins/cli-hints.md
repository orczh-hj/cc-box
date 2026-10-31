> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# 从您的 CLI 推荐您的插件

> 通过从您的 CLI 或 SDK 发出 claude-code-hint 标签，提示 Claude Code 用户安装您的官方市场插件。

如果您维护 CLI 或 SDK，您的工具可以提示 Claude Code 用户安装您的插件。当您的 CLI 检测到它在 Claude Code 内运行时，让它向 stderr 写入一行 `<claude-code-hint />` 标签。Claude Code 在模型看到输出之前从 Bash 和 PowerShell 工具输出中删除该行，然后向用户显示一次性安装提示。

本页仅适用于您的插件是否列在 `claude-plugins-official` 或其他市场中，该市场具有 Anthropic 的[官方市场名称](https://code.claude.com/docs/zh-CN/plugins/security.md#official-marketplace-names)之一。社区市场 `claude-community` 不是其中之一。

> [!NOTE]
>
> 要发布插件，请参阅[发布和分发插件](https://code.claude.com/docs/zh-CN/plugins/publish.md)。

## 发出提示

仅当设置了 `CLAUDECODE` 或 `CLAUDE_CODE_CHILD_SESSION` 时才发出标签，以便当人员直接运行您的 CLI 时它不会出现。

Claude Code 在通过 Bash 和 PowerShell 工具运行的命令以及 hook 命令中设置 `CLAUDECODE=1`。在 v2.1.172 及更高版本上，它也在那里设置 `CLAUDE_CODE_CHILD_SESSION=1`。这些变量在哪些进程中携带它们方面有所不同：

* **`CLAUDECODE`**：由每个 Claude Code 版本设置。IDE 扩展也在其集成终端中设置它，因此仅在 `CLAUDECODE` 上的门控也会在人员在其中一个终端中直接运行您的 CLI 时发出标签
* **`CLAUDE_CODE_CHILD_SESSION`**：仅在 Claude Code 本身启动的子进程中设置。当您可以要求 v2.1.172 或更高版本时使用它

[环境变量参考](https://code.claude.com/docs/zh-CN/env-vars.md)有详细信息。

以下示例在 `CLAUDECODE` 上进行门控以获得最广泛的覆盖范围，并为官方市场中名为 `example-cli` 的插件发出提示：

```javascript Node.js theme={null}
if (process.env.CLAUDECODE) {
  process.stderr.write(
    '<claude-code-hint v="1" type="plugin" value="example-cli@claude-plugins-official" />\n',
  )
}
```

```python Python theme={null}
import os, sys

if os.environ.get("CLAUDECODE"):
    print(
        '<claude-code-hint v="1" type="plugin" value="example-cli@claude-plugins-official" />',
        file=sys.stderr,
    )
```

```go Go theme={null}
if os.Getenv("CLAUDECODE") != "" {
    fmt.Fprintln(os.Stderr,
        `<claude-code-hint v="1" type="plugin" value="example-cli@claude-plugins-official" />`)
}
```

```shell Shell theme={null}
if [ -n "$CLAUDECODE" ]; then
  printf '%s\n' '<claude-code-hint v="1" type="plugin" value="example-cli@claude-plugins-official" />' >&2
fi
```

将 `example-cli` 替换为您的插件在官方市场中的名称。

您可以在每次调用时发出提示，因为 Claude Code 为每个插件提示一次。

要检查发出器，请在终端中运行 `CLAUDECODE=1 example-cli` 并确认标签行出现在 stderr 上，然后在没有变量的情况下运行 `example-cli` 并确认没有额外的内容打印。

## 提示格式

标签必须占据自己的行；Claude Code 忽略嵌入在行中间的标签。

标签采用三个属性，全部必需：

| 属性 | 描述 |
| :- | :- |
| `v` | 协议版本。`1` 是唯一支持的值 |
| `type` | 提示类型。`plugin` 是唯一支持的值 |
| `value` | `name@marketplace` 形式的插件标识符 |

值可以是双引号或不带引号；不带引号的值不能包含空格。

即使 `v` 或 `type` 无法识别，Claude Code 也会从输出中删除该行。

## 检查提示何时出现

提示仅在交互式终端会话中出现。在 `claude -p` 运行中、在子代理运行中以及在 hook 命令输出中，标签被剥离且不显示提示。所有这些检查也必须通过：

* **官方且可安装**：`value` 命名一个 Claude Code 在其官方市场本地副本中找到的插件，该插件尚未安装，且没有策略阻止
* **分析打开**：Claude Code 的分析关闭的会话永远不会提示，例如设置了 `DISABLE_TELEMETRY`、`DO_NOT_TRACK` 或 `CLAUDE_CODE_DISABLE_NONESSENTIAL_TRAFFIC` 的会话，或在第三方提供商（如 Amazon Bedrock）上的会话，其中[自动遥测选择退出](https://code.claude.com/docs/zh-CN/data-usage.md#default-behaviors-by-api-provider)适用
* **频率限制**：每个会话一个提示，每个插件一个提示（无论用户的答案如何），一旦在该机器上为 100 个插件提示过，就没有提示
* **未关闭**：用户未选择**否，不再显示插件安装提示**
* **本地、有人值守的会话**：会话的工作区是本地的而不是在云或远程机器上，会话不是无人值守运行的。例如，使用 `--cloud` 启动的会话、提供远程控制的会话或代理团队队友永远不会提示

## 预览用户看到的内容

当[检查提示何时出现](#check-when-the-prompt-appears)中的检查通过时，Claude Code 显示一个**插件推荐**对话框，如下所示：

```text theme={null}
─────────────────────────────────────────────────────────────
  Plugin recommendation

    The example-cli command suggests installing a plugin.

    Plugin: example-cli
    Marketplace: claude-plugins-official
    Description: Official integration for example-cli deployments

    Would you like to install it?
    ❯ 1. Yes, install
      2. No
      3. No, and don't show plugin installation hints again

─────────────────────────────────────────────────────────────
```

对话框命名 Claude 运行的 shell 命令的第一个单词，以便用户可以发现不匹配。每个答案都有一个效果：

* **是的，安装**：在[用户范围](https://code.claude.com/docs/zh-CN/plugins/install.md)安装插件
* **否，不再显示插件安装提示**：为该用户关闭未来的提示提示
* **30 秒内无答案**：计为**否**

## 后续步骤

* [发布和分发插件](https://code.claude.com/docs/zh-CN/plugins/publish.md)：进入每个市场的路由，包括提示所需的官方市场
* [插件命令参考](https://code.claude.com/docs/zh-CN/plugins/cli-reference.md#plugin-install)：在会话外安装相同插件的 shell 命令
