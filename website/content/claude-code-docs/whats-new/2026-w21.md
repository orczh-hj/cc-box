> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# 第 21 周 · 2026 年 5 月 18–22 日

> 在 Pro 计划上使用自动模式并支持 Sonnet 4.6，在 /usage 中查看哪些 skills、subagents 和 MCP servers 驱动您的计划限制，并使用新的 /code-review 命令查看差异。

Releases [v2.1.143 → v2.1.149](https://code.claude.com/docs/en/changelog.md#2-1-143)
1 feature · 5 月 18–22

Pro 计划上的自动模式
CLI

自动模式现已在 Pro 计划上可用，并支持 Sonnet 4.6 以及 Opus。它用后台安全检查替代权限提示：常规操作无需中断您即可运行，而破坏性或可疑操作会被阻止并显示。

更新 Claude Code，然后使用 Shift+Tab 切换模式；一旦您的账户满足要求，自动模式就会出现：

```bash terminal theme={null}
claude update
```

该命令打印 `Successfully updated` 和新版本号，或如果不需要更新则打印 `Claude Code is up to date`。一旦自动模式激活，提示符页脚会显示 `auto mode on`。

[自动模式要求](https://code.claude.com/docs/zh-CN/permission-modes.md#eliminate-prompts-with-auto-mode)

其他改进

[`/usage`](https://code.claude.com/docs/zh-CN/costs#track-your-costs) 现在显示按类别细分的内容，说明是什么驱动您的计划限制，将最近的使用情况归因于 skills、subagents、plugins 和单个 MCP servers"Extra usage"在整个 CLI 中重命名为"usage credits"，`/extra-usage` 现在是 `/usage-credits`。旧名称仍然有效。该命令需要通过 `/login` 使用您的 claude.ai 订阅登录，不适用于 API 密钥身份验证。新的 [`/code-review`](https://code.claude.com/docs/zh-CN/code-review) 命令在选定的工作量级别（如 `/code-review high`）报告正确性错误，`--comment` 将发现作为内联 GitHub PR 注释发布。`/simplify` 保留为单独的仅清理审查。后台会话现在与交互式会话一起出现在 `/resume` 中，标记为 `bg`，在 `claude agents` 中使用 `Ctrl+T` 固定的会话在空闲时保持活跃`claude agents --json` 将实时会话列为 JSON 以供脚本编写，例如状态栏和会话选择器PowerShell 工具现在在 Windows 上默认为 Amazon Bedrock、Google Cloud 的 Agent Platform 和 Microsoft Foundry 用户启用；使用 `CLAUDE_CODE_USE_POWERSHELL_TOOL=0` 选择退出`claude plugin disable` 现在在另一个启用的插件依赖于目标时拒绝，`claude plugin enable` 强制启用传递依赖`/plugin` marketplace 浏览窗格显示预计的上下文成本，Discover 和 Browse 屏幕在安装前列出插件的命令、agents、skills、hooks 和 MCP/LSP servers新的 `worktree.bgIsolation: "none"` 设置允许后台会话直接编辑工作副本，无需 `EnterWorktree`，适用于 worktrees 不实用的存储库Markdown 输出呈现 GFM 任务列表复选框，`/diff` 详细视图使用键盘滚动状态行 JSON 输入现在在检测到时包括 GitHub 存储库和 PR 信息Enterprise：`allowAllClaudeAiMcps` 托管设置加载 claude.ai 云 MCP 连接器以及 `managed-mcp.json`

[v2.1.143–v2.1.149 的完整更新日志 →](https://code.claude.com/docs/en/changelog.md#2-1-143)
