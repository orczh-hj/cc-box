> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Code Review 费用和用量

> 估算并限制您的组织在 Claude Code Review 上的支出：审查如何通过使用额度计费、哪些因素影响费用，以及在哪里跟踪用量。

[Code Review](https://code.claude.com/docs/zh-CN/code-review.md) 没有按次计算的固定价格。您的支出取决于每次审查的费用以及运行的审查次数：

* **每次审查的费用**：随[审查级别](https://code.claude.com/docs/zh-CN/code-review-setup.md#set-the-default-review-level)、变更的大小、相关代码的复杂度以及需要验证的候选问题数量而变化
* **审查次数**：由每个仓库中[审查的运行频率](#how-often-reviews-run)决定

## 定价

Code Review 通过[使用额度](https://support.claude.com/en/articles/12429409-extra-usage-for-paid-claude-plans)按 token 用量计费，与您套餐中包含的用量分开计算。审查费用计入您的组织，而不是计入开启 PR 或请求审查的开发者。无论您的组织是否将 Amazon Bedrock、Google Cloud 的 Agent Platform 或 Microsoft Foundry 用于其他 Claude Code 功能，费用都会显示在您的 Anthropic 账单上。

### 审查的运行频率

从管理页面添加的仓库默认设为 **After every push**，这种模式运行的审查次数最多。每个仓库的 [**Review runs** 设置](https://code.claude.com/docs/zh-CN/code-review-setup.md#set-when-reviews-run)决定一个 Pull Request 会获得多少次审查：

* **Once after PR creation**：在 Pull Request 被打开、重新打开或标记为可供审查时运行一次审查
* **After every push**：每次推送时运行，费用会按推送次数成倍增加
* **Manual (@claude review)**：打开或推送时不运行审查

评论命令、草稿以及来自 fork 的 Pull Request 会影响上述次数：

* **在 Once after PR creation 或 Manual 模式下使用 `@claude review always`**：[让该 PR 启用推送触发的审查](https://code.claude.com/docs/zh-CN/code-review-pull-requests.md#comment-commands)，因此在该评论之后每次推送都会产生额外费用
* **在 After every push 模式下使用 `@claude review always`**：推送本来就会触发审查，因此该订阅不会改变每次推送的费用
* **`@claude review`**：运行一次审查，但不订阅后续推送
* **草稿和来自 fork 的 Pull Request**：[自动审查会跳过这两者](https://code.claude.com/docs/zh-CN/code-review-pull-requests.md#review-drafts-and-pull-requests-from-forks)，因此向它们推送不会增加审查次数

## 估算您的支出

在大范围推广之前，先进行小规模试点，根据您自己的 Pull Request 估算每月支出。

1. 为试点设置[每月支出上限](#limit-your-spend)。
2. 为几个具有代表性的仓库启用 Code Review，并将 **Review runs** 设为 **Once after PR creation** 或 **Manual (@claude review)**。
3. 运行若干次审查后，在 [**Admin settings > Claude Code**](https://claude.ai/admin-settings/claude-code#code-review) 的 **Code review** 部分中，查看 **Repositories** 表的 **Average cost** 列，了解每个仓库每次审查的平均费用。
4. 将该平均值乘以该仓库每月打开的 Pull Request 数量。对于 **After every push**，还需再乘以每个 Pull Request 的典型推送次数。

仓库的平均费用会随其 Pull Request 的大小以及您选择的审查级别而变化。

## 限制您的支出

要为 Code Review 设置每月支出上限，请前往 [claude.ai/admin-settings/usage](https://claude.ai/admin-settings/usage)，为 Claude Code Review 服务配置上限。该上限适用于整个组织。

当以下任一项用尽时，Code Review 会跳过审查：

* **Code Review 支出上限**：您为 Claude Code Review 服务设置的每月上限。它会在下一个计费周期开始时重置；如果您提高上限，审查会更早恢复。
* **组织支出上限**：您的组织在使用额度上的整体每月支出上限。可在同一页面提高该上限。
* **使用额度余额**：您的组织的使用额度余额。可在同一页面添加额度。

当剩余额度低于一次审查的预期费用时，Code Review 也会跳过审查。审查被跳过时，开发者会看到一条 [PR 评论](https://code.claude.com/docs/zh-CN/code-review-troubleshooting.md#review-didn’t-run-and-the-pr-shows-a-budget-message)，其中说明原因并链接到用量设置。

要监控支出，请使用[分析仪表板](#view-usage)。

## 查看用量

前往 [claude.ai/analytics/code-review](https://claude.ai/analytics/code-review) 查看整个组织的 Code Review 活动。

| 部分 | 显示内容 |
| :- | :- |
| PRs reviewed | 所选时间范围内每天审查的 Pull Request 数量 |
| Code Review cost | 本月截至目前的 Code Review 支出 |
| Feedback | 因开发者解决了问题而被自动标记为已解决的审查评论数量 |
| Code Review findings | 所选时间范围内 Claude 发布的审查中，每天[按严重级别划分的发现](https://code.claude.com/docs/zh-CN/code-review-pull-requests.md#severity-levels)数量 |
| Repository breakdown | 每个仓库的已审查 PR 数、已解决评论数和审查运行次数，以及估算费用和按 PR 的视图 |

仅当选择当前月份时，Code Review cost 卡片才会显示金额。分析中的费用数据可能与您的发票不同。Repository breakdown 中的费用按标价估算，未计入任何折扣或额度，并且仅涵盖 Claude 在 Pull Request 上发布的审查。如需与发票一致的支出数据，请参阅您的 Anthropic 账单。

## 相关资源

* [设置 Code Review](https://code.claude.com/docs/zh-CN/code-review-setup.md)：在每个仓库中选择审查级别以及审查的运行时机
* [在 Pull Request 上阅读和请求审查](https://code.claude.com/docs/zh-CN/code-review-pull-requests.md)：评论命令、来自 fork 的 Pull Request，以及审查被跳过时开发者看到的预算消息
* [Code Review](https://code.claude.com/docs/zh-CN/code-review.md)：审查检查哪些内容以及发布哪些内容
* [有效管理费用](https://code.claude.com/docs/zh-CN/costs.md)：Claude Code 其他部分的支出控制和用量跟踪
