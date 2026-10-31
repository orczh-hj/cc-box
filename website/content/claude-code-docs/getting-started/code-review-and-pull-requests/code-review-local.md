> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# 使用 /code-review 在本地审查

> 在 Claude Code 会话中运行 `/code-review`，查找分支变更中的 bug，将审查指向 Pull Request 或路径，并让 Claude 应用修复。

`/code-review` 是一个在 Claude Code 输入框中输入的命令，在您正在工作的会话中运行。Claude 会审查您分支上的变更，查找正确性 bug，并在对话中报告发现的问题。根据您的模型和 effort 级别，审查还会涵盖复用、简化和效率方面的清理。

该命令适用于任何套餐。审查在您自己的会话中运行，因此与会话中的其他部分一样，计入您套餐的用量或 API 计费。

> [!NOTE]
>
> 如需在 GitHub 上自动审查仓库中的每个 Pull Request，请参阅 [Code Review](https://code.claude.com/docs/zh-CN/code-review.md)，这是一项由 Owner 为您的组织启用的托管服务。

`/review` 是 `/code-review` 的别名。在 v2.1.223 之前，`/review` 是一个单独的命令，对 GitHub Pull Request 执行单次、只读的审查。

**1. 运行 /code-review**

运行命令：

```text theme={null}
/code-review
```

不指定目标时，审查涵盖当前分支上的变更。如需将审查指向 Pull Request、其他分支、路径或提交范围，请参阅[选择审查内容](#choose-what-to-review)。

您还可以添加标志：

* `--fix`：在审查后还要求 Claude 将发现的问题修复应用到您的工作树。其编辑会经过您的[权限模式](https://code.claude.com/docs/zh-CN/permission-modes.md)。
* `--comment`：将发现的问题以行内评论形式发布到 GitHub Pull Request 上，或以单条备注形式发布到 [GitLab merge request](#post-findings-to-a-gitlab-merge-request)上

**2. 继续工作**

审查作为后台[子代理](https://code.claude.com/docs/zh-CN/sub-agents.md)运行，拥有自己的上下文窗口，因此不会占满您的对话。审查完成后，发现的问题会出现在您的对话中。

**3. 处理发现的问题**

让 Claude 修复审查发现的问题。使用 `--fix` 或 `--comment` 时，Claude 会在审查过程中应用或发布发现的问题，系统可能会请您批准其编辑。

## 选择审查内容

如需审查当前分支以外的内容，请在 effort 级别和任何标志之后输入目标。Claude 会将该文本作为审查目标接收。目标可以是 Pull Request 编号、分支名称、文件路径或 Git 修订范围。在第一列中找到您要审查的内容，然后输入第二列所示的内容：

| 审查目标 | 输入 |
| :- | :- |
| 当前分支和未提交的变更 | `/code-review` |
| 最近 15 个提交 | `/code-review HEAD~15..HEAD` |
| 一个分支相对于另一个分支 | `/code-review main...my-feature` |
| 单个提交，其中 `abc1234` 是该提交的哈希值 | `/code-review abc1234^..abc1234` |
| 按编号指定的 Pull Request | `/code-review 1234` |
| 文件或目录路径 | `/code-review src/auth` |

不指定目标时，`/code-review` 会审查您的分支领先于其上游分支的提交以及未提交的变更。如果分支没有上游，Claude 会与 `main` 或上一个提交进行比较。

使用 `ultra` 时，Claude Code 会以不同方式解读目标，详见 [effort 级别和标志之后的文本](#text-after-the-effort-level-and-flags)。

## 审查读取和编辑的内容

在运行审查之前，尤其是使用 `--fix` 时，请检查以下内容：

* **编辑**：不使用 `--fix` 时，审查的指令仅要求 Claude 提供报告。使用 `--fix` 时，还会要求它将发现的问题修复应用到您的工作树。无论哪种情况，任何编辑都会经过您的权限模式。
* **回退**：后台审查在会话的[检查点](https://code.claude.com/docs/zh-CN/checkpointing.md#subagent-edits-not-restored)之外应用其 `--fix` 编辑。在此之后，`/rewind` 只会恢复 Claude 此前已在您的对话中编辑过的文件。审查对其他文件的编辑会保留，因此请使用 git 撤销这些编辑。当审查[在前台运行](#run-in-the-foreground)时，它会在您自己的轮次中编辑工作树，因此 `/rewind` 会照常恢复其编辑。
* **指令**：审查会遵循您的 `CLAUDE.md`，包括在非交互的 `-p` 运行中。它不会读取 `REVIEW.md`。该文件用于配置 GitHub Pull Request 上的托管 [Code Review 服务](https://code.claude.com/docs/zh-CN/code-review-customize.md#use-review-md-for-review-only-rules)。

## 如何报告发现的问题

在以下两种运行中，Claude 都会在回复中以文本形式报告发现的问题：

* 在终端会话中，`/code-review` 以[分叉子代理](https://code.claude.com/docs/zh-CN/skills.md#run-skills-in-a-subagent)的形式运行审查
* 在输出为文本或 JSON 的 `-p` 运行中

在请求发现列表的宿主应用程序（例如[桌面应用](https://code.claude.com/docs/zh-CN/desktop.md#review-your-code)）中，Claude 会改为通过 [`ReportFindings` 工具](https://code.claude.com/docs/zh-CN/tools-reference)报告审查发现的问题。报告以发现列表的形式显示，每个条目会显示文件位置、一句话摘要，以及类别标签（例如 `correctness`，如果该问题带有类别）。宿主请求适用于所有 effort 级别。

当 Claude 在会话后续修复已报告的问题时，会再次报告这些问题。Claude Code 随后会在更新后的发现列表中将每个问题标记为已修复、已跳过或无需更改。

## 调整 effort 和参数

除了 `--fix` 和 `--comment` 之外，该命令还接受以下标志：

* `--post`：在对 `github.com` Pull Request 进行 `ultra` 云端审查时，在启动对话框中预先选择将最终发现的问题发布到 PR。请参阅[将发现的问题发布到 Pull Request](https://code.claude.com/docs/zh-CN/ultrareview.md#post-findings-to-the-pull-request)。需要 Claude Code v2.1.227 或更高版本。
* `--max-findings <n>`、`--max-findings all` 或 `--max-findings default`：最多报告 `n` 个发现的问题，或使用 `all` 报告所有发现的问题，以替代审查的常规上限。当审查通过发现工具报告时，上限为 32。之后的审查会沿用您输入的值，直到您传入 `--max-findings default`，但在非交互的 `-p` 运行中传入的值不会被记住。需要 Claude Code v2.1.288 或更高版本。

传入 [effort 级别](https://code.claude.com/docs/zh-CN/model-config.md#adjust-effort-level)以在覆盖范围和置信度之间进行权衡。在 `low` 级别，审查会报告它最有把握的问题，因此您看到的误报更少。从 `medium` 到 `max`，审查会扩大覆盖范围。

如果您不输入级别，审查会沿用您上次输入的 `low` 到 `max` 之间的级别，即使是在之前的会话中输入的。Claude Code 会显示类似 `Reusing high effort, the level you typed last time` 的提示。以下规则决定一次运行使用哪个级别：

* **输入级别**：您输入的级别（如 `/code-review high`）会改变之后运行所沿用的级别
* **非交互运行**：在非交互的 `-p` 运行中传入的级别不会更新记住的级别
* **`ultra`**：审查既不使用也不更新记住的级别
* **从未输入过级别**：审查使用会话当前的 effort

在 v2.1.223 之前，不带级别的 `/code-review` 始终使用会话当前的 effort。

### 将发现的问题发布到 GitLab 合并请求

当您为 GitLab 合并请求传入 `--comment` 时，Claude Code 会通过 GitLab 的 `glab` CLI 发布发现的问题。需要 Claude Code v2.1.257 或更高版本。如果未安装 `glab`，Claude 会改为在终端中打印发现的问题。

请以 URL 或 `!123` 引用的形式传入合并请求。仅当检出的 origin 位于 `gitlab.com` 上时，Claude Code 才会将单独的数字或分支名称视为合并请求。在自托管的 GitLab 实例上，请传入 URL 或 `!123` 形式。

### effort 级别和标志之后的文本

在 effort 级别和标志之后，Claude Code 会将该行的其余部分读取为审查目标。`ultra` 参数会改变这种解读方式：

* **使用 `ultra` 时**：Claude Code 会将单个单词读取为基础分支或 PR 编号。未指明分支或 PR 的较长文本会成为[附加到审查的备注](https://code.claude.com/docs/zh-CN/ultrareview.md#pass-a-request-in-plain-words)。`/code-review ultra check my auth changes` 会审查您的当前分支，Claude 会将发现的问题与您的备注关联起来。
* **不使用 `ultra` 时**：剩余的所有内容都是审查目标，即使它以另一个命令名称开头。`/code-review /fix-issue 123` 会以 `/fix-issue 123` 作为目标文本进行审查，而不是将 `/fix-issue` 作为第二个[叠加的 skill](https://code.claude.com/docs/zh-CN/skills.md#stack-several-skills-in-one-message) 加载。

## 在前台运行

如需让审查在前台运行，请执行以下操作之一：

* 在非交互模式下运行，使用 `-p` 标志或 Agent SDK。Claude Code 会等待审查完成，并在响应中包含发现的问题，但 `ultra` 除外，它[不会等待云端审查](https://code.claude.com/docs/zh-CN/ultrareview.md#run-ultrareview-non-interactively)。
* 将 [`CLAUDE_CODE_DISABLE_BACKGROUND_TASKS`](https://code.claude.com/docs/zh-CN/env-vars) 设置为 `1`，这也会关闭其他后台任务功能
* 在之前的审查仍在进行时再次运行 `/code-review`

## 让 Claude 启动审查

Claude 可以自行启动 `/code-review`，无需您输入命令：

* **当您用自然语言提出请求时**：请 Claude 审查您的变更，它就可以运行该 skill
* **通过定时任务**：以 `/code-review` 作为提示词的[会话级定时任务](https://code.claude.com/docs/zh-CN/scheduled-tasks.md)会运行审查

会话级定时任务永远不会启动[云端审查](#escalate-to-ultrareview)，因此请在不带 `ultra` 参数的情况下安排 `/code-review`。

如需阻止 Claude 和定时任务启动审查，同时保留 `/code-review` 供您手动输入，请在[设置文件](https://code.claude.com/docs/zh-CN/settings.md#where-settings-live)（例如 `~/.claude/settings.json`）中添加 [`skillOverrides`](https://code.claude.com/docs/zh-CN/skills#override-skill-visibility-from-settings) 条目：

```json theme={null}
{
  "skillOverrides": {
    "code-review": "user-invocable-only"
  }
}
```

在 v2.1.246 之前，只有在从 Anthropic 获取的功能标志启用了该功能时，Claude 才会自行启动 `/code-review`。在[不获取功能标志的会话](https://code.claude.com/docs/zh-CN/env-vars.md#features-that-need-feature-flag-fetching)中，`/code-review` 仅在您输入时运行，而定时的 `/code-review` 会作为纯文本发送给 Claude。

## 升级到 ultrareview

`/code-review ultra --fix` 会在云端运行更深入的 [ultrareview](https://code.claude.com/docs/zh-CN/ultrareview.md)，然后在发现的问题返回到您的会话时，要求 Claude 将其修复应用到您的工作树。

Ultrareview 使用自己的范围：

* **比较内容**：您的当前分支与仓库默认分支的对比，以及工作树中未提交和已暂存的变更
* **敏感文件**：对于名称类似凭据或密钥的文件（例如 `.env` 和 `*.tfvars` 文件）的未提交变更，Claude Code 会遵循[将本地仓库上传到云端会话](https://code.claude.com/docs/zh-CN/claude-code-on-the-web.md#send-local-repositories-without-github)的规则
* **不同的基础分支**：传入分支名称（例如 `/code-review ultra develop`）以与不同的基础分支进行比较

当目标是 `github.com` Pull Request 时，您可以让 Claude 以您的 GitHub 账户身份[将最终发现的问题作为评论发布到 PR](https://code.claude.com/docs/zh-CN/ultrareview.md#post-findings-to-the-pull-request)。需要 Claude Code v2.1.227 或更高版本。

> [!NOTE]
>
> Ultrareview 需要使用 claude.ai 账户进行身份验证。它不适用于 Amazon Bedrock、Google Cloud 的 Agent Platform 或 Microsoft Foundry，也不适用于启用了零数据保留或应用了 [HIPAA 配置](https://code.claude.com/docs/zh-CN/hipaa-setup.md)的组织。在这些情况下，`/code-review ultra` 会改为在您的会话中运行本地审查。使用[客户管理的加密密钥](https://platform.claude.com/docs/en/manage-claude/cmek)（CMEK）的组织无法运行 ultrareview。如需在本地审查您的变更，请运行 `/code-review`。

如需从脚本或 CI 作业运行云端审查，请使用 [`claude ultrareview` 子命令](https://code.claude.com/docs/zh-CN/ultrareview#run-ultrareview-non-interactively)，它会等待发现的问题并将其打印到 stdout。

## 修改或基于审查进行构建

`/code-review` 是一个[随附 skill](https://code.claude.com/docs/zh-CN/skills.md#bundled-skills)：其指令内置于 Claude Code 中，因此磁盘上没有可打开或编辑的 `SKILL.md` 文件。如需为每次审查提供需遵循的项目规则，请将其添加到您的 `CLAUDE.md` 中。如需更进一步，请从以下页面之一开始：

* **替换命令**：名为 `code-review` 的项目或个人 skill 会替代随附的 skill 运行，而 `/review` 别名仍会运行随附的审查。请参阅[解决同名 skill 的冲突](https://code.claude.com/docs/zh-CN/skills.md#resolve-skills-that-share-a-name)。
* **构建您自己的审查器**：编写一个审查[子代理](https://code.claude.com/docs/zh-CN/sub-agents.md)或审查 [skill](https://code.claude.com/docs/zh-CN/skills.md)
* **使用相邻命令**：[`/simplify`](https://code.claude.com/docs/zh-CN/commands) 仅审查清理方面的问题并应用其修复，[`/verify`](https://code.claude.com/docs/zh-CN/commands) 会运行您的应用以确认变更有效

在 v2.1.147 之前，该命令名为 `/simplify`，当时它默认会应用修复。如果您曾编写脚本使用 `/simplify` 查找 bug，请改用 `/code-review --fix`。

## 相关资源

* [Code Review](https://code.claude.com/docs/zh-CN/code-review.md)：为您的组织审查 GitHub 上每个 Pull Request 的托管服务
* [Ultrareview](https://code.claude.com/docs/zh-CN/ultrareview.md)：更深入的云端审查的设置、限制和费用
* [命令](https://code.claude.com/docs/zh-CN/commands.md)：`/code-review`、`/review`、`/simplify` 和 `/verify` 的单行参考
* [Skills](https://code.claude.com/docs/zh-CN/skills.md)：随附 skill 的工作原理以及如何编写您自己的 skill
