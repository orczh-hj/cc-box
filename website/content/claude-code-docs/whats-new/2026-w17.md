> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# 第17周 · 2026年4月20–24日

> /ultrareview 作为研究预览版开放，返回终端时自动生成会话摘要，可以在插件中构建和发布自定义颜色主题，以及重新设计的网页版 Claude Code。

发布版本 [v2.1.114 → v2.1.119](https://code.claude.com/docs/en/changelog.md#2-1-114)
4 项功能 · 4月20–24日

/ultrareview
research preview

现已公开研究预览版。Ultrareview 在云中针对您的分支或 PR 运行一队 bug 搜寻代理，发现的问题会自动返回到 CLI 或桌面应用。在合并关键更改（如身份验证或数据迁移）之前运行它。

<video autoPlay muted loop playsInline src="https://mintcdn.com/claude-code/FTi4SBJ9YRs7d-5X/images/whats-new/ultrareview.mp4?fit=max&auto=format&n=FTi4SBJ9YRs7d-5X&q=85&s=0fb1271365d38f414ad155aeb8edb08e" />

审查您当前所在的分支：

```text Claude Code theme={null}
> /ultrareview
```

或将其指向 PR：

```text Claude Code theme={null}
> /ultrareview 1234
```

[Ultrareview 指南](https://code.claude.com/docs/zh-CN/ultrareview.md)

会话摘要
CLI

将焦点从会话转移开，然后返回时会看到一行摘要，说明您离开期间发生了什么。在同时运行多个 Claude 会话时，有助于保持工作流程。

<video autoPlay muted loop playsInline src="https://mintcdn.com/claude-code/FTi4SBJ9YRs7d-5X/images/whats-new/session-recap.mp4?fit=max&auto=format&n=FTi4SBJ9YRs7d-5X&q=85&s=0a8db1470bd0161a47efeb2f322af76f" />

按需生成摘要，或从 `/config` 关闭自动摘要：

```text Claude Code theme={null}
> /recap
```

[交互模式：会话摘要](https://code.claude.com/docs/zh-CN/interactive-mode.md#session-recap)

自定义主题
v2.1.118

从 `/theme` 构建和切换命名颜色主题，或在 `~/.claude/themes/` 中手动编辑 JSON 文件。每个主题选择一个基础预设，并仅覆盖您关心的令牌。插件也可以附带主题。

打开主题选择器并创建新主题：

```text Claude Code theme={null}
> /theme
```

[终端配置：创建自定义主题](https://code.claude.com/docs/zh-CN/terminal-config.md#create-a-custom-theme)

网页版 Claude Code
web

[claude.ai/code](https://claude.ai/code) 的新外观与重新设计的桌面应用相匹配：会话侧边栏、拖放布局和刷新的例程视图。关键部分已重建，以实现更快的响应和更可靠的体验。

![网页版 Claude Code 重新设计概览：新 UI、速度和可靠性、跨网页、移动和 CLI 工作](https://mintcdn.com/claude-code/FTi4SBJ9YRs7d-5X/images/whats-new/web-redesign.jpeg?fit=max\&auto=format\&n=FTi4SBJ9YRs7d-5X\&q=85\&s=a2aca1b49e295b7337f5779038db8e2c)

[网页版 Claude Code](https://code.claude.com/docs/zh-CN/claude-code-on-the-web.md)

其他改进

[Vim 可视模式](https://code.claude.com/docs/zh-CN/interactive-mode.md#vim-editor-mode)：在提示输入中按 `v` 进行字符选择或按 `V` 进行行选择，带有运算符和可视反馈Hooks 现在可以通过 [`type: "mcp_tool"`](https://code.claude.com/docs/zh-CN/hooks#mcp-tool-hook-fields) 直接调用 MCP 工具，因此 hook 可以访问已连接的服务器，而无需生成进程`/cost` 和 `/stats` 已合并到 [`/usage`](https://code.claude.com/docs/zh-CN/commands)；旧名称仍然可用作打开相关选项卡的输入快捷方式`/config` 更改（主题、编辑器模式、详细信息等）现在持久化到 `~/.claude/settings.json`，并遵循与其他 [设置](https://code.claude.com/docs/zh-CN/settings.md) 相同的项目/本地/策略优先级[分叉的子代理](https://code.claude.com/docs/zh-CN/sub-agents.md#fork-the-current-conversation) 可以通过 `CLAUDE_CODE_FORK_SUBAGENT=1` 在外部构建上启用：分叉继承您的完整对话上下文，而不是从头开始Pro 和 Max 订阅者在 Opus 4.6 和 Sonnet 4.6 上的默认 [努力级别](https://code.claude.com/docs/zh-CN/model-config.md#adjust-effort-level) 现在是 `high`（之前是 `medium`）原生 macOS 和 Linux 构建用嵌入式 `bfs` 和 `ugrep`（通过 Bash 可用）替换了 `Glob` 和 `Grep` 工具，可实现更快的搜索，无需单独的工具往返`--from-pr` 现在除了接受 github.com 外，还接受 GitLab 合并请求、Bitbucket 拉取请求和 GitHub Enterprise PR URL自动模式：在 [`autoMode.allow`、`soft_deny` 或 `environment`](https://code.claude.com/docs/zh-CN/auto-mode-config) 中包含 `"$defaults"`，以在内置列表旁边添加自定义规则，而不是替换它新的 [`claude plugin tag`](https://code.claude.com/docs/zh-CN/plugins/dependencies#tag-plugin-releases-for-version-resolution) 命令为具有版本验证的插件创建发布 git 标签Opus 4.7 会话现在针对模型的原生 1M 上下文窗口进行计算，修复了膨胀的 `/context` 百分比和过早的自动压缩`/resume` 在大型会话上的速度提高了 67%，现在在重新读取之前提供总结陈旧的大型会话的选项

[v2.1.114–v2.1.119 的完整更新日志 →](https://code.claude.com/docs/en/changelog.md#2-1-114)
