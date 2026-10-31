> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# 第 35 周 · 2026 年 8 月 24–28 日

> 在 Claude Code Desktop 应用中恢复终端会话，查看 Claude 为您起草的反馈报告，并在受限模式下启动会话。

发布版本 [v2.1.240 → v2.1.250](https://code.claude.com/docs/en/changelog.md#2-1-240)
3 项功能 · 8 月 24–28 日

在 Desktop 应用中恢复终端会话
Desktop

在 Claude Code Desktop 提示框中输入 `/resume` 以选择您从 CLI 启动的任何会话，并在应用中继续该会话，保持完整的对话和上下文。按标题、文件夹或分支搜索您的会话，并在恢复前预览您停止的位置。

<video autoPlay muted loop playsInline src="https://mintcdn.com/claude-code/f9HTZGyMtxIFOUgt/images/whats-new/desktop-resume-cli-session.mp4?fit=max&auto=format&n=f9HTZGyMtxIFOUgt&q=85&s=41e4a5fda6b9d63280589f2cbdabf44f" />

在 Desktop 会话中，运行命令以列出您的终端会话：

```text Claude Code theme={null}
> /resume
```

选择一个会话并按 `Enter`。对话将在您停止的位置在应用中打开。

[在 CLI 和 Desktop 之间移动](https://code.claude.com/docs/zh-CN/desktop.md#coming-from-the-cli)

Claude 起草的反馈
CLI

当工具持续失败、Claude 无法帮助处理请求或您指出错误时，Claude 现在会使用 `SendFeedback` 工具为您起草反馈报告。您的提示上方会显示一张卡片，您可以从那里查看、发送或关闭它。在您发送之前，任何内容都不会到达 Anthropic。需要 v2.1.238 或更高版本。

![一个 Claude Code 会话，其中 Claude 已起草了一份标题为"Sandbox image pull fails behind proxy"的错误报告，显示为提示上方的卡片，带有查看、发送或关闭的选项](https://mintcdn.com/claude-code/f9HTZGyMtxIFOUgt/images/whats-new/claude-drafted-feedback.jpg?fit=max\&auto=format\&n=f9HTZGyMtxIFOUgt\&q=85\&s=5cacb3be0dffd1cbd417381f3721637e)

运行 `/feedback` 不带参数以打开来自每个会话的草稿队列：

```text Claude Code theme={null}
> /feedback
```

选择一份草稿，然后编辑、发送或丢弃它。要关闭起草功能，请在 `/config` 中将 **Claude-drafted feedback** 设置为 `off`。

[SendFeedback 工具行为](https://code.claude.com/docs/zh-CN/tools-reference.md#sendfeedback-tool-behavior)

受限模式
v2.1.248

受限模式启动 Claude Code 时不包含运行命令或代码的内置工具。当评估工具在共享机器上驱动 `claude` 时使用它。使用 `--restricted` 启动或设置 `CLAUDE_CODE_RESTRICTED=1`。Claude Code 还会移除 `WebFetch`，将文件工具限制在工作目录中，仅加载托管设置和 `--settings`，并拒绝 `bypassPermissions` 权限模式。

运行不带命令运行工具的非交互式查询：

```bash terminal theme={null}
claude --restricted -p "review src/ for SQL injection risks"
```

要为 Claude 恢复已移除的工具之一，请在 `--tools` 中将其与您想要的其他内置工具一起列出，例如 `--tools "Bash,Read,Edit"`。`--tools` 是一个允许列表，其 `default` 预设不会恢复已移除的工具。

[CLI 标志](https://code.claude.com/docs/zh-CN/cli-reference.md#cli-flags)

其他亮点

设置新的 [`modelPicker`](https://code.claude.com/docs/zh-CN/settings-reference#modelpicker) 设置以使用您自己的有序、标记的条目扩展或替换 `/model` 选择器的内置列表，包括 Amazon Bedrock 或 Google Cloud 的 Agent Platform 模型 ID将 [`promptCacheTtl`](https://code.claude.com/docs/zh-CN/prompt-caching#choose-the-ttl-yourself) 设置为 `1h` 以在使用 API 密钥或云提供商时在主对话上保持一小时的提示缓存；`subagentPromptCacheTtl` 为子代理和主对话外的所有其他请求设置 TTL在 Pro、Max、Team 和 Enterprise 计划上，[`/usage`](https://code.claude.com/docs/zh-CN/costs#plan-usage-breakdown) 添加了 Loops 分解：运行计数、总令牌数、每次运行的令牌数以及使用最多令牌的 `/loop` 和计划任务的最后一次运行按合同费率的组织可以设置 [`modelPricing`](https://code.claude.com/docs/zh-CN/costs#report-spend-at-your-contracted-rates) 托管设置，以便 `/usage`、状态行和 OpenTelemetry 按这些费率而不是列表价格报告成本`/login` 在 **Anthropic Console 账户**选项下提供 **使用您的 Console 账户登录**，因此 Console 组织的成员如果不允许 API 密钥，可以在不创建 API 密钥的情况下登录运行 `/permissions` 并打开新的 [**Auto mode** 标签页](https://code.claude.com/docs/zh-CN/auto-mode-config.md#edit-rules-from-permissions)以查看和编辑自动模式分类器规则，而无需打开设置文件当自动模式可用时，Manual 和 `acceptEdits` 权限模式中的 Bash 权限提示提供 [**是的，并切换到自动模式**](https://code.claude.com/docs/zh-CN/permission-modes.md#switch-permission-modes)选项；选择它以批准命令并将会话切换到自动模式在您 [使用 `/cd` 移动会话](https://code.claude.com/docs/zh-CN/permissions#move-the-session-to-another-directory)后，新目录的项目设置、hooks、`.mcp.json` 服务器、skills 和子代理立即生效，而不是在下一个 `--resume` 时生效在非交互式会话中，包括 `-p` 运行、Agent SDK 运行和云会话，Claude Code [继续响应](https://code.claude.com/docs/zh-CN/errors.md#the-response-above-may-be-incomplete)，该响应被服务器错误、连接断开或停滞中断，当部分响应包含文本且没有工具调用时在其 `maxTurns` 限制处停止的子代理返回其输出标记为部分，并提示 Claude 可以 [使用 `SendMessage` 继续它](https://code.claude.com/docs/zh-CN/sub-agents#resume-subagents)，而不是显示为已完成在 Amazon Bedrock、Google Cloud 的 Agent Platform 和 Microsoft Foundry 上，同一机器上的会话现在可以 [相互消息](https://code.claude.com/docs/zh-CN/cross-session-messaging.md#availability)，`/loop` 可以 [选择自己的间隔](https://code.claude.com/docs/zh-CN/scheduled-tasks.md#let-claude-choose-the-interval)，`/model` 和 `/effort` 立即应用而不是在回合结束后应用本机安装程序和自动更新程序下载 zstd 压缩的构建，在 Linux x64 上约为 75 MB 而不是 340 MB，本机构建按需加载代码，每个会话使用的内存大约少 40 到 70 MB

[v2.1.240–v2.1.250 的完整更新日志 →](https://code.claude.com/docs/en/changelog.md#2-1-240)
