> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# 使用定时任务自动化工作

> 让 Claude Code 自动运行。创建定时任务（原名 Routine），使其在云端按计划、通过 API 调用或响应 GitHub 事件运行。

> [!NOTE]
>
> 定时任务目前为研究预览版。其行为、限制和 API 接口可能会发生变化。

云端定时任务（原名 Routine）是一个已保存的 Claude Code 配置：包含一个提示词、一个或多个仓库以及一组[连接器](https://code.claude.com/docs/zh-CN/mcp.md)，一次打包后即可自动运行。定时任务在 Anthropic 管理的云基础设施上执行，或在路由到您组织的[自托管环境](https://code.claude.com/docs/zh-CN/self-hosted-environments)时在该环境中执行，因此即使您的笔记本电脑已合上，它们也能继续工作。

每个定时任务可以附加一个或多个触发器：

* **Schedule**：按照每小时、每晚或每周等定期节奏运行，或在未来某个特定时间运行一次
* **API**：通过向该任务专属的端点发送带有 Bearer 令牌的 HTTP POST 请求来按需触发
* **GitHub**：自动响应 Pull Request 或发布等仓库事件而运行

单个定时任务可以组合多个触发器。例如，一个 PR 审查任务可以每晚运行、从部署脚本触发，同时还对每个新 PR 做出响应。

定时任务不一定需要计划：

* 仅有 API 触发器的任务会在您调用其端点时运行。
* 仅有 GitHub 触发器的任务会在发生匹配的仓库事件时运行。

定时任务适用于 Pro、Max、Ultra、Team 和 Enterprise 套餐。您可以在 [claude.ai/code/routines](https://claude.ai/code/routines) 创建和管理它们，也可以在 CLI 中使用 `/schedule`。旧名称 Routine [仍会出现](#where-the-name-routines-still-appears)。

> [!NOTE]
>
> 另有两项功能会在您的计算机上而非云端安排工作：[Desktop 定时任务](https://code.claude.com/docs/zh-CN/desktop-scheduled-tasks.md)和 [`/loop`](https://code.claude.com/docs/zh-CN/scheduled-tasks)。要在这三者之间进行选择，请参阅[比较调度选项](https://code.claude.com/docs/zh-CN/scheduled-tasks.md#compare-scheduling-options)。

Team 和 Enterprise 的 Owner 可以通过 [**Organization settings > Capabilities > Remote sessions**](https://claude.ai/admin-settings/capabilities) 中的 **Scheduled tasks (Was Routines)** 开关为所有成员关闭定时任务。关闭该开关后，现有定时任务将停止运行，成员也无法创建新的定时任务。

如果您的 Enterprise 组织通过[自定义角色](https://support.claude.com/en/articles/13930452)管理您的访问权限，您还需要一个包含定时任务权限的角色。Owner 可以在 [**Organization settings > Roles**](https://claude.ai/admin-settings/roles) 中角色的 **Remote sessions** 部分添加该权限。

## 示例用例

每个示例都将一种触发器类型与适合定时任务的工作类型相匹配：无人值守、可重复且有明确结果。

* **积压工作维护**：一个计划触发器每个工作日晚上在您的仓库中运行，并将您的问题跟踪器添加为连接器。定时任务读取自上次运行以来新开的问题，添加标签，并根据所引用的代码区域分配负责人。然后它会将摘要发布到 Slack，让团队以整理好的队列开始新的一天。
* **警报分诊**：当错误超过阈值时，您的监控工具会调用定时任务的 API 端点，并将警报正文作为 `text` 传入。任务的提示词指示 Claude 调查触发负载中的警报，因此它会提取堆栈跟踪，并将其与仓库中的最近提交相关联。然后它会打开一个草稿 Pull Request，其中包含建议的修复方案和指回该警报的链接。值班人员只需审查该 PR，而不必从空白终端开始。
* **定制代码审查**：一个 GitHub 触发器在 `pull_request.opened` 时运行。定时任务应用您团队自己的审查清单，并针对安全、性能和风格问题留下行内评论。它还会添加一条摘要评论，让人工审查者可以专注于设计，而不是机械性检查。
* **部署验证**：您的 CD 流水线在每次生产部署后调用定时任务的 API 端点。任务针对新构建运行冒烟检查，扫描错误日志以查找回归问题，并在部署窗口关闭前向发布频道发布可行或不可行的结论。
* **文档偏差检查**：一个计划触发器每周运行。定时任务扫描自上次运行以来合并的 PR，标记引用了已更改 API 的文档，并针对文档仓库打开更新 PR 供编辑审查。
* **库移植**：一个 GitHub 触发器在 `pull_request.closed` 时运行，并筛选为某个 SDK 仓库中已合并的 PR。定时任务将该更改移植到另一种语言的对应 SDK 中并打开一个相应的 PR，使两个库保持同步，而无需人工重新实现每项更改。

## 创建定时任务

您可以在 Web 端的 [claude.ai/code/routines](https://claude.ai/code/routines)、Desktop 应用或 CLI 中创建定时任务。这三个使用入口都写入同一个云端账户，因此您在其中一处创建的定时任务会立即显示在其他位置。在 Desktop 应用的 **Code** 标签页中，点击侧边栏中或侧边栏 **More** 菜单中的 **Scheduled**，然后点击 **New task**，并继续按照[从 Web 创建](#create-from-the-web)进行操作。

定时任务作为完整的 Claude Code 云端会话自主运行。会话会运行 shell 命令、使用 [skill](https://code.claude.com/docs/zh-CN/skills.md)，并调用您包含的任何连接器。在[项目](https://code.claude.com/docs/zh-CN/claude-projects.md#what-threads-pick-up-from-your-repositories)的线程之外，这些 skill 包括提交到所克隆仓库中的 skill。创建任务时无需选择权限模式：有关运行何时会停下来等待您，请参阅[运行期间的权限提示](#permission-prompts-during-a-run)。

定时任务可以访问的内容由以下各项决定：

* 您选择的仓库
* [环境](https://code.claude.com/docs/zh-CN/cloud-environments.md)的网络访问和变量
* 您包含的连接器

请将每一项的范围限定在任务所需的内容之内。

定时任务属于您的个人 claude.ai 账户。它们不会与队友共享，其运行计入您账户的[用量和限制](#usage-and-limits)。定时任务通过您已连接的 GitHub 身份或连接器执行的任何操作都会显示为由您执行。提交和 Pull Request 会带有您的 GitHub 用户，Slack 消息、Linear 工单或其他连接器操作会使用您为这些服务关联的账户。

### 从 Web 创建

**1. 打开创建对话框**

访问 [claude.ai/code/routines](https://claude.ai/code/routines) 并点击 **New task**，以打开 **New scheduled task** 对话框。继续之前，请留意以下界面：

* 如果按钮显示为 **New routine** 而不是 **New task**，请点击 **New routine**，并继续按照[完整页面表单](#full-page-form)进行操作。
* 在 Desktop 应用中，如果打开了包含 **Local** 和 **Cloud** 的菜单，请转到[完整页面表单](#full-page-form)。
* 如果打开了包含 **Create with Claude** 和 **Set up manually** 的菜单，请选择 **Set up manually** 以打开对话框。**Create with Claude** 则允许您在聊天中描述任务，由 Claude 为您创建。
* 如果打开的是完整页面而不是对话框，请转到[完整页面表单](#full-page-form)。

如果对话框打开时已显示 **Repositories** 和 **Environment**，请参阅[该对话框有何不同](#dialog-with-repositories-and-environment-in-the-main-view)。否则，此对话框也可用于创建 claude.ai 聊天的定时任务。要将您的任务设为对话框中所称的 Claude Code 任务，请在后续步骤中选择一个仓库，或添加 GitHub 事件触发器或 API 触发器。此类任务会在云端会话中运行 Claude Code。

**2. 命名任务并编写提示词**

为定时任务取一个描述性名称，并编写 Claude 每次运行的提示词。提示词是最重要的部分：任务是自主运行的，因此提示词必须是自包含的，并明确说明要做什么以及怎样算成功。

当触发器触发时，会话会将已保存的提示词作为其分配的任务接收并执行，而不是将其视为在对话中途到达的不受信任内容。触发器仅证明该提示词是由您账户上的授权会话预先存储的。因此，触发的提示词不是实时用户输入，不能充当运行期间操作的批准或同意。会话在运行期间获取的内容仍按正常方式处理。

**3. 选择触发器**

在 **Trigger** 行中，选择任务的启动方式。要组合多个触发器，请点击该行末尾的 **+**。

> [tab] Schedule
>
> 为定期运行选择一个预设频率。有关时区处理、延迟启动、自定义 cron 间隔和一次性运行，请参阅[添加计划触发器](#add-a-schedule-trigger)。

> [tab] GitHub event
>
> 选择仓库、要响应的事件以及可选的筛选条件。有关支持的事件和筛选字段的完整列表，请参阅[添加 GitHub 触发器](#add-a-github-trigger)。

> [tab] API
>
> 在此处选择 **API**。当您点击 **Create** 时，**API token** 对话框会显示一次令牌，因此请在关闭对话框之前复制它。要复制 URL 或生成新令牌，请参阅[添加 API 触发器](#add-an-api-trigger)。

**4. 选择仓库**

展开 **Advanced settings**。在 **General** 标签页的 **Repository** 下，选择一个或多个供 Claude 在其中工作的 GitHub 仓库。

每个仓库都会在运行开始时被克隆。Claude 会为其更改创建带有 `claude/` 前缀的分支。

**5. 选择环境**

选择仓库后，其下方会出现 **Cloud environment**。为任务选择一个[云环境](https://code.claude.com/docs/zh-CN/cloud-environments.md)。环境控制云端会话可以访问的内容：

* **Network access**：设置每次运行期间可用的互联网访问级别
* **Environment variables**：提供 Claude 在每次运行期间可以使用的值。它们[对使用该环境的任何人都可见](https://code.claude.com/docs/zh-CN/cloud-environments.md#what-carries-over-from-your-setup)，因此在 Anthropic 托管的环境中，请改为将 Claude 在运行期间调用的 API 的密钥存储为[网络密钥](https://code.claude.com/docs/zh-CN/cloud-environments.md#add-network-secrets)。该部分还列出了永远不会获得密钥的请求
* **Setup script**：安装任务所需的依赖和工具。结果会被[缓存](https://code.claude.com/docs/zh-CN/cloud-environments.md#environment-caching)，因此脚本不会在每个会话中重新运行

系统提供了一个 **Default** 环境，具有 **Trusted** 网络访问，仅允许[默认允许列表](https://code.claude.com/docs/zh-CN/cloud-environments.md#default-allowed-domains)中的包注册表、云提供商 API、容器注册表和常见开发域名通过会话的网络。您添加到任务中的连接器通过 Anthropic 的服务器访问其服务，因此无需更改允许列表。

如果您的任务需要直接访问您自己的服务或该列表之外的域名，请在运行前编辑环境的[网络访问](https://code.claude.com/docs/zh-CN/cloud-environments.md#network-access)。要使用单独的环境，请先[创建一个](https://code.claude.com/docs/zh-CN/cloud-environments.md#configure-your-environment)。

**6. 选择模型和 effort 级别**

在提示词输入框的模型选择器中选择一个模型，供 Claude 在每次运行时使用。对于 Claude Code 任务，如果该模型提供 [effort 级别](https://code.claude.com/docs/zh-CN/model-config.md#adjust-effort-level)，同一选择器中的 **Effort** 会显示当前级别。要更改级别，请选择其他级别。选择器的 **Default** 选项不提供任何级别。

**7. 检查连接器**

切换到 **Advanced settings** 的 **Connectors** 标签页。默认包含您所有已连接的 [MCP 连接器](https://code.claude.com/docs/zh-CN/mcp.md)。请移除任务不需要的连接器：Claude 可以在运行期间使用所包含的连接器，包括写入操作。有关运行何时会停下来先询问您，请参阅[运行期间的权限提示](#permission-prompts-during-a-run)。

**8. 创建任务**

点击 **Create**。对话框关闭后，新任务的详情页将打开。任务会在其某个触发器下次匹配时运行。要立即启动一次运行，请在详情页上点击 **Run now**。

要查看某次运行执行了哪些操作，请在详情页的 **History** 列表中点击该运行。该运行会[以完整会话的形式打开](#view-and-interact-with-runs)（每次运行对应一个新会话），您可以在其中查看 Claude 执行了哪些操作、审查更改并创建 Pull Request。

#### 主视图中包含 Repositories 和 Environment 的对话框

按照[从 Web 创建的步骤](#create-from-the-web)操作，并在对话框的主视图中（而不是在 **Advanced settings** 下）填写各个字段。此对话框仅创建 Claude Code 任务。

请在添加 GitHub 事件触发器之前选择仓库，因为在选择仓库之前该触发器不可用。

#### 完整页面表单

在 Desktop 应用中，如果打开了包含 **Local** 和 **Cloud** 的菜单，请选择 **Cloud**。随后会打开一个标题为 **New routine** 或 **New scheduled task** 的页面。填写其中的字段：

* **Name**：输入一个名称。
* **Instructions**：输入提示词。在同一个输入框中，选择模型、effort 级别（如果该模型提供）、仓库和环境。有关每一项所控制的内容，请参阅[从 Web 创建](#create-from-the-web)下的步骤。
* **Select a trigger**：选择 **Schedule**、**GitHub event** 或 **API**。对于计划，请参阅[添加计划触发器](#add-a-schedule-trigger)。对于 [GitHub 事件](#add-a-github-trigger)，请选择一个仓库。对于 [API 触发器](#add-an-api-trigger)，请从点击 **Create** 后打开的 **API token** 对话框中复制令牌，因为它只显示一次。要组合多个触发器，请点击 **Add another trigger**。
* **Connectors**：默认包含您所有已连接的连接器，因此请移除任务不需要的连接器。

如果 **Create** 不可用，请将鼠标悬停在其上，查看表单还需要填写哪些内容。点击 **Create**，任务的详情页随即打开。

### 从 CLI 创建

在任意会话中运行 `/schedule`，即可通过对话方式创建定时任务。您也可以直接传入描述，例如定期任务 `/schedule daily PR review at 9am` 或一次性任务 `/schedule clean up feature flag in one week`。Claude 会逐步收集与 Web 对话框相同的信息，然后将任务保存到您的账户。该命令也可以通过别名 `/routines` 使用。

成功启动时看起来像一次对话：Claude 会在保存前询问有关计划、仓库和提示词的后续问题。如果 Claude 反而回复说您需要进行身份验证，或者无法连接到您的远程 claude.ai 账户，则表示未创建定时任务；请参阅[故障排除](#troubleshooting)。

使用 `/schedule` 创建的任务带有计划触发器。您还可以添加其他类型的触发器：

* **API 触发器**：在 Web 端的 [claude.ai/code/routines](https://claude.ai/code/routines) 编辑任务
* **[GitHub 触发器](#add-a-github-trigger)**：在 Web 端编辑任务，或在 CLI 中询问 Claude。从 CLI 添加需要 Claude Code v2.1.225 或更高版本

没有计划触发器的定时任务（例如仅由 API 调用或 GitHub 事件启动的任务）没有下次运行时间。当 Claude 保存或更新此类任务时，CLI 不会显示下次运行时间。

## 配置触发器

当定时任务的某个触发器匹配时，任务即会启动。您可以随时在任务[编辑对话框](#edit-and-control-scheduled-tasks)的 **Trigger** 行中添加或移除任务的触发器。

当一个任务有两个或更多触发器时，每个触发器在该对话框中都有一个垃圾桶图标。要移除计划触发器或 GitHub 事件触发器，请点击其垃圾桶图标，然后点击 **Save changes**。要移除 API 触发器，请改为[删除其令牌](#add-an-api-trigger)。

### 添加计划触发器

计划触发器按定期节奏运行任务，或在未来某个特定时间运行一次。在 Web 端任务的创建或编辑对话框中，选择一个预设频率，例如每小时、每天或每周：

* **整点运行**：如果您将运行安排在整点（例如 9:00），它可能会晚几分钟才开始。要在接近计划时间时开始，请选择整点后几分钟，例如 9:07。
* **自定义间隔**：如果预设中没有您需要的节奏，请先创建任务，然后在其[编辑对话框](#edit-and-control-scheduled-tasks)中选择 **Custom**。接着输入 cron 表达式，例如 `0 */2 * * *` 表示每两小时运行一次。要在 CLI 中设置，请运行 [`/schedule update`](#manage-scheduled-tasks-from-the-cli)。运行频率高于每小时一次的表达式会被拒绝。
* **时区**：选择预设频率时，请以您的本地时区输入时间。
* **Cron 表达式时区**：当表达式仅由五个字段组成时，请以 UTC 填写小时，因此 `0 9 * * 1-5` 会在工作日的 UTC 上午 9:00 运行。如果该字段以 `CRON_TZ=` 前缀开头，请保留该前缀并以其指定的时区填写小时，或者删除该前缀并以 UTC 填写小时。

#### 安排一次性运行

一次性计划会在特定时间戳运行任务一次。例如，您可以用它来：

* 在本周晚些时候提醒自己
* 在发布完成后打开一个清理 PR
* 在上游更改发布后启动后续工作

任务运行后，它会自行关闭，Web UI 会将其标记为 **Completed**。要再次运行它，请在 Web 端打开任务的[编辑对话框](#edit-and-control-scheduled-tasks)，并选择 **One-time** 或 **Once**（以您的对话框中显示的标签为准）。然后以您的本地时区设置新的日期和时间并保存。

在 CLI 中用自然语言描述时间即可创建一次性运行。Claude 会根据当前时间解析该短语，并在保存前确认绝对时间戳。

```text theme={null}
/schedule tomorrow at 9am, summarize yesterday's merged PRs
```

```text theme={null}
/schedule in 2 weeks, open a cleanup PR that removes the feature flag
```

一次性运行与其他计划运行计入相同的每小时限制。详情请参阅[用量和限制](#usage-and-limits)。

### 添加 API 触发器

API 触发器为定时任务提供一个专用的 HTTP 端点。使用任务的 Bearer 令牌向该端点发送 POST 请求，会启动一个新会话并返回会话 URL。您可以借此将 Claude Code 接入警报系统、部署流水线、内部工具，或任何可以发出经过身份验证的 HTTP 请求的地方。

您可以在 Web 端添加 API 触发器，既可以在[创建任务](#create-from-the-web)时添加，也可以稍后按照以下步骤添加。您无法在 CLI 中添加 API 触发器或管理其令牌。

**1. 打开任务进行编辑**

访问 [claude.ai/code/routines](https://claude.ai/code/routines)，点击您要通过 API 触发的定时任务，然后打开其名称旁边的菜单并选择 **Edit**。随后会打开 **Edit scheduled task** 对话框。

**2. 添加 API 触发器**

在 **Trigger** 行中，点击 **+**，然后从打开的菜单中选择 **API**。随后会出现一个面板，显示此任务的 **Fire URL** 和 **Token**。

**3. 复制 URL 并生成令牌**

复制 **Fire URL**。点击 **Generate token**，然后立即从 **Token generated** 对话框中复制令牌。令牌仅显示一次，之后无法再次获取，因此请将其存储在安全的位置，例如您警报工具的密钥存储中。

生成令牌后，API 触发器即刻生效。您无需点击 **Save changes**，完成后直接关闭对话框即可。

**4. 调用端点**

向该 URL 发送 POST 请求时，请在 `Authorization: Bearer` 标头中发送令牌。[触发定时任务](#trigger-a-scheduled-task)展示了一个完整示例。

每个定时任务都有自己的令牌，仅限于触发该任务。要关闭 API 访问或轮换令牌，请再次[打开任务的 **Edit scheduled task** 对话框](#edit-and-control-scheduled-tasks)。这两个控件都位于 API 触发器面板中令牌的旁边，并会立即生效：

* **关闭 API 访问**：点击 **Delete token**，即 **Regenerate** 旁边的垃圾桶图标。这会移除 API 触发器
* **轮换令牌**：点击 **Regenerate**。之前的令牌会失效，**Token generated** 对话框会显示一次新令牌

#### 触发定时任务

向 `/fire` 端点发送 POST 请求，并在 `Authorization` 标头中包含 Bearer 令牌。请求体接受一个可选的 `text` 字段，用于提供特定于本次运行的上下文（例如警报正文或失败日志），它会与任务已保存的提示词一起传递给任务。该值是自由格式文本，不会被解析：如果您发送 JSON 或其他结构化数据，任务会将其作为字面字符串接收。

`text` 值不会以裸消息的形式传递给 Routine。它会被包装在一个 `<routine-fire-payload>` 块中，该块将其标记为不受信任的数据，并告诉 Claude 不要遵循其中的指令，除非 Routine 自身的提示词要求这样做。在 Web UI 中通过 **Run now** 提供的文本也适用相同的包装。

这意味着 Routine 的已保存提示词必须明确选择对触发文本采取行动：编写提示词时要显式引用该负载，例如"Investigate the alert described in the routine-fire-payload block"，否则 Routine 会将该文本视为惰性上下文。任何持有 Bearer 令牌的人都可以发送 `text`，因此该包装可确保来自泄露令牌的触发文本被标记为不受信任的数据，而不是作为对您 Routine 的直接指令。

以下示例从 shell 触发定时任务。其中显示的任务 ID 和令牌均为占位符：请将它们替换为您在[添加 API 触发器](#add-an-api-trigger)时复制的 URL 和令牌，否则请求会失败并返回 `401` 身份验证错误：

```bash theme={null}
curl -X POST https://api.anthropic.com/v1/claude_code/routines/trig_01ABCDEFGHJKLMNOPQRSTUVW/fire \
  -H "Authorization: Bearer sk-ant-oat01-xxxxx" \
  -H "anthropic-beta: experimental-cc-routine-2026-04-01" \
  -H "anthropic-version: 2023-06-01" \
  -H "Content-Type: application/json" \
  -d '{"text": "Sentry alert SEN-4521 fired in prod. Stack trace attached."}'
```

成功的请求会返回一个包含新会话 ID 和 URL 的 JSON 响应体：

```json theme={null}
{
  "type": "routine_fire",
  "claude_code_session_id": "session_01HJKLMNOPQRSTUVWXYZ",
  "claude_code_session_url": "https://claude.ai/code/session_01HJKLMNOPQRSTUVWXYZ"
}
```

在浏览器中打开会话 URL，即可实时观看运行、审查更改或手动继续对话。

> [!WARNING]
>
> `/fire` 端点在 `experimental-cc-routine-2026-04-01` beta 标头下发布。在该功能处于研究预览阶段期间，请求和响应的结构、速率限制以及令牌语义都可能发生变化。破坏性更改会在新的带日期 beta 标头版本下发布，并且最近的两个旧标头版本会继续可用，以便调用方有时间迁移。

#### API 参考

有关完整的 API 参考，包括所有错误响应、验证规则和字段限制，请参阅 Claude Platform 文档中的 [Trigger a routine through the API](https://platform.claude.com/docs/en/api/claude-code/routines-fire)。

`/fire` 端点仅供 claude.ai 用户使用，不属于 Claude Platform API 接口的一部分。

### 添加 GitHub 触发器

当已连接的仓库上发生匹配事件时，GitHub 触发器会自动启动一个新会话。Claude Code 不会在不同事件之间复用会话，因此两次 PR 更新会产生两个独立的会话。

> [!NOTE]
>
> GitHub webhook 事件受每个任务和每个账户的每小时上限约束。超出限制的事件会被丢弃，直到时间窗口重置。

无论您从哪个使用入口配置触发器，都必须在要订阅的仓库上安装 Claude GitHub App。

* 在 Web UI 中配置 GitHub 触发器时，如果缺少该应用，系统会提示您安装。按照以下步骤在 Web 端进行配置。
* 在 CLI 中，请先从 [GitHub App 页面](https://github.com/apps/claude)安装该应用，然后让 Claude 将 GitHub 触发器附加到现有的定时任务，例如 `/schedule add a GitHub trigger to my nightly review for pull requests opened in acme/webapp`。CLI 方式需要 Claude Code v2.1.225 或更高版本。Claude 添加触发器后，会回复一个指向该触发器所触发任务的链接。

**1. 打开任务进行编辑**

访问 [claude.ai/code/routines](https://claude.ai/code/routines)，点击该定时任务，然后打开其名称旁边的菜单并选择 **Edit**。随后会打开 **Edit scheduled task** 对话框。

**2. 添加 GitHub 事件触发器**

在 **Trigger** 行中，点击 **+**，然后从打开的菜单中选择 **GitHub event**。

> [!NOTE]
>
> 在 CLI 中运行 `/web-setup` 会授予克隆仓库所需的访问权限，但它不会安装 Claude GitHub App，也不会启用 webhook 投递。

**3. 配置触发器**

如有需要，更改仓库，从[支持的事件](#supported-events)列表中选择一个事件，并可选择添加筛选条件。然后点击对话框底部的 **Save changes** 以应用该触发器。

#### 支持的事件

从以下任一类别中选择一个事件。在每个类别中，您可以选择特定操作（例如 `pull_request.opened`），也可以响应该类别中的所有操作。

| 事件 | 触发时机 |
| :- | :- |
| Pull request | PR 被打开、关闭、分配、添加标签、同步或以其他方式更新 |
| Release | 发布被创建、发布、编辑或删除 |
| Issue | Issue 被打开、编辑、关闭、重新打开、添加标签、分配或以其他方式更新 |

#### 筛选 Pull Request

使用筛选条件来限定哪些 Pull Request 会启动新会话。所有筛选条件都必须匹配，定时任务才会触发。可用的筛选字段如下：

| 筛选条件 | 匹配内容 |
| :- | :- |
| Author | PR 作者的 GitHub 用户名 |
| Title | PR 标题文本 |
| Body | PR 描述文本 |
| Base branch | PR 的目标分支 |
| Head branch | PR 的来源分支 |
| Labels | 应用于 PR 的标签 |
| Is draft | PR 是否处于草稿状态 |
| Is merged | PR 是否已合并 |

每个筛选条件都将一个字段与一个运算符配对：equals、contains、starts with、is one of、is not one of 或 matches regex。

`matches regex` 运算符会测试整个字段值，而不是其中的子字符串。要匹配任何包含 `hotfix` 的标题，请写成 `.*hotfix.*`。如果没有前后的 `.*`，该筛选条件只会匹配恰好为 `hotfix`、前后没有任何内容的标题。如需不使用正则表达式语法的字面子字符串匹配，请改用 `contains` 运算符。

一些筛选条件组合示例：

* **Auth module review**：base branch 为 `main`，head branch contains `auth-provider`。将任何涉及身份验证的 PR 发送给专门的审查者。
* **Ready-for-review only**：is draft 为 `false`。跳过草稿，使任务仅在 PR 准备好审查时运行。
* **Label-gated backport**：labels 包含 `needs-backport`。仅当维护者为 PR 添加该标签时，才触发移植到另一个分支的任务。

## 管理定时任务

在列表中点击某个定时任务即可打开其详情页。详情页会显示该任务的仓库、连接器、提示词、计划、API 令牌、GitHub 触发器以及过往运行的列表。

### 查看运行并与之交互

点击任意运行即可将其作为完整会话打开。在那里，您可以查看 Claude 执行了哪些操作、审查更改、创建 Pull Request 或继续对话。每个运行会话的用法与其他会话相同：使用会话标题旁边的下拉菜单对其进行重命名、归档或删除。

在任务详情页的 **History** 下，每个已结束运行所在行的末尾会显示其结果：运行失败时显示 **Error**，运行被取消时显示 **Cancelled**，运行成功时则两者都不显示。**Status** 筛选器将这三种结果分别称为 **Failed**、**Cancelled** 和 **Succeeded**。这些结果表示会话是如何结束的，而不表示 Claude 是否完成了提示词中描述的工作，因此请打开该运行并阅读其会话记录，以了解实际发生了什么。

* **Succeeded**：会话结束时 Claude Code 未报告失败。被阻止的网络请求、缺失的连接器工具以及 Claude 未能完成的工作会显示在会话记录中，而不会体现在该行上
* **Failed**：会话未正常结束。例如，它意外停止、因错误而结束，或在运行开始之前就已被归档
* **Cancelled**：运行并未失败，但其会话已被归档。如果您归档了一个成功运行的会话，该运行会变为 **Cancelled**。任务本身无需做任何更改

如果 **History** 下出现警告，请参阅[“The last run”警告](#the-last-run-didnt-start)。

### 编辑和控制定时任务

在详情页中，您可以：

* 点击 **Run now** 立即启动一次运行，而无需等待下一个计划时间。
* 使用页面顶部的开关暂停或恢复计划。已暂停的任务会保留其配置，但在您重新打开之前不会运行。
* 打开任务名称旁边的菜单并选择 **Edit**，以更改名称、提示词、仓库、环境、连接器或任务的任何触发器。随后会打开 **Edit scheduled task** 对话框，其布局与[创建对话框](#create-from-the-web)相同。要添加触发器，请点击 **Trigger** 行中的 **+**。点击 **Save changes** 以应用您的编辑。API 令牌操作会立即生效，无需点击 **Save changes**。
* 打开同一菜单并选择 **Delete** 以删除该任务。

### 从 CLI 管理定时任务

运行 `/schedule list` 可查看您的定时任务，运行 `/schedule update` 可更改某个任务，运行 `/schedule run` 可立即启动某个任务。

您还可以询问某个定时任务的运行历史，例如 `/schedule why did my nightly review do nothing this morning?`。Claude 会列出该任务最近的运行及其状态，并附上[在 Web 端打开每次运行](#view-and-interact-with-runs)的链接。它还会读取运行日志来解释发生了什么，包括工具错误、权限拒绝和最终结果。需要 Claude Code v2.1.227 或更高版本。

### 仓库和分支权限

定时任务需要 GitHub 访问权限才能克隆仓库。当您在 CLI 中使用 `/schedule` 创建定时任务时，Claude 会检查您的账户是否拥有对您运行该命令所在仓库的 GitHub 访问权限。如果没有，Claude 会添加一条设置说明，指明如何授予访问权限。

> [!NOTE]
>
> 请参阅 [GitHub 身份验证选项](https://code.claude.com/docs/zh-CN/claude-code-on-the-web.md#github-authentication-options)，了解授予访问权限的两种方式。在 Team 和 Enterprise 套餐中，必须由您 Claude 组织的 [Owner](https://code.claude.com/docs/zh-CN/server-managed-settings.md#access-control) 先启用每种方式，您才能使用；请参阅[连接 GitHub](https://code.claude.com/docs/zh-CN/web-quickstart.md#connect-github)。

如果在某次运行到期时您的 GitHub 连接缺失或已过期，定时任务会跳过运行，直到您重新连接为止，最长持续 72 小时。在此期间内重新连接 GitHub，任务会自行恢复。如果 72 小时后仍未连接，任务将被关闭，您需要在重新连接 GitHub 后再将其重新打开。

您添加的每个仓库都会在每次运行时被克隆。除非您的提示词另有指定，否则 Claude 会从仓库的默认分支开始。如果运行由 [GitHub Pull Request 事件](#add-a-github-trigger)触发，且该 Pull Request 所在的仓库是任务中的第一个仓库，则该仓库会改为从 Pull Request 的 head 提交开始。

除非您的提示词指示 Claude 推送到其他分支，否则 Claude 会将其工作推送到以 `claude/` 为前缀的分支。要控制运行可以推送到哪些分支，请在 GitHub 上使用分支保护规则或规则集。对于在 Anthropic 托管基础设施上的运行，以及通过 [Anthropic 的 git 代理](https://code.claude.com/docs/zh-CN/self-hosted-environments-deploy.md#use-the-anthropic-git-proxy)推送的自托管运行，GitHub 会将这些规则应用于您所连接的 GitHub 访问权限，因此该访问权限可以绕过的规则不会阻止运行的推送。使用您的部署所提供的 git 凭据进行推送的自托管运行，则会依据这些凭据进行检查。请参阅[配置 git](https://code.claude.com/docs/zh-CN/self-hosted-environments-deploy.md#configure-git)。

### 连接器

定时任务可以在每次运行期间使用您已连接的 MCP 连接器来读取和写入外部服务。例如，一个对支持请求进行分诊的任务可能会从 Slack 频道读取内容，并在 Linear 中创建问题。

连接器是您账户中的 [claude.ai 集成](https://code.claude.com/docs/zh-CN/mcp.md#use-mcp-servers-from-claude-ai)。您在 CLI 中使用 `claude mcp add` 在本地添加的 MCP 服务器存储在您的计算机上，而不是您的 claude.ai 账户中，因此它们不会出现在连接器列表中。要在定时任务中使用其中某个服务器：

* **作为连接器**：在 [claude.ai/customize/connectors](https://claude.ai/customize/connectors) 将其添加为连接器
* **在仓库中**：对于只有一个仓库的定时任务，您也可以改为在已提交的 [`.mcp.json`](https://code.claude.com/docs/zh-CN/mcp#project-scope) 中声明该服务器，除非该任务在[项目](https://code.claude.com/docs/zh-CN/claude-projects.md#what-threads-pick-up-from-your-repositories)中以线程方式运行。如果是这种情况，请将该服务器添加为连接器。

创建定时任务时，默认会包含您当前已连接的所有连接器。请移除不需要的连接器，以限制 Claude 在运行期间可以访问的工具。您也可以直接在任务的创建或编辑对话框中添加连接器。

要在该对话框之外管理或添加连接器，请访问 [claude.ai/customize/connectors](https://claude.ai/customize/connectors)，或在 CLI 中使用 `/schedule update`。

### 运行期间的权限提示

> [!NOTE]
>
> 对于在您组织的自托管环境中运行的任务，请参阅[权限和工具批准](https://code.claude.com/docs/zh-CN/self-hosted-environments-configuration.md#permissions-and-tool-approval)。

当运行在 Anthropic 云端启动新会话时，该会话会以[自动模式](https://code.claude.com/docs/zh-CN/permission-modes.md#eliminate-prompts-with-auto-mode)开始，除非自动模式不可用。在自动模式下，分类器模型会允许或阻止原本需要等待您批准的操作。

在以下等情况下，运行仍会停止并等待回答：

* **即使在自动模式下也会提示的操作**：例如，与[询问规则](https://code.claude.com/docs/zh-CN/permissions.md#manage-permissions)匹配的操作。请参阅[自动模式如何评估操作](https://code.claude.com/docs/zh-CN/permission-modes.md#how-auto-mode-evaluates-actions)
* **重复阻止**：一次阻止不会结束运行。一旦阻止次数达到自动模式的[重复阻止阈值](https://code.claude.com/docs/zh-CN/permission-modes.md#repeated-block-thresholds)，运行的会话中就会出现权限提示，并一直等待直到有人回答

如果自动模式不可用，例如您的组织已将其关闭或任务的模型不支持它，运行将保持其启动时的权限模式。要回答等待中的提示，请从任务的[详情页](#view-and-interact-with-runs)打开该运行。

在[项目](https://code.claude.com/docs/zh-CN/claude-projects.md)中，运行可以继续某个线程，而不是启动新会话。该运行会保持线程当时所处的权限模式。该线程在创建时会以自动模式开始，除非自动模式不可用。要回答在该线程中等待的提示，请参阅[解除等待批准的线程的阻塞](https://code.claude.com/docs/zh-CN/claude-projects.md#unblock-a-thread-waiting-on-approval)。

### 运行在未询问的情况下发布 Artifact 的情形

在[自动模式](#permission-prompts-during-a-run)下的运行期间，一旦分类器允许发布，Claude 就会在不询问您的情况下发布 [Artifact](https://code.claude.com/docs/zh-CN/artifacts.md)。在某些情况下仍会先询问您，例如发布操作与[询问规则](https://code.claude.com/docs/zh-CN/permissions.md#manage-permissions)匹配时。在您[共享它](https://code.claude.com/docs/zh-CN/artifacts.md#share-an-artifact)之前，新的 Artifact 仅对您可见。下文的 [Accept edits 规则](#artifacts-in-accept-edits-mode)不适用于自动模式。

如果定时任务的工作是保持某个页面处于最新状态，请为其提供一个您已经发布的 Artifact。

#### Accept edits 模式下的 Artifact

当定时任务的计划或 **Run now** 启动一次运行时，Claude 可以在不询问的情况下发布新的 [Artifact](https://code.claude.com/docs/zh-CN/artifacts.md)。在您[共享它](https://code.claude.com/docs/zh-CN/artifacts.md#share-an-artifact)之前，新的 Artifact 仅对您可见。如果发布操作为页面声明了运行时能力，例如[连接器调用](https://code.claude.com/docs/zh-CN/artifacts.md#pull-live-data-with-mcp-connectors)或[文件下载](https://code.claude.com/docs/zh-CN/artifacts.md#offer-a-file-download)，Claude 会先询问。

在这些运行中，只有当以下所有条件都满足时，Claude 才会在不询问的情况下重新发布现有的 Artifact：

* 您可以编辑该 Artifact，且它属于您自己的组织
* 如果发布内容不止页面本身（例如附带支持文件），则您是该 Artifact 的所有者
* 该 Artifact 未公开共享
* 如果该 Artifact 已与特定人员或您的组织共享，其查看者不会自动看到每个新版本
* 发布操作不会强制覆盖较新的版本
* 发布操作未为页面声明运行时能力
* 现有页面不持有任何超出页面范围的授权，例如[连接器调用](https://code.claude.com/docs/zh-CN/artifacts.md#pull-live-data-with-mcp-connectors)

在所有其他情况下，包括由 API 或 GitHub 触发器启动的运行，Claude 都会先询问。

### 环境和网络访问

每个定时任务都使用一个[云环境](https://code.claude.com/docs/zh-CN/cloud-environments.md)，用于控制网络访问、环境变量和设置脚本。任务在每次运行时都会继承该环境的网络策略。

**Default** 环境使用 **Trusted** 网络访问，仅允许[默认允许列表](https://code.claude.com/docs/zh-CN/cloud-environments.md#default-allowed-domains)中的主机通过会话的网络。通过该路径向允许列表以外主机发出的请求会失败，并返回 `403` 和 `x-deny-reason: host_not_allowed`。MCP 连接器流量通过 Anthropic 的服务器路由，而不经过该路径，因此您添加到任务中的连接器无需将其主机添加到 **Allowed domains** 即可正常工作。请在[连接器](#connectors)下移除您不需要的任何连接器。

要在您自己的某个环境中允许更多域名，请按照以下步骤操作。[组织共享环境](https://code.claude.com/docs/zh-CN/cloud-environments.md#organization-shared-environments)在此处以只读方式打开，因此需要由 Owner 在[管理设置](https://claude.ai/admin-settings)的 **Cloud environments** 页面中更改其网络访问。

**1. 打开任务进行编辑**

访问 [claude.ai/code/routines](https://claude.ai/code/routines)，点击该定时任务，然后打开其名称旁边的菜单并选择 **Edit**。随后会打开 **Edit scheduled task** 对话框。

**2. 打开环境选择器**

前往 **Advanced settings > General > Cloud environment**，如果 **Advanced settings** 处于折叠状态，请先将其展开。点击显示您环境名称（例如 **Default**）的云图标。

**3. 打开环境设置**

将鼠标悬停在列表中的环境上，然后点击右侧出现的设置图标。

**4. 更改网络访问级别**

在 **Edit cloud environment** 对话框中，将 **Network access** 更改为 **Custom**，并在 **Allowed domains** 中输入您的域名。勾选 **Also include default list of common package managers**，以便在自定义域名之外保留[默认允许列表](https://code.claude.com/docs/zh-CN/cloud-environments.md#default-allowed-domains)。如需不受限制的访问，请改为选择 **Full**。

**5. 保存**

在 **Edit cloud environment** 对话框中点击 **Save changes**，然后关闭其后方的 **Edit scheduled task** 对话框。新策略将从下一次运行开始生效。

有关访问级别和默认允许列表的详细信息，请参阅[网络访问](https://code.claude.com/docs/zh-CN/cloud-environments.md#network-access)。

## 用量和限制

定时任务与交互式会话以相同的方式消耗订阅用量。您可以在 [claude.ai/settings/usage](https://claude.ai/settings/usage) 查看当前的消耗情况。

除订阅用量外，每种启动运行的方式都有每小时限制：

| 操作 | 限制 | 计数对象 | 超出限制时 |
| :- | :- | :- | :- |
| 计划运行，包括一次性运行 | 每小时 100 次 | 您的账户 | 运行会等待，直到限制重置 |
| **Run now**、API 触发以及将一次性任务设置为再次运行 | 每小时 30 次 | 每个定时任务，三者共用一个计数 | 操作失败，直到限制重置 |
| **Run now** 以及将一次性任务设置为再次运行 | 每小时 100 次 | 您的账户 | 同上 |
| API 触发 | 每小时 100 次 | 您的账户，与 **Run now** 分开计数 | 同上 |
| GitHub 事件 | 请参阅[添加 GitHub 触发器](#add-a-github-trigger) | | |

这些每小时限制均不提供超额使用。

当定时任务达到您的订阅用量限制时，已启用使用额度的组织可以继续以计量超额方式运行定时任务。如果没有使用额度，额外的运行会被拒绝，直到您的用量窗口重置。请在 [claude.ai/settings/usage](https://claude.ai/settings/usage) 启用使用额度。在 Team 和 Enterprise 套餐中，由管理员在 [claude.ai/admin-settings/usage](https://claude.ai/admin-settings/usage) 为组织启用使用额度。

在您的订阅暂停期间，您的定时任务会被挂起且不会运行。订阅恢复后，请将它们重新打开。

## Routine 名称仍出现的位置

定时任务以前称为 Routine，以下位置仍保留这一旧名称。它们指的都是同一项功能：

* **应用**：在某些账户中，侧边栏条目和页面标题显示为 **Routines**，按钮显示为 **New routine**，其他标签也使用 routine 一词。如需在此处创建任务，请参阅[完整页面表单](#full-page-form)。
* **网址**：`claude.ai/code/routines`
* **CLI**：`/schedule` 的别名 `/routines`、该命令的描述，以及 Claude 在设置任务时的回复
* **API**：`/fire` 端点路径、其 beta 标头、其响应中的 `type`、`<routine-fire-payload>` 标签，以及 [Trigger a routine through the API](https://platform.claude.com/docs/en/api/claude-code/routines-fire) 参考页面
* **组织设置**：**Scheduled tasks (Was Routines)** 开关

## 故障排除

### `/schedule` 返回 "Unknown command"

当某项要求未满足时，CLI 会隐藏 `/schedule`：您输入时命令菜单会显示 `No commands match "/schedule"`。提交该命令会返回 `Unknown command: /schedule`，但下文注明会给出不同回复的情况除外。

原因通常是以下之一：

* 您使用 Console API 密钥、[Anthropic profile 或联合凭据](https://code.claude.com/docs/zh-CN/authentication.md#anthropic-profiles-and-federation-credentials)，或者 Amazon Bedrock、Google Cloud 的 Agent Platform 或 Microsoft Foundry 等云提供商进行身份验证。`/schedule` 需要使用 claude.ai 订阅登录。使用 Console API 密钥或 profile 且启用了功能标志获取时，提交 `/schedule` 会改为显示 `/schedule is available with Claude for Enterprise — ask your admin about migrating from API-key access`。使用云提供商登录时，您仍会看到 `Unknown command: /schedule`。如果您的 shell 中设置了 `ANTHROPIC_API_KEY` 或 `ANTHROPIC_AUTH_TOKEN`，或者 `settings.json` 中设置了 `apiKeyHelper`，请先将其移除，因为它们优先于 claude.ai 登录。profile 或联合凭据同样具有优先级，因此也请将其关闭
* 您已完全退出登录，没有 API 密钥或其他凭据。启用功能标志获取时，提交 `/schedule` 会显示 `/schedule requires a claude.ai subscription. Run /login to sign in with your claude.ai account.` 在 v2.1.268 之前，已退出登录的会话会显示与 Console API 密钥相同的 Claude for Enterprise 消息
* 您处于云端会话中，此时提交 `/schedule` 会回复该命令在该环境中不可用。请改为在 [Web UI](https://claude.ai/code/routines) 中管理定时任务
* 您组织的策略禁用了定时任务所需的[云端会话](https://code.claude.com/docs/zh-CN/claude-code-on-the-web.md)。在这种情况下，提交 `/schedule` 会改为回复 [`Cloud sessions are disabled by your organization's policy`](https://code.claude.com/docs/zh-CN/errors#cloud-sessions-are-disabled-by-your-organizations-policy)。在 v2.1.268 之前，它会返回 `Unknown command: /schedule`
* Owner 为您的 Team 或 Enterprise 组织[关闭了定时任务](#routines-are-disabled-by-your-organizations-policy)。在 v2.1.227 之前，这种情况下命令仍会显示，而当 Claude 尝试创建或运行任务时，claude.ai 会拒绝该任务
* 您的 Enterprise 组织通过[自定义角色](https://support.claude.com/en/articles/13930452)管理您的访问权限，而您的所有角色都不包含 **Remote sessions** 下的定时任务权限。请联系 Owner 将该权限添加到您的某个角色中

如果原因在于您的 CLI 登录方式或您处于云端会话中，您仍可以在 [claude.ai/code/routines](https://claude.ai/code/routines) 创建和管理定时任务。您需要在该处使用 Pro、Max、Ultra、Team 或 Enterprise 账户。如果您的组织关闭了定时任务或云端会话，或者您的角色中缺少相应权限，您也无法在 Web 端创建定时任务。

### "Routines are disabled by your organization's policy"

您 Team 或 Enterprise 组织中的 Owner 可能已在 [**Organization settings > Capabilities > Remote sessions**](https://claude.ai/admin-settings/capabilities) 关闭了 **Scheduled tasks (Was Routines)** 开关。在 Claude Code v2.1.227 或更高版本中，同一开关还会在 CLI 中隐藏 `/schedule`。这是一项服务器端的组织设置，因此无法通过您的本地配置覆盖。请联系 Owner 为您的组织重新启用定时任务。

### "The last run" 警告

当 **History** 下的警告以 "The last run" 开头时，请在第一列中找到该警告，并按照其所在行的说明操作。之后的某次运行启动后，该警告即会清除。

| 警告 | 处理方法 |
| :- | :- |
| "The last run didn't start because your organization had reached its limit on cloud sessions running at once." | 该任务无需更改。稍后点击 **Run now**，或等待下一次运行 |
| "The last run didn't start because its cloud environment couldn't be found or used." | 在任务的详情页面上，打开其名称旁边的菜单，选择 **Edit**，然后找到[云环境选择器](#environments-and-network-access)。如果其中显示 **Unavailable environment**，说明该环境已被归档或您的账户已无法再使用该环境，请在此处选择另一个环境并保存更改。否则，请在详情页面上使用 **Run now** 重试，或在选择器中选择另一个环境并保存更改 |
| "The last run was blocked because it couldn't access something it needs, or your organization's policy doesn't allow it." | 检查您的 [GitHub 访问权限](#repositories-and-branch-permissions)是否涵盖该任务的仓库。如果已涵盖，请询问您 Claude 组织的 [Owner](https://code.claude.com/docs/zh-CN/server-managed-settings.md#access-control) 是否有策略阻止了该任务 |
| "The last run didn't start because of a problem that's usually temporary." | 点击 **Run now** 重试，或等待下一次运行 |
| "The last run didn't start because its repository or branch wasn't available." | 在 GitHub 上检查任务中的每个仓库是否都有[运行所基于的分支](#repositories-and-branch-permissions)。同时检查您的 [GitHub 访问权限](#repositories-and-branch-permissions)是否涵盖每个仓库，如果连接已过期，请重新连接 GitHub |
| "The last run didn't start because some of its settings weren't accepted." | 在任务的详情页面上，打开其名称旁边的菜单并选择 **Edit**。该对话框不会标明哪项设置未被接受，因此请在其中逐一检查每项设置，包括任务的名称、提示词、模型、仓库、环境、连接器和触发器 |
| "The last run stopped because a usage limit was reached." | 关于达到用量限制后任务何时可以再次开始运行，请参阅[用量和限制](#usage-and-limits) |
| "The last run failed." | 点击 **Run now** 重试，或等待下一次运行。失败的运行没有可打开的会话 |

## 相关资源

* [`/loop` 和会话内调度](https://code.claude.com/docs/zh-CN/scheduled-tasks)：在打开的 CLI 会话中安排本地任务
* [Desktop 定时任务](https://code.claude.com/docs/zh-CN/desktop-scheduled-tasks.md)：在您的计算机上运行、可访问本地文件的本地定时任务
* [云环境](https://code.claude.com/docs/zh-CN/cloud-environments.md)：为云端会话配置网络访问、环境变量和设置脚本
* [Projects](https://code.claude.com/docs/zh-CN/claude-projects.md)：Claude 跨并行云端会话协调的持续性工作；从项目创建的定时任务会显示在其 **Scheduled tasks** 标签页上
* [MCP 连接器](https://code.claude.com/docs/zh-CN/mcp.md)：连接 Slack、Linear 和 Google Drive 等外部服务
* [GitHub Actions](https://code.claude.com/docs/zh-CN/github-actions.md)：在 CI 流水线中针对仓库事件运行 Claude
