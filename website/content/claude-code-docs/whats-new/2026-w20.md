> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# 第 20 周 · 2026 年 5 月 11–15 日

> 从一个屏幕管理每个 Claude Code 会话，使用 agent view，让 Claude 持续朝着目标工作直到条件满足，并在 Opus 4.7 上默认运行快速模式。

发布版本 [v2.1.139 → v2.1.142](https://code.claude.com/docs/en/changelog.md#2-1-139)
3 项功能 · 5 月 11–15

Agent view
research preview

`claude agents` 为每个 Claude Code 会话打开一个屏幕：显示正在运行的内容、被阻止等待您输入的内容以及已完成的内容。分派一个错误修复、一个拉取请求审查和一个不稳定测试调查作为三行，在另一个窗口中继续工作，仅在某一行需要您时才介入。附加到任何行以进入其完整对话，然后按 `←` 返回列表。每个后台会话在没有附加终端的情况下继续运行。

<video autoPlay muted loop playsInline src="https://mintcdn.com/claude-code/ITvjicPxe1SM3GX7/images/whats-new/agent-view.mp4?fit=max&auto=format&n=ITvjicPxe1SM3GX7&q=85&s=0eefe6cbe75464c8f7902bba630ab7a4" />

从您的 shell 打开仪表板：

```bash terminal theme={null}
claude agents
```

[Agent view](https://code.claude.com/docs/zh-CN/agent-view.md)

/goal
v2.1.139

设置完成条件，Claude 会在多个回合中持续朝着该条件工作，无需您在每一步进行提示。在每个回合之后，一个快速模型检查条件是否满足；如果不满足，Claude 会开始另一个回合，而不是将控制权交还给您。适用于具有可验证结束状态的大量工作，例如迁移模块直到每个调用站点都编译并通过测试。一旦条件满足，目标就会清除，并在交互式、`-p` 和 Remote Control 中工作。

<video autoPlay muted loop playsInline src="https://mintcdn.com/claude-code/ITvjicPxe1SM3GX7/images/whats-new/goal.mp4?fit=max&auto=format&n=ITvjicPxe1SM3GX7&q=85&s=6806df3780c548b93a02d6fa71da276b" />

设置一个目标并让 Claude 运行直到它满足：

```text Claude Code theme={null}
> /goal all tests in test/auth pass and the lint step is clean
```

[Goals](https://code.claude.com/docs/zh-CN/goal.md)

Fast mode on Opus 4.7
research preview

`/fast` 现在默认在 Opus 4.7 上运行，而不是 Opus 4.6。快速模式是一个高速 Opus 配置：相同的模型质量，速度约为 2.5 倍，但每个令牌的成本更高，适用于快速迭代和实时调试。定价保持不变，为 $30/$150 每 MTok，与 Opus 4.6 快速模式相同。要将快速模式固定到 Opus 4.6，请设置 `CLAUDE_CODE_OPUS_4_6_FAST_MODE_OVERRIDE=1`。

![Claude Code 模型选择器显示 Opus 4.7 Fast 1M 作为默认值，并启用了 Fast 切换](https://mintcdn.com/claude-code/ITvjicPxe1SM3GX7/images/whats-new/fast-mode-opus-47.png?fit=max\&auto=format\&n=ITvjicPxe1SM3GX7\&q=85\&s=6b6d92f7748ce5328a1ee9a269fb1a87)

切换快速模式，现在在 Opus 4.7 上运行：

```text Claude Code theme={null}
> /fast
```

[Fast mode on Opus 4.7](https://code.claude.com/docs/zh-CN/fast-mode.md#understand-the-cost-tradeoff)

其他改进

`claude agents` 获得了分派标志（`--add-dir`、`--settings`、`--mcp-config`、`--plugin-dir`、`--permission-mode`、`--model`、`--effort`、`--dangerously-skip-permissions`）来配置后台会话，`claude agents --cwd <path>` 将会话列表范围限制在一个目录新的 hook `args: string[]` exec 形式直接生成命令而不使用 shell，因此路径占位符永远不需要引用新的 `continueOnBlock` 配置选项用于 `PostToolUse` hooks，将 hook 的拒绝原因反馈给 Claude 并继续该回合，而不是结束它hook JSON 输出中的新 `terminalSequence` 字段让 hooks 发出桌面通知、窗口标题和铃声，无需控制终端Rewind 菜单添加了"总结到此处"以压缩早期上下文，同时保持最近的回合完整当设置 `ANTHROPIC_API_KEY`、`apiKeyHelper` 或 `ANTHROPIC_AUTH_TOKEN` 时，Remote Control、`/schedule`、claude.ai MCP 连接器和通知首选项现在被禁用，即使与 claude.ai 登录一起使用；取消设置 API 密钥以使用这些功能MCP stdio 服务器现在在其环境中接收 `CLAUDE_PROJECT_DIR`，与 hooks 匹配，插件配置可以在命令中引用 `${CLAUDE_PROJECT_DIR}``claude plugin details <name>` 显示插件的组件清单和预计的每个会话令牌成本，`/plugin` 详细信息窗格现在也列出了插件提供的 LSP 服务器具有根级 `SKILL.md` 且没有 `skills/` 子目录的插件现在被显示为一个 skill`/feedback` 现在可以包括过去 24 小时或 7 天内的最近会话，用于跨越多个会话的问题Agent tool `subagent_type` 现在不区分大小写和分隔符匹配，因此 `"Code Reviewer"` 解析为 `code-reviewer`

[v2.1.139–v2.1.142 的完整更新日志 →](https://code.claude.com/docs/en/changelog.md#2-1-139)
