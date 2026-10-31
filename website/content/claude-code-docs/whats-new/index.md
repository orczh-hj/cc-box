> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# 最新动态

> Claude Code 功能的每周摘要，包含代码片段、演示和背景信息，说明为什么这些功能很重要。

每周开发摘要突出了最有可能改变您工作方式的功能。每个条目都包括可运行的代码、简短的演示和完整文档的链接。有关每个错误修复和次要改进，请参阅[更新日志](https://code.claude.com/docs/en/changelog.md)。

**Week 37**

September 7–11, 2026

**`claude plugin eval`**：针对一套测试用例运行您的插件，对结果进行评分，并与无插件基线进行比较。`claude plugin eval init` 为您起草用例和评分器。

本周还有：将任何 **Claude Code Desktop 窗格**弹出到其自己的窗口中，稍后将其停靠回来；**`maxEffortLevel`** 设置限制每个提供商的努力级别；以及 **WebFetch** 在五分钟内未完成下载的页面会失败而不是挂起。

[阅读 Week 37 摘要 →](https://code.claude.com/docs/zh-CN/whats-new/2026-w37.md)

**Week 36**

August 31 – September 4, 2026

**Claude Fable 5.1**：在 Claude Code 中可用，具有 1M 令牌上下文窗口。

本周还有：在 Pro 和 Max 计划上，**Desktop 应用中的计算机使用**在 macOS 上在后台工作，同时您继续工作；在全屏渲染中，**`/diff`** 在对话旁边打开一个实时面板，在 Claude 编辑时刷新；**`/skill-doctor`** 显示您每个技能在上下文中的成本以及它被使用的频率。

[阅读 Week 36 摘要 →](https://code.claude.com/docs/zh-CN/whats-new/2026-w36.md)

**Week 35**

August 24–28, 2026

**在 Desktop 应用中恢复终端会话**：在 Claude Code Desktop 提示框中键入 `/resume` 以继续您从 CLI 启动的任何会话，保持完整的对话和上下文。

本周还有：**Claude 起草的反馈**在会话中出现问题时让 Claude 编写反馈报告，您审查并从 `/feedback` 发送；**`--restricted`** 启动没有命令运行工具或您的用户和项目设置的会话，用于共享机器上的评估工具；**`modelPicker`** 设置控制 `/model` 选择器列出的模型。

[阅读 Week 35 摘要 →](https://code.claude.com/docs/zh-CN/whats-new/2026-w35.md)

**Week 34**

August 17–21, 2026

**`/design`**：一个研究预览版，将 Claude Design 的画板工作流程引入 CLI 和 Claude Code Desktop，基于 artifacts 构建，因此 Claude 为您的 UI 草拟可编辑的画板并实现您选择的那个。

本周还有：内置的 **Concise output style** 使 Claude 以结果开头并跳过前言；任何运行 `claude remote-control` 的机器都会在您的手机上显示为 **device card**，以便您可以从 Code 选项卡在其上启动会话；**`ANTHROPIC_DEFAULT_MODEL`** 设置新会话启动的模型。

[阅读 Week 34 摘要 →](https://code.claude.com/docs/zh-CN/whats-new/2026-w34.md)

**Week 33**

August 10–14, 2026

**Desktop 上的使用限制后自动继续**：当您在 Claude Code Desktop 中达到会话限制时，在限制卡上检查 **Auto-continue when limits reset**，应用将在限制重置后重试中断的回合。

本周还有：**fork mode** 在交互式会话中默认启用，因此 Claude 可以将侧任务交给继承完整对话的子代理；**GitLab** 合并请求 URL 适用于 `--worktree` 和 `claude agents` 视图，市场克隆裸 `gitlab.com` URL；在提示中键入 **`@`** 按名称提及另一个 Claude 会话。

[阅读 Week 33 摘要 →](https://code.claude.com/docs/zh-CN/whats-new/2026-w33.md)

**Week 32**

August 3–7, 2026

**跨会话消息传递**：在 macOS 和 Linux 上，您的 Claude Code 会话现在可以相互发送消息，因此 Claude 可以将发现或决定从一个会话传递到另一个会话，而不是您重新解释它。

本周还有：**自托管环境**在您的组织运营的基础设施上运行 Claude Code 云会话，在 Team 和 Enterprise 计划上处于公开测试版；**auto mode** 从 8 月 14 日开始成为 Pro、Max 和 Team 计划上新会话的默认权限模式；**VS Code extension** 获得 Focus view。

[阅读 Week 32 摘要 →](https://code.claude.com/docs/zh-CN/whats-new/2026-w32.md)

**Week 30**

July 20–24, 2026

**Claude Opus 5**：Claude Code 中的新默认 Opus 模型，具有 1M 令牌上下文窗口和快速模式，每 MTok 费用为 \$10/\$50。

本周还有：**Claude Code Desktop** 在公开测试版中打开 iOS Simulator 窗格，以便 Claude 可以运行您的应用并在您观看时点击浏览它；**Claude Security plugin** 运行您的代码库的多代理漏洞扫描，并将您选择的发现转变为您自己应用的补丁；**`/code-review`** 作为后台子代理运行。

[阅读 Week 30 摘要 →](https://code.claude.com/docs/zh-CN/whats-new/2026-w30.md)

**Week 29**

July 13–17, 2026

**Artifacts 调用您的 MCP 连接器**：已发布的 artifact 可以在查看者打开页面时通过每个查看者自己的 MCP 连接器拉取实时数据并采取行动，本周还添加了公开共享链接、Team 和 Enterprise 上的编辑器角色以及从 Claude Tag 会话创建的 artifacts。

本周还有：**屏幕阅读器模式**用纯线性文本替换视觉终端界面，适用于 VoiceOver 和 NVDA 等屏幕阅读器；**`/fork`** 将您的对话复制到新的后台会话中，同时您继续工作；**auto mode** 在 Amazon Bedrock、Google Cloud 的 Agent Platform 和 Microsoft Foundry 上不再需要选择加入变量。

[阅读 Week 29 摘要 →](https://code.claude.com/docs/zh-CN/whats-new/2026-w29.md)

**Week 28**

July 6–10, 2026

**桌面应用内置浏览器**：Claude Code 桌面版获得了内置浏览器，因此 Claude 可以调出文档、设计或任何其他网站，并以与本地开发服务器预览相同的方式与页面交互。

本周还有：**`/doctor`** 是一个完整的设置检查，可以诊断问题并修复它们，`/checkup` 是其别名；**自动模式**阻止成绩单篡改，并在未解决的变量上在 `rm -rf` 之前询问；**代理视图行**显示彩色状态词和分类器编写的标题。

[阅读 Week 28 摘要 →](https://code.claude.com/docs/zh-CN/whats-new/2026-w28.md)

**Week 27**

June 29 – July 3, 2026

**Claude Sonnet 5**：Pro、Team Standard 和 Enterprise 订阅席位的新默认模型，具有顶级编码和工具使用能力，采用 Sonnet 定价，原生 1M 令牌上下文窗口，默认启用自适应思考。

本周还有：**Chrome 中的 Claude** 在所有直接 Anthropic 计划上正式推出；**子代理默认在后台运行**，以便 Claude 在它们运行时继续工作；**Claude Desktop on Linux** 在 Ubuntu 和 Debian 上进入测试版；**`/radio`** 调入 Claude FM lo-fi 电台。

[阅读 Week 27 摘要 →](https://code.claude.com/docs/zh-CN/whats-new/2026-w27.md)

**Week 26**

June 22–26, 2026

**`claude mcp login`**：从您的 shell 中验证配置的 MCP 服务器，而不是使用交互式 `/mcp` 菜单，稍后可以使用 `claude mcp logout` 清除其存储的凭证。

本周还有：**shell 模式响应命令输出**（`! npm test` 无需第二个提示即可获得解释）；**`/rewind`** 可以从运行 `/clear` 之前恢复对话；**后台子代理**现在在主会话中显示权限提示，而不是自动拒绝。

[阅读 Week 26 摘要 →](https://code.claude.com/docs/zh-CN/whats-new/2026-w26.md)

**Week 25**

June 15–19, 2026

**Artifacts**：将会话的输出转换为 claude.ai 上的实时、可共享页面，在会话工作时原地更新，现在在 Team 和 Enterprise 计划中处于测试版。

本周还有：**拒绝和询问规则与工具参数匹配**，使用 `Tool(param:value)`，例如 `Agent(model:opus)`；**`/config key=value`** 从提示、`-p` 模式和远程控制中设置任何设置；**自动模式阻止破坏性 git 命令**，当您没有要求丢弃本地工作时。

[阅读 Week 25 摘要 →](https://code.claude.com/docs/zh-CN/whats-new/2026-w25.md)

**Week 24**

June 8–12, 2026

**`/cd`**：在对话中途将当前会话移动到新的工作目录，无需重建提示缓存。

本周还有：**子代理可以生成自己的子代理**（后台链最多五层深）；**`--safe-mode`** 启动 Claude Code 时禁用所有自定义以进行故障排除；**`fallbackModel`** 配置最多三个按顺序尝试的备用模型。

[阅读 Week 24 摘要 →](https://code.claude.com/docs/zh-CN/whats-new/2026-w24.md)

**Week 23**

June 1–5, 2026

**Amazon Bedrock、Google Cloud 的 Agent Platform 和 Microsoft Foundry 上的 Auto mode**：auto mode 现在在第三方提供商上可用，支持 Opus 4.7 和 Opus 4.8，用后台安全检查替换权限提示。

本周还有：**更安全的自动编辑**在 `acceptEdits` 模式下写入可以运行代码的文件前提示；**`/plugin list`** 内联打印您安装的插件；**版本要求**让托管部署要求批准的 Claude Code 版本范围。

[阅读 Week 23 摘要 →](https://code.claude.com/docs/zh-CN/whats-new/2026-w23.md)

**Week 22**

May 25–29, 2026

**Claude Opus 4.8**：Max、Team Premium、Enterprise 按需付费和 Anthropic API 账户的新默认模型，默认情况下高努力级别，对于最困难的任务使用 `/effort xhigh`。

本周还有：**动态工作流**从 Claude 编写的脚本中编排数十到数百个子代理；**security-guidance plugin** 在 Claude 工作时审查其更改以查找漏洞；**快速模式**在 Opus 4.8 上运行，每 MTok 费用为 \$10/\$50。

[阅读 Week 22 摘要 →](https://code.claude.com/docs/zh-CN/whats-new/2026-w22.md)

**Week 21**

May 18–22, 2026

**Pro 计划上的 Auto mode**：auto mode 现在在 Pro 账户上运行，支持 Sonnet 4.6 和 Opus，用后台安全检查替换权限提示。

本周还有：**`/usage`** 按 skill、subagent、plugin 和 MCP server 分解驱动您计划限制的因素；新的 **`/code-review`** 命令报告正确性错误；**后台会话**出现在 `/resume` 中，固定时保持活跃。

[阅读 Week 21 摘要 →](https://code.claude.com/docs/zh-CN/whats-new/2026-w21.md)

**Week 20**

May 11–15, 2026

**Agent view**：`claude agents` 为每个 Claude Code 会话打开一个屏幕，显示正在运行的内容、被您阻止的内容以及已完成的内容。

本周还有：**`/goal`** 让 Claude 在多个回合中继续工作，直到满足完成条件；**快速模式**现在默认在 Opus 4.7 上运行；**Rewind 菜单**可以使用"Summarize up to here"压缩早期上下文。

[阅读 Week 20 摘要 →](https://code.claude.com/docs/zh-CN/whats-new/2026-w20.md)

**Week 19**

May 4–8, 2026

**插件从 `.zip` 存档和 URL 加载**：`--plugin-dir` 现在接受 `.zip` 文件，`--plugin-url` 为当前会话获取插件存档。

本周还有：**`worktree.baseRef`** 选择新的 worktrees 是从远程默认分支还是本地 `HEAD` 分支；**auto mode 硬拒绝规则**无条件阻止操作，无论允许例外如何；**hooks 看到活跃的努力级别**通过 `effort.level` 和 `$CLAUDE_EFFORT`。

[阅读 Week 19 摘要 →](https://code.claude.com/docs/zh-CN/whats-new/2026-w19.md)

**Week 18**

April 27 – May 1, 2026

**没有 Git Bash 的 Windows**：不再需要 Git for Windows，当 Bash 不存在时，Claude Code 使用 PowerShell 作为 shell 工具。

本周还有：**`claude ultrareview`** 将云代码审查带到 CI 和脚本；**`claude project purge`** 清理项目的本地状态；将 **PR URL 粘贴到 `/resume`** 中找到创建它的会话。

[阅读 Week 18 摘要 →](https://code.claude.com/docs/zh-CN/whats-new/2026-w18.md)

**Week 17**

April 20–24, 2026

**`/ultrareview`** 作为公开研究预览版开放：一队错误搜寻代理在云中运行，发现结果会自动返回到您的 CLI 或桌面应用。

本周还有：**会话回顾**显示终端失焦时发生的情况；**自定义主题**让您可以从 `/theme` 或插件构建和发布调色板；**Claude Code 网页版**进行了重新设计，包括新的会话侧边栏和拖放布局。

[阅读 Week 17 摘要 →](https://code.claude.com/docs/zh-CN/whats-new/2026-w17.md)

**Week 16**

April 13–17, 2026

**Claude Opus 4.7** 成为 Max 和 Team Premium 的新默认版本，具有新的 `xhigh` 努力级别（推荐用于大多数编码工作）和交互式 `/effort` 滑块来调整它。

本周还有：**Routines** 在 Claude Code 网页版上从计划、GitHub 事件或 API 调用触发模板化云代理；**移动推送通知**在长任务完成或 Claude 需要您时向您的手机发送通知；`/usage` 显示驱动您限制的因素；CLI 迁移到本机二进制文件。

[阅读 Week 16 摘要 →](https://code.claude.com/docs/zh-CN/whats-new/2026-w16.md)

**Week 15**

April 6–10, 2026

**Ultraplan** 进入早期预览：从您的 CLI 在云中草拟计划，在网页编辑器中审查和评论，然后远程运行或拉回本地。第一次运行现在会自动为您创建云环境。

本周还有：**Monitor** 工具将后台事件流式传输到对话中，以便 Claude 可以跟踪日志并实时响应，`/loop` 在您省略间隔时自动调整速度，`/team-onboarding` 将您的设置打包成可重放的指南，`/autofix-pr` 从您的终端打开 PR 自动修复。

[阅读 Week 15 摘要 →](https://code.claude.com/docs/zh-CN/whats-new/2026-w15.md)

**Week 14**

March 30 – April 3, 2026

**计算机使用**在研究预览版中来到 CLI：Claude 可以打开本机应用、点击 UI 并从您的终端验证更改。最适合关闭只有 GUI 才能验证的事情。

本周还有：`/powerup` 交互式课程、无闪烁的替代屏幕渲染、每个工具的 MCP 结果大小覆盖（最高 500K）和 Bash 工具 `PATH` 上的插件可执行文件。

[阅读 Week 14 摘要 →](https://code.claude.com/docs/zh-CN/whats-new/2026-w14.md)

**Week 13**

March 23–27, 2026

**Auto mode** 在研究预览版中推出：分类器处理您的权限提示，以便安全操作无中断运行，风险操作被阻止。这是批准所有内容和 `--dangerously-skip-permissions` 之间的中间地带。

本周还有：桌面应用中的计算机使用、Web 上的 PR 自动修复、使用 `/` 的成绩单搜索、适用于 Windows 的本机 PowerShell 工具和条件 `if` hooks。

[阅读 Week 13 摘要 →](https://code.claude.com/docs/zh-CN/whats-new/2026-w13.md)
