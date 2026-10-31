> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# 使用 hooks 自动化操作

> 当 Claude Code 编辑文件、完成任务或需要输入时自动运行 shell 命令。格式化代码、发送通知、验证命令并强制执行项目规则。

Hooks 是用户定义的 shell 命令。Claude Code 在其生命周期中的特定点运行它们，这为你提供确定性控制：某些操作总是会发生，而不是依赖 LLM 选择运行它们。使用 hooks 来强制执行项目规则、自动化重复任务，并将 Claude Code 与现有工具集成。

对于需要判断而不是确定性规则的决策，你也可以使用 [基于提示的 hooks](#prompt-based-hooks) 或 [基于代理的 hooks](#agent-based-hooks)，它们使用 Claude 模型来评估条件。

有关扩展 Claude Code 的其他方式，请参阅 [skills](https://code.claude.com/docs/zh-CN/skills.md) 用于为 Claude 提供额外的指令和可执行命令，[subagents](https://code.claude.com/docs/zh-CN/sub-agents.md) 用于在隔离的上下文中运行任务，以及 [plugins](https://code.claude.com/docs/zh-CN/plugins/overview.md) 用于打包要在项目间共享的扩展。

> [!TIP]
>
> 本指南涵盖常见用例和入门方法。有关完整的事件架构、JSON 输入/输出格式和异步 hooks 和 MCP 工具 hooks 等高级功能，请参阅 [Hooks 参考](https://code.claude.com/docs/zh-CN/hooks.md)。

## 设置你的第一个 hook

要创建 hook，请将 `hooks` 块添加到 [设置文件](#configure-hook-location)。本演练创建一个桌面通知 hook，这样每当 Claude 等待你的输入而不是监视终端时，你都会收到警报。

**1. 将 hook 添加到你的设置**

打开 `~/.claude/settings.json` 并添加一个 `Notification` hook。如果文件不存在，请创建它。下面的示例使用 `osascript` 用于 macOS；有关 Linux 和 Windows 命令，请参阅 [在 Claude 需要输入时获得通知](#get-notified-when-claude-needs-input)。

```json theme={null}
{
  "hooks": {
    "Notification": [
      {
        "matcher": "",
        "hooks": [
          {
            "type": "command",
            "command": "osascript -e 'display notification \"Claude Code needs your attention\" with title \"Claude Code\"'"
          }
        ]
      }
    ]
  }
}
```

如果你的设置文件已经有一个 `hooks` 键，请将 `Notification` 作为现有事件键的同级添加，而不是替换整个对象。每个事件名称是单个 `hooks` 对象内的一个键：

```json theme={null}
{
  "hooks": {
    "PostToolUse": [
      {
        "matcher": "Edit|Write",
        "hooks": [{ "type": "command", "command": "jq -r '.tool_input.file_path' | xargs npx prettier --write" }]
      }
    ],
    "Notification": [
      {
        "matcher": "",
        "hooks": [{ "type": "command", "command": "osascript -e 'display notification \"Claude Code needs your attention\" with title \"Claude Code\"'" }]
      }
    ]
  }
}
```

您也可以通过在 CLI 中描述您想要的内容来要求 Claude 为您编写 hook。

**2. 验证配置**

在 Claude Code 输入框中输入 `/hooks` 以打开 hook 浏览器。您的新 hook 会出现在 `Notification` 下的列表中。

**3. 测试 hook**

按 `Esc` 返回 CLI。按 `Shift+Tab` 直到状态栏显示 `⏸ manual mode on`，要求 Claude 做需要权限的事情，然后切换离开终端。您应该会收到桌面通知。

## 可以自动化的内容

hook 让您可以在 Claude Code 生命周期的关键节点运行代码：在编辑后格式化文件、在命令执行前阻止命令、在 Claude 需要输入时发送通知、在会话开始时注入上下文，等等。有关 hook 事件的完整列表，请参阅 [Hooks 参考](https://code.claude.com/docs/zh-CN/hooks.md#hook-lifecycle)。

每个示例都包含一个可直接使用的配置块，您可以将其添加到[设置文件](#configure-hook-location)中。

有关运行独立模型审查并将发现结果反馈回会话的 hook 的生产示例，请参阅 [`security-guidance` 插件如何与 Claude Code 集成](https://code.claude.com/docs/zh-CN/security-guidance#how-the-plugin-integrates-with-claude-code)。

### 在 Claude 需要输入时收到通知

每当 Claude 完成工作并需要您输入时，都会收到桌面通知，这样您就可以切换到其他任务，而无需时刻查看终端。

此 hook 使用 `Notification` 事件，Claude Code 会在 Claude 等待输入或权限时触发该事件。有关确切的触发时机，请参阅[各通知类型的触发时机](https://code.claude.com/docs/zh-CN/hooks.md#notification)。

下面的每个选项卡都使用该平台的原生通知命令。将以下内容添加到 `~/.claude/settings.json`：

> [tab] macOS
>
> ```json theme={null}
> {
>   "hooks": {
>     "Notification": [
>       {
>         "matcher": "",
>         "hooks": [
>           {
>             "type": "command",
>             "command": "osascript -e 'display notification \"Claude Code needs your attention\" with title \"Claude Code\"'"
>           }
>         ]
>       }
>     ]
>   }
> }
> ```
>
> > [accordion] 如果没有出现通知
> >
> > `osascript` 通过内置的“脚本编辑器”应用发送通知。如果“脚本编辑器”没有通知权限，该命令会静默失败，并且 macOS 不会提示您授予权限。
> >
> > 在终端中运行一次以下命令，使“脚本编辑器”出现在通知设置中：
> >
> > ```bash theme={null}
> > osascript -e 'display notification "test"'
> > ```
> >
> > 此时还不会显示任何内容。打开 **System Settings > Notifications**，在列表中找到 **Script Editor**，然后打开 **Allow Notifications**。再次运行该命令，确认测试通知出现。

> [tab] Linux
>
> ```json theme={null}
> {
>   "hooks": {
>     "Notification": [
>       {
>         "matcher": "",
>         "hooks": [
>           {
>             "type": "command",
>             "command": "notify-send 'Claude Code' 'Claude Code needs your attention'"
>           }
>         ]
>       }
>     ]
>   }
> }
> ```
>
> > [accordion] 如果没有出现通知
> >
> > `notify-send` 需要桌面通知守护进程，而无头服务器、SSH 会话和大多数容器都没有该进程。请先直接测试该命令：
> >
> > ```bash theme={null}
> > notify-send 'Claude Code' 'test'
> > ```
> >
> > 如果找不到该命令，请在 Debian 和 Ubuntu 上安装 `libnotify-bin` 软件包，或安装您所用发行版中的等效软件包。

> [tab] Windows (PowerShell)
>
> ```json theme={null}
> {
>   "hooks": {
>     "Notification": [
>       {
>         "matcher": "",
>         "hooks": [
>           {
>             "type": "command",
>             "command": "powershell.exe -Command \"[System.Reflection.Assembly]::LoadWithPartialName('System.Windows.Forms'); [System.Windows.Forms.MessageBox]::Show('Claude Code needs your attention', 'Claude Code')\""
>           }
>         ]
>       }
>     ]
>   }
> }
> ```
>
> > [accordion] 如果没有出现对话框
> >
> > 此命令打开的是一个对话框，而不是屏幕角落的通知，因此该对话框可能会在终端窗口后面打开。请先在 PowerShell 中直接测试该命令。
> >
> > 如果您在 WSL 中运行 Claude Code，则 `powershell.exe` 必须能够通过 Windows 互操作在您的 `PATH` 中使用。

空的 `matcher` 会对所有通知类型触发。若只想在特定事件时触发，请将其设置为以下值之一：

| 匹配器 | 触发时机 |
| :- | :- |
| `permission_prompt` | Claude 需要您批准某次工具使用或某个沙箱命令的[网络请求](https://code.claude.com/docs/zh-CN/sandboxing.md#network-isolation)，且该提示已等待约六秒 |
| `idle_prompt` | Claude 大约 60 秒前已完成回复，且您此后一直没有输入 |
| `auth_success` | 身份验证完成 |
| `elicitation_dialog` | MCP 服务器打开了一个信息征询表单，且您约六秒没有输入 |
| `elicitation_url_dialog` | MCP 服务器要求您在浏览器中打开某个 URL，且您约六秒没有输入 |
| `elicitation_complete` | MCP 服务器报告某个 [URL 模式信息征询](https://code.claude.com/docs/zh-CN/hooks.md#elicitation-input)已完成 |
| `elicitation_response` | MCP 信息征询的响应被发回服务器 |
| `agent_needs_input` | 在 [Agent 视图](https://code.claude.com/docs/zh-CN/agent-view.md)打开时，某个后台会话开始等待您的输入。当终端会话向您显示 [agent team 队友的终端设置问题](https://code.claude.com/docs/zh-CN/agent-teams.md#choose-a-display-mode)或自动模式关于[分类器请求费用](https://code.claude.com/docs/zh-CN/auto-mode-classifier-billing.md)的通知，且您约六秒没有输入时，也会触发 |
| `agent_completed` | 某个后台会话完成或失败。仅在 [Agent 视图](https://code.claude.com/docs/zh-CN/agent-view.md)打开时触发 |
| `quota_auto_resume_fired` | 在 claude.ai 用量限制暂停任务后，Claude Code 继续执行您的任务：在重置时继续，或者当您在等待期间于 Claude Code 中执行的某些操作（例如添加使用额度、升级套餐或切换模型）使用量重新可用时更早继续，[模型设置例外](https://code.claude.com/docs/zh-CN/interactive-mode.md#wait-for-a-usage-limit-to-reset)除外 |
| `quota_auto_resume_stale` | 在您的计算机休眠超过约 30 分钟期间，claude.ai 用量限制已重置。Claude Code 会等待您按 `Enter`，而不是继续执行。如果休眠时间较短，它会继续执行并改为触发 `quota_auto_resume_fired` |
| `quota_auto_resume_disabled` | Claude Code 结束对 claude.ai 用量限制的等待，但没有继续您的任务：[`autoContinueAtUsageLimit`](https://code.claude.com/docs/zh-CN/settings-reference#autocontinueatusagelimit) 被关闭，或者在 Claude Code 自行开始的等待期间重置时间被推迟到 24 小时以后，继续执行的任务不断触及限制，或者继续执行在到达模型之前被阻止。当您按 `Esc` 或 `Ctrl+C`，或选择 **Don't continue automatically** 时不会触发 |
| `plugin_notification` | 某个 [mod](https://code.claude.com/docs/zh-CN/plugins/mods/api.md#show-something-without-starting-a-turn) 通过 `$.ui.notify` 发出通知。除非 mod 设置了 `title`，否则该 hook 的 `title` 为 mod 的名称 |

在终端中，与在 Claude Desktop、VS Code 扩展以及其他通过 Agent SDK 响应权限请求的宿主中，Claude Code 对 `permission_prompt` 的计时方式不同。有关两种计时方式，请参阅[各通知类型的触发时机](https://code.claude.com/docs/zh-CN/hooks.md#notification)。

`quota_auto_resume_fired`、`quota_auto_resume_stale` 和 `quota_auto_resume_disabled` 匹配器需要 Claude Code v2.1.234 或更高版本。

在终端会话中，针对沙箱命令网络请求的 `permission_prompt` 需要 Claude Code v2.1.246 或更高版本。

针对队友终端设置问题的 `agent_needs_input` 需要 Claude Code v2.1.248 或更高版本。

在 Claude Code 输入框中键入 `/hooks`，确认该 hook 出现在 `Notification` 下。

### 编辑后自动格式化代码

对 Claude 编辑的每个文件自动运行 [Prettier](https://prettier.io/)，从而无需手动干预即可保持格式一致。

此 hook 使用带有 `Edit|Write` 匹配器的 `PostToolUse` 事件，因此仅在文件编辑工具之后运行。该命令使用 [`jq`](https://jqlang.org/) 提取被编辑文件的路径，并将其传递给 Prettier。将以下内容添加到项目根目录中的 `.claude/settings.json`：

```json theme={null}
{
  "hooks": {
    "PostToolUse": [
      {
        "matcher": "Edit|Write",
        "hooks": [
          {
            "type": "command",
            "command": "jq -r '.tool_input.file_path' | xargs npx prettier --write"
          }
        ]
      }
    ]
  }
}
```

要测试该 hook，请让 Claude 向某个 JavaScript 文件添加一行包含单引号字符串的代码，然后打开该文件：在 Prettier 的默认设置下，该 hook 会将它们改写为双引号。

hook 成功执行时，Claude Code 不会在对话中显示任何内容。要确认 hook 已运行，请检查被编辑的文件是否已重新格式化，或参阅[检查 hook 执行了什么](#check-what-a-hook-did)。

若要在特定文件发生任何变更时（包括被 `Bash` 命令改写时）重新格式化该文件，请改用 [FileChanged](https://code.claude.com/docs/zh-CN/hooks.md#filechanged) hook。

> [!NOTE]
>
> 本页的 Bash 示例使用 `jq` 解析 JSON。在 macOS 上使用 `brew install jq` 安装，在 Debian 和 Ubuntu 上使用 `apt-get install jq` 安装，或参阅 [`jq` 下载](https://jqlang.org/download/)。

### 阻止编辑受保护的文件

防止 Claude 修改敏感文件，例如 `.env`、`package-lock.json` 或 `.git/` 中的任何内容。Claude 会收到说明编辑被阻止原因的反馈，从而可以调整其做法。

此示例使用一个由 hook 调用的独立脚本文件。该脚本将目标文件路径与受保护模式列表进行比对，并以退出码 2 退出以阻止编辑。

**1. 创建 hook 脚本**

将以下内容保存到 `.claude/hooks/protect-files.sh`：

```bash theme={null}
#!/bin/bash
# protect-files.sh

INPUT=$(cat)
FILE_PATH=$(echo "$INPUT" | jq -r '.tool_input.file_path // empty')

# Normalize Windows backslash separators so the patterns below match
FILE_PATH="${FILE_PATH//\\//}"

PROTECTED_PATTERNS=(".env" "package-lock.json" ".git/")

for pattern in "${PROTECTED_PATTERNS[@]}"; do
  if [[ "$FILE_PATH" == *"$pattern"* ]]; then
    echo "Blocked: $FILE_PATH matches protected pattern '$pattern'" >&2
    exit 2
  fi
done

exit 0
```

**2. 在 macOS 和 Linux 上使脚本可执行**

hook 脚本必须具有可执行权限，Claude Code 才能运行它们：

```bash theme={null}
chmod +x .claude/hooks/protect-files.sh
```

**3. 注册 hook**

向 `.claude/settings.json` 添加一个 `PreToolUse` hook，在任何 `Edit` 或 `Write` 工具调用之前运行该脚本：

```json theme={null}
{
  "hooks": {
    "PreToolUse": [
      {
        "matcher": "Edit|Write",
        "hooks": [
          {
            "type": "command",
            "command": "\"$CLAUDE_PROJECT_DIR\"/.claude/hooks/protect-files.sh"
          }
        ]
      }
    ]
  }
}
```

**4. 测试 hook**

让 Claude 向您的 `.env` 文件添加一条注释。Claude Code 会在编辑执行前将其阻止，并将脚本的 `Blocked:` 消息作为反馈传递给 Claude。

### 上下文压缩后重新注入上下文

当 Claude 的上下文窗口填满时，压缩会总结对话以释放空间。这可能会丢失重要细节。使用带有 `compact` 匹配器的 `SessionStart` hook，在每次压缩后重新注入关键上下文。

Claude Code 会将您的命令写入 stdout 的纯文本添加到 Claude 的上下文中。此示例提醒 Claude 项目约定和近期工作。将以下内容添加到项目根目录中的 `.claude/settings.json`：

```json theme={null}
{
  "hooks": {
    "SessionStart": [
      {
        "matcher": "compact",
        "hooks": [
          {
            "type": "command",
            "command": "echo 'Reminder: use Bun, not npm. Run bun test before committing. Current sprint: auth refactor.'"
          }
        ]
      }
    ]
  }
}
```

您可以将 `echo` 替换为任何产生动态输出的命令，例如使用 `git log --oneline -5` 显示最近的提交。若要在每次会话开始时注入上下文，请考虑改用 [CLAUDE.md](https://code.claude.com/docs/zh-CN/memory.md)。有关环境变量，请参阅参考中的 [`CLAUDE_ENV_FILE`](https://code.claude.com/docs/zh-CN/hooks#persist-environment-variables)。

### 审计配置变更

跟踪会话期间设置文件或 skill 文件何时发生变更。当外部进程或编辑器修改配置文件时，会触发 `ConfigChange` 事件，因此您可以记录变更以满足合规要求，或阻止未经授权的修改。

此示例将每次变更追加到审计日志中。将以下内容添加到 `~/.claude/settings.json`：

```json theme={null}
{
  "hooks": {
    "ConfigChange": [
      {
        "matcher": "",
        "hooks": [
          {
            "type": "command",
            "command": "jq -c '{timestamp: now | todate, source: .source, file: .file_path}' >> ~/claude-config-audit.log"
          }
        ]
      }
    ]
  }
}
```

匹配器按配置类型进行筛选：`user_settings`、`project_settings`、`local_settings`、`policy_settings` 或 `skills`。若要阻止变更生效，请以退出码 2 退出或返回 `{"decision": "block"}`。有关完整的输入 schema，请参阅 [ConfigChange 参考](https://code.claude.com/docs/zh-CN/hooks.md#configchange)。

要确认该 hook 会记录变更，请在会话运行期间用另一个编辑器编辑某个设置文件，然后打开 `~/claude-config-audit.log`：该 hook 会为每次变更追加一行 JSON，其中包含时间戳、来源和文件路径。

### 目录或文件变更时重新加载环境

有些项目会根据您所在的目录设置不同的环境变量。[direnv](https://direnv.net/) 等工具会在您的 shell 中自动完成此操作，但 Claude 的 Bash 工具本身不会获取这些变更。

将 `SessionStart` hook 与 `CwdChanged` hook 搭配使用即可解决此问题。`SessionStart` 加载您启动时所在目录的变量，而 `CwdChanged` 会在 Claude 每次切换目录时重新加载这些变量。两者都会写入 `CLAUDE_ENV_FILE`，Claude Code 会在每个 Bash 命令之前将其作为脚本前导运行。将以下内容添加到 `~/.claude/settings.json`：

```json theme={null}
{
  "hooks": {
    "SessionStart": [
      {
        "hooks": [
          {
            "type": "command",
            "command": "direnv export bash > \"$CLAUDE_ENV_FILE\""
          }
        ]
      }
    ],
    "CwdChanged": [
      {
        "hooks": [
          {
            "type": "command",
            "command": "direnv export bash > \"$CLAUDE_ENV_FILE\""
          }
        ]
      }
    ]
  }
}
```

在每个包含 `.envrc` 的目录中运行一次 `direnv allow`，以允许 direnv 加载它。如果您使用 devbox 或 nix 而不是 direnv，同样的模式也适用，只需用 `devbox shellenv` 或 `devbox global shellenv` 替换 `direnv export bash`。

若要对特定文件而不是每次目录变更做出响应，请使用 `FileChanged`，并在 `matcher` 中列出要监视的文件名，以 `|` 分隔。构建监视列表时，Claude Code 会将该值拆分为字面文件名，而不是将其作为正则表达式求值。有关同一个值如何在文件变更时筛选要运行的 hook 组，请参阅 [FileChanged](https://code.claude.com/docs/zh-CN/hooks.md#filechanged)。此示例监视工作目录中的 `.envrc` 和 `.env`：

```json theme={null}
{
  "hooks": {
    "FileChanged": [
      {
        "matcher": ".envrc|.env",
        "hooks": [
          {
            "type": "command",
            "command": "direnv export bash > \"$CLAUDE_ENV_FILE\""
          }
        ]
      }
    ]
  }
}
```

有关输入 schema、`watchPaths` 输出以及 `CLAUDE_ENV_FILE` 的详细信息，请参阅 [CwdChanged](https://code.claude.com/docs/zh-CN/hooks.md#cwdchanged) 和 [FileChanged](https://code.claude.com/docs/zh-CN/hooks.md#filechanged) 参考条目。

### 自动批准特定的权限提示

对于您始终允许的工具调用，跳过批准对话框。此示例自动批准 `ExitPlanMode`，即 Claude 在展示完计划并请求继续时调用的工具，这样每次计划就绪时都不会提示您。

与上面的退出码示例不同，自动批准要求您的 hook 将 JSON 决策写入 stdout。Claude Code 在即将向您请求权限时运行 `PermissionRequest` hook，如果您的 hook 返回 `"behavior": "allow"`，Claude Code 就会代表您响应该请求。

匹配器将该 hook 仅限定于 `ExitPlanMode`，因此不会影响其他提示。将以下内容添加到 `~/.claude/settings.json`：

```json theme={null}
{
  "hooks": {
    "PermissionRequest": [
      {
        "matcher": "ExitPlanMode",
        "hooks": [
          {
            "type": "command",
            "command": "echo '{\"hookSpecificOutput\": {\"hookEventName\": \"PermissionRequest\", \"decision\": {\"behavior\": \"allow\"}}}'"
          }
        ]
      }
    ]
  }
}
```

当该 hook 批准时，Claude Code 会退出计划模式，并恢复您进入计划模式之前处于活动状态的权限模式。会话记录会在原本出现对话框的位置显示“Allowed by PermissionRequest hook”。hook 路径始终保留当前对话：它无法像对话框那样清除上下文并开始一个全新的实现会话。

若要改为设置特定的权限模式，您的 hook 输出可以包含一个带有 `setMode` 条目的 `updatedPermissions` 数组。`mode` 值可以是任何权限模式，例如 `default`、`acceptEdits` 或 `bypassPermissions`，而 `destination: "session"` 会使其仅对当前会话生效。

> [!NOTE]
>
> `bypassPermissions` 仅在您启动会话时 bypass 模式已可用的情况下才会生效：通过 `--dangerously-skip-permissions`、`--permission-mode bypassPermissions`、`--allow-dangerously-skip-permissions`，或在[用户设置、`--settings` 或托管设置](https://code.claude.com/docs/zh-CN/settings-reference#permissions-defaultmode)中设置 `permissions.defaultMode: "bypassPermissions"`。如果 bypass 模式已被 [`permissions.disableBypassPermissionsMode`](https://code.claude.com/docs/zh-CN/permissions#managed-settings) 禁用，或者您在[受限模式](https://code.claude.com/docs/zh-CN/cli-reference.md#cli-flags)下启动了会话，则它不会生效。
>
> Claude Code 永远不会将其保存为 `defaultMode`。

若要将会话切换到 `acceptEdits`，您的 hook 将以下 JSON 写入 stdout：

```json theme={null}
{
  "hookSpecificOutput": {
    "hookEventName": "PermissionRequest",
    "decision": {
      "behavior": "allow",
      "updatedPermissions": [
        { "type": "setMode", "mode": "acceptEdits", "destination": "session" }
      ]
    }
  }
}
```

请让匹配器尽可能精确。匹配 `.*` 或将匹配器留空会自动批准所有工具权限提示，包括文件写入和 shell 命令。有关完整的决策字段集，请参阅 [PermissionRequest 参考](https://code.claude.com/docs/zh-CN/hooks.md#permissionrequest-decision-control)。

## Hooks 如何工作

Claude Code 在其生命周期中的特定点触发 hook 事件。当事件触发时，Claude Code 会并行运行所有匹配的 hook；有关重复处理程序如何处理的信息，请参阅 [Hook 处理程序字段](https://code.claude.com/docs/zh-CN/hooks.md#hook-handler-fields)。下表显示每个事件及其触发时间：

| 事件 | 触发时机 |
| :- | :- |
| `SessionStart` | 当会话开始或恢复时 |
| `Setup` | 当你使用 `--init-only` 启动 Claude Code，或在 `-p` 模式下使用 `--init` 或 `--maintenance` 时。用于 CI 或脚本中的一次性准备 |
| `UserPromptSubmit` | 当提交提示词时，在 Claude 处理之前。对于 [Claude Code 自行发起的轮次](https://code.claude.com/docs/zh-CN/hooks.md#userpromptsubmit)也会触发 |
| `UserPromptExpansion` | 当用户输入的命令扩展为提示词时，在到达 Claude 之前。可以阻止扩展 |
| `PreToolUse` | 在工具调用执行之前。可以阻止它 |
| `PermissionRequest` | 当工具调用需要权限决策时 |
| `PermissionDenied` | 当自动模式拒绝工具调用时，包括没有分类器判决的拒绝。使用 JSON `hookSpecificOutput.retry: true` 来告诉模型它可以重试被拒绝的工具调用。Claude Code 在分类器未产生判决时忽略 `retry` |
| `PostToolUse` | 在工具调用成功后 |
| `PostToolUseFailure` | 在工具调用失败后 |
| `PostToolBatch` | 在一整批并行工具调用解决后，在下一次模型调用之前 |
| `Notification` | 当 Claude Code 发送通知时 |
| `MessageDisplay` | 当助手消息文本正在显示时 |
| `SubagentStart` | 当子代理被生成时 |
| `SubagentStop` | 当子代理完成时 |
| `TaskCreated` | 当通过 `TaskCreate` 创建任务时 |
| `TaskCompleted` | 当任务被标记为已完成时 |
| `Stop` | 当 Claude 完成响应时 |
| `StopFailure` | 当轮次因 API 错误而结束时 |
| `TeammateIdle` | 当[代理团队](https://code.claude.com/docs/zh-CN/agent-teams.md)队友即将空闲时 |
| `InstructionsLoaded` | 当 CLAUDE.md 或 `.claude/rules/*.md` 文件被加载到上下文中时。在会话开始时和文件在会话期间被延迟加载时触发 |
| `ConfigChange` | 当配置文件在会话期间更改时 |
| `CwdChanged` | 当工作目录更改时，例如当 Claude 执行 `cd` 命令时。对于使用 direnv 等工具的反应式环境管理很有用 |
| `DirectoryAdded` | 当工作目录在会话中期通过 `/add-dir` 或 SDK `register_repo_root` 控制请求添加时 |
| `FileChanged` | 当监视的文件在磁盘上更改时。`matcher` 字段指定要监视的文件名 |
| `WorktreeCreate` | 当通过 `--worktree`、`isolation: "worktree"` 创建工作树时，或用于后台会话。替换默认的 git 行为 |
| `WorktreeRemove` | 当由 `WorktreeCreate` hook 创建的 worktree 正在被移除时 |
| `PreCompact` | 在上下文压缩之前 |
| `PostCompact` | 在上下文压缩完成后 |
| `PreModelSwitch` | 在 Claude Code 应用你或客户端请求的模型切换之前。可以阻止切换 |
| `PostModelSwitch` | 在会话的模型更改后，包括 Claude Code 自己进行的更改，例如在你恢复会话时恢复模型 |
| `Elicitation` | 当 MCP 服务器在工具调用期间请求用户输入时 |
| `ElicitationResult` | 在用户响应 MCP 引出后，在响应发送回服务器之前 |
| `SessionEnd` | 当会话终止时 |

每个 hook 都有一个 `type` 来确定它如何运行。大多数 hooks 使用 `"type": "command"`，它运行 shell 命令。还有四种其他类型可用：

* `"type": "http"`：将事件数据 POST 到 URL。请参阅 [HTTP hooks](#http-hooks)。
* `"type": "mcp_tool"`：在已配置的 MCP 服务器上调用工具。请参阅 [MCP tool hooks](https://code.claude.com/docs/zh-CN/hooks.md#mcp-tool-hook-fields)。
* `"type": "prompt"`：单轮 LLM 评估。请参阅 [Prompt-based hooks](#prompt-based-hooks)。
* `"type": "agent"`：具有工具访问权限的多轮验证。Agent hooks 是实验性的，可能会改变。请参阅 [Agent-based hooks](#agent-based-hooks)。

### 合并来自多个 hooks 的结果

当多个 hooks 匹配同一事件时，每个 hook 的命令都会运行到完成，然后 Claude Code 合并结果。一个 hook 返回 `deny` 不会阻止兄弟 hooks 执行。不要依赖一个 hook 的 `deny` 来抑制另一个 hook 中的副作用。

所有匹配的 hooks 完成后，Claude Code 合并它们的输出。对于 `PreToolUse` 权限决策，最严格的答案获胜，顺序为 `deny`、`defer`、`ask`、`allow`。来自 `additionalContext` 的文本从每个 hook 保留并一起传递给 Claude。

下面的示例在 `Bash` 上注册两个 `PreToolUse` hooks。第一个将每个命令附加到日志文件并以 0 退出。第二个运行一个脚本，当命令包含 `rm -rf` 时以 2 退出以拒绝：

```json theme={null}
{
  "hooks": {
    "PreToolUse": [
      {
        "matcher": "Bash",
        "hooks": [
          {
            "type": "command",
            "command": "jq -r .tool_input.command >> ~/.claude/bash.log"
          },
          {
            "type": "command",
            "command": "\"$CLAUDE_PROJECT_DIR\"/.claude/hooks/block-rm-rf.sh"
          }
        ]
      }
    ]
  }
}
```

当 Claude 尝试运行 `rm -rf /tmp/build` 时，两个 hooks 并行执行。日志 hook 将命令写入 `~/.claude/bash.log` 并以 0 退出，这表示没有决策。防护栏 hook 以 2 退出，这拒绝了工具调用。拒绝获胜，所以 Claude Code 阻止命令并向 Claude 显示防护栏的 stderr。日志条目仍然被写入，因为日志 hook 已经运行。

### 读取输入并返回输出

Hooks 通过 stdin、stdout、stderr 和退出代码与 Claude Code 通信。当事件触发时，Claude Code 将事件特定的数据作为 JSON 传递到脚本的 stdin。你的脚本读取该数据，完成其工作，并通过退出代码告诉 Claude Code 接下来要做什么。

#### Hook 输入

每个事件都包含常见字段，如 `session_id`（会话的唯一 ID）和 `cwd`（事件触发时的工作目录），但每个事件类型添加不同的数据。当 Claude 运行 Bash 命令时，`PreToolUse` hook 在 stdin 上接收这些字段：

* `hook_event_name`：触发 hook 的事件
* `tool_name`：Claude 即将使用的工具
* `tool_input`：Claude 传递给工具的参数。对于 Bash，其 `command` 字段包含 shell 命令。

例如，`npm test` 命令的 hook 输入看起来像这样：

```json theme={null}
{
  "session_id": "abc123",
  "cwd": "/Users/sarah/myproject",
  "hook_event_name": "PreToolUse",
  "tool_name": "Bash",
  "tool_input": {
    "command": "npm test"
  }
}
```

你的脚本可以解析该 JSON 并对任何这些字段进行操作。`UserPromptSubmit` hooks 获取 `prompt` 文本，`SessionStart` hooks 获取 `source`（`startup`、`resume`、`clear`、`compact` 或 `fork`），等等。有关共享字段，请参阅参考中的 [常见输入字段](https://code.claude.com/docs/zh-CN/hooks.md#common-input-fields)，以及每个事件的部分了解事件特定的架构。

#### Hook 输出

你的脚本通过写入 stdout 或 stderr 并以特定代码退出来告诉 Claude Code 接下来要做什么。以下 `PreToolUse` hook 阻止一个命令：

```bash theme={null}
#!/bin/bash
INPUT=$(cat)
COMMAND=$(echo "$INPUT" | jq -r '.tool_input.command')

if echo "$COMMAND" | grep -q "drop table"; then
  echo "Blocked: dropping tables is not allowed" >&2  # stderr becomes Claude's feedback
  exit 2 # exit 2 = block the action
fi

exit 0  # exit 0 = no decision; the normal permission flow applies
```

当退出码既不是 0 也不是 2，且没有有效的 JSON 输出时，对于大多数事件，操作会继续进行，除非该 hook 属于 [失败时会阻止操作](https://code.claude.com/docs/zh-CN/hooks.md#block-the-action-when-a-hook-fails) 的 hook。将 `onFailure` 设置为 `"block"` 的 hook 就是一个例子。以 0 退出表示没有异议，以 2 退出则阻止操作：

* **退出 0**：如果从 `PreToolUse` hook 以 0 退出，工具调用仍会经过正常的 [权限流程](https://code.claude.com/docs/zh-CN/permissions.md)，因为退出 0 并不会批准它。对于 `UserPromptSubmit`、`UserPromptExpansion`、`SessionStart` 和 `PostModelSwitch` hook，Claude Code 会将其 [视为纯文本](https://code.claude.com/docs/zh-CN/hooks.md#exit-code-0) 的 stdout 添加到 Claude 的上下文中。
* **退出 2**：将原因写入 stderr。根据事件的不同，Claude 会将其作为反馈接收以便进行调整，或者向用户显示，或者不显示任何消息（如 `ConfigChange` 和 `Elicitation`）。某些事件无法被阻止：对于 `SessionStart` 等事件，退出 2 会向用户显示 stderr，并继续执行。有关完整列表，请参阅 [每个事件的退出码 2 行为](https://code.claude.com/docs/zh-CN/hooks.md#exit-code-2-behavior-per-event)。
* **任何其他退出码**：有关每个事件的例外情况（例如 `WorktreeCreate` 在任何非零退出时都会失败），请参阅 [退出码输出](https://code.claude.com/docs/zh-CN/hooks.md#exit-code-output)。对于其他事件，除非该 hook 属于失败时会阻止操作的 hook，否则请根据 hook 打印到 stdout 的内容找到对应的情况：
  * 通过 schema 验证的已解析对象：Claude Code 忽略退出码，仅由 JSON 决定结果，hook 不会被报告为错误。
  * 未通过 schema 验证的已解析对象，或 Claude Code [尝试解析为 JSON](https://code.claude.com/docs/zh-CN/hooks.md#exit-code-0) 但不是有效 JSON 的 stdout：操作作为非阻止错误继续进行。请查看 `<hook name> hook error` 通知中的验证或解析消息。
  * Claude Code [视为纯文本](https://code.claude.com/docs/zh-CN/hooks.md#exit-code-0) 的 stdout，或空 stdout：操作作为非阻止错误继续进行。会话记录显示 `<hook name> hook error` 通知，然后是 stderr 的第一行，前缀为 `Failed with non-blocking status code:`。要捕获完整的 stderr，请使用 `claude --debug` 或在会话中运行 `/debug` 启用 [调试日志](https://code.claude.com/docs/zh-CN/hooks.md#debug-hooks)。

#### 结构化 JSON 输出

退出代码只让你阻止或保持沉默。为了获得更多控制，退出 0 并改为将 JSON 对象打印到 stdout。

> [!NOTE]
>
> 使用退出 2 以 stderr 消息阻止，或使用 JSON 退出 0 以获得结构化控制。每个 hook 选择一种方法。有关混合它们时会发生什么，请参阅 [退出代码输出](https://code.claude.com/docs/zh-CN/hooks.md#exit-code-output)。

例如，`PreToolUse` hook 可以拒绝工具调用并告诉 Claude 为什么，或将其升级给用户以获得批准：

```json theme={null}
{
  "hookSpecificOutput": {
    "hookEventName": "PreToolUse",
    "permissionDecision": "deny",
    "permissionDecisionReason": "Use rg instead of grep for better performance"
  }
}
```

使用 `"deny"`，Claude Code 取消工具调用并将 `permissionDecisionReason` 反馈给 Claude。

在 `PreToolUse` 上，还可以返回 `"ask"` 以向用户显示正常的权限提示，或返回 `"allow"` 以跳过交互式权限提示。使用 `"allow"` 时，拒绝和询问规则（包括企业托管拒绝列表）仍然适用，以下情况的提示也仍然适用：

* 从 [网络路径](https://code.claude.com/docs/zh-CN/permissions.md#network-paths) 读取
* 标记为 [`requiresUserInteraction`](https://code.claude.com/docs/zh-CN/mcp#require-approval-for-a-specific-tool) 的 MCP 工具
* 对 [Agent SDK 应用标记为 `always_ask` 的工具](https://code.claude.com/docs/zh-CN/agent-sdk/custom-tools#require-approval-for-specific-tools) 的调用，除非 [权限模式批准该调用](https://code.claude.com/docs/zh-CN/agent-sdk/typescript.md#permission-modes-that-can-approve-an-always_ask-call)
* 在该设置到达 Claude Code 的会话中，[您的组织设置为 `ask`](https://code.claude.com/docs/zh-CN/mcp#organization-controls-on-connector-tools) 的连接器工具

  有关 `"allow"` 在计划模式下的作用，请参阅 [计划模式下的 hook 批准](https://code.claude.com/docs/zh-CN/permissions.md#hook-approvals-in-plan-mode)。

第四个值 `"defer"` 在 [非交互模式](https://code.claude.com/docs/zh-CN/headless.md) 中使用 `-p` 标志时可用。返回该值会结束当前轮次并保留工具调用，以便 Agent SDK 包装器可以收集输入并恢复。请参阅参考中的 [延迟工具调用以供稍后使用](https://code.claude.com/docs/zh-CN/hooks.md#defer-a-tool-call-for-later)。

`PreModelSwitch` hook 返回相同的 `permissionDecision` 字段：`"allow"` 让模型切换进行，`"deny"` 取消它。`"ask"` 在你在交互式会话中运行 `/model` 时让你确认切换；在其他地方，Claude Code 将 `"ask"` 视为拒绝。请参阅 [PreModelSwitch 决策控制](https://code.claude.com/docs/zh-CN/hooks.md#premodelswitch-decision-control)。

其他事件使用不同的决策模式。例如，`PostToolUse` 和 `Stop` hooks 使用顶级 `decision: "block"` 字段，而 `PermissionRequest` 使用 `hookSpecificOutput.decision.behavior`。有关按事件的完整分解，请参阅参考中的 [摘要表](https://code.claude.com/docs/zh-CN/hooks.md#decision-control)。

对于 `UserPromptSubmit` hooks，改用 `hookSpecificOutput.additionalContext` 将文本注入到 Claude 的上下文中。将 `additionalContext` 嵌套在 `hookSpecificOutput` 内；如果你将其放在 JSON 的顶级，Claude Code 会默默忽略它。例如，此输出将当前分支状态添加到每个提示：

```json theme={null}
{
  "hookSpecificOutput": {
    "hookEventName": "UserPromptSubmit",
    "additionalContext": "Current branch: release-42. Deploy freeze until Friday."
  }
}
```

有关完整的输出形状（包括阻止提示和设置会话标题），请参阅 [UserPromptSubmit 决策控制](https://code.claude.com/docs/zh-CN/hooks.md#userpromptsubmit-decision-control)。

具有 `type: "prompt"` 的 Hooks 处理输出的方式不同：请参阅 [Prompt-based hooks](#prompt-based-hooks)。

### 使用匹配器过滤 hooks

没有匹配器，hook 会在其事件的每次出现时触发。匹配器让你缩小范围。例如，如果你只想在文件编辑后运行格式化程序（而不是在每个工具调用后），将匹配器添加到你的 `PostToolUse` hook：

```json theme={null}
{
  "hooks": {
    "PostToolUse": [
      {
        "matcher": "Edit|Write",
        "hooks": [
          { "type": "command", "command": "prettier --write ..." }
        ]
      }
    ]
  }
}
```

`"Edit|Write"` 匹配器仅在 Claude 使用 `Edit` 或 `Write` 工具时触发，而不是在它使用 `Bash`、`Read` 或任何其他工具时触发。逗号以相同的方式分隔替代项，所以 `"Edit, Write"` 是等效的。请参阅 [匹配器模式](https://code.claude.com/docs/zh-CN/hooks.md#matcher-patterns) 了解纯名称和正则表达式如何被评估。

> [!NOTE]
>
> Claude 也可以通过运行 shell 命令来创建或修改文件。如果你的 hook 必须看到每个文件更改（例如用于合规性扫描或审计日志），添加一个 [`Stop`](https://code.claude.com/docs/zh-CN/hooks#stop) hook，它每轮扫描一次工作树。为了获得每次调用的覆盖，也匹配 `Bash|PowerShell` 并让你的脚本使用 `git status --porcelain` 列出修改和未跟踪的文件。[PowerShell hook 输入部分](https://code.claude.com/docs/zh-CN/hooks.md#powershell) 解释了为什么仅匹配 `Bash` 是不够的。要在特定文件在磁盘上更改时运行 hook（无论是什么写入它），使用 [FileChanged](https://code.claude.com/docs/zh-CN/hooks.md#filechanged) hook。

每个事件类型在特定字段上匹配：

| 事件 | 匹配器过滤的内容 | 示例匹配器值 |
| :- | :- | :- |
| `PreToolUse`、`PostToolUse`、`PostToolUseFailure`、`PermissionRequest`、`PermissionDenied` | 工具名称 | `Bash`、`Edit\|Write`、`mcp__.*` |
| `SessionStart` | 会话如何启动 | `startup`、`resume`、`clear`、`compact`、`fork` |
| `Setup` | 哪个 CLI 标志触发了设置 | `init`、`maintenance` |
| `SessionEnd` | 会话为什么结束 | `clear`、`resume`、`logout`、`prompt_input_exit`、`other` |
| `Notification` | 通知类型 | `permission_prompt`、`idle_prompt`、`auth_success`、`elicitation_dialog`、`elicitation_url_dialog`、`elicitation_complete`、`elicitation_response`、`agent_needs_input`、`agent_completed`、`quota_auto_resume_fired`、`quota_auto_resume_stale`、`quota_auto_resume_disabled`、`plugin_notification` |
| `SubagentStart` | 代理类型 | `general-purpose`、`Explore`、`Plan` 或自定义代理名称 |
| `PreCompact`、`PostCompact` | 什么触发了压缩 | `manual`、`auto` |
| `PreModelSwitch`、`PostModelSwitch` | 会话切换到的模型的规范名称，如 [PreModelSwitch](https://code.claude.com/docs/zh-CN/hooks.md#premodelswitch) 下所述 | `claude-opus-5`、`claude-opus-4-6\|claude-opus-5`、`.*opus.*` |
| `SubagentStop` | 代理类型 | 与 `SubagentStart` 相同的值 |
| `ConfigChange` | 配置源 | `user_settings`、`project_settings`、`local_settings`、`policy_settings`、`skills` |
| `DirectoryAdded` | 目录如何被添加 | `slash_command`、`register_repo_root` |
| `StopFailure` | 错误类型 | `rate_limit`、`overloaded`、`authentication_failed`、`oauth_org_not_allowed`、`account_on_hold`、`billing_error`、`invalid_request`、`model_not_found`、`server_error`、`max_output_tokens`、`cloud_credential_error`、`unknown` |
| `InstructionsLoaded` | 加载原因 | `session_start`、`nested_traversal`、`path_glob_match`、`include`、`compact` |
| `Elicitation` | MCP 服务器名称 | 你配置的 MCP 服务器名称 |
| `ElicitationResult` | MCP 服务器名称 | 与 `Elicitation` 相同的值 |
| `FileChanged` | 要监视的文字文件名（请参阅 [FileChanged](https://code.claude.com/docs/zh-CN/hooks.md#filechanged)） | `.envrc\|.env` |
| `UserPromptExpansion` | 命令名称 | 你的 skill 或命令名称 |
| `UserPromptSubmit`、`PostToolBatch`、`Stop`、`TeammateIdle`、`TaskCreated`、`TaskCompleted`、`WorktreeCreate`、`WorktreeRemove`、`CwdChanged`、`MessageDisplay` | 不支持匹配器 | 始终在每次出现时触发 |

下面的选项卡显示不同事件类型上的更多匹配器示例。

> [tab] 记录每个 Bash 命令
>
> 仅匹配 `Bash` 工具调用并将每个命令记录到文件。`PostToolUse` 事件在命令完成后触发，因此 `tool_input.command` 包含运行的内容。hook 在 stdin 上接收事件数据作为 JSON，`jq -r '.tool_input.command'` 仅提取命令字符串，`>>` 将其附加到日志文件：
>
> ```json theme={null}
> {
>   "hooks": {
>     "PostToolUse": [
>       {
>         "matcher": "Bash",
>         "hooks": [
>           {
>             "type": "command",
>             "command": "jq -r '.tool_input.command' >> ~/.claude/command-log.txt"
>           }
>         ]
>       }
>     ]
>   }
> }
> ```

> [tab] 匹配 MCP 工具
>
> MCP 工具使用与内置工具不同的命名约定：`mcp__<server>__<tool>`，其中 `<server>` 是 MCP 服务器名称，`<tool>` 是它提供的工具。例如，`mcp__github__search_repositories` 或 `mcp__filesystem__read_file`。来自 [插件提供的 MCP 服务器](https://code.claude.com/docs/zh-CN/mcp.md#plugin-provided-mcp-servers) 的工具使用作用域服务器段，例如 `mcp__plugin_my-plugin_db__query`。使用正则表达式匹配器来针对来自特定服务器的所有工具，或使用 `mcp__.*__write.*` 之类的模式跨服务器匹配。有关完整的示例列表，请参阅参考中的 [匹配 MCP 工具](https://code.claude.com/docs/zh-CN/hooks.md#match-mcp-tools)。
>
> 下面的命令使用 `jq` 从 hook 的 JSON 输入中提取工具名称，并将其写入 stderr。将其写入 stderr 保持 stdout 清洁以用于 JSON 输出，并将消息发送到 [调试日志](https://code.claude.com/docs/zh-CN/hooks.md#debug-hooks)：
>
> ```json theme={null}
> {
>   "hooks": {
>     "PreToolUse": [
>       {
>         "matcher": "mcp__github__.*",
>         "hooks": [
>           {
>             "type": "command",
>             "command": "echo \"GitHub tool called: $(jq -r '.tool_name')\" >&2"
>           }
>         ]
>       }
>     ]
>   }
> }
> ```

> [tab] 在会话结束时清理
>
> `SessionEnd` 事件支持会话结束原因的匹配器。此 hook 仅在 `clear` 原因时触发（当你运行 `/clear` 时），而不是在正常退出时：
>
> ```json theme={null}
> {
>   "hooks": {
>     "SessionEnd": [
>       {
>         "matcher": "clear",
>         "hooks": [
>           {
>             "type": "command",
>             "command": "rm -f /tmp/claude-scratch-*.txt"
>           }
>         ]
>       }
>     ]
>   }
> }
> ```

#### 使用 `if` 字段按工具名称和参数过滤

`if` 字段使用 [权限规则语法](https://code.claude.com/docs/zh-CN/permissions.md) 按工具名称和参数一起过滤 hooks，因此 hook 进程仅在工具调用匹配时生成。这超越了 `matcher`，它仅在工具名称级别按组过滤。

例如，这个配置仅在 Claude 使用 `git` 命令而不是所有 Bash 命令时运行 hook：

```json theme={null}
{
  "hooks": {
    "PreToolUse": [
      {
        "matcher": "Bash",
        "hooks": [
          {
            "type": "command",
            "if": "Bash(git *)",
            "command": "\"$CLAUDE_PROJECT_DIR\"/.claude/hooks/check-git-policy.sh"
          }
        ]
      }
    ]
  }
}
```

你的 hook 命令是否运行取决于你的 `if` 模式的形状和 Claude 正在调用的 Bash 命令：

| `if` 模式 | Bash 命令 | Hook 运行？ | 为什么 |
| :- | :- | :- | :- |
| `Bash(git *)` | `git push` | 是 | 命令名称匹配 |
| `Bash(git *)` | `npm test && git push` | 是 | 每个子命令都被检查；`git push` 匹配 |
| `Bash(git *)` | `echo $(git log)` | 是 | `$()` 和反引号内的命令被检查；`git log` 匹配 |
| `Bash(git *)` | `echo $(date)` | 否 | 没有子命令匹配 `git *` |
| `Bash(git push *)` | `echo $(date)` | 是 | 指定超过命令名称的模式在 `$()`、反引号或 `$VAR` 上运行 hook |

当 Claude Code 无法确定 Bash 输入运行哪些命令时，它无论如何都会运行你的 hook。[Bash 匹配表](https://code.claude.com/docs/zh-CN/hooks.md#bash-if-matching) 涵盖 Claude Code 可以和不能按子命令缩小的命令形状。因为过滤器是尽力而为的，使用 [权限系统](https://code.claude.com/docs/zh-CN/permissions.md) 而不是 hook 来强制执行硬允许或拒绝。

`if` 字段接受与权限规则相同的模式：`"Bash(git *)"`、`"Edit(*.ts)"` 等。要匹配多个工具名称，使用单独的处理程序，每个都有自己的 `if` 值，或在 `matcher` 级别匹配，其中支持管道交替。

`if` 仅适用于工具事件：`PreToolUse`、`PostToolUse`、`PostToolUseFailure`、`PermissionRequest` 和 `PermissionDenied`。将其添加到任何其他事件会阻止 hook 运行。

### 配置 hook 位置

你添加 hook 的位置决定了其范围：

| 位置 | 范围 | 可共享 |
| :- | :- | :- |
| `~/.claude/settings.json` | 所有你的项目 | 否，本地到你的机器 |
| `.claude/settings.json` | 单个项目 | 是，可以提交到仓库 |
| `.claude/settings.local.json` | 单个项目 | 否，gitignored 当 Claude Code 保存设置到它时 |
| 托管策略设置 | 组织范围 | 是，管理员控制 |
| [Plugin](https://code.claude.com/docs/zh-CN/plugins/overview.md) `hooks/hooks.json` | 启用插件时 | 是，与插件捆绑 |
| [Skill](https://code.claude.com/docs/zh-CN/skills.md) frontmatter | 调用 skill 后的会话的其余部分。请参阅 [Skills 和 agents 中的 Hooks](https://code.claude.com/docs/zh-CN/hooks.md#hooks-in-skills-and-agents) | 是，在 skill 文件中定义 |
| [Subagent](https://code.claude.com/docs/zh-CN/sub-agents.md) frontmatter | 该 subagent 运行时 | 是，在 subagent 文件中定义 |

在 Claude Code 中运行 [`/hooks`](https://code.claude.com/docs/zh-CN/hooks#the-%2Fhooks-menu) 以浏览所有按事件分组的配置 hooks。

要禁用 hooks，在设置文件中设置 `"disableAllHooks": true`。Claude Code 读取 [设置优先级](https://code.claude.com/docs/zh-CN/hooks.md#disable-or-remove-hooks) 应用后剩余的值，因此项目的设置文件可以覆盖你的。托管设置中配置的 Hooks 仍然运行，除非 `disableAllHooks` 也在那里设置。有关每个级别的完整范围，请参阅 [`disableAllHooks`](https://code.claude.com/docs/zh-CN/settings-reference#disableallhooks)。

如果你在 Claude Code 运行时直接编辑设置文件，文件监视器通常会自动拾取 hook 更改。

## 基于提示的 hooks

对于需要判断而不是确定性规则的决策，使用 `type: "prompt"` hooks。Claude Code 不运行 shell 命令，而是将你的提示和 hook 的输入数据发送到 Claude 模型来做出决策。如果你需要更多功能，可以使用 `model` 字段指定不同的模型。

模型的唯一工作是返回其决策作为 JSON：

* `"ok": true`：操作继续
* `"ok": false`：发生的情况取决于事件：
  * `Stop` 和 `SubagentStop`：`reason` 被反馈给 Claude，以便它继续工作，除非响应也设置 `"impossible": true` 来标记该条件为永远无法满足的条件，在这种情况下 Claude Code 允许停止并且该轮次结束
  * `PreToolUse`：工具调用被拒绝；默认情况下该轮次结束，拒绝 `reason` 在聊天中显示为警告行。在 hook 上设置 `continueOnBlock: true` 以改为将 `reason` 作为工具错误返回给 Claude，以便它可以调整并继续
  * `PostToolUse`：默认情况下该轮次结束，`reason` 在聊天中显示为警告行。设置 `continueOnBlock: true` 以将 `reason` 反馈给 Claude 并继续该轮次
  * `PostToolBatch`、`UserPromptSubmit` 和 `UserPromptExpansion`：该轮次结束，`reason` 在聊天中显示为警告行

此示例使用 `Stop` hook 询问模型是否所有请求的任务都已完成。如果模型返回 `"ok": false` 因为条件尚未满足，Claude 继续工作并使用 `reason` 作为其下一条指令：

```json theme={null}
{
  "hooks": {
    "Stop": [
      {
        "hooks": [
          {
            "type": "prompt",
            "prompt": "Check if all tasks are complete. If not, respond with {\"ok\": false, \"reason\": \"what remains to be done\"}."
          }
        ]
      }
    ]
  }
}
```

有关完整的配置选项，请参阅参考中的 [基于提示的 hooks](https://code.claude.com/docs/zh-CN/hooks.md#prompt-based-hooks)。

## 基于代理的 hooks

> [!WARNING]
>
> 代理 hooks 是实验性的。行为和配置可能在未来版本中改变。对于生产工作流，更倾向于 [命令 hooks](https://code.claude.com/docs/zh-CN/hooks.md#command-hook-fields)。

当验证需要检查文件或运行命令时，使用 `type: "agent"` hooks。与只进行单个 LLM 调用的提示 hooks 不同，代理 hooks 生成一个 subagent，它可以读取文件、搜索代码和使用其他工具来验证条件，然后返回决策。

代理 hooks 使用 `"ok"` / `"reason"` 响应格式，默认超时更长（60 秒），最多支持 50 个工具使用轮次。它们不支持提示 hook 的 `impossible` 字段。当 `ok: false` 时，Claude Code 处理代理 hook 的方式与处理在同一事件上设置 `continueOnBlock: true` 的提示 hook 相同，因此在 `PreToolUse` 和 `PostToolUse` 上轮次继续；代理 hooks 没有 `continueOnBlock` 字段。有关字段的详细信息，请参阅 [代理 hook 配置](https://code.claude.com/docs/zh-CN/hooks.md#agent-hook-configuration)，包括 Claude Code 用 hook 的 JSON 输入替换的 `$ARGUMENTS` 占位符。

此示例验证在允许 Claude 停止之前测试通过：

```json theme={null}
{
  "hooks": {
    "Stop": [
      {
        "hooks": [
          {
            "type": "agent",
            "prompt": "Verify that all unit tests pass. Run the test suite and check the results. $ARGUMENTS",
            "timeout": 120
          }
        ]
      }
    ]
  }
}
```

当 hook 输入数据本身足以做出决策时使用提示 hooks。当你需要根据代码库的实际状态验证某些内容时使用代理 hooks。

有关完整的配置选项，请参阅参考中的 [基于代理的 hooks](https://code.claude.com/docs/zh-CN/hooks.md#agent-based-hooks)。

## HTTP hooks

使用 `type: "http"` hooks 将事件数据 POST 到 HTTP 端点，而不是运行 shell 命令。端点接收命令 hook 在 stdin 上接收的相同 JSON，并使用相同的 JSON 格式通过 HTTP 响应体返回结果。

HTTP hooks 在你想要 web 服务器、云函数或外部服务处理 hook 逻辑时很有用：例如，一个跨团队记录工具使用事件的共享审计服务。

此示例将每个工具使用 POST 到本地日志服务：

```json theme={null}
{
  "hooks": {
    "PostToolUse": [
      {
        "hooks": [
          {
            "type": "http",
            "url": "http://localhost:8080/hooks/tool-use",
            "headers": {
              "Authorization": "Bearer $MY_TOKEN"
            },
            "allowedEnvVars": ["MY_TOKEN"]
          }
        ]
      }
    ]
  }
}
```

您的端点使用与命令 hook 相同的[输出格式](https://code.claude.com/docs/zh-CN/hooks.md#json-output)返回 JSON 响应体，Claude Code 还会检查响应状态：

* **2xx 状态**：要阻止工具调用，请在响应体中返回相应的 `hookSpecificOutput` 字段。
* **任何其他状态，或请求失败**：Claude Code 会报告[非阻塞错误](https://code.claude.com/docs/zh-CN/hooks.md#exit-code-output)并允许操作继续。要让失败的端点阻止该操作，请在 hook 上设置 [`onFailure: "block"`](https://code.claude.com/docs/zh-CN/hooks#block-the-action-when-a-hook-fails)。

标头值支持使用 `$VAR_NAME` 或 `${VAR_NAME}` 语法的环境变量插值。仅解析 `allowedEnvVars` 数组中列出的变量；所有其他 `$VAR` 引用保持为空。

有关完整的配置选项和响应处理，请参阅参考中的 [HTTP hooks](https://code.claude.com/docs/zh-CN/hooks.md#http-hook-fields)。

## 限制和故障排除

### 限制

设计 hooks 时请记住这些约束：

* 命令 hooks 仅通过 stdout、stderr 和退出代码通信。它们无法触发 `/` 命令或工具调用。通过 `additionalContext` 返回的文本被注入为 Claude 作为纯文本读取的[系统提醒](https://code.claude.com/docs/zh-CN/glossary.md#system-reminder)。HTTP hooks 改为通过响应体通信。
* Hook 超时因类型而异。通过 `timeout` 字段（以秒为单位）按 hook 覆盖。
  * `command`、`http`、`mcp_tool`：10 分钟。Claude Code 对 `UserPromptSubmit`、`PreModelSwitch` 和 `PostModelSwitch` hooks 将此默认值降低到 30 秒，对 `MessageDisplay` 降低到 10 秒。
  * `prompt`：30 秒。
  * `agent`：60 秒。
  * [`SessionEnd`](https://code.claude.com/docs/zh-CN/hooks#sessionend) 任何类型的 hooks 共享 1.5 秒的预算。如果你的设置为每个 hook 设置了更长的 `timeout`，Claude Code 会将预算提高到匹配，最多 60 秒。
* `PostToolUse` hooks 无法撤销操作，因为工具已经执行。
* [`PermissionRequest`](https://code.claude.com/docs/zh-CN/hooks#permissionrequest) hook 在 Claude Code 即将向您请求权限时触发，或在它原本会自动拒绝无法显示提示的调用时触发。在带 `-p` 标志的[非交互模式](https://code.claude.com/docs/zh-CN/headless.md)中，它们在 [`dontAsk` 模式](https://code.claude.com/docs/zh-CN/permission-modes#allow-only-pre-approved-tools-with-dontask-mode)之外仍会运行，而没有 hook 做出决策且无其他方式可以应答的调用会被拒绝。
* `Stop` hooks 在 Claude 完成响应时触发，而不仅仅在任务完成时。它们不在用户中断时触发。API 错误触发 [StopFailure](https://code.claude.com/docs/zh-CN/hooks.md#stopfailure) 代替。
* 当多个 `PreToolUse` hook 返回 [`updatedInput`](https://code.claude.com/docs/zh-CN/hooks#pretooluse) 来改写工具的输入时，Claude Code 不会合并这些改写，也不保证工具使用哪一个改写运行。避免让多个 hook 改写同一工具的输入。

### Hooks 和权限模式

通过设置文件或插件的 `hooks/hooks.json` 中的 hook，您无法将限制放宽到超出权限规则允许的范围，而 hook 的 `"deny"` 在每种权限模式下都有效：

* **对 `"allow"` 仍然有效的限制**：返回 `"allow"` 的 hook 不会绕过设置中的拒绝规则，并且在以下情况下也无法抑制提示：
  * 从[网络路径](https://code.claude.com/docs/zh-CN/permissions.md#network-paths)读取
  * 标记为 [`requiresUserInteraction`](https://code.claude.com/docs/zh-CN/mcp#require-approval-for-a-specific-tool) 的 MCP 工具
  * 调用 [Agent SDK 应用标记为 `always_ask` 的工具](https://code.claude.com/docs/zh-CN/agent-sdk/custom-tools#require-approval-for-specific-tools)，除非[权限模式批准该调用](https://code.claude.com/docs/zh-CN/agent-sdk/typescript.md#permission-modes-that-can-approve-an-always_ask-call)
  * 在该设置会传达到 Claude Code 的会话中，[您的组织设置为 `ask`](https://code.claude.com/docs/zh-CN/mcp#organization-controls-on-connector-tools) 的连接器工具

在[计划模式](https://code.claude.com/docs/zh-CN/permissions.md#hook-approvals-in-plan-mode)中，如果您的 `PreToolUse` hook 对文件编辑返回 `"allow"`，您得到的提示、阻止或批准与没有 hook 时该编辑所得到的相同。要解除对 hook 批准的这一限制，请[批准计划](https://code.claude.com/docs/zh-CN/permission-modes.md#review-and-approve-a-plan)或切换到其他权限模式。

* **`"deny"` 在每种权限模式下都有效**：返回 `permissionDecision: "deny"` 的 hook 会阻止工具，即使在 `bypassPermissions` 模式或使用 `--dangerously-skip-permissions` 时也是如此。`PreToolUse` hook 在任何权限模式检查之前触发，在每个[权限模式](https://code.claude.com/docs/zh-CN/permission-modes.md)中，包括 `dontAsk`。这让您能够强制执行用户无法通过更改其权限模式来绕过的策略。

> [!NOTE]
>
> 如果您的机器上没有[托管设置](https://code.claude.com/docs/zh-CN/settings.md#check-what-your-organization-enforces)，并且您的会话未使用 Team 或 Enterprise 计划的 [`/login` 凭据](https://code.claude.com/docs/zh-CN/authentication#authentication-precedence)，那么 [mod](https://code.claude.com/docs/zh-CN/plugins/mods/overview.md) 可以批准被您的 `PreToolUse` hook 阻止的调用。改为使用 API 密钥或长期有效的 [`CLAUDE_CODE_OAUTH_TOKEN`](https://code.claude.com/docs/zh-CN/authentication#generate-a-long-lived-token) 运行的会话并未使用该登录，即使该登录已存储。
>
> 在计划模式中，仅当此类被阻止的调用为只读（因此不是文件编辑）时，mod 才能批准它。有关 mod 的批准无法越过您的规则和 hook 的情况，请参阅[使用 hook 扩展权限](https://code.claude.com/docs/zh-CN/permissions.md#precedence-among-rules-hooks-and-mods)。

### Hook 未触发

Hook 已配置但从不执行。

* 运行 `/hooks` 并确认 hook 出现在正确的事件下
* 对于 skill frontmatter 中的 hook，请调用该 skill 或让 Claude 调用它，然后再次运行 `/hooks`。`/hooks` 不会列出会话中尚未调用的 skill 中的 hook。如果 `/hooks` 仍未列出该 hook，请检查其他原因，例如[未能解析的 frontmatter](https://code.claude.com/docs/zh-CN/skills.md#skill-not-triggering) 或某项 [skill hook 的限制](https://code.claude.com/docs/zh-CN/hooks.md#limits-on-skill-hooks)
* 检查匹配器模式是否与工具名称完全匹配。匹配器区分大小写
* 验证你是否触发了正确的事件类型：`PreToolUse` 在工具执行前触发，`PostToolUse` 在之后触发。`PermissionRequest` hook 在 Claude Code 即将要求你获得权限时触发；有关非交互模式情况，请参阅[限制](#limitations)

### Hook 输出中的错误

你在成绩单中看到类似"PreToolUse hook error: ..."的消息。

* 你的脚本意外以非零代码退出。通过管道传递示例 JSON 来手动测试它：
  ```bash theme={null}
  echo '{"tool_name":"Bash","tool_input":{"command":"ls"}}' | ./my-hook.sh
  echo $?  # Check the exit code
  ```
* 如果你看到"command not found"，使用绝对路径或 `${CLAUDE_PROJECT_DIR}` 来引用脚本。为了完全避免 shell 引用，添加 `"args": []` 来切换到 [exec 形式](https://code.claude.com/docs/zh-CN/hooks.md#exec-form-and-shell-form)，它直接生成脚本而不使用 shell
* 如果你看到"jq: command not found"，安装 `jq` 或使用 Python/Node.js 进行 JSON 解析
* 如果消息包含 `blocking because Git Bash wasn't found`，说明您使用的是 Windows，Claude Code 未找到 Git Bash，并且失败或超时的 hook 阻止了该工具调用。有关修复方法，请参阅[在没有 Git Bash 的 Windows 上失败的 hook](https://code.claude.com/docs/zh-CN/hooks.md#failed-hooks-on-windows-without-git-bash)
* 如果通知显示 JSON 验证消息，你的 hook 的 stdout 解析为 JSON 但未通过架构验证。如果显示 JSON 解析消息，stdout 看起来像 JSON 对象但不是有效的 JSON。即使在退出 0 时也会发生两者。

  要修复解析失败，使用 JSON 编码器（如 `jq`）而不是字符串连接来构建有效负载，以便值内的引号和反斜杠被转义。参考的[退出代码输出](https://code.claude.com/docs/zh-CN/hooks.md#exit-code-output)部分涵盖了退出代码和 JSON 组合
* 如果脚本根本没有运行，使其可执行：`chmod +x ./my-hook.sh`

### `/hooks` 显示未配置 hooks

你编辑了设置文件但 hooks 不出现在菜单中。

* 文件编辑通常会自动拾取。如果几秒钟后它们还没有出现，文件监视器可能错过了更改：重新启动你的会话以强制重新加载。
* 验证你的 JSON 有效：不允许尾随逗号和注释
* 确认设置文件在正确的位置：`.claude/settings.json` 用于项目 hooks，`~/.claude/settings.json` 用于全局 hooks
* 如果菜单显示 `Only hooks from managed settings run here`，说明您的组织设置了 [`allowManagedHooksOnly`](https://code.claude.com/docs/zh-CN/settings-reference#allowmanagedhooksonly)。您的用户、项目和本地设置文件中的 hook 不会运行，也不会列出

### Stop hook 达到阻止上限

Claude 继续工作而不是停止，然后以警告结束该轮，表示 Stop hook 连续阻止了太多次。

当 Stop hook 连续阻止 8 次、且期间 Claude 没有进行任何工具调用时，Claude Code 会覆盖该 hook。您的 hook 脚本需要检查它是否已经触发过继续。从 JSON 输入中解析 `stop_hook_active` 字段，如果为 `true` 则提前退出：

```bash theme={null}
#!/bin/bash
INPUT=$(cat)
if [ "$(echo "$INPUT" | jq -r '.stop_hook_active')" = "true" ]; then
  exit 0  # Allow Claude to stop
fi
# ... rest of your hook logic
```

如果你的 hook 合理地需要超过八次迭代才能收敛，使用 [`CLAUDE_CODE_STOP_HOOK_BLOCK_CAP`](https://code.claude.com/docs/zh-CN/env-vars) 提高上限。

### Hook JSON 无效果

你的 hook 打印有效的 JSON，但决策没有生效，成绩单中没有出现错误。检查哪个原因适用：

* **JSON 前面有额外输出**：有其他内容先写入了 stdout，通常是您的 shell profile 中无条件的 `echo`，导致输出不再以 `{` 开头。请参阅 [JSON 之前的 shell profile 输出](#shell-profile-output-before-the-json)。
* **字段位于错误的层级**：将每个字段的位置与 [JSON 输出](https://code.claude.com/docs/zh-CN/hooks.md#json-output)格式进行比较。例如，`permissionDecision` 应位于 `hookSpecificOutput` 内部，而不是顶层。请参阅[字段位于错误的层级](#fields-at-the-wrong-level)。

#### JSON 之前的 shell profile 输出

Hook 在非交互式 shell 中运行，但 Git Bash 和某些配置（例如 `BASH_ENV` 指向 `~/.bashrc`）仍会加载您的 profile，profile 打印的任何内容都会先于 hook 的 JSON 进入 stdout：

```text theme={null}
Shell ready on arm64
{"decision": "block", "reason": "Not allowed"}
```

除非该 hook 是[异步](https://code.claude.com/docs/zh-CN/hooks.md#how-async-hooks-execute)的，否则 Claude Code 会将不以 `{` 开头的输出作为纯文本读取，因此您的 JSON 会被忽略。由于 hook 以 0 退出，会话记录中也不会显示错误。要检查是否为此原因，请使用 `claude --debug` 启动 Claude Code，触发该 hook，然后在[调试日志](https://code.claude.com/docs/zh-CN/hooks.md#debug-hooks)中搜索 `Hook output does not start with {`。要修复此问题，请包装 profile 中的 `echo` 语句，使其仅在交互式 shell 中运行：

```bash theme={null}
# 在 ~/.zshrc 或 ~/.bashrc 中
if [[ $- == *i* ]]; then
  echo "Shell ready"
fi
```

`$-` 变量包含 shell 标志，`i` 表示交互式。Hooks 在非交互式 shell 中运行，因此 echo 被跳过。

#### 字段位于错误的层级

当你的 hook 返回 `permissionDecision` 或 `additionalContext` 在顶级而不是在 `hookSpecificOutput` 内部时，JSON 仍然解析，Claude Code 忽略错误放置的字段而不报告错误。要查看它忽略了哪些字段，使用 `claude --debug` 启动 Claude Code 并在[调试日志](https://code.claude.com/docs/zh-CN/hooks.md#debug-hooks)中搜索 `Hook JSON output had unrecognized keys`。

### 检查 hook 的执行结果

按 `Ctrl+O` 打开会话记录视图，查找 hook 的结果：

* **成功**：您看不到任何内容，除非 hook 的 JSON 显示了某些内容，例如 `systemMessage` 或 Stop hook 反馈。
  * 要确认 hook 已运行，请检查其效果，例如被重新格式化的文件
* **阻止错误**：在大多数事件上，您会看到随阻止一起返回的消息，例如 `Blocked: rm commands are not allowed`。在少数事件上，例如 `ConfigChange` 和 `Elicitation`，您不会看到消息。[退出码 2](https://code.claude.com/docs/zh-CN/hooks.md#exit-code-2) 介绍了该消息的来源。
* **非阻止错误**：您会看到 `<hook name> hook error` 通知，其中包含简短说明，例如 `Failed with non-blocking status code:` 之后的 stderr 第一行，或 JSON 验证或解析消息。操作会继续进行。

要查询特定退出码和 stdout 对应的结果（包括各事件的例外情况），请参阅参考文档中的[退出码输出](https://code.claude.com/docs/zh-CN/hooks.md#exit-code-output)。

有关完整的执行详情，包括 hook 的退出码、stdout 和 stderr，请阅读[调试日志](https://code.claude.com/docs/zh-CN/hooks.md#debug-hooks)。使用 `claude --debug-file /tmp/claude.log` 启动 Claude Code 以写入已知路径，然后在另一个终端中运行 `tail -f /tmp/claude.log`。如果您启动时没有使用该标志，请在会话中运行 `/debug` 以启用日志记录并找到日志路径。

## 了解更多

* [Hooks 参考](https://code.claude.com/docs/zh-CN/hooks.md)：完整的事件架构、JSON 输出格式、异步 hooks 和 MCP 工具 hooks
* [安全考虑](https://code.claude.com/docs/zh-CN/hooks.md#security-considerations)：在共享或生产环境中部署 hooks 之前查看
* [Bash 命令验证器示例](https://github.com/anthropics/claude-code/blob/main/examples/hooks/bash_command_validator_example.py)：完整的参考实现
