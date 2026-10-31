> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Code Review 故障排除

> 修复未启动、失败或因预算原因被跳过的 Code Review，查找缺失的发现项，并解决设置和 `REVIEW.md` 问题。

当 [Code Review](https://code.claude.com/docs/zh-CN/code-review.md) 在 Pull Request 上的表现不符合预期时，请先查看 **Claude Code Review** 检查运行。它的标题会说明审查进行到了哪一步。

## 查看审查进行到了哪一步

在您的 Pull Request 上找到 **Claude Code Review** 检查运行，然后将其标题与以下常见标题之一进行对照：

| 检查运行标题 | 含义 | 处理方法 |
| :- | :- | :- |
| **Code review queued...** 或 **Code review in progress...** | 审查尚未完成 | 等待标题变化 |
| **Code review found no issues** | 审查已完成，未发布任何发现项 | 如果这不是该 Pull Request 的首次审查，请在审查正文中查看[审查未包含的发现项](https://code.claude.com/docs/zh-CN/code-review-pull-requests.md#what-a-later-review-covers)的数量。否则无需操作 |
| 表示发现了问题的标题 | 审查已完成并有发现项 | 如果在 diff 上看不到这些发现项，请参阅[查找未显示为内联评论的问题](#find-issues-that-aren’t-showing-as-inline-comments) |
| **Code review skipped — no new changes** | 自动审查未在您最新的推送上运行 | 如果仍想获得审查，请评论 `@claude review` |
| **Code review not repeated — earlier findings still open** | 自动审查未在您最新的推送上运行，且之前审查中的 Important 发现项仍未解决 | 推送修复；如果仍想获得审查，请评论 `@claude review`。请参阅[推送保留之前结果的情况](https://code.claude.com/docs/zh-CN/code-review-pull-requests.md#when-a-push-keeps-the-earlier-result) |
| **Code review failed** 或 **Code review timed out** | 审查未完成 | 要重新运行，请参阅[重新触发失败或超时的审查](#retrigger-a-failed-or-timed-out-review) |
| **Code review declined** | 审查未运行 | 阅读检查运行的摘要。如果其中提到预算或支出限额，请参阅[审查未运行且 PR 显示预算消息](#review-didn’t-run-and-the-pr-shows-a-budget-message) |
| 没有检查运行 | 没有启动任何审查 | 评论 `@claude review`。如果之后仍未出现检查运行，请参阅[评论命令未启动审查](#a-comment-command-didn’t-start-a-review) |

## 审查未启动

当没有出现 **Claude Code Review** 检查运行，或 Claude 发布评论表示审查已被跳过时，请检查以下情况。

### 评论命令未启动审查

在您评论 `@claude review` 之后，请查看 Pull Request 上是否出现标题为 **Code review queued...** 或 **Code review in progress...** 的 **Claude Code Review** 检查运行。您的评论上出现 👀 表情回应并不能确认审查即将开始。如果没有出现检查运行，请逐一检查以下原因：

* **此仓库未启用 Code Review**：Claude 不会针对该命令发布任何评论或检查运行。请让 Owner [添加该仓库](https://code.claude.com/docs/zh-CN/code-review-setup.md#add-more-repositories)。
* **评论不满足某个条件**：评论是内联评论或审查正文，而不是新的顶层评论，或者命令不在评论开头。编辑先前的评论而不是发布新评论也不算数，并且 Pull Request 必须处于打开状态。请参阅[评论命令的条件](https://code.claude.com/docs/zh-CN/code-review-pull-requests.md#comment-commands)，并发布一条满足这些条件的新评论。
* **您没有写入权限**：您需要拥有该仓库的 write、maintain 或 admin 权限；对于来自 fork 的 Pull Request，起作用的是基础仓库上的权限。除非该 Pull Request 是您自己的，否则 Claude 可能会回复一条[提示](https://code.claude.com/docs/zh-CN/code-review-setup.md#turn-tip-comments-on-or-off)："No review was started: manual reviews can only be requested by someone with write access to this repository."。请让具有写入权限的人来发表评论。
* **预算已用尽**：Claude 会发布一条以 "⚠️ Code review skipped" 开头的评论。请参阅[审查未运行且 PR 显示预算消息](#review-didn’t-run-and-the-pr-shows-a-budget-message)。

### 审查未运行且 PR 显示预算消息

当您组织的预算不足以支付一次审查时，Code Review 会跳过该审查，Claude 会在 Pull Request 上发布一条以 "⚠️ Code review skipped" 开头的评论。在其中某些情况下，还会出现标题为 **Code review declined** 的检查运行。

该评论会链接到 [claude.ai/admin-settings/usage](https://claude.ai/admin-settings/usage)，管理员可在此恢复预算。评论的措辞会告诉您达到了哪项限额：

* **"has reached its monthly code review spending cap"**：已达到 [Code Review 的每月支出限额](https://code.claude.com/docs/zh-CN/code-review-costs.md#limit-your-spend)。审查将在下一个计费周期开始时恢复。如需更早恢复，请让管理员提高该限额。
* **"has no extra usage available to pay for this review"**：您组织的[使用额度](https://support.claude.com/en/articles/12429409-extra-usage-for-paid-claude-plans)余额已用完，或已达到其总体每月支出限额。请让管理员添加使用额度或提高该限额。
* **"is too low to start another review"**：剩余的预算或余额低于一次审查的预计费用。请让管理员提高限额或添加使用额度。

预算恢复后，在 Pull Request 上评论 `@claude review` 即可获得之前被跳过的审查。[评论命令的条件](https://code.claude.com/docs/zh-CN/code-review-pull-requests.md#comment-commands)同样适用，因此评论者需要拥有写入权限。

## 重新触发失败或超时的审查

如果检查运行的摘要显示 "The review session was interrupted before it could finish. A new review of this commit has been queued automatically."，则您无需执行任何操作。该审查会自动重试一次。

否则，当检查运行的标题为 **Code review failed** 或 **Code review timed out** 时，请通过评论或重新运行来再次运行审查：

* **评论 `@claude review`**：将其作为顶层评论发布在 Pull Request 上。这会启动一次全新的审查，而不会让该 Pull Request 订阅后续推送。
* **点击 Re-run**：如果该 Pull Request 不是[来自 fork](https://code.claude.com/docs/zh-CN/code-review-pull-requests.md#review-drafts-and-pull-requests-from-forks)，请在 GitHub 的 **Checks** 标签页中点击 **Claude Code Review** 检查上的 **Re-run**。重新运行会覆盖 Pull Request 当前的 head 提交，同样不会让该 Pull Request 订阅后续推送。

## 查找未显示为内联评论的问题

如果检查运行标题表示发现了问题，但您在 diff 上看不到内联审查评论，请查看以下其他展示发现项的位置：

* **检查运行 Details**：在 **Checks** 标签页中，点击 **Claude Code Review** 检查旁边的 **Details**。严重程度表会列出发现项及其文件、行号和摘要，无论内联评论是否被接受。
* **Files changed 注释**：打开 PR 上的 **Files changed** 标签页。有行号但未作为内联评论发布的发现项会以注释的形式显示在相关行上。
* **审查正文**：没有内联位置或位于当前 diff 之外的发现项，会显示在审查正文的 **Additional findings** 标题下。

## 检查运行提到 REVIEW\.md

检查运行可能携带的关于 `REVIEW.md` 的消息中，有以下四条：

* **"This review applied the repository's review instructions (REVIEW\.md) from its default branch, which is not protected against direct pushes."**：该审查仅读取了根目录下的文件。请参阅[从哪个分支读取 REVIEW.md](https://code.claude.com/docs/zh-CN/code-review-customize.md#which-branch-review-md-is-read-from)，了解保护该分支会带来哪些变化。
* **"were not applied to this review: branch protection could not be confirmed"**：该审查没有读取任何 `REVIEW.md`。
* **"they link to a file this pull request modifies, or carry more links than the review checks"**：该审查跳过了根目录下的 `REVIEW.md`，或跳过了消息中给出数量的子目录 `REVIEW.md` 文件。请针对以下每种原因检查被跳过的文件。对于根目录下的 `REVIEW.md`，还需检查[它所包含的文件](https://code.claude.com/docs/zh-CN/code-review-customize.md#include-other-files-in-review-md)：
  * **链接过多**：文件包含超过 64 个 Markdown 链接。请将链接削减至 [64 个或更少](https://code.claude.com/docs/zh-CN/code-review-customize.md#include-other-files-in-review-md)。
  * **审查无法解析的链接**：文件中有格式错误的 Markdown 链接。请修复或删除该链接。
  * **指向已更改文件的链接**：文件链接到此 Pull Request 更改的文件或文件夹，且审查无法仅排除包含该链接的行。请直接在 `REVIEW.md` 中编写该规则。
* **"include line(s) in the root REVIEW\.md were skipped on this review"**：根目录下的 `REVIEW.md` 已应用，但这些行所指定的文件未被应用。请查看[在 REVIEW.md 中包含其他文件](https://code.claude.com/docs/zh-CN/code-review-customize.md#include-other-files-in-review-md)下的列表；对于 skill，还需查看[包含 skill 文件](https://code.claude.com/docs/zh-CN/code-review-customize.md#include-a-skill-file)下的列表。

## 设置问题

以下情况会在 Owner [设置 Code Review](https://code.claude.com/docs/zh-CN/code-review-setup.md) 时出现。

### 仓库未在 Add a repository 中列出

请先按名称搜索该仓库，因为对话框不会完整加载较长的列表。只有当 Claude GitHub App 能够访问某个仓库时，对话框才会列出该仓库。点击 **Install GitHub App** 并授予该应用对该仓库的访问权限。当没有列出任何仓库时，**Install GitHub App** 是对话框中的一个按钮。否则，它位于对话框的 GitHub 组织筛选菜单底部。

### 测试 Pull Request 上没有出现检查运行

确认 **Allow code review** 已开启，该仓库已列在 **Code review** 部分的 **Repositories** 下，并且 Claude GitHub App 可以访问该仓库。自动审查会跳过草稿 Pull Request，而设置为 **Manual (@claude review)** 的仓库需要一条 `@claude review` 评论才能启动首次审查。

## 相关资源

* [在 Pull Request 上阅读和请求审查](https://code.claude.com/docs/zh-CN/code-review-pull-requests.md)：发现项和检查运行的含义，以及评论命令
* [设置 Code Review](https://code.claude.com/docs/zh-CN/code-review-setup.md)：供 Owner 使用，启用 Code Review 并管理仓库
* [Code Review 费用和使用情况](https://code.claude.com/docs/zh-CN/code-review-costs.md)：预算消息背后的支出限额
* [自定义 Code Review](https://code.claude.com/docs/zh-CN/code-review-customize.md)：审查读取的是哪一份 `REVIEW.md`
