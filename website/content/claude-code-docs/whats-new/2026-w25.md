> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# 第 25 周 · 2026 年 6 月 15–19 日

> 从您的会话中使用 Artifacts 发布实时可共享页面，在拒绝和询问规则中匹配工具参数，以及使用 /config 从提示中设置任何设置。

发布版本 [v2.1.178 → v2.1.183](https://code.claude.com/docs/en/changelog.md#2-1-178)
3 项功能 · 6 月 15–19 日

Artifacts

Artifact 是一个实时交互式页面，Claude Code 从您的会话发布到 claude.ai 上的私有 URL，并在会话继续工作时实时更新。当终端文本不是合适的媒介时，请要求创建一个，例如带有内联注释差异的 PR 演练或从会话数据构建的仪表板。Artifacts 在 Team 和 Enterprise 计划中处于测试阶段。

<video autoPlay muted loop playsInline src="https://mintcdn.com/claude-code/1ylKDoQynT1UgfEK/images/whats-new/artifacts.mp4?fit=max&auto=format&n=1ylKDoQynT1UgfEK&q=85&s=7f5391559d2bc69989621b36322fcff1" />

向 Claude 要求一个页面，然后批准发布提示：

```text title="Claude Code" wrap theme={null}
Make an artifact that walks through this PR with the diff annotated inline.
```

[创建 Artifact](https://code.claude.com/docs/zh-CN/artifacts.md#create-an-artifact)

按输入参数匹配
v2.1.178

拒绝和询问权限规则现在可以使用 `Tool(param:value)` 语法匹配工具的输入参数。例如，`Agent(model:opus)` 匹配请求 Opus 模型层级的子代理生成。该值接受 `*` 作为通配符，因此 `Agent(isolation:*)` 匹配任何显式隔离值。

在 `settings.json` 中的拒绝列表中添加参数规则：

```json .claude/settings.json {3} theme={null}
{
  "permissions": {
    "deny": ["Agent(model:opus)"]
  }
}
```

[按输入参数匹配](https://code.claude.com/docs/zh-CN/permissions.md#match-by-input-parameter)

从提示中设置任何设置
v2.1.181

将 `key=value` 传递给 `/config` 以直接更改设置，无需打开设置界面。该语法也适用于使用 `-p` 标志的非交互模式和远程控制。

从提示中设置 `thinking` 设置：

```text title="Claude Code" wrap theme={null}
/config thinking=false
```

[命令参考](https://code.claude.com/docs/zh-CN/commands.md#all-commands)

其他改进

自动模式现在在您未要求丢弃本地工作时阻止破坏性 git 命令（`git reset --hard`、`git clean -fd`、`git stash drop`），并在您未要求特定堆栈时阻止 `terraform destroy`将新的 `attribution.sessionUrl` 设置设为 `false` 以在 web 和远程控制会话中的提交和 PR 中省略 claude.ai 会话链接在 `/config` 界面中，Enter 和 Space 都可以更改选定的设置，Esc 现在保存并关闭而不是还原新的 `sandbox.allowAppleEvents` 选择加入设置允许沙箱命令在 macOS 上发送 Apple Events指向 `CLAUDE_CLIENT_PRESENCE_FILE` 到标记文件以在您在机器上时抑制移动推送通知长段落现在逐行流式传输，而不是等待第一个换行符API 连接在思考中断开现在自动重试，而不是显示"思考时连接已关闭"设置 `CLAUDE_CODE_EXPERIMENTAL_AGENT_TEAMS=1` 后，每个会话都有一个隐式团队，因此您可以直接使用 Agent 工具的 `name` 参数生成队友嵌套 `.claude/skills` 目录中的 Skills 在处理那里的文件时加载；在名称冲突时，嵌套 skill 显示为 `<dir>:<name>`，以便两者都保持可用修复了 prompt caching 在自定义 `ANTHROPIC_BASE_URL` 和 Foundry 上不读取的问题修复了 Write 和 Edit 在网络驱动器和云同步文件夹上生成零字节或截断文件的问题

[v2.1.178–v2.1.183 完整更新日志 →](https://code.claude.com/docs/en/changelog.md#2-1-178)
