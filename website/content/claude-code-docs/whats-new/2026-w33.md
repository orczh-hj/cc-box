> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# 第 33 周 · 2026 年 8 月 10–14 日

> Claude Code Desktop 在用量限制重置后自动继续，fork 模式默认启用，GitLab 合并请求和市场加入 GitHub。

发布版本 [v2.1.225 → v2.1.233](https://code.claude.com/docs/en/changelog.md#2-1-225)
3 项功能 · 8 月 10–14

Desktop 上用量限制重置后自动继续
Desktop

当您在 Claude Code Desktop 的 Code 选项卡中达到会话限制时，限制卡现在提供一个**限制重置时自动继续**复选框。勾选它，Desktop 应用将在重置后重试中断的轮次。该卡显示重试时间。每周限制卡不提供此选项。

<video autoPlay muted loop playsInline src="https://mintcdn.com/claude-code/2SnAdpL4dJ18nKb3/images/whats-new/desktop-auto-continue.mp4?fit=max&auto=format&n=2SnAdpL4dJ18nKb3&q=85&s=1937f489695feaea715e48ecfd7e62cd" />

下次出现会话限制卡时，勾选**限制重置时自动继续**并保持会话打开。该卡显示 `Auto-resuming at` 后跟重置时间，一旦限制重置，轮次将自动继续。

[达到用量限制时该怎么办](https://code.claude.com/docs/zh-CN/errors.md#youve-hit-your-session-limit)

Fork 模式默认启用
v2.1.232

Fork 模式现在在交互式会话中默认启用。Claude 可以请求 `fork` 子代理类型，它继承完整的对话和提示词缓存，而不是从头开始，因此您不必为辅助任务重新解释上下文。Claude 在交互式会话中生成的子代理（agent team 队友生成的除外）也默认在后台运行。

使用需要您迄今为止讨论的所有内容的任务自己启动 fork：

```text Claude Code theme={null}
> /subtask draft unit tests for the parser changes so far
```

fork 出现在您的输入框下方的面板中，其结果在完成时到达您的对话。要关闭 fork 模式，请设置 `CLAUDE_CODE_FORK_SUBAGENT=0`。

[启用或关闭 fork 模式](https://code.claude.com/docs/zh-CN/sub-agents.md#turn-fork-mode-on-or-off)

GitLab 合并请求和市场
v2.1.232

插件市场克隆裸 `gitlab.com` URL，包括嵌套子组。在 v2.1.233 或更高版本上，将 GitLab 合并请求 URL 传递给 `--worktree` 以从其分支，`claude agents` 视图将链接到合并请求的会话标记为 `!N`。Claude Code 还会对 GitLab 令牌族（如 `glpat-` 和 `glrt-`）进行脱敏处理，并以与保护 `gh` 相同的方式保护 `glab` CLI 的配置存储。

在从合并请求分支的 worktree 中启动会话：

```bash terminal theme={null}
claude --worktree https://gitlab.com/group/project/-/merge_requests/42
```

当 `origin` 在 gitlab.com 上时，Claude Code 获取 `merge-requests/42/head` 并在其自己的 worktree 中的该分支上打开会话。

[从拉取或合并请求分支 worktree](https://code.claude.com/docs/zh-CN/worktrees.md#branch-from-a-pull-request)

其他改进

在输入框中键入 `@` 以按名称[提及另一个 Claude 会话](https://code.claude.com/docs/zh-CN/cross-session-messaging.md#message-another-session)，Claude 使用 `SendMessage` 直接向其发送消息；与恰好一个活跃会话完全匹配的裸名称现在无需确认步骤即可传递一台机器上的交互式会话保持[唯一名称](https://code.claude.com/docs/zh-CN/cross-session-messaging.md#see-which-sessions-claude-can-reach)：如果您启动或重命名会话时使用另一个活跃会话已在使用的名称，Claude Code 会为您的会话提供 `name-word-word` 变体并告知您插件市场接受[`command` 源](https://code.claude.com/docs/zh-CN/plugins/marketplace-reference#command-plugin-source)：本地命令打印插件目录，Claude Code 在每个会话中重新解析并应用，无需重启在 Linux 和 WSL 上，设置[`CLAUDE_CODE_TOOL_MEMORY_LIMIT`](https://code.claude.com/docs/zh-CN/tools-reference#memory-limit-on-linux-and-wsl) 为大小（如 `4G`）以限制 Bash 和 PowerShell 工具命令可以使用的内存任务跟踪工具，如 `TaskCreate`、`TaskUpdate` 和 `TodoWrite`，[在 Opus 4.8、Sonnet 5、Fable 5、Mythos 5 及这些系列中的更高版本上不再可用](https://code.claude.com/docs/zh-CN/tools-reference.md#task-tool-availability)；设置 `CLAUDE_CODE_ENABLE_TODO_TOOLS=1` 以重新启用它们[`/code-review`](https://code.claude.com/docs/zh-CN/code-review-local) 在高、超高和最大努力级别现在像其他级别一样在后台 Agent 中运行[`/plugin install plugin@marketplace`](https://code.claude.com/docs/zh-CN/plugins/install#install-a-plugin) 首先刷新市场，因此新发布的插件无需手动市场更新即可安装设置接受[`additionalMarketplaces` 和 `allowedMarketplaces`](https://code.claude.com/docs/zh-CN/settings-reference#marketplace-key-aliases) 作为 `extraKnownMarketplaces` 和 `strictKnownMarketplaces` 的别名在较新的模型上，Claude 可以[使用 Write 工具覆盖现有文件](https://code.claude.com/docs/zh-CN/tools-reference.md#write-tool-behavior)而无需在此会话中首先读取它，与 Edit 工具的规则匹配；较旧的模型需要读取VS Code 扩展可以[将会话列表组织成组](https://code.claude.com/docs/zh-CN/vs-code.md#organize-sessions-into-groups)：右键单击以创建、重命名或删除组，使用 Cmd/Ctrl- 或 Shift-单击一次移动多个会话如果您的组织通过[具有支出限制的 Claude 应用网关](https://code.claude.com/docs/zh-CN/claude-apps-gateway-spend-limits.md)路由 Claude Code，Claude Code 会在您达到限制时显示限制期间、其重置时间和运营商的消息

[v2.1.225–v2.1.233 的完整更新日志 →](https://code.claude.com/docs/en/changelog.md#2-1-225)
