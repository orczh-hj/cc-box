> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# 第 26 周 · 2026 年 6 月 22–26 日

> 使用 claude mcp login 从 shell 中对 MCP 服务器进行身份验证，使用 ! 前缀获取对 shell 模式命令输出的响应，以及使用 /rewind 从 /clear 之前恢复对话。

发布版本 [v2.1.185 → v2.1.193](https://code.claude.com/docs/en/changelog.md#2-1-185)
2 项功能 · 6 月 22–26 日

从 CLI 对 MCP 服务器进行身份验证
v2.1.186

新的 `claude mcp login <name>` 和 `claude mcp logout <name>` 命令从 shell 而不是交互式 `/mcp` 菜单对配置的 MCP 服务器进行身份验证。`claude mcp login` 直接运行服务器的 OAuth 流程，`claude mcp logout` 清除存储的凭证。

为配置的服务器运行 OAuth 流程而无需打开会话：

```bash terminal theme={null}
claude mcp login sentry
```

[从命令行进行身份验证](https://code.claude.com/docs/zh-CN/mcp.md#authenticate-from-the-command-line)

Shell 模式响应命令输出
v2.1.186

使用 `!` 前缀运行的命令现在会在输出进入记录后从 Claude 获得响应，因此您可以运行 `! npm test` 并获得对失败的解释，而无需第二个提示。响应成本与发送普通提示相同。要保持之前的行为（其中输出被添加到上下文而不获得响应），请在 `settings.json` 中将 `respondToBashCommands` 设置为 `false`。

运行命令并获得对其输出的响应：

```text Claude Code theme={null}
> ! npm test
```

[带有 ! 前缀的 Shell 模式](https://code.claude.com/docs/zh-CN/interactive-mode.md#shell-mode-with-prefix)

其他改进

`/rewind` 现在可以从运行 `/clear` 之前恢复对话新的 `sandbox.credentials` 设置阻止沙箱命令读取凭证文件和秘密环境变量组织配置的模型限制现在适用于模型选择器、`--model`、`/model` 和 `ANTHROPIC_MODEL`，当选择受限制的模型时显示"受您的组织设置限制"消息新的 `autoMode.classifyAllShell` 设置将所有 Bash 和 PowerShell 命令路由通过自动模式分类器，拒绝原因现在显示在记录、拒绝提示和 `/permissions` 中新的 `claude_code.assistant_response` OpenTelemetry 日志事件携带模型的响应文本；已经记录提示内容的部署在升级时开始接收它，因此设置 `OTEL_LOG_ASSISTANT_RESPONSES=0` 以仅保留提示后台子代理现在在主会话中显示权限提示而不是自动拒绝；对话框显示哪个代理在请求，Esc 仅拒绝该工具`/install-github-app` 现在可以仅安装 GitHub App 并跳过 Actions 工作流和秘密步骤您在沙箱网络权限对话框中允许的主机在会话的其余部分被记住，而不是在每次连接时重新提示流式响应使用的 CPU 减少约 37%，来自终端输出缓存的长会话内存增长被减少`/review <pr>` 现在使用与 `/code-review medium` 相同的审查引擎Bash 模式 `!` 命令获得实时文件路径自动完成

[v2.1.185–v2.1.193 的完整更改日志 →](https://code.claude.com/docs/en/changelog.md#2-1-185)
