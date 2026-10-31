> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# 使用顾问工具升级困难决策

> 将您的主模型与更强大的顾问模型配对，Claude 在任务期间的关键时刻咨询该模型。

> [!NOTE]
>
> 顾问工具是实验性的，需要 Anthropic API。它在 Amazon Bedrock、Claude Platform on AWS、Google Cloud 的 Agent Platform 或 Microsoft Foundry 上不可用。行为、定价和可用性可能会改变。

顾问工具让 Claude 在任务期间的关键时刻咨询第二个通常更强大的模型，例如在提交方法之前、陷入重复错误时或在声明任务完成之前。顾问接收完整的对话，包括每个工具调用和结果，并返回 Claude 在继续之前应用的指导。

顾问在 Anthropic 基础设施上作为[服务器工具](https://platform.claude.com/docs/en/agents-and-tools/tool-use/advisor-tool)运行，可供订阅和 API 计费账户使用。您选择哪个模型充当顾问，Claude 决定何时调用它。

本页介绍如何启用顾问、接受哪些模型配对、Claude 在咨询期间显示什么，以及顾问使用如何计费。

## 何时使用顾问

顾问适合长期、多步骤的任务，其中大多数轮次是常规的，但计划质量决定了结果。示例包括大型重构、错误不断重复的调试会话，以及您希望在 Claude 声明完成之前独立检查的任务。

在短任务上（几乎没有计划的地方）或需要每一轮都使用最强模型的工作上，它的价值较少。对于这些情况，[切换主模型](https://code.claude.com/docs/zh-CN/model-config.md#setting-your-model)，或查看[顾问与 opusplan 和子代理的比较](#compare-with-related-features)以获取其他获取第二意见的方式。

## 启用顾问

您可以通过三种方式设置顾问模型：

* **`/advisor` 命令**：在会话中途设置或更改顾问，并将其保存为默认值
* **`advisorModel` 设置**：在您的[设置文件](https://code.claude.com/docs/zh-CN/settings.md)中配置持久默认值
* **`--advisor` 标志**：在启动时为单个会话设置顾问

这些方法中的每一种都会为主模型[支持它](#choose-an-advisor-model)的会话启用顾问。会话启动后，Claude Code 会显示一条 `Advisor Tool (experimental) is on and may use more tokens · /advisor` 通知。要停止使用顾问，请参阅[关闭顾问](#turn-the-advisor-off)。

在某些计划中，使用 Fable 作为顾问还需要您一次性[同意将 Fable 使用费用计入使用额度](https://code.claude.com/docs/zh-CN/model-config.md#fable-and-usage-credits)。有关您给予该同意之前会发生什么，请参阅 [Fable 顾问和使用额度](#fable-advisor-and-usage-credits)。

### 使用 `/advisor` 命令

运行不带参数的 `/advisor` 以打开列出可用顾问模型的选择器，或直接传递模型：

```
/advisor opus
```

该命令会确认 `Advisor set to` 后跟顾问模型名称。您的选择被保存到用户设置中的 `advisorModel`，并在会话之间持久化，除了 [`advisorModel` 条目](https://code.claude.com/docs/zh-CN/settings-reference#advisormodel)列出的仅适用于当前会话的情况。

该命令也适用于没有终端选择器的地方：在[非交互模式](https://code.claude.com/docs/zh-CN/headless.md)中使用 `-p`、在 Agent SDK 中、在桌面应用中以及通过 [Remote Control](https://code.claude.com/docs/zh-CN/remote-control.md)。这需要 Claude Code v2.1.260 或更高版本。在这些使用入口上：

* 运行不带参数的 `/advisor` 以打印当前顾问模型及其接受的别名。
* 运行带有模型的 `/advisor`，例如 `/advisor opus`，以设置它。
* 运行 `/advisor off` 以关闭它。

Claude Code 不会调用您的组织的 [`availableModels`](https://code.claude.com/docs/zh-CN/model-config#restrict-model-selection) 允许列表排除的已保存顾问。要使用顾问，请使用 `/advisor` 选择允许的模型。

Claude Code 仍然会保存您当前主模型不支持的顾问。该顾问在您使用 [`/model`](https://code.claude.com/docs/zh-CN/model-config#setting-your-model) 切换到[兼容的主模型](#choose-an-advisor-model)后激活。如果 API 已在当前对话中拒绝了已保存的顾问，它将保持关闭状态，直到 `/clear` 或 `/compact`，即使在您切换模型之后。

在某些计划中，使用 Fable 作为顾问还需要您一次性[同意将 Fable 使用费用计入使用额度](https://code.claude.com/docs/zh-CN/model-config.md#fable-and-usage-credits)。有关您给予该同意之前 `/advisor fable` 会做什么，请参阅 [Fable 顾问和使用额度](#fable-advisor-and-usage-credits)。

### 在设置中设置 `advisorModel`

要在不打开会话的情况下将顾问配置为默认值，请在设置文件中设置它：

```json theme={null}
{
  "advisorModel": "opus"
}
```

### 使用 `--advisor` 标志

要为单个会话设置顾问而不更改保存的设置，请使用该标志启动：

```bash theme={null}
claude --advisor opus
```

Claude Code 在该会话中使用该标志而不是 `advisorModel` 设置。它不会在 `claude --help` 中列出 `--advisor`。如果以下任何情况成立，Claude Code 在启动时会以错误退出：

* 会话的主模型不支持顾问
* 请求的模型（例如 Haiku 4.5）无法充当顾问
* 您的组织的 [`availableModels`](https://code.claude.com/docs/zh-CN/model-config#restrict-model-selection) 允许列表排除了请求的模型
* 您请求了 Fable，而您的账户仍然需要[使用额度同意](#fable-advisor-and-usage-credits)

如果您使用 `--advisor` 启动[后台会话](https://code.claude.com/docs/zh-CN/agent-view.md)，并且上述任何情况成立，Claude Code 会在没有顾问的情况下启动会话，而不是退出。

如果您传递的模型可以充当顾问，但[排名低于您的主模型](#choose-an-advisor-model)，则无论是否为后台会话，会话都会启动。随后您的主模型将在没有顾问的情况下运行。

## 选择顾问模型

Claude Code 按照担任顾问角色的能力对模型进行排名，顾问的排名必须等于或高于会话的主模型。各行按主模型从排名最低到最高排列：

| 主模型 | 接受的顾问 |
| - | - |
| Haiku 4.5 | Fable、Opus、Sonnet、Haiku 5.5 |
| Sonnet 4.6 | Fable、Opus、Sonnet、Haiku 5.5 |
| Opus 4.6 | Fable、Opus、Sonnet 5 或更高版本、Haiku 5.5 |
| Sonnet 5 或 Haiku 5.5 | Fable、Opus 4.7 或更高版本、Sonnet 5 或更高版本、Haiku 5.5 |
| Opus 4.7 或 Opus 4.8 | Fable、Opus 4.7 或更高版本、Sonnet 5.5 |
| Sonnet 5.5 | Fable、Opus 5 或更高版本、Sonnet 5.5 |
| Opus 5 或 Opus 5.5 | Fable、Opus 5 或更高版本 |
| Fable 5 | Fable 5.1 或 Fable 5 |
| Fable 5.1 | Fable 5.1 |

Fable 5.1 需要 Claude Code v2.1.257 或更高版本。Fable 模型需要 [Fable 访问权限](https://code.claude.com/docs/zh-CN/model-config.md#work-with-fable)。将 Sonnet 5.5 作为 Opus 4.7 或 Opus 4.8 主模型的顾问需要 Claude Code v2.1.287 或更高版本。将 Haiku 5.5 作为主模型或顾问需要 Claude Code v2.1.293 或更高版本。

将顾问设置为 `fable`、`opus` 或 `sonnet`。这些别名解析为 Claude Code 为每个模型系列[内置的默认版本](https://code.claude.com/docs/zh-CN/model-config.md#model-aliases)，该版本随新的 Claude Code 版本而推进。您也可以传递完整的模型 ID，例如 `claude-opus-5-5` 或 `claude-haiku-5-5`。Haiku 4.5 可以调用顾问，但不能充当顾问。

子代理继承配置的顾问，并对其自己的模型应用相同的配对检查。

Claude Code 在发送请求之前验证配对，API 也会再次验证：

* 对于排名低于主模型的顾问，Claude Code 不会将其附加到主模型的请求中。`/advisor` 命令输出和通知会显示这一点；请参阅[顾问的能力低于当前主模型](https://code.claude.com/docs/zh-CN/errors.md#advisor-is-less-capable-than-the-current-main-model)。其自己的模型满足配对的子代理仍然可以使用顾问。
* 如果 API 拒绝了 Claude Code 所附加顾问的配对，Claude Code 会在没有顾问的情况下重新发送该请求。对话会在没有顾问的情况下继续，因此您看不到错误，也不会获得顾问调用。如果您随后使用 `/advisor` 选择其他顾问，更改将在 `/clear` 或 `/compact` 之后以及新会话中生效。
* 如果主模型或顾问是 Claude Code 无法识别的模型，顾问不会附加。

### Fable 顾问和使用额度

在某些计划中，Fable 使用费用计入使用额度，Fable 作为顾问也以相同方式计费。如果您的账户需要 [一次性同意将 Fable 使用费用计入使用额度](https://code.claude.com/docs/zh-CN/model-config.md#fable-and-usage-credits)，当您使用 `/model` 选择 Fable 模型时，Claude Code 会要求您同意，在您接受该同意之前不会将 Fable 应用为顾问。

在您接受之前，当您输入 `/advisor fable` 或在 `/advisor` 选择器中选择 Fable 时，Claude Code 不会将 Fable 保存为顾问。它会改为指向您 `/model fable`。使用 `claude --advisor fable`，Claude Code 在启动时会退出并显示指向 `/model fable` 的消息。在 [后台会话](#use-the-advisor-flag) 中，它会在没有顾问的情况下启动会话，而不是退出。如果 Fable 已保存为您的 `advisorModel`，Claude Code 会在没有顾问的情况下发送请求。在支持顾问的交互式会话中，它还会显示指向 `/model fable` 的通知。

要接受同意，请运行 `/model fable` 并选择继续使用 Fable。Claude Code 会记录同意并 [将 Fable 保存为您选择的模型](https://code.claude.com/docs/zh-CN/model-config.md#default-model-setting)。然后选择 Fable 作为顾问。

### 常见模型配对

任何接受的配对都有效。这些组合以不同的方式平衡成本和能力：

| 配对 | 何时使用 |
| - | - |
| Sonnet 主模型 + Opus 顾问 | Sonnet 处理常规工作，并将规划、模糊失败和完成检查升级到 Opus |
| Sonnet 主模型 + Fable 顾问 | 在决策点进行 Fable 指导，无需全程运行 Fable。需要 Fable 访问权限 |
| Haiku 主模型 + Opus 顾问 | 最低成本的主模型具有强大的规划能力。预期成本高于仅 Haiku，但低于将主模型切换到 Sonnet 或 Opus |
| Opus 主模型 + Opus 顾问 | 第二个 Opus 审查第一个。对于高风险任务很有用，其中独立检查比成本更重要 |
| Fable 主模型 + Fable 顾问 | 当 Fable 可用时的最高能力配对。Claude Code 不会将 Opus 或 Sonnet 顾问应用于 Fable 主模型 |
| Sonnet 主模型 + Sonnet 顾问 | 用于捕捉常规疏忽的低成本第二意见 |

## Claude 何时咨询顾问

Claude 决定何时调用顾问。它倾向于在提交方法之前、错误不断重复时以及在声明任务完成之前咨询，但时间是由模型驱动的，而不是基于规则的。

您可以在提示中要求咨询，就像您会请求任何工具一样，例如 `consult the advisor before you continue`。没有设置来限制或强制顾问调用；如果您希望 Claude 在任务期间更频繁或更少地咨询顾问，请在您的说明中说明。

## 会话期间您看到的内容

当 Claude 调用顾问时，成绩单显示一条 `Advising` 行，其中包含顾问模型名称，同时调用正在进行中。当结果返回时，该行报告顾问是否提供了指导：

* **Reviewed**：该行确认顾问已审查对话。当顾问返回可读的指导时，按 `Ctrl+O` 阅读。
* **Declined**：该行显示 `Advisor declined to advise on this request`。如果顾问给出了原因，按 `Ctrl+O` 阅读。
* **Unavailable**：顾问调用失败，该行读取 `Advisor unavailable (<error_code>)`，其中 `<error_code>` 是调用返回的代码。

Claude 通常遵循顾问的指导，但在其自己的证据与特定声明相矛盾时进行调整：如果推荐的步骤在尝试时失败，或文件内容与建议相矛盾，Claude 会显示冲突而不是无条件地遵循指导。

顾问始终接收完整的对话，Claude 控制时间。为了获得更多控制或不同的配置，请参阅[顾问与子代理和 opusplan 的比较](#compare-with-related-features)。

## 成本

当 Claude 调用顾问时，顾问模型会读取对话，因此除了主模型的使用外，每次调用都会以顾问模型的费率消耗令牌。这些顾问令牌如何计费取决于您的付款方式：

* **API 计费**：您需要按顾问模型的输入和输出费率为顾问令牌付费
* **订阅计划**：顾问使用计入您的计划使用限制，但 Fable 顾问在支持 Fable 使用的计划上计入[使用额度](https://code.claude.com/docs/zh-CN/model-config.md#fable-and-usage-credits)

如果您的账户需要使用额度同意，Fable 顾问在您给予同意之前不会产生任何费用，因为 Claude Code [在那之前不会应用该选择](#fable-advisor-and-usage-credits)。

Claude 在决策点而不是每一轮都调用顾问，因此将更快的主模型与更强的顾问配对通常比全程运行更强的模型成本更低。顾问使用计入由 [`/usage`](https://code.claude.com/docs/zh-CN/costs#track-your-costs) 显示的会话总计。

有关顾问令牌如何在 API 响应中报告的信息，请参阅 Claude API 文档中的[使用和计费](https://platform.claude.com/docs/en/agents-and-tools/tool-use/advisor-tool#usage-and-billing)。

## 对提示缓存的影响

在会话中途启用或禁用顾问不会使主模型的[提示缓存](https://code.claude.com/docs/zh-CN/prompt-caching.md)失效。与[切换模型](https://code.claude.com/docs/zh-CN/prompt-caching.md#switching-models)不同，切换 `/advisor` 会保持缓存的前缀完整，顾问返回的指导在后续轮次中作为成绩单的一部分被缓存。

顾问模型自己对对话的读取不被缓存。每个顾问调用都会重新处理完整的成绩单，调用之间没有重用。

## 要求

顾问工具需要以下所有条件：

* **仅 Anthropic API**：顾问是服务器执行的工具。它在 Amazon Bedrock、Claude Platform on AWS、Google Cloud 的 Agent Platform 或 Microsoft Foundry 上不可用。通过配置了 `ANTHROPIC_BASE_URL` 的 [LLM 网关](https://code.claude.com/docs/zh-CN/llm-gateway)，可用性取决于网关是否将请求完整转发到 Anthropic API。如果网关或其上游不识别顾问工具，请参阅[自动重试和错误转发](https://code.claude.com/docs/zh-CN/llm-gateway-protocol.md#automatic-retry-and-error-forwarding)了解 Claude Code 如何响应。
* **支持的主模型**：Fable、Opus 4.6 或更高版本、Sonnet 4.6 或更高版本、Haiku 4.5 或 Haiku 5.5。请参阅[选择顾问模型](#choose-an-advisor-model)了解每个主模型接受哪些顾问。
* **功能标志获取**：Claude Code 通过从 Anthropic 获取的功能标志来启用顾问。在设置了关闭标志获取的变量（例如 `DISABLE_TELEMETRY`）的会话中，顾问保持关闭状态。请参阅[需要功能标志获取的功能](https://code.claude.com/docs/zh-CN/env-vars.md#features-that-need-feature-flag-fetching)。

## 关闭顾问

要停止使用顾问，运行 `/advisor off` 或在 `/advisor` 选择器中选择 **No advisor**：

```
/advisor off
```

要完全禁用顾问工具，设置 `CLAUDE_CODE_DISABLE_ADVISOR_TOOL=1`。`/advisor` 命令变为不可用，任何配置的 `advisorModel` 都会被忽略。`--advisor` 标志被接受但没有效果。请参阅[环境变量](https://code.claude.com/docs/zh-CN/env-vars.md)。

## 与相关功能比较

顾问是结合模型优势的几种方式之一。根据您希望何时涉及第二个模型来选择。

| 方法 | 更强的模型何时运行 | 如何启动 |
| - | - | - |
| 顾问工具 | 在任务中途的决策点 | Claude 在需要指导时调用它 |
| [`opusplan`](https://code.claude.com/docs/zh-CN/model-config#opusplan-model-setting) | 在计划模式期间当[由 `availableModels` 允许](https://code.claude.com/docs/zh-CN/model-config#restrict-model-selection)时，然后切换到 Sonnet 执行 | 您进入计划模式 |
| [子代理](https://code.claude.com/docs/zh-CN/sub-agents.md#choose-a-model)，设置了 `model` | 对于整个委派的子任务 | Claude 委派，或您调用子代理 |
| [`/model`](https://code.claude.com/docs/zh-CN/model-config#setting-your-model) | 从下一个请求开始 | 您切换模型 |

## 另请参阅

* [模型配置](https://code.claude.com/docs/zh-CN/model-config.md)：切换模型、设置努力级别并使用 `opusplan`
* [有效管理成本](https://code.claude.com/docs/zh-CN/costs.md)：跨模型跟踪令牌使用情况
* [Claude API 中的顾问工具](https://platform.claude.com/docs/en/agents-and-tools/tool-use/advisor-tool)：了解底层服务器工具，或直接从 Messages API 使用它
* [顾问策略](https://claude.com/blog/the-advisor-strategy)：为什么将快速主模型与更强的顾问配对有效
