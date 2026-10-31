> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# 在企业启动器后面运行 Claude Code

> 通过 CLAUDE_CODE_PROCESS_WRAPPER 或 processWrapper 设置，使用必需的启动器路由 Claude Code 从其自身二进制文件启动的进程，包括后台服务和每个代理视图会话。

某些组织要求工作站上的每个进程都通过强制启动器启动。启动器应用沙箱、网络控制或凭证注入，这些是公司安全态势所依赖的，而不通过它启动的二进制文件是策略违规。

`CLAUDE_CODE_PROCESS_WRAPPER` 通过您的启动器启动 Claude Code 从其自身二进制文件启动的每个进程：[后台服务](https://code.claude.com/docs/zh-CN/agent-view.md#the-supervisor-process)、它在 [agent view](https://code.claude.com/docs/zh-CN/agent-view.md) 中托管的每个会话，以及 Claude Code 在更新后的重新启动。将其设置为启动器的绝对路径，Claude Code 将使用 Claude Code 命令作为其参数运行启动器。

在您的 `PATH` 上包装 `claude` 命令的启动器无法到达这些进程，因为它们从二进制文件的直接路径启动，不查询 `claude`。

> [!NOTE]
>
> `CLAUDE_CODE_PROCESS_WRAPPER` 需要 Claude Code v2.1.208 或更高版本。早期版本忽略该变量并启动每个未包装的进程。等效的 [`processWrapper` 设置](https://code.claude.com/docs/zh-CN/settings-reference#processwrapper) 需要 v2.1.210 或更高版本。早期版本将其忽略为未知密钥，不应用启动器，并且不报告错误。
>
> 部署任一形式后，使用 [验证步骤](#set-up-the-launcher) 来确认运行的版本应用了它。

## 启动器覆盖的内容

设置 `CLAUDE_CODE_PROCESS_WRAPPER` 后，Claude Code 通过您的启动器启动以下每个进程：

* `claude agents` 和后台会话按需启动的后台服务。
* 每个代理视图行内的终端主机和 Claude Code 会话，包括服务保持就绪的热备用会话。
* 服务在更新或崩溃后重新生成的会话。
* Claude Code 执行的自身重新启动以完成更新安装，包括代理视图的重启以更新操作。
* [Remote Control](https://code.claude.com/docs/zh-CN/remote-control.md) 启动的会话进程。需要 Claude Code v2.1.210 或更高版本。
* [agent teams](https://code.claude.com/docs/zh-CN/agent-teams.md) 在 tmux 或 iTerm2 中启动的分割窗格队友会话。队友窗格是交互式的而不是后台进程，但 Claude Code 从其自身二进制文件启动它们，所以启动器覆盖它们。需要 Claude Code v2.1.210 或更高版本。

在 Windows 上，该变量被忽略：启动器契约取决于 `exec`，Windows 不支持。设置了该变量的 Windows 机器运行每个未包装的进程并继续工作，唯一的信号是 [debug log](https://code.claude.com/docs/zh-CN/troubleshooting.md) 中的警告。如果您的启动器策略涵盖 Windows，该变量在那里不满足它：在规划推出时将 Windows 机器计为未包装。

### 在启动器外启动的进程

以下进程不通过启动器启动：

* 您自己在终端中启动的会话，它运行的方式取决于您如何调用它。要覆盖这些会话，在 `PATH` 上较早的目录中放置一个名为 `claude` 的脚本，该脚本使用真实二进制文件运行您的启动器；不要替换托管符号链接。后台服务及其会话启动时不进行 `PATH` 查询，所以两个启动器不会在那里堆叠。
* 从 `claude-cli://` 深层链接打开的会话，它会在新终端中启动，而不经过您的启动器。要防止深层链接为新用户打开会话，请在该用户首次运行 Claude Code 之前将 [`disableDeepLinkRegistration`](https://code.claude.com/docs/zh-CN/settings-reference#disabledeeplinkregistration) 设置为 `"disable"`。对于已有[已注册处理程序](https://code.claude.com/docs/zh-CN/deep-links.md#registration-and-supported-platforms)的用户，深层链接仍会继续打开会话，因为该设置不会移除处理程序。
* `--worktree` 与 `--tmux` 结合执行的重新启动：终端多路复用器启动该窗格，而不是 Claude Code 的二进制文件。
* [Claude in Chrome](https://code.claude.com/docs/zh-CN/chrome.md) 注册的本机消息主机：浏览器启动它，而不是 Claude Code 的二进制文件。

### 进程监视器中的辅助进程名称

配置了启动器后，`ps` 和 Activity Monitor 不再显示后台辅助进程的 Claude Code 的 `claude bg-pty-host` 和 `claude bg-spare` 标签，因为启动器的 `exec` 重建了参数列表。丢失标签是副作用，而不是隐瞒：进程在其他方面保持不变，Claude Code 通过二进制路径识别自己的进程，从不通过显示名称。

## 设置启动器

**1. 编写启动器脚本**

在绝对路径（例如 `/opt/corp/launcher`）创建可执行脚本。Claude Code 使用完整的 Claude Code 命令作为其参数运行它，脚本必须以调用 `exec "$@"` 结尾，以便它用 Claude Code 替换自己：

```bash theme={null}
#!/bin/sh
# 您组织的设置：进入沙箱、应用
# 网络控制或注入凭证。
exec "$@"
```

使用 `chmod +x` 使其可执行。设置部分是启动器在 Claude Code 运行前必须做的任何事情；下面的 [启动器契约](#the-launcher-contract) 列出脚本必须遵循的规则。

> [!NOTE]
>
> 如果您之前用启动器替换了 `~/.local/bin/claude` 符号链接，请在同一更改中恢复原始符号链接。替换的符号链接会导致第一个包装的会话同时通过两个启动器启动后台服务，并将安装置于外部管理状态：`/doctor` 报告它，自动更新保留文件，旧版本的清理保持禁用，直到安装程序再次管理该路径。

**2. 在设置中设置 CLAUDE\_CODE\_PROCESS\_WRAPPER**

在设置文件的 `env` 块中设置变量，以便分离的后台服务继承它。shell `export` 不够：后台服务按需启动，超过您的 shell 生命周期，并且从不重新读取 shell 配置文件。

对于一台机器，将其添加到 `~/.claude/settings.json`。要将其部署到组织中的每台机器，请在 [托管设置](https://code.claude.com/docs/zh-CN/managed-settings.md) 中放置相同的块：

```json theme={null}
{
  "env": {
    "CLAUDE_CODE_PROCESS_WRAPPER": "/opt/corp/launcher"
  }
}
```

当多个源设置变量时，托管设置值覆盖 `~/.claude/settings.json` 和 shell 中导出的值，因此用户无法将自生成指向不同的启动器。

[`processWrapper` 设置](https://code.claude.com/docs/zh-CN/settings-reference#processwrapper) 与命名的顶级设置键具有相同的值。当您的组织将设置作为单个键而不是 `env` 块推送时，请设置它。`processWrapper` 设置需要 Claude Code v2.1.210 或更高版本。以下设置文件通过该键设置相同的启动器：

```json theme={null}
{
  "processWrapper": "/opt/corp/launcher"
}
```

当两者都设置时，`CLAUDE_CODE_PROCESS_WRAPPER` 优先。

因为 `processWrapper` 是一个命名设置，通过 [远程托管设置](https://code.claude.com/docs/zh-CN/managed-settings.md#delivery-mechanisms) 提供它的组织会在 [安全批准对话框](https://code.claude.com/docs/zh-CN/server-managed-settings.md#security-approval-dialogs) 上看到它与运行管理员提供的可执行文件的其他设置一起列出。

项目和本地设置无法配置启动器。提交到仓库的文件不能在机器上的每个 Claude Code 进程前放置二进制文件，因此 Claude Code 在 `.claude/settings.json` 或 `.claude/settings.local.json` 中忽略 `CLAUDE_CODE_PROCESS_WRAPPER`，并在 [调试日志](https://code.claude.com/docs/zh-CN/troubleshooting.md) 中发出警告，并且从不从这些文件中读取 `processWrapper` 键。

**3. 重新启动后台服务和您的会话**

运行的后台服务和任何打开的 `claude` 会话在启动时读取变量一次，因此它们继续启动未包装的进程，直到重新启动。运行 `claude daemon stop --any` 停止按需服务。下一个需要它的命令（例如 `claude agents`）会启动一个包装的服务。然后重新启动您打开的 `claude` 会话。

在您无法手动重新启动的机器上，设置推送后启动的第一个会话会自动停用剩余的未包装按需服务。没有新会话启动的机器会保持其未包装的后台服务，直到有新会话启动。

**4. 验证**

在会话中运行 `/status`：Self-exec 条目显示已解析的启动命令，并在运行的后台服务与其不匹配时发出警告。`claude daemon status` 从 shell 打印相同的信息，包括在您取消设置变量后，当 `/status` 不再显示条目时。

## 启动器契约

当启动器无法运行时，Claude Code 拒绝启动进程，而不是启动它未包装。在 Windows 上，[该变量被忽略](#what-the-launcher-covers)并且进程启动未包装。Claude Code 对脚本持有这些规则：

* **以 `exec "$@"` 结尾。** 启动器分叉子进程并退出会留下孤立的 Claude Code 进程，后台服务无法跟踪。代理视图用命名启动器的消息标记此类会话失败，服务收割启动器留下的内容。
* **不要重新排序、吸收或添加 Claude Code 的参数。** 移除排在最前面的启动器[自身参数](#pass-arguments-to-the-launcher)，并将其余参数按原样传递给 `exec`。
* **将每个继承的环境变量传递给 `exec`。** 添加变量（例如注入的凭证）很好；删除继承的变量不是。
  * 每个会话的身份验证令牌、模型和提供程序选择以及 `CLAUDE_CODE_PROCESS_WRAPPER` 本身都在继承的环境中传输，因此从允许列表重建它的启动器会破坏它启动的会话，`/status` 报告启动器不匹配。
  * 如果启动器必须进入重置环境的命名空间或沙箱，请在其内部逐字重新导出继承的环境。
* **每次启动器运行时，都要在大约三秒内到达 `exec`。** 后台会话在产生第一个输出之前可能会连续运行启动器三次。请以懒加载方式或从缓存中执行单点登录交换等缓慢工作，并为任何需要等待网络的步骤设置单独的时间限制。
* **容忍从内部调用自己。** Claude Code 将启动器应用于每个嵌套的自生成，因此获取独占资源的启动器必须检测它是否已持有它。
* **不要在 Claude Code 启动前写入终端或标准输出。** 在 `exec` 前打印的任何内容都会在会话在初始化前终止时被报告为崩溃原因。标准输出上的文本还会传给读取 Claude Code 输出的程序，例如解析 `claude -p` 结果的脚本。
* **不要依赖参数的书写方式。** 标志的值可能作为独立参数传入（`--flag value`），也可能与标志连在一起（`--flag=value`）。标志使用哪种形式可能会在不同版本之间发生变化。
* **从启动器启动时所在的目录运行 `exec`。** Claude Code 在其启动时所在的目录中工作，因此更改目录的启动器可能会在错误的项目中启动会话。如果启动器必须进入重置工作目录的命名空间或沙箱，请在其内部、`exec` 之前切换回起始目录。
* **不要读取标准输入。** 在 `exec` 之前读取标准输入的命令（例如要求输入密码的命令）可能会等待永远不会到来的输入，导致会话卡住。它还可能取走本应传给 Claude Code 的输入，之后会话可能在缺少该输入的情况下运行且不报告任何错误。请将此类命令的输入重定向自 `/dev/null`。
* **通过绝对路径调用其他程序。** 启动器启动时的 `PATH` 可能不包含您的 shell 配置文件所添加的目录。

### 启动器值的格式

`CLAUDE_CODE_PROCESS_WRAPPER` 和 `processWrapper` 设置采用相同的格式。对于大多数启动器，该值是脚本的绝对路径，例如 `/opt/corp/launcher`。

要传递启动器自己的参数，请在路径后写入它们。Claude Code 将值解析为参数列表，而不是 shell 命令：

* 空白字符分隔各个 token，双引号将包含空格的 token 组合在一起。单引号是普通字符，因此 `'a b'` 是两个 token。
* 以 `[` 开头的值被读取为 JSON 字符串数组，例如 `["/opt/corp/launcher", "--profile", "cc"]`。
* Shell 语法不起作用：没有变量扩展或通配符，未引用的运算符（例如 `;`、`|`、`&` 或 `$(`）被拒绝为配置错误，而不是重新解释。

当无法使用该值时，Claude Code 拒绝启动受影响的进程并[报告原因](https://code.claude.com/docs/zh-CN/errors.md#claude_code_process_wrapper-launcher-errors)。

在设置文件中，请将数组形式写在 JSON 字符串内并转义其中的引号：`"[\"/opt/corp/launcher\", \"--profile\", \"cc\"]"`。

### 向启动器传递参数

在[该值](#format-of-the-launcher-value)中将启动器自身的参数写在其路径之后，并在脚本中于 `exec` 调用之前将其移除。这些参数会最先到达脚本，排在 Claude Code 命令之前。以下脚本针对值 `/opt/corp/launcher --profile cc` 编写，它保存配置档案名称，然后使用 `shift` 移除这两个参数：

```bash theme={null}
#!/bin/sh
if [ "$1" = "--profile" ]; then
  profile="$2"
  shift 2
fi
# Your organization's setup for "$profile".
exec "$@"
```

使用[验证步骤](#set-up-the-launcher)查看 Claude Code 根据该值解析出的启动命令。

## 与 `CLAUDE_CODE_SHELL_PREFIX` 的关系

`CLAUDE_CODE_PROCESS_WRAPPER` 包装 Claude Code 自己的进程，并将命令作为单独的 argv 令牌传递给启动器以 `exec`。[`CLAUDE_CODE_SHELL_PREFIX`](https://code.claude.com/docs/zh-CN/env-vars) 包装 Claude 代表您运行的 shell 命令，例如 Bash 工具调用、hooks 和启动 stdio MCP 服务器的命令，并将每个作为单个 shell 引用的字符串在 `$1` 中传递给包装器以重新评估。为一个编写的启动器不能作为另一个工作。

## 相关资源

* [Agent view](https://code.claude.com/docs/zh-CN/agent-view.md)：启动器覆盖的后台会话和监督进程
* [Environment variables](https://code.claude.com/docs/zh-CN/env-vars.md)：`CLAUDE_CODE_PROCESS_WRAPPER` 参考条目
* [Managed settings](https://code.claude.com/docs/zh-CN/permissions.md#managed-settings)：在整个车队中传递 `env` 块
* [Launcher error reference](https://code.claude.com/docs/zh-CN/errors.md#claude_code_process_wrapper-launcher-errors)：拒绝消息和如何恢复
