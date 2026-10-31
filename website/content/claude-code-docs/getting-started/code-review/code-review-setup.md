> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# 设置 Code Review

> 为您的 Claude 组织启用 Code Review：连接 GitHub、安装 Claude GitHub App、选择仓库，并设置审查的运行时机

作为 Claude 组织的 Owner，您可以在 claude.ai 上的 [**Admin settings > Claude Code**](https://claude.ai/admin-settings/claude-code#code-review) 中启用 Code Review。审查在 Anthropic 基础设施上运行，因此无需向仓库中添加工作流文件。

设置完成后，您可以为每个仓库选择审查的细致程度及运行时机。审查级别越高、审查运行越频繁，花费就越多。请参阅 [Code Review 费用](https://code.claude.com/docs/zh-CN/code-review-costs.md#pricing)。

## 启用 Code Review 所需的条件

首先确认您的组织[符合 Code Review 的使用条件](https://code.claude.com/docs/zh-CN/code-review.md#availability)。然后确保您拥有以下访问权限，或有拥有这些权限的人员协助：

| 访问权限 | 用途 |
| :- | :- |
| Claude 组织中的 [Owner 角色](https://code.claude.com/docs/zh-CN/server-managed-settings.md#access-control) | 管理页面上的每个步骤 |
| GitHub 组织的所有者 | 安装 Claude GitHub App 并[将其关联到您的 Claude 组织](https://code.claude.com/docs/zh-CN/admin-setup.md#connected-github-accounts)。如果您不是所有者，请在开始前让一位所有者安装并关联该应用 |
| 从您连接到 Claude 的 GitHub 账户对要添加的每个仓库拥有写入权限 | 添加仓库 |

## 启用 Code Review

**1. 点击 Allow code review 开关**

前往 [**Admin settings > Claude Code**](https://claude.ai/admin-settings/claude-code#code-review) 的 **Code review** 部分，点击 **Allow code review** 开关。

如果 Code Review 立即启用且没有弹出对话框，说明您的组织已经关联了 Claude GitHub App 或已添加了仓库。请前往[添加更多仓库](#add-more-repositories)，然后再返回完成最后一步。

**2. 连接您的 GitHub 账户**

如果打开了 **Connect GitHub** 对话框，请点击 **Connect**。如果您的组织使用 GitHub Enterprise Server，请先在对话框中选择您的实例。GitHub 会要求您授权 Claude，然后将您带回管理页面，并打开 **Add a repository** 对话框。

如果直接打开了 **Add a repository**，说明您的账户已经连接，请继续下一步。

**3. 安装 Claude GitHub App**

如果所需的仓库已列在 **Add a repository** 对话框中，或者您可以在其中搜索到它们，请继续下一步。只有 Claude GitHub App 能够访问的仓库才会出现在该对话框中，因此如果找不到这些仓库，请安装该应用：

* 点击 **Install GitHub App**。当没有列出任何仓库时，它是对话框中的一个按钮；否则，它位于对话框中 GitHub 组织筛选菜单的底部。GitHub 的安装页面会在新标签页中打开。
* 选择拥有这些仓库的 GitHub 组织。
* 选择该应用可以访问的仓库。
* 批准所请求的权限。这组权限比审查实际使用的范围更广，并由其他 Claude 功能共享，例如 [GitHub Actions](https://code.claude.com/docs/zh-CN/github-actions.md)。请参阅[审查可以访问的内容](https://code.claude.com/docs/zh-CN/code-review.md#what-a-review-can-access)以及[完整权限列表](https://code.claude.com/docs/zh-CN/github-actions.md#github-app-permissions)。
* 安装完成后，该标签页会返回 Claude。如果显示 **Finish connecting GitHub** 按钮，请点击它，将此次安装关联到您的 Claude 组织。
* 返回管理页面所在的标签页，对话框中的列表会随之刷新。

**4. 选择仓库**

选择需要审查的仓库，然后点击 **Continue**。添加仓库需要您连接的 GitHub 账户对该仓库拥有写入权限。您可以稍后[添加更多仓库](#add-more-repositories)。

**5. 点击 Finish**

随后会出现 **Code review is set up** 对话框。点击 **Finish** 以启用 Code Review。如果您以其他方式关闭该对话框，仓库仍会保持已添加状态，但 **Allow code review** 会保持关闭，您需要通过该开关将其启用。

每个仓库初始设置为每次推送后都进行审查，这会运行最多的审查。如需减少审查次数，请[设置审查的运行时机](#set-when-reviews-run)。

**6. 通过测试 Pull Request 进行验证**

在您添加的某个仓库中打开一个测试 Pull Request，且不要将其标为草稿。几分钟内会出现一个名为 **Claude Code Review** 的检查运行。如果您已将该仓库设置为 **Manual (@claude review)**，请在 Pull Request 上评论 `@claude review` 以启动首次审查。

如果没有出现检查运行，请参阅[测试 Pull Request 上没有出现检查运行](https://code.claude.com/docs/zh-CN/code-review-troubleshooting.md#no-check-run-appears-on-the-test-pull-request)。

审查结果出现后，请将[在 Pull Request 上阅读和请求审查](https://code.claude.com/docs/zh-CN/code-review-pull-requests.md)发送给您的开发人员，其中介绍了严重级别、评论命令以及如何处理审查发现的问题。

## 为组织设置默认值

作为 Owner，您可以决定审查的细致程度和运行频率。这两项都位于 [**Admin settings > Claude Code**](https://claude.ai/admin-settings/claude-code#code-review) 的 **Code review** 部分。

### 设置默认审查级别

审查级别决定每次审查的细致程度，审查越细致，费用越高。您可以为组织设置一个默认值，并为单个仓库覆盖该值。

| 级别 | 权衡取舍 |
| :- | :- |
| Quick | 较轻量的检查。速度最快，费用最低 |
| Standard | 在深度、速度和费用之间取得平衡。默认级别 |
| Deep | 最细致的检查。耗时最长，费用最高 |

* **组织默认值**：在 **Review level** 下选择一个级别，然后点击 **Change level** 确认。除非为某个仓库单独设置级别，否则所有仓库都使用该默认值。
* **单个仓库**：点击该仓库所在行的 **Edit**，然后在其中选择 **Review level**。第一个选项 **Default** 遵循组织设置。选择后级别会立即保存。

新级别适用于更改之后开始的审查。有关影响每次审查费用的其他因素，请参阅 [Code Review 费用和用量](https://code.claude.com/docs/zh-CN/code-review-costs.md)。

### 设置审查的运行时机

每个仓库都有各自的 **Review runs** 设置，用于决定何时审查该仓库的 Pull Request。

* **Once after PR creation**：在 Pull Request 被打开、重新打开或标记为可供审查时运行一次审查
* **After every push**：在向 Pull Request 分支推送时运行审查，随着 Pull Request 的演进捕获新问题，并在您修复所指出的问题后自动解决相应讨论串
* **Manual (@claude review)**：打开 Pull Request 或向其推送不会启动审查。评论 [`@claude review`](https://code.claude.com/docs/zh-CN/code-review-pull-requests#comment-commands) 以请求审查，或评论 `@claude review always` 以同时让该 Pull Request 在后续推送时也接受审查。

对于流量较大的仓库，如果您希望只让特定的 Pull Request 接受审查，或仅在 Pull Request 准备就绪后才开始审查，Manual 会很有用。要比较每个选项对花费的影响，请参阅[审查的运行频率](https://code.claude.com/docs/zh-CN/code-review-costs.md#how-often-reviews-run)。

要更改此设置，请点击 **Repositories** 表中该仓库所在行的 **Edit**，然后选择一个 **Review runs** 选项。选择后会立即保存，表中的 **Review runs** 列会以缩写形式显示为 Once per PR、Every push 或 Manual。

* **草稿以及来自复刻的 Pull Request**：无论您选择哪个选项，[自动审查都会跳过这两类](https://code.claude.com/docs/zh-CN/code-review-pull-requests.md#review-drafts-and-pull-requests-from-forks)
* **新仓库**：从 **Add a repository** 对话框添加的仓库初始设置为 **After every push**。如果其所属 GitHub 组织已[整体覆盖](#cover-a-whole-github-organization)，则改为采用该组织行的设置。

### 开启或关闭提示评论

提示评论是在未启动审查时 Claude 在 Pull Request 上发布的简短评论，用于说明原因以及下一步该怎么做。**Show tips on pull requests** 默认开启。开启后，Claude 会在以下每种情况下发布提示，说明：

* **仓库设置为 Manual**：如何通过评论请求审查
* **Pull Request 来自复刻**：维护者可以请求审查
* **机器人，或者既没有写入权限、也不是该 Pull Request 作者的人，评论了 `@claude review`**：未启动审查，以及谁可以请求审查
* **GitHub 组织未关联到您的 Claude 组织**：审查已暂停，以及如何进行关联

关闭此设置即可停止发布这些评论。

### 开发人员可以更改的内容

开发人员如需更改审查级别、**Review runs** 设置或接受审查的仓库，需要联系您。在没有 Owner 角色的情况下，他们可以在 GitHub 上执行以下操作：

* **请求审查**：在 Pull Request 上评论 [`@claude review`](https://code.claude.com/docs/zh-CN/code-review-pull-requests#comment-commands)。这需要对仓库拥有写入权限，并且适用于自动审查会跳过的[草稿以及来自复刻的 Pull Request](https://code.claude.com/docs/zh-CN/code-review-pull-requests.md#review-drafts-and-pull-requests-from-forks)。
* **让单个 Pull Request 在每次推送时接受审查**：在[既非草稿也非来自复刻](https://code.claude.com/docs/zh-CN/code-review-pull-requests.md#review-drafts-and-pull-requests-from-forks)的 Pull Request 上评论 `@claude review always`，例如在设置为 Manual 的仓库中
* **更改审查所指出的内容**：向仓库提交 [`REVIEW.md` 和 `CLAUDE.md` 文件](https://code.claude.com/docs/zh-CN/code-review-customize)，例如添加一项检查

开发人员请求的审查会[计费到您的组织](https://code.claude.com/docs/zh-CN/code-review-costs.md#pricing)。要限制 Code Review 每月的花费，请参阅[限制您的花费](https://code.claude.com/docs/zh-CN/code-review-costs.md#limit-your-spend)。

## 管理仓库

您可以逐个添加仓库，也可以一次性覆盖整个 GitHub 组织，并在不需要的地方停止审查。

### 添加更多仓库

要在设置完成后添加仓库，请点击 **Code review** 部分中 **Repositories** 表上方的 **Add**。**Add a repository** 对话框随即打开。选择仓库并点击 **Continue**。

* **某个仓库未列出**：请参阅[某个仓库未在 Add a repository 中列出](https://code.claude.com/docs/zh-CN/code-review-troubleshooting.md#a-repository-isn’t-listed-in-add-a-repository)。
* **您希望包含某个 GitHub 组织中的所有仓库**：请[覆盖整个组织](#cover-a-whole-github-organization)，而不是逐个选择仓库

### 覆盖整个 GitHub 组织

要为某个 GitHub 组织中的所有仓库启用 Code Review，请在 **Repositories** 表中该组织所在行设置 **Review runs**。这会覆盖该组织中 Claude GitHub App 可以访问的所有仓库，包括之后添加的仓库。这可能会显著增加花费。需要同时满足以下所有条件：

* 您连接到 Claude 的 GitHub 账户是该 GitHub 组织的所有者。
* 该组织位于 github.com 上。GitHub Enterprise Server 实例和个人 GitHub 账户不符合条件。
* 已为您的 Claude 组织或 Code Review 设置了[每月花费上限](https://code.claude.com/docs/zh-CN/code-review-costs.md#limit-your-spend)。

要覆盖该组织，请点击其所在行的 **Edit**，选择一个 **Review runs** 选项并确认。可选项为[设置审查的运行时机](#set-when-reviews-run)中的三个选项，外加 **Off**。

确认后，以下更改将生效：

* **覆盖范围**：每个仓库从下一个 Pull Request 开始被覆盖。如果 Claude GitHub App 安装时只选择了部分仓库而非全部仓库，Claude 会审查这些仓库；要覆盖其余仓库，请在 GitHub 上更改该应用的仓库访问权限。
* **现有设置**：该 GitHub 组织中的每个仓库都会采用您选择的选项，包括您单独设置过的仓库。再次设置该行也会产生相同效果。在各自行中被关闭的仓库会保持关闭。
* **之后的单独更改**：之后设置得与其所属组织不同的仓库，会在表中标注为 **Custom**。
* **审查级别**：组织行本身没有审查级别。

### 关闭 Code Review 或移除仓库

在 [**Admin settings > Claude Code**](https://claude.ai/admin-settings/claude-code#code-review) 的 **Code review** 部分，根据您要停止的内容选择相应的控件：

* **单个仓库，保留在列表中**：点击其所在行的 **Edit**，然后关闭 **Code review**。表中会将该仓库显示为 Off。
* **单个仓库，将其移除**：点击其所在行的 **Edit**，然后点击对话框底部的 **Remove repository**。该仓库的审查将停止，之后其他 Claude 组织就可以设置该仓库。当该仓库所属的 GitHub 组织被整体覆盖时，无法将其移除；请改为关闭其 **Code review** 开关。
* **整个 GitHub 组织**：点击该组织所在行的 **Edit**，然后选择 **Off**。这也会关闭该组织中您逐个添加的仓库。
* **全部**：关闭 **Allow code review**。所有仓库都不会再启动新的审查，您的仓库列表会被保留，以便日后重新启用。

## 相关资源

* [Code Review 费用和用量](https://code.claude.com/docs/zh-CN/code-review-costs.md)：审查级别和审查运行时机如何影响花费、如何设置花费上限，以及在哪里查看用量
* [在 Pull Request 上阅读和请求审查](https://code.claude.com/docs/zh-CN/code-review-pull-requests.md)：开发人员在 Pull Request 上看到的内容以及评论命令
* [自定义 Code Review](https://code.claude.com/docs/zh-CN/code-review-customize.md)：使用 `REVIEW.md` 和 `CLAUDE.md` 调整 Claude 指出的内容
* [GitHub Enterprise Server](https://code.claude.com/docs/zh-CN/github-enterprise-server.md)：在设置前连接自托管的 GitHub 实例
