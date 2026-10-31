> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# 第29周 · 2026年7月13–17日

> 通过MCP连接器将实时数据拉入已发布的工件中，并在新的屏幕阅读器模式下使用Claude Code。

发布版本 [v2.1.207 → v2.1.212](https://code.claude.com/docs/en/changelog.md#2-1-207)
2项功能 · 7月13–17

工件调用您的MCP连接器
web

已发布的工件现在可以在每次有人查看时调用MCP连接器，因此仪表板显示实时数据并可以按需执行操作，而不是构建它的会话中的快照。每次调用都通过查看者自己的连接运行，查看者在页面首次连接器调用前批准访问。本周还添加了公开共享链接、Team和Enterprise计划上的编辑者角色，以及从Claude Tag会话创建的工件。

<video autoPlay muted loop playsInline src="https://mintcdn.com/claude-code/ItzF3QVI6L0QypjJ/images/whats-new/artifacts-mcp.mp4?fit=max&auto=format&n=ItzF3QVI6L0QypjJ&q=85&s=ff8b81ed52b26c773899dc28cec959e6" />

在您的提示中命名连接器和您想要的数据：

```text title="Claude Code" wrap theme={null}
Build a dashboard artifact of open pull requests that pulls the live list through my GitHub connector when the page loads.
```

[使用MCP连接器拉取实时数据](https://code.claude.com/docs/zh-CN/artifacts.md#pull-live-data-with-mcp-connectors)

屏幕阅读器模式
CLI

屏幕阅读器模式用纯文本、线性文本替换可视化终端界面：不使用框、旋转器和原地重绘，Claude Code打印标记的行，屏幕阅读器（如VoiceOver或NVDA）按顺序读取，因此您可以批准权限并端到端审查输出。使用标志按会话打开它，使用`CLAUDE_AX_SCREEN_READER`环境变量按shell打开它，或使用`axScreenReader`设置在任何地方打开它。

在屏幕阅读器模式下启动会话：

```bash terminal theme={null}
claude --ax-screen-reader
```

[打开屏幕阅读器模式](https://code.claude.com/docs/zh-CN/accessibility.md#turn-on-screen-reader-mode)

其他改进

`/fork`现在将您的对话复制到新的后台会话中，在`claude agents`中有自己的行，同时您继续工作；它曾经启动的会话内分叉子代理现在是`/subtask`[自动模式](https://code.claude.com/docs/zh-CN/permission-modes.md#enable-auto-mode-on-bedrock-agent-platform-or-foundry)在Amazon Bedrock、Google Cloud的Agent Platform和Microsoft Foundry上不再需要`CLAUDE_CODE_ENABLE_AUTO_MODE`选择加入；管理员可以使用`disableAutoMode`关闭它运行时间超过两分钟的MCP工具调用现在自动移到后台，以便会话保持可用；使用`CLAUDE_CODE_MCP_AUTO_BACKGROUND_MS`调整或禁用阈值新的`claude auto-mode reset`恢复默认自动模式配置，`--yes`跳过确认提示新的[企业启动器](https://code.claude.com/docs/zh-CN/corporate-launcher.md)支持：`CLAUDE_CODE_PROCESS_WRAPPER`或`processWrapper`设置通过必需的包装器可执行文件运行Claude Code从其自己的二进制文件启动的进程，例如后台服务和代理视图会话`vimInsertModeRemaps`设置将两键插入模式序列（如`jj`）映射到vim模式中的Escape`--forward-subagent-text`和`CLAUDE_CODE_FORWARD_SUBAGENT_TEXT`在[stream-json输出](https://code.claude.com/docs/zh-CN/headless.md)中包含子代理文本和思考块会话范围的上限停止失控循环：WebSearch调用和子代理生成各默认为200，可使用`CLAUDE_CODE_MAX_WEB_SEARCHES_PER_SESSION`和`CLAUDE_CODE_MAX_SUBAGENTS_PER_SESSION`调整"始终允许"权限规则保存在存储库根目录，因此在git worktree中授予的批准在会话和worktree中持续Amazon Bedrock、Google Cloud的Agent Platform和AWS上的Claude Platform现在默认为Claude Opus 4.8折叠的工具摘要行显示实时经过时间计数器，因此长时间运行的工具调用可见地计时，而不是看起来卡住

[v2.1.207–v2.1.212的完整更新日志 →](https://code.claude.com/docs/en/changelog.md#2-1-207)
