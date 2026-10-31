> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# 有效管理成本

> 跟踪令牌使用情况，设置团队支出限制，并通过上下文管理、模型选择、扩展思考设置和预处理 hooks 来降低 Claude Code 成本。

Claude Code 按 API token 消耗收费。有关订阅套餐定价（Pro、Max、Ultra、Team、Enterprise），请参阅 [claude.com/pricing](https://claude.com/pricing)。每个开发者的成本差异很大，取决于模型选择、代码库大小和使用模式，例如运行多个实例或自动化。

在企业部署中，平均成本约为每个开发者每个活跃日 $13，每个开发者每月 $150-250，90% 的用户每个活跃日成本保持在 \$30 以下。要估计您自己团队的支出，请从一个小的试点团体开始，并使用下面的跟踪工具建立基线，然后再进行更广泛的推出。

本页面仅涵盖 Claude Code 的使用：如何[跟踪成本](#track-your-costs)、[管理组织成本](#manage-costs-for-your-organization)和[减少 token 使用](#reduce-token-usage)。有关其他 Claude 产品的用量限制，请参阅 [Claude 帮助中心](https://support.claude.com)。

## 跟踪成本

### 使用 `/usage` 命令

> [!NOTE]
>
> `/usage` 中的 Session 块显示 API token 使用情况，适用于 API 用户。Claude Pro、Max 和 Ultra 订阅者的使用量包含在订阅中，因此会话成本数据与计费无关。订阅者会在同一屏幕上看到套餐使用条、活动统计以及使用情况明细。

`/usage` 顶部的 Session 块显示当前会话的详细令牌使用统计。Claude Code 从令牌计数本地计算美元数字，按列表价格计算，除非 [`modelPricing`](https://code.claude.com/docs/zh-CN/settings-reference#modelpricing) 表生效。管理员在您组织的托管设置中设置一个表，以便该数字使用您的合同费率，当表生效时，`Total cost` 行会带有注释 `at your organization's configured rates`。该数字是估计值，有关权威计费，请参阅 [Claude Console](https://platform.claude.com/usage) 中的使用情况页面。

```text theme={null}
Total cost:            $0.55
Total duration (API):  6m 20s
Total duration (wall): 6h 33m 10s
Total code changes:    0 lines added, 0 lines removed
Usage by model:
   claude-sonnet-4-6:  1.2k input, 5.3k output, 940.0k cache read, 50.0k cache write ($0.55)
```

这些总计在 `/clear` 启动新会话时重置，因此下一个会话的总成本从 \$0 开始。在 v2.1.211 之前，它们在 `/clear` 后继续累积，直到 Claude Code 进程的生命周期结束。

对于以 1.1× [数据驻留费率](https://platform.claude.com/docs/en/about-claude/pricing#data-residency-pricing) 计费的 Claude API 响应，Claude Code 在会话成本数字中将该响应 token 的列表价格乘以 1.1。相同的总计也会出现在[状态栏的成本字段](https://code.claude.com/docs/zh-CN/statusline.md#cost-and-duration-tracking)中，并且乘以后的数字同样计入 [`--max-budget-usd`](https://code.claude.com/docs/zh-CN/cli-reference#cli-flags)。

#### Prompt cache 统计

在主对话的第一个 API 响应之后，Claude Code 还会向 Session 块添加一个 `Prompt cache (main)` 行，总结会话的 [prompt cache](https://code.claude.com/docs/zh-CN/prompt-caching.md) 使用情况：请求计数、从缓存提供的输入令牌份额、缓存未命中，以及缓存现在是否处于热状态。需要 Claude Code v2.1.251 或更高版本。

```text theme={null}
Prompt cache (main):   14 requests · 91% of input tokens from cache · 2 misses (last 6m 10s ago, 310.2k tokens re-cached) · 1 expected rebuild (compaction or tool-result clearing) · warm (1h TTL, last activity 40s ago)
```

该行中的未命中、预期重建以及热或冷部分的含义如下：

* **Misses（未命中）**：重新处理缓存已保存内容的请求，包括最后一次未命中的时间以及这些请求写回缓存的令牌数。[使缓存失效的操作](https://code.claude.com/docs/zh-CN/prompt-caching.md#actions-that-invalidate-the-cache)列出了常见原因。当 Claude Code 能够识别最后一次未命中的可能原因时，该行也会命名它，例如 `likely cause: tool definitions changed`。可能原因文本需要 Claude Code v2.1.260 或更高版本。
* **Expected rebuilds（预期重建）**：当 Claude Code 本身刚刚重写对话时，通过[压缩](https://code.claude.com/docs/zh-CN/prompt-caching.md#compacting-the-conversation)或从上下文中清除旧工具结果，它会将相同类型的未命中计为预期重建。此部分仅在至少发生过一次预期重建后出现。
* **Warm or cold（热或冷）**：缓存的前缀是否仍在其[缓存生命周期](https://code.claude.com/docs/zh-CN/prompt-caching.md#cache-lifetime)内，以及生效的 TTL。当缓存冷时，该行显示会话已空闲多长时间。当没有响应报告缓存令牌时，该行以 `no prompt caching reported by the API` 结尾。

计数来自 API 响应中的缓存令牌字段，因此该行适用于每个提供商和网关。它仅覆盖主对话，不包括子代理。`/clear` 与 Session 块的其余部分一起重置它。

状态行脚本可以从 [`prompt_cache` 对象](https://code.claude.com/docs/zh-CN/statusline#prompt-cache-fields)读取相同的数字。

#### 计划使用情况明细

在 Pro、Max、Ultra、Team 或 Enterprise 套餐上，`/usage` 还显示计入您的套餐限制的内容明细，并附带建议：

* **Attribution（归属）**：最近使用情况归属于 skills、subagents、plugins 和各个 MCP 服务器，每个都显示为总数的百分比。MCP 服务器的份额仅计算消耗其工具结果之一的请求。在 v2.1.222 之前，在调用 MCP 服务器一次后，Claude Code 将每个后续请求归属于该服务器，高估了其份额。
* **Recommendations（建议）**：建议的更改，每行一项。每行开头是您最近使用情况的一个份额（至少 10%），例如 `25%  Trim ~/code/api/CLAUDE.md · 49k tokens per request`。
  * **如何解读份额**：它表示该行适用于您多少使用量，而不是该更改能节省多少。一个请求可以计入多行，因此各份额相加可能超过 100%。
  * **如何应用建议**：要让 Claude Code 应用其能够应用的建议，请参阅[使用 fix 快捷键应用建议](#apply-recommendations-with-the-fix-shortcut)
* **Loops（循环）**：最近运行的每个最重的 [`/loop` 或其他会话范围的定时任务](https://code.claude.com/docs/zh-CN/scheduled-tasks)各占一行，按总 token 排序，其余的显示计数。需要 Claude Code v2.1.242 或更高版本。
  * **每行报告的内容**：任务触发的频率、运行次数、其总 token 和每次运行 token，以及最后一次运行的时间
  * **停止并重新创建的循环**：Claude Code 以任务的提示词作为行的键，因此您停止并重新创建的循环保持为一行

按 `d` 或 `w` 在过去 24 小时和过去 7 天之间切换。这些数据是近似值，从此机器上的本地会话历史记录计算，因此不包括来自其他设备或 claude.ai 的使用情况。

在 [VS Code 扩展](https://code.claude.com/docs/zh-CN/vs-code.md#check-account-and-usage)中，归属份额和建议显示在"账户和使用情况"对话框中，带有"日"和"周"切换，不包括"循环"行。

#### 使用 fix 快捷键应用建议

当 `/usage` 页脚显示 `f to fix 2 of 3` 之类的提示时，按 `f` 可让 Claude Code 处理其能够处理的建议。`f` 快捷键需要 Claude Code v2.1.305 或更高版本。

1. 在任何进行中的轮次结束后，Claude Code 会运行各行中显示的、前面不带 `run` 的每条 [`/effort`](https://code.claude.com/docs/zh-CN/model-config#adjust-effort-level) 命令。
2. 然后 Claude 会收到一条提示词，其中为它能够处理的其他每一行各包含一个步骤。该提示词要求 Claude 在做出每项更改之前先向您展示该更改。

要了解按 `f` 会如何处理您的每一行，请在第一列中找到该行的形式：

| 行的形式 | 示例行 | 按 `f` 时 |
| :- | :- | :- |
| 以前面不带 `run` 的 `/effort` 命令结尾 | `40%  Switch from max to high effort · /effort high` | Claude Code 运行该命令 |
| 以 `run` 加一条命令结尾 | `30%  Compact before you step away · run /compact` | 此行不会有任何操作。请您自己在 Claude Code 提示符处运行该命令 |
| 以其他任何内容结尾 | `25%  Trim ~/code/api/CLAUDE.md · 49k tokens per request` | 对于其中部分行，Claude 会在提示词中获得一个步骤，其余行则不会有任何操作。如果 Claude 没有针对某一行向您展示任何更改，请您自己按照该建议操作 |

在 `f to fix 2 of 3` 中，第一个数字表示按 `f` 时 Claude Code 会处理的行数，第二个数字表示显示的行数。不含 `of` 的提示（例如 `f to fix both`）表示 Claude Code 会处理显示的每一行。

当 [Remote Control](https://code.claude.com/docs/zh-CN/remote-control.md) 已连接时，页脚不显示 `f` 提示。如果按 `f` 后 `/usage` 关闭并显示 `Nothing was sent. Turn off Remote Control to use fixes.`，说明在 `/usage` 打开期间 Remote Control 建立了连接。要再次使用 `f`，请[断开 Remote Control](https://code.claude.com/docs/zh-CN/remote-control.md#check-connection-status)，然后在 Claude Code 提示符处运行 `/usage` 并再次按 `f`。

#### 检查您的使用额度支出

`/usage` 还在[使用额度](#add-usage-credits-to-your-subscription)开启时显示使用额度行。该行显示的内容取决于您的计划：

* **Pro、Max 和 Ultra**：当前月份的支出，与您设置的每月支出限制进行比较（如果您设置了一个）。当您未设置限制时，该行显示 `Unlimited`，没有支出数字。
* **Team 和 Enterprise**：当前月份的您自己的支出，与您的组织设置的任何[限制](#claude-for-teams-and-enterprise)进行比较，该限制适用于您。覆盖整个组织的限制不会出现在该行中。当您没有自己的限制时，该行显示您的支出，旁边没有限制。当使用额度对您关闭时，`/usage` 不显示使用额度行。

当您有支出限制时，该行在使用额度开启后立即出现，并显示 0%，直到您首次支出使用额度。在 v2.1.236 之前，`/usage` 仅在 Pro、Max 和 Ultra 套餐上显示该行，带有支出限制的行在您支出某些内容之前保持隐藏。

#### 当使用情况请求失败时

当您的计划限制请求失败时（通常是因为使用情况端点受到速率限制），`/usage` 会显示它在过去 60 分钟内在此机器上加载的最后一个使用情况条，以及一个 `Showing last-known usage` 注释，说明该数据是多久前获取的。按 `r` 重试；成功重试会用新数据替换最后已知的条。如果没有过去 60 分钟内的快照，`/usage` 会报告使用情况端点受到速率限制，并提供相同的重试快捷方式。在 v2.1.208 之前，在尚未加载使用情况的会话中受速率限制的请求始终显示错误，没有条。

`/usage` 中出现 `rate limited until 3:14am` 之类的消息，表示 Claude Code 在该时间之前无法再次获取您的套餐限制，而不是表示您与 Claude 的工作受到限制。如果您在该时间之前按 `r`，预计会看到相同的消息。请在该时间之后按 `r` 获取新数据。

### 读取加载指示器旁的 token 计数

当 Claude 在主对话中工作时，加载指示器旁的行末尾可能显示已用时间和 token 计数，例如 `Deciphering… (10m 27s · ↓ 5.5k tokens)`。这是当前轮次迄今为止所产生输出的实时近似计数。在输出到达或工具运行时，计数旁的箭头指向下方。

* **涵盖的内容**：该轮次生成的输出，例如 Claude 流式返回的文本和工具调用、其思考，以及在[前台](https://code.claude.com/docs/zh-CN/sub-agents.md#run-subagents-in-foreground-or-background)运行的子代理的输出
* **不包括的内容**：Claude Code 发送给模型的内容，因此您的提示词、对话历史记录以及上下文的其余部分不会使其变化
* **何时重置**：在每个轮次开始时，以及 Claude Code [压缩对话](https://code.claude.com/docs/zh-CN/prompt-caching.md#compacting-the-conversation)时
* **何时保持隐藏**：在输出开始到达之前、该行过窄无法容纳时，以及在[屏幕阅读器模式](https://code.claude.com/docs/zh-CN/accessibility.md)中

它不会与 `/usage` Session 块中 `Usage by model` 的输出数字一致，后者累加的是整个会话而非单个轮次。它也不同于[详细模式添加](https://code.claude.com/docs/zh-CN/statusline.md#notifications-share-the-status-line-row)的 token 计数，因为后者衡量的是您上下文的大小，而非单个轮次的输出。

### 分析您的使用情况模式

运行 [`/insights`](https://code.claude.com/docs/zh-CN/commands#all-commands) 以获取关于您如何工作而不是您使用了多少令牌的报告。它分析此机器上的最近会话，并编写一份 HTML 报告，涵盖您处理的内容、摩擦点（例如误解的请求或有缺陷的代码）以及有关更有效地使用 Claude Code 的建议。单次运行分析最多 200 个它之前未见过的会话，并跳过非常短的会话。当会话被遗漏时，报告标题显示分析的计数，括号中显示总数，例如 `200 sessions (412 total)`。

当[自动模式](https://code.claude.com/docs/zh-CN/permission-modes.md#eliminate-prompts-with-auto-mode)对会话可用且您最近的会话大多在没有它的情况下运行时，报告还可以包括自动模式在这些会话中可以处理多少权限提示的估计。

Claude Code 将最新报告写入 `~/.claude/usage-data/report.html`，并在同一目录中保存每次运行的时间戳副本，因此早期报告不会被覆盖。Claude Code 按与其余会话数据相同的计划删除报告：在启动时，它删除早于 [`cleanupPeriodDays`](https://code.claude.com/docs/zh-CN/claude-directory#cleaned-up-automatically) 的文件，默认为 30 天。

您可以在任何计划和任何提供商上运行 `/insights`。分析通过与您的常规会话相同的提供商和账户运行，令牌计入您的计划或 API 使用情况。不包括来自其他设备和 claude.ai 的会话。

### 向您的订阅添加使用额度

[使用额度](https://support.claude.com/en/articles/12429409-extra-usage-for-paid-claude-plans)让您可以在计划的使用限制之外继续工作。要管理它们，请在通过 `/login` 使用您的 claude.ai 订阅登录后运行 `/usage-credits`；该命令不适用于 API 密钥身份验证。在自助服务 Enterprise 组织、Enterprise 试用版和通过 AWS Marketplace 计费的 Enterprise 组织中，该命令需要 Claude Code v2.1.248 或更高版本；早期版本会以 [`Unknown command: /usage-credits`](https://code.claude.com/docs/zh-CN/errors#unknown-command) 拒绝它。它打开的内容取决于您的角色：

| 您的角色 | `/usage-credits` 的作用 |
| :- | :- |
| Pro、Max 或 Ultra 订阅者 | 在浏览器中打开 claude.ai 上的 [**Settings > Usage**](https://claude.ai/settings/usage)。在其 **Usage credits** 部分中，您可以打开或关闭使用额度，并检查您的额度余额、本月支出和每月支出限制 |
| 具有计费访问权限的 Team 或 Enterprise 成员 | 在浏览器中打开您组织的用量设置 [**Organization settings > Usage**](https://claude.ai/admin-settings/usage) |
| 没有计费访问权限的 Team 或 Enterprise 成员 | 要求您确认，然后向您的组织管理员发送请求。在 v2.1.211 之前，Claude Code 在没有确认步骤的情况下发送请求 |

对于没有计费访问权限的 Team 和 Enterprise 成员，确认仅在交互式会话中出现：在使用 `-p` 标志的非交互式模式和从[远程控制](https://code.claude.com/docs/zh-CN/remote-control.md)中，该命令不发送请求，并告诉您在交互式会话中运行它。

如果您在早期请求等待管理员时再次运行 `/usage-credits`，Claude Code 会告诉您已经发送了请求，而不是发送重复请求。在管理员驳回您的请求后，再次运行该命令会发送新请求。在 v2.1.222 之前，被驳回的请求也会阻止新请求。

在 Pro 和 Max 计划上，当您达到支出限制且仍有使用额度可用时，Claude Code 会提示您提高或移除限制，而无需离开 CLI。如果服务器拒绝更改，请参阅[无法更新您的支出限制](https://code.claude.com/docs/zh-CN/errors.md#could-not-update-your-spend-limit)。

## 管理组织的成本

您拥有哪些控制手段，取决于您的组织访问 Claude Code 的方式：Claude for Teams 或 Enterprise 套餐、Claude Console，或云服务提供商。在 Teams 和 Enterprise 套餐中，用量从每位成员的席位额度中扣除。在 Console 和云服务提供商上，用量按 token 向您的组织计费。如果您的组织混合使用多种登录方式，每位开发者将按其进行身份验证时所用的方式计量。

下表列出了每种设置下在哪里查看支出、在哪里设置支出上限，以及如何获取按用户统计的数据。在个人 Pro、Max 或 Ultra 套餐中，您没有需要管理的组织，因此请在[为您的订阅添加使用额度](#add-usage-credits-to-your-subscription)下跟踪您自己的使用额度支出，包括[快速模式](https://code.claude.com/docs/zh-CN/fast-mode.md#see-where-fast-mode-spend-appears)。

| 您的设置 | 查看支出 | 限制支出 | 按用户报告 |
| :- | :- | :- | :- |
| [Claude for Teams 或 Enterprise](#claude-for-teams-and-enterprise) | [组织分析中的支出报告](https://support.claude.com/en/articles/12883420-view-usage-analytics-for-team-and-enterprise-plans) | 管理设置中的支出限制 | [支出报告 CSV](https://support.claude.com/en/articles/12883420-view-usage-analytics-for-team-and-enterprise-plans)；Enterprise 套餐可使用 [Enterprise Analytics API](https://platform.claude.com/docs/en/api/admin/analytics) |
| [Claude Console (API)](#claude-console) | [Console 用量页面](https://platform.claude.com/usage) | 工作区支出限制 | [Console 仪表板](https://platform.claude.com/claude-code)、[Claude Code Analytics API](https://platform.claude.com/docs/en/build-with-claude/claude-code-analytics-api) |
| [Amazon Bedrock、Google Cloud's Agent Platform 或 Microsoft Foundry](#cloud-providers) | 您的云计费控制台 | 您的云预算控制 | [OpenTelemetry](https://code.claude.com/docs/zh-CN/monitoring-usage.md) 或 [LLM 网关](https://code.claude.com/docs/zh-CN/llm-gateway) |

[OpenTelemetry 导出](https://code.claude.com/docs/zh-CN/monitoring-usage.md)适用于所有设置，并且是唯一能将按用户统计的 token 和成本指标近乎实时地流式传输到您自己的可观测性平台的选项。

### 按您的合同费率报告支出

默认情况下，Claude Code 向开发者显示的每个成本数字都按标价计算，因此如果您的组织支付的是合同费率，`/usage`、状态栏和 OpenTelemetry 中的数字将与您的账单不符。要使其一致，请将 [`modelPricing`](https://code.claude.com/docs/zh-CN/settings-reference#modelpricing) 托管设置设为您的费率。该设置改变的是 Claude Code 报告的内容，而不是 Anthropic 收取的费用。需要 Claude Code v2.1.242 或更高版本。

**1. 从合同中获取费率**

输入合同中的每百万 token 费率。Claude Code 不会从 Claude Console 获取这些费率，因此合同变更时请更新该设置。

**2. 编写设置**

将 `multiplier` 设为小于 1 的值以获得统一折扣，或设为大于 1 的值以进行加价；也可以在 `overrides` 下列出每个模型的四项按 token 费率，或两者同时使用。加价需要 Claude Code v2.1.271 或更高版本。[`modelPricing` 条目](https://code.claude.com/docs/zh-CN/settings-reference#modelpricing)中提供了结构说明和可直接粘贴的示例。

**3. 通过托管设置部署**

将其作为[托管设置](https://code.claude.com/docs/zh-CN/managed-settings.md)下发：服务器托管设置、MDM 策略、`managed-settings.json` 或[策略辅助程序](https://code.claude.com/docs/zh-CN/managed-settings.md#compute-the-policy-with-a-helper-program)。Claude Code 会忽略用户、项目和本地设置以及 `--settings` 中的该键。

要确认费率已生效，请在[已收到托管设置](https://code.claude.com/docs/zh-CN/managed-settings.md#read-the-source-in-%2Fstatus)的会话中运行 `/usage`：Session 区块的 `Total cost` 行会附带注释 `at your organization's configured rates`。这些数字仍是估算值，而非发票。`/model` 选择器中的每百万 token 价格仍保持标价。

### Claude for Teams 和 Enterprise

在 Claude for Teams 和 Enterprise 套餐中，每位成员的 Claude Code 用量从按席位分配的额度中扣除，该额度按滚动的五小时窗口和每周窗口重置。该额度与 Claude 聊天和 Cowork 共享，其大小取决于成员的[席位等级](https://support.claude.com/en/articles/11845131-use-claude-code-with-your-team-or-enterprise-plan)（Standard 或 Premium）。您的控制手段位于 claude.ai 管理控制台中，而不是 Claude Console 中。

* **查看支出**：[组织分析中的支出报告](https://support.claude.com/en/articles/12883420-view-usage-analytics-for-team-and-enterprise-plans)按用户和按模型显示估算支出，支持 CSV 导出，每日更新。该报告涵盖使用额度支出，并在启用使用额度后显示。席位额度内的用量不以美元计量。
* **查看采用情况**：[分析仪表板](https://claude.ai/analytics/claude-code)显示每日活跃用户、会话和贡献指标，并支持以 CSV 导出贡献数据。请参阅[使用分析跟踪团队用量](https://code.claude.com/docs/zh-CN/analytics.md)。
* **限制支出**：席位额度是默认上限。要让成员在超出额度后继续使用，请启用[使用额度](https://support.claude.com/en/articles/12429409-extra-usage-for-paid-claude-plans)，并在组织、群组或单个成员级别设置支出限制。
* **获取按用户统计的数据**：在 Enterprise 套餐中，[Enterprise Analytics API](https://platform.claude.com/docs/en/api/admin/analytics) 返回跨各 Claude 使用入口（包括 Claude Code）的按用户用量和成本报告。Primary Owner 可在 [claude.ai/analytics/api-keys](https://claude.ai/analytics/api-keys) 创建具有 `read:analytics` 作用域的密钥。在 Teams 套餐中，请导出[支出报告 CSV](https://support.claude.com/en/articles/12883420-view-usage-analytics-for-team-and-enterprise-plans)，其中列出了按用户和按模型的 token 用量及估算支出。

[Claude Enterprise 消耗指南](https://support.claude.com/en/articles/14782391-claude-enterprise-consumption-guide)是供管理员进行规划的参考资料。它说明了 Claude 聊天、Claude Code 和 Cowork 之间的消耗差异，并提供了用于预算编制的按用户美元起始参考值。为编码席位编制的预算应多于聊天席位：每个 Claude Code 轮次都携带文件内容、工具调用和多步推理，因此一次调试会话的消耗可能超过一整天的聊天。

### Claude Console

API 组织通过[工作区](https://platform.claude.com/docs/en/build-with-claude/workspaces)管理 Claude Code 支出。您可以针对 Claude Code 的总支出[设置工作区支出限制](https://platform.claude.com/docs/en/build-with-claude/workspaces#workspace-limits)，并在 Console 中[查看成本和用量报告](https://platform.claude.com/docs/en/build-with-claude/workspaces#usage-and-cost-tracking)。

> [!NOTE]
>
> 当您首次使用 Claude Console 账户对 Claude Code 进行身份验证时，系统会自动为您创建一个名为"Claude Code"的工作区。该工作区为您组织中所有 Claude Code 用量提供集中的成本跟踪和管理。您无法为该工作区创建 API 密钥；它专门用于 Claude Code 的身份验证和使用。
>
> 对于具有自定义速率限制的组织，该工作区中的 Claude Code 流量会计入您组织的整体 API 速率限制。您可以在 Claude Console 中该工作区的 Limits 页面上设置[工作区速率限制](https://platform.claude.com/docs/en/api/rate-limits#setting-lower-limits-for-workspaces)，以限制 Claude Code 的占用份额并保护其他生产工作负载。

对于按用户报告，[Console 仪表板](https://platform.claude.com/claude-code)显示每位成员的支出和已接受的代码行数，[Claude Code Analytics API](https://platform.claude.com/docs/en/build-with-claude/claude-code-analytics-api) 则可通过 [Admin API 密钥](https://platform.claude.com/settings/admin-keys)以编程方式返回相同的每日按用户指标。请参阅[面向 API 客户的分析](https://code.claude.com/docs/zh-CN/analytics.md#access-analytics-for-api-customers)。

#### 速率限制建议

为团队设置 Claude Code 时，请根据您的组织规模参考以下每用户每分钟 token 数（TPM）和每分钟请求数（RPM）建议：

| 团队规模 | 每用户 TPM | 每用户 RPM |
| - | - | - |
| 1-5 名用户 | 200k-300k | 5-7 |
| 5-20 名用户 | 100k-150k | 2.5-3.5 |
| 20-50 名用户 | 50k-75k | 1.25-1.75 |
| 50-100 名用户 | 25k-35k | 0.62-0.87 |
| 100-500 名用户 | 15k-20k | 0.37-0.47 |
| 500+ 名用户 | 10k-15k | 0.25-0.35 |

例如，如果您有 200 名用户，您可以为每位用户申请 20k TPM，即总计 400 万 TPM（200\*20,000 = 400 万）。

每用户 TPM 随团队规模增大而降低，因为在较大的组织中，同时使用 Claude Code 的用户比例往往较低。这些速率限制作用于组织级别，而不是针对单个用户，这意味着当其他人未在积极使用该服务时，单个用户可以暂时消耗超过其计算份额的量。

> [!NOTE]
>
> 如果您预计会出现并发用量异常高的场景（例如大型团体的现场培训），您可能需要为每位用户分配更高的 TPM。

### 云服务提供商

在 Amazon Bedrock、Google Cloud's Agent Platform 和 Microsoft Foundry 上，Claude Code 按 token 向您的云账户计费，支出控制位于您的云服务提供商的计费控制台中。Claude Code 不会将您云端的指标发送回 Anthropic，因此[分析仪表板](https://code.claude.com/docs/zh-CN/analytics.md)和 Claude Code Analytics API 不涵盖这部分用量。

对于按用户的成本归属，您有三种选择：

* **OpenTelemetry**：从每位开发者的机器[导出指标](https://code.claude.com/docs/zh-CN/monitoring-usage.md)到您自己的可观测性平台。无论使用哪家提供商，这都能为您提供按用户统计的 token 数、成本和工具活动。
* **Claude apps 网关**：自托管的 [Claude apps gateway](https://code.claude.com/docs/zh-CN/claude-apps-gateway) 在这些提供商上提供按用户的用量归属、包含 token 数的 OTLP 指标以及[按用户的支出限制](https://code.claude.com/docs/zh-CN/claude-apps-gateway-spend-limits.md)。
* **LLM 网关**：将所有 Claude Code 流量通过一个按密钥跟踪支出的代理进行路由。多家大型企业报告称使用了 [LiteLLM](https://code.claude.com/docs/zh-CN/llm-gateway)，这是一个可以[按密钥跟踪支出](https://docs.litellm.ai/docs/proxy/virtual_keys#tracking-spend)的开源工具。该项目与 Anthropic 无关，且未经过安全审计。

### 当开发者询问限制问题时

开发者通常会向管理员咨询限制问题，因此了解他们触及的是哪种上限会很有帮助。以下几种情况含义各不相同：

* **"You've hit your session limit"或"You've hit your weekly limit"**：订阅套餐中基于席位的用量窗口，由所有模型共享，因此开发者无法通过 `/model` 切换模型来恢复访问。该消息会显示窗口的重置时间。在出现特定于模型的"You've hit your Opus limit"或"You've hit your Sonnet limit"消息后，使用 `/model` 切换到该系列以外的模型确实可以让开发者继续工作。请参阅[用量限制错误](https://code.claude.com/docs/zh-CN/errors.md#youve-hit-your-session-limit)。在此期间开发者可以：
  * 如果您已启用[使用额度](https://support.claude.com/en/articles/12429409-extra-usage-for-paid-claude-plans)，运行 `/usage-credits` 申请超出额度的用量。
  * 在 Claude Code v2.1.234 或更高版本中，[等待重置后自动继续被中断的任务](https://code.claude.com/docs/zh-CN/interactive-mode.md#wait-for-a-usage-limit-to-reset)；该部分列出了 Claude Code 何时会自行开始等待，以及开发者何时需要从 `/rate-limit-options` 中选择等待。要为您的所有设备控制 Claude Code 是否自行开始该等待，请在[托管设置](https://code.claude.com/docs/zh-CN/settings.md#settings-precedence)中设置 [`autoContinueAtUsageLimit`](https://code.claude.com/docs/zh-CN/settings-reference#autocontinueatusagelimit)。
* **"You've hit your individual spend limit"、"org's monthly spend limit"或"team's shared budget"**：开发者的请求本应计入使用额度，而这些额度已达到您设置的支出限制。要让开发者继续使用，请前往 [**Organization settings > Usage**](https://claude.ai/admin-settings/usage) 并提高消息中提到的限制。如果消息中还指明了套餐重置时间，开发者也可以等到那时。各种变体请参阅[错误参考](https://code.claude.com/docs/zh-CN/errors.md#youve-hit-your-monthly-spend-limit)。
* **来自 [Claude apps gateway](https://code.claude.com/docs/zh-CN/claude-apps-gateway) 的支出限制消息**：开发者超出了您在自托管网关上设置的支出上限，网关会阻止其请求，直到周期重置或您提高上限。有关上限、重置计划以及开发者看到的消息，请参阅[网关支出限制](https://code.claude.com/docs/zh-CN/claude-apps-gateway-spend-limits.md)。
* **上下文或自动压缩警告**：这不是用量限制。对话已接近会话的[自动压缩窗口](https://code.claude.com/docs/zh-CN/model-config.md#set-the-auto-compact-window)，即 Claude Code 汇总较早历史记录以释放空间的阈值。请引导开发者参阅[减少 token 用量](#reduce-token-usage)。
* **API 或云服务提供商套餐上的意外高额支出**：通常源于从未清除的长会话，或将 Opus 保留为默认模型。最值得分享的高影响习惯是在不相关的任务之间进行清除，以及根据工作选择合适的模型，这两点都在[减少 token 用量](#reduce-token-usage)中有介绍。

### Agent team 的 token 成本

[Agent team](https://code.claude.com/docs/zh-CN/agent-teams.md) 会启动多个 Claude Code 实例，每个实例都有自己的上下文窗口。token 用量随活跃队友的数量以及每个队友的运行时长而增长。

要使 agent team 的成本保持在可控范围内：

* 为队友使用 Sonnet。它在协调任务的能力与成本之间取得了平衡。
* 保持团队规模较小。每个队友都运行自己的上下文窗口，因此 token 用量大致与团队规模成正比。
* 保持启动提示词简洁聚焦。队友会自动加载 CLAUDE.md、MCP 服务器和 skill，但启动提示词中的所有内容都会从一开始就增加其上下文。
* 在队友完成工作后将其关闭。每个活跃的队友都会持续消耗 token，直到其退出或会话结束。
* Agent team 默认处于禁用状态。在您的 [settings.json](https://code.claude.com/docs/zh-CN/settings.md) 或环境中设置 `CLAUDE_CODE_EXPERIMENTAL_AGENT_TEAMS=1` 以启用它们。请参阅[启用 agent team](https://code.claude.com/docs/zh-CN/agent-teams.md#enable-agent-teams)。

## 减少 token 使用

token 成本随上下文大小而扩展：Claude 处理的上下文越多，您使用的 token 就越多。Claude Code 通过[提示缓存](https://code.claude.com/docs/zh-CN/prompt-caching.md)（减少重复内容（如系统提示词）的成本）和自动压缩（在接近上下文限制时总结对话历史）自动优化成本。

以下策略可帮助您保持上下文较小并降低每条消息的成本。

### 主动管理上下文

使用 `/usage` 检查您当前的 token 使用情况，或[配置您的状态栏](https://code.claude.com/docs/zh-CN/statusline.md#context-window-usage)以持续显示它。

* **在任务之间清除**：在切换到不相关的工作时，使用 `/clear` 重新开始。陈旧的上下文会在随后的每条消息上浪费 token。在清除之前使用 `/rename`，以便您稍后可以找到该会话，然后使用 `/resume` 返回到它。
* **添加自定义压缩指令**：`/compact Focus on code samples and API usage` 告诉 Claude 在总结期间保留什么。

您还可以在项目根目录的 CLAUDE.md 文件中自定义压缩行为：

```markdown theme={null}
# Compact instructions

When you are using compact, please focus on test output and code changes
```

### 选择正确的模型

Sonnet 处理大多数编码任务效果很好，成本低于 Opus。为复杂的架构决策或多步推理保留 Opus。使用 `/model` 在会话中途切换模型，或在 `/config` 中设置默认值。对 Opus 的切换也适用于[继承您会话模型的子代理](https://code.claude.com/docs/zh-CN/model-config.md#setting-your-model)。对于简单的子代理任务，在您的[子代理配置](https://code.claude.com/docs/zh-CN/sub-agents.md#choose-a-model)中指定 `model: haiku`。

### 减少 MCP 服务器开销

MCP 工具定义[默认被延迟](https://code.claude.com/docs/zh-CN/mcp.md#scale-with-mcp-tool-search)，因此只有工具名称和服务器指令进入上下文，直到 Claude 使用特定工具。运行 `/context` 查看占用空间的内容。

* **在可用时优先使用 CLI 工具**：`gh`、`aws`、`gcloud` 和 `sentry-cli` 等工具仍比 MCP 服务器更节省上下文，因为它们不添加任何每工具列表。Claude 可以直接运行 CLI 命令。
* **禁用未使用的服务器**：运行 `/mcp` 查看已配置的服务器，并禁用您未积极使用的任何服务器。

### 为类型化语言安装代码智能插件

[代码智能插件](https://code.claude.com/docs/zh-CN/plugins/code-intelligence.md)为 Claude 提供精确的符号导航，而不是基于文本的搜索，减少在探索不熟悉的代码时不必要的文件读取。单个"转到定义"调用替代了可能需要的 grep 后跟读取多个候选文件。已安装的语言服务器还会在编辑后自动报告类型错误，因此 Claude 无需运行编译器即可捕获错误。

### 将处理卸载到 hook 和 skill

自定义 [hook](https://code.claude.com/docs/zh-CN/hooks.md) 可以在 Claude 看到数据之前对其进行预处理。Claude 不是读取 10,000 行日志文件来查找错误，hook 可以 grep `ERROR` 并仅返回匹配的行，将上下文从数万个 token 减少到数百个。

[skill](https://code.claude.com/docs/zh-CN/skills.md) 可以为 Claude 提供领域知识，这样它就不必进行探索。例如，"codebase-overview" skill 可以描述您的项目架构、关键目录和命名约定。当 Claude 调用该 skill 时，它会立即获得此上下文，而不是花费 token 读取多个文件来理解结构。

例如，此 PreToolUse hook 过滤测试输出以仅显示失败：

> [tab] settings.json
>
> 将此添加到您的 [settings.json](https://code.claude.com/docs/zh-CN/settings.md#where-settings-live)，以在每个 Bash 命令之前运行 hook：
>
> ```json theme={null}
> {
>   "hooks": {
>     "PreToolUse": [
>       {
>         "matcher": "Bash",
>         "hooks": [
>           {
>             "type": "command",
>             "command": "~/.claude/hooks/filter-test-output.sh"
>           }
>         ]
>       }
>     ]
>   }
> }
> ```

> [tab] filter-test-output.sh
>
> hook 调用此脚本。使用 `mkdir -p ~/.claude/hooks` 创建文件夹，将下面的脚本保存为 `~/.claude/hooks/filter-test-output.sh`，并使用 `chmod +x ~/.claude/hooks/filter-test-output.sh` 使其可执行。它检查命令是否为测试运行器并修改它以仅显示失败：
>
> ```bash theme={null}
> #!/bin/bash
> input=$(cat)
> cmd=$(echo "$input" | jq -r '.tool_input.command')
>
> # If running tests, filter to show only failures
> pattern='^(npm test|pytest|go test)( [^;&|<>`$#\\[:cntrl:]]*)?$'
> if [[ "$cmd" =~ $pattern ]]; then
>   filtered_cmd="$cmd 2>&1 | grep -A 5 -E '(FAIL|ERROR|error:)' | head -100"
>   echo "$input" | jq --arg filtered "$filtered_cmd" \
>     '{hookSpecificOutput: {hookEventName: "PreToolUse", updatedInput: (.tool_input + {command: $filtered})}}'
> else
>   echo "{}"
> fi
> ```

要验证设置，运行 `/hooks` 并检查 hook 是否出现在 PreToolUse 下。您也可以使用 `claude --debug-file ./claude-debug.txt` 启动 Claude Code 并要求 Claude 运行 `npm test`。当 hook 重写命令时，该日志文件包含一个 `modified tool input keys` 行，列出 `command` 和其他 Bash 输入字段。

### 将指令从 CLAUDE.md 移动到 skill

您的 [CLAUDE.md](https://code.claude.com/docs/zh-CN/memory.md) 文件在会话开始时加载到上下文中。如果它包含特定工作流的详细指令（如 PR 审查或数据库迁移），即使您在做不相关的工作时，这些 token 也会存在。[Skills](https://code.claude.com/docs/zh-CN/skills.md) 仅在调用时按需加载，因此将专门指令移动到 skill 中可以保持您的基础上下文较小。目标是通过仅包含必要内容来将 CLAUDE.md 保持在 200 行以下。

### 调整扩展思考

扩展思考默认启用，因为它显著改进了复杂规划和推理任务的性能。思考 token 作为输出 token 计费，默认预算可能是每个请求数万个 token，具体取决于模型。

对于不需要深度推理的更简单任务，您可以通过使用 `/effort` 或在 `/model` 中降低 [effort 级别](https://code.claude.com/docs/zh-CN/model-config.md#adjust-effort-level)、或在 `/config` 中禁用思考来降低成本。您无法在 Opus 5.5、Sonnet 5.5、Haiku 5.5 或 Fable 模型上关闭思考，它们始终使用扩展思考。

在具有[固定思考预算](https://code.claude.com/docs/zh-CN/model-config.md#adaptive-reasoning-and-fixed-thinking-budgets)的模型上，您也可以通过设置 `MAX_THINKING_TOKENS` [环境变量](https://code.claude.com/docs/zh-CN/env-vars.md)（例如 `MAX_THINKING_TOKENS=8000`）来降低预算。自适应推理模型忽略非零预算，因此请改用 effort 级别。

### 将冗长的操作委托给子代理

运行测试、获取文档或处理日志文件可能会消耗大量上下文。将这些委托给[子代理](https://code.claude.com/docs/zh-CN/sub-agents.md#isolate-high-volume-operations)，以便冗长的输出保留在子代理的上下文中，而只有摘要返回到您的主对话。

子代理自己的请求仍然会消耗您的使用量。为了在这些请求上花费更少，[为子代理选择更小的模型](https://code.claude.com/docs/zh-CN/sub-agents.md#choose-a-model)或[在一个模型上运行每个子代理](https://code.claude.com/docs/zh-CN/sub-agents.md#run-every-subagent-on-one-model)。

### 管理 agent team 成本

当队友在计划模式中运行时，agent team 使用的 token 大约是标准会话的 7 倍，因为每个队友维护自己的上下文窗口并作为单独的 Claude 实例运行。保持团队任务小且独立，以限制每个队友的 token 使用。有关详细信息，请参阅 [agent team](https://code.claude.com/docs/zh-CN/agent-teams.md)。

### 编写具体的提示词

模糊的请求（如"改进此代码库"）会触发广泛扫描。具体的请求（如"向 auth.ts 中的登录函数添加输入验证"）让 Claude 能够以最少的文件读取高效地工作。

### 高效处理复杂任务

对于较长或更复杂的工作，这些习惯有助于避免因走错路而浪费的 token：

* **对复杂任务使用计划模式**：在实现之前，按 Shift+Tab 切换到[计划模式](https://code.claude.com/docs/zh-CN/permission-modes.md#analyze-before-you-edit-with-plan-mode)。Claude 探索代码库并提出一个方法供您批准，防止当初始方向错误时的昂贵返工。
* **尽早纠正方向**：如果 Claude 开始朝错误的方向发展，按 Escape 立即停止。使用 `/rewind` 或双击 Escape 将对话和代码恢复到之前的检查点。
* **给出验证目标**：在您的提示词中包含测试用例、粘贴屏幕截图或定义预期输出。当 Claude 可以验证自己的工作时，它会在您需要请求修复之前捕获问题。
* **增量测试**：编写一个文件，测试它，然后继续。这样可以尽早发现问题。

## 后台令牌使用

会话回顾和空闲压缩可能会在您空闲时使用 token。有关各自的运行时机以及如何关闭，请参阅[会话回顾](https://code.claude.com/docs/zh-CN/interactive-mode.md#session-recap)和[空闲压缩](https://code.claude.com/docs/zh-CN/settings-reference.md#idlecompaction)。

启用提示建议时，Claude Code 在 Claude 响应后还会向您的会话使用的模型发送一个简短请求，以[建议您的下一个提示](https://code.claude.com/docs/zh-CN/interactive-mode.md#prompt-suggestions)。该请求重用对话的提示缓存，因此主要是缓存读取加上少量输出令牌。当您的账户接近或达到使用限制时，Claude Code [会跳过这些请求](https://code.claude.com/docs/zh-CN/interactive-mode.md#when-claude-code-skips-suggestions)。要停止这些请求，[关闭提示建议](https://code.claude.com/docs/zh-CN/interactive-mode.md#turn-prompt-suggestions-off)。

## 为什么长时间会话中使用量会增加

一个已经打开数小时的会话可能会使用远超你的活动量所暗示的计划限额，通常是由于以下原因之一：

* **长上下文**：Claude Code 在每个请求中发送你与它的完整对话，每当 Claude 使用工具时，它会发送另一个请求，其中包含该批工具结果。使用 [prompt caching](https://code.claude.com/docs/zh-CN/prompt-caching.md)，Claude Code 以 [缓存令牌速率](https://platform.claude.com/docs/en/about-claude/pricing) 重新读取该历史记录，因此即使在已打开一整天的会话中提出一行问题，仍然会为整个对话消耗使用量。请参阅 [主动管理上下文](#manage-context-proactively) 了解保持上下文较小的方法
* **缓存未命中**：在超过 [缓存生命周期](https://code.claude.com/docs/zh-CN/prompt-caching.md#cache-lifetime) 的中断后，您的第一条消息会错过缓存并重新处理您的完整上下文。请参阅 [您的请求使用哪种生命周期](https://code.claude.com/docs/zh-CN/prompt-caching.md#which-ttl-each-request-gets) 以及 [设置生命周期的控制方式](https://code.claude.com/docs/zh-CN/prompt-caching.md#choose-the-ttl-yourself)
* **定时任务**：[会话范围的定时任务](https://code.claude.com/docs/zh-CN/scheduled-tasks.md)即使在会话处于空闲状态时也会按其间隔触发，每次都发送您的完整上下文
* **跨会话消息**：当此会话处于空闲状态时，Claude Code 将 [来自你另一个会话的消息](https://code.claude.com/docs/zh-CN/cross-session-messaging.md) 作为新轮次传递，每次都发送你的完整上下文。要保留入站消息而不是传递它们，请将 [`crossSessionInbound`](https://code.claude.com/docs/zh-CN/settings-reference#crosssessioninbound) 设置为 `hold`
* **目标检查**：当后台工作使活跃的 [目标](https://code.claude.com/docs/zh-CN/goal.md) 保持等待时，Claude Code [要求 Claude 检查该工作](https://code.claude.com/docs/zh-CN/goal.md#background-work-defers-evaluation)，即使会话处于空闲状态，启动发送你完整上下文的新轮次。Claude Code 在你的提示之间每个目标最多启动三个空闲检查。在 v2.1.246 之前，空闲检查是无限制的。要关闭检查，请将 [`CLAUDE_CODE_GOAL_CHECKIN_MINUTES`](https://code.claude.com/docs/zh-CN/env-vars) 设置为 `0`。空闲检查需要 Claude Code v2.1.236 或更高版本
* **子代理和工作流**：每个子代理，以及每个 [动态工作流](https://code.claude.com/docs/zh-CN/workflows.md#cost) 生成的代理，都会在主对话的基础上发送自己的请求。[属性分解](#plan-usage-breakdown) 显示子代理份额
* **代理队友**：每个活跃的 [队友](#agent-team-token-costs) 会继续消耗令牌，直到它退出
* **压缩**：`/compact` 读取它总结的对话，因此 [压缩大型上下文](https://code.claude.com/docs/zh-CN/prompt-caching.md#compacting-the-conversation) 本身就是一个大型请求。当你想要全新开始而不是连续性时，`/clear` 不消耗任何成本

要了解在 Pro、Max、Ultra、Team 或 Enterprise 计划上应做哪些调整，请在 Claude Code 输入框中运行 [`/usage`](#plan-usage-breakdown)。其明细会针对占您最近使用量 10% 或更多的行为建议相应的调整。这些行为包括长上下文和缓存未命中。

## 了解 Claude Code 行为的变化

Claude Code 定期接收更新，这些更新可能会改变功能的工作方式，包括成本报告。运行 `claude --version` 来检查您当前的版本。

有关您特定账户的计费问题，请通过产品内信使联系 Anthropic 支持：

* **订阅套餐**（Pro、Max、Ultra、Team、Enterprise）：在 [claude.ai](https://claude.ai) 登录，点击左下角的您的首字母缩写，然后选择**获取帮助**
* **Console（API）计费**：在 [platform.claude.com](https://platform.claude.com) 登录，点击您的首字母缩写，然后选择**获取帮助**

请参阅[如何获取支持](https://support.claude.com/en/articles/9015913-how-to-get-support)了解完整流程，包括每个计划中谁可以联系人工代理。
