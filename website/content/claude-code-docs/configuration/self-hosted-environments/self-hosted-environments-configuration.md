> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# 在自托管环境中自定义会话

> 使用包装脚本在自托管环境会话中自定义每个会话的凭据、生命周期 hook 和按需运行程序生成。

> [!NOTE]
>
> 自托管环境在 Team 和 Enterprise 计划中处于公开测试阶段；[Owner](https://code.claude.com/docs/zh-CN/cloud-environments.md#organization-shared-environments) 通过在 [**Cloud environments** 管理页面](https://claude.ai/admin-settings/cloud-environments) 上打开 **Allow self-hosted environments** 来启用它们。本页面假设您已有一个正常运行的运行程序；有关设置，请参阅[快速入门](https://code.claude.com/docs/zh-CN/self-hosted-environments-quickstart.md)，有关集群部署方案，请参阅[部署到生产环境](https://code.claude.com/docs/zh-CN/self-hosted-environments-deploy.md)。

[自托管环境](https://code.claude.com/docs/zh-CN/self-hosted-environments)在您自己的基础设施上运行 Claude Code [云端会话](https://code.claude.com/docs/zh-CN/claude-code-on-the-web.md)，由您部署的运行程序进程执行。在没有配置的情况下，该运行程序克隆会话的仓库，生成 Claude Code，然后进行清理。本页面适用于操作运行程序的平台工程师：它涵盖了当这些默认值不适用时的扩展点，从每个会话的凭据配置到完全替换检出。包装脚本和 hook 作为运行程序主机上的可执行文件运行，该主机是 Linux 或 macOS，本页面上的示例假设使用 POSIX shell。

本页面上的一些 hook 环境变量仍然使用 `pool`，例如 `CLAUDE_RUNNER_POOL_ID`；CLI 标志和环境变量名称使用 `environment`，例如 `--environment-secret-file`。

## 包装脚本

当每个会话需要运行器无法自行完成的设置时，使用包装脚本：为会话创建者配置作用域的短期凭据、导出特定于环境的密钥、准备语言工具链或围绕子进程应用资源限制。运行器每个会话启动一次您的包装脚本，而不是 Claude Code 二进制文件。通过 `exec` 进入 `$CLAUDE_RUNNER_CLAUDE_BIN`（运行器自己的二进制文件）来结束包装脚本，以便信号和退出码正确传播。

启动运行器时，使用 `--exec-path` 或 `SELF_HOSTED_RUNNER_EXEC_PATH` 指向包装脚本，并按照[快速入门](https://code.claude.com/docs/zh-CN/self-hosted-environments-quickstart.md#set-up-manually)中的说明替换 `<writable-dir>`：

```bash theme={null}
claude self-hosted-runner --environment-secret-file /etc/claude/environment-secret --base-dir '<writable-dir>' --exec-path /etc/claude/session-wrapper.sh
```

运行器在包装脚本的环境中设置以下内容：

| 变量 | 描述 |
| :- | :- |
| `CLAUDE_CODE_SESSION_ACCESS_TOKEN` | 会话 JWT，前缀为 `sk-ant-cc-`。其 `act` 声明标识会话创建者，并在创建会话的使用入口记录了创建者电子邮件时包含该电子邮件。该值是生成时的令牌；刷新通过子进程的 stdin 到达，因此包装脚本只看到初始值。请参阅 [Verify session identity](https://code.claude.com/docs/zh-CN/self-hosted-environments-identity.md)。 |
| `CCR_SESSION_ACCOUNT_EMAIL` | 会话创建者的电子邮件，由运行器从令牌的 `act.email` 声明中预先提取，无需签名验证。适合用于标记，例如提交 trailer。当电子邮件控制凭据发放时，请改为验证令牌并从中读取声明。请参阅 [Provision credentials scoped to the session creator](#provision-credentials-scoped-to-the-session-creator)。当令牌不包含创建者电子邮件时未设置，例如在由您组织的服务身份创建的会话中。视为个人可识别信息。 |
| `CLAUDE_RUNNER_CLIENT_PLATFORM` | 创建会话的客户端使用入口，例如 `web_claude_ai`、`desktop_app`、`ios`、`claude_code_cli` 或 `scheduled_trigger`。Anthropic 在会话创建时记录该值一次，因此包装脚本和每个生命周期 hook 都看到相同的值。仅将其用于采用分析和标记，不用作授权信号。当会话没有记录或识别的使用入口时未设置。需要 Claude Code v2.1.229 或更高版本。 |
| `CLAUDE_RUNNER_CLAUDE_BIN` | 运行器自己的 Claude Code 二进制文件的绝对路径。使用 `exec "$CLAUDE_RUNNER_CLAUDE_BIN" "$@"` 结束您的包装脚本，以移交到固定的二进制文件，而无需硬编码安装路径。 |
| `CLAUDE_CODE_REMOTE_SESSION_ID` | 会话 ID，采用标记的 `cse_...` 形式。这与[生命周期 hook](#lifecycle-hooks)以 `session_...` 形式在 `CLAUDE_RUNNER_SESSION_ID` 中看到的是同一个会话；UUID 变量在两者之间匹配，将 `cse_` 前缀替换为 `session_` 会产生会话 URL 中显示的 ID。 |
| `CLAUDE_CODE_REMOTE_SESSION_UUID` | 相同的会话 ID，采用规范 UUID 形式，供以 UUID 作为键的系统使用。 |
| `CLAUDE_CODE_REMOTE_SLACK_THREAD_URL` | 对于属于某个 Slack 线程的 [Claude Tag](https://claude.com/docs/claude-tag/overview) 会话，为该线程的链接。其他会话未设置此变量，线程会话也可能未设置。 |
| `CLAUDE_CODE_REMOTE_SLACK_THREAD_TS` | 对于属于某个 Slack 线程的 Claude Tag 会话，为该线程的 Slack 时间戳，例如 `1700000000.000100`。可能未设置，也可能在 `CLAUDE_CODE_REMOTE_SLACK_THREAD_URL` 未设置时被设置，因此请分别检查每个变量。 |
| `CLAUDE_CODE_REMOTE_SLACK_CHANNEL_ID` | 对于 Claude Tag 会话，为该会话所服务的、或其线程所在的 Slack 频道或私信的 ID，例如 `C0123ABCDE`。Slack 时间戳只在单个频道内标识一条消息，因此请将此 ID 与 `CLAUDE_CODE_REMOTE_SLACK_THREAD_TS` 一起记录。其他会话未设置此变量，Claude Tag 会话也可能未设置。 |
| `CLAUDE_SESSION_INGRESS_TOKEN_FILE` | 绝对路径，指向保存当前会话 JWT 的按会话文件，在令牌刷新时保持最新。Shell 子进程在下载用户添加到会话的附件时从中读取其 `Authorization` 标头。`exec` 自动保留该变量；重建子进程环境的包装脚本必须携带该变量，否则附件下载会无声地停止工作。 |
| `CLAUDE_CONFIG_DIR` | 按会话 Claude 配置目录，在会话启动时从运行器在启动时捕获的运行器主机配置快照中写入；请参阅 [Permissions and tool approval](#permissions-and-tool-approval)。此处的写入仅限于此会话。除非您使用 [`--remove-session-state`](https://code.claude.com/docs/zh-CN/self-hosted-environments-reference#runner-cli-flags) 启动运行器，否则会话结束后该目录仍会保留在 `<base-dir>/_sessions/` 下；请参阅 [Reuse a pre-warmed checkout](https://code.claude.com/docs/zh-CN/self-hosted-environments-deploy.md#reuse-a-pre-warmed-checkout)。 |
| `CLAUDE_CODE_REMOTE_MCP_CONFIG` | 指向 `CLAUDE_CONFIG_DIR` 中某个文件的路径，该文件保存 Anthropic 控制平面为此会话交付的 MCP 服务器定义。如果您的包装脚本重建子进程的环境，请携带此变量。如果缺少此变量，当控制平面更新交付的服务器时，子进程会关闭您的包装脚本通过 [`--mcp-config`](https://code.claude.com/docs/zh-CN/cli-reference#cli-flags) 添加但不在更新中的任何 stdio、HTTP 或 SSE 服务器。需要 Claude Code v2.1.299 或更高版本。 |
| `ANTHROPIC_BASE_URL` | 子进程将使用的 API 基础 URL，由控制平面按会话交付，通常为 `https://api.anthropic.com`。不要覆盖它：会话的推理凭据是 Anthropic 颁发的 OAuth 令牌，其他提供者不接受。 |
| `CLAUDE_CODE_OAUTH_TOKEN` | 子进程用于模型推理的短期 OAuth 访问令牌，生命周期约为 30 分钟。运行器在过期前重新生成它，并通过子进程的 stdin 交付轮换，因此不 [keep stdin attached](#keep-stdin-and-file-descriptor-3-attached) 的包装脚本只看到初始值。不要依赖您的组织 IP 允许列表来限制此令牌的使用：将其视为持有者凭据，如果泄露，大约 30 分钟内仍可使用，不要记录它、写入磁盘或在会话容器外转发它。 |
| `CLAUDE_RUNNER_ALIVE_EVERY_MS` | 当[轮次在子进程初始化之前开始](https://code.claude.com/docs/zh-CN/self-hosted-environments-reference.md#session-startup-timeout)时，运行器通过此变量设置子进程报告自身存活的频率（以毫秒为单位）。如果您的包装脚本重建子进程的环境，请携带此变量，否则运行器可能会在该轮次期间释放会话并记录 `no child output for <n> — releasing`。需要运行器和子进程均为 v2.1.309 或更高版本。 |

包装脚本还继承子进程的其余托管环境，包括任何服务器提供的环境变量。`exec` 自动传播所有内容；如果您的包装脚本以其他方式生成子进程，请转发完整环境。

您可以在包装脚本或 [`command` hook](#command) 中读取 `CLAUDE_CODE_REMOTE_SLACK_*` 变量，也可以在会话运行的内容中读取它们，例如 shell 命令、git 钩子和 Claude Code hook。`checkout`、`post-session` 和 `spawn-runner` hook 不会接收它们。

### 为可能未设置的变量提供默认值

如果您的脚本使用 `set -u`，请使用默认值展开 `CCR_SESSION_ACCOUNT_EMAIL`、`CLAUDE_RUNNER_CLIENT_PLATFORM`、`CLAUDE_CODE_REMOTE_SLACK_THREAD_URL`、`CLAUDE_CODE_REMOTE_SLACK_THREAD_TS` 和 `CLAUDE_CODE_REMOTE_SLACK_CHANNEL_ID`，例如 `"${CCR_SESSION_ACCOUNT_EMAIL:-}"`。它们都可能未设置，而在 `set -u` 下，Bash 在展开未设置的变量时会以 `unbound variable` 停止。

### 为脚本展开的参数和变量加引号

为运行器传递给您的包装脚本或 [`command` hook](#command) 的每个参数和变量加引号。某些值包含 shell 会处理的字符，因为它们包含 JSON、完整的句子或[权限规则](https://code.claude.com/docs/zh-CN/permissions.md#permission-rule-syntax)，例如 `Bash(npm run test *)`。

* **参数**：以 `"$@"` 的形式传递它们
* **变量**：为每个变量加引号，如 `"${CLAUDE_CODE_REMOTE_SLACK_THREAD_URL:-}"`。例如，Slack 线程链接可能包含 `?` 和 `&`。
* **`eval` 和 `sh -c` 字符串**：不要将值替换到 `eval` 或 `sh -c` 运行的字符串中，即使在引号内也不行。应让该字符串引用该变量。

### 保持 stdin 和文件描述符 3 的连接

子进程的 stdin 是运行器的控制通道。令牌轮换和会话结束信号在其上到达。运行器还在文件描述符 3 上打开一个管道，并从中读取子进程的活动信号以驱动空闲和启动超时。普通的 `exec "$CLAUDE_RUNNER_CLAUDE_BIN" "$@"` 自动保留两者。

如果您的包装脚本使用裸 `&` 在后台运行子进程，它会切断子进程的 stdin。会话看起来健康，直到初始 OAuth 令牌的大约 30 分钟生命周期过期，然后每个使用该令牌的 API 调用都失败，出现 `401 authentication_error`。如果您的包装脚本必须在后台运行子进程，例如保持拆卸陷阱活跃，请在文件描述符 9 上保存 stdin 并显式重新连接它：

```bash theme={null}
exec 9<&0   # the runner uses 3, and from v2.1.309 reserves 4 and 5
"$CLAUDE_RUNNER_CLAUDE_BIN" "$@" <&9 9<&- &
CHILD=$!
trap 'teardown' EXIT
wait "$CHILD"
```

您可以重定向子进程的 stdout。请保持文件描述符 3 和 stderr 连接到运行器：

* **文件描述符 3**：将子进程的活动信号传送给运行器。不要在包装脚本中关闭或重用它。
* **文件描述符 4 和 5**：Claude Code v2.1.309 或更高版本上的运行器会为子进程保留它们。不要在包装脚本中关闭或重用它们。如果您的包装脚本使用了其中任何一个（例如用于保存 stdin），请将该用途改为 6 到 9 之间的描述符，早期版本同样不会占用这些描述符。
* **stderr**：当包装脚本或子进程以非零状态退出时，运行器会将 stderr 的最后几行发布到会话中，并在其自身日志中打印这些行。会话的用户会看到这些行，因此不要将密钥打印到 stderr，并在部署包装脚本之前移除 `set -x`。如果您重定向 stderr，会话仍会运行，但运行器仅以退出码报告失败。

### 透传系统提示词标志

Anthropic 控制平面为会话发送的系统提示词和追加系统提示词以文件路径的形式（而不是内联文本）到达您的包装脚本。运行器将每个提示词写入会话配置目录 `CLAUDE_CONFIG_DIR` 中的文件，并在您的包装脚本接收的参数中传递其路径，形式为 [`--system-prompt-file <path>` 或 `--append-system-prompt-file <path>`](https://code.claude.com/docs/zh-CN/cli-reference#system-prompt-flags)。

Claude Code v2.1.281 或更高版本上的运行器以文件形式传递提示词。在 v2.1.281 之前，运行器以 `--system-prompt <text>` 和 `--append-system-prompt <text>` 的形式传递它们。

在您的包装脚本或 [`command` hook](#command) 中，按如下方式处理这些标志：

* **透传它们**：使用 `exec "$CLAUDE_RUNNER_CLAUDE_BIN" "$@"` 结束包装脚本，这会将文件标志与其他所有参数一起转发。不要丢弃或改写它们。如果会话丢失了某个提示词文件标志，它将在没有控制平面为其发送的指令的情况下运行。
* **在 v2.1.281 或更高版本的运行器上，您追加的文件标志会替换服务器的标志，而不会叠加**：每个提示词文件标志只接受单个值，Claude Code 保留最后一次出现的值，因此如果您在 `"$@"` 之后追加 `--append-system-prompt-file <path>`，您文件的内容将替换服务器追加的指令。要在服务器指令之上添加指令，请将其放入运行器镜像的 `CLAUDE.md` 中，运行器会将其[植入每个会话的用户级配置](#how-each-session’s-config-is-assembled)。

#### 修复会话启动时的 `Argument list too long`

在 Linux 上，如果您将控制平面的提示词和您自己的文本合并为一个 `--append-system-prompt` 参数，一旦该参数达到 128 KB，运行器就会记录如下一行日志：

```text theme={null}
Session <id> finished: failed — reason: child exited 126: /opt/wrapper.sh: line 9: /usr/local/bin/claude: Argument list too long
```

Linux 将每个单独的参数限制为 32 个内存页，在通常的 4 KB 页大小下即 128 KB。`getconf ARG_MAX` 给出的数值（通常为 2 MB）是另一项限制，针对的是所有参数和环境的总和。

在 Claude Code v2.1.283 或更高版本上，请将文件标志保留在 `"$@"` 中，并仅将您自己的文本作为 `--append-system-prompt` 传递。Claude Code 会[同时使用两者](https://code.claude.com/docs/zh-CN/cli-reference.md#system-prompt-flags)，先是控制平面的提示词，然后是您自己的文本：

```bash theme={null}
exec "$CLAUDE_RUNNER_CLAUDE_BIN" "$@" --append-system-prompt "$(cat /opt/team-instructions.md)"
```

在 Linux 上，如果您自己的文本达到 128 KB，请将其移到运行器镜像的 `CLAUDE.md` 中，因为它仍然是一个参数。

在 v2.1.281 或 v2.1.282 上，该命令会以 `Cannot use both --append-system-prompt and --append-system-prompt-file` 退出。在更早的版本上，您的文本会替换控制平面的提示词，且不报错。无论哪种版本，请先更新，或将您自己的文本移到 `CLAUDE.md` 中。

### 配置作用域限定为会话创建者的凭据

使用 `decode-token` 子命令从会话 JWT 读取声明。它从参数、`CLAUDE_CODE_SESSION_ACCESS_TOKEN` 或 stdin 读取令牌，按该顺序；请参阅 [Verify the token inside the session](https://code.claude.com/docs/zh-CN/self-hosted-environments-identity.md#verify-the-token-inside-the-session) 了解它检查的内容。下面的示例解码创建者身份，将其交换为短期 AWS 凭据，并 exec 进入 Claude Code：

```bash theme={null}
#!/bin/bash
# Key on the stable Anthropic user ID and require a human creator.
CREATOR_SUB=$("$CLAUDE_RUNNER_CLAUDE_BIN" self-hosted-runner decode-token \
  | jq -re '.act.sub // "" | select(startswith("user:"))') \
  || { echo "decode-token: verification failed or no human creator" >&2; exit 1; }

creds=$(your-sts-helper assume-role --subject "$CREATOR_SUB") \
  || { echo "credential exchange failed" >&2; exit 1; }
eval "$creds"

exec "$CLAUDE_RUNNER_CLAUDE_BIN" "$@"
```

在提取的声明控制身份验证决策时，使用 `jq -re` 而不是 `jq -r`，以便缺失的声明以非零状态退出，而不是将字面字符串 `null` 传递给下游。由组织服务身份（例如机器人和 Agent 会话）创建的会话携带 `agent:` 主题而不是 `user:`，因此此示例拒绝它们；如果您的环境为这些会话提供服务，请明确决定包装脚本是否为它们回退到默认凭据，而不是退出。当您的凭据交换改为需要电子邮件时，读取 `.act.email` 并处理其缺失：令牌仅在创建会话的使用入口记录了电子邮件时才携带它，[CLI 分派的会话](https://code.claude.com/docs/zh-CN/self-hosted-environments-testing.md#run-the-test-loop) 可能缺少它。有关完整的声明参考和来自运行器外部服务的验证，请参阅 [Verify session identity](https://code.claude.com/docs/zh-CN/self-hosted-environments-identity.md)。

## 生命周期钩子

生命周期钩子用您自己的脚本替换运行器按会话管道的阶段。使用 `--hooks-dir <path>` 或 `SELF_HOSTED_RUNNER_HOOKS_DIR` 将运行器指向钩子目录。运行器查找具有众所周知名称的可执行文件；任何不存在的钩子都会回退到内置行为，因此您只需编写需要的钩子。钩子以运行器自己的权限运行，会话子进程共享该 UID，因此请以只读方式挂载钩子目录，或将其烘焙到镜像中，以便会话代码无法修改它；请参阅 [hardening section](https://code.claude.com/docs/zh-CN/self-hosted-environments-deploy.md#harden-your-deployment)。

这些钩子不同于 [Claude Code hooks](https://code.claude.com/docs/zh-CN/hooks.md)，后者在会话内运行；生命周期钩子在运行器上运行，围绕会话。

### checkout

每个仓库运行一次，代替运行器的内置克隆和获取。使用此 hook 从您通过 HTTPS 或 SSH 访问的读通镜像克隆、从存档为工作树设置种子，或应用按会话的 git 身份验证。运行器设置以下变量，并且可能设置表中未列出的其他 `CLAUDE_RUNNER_` 变量：

| 变量 | 描述 |
| :- | :- |
| `CLAUDE_RUNNER_REPO_URL` | 要克隆的存储库 URL，在应用任何 `--git-host-rewrite` 和 `--git-ssh-rewrite` 之后 |
| `CLAUDE_RUNNER_REPO_REF` | 要检出的修订版本，即会话所请求的形式：分支、标签、提交 SHA，或完整引用名称（例如 `refs/heads/<branch>` 或 `refs/pull/<number>/head`）。为空表示仓库的默认分支。 |
| `CLAUDE_RUNNER_CHECKOUT_PATH` | 必须留下工作树的绝对路径 |
| `CLAUDE_RUNNER_SESSION_ID` | 会话 ID，采用标记的 `session_...` 形式，用于日志记录和关联 |
| `CLAUDE_RUNNER_SESSION_UUID` | 相同的会话 ID，采用规范 UUID 形式 |
| `CLAUDE_RUNNER_API_BASE_URL` | Anthropic API 基础 URL，用于会话范围的调用 |
| `CLAUDE_RUNNER_CLIENT_PLATFORM` | 创建会话的客户端使用入口，例如 `web_claude_ai`、`desktop_app` 或 `ios`。当会话没有记录或可识别的使用入口时未设置，因此在 `set -u` 下请以 `"${CLAUDE_RUNNER_CLIENT_PLATFORM:-}"` 的形式引用它。需要 Claude Code v2.1.229 或更高版本。 |
| `CLAUDE_CODE_SESSION_ACCESS_TOKEN` | 会话访问令牌，用于会话范围的 API 调用 |
| `GIT_CONFIG_COUNT`、`GIT_CONFIG_KEY_n`、`GIT_CONFIG_VALUE_n` | 运行器为您的 hook 所运行的 git 固定的 Git 设置。[生命周期 hook 中的 Git 配置](#git-configuration-inside-lifecycle-hooks)对其进行了说明。需要 Claude Code v2.1.280 或更高版本。 |

脚本必须在 `CLAUDE_RUNNER_CHECKOUT_PATH` 处留下一个检出到所请求修订版本的工作树。分离的 HEAD 也可以，因为运行器会在其上创建会话的工作分支。

在您的 hook 返回后，运行器会验证 `CLAUDE_RUNNER_CHECKOUT_PATH` 包含 `.git`。如果您的 hook 具体化的是非 git 源（例如 Perforce 或解包的 tarball），请在运行器的环境中设置 `CLAUDE_RUNNER_SKIP_GIT_VERIFY=1` 以跳过该检查。基于 Git 的流程（例如工作分支创建和推送结果）需要 git 检出，因此请使用 [`post-session` hook](#post-session) 从非 git 树导出结果。

#### 在 hook 中获取 git 凭据

运行器不会将 git 凭据传递给 hook。`decode-token` 子命令在此处同样不可用，因为 `CLAUDE_RUNNER_CLAUDE_BIN` 未在 checkout-hook 环境中设置。请改为从会话的身份生成按会话的克隆凭据，或回退到主机自身的 git 身份验证：

* **按会话的克隆凭据**：使用标准 JWT 库，针对 `CLAUDE_RUNNER_API_BASE_URL` 下的 JWKS 端点验证 `CLAUDE_CODE_SESSION_ACCESS_TOKEN`，如[从您的服务验证令牌](https://code.claude.com/docs/zh-CN/self-hosted-environments-identity.md#verify-the-token-from-your-service)中所述。然后让您的凭据服务为令牌 `act` 声明中的身份发放短期克隆凭据。请以 `act.sub` 作为该凭据的键，不要依赖 `act.email`。
* **主机 git 身份验证**：使用主机已有的任何 git 身份验证，例如 SSH agent、凭据助手或 `.netrc`。

#### hook 失败时

当 hook 以非零状态退出，或以 0 退出但没有留下可用的检出时，hook 即为失败：

* **会话向其推送结果的仓库**：运行器会使会话失败，并在非零退出时将脚本 stderr 的末尾部分呈现给用户。请将原因以简短的一行写入 stderr，将其放在最后写入，然后以非零状态退出。请勿将机密信息打印到 stderr，并在部署 hook 之前移除 `set -x`。

  如果 git 的 `couldn't find remote ref` 出现在 hook stderr 的大约最后 4 KB 中，运行器会改为报告分支已被删除。此时会话的错误消息会说明该分支在远程上已不存在，并请用户恢复它。如果该分支确实存在，请参阅[会话失败并显示 `no longer exists on the remote` 或 `couldn't find remote ref`](https://code.claude.com/docs/zh-CN/self-hosted-environments-deploy#no-longer-exists-on-the-remote)。
* **会话仅从中读取的仓库**，例如添加到正在运行的会话中的仓库：运行器会跳过该仓库并继续处理其余仓库。运行器会向会话发布一个 `Skipped` 步骤，并删除 hook 在检出路径处留下的任何内容。

  当 `Skipped` 步骤提到 checkout hook 时，请在运行器日志中的 `[runner:warn] checkout hook failed` 行中查找失败详情。当该步骤表示某个源引用 `no longer exists on remote` 时，请参阅[会话失败并显示 `no longer exists on the remote` 或 `couldn't find remote ref`](https://code.claude.com/docs/zh-CN/self-hosted-environments-deploy#no-longer-exists-on-the-remote)。如果跳过后会话完全没有仓库，运行器仍然会使会话失败。

当 hook 成功时，运行器会在会话结束后删除检出路径。

### post-session

每个会话运行一次，在 Claude Code 子进程退出后和运行器拆卸工作区之前。此钩子是保存未提交工作的唯一机会：在 `--capacity` 高于 1 时，运行器在钩子返回后立即删除按会话工作树，在 `--capacity 1` 时重用的 [canonical clone](https://code.claude.com/docs/zh-CN/self-hosted-environments-deploy.md#reuse-a-pre-warmed-checkout) 在下一个会话启动时硬重置，因此未提交的跟踪更改在两条路径上都不会存活。典型用途是推送未提交更改的快照分支、存档日志或向您自己的系统发出会话结束事件。

钩子在每个会话结束时触发，其中生成了子进程，无论原因如何；下面的 `CLAUDE_RUNNER_EXIT_REASON` 值枚举了这些情况。当运行器突然终止时（例如 VM 抢占或断电）它无法触发；如果您需要针对突然终止的保证，请改为使用 Claude Code `PostToolUse` 钩子从会话内定期快照。运行器设置：

| 变量 | 描述 |
| :- | :- |
| `CLAUDE_RUNNER_SESSION_ID` | 会话 ID，采用标记的 `session_...` 形式 |
| `CLAUDE_RUNNER_SESSION_UUID` | 相同的会话 ID，采用规范 UUID 形式 |
| `CLAUDE_RUNNER_EXIT_REASON` | 会话如何结束；请参阅表下方的值 |
| `CLAUDE_RUNNER_WORKSPACE_PATHS` | 会话工作树的冒号分隔绝对路径。对于零存储库会话为空。 |
| `CLAUDE_RUNNER_DEBUG_LOG_PATH` | 会话的调试日志的路径，在钩子运行时仍在磁盘上 |
| `CLAUDE_RUNNER_API_BASE_URL` | Anthropic API 基础 URL，用于会话范围的调用 |
| `CLAUDE_RUNNER_CLIENT_PLATFORM` | 创建会话的客户端使用入口，例如 `web_claude_ai`、`desktop_app` 或 `ios`。当会话没有记录或可识别的使用入口时未设置，因此在 `set -u` 下请以 `"${CLAUDE_RUNNER_CLIENT_PLATFORM:-}"` 的形式引用它。需要 Claude Code v2.1.229 或更高版本。 |
| `CLAUDE_CODE_SESSION_ACCESS_TOKEN` | 会话访问令牌，用于会话范围的 API 调用 |
| `GIT_CONFIG_COUNT`、`GIT_CONFIG_KEY_n`、`GIT_CONFIG_VALUE_n` | 运行器为您的 hook 所运行的 git 固定的 Git 设置。[生命周期 hook 中的 Git 配置](#git-configuration-inside-lifecycle-hooks)对其进行了说明。需要 Claude Code v2.1.280 或更高版本。 |

`CLAUDE_RUNNER_EXIT_REASON` 采用四个值之一：

* `completed`：会话正常结束。Claude Code 进程正常退出，或在会话被存档或删除后自行退出。
* `failed`：Claude Code 进程崩溃，或在启动后设置失败。
* `interrupted`：运行器停止了会话，属于以下情况之一：
  * 运行器释放了会话以腾出插槽。
  * 会话在启动时超时。
  * 服务器将会话移出了此运行器。
  * 运行器的轮询在进程退出之前发现了存档或删除操作。
  * 运行器正在排空。
  * 会话超过了其 [`--kill-session-after-min`](https://code.claude.com/docs/zh-CN/self-hosted-environments-reference#runner-cli-flags) 限制。
* `abandoned`：为另一个运行器声称的会话保留。钩子目前在这种情况下不触发。

如果您将 hook 收据与[会话生命周期计数器](https://code.claude.com/docs/zh-CN/self-hosted-environments-reference.md#session-lifecycle-counter-semantics)进行比较，请预期某些 `interrupted` 收据在计数器中会计为 `completed`。计数器会将释放、启动超时、服务器移动，以及运行器轮询先发现的存档或删除计为 `completed`，因为运行器干净地交还了插槽。

钩子的退出状态永远不会影响会话结果；失败被记录并忽略。运行器在每个会话结束（包括运行器关闭）时等待最多 `--post-session-hook-timeout-sec`（默认 60 秒）。此示例将未提交的工作保存到救援分支：

```bash theme={null}
#!/usr/bin/env bash
set -u
export GIT_ALLOW_PROTOCOL=${GIT_ALLOW_PROTOCOL:-https:http:ssh}
IFS=':'
# -c overrides beat repo-local settings, blocking session-written fsmonitor,
# hook-path, and gpg-program config from executing code with the hook's
# privileges. -c commit.gpgsign=false also leaves these rescue commits
# unsigned under --configure-git.
# Repo-local credential.helper and pushurl still apply, and on a runner
# before v2.1.280 so does core.sshCommand; see the note below the script
# before you give this push a credential.
g() { git -c core.fsmonitor=false -c core.hooksPath=/dev/null \
        -c commit.gpgsign=false "$@"; }
for ws in $CLAUDE_RUNNER_WORKSPACE_PATHS; do
  cd "$ws" 2>/dev/null || continue
  [ -z "$(g status --porcelain 2>/dev/null)" ] && continue
  g add -A
  g commit -q -m "runner snapshot: $CLAUDE_RUNNER_SESSION_ID ($CLAUDE_RUNNER_EXIT_REASON)" || continue
  g push -q origin "HEAD:refs/heads/rescue/$CLAUDE_RUNNER_SESSION_ID" || true
done
```

脚本中的 `GIT_ALLOW_PROTOCOL` 行将 git 限制为 HTTPS、HTTP 和 SSH 远程。如果运行器的环境已经设置了自己的非空 `GIT_ALLOW_PROTOCOL` 列表，脚本会保留该列表。

hook 使用运行器主机上其自身环境中可用的任何 git 凭据进行推送。在[镜像中不含凭据的部署方式](https://code.claude.com/docs/zh-CN/self-hosted-environments-deploy.md#configure-git)下，包括内置克隆通过 Anthropic git 代理进行时，都没有可用凭据，因此请在推送前于 hook 内生成短期推送凭据：将 hook 在 `CLAUDE_CODE_SESSION_ACCESS_TOKEN` 中收到的会话令牌与您自己的令牌服务进行交换，并按照[验证会话身份](https://code.claude.com/docs/zh-CN/self-hosted-environments-identity.md)中的说明对其进行验证。

请将您的 hook 提供给 git 的任何凭据都视为会话可以获取的凭据，并在生成时使其只能完成此次推送。hook 中的 git 会读取会话可以写入的配置文件，而其中某个文件指定的凭据助手或过滤器驱动程序会以您的 hook 的权限运行。无论您指定哪个远程，这些文件中的设置还可能改变推送的目标位置。有关运行器在您的 hook 中固定的 git 设置以及留给这些文件决定的设置，请参阅[生命周期 hook 中的 Git 配置](#git-configuration-inside-lifecycle-hooks)。

#### 运行器释放会话时的钩子时序

已释放的会话可以在另一个运行器上恢复。在 v2.1.236 或更高版本的运行器上，会话在释放时所做的事情决定了它是否可以在此钩子完成前在另一个运行器上恢复：

* **在轮次后空闲，或在启动时超时**：运行器停止子进程并运行此钩子至完成。只有这样它才会释放会话。在钩子运行时发送的用户消息无法在钩子完成前在另一个运行器上恢复会话。
* **等待用户回答提示，例如权限提示**：运行器首先释放会话，然后运行此钩子。在钩子运行时发送的用户消息可以在钩子完成前在另一个运行器上恢复会话。

这适用于运行器释放会话的任何时候：在空闲超时、在 [`--retire-at`](https://code.claude.com/docs/zh-CN/self-hosted-environments-reference#runner-cli-flags) 时间，以及 在 v2.1.260 或更高版本的运行器上，在会话的 [`--kill-session-after-min`](https://code.claude.com/docs/zh-CN/self-hosted-environments-reference#runner-cli-flags) 限制。其轮次已结束且仅持有后台任务的会话在此处计为空闲。在 v2.1.236 之前，运行器在两种情况下都首先释放会话，然后运行此钩子。

在 `SIGTERM` 排空期间，运行器持有会话租约直到钩子完成；请参阅 [Shutdown timing](https://code.claude.com/docs/zh-CN/self-hosted-environments-deploy.md#shutdown-timing)。

### 生命周期 hook 中的 Git 配置

`checkout` 和 `post-session` hook 运行时，其环境中包含会话的访问令牌，而它们运行的 git 会读取会话可以写入的配置文件，例如 `~/.gitconfig` 和检出目录中的 `.git/config`。在任一 hook 运行之前，运行器会在 hook 的环境中以 `GIT_CONFIG_COUNT`/`GIT_CONFIG_KEY_n`/`GIT_CONFIG_VALUE_n` 对和 git 环境变量的形式设置 git 设置，包括以下各项。Git 将这些设置的优先级排在所有配置文件之上，并且它们仅适用于您的 hook 运行的 git，而不适用于会话自己的 git。启动时，运行器会打印一行 `[runner:git] lifecycle hooks:`，显示当前生效的钩子路径、允许的协议、gpg 程序和签名模式。需要 Claude Code v2.1.280 或更高版本。

* **Git 钩子**：除非您提供值，否则 `core.hooksPath` 为 `/dev/null`，因此 git 会跳过仓库 `.git/hooks` 中的钩子以及 `~/.gitconfig` 指定的任何钩子目录。要提供一个值，请在运行器的环境中将 `core.hooksPath` 导出为 `GIT_CONFIG_KEY_n`/`GIT_CONFIG_VALUE_n` 对。运行器还会从系统 git 配置中读取 `core.hooksPath`，并且仅当运行器的用户无法写入该文件、其指定的目录或其中的钩子文件时才使用它。当运行器忽略某个值时，启动时会有一行 `[runner:warn]` 指明该值及原因。
* **文件系统监视器**：`core.fsmonitor` 为空，因此 hook 中的 git 不会运行配置文件中指定的监视程序。
* **远程协议**：`GIT_ALLOW_PROTOCOL` 为 `https:http:ssh`。使用本地路径、`file://` URL 或 `git://` URL 的克隆、获取或推送会失败，并报错 `fatal: transport 'file' not allowed` 或 `fatal: transport 'git' not allowed`。
* **SSH 命令和凭据提示**：hook 中的 git 会忽略配置文件中的 `core.sshCommand` 和 `core.askPass`。要使用您自己的 SSH 命令，请在运行器的环境中设置 `GIT_SSH_COMMAND`。要使用凭据提示程序，请在其中设置 `GIT_ASKPASS`。会话会继承运行器的环境，因此这两个变量也会影响会话自己的 git。请勿在其中任何一个中放入凭据。
* **gpg 程序**：`gpg.program`、`gpg.openpgp.program`、`gpg.x509.program` 和 `gpg.ssh.program` 是运行器设置的路径，绝不会取自配置文件中的值。
* **提交签名**：使用 [`--configure-git`](https://code.claude.com/docs/zh-CN/self-hosted-environments-deploy#let-the-runner-configure-git) 时，您从 hook 中进行的提交会以会话身份签名。不使用该标志时，`commit.gpgsign` 和 `tag.gpgsign` 为 `false`。

要更改其中某项设置，请使用运行器的环境或在 hook 内使用 `git -c` 选项：

* **配置对**：您在运行器环境中导出的 `GIT_CONFIG_KEY_n`/`GIT_CONFIG_VALUE_n` 对会替换运行器为同一键设置的值。请从 `0` 开始为您的配置对编号，并将 `GIT_CONFIG_COUNT` 设置为配置对的数量。当计数所声明的最后一个配置对缺失时，运行器会忽略您的所有配置对，并在启动时记录一行 `[runner:warn]`。
* **Git 环境变量**：运行器会保留您在其环境中设置的 `GIT_ALLOW_PROTOCOL`、`GIT_SSH_COMMAND` 和 `GIT_ASKPASS`。
* **`git -c` 选项**：hook 内的 `git -c` 选项会覆盖 `GIT_CONFIG_KEY_n` 对，无论是运行器的还是您的。它不会更改 `GIT_ALLOW_PROTOCOL`、`GIT_SSH_COMMAND` 或 `GIT_ASKPASS`，git 会先于任何配置读取这些变量。

hook 中的 git 仍会从每个配置文件（包括会话可以写入的配置文件）中读取运行器未设置的所有设置，例如凭据助手、`url.*.insteadOf` 重写和过滤器驱动程序。这些文件之一中指定的凭据助手或过滤器驱动程序会以您的 hook 的权限作为程序运行，并且这些文件中的配置仍可能改变您的 hook 推送的目标位置，包括推送到您在命令行上传递的 URL。

在 v2.1.280 之前，运行器不设置这些设置中的任何一项，并且在 `--configure-git` 下，从 hook 中进行的提交会失败，除非 hook 传递了 `-c commit.gpgsign=false`。

### command

每个会话在检出后运行一次，代替内置子进程生成。钩子接收与 [wrapper script](#wrapper-scripts) 相同的环境，应该以相同的方式 `exec` 进入 `"$CLAUDE_RUNNER_CLAUDE_BIN"`。使用 `command` 钩子将所有自定义保留在一个钩子目录中；当包装脚本在其他地方时使用 `--exec-path`。如果也设置了 `--exec-path`，标志优先，`command` 钩子被忽略。

始终 `exec` 运行器自己的二进制文件，而不是 PATH 解析的 `claude`；否则您会破坏 [version pinning](https://code.claude.com/docs/zh-CN/self-hosted-environments-deploy.md#pin-the-version)。

## 按需运行器

您可以为每个会话启动一个运行器，而不是运行固定的队列。编排器是一个单独的、无状态的子命令，它轮询 Anthropic 以获取生成请求（每个没有可用运行器的排队会话一个），并为每个请求运行您的 `spawn-runner` hook。您的 hook 向您的平台提交工作负载：Kubernetes Job、EC2 实例、Nomad dispatch。

按需运行器改进了凭据卫生。在固定队列上，环境密钥存在于每个运行器主机上，这是运行用户会话的同一主机。使用编排器，环境密钥仅保留在编排器主机上，该主机从不运行用户代码；每个生成的运行器接收一个单次使用的工作单，恰好注册一个运行器，然后过期。

要启动编排器，请传递环境密钥和包含可执行 `spawn-runner` 脚本的 hook 目录：

```bash theme={null}
claude self-hosted-runner orchestrator \
  --environment-secret-file /etc/claude/environment-secret \
  --hooks-dir /etc/claude/hooks
```

编排器在轮询之间保持无状态，因此您可以针对同一环境运行两个或多个副本以实现可用性。每个生成请求由服务器端的恰好一个副本声称。所有副本必须使用相同的 `--expected-spawn-seconds` 值；请参阅 [hook 约定](#the-spawn-runner-hook)。

### spawn-runner hook

编排器为每个生成请求运行一次 `${hooks-dir}/spawn-runner`。hook 必须异步提交工作，不等待运行器启动，并在 `--hook-timeout`（默认 60 秒）内返回。hook 接收：

| 变量 | 描述 |
| :- | :- |
| `CLAUDE_RUNNER_WORK_ORDER_FILE` | 包含新运行器注册所用的已签名工作单 JWT 的临时文件的路径。hook 退出后删除。不要记录文件的内容。 |
| `CLAUDE_RUNNER_ORDER_ID` | 不透明的幂等性密钥，每个生成请求唯一，对 Kubernetes 资源名称安全。仅将订单 ID 用作您的配置器的去重密钥。 |
| `CLAUDE_RUNNER_SESSION_ID` | 此请求所针对的会话，采用会话 URL 中的 `session_...` ID 形式。该会话的每次重新请求都会重复此值，因此请将其用于日志记录和路由，而不要用作去重密钥。对于预热请求为空，预热请求在设置 [`--min-idle`](https://code.claude.com/docs/zh-CN/self-hosted-environments-reference#orchestrator-cli-flags) 时于任何特定会话之前启动待命运行器，因此不要假设变量已设置。 |
| `CLAUDE_RUNNER_SESSION_UUID` | 相同的会话 ID，采用规范 UUID 形式。对于预热请求为空。 |
| `CLAUDE_RUNNER_ATTEMPT` | 用于日志记录的按会话计数器。它不是重试次数，也不是请求次数。对于预热请求为 `0`，但针对某个会话的请求也可能携带 `0`。 |
| `CLAUDE_RUNNER_ORDER_SERVER_TIME` | 来自轮询响应的 HTTP `Date` 标头的服务器时间。当 hook 验证工作单 JWT 的 `exp` 时，与此值进行比较而不是本地时钟，以容忍时钟偏差。当网关省略标头时为空。 |
| `CLAUDE_RUNNER_POOL_ID` | 新运行器应加入的环境的 ID，采用 `ccpool_...` 形式 |
| `CLAUDE_RUNNER_ACCOUNT_ID` | 排队会话的帐户的标记 ID。请将其用于按帐户路由、配额或费用分摊，而不要用作去重密钥。当不可用时为空。 |
| `CLAUDE_RUNNER_ACCOUNT_EMAIL` | 排队会话的帐户的电子邮件。当不可用时为空。将电子邮件视为个人可识别信息，不要记录它。 |
| `CLAUDE_RUNNER_PRIMARY_REPO_URL` | 会话的第一个 git 源的 URL，用于路由到已预热该仓库的运行器。当会话没有 git 源时为空。 |
| `CLAUDE_RUNNER_PRIMARY_REPO_REVISION` | 会话的第一个 git 源的修订版本：分支、SHA、标签或完整引用名称。当未指定时为空。 |
| `CLAUDE_RUNNER_REPO_SOURCES` | 所有会话的 git 源的 `{url, revision}` 的 JSON 数组，用于根据辅助仓库进行路由的 hook。当没有源时为空。 |
| `CLAUDE_RUNNER_CORRELATION_ID` | 在会话创建时提供的关联 ID，回显以便 hook 可以将此工作单映射到创建会话的请求。当会话没有时为空。 |
| `CLAUDE_RUNNER_CLIENT_PLATFORM` | 创建会话的客户端使用入口，例如 `web_claude_ai`、`desktop_app`、`ios` 或 `scheduled_trigger`，用于采用分析。当会话没有记录或识别的使用入口时未设置，对于预热请求也未设置；使用 `[ -n "${CLAUDE_RUNNER_CLIENT_PLATFORM:-}" ]` 检查它，这在 `set -u` 下保持安全。 |

生成的运行器使用工作单代替环境密钥进行注册：

* **使用工作单启动它**：将 [`--environment-secret-file`](https://code.claude.com/docs/zh-CN/self-hosted-environments-reference#runner-cli-flags) 指向包含工作单 JWT 的文件，或将 `SELF_HOSTED_RUNNER_ENVIRONMENT_SECRET` 设置为 JWT 值。
* **在 hook 退出前复制 JWT**：编排器在 hook 退出后删除工作单文件，因此将 JWT 复制到您提交的工作负载中，例如生成的 Job 上的 Kubernetes Secret，而不是传递文件路径。
* **在生成的运行器上使用 `--capacity 1`**：会话绑定的工作单恰好注册一个绑定到该会话的运行器，因此更高的容量添加永远不会接收工作的插槽，运行器在启动时记录警告。
* **预热工作单注册未绑定**：待命运行器未绑定到会话，并像固定队列运行器一样声称排队的工作。

无论您的 hook 在哪个平台上配置资源，约定都有四条规则：

1. **在 `CLAUDE_RUNNER_ORDER_ID` 上保持幂等。** 相同请求的重新交付必须最多生成一个运行器。从订单 ID 派生确定性资源名称，让您的平台拒绝重复。不要改为以 `CLAUDE_RUNNER_SESSION_ID` 作为键。会话的每次重新请求都携带相同的会话 ID 和新的订单 ID。按会话 ID 命名或去重的工作负载，在第一个工作负载仍存在期间不会再次创建。
2. **不要重试工作负载。** 一个订单 ID 意味着最多创建一个工作负载。如果运行器从不注册，Anthropic 在 `--expected-spawn-seconds` 后使用新订单 ID 重新请求。
3. **使用退出码约定。** 以与结果相匹配的状态退出：

   * **退出 0**：已提交。
   * **退出 1，或在 `--hook-timeout` 结束时仍在运行**：可重试失败。会话退避并被重新提供。连续 10 次请求失败后，会话会像退出 2 或更高时一样被阻止。
   * **退出 2 或更高**：不可重试失败。会话被阻止再次生成，直到用户向其发送新消息，或 [Owner](https://code.claude.com/docs/zh-CN/cloud-environments.md#organization-shared-environments) 在环境的 **Activity** 标签中对其选择 **Retry**。

   会话退避期间，它在 **Activity** 标签中显示为 **Queued**，且没有生成错误，因此请在编排器的日志中查看 hook 的输出。会话被阻止后，会显示为 **Failed**。展开该会话即可查看生成错误，该错误取自 hook 的 stderr 的最后 512 个字符。请将可操作的错误写入 stderr，并且不要写入任何密钥。在 shell hook 中，请[保持暂时性失败可重试](#keep-transient-failures-retryable-in-a-shell-hook)。

   预热请求没有可失败的会话：编排器仅在本地记录非零退出，服务器在 `--expected-spawn-seconds` 租约到期后重新请求生成。
4. **将 `--expected-spawn-seconds` 设置为至少您从生成请求到运行器注册的 p99 时间。** 从编排器收到生成请求时开始计算，并包括在您的平台上等待容量的时间以及启动时间。此值是服务器端租约，工作单也随之过期，因此工作负载耗时更长的运行器无法注册。所有编排器副本必须使用相同的值。

hook 写入 stdout 或 stderr 的所有内容都出现在编排器的日志中，凭据会自动脱敏。如果会话保持排队，检查编排器的 `/healthz` 正文以获取队列计数，然后在 [**Cloud environments** 管理页面](https://claude.ai/admin-settings/cloud-environments) 上打开您的环境的 **Activity** 标签：在那里展开失败的会话以获取其生成错误，并选择 **Retry** 以重新请求它。

#### 修复保持 Queued 或显示 Failed 的重新打开的会话

如果重新打开的会话保持 **Queued** 或显示 **Failed**，或者您的 hook 在处理这些会话时超时，原因之一可能是 hook 按会话 ID 命名或去重工作负载。当会话的运行器退出后，其用户又向该会话发送消息时，该会话即被重新打开。例如，假设某个 hook 按如下方式构建每个工作负载的名称：

```bash theme={null}
name="runner-${CLAUDE_RUNNER_SESSION_ID}"
```

1. 在会话的第一次生成请求时，hook 以该名称创建工作负载，并启动一个运行器
2. 运行器退出（例如在 [`--release-idle-session-min`](#make-on-demand-runners-exit-when-idle) 之后），用户之后重新打开该会话
3. 会话的下一次生成请求携带相同的会话 ID 和新的订单 ID，因此 hook 构建出相同的名称
4. 如果您的平台仍保留着第一次请求的工作负载（因为在其运行器退出时没有任何机制[移除](#make-on-demand-runners-exit-when-idle)它），hook 不会创建新的工作负载，也不会启动运行器
5. 该会话之后的每次生成请求都以同样的方式结束

打开您自己的 hook，查看它在工作负载已存在时（例如您的平台拒绝重复名称时）的行为。在第一列中找到该行为，即可了解重新打开的会话在 **Activity** 标签中显示什么：

| 当工作负载已存在时，您的 hook | **Queued** 变为 **Failed** | 生成错误 |
| :- | :- | :- |
| 以 0 退出且未创建任何内容 | 连续 10 次此类请求之后 | `runner did not register within ETA after max attempts` |
| 以 1 退出 | 连续 10 次此类请求之后 | 您的 hook 的 stderr 末尾内容；如果它未向 stderr 写入任何内容，则为 `exit 1` |
| 以 2 或更高退出 | 立即 | 您的 hook 的 stderr 末尾内容；如果它未向 stderr 写入任何内容，则为 `exit` 加上退出码，例如 `exit 2` |
| 一直等待到 `--hook-timeout` 结束 | 连续 10 次此类请求之后 | 您的 hook 的 stderr 末尾内容；如果它未向 stderr 写入任何内容，则为 `spawn-runner hook timed out after 60000ms` |

在最后一行的生成错误中，`60000` 是以毫秒为单位的 `--hook-timeout`。当 hook 一直等待到 `--hook-timeout` 结束时，编排器的日志中还会出现一行 `timed out after`。

要修复 hook，请按照 [hook 约定的第 1 条规则](#the-spawn-runner-hook)的要求，以 `CLAUDE_RUNNER_ORDER_ID` 构建每个名称或键。在该示例中，名称变为：

```bash theme={null}
name="runner-${CLAUDE_RUNNER_ORDER_ID}"
```

如果您的 hook 会等待，还请让它在提交工作负载后立即退出，因为等待中的 hook 会[延迟其编排器的后续 hook](https://code.claude.com/docs/zh-CN/self-hosted-environments-reference.md#orchestrator-cli-flags)。修改 hook 后，请在 **Activity** 标签中检查每个卡住会话的状态：

* **Queued**：会话会自动获得下一次生成请求
* **Failed**：以 Owner 身份对其选择 **Retry**，或让其用户向其发送新消息

#### 让按需运行器在空闲时退出

默认情况下，您的 hook 启动的运行器不会在其会话的轮次结束时退出，因此您提交的工作负载会持续运行。要让运行器退出，请让您的 hook 传递以下[运行器标志](https://code.claude.com/docs/zh-CN/self-hosted-environments-reference.md#runner-cli-flags)：

| 标志及示例值 | 传递给 | 作用 |
| :- | :- | :- |
| `--release-idle-session-min 30` | 您的 hook 启动的每个运行器 | 运行器在空闲达到该分钟数后释放其会话，然后以 0 退出。当会话的用户发送下一条消息时，会话会恢复，但[其尚未推送的工作将会丢失](https://code.claude.com/docs/zh-CN/self-hosted-environments-deploy.md#resumed-sessions-lose-unpushed-work)。对于会话遗留的仍在运行的进程，请[保留会话](https://code.claude.com/docs/zh-CN/self-hosted-environments-deploy.md#keep-an-idle-session-while-its-processes-run)。 |
| `--kill-session-after-min 480` | 您的 hook 启动的每个运行器 | 将每个会话限制为该分钟数，作为[不计为空闲的会话](https://code.claude.com/docs/zh-CN/self-hosted-environments-deploy.md#some-sessions-don’t-count-as-idle)的兜底措施，因此请将其设置为高于您预期的最长会话时长 |
| `--exit-if-unused-min 60` | 待命运行器，即您的 hook 为预热请求启动的运行器 | 从未接收会话的待命运行器会在该分钟数后退出。如果您设置了 [`--min-idle`](https://code.claude.com/docs/zh-CN/self-hosted-environments-reference#orchestrator-cli-flags)，除非该待命运行器是多余的，否则您会获得替代运行器，因此请使用远高于启动时间的值。 |

在运行器退出时移除工作负载，例如在 Kubernetes Job 上设置 `ttlSecondsAfterFinished`，或让 VM 的启动脚本在之后删除该 VM。

#### 将后台进程的输出发送到文件

如果您的 `spawn-runner` hook 让某个进程在后台持续运行，请将该进程的输出发送到 hook 的用户可以创建的文件。例如，在启动该进程的命令末尾、任何 `&` 之前追加 `>> "$HOME/claude-runner.log" 2>&1`。以下 hook 行展示了重定向的位置，其中 `your-background-command` 代表您的 hook 放到后台运行的命令：

```bash theme={null}
your-background-command >> "$HOME/claude-runner.log" 2>&1 &
```

如果没有重定向，该进程会写入 hook 的 stdout 和 stderr：

* hook 退出约一秒后，编排器会记录 `exited, but a process it started still has the hook's stdout or stderr open`。从该行起，它会在 `[runner:hook:spawn-runner:after-exit jti=<order id>]` 下再记录最多 64 KB 的该输出，然后丢弃其余部分。
* 编排器退出后，该进程对这些位置的写入会失败。

由您的 hook 自行启动的运行器会在编排器主机上运行会话，而该主机持有[环境密钥](https://code.claude.com/docs/zh-CN/self-hosted-environments-deploy.md#harden-your-deployment)。

在 v2.1.305 之前，当 hook 已退出但其启动的进程仍打开着 hook 的 stdout 或 stderr 时，hook 仍会达到 [`--hook-timeout`](https://code.claude.com/docs/zh-CN/self-hosted-environments-reference#orchestrator-cli-flags)，显示如下：

* **Activity 标签**，在失败会话的生成错误中：`spawn-runner hook timed out after 60000ms`，或 hook 的 stderr 上的最后几行。
* **编排器日志**：`timed out after 60000ms, sending SIGTERM to process tree`。

#### 退出 hook 而不等待运行器

如果每个按需运行器都在其 hook 启动约 60 秒后停止，并且您的 hook 针对同一会话再次运行，请检查 hook 是否在等待运行器。这 60 秒是默认的 [`--hook-timeout`](https://code.claude.com/docs/zh-CN/self-hosted-environments-reference#orchestrator-cli-flags)，因此如果您设置了自己的值，运行器会在 hook 启动后经过该时长停止。

在编排器主机上启动 `claude self-hosted-runner` 并等待它（无论是在前台运行还是使用 `wait`）的 hook，在 `--hook-timeout` 结束时仍在运行。编排器随后会停止该 hook 及其下运行的进程，包括该运行器和会话的 Claude 进程，因此仍在运行的轮次会被中断。

* **确认是超时导致运行器停止**：在编排器的日志中查找每个生成请求是否都出现 `timed out after 60000ms, sending SIGTERM to process tree`。其中 `60000` 是以毫秒为单位的 `--hook-timeout`。
* **修复方法**：让 hook 将运行器提交到您的平台并以 0 退出。如果改为调高 `--hook-timeout`，运行器只会更晚停止。

#### 在 shell hook 中保持暂时性失败可重试

在使用 `set -e` 的 shell hook 中，本可通过重试解决的失败可能会导致会话被阻止。hook 会在失败的命令处停止，并以该命令自身的状态退出，而编排器会对该状态应用退出码约定。许多失败返回 2 或更高的状态，例如命令未安装时返回的 `127`，以及 `curl --fail` 遇到 HTTP 错误时返回的 `22`，因此它们会在第一次失败时就阻止会话。

已被 hook 阻止的会话会保持阻止状态，直到用户向其发送新消息，或 [Owner](https://code.claude.com/docs/zh-CN/cloud-environments.md#organization-shared-environments) 在环境的 **Activity** 标签中对其选择 **Retry**。

要将此类失败改为退出 1，请将以下几行直接放在 hook 的 `#!` 行下方、任何可能失败的内容之上：

```bash theme={null}
set -e
PERMANENT=; permanent() { printf '%s\n' "$*" >&2; PERMANENT=1; exit 2; }
trap 'rc=$?; [ "$rc" -eq 0 ] || [ -n "${PERMANENT:-}" ] || exit 1' EXIT
```

这几行会改变 hook 其余部分的行为方式，因此添加后请检查 hook 中是否存在以下每种模式：

* **单独的 `exit 2` 或更高**：设置 trap 后，它会变为退出 1。对于任何重试都无法修复的错误，请改为调用 `permanent` 并附上原因，例如 `permanent "namespace claude-runners does not exist"`。请在主 shell 中调用它，而不要在 `$( )`、`( )` 或管道内调用。
* **`exec`**：不要以 `exec` 开始 hook 的最后一条命令，因为 `exec` 会替换 shell，trap 将不会运行。
* **第二个 `EXIT` trap**：第二个 `trap ... EXIT` 会替换第一个，因此请将两者合并为一个 trap。将您的清理命令直接放在 `rc=$?;` 之后，并在每条命令末尾加上 `|| true;`。这样清理在失败和成功时都会运行，而且失败的清理命令不会设置 hook 的退出状态。以下合并后的 trap 展示了其结构，其中 `your-cleanup-command` 代表您自己的命令：

  ```bash theme={null}
  trap 'rc=$?; your-cleanup-command || true; [ "$rc" -eq 0 ] || [ -n "${PERMANENT:-}" ] || exit 1' EXIT
  ```
* **允许失败的命令**：如果 hook 之前未使用 `set -e`，它现在会在第一条返回非零值的命令处停止，例如未找到任何结果的查找。如果 hook 会根据结果执行操作，请将该命令作为 `if` 的条件。如果 hook 忽略结果，请在该命令后加上 `|| true`。
* **提交工作负载的命令**：不要在其后加上 `|| true`。加上 `|| true` 后，即使您的平台拒绝了提交（例如超出配额），hook 也会以 0 退出，而会话会一直等待一个从未启动的运行器。

  请按照 [hook 的第一条规则](#the-spawn-runner-hook)所述，以 `CLAUDE_RUNNER_ORDER_ID` 命名工作负载。这样，“已存在”的应答就是同一请求的重新交付，因此仅将该应答视为成功。对于其他任何应答，请将应答打印到 stderr 并以 1 退出；如果任何重试都无法修复，则调用 `permanent`。以下示例针对 `kubectl create` 执行此操作，其中 `job.yaml` 仅包含该工作负载：

  ```bash theme={null}
  if ! out=$(kubectl create -f job.yaml 2>&1); then
    case "$out" in
      *AlreadyExists*) ;;
      *) printf '%s\n' "${out:-kubectl create failed}" >&2; exit 1 ;;
    esac
  fi
  ```

  随后编排器的日志中会显示该消息。如果 hook 还会创建其他对象，请以订单 ID 命名每个对象，为每个对象使用单独的命令，并以同样方式将其“已存在”应答视为成功。对于包含密钥的对象，请打印固定消息来代替命令的输出。

要确认 trap 是否生效，请在 `trap` 行正下方添加一行，调用一个不存在的命令，例如 `no-such-command`。从您的 shell 运行 hook 文件，检查 `echo $?` 是否输出 `1`，然后删除该行。

### 停止或重启编排器

要在不中断正在运行的 `spawn-runner` hook 的情况下停止或重启编排器，请使用 [`--shutdown-drain-sec`](https://code.claude.com/docs/zh-CN/self-hosted-environments-reference#orchestrator-cli-flags) 启动它。这样，收到 `SIGTERM` 时，编排器会给正在运行的 hook 最多您所设置的秒数来完成。要选择该值并相应地设置您的进程管理器，请参阅[确定 `--shutdown-drain-sec` 和编排器的停止超时时间](#size-shutdown-drain-sec-and-the-supervisors-stop-timeout-for-the-orchestrator)。

以下启动命令允许 70 秒：

```bash theme={null}
claude self-hosted-runner orchestrator \
  --environment-secret-file /etc/claude/environment-secret \
  --hooks-dir /etc/claude/hooks \
  --shutdown-drain-sec 70
```

当已发送的轮询和每个 hook 都完成后，编排器即退出。当 `--shutdown-drain-sec` 的秒数用尽时，仍在运行的 hook 会收到 `SIGTERM`，编排器随即退出。第二个 `SIGTERM` 或任何 `SIGINT` 会产生相同效果，且不再等待。包含 `[runner:orchestrator] stop summary:` 的日志行会按每个 hook 和生成请求的结束方式进行计数。

如果不使用 `--shutdown-drain-sec`，编排器会在收到 `SIGTERM` 或 `SIGINT` 时立即停止。每个正在运行的 hook 都会收到 `SIGTERM`，且不会通知 Anthropic。这样，运行器从未启动的会话可能会一直停留在队列中，直到 Anthropic [重新请求它](#the-spawn-runner-hook)，这会在 `--expected-spawn-seconds`（默认 120 秒）过后发生。

> [!NOTE]
>
> `--shutdown-drain-sec` 需要 Claude Code v2.1.299 或更高版本。较旧的版本会在启动时以 `error: unknown flag --shutdown-drain-sec` 退出。

#### 确定 `--shutdown-drain-sec` 和编排器的停止超时时间

在默认设置下，请使用 [`--shutdown-drain-sec 70`](https://code.claude.com/docs/zh-CN/self-hosted-environments-reference#orchestrator-cli-flags) 并将停止超时时间设为 85 秒，即在 Kubernetes 上使用 `terminationGracePeriodSeconds: 85`，或在 systemd 下使用 `TimeoutStopSec=85`。如果您更改了 `--hook-timeout`，请按比例调整这两个值：

* **`--shutdown-drain-sec`**：至少为 `--hook-timeout` 加 10 秒，以便在 `SIGTERM` 之前刚启动的 hook 仍能完成
* **停止超时时间**：至少为 `--shutdown-drain-sec` 加 15 秒。编排器会在启动时记录此数值：请查找包含 `shutdown drain:` 的行。

请让您的进程管理器只发送一个 `SIGTERM`，且仅发送给编排器进程，因为第二个 `SIGTERM` 会缩短 `--shutdown-drain-sec` 的等待时间。如果您保留 systemd 的默认设置 [`KillMode=control-group`](https://code.claude.com/docs/zh-CN/self-hosted-environments-deploy#what-reaches-a-running-post-session-hook)，hook 也会收到 `SIGTERM`，hook 可能会在提交其工作负载之前终止。在更改 `KillMode` 之前，请检查您的 `spawn-runner` hook 是如何启动运行器的：

* **如果您的 hook 将其工作负载提交到平台后即退出**：将编排器作为该单元的主进程运行，并设置 `KillMode=mixed`，这样只会向该进程发送 `SIGTERM`。
* **如果您的 hook 在编排器自身的单元中以进程形式启动运行器**：`KillMode=mixed` 不会向这些运行器发送 `SIGTERM`。相反，systemd 稍后会向它们发送 `SIGKILL`，因此它们会在未排空其会话、也未运行其 `post-session` hook 的情况下退出。

## 将模型请求发送到 Bedrock 或 Agent Platform

如果您的组织需要模型请求通过其自己的 AWS 或 Google Cloud 账户，请为 runner 配置 [Amazon Bedrock](https://code.claude.com/docs/zh-CN/amazon-bedrock.md) 或 [Google Cloud 的 Agent Platform（前身为 Vertex AI）](https://code.claude.com/docs/zh-CN/google-vertex-ai.md)。之后，该 runner 启动的每个会话都会使用您的云凭据在您的云账户中调用模型。如果没有此配置，会话会将模型请求发送到 Anthropic API。

runner 仍会向 Anthropic 轮询会话，且每个会话仍会将其事件流发送到 `api.anthropic.com`。事件流包含提示词、响应和工具结果。[可用性和限制](https://code.claude.com/docs/zh-CN/self-hosted-environments#availability-and-limitations)中的套餐要求和零数据保留（Zero Data Retention）排除规定仍然适用。

会话是路由到环境而非 runner 的，重新排队或恢复的会话可能会在不同的 runner 上运行。请以相同方式配置环境中的每个 runner。开始之前，请阅读[这些提供商的不同之处](#what-differs-from-sessions-on-the-anthropic-api)。

**1. 准备云账户和出站规则**

设置模型访问权限、范围严格受限的策略或角色以及网络访问：

* **Amazon Bedrock**：[提交用例详情](https://code.claude.com/docs/zh-CN/amazon-bedrock.md#1-submit-use-case-details)，然后按照 [IAM 配置](https://code.claude.com/docs/zh-CN/amazon-bedrock.md#iam-configuration)创建策略，将 `bedrock:InvokeModel` 和 `bedrock:InvokeModelWithResponseStream` 限制为您的会话所使用的推理配置文件及其背后的基础模型
* **Agent Platform**：[启用 API](https://code.claude.com/docs/zh-CN/google-vertex-ai.md#1-enable-agent-platform-api) 并[申请模型访问权限](https://code.claude.com/docs/zh-CN/google-vertex-ai.md#2-request-model-access)，然后创建 [IAM 配置](https://code.claude.com/docs/zh-CN/google-vertex-ai.md#iam-configuration)中所述的自定义角色，仅包含 `aiplatform.endpoints.predict`
* **出站流量**：在出站规则中放行您的提供商的端点。请参阅[网络要求](https://code.claude.com/docs/zh-CN/self-hosted-environments-deploy.md#network-requirements)。如果会话无法访问这些端点，Claude Code 可能会持续重试数小时，之后会话才会显示错误。

**2. 为会话提供范围严格受限的凭据**

将步骤 1 中的策略或角色附加到一个不能执行任何其他操作的身份。有关 Claude Code 接受的方法，请参阅[配置 AWS 凭据](https://code.claude.com/docs/zh-CN/amazon-bedrock.md#2-configure-aws-credentials)和[配置 GCP 凭据](https://code.claude.com/docs/zh-CN/google-vertex-ai.md#3-configure-gcp-credentials)。

> [!WARNING]
>
> 任何能够让代码在会话中运行的人（包括通过提示词注入）都可以在这些凭据有效期间使用它们，费用由您承担。Claude Code 在会话内部运行，因此它调用模型所用的凭据必须在会话中可读。
>
> Claude 运行的 shell 命令、您的 [Claude Code hook](https://code.claude.com/docs/zh-CN/hooks.md) 以及 stdio MCP 服务器都会继承会话的环境，并以与 Claude Code 相同的用户身份运行。因此，它们可以读取凭据变量和凭据文件。
>
> 当您[加固部署](https://code.claude.com/docs/zh-CN/self-hosted-environments-deploy.md#harden-your-deployment)时，可以让主机凭据远离会话，但无法将此凭据排除在外。请确保其背后的身份除步骤 1 中的策略或角色外不具备任何其他权限。

请对照以下 runner 行为检查您选择的方法：

* **元数据端点**：如果您完全禁止会话访问[云元数据端点](https://code.claude.com/docs/zh-CN/self-hosted-environments-deploy.md#harden-your-deployment)，则由该端点提供的凭据（例如实例配置文件）也无法到达 Claude Code。基于文件的 Web 身份（例如 Amazon EKS 上的 IAM Roles for Service Accounts (IRSA) 或 Workload Identity Federation 凭据文件）不依赖于该端点。
* **续期**：会话的持续时间可能超过凭据的有效期，因此请使用能够自动续期的方法，例如基于文件的 Web 身份
* **包装脚本**：runner 每个会话只启动一次您的[包装脚本](#provision-credentials-scoped-to-the-session-creator)，因此它导出的凭据不会续期。Claude Code 会从其环境中读取 AWS 凭据，因此如果您的包装脚本已为其他工作导出了 AWS 凭据，Claude Code 可能会使用这些凭据对模型请求进行签名。

**3. 在 runner 的环境中设置一个提供商的变量**

在设置 runner 其他环境变量的位置（例如容器规范或服务单元）中，只设置一个提供商的变量，然后重启 runner。示例中以 shell export 的形式展示这些变量。使用[按需 runner](#on-demand-runners) 时，请在您的 `spawn-runner` hook 启动的工作负载上设置它们。

使用 [`--confine-repo-settings enforce`](https://code.claude.com/docs/zh-CN/self-hosted-environments-deploy#harden-your-deployment) 启动这些 runner。它会拒绝那些已提交设置被其标记的仓库上的会话，因此请先在默认的 `warn` 模式下运行，并清理其记录的问题。

> [tab] Amazon Bedrock
>
> 将区域替换为您自己的区域：
>
> ```bash theme={null}
> export CLAUDE_CODE_USE_BEDROCK=1
> export AWS_REGION=us-east-1
> ```
>
> 有关 Claude Code 如何解析区域，请参阅[配置 Claude Code](https://code.claude.com/docs/zh-CN/amazon-bedrock.md#3-configure-claude-code)。有关 Claude Code 针对您的区域使用哪个推理配置文件前缀，请参阅[跨区域推理配置文件前缀](https://code.claude.com/docs/zh-CN/amazon-bedrock.md#cross-region-inference-profile-prefixes)。

> [tab] Google Cloud's Agent Platform
>
> 将区域和项目 ID 替换为您自己的值：
>
> ```bash theme={null}
> export CLAUDE_CODE_USE_VERTEX=1
> export CLOUD_ML_REGION=global
> export ANTHROPIC_VERTEX_PROJECT_ID=YOUR-PROJECT-ID
> ```
>
> 要选择区域，请参阅[区域配置](https://code.claude.com/docs/zh-CN/google-vertex-ai.md#region-configuration)。

**4. 检查变量是否已传递到会话**

您在主机上自己的 shell 是另一个进程，因此请从会话内部进行检查。在该环境中启动一个会话，并让 Claude 运行以下命令：

```bash theme={null}
env | grep -E 'CLAUDE_CODE_USE_(BEDROCK|VERTEX)'
```

如果有一行将 `CLAUDE_CODE_USE_BEDROCK` 或 `CLAUDE_CODE_USE_VERTEX` 设置为 `1`，则表示该变量已传递到会话。如果两者都出现，Claude Code 将使用 Amazon Bedrock。如果没有输出，则表示两者都未传递到会话。

该命令显示的是配置，而非流量。要确认请求本身，请在您云账户自身的指标或请求日志中查找这些请求。如果第一条消息失败，请参阅 [Amazon Bedrock](https://code.claude.com/docs/zh-CN/amazon-bedrock.md#troubleshooting) 或 [Agent Platform](https://code.claude.com/docs/zh-CN/google-vertex-ai.md#troubleshooting) 的故障排除。

### 与 Anthropic API 上的会话的不同之处

将模型请求发送到 Amazon Bedrock 或 Google Cloud 的 Agent Platform 的会话与 Anthropic API 上的会话存在以下不同：

* **来自 claude.ai 的策略**：[服务器托管设置](https://code.claude.com/docs/zh-CN/server-managed-settings.md)不会传递到这些会话。Owner 在 Claude Code 管理设置中设定的组织策略也不会传递到这些会话，因此 Claude Code 不会在会话中强制执行这些策略。请将您依赖的规则放入 runner 镜像的[托管设置文件](https://code.claude.com/docs/zh-CN/managed-settings.md#delivery-mechanisms)中。
* **账户 skill**：这些会话不会下载用户的 claude.ai 账户中已启用的 skill。请参阅[每个会话的配置是如何组装的](#how-each-session’s-config-is-assembled)。
* **文件**：用户在 claude.ai 或移动端、桌面端应用中附加到会话的文件不会传递到会话，Claude 也无法通过 [`SendUserFile` 工具](https://code.claude.com/docs/zh-CN/tools-reference)回传文件。请改为将输入文件放在仓库中或 runner 上。
* **模型选择**：Anthropic 的控制平面会发送每个会话的模型；当会话启动时未指定模型，Claude Code 会使用该提供商的默认模型。您无法通过 runner 环境中的 `ANTHROPIC_MODEL` 或 `ANTHROPIC_DEFAULT_MODEL` 选择模型，但可以固定别名解析到的模型：
  * **`ANTHROPIC_MODEL` 和 `ANTHROPIC_DEFAULT_MODEL`**：runner 会从其传递给会话的环境中移除这两个变量，尽管提供商页面的示例设置了 `ANTHROPIC_MODEL`。
  * **各模型系列的固定变量**：[Amazon Bedrock](https://code.claude.com/docs/zh-CN/amazon-bedrock.md#4-pin-model-versions) 和 [Agent Platform](https://code.claude.com/docs/zh-CN/google-vertex-ai.md#5-pin-model-versions) 的"固定模型版本"中的变量确实会传递到会话。它们决定的是 `opus` 等别名解析为哪个模型，而不是完整模型 ID 解析为哪个模型。
* **您的账户不提供的模型**：会话可能在某条消息上失败，并显示指明该模型的错误。请启用您的开发人员可以选择的模型、"固定模型版本"中所述的后台模型，以及[自动模式](https://code.claude.com/docs/zh-CN/permission-modes.md#enable-auto-mode-on-bedrock-agent-platform-or-foundry)使用的分类器模型。在 Amazon Bedrock 上，请在策略中允许其中的每一个模型。
* **Web 搜索和快速模式**：[Web 搜索](https://code.claude.com/docs/zh-CN/tools-reference.md#websearch-tool-behavior)在 Amazon Bedrock 上不可用，[快速模式](https://code.claude.com/docs/zh-CN/fast-mode.md)在这两个提供商上均不可用。有关因提供商而异的其他功能，请参阅[因提供商而异的 CLI 功能](https://code.claude.com/docs/zh-CN/feature-availability.md#cli-capabilities-that-vary-by-provider)。

## MCP 服务器

要让 [MCP 服务器](https://code.claude.com/docs/zh-CN/mcp.md)在每个会话中都可用，请在镜像构建时使用与桌面安装相同的 `claude mcp add` 命令添加它们。如果您的运行器是裸进程而非容器，请在主机上以运行器的用户身份运行相同的命令，然后重启运行器：运行器仅在启动时读取一次主机配置。必须使用 `--scope user` 标志；默认的 local 作用域会写入按目录区分的键下，而运行器不会将该键注入会话。例如，在您的 Dockerfile 中：

```dockerfile theme={null}
RUN claude mcp add --scope user sidecar -- /usr/local/bin/mcp-sidecar
RUN claude mcp add --scope user --transport http internal http://mcp-gateway.svc.cluster.local:8080
```

运行器在启动时对主机配置做一次快照。该快照会捕获主机 `.claude.json` 中的 `mcpServers` 键（该文件位于 `~/.claude/` 旁边，而非其内部），运行器只会将这一个键注入每个会话的隔离配置中；账户状态和项目历史会被丢弃。要确认服务器已到达会话，请在该环境上启动一个会话，并让 Claude 列出其 MCP 工具；对于捕获到的任何 `type` 无法识别的条目，运行器还会在启动时记录一条警告并丢弃该条目，因此您可以看到该服务器为何没有出现在会话中。设置 `SELF_HOSTED_RUNNER_HOST_CONFIG_DIR` 后，运行器会改为从该目录读取 `.claude.json`，因此将该变量指向一个空目录也会禁用 MCP 注入。

Claude Code 还会从其他来源加载 MCP 服务器：

* 位于标准系统路径的企业作用域[托管 MCP 文件](https://code.claude.com/docs/zh-CN/managed-mcp.md)：Linux 运行器主机上为 `/etc/claude-code/managed-mcp.json`，macOS 主机上为 `/Library/Application Support/ClaudeCode/managed-mcp.json`。适用于只允许加载管理员列出的服务器的锁定机群。有关优先级规则，请参阅[使用 managed-mcp.json 进行独占控制](https://code.claude.com/docs/zh-CN/managed-mcp.md#exclusive-control-with-managed-mcp-json)。当运行器主机上存在此文件时，Claude Code 会跳过 Anthropic 控制平面下发给会话的 MCP 服务器（包括 claude.ai 连接器），并在会话子进程的 stderr 上以警告形式列出它们的名称，运行器会以 `debug` 日志级别记录这些警告。在 v2.1.229 之前，这些会话会在启动时退出并显示 `You cannot dynamically configure MCP servers when an enterprise MCP config is present`。
* 运行器主机上[托管设置](https://code.claude.com/docs/zh-CN/managed-settings.md)中的 [`managedMcpServers`](https://code.claude.com/docs/zh-CN/settings-reference#managedmcpservers) 键：提供 HTTP 和 SSE 服务器，但不进行独占控制，因此来自其他来源的服务器仍会加载。需要 Claude Code v2.1.259 或更高版本。
* `<repo>/.mcp.json`：项目作用域。在[项目](https://code.claude.com/docs/zh-CN/claude-projects.md#what-threads-pick-up-from-your-repositories)线程之外，将该文件提交到仓库，其中的服务器在云端会话中会被自动批准。在包含多个仓库的会话中，[最多只会加载一个仓库的该文件](#repository-settings-in-sessions-with-several-repositories)。
* 来自您的包装脚本的 [`--mcp-config`](https://code.claude.com/docs/zh-CN/cli-reference#cli-flags) 文件：将该标志追加在 `"$@"` 之后，例如 `exec "$CLAUDE_RUNNER_CLAUDE_BIN" "$@" --mcp-config /etc/claude/mcp.json`。为了在会话中途更新会话所下发的服务器时让该文件中的服务器保持运行，请让您的包装脚本将[完整环境](#wrapper-scripts)传递给子进程，就像 `exec` 所做的那样。在 v2.1.299 之前，该文件中的 stdio、HTTP 和 SSE 服务器会在第一次未指明它们的此类更新时关闭。

当您的组织启用了连接器下发时，Anthropic 的控制平面会通过服务器提供的 MCP 配置，将您在 claude.ai 上配置的连接器下发到以交互方式创建的会话，请求经由 `api.anthropic.com` 路由。以编程方式创建的会话（例如 [CLI 调度](https://code.claude.com/docs/zh-CN/self-hosted-environments-testing.md#run-the-test-loop)）不会接收连接器下发；请改为通过本节列出的任何其他来源为它们提供 MCP 服务器。子进程的 OAuth 令牌不带有直接获取连接器的作用域，因此子进程本身不会尝试获取；下发由服务器驱动。

> [!NOTE]
>
> 在已接收连接器的会话中，来自主机 `.claude.json` 或仓库 `.mcp.json` 的同名服务器不会被连接。要同时使用两者，请为您的服务器指定不同的名称。

`settings.json` 不包含 MCP 服务器定义，设置 schema 中也没有顶层 `mcpServers` 字段。在托管设置中，请改用 [`managedMcpServers`](https://code.claude.com/docs/zh-CN/settings-reference#managedmcpservers) 键提供服务器。

会话会继承运行器的环境，因此请在运行器环境中设置 [`ENABLE_TOOL_SEARCH`](https://code.claude.com/docs/zh-CN/mcp#scale-with-mcp-tool-search)，以控制该运行器生成的每个会话的 MCP 工具搜索；MCP 页面介绍了可用的值。

### 在第一轮之前等待 MCP 服务器

自托管会话会在两个不同的时间点短暂等待仍在连接中的 MCP 服务器。错过等待的服务器，其工具在第一轮开始时不可用，之后会自动变为可用，无需您进行任何操作。这两次等待分别是：

* **会话启动**：在首次获取工具列表之前，会话默认最多等待 5 秒，等待条目中设置了 [`alwaysLoad: true`](https://code.claude.com/docs/zh-CN/mcp#exempt-a-server-from-deferral) 的 HTTP 或 SSE 服务器；如果您在运行器的环境中设置了 [`MCP_CONNECTION_NONBLOCKING=0`](https://code.claude.com/docs/zh-CN/env-vars)，则会等待所有服务器。否则，HTTP 和 SSE 服务器会在后台连接。会话在此处等待期间，初始化会变慢。[`MCP_CONNECT_TIMEOUT_MS`](https://code.claude.com/docs/zh-CN/env-vars) 可更改 5 秒的默认值。
* **第一轮**：消息到达后，第一轮最多等待 2 秒，等待仍在连接中的 stdio 服务器。会话在此处等待期间，第一条回复会变慢。要更改此等待的时长，请在运行器的环境中设置 [`CLAUDE_CODE_MCP_STARTUP_WAIT_MS`](https://code.claude.com/docs/zh-CN/env-vars)。它不会改变此等待涵盖哪些服务器。需要 Claude Code v2.1.274 或更高版本。

`claude mcp add` 没有 `alwaysLoad` 标志。要设置该键，请改用 `claude mcp add-json` 添加服务器，该命令从服务器的 JSON 中接收该键并将其写入 `.claude.json`。在您的 Dockerfile 中：

```dockerfile theme={null}
RUN claude mcp add-json core '{"type":"http","url":"https://mcp.example.com/mcp","alwaysLoad":true}' --scope user
```

如果某个服务器的工具在后续轮次中也没有出现，请按照 [MCP 服务器](#mcp-servers)中的说明，检查该服务器是否到达了会话。

### 关闭内置会话工具

Anthropic 的控制平面会将其自己的 MCP 服务器（名为 Claude Code Remote）附加到云端会话。Claude 使用该服务器的工具来创建[云端定时任务](https://code.claude.com/docs/zh-CN/routines.md)、启动和引导其他云端会话、附加更多仓库，以及跟踪 Pull Request 活动。

要关闭整个服务器，请在您的设置中添加一条[服务器级拒绝规则](https://code.claude.com/docs/zh-CN/permissions.md#mcp)。根据会话的创建方式，控制平面会以三个名称之一注册该服务器。Claude Code 会精确匹配规则中的名称（包括大小写），因此请按如下所示为每个名称各写一条规则：

```json theme={null}
{
  "permissions": {
    "deny": [
      "mcp__Claude_Code_Remote",
      "mcp__claude-code-remote",
      "mcp__bf7c680d-5fdc-5ef4-b4a0-abadb619bf0a"
    ]
  }
}
```

指定整个服务器的规则也会覆盖该服务器以后新增的工具。要关闭某一个工具并保留其余工具，请在每条规则后追加两个下划线和工具名称，例如 `mcp__Claude_Code_Remote__add_repo`。如果要完全阻止该服务器连接，而不仅是移除其工具，请改为将这三个名称（不带 `mcp__` 前缀）作为 `serverName` 条目添加到 [`deniedMcpServers`](https://code.claude.com/docs/zh-CN/managed-mcp#policy-based-control-with-allowlists-and-denylists) 下。

将这些规则放在[服务器托管设置](https://code.claude.com/docs/zh-CN/server-managed-settings.md)中，无需更改运行器即可作用于会话；也可以放在运行器上的 `~/.claude/settings.json` 中。在[将模型请求发送到 Bedrock 或 Agent Platform](#send-model-requests-to-bedrock-or-agent-platform) 的运行器上，请使用该文件，因为服务器托管设置不会作用于这些会话。[权限和工具批准](#permissions-and-tool-approval)说明了运行器上的设置如何作用于会话。

要确认规则已生效，请在该环境上启动一个会话，并让 Claude 列出其 MCP 工具。Claude Code 会从 Claude 的上下文中移除被拒绝的工具，因此被拒绝的工具不会出现在其回答中。

## 提示会话推送其工作

Anthropic 托管的会话运行 [`Stop` hook](https://code.claude.com/docs/zh-CN/hooks#stop)，Claude Code 钩子在 Claude 完成响应时运行，提示 Claude 提交并推送其工作。运行器不安装一个。没有它，以未提交更改结束的会话仅在运行器的磁盘上留下该工作，claude.ai/code 中的 **Create PR** 按钮保持不活跃，直到分支存在于远程。

下面的参考实现有两部分。将设置块合并到运行器主机上的 `~/.claude/settings.json` 中，运行器将其播种到每个会话中，并将脚本保存为运行器主机上的 `~/.claude/hooks/stop-hook-nudge.sh` 并使其可执行：

```json theme={null}
{
  "hooks": {
    "Stop": [
      {
        "hooks": [
          {
            "type": "command",
            "timeout": 10,
            "command": "\"$CLAUDE_CONFIG_DIR/hooks/stop-hook-nudge.sh\""
          }
        ]
      }
    ]
  }
}
```

```sh theme={null}
#!/bin/sh
# Stop-hook reference implementation for self-hosted runners.
#
# Nudges Claude once per turn if the project directory has uncommitted
# changes OR unpushed commits, so work isn't lost when an idle session
# is released and so the "Create PR" button on claude.ai/code lights up.
#
# Runner-level (no repo changes): drop this file at ~/.claude/hooks/ on
# the runner host and merge the accompanying Stop-hook settings block
# into ~/.claude/settings.json — the runner seeds both into every session.
# Repo-level alternative: commit to <repo>/.claude/hooks/ and change the
# settings.json command path to $CLAUDE_PROJECT_DIR/.claude/hooks/.
#
# stdin: hook JSON payload (see https://code.claude.com/docs/en/hooks)
# stdout: {"decision":"block","reason":"..."} to nudge, or nothing to allow stop.

# Re-entry guard: the harness sets stop_hook_active=true when re-invoking
# the Stop hook after a block. Bail so we only nudge once per turn. The
# harness emits compact JSON (no space after the colon), which this
# pattern relies on; use jq if you need a whitespace-tolerant check.
in=$(cat)
case "$in" in *'"stop_hook_active":true'*) exit 0 ;; esac

d="$CLAUDE_PROJECT_DIR"

# Hardening: this script runs outside Claude Code's Bash sandbox, so its git
# calls skip some features that can start a program named in the checkout's
# git settings. They don't skip them all, so don't count on the sandbox alone.
GIT_OPTIONAL_LOCKS=0
export GIT_OPTIONAL_LOCKS
git() { command git -c core.fsmonitor= -c core.hooksPath=/dev/null "$@"; }

# Not a git repo → nothing to nudge.
git -C "$d" rev-parse --git-dir >/dev/null 2>&1 || exit 0

# No remote → "push to the remote" is unsatisfiable; bail.
[ -z "$(git -C "$d" remote 2>/dev/null)" ] && exit 0

# Uncommitted changes (staged, unstaged, or untracked). Exclude .claude/
# entirely — operator-seeded settings and CLI-written runtime state
# (scheduler lock, worktrees, routine state) live there and neither is
# "uncommitted work" the model needs to push.
s=$(git -C "$d" status --porcelain --ignore-submodules=dirty -- . ':(exclude).claude/' 2>/dev/null)
if [ -n "$s" ]; then
  printf '{"decision":"block","reason":"There are uncommitted changes in the repository. Please commit and push these changes to the remote branch."}'
  exit 0
fi

# Unpushed commits. Count commits on HEAD not reachable from any
# remote-tracking ref or FETCH_HEAD. This works uniformly for:
#   - init+fetch checkouts (runner default: only FETCH_HEAD exists)
#   - clone-based checkouts (origin/* exist)
#   - the runner default: the child starts on the session's outcome
#     branch, which the runner creates after checkout
#   - detached HEAD, when a custom setup skips that branch creation
# With no reference point at all (never fetched), stay silent rather
# than false-positive on a read-only turn.
base=""
git -C "$d" rev-parse --verify -q FETCH_HEAD >/dev/null && base="FETCH_HEAD"
if [ -z "$base" ] && [ -z "$(git -C "$d" for-each-ref --count=1 refs/remotes/origin 2>/dev/null)" ]; then
  exit 0
fi
# shellcheck disable=SC2086  # $base is either "" or "FETCH_HEAD", intentional word-split
unpushed=$(git -C "$d" rev-list HEAD --not $base --remotes=origin --count 2>/dev/null) || unpushed=0
if [ "$unpushed" -gt 0 ]; then
  branch=$(git -C "$d" symbolic-ref --short -q HEAD)
  if [ -n "$branch" ]; then
    # $branch is attacker-influenced — git-check-ref-format(1) allows `"`
    # in ref names. `\` is forbidden (rule 10) but escaped anyway as cheap
    # defense-in-depth.
    # Escape JSON metacharacters before interpolating into the hand-built
    # payload so a branch like x","continue":false can't inject keys into
    # the hook-output JSON the harness parses. $unpushed is safe — the
    # -gt guard above rejects anything that isn't a plain integer.
    branch_esc=$(printf '%s' "$branch" | sed 's/\\/\\\\/g; s/"/\\"/g')
    printf '{"decision":"block","reason":"There are %s unpushed commit(s) on branch '\''%s'\''. Please push these changes to the remote repository."}' "$unpushed" "$branch_esc"
  else
    printf '{"decision":"block","reason":"There are %s unpushed commit(s) on a detached HEAD. Please create a branch and push it to the remote repository."}' "$unpushed"
  fi
  exit 0
fi

exit 0
```

该 hook 在会话结束前提示 Claude 提交并推送，当目录不是 git 仓库或没有远程时保持沉默。对于包含多个仓库的会话，请参阅 [`$CLAUDE_PROJECT_DIR` 指向的内容](#repository-settings-in-sessions-with-several-repositories)。

Stop hook 只负责提示 Claude。要使提交和推送能够成功，请完成以下设置：

* **git 身份**：在运行器镜像中设置 `user.name` 和 `user.email`，或使用 [`--configure-git`](https://code.claude.com/docs/zh-CN/self-hosted-environments-deploy#let-the-runner-configure-git) 启动运行器
* **推送凭据**：使用 [Anthropic 托管的 git](https://code.claude.com/docs/zh-CN/self-hosted-environments-deploy.md#use-the-anthropic-git-proxy)，或按照[在镜像中提供 git 配置](https://code.claude.com/docs/zh-CN/self-hosted-environments-deploy.md#ship-git-config-in-your-image)中的说明提供凭据
* **对于恢复的会话**：在环境中的每个运行器上设置 [`--push-outcome-on-release`](https://code.claude.com/docs/zh-CN/self-hosted-environments-reference#runner-cli-flags)。这样，恢复的会话将从 Claude 推送到该会话分支的提交开始。有关该标志的限制，请参阅[恢复的会话会丢失未推送的工作](https://code.claude.com/docs/zh-CN/self-hosted-environments-deploy.md#resumed-sessions-lose-unpushed-work)。

Stop hook 每个轮次最多提示一次。如果提交或推送失败，该轮次仍会结束。请在会话中查找 git 的错误信息，并修复其中指出的问题。

对于大型检出，请调高 Stop hook 设置中的 `timeout`，然后重启运行器。脚本中的 `git status` 会跳过 `core.fsmonitor`，因此耗时可能超过其中设置的 10 秒。超时的 hook 会直接结束该轮次，不会发出提示。

### 子模块与 Stop hook

Stop hook 检查的是每个子模块所处的提交，而不是其中的文件：

* **子模块中有新提交**：Claude 会一直收到提示，直到父仓库记录该提交为止，即使 `.gitmodules` 为该子模块设置了 `ignore = all`
* **子模块中有未提交的文件**：Claude 不会收到提示，因此这些工作只会留在运行器的磁盘上
* **子模块中有未推送的提交**：Stop hook 不会检查它们是否已推送，因此父仓库可能指向一个其他人都无法获取的提交

### 替换旧版 Stop hook 脚本

在您的 `stop-hook-nudge.sh` 副本中搜索 `GIT_OPTIONAL_LOCKS`。如果找不到，说明您使用的是旧版副本。请在所有安装位置将其替换为[当前脚本](#prompt-sessions-to-push-their-work)，然后重启运行器。保留您的 Stop hook 设置，但在大型检出上可调高 `timeout`。

* **时机**：如果您的会话在启用 [Bash 沙箱](https://code.claude.com/docs/zh-CN/sandboxing.md)的情况下运行，请立即替换。否则，请在下次构建镜像或重启运行器时替换。
* **变化**：子模块中未提交的文件不再提示 Claude。而 `.gitmodules` 标记为 `ignore = all` 的子模块中的新提交现在会提示 Claude。

> [!WARNING]
>
> hook 在 Bash 沙箱之外运行，因此当前脚本的 git 调用会跳过某些可能启动检出的 git 设置中所指定程序的 git 功能，但并不会跳过全部此类功能。不要仅依赖沙箱，并请按照[加固您的部署](https://code.claude.com/docs/zh-CN/self-hosted-environments-deploy.md#harden-your-deployment)中的说明隔离会话。

## 权限和工具批准

自托管会话没有连接的终端，因此未回答的权限提示会使当前轮次停滞，直到用户在 UI 中响应。Anthropic 的控制平面随工作负载一起发送每个会话的工具列表和权限规则；默认配置预批准常规工具调用（包括 `Bash`），并且云端会话[无论处于何种模式都会预批准文件编辑](https://code.claude.com/docs/zh-CN/permission-modes.md#switch-permission-modes)。任何未被预批准的调用都会通过会话 UI 进行提示。

> [!NOTE]
>
> 仅在会话容器运行时启用了[默认拒绝网络出口](https://code.claude.com/docs/zh-CN/self-hosted-environments-deploy.md#default-deny-egress)并落实了[加固部分](https://code.claude.com/docs/zh-CN/self-hosted-environments-deploy.md#harden-your-deployment)中其余措施的环境上固定自动模式。在默认预批准工具集和自动模式下，常规工具调用（包括 `Bash` 网络请求）都会在无人参与的情况下运行，因此网络边界才是限制这些调用可访问范围的关键。

要无论控制平面发送什么都将提示保持在最低限度，请从您的包装脚本或 [`command` hook](#command) 固定[自动模式](https://code.claude.com/docs/zh-CN/permission-modes.md#eliminate-prompts-with-auto-mode)。自动模式让会话无需常规权限提示即可运行：单独的分类器模型在操作运行前对其进行审查，并阻止它拒绝的操作，而显式的询问规则仍会强制提示；权限模式页面介绍了分类器检查的内容。运行器在调用包装脚本前追加服务器计算的标志，对于单值标志（如 `--permission-mode`），解析器以最后一次出现的值为准，因此您在 `"$@"` 之后追加的标志会覆盖服务器发送的值：

```bash theme={null}
#!/bin/bash
exec "$CLAUDE_RUNNER_CLAUDE_BIN" "$@" --permission-mode auto
```

要改为预批准特定工具，请追加 `--allowed-tools` 和您的规则，例如 `--allowed-tools "Bash(bazel *) Bash(yarn *) mcp__internal__*"`。列表标志（如 `--allowed-tools` 和 `--disallowed-tools`）会在多次出现时累积而不是覆盖，因此您的规则会叠加在控制平面发送的任何规则之上。要缩小范围，请追加 `--disallowed-tools`，即使其他规则允许某些工具，它也会拒绝这些工具。

### 每个会话的配置如何组装

运行器为每个会话提供自己的配置目录，该目录以运行器在启动时一次性捕获的主机 `~/.claude/` 快照为初始内容：您的运行器镜像中的 `settings.json`、`CLAUDE.md`、hook、Agent、命令和 skill 会作为用户级基线应用于每个会话。如果您更改正在运行的主机上的配置，更改仅在您重启运行器后生效。

设置 `SELF_HOSTED_RUNNER_HOST_CONFIG_DIR` 可从其他路径获取初始内容，或将其指向空目录以禁用初始内容填充。

会话还会读取以下设置文件：

* **项目设置**：在[项目](https://code.claude.com/docs/zh-CN/claude-projects.md#what-threads-pick-up-from-your-repositories)线程之外，仓库中提交的 `.claude/settings.json` 会叠加在用户级基线之上。在包含多个仓库的会话中，[最多只有一个仓库的文件生效](#repository-settings-in-sessions-with-several-repositories)。
* **托管设置**：会话会从运行器镜像中的标准系统路径读取 [`managed-settings.json`](https://code.claude.com/docs/zh-CN/settings#where-settings-live)。关于其中的键是否与[服务器托管设置](https://code.claude.com/docs/zh-CN/server-managed-settings.md)一起应用，请参阅 [Claude Code 如何合并托管来源](https://code.claude.com/docs/zh-CN/managed-settings.md#how-claude-code-combines-managed-sources)。

有关这些来源的应用顺序，请参阅[设置优先级](https://code.claude.com/docs/zh-CN/settings.md#settings-precedence)。

当 Anthropic 的控制平面为会话提供 [Claude Code hook](https://code.claude.com/docs/zh-CN/hooks.md) 时，运行器会将它们与您自己的配置并行安装，而不是覆盖您的配置。需要 Claude Code v2.1.229 或更高版本；较旧的运行器会忽略所提供的 hook。所提供的 hook 会运行 `python3`，因此请在您的[运行器镜像](https://code.claude.com/docs/zh-CN/self-hosted-environments-deploy.md#build-the-runner-image)中安装 `python3`。

* **安装位置**：运行器将提供的每个 hook 脚本写入会话配置目录中保留的 `hooks/.ccr-launcher/` 子目录，并在一个单独的设置文件中注册这些脚本，该文件通过 `--settings` 传递给会话，从而使初始填充的 `settings.json` 以及您位于 `hooks/<name>` 的脚本保持不变。运行器会为每个会话重新创建该保留子目录，并且不会将主机上 `~/.claude/hooks/.ccr-launcher/` 中的内容填充到会话中。
* **编写者**：控制平面使用其自身部署中的固定常量填充这些脚本，绝不使用按会话或第三方的输入。
* **仍然适用的管控**：通过 `--settings` 下发的 hook 会进入普通的合并 hook 配置，而不是托管层，因此您的托管设置仍然适用。`disableAllHooks` 会禁用它们，并且它们不属于 [`allowManagedHooksOnly`](https://code.claude.com/docs/zh-CN/settings-reference#allowmanagedhooksonly) 保持加载的类别。

当某人启动自己的会话时，Claude Code 会将[其 claude.ai 账户中启用的 skill](https://code.claude.com/docs/zh-CN/skills.md#skills-in-cowork-and-cloud-sessions) 下载到该会话的配置目录中。[云端定时任务](https://code.claude.com/docs/zh-CN/routines.md)运行不会获得其所有者的 skill，而[将模型请求发送到 Bedrock 或 Agent Platform](#send-model-requests-to-bedrock-or-agent-platform) 的会话不会下载任何 skill。对于这些会话需要的 skill，请将其添加到您的运行器镜像中，或者在[项目](https://code.claude.com/docs/zh-CN/claude-projects.md#what-threads-pick-up-from-your-repositories)线程之外，将其提交到仓库的 `.claude/skills/` 中。

除 [Claude Tag](https://claude.com/docs/claude-tag/overview) 会话外，自托管环境中的会话默认关闭[自动记忆](https://code.claude.com/docs/zh-CN/memory.md#auto-memory)。对于需要跨会话保留的指令，请使用运行器镜像或仓库中的 `CLAUDE.md`。

运行器对主机 `~/.claude/` 的快照不包含 `projects/` 目录。自动记忆的默认存储位置就在该目录下。如果您将记忆文件放在那里，运行器不会将它们填充到会话中，它们也不会启用自动记忆。

### 在获取服务器托管设置失败时阻止会话启动

要在获取[服务器托管设置](https://code.claude.com/docs/zh-CN/server-managed-settings.md)失败时阻止执行该获取的会话启动，请在运行器镜像中的 [`managed-settings.json`](https://code.claude.com/docs/zh-CN/settings#where-settings-live) 里将 [`forceRemoteSettingsRefresh`](https://code.claude.com/docs/zh-CN/settings-reference#forceremotesettingsrefresh) 设置为 `true`。如果 Anthropic 的 API 回复您的组织没有服务器托管设置，这也算作获取成功。

在添加该键之前，请检查该环境是否运行 Claude Tag 频道会话，或是否将模型请求发送到 Bedrock 或 Agent Platform：

* **Claude Tag 频道会话**：对于 Owner [将 Claude Tag 频道路由到](https://code.claude.com/docs/zh-CN/cloud-environments.md#set-the-environment-a-claude-tag-channel-uses)的环境中的运行器，请不要设置该键。Anthropic 的 API 会拒绝这些会话获取服务器托管设置的请求，因此设置该键后它们将无法启动。请将它们的规则放在运行器镜像的 `managed-settings.json` 中，[它们仍会读取该文件](https://code.claude.com/docs/zh-CN/managed-settings.md#where-and-when-a-policy-applies)。要为其他会话使用该键，请在单独的环境中运行这些会话。
* **Bedrock 或 Agent Platform**：使用这些提供商的会话[不会获取服务器托管设置](#what-differs-from-sessions-on-the-anthropic-api)，因此无论是否设置该键，它们都会启动。请改为通过同一个 `managed-settings.json` 强制执行您的规则。

以下 `managed-settings.json` 设置了该键：

```json managed-settings.json theme={null}
{
  "forceRemoteSettingsRefresh": true
}
```

即使您也在 claude.ai 管理控制台中设置了该键，也请在该文件中设置它。新会话没有您的服务器托管设置的[缓存副本](https://code.claude.com/docs/zh-CN/server-managed-settings.md#fetch-and-caching-behavior)，因此如果您只在控制台中设置该键，会话尚不具备该设置。如果此时获取失败，会话将在没有这些设置的情况下运行，直到后续获取成功为止。

要确认会话未启动是因为设置获取失败，请查看会话中或运行器日志中 [Claude Code 的最后输出](#keep-stdin-and-file-descriptor-3-attached)。其中会包含 `Your organization requires remote managed settings to load, but they could not be loaded.`

### 包含多个仓库的会话中的仓库设置

在[项目](https://code.claude.com/docs/zh-CN/claude-projects.md#what-threads-pick-up-from-your-repositories)中的线程里，无论线程在何处启动，都没有任何仓库的 `.claude/settings.json` 作为项目设置生效，也没有任何仓库的 `.mcp.json` 或 skill 会被加载。要为这些线程提供 skill，请将其添加到您的运行器镜像中。

在其他任何包含多个仓库的会话中，Claude Code 从会话启动所在的目录读取项目设置，因此最多只有一个仓库的 `.claude/settings.json` 作为项目设置生效。在其他仓库的文件中定义的 hook 不会运行，其中的拒绝规则不会生效，其 `env` 也不会被设置。

* **`--capacity 1`（默认值）并使用内置检出**：会话在其仓库列表中的第一个仓库中启动。该仓库的 `.claude/settings.json` 作为项目设置生效，其 `.mcp.json` 会被加载，而其他仓库的则不会。
* **`--capacity` 大于 1，或使用 [`checkout` hook](#checkout)**：会话在包含各检出内容的按会话目录中启动。没有任何仓库的 `.claude/settings.json` 作为项目设置生效，没有任何仓库的 `.mcp.json` 会被加载，并且 hook 命令中的 [`$CLAUDE_PROJECT_DIR`](https://code.claude.com/docs/zh-CN/hooks#reference-scripts-by-path) 是该目录，而不是某个检出目录。

无论会话在何处启动，每个仓库的 `CLAUDE.md` 都会被加载。在项目线程之外，其 skill 也会被加载。运行器将每个仓库作为[附加目录](https://code.claude.com/docs/zh-CN/permissions.md#additional-directories-grant-file-access-not-configuration)传递给 Claude Code，因此 Claude Code 还会从每个仓库的 `.claude/settings.json` 中读取 `enabledPlugins` 和 `extraKnownMarketplaces` 键。

要在每个会话中运行某个 hook 或应用某条权限规则，请将其放在运行器主机上的 `~/.claude/settings.json` 中。无论会话在何处启动，运行器都会[将该主机文件填充到每个会话中](#how-each-session’s-config-is-assembled)。在 `Read` 或 `Edit` 规则中，请将路径写为以 `//` 开头的绝对路径或以 `~/` 开头的相对于主目录的[模式](https://code.claude.com/docs/zh-CN/permissions.md#read-and-edit)，因为其他模式会以设置来源或当前目录为锚点。

### 仓库中提交的权限规则

不要在仓库中提交的 `permissions.allow` 中放置不带限定的 `"Edit"`、`"Write"` 或 `"NotebookEdit"` 条目。不带限定的文件工具规则会匹配该工具而不论路径如何，从而授予在主机上任意位置写入的权限，而不仅限于工作区，因此运行器的写入范围限制守卫会标记该会话；使用 [`--confine-repo-settings enforce`](https://code.claude.com/docs/zh-CN/self-hosted-environments-reference#runner-cli-flags) 时，它会拒绝生成该会话，而不是记录日志后继续。请参阅[加固部分](https://code.claude.com/docs/zh-CN/self-hosted-environments-deploy.md#harden-your-deployment)。

仓库根本不需要文件工具规则：云端会话[无论处于何种模式都会预批准文件编辑](https://code.claude.com/docs/zh-CN/permission-modes.md#switch-permission-modes)。如果您确实要提交规则，请将其限定到工作区，例如 `"Edit(/**)"`；单个前导斜杠相对于项目根目录，即会话的工作区。不带限定的文件工具规则可以放在操作员的主机级 `settings.json` 中，因为该文件不是在仓库中提交的。

`defaultMode` 为 `auto` 的设置仅在来自镜像范围或用户级设置文件时才会生效，因此检出的仓库无法为自己授予自动模式。有关云端会话接受哪些模式以及完整的规则语法，请参阅[权限模式](https://code.claude.com/docs/zh-CN/permission-modes.md)。

## 接下来

* [Reference](https://code.claude.com/docs/zh-CN/self-hosted-environments-reference.md)：每个 CLI 标志、环境变量和指标
* [Verify session identity](https://code.claude.com/docs/zh-CN/self-hosted-environments-identity.md)：从运行器外部的服务验证会话令牌
