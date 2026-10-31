> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# 自定义 Code Review

> 使用 REVIEW.md 和 CLAUDE.md 调整 Code Review 在您的 Pull Request 上标记的内容：重新校准严重程度、跳过路径、添加检查项，并查看适用的是哪个副本。

您可以通过向仓库提交指令来更改 [Code Review](https://code.claude.com/docs/zh-CN/code-review.md) 在该仓库上标记的内容。借此可以减少吹毛求疵类评论的数量、停止对生成文件发表评论，或添加团队关注的规则。请将这些指令放在以下两个文件之一中：

* **`CLAUDE.md`**：Claude Code 在执行每项任务（包括审查）时都会读取的[项目指令](https://code.claude.com/docs/zh-CN/memory.md)。Code Review 将其作为项目上下文读取，并标记违反这些指令的更改。
* **`REVIEW.md`**：仅用于审查的指令。使用它来说明您的团队希望标记哪些内容、以何种严重程度标记，以及要跳过哪些内容。

在仓库根目录添加 `REVIEW.md` 并将其合并。然后[保护您的默认分支](#which-branch-review-md-is-read-from)。

> [!NOTE]
>
> 如需更改审查的详尽程度或运行时机，请让 Owner 更改[审查级别](https://code.claude.com/docs/zh-CN/code-review-setup.md#set-the-default-review-level)或 [**Review runs** 设置](https://code.claude.com/docs/zh-CN/code-review-setup.md#set-when-reviews-run)。

## 使用 CLAUDE.md 设置项目范围的规则

Code Review 将仓库中的 `CLAUDE.md` 文件作为项目上下文读取。违反其中某条规则的更改会成为一项发现，并按照与问题相符的[严重程度级别](https://code.claude.com/docs/zh-CN/code-review-pull-requests.md#severity-levels)进行标记。

审查会读取仓库根目录下的 `CLAUDE.md`，以及每个已更改文件的各级父目录中的 `CLAUDE.md` 文件。当 Pull Request 更改了某个子目录路径下的文件时，会读取该子目录中的 `CLAUDE.md`。

## 使用 REVIEW\.md 设置仅用于审查的规则

`REVIEW.md` 是位于仓库根目录的文件，用于针对您的仓库定制 Code Review。您还可以在子目录中添加 `REVIEW.md`，在根文件的基础上为该路径下的更改添加检查项。

负责发现问题的 Agent 和负责验证问题的 Agent 会将其内容作为您仓库的审查指令接收，同时接收 Code Review 的默认审查指导。撰写最终审查的步骤会在确定严重程度和撰写摘要之前参考该文件。

以下示例布局包含一个根文件和一个子目录文件：

```text theme={null}
your-repo/
├── REVIEW.md            # Root file
├── services/
│   └── billing/
│       └── REVIEW.md    # Adds checks for changes under services/billing/
└── src/
```

在依赖子目录文件之前，请先保护您的默认分支，并查看[针对您的分支设置，审查会读取哪些内容](#what-a-review-reads-for-each-branch-setup)。

> [!NOTE]
>
> Code Review 服务和本地 [`/code-review` 命令](https://code.claude.com/docs/zh-CN/code-review-local)是两个独立的工具，它们读取的文件不同。该服务会同时读取 `CLAUDE.md` 和 `REVIEW.md`。`/code-review` 命令在您自己机器上的 Claude Code 会话中运行，它读取您的 `CLAUDE.md` 文件，但不读取 `REVIEW.md`。请将希望在两处都生效的指导放在 `CLAUDE.md` 中，并将 `REVIEW.md` 留给 Pull Request 审查专用的规则。

### REVIEW\.md 示例

此 `REVIEW.md` 为一个后端服务重新校准了严重程度、限制了 nit 数量、跳过了生成文件，并添加了仓库特定的检查项。

```markdown theme={null}
# Review instructions

## What Important means here

Reserve Important for findings that would break behavior, leak data,
or block a rollback: incorrect logic, unscoped database queries, PII
in logs or error messages, and migrations that aren't backward
compatible. Style, naming, and refactoring suggestions are Nit at
most.

## Cap the nits

Report at most five Nits per review.

## Do not report

- Anything CI already enforces: lint, formatting, type errors
- Generated files under `src/gen/` and any `*.lock` file
- Test-only code that intentionally violates production rules

## Always check

- New API routes have an integration test
- Log lines don't include email addresses, user IDs, or request bodies
- Database queries are scoped to the caller's tenant
```

### 选择在 REVIEW\.md 中写入的内容

`REVIEW.md` 是格式自由的 Markdown，用于更改审查发现和报告的内容。它不会改变审查的格式。找到您遇到的问题，然后添加类似所示的指令。[示例](#example-review-md)使用了前四项。

* **错误的内容被标记为 Important**：为您的仓库重新定义 🔴 Important 的含义。默认校准以生产代码为目标，而文档仓库、配置仓库或原型可能需要更窄的定义。说明哪些类别的发现属于 Important，哪些最多属于 Nit。您也可以提高级别，例如将任何违反 `CLAUDE.md` 的情况视为 Important。
* **nit 过多**：限制单次审查发布的 🟡 Nit 评论数量。诸如“最多报告五个 nit”之类的上限可以让审查保持可操作性。
* **对生成文件或 CI 已检查的内容发表评论**：列出 Claude 应跳过发现的路径和发现类别。常见的候选项包括生成的代码、锁文件和 vendored 依赖，以及 CI 已强制执行的任何内容，如 lint 或拼写检查。对于值得一定程度审查但无需全面审视的路径，可改为设置更高的门槛：“在 `scripts/` 中，仅在几乎确定且严重时才报告。”
* **某条团队规则未被检查**：添加您希望在每个 Pull Request 上标记的规则，例如“新的 API 路由必须有集成测试”。
* **事后证明有误的发现**：要求某类发现在发布前提供证据。例如，“关于行为的断言需要在源代码中引用 `file:line`，而不是根据命名进行推断”，这可以减少误报，否则作者需要为此额外往返一次。

其中一些指令[需要根文件位于受保护的默认分支上](#which-branch-review-md-is-read-from)。

`REVIEW.md` 中只保留会改变审查行为的指令，并将一般项目上下文留在 `CLAUDE.md` 中。

### 在 REVIEW\.md 中包含其他文件

您可以将审查指令拆分到多个文件中，再将它们引入根目录的 `REVIEW.md`。当 `REVIEW.md` 变得很长，或者您已经在其他文件中维护审查规则时，可以使用此方法。

要包含某个文件，请在根目录的 `REVIEW.md` 中添加一行，该行仅包含 `@` 和文件名。Code Review 会用该文件的内容替换这一行，使其作为 `REVIEW.md` 的一部分生效。在此示例中，根文件给出了一条自己的指令，然后包含了两个规则文件：

```markdown theme={null}
# Review instructions

Report at most five Nits per review.

@review-security.md
@review-api.md
```

将被包含的文件保存在仓库根目录，与 `REVIEW.md` 放在一起：

```text theme={null}
your-repo/
├── REVIEW.md
├── review-security.md
└── review-api.md
```

Code Review 仅在从受保护分支读取 `REVIEW.md` 时才会包含这些文件，因此请查看[针对您的仓库，审查会读取哪个分支](#what-a-review-reads-for-each-branch-setup)：

* **受保护分支**：Code Review 会用被包含的文件替换其 `@` 行
* **未受保护的默认分支**：`@` 行会作为纯文本保留在 `REVIEW.md` 中。如果 Pull Request 更改了该行所指定的文件，则该行会被省略。

当审查未应用被包含的文件时，请逐项检查以下内容：

* **位置**：将文件移至仓库根目录。除 [skill 文件](#include-a-skill-file)外，指向子目录的路径（例如 `@docs/review-rules.md`）不会被包含。
* **名称**：文件名须完全匹配（包括大小写），并使用不含空格的名称。如果某行中的名称与 `REVIEW.md` 同级的任何文件都不匹配，该行会作为纯文本保留在 `REVIEW.md` 中，且检查运行不会将其报告为被跳过的包含项。
* **仅限一层**：在根目录的 `REVIEW.md` 中列出所有文件。被包含文件或子目录 `REVIEW.md` 中的 `@` 行会保留为纯文本（[skill 文件有所不同](#include-a-skill-file)），而指向其他文件的 Markdown 链接不属于包含项。
* **限制**：最多包含八个不同的文件，并使 `REVIEW.md` 及其包含的文件合计小于 150,000 字节。会达到字节上限的文件将被整体跳过，而不会被截断。检查运行会报告因任一限制而跳过了多少个包含行。
* **链接**：`REVIEW.md` 及其包含的文件中的 Markdown 链接总数应不超过 64 个。指向 `http`、`https` 或 `mailto` 地址以及 `#` 锚点的链接不计入，[skill 文件](#include-a-skill-file)中的链接也不计入。超过该上限时，审查会跳过您的整个根目录 `REVIEW.md`，而不仅仅是被包含的文件。此时检查运行会[说明指令未被应用](https://code.claude.com/docs/zh-CN/code-review-troubleshooting.md#the-check-run-mentions-review-md)。
* **已合并的副本**：将被包含的文件合并到[读取 `REVIEW.md` 的分支](#which-branch-review-md-is-read-from)中。审查会从同一个提交中读取两者，从不从 Pull Request 自身的更改中读取。

### 包含 Skill 文件

要让审查遵循您在 [skill](https://code.claude.com/docs/zh-CN/skills.md) 中维护的指导，请在根目录的 `REVIEW.md` 中包含该 skill 的 `SKILL.md`。Code Review 不会运行 skill。

在单独的一行中添加 `@` 和 `SKILL.md` 的路径，格式为 `.claude/skills/<folder>/SKILL.md`：

```markdown theme={null}
# Review instructions

Report at most five Nits per review.

@.claude/skills/api-review/SKILL.md

Treat any change under `migrations/` as Important.
```

请保护[读取 `REVIEW.md` 的分支](#which-branch-review-md-is-read-from)，因为 Code Review 仅在从受保护分支读取 `REVIEW.md` 时才会包含 skill 文件。skill 文件计入八个文件和 150,000 字节的限制，并且有其自身 30,000 字节的限制。

#### 编写能被 Code Review 包含的 Skill 行

请使用不含空格的文件夹名称。路径中含有空格的行会作为纯文本保留在 `REVIEW.md` 中，且检查运行不会将其报告为被跳过的包含项。

如果某个 skill 的 `@` 行违反了以下任一规则，Code Review 不会从该行包含该 skill：

* **路径**：从仓库根目录以 `.claude/skills/<folder>/SKILL.md` 的形式写出路径，skill 的文件夹须直接位于 `skills/` 下。与根目录下的文件名不同，此形式的路径如果不匹配任何文件，会被跳过并计数。
* **来源**：将 `.claude/` 下任何其他文件中的指导，或[内置、个人或插件 skill](https://code.claude.com/docs/zh-CN/skills.md#where-skills-live) 中的指导，复制到与 `REVIEW.md` 同级的文件中。您无法直接包含这些内容。
* **文件**：将 `SKILL.md` 作为常规文本文件提交。Code Review 不会跟随符号链接，无论该链接是文件本身还是其路径中的某个文件夹。
* **大小**：使 `SKILL.md` 不超过 30,000 字节（包括 frontmatter）
* **数量**：最多编写八个 skill 行。重复的行会再次计入这八个。
* **`REVIEW.md` 中的块**：不要将 `@` 行放在代码围栏内。关闭其上方的每个 HTML 注释以及每个 `<pre>`、`<script>`、`<style>` 或 `<textarea>` 块。

当 Code Review 因违反文件、大小、数量或块规则而跳过某个 skill 行时，检查运行中[关于被跳过包含行的消息](https://code.claude.com/docs/zh-CN/code-review-troubleshooting.md#the-check-run-mentions-review-md)会将其计入。仅当审查从受保护分支应用 `REVIEW.md` 时，才会显示该消息。

#### 检查审查从 Skill 中获得的内容

当 skill 被包含时，审查只会获得 `SKILL.md` 的正文，而不会获得该 skill 的其他任何内容，因此请对照以下列表检查该 skill。

* **Frontmatter**：在正文中重复审查所需的 [frontmatter](https://code.claude.com/docs/zh-CN/skills.md#frontmatter-reference) 内容，例如 `description` 给出的使用该 skill 的条件
* **注入的命令**：写出 [`!` 命令](https://code.claude.com/docs/zh-CN/skills#inject-dynamic-context)本应提供的内容。Code Review 不会运行该命令，并会用占位符替换包含该命令的整行或整个块。
* **替换变量**：将 `$ARGUMENTS` 和其他[替换变量](https://code.claude.com/docs/zh-CN/skills.md#available-string-substitutions)替换为固定文本，因为审查不传递任何参数，并会将它们原样保留
* **支持文件**：将审查所需的内容复制到 `SKILL.md` 中，因为 Code Review 不会加载脚本、链接的文件或 `@file` 引用
* **排除项和 Nit 上限**：将排除发现或限制 Nit 数量的指令写在 `REVIEW.md` 或与其同级的文件中，而不是写在 skill 中。Code Review 不接受以 skill 的文本作为这两者的依据。
* **指向仓库文件的链接**：用文字说明链接或 `@file` 引用所指向的内容。审查会将该路径视为纯文本或占位符。

如果您编辑 skill 的 `!` 命令、`$ARGUMENTS` 或链接，也会改变它在 Claude Code 中的工作方式。如果您的团队依赖这些功能，请将指导复制到与 `REVIEW.md` 同级的文件中，并包含该文件来代替 skill。

#### Code Review 在 Skill 文本中更改的字符

Code Review 会更改 skill 文本中的某些字符，因此您的代码示例会以下面的形式传达给审查。

```text theme={null}
In SKILL.md:       Use `items[0]`, never `List<String>`.
The review reads:  Use ˋitems(0)ˋ, never ˋList< String>ˋ.

In SKILL.md:       Run `ps | grep api` and ask @alice.
The review reads:  Run ˋps ¦ grep apiˋ and ask @ alice.
```

如果某条规则依赖于其中某个字符，也请用文字加以说明。

### 从哪个分支读取 REVIEW\.md

Code Review 从 Pull Request 的基础分支或默认分支读取 `REVIEW.md`，从不从 Pull Request 自身的更改中读取。如需更改审查行为，请先合并对 `REVIEW.md` 的更改。

请将 `REVIEW.md` 保留在默认分支上，并保护该分支。当审查读取受保护默认分支上的副本时（即下表第一行的情况），它会遵循根文件中的以下指令：

* 降低某项发现的严重程度
* 排除路径、文件或问题类别
* 限制 nit 评论数量

请将排除项和 Nit 上限写在 `REVIEW.md` 或与其同级的文件中，而不是写在[被包含的 skill](#include-a-skill-file) 中。

#### 哪些设置算作分支保护

Code Review 将以下设置视为保护：

* **默认分支**：经典分支保护规则或规则集
* **任何其他分支**：仅限规则集

当规则集限制更新，或要求 Pull Request 获得至少一次审批时，该规则集才算作保护。如果它要求 Pull Request，则还必须在有新推送时撤销该审批，或要求审批发生在最后一次推送之后。

#### 针对每种分支设置，审查会读取哪些内容

在下表第一列中找到您仓库的情况。

| 分支保护 | 读取的副本 | 读取的文件 |
| :- | :- | :- |
| 默认分支受保护，且 Pull Request 以其为目标，或以未受任何规则集保护的分支为目标 | 默认分支 | 根文件、其包含的文件和子目录文件 |
| Pull Request 以另一个受规则集保护的分支为目标 | 该基础分支 | 根文件、其包含的文件和子目录文件 |
| 默认分支未受保护，且基础分支没有任何一种保护规则 | 默认分支 | 根文件 |
| Code Review 无法确认保护设置。这包括默认分支未受保护、而基础分支仅有经典规则的情况 | 无 | 无 |

如果检查运行中提到了 `REVIEW.md`，请参阅[检查运行中提到 REVIEW.md](https://code.claude.com/docs/zh-CN/code-review-troubleshooting.md#the-check-run-mentions-review-md)。

## 相关资源

* [Code Review](https://code.claude.com/docs/zh-CN/code-review.md)：该服务是什么以及审查如何运行
* [在 Pull Request 上阅读和请求审查](https://code.claude.com/docs/zh-CN/code-review-pull-requests.md#severity-levels)：每个严重程度级别的含义以及发现的呈现方式
* [设置 Code Review](https://code.claude.com/docs/zh-CN/code-review-setup.md)：由 Owner 设置的审查级别和审查运行时机
* [使用 `/code-review` 在本地审查](https://code.claude.com/docs/zh-CN/code-review-local)：`/code-review` 命令、其参数和 effort 级别
* [记忆](https://code.claude.com/docs/zh-CN/memory.md)：`CLAUDE.md` 文件在 Claude Code 中的工作方式
