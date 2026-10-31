> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# 第 23 周 · 2026 年 6 月 1–5 日

> 在 Amazon Bedrock、Google Cloud 的 Agent Platform 和 Microsoft Foundry 上运行自动模式，在 acceptEdits 模式下提示写入可运行代码的文件，使用 /plugin list 列出已安装的插件，以及为托管部署要求批准的版本范围。

发布版本 [v2.1.158 → v2.1.165](https://code.claude.com/docs/en/changelog.md#2-1-158)
4 项功能 · 6 月 1–5 日

Amazon Bedrock、Google Cloud 的 Agent Platform 和 Microsoft Foundry 上的自动模式
v2.1.158

自动模式现已在 Amazon Bedrock、Google Cloud 的 Agent Platform 和 Microsoft Foundry 上可用，支持 Opus 4.7 和 Opus 4.8，用第三方提供商的后台安全检查替代权限提示。通过设置 `CLAUDE_CODE_ENABLE_AUTO_MODE=1` 来选择加入。

在第三方提供商上选择加入，然后使用 Shift+Tab 切换到自动模式：

```bash terminal theme={null}
export CLAUDE_CODE_ENABLE_AUTO_MODE=1
```

[在第三方提供商上启用自动模式](https://code.claude.com/docs/zh-CN/permission-modes.md#enable-auto-mode-on-bedrock-agent-platform-or-foundry)

更安全的自动编辑
v2.1.160

Claude Code 现在在写入可运行代码的文件之前进行提示，即使在 `acceptEdits` 模式下也是如此。受保护的集合包括 shell 启动文件，如 `.zshenv` 和 `.bash_login`、`~/.config/git/` 下的 git 配置，以及构建工具配置，如 `.npmrc`、`.bazelrc` 和 `.pre-commit-config.yaml`。除了 `bypassPermissions` 模式外，这些写入在任何模式下都不会自动批准。

在 acceptEdits 模式下工作；Claude 现在在写入这些文件之前暂停：

```bash terminal theme={null}
claude --permission-mode acceptEdits
```

[受保护的路径](https://code.claude.com/docs/zh-CN/permission-modes.md#protected-paths)

使用 /plugin list 列出已安装的插件
v2.1.163

新的 `/plugin list` 命令内联打印已安装的插件，无需打开 `/plugin` 菜单，也可从 shell 中作为 `claude plugin list` 使用。在交互形式中，添加 `--enabled` 或 `--disabled` 以仅显示处于该状态的插件。

列出当前打开的插件：

```text Claude Code theme={null}
> /plugin list --enabled
```

[插件命令](https://code.claude.com/docs/zh-CN/plugins/cli-reference.md#plugin-list)

托管部署的版本要求
v2.1.163

两个托管设置 `requiredMinimumVersion` 和 `requiredMaximumVersion` 允许您的组织要求批准的 Claude Code 版本范围。超出范围的客户端在启动时退出，并告诉用户通过组织的方法进行更新。`claude update`、`claude install` 和 `claude doctor` 继续工作，以便用户仍然可以恢复。

向托管设置添加下限，以便较旧的客户端拒绝启动：

```json managed-settings.json theme={null}
{
  "requiredMinimumVersion": "2.1.163"
}
```

[决定要强制执行的内容](https://code.claude.com/docs/zh-CN/admin-setup.md#decide-what-to-enforce)

其他改进

[动态工作流](https://code.claude.com/docs/zh-CN/workflows.md)的触发关键字从 `workflow` 更改为 `ultracode`；用您自己的话要求工作流仍然有效，关键字在提示中以紫色突出显示[Stop 和 SubagentStop hooks](https://code.claude.com/docs/zh-CN/hooks.md) 可以返回 `hookSpecificOutput.additionalContext` 以向 Claude 提供反馈并继续轮次，而不是被视为错误`claude mcp` list、get 和 add 不再打印机密：环境变量引用不会展开，凭证标头和 URL 机密会被编辑并行工具批处理中失败的 Bash 命令不再取消其他命令；每个工具独立返回自己的结果当您使用单文件 `grep`、`egrep` 或 `fgrep` 查看文件时，编辑文件不再需要单独的 Read单击自动完成菜单中的命令现在会将其填充到您的提示中，而不是立即运行；按 Enter 键运行在 `--tools` 中列出 `Grep` 或 `Glob` 现在在具有嵌入式搜索的本机构建上提供专用搜索工具，而不是静默忽略这些名称`/effort` 现在确认您选择的级别将作为新会话的默认值持久化`OTEL_RESOURCE_ATTRIBUTES` 值现在作为标签附加到指标数据点，因此您可以按自定义维度（如团队或存储库）对使用指标进行切片Windsurf 在 `/ide`、`/terminal-setup` 和 `/scroll-speed` 中重命名为 Devin Desktop，遵循编辑器的品牌重塑`/btw` 获得了 `c to copy` 快捷键，可将原始 markdown 答案复制到剪贴板

[v2.1.158–v2.1.165 的完整更新日志 →](https://code.claude.com/docs/en/changelog.md#2-1-158)
