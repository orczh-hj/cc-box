> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# 所有设置

> Claude Code 每个 settings.json 键的完整参考：每个键的放置位置、类型和默认值，以及可直接粘贴的示例，并附有所有键的索引。

[返回索引](#all-settings)

本参考页面列出了 Claude Code 从设置文件中读取的每个键，以及它改为保存在 `~/.claude.json` 中的[一小组键](#global-config-settings)。如需选择文件或查看优先级，请从[设置文件和优先级](https://code.claude.com/docs/zh-CN/settings.md)开始。

## 设置索引

下面的每个键都链接到其对应条目。作用域列出了该键可以写入的[文件](https://code.claude.com/docs/zh-CN/settings.md#settings-files-and-who-they-affect)：`用户` 指 `~/.claude/settings.json`，`项目` 指 `.claude/settings.json`，`本地` 指 `.claude/settings.local.json`，`托管` 指[您的组织部署的内容](https://code.claude.com/docs/zh-CN/managed-settings.md)。`任意文件` 表示以上全部四种，`全局配置` 表示 [`~/.claude.json`](#global-config-settings)。

| Key | Description | Topic | Scope |
| :- | :- | :- | :- |
| [`advisorModel`](#advisormodel) | 选择 Claude 调用 [advisor 工具](https://code.claude.com/docs/zh-CN/advisor.md)时由哪个模型回答 | 模型与回复 | 任意文件 |
| [`agent`](#agent) | 让每个会话都以指定的[子代理](https://code.claude.com/docs/zh-CN/sub-agents.md)身份启动，使用其提示词、工具和模型 | Agent、会话与 worktree | 任意文件 |
| [`agentPushNotifEnabled`](#agentpushnotifenabled) | 允许 Claude 在其认为需要时[向您的手机发送推送通知](https://code.claude.com/docs/zh-CN/remote-control.md#mobile-push-notifications) | 远程、桌面与通知 | 任意文件 |
| [`allowAllClaudeAiMcps`](#allowallclaudeaimcps) | 在已部署 [`managed-mcp.json`](https://code.claude.com/docs/zh-CN/managed-mcp#exclusive-control-with-managed-mcp-json) 的同时，加载 Claude Code 自行获取的 [claude.ai 连接器](https://code.claude.com/docs/zh-CN/mcp.md) | MCP | 托管 |
| [`allowClaudeInChromeWithManagedMcp`](#allowclaudeinchromewithmanagedmcp) | 允许内置的 [Claude in Chrome](https://code.claude.com/docs/zh-CN/chrome.md) 服务器与已部署的 [`managed-mcp.json`](https://code.claude.com/docs/zh-CN/managed-mcp#exclusive-control-with-managed-mcp-json) 一起运行 | MCP | 托管 |
| [`allowedChannelPlugins`](#allowedchannelplugins) | 替换可推送消息的[频道插件](https://code.claude.com/docs/zh-CN/channels.md#restrict-which-channel-plugins-can-run)的默认允许列表 | 插件与 skill | 托管 |
| [`allowedHttpHookUrls`](#allowedhttphookurls) | 限制 [HTTP hook](https://code.claude.com/docs/zh-CN/hooks.md) 可以访问的 URL | hook 与自动化 | 任意文件 |
| [`allowedMcpServers`](#allowedmcpservers) | 通过允许列表限定用户可以添加的 [MCP 服务器](https://code.claude.com/docs/zh-CN/mcp.md) | MCP | 任意文件 |
| [`allowedProviders`](#allowedproviders) | 限制一台机器可以使用的 [API 提供商](https://code.claude.com/docs/zh-CN/third-party-integrations.md) | 身份验证与提供商 | 托管 |
| [`allowManagedHooksOnly`](#allowmanagedhooksonly) | 仅运行您的组织部署的 [hook](https://code.claude.com/docs/zh-CN/hooks.md) | hook 与自动化 | 托管 |
| [`allowManagedMcpServersOnly`](#allowmanagedmcpserversonly) | 使托管的 [MCP](https://code.claude.com/docs/zh-CN/mcp.md) 允许列表成为唯一生效的允许列表 | MCP | 托管 |
| [`allowManagedPermissionRulesOnly`](#allowmanagedpermissionrulesonly) | 使[托管设置](https://code.claude.com/docs/zh-CN/managed-settings.md)成为[权限规则](https://code.claude.com/docs/zh-CN/permissions.md#managed-settings)的唯一设置来源 | 权限设置 | 托管 |
| [`alwaysThinkingEnabled`](#alwaysthinkingenabled) | 为所有会话关闭[扩展思考](https://code.claude.com/docs/zh-CN/model-config.md#extended-thinking) | 模型与回复 | 任意文件 |
| [`apiKeyHelper`](#apikeyhelper) | 使用您自己的命令生成 [API 凭据](https://code.claude.com/docs/zh-CN/authentication.md#credential-management) | 身份验证与提供商 | 任意文件 |
| [`askUserQuestionTimeout`](#askuserquestiontimeout) | 让未回答的问题在空闲一段时间后[自动继续](https://code.claude.com/docs/zh-CN/tools-reference.md#question-auto-continue-timeout) | 界面与终端 | 用户或托管 |
| [`appendPlugins`](#appendplugins) | 在用户安装的每个 mod 之后运行您组织的 [mod](https://code.claude.com/docs/zh-CN/plugins/mods/admin.md) | 插件与 skill | 用户或托管 |
| [`attribution`](#attribution) | 自定义 Claude Code 添加到提交和 Pull Request 中的署名 | Git 与署名 | 任意文件 |
| [`attribution.commit`](#attribution-commit) | 更改或隐藏 Claude Code 添加到提交中的尾注 | Git 与署名 | 任意文件 |
| [`attribution.pr`](#attribution-pr) | 更改或隐藏 Pull Request 描述中的署名行 | Git 与署名 | 任意文件 |
| [`attribution.sessionUrl`](#attribution-sessionurl) | 在[云端](https://code.claude.com/docs/zh-CN/claude-code-on-the-web.md)和 [Remote Control](https://code.claude.com/docs/zh-CN/remote-control.md) 提交中省略 claude.ai 会话链接 | Git 与署名 | 任意文件 |
| [`autoCompactEnabled`](#autocompactenabled) | 关闭或开启[自动压缩](https://code.claude.com/docs/zh-CN/context-window.md) | 记忆与上下文 | 任意文件 |
| [`autoCompactWindow`](#autocompactwindow) | 设置上下文达到多满时 Claude Code 执行[压缩](https://code.claude.com/docs/zh-CN/context-window.md) | 记忆与上下文 | 任意文件 |
| [`autoConnectIde`](#autoconnectide) | 从外部终端自动连接到正在运行的 [VS Code](https://code.claude.com/docs/zh-CN/vs-code.md) 或 [JetBrains](https://code.claude.com/docs/zh-CN/jetbrains.md#from-external-terminals) IDE | 全局配置设置 | 全局配置 |
| [`autoContinueAtUsageLimit`](#autocontinueatusagelimit) | 在 claude.ai 用量限制重置后，在已打开的会话中等待并[自动继续任务](https://code.claude.com/docs/zh-CN/interactive-mode.md#wait-for-a-usage-limit-to-reset) | 界面与终端 | 用户或托管 |
| [`autoInstallIdeExtension`](#autoinstallideextension) | 关闭从 VS Code 终端自动安装 [IDE 扩展](https://code.claude.com/docs/zh-CN/vs-code.md#install-the-extension) | 全局配置设置 | 全局配置 |
| [`autoMemoryDirectory`](#automemorydirectory) | 将[自动记忆](https://code.claude.com/docs/zh-CN/memory.md#auto-memory)存储在您选择的目录中 | 记忆与上下文 | 任意文件 |
| [`autoMemoryEnabled`](#automemoryenabled) | 关闭或开启[自动记忆](https://code.claude.com/docs/zh-CN/memory.md#auto-memory) | 记忆与上下文 | 任意文件 |
| [`autoMode`](#automode) | 为[自动模式](https://code.claude.com/docs/zh-CN/permission-modes.md#eliminate-prompts-with-auto-mode)分类器添加您自己的允许和拒绝规则 | 权限设置 | 用户或托管 |
| [`autoMode.classifyAllShell`](#automode-classifyallshell) | 将每条 shell 命令都交给[自动模式分类器](https://code.claude.com/docs/zh-CN/permission-modes.md#what-the-classifier-blocks-by-default)审查，即使是匹配了窄范围允许规则的命令 | 权限设置 | 用户或托管 |
| [`autoScrollEnabled`](#autoscrollenabled) | 在全屏渲染中[跟随新输出](https://code.claude.com/docs/zh-CN/fullscreen.md#auto-follow)滚动到底部 | 界面与终端 | 任意文件 |
| [`autoUpdatesChannel`](#autoupdateschannel) | 跟随 stable [发布渠道](https://code.claude.com/docs/zh-CN/setup.md#configure-release-channel)而不是 latest | 更新与版本 | 任意文件 |
| [`availableModels`](#availablemodels) | [限制用户可以选择的模型](https://code.claude.com/docs/zh-CN/model-config.md#restrict-model-selection) | 模型与回复 | 任意文件 |
| [`availableModelsMatch`](#availablemodelsmatch) | 使 `availableModels` 中的每个模型 ID 条目[仅允许其指定的版本](https://code.claude.com/docs/zh-CN/model-config.md#block-specific-models-or-versions) | 模型与回复 | 托管 |
| [`awaySummaryEnabled`](#awaysummaryenabled) | 关闭您返回终端时显示的[会话回顾](https://code.claude.com/docs/zh-CN/interactive-mode.md#session-recap) | 远程、桌面与通知 | 任意文件 |
| [`awsAuthRefresh`](#awsauthrefresh) | 使用您自己的命令刷新 `.aws` 中已过期的 [Bedrock 凭据](https://code.claude.com/docs/zh-CN/amazon-bedrock.md#advanced-credential-configuration) | 身份验证与提供商 | 任意文件 |
| [`awsCredentialExport`](#awscredentialexport) | 通过您自己的命令以 JSON 形式提供 [Bedrock 凭据](https://code.claude.com/docs/zh-CN/amazon-bedrock.md#advanced-credential-configuration) | 身份验证与提供商 | 任意文件 |
| [`axScreenReader`](#axscreenreader) | 渲染[适合屏幕阅读器的输出](https://code.claude.com/docs/zh-CN/accessibility.md) | 界面与终端 | 用户或托管 |
| [`bashEditDiffEnabled`](#basheditdiffenabled) | 在所有权限模式下记录[在 Bash 命令运行期间发生更改的文件](https://code.claude.com/docs/zh-CN/hooks.md#bash) | 界面与终端 | 用户或托管 |
| [`bashOutputMaxChars`](#bashoutputmaxchars) | 设置 Claude 以内联方式接收成功命令[输出](https://code.claude.com/docs/zh-CN/tools-reference.md#output-limits)的长度 | 记忆与上下文 | 任意文件 |
| [`blockedMarketplaces`](#blockedmarketplaces) | 为您的组织屏蔽[插件市场](https://code.claude.com/docs/zh-CN/plugins/overview.md)来源 | 插件与 skill | 托管 |
| [`browserExternalPageTools`](#browserexternalpagetools) | 在[桌面版](https://code.claude.com/docs/zh-CN/desktop.md) Browser 面板中禁止 Claude 的工具作用于外部页面 | 工具 | 托管 |
| [`channelsEnabled`](#channelsenabled) | 为您的组织允许[频道](https://code.claude.com/docs/zh-CN/channels.md#enable-channels-for-your-organization) | 插件与 skill | 托管 |
| [`claudeInChromeDefaultEnabled`](#claudeinchromedefaultenabled) | 在交互式 CLI 和 VS Code 扩展中，会话启动时开启 [Chrome 集成](https://code.claude.com/docs/zh-CN/chrome.md) | 全局配置设置 | 全局配置 |
| [`claudeMd`](#claudemd) | 通过托管设置注入组织范围的 [CLAUDE.md](https://code.claude.com/docs/zh-CN/memory.md#deploy-organization-wide-claude-md) 指令 | 记忆与上下文 | 托管 |
| [`claudeMdExcludes`](#claudemdexcludes) | 在加载记忆时跳过特定的 [CLAUDE.md](https://code.claude.com/docs/zh-CN/memory.md#exclude-specific-claude-md-files) 文件 | 记忆与上下文 | 任意文件 |
| [`cleanupPeriodDays`](#cleanupperioddays) | 选择 Claude Code 在删除[会话记录](https://code.claude.com/docs/zh-CN/data-usage.md#data-retention)之前保留多少天 | 隐私与遥测 | 任意文件 |
| [`companyAnnouncements`](#companyannouncements) | 在启动时显示您组织的公告 | 界面与终端 | 任意文件 |
| [`copyFullResponse`](#copyfullresponse) | 让 [`/copy`](https://code.claude.com/docs/zh-CN/commands) 直接复制完整回复而不显示选择器 | 全局配置设置 | 全局配置 |
| [`copyOnSelect`](#copyonselect) | 关闭在[全屏渲染](https://code.claude.com/docs/zh-CN/fullscreen.md#use-the-mouse)和 Agent 视图中自动复制鼠标所选文本的功能 | 全局配置设置 | 全局配置 |
| [`crossSessionInbound`](#crosssessioninbound) | 选择 Claude Code 是投递[来自您其他会话的消息](https://code.claude.com/docs/zh-CN/cross-session-messaging.md#control-inbound-messages)、仅显示通知而不投递，还是拒绝这些消息 | Agent、会话与 worktree | 任意文件 |
| [`defaultShell`](#defaultshell) | 选择由 Bash 还是 PowerShell 运行您以 [`!` 前缀](https://code.claude.com/docs/zh-CN/interactive-mode#shell-mode-with-prefix)输入的 shell 命令 | 界面与终端 | 任意文件 |
| [`defaultToAgentsView`](#defaulttoagentsview) | 在不带参数运行 `claude` 时打开 [Agent 视图](https://code.claude.com/docs/zh-CN/agent-view.md)而不是新对话 | 全局配置设置 | 全局配置 |
| [`deniedMcpServers`](#deniedmcpservers) | 按 URL、命令或名称屏蔽特定的 [MCP 服务器](https://code.claude.com/docs/zh-CN/mcp.md) | MCP | 任意文件 |
| [`deniedModels`](#deniedmodels) | [屏蔽特定模型](https://code.claude.com/docs/zh-CN/model-config.md#block-specific-models-or-versions)，即使 `availableModels` 允许了这些模型 | 模型与回复 | 托管 |
| [`desktopSessionCleanupPeriodDays`](#desktopsessioncleanupperioddays) | 为 [Claude Desktop 和 Cowork 会话记录](https://code.claude.com/docs/zh-CN/claude-directory.md#cleaned-up-automatically)设置以天为单位的保留期限 | 隐私与遥测 | 用户或托管 |
| [`dialogExpiry`](#dialogexpiry) | 设置 Claude Code 在取消转发的对话框之前，等待 [Remote Control](https://code.claude.com/docs/zh-CN/remote-control.md) 或 SDK 宿主应答的时长 | 界面与终端 | 用户或托管 |
| [`diffTool`](#difftool) | 选择 Claude 提议的文件更改是在 [VS Code](https://code.claude.com/docs/zh-CN/vs-code.md) 或 [JetBrains](https://code.claude.com/docs/zh-CN/jetbrains.md#features) 的 diff 查看器中打开，还是保留在终端中 | 全局配置设置 | 全局配置 |
| [`disableAgentView`](#disableagentview) | 关闭后台 Agent 和 [Agent 视图](https://code.claude.com/docs/zh-CN/agent-view.md) | Agent、会话与 worktree | 任意文件 |
| [`disableAllHooks`](#disableallhooks) | 一次性关闭 [hook](https://code.claude.com/docs/zh-CN/hooks.md)、自定义[状态栏](https://code.claude.com/docs/zh-CN/statusline.md)和自定义 [`@` 文件建议](https://code.claude.com/docs/zh-CN/interactive-mode#quick-commands)命令 | hook 与自动化 | 任意文件 |
| [`disableArtifact`](#disableartifact) | 已弃用；请使用 `enableArtifact` 关闭 [Artifact 工具](https://code.claude.com/docs/zh-CN/artifacts.md) | 远程、桌面与通知 | 任意文件 |
| [`disableAutoMode`](#disableautomode) | 从权限模式循环中移除[自动模式](https://code.claude.com/docs/zh-CN/permission-modes.md#eliminate-prompts-with-auto-mode) | 权限设置 | 任意文件 |
| [`disableBrowserExternalNavigation`](#disablebrowserexternalnavigation) | 将[桌面版](https://code.claude.com/docs/zh-CN/desktop.md) Browser 面板限制为仅供用户和 Claude 访问 localhost | 工具 | 托管 |
| [`disableBundledSkills`](#disablebundledskills) | 关闭 Claude Code 随附的 [skill](https://code.claude.com/docs/zh-CN/skills.md#bundled-skills) 和[工作流](https://code.claude.com/docs/zh-CN/workflows.md) | 插件与 skill | 任意文件 |
| [`disableClaudeAiConnectors`](#disableclaudeaiconnectors) | 关闭 [claude.ai 连接器](https://code.claude.com/docs/zh-CN/mcp.md#disable-claude-ai-connectors)，使 Claude Code 不再获取它们 | MCP | 任意文件 |
| [`disableCommandPluginSources`](#disablecommandpluginsources) | 屏蔽通过运行市场声明的命令进行安装的[插件](https://code.claude.com/docs/zh-CN/plugins/overview.md) | 插件与 skill | 托管 |
| [`disableDeepLinkRegistration`](#disabledeeplinkregistration) | 阻止 Claude Code 注册 [`claude-cli://` 处理程序](https://code.claude.com/docs/zh-CN/deep-links) | 远程、桌面与通知 | 任意文件 |
| [`disableDesktopLocalSessions`](#disabledesktoplocalsessions) | 关闭在设备本机上运行的 [Desktop Code 会话](https://code.claude.com/docs/zh-CN/desktop.md#local-sessions-on-managed-devices)，仅保留到其他主机的 SSH 和云端会话 | 远程、桌面与通知 | 托管 |
| [`disabledMcpjsonServers`](#disabledmcpjsonservers) | 拒绝项目 [`.mcp.json`](https://code.claude.com/docs/zh-CN/mcp#project-scope) 中的特定服务器 | MCP | 任意文件 |
| [`disableMobileSimulatorTools`](#disablemobilesimulatortools) | 在[桌面版](https://code.claude.com/docs/zh-CN/desktop.md) iOS Simulator 面板中屏蔽 Claude 的工具 | 工具 | 托管 |
| [`disableRemoteControl`](#disableremotecontrol) | 在所有可启动 [Remote Control](https://code.claude.com/docs/zh-CN/remote-control.md) 的位置将其关闭 | 远程、桌面与通知 | 任意文件 |
| [`disableSideloadFlags`](#disablesideloadflags) | 拒绝用于旁加载[插件](https://code.claude.com/docs/zh-CN/plugins/overview.md)、[子代理](https://code.claude.com/docs/zh-CN/sub-agents.md)和 [MCP 服务器](https://code.claude.com/docs/zh-CN/mcp.md)的 CLI 标志 | 企业与托管设置 | 托管 |
| [`disableSkillShellExecution`](#disableskillshellexecution) | 阻止 [skill](https://code.claude.com/docs/zh-CN/skills.md) 和自定义命令运行内联 shell | 插件与 skill | 任意文件 |
| [`disableWorkflows`](#disableworkflows) | 为所有人关闭[动态工作流](https://code.claude.com/docs/zh-CN/workflows.md)；若仅针对自己，请使用 `enableWorkflows` | hook 与自动化 | 任意文件 |
| [`editorMode`](#editormode) | 在输入框中使用 [vim 键绑定](https://code.claude.com/docs/zh-CN/interactive-mode.md#vim-editor-mode) | 界面与终端 | 任意文件 |
| [`effortLevel`](#effortlevel) | 为没有已保存级别的模型设置默认的 [effort 级别](https://code.claude.com/docs/zh-CN/model-config.md#adjust-effort-level) | 模型与回复 | 任意文件 |
| [`emojiCompletionEnabled`](#emojicompletionenabled) | 关闭输入框中的 [`:shortcode:` emoji 建议和替换](https://code.claude.com/docs/zh-CN/interactive-mode#emoji-shortcodes) | 界面与终端 | 任意文件 |
| [`enableAllProjectMcpServers`](#enableallprojectmcpservers) | 无需确认提示即批准项目 [`.mcp.json`](https://code.claude.com/docs/zh-CN/mcp#project-server-approvals-and-workspace-trust) 文件中的所有服务器 | MCP | 任意文件 |
| [`enableArtifact`](#enableartifact) | 在任意文件中设为 `false` 即可关闭 [Artifact 工具](https://code.claude.com/docs/zh-CN/artifacts.md)；任何文件都无法再将其重新开启 | 远程、桌面与通知 | 任意文件 |
| [`enabledMcpjsonServers`](#enabledmcpjsonservers) | 批准项目 [`.mcp.json`](https://code.claude.com/docs/zh-CN/mcp#project-server-approvals-and-workspace-trust) 中的特定服务器 | MCP | 任意文件 |
| [`enabledPlugins`](#enabledplugins) | 按作用域开启或关闭单个[插件](https://code.claude.com/docs/zh-CN/plugins/overview.md) | 插件与 skill | 任意文件 |
| [`enableWorkflows`](#enableworkflows) | 相对于您套餐的默认值开启或关闭[动态工作流](https://code.claude.com/docs/zh-CN/workflows.md) | hook 与自动化 | 任意文件 |
| [`enforceAvailableModels`](#enforceavailablemodels) | 将 [`/model` 的 Default 选项](https://code.claude.com/docs/zh-CN/model-config#enforce-the-allowlist-for-the-default-model)限制在您的 `availableModels` 允许列表之内 | 模型与回复 | 任意文件 |
| [`env`](#env) | 为每个会话及其子进程设置[环境变量](https://code.claude.com/docs/zh-CN/env-vars.md#in-settings-files) | 记忆与上下文 | 任意文件 |
| [`externalEditorContext`](#externaleditorcontext) | 按 [Ctrl+G](https://code.claude.com/docs/zh-CN/interactive-mode.md#general-controls) 进行编辑时，将 Claude 的上一条回复以注释形式显示 | 全局配置设置 | 全局配置 |
| [`extraKnownMarketplaces`](#extraknownmarketplaces) | 为仓库或组织注册[市场](https://code.claude.com/docs/zh-CN/plugins/overview.md) | 插件与 skill | 任意文件 |
| [`fallbackModel`](#fallbackmodel) | 指定主模型过载时使用的[备用模型](https://code.claude.com/docs/zh-CN/model-config.md#fallback-model-chains) | 模型与回复 | 任意文件 |
| [`fastMode`](#fastmode) | 在可用的会话中开启[快速模式](https://code.claude.com/docs/zh-CN/fast-mode.md) | 模型与回复 | 任意文件 |
| [`fastModePerSessionOptIn`](#fastmodepersessionoptin) | 要求用户在每个会话中手动开启[快速模式](https://code.claude.com/docs/zh-CN/fast-mode.md) | 模型与回复 | 任意文件 |
| [`feedbackDrafts`](#feedbackdrafts) | 控制 Claude 是否将[反馈草稿](https://code.claude.com/docs/zh-CN/tools-reference.md#sendfeedback-tool-behavior)排队供您审阅 | 隐私与遥测 | 用户或托管 |
| [`feedbackSurveyRate`](#feedbacksurveyrate) | 更改[会话质量调查](https://code.claude.com/docs/zh-CN/data-usage.md#session-quality-surveys)的出现频率 | 隐私与遥测 | 任意文件 |
| [`fileCheckpointingEnabled`](#filecheckpointingenabled) | 关闭或开启 [`/rewind`](https://code.claude.com/docs/zh-CN/checkpointing) 所恢复的文件快照 | 记忆与上下文 | 任意文件 |
| [`fileSuggestion`](#filesuggestion) | 通过您自己的命令提供 [`@` 文件自动补全](https://code.claude.com/docs/zh-CN/interactive-mode#quick-commands) | 界面与终端 | 任意文件 |
| [`footerLinksRegexes`](#footerlinksregexes) | 将输出中的 issue 或审查 ID 转换为输入框下方的[可点击链接](https://code.claude.com/docs/zh-CN/statusline.md#clickable-links) | 界面与终端 | 用户或托管 |
| [`forceLoginGatewayUrl`](#forcelogingatewayurl) | 设置登录界面连接的[网关 URL](https://code.claude.com/docs/zh-CN/claude-apps-gateway#set-the-gateway-url) | 身份验证与提供商 | 用户或托管 |
| [`forceLoginMethod`](#forceloginmethod) | 将[登录限制](https://code.claude.com/docs/zh-CN/authentication.md#restrict-login-to-your-organization)为 claude.ai、Claude Console 或[云网关](https://code.claude.com/docs/zh-CN/claude-apps-gateway) | 身份验证与提供商 | 任意文件 |
| [`forceLoginOrgUUID`](#forceloginorguuid) | [将 claude.ai 登录限定到您的组织](https://code.claude.com/docs/zh-CN/authentication.md#restrict-login-to-your-organization)；仅托管来源会强制执行此设置 | 身份验证与提供商 | 任意文件 |
| [`forceRemoteSettingsRefresh`](#forceremotesettingsrefresh) | 阻塞启动，直到成功获取最新的[服务器托管设置](https://code.claude.com/docs/zh-CN/server-managed-settings.md)为止 | 企业与托管设置 | 托管 |
| [`gatewayInternalNetworks`](#gatewayinternalnetworks) | 允许 `/login` 访问位于您组织内部使用的公共 IPv4 地址空间上的[云网关](https://code.claude.com/docs/zh-CN/claude-apps-gateway#allow-a-gateway-on-public-address-space-you-own) | 身份验证与提供商 | 托管 |
| [`gcpAuthRefresh`](#gcpauthrefresh) | 使用您自己的命令刷新 [Google Cloud 凭据](https://code.claude.com/docs/zh-CN/google-vertex-ai.md#advanced-credential-configuration) | 身份验证与提供商 | 任意文件 |
| [`hooks`](#hooks) | 在 Claude Code 生命周期的各个节点以 [hook](https://code.claude.com/docs/zh-CN/hooks.md) 形式运行您自己的命令 | hook 与自动化 | 任意文件 |
| [`httpHookAllowedEnvVars`](#httphookallowedenvvars) | 限制 [HTTP hook](https://code.claude.com/docs/zh-CN/hooks.md) 可以放入请求头的环境变量 | hook 与自动化 | 任意文件 |
| [`idleCompaction`](#idlecompaction) | 阻止 Claude Code 压缩您已离开的较长对话 | 记忆与上下文 | 任意文件 |
| [`includeCoAuthoredBy`](#includecoauthoredby) | 已弃用；请使用 `attribution` 隐藏或更改提交和 PR 署名 | Git 与署名 | 任意文件 |
| [`includeGitInstructions`](#includegitinstructions) | 从 Claude 的上下文中移除内置的提交和 PR 指令 | Git 与署名 | 任意文件 |
| [`inputNeededNotifEnabled`](#inputneedednotifenabled) | 在 Claude 等待您操作时收到[推送通知](https://code.claude.com/docs/zh-CN/remote-control.md#mobile-push-notifications) | 远程、桌面与通知 | 任意文件 |
| [`isolatePeerMachines`](#isolatepeermachines) | 在 Claude [向您在另一台机器上的会话发送消息](https://code.claude.com/docs/zh-CN/cross-session-messaging.md#require-approval-for-cross-machine-messages)之前先询问您 | Agent、会话与 worktree | 任意文件 |
| [`keybindingFlavor`](#keybindingflavor) | 已弃用且不再生效；单词编辑快捷键始终[遵循 readline 约定](https://code.claude.com/docs/zh-CN/interactive-mode.md#make-ctrl-w-delete-back-to-whitespace) | 界面与终端 | 任意文件 |
| [`language`](#language) | 让 Claude 使用英语以外的语言回复 | 模型与回复 | 任意文件 |
| [`leftArrowOpensAgents`](#leftarrowopensagents) | 关闭用于[将会话转入后台并打开 Agent 视图](https://code.claude.com/docs/zh-CN/agent-view.md#switch-sessions-without-leaving-the-terminal)的 `←` 快捷键 | 全局配置设置 | 全局配置 |
| [`managedMcpServers`](#managedmcpservers) | 在用户自行添加的服务器之外，为每位用户提供远程 [MCP 服务器](https://code.claude.com/docs/zh-CN/managed-mcp.md#provide-servers-through-managed-settings) | MCP | 托管 |
| [`managedSourcesBehavior`](#managedsourcesbehavior) | 组合您部署的所有[托管来源](https://code.claude.com/docs/zh-CN/managed-settings.md#how-claude-code-combines-managed-sources)，而不是仅使用优先级最高的一个 | 企业与托管设置 | 托管 |
| [`maxEffortLevel`](#maxeffortlevel) | 在所有提供商上为所有模型或单个模型设置 [effort 级别](https://code.claude.com/docs/zh-CN/model-config.md#adjust-effort-level)上限 | 模型与回复 | 任意文件 |
| [`maxProseWidth`](#maxprosewidth) | 在宽终端中限制 Claude 回复中正文文本的最大宽度 | 界面与终端 | 任意文件 |
| [`minimumVersion`](#minimumversion) | 阻止[自动更新](https://code.claude.com/docs/zh-CN/setup.md#pin-a-minimum-version)安装低于某一版本的任何版本 | 更新与版本 | 任意文件 |
| [`model`](#model) | 更改 Claude Code 启动时使用的[模型](https://code.claude.com/docs/zh-CN/model-config.md#set-a-default-model-for-new-sessions) | 模型与回复 | 任意文件 |
| [`modelOverrides`](#modeloverrides) | 将[模型 ID 映射](https://code.claude.com/docs/zh-CN/model-config.md#override-model-ids-per-version)到您的提供商的 ID，例如 Bedrock ARN | 模型与回复 | 任意文件 |
| [`modelPicker`](#modelpicker) | 选择 [`/model` 选择器](https://code.claude.com/docs/zh-CN/model-config#available-models)列出哪些模型，并使用您自己的顺序和标签 | 模型与回复 | 用户或托管 |
| [`modelPricing`](#modelpricing) | 按您组织的合同费率而非标价报告花费 | 模型与回复 | 托管 |
| [`modelSettings`](#modelsettings) | 按模型保存 [effort 级别](https://code.claude.com/docs/zh-CN/model-config.md#adjust-effort-level)或[自动压缩窗口](https://code.claude.com/docs/zh-CN/model-config.md#set-the-auto-compact-window)，或为某个模型设置 effort 上限 | 模型与回复 | 任意文件 |
| [`otelHeadersHelper`](#otelheadershelper) | 使用您自己的命令生成轮换的 [OpenTelemetry](https://code.claude.com/docs/zh-CN/monitoring-usage.md#dynamic-headers) 请求头 | 身份验证与提供商 | 任意文件 |
| [`outputStyle`](#outputstyle) | 通过[输出样式](https://code.claude.com/docs/zh-CN/output-styles.md)更改 Claude 的角色、语气和输出格式 | 模型与回复 | 任意文件 |
| [`parentSettingsBehavior`](#parentsettingsbehavior) | 在部署[托管设置](https://code.claude.com/docs/zh-CN/managed-settings.md)时，应用或丢弃 [SDK 或 IDE 宿主](https://code.claude.com/docs/zh-CN/managed-settings.md#let-an-embedding-host-add-policy)传入的限制 | 企业与托管设置 | 托管 |
| [`permissionExplainerEnabled`](#permissionexplainerenabled) | 已在 v2.1.257 中移除，shell 权限提示上的 `Ctrl+E` 命令说明也一并移除 | 全局配置设置 | 全局配置 |
| [`permissions`](#permissions) | 设置允许、询问和拒绝规则以及初始[权限模式](https://code.claude.com/docs/zh-CN/permission-modes.md) | 权限设置 | 任意文件 |
| [`permissions.additionalDirectories`](#permissions-additionaldirectories) | 授予 Claude 访问[当前目录之外的目录](https://code.claude.com/docs/zh-CN/permissions.md#working-directories)中文件的能力 | 权限设置 | 任意文件 |
| [`permissions.allow`](#permissions-allow) | 无需确认提示即批准所列的[工具使用](https://code.claude.com/docs/zh-CN/permissions.md#permission-rule-syntax) | 权限设置 | 任意文件 |
| [`permissions.ask`](#permissions-ask) | 在所列的[工具使用](https://code.claude.com/docs/zh-CN/permissions.md#permission-rule-syntax)之前始终进行提示 | 权限设置 | 任意文件 |
| [`permissions.blockReadsOutsideWorkingDirectories`](#permissions-blockreadsoutsideworkingdirectories) | 使文件工具在所有权限模式下拒绝读取[工作目录](https://code.claude.com/docs/zh-CN/permissions.md#working-directories)之外的内容 | 权限设置 | 任意文件 |
| [`permissions.defaultMode`](#permissions-defaultmode) | 设置新会话启动时的[权限模式](https://code.claude.com/docs/zh-CN/permission-modes.md#which-mode-a-session-starts-in) | 权限设置 | 任意文件 |
| [`permissions.deny`](#permissions-deny) | 阻止所列的[工具使用](https://code.claude.com/docs/zh-CN/permissions.md#permission-rule-syntax)，包括读取包含机密的文件 | 权限设置 | 任意文件 |
| [`permissions.disableBypassPermissionsMode`](#permissions-disablebypasspermissionsmode) | 阻止任何人进入 [bypassPermissions 模式](https://code.claude.com/docs/zh-CN/permission-modes.md#skip-all-checks-with-bypasspermissions-mode) | 权限设置 | 任意文件 |
| [`plansDirectory`](#plansdirectory) | 选择[计划模式](https://code.claude.com/docs/zh-CN/permission-modes.md#analyze-before-you-edit-with-plan-mode)写入计划文件的位置 | 记忆与上下文 | 任意文件 |
| [`pluginConfigs`](#pluginconfigs) | 存储您在[插件](https://code.claude.com/docs/zh-CN/plugins/overview.md)配置对话框中给出的答案 | 插件与 skill | 用户或托管 |
| [`pluginSuggestionMarketplaces`](#pluginsuggestionmarketplaces) | 选择哪些[市场](https://code.claude.com/docs/zh-CN/plugins/org.md#restrict-what-users-can-install)可以在 `/plugin` 中显示插件安装建议 | 插件与 skill | 托管 |
| [`pluginTrustMessage`](#plugintrustmessage) | 在[插件](https://code.claude.com/docs/zh-CN/plugins/overview.md)信任警告中添加您自己的文字 | 插件与 skill | 托管 |
| [`policyHelper`](#policyhelper) | 在启动时运行一个计算[托管设置](https://code.claude.com/docs/zh-CN/managed-settings.md#compute-the-policy-with-a-helper-program)的可执行文件 | 企业与托管设置 | 托管 |
| [`policyHelper.path`](#policyhelper-path) | 指定 Claude Code 运行的[辅助可执行文件](https://code.claude.com/docs/zh-CN/managed-settings.md#compute-the-policy-with-a-helper-program) | 企业与托管设置 | 托管 |
| [`policyHelper.refreshIntervalMs`](#policyhelper-refreshintervalms) | 在后台按时间间隔重新运行[辅助程序](https://code.claude.com/docs/zh-CN/managed-settings.md#compute-the-policy-with-a-helper-program) | 企业与托管设置 | 托管 |
| [`policyHelper.timeoutMs`](#policyhelper-timeoutms) | 设置 Claude Code 等待[辅助程序](https://code.claude.com/docs/zh-CN/managed-settings.md#compute-the-policy-with-a-helper-program)的时长 | 企业与托管设置 | 托管 |
| [`preferredNotifChannel`](#preferrednotifchannel) | 为任务完成选择[终端响铃或桌面通知](https://code.claude.com/docs/zh-CN/terminal-config.md#get-a-terminal-bell-or-notification) | 远程、桌面与通知 | 任意文件 |
| [`prefersReducedMotion`](#prefersreducedmotion) | [减少或关闭](https://code.claude.com/docs/zh-CN/accessibility.md#accessibility-settings)加载指示器、闪烁微光和闪屏动画 | 界面与终端 | 任意文件 |
| [`prependPlugins`](#prependplugins) | 在用户安装的每个 mod 之前运行您组织的 [mod](https://code.claude.com/docs/zh-CN/plugins/mods/admin.md) | 插件与 skill | 用户或托管 |
| [`processWrapper`](#processwrapper) | 在 macOS 和 Linux 上通过[企业启动器](https://code.claude.com/docs/zh-CN/corporate-launcher.md)运行 Claude Code 的后台进程 | Agent、会话与 worktree | 用户或托管 |
| [`promptCacheTtl`](#promptcachettl) | 选择主对话的[提示缓存生命周期](https://code.claude.com/docs/zh-CN/prompt-caching.md#cache-lifetime) | 模型与回复 | 任意文件 |
| [`promptSuggestionEnabled`](#promptsuggestionenabled) | 隐藏输入框中灰色显示的[提示词建议](https://code.claude.com/docs/zh-CN/interactive-mode.md#prompt-suggestions) | 界面与终端 | 任意文件 |
| [`prStatusFooterEnabled`](#prstatusfooterenabled) | 关闭输入框页脚中的 [PR 审查状态](https://code.claude.com/docs/zh-CN/interactive-mode.md#pr-review-status)徽标及其背后的 Pull Request 检查 | 全局配置设置 | 全局配置 |
| [`prUrlTemplate`](#prurltemplate) | 将 PR 链接指向内部代码审查工具而不是 github.com | Git 与署名 | 任意文件 |
| [`remote.defaultEnvironmentId`](#remote-defaultenvironmentid) | 为 `claude --cloud` 选择默认的[云环境](https://code.claude.com/docs/zh-CN/cloud-environments.md)；自托管的 `ccpool_` ID 仅从用户设置、托管设置和 `--settings` 中读取 | 远程、桌面与通知 | 任意文件 |
| [`remoteControlAtStartup`](#remotecontrolatstartup) | 在会话启动时自动连接 [Remote Control](https://code.claude.com/docs/zh-CN/remote-control.md#enable-remote-control-for-all-sessions) | 远程、桌面与通知 | 任意文件 |
| [`requiredMaximumVersion`](#requiredmaximumversion) | 当版本高于您组织允许的版本时[拒绝启动](https://code.claude.com/docs/zh-CN/setup.md#pin-a-minimum-version) | 更新与版本 | 托管 |
| [`requiredMinimumVersion`](#requiredminimumversion) | 当版本低于您组织要求的版本时[拒绝启动](https://code.claude.com/docs/zh-CN/setup.md#pin-a-minimum-version) | 更新与版本 | 托管 |
| [`respectGitignore`](#respectgitignore) | 将被 gitignore 忽略的文件排除在 [`@` 文件选择器](https://code.claude.com/docs/zh-CN/interactive-mode#quick-commands)之外 | 界面与终端 | 任意文件 |
| [`respondToBashCommands`](#respondtobashcommands) | 阻止 Claude 在 [`!` shell 命令](https://code.claude.com/docs/zh-CN/interactive-mode#shell-mode-with-prefix)运行后作出回复 | 界面与终端 | 任意文件 |
| [`sandbox`](#sandbox) | 在 macOS、Linux 和 WSL2 上[将 Bash 命令与您的文件系统和网络隔离](https://code.claude.com/docs/zh-CN/sandboxing.md) | 沙箱设置 | 任意文件 |
| [`sandbox.allowAppleEvents`](#sandbox-allowappleevents) | 允许[沙箱](https://code.claude.com/docs/zh-CN/sandboxing.md)中的命令在 macOS 上发送 Apple Events | 沙箱设置 | 用户或托管 |
| [`sandbox.allowNestedUserNamespaces`](#sandbox-allownestedusernamespaces) | 允许[沙箱](https://code.claude.com/docs/zh-CN/sandboxing.md#chromium-or-rootless-podman-fails-inside-the-sandbox-on-linux)中的命令在 Linux 和 WSL2 上创建用户命名空间 | 沙箱设置 | 用户或托管 |
| [`sandbox.allowUnsandboxedCommands`](#sandbox-allowunsandboxedcommands) | 允许 Claude 在[沙箱](https://code.claude.com/docs/zh-CN/sandboxing.md#the-unsandboxed-retry-escape-hatch)之外重试被阻止的命令，或禁止这样做 | 沙箱设置 | 任意文件 |
| [`sandbox.autoAllowBashIfSandboxed`](#sandbox-autoallowbashifsandboxed) | 无需权限提示即可运行[沙箱](https://code.claude.com/docs/zh-CN/sandboxing.md#auto-allow-mode)中的命令 | 沙箱设置 | 任意文件 |
| [`sandbox.bwrapPath`](#sandbox-bwrappath) | 将[沙箱](https://code.claude.com/docs/zh-CN/sandboxing.md)指向 `PATH` 之外的 bubblewrap 二进制文件 | 沙箱设置 | 托管 |
| [`sandbox.credentials`](#sandbox-credentials) | 在[沙箱](https://code.claude.com/docs/zh-CN/sandboxing.md#protect-credentials)中隐藏或掩码凭据文件和变量 | 沙箱设置 | 任意文件 |
| [`sandbox.credentials.allowPlaintextInject`](#sandbox-credentials-allowplaintextinject) | 允许[已掩码的凭据](https://code.claude.com/docs/zh-CN/sandboxing.md#mask-credentials)在受信任的测试网络上发送到纯 HTTP 服务 | 沙箱设置 | 用户或托管 |
| [`sandbox.credentials.awsPairs`](#sandbox-credentials-awspairs) | 将自定义命名的 AWS 密钥变量关联为一个凭据以进行[重新签名](https://code.claude.com/docs/zh-CN/sandboxing.md#re-sign-aws-requests) | 沙箱设置 | 用户或托管 |
| [`sandbox.credentials.envVars`](#sandbox-credentials-envvars) | 在[沙箱](https://code.claude.com/docs/zh-CN/sandboxing.md#mask-environment-variables)中取消设置或掩码某个环境变量 | 沙箱设置 | 任意文件 |
| [`sandbox.credentials.files`](#sandbox-credentials-files) | 在[沙箱](https://code.claude.com/docs/zh-CN/sandboxing.md#mask-credential-files)中阻止或掩码对凭据文件的读取 | 沙箱设置 | 任意文件 |
| [`sandbox.credentials.sigv4`](#sandbox-credentials-sigv4) | 选择流式、预签名或 [SigV4A AWS 请求](https://code.claude.com/docs/zh-CN/sandboxing.md#re-sign-aws-requests)是失败还是直接放行 | 沙箱设置 | 用户或托管 |
| [`sandbox.enabled`](#sandbox-enabled) | 在 macOS、Linux 和 WSL2 上开启 [Bash 沙箱隔离](https://code.claude.com/docs/zh-CN/sandboxing.md#get-started) | 沙箱设置 | 任意文件 |
| [`sandbox.enableWeakerNestedSandbox`](#sandbox-enableweakernestedsandbox) | 在非特权容器内运行 Linux [沙箱](https://code.claude.com/docs/zh-CN/sandboxing.md) | 沙箱设置 | 任意文件 |
| [`sandbox.enableWeakerNetworkIsolation`](#sandbox-enableweakernetworkisolation) | 允许 `gh`、`gcloud` 和 `terraform` 在 macOS 上的[沙箱](https://code.claude.com/docs/zh-CN/sandboxing.md#go-based-clis-fail-tls-verification-on-macos)中于 MITM 代理之后验证 TLS | 沙箱设置 | 任意文件 |
| [`sandbox.excludedCommands`](#sandbox-excludedcommands) | 指定 Claude Code 可以在[沙箱](https://code.claude.com/docs/zh-CN/sandboxing.md)之外运行的命令 | 沙箱设置 | 任意文件 |
| [`sandbox.failIfUnavailable`](#sandbox-failifunavailable) | 当[沙箱](https://code.claude.com/docs/zh-CN/sandboxing.md)无法启动时拒绝启动，而不是在无沙箱的情况下运行 | 沙箱设置 | 任意文件 |
| [`sandbox.filesystem`](#sandbox-filesystem) | 控制[沙箱](https://code.claude.com/docs/zh-CN/sandboxing.md#filesystem-isolation)中的命令可以读取和写入哪些路径 | 沙箱设置 | 任意文件 |
| [`sandbox.filesystem.allowManagedReadPathsOnly`](#sandbox-filesystem-allowmanagedreadpathsonly) | 阻止开发者重新开放[您的组织已屏蔽的读取路径](https://code.claude.com/docs/zh-CN/sandboxing.md#keep-developers-from-widening-the-policy) | 沙箱设置 | 托管 |
| [`sandbox.filesystem.allowRead`](#sandbox-filesystem-allowread) | 在 [`denyRead`](#sandbox-filesystem-denyread) 屏蔽的区域内重新开放读取 | 沙箱设置 | 任意文件 |
| [`sandbox.filesystem.allowWrite`](#sandbox-filesystem-allowwrite) | 添加[沙箱](https://code.claude.com/docs/zh-CN/sandboxing.md)中的命令可以写入的路径 | 沙箱设置 | 任意文件 |
| [`sandbox.filesystem.denyRead`](#sandbox-filesystem-denyread) | 阻止[沙箱](https://code.claude.com/docs/zh-CN/sandboxing.md)中的命令读取特定路径 | 沙箱设置 | 任意文件 |
| [`sandbox.filesystem.denyWrite`](#sandbox-filesystem-denywrite) | 阻止[沙箱](https://code.claude.com/docs/zh-CN/sandboxing.md)中的命令写入特定路径 | 沙箱设置 | 任意文件 |
| [`sandbox.filesystem.disabled`](#sandbox-filesystem-disabled) | [关闭文件系统隔离](https://code.claude.com/docs/zh-CN/sandboxing.md#disable-filesystem-isolation)，同时保留网络隔离 | 沙箱设置 | 用户或托管 |
| [`sandbox.ignoreViolations`](#sandbox-ignoreviolations) | 对命令预期会探测的路径静默违规报告 | 沙箱设置 | 任意文件 |
| [`sandbox.network`](#sandbox-network) | 控制[沙箱](https://code.claude.com/docs/zh-CN/sandboxing.md#network-isolation)中的命令可以访问哪些主机、端口和套接字 | 沙箱设置 | 任意文件 |
| [`sandbox.network.allowAllUnixSockets`](#sandbox-network-allowallunixsockets) | 允许[沙箱](https://code.claude.com/docs/zh-CN/sandboxing.md)中的命令连接到所有 Unix 套接字 | 沙箱设置 | 任意文件 |
| [`sandbox.network.allowedDomains`](#sandbox-network-alloweddomains) | 预先允许域名，使[沙箱](https://code.claude.com/docs/zh-CN/sandboxing.md)中的命令访问这些域名时不再提示 | 沙箱设置 | 任意文件 |
| [`sandbox.network.allowLocalBinding`](#sandbox-network-allowlocalbinding) | 允许[沙箱](https://code.claude.com/docs/zh-CN/sandboxing.md)中的命令在 macOS 上监听网络端口并连接到 localhost | 沙箱设置 | 任意文件 |
| [`sandbox.network.allowMachLookup`](#sandbox-network-allowmachlookup) | 允许 macOS 上[沙箱](https://code.claude.com/docs/zh-CN/sandboxing.md)中的工具（如 iOS Simulator 或 Playwright）访问其 XPC 服务 | 沙箱设置 | 任意文件 |
| [`sandbox.network.allowManagedDomainsOnly`](#sandbox-network-allowmanageddomainsonly) | 将网络允许列表锁定为[托管设置](https://code.claude.com/docs/zh-CN/sandboxing.md#keep-developers-from-widening-the-policy) | 沙箱设置 | 托管 |
| [`sandbox.network.allowUnixSockets`](#sandbox-network-allowunixsockets) | 列出[沙箱](https://code.claude.com/docs/zh-CN/sandboxing.md)中的命令在 macOS 上可以使用的 Unix 套接字路径 | 沙箱设置 | 任意文件 |
| [`sandbox.network.deniedDomains`](#sandbox-network-denieddomains) | 为[沙箱](https://code.claude.com/docs/zh-CN/sandboxing.md)中的命令屏蔽域名，即使这些域名位于允许的通配符范围内 | 沙箱设置 | 任意文件 |
| [`sandbox.network.httpProxyPort`](#sandbox-network-httpproxyport) | 通过您自己的代理路由[沙箱](https://code.claude.com/docs/zh-CN/sandboxing.md#custom-proxy-configuration)的 HTTP 流量 | 沙箱设置 | 任意文件 |
| [`sandbox.network.socksProxyPort`](#sandbox-network-socksproxyport) | 通过您自己的代理路由[沙箱](https://code.claude.com/docs/zh-CN/sandboxing.md#custom-proxy-configuration)的 SOCKS 流量 | 沙箱设置 | 任意文件 |
| [`sandbox.network.strictAllowlist`](#sandbox-network-strictallowlist) | 直接拒绝[允许列表](https://code.claude.com/docs/zh-CN/sandboxing.md#network-isolation)之外的主机，而不是进行提示 | 沙箱设置 | 用户或托管 |
| [`sandbox.network.tlsTerminate`](#sandbox-network-tlsterminate) | 让[沙箱](https://code.claude.com/docs/zh-CN/sandboxing.md#network-isolation)代理终止 TLS，以便读取 HTTPS 请求 | 沙箱设置 | 用户或托管 |
| [`sandbox.ripgrep`](#sandbox-ripgrep) | 在[沙箱](https://code.claude.com/docs/zh-CN/sandboxing.md)中使用您自己的 ripgrep 二进制文件 | 沙箱设置 | 用户或托管 |
| [`sandbox.socatPath`](#sandbox-socatpath) | 将[沙箱](https://code.claude.com/docs/zh-CN/sandboxing.md)代理指向 `PATH` 之外的 `socat` 二进制文件 | 沙箱设置 | 托管 |
| [`showClearContextOnPlanAccept`](#showclearcontextonplanaccept) | 在[计划接受界面](https://code.claude.com/docs/zh-CN/permission-modes.md#review-and-approve-a-plan)上显示“清除上下文”选项 | 界面与终端 | 任意文件 |
| [`showThinkingSummaries`](#showthinkingsummaries) | 查看 Claude [思考](https://code.claude.com/docs/zh-CN/model-config.md#extended-thinking)过程的摘要，而不是折叠的占位内容 | 模型与回复 | 任意文件 |
| [`showTurnDuration`](#showturnduration) | 隐藏每条回复后显示的“Cooked for”时长 | 界面与终端 | 任意文件 |
| [`skillListingBudgetFraction`](#skilllistingbudgetfraction) | 为 [skill 列表](https://code.claude.com/docs/zh-CN/skills.md#skill-descriptions-are-cut-short)预留更多或更少的上下文 | 记忆与上下文 | 任意文件 |
| [`skillListingMaxDescChars`](#skilllistingmaxdescchars) | 限制 [skill 列表](https://code.claude.com/docs/zh-CN/skills.md#skill-descriptions-are-cut-short)中每个 skill 描述的长度 | 记忆与上下文 | 任意文件 |
| [`skillOverrides`](#skilloverrides) | 无需编辑 SKILL.md 即可[隐藏或折叠某个 skill](https://code.claude.com/docs/zh-CN/skills.md#override-skill-visibility-from-settings) | 插件与 skill | 任意文件 |
| [`skipAutoPermissionPrompt`](#skipautopermissionprompt) | 跳过您首次自行进入（而非通过内置默认值进入）[自动模式](https://code.claude.com/docs/zh-CN/permission-modes.md#eliminate-prompts-with-auto-mode)时 Claude Code 显示的一次性提示 | 权限设置 | 用户或托管 |
| [`skipDangerousModePermissionPrompt`](#skipdangerousmodepermissionprompt) | 跳过进入 [bypassPermissions 模式](https://code.claude.com/docs/zh-CN/permission-modes.md#skip-all-checks-with-bypasspermissions-mode)之前的确认对话框 | 权限设置 | 用户、本地或托管 |
| [`skipWebFetchPreflight`](#skipwebfetchpreflight) | 在无法访问 Anthropic 时跳过 [WebFetch 主机名检查](https://code.claude.com/docs/zh-CN/tools-reference.md#webfetch-tool-behavior) | 隐私与遥测 | 任意文件 |
| [`spellcheck`](#spellcheck) | 使用您安装的[拼写检查器](https://code.claude.com/docs/zh-CN/interactive-mode.md#check-spelling-as-you-type)为输入框中拼写错误的单词添加下划线 | 界面与终端 | 用户或托管 |
| [`spinnerTipsEnabled`](#spinnertipsenabled) | 隐藏 Claude 工作时加载指示器中的提示 | 界面与终端 | 任意文件 |
| [`spinnerTipsOverride`](#spinnertipsoverride) | 向加载指示器的轮播中添加您自己的提示，或替换内置提示 | 界面与终端 | 任意文件 |
| [`spinnerVerbs`](#spinnerverbs) | 添加或替换轮次运行期间显示的动词 | 界面与终端 | 任意文件 |
| [`sshConfigs`](#sshconfigs) | 向 Desktop 环境选择器添加 [SSH 连接](https://code.claude.com/docs/zh-CN/desktop.md#pre-configure-ssh-connections-for-your-team) | 远程、桌面与通知 | 用户或托管 |
| [`sshHostAllowlist`](#sshhostallowlist) | 限制 [Desktop SSH 会话](https://code.claude.com/docs/zh-CN/desktop.md#restrict-which-ssh-hosts-users-can-connect-to)可以访问的主机 | 远程、桌面与通知 | 托管 |
| [`sshHostCommands`](#sshhostcommands) | 向 Desktop 环境选择器添加[主机由命令提供的 SSH 连接](https://code.claude.com/docs/zh-CN/desktop.md#create-an-ssh-connection-from-a-command) | 远程、桌面与通知 | 用户或托管 |
| [`statusLine`](#statusline) | 运行您自己的命令，在输入框下方渲染[状态栏](https://code.claude.com/docs/zh-CN/statusline.md) | 界面与终端 | 任意文件 |
| [`strictKnownMarketplaces`](#strictknownmarketplaces) | 通过允许列表限定用户可以添加并从中安装的[市场](https://code.claude.com/docs/zh-CN/plugins/overview.md)来源 | 插件与 skill | 托管 |
| [`strictPluginOnlyCustomization`](#strictpluginonlycustomization) | 屏蔽来自用户和项目来源的 [skill](https://code.claude.com/docs/zh-CN/skills.md)、[Agent](https://code.claude.com/docs/zh-CN/sub-agents.md)、[hook](https://code.claude.com/docs/zh-CN/hooks.md) 和 [MCP 服务器](https://code.claude.com/docs/zh-CN/mcp.md) | 插件与 skill | 托管 |
| [`strictPluginOnlyCustomization.agents`](#strictpluginonlycustomization-agents) | 将 [Agent](https://code.claude.com/docs/zh-CN/sub-agents.md) 限定为插件和托管来源 | 插件与 skill | 托管 |
| [`strictPluginOnlyCustomization.hooks`](#strictpluginonlycustomization-hooks) | 将 [hook](https://code.claude.com/docs/zh-CN/hooks.md) 限定为插件和托管来源 | 插件与 skill | 托管 |
| [`strictPluginOnlyCustomization.mcp`](#strictpluginonlycustomization-mcp) | 将 [MCP 服务器](https://code.claude.com/docs/zh-CN/mcp.md)限定为插件和托管来源 | 插件与 skill | 托管 |
| [`strictPluginOnlyCustomization.skills`](#strictpluginonlycustomization-skills) | 将 [skill](https://code.claude.com/docs/zh-CN/skills.md) 限定为插件和托管来源 | 插件与 skill | 托管 |
| [`subagentPromptCacheTtl`](#subagentpromptcachettl) | 为子代理和主对话之外的其他请求选择[提示缓存生命周期](https://code.claude.com/docs/zh-CN/prompt-caching.md#cache-lifetime) | 模型与回复 | 任意文件 |
| [`subagentStatusLine`](#subagentstatusline) | 使用您自己的命令重写[子代理](https://code.claude.com/docs/zh-CN/sub-agents.md)任务显示中的行 | 界面与终端 | 任意文件 |
| [`switchModelsOnFlag`](#switchmodelsonflag) | 当[安全分类器](https://code.claude.com/docs/zh-CN/model-config.md#ask-before-switching)标记某个请求时，自动切换模型或暂停 | 模型与回复 | 任意文件 |
| [`syncClaudeAiPlugins`](#syncclaudeaiplugins) | 停止加载[在您的 claude.ai 账户上启用的插件](https://code.claude.com/docs/zh-CN/plugins/loading.md#synced-plugins)，并停止下载新插件 | 插件与 skill | 用户、本地或托管 |
| [`syncClaudeAiSkills`](#syncclaudeaiskills) | 停止加载[在您的 claude.ai 账户上启用的 skill](https://code.claude.com/docs/zh-CN/skills.md#how-synced-skills-behave)，并停止下载新 skill | 插件与 skill | 用户、本地或托管 |
| [`syntaxHighlightingDisabled`](#syntaxhighlightingdisabled) | 关闭 diff 和代码块中的语法高亮 | 界面与终端 | 任意文件 |
| [`taskOutputMaxChars`](#taskoutputmaxchars) | 已在 v2.1.277 中移除，其所控制大小的 `TaskOutput` 工具也一并移除 | 记忆与上下文 | 任意文件 |
| [`teammateDefaultModel`](#teammatedefaultmodel) | 已在 v2.1.234 中移除；有关 Claude Code 如何为队友选择模型，请参阅[指定队友和模型](https://code.claude.com/docs/zh-CN/agent-teams.md#specify-teammates-and-models) | 全局配置设置 | 全局配置 |
| [`teammateMode`](#teammatemode) | 选择 [agent team 队友的显示方式](https://code.claude.com/docs/zh-CN/agent-teams.md#choose-a-display-mode) | Agent、会话与 worktree | 任意文件 |
| [`terminalProgressBarEnabled`](#terminalprogressbarenabled) | 在支持终端进度条的终端中将其隐藏 | 界面与终端 | 任意文件 |
| [`terminalTitleFromRename`](#terminaltitlefromrename) | 阻止 [`/rename`](https://code.claude.com/docs/zh-CN/sessions#name-your-sessions) 和 `--name` 更改终端标签页标题 | 界面与终端 | 任意文件 |
| [`theme`](#theme) | 选择界面的[颜色主题](https://code.claude.com/docs/zh-CN/terminal-config.md#match-the-color-theme)，内置或自定义均可 | 界面与终端 | 任意文件 |
| [`timeFormat`](#timeformat) | 以 12 小时制或 24 小时制、UTC 或 strftime 模式显示界面中的时间 | 界面与终端 | 任意文件 |
| [`timeZone`](#timezone) | 以系统时区之外的时区显示界面中的时间 | 界面与终端 | 任意文件 |
| [`tui`](#tui) | 选择[全屏](https://code.claude.com/docs/zh-CN/fullscreen.md)或经典终端渲染器 | 界面与终端 | 任意文件 |
| [`ultracode`](#ultracode) | 让 Claude 无需要求即可为每个实质性任务规划[工作流](https://code.claude.com/docs/zh-CN/workflows.md#let-claude-decide-with-ultracode) | 模型与回复 | 任意文件 |
| [`useAutoModeDuringPlan`](#useautomodeduringplan) | 允许[自动模式](https://code.claude.com/docs/zh-CN/permission-modes.md#eliminate-prompts-with-auto-mode)分类器在[计划模式](https://code.claude.com/docs/zh-CN/permission-modes.md#analyze-before-you-edit-with-plan-mode)中审查 shell 命令；设为 `false` 则改为显示提示 | 权限设置 | 用户、本地或托管 |
| [`verbose`](#verbose) | 显示[完整的工具输出](https://code.claude.com/docs/zh-CN/cli-reference.md#cli-flags)而不是截断的摘要；两者都设置时 `viewMode` 优先 | 界面与终端 | 任意文件 |
| [`viewMode`](#viewmode) | 让每个会话都以[默认、详细或专注视图](https://code.claude.com/docs/zh-CN/cli-reference.md#cli-flags)启动 | 界面与终端 | 任意文件 |
| [`vimInsertModeRemaps`](#viminsertmoderemaps) | 将双键 [INSERT 模式序列](https://code.claude.com/docs/zh-CN/interactive-mode.md#remap-insert-mode-key-sequences)（如 `jj`）映射为 Escape | 界面与终端 | 用户或托管 |
| [`voice`](#voice) | 开启[语音听写](https://code.claude.com/docs/zh-CN/voice-dictation.md)并选择按住或轻按模式 | 界面与终端 | 任意文件 |
| [`voiceEnabled`](#voiceenabled) | 使用旧版的单键形式开启[语音听写](https://code.claude.com/docs/zh-CN/voice-dictation.md) | 界面与终端 | 任意文件 |
| [`wheelScrollAccelerationEnabled`](#wheelscrollaccelerationenabled) | 在全屏渲染中关闭[鼠标滚轮加速](https://code.claude.com/docs/zh-CN/fullscreen.md#mouse-wheel-scrolling) | 界面与终端 | 任意文件 |
| [`workflowKeywordTriggerEnabled`](#workflowkeywordtriggerenabled) | 允许提示词中的 `ultracode` 一词启动[工作流](https://code.claude.com/docs/zh-CN/workflows.md)；设为 `false` 则输入该词时不会启动工作流 | hook 与自动化 | 任意文件 |
| [`workflowSizeGuideline`](#workflowsizeguideline) | 设置 Claude 在[动态工作流](https://code.claude.com/docs/zh-CN/workflows.md)中的目标 Agent 数量 | hook 与自动化 | 任意文件 |
| [`worktree`](#worktree) | 配置 Claude Code 创建 git [worktree](https://code.claude.com/docs/zh-CN/worktrees.md) 的方式 | Agent、会话与 worktree | 任意文件 |
| [`worktree.baseRef`](#worktree-baseref) | 从远程默认分支或本地 HEAD 创建新 [worktree](https://code.claude.com/docs/zh-CN/worktrees.md) 的分支 | Agent、会话与 worktree | 任意文件 |
| [`worktree.bgIsolation`](#worktree-bgisolation) | 允许后台会话在不使用 [worktree](https://code.claude.com/docs/zh-CN/worktrees.md) 的情况下编辑工作副本 | Agent、会话与 worktree | 任意文件 |
| [`worktree.location`](#worktree-location) | 选择 [Desktop SSH 会话](https://code.claude.com/docs/zh-CN/desktop.md#ssh-sessions)在远程机器上创建 worktree 的位置 | Agent、会话与 worktree | 用户 |
| [`worktree.sparsePaths`](#worktree-sparsepaths) | 在每个 [worktree](https://code.claude.com/docs/zh-CN/worktrees.md) 中仅检出您需要的目录 | Agent、会话与 worktree | 任意文件 |
| [`worktree.symlinkDirectories`](#worktree-symlinkdirectories) | 将大型目录以符号链接方式链接到每个 [worktree](https://code.claude.com/docs/zh-CN/worktrees.md) 中，而不是复制它们 | Agent、会话与 worktree | 任意文件 |
| [`wslInheritsWindowsSettings`](#wslinheritswindowssettings) | 让 WSL 从 Windows 策略链读取[托管设置](https://code.claude.com/docs/zh-CN/managed-settings.md) | 企业与托管设置 | 托管 |

## 模型与回复

选择 Claude Code 使用哪些模型以及如何回复。有关这些设置如何与 `/model` 命令和环境变量交互，请参阅[模型配置](https://code.claude.com/docs/zh-CN/model-config.md)。

### `advisorModel`

选择 Claude 调用服务器端 [advisor 工具](https://code.claude.com/docs/zh-CN/advisor.md)时由哪个模型来回答。取消设置即可关闭 advisor。advisor 的能力必须至少与主模型相当。有关可接受的模型组合以及选择不被接受的模型时会发生什么，请参阅[选择 advisor 模型](https://code.claude.com/docs/zh-CN/advisor.md#choose-an-advisor-model)。

通常无需手动编辑此键。运行 `/advisor` 会打开一个选择器，其中显示当前选择、可以担任 advisor 的模型以及 **No advisor**。Claude Code 会将您的选择保存到 `~/.claude/settings.json` 中的此键。如果您从 [Remote Control](https://code.claude.com/docs/zh-CN/remote-control.md) 客户端或在连接到远程 worker 的会话中进行选择，该选择仅适用于该会话，不会更改此键。

如果您的账户需要[使用额度同意](https://code.claude.com/docs/zh-CN/advisor.md#fable-advisor-and-usage-credits)，请先运行 `/model fable` 接受。在接受之前，在 `/advisor` 中选择 Fable 不会保存任何内容，Claude Code 会提示您先运行 `/model fable`。

* **作用域**：[`Any file`](#scopes)
* **类型**：字符串，可以是别名 `"fable"`、`"opus"` 或 `"sonnet"` 之一（解析为 Claude Code 当前该模型系列的默认版本），也可以是完整的模型 ID，例如 `"claude-opus-5-5"`
* **默认值**：未设置，因此 advisor 处于关闭状态
* **单会话覆盖**：`--advisor` 在单个会话中优先于此键。[`CLAUDE_CODE_DISABLE_ADVISOR_TOOL`](https://code.claude.com/docs/zh-CN/env-vars) 会关闭 advisor，且此键无法将其重新打开

```json settings.json theme={null}
{
  "advisorModel": "opus"
}
```

在 advisor [不可用](https://code.claude.com/docs/zh-CN/advisor.md#requirements)的提供商上（例如 Amazon Bedrock 和 Claude Platform on AWS），此键不起作用。`"fable"` 需要 [Fable 访问权限](https://code.claude.com/docs/zh-CN/advisor.md#choose-an-advisor-model)。

### `alwaysThinkingEnabled`

将此项设置为 `false` 可在所有会话中关闭[扩展思考](https://code.claude.com/docs/zh-CN/model-config.md#extended-thinking)。思考默认处于开启状态，因此 `true` 不会改变任何内容。大多数人通过 `/config` 设置此项，而不是编辑文件。

在始终进行思考的模型上（例如 Opus 5.5、Sonnet 5.5、Haiku 5.5 和 Fable 模型），`false` 不起作用。在[第三方提供商](https://code.claude.com/docs/zh-CN/third-party-integrations.md)上，Claude Code 会省略 `thinking` 参数而不是关闭思考，因此自适应推理模型可能仍会思考。在 Anthropic API 上关闭思考时，对于已知[不接受该组合](https://code.claude.com/docs/zh-CN/errors.md#effort-isnt-available-with-thinking-turned-off)的模型（例如 Opus 5），Claude Code 会发送 effort `high` 而不是更高的级别。

* **作用域**：[`Any file`](#scopes)
* **类型**：Boolean
  * `true`：不起作用；思考已经处于开启状态
  * `false`：Claude Code 在所有会话中关闭扩展思考
* **默认值**：未设置，因此在支持思考的模型上思考处于开启状态
* **单会话覆盖**：[`MAX_THINKING_TOKENS`](https://code.claude.com/docs/zh-CN/env-vars) 在单个会话中优先于此键：`0` 会关闭思考（受与 `false` 相同的模型和提供商限制），正值则会开启思考，即使此键为 `false`。在自适应推理模型上，该数值本身会被忽略

```json settings.json theme={null}
{
  "alwaysThinkingEnabled": false
}
```

### `availableModels`

限制用户可以为主会话、[子代理](https://code.claude.com/docs/zh-CN/sub-agents.md)、[skill](https://code.claude.com/docs/zh-CN/skills.md) 和 [advisor](https://code.claude.com/docs/zh-CN/advisor.md) 选择哪些模型。托管列表会约束 `/model`、`--model` 以及开发者自己文件中的 `model` 键；列表之外的模型无法被选择。在默认的前缀匹配下，此项本身不会影响 Default 选项；如需影响，请将其与 [`enforceAvailableModels`](#enforceavailablemodels) 配合使用。

* **作用域**：[`Any file`](#scopes)。在托管设置中部署此项可对整个组织强制执行。
* **类型**：模型别名或 ID 的数组
* **默认值**：未设置，因此所有模型都可用

此示例只允许用户选择 Sonnet 和 Haiku 模型：

```json settings.json theme={null}
{
  "availableModels": ["sonnet", "haiku"]
}
```

诸如 `"claude-opus-5"` 之类的模型 ID 条目也会允许在其基础上扩展的后续版本，例如 Opus 5.5。要阻止其中某个版本，请使用 [`deniedModels`](#deniedmodels)。要让每个模型 ID 条目只允许其指定的版本，请使用 [`availableModelsMatch`](#availablemodelsmatch)。请参阅[限制模型选择](https://code.claude.com/docs/zh-CN/model-config.md#restrict-model-selection)。有关第三方提供商 ID，请参阅[为第三方部署固定模型](https://code.claude.com/docs/zh-CN/model-config.md#pin-models-for-third-party-deployments)。

### `availableModelsMatch`

选择 [`availableModels`](#availablemodels) 条目如何匹配模型 ID。默认情况下，模型 ID 条目也会允许在其基础上扩展的后续版本，因此 `"claude-opus-5"` 会允许 Opus 5.5。使用 `"exact"` 时，每个模型 ID 条目只允许其指定的版本，因此该模型的较新版本在您将其列出之前会一直被阻止。需要 Claude Code v2.1.283 或更高版本。

* **作用域**：[`Managed`](#scopes)。Claude Code 会忽略用户、项目和本地设置以及 `--settings` 中的此键，并给出警告
* **类型**：字符串，可选值之一：
  * `"prefix"`：模型 ID 条目允许其对应版本以及在其基础上再扩展一个段的任何模型 ID
  * `"exact"`：模型 ID 条目只允许其指定的版本，因此 `"claude-opus-5"` 允许 Opus 5 但不允许 `claude-opus-5-5`。诸如 `"claude-opus-4-5"` 之类的条目还会允许其带日期的 Anthropic 模型 ID `claude-opus-4-5-20251101`。诸如 `"opus"` 之类的系列别名仍允许整个系列，而 `best`、`opusplan` 和 `default` 条目会被忽略
* **默认值**：`"prefix"`

此示例允许 Opus 5 和 Sonnet 5，但不允许两者的任何后续版本：

```json managed-settings.json theme={null}
{
  "availableModels": ["claude-opus-5", "claude-sonnet-5"],
  "availableModelsMatch": "exact"
}
```

使用 `"exact"` 时，只要列表中至少指定了一个模型或系列，Default 选项也会被限制为列出的模型。请参阅[阻止特定模型或版本](https://code.claude.com/docs/zh-CN/model-config.md#block-specific-models-or-versions)。

### `deniedModels`

阻止特定模型，无论是否配合 [`availableModels`](#availablemodels) 允许列表使用，即使该列表允许这些模型也同样生效。Claude Code 会在 `/model` 选择器中隐藏被阻止的模型，并且在强制执行 `availableModels` 的任何地方都无法选择该模型。使用 Default 选项的会话也不会运行被阻止的模型，详见[阻止特定模型或版本](https://code.claude.com/docs/zh-CN/model-config.md#block-specific-models-or-versions)。需要 Claude Code v2.1.283 或更高版本。

* **作用域**：[`Managed`](#scopes)。Claude Code 会忽略用户、项目和本地设置以及 `--settings` 中的此键，并给出警告
* **类型**：模型别名或 ID 的数组
  * 诸如 `"opus"` 之类的系列别名会阻止该系列中的所有模型
  * 诸如 `"claude-opus-5-5"` 之类的模型 ID 会阻止该版本的所有写法，包括带日期的 ID 和提供商特定的 ID
  * 不带次版本号的模型 ID（例如 `"claude-opus-5"`）也会阻止后续的次版本，例如 Opus 5.5。如需仅阻止 Opus 5，请写成 `"claude-opus-5-0"`
  * `best`、`opusplan` 和 `default` 条目会被忽略
* **默认值**：未设置，因此不阻止任何模型

此示例允许 Opus 和 Sonnet 模型，并阻止 Opus 5.5：

```json managed-settings.json theme={null}
{
  "availableModels": ["opus", "sonnet"],
  "deniedModels": ["claude-opus-5-5"]
}
```

请参阅[阻止特定模型或版本](https://code.claude.com/docs/zh-CN/model-config.md#block-specific-models-or-versions)。

### `effortLevel`

为尚未保存级别的模型设置默认 [effort 级别](https://code.claude.com/docs/zh-CN/model-config.md#adjust-effort-level)。较低的级别在简单任务上更快、更便宜，较高的级别则在复杂问题上进行更深入的推理。

当您在本机的交互式会话中运行 `/effort low`、`medium`、`high` 或 `xhigh` 时，Claude Code 会将该级别保存到 [`modelSettings`](#modelsettings) 下当前模型的条目中，而不是写入此键。在 v2.1.251 之前，`/effort` 会写入此键。

在同一个设置文件中，Claude Code 使用模型已保存的级别，而不是此键。跨文件的优先级请参阅 [`modelSettings`](#modelsettings)。

在连接到远程 worker 的会话中、在 `-p` 运行中以及在 Agent SDK 中，`/effort` 仅适用于该会话。[调整 effort 级别](https://code.claude.com/docs/zh-CN/model-config.md#adjust-effort-level)列出了同样仅适用于该会话的交互式选择。`/effort` 输出的消息会说明实际发生了哪种情况。

* **作用域**：[`Any file`](#scopes)
* **类型**：字符串，可选值之一：
  * `"low"`：推理最少，适用于简短、范围明确、对延迟敏感且对智能要求不高的任务
  * `"medium"`：为可以牺牲部分智能的成本敏感型工作减少 token 使用量
  * `"high"`：在 token 使用量和智能之间取得平衡
  * `"xhigh"`：以更高的 token 消耗进行更深入的推理
* **默认值**：未设置
* **单会话覆盖**：`--effort` 在单个会话中优先于此键，而 [`CLAUDE_CODE_EFFORT_LEVEL`](https://code.claude.com/docs/zh-CN/env-vars) 优先于两者

```json settings.json theme={null}
{
  "effortLevel": "xhigh"
}
```

在您的用户设置文件 `~/.claude/settings.json` 中，此键是 `/effort` 在按模型保存级别之前所写入的旧形式，它会继续在之前适用的地方生效，即 Opus 5、Fable 5.1 及更早的模型。Opus 5.5 及之后发布的模型会忽略它，并以各自的默认级别启动，直到您为它们保存级别为止，`/effort` 会将该级别写入 [`modelSettings`](#modelsettings)。在项目、本地和托管设置中，以及使用 `--settings` 时，此键适用于所有模型。

### `enforceAvailableModels`

`/model` 选择器有一个 **Default** 选项，[`default` 模型设置](https://code.claude.com/docs/zh-CN/model-config#default-model-setting)描述了它解析到的模型。[`availableModels`](#availablemodels) 允许列表会限制您可以指定的模型，但在默认的[前缀匹配](#availablemodelsmatch)下，它不会重新映射您账户类型的默认模型，因此 **Default** 仍可能解析到列表之外的模型。此键弥补了这一漏洞。需要 Claude Code v2.1.175 或更高版本。

当您的组织部署了任何托管设置时，Claude Code 只从托管来源读取此键，并忽略您其他文件中的此键。

有关此键如何应用于启动时的模型检查，请参阅 [Amazon Bedrock](https://code.claude.com/docs/zh-CN/amazon-bedrock.md#when-your-organization-enforces-a-model-allowlist) 和 [Google Cloud 的 Agent Platform](https://code.claude.com/docs/zh-CN/google-vertex-ai.md#when-your-organization-enforces-a-model-allowlist)。

* **作用域**：[`Any file`](#scopes)
* **类型**：Boolean
  * `true`：当 **Default** 会解析到 `availableModels` 之外的模型时，Claude Code 会将其解析为列表中第一个可用的模型
  * `false`：此键不改变 **Default** 的解析方式
* **默认值**：`false`

此示例将指定选择限制为 Sonnet 和 Haiku 模型，并使 **Default** 解析为其中第一个可用的模型：

```json settings.json theme={null}
{
  "availableModels": ["sonnet", "haiku"],
  "enforceAvailableModels": true
}
```

当 `availableModels` 未设置或为空时，此键不起作用。请参阅[为 Default 模型强制执行允许列表](https://code.claude.com/docs/zh-CN/model-config.md#enforce-the-allowlist-for-the-default-model)。需要 Claude Code v2.1.175 或更高版本。

### `fallbackModel`

指定当主模型过载或不可用时 Claude Code 按顺序尝试的备用模型。Claude Code 会在当前轮次的剩余部分切换到链中下一个可用的模型，并显示通知。如果没有配置链，Claude Code 会重试同一模型，然后显示服务器的错误，由您自行重试或切换模型。

一次切换意味着备用模型上会有一个轮次的[提示缓存](https://code.claude.com/docs/zh-CN/prompt-caching.md#switching-models)是冷的；您的下一条消息会再次先尝试主模型。

* **作用域**：[`Any file`](#scopes)
* **类型**：模型别名或 ID 的数组；`"default"` 会展开为默认模型
* **默认值**：未设置，因此失败的请求不会在其他模型上重试
* **单会话覆盖**：`--fallback-model` 在单个会话中优先于此键

此示例在主模型失败时先尝试 Sonnet 5，然后尝试 Haiku 4.5：

```json settings.json theme={null}
{
  "fallbackModel": ["claude-sonnet-5", "claude-haiku-4-5"]
}
```

与大多数数组设置不同，此键不会跨设置文件合并：定义了它的最高优先级文件会提供整个链。如果您的项目文件设置为 `["claude-sonnet-5"]`，而用户文件设置为 `["claude-haiku-4-5"]`，则链只有 `["claude-sonnet-5"]`。Claude Code 最多保留列表中三个不同的允许模型，其余的会被忽略。请参阅[备用模型链](https://code.claude.com/docs/zh-CN/model-config.md#fallback-model-chains)。

### `fastMode`

在可用的会话中开启[快速模式](https://code.claude.com/docs/zh-CN/fast-mode.md)，适用于快速迭代或实时调试等需要以更高的每 token 成本换取速度的交互式工作。通常无需手动编辑此键：运行 `/fast` 会将 `fastMode: true` 写入 `~/.claude/settings.json`，再次运行以关闭快速模式则会移除该键。快速模式仅在 Opus 5.5、Opus 5 和 Opus 4.8 上运行：在其他模型上开启它会将您切换到 Opus，而切换到不受支持的模型则会将其关闭。请参阅[在快速模式开启时切换模型](https://code.claude.com/docs/zh-CN/fast-mode.md#switch-models-while-fast-mode-is-on)。

* **作用域**：[`Any file`](#scopes)
* **类型**：Boolean
  * `true`：Claude Code 在可用的会话中开启快速模式
  * `false`：快速模式保持关闭
* **默认值**：未设置，因此快速模式处于关闭状态
* **单会话覆盖**：[`CLAUDE_CODE_DISABLE_FAST_MODE`](https://code.claude.com/docs/zh-CN/env-vars) 会在单个会话中关闭快速模式，且此键无法将其重新开启

```json settings.json theme={null}
{
  "fastMode": true
}
```

### `fastModePerSessionOptIn`

通常，运行 `/fast` 会将 [`fastMode`](#fastmode) 保存到个人的用户设置中，因此之后每个会话开始时快速模式都会开启。将此键设置为 `true` 可阻止这种行为：已保存的 `fastMode: true` 不再在会话开始时开启快速模式，每个人需要在每个想要使用快速模式的会话中运行 `/fast`。Claude Code 会保留其文件中的 `fastMode` 键，因此关闭此键即可恢复原来的行为。

Team 或 Enterprise 计划的所有者可以通过[服务器托管设置](https://code.claude.com/docs/zh-CN/server-managed-settings.md)在整个组织中部署此项。当托管设置设置了此键时，`/fast on` 在交互式终端会话之外会被拒绝，并报告您的组织已禁用快速模式。这涵盖[非交互模式](https://code.claude.com/docs/zh-CN/headless.md)、[VS Code 扩展](https://code.claude.com/docs/zh-CN/vs-code.md)和[云端会话](https://code.claude.com/docs/zh-CN/claude-code-on-the-web.md)。

* **作用域**：[`Any file`](#scopes)
* **类型**：Boolean
  * `true`：已保存的 `fastMode: true` 不再在会话开始时开启快速模式，因此每个人需要在每个想要使用快速模式的会话中运行 `/fast`；通过 `--settings` 传入的 `fastMode: true` 仍对该会话有效，除非托管设置设置了此键
  * `false`：已保存的 `fastMode: true` 会在之后每个会话开始时开启快速模式
* **默认值**：`false`

```json settings.json theme={null}
{
  "fastModePerSessionOptIn": true
}
```

请参阅[要求按会话选择启用](https://code.claude.com/docs/zh-CN/fast-mode.md#require-per-session-opt-in)。

### `language`

让 Claude 默认使用英语以外的语言回复。回复语言没有固定列表：Claude Code 会将该值原样传递给 Claude，作为始终使用该语言回复的指令，因此 Claude 能识别的任何语言名称都可以使用。Claude Code 不会检查该值，因此拼写错误的名称会按原样传递给 Claude，而不会产生错误。同一个值还会设置[语音听写](https://code.claude.com/docs/zh-CN/voice-dictation.md#change-the-dictation-language)的语言（语音听写确实有固定的[支持的听写语言](https://code.claude.com/docs/zh-CN/voice-dictation.md#change-the-dictation-language)列表），以及自动生成的会话标题的语言。

* **作用域**：[`Any file`](#scopes)
* **类型**：字符串，任意语言名称，例如 `"japanese"`、`"spanish"` 或 `"french"`；Claude Code 不会对其进行验证
* **默认值**：未设置；此时会话标题与您对话所用的语言一致

```json settings.json theme={null}
{
  "language": "japanese"
}
```

### `maxEffortLevel`

为会话可以使用的 [effort 级别](https://code.claude.com/docs/zh-CN/model-config.md#adjust-effort-level)设置上限，较低的级别仍然可用。任何更高的级别都会改为以上限运行，包括来自 `/effort`、`/model` 选择器、`--effort`、[`CLAUDE_CODE_EFFORT_LEVEL`](https://code.claude.com/docs/zh-CN/env-vars)、skill 或子代理的 `effort` frontmatter 或模型自身默认值的级别。Claude Code 会在每个请求之前自行应用该上限，因此它在所有提供商上都有效，包括 Amazon Bedrock、Google Cloud 的 Agent Platform 和 Microsoft Foundry。需要 Claude Code v2.1.267 或更高版本。

* **作用域**：[`Any file`](#scopes)。在托管设置中部署此项可对整个组织强制执行。当多个作用域设置了上限时，以最低的上限为准，因此在一个作用域中设置的上限无法从另一个作用域提高
* **类型**：字符串，可以是 `"low"`、`"medium"`、`"high"`、`"xhigh"` 或 `"max"` 之一。值为 `"max"` 时不设上限
* **默认值**：未设置，因此不应用任何上限
* **按模型设置上限**：在模型的 [`modelSettings`](#modelsettings) 条目中添加 `maxEffortLevel`。只有在同时设置了两者的设置来源中（例如您的用户设置或某个[托管来源](https://code.claude.com/docs/zh-CN/managed-settings.md#how-claude-code-combines-managed-sources)），该条目才会针对该模型替代此键。在该条目中设置 `"max"` 可使该模型不受该来源上限的限制；Claude Code 仍会应用来自其他来源的上限

此示例将所有模型的上限设为 `medium`，并使 Sonnet 4.6 不受限制：

```json settings.json theme={null}
{
  "maxEffortLevel": "medium",
  "modelSettings": {
    "claude-sonnet-4-6": {
      "maxEffortLevel": "max"
    }
  }
}
```

当您的组织还为某个模型设置了 [effort 限制](https://code.claude.com/docs/zh-CN/model-config.md#organization-effort-limits)时，以两个上限中较低的为准。

### `model`

设置每个新会话使用的模型，这样您就不必每次都通过 `/model` 选择。在此处设置并不妨碍您在会话中途切换。如果您的管理员设置了覆盖用户选择的[组织默认模型](https://code.claude.com/docs/zh-CN/model-config.md#organization-default-model)，即使您在用户、项目或本地设置中设置了此键，您也会使用该模型。

* **作用域**：[`Any file`](#scopes)
* **类型**：字符串，模型别名或完整的模型 ID
* **默认值**：未设置，因此 Claude Code 使用您账户的默认模型
* **单会话覆盖**：`--model` 优先于 [`ANTHROPIC_MODEL`](https://code.claude.com/docs/zh-CN/env-vars)，两者在单个会话中都优先于此键，包括托管的 `model`；[`availableModels`](#availablemodels) 列表仍适用于所做的选择

```json settings.json theme={null}
{
  "model": "claude-sonnet-5"
}
```

此处的值优先于 [`ANTHROPIC_DEFAULT_MODEL`](https://code.claude.com/docs/zh-CN/model-config#set-a-default-model-for-new-sessions)，后者仅在没有其他方式选择模型时才会被 Claude Code 使用。

### `modelOverrides`

将 Anthropic 模型 ID 映射到提供商特定的模型 ID，例如 Amazon Bedrock 推理配置文件 ARN。之后，每个模型选择器条目在调用提供商 API 时都会使用其映射的值。管理员在 [Amazon Bedrock、Google Cloud 的 Agent Platform 和 Microsoft Foundry](https://code.claude.com/docs/zh-CN/model-config.md#override-model-ids-per-version) 上使用此项，将每个模型版本路由到特定的推理配置文件、版本名称或部署，以便进行治理、成本分摊或区域路由。

* **作用域**：[`Any file`](#scopes)
* **类型**：将模型 ID 映射到提供商模型 ID 的对象
* **默认值**：未设置

此示例将 Opus 4.6 的所有调用路由到指定的 Bedrock 推理配置文件：

```json settings.json theme={null}
{
  "modelOverrides": {
    "claude-opus-4-6": "arn:aws:bedrock:us-east-1:123456789012:inference-profile/example"
  }
}
```

请参阅[按版本覆盖模型 ID](https://code.claude.com/docs/zh-CN/model-config.md#override-model-ids-per-version)。

### `modelPicker`

按您编写的顺序、使用您选择的标签列出 `/model` 选择器提供的模型，使选择器在内置模型列表之后或代替内置列表显示您组织所运行的模型。每一行的 `model` 都按原样使用，因此它接受 `--model` 所接受的任何内容：诸如 `opus` 之类的别名、Anthropic 模型 ID，或适用于 Amazon Bedrock、Google Cloud 的 Agent Platform、Microsoft Foundry 或 LLM 网关的提供商格式 ID。需要 Claude Code v2.1.242 或更高版本。

* **作用域**：[`User or managed`](#scopes)。Claude Code 从托管设置、`--settings` 和用户设置中读取此键，并忽略项目和本地设置中的此键，以防止您克隆的仓库重新标记选择器。这三者中设置了此键的最高一级提供整个模型列表，Claude Code 从不合并来自两个来源的列表。
* **类型**：包含行数组 `options` 和可选 Boolean 值 `replaceBuiltInOptions` 的对象
* **默认值**：未设置，因此选择器显示内置模型列表

此示例在内置模型列表之后添加两个 Bedrock 部署，并使用您的团队熟悉的名称：

```json managed-settings.json theme={null}
{
  "modelPicker": {
    "options": [
      { "model": "us.anthropic.claude-opus-4-8", "label": "Opus (production)" },
      {
        "model": "us.anthropic.claude-sonnet-4-6",
        "label": "Sonnet (production)",
        "description": "Day-to-day work"
      }
    ]
  }
}
```

#### `modelPicker` 的字段

此键接受两个字段，一个用于行本身，另一个用于指定这些行是替代内置模型列表还是添加到其中。

| 字段 | 类型 | 作用 |
| :- | :- | :- |
| `options` | 行数组，每行包含必需的 `model` 以及可选的 `label`、`description` 和 `behavesAs` | 选择器按此顺序显示的行，但灰显的行会移到底部。如果没有 `label`，对于 Claude Code 已知的模型，会使用内置名称作为行标题，否则使用模型 ID；如果没有 `description`，则会写入通用的第二行说明 |
| `replaceBuiltInOptions` | Boolean，默认为 `false` | 设置为 `true` 时只显示这些行、**Default** 以及会话当前正在使用的模型所在的行。保持未设置则会将这些行添加到内置模型列表之后 |

`options` 中的条目还可以在其 `model` 旁边带有一个可选的 `behavesAs` 字符串，这需要 v2.1.257 或更高版本。当条目的 `model` 比您的 Claude Code 版本更新时，将其设置为您的版本已知的模型 ID，例如 `claude-opus-4-8`。Claude Code 随后会将该已知模型的能力和 effort 默认值应用于该条目，而不是将其模型视为未知模型。条目的标签以及 Claude Code 在请求中发送的模型 ID 不会改变。

开启 `replaceBuiltInOptions` 时，Claude Code 会隐藏所有其他行：内置模型列表、为 [`availableModels`](#availablemodels) 条目添加的行、[网关发现](https://code.claude.com/docs/zh-CN/llm-gateway-protocol.md#model-discovery)找到的模型，以及 [`ANTHROPIC_CUSTOM_MODEL_OPTION`](https://code.claude.com/docs/zh-CN/model-config#add-a-custom-model-option)。关闭时，Claude Code 会跳过内置模型列表已涵盖的列出模型。标签改变的是选择器显示的内容，而不是 Claude Code 运行的模型。

[`availableModels`](#availablemodels) 允许列表仍然适用于这些行。在将列出的模型添加到允许列表之前，请阅读[合并行为](https://code.claude.com/docs/zh-CN/model-config.md#merge-behavior)：特定的模型 ID 会缩小其系列通配符条目的范围。Claude Code 还会在显示选择器之前根据会话检查每一行：

* **丢弃**：Claude Code 无法提供的行，例如已停用的模型或您的组织无权访问的模型
* **灰显**：您暂时无法选择的行，会显示原因
* **没有保留任何行**：Claude Code 保留内置模型列表，并照常按允许列表过滤

Claude Code 会丢弃无法解析的行并保留其余行。请参阅[修复损坏的设置文件](https://code.claude.com/docs/zh-CN/settings.md#fix-a-broken-settings-file)。

### `modelPricing`

按照您组织实际支付的费率而不是标价报告支出。当您的组织有合同费率时设置此项，使开发者看到的金额与您的账单一致。Claude Code 会在 `/usage`、[状态栏](https://code.claude.com/docs/zh-CN/statusline.md)、Agent SDK 的 `total_cost_usd`、[`--max-budget-usd`](https://code.claude.com/docs/zh-CN/cli-reference) 限制以及 [OpenTelemetry](https://code.claude.com/docs/zh-CN/monitoring-usage.md) 成本指标和事件中应用这些费率。费率由您提供：Claude Code 不会从您的合同或 Claude Console 中读取它们。需要 Claude Code v2.1.242 或更高版本。

* **作用域**：[`Managed`](#scopes)。通过服务器托管设置、MDM 策略、`managed-settings.json` 文件或[策略辅助程序](https://code.claude.com/docs/zh-CN/managed-settings.md#compute-the-policy-with-a-helper-program)部署此键。Claude Code 会忽略用户、项目和本地设置、`--settings` 以及 Windows 上用户可写的 [HKCU 注册表](https://code.claude.com/docs/zh-CN/managed-settings.md#where-each-mechanism-stores-the-policy)中的此键。使用服务器托管设置时，每个会话会按标价报告成本，直到该会话的[设置获取](https://code.claude.com/docs/zh-CN/server-managed-settings.md#fetch-and-caching-behavior)确认了该设置为止。嵌入 Claude Code 并设置了 [`CLAUDE_CODE_PROVIDER_MANAGED_BY_HOST`](https://code.claude.com/docs/zh-CN/env-vars) 的宿主应用程序可以通过 SDK 的 [`managedSettings`](https://code.claude.com/docs/zh-CN/agent-sdk/typescript#options) 选项提供自己的费率表，Claude Code 仅在没有托管来源设置此键时才使用该表，并且仅限 Claude Code v2.1.246 或更高版本。
* **类型**：包含可选 `multiplier` 和可选 `overrides` 映射的对象
* **默认值**：未设置，因此除非宿主应用程序提供了费率表，否则 Claude Code 按标价报告

单独设置 `multiplier` 可实现统一折扣或加价，单独设置 `overrides` 可实现按模型的费率，也可以两者都设置。

此示例为 Sonnet 4.6 设置合同费率，然后将所有金额（包括 Sonnet 这一行）降低 15%：

```json managed-settings.json theme={null}
{
  "modelPricing": {
    "multiplier": 0.85,
    "overrides": {
      "claude-sonnet-4-6": {
        "input": 2.4,
        "output": 12,
        "cacheRead": 0.24,
        "cacheWrite": 3
      }
    }
  }
}
```

将 `multiplier` 设置为大于 1（最高 10）可对所有金额加价。加价需要 Claude Code v2.1.271 或更高版本。较早的版本会忽略大于 1 的 `multiplier` 并给出警告，同时保留该设置的其余部分。

有关具体步骤，包括如何确认费率已生效，请参阅[按合同费率报告支出](https://code.claude.com/docs/zh-CN/costs.md#report-spend-at-your-contracted-rates)。

#### `modelPricing` 的字段

| 字段 | 类型 | 作用 |
| :- | :- | :- |
| `multiplier` | 大于 0 且不超过 10 的数字 | 缩放 Claude Code 计算的每一项成本，无论是否有 `overrides` 行涵盖它。小于 1 为折扣，大于 1 为加价 |
| `overrides` | 从模型 ID 到费率对象的映射，费率对象包含 `input`、`output`、`cacheRead` 和 `cacheWrite`，每项取值为 0 到 10000 | 该模型每百万 token 的美元费率，四项均为必需。`cacheWrite` 同时涵盖五分钟和一小时的缓存写入。请参阅[行适用于哪些模型](#which-models-a-modelpricing-row-applies-to) |

Claude Code 完全按照您编写的费率使用某一行，不会加上快速模式附加费或[仅限美国推理的费率](https://platform.claude.com/docs/en/about-claude/pricing)。如果您还设置了 `multiplier`，Claude Code 会在该行费率的基础上应用它。Claude Code 会丢弃包含无法解析的费率的行，或无法解析的 `multiplier`，并保留其余部分；请参阅[修复损坏的设置文件](https://code.claude.com/docs/zh-CN/settings.md#fix-a-broken-settings-file)。

#### `modelPricing` 行适用于哪些模型

Claude Code 根据行的键来决定该行适用于哪些模型：

* **内置模型的 ID**：Claude Code 自身用于内置模型的键，无论该键是模型自己的 ID（例如 `claude-sonnet-4-6`），还是其 Bedrock、Agent Platform 或 Foundry ID。Claude Code 会将该行应用于该模型的每个带日期的快照 ID 和提供商特定 ID。
* **任何其他键**：不是内置模型 ID 的键，例如网关模型别名。Claude Code 只将该行应用于这一个 ID。当某个模型 ID 与您的某个键完全匹配，同时又属于以内置模型 ID 为键的行时，Claude Code 使用完全匹配的那一行。
* **Bedrock 应用程序推理配置文件**：一旦 Claude Code 通过您的 [`modelOverrides`](#modeloverrides) 映射或 [`bedrock:GetInferenceProfile` 查询](https://code.claude.com/docs/zh-CN/amazon-bedrock#iam-configuration)将该配置文件解析为其路由到的模型，Claude Code 就会将该模型的行应用于该配置文件。

### `modelSettings`

为您使用的每个模型保存 [effort 级别](https://code.claude.com/docs/zh-CN/model-config.md#adjust-effort-level)。需要 Claude Code v2.1.251 或更高版本。

在本机的交互式会话中，当您通过 `/effort` 或 `/model` 选择器的 effort 滑块将 `low`、`medium`、`high` 或 `xhigh` 保存为默认值时，Claude Code 会将该级别写入此处您正在使用的模型下，因此您很少需要自己编辑此键。当您在 [VS Code 扩展的模型选择器](https://code.claude.com/docs/zh-CN/vs-code.md#use-the-prompt-box)中选择这些级别之一时，Claude Code 也会以同样的方式将其保存在此处。[`effortLevel`](#effortlevel) 条目列出了 `/effort` 仅适用于该会话的会话类型。

手动编辑此键可以更改或删除您已保存的级别。

此处某个模型的 `effortLevel` 优先于同一设置文件中的顶层 [`effortLevel`](#effortlevel)。跨文件时，Claude Code 会分别解析每个模型：为该模型设置了 `effortLevel`，或设置了[适用于该模型](#effortlevel)的顶层 `effortLevel` 的最高优先级[设置文件](https://code.claude.com/docs/zh-CN/settings.md#settings-precedence)起决定作用，因此托管设置中的 `effortLevel` 优先于您在用户设置中保存的级别。[调整 effort 级别](https://code.claude.com/docs/zh-CN/model-config.md#adjust-effort-level)列出了还有哪些方式可以覆盖已保存的级别，例如启动时的 `--effort`。

要为某个模型的 effort 设置上限而不是设置其级别，请在该模型的条目中添加 [`maxEffortLevel`](#maxeffortlevel) 字段。该字段需要 Claude Code v2.1.267 或更高版本。

* **作用域**：[`Any file`](#scopes)
* **类型**：将模型名称映射到对象的对象，该对象可包含以下任意字段：
  * `effortLevel`：`"low"`、`"medium"`、`"high"` 或 `"xhigh"` 之一
  * [`maxEffortLevel`](#maxeffortlevel)：该模型可以运行的最高 effort 级别
  * `autoCompactWindow`：从 `100000` 到 `1000000` 的 token 数，或表示为该模型调优的窗口的 `"auto"`。[`/autocompact`](https://code.claude.com/docs/zh-CN/model-config#set-the-auto-compact-window) 会保存到此处。对于该模型，此值优先于同一设置文件中的顶层 [`autoCompactWindow`](#autocompactwindow)。需要 Claude Code v2.1.288 或更高版本
* **默认值**：未设置

Claude Code 会将每个条目写在模型的规范名称下（例如 `claude-opus-5-5`），并将该模型的别名、带日期后缀的 ID、`[1m]` ID 以及可识别的提供商特定 ID 匹配到同一个条目。

此示例将 Opus 5.5 保持在 `high`，而其他模型使用各自已保存的级别或默认级别：

```json settings.json theme={null}
{
  "modelSettings": {
    "claude-opus-5-5": {
      "effortLevel": "high"
    }
  }
}
```

运行 `/effort auto` 可清除您为当前使用的模型保存的级别。Claude Code 会保留其他条目以及任何顶层 `effortLevel`。

### `outputStyle`

按名称选择[输出样式](https://code.claude.com/docs/zh-CN/output-styles.md)。输出样式是一组已保存的指令，用于改变 Claude 的角色、语气和输出格式，例如内置的 Explanatory 和 Learning 样式，或您自己编写的样式。

如果您在会话期间更改此键，Claude 会从您的下一条消息开始使用新样式。有关该消息在提示缓存方面的成本，请参阅[更改输出样式](https://code.claude.com/docs/zh-CN/prompt-caching.md#changing-output-style)。在 v2.1.251 之前，该修改只有在您运行 `/clear` 或开始新会话后才会生效。

* **作用域**：[`Any file`](#scopes)
* **类型**：字符串，[内置](https://code.claude.com/docs/zh-CN/output-styles.md#built-in-output-styles)或[自定义](https://code.claude.com/docs/zh-CN/output-styles.md#create-a-custom-output-style)输出样式的名称
* **默认值**：未设置，因此 Claude Code 使用默认样式

此示例选择内置的 Explanatory 样式，它会在任务之间添加教育性的见解：

```json settings.json theme={null}
{
  "outputStyle": "Explanatory"
}
```

### `promptCacheTtl`

选择[提示缓存](https://code.claude.com/docs/zh-CN/prompt-caching.md)保留主对话的时长。此键适用于您的交互式、`-p` 和 Agent SDK 轮次，以及 Claude Code 随这些轮次内联运行的辅助请求。一小时的生命周期可以在较长的间隔期间保持缓存处于热状态，但 API 对每次缓存写入的[计费费率](https://platform.claude.com/docs/en/build-with-claude/prompt-caching#pricing)高于五分钟生命周期。需要 Claude Code v2.1.242 或更高版本。

* **作用域**：[`Any file`](#scopes)
* **类型**：字符串，可选值之一：
  * `"5m"`：缓存保留五分钟
  * `"1h"`：缓存保留一小时
* **默认值**：未设置，因此每个主对话请求使用[其默认生命周期](https://code.claude.com/docs/zh-CN/prompt-caching.md#which-ttl-each-request-gets)
* **单会话覆盖**：[`FORCE_PROMPT_CACHING_5M`](https://code.claude.com/docs/zh-CN/env-vars) 优先于其他所有方式，其次是 [`CLAUDE_CODE_PROMPT_CACHE_TTL`](https://code.claude.com/docs/zh-CN/env-vars)，然后是此键，最后是 [`ENABLE_PROMPT_CACHING_1H`](https://code.claude.com/docs/zh-CN/env-vars)

此示例让主对话使用一小时生命周期，而子代理保持五分钟：

```json settings.json theme={null}
{
  "promptCacheTtl": "1h",
  "subagentPromptCacheTtl": "5m"
}
```

有关每种生命周期的成本，请参阅[缓存生命周期](https://code.claude.com/docs/zh-CN/prompt-caching.md#cache-lifetime)。

### `showThinkingSummaries`

在交互式会话中查看 Claude [扩展思考](https://code.claude.com/docs/zh-CN/model-config.md#extended-thinking)的摘要。如果您希望在使用 `Ctrl+O` 展开思考时看到完整的摘要，请设置此项。未设置或为 `false` 时，Anthropic API 会对思考块进行编辑隐藏，Claude Code 会显示一个折叠的占位内容。

* **作用域**：[`Any file`](#scopes)
* **类型**：Boolean
  * `true`：使用 `Ctrl+O` 展开思考时，您会看到完整的思考摘要
  * `false`：Anthropic API 会对思考块进行编辑隐藏，Claude Code 显示折叠的占位内容
* **默认值**：`false`

```json settings.json theme={null}
{
  "showThinkingSummaries": true
}
```

编辑隐藏只会改变您看到的内容，不会改变模型生成的内容。要减少思考消耗，请改为[降低预算或禁用思考](https://code.claude.com/docs/zh-CN/model-config.md#extended-thinking)。

### `subagentPromptCacheTtl`

选择[提示缓存](https://code.claude.com/docs/zh-CN/prompt-caching.md)保留 Claude Code 在主对话之外发出的请求的时长。此键适用于[子代理](https://code.claude.com/docs/zh-CN/sub-agents.md)、[工作流](https://code.claude.com/docs/zh-CN/workflows.md)以及 Claude Code 自身的后台和辅助请求，例如压缩和会话标题。一小时的生命周期可以在较长的间隔期间保持缓存处于热状态，但 API 对每次缓存写入的[计费费率](https://platform.claude.com/docs/en/build-with-claude/prompt-caching#pricing)高于五分钟生命周期。需要 Claude Code v2.1.242 或更高版本。

* **作用域**：[`Any file`](#scopes)
* **类型**：字符串，可选值之一：
  * `"5m"`：缓存保留五分钟
  * `"1h"`：缓存保留一小时
* **默认值**：未设置，因此这些请求各自使用[其默认生命周期](https://code.claude.com/docs/zh-CN/prompt-caching.md#which-ttl-each-request-gets)
* **单会话覆盖**：[`FORCE_PROMPT_CACHING_5M`](https://code.claude.com/docs/zh-CN/env-vars) 优先于其他所有方式，其次是 [`CLAUDE_CODE_SUBAGENT_PROMPT_CACHE_TTL`](https://code.claude.com/docs/zh-CN/env-vars)，然后是此键，再然后是 [`ENABLE_PROMPT_CACHING_1H`](https://code.claude.com/docs/zh-CN/env-vars)，后者会为每个请求要求一小时生命周期。有关子代理自身 frontmatter 值的优先级位置，请参阅[自行选择 TTL](https://code.claude.com/docs/zh-CN/prompt-caching.md#choose-the-ttl-yourself)

此示例为子代理以及主对话之外的其他请求设置一小时生命周期：

```json settings.json theme={null}
{
  "subagentPromptCacheTtl": "1h"
}
```

此键涵盖 [`promptCacheTtl`](#promptcachettl) 未涵盖的请求，因此同时设置两者即可为 Claude Code 发出的每个请求选择生命周期。有关子代理的缓存与主对话的缓存有何不同，请参阅[子代理与缓存](https://code.claude.com/docs/zh-CN/prompt-caching.md#subagents-and-the-cache)。

### `switchModelsOnFlag`

选择当[安全分类器标记某个请求](https://code.claude.com/docs/zh-CN/model-config.md#automatic-model-fallback)时会发生什么：切换到备用模型并继续，或者暂停以便您在切换和编辑提示词之间做出选择。

* **作用域**：[`Any file`](#scopes)。在 `/config` 中显示为 **Switch models when a message is flagged**，选项为 **Switch automatically** 和 **Ask each time**。
* **类型**：Boolean
  * `true`：Claude Code 切换到备用模型并继续
  * `false`：在交互式会话中，Claude Code 会暂停，以便您在切换和编辑提示词之间做出选择；在无法显示对话框的情况下（例如 `-p` 运行），被标记的请求会以错误结束
* **默认值**：未设置。Claude Code 会自动切换，但在交互式会话中可能会[先询问](https://code.claude.com/docs/zh-CN/model-config.md#ask-before-switching)

```json settings.json theme={null}
{
  "switchModelsOnFlag": false
}
```

请参阅[切换前询问](https://code.claude.com/docs/zh-CN/model-config.md#ask-before-switching)。

### `ultracode`

在开启 [ultracode](https://code.claude.com/docs/zh-CN/workflows.md#let-claude-decide-with-ultracode) 的状态下启动会话。开启后，Claude 会为每个实质性任务规划工作流，而不是等待您提出要求。只有在为您启用了[动态工作流](https://code.claude.com/docs/zh-CN/workflows.md)且您的模型支持 `xhigh` effort 时，Claude 才会规划工作流。此键不会改变会话的 effort 级别：ultracode 以会话所使用的任何级别运行。Claude Code 会读取此键但从不写入它：`/effort ultracode` 仅为当前会话开启 ultracode。

* **作用域**：[`Any file`](#scopes)
* **类型**：Boolean
  * `true`：当为您启用了动态工作流且您的模型支持 `xhigh` 时，会话以开启 ultracode 的状态启动
  * `false`：会话以关闭 ultracode 的状态启动
* **默认值**：未设置，因此 ultracode 处于关闭状态
* **单会话覆盖**：`/effort ultracode` 无需此键即可为单个会话开启 ultracode，而当此键为 `true` 时，`/effort ultracode off` 会为单个会话关闭它。`--effort ultracode` 标志也会为单个会话开启它，并使用 `xhigh` effort

```json settings.json theme={null}
{
  "ultracode": true
}
```

会话的 effort 级别来自 [`effortLevel`](#effortlevel)、[`modelSettings`](#modelsettings) 以及其他 [effort 来源](https://code.claude.com/docs/zh-CN/model-config.md#adjust-effort-level)，而诸如 [`maxEffortLevel`](#maxeffortlevel) 之类的 [effort 上限](https://code.claude.com/docs/zh-CN/model-config.md#organization-effort-limits)会降低该级别，但不会关闭 ultracode。此行为以及 `/effort ultracode off` 形式需要 Claude Code v2.1.284 或更高版本。在 v2.1.284 之前，`ultracode: true` 会以 `xhigh` effort 运行会话，而低于 `xhigh` 的 effort 上限会使 ultracode 保持关闭。Agent SDK 的 `apply_flag_settings` 控制请求也接受此键。

## 权限设置

决定 Claude 无需询问即可执行哪些操作、会话以哪种权限模式启动，以及自动模式的分类器允许哪些操作。有关规则语法和权限模型，请参阅[配置权限](https://code.claude.com/docs/zh-CN/permissions.md)。

### `allowManagedPermissionRulesOnly`

使托管设置成为权限规则的唯一设置来源。此后 Claude Code 会忽略用户、项目、本地以及 `--settings` 文件中的 `allow`、`ask` 和 `deny` 规则，忽略 `--allowedTools`，在权限提示中隐藏"始终允许"选项，并停止保存新规则。

当[来自嵌入宿主的父级设置](https://code.claude.com/docs/zh-CN/managed-settings.md#let-an-embedding-host-add-policy)生效时，Claude Code 会将其视为托管层级的一部分。它会丢弃这些设置中的 `allow` 规则和 `additionalDirectories`，并保留其 `deny` 和 `ask` 规则，但模式以 `!` 开头的 `Read` 和 `Edit` 规则除外。无论您是否设置此键，宿主都无法通过 `!` 规则从托管规则中划出例外路径。

`--disallowedTools` 规则以及当前会话的 `deny` 和 `ask` 规则仍然适用，包括 Claude Code 在会话中途重新加载设置之后。这些规则只起限制作用，因此无法扩大托管规则所授予的范围。在 v2.1.257 之前，Claude Code 会在第一次重新加载设置时丢弃这些命令行规则和会话规则。

有关 `--disallowedTools` 或会话规则中的 `!` 模式可以划出哪些例外，请参阅 [Read 和 Edit 规则](https://code.claude.com/docs/zh-CN/permissions.md#read-and-edit)。

设置此键后，Claude Code v2.1.282 或更高版本还会忽略以下来源中 skill 和 `.claude/commands/` 文件里的 [`allowed-tools`](https://code.claude.com/docs/zh-CN/skills#pre-approve-tools-for-a-skill) frontmatter：

* 仓库的 `.claude/` 目录
* 您的 `~/.claude/skills/` 和 `~/.claude/commands/` 目录，包括[从 claude.ai 同步的 skill](https://code.claude.com/docs/zh-CN/skills.md#where-synced-skills-load)
* `--add-dir` 目录
* 位于 `~/.claude/skills/` 或项目 `.claude/skills/` 内、[通过 `.claude-plugin` 清单声明的插件](https://code.claude.com/docs/zh-CN/plugins/loading#plugins-shared-through-a-repository)
* 仅为单个会话加载、ID 以 [`@inline`](https://code.claude.com/docs/zh-CN/plugins/loading#find-where-a-plugin-came-from) 结尾的插件，前提是同时满足以下所有条件：
  * 该会话是由 Claude Desktop 应用自身启动 Claude Code 的 [Code 会话](https://code.claude.com/docs/zh-CN/desktop.md#environment-configuration)或 [Cowork 会话](https://code.claude.com/docs/zh-CN/managed-settings.md#where-and-when-a-policy-applies)，云端会话除外
  * 该键由您的托管设置设置，而不仅仅由嵌入宿主的[父级设置](https://code.claude.com/docs/zh-CN/managed-settings.md#let-an-embedding-host-add-policy)设置
  * 该会话运行 Claude Code v2.1.305 或更高版本

来自托管设置的 skill 和随附 skill 会保留其 `allowed-tools`。这些 Desktop 会话之外的 `@inline` 插件也会保留其 `allowed-tools`，但名为 [`anthropic-skills`](https://code.claude.com/docs/zh-CN/skills#names-reserved-for-synced-skills) 的插件除外。skill 的 `disallowed-tools` 仍然适用。有关 Claude Code 忽略该字段时开发者会看到什么，请参阅[仅托管权限规则生效时](https://code.claude.com/docs/zh-CN/skills.md#when-only-managed-permission-rules-apply)。

* **作用域**：[`Managed`](#scopes)
* **类型**：Boolean
  * `true`：托管设置成为权限规则的唯一设置来源
  * `false`：除托管规则外，Claude Code 还会应用来自用户、项目、本地以及 `--settings` 文件的权限规则
* **默认值**：未设置，因此除托管规则外，Claude Code 还会应用来自用户、项目和本地设置以及 `--settings` 的权限规则

```json managed-settings.json theme={null}
{
  "allowManagedPermissionRulesOnly": true
}
```

此键不会锁定 MCP 服务器允许列表；如需锁定，请设置 [`allowManagedMcpServersOnly`](#allowmanagedmcpserversonly)。请参阅[仅限托管的设置](https://code.claude.com/docs/zh-CN/managed-settings.md#managed-only-settings)。

### `autoMode`

为[自动模式](https://code.claude.com/docs/zh-CN/permission-modes.md#eliminate-prompts-with-auto-mode)分类器的阻止和允许范围添加您自己的规则。使用它告诉分类器您的组织信任哪些仓库、存储桶和域名，使其不再阻止日常的内部操作。分类器自带[内置的允许和拒绝规则](https://code.claude.com/docs/zh-CN/auto-mode-config.md#inspect-the-defaults-and-your-effective-config)。在数组中包含字面字符串 `"$defaults"`，即可在该位置保留这些内置规则并在其前后添加您的规则；省略它则会用您的规则替换内置规则。

* **作用域**：[`User or managed`](#scopes)
* **类型**：包含 `environment`、`allow`、`soft_deny` 和 `hard_deny` 文字描述规则数组以及 [`classifyAllShell`](#automode-classifyallshell) Boolean 的对象
* **默认值**：未设置，因此分类器仅使用其[内置规则](https://code.claude.com/docs/zh-CN/auto-mode-config.md#inspect-the-defaults-and-your-effective-config)

此示例通过 `"$defaults"` 保留内置的 `soft_deny` 规则，并额外添加一条阻止 `terraform apply` 的规则：

```json settings.json theme={null}
{
  "autoMode": {
    "soft_deny": ["$defaults", "Never run terraform apply"]
  }
}
```

当这些文件中有多个设置了同一个数组时，Claude Code 会将各条目串联起来。有关规则格式以及每个数组的应用方式，请参阅[配置自动模式](https://code.claude.com/docs/zh-CN/auto-mode-config.md)。

### `autoMode.classifyAllShell`

在自动模式处于活动状态时，将每条 Bash 和 PowerShell 命令都交由自动模式分类器处理。默认情况下，自动模式仅暂停可能运行任意代码的允许规则：针对整个工具的规则和通配符规则（例如 `Bash(*)`），以及解释器或 shell 包装器前缀（例如 `Bash(python *)`）。与任何其他允许规则匹配的命令（例如 `Bash(npm test)`）会跳过分类器，除非该命令带有[按命令允许的域名](https://code.claude.com/docs/zh-CN/sandboxing.md#per-command-allowed-domains-in-auto-mode)。跳过时，规则前缀未预料到的破坏性参数可能会在未被检查的情况下通过。设置此键会在会话期间暂停所有 shell 允许规则，使分类器能检查每一条命令。需要 Claude Code v2.1.193 或更高版本。

* **作用域**：[`User or managed`](#scopes)。凡是读取 [`autoMode`](#automode) 的位置都会读取此键。
* **类型**：Boolean
  * `true`：在自动模式处于活动状态时，Claude Code 将每条 Bash 和 PowerShell 命令交由分类器处理，并暂停您的 shell 允许规则；在自动模式之外，这些规则仍然适用
  * `false`：自动模式仅暂停可能运行任意代码的允许规则，例如 `Bash(*)` 和 `Bash(python *)`；与任何其他允许规则匹配的命令会跳过分类器，除非它带有[按命令允许的域名](https://code.claude.com/docs/zh-CN/sandboxing.md#per-command-allowed-domains-in-auto-mode)，而其他所有 shell 命令都会经过分类器
* **默认值**：`false`

```json settings.json theme={null}
{
  "autoMode": {
    "classifyAllShell": true
  }
}
```

请参阅[将所有 shell 命令交由分类器处理](https://code.claude.com/docs/zh-CN/auto-mode-config.md#route-all-shell-commands-through-the-classifier)。

### `disableAutoMode`

从 `Shift+Tab` 循环中移除[自动模式](https://code.claude.com/docs/zh-CN/permission-modes.md#eliminate-prompts-with-auto-mode)。任何原本会[以自动模式启动](https://code.claude.com/docs/zh-CN/permission-modes.md#which-mode-a-session-starts-in)的会话，无论是由于 `--permission-mode auto`、设置文件还是内置默认值，都会改为以 `default` 启动。管理员可在托管设置中设置此项，以阻止其组织中的开发者使用自动模式。

* **作用域**：[`Any file`](#scopes)。在[托管设置](https://code.claude.com/docs/zh-CN/managed-settings.md)中最为有用，因为用户无法覆盖它。也可以在 `permissions` 下以 `permissions.disableAutoMode` 的形式设置。
* **类型**：字符串 `"disable"`
* **默认值**：未设置

```json settings.json theme={null}
{
  "disableAutoMode": "disable"
}
```

### `permissions`

控制 Claude 无需询问即可使用哪些工具、哪些工具始终需要提示、哪些工具被阻止，并设置会话启动时的[权限模式](https://code.claude.com/docs/zh-CN/permission-modes.md)。下面的每个 `permissions.*` 键都嵌套在此对象下。

* **作用域**：[`Any file`](#scopes)
* **类型**：包含 `allow`、`ask`、`deny`、`additionalDirectories`、`blockReadsOutsideWorkingDirectories`、`defaultMode`、`disableBypassPermissionsMode` 和 `disableAutoMode` 的对象
* **默认值**：未设置

此示例无需询问即批准 `npm run` 命令，在 `git push` 之前进行提示，阻止读取 `.env`，并以 `acceptEdits` 启动会话：

```json settings.json theme={null}
{
  "permissions": {
    "allow": ["Bash(npm run *)"],
    "ask": ["Bash(git push *)"],
    "deny": ["Read(./.env)"],
    "defaultMode": "acceptEdits"
  }
}
```

这三个规则数组使用同一种语法；请参阅 `permissions.allow` 下的[权限规则语法](#permission-rule-syntax)。有关来自不同文件的权限规则如何组合，请参阅[权限规则如何跨作用域合并](https://code.claude.com/docs/zh-CN/permissions.md#settings-precedence)；有关一般设置键如何组合，请参阅设置指南中的[设置优先级](https://code.claude.com/docs/zh-CN/settings.md#settings-precedence)。

### `useAutoModeDuringPlan`

选择 Claude Code 是否在计划模式下使用自动模式分类器审查 shell 命令。在默认值 `true` 下，当自动模式可用时，分类器会在规划期间审查每条命令，您不会看到提示，[关键路径删除](https://code.claude.com/docs/zh-CN/permission-modes.md#critical-paths)除外。设置为 `false` 后，对于内置只读命令集之外的每条命令，您都会收到权限提示。在 `/config` 中显示为 **Use auto mode during plan**。

* **作用域**：[`User, local, or managed`](#scopes)。仓库无法替您关闭它。
* **类型**：Boolean
  * `true`：与未设置相同；当自动模式可用时，分类器会在规划期间审查每条 shell 命令，而不是向您发出提示，[关键路径删除](https://code.claude.com/docs/zh-CN/permission-modes.md#critical-paths)除外。这些文件中任何一个设置为 `false` 仍会将其关闭
  * `false`：对于内置只读命令集之外的每条命令，您都会收到权限提示
* **默认值**：`true`

```json settings.json theme={null}
{
  "useAutoModeDuringPlan": false
}
```

### `permissions.allow`

列出 Claude Code 无需询问您即可批准的工具使用。在 MCP 规则中，`*` 只能出现在 `mcp__<server>__` 前缀之后的工具名称中，例如 `mcp__github__get_*`；它不能出现在服务器名称中。

* **作用域**：[`Any file`](#scopes)
* **类型**：权限规则字符串数组
* **默认值**：未设置
* **按会话覆盖**：`--allowedTools` 为单个会话添加允许规则，而任何设置文件中的拒绝规则仍会阻止其所指定的工具

此示例批准 `git diff`，并允许 Claude Code 无需询问即可读取您的 `.zshrc`：

```json settings.json theme={null}
{
  "permissions": {
    "allow": ["Bash(git diff *)", "Read(~/.zshrc)"]
  }
}
```

只有在您为该文件夹接受[工作区信任对话框](https://code.claude.com/docs/zh-CN/permissions.md#project-allow-rules-and-workspace-trust)之后，Claude Code 才会应用项目 `.claude/settings.json` 中的 `allow` 规则。

#### 权限规则语法

权限规则采用 `Tool` 或 `Tool(specifier)` 格式。Claude Code 先评估 `deny` 规则，然后是 `ask`，最后是 `allow`，第一个匹配项决定结果，而不论每条规则的具体程度如何；请参阅[权限规则评估顺序](https://code.claude.com/docs/zh-CN/permissions.md#manage-permissions)。

每一行显示一种规则形式及其匹配的内容。

| 规则 | 匹配内容 |
| :- | :- |
| `Bash` | 所有 Bash 命令 |
| `Bash(npm run *)` | 以 `npm run` 开头的命令 |
| `Read(./.env)` | 对 `.env` 文件的读取 |
| `WebFetch(domain:example.com)` | 对 example.com 的获取请求 |

有关完整的规则语法，包括通配符行为、针对 Read、Edit、WebFetch、MCP 和 Agent 规则的工具特定模式，以及 Bash 模式的安全限制，请参阅[权限规则语法](https://code.claude.com/docs/zh-CN/permissions.md#permission-rule-syntax)。

### `permissions.ask`

列出即使在原本会批准它们的权限模式（例如 `acceptEdits` 或 `bypassPermissions`）下，也会提示您确认的工具使用。在 `dontAsk` 模式下，Claude Code 会拒绝匹配的工具使用，而不是发出提示。

* **作用域**：[`Any file`](#scopes)
* **类型**：权限规则字符串数组
* **默认值**：未设置

```json settings.json theme={null}
{
  "permissions": {
    "ask": ["Bash(git push *)"]
  }
}
```

### `permissions.deny`

列出 Claude Code 阻止的工具使用。将其用于存放 API 密钥、机密或环境值的文件：Claude Code 会从文件发现和搜索结果中排除匹配的文件，拒绝对其进行读取，并在匹配路径上阻止 [Edit 和 Write 工具](https://code.claude.com/docs/zh-CN/permissions.md#read-and-edit)。

Read 和 Edit 拒绝规则适用于 Claude 的内置文件工具、Claude Code 在 Bash 中识别的文件命令（例如 `cat`、`head`、`tail`、`sed` 和 `tee`），以及 Bash [重定向](https://code.claude.com/docs/zh-CN/permissions.md#redirections)（例如 `> file` 和 `< file`）的目标；它们不适用于不指明文件名即可读取文件的命令（例如 `grep -r pattern .`），也不适用于任意子进程，因此如需操作系统级别的强制执行，请[启用沙箱](https://code.claude.com/docs/zh-CN/sandboxing.md)。

* **作用域**：[`Any file`](#scopes)
* **类型**：权限规则字符串数组
* **默认值**：未设置
* **按会话覆盖**：`--disallowedTools` 会在此键之外为单个会话添加拒绝规则

此示例拒绝读取 `.env` 文件、`secrets` 目录和一个凭据文件，并阻止 `curl` 命令：

```json settings.json theme={null}
{
  "permissions": {
    "deny": [
      "Read(./.env)",
      "Read(./.env.*)",
      "Read(./secrets/**)",
      "Read(./config/credentials.json)",
      "Bash(curl *)"
    ]
  }
}
```

工具名称接受 glob 模式，因此 `"*"` 会拒绝所有工具，`"mcp__*"` 会拒绝所有 MCP 工具。只要 Claude 仍有任何其他工具可用，Claude Code 就会忽略针对 [`EndConversation`](https://code.claude.com/docs/zh-CN/tools-reference#endconversation-tool-behavior) 工具的拒绝规则。`Bash` 拒绝规则按照 Claude 编写的命令原样进行匹配，因此 `Bash(curl *)` 无法阻止 `/usr/bin/curl` 或 `sh -c 'curl …'`；请参阅 [Bash 规则无法匹配的内容](https://code.claude.com/docs/zh-CN/permissions.md#bash-rule-limits)。此键取代了已弃用的 `ignorePatterns` 配置。

### `permissions.additionalDirectories`

允许 Claude 访问您启动时所在目录之外的目录中的文件，将其作为额外的[工作目录](https://code.claude.com/docs/zh-CN/permissions.md#working-directories)。大多数 `.claude/` 配置[不会](https://code.claude.com/docs/zh-CN/permissions.md#additional-directories-grant-file-access-not-configuration)从这些目录中被发现。

* **作用域**：[`Any file`](#scopes)，但项目和本地条目所授予的[沙箱写入访问权限受到限制](https://code.claude.com/docs/zh-CN/sandboxing.md#repository-settings-under-an-admin-required-sandbox)
* **类型**：目录路径数组
* **默认值**：未设置
* **按会话覆盖**：`--add-dir` 和 `/add-dir` 会在此键之外为单个会话添加目录

```json settings.json theme={null}
{
  "permissions": {
    "additionalDirectories": ["../docs/"]
  }
}
```

与 `allow` 规则一样，项目 `.claude/settings.json` 中的条目只有在您为该文件夹接受[工作区信任对话框](https://code.claude.com/docs/zh-CN/permissions.md#project-allow-rules-and-workspace-trust)之后才会生效。

### `permissions.blockReadsOutsideWorkingDirectories`

使 Claude 的文件工具在所有权限模式下（包括 `bypassPermissions`）拒绝读取[工作目录](https://code.claude.com/docs/zh-CN/permissions.md#working-directories)之外的内容。Claude Code 会拒绝对这些路径的 `Read`、`Grep`、`Glob` 和 `LSP` 调用，并告知 Claude 请您使用 `/add-dir` 添加该目录。Claude Code 自身需要的文件仍可读取，例如 `~/.claude/` 下的 skill、插件、规则、Agent、命令以及 `CLAUDE.md` 记忆文件。需要 Claude Code v2.1.257 或更高版本。

Claude Code 不会以同样的方式拒绝 shell 命令：

* [任何模式都不会自动批准的操作](https://code.claude.com/docs/zh-CN/permission-modes.md#actions-no-mode-auto-approves)介绍了读取此类路径的 shell 命令何时会向您发出提示
* [阻止状态下的沙箱命令](#sandboxed-commands-under-the-block)介绍了沙箱中的命令可以读取哪些内容

shell 解析器无法追踪的 Bash 命令（例如多次切换目录或运行子 shell 的命令），即使在自动模式和 `bypassPermissions` 模式下也会向您发出提示。即使命令未指明工作目录之外的任何路径，也会出现该提示。当命令在[沙箱](https://code.claude.com/docs/zh-CN/sandboxing.md)中运行且沙箱强制执行该阻止时，此提示不适用。

当您在[自动模式首次读取工作目录之外内容前的提示](https://code.claude.com/docs/zh-CN/permission-modes.md#first-read-outside-the-working-directories)中选择阻止此类读取时，Claude Code 也会在此处写入 `true`。

* **作用域**：[`Any file`](#scopes)。任何文件中的 `true` 都会生效，因此仓库可以为自身开启阻止，但无法解除您设置的阻止。
* **类型**：Boolean
  * `true`：Claude 的文件工具拒绝读取工作目录之外的内容
  * `false`：与未设置相同；如果其他文件设置了 `true`，阻止仍然适用
* **默认值**：未设置，因此对工作目录之外内容的读取遵循您的[权限模式](https://code.claude.com/docs/zh-CN/permission-modes.md)

```json settings.json theme={null}
{
  "permissions": {
    "blockReadsOutsideWorkingDirectories": true
  }
}
```

您通过 `--add-dir`、`/add-dir` 或用户设置或托管设置中的 `additionalDirectories` 添加的目录，在该阻止中均视为工作目录。仅在仓库设置中添加的目录不计入：即 `.claude/settings.json` 中的目录，以及 `.claude/settings.local.json` 中的目录（除非 git 将该文件报告为未跟踪）。在不是 git 仓库的目录中，或当 git 跟踪该文件时，Claude Code 会将 `.claude/settings.local.json` 视为仓库设置，因此请将您希望保持可读的目录放在用户设置中。

当 [`autoMemoryDirectory`](#automemorydirectory) 来自项目的 `.claude/settings.json`，或来自[被视为由仓库提供](https://code.claude.com/docs/zh-CN/permissions.md#when-your-local-settings-file-needs-trust)的 `.claude/settings.local.json` 时，Claude Code 不会从该目录加载任何[自动记忆](https://code.claude.com/docs/zh-CN/memory.md#storage-location)，也不会向其中保存任何内容。

要解除阻止，请从所有设置了此键的设置文件中删除该键，然后启动新会话。

#### 阻止状态下的沙箱命令

启用[沙箱隔离](https://code.claude.com/docs/zh-CN/sandboxing.md)时，该阻止也涵盖沙箱中的命令。Claude Code 会拒绝它们读取您的主目录以及存放用户文件的其他根目录：`/Users`、`/home`、`/root`、`/Volumes`、`/mnt`、`/media`、`/run/media` 和 `/srv`。然后它会重新开放工作目录、Claude Code 在会话中创建的 [worktree](https://code.claude.com/docs/zh-CN/worktrees.md)、会话临时目录，以及命令所需的 `~/.claude` 部分（例如 skill 和插件）。在阻止生效期间，来自仓库设置的 `allowRead` 和 `allowWrite` 条目不计入。

当会话的工作目录是一个链接的 [git worktree](https://code.claude.com/docs/zh-CN/worktrees.md)（包括 Claude Code 在会话中途进入的 worktree）时，仓库的公共 `.git` 目录对沙箱中的命令保持可读可写，因此 git 可以在其中继续正常工作。

在以下情况下，该阻止不会作用于沙箱中的命令，但 Claude 的文件工具仍会强制执行：

* 已通过 [`sandbox.filesystem.disabled`](#sandbox-filesystem-disabled) 关闭文件系统隔离
* 已设置 [`allowManagedReadPathsOnly`](#sandbox-filesystem-allowmanagedreadpathsonly)
* 您启动 Claude Code 时所在目录的路径包含 glob 字符，例如 `*`、`?` 或 `[`

在阻止状态下，Claude Code 会向沙箱中的命令重新开放您的全局 git 配置文件，使 `git` 保留您的身份和设置：

* `~/.gitconfig`
* `$XDG_CONFIG_HOME/git`（默认为 `~/.config/git`）下的 `config`、`ignore` 和 `attributes` 文件
* 您的全局 git 配置通过 `[include]`、`[includeIf]`、`core.excludesFile` 或 `core.attributesFile` 指定的文件

Claude Code 会分别判断每个文件。如果某个文件位于沙箱中的命令可以直接或通过符号链接写入的位置，Claude Code 不会重新开放该文件所指定的文件。

在 Linux 和 WSL2 上，作为符号链接的配置文件在其自身路径上可能仍不可读，此时 `git` 会在没有该文件的情况下运行。`~/.git-credentials` 和 `$XDG_CONFIG_HOME/git/credentials` 仍保持阻止状态。

如果重新开放的文件中包含机密（例如 `http.extraHeader` 令牌），请将其路径添加到 [`sandbox.filesystem.denyRead`](#sandbox-filesystem-denyread)。覆盖某个文件的 `denyRead` 条目始终优先于此重新开放。

### `permissions.defaultMode`

设置新会话启动时的[权限模式](https://code.claude.com/docs/zh-CN/permission-modes.md)。如果不设置，会话将以您的使用入口的[内置默认值](https://code.claude.com/docs/zh-CN/permission-modes.md#which-mode-a-session-starts-in)启动。

* **作用域**：[`Any file`](#scopes)。`auto` 和 `bypassPermissions` 在项目或本地设置中不生效，因此请改为在 `~/.claude/settings.json` 中设置它们。在 v2.1.257 之前，`bypassPermissions` 在任何文件中都会生效。对于 VS Code 扩展启动的对话，Claude Code 仅读取用户、托管和 `--settings` 中的值。
* **类型**：字符串，取值之一：
  * `"default"`：Claude Code 无需询问仅执行读取操作
  * `"acceptEdits"`：Claude Code 还会无需询问即执行文件编辑和常见的文件系统命令，例如 `mkdir` 和 `mv`
  * `"plan"`：Claude Code 会进行读取和规划，但在您批准计划之前阻止编辑
  * `"auto"`：Claude Code 运行时不会出现常规提示；在 shell 命令和网络请求等操作运行之前，后台分类器会检查这些操作是否与您的请求一致
  * `"dontAsk"`：Claude Code 自动拒绝所有原本会发出提示的调用，不过您安装的 [mod](https://code.claude.com/docs/zh-CN/plugins/mods/overview.md) 仍可批准[其中部分调用](https://code.claude.com/docs/zh-CN/permissions.md#precedence-among-rules-hooks-and-mods)。工作目录内的文件读取、其他无需批准的操作以及预先批准的工具仍会运行，但从[网络路径](https://code.claude.com/docs/zh-CN/permissions.md#network-paths)进行的读取除外
  * `"bypassPermissions"`：Claude Code 无需询问即执行所有操作
  * `"manual"`：`"default"` 的别名
* **默认值**：未设置
* **按会话覆盖**：`--permission-mode` 及其在 `bypassPermissions` 方面的等效项 `--dangerously-skip-permissions` 在单个会话中优先于此键

```json settings.json theme={null}
{
  "permissions": {
    "defaultMode": "acceptEdits"
  }
}
```

权限规则叠加在每种模式之上：`deny` 规则在所有模式下都会阻止，包括 `bypassPermissions`。请参阅[权限模式](https://code.claude.com/docs/zh-CN/permission-modes.md)。在云端会话中，Claude Code 仅采用此键中的 `acceptEdits`、`plan`、`default` 和 `auto`。对于 VS Code 扩展启动的对话，请参阅[扩展读取哪个设置作为起始权限模式](https://code.claude.com/docs/zh-CN/permission-modes.md#switch-permission-modes)。

### `permissions.disableBypassPermissionsMode`

阻止任何人进入 `bypassPermissions` 模式。此后 Claude Code 会拒绝 `--dangerously-skip-permissions` 标志，并忽略 [Agent 定义](https://code.claude.com/docs/zh-CN/sub-agents.md#permission-modes)中的 `permissionMode: bypassPermissions`，因此子代理会以父会话的权限模式运行。

* **作用域**：[`Any file`](#scopes)。通常在[托管设置](https://code.claude.com/docs/zh-CN/managed-settings.md)中设置，以强制执行组织策略。
* **类型**：字符串 `"disable"`
* **默认值**：未设置
* **按会话覆盖**：此键优先于 `--dangerously-skip-permissions`，设置此键期间 Claude Code 会拒绝该标志

```json settings.json theme={null}
{
  "permissions": {
    "disableBypassPermissionsMode": "disable"
  }
}
```

在 v2.1.223 之前，即使禁用了绕过模式，Claude Code 仍会应用 frontmatter 中的权限模式。

### `skipAutoPermissionPrompt`

跳过描述[自动模式](https://code.claude.com/docs/zh-CN/permission-modes.md#eliminate-prompts-with-auto-mode)的一次性通知。当您首次自行进入自动模式时（例如通过您自己的设置或模式选择器，而不是由内置默认值以该模式启动会话），Claude Code 会显示此通知。Claude Code 只显示一次该通知，之后会记录它已显示过，因此此键仅在通知尚未出现时才有意义。

* **作用域**：[`User or managed`](#scopes)。仓库无法替您设置它。
* **类型**：Boolean
  * `true`：Claude Code 跳过该通知
  * `false`：与未设置相同；除非这些文件中的另一个设置了 `true`，否则通知会出现一次
* **默认值**：未设置，因此通知会出现一次

```json settings.json theme={null}
{
  "skipAutoPermissionPrompt": true
}
```

### `skipDangerousModePermissionPrompt`

跳过 Claude Code 在会话进入 `bypassPermissions` 模式之前显示的确认对话框，无论是由 `--dangerously-skip-permissions` 还是 `defaultMode: "bypassPermissions"` 触发。当您接受过一次该对话框后，Claude Code 会在您的用户设置中此处写入 `true`。

* **作用域**：[`User, local, or managed`](#scopes)。不受信任的仓库无法替您跳过该对话框。
* **类型**：Boolean
  * `true`：Claude Code 在会话进入 `bypassPermissions` 模式之前跳过确认对话框
  * `false`：与未设置相同；除非这些文件中的另一个设置了 `true`，否则对话框会出现
* **默认值**：未设置，因此对话框会出现

```json settings.json theme={null}
{
  "skipDangerousModePermissionPrompt": true
}
```

## 沙箱设置

将 Claude 运行的命令与您的文件系统、网络和凭据隔离。有关沙箱隔离的工作原理和平台要求，请参阅[沙箱隔离](https://code.claude.com/docs/zh-CN/sandboxing.md)。

### `sandbox`

通过[沙箱隔离](https://code.claude.com/docs/zh-CN/sandboxing.md)将 Claude 运行的 Bash 命令与您的文件系统和网络隔离。使用 `enabled` 启用沙箱，然后通过 `filesystem`、`network` 和 `credentials` 子对象收窄或放宽沙箱化命令可以访问的范围。沙箱可在 macOS、Linux 和 WSL2 上运行。

* **作用域**：[`Any file`](#scopes)
* **类型**：包含 `enabled`、`failIfUnavailable`、`autoAllowBashIfSandboxed`、`excludedCommands`、`allowUnsandboxedCommands`、`enableWeakerNestedSandbox`、`enableWeakerNetworkIsolation`、`allowAppleEvents`、`allowNestedUserNamespaces`、`bwrapPath`、`socatPath`、`ignoreViolations` 和 `ripgrep` 的对象，外加 `filesystem`、`network` 和 `credentials` 对象
* **默认值**：未设置，因此 Claude Code 在没有沙箱的情况下运行命令

以下配置启用沙箱，对沙箱化命令跳过权限提示，在沙箱外运行 `docker`，开放两个额外的写入路径，隐藏您的 AWS 凭据文件，并预先允许 GitHub 和 npm：

```json settings.json theme={null}
{
  "sandbox": {
    "enabled": true,
    "autoAllowBashIfSandboxed": true,
    "excludedCommands": ["docker *"],
    "filesystem": {
      "allowWrite": ["/tmp/build", "~/.kube"],
      "denyRead": ["~/.aws/credentials"]
    },
    "network": {
      "allowedDomains": ["github.com", "*.npmjs.org"]
    }
  }
}
```

当托管设置设定了 `enabled` 或 `failIfUnavailable` 等布尔键时，该值会覆盖开发者设置的任何值。Claude Code 会在会话加载的各设置作用域之间合并数组键，因此开发者可以追加条目；有关仅限托管设置的锁定，请参阅[防止开发者放宽策略](https://code.claude.com/docs/zh-CN/sandboxing.md#keep-developers-from-widening-the-policy)。要为组织强制要求沙箱，请参阅[使用托管设置强制沙箱隔离](https://code.claude.com/docs/zh-CN/sandboxing.md#enforce-sandboxing-with-managed-settings)。

### `sandbox.enabled`

为 Bash 命令启用[沙箱隔离](https://code.claude.com/docs/zh-CN/sandboxing.md)。当您在 `/sandbox` 面板中选择模式时，Claude Code 会将此键写入当前项目的 `.claude/settings.local.json`；在 `~/.claude/settings.json` 中设置它可对所有项目启用沙箱。

* **作用域**：[`Any file`](#scopes)，[对项目设置和本地设置有限制](https://code.claude.com/docs/zh-CN/sandboxing.md#repository-settings-under-an-admin-required-sandbox)
* **类型**：布尔值
  * `true`：Claude Code 对 Bash 命令进行沙箱隔离
  * `false`：Bash 命令在沙箱外运行
* **默认值**：`false`

```json settings.json theme={null}
{
  "sandbox": {
    "enabled": true
  }
}
```

在 Linux 和 WSL2 上，沙箱需要 `bubblewrap` 和 `socat`；请参阅[设置 Linux 和 WSL2](https://code.claude.com/docs/zh-CN/sandboxing.md#set-up-linux-and-wsl2)。当沙箱无法启动时，Claude Code 会在沙箱外运行命令，除非您还设置了 [`failIfUnavailable`](#sandbox-failifunavailable)。

### `sandbox.failIfUnavailable`

当 `sandbox.enabled` 为 `true` 但由于缺少依赖或平台不受支持而导致沙箱无法启动时，让 Claude Code 在启动时报错退出。没有此键时，Claude Code 会在沙箱外运行命令。将沙箱隔离作为安全门槛的托管部署可以使用此设置。

在沙箱不支持的平台上，启用此键后 Claude Code 将无法启动。请参阅[使用托管设置强制沙箱隔离](https://code.claude.com/docs/zh-CN/sandboxing.md#enforce-sandboxing-with-managed-settings)。

* **作用域**：[`Any file`](#scopes)，[对项目设置和本地设置有限制](https://code.claude.com/docs/zh-CN/sandboxing.md#repository-settings-under-an-admin-required-sandbox)
* **类型**：布尔值
  * `true`：当 `sandbox.enabled` 为 `true` 但沙箱无法启动时，Claude Code 在启动时报错退出。如果沙箱通过了该检查但随后初始化失败，会话将继续运行，而 Claude Code 会[拒绝每个沙箱命令](https://code.claude.com/docs/zh-CN/sandboxing.md#commands-refused-with-sandbox-is-required)
  * `false`：当沙箱无法启动时，Claude Code 在沙箱外运行命令
* **默认值**：`false`

以下配置为托管计算机启用沙箱，并将 `failIfUnavailable` 设置为 `true`：

```json managed-settings.json theme={null}
{
  "sandbox": {
    "enabled": true,
    "failIfUnavailable": true
  }
}
```

请参阅[使用托管设置强制沙箱隔离](https://code.claude.com/docs/zh-CN/sandboxing.md#enforce-sandboxing-with-managed-settings)。

### `sandbox.autoAllowBashIfSandboxed`

让 Claude Code 在没有权限提示的情况下运行沙箱化的 Bash 命令。无法在沙箱中运行的命令仍会走常规权限流程，`deny` 规则和限定内容的 `ask` 规则（例如 `Bash(git push *)`）仍然适用；单独的 `Bash` ask 规则对沙箱化命令会被跳过。将其设置为 `false` 可让沙箱化命令也走常规权限流程，`/sandbox` 的 **Mode** 选项卡将此称为常规权限模式。

* **作用域**：[`Any file`](#scopes)
* **类型**：布尔值
  * `true`：Claude Code 在没有权限提示的情况下运行沙箱化的 Bash 命令，[自动允许模式](https://code.claude.com/docs/zh-CN/sandboxing.md#auto-allow-mode)中描述的情况除外；`CLAUDE_CODE_SUBPROCESS_ENV_SCRUB` 会关闭自动允许
  * `false`：沙箱化命令走常规权限流程，由您的允许规则和权限模式决定。`/sandbox` 的 **Mode** 选项卡将此称为常规权限模式
* **默认值**：`true`

以下配置保持沙箱启用，并让沙箱化命令走常规权限流程：

```json settings.json theme={null}
{
  "sandbox": {
    "enabled": true,
    "autoAllowBashIfSandboxed": false
  }
}
```

有关自动允许模式仍会提示的情况以及它在计划模式下的行为，请参阅[自动允许模式](https://code.claude.com/docs/zh-CN/sandboxing.md#auto-allow-mode)。

### `sandbox.excludedCommands`

指定 Claude Code 在沙箱外运行的命令，例如在沙箱下无法工作的工具。每个条目使用与 `Bash(...)` [权限规则](https://code.claude.com/docs/zh-CN/permissions.md#permission-rule-syntax)内容相同的语法：精确命令、前缀（如 `docker *`）或通配符模式。不含通配符的模式为精确匹配，因此 `docker` 只匹配不带参数的 `docker`。

只有当您的条目覆盖某个 Bash 调用中的每条命令时，该调用才会被移出沙箱，而且即便如此，某些调用形式仍会保持沙箱化。仅有 `docker *` 条目并不会将 `npm ci && docker build .` 移出沙箱。

* **作用域**：[`Any file`](#scopes)，[对项目设置和本地设置有限制](https://code.claude.com/docs/zh-CN/sandboxing.md#repository-settings-under-an-admin-required-sandbox)
* **类型**：命令模式数组
* **默认值**：未设置，因此不排除任何命令

```json settings.json theme={null}
{
  "sandbox": {
    "excludedCommands": ["docker *"]
  }
}
```

当 Bash 调用具有以下形式之一（但不限于这些）时，Claude Code 会使其保持沙箱化：

* 以 `sudo`、`eval` 或 `xargs` 开头的命令
* `cd`、`pushd` 或 `popd`，无论出现在调用中的哪个位置
* 命令替换、子 shell，或 `if`、`for` 等控制流块
* 重定向，例如 `docker build . > build.log` 或 `<<EOF` heredoc，仅复制文件描述符的重定向（如 `2>&1`）除外
* 命令名来自变量
* 带有绝对路径、以 `~` 开头或包含 `..` 段的路径参数的 `git clone`、`git init`、`git worktree add`、`git worktree move` 或 `git bundle create`

例如，在 `docker *` 条目下，`cd build && docker compose up` 仍保持沙箱化，添加 `cd` 条目也不会改变这一点。在 `git *` 条目下，`git clone <url> vendor/lib` 在沙箱外运行，但 `git clone <url> ~/tools` 保持沙箱化。克隆会在其目标路径指向的任何位置写入整个文件树，其中可能包含可执行文件。

被排除的命令仍会走常规权限流程。排除只是一种便利措施，而不是安全边界：当某个工具只需要写入特定位置时，[`filesystem.allowWrite`](#sandbox-filesystem-allowwrite) 可以让它保持沙箱化。

来自会话加载的各设置作用域的条目会合并为一个列表，除非沙箱是[管理员强制要求的](https://code.claude.com/docs/zh-CN/sandboxing.md#repository-settings-under-an-admin-required-sandbox)。在这种情况下，Claude Code 会忽略 `.claude/settings.json` 和 `.claude/settings.local.json` 中的条目，因此克隆的仓库无法将命令移出沙箱。托管设置、`--settings` 和您的 `~/.claude/settings.json` 中的条目仍然适用，并且没有仅限托管设置的锁定覆盖此列表。

### `sandbox.allowUnsandboxedCommands`

允许 Claude 在沙箱阻止某条命令后，使用 `dangerouslyDisableSandbox` 参数在沙箱外重试该命令。当其为 `false` 时，Claude Code 会忽略该参数。此时在沙箱运行期间，Claude 运行的命令都会被沙箱化，除非它们匹配 [`excludedCommands`](#sandbox-excludedcommands) 条目。`/sandbox` 的 **Overrides** 选项卡将该状态显示为 **Strict sandbox mode**。托管设置中的 `false` 会为其覆盖的开发者启用严格沙箱模式。

* **作用域**：[`Any file`](#scopes)，[对项目设置和本地设置有一项限制](https://code.claude.com/docs/zh-CN/sandboxing.md#turn-off-the-retry-with-strict-sandbox-mode)
* **类型**：布尔值
  * `true`：沙箱阻止某条命令后，Claude 可以使用 `dangerouslyDisableSandbox` 参数在沙箱外重试该命令
  * `false`：Claude Code 忽略该参数，因此在沙箱运行期间，Claude 运行的命令都会被沙箱化，除非它们匹配 `excludedCommands` 条目
* **默认值**：`true`

以下配置为托管设置覆盖的所有人强制启用严格沙箱模式：

```json managed-settings.json theme={null}
{
  "sandbox": {
    "enabled": true,
    "allowUnsandboxedCommands": false
  }
}
```

来自托管设置或 `--settings` 的 `false` 还会使沙箱成为[管理员强制要求的](https://code.claude.com/docs/zh-CN/sandboxing.md#repository-settings-under-an-admin-required-sandbox)。用户设置中的 `false` 可以抵御项目中的 `true`，但不会使沙箱成为管理员强制要求的。抵御项目的值需要 Claude Code v2.1.285 或更高版本。

由谁批准沙箱外重试取决于您的权限模式和允许规则。请参阅[沙箱外重试这一逃生通道](https://code.claude.com/docs/zh-CN/sandboxing.md#the-unsandboxed-retry-escape-hatch)。

要了解您自己在 [`!` shell 模式提示符](https://code.claude.com/docs/zh-CN/interactive-mode#shell-mode-with-prefix)中输入的命令何时会被沙箱化运行，请参阅[严格沙箱模式](https://code.claude.com/docs/zh-CN/sandboxing.md#turn-off-the-retry-with-strict-sandbox-mode)。

### `sandbox.filesystem`

控制沙箱化命令可以读取和写入哪些路径。默认情况下，它们可以写入工作目录、每用户临时目录，以及您通过 `--add-dir`、`/add-dir` 或 `permissions.additionalDirectories` 添加的目录，并可以读取文件系统的其余部分，包括凭据文件。使用四个路径列表来放宽或收窄这些范围，或使用 `disabled` 关闭文件系统层。有关默认边界，请参阅[文件系统隔离](https://code.claude.com/docs/zh-CN/sandboxing.md#filesystem-isolation)。

* **作用域**：[`Any file`](#scopes)
* **类型**：包含 `allowWrite`、`denyWrite`、`denyRead` 和 `allowRead` 数组，以及 `allowManagedReadPathsOnly` 和 `disabled` 布尔值的对象
* **默认值**：未设置，因此适用默认的读写边界

以下配置允许沙箱化命令写入构建目录和您的 kubeconfig，并隐藏您的 AWS 凭据文件：

```json settings.json theme={null}
{
  "sandbox": {
    "filesystem": {
      "allowWrite": ["/tmp/build", "~/.kube"],
      "denyRead": ["~/.aws/credentials"]
    }
  }
}
```

Claude Code 在操作系统沙箱边界强制执行这些列表，因此它们适用于沙箱化命令启动的每个子进程，例如 `kubectl`、`terraform` 或 `npm`。Claude Code 会将您的[权限规则](https://code.claude.com/docs/zh-CN/sandboxing.md#permission-rules)添加到相同的列表中：`Edit` 允许和拒绝规则添加到 `allowWrite` 和 `denyWrite`，`Read` 拒绝规则添加到 `denyRead`，`WebFetch(domain:...)` 允许和拒绝规则添加到 [`network`](#sandbox-network) 域名列表。

除非有锁定适用，否则 Claude Code 会在会话加载的各设置文件之间合并这些列表。[`allowManagedReadPathsOnly`](#sandbox-filesystem-allowmanagedreadpathsonly) 将 `allowRead` 限制为来自托管设置的条目，[`allowManagedDomainsOnly`](#sandbox-network-allowmanageddomainsonly) 对允许的域名执行同样的限制。[仓库锁定](https://code.claude.com/docs/zh-CN/sandboxing.md#repository-settings-under-an-admin-required-sandbox)会排除来自仓库设置文件的条目。

[配置沙箱隔离](https://code.claude.com/docs/zh-CN/sandboxing.md#configure-sandboxing)介绍了您通过 `--setting-sources` 排除的来源。当您在会话期间编辑列表时，Claude Code 会[将更改应用于正在运行的会话](https://code.claude.com/docs/zh-CN/settings.md#when-edits-take-effect)。

#### 沙箱路径前缀

`allowWrite`、`denyWrite`、`denyRead`、`allowRead` 和 [`credentials.files`](#sandbox-credentials-files) 中的路径按其前缀解析：

| 前缀 | 含义 | 示例 |
| :- | :- | :- |
| `/` | 从文件系统根目录开始的绝对路径 | `/tmp/build` 保持为 `/tmp/build` |
| `~/` | 相对于主目录 | `~/.kube` 变为 `$HOME/.kube` |
| `./` 或无前缀 | 在项目设置中相对于项目根目录，在用户设置中相对于 `~/.claude` | `.claude/settings.json` 中的 `./output` 解析为 `<project-root>/output` |

用于绝对路径的 `//path` 前缀也可以使用。如果您使用单斜杠 `/path` 并期望按项目相对路径解析，请改用 `./path`。此语法与 [Read 和 Edit 权限规则](https://code.claude.com/docs/zh-CN/permissions.md#read-and-edit)不同，后者使用 `//path` 表示绝对路径，使用 `/path` 表示项目相对路径：沙箱文件系统路径使用标准约定，因此 `/tmp/build` 是绝对路径。

Claude Code 会去除目录路径末尾的斜杠，因此 `~/.aws` 和 `~/.aws/` 匹配同一目录。在 v2.1.224 之前，Claude Code 会将末尾斜杠原样传递给沙箱，对于带末尾斜杠写法的 `denyRead` 或 `denyWrite` 条目，Claude 仍可能读取或写入其下的路径。

##### 通配符在沙箱路径中的适用位置

通配符 `*`、`?` 和 `**` 在 macOS 上适用于全部四个 `sandbox.filesystem` 路径列表，在 Linux 和 WSL2 上则适用的位置较少：

| `sandbox.filesystem` 列表 | 在 Linux 和 WSL2 上有效的写法 | 在 Linux 和 WSL2 上无效的示例 | 替代写法 |
| :- | :- | :- | :- |
| `denyRead`、`allowRead` | 末尾的 `/**` 有效，因此 `~/build/**` 与 `~/build` 覆盖同一目录。其他任何通配符只有出现在条目[完整路径](#sandbox-path-prefixes)的第一个目录名之后才有效。 | `denyRead` 中的 `/**/*.pem` 不覆盖任何内容，因此不会阻止任何读取 | `/srv/**/*.pem`，每个顶层目录各写一个条目 |
| `allowWrite`、`denyWrite` | 只有末尾的 `/**` 有效，因此 `~/build/**` 与 `~/build` 覆盖同一目录 | `./src/*` 只覆盖 `src` 中本身名为 `*` 的文件或目录，若不存在则不覆盖任何内容 | `./src`，它覆盖 `src` 下的所有内容，或者每个路径各写一个条目 |

在 `~/keys/*.pem` 中，通配符位于第一个目录名之后；对于主目录为 `/home/alice` 的情况，第一个目录名是 `home`：

```text theme={null}
~/keys/*.pem              the entry
/home/alice/keys/*.pem    its full path
 ^^^^                     the first directory name, home
```

在 Linux 和 WSL2 上，会话中 `/sandbox` 的 **Config** 选项卡会针对包含末尾 `/**` 以外通配符的 `Edit` 权限规则发出警告。这些通配符对沙箱化命令无效。请将该规则应覆盖的每个路径或包含这些路径的目录添加到 `allowWrite`（对于允许规则）或 `denyWrite`（对于拒绝规则）。

##### 包含方括号、花括号或圆括号的目录名

要允许或阻止名称中包含方括号的目录，请按原样写出名称，例如 `./app/[slug]`。在该条目中，`[slug]` 是目录的真实名称，包括方括号：

```text theme={null}
app/
└── [slug]/
    └── page.tsx
```

该条目覆盖该目录下的所有内容，因此不要添加末尾的 `/**`。在 macOS 以及 Linux 和 WSL2 上，`app/[slug]` 目录必须在沙箱化命令启动时已存在。

* **方括号，例如 `[slug]`**：在 macOS 的所有列表中，以及 Linux 和 WSL2 的 `denyRead` 和 `allowRead` 中，`./app/[slug]` 中的方括号同时也是一个模式。该条目还会匹配 `app` 下以 `slug` 中某一个字符命名的任何路径，例如 `app/s` 或 `app/g`。如果该条目位于 `allowRead` 中，或在 macOS 上位于 `allowWrite` 中，沙箱化命令也可以访问这些路径。请重命名任何不应被它们访问的路径。
* **方括号，例如 `[user-id]` 或 `[post-id]`，其中连字符左侧字符的排序位于右侧字符之后**：在所有平台的所有列表中，`./app/[user-id]` 只覆盖恰好具有该名称的目录。在 v2.1.304 之前，当 `denyRead` 或 `allowRead` 包含此类条目时，Linux 和 WSL2 上的沙箱化命令无法启动。
* **花括号和圆括号**：它们是普通字符，因此 `~/data/{keys,tokens}` 覆盖的是一个名为 `{keys,tokens}` 的目录，而不是两个备选项。要覆盖 `~/data/keys` 和 `~/data/tokens`，请将它们分别添加为单独的条目。

### `sandbox.filesystem.allowWrite`

在工作目录、每用户临时目录以及您通过 `--add-dir`、`/add-dir` 或 `permissions.additionalDirectories` 添加的目录之外，添加沙箱化命令可以写入的路径。当 `kubectl` 或构建工具等子进程需要在项目外写入时使用此设置。

* **作用域**：[`Any file`](#scopes)，[对项目设置和本地设置有限制](https://code.claude.com/docs/zh-CN/sandboxing.md#repository-settings-under-an-admin-required-sandbox)
* **类型**：路径字符串数组，使用[沙箱路径前缀](#sandbox-path-prefixes)
* **默认值**：未设置，因此沙箱化命令可以写入工作目录、每用户临时目录、您通过 `--add-dir` 或 `/add-dir` 添加的目录，以及 [`permissions.additionalDirectories`](#permissions-additionaldirectories) 中的目录

以下配置允许构建写入 `/tmp/build` 下的内容，并允许 `kubectl` 更新您的 kubeconfig：

```json settings.json theme={null}
{
  "sandbox": {
    "filesystem": {
      "allowWrite": ["/tmp/build", "~/.kube"]
    }
  }
}
```

Claude Code 会在会话加载的各设置作用域之间合并 `allowWrite` 条目以及来自您的 `Edit(...)` 允许权限规则的路径，当 [`permissions.blockReadsOutsideWorkingDirectories`](#sandboxed-commands-under-the-block) 启用时，会排除来自仓库设置的条目。[仓库锁定](https://code.claude.com/docs/zh-CN/sandboxing.md#repository-settings-under-an-admin-required-sandbox)也可能排除仓库的条目。`allowWrite` 条目无法解除[受保护路径](https://code.claude.com/docs/zh-CN/sandboxing.md#protected-paths)的保护。

### `sandbox.filesystem.denyWrite`

阻止沙箱化命令写入特定路径，包括原本可写目录内的路径。

* **作用域**：[`Any file`](#scopes)
* **类型**：路径字符串数组，使用[沙箱路径前缀](#sandbox-path-prefixes)
* **默认值**：未设置

以下配置阻止沙箱化命令更改系统配置或安装二进制文件：

```json settings.json theme={null}
{
  "sandbox": {
    "filesystem": {
      "denyWrite": ["/etc", "/usr/local/bin"]
    }
  }
}
```

Claude Code 会在会话加载的所有设置作用域之间合并条目，并添加来自您的 `Edit(...)` 拒绝权限规则的路径。

### `sandbox.filesystem.denyRead`

阻止沙箱化命令读取特定路径，例如默认读取策略原本会暴露的凭据文件。要保护凭据文件并使其仍可通过沙箱代理使用，请改用 [`sandbox.credentials`](#sandbox-credentials)。

* **作用域**：[`Any file`](#scopes)
* **类型**：路径字符串数组，使用[沙箱路径前缀](#sandbox-path-prefixes)
* **默认值**：未设置，因此沙箱化命令保留[默认读取权限](https://code.claude.com/docs/zh-CN/sandboxing.md#filesystem-isolation)，其中包括 `~/.aws/credentials` 等凭据文件

```json settings.json theme={null}
{
  "sandbox": {
    "filesystem": {
      "denyRead": ["~/.aws/credentials"]
    }
  }
}
```

Claude Code 会在会话加载的所有设置作用域之间合并条目，并添加来自您的 `Read(...)` 拒绝权限规则的路径。当 [`filesystem.disabled`](#sandbox-filesystem-disabled) 为 `true` 时，Claude Code 不会强制执行这些条目。

### `sandbox.filesystem.allowRead`

在 [`denyRead`](#sandbox-filesystem-denyread) 阻止的区域内重新开放特定路径的读取权限，以构建仅限工作区的读取访问。精确或通配符 `denyRead` 条目在更宽泛的 `allowRead` 中仍保持阻止，如[重叠表](https://code.claude.com/docs/zh-CN/sandboxing.md#configure-sandboxing)所示。当 `~/**/.env` 等通配符 `denyRead` 条目匹配某个目录时，Claude Code 也会阻止读取其内容。在 v2.1.236 之前的 macOS 上，只要更宽泛的 `allowRead` 条目覆盖了通配符 `denyRead` 条目匹配的路径，Claude Code 就会重新开放这些路径，并使被匹配目录的内容保持可读。

* **作用域**：[`Any file`](#scopes)，[对项目设置和本地设置有限制](https://code.claude.com/docs/zh-CN/sandboxing.md#repository-settings-under-an-admin-required-sandbox)
* **类型**：路径字符串数组，使用[沙箱路径前缀](#sandbox-path-prefixes)
* **默认值**：未设置

以下配置阻止读取您的主目录，但项目本身除外：

```json settings.json theme={null}
{
  "sandbox": {
    "filesystem": {
      "denyRead": ["~/"],
      "allowRead": ["."]
    }
  }
}
```

Claude Code 在项目设置中将 `.` 条目解析为项目根目录，在用户设置中解析为 `~/.claude`。除非设置了 [`allowManagedReadPathsOnly`](#sandbox-filesystem-allowmanagedreadpathsonly)，否则 Claude Code 会在会话加载的各设置文件之间合并条目；当 [`permissions.blockReadsOutsideWorkingDirectories`](#sandboxed-commands-under-the-block) 启用时，会排除来自仓库设置的条目。[仓库锁定](https://code.claude.com/docs/zh-CN/sandboxing.md#repository-settings-under-an-admin-required-sandbox)也可能排除仓库的条目。

### `sandbox.filesystem.allowManagedReadPathsOnly`

仅遵循来自托管设置的 [`allowRead`](#sandbox-filesystem-allowread) 条目，使开发者无法重新开放组织已阻止路径的读取权限。Claude Code 仍会合并来自会话加载的所有设置作用域的 `denyRead` 条目。

* **作用域**：[`Managed`](#scopes)
* **类型**：布尔值
  * `true`：Claude Code 仅遵循来自托管设置的 `allowRead` 条目
  * `false`：来自其他设置文件的 `allowRead` 条目可以合并进来
* **默认值**：`false`

以下配置阻止读取主目录，重新开放 `~/work`，并阻止开发者重新开放其他任何内容：

```json managed-settings.json theme={null}
{
  "sandbox": {
    "filesystem": {
      "denyRead": ["~/"],
      "allowRead": ["~/work"],
      "allowManagedReadPathsOnly": true
    }
  }
}
```

请参阅[防止开发者放宽策略](https://code.claude.com/docs/zh-CN/sandboxing.md#keep-developers-from-widening-the-policy)。

### `sandbox.filesystem.disabled`

跳过文件系统隔离，同时保留网络隔离。沙箱化命令可以不受限制地读写主机文件系统，而其网络出站流量仍限制在 [`network.allowedDomains`](#sandbox-network-alloweddomains) 范围内。当您使用沙箱是为了控制命令连接到哪里而非写入什么时使用此设置。

* **作用域**：[`User or managed`](#scopes)。当托管设置配置了任何 `sandbox.filesystem` 内容，或列出了带有 `"mode": "deny"` 的 `sandbox.credentials.files` 条目时，只有托管设置可以设置它。
* **类型**：布尔值
  * `true`：Claude Code 跳过文件系统隔离并保留网络隔离
  * `false`：文件系统隔离保持启用
* **默认值**：`false`，因此文件系统隔离保持启用

以下配置使文件系统保持开放，并将网络出站流量限制为 GitHub 和 npm：

```json settings.json theme={null}
{
  "sandbox": {
    "enabled": true,
    "filesystem": {
      "disabled": true
    },
    "network": {
      "allowedDomains": ["github.com", "*.npmjs.org"]
    }
  }
}
```

关闭该层后，Claude Code 不会强制执行 `denyRead` 或 `credentials.files` 的 `deny` 条目，而 `credentials.envVars` 条目和已应用的 `mask` 条目仍然有效。[`autoAllowBashIfSandboxed`](#sandbox-autoallowbashifsandboxed) 仍默认为 `true`，因此请将其设置为 `false` 以继续显示提示。有关可以设置它的完整来源列表以及关闭隔离后的变化，请参阅[禁用文件系统隔离](https://code.claude.com/docs/zh-CN/sandboxing.md#disable-filesystem-isolation)。

### `sandbox.ignoreViolations`

对于您预期某条命令会探测并被拒绝的路径（例如某个在启动时检查 `/etc/hosts` 的工具），静默其沙箱违规报告，使这些拒绝不会显示为违规，也不会出现在 Claude 看到的内容中。沙箱仍会阻止该访问；只是报告被抑制。键是用于匹配命令的子字符串，`*` 匹配所有命令；值是该命令要忽略的违规子字符串，例如文件系统路径。

* **作用域**：[`Any file`](#scopes)，[对项目设置和本地设置有限制](https://code.claude.com/docs/zh-CN/sandboxing.md#repository-settings-under-an-admin-required-sandbox)
* **类型**：将命令子字符串映射到违规子字符串（通常是路径）数组的对象
* **默认值**：未设置，因此报告所有违规

```json settings.json theme={null}
{
  "sandbox": {
    "ignoreViolations": {
      "*": ["/etc/hosts"]
    }
  }
}
```

### `sandbox.enableWeakerNestedSandbox`

在非特权 Docker 容器内运行 Linux 沙箱，在这种容器中 bubblewrap 无法挂载新的 `/proc`。此时内部沙箱会绑定挂载容器现有的 `/proc`，这会暴露新挂载本应隐藏的进程信息。这会降低安全性；仅当外部容器已提供您所需的隔离时才使用。

* **作用域**：[`Any file`](#scopes)，[对项目设置和本地设置有限制](https://code.claude.com/docs/zh-CN/sandboxing.md#repository-settings-under-an-admin-required-sandbox)
* **类型**：布尔值
  * `true`：内部沙箱绑定挂载容器现有的 `/proc`，而不是挂载新的
  * `false`：沙箱挂载新的 `/proc`，这在非特权 Docker 容器中无法工作
* **默认值**：`false`

```json settings.json theme={null}
{
  "sandbox": {
    "enabled": true,
    "enableWeakerNestedSandbox": true
  }
}
```

仅限 Linux 和 WSL2。请参阅[Bubblewrap 在容器内无法启动](https://code.claude.com/docs/zh-CN/sandboxing.md#bubblewrap-fails-to-start-inside-a-container)。

### `sandbox.enableWeakerNetworkIsolation`

允许 macOS 上的沙箱化命令访问系统 TLS 信任服务 `com.apple.trustd.agent`。当您将 [`network.httpProxyPort`](#sandbox-network-httpproxyport) 与 MITM 代理和自定义 CA 一起使用时，`gh`、`gcloud` 和 `terraform` 等基于 Go 的工具需要它来验证 TLS 证书。这会通过信任服务打开一条潜在的数据外泄路径，从而降低安全性。

* **作用域**：[`Any file`](#scopes)，[对项目设置和本地设置有限制](https://code.claude.com/docs/zh-CN/sandboxing.md#repository-settings-under-an-admin-required-sandbox)
* **类型**：布尔值
  * `true`：macOS 上的沙箱化命令可以访问 `com.apple.trustd.agent`
  * `false`：macOS 上的沙箱化命令无法访问系统 TLS 信任服务
* **默认值**：`false`

```json settings.json theme={null}
{
  "sandbox": {
    "enabled": true,
    "enableWeakerNetworkIsolation": true
  }
}
```

如果您不使用 MITM 代理，请改为在 [`excludedCommands`](#sandbox-excludedcommands) 中列出失败的工具；请参阅[基于 Go 的 CLI 在 macOS 上 TLS 验证失败](https://code.claude.com/docs/zh-CN/sandboxing.md#go-based-clis-fail-tls-verification-on-macos)。

### `sandbox.allowAppleEvents`

允许 macOS 上的沙箱化命令发送 Apple Events，`open`、`osascript` 以及在浏览器中打开 URL 的工具都需要它；没有它时，这些命令会以错误 `-600` 失败。这会移除代码执行隔离：沙箱化命令可以在没有用户提示的情况下在沙箱外启动其他应用程序，并可以向正在运行的应用程序（如 Terminal）发送 AppleScript 命令，但需经过 macOS 针对每个应用的自动化授权提示（TCC）。

* **作用域**：[`User or managed`](#scopes)
* **类型**：布尔值
  * `true`：macOS 上的沙箱化命令可以发送 Apple Events
  * `false`：macOS 上的沙箱化命令无法发送 Apple Events，因此 `open` 和 `osascript` 会以错误 `-600` 失败
* **默认值**：`false`

```json settings.json theme={null}
{
  "sandbox": {
    "enabled": true,
    "allowAppleEvents": true
  }
}
```

要保留隔离并仍运行某个此类工具，请改为将其添加到 [`excludedCommands`](#sandbox-excludedcommands)。请参阅 [macOS 上的 Apple Events](https://code.claude.com/docs/zh-CN/sandboxing.md#security-limitations)。

### `sandbox.allowNestedUserNamespaces`

允许 Linux 和 WSL2 上的沙箱化命令创建用户命名空间，Chromium 和 Electron 自身的沙箱、嵌套的 bubblewrap 以及 `unshare` 都需要它。如果将此键设置为 `true`，您就关闭了一道安全边界：来自沙箱化命令的代码可能最终在沙箱外运行。对于 rootless Podman 或 Buildah，请改用 [`excludedCommands`](#sandbox-excludedcommands)，因为沙箱还会以其他方式阻止它们。需要 Claude Code v2.1.305 或更高版本。

* **作用域**：[`User or managed`](#scopes)。来自 `~/.claude/settings.json`、通过 `--settings` 传入的 JSON 或[托管设置](https://code.claude.com/docs/zh-CN/managed-settings.md)的 `true` 有效。Claude Code 会忽略项目 `.claude/settings.json` 或 `.claude/settings.local.json` 中的 `true`。在 [`CLAUDE_CODE_SUBPROCESS_ENV_SCRUB`](https://code.claude.com/docs/zh-CN/env-vars#variables) 设置为 `1` 的会话中，Claude Code 会忽略来自所有来源的 `true`。来自任何文件的 `false` 都有效，按[设置优先级](https://code.claude.com/docs/zh-CN/settings.md#settings-precedence)处理。
* **类型**：布尔值
  * `true`：沙箱化命令可以创建用户命名空间
  * `false`：此键不额外允许任何内容，适用沙箱的默认行为
* **默认值**：`false`

```json settings.json theme={null}
{
  "sandbox": {
    "enabled": true,
    "allowNestedUserNamespaces": true
  }
}
```

启用此键后，不要依赖沙箱的任何限制（包括[受保护路径](https://code.claude.com/docs/zh-CN/sandboxing.md#protected-paths)和 [`credentials.files`](#sandbox-credentials-files) 条目）来阻止恶意命令。在[自动允许模式](https://code.claude.com/docs/zh-CN/sandboxing.md#auto-allow-mode)下，该命令可以在没有权限提示的情况下运行。如果您的组织有安全团队，请在启用此键之前与他们确认。

要找到适用于您工具的解决方法，请参阅[Chromium 或 rootless Podman 在 Linux 沙箱内失败](https://code.claude.com/docs/zh-CN/sandboxing.md#chromium-or-rootless-podman-fails-inside-the-sandbox-on-linux)。

### `sandbox.ripgrep`

让沙箱使用您自己的 ripgrep 二进制文件，而不是 Claude Code 使用的那个，例如当您的平台需要以不同方式构建的 `rg` 时。

* **作用域**：[`User or managed`](#scopes)
* **类型**：包含 `command`（ripgrep 二进制文件的路径）和可选 `args`（要前置的参数数组）的对象
* **默认值**：未设置，因此沙箱使用与 Claude Code 相同的 ripgrep 二进制文件。除非您将 [`USE_BUILTIN_RIPGREP`](https://code.claude.com/docs/zh-CN/env-vars) 设置为 `0`，否则使用的是内置二进制文件

```json settings.json theme={null}
{
  "sandbox": {
    "ripgrep": {
      "command": "/usr/local/bin/rg"
    }
  }
}
```

### `sandbox.bwrapPath`

让沙箱使用安装在 `PATH` 之外的 bubblewrap 二进制文件，例如隔离网络主机上的内置副本。Claude Code 在启动时的依赖检查以及包装每条沙箱化命令时都会使用此路径。

* **作用域**：[`Managed`](#scopes)。Claude Code 仅从托管设置中读取它，以防用户、项目或本地文件将沙箱指向其他二进制文件。
* **类型**：字符串，一个绝对路径；Claude Code 会丢弃相对路径并回退到 `PATH` 查找
* **默认值**：未设置，因此 Claude Code 在 `PATH` 上查找 `bwrap`

```json managed-settings.json theme={null}
{
  "sandbox": {
    "enabled": true,
    "bwrapPath": "/opt/admin/bwrap"
  }
}
```

仅限 Linux 和 WSL2。

### `sandbox.socatPath`

让沙箱网络代理使用安装在 `PATH` 之外的 `socat` 二进制文件。

* **作用域**：[`Managed`](#scopes)
* **类型**：字符串，一个绝对路径；Claude Code 会丢弃相对路径并回退到 `PATH` 查找
* **默认值**：未设置，因此 Claude Code 在 `PATH` 上查找 `socat`

```json managed-settings.json theme={null}
{
  "sandbox": {
    "enabled": true,
    "socatPath": "/opt/admin/socat"
  }
}
```

仅限 Linux 和 WSL2。

### `sandbox.credentials`

声明要[对沙箱化命令进行保护](https://code.claude.com/docs/zh-CN/sandboxing.md#protect-credentials)的凭据文件和环境变量。每个条目指定一个文件 `path` 或变量 `name` 以及一个 `mode`：`deny` 在沙箱内隐藏凭据，`mask` 向沙箱化命令显示一个占位符，同时由[沙箱代理](https://code.claude.com/docs/zh-CN/sandboxing.md#mask-credentials)在出站请求中替换为真实值。Claude Code 仅保护您列出的条目；没有内置的凭据拒绝列表。

* **作用域**：[`Any file`](#scopes)。Claude Code 仅从用户设置、托管设置和 `--settings` 标志中遵循 `mask` 条目、`allowPlaintextInject`、`awsPairs` 和 `sigv4`。
* **类型**：包含 `files`、`envVars`、`allowPlaintextInject`、`awsPairs` 和 `sigv4` 的对象
* **默认值**：未设置，因此不保护任何凭据

以下配置隐藏您的 AWS 凭据文件，并从沙箱化命令中移除 `GITHUB_TOKEN`：

```json settings.json theme={null}
{
  "sandbox": {
    "credentials": {
      "files": [{ "path": "~/.aws/credentials", "mode": "deny" }],
      "envVars": [{ "name": "GITHUB_TOKEN", "mode": "deny" }]
    }
  }
}
```

`deny` 文件保护属于文件系统层，因此当您[禁用文件系统隔离](https://code.claude.com/docs/zh-CN/sandboxing.md#disable-filesystem-isolation)时它不适用；环境变量保护仍然适用。

#### 托管设置中的无效凭据条目

当托管的 `sandbox.credentials` 条目验证失败时，Claude Code 会尽可能继续保护该凭据：

* `files` 或 `envVars` 中仍具有有效 `path` 或 `name` 以及 `mask` 或 `deny` 的 `mode` 的条目（例如其 `extract` 模式没有捕获组的条目）会被降级为 `mode: "deny"` 并附带警告，因此在您修复该条目之前，凭据保持被阻止而不是被掩码。降级后的 `files` 条目会像显式 `deny` 条目一样锁定 [`filesystem.disabled`](https://code.claude.com/docs/zh-CN/sandboxing#disable-filesystem-isolation)，并且警告会注明：如果托管设置关闭了文件系统隔离，则其读取阻止不会被强制执行。
* 具有未知 `mode` 或无效 `path`、`name` 的条目会被移除。
* 每种情况都会发出警告；无论条目被降级还是移除，其余有效条目仍会被强制执行。

适用于 v2.1.191 及更高版本；在 v2.1.221 之前，每个无效条目都会被移除。有关其他按字段处理的托管键，请参阅[托管设置中的无效条目](https://code.claude.com/docs/zh-CN/managed-settings.md#invalid-entries-in-managed-settings)。

### `sandbox.credentials.files`

保护凭据文件或目录免受沙箱化命令访问。使用 `"mode": "deny"` 时，Claude Code 会在沙箱内阻止读取该路径，与 [`sandbox.filesystem.denyRead`](#sandbox-filesystem-denyread) 的读取阻止相同。使用 `"mode": "mask"` 时，Linux 和 WSL2 上的沙箱化命令读取的是该文件的哨兵副本，沙箱代理会在发往该条目 `injectHosts` 的出站请求中替换为真实值；在 macOS 上，该文件在沙箱内则不可读。`"mode": "mask"` 需要 Claude Code v2.1.221 或更高版本。

* **作用域**：[`Any file`](#scopes)。Claude Code 会丢弃来自项目 `.claude/settings.json` 和本地 `.claude/settings.local.json` 的 `mask` 条目。
* **类型**：对象数组，每个对象包含 `path` 和值为 `"deny"` 或 `"mask"` 的 `mode`，外加可选的[文件掩码字段](#mask-fields-for-files)
* **默认值**：未设置，因此不保护任何凭据文件

以下配置隐藏您的 AWS 凭据文件并掩码 `gh` 主机文件，仅在发往 `api.github.com` 的请求中替换为真实值：

```json settings.json theme={null}
{
  "sandbox": {
    "credentials": {
      "files": [
        { "path": "~/.aws/credentials", "mode": "deny" },
        { "path": "~/.config/gh/hosts.yml", "mode": "mask", "injectHosts": ["api.github.com"] }
      ]
    }
  }
}
```

路径使用与 `sandbox.filesystem.*` 设置相同的[前缀](#sandbox-path-prefixes)，Claude Code 会合并来自会话加载的所有设置作用域的数组。[保护凭据](https://code.claude.com/docs/zh-CN/sandboxing.md#protect-credentials)介绍了您通过 `--setting-sources` 排除的来源中仍然适用的内容。`mask` 条目需要 Claude Code v2.1.221 或更高版本。

`mask` 替换仅通过沙箱代理进行，因此请设置 [`sandbox.network.tlsTerminate`](#sandbox-network-tlsterminate)，对于纯 HTTP 测试网络则设置 [`allowPlaintextInject`](#sandbox-credentials-allowplaintextinject)。`mask` 适用于单个文件，因此请逐个列出每个凭据文件。Claude Code 接受但会忽略 `deny` 条目上的 `mask` 字段。[掩码凭据](https://code.claude.com/docs/zh-CN/sandboxing.md#mask-credentials)介绍了会遵循哪些设置来源，[掩码凭据文件](https://code.claude.com/docs/zh-CN/sandboxing.md#mask-credential-files)介绍了条目何时会回退为 `deny`。

#### 文件掩码字段

`mask` 条目接受以下可选字段。没有 `extract` 或 `decode` 时，Claude Code 会将整个文件内容替换为一个哨兵值。在启用了文件系统隔离的 macOS 上，Claude Code 会在 `extract` 或 `decode` 运行之前将 `mask` 条目作为 `deny` 应用；请参阅[掩码凭据文件](https://code.claude.com/docs/zh-CN/sandboxing.md#mask-credential-files)。

| 字段 | 类型 | 作用 |
| :- | :- | :- |
| `extract` | 字符串，至少包含一个捕获组的正则表达式 | 仅掩码每个匹配中第 1 组捕获的文本，使文件其余部分保持可解析。如果同时设置了 `decode`，Claude Code 会将每个捕获检查为可能的 JWT，而不是直接替换。需要 v2.1.221 或更高版本 |
| `onExtractNoMatch` | `"warn"`、`"deny"` 或 `"error"`；默认为 `"warn"` | 当 `extract` 或 `decode` 找不到任何可掩码内容时的处理方式。`warn` 使文件在沙箱内保持原样可读，`deny` 使其不可读，`error` 会停止沙箱设置，直到您修复配置。当读取阻止不会被强制执行时（因为您[禁用了文件系统隔离](https://code.claude.com/docs/zh-CN/sandboxing.md#disable-filesystem-isolation)，或某个 [`sandbox.filesystem.allowRead`](#sandbox-filesystem-allowread) 条目重新开放了该路径），Claude Code 会将 `deny` 视为 `error`。需要 v2.1.221 或更高版本；`decode` 情况需要 v2.1.224 或更高版本 |
| `decode` | 字符串 `"jwt"` | 使用内置模式（或在设置了 `extract` 时使用它）查找文件中的 JSON Web Token（JWT），验证每个候选项，并将其替换为结构有效的伪造令牌，使沙箱内解码该令牌的代码继续正常工作。当没有候选项通过验证时，由 `onExtractNoMatch` 决定结果。需要 v2.1.224 或更高版本 |
| `maskClaims` | 字符串数组，至少包含一个声明名称；需要 `decode` | 仅掩码每个已验证 JWT 中指定的顶层负载声明，并围绕修改后的负载重建令牌，使其他声明保持可读。当没有指定的声明匹配时，由 `onExtractNoMatch` 决定结果。需要 v2.1.224 或更高版本 |
| `maskDuplicates` | 布尔值，默认为 `false` | 同时替换文件中其他位置出现的每个被掩码值的逐字副本，例如粘贴到注释中的密钥。Claude Code 按原始子字符串匹配，因此请仅将其用于较长的高熵密钥。仅在设置了 `extract` 或 `decode` 时才会参考此字段。需要 v2.1.221 或更高版本 |
| `injectHosts` | 字符串数组，每项都是 [`sandbox.network.allowedDomains`](#sandbox-network-alloweddomains) 也允许的主机 | 收窄沙箱代理替换真实值的主机范围。未设置时，代理会在发往 `sandbox.network.allowedDomains` 中每个主机的请求中进行替换。需要 v2.1.221 或更高版本 |

以下配置仅掩码 `gh` 主机文件中的 `oauth_token` 值，替换该值在文件中的所有其他副本，在模式未匹配任何内容时使文件不可读，并仅在发往 `api.github.com` 的请求中替换为真实令牌：

```json settings.json theme={null}
{
  "sandbox": {
    "credentials": {
      "files": [
        {
          "path": "~/.config/gh/hosts.yml",
          "mode": "mask",
          "extract": "oauth_token:\\s*(\\S+)",
          "maskDuplicates": true,
          "onExtractNoMatch": "deny",
          "injectHosts": ["api.github.com"]
        }
      ]
    }
  }
}
```

### `sandbox.credentials.envVars`

保护环境变量免受沙箱化命令访问。使用 `"mode": "deny"` 时，Claude Code 会从沙箱化命令的环境中移除该变量。使用 `"mode": "mask"` 时，沙箱化命令看到的是每会话的哨兵值，沙箱代理会在发往该条目 `injectHosts` 的出站请求中替换为真实值，因此 `gh` 和 `npm` 等工具可以继续进行身份验证，而无需持有真实凭据。

* **作用域**：[`Any file`](#scopes)。Claude Code 会丢弃来自项目 `.claude/settings.json` 和本地 `.claude/settings.local.json` 的 `mask` 条目。
* **类型**：对象数组，每个对象包含 `name` 和值为 `"deny"` 或 `"mask"` 的 `mode`，外加可选的[环境变量掩码字段](#mask-fields-for-environment-variables)
* **默认值**：未设置，因此不保护任何环境变量

以下配置从沙箱化命令中移除 `NPM_TOKEN` 并掩码 `GITHUB_TOKEN`，仅在发往 `api.github.com` 的请求中替换为真实值：

```json settings.json theme={null}
{
  "sandbox": {
    "credentials": {
      "envVars": [
        { "name": "NPM_TOKEN", "mode": "deny" },
        { "name": "GITHUB_TOKEN", "mode": "mask", "injectHosts": ["api.github.com"] }
      ]
    }
  }
}
```

`name` 必须以字母或下划线开头，且只能包含字母、数字和下划线。Claude Code 会合并来自会话加载的所有设置作用域的数组，当同一变量以两种模式出现时应用 `deny`。[保护凭据](https://code.claude.com/docs/zh-CN/sandboxing.md#protect-credentials)介绍了您通过 `--setting-sources` 排除的来源中仍然适用的内容。

`mask` 替换仅通过沙箱代理进行，因此请设置 [`sandbox.network.tlsTerminate`](#sandbox-network-tlsterminate)，对于纯 HTTP 测试网络则设置 [`allowPlaintextInject`](#sandbox-credentials-allowplaintextinject)；请参阅[掩码凭据](https://code.claude.com/docs/zh-CN/sandboxing.md#mask-credentials)。Claude Code 接受但会忽略 `deny` 条目上的 `mask` 字段。

#### 环境变量掩码字段

`mask` 条目接受以下可选字段。没有 `extract` 或 `decode` 时，Claude Code 会将整个值替换为一个哨兵值。`extract` 和 `decode` 不能在同一条目上组合使用。

| 字段 | 类型 | 作用 |
| :- | :- | :- |
| `extract` | 字符串，至少包含一个捕获组的正则表达式 | 仅掩码每个匹配中第 1 组捕获的文本，例如 `DATABASE_URL` 连接字符串中的密码，使值的其余部分保持可解析。需要 v2.1.224 或更高版本 |
| `onExtractNoMatch` | `"warn"`、`"deny"` 或 `"error"`；默认为 `"warn"`。在带有 `decode` 的条目上，仅接受 `"warn"` | 当 `extract` 未匹配任何内容时的处理方式。`warn` 使变量不经掩码直接传递，`deny` 在沙箱内取消设置该变量，`error` 会停止沙箱设置，直到您修复配置。需要 v2.1.224 或更高版本 |
| `decode` | 字符串 `"jwt"` | 验证整个值是否为 JWT，并将其替换为结构有效的伪造令牌，使沙箱内解码该令牌的代码继续正常工作；代理会在出站时替换为完整的真实令牌。未通过验证的值会不经掩码直接传递并附带警告。需要 v2.1.224 或更高版本 |
| `maskClaims` | 字符串数组，至少包含一个声明名称；需要 `decode` | 仅掩码已解码 JWT 中指定的顶层负载声明，并围绕修改后的负载重建令牌，使其他声明保持可读。当没有指定的声明匹配时，变量会不经掩码直接传递并附带警告。需要 v2.1.224 或更高版本 |
| `injectHosts` | 字符串数组，每项都是 [`sandbox.network.allowedDomains`](#sandbox-network-alloweddomains) 也允许的主机 | 收窄沙箱代理替换真实值的主机范围。未设置时，代理会在发往 `sandbox.network.allowedDomains` 中每个主机的请求中进行替换。IPv6 目标请写为不带方括号的压缩地址，例如 `"::1"`，而不是带方括号的形式；请参阅[`injectHosts` 中的 IPv6 目标](https://code.claude.com/docs/zh-CN/sandboxing#ipv6-destinations-in-injecthosts) |

以下配置仅掩码 `DATABASE_URL` 中的密码，在模式未匹配任何内容时取消设置该变量，并掩码 `SERVICE_JWT` 中的 JWT，同时使除 `api_key` 之外的所有声明保持可读：

```json settings.json theme={null}
{
  "sandbox": {
    "credentials": {
      "envVars": [
        {
          "name": "DATABASE_URL",
          "mode": "mask",
          "extract": "://[^:]+:([^@]+)@",
          "onExtractNoMatch": "deny"
        },
        {
          "name": "SERVICE_JWT",
          "mode": "mask",
          "decode": "jwt",
          "maskClaims": ["api_key"]
        }
      ]
    }
  }
}
```

### `sandbox.credentials.allowPlaintextInject`

除 TLS 终止的 HTTPS 外，还允许在纯 HTTP 请求上进行 `mask` 替换。在纯 HTTP 上，上游身份未经验证，凭据以明文传输，因此在受信任的测试网络之外请保持关闭。

* **作用域**：[`User or managed`](#scopes)
* **类型**：布尔值
  * `true`：除 TLS 终止的 HTTPS 外，Claude Code 还允许在纯 HTTP 请求上进行 `mask` 替换
  * `false`：Claude Code 仅允许在 TLS 终止的 HTTPS 上进行 `mask` 替换
* **默认值**：`false`

```json settings.json theme={null}
{
  "sandbox": {
    "credentials": {
      "allowPlaintextInject": true
    }
  }
}
```

### `sandbox.credentials.awsPairs`

当您的凭据存放在非标准名称的变量中时，将构成同一 AWS 凭据的被掩码环境变量分组，用于 [SigV4 重新签名](https://code.claude.com/docs/zh-CN/sandboxing.md#re-sign-aws-requests)。当您掩码常规的 `AWS_ACCESS_KEY_ID`、`AWS_SECRET_ACCESS_KEY` 和 `AWS_SESSION_TOKEN` 三者的完整值时，Claude Code 会自动将它们关联，因此仅在使用其他名称时才需要此键。需要 Claude Code v2.1.224 或更高版本。

* **作用域**：[`User or managed`](#scopes)
* **类型**：对象数组，每个对象包含 `accessKeyIdVar`、`secretAccessKeyVar` 以及可选的 `sessionTokenVar`，用于指定 `sandbox.credentials.envVars` 条目
* **默认值**：未设置，因此只有常规的三个变量会被配对

以下配置将三个自定义名称的变量关联为一个 AWS 凭据以进行重新签名：

```json settings.json theme={null}
{
  "sandbox": {
    "credentials": {
      "awsPairs": [
        {
          "accessKeyIdVar": "MY_KEY_ID",
          "secretAccessKeyVar": "MY_SECRET_KEY",
          "sessionTokenVar": "MY_SESSION_TOKEN"
        }
      ]
    }
  }
}
```

每个指定的变量都必须是 [`sandbox.credentials.envVars`](#sandbox-credentials-envvars) 中完整值的 `mask` 条目，不带 `extract` 或 `decode`，并且在所有配对中只能占据一个位置。以下规则同样适用：

* 代理会对访问密钥 ID 条目的 `injectHosts` 中列出的主机上的请求进行重新签名
* 设置了 `sessionTokenVar` 时，代理会在重新签名的请求中以 `x-amz-security-token` 发送真实令牌
* 在配对中指定任何常规变量都会替换自动配对

### `sandbox.credentials.sigv4`

选择沙箱代理如何处理它[无法重新签名](https://code.claude.com/docs/zh-CN/sandboxing.md#re-sign-aws-requests)的 AWS 请求形式：`streaming` 用于 aws-chunked 流式上传，`presigned` 用于预签名 URL，`sigv4a` 用于 SigV4A 非对称签名。这仅适用于使用被掩码配对的占位访问密钥 ID 签名的请求。需要 Claude Code v2.1.224 或更高版本。

* **作用域**：[`User or managed`](#scopes)
* **类型**：包含 `streaming`、`presigned` 和 `sigv4a` 的对象，每项取以下值之一：
  * `"deny"`：代理使请求失败
  * `"passthrough"`：代理转发使用被掩码占位符签名的请求，因此工具会收到 AWS 自身的拒绝
* **默认值**：未设置，因此每种形式都为 `"deny"`

以下配置转发流式上传，而不是在代理处使其失败：

```json settings.json theme={null}
{
  "sandbox": {
    "credentials": {
      "sigv4": {
        "streaming": "passthrough"
      }
    }
  }
}
```

使用 `deny` 时，代理会使请求失败。使用 `passthrough` 时，代理会转发签名基于被掩码占位符计算的请求，因此 AWS 会拒绝它，调用工具收到的是 AWS 自身的响应，而不是代理错误。

### `sandbox.network`

控制沙箱化命令可以访问哪些主机、端口和套接字。沙箱通过一个强制执行这些列表的代理路由出站流量；有关代理如何决策以及何时提示，请参阅[网络隔离](https://code.claude.com/docs/zh-CN/sandboxing.md#network-isolation)。

* **作用域**：[`Any file`](#scopes)。`strictAllowlist`、`allowManagedDomainsOnly` 和 `tlsTerminate` 从较少的来源读取，详见各自条目。
* **类型**：包含以下子键的对象
* **默认值**：未设置，因此没有预先允许的域名，由您的权限模式决定[每个新主机的处理方式](https://code.claude.com/docs/zh-CN/sandboxing.md#hosts-outside-your-allowed-domains)

以下配置预先允许 GitHub 和 npm，阻止 `uploads.github.com`，并允许命令绑定到 localhost：

```json settings.json theme={null}
{
  "sandbox": {
    "network": {
      "allowedDomains": ["github.com", "*.npmjs.org"],
      "deniedDomains": ["uploads.github.com"],
      "allowLocalBinding": true
    }
  }
}
```

Claude Code 会在各设置作用域之间合并数组子键，因此除非适用[仓库锁定](https://code.claude.com/docs/zh-CN/sandboxing.md#repository-settings-under-an-admin-required-sandbox)，否则项目可以向您的用户列表添加域名。`WebFetch(domain:...)` 允许和拒绝[权限规则](https://code.claude.com/docs/zh-CN/sandboxing.md#permission-rules)也会添加到相同的允许和拒绝列表中。

### `sandbox.network.allowUnixSockets`

列出 macOS 上沙箱化命令可以连接的 Unix 套接字路径。Claude Code 在 Linux 和 WSL2 上会忽略此列表，因为 seccomp 过滤器无法检查套接字路径；在这些平台上请改用 [`allowAllUnixSockets`](#sandbox-network-allowallunixsockets)。

* **作用域**：[`Any file`](#scopes)，[对项目设置和本地设置有限制](https://code.claude.com/docs/zh-CN/sandboxing.md#repository-settings-under-an-admin-required-sandbox)
* **类型**：字符串数组，每项是一个套接字路径
* **默认值**：未设置，因此 macOS 沙箱阻止所有 Unix 套接字

```json settings.json theme={null}
{
  "sandbox": {
    "network": {
      "allowUnixSockets": ["~/.ssh/agent-socket"]
    }
  }
}
```

套接字路径可能授予广泛的访问权限：例如，允许 `/var/run/docker.sock` 会让沙箱化命令控制 Docker 守护进程。请参阅[安全限制](https://code.claude.com/docs/zh-CN/sandboxing.md#security-limitations)。

### `sandbox.network.allowAllUnixSockets`

允许沙箱化命令连接到所有 Unix 套接字。在 Linux 和 WSL2 上，沙箱的 [seccomp 过滤器](https://code.claude.com/docs/zh-CN/sandboxing.md#set-up-linux-and-wsl2)会阻止 `socket(AF_UNIX, ...)` 调用，因此这是在这些平台上允许 Unix 套接字的唯一方式。当过滤器缺失时（`/sandbox` 会在其 Dependencies 选项卡中报告），沙箱不会阻止 Unix 套接字调用。有关过滤器的来源，请参阅[设置 Linux 和 WSL2](https://code.claude.com/docs/zh-CN/sandboxing.md#set-up-linux-and-wsl2)。

* **作用域**：[`Any file`](#scopes)，[对项目设置和本地设置有限制](https://code.claude.com/docs/zh-CN/sandboxing.md#repository-settings-under-an-admin-required-sandbox)
* **类型**：Boolean
  * `true`：沙箱内命令可以连接到所有 Unix 套接字。在 Linux 和 WSL2 上，它们还可以连接到 VM 套接字，即 `AF_VSOCK` 地址族
  * `false`：沙箱阻止 Unix 套接字连接：在 macOS 上，`allowUnixSockets` 中的路径除外；在 Linux 和 WSL2 上，在 seccomp 过滤器存在时通过该过滤器阻止
* **默认值**：`false`

```json settings.json theme={null}
{
  "sandbox": {
    "network": {
      "allowAllUnixSockets": true
    }
  }
}
```

在 WSL2 上，`true` 还会重新开放用于启动 `cmd.exe` 和 `powershell.exe` 等 Windows 二进制文件的互操作套接字。

### `sandbox.network.allowLocalBinding`

允许 macOS 上的沙箱化命令监听网络端口（例如启动开发服务器），并连接到 localhost 上的任意端口。监听非回环地址的命令会接受来自其他机器的连接。此键在 Linux 和 WSL2 上无效，因为在这些平台上每条沙箱化命令都有自己的回环接口。要从 Linux 或 WSL2 访问主机上的服务器，请参阅[命令无法访问 localhost 上的服务器](https://code.claude.com/docs/zh-CN/sandboxing.md#a-command-fails-to-reach-a-server-on-localhost)。

* **作用域**：[`Any file`](#scopes)，[对项目设置和本地设置有限制](https://code.claude.com/docs/zh-CN/sandboxing.md#repository-settings-under-an-admin-required-sandbox)
* **类型**：布尔值
  * `true`：macOS 上的沙箱化命令可以监听任何本地地址，并连接到 localhost 上的任意端口
  * `false`：macOS 上的沙箱化命令无法监听端口，也无法直接连接到 localhost 上的服务器
* **默认值**：`false`

```json settings.json theme={null}
{
  "sandbox": {
    "network": {
      "allowLocalBinding": true
    }
  }
}
```

### `sandbox.network.allowMachLookup`

列出 macOS 沙箱可以查找的其他 XPC 和 Mach 服务名称。通过 XPC 通信的工具（如 iOS Simulator 或 Playwright）需要在此处列出其服务。

* **作用域**：[`Any file`](#scopes)，[对项目设置和本地设置有限制](https://code.claude.com/docs/zh-CN/sandboxing.md#repository-settings-under-an-admin-required-sandbox)
* **类型**：字符串数组，每项是一个服务名称；单个末尾 `*` 匹配前缀，单独的 `"*"` 匹配所有服务
* **默认值**：未设置

以下配置允许 `com.apple.CoreSimulator.` 前缀下的所有服务：

```json settings.json theme={null}
{
  "sandbox": {
    "network": {
      "allowMachLookup": ["com.apple.CoreSimulator.*"]
    }
  }
}
```

### `sandbox.network.allowedDomains`

为沙箱化命令的出站流量预先允许域名，使沙箱不再为它们提示。`*.example.com` 等通配符匹配子域名，可选的 `:port` 后缀将条目限制为一个端口；不带端口的条目匹配所有端口。

* **作用域**：[`Any file`](#scopes)，[对项目设置和本地设置有限制](https://code.claude.com/docs/zh-CN/sandboxing.md#repository-settings-under-an-admin-required-sandbox)。设置了 [`allowManagedDomainsOnly`](#sandbox-network-allowmanageddomainsonly) 时仅限托管设置。
* **类型**：字符串数组，每项是一个域名、通配符模式或 IP 字面量，可带可选的 `:port` 后缀
* **默认值**：未设置，因此由您的权限模式决定[每个新主机的处理方式](https://code.claude.com/docs/zh-CN/sandboxing.md#hosts-outside-your-allowed-domains)

以下配置在所有端口上预先允许 GitHub、所有 npm 子域名，以及仅在 443 端口上的一个 API 主机：

```json settings.json theme={null}
{
  "sandbox": {
    "network": {
      "allowedDomains": ["github.com", "*.npmjs.org", "api.example.com:443"]
    }
  }
}
```

IPv6 字面量请用方括号括起，可带可选端口：`"[::1]"` 允许所有端口，`"[::1]:443"` 允许一个端口。方括号形式需要 Claude Code v2.1.229 或更高版本。请参阅[域名列表中的 IPv6 地址](https://code.claude.com/docs/zh-CN/sandboxing.md#ipv6-addresses-in-domain-lists)。

### `sandbox.network.deniedDomains`

阻止沙箱化命令出站流量访问某些域名，使用与 [`allowedDomains`](#sandbox-network-alloweddomains) 相同的通配符、端口和 IPv6 语法。即使某个 `allowedDomains` 条目也匹配被拒绝的域名，该域名仍保持阻止。

* **作用域**：[`Any file`](#scopes)
* **类型**：字符串数组，每项是一个域名、通配符模式或 IP 字面量，可带可选的 `:port` 后缀
* **默认值**：未设置

```json settings.json theme={null}
{
  "sandbox": {
    "network": {
      "deniedDomains": ["sensitive.cloud.example.com"]
    }
  }
}
```

即使设置了 `allowManagedDomainsOnly`，Claude Code 也会合并来自会话加载的所有设置来源的此列表，因此开发者始终可以收紧拒绝列表。有关 IPv6 字面量，请参阅[域名列表中的 IPv6 地址](https://code.claude.com/docs/zh-CN/sandboxing.md#ipv6-addresses-in-domain-lists)。

使用标记完全限定域名的末尾点书写的条目（如 `example.com.`）与 `example.com` 阻止相同的连接。

### `sandbox.network.strictAllowlist`

拒绝沙箱化命令访问允许列表之外的主机，而不是提示批准。允许列表为 [`allowedDomains`](#sandbox-network-alloweddomains) 加上来自 `WebFetch(domain:...)` 允许规则的域名；设置了 [`allowManagedDomainsOnly`](#sandbox-network-allowmanageddomainsonly) 时，则仅为托管设置中的条目。[无需管理员强制要求沙箱即适用的锁定](https://code.claude.com/docs/zh-CN/sandboxing.md#locks-that-apply-without-an-admin-required-sandbox)介绍了仓库的条目。需要 Claude Code v2.1.219 或更高版本。

* **作用域**：[`User or managed`](#scopes)。仓库无法启用或关闭它。
* **类型**：布尔值
  * `true`：Claude Code 拒绝沙箱化命令访问允许列表之外的主机
  * `false`：除非另一个受信任的设置文件设置了 `true`，否则 Claude Code 会按权限模式处理允许列表之外的主机，而不是直接拒绝：在自动模式下，它会根据命令的[每命令允许域名](https://code.claude.com/docs/zh-CN/sandboxing.md#per-command-allowed-domains-in-auto-mode)检查该主机；在 `dontAsk` 模式下拒绝；在 `bypassPermissions` 模式下以及可使用绕过的交互式终端计划模式会话中允许；其他情况下询问您
* **默认值**：`false`

```json settings.json theme={null}
{
  "sandbox": {
    "network": {
      "strictAllowlist": true
    }
  }
}
```

Claude Code 仅对沙箱化命令强制执行此设置；`WebFetch` 等进程内工具仍遵循其[权限规则](https://code.claude.com/docs/zh-CN/sandboxing.md#permission-rules)。当任何被遵循的来源将其设置为 `true` 时，它会保持启用。请参阅[网络隔离](https://code.claude.com/docs/zh-CN/sandboxing.md#network-isolation)。需要 Claude Code v2.1.219 或更高版本。

### `sandbox.network.allowManagedDomainsOnly`

将网络允许列表锁定为托管设置所定义的内容。此时 Claude Code 仅遵循来自托管设置的 `allowedDomains` 和 `WebFetch(domain:...)` 允许规则，忽略来自用户、项目、本地和 `--settings` 设置的域名，并自动阻止未允许的域名而不是提示。

* **作用域**：[`Managed`](#scopes)
* **类型**：布尔值
  * `true`：Claude Code 仅遵循来自托管设置的 `allowedDomains` 和 `WebFetch(domain:...)` 允许规则，并阻止未允许的域名而不是提示
  * `false`：来自其他设置文件的域名可以合并到允许列表中
* **默认值**：`false`

以下配置将允许列表锁定为 GitHub 和 npm，并忽略开发者添加的任何域名：

```json managed-settings.json theme={null}
{
  "sandbox": {
    "network": {
      "allowManagedDomainsOnly": true,
      "allowedDomains": ["github.com", "*.npmjs.org"]
    }
  }
}
```

当此键为 `true` 时，沙箱是[管理员强制要求的](https://code.claude.com/docs/zh-CN/sandboxing.md#repository-settings-under-an-admin-required-sandbox)，并且只有托管设置可以设置[代理端口](#sandbox-network-httpproxyport)。

被拒绝的域名仍会从会话加载的所有来源合并。请参阅[防止开发者放宽策略](https://code.claude.com/docs/zh-CN/sandboxing.md#keep-developers-from-widening-the-policy)。

### `sandbox.network.httpProxyPort`

让沙箱使用您自己的 HTTP 代理，而不是 Claude Code 运行的代理。组织这样做是为了检查 HTTPS 流量、应用自己的过滤规则或记录请求日志。您的代理接管过滤工作，Claude Code 不再对发送到那里的流量应用其域名列表和网络提示。未设置时，Claude Code 会为 HTTP 流量启动自己的代理。

* **作用域**：[`Any file`](#scopes)，除非[其他沙箱设置限制了哪些文件可以设置端口](https://code.claude.com/docs/zh-CN/sandboxing.md#custom-proxy-configuration)
* **类型**：数字，一个本地 TCP 端口
* **默认值**：未设置，因此 Claude Code 运行自己的代理

```json settings.json theme={null}
{
  "sandbox": {
    "network": {
      "httpProxyPort": 8080
    }
  }
}
```

如果您的代理也应承载 SOCKS 流量，请同时设置 [`socksProxyPort`](#sandbox-network-socksproxyport)；如果只设置了两者之一，Claude Code 仍会为另一种协议运行自己的代理。请参阅[自定义代理配置](https://code.claude.com/docs/zh-CN/sandboxing.md#custom-proxy-configuration)。

### `sandbox.network.socksProxyPort`

让沙箱使用您自己的 SOCKS5 代理，而不是 Claude Code 运行的代理。您的代理接管过滤工作，Claude Code 不再对发送到那里的流量应用其域名列表和网络提示。未设置时，Claude Code 会为 SOCKS 流量启动自己的代理。

* **作用域**：[`Any file`](#scopes)，除非[其他沙箱设置限制了哪些文件可以设置端口](https://code.claude.com/docs/zh-CN/sandboxing.md#custom-proxy-configuration)
* **类型**：数字，一个本地 TCP 端口
* **默认值**：未设置，因此 Claude Code 运行自己的代理

```json settings.json theme={null}
{
  "sandbox": {
    "network": {
      "socksProxyPort": 8081
    }
  }
}
```

请参阅[自定义代理配置](https://code.claude.com/docs/zh-CN/sandboxing.md#custom-proxy-configuration)。

### `sandbox.network.tlsTerminate`

让沙箱代理终止 TLS，以便读取 HTTPS 请求的内容。此功能为实验性功能，`mask` [凭据替换](https://code.claude.com/docs/zh-CN/sandboxing.md#mask-credentials)需要启用它。设置为 `{}` 可为会话生成一个临时证书颁发机构，或设置 `caCertPath` 和 `caKeyPath` 以使用您自己的证书颁发机构。

* **作用域**：[`User or managed`](#scopes)。仓库无法启用此设置，也无法提供证书颁发机构。
* **类型**：对象，包含可选的 `caCertPath` 和 `caKeyPath` 字符串，每个均为文件路径
* **默认值**：未设置，因此代理不会终止或检查 TLS

```json settings.json theme={null}
{
  "sandbox": {
    "network": {
      "tlsTerminate": {}
    }
  }
}
```

当多个受认可的来源都设置了此项时，Claude Code 使用优先级最高的来源中的值：托管设置，其次是 `--settings` 标志，然后是用户设置。

## 记忆与上下文

控制 Claude Code 将哪些内容加载到上下文中、如何进行压缩，以及将记忆和计划保存在何处。请参阅[管理上下文](https://code.claude.com/docs/zh-CN/context-window.md)和[记忆](https://code.claude.com/docs/zh-CN/memory.md)。

### `autoCompactEnabled`

让 Claude Code 在上下文接近上限时[自动压缩对话](https://code.claude.com/docs/zh-CN/context-window.md#when-your-context-fills-up)。在 `/config` 中显示为 **Auto-compact**，在其中切换时会将此键写入您的用户设置。

* **作用域**：[`Any file`](#scopes)
* **类型**：布尔值
  * `true`：Claude Code 在上下文接近上限时自动压缩对话
  * `false`：Claude Code 不会自动压缩
* **默认值**：`true`
* **单会话覆盖**：[`DISABLE_AUTO_COMPACT`](https://code.claude.com/docs/zh-CN/env-vars) 可为单个会话关闭自动压缩；两者中任意一个将其关闭后，另一个都无法重新将其开启

```json settings.json theme={null}
{
  "autoCompactEnabled": false
}
```

关闭自动压缩后，手动 `/compact` 命令仍然可用。如果您关闭自动压缩，[空闲压缩](#idlecompaction)也会随之关闭。

### `autoCompactWindow`

设置上下文窗口填充到多满时 Claude Code 会[自动压缩](https://code.claude.com/docs/zh-CN/context-window.md#when-your-context-fills-up)。

* **作用域**：[`Any file`](#scopes)
* **类型**：token 数量，范围为 `100000` 到 `1000000`。Claude Code 会将该值限制在您所用模型的上下文窗口以内；[模型概览](https://platform.claude.com/docs/en/about-claude/models/overview)列出了每个模型的上下文窗口
* **默认值**：未设置，此时 Claude Code 会选择针对您的模型调优的窗口
* **单会话覆盖**：[`--autocompact`](https://code.claude.com/docs/zh-CN/cli-reference#cli-flags) 在单个会话中优先于此键，而 [`CLAUDE_CODE_AUTO_COMPACT_WINDOW`](https://code.claude.com/docs/zh-CN/env-vars) 优先于这两者，除非 Claude Code [忽略该变量的值](https://code.claude.com/docs/zh-CN/model-config.md#claude_code_auto_compact_window-values)

```json settings.json theme={null}
{
  "autoCompactWindow": 500000
}
```

[`/autocompact`](https://code.claude.com/docs/zh-CN/commands#all-commands) 命令会将当前模型的窗口保存在 [`modelSettings`](#modelsettings) 下，对于该模型，它优先于同一文件中的此键。[设置自动压缩窗口](https://code.claude.com/docs/zh-CN/model-config.md#set-the-auto-compact-window)介绍了该命令、标志、变量和设置之间的相互作用。

### `autoMemoryDirectory`

将[自动记忆](https://code.claude.com/docs/zh-CN/memory.md#storage-location)存储在您选择的目录中，而不是每个项目的默认目录。

* **作用域**：[`Any file`](#scopes)
* **类型**：字符串，绝对路径或以 `~/` 开头的目录路径
* **默认值**：未设置，此时 Claude Code 使用 `~/.claude/projects/<project>/memory/`

```json settings.json theme={null}
{
  "autoMemoryDirectory": "~/my-memory-dir"
}
```

对于项目设置或本地设置中的此键，Claude Code 遵循[与 hook 相同的工作区信任规则](https://code.claude.com/docs/zh-CN/permissions.md#what-runs-before-you-trust-a-folder)，因为克隆的仓库可能会提供这些文件。

### `autoMemoryEnabled`

开启或关闭[自动记忆](https://code.claude.com/docs/zh-CN/memory.md#enable-or-disable-auto-memory)。为 `false` 时，Claude 不会读取或写入自动记忆目录。您也可以在会话中使用 `/memory` 切换它，这会将此键写入您的用户设置。

* **作用域**：[`Any file`](#scopes)
* **类型**：布尔值
  * `true`：与未设置相同；自动记忆保持开启，除非有优先级高于此键的设置为当前会话将其关闭，例如 `--bare`、安全模式或 `CLAUDE_CODE_DISABLE_AUTO_MEMORY`
  * `false`：Claude 不会读取或写入自动记忆目录
* **默认值**：`true`
* **单会话覆盖**：[`CLAUDE_CODE_DISABLE_AUTO_MEMORY`](https://code.claude.com/docs/zh-CN/env-vars) 在单个会话中优先于此键，无论开启还是关闭

```json settings.json theme={null}
{
  "autoMemoryEnabled": false
}
```

### `bashOutputMaxChars`

设置成功执行的 Bash 或 PowerShell 命令的输出中，有多少字符会[以内联方式提供给 Claude](https://code.claude.com/docs/zh-CN/tools-reference.md#output-limits)。当输出超过此上限时，Claude Code 会将其保存到文件中，Claude 会收到一段简短预览以及该文件的路径。如果命令输出（例如详细的构建输出或完整的测试套件日志）经常超出默认上限，而您希望 Claude 无需打开文件即可读取，请提高此上限。需要 Claude Code v2.1.261 或更高版本。

* **作用域**：[`Any file`](#scopes)
* **类型**：字符数，正整数。Claude Code 会将该值限制在 `4000` 到 `128000` 的范围内
* **默认值**：未设置，此时 Claude 以内联方式最多接收 30,000 个字符

```json settings.json theme={null}
{
  "bashOutputMaxChars": 100000
}
```

设置此键后，Claude Code 会忽略 [`BASH_MAX_OUTPUT_LENGTH`](https://code.claude.com/docs/zh-CN/env-vars) 环境变量。

### `claudeMd`

将 CLAUDE.md 风格的指令作为组织托管的记忆注入，而无需部署单独的文件。Claude Code 会将该文本作为托管记忆条目加载，其顺序先于用户和项目的 CLAUDE.md 文件。

* **作用域**：[`Managed`](#scopes)
* **类型**：字符串，即 CLAUDE.md 文件的文本；按照编写该文件的方式书写，包括 Markdown，换行符写作 `\n`
* **默认值**：未设置

此示例以简短的 Markdown 列表形式部署两条规则：

```json managed-settings.json theme={null}
{
  "claudeMd": "# Engineering rules\n\n- Always run make lint before committing.\n- Never push directly to main."
}
```

请参阅[部署组织范围的 CLAUDE.md](https://code.claude.com/docs/zh-CN/memory.md#deploy-organization-wide-claude-md)。

### `claudeMdExcludes`

在 Claude Code 加载[记忆](https://code.claude.com/docs/zh-CN/memory.md#exclude-specific-claude-md-files)时跳过特定的 `CLAUDE.md` 文件。在大型 monorepo 中，可用它跳过其他团队与您的工作无关的 CLAUDE.md 文件；大型代码库指南中的[排除无关的 CLAUDE.md 文件](https://code.claude.com/docs/zh-CN/large-codebases.md#exclude-irrelevant-claude-md-files)详细介绍了这种情况。模式会与绝对文件路径进行匹配。

* **作用域**：[`Any file`](#scopes)
* **类型**：字符串数组，每一项为 glob 模式或绝对路径
* **默认值**：未设置，此时 Claude Code 会加载它找到的每个 CLAUDE.md

```json settings.json theme={null}
{
  "claudeMdExcludes": ["**/vendor/**/CLAUDE.md"]
}
```

排除仅适用于用户、项目和本地记忆文件；托管策略的 CLAUDE.md 文件无法被排除。

### `env`

为每个会话以及 Claude Code 从中启动的子进程设置环境变量。[环境变量参考](https://code.claude.com/docs/zh-CN/env-vars.md)中的大多数变量都可以放在这里，这样就能将某个变量应用到每个会话或推广到您的团队。项目设置和本地设置无法设置[其中一部分变量](#variables-claude-code-ignores-in-env)。

* **作用域**：[`Any file`](#scopes)
* **类型**：将变量名映射到字符串值的对象
* **默认值**：未设置

此示例关闭自动压缩，并通过代理路由 API 请求：

```json settings.json theme={null}
{
  "env": {
    "DISABLE_AUTO_COMPACT": "1",
    "ANTHROPIC_BASE_URL": "https://proxy.example.com"
  }
}
```

#### `env` 值如何与您的 shell 交互

* 此处的值会覆盖您在 shell 中导出的同名变量；当多个设置文件设置了同一变量时，以[优先级最高](https://code.claude.com/docs/zh-CN/settings.md#settings-precedence)的为准。[Claude Code 在 `env` 中忽略的变量](#variables-claude-code-ignores-in-env)列出了项目设置和本地设置的例外情况。
* 当 Claude Desktop 应用或[自托管环境](https://code.claude.com/docs/zh-CN/self-hosted-environments)运行器启动会话时，它所构建的启动环境优先：对于启动环境已设置的变量，Claude Code 会忽略任何设置文件中的 `env` 值。[调试日志](https://code.claude.com/docs/zh-CN/debug-your-config.md)会列出每个被忽略的变量。
* 要取消 shell 中的导出，请将该变量设置为 `""`。在选择提供商时，Claude Code 将空值视为未设置，而子进程会继承该空值。
* 在此处设置的 `NO_COLOR` 和 `FORCE_COLOR` 仅作用于子进程。要更改 Claude Code 自身的界面颜色，请在启动 `claude` 之前在 shell 中设置它们。
* 此处的值以明文形式保存在设置文件中，并会传递给 Claude Code 启动的每个子进程。对于会轮换的 OTLP bearer 令牌，请使用 [`otelHeadersHelper`](#otelheadershelper)；对于 API 凭据，请使用 [`apiKeyHelper`](#apikeyhelper)。

#### Claude Code 何时应用 `env` 值

* 来自用户设置、`--settings` 和托管设置的值：在启动时应用；当保存的更改改变了合并后的 `env` 时，会在正在运行的会话中再次应用。
* 来自项目设置和本地设置的值：在您信任工作区之后应用，或在 `-p` 模式（从不显示信任对话框）下于启动时应用；当保存的更改改变了合并后的 `env` 时会再次应用。
* Claude Code 归类为安全的变量，例如模型选择、超时和限制以及功能开关：在启动时从每个设置文件中应用，但[项目设置和本地设置无法设置的变量](#variables-claude-code-ignores-in-env)除外。
* 在 v2.1.246 或更高版本中[使用 `/cd` 移动会话](https://code.claude.com/docs/zh-CN/permissions#move-the-session-to-another-directory)之后：在前一个目录的值之上应用新目录的项目和本地 `env` 值。

#### Claude Code 在 `env` 中忽略的变量

* 项目设置和本地设置无法设置那些不应由检出的仓库控制的变量；请改为在 shell、用户设置或托管设置中设置它们。Claude Code 会丢弃每个此类变量（少数关闭遥测的值除外），并记录一条可通过 `claude --debug` 查看的警告。它们包括：

  * 选择 Claude Code 存储或写入自身文件位置的变量：`CLAUDE_CONFIG_DIR`、`CLAUDE_CODE_TMPDIR`，以及操作系统目录变量，例如 `HOME`、`TMPDIR`、`TMP`、`TEMP` 和 `XDG_*` 系列。
  * 为 Claude Code 启动的进程选择程序和全机配置的 Windows 变量，例如 `SystemRoot`、`ComSpec`、`ProgramData`、`LOCALAPPDATA`、`PATHEXT`、`PSModulePath` 和 `ProgramFiles` 系列。
  * 导出会话内容的变量：[`OTEL_LOG_RAW_API_BODIES`](https://code.claude.com/docs/zh-CN/env-vars#variables) 以及详细 beta 追踪变量对 `ENABLE_BETA_TRACING_DETAILED` 和 `BETA_TRACING_ENDPOINT`。
  * 用于开启遥测、选择遥测去向或选择其捕获内容的 [OpenTelemetry 导出器](https://code.claude.com/docs/zh-CN/monitoring-usage.md)变量：

    * `CLAUDE_CODE_ENABLE_TELEMETRY`，以及增强遥测 beta 变量对 `CLAUDE_CODE_ENHANCED_TELEMETRY_BETA` 和 `ENABLE_ENHANCED_TELEMETRY_BETA`
    * 导出器选择变量 `OTEL_LOGS_EXPORTER`、`OTEL_METRICS_EXPORTER` 和 `OTEL_TRACES_EXPORTER`
    * 内容变量 `OTEL_LOG_USER_PROMPTS`、`OTEL_LOG_ASSISTANT_RESPONSES`、`OTEL_LOG_TOOL_CONTENT` 和 `OTEL_LOG_TOOL_DETAILS`
    * 名称以 `_ENDPOINT`、`_HEADERS`、`_PROTOCOL`、`_CERTIFICATE`、`_CLIENT_KEY` 或 `_INSECURE` 结尾的 `OTEL_EXPORTER_OTLP_*` 变量，包括通用形式和按信号区分的形式，例如 `OTEL_EXPORTER_OTLP_ENDPOINT` 和 `OTEL_EXPORTER_OTLP_METRICS_HEADERS`
    * `OTEL_EXPORTER_PROMETHEUS_HOST` 和 `OTEL_EXPORTER_PROMETHEUS_PORT`

    只有以下值仍会从项目设置和本地设置中生效，因为它们用于关闭某项功能：三个导出器选择变量的 `none`，以及 `OTEL_LOG_USER_PROMPTS`、`OTEL_LOG_TOOL_CONTENT` 和 `OTEL_LOG_TOOL_DETAILS` 的关闭值（例如 `0`）。此类值会覆盖您用户设置中的同名变量，但不会覆盖由启动 Claude Code 的环境、`--settings` 文件或托管设置所设置的变量。

    当项目或本地设置文件设置了此组中的变量时，本地交互式会话会在启动时显示一条通知。运行 `/status` 或 `claude doctor` 可查看 Claude Code 忽略了哪些变量以及哪些变量关闭了遥测；两者都只列出名称，从不列出值。使用 `-p` 的非交互式运行或 Agent SDK 会话不会显示通知，因此升级后请检查您的收集器是否仍能收到数据。如果收不到，请在用户设置、托管设置、作业环境或通过 `--settings` 传入的文件中设置这些变量。

    在项目设置和本地设置中忽略此组变量需要 Claude Code v2.1.282 或更高版本。
  * 改变 Claude Code 启动或同步方式的变量，例如 `CLAUDE_CODE_PROCESS_WRAPPER`、`CLAUDE_CODE_SYNC_SKILLS`、`CLAUDE_CODE_SYNC_PLUGINS`、`CLAUDE_CODE_PLUGIN_CACHE_DIR` 和 `CLAUDE_CODE_PLUGIN_SEED_DIR`。
  * [屏幕阅读器模式](https://code.claude.com/docs/zh-CN/accessibility.md)变量 `CLAUDE_AX_SCREEN_READER`，以及其他名称以 `CLAUDE_AX_` 开头的变量，例如 [`CLAUDE_AX_STARTUP_QUIET_MS`](https://code.claude.com/docs/zh-CN/env-vars#variables)。
  * [`CLAUDE_CODE_USER_DIALOG_TIMEOUT_MS`](https://code.claude.com/docs/zh-CN/env-vars#variables)，用于设置未响应对话框的计时器。
  * [`CLAUDE_CODE_DISABLE_ATTACHMENTS`](https://code.claude.com/docs/zh-CN/env-vars#variables)，用于关闭附件处理。
  * [`CLAUDE_CODE_DISABLE_FILE_MENTIONS`](https://code.claude.com/docs/zh-CN/env-vars#variables)，用于阻止 Claude Code 展开对文件和目录的 `@` 提及。

  在 v2.1.251 之前，项目设置和本地设置还可以设置此列表中选择 Claude Code 文件写入位置或导出会话内容的变量，但 `HOME` 和 `XDG_CONFIG_HOME` 除外。在 v2.1.290 之前，它们还可以设置对话框计时器变量和 `CLAUDE_CODE_DISABLE_ATTACHMENTS`。
* 由 Claude Code 托管环境所拥有的身份变量（例如 `CLAUDE_CODE_REMOTE` 和 `CLAUDE_CODE_ACCOUNT_UUID`）在所有文件中都会被忽略。
* [`CLAUDE_CODE_MESSAGING_SOCKET` 和 `CLAUDE_CODE_MESSAGING_TOKEN`](https://code.claude.com/docs/zh-CN/env-vars#variables) 由 Claude Code 自行导出，在所有文件中都会被忽略。忽略 socket 变量需要 Claude Code v2.1.224 或更高版本，忽略 token 变量需要 v2.1.228 或更高版本。
* [`CLAUDE_CODE_PROJECT_DIR_NAME`](https://code.claude.com/docs/zh-CN/sessions#name-the-project-directory-yourself) 仅由 Claude Code 从启动环境中读取，在所有文件中都会被忽略；需要 v2.1.234 或更高版本。
* [`CLAUDE_CODE_RESTRICTED`](https://code.claude.com/docs/zh-CN/env-vars#variables) 仅由 Claude Code 从启动环境中读取，在所有文件中都会被忽略。
* [`CLAUDE_CODE_DISABLE_POWERSHELL_CMD_RM_DENY`](https://code.claude.com/docs/zh-CN/env-vars#variables) 仅由 Claude Code 从启动环境中读取，在所有文件中都会被忽略。该变量需要 Claude Code v2.1.283 或更高版本。
* [`CLAUDE_CODE_DISABLE_DANGEROUS_RM_TIMEOUT`、`CLAUDE_CODE_DISABLE_SUBSTITUTION_RM_PROMPT`、`CLAUDE_CODE_DISABLE_INLINE_SHELL_RM_PROMPT` 和 `CLAUDE_CODE_DISABLE_MOUNTED_DRIVE_RM_PROMPT`](https://code.claude.com/docs/zh-CN/env-vars#variables) 仅由 Claude Code 从启动环境中读取，在所有文件中都会被忽略。
* [`CLAUDE_CODE_EVEN_MORE_DANGEROUSLY_SKIP_PERMISSIONS`](https://code.claude.com/docs/zh-CN/env-vars#variables) 仅由 Claude Code 从启动环境中读取，在所有文件中都会被忽略。
* [`CLAUDE_CODE_WARN_DISK_FREE_GIB` 和 `CLAUDE_CODE_WARN_MEMORY_FREE_GIB`](https://code.claude.com/docs/zh-CN/env-vars#variables) 仅由 Claude Code 从启动环境中读取，在所有文件中都会被忽略。这些变量需要 Claude Code v2.1.304 或更高版本。

### `fileCheckpointingEnabled`

让 Claude Code 在每次编辑前为文件创建快照，以便 [`/rewind`](https://code.claude.com/docs/zh-CN/checkpointing) 可以恢复它们。在 `/config` 中显示为 **Rewind code (checkpoints)**，在其中切换时会将此键写入您的用户设置。

* **作用域**：[`Any file`](#scopes)
* **类型**：布尔值
  * `true`：Claude Code 在每次编辑前为文件创建快照，以便 `/rewind` 可以恢复它们
  * `false`：Claude Code 不为文件创建快照，因此 `/rewind` 无法恢复它们
* **默认值**：`true`
* **单会话覆盖**：[`CLAUDE_CODE_DISABLE_FILE_CHECKPOINTING`](https://code.claude.com/docs/zh-CN/env-vars) 可为单个会话关闭检查点功能；两者中任意一个将其关闭后，另一个都无法重新将其开启

```json settings.json theme={null}
{
  "fileCheckpointingEnabled": false
}
```

在 `-p` 运行或 Agent SDK 会话中，Claude Code 会忽略此键。SDK 通过其 `enableFileCheckpointing` 选项开启检查点功能，而单纯的 `-p` 运行需要 `CLAUDE_CODE_ENABLE_SDK_FILE_CHECKPOINTING=true`。请参阅 [Agent SDK 中的文件检查点功能](https://code.claude.com/docs/zh-CN/agent-sdk/file-checkpointing.md)。

### `idleCompaction`

空闲压缩会对您离开的长对话进行[压缩](https://code.claude.com/docs/zh-CN/context-window.md#what-survives-compaction)。这样，您的下一条消息发送的将是摘要，而不是整个对话。压缩完成后，对话中会显示一条通知：

* **终端**：一行以 `Compacted while idle, before the prompt cache expired` 开头的内容
* **[VS Code 聊天面板](#idle-compaction-in-vs-code-and-the-desktop-app)**：一条包含 `a summary written while you were away` 的通知
* **[桌面应用](#idle-compaction-in-vs-code-and-the-desktop-app)**：同样的通知，位于标签包含 `Claude Code notice` 的行中。如果您看不到该行或其文本，请切换到 [Verbose 视图模式](https://code.claude.com/docs/zh-CN/desktop.md#switch-view-modes)。

对于满足以下所有条件的对话，空闲压缩默认开启：

* **会话**：您处于交互式终端会话中、VS Code 扩展的聊天面板中，或者在会话运行于您的计算机上或通过 SSH 运行时处于桌面应用的 Code 标签页中
* **自动压缩**：[`autoCompactEnabled`](#autocompactenabled) 已开启
* **提示缓存**：该对话使用[一小时提示缓存 TTL](https://code.claude.com/docs/zh-CN/prompt-caching.md#which-ttl-each-request-gets)
* **功能标志**：Claude Code 可以[从 Anthropic 获取功能标志](https://code.claude.com/docs/zh-CN/env-vars.md#features-that-need-feature-flag-fetching)
* **大小**：对话的上下文已达到 token 阈值，您可以将其从默认的 200,000 进行[更改](#change-the-idle-compaction-threshold)
* **空闲时间**：自上次请求以来已过去约一小时，且在此期间您的计算机保持唤醒状态

要关闭空闲压缩，请将此键设置为 `false`。

* **作用域**：[`Any file`](#scopes)
* **类型**：布尔值
  * `true`：Claude Code 会压缩满足条件的空闲对话
  * `false`：Claude Code 不会在会话空闲时进行压缩
* **默认值**：`true`
* **单会话覆盖**：[`DISABLE_AUTO_COMPACT`](https://code.claude.com/docs/zh-CN/env-vars) 会在关闭自动压缩的同时关闭空闲压缩

```json settings.json theme={null}
{
  "idleCompaction": false
}
```

要在终端会话中开启或关闭空闲压缩，请运行 `/config` 并更改 **Idle compaction** 行。您的选择会保存到用户设置中并立即生效。该行需要 Claude Code v2.1.302 或更高版本。如果您使用的是这些版本之一，但 `/config` 中未显示该行，请检查以下原因：

* **其他设置来源设置了此键**：托管设置、`--settings` 标志或项目或本地设置文件中的值覆盖了您的用户设置。请改为在该来源中更改此键
* **功能标志不可用**：当 Claude Code 无法[从 Anthropic 获取功能标志](https://code.claude.com/docs/zh-CN/env-vars.md#features-that-need-feature-flag-fetching)时，空闲压缩不会运行，因此没有需要关闭的内容

此键需要 Claude Code v2.1.290 或更高版本。较早版本会忽略此键，要停止空闲压缩，请关闭 [`autoCompactEnabled`](#autocompactenabled)。

#### 更改空闲压缩阈值

将 [`CLAUDE_CODE_IDLE_COMPACT_MIN_TOKENS`](https://code.claude.com/docs/zh-CN/env-vars) 设置为对话上下文在适用空闲压缩前必须达到的 token 数。请使用正整数。低于 `100000` 的值会被提高到最低阈值 `100000`。

在默认值下，使用 200K 上下文窗口的对话会在达到 200,000 个 token 之前自动压缩。此示例将阈值降低到 150,000，以便这些对话符合条件：

```json settings.json theme={null}
{
  "env": {
    "CLAUDE_CODE_IDLE_COMPACT_MIN_TOKENS": "150000"
  }
}
```

#### VS Code 和桌面应用中的空闲压缩

类似下面这样的通知表示 Claude Code 已[用摘要替换了您之前的对话](https://code.claude.com/docs/zh-CN/context-window.md#what-survives-compaction)，Claude 现在基于该摘要继续工作：

```text theme={null}
Compacted using a summary written while you were away · turn off with "idleCompaction": false in settings.json
```

您无需执行任何操作。要关闭空闲压缩，请在运行该会话的计算机或 WSL 发行版上的 `~/.claude/settings.json` 中，或在其他[设置文件](https://code.claude.com/docs/zh-CN/settings.md#settings-files-and-who-they-affect)中，将 `idleCompaction` 设置为 `false`。

### `plansDirectory`

选择 Claude Code 存储其在[计划模式](https://code.claude.com/docs/zh-CN/permission-modes.md#analyze-before-you-edit-with-plan-mode)中编写的计划文件的位置。Claude Code 会相对于项目根目录解析该路径。

* **作用域**：[`Any file`](#scopes)
* **类型**：字符串，相对于项目根目录的路径
* **默认值**：未设置，此时 Claude Code 使用 `~/.claude/plans`

```json settings.json theme={null}
{
  "plansDirectory": "./plans"
}
```

在以下情况下，Claude Code 会将计划存储在 `~/.claude/plans` 中，而不是您设置的目录中：

* **位于项目根目录之外**：路径解析到项目根目录之外，例如 `"../plans"`。
* **在 macOS、Linux 和 WSL 上包含反斜杠**：解析后的路径包含反斜杠，例如 Windows 风格的 `"docs\\plans"`。请写成 `"docs/plans"`，它在 Windows 上同样适用。

### `skillListingBudgetFraction`

每一轮中，Claude 都会看到一份[skill 列表](https://code.claude.com/docs/zh-CN/skills.md#skill-descriptions-are-cut-short)及其描述，Claude Code 会将该列表限制在上下文窗口的一定比例以内。当列表超出上限时，Claude Code 会保留每个 skill 的名称，但会删除最少使用的 skill 的描述，因此 Claude 仍可调用这些 skill，但不太可能主动选择它们。提高此键可保留更多可见的描述，代价是每轮占用更多上下文。

* **作用域**：[`Any file`](#scopes)
* **类型**：数字，大于 `0` 且不超过 `1` 的比例
* **默认值**：`0.01`，即预留上下文窗口的 1%

```json settings.json theme={null}
{
  "skillListingBudgetFraction": 0.02
}
```

要查看该列表占用了多少上下文以及哪些 skill 占用最多，请运行 `/doctor`。

### `skillListingMaxDescChars`

每一轮中，Claude 都会看到一份[skill 列表](https://code.claude.com/docs/zh-CN/skills.md#skill-descriptions-are-cut-short)，其中显示每个 skill 的 `description` 和 `when_to_use` 文本。此键限制 Claude Code 为每个 skill 显示的该文本的字符数；更长的文本会在上限处截断。

* **作用域**：[`Any file`](#scopes)
* **类型**：字符数，正整数
* **默认值**：`1536`

```json settings.json theme={null}
{
  "skillListingMaxDescChars": 2048
}
```

提高此值可保持长描述完整，代价是每轮占用更多上下文；降低此值可在 [`skillListingBudgetFraction`](#skilllistingbudgetfraction) 限制内容纳更多 skill。

### `taskOutputMaxChars`

> [!WARNING]
>
> 已在 v2.1.277 中移除，同时移除的还有它所控制大小的 `TaskOutput` 工具。在当前版本中设置它没有任何效果。Claude 现在改用 `Read` 读取后台任务的[输出文件](https://code.claude.com/docs/zh-CN/tools-reference.md#background-commands)。

在 v2.1.276 及之前的版本中，您可以将此键设置为 Claude 使用 `TaskOutput` 工具读取[后台任务](https://code.claude.com/docs/zh-CN/tools-reference.md#background-commands)时以内联方式接收的输出字符数。

## 界面与终端

更改 Claude Code 在终端中的外观和行为：主题、编辑器模式、状态栏、加载指示器、会话内通知以及无障碍功能。请参阅[终端配置](https://code.claude.com/docs/zh-CN/terminal-config.md)。

### `askUserQuestionTimeout`

让未回答的 [`AskUserQuestion`](https://code.claude.com/docs/zh-CN/tools-reference) 对话框在空闲一段时间后自动继续，并提交您已选择的选项。当您暂时离开并希望 Claude 在没有您的情况下继续时，可设置此项。使用默认值时，问题会一直等待您回答。关于计时器何时暂停或从不启动，请参阅[问题自动继续超时](https://code.claude.com/docs/zh-CN/tools-reference.md#question-auto-continue-timeout)。

* **作用域**：[`User or managed`](#scopes)
* **类型**：字符串，取值为 `"60s"`、`"5m"`、`"10m"` 或 `"never"` 之一
* **默认值**：`"never"`

```json settings.json theme={null}
{
  "askUserQuestionTimeout": "5m"
}
```

在 `/config` 中显示为 **Question auto-continue timeout**，该选项会将此键写入用户设置；当托管设置或 `--settings` 标志设置了此键时，Claude Code 会隐藏该行。

### `autoContinueAtUsageLimit`

当 claude.ai 用量限制中止您的会话后，在打开的会话中等待，并在重置后自动继续任务。请参阅[关闭自动继续](https://code.claude.com/docs/zh-CN/interactive-mode.md#turn-automatic-continue-off)。需要 Claude Code v2.1.234 或更高版本。

* **作用域**：[`User or managed`](#scopes)。仅从用户设置、`--settings` 和托管设置中读取。当这些都未设置此键时，设置了此键的项目或本地设置文件会关闭此功能，而不是被忽略。
* **类型**：布尔值
  * `true`：当 claude.ai 用量限制中止您的会话后，Claude Code 会在打开的会话中等待，并在重置后自动继续任务
  * `false`：Claude Code 不会自行开始等待。您仍然可以从用量限制选项菜单中[自行开始等待](https://code.claude.com/docs/zh-CN/interactive-mode.md#start-a-wait-yourself)
* **默认值**：`true`

```json settings.json theme={null}
{
  "autoContinueAtUsageLimit": false
}
```

在 `/config` 中显示为 **Continue automatically at usage limit**，该选项会将此键写入用户设置；当托管设置或 `--settings` 标志设置了此键时，Claude Code 会隐藏该行。

### `autoScrollEnabled`

在[全屏渲染](https://code.claude.com/docs/zh-CN/fullscreen.md)中，跟随新输出滚动到对话底部。关闭此项后，Claude 继续工作时视图会停留在您滚动到的位置；权限提示仍会滚动到视野中。

* **作用域**：[`Any file`](#scopes)
* **类型**：布尔值
  * `true`：对话跟随新输出滚动到底部
  * `false`：Claude 继续工作时视图停留在您滚动到的位置；权限提示仍显示在会话记录下方
* **默认值**：`true`

```json settings.json theme={null}
{
  "autoScrollEnabled": false
}
```

开启全屏渲染时，在 `/config` 中显示为 **Auto-scroll**，该选项会将此键写入用户设置。

### `axScreenReader`

渲染适合屏幕阅读器的输出：不带装饰性边框或动画的纯文本。屏幕阅读器模式使用经典渲染器，因此在该模式启用时 `tui` 设置无效；附加的[后台会话](https://code.claude.com/docs/zh-CN/agent-view.md)仍以全屏方式渲染。

* **作用域**：[`User or managed`](#scopes)。您也可以在通过 `--settings` 传入的文件中设置此键。Claude Code 会忽略项目设置和本地设置中的此键，因此无论是仓库还是您自己的 `.claude/settings.local.json` 都无法开启或关闭屏幕阅读器模式。
* **类型**：布尔值
  * `true`：Claude Code 使用经典渲染器，渲染不带装饰性边框或动画的纯文本
  * `false`：Claude Code 正常渲染
* **默认值**：未设置，因此屏幕阅读器模式关闭
* **单会话覆盖**：[`--ax-screen-reader`](https://code.claude.com/docs/zh-CN/cli-reference#cli-flags) 优先于 [`CLAUDE_AX_SCREEN_READER`](https://code.claude.com/docs/zh-CN/env-vars)，两者在单个会话中均优先于此键

```json settings.json theme={null}
{
  "axScreenReader": true
}
```

### `bashEditDiffEnabled`

选择 Claude Code 是否记录 Bash 命令运行期间 Git 仓库中哪些文件发生了更改。记录时，您会在命令执行后于终端中看到这些文件的 diff，并且您的 [PostToolUse Bash hook](https://code.claude.com/docs/zh-CN/hooks.md#bash) 会收到已更改文件列表。

列出的文件不一定是该命令更改的。在命令运行期间由其他程序或其他 Bash 调用所做的更改也可能出现在其中。

将此键设置为 `true` 可在所有权限模式下记录这些文件。需要 Claude Code v2.1.269 或更高版本。

* **作用域**：[`User or managed`](#scopes)。`true` 仅在来自用户设置、通过 `--settings` 传入的 JSON 或[托管设置](https://code.claude.com/docs/zh-CN/managed-settings.md)时生效，因此仓库的 `.claude/settings.json` 或 `.claude/settings.local.json` 中的 `true` 无法开启记录。但这两个仓库文件中的 `false` 仍会关闭记录，除非某个[优先级更高](https://code.claude.com/docs/zh-CN/settings.md#settings-precedence)的文件设置为 `true`。
* **类型**：布尔值
* **默认值**：未设置，因此 Claude Code 在自动模式和 `bypassPermissions` 模式下，当其指示 Claude 通过 Bash 编辑文件时记录更改
* **单会话覆盖**：[`CLAUDE_CODE_BASH_EDIT_DIFF`](https://code.claude.com/docs/zh-CN/env-vars) 在单个会话中优先于此键

```json settings.json theme={null}
{
  "bashEditDiffEnabled": true
}
```

### `companyAnnouncements`

在启动时向用户显示您组织的公告。当您列出多条公告时，Claude Code 会为每个会话随机选择一条；在用户首次启动时则显示第一条。

* **作用域**：[`Any file`](#scopes)
* **类型**：字符串数组
* **默认值**：未设置，因此不显示公告

```json settings.json theme={null}
{
  "companyAnnouncements": [
    "Welcome to Acme Corp! Review our code guidelines at docs.example.com"
  ]
}
```

### `defaultShell`

选择由 Bash 还是 PowerShell 来运行您在输入框中使用 [`!` 前缀](https://code.claude.com/docs/zh-CN/interactive-mode#shell-mode-with-prefix)键入的 shell 命令，即 Claude Code 直接运行并添加到会话中的命令。

`"powershell"` 仅在 [PowerShell 工具](https://code.claude.com/docs/zh-CN/tools-reference.md#powershell-tool)开启时有效。在未安装 Git Bash 的 Windows 上，该工具默认开启；在安装了 Git Bash 的 Windows 上，对于 claude.ai 和 Console 账户，该工具也默认开启。在 Amazon Bedrock、Google Cloud 的 Agent Platform 和 Microsoft Foundry 会话中，以及在 macOS、Linux 和 WSL 上，设置 `CLAUDE_CODE_USE_POWERSHELL_TOOL=1` 可开启该工具。将该变量设置为 `0` 可关闭该工具。

* **作用域**：[`Any file`](#scopes)
* **类型**：字符串，取值为以下之一：
  * `"bash"`：Claude Code 在 Bash 中运行您的 `!` 命令
  * `"powershell"`：Claude Code 在 PowerShell 中运行您的 `!` 命令
* **默认值**：`"bash"`；在 Windows 上 Bash 不可用时为 `"powershell"`

```json settings.json theme={null}
{
  "defaultShell": "powershell"
}
```

如果您指定的 shell 不可用，Claude Code 会使用另一个：当 PowerShell 工具关闭时，`"powershell"` 回退到 Bash；当未安装 Bash 时，`"bash"` 回退到 PowerShell。

### `dialogExpiry`

为 Claude Code [转发到远程客户端](https://code.claude.com/docs/zh-CN/remote-control.md#limitations)（例如 Remote Control 或 SDK 主机）的对话框，以及[被暂留的跨会话消息](https://code.claude.com/docs/zh-CN/cross-session-messaging.md#control-inbound-messages)的批准对话框设置截止时间。在 Claude Code v2.1.236 或更高版本中，同一截止时间也适用于在可能无人值守终端的会话中出现的会话中途 [Fable 使用额度同意提示](https://code.claude.com/docs/zh-CN/model-config.md#fable-and-usage-credits)。如果在截止时间之前未收到答复，Claude Code 会取消该对话框，并按其不执行操作的默认行为继续。需要 Claude Code v2.1.224 或更高版本。

* **作用域**：[`User or managed`](#scopes)
* **类型**：字符串，取值为 `"60s"`、`"5m"`、`"10m"` 或 `"never"`（禁用截止时间）之一
* **默认值**：`"5m"`
* **单会话覆盖**：[`CLAUDE_CODE_USER_DIALOG_TIMEOUT_MS`](https://code.claude.com/docs/zh-CN/env-vars) 在单个会话中优先于此键

```json settings.json theme={null}
{
  "dialogExpiry": "10m"
}
```

权限提示和 [`AskUserQuestion`](https://code.claude.com/docs/zh-CN/tools-reference#askuserquestion-tool-behavior) 问题使用各自的流程，不受此截止时间约束。在 `/config` 中显示为 **Dialog expiry**，该选项会将此键写入用户设置；该行需要 Claude Code v2.1.232 或更高版本，并且当托管设置或 `--settings` 标志设置了此键时，Claude Code 会隐藏该行。

### `editorMode`

选择输入框的按键绑定模式。

* **作用域**：[`Any file`](#scopes)
* **类型**：字符串，取值为以下之一：
  * `"normal"`：输入框中的标准按键绑定
  * `"vim"`：带有 NORMAL、INSERT 和 VISUAL 模式的 vim 风格编辑
* **默认值**：`"normal"`

```json settings.json theme={null}
{
  "editorMode": "vim"
}
```

在 `/config` 中显示为 **Editor mode**，该选项会将此键写入用户设置。

### `emojiCompletionEnabled`

在输入框中键入 `:` 加短代码时显示 emoji 建议，并将完整的短代码（例如 `:heart:`）替换为对应的 emoji。设置为 `false` 可同时关闭这两项功能。

* **作用域**：[`Any file`](#scopes)
* **类型**：布尔值
  * `true`：Claude Code 在 `:` 之后显示 emoji 建议，并将完整的短代码替换为对应的 emoji
  * `false`：Claude Code 既不建议 emoji，也不替换短代码
* **默认值**：`true`

```json settings.json theme={null}
{
  "emojiCompletionEnabled": false
}
```

请参阅 [Emoji 短代码](https://code.claude.com/docs/zh-CN/interactive-mode.md#emoji-shortcodes)。

### `fileSuggestion`

运行您自己的命令来提供 `@` 文件路径自动补全，以替代内置的文件建议。内置建议使用快速文件系统遍历；对于大型 monorepo，使用项目特定的索引（例如预构建的文件索引）可能效果更好。

* **作用域**：[`Any file`](#scopes)。在[状态栏和文件建议限制](#status-line-and-file-suggestion-gates)下，Claude Code 会关闭该命令或仅运行托管值，并在不发出警告的情况下跳过您的命令。
* **类型**：对象，包含 `type`（始终为 `"command"`）和 `command`（要运行的 shell 命令）
* **默认值**：未设置，因此 Claude Code 使用内置的文件建议

```json settings.json theme={null}
{
  "fileSuggestion": {
    "type": "command",
    "command": "~/.claude/file-suggestion.sh"
  }
}
```

保存后，在输入框中键入 `@` 并跟上部分路径：建议将来自您命令的输出。

#### 命令输入和输出

Claude Code 使用与 [hook](https://code.claude.com/docs/zh-CN/hooks.md) 相同的环境变量（包括 `CLAUDE_PROJECT_DIR`）运行该命令，并在五秒后停止等待。该命令通过 stdin 接收 JSON，其中 `query` 字段包含您目前已键入的内容：

```json theme={null}
{"query": "src/comp"}
```

将以换行符分隔的文件路径输出到 stdout。Claude Code 最多显示 15 个：

```text theme={null}
src/components/Button.tsx
src/components/Modal.tsx
src/components/Form.tsx
```

以下脚本读取查询并将其传递给仓库文件索引：

```bash theme={null}
#!/bin/bash
query=$(cat | jq -r '.query')
# Replace your-repo-file-index with your own file search command
your-repo-file-index --query "$query" | head -20
```

### `footerLinksRegexes`

当正则表达式匹配轮次输出（包括文件内容和获取的页面在内的工具结果，以及 Claude 自己的回复）时，在输入框下方的页脚中渲染额外的可点击徽章。可用它将项目 CLI（例如审查工具和问题跟踪器）打印的 ID 转换为会话中的链接。

* **作用域**：[`User or managed`](#scopes)
* **类型**：对象数组，每个对象包含设置为 `"regex"` 的 `type`、一个 `pattern` 正则表达式、一个 `url` 模板以及可选的 `label`；`url` 和 `label` 中的 `{name}` 占位符由 `pattern` 中的命名捕获组填充
* **默认值**：未设置，因此不渲染徽章

此示例匹配 `PROJ-1234` 之类的问题键，并根据捕获的键构建每个链接：

```json settings.json theme={null}
{
  "footerLinksRegexes": [
    {
      "type": "regex",
      "pattern": "\\b(?<key>PROJ-\\d+)\\b",
      "url": "https://issues.example.com/browse/{key}",
      "label": "{key}"
    }
  ]
}
```

配置此项后，当 `PROJ-1234` 出现在工具结果或 Claude 的回复中时，页脚中会出现一个链接到 `https://issues.example.com/browse/PROJ-1234` 的 `PROJ-1234` 徽章。

#### 徽章限制

每个条目的 URL、标签和徽章数量受以下限制：

| 限制 | 行为 |
| :- | :- |
| URL 源 | 捕获的值会进行 URL 编码，并且构建的 URL 必须与模板的字面源相同。捕获值可以填充路径段或查询值，但不能更改链接指向的位置 |
| URL 长度 | 超过 2048 个字符的构建 URL 会被丢弃 |
| URL 协议 | 必须是 `https`、`http` 或可识别的编辑器或工作区深层链接协议：`vscode`、`vscode-insiders`、`cursor`、`windsurf`、`zed`、`jetbrains`、`idea`、`slack`、`linear`、`notion`、`figma` |
| 标签 | 默认为匹配的文本，并截断为 28 个显示列 |
| 徽章数量 | 最多渲染 5 个徽章。最旧的徽章会被较新的匹配项替换，`/clear` 会将其移除 |

轮次完成时，Claude Code 会在主线程上将每个条目的 `pattern` 正则表达式与轮次输出进行匹配，因此缓慢的正则表达式会阻塞 UI 直至完成。诸如 `(a+)+$` 之类的嵌套量词在某些输入下可能耗费指数级时间并冻结会话，因此请保持每个 `pattern` 为线性，并避免嵌套 `+` 或 `*`。

配置了[自定义状态栏](https://code.claude.com/docs/zh-CN/statusline.md)时，页脚徽章会与其一同渲染；两者互不替代。状态栏适用于由脚本驱动、根据会话数据计算自身内容的行，页脚徽章则适用于无需脚本即可将对话中的 ID 转换为链接。

### `keybindingFlavor`

> [!WARNING]
>
> 自 v2.1.261 起已弃用，且不再生效。输入框的单词编辑按键始终[遵循 readline 约定](https://code.claude.com/docs/zh-CN/interactive-mode.md#make-ctrl-w-delete-back-to-whitespace)，与 Bash 中相同。Claude Code 仍然接受 `keybindingFlavor`，因此设置了此项的设置文件仍然有效。

在 v2.1.238 到 v2.1.260 中，将其设置为 `"readline"` 会使 `Ctrl+W` 向后删除到上一个空白字符，而不仅仅是上一个单词。

* **作用域**：[`Any file`](#scopes)
* **类型**：字符串，`"classic"` 或 `"readline"`
* **默认值**：未设置

### `maxProseWidth`

限制 Claude 回复中正文的宽度，使其在宽终端中保持易读。段落、标题、列表和引用块会在此列数内换行，而表格和代码块保持完整的终端宽度。需要 Claude Code v2.1.282 或更高版本。

* **作用域**：[`Any file`](#scopes)
* **类型**：终端列数，整数，最小值为 `40`。Claude Code 会忽略任何其他值
* **默认值**：未设置，因此正文在终端边缘换行

```json settings.json theme={null}
{
  "maxProseWidth": 80
}
```

### `prefersReducedMotion`

减少或关闭界面动画，例如加载指示器、闪光和闪烁效果。在 `/config` 中显示为 **Reduce motion**。

* **作用域**：[`Any file`](#scopes)
* **类型**：布尔值
  * `true`：Claude Code 减少或关闭界面动画，例如加载指示器、闪光和闪烁效果
  * `false`：与未设置相同；Claude Code 显示其动画
* **默认值**：`false`

```json settings.json theme={null}
{
  "prefersReducedMotion": true
}
```

### `promptSuggestionEnabled`

显示或隐藏[提示词建议](https://code.claude.com/docs/zh-CN/interactive-mode.md#prompt-suggestions)，即出现在输入框中的灰色预测内容。将其设置为 `false`，或在 `/config` 中关闭 **Prompt suggestions**，即可隐藏它们。

* **作用域**：[`Any file`](#scopes)
* **类型**：布尔值
  * `true`：您会在输入框中看到提示词建议
  * `false`：Claude Code 隐藏提示词建议
* **默认值**：`true`
* **单会话覆盖**：[`CLAUDE_CODE_ENABLE_PROMPT_SUGGESTION`](https://code.claude.com/docs/zh-CN/env-vars) 在单个会话中优先于此键

```json settings.json theme={null}
{
  "promptSuggestionEnabled": false
}
```

提示词建议需要开启了遥测的 claude.ai 或 Console 账户。在 Amazon Bedrock、Google Cloud 的 Agent Platform 和 Microsoft Foundry 上，或者在遥测已关闭（例如通过 [`DISABLE_TELEMETRY`](https://code.claude.com/docs/zh-CN/env-vars)）的情况下，此键无效，只有 `CLAUDE_CODE_ENABLE_PROMPT_SUGGESTION=1` 能开启提示词建议。

### `respectGitignore`

控制 `@` 文件选择器是否排除匹配 `.gitignore` 模式的文件。在 `/config` 中显示为 **Respect .gitignore in file picker**。

* **作用域**：[`Any file`](#scopes)。当没有任何设置文件设置此项时，Claude Code 会回退到 `~/.claude.json` 中的 `respectGitignore`，该值由 `/config` 开关写入。
* **类型**：布尔值
  * `true`：`@` 文件选择器排除匹配 `.gitignore` 模式的文件
  * `false`：`@` 文件选择器包含匹配 `.gitignore` 模式的文件
* **默认值**：`true`

```json settings.json theme={null}
{
  "respectGitignore": false
}
```

### `respondToBashCommands`

选择在您于输入框中使用 [`!` 前缀](https://code.claude.com/docs/zh-CN/interactive-mode#shell-mode-with-prefix)运行 shell 命令后，Claude 是否进行回复。默认情况下，Claude Code 会将命令的输出添加到对话中，并由 Claude 对其进行回复。将此键设置为 `false` 可将输出添加到上下文而不回复，这样您就可以运行多个命令，然后一起询问。

* **作用域**：[`Any file`](#scopes)
* **类型**：布尔值
  * `true`：Claude Code 将命令的输出添加到对话中，并由 Claude 对其进行回复
  * `false`：Claude Code 将输出添加到上下文而不回复
* **默认值**：`true`

```json settings.json theme={null}
{
  "respondToBashCommands": false
}
```

请参阅[使用 `!` 前缀的 shell 模式](https://code.claude.com/docs/zh-CN/interactive-mode#shell-mode-with-prefix)。

### `showClearContextOnPlanAccept`

当 Claude 在[计划模式](https://code.claude.com/docs/zh-CN/permission-modes.md#review-and-approve-a-plan)中完成计划时，会显示一个批准菜单。制定计划可能会占用大量上下文，因此此键会在该菜单中添加第一个选项 **Yes, clear context and …**，用于批准计划、清除对话上下文，并仅依据计划开始实施。标签的其余部分会说明会话将继续使用的权限模式，并显示计划占用了多少上下文。

* **作用域**：[`Any file`](#scopes)
* **类型**：布尔值
  * `true`：计划批准菜单获得第一个选项 **Yes, clear context and …**，用于批准计划并清除对话上下文
  * `false`：计划批准菜单不显示清除上下文选项
* **默认值**：`false`

```json settings.json theme={null}
{
  "showClearContextOnPlanAccept": true
}
```

### `showTurnDuration`

显示或隐藏每次回复后的轮次耗时消息，例如 "Cooked for 1m 6s · done 6:05 PM"。"done" 后面的时钟显示轮次完成的时间；[`timeFormat`](#timeformat) 和 [`timeZone`](#timezone) 控制其格式和时区。在 `/config` 中显示为 **Show turn duration**。

* **作用域**：[`Any file`](#scopes)。当没有任何设置文件设置此项时，旧版本写入 `~/.claude.json` 的值会生效。
* **类型**：布尔值
  * `true`：您会在每次回复后看到轮次耗时消息
  * `false`：Claude Code 隐藏轮次耗时消息
* **默认值**：`true`

```json settings.json theme={null}
{
  "showTurnDuration": false
}
```

### `spellcheck`

在您键入时，使用您安装的拼写检查器为输入框中的拼写错误单词添加下划线。Claude Code 仅检查输入框中的文本。[键入时检查拼写](https://code.claude.com/docs/zh-CN/interactive-mode.md#check-spelling-as-you-type)介绍了如何安装 aspell、hunspell 或 ispell，以及检查器的检查范围。需要 Claude Code v2.1.235 或更高版本。

* **作用域**：[`User or managed`](#scopes)。来自设置了此项的最高层级的整个块作为一个整体生效。
* **类型**：对象，包含 `enabled`（布尔值）、`checker`（`"aspell"`、`"hunspell"`、`"ispell"` 或 `"auto"`）、`language`（字符串，作为词典名称传递给检查器）和 `color`（字符串，终端颜色名称、`#rrggbb`、`rgb(r,g,b)`、`ansi256(n)` 或 `ansi:<name>`）
* **默认值**：未设置，因此拼写检查关闭；`checker` 默认为 `"auto"`，即在 `PATH` 中找到的三者中的第一个；`language` 默认为检查器自己的词典；`color` 默认为主题的错误颜色

```json settings.json theme={null}
{
  "spellcheck": { "enabled": true, "language": "en_GB" }
}
```

### `spinnerTipsEnabled`

在 Claude 工作时，加载指示器行会轮流显示关于 Claude Code 功能的简短提示，例如 "Use Plan Mode to prepare for a complex request before making changes. Press Shift+Tab twice to enable."。将此键设置为 `false` 可隐藏这些提示。在 `/config` 中显示为 **Show tips**。

* **作用域**：[`Any file`](#scopes)
* **类型**：布尔值
  * `true`：Claude 工作时，您会在加载指示器中看到提示
  * `false`：Claude Code 隐藏加载指示器提示
* **默认值**：`true`

```json settings.json theme={null}
{
  "spinnerTipsEnabled": false
}
```

### `spinnerTipsOverride`

向 Claude 工作时 Claude Code 显示的[加载指示器提示](#spinnertipsenabled)中添加您自己的提示，或用您的提示替换内置提示。Claude Code 会将您的提示与内置提示放在同一轮换中。

如果您将 [`spinnerTipsEnabled`](#spinnertipsenabled) 设置为 `false`，Claude Code 会隐藏所有提示，包括您的提示。

* **作用域**：[`Any file`](#scopes)。Claude Code 接受来自用户设置、`--settings` 标志和托管设置的提示对象、`tipsFile`、`label` 和 `excludeDefault`；从项目和本地设置中仅读取纯字符串提示。
* **类型**：对象，包含 `tips`、`tipsFile`、`label` 和 `excludeDefault` 字段，均为可选
* **默认值**：未设置，因此 Claude Code 仅显示内置提示

提示对象、`tipsFile`、`label`，以及作用域说明中项目和本地设置仅提供纯字符串的规则，需要 Claude Code v2.1.247 或更高版本。

每个 `tips` 条目是一个纯字符串或包含以下字段的对象：

| 字段 | 必填 | 描述 |
| :- | :- | :- |
| `id` | 是 | 最多 64 个字母、数字、`.`、`_` 或 `-`。Claude Code 以此记录提示的显示历史，因此重新排列列表后提示的冷却期仍然保留。若两个条目 id 相同，Claude Code 使用第一个 |
| `text` | 是 | 提示内容，单行，最多 500 个字符。Claude Code 会去除 ANSI 转义序列和控制字符，并合并空白字符 |
| `cooldownSessions` | 否 | Claude Code 再次显示该提示前等待的会话数，`0` 到 `1000`，默认为 `0` |
| `priority` | 否 | 在未显示时长相同的提示之间的顺序，值越高越靠前，`-10` 到 `10`，默认为 `0` |

Claude Code 将纯字符串视为使用上述默认值和基于位置的 id 的提示，因此重新排列列表时其显示历史会重置。为提示指定 `id` 可在编辑时保留其历史。

Claude Code 在 `tips` 和 `tipsFile` 中总共最多读取 200 条提示，并会丢弃无效条目并输出调试警告，而不是拒绝整个设置文件。

使用其余字段来指定提示文件、设置前缀以及隐藏内置提示：

* `tipsFile`：指向本地 JSON 文件的绝对路径或 `~/` 路径，该文件包含由相同条目组成的数组，或包含 `tips` 数组的对象，最大 256 KB。Claude Code 每个进程只读取该文件一次，因此会在下次启动时加载您的修改。您不能通过[服务器托管设置](https://code.claude.com/docs/zh-CN/server-managed-settings.md)设置此项；请在那里部署内联的 `tips`，或在磁盘上的 `managed-settings.json` 中部署该路径。
* `label`：Claude Code 在来自用户设置、`--settings` 和托管设置的提示前显示的前缀，最多 40 个字符。默认为 `Tip`，与内置提示的前缀相同，来自项目和本地设置的提示始终使用该默认前缀。
* `excludeDefault`：设置为 `true` 可隐藏内置提示，仅显示您的提示。当 Claude Code 无法加载您的任何提示时（例如 `tipsFile` 不存在或所有条目均无效），它会保留内置轮换，而不是显示空的加载指示器。

当多个设置文件设置了此键时，Claude Code 会显示所有这些文件中的提示，并从托管设置、`--settings` 标志和用户设置中设置了相应字段且优先级最高的那个获取 `tipsFile`、`label` 和 `excludeDefault`。

此示例位于您的用户设置中，在 `Acme tip` 前缀下向轮换中添加一个纯字符串提示和一个对象提示：

```json settings.json theme={null}
{
  "spinnerTipsOverride": {
    "label": "Acme tip",
    "tips": [
      "Run /review before opening a PR",
      {
        "id": "gateway-errors",
        "text": "Seeing 5xx errors? Check the gateway status page first",
        "cooldownSessions": 5,
        "priority": 2
      }
    ]
  }
}
```

示例中的每个字段都会改变 Claude Code 显示提示的一个方面：

* `label`：Claude Code 将两个提示都显示为 `Acme tip: ...`，而不是 `Tip: ...`。
* 纯字符串：Claude Code 为其使用默认值，因此它可能在紧接着的下一个会话中再次出现。
* `id`：Claude Code 以 `gateway-errors` 记录第二个提示的显示历史，因此在您添加或重新排列提示后，其冷却期仍然适用。
* `cooldownSessions`：Claude Code 显示 `gateway-errors` 提示后，要等到五个会话之后才会再次显示该提示。
* `priority`：当 `gateway-errors` 提示与另一个提示未显示的会话数相同时（例如两者都尚未显示过），Claude Code 会先显示 `gateway-errors`。纯字符串使用默认优先级 `0`。

Claude 工作时，Claude Code 会在加载指示器中以您的前缀显示您的提示，例如 `Acme tip: Run /review before opening a PR`。

### `spinnerVerbs`

轮次进行期间，加载指示器会轮流显示动词，例如 "Accomplishing"、"Architecting" 或 "Baking"。使用此键可将您自己的动词添加到该轮换中，或用您的动词替换内置列表。

* **作用域**：[`Any file`](#scopes)
* **类型**：对象，包含字符串数组 `verbs` 和 `mode`，`mode` 取值为以下之一：
  * `"append"`：Claude Code 将您的动词添加到内置集合中
  * `"replace"`：Claude Code 仅显示您的动词
* **默认值**：未设置，因此 Claude Code 使用内置动词

此示例向内置集合中添加两个动词：

```json settings.json theme={null}
{
  "spinnerVerbs": {
    "mode": "append",
    "verbs": ["Pondering", "Crafting"]
  }
}
```

在 `"replace"` 模式下，如果 `verbs` 数组为空，Claude Code 会保留内置动词。

### `statusLine`

运行您自己的命令，在输入框下方渲染[状态栏](https://code.claude.com/docs/zh-CN/statusline.md)，显示模型、费用或 git 分支等上下文信息。可选字段用于调整间距、添加定期重新运行，以及在您的脚本自行渲染 `vim.mode` 时隐藏内置的 vim 模式指示器。

* **作用域**：[`Any file`](#scopes)。当 [`allowManagedHooksOnly`](#allowmanagedhooksonly) 开启，或在托管设置之外设置了 [`disableAllHooks`](#disableallhooks) 时，仅运行托管设置中的值。
* **类型**：对象，包含设置为 `"command"` 的 `type` 和 `command` 字符串，以及可选的 `padding`（字符数）、`refreshInterval`（秒数，最小值为 `1`）和 `hideVimModeIndicator`（布尔值）
* **默认值**：未设置，因此不显示状态栏

此示例打印模型名称和上下文使用情况，并添加两个字符的水平间距：

```json settings.json theme={null}
{
  "statusLine": {
    "type": "command",
    "command": "jq -r '\"[\\(.model.display_name)] \\(.context_window.used_percentage // 0)% context\"'",
    "padding": 2
  }
}
```

该示例需要安装 [`jq`](https://jqlang.org/) 并在 shell 中运行。有关 PowerShell 和 Git Bash 的等效写法，请参阅 [Windows 配置](https://code.claude.com/docs/zh-CN/statusline.md#windows-configuration)；有关完整设置，请参阅[手动配置状态栏](https://code.claude.com/docs/zh-CN/statusline.md#manually-configure-a-status-line)。

### `subagentStatusLine`

当 Claude 运行[子代理](https://code.claude.com/docs/zh-CN/sub-agents.md)时，Claude Code 会在输入框下方的任务显示区域中列出它们，每个子代理占一行，显示 `name · description · token count`。此键允许您运行自己的命令来改写这些行，例如以百分比显示每个子代理的上下文使用情况。每次刷新时，Claude Code 会通过 stdin 发送一个 JSON 对象来表示可见的行，其中 `tasks` 数组包含每个子代理的 `id`、`name`、`status`、`model`、`tokenCount` 等信息；对于您以 `{"id", "content"}` 行形式写回的每个 `id`，Claude Code 会替换相应的行。您未写回的行保持默认渲染。

* **作用域**：[`Any file`](#scopes)。当 [`allowManagedHooksOnly`](#allowmanagedhooksonly) 开启，或在托管设置之外设置了 [`disableAllHooks`](#disableallhooks) 时，仅运行托管设置中的值。
* **类型**：对象，包含设置为 `"command"` 的 `type` 和 `command` 字符串
* **默认值**：未设置，因此 Claude Code 渲染默认行

```json settings.json theme={null}
{
  "subagentStatusLine": {
    "type": "command",
    "command": "jq -c '.tasks[] | {id, content: \"\\(.name): \\(.tokenCount) tokens\"}'"
  }
}
```

请参阅[子代理状态栏](https://code.claude.com/docs/zh-CN/statusline.md#subagent-status-lines)。

### `syntaxHighlightingDisabled`

Claude Code 使用其内置的高亮器，按语言为其在终端中显示的 diff、代码块和文件预览中的代码着色；不涉及任何插件或语言服务器。将此键设置为 `true` 可改为以纯文本显示，例如当颜色与您的终端主题冲突或拖慢屏幕阅读器时。

* **作用域**：[`Any file`](#scopes)
* **类型**：布尔值
  * `true`：Claude Code 关闭 diff、代码块和文件预览中的语法高亮
  * `false`：Claude Code 进行语法高亮
* **默认值**：`false`

```json settings.json theme={null}
{
  "syntaxHighlightingDisabled": true
}
```

### `terminalProgressBarEnabled`

某些终端可以在标签页或任务栏上为其中运行的程序显示进度指示器。Claude 工作时，Claude Code 会向终端报告进行中状态，因此您可以从其他标签页或窗口查看会话是否仍在忙碌。当[后台子代理](https://code.claude.com/docs/zh-CN/sub-agents.md#run-subagents-in-foreground-or-background)或[动态工作流](https://code.claude.com/docs/zh-CN/workflows.md)仍在运行时，指示器在轮次结束后仍保持可见，并在会话空闲后清除。

Claude Code 仅在支持该指示器的终端中报告：ConEmu、Ghostty 1.2.0 或更高版本，以及 iTerm2 3.6.6 或更高版本。将此键设置为 `false` 可阻止 Claude Code 报告。在 `/config` 中显示为 **Terminal progress bar**。

* **作用域**：[`Any file`](#scopes)。当没有任何设置文件设置此项时，旧版本写入 `~/.claude.json` 的值会生效。
* **类型**：布尔值
  * `true`：您会在支持的终端中看到终端进度条
  * `false`：Claude Code 隐藏终端进度条
* **默认值**：`true`

```json settings.json theme={null}
{
  "terminalProgressBarEnabled": false
}
```

### `terminalTitleFromRename`

Claude Code 会设置您的终端标签页标题。默认情况下，它使用根据对话生成的标题；一旦您使用 `/rename` 或 `--name` 为会话[命名](https://code.claude.com/docs/zh-CN/sessions.md#name-your-sessions)，标签页就会改为显示该名称。将此键设置为 `false` 可在您为会话命名后仍在标签页上保留生成的标题。名称本身仍然生效，因此 `/resume <name>` 和会话选择器都能找到它。

* **作用域**：[`Any file`](#scopes)
* **类型**：布尔值
  * `true`：终端标签页标题显示您设置的会话名称
  * `false`：标签页保留 Claude Code 根据您的对话生成的标题
* **默认值**：`true`

```json settings.json theme={null}
{
  "terminalTitleFromRename": false
}
```

如需完全阻止 Claude Code 更新终端标题，请改为将 [`CLAUDE_CODE_DISABLE_TERMINAL_TITLE`](https://code.claude.com/docs/zh-CN/env-vars) 设置为 `1`。

### `theme`

选择界面的颜色主题。在 `/config` 中显示为 **Theme**。

* **作用域**：[`Any file`](#scopes)。当没有任何设置文件设置此项时，旧版本写入 `~/.claude.json` 的值会生效。
* **类型**：字符串，取值为以下之一：
  * `"auto"`：匹配终端的浅色或深色背景
  * `"dark"`：深色主题
  * `"light"`：浅色主题
  * `"dark-daltonized"`：使用色盲友好颜色的深色主题
  * `"light-daltonized"`：使用色盲友好颜色的浅色主题
  * `"dark-ansi"`：仅使用终端 ANSI 调色板的深色主题
  * `"light-ansi"`：仅使用终端 ANSI 调色板的浅色主题
  * `"custom:<slug>"` 或 `"custom:<plugin-name>:<slug>"`：来自 `~/.claude/themes/` 或插件的自定义主题
* **默认值**：`"dark"`

```json settings.json theme={null}
{
  "theme": "light-daltonized"
}
```

请参阅[创建自定义主题](https://code.claude.com/docs/zh-CN/terminal-config.md#create-a-custom-theme)。

### `timeFormat`

选择 Claude Code 在界面中显示时间的格式，例如每条轮次耗时消息末尾的 `done 6:05 PM`，以及[会话记录查看器](https://code.claude.com/docs/zh-CN/interactive-mode.md#transcript-viewer)中的时间戳。要选择预设，请运行 `/config` 并设置 **Time format**。需要 Claude Code v2.1.257 或更高版本。

* **作用域**：[`Any file`](#scopes)
* **类型**：字符串，取值为以下之一：
  * `"auto"`：与未设置相同；每个时间保持其内置格式，轮次耗时消息中的时间遵循您的区域设置
  * `"12-hour"`：12 小时制
  * `"24-hour"`：24 小时制
  * `"24-hour-utc"`：UTC 24 小时制，分钟后带有 `Z`，例如 `18:05Z`；对于此预设，Claude Code 会忽略 [`timeZone`](#timezone)
  * strftime 模式，例如 `"%H:%M"`：Claude Code 使用该模式写入每个时间。任何包含 `%` 的值都是模式，预设以外的任何其他值都视为 `"auto"`
* **默认值**：`"auto"`

```json settings.json theme={null}
{
  "timeFormat": "24-hour"
}
```

`/config` 仅提供预设，因此如需使用 strftime 模式，请将此键添加到设置文件中。此示例将每个时间显示为两位数的 24 小时制：

```json settings.json theme={null}
{
  "timeFormat": "%H:%M"
}
```

之后，轮次耗时消息和会话记录查看器会显示 `18:05` 之类的时间。在会话记录查看器中，该模式就是完整的时间戳，因此如果您希望其中显示日期，请添加日期指令。此示例在时钟前加上日期：

```json settings.json theme={null}
{
  "timeFormat": "%Y-%m-%d %H:%M"
}
```

之后，这些位置会显示 `2026-09-01 18:05` 之类的时间。

### `timeZone`

以系统时区以外的时区显示界面中的时间。将其设置为 [IANA 时区名称](https://www.iana.org/time-zones)，例如 `"UTC"` 或 `"Europe/Dublin"`。由 [`timeFormat`](#timeformat) 控制的时间随后会以此时区显示。如果 `timeFormat` 为 `"24-hour-utc"`，时间保持为 UTC，Claude Code 会忽略此键。`/config` 中没有此键对应的行，因此请在设置文件中设置。需要 Claude Code v2.1.257 或更高版本。

* **作用域**：[`Any file`](#scopes)
* **类型**：字符串，IANA 时区名称。当 Claude Code 无法识别该名称时，会使用您的系统时区
* **默认值**：未设置，因此时间以您的系统时区显示

```json settings.json theme={null}
{
  "timeZone": "Europe/Dublin"
}
```

### `tui`

选择终端 UI 渲染器。使用 `"fullscreen"` 选择带有虚拟化回滚的无闪烁[备用屏幕渲染器](https://code.claude.com/docs/zh-CN/fullscreen.md)，或使用 `"default"` 选择经典的主屏幕渲染器。运行 `/tui fullscreen` 或 `/tui default` 会为您写入此键。

* **作用域**：[`Any file`](#scopes)
* **类型**：字符串，取值为以下之一：
  * `"default"`：经典的主屏幕渲染器
  * `"fullscreen"`：带有虚拟化回滚的无闪烁备用屏幕渲染器
* **默认值**：未设置，因此 Claude Code [为您选择渲染器](https://code.claude.com/docs/zh-CN/fullscreen.md#fullscreen-by-default)
* **单会话覆盖**：[`CLAUDE_CODE_NO_FLICKER`](https://code.claude.com/docs/zh-CN/env-vars) 和 [`CLAUDE_CODE_DISABLE_ALTERNATE_SCREEN`](https://code.claude.com/docs/zh-CN/env-vars) 在单个会话中优先于此键：`CLAUDE_CODE_NO_FLICKER=1` 开启全屏，`CLAUDE_CODE_NO_FLICKER=0` 或 `CLAUDE_CODE_DISABLE_ALTERNATE_SCREEN=1` 关闭全屏；两者同时设置时，Claude Code 关闭全屏

```json settings.json theme={null}
{
  "tui": "fullscreen"
}
```

在 tmux `-CC` 下或通过 SSH 连接到 Windows 时，除非您设置 `CLAUDE_CODE_NO_FLICKER=1`，否则 Claude Code 会保持使用经典渲染器。从 [Agent 视图](https://code.claude.com/docs/zh-CN/agent-view.md)打开的后台会话始终使用全屏渲染器，不受此设置影响。

### `verbose`

默认情况下，会话记录会将每个工具调用折叠为简短摘要，例如 Claude 运行的命令及其输出的行数；当您需要查看详细信息时，可按 `Ctrl+O` 将整个会话记录切换为展开视图。将此键设置为 `true` 可在工具调用发生时内联显示每个工具调用的完整输入和输出，这在调试 hook、MCP 服务器或长时间运行的 shell 命令时非常有用。在 `/config` 中显示为 **Verbose output**。

* **作用域**：[`Any file`](#scopes)。当没有任何设置文件设置此项时，旧版本写入 `~/.claude.json` 的值会生效。
* **类型**：布尔值
  * `true`：您会看到完整的工具输出
  * `false`：您会看到截断的工具输出摘要
* **默认值**：`false`
* **单会话覆盖**：[`--verbose`](https://code.claude.com/docs/zh-CN/cli-reference#cli-flags) 在单个会话中优先于此键

```json settings.json theme={null}
{
  "verbose": true
}
```

[`viewMode`](#viewmode) 值或持久的 `/focus` 选择在每个会话中都会覆盖此键。

### `viewMode`

设置 Claude Code 启动时的会话记录视图：`"default"`、`"verbose"` 或 `"focus"`。设置后，它会同时覆盖持久的 `/focus` 选择和 [`verbose`](#verbose) 设置。

* **作用域**：[`Any file`](#scopes)
* **类型**：字符串，取值为以下之一：
  * `"default"`：带有截断工具输出的普通会话记录
  * `"verbose"`：带有完整工具输出的会话记录
  * `"focus"`：仅显示您的最后一个提示词、带有编辑 diffstat 的工具调用单行摘要以及最终回复。Focus 视图需要[全屏渲染器](#tui)
* **默认值**：未设置，因此 `verbose` 设置和您最后一次的 `/focus` 选择生效
* **单会话覆盖**：[`--verbose`](https://code.claude.com/docs/zh-CN/cli-reference#cli-flags) 在单个会话中优先于此键

```json settings.json theme={null}
{
  "viewMode": "focus"
}
```

### `vimInsertModeRemaps`

在 [vim 编辑器模式](https://code.claude.com/docs/zh-CN/interactive-mode.md#vim-editor-mode)中，将两键 INSERT 模式序列映射为 Escape。每个键必须恰好是按顺序键入的两个可打印字符，且 `"<Esc>"` 是唯一支持的目标；Claude Code 会忽略其他条目。

* **作用域**：[`User or managed`](#scopes)。仓库无法重新映射您的按键。
* **类型**：将两个字符的序列映射到 `"<Esc>"` 的对象
* **默认值**：未设置

```json settings.json theme={null}
{
  "vimInsertModeRemaps": {
    "jj": "<Esc>"
  }
}
```

除非 `editorMode` 为 `"vim"`，否则此项无效。请参阅[重新映射 INSERT 模式按键序列](https://code.claude.com/docs/zh-CN/interactive-mode.md#remap-insert-mode-key-sequences)。

### `voice`

开启[语音听写](https://code.claude.com/docs/zh-CN/voice-dictation.md)并选择听写键的行为方式。当您运行 `/voice` 时，Claude Code 会为您写入此对象。

* **作用域**：[`Any file`](#scopes)
* **类型**：对象，包含布尔值 `enabled`、仅在按住模式下生效的布尔值 `autoSubmit`，以及 `mode`，`mode` 取值为以下之一：
  * `"hold"`：说话时按住听写键，松开即停止
  * `"tap"`：点按一次开始录音，再点按一次发送
* **默认值**：未设置，因此听写关闭；当 `enabled` 为 `true` 且未设置 `mode` 时，Claude Code 使用 `"hold"`

此示例开启听写，并使按键点按一次开始录音、再点按一次发送：

```json settings.json theme={null}
{
  "voice": {
    "enabled": true,
    "mode": "tap"
  }
}
```

`autoSubmit` 会在按住模式下松开按键时发送提示词。语音听写需要 claude.ai 账户。

### `voiceEnabled`

> [!WARNING]
>
> 自 v2.1.92 起已弃用，当时 [`voice`](#voice) 对象取代了它。Claude Code 仍会读取它，以便旧的设置文件继续有效，但新的配置应设置 `voice.enabled`。

使用早于 `voice` 对象的单一布尔值形式开启语音听写。两者同时设置时，`voice.enabled` 生效。

* **作用域**：[`Any file`](#scopes)
* **类型**：布尔值
  * `true`：当您使用 claude.ai 账户登录且您组织的策略允许语音时，语音听写开启，除非设置了 `voice.enabled`
  * `false`：语音听写关闭，除非设置了 `voice.enabled`
* **默认值**：未设置

```json settings.json theme={null}
{
  "voiceEnabled": true
}
```

### `wheelScrollAccelerationEnabled`

在[全屏渲染](https://code.claude.com/docs/zh-CN/fullscreen.md#mouse-wheel-scrolling)中快速滚动时加快鼠标滚轮的滚动速度。设置为 `false` 可使每个滚轮刻度的滚动速率保持恒定。

* **作用域**：[`Any file`](#scopes)
* **类型**：布尔值
  * `true`：Claude Code 在快速滚动时加快鼠标滚轮的滚动速度
  * `false`：Claude Code 以每个滚轮刻度恒定的速率滚动
* **默认值**：`true`

```json settings.json theme={null}
{
  "wheelScrollAccelerationEnabled": false
}
```

## Git 与署名

控制 Claude Code 添加到提交和 Pull Request 中的署名，以及它与 git 的协作方式。

### `attribution`

自定义 Claude Code 添加到 git 提交和 Pull Request 中的署名。默认情况下，提交会带有 [git trailer](https://git-scm.com/docs/git-interpret-trailers)，例如 `Co-Authored-By`；Pull Request 描述则带有纯文本。可通过下方的子键分别设置各部分。

* **作用域**：[`Any file`](#scopes)
* **类型**：包含 `commit` 和 `pr` 字符串以及 `sessionUrl` 布尔值的对象，或设为 `false` 以隐藏所有署名。`false` 值需要 Claude Code v2.1.281 或更高版本；更早的版本会拒绝该值，并[跳过包含它的整个用户、项目或本地设置文件](https://code.claude.com/docs/zh-CN/settings.md#fix-a-broken-settings-file)
* **默认值**：未设置，此时 Claude Code 使用各子键下所示的标准署名

要隐藏所有署名，请将 `attribution` 设为 `false`。如果设置文件也会被更早的版本读取，请改为将 [`commit`](#attribution-commit) 和 [`pr`](#attribution-pr) 设为空字符串，并将 [`sessionUrl`](#attribution-sessionurl) 设为 `false`。

以下示例替换了提交署名，移除了 Pull Request 署名，并去掉了会话链接：

```json settings.json theme={null}
{
  "attribution": {
    "commit": "Generated with AI\n\nCo-Authored-By: AI <ai@example.com>",
    "pr": "",
    "sessionUrl": false
  }
}
```

一旦设置了 `commit` 或 `pr`，Claude Code 将忽略已弃用的 `includeCoAuthoredBy` 设置，并对两者中未设置的那一项使用其默认文本。

Claude Code 会告知 Claude，您自己关于署名的指令（例如 CLAUDE.md 或[记忆](https://code.claude.com/docs/zh-CN/memory.md)中的规则）优先于这些提交和 PR 署名行，除非该署名行是在[托管设置](https://code.claude.com/docs/zh-CN/managed-settings.md)中设置的。

### `includeCoAuthoredBy`

> [!WARNING]
>
> 自 v2.0.62 起已弃用，由 [`attribution`](#attribution) 取代。Claude Code 仍会读取该设置，但新的配置应使用 `attribution`。

请改用 [`attribution`](#attribution)，它取代了此键，并允许您分别更改或隐藏提交 trailer、Pull Request 文本和会话链接。对于早于 `attribution` 的设置文件，Claude Code 仍会遵循其中的 `includeCoAuthoredBy: false`，但一旦设置了 `attribution.commit` 或 `attribution.pr`，就会忽略它。

* **作用域**：[`Any file`](#scopes)
* **类型**：布尔值
  * `true`：与未设置相同；Claude Code 会添加提交 trailer 和 Pull Request 署名文本
  * `false`：Claude Code 会同时省略提交 trailer 和 Pull Request 署名文本，除非 `attribution` 设置了 `commit` 或 `pr`，此时适用 [`attribution`](#attribution) 的规则
* **默认值**：`true`

```json settings.json theme={null}
{
  "includeCoAuthoredBy": false
}
```

要隐藏所有署名，请参阅 [`attribution`](#attribution)。

### `includeGitInstructions`

Claude Code 会向 Claude 提供两项与 git 相关的上下文：一是 Bash 工具描述中关于如何编写提交和 Pull Request 的内置指令，二是仓库的 git 状态快照。该快照包含当前分支、主分支、`git status` 输出以及最近的提交。Claude Code 会在对话开始时读取它。

将此键设为 `false` 可同时省略这两项，例如在您使用自己的 git 工作流 skill 时。

* **作用域**：[`Any file`](#scopes)
* **类型**：布尔值
  * `true`：Claude Code 包含其内置的提交和 Pull Request 工作流指令以及 git 状态快照。云端会话从不包含该快照
  * `false`：Claude Code 省略这两项
* **默认值**：`true`
* **按会话覆盖**：[`CLAUDE_CODE_DISABLE_GIT_INSTRUCTIONS`](https://code.claude.com/docs/zh-CN/env-vars) 在单个会话中优先于此键

```json settings.json theme={null}
{
  "includeGitInstructions": false
}
```

如果您在[非交互模式](https://code.claude.com/docs/zh-CN/headless.md)或 Agent SDK 中替换了系统提示词，即使此键为 `true`，会话启动时也不会包含该快照：

* **`-p` 运行**：当您传入 [`--system-prompt`](https://code.claude.com/docs/zh-CN/cli-reference#system-prompt-flags) 或 `--system-prompt-file` 时，即替换了提示词。当您运行带有自身提示词的自定义 Agent（通过 [`--agent`](https://code.claude.com/docs/zh-CN/sub-agents#invoke-subagents-explicitly) 或 [`agent`](#agent) 设置选择）时，也会替换提示词。在 [bare 模式](https://code.claude.com/docs/zh-CN/headless.md#start-faster-with-bare-mode)之外，若要保留快照，请不使用自定义 Agent 运行，并使用 [`--append-system-prompt`](https://code.claude.com/docs/zh-CN/cli-reference#system-prompt-flags) 添加您的指令，而不是替换提示词。
* **[Agent SDK](https://code.claude.com/docs/zh-CN/agent-sdk/modifying-system-prompts.md)**：当您使用自定义系统提示词或自定义主线程 Agent 时，即替换了提示词。如果您未设置系统提示词，SDK 会使用其最简默认提示词，会话启动时同样不包含快照。在 [bare 模式](https://code.claude.com/docs/zh-CN/headless.md#start-faster-with-bare-mode)之外，若要保留快照，请使用 `claude_code` 预设，且不使用自定义 `agent`。

### `prUrlTemplate`

将 Claude Code 在页脚徽章和工具结果摘要中呈现的 PR 链接指向内部代码审查工具，而不是 `github.com`。Claude Code 会根据 PR URL 替换 `{host}`、`{owner}`、`{repo}`、`{number}` 和 `{url}`。在这两处，[GitLab 合并请求](https://code.claude.com/docs/zh-CN/interactive-mode.md#gitlab-merge-requests)链接都会保留其 GitLab URL。

* **作用域**：[`Any file`](#scopes)
* **类型**：字符串，可使用上述五个占位符中任意一个的 URL 模板
* **默认值**：未设置

```json settings.json theme={null}
{
  "prUrlTemplate": "https://reviews.example.com/{owner}/{repo}/pull/{number}"
}
```

Claude Code 仅将模板应用于其自身呈现的链接；Claude 在消息中写出的 PR 编号（例如 `#123`）会保持 Claude 所写的原样。不符合 `/pull/<number>` 形式的 URL 保持不变。

### `attribution.commit`

设置 Claude Code 添加到 git 提交中的署名文本，包括任何 trailer。将其设为空字符串可隐藏提交署名。

* **作用域**：[`Any file`](#scopes)
* **类型**：字符串
* **默认值**：未设置，此时 Claude Code 会添加 `Co-Authored-By: <name> <noreply@anthropic.com>`。其中的名称是进行提交时所使用的模型，例如 `Claude Sonnet 5`。当由[子代理](https://code.claude.com/docs/zh-CN/sub-agents.md)进行提交时，trailer 中的名称为该子代理的模型。
  * 当 Claude Code 能识别出该模型是 Claude 模型但无法确认其确切版本时，只写入 `Claude`。
  * 当无法将模型 ID 与任何 Claude 模型匹配时（例如通过自定义 [`ANTHROPIC_BASE_URL`](https://code.claude.com/docs/zh-CN/env-vars) 提供的第三方模型），写入 `Claude Code`。

以下示例将默认 trailer 替换为一行自定义文本和一个自定义的 `Co-Authored-By` trailer：

```json settings.json theme={null}
{
  "attribution": {
    "commit": "Generated with AI\n\nCo-Authored-By: AI <ai@example.com>"
  }
}
```

### `attribution.pr`

设置 Claude Code 添加到 Pull Request 描述中的署名文本。将其设为空字符串可隐藏 Pull Request 署名。

* **作用域**：[`Any file`](#scopes)
* **类型**：字符串
* **默认值**：未设置，此时 Claude Code 会添加 `🤖 Generated with [Claude Code](https://claude.com/claude-code)`

```json settings.json theme={null}
{
  "attribution": {
    "pr": ""
  }
}
```

### `attribution.sessionUrl`

选择 Claude Code 在从[云端](https://code.claude.com/docs/zh-CN/claude-code-on-the-web.md)或 [Remote Control](https://code.claude.com/docs/zh-CN/remote-control.md) 会话中进行提交或创建 Pull Request 时，是否附加 claude.ai 会话链接。Claude Code 会在提交中以 `Claude-Session` trailer 的形式添加该链接，在 Pull Request 描述中以链接形式添加。将其设为 `false` 可省略该链接。

* **作用域**：[`Any file`](#scopes)
* **类型**：布尔值
  * `true`：Claude Code 在从云端或 Remote Control 会话中进行提交或创建 Pull Request 时附加 claude.ai 会话链接
  * `false`：Claude Code 省略该链接
* **默认值**：`true`

```json settings.json theme={null}
{
  "attribution": {
    "sessionUrl": false
  }
}
```

## Hook 与自动化

注册 hook、限制可运行的 hook，并控制工作流。有关 hook 事件及其负载，请参阅 [hook 参考](https://code.claude.com/docs/zh-CN/hooks.md)。

### `allowedHttpHookUrls`

限制 [HTTP hook](https://code.claude.com/docs/zh-CN/hooks.md#http-hook-fields) 可以访问的 URL。定义此键后，Claude Code 仅在 HTTP hook 的 URL 与其中某个模式匹配时才运行该 hook，其余 hook 会被阻止且不会运行；空数组会阻止所有 HTTP hook。

* **作用域**：[`Any file`](#scopes)。数组会在各设置文件之间合并。
* **类型**：URL 模式数组，使用 `*` 作为通配符
* **默认值**：未设置，因此允许任何 URL

此示例允许 `https://hooks.example.com/` 下的任何 URL 以及任何 `http://localhost` URL：

```json settings.json theme={null}
{
  "allowedHttpHookUrls": ["https://hooks.example.com/*", "http://localhost:*"]
}
```

主机名匹配不区分大小写，并且会将 `hooks.example.com.`（末尾带有表示完全限定域名的点）视为与 `hooks.example.com` 相同，这与 DNS 的处理方式一致。此允许列表适用于所有来源的 hook，包括托管设置。

在 Agent SDK 的 `managedSettings` 选项中，如果此键的列表包含条目，Claude Code 会忽略此键，因此请改为在 SDK 的 [`settings` 选项](https://code.claude.com/docs/zh-CN/agent-sdk/typescript#options)中传入该列表。请参阅[空列表在父级设置中的作用](https://code.claude.com/docs/zh-CN/claude-apps-gateway#how-hook-and-skill-keys-in-parent-settings-take-effect)。

### `allowManagedHooksOnly`

将 hook 的执行限制为您的组织所部署的 hook。

* **作用域**：[`Managed`](#scopes)
* **类型**：Boolean
  * `true`：仅运行托管 hook，以及 Agent SDK hook 和托管设置强制启用的插件中的 hook。请参阅 [`allowManagedHooksOnly` 下会运行哪些内容](#what-runs-under-allowmanagedhooksonly)
  * `false`：运行来自所有设置作用域和插件的 hook
* **默认值**：未设置，因此运行来自所有设置作用域和插件的 hook

```json managed-settings.json theme={null}
{
  "allowManagedHooksOnly": true
}
```

#### `allowManagedHooksOnly` 下会运行哪些内容

将其设置为 `true` 后，Claude Code 会改变加载哪些 hook 以及类似 hook 的命令：

* **托管 hook 和 SDK hook 会运行**：来自托管设置的 hook，以及 [Agent SDK](https://code.claude.com/docs/zh-CN/agent-sdk/overview.md) 在进程内注册的 hook
* **强制启用的插件 hook 会运行**：来自托管设置通过 [`enabledPlugins`](#enabledplugins) 强制启用的插件中的 hook。Claude Code 会按完整的 `plugin@marketplace` ID 进行匹配，因此来自其他市场的同名插件仍会被阻止。这样，您可以通过组织市场分发经过审核的 hook，同时阻止其他所有内容。此类插件中的 [mod](https://code.claude.com/docs/zh-CN/plugins/mods/overview.md) 仅在[被视为您组织的 mod](https://code.claude.com/docs/zh-CN/plugins/mods/admin.md#install-your-organizations-mods) 时才会加载
* **其他插件中的监视器会被阻止**：除非插件被强制启用，否则插件的[监视器](https://code.claude.com/docs/zh-CN/plugins/components.md#monitors)不会启动。需要 Claude Code v2.1.310 或更高版本
* **其他所有内容都会被阻止**：用户、项目和本地 hook，其他已安装插件中的 hook 和 mod，以及在 Agent frontmatter 中声明的 hook。[Claude Code 内置的 mod](https://code.claude.com/docs/zh-CN/plugins/mods/overview.md#mods-built-into-claude-code) 会继续运行。如果只想阻止用户的 mod，请改为设置 [`allowManagedModsOnly`](https://code.claude.com/docs/zh-CN/plugins/mods/admin#set-options-on-the-built-in-guard)。
* **命令来源的插件会被禁用**：Claude Code 还会禁用具有 [`command` 来源](https://code.claude.com/docs/zh-CN/plugins/marketplace-reference#command-plugin-source)的插件，包括在托管 `enabledPlugins` 中强制启用的插件，除非您显式将 [`disableCommandPluginSources`](#disablecommandpluginsources) 设置为 `false`
* **市场的 `headersHelper` 命令会被阻止**：Claude Code 还会阻止市场的 [`headersHelper` 命令](https://code.claude.com/docs/zh-CN/plugins/host-marketplace#authenticate-archive-downloads)，除非显式将 [`disableCommandPluginSources`](#disablecommandpluginsources) 设置为 `false`，但由托管设置本身声明的市场除外。需要 Claude Code v2.1.238 或更高版本
* **状态栏和文件建议仅限于托管设置**：Claude Code 仅从托管设置中读取 [`statusLine`](https://code.claude.com/docs/zh-CN/statusline)、[`fileSuggestion`](#filesuggestion) 和 [`subagentStatusLine`](https://code.claude.com/docs/zh-CN/statusline#subagent-status-lines)，遵循[状态栏和文件建议的限制规则](#status-line-and-file-suggestion-gates)

设置此键后，[`/goal`](https://code.claude.com/docs/zh-CN/goal) 命令无法运行，因为它依赖于 hook。

### `disableAllHooks`

关闭 [hook](https://code.claude.com/docs/zh-CN/hooks.md#disable-or-remove-hooks)、任何自定义[状态栏](https://code.claude.com/docs/zh-CN/statusline.md)以及任何自定义[文件建议](#filesuggestion)命令。可以使用它临时关闭所有这些功能，而无需将它们从设置中删除。

* **作用域**：[`Any file`](#scopes)。只有托管设置才能禁用托管 hook。
* **类型**：Boolean
  * `true`：Claude Code 关闭 hook、任何自定义状态栏以及任何自定义文件建议命令
  * `false`：hook、状态栏和文件建议命令会运行
* **默认值**：未设置，因此 hook 会运行

```json settings.json theme={null}
{
  "disableAllHooks": true
}
```

其影响范围取决于哪个文件包含此键：

* **在托管设置中**：Claude Code 会禁用所有已配置的 hook（包括托管 hook），但会继续运行 [Agent SDK](https://code.claude.com/docs/zh-CN/agent-sdk/overview.md) 在进程内注册的 hook
* **在 Agent SDK 的 `managedSettings` 选项中**：在管理员部署了托管设置的机器上，除非管理员[允许嵌入宿主添加策略](https://code.claude.com/docs/zh-CN/managed-settings.md#let-an-embedding-host-add-policy)，否则 Claude Code 会忽略此键。在此键生效的情况下，用户、项目、本地和插件 hook 会停止，SDK 的 `settings` 选项中的 hook 也会停止。管理员的托管 hook、[强制启用](#enabledplugins)的插件中的 hook 以及您的 [SDK hook 回调](https://code.claude.com/docs/zh-CN/agent-sdk/hooks.md)会继续运行。需要 Claude Code v2.1.309 或更高版本。
* **在任何其他设置文件中**：Claude Code 会禁用用户、项目、本地和插件 hook；托管 hook、Agent SDK hook 以及在托管 [`enabledPlugins`](#enabledplugins) 中强制启用的插件中的 hook 会继续运行

此键还会停止 [mod](https://code.claude.com/docs/zh-CN/plugins/mods/overview.md)，即其代码会注册 hook 的插件：

* **在托管设置中**：所有已安装插件中的 mod 都会停止，包括您组织的 mod
* **在任何其他设置文件中**：您安装的 mod 会停止，而[您组织的 mod](https://code.claude.com/docs/zh-CN/plugins/mods/admin.md#install-your-organizations-mods) 会继续运行

在这两种情况下，Claude Code 内置的 mod 都会继续运行。每个内置 mod 都有[各自的开关](https://code.claude.com/docs/zh-CN/plugins/mods/overview.md#mods-built-into-claude-code)。

在托管设置设置此键时保持 Agent SDK hook 继续运行，需要 Claude Code v2.1.242 或更高版本。

禁用 hook 时，[`/goal`](https://code.claude.com/docs/zh-CN/goal) 命令无法运行，并且 `/hooks` 菜单会显示一条通知，而不是您的 hook。

#### 状态栏和文件建议的限制规则

对于 `statusLine`、`fileSuggestion` 和 `subagentStatusLine`，Claude Code 会按以下顺序做出两项判断：

* **完全关闭**：当托管设置设置了 `disableAllHooks` 时，或者当文件夹在与[设置文件中的 hook 相同的工作区信任规则](https://code.claude.com/docs/zh-CN/permissions.md#what-runs-before-you-trust-a-folder)下不受信任时
* **仅限于托管设置**：当设置了 [`allowManagedHooksOnly`](#allowmanagedhooksonly) 时，当应用[设置优先级](https://code.claude.com/docs/zh-CN/hooks.md#disable-or-remove-hooks)后 `disableAllHooks` 在托管设置之外为 `true` 时，或者当您使用 `--safe-mode` 启动 Claude Code 时

在仅限于托管设置的情况下，如果部署了托管值，Claude Code 会运行该托管值。否则，它会在不发出警告的情况下跳过您的值：状态栏被禁用，`@` 自动补全会回退到内置的文件建议。

### `disableWorkflows`

为您的设置所覆盖的所有人（例如通过托管设置覆盖整个组织）关闭[动态工作流](https://code.claude.com/docs/zh-CN/workflows.md#turn-workflows-off)和内置的工作流命令。如果只想为自己打开或关闭工作流，请改用 [`enableWorkflows`](#enableworkflows)，`/config` 中的 **Dynamic workflows** 开关会将其写入您的用户设置。

* **作用域**：[`Any file`](#scopes)
* **类型**：Boolean
  * `true`：Claude Code 为您的设置所覆盖的所有人关闭动态工作流和内置的工作流命令
  * `false`：与未设置相同；此时工作流是否开启取决于 [`enableWorkflows`](#enableworkflows) 和您套餐的默认值
* **默认值**：`false`
* **按会话覆盖**：[`CLAUDE_CODE_DISABLE_WORKFLOWS`](https://code.claude.com/docs/zh-CN/env-vars) 可在单个会话中关闭工作流；无论二者中哪一个关闭了工作流，另一个都无法将其重新开启

```json settings.json theme={null}
{
  "disableWorkflows": true
}
```

### `enableWorkflows`

当您套餐的默认值不符合需求时，为自己打开或关闭[动态工作流](https://code.claude.com/docs/zh-CN/workflows.md)。在 `/config` 中显示为 **Dynamic workflows**，该选项会将此键写入您的用户设置，并在您切换回套餐默认值时再次将其移除。如果要通过托管设置为所有人关闭工作流，请改用 [`disableWorkflows`](#disableworkflows)。

* **作用域**：[`Any file`](#scopes)
* **类型**：Boolean
  * `true`：Claude Code 为您开启动态工作流
  * `false`：Claude Code 为您关闭动态工作流
* **默认值**：未设置，因此工作流处于开启状态，除非您使用的是 Pro 套餐，此时工作流处于关闭状态
* **按会话覆盖**：[`CLAUDE_CODE_DISABLE_WORKFLOWS`](https://code.claude.com/docs/zh-CN/env-vars) 可在单个会话中关闭工作流，设置该变量期间，此处的 `true` 无法将其重新开启

```json settings.json theme={null}
{
  "enableWorkflows": true
}
```

[`disableWorkflows`](#disableworkflows) 和您组织的工作流策略同样优先于此设置：只要有任何来源关闭了工作流，`enableWorkflows: true` 就无法将其重新开启。当用户设置以外的来源设置了 `enableWorkflows`，或将 `disableWorkflows` 设置为 `true` 时，Claude Code 会隐藏 `/config` 中的该行。

### `hooks`

在 Claude Code 生命周期的各个节点（例如工具调用之前或会话开始时）以 [hook](https://code.claude.com/docs/zh-CN/hooks.md) 的形式运行您自己的命令、提示词、Agent、HTTP 请求或 MCP 工具；[hook 参考](https://code.claude.com/docs/zh-CN/hooks.md#hook-events)列出了每个事件及其负载和退出码。每个事件对应一个匹配器组列表，每个组列出了匹配器适用时要运行的处理程序。

* **作用域**：[`Any file`](#scopes)。hook 会在各文件之间合并，而不是相互替换，并且来自托管设置的 hook 无法从其他文件中移除。
* **类型**：以 [hook 事件](https://code.claude.com/docs/zh-CN/hooks.md#hook-events)为键的对象；每个值是一个 `{ "matcher", "hooks" }` 组数组，其中 `hooks` 条目的 `type` 为 `"command"`、`"prompt"`、`"agent"`、`"http"` 或 `"mcp_tool"`
* **默认值**：未设置，因此不运行任何 hook

此示例在每次 Bash 工具调用之前运行一个脚本：

```json settings.json theme={null}
{
  "hooks": {
    "PreToolUse": [
      {
        "matcher": "Bash",
        "hooks": [
          { "type": "command", "command": "~/.claude/hooks/check-bash.sh" }
        ]
      }
    ]
  }
}
```

有关每个事件、匹配器模式和处理程序字段，请参阅 [hook 参考](https://code.claude.com/docs/zh-CN/hooks.md#configuration)。要关闭 hook，请参阅 [`disableAllHooks`](#disableallhooks)；要将 hook 限制为您组织部署的 hook，请参阅 [`allowManagedHooksOnly`](#allowmanagedhooksonly)。

### `httpHookAllowedEnvVars`

[HTTP hook](https://code.claude.com/docs/zh-CN/hooks.md#http-hook-fields) 可以将环境变量的值放入请求头中，例如 `Authorization: Bearer $HOOK_TOKEN` 请求头，但仅限于该 hook 在其自身 `allowedEnvVars` 中列出的变量。此键为所有 HTTP hook 的该列表设置了一个外层限制：只有当 hook 自身的 `allowedEnvVars` 和此键都列出某个变量时，hook 才能使用该变量。可以使用它来阻止 hook 读取不应读取的机密，即使 hook 的定义请求了该机密。

* **作用域**：[`Any file`](#scopes)。数组会在各设置文件之间合并。
* **类型**：环境变量名称数组
* **默认值**：未设置，因此适用每个 hook 自身的 `allowedEnvVars` 列表

此示例将请求头插值限制为 `MY_TOKEN` 和 `HOOK_SECRET`：

```json settings.json theme={null}
{
  "httpHookAllowedEnvVars": ["MY_TOKEN", "HOOK_SECRET"]
}
```

此允许列表适用于所有来源的 hook，包括托管设置。

在 Agent SDK 的 `managedSettings` 选项中，如果此键的列表包含条目，Claude Code 会忽略此键，因此请改为在 SDK 的 [`settings` 选项](https://code.claude.com/docs/zh-CN/agent-sdk/typescript#options)中传入该列表。请参阅[空列表在父级设置中的作用](https://code.claude.com/docs/zh-CN/claude-apps-gateway#how-hook-and-skill-keys-in-parent-settings-take-effect)。

### `workflowKeywordTriggerEnabled`

选择在提示词中输入关键字 `ultracode` 时是否触发[动态工作流](https://code.claude.com/docs/zh-CN/workflows.md#ask-for-a-workflow-in-your-prompt)。将其设置为 `false` 后，输入该词不会触发工作流。

* **作用域**：[`Any file`](#scopes)。在 `/config` 中显示为 **Ultracode keyword trigger**。
* **类型**：Boolean
  * `true`：在提示词中输入 `ultracode` 会触发动态工作流
  * `false`：您可以输入该词而不触发工作流
* **默认值**：`true`

```json settings.json theme={null}
{
  "workflowKeywordTriggerEnabled": false
}
```

`ultracode` 投入程度设置、`/workflows` 以及已保存的工作流命令不受影响。

### `workflowSizeGuideline`

设置 Claude 在其编写的动态工作流中[力求达到的 Agent 数量](https://code.claude.com/docs/zh-CN/workflows.md#set-a-size-guideline)。Claude Code 会将该值作为建议而非强制上限发送给 Claude：`"small"` 要求少于 5 个 Agent，`"medium"` 少于 10 个，`"large"` 少于 50 个。如果您希望限制工作流的开销，请选择 `"small"`。

* **作用域**：[`Any file`](#scopes)。在设置文件中设置的值优先于 `/config` 中的 **Dynamic workflow size** 选项（Claude Code 将其存储在 `~/.claude.json` 中），并且当设置文件设置了此键时，Claude Code 会隐藏该行。
* **类型**：字符串，取值为以下之一：
  * `"unrestricted"`：无指导值，因此 Claude 会根据任务确定工作流规模
  * `"small"`：Claude 力求使用少于 5 个 Agent
  * `"medium"`：Claude 力求使用少于 10 个 Agent
  * `"large"`：Claude 力求使用少于 50 个 Agent
* **默认值**：`"medium"`；如果您使用 Pro 套餐登录且 Claude Code 为 v2.1.271 或更高版本，则为 `"small"`

```json settings.json theme={null}
{
  "workflowSizeGuideline": "small"
}
```

## 插件与 skill

启用插件、注册市场、限制组织允许的插件来源，以及控制加载哪些 skill。有关安装和构建插件的信息，请参阅[插件](https://code.claude.com/docs/zh-CN/plugins/overview.md)。

### `disableBundledSkills`

关闭 Claude Code 随附的 [skill](https://code.claude.com/docs/zh-CN/skills.md) 和工作流。Claude Code 会完全移除随附 skill 和工作流，而 `/init` 等内置命令仍可输入，但对模型隐藏。

* **作用域**：[`Any file`](#scopes)
* **类型**：Boolean
  * `true`：Claude Code 移除随附 skill 和工作流，并对模型隐藏 `/init` 等内置命令
  * `false`：加载随附 skill
* **默认值**：未设置，因此加载随附 skill
* **按会话覆盖**：将 [`CLAUDE_CODE_DISABLE_BUNDLED_SKILLS`](https://code.claude.com/docs/zh-CN/env-vars) 设置为 `1` 可在单个会话中关闭随附 skill；两者中任何一个将其关闭后，另一个都无法重新开启

```json settings.json theme={null}
{
  "disableBundledSkills": true
}
```

来自插件、`.claude/skills/` 和 `.claude/commands/` 的 skill 不受影响。`/doctor` 与内置命令一样仍可输入；要隐藏它，请改为设置 [`DISABLE_DOCTOR_COMMAND`](https://code.claude.com/docs/zh-CN/env-vars)。

### `disableSkillShellExecution`

关闭来自用户、项目、插件或附加目录来源的 [skill](https://code.claude.com/docs/zh-CN/skills.md) 和自定义命令中 `` !`...` `` 和 ` ```! ` 块的内联 shell 执行。Claude Code 会将每条命令替换为 `[shell command execution disabled by policy]`，而不是运行它。

* **作用域**：[`Any file`](#scopes)。托管设置中的 `true` 无法被其他位置的 `false` 覆盖。
* **类型**：Boolean
  * `true`：Claude Code 将每条内联 shell 命令替换为 `[shell command execution disabled by policy]`，而不是运行它
  * `false`：运行内联 shell
* **默认值**：未设置，因此运行内联 shell

```json settings.json theme={null}
{
  "disableSkillShellExecution": true
}
```

随附 skill 以及通过托管设置部署的 skill 不受影响。

### `skillOverrides`

无需编辑 `SKILL.md` 即可隐藏或折叠某个 [skill](https://code.claude.com/docs/zh-CN/skills.md#override-skill-visibility-from-settings)。Claude Code 会将每个 skill 名称下的值应用于 Claude 看到的 skill 列表以及您的 `/` 自动补全。

* **作用域**：[`Any file`](#scopes)。`/skills` 菜单会写入 `.claude/settings.local.json`。
* **类型**：将 skill 名称映射到以下值之一的对象：
  * `"on"`：Claude 可以看到该 skill，您可以输入 `/name`
  * `"name-only"`：Claude 只能看到该 skill 的名称，看不到其描述
  * `"user-invocable-only"`：Claude 看不到该 skill，但您仍可输入 `/name`
  * `"off"`：Claude 看不到该 skill，且 `/name` 在自动补全中隐藏
* **默认值**：未设置，因此每个 skill 均为 `"on"`

此示例仅以名称向 Claude 列出 `legacy-context`，并对 Claude 和 `/` 自动补全隐藏 `deploy`：

```json settings.json theme={null}
{
  "skillOverrides": {
    "legacy-context": "name-only",
    "deploy": "off"
  }
}
```

覆盖不适用于插件 skill，插件 skill 通过 `/plugin` 进行管理。

在托管设置以及通过 `--settings` 传入的文件中，针对随附 skill 别名的键（例如 `/doctor` 的别名 `checkup`）也会应用于该 skill；请参阅[别名键如何与 skill 自身名称的键组合](https://code.claude.com/docs/zh-CN/skills.md#override-skill-visibility-from-settings)。

### `syncClaudeAiSkills`

关闭[为您的 claude.ai 账户启用的 skill](https://code.claude.com/docs/zh-CN/skills.md#how-synced-skills-behave) 的下载。在[您使用 claude.ai 账户登录的终端会话](https://code.claude.com/docs/zh-CN/skills.md#where-synced-skills-load)中（无论交互式还是非交互式），以及在 Cowork 和云端会话中，Claude Code 会将这些 skill 下载到 `~/.claude/skills/synced/`。设置为 `false` 可停止该下载，并停止加载已同步的 skill。Claude Code 仅识别 `false`：`true` 与未设置相同，不会在原本关闭同步的地方开启同步。

* **作用域**：[`User, local, or managed`](#scopes)，以及通过 `--settings` 传入的文件。仓库无法替您关闭它。
* **类型**：Boolean
  * `false`：Claude Code 停止下载同步的 skill，并停止加载 `~/.claude/skills/synced/` 中已有的 skill。在用户设置或托管设置中，还会将它们移动到 `~/.claude/skills/.trash/`
  * `true`：与未设置相同
* **默认值**：未设置，因此使用 claude.ai 账户登录的会话会同步您的 skill

此示例阻止某台机器在任何会话中下载该账户的 skill：

```json settings.json theme={null}
{
  "syncClaudeAiSkills": false
}
```

### `syncClaudeAiPlugins`

关闭[为您的 claude.ai 账户启用的插件](https://code.claude.com/docs/zh-CN/plugins/loading.md#synced-plugins)的下载。在您使用 claude.ai 账户登录的终端会话以及 Cowork 会话开始时，Claude Code 会将这些插件下载到 `~/.claude/plugins/synced/`，并将每个插件加载为 `<name>@synced`。设置为 `false` 可停止该下载，并停止加载已同步的插件。Claude Code 仅识别 `false`：`true` 与未设置相同，不会在原本关闭同步的地方开启同步。需要 Claude Code v2.1.273 或更高版本。

* **作用域**：[`User, local, or managed`](#scopes)，以及通过 `--settings` 传入的文件。仓库无法替您关闭它。
* **类型**：Boolean
  * `false`：Claude Code 停止下载同步的插件，并停止加载 `~/.claude/plugins/synced/` 中已有的插件。在用户设置或托管设置中，还会将它们移动到 `~/.claude/plugins/.trash/`
  * `true`：与未设置相同
* **默认值**：未设置，因此使用 claude.ai 账户登录的会话会同步您的插件

要关闭单个同步插件而不是全部插件，请在 [`enabledPlugins`](#enabledplugins) 中设置 `"<name>@synced": false`。

此示例阻止某台机器在任何会话中下载该账户的插件：

```json settings.json theme={null}
{
  "syncClaudeAiPlugins": false
}
```

### `allowedChannelPlugins`

选择哪些[频道](https://code.claude.com/docs/zh-CN/channels.md)插件可以向您组织中的会话推送消息。设置后，Claude Code 会使用您的列表代替默认的 Anthropic 允许列表；每个条目指定一个插件及其所属的市场。

* **作用域**：[`Managed`](#scopes)
* **类型**：对象数组，每个对象包含 `marketplace` 和 `plugin` 字符串。条目也可以是 `"plugin@marketplace"` 字符串，例如 `"telegram@claude-plugins-official"`，Claude Code 会将其视为等效的对象。字符串形式需要 Claude Code v2.1.267 或更高版本；较早版本在 `allowedChannelPlugins` 值包含字符串形式时会拒绝整个值
* **默认值**：未设置，因此 Claude Code 使用默认的 Anthropic 允许列表

此示例启用频道，并且仅允许来自官方 Anthropic 市场的 Telegram 插件：

```json managed-settings.json theme={null}
{
  "channelsEnabled": true,
  "allowedChannelPlugins": [
    { "marketplace": "claude-plugins-official", "plugin": "telegram" }
  ]
}
```

空数组会阻止所有频道插件。

此键在频道通过该账户的 [`channelsEnabled`](#channelsenabled) 门控后才生效：在 Team 和 Enterprise 计划上，以及在具有托管设置的 Console 账户上，这意味着需要 `channelsEnabled: true`。请参阅[限制可以运行哪些频道插件](https://code.claude.com/docs/zh-CN/channels.md#restrict-which-channel-plugins-can-run)。

### `blockedMarketplaces`

为您的组织阻止插件市场来源。Claude Code 在添加市场以及安装、更新、刷新和自动更新插件时检查拒绝列表，因此在您设置该策略之前已有人添加的市场也无法用于获取插件。被阻止的来源会在下载之前进行检查，因此它们永远不会接触文件系统。

如果您在 [claude.ai 管理控制台](https://code.claude.com/docs/zh-CN/server-managed-settings.md)中设置此键，那么当组织中任何人在 claude.ai 上从 git 仓库添加市场时，claude.ai 也会应用它，如[限制的工作方式](https://code.claude.com/docs/zh-CN/plugins/org.md#restrict-what-users-can-install)所述。

* **作用域**：[`Managed`](#scopes)
* **类型**：市场来源对象数组，形式与 [`strictKnownMarketplaces`](#allowed-source-types) 相同
* **默认值**：未设置，因此不阻止任何市场

此示例将一个 GitHub 仓库作为市场来源进行阻止：

```json managed-settings.json theme={null}
{
  "blockedMarketplaces": [
    { "source": "github", "repo": "untrusted/plugins" }
  ]
}
```

`github` 条目可以使用[所有者通配符形式](#owner-wildcards) `"owner/*"` 来阻止该 GitHub 所有者下的所有仓库，这需要 Claude Code v2.1.223 或更高版本。添加 `{ "source": "skills-dir" }` 可阻止 Claude Code 从 `~/.claude/skills/` 和项目的 `.claude/skills/` 加载 [`@skills-dir` 插件](https://code.claude.com/docs/zh-CN/plugins/loading#plugins-shared-through-a-repository)，而不限制任何市场。请参阅[托管市场限制](https://code.claude.com/docs/zh-CN/plugins/org.md#restrict-what-users-can-install)。

### `channelsEnabled`

为您的组织允许[频道](https://code.claude.com/docs/zh-CN/channels.md)。在 claude.ai Team 和 Enterprise 计划上，Claude Code 会阻止频道，直到您将其设置为 `true`。对于使用 API 密钥进行身份验证的 [Anthropic Console](https://code.claude.com/docs/zh-CN/authentication.md#claude-console-authentication) 账户，默认允许频道。如果您的组织部署了托管设置，Claude Code 也会在这些账户上阻止频道，直到您将此键设置为 `true`。

* **作用域**：[`Managed`](#scopes)
* **类型**：Boolean
  * `true`：Claude Code 为您的组织允许频道
  * `false`：与未设置相同；频道是否被阻止取决于您的计划，如默认值所述
* **默认值**：未设置；在 Team 和 Enterprise 计划以及具有托管设置的 Console 账户上阻止频道，在 Pro、Max 和 Ultra 计划以及没有托管设置的 Console 账户上允许频道

```json managed-settings.json theme={null}
{
  "channelsEnabled": true
}
```

要限制启用频道后哪些插件可以注册为频道，请设置 [`allowedChannelPlugins`](#allowedchannelplugins)。请参阅[企业控制](https://code.claude.com/docs/zh-CN/channels.md#enterprise-controls)。

### `disableCommandPluginSources`

阻止 [`command` 插件来源](https://code.claude.com/docs/zh-CN/plugins/marketplace-reference#command-plugin-source)，该来源通过在用户机器上运行市场声明的命令来安装插件。将其设置为 `true` 后，Claude Code 永远不会运行该命令，不会安装或更新来自命令来源的插件，并停止加载已安装的此类插件。将其设置为 `false` 可显式允许它们。每当它阻止命令来源时，无论是您将其设置为 `true`，还是在 [`allowManagedHooksOnly`](#allowmanagedhooksonly) 下保持未设置，它还会阻止市场的 [`headersHelper` 命令](https://code.claude.com/docs/zh-CN/plugins/host-marketplace#authenticate-archive-downloads)，但托管设置自身声明的市场除外。需要 Claude Code v2.1.229 或更高版本，`headersHelper` 阻止需要 v2.1.238 或更高版本。

* **作用域**：[`Managed`](#scopes)
* **类型**：Boolean
  * `true`：Claude Code 永远不会运行市场声明的命令，不会安装或更新来自命令来源的插件，并停止加载已安装的此类插件
  * `false`：Claude Code 显式允许来自命令来源的插件
* **默认值**：未设置，因此 Claude Code 遵循 [`allowManagedHooksOnly`](#allowmanagedhooksonly)：将 hook 执行限制为托管设置的组织也会禁用命令来源。在 Agent SDK 的 `managedSettings` 选项中传入 [`disableAllHooks: true`](#disableallhooks) 的应用程序，其本身并不会禁用命令来源。

```json managed-settings.json theme={null}
{
  "disableCommandPluginSources": true
}
```

需要 Claude Code v2.1.229 或更高版本。

### `pluginSuggestionMarketplaces`

指定哪些市场的插件可以作为上下文安装建议出现在加载提示中，并固定在 `/plugin` **Discover** 选项卡顶部。内置的第一方 frontend-design 提示不受影响。建议来自每个插件在其市场条目中的 `relevance` 声明。

* **作用域**：[`Managed`](#scopes)
* **类型**：市场名称数组
* **默认值**：未设置，因此不会显示任何市场声明的建议

```json managed-settings.json theme={null}
{
  "pluginSuggestionMarketplaces": ["acme-corp-plugins"]
}
```

只有当该市场已在机器上注册，并且其注册来源也在同一托管设置中声明（作为该名称的 [`extraKnownMarketplaces`](#extraknownmarketplaces) 条目，或作为 [`strictKnownMarketplaces`](#strictknownmarketplaces) 的条目）时，名称才会生效。对于以允许列表中的名称但从不同来源注册的市场，Claude Code 会忽略它。官方市场不受来源要求限制：仅将其名称加入允许列表即可，因为该名称只能从官方 Anthropic 来源注册。请参阅[按上下文建议插件](https://code.claude.com/docs/zh-CN/plugins/relevance.md)。

### `pluginTrustMessage`

在 Claude Code 安装前显示的插件信任警告中添加您组织自己的文本，例如确认来自您内部市场的插件已经过审查。

* **作用域**：[`Managed`](#scopes)
* **类型**：字符串
* **默认值**：未设置，因此 Claude Code 仅显示标准警告

```json managed-settings.json theme={null}
{
  "pluginTrustMessage": "All plugins from our marketplace are approved by IT"
}
```

### `strictKnownMarketplaces`

限制组织中的人员可以添加哪些插件市场来源以及从中安装插件。Claude Code 在添加市场以及安装、更新、刷新和自动更新插件时，在任何网络或文件系统操作之前强制执行允许列表，因此在您设置该策略之前已有人添加的市场，一旦其来源不再匹配，就无法用于获取插件。被阻止的用户会看到一条指明托管策略的错误。

如果您在 [claude.ai 管理控制台](https://code.claude.com/docs/zh-CN/server-managed-settings.md)中设置此键，那么当组织中任何人在 claude.ai 上从 git 仓库添加市场时，claude.ai 也会应用它，如[限制的工作方式](https://code.claude.com/docs/zh-CN/plugins/org.md#restrict-what-users-can-install)所述。

* **作用域**：[`Managed`](#scopes)
* **类型**：市场来源对象数组；请参阅[允许的来源类型](#allowed-source-types)
* **默认值**：未设置，因此用户可以添加任何市场。空数组表示完全锁定，会阻止所有市场来源，包括官方 Anthropic 市场

此示例允许两个 GitHub 仓库（其中一个固定到 `v2.0` ref）和一个托管的 `marketplace.json` URL：

```json managed-settings.json theme={null}
{
  "strictKnownMarketplaces": [
    { "source": "github", "repo": "acme-corp/approved-plugins" },
    { "source": "github", "repo": "acme-corp/security-tools", "ref": "v2.0" },
    { "source": "url", "url": "https://plugins.example.com/marketplace.json" }
  ]
}
```

您也可以将此键写作 `allowedMarketplaces`；[市场键别名](#marketplace-key-aliases)介绍了 Claude Code 如何处理该别名以及哪个版本接受它。此键是一个策略门控：它控制用户可以添加什么，但不注册任何内容。要在一个文件中同时限制和预注册，请参阅[与 `extraKnownMarketplaces` 结合使用](#combine-with-extraknownmarketplaces)。有关面向用户的视图，请参阅[托管市场限制](https://code.claude.com/docs/zh-CN/plugins/org.md#restrict-what-users-can-install)。

#### 允许的来源类型

下表为每种来源类型展示一个允许列表条目及其接受的字段。大多数类型精确匹配；`hostPattern` 和 `pathPattern` 按正则表达式匹配，`github` 条目可以使用[所有者通配符](#owner-wildcards)。

| 来源 | 示例条目 | 字段 |
| :- | :- | :- |
| `github` | `{ "source": "github", "repo": "acme-corp/plugins", "ref": "main", "path": "marketplace" }` | `repo` 必填；`ref` 为分支或标签；`path` 为子目录 |
| `git` | `{ "source": "git", "url": "https://gitlab.example.com/tools/plugins.git", "ref": "production" }` | `url` 必填；`ref` 和 `path` 与 `github` 相同 |
| `url` | `{ "source": "url", "url": "https://plugins.example.com/marketplace.json", "headers": { "Authorization": "Bearer ${TOKEN}" } }` | `url` 必填；`headers` 添加用于身份验证访问的 HTTP 标头 |
| `file` | `{ "source": "file", "path": "/opt/acme-corp/plugins/marketplace.json" }` | `path` 必填，为 `marketplace.json` 文件的绝对路径 |
| `directory` | `{ "source": "directory", "path": "/opt/acme-corp/approved-marketplaces" }` | `path` 必填，为包含 `.claude-plugin/marketplace.json` 的目录的绝对路径 |
| `hostPattern` | `{ "source": "hostPattern", "hostPattern": "^github\\.example\\.com$" }` | `hostPattern` 必填，为在市场主机中任意位置匹配的正则表达式；使用 `^` 和 `$` 锚定以匹配整个主机 |
| `pathPattern` | `{ "source": "pathPattern", "pathPattern": "^/opt/approved/" }` | `pathPattern` 必填，为在 `file` 和 `directory` 来源的 `path` 中任意位置匹配的正则表达式；以 `^` 开头以固定前缀 |
| `skills-dir` | `{ "source": "skills-dir" }` | 无字段 |

部分来源类型具有表格之外的规则：

* **`url`**：URL 市场仅下载 `marketplace.json` 文件，Claude Code 不会按相对路径从该服务器获取插件文件，因此其插件必须使用相对路径以外的[插件来源](https://code.claude.com/docs/zh-CN/plugins/marketplace-reference.md#plugin-sources)，例如归档 URL（可以位于同一主机上）。对于使用相对路径的插件，请改用基于 Git 的市场。请参阅[使用相对路径的插件在基于 URL 的市场中失败](https://code.claude.com/docs/zh-CN/plugins/troubleshooting.md#plugins-with-relative-paths-fail-in-url-based-marketplaces)。
* **`hostPattern`**：使用它可允许内部 GitHub Enterprise 或 GitLab 服务器上的所有市场，而无需逐个列出每个仓库。Claude Code 将 `github` 来源与 `github.com` 进行匹配，从 `url` 来源中获取主机名，并根据 [git URL](https://git-scm.com/docs/git-clone#_git_urls) 的形式从 `git` 来源中获取主机名：

  * 带有协议的 URL，例如 `https://` 或 `ssh://`：URL 中的主机名。
  * 不带协议、采用 git 的 `user@host:path` 形式的 SSH 地址，例如 `git@git.example.com:tools/plugins.git`：`@` 和 `:` 之间的主机，即 git 连接的主机。
  * 任何其他不带协议的形式：没有主机，因此没有 `strictKnownMarketplaces` `hostPattern` 条目能与之匹配。对于 `blockedMarketplaces` `hostPattern`，Claude Code 会从更广泛的形式中获取主机，因此拒绝列表条目仍可能匹配此类形式。在 v2.1.234 之前，`strictKnownMarketplaces` `hostPattern` 还会匹配一些 git 不视为 SSH 地址的形式。

  `file` 和 `directory` 来源没有主机，永远不会匹配 `hostPattern` 条目。
* **`pathPattern`**：使用它可在为网络来源设置 `hostPattern` 条目的同时允许文件系统市场。`".*"` 允许所有本地路径；更窄的模式（例如 `"^/opt/approved/"`）则限制到某个目录。
* **`skills-dir`**：任何允许列表（即使为空）也会阻止 Claude Code 从 `~/.claude/skills/` 和项目的 `.claude/skills/` 加载 [`@skills-dir` 插件](https://code.claude.com/docs/zh-CN/plugins/loading#plugins-shared-through-a-repository)。要继续加载它们，请添加 `{ "source": "skills-dir" }` 条目，并保持 [`strictPluginOnlyCustomization`](#strictpluginonlycustomization) 未设置。该条目在此键和 `blockedMarketplaces` 之外没有意义。

#### 所有者通配符

`repo` 值为 `"<owner>/*"` 的 `github` 条目匹配该 GitHub 所有者下的所有仓库。所有者通配符需要 Claude Code v2.1.223 或更高版本，并且仅在 `strictKnownMarketplaces` 和 `blockedMarketplaces` 中有效。在出现 `github` 来源的其他任何位置（例如 `extraKnownMarketplaces` 或 `/plugin marketplace add`），`repo` 值必须指定单个仓库。在 v2.1.223 之前，Claude Code 按字面比较该条目，因此允许列表条目不匹配任何仓库，拒绝列表条目也不阻止任何内容；单仓库条目在所有版本中均会强制执行。

此条目允许 `acme-corp` 组织中的任何市场仓库：

```json managed-settings.json theme={null}
{
  "strictKnownMarketplaces": [
    { "source": "github", "repo": "acme-corp/*" }
  ]
}
```

只有整个仓库名称位置可以是通配符。Claude Code 会将 `*`、`*/plugins` 或 `acme-corp/tools-*` 等条目视为无效而忽略，因此它们不匹配任何仓库。

两个设置之间的匹配规则有所不同：

| 规则 | `strictKnownMarketplaces` | `blockedMarketplaces` |
| - | - | - |
| 匹配的来源写法 | 仅 `owner/repo` 形式。克隆同一仓库的 git URL 不匹配 | 任何写法，包括解析到同一 github.com 仓库的 git URL |
| 所有者大小写 | 区分大小写 | 不区分大小写 |
| `ref` | 遵循精确条目规则：带有 `ref` 的条目仅匹配具有完全相同 ref 的来源，不带 ref 的条目仅匹配未指定 ref 的来源 | 不带 `ref` 的条目会阻止其匹配仓库的所有 ref |
| `path` | 比精确条目规则宽松：带有 `path` 的条目要求完全相同的值，不带 path 的条目匹配仓库内的任何路径 | 不带 `path` 的条目会阻止其匹配仓库的所有路径 |

#### 精确匹配

对于除所有者通配符 `github` 条目以及按正则表达式匹配的 `hostPattern` 和 `pathPattern` 条目之外的所有来源类型，只有当市场来源与某个条目精确匹配时，Claude Code 才允许用户添加。对于基于 git 的来源 `github` 和 `git`，精确匹配包括可选字段：

* `repo` 或 `url` 必须完全匹配
* `ref` 字段必须完全匹配，或两者均未定义
* `path` 字段必须完全匹配，或两者均未定义

例如，Claude Code 将以下每一对视为两个不同的来源：

* `{ "source": "github", "repo": "acme-corp/plugins" }` 和 `{ "source": "github", "repo": "acme-corp/plugins", "ref": "main" }`
* `{ "source": "github", "repo": "acme-corp/plugins", "path": "marketplace" }` 和 `{ "source": "github", "repo": "acme-corp/plugins" }`

#### 仅允许官方市场

要仅允许官方 Anthropic 市场，请列出其仓库：

```json managed-settings.json theme={null}
{
  "strictKnownMarketplaces": [
    { "source": "github", "repo": "anthropics/claude-plugins-official" }
  ]
}
```

使用此条目后，Claude Code 会保持已注册的官方市场可用，并且在新机器上，会在您首次启动交互式终端会话时自动注册该市场。自动注册最常遗漏以下情况：

* 在机器的首个交互式终端会话之前运行的非交互式环境。
* Claude Code 仅通过 VS Code 扩展运行过的机器。
* Claude Code 曾在阻止该市场的策略（例如空数组锁定）下运行过交互式终端会话的机器。Claude Code 会记录被阻止的尝试，并且在策略更改后不会重试。

在这些机器上，请将该市场添加到同一 `managed-settings.json` 中的 [`extraKnownMarketplaces`](#extraknownmarketplaces)，以便 Claude Code 自动注册它，或者运行 `claude plugin marketplace add anthropics/claude-plugins-official`。

#### 与 `extraKnownMarketplaces` 结合使用

这两个键的作用不同。下表对它们进行了比较：

| 方面 | `strictKnownMarketplaces` | `extraKnownMarketplaces` |
| - | - | - |
| 用途 | 组织策略强制执行 | 团队便利 |
| 设置文件 | 仅限托管设置 | 任何设置文件 |
| 行为 | 阻止不在允许列表中的添加 | 注册缺失的市场 |
| 强制执行时机 | 在网络和文件系统操作之前 | 对于用户设置或托管设置立即生效；对于仓库的文件，在工作区信任对话框之后生效 |
| 能否被覆盖 | 否，优先级最高 | 是，可被优先级更高的设置覆盖 |
| 来源格式 | 直接的来源对象 | 带有嵌套 `source` 对象的命名市场 |

要为所有用户同时限制并预注册某个市场，请在 `managed-settings.json` 中同时设置两者：

```json managed-settings.json theme={null}
{
  "strictKnownMarketplaces": [
    { "source": "github", "repo": "acme-corp/plugins" }
  ],
  "extraKnownMarketplaces": {
    "acme-tools": {
      "source": { "source": "github", "repo": "acme-corp/plugins" }
    }
  }
}
```

如果仅设置了 `strictKnownMarketplaces`，用户仍可以通过 `/plugin marketplace add` 自行添加允许的市场。官方 Anthropic 市场是 Claude Code 唯一会自动注册的市场，并且仅在允许列表允许时才会注册。[仅允许官方市场](#allow-only-the-official-marketplace)列出了自动注册会遗漏的机器。

### `strictPluginOnlyCustomization`

阻止来自用户和项目来源的 skill、Agent、hook 和 MCP 服务器，使它们只能来自插件或托管设置。将其与 [`strictKnownMarketplaces`](#strictknownmarketplaces) 结合使用可控制完整的自定义供应链：市场允许列表控制用户可以安装哪些插件。

* **作用域**：[`Managed`](#scopes)
* **类型**：`true` 表示锁定全部四种自定义，或者是一个数组，从 `"skills"`、`"agents"`、`"hooks"` 和 `"mcp"` 中指定要锁定的类型
* **默认值**：未设置，因此不锁定任何内容

此示例锁定 skill 和 hook，而不锁定 Agent 和 MCP 服务器：

```json managed-settings.json theme={null}
{
  "strictPluginOnlyCustomization": ["skills", "hooks"]
}
```

如果您锁定任何自定义类别，Claude Code 还会停止加载 [skills 目录插件](https://code.claude.com/docs/zh-CN/plugins/loading.md#skills-directory-plugins-blocked-by-managed-settings)，即用户存放在 `~/.claude/skills/` 或 `.claude/skills/` 下的插件文件夹。这些文件夹由用户自行放置，而不是从市场安装，因此 Claude Code 在此设置下将它们视为用户自定义。

下面的四个子键条目列出了每个自定义类别还会阻止哪些内容，以及哪些内容仍会加载。Claude Code 会忽略无法识别的类别名称，而不是使设置文件失败，因此您可以在所有客户端更新之前添加新的类别名称。

### `strictPluginOnlyCustomization.skills`

锁定 `skills` 类别。Claude Code 停止加载来自 `~/.claude/skills/` 和 `.claude/skills/` 的 skill、来自 `~/.claude/commands/` 和 `.claude/commands/` 的自定义命令、`--add-dir` 目录下的 skill 和命令，以及从您的 claude.ai 账户同步的 skill。它会继续加载随附 skill、托管策略目录中的 skill，以及来自 [skills 目录插件](https://code.claude.com/docs/zh-CN/plugins/loading.md#skills-directory-plugins-blocked-by-managed-settings)以外插件的 skill。

* **作用域**：[`Managed`](#scopes)
* **类型**：[`strictPluginOnlyCustomization`](#strictpluginonlycustomization) 数组中的字符串 `"skills"`
* **默认值**：不锁定

```json managed-settings.json theme={null}
{
  "strictPluginOnlyCustomization": ["skills"]
}
```

### `strictPluginOnlyCustomization.agents`

锁定 `agents` 类别。Claude Code 停止加载来自 `~/.claude/agents/`、`.claude/agents/` 和 `--add-dir` 目录的 Agent。它会继续加载内置 Agent、托管策略目录中的 Agent，以及来自 [skills 目录插件](https://code.claude.com/docs/zh-CN/plugins/loading.md#skills-directory-plugins-blocked-by-managed-settings)以外插件的 Agent。

* **作用域**：[`Managed`](#scopes)
* **类型**：[`strictPluginOnlyCustomization`](#strictpluginonlycustomization) 数组中的字符串 `"agents"`
* **默认值**：不锁定

```json managed-settings.json theme={null}
{
  "strictPluginOnlyCustomization": ["agents"]
}
```

### `strictPluginOnlyCustomization.hooks`

锁定 `hooks` 类别：

* **停止运行**：
  * 来自用户、项目和本地 `settings.json` 的 hook
  * skill、自定义命令和 Agent 中的 frontmatter hook，其来源为 [`skills`](#strictpluginonlycustomization-skills) 或 [`agents`](#strictpluginonlycustomization-agents) 锁定会阻止的来源
* **继续运行**：托管设置中的 hook，以及来自 [skills 目录插件](https://code.claude.com/docs/zh-CN/plugins/loading.md#skills-directory-plugins-blocked-by-managed-settings)以外插件的 hook

frontmatter hook 停止运行的 skill、自定义命令和 Agent，只要其自身类别未被锁定，仍会加载。

在 frontmatter 中声明了 `hooks` 的个人或项目 [skill](https://code.claude.com/docs/zh-CN/skills.md#where-skills-live) 还会失去其 [`allowed-tools`](https://code.claude.com/docs/zh-CN/skills#pre-approve-tools-for-a-skill) 预先批准，因此您的用户在使用该字段列出的工具时会收到权限提示。当此类 skill 通过 [`context: fork`](https://code.claude.com/docs/zh-CN/skills#run-skills-in-a-subagent) 在子代理中运行时，它会保留预先批准。未声明 `hooks` 的 skill 同样会保留其预先批准。

* **作用域**：[`Managed`](#scopes)
* **类型**：[`strictPluginOnlyCustomization`](#strictpluginonlycustomization) 数组中的字符串 `"hooks"`
* **默认值**：不锁定

```json managed-settings.json theme={null}
{
  "strictPluginOnlyCustomization": ["hooks"]
}
```

### `strictPluginOnlyCustomization.mcp`

锁定 `mcp` 类别。Claude Code 停止加载来自 `~/.claude.json` 和 `.mcp.json` 的 MCP 服务器。它会继续加载 [`managed-mcp.json`](https://code.claude.com/docs/zh-CN/managed-mcp) 服务器、来自 [`managedMcpServers`](#managedmcpservers) 的服务器，以及来自 [skills 目录插件](https://code.claude.com/docs/zh-CN/plugins/loading.md#skills-directory-plugins-blocked-by-managed-settings)以外插件的 MCP 服务器。

* **作用域**：[`Managed`](#scopes)
* **类型**：[`strictPluginOnlyCustomization`](#strictpluginonlycustomization) 数组中的字符串 `"mcp"`
* **默认值**：不锁定

```json managed-settings.json theme={null}
{
  "strictPluginOnlyCustomization": ["mcp"]
}
```

### `enabledPlugins`

启用或禁用单个[插件](https://code.claude.com/docs/zh-CN/plugins/overview.md)，以 `plugin-name@marketplace-name` 为键。在任何作用域中都没有条目的插件会回退到其 [`defaultEnabled`](https://code.claude.com/docs/zh-CN/plugins/manifest-reference#fields) 值。当您使用 `/plugin` 或 `claude plugin enable` 启用或禁用插件时，Claude Code 会为您写入此键。

* **作用域**：[`Any file`](#scopes)
* **类型**：将 `plugin-name@marketplace-name` 映射到 Boolean 的对象
* **默认值**：未设置，因此每个插件遵循其 `defaultEnabled` 值

此示例启用 `team-tools` 市场中的两个插件，并禁用 `personal` 中的一个插件：

```json settings.json theme={null}
{
  "enabledPlugins": {
    "code-formatter@team-tools": true,
    "deployment-tools@team-tools": true,
    "experimental-features@personal": false
  }
}
```

每个作用域有不同的用途：

* **用户设置**：您的个人插件偏好
* **项目设置**：与仓库中所有人共享的插件
* **本地设置**：按机器的覆盖，Claude Code 在此保存设置时会将其加入 gitignore
* **托管设置**：组织范围的策略。在此设置为 `false` 的插件在所有作用域中都被禁止安装，并在市场中隐藏

项目设置优先于用户设置，因此在 `~/.claude/settings.json` 中将插件设置为 `false` 不会禁用项目的 `.claude/settings.json` 所启用的插件。要在您的机器上退出项目启用的插件，请改为在 `.claude/settings.local.json` 中将其设置为 `false`。由托管设置强制启用的插件无法通过这种方式禁用，因为托管设置会覆盖本地设置。

在项目的 `.claude/settings.json` 中启用来自外部来源（例如 GitHub 仓库或 npm 包）的插件，并不会为其他人安装该插件。在每条加载插件的路径上，Claude Code 都会报告该插件未安装，直到每个用户[自行安装](https://code.claude.com/docs/zh-CN/plugins/org.md#require-plugins-per-repository)。

### `extraKnownMarketplaces`

按名称注册额外的插件市场，使打开该仓库的人或您的托管设置覆盖到的所有人无需自行添加即可获得该市场。Claude Code 会注册每个它尚不知道的市场。[`enabledPlugins`](#enabledplugins) 从中指定的插件是否会安装，取决于插件的来源以及由哪个文件启用它；该条目介绍了相关规则。

* **作用域**：[`Any file`](#scopes)。只有在您接受该文件夹的工作区信任对话框后，Claude Code 才会识别仓库的 `.claude/settings.json` 或 `.claude/settings.local.json` 中的条目；在您尚未信任的文件夹中（包括在那里执行的 `-p` 运行），它会忽略这些条目且不显示消息。
* **类型**：将市场名称映射到对象的对象，该对象包含一个 `source` 对象和一个可选的 `autoUpdate` Boolean
* **默认值**：未设置

此示例注册一个 GitHub 市场和一个来自自托管 git URL 的市场：

```json settings.json theme={null}
{
  "extraKnownMarketplaces": {
    "acme-tools": {
      "source": {
        "source": "github",
        "repo": "acme-corp/claude-plugins"
      }
    },
    "security-plugins": {
      "source": {
        "source": "git",
        "url": "https://git.example.com/security/plugins.git"
      }
    }
  }
}
```

[信任文件夹之前会运行什么](https://code.claude.com/docs/zh-CN/permissions.md#what-runs-before-you-trust-a-folder)将信任门控与仓库可以提供的其他内容进行了比较。您也可以将此键写作 `additionalMarketplaces`；请参阅[市场键别名](#marketplace-key-aliases)。

在 `source` 旁边设置 `"autoUpdate": true`，可让 Claude Code 在启动后于后台刷新该市场并更新其已安装的插件。省略时，`claude-plugins-official` 和大多数其他官方 Anthropic 市场默认为 `true`，第三方市场默认为 `false`。请参阅[配置自动更新](https://code.claude.com/docs/zh-CN/plugins/install.md#keep-plugins-updated)。

当多个设置文件以相同名称定义市场条目时，Claude Code 会完整使用[优先级最高的文件](https://code.claude.com/docs/zh-CN/settings.md#settings-precedence)中的条目。该条目会替换优先级较低的条目，且不继承其任何字段，因此重新定义无法将一个文件的 `source.headers` 凭据与另一个文件控制的 URL 组合在一起。在 v2.1.228 之前，Claude Code 会逐字段合并同名条目，因此优先级较高的文件中的条目可能会继承其未设置的字段，包括另一个文件的 `headers`。

#### 市场来源类型

`source` 对象采用以下形式之一：

* **`github`**：GitHub 仓库，使用 `repo`
* **`git`**：任何 git URL，使用 `url`
* **`url`**：指向 `marketplace.json` 文件的直接 URL，使用 `url`，以及用于身份验证访问的可选 `headers` 和 `headersHelper`。`headersHelper` 指定一条命令，该命令输出有效期过短而无法列在 `headers` 中的标头值，需要 Claude Code v2.1.238 或更高版本
* **`file`**：指向 `marketplace.json` 文件的本地路径，使用 `path`
* **`directory`**：本地文件系统路径，使用 `path`。可用于开发，或用于您的组织[部署到每台机器](https://code.claude.com/docs/zh-CN/plugins/mods/admin.md#install-your-organizations-mods)的市场。
* **`settings`**：直接在设置文件中声明的内联市场，无需托管仓库，使用 `name` 和 `plugins`

`git` 来源类型适用于任何 git 托管服务，包括自托管的 GitLab 和 Bitbucket。Claude Code 使用与该机器上 `git clone` 相同的身份验证方式克隆仓库：已配置的凭据助手或 SSH 密钥。`GITHUB_TOKEN` 等提供商令牌通过读取它的凭据助手生效。有关设置详情，请参阅[私有仓库](https://code.claude.com/docs/zh-CN/plugins/host-marketplace.md#grant-access-to-a-private-marketplace)。

对于 `github` 和 `git` 来源，Claude Code 在克隆市场仓库以添加或更新它时，永远不会下载 [Git LFS](https://git-lfs.com) 内容。由 LFS 跟踪的文件会被检出为指针文件，添加或更新的输出会报告其数量。

`source` 对象中的 `skipLfs` 字段会被接受，但没有任何效果。在 v2.1.274 之前，除非您设置了 `"skipLfs": true`，否则 Claude Code 会下载 LFS 内容。

对于 `url` 来源，当 `headers` 中的凭据会过期、需要由命令生成新凭据时，请在 `source` 对象中设置 `headersHelper`。需要 Claude Code v2.1.238 或更高版本。有关该命令必须输出的内容以及 Claude Code 在何处运行它，请参阅[编写 headersHelper 命令](https://code.claude.com/docs/zh-CN/plugins/host-marketplace.md#write-the-headershelper-command)；有关 Claude Code 不运行它的情况，请参阅[Claude Code 何时跳过 headersHelper 命令](https://code.claude.com/docs/zh-CN/plugins/host-marketplace.md#when-claude-code-skips-a-headershelper-command-or-drops-its-output)。在 `https://` 市场 URL 上设置 `headersHelper` 后，Claude Code 会在两个时间点运行该命令，并在最多 60 秒内复用一次运行的输出：

* 每次获取该市场的 `marketplace.json` 之前，包括之后的刷新。Claude Code 会随该次获取一起发送输出的标头。
* 在市场 URL 的同源（即相同的协议、主机和端口）上每次下载插件归档之前。Claude Code 会随该次下载一起发送输出，其他下载不会获得这些标头。

对于您通过 [`--add-dir`](https://code.claude.com/docs/zh-CN/permissions#what-runs-before-you-trust-a-folder) 添加的目录，Claude Code 会忽略其 `.claude/settings.json` 或 `.claude/settings.local.json` 中设置的任何 `headersHelper`（无论是在 `url` 来源上还是在内联插件条目上），并仅发送该文件中设置的固定 `headers`。[用户如何接受 headersHelper 命令](https://code.claude.com/docs/zh-CN/plugins/host-marketplace.md#how-users-accept-a-headershelper-command)介绍了其他设置文件的情况。

`settings` 来源中列出的插件必须引用外部来源（例如 GitHub 或 npm），并且 `name` 必须与市场键匹配。您仍需在 `enabledPlugins` 中单独启用每个插件。此示例以内联方式声明一个插件：

```json settings.json theme={null}
{
  "extraKnownMarketplaces": {
    "team-tools": {
      "source": {
        "source": "settings",
        "name": "team-tools",
        "plugins": [
          {
            "name": "code-formatter",
            "source": {
              "source": "github",
              "repo": "acme-corp/code-formatter"
            }
          }
        ]
      }
    }
  }
}
```

`source: 'settings'` 下自身 `source` 为 [`archive`](https://code.claude.com/docs/zh-CN/plugins/marketplace-reference#archive-plugin-source) 的插件条目可以为归档下载设置 `headers`。如果您要放入 `headers` 的值是短期有效的（例如您的注册表按请求生成的令牌），请改为设置 `headersHelper` 命令。一个条目可以同时设置两者。这两个字段都需要 Claude Code v2.1.238 或更高版本。

Claude Code 会随该插件的归档下载发送条目的 `headers` 以及命令输出的任何内容，而不会随其他任何下载发送。只有当用户[单独安装或更新该插件](https://code.claude.com/docs/zh-CN/plugins/host-marketplace.md#how-users-accept-a-headershelper-command)时，Claude Code 才会运行该命令。另外三条规则取决于条目所在的文件：

* **`strict`**：与市场的 `marketplace.json` 中的条目不同，设置中的条目不需要 `"strict": false`，因为设置文件不包含可内联的清单字段。请参阅[严格模式](https://code.claude.com/docs/zh-CN/plugins/marketplace-reference.md#strict-mode)。
* **文件夹信任**：对于项目的 `.claude/settings.json` 或 `.claude/settings.local.json` 中的条目，Claude Code 仅在用户也已[信任该文件夹](https://code.claude.com/docs/zh-CN/permissions.md#what-runs-before-you-trust-a-folder)后才运行该命令。
* **标头过滤**：Claude Code 会从项目的 `.claude/settings.json` 或 `.claude/settings.local.json` 中的条目里丢弃[请求路由和客户端身份标头名称](https://code.claude.com/docs/zh-CN/plugins/host-marketplace.md#when-claude-code-skips-a-headershelper-command-or-drops-its-output)，因为仓库可以提供这些文件。Claude Code 对目录条目以及 `--add-dir` 目录设置中的条目应用相同的过滤，而对您的用户设置、`--settings` 文件或托管设置中的条目不应用过滤。

#### 市场键别名

在 Claude Code v2.1.232 或更高版本中，您可以将 `extraKnownMarketplaces` 写作 `additionalMarketplaces`，将 `strictKnownMarketplaces` 写作 `allowedMarketplaces`。Claude Code 对每个别名的处理方式如下：

* 较早版本会忽略别名，因此在较旧版本也会读取的文件中（例如为混合 Claude Code 版本的机群提供的托管设置文件）请保留规范写法。
* 在任何接受规范键的设置文件中，Claude Code 读取别名的方式与读取规范键完全相同。
* Claude Code 在更新文件时可能会将 `additionalMarketplaces` 重写为 `extraKnownMarketplaces`。
* 如果您在一个文件中同时设置了两种写法，Claude Code 会使用规范值并忽略别名。

### `pluginConfigs`

存储您在插件的 [`userConfig`](https://code.claude.com/docs/zh-CN/plugins/manifest-reference#user-configuration) 配置对话框中提供的非敏感答案，以插件 ID 为键。当您填写该对话框时，Claude Code 会将此键写入您的用户设置，因此您无需手动编辑。Claude Code 会将敏感选项改为存储在 macOS 钥匙串中，当钥匙串拒绝写入时回退到 `~/.claude/.credentials.json`；在没有受支持钥匙串的平台上，会将其存储在 `~/.claude/.credentials.json` 中。

* **作用域**：[`User or managed`](#scopes)
* **类型**：将插件 ID 映射到对象的对象，该对象包含一个 `options` 字段（将每个选项名称映射到字符串、数字、Boolean 或字符串数组），以及一个可选的 `mcpServers` 字段（以相同结构保存每个服务器的用户配置值）
* **默认值**：未设置

此示例为来自 `acme-tools` 的 `deployer` 插件存储 `api_endpoint` 选项：

```json settings.json theme={null}
{
  "pluginConfigs": {
    "deployer@acme-tools": {
      "options": {
        "api_endpoint": "https://api.example.com"
      }
    }
  }
}
```

内置插件以带 `@builtin` 后缀的相同键存储其选项。例如，控制 Claude Code 是否读取 `AGENTS.md` 文件的 [**Project instructions**](https://code.claude.com/docs/zh-CN/memory.md#choose-which-instruction-files-load) 设置为 `pluginConfigs["cc-plugin-agents-md@builtin"].options.instructionFiles`。在 v2.1.285 之前，该插件的 ID 为 `agents-md@builtin`。更高版本会读取这两个 ID 中任意一个下的条目。

Claude Code 会忽略项目和本地条目，因为它会将这些值代入插件的 hook、MCP 和 LSP 配置中，而克隆的仓库不得能够提供这些值。

### `prependPlugins`

列出托管插件，其 [mod](https://code.claude.com/docs/zh-CN/plugins/mods/overview.md) 会按列出的顺序在用户安装的所有 mod 之前运行。在托管设置中设置此键时，请在列表中指定 `sec-default@builtin` 以保留内置守卫。在托管设置中，如果某个 id 对应的插件不算作您组织的插件，Claude Code 会跳过它。有关这些条件以及两个排序键如何协同工作，请参阅[安装您组织的 mod 并设置顺序](https://code.claude.com/docs/zh-CN/plugins/mods/admin.md#install-your-organizations-mods)。

* **作用域**：[`User or managed`](#scopes)。Claude Code 从托管设置中读取此键。仅当机器上没有托管设置，且会话的凭据不是 Team 或 Enterprise `/login` 时，它才会从用户设置中读取此键。它会忽略项目设置、本地设置以及 `--settings` 文件中的此键。
* **类型**：`plugin-name@marketplace-name` 字符串数组
* **默认值**：未设置

```json managed-settings.json theme={null}
{
  "extraKnownMarketplaces": {
    "acme-tools": {
      "source": { "source": "directory", "path": "/opt/acme/claude-plugins" }
    }
  },
  "enabledPlugins": { "acme-guard@acme-tools": true },
  "prependPlugins": ["acme-guard@acme-tools", "sec-default@builtin"]
}
```

### `appendPlugins`

列出托管插件，其 [mod](https://code.claude.com/docs/zh-CN/plugins/mods/overview.md) 会按列出的顺序在用户安装的所有 mod 之后运行。同时列在 `prependPlugins` 和 `appendPlugins` 中的 id 会被前置。在托管设置中，如果某个 id 对应的插件不[算作您组织的插件](https://code.claude.com/docs/zh-CN/plugins/mods/admin.md#install-your-organizations-mods)，Claude Code 会跳过它。

* **作用域**：[`User or managed`](#scopes)。Claude Code 从托管设置中读取此键。仅当机器上没有托管设置，且会话的凭据不是 Team 或 Enterprise `/login` 时，它才会从用户设置中读取此键。它会忽略项目设置、本地设置以及 `--settings` 文件中的此键。
* **类型**：`plugin-name@marketplace-name` 字符串数组
* **默认值**：未设置

```json managed-settings.json theme={null}
{
  "extraKnownMarketplaces": {
    "acme-tools": {
      "source": { "source": "directory", "path": "/opt/acme/claude-plugins" }
    }
  },
  "enabledPlugins": { "acme-audit@acme-tools": true },
  "appendPlugins": ["acme-audit@acme-tools"]
}
```

## MCP

控制 Claude Code 连接哪些 MCP 服务器，以及组织允许哪些 MCP 服务器。请参阅[使用 MCP 连接外部工具](https://code.claude.com/docs/zh-CN/mcp.md)和[托管 MCP 配置](https://code.claude.com/docs/zh-CN/managed-mcp.md)。

### `allowAllClaudeAiMcps`

在已部署 `managed-mcp.json` 的同时，加载 Claude Code 自行获取的 [claude.ai 连接器](https://code.claude.com/docs/zh-CN/mcp.md#use-mcp-servers-from-claude-ai)。如果没有此键，`managed-mcp.json` 将独占控制 MCP 服务器，并屏蔽这些连接器。

* **作用域**：[`Managed`](#scopes)。用户无法重新启用被独占控制屏蔽的连接器。
* **类型**：Boolean
  * `true`：Claude Code 在已部署 `managed-mcp.json` 的同时加载 claude.ai 连接器
  * `false`：已部署的 `managed-mcp.json` 独占控制 MCP 服务器，并屏蔽 [Claude Code 自行获取](https://code.claude.com/docs/zh-CN/mcp.md#how-connectors-reach-claude-code)的 claude.ai 连接器
* **默认值**：`false`，因此已部署的 `managed-mcp.json` 会屏蔽 Claude Code 自行获取的 claude.ai 连接器

```json managed-settings.json theme={null}
{
  "allowAllClaudeAiMcps": true
}
```

[`allowedMcpServers`](#allowedmcpservers) 和 [`deniedMcpServers`](#deniedmcpservers) 仍然适用于此键加载的连接器。交付到[云端会话](https://code.claude.com/docs/zh-CN/claude-code-on-the-web.md)的连接器，如果其主机带有 `managed-mcp.json`（例如自托管 runner），则仍保持屏蔽状态。请参阅[在托管集合之外允许 claude.ai 连接器](https://code.claude.com/docs/zh-CN/managed-mcp.md#allow-claude-ai-connectors-alongside-the-managed-set)。

### `allowClaudeInChromeWithManagedMcp`

允许内置的 [Claude in Chrome](https://code.claude.com/docs/zh-CN/chrome.md) 服务器与已部署的 `managed-mcp.json` 同时运行。如果没有此键，已部署的 `managed-mcp.json` 会在终端会话中阻止 Claude in Chrome。需要 Claude Code v2.1.282 或更高版本。

* **作用域**：[`Managed`](#scopes)，仅限设备自身的托管设置：通过 MDM 部署的 plist 或 HKLM 注册表项，或系统级 `managed-settings.json` 文件。Claude Code 会忽略服务器托管设置、用户可写的 HKCU 注册表以及用户或项目设置中的此键。
* **类型**：Boolean
  * `true`：内置的 Claude in Chrome 服务器可以与已部署的 `managed-mcp.json` 同时运行
  * `false`：已部署的 `managed-mcp.json` 会在终端会话中阻止 Claude in Chrome
* **默认值**：`false`，因此已部署的 `managed-mcp.json` 会在终端会话中阻止 Claude in Chrome

```json managed-settings.json theme={null}
{
  "allowClaudeInChromeWithManagedMcp": true
}
```

即使启用此键，针对 `claude-in-chrome` 的 [`deniedMcpServers`](#deniedmcpservers) 条目仍会阻止该服务器。请参阅[在托管集合之外允许 Claude in Chrome](https://code.claude.com/docs/zh-CN/managed-mcp.md#allow-claude-in-chrome-alongside-the-managed-set)。

### `allowedMcpServers`

将用户可以添加的 MCP 服务器加入允许列表。无论服务器在何处定义，Claude Code 都会阻止任何与条目不匹配的服务器，包括插件服务器、用户通过 `--mcp-config` 传入的服务器以及来自 claude.ai 的服务器。

内置服务器（例如 Claude in Chrome、Claude Code 在运行中的 [VS Code](https://code.claude.com/docs/zh-CN/vs-code.md#the-built-in-ide-mcp-server) 或 [JetBrains](https://code.claude.com/docs/zh-CN/jetbrains.md#the-built-in-ide-mcp-server) IDE 中连接的 `ide` 服务器）以及 CLI 自身配置的服务器不受允许列表限制，但拒绝列表仍然对它们适用。在 Claude Code v2.1.268 或更高版本中，[Claude Tag](https://code.claude.com/docs/zh-CN/claude-tag.md) 会话的 Slack 工具同样不受允许列表限制，但拒绝列表仍然对它们适用。进程内的 `type: "sdk"` 服务器不受这两个列表限制；它们由[启动会话的应用](https://code.claude.com/docs/zh-CN/mcp.md#how-connectors-reach-claude-code)注册。

您的组织交付的服务器同样不受允许列表限制，但拒绝列表仍然对它们适用。此豁免涵盖每个 [`managedMcpServers`](#managedmcpservers) 条目，以及值中[除 `env` 外未使用 `${VAR}` 展开](https://code.claude.com/docs/zh-CN/managed-mcp#how-allowlists-and-denylists-apply-to-the-managed-set)的任何 [`managed-mcp.json`](https://code.claude.com/docs/zh-CN/managed-mcp#exclusive-control-with-managed-mcp-json) 条目。完整的检查顺序请参阅[如何评估服务器](https://code.claude.com/docs/zh-CN/managed-mcp.md#how-a-server-is-evaluated)。在 v2.1.259 之前，来自 `managed-mcp.json` 的服务器也必须匹配。

* **作用域**：[`Any file`](#scopes)。除非设置了 [`allowManagedMcpServersOnly`](#allowmanagedmcpserversonly)，否则所有文件中的条目会合并为一个允许列表。将其部署在托管设置中即可强制执行。
* **类型**：对象数组，每个对象恰好包含一个键。使用 `serverName`（仅限字母、数字、连字符和下划线的字符串）；`serverCommand`（由命令及其参数组成的数组，需完全匹配）；或 `serverUrl`（带 `*` 通配符的 URL 模式）
* **默认值**：未设置，因此允许所有服务器；空数组会阻止用户添加的所有服务器

此示例仅允许由所列 `npx` 命令启动的 stdio 服务器：

```json settings.json theme={null}
{
  "allowedMcpServers": [
    { "serverCommand": ["npx", "-y", "@modelcontextprotocol/server-filesystem"] }
  ]
}
```

[`deniedMcpServers`](#deniedmcpservers) 条目优先，因此同时出现在两个列表中的服务器会被阻止。一旦列表中包含任何 `serverCommand` 条目，stdio 服务器就必须匹配某个 `serverCommand` 条目；一旦列表中包含任何 `serverUrl` 条目，远程服务器就必须匹配某个 `serverUrl` 条目：此时 `serverName` 匹配将不再放行该类服务器。请参阅[基于策略的允许列表和拒绝列表控制](https://code.claude.com/docs/zh-CN/managed-mcp.md#policy-based-control-with-allowlists-and-denylists)。

### `allowManagedMcpServersOnly`

使托管允许列表成为唯一生效的允许列表。启用后，Claude Code 仅从托管设置中读取 [`allowedMcpServers`](#allowedmcpservers)，并忽略用户、项目和本地设置中的允许列表；[`deniedMcpServers`](#deniedmcpservers) 仍会从所有设置作用域合并，因此用户仍可为自己阻止服务器。管理员设置此项，可确保用户自己的设置无法扩大托管允许列表所允许的范围。

* **作用域**：[`Managed`](#scopes)
* **类型**：Boolean
  * `true`：Claude Code 仅从托管设置中读取 `allowedMcpServers`，并忽略用户、项目和本地设置中的允许列表
  * `false`：所有设置作用域中的允许列表会合并
* **默认值**：`false`，因此所有设置作用域中的允许列表会合并

此示例将允许列表锁定为托管设置，并仅允许名为 `github` 的服务器：

```json managed-settings.json theme={null}
{
  "allowManagedMcpServersOnly": true,
  "allowedMcpServers": [
    { "serverName": "github" }
  ]
}
```

用户仍可添加自己的 MCP 服务器；只有与托管允许列表匹配的服务器才会加载。请参阅[将允许列表限制为仅托管设置](https://code.claude.com/docs/zh-CN/managed-mcp.md#restrict-the-allowlist-to-managed-settings-only)。

### `deniedMcpServers`

阻止特定的 MCP 服务器。无论匹配的服务器在何处定义，Claude Code 都会拒绝加载，包括插件服务器、通过 `--mcp-config` 传入的服务器、来自 `managed-mcp.json` 的服务器、来自 [`managedMcpServers`](#managedmcpservers) 的服务器，以及[它自行获取](https://code.claude.com/docs/zh-CN/mcp.md#how-connectors-reach-claude-code)的 claude.ai 连接器。进程内的 `type: "sdk"` 服务器不受限制；它们由启动会话的应用注册。

* **作用域**：[`Any file`](#scopes)。所有文件中的条目会合并为一个拒绝列表，[`allowManagedMcpServersOnly`](#allowmanagedmcpserversonly) 不会改变这一点。将其部署在托管设置中即可强制执行。
* **类型**：对象数组，每个对象恰好包含一个键：`serverName`（字符串，因此可以使用 claude.ai 连接器的显示名称，例如 `"claude.ai Slack"`）；`serverCommand`（由命令及其参数组成的数组，需完全匹配）；或 `serverUrl`（带 `*` 通配符的 URL 模式）
* **默认值**：未设置，因此不阻止任何服务器；空数组同样不阻止任何服务器

```json settings.json theme={null}
{
  "deniedMcpServers": [
    { "serverName": "filesystem" }
  ]
}
```

拒绝列表优先于 [`allowedMcpServers`](#allowedmcpservers)，因此同时出现在两个列表中的服务器会被阻止。请参阅[基于策略的允许列表和拒绝列表控制](https://code.claude.com/docs/zh-CN/managed-mcp.md#policy-based-control-with-allowlists-and-denylists)。

### `disableClaudeAiConnectors`

关闭 [Claude Code 自行获取](https://code.claude.com/docs/zh-CN/mcp.md#how-connectors-reach-claude-code)的 [claude.ai MCP 连接器](https://code.claude.com/docs/zh-CN/mcp.md#use-mcp-servers-from-claude-ai)，使其既不获取也不连接这些连接器。任何设置文件中的 `true` 都会生效：签入仓库的项目 `.claude/settings.json` 可以让该仓库退出这些连接器，但项目级的 `false` 无法覆盖用户级或托管级的 `true`。

* **作用域**：[`Any file`](#scopes)
* **类型**：Boolean
  * `true`：Claude Code 既不获取也不连接这些连接器
  * `false`：与未设置相同；除非其他设置文件或 `ENABLE_CLAUDEAI_MCP_SERVERS` 将其关闭，否则 Claude Code 会获取您的连接器
* **默认值**：`false`，因此 Claude Code 会获取您的连接器
* **按会话覆盖**：将 [`ENABLE_CLAUDEAI_MCP_SERVERS`](https://code.claude.com/docs/zh-CN/env-vars) 设置为 `false` 可在单个会话中关闭连接器；无论由两者中的哪一个关闭，另一个都无法将其重新打开

```json settings.json theme={null}
{
  "disableClaudeAiConnectors": true
}
```

您通过 `--mcp-config` 显式传入的服务器不受影响。如需阻止单个连接器而不是全部连接器，请使用 [`deniedMcpServers`](#deniedmcpservers)。请参阅[禁用 claude.ai 连接器](https://code.claude.com/docs/zh-CN/mcp.md#disable-claude-ai-connectors)。

### `disabledMcpjsonServers`

拒绝项目 `.mcp.json` 文件中定义的特定服务器，使 Claude Code 永远不会连接它们，也不会请求您批准它们。任何设置文件中的拒绝都会生效，包括签入仓库的项目 `.claude/settings.json`。

* **作用域**：[`Any file`](#scopes)
* **类型**：字符串数组，即服务器在 `.mcp.json` 中显示的名称
* **默认值**：未设置

```json settings.json theme={null}
{
  "disabledMcpjsonServers": ["filesystem"]
}
```

当您在批准对话框中拒绝某个服务器时，Claude Code 会将此键写入 `.claude/settings.local.json`。`claude mcp get <name>` 会将被拒绝的服务器显示为 `✘ Rejected (see disabledMcpjsonServers in settings)`。拒绝优先于 [`enabledMcpjsonServers`](#enabledmcpjsonservers) 和 [`enableAllProjectMcpServers`](#enableallprojectmcpservers)。

### `enableAllProjectMcpServers`

无需询问即批准项目 `.mcp.json` 文件中定义的所有 MCP 服务器。当您在批准对话框中选择批准所有服务器时，Claude Code 会将此键写入 `.claude/settings.local.json`。

* **作用域**：[`Any file`](#scopes)。在您尚未接受信任对话框的文件夹中，Claude Code 会采用用户设置、托管设置和 `--settings` 中的此键，而忽略共享项目文件中的此键，这同时适用于会话以及 `claude mcp list` 和 `claude mcp get`；[项目服务器批准与工作区信任](https://code.claude.com/docs/zh-CN/mcp.md#project-server-approvals-and-workspace-trust)说明了未被跟踪的 `.claude/settings.local.json` 何时也会被计入。
* **类型**：Boolean
  * `true`：Claude Code 无需询问即批准项目 `.mcp.json` 文件中定义的所有 MCP 服务器
  * `false`：Claude Code 会请求您批准每个服务器。在受信任的文件夹中，优先级较高文件中的 `false` 会覆盖优先级较低文件中的 `true`；在您尚未信任的文件夹中，任何被采用的文件中有 `true` 即可生效
* **默认值**：未设置，因此 Claude Code 会请求您批准每个服务器

```json settings.json theme={null}
{
  "enableAllProjectMcpServers": true
}
```

[`disabledMcpjsonServers`](#disabledmcpjsonservers) 条目仍会拒绝服务器。

### `enabledMcpjsonServers`

批准项目 `.mcp.json` 文件中定义的特定服务器，使 Claude Code 无需询问即可连接它们。当您在批准对话框中批准某个服务器时，Claude Code 会将此键写入 `.claude/settings.local.json`。

* **作用域**：[`Any file`](#scopes)。在您尚未接受信任对话框的文件夹中，Claude Code 会采用用户设置、托管设置和 `--settings` 中的此键，而忽略共享项目文件中的此键，这同时适用于会话以及 `claude mcp list` 和 `claude mcp get`；[项目服务器批准与工作区信任](https://code.claude.com/docs/zh-CN/mcp.md#project-server-approvals-and-workspace-trust)说明了未被跟踪的 `.claude/settings.local.json` 何时也会被计入。
* **类型**：字符串数组，即服务器在 `.mcp.json` 中显示的名称
* **默认值**：未设置

此示例批准项目 `.mcp.json` 中的 `memory` 和 `github` 服务器：

```json settings.json theme={null}
{
  "enabledMcpjsonServers": ["memory", "github"]
}
```

[`disabledMcpjsonServers`](#disabledmcpjsonservers) 条目仍会拒绝服务器。

### `managedMcpServers`

通过托管设置向每位用户提供远程 MCP 服务器。用户保留自己添加的服务器，但无法编辑或删除您提供的服务器。需要 Claude Code v2.1.259 或更高版本。

* **作用域**：[`Managed`](#scopes)。Claude Code 会在用户、项目和本地设置中丢弃此键并发出警告；在第三方部署上的 Claude Desktop 应用 Code 标签页中，以及该应用的 Cowork 会话中，Claude Code 不会读取此键，因为 Claude Desktop 会自行提供并锁定这些会话的 MCP 服务器。
* **类型**：以服务器名称为键的对象。每个条目采用 `.mcp.json` 中 `http` 或 `sse` 服务器的结构：必需的 `https://` `url`，以及可选的 `headers`、`oauth` 和其他 HTTP 与 SSE 选项。Claude Code 会丢弃未通过验证的条目，[条目可以包含的内容](https://code.claude.com/docs/zh-CN/managed-mcp.md#what-an-entry-can-contain)列出了相关条件
* **默认值**：未设置，因此托管设置不提供任何服务器

此示例提供一个名为 `search` 的 HTTP 服务器：

```json managed-settings.json theme={null}
{
  "managedMcpServers": {
    "search": {
      "type": "http",
      "url": "https://search.example.com/mcp"
    }
  }
}
```

关于优先级、所提供的服务器如何与 `managed-mcp.json` 以及允许列表和拒绝列表结合，以及用户会看到什么，请参阅[通过托管设置提供服务器](https://code.claude.com/docs/zh-CN/managed-mcp.md#provide-servers-through-managed-settings)。

## Agent、会话与 worktree

设置默认 Agent，控制队友和跨会话消息，并配置 worktree。请参阅[子代理](https://code.claude.com/docs/zh-CN/sub-agents.md)和 [Worktrees](https://code.claude.com/docs/zh-CN/worktrees.md)。

### `agent`

将主线程作为指定名称的[子代理](https://code.claude.com/docs/zh-CN/sub-agents.md#invoke-subagents-explicitly)运行，使 Claude Code 将该子代理的系统提示词、工具限制和模型应用到您的会话中。同一个键还会为您从 `claude agents` 派发的会话设置默认 Agent。

* **作用域**：[`任意文件`](#scopes)
* **类型**：字符串，内置或自定义 Agent 的名称
* **默认值**：未设置，因此主线程以 Claude Code 的默认 Agent 运行
* **单会话覆盖**：`--agent` 在单个会话中优先于此键

```json settings.json theme={null}
{
  "agent": "code-reviewer"
}
```

插件自身的 `settings.json` 也可以提供此键；请参阅[随插件提供默认设置](https://code.claude.com/docs/zh-CN/plugins/components.md#default-settings)。

### `crossSessionInbound`

选择此会话如何处理[来自您其他 Claude Code 会话的消息](https://code.claude.com/docs/zh-CN/cross-session-messaging.md#control-inbound-messages)。当没有适用的值时，Claude Code 会根据两个会话的权限模式类别逐条消息做出决定。需要 Claude Code v2.1.224 或更高版本。

* **作用域**：[`任意文件`](#scopes)。项目或本地值仅在比托管设置、`--settings` 标志或用户设置给出的值更严格时才生效。
* **类型**：字符串，取值之一：
  * `"accept"`：Claude Code 将消息传递给 Claude
  * `"hold"`：Claude Code 为该消息显示通知，但不传递消息
  * `"refuse"`：Claude Code 丢弃该消息
* **默认值**：未设置，因此 Claude Code 逐条消息做出决定

```json settings.json theme={null}
{
  "crossSessionInbound": "hold"
}
```

Claude Code 首先读取托管设置，然后读取 `--settings` 标志，再读取用户设置，并应用找到的第一个值。`refuse` 比 `hold` 更严格，`hold` 比 `accept` 更严格。当所有受信任的来源都未设置值时，项目或本地的 `hold` 或 `refuse` 仍然生效，取代逐条消息的默认行为。在启用跨会话消息的会话中，此键在 `/config` 中显示为 **Messages from your other sessions**，该项会将其写入用户设置；此行需要 Claude Code v2.1.232 或更高版本，并且当 `--settings` 标志或托管设置设置了此键时，Claude Code 会将其隐藏。

当您设置了无法识别的值时，Claude Code 会[发出警告](https://code.claude.com/docs/zh-CN/errors.md#crosssessioninbound-must-be-one-of-accept-hold-refuse)。只要该值存在于用户、项目、本地或 `--settings` 文件中，Claude Code 就会暂留传入消息，即使优先级更高的来源设置了 `accept` 也是如此。其他来源设置的 `refuse` 仍然生效。修正或删除该值即可解除暂留。

当无法识别的值位于[托管设置](https://code.claude.com/docs/zh-CN/managed-settings.md)中时，Claude Code 会将其视为 `refuse`，直到管理员修正为止。在 v2.1.248 之前，Claude Code 会忽略无法识别的值且不发出警告。

### `disableAgentView`

关闭[后台 Agent 和 Agent 视图](https://code.claude.com/docs/zh-CN/agent-view.md)：`claude agents`、`--bg`、`/background` 以及按需监管器。在[托管设置](https://code.claude.com/docs/zh-CN/managed-settings.md)中设置此项可在整个组织范围内强制执行。

* **作用域**：[`任意文件`](#scopes)
* **类型**：布尔值
  * `true`：Claude Code 关闭 `claude agents`、`--bg`、`/background` 以及按需监管器
  * `false`：Agent 视图可用
* **默认值**：未设置，因此 Agent 视图可用
* **单会话覆盖**：[`CLAUDE_CODE_DISABLE_AGENT_VIEW`](https://code.claude.com/docs/zh-CN/env-vars) 可在单个会话中关闭 Agent 视图；两者中任一项将其关闭后，另一项都无法将其重新打开

```json settings.json theme={null}
{
  "disableAgentView": true
}
```

### `isolatePeerMachines`

在 Claude 的 `SendMessage` 到达您位于本机之外的某个会话之前，要求您明确批准；请参阅[要求批准跨机器消息](https://code.claude.com/docs/zh-CN/cross-session-messaging.md#require-approval-for-cross-machine-messages)。即使在 [`bypassPermissions` 模式](https://code.claude.com/docs/zh-CN/permission-modes#skip-all-checks-with-bypasspermissions-mode)下，也会出现批准提示。

* **作用域**：[`任意文件`](#scopes)。任何作用域中的 `true` 都会生效，因此签入的项目文件可以开启此要求，但不能将其关闭。
* **类型**：布尔值
  * `true`：在 Claude 的 `SendMessage` 到达您位于本机之外的某个会话之前，Claude Code 会请求您的批准
  * `false`：跨机器消息不会弹出提示
* **默认值**：未设置，因此跨机器消息不会弹出提示

```json settings.json theme={null}
{
  "isolatePeerMachines": true
}
```

跨机器 `SendMessage` 批准需要 Claude Code v2.1.224 或更高版本。

### `processWrapper`

在 macOS 和 Linux 上，在 [Claude Code 启动的后台进程](https://code.claude.com/docs/zh-CN/corporate-launcher.md#what-the-launcher-covers)前面放置一个企业启动器命令。Claude Code 运行启动器时会附加自身的命令行，因此启动器必须 exec 进入 Claude Code；有关启动器约定，请参阅[在企业启动器后运行 Claude Code](https://code.claude.com/docs/zh-CN/corporate-launcher.md)。需要 Claude Code v2.1.210 或更高版本。

* **作用域**：[`用户或托管`](#scopes)
* **类型**：字符串，作为 argv 前缀的启动器命令，例如带可选参数的绝对路径
* **默认值**：未设置，因此后台进程在不经包装的情况下启动
* **单会话覆盖**：[`CLAUDE_CODE_PROCESS_WRAPPER`](https://code.claude.com/docs/zh-CN/env-vars) 在单个会话中优先于此键

```json settings.json theme={null}
{
  "processWrapper": "/opt/corp/launcher --profile claude"
}
```

在 Windows 上，Claude Code 会忽略启动器，并以不经包装的方式启动每个进程。需要 Claude Code v2.1.210 或更高版本。

### `teammateMode`

选择 Claude Code 在何处显示 [agent team](https://code.claude.com/docs/zh-CN/agent-teams.md) 队友：在您的主终端窗格中，或者在终端支持时显示在拆分窗格中。请参阅[选择显示模式](https://code.claude.com/docs/zh-CN/agent-teams.md#choose-a-display-mode)。

* **作用域**：[`任意文件`](#scopes)。Claude Code 也会读取旧版本遗留在 `~/.claude.json` 中的值。
* **类型**：字符串，取值之一：
  * `"in-process"`：队友在您的主终端窗格中运行
  * `"auto"`：当您在 tmux 中运行，或在 iTerm2 中运行且 `PATH` 中有 `it2` 或已安装 tmux 时，使用拆分窗格；否则使用进程内模式
  * `"tmux"`：使用 tmux 或 iTerm2 拆分窗格，根据您的终端自动检测
  * `"iterm2"`：通过 `it2` CLI 使用 iTerm2 原生拆分窗格
* **默认值**：`"in-process"`
* **单会话覆盖**：`--teammate-mode` 在单个会话中优先于此键

```json settings.json theme={null}
{
  "teammateMode": "auto"
}
```

### `worktree`

配置 Claude Code 如何为 `--worktree`、`EnterWorktree` 工具以及隔离的子代理和后台会话创建和管理 [git worktree](https://code.claude.com/docs/zh-CN/worktrees.md)。

* **作用域**：[`任意文件`](#scopes)
* **类型**：包含 `baseRef`、`symlinkDirectories`、`sparsePaths` 和 `bgIsolation` 的对象
* **默认值**：未设置

以下示例从您当前的 `HEAD` 创建新 worktree 的分支，并将 `node_modules` 符号链接到每个 worktree 中：

```json settings.json theme={null}
{
  "worktree": {
    "baseRef": "head",
    "symlinkDirectories": ["node_modules"]
  }
}
```

要将 `.env` 等被 gitignore 的文件复制到新 worktree 中，请在项目根目录添加 [`.worktreeinclude` 文件](https://code.claude.com/docs/zh-CN/worktrees#copy-gitignored-files-into-worktrees)，而不是使用设置项。

### `worktree.baseRef`

选择新 worktree 从哪个 ref 创建分支。`"fresh"` 从 `origin/<default-branch>` 创建分支，得到与远程一致的干净树；`"head"` 从您当前的本地 `HEAD` 创建分支，因此未推送的提交和功能分支状态会出现在 worktree 中。

* **作用域**：[`任意文件`](#scopes)
* **类型**：字符串，取值之一：
  * `"fresh"`：新 worktree 从 `origin/<default-branch>` 创建分支
  * `"head"`：新 worktree 从您当前的本地 `HEAD` 创建分支，包括未推送的提交
* **默认值**：`"fresh"`

```json settings.json theme={null}
{
  "worktree": {
    "baseRef": "head"
  }
}
```

在链接的 worktree 内部，`"head"` 解析为该 worktree 的 `HEAD`，而不是主检出的 `HEAD`。

### `worktree.symlinkDirectories`

将主仓库中的目录符号链接到每个 worktree 中，这样就不会在磁盘上重复存放大型目录。

* **作用域**：[`任意文件`](#scopes)
* **类型**：字符串数组，相对于仓库根目录的目录路径
* **默认值**：未设置，因此 Claude Code 不创建任何目录的符号链接

以下示例将主仓库中的 `node_modules` 和 `.cache` 符号链接到每个新 worktree 中：

```json settings.json theme={null}
{
  "worktree": {
    "symlinkDirectories": ["node_modules", ".cache"]
  }
}
```

### `worktree.sparsePaths`

通过 git sparse-checkout 在每个 worktree 中仅检出列出的目录。Claude Code 只将这些目录以及根级文件写入磁盘，这在大型 monorepo 中速度更快；请参阅[仅检出您需要的目录](https://code.claude.com/docs/zh-CN/large-codebases.md#check-out-only-the-directories-you-need)。

* **作用域**：[`任意文件`](#scopes)
* **类型**：字符串数组，相对于仓库根目录的目录路径
* **默认值**：未设置，因此每个 worktree 检出整个树

请在列表中包含 `.claude`，以便每个 worktree 中也包含仓库的 `.claude/settings.json` 和 `.claude/rules/`。以下示例在每个 worktree 中仅检出 `.claude`、`packages/my-app` 和 `shared/utils` 以及根级文件：

```json settings.json theme={null}
{
  "worktree": {
    "sparsePaths": [".claude", "packages/my-app", "shared/utils"]
  }
}
```

只要存在稀疏 worktree，git 就会在仓库共享的 `.git/config` 中启用 `extensions.worktreeConfig`。

### `worktree.bgIsolation`

选择[后台会话](https://code.claude.com/docs/zh-CN/agent-view.md#how-file-edits-are-isolated)如何隔离其文件编辑。如果您使用 `←` 或 `/background` 将会话移至后台，则无论此键如何设置，该会话都会就地编辑文件。使用 `"worktree"` 时，Claude Code 会在主检出中阻止 `Edit` 和 `Write`，直到会话调用 `EnterWorktree`；使用 `"none"` 时，后台任务直接编辑工作副本。对于不适合使用 git worktree 的仓库，请设置为 `"none"`。

* **作用域**：[`任意文件`](#scopes)
* **类型**：字符串，取值之一：
  * `"worktree"`：Claude Code 在主检出中阻止 `Edit` 和 `Write`，直到会话调用 `EnterWorktree`
  * `"none"`：后台任务直接编辑工作副本
* **默认值**：`"worktree"`

```json settings.json theme={null}
{
  "worktree": {
    "bgIsolation": "none"
  }
}
```

在 git 仓库之外，失败的 [`WorktreeCreate` hook](https://code.claude.com/docs/zh-CN/worktrees#non-git-version-control) 会解除阻止，使会话可以就地编辑工作目录。

### `worktree.location`

选择远程机器上 [Desktop SSH 会话](https://code.claude.com/docs/zh-CN/desktop.md#choose-where-ssh-session-worktrees-go)创建 worktree 的文件夹，以替代 `<project-root>/.claude/worktrees/`。只有桌面应用会读取此键：`--worktree`、`EnterWorktree` 工具、隔离的子代理和后台会话都会忽略它。需要 Claude Desktop v1.44121.0 或更高版本。

* **作用域**：[`用户`](#scopes)，位于远程机器上的 `~/.claude/settings.json` 中
* **类型**：字符串，绝对路径或以 `~/` 开头的路径
* **默认值**：未设置，因此 worktree 位于项目内

以下示例将文件夹设置为 `~/worktrees`：

```json settings.json theme={null}
{
  "worktree": {
    "location": "~/worktrees"
  }
}
```

在 Desktop 中为 SSH 连接设置的 **Worktree folder** 优先。如果您的组织限制了会话可使用的文件夹，Desktop 会将 worktree 保留在项目内。

## Remote、桌面与通知

配置 Remote Control、云环境、桌面应用，以及 Claude Code 在需要您时发送的通知。请参阅 [Remote Control](https://code.claude.com/docs/zh-CN/remote-control.md)。

### `agentPushNotifEnabled`

允许 Claude 在认为值得发送时向您的手机发送推送通知，例如在长时间任务完成时。Claude 可以在 [Remote Control](https://code.claude.com/docs/zh-CN/remote-control.md) 已连接时发送通知。在 `/config` 中显示为 **Push when Claude decides**。在该处更改时，Claude Code 还会将该值保存到您的账户。

* **作用域**：[`Any file`](#scopes)。Claude Code 也会读取旧版本遗留在 `~/.claude.json` 中的值。
* **类型**：Boolean
  * `true`：Claude 在认为值得发送时可以向您的手机发送推送通知
  * `false`：Claude 不发送这些通知
* **默认值**：`false`

```json settings.json theme={null}
{
  "agentPushNotifEnabled": true
}
```

如果您在 [`claude remote-control`](https://code.claude.com/docs/zh-CN/remote-control#start-a-remote-control-session) 服务器下运行会话，这些会话将遵循保存在您账户中的值，而不是此设置项。要更改该值，请在终端的 Claude Code 会话中运行 `/config`，并打开或关闭 **Push when Claude decides**。如果开关已显示您想要的值，请先更改再改回，以便 Claude Code 将该值保存到您的账户。

请参阅 [移动推送通知](https://code.claude.com/docs/zh-CN/remote-control.md#mobile-push-notifications)。

### `awaySummaryEnabled`

在您离开几分钟后返回终端时，显示一行会话回顾。将其设置为 `false`，或在 `/config` 中关闭 **Session recap**，即可停止显示回顾。

* **作用域**：[`Any file`](#scopes)
* **类型**：Boolean
  * `true`：离开几分钟后返回时，您会看到一行会话回顾
  * `false`：Claude Code 不显示回顾
* **默认值**：未设置，因此回顾处于开启状态
* **单会话覆盖**：[`CLAUDE_CODE_ENABLE_AWAY_SUMMARY`](https://code.claude.com/docs/zh-CN/env-vars) 在单个会话中优先于此设置项，无论开启还是关闭

```json settings.json theme={null}
{
  "awaySummaryEnabled": false
}
```

Claude Code 在非交互模式下从不显示回顾。

### `disableArtifact`

> [!WARNING]
>
> 已弃用，并已由 [`enableArtifact`](#enableartifact) 取代。Claude Code 仍会将 `disableArtifact: true` 视为等同于 `enableArtifact: false`，并忽略 `disableArtifact: false`。

请改用 [`enableArtifact`](#enableartifact) 来关闭 [Artifact](https://code.claude.com/docs/zh-CN/artifacts.md) 工具，该工具会将会话输出发布为 claude.ai 上的私有网页。当您在 `/config` 中关闭 **Artifacts** 行时，Claude Code 会将 `enableArtifact` 写入您的用户设置并清除此设置项。

* **作用域**：[`Any file`](#scopes)
* **类型**：Boolean
  * `true`：Claude Code 会在该文件适用的每个会话中关闭 Artifact 工具，且其他任何文件都无法将其重新开启
  * `false`：被忽略；要保持工具开启，请删除该设置项
* **默认值**：未设置，因此该工具遵循您账户的[可用性](https://code.claude.com/docs/zh-CN/artifacts.md#availability)
* **单会话覆盖**：将 [`CLAUDE_CODE_DISABLE_ARTIFACT`](https://code.claude.com/docs/zh-CN/env-vars) 设置为 `1` 可在单个会话中关闭该工具

```json settings.json theme={null}
{
  "disableArtifact": true
}
```

[禁用 Artifact](https://code.claude.com/docs/zh-CN/artifacts.md#disable-artifacts) 列出了关闭该工具的所有方式。

### `disableDeepLinkRegistration`

阻止 Claude Code 向操作系统注册 `claude-cli://` 协议处理程序。[深度链接](https://code.claude.com/docs/zh-CN/deep-links.md)允许外部工具打开带有预填提示词的 Claude Code 会话。在协议处理程序注册受限或单独管理的环境中设置此项。

* **作用域**：[`Any file`](#scopes)
* **类型**：字符串 `"disable"`
* **默认值**：未设置，因此 Claude Code 会注册该处理程序

```json settings.json theme={null}
{
  "disableDeepLinkRegistration": "disable"
}
```

### `disableDesktopLocalSessions`

关闭在[桌面应用](https://code.claude.com/docs/zh-CN/desktop.md#local-sessions-on-managed-devices)中于设备本机运行的 Code 会话，适用于开发者应通过 SSH 在远程机器上工作的部署。将此设置项设为 `true` 后，用户会在 Code 标签页中看到：

* **环境选择器**：用户计算机对应的条目显示为灰色且不可选择，并带有工具提示 **Disabled by your organization**。在 Windows 上，WSL 条目也会以同样方式显示为灰色，不过 WSL 会话是否能在托管设备上运行由[另外的设置控制](https://code.claude.com/docs/zh-CN/admin-setup.md#wsl-sessions-in-claude-code-desktop)。
* **新会话**：如果已配置 [SSH 连接](https://code.claude.com/docs/zh-CN/desktop.md#ssh-sessions)，则默认使用第一个 SSH 连接
* **设备上的会话**：应用拒绝在设备上启动或恢复会话，包括通过 SSH 连接回同一台机器的会话
* **到其他主机的 SSH 会话和云端会话**：不受影响

桌面应用读取此设置项；终端 CLI 会忽略它。需要 Claude Desktop v1.37937.0 或更高版本。

* **作用域**：[`Managed`](#scopes)。默认情况下，桌面应用从[一个托管来源](https://code.claude.com/docs/zh-CN/managed-settings.md#how-claude-code-combines-managed-sources)读取此设置项。
* **类型**：Boolean；只有 JSON Boolean `true` 才会生效
  * `true`：桌面应用不提供设备本机的 Code 会话；现有本地会话仍会列出，但无法继续
  * `false`：本地会话保持可用
* **默认值**：未设置，因此本地会话可用

```json managed-settings.json theme={null}
{
  "disableDesktopLocalSessions": true
}
```

桌面应用会忽略任何其他值，而非 Boolean 的值（例如字符串 `"true"` 或 `1`）还会记录一条警告。请将其与 [`sshConfigs`](#sshconfigs) 搭配使用，使用户连接到可用的连接，并与 [`sshHostAllowlist`](#sshhostallowlist) 搭配使用，以限制用户可以访问的主机。请参阅[托管设备上的本地会话](https://code.claude.com/docs/zh-CN/desktop.md#local-sessions-on-managed-devices)。

Claude Desktop 会向 Code 会话提供从您的桌面配置派生的策略，例如第三方部署中的出站允许列表、文件系统沙箱和 MCP 限制。只要存在[管理员来源](https://code.claude.com/docs/zh-CN/managed-settings.md#how-claude-code-combines-managed-sources)（服务器托管设置、MDM 或操作系统级策略，或托管设置文件），Claude Code 就会忽略这些父级设置。因此，在此前没有任何此类来源的设备上通过其中之一部署此设置项（例如在第三方部署中），会使从桌面派生的策略不再生效。[允许嵌入宿主添加策略](https://code.claude.com/docs/zh-CN/managed-settings.md#let-an-embedding-host-add-policy)说明了父级设置在何种情况下仍可合并；这适用于您以这种方式部署的任何设置项，而不仅仅是此设置项。

### `disableRemoteControl`

关闭 [Remote Control](https://code.claude.com/docs/zh-CN/remote-control.md)：Claude Code 随后会拒绝 `claude remote-control`、`--remote-control` 标志、自动启动以及会话内开关，并报告其已被您组织的策略禁用。将其放在[托管设置](https://code.claude.com/docs/zh-CN/managed-settings.md)中，可实现按设备的 MDM 强制执行。

* **作用域**：[`Any file`](#scopes)
* **类型**：Boolean
  * `true`：Claude Code 拒绝 `claude remote-control`、`--remote-control` 标志、自动启动以及会话内开关
  * `false`：Remote Control 保持可用
* **默认值**：`false`

```json settings.json theme={null}
{
  "disableRemoteControl": true
}
```

### `enableArtifact`

关闭 [Artifact](https://code.claude.com/docs/zh-CN/artifacts.md) 工具，该工具会将会话输出发布为 claude.ai 上的私有网页。当您在 `/config` 中关闭 **Artifacts** 行时，Claude Code 会将此设置项写入您的用户设置，因此您通常无需手动编辑它。

* **作用域**：[`Any file`](#scopes)。每个文件都可以关闭该工具，但任何文件都无法将其重新开启。
* **类型**：Boolean
  * `false`：Claude Code 会在该文件适用的每个会话中关闭 Artifact 工具
  * `true`：与不设置该设置项相同，因为它永远不会覆盖来自其他文件、[`CLAUDE_CODE_DISABLE_ARTIFACT`](https://code.claude.com/docs/zh-CN/env-vars) 或您组织的[管理员设置](https://code.claude.com/docs/zh-CN/artifacts.md#manage-artifacts-for-your-organization)的 `false`
* **默认值**：未设置，因此该工具遵循您账户的[可用性](https://code.claude.com/docs/zh-CN/artifacts.md#availability)

```json settings.json theme={null}
{
  "enableArtifact": false
}
```

当您自己的用户设置以外的来源使该工具保持关闭时，Claude Code 会在 `/config` 中隐藏 **Artifacts** 行，因为在那里开启它不会产生任何改变。[禁用 Artifact](https://code.claude.com/docs/zh-CN/artifacts.md#disable-artifacts) 列出了关闭该工具的所有方式。

### `inputNeededNotifEnabled`

当权限提示或问题正在等待您输入时，在手机上接收推送通知。Claude Code 仅在 [Remote Control](https://code.claude.com/docs/zh-CN/remote-control.md) 已连接时发送这些通知。在 `/config` 中显示为 **Push when actions required**。

* **作用域**：[`Any file`](#scopes)。Claude Code 也会读取旧版本遗留在 `~/.claude.json` 中的值。
* **类型**：Boolean
  * `true`：在 Remote Control 已连接时，如有权限提示或问题正在等待，您会在手机上收到推送通知
  * `false`：Claude Code 不发送此类通知
* **默认值**：`false`

```json settings.json theme={null}
{
  "inputNeededNotifEnabled": true
}
```

请参阅 [移动推送通知](https://code.claude.com/docs/zh-CN/remote-control.md#mobile-push-notifications)。

### `preferredNotifChannel`

选择在任务完成或权限提示正在等待时 Claude Code 通知您的方式。[mod 的通知](https://code.claude.com/docs/zh-CN/plugins/mods/api.md#show-something-without-starting-a-turn)也使用同一设置。在 `/config` 中显示为 **Local notifications**。

* **作用域**：[`Any file`](#scopes)。Claude Code 也会读取旧版本遗留在 `~/.claude.json` 中的值。
* **类型**：字符串，取以下值之一：
  * `"auto"`：Claude Code 在 iTerm2、Ghostty 和 Kitty 中发送桌面通知，在 Terminal.app 中仅当其响铃提示音关闭时才响铃，在其他终端中不执行任何操作
  * `"terminal_bell"`：Claude Code 在任何终端中发出响铃字符
  * `"iterm2"`：Claude Code 发送 iTerm2 桌面通知
  * `"iterm2_with_bell"`：Claude Code 发送 iTerm2 桌面通知并响铃
  * `"kitty"`：Claude Code 发送 Kitty 桌面通知
  * `"ghostty"`：Claude Code 发送 Ghostty 桌面通知
  * `"notifications_disabled"`：Claude Code 不发送任何通知
* **默认值**：`"auto"`

```json settings.json theme={null}
{
  "preferredNotifChannel": "terminal_bell"
}
```

使用 `"auto"` 时，Claude Code 会在 iTerm2、Ghostty 和 Kitty 中发送桌面通知。在 Terminal.app 中，仅当您关闭了 Terminal 的响铃提示音时它才会发出响铃字符，而在其他终端中不执行任何操作。设置 `"terminal_bell"` 可在任何终端中发出响铃字符。请参阅[获取终端响铃或通知](https://code.claude.com/docs/zh-CN/terminal-config.md#get-a-terminal-bell-or-notification)。

### `remote.defaultEnvironmentId`

为您从 CLI 创建的云端会话（例如使用 `claude --cloud`）选择默认[云环境](https://code.claude.com/docs/zh-CN/cloud-environments.md)。当您使用 [`/remote-env`](https://code.claude.com/docs/zh-CN/cloud-environments#select-an-environment-from-the-cli) 选择环境时，Claude Code 会将此设置项写入您的用户设置。

* **作用域**：[`Any file`](#scopes)。对于自托管环境 ID，仅限用户设置或托管设置，或 `--settings` 标志。
* **类型**：字符串，即环境 ID，例如 `env_...` 或 `ccpool_...`
* **默认值**：未设置，因此如果您的列表中有 Anthropic 托管环境，Claude Code 会使用它；否则使用列表中第一个不是 [Remote Control 桥接环境](https://code.claude.com/docs/zh-CN/cloud-environments.md#the-default-environment)的环境；如果所有环境都是桥接环境，则使用第一个环境
* **单会话覆盖**：对于 `--environment` 所创建的那一个云端会话，该标志优先于此设置项

```json settings.json theme={null}
{
  "remote": {
    "defaultEnvironmentId": "env_0123abcd"
  }
}
```

以 `env_` 开头的 Anthropic 托管环境 ID 遵循标准设置优先级，因此仓库项目设置中的值会覆盖您在用户级别的选择。以 `ccpool_` 开头的[自托管环境](https://code.claude.com/docs/zh-CN/self-hosted-environments) ID 仅在来自用户设置、托管设置和 `--settings` 标志时才会被采用；Claude Code 会忽略仓库项目设置或本地设置中的此类 ID，并且 `/remote-env` 会显示它忽略了哪个值，因此签入仓库的文件无法将会话引导到您未选择的自托管环境上。

### `remoteControlAtStartup`

在每个交互式会话启动时自动连接 [Remote Control](https://code.claude.com/docs/zh-CN/remote-control.md)，而不是等待 `/remote-control`。将其设置为 `true` 可开启自动连接，设置为 `false` 可关闭。在 `/config` 中显示为 **Enable Remote Control for all sessions**。

* **作用域**：[`Any file`](#scopes)。Claude Code 也会读取旧版本遗留在 `~/.claude.json` 中的值。
* **类型**：Boolean
  * `true`：Claude Code 在每个交互式会话启动时自动连接 Remote Control
  * `false`：Claude Code 等待 `/remote-control`
* **默认值**：未设置，因此适用[自动连接默认值](https://code.claude.com/docs/zh-CN/remote-control.md#enable-remote-control-for-all-sessions)
* **单会话覆盖**：即使此设置项为 `false`，`--remote-control` 也会为单个会话开启 Remote Control，而没有任何标志可以为单个会话关闭它

```json settings.json theme={null}
{
  "remoteControlAtStartup": true
}
```

Claude Code 会忽略来自项目设置或本地设置的 `true`，因此仓库可以为其检出副本关闭自动连接，但无法开启它。有关各作用域的完整行为，请参阅[为所有会话启用 Remote Control](https://code.claude.com/docs/zh-CN/remote-control.md#enable-remote-control-for-all-sessions)以及[采用更严格值的安全设置项](https://code.claude.com/docs/zh-CN/settings.md#security-keys-where-the-stricter-value-applies)。

### `sshConfigs`

向 [Desktop](https://code.claude.com/docs/zh-CN/desktop.md#pre-configure-ssh-connections-for-your-team) 环境选择器添加 SSH 连接。管理员使用 `sshConfigs` 向团队分发共享连接。用户可以选择托管连接，并为其[设置自己的 **Worktree folder**](https://code.claude.com/docs/zh-CN/desktop.md#set-the-worktree-folder-on-an-existing-connection)。他们无法在应用中编辑其其他字段或删除它。

* **作用域**：[`User or managed`](#scopes)。桌面应用读取此设置项。默认情况下，它从[一个托管来源](https://code.claude.com/docs/zh-CN/managed-settings.md#how-claude-code-combines-managed-sources)读取托管连接。
* **类型**：对象数组，每个对象包含必需的 `id`、`name` 和 `sshHost`，以及可选的 `sshPort` 和 `sshIdentityFile`
* **默认值**：未设置

此示例添加一个名为 `Dev VM` 的连接，该连接连接到 `user@dev.example.com`：

```json settings.json theme={null}
{
  "sshConfigs": [
    {
      "id": "dev-vm",
      "name": "Dev VM",
      "sshHost": "user@dev.example.com"
    }
  ]
}
```

### `sshHostAllowlist`

限制 [Desktop SSH 会话](https://code.claude.com/docs/zh-CN/desktop.md#restrict-which-ssh-hosts-users-can-connect-to)可以连接的主机。只有 Desktop 应用读取此设置项；CLI 不读取。模式不区分大小写：`*` 匹配任何主机，`*.example.com` 匹配 `example.com` 及其所有子域，其他任何模式都与经过 `~/.ssh/config` 解析后的主机名进行精确匹配。空数组会关闭 SSH 会话。

在 Claude Desktop v3.1623.0 或更高版本中，会话的[跳板主机](https://code.claude.com/docs/zh-CN/desktop.md#allow-jump-hosts)也必须与某个模式匹配。

* **作用域**：[`Managed`](#scopes)。默认情况下，Desktop 从[一个托管来源](https://code.claude.com/docs/zh-CN/managed-settings.md#how-claude-code-combines-managed-sources)读取此设置项。
* **类型**：主机名模式数组
* **默认值**：未设置，因此允许任何主机

此示例允许 `devboxes.example.com` 及其子域，以及精确主机 `bastion.example.com`：

```json managed-settings.json theme={null}
{
  "sshHostAllowlist": ["*.devboxes.example.com", "bastion.example.com"]
}
```

Desktop 无法读取为主机列表的值（例如 `true` 或对象）在您更正之前会被视为空数组，但 `null` 除外，它被视为未设置。需要 Claude Desktop v2.26454.0 或更高版本。

如果您在优先级最高的来源中将 [`managedSourcesBehavior`](#managedsourcesbehavior) 设置为 `"merge"`，Desktop 会合并来自每个[管理员来源](https://code.claude.com/docs/zh-CN/managed-settings.md#how-claude-code-combines-managed-sources)的列表，并允许与其中任一列表匹配的主机。如果您在某个来源中设置了空数组，对于另一个来源所列出的主机，SSH 会话仍保持开启。

### `sshHostCommands`

向 [Desktop](https://code.claude.com/docs/zh-CN/desktop.md#create-an-ssh-connection-from-a-command) 环境选择器添加其主机来自某个命令的 SSH 连接。当您创建连接或替换其主机时，Desktop 会在您的计算机上运行该条目的命令，并连接到该命令输出的主机。需要在 macOS 或 Linux 上使用 Claude Desktop v3.6556.0 或更高版本，并使用 Claude 账户登录。

* **作用域**：[`User or managed`](#scopes)。只有桌面应用读取此设置项。它从计算机上的[一个托管设置文件或 MDM 策略](https://code.claude.com/docs/zh-CN/desktop.md#distribute-commands-to-your-team)读取托管条目，并忽略[服务器托管设置](https://code.claude.com/docs/zh-CN/server-managed-settings.md)中的此设置项。
* **类型**：对象数组，每个对象包含必需的 `id`、`name` 和 `command`，以及可选的 `sshIdentityFile` 和 `startDirectory`。Desktop 在不使用 shell 的情况下启动 `command`，因此请[将其写为程序加普通参数](https://code.claude.com/docs/zh-CN/desktop.md#write-the-command)。
* **默认值**：未设置

此示例添加一个名为 `Dev box` 的连接，其中 `acme-dev create` 代表您自己的工具：

```json settings.json theme={null}
{
  "sshHostCommands": [
    {
      "id": "devbox",
      "name": "Dev box",
      "command": "acme-dev create"
    }
  ]
}
```

在 Claude Code v2.1.307 之前，Edit 工具会拒绝向有效设置文件添加此设置项的更改，并报错 `Unrecognized field: sshHostCommands`。在这些版本上，请改用文本编辑器添加此设置项；Claude Code 可以加载包含此设置项的文件。在 Desktop 中，起作用的版本是 [Code 标签页的 Claude Code 版本](https://code.claude.com/docs/zh-CN/desktop.md#claude-code-version-in-the-code-tab)，而不是您终端中的版本。

## 身份验证与提供商

通过辅助脚本提供凭据；对于组织，可强制指定登录方式或组织。请参阅[身份验证](https://code.claude.com/docs/zh-CN/authentication.md)。

### `allowedProviders`

列出机器可以通过哪些服务访问 Claude，例如 Anthropic API、Amazon Bedrock 或 LLM 网关。使用未列出提供商的会话会在启动时、登录时以及下次联系 API 时被拒绝，因此在会话中途切换到未列出的提供商也会被拒绝。[拒绝消息](https://code.claude.com/docs/zh-CN/errors.md#managed-settings-dont-allow-this-api-provider)会指出是什么选择了该提供商以及继续操作的步骤。需要 Claude Code v2.1.285 或更高版本。

* **作用域**：[`Managed`](#scopes)。当服务器托管设置也下发列表时，由机器自身的管理员来源（MDM 策略和托管设置文件）设置的列表仍然生效：此时会话只能使用同时出现在两个列表中的提供商，因此服务器托管列表可以收窄机器允许的范围，但永远无法扩大。哪个机器来源的 `allowedProviders` 生效，遵循 [Claude Code 如何合并托管来源](https://code.claude.com/docs/zh-CN/managed-settings.md#how-claude-code-combines-managed-sources)。仅通过服务器托管设置下发的列表只会作用于[获取服务器托管设置](https://code.claude.com/docs/zh-CN/server-managed-settings.md#platform-availability)的会话。
* **类型**：字符串数组，每项为以下之一：
  * `"anthropic"`：Anthropic 自有主机上的 Anthropic API，通过 claude.ai 或 Console 登录或 API 密钥访问。将其与 [`forceLoginMethod`](#forceloginmethod) 或 [`forceLoginOrgUUID`](#forceloginorguuid) 搭配使用，可同时限制登录
  * `"bedrock"`：[Amazon Bedrock](https://code.claude.com/docs/zh-CN/amazon-bedrock.md)
  * `"vertex"`：[Google Cloud 的 Agent Platform](https://code.claude.com/docs/zh-CN/google-vertex-ai.md)，前身为 Vertex AI
  * `"foundry"`：[Microsoft Foundry](https://code.claude.com/docs/zh-CN/microsoft-foundry.md)
  * `"anthropicAws"`：[Claude Platform on AWS](https://code.claude.com/docs/zh-CN/claude-platform-on-aws.md)
  * `"mantle"`：Amazon Bedrock [Mantle 端点](https://code.claude.com/docs/zh-CN/amazon-bedrock.md#use-the-mantle-endpoint)。[同时运行 Mantle 和 Invoke API](https://code.claude.com/docs/zh-CN/amazon-bedrock.md#run-mantle-alongside-the-invoke-api) 的会话会使用这两个提供商，因此需要同时列出 `"bedrock"` 和 `"mantle"`
  * `"customEndpoint"`：发送到其他主机的 Anthropic API 或云提供商 API，例如由 `ANTHROPIC_BASE_URL` 或提供商端点变量（如 `ANTHROPIC_BEDROCK_BASE_URL`）指定的 [LLM 网关](https://code.claude.com/docs/zh-CN/llm-gateway)。Claude Code 仅接受托管 [`env`](#env) 块所固定的确切值
  * `"gateway"`：[Cloud gateway](https://code.claude.com/docs/zh-CN/claude-apps-gateway) 登录
* **默认值**：未设置，因此可以使用任何提供商

```json managed-settings.json theme={null}
{
  "allowedProviders": ["anthropic", "bedrock"]
}
```

每个云提供商条目都指该提供商自己的服务，包括其区域端点、FIPS 端点和私有端点。

Claude Code 无法识别为提供商名称的条目会被丢弃并报告，列表其余部分仍然强制执行。如果列表为空，或其中每个条目都无法识别，Claude Code 会拒绝所有提供商，并且不会在该机器上启动。

#### 需要在托管 `env` 中固定的端点

固定值是在托管 [`env`](#env) 块中设置的端点变量值。当会话将某提供商的流量发送到该提供商自有服务以外的地方时，只有当会话的值与固定值相同时，Claude Code 才会接受。以下端点需要固定值：

* **`"customEndpoint"` 会话**：指定主机的变量，例如 `ANTHROPIC_BASE_URL`
* **Amazon Bedrock**：AWS SDK 的 `AWS_ENDPOINT_URL`、`AWS_ENDPOINT_URL_BEDROCK` 和 `AWS_ENDPOINT_URL_BEDROCK_RUNTIME` 变量指向 Bedrock 自有服务之外时。会话仍归于 `"bedrock"` 而非 `"customEndpoint"`
* **网关登录的 URL**：会话仍归于 `"gateway"`，并且 [`forceLoginGatewayUrl`](#forcelogingatewayurl) 也算作固定值

哪些 `env` 块算作固定值取决于列表的设置位置：

* **机器上的管理员来源设置了列表**：只有机器自身管理员来源的 `env` 块才算数
* **仅服务器托管设置设置了列表**：这些服务器托管设置中的 `env` 值也算数

该列表不会评判云提供商的凭据和租户变量或网络路径，例如 `HTTPS_PROXY` 和证书设置。请在托管 `env` 块中为整个机群设置这些项。

### `apiKeyHelper`

运行您自己的命令来生成 Claude Code 随模型请求发送的凭据。Claude Code 通过系统 shell 运行该命令（macOS 和 Linux 上为 `/bin/sh`，Windows 上为 `cmd`），并将其输出同时作为 `X-Api-Key` 和 `Authorization: Bearer` 标头发送。可将其用于动态或轮换的凭据，例如从保管库获取的短期令牌。

* **作用域**：[`Any file`](#scopes)
* **类型**：字符串，一条 shell 命令行
* **默认值**：未设置，因此 Claude Code 不运行辅助程序

```json settings.json theme={null}
{
  "apiKeyHelper": "/bin/generate_temp_api_key.sh"
}
```

Claude Code 会缓存该值，并在以下情况下重新运行命令：

* 缓存有效期过后，默认为五分钟，或为您通过 [`CLAUDE_CODE_API_KEY_HELPER_TTL_MS`](https://code.claude.com/docs/zh-CN/env-vars) 设置的间隔。
* 当直接或通过 [LLM 网关](https://code.claude.com/docs/zh-CN/llm-gateway)发往 Anthropic API 的请求以 `401` 或 `403` 失败时。
* 在直接或通过 LLM 网关向 Anthropic API 发送请求之前，如果缓存的输出是一个在辅助程序生成后已过期的 JWT。需要 Claude Code v2.1.246 或更高版本。

后两种情况仅在辅助程序的输出就是 Claude Code 发送的凭据且未设置 `ANTHROPIC_AUTH_TOKEN` 时适用。

在交互式会话中，如果该命令来自项目设置或本地设置，Claude Code 会在您接受工作区信任提示后才运行它。请参阅[凭据管理](https://code.claude.com/docs/zh-CN/authentication.md#credential-management)。

### `awsAuthRefresh`

当 Claude Code 用于 [Amazon Bedrock](https://code.claude.com/docs/zh-CN/amazon-bedrock.md) 的凭据失效时，运行您自己的命令（例如 `aws sso login`）来刷新 `.aws` 目录中的凭据。Claude Code 会先通过 STS 检查当前凭据，仅在检查失败时运行该命令，然后读取刷新后的 `.aws` 目录。

当多个使用相同命令和凭据的 Claude Code 进程（例如不同的终端或 IDE 窗口）同时检查失败时，由一个进程运行该命令，其余进程等待这次运行，而不是各自启动。一个有请求挂起且已等待 60 秒的进程会自行运行该命令。要关闭此行为，请将 [`CLAUDE_CODE_DISABLE_AUTH_REFRESH_LOCK`](https://code.claude.com/docs/zh-CN/env-vars) 设置为 `1`。

* **作用域**：[`Any file`](#scopes)
* **类型**：字符串，一条 shell 命令行
* **默认值**：未设置，因此 Claude Code 不会为您刷新 AWS 凭据

```json settings.json theme={null}
{
  "awsAuthRefresh": "aws sso login --profile myprofile"
}
```

当您的刷新流程写入 `.aws` 时使用此设置项；当它改为打印凭据时，请使用 [`awsCredentialExport`](#awscredentialexport)。请参阅[高级凭据配置](https://code.claude.com/docs/zh-CN/amazon-bedrock.md#advanced-credential-configuration)。

### `awsCredentialExport`

运行您自己的命令，以 JSON 形式打印 AWS 凭据，使 Claude Code 能够使用不在 `.aws` 目录中的凭据调用 [Amazon Bedrock](https://code.claude.com/docs/zh-CN/amazon-bedrock.md)。Claude Code 接受 `aws sts` 的输出格式和扁平的 `aws configure export-credentials` 格式，并将凭据限定于其自身的 Bedrock 客户端，因此 Claude 运行的 shell 命令仍然看到您的环境凭据。

* **作用域**：[`Any file`](#scopes)
* **类型**：字符串，一条 shell 命令行
* **默认值**：未设置，因此 Claude Code 使用环境中的 AWS 凭据链

```json settings.json theme={null}
{
  "awsCredentialExport": "/bin/generate_aws_grant.sh"
}
```

与 [`awsAuthRefresh`](#awsauthrefresh) 不同，只要设置了此命令，Claude Code 就会始终运行它，而不会先检查环境凭据。请参阅[高级凭据配置](https://code.claude.com/docs/zh-CN/amazon-bedrock.md#advanced-credential-configuration)。

### `forceLoginMethod`

限制用户可以使用哪种账户登录。设置为 `"claudeai"` 仅允许 claude.ai 账户，设置为 `"console"` 仅允许 Claude Console 账户，设置为 `"gateway"` 则将用户引导至 [cloud gateway](https://code.claude.com/docs/zh-CN/claude-apps-gateway)，而不是第一方登录。管理员在托管设置中设置此项，并与 [`forceLoginOrgUUID`](#forceloginorguuid) 搭配使用，使开发者的 claude.ai 登录限定在一个组织内。

如果您在任何设置文件中将其设置为 `"claudeai"`，Claude Code 还会在该文件适用的会话中停止提供[无密钥 Console 登录](https://code.claude.com/docs/zh-CN/authentication.md#sign-in-without-an-api-key)。仅设置 `"console"` 时，用户在登录失败后仍可访问无密钥登录。要让 Claude Code 停止提供该登录方式，请参阅[在托管机器上停止提供无密钥登录](https://code.claude.com/docs/zh-CN/authentication.md#stop-offering-the-keyless-sign-in-on-managed-machines)，其中还列出了 `forceLoginOrgUUID` 带来的其他变化。

* **作用域**：[`Any file`](#scopes)。Claude Code 仅在与 [`forceLoginGatewayUrl`](#forcelogingatewayurl) 相同的来源中认可 `"gateway"`，在其他任何地方都将其视为未设置。
* **类型**：字符串，以下之一：
  * `"claudeai"`：只有 claude.ai 账户可以登录
  * `"console"`：只有 Claude Console 账户可以登录
  * `"gateway"`：Claude Code 将用户引导至 cloud gateway，而不是第一方登录
* **默认值**：未设置，因此用户自行选择登录方式

```json settings.json theme={null}
{
  "forceLoginMethod": "claudeai"
}
```

每个第一方登录路径都会应用此限制，包括 [VS Code 扩展](https://code.claude.com/docs/zh-CN/vs-code.md)、Agent SDK、`claude setup-token` 和 `/install-github-app`，但终端的交互式登录界面（通过 `/login` 或首次运行引导进入）除外，该界面会预先选择登录方式但不强制执行。在 v2.1.212 之前，只有终端登录会应用此限制。有关各登录路径、环境凭据和第三方提供商的处理方式，请参阅[将登录限制在您的组织内](https://code.claude.com/docs/zh-CN/authentication.md#restrict-login-to-your-organization)。

当机器上的托管来源设置了 `"gateway"` 时，Claude Code 不会使用遗留的登录、API 密钥或 `apiKeyHelper` 凭据。有关每种情况产生的消息，请参阅 [Administrator policy requires a Cloud gateway sign-in](https://code.claude.com/docs/zh-CN/errors.md#administrator-policy-requires-a-cloud-gateway-sign-in)。如果您通过 `CLAUDE_CODE_USE_BEDROCK` 或类似的环境变量选择了云提供商，则该会话不需要网关登录。在 v2.1.261 之前，Claude Code 会在这些机器上使用遗留的登录。

### `forceLoginGatewayUrl`

设置 `/login` Cloud gateway 界面连接的网关 URL，使用户无需输入地址即可访问您的 [cloud gateway](https://code.claude.com/docs/zh-CN/claude-apps-gateway)。该界面没有 URL 输入框：设置此项后，界面会显示您的网关 URL，并在用户按下 Enter 时连接；未设置时，界面会提示用户联系其 IT 管理员。

在托管设置中，此项或 `forceLoginMethod: "gateway"` 任一项都会使机器变为仅限网关，但使用 `CLAUDE_CODE_USE_*` 选择云提供商的会话除外。此时 `/login` 会直接打开 Cloud gateway 界面，不显示登录方式选择器。有关遗留的第一方登录或 API 密钥会如何处理，请参阅 [Administrator policy requires a Cloud gateway sign-in](https://code.claude.com/docs/zh-CN/errors.md#administrator-policy-requires-a-cloud-gateway-sign-in)。请同时设置这两项，使界面能够连接而不是显示错误。

* **作用域**：[`User or managed`](#scopes)。从机器上的托管来源读取：`managed-settings.json`、macOS plist 或 Windows HKLM 注册表，或策略辅助程序。在没有上述任何来源的机器上，Claude Code v2.1.295 或更高版本还会从[用户设置](https://code.claude.com/docs/zh-CN/claude-apps-gateway#set-the-gateway-url-in-user-settings)中读取。Claude Code 会忽略 HKCU 和服务器托管设置中的此项。
* **类型**：字符串，包含协议的完整 URL
* **默认值**：未设置，因此 Cloud gateway 界面会显示错误，提示用户联系其 IT 管理员

```json managed-settings.json theme={null}
{
  "forceLoginGatewayUrl": "https://claude-gateway.example.com"
}
```

如果该值不是有效的 URL，登录界面会报告此问题，托管设置文件的其余部分仍然适用。请参阅[设置网关 URL](https://code.claude.com/docs/zh-CN/claude-apps-gateway#set-the-gateway-url)。

### `forceLoginOrgUUID`

在托管来源中，要求 claude.ai 账户登录必须属于某一个 Anthropic 组织（以单个 UUID 给出），或属于多个组织中的任意一个（以数组给出）。在任何设置文件中，Claude Code 还会使用单个 UUID 在 claude.ai 或 Claude Console 登录期间预先选择该组织；对于数组则不预先选择任何组织。如果您在任何设置文件中设置了此项，Claude Code 还会在该文件适用的会话中停止提供[无密钥 Console 登录](https://code.claude.com/docs/zh-CN/authentication.md#sign-in-without-an-api-key)，并改为创建 API 密钥。

* **作用域**：[`Any file`](#scopes)。只有托管来源会强制执行该限制；其他任何设置文件中的单个 UUID 只会在登录期间预先选择该组织，而不进行限制。
* **类型**：字符串（一个 UUID），或字符串数组（多个 UUID）
* **默认值**：未设置，因此任何组织都可以登录

此示例接受来自两个组织中任一组织的登录，且不预先选择其中之一：

```json managed-settings.json theme={null}
{
  "forceLoginOrgUUID": ["xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx", "yyyyyyyy-yyyy-yyyy-yyyy-yyyyyyyyyyyy"]
}
```

如果托管来源设置了空数组，或者值既不是字符串也不是字符串数组，则使用 Anthropic 账户登录的用户将无法启动 Claude Code 或完成登录。他们会看到一条指出 `forceLoginOrgUUID` 并提示其联系管理员的消息。而输出错误类型值的 [`policyHelper`](#policyhelper) 则会[运行失败](#helper-failures)。

有关 Claude Code 如何处理 Claude Console 登录、其他登录路径和环境凭据，请参阅[将登录限制在您的组织内](https://code.claude.com/docs/zh-CN/authentication.md#restrict-login-to-your-organization)。

### `gatewayInternalNetworks`

声明您的组织用于内部网络编址的公共 IPv4 地址块，使 `/login` 接受位于其中的 [cloud gateway](https://code.claude.com/docs/zh-CN/claude-apps-gateway)。需要 Claude Code v2.1.268 或更高版本。

未设置此项时，`/login` 只连接位于私有地址上的网关。设置后，`/login` 还会接受位于所列地址块内的网关，但仅限直接连接。该连接上机器自身的地址也必须位于同一地址块内。

* **作用域**：[`Managed`](#scopes)。仅从机器上的来源读取：`managed-settings.json`、macOS plist 或 Windows HKLM 注册表，或策略辅助程序。Claude Code 会忽略 HKCU 和服务器托管设置中的此项。
* **类型**：字符串数组，最多四个 IPv4 CIDR 地址块，每个范围为 `/8` 到 `/32`，彼此不重叠，且均不与私有地址空间重叠。
* **默认值**：未设置，因此 `/login` 只接受位于私有地址上的网关

```json managed-settings.json theme={null}
{
  "gatewayInternalNetworks": ["203.0.113.0/24"]
}
```

请将示例中的文档用地址范围替换为您自己的地址块。Claude Code 会拒绝文档用地址范围、VPN 和 NAT64 客户端在本地使用的地址范围，以及没有任何网络使用的保留地址空间，例如组播。

如果某个条目无效，或者值不是字符串列表，`/login` 会指出问题，并在您修正该值之前拒绝该机器上所有新的网关登录。现有登录继续有效。有关完整规则以及开发者会看到的内容，请参阅[允许位于您拥有的公共地址空间上的网关](https://code.claude.com/docs/zh-CN/claude-apps-gateway#allow-a-gateway-on-public-address-space-you-own)。

### `gcpAuthRefresh`

当 Claude Code 发现 Google Cloud 应用默认凭据已过期或无法加载时，运行您自己的命令来刷新它们，使 [Google Cloud 的 Agent Platform](https://code.claude.com/docs/zh-CN/google-vertex-ai.md) 请求持续有效，而无需您手动重新进行身份验证。

当多个使用相同命令和凭据的 Claude Code 进程（例如不同的终端或 IDE 窗口）同时发现凭据过期时，由一个进程运行该命令，其余进程等待这次运行，而不是各自启动。一个有请求挂起且已等待 60 秒的进程会自行运行该命令。要关闭此行为，请将 [`CLAUDE_CODE_DISABLE_AUTH_REFRESH_LOCK`](https://code.claude.com/docs/zh-CN/env-vars) 设置为 `1`。

* **作用域**：[`Any file`](#scopes)
* **类型**：字符串，一条 shell 命令行
* **默认值**：未设置，因此 Claude Code 的凭据错误会提示您自行运行 `gcloud auth application-default login`

```json settings.json theme={null}
{
  "gcpAuthRefresh": "gcloud auth application-default login"
}
```

请参阅[高级凭据配置](https://code.claude.com/docs/zh-CN/google-vertex-ai.md#advanced-credential-configuration)。

### `otelHeadersHelper`

运行您自己的命令来生成 Claude Code 随 OpenTelemetry 导出发送的标头，适用于令牌会轮换的后端。Claude Code 会在启动时运行它，之后定期运行，并期望在 stdout 上得到一个由字符串标头值组成的 JSON 对象。

* **作用域**：[`Any file`](#scopes)
* **类型**：字符串，可执行文件路径或一条 shell 命令行
* **默认值**：未设置，因此 Claude Code 不添加由辅助程序生成的标头

```json settings.json theme={null}
{
  "otelHeadersHelper": "/bin/generate_otel_headers.sh"
}
```

使用 [`CLAUDE_CODE_OTEL_HEADERS_HELPER_DEBOUNCE_MS`](https://code.claude.com/docs/zh-CN/env-vars) 设置刷新间隔。有关脚本要求以及辅助程序失败时的行为，请参阅[动态标头](https://code.claude.com/docs/zh-CN/monitoring-usage.md#dynamic-headers)。

## 更新与版本控制

选择更新渠道，并且对于组织而言，可以固定成员能够运行的版本。请参阅[更新 Claude Code](https://code.claude.com/docs/zh-CN/setup.md#update-claude-code)。

### `autoUpdatesChannel`

选择后台自动更新和 `claude update` 所遵循的[发布渠道](https://code.claude.com/docs/zh-CN/setup.md#configure-release-channel)。设置为 `"stable"` 将使用通常约一周前发布、并跳过存在重大回归问题的版本的版本；设置为 `"latest"` 则使用最新发布的版本。

* **作用域**：[`Any file`](#scopes)。在托管设置中设置此项，可在整个组织内强制使用同一渠道。
* **类型**：字符串，取值为以下之一：
  * `"latest"`：更新遵循最新发布的版本
  * `"stable"`：更新遵循通常约一周前发布、并跳过存在重大回归问题的版本的版本
* **默认值**：未设置，因此 Claude Code 遵循 `"latest"`

```json settings.json theme={null}
{
  "autoUpdatesChannel": "stable"
}
```

当您在 `/config` 中的 **Auto-update channel** 下选择 stable 时，Claude Code 会将 `"stable"` 写入您的用户设置；当您在该处切换回 latest 时，则会删除此键。`claude install stable` 和 `claude install latest` 也会保存您指定的渠道。在 `/config` 中从 `"latest"` 切换到 `"stable"` 时，系统会询问是允许降级还是保留当前版本；选择保留当前版本会设置 [`minimumVersion`](#minimumversion)。通过 Homebrew 安装时会忽略此键：`claude-code` cask 跟踪 stable，`claude-code@latest` 跟踪 latest，且 `claude update` 会交由 `brew upgrade` 处理。如需完全关闭自动更新，请在 `env` 中设置 [`DISABLE_AUTOUPDATER`](https://code.claude.com/docs/zh-CN/setup#disable-auto-updates)。

### `minimumVersion`

阻止后台自动更新和 `claude update` 安装低于此版本的任何版本，这样切换到 `"stable"` 渠道时不会使您从较新的 `"latest"` 构建版本降级。当您在 `/config` 中切换渠道并选择保留当前版本时，Claude Code 会自动为您写入此键；当您切换回 `"latest"` 时则会清除它。

* **作用域**：[`Any file`](#scopes)。在托管设置中设置此项，可固定一个用户设置和项目设置无法调低的组织级最低版本。
* **类型**：字符串，即版本号，例如 `"2.1.100"`；无效的版本值将被忽略
* **默认值**：未设置，因此更新可以安装该渠道提供的任何版本

以下示例遵循 stable 渠道，并拒绝安装低于 2.1.100 的任何版本：

```json settings.json theme={null}
{
  "autoUpdatesChannel": "stable",
  "minimumVersion": "2.1.100"
}
```

此键仅约束更新。如需让 Claude Code 在低于某个版本时拒绝启动，请改用 [`requiredMinimumVersion`](#requiredminimumversion)。请参阅[固定最低版本](https://code.claude.com/docs/zh-CN/setup.md#pin-a-minimum-version)。

### `requiredMaximumVersion`

设置您的组织允许启动的最新 Claude Code 版本。当正在运行的版本高于此版本时，Claude Code 会在启动时退出，并提示用户通过组织批准的方式安装获准的版本；`claude install <version>` 也可能可行。需要 Claude Code v2.1.163 或更高版本。

* **作用域**：[`Managed`](#scopes)。在其他位置设置此键时，Claude Code 会忽略它且不会给出任何警告。
* **类型**：字符串，即版本号，例如 `"2.1.150"`；无效的版本值将被忽略
* **默认值**：未设置，因此不设上限

```json managed-settings.json theme={null}
{
  "requiredMaximumVersion": "2.1.150"
}
```

后台自动更新和 `claude update` 会跳过高于上限的版本，因此处于允许范围内的安装会始终保持在该范围内。即使版本高于上限，`claude update`、`claude install` 和 `claude doctor` 仍可正常使用，以便用户进行恢复。将其与 [`requiredMinimumVersion`](#requiredminimumversion) 搭配使用，可强制限定版本范围。

### `requiredMinimumVersion`

设置您的组织允许启动的最旧 Claude Code 版本。当正在运行的版本低于此版本时，Claude Code 会在启动时退出，并提示用户通过组织批准的方式进行更新。此检查仅在启动时执行，因此已在运行的会话会继续运行。需要 Claude Code v2.1.163 或更高版本。

* **作用域**：[`Managed`](#scopes)。在其他位置设置此键时，Claude Code 会忽略它且不会给出任何警告。
* **类型**：字符串，即版本号，例如 `"2.1.150"`；无效的版本值将被忽略
* **默认值**：未设置，因此不设下限

```json managed-settings.json theme={null}
{
  "requiredMinimumVersion": "2.1.150"
}
```

即使版本低于下限，`claude update`、`claude install` 和 `claude doctor` 仍可正常使用，以便用户进行恢复。[`minimumVersion`](#minimumversion) 仅用于防止降级，而此键则会阻止启动。将其与 [`requiredMaximumVersion`](#requiredmaximumversion) 搭配使用，可强制限定版本范围。

## 工具

在 [Claude Code 桌面应用](https://code.claude.com/docs/zh-CN/desktop.md)中关闭特定工具。终端 CLI 会忽略这些键。有关工具本身，请参阅 [Claude 可用的工具](https://code.claude.com/docs/zh-CN/tools-reference.md)。

### `browserExternalPageTools`

阻止 Claude 使用其工具读取或操作桌面应用 [Browser 窗格](https://code.claude.com/docs/zh-CN/desktop.md#browse-external-sites)中的外部页面。您组织中的人员仍可自行打开外部网站，本地开发服务器预览也仍可配合 Claude 的工具正常使用。桌面应用会读取此键；终端 CLI 会忽略它。

* **作用域**：[`Managed`](#scopes)
* **类型**：字符串，`"disabled"`；桌面应用也接受 `"disable"`，大小写均可
* **默认值**：未设置，因此 Claude 的工具可在外部页面上使用

```json managed-settings.json theme={null}
{
  "browserExternalPageTools": "disabled"
}
```

任何其他值都会使 Claude 的工具保持开启，并且不属于这两个可接受值的非空字符串会记录一条警告。若要同时对人员和 Claude 屏蔽外部网站，请改为设置 [`disableBrowserExternalNavigation`](#disablebrowserexternalnavigation)。请参阅[为您的组织限制外部浏览](https://code.claude.com/docs/zh-CN/desktop.md#restrict-external-browsing-for-your-organization)。

### `disableBrowserExternalNavigation`

在桌面应用的 [Browser 窗格](https://code.claude.com/docs/zh-CN/desktop.md#browse-external-sites)中同时为人员和 Claude 关闭外部浏览。Localhost 开发服务器预览仍可正常使用。桌面应用会读取此键；终端 CLI 会忽略它。

* **作用域**：[`Managed`](#scopes)
* **类型**：布尔值；仅 JSON 布尔值 `true` 生效
  * `true`：桌面应用在 Browser 窗格中同时为人员和 Claude 关闭外部浏览；localhost 预览仍可正常使用
  * `false`：外部浏览保持开启
* **默认值**：未设置，因此外部浏览处于开启状态

```json managed-settings.json theme={null}
{
  "disableBrowserExternalNavigation": true
}
```

桌面应用会忽略任何其他值，并且非布尔值（例如字符串 `"true"` 或 `1`）还会记录一条警告。若要保持外部浏览开启但禁止 Claude 的工具作用于外部页面，请改为设置 [`browserExternalPageTools`](#browserexternalpagetools)。请参阅[为您的组织限制外部浏览](https://code.claude.com/docs/zh-CN/desktop.md#restrict-external-browsing-for-your-organization)。

### `disableMobileSimulatorTools`

阻止 Claude 的工具访问桌面应用的 [iOS Simulator 窗格](https://code.claude.com/docs/zh-CN/desktop-ios-simulator.md#turn-off-simulator-access)。人员仍可手动使用该窗格；仅移除 Claude 的访问权限，且任何人都无法在应用内重新开启。桌面应用会读取此键；终端 CLI 会忽略它。

* **作用域**：[`Managed`](#scopes)
* **类型**：布尔值；仅 JSON 布尔值 `true` 生效
  * `true`：桌面应用阻止 Claude 的工具访问 iOS Simulator 窗格
  * `false`：Claude 的模拟器工具遵循每个人在桌面应用中的设置开关
* **默认值**：未设置，因此 Claude 的模拟器工具遵循每个人在桌面应用中的设置开关

```json managed-settings.json theme={null}
{
  "disableMobileSimulatorTools": true
}
```

桌面应用会忽略任何其他值，并且非布尔值（例如字符串 `"true"` 或 `1`）还会记录一条警告。

## 隐私与遥测

控制 Claude Code 保留会话数据的时长以及它发送的内容。关闭使用指标和错误报告的开关是环境变量，而不是设置键：在 [`env`](#env) 键或 shell 中设置 `DISABLE_TELEMETRY`、`DISABLE_ERROR_REPORTING` 或 `CLAUDE_CODE_DISABLE_NONESSENTIAL_TRAFFIC`。[遥测服务](https://code.claude.com/docs/zh-CN/data-usage.md#telemetry-services)说明了每个变量会停止哪些内容。有两个例外可以通过设置文件关闭：下文的 [`feedbackDrafts`](#feedbackdrafts) 用于 Claude 起草的反馈，下文的 [`feedbackSurveyRate`](#feedbacksurveyrate) 用于会话调查。

### `cleanupPeriodDays`

设置 Claude Code 在删除[会话记录和其他应用程序数据](https://code.claude.com/docs/zh-CN/claude-directory.md#cleaned-up-automatically)之前保留它们的天数。只要 Claude Code 能够安全地确定保留期限，它就会在会话启动后以后台清理的方式执行删除。清理在删除会话记录时不会显示任何消息，因此超过保留期限未使用的会话将不再出现在 [`/resume`](https://code.claude.com/docs/zh-CN/sessions#resume-a-session) 选择器中。

* **作用域**：[`任意文件`](#scopes)
* **类型**：天数，整数，最小值为 `1`
* **默认值**：`30`

```json settings.json theme={null}
{
  "cleanupPeriodDays": 20
}
```

设置为 `0` 无法通过验证，因此如需长期保留，请选择较大的值，例如 `3650`。若要让 Claude Code 完全不写入会话记录，请参阅[明文存储](https://code.claude.com/docs/zh-CN/claude-directory.md#plaintext-storage)。

### `desktopSessionCleanupPeriodDays`

为您在 Claude Desktop 或 Cowork 中启动或最近继续的会话的会话记录设置以天为单位的期限。如果未设置此键，Claude Code 会[无论时长都保留这些会话记录](https://code.claude.com/docs/zh-CN/claude-directory.md#cleaned-up-automatically)。当每条会话记录的时长同时超过此期限和 [`cleanupPeriodDays`](#cleanupperioddays) 时，Claude Code 会将其删除，因此在 `cleanupPeriodDays` 为默认值 30 的情况下，值 `7` 仍会保留它们 30 天。[自动清理](https://code.claude.com/docs/zh-CN/claude-directory.md#cleaned-up-automatically)列出了改用 `cleanupPeriodDays` 且 Claude Code 忽略此键的情况。需要 Claude Code v2.1.248 或更高版本。

* **作用域**：[`用户或托管`](#scopes)。Claude Code 也会从您通过 `--settings` 传入的文件中读取此键，并在项目设置和本地设置中忽略它。
* **类型**：天数，整数，最小值为 `0`
* **默认值**：`0`，表示不设置期限

```json settings.json theme={null}
{
  "desktopSessionCleanupPeriodDays": 90
}
```

### `feedbackDrafts`

控制 [Claude 起草的反馈](https://code.claude.com/docs/zh-CN/tools-reference.md#sendfeedback-tool-behavior)：Claude 是否可以将反馈草稿加入队列供您审阅，以及 Claude 将草稿加入队列时 Claude Code 是否显示卡片。

* **作用域**：[`用户或托管`](#scopes)
* **类型**：字符串，取值为 `"notify"`、`"quiet"` 或 `"off"` 之一
  * `"notify"`：Claude 将草稿加入队列时，Claude Code 在输入框上方显示一张卡片，默认情况下[每个会话最多三张卡片](https://code.claude.com/docs/zh-CN/tools-reference.md#what-you-see-when-claude-drafts)
  * `"quiet"`：Claude 起草时不显示卡片。您可以在输入框底部看到已加入队列的草稿数量，并在 `/feedback` 中审阅它们
  * `"off"`：Claude Code 移除 SendFeedback 工具，因此 Claude 无法将草稿加入队列
* **默认值**：`"notify"`
* **单会话覆盖**：将 [`CLAUDE_CODE_SEND_FEEDBACK`](https://code.claude.com/docs/zh-CN/env-vars) 设置为 `0` 可在单个会话中关闭此功能

```json settings.json theme={null}
{
  "feedbackDrafts": "quiet"
}
```

在 `/config` 中显示为 **Claude-drafted feedback**，它会将此键写入您的用户设置。只有在 [Claude 可以起草反馈的会话](https://code.claude.com/docs/zh-CN/tools-reference.md#sessions-without-claude-drafted-feedback)中，您才会看到此 `/config` 行；设置为 `"off"` 不会隐藏该行，因此您可以从同一行重新启用此功能。托管设置中的值优先于您的用户设置，因此当管理员设置了此键时，该行会显示托管值，更改它不会产生任何效果。Claude Code 在项目设置和本地设置中会忽略此键。

### `feedbackSurveyRate`

设置当会话符合条件时，[会话质量调查](https://code.claude.com/docs/zh-CN/data-usage.md#session-quality-surveys)出现的概率。设置为 `0` 可阻止调查出现。

* **作用域**：[`任意文件`](#scopes)
* **类型**：介于 `0` 和 `1` 之间的数字
* **默认值**：未设置，因此 Claude Code 使用 Anthropic 远程设置的比率；在不接收远程配置的 Amazon Bedrock、Google Cloud 的 Agent Platform 和 Microsoft Foundry 上，则使用其内置比率 `0.005`
* **单会话覆盖**：将 [`CLAUDE_CODE_DISABLE_FEEDBACK_SURVEY`](https://code.claude.com/docs/zh-CN/env-vars) 设置为 `1` 可在单个会话中关闭调查，无论此键设置的比率是多少

```json settings.json theme={null}
{
  "feedbackSurveyRate": 0.05
}
```

相同的比率也适用于 VS Code 扩展中的调查。

### `skipWebFetchPreflight`

跳过 [WebFetch 域名安全检查](https://code.claude.com/docs/zh-CN/data-usage.md#webfetch-domain-safety-check)，该检查会在获取之前将每个请求的主机名发送到 `api.anthropic.com`。在阻止访问 Anthropic 流量的环境中设置为 `true`，例如出站流量受限的 Amazon Bedrock、Google Cloud 的 Agent Platform 或 Microsoft Foundry 部署。

* **作用域**：[`任意文件`](#scopes)
* **类型**：布尔值
  * `true`：Claude Code 跳过 WebFetch 域名安全检查
  * `false`：在会话中首次获取每个主机名之前运行检查，对于先前检查被阻止或失败的主机名会再次运行
* **默认值**：未设置，因此在会话中首次获取每个主机名之前运行检查

```json settings.json theme={null}
{
  "skipWebFetchPreflight": true
}
```

跳过检查后，WebFetch 会尝试访问任何 URL 而不查询阻止列表，因此如果您需要限制 Claude 可以访问的域名，请将其与 [`WebFetch` 权限规则](https://code.claude.com/docs/zh-CN/permissions#webfetch)搭配使用。

## 企业与托管设置

组织用于计算、刷新和合并托管设置的键。请参阅[设置托管设置](https://code.claude.com/docs/zh-CN/admin-setup.md)。

### `disableSideloadFlags`

在启动时拒绝 `--plugin-dir`、`--plugin-url`、`--agents` 和 `--mcp-config` CLI 标志，否则用户可以通过传递这些标志在单次运行中绕过 [`strictKnownMarketplaces`](#strictknownmarketplaces)。Claude Code 会退出并显示一条列出被拒绝标志的错误。在[云端会话](https://code.claude.com/docs/zh-CN/claude-code-on-the-web.md)中，Claude Code 则会启动会话，并丢弃服务器下发的所有 `--mcp-config` 条目，但进程内 `type: "sdk"` 条目和 [Claude Tag](https://code.claude.com/docs/zh-CN/claude-tag.md) 会话的 Slack 工具除外。需要 Claude Code v2.1.193 或更高版本。

* **作用域**：[`Managed`](#scopes)
* **类型**：Boolean
  * `true`：Claude Code 在启动时拒绝 `--plugin-dir`、`--plugin-url`、`--agents` 和 `--mcp-config`，并退出且显示列出这些标志的错误。在云端会话中，它则会启动会话，并丢弃服务器下发的所有 `--mcp-config` 条目，但进程内 `type: "sdk"` 条目和 Claude Tag 会话的 Slack 工具除外
  * `false`：Claude Code 接受这些标志
* **默认值**：`false`

```json managed-settings.json theme={null}
{
  "disableSideloadFlags": true
}
```

如果 `--mcp-config` 中的服务器全部是进程内 `type: "sdk"` 条目，Claude Code 仍会接受它，因此 Agent SDK 和 VS Code 扩展可以继续正常工作。用户仍然可以通过 `claude mcp add` 或 `.mcp.json` 文件添加服务器；如需按服务器进行控制，请同时设置 [`allowedMcpServers`](https://code.claude.com/docs/zh-CN/managed-mcp)。需要 Claude Code v2.1.193 或更高版本。

`disableSideloadFlags` 还会拒绝来自以下来源的插件文件夹：

* **[`CLAUDE_CODE_PLUGIN_DIRS`](https://code.claude.com/docs/zh-CN/env-vars#variables) 环境变量**：当该变量指定了插件文件夹时，Claude Code 会在启动时退出，并显示一条指出该变量并提示取消设置它的错误。需要 Claude Code v2.1.280 或更高版本
* **Agent SDK [`addPlugins`](https://code.claude.com/docs/zh-CN/agent-sdk/typescript#load-more-plugin-folders-with-addplugins) 选项**：当应用程序在 `addPlugins` 中将插件文件夹传递给 `reloadPlugins()` 时，该调用会失败并报错。会话将继续运行，并保留其已有的插件。需要 Claude Code v2.1.301 或更高版本

在云端会话中，Claude Code 还会忽略服务器在会话中途下发的 MCP 更新，云端会话配置以及到达这些会话的 SDK `setMcpServers()` 调用都是通过这一路径生效的。在这里，进程内 `type: "sdk"` 条目和 Claude Tag 会话的 Slack 工具同样不受影响。在 v2.1.268 之前，这种丢弃和启动时的丢弃也会移除 Claude Tag 会话的 Slack 工具。在 v2.1.239 之前，服务器下发的 `--mcp-config` 会导致云端会话无法启动。

桌面应用会自行管理部分插件，包括从 claude.ai 同步的插件以及您的组织通过该应用部署的插件。如果您通过 MDM、操作系统级策略或托管设置文件将此键部署到设备上，桌面应用不会将这些插件传递给该设备上的以下会话：

* **[用户计算机上的 Code 会话](https://code.claude.com/docs/zh-CN/desktop.md#environment-configuration)**：这些会话在启动时也不会带有用户 claude.ai 账户已启用的 skill。Claude Code 从您托管设置中的市场安装的插件仍会加载。在 [Claude Desktop on 3P](https://claude.com/docs/third-party/claude-desktop/overview) 中，您部署到设备 `org-plugins` 目录的插件所提供的 MCP 服务器也仍然可用，因为桌面应用会自行连接它们。在 Claude Desktop v1.37937.0 之前，这些会话会在启动时失败。
* **[用户计算机上的 Cowork 会话](https://code.claude.com/docs/zh-CN/managed-settings.md#where-and-when-a-policy-applies)**：这些插件中的 skill 以及用户 claude.ai 账户已启用的 skill 仍然可用。在 [Claude Desktop on 3P](https://claude.com/docs/third-party/claude-desktop/overview) 中，您部署到设备 `org-plugins` 目录的插件所提供的 MCP 服务器也仍然可用，因为桌面应用会自行连接它们。在 Claude Desktop v1.44121.0 之前，这些会话会在启动时失败。

### `forceRemoteSettingsRefresh`

阻止 CLI 启动，直到 Claude Code 重新获取了最新的[服务器托管设置](https://code.claude.com/docs/zh-CN/server-managed-settings.md)。如果获取失败，Claude Code 会退出，而不是使用缓存的设置或在没有设置的情况下继续运行。如果您的环境无法接受会话在没有托管策略的情况下运行，哪怕只是短暂的时间窗口，请设置此项。

未设置此键时，Claude Code 不会因获取操作而阻止启动，不过当开发者在启动时登录时，它会最多等待五秒钟以完成获取。Cloud 网关会话始终会等待，如果无法连接到网关则会退出。

* **作用域**：[`Managed`](#scopes)。Claude Code 会遵循任何管理员控制的托管来源中的 `true`，即使该来源不是最高优先级的来源。
* **类型**：Boolean
  * `true`：Claude Code 阻止启动，直到重新获取了最新的服务器托管设置，如果获取失败则退出
  * `false`：Claude Code 不会因获取操作而阻止启动，不过在登录启动时会最多等待五秒钟以完成获取
* **默认值**：`false`

```json managed-settings.json theme={null}
{
  "forceRemoteSettingsRefresh": true
}
```

在 MDM 配置文件或托管设置文件中设置此项，即可在第一个服务器负载到达之前强制执行故障关闭式启动。Claude Code 仅在会获取服务器托管设置的会话中应用此检查，因此[不获取这些设置](https://code.claude.com/docs/zh-CN/server-managed-settings.md#platform-availability)的会话会直接启动而不等待。`claude auth` 子命令不受此限制，因此当获取失败是由凭据过期导致时，用户可以重新进行身份验证。请参阅[强制执行故障关闭式启动](https://code.claude.com/docs/zh-CN/server-managed-settings.md#enforce-fail-closed-startup)。

### `managedSourcesBehavior`

选择 Claude Code 是仅应用您的组织下发的最高优先级[托管来源](https://code.claude.com/docs/zh-CN/managed-settings.md#how-claude-code-combines-managed-sources)，还是合并其下发的所有管理员来源。默认情况下，Claude Code 采用包含[策略键](https://code.claude.com/docs/zh-CN/managed-settings.md#how-claude-code-combines-managed-sources)的最高优先级来源，并忽略其余来源。策略键是指除此键和 `wslInheritsWindowsSettings` 之外的任何设置键。在该默认行为下，一旦服务器托管设置或 MDM 策略下发了策略键，`managed-settings.json` 文件就只会提供 [Claude Code 从每个管理员来源读取的键](https://code.claude.com/docs/zh-CN/managed-settings.md#keys-read-from-every-admin-source)。使用 `"merge"` 时，您下发的每个管理员来源都会将其键贡献到一个合并后的策略中。需要 Claude Code v2.1.242 或更高版本。

仅当[排名](https://code.claude.com/docs/zh-CN/managed-settings.md#how-claude-code-combines-managed-sources)低于您最高来源的每个来源都处于管理员控制之下时，才设置 `"merge"`，因为此时 Claude Code 会将较低来源中的条目（例如 `permissions.allow` 规则）添加到策略中。

* **作用域**：[`Managed`](#scopes)。Claude Code 从包含此键或策略键的最高优先级来源读取此键，并忽略所有排名较低来源中的此键，因此较低的来源无法自行选择与其上方的来源合并。Windows HKCU 注册表和[来自嵌入宿主的父级设置](https://code.claude.com/docs/zh-CN/managed-settings.md#let-an-embedding-host-add-policy)都不参与合并。
* **类型**：字符串，取值之一：
  * `"first-wins"`：包含策略键的最高优先级来源提供策略，较低来源仅提供 [Claude Code 从每个管理员来源读取的键](https://code.claude.com/docs/zh-CN/managed-settings.md#keys-read-from-every-admin-source)
  * `"merge"`：您下发的每个管理员来源都会贡献其键，并按以下规则合并
* **默认值**：`"first-wins"`

请在您部署的最高优先级来源中下发此键。从不接收服务器托管设置的计算机也需要在其 MDM 配置文件中包含此键，因为 Claude Code 从包含此键或策略键的最高优先级来源读取此键。`managed-settings.json` 文件是排名最低的管理员来源，因此在其中设置的 `"merge"` 没有更低的来源可供合并。在服务器托管设置中，此键如下所示：

```json theme={null}
{
  "managedSourcesBehavior": "merge"
}
```

在 `"merge"` 下，Claude Code 按键的类型合并每个键。下表给出了每种类型的规则。"限制类允许列表"、"整体采用的值"和"仅从最高优先级来源读取"这几行列出了其涵盖的所有键，其他行则给出示例：

| 键的类型 | Claude Code 的合并方式 | 键 |
| :- | :- | :- |
| 列表 | 合并所有来源中的条目 | [`permissions.allow`](#permissions-allow)、[`sandbox.network.allowedDomains`](#sandbox-network-alloweddomains) 以及其他列表键 |
| 锁定项 | 应用任一来源设置的最严格值。当没有来源设置严格值时，仅应用最高来源中的较宽松值 | [`allowManagedPermissionRulesOnly`](#allowmanagedpermissionrulesonly)、[`permissions.disableBypassPermissionsMode`](#permissions-disablebypasspermissionsmode) 以及其他布尔或枚举锁定项 |
| 限制类允许列表 | 从设置了该列表的最高来源整体采用该列表，不添加较低来源中的条目。当最高来源未设置时，从下一个来源整体采用 | [`availableModels`](#availablemodels)、[`allowedMcpServers`](#allowedmcpservers)、[`allowedProviders`](#allowedproviders)、[`strictKnownMarketplaces`](#strictknownmarketplaces)、[`allowedChannelPlugins`](#allowedchannelplugins) 以及 [`fallbackModel`](#fallbackmodel) 链 |
| 整体采用的值 | 从设置了该值的最高来源整体采用该值，不合并较低来源中的条目或字段。当最高来源未设置时，从下一个来源整体采用 | [`sandbox.credentials.awsPairs`](#sandbox-credentials-awspairs)、[`sandbox.ripgrep`](#sandbox-ripgrep) |
| 提供的 MCP 服务器 | 合并所有来源中的服务器名称。当两个来源设置了相同的名称时，应用较高来源的完整条目 | [`managedMcpServers`](#managedmcpservers) |
| 仅从最高优先级来源读取 | 仅从包含策略键的最高优先级来源读取该键，因此即使最高来源未设置该键，较低来源的值也会被忽略 | [`apiKeyHelper`](#apikeyhelper)、[`awsAuthRefresh`](#awsauthrefresh)、[`awsCredentialExport`](#awscredentialexport)、[`gcpAuthRefresh`](#gcpauthrefresh)、[`otelHeadersHelper`](#otelheadershelper)、`proxyAuthHelper`、[`parentSettingsBehavior`](#parentsettingsbehavior)、[`modelPicker`](#modelpicker)、[`policyHelper`](#policyhelper)、[`permissions.defaultMode`](#permissions-defaultmode) |
| `env` | 在 `"first-wins"` 和 `"merge"` 下均[按变量跨管理员来源合并](https://code.claude.com/docs/zh-CN/managed-settings.md#keys-read-from-every-admin-source) | [`env`](#env) |
| 其他所有键 | 从设置了该值的最高来源采用该值 | [`cleanupPeriodDays`](#cleanupperioddays)、[`model`](#model) |

整体采用 `sandbox.credentials.awsPairs` 和 `sandbox.ripgrep` 需要 Claude Code v2.1.257 或更高版本。

有几个键附加了表中未显示的条件：

* **[`policyHelper`](#policyhelper)**：仅当包含策略键的最高来源是 MDM 策略或托管设置文件时，Claude Code 才会遵循此键，因此在服务器托管设置下它不生效。
* **[`modelOverrides`](#modeloverrides)**：与 `availableModels` 配对。Claude Code 从设置了 `modelOverrides` 的最高来源采用它，除非某个更高的来源设置了 `availableModels` 但未设置 `modelOverrides`。在这种情况下，它会忽略所有来源中的 `modelOverrides`。
* **[`forceLoginGatewayUrl`](#forcelogingatewayurl)、[`gatewayInternalNetworks`](#gatewayinternalnetworks) 以及 [`forceLoginMethod`](#forceloginmethod) 的 `"gateway"` 值**：Claude Code 从不从服务器托管设置中读取这些项，因此其中的值既不会生效，也不会覆盖 MDM 策略或托管设置文件中设置的值。在计算机上的管理员来源中，仅由包含策略键的排名最高的来源提供这些项，无论是否同时存在服务器托管设置。
* **[`forceLoginOrgUUID`](#forceloginorguuid) 以及 [`forceLoginMethod`](#forceloginmethod) 的 `"claudeai"` 和 `"console"` 值**：请在[哪个托管来源提供登录键](https://code.claude.com/docs/zh-CN/managed-settings.md#which-managed-source-supplies-login-keys)中查找适用的值，而不是在表中查找。
* **[`allowedProviders`](#allowedproviders)**：在应用表中的规则之后，计算机自身的列表仍会限制结果，如其条目的作用域说明所述。

要确认计算机上合并了哪些来源，请运行 `/status` 并[查看 `Setting sources` 行](https://code.claude.com/docs/zh-CN/managed-settings#read-the-source-in-/status)。

### `parentSettingsBehavior`

选择当同时存在管理员部署的托管层级时，Claude Code 是否应用由嵌入宿主进程（例如 Agent SDK 或 IDE 扩展）提供的托管设置。使用 `"first-wins"` 时，Claude Code 会丢弃宿主提供的设置；使用 `"merge"` 时，它会通过仅限限制性值的过滤器，将这些设置应用于管理员层级之下。当宿主需要将自己的限制传递给它启动的会话时（例如 Claude Desktop 下发网关的出站允许列表），请设置 `"merge"`。

* **作用域**：[`Managed`](#scopes)。Claude Code 从最高优先级的管理员控制托管来源读取此项。
* **类型**：字符串，取值之一：
  * `"first-wins"`：当存在管理员部署的托管层级时，Claude Code 丢弃宿主提供的设置
  * `"merge"`：Claude Code 通过仅限限制性值的过滤器，将宿主提供的设置应用于管理员层级之下
* **默认值**：`"first-wins"`

```json managed-settings.json theme={null}
{
  "parentSettingsBehavior": "merge"
}
```

当不存在管理员部署的托管层级时，此键不起作用：此时宿主的设置将作为唯一的托管层级生效，但仍会被过滤为仅保留限制性值。有关过滤器的限制以及各托管来源之间的相互作用，请参阅[来自嵌入宿主的父级设置](https://code.claude.com/docs/zh-CN/managed-settings.md#parent-settings-from-embedding-hosts)和[限制父级设置](https://code.claude.com/docs/zh-CN/claude-apps-gateway#restrict-parent-settings)。

### `policyHelper`

运行您部署的可执行文件，在启动时计算托管设置，这样您就可以根据设备状态、身份或远程服务得出策略，而不必使用静态文件。Claude Code 会在接受第一个提示词之前运行该辅助程序，并将其输出的设置作为会话的托管设置。

* **作用域**：[`Managed`](#scopes)。从 macOS plist、Windows HKLM 注册表或托管设置文件中读取。Claude Code 从包含[策略键](https://code.claude.com/docs/zh-CN/managed-settings.md#how-claude-code-combines-managed-sources)的最高优先级托管来源读取此键，并且仅当该来源是上述三者之一时才运行辅助程序；它会忽略服务器托管设置、HKCU 注册表和宿主提供的父级设置中的此键。
* **类型**：包含 `path`、`timeoutMs` 和 `refreshIntervalMs` 的对象
* **默认值**：未设置，因此不运行任何辅助程序

当服务器托管设置在启动时下发策略时，它们优先于辅助程序所在的来源，辅助程序不会运行。

如果之后的某次设置获取报告服务器托管设置已被移除，Claude Code 会在那时运行辅助程序，而不是等到下次启动。其输出将在会话的剩余时间内生效，如果运行失败，会话将以与[启动运行失败](#helper-failures)相同的消息结束。

此示例以 5 秒超时时间运行辅助程序，并每五分钟重新运行一次：

```json managed-settings.json theme={null}
{
  "policyHelper": {
    "path": "/usr/local/bin/claude-policy",
    "timeoutMs": 5000,
    "refreshIntervalMs": 300000
  }
}
```

#### 编写辅助程序输出

Claude Code 在不带参数的情况下运行辅助程序，在其环境中设置 `CLAUDE_CODE_VERSION`，并从 stdout 读取 JSON 封装对象，上限为 1 MiB。

请将设置放在 `managedSettings` 键下。不带 `managedSettings` 键的裸设置对象在解析时 `managedSettings` 为 undefined，不会应用任何内容，且 Claude Code 不会报告错误：

```json theme={null}
{
  "managedSettings": {
    "permissions": { "deny": ["Read(//etc/secrets/**)"] }
  }
}
```

当辅助程序输出 `managedSettings` 时，该对象将成为本次运行唯一的托管设置来源：Claude Code 会忽略 MDM、文件和 HKCU 来源，仅从辅助程序的输出中读取[跨来源键](https://code.claude.com/docs/zh-CN/managed-settings.md#keys-read-from-every-admin-source)，并且从不合并[父级设置](https://code.claude.com/docs/zh-CN/managed-settings.md#parent-settings-from-embedding-hosts)。

启动时的 `forceRemoteSettingsRefresh` 检查在辅助程序之前运行，并读取任何管理员来源。如果辅助程序以 `0` 退出，但输出的封装对象中省略了 `managedSettings`，则它不提供任何托管设置，其他来源照常生效。

#### 辅助程序失败

在以下情况下，辅助程序运行会失败：

* `path` 违反了 [`policyHelper.path`](#policyhelper-path) 中的规则。
* `path` 处没有常规文件。Claude Code 在启动辅助程序之前检查该文件，且检查计入同一个 `timeoutMs` 时间预算，因此无响应的网络挂载可能导致运行失败。
* 辅助程序以非零状态退出、在 `timeoutMs` 到期时仍在运行，或根本无法启动（例如因为它不可执行）。
* 辅助程序向 stdout 或 stderr 写入超过 1 MiB 的内容。
* stdout 不是单个 JSON 对象，或者其 `managedSettings` 存在 [Claude Code 无法修复的 schema 违规](https://code.claude.com/docs/zh-CN/managed-settings.md#find-entries-claude-code-dropped)。

当启动运行失败时，Claude Code 会打印原因并拒绝启动。在非零退出后，原因中会包含辅助程序的 stderr，如果 stderr 为空则包含其 stdout。在超时后，原因中会指明 `timeoutMs` 限制，但不包含辅助程序的任何输出。这种拒绝适用于交互式会话、`claude -p`、Agent SDK 会话、[后台会话](https://code.claude.com/docs/zh-CN/agent-view.md)以及大多数子命令。

这种拒绝是有意为之的，因此需要具备故障恢复能力的辅助程序应从自己的缓存中提供结果并以 `0` 退出。

当后台刷新失败时，Claude Code 会继续使用上一次成功的策略，并且 `/status` 会显示失败的刷新及其原因，直到刷新成功为止。每次刷新都遵循与启动运行相同的 `timeoutMs` 和失败规则。

使用 `--debug` 时，Claude Code 会将每次运行中辅助程序的 stderr 写入[调试日志](https://code.claude.com/docs/zh-CN/debug-your-config.md)。

Claude Code 会将无效的 `policyHelper` 值报告为[已丢弃的条目](https://code.claude.com/docs/zh-CN/managed-settings.md#find-entries-claude-code-dropped)，并在不运行辅助程序的情况下基于其余托管设置启动会话。无效值包括裸路径字符串以及低于[其最小值](#policyhelper-timeoutms)的 `timeoutMs`。

要关闭辅助程序，请从设置它的来源中移除此键。

### `policyHelper.path`

指定 Claude Code 运行的辅助程序可执行文件。有关路径违反以下规则时会发生什么，请参阅[辅助程序失败](#helper-failures)。

* **作用域**：[`Managed`](#scopes)。从 macOS plist、Windows HKLM 注册表或托管设置文件中读取，即读取 [`policyHelper`](#policyhelper) 的任何位置。
* **类型**：字符串，规范化形式的绝对路径，不含 `.` 或 `..` 段；在 Windows 上，为以 `.exe` 结尾的驱动器号路径或 UNC 路径
* **默认值**：无；设置 `policyHelper` 时为必填项

```json managed-settings.json theme={null}
{
  "policyHelper": {
    "path": "/usr/local/bin/claude-policy"
  }
}
```

### `policyHelper.timeoutMs`

设置 Claude Code 在将运行视为失败之前等待辅助程序的时长。超时的运行与非零退出的失败方式相同，因此在启动时 Claude Code 会拒绝启动。

* **作用域**：[`Managed`](#scopes)。从 macOS plist、Windows HKLM 注册表或托管设置文件中读取，即读取 [`policyHelper`](#policyhelper) 的任何位置。
* **类型**：整数，单位为毫秒，最小值为 `1000`
* **默认值**：`10000`

```json managed-settings.json theme={null}
{
  "policyHelper": {
    "path": "/usr/local/bin/claude-policy",
    "timeoutMs": 5000
  }
}
```

### `policyHelper.refreshIntervalMs`

让 Claude Code 按一定间隔在后台重新运行辅助程序，以便策略更改能够作用于正在运行的会话。刷新成功时，其输出会替换之前的托管设置，无需重启；刷新失败时，Claude Code 会保留现有的策略。

* **作用域**：[`Managed`](#scopes)。从 macOS plist、Windows HKLM 注册表或托管设置文件中读取，即读取 [`policyHelper`](#policyhelper) 的任何位置。
* **类型**：整数，单位为毫秒：`0` 表示禁用刷新，否则至少为 `60000`
* **默认值**：未设置，因此 Claude Code 仅在启动时运行一次辅助程序

此示例每五分钟重新运行一次辅助程序：

```json managed-settings.json theme={null}
{
  "policyHelper": {
    "path": "/usr/local/bin/claude-policy",
    "refreshIntervalMs": 300000
  }
}
```

### `wslInheritsWindowsSettings`

让 WSL 上的 Claude Code 从 Windows 策略链读取托管设置，其中 HKLM 和 Windows 托管设置文件的优先级高于 `/etc/claude-code`，HKCU 排在其下。启用该策略链后，仅当 HKLM 注册表值或 `C:\Program Files\ClaudeCode\` 文件夹中[不存在 Windows 管理员文档](https://code.claude.com/docs/zh-CN/managed-settings.md#present-admin-documents)时，Claude Code 才会读取 `/etc/claude-code`。设置此项可将您已在 Windows 上部署的策略扩展到同一台计算机上的 WSL 会话，使其遵循与宿主会话相同的规则。仅当此项设置在 HKLM 注册表项中，或设置在 `C:\Program Files\ClaudeCode\` 下的托管设置文件或附加文件中时，Claude Code 才会遵循它，而这两者都需要 Windows 管理员权限才能写入。

* **作用域**：[`Managed`](#scopes)。位于管理员控制的 Windows 来源中。
* **类型**：Boolean
  * `true`：WSL 上的 Claude Code 从 Windows 策略链读取托管设置，并且仅在不存在 Windows 管理员文档时读取 `/etc/claude-code`
  * `false`：WSL 仅读取 `/etc/claude-code`
* **默认值**：`false`，因此 WSL 仅读取 `/etc/claude-code`

```json managed-settings.json theme={null}
{
  "wslInheritsWindowsSettings": true
}
```

一旦某个管理员来源启用了该策略链，仅当 HKCU 也将此键设置为 `true` 时，HKCU 策略才会在 WSL 上加入该链。HKCU 中的这一设置本身并不会启用该策略链。仅包含此键（设置为 `true` 或 `false`）的 Windows 来源不算作策略来源，因此较低优先级的来源仍会提供策略。此键对原生 Windows 没有影响。

Claude Code 可读取带引号或不带引号的 `true` 和 `false`，并将 `null` 视为移除此键。包含任何其他值的管理员控制 Windows 来源会被视为[存在的管理员文档](https://code.claude.com/docs/zh-CN/managed-settings.md#present-admin-documents)，且策略链处于启用状态：此时 `/etc/claude-code` 和 HKCU 均不生效，并且会有一条启动警告指出此键。存在但无法读取的 HKLM 值或 Windows 文件夹中的文件，无论策略链是否启用，也都会阻止 `/etc/claude-code` 生效。需要 Claude Code v2.1.282 或更高版本。

## 全局配置设置

请将这些键保存在 `~/.claude.json` 中，而不是设置文件中。Claude Code 会忽略其他位置的这些键。其中大多数键由 Claude Code 和 `/config` 自动写入，您也可以手动编辑。

### `autoConnectIde`

从外部终端启动 Claude Code 时，自动连接到正在运行的 IDE。在 VS Code 或 JetBrains 终端之外运行 Claude Code 时，此项在 `/config` 中显示为 **Auto-connect to IDE (external terminal)**。

* **作用域**：[`Global config`](#scopes)
* **类型**：Boolean
  * `true`：从外部终端启动 Claude Code 时，它会自动连接到正在运行的 IDE
  * `false`：Claude Code 不会从外部终端自动连接；在 VS Code 或 JetBrains 终端内，或使用 `--ide` 时，仍会连接
* **默认值**：`false`
* **单会话覆盖**：[`CLAUDE_CODE_AUTO_CONNECT_IDE`](https://code.claude.com/docs/zh-CN/env-vars) 在单个会话中优先于此键，无论取值方向如何

```json ~/.claude.json theme={null}
{
  "autoConnectIde": true
}
```

Claude Code 会忽略 `settings.json` 中的此键。

### `autoInstallIdeExtension`

从 VS Code 终端运行 Claude Code 时，自动安装 Claude Code IDE 扩展。在 VS Code 或 JetBrains 终端内运行 Claude Code 时，此项在 `/config` 中显示为 **Auto-install IDE extension**。

* **作用域**：[`Global config`](#scopes)
* **类型**：Boolean
  * `true`：从 VS Code 终端运行 Claude Code 时，它会自动安装 IDE 扩展
  * `false`：Claude Code 不会自动安装扩展
* **默认值**：`true`
* **单会话覆盖**：将 [`CLAUDE_CODE_IDE_SKIP_AUTO_INSTALL`](https://code.claude.com/docs/zh-CN/env-vars) 设置为 `1` 可在单个会话中跳过安装，即使此键为 `true`

```json ~/.claude.json theme={null}
{
  "autoInstallIdeExtension": false
}
```

Claude Code 会忽略 `settings.json` 中的此键。

### `claudeInChromeDefaultEnabled`

在每个交互式 CLI 会话启动时开启 [Chrome 集成](https://code.claude.com/docs/zh-CN/chrome.md)，无需每次都传入 `--chrome`。如果您运行 [`claude remote-control`](https://code.claude.com/docs/zh-CN/remote-control)，它为您的某个[项目](https://code.claude.com/docs/zh-CN/claude-projects.md)线程启动的会话也会遵循此键，`bypassPermissions` 模式除外。在 Claude Code v2.1.287 或更高版本中，此键也适用于 [VS Code 扩展](https://code.claude.com/docs/zh-CN/vs-code.md#automate-browser-tasks-with-chrome)中的会话：请参阅[默认启用 Chrome](https://code.claude.com/docs/zh-CN/chrome.md#enable-chrome-by-default)。

运行 `/chrome` 并选择 **Enabled by default** 会自动设置此键，回应 [“Claude in Chrome extension detected” 对话框](https://code.claude.com/docs/zh-CN/chrome.md#answer-the-startup-dialog)也会设置此键。此项在 `/config` 中显示为 **Claude in Chrome enabled by default**。

* **作用域**：[`Global config`](#scopes)
* **类型**：Boolean
  * `true`：交互式 CLI 会话启动时，Claude Code 会开启 Chrome 集成，与传入 `--chrome` 的效果相同。在 VS Code 扩展中，会话会在启动时连接到浏览器
  * `false`：交互式 CLI 会话启动时 Chrome 集成处于关闭状态，并且 Claude Code 不再[主动提出进行设置](https://code.claude.com/docs/zh-CN/chrome.md#install-the-extension-when-claude-asks)。传入 `--chrome` 可为单个交互式会话开启该功能。在 VS Code 扩展中，会话会在您输入 `@browser` 时连接，与未设置此键时相同
* **默认值**：未设置，因此 Chrome 集成处于关闭状态，Claude Code 仍可主动提出进行设置
* **单会话覆盖**：`--chrome` 和 [`--no-chrome`](https://code.claude.com/docs/zh-CN/cli-reference) 在单个交互式会话中优先于此键

```json ~/.claude.json theme={null}
{
  "claudeInChromeDefaultEnabled": true
}
```

Claude Code 会忽略 `settings.json` 中的此键。

### `copyFullResponse`

让 [`/copy`](https://code.claude.com/docs/zh-CN/commands) 每次都复制完整回复，而不显示选择器。在该选择器中选择 **Always copy full response** 会将此键设置为 `true`。此项在 `/config` 中显示为 **Skip the /copy picker**。

* **作用域**：[`Global config`](#scopes)
* **类型**：Boolean
  * `true`：`/copy` 直接复制完整回复，不显示选择器
  * `false`：当回复包含代码块或引用块时，`/copy` 会显示一个选择器，供您选择单个块或完整回复
* **默认值**：`false`

```json ~/.claude.json theme={null}
{
  "copyFullResponse": true
}
```

Claude Code 会忽略 `settings.json` 中的此键。

### `copyOnSelect`

在[全屏渲染](https://code.claude.com/docs/zh-CN/fullscreen.md#use-the-mouse)或 [Agent 视图](https://code.claude.com/docs/zh-CN/agent-view.md)中用鼠标完成文本选择时，自动将文本复制到剪贴板。全屏渲染开启时，此项在 `/config` 中显示为 **Copy on select**。

* **作用域**：[`Global config`](#scopes)
* **类型**：Boolean
  * `true`：完成文本选择时，Claude Code 会将文本复制到剪贴板
  * `false`：选择文本不会改变剪贴板内容，您需要改为[使用快捷键复制所选内容](https://code.claude.com/docs/zh-CN/fullscreen.md#use-the-mouse)
* **默认值**：`true`

```json ~/.claude.json theme={null}
{
  "copyOnSelect": false
}
```

Claude Code 会忽略 `settings.json` 中的此键。

### `defaultToAgentsView`

不带参数运行 `claude` 时，打开 [Agent 视图](https://code.claude.com/docs/zh-CN/agent-view.md)而不是新对话。除非 Agent 视图已被[关闭](#disableagentview)，否则此项在 `/config` 中显示为 **Open agents view by default**。

* **作用域**：[`Global config`](#scopes)
* **类型**：Boolean
  * `true`：不带参数的 `claude` 会打开 Agent 视图，除非 Agent 视图已被[关闭](#disableagentview)
  * `false`：不带参数的 `claude` 会开始新对话
* **默认值**：`false`

```json ~/.claude.json theme={null}
{
  "defaultToAgentsView": true
}
```

Claude Code 会忽略 `settings.json` 中的此键。

### `diffTool`

选择在连接了 [VS Code](https://code.claude.com/docs/zh-CN/vs-code.md) 或 [JetBrains](https://code.claude.com/docs/zh-CN/jetbrains.md#features) IDE 时，Claude Code 在何处显示其提议的 `Edit` 或 `Write` 更改的 diff：`"auto"` 会在 IDE 的 diff 查看器中打开，`"terminal"` 会保留在终端中。仅当 Claude Code 连接到 VS Code 或 JetBrains IDE 时，此项才会在 `/config` 中显示为 **Diff tool**。

* **作用域**：[`Global config`](#scopes)
* **类型**：string，取值为以下之一：
  * `"auto"`：连接了 VS Code 或 JetBrains IDE 时，Claude Code 在 IDE 的 diff 查看器中打开 diff
  * `"terminal"`：Claude Code 将 diff 保留在终端中
* **默认值**：`"auto"`

```json ~/.claude.json theme={null}
{
  "diffTool": "terminal"
}
```

Claude Code 会忽略 `settings.json` 中的此键。

### `externalEditorContext`

按下 `Ctrl+G` 时，Claude Code 会在您的[外部编辑器](https://code.claude.com/docs/zh-CN/interactive-mode.md#general-controls)中打开您正在输入的提示词。开启此键后，编辑器缓冲区开头会以 `#` 注释行的形式包含 Claude 的上一条回复，以便您在编写时阅读，保存时 Claude Code 会删除这些行。此项在 `/config` 中显示为 **Show last response in external editor**。

* **作用域**：[`Global config`](#scopes)
* **类型**：Boolean
  * `true`：编辑器缓冲区开头以 `#` 注释行的形式包含 Claude 的上一条回复，保存时 Claude Code 会删除这些行
  * `false`：编辑器缓冲区打开时仅包含您的提示词
* **默认值**：`false`

```json ~/.claude.json theme={null}
{
  "externalEditorContext": true
}
```

开启后，Claude Code 打开的缓冲区如下所示，只有标记行下方的文本会作为您的提示词发送：

```text theme={null}
# ─── Claude's last response (for reference; removed on save) ───
# I added the retry loop to fetchUser in src/api.ts and a test
# for the timeout case. Want me to wire the same retry into
# fetchOrders?
# ─── Write your reply below this line ──────────────────────────

Yes, and cap it at three attempts.
```

Claude Code 会保留回复的最后 50 行，并用 `# … (earlier output truncated)` 标记截断位置。

Claude Code 会忽略 `settings.json` 中的此键。

### `leftArrowOpensAgents`

在空输入框中按 `←` 可[将会话转到后台并打开 Agent 视图](https://code.claude.com/docs/zh-CN/agent-view.md#switch-sessions-without-leaving-the-terminal)。将此键设置为 `false` 可关闭该快捷键。当 Agent 视图可用时，此项在 `/config` 中显示为 **← opens agents**。

* **作用域**：[`Global config`](#scopes)
* **类型**：Boolean
  * `true`：在终端中启动的会话里，于空输入框中按 `←` 会将该会话转到后台并打开 Agent 视图
  * `false`：Claude Code 关闭该快捷键；在您[从 Agent 视图附加到](https://code.claude.com/docs/zh-CN/agent-view.md#attach-to-a-session)的会话中，于空输入框中按 `←` 仍会分离会话
* **默认值**：`true`

```json ~/.claude.json theme={null}
{
  "leftArrowOpensAgents": false
}
```

Claude Code 会忽略 `settings.json` 中的此键。

### `permissionExplainerEnabled`

> [!WARNING]
>
> 已在 v2.1.257 中移除，同时移除的还有 Bash 和 PowerShell 权限提示中的 `Ctrl+E` 命令解释功能。在当前版本中设置此键不会产生任何效果。

在 v2.1.256 及之前的版本中，您可以在 Bash 或 PowerShell 权限提示中按 `Ctrl+E` 查看模型生成的命令解释，并可将此键设置为 `false` 来关闭该快捷键。

* **作用域**：[`Global config`](#scopes)。适用于 v2.1.256 及更早版本。
* **类型**：Boolean
* **默认值**：`true`

### `prStatusFooterEnabled`

在提示词页脚中显示当前分支已打开的 Pull Request 或 Merge Request 的徽章，并用彩色下划线显示其[状态](https://code.claude.com/docs/zh-CN/interactive-mode.md#pr-review-status)。此项在 `/config` 中显示为 **Show PR status footer**。

* **作用域**：[`Global config`](#scopes)
* **类型**：Boolean
  * `true`：在满足 [PR 审查状态](https://code.claude.com/docs/zh-CN/interactive-mode.md#pr-review-status)中所述条件时，页脚会显示该徽章
  * `false`：Claude Code 跳过页脚的 Pull Request 和 Merge Request 检查，不显示该徽章。您[从 Agent 视图附加到](https://code.claude.com/docs/zh-CN/agent-view.md#attach-to-a-session)的会话仍可显示指向[与其关联](https://code.claude.com/docs/zh-CN/agent-view.md#pull-request-status)的 Pull Request 的普通链接
* **默认值**：`true`

```json ~/.claude.json theme={null}
{
  "prStatusFooterEnabled": false
}
```

Claude Code 会忽略 `settings.json` 中的此键。

### `teammateDefaultModel`

> [!WARNING]
>
> 已在 v2.1.234 中移除，同时移除的还有其 `/config` 行 **Default teammate model**。在当前版本中设置此键不会产生任何效果。

在 v2.1.233 及之前的版本中，您可以将此键设置为 [agent team](https://code.claude.com/docs/zh-CN/agent-teams.md#specify-teammates-and-models) 中提示词未指定模型的队友所使用的模型：可以是 `"sonnet"` 之类的别名，也可以是 `null` 以沿用主导者的模型。关于 Claude Code 现在为此类队友选择的模型，请参阅[指定队友和模型](https://code.claude.com/docs/zh-CN/agent-teams.md#specify-teammates-and-models)。

* **作用域**：[`Global config`](#scopes)。适用于 v2.1.233 及更早版本。
* **类型**：string，模型别名或完整模型 ID，或 `null`
* **默认值**：未设置

## 另请参阅

* [配置权限](https://code.claude.com/docs/zh-CN/permissions.md)：规则语法、权限模式和工作区信任
* [环境变量](https://code.claude.com/docs/zh-CN/env-vars.md)：Claude Code 读取的所有 `CLAUDE_*`、`ANTHROPIC_*` 和提供商变量
* [Claude 可用的工具](https://code.claude.com/docs/zh-CN/tools-reference.md)：内置工具以及哪些工具需要批准
* [设置文件示例](https://code.claude.com/docs/zh-CN/settings-example.md)：个人文件、团队文件和组织的托管文件
* [设置托管设置](https://code.claude.com/docs/zh-CN/admin-setup.md)：组织如何决定要强制执行的内容
* [部署托管设置](https://code.claude.com/docs/zh-CN/managed-settings.md)：交付机制、托管层级内的优先级，以及托管设置中的无效条目
* [调试您的配置](https://code.claude.com/docs/zh-CN/debug-your-config.md)：`claude doctor` 和 Settings Error 对话框
