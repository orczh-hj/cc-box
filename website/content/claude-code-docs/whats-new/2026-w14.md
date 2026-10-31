> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# 第 14 周 · 3 月 30 日 – 4 月 3 日，2026 年

> CLI 中的计算机使用、交互式产品内课程、无闪烁渲染、按工具 MCP 结果大小覆盖以及 PATH 上的插件可执行文件。

发布版本 [v2.1.86 → v2.1.91](https://code.claude.com/docs/en/changelog.md#2-1-86)
5 项功能 · 3 月 30 日 – 4 月 3 日

CLI 中的计算机使用
研究预览

上周计算机使用功能登陆了桌面应用。本周它进入了 CLI：Claude 可以打开原生应用、点击 UI、测试自己的更改，以及修复损坏的内容，所有这些都可以从你的终端完成。Web 应用已经有验证循环；原生 iOS、macOS 和其他仅限 GUI 的应用没有。现在有了。最适合在没有 API 可调用的应用和工具上闭合循环。仍处于早期阶段；预期会有粗糙的边缘。

<video autoPlay muted loop playsInline src="https://mintcdn.com/claude-code/CfffsX01JHFnIKvD/images/whats-new/cli-computer-use.mp4?fit=max&auto=format&n=CfffsX01JHFnIKvD&q=85&s=c17a337902308d7c9121013ded0494db" />

需要 macOS 和 Pro 或 Max 计划；否则，`computer-use` 不会出现在 `/mcp` 中。运行 `/mcp`，找到 `computer-use`，然后将其打开。然后要求 Claude 端到端验证更改：

```text title="Claude Code" wrap theme={null}
Open the iOS simulator, tap through onboarding, and screenshot each step
```

[计算机使用指南](https://code.claude.com/docs/zh-CN/computer-use.md)

/powerup
v2.1.90

交互式课程，通过动画演示教授 Claude Code 功能，直接在你的终端内进行。Claude Code 发布频繁，上个月会改变你工作方式的功能可能会被遗漏。运行一次 `/powerup`，你就会知道有什么功能。

<video autoPlay muted loop playsInline src="https://mintcdn.com/claude-code/CfffsX01JHFnIKvD/images/whats-new/powerup.mp4?fit=max&auto=format&n=CfffsX01JHFnIKvD&q=85&s=fb88beddc0ecc8029da5ab029e4b28f1" />

运行它：

```text title="Claude Code" wrap theme={null}
/powerup
```

[命令参考](https://code.claude.com/docs/zh-CN/commands.md)

无闪烁渲染
v2.1.89

选择加入新的替代屏幕渲染器，具有虚拟化的回滚。提示输入保持固定在底部，鼠标选择可跨长对话工作，重绘时的闪烁消失了。取消设置 `CLAUDE_CODE_NO_FLICKER` 以回滚。

<video autoPlay muted loop playsInline src="https://mintcdn.com/claude-code/CfffsX01JHFnIKvD/images/whats-new/flicker-free.mp4?fit=max&auto=format&n=CfffsX01JHFnIKvD&q=85&s=7719e35e52a3f9734b0cf69edac333ad" />

设置环境变量并重启 Claude Code：

```bash theme={null}
export CLAUDE_CODE_NO_FLICKER=1
claude
```

[全屏渲染](https://code.claude.com/docs/zh-CN/fullscreen.md)

MCP 结果大小覆盖
v2.1.91

MCP 服务器作者现在可以通过在工具的 `tools/list` 条目中设置 `anthropic/maxResultSizeChars` 来提高特定工具的截断上限，最高可达 500K 字符的硬上限。上限曾经是全局的，所以偶尔返回数据库架构或完整文件树等固有大型有效负载的工具会达到默认限制，并被持久化到磁盘，带有文件引用。按工具覆盖在工具真正需要时将这些结果保持内联。

在你的服务器的 `tools/list` 响应中注释工具：

```json highlight={5} theme={null}
{
  "name": "get_schema",
  "description": "Returns the full database schema",
  "_meta": {
    "anthropic/maxResultSizeChars": 500000
  }
}
```

[MCP 参考](https://code.claude.com/docs/zh-CN/mcp.md#raise-the-limit-for-a-specific-tool)

PATH 上的插件可执行文件
v2.1.91

在插件根目录的 `bin/` 目录中放置可执行文件，Claude Code 会在启用插件时将该目录添加到 Bash 工具的 `PATH`。Claude 随后可以从任何 Bash 工具调用中将二进制文件作为裸命令调用，无需绝对路径或包装脚本。便于将 CLI 助手与调用它们的命令、代理和钩子一起打包。

在插件根目录添加 `bin/` 目录：

```text highlight={4, 5} theme={null}
my-plugin/
├── .claude-plugin/
│   └── plugin.json
└── bin/
    └── my-tool
```

[插件参考](https://code.claude.com/docs/zh-CN/plugins/manifest-reference.md#standard-layout)

其他成就

自动模式后续：新的 `PermissionDenied` 钩子在分类器拒绝时触发（返回 `retry: true` 让 Claude 尝试不同的方法），`/permissions` → 最近拒绝让你用 `r` 手动重试`PreToolUse` 钩子中 `permissionDecision` 的新 `defer` 值：`-p` 会话在工具调用处暂停并以 `deferred_tool_use` 有效负载退出，以便 SDK 应用或自定义 UI 可以显示它，然后用 `--resume` 恢复`/buddy`：孵化一个小生物来观看你编码。一个愚人节玩笑，不再可用`disableSkillShellExecution` 设置阻止来自技能、斜杠命令和插件命令的内联 shell编辑工具现在可以在通过 `cat` 或 `sed -n` 查看的文件上工作，无需单独的读取超过 50K 的钩子输出保存到磁盘，带有路径和预览，而不是注入到上下文中思考摘要在交互式会话中默认关闭（`showThinkingSummaries: true` 以恢复）语音模式：按住说话修饰符组合、Windows WebSocket、macOS Apple Silicon 麦克风权限`claude-cli://` 深层链接接受多行提示（编码 `%0A`）

[v2.1.86–v2.1.91 的完整更改日志 →](https://code.claude.com/docs/en/changelog.md#2-1-86)
