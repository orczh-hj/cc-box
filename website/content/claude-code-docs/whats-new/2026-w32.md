> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# 第 32 周 · 2026 年 8 月 3–7 日

> Claude Code 会话可以相互发送消息，自托管环境在您的基础设施上运行云会话，自动模式成为默认权限模式。

发布版本 [v2.1.220 → v2.1.224](https://code.claude.com/docs/en/changelog.md#2-1-220)
3 项功能 · 8 月 3–7 日

跨会话消息传递
v2.1.224

您的 Claude Code 会话现在可以相互发送消息。Claude 使用 `ListAgents` 工具发现您的其他会话，并使用 `SendMessage` 发送消息，可以在您要求时发送，也可以自动发送，例如在一个会话中的更改影响另一个会话的工作后。消息是 Claude 为另一个会话编写的文本，永远不会是您的对话历史或文件。在 macOS 和 Linux 上可用。需要 v2.1.224 或更高版本。

<video autoPlay muted loop playsInline src="https://mintcdn.com/claude-code/N3yEaTYPXMXFrF6k/images/whats-new/cross-session-messaging.mp4?fit=max&auto=format&n=N3yEaTYPXMXFrF6k&q=85&s=8f33c3390f78660a4a26dc980f46159f" />

在同一台机器上打开两个会话，要求其中一个会话传递一些内容：

```text title="Claude Code" wrap theme={null}
告诉处理支付 API 的会话 users.name 现在是 users.display_name
```

一旦 Claude 读取了消息，另一个会话会显示一个 `Message from` 行；按 `Ctrl+O` 展开它。要查看 Claude 可以访问哪些会话，请运行 `/list-agents`。

[向另一个会话发送消息](https://code.claude.com/docs/zh-CN/cross-session-messaging.md#message-another-session)

自托管环境
v2.1.224

自托管环境在您组织自己的基础设施上运行 Claude Code 云会话，在 Team 和 Enterprise 计划上处于公开测试阶段。在您的机器或容器上运行 `claude self-hosted-runner` 将它们转变为运行器。当有人在从 claude.ai、移动或桌面应用程序或 `claude --cloud` 启动会话时选择您的环境时，该会话在您的网络内运行，可以访问您的内部服务。所有者首先在 [管理设置](https://claude.ai/admin-settings/cloud-environments) 中打开 **允许自托管环境**。

![自托管环境管理页面，列出了 linux-dev 和 macos-prod 等环境及其状态和活跃会话计数](https://mintcdn.com/claude-code/N3yEaTYPXMXFrF6k/images/whats-new/self-hosted-environments.jpg?fit=max\&auto=format\&n=N3yEaTYPXMXFrF6k\&q=85\&s=ae9152cb1670c8af517d1aee57689b14)

以所有者身份登录，运行引导式设置，它将引导您创建环境并启动运行器：

```bash terminal theme={null}
claude self-hosted-runner setup
```

运行器注册后，该环境在管理设置中显示 **健康**。

[自托管环境快速入门](https://code.claude.com/docs/zh-CN/self-hosted-environments-quickstart.md#set-up-an-environment-and-runner)

自动模式成为默认值
CLI

从 8 月 14 日开始，自动模式是 Pro、Max 和 Team 计划上新会话的默认权限模式。如果您自己设置了默认模式，它将保持不变，除非您接受一次性切换提示，而您的组织管理的默认值不会改变。您仍然可以随时切换模式。已在这些计划上生效：自动模式进行的分类器调用不再计入您的使用限制。

在切换前以自动模式启动每个会话，请在您的用户设置中将其设置为默认值：

```json ~/.claude/settings.json {3} theme={null}
{
  "permissions": {
    "defaultMode": "auto"
  }
}
```

新会话随后在状态栏中显示 `auto mode on`。

[自动模式要求和控制](https://code.claude.com/docs/zh-CN/permission-modes.md#eliminate-prompts-with-auto-mode)

其他改进

VS Code 扩展获得 [焦点视图](https://code.claude.com/docs/zh-CN/vs-code.md#extension-settings)，它将工具活动隐藏在每轮一个可展开行之后；从命令菜单或使用 `Ctrl+Alt+F`（Mac 上为 `Ctrl+Option+F`）切换它沙箱凭据文件在 Linux 和 WSL2 上接受 [`mode: "mask"`](https://code.claude.com/docs/zh-CN/sandboxing#mask-credential-files)，因此沙箱中的命令读取哨兵副本，而沙箱代理在出口时替换真实值；凭据掩蔽还获得 `extract`、JWT 感知的 `decode` 和 AWS SigV4 重新签名选项市场可以使用新的 `archive` 源将插件分发为 [zip 存档](https://code.claude.com/docs/zh-CN/plugins/marketplace-reference.md#archive-plugin-source)，通过 HTTPS 下载，带有可选的 SHA-256 固定校验，因此安装无需 git 或 npm`/review` 现在是 [`/code-review`](https://code.claude.com/docs/zh-CN/code-review-local) 的别名，`/code-review` 不带 effort 级别时会重用您上次输入的级别您使用 [`/fork`](https://code.claude.com/docs/zh-CN/agent-view#copy-the-session-with-%2Ffork) 复制的会话现在在其自己的 worktree 中进行代码更改，而不是在原始会话的检出中您从 [`/plugin`](https://code.claude.com/docs/zh-CN/plugins/install#install-a-plugin) 安装的插件在安全的情况下会在当前会话中激活；安装摘要会报告 `Plugin is now active.` 或告诉您运行 `/reload-plugins`在 worktree 中更改了代码的[后台会话](https://code.claude.com/docs/zh-CN/agent-view.md#how-file-edits-are-isolated)现在会在完成前提交和推送，仅当任务需要时才打开草稿 Pull Request，并遵循您的 `CLAUDE.md` 中的 git 指令每个会话 200 个子代理的上限被移除，因此长时间运行的会话不再拒绝新的子代理；[并发](https://code.claude.com/docs/zh-CN/sub-agents.md#concurrent-subagent-limit) 和深度限制仍然适用仓库中签入的设置不再能打开 [Remote Control 自动连接](https://code.claude.com/docs/zh-CN/remote-control.md#enable-remote-control-for-all-sessions)；改为在您的用户或托管设置中设置 `remoteControlAtStartup`，项目和本地设置只能将其关闭[Worktree 隔离](https://code.claude.com/docs/zh-CN/worktrees.md#how-claude-code-enforces-isolation) 现在不仅阻止文件编辑，还阻止到达主检出的 Bash 命令和 git 重定向，适用于每种会话类型以及会话的子代理Bash 命令不再能从权限检查中隐藏其自身的一部分，制表符或不可见的 Unicode 填充不再从批准对话框中隐藏命令的一部分PreToolUse 自动允许 hook 不再绕过 Claude Code 内部侧任务（如摘要和压缩）中的工具限制[Ultraplan](https://code.claude.com/docs/zh-CN/ultraplan) 研究预览版被移除，包括 `/ultraplan` 命令和 `ultraplan` 关键字；改为使用计划模式或 Web 版 Claude Code

[v2.1.220–v2.1.224 的完整更新日志 →](https://code.claude.com/docs/en/changelog.md#2-1-220)
