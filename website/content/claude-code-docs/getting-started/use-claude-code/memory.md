> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Claude 如何记住您的项目

> 使用 CLAUDE.md 或 AGENTS.md 文件为 Claude 提供持久指令，并让 Claude 通过自动记忆自动积累学习。

每个 Claude Code 会话都以全新的上下文窗口开始。两种机制可以跨会话传递知识：

* **CLAUDE.md 文件**：您编写的指令，为 Claude 提供持久上下文。Claude 也可以读取仓库的 [`AGENTS.md` 文件](#agents-md)来代替 CLAUDE.md
* **自动记忆**：Claude 根据您的更正和偏好自己编写的笔记

本页面涵盖以下内容：

* [编写和组织 CLAUDE.md 文件](#claude-md-files)
* [使用现有 AGENTS.md](#agents-md) 作为您的项目指令
* [使用 `.claude/rules/` 将规则范围限定为特定文件类型](#organize-rules-with-claude/rules/)
* [配置自动记忆](#auto-memory)，以便 Claude 自动记笔记
* [故障排除](#troubleshoot-memory-issues)当指令未被遵循时

## CLAUDE.md 与自动记忆

Claude Code 有两个互补的记忆系统。两者都在每次对话开始时加载。Claude 将它们视为上下文，而不是强制配置。要阻止某个操作，无论 Claude 决定什么，请改用 [PreToolUse hook](https://code.claude.com/docs/zh-CN/hooks-guide.md)。你的指令越具体和简洁，Claude 遵循它们的一致性就越高。

| | CLAUDE.md 文件 | 自动记忆 |
| :- | :- | :- |
| **谁编写** | 你 | Claude |
| **包含内容** | 指令和规则 | 学习和模式 |
| **范围** | 项目、用户或组织 | 每个存储库，跨 worktrees 共享 |
| **加载到** | 每个会话 | 每个会话（前 200 行或 25KB） |
| **用于** | 编码标准、工作流、项目架构 | 你的偏好、你给 Claude 的更正、Claude 无法从代码中推导的项目上下文 |

当你想指导 Claude 的行为时，使用 CLAUDE.md 文件。自动记忆让 Claude 从你的更正中学习，无需手动操作。

Subagents 也可以维护自己的自动记忆。有关详细信息，请参阅 [subagent 配置](https://code.claude.com/docs/zh-CN/sub-agents.md#enable-persistent-memory)。

## CLAUDE.md 文件

CLAUDE.md 文件是 markdown 文件，为 Claude 提供项目、个人工作流或整个组织的持久指令。您用纯文本编写这些文件；Claude 在每个会话开始时读取它们。如果您的仓库改用 `AGENTS.md`，请参阅 [AGENTS.md](#agents-md)。

### 何时添加到 CLAUDE.md

将 CLAUDE.md 视为您写下本来需要重新解释的内容的地方。在以下情况下添加到它：

* Claude 第二次犯同样的错误
* 代码审查发现 Claude 应该了解的关于此代码库的内容
* 您在聊天中输入的相同更正或澄清是您上一个会话中输入的
* 新队友需要相同的上下文才能提高生产力

将其保持为 Claude 应该在每个会话中保留的事实：构建命令、约定、项目布局、"始终执行 X"规则。如果条目是多步骤过程或仅对代码库的一部分重要，请将其移至 [skill](https://code.claude.com/docs/zh-CN/skills.md) 或 [path-scoped rule](#organize-rules-with-claude/rules/) 代替。[扩展概述](https://code.claude.com/docs/zh-CN/features-overview.md#build-your-setup-over-time) 涵盖何时使用每种机制。

### 选择 CLAUDE.md 文件的位置

CLAUDE.md 文件可以位于多个位置，每个位置具有不同的作用域。下表按加载顺序列出它们，从最广泛的作用域到最具体的作用域，因此项目指令在用户指令之后出现在上下文中。

| 作用域 | 位置 | 目的 | 用例示例 | 共享对象 |
| - | - | - | - | - |
| **托管策略** | • macOS: `/Library/Application Support/ClaudeCode/CLAUDE.md` • Linux 和 WSL: `/etc/claude-code/CLAUDE.md` • Windows: `C:\Program Files\ClaudeCode\CLAUDE.md` | 由 IT/DevOps 管理的组织范围指令 | 公司编码标准、安全策略、合规要求 | 组织中的所有用户 |
| **用户指令** | `~/.claude/CLAUDE.md` | 所有项目的个人偏好 | 代码样式偏好、个人工具快捷方式 | 仅您（所有项目） |
| **项目指令** | `./CLAUDE.md` 或 `./.claude/CLAUDE.md`。有关何时加载 `./AGENTS.md` 而不是或与它们一起加载，请参阅 [AGENTS.md](#agents-md) | 项目的团队共享指令 | 项目架构、编码标准、常见工作流 | 通过源代码控制的团队成员 |
| **本地指令** | `./CLAUDE.local.md` | 个人项目特定偏好；添加到 `.gitignore` | 您的沙箱 URL、首选测试数据 | 仅您（当前项目） |

工作目录上方目录层次结构中的 CLAUDE.md 和 CLAUDE.local.md 文件在启动时加载。子目录中的文件按需加载。有关它们何时加载以及完整的解析顺序，请参阅 [CLAUDE.md 文件如何加载](#how-claude-md-files-load)。

对于大型项目，您可以使用 [project rules](#organize-rules-with-claude/rules/) 将指令分解为特定主题的文件。规则允许您将指令范围限定为特定文件类型或子目录。

### 设置项目 CLAUDE.md

项目 CLAUDE.md 可以存储在 `./CLAUDE.md` 或 `./.claude/CLAUDE.md` 中。创建此文件并添加适用于在项目上工作的任何人的指令：构建和测试命令、编码标准、架构决策、命名约定和常见工作流。这些指令通过版本控制与您的团队共享，因此请关注项目级标准而不是个人偏好。要确认文件已加载，请在会话中运行 `/context` 并检查 **Memory files** 下的列表。

> [!TIP]
>
> 运行 `/init` 自动生成起始 CLAUDE.md。Claude 分析您的代码库并创建一个包含构建命令、测试指令和它发现的项目约定的文件。如果 CLAUDE.md 已存在，`/init` 会建议改进而不是覆盖它。从那里进行细化，添加 Claude 不会自己发现的指令。
>
> 为了启用交互式多阶段流程，请在运行 `/init` 之前将 `CLAUDE_CODE_NEW_INIT` 环境变量设置为 `1`。在您的 shell 中或在设置文件的 `env` 块中设置它，如 [设置环境变量](https://code.claude.com/docs/zh-CN/env-vars.md#set-environment-variables) 中所示。设置后，`/init` 会询问要设置哪些制品：CLAUDE.md 文件、skill 和 hook。然后它使用子代理探索您的代码库，通过后续问题填补空白，并在写入任何文件之前呈现可审查的提案。该变量仅改变 `/init` 的运行方式，因此您可以保持它的设置。

### 编写有效的指令

Claude 将 CLAUDE.md 文件视为上下文而不是强制配置，因此您编写指令的方式会影响 Claude 遵循它们的可靠性。编写具体到足以验证的指令：

* "使用 2 空格缩进"而不是"正确格式化代码"
* "在提交前运行 `npm test`"而不是"测试您的更改"
* "API 处理程序位于 `src/api/handlers/`"而不是"保持文件有组织"

保持您的文件简短、有组织和一致：

* **大小**：每个 CLAUDE.md 文件目标在 200 行以下。较长的文件消耗更多上下文并降低遵守度。将仅对代码库的一部分重要的指令移至 [path-scoped rules](#path-specific-rules)，这样它们仅在 Claude 处理匹配文件时加载。[导入](#import-additional-files) 帮助您组织一个长文件，但不会减少其上下文成本，因为导入的文件会与导入它们的 CLAUDE.md 一起加载。
* **结构**：使用 markdown 标题和项目符号来分组相关指令。有组织的部分比密集段落更容易让 Claude 遵循。
* **一致性**：如果两条指令相互矛盾，Claude 可能会任意选择一条。定期审查您的 CLAUDE.md 文件、子目录中的嵌套 CLAUDE.md 文件和 [`.claude/rules/`](#organize-rules-with-claude/rules/)，以删除过时或冲突的指令。要让 Claude 为您找到它们，请 [运行提示词审计](#audit-your-instruction-files)。

#### 审计您的指令文件

要让 Claude 检查您的指令文件是否有过时或冲突的内容，请在会话中运行 `/doctor prompt-audit`。Claude 查找问题，例如为旧模型编写的指令、对不存在的文件或命令的引用，以及相互矛盾的文件。您会获得一份发现报告和一组建议的编辑，在您要求 Claude 应用它们之前，您的文件中不会有任何更改。

默认情况下，审计涵盖您的 CLAUDE.md、CLAUDE.local.md 和 AGENTS.md 文件，以及 `.claude/` 和 `~/.claude/` 下的规则、skill、命令、子代理和输出样式。要审计一个文件或目录，请改为传递其路径，例如 `/doctor prompt-audit .claude/skills/deploy`。

审计通过捆绑的 `/claude-api` skill 运行。当该 skill 在 [`skillOverrides`](https://code.claude.com/docs/zh-CN/skills#override-skill-visibility-from-settings) 中关闭或使用 [`disableBundledSkills`](https://code.claude.com/docs/zh-CN/settings-reference#disablebundledskills) 时，它不可用。`/doctor prompt-audit` 需要 Claude Code v2.1.283 或更高版本。

### 导入其他文件

CLAUDE.md 文件可以使用 `@path/to/import` 语法导入其他文件。导入的文件被展开并加载到上下文中，与引用它们的 CLAUDE.md 一起。

允许相对路径和绝对路径。相对路径相对于包含导入的文件解析，而不是工作目录。导入的文件可以递归导入其他文件，最大深度为四跳。

要导入其路径包含空格的文件，请在每个空格前放置反斜杠。没有反斜杠，路径在第一个空格处结束，即使导入在其自己的行上。用引号包装的路径根本不导入，无论是否有反斜杠。此导入从名为 `Design Docs` 的文件夹加载文件：

```text theme={null}
- API conventions @Design\ Docs/api-conventions.md
```

导入解析跳过 Markdown 代码跨度和围栏代码块。要在您的 CLAUDE.md 中提及路径而不导入它，请将其包装在反引号中：编写 `` `@README` `` 保持文本字面，而 `@README` 在反引号外导入文件。

要引入 README、package.json 和工作流指南，请在 CLAUDE.md 中的任何地方使用 `@` 语法引用它们：

```text theme={null}
See @README for project overview and @package.json for available npm commands for this project.

# Additional Instructions
- git workflow @docs/git-instructions.md
```

对于不应该检入版本控制的私人项目特定偏好，请在项目根目录创建 `CLAUDE.local.md`。它与 `CLAUDE.md` 一起加载并以相同方式处理。将 `CLAUDE.local.md` 添加到您的 `.gitignore` 以便不提交它。设置 `CLAUDE_CODE_NEW_INIT=1` 后，运行 `/init` 并选择个人选项会为您执行此操作。

如果您在同一仓库的多个 git worktree 中工作，gitignored `CLAUDE.local.md` 仅存在于您创建它的 worktree 中。要在 worktree 之间共享个人指令，请改为从您的主目录导入文件：

```text theme={null}
# Individual Preferences
- @~/.claude/my-project-instructions.md
```

> [!WARNING]
>
> 项目级记忆文件中的导入是外部的，当其路径解析到工作目录外时，例如上面的主目录导入。Claude Code 首次在项目中遇到外部导入时，会显示一个批准对话框，列出文件。如果您拒绝，导入保持禁用状态，对话框不会再出现。
>
> Claude Code 显示对话框以保护您免受其他人提交到共享项目的文件。用户作用域记忆文件，例如 `~/.claude/CLAUDE.md` 和 `~/.claude/rules/`，是您自己编写的文件。除了在您的桌面上的 [Cowork](https://claude.com/product/cowork) 会话中，Claude Code 加载它们的导入而不显示对话框，并像信任您的其余个人配置一样信任它们。
>
> 在您的桌面上的 Cowork 会话中，Claude Code 跳过用户作用域文件中解析到会话工作目录外的路径的任何导入，并加载文件的其余部分。在这些会话中，它也跳过本身是符号链接或硬链接的 `~/.claude/CLAUDE.md`，以及指向工作目录外的符号链接 `~/.claude/rules/` 目录或规则文件。

### CLAUDE.md 文件如何加载

Claude Code 从您的当前工作目录和其上方的每个目录加载 `CLAUDE.md` 和 `CLAUDE.local.md`。在 `foo/bar/` 中运行 Claude Code，它从 `foo/bar/CLAUDE.md`、`foo/CLAUDE.md` 和任何 `CLAUDE.local.md` 文件加载指令。

所有发现的文件被连接到上下文中，而不是相互覆盖。在目录树中，内容从文件系统根目录向下排序到您的工作目录。对于 `foo/bar/` 示例，`foo/CLAUDE.md` 在上下文中出现在 `foo/bar/CLAUDE.md` 之前，因此更接近您启动 Claude 的位置的指令最后读取。在每个目录中，`CLAUDE.local.md` 附加在 `CLAUDE.md` 之后，因此您的个人笔记是 Claude 在该级别读取的最后一件事。

Claude 还发现当前工作目录下子目录中的 `CLAUDE.md` 和 `CLAUDE.local.md` 文件。它们不会在启动时加载，而是在 Claude 读取、写入或编辑该子目录中的其他文件时，由 Claude Code 逐个加载。读取包括使用[算作读取](https://code.claude.com/docs/zh-CN/tools-reference.md#edit-tool-behavior)的 Bash 命令查看文件，例如对单个文件使用 `cat` 或 `head`。对于 `.claude/worktrees/` 下 worktree 中的文件，请参阅 [使用 worktree 隔离子代理](https://code.claude.com/docs/zh-CN/worktrees.md#isolate-subagents-with-worktrees)。

如果您在大型 monorepo 中工作，其中其他团队的 CLAUDE.md 文件被拾取，请使用 [`claudeMdExcludes`](#exclude-specific-claude-md-files) 跳过它们。有关根目录和每目录 CLAUDE.md 文件和规则的完整布局，请参阅 [Monorepos 和大型仓库](https://code.claude.com/docs/zh-CN/large-codebases.md)。

CLAUDE.md 文件中的块级 HTML 注释（`<!-- maintainer notes -->`）在内容注入到 Claude 的上下文之前被剥离。使用它们为人类维护者留下笔记，而不在注释上花费上下文 token。代码块内的注释被保留。当您直接使用 Read 工具打开 CLAUDE.md 文件时，注释保持可见。

#### 从其他目录加载

`--add-dir` 标志使 Claude 能够访问主工作目录外的其他目录。默认情况下，这些目录中的 CLAUDE.md 文件不加载。

要也从其他目录加载记忆文件，请设置 `CLAUDE_CODE_ADDITIONAL_DIRECTORIES_CLAUDE_MD` 环境变量：

```bash theme={null}
CLAUDE_CODE_ADDITIONAL_DIRECTORIES_CLAUDE_MD=1 claude --add-dir ../shared-config
```

内联形式为该单次启动在 Bash 或 Zsh 中设置变量。要为每个会话保持它，请将其添加到 `~/.claude/settings.json` 中的 `env` 块，如 [设置环境变量](https://code.claude.com/docs/zh-CN/env-vars.md#set-environment-variables) 中所示。

这从其他目录加载 `CLAUDE.md`、`.claude/CLAUDE.md`、`.claude/rules/*.md` 和 `CLAUDE.local.md`。如果您从 [`--setting-sources`](https://code.claude.com/docs/zh-CN/cli-reference) 中排除 `local`，则跳过 `CLAUDE.local.md`。

### 使用 `.claude/rules/` 组织规则

对于较大的项目，您可以使用 `.claude/rules/` 目录将指令组织到多个文件中。这使指令模块化并更容易让团队维护。规则也可以 [范围限定到特定文件路径](#path-specific-rules)，因此它们仅在 Claude 处理匹配文件时加载到上下文中，减少噪音并节省上下文空间。

> [!NOTE]
>
> 规则在每个会话或打开匹配文件时加载到上下文中。对于不需要始终在上下文中的特定任务指令，请改用 [skills](https://code.claude.com/docs/zh-CN/skills.md)，它仅在您调用它们或 Claude 确定它们与您的提示词相关时加载。

#### 设置规则

在项目的 `.claude/rules/` 目录中放置 markdown 文件。每个文件应涵盖一个主题，具有描述性文件名，如 `testing.md` 或 `api-design.md`。所有 `.md` 文件被递归发现，因此您可以将规则组织到子目录中，如 `frontend/` 或 `backend/`：

```text theme={null}
your-project/
├── .claude/
│   ├── CLAUDE.md           # Main project instructions
│   └── rules/
│       ├── code-style.md   # Code style guidelines
│       ├── testing.md      # Testing conventions
│       └── security.md     # Security requirements
```

没有 [`paths` frontmatter](#path-specific-rules) 的规则在启动时加载，优先级与 `.claude/CLAUDE.md` 相同。

如果您从 [`--setting-sources`](https://code.claude.com/docs/zh-CN/cli-reference) 中排除 `project`，项目规则被跳过。在 v2.1.211 之前，加载按需的规则，包括路径范围规则和嵌套 `.claude/rules/` 目录中的规则，即使 `project` 被排除也会加载。

#### 路径特定规则

规则可以使用带有 `paths` 字段的 YAML frontmatter 范围限定到特定文件。这些条件规则仅在 Claude 处理与指定模式匹配的文件时适用。

```markdown theme={null}
---
paths:
  - "src/api/**/*.ts"
---

# API Development Rules

- All API endpoints must include input validation
- Use the standard error response format
- Include OpenAPI documentation comments
```

没有 `paths` 字段的规则无条件加载并适用于所有文件。当 Claude 对匹配的文件使用 Read、Write 或 Edit 工具时，路径范围规则会加载。当 Claude 使用[算作读取](https://code.claude.com/docs/zh-CN/tools-reference.md#edit-tool-behavior)的 Bash 命令查看匹配的文件时（例如对单个文件使用 `cat` 或 `head`），该规则也会加载。当 Claude 通过项目目录的符号链接路径到达文件时，匹配也有效，例如在符号链接检出中。

每个模式都相对于规则的基准目录：

* **项目规则**：包含该规则所在 `.claude` 文件夹的目录
* **[用户级规则](#user-level-rules)**：会话的[主工作目录](https://code.claude.com/docs/zh-CN/permissions.md#working-directories)

要让路径范围规则为同级 git worktree 中的文件加载，请在该 worktree 中启动 Claude Code，或[使用 `/cd` 将会话移至该处](https://code.claude.com/docs/zh-CN/permissions#move-the-session-to-another-directory)。

在 `paths` 字段中使用 glob 模式按扩展名、目录或任何组合匹配文件：

| 模式 | 匹配 |
| - | - |
| `**/*.ts` | 任何目录中的所有 TypeScript 文件 |
| `src/**/*` | `src/` 目录下的所有文件 |
| `*.md` | 项目根目录中的 Markdown 文件 |
| `src/components/*.tsx` | 特定目录中的 React 组件 |

您可以指定多个模式并使用大括号扩展在一个模式中匹配多个扩展名：

```markdown theme={null}
---
paths:
  - "src/**/*.{ts,tsx}"
  - "lib/**/*.ts"
  - "tests/**/*.test.ts"
---
```

每个大括号组将扩展的模式数量相乘：`src/*.{ts,tsx}` 扩展为两个模式，`{a,b}/{c,d}/*.{ts,tsx}` 扩展为八个。为了保持扩展有界，规则的整个 `paths` 列表共享一个 1,000 个扩展模式和 4 MiB 的预算，没有大括号的模式不计入其中。

Claude Code 使用任何会超过预算的未展开模式，其字面大括号不匹配任何文件。

Glob 语法将 `[` 视为括号表达式的开始，例如 `[abc]`。具有无法读作括号表达式的 `[` 的模式，例如 `photos [2024/**`，是无效的：它不匹配任何内容，规则的其他模式继续工作。要匹配文件名中的字面 `[`，请将其转义为 `photos \[2024/**`。

#### 规则 frontmatter 参考

使用 YAML [frontmatter](https://code.claude.com/docs/zh-CN/glossary.md#frontmatter) 在文件顶部的 `---` 标记之间配置规则。`paths` 是 Claude Code 从规则中读取的唯一字段；任何其他字段都被忽略而不出现错误。Claude Code 在将规则加载到上下文之前删除 frontmatter。

| 字段 | 必需 | 描述 |
| :- | :- | :- |
| `paths` | 否 | Glob 模式，[将规则范围限定到匹配文件](#path-specific-rules)。接受 YAML 列表或逗号分隔的字符串 |

如果标记之间的 YAML 不解析，Claude Code 忽略 frontmatter 并加载规则，就像它没有 `paths` 一样。运行 `claude --debug` 查看解析错误。

#### 使用符号链接在项目间共享规则

`.claude/rules/` 目录支持符号链接，因此您可以维护一组共享规则并将它们链接到多个项目中。循环符号链接被检测并妥善处理。

Claude Code 将其目标在工作目录外的符号链接视为 [external import](#import-additional-files)。链接的规则不加载，直到您批准项目的外部导入，之后仅加载没有 [`paths` 字段](#path-specific-rules) 的规则。

Claude Code 会针对每个项目请求一次该批准，方式是在交互式会话开始时显示一个对话框。该对话框会列出链接的规则文件以及任何外部 `@path` 导入。要加载共享规则而不需要该批准，请将它们保存在 [`~/.claude/rules/`](#user-level-rules) 中，它们适用于您机器上的每个项目。

此示例链接共享目录和单个文件：

```bash theme={null}
ln -s ~/shared-claude-rules .claude/rules/shared
ln -s ~/company-standards/security.md .claude/rules/security.md
```

如果您指向网络路径（如 UNC 共享 `\\server\share` 或 `/net` 或 `/Network` 下的路径）的 `.claude/rules/` 或 `CLAUDE.md` 符号链接，链接的指令不加载。Claude Code 不跟随链接，因为查找此类路径可能会联系它命名的主机。`\\wsl$` 路径不计为网络路径。

#### 用户级规则

`~/.claude/rules/` 中的个人规则适用于您机器上的每个项目。使用它们来处理不是项目特定的偏好：

```text theme={null}
~/.claude/rules/
├── preferences.md    # Your personal coding preferences
└── workflows.md      # Your preferred workflows
```

Claude Code 在项目规则之前加载用户级规则，因此项目规则在 Claude 的上下文中出现在用户规则之后。两个集合都不会覆盖另一个：如果用户规则和项目规则冲突，Claude 可能会遵循任一个，因此保持两者一致。

### 为大型团队管理 CLAUDE.md

对于在团队中部署 Claude Code 的组织，您可以集中指令并控制加载哪些 CLAUDE.md 文件。

#### 部署组织范围的 CLAUDE.md

组织可以部署一个集中管理的 CLAUDE.md，适用于机器上的所有用户。开发者可以使用 `claudeMdExcludes` 设置[排除其他 CLAUDE.md 文件](#exclude-specific-claude-md-files)，但无法排除此文件。如果开发者在完全关闭 CLAUDE.md 加载的情况下启动会话，该会话也不会加载托管内容。请将此文件用于指导，并使用 `permissions.deny` 等[托管设置](https://code.claude.com/docs/zh-CN/managed-settings.md)来强制执行规则。

**1. 在托管策略位置创建文件**

* macOS: `/Library/Application Support/ClaudeCode/CLAUDE.md`
* Linux 和 WSL: `/etc/claude-code/CLAUDE.md`
* Windows: `C:\Program Files\ClaudeCode\CLAUDE.md`

**2. 使用您的配置管理系统部署**

使用 MDM、Group Policy、Ansible 或类似工具在开发者机器中分发文件。有关其他组织范围配置选项，请参阅 [managed settings](https://code.claude.com/docs/zh-CN/managed-settings.md)。

`claudeMd` 键允许您将托管 CLAUDE.md 内容直接放入 `managed-settings.json` 中，而不是部署单独的文件。

**作用域**：机器上的所有用户，在任何仓库中。对于仓库特定的指导，改为提交项目 CLAUDE.md。

**优先级**：与托管 CLAUDE.md 文件相同。在用户和项目 CLAUDE.md 之前加载。

**在哪里被遵守**：仅托管和策略设置。在用户、项目或本地设置中设置 `claudeMd` 无效。

下面的示例直接在托管设置文件中添加行为指令：

```json theme={null}
{
  "claudeMd": "Always run `make lint` before committing.\nNever push directly to main."
}
```

托管 CLAUDE.md 和 [managed settings](https://code.claude.com/docs/zh-CN/managed-settings.md) 服务于不同的目的。使用设置进行技术强制，使用 CLAUDE.md 进行行为指导：

| 关注点 | 配置在 |
| :- | :- |
| 阻止特定工具、命令或文件路径 | 托管设置：`permissions.deny` |
| 强制沙箱隔离 | 托管设置：`sandbox.enabled` |
| 环境变量和 API 提供商路由 | 托管设置：`env` |
| 登录方法和组织限制 | 托管设置：`forceLoginMethod`、`forceLoginOrgUUID` |
| 代码样式和质量指南 | 托管 CLAUDE.md |
| 数据处理和合规提醒 | 托管 CLAUDE.md |
| Claude 的行为指令 | 托管 CLAUDE.md |

设置规则由客户端强制执行，无论 Claude 决定做什么。CLAUDE.md 指令塑造 Claude 的行为，但不是硬强制层。

#### 排除特定的 CLAUDE.md 文件

在大型 monorepos 中，祖先 CLAUDE.md 文件可能包含与您的工作无关的指令。`claudeMdExcludes` 设置允许您按路径或 glob 模式跳过特定文件。您无法使用 `claudeMdExcludes` 排除[托管 CLAUDE.md 文件](#deploy-organization-wide-claude-md)。

此示例排除顶级 CLAUDE.md 和来自父文件夹的规则目录。将其添加到 `.claude/settings.local.json` 以使排除保持本地到您的机器：

```json theme={null}
{
  "claudeMdExcludes": [
    "**/monorepo/CLAUDE.md",
    "/home/user/monorepo/other-team/.claude/rules/**"
  ]
}
```

模式使用 glob 语法与绝对文件路径匹配。您可以在任何 [settings layer](https://code.claude.com/docs/zh-CN/settings.md#where-settings-live) 配置 `claudeMdExcludes`：用户、项目、本地或托管策略。数组跨层合并。

要排除您通过 [symlink](#share-rules-across-projects-with-symlinks) 到达的规则文件，无论文件还是其目录是链接，请针对任一路径编写模式：文件在 `.claude/rules/` 下的路径或其链接目标。匹配任一路径的模式排除文件。在 v2.1.239 之前，仅匹配链接目标的模式排除文件。

## AGENTS.md

Claude Code 可以将 [`AGENTS.md`](https://code.claude.com/docs/zh-CN/glossary#agents-md) 作为您的项目说明读取，因此已为其他编码代理设置的存储库无需添加 `CLAUDE.md`、导入或设置即可工作。此表显示了存储库中指令文件的每种组合下 Claude 默认读取的内容：

| 您的存储库有 | Claude 读取 |
| :- | :- |
| 一个 `AGENTS.md`，且在您的工作目录或其上方没有 `CLAUDE.md` 或 `CLAUDE.local.md` | 您的 `AGENTS.md` |
| 一个 `AGENTS.md` 和一个 `CLAUDE.md` 或 `CLAUDE.local.md` 在您的工作目录或其上方 | 仅您的 `CLAUDE.md` 文件 |
| 一个已[导入 `AGENTS.md`](#share-one-file-with-other-coding-tools)的 `CLAUDE.md` | 您的 `CLAUDE.md`，通过导入包含 `AGENTS.md` |

要更改默认值，例如让 Claude 始终读取两个文件、仅读取 `CLAUDE.md` 或仅读取您组织的托管说明，请[更改**项目说明**设置](#choose-which-instruction-files-load)。

> [!NOTE]
>
> 直接读取 `AGENTS.md` 需要 Claude Code v2.1.277 或更高版本。在某些会话中 Claude [无法读取 `AGENTS.md`](#when-agents-md-support-is-unavailable)，因此请[从 `CLAUDE.md` 中导入它](#share-one-file-with-other-coding-tools)。

### Claude Code 何时读取 AGENTS.md

默认情况下，Claude 仅在您的工作目录或其上方没有 `CLAUDE.md` 时才读取 `AGENTS.md`。以下是此检查中计数的文件：

* **计数，因此 Claude 读取它们而不是 `AGENTS.md`**：您的工作目录或其上方任何目录中的 `CLAUDE.md`、`.claude/CLAUDE.md` 或 `CLAUDE.local.md`
* **不计数，并继续与 `AGENTS.md` 一起加载**：您的 `~/.claude/CLAUDE.md`、您组织的托管 `CLAUDE.md` 和 `.claude/rules/` 文件

当没有计数时，以下是 Claude 读取的内容以及您如何判断：

* **在会话开始时**：您的工作目录和其上方目录中的每个 `AGENTS.md` 和 `.claude/AGENTS.md`。在交互式会话中，您会看到一行，例如 `no CLAUDE.md found; AGENTS.md loaded: /home/you/repo/AGENTS.md` 在对话中
* **当 Claude 在子目录中工作时**：当 Claude 使用 Read 工具打开该处的文件且该子目录没有三个 `CLAUDE.md` 文件中的任何一个时，子目录的 `AGENTS.md`
* **在每个 `AGENTS.md` 内**：[`@path` 导入](#import-additional-files)被展开，[`claudeMdExcludes`](#exclude-specific-claude-md-files) 模式适用，[启动时不带项目说明](https://code.claude.com/docs/zh-CN/sub-agents.md#what-loads-at-startup)的子代理启动时同样不带这些文件
* **不读取**：`AGENTS.local.md`、`AGENTS.override.md` 或 `.agents/` 目录下的任何内容

> [!NOTE]
>
> 因为 `CLAUDE.local.md` 计数，在依赖 `AGENTS.md` 的项目中添加一个来保留您自己的未提交说明会停止 Claude 为您读取 `AGENTS.md`。要保留您的 `CLAUDE.local.md` 并仍然让 Claude 读取 `AGENTS.md`，请将**项目说明**设置为 [`claude-md-and-agents-md`](#choose-which-instruction-files-load)。

### 选择加载哪些说明文件

要更改 Claude 读取的文件，请在 Claude Code 会话中键入 `/config` 以打开设置面板，然后将**项目说明**设置为以下值之一：

| 值 | Claude 读取的内容 |
| :- | :- |
| `claude-md-or-agents-md` | 您的 `CLAUDE.md` 文件，或当您的工作目录或其上方没有 `CLAUDE.md` 或 `CLAUDE.local.md` 时您的 `AGENTS.md` 文件。这是默认值 |
| `claude-md-and-agents-md` | 您的 `CLAUDE.md` 和 `AGENTS.md` 文件一起，每个目录的 `CLAUDE.md` 文件首先，其 `AGENTS.md` 在之后。Claude Code 跳过已加载的 `AGENTS.md`，因此您的 `CLAUDE.md` 导入或符号链接到的 `AGENTS.md` 不会被读取两次 |
| `claude-md` | 仅您的 `CLAUDE.md` 文件 |
| `managed-only` | 仅您组织的托管 `CLAUDE.md` 和启动时的[自动记忆](#auto-memory)。您的项目、本地和用户 `CLAUDE.md` 文件、您的 `.claude/rules/` 文件和每个 `AGENTS.md` 都被排除在外。子目录的 `CLAUDE.md` 和 `.claude/rules/` 文件以及[路径范围规则](#path-specific-rules)仍会按需加载 |

您也可以在设置文件中设置该值，而不是在 `/config` 中。将其添加到 [`pluginConfigs`](https://code.claude.com/docs/zh-CN/settings-reference#pluginconfigs) 中的 `cc-plugin-agents-md@builtin` 下，这是读取 `AGENTS.md` 的内置插件的 ID。Claude Code 从 `~/.claude/settings.json`、`--settings` 文件或[托管设置](https://code.claude.com/docs/zh-CN/managed-settings.md)中读取该条目，并在项目和本地设置文件中忽略它。此示例让 Claude 读取两个文件：

```json settings.json theme={null}
{
  "pluginConfigs": {
    "cc-plugin-agents-md@builtin": {
      "options": { "instructionFiles": "claude-md-and-agents-md" }
    }
  }
}
```

在 v2.1.285 之前，该插件的 ID 是 `agents-md@builtin`，并且 Claude Code 会忽略 `cc-plugin-agents-md@builtin` 下的条目。如果早期版本也会读取您的设置文件，请在其中使用 `agents-md@builtin`。Claude Code v2.1.285 及更高版本会读取任一 ID 下的条目。

您的更改从您发送的下一条消息和每个新会话中应用。

### 当 AGENTS.md 支持不可用时

在这些会话中，Claude 仅读取 `CLAUDE.md` 文件，**项目说明**不会出现在 `/config` 设置面板中：

* 您使用的是 v2.1.277 之前的 Claude Code 版本
* 您使用 `/plugin` 禁用了读取 `AGENTS.md` 的内置插件
* 在某些情况下，这是您[从 v2.1.276 或更早版本升级](https://code.claude.com/docs/zh-CN/env-vars.md#first-session-after-an-install-or-upgrade)后的第一个会话。Claude 从您的下一个会话开始读取 `AGENTS.md`

在 v2.1.281 之前，某些会话，例如在 Amazon Bedrock 上或禁用遥测的会话，仅读取 `CLAUDE.md` 文件。在这些版本上，更新 Claude Code。要在这些会话中向 Claude 提供您的 `AGENTS.md`，请[从 `CLAUDE.md` 中导入它](#share-one-file-with-other-coding-tools)。

### AGENTS.md 与 CLAUDE.md 的区别

通过**项目说明**设置读取的 `AGENTS.md` 与 `CLAUDE.md` 在以下方面有所不同：

| | `CLAUDE.md` | 通过设置读取的 `AGENTS.md` |
| :- | :- | :- |
| [`InstructionsLoaded` hooks](https://code.claude.com/docs/zh-CN/hooks#instructionsloaded) | 触发 | 不触发。它们照常为 `CLAUDE.md` 导入或符号链接到的 `AGENTS.md` 触发 |
| 当设置了 [`CLAUDE_CODE_ADDITIONAL_DIRECTORIES_CLAUDE_MD`](#load-from-additional-directories) 时，您使用 `--add-dir` 添加的目录 | 它们的 `CLAUDE.md` 加载 | 它们的 `AGENTS.md` 不加载 |
| `@path` 导入工作目录外的文件 | Claude Code 要求您批准[外部导入](#import-additional-files) | 仅在您已为此项目批准外部导入时加载，无提示 |

### 删除早期的 AGENTS.md 解决方案

如果您在 Claude Code 自行读取 `AGENTS.md` 之前设置它来读取 `AGENTS.md`，以下是对每个常见设置的处理方法：

* **包含 `@AGENTS.md` 的 `CLAUDE.md`**：您可以保留它。保留导入永远不会让 Claude 读取 `AGENTS.md` 两次，无论您使用哪个**项目说明**值。如果 `CLAUDE.md` 不包含其他内容，请删除它，或如果您的某些会话[无法直接加载 `AGENTS.md`](#when-agents-md-support-is-unavailable)，请保留它。
* **告诉 Claude 用词语读取 `AGENTS.md` 的 `CLAUDE.md`**：仅当 Claude 决定打开文件时，它才会看到 `AGENTS.md`。删除 `CLAUDE.md` 以便 Claude 直接读取 `AGENTS.md`，或用 `@AGENTS.md` 导入替换该句子。
* **符号链接到 `AGENTS.md` 的 `CLAUDE.md`**：无，或删除符号链接。无论哪种方式，Claude 都会读取内容一次。
* **打印 `AGENTS.md` 的 `SessionStart` hook**：删除它。一旦 Claude 直接读取 `AGENTS.md`，该 hook 会向上下文添加第二个副本。

### 与其他编码工具共享一个文件

当 Claude 不直接读取您的 `AGENTS.md` 时，您仍然可以通过在其旁边的 `CLAUDE.md` 中放置 `@AGENTS.md` 导入来将其保留为每个工具共享的一个文件。当您的项目也有 `CLAUDE.md` 时、当您已将**项目说明**设置为 `claude-md` 时，或在[无法加载 `AGENTS.md`](#when-agents-md-support-is-unavailable)的会话中执行此操作。在导入下方添加任何 Claude 特定的说明，Claude 会先读取导入的文件，然后读取其余部分：

```markdown CLAUDE.md theme={null}
@AGENTS.md

## Claude Code

Use plan mode for changes under `src/billing/`.
```

如果您不需要 Claude 特定的内容，符号链接也可以工作：

```bash theme={null}
ln -s AGENTS.md CLAUDE.md
```

该命令在成功时不打印任何输出。在选择符号链接而不是导入之前，请检查这些约束：

* **编辑**：Claude 通过链接读取 `CLAUDE.md`，但 Edit 和 Write 工具[拒绝通过符号链接写入](https://code.claude.com/docs/zh-CN/errors.md#refusing-after-a-symlink-changed)，拒绝指示 Claude 编辑链接的目标 `AGENTS.md`
* **Windows**：如果您或克隆存储库的任何人在 Windows 上工作，请改用 `@AGENTS.md` 导入。在那里创建符号链接需要管理员权限或开发人员模式，Git 会将提交的符号链接检出为纯文本文件，除非启用了 `core.symlinks`，这会使该克隆具有一行 `CLAUDE.md` 代替您的说明

使用任一方法，在您的下一个会话中运行 `/context` 并确认 `CLAUDE.md` 出现在**内存文件**下。

### 从其他工具迁移说明

运行 [`/init`](https://code.claude.com/docs/zh-CN/commands) 会读取其他工具的说明文件并将相关部分合并到生成的 `CLAUDE.md` 中：

* `.cursor/rules/` 或 `.cursorrules` 中的 Cursor 规则
* `.github/copilot-instructions.md` 中的 Copilot 规则
* 设置 `CLAUDE_CODE_NEW_INIT=1` 时：`AGENTS.md`、`.devin/rules/`、`.windsurf/rules/` 或 `.windsurfrules`，以及 `.clinerules`

您也可以运行 [`/import`](https://code.claude.com/docs/zh-CN/commands) 将支持的编码 Agent 的配置引入 Claude Code，这会将 `AGENTS.md` 等说明文件的一次性副本追加到匹配的 `CLAUDE.md`，并携带 MCP 服务器、命令、子代理和 skill。

## 自动记忆

自动记忆让 Claude 跨会话积累知识，无需你编写任何内容。在工作时，Claude 为自己保存四种笔记。Claude 在记忆文件的 frontmatter 中用 `type` 字段记录类型：

* `user`：你的角色、专业知识和工作偏好
* `feedback`：你给 Claude 的更正和你确认的方法
* `project`：正在进行的工作、截止日期和 Claude 无法从代码或 git 历史推导的决策
* `reference`：在项目外查找信息的位置，例如问题跟踪器或仪表板

Claude 跳过任何它可以从代码库推导的内容，例如架构、文件路径或调试修复。它也跳过你的 CLAUDE.md 文件已经说过的任何内容。

Claude 不会每个会话都保存内容。它根据信息在未来对话中是否有用来决定什么值得记住。

### 启用或禁用自动记忆

自动记忆在本地会话中默认开启。在 [Claude Tag](https://claude.com/docs/claude-tag/overview) 会话之外，[自托管环境](https://code.claude.com/docs/zh-CN/self-hosted-environments-configuration.md#how-each-session’s-config-is-assembled)中的会话默认关闭自动记忆。

要切换它，在会话中打开 `/memory` 并使用自动记忆切换，它将 `autoMemoryEnabled` 保存到您的用户设置 `~/.claude/settings.json`。

在以下会话中，该切换可以关闭自动记忆，但无法将其重新开启：

* [后台会话](https://code.claude.com/docs/zh-CN/agent-view.md)
* 由另一个 Claude Code 会话启动的会话，例如 Claude 通过其 Bash 工具运行 `claude` 时

在这些会话中自动记忆关闭时，切换显示为 `off · can't be turned on here; use a session started outside Claude Code`。要重新开启自动记忆，请直接在终端中运行 `claude`，并在该会话中使用 `/memory` 切换。

要为单个项目关闭它，在该项目的设置中设置 `autoMemoryEnabled`：

```json theme={null}
{
  "autoMemoryEnabled": false
}
```

要通过环境变量禁用自动记忆，设置 `CLAUDE_CODE_DISABLE_AUTO_MEMORY=1`。

### 存储位置

每个项目在 `~/.claude/projects/<project>/memory/` 获得自己的记忆目录。`<project>` 路径来自 git 存储库，因此同一存储库中的所有 worktrees 和子目录共享一个自动记忆目录。在 git 存储库外，改用项目根目录。

如果你在 `CLAUDE_CONFIG_DIR` 旁边设置 [`CLAUDE_CODE_PROJECT_DIR_NAME`](https://code.claude.com/docs/zh-CN/sessions#name-the-project-directory-yourself)，Claude Code 会使用该名称作为 `<config dir>/projects/` 下的 `<project>` 目录，无论你在哪个存储库中启动它，因此使用该配置目录启动的项目共享一个自动记忆目录。需要 Claude Code v2.1.234 或更高版本。

要将自动记忆存储在不同位置，在你的 `settings.json` 中设置 `autoMemoryDirectory`。它从任何[设置范围](https://code.claude.com/docs/zh-CN/settings.md#settings-precedence)读取：用户、项目、本地、策略或 `--settings`。

```json theme={null}
{
  "autoMemoryDirectory": "~/my-custom-memory-dir"
}
```

该值必须是绝对路径或以 `~/` 开头。

当在项目的 `.claude/settings.json` 或 `.claude/settings.local.json` 中设置时，Claude Code 在与[设置文件中的 hooks 相同的工作区信任规则](https://code.claude.com/docs/zh-CN/permissions.md#what-runs-before-you-trust-a-folder)下遵守它。当 [`permissions.blockReadsOutsideWorkingDirectories`](https://code.claude.com/docs/zh-CN/settings-reference#permissions-blockreadsoutsideworkingdirectories) 开启时，Claude Code 不从[存储库提供的设置文件](https://code.claude.com/docs/zh-CN/permissions.md#when-your-local-settings-file-needs-trust)选择的目录加载自动记忆，也不向其保存任何内容，无论该目录位于何处。

目录包含一个 `MEMORY.md` 索引和每个记忆一个主题文件：

```text theme={null}
~/.claude/projects/<project>/memory/
├── MEMORY.md           # Index, one line per memory, loaded into every session
├── user_role.md        # One memory
├── feedback_testing.md # One memory
└── ...                 # Any other topic files Claude creates
```

`MEMORY.md` 充当记忆目录的索引。Claude 在你的会话中读取和写入此目录中的文件，使用 `MEMORY.md` 跟踪存储的内容。

自动记忆是机器本地的。同一 git 存储库中的所有 worktrees 和子目录共享一个自动记忆目录。文件不在机器或云环境之间共享。

Claude Code 在 [`cleanupPeriodDays`](https://code.claude.com/docs/zh-CN/settings-reference#cleanupperioddays) 保留期后删除旧会话记录，但从该[保留清理](https://code.claude.com/docs/zh-CN/claude-directory.md#cleaned-up-automatically)中排除记忆目录中的记忆文件。`MEMORY.md` 和主题文件保留到你或 Claude 编辑或删除它们。

### 它如何工作

`MEMORY.md` 的前 200 行或前 25KB（以先到者为准）在每次对话开始时加载。超过该阈值的内容在会话开始时不加载。Claude 通过将详细笔记移到单独的主题文件中来保持 `MEMORY.md` 简洁。

Claude 写入 `MEMORY.md` 后，Claude Code 根据 200 行和 25KB 读取限制测量文件。如果文件接近限制，Claude Code 提醒 Claude 缩短它：每个条目保留一行，将详细信息移到主题文件，并合并或删除陈旧条目。如果文件超过限制，写入仍然成功，但 Claude Code 返回一个[错误告诉 Claude 重写索引](https://code.claude.com/docs/zh-CN/errors.md#memory-index-is-over-its-read-limit)，因为下次加载时超过限制的所有内容都会被丢弃。

> [!NOTE]
>
> 此限制仅适用于 `MEMORY.md`。对于启动时加载的每个 CLAUDE.md 文件，只要文件不超过 4 MiB，Claude 就会获得其完整内容。Claude Code 会跳过超过 4 MiB 的文件，不加载其中任何内容。路径限定的规则和子目录中的 CLAUDE.md 文件有[各自更小的单文件限制](#claude-ignores-the-end-of-my-subdirectory-claude-md)。

Claude Code 在启动时不加载主题文件，例如 `user_role.md` 或 `feedback_testing.md`。Claude 在需要信息时使用其标准文件工具按需读取它们。

主对话的自动记忆不加载到[子代理](https://code.claude.com/docs/zh-CN/sub-agents.md#what-loads-at-startup)中；例外是[分叉](https://code.claude.com/docs/zh-CN/sub-agents.md#fork-the-current-conversation)，它继承父对话和系统提示。子代理自己的自动记忆（通过子代理 `memory` 字段启用）是一个单独的目录。

Claude 在你的会话中读取和写入记忆文件。当你在 Claude Code 界面中看到"Saved 2 memories"或"Recalled 2 memories"之类的消息时，Claude 正在主动更新或读取 `~/.claude/projects/<project>/memory/`。

当 Claude 写入以 YAML frontmatter 开头的记忆文件时，Claude Code 在 `modified` frontmatter 字段中记录写入时间为 ISO 8601 时间戳。时间戳显示事实有多新，对您和 Claude 读取记忆时都有用。

### 审计和编辑你的记忆

自动记忆文件是纯 markdown，你可以随时编辑或删除。运行 [`/memory`](#view-and-edit-with-%2Fmemory) 从会话中浏览和打开记忆文件。

## 使用 `/memory` 查看和编辑

`/memory` 命令列出您的 CLAUDE.md、CLAUDE.local.md 和其他记忆文件在用户和项目作用域内的位置，包括尚不存在的文件的用户和项目 CLAUDE.md 条目。它还允许您开启或关闭自动记忆，并提供打开自动记忆文件夹的选项。选择任何文件即可在您的编辑器中打开它；选择一个尚不存在的文件会先创建它。要检查启动时加载了哪些 `CLAUDE.md` 和规则文件，请运行 `/context`。

VS Code 等 GUI 编辑器在单独的窗口中打开文件，您可以在文件打开时继续使用会话。Vim 等终端编辑器会接管终端，直到您退出。

当你要求 Claude 记住某些内容时，如"总是使用 pnpm，而不是 npm"或"记住 API 测试需要本地 Redis 实例"，Claude 将其保存到自动记忆。要改为添加指令到 CLAUDE.md，直接要求 Claude，如"将其添加到 CLAUDE.md"，或通过 `/memory` 自己编辑文件。

## 故障排除记忆问题

这些是 CLAUDE.md 和自动记忆最常见的问题，以及调试步骤。

### Claude 不遵循我的 CLAUDE.md

CLAUDE.md 内容作为用户消息在系统提示词之后传递，而不是系统提示词本身的一部分。Claude 读取它并尝试遵循它，但没有严格遵守的保证，特别是对于模糊或冲突的指令。

要调试：

* 运行 `/context` 并检查 **Memory files** 下的列表，以验证应在启动时加载的 CLAUDE.md 和 CLAUDE.local.md 文件。如果其中某个文件不在列表中，Claude 就看不到它。使用 `/memory` 打开和编辑文件。
* 如果出现 `Instruction file not loaded` 提示，请参阅[其含义及处理方法](https://code.claude.com/docs/zh-CN/errors.md#instruction-file-not-loaded)。
* 工作目录的子目录中的 `CLAUDE.md` 不会出现在 **Memory files** 下，因为它是按需加载而不是在启动时加载的。加载时，终端中会出现一行带有其路径的 `Loaded`。要测试新建的此类文件，请从 shell 中创建它，而不是让 Claude 编写它，然后让 Claude 读取该子目录中的某个文件。
* 检查相关 CLAUDE.md 是否在为您的会话加载的位置（参见 [选择 CLAUDE.md 文件的位置](#choose-where-to-put-claude-md-files)）。
* 使指令更具体。"使用 2 空格缩进"比"格式化代码很好"效果更好。
* 查找跨 CLAUDE.md 文件的冲突指令。如果两个文件为相同行为提供不同的指导，Claude 可能会任意选择一个。
* 检查您的指令是否与 Claude Code 自身添加的指导相竞争。如果您的 CLAUDE.md 设置了提交或 Pull Request 规则，请使用 [`includeGitInstructions`](https://code.claude.com/docs/zh-CN/settings-reference#includegitinstructions) 关闭内置规则，并使用 [`attribution`](https://code.claude.com/docs/zh-CN/settings-reference#attribution) 设置归属文本。

如果指令是必须在特定点运行的内容，例如在每次提交之前或每次文件编辑之后，请将其写成 [hook](https://code.claude.com/docs/zh-CN/hooks-guide.md) 代替。hook 在固定的生命周期事件处作为 shell 命令执行，并且无论 Claude 决定做什么都适用。

对于希望处于系统提示词级别的指令，使用 [`--append-system-prompt`](https://code.claude.com/docs/zh-CN/cli-reference#system-prompt-flags)。它在启动时传递，因此更适合脚本和自动化而不是交互式使用。有关它在恢复对话时的行为，请参见 [恢复对话中的系统提示词标志](https://code.claude.com/docs/zh-CN/cli-reference.md#system-prompt-flags-in-resumed-conversations)。

> [!TIP]
>
> 使用 [`InstructionsLoaded` hook](https://code.claude.com/docs/zh-CN/hooks#instructionsloaded) 记录加载了哪些 `CLAUDE.md` 和规则文件、何时加载以及为什么。这对于调试特定路径规则或子目录中的延迟加载文件很有用。

### 我的 AGENTS.md 未加载

如果您的仓库有 `AGENTS.md` 而 Claude 似乎不知道它说什么，通常原因是项目路径上某处有 `CLAUDE.md`。默认情况下，Claude 仅在您的工作目录或其上方没有 `CLAUDE.md` 或 `CLAUDE.local.md` 时读取 `AGENTS.md`。按顺序检查这些：

1. 在您的工作目录或其上方的任何目录中查找 `CLAUDE.md`、`.claude/CLAUDE.md` 或 `CLAUDE.local.md`，除了您的 `~/.claude/CLAUDE.md`。如果找到一个，Claude 会读取它而不是 `AGENTS.md`，除非您将 **Project instructions** 设置为 `claude-md-and-agents-md`。
2. 运行 `claude --version` 并确认 v2.1.277 或更高版本。在 v2.1.281 之前，某些会话，例如 Amazon Bedrock 上的会话或禁用遥测的会话，也[无法加载 `AGENTS.md`](#when-agents-md-support-is-unavailable)，因此在这些版本上请更新到 v2.1.281 或更高版本。
3. 在会话中输入 `/config` 以打开设置面板，并确认 **Project instructions** 未设置为 `claude-md` 或 `managed-only`。如果根本看不到该设置，则您的会话属于[无法加载 `AGENTS.md`](#when-agents-md-support-is-unavailable) 的会话。

要检查 Claude 是否读取了您的 `AGENTS.md`，运行 `/memory` 并在列表中查找其路径。

在 v2.1.280 之前，`/memory` 和 `/context` 没有列出 Claude 直接读取的 `AGENTS.md`。在这些版本上，改为询问 Claude 其项目指令说什么。

如果您想保留找到的 `CLAUDE.md`，或您的会话无法加载 `AGENTS.md`，请[在您的 `AGENTS.md` 旁边添加一个导入它的 `CLAUDE.md`](#share-one-file-with-other-coding-tools)。

### 我不知道自动记忆保存了什么

运行 `/memory` 并选择自动记忆文件夹来浏览 Claude 保存的内容。一切都是纯 markdown，您可以读取、编辑或删除。

### 我的 CLAUDE.md 太大了

超过 200 行的文件消耗更多上下文并可能降低遵守度。Claude Code 会跳过超过 4 MiB 的文件。使用 [path-scoped rules](#path-specific-rules) 仅在 Claude 处理匹配文件时加载指令，或修剪不是每个会话都需要的内容。分割到 [`@path` imports](#import-additional-files) 有助于组织，但不会减少上下文，因为导入的文件会与导入它们的 CLAUDE.md 一起加载。

如果您的某个指令文件超过推荐长度，您会在启动时和运行 `/status` 时看到警告。当各自都在该长度内的文件在会话开始时加起来超过组合限制时，您也会看到警告。每个 CLAUDE.md、规则文件和 `@path` 导入都计为单独的文件。

[`/doctor`](https://code.claude.com/docs/zh-CN/commands#all-commands) 检查为已检入的 CLAUDE.md 提议修剪：它删除 Claude 可以从代码库派生的内容，例如目录布局、依赖项列表和架构概览，并保留与工具默认值不同的陷阱、基本原理和约定。

> [!NOTE]
>
> 请将每个路径范围规则文件控制在[其单文件大小限制](#claude-ignores-the-end-of-my-subdirectory-claude-md)之内，因为对于超过该限制的文件，Claude 只会收到文件的开头部分。

### Claude 忽略了我的子目录 CLAUDE.md 的末尾部分

当子目录中的 `CLAUDE.md` 或按需加载的规则文件超过单文件大小限制时，Claude 只会收到其开头部分。该部分末尾附有一条说明，告诉 Claude 使用 Read 工具读取其余内容；在 Claude 选择这样做之前，后面的指令不会出现在其上下文中。

检查您的文件是否超过限制，如果超过则缩减它：

1. 确认该限制适用于您的文件。该限制适用于以下按需加载的文件：
   * 您启动 Claude Code 所在目录的子目录中的 `CLAUDE.md` 和 `CLAUDE.local.md`，以及它们通过 `@path` [导入](#import-additional-files)的文件
   * 子目录的 `.claude/rules/` 中的规则文件
   * 项目的 `.claude/rules/` 或 `~/.claude/rules/` 中的[路径范围规则](#path-specific-rules)
2. 根据您会话的[上下文窗口](https://code.claude.com/docs/zh-CN/model-config.md#extended-context)找到对应的限制：
   * **200K 上下文窗口**：40,000 个字符
   * **1M 上下文窗口**：Claude Opus 4.6 和 Claude Sonnet 4.6 上为 200,000 个字符，更新的模型上为 150,000 个字符
3. 在 Bash 或 Zsh 中，运行 `wc -m path/to/CLAUDE.md` 以输出文件的字符数。
4. 将字符数与您的限制进行比较：
   * **超过限制**：精简文件，或将其中一部分移到一个或多个规则文件中，每个规则文件会单独计算是否超过限制
   * **未超过限制**：在 [Claude 不遵循我的 CLAUDE.md](#claude-isn’t-following-my-claude-md) 中查找其他原因

在 v2.1.309 之前，Claude Code 不会对按需加载的文件应用此限制。

### 在 `/compact` 后指令似乎丢失了

项目根 CLAUDE.md 在压缩后仍会保留：在 `/compact` 之后，Claude 从磁盘重新读取它并将其重新注入到会话中。子目录中的嵌套 CLAUDE.md 文件和具有 [`paths:` frontmatter](#path-specific-rules) 的规则会按需重新加载。

如果指令在压缩后消失，它要么仅在对话中给出，要么位于尚未重新加载的嵌套 CLAUDE.md 中，或者是此后尚未匹配到文件的路径范围规则。将仅在对话中给出的指令添加到 CLAUDE.md 以使其持久化。有关完整的细分，请参阅 [压缩后保留的内容](https://code.claude.com/docs/zh-CN/context-window.md#what-survives-compaction)。

有关大小、结构和具体性的指导，请参阅 [编写有效的指令](#write-effective-instructions)。

## 相关资源

* [调试你的配置](https://code.claude.com/docs/zh-CN/debug-your-config.md)：诊断为什么 CLAUDE.md 或设置未生效
* [Skills](https://code.claude.com/docs/zh-CN/skills.md)：打包按需加载的可重复工作流
* [Settings](https://code.claude.com/docs/zh-CN/settings.md)：使用设置文件配置 Claude Code 行为
* [Subagent 记忆](https://code.claude.com/docs/zh-CN/sub-agents.md#enable-persistent-memory)：让 subagents 维护自己的自动记忆
