> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# 探索 .claude 目录

> Claude Code 读取 CLAUDE.md、settings.json、hooks、skills、commands、subagents、workflows、rules 和自动内存的位置。探索项目中的 .claude 目录和主目录中的 ~/.claude。

Claude Code 从您的项目目录和主目录中的 `~/.claude` 读取指令、设置、skills、subagents 和内存。将项目文件提交到 git 以与您的团队共享；`~/.claude` 中的文件是个人配置，适用于您的所有项目。

在 Windows 上，`~/.claude` 解析为 `%USERPROFILE%\.claude`。如果您设置了 [`CLAUDE_CONFIG_DIR`](https://code.claude.com/docs/zh-CN/env-vars)，此页面上的每个 `~/.claude` 路径都将位于该目录下。这同样适用于 `.claude.json`，否则它位于 `~/.claude.json`。

大多数用户只编辑 `CLAUDE.md` 和 `settings.json`。如果您的仓库已经有一个供其他编码 Agent 使用的 `AGENTS.md`，Claude Code [可以读取它](https://code.claude.com/docs/zh-CN/memory.md#agents-md)来代替 `CLAUDE.md`。目录的其余部分是可选的：根据需要添加 skill、规则或子代理。

## 探索目录

单击树中的文件以查看每个文件的作用、何时加载以及示例。

## 未显示的内容

浏览器涵盖您创作和编辑的文件。一些相关文件位于其他位置：

| 文件 | 位置 | 用途 |
| - | - | - |
| `managed-settings.json` | 系统级别，因操作系统而异 | 企业强制执行的设置，您自己的设置文件和 `--settings` 值无法覆盖这些设置，除了[狭窄的例外](https://code.claude.com/docs/zh-CN/settings.md#exceptions-to-managed-settings-precedence)。请参阅[保存文件的位置](https://code.claude.com/docs/zh-CN/managed-settings.md#deploy-a-managed-settings-file)和[Claude Code 使用的托管源](https://code.claude.com/docs/zh-CN/managed-settings.md#precedence-within-the-managed-tier)。 |
| `CLAUDE.local.md` | 项目根目录 | 您对此项目的私人偏好，与 CLAUDE.md 一起加载。手动创建它并将其添加到 `.gitignore`。 |
| `AGENTS.md` | 项目根目录、`.claude/` 或任何目录 | 您为 AI 编码 Agent 编写的项目说明。Claude Code 可以[加载它](https://code.claude.com/docs/zh-CN/memory.md#agents-md)来代替 `CLAUDE.md`。 |
| 已安装的插件 | `~/.claude/plugins` | 克隆的市场、已安装的插件版本、`installed_plugins.json` 安装记录和每个插件的数据，由 `claude plugin` 命令管理。从您的 claude.ai 账户[同步的插件](https://code.claude.com/docs/zh-CN/plugins/loading.md#synced-plugins)下载到 `~/.claude/plugins/synced/`。对于从市场[`command` 源](https://code.claude.com/docs/zh-CN/plugins/marketplace-reference#command-plugin-source)以链接模式安装的插件，Claude Code 在此处存储链接而不是副本，插件的文件保留在命令打印的目录中。`command` 源需要 Claude Code v2.1.229 或更高版本。在您从本地路径添加的市场中按相对路径列出的插件也会[从其源目录就地加载](https://code.claude.com/docs/zh-CN/plugins/loading.md#find-plugins-on-disk)，而不是从缓存副本加载。请参阅[插件缓存](https://code.claude.com/docs/zh-CN/plugins/loading.md#find-plugins-on-disk)了解孤立版本如何被清理。 |

`~/.claude` 还保存 Claude Code 在您工作时写入的数据：记录、提示历史、文件快照、缓存和日志。请参阅下面的[应用数据](#application-data)。

## 选择正确的文件

不同类型的自定义位于不同的文件中。使用此表找到更改应该放在哪里。

| 您想要 | 编辑 | 范围 | 参考 |
| :- | :- | :- | :- |
| 为 Claude 提供项目上下文和约定 | `CLAUDE.md` | 项目或全局 | [Memory](https://code.claude.com/docs/zh-CN/memory.md) |
| 允许或阻止特定工具调用 | `settings.json` `permissions` 或 `hooks` | 项目或全局 | [Permissions](https://code.claude.com/docs/zh-CN/permissions.md)、[Hooks](https://code.claude.com/docs/zh-CN/hooks.md) |
| 在工具调用前后运行脚本 | `settings.json` `hooks` | 项目或全局 | [Hooks](https://code.claude.com/docs/zh-CN/hooks.md) |
| 为会话设置环境变量 | `settings.json` `env` | 项目或全局 | [Settings](https://code.claude.com/docs/zh-CN/settings-reference.md#all-settings) |
| 将个人覆盖保留在 git 之外 | `settings.local.json` | 仅项目 | [Settings scopes](https://code.claude.com/docs/zh-CN/settings.md#where-settings-live) |
| 添加使用 `/name` 调用的提示或功能 | `skills/<name>/SKILL.md` | 项目或全局 | [Skills](https://code.claude.com/docs/zh-CN/skills.md) |
| 定义具有自己工具的专门 subagent | `agents/*.md` | 项目或全局 | [Subagents](https://code.claude.com/docs/zh-CN/sub-agents.md) |
| 通过脚本编排许多 subagent | `workflows/*.js` | 项目或全局 | [Dynamic workflows](https://code.claude.com/docs/zh-CN/workflows.md) |
| 通过 MCP 连接外部工具 | `.mcp.json` | 仅项目 | [MCP](https://code.claude.com/docs/zh-CN/mcp.md) |
| 更改 Claude 格式化响应的方式 | `output-styles/*.md` | 项目或全局 | [Output styles](https://code.claude.com/docs/zh-CN/output-styles.md) |

## 文件参考

此表列出浏览器涵盖的每个文件。项目范围的文件位于您的仓库中的 `.claude/` 下（或 `CLAUDE.md`、`.mcp.json` 和 `.worktreeinclude` 的根目录）。全局范围的文件位于 `~/.claude/` 中，适用于所有项目。

> [!NOTE]
>
> 有几件事可以覆盖您在这些文件中放入的内容：
>
> * 您的组织部署的[托管设置](https://code.claude.com/docs/zh-CN/server-managed-settings.md)优先于所有设置文件和 `--settings` 值，[设置优先级下的例外](https://code.claude.com/docs/zh-CN/settings.md#exceptions-to-managed-settings-precedence)除外
> * CLI 标志（如 `--permission-mode` 或 `--settings`）在该会话中覆盖 `settings.json`
> * 某些环境变量优先于其等效设置，但这会有所不同：检查[环境变量参考](https://code.claude.com/docs/zh-CN/env-vars.md)以了解每个变量
>
> 请参阅[设置优先级](https://code.claude.com/docs/zh-CN/settings.md#settings-precedence)以了解完整顺序。

单击文件名以在上面的浏览器中打开该节点。

| 文件 | 范围 | 提交 | 作用 | 参考 |
| - | - | - | - | - |
| [`CLAUDE.md`](#ce-claude-md) | 项目和全局 | ✓ | 每个会话加载的指令 | [内存](https://code.claude.com/docs/zh-CN/memory.md) |
| [`rules/*.md`](#ce-rules) | 项目和全局 | ✓ | 主题范围的指令，可选择路径门控 | [Rules](https://code.claude.com/docs/zh-CN/memory.md#organize-rules-with-claude/rules/) |
| [`settings.json`](#ce-settings-json) | 项目和全局 | ✓ | 权限、hooks、环境变量、模型默认值 | [设置](https://code.claude.com/docs/zh-CN/settings.md) |
| [`settings.local.json`](#ce-settings-local-json) | 仅项目 | | 您的个人覆盖，当 Claude Code 将设置保存到其中时自动 gitignored | [设置范围](https://code.claude.com/docs/zh-CN/settings.md#where-settings-live) |
| [`.mcp.json`](#ce-mcp-json) | 仅项目 | ✓ | 团队共享的 MCP 服务器 | [MCP 范围](https://code.claude.com/docs/zh-CN/mcp.md#mcp-installation-scopes) |
| [`.worktreeinclude`](#ce-worktreeinclude) | 仅项目 | ✓ | Gitignored 文件以复制到新的 worktrees | [Worktrees](https://code.claude.com/docs/zh-CN/worktrees.md#copy-gitignored-files-into-worktrees) |
| [`skills/<name>/SKILL.md`](#ce-skills) | 项目和全局 | ✓ | 可重用的提示，使用 `/name` 调用或自动调用 | [Skills](https://code.claude.com/docs/zh-CN/skills.md) |
| [`commands/*.md`](#ce-commands) | 项目和全局 | ✓ | 单文件提示；与 skills 相同的机制 | [Skills](https://code.claude.com/docs/zh-CN/skills.md) |
| [`output-styles/*.md`](#ce-output-styles) | 项目和全局 | ✓ | 自定义指令集，调整 Claude 的工作方式 | [输出样式](https://code.claude.com/docs/zh-CN/output-styles.md) |
| [`agents/*.md`](#ce-agents) | 项目和全局 | ✓ | Subagent 定义及其自己的提示和工具 | [Subagents](https://code.claude.com/docs/zh-CN/sub-agents.md) |
| [`workflows/*.js`](#ce-workflows) | 项目和全局 | ✓ | 由 Claude 编写并从 `/workflows` 保存的动态工作流脚本；每个文件都成为一个 `/<name>` 命令 | [动态工作流](https://code.claude.com/docs/zh-CN/workflows.md) |
| [`agent-memory/<name>/`](#ce-agent-memory) | 项目和全局 | ✓ | Subagents 的持久内存 | [持久内存](https://code.claude.com/docs/zh-CN/sub-agents.md#enable-persistent-memory) |
| [`~/.claude.json`](#ce-claude-json) | 仅全局 | | 应用状态、账户详情、UI 切换、个人 MCP 服务器。如果该文件不存在，Claude Code 会在您启动会话或运行 `claude auth status` 等子命令时创建它，但运行 `--version` 或 `--help` 时不会创建。 | [全局配置](https://code.claude.com/docs/zh-CN/settings-reference.md#global-config-settings) |
| [`projects/<project>/memory/`](#ce-global-projects) | 仅全局 | | 自动内存：Claude 在会话间对自己的笔记 | [自动内存](https://code.claude.com/docs/zh-CN/memory.md#auto-memory) |
| [`keybindings.json`](#ce-keybindings) | 仅全局 | | 自定义快捷键 | [快捷键](https://code.claude.com/docs/zh-CN/keybindings.md) |
| [`themes/*.json`](#ce-themes) | 仅全局 | | 自定义颜色主题 | [自定义主题](https://code.claude.com/docs/zh-CN/terminal-config.md#create-a-custom-theme) |

## 按文件分类的 Frontmatter 字段

Skills、命令文件、subagents、输出样式和规则从文件顶部的 YAML [frontmatter](https://code.claude.com/docs/zh-CN/glossary.md#frontmatter) 读取其配置，每个都接受自己的一组字段。此表列出了每个文件的字段名称，并链接到描述它们的参考资料。

| 文件 | Frontmatter 字段 | 参考资料 |
| - | - | - |
| `skills/<name>/SKILL.md` | `name`, `description`, `when_to_use`, `argument-hint`, `arguments`, `disable-model-invocation`, `user-invocable`, `allowed-tools`, `disallowed-tools`, `model`, `effort`, `context`, `agent`, `background`, `hooks`, `paths`, `shell`, `metadata`, `license`, `compatibility` | [Skill frontmatter](https://code.claude.com/docs/zh-CN/skills.md#frontmatter-reference) |
| `commands/*.md` | 除 `name` 和 `paths` 外的 skill 字段 | [Skill frontmatter](https://code.claude.com/docs/zh-CN/skills.md#frontmatter-reference) |
| `agents/*.md` | `name`, `description`, `tools`, `disallowedTools`, `model`, `permissionMode`, `maxTurns`, `autoCompactWindow`, `skills`, `mcpServers`, `hooks`, `memory`, `background`, `effort`, `isolation`, `color`, `initialPrompt`, `omitClaudeMd`, `experimental` | [子代理 frontmatter](https://code.claude.com/docs/zh-CN/sub-agents.md#supported-frontmatter-fields) |
| `output-styles/*.md` | `name`, `description`, `keep-coding-instructions`, `force-for-plugin` | [Output style frontmatter](https://code.claude.com/docs/zh-CN/output-styles.md#frontmatter) |
| `rules/*.md` | `paths` | [Rule frontmatter](https://code.claude.com/docs/zh-CN/memory.md#rules-frontmatter-reference) |

在 [plugin](https://code.claude.com/docs/zh-CN/plugins/components.md#agents) 中提供的 Agents 遵守 subagent 字段的一个子集。

## 排查配置问题

如果设置、hook 或文件未生效，请参阅[调试您的配置](https://code.claude.com/docs/zh-CN/debug-your-config.md)以获取检查命令和症状优先查找表。

## 应用数据

除了您创作的配置外，`~/.claude` 还保存 Claude Code 在会话期间写入的数据。这些文件是纯文本。通过工具传递的任何内容都会在磁盘上的记录中：文件内容、命令输出、粘贴的文本。

会话记录还可能包含 Claude Code 随您的消息一起发送给模型的内容：

* 系统提示词
* 工具定义
* 请求参数
* 它加载的指令文件，例如 CLAUDE.md

### 自动清理

Claude Code 删除下面路径中的文件，一旦它们的年龄超过 [`cleanupPeriodDays`](https://code.claude.com/docs/zh-CN/settings-reference#cleanupperioddays)，只要它能安全地确定保留期。默认值为 30 天，最小值为 1；设置 `0` 会导致验证错误。相同的年龄截止值也适用于 [孤立 worktrees](https://code.claude.com/docs/zh-CN/worktrees.md#clean-up-subagent-and-background-session-worktrees) 的自动删除。

| `~/.claude/` 下的路径 | 内容 |
| - | - |
| `projects/<project>/<session>.jsonl` | 对话的会话记录：消息、工具调用和工具结果，以及 Claude Code 发送给模型的请求中的系统提示词、工具定义和参数 |
| `projects/<project>/<session>.orphaned-<timestamp>-<suffix>.jsonl`、`projects/<project>/<session>.jsonl.superseded-<timestamp>` | 会话的先前记录，Claude Code 将其搁置而不是覆盖或删除它。它不会出现在会话选择器中 |
| `projects/<project>/<session>/subagents/` | [子代理](https://code.claude.com/docs/zh-CN/sub-agents.md)的会话记录，格式与会话的会话记录相同，当父会话记录过期时随之删除 |
| `projects/<project>/<session>/tool-results/` | 大型工具输出溢出到单独的文件，以及 [MCP 工具返回的图像](https://code.claude.com/docs/zh-CN/mcp.md#images-in-tool-results) 的完整大小副本 |
| `projects/<project>/<session>/mcp-tasks/` | 恢复的会话重新跟踪其正在运行的 [MCP 任务](https://code.claude.com/docs/zh-CN/mcp.md#mcp-tasks-that-run-on-the-server)所需的内容，以及 Claude 尚未读取的已完成任务的结果 |
| `file-history/<session>/` | Claude 更改的文件的编辑前快照，用于 [checkpoint 恢复](https://code.claude.com/docs/zh-CN/checkpointing.md)。保存最近 100 个 checkpoint 的快照；没有保留 checkpoint 引用的快照文件被删除，除了每个文件的第一个快照 |
| `plans/` | 在 [Plan Mode](https://code.claude.com/docs/zh-CN/permission-modes.md#analyze-before-you-edit-with-plan-mode) 期间写入的计划文件 |
| `debug/` | 每个会话的调试日志，在启用调试日志时写入，例如当您使用 [`--debug`](https://code.claude.com/docs/zh-CN/cli-reference#cli-flags) 启动或运行 `/debug` 时 |
| `paste-cache/` | 大型粘贴的内容 |
| `image-cache/<session>/` | Claude Code v2.1.274 及更早版本保存的附加图像。更高版本将粘贴和附加的图像保存在 `~/.claude` 之外，在 [`CLAUDE_CODE_TMPDIR`](https://code.claude.com/docs/zh-CN/env-vars) 控制的临时目录下每个会话的 `images/` 目录中。扫描会删除其他会话在此处留下的目录，无论其年龄如何。 |
| `uploads/<session>/` | 您从网络或移动应用附加的文件，以及从移动应用附加的照片，当向 [Remote Control](https://code.claude.com/docs/zh-CN/remote-control.md) 会话发送消息时。对 [cloud session](https://code.claude.com/docs/zh-CN/claude-code-on-the-web.md) 的附件保存在该会话自己的云环境中，而不是在您的机器上。 |
| `dev-mods/<session>/` | [Claude 在会话期间编写的 Mods](https://code.claude.com/docs/zh-CN/plugins/mods/create.md#ask-claude-for-a-mod) |
| `session-env/` | 每个会话的环境元数据 |
| `tasks/` | 由 task 工具写入的任务列表，每个列表一个目录 |
| `shell-snapshots/` | 在启动时捕获的别名、函数和 shell 选项，由 [Bash 工具](https://code.claude.com/docs/zh-CN/tools-reference.md#bash-tool-behavior) 应用于每个命令。在正常退出时删除。扫描清理任何在崩溃后留下的内容。 |
| `backups/` | `~/.claude.json` 的早期版本，在 Claude Code 重写文件时复制。Claude Code 保留五个最新的版本，加上它无法解析的任何版本的副本。 |
| `feedback-bundles/` | 由 `/feedback` 在第三方提供商上或当未配置 Anthropic 凭证时写入的编辑后的记录存档，用于发送到您的 Anthropic 账户团队 |
| `feedback/drafts/` | 排队的 [Claude 起草的反馈](https://code.claude.com/docs/zh-CN/tools-reference.md#sendfeedback-tool-behavior)，等待您在 `/feedback` 中审查。在 `cleanupPeriodDays` 或 30 天后扫除，以较短者为准。当队列达到其 10 个草稿的限制时，Claude Code 删除最旧的草稿以腾出空间。 |
| `usage-data/` | `report.html` 和由 [`/insights`](https://code.claude.com/docs/zh-CN/costs#analyze-your-usage-patterns) 写入的时间戳报告副本，加上用于构建它们的缓存的每个会话分析数据 |
| `skills/.trash/`、`plugins/.trash/` | [Skills](https://code.claude.com/docs/zh-CN/skills.md#how-synced-skills-behave) 和 [plugins](https://code.claude.com/docs/zh-CN/plugins/loading.md#synced-plugins)，从 claude.ai 同步中删除，例如在您在 claude.ai 上关闭其中一个或停止同步后。文件保留在此处，以便您可以恢复它们，直到扫描删除它们 |
| `plugins/installed_plugins.set-aside.<date>.<hash>.json`、`plugins/installed_plugins.unreadable.<date>.<hash>.kept` | Claude Code 在重写 [`installed_plugins.json`](https://code.claude.com/docs/zh-CN/plugins/loading#find-plugins-on-disk) 之前制作的日期副本：它删除的安装记录，以及它无法读取的文件的内容。 |
| `todos/`、`statsig/`、`logs/` | 来自旧版本的旧版目录。不再写入。扫描删除其内容，然后删除空目录。 |

`sessions/` 中的会话文件、自动内存以及 Claude Desktop 和 Cowork 记录各自遵循自己的保留规则：

* **`sessions/`**：为每个运行的会话保存一个小文件，用于检测并发会话和崩溃。它不是基于年龄的扫描的一部分：Claude Code 在其会话退出时删除每个文件，并在下次启动时清理崩溃遗留物。
* **自动内存**：扫描不删除项目 [自动内存](https://code.claude.com/docs/zh-CN/memory.md#auto-memory) 目录中的内存文件，`projects/<project>/memory/`。Claude Code 仅在整个保留期内该目录为空时才删除该目录。在 v2.1.228 之前，扫描将内存目录内的文件夹视为会话数据，可能删除其下的旧文件。
* **Claude Desktop 和 Cowork 会话记录**：Claude Code 保留您在 Claude Desktop 或 Cowork 中启动或最近继续的会话的会话记录，无论其年龄如何。要给这些会话记录设置年龄限制，请设置 [`desktopSessionCleanupPeriodDays`](https://code.claude.com/docs/zh-CN/settings-reference#desktopsessioncleanupperioddays)。需要 Claude Code v2.1.248 或更高版本；早期版本在 `cleanupPeriodDays` 后删除它们。

  在以下任一情况下，Claude Code 改为在 `cleanupPeriodDays` 后删除这些会话记录：

  * [托管设置](https://code.claude.com/docs/zh-CN/managed-settings.md)设置了 `cleanupPeriodDays`
  * [HIPAA 配置适用于您的会话](https://code.claude.com/docs/zh-CN/hipaa-setup.md#check-how-developers-sign-in-and-connect)

Claude Code 在这些情况下跳过基于年龄的扫描：

* **Bare mode**：当您使用 [`--bare`](https://code.claude.com/docs/zh-CN/headless#start-faster-with-bare-mode) 运行 `claude -p` 时，Claude Code 不会在该会话中运行扫描。
* **暂停扫描**：如果 Claude Code 无法安全地确定保留期，它会暂停保留清理扫描；[`retention_sweep` 事件](https://code.claude.com/docs/zh-CN/monitoring-usage#retention-sweep-event)列出每个暂停它的配置。当原因是无法读取或解析的设置文件，或 `cleanupPeriodDays` 或 `desktopSessionCleanupPeriodDays` 明确设置的设置错误时，Claude Code 也会在 `/status` 中显示警告，直到您修复设置错误。当 [managed settings](https://code.claude.com/docs/zh-CN/server-managed-settings.md) 提供 `cleanupPeriodDays` 时，Claude Code 在任何情况下都以 managed 值运行扫描。

要检查您组织中的机器是否按您设置的保留期运行扫描，请参阅[检查保留扫描](https://code.claude.com/docs/zh-CN/monitoring-usage.md#check-the-retention-sweep)。

### 会话暂存目录

暂存是 Claude Code 为 Claude 提供的每个会话目录，用于临时文件：中间结果、辅助脚本和不属于您的项目的草稿。当 Claude 说它将某些内容保存"到暂存"时，该文件就在那里。Claude 使用它而不是 `/tmp`，可以在其中创建、编辑和读取文件而无需权限提示。

暂存位于 Claude Code 的临时目录下，而不是 `~/.claude`。找到您的平台的当前会话路径：

* **macOS**：`/private/tmp/claude-<uid>/<project>/<session-id>/scratchpad/`
* **Linux**：`/tmp/claude-<uid>/<project>/<session-id>/scratchpad/`，或当您的系统设置 `$TMPDIR` 时在其下的相同形状
* **Windows**：`%TEMP%\claude\<project>\<session-id>\scratchpad\`

`<project>` 是您的工作目录路径，其中除字母和数字外的每个字符都被替换为 `-`，例如 `-Users-you-my-project`。如果您设置了 [`CLAUDE_CODE_TMPDIR`](https://code.claude.com/docs/zh-CN/env-vars)，树会改为移动到该目录下。Hooks 接收当前会话的路径作为 [`scratchpad_dir`](https://code.claude.com/docs/zh-CN/hooks#common-input-fields)。

暂存文件的生命周期与会话的会话记录相同：[保留扫描](#cleaned-up-automatically)在删除会话记录时删除该目录，而 [`claude purge`](#clear-local-data) 不会触及临时目录。由于该目录位于系统临时位置下，您的操作系统也可能清除它，例如在重启时。要保留 Claude 在那里写入的内容，请让 Claude 将其移动到您的项目中。

会话仅在以下所有条件成立时才有暂存：

* 您使用 claude.ai 账户而不是 API 密钥登录
* 会话使用 Anthropic API，而不是 Amazon Bedrock、Google Cloud 的 Agent Platform 或 Microsoft Foundry
* [`enableArtifact`](https://code.claude.com/docs/zh-CN/settings-reference#enableartifact) 未设置为 `false`

### 保留直到您删除它们

除表中另有说明的行外，保留清理扫描不会删除下面的路径，Claude Code 会保留它们，直到您删除它们。

| `~/.claude/` 下的路径 | 内容 |
| - | - |
| `history.jsonl` | 您输入过的提示词，附带时间戳和项目路径。用于上箭头调取历史、`Ctrl+R` 历史搜索以及 `!` shell 命令补全。当 [HIPAA 配置适用于您的会话](https://code.claude.com/docs/zh-CN/hipaa-setup.md#check-how-developers-sign-in-and-connect)时，每次清理都会删除早于 `cleanupPeriodDays` 的条目。 |
| `stats-cache.json` | 由 `/usage` 显示的聚合令牌和成本计数 |
| `remote-settings.json` | [server-managed settings](https://code.claude.com/docs/zh-CN/server-managed-settings.md) 的缓存副本，用于您的组织，或当您的组织未配置任何设置时为 `{}`。仅在会话 [获取它们](https://code.claude.com/docs/zh-CN/server-managed-settings.md#platform-availability) 时存在。Claude Code 在启动时和会话期间每小时检查更新。Claude Code 在您注销时删除它。 |
| `cache/changelog.md` | Claude Code changelog 的缓存副本，由 `/release-notes` 显示。在后台刷新。 |
| `policy-limits.json` | 为您的组织缓存的功能策略设置。仅对某些账户类型存在。自动刷新。`policy-limits.json.stamp.json` sidecar 记录缓存属于哪个账户或 API 密钥。Claude Code 在您注销时删除两个文件。 |

#### 需保留的状态文件

根据您使用的功能，`~/.claude/` 中还会保存[应用数据](#application-data)下各表未列出的文件。其中，缓存和锁定文件可以安全删除。请保留以下状态文件：

* `.credentials.json`：您的 [login credentials](https://code.claude.com/docs/zh-CN/authentication.md#credential-management)
* `agent-memory/`：[subagent memory](https://code.claude.com/docs/zh-CN/sub-agents.md#enable-persistent-memory)
* `jobs/` 和 `daemon/`：[background session](https://code.claude.com/docs/zh-CN/agent-view.md#where-state-is-stored) 状态

### 纯文本存储

记录和历史在静止时未加密。操作系统文件权限是唯一的保护。如果工具读取 `.env` 文件或命令打印凭证，该值将写入 `projects/<project>/<session>.jsonl`。要减少暴露：

* 降低 `cleanupPeriodDays` 以缩短 Claude Code 保留记录的时间
* 设置 [`desktopSessionCleanupPeriodDays`](https://code.claude.com/docs/zh-CN/settings-reference#desktopsessioncleanupperioddays) 以给 Claude Desktop 和 Cowork 记录设置年龄限制
* 设置 [`CLAUDE_CODE_SKIP_PROMPT_HISTORY`](https://code.claude.com/docs/zh-CN/env-vars) 环境变量以跳过在任何模式下写入记录和提示历史。在非交互模式下，您可以改为在 `-p` 旁边传递 `--no-session-persistence`，或在 TypeScript Agent SDK 中设置 `persistSession: false`；Python SDK 没有等效选项。
* 使用 [权限规则](https://code.claude.com/docs/zh-CN/permissions.md) 拒绝读取凭证文件

### 清除本地数据

运行 `claude purge` 以删除 Claude Code 为某个项目保存的状态。它会删除：

* `projects/` 下的记录和自动内存
* 每个会话的 `tasks/`、`debug/` 和 `file-history/` 条目
* `history.jsonl` 中的匹配提示行
* `~/.claude.json` 中的项目条目

您在项目会话中粘贴或附加的图像以及每个会话的 [暂存](#session-scratchpad-directory) 存储在 Claude Code 的临时目录下，而不是 `~/.claude`，因此清除不会删除它们。[保留扫描](#cleaned-up-automatically) 仍会在它们的年龄超过 `cleanupPeriodDays` 时删除图像；清除的会话的暂存会保留，直到您删除它或您的操作系统清除临时目录。

该命令打印完整的删除计划，并在删除任何内容之前要求确认。

在 v2.1.288 之前，该命令为 `claude project purge`。

下面的示例使用 `~/work/my-repo` 作为占位符。将其替换为您的项目的路径。如果没有状态与路径匹配，该命令打印错误并以状态 1 退出。

预览计划而不删除任何内容：

```bash theme={null}
claude purge ~/work/my-repo --dry-run
```

该计划列出每个匹配项及其包含的原因：

```text theme={null}
Purge plan for /home/user/work/my-repo:

  dir:    /home/user/.claude/projects/-home-user-work-my-repo
           project transcripts (.jsonl) and memory/
  config: projects["/home/user/work/my-repo"]
           project entry in ~/.claude.json (trust, history, MCP servers)
  filter: /home/user/.claude/history.jsonl
           12 prompt(s) typed in this project

shell-snapshots/ are not project-scoped and will not be touched
backups/ may still contain this project entry in old .claude.json snapshots (/home/user/.claude/backups); at most 5 are kept and they rotate out automatically
Dry run: 3 item(s) would be deleted.
```

通过单个确认提示删除：

```bash theme={null}
claude purge ~/work/my-repo
```

该命令打印相同的计划，然后询问 `Delete 3 item(s) for /home/user/work/my-repo? This cannot be undone. [y/N]` 并仅在您回答 `y` 时删除。

省略路径以从交互式列表中选择项目。

跳过确认提示以在脚本中使用：

```bash theme={null}
claude purge ~/work/my-repo --yes
```

其他选项：

* 传递 `--all` 而不是路径，以一次清除所有项目的状态，这会直接删除 `history.jsonl` 而不是对其进行过滤。
* 传递 `-i` 以逐项执行删除计划。

该命令不理会 `shell-snapshots/` 和 `backups/`，因为这些不是项目范围的，并在计划输出中警告它们。如果有人在该机器上运行过 [`/heapdump`](https://code.claude.com/docs/zh-CN/troubleshooting#high-cpu-or-memory-usage)，也请删除它写入的 `.heapsnapshot` 文件。堆快照包含完整的对话以及进程持有的所有凭据，保留扫描和清除都不会触及它。

除[需保留的状态文件](#state-files-to-keep)外，您也可以自行删除上面的任何应用数据路径。

| 删除 | 您失去 |
| - | - |
| `~/.claude/projects/` | 恢复、继续和倒回过去的会话，以及每个项目的自动内存 |
| `~/.claude/history.jsonl` | 向上箭头提示回忆、`Ctrl+R` 历史搜索和 `!` shell 命令补全 |
| `~/.claude/paste-cache/` | 回忆的提示中的粘贴文本；请参阅 [paste large content](https://code.claude.com/docs/zh-CN/terminal-config.md#paste-large-content) |
| `~/.claude/uploads/` | 过去 [Remote Control](https://code.claude.com/docs/zh-CN/remote-control.md) 会话按路径引用的附件 |
| `~/.claude/file-history/` | 过去会话的 checkpoint 恢复 |
| `~/.claude/stats-cache.json` | 由 `/usage` 显示的历史总计 |
| `~/.claude/usage-data/` | 过去的 [`/insights`](https://code.claude.com/docs/zh-CN/costs#analyze-your-usage-patterns) 报告和用于构建它们的缓存分析数据 |
| `~/.claude/feedback-bundles/` | 您尚未发送给您的 Anthropic 账户团队的反馈和错误报告存档 |
| `~/.claude/feedback/drafts/` | 您尚未发送的 [Claude 起草的反馈](https://code.claude.com/docs/zh-CN/tools-reference.md#sendfeedback-tool-behavior) |
| `~/.claude/remote-settings.json` | 无。在下次启动时重新获取。 |
| `~/.claude/cache/changelog.md` | 无。在后台刷新。 |
| `~/.claude/policy-limits.json` | 无。自动刷新。 |
| `~/.claude/tasks/` | 您的[任务列表](https://code.claude.com/docs/zh-CN/interactive-mode.md#task-list)，包括仍在使用中的列表 |
| `~/.claude/skills/.trash/`、`~/.claude/plugins/.trash/` | 恢复 [synced skills](https://code.claude.com/docs/zh-CN/skills.md#how-synced-skills-behave) 和 [synced plugins](https://code.claude.com/docs/zh-CN/plugins/loading.md#synced-plugins) 的机会，Claude Code 已删除 |
| `~/.claude/plugins/installed_plugins.set-aside.<date>.<hash>.json`、`~/.claude/plugins/installed_plugins.unreadable.<date>.<hash>.kept` | Claude Code 删除的 plugin 安装记录副本或无法读取的文件副本。没有任何内容读取它们 |
| `~/.claude/debug/`、`~/.claude/plans/`、`~/.claude/session-env/`、`~/.claude/shell-snapshots/`、`~/.claude/backups/` | 没有面向用户的内容 |
| `~/.claude/todos/`、`~/.claude/statsig/`、`~/.claude/logs/`、`~/.claude/image-cache/` | 无。旧版目录不由当前版本写入。 |

不要删除 `~/.claude.json`、`~/.claude/settings.json` 或 `~/.claude/plugins/`：这些保存您的账户详细信息、偏好和已安装的插件。

## 相关资源

* [管理 Claude 的内存](https://code.claude.com/docs/zh-CN/memory.md)：编写和组织 CLAUDE.md、rules 和自动内存
* [配置设置](https://code.claude.com/docs/zh-CN/settings.md)：设置权限、hooks、环境变量和模型默认值
* [创建 skills](https://code.claude.com/docs/zh-CN/skills.md)：构建可重用的提示和工作流
* [配置 subagents](https://code.claude.com/docs/zh-CN/sub-agents.md)：定义具有自己上下文的专门代理
