# Claude Code

## 它是什么

Claude Code 是 Anthropic 开发的命令行编码工具，采用代理式（agentic）工作方式：接收自然语言指令后，自主读取代码、制定计划、调用工具执行、根据执行结果继续迭代，直到任务完成或需要用户决策。

它与代码补全类工具的区别在于执行者：补全工具建议代码片段、由人决定是否写入；Claude Code 直接执行操作——读写文件、运行 shell 命令、搜索代码库、调用外部系统，每个敏感操作经过权限确认。

可用形态：终端 CLI、IDE 扩展、桌面应用、浏览器。CC-Box 中使用的是终端形态，行为与终端中完全一致。

## 工作逻辑

Claude Code 以项目为单位工作。在项目根目录启动会话后，整个执行过程围绕当前代码库展开。

**启动**：在项目目录运行 `claude`。自动加载该项目的 CLAUDE.md（约定与偏好）与 `.claude/` 目录下的配置（skills、agents、hooks、权限规则）及 `.mcp.json`（外部工具连接）。

**执行循环**：理解指令 → 检索相关代码（读文件、搜索、运行命令了解现状）→ 给出计划或直接执行（编辑文件、运行命令、提交 git）→ 检查执行结果 → 继续下一步或向用户汇报。一次指令可触发多轮工具调用；执行中出现问题时会读取报错、修改方案、重试。

**权限控制**：工具调用受权限系统约束。默认模式下，写文件、运行命令等操作先请求批准；批准时可选「本次 / 总是」。规则持久化在 settings.json 的 `permissions` 字段（allow / ask / deny），可用 `/permissions` 管理。`Shift+Tab` 切换权限模式：默认（逐项询问）→ 自动接受编辑 → 计划模式（只读分析，先出方案再执行）。

**上下文管理**：对话历史与读取的文件共同占用上下文窗口。接近上限时，`/compact` 将历史总结压缩后继续；`/clear` 开启全新对话；`/resume` 回到任意历史会话。无关会话用 `/clear` 分隔可以保持上下文干净。

## 命令速查

会话内输入 `/` 查看全部命令，`/` 加字母筛选。

| 命令 | 用途 |
|------|------|
| `/init` | 分析项目并生成 CLAUDE.md 初稿 |
| `/clear` | 开启新对话（旧对话保留在 /resume 列表） |
| `/compact` | 总结当前对话，压缩释放上下文 |
| `/resume` | 恢复历史会话（按 ID、名称或选择器） |
| `/model` | 切换模型 |
| `/memory` | 编辑 CLAUDE.md 记忆文件 |
| `/permissions` | 管理工具权限规则 |
| `/agents` | 管理 subagents 配置 |
| `/skills` | 列出可用 skills |
| `/hooks` | 查看 hooks 配置 |
| `/mcp` | 管理 MCP 服务器连接 |
| `/plugin` | 管理插件 |
| `/config` | 设置：主题、模型、输出样式等 |
| `/context` | 可视化当前上下文占用 |
| `/doctor` | 诊断安装与配置问题 |
| `/export` | 导出当前对话为纯文本 |
| `/usage` | 查看成本与用量 |
| `/help` | 帮助与完整命令列表 |

## 快捷键速查

| 快捷键 | 用途 |
|--------|------|
| `Esc` | 中断当前生成 |
| `Esc Esc` | 回退对话 / 代码到之前的状态 |
| `Shift+Tab` | 循环切换权限模式 |
| `Ctrl+R` | 反向搜索命令历史 |
| `Ctrl+O` | 展开详细转录，查看工具调用过程 |
| `Ctrl+B` | 将当前命令转入后台运行 |
| `Ctrl+T` | 显示 / 隐藏后台任务列表 |
| `Ctrl+L` | 清屏重绘（对话保留） |
| `Ctrl+G` | 在外部编辑器中编辑提示 |
| `↑` / `↓` | 浏览命令历史 |

输入前缀：

| 前缀 | 作用 |
|------|------|
| `/` 开头 | 斜杠命令或 skill |
| `!` 开头 | 直接执行 shell 命令，输出加入会话上下文 |
| `@` | 文件路径自动补全与引用 |

## CLAUDE.md

CLAUDE.md 是每次会话启动时自动加载的指令文件，用于向 Claude 说明项目约定，内容持续占用上下文。它是「建议」性质：Claude 读取并尽量遵循，但不保证——需要保证执行的行为应使用 hooks 或权限规则。

三个层级：

| 文件 | 位置 | 内容 | 提交 |
|------|------|------|------|
| 项目 CLAUDE.md | 项目根目录 | 项目约定：构建命令、代码风格、架构说明 | 提交，团队共享 |
| 全局 CLAUDE.md | `~/.claude/CLAUDE.md` | 个人偏好：回复风格、提交信息格式 | 不提交 |
| CLAUDE.local.md | 项目根目录 | 对此项目的私人偏好 | 不提交（加入 .gitignore） |

加载规则：三者同时加载；指令冲突时项目级优先。全局文件进入所有项目的上下文，应保持精简。项目文件接近 200 行时，可拆分到 `.claude/rules/` 按主题组织（支持按文件路径条件加载）。

建议写入的内容：

- 常用命令——构建、测试、lint 的确切命令
- 代码约定——命名、错误处理、测试要求
- 架构要点——目录结构、模块边界、关键设计决策
- 注意事项——已知的坑、不应改动的文件、环境差异

生成与维护：新项目运行 `/init` 生成初稿（Claude 分析代码库后写入）；日常修改用 `/memory` 打开编辑。

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

## Subagents（子代理）

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
