> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# 在 Claude Code Desktop 中安排定期任务

> 在 Claude Code Desktop 中设置定期任务，以定期自动运行 Claude 进行日常代码审查、依赖项审计或早晨简报。

定期任务在您选择的时间和频率自动启动新会话。使用它们进行定期工作，如日常代码审查、依赖项更新检查或从您的日历和收件箱中提取信息的早晨简报。

Desktop 应用的 **Scheduled tasks** 页面会同时列出本地定时任务和[云端定时任务](https://code.claude.com/docs/zh-CN/routines.md)（原名 Routines）：

* **本地任务**：在您的机器上运行，可直接访问您的文件和工具，但仅在应用打开且计算机处于唤醒状态时才会触发
* **云端任务**：即使您的计算机关闭也可以运行，还可以通过 API 调用或 GitHub 事件启动

本页面涵盖本地定时任务；有关云端定时任务及其触发选项，请参阅[使用定时任务自动化工作](https://code.claude.com/docs/zh-CN/routines.md)。

> [!NOTE]
>
> 如果您的应用以其[旧名称](https://code.claude.com/docs/zh-CN/routines.md#where-the-name-routines-still-appears)显示此功能，请在本页面提到 **Scheduled** 的地方点击 **Routines**，在提到 **New task** 的地方点击 **New routine**。

## 比较调度选项

Claude Code 提供三种方式来安排定期或一次性工作：

| | [Cloud](https://code.claude.com/docs/zh-CN/routines.md) | [Desktop](https://code.claude.com/docs/zh-CN/desktop-scheduled-tasks.md) | [`/loop`](https://code.claude.com/docs/zh-CN/scheduled-tasks) |
| :- | :- | :- | :- |
| 运行位置 | Cloud，默认由 Anthropic 管理 | 您的机器 | 您的机器 |
| 需要机器开启 | 否 | 是 | 是 |
| 需要打开会话 | 否 | 否 | 是 |
| 重启后持久化 | 是 | 是 | 在 `--resume` 上恢复，有[例外](https://code.claude.com/docs/zh-CN/scheduled-tasks.md#limitations) |
| 访问本地文件 | 否（新克隆） | 是 | 是 |
| MCP servers | 每个任务配置的连接器 | [配置文件](https://code.claude.com/docs/zh-CN/mcp.md)和连接器 | 从会话继承 |
| 权限提示 | 默认为[自动模式](https://code.claude.com/docs/zh-CN/routines.md#permission-prompts-during-a-run) | 每个任务可配置 | 从会话继承 |
| 可自定义的计划 | 是 | 是 | 是 |
| 最小间隔 | 1 小时 | 由 Claude 设置时为 1 分钟，[在表单中为 1 小时](https://code.claude.com/docs/zh-CN/desktop-scheduled-tasks.md#schedule-options) | 1 分钟 |

> [!TIP]
>
> 对于应该在没有您的机器的情况下可靠运行的工作，使用**云任务**。当您需要访问本地文件和工具时，使用**桌面任务**。对于会话期间的快速轮询，使用 **`/loop`**。

> [!NOTE]
>
> 默认情况下，本地定时任务针对您的工作目录的任何状态运行，包括未提交的更改。如需为每次运行提供其自己的[隔离 Git worktree](https://code.claude.com/docs/zh-CN/desktop.md#run-a-session-in-its-own-worktree)，请在创建任务后，在该任务的[**Edit** 表单](#manage-scheduled-tasks)中选择 **Worktree**。当任务的文件夹是 Git 仓库时，该复选框可用。

## 创建定期任务

本地定时任务需要 Claude Desktop 1.1.5368 或更高版本。

**1. 打开 Scheduled tasks 页面**

在 [**Code** 选项卡](https://code.claude.com/docs/zh-CN/desktop.md)中，单击侧边栏中的 **Scheduled**，或侧边栏 **More** 菜单中的 **Scheduled**。如果侧边栏显示的是 **Routines** 而不是 **Scheduled**，请单击 **Routines**，然后单击 **New routine**，并继续参阅[整页表单](#full-page-form)。

**2. 开始新任务**

单击 **New task** 以打开 **New scheduled task** 对话框。在继续之前，请检查是否出现以下界面：

* 如果弹出包含 **Create with Claude** 和 **Set up manually** 的菜单，请选择 **Set up manually** 以打开该对话框。
* 如果 **Only on this computer** 位于对话框顶部，请在输入任何内容之前，转到[顶部带有 Only on this computer 的对话框](#dialog-with-only-on-this-computer-at-the-top)。
* 如果弹出包含 **Local** 和 **Cloud** 的菜单，或者打开的是整个页面而不是对话框，请转到[整页表单](#full-page-form)。

**3. 为任务命名**

在 **Name** 字段中输入名称。Desktop 会将其转换为小写 kebab-case，并将其用作磁盘上该任务的文件夹名称，因此该名称在您的所有任务中必须是唯一的。

**4. 编写指令**

在名称下方的输入框中，输入任务运行时 Claude 应该执行的操作。

**5. 启用 Only on this computer**

暂时跳过 **Trigger** 卡片，展开其下方的 **Advanced settings**。在 **General** 选项卡上，启用 **Only on this computer**。启用后，任务会在您的计算机上的本地文件夹中运行。

如果该开关不可用，请清除仓库或 GitHub 事件或 API 触发器，因为这些设置会使该任务成为[云端定时任务](https://code.claude.com/docs/zh-CN/routines.md)。

**6. 选择本地文件夹**

在 **Local folder** 行中，选择一个文件夹。在您选择文件夹之前，**Create** 将保持不可用。

**7. 设置计划**

返回 **Trigger** 卡片，选择一个[计划预设](#schedule-options)。

**8. 选择权限模式和模型**

在指令输入框的工具栏中，选择[权限模式](#permissions-for-scheduled-tasks)和模型。如果保持不变，运行时将使用您的 Claude Code 设置和默认模型。

**9. 创建任务**

单击 **Create**。如果您尚未信任该文件夹，Desktop 会在此时要求您信任它。随后将打开该任务的详情页面。

在详情页面上，单击铅笔图标以打开 [**Edit** 表单](#manage-scheduled-tasks)。在该表单中，您可以更改 Desktop 根据您的指令填写的 **Description**、选择 **Worktree**，或将计划切换为 **Manual** 或 **Custom**。

您也可以通过在任何会话中描述您想要的内容来创建任务。例如，"设置一个每天早上 9 点运行的日常代码审查"会创建一个周期性任务。像"提醒我明天下午 3 点检查部署"这样的请求会创建一个一次性任务，该任务在触发后会自行禁用。

### 顶部带有 Only on this computer 的对话框

如果 **New scheduled task** 对话框打开时 **Only on this computer** 位于顶部，请在输入任何内容之前先启用它。启用后，一个用于本地任务的独立空白表单将替换原表单。

然后填写本地表单的字段：

* **Name**：输入名称，与[创建定时任务的步骤](#create-a-scheduled-task)中相同。
* **Description**：输入描述。在您输入描述之前，**Create** 将保持不可用。
* **指令输入框**：输入任务运行时 Claude 应该执行的操作。从输入框中的菜单选择权限模式和模型。
* **Working folder**：选择任务运行所在的文件夹。在您选择文件夹之前，**Create** 将保持不可用。
* **Trigger**：选择计划。有关预设，请参阅[计划选项](#schedule-options)。

单击 **Create**。随后将打开该任务的详情页面。

### 整页表单

如果弹出包含 **Local** 和 **Cloud** 的菜单，请选择 **Local**。此时会打开标题为 **New local routine** 或 **New local scheduled task** 的页面。请填写其中的字段：

* **Name**：输入在您的所有任务中唯一的名称。
* **Description**：输入描述。
* **Instructions**：输入任务运行时 Claude 应该执行的操作。在同一输入框中，选择权限模式、模型以及任务运行所在的文件夹。
* **Schedule**：选择一个[计划选项](#schedule-options)。

如果 **Create** 不可用，请将鼠标悬停在其上方，查看表单还缺少哪些内容。单击 **Create**，随后将打开该任务的详情页面。

## 调度选项

当您在 **Advanced settings** 下启用 **Only on this computer** 时，**Trigger** 卡片提供 Hourly、Daily、Weekdays 和 Weekly 选项。若要为此类任务使用 **Manual** 或 **Custom**，请在创建任务后打开其 [**Edit** 表单](#manage-scheduled-tasks)。该表单、[顶部带有 **Only on this computer** 的对话框](#dialog-with-only-on-this-computer-at-the-top)以及[整页表单](#full-page-form)提供以下六个选项：

* **Hourly**：每小时运行一次
* **Daily**：每天在您选择的本地时间运行
* **Weekdays**：与 Daily 相同，但跳过星期六和星期日
* **Weekly**：显示时间选择器和日期选择器
* **Manual**：无调度，仅在您单击 **Run now** 时运行。适用于保存您按需触发的提示词
* **Custom**：按照您输入的 cron 表达式运行，该表达式以您的本地时间计算，例如 `0 9 * * *` 表示每天上午 9:00。星期和月份请使用数字表示，因为不接受其名称。星期日为 `0`，而不是 `7`。表单会拒绝运行频率高于每小时一次的表达式。

您也可以在任何 Desktop 会话中请 Claude 设置调度。Claude 不受每小时一次的限制，并且可以设置在未来某个时间的单次运行。例如，可以要求"每 30 分钟运行一次测试套件"或"下周一上午 9 点，总结自周五以来合并的 Pull Request"。

## 定期任务如何运行

本地定时任务在您的机器上运行。Desktop 在应用打开时每分钟检查一次调度，并在任务到期时启动一个新会话，独立于您打开的任何手动会话。每个任务在计划时间后会有几分钟的小延迟，以错开 API 流量。延迟是确定性的：同一任务总是在相同的偏移量处启动。

任务仅在 desktop 应用运行且计算机处于唤醒状态时运行。如果您的计算机在计划时间内进入睡眠状态，该运行将被跳过。要防止空闲睡眠，请在 **Settings > Desktop app** 或 **Settings > This computer > System** 中启用 **Keep computer awake**。关闭笔记本电脑盖仍会使其进入睡眠状态。

> [!NOTE]
>
> 对于需要在计算机关闭时运行或应该通过 API 调用或 GitHub 事件触发的任务，请改为创建[云端定时任务](https://code.claude.com/docs/zh-CN/routines.md)。

### 查找某次运行的会话

要打开某次运行所启动的会话，请前往该任务的[详情页面](#manage-scheduled-tasks)，然后在 **History** 列表中点击该次运行所在的行。

侧边栏的会话列表中也可以显示 **Scheduled** 分组，每个任务对应一行。要打开某个任务最近一次运行的会话，请在该分组中点击该任务所在的行。要显示该分组，请先[确认您使用的是哪种布局](https://code.claude.com/docs/zh-CN/desktop.md#check-which-layout-you-have)：

* **当前布局**：打开[**Group and sort** 菜单](https://code.claude.com/docs/zh-CN/desktop.md#organize-and-find-sessions)，选择 **Show**，然后选择 **Scheduled**。该分组默认处于折叠状态，因此请点击其标题将其展开。当您至少有一个任务处于 **Active** 状态（即任务详情页面所显示的 **Status**）时，**Show** 中才会列出 **Scheduled**。如果未列出，请使用 **History** 列表，无论该分组是否显示，它都可以使用。
* **早期布局**：该分组直接显示在会话列表中，无需启用任何选项

无论通过哪种方式进入某次运行的会话，您都可以查看 Claude 做了什么、审查更改并响应权限提示。Claude 可以编辑文件、运行命令、创建提交和打开 Pull Request，与您自己启动的会话相同。

## 错过的运行

当应用启动或计算机唤醒时，Desktop 会检查每个任务是否在过去七天内错过了任何运行。如果有，Desktop 会为最近错过的时间启动恰好一次追赶运行，并丢弃任何更早的运行。一个错过六天的日常任务在唤醒时运行一次。当追赶运行启动时，Desktop 会显示通知。

在编写提示词时请记住这一点。计划在上午 9 点运行的任务可能在晚上 11 点运行，如果您的计算机整天处于睡眠状态。如果时间很重要，请在提示词本身中添加护栏，例如："仅审查今天的提交。如果已经是下午 5 点之后，请跳过审查，只发布一份错过内容的摘要。"

## 定期任务的权限

每个任务都有自己的权限模式，您在创建或编辑任务时设置。来自 `~/.claude/settings.json` 的允许规则也适用于定时任务会话。如果任务在 [Manual 模式](https://code.claude.com/docs/zh-CN/desktop.md#choose-a-permission-mode)下运行并需要运行它没有权限的工具，运行将停滞，直到您批准它。要批准它，请[找到该运行的会话](#find-a-runs-session)并在其中回答权限提示。

为了避免停滞，在创建任务后单击 **Run now**，查看权限提示，并为每个提示选择"always allow"。该任务的未来运行会自动批准相同的工具，无需提示。您可以从任务的详细信息页面查看和撤销这些批准。

标记为 [`requiresUserInteraction`](https://code.claude.com/docs/zh-CN/mcp#require-approval-for-a-specific-tool) 的 MCP 工具在每次调用时都会提示，并且不提供"always allow"选项。调用这些工具的运行每次都会停滞。

## 管理定期任务

在 **Code** 选项卡中，单击侧边栏中的 **Scheduled**，然后单击列表中的任务以打开其详细信息页面。从这里您可以：

* **Run now**：立即启动任务，无需等待下一个计划时间
* **Status**：在 Active 和 Paused 之间切换，以暂停或恢复定期运行，而无需删除任务
* **Edit**：更改 instructions、schedule、folder 或其他设置
* **Review history**：查看每次过去的运行，包括跳过的运行。将鼠标悬停在跳过的条目上以查看原因：您的计算机处于睡眠状态、前一次运行仍在进行中，或其他定期任务已在运行。单击 **Show more** 以加载较早的条目。
* **Review allowed permissions**：从 **Always allowed** 面板查看和撤销此任务的已保存工具批准
* **Delete**：删除任务。该任务创建的会话会被[存档](https://code.claude.com/docs/zh-CN/desktop.md#organize-and-find-sessions)，而不是被删除。存放任务 `SKILL.md` 文件的文件夹会保留在磁盘上的 `~/.claude/scheduled-tasks/<task-name>/` 中；如果您设置了 [`CLAUDE_CONFIG_DIR`](https://code.claude.com/docs/zh-CN/env-vars) 变量，则保留在其下的 `scheduled-tasks/<task-name>/` 中。要删除该文件夹，请您自行删除。

您也可以通过在任何 Desktop 会话中询问 Claude 来列出、创建、编辑和暂停任务。例如，"pause my dependency-audit task"或"show me my scheduled tasks"。要删除任务，请使用其详细信息页面上的 **Delete** 按钮。

定期任务还可以使用 `update_scheduled_task` MCP 工具从运行中的会话内修改其自己的调度或提示。这让任务可以根据它发现的内容重新调度自己，例如，当它检测到发布分支已创建时，将代码审查重新调度为更早运行。

要在磁盘上编辑任务的提示，请打开 `~/.claude/scheduled-tasks/<task-name>/SKILL.md`（如果设置了 [`CLAUDE_CONFIG_DIR`](https://code.claude.com/docs/zh-CN/env-vars)，则在其下）。该文件使用 YAML frontmatter 表示 `name` 和 `description`，提示作为正文。更改在下一次运行时生效。Schedule、folder、model 和 enabled 状态不在此文件中：通过 Edit 表单更改它们或询问 Claude。

## 相关资源

* [云端定时任务](https://code.claude.com/docs/zh-CN/routines.md)：在云端按计划、通过 API 调用或响应 GitHub 事件运行任务，即使您的计算机关闭
* [Run prompts on a schedule](https://code.claude.com/docs/zh-CN/scheduled-tasks.md)：在 CLI 中使用 `/loop` 的会话范围调度
* [Claude Code GitHub Actions](https://code.claude.com/docs/zh-CN/github-actions.md)：在 CI 中按计划运行 Claude，而不是在您的机器上
* [Use Claude Code Desktop](https://code.claude.com/docs/zh-CN/desktop.md)：完整的 Desktop 应用指南
