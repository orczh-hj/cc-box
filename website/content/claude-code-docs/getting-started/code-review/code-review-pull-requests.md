> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# 阅读和请求 Pull Request 上的审查

> 阅读 GitHub Pull Request 上的 Code Review 发现，回复或忽略这些发现，并使用 `@claude review` 请求审查。

Code Review 是一项托管服务，由您的 Claude 组织中的 Owner 为一组 GitHub 仓库启用。启用后，Claude 会审查这些仓库中的 Pull Request，并将其发现以内联评论、审查正文中的摘要以及名为 **Claude Code Review** 的检查运行的形式发布。审查永远不会批准或阻止您的 Pull Request，检查运行也永远不会以失败结束。

Owner 可以为每个仓库控制[审查何时运行](https://code.claude.com/docs/zh-CN/code-review-setup.md#set-when-reviews-run)以及[审查的详尽程度](https://code.claude.com/docs/zh-CN/code-review-setup.md#set-the-default-review-level)。

> [!NOTE]
>
> 如果您的套餐不包含 Code Review，您仍然可以在 Claude Code 会话中让 Claude 审查您的分支：请参阅[使用 `/code-review` 在本地审查](https://code.claude.com/docs/zh-CN/code-review-local)。

## 严重程度级别

Claude 将每个发现作为内联评论发布在相关代码行上，并用以下三种严重程度级别之一进行标记：

| 标记 | 严重程度 | 含义 |
| :- | :- | :- |
| 🔴 | Important | 应在合并前修复的 bug |
| 🟡 | Nit | 小问题，值得修复但不阻塞合并 |
| 🟣 | Pre-existing | 代码库中已存在、但并非由此 PR 引入的 bug |

如果审查记录了其推理过程，该发现会带有一个折叠的 **Why this was flagged** 部分。展开它即可了解 Claude 为何标记该问题以及如何验证该问题。

要更改 Claude 在您的仓库中标记的内容或其对严重程度的判定方式，请参阅[自定义 Code Review](https://code.claude.com/docs/zh-CN/code-review-customize.md)。

## 回应发现

除非您的仓库自身规则有要求，否则您无需回应发现即可合并。如果仓库的分支保护[要求解决对话](https://docs.github.com/en/repositories/configuring-branches-and-merges-in-your-repository/managing-protected-branches/about-protected-branches#require-conversation-resolution-before-merging)，请在合并前解决每个发现的讨论串。

要清除某个发现，请解决其讨论串或推送修复。

* **不修改代码直接忽略**：在 GitHub 上自行解决该讨论串。仅回复并不会忽略发现。
* **修复问题**：修改代码并推送。当下一次对该 Pull Request 的审查判断问题已修复时，会解决之前的讨论串，无论该审查是在您推送时自动运行还是由您请求的。
* **解释或提出异议**：回复该评论。Claude 不会立即回复，回复也不会启动审查。下一次对该 Pull Request 的审查会读取对其之前发现的回复，并可以解决该讨论串或发布一条后续回复。如果同类发现反复出现，请让维护者在 [`REVIEW.md`](https://code.claude.com/docs/zh-CN/code-review-customize#choose-what-to-write-in-review-md) 中添加规则。

要在不推送的情况下获得下一次审查，请以[顶层 Pull Request 评论](#comment-commands)的形式发表 `@claude review`。

## 检查运行输出

**Claude Code Review** 检查运行上的 **Details** 链接会在一个按严重程度排序的表格中打开所有发现，包括 GitHub 未接受其行内评论的发现：

| 严重程度 | 文件:行 | 问题 |
| - | - | - |
| 🔴 Important | `src/auth/session.ts:142` | token 刷新与登出之间存在竞争条件，导致本应失效的陈旧会话仍保持活动状态 |
| 🟡 Nit | `src/auth/session.ts:88` | `parseExpiry` 在输入格式错误时静默返回 0 |

对于有对应行号但未作为行内评论发布的发现，会以注释的形式显示在 **Files changed** 标签页中的相关行上。Important 发现显示为红色，nit 显示为黄色，已存在的 bug 显示为灰色。

检查运行的结论为 neutral；在某些未更改任何文件并[保留先前结果](#when-a-push-keeps-the-earlier-result)的推送之后，结论为 skipped。它的结论永远不会是 failure，因此已完成的审查不会导致必需检查失败。如果您没有看到发现，或者审查未运行，请参阅[排查 Code Review 问题](https://code.claude.com/docs/zh-CN/code-review-troubleshooting.md)。

### 推送何时保留先前结果

如果推送后检查运行显示以下标题之一，则表示 Code Review 保留了先前审查的结果，而没有启动新的审查：

* **Code review skipped — no new changes**：先前审查中没有仍处于打开状态的 [Important](#severity-levels) 发现，因此无需进行新的审查
* **Code review not repeated — earlier findings still open**：要启动新的审查，请推送针对某个打开的 Important 发现的修复，或[解决其讨论串](#respond-to-findings)后再次推送

在 github.com 上，当以下两个条件同时满足时，Code Review 可以保留结果：

* Pull Request 仍然添加和删除相同的行，例如在空提交、干净地合并基础分支或干净地变基之后
* 自该次审查以来，没有人解决过任何 Important 发现的讨论串

要在不更改代码的情况下获得审查，请评论 [`@claude review`](#comment-commands) 或点击检查运行上的 **Re-run**。

### 通过脚本读取结果

要在您自己的 CI 中控制合并或发布摘要，请使用 `gh` 和 jq 解析 Details 文本的最后一行。该行是一条 HTML 注释，以 JSON 形式保存各严重程度的计数：

```text theme={null}
<!-- bughunter-severity: {"nit":1,"normal":2,"pre_existing":0} -->
```

其上方的行可能包含 Pull Request 中的文件名。如果您以自己的方式解析文本，请仅从最后一行获取计数，就像以下步骤中的命令所做的那样。

**1. 查找检查运行 ID**

在您的 shell 中，列出该提交的检查运行，并获取 `Claude Code Review` 运行的 `id`。将 `OWNER` 和 `REPO` 替换为您的仓库所有者和名称，将 `<commit-sha>` 替换为 Pull Request 的头部提交：

```bash theme={null}
gh api repos/OWNER/REPO/commits/<commit-sha>/check-runs --jq '.check_runs[] | {id, name}'
```

如果该提交有多次 `Claude Code Review` 运行（例如在 **Re-run** 之后），GitHub 只会列出最新的一次。

**2. 读取严重程度计数**

从检查运行的 `output.text` 字段中读取计数，该字段保存 Details 文本。将 `CHECK_RUN_ID` 替换为该 ID：

```bash theme={null}
gh api repos/OWNER/REPO/check-runs/CHECK_RUN_ID --jq '
  (.output.text | split("\n") | last
    | scan("^<!-- bughunter-severity: (.*) -->$")[0] | fromjson)?
  // error("no severity line")'
```

这会输出一个 JSON 对象，其中包含每种严重程度的计数，键按字母顺序排列，例如 `{"nit": 1, "normal": 2, "pre_existing": 0}`。除了[保留先前结果](#when-a-push-keeps-the-earlier-result)的检查运行之外，计数包括该 Pull Request 早期审查中讨论串仍处于打开状态的发现。

在标题为 `Code review skipped — no new changes` 时，计数不包括早期审查中仍处于打开状态的 nit 和已存在问题的发现。此时，先前审查中没有仍处于打开状态的 Important 发现。

**3. 根据结果采取行动**

让您的脚本按顺序处理以下情况：

* **标题以 `Code review not repeated` 开头**：Code Review [保留了先前结果](#when-a-push-keeps-the-earlier-result)，其 Important 发现仍处于打开状态，且检查运行中没有严重程度行。请在读取计数之前先读取标题，使用 `gh api repos/OWNER/REPO/check-runs/CHECK_RUN_ID --jq .output.title`。如果您希望控制合并，请让作业失败。
* **标题以 `no new changes` 结尾**：无需读取计数，直接让作业通过
* **`normal` 大于零**：如果您希望控制合并，请让作业失败。`normal` 键保存的是 Important 发现的数量，这意味着 Claude 至少发现了一个值得在合并前修复的 bug。如果只想输出该计数，请在结尾的单引号之前添加 `| .normal`。
* **该行缺失**：命令会输出包含 `no severity line` 的错误而不是计数，并以非零状态退出。将其视为没有结论，而不是零个发现。对于失败、超时、被取消或被更新审查取代的审查，其检查运行中没有此行，某些已完成的审查也没有此行。

## 请求审查

如果您的 Pull Request 是草稿或来自 fork，自动审查会跳过它，因此请[使用评论命令请求审查](#review-drafts-and-pull-requests-from-forks)。要让您的 Pull Request 获得审查，请执行以下操作之一：

* **打开或标记为就绪**：在设置为 **Once after PR creation** 或 **After every push** 的仓库中，当您打开或重新打开 Pull Request，或将草稿标记为可供审查时，会启动审查。
* **发表命令评论**：在 Pull Request 上发布 [`@claude review`](#comment-commands)。
* **推送提交**：在设置为 **After every push** 的仓库中，或在您使用 `@claude review always` 订阅的 Pull Request 上，推送会启动审查。如果推送后您的 Pull Request 添加和删除的行仍然相同，Code Review 可能会改为[保留之前的结果](#when-a-push-keeps-the-earlier-result)。
* **点击 Re-run**：在非 fork 的 Pull Request 上审查失败或超时后，从检查运行中[再次运行](https://code.claude.com/docs/zh-CN/code-review-troubleshooting.md#retrigger-a-failed-or-timed-out-review)。在非 fork 的 Pull Request 上，您也可以在某次推送[保留了之前的结果](#when-a-push-keeps-the-earlier-result)后点击 **Re-run**。
* **等待自动重试**：会话中断的审查会自动重试一次。[检查运行的摘要](https://code.claude.com/docs/zh-CN/code-review-troubleshooting.md#retrigger-a-failed-or-timed-out-review)会显示何时正在进行重试。

### 评论命令

在 Pull Request 上以评论形式发布以下命令之一：

| 命令 | 作用 | 使用时机 |
| :- | :- | :- |
| `@claude review` | 启动一次审查，但不为 PR 订阅后续推送 | 您现在需要一次审查，例如在设置为 **Manual** 的仓库中的 Pull Request 上，或在其他模式下立即再进行一次审查 |
| `@claude review always` | 启动一次审查，并为该 PR 订阅今后由推送触发的审查 | 您希望后续推送都能启动审查，例如在设置为 **Manual** 的仓库中处理高优先级 PR 时 |
| `@claude review once` | 与 `@claude review` 相同：启动一次审查而不订阅 | 与 `@claude review` 的情况相同 |
| `@claude review stop` | 结束 PR 对由推送触发的审查的订阅，既不启动审查，也不取消已在运行的审查。Claude 会以评论而非检查运行进行回复。 | 您在设置为 **Manual** 或 **Once after PR creation** 的仓库中使用 `@claude review always` 订阅了某个 PR，但不再希望每次推送都启动审查 |
| `@claude review off` 或 `@claude review pause` | 与 `@claude review stop` 相同 | 与 `@claude review stop` 的情况相同 |

在设置为 **After every push** 的仓库中，您无法通过评论停止审查，因为在那里无论是否有订阅，推送都会启动审查。要在此类仓库中阻止推送为某个 Pull Request 启动审查，请将其转换为[草稿](#review-drafts-and-pull-requests-from-forks)。当您再次将其标记为可供审查时，会启动一次审查。

要让任何评论命令生效，必须满足以下所有条件：

* 将其作为新的顶层 Pull Request 评论发布，而不是 diff 行上的内联评论，也不是审查的正文。编辑之前的评论不算数。
* 将整个命令放在评论开头的同一行中。命令之后可以跟其他文本。如果 `always` 或 `stop` 拼写错误或位于后面的行中，Claude 会将该评论视为普通的 `@claude review` 并启动一次审查。大小写无关紧要。
* 您需要拥有该仓库的 write、maintain 或 admin 权限。Claude 会向 GitHub 核实每位评论者的权限，来自 bot 的评论不会启动审查。
* Pull Request 必须处于打开状态。

启动审查的命令被接受后，Pull Request 上会出现一个标题为 **Code review queued...** 的 **Claude Code Review** 检查运行。审查开始时，标题会变为 **Code review in progress...**。Claude 也可能会用 👀 回应您的评论，但即使没有启动审查，该回应也可能出现，因此请以检查运行为准。

如果 Pull Request 的最新提交上已有审查正在运行，新的请求不会启动第二次审查。

> [!NOTE]
>
> 在 2026 年 7 月的更新之前，`@claude review` 会为 PR 订阅由推送触发的审查。如果您依赖该行为，请改为评论 `@claude review always`。

### 后续审查涵盖的内容

自行启动的后续审查（例如推送后）会侧重于自上次审查以来的更改，并发布较少的 [nit 和已存在问题的发现](#severity-levels)。要了解它是否因此省略了某些发现，请在审查正文中查找统计这些发现的句子：

* **针对自上次审查以来未更改代码的发现**：该句以 "outside the scope of this re-review and not posted" 结尾
* **nit 和已存在问题的发现**：该句以 "found and not posted" 结尾

要获得不省略任何这些内容的审查，请评论 [`@claude review`](#comment-commands)。它会再次涵盖整个 Pull Request。

### 审查草稿和来自 fork 的 Pull Request

自动审查会跳过这两种情况，因此请使用评论命令请求审查：

* **草稿 Pull Request**：评论命令可以在草稿上运行，因此您可以在将 Pull Request 标记为就绪之前请求审查。即使使用了 `@claude review always`，推送到草稿也不会启动审查。订阅会在您将 Pull Request 标记为就绪后生效。
* **来自 fork 的 Pull Request**：无论仓库的 **Review runs** 设置如何，Claude 都不会自动审查来自 fork 的 Pull Request。要启动审查，请在 Pull Request 上评论 `@claude review`。[评论命令的条件](#comment-commands)仍然适用，并且您需要的是对基础仓库而非 fork 的写入权限。

在来自 fork 的 Pull Request 上，`@claude review always` 同样有效，但不会为该 Pull Request 订阅后续推送的审查。在那里点击检查运行上的 **Re-run** 不会启动审查。

### 将 Code Review 与 Claude Code GitHub Action 一起使用

如果仓库还以交互模式运行 [Claude Code GitHub Action](https://code.claude.com/docs/zh-CN/github-actions.md)，并使用其默认的 `@claude` 触发短语，那么 `@claude review` 评论也会启动该 Action。要将两者区分开，请将该 Action 的 `trigger_phrase` 输入设置为其他短语。

## 相关资源

* [Code Review 故障排除](https://code.claude.com/docs/zh-CN/code-review-troubleshooting.md)：未启动、失败或因预算而被跳过的审查
* [Code Review](https://code.claude.com/docs/zh-CN/code-review.md)：该服务是什么以及审查如何运行
* [设置 Code Review](https://code.claude.com/docs/zh-CN/code-review-setup.md)：供 Owner 使用，启用 Code Review 并选择审查的运行时机和级别
* [自定义 Code Review](https://code.claude.com/docs/zh-CN/code-review-customize.md)：使用 `REVIEW.md` 和 `CLAUDE.md` 更改 Claude 标记的内容
* [Code Review 费用和用量](https://code.claude.com/docs/zh-CN/code-review-costs.md)：审查如何计费以及在何处设置支出限额
* [使用 `/code-review` 在本地审查](https://code.claude.com/docs/zh-CN/code-review-local)：从 Claude Code 会话中对您的分支运行审查
