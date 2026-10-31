# Claude Code

## 它是什么

Claude Code 是 Anthropic 开发的命令行 AI 工具。虽然编码是它的强项，但凡是命令行能做的事它都能做：你说「修复这个失败的测试」，它会自己读代码、找原因、改掉、跑通；你说「把这周的会议记录整理成一页文档」「分析这份访问日志，找出响应最慢的接口」，它同样自己翻文件、跑命令、给你结果——而不是丢给你一段建议，由你来执行。

它的工作方式是代理式（agentic）的：接收自然语言指令后，自主读取文件、制定计划、调用工具执行、根据执行结果继续迭代，直到任务完成或需要你决策。它能直接执行的操作包括读写文件、运行 shell 命令、搜索代码库、查询数据、调用外部系统；每个敏感操作都会先经过你的确认。

可用形态：终端 CLI、IDE 扩展、桌面应用、浏览器。CC-Box 中使用的是终端形态，行为与终端中完全一致。

> 延伸：[Claude Code 概览](reference.html#/getting-started/overview)、[Claude Code 如何工作](reference.html#/getting-started/core-concepts/how-claude-code-works)

## 工作逻辑

在项目目录运行 `claude` 启动会话后，Claude 能看到你的整个项目：所有文件、你能运行的任何命令、git 状态。所以你不必告诉它「先打开 src/auth/session.ts 再看第 120 行」——说「登录过期后会话没清理，查一下并修好」，它自己会找到相关文件；也不必先教它项目怎么构建——构建脚本就在那里，它读得到。

执行过程分三步：先收集上下文（读文件、搜索、跑命令了解现状），再动手（编辑文件、运行命令），然后验证（跑测试、检查输出）。不对就调整方案重试。你随时可以按 `Esc` 打断它改方向；也可以不打断，直接把更正发过去，它做完当前这步就会看到。

> 延伸：[Claude Code 如何工作](reference.html#/getting-started/core-concepts/how-claude-code-works)、[常见工作流](reference.html#/getting-started/use-claude-code/common-workflows)

## 会话与上下文

每个会话独立保存：对话记录在本地，关掉终端、重启电脑都不会丢，随时可以接着上次的进度继续。任务做完的会话用 `/clear` 结束，上下文从零开始，不被旧内容干扰。

Claude 的记忆有上限：对话越长、读的文件越多，早期内容会被压缩成摘要，细节可能丢失。两个习惯可以避免踩坑——要它长期记住的规则写进 CLAUDE.md（每次会话都会完整加载）；拿不准时用 `/context` 看一眼当前什么占了空间。

| 操作 | 命令 / 快捷键 |
|------|---------------|
| 接着上次继续 | `claude --continue` |
| 挑一个历史会话恢复 | `claude --resume` / 会话内 `/resume` |
| 按名称恢复 | `claude --resume <name>` |
| 恢复关联某 PR 的会话 | `claude --from-pr <number>` |
| 复制一份会话另走一条路（原会话保留） | `--fork-session` / `/branch` |
| 立刻压缩上下文（可带焦点） | `/compact`、`/compact focus on the API changes` |
| 结束当前任务、开新对话 | `/clear` |
| 查看上下文占用 | `/context` |

> 延伸：[管理会话](reference.html#/getting-started/use-claude-code/sessions)、[探索上下文窗口](reference.html#/getting-started/core-concepts/context-window)、[提示缓存](reference.html#/getting-started/core-concepts/prompt-caching)

## 权限与安全

改错了能退回：Claude 每次编辑文件前都会保存快照，输入框为空时连按两次 `Esc`，可以从列表里选一个时间点恢复代码和对话。这个回退不依赖 git，未提交的改动也能退。

要不要每次都问你，由权限模式决定，`Shift+Tab` 切换：

| 模式 | 行为 |
|------|------|
| Manual（`default`） | 每个写操作、每个命令都先问你 |
| Accept edits | 文件编辑不再问，命令仍会问 |
| Plan | 只读分析，先给出方案，你批准后才动手 |
| Auto | 大多数操作直接执行，风险操作由后台分类器拦截 |
| `dontAsk` | 只允许预先批准过的操作，其余直接拒绝（用于 CI） |
| `bypassPermissions` | 全部放行（只应在隔离容器里用） |

对常用命令可以一劳永逸：把 `npm test`、`git status` 这类放心命令加进 settings.json 的 `permissions.allow`，以后不再询问；`deny` 规则在任何模式下都生效。注意回退只覆盖文件改动——数据库、部署这类影响远端的操作退不回来，靠权限规则管住。

> 延伸：[选择权限模式](reference.html#/configuration/permissions-and-sandboxing/permission-modes)、[权限](reference.html#/configuration/permissions-and-sandboxing/permissions)、[Bash 沙箱](reference.html#/configuration/permissions-and-sandboxing/sandboxing)、[检查点与回退](reference.html#/reference/checkpointing)

## CLAUDE.md 与记忆

同样的话不需要说第二遍：第二次纠正 Claude 的同一个错误时，把这句话写进 CLAUDE.md，以后每个会话它都自动知道。文件就是 Markdown，放在项目根目录、随仓库提交，团队所有人共享。

放的位置决定对谁生效：

| 位置 | 生效范围 |
|------|----------|
| `./CLAUDE.md` | 本项目，提交共享 |
| `~/.claude/CLAUDE.md` | 你的所有项目 |
| `CLAUDE.local.md` | 本项目、仅自己（不提交） |
| `.claude/rules/*.md` | 按主题拆分，可限定只在处理某些文件时加载 |

写什么：构建和测试的确切命令、代码约定、目录结构、已知的坑。保持每个文件 200 行以内——它每次会话都占上下文，越长遵守得越差。`/init` 让 Claude 分析代码库生成初稿，`/memory` 打开编辑。

此外 Claude 会自己记笔记（自动记忆）：你纠正过它的偏好、它了解到的项目背景，存在本地并在后续会话自动带上，不需要你维护。仓库里没有 CLAUDE.md 但有 `AGENTS.md` 时，Claude 也可以直接读它。

需要注意：CLAUDE.md 是提示不是保证。必须每次执行的事（比如禁止某个命令），用 hook 实现。

> 延伸：[Claude 如何记住你的项目](reference.html#/getting-started/use-claude-code/memory)、[探索 .claude 目录](reference.html#/getting-started/core-concepts/claude-directory)

## 命令速查

会话内输入 `/` 查看全部命令（含 skills 与插件提供的命令），`/` 加字母筛选。

| 命令 | 用途 |
|------|------|
| `/init` | 分析项目并生成 CLAUDE.md 初稿 |
| `/clear` | 开启新对话（旧对话保留在 /resume 列表） |
| `/compact` | 压缩上下文，可带焦点参数 |
| `/resume` | 切换到历史会话 |
| `/branch` | 分叉当前会话 |
| `/model` | 切换模型（也可用 `Alt+P`） |
| `/memory` | 编辑记忆文件、开关自动记忆 |
| `/permissions` | 管理权限规则 |
| `/agents` | 管理 subagents |
| `/skills` | 列出可用 skills |
| `/hooks` | 查看 hooks 配置 |
| `/mcp` | 管理 MCP 连接与授权 |
| `/plugin` | 管理插件与市场 |
| `/sandbox` | 配置 Bash 沙箱 |
| `/config` | 设置：主题、模型、输出样式等 |
| `/context` | 可视化上下文占用 |
| `/tasks` | 查看运行中的 shell 与子代理 |
| `/doctor` | 诊断安装与配置；`prompt-audit` 审计指令文件 |
| `/export` | 导出当前对话 |
| `/usage` | 查看成本与用量 |
| `/help` | 帮助与完整命令列表 |

> 延伸：[命令参考](reference.html#/reference/commands)、[CLI 参考](reference.html#/reference/cli-reference)

## 快捷键速查

| 快捷键 | 用途 |
|--------|------|
| `Esc` | 中断当前生成 / 关闭对话框 / 拒绝权限请求 |
| `Esc Esc` | 输入有文字：清空草稿；输入为空：打开回退菜单 |
| `Shift+Tab` | 循环切换权限模式 |
| `Ctrl+C` | 中断运行；无运行时清空输入，再按退出 |
| `Ctrl+D` | 退出会话（两次确认） |
| `Ctrl+R` | 反向搜索命令历史 |
| `Ctrl+O` | 展开查看每次工具调用的细节与时间戳 |
| `Ctrl+B` | 当前命令转入后台运行 |
| `Ctrl+T` | 显示 / 隐藏 Claude 的任务清单 |
| `Ctrl+Enter` | 立即发送排队消息 |
| `Ctrl+L` | 重绘屏幕（对话保留） |
| `Ctrl+G` / `Ctrl+X Ctrl+E` | 在外部编辑器中编辑提示 |
| `Ctrl+S` | 隐藏 / 恢复输入草稿 |
| `Alt+P` | 切换模型（不清空提示） |
| `Alt+T` | 开关扩展思考 |
| `↑` / `↓` | 命令历史（多行输入时先移动光标） |

输入前缀：

| 前缀 | 作用 |
|------|------|
| `/` 开头 | 斜杠命令或 skill |
| `!` 开头 | 直接执行 shell 命令，输出加入会话上下文 |
| `@` | 文件路径补全与引用 |
| `?` 空输入 | 快捷键帮助面板 |

多行输入：`\` + Enter 通用；`Shift+Enter` / `Option+Enter` 取决于终端；`Ctrl+J` 无需配置。

> 延伸：[交互模式参考](reference.html#/reference/interactive-mode)、[自定义快捷键](reference.html#/configuration/interface/keybindings)

## Skills

**是什么**：以 Markdown 文件定义的指令包，一个 skill 是一个目录，入口为 `SKILL.md`。

**本质：经验的复用**。一件事该按什么步骤、什么标准来做，这类经验可以固化成一个可反复调用的指令包。不必每次在对话里重新交代流程，也不用指望模型临场领会团队约定——需要时调用一次，或由 Claude 根据描述自动加载。

**什么时候用**：流程固定且会重复出现的场景——发布 / 部署步骤、代码审查清单、项目脚手架生成、周期性的分析任务。

**怎么用**：位置分两级——

| 级别 | 位置 | 生效范围 |
|------|------|----------|
| 个人 | `~/.claude/skills/<名称>/SKILL.md` | 你的所有项目 |
| 项目 | `.claude/skills/<名称>/SKILL.md` | 仅此项目，可提交 |

`SKILL.md` 由 YAML frontmatter 与 Markdown 正文组成，frontmatter 的 `name` 即命令名，`description` 供自动加载判断：

```markdown
---
name: api-conventions
description: 本仓库的 API 设计约定，设计新接口时使用
---

新增接口时遵循：
1. 路径使用复数名词
2. 错误响应统一使用 Problem Details 格式
3. …
```

目录中可附支持文件（模板、示例、脚本），在 SKILL.md 中引用即可。同级同名的旧式 `.claude/commands/<名称>.md` 仍然有效。

调用：`/<名称>` 手动调用；`/skills` 列出全部。

> 延伸：[使用 skills 扩展 Claude](reference.html#/build-with-claude-code/skills/skills)

## Subagents

**是什么**：用 Markdown 文件定义的专门代理，拥有独立的上下文窗口，可限定工具集与模型。

**本质：隔离上下文的分工**。主对话负责思路和结论；需要大量阅读、搜索、试错的活交给子代理，它在自己的上下文里完成过程，只把结果交回来。主对话不会因为子代理读过几十个文件而变臃肿——这就是它最大的价值。

**什么时候用**：需要大量阅读代码的独立任务（代码审查、依赖分析）、可并行拆分的子任务、需要限制权限的场合（如只读分析）。

**怎么用**：位置分两级——

| 级别 | 位置 | 生效范围 |
|------|------|----------|
| 个人 | `~/.claude/agents/<名称>.md` | 你的所有项目 |
| 项目 | `.claude/agents/<名称>.md` | 仅此项目，可提交 |

```markdown
---
name: code-reviewer
description: 审查代码变更，关注正确性与边界条件。完成代码修改后使用。
tools: Read, Glob, Grep
model: sonnet
---

审查时逐项检查：
- 逻辑正确性与边界条件
- 错误处理是否完整
- 是否引入安全问题
输出按文件分组的审查意见清单。
```

frontmatter 字段：`name`、`description`（供委派判断）、`tools`（可选，逗号分隔的工具清单；缺省为全部）、`model`（可选）。`/agents` 打开交互管理界面。

调用：由 Claude 依据 description 自主委派，或在指令中显式要求使用某个子代理。

> 延伸：[创建自定义 subagents](reference.html#/build-with-claude-code/agents-and-parallel-work/sub-agents)、[并行运行代理](reference.html#/build-with-claude-code/agents-and-parallel-work/agents)、[用 worktrees 运行并行会话](reference.html#/build-with-claude-code/agents-and-parallel-work/worktrees)、[动态工作流](reference.html#/build-with-claude-code/agents-and-parallel-work/workflows)

## Hooks

**是什么**：绑定在会话事件上的 shell 命令，事件到达时由 Claude Code 执行。

**本质：关键流程节点的自动化**。CLAUDE.md 与 skill 是“建议”——模型读到并尽量遵循，但不保证；hook 是“程序”——到了那个节点一定会执行。凡是“必须发生”的事都适合做成 hook：每次编辑后统一格式化、拦截危险命令、等待输入时发通知。

**什么时候用**：文件编辑后自动格式化、拦截危险命令、等待输入时发送系统通知、会话开始时注入环境信息。

**怎么用**：写入项目 `.claude/settings.json` 或全局 `~/.claude/settings.json` 的 `hooks` 字段。常用事件：

| 事件 | 触发时机 |
|------|----------|
| `SessionStart` | 会话开始或恢复 |
| `UserPromptSubmit` | 用户提交提示后、处理前 |
| `PreToolUse` | 工具调用执行前（可阻止） |
| `PostToolUse` | 工具调用成功后 |
| `PostToolUseFailure` | 工具调用失败后 |
| `Notification` | 发送通知（等待输入、请求权限） |
| `SubagentStart` / `SubagentStop` | 子代理启动 / 结束 |
| `Stop` | 一轮响应结束 |
| `PreCompact` / `PostCompact` | 上下文压缩前 / 后 |

```json
{
  "hooks": {
    "PostToolUse": [
      {
        "matcher": "Edit|Write",
        "hooks": [
          {
            "type": "command",
            "command": "jq -r '.tool_input.file_path' | xargs prettier --write"
          }
        ]
      }
    ]
  }
}
```

结构：事件名 → 数组，每项含 `matcher`（工具名正则，可省略表示全部）与 `hooks`（命令列表）。命令通过 stdin 收到事件 JSON，可用退出码与 stdout JSON 控制行为（如 PreToolUse 返回阻止该次调用）。`/hooks` 查看当前全部配置。

> 延伸：[使用 hooks 自动化操作](reference.html#/build-with-claude-code/automation/hooks-guide)、[Hooks 参考](reference.html#/reference/hooks)

## MCP

**是什么**：Model Context Protocol，连接外部工具与数据源的开放协议。一个 MCP 服务器向 Claude 提供三类能力：工具（可调用的函数）、资源（可读取的数据）、提示（预定义的提示词）。

**本质：扩展 agent 能做的事**。模型本身只能读写文件和运行命令；MCP 把成熟、固定的外部操作——查询数据库、操作浏览器、调用内部 API——变成 Claude 可以直接调用的工具。相当于给 agent 装上新的手脚，而不必在代码里重新实现一遍。

**什么时候用**：查询公司数据库、用浏览器操作并测试网页、对接项目管理 / 文档系统、调用内部工具链。

**怎么用**：两种方式——

命令行添加：

```bash
# 本地进程（stdio）
claude mcp add --transport stdio my-db -- npx db-mcp-server --config db.json
# 远程服务器（HTTP）
claude mcp add --transport http notion https://mcp.notion.com/mcp
```

或直接写项目根目录 `.mcp.json`（可提交，团队共享）：

```json
{
  "mcpServers": {
    "database-tools": {
      "command": "npx",
      "args": ["-y", "db-mcp-server"],
      "env": { "DB_URL": "postgres://localhost/app" }
    },
    "internal-api": {
      "type": "http",
      "url": "https://internal.example.com/mcp"
    }
  }
}
```

管理：`/mcp` 查看连接状态、工具列表与 OAuth 授权；`claude mcp list` / `claude mcp remove` 维护清单。stdio 服务器随会话启动，HTTP 服务器按需连接。

> 延伸：[MCP 完整参考](reference.html#/build-with-claude-code/mcp/mcp)、[MCP 快速入门](reference.html#/build-with-claude-code/mcp/mcp-quickstart)

## Plugins

**是什么**：把 skills、agents、hooks、MCP 服务器、命令等组件打包成的可安装单元，以 `.claude-plugin/plugin.json` 清单描述身份与版本。

**本质：把能力打包成一个整体**。前面几种机制各自解决一个问题；plugin 把它们组合起来，一次安装、一起生效、按版本分发。单个 skill 或 agent 文件适合放在项目里随手维护，成套的工作流则适合做成插件共享。

**什么时候用**：团队统一的工作流配置（审查 agents + 格式化 hooks + 内部 MCP）、开源社区共享、个人跨项目复用整套扩展。

**怎么用**：`/plugin` 打开管理器，可从 marketplace 安装、更新、启停插件，也可加载本地目录。插件清单示例：

```json
{
  "name": "team-workflow",
  "description": "团队审查与发布流程",
  "version": "1.2.0"
}
```

插件目录结构：`.claude-plugin/plugin.json`（清单，必需）+ `skills/`、`agents/`、`hooks/`、`.mcp.json`（可选组件）。插件中的组件带命名空间（`插件名:skill名`），与本地配置不冲突。开发时可从本地路径加载调试；项目可通过 settings.json 的 `enabledPlugins` 控制默认启用集合。

> 延伸：[插件概览](reference.html#/plugins/overview)、[安装插件](reference.html#/plugins/use-plugins/install)、[创建插件](reference.html#/plugins/create-plugins/create)、[插件市场](reference.html#/plugins/use-plugins/anthropic-marketplaces)

## 扩展机制怎么选

| 需求 | 用 |
|------|-----|
| Claude 每个会话都该知道的事实与规则 | CLAUDE.md |
| 特定条件下的规则，根据文件名自动加载 | `.claude/rules/` |
| 反复粘贴的流程、参考文档 | Skill |
| 大量阅读的辅助任务、并行工作 | Subagent |
| 必须每次发生的事件自动化 | Hook |
| 连接外部系统、可重复的固定操作 | MCP |
| 跨仓库复用整套配置 / 分发 | Plugin |
| 固定 Claude 的角色与响应格式 | 输出样式（`/config`） |

> 延伸：[扩展 Claude Code](reference.html#/getting-started/core-concepts/features-overview)

## 更多主题

速查之外的内容，按需查阅：

- [设置文件](reference.html#/configuration/settings/settings)、[设置项参考](reference.html#/configuration/settings/settings-reference)
- [用 Claude API 构建自己的代理（Agent SDK）](reference.html#/agent-sdk/overview)
- [术语表](reference.html#/reference/glossary/glossary)
