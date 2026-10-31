> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# 第 18 周 · 2026 年 4 月 27 日 – 5 月 1 日

> Claude Code 在 Windows 上无需 Git Bash 即可运行，claude auth login 在浏览器回调无法到达 localhost 时接受粘贴的 OAuth 代码，claude project purge 清理每个项目的本地状态，将 PR URL 粘贴到 /resume 中可找到创建该会话的会话。

发布版本 [v2.1.120 → v2.1.126](https://code.claude.com/docs/en/changelog.md#2-1-120)
4 项功能 · 4 月 27 日 – 5 月 1 日

无需浏览器回调即可登录
v2.1.126

`claude auth login` 现在接受在浏览器回调无法到达 localhost 时直接粘贴到终端的 OAuth 代码。这涵盖了 WSL2、SSH 会话和容器，其中重定向到本地端口不起作用。同一版本还修复了在慢速或代理连接以及仅 IPv6 devcontainers 上的登录超时。

登录，然后粘贴浏览器中的代码：

```bash theme={null}
claude auth login
```

[CLI 参考](https://code.claude.com/docs/zh-CN/cli-reference.md#cli-commands)

claude project purge
v2.1.124

删除项目的所有 Claude Code 状态：记录、任务、文件历史和项目的配置条目。支持 `--dry-run` 预览、`-y`/`--yes` 跳过确认、`-i`/`--interactive` 选择以及 `--all` 清除每个项目。

预览将删除的内容：

```bash theme={null}
claude project purge --dry-run
```

然后真正运行它：

```bash theme={null}
claude project purge
```

[CLI 参考](https://code.claude.com/docs/zh-CN/cli-reference.md)

通过 PR URL 恢复
v2.1.122

当你使用 `gh pr create` 创建拉取请求时，Claude Code 会将其链接到生成它的会话。现在你可以仅从 PR URL 返回到该会话，而无需记住其名称。

打开会话选择器：

```text Claude Code theme={null}
> /resume
```

将 PR URL 粘贴到选择器中。粘贴的第一个字符将你置于搜索模式，列表筛选到创建该 PR 的会话。按 Enter 恢复它。GitHub、GitHub Enterprise、GitLab 和 Bitbucket 拉取和合并请求 URL 都可以使用。

```text Claude Code theme={null}
https://github.com/your-org/your-repo/pull/1234
```

要打开已筛选到 PR 的选择器，请在命令行上传递 PR 号：

```bash theme={null}
claude --from-pr 1234
```

[会话：使用会话选择器](https://code.claude.com/docs/zh-CN/sessions.md#use-the-session-picker)

Windows 无需 Git Bash
Windows

不再需要 Git for Windows。当 Bash 不存在时，Claude Code 使用 PowerShell 作为 shell 工具，当启用 PowerShell 工具时，它被视为主要 shell。现在自动检测通过 Microsoft Store、MSI 不带 PATH 或 `.NET` 全局工具安装的 PowerShell 7。

[设置指南](https://code.claude.com/docs/zh-CN/setup.md)

其他优化

MCP 服务器可以通过其配置中的 `alwaysLoad: true` 选择退出工具搜索延迟，以便该服务器的所有工具始终可用新的 `claude plugin prune` 删除孤立的自动安装的插件依赖项，`plugin uninstall --prune` 级联删除`/skills` 现在有一个类型过滤搜索框，因此你可以在长列表中找到技能而无需滚动`PostToolUse` hooks 可以通过 `hookSpecificOutput.updatedToolOutput` 替换任何工具的工具输出，不仅仅是 MCP 工具新的 [`claude ultrareview`](https://code.claude.com/docs/zh-CN/ultrareview) 子命令从 CI 或脚本非交互式地运行 `/ultrareview`：将发现打印到 stdout（`--json` 用于原始输出）并在完成时退出 0 或失败时退出 1`--dangerously-skip-permissions` 现在绕过对 `.claude/`、`.git/`、`.vscode/`、shell 配置文件和其他以前受保护的路径的写入提示，而灾难性删除命令仍然作为安全网提示`/model` 选择器可以在 `ANTHROPIC_BASE_URL` 指向 Anthropic 兼容网关时列出来自网关的 `/v1/models` 端点的模型；自 v2.1.129 起使用 `CLAUDE_CODE_ENABLE_GATEWAY_MODEL_DISCOVERY=1` 选择加入在启动期间遇到瞬时错误的 MCP 服务器现在自动重试最多 3 次，而不是保持断开连接`ANTHROPIC_BEDROCK_SERVICE_TIER` 选择 Amazon Bedrock 服务层：`default`、`flex` 或 `priority``/terminal-setup` 启用 iTerm2 的剪贴板访问设置，以便 `/copy` 工作，包括来自 tmuxGoogle Cloud 的 Agent Platform 现在支持基于 X.509 证书的工作负载身份联合 (mTLS ADC)重大内存泄漏修复：图像繁重的会话、大型记录历史上的 `/usage` 以及没有进度事件的长时间运行工具

[v2.1.120–v2.1.126 的完整更新日志 →](https://code.claude.com/docs/en/changelog.md#2-1-120)
