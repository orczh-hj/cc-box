> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# 第 22 周 · 2026 年 5 月 25–29 日

> 在 Claude Opus 4.8 上运行 Claude Code，使用动态工作流编排大型任务，使用 security-guidance 插件捕获安全问题，并以更低的价格在 Opus 4.8 上使用快速模式。

发布版本 [v2.1.150 → v2.1.157](https://code.claude.com/docs/en/changelog.md#2-1-150)
4 项功能 · 5 月 25–29

Claude Opus 4.8
新模型

Opus 4.8 现在是 Max、Team Premium、Enterprise 按量付费和 Anthropic API 上的默认模型。它默认为高努力级别；对于更难的任务，使用 `/effort xhigh`。需要 v2.1.154 或更高版本。

<video autoPlay muted loop playsInline src="https://mintcdn.com/claude-code/QsIrGXGFg6xd7joy/images/whats-new/opus-4-8.mp4?fit=max&auto=format&n=QsIrGXGFg6xd7joy&q=85&s=6ebcf5fe136467da2b254de1fe749ea7" />

按名称切换到 Opus 4.8，或从模型选择器中选择它：

```text title="Claude Code" wrap theme={null}
/model claude-opus-4-8
```

[模型配置](https://code.claude.com/docs/zh-CN/model-config.md#available-models)

Dynamic workflows
研究预览

工作流是 Claude 为您的任务编写的编排脚本，在后台跨许多子代理运行。当任务太大而无法由一个对话来协调时，请使用工作流：整个代码库审计、大型迁移、需要交叉检查的研究问题。使用 `/workflows` 管理运行。

![Claude Code on Opus 4.8 showing a Dynamic workflow requested indicator for a prompt that asks for a workflow to migrate every internal fetch() call](https://mintcdn.com/claude-code/QsIrGXGFg6xd7joy/images/whats-new/dynamic-workflows.png?fit=max\&auto=format\&n=QsIrGXGFg6xd7joy\&q=85\&s=26671fa8607cec3453ed9753f821bd4f)

描述任务并请求工作流：

```text title="Claude Code" wrap theme={null}
create a workflow that migrates every internal fetch() call to the new HttpClient wrapper
```

[Dynamic workflows](https://code.claude.com/docs/zh-CN/workflows.md)

Security guidance plugin
插件

security-guidance 插件审查 Claude 的代码更改是否存在漏洞，并在同一会话中修复它们。它在每次编辑时运行快速模式检查，在每个回合结束时进行模型审查，以及在提交或推送时进行更深入的代理审查。在 `.claude/claude-security-guidance.md` 中添加项目规则。

<video autoPlay muted loop playsInline src="https://mintcdn.com/claude-code/QsIrGXGFg6xd7joy/images/whats-new/security-guidance.mp4?fit=max&auto=format&n=QsIrGXGFg6xd7joy&q=85&s=c91d865936411586f42b24c558bcdd1d" />

从官方 Anthropic 市场安装它：

```text title="Claude Code" wrap theme={null}
/plugin install security-guidance@claude-plugins-official
```

然后在当前会话中激活它：

```text title="Claude Code" wrap theme={null}
/reload-plugins
```

[Security guidance plugin](https://code.claude.com/docs/zh-CN/security-guidance.md)

Fast mode on Opus 4.8
研究预览

快速模式现在默认为 Opus 4.8，价格为 \$10/\$50 每 MTok：标准速率的 2 倍，速度约为 2.5 倍。Opus 4.7 和 4.6 保持在 \$30/\$150。Opus 4.6 快速模式已弃用。

切换快速模式，现在在 Opus 4.8 上：

```text title="Claude Code" wrap theme={null}
/fast
```

[快速模式定价](https://code.claude.com/docs/zh-CN/fast-mode.md#understand-the-cost-tradeoff)

其他亮点

在 `claude agents` 中，在 shell 命令前加上 `!` 以将其作为后台作业运行，您可以附加到该作业并从中分离；也可用作 `claude --bg --exec 'pytest -x'``.claude/skills` 目录中的插件现在会自动加载，无需市场，`claude plugin init <name>` 为新插件搭建框架新的 `/reload-skills` 命令重新扫描技能目录而无需重启，`SessionStart` hooks 可以返回 `reloadSkills: true` 以使它们安装的技能在同一会话中可用技能和命令可以在 frontmatter 中设置 `disallowed-tools` 以在技能处于活动状态时从模型中删除工具新的 `MessageDisplay` hook 事件让 hooks 在显示助手消息文本时转换或隐藏它Claude Code 现在在找不到主模型时切换到您配置的 `--fallback-model` 以继续会话的其余部分，而不是使每个请求都失败插件可以在 `plugin.json` 或市场条目中声明 `defaultEnabled: false`，以便它们安装时不会打开，直到您启用它们Vim 模式：在 NORMAL 模式下按 `/` 打开反向历史搜索，与 Bash 和 Zsh vi-mode 匹配流式工具执行现在始终启用，包括在禁用遥测和在 Amazon Bedrock、Google Cloud 的 Agent Platform 和 Microsoft Foundry 上`←←` 打开代理视图现在在 Amazon Bedrock、Google Cloud 的 Agent Platform、Microsoft Foundry 和禁用遥测时有效Chrome 中的 Claude：通过 `/chrome` → "选择浏览器…" 选择要使用的连接浏览器，或在浏览器操作运行时在聊天中选择（当有多个连接时）`claude mcp list` 和 `claude mcp get` 现在将未批准的 `.mcp.json` 服务器显示为待批准，而不是在输出被管道传输时自动批准和连接

[v2.1.150–v2.1.157 的完整更新日志 →](https://code.claude.com/docs/en/changelog.md#2-1-150)
