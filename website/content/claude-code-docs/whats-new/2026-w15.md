> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# 第15周 · 2026年4月6–10日

> Ultraplan 云规划、具有自适应 /loop 的 Monitor 工具、用于打包设置的 /team-onboarding 以及从终端运行的 /autofix-pr。

发布版本 [v2.1.92 → v2.1.101](https://code.claude.com/docs/en/changelog.md#2-1-92)
4 项功能 · 4月6–10日

Ultraplan
研究预览

从终端在云中启动 Plan Mode，然后在浏览器中查看结果。Claude 在网络会话的 Claude Code 中起草计划，同时您的终端保持空闲；准备好后，您可以对各个部分进行评论、请求修订，并选择远程执行或将其发送回 CLI。从 v2.1.101 开始，首次运行会自动创建默认云环境，因此在尝试之前无需进行网络设置步骤。

<video autoPlay muted loop playsInline src="https://mintcdn.com/claude-code/aFXPQxiBOW99MHS3/images/whats-new/ultraplan.mp4?fit=max&auto=format&n=aFXPQxiBOW99MHS3&q=85&s=e8f2f23730c6a5c289dbf3e7b13eadf6" />

运行命令，或在任何提示中包含关键字：

```text title="Claude Code" wrap theme={null}
/ultraplan migrate the auth service from sessions to JWTs
```

[Ultraplan 指南](https://code.claude.com/docs/zh-CN/ultraplan)

Monitor 工具
v2.1.98

一个新的内置工具，可生成后台监视程序并将其事件流式传输到对话中：每个事件都作为新的转录消息出现，Claude 立即对其做出反应。跟踪训练运行、监督 PR 的 CI，或在开发服务器崩溃发生时立即自动修复，所有这些都无需 Bash sleep 循环占用回合。

<video autoPlay muted loop playsInline src="https://mintcdn.com/claude-code/aFXPQxiBOW99MHS3/images/whats-new/monitor-tool.mp4?fit=max&auto=format&n=aFXPQxiBOW99MHS3&q=85&s=f4156c15a0999de5c5157f54a3117c89" />

要求 Claude 在您继续工作时监视某些内容：

```text title="Claude Code" wrap theme={null}
Tail server.log in the background and tell me the moment a 5xx shows up
```

这与 `/loop` 配对，现在可自适应：省略间隔，Claude 会根据任务安排下一个时刻，或使用 Monitor 工具完全跳过轮询。

```text title="Claude Code" wrap theme={null}
/loop check CI on my PR
```

[Monitor 工具参考](https://code.claude.com/docs/zh-CN/tools-reference.md#monitor-tool)

/autofix-pr
CLI

PR 自动修复在第 13 周登陆网络。现在您可以在不离开终端的情况下启用它：`/autofix-pr` 推断当前分支的开放 PR 并在一个步骤中为其启用 Claude Code on the web 上的自动修复。推送您的分支，运行命令，然后离开；Claude 监视 CI 和审查评论并推送修复，直到它变为绿色。

<video autoPlay muted loop playsInline src="https://mintcdn.com/claude-code/aFXPQxiBOW99MHS3/images/whats-new/autofix-pr.mp4?fit=max&auto=format&n=aFXPQxiBOW99MHS3&q=85&s=95f191eb4711130a128aec3f6b720527" />

从 PR 的分支运行它：

```text title="Claude Code" wrap theme={null}
/autofix-pr
```

[自动修复拉取请求](https://code.claude.com/docs/zh-CN/claude-code-on-the-web.md#auto-fix-pull-requests)

/team-onboarding
v2.1.101

从您的本地 Claude Code 使用情况生成团队成员快速入门指南。在您熟悉的项目中运行它，并将输出交给新团队成员，以便他们可以重放您的设置，而不是从默认值开始。

在您花费了真实时间的项目中运行它：

```text title="Claude Code" wrap theme={null}
/team-onboarding
```

[命令参考](https://code.claude.com/docs/zh-CN/commands.md)

其他亮点

焦点视图：在无闪烁模式下按 `Ctrl+O` 将视图折叠到您的最后一个提示、单行工具摘要（带有 diffstats）和 Claude 的最终响应登录屏幕上的引导式 [Amazon Bedrock](https://code.claude.com/docs/zh-CN/amazon-bedrock.md) 和 [Google Cloud 的 Agent Platform](https://code.claude.com/docs/zh-CN/google-vertex-ai.md) 设置向导：选择"第三方平台"进行分步身份验证、区域、凭证检查和模型固定`/agents` 获得选项卡式布局：Running 选项卡显示带有 `● N running` 计数的实时子代理，以及库选项卡中的 Run agent 和 View running instance 操作默认工作量级别现在对 API 密钥、Amazon Bedrock、Google Cloud 的 Agent Platform、Microsoft Foundry、Team 和 Enterprise 用户为 `high`（使用 `/effort` 控制）`/cost` 为订阅用户显示按模型和缓存命中的分解`/release-notes` 现在是交互式版本选择器状态行：新的 `refreshInterval` 设置每 N 秒重新运行命令，JSON 输入中的 `workspace.git_worktree``CLAUDE_CODE_PERFORCE_MODE`：Edit/Write 在只读文件上失败，并显示 `p4 edit` 提示，而不是静默覆盖OS CA 证书存储现在默认受信任，因此企业 TLS 代理无需额外设置即可工作（`CLAUDE_CODE_CERT_STORE=bundled` 选择退出）由 Mantle 提供支持的 Amazon Bedrock：设置 `CLAUDE_CODE_USE_MANTLE=1`强化的 Bash 工具权限：反斜杠转义标志、环境变量前缀、`/dev/tcp` 重定向和复合命令现在正确提示`UserPromptSubmit` hooks 可以通过 `hookSpecificOutput.sessionTitle` 设置会话标题

[v2.1.92–v2.1.101 的完整更改日志 →](https://code.claude.com/docs/en/changelog.md#2-1-92)
