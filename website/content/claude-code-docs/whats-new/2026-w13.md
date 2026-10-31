> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# 第13周 · 2026年3月23–27日

> 自动模式用于免提权限、内置计算机使用、云端PR自动修复、转录搜索和Windows PowerShell工具。

发布版本 [v2.1.83 → v2.1.85](https://code.claude.com/docs/en/changelog.md#2-1-83)
6项功能 · 3月23–27日

自动模式
研究预览

自动模式将您的权限提示交给分类器处理。安全的编辑和命令无需中断即可运行；任何破坏性或可疑的操作都会被阻止并显示。这是批准每个文件写入和使用 `--dangerously-skip-permissions` 运行之间的折中方案。

![Claude Code 提示页脚显示黄色的'自动模式开启（Shift+Tab切换）'指示器](https://mintcdn.com/claude-code/CfffsX01JHFnIKvD/images/whats-new/auto-mode.png?fit=max\&auto=format\&n=CfffsX01JHFnIKvD\&q=85\&s=367c9e9d4ba5bc57ec4b935154bf1fbb)

使用 Shift+Tab 切换到自动模式，或将其设置为默认值：

```json ~/.claude/settings.json {3} theme={null}
{
  "permissions": {
    "defaultMode": "auto"
  }
}
```

[权限模式指南](https://code.claude.com/docs/zh-CN/permission-modes.md)

计算机使用
Desktop

Claude 现在可以从 Claude Code Desktop 应用控制您的实际桌面：打开原生应用、点击 iOS 模拟器、驱动硬件控制面板，并验证屏幕上的更改。默认情况下它是关闭的，每次操作前都会询问。最适合用于其他方法无法到达的事情：没有 API 的应用、专有工具、任何仅作为 GUI 存在的东西。

![Claude Desktop 设置，计算机使用切换已启用，显示允许 Claude 在您允许的应用中截屏和控制键盘和鼠标的选项](https://mintcdn.com/claude-code/CfffsX01JHFnIKvD/images/whats-new/computer-use.png?fit=max\&auto=format\&n=CfffsX01JHFnIKvD\&q=85\&s=d631de2017edafff463505f8ddbc0f51)

在设置中启用它，授予操作系统权限，然后要求 Claude 端到端验证更改：

```text title="Claude Code" wrap theme={null}
打开 iOS 模拟器，点击完成入门流程，并对每个步骤进行截屏
```

[计算机使用指南](https://code.claude.com/docs/zh-CN/desktop.md#let-claude-use-your-computer)

PR 自动修复
Web

打开 PR 时切换开关然后离开。Claude 监视 CI，修复失败，处理细节，并推送直到通过。不再需要通过六轮 lint 错误来照看 PR。

![Claude Code web CI 面板显示自动修复切换已启用，描述为'主动修复 CI 失败和审查评论'](https://mintcdn.com/claude-code/CfffsX01JHFnIKvD/images/whats-new/auto-fix.png?fit=max\&auto=format\&n=CfffsX01JHFnIKvD\&q=85\&s=c62b181c6c5d96929f0b43525f9f3584)

在 Claude Code web 上创建 PR 后，在 CI 面板中切换自动修复。

[自动修复拉取请求](https://code.claude.com/docs/zh-CN/claude-code-on-the-web.md#auto-fix-pull-requests)

转录搜索
v2.1.83

在转录模式中按 `/` 搜索您的对话。`n` 和 `N` 逐步浏览匹配项。最后有办法找到 Claude 在 400 条消息前运行的那个 Bash 命令。

打开转录模式并搜索：

```text Claude Code theme={null}
Ctrl+O    # 打开转录
/migrate  # 搜索"migrate"
n         # 下一个匹配项
N         # 上一个匹配项
```

[全屏指南](https://code.claude.com/docs/zh-CN/fullscreen.md#search-and-review-the-conversation)

PowerShell 工具
预览
v2.1.84

Windows 获得了与 Bash 并行的原生 PowerShell 工具。Claude 可以运行 cmdlet、管道对象，并使用 Windows 原生路径，无需通过 Git Bash 翻译所有内容。

从设置中选择加入：

```json .claude/settings.json {3} theme={null}
{
  "env": {
    "CLAUDE_CODE_USE_POWERSHELL_TOOL": "1"
  }
}
```

[PowerShell 工具文档](https://code.claude.com/docs/zh-CN/tools-reference.md#powershell-tool)

条件 hooks
v2.1.85

Hooks 现在可以使用权限规则语法声明 `if` 字段。您的 pre-commit 检查仅针对 `Bash(git commit *)` 生成，而不是每个 bash 调用，减少繁忙会话中的进程开销。

将 hook 限制为仅 git 提交：

```json .claude/settings.json {5} theme={null}
{
  "hooks": {
    "PreToolUse": [{
      "hooks": [{
        "if": "Bash(git commit *)",
        "type": "command",
        "command": ".claude/hooks/lint-staged.sh"
      }]
    }]
  }
}
```

[Hooks 参考](https://code.claude.com/docs/zh-CN/hooks.md)

其他亮点

Plugin `userConfig` 现已公开：在启用时提示设置，钥匙串支持的密钥粘贴的图像插入 `[Image #N]` 芯片，您可以按位置引用`managed-settings.d/` 分层策略片段的 drop-in 目录`CwdChanged` 和 `FileChanged` hook 事件用于 direnv 风格的设置Agents 可以在 frontmatter 中声明 `initialPrompt` 以自动提交第一轮`Ctrl+X Ctrl+E` 打开您的外部编辑器，匹配 readline在任何响应前中断会自动恢复您的输入`/status` 现在在 Claude 响应时也能工作深层链接在您首选的终端中打开，而不是首次检测到的离开 75+ 分钟后的空闲返回提示到 `/clear`VS Code：速率限制横幅，Esc 两次倒带选择器

[v2.1.83–v2.1.85 完整更新日志 →](https://code.claude.com/docs/en/changelog.md#2-1-83)
