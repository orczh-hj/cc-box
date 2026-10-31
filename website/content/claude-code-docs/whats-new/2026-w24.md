> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# 第24周 · 2026年6月8日–12日

> 使用 /cd 将会话移动到新目录，让子代理生成自己的子代理，并使用安全模式排查损坏的配置。

发布版本 [v2.1.166 → v2.1.176](https://code.claude.com/docs/en/changelog.md#2-1-166)
3 项功能 · 6月8日–12日

使用 /cd 移动会话
v2.1.169

新的 `/cd` 命令将当前会话移动到不同的工作目录，而无需重建提示缓存：新目录的 `CLAUDE.md` 作为消息附加，而不是替换系统提示。会话重新定位到新目录的项目存储，因此 `--resume` 和 `--continue` 会在那里找到它。如果您之前未在该目录中工作过，Claude 会提示您信任该目录。

将会话移动到另一个项目而无需重新启动：

```text Claude Code theme={null}
> /cd ../other-project
```

[命令参考](https://code.claude.com/docs/zh-CN/commands.md#all-commands)

子代理可以生成子代理
v2.1.172

子代理现在可以生成自己的子代理。提示下方的子代理面板显示完整的树：每一行都包含其后代的计数和返回到 `main` 的路径。子代理链的深度限制为五级，以防止失控的并发树。

打开代理视图以观看嵌套树的工作展开：

```text Claude Code theme={null}
> /agents
```

[生成嵌套子代理](https://code.claude.com/docs/zh-CN/sub-agents.md#let-subagents-spawn-their-own-subagents)

使用安全模式排查问题
v2.1.169

使用 `--safe-mode` 启动 Claude Code，或设置 `CLAUDE_CODE_SAFE_MODE`，以禁用所有自定义项启动：`CLAUDE.md`、skills、plugins、hooks、MCP 服务器以及自定义命令和代理不会加载。身份验证、模型选择、内置工具和权限仍然有效。如果问题在安全模式下消失，则其中一个表面是原因。

启动干净会话以隔离损坏的配置：

```bash terminal theme={null}
claude --safe-mode
```

[针对干净配置进行测试](https://code.claude.com/docs/zh-CN/debug-your-config.md#test-against-a-clean-configuration)

其他改进

[`fallbackModel`](https://code.claude.com/docs/zh-CN/model-config#fallback-model-chains) 配置最多三个备用模型，在主模型过载或不可用时按顺序尝试，`--fallback-model` 现在也适用于交互式会话会话标题现在以您的对话语言生成；使用 `language` 设置固定特定的标题`claude agents --json` 添加 `--all` 以包含已完成的会话以及新的 `id` 和 `state` 字段，不再省略被阻止或新分派的会话在 `/plugin` 中浏览市场的插件现在有搜索栏新的 `disableBundledSkills` 设置和 `CLAUDE_CODE_DISABLE_BUNDLED_SKILLS` 隐藏捆绑的 skills、工作流和内置命令不让模型看到拒绝规则在工具名称位置接受 glob，因此 `"*"` 拒绝所有工具，拒绝规则中的未知工具名称现在在启动时发出警告代理消息传递得到加强：通过 `SendMessage` 从其他代理中继的消息不再携带用户权限，自动模式会阻止它们Amazon Bedrock 在 `AWS_REGION` 未设置时从 `~/.aws` 配置文件读取 AWS 区域，`/status` 显示区域来自何处新的 `enforceAvailableModels` 托管设置使 `availableModels` 允许列表也约束默认模型Chrome 浏览器工具中的 Claude 现在在单个批处理调用中加载，而不是每个工具一个`claude update` 在下载前宣布目标版本，而不是保持沉默新的 `footerLinksRegexes` 设置将正则表达式匹配的链接徽章添加到页脚行

[v2.1.166–v2.1.176 的完整更新日志 →](https://code.claude.com/docs/en/changelog.md#2-1-166)
