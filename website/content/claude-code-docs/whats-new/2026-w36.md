> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# 第 36 周 · 8 月 31 日 – 9 月 4 日，2026 年

> 切换到 Claude Fable 5.1，让计算机使用在 Desktop 上后台运行，并在实时 /diff 面板中观看 Claude 的编辑。

Releases [v2.1.251 → v2.1.261](https://code.claude.com/docs/en/changelog.md#2-1-251)
4 个功能 · 8 月 31 日 – 9 月 4 日

Claude Fable 5.1
新模型

Claude Fable 5.1 在 Claude Code 中可用，具有 1M 令牌上下文窗口，`fable` 别名现在选择它。在 Claude 应用网关会话中，`fable` 仍然选择 Fable 5。如果您的网关提供 Fable 5.1，请运行 `/model claude-fable-5-1`。需要 v2.1.257 或更高版本。

将当前会话切换到 Fable 5.1 并将其保存为默认值：

```text Claude Code theme={null}
> /model fable
```

在 Anthropic API 上，选择器仅在服务器报告您的组织可用时才列出 Fable，但键入 `/model fable` 会直接与服务器检查。

[使用 Fable](https://code.claude.com/docs/zh-CN/model-config.md#work-with-fable)

计算机使用在 Desktop 上后台运行
Desktop

在 macOS 上，Claude Code Desktop 应用中的计算机使用现在可以在后台工作：Claude 可以在您批准的应用中查看和操作，同时您继续工作。后台计算机使用在 Pro 和 Max 计划上处于测试阶段。

![一个 Claude Code Desktop 会话，其中 Claude 请求使用 Xcode，旁边有一张计算机使用权限卡，上面写着"让 Claude 在您批准的应用中查看和操作，在后台或完全控制您的屏幕"，以及一个"启用"按钮](https://mintcdn.com/claude-code/f9HTZGyMtxIFOUgt/images/whats-new/background-computer-use.jpg?fit=max\&auto=format\&n=f9HTZGyMtxIFOUgt\&q=85\&s=a599a6c6fa544cb8d1b426b93706caf4)

[让 Claude 使用您的计算机](https://code.claude.com/docs/zh-CN/desktop.md#let-claude-use-your-computer)

全屏渲染中的实时 diff 面板
v2.1.260

在全屏渲染中，`/diff` 现在在对话旁边打开一个面板，而不是您必须关闭的查看器。该面板列出更改的文件及其添加和删除的行数，并在每次 Claude 编辑文件或运行 shell 命令时刷新。在面板中用鼠标选择行以将其附加到您的下一个提示。

<video autoPlay muted loop playsInline src="https://mintcdn.com/claude-code/f9HTZGyMtxIFOUgt/images/whats-new/diff-panel.mp4?fit=max&auto=format&n=f9HTZGyMtxIFOUgt&q=85&s=9d7553c19e7f227891cd95f1f59d796d" />

启用全屏渲染，在 git 存储库中，并在至少 110 列宽的终端中，切换面板：

```text Claude Code theme={null}
> /diff
```

再次运行 `/diff` 或单击其标题中的 `✕` 来关闭它。

[Diff 面板](https://code.claude.com/docs/zh-CN/interactive-mode.md#diff-panel)

使用 /skill-doctor 查找未使用的 skills
CLI

`/skill-doctor` 显示您的每个 skill 在上下文中的成本以及它被使用的频率，因此您可以决定关闭哪些。[skill 列表](https://code.claude.com/docs/zh-CN/skills.md#skill-descriptions-are-cut-short)中的每个 skill 都会在每个回合中添加到您的上下文中，无论 Claude 是否使用它。需要 v2.1.252 或更高版本，在跳过[功能标志获取](https://code.claude.com/docs/zh-CN/env-vars.md#features-that-need-feature-flag-fetching)的会话中不可用。

在交互式会话中运行它以在 `/plugin` 管理器的**Stats**选项卡中打开报告：

```text Claude Code theme={null}
> /skill-doctor
```

在非交互模式下使用 `-p`，Claude Code 会将报告打印为文本。

[查找未使用的 skills](https://code.claude.com/docs/zh-CN/skills.md#find-unused-skills)

其他亮点

[`PreModelSwitch`](https://code.claude.com/docs/zh-CN/hooks#premodelswitch) hook 可以阻止您请求的模型切换，[`PostModelSwitch`](https://code.claude.com/docs/zh-CN/hooks#postmodelswitch) hook 可以在会话的模型更改后为 Claude 添加上下文[`/cost`](https://code.claude.com/docs/zh-CN/costs#prompt-cache-statistics) 添加了一行 `Prompt cache (main)`：从缓存提供的输入令牌份额、缓存未命中、缓存是否预热，以及当 Claude Code 可以命名一个时上次未命中的可能原因。状态行脚本获得匹配的 `prompt_cache` 对象组织可以在[`managedMcpServers`](https://code.claude.com/docs/zh-CN/managed-mcp#provide-servers-through-managed-settings) 托管设置下列出 HTTP 和 SSE MCP 服务器，以将其提供给每个用户，除了用户自己添加的服务器`/effort` 和 `/model` 选择器现在[为每个模型保存单独的努力级别](https://code.claude.com/docs/zh-CN/model-config.md#adjust-effort-level)；按 `s` 而不是 `Enter` 仅将级别应用于当前会话默认情况下，自动模式分类器[现在也阻止](https://code.claude.com/docs/zh-CN/permission-modes.md#what-the-classifier-blocks-by-default)诸如从云实例元数据端点请求凭证或连接到 Claude 未启动的同级容器等操作在自动模式下，Claude Code 在 Claude [首次读取工作目录外的文件](https://code.claude.com/docs/zh-CN/permission-modes.md#first-read-outside-the-working-directories)之前询问您，并提供从那时起阻止此类读取的选项提高 [`bashOutputMaxChars`](https://code.claude.com/docs/zh-CN/settings-reference#bashoutputmaxchars) 和 [`taskOutputMaxChars`](https://code.claude.com/docs/zh-CN/settings-reference#taskoutputmaxchars)，最高可达 128,000 个字符，以便 Claude 内联接收来自成功命令或后台任务的更多输出提示的[字编辑快捷键遵循 readline](https://code.claude.com/docs/zh-CN/interactive-mode.md#make-ctrl-w-delete-back-to-whitespace) 对所有人，`keybindingFlavor` 设置不再有任何效果。`Ctrl+W` 删除回到前一个空格，`Alt+B`、`Alt+F` 和 `Alt+D` 将标点符号（如 `/` 和 `.`）视为单词分隔符如果您在项目的 `.claude/settings.json` 或 `.claude/settings.local.json` 中将 `defaultMode` 设置为 `"bypassPermissions"`，它[不再生效](https://code.claude.com/docs/zh-CN/permission-modes.md#which-mode-a-session-starts-in)，会话以手动模式启动；改为在用户或托管设置中设置 `"bypassPermissions"`，或传递 `--permission-mode`基于座位的企业计划现在[默认为 Opus 5](https://code.claude.com/docs/zh-CN/model-config.md#default-model-setting)在 VS Code 扩展中，单击提示框底部的模型名称以[打开模型选择器](https://code.claude.com/docs/zh-CN/vs-code.md#use-the-prompt-box)在 VS Code 扩展中，在命令菜单的"自定义"部分中选择**输出样式**以[选择输出样式](https://code.claude.com/docs/zh-CN/vs-code.md#use-the-prompt-box)，包括您的自定义样式

[v2.1.251–v2.1.261 的完整更新日志 →](https://code.claude.com/docs/en/changelog.md#2-1-251)
