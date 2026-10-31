> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# 使用快速模式加快响应速度

> 通过切换快速模式在 Claude Code 中获得更快的 Opus 响应。

> [!NOTE]
>
> Claude Opus 5.5 的快速模式现已可用，Claude Opus 5 和 Claude Opus 4.8 的快速模式目前为[研究预览版](#research-preview)。

快速模式是 Claude Opus 的高速配置，使模型速度提高最多 2.5 倍，但每个令牌的成本更高。当您需要速度进行交互式工作（如快速迭代或实时调试）时，使用 `/fast` 将其打开，当成本比延迟更重要时，将其关闭。

快速模式不是一个不同的模型。它使用 Claude Opus，但采用不同的 API 配置，优先考虑速度而不是成本效率。您获得相同的质量和功能，只是响应速度更快。快速模式在 Opus 5.5、Opus 5 和 Opus 4.8 上受支持。它在 Sonnet、Haiku 或其他模型上不可用。

Opus 4.7 不支持快速模式，因此切换到它会关闭快速模式。Opus 4.7 的快速模式已于 2026 年 6 月 25 日弃用，并于 2026 年 7 月 24 日移除。

需要了解的内容：

* 使用 `/fast` 在 Claude Code CLI 中切换快速模式。[VS Code 扩展](https://code.claude.com/docs/zh-CN/vs-code.md)在选定的模型支持快速模式时提供**切换快速模式**命令。Claude Code 将该切换保存到您的 [`fastMode` 设置](#toggle-fast-mode)。
* 快速模式定价在 Opus 5.5 上为 $8/$40 MTok 输入/输出，在 Opus 5 和 Opus 4.8 上为 $10/$50 MTok 输入/输出。
* 可供订阅套餐（Pro/Max/Ultra/Team/Enterprise）上的 Claude Code 用户和 Claude 控制台使用。Team 和 Enterprise 组织需要所有者先启用它，Console 组织需要先配置访问权限才能在 Opus 5 或 Opus 4.8 上使用它。[要求](#requirements)对这两者进行了说明。
* 对于订阅套餐（Pro/Max/Ultra/Team/Enterprise）上的 Claude Code 用户，快速模式仅通过使用额度提供，不包含在订阅速率限制中。

## 切换快速模式

在 CLI 中，可以通过以下任一方式切换快速模式：

* 运行 `/fast`，按 Space 切换开启或关闭，然后按 Enter 确认
* 在[用户设置文件](https://code.claude.com/docs/zh-CN/settings.md)中设置 `"fastMode": true`

默认情况下，在交互式会话中开启的快速模式会跨会话保留。您可以将快速模式配置为每个会话重置。有关详细信息，请参阅[要求按会话选择启用](#require-per-session-opt-in)。

在[云端会话](#use-fast-mode-in-cloud-sessions)之外，在使用 `-p` 标志的[非交互模式](https://code.claude.com/docs/zh-CN/headless.md)下，`/fast` 仅在其 [`--settings`](https://code.claude.com/docs/zh-CN/cli-reference#cli-flags) 值中包含快速模式的会话中有效，例如 `claude -p --settings '{"fastMode": true}'`；此时切换仅适用于该会话，不会保存为您的默认设置。`-p` 形式需要 Claude Code v2.1.205 或更高版本。在非交互模式的其他情况下，该命令会报告快速模式不可用。

您可以在 Claude 工作时运行 `/fast`，Claude Code 会切换快速模式，而无需等待当前轮次结束。Claude Code 会以原来的速度完成正在运行的轮次，因此速度变化从您的下一轮次开始生效。如果您当前的模型不支持快速模式，开启快速模式还会切换您的模型，Claude Code 会从该轮次中的下一个请求开始使用新模型。

为获得最佳成本效益，请在会话开始时启用快速模式，而不是在对话中途切换。有关详细信息，请参阅[了解成本权衡](#understand-the-cost-tradeoff)。

启用快速模式时：

* 如果您当前的模型不支持快速模式，Claude Code 会切换到 Opus
* 您将看到一条确认消息："Fast mode ON"
* 快速模式处于活动状态时，提示词旁边会出现一个小的 `↯` 图标
* 随时再次运行 `/fast` 可检查快速模式是开启还是关闭

在 Claude Code v2.1.280 及更高版本中，Opus 5.5 是快速模式的默认模型。在 v2.1.280 之前，从 v2.1.219 起快速模式默认使用 Opus 5。

当您再次使用 `/fast` 禁用快速模式时，您仍将使用 Opus。要切换到其他模型，请使用 `/model`。

### 在快速模式开启时切换模型

快速模式会双向跟随您的模型切换：

* **切换离开**：当您切换到不支持快速模式的模型时，Claude Code 会关闭快速模式。这包括 Opus 4.7；在 v2.1.221 之前，切换到 Opus 4.7 后快速模式仍保持开启，API 会拒绝这些请求。
* **切换回来**：当您保存的快速模式偏好为开启时（新会话默认即从该偏好开始），切换回受支持的 Opus 模型会再次开启快速模式。对于保存的偏好为关闭的会话，模型切换永远不会开启快速模式；如果配置了[按会话选择启用](#require-per-session-opt-in)，切换回来也不会开启快速模式；请运行 `/fast` 重新启用。

每当模型切换开启或关闭快速模式时，Claude Code 都会显示 `Fast mode ON` 或 `Fast mode OFF` 确认消息，并且在快速模式开启时会出现 `↯` 图标。无论您是使用 `/model`、使用 [`/config model=<model>`](https://code.claude.com/docs/zh-CN/settings) 还是从通过 [Remote Control](https://code.claude.com/docs/zh-CN/remote-control.md) 连接的设备进行切换，都是如此。

在模型切换、重新连接或[可用性检查](#use-fast-mode-behind-proxies-and-llm-gateways)失败后，Claude Code 会将会话的快速模式状态重新发送到通过 Remote Control 连接的设备。

### 在云端会话中使用快速模式

当您的账户可以使用快速模式时，快速模式可在[云端会话](https://code.claude.com/docs/zh-CN/claude-code-on-the-web.md)中使用，无论会话是在 Anthropic 托管的基础设施上运行还是在[自托管运行器](https://code.claude.com/docs/zh-CN/self-hosted-environments)上运行。需要会话环境中的 Claude Code 为 v2.1.271 或更高版本。

在会话中输入 `/fast on` 以开启快速模式。它仅对该会话保持开启，不会保存为您的默认设置。[要求](#requirements)同样适用于云端会话。

在 [claude.ai/code](https://claude.ai/code) 的浏览器中，您还可以从消息框上的模型菜单开启和关闭快速模式。当您的套餐包含快速模式且所选模型支持快速模式时，菜单会显示该开关。

## 了解成本权衡

快速模式的每个令牌定价高于标准 Opus：

| 模型 | 输入 (MTok) | 输出 (MTok) |
| - | - | - |
| Opus 5.5 | \$8 | \$40 |
| Opus 5 | \$10 | \$50 |
| Opus 4.8 | \$10 | \$50 |

快速模式定价在整个 1M 令牌上下文窗口中是固定的。有关要比较的标准 Opus 费率，请参阅 [Claude 定价参考](https://platform.claude.com/docs/zh-CN/about-claude/pricing)。

在对话中首次启用快速模式时，您需要为整个对话上下文支付完整的快速模式未缓存输入 token 价格。对话进行得越深入，成本就越高，因此在对话开始时启用快速模式的费用最低。快速模式和标准速度使用[各自独立的缓存](https://code.claude.com/docs/zh-CN/prompt-caching.md#turning-on-fast-mode)，因此之后的每次切换都会再次以未缓存方式读取部分或全部对话。

### 查看快速模式支出出现的位置

您看到快速模式支出的位置取决于您如何登录，因此首先运行 [`/status`](https://code.claude.com/docs/zh-CN/commands) 来检查。如果它显示 `Login method` 行（例如 `Claude Max account`），则您使用 Claude 订阅登录。如果它显示 `API key` 行，则您的请求计费到 Claude Console 组织。

* **Pro、Max 和 Ultra**：您从使用额度中支付快速模式费用。转到 claude.ai 上的 [**Settings > Usage**](https://claude.ai/settings/usage)，其中 **Usage credits** 部分显示您本月在使用额度中花费了多少。该数字包括快速模式，但不单独列出。
* **Team 和 Enterprise**：您的组织从其使用额度中支付您的快速模式使用费用。要查看您自己的使用额度支出，运行 [`/usage`](https://code.claude.com/docs/zh-CN/costs#check-your-usage-credits-spend)。有关您的组织在哪里看到该支出，请参阅 [Claude for Teams and Enterprise](https://code.claude.com/docs/zh-CN/costs.md#claude-for-teams-and-enterprise)。
* **Claude Console**：您的组织使用其 API 使用的其余部分支付快速模式费用。在 Console [Usage](https://platform.claude.com/usage) 和 [Cost](https://platform.claude.com/cost) 页面上，在 **Group by** 菜单中选择 **Speed (Research Preview)** 以将快速模式与标准速度使用分开。只有当选定的日期范围包括快速模式使用时，您才会看到该选项。

## 决定何时使用快速模式

快速模式最适合响应延迟比成本更重要的交互式工作：

* 快速迭代代码更改
* 实时调试会话
* 时间敏感的工作，有紧迫的截止日期

标准模式更适合：

* 速度不那么重要的长期自主任务
* 批处理或 CI/CD 管道
* 成本敏感的工作负载

### 快速模式与努力级别

快速模式和努力级别都会影响响应速度，但方式不同：

| 设置 | 效果 |
| - | - |
| **快速模式** | 相同的模型质量，更低的延迟，更高的成本 |
| **较低的努力级别** | 更少的思考时间，更快的响应，在复杂任务上可能质量较低 |

您可以结合两者：在直接任务上使用快速模式和较低的[努力级别](https://code.claude.com/docs/zh-CN/model-config.md#adjust-effort-level)以获得最大速度。

## 要求

快速模式需要以下所有条件：

* **仅限 Anthropic API 或订阅**：快速模式可通过 Anthropic 控制台 API 和使用使用额度的 Claude 订阅套餐获得。它在 Amazon Bedrock、Google Cloud 的 Agent Platform、Microsoft Foundry 或 AWS 上的 Claude Platform 上不可用。控制台组织还必须[配置快速模式访问权限](#enable-fast-mode-for-your-organization)，才能在 Opus 5 或 Opus 4.8 上使用快速模式。
* **为订阅套餐启用使用额度**：在 Pro、Max、Ultra、Team 或 Enterprise 套餐上，您的账户必须[启用使用额度](https://code.claude.com/docs/zh-CN/costs.md#add-usage-credits-to-your-subscription)，这允许在您的套餐包含的使用量之外进行计费。在启用之前，`/fast` 显示"Fast mode requires usage credits"。要启用它们，请按照适用于您套餐的步骤操作：
  * 在 Pro、Max 和 Ultra 上，在 claude.ai 上的[**Settings > Usage**](https://claude.ai/settings/usage)的**Usage credits**部分中启用它们，或运行 `/usage-credits` 来打开该页面。
  * 在 Team 和 Enterprise 上，具有计费访问权限的成员在[**Organization settings > Usage**](https://claude.ai/admin-settings/usage)处为组织启用它们，没有访问权限的成员运行 `/usage-credits` 向组织的管理员发送请求。

> [!NOTE]
>
> 快速模式使用直接计入使用额度，即使您的计划上还有剩余使用量。

* **付费控制台组织**：Claude 控制台账户不使用使用额度，您的组织按令牌为快速模式付费，与其余 API 使用量一起。在控制台的免费 Evaluation 计划上，`/fast` 显示"Fast mode unavailable during evaluation. Please purchase credits."。要清除它，在您的[控制台计费设置](https://platform.claude.com/settings/billing)中购买额度。

  Opus 5.5 上的快速模式还取决于您的组织的[使用层级](https://platform.claude.com/docs/en/api/rate-limits)。
* **团队和企业的所有者启用**：快速模式默认对团队和企业组织禁用。所有者必须明确[启用快速模式](#enable-fast-mode-for-your-organization)，用户才能访问它。

> [!NOTE]
>
> 四个组织设置可以阻止使用 `/fast` 启用快速模式：
>
> * **快速模式未启用**：如果您的组织尚未启用快速模式，使用 `/fast` 启用快速模式会显示"Fast mode has been disabled by your organization."。
> * **快速模式被托管设置关闭**：如果您的组织部署[托管设置](https://code.claude.com/docs/zh-CN/managed-settings.md)，设置 [`fastMode: false`](https://code.claude.com/docs/zh-CN/settings-reference#fastmode)，使用 `/fast` 启用快速模式会显示相同的"Fast mode has been disabled by your organization"消息。
> * **需要每个会话选择加入**：托管设置设置 [`fastModePerSessionOptIn: true`](#require-per-session-opt-in) 在除交互式终端会话外的任何地方都拒绝 `/fast on`，显示相同的消息。
> * **快速模式模型不允许**：如果您的组织的 [`availableModels`](https://code.claude.com/docs/zh-CN/model-config#restrict-model-selection) 允许列表排除了快速模式 Opus 模型，启用它会被拒绝，显示"is not in your organization's allowed models"。在已在支持快速模式的允许 Opus 模型上运行的会话中，`/fast` 改为在您当前的模型上启用快速模式，而不是切换模型。

### 为您的组织启用快速模式

您启用快速模式的位置取决于您的组织使用的产品：

* **控制台**（API 客户）：管理员在 [Claude Code 偏好设置](https://platform.claude.com/claude-code/preferences)中启用它
* **Claude AI**（团队和企业）：所有者在[**Organization settings > Claude Code**](https://claude.ai/admin-settings/claude-code)中启用它

使用 Claude Opus 5 或 Claude Opus 4.8 的控制台组织还需要配置访问权限，如 [Claude API 上的快速模式](https://platform.claude.com/docs/en/build-with-claude/fast-mode)中所述。如果没有该访问权限，这些模型上的快速模式请求会触发[快速模式速率限制](#handle-rate-limits)，并回退到标准速度，直到配置了访问权限。

另一个完全禁用快速模式的选项是设置 `CLAUDE_CODE_DISABLE_FAST_MODE=1`。请参阅[环境变量](https://code.claude.com/docs/zh-CN/env-vars.md)。

### 在代理和 LLM 网关后面使用快速模式

在提供快速模式之前，Claude Code 会通过直接请求 `api.anthropic.com` 来检查您的组织的快速模式可用性。该检查不遵循 [`ANTHROPIC_BASE_URL`](https://code.claude.com/docs/zh-CN/llm-gateway-connect#set-the-base-url-and-credential)，因此在通过 [LLM 网关](https://code.claude.com/docs/zh-CN/llm-gateway)路由 Claude 流量并阻止直接出站到 `api.anthropic.com` 的网络上，即使推理请求有效，检查也会失败。该检查确实使用配置的 [HTTP 代理](https://code.claude.com/docs/zh-CN/network-config.md#proxy-configuration)，因此网络块仅在 `api.anthropic.com` 即使通过代理也无法到达的地方才会导致检查失败。

如果您的组织已启用快速模式，而 `/fast` 显示的结果与之不符，请在以下情况中查找它显示的消息。

#### Fast mode unavailable due to network connectivity issues

可用性检查失败（例如因为您的网络阻止了 `api.anthropic.com`），请求以标准速度运行。过去成功的检查会从其缓存结果继续工作，因此被阻止的检查主要影响新安装。

要恢复快速模式，请在允许列表中允许直接出站到 `api.anthropic.com`，或设置 `CLAUDE_CODE_SKIP_FAST_MODE_NETWORK_ERRORS=1`，它会将失败的检查视为可用，并仍然遵守"disabled by your organization"响应。

#### Fast mode is currently unavailable

在网关后面时，请检查是否存在以下配置，在这些配置中 Claude Code 不会发送可用性检查，且没有缓存之前的成功检查：

* 您将 `ANTHROPIC_BASE_URL` 指向您的网关，并将 [`ANTHROPIC_API_KEY`](https://code.claude.com/docs/zh-CN/llm-gateway-connect#set-the-base-url-and-credential) 设置为网关颁发的、不符合 Anthropic API 密钥格式的密钥。
* 您设置了 [`CLAUDE_CODE_DISABLE_NONESSENTIAL_TRAFFIC`](https://code.claude.com/docs/zh-CN/env-vars)，因此 Claude Code 会跳过该检查。

要在上述任一配置中恢复快速模式，请设置 `CLAUDE_CODE_SKIP_FAST_MODE_NETWORK_ERRORS=1`。

#### Fast mode has been disabled by your organization

在网关后面时，即使您的组织已启用快速模式，在以下任一配置中您也可能看到此消息：

* 您仅使用 [`ANTHROPIC_AUTH_TOKEN`](https://code.claude.com/docs/zh-CN/llm-gateway-connect#set-the-base-url-and-credential) 进行身份验证，没有 claude.ai 登录、没有 API 密钥，也没有缓存的成功检查。此时 Claude Code 会将快速模式视为由您的组织禁用，而不发送请求。
* 代理拦截了检查并用自己的页面进行回答，例如返回 HTTP 200 阻止页面的 TLS 检查代理。Claude Code 会将该页面读取为表明您的组织已禁用快速模式的响应。

在这两种情况下，请设置 `CLAUDE_CODE_SKIP_FAST_MODE_ORG_CHECK=1`（它会完全跳过检查），而不是 `CLAUDE_CODE_SKIP_FAST_MODE_NETWORK_ERRORS`（它不适用于其中任何一种情况）。

> [!NOTE]
>
> 跳过变量仅更改客户端检查。当您的组织禁用了快速模式时，无论是否设置了这些变量，API 都会拒绝快速模式请求。随后 Claude Code 会以标准速度重试该请求，关闭快速模式，并且 `/fast` 报告您的组织已禁用快速模式。

### 要求每个会话选择加入

默认情况下，用户在交互式会话中启用的快速模式会在会话之间保持。要更改此行为，在任何[设置文件](https://code.claude.com/docs/zh-CN/settings.md#where-settings-live)中将 `fastModePerSessionOptIn` 设置为 `true`，这会导致每个会话以快速模式关闭开始，并要求用户使用 `/fast` 明确启用它。[Team](https://claude.com/pricing?utm_source=claude_code\&utm_medium=docs\&utm_content=fast_mode_teams#team-&-enterprise) 或 [Enterprise](https://anthropic.com/contact-sales?utm_source=claude_code\&utm_medium=docs\&utm_content=fast_mode_enterprise) 计划上的所有者可以通过[服务器托管设置](https://code.claude.com/docs/zh-CN/server-managed-settings.md)在组织范围内部署它。

```json theme={null}
{
  "fastModePerSessionOptIn": true
}
```

这对于在用户运行多个并发会话的组织中控制成本很有用。用户的快速模式偏好仍然被保存，因此删除此设置会恢复默认的持久行为。

当托管设置设置该密钥时，`/fast on` 仅在交互式终端会话中有效。在其他任何地方，包括[非交互式模式](https://code.claude.com/docs/zh-CN/headless.md)、[VS Code 扩展](https://code.claude.com/docs/zh-CN/vs-code.md)和[云会话](#use-fast-mode-in-cloud-sessions)，它会被拒绝，显示您的组织已禁用快速模式的消息。

## 处理速率限制

快速模式与标准 Opus 有单独的速率限制；对于 Console 组织，Claude Platform 的[速率限制](https://platform.claude.com/docs/en/api/rate-limits)页面对此进行了说明。当您达到快速模式速率限制时：

1. 快速模式自动回退到标准速度
2. `↯` 图标变灰以指示冷却
3. 您继续以标准速度和定价工作
4. 冷却过期时，快速模式自动重新启用

要手动禁用快速模式而不是等待冷却，请再次运行 `/fast`。

如果您在会话中途用完使用额度，Claude Code 会在标准速度和定价下重试每个被拒绝的快速模式请求，因此您可以继续工作，没有冷却。您看到拒绝的方式取决于会话类型：

* 在交互式会话中，Claude Code 显示"快速模式已禁用 · 使用额度已耗尽"通知，并在会话的其余部分关闭快速模式。您保存的快速模式偏好设置不会改变；运行 `/fast` 以重新打开快速模式。
* 在[非交互式模式](https://code.claude.com/docs/zh-CN/headless.md)中使用 `--output-format stream-json`，以及通过 Agent SDK，Claude Code 在消息流上作为 `system` 消息发出相同的文本，子类型为 `notification`，在您用完使用额度时每轮一次。快速模式保持打开。需要 Claude Code v2.1.221 或更高版本。

## 研究预览

Claude Opus 5 和 Claude Opus 4.8 的快速模式是一个研究预览功能。这意味着：

* 该功能可能会根据反馈而改变
* 可用性和定价可能会改变
* 底层 API 配置可能会演变

通过您通常的 Anthropic 支持渠道报告问题或反馈。

## 另请参阅

* [模型配置](https://code.claude.com/docs/zh-CN/model-config.md)：切换模型并调整努力级别
* [有效管理成本](https://code.claude.com/docs/zh-CN/costs.md)：跟踪令牌使用情况并降低成本
* [状态行配置](https://code.claude.com/docs/zh-CN/statusline.md)：显示模型和上下文信息
