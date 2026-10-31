> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# 第 34 周 · 2026 年 8 月 17–21 日

> 使用 /design skill 草拟可编辑的 UI 画板，设置 Concise 输出样式，并从手机在您的机器上启动 Claude Code 会话。

发布版本 [v2.1.234 → v2.1.239](https://code.claude.com/docs/en/changelog.md#2-1-234)
3 项功能 · 8 月 17–21

/design
research preview

`/design` skill 将 Claude Design 的画板工作流程引入 CLI 和 Claude Code Desktop，基于 artifacts 构建。使用简要说明运行它，Claude 会发布一个可编辑画板的画布供您的 UI 使用。选择一个，调整它，然后让 Claude 实现它。适用于 Pro、Max、Team 和 Enterprise。需要 v2.1.234 或更高版本。

<video autoPlay muted loop playsInline src="https://mintcdn.com/claude-code/2SnAdpL4dJ18nKb3/images/whats-new/design-skill.mp4?fit=max&auto=format&n=2SnAdpL4dJ18nKb3&q=85&s=0b376a94227c14a4204af89c4c9fd7ac" />

描述您想要设计的内容，让 Claude 草拟选项：

```text Claude Code theme={null}
> /design redesign the composer based on what people actually use it for
```

Claude 打印已发布画布的链接。打开它，选择一个画板，并告诉 Claude 要实现哪个选项。

[artifacts 可用的位置](https://code.claude.com/docs/zh-CN/artifacts.md#availability)

Concise 输出样式
v2.1.237

Concise 是一种新的内置输出样式。Claude 以结果开头，跳过前言和叙述，同时以与默认样式相同的彻底程度完成工作。当您要求解释或更多详细信息时，Claude 会完整回答。错误报告、安全警告和破坏性操作的确认保持其完整内容。

<video autoPlay muted loop playsInline src="https://mintcdn.com/claude-code/2SnAdpL4dJ18nKb3/images/whats-new/concise-output-style.mp4?fit=max&auto=format&n=2SnAdpL4dJ18nKb3&q=85&s=dfb40ec8921ed1bc82eb629042a8ec17" />

在 `/config` 中的 **Output style** 下打开它，或在您的设置文件中设置它：

```json ~/.claude/settings.json {2} theme={null}
{
  "outputStyle": "Concise"
}
```

运行 `/clear` 或启动新会话，Claude 的回复以结果开头。

[内置输出样式](https://code.claude.com/docs/zh-CN/output-styles.md#built-in-output-styles)

从手机在您的机器上启动会话
mobile

任何运行 `claude remote-control` 的机器现在都会在 Claude 应用的 Code 标签页顶部显示为设备卡片。Remote Control 也已退出研究预览。

![Claude 移动应用中的 Code 标签页，其中 Devices 部分显示连接的 MacBook 作为设备卡片，位于会话列表上方](https://mintcdn.com/claude-code/2SnAdpL4dJ18nKb3/images/whats-new/remote-control-phone-start.jpg?fit=max\&auto=format\&n=2SnAdpL4dJ18nKb3\&q=85\&s=9f0ebedab23aa0e1732cc37782573907)

在您想要访问的机器上启动 Remote Control，然后在手机上打开 Code 标签页：

```bash terminal theme={null}
claude remote-control
```

您的机器显示为 Code 标签页顶部的设备卡片。点击它以选择目录并在那里启动会话。

[启动 Remote Control 会话](https://code.claude.com/docs/zh-CN/remote-control.md#start-a-remote-control-session)

其他改进

Claude Code 现在在 claude.ai 使用限制重置时自动继续您的会话；从 `/config` 中的 **Continue automatically at usage limit** 行关闭它可选的 [`spellcheck` 设置](https://code.claude.com/docs/zh-CN/interactive-mode#check-spelling-as-you-type)在您键入时在提示输入中为拼写错误的单词加下划线，使用您安装的 `aspell`、`hunspell` 或 `ispell`在具有开放 GitLab 合并请求的分支上，使用通过 `glab auth login` 进行身份验证的 `glab` CLI，页脚显示一个 [`MR !N` 徽章](https://code.claude.com/docs/zh-CN/interactive-mode#gitlab-merge-requests)，其颜色取决于合并请求是草稿、开放还是可合并从手机或 claude.ai/code 更改工作量级别，它 [应用于您机器上的会话](https://code.claude.com/docs/zh-CN/remote-control.md#what-connected-devices-see)；由 Desktop 或 VS Code 托管的 Remote Control 会话也向连接的设备显示会话的当前权限模式您可以在 Claude 工作时打开 [`/permissions`](https://code.claude.com/docs/zh-CN/permissions#manage-permissions) 或运行 `/add-dir <path>`；权限规则更改适用于当前轮次的其余部分当后台任务使 [`/goal`](https://code.claude.com/docs/zh-CN/goal#background-work-defers-evaluation) 等待时，Claude 在 30 分钟后检查它们，而不是无限期等待，并继续检查，在会话空闲时以更长的间隔检查；设置 `CLAUDE_CODE_GOAL_CHECKIN_MINUTES=0` 以选择退出您自己的提示现在在成绩单中呈现 markdown，具有突出显示的代码块、内联代码和列表，与回复的方式相同新的 [`ANTHROPIC_DEFAULT_MODEL`](https://code.claude.com/docs/zh-CN/model-config#set-a-default-model-for-new-sessions) 环境变量设置新会话启动的模型；`/model` 选择仍会覆盖它并在重启后保持使用 `SendMessage` 上的 `notify_when_idle` 输入，Claude 可以要求同一机器上的另一个 Claude Code 会话 [在它下次空闲时发送一个通知](https://code.claude.com/docs/zh-CN/cross-session-messaging.md#get-a-notice-when-another-session-goes-idle)将 [`keybindingFlavor`](https://code.claude.com/docs/zh-CN/interactive-mode#make-ctrl-w-delete-back-to-whitespace) 设置为 `"readline"`，使提示中的 `Ctrl+W` 删除回到前一个空格，如 Bash 所做的那样，而不是在标点符号（如 `/`）处停止在本机 Windows 上，您的 Claude Code 会话现在可以 [相互消息](https://code.claude.com/docs/zh-CN/cross-session-messaging.md#availability)，使用 `SendMessage` 并使用 `ListAgents` 找到彼此，如在 macOS 和 Linux 上一样自托管运行器接受 `--defer-shutdown-max-min`，它 [在 SIGTERM 后的设定分钟数内继续为附加会话提供服务](https://code.claude.com/docs/zh-CN/self-hosted-environments-deploy.md#defer-the-drain-past-the-first-signal)自托管运行器接受 `--proxy-authorization-command` 或 `--proxy-authorization-file` 为 [需要一个的出口代理提供新的 `Proxy-Authorization` 标头](https://code.claude.com/docs/zh-CN/self-hosted-environments-deploy#authenticate-to-an-egress-proxy)

[v2.1.234–v2.1.239 的完整更新日志 →](https://code.claude.com/docs/en/changelog.md#2-1-234)
