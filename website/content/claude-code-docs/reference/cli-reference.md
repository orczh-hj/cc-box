> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# CLI 参考

> Claude Code 命令行界面的完整参考，包括命令和标志。

## CLI 命令

您可以使用这些命令启动会话、管道内容、恢复对话和管理更新：

| 命令 | 描述 | 示例 |
| :- | :- | :- |
| `claude` | 启动交互式会话 | `claude` |
| `claude "query"` | 使用初始提示词启动交互式会话 | `claude "explain this project"` |
| `claude -p "query"` | 通过 SDK 查询，然后退出 | `claude -p "explain this function"` |
| `cat file \| claude -p "query"` | 处理管道内容 | `cat logs.txt \| claude -p "explain"` |
| `claude -c` | 在当前目录中继续最近的对话 | `claude -c` |
| `claude -c -p "query"` | 通过 SDK 继续 | `claude -c -p "Check for type errors"` |
| `claude -r "<session>" "query"` | 按 ID 或名称恢复会话 | `claude -r "auth-refactor" "Finish this PR"` |
| `claude update` | 更新到最新版本 | `claude update` |
| `claude gateway` | 启动自托管 [Claude apps gateway](https://code.claude.com/docs/zh-CN/claude-apps-gateway) 服务器，供在 Amazon Bedrock、Google Cloud 的 Agent Platform 或 Microsoft Foundry 上部署 SSO 和策略在 Claude Code 前面的管理员使用。需要 `--config` 指向 [`gateway.yaml`](https://code.claude.com/docs/zh-CN/claude-apps-gateway-config)。 | `claude gateway --config gateway.yaml` |
| `claude install [version]` | 安装或重新安装本机二进制文件。接受版本号如 `2.1.118`、`stable` 或 `latest`。请参阅 [安装特定版本](https://code.claude.com/docs/zh-CN/setup.md#install-a-specific-version) | `claude install stable` |
| `claude auth login` | 登录您的 Anthropic 账户。使用 `--email` 预填充您的电子邮件地址，使用 `--sso` 强制 SSO 身份验证，使用 `--console` 使用 Anthropic Console 登录以进行 API 使用计费而不是 Claude 订阅 | `claude auth login --console` |
| `claude auth logout` | 从您的 Anthropic 账户登出 | `claude auth logout` |
| `claude auth status` | 以 JSON 格式显示身份验证状态。使用 `--text` 获取人类可读的输出。如果已登录，则以代码 0 退出，如果未登录，则以代码 1 退出。JSON 包含一个 `configDirectory` 字段，命名 CLI 使用的 [配置目录](https://code.claude.com/docs/zh-CN/claude-directory.md)。该字段需要 Claude Code v2.1.268 或更高版本。JSON 的 `authMethod` 字段取值为 `none`、`claude.ai`、`oauth_token`、`api_key`、`api_key_helper` 或 `third_party` 之一 | `claude auth status` |
| `claude agents` | 打开 [agent view](https://code.claude.com/docs/zh-CN/agent-view.md) 以监控和分派并行后台会话。使用 `--cwd <path>` 仅显示在该目录下启动的会话，或使用 `--json` 将活动会话打印为 JSON 数组以供脚本使用（`--json --all` 也包括已完成的后台会话）。传递 `--permission-mode`、`--model`、`--effort` 或 `--agent` 以设置 [分派会话的默认值](https://code.claude.com/docs/zh-CN/agent-view.md#permission-mode-model-and-effort)。接受 `--settings`、`--add-dir`、`--plugin-dir` 和 `--mcp-config`，如顶级 `claude` 命令。打开 agent view 需要交互式终端 | `claude agents --json` |
| `claude attach <id\|name>` | 在此终端中附加到 [后台会话](https://code.claude.com/docs/zh-CN/agent-view.md#manage-sessions-from-the-shell)。传递正在运行的会话名称的一部分来代替 ID 需要 Claude Code v2.1.290 或更高版本 | `claude attach 7c5dcf5d` |
| `claude auto-mode defaults` | 以 JSON 格式打印内置 [自动模式](https://code.claude.com/docs/zh-CN/permission-modes.md#eliminate-prompts-with-auto-mode) 分类器规则。使用 `claude auto-mode config` 查看应用了设置的有效配置。`--label <prefix>` 仅打印标签以该前缀开头的规则，不区分大小写匹配。需要 Claude Code v2.1.208 或更高版本 | `claude auto-mode defaults --label 'Git Destructive'` |
| `claude auto-mode reset` | 通过从用户设置文件中删除 `autoMode` 部分来恢复默认 [自动模式](https://code.claude.com/docs/zh-CN/permission-modes.md#eliminate-prompts-with-auto-mode) 配置。在写入前提示确认；传递 `-y`/`--yes` 以跳过提示。来自 [托管设置](https://code.claude.com/docs/zh-CN/server-managed-settings.md) 或 `--settings` 标志的规则仍然适用。需要 Claude Code v2.1.212 或更高版本。请参阅 [检查默认值和您的有效配置](https://code.claude.com/docs/zh-CN/auto-mode-config.md#inspect-the-defaults-and-your-effective-config) | `claude auto-mode reset --yes` |
| `claude daemon logs` | 跟踪后台会话 [supervisor](https://code.claude.com/docs/zh-CN/agent-view.md#the-supervisor-process) 的日志文件 `~/.claude/daemon.log`，在新行到达时将其打印出来，直到您按下 `Ctrl+C` | `claude daemon logs` |
| `claude daemon run` | 在此终端的前台运行后台会话 [supervisor](https://code.claude.com/docs/zh-CN/agent-view.md#the-supervisor-process)，并打印其日志 | `claude daemon run` |
| `claude daemon status` | 打印后台会话 [supervisor](https://code.claude.com/docs/zh-CN/agent-view.md#the-supervisor-process) 的状态、版本、套接字目录和工作进程数以进行诊断。如果 supervisor 未运行，则退出代码 1 | `claude daemon status` |
| `claude daemon stop --any` | 停止后台会话 [supervisor](https://code.claude.com/docs/zh-CN/agent-view.md#the-supervisor-process) 及其托管的会话。传递 `--keep-workers` 以保持后台会话运行，以便下一个 supervisor 重新连接到它们。`--any` 确认停止按需 supervisor，这是默认值。使用此命令从 [无响应的 supervisor](https://code.claude.com/docs/zh-CN/agent-view.md#agent-view-says-the-background-service-did-not-respond) 恢复 | `claude daemon stop --any --keep-workers` |
| `claude doctor` | 从终端打印只读安装和设置诊断，无需启动会话，包括安装健康状况、设置文件验证错误和 Remote Control 资格。对于可以应用修复的会话内设置检查，请运行 [`/doctor`](https://code.claude.com/docs/zh-CN/commands#all-commands) | `claude doctor` |
| `claude import [source]` | 启动交互式会话，运行 [`/import`](https://code.claude.com/docs/zh-CN/commands#all-commands) 以将来自其他编码 Agent 的配置引入 Claude Code。接受与命令相同的 `--dry-run` 和 `--yes` 选项。在 Amazon Bedrock、Google Cloud 的 Agent Platform、Microsoft Foundry 或 AWS 上的 Claude Platform 上不可用。当您关闭 [功能标志获取](https://code.claude.com/docs/zh-CN/env-vars.md#features-that-need-feature-flag-fetching) 时也不可用。需要 Claude Code v2.1.213 或更高版本 | `claude import codex --dry-run` |
| `claude logs <id\|name>` | 从 [后台会话](https://code.claude.com/docs/zh-CN/agent-view.md#manage-sessions-from-the-shell) 打印最近的输出。传递会话名称的一部分来代替 ID 需要 Claude Code v2.1.290 或更高版本 | `claude logs 7c5dcf5d` |
| `claude mcp` | 配置 Model Context Protocol (MCP) 服务器 | 请参阅 [Claude Code MCP 文档](https://code.claude.com/docs/zh-CN/mcp.md)。 |
| `claude mcp login <name>` | 运行配置的 MCP 服务器的 OAuth 流程而不打开交互式 `/mcp` 面板。适用于 HTTP、SSE 和 claude.ai 连接器服务器。在 SSH 上添加 `--no-browser` 以打印授权 URL 而不是打开浏览器。对于 HTTP 或 SSE 服务器，请在提示处粘贴重定向 URL。对于 claude.ai 连接器，请参阅 [从 shell 重新授权连接器](https://code.claude.com/docs/zh-CN/remote-control.md#authorize-a-connector-again-from-your-shell)。请参阅 [从命令行进行身份验证](https://code.claude.com/docs/zh-CN/mcp.md#authenticate-from-the-command-line) | `claude mcp login sentry` |
| `claude mcp logout <name>` | 清除 MCP 服务器的存储 OAuth 凭据 | `claude mcp logout sentry` |
| `claude plugin` | 管理 Claude Code [插件](https://code.claude.com/docs/zh-CN/plugins/overview.md)。别名：`claude plugins`。请参阅 [插件参考](https://code.claude.com/docs/zh-CN/plugins/cli-reference.md#claude-plugin-commands) 了解子命令 | `claude plugin install code-review@claude-plugins-official` |
| `claude purge [path]` | 删除项目的所有本地 Claude Code 状态：会话记录、任务列表、调试日志、文件编辑历史、提示词历史行和项目在 `~/.claude.json` 中的条目。省略 `[path]` 以从交互式列表中选择。标志：`--dry-run` 预览，`-y`/`--yes` 跳过确认，`-i`/`--interactive` 确认每一项，`--all` 用于每个项目。请参阅 [清除本地数据](https://code.claude.com/docs/zh-CN/claude-directory.md#clear-local-data) | `claude purge ~/work/repo --dry-run` |
| `claude remote-control` | 启动 [Remote Control](https://code.claude.com/docs/zh-CN/remote-control.md) 服务器以从 Claude.ai 或 Claude 应用控制 Claude Code。在服务器模式下运行（无本地交互式会话）。请参阅 [服务器模式标志](https://code.claude.com/docs/zh-CN/remote-control.md#start-a-remote-control-session)。停止服务器后，您可以恢复它正在服务的会话。请参阅 [停止服务器后恢复会话](https://code.claude.com/docs/zh-CN/remote-control.md#resume-sessions-after-stopping-the-server) | `claude remote-control --name "My Project"` |
| `claude respawn <id>` | 重启 [后台会话](https://code.claude.com/docs/zh-CN/agent-view.md#manage-sessions-from-the-shell)，运行或已停止，保持其对话完整。使用 `--all` 重启每个运行中的会话，例如以获取更新的 Claude Code 二进制文件 | `claude respawn 7c5dcf5d` |
| `claude rm <id>` | 从列表中删除 [后台会话](https://code.claude.com/docs/zh-CN/agent-view.md#manage-sessions-from-the-shell)。当删除因 [会话的 worktree 而被拒绝](https://code.claude.com/docs/zh-CN/agent-view.md#what-deleting-a-session-removes) 且第二个 `claude rm` 可以解决它时，拒绝会打印要传递的确切标志和值：`--discard-unpushed <commit>@<worktree-id>` 丢弃具有未推送提交的 worktree 及其提交，`--force-remove-worktree <worktree-id>` 删除 git 或 `WorktreeRemove` hook 无法删除的 worktree 目录。`--discard-unpushed` 需要 Claude Code v2.1.260 或更高版本，而 `--force-remove-worktree` 需要 v2.1.268 或更高版本。会话记录保留在您的本地计算机上，可通过 `claude --resume` 访问 | `claude rm 7c5dcf5d` |
| `claude self-hosted-runner` | 启动运行程序进程，将此计算机或容器注册到 [self-hosted environment](https://code.claude.com/docs/zh-CN/self-hosted-environments)，并在您的基础设施上托管 Claude Code 云端会话。运行 `claude self-hosted-runner setup` 以获得引导式操作员演练，运行 `claude self-hosted-runner doctor` 以 [诊断已部署的运行程序](https://code.claude.com/docs/zh-CN/self-hosted-environments-deploy.md#troubleshooting)，运行 `claude self-hosted-runner orchestrator` 以生成 [on-demand runners](https://code.claude.com/docs/zh-CN/self-hosted-environments-configuration.md#on-demand-runners)。需要 Claude Code v2.1.224 或更高版本 | `claude self-hosted-runner setup` |
| `claude setup-token` | 为 CI 和脚本生成长期 OAuth 令牌。将令牌打印到终端而不保存。需要 Claude 订阅。请参阅 [生成长期令牌](https://code.claude.com/docs/zh-CN/authentication.md#generate-a-long-lived-token) | `claude setup-token` |
| `claude stop <id>` | 停止 [后台会话](https://code.claude.com/docs/zh-CN/agent-view.md#manage-sessions-from-the-shell)。也接受 `claude kill` | `claude stop 7c5dcf5d` |
| `claude ultrareview [target]` | 非交互式运行 [ultrareview](https://code.claude.com/docs/zh-CN/ultrareview.md#run-ultrareview-non-interactively)。将发现结果打印到标准输出，成功时退出代码 0，失败时退出代码 1。使用 `--json` 获取原始负载，使用 `--timeout <minutes>` 覆盖 45 分钟的默认值。在 `github.com` Pull Request 目标上使用 `--post` 以将完成的发现结果作为来自您的 GitHub 账户的一条纯文本评论发布到 PR。`--no-post` 是默认值。`--post` 和 `--no-post` 需要 Claude Code v2.1.227 或更高版本。请参阅 [将发现结果发布到 Pull Request](https://code.claude.com/docs/zh-CN/ultrareview.md#post-findings-to-the-pull-request) | `claude ultrareview 1234 --json` |

如果您输入错误的子命令，Claude Code 会建议最接近的匹配项并退出而不启动会话。例如，`claude udpate` 会打印 `Did you mean claude update?`。

`claude --dangerously-skip-permissions daemon <subcommand>` 会运行 `daemon` 子命令，因此当 `claude` 的别名包含该标志时，子命令仍可正常工作。只有前导的 `--dangerously-skip-permissions` 或 `--allow-dangerously-skip-permissions` 会以这种方式路由到 `daemon`；如果 `daemon` 前面有任何其他标志，子命令将不会运行。

## CLI 标志

使用这些命令行标志自定义 Claude Code 的行为。`claude --help` 不会列出每个标志，因此标志在 `--help` 中不出现并不意味着它不可用。

| 标志 | 描述 | 示例 |
| :- | :- | :- |
| `--add-dir` | 添加额外的工作目录供 Claude 读取和编辑文件。授予文件访问权限；Claude Code [不会发现](https://code.claude.com/docs/zh-CN/permissions.md#additional-directories-grant-file-access-not-configuration)这些目录中的大多数 `.claude/` 配置。验证每个路径是否作为目录存在。您不能添加大多数[网络路径](https://code.claude.com/docs/zh-CN/errors.md#working-directory-is-a-network-path)，例如 `\\server\share`。要在会话之间保持这些目录，请在设置中设置 [`permissions.additionalDirectories`](https://code.claude.com/docs/zh-CN/settings-reference#permissions-additionaldirectories) | `claude --add-dir ../apps ../lib` |
| `--advisor <model>` | 使用模型别名 `fable`、`opus` 或 `sonnet`，或完整模型 ID 为此会话启用服务器端[顾问工具](https://code.claude.com/docs/zh-CN/advisor.md)。优先于会话的 `advisorModel` 设置。`fable` 需要 [Fable 访问权限](https://code.claude.com/docs/zh-CN/advisor.md#choose-an-advisor-model) | `claude --advisor opus` |
| `--agent` | 为当前会话指定 Agent（覆盖 `agent` 设置） | `claude --agent my-custom-agent` |
| `--agents` | 通过 JSON 动态定义自定义子代理。接受[为 CLI 定义的子代理列出的字段](https://code.claude.com/docs/zh-CN/sub-agents.md#choose-the-subagent-scope)。使用 `--print` 时，该值可以是包含该对象的 JSON 文件的路径；文件形式需要 Claude Code v2.1.281 或更高版本。Claude Code 在启动时验证该值并在值无效时退出；有关消息以及跳过验证的标志和环境变量，请参阅 [`Invalid --agents configuration`](https://code.claude.com/docs/zh-CN/errors#invalid-agents-configuration)。验证需要 Claude Code v2.1.242 或更高版本 | `claude --agents '{"reviewer":{"description":"Reviews code","prompt":"You are a code reviewer"}}'` |
| `--allow-dangerously-skip-permissions` | 将 `bypassPermissions` 添加到 `Shift+Tab` 模式循环中而不以该模式启动。让您可以从不同的模式（如 `plan`）开始，稍后切换到 `bypassPermissions`。请参阅[权限模式](https://code.claude.com/docs/zh-CN/permission-modes.md#skip-all-checks-with-bypasspermissions-mode) | `claude --permission-mode plan --allow-dangerously-skip-permissions` |
| `--allowedTools`, `--allowed-tools` | 无需提示权限即可执行的工具，但从[网络路径](https://code.claude.com/docs/zh-CN/permissions.md#network-paths)读取除外。有关模式匹配，请参阅[权限规则语法](https://code.claude.com/docs/zh-CN/settings-reference.md#permission-rule-syntax)。要限制哪些工具可用，请改用 `--tools`。如果您在此处指定[任务跟踪工具](https://code.claude.com/docs/zh-CN/tools-reference.md#task-tool-availability)之一，Claude Code 也会让会话选择加入 | `"Bash(git log *)" "Bash(git diff *)" "Read"` |
| `--append-subagent-system-prompt` | 将自定义文本附加到每个[子代理](https://code.claude.com/docs/zh-CN/sub-agents.md)的系统提示词末尾，包括嵌套子代理，但[分叉的子代理](https://code.claude.com/docs/zh-CN/sub-agents.md#fork-the-current-conversation)除外，它重用对话自身的提示词。仅在使用 `-p` 的非交互模式下应用。需要 Claude Code v2.1.205 或更高版本 | `claude -p --append-subagent-system-prompt "Cite file paths in every answer" "query"` |
| `--append-subagent-system-prompt-file` | 从文件加载文本并将其附加到[子代理](https://code.claude.com/docs/zh-CN/sub-agents.md)系统提示词。当文本过长无法在命令行上传递时，可作为 `--append-subagent-system-prompt` 的替代方案。这两个标志不能组合使用。仅在使用 `-p` 的非交互模式下应用。需要 Claude Code v2.1.261 或更高版本 | `claude -p --append-subagent-system-prompt-file ./subagent-rules.txt "query"` |
| `--append-system-prompt` | 将自定义文本附加到默认系统提示词的末尾 | `claude --append-system-prompt "Always use TypeScript"` |
| `--append-system-prompt-file` | 从文件加载额外的系统提示词文本并附加到默认提示词 | `claude --append-system-prompt-file ./extra-rules.txt` |
| `--autocompact <auto\|tokens>` | 为此会话设置[自动压缩窗口](https://code.claude.com/docs/zh-CN/model-config.md#set-the-auto-compact-window)而不更改您保存的设置。接受与 `/autocompact` 相同的值；该部分涵盖值形式以及哪些内容会覆盖此标志。需要 Claude Code v2.1.221 或更高版本 | `claude --autocompact 500k` |
| `--ax-screen-reader` | 呈现屏幕阅读器友好的输出：没有装饰性边框或动画的平面文本。强制使用经典渲染器，因此 [`tui`](https://code.claude.com/docs/zh-CN/settings-reference#tui) 设置无效；附加的[后台会话](https://code.claude.com/docs/zh-CN/agent-view.md)仍然全屏呈现。优先于 [`CLAUDE_AX_SCREEN_READER`](https://code.claude.com/docs/zh-CN/env-vars) 和 [`axScreenReader`](https://code.claude.com/docs/zh-CN/settings-reference#axscreenreader) 设置。需要 Claude Code v2.1.181 或更高版本 | `claude --ax-screen-reader` |
| `--bare` | 最小模式：跳过 hook、skill、自定义命令、子代理、已安装插件、MCP 服务器、自动记忆和 CLAUDE.md 的自动发现，以便脚本化调用启动更快。使用 `--add-dir` 传递的目录中的 skill 仍然加载。Claude 可以访问 Bash、文件读取和文件编辑工具。设置 [`CLAUDE_CODE_SIMPLE`](https://code.claude.com/docs/zh-CN/env-vars)。请参阅 [bare 模式](https://code.claude.com/docs/zh-CN/headless.md#start-faster-with-bare-mode) | `claude --bare -p "query"` |
| `--betas` | 要包含在 API 请求中的 Beta 标头（仅限 API 密钥用户） | `claude --betas interleaved-thinking` |
| `--bg`, `--background` | 将会话作为[后台 Agent](https://code.claude.com/docs/zh-CN/agent-view.md) 启动并立即返回。打印会话 ID 和管理命令。与 `--exec` 结合以将 shell 命令作为后台作业运行，而不是启动 Claude 会话，或与 `--agent` 结合以运行特定的子代理。在启动前检查目录的[工作区信任](https://code.claude.com/docs/zh-CN/permissions.md#project-allow-rules-and-workspace-trust)。不能与 `-p`/`--print` 结合；请参阅[错误参考](https://code.claude.com/docs/zh-CN/errors.md#command-line-errors) | `claude --bg "investigate the flaky test"` |
| `--channels` | （研究预览）Claude 应在此会话中侦听其[频道](https://code.claude.com/docs/zh-CN/channels.md)通知的 MCP 服务器。空格分隔的 `plugin:<name>@<marketplace>` 条目列表。需要通过 claude.ai 或 Console API 密钥进行 Anthropic 身份验证 | `claude --channels plugin:my-notifier@my-marketplace` |
| `--chrome` | 启用 [Chrome 浏览器集成](https://code.claude.com/docs/zh-CN/chrome.md)以进行网络自动化和测试 | `claude --chrome` |
| `--cloud` | 使用任务描述创建新的[云端会话](https://code.claude.com/docs/zh-CN/claude-code-on-the-web.md)。使用会话 ID（`session_...` 或 `cse_...`）或 claude.ai/code URL 时，则配合 `-p` 将消息排队到该现有会话。请参阅[发送后续消息](https://code.claude.com/docs/zh-CN/claude-code-on-the-web.md#send-follow-ups-from-the-cli)。 | `claude --cloud "Fix the login bug"` |
| `--continue`, `-c` | 加载当前目录中最近的对话，包括[已完成的后台会话](https://code.claude.com/docs/zh-CN/sessions.md#where-the-session-picker-looks)；打开已完成的后台会话需要 Claude Code v2.1.257 或更高版本。跳过使用 `claude -p` 或 Agent SDK 创建的会话，以及第一个提示词为 `/loop` 的会话。`claude -p --continue` 包括 `-p`、SDK 和 `/loop` 会话。包括使用 `/add-dir` 添加此目录的会话 | `claude --continue` |
| `--dangerously-load-development-channels` | 启用不在批准的允许列表上的[频道](https://code.claude.com/docs/zh-CN/channels-reference.md#test-during-the-research-preview)，用于本地开发。接受 `plugin:<name>@<marketplace>` 和 `server:<name>` 条目。会提示确认，因此它在交互式会话中生效。使用 `-p` 时，Claude Code 会忽略此标志 | `claude --dangerously-load-development-channels server:webhook` |
| `--dangerously-skip-permissions` | 跳过权限提示。等同于 `--permission-mode bypassPermissions`。请参阅[权限模式](https://code.claude.com/docs/zh-CN/permission-modes.md#skip-all-checks-with-bypasspermissions-mode)了解此操作跳过和不跳过的内容。对于使用 `--bg` 启动的会话，当主管重新启动会话时，该模式[持续存在](https://code.claude.com/docs/zh-CN/agent-view.md#permission-mode-model-and-effort) | `claude --dangerously-skip-permissions` |
| `--debug` | 启用调试模式，可选类别过滤，例如 `--debug='mcp,startup'` 或 `--debug='!mcp'`。过滤器仅在 `=` 形式中绑定；空格分隔的过滤器启用调试模式而不进行过滤 | `claude --debug='mcp,startup'` |
| `--debug-file <path>` | 将调试日志写入特定文件路径。隐式启用调试模式。优先于 `CLAUDE_CODE_DEBUG_LOGS_DIR` | `claude --debug-file /tmp/claude-debug.log` |
| `--desktop` | 在当前目录上打开 [Claude Desktop 应用](https://code.claude.com/docs/zh-CN/desktop.md)并退出，而不在终端中启动会话。添加 `--continue`，或带会话 ID 的 `--resume`，可改为[在 Desktop 中打开该会话](https://code.claude.com/docs/zh-CN/desktop.md#coming-from-the-cli)。此处的 `--resume` 仅接受会话 ID，不接受名称或会话记录路径。不接受提示词，也不接受除 `--verbose` 和 `--debug` 标志外的其他标志，因为应用会自行启动会话。当您使用 Claude 订阅登录时，可在 macOS 和 x64 Windows 上使用。需要 Claude Code v2.1.285 或更高版本 | `claude --desktop` |
| `--disable-slash-commands` | 为此会话禁用所有 skill 和命令 | `claude --disable-slash-commands` |
| `--disallowedTools`, `--disallowed-tools` | 拒绝规则。裸工具名称从 Claude 的上下文中删除匹配的工具：`"Edit"` 删除 Edit，`"*"` 删除每个工具，`"mcp__*"` 删除每个 MCP 工具。作用域规则（如 `Bash(rm *)`）使工具保持可用，仅拒绝[按字面](https://code.claude.com/docs/zh-CN/permissions.md#bash-rule-limits)匹配的调用。只要还有任何其他工具保留，命名 [`EndConversation`](https://code.claude.com/docs/zh-CN/tools-reference#endconversation-tool-behavior) 的规则就无法删除它 | `"Bash(git log *)" "Bash(git diff *)" "Edit"` |
| `--effort` | 为当前会话设置 [effort 级别](https://code.claude.com/docs/zh-CN/model-config.md#adjust-effort-level)。选项：`low`、`medium`、`high`、`xhigh`、`max` 或 `ultracode`。可用级别取决于模型。`ultracode` 请求 `xhigh` effort 并开启 [ultracode](https://code.claude.com/docs/zh-CN/workflows.md#let-claude-decide-with-ultracode)，需要 Claude Code v2.1.203 或更高版本。覆盖此会话的 [`modelSettings`](https://code.claude.com/docs/zh-CN/settings-reference#modelsettings) 和 [`effortLevel`](https://code.claude.com/docs/zh-CN/settings-reference#effortlevel) 设置，不会持久保存 | `claude --effort high` |
| `--enable-auto-mode` | 在 v2.1.111 中删除。自动模式现在默认在 `Shift+Tab` 循环中；使用 `--permission-mode auto` 以该模式启动 | `claude --permission-mode auto` |
| `--environment <environment-id>` | 创建在具有给定 ID 的[自托管环境](https://code.claude.com/docs/zh-CN/self-hosted-environments)上运行的新云端会话。环境 ID 以 `ccpool_` 开头。有关调度行为和它拒绝的标志组合，请参阅 [`--environment` 调度行为](https://code.claude.com/docs/zh-CN/self-hosted-environments-testing#environment-dispatch-behavior)。需要 Claude Code v2.1.224 或更高版本 | `claude -p "Fix the login bug" --environment ccpool_abc123` |
| `--exclude-dynamic-system-prompt-sections` | 将每用户上下文（如自动记忆位置）从系统提示词移到第一条用户消息中。改进在运行相同任务的不同用户和机器之间的提示词缓存重用。仅适用于默认系统提示词；当设置 `--system-prompt` 或 `--system-prompt-file` 时忽略。与 `-p` 一起用于脚本化的多用户工作负载 | `claude -p --exclude-dynamic-system-prompt-sections "query"` |
| `--exec` | 将 shell 命令作为 PTY 支持的后台作业运行，而不是启动 Claude 会话。与 `--bg` 一起使用以从 shell 启动 | `claude --bg --exec 'pytest -x'` |
| `--fallback-model` | 启用当主模型过载或不可用（例如已停用的模型）时自动回退到指定的模型。接受按顺序尝试的逗号分隔列表。请参阅[备用模型链](https://code.claude.com/docs/zh-CN/model-config.md#fallback-model-chains)。要在会话之间保持模型链，请使用 [`fallbackModel` 设置](https://code.claude.com/docs/zh-CN/settings-reference#fallbackmodel)，此标志会覆盖该设置 | `claude --fallback-model sonnet,haiku` |
| `--fork-session` | 恢复时，创建新的会话 ID 而不是重用原始 ID（与 `--resume` 或 `--continue` 一起使用） | `claude --resume abc123 --fork-session` |
| `--forward-subagent-text` | 在输出流中将[子代理](https://code.claude.com/docs/zh-CN/sub-agents.md)文本和思考块作为设置了 `parent_tool_use_id` 的 `assistant` 和 `user` 消息发出，以便您可以重建每个子代理的会话记录。没有此标志，Claude Code 会省略在[前台](https://code.claude.com/docs/zh-CN/sub-agents.md#run-subagents-in-foreground-or-background)运行的子代理的文本和思考块。需要 `--print` 和 `--output-format stream-json`。有关嵌套子代理、分叉 skill 以及各自所需的版本，请参阅[跟踪子代理消息](https://code.claude.com/docs/zh-CN/headless.md#follow-subagent-messages)。[`CLAUDE_CODE_FORWARD_SUBAGENT_TEXT`](https://code.claude.com/docs/zh-CN/env-vars) 环境变量启用相同的行为。需要 Claude Code v2.1.211 或更高版本 | `claude -p --output-format stream-json --verbose --forward-subagent-text "query"` |
| `--from-pr` | 打开会话选择器，过滤到链接到特定 Pull Request 的会话。接受 PR 编号、GitHub 或 GitHub Enterprise PR URL、GitLab merge request URL 或 Bitbucket Pull Request URL。当 Claude 创建 Pull Request 时，会话会自动链接 | `claude --from-pr 123` |
| `--ide` | 如果恰好有一个有效的 IDE 可用，在启动时自动连接到 IDE | `claude --ide` |
| `--init` | 在会话之前使用 `init` 匹配器运行 [Setup hook](https://code.claude.com/docs/zh-CN/hooks.md#setup)（仅限 print 模式） | `claude -p --init "query"` |
| `--init-only` | 运行 [Setup](https://code.claude.com/docs/zh-CN/hooks.md#setup) 和 `SessionStart` hook，然后退出而不启动对话 | `claude --init-only` |
| `--include-hook-events` | 在输出流中包含 hook 生命周期事件。`SessionStart` 和 `Setup` hook 事件始终包含，不需要此标志。某些 hook 事件（如 `Notification`、`SessionEnd`、`PreCompact` 和 `PostCompact`）永远不会产生 `hook_started` 事件，即使使用此标志也是如此。对于这些事件，当运行超过一秒的命令 hook 产生输出时，Claude Code 仍会发出 `hook_progress`，并仅在[在后台运行的 hook](https://code.claude.com/docs/zh-CN/hooks.md#run-hooks-in-the-background) 完成时发出 `hook_response`。需要 `--output-format stream-json` | `claude -p --output-format stream-json --verbose --include-hook-events "query"` |
| `--include-partial-messages` | 在输出中包含部分流式事件。需要 `--print` 和 `--output-format stream-json` | `claude -p --output-format stream-json --verbose --include-partial-messages "query"` |
| `--input-format` | 为 print 模式指定输入格式（选项：`text`、`stream-json`） | `claude -p --output-format json --input-format stream-json` |
| `--json-schema` | 在 Agent 完成其工作流后获得与 JSON Schema 匹配的经过验证的 JSON 输出（仅限 print 模式）。请参阅[结构化输出](https://code.claude.com/docs/zh-CN/agent-sdk/structured-outputs.md)。Claude Code 在 schema 无效时以错误退出，并接受 `format` 关键字作为注释而不进行客户端验证 | `claude -p --json-schema '{"type":"object","properties":{...}}' "query"` |
| `--maintenance` | 在会话之前使用 `maintenance` 匹配器运行 [Setup hook](https://code.claude.com/docs/zh-CN/hooks.md#setup)（仅限 print 模式） | `claude -p --maintenance "query"` |
| `--max-budget-usd` | 一旦 API 调用的估算支出达到此金额，即停止运行（仅限 print 模式）。Claude Code 根据其[客户端成本估算](https://code.claude.com/docs/zh-CN/agent-sdk/cost-tracking.md#estimates-not-billing)检查上限，该估算可能与您的账单不同。来自[子代理](https://code.claude.com/docs/zh-CN/sub-agents.md)的支出计入上限。支出可能超过上限，因此请[预留余量](https://code.claude.com/docs/zh-CN/agent-sdk/agent-loop.md#budget-headroom)。当您使用 `--continue` 或 `--resume` 返回对话时，[从早期运行恢复的](https://code.claude.com/docs/zh-CN/agent-sdk/cost-tracking.md#accumulate-costs-across-multiple-calls)总数不计入上限。一旦支出达到上限，生成另一个子代理会失败并显示 `Budget limit reached`，Claude Code 会停止仍在运行的后台子代理；上限执行行为需要 Claude Code v2.1.217 或更高版本 | `claude -p --max-budget-usd 5.00 "query"` |
| `--max-turns` | 限制 Agent 轮次数（仅限 print 模式）。达到限制时以错误退出。默认无限制。使用 `--input-format stream-json` 时，当限制结束某一轮次时仍在排队的消息会保持排队，并以其自己的限制启动新轮次 | `claude -p --max-turns 3 "query"` |
| `--mcp-config` | 从 JSON 文件或字符串加载 MCP 服务器（空格分隔）。当您使用 `-p` 传递此标志时，Claude Code 在运行第一轮之前等待仍待处理的服务器连接，最多等待 [`MCP_TIMEOUT`](https://code.claude.com/docs/zh-CN/env-vars) 启动超时时间，默认 30 秒；具有[缓存工具列表](https://code.claude.com/docs/zh-CN/mcp.md#managing-your-servers)的服务器跳过等待并在首次使用时连接。在[自托管环境](https://code.claude.com/docs/zh-CN/self-hosted-environments-configuration.md#connection-timing)中，则改为适用较短的等待时间。等待需要 Claude Code v2.1.221 或更高版本 | `claude --mcp-config ./mcp.json` |
| `--model` | 使用[模型别名](https://code.claude.com/docs/zh-CN/model-config.md#model-aliases)（如 `sonnet`、`opus`、`haiku` 或 `fable`）或模型的完整名称为当前会话设置模型。覆盖 [`model`](https://code.claude.com/docs/zh-CN/settings-reference#model) 设置和 [`ANTHROPIC_MODEL`](https://code.claude.com/docs/zh-CN/model-config#environment-variables) | `claude --model claude-sonnet-5` |
| `--name`, `-n` | 为会话设置显示名称，显示在 `/resume` 和终端标题中。您可以使用 `claude --resume <name>` 恢复命名会话。 [`/rename`](https://code.claude.com/docs/zh-CN/commands) 在会话中途更改名称，并且还会在提示栏上显示它 | `claude -n "my-feature-work"` |
| `--no-chrome` | 为此会话禁用 [Chrome 浏览器集成](https://code.claude.com/docs/zh-CN/chrome.md) | `claude --no-chrome` |
| `--no-session-persistence` | 禁用会话持久性，以便会话不保存到磁盘且无法恢复。仅限 print 模式。[`CLAUDE_CODE_SKIP_PROMPT_HISTORY`](https://code.claude.com/docs/zh-CN/env-vars) 环境变量在任何模式下执行相同操作 | `claude -p --no-session-persistence "query"` |
| `--output-format` | 为 print 模式指定输出格式（选项：`text`、`json`、`stream-json`） | `claude -p "query" --output-format json` |
| `--permission-mode` | 在指定的[权限模式](https://code.claude.com/docs/zh-CN/permission-modes.md)中开始。接受 `default`、`acceptEdits`、`plan`、`auto`、`dontAsk`、`bypassPermissions` 或 `manual`（作为 `default` 的别名）。`manual` 别名选择 UI 标记为 Manual 的权限模式，需要 Claude Code v2.1.200 或更高版本；`claude --help` 列出它来代替 `default`，两个值都有效。覆盖设置文件中的 `defaultMode`。没有此标志或 `--dangerously-skip-permissions` 时，新会话以[会话以哪种权限模式启动](https://code.claude.com/docs/zh-CN/permission-modes.md#which-mode-a-session-starts-in)中描述的权限模式启动，该部分也涵盖了 `-p` 运行以何种模式启动 | `claude --permission-mode plan` |
| `--permission-prompt-tool` | 指定 MCP 工具以在非交互模式下处理权限提示。传递本次运行所加载的 MCP 服务器上某个工具的完整名称，例如 `mcp__<server>__<tool>`。Claude Code 在运行第一轮之前等待该工具的 MCP 服务器连接，最多等待 [`MCP_TIMEOUT`](https://code.claude.com/docs/zh-CN/env-vars) 启动超时时间，默认 30 秒。 该提示工具无法批准标记为[需要用户交互](https://code.claude.com/docs/zh-CN/mcp.md#require-approval-for-a-specific-tool)的 MCP 工具：Claude Code 会将此类工具的 `allow` 结果转换为拒绝。此限制需要 Claude Code v2.1.199 或更高版本 | `claude -p --mcp-config ./mcp.json --permission-prompt-tool mcp__permissions__approve "query"` |
| `--permission-prompts` | 在 print 模式下设置由谁回答权限提示。使用默认值 `host` 时，Claude Code 将它们发送到 Agent SDK 主机或 `--permission-prompt-tool` 工具。当没有人可以回答时传递 `none`，Claude Code 会改为拒绝它们。请参阅[在无人值守运行中关闭权限提示](https://code.claude.com/docs/zh-CN/headless.md#turn-off-permission-prompts-in-unattended-runs)。需要 Claude Code v2.1.259 或更高版本 | `claude -p --permission-prompts none "query"` |
| `--plugin-dir` | 从目录或 `.zip` 存档加载插件，或从[插件文件夹](https://code.claude.com/docs/zh-CN/plugins/create.md#load-a-directory-or-archive-for-one-session)加载多个插件，仅用于此会话。每个标志接受一个路径。重复标志以传递更多路径：`--plugin-dir A --plugin-dir B.zip`。传递插件文件夹需要 Claude Code v2.1.265 或更高版本 | `claude --plugin-dir ./my-plugin` |
| `--plugin-url` | 从 URL 获取插件 `.zip` 存档，仅用于此会话。重复标志以获取多个插件，或在单个带引号的值中传递空格分隔的 URL | `claude --plugin-url https://example.com/plugin.zip` |
| `--print`, `-p` | 打印回复而不进入交互模式（有关编程使用详情，请参阅 [Agent SDK 文档](https://code.claude.com/docs/zh-CN/agent-sdk/overview.md)）。关于对仍在运行的后台会话使用 `--resume`，请参阅[恢复正在运行的后台会话](https://code.claude.com/docs/zh-CN/sessions.md#resume-a-running-background-session) | `claude -p "query"` |
| `--prompt-suggestions` | 在每个生成了提示词建议的轮次之后发出 `prompt_suggestion` 消息，其中包含预测的下一个用户提示词；非常短的对话可能不会产生任何建议。需要 `--print`、`--output-format stream-json` 和 `--verbose`。请参阅[提示词建议](https://code.claude.com/docs/zh-CN/interactive-mode.md#prompt-suggestions) | `claude -p --prompt-suggestions --output-format stream-json --verbose "query"` |
| `--ref <branch>` | 与 `--environment` 一起使用时，让新会话的检出基于命名的 ref 而不是本地 `HEAD` | `claude -p "Run the smoke test" --environment ccpool_abc123 --ref main` |
| `--remote` | `--cloud` 的已弃用别名，包括现有会话形式 | `claude --remote "Fix the login bug"` |
| `--remote-control`, `--rc` | 启动启用 [Remote Control](https://code.claude.com/docs/zh-CN/remote-control.md#start-a-remote-control-session) 的交互式会话，以便您也可以从 claude.ai 或 Claude 应用控制它。可选地为会话传递名称 | `claude --remote-control "My Project"` |
| `--remote-control-session-name-prefix <prefix>` | 当未设置显式名称时，[Remote Control](https://code.claude.com/docs/zh-CN/remote-control.md) 自动生成会话名称的前缀。默认为您的机器主机名，生成类似 `myhost-graceful-unicorn` 的名称。设置 `CLAUDE_REMOTE_CONTROL_SESSION_NAME_PREFIX` 可获得相同效果 | `claude remote-control --remote-control-session-name-prefix dev-box` |
| `--replay-user-messages` | 将来自 stdin 的用户消息重新发出到 stdout 以进行确认。需要 `--input-format stream-json` 和 `--output-format stream-json` | `claude -p --input-format stream-json --output-format stream-json --verbose --replay-user-messages` |
| `--restricted` | 在受限模式下启动。当评估工具在共享机器上驱动 `claude` 且 Claude Code 不得运行命令或读取该机器的用户和项目设置时使用。Claude Code 删除运行命令或代码的内置工具和 WebFetch，除非您在 `--tools` 中单独指定它们，而不是通过 `default` 预设。它还将内置文件工具限制在[工作目录](https://code.claude.com/docs/zh-CN/permissions.md#working-directories)，仅加载[托管设置](https://code.claude.com/docs/zh-CN/managed-settings.md)和 `--settings`，拒绝 [`bypassPermissions`](https://code.claude.com/docs/zh-CN/permission-modes#skip-all-checks-with-bypasspermissions-mode)，并[拒绝创建云端会话](https://code.claude.com/docs/zh-CN/errors.md#cloud-sessions-cannot-be-created-from-a-restricted-session)。需要 Claude Code v2.1.248 或更高版本 | `claude --restricted -p "query"` |
| `--resume`, `-r` | 按 ID 或名称恢复特定会话，或显示交互式选择器以选择会话。您可以传递会话的 `.jsonl` [会话记录文件](https://code.claude.com/docs/zh-CN/sessions.md#where-transcripts-are-stored)的绝对路径来代替 ID。选择器和名称搜索包括使用 `/add-dir` 添加此目录的会话。当您传递会话 ID 时，Claude Code 搜索当前项目目录及其 git worktree，然后搜索此机器上的所有其他项目。在 v2.1.223 之前，ID 搜索仅涵盖当前项目目录及其 git worktree。[后台会话](https://code.claude.com/docs/zh-CN/agent-view.md)在选择器中显示，标记为 `bg`。恢复仍在运行的后台会话时，会通过 `claude attach` 在此终端中[打开该会话](https://code.claude.com/docs/zh-CN/sessions.md#resume-a-running-background-session)，您在命令行上传递的提示词会作为其下一轮次发送给它。在 v2.1.285 之前，Claude Code 会拒绝并打印应改为运行的 `claude attach` 命令 | `claude --resume auth-refactor` |
| `--safe-mode` | 在禁用所有自定义的情况下启动，以排查损坏的配置：CLAUDE.md、skill、插件、hook、MCP 服务器、自定义命令和 Agent、输出样式、工作流、自定义主题、自定义快捷键、状态栏和文件建议命令、LSP 服务器以及自动记忆都不会加载。身份验证、模型选择、内置工具和权限正常工作，这与 [`--bare`](https://code.claude.com/docs/zh-CN/headless#start-faster-with-bare-mode) 不同。托管设置策略仍然适用，包括策略配置的 hook、状态栏和文件建议命令；托管插件、托管 skill、托管 CLAUDE.md 和策略配置的 MCP 服务器不适用。可用于检查是否是某个自定义触发了[自动模型回退](https://code.claude.com/docs/zh-CN/model-config.md#automatic-model-fallback)。设置 [`CLAUDE_CODE_SAFE_MODE`](https://code.claude.com/docs/zh-CN/env-vars) | `claude --safe-mode` |
| `--session-id` | 为对话使用特定的会话 ID（必须是有效的 UUID） | `claude --session-id "550e8400-e29b-41d4-a716-446655440000"` |
| `--setting-sources` | 要加载的设置源的逗号分隔列表（`user`、`project`、`local`）。有关从此会话启动并继承该列表的会话，请参阅 [Agent 视图](https://code.claude.com/docs/zh-CN/agent-view.md#what-carries-over-when-you-background)和 [agent team](https://code.claude.com/docs/zh-CN/agent-teams.md#context-and-communication) | `claude --setting-sources user,project` |
| `--settings` | 设置 JSON 文件的路径或内联 JSON 字符串。您在此处设置的值会在此会话中覆盖 `settings.json` 文件中的相同键。您省略的键保持其基于文件的值。文件必须是不超过 2 MiB 的常规文件。请参阅[设置优先级](https://code.claude.com/docs/zh-CN/settings.md#settings-precedence) | `claude --settings ./settings.json` |
| `--strict-mcp-config` | 仅使用 `--mcp-config` 中的 MCP 服务器，忽略所有其他 MCP 配置。有关此标志在托管 MCP 文件下的行为，请参阅[使用 managed-mcp.json 的独占控制](https://code.claude.com/docs/zh-CN/managed-mcp.md#exclusive-control-with-managed-mcp-json) | `claude --strict-mcp-config --mcp-config ./mcp.json` |
| `--system-prompt` | 用自定义文本替换整个系统提示词 | `claude --system-prompt "You are a Python expert"` |
| `--system-prompt-file` | 从文件加载系统提示词，替换默认提示词 | `claude --system-prompt-file ./custom-prompt.txt` |
| `--system-prompt-snapshot` | 传递 `off` 以在每个请求上重建系统提示词，而不是重用[在对话的第一个请求上记录的](#system-prompt-flags-in-resumed-conversations)提示词，例如在跨多次 `--continue` 运行迭代 `--append-system-prompt` 文本时。需要 Claude Code v2.1.257 或更高版本 | `claude --system-prompt-snapshot off` |
| `--teleport` | 在本地终端中恢复[云端会话](https://code.claude.com/docs/zh-CN/claude-code-on-the-web.md) | `claude --teleport` |
| `--teammate-mode` | 设置 [agent team](https://code.claude.com/docs/zh-CN/agent-teams.md) 队友的显示方式：`in-process`（默认）、`auto`、`tmux` 或 `iterm2`。覆盖此会话的 [`teammateMode`](https://code.claude.com/docs/zh-CN/settings-reference#teammatemode) 设置。请参阅[选择显示模式](https://code.claude.com/docs/zh-CN/agent-teams.md#choose-a-display-mode) | `claude --teammate-mode auto` |
| `--tmux` | 为 worktree 创建 tmux 会话。需要 `--worktree`。在可用时使用 iTerm2 原生窗格；传递 `--tmux=classic` 以使用传统 tmux | `claude -w feature-auth --tmux` |
| `--tools` | 限制 Claude 可以使用的内置工具。使用 `""` 禁用所有工具，`"default"` 使用默认集，或使用工具名称如 `"Bash,Edit,Read"`。在 macOS、Linux 和 WSL 上，默认集不包含 `Glob` 和 `Grep`，如 [Glob 工具行为](https://code.claude.com/docs/zh-CN/tools-reference.md#glob-tool-behavior)中所述。如果您在此处指定[任务跟踪工具](https://code.claude.com/docs/zh-CN/tools-reference.md#task-tool-availability)之一，Claude Code 也会让会话选择加入。此标志不影响 MCP 工具；要同时拒绝这些工具，请使用 `--disallowedTools "mcp__*"`。省略 [`EndConversation`](https://code.claude.com/docs/zh-CN/tools-reference#endconversation-tool-behavior) 的列表不会删除它；`""` 仅在没有 MCP 工具保留时删除它 | `claude --tools "Bash,Edit,Read"` |
| `--verbose` | 启用详细日志记录，显示完整的逐轮输出。覆盖此会话的 [`viewMode`](https://code.claude.com/docs/zh-CN/settings-reference#viewmode) 设置 | `claude --verbose` |
| `--version`, `-v` | 输出版本号 | `claude -v` |
| `--worktree`, `-w` | 在位于 `<repo>/.claude/worktrees/<name>` 的隔离 [git worktree](https://code.claude.com/docs/zh-CN/worktrees.md) 中启动 Claude。如果您不提供名称，Claude Code 会生成一个。传递 `#<number>`、GitHub Pull Request URL 或 GitLab merge request URL 以[从 `origin` 获取该 PR 或 MR 并基于它创建 worktree 分支](https://code.claude.com/docs/zh-CN/worktrees#branch-from-a-pull-request)。基于 GitLab merge request 创建分支需要 Claude Code v2.1.233 或更高版本 | `claude -w feature-auth` |

### 系统提示词标志

Claude Code 提供五个用于自定义系统提示词的标志。其中四个设置其文本，而通过 `--system-prompt-snapshot`，您可以控制对话是否保留其开始时的文本。这五个标志在交互和非交互模式下均可使用。

| 标志 | 行为 | 示例 |
| :- | :- | :- |
| `--system-prompt` | 替换整个默认提示词 | `claude --system-prompt "You are a Python expert"` |
| `--system-prompt-file` | 用文件内容替换 | `claude --system-prompt-file ./prompts/review.txt` |
| `--append-system-prompt` | 附加到默认提示词 | `claude --append-system-prompt "Always use TypeScript"` |
| `--append-system-prompt-file` | 将文件内容附加到默认提示词 | `claude --append-system-prompt-file ./style-rules.txt` |
| `--system-prompt-snapshot` | 使用 `off` 时，在每个请求上重建提示词。使用 `on`（默认）时，在[适用记录的情况下](#system-prompt-flags-in-resumed-conversations)重用已记录的提示词 | `claude --append-system-prompt "Draft rules" --system-prompt-snapshot off` |

您可以组合使用这些标志。要替换默认提示词并仍然附加您自己的文本，请将 `--append-system-prompt` 或 `--append-system-prompt-file` 与 `--system-prompt` 或 `--system-prompt-file` 一起传递。在 Claude Code v2.1.283 或更高版本中，您还可以将某个标志与其自身的文件形式一起传递，例如将 `--append-system-prompt` 与 `--append-system-prompt-file` 一起使用，Claude Code 会同时使用两者。

例如，在 shell 中运行以下命令，以同时附加来自文件的样式指南和一条额外的指令：

```bash theme={null}
claude -p --append-system-prompt-file ./style.md --append-system-prompt "Always reply in French" "Summarize README.md"
```

Claude 收到的是默认系统提示词，后跟 `style.md` 的内容、一个空行，然后是 `Always reply in French`。即使您在 `--append-system-prompt-file` 之前传递 `--append-system-prompt`，文件的内容也会排在前面。

当替换文本将每次运行都相同的指令与每次运行都会变化的上下文结合时，请在指令和上下文之间添加一行仅包含 `__SYSTEM_PROMPT_DYNAMIC_BOUNDARY__` 的内容。Claude Code 会在第一个这样的行处拆分提示词并删除该行，因此其上方的部分保持缓存，而其下方的部分可以变化。需要 Claude Code v2.1.275 或更高版本。[缓存自定义提示词的静态部分](https://code.claude.com/docs/zh-CN/agent-sdk/modifying-system-prompts.md#cache-the-static-part-of-a-custom-prompt)列出了适用此拆分的配置。

根据 Claude Code 的默认身份是否仍适合您的任务进行选择。当 Claude 应保持为编码助手，同时遵循您的额外规则时，请使用附加标志：每次调用的指令、输出格式，或 `-p` 脚本的领域上下文。附加会保留默认的工具指导、安全指令和编码约定，因此您只需提供不同的内容。当使用入口、身份或权限模型与 Claude Code 不同时，请使用替换标志，例如管道中无人监视的非编码 Agent。替换会删除整个默认提示词，包括工具指导和安全指令，因此您需要自行负责任务仍然需要的任何内容。

对于可以切换并在项目中共享的持久角色，请使用[输出样式](https://code.claude.com/docs/zh-CN/output-styles.md)。对于 Claude 应始终遵循的项目约定，请使用 [CLAUDE.md](https://code.claude.com/docs/zh-CN/memory.md)。[Agent SDK 关于系统提示词的指南](https://code.claude.com/docs/zh-CN/agent-sdk/modifying-system-prompts.md#decide-on-a-starting-point)更深入地介绍了同样的决策。

#### 恢复对话中的系统提示词标志

默认情况下，Claude Code 在对话的第一个请求上构建一次系统提示词，应用任何系统提示词标志中的文本，并将其记录在会话中。在对话被压缩之前，之后的每个请求都使用该已记录的提示词，包括在您使用 `--resume` 或 `--continue` 返回对话之后。如果您在之后的启动中传递了不同的系统提示词标志文本，或未传递任何标志，则它会在对话被压缩后或您开始新对话时生效。

在[云端会话](https://code.claude.com/docs/zh-CN/cloud-environments.md)之外，如果您通过传递 `--bare` 或设置 `CLAUDE_CODE_SIMPLE=1` 以 [bare 模式](https://code.claude.com/docs/zh-CN/headless.md#start-faster-with-bare-mode)启动 Claude Code，则记录保持关闭，除非您传递 `--system-prompt-snapshot on`。在 v2.1.268 之前，不[获取功能标志](https://code.claude.com/docs/zh-CN/env-vars.md#features-that-need-feature-flag-fetching)的会话（包括 Amazon Bedrock、Google Cloud 的 Agent Platform 和 Microsoft Foundry 上的会话）会在每个请求上重建提示词，且 `--system-prompt-snapshot` 无效。

要改为在每个请求上重建提示词，例如在跨多次 `--continue` 运行迭代其措辞时，请传递 `--system-prompt-snapshot off`。在 v2.1.265 之前，传递任何系统提示词标志也会关闭记录，除非您传递 `--system-prompt-snapshot on`。

## 另请参阅

* [Chrome 扩展](https://code.claude.com/docs/zh-CN/chrome.md) - 浏览器自动化和网络测试
* [交互模式](https://code.claude.com/docs/zh-CN/interactive-mode.md) - 快捷键、输入模式和交互功能
* [快速入门指南](https://code.claude.com/docs/zh-CN/quickstart.md) - Claude Code 入门
* [常见工作流](https://code.claude.com/docs/zh-CN/common-workflows.md) - 高级工作流和模式
* [设置](https://code.claude.com/docs/zh-CN/settings.md) - 配置选项
* [Agent SDK 文档](https://code.claude.com/docs/zh-CN/agent-sdk/overview.md) - 编程使用和集成
