> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# 第 28 周 · 2026 年 7 月 6–10 日

> 从 Desktop 应用的内置浏览器浏览外部网站，使用 /doctor 运行完整的设置检查，并获取自动模式的文本记录保护和代理视图升级。

发布版本 [v2.1.202 → v2.1.206](https://code.claude.com/docs/en/changelog.md#2-1-202)
2 项功能 · 7 月 6–10 日

Desktop 上的应用内浏览器
Desktop

Desktop 上的 Claude Code 现在具有内置浏览器。Claude 可以打开文档、设计或任何其他网站，并以与处理本地开发服务器预览相同的方式读取、点击和与页面交互。浏览器是沙箱化的且可配置的：您可以选择浏览会话是否持续，安全分类器会审查对外部网站的操作。

<video autoPlay muted loop playsInline src="https://mintcdn.com/claude-code/x358isu_VzLnyTEN/images/whats-new/desktop-browser.mp4?fit=max&auto=format&n=x358isu_VzLnyTEN&q=85&s=8033e85a1cb0a37870a79e702c18f4e4" />

[浏览外部网站](https://code.claude.com/docs/zh-CN/desktop.md#browse-external-sites)

/doctor 是完整的设置检查
v2.1.205

`/doctor` 现在可以诊断问题并修复它们，而不是打印只读报告。它检查安装健康状况，查找未使用的 skills、MCP 服务器和插件与其上下文成本的对比，针对已检入的文件对本地 `CLAUDE.md` 文件进行重复数据删除，建议修剪 Claude 可以从代码库派生的 `CLAUDE.md` 内容，并标记缓慢的 hooks。它首先报告发现，然后在更改任何内容之前请求确认。`/checkup` 是它的别名。

从任何会话运行检查：

```text Claude Code theme={null}
> /doctor
```

[所有命令](https://code.claude.com/docs/zh-CN/commands.md#all-commands)

其他改进

自动模式现在阻止篡改会话文本记录文件，并在运行 `rm -rf` 处理无法从上下文解析的变量之前询问`/cd` 现在在您输入时建议目录路径，与 `/add-dir` 匹配`/commit-push-pr` 自动允许 `git push` 到存储库配置的推送远程，除了 `origin`Gateway：`/login` 现在支持 Anthropic 运营的公共网关端点`EnterWorktree` 在进入项目 `.claude/worktrees/` 目录之外的 git worktree 之前请求确认后台代理在 Claude Code 更新后立即升级到新版本，而不是在您附加时支付缓慢的过时会话升级代理视图行现在显示彩色状态词和分类器编写的标题，而不是原始工具调用文本，编辑、合并、评论或推送到现有 PR 的会话在 `claude agents` 中链接它自动更新二进制下载现在流式传输到磁盘，而不是在内存中缓冲，将更新程序的峰值内存使用量减少了大约 400 MB后台任务通知现在明确说明没有发生人工输入，防止虚构的文本记录内批准被执行在 Opus 4.8 上改进了 `/code-review` 发现质量，涵盖所有工作量级别

[v2.1.202–v2.1.206 的完整更新日志 →](https://code.claude.com/docs/en/changelog.md#2-1-202)
