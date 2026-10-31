> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# 模型配置

> 配置 Claude Code 使用的模型、工作量级别、扩展上下文和自动压缩窗口

## 可用模型

对于 Claude Code 中的 `model` 设置，您可以配置以下任一项：

* 一个**模型别名**
* 一个**模型名称**
  * Anthropic API：完整的\*\*[模型名称](https://platform.claude.com/docs/en/about-claude/models/overview)\*\*
  * Amazon Bedrock：推理配置文件 ARN
  * Microsoft Foundry：部署名称
  * Google Cloud 的 Agent Platform：版本名称

有关哪种模型和 effort 级别适合不同类型工作的指导，请参阅博客上的 [Choosing a Claude model and effort level in Claude Code](https://claude.com/blog/claude-model-and-effort-level-in-claude-code)。

> [!NOTE]
>
> `ANTHROPIC_BASE_URL` 改变的是请求发送的位置，而不是由哪个模型回答。要通过 LLM 网关路由 Claude，请参阅 [LLM gateways](https://code.claude.com/docs/zh-CN/llm-gateway)。

### 模型别名

使用模型别名来选择模型设置，而无需记住确切的版本号：

| 模型别名 | 行为 |
| - | - |
| **`default`** | 特殊值，清除任何模型覆盖并恢复到[您账户的运行时默认值](#default-model-setting)。本身不是模型别名 |
| **`best`** | 如果 Fable 对您可用，则使用 [`fable` 别名解析到的模型](#fable-alias-resolution)，否则使用与 `opus` 相同的模型 |
| **`fable`** | 为您最困难和运行时间最长的任务使用[您的提供商的 Fable 模型](#fable-alias-resolution) |
| **`sonnet`** | 为日常编码任务使用最新的 Sonnet 模型 |
| **`opus`** | 为复杂推理任务使用最新的 Opus 模型 |
| **`haiku`** | 为简单任务使用快速高效的 Haiku 模型 |
| **`sonnet[1m]`** | 为长会话使用具有 [100 万 token 上下文窗口](https://platform.claude.com/docs/en/build-with-claude/context-windows#context-window-sizes-by-model) 的 Sonnet。当 `sonnet` 已解析到具有原生 1M 窗口的 Sonnet 5.5 或 Sonnet 5 时无效 |
| **`opus[1m]`** | 为长会话使用具有 [100 万 token 上下文窗口](https://platform.claude.com/docs/en/build-with-claude/context-windows#context-window-sizes-by-model) 的 Opus。当 `opus` 已解析到具有原生 1M 窗口的 Opus 4.7 或更高版本时无效 |
| **`opusplan`** | 特殊模式，在计划模式期间使用 `opus`，然后在执行期间切换到 `sonnet` |

`opus`、`sonnet` 和 `haiku` 别名在 Anthropic API 上解析到最新版本，在其他一些提供商上解析到较早的版本：

| 提供商 | `opus` | `sonnet` | `haiku` |
| :- | :- | :- | :- |
| Anthropic API | Opus 5.5 | Sonnet 5.5 | Haiku 5.5 |
| [Claude Platform on AWS](https://code.claude.com/docs/zh-CN/claude-platform-on-aws.md) | Opus 5.5 | Sonnet 4.6 | Haiku 4.5 |
| Amazon Bedrock、Google Cloud 的 Agent Platform | Opus 5.5 | Sonnet 4.5 | Haiku 4.5 |
| Microsoft Foundry | Opus 4.6 | Sonnet 4.5 | Haiku 4.5 |

除非您设置 `ANTHROPIC_DEFAULT_FABLE_MODEL`，否则 `fable` 别名解析到 Fable 5.1，但在 [Claude apps gateway](https://code.claude.com/docs/zh-CN/claude-apps-gateway) 会话中除外，其中 `fable` 和 `best` 解析到 Fable 5。

未配置为提供 `claude-fable-5-1` 的网关会拒绝对该模型的请求。要通过提供它的网关使用 Fable 5.1，请使用 `/model claude-fable-5-1` 选择它。

当 `opus` 或 `sonnet` 解析到较旧的模型时，可以通过显式选择完整模型名称或设置 `ANTHROPIC_DEFAULT_OPUS_MODEL` 或 `ANTHROPIC_DEFAULT_SONNET_MODEL` 来使用较新的模型。

较早的版本将这些别名解析到较旧的模型。有关每个别名更改的版本，请参阅[版本历史](#version-history)。

别名指向您的提供商的推荐版本，并随时间更新。要固定到特定版本，请使用完整模型名称，例如 `claude-opus-5-5`，或设置相应的环境变量，如 `ANTHROPIC_DEFAULT_OPUS_MODEL`。

> [!NOTE]
>
> Sonnet 5.5 需要 Claude Code v2.1.284 或更高版本，Opus 5.5 需要 v2.1.280 或更高版本。如果从较旧版本对其中某个模型的请求失败，请参阅 [Claude Code does not support this model](https://code.claude.com/docs/zh-CN/errors.md#claude-code-does-not-support-this-model)。使用 Haiku 5.5 时请使用 v2.1.293 或更高版本。运行 `claude update` 进行升级。

### 使用 Fable

[Claude Fable 5.1](https://platform.claude.com/docs/en/about-claude/models/overview) 和 Claude Fable 5 是 Claude Code 中最强大的模型，适合一次无法完成的大型任务。它们能够维持长时间的自主会话，在行动前进行调查，并比较小的模型更频繁地验证其工作。Fable 5.1 是较新的版本。

这两个 Fable 模型都不是任何套餐或提供商上的账户类型默认值。请显式选择一个：

* **Fable 5.1**：运行 `/model fable`，或使用 `claude --model fable` 启动。在 [Claude apps gateway](https://code.claude.com/docs/zh-CN/claude-apps-gateway) 会话中，别名解析到 Fable 5，请改为运行 `/model claude-fable-5-1`。
* **Fable 5**：按模型 ID 选择它。在 Anthropic API 上，运行 `/model claude-fable-5` 或使用 `claude --model claude-fable-5` 启动。在其他提供商上，使用您的提供商的 Fable 5 模型 ID，或使用 `ANTHROPIC_DEFAULT_FABLE_MODEL` [固定它](#pin-models-for-third-party-deployments)。

如果您直接连接到 Anthropic API，并且您的用户设置将 `claude-fable-5` 或 `claude-fable-5[1m]` 作为模型（例如因为您在 v2.1.257 之前在 `/model` 选择器中选择了 Fable），Claude Code 会在您首次运行 v2.1.257 或更高版本时将该保存的值更改为 `fable` 或 `fable[1m]` 别名。启动模型行会显示一次 `(auto-updated)`。项目、本地或托管设置中的 `claude-fable-5` 值保持原样。

被 Fable 模型的安全分类器标记的请求（最常见于网络安全和生物学领域）会触发[自动模型回退](#automatic-model-fallback)。

要充分利用 Fable：

* **描述结果，而不是步骤**：告诉它您想要的结果，让它规划路径。要让它持续朝着该结果工作，请[设置一个目标](https://code.claude.com/docs/zh-CN/goal.md)。
* **交给它模糊的问题**：根本原因调查、故障调试和架构决策正是额外调查和验证发挥作用的地方。
* **省去验证提醒**：它无需太多提示就会验证自己的工作，因此通常不必提醒它测试或检查。
* **交给它更大的任务**：把您通常会拆分成几部分的工作交给它。它能够维持长时间的会话而不失去思路。

> [!NOTE]
>
> Fable 5.1 需要 Claude Code v2.1.257 或更高版本。如果来自较旧版本的请求失败，请参阅 [Claude Code does not support this model](https://code.claude.com/docs/zh-CN/errors.md#claude-code-does-not-support-this-model)。运行 `claude update` 进行升级。有关零数据保留下的可用性，请参阅 [Model availability under ZDR](https://code.claude.com/docs/zh-CN/zero-data-retention.md#model-availability-under-zdr)。

在 Anthropic API 上，Fable 模型会出现在 `/model` 选择器中，除非 [`availableModels`](#restrict-model-selection) 或[组织模型限制](#organization-model-restrictions)将其排除。当您的组织完全无法使用 Fable 时，例如在[零数据保留](https://code.claude.com/docs/zh-CN/zero-data-retention.md#model-availability-under-zdr)下，该行在选择器中保持灰显，并附有说明原因的注释。

#### Fable 和使用额度

根据您的套餐和席位等级，Fable 的使用可能计入[使用额度](https://support.claude.com/en/articles/12429409-extra-usage-for-paid-claude-plans)，而不是从您套餐包含的限额中扣除。在这种情况下，`/model` 选择器会在 Fable 行上显示"Requires usage credits"。要管理使用额度，请参阅 [Add usage credits to your subscription](https://code.claude.com/docs/zh-CN/costs.md#add-usage-credits-to-your-subscription)。

在交互式会话中，Claude Code 可能会显示 Fable 使用额度同意提示。使用组织计费的企业套餐成员不会看到该提示。您可以选择使用使用额度继续使用 Fable，或切换到您的默认模型。您也可以关闭提示：

* 当您使用 `/model` 选择 Fable 模型时，将保持当前模型。
* 在会话中途，Claude Code 会在您的默认模型上继续该轮次。

在您选择使用使用额度继续使用 Fable 后，Claude Code 不会再显示该提示。

在连接了 [Remote Control](https://code.claude.com/docs/zh-CN/remote-control.md) 的会话、[后台会话](https://code.claude.com/docs/zh-CN/agent-view.md)或 [agent team](https://code.claude.com/docs/zh-CN/agent-teams.md) 队友的会话中，终端前可能没有人，因此 Claude Code 会将会话中途的同意提示保留到 [`dialogExpiry`](https://code.claude.com/docs/zh-CN/settings-reference#dialogexpiry) 截止时间，默认为五分钟。如果到截止时间仍无人回答，Claude Code 会结束该轮次而不发送请求，并在会话记录中添加通知，Remote Control 客户端也会显示该通知。您的模型选择保持不变，Claude Code 会在您发送下一条消息时再次请求同意。

提示等待期间您可以执行的操作取决于会话：

* 在连接了 Remote Control 的会话或队友的会话中，在终端按任意键可取消截止时间，Claude Code 会等待您的回答。
* 在后台会话中，请在截止时间前回答。
* 如果在有人在终端输入之前，您从远程客户端发送了新消息，Claude Code 会以相同的方式结束该轮次，您的新消息将开始下一轮。在有人在终端输入之后，Claude Code 会继续等待回答，并将您的新消息排在其后。

在由其他应用通过 [Agent SDK](https://code.claude.com/docs/zh-CN/agent-sdk/overview.md) 托管的会话中，是否显示提示由该应用决定。如果显示了提示，并且在相同的 [`dialogExpiry`](https://code.claude.com/docs/zh-CN/settings-reference#dialogexpiry) 截止时间前无人回答，Claude Code 会结束该轮次而不发送请求。

在使用 `-p` 标志的[非交互模式](https://code.claude.com/docs/zh-CN/headless.md)中，以及在不显示提示的 Agent SDK 应用中，Claude Code 永远不会请求同意。当其中的 Fable 请求会计入使用额度时，Claude Code 会直接计费而不询问。

### 设置您的模型

您可以通过多种方式配置模型，按优先级顺序列出：

1. **在会话期间**：使用 `/model <alias|name>` 立即切换，或运行不带参数的 `/model` 打开选择器。参阅 [when Claude Code asks you to confirm the switch](https://code.claude.com/docs/zh-CN/prompt-caching.md#switching-models)
2. **在启动时**：使用 `claude --model <alias|name>` 启动
3. **环境变量**：设置 `ANTHROPIC_MODEL=<alias|name>`
4. **设置**：使用 `model` 字段在设置文件中永久配置
5. **[新会话的默认值](#set-a-default-model-for-new-sessions)**：设置 `ANTHROPIC_DEFAULT_MODEL=<alias|name>`

`/model` 通过在您的用户设置中写入 `model` 字段，将您的选择保存为新会话的默认值。在选择器中：

* `Enter`：切换模型并保存为默认值
* `s`：仅为此会话切换模型，保持默认值不变。要使用其他按键，请重新绑定 [`modelPicker:thisSessionOnly`](https://code.claude.com/docs/zh-CN/keybindings#model-picker-actions)

直接输入 `/model <name>` 的行为与 `Enter` 相同。要仅为此会话切换，请使用 `/model` 打开选择器，然后在该模型的行上按 `s`。

在企业套餐上，当您使用 claude.ai 账户登录并使用 `/model` 保存默认值时，Claude Code 也会在该账户上记录该选择。这需要 Claude Code v2.1.280 或更高版本。

* 当您的管理员未设置[组织默认模型](#organization-default-model)时，[Default 选项](#default-model-setting)可以解析为记录的模型，此时选择器的 Default 行会显示该模型的名称。
* 如果[模型限制](#restrict-model-selection)排除了记录的模型，或该模型对您的账户不可用，并且您的管理员未设置组织默认模型，则 Default 选项会按未记录任何内容的情况解析。
* 如果您在 `/model` 中选择 Default 或 `opusplan`，记录的选择不会改变。

如果您使用 `/model` 切换模型，该切换也会影响[继承主对话模型的子代理](https://code.claude.com/docs/zh-CN/sub-agents.md#choose-a-model)，因为 Claude Code 会在 Claude 启动它们时根据会话正在使用的模型解析它们的模型。在 Claude 将研究或测试运行委派给其中某个子代理之前切换到 Opus，该工作也会在 Opus 上运行。要让自定义子代理保持使用较小的模型，请在其定义中设置 `model`。

如果您在使用 `-p` 标志的[非交互模式](https://code.claude.com/docs/zh-CN/headless.md)中通过 `/model` 设置模型，您的选择仅适用于当前会话，不会保存为默认值；该模式中的 `/model` 需要 Claude Code v2.1.205 或更高版本。项目和托管设置仍然优先，并在下次启动时重新应用。您的管理员配置为覆盖用户选择的[组织默认模型](#organization-default-model)也会在下次启动时重新应用。

在 v2.1.144 到 v2.1.152 中，`/model` 仅适用于当前会话，在选择器中按 `d` 可保存默认值。

`--model` 标志和 `ANTHROPIC_MODEL` 环境变量仅适用于使用它们启动的会话。要同时在不同的终端中运行不同的模型，请分别使用各自的 `--model` 标志启动，而不是使用 `/model` 切换。

当 Claude Code 直接或通过代理 Anthropic API 的 [LLM 网关](https://code.claude.com/docs/zh-CN/llm-gateway)与 Anthropic API 通信时，`/model` 选择器中会显示价格，每行的价格即该行所选模型的价格。在[第三方提供商](https://code.claude.com/docs/zh-CN/third-party-integrations.md)（如 Amazon Bedrock）和 [Claude apps gateway](https://code.claude.com/docs/zh-CN/claude-apps-gateway) 上，由您的提供商或网关决定您支付的费用，因此选择器行不显示价格。价格仅是显示标签；它不影响某行选择哪个模型，也不影响提供商的计费。在 v2.1.206 之前，[Claude Platform on AWS](https://code.claude.com/docs/zh-CN/claude-platform-on-aws.md) 和网关会话显示 Anthropic 标价，且某行可能显示与其所选模型不同的模型的价格。

使用 `claude --resume`、`--continue` 或 `/resume` 选择器启动的恢复会话会保持保存会话记录时所使用的模型。如果恢复的模型已被停用或被 [`availableModels`](#restrict-model-selection) 排除，会话会回退到正常的优先级顺序。在使用提供商特定部署 ID 而不是 Anthropic 模型 ID 的提供商上，如 Amazon Bedrock、Google Cloud 的 Agent Platform 和 Microsoft Foundry，完全不会恢复会话记录中的模型，会话会通过正常的优先级顺序解析其模型。

如果您的 `model` 设置为 `haiku`，在 Haiku 模型上保存的会话会在 `haiku` 当前解析到的模型上恢复。例如，当 `haiku` 解析到 Haiku 5.5 后，在 Haiku 4.5 上保存的会话会在 Haiku 5.5 上恢复。

您在新启动时通过 `--model` 或 `ANTHROPIC_MODEL` 选择的模型仍然优先于恢复的模型。从 v2.1.195 开始，[`ANTHROPIC_DEFAULT_OPUS_MODEL`](#environment-variables) 系列变量也是如此。[`ANTHROPIC_DEFAULT_MODEL`](#set-a-default-model-for-new-sessions) 在其章节所列条件下也可以优先。

当启动时的活动模型来自项目或托管设置而不是您自己的选择时，启动标题会显示是哪个设置文件设置了它。运行 `/model` 可覆盖；项目或托管设置会在下次启动时重新应用。在嵌入 Claude Code 并设置 [`CLAUDE_CODE_PROVIDER_MANAGED_BY_HOST`](https://code.claude.com/docs/zh-CN/env-vars) 的平台上，主机的模型配置优先于托管模型设置，而托管的 `availableModels` 允许列表仍然有效，除非主机提供自己的允许列表；[Exceptions to managed settings precedence](https://code.claude.com/docs/zh-CN/settings.md#exceptions-to-managed-settings-precedence) 说明主机会覆盖哪些键和变量。

如果您或您的组织配置了 [PreModelSwitch hook](https://code.claude.com/docs/zh-CN/hooks.md#premodelswitch)，它们会在请求的切换生效之前运行，并可以阻止切换或要求您确认。

当 Claude Code 无法判断您组织的[托管插件](https://code.claude.com/docs/zh-CN/settings-reference.md#enabledplugins)提供了哪些 PreModelSwitch hook 时（例如因为某个托管插件加载失败），它会拒绝切换而不是在未经检查的情况下应用，并在每次新尝试时再次检查。有关消息和恢复方法，请参阅 [Model switch was blocked by a PreModelSwitch hook](https://code.claude.com/docs/zh-CN/errors.md#model-switch-was-blocked-by-a-premodelswitch-hook)。

当您通过 [Agent SDK](https://code.claude.com/docs/zh-CN/agent-sdk/overview.md) 的 `setModel()` 方法、通过应用（如 [Desktop app](https://code.claude.com/docs/zh-CN/desktop.md)）或从通过 [Remote Control](https://code.claude.com/docs/zh-CN/remote-control.md) 连接的设备切换模型时，Claude Code 会在切换时检查该值：

* **Agent SDK 或应用**：使用 Claude Code v2.1.268 或更高版本时，除非 Claude Code 在本地接受该模型 ID（如对您的[自定义模型选项](#add-a-custom-model-option)所做的那样），否则它会在会话首次切换到该 ID 时向您的提供商确认。该确认在所有提供商上都会运行，您的提供商不提供的 ID 会在切换时被拒绝，而不是在您的下一个请求时失败。
* **Remote Control**：在 Anthropic API 上，Claude Code 在本地检查该值，不发送请求。

有关这些消息，请参阅 [Model is not a recognized model id](https://code.claude.com/docs/zh-CN/errors.md#model-is-not-a-recognized-model-id) 和 [Model not found](https://code.claude.com/docs/zh-CN/errors.md#model-not-found)。

如果您使用 `--model` 标志、`ANTHROPIC_MODEL` 环境变量或 `model` 设置来设置模型，Claude Code 不会预先检查，拼写错误的值会在第一个请求时产生 [There's an issue with the selected model](https://code.claude.com/docs/zh-CN/errors.md#theres-an-issue-with-the-selected-model)。

当请求的模型有计划的停用日期或会被自动重新映射到较新版本时，Claude Code 会显示一条指明所请求模型的警告。交互式会话将其显示为启动通知。从 v2.1.182 开始，在使用默认文本输出格式的[非交互模式](https://code.claude.com/docs/zh-CN/headless.md)中，相同的警告会写入 stderr。该检查也涵盖在[子代理 frontmatter](https://code.claude.com/docs/zh-CN/sub-agents.md) 中设置的 `model`。对于 `--output-format json` 和 `stream-json`，stderr 警告会被抑制；请改为从[结果消息](https://code.claude.com/docs/zh-CN/headless.md#get-structured-output)的 `modelUsage` 字段读取实际模型。

例如，在 Opus 上启动会话：

```bash theme={null}
claude --model opus
```

然后在会话内切换模型：

```text theme={null}
/model sonnet
```

示例设置文件：

```json theme={null}
{
    "permissions": {
        "allow": ["Bash(npm run lint)"]
    },
    "model": "opus"
}
```

#### 为新会话设置默认模型

设置 `ANTHROPIC_DEFAULT_MODEL=<alias|name>` 来选择会话默认启动的模型。需要 Claude Code v2.1.236 或更高版本。

仅当以下各项都未选择模型时，Claude Code 才会在该变量的模型上启动新会话：

* `--model` 标志
* `ANTHROPIC_MODEL`
* 任何设置文件中的 `model` 值，包括您使用 `/model` 保存的选择
* [组织默认模型](#organization-default-model)

您使用 `/model` 保存的选择在后续启动时也优先于该变量。如果改为设置 `ANTHROPIC_MODEL`，无论您使用 `/model` 保存了什么，Claude Code 都会在下次启动时回到该变量的模型。

除非应用了组织默认模型，Claude Code 也会将 Default 选项解析为该变量的模型。当 Default 选项解析为该变量的模型时，`/model` 选择器中的 Default 行会显示标签 Set by ANTHROPIC\_DEFAULT\_MODEL。

在以下情况下，Claude Code 会忽略该变量，Default 选项会按未设置该变量的情况解析：

* 您将其设置为 `default`、`inherit`、`opusplan` 或 `haiku`
* [`enforceAvailableModels`](#enforce-the-allowlist-for-the-default-model) 已启用
* 您组织的[模型限制](#restrict-model-selection)排除了该模型
* 该模型对您的账户不可用

当新会话会在该变量的模型上启动时，您使用 `claude --resume`、`--continue` 或 `/resume` 选择器恢复的会话也会在该模型上启动。Claude Code 不会恢复该会话的会话记录中保存的模型。在其他情况下，Claude Code 在您[恢复会话](#setting-your-model)时不使用该变量。

#### 新会话在与您所选不同的模型上启动

当您使用 `/model` 选择了模型，但下一个会话却在其他模型上启动时，常见原因如下：

* **您只为一个会话选择了它。** 在选择器中按 `s`、使用 `--model` 启动以及在非交互模式中运行 `/model` 都仅适用于当前会话，不会改变已保存的默认值。
* **优先级更高的项设置了模型。** 项目或托管设置中的 `model` 值、shell 中的 `ANTHROPIC_MODEL`，或管理员设置为覆盖用户选择的[组织默认值](#organization-default-model)，会在每次启动时再次生效。您的 `/model` 选择仍然已保存，只是优先级较低。当项目或托管设置设置了模型时，启动标题会指明该文件。
* **Claude Code 无法保存您的选择。** `/model` 会将 `model` 写入 `~/.claude/settings.json`。如果您无法写入该文件（例如因为另一个工具生成了该文件，或将其链接到只读副本），您所选的模型仅在当前会话中有效，下次启动时会读取旧值。请在生成该文件的工具中设置 `model`，或使该文件可写。参阅 [A change you made in Claude Code is lost in new sessions](https://code.claude.com/docs/zh-CN/settings.md#a-change-you-made-in-claude-code-is-lost-in-new-sessions)。
* **您恢复了一个会话。** 使用 `claude --resume` 或 `--continue` 恢复的会话通常会[保持其原先使用的模型](#setting-your-model)，而不是当前的默认值。

## 限制模型选择

管理员可以在[托管或策略设置](https://code.claude.com/docs/zh-CN/managed-settings.md)中使用 `availableModels` 来限制用户可以选择的模型。条目可以匹配模型系列（如 `sonnet`）、版本前缀（如 `claude-sonnet-4-5`）或完整模型 ID（如 `claude-sonnet-4-5-20250929`）。版本前缀也会匹配扩展它的后续模型 ID，因此 `claude-fable-5` 允许 Fable 5 和 Fable 5.1，而 `claude-fable-5-1` 仅允许 Fable 5.1。要阻止列表允许的模型，或使每个模型 ID 条目仅允许它命名的版本，请参阅[阻止特定模型或版本](#block-specific-models-or-versions)。

在嵌入 Claude Code 并设置 [`CLAUDE_CODE_PROVIDER_MANAGED_BY_HOST`](https://code.claude.com/docs/zh-CN/env-vars) 的平台上，主机的模型配置优先于托管模型设置，而托管的 `availableModels` 允许列表保持有效，除非主机提供自己的列表；[托管设置优先级的例外](https://code.claude.com/docs/zh-CN/settings.md#exceptions-to-managed-settings-precedence)说明了主机覆盖的键和变量。

设置 `availableModels` 时，允许列表适用于用户可以指定模型的所有地方：

* **主会话模型**：`/model`、`--model` 标志、`ANTHROPIC_MODEL` 环境变量、`model` 设置、[`ANTHROPIC_DEFAULT_MODEL`](#set-a-default-model-for-new-sessions) 和[恢复会话](#setting-your-model)时恢复的模型
* **别名解析**：环境变量 `ANTHROPIC_DEFAULT_OPUS_MODEL`、`ANTHROPIC_DEFAULT_SONNET_MODEL`、`ANTHROPIC_DEFAULT_HAIKU_MODEL` 和 `ANTHROPIC_DEFAULT_FABLE_MODEL` 无法将允许的别名重定向到列表外的模型
* **快速模式**：当 `/fast` 会隐式切换到列表外的 Opus 模型时，它会拒绝切换，并显示消息"不在您组织的允许模型中"
* **子代理和队友模型**：[子代理](https://code.claude.com/docs/zh-CN/sub-agents.md#choose-a-model) frontmatter 中的 `model` 字段、Agent 工具的 `model` 参数、[agent team](https://code.claude.com/docs/zh-CN/agent-teams.md#specify-teammates-and-models) 队友模型、`CLAUDE_CODE_SUBAGENT_MODEL`，以及在 v2.1.197 及更早版本中，`/agents` 向导中的模型选择器&#x20;
* **skill 和命令模型**：[skill 和命令](https://code.claude.com/docs/zh-CN/skills.md)中的 `model` frontmatter
* **顾问模型**：配置的 [`advisorModel`](https://code.claude.com/docs/zh-CN/advisor) 设置和 `--advisor` 标志
* **后台 Agent 模型**：在 [Dispatch 选择器](https://code.claude.com/docs/zh-CN/agent-view.md)中选择的模型

在 Anthropic API 和 [AWS 上的 Claude Platform](https://code.claude.com/docs/zh-CN/claude-platform-on-aws.md) 上，当允许列表允许该模型时，模型系列别名 `opus`、`sonnet`、`haiku` 或 `fable` 解析为其通常的模型。当允许列表阻止该模型时，Claude Code 替换允许列表允许的该系列的最新版本，并显示一条通知，命名请求的和替换的模型。例如，使用 `["sonnet", "claude-opus-4-6"]`，`/model opus` 和 `--model opus` 都选择 Claude Opus 4.6，这是允许的最新 Opus。在 v2.1.205 之前，最新发布版本在列表外的别名被拒绝或替换，就像任何其他被阻止的选择一样，即使列表允许较旧版本。

替换需要一个允许的版本来落地：当允许列表不允许别名系列的任何版本时，别名遵循下面的拒绝和替换行为，就像任何其他被阻止的值一样。

Claude Code 根据模型的设置位置处理任何其他被阻止的选择：

* **`/model`**：Claude Code 以错误拒绝切换
* **`--model` 标志、`ANTHROPIC_MODEL` 或 `model` 设置**：Claude Code 在启动时用警告替换该值，命名请求的和替换的模型，会话在默认模型上启动
* **[`ANTHROPIC_DEFAULT_MODEL`](#set-a-default-model-for-new-sessions)**：Claude Code 忽略该变量
* **子代理或队友覆盖**：Claude Code 在备用模型上运行子代理或队友，而不是使请求失败。有关子代理回退，请参阅[选择模型](https://code.claude.com/docs/zh-CN/sub-agents.md#choose-a-model)，有关队友回退，请参阅[指定队友和模型](https://code.claude.com/docs/zh-CN/agent-teams.md#specify-teammates-and-models)。

  在交互式会话中，当 Claude Code 通过此回退或上面的最新允许版本替换来替换子代理的模型时，它会警告您，命名请求的和替换的模型；它不报告队友的回退。

  在上面的最新允许版本替换操作的地方，被阻止的系列别名遵循它。在 v2.1.222 之前，别名在每个提供商上像任何其他被阻止的值一样回退
* **skill 或命令覆盖**：Claude Code 忽略覆盖，包括被阻止的系列别名，skill 或命令在会话模型上运行。[在子代理中运行](https://code.claude.com/docs/zh-CN/skills.md#run-skills-in-a-subagent)的 skill 或命令遵循上面的子代理行为
* **`advisorModel` 设置**：顾问对会话禁用
* **`--advisor` 标志**：Claude Code 在启动时以错误退出。在[后台会话](https://code.claude.com/docs/zh-CN/agent-view.md)中，它改为在没有顾问的情况下启动会话，而不是退出
* **`CLAUDE_CODE_AUTO_MODE_MODEL`**：Claude Code 忽略该变量，并像您未设置它一样为其[自动模式分类器请求](https://code.claude.com/docs/zh-CN/permission-modes.md#default-classifier-model)选择模型

Claude Code 从 `/model` 选择器中隐藏排除的模型。您列出的模型 ID 是否也会获得自己的一行，因提供商而异：

* **Anthropic API、[AWS 上的 Claude Platform](https://code.claude.com/docs/zh-CN/claude-platform-on-aws.md)、[Claude apps 网关](https://code.claude.com/docs/zh-CN/claude-apps-gateway)或通过 `ANTHROPIC_BASE_URL` 设置的 [LLM 网关](https://code.claude.com/docs/zh-CN/llm-gateway)**：您列出的没有内置选择器行的 Anthropic 模型 ID 显示为其自己的标记行。Claude Code 会为 Opus、Sonnet 和 Haiku 版本添加这样的行，例如列表固定的较旧版本。如果您在 [`modelPicker`](https://code.claude.com/docs/zh-CN/settings-reference#modelpicker) 阵容中设置了 `replaceBuiltInOptions`，则该行不会显示。在 v2.1.199 之前，这样的 ID 只能通过键入 `/model <id>` 来选择。
* **Amazon Bedrock、Google Cloud 的 Agent Platform 或 Microsoft Foundry**：除非您列出的模型 ID 以 `anthropic.` 开头，否则 Claude Code 不会为其添加行，无论它是 Anthropic 模型 ID 还是提供商特定的 ID。[Mantle 模型 ID](#mantle-model-ids) 带有该前缀。要显示没有内置行的已列出版本，还需将其添加到 [`modelPicker`](https://code.claude.com/docs/zh-CN/settings-reference#modelpicker) 阵容中，该阵容接受您的提供商格式的 ID。

Claude Code 代表您进行的模型更改以相同的方式检查：

* **[备用模型链](#fallback-model-chains)**：允许列表外的条目被删除
* **计划模式升级**：在 Anthropic API 和 AWS 上的 Claude Platform 上，升级（如 [`opusplan`](#opusplan-model-setting)）到排除的模型使用升级系列允许的最新版本。在具有提供商特定模型 ID 的提供商上，以及当不允许任何版本时，升级被跳过，规划继续在会话的模型上进行
* **[自动模型回退](#automatic-model-fallback)**：目标被排除的回退不运行，因此标记的请求以拒绝结束
* **[自动模式分类器](https://code.claude.com/docs/zh-CN/permission-modes.md#eliminate-prompts-with-auto-mode)**：当允许列表排除[默认分类器模型](https://code.claude.com/docs/zh-CN/permission-modes.md#default-classifier-model)时，原本会使用该模型的分类器请求改为在会话模型上运行，或在 [Fable](#work-with-fable) 会话中在 Opus 模型上运行。在 Anthropic API 以外的提供商上，该 Opus 回退在您于 `ANTHROPIC_DEFAULT_OPUS_MODEL` 中设置的模型上运行，否则在 Opus 5 上运行，不咨询允许列表。需要 Claude Code v2.1.210 或更高版本
* **[快速模式](https://code.claude.com/docs/zh-CN/fast-mode.md)**：当会话之后运行的模型在允许列表外时，启用快速模式被拒绝
* **Amazon Bedrock 和 Google Cloud 的 Agent Platform 上的可用性回退**：当您的帐户在会话中途失去对某个模型的访问权限时，切换到其他模型时会跳过被排除的模型。[Amazon Bedrock](https://code.claude.com/docs/zh-CN/amazon-bedrock.md#when-your-organization-enforces-a-model-allowlist) 和 [Google Cloud 的 Agent Platform](https://code.claude.com/docs/zh-CN/google-vertex-ai.md#when-your-organization-enforces-a-model-allowlist) 上的启动模型检查仅在托管设置同时设置了 [`enforceAvailableModels`](#enforce-the-allowlist-for-the-default-model) 时才跳过被排除的模型

```json theme={null}
{
  "availableModels": ["sonnet", "haiku"]
}
```

### 使用入口覆盖范围

每个使用入口都强制执行它接收的允许列表。哪个交付机制到达每个使用入口不同：

| 交付机制 | CLI 和 IDE | 桌面本地会话 | Web、移动和云端会话 | Agent SDK 和非交互式 | Cowork |
| :- | :- | :- | :- | :- | :- |
| 来自管理控制台的[服务器管理设置](https://code.claude.com/docs/zh-CN/server-managed-settings.md) | 强制执行 | 强制执行 | 强制执行，除了[Claude Tag](https://claude.com/docs/claude-tag/overview) 会话 | 强制执行 | 远程 Cowork 会话：服务器检查模型。在用户的机器上：未交付。 |
| [MDM 或托管设置文件](https://code.claude.com/docs/zh-CN/managed-settings.md#delivery-mechanisms) | 强制执行 | 强制执行 | 在 Anthropic 托管环境中未交付；在[自托管环境](https://code.claude.com/docs/zh-CN/self-hosted-environments)中，根据[Claude Code 如何组合托管源](https://code.claude.com/docs/zh-CN/managed-settings.md#how-claude-code-combines-managed-sources)从运行器镜像强制执行 | 强制执行 | 在部署的地方强制执行 |

* [云端会话](https://code.claude.com/docs/zh-CN/claude-code-on-the-web.md)（包括您从桌面应用启动的会话）默认在 Anthropic 管理的 VM 上运行：部署到您的设备的设置不会到达它们，因此通过服务器管理设置交付允许列表。您的组织路由到[自托管环境](https://code.claude.com/docs/zh-CN/self-hosted-environments)的会话在您自己的计算上运行，也读取运行器镜像中的托管设置文件。[Claude Code 如何组合托管源](https://code.claude.com/docs/zh-CN/managed-settings.md#how-claude-code-combines-managed-sources)说明了该文件何时适用。云端会话中的中途模型切换在请求的模型被允许列表排除时被拒绝。当您的服务器管理设置中的 `availableModels` 列表非空时，服务器拒绝在 claude.ai/code 或从桌面应用以列表排除的模型启动云端会话的请求。
* [Claude Tag](https://claude.com/docs/claude-tag/overview) 会话在云环境中运行，但不接收服务器管理设置；在[自托管环境](https://code.claude.com/docs/zh-CN/self-hosted-environments)中，它们仍然读取运行器镜像中的托管设置文件。要为这些会话设置模型，请参阅 Claude Tag 管理员指南中的[为作用域选择模型](https://claude.com/docs/claude-tag/admins/customize#choose-the-model-for-a-scope)。
* Cowork（Claude 桌面应用中的 Agent 式工作选项卡）在 Claude Code 上运行其会话，但按设计不从 claude.ai 管理控制台接收服务器管理设置。当您的服务器管理设置中的 `availableModels` 列表非空且用户选择列表外的模型时，服务器拒绝该模型用于远程 Cowork 会话。当托管设置文件存在于会话运行的地方时，它适用于 Cowork 会话；远程 Cowork 会话在 Anthropic 管理的 VM 上运行，其中不存在设备部署的文件。
* [第三方提供商](https://code.claude.com/docs/zh-CN/server-managed-settings.md#platform-availability)（如 Amazon Bedrock、Google Cloud 的 Agent Platform、Microsoft Foundry 和 [AWS 上的 Claude Platform](https://code.claude.com/docs/zh-CN/claude-platform-on-aws.md)）上的会话不接收服务器管理设置，因此通过 MDM 或托管设置文件在那里交付允许列表。
* 来自管理控制台的交付还要求会话使用登录到您组织的[符合条件的登录](https://code.claude.com/docs/zh-CN/server-managed-settings.md#platform-availability)或为您组织签发的 OAuth 令牌来获取设置。对于使用 API 密钥进行身份验证的设备群（无论是直接配置的，还是由 [`apiKeyHelper`](https://code.claude.com/docs/zh-CN/settings-reference#apikeyhelper) 脚本生成的），请通过 MDM 或托管设置文件交付允许列表。
* 桌面代码选项卡还托管 [SSH 会话](https://code.claude.com/docs/zh-CN/desktop.md#ssh-sessions)，它们从运行的远程主机读取托管设置文件。请参阅[桌面托管设置](https://code.claude.com/docs/zh-CN/desktop.md#managed-settings)。
* claude.ai 和桌面应用中的模型选择器隐藏或灰显您的组织的允许列表排除的模型。选择器状态是用户的便利；它不强制执行允许列表。

### 默认模型行为

使用默认前缀匹配，`availableModels` 本身将默认选项留在帐户的系统[运行时默认](#default-model-setting)上，直到您也设置 [`enforceAvailableModels`](#enforce-the-allowlist-for-the-default-model)。如果该默认值是您打算限制的模型，也设置 `enforceAvailableModels`，或[阻止该模型](#block-specific-models-or-versions)。

使用 `availableModels: []`，命名的模型选择被阻止，`enforceAvailableModels` 无效。

### 为默认模型强制执行允许列表

在托管设置中将 `enforceAvailableModels: true` 与非空 `availableModels` 一起设置，以将允许列表扩展到默认选项。这需要 Claude Code v2.1.175 或更高版本。

```json theme={null}
{
  "availableModels": ["sonnet", "haiku"],
  "enforceAvailableModels": true
}
```

对于在其帐户上没有模型[记录](#setting-your-model)的成员，默认选项解析为帐户类型默认值，或当管理员设置了一个时解析为[组织默认模型](#organization-default-model)。当该模型不在允许列表中时，默认选项改为解析为命名允许的、可用模型的第一个 `availableModels` 条目，`/model` 选择器的默认行显示该模型。这适用于到达默认值的所有地方：会话启动、在 `/model` 中选择默认值、[备用模型链](#fallback-model-chains)中的 `"default"` 关键字，以及排除选择被删除时使用的备用模型。在成员帐户上记录的模型也针对 `availableModels` 进行检查；[设置您的模型](#setting-your-model)描述了默认选项如何处理它。

`enforceAvailableModels` 仅当 `availableModels` 非空时才重新映射默认选项。当 `availableModels` 非空但没有条目解析为允许的、可用的模型时，强制执行被跳过，并显示仅在 `--debug` 下可见的警告。在列表中保留至少一个保证可用的条目以避免这种情况。

在您交付的最高排名的托管源中一起部署两个键。默认情况下，Claude Code 仅读取该源，因此放在托管设置文件中的对在管理控制台交付任何设置时被忽略；在[Claude Code 如何组合托管源](https://code.claude.com/docs/zh-CN/managed-settings.md#how-claude-code-combines-managed-sources)中的选择加入合并下，Claude Code 仍然忽略来自排名低于设置 `availableModels` 的源的 `modelOverrides` 映射。

### 控制用户运行的模型

`model` 设置是初始选择，不是强制执行。它设置会话启动时哪个模型处于活动状态，但用户仍然可以打开 `/model` 并选择默认值，该值解析为系统的[运行时默认](#default-model-setting)，无论 `model` 设置为什么，除非 [`enforceAvailableModels`](#enforce-the-allowlist-for-the-default-model) 或[阻止特定版本的键](#block-specific-models-or-versions)适用于它。

要完全控制模型体验，请组合这些设置：

* **`availableModels`**：限制用户可以切换到的命名模型
* **`enforceAvailableModels`**：将 `availableModels` 允许列表扩展到默认选项，因此默认值无法解析到列表外的模型
* **`deniedModels`** 和 **`availableModelsMatch`**：[阻止特定版本](#block-specific-models-or-versions)，`availableModels` 条目会允许这些版本
* **`model`**：在会话启动时设置初始模型选择
* **`ANTHROPIC_DEFAULT_SONNET_MODEL`** / **`ANTHROPIC_DEFAULT_OPUS_MODEL`** / **`ANTHROPIC_DEFAULT_HAIKU_MODEL`** / **`ANTHROPIC_DEFAULT_FABLE_MODEL`**：控制 `sonnet`、`opus`、`haiku` 和 `fable` 别名解析为什么，以及[帐户类型默认](#default-model-setting)使用哪个版本

此示例在 Sonnet 4.5 上启动用户，将选择器限制为 Sonnet 和 Haiku，并确保默认值解析为允许列表上的模型，而不是层级默认值：

```json theme={null}
{
  "model": "claude-sonnet-4-5",
  "availableModels": ["claude-sonnet-4-5", "haiku"],
  "enforceAvailableModels": true,
  "env": {
    "ANTHROPIC_DEFAULT_SONNET_MODEL": "claude-sonnet-4-5"
  }
}
```

没有 `enforceAvailableModels` 或 `env` 块，在选择器中选择默认值的用户获得[运行时默认](#default-model-setting)，而不是 `model` 中固定的版本。这两个设置覆盖不同的范围：`enforceAvailableModels` 使默认值遵守允许列表，而 `env` 块固定允许的别名（如 `sonnet`）解析为哪个版本。当限制模型系列足够时单独使用 `enforceAvailableModels`；当您还需要固定特定版本时添加 `env` 块。

### 合并行为

当 Claude Code 应用的托管设置定义 `availableModels` 时，该列表单独适用，除了[提供自己的主机平台](https://code.claude.com/docs/zh-CN/settings.md#exceptions-to-managed-settings-precedence)：用户、项目或本地设置中的条目无法扩展它，Claude Code 也永远不会跨托管源合并 `availableModels`；[Claude Code 如何组合托管源](https://code.claude.com/docs/zh-CN/managed-settings.md#how-claude-code-combines-managed-sources)说明了哪个源的列表适用。否则，来自用户、项目和本地设置的列表像其他数组设置一样[连接和去重](https://code.claude.com/docs/zh-CN/settings.md#settings-precedence)。在 Claude Code v2.1.175 之前，来自较低优先级作用域的条目合并到托管列表中，而不是被它替换。

在有效列表中，命名系列中特定模型的条目（无论是版本前缀还是完整模型 ID）禁用该系列的通配符条目。例如，使用 `["sonnet", "claude-sonnet-4-5"]` 时，用户只能选择 `claude-sonnet-4-5` [匹配](#restrict-model-selection)的模型 ID，而不是每个 Sonnet 模型。

### Mantle 模型 ID

`availableModels` 中以 `anthropic.` 开头的条目会作为自定义选项添加到 `/model` 选择器中。这是[为第三方部署固定模型](#pin-models-for-third-party-deployments)中描述的别名匹配的例外。启用 [Amazon Bedrock Mantle 端点](https://code.claude.com/docs/zh-CN/amazon-bedrock.md#use-the-mantle-endpoint)后，Claude Code 会将符合 Mantle 格式的条目路由到该端点。该设置仍然将选择器限制为列出的条目，Mantle ID 嵌入系列名称，因此它计为特定条目并禁用该系列的通配符：在任何 Mantle ID 旁边，列出您想保持可选择的版本前缀或完整 ID。请参阅[合并行为](#merge-behavior)。

### 阻止特定模型或版本

`availableModels` 条目（如 `claude-opus-5`）也允许扩展它的后续版本（如 Opus 5.5），一旦 Claude Code 支持它们。两个托管设置让您保留一个版本，两者都需要 Claude Code v2.1.283 或更高版本：

* [`deniedModels`](https://code.claude.com/docs/zh-CN/settings-reference#deniedmodels)：列出要阻止的模型。即使 `availableModels` 允许，列出的模型也被阻止，该键也适用于根本没有允许列表的情况。没有条目阻止的版本保持允许
* [`availableModelsMatch`](https://code.claude.com/docs/zh-CN/settings-reference#availablemodelsmatch)：将其设置为 `"exact"`，以便 `availableModels` 中的每个模型 ID 仅允许它命名的版本。列出的模型 ID 的较新版本然后保持被阻止，直到您将其添加到列表中

较早的版本忽略两个键，因此也设置 [`requiredMinimumVersion`](https://code.claude.com/docs/zh-CN/settings-reference#requiredminimumversion) 以防止这些版本启动。

此示例允许 Opus 和 Sonnet 模型，并在每种拼写中阻止 Opus 5.5，包括日期和提供商特定的 ID：

```json theme={null}
{
  "availableModels": ["opus", "sonnet"],
  "deniedModels": ["claude-opus-5-5"]
}
```

被阻止的模型（无论 `deniedModels` 是否命名它或 `"exact"` 列表是否省略它）在[允许列表适用](#restrict-model-selection)的所有地方被视为被阻止的选择。它从 `/model` 选择器中隐藏，`/model <name>` 拒绝它。如果您用 `--model`、`ANTHROPIC_MODEL` 或 `model` 设置命名被阻止的模型 ID，Claude Code 在启动时删除它并解析默认选项。如果 [hook](https://code.claude.com/docs/zh-CN/hooks.md) 或后台请求命名 `deniedModels` 阻止的模型（如 Agent hook 的 `model` 字段），该请求在会话的模型上运行。

默认选项遵循两个键，无论您是否设置 [`enforceAvailableModels`](#enforce-the-allowlist-for-the-default-model)。如果您使用非空 `availableModels` 设置它，被阻止的默认值计为允许列表外的模型。否则，会解析为被阻止模型的默认选项按此顺序下降：

1. 同一系列允许的最新版本
2. 每个较低成本系列允许的最新模型：Sonnet，然后 Haiku
3. 命名允许模型的第一个 `availableModels` 条目

如果这些都不允许，在默认选项上启动的会话[拒绝启动](https://code.claude.com/docs/zh-CN/errors.md#managed-settings-block-the-default-model)，并显示命名要修复的键的错误。`"exact"` 列表仅当托管 `availableModels` 列表至少命名一个模型或系列时才影响默认选项。

Claude Code 仅从托管设置读取两个键。如果您在用户、项目或本地设置中或使用 `--settings` 设置其中任何一个，Claude Code 会忽略它并显示警告。

### 组织模型限制

Claude Enterprise 计划上的组织管理员通过在 claude.ai 管理控制台中禁用单个模型来限制成员可以运行的模型。此限制在 Claude Code 进行身份验证时与帐户的权利一起交付，与设置中的任何 `availableModels` 列表分开，服务器在创建会话时独立强制执行相同的限制。需要 Claude Code v2.1.187 或更高版本。

当成员登录或使用自己的 API 密钥时，限制适用。限定于组织的凭据（如组织服务密钥）不与用户绑定，因此限制不适用于它们。

Claude Console 没有模型限制控制。没有 Claude Enterprise 计划的组织（包括其成员通过 Anthropic API 进行身份验证的组织）使用[托管设置](https://code.claude.com/docs/zh-CN/managed-settings.md)中的 [`availableModels`](#restrict-model-selection) 限制模型，添加 [`enforceAvailableModels`](#enforce-the-allowlist-for-the-default-model) 以覆盖默认选项。[使用入口覆盖范围](#surface-coverage)说明了每个使用入口如何接收和强制执行这些设置。

受限模型从 `/model` 选择器中隐藏。用 `--model`、`ANTHROPIC_MODEL` 环境变量或 `model` 设置按名称选择它显示通知 `Model "<name>" is restricted by your organization's settings. Using <model> instead.`，会话在允许的模型上启动。为受限模型键入 `/model <name>` 被拒绝，显示 `Model '<name>' is restricted by your organization's settings. Run /model to choose a different model.`，会话保持其当前模型。

[模型系列别名](#restrict-model-selection)（如 `opus`）当组织允许它时解析为其通常的模型。当组织限制该模型时，Claude Code 替换组织允许的该系列的最新版本，显示相同的替换通知。`/model <alias>` 仅当其系列的每个版本都被限制时才被拒绝；用 `--model`、`ANTHROPIC_MODEL` 或 `model` 设置的别名在这种情况下仍在启动时被替换。在 v2.1.205 之前，系列别名基于其最新发布版本单独被替换或拒绝，即使允许较旧版本。

限制适用于组织范围或按角色：

* 在组织级别禁用模型会为每个成员删除它。
* 角色级别访问向不同的自定义角色授予不同的模型，持有多个角色的成员可以使用其任何角色授予的模型。
* Haiku 模型始终可用，无法禁用，因此每个成员至少保留一个可用模型。
* 访问更改在约一分钟内对新请求生效；`/model` 选择器在下次会话启动时反映它。

两个限制一起适用：仅当模型由 `availableModels` 允许且不受组织限制时，模型才可选择。组织限制仅到达 Anthropic API 和[LLM 网关](https://code.claude.com/docs/zh-CN/llm-gateway)部署上的会话；在任何其他提供商上，改为使用 `availableModels`。

## 组织默认模型

Claude Enterprise 计划上的组织管理员可以从 claude.ai 管理控制台为 Claude Code 成员设置默认模型，可以为整个组织设置，也可以按自定义角色设置。设置后，"默认"选项将解析为该模型。需要 Claude Code v2.1.196 或更高版本。

`/model` 选择器中的"默认"行显示组织默认值的名称，并带有"组织默认"标签。无论管理员是为整个组织设置默认值还是为您的角色设置默认值，标签都显示"组织默认"。角色默认值适用于该自定义角色的成员，优先于组织范围的默认值；当您的多个角色设置不同的默认值时，应用最强大的模型。

组织默认值是一个起点，而不是限制。这些选择优先于它：

* `--model` 标志和 `ANTHROPIC_MODEL` 环境变量
* [托管设置](https://code.claude.com/docs/zh-CN/managed-settings.md)中的 `model` 值或通过 `--settings` 提供的值
* 您的用户、项目或本地设置中的 `model` 值，包括您使用 `/model` 保存的模型

管理员还可以配置组织默认值以覆盖用户选择。启用覆盖后，它优先于用户、项目和本地设置中的 `model` 值，因此您使用 `/model` 保存的模型在当前会话中应用，组织默认值在下次启动时返回。当您的选择不同时，`/model` 显示 `Your organization's default (<model>) applies on restart`。即使启用了覆盖，`--model` 标志、`ANTHROPIC_MODEL`、托管设置和 `--settings` 仍然优先。

要限制成员可以选择的模型，请改用[组织模型限制](#organization-model-restrictions)或 [`availableModels`](#restrict-model-selection)。

Claude Code 在启动时读取组织默认值一次，因此管理员在会话中期更改的默认值在下次启动时生效。

当组织默认值不覆盖用户选择时，管理员更改后的第一次交互式启动会从您的用户设置中清除 `model` 键一次，以便应用新的默认值。它不会更改文件中的任何其他内容，您在该启动后使用 `/model` 保存的模型会被保留。

组织默认值在被采用之前会通过这些限制检查：

* 使用默认前缀匹配时，[`availableModels`](#restrict-model-selection) 本身不适用于组织默认值，因此允许列表外的组织默认值仍然适用。当同时设置了 [`enforceAvailableModels`](#enforce-the-allowlist-for-the-default-model) 时，允许列表外的组织默认值也会被重新映射到第一个允许列表条目
* [组织模型限制](#organization-model-restrictions)对您的账户拒绝的组织默认值会被替换为其系列中最新的允许模型，或当该系列的每个版本都被限制时被替换为成本较低的系列
* 对于 `deniedModels` 或 `"exact"` 列表阻止的组织默认值，请参阅[阻止特定模型或版本](#block-specific-models-or-versions)
* 您的账户完全无法使用的组织默认值会被跳过，"默认"选项的解析方式与[没有组织默认值](#default-model-setting)时相同

从 v2.1.199 开始，当组织默认值是与您的账户类型通常默认值不同的模型系列时，`/model` 选择器会为该通常系列保留单独的行，以便您仍然可以为会话切换到它。在 v2.1.196 到 v2.1.198 中，该行在选择器中缺失。

组织默认值仅适用于使用 Anthropic API 进行身份验证的会话。要在其他任何地方设置默认值，包括 [LLM 网关](https://code.claude.com/docs/zh-CN/llm-gateway)部署，请改用[托管设置](https://code.claude.com/docs/zh-CN/managed-settings.md)中的 `model` 键。

## 组织工作量限制

您的组织可以通过两种方式限制[工作量级别](#adjust-effort-level)。在 Claude Enterprise 计划中，组织管理员设置按角色的工作量限制，如下所述。在任何计划和任何提供商上，包括 Amazon Bedrock、Google Cloud 的 Agent Platform 和 Microsoft Foundry，[`maxEffortLevel`](https://code.claude.com/docs/zh-CN/settings-reference#maxeffortlevel) 托管设置在客户端上限制工作量。当两者都适用于某个模型时，较低的限制适用。

Claude Enterprise 计划上的组织管理员可以为每个自定义角色按模型设置最大 [effort 级别](#adjust-effort-level)，以及角色级别的[组织模型限制](#organization-model-restrictions)。需要 Claude Code v2.1.195 或更高版本。

* 超过限制的级别不会在 `/effort` 选择器中提供，使用 `--effort` 或 `/effort` 指定更高级别时会改为以限制级别运行。如果您使用高于限制的 `--effort` 启动纯文本 `--print` 运行，stderr 上的警告会指出请求的级别和实际应用的级别。在交互式会话中、使用 `json` 或 `stream-json` 输出时，或在后台 Agent 中，该警告会改为写入[调试日志](https://code.claude.com/docs/zh-CN/errors.md#configuration-warnings)。
* 限制是按模型的，因此切换模型可能会改变哪些级别可用。
* 当您的多个角色授予相同的模型时，最不严格的限制适用。

工作量限制与[组织模型限制](#organization-model-restrictions)一起交付，并到达相同的会话。

## 特殊模型行为

### `default` 模型设置

`default` 的行为取决于您的账户类型：

* **Pro、Max、Ultra、Team、Enterprise 和 Anthropic API**：默认为 Opus 5.5
* **Claude Platform on AWS、Amazon Bedrock 和 Google Cloud's Agent Platform**：默认为 Opus 5.5
* **Microsoft Foundry**：默认为 Sonnet 4.5

在 v2.1.280 之前，`default` 在 Pro 和 Team Standard 上解析为 Sonnet 5，在 Max、Ultra、Team Premium、Enterprise、Anthropic API、Claude Platform on AWS、Amazon Bedrock 和 Google Cloud's Agent Platform 上从 v2.1.219 开始解析为 Opus 5。在 v2.1.219 之前，`default` 在 Anthropic API、Max、Ultra、Team Premium 和 Enterprise 按量付费上从 v2.1.154 开始解析为 Opus 4.8，在 Claude Platform on AWS、Amazon Bedrock 和 Google Cloud's Agent Platform 上从 v2.1.207 开始解析为 Opus 4.8。在 v2.1.207 之前，`default` 在 Claude Platform on AWS 上解析为 Opus 4.7，在 Amazon Bedrock 和 Google Cloud's Agent Platform 上解析为 Sonnet 4.5。

当管理员设置了[组织默认模型](#organization-default-model)时，`default` 会解析为该模型，而不是上面的账户类型默认值。需要 Claude Code v2.1.196 或更高版本。`default` 也可以解析为您使用 [`ANTHROPIC_DEFAULT_MODEL`](#set-a-default-model-for-new-sessions) 设置的模型，具体条件见其部分说明，或解析为[记录在您账户上的](#setting-your-model)模型。

当您的账户上没有记录任何内容、托管设置[对默认模型强制执行允许列表](#enforce-the-allowlist-for-the-default-model)且账户类型默认值不在 `availableModels` 中时，`default` 会解析为强制执行的默认值，而不是上面的账户类型默认值。当组织默认值和强制执行都适用时，组织默认值首先替换账户类型默认值，然后强制执行应用于它：允许列表中的组织默认值被保留，而列表外的值解析为强制执行的默认值。

Fable 模型在任何套餐或提供商上都不是账户类型默认值。使用 `/model` 选择一个会将其保存为用户设置中的选定模型，以便后续会话从它开始。关于 Claude Code v2.1.257 对保存的 Fable 5 选择所做的一次性更改，请参阅[使用 Fable](#work-with-fable)。

### `opusplan` 模型设置

`opusplan` 模型别名提供了一种自动化混合方法：

* **在计划模式中**：使用 `opus` 进行复杂推理和架构决策
* **在执行模式中**：自动切换到 `sonnet` 进行代码生成和实现

这将 Opus 的推理能力与 Sonnet 的执行效率相结合。

计划模式 Opus 阶段使用与 `opus` 模型设置相同的上下文窗口，执行阶段使用与 `sonnet` 相同的窗口。当 `opus` 和 `sonnet` 解析为默认运行[1M 上下文窗口](#extended-context)的模型时，如当前模型在 Anthropic API 上所做的那样，两个阶段都使用它运行。要在它们不这样做的地方为两个阶段请求 1M 上下文，[设置模型](#setting-your-model)为 `opusplan[1m]`，例如使用 `/model opusplan[1m]`。使用 `/model` 设置它需要 Claude Code v2.1.265 或更高版本；在早期版本上，使用 `--model` 标志或 `model` 设置。

当 [`availableModels`](#restrict-model-selection) 排除最新的 Opus 但允许较旧版本时，例如 `["sonnet", "claude-opus-4-6"]`，`opusplan` 使用最新的允许的 Opus 进行规划，仅当每个 Opus 都被排除时才保持在 Sonnet 上。在计划模式中通常会升级到 Sonnet 的 Haiku 会话同样使用最新的允许的 Sonnet，仅当每个 Sonnet 都被排除时才保持在 Haiku 上。在 v2.1.205 之前，当升级系列的最新版本被排除时，计划模式会保持在会话的模型上，即使允许列表允许较旧的版本。

较旧的允许版本的替换适用于 Anthropic API 和 [Claude Platform on AWS](https://code.claude.com/docs/zh-CN/claude-platform-on-aws.md)。在 Amazon Bedrock、Google Cloud's Agent Platform、Microsoft Foundry 和 Mantle 上，其部署使用提供商特定的模型 ID，当升级模型被排除时，计划模式会保持在会话的模型上。

关于 Claude 在任务中途决定何时咨询第二个模型而不是在计划边界处切换的混合方法，请参阅[顾问工具](https://code.claude.com/docs/zh-CN/advisor.md)。

### 备用模型链

当主模型过载、不可用或返回另一个不可重试的服务器错误时，Claude Code 可以切换到备用模型，而不是使请求失败。身份验证、计费、速率限制、请求大小和传输错误，以及[您组织的策略检查拒绝](https://code.claude.com/docs/zh-CN/errors.md#automatic-retries)，永远不会触发切换；这些遵循其正常的重试和错误处理。当 [Amazon Bedrock](https://code.claude.com/docs/zh-CN/amazon-bedrock.md#when-a-model-is-disabled-mid-session) 或 [Google Cloud's Agent Platform](https://code.claude.com/docs/zh-CN/google-vertex-ai.md#when-a-model-is-disabled-mid-session) 拒绝您的账户无法调用的模型时，它会切换，Claude Code 将其视为模型不可用而不是身份验证错误。

配置一个或多个备用模型，Claude Code 会按顺序尝试它们，在切换时显示通知。切换仅持续当前轮次，因此您的下一条消息会首先再次尝试主模型。Claude Code 在删除重复项后将链限制为三个模型，并忽略额外条目。

使用 `--fallback-model` 标志为一个会话设置链，该标志接受逗号分隔的列表：

```bash theme={null}
claude --fallback-model sonnet,haiku
```

要在会话间保持链，在[设置](https://code.claude.com/docs/zh-CN/settings.md)中将 `fallbackModel` 设置为数组：

```json theme={null}
{
  "fallbackModel": ["claude-sonnet-5", "claude-haiku-4-5"]
}
```

`--fallback-model` 标志优先于 `fallbackModel` 设置。每个条目接受模型名称或别名，`"default"` 扩展为默认模型。

Claude Code 在启动时不确认链，`/status` 也不显示它。切换发生时显示的通知是回退已配置的第一个可见迹象。

当请求失败转移时，Claude Code 会按顺序尝试每个条目，直到一个接受它。无法到达的条目，例如在设置中固定的已停用模型，会以相同方式失败转移到下一个。Claude Code 在该遍历开始前删除两种条目：

* **超出允许列表**：当 Claude Code 读取链时，会删除 [`availableModels`](#restrict-model-selection) 不允许的任何条目。
* **压缩期间上下文窗口较小**：链也涵盖[压缩](https://code.claude.com/docs/zh-CN/context-window.md#what-survives-compaction)，但 Claude Code 不会回退到上下文窗口小于主模型的模型，因为在那里进行摘要会首先切断部分对话。如果每个备用模型都较小，压缩会显示原始错误，您可以重试。

Claude Code 也将链应用于[子代理](https://code.claude.com/docs/zh-CN/sub-agents.md)。当子代理的请求失败转移时，Claude Code 会按顺序尝试您配置的备用模型，子代理继续在接受请求的模型上运行。您的会话模型保持不变。在 v2.1.247 之前，链涵盖的失败会结束子代理。

### 自动模型回退

本部分涵盖来自 Fable 模型、Opus 5.5、Sonnet 5.5 和 Opus 5 的基于内容的回退。关于模型过载或不可用时的基于可用性的回退，请参阅[备用模型链](#fallback-model-chains)。

Fable 模型、Opus 5.5、Sonnet 5.5 和 Opus 5 运行安全分类器，最常标记网络安全和生物学内容。对于这两个类别，备用模型取决于哪个模型拒绝：

* **Fable 5.1、Fable 5 和 Opus 5.5**：生物学标记的请求在 Opus 5 上重新运行，网络安全标记的请求在 Opus 4.8 上重新运行。
* **Sonnet 5.5**：网络安全标记的请求在 Sonnet 5 上重新运行。生物学标记的请求以拒绝结束，因为 Sonnet 5.5 没有生物学备用模型。
* **Opus 5**：网络安全标记的请求在 Opus 4.8 上重新运行。生物学标记的请求以拒绝结束，因为 Opus 5 运行自己的生物学分类器，没有备用模型。

在 Amazon Bedrock、Google Cloud's Agent Platform 和 Microsoft Foundry 上，Claude Code 通过您的部署的模型 ID 解析这些目标。请参阅[在 Bedrock、Agent Platform 和 Foundry 上启用回退](#enable-fallback-on-bedrock-agent-platform-and-foundry)。

当 Claude Code 将被标记的请求切换到其类别对应的备用模型时，它会在该模型上重新运行请求。在您的主对话中，它会在会话记录中显示通知。如果希望先被询问，请参阅[切换前询问](#ask-before-switching)。

回退后，会话继续在备用模型上。要返回到您的原始模型，运行 [`/model`](#setting-your-model)。

基于类别的回退需要 Claude Code v2.1.219 或更高版本。在 v2.1.219 之前，每个标记的 Fable 5 请求都在您提供商的默认 Opus 模型上重新运行，Opus 5 不是回退源。

备用模型针对 [`availableModels`](#restrict-model-selection) 进行检查。当它被阻止时，不会发生回退。拒绝显示为正常错误，会话的模型保持不变。

#### 回退后的 effort 级别

当 Claude Code 将您的会话切换到备用模型时，它会保留被标记请求运行时的 effort 级别。例如，在 Opus 5.5 上以其默认 `medium` 运行的会话回退到 Opus 4.8 后仍保持 `medium`，尽管 Opus 4.8 默认为 `high`。

在以下情况下会应用不同的级别：

* **您自己的更改**：一旦您选择了 effort 级别、在 `/model` 中选择了模型或稍后恢复会话，被标记请求的级别就不再沿用。
* **为单个轮次设置的 effort**：如果您使用 `ultrathink` 提升了被标记的轮次，或 skill 的 `effort` frontmatter 设置了其级别，该级别适用于该轮次。后续轮次以 [effort 解析顺序](#adjust-effort-level)为备用模型给出的级别运行。

要查看回退后为您的会话设置的级别，请运行 `/effort status`。要更改它，请运行 `/effort`。

#### 检查触发回退的原因

回退可以在会话的第一个请求上触发，在您发送任何不寻常的内容之前，因为第一个请求携带工作区上下文，例如您的 CLAUDE.md 内容和 git 状态。包含安全或生物学材料的仓库可以仅在该上下文上触发分类器。

要检查自定义是否是触发器，使用 `claude --safe-mode` 启动会话，这会禁用自定义，例如 CLAUDE.md、skill、MCP 服务器和 hook。Git 状态和目录名称不是自定义，仍然包括在内。

#### 切换前询问

要决定每次请求被标记时发生什么，运行 `/config`，选择 **Switch models when a message is flagged**，然后选择 **Ask each time**。您也可以在设置文件中将 [`switchModelsOnFlag`](https://code.claude.com/docs/zh-CN/settings-reference#switchmodelsonflag) 设置为 `false`。之后，Claude Code 会在将要切换模型的标记请求处暂停，并为您提供两个选项：切换到备用模型，或编辑提示词并重试。

在交互式会话中，第一次有标记的请求将要切换模型时，Claude Code 可能会询问今后是否自动切换。仅当您尚未设置 `switchModelsOnFlag` 时它才会询问，并将您的选择作为该键保存到您的用户设置中。

如果您选择保留在当前模型上，保存的值为 `false`，与 **Ask each time** 相同。如果您关闭该询问，Claude Code 不保存任何内容，并在下次有标记的请求将要切换模型时再次询问。

当您选择了 **Ask each time** 时，某些情况的行为不同：

* 当标记的类别没有备用模型时，例如 Opus 5 或 Sonnet 5.5 上的生物学标记，Claude Code 不显示提示，请求以拒绝结束。
* 如果两个模型都标记相同的请求，您可以编辑提示词并重试，或启动新会话。
* 在移动应用上的[云端会话](https://code.claude.com/docs/zh-CN/claude-code-on-the-web.md)中，不支持编辑和重试。切换模型，或从桌面浏览器或桌面应用继续会话。
* 在[非交互模式](https://code.claude.com/docs/zh-CN/cli-reference.md#cli-flags)和无法显示提示的 SDK 集成中，标记的请求以拒绝结束轮次。
* 在[子代理](https://code.claude.com/docs/zh-CN/sub-agents.md)中，Claude Code 不显示提示，将要切换模型的标记请求会在备用模型上重新运行。
* 当回退目标被 [`availableModels`](#restrict-model-selection) 阻止时，Claude Code 不显示提示。标记的请求以拒绝结束，与目标被阻止时的自动回退相同。

#### 在 Bedrock、Agent Platform 和 Foundry 上启用回退

在 [Amazon Bedrock](https://code.claude.com/docs/zh-CN/amazon-bedrock.md)、[Google Cloud's Agent Platform](https://code.claude.com/docs/zh-CN/google-vertex-ai.md) 和 [Microsoft Foundry](https://code.claude.com/docs/zh-CN/microsoft-foundry.md) 上，模型 ID 是提供商特定的，因此自动回退仅在 Claude Code 可以识别每个涉及的模型时运行：

* Claude Code 必须将当前模型识别为回退源。当模型 ID 包含 `claude-fable-5`、匹配 `ANTHROPIC_DEFAULT_FABLE_MODEL` 的值或使用 [`modelOverrides`](#override-model-ids-per-version) 映射时，Fable 5.1 和 Fable 5 被识别。Opus 5.5、Sonnet 5.5 和 Opus 5 通过其提供商模型 ID 或 [`modelOverrides`](#override-model-ids-per-version) 映射被识别。
* 一个 Opus 目标必须在您的部署中解析，无论哪个模型拒绝：设置 `ANTHROPIC_DEFAULT_OPUS_MODEL`，或在提供商的模型列表中保留一个 Opus 4.8 条目。没有一个，回退对每个源模型都保持关闭，包括 Sonnet 5.5，标记的请求以拒绝结束。
* 标记的类别的备用模型必须在您的部署中解析。从 Fable 模型、Opus 5.5 或 Opus 5，如果您设置了 `ANTHROPIC_DEFAULT_OPUS_MODEL`，标记的请求会在该模型上为每个具有回退的类别重新运行；Opus 5 上的生物学标记仍以拒绝结束。如果您没有设置它，网络安全标记的请求会在 Opus 4.8 条目上重新运行，来自 Fable 模型或 Opus 5.5 的生物学标记请求会在 Opus 5 条目上重新运行。从 Sonnet 5.5，网络安全标记的请求会在您在 `ANTHROPIC_DEFAULT_SONNET_MODEL` 中设置的模型上重新运行，或在提供商模型列表中的 Sonnet 5 条目上（如果您没有设置它）。

如果任一模型无法识别，Claude Code 不会切换。标记的请求以拒绝消息结束，您可以使用 [`/model`](#setting-your-model) 切换模型并重试。要使两个模型都可识别，为您的源模型设置固定值：

* **Fable 模型**：将 `ANTHROPIC_DEFAULT_FABLE_MODEL` 设置为您的 Fable 模型 ID，以便 Claude Code 将其识别为回退源。
* **每个源模型**：将 `ANTHROPIC_DEFAULT_OPUS_MODEL` 设置为 Opus 模型 ID 以打开回退并为标记的类别提供目标。命名 Opus 系列外的模型或拒绝的模型的固定值会使拒绝成立。
* **Sonnet 5.5**：除了 Opus 固定值外，设置 `ANTHROPIC_DEFAULT_SONNET_MODEL` 或在提供商的模型列表中保留 Sonnet 5 条目以提供请求重新运行的模型。命名 Sonnet 系列外的模型或 Sonnet 5.5 本身的 Sonnet 固定值会使拒绝成立。

备用模型的上下文窗口还必须至少与会话的上下文窗口一样大，否则 Claude Code 不会切换，标记的请求会以相同的拒绝结束。在这些提供商上，源模型默认运行 [1M 上下文窗口](#extended-context)。请固定一个同样如此的模型，例如在 `ANTHROPIC_DEFAULT_OPUS_MODEL` 中固定 Opus 4.8，或在 `ANTHROPIC_DEFAULT_SONNET_MODEL` 中固定 Sonnet 5，并使用 Claude Code [能够匹配到该模型](#pin-models-for-third-party-deployments)的 ID。

#### 安全研究和生物学工作负载

进攻性安全或生物学中的工作负载，包括渗透测试、Capture the Flag (CTF) 练习和生物学相邻代码库，经常触发回退，通常在第一个请求上。对于 Fable 5.1、Fable 5 或 Opus 5.5 上的实质性生物学工作，第一个切换模型的标记请求会将会话移动到 Opus 5，后来的生物学标记请求在那里以拒绝结束，因为 Opus 5 没有生物学回退。在 Opus 5 和 Sonnet 5.5 上，您从第一个标记的请求获得这些拒绝。

这是这些域的预期路由，不是账户标记。如果您的组织需要 Fable 级别的能力来完成这项工作，请向您的 Anthropic 账户团队询问受信任的访问计划。

### 调整 effort 级别

[Effort 级别](https://platform.claude.com/docs/en/build-with-claude/effort)控制自适应推理，让模型根据任务复杂性决定是否以及在每一步上思考多少。较低的 effort 对于直接的任务更快且更便宜，而较高的 effort 为复杂问题提供更深入的推理。

可用的 effort 级别取决于模型。此处未列出的模型不支持 effort：

| 模型 | 级别 |
| :- | :- |
| Fable 5.1 和 Fable 5 | `low`、`medium`、`high`、`xhigh`、`max` |
| Opus 5.5、Sonnet 5.5、Haiku 5.5、Opus 5、Sonnet 5、Opus 4.8 和 Opus 4.7 | `low`、`medium`、`high`、`xhigh`、`max` |
| Opus 4.6 和 Sonnet 4.6 | `low`、`medium`、`high`、`max` |

如果您设置活动模型不支持的级别，Claude Code 会回退到不高于您所设级别的最高支持级别。例如，`xhigh` 在 Opus 4.6 上运行为 `high`。您的组织或您自己的设置也可以限制模型提供的级别；请参阅[组织 effort 限制](#organization-effort-limits)。

Claude Code 按此顺序解析会话的 effort 级别，采用首先适用的：

1. [`CLAUDE_CODE_EFFORT_LEVEL`](https://code.claude.com/docs/zh-CN/env-vars#variables) 环境变量
2. 您在会话中选择的级别，例如通过 `/effort`、`/model` 选择器或 [`applyFlagSettings()`](https://code.claude.com/docs/zh-CN/agent-sdk/typescript#applyflagsettings)
3. 使用 `--effort` 启动
4. 您的设置：您为模型保存的级别或 [`effortLevel`](https://code.claude.com/docs/zh-CN/settings-reference#effortlevel) 键，在 [`modelSettings`](https://code.claude.com/docs/zh-CN/settings-reference#modelsettings) 中说明它们之间和跨设置文件的优先级
5. 模型的默认 effort：在支持 effort 的每个模型上为 `high`，除了 Opus 5.5、Sonnet 5.5 和 Haiku 5.5 默认为 `medium`、Opus 4.7 默认为 `xhigh`。如果您的组织为其[组织默认模型](#organization-default-model)设置了默认 effort 级别，则当您运行该模型时，该级别即为默认值。

自动模型回退后适用的级别，请参阅[回退后的 effort 级别](#effort-level-after-a-fallback)。

Opus 5.5 从 `medium` 开始，除非上面的源之一为其设置级别，您的用户设置文件中的顶级 `effortLevel` 不计入 Opus 5.5。该键是较旧的形式 `/effort` 在 Claude Code 按模型保存级别之前写入的：它继续在它之前应用的地方应用，在 Opus 5、Fable 5.1 和更早的模型上，而 Opus 5.5 和在它之后发布的模型从它们自己的默认开始，直到您使用 `/effort` 或 `/model` 选择器为它们选择级别。项目、本地或托管设置中的顶级 `effortLevel`，或使用 `--settings` 传递的，适用于每个模型。

当您在机器上的交互式会话中设置 `low`、`medium`、`high` 或 `xhigh` 时，您通过如何确认它来选择它持续多长时间：

* 在 `/effort` 滑块或 `/model` 选择器中使用 `Enter`，或在 `/effort` 后键入的级别：将级别保存为您的默认值并在后续会话中应用它
* 在 `/effort` 滑块或 `/model` 选择器中的 `s`：仅将级别应用于此会话。需要 Claude Code v2.1.257 或更高版本

Claude Code 在用户设置中的 [`modelSettings`](https://code.claude.com/docs/zh-CN/settings-reference#modelsettings) 键下按模型保存级别，因此每个模型保持其自己的保存级别。

`max` 是最深的推理级别。除非您通过 `CLAUDE_CODE_EFFORT_LEVEL` 环境变量设置它，Claude Code 仅将 `max` 应用于当前会话。

> [!NOTE]
>
> 您从通过 [Remote Control](https://code.claude.com/docs/zh-CN/remote-control.md#what-connected-devices-see) 连接的手机或浏览器上的 effort 控件中选择的级别仅适用于该会话。

当您在 [`-p` 运行](https://code.claude.com/docs/zh-CN/headless)中使用 `/effort` 设置级别时，Claude Code 仅将其应用于该会话，不将其保存为您的默认值。

`/effort` 滑块也有一个 **Ultracode** 切换。Ultracode 是 Claude Code 设置而不是模型 effort 级别：启用它时，Claude 为实质性任务编排[动态工作流](https://code.claude.com/docs/zh-CN/workflows.md)，在会话运行的任何 effort 级别。关于它可以在哪里持久设置，请参阅 [`ultracode`](https://code.claude.com/docs/zh-CN/settings-reference#ultracode) 设置。

使用 `/effort` 或 `ultracode` 设置打开或关闭 ultracode 会使 effort 级别保持不变。`--effort ultracode` 标志和 Agent SDK `effortLevel: "ultracode"` 值打开它，也将级别设置为 `xhigh`。在 `/effort` 滑块或 `/model` 选择器中选择级别会使 ultracode 保持原样。

您可以通过以下任何方式打开 ultracode：

* **`/effort`**：运行 `/effort ultracode` 为当前会话打开它或 `/effort ultracode off` 关闭它。在 `/effort` 滑块中，按 `Tab` 翻转 **Ultracode** 切换，然后 `Enter` 应用它
* **`--effort` 标志**：使用 `claude --effort ultracode` 启动，这会在 `xhigh` effort 和 ultracode 打开的情况下启动会话
* **`ultracode` 设置**：在设置文件中、使用 `--settings` 或在 Agent SDK 控制请求中设置 [`"ultracode": true`](https://code.claude.com/docs/zh-CN/settings-reference#ultracode)。[`applyFlagSettings()`](https://code.claude.com/docs/zh-CN/agent-sdk/typescript#applyflagsettings) 请求也接受 `effortLevel: "ultracode"`，它打开它并将 effort 级别设置为 `xhigh`

`/effort ultracode off` 形式、滑块切换和在 `xhigh` 以外的 effort 级别保持 ultracode 打开需要 Claude Code v2.1.284 或更高版本。在 v2.1.284 之前，打开 ultracode 将会话设置为 `xhigh` effort，选择另一个级别关闭它，effort 上限低于 `xhigh` 使其不可用。

将 `ultracode` 传递给 `--effort` 标志或 Agent SDK `effortLevel` 值需要 Claude Code v2.1.203 或更高版本。在 v2.1.203 之前，`--effort ultracode` 打印 `Unknown --effort value 'ultracode'`，会话以默认 effort 开始。

持久化的 `effortLevel` 设置和 `CLAUDE_CODE_EFFORT_LEVEL` 环境变量不接受 `ultracode`。如果 `CLAUDE_CODE_EFFORT_LEVEL` 或 [effort 上限](#organization-effort-limits)设置会话的级别，ultracode 在该级别保持打开。

Ultracode 在以下情况下不可用：

* [工作流被关闭](https://code.claude.com/docs/zh-CN/workflows.md#turn-workflows-off)
* 模型不支持 `xhigh` effort

在这些情况下，`--effort ultracode` 启动会话时 ultracode 关闭，effort 级别为模型和任何上限允许的最高级别，最高为 `xhigh`。

#### 选择 effort 级别

每个级别在 token 支出和能力之间进行权衡。默认值适合大多数编码任务；当您想要不同的平衡时进行调整。

| 级别 | 何时使用 |
| :- | :- |
| `low` | 快速交换，您审查每个结果，例如头脑风暴、初稿或小改动如重命名 |
| `medium` | Opus 5.5、Sonnet 5.5 和 Haiku 5.5 上的默认值。在 Opus 5.5 和 Sonnet 5.5 上，它适合具有明确范围的日常工程工作，例如实现新功能。在默认值更高的模型上，减少成本敏感工作的 token 使用，可以权衡一些智能 |
| `high` | 验证重要或边界情况可能的工作，例如修复现有代码库中的错误。除 Opus 5.5、Sonnet 5.5、Haiku 5.5 和 Opus 4.7 外，每个模型上的默认值 |
| `xhigh` | 更高 token 支出的更深推理。Opus 4.7 上的默认值 |
| `max` | 您想让 Claude 自己完成的难题，例如发现安全漏洞。`max` 可能显示收益递减，容易过度思考，所以在广泛采用前测试 |
| `ultracode` | 一个 Claude Code 设置而不是级别：为每个实质性任务规划[动态工作流](https://code.claude.com/docs/zh-CN/workflows.md)，在任何 effort 级别 |

在 Opus 5.5 和 Fable 5.1 的测试中，Claude 在更高级别测试了更多边界情况，在回答前验证了更多工作。它也自己做了更多选择。在较低级别，Claude 更快地返回起点，适合您审查每个结果并指导下一步的工作。要查看在每个级别运行的相同任务，请阅读博客上的[Using Claude Code: Spending your effort](https://claude.dev/blog/spending-your-effort/)。

Effort 规模按模型校准，因此相同的级别名称在模型间不代表相同的基础值。

Opus 5.5 [默认为 `medium`](#adjust-effort-level)，比 Opus 5 的默认值 `high` 低一个级别。在 Anthropic 的测试中，Opus 5.5 在 `medium` 时在编码和知识工作评估上匹配或超过 Opus 5 在 `high` 时的表现。在给定的级别，Opus 5.5 倾向于每轮比 Opus 5 思考更多。当您从 Opus 5 移动到 Opus 5.5 时，从 `medium` 开始，而不是沿用您在 Opus 5 上使用的级别。要针对您自己的工作测试级别，请参阅 Opus 5.5 提示词指南中的[校准 effort](https://platform.claude.com/docs/en/build-with-claude/prompt-engineering/prompting-claude-opus-5-5#calibrate-effort)。

#### 使用 ultrathink 进行一次性深度推理

在您的提示词中的任何地方包含 `ultrathink` 以请求该轮次的更深推理，而不改变您的会话 effort 设置。Claude Code 识别关键字并添加上下文内指令。Claude Code 将其他短语如"think"、"think hard"和"think more"作为普通提示词文本传递，不将它们识别为关键字。

在终端输入框中键入时，该词还可以将该轮次提升到 `xhigh` effort。当它可以这样做时，该词会以彩虹色显示。需要 Claude Code v2.1.311 或更高版本。如果您开始新轮次时该词没有以彩虹色显示，请尝试以下方法之一：

* 在不以 `/` 开头的提示词中重新键入该词，并确保它不在引号、括号、路径或命令等任何内容之内
* 如果 `CLAUDE_CODE_EFFORT_LEVEL` 设置为低于 `xhigh` 的级别，请取消设置它。或者将您自己的 [`maxEffortLevel`](https://code.claude.com/docs/zh-CN/settings-reference#maxeffortlevel) 提高到 `xhigh` 或更高，或请组织管理员提高您的[组织限制](#organization-effort-limits)
* 在会话中运行 `/model` 切换到 Opus 5.5、Sonnet 5.5、Haiku 5.5 或 Fable 5.1 模型。在该词无法提升轮次的情况下（例如在 Amazon Bedrock 上），请改为[设置 effort 级别](#set-the-effort-level)

#### 设置 effort 级别

您可以通过以下任何方式更改 effort：

* **`/effort`**：运行 `/effort` 不带参数以打开交互式滑块，`/effort` 后跟级别名称以直接设置它，或 `/effort auto` 以清除活动模型的保存级别
* **在 `/model` 中**：选择模型时使用左/右箭头键调整 effort 滑块。新级别最迟从您的下一条消息开始生效。在此之前，Claude 可能继续以旧级别工作，即使是在新模型上。当[优先级高于您所选级别的来源](#adjust-effort-level)（例如 `CLAUDE_CODE_EFFORT_LEVEL`）适用时，Claude 改为以该来源的级别工作。
* **`--effort` 标志**：启动 Claude Code 时传递级别名称以为单个会话设置它
* **环境变量**：将 `CLAUDE_CODE_EFFORT_LEVEL` 设置为级别名称。要让 Claude Code [不发送 effort 级别](#send-no-effort-level)，请改为将其设置为 `auto` 或 `unset`
* **设置**：在 [`modelSettings`](https://code.claude.com/docs/zh-CN/settings-reference#modelsettings) 中设置每个模型的级别，或将 [`effortLevel`](https://code.claude.com/docs/zh-CN/settings-reference#effortlevel) 设置为 `low`、`medium`、`high` 或 `xhigh` 作为没有级别的模型的默认值。`max` 在任一键中都不被接受为级别，`ultracode` 有其自己的 [`ultracode`](https://code.claude.com/docs/zh-CN/settings-reference#ultracode) 键
* **从连接的设备**：在 [Remote Control](https://code.claude.com/docs/zh-CN/remote-control.md#what-connected-devices-see) 会话中，从您的手机或浏览器上的 effort 控件中选择级别。该级别仅适用于当前会话。需要 Claude Code v2.1.234 或更高版本
* **Skill 和子代理 frontmatter**：在 [skill](https://code.claude.com/docs/zh-CN/skills.md#frontmatter-reference) 或[子代理](https://code.claude.com/docs/zh-CN/sub-agents.md#supported-frontmatter-fields) markdown 文件中设置 `effort` 以在该 skill 或子代理运行时覆盖 effort 级别

skill 的 `effort` 在其运行所在的轮次结束前一直适用；使用 `context: fork` 时，则在分叉出的子代理运行期间适用。子代理的 `effort` 在子代理运行期间适用，除非[其调用覆盖了它](https://code.claude.com/docs/zh-CN/sub-agents.md#choose-an-effort-level)。当其中任一项适用时，skill 或子代理都以其级别运行，无论您启动时使用、保存或选择的是什么级别。如果设置了 `CLAUDE_CODE_EFFORT_LEVEL`，则改为应用该级别。

[`maxEffortLevel`](https://code.claude.com/docs/zh-CN/settings-reference#maxeffortlevel) 或[组织 effort 上限](#organization-effort-limits)仍然限制 skill 或子代理运行的级别。如果您在[托管设置](https://code.claude.com/docs/zh-CN/managed-settings.md)中设置 `effortLevel`，Claude Code 在 [effort 解析顺序](#adjust-effort-level)的设置步骤处应用它，用户仍然可以使用 `/effort` 或 `--effort` 更改级别。要将用户保持在或低于某个级别，设置 [`maxEffortLevel`](https://code.claude.com/docs/zh-CN/settings-reference#maxeffortlevel)。

#### 检查您设置的 effort 级别

选择支持的模型时，effort 滑块会出现在 `/model` 中。会话标题中显示的级别（例如"with low effort"）来自您的 effort 设置，即使 Claude [仍在以旧级别工作](#when-a-new-effort-level-starts)。在启动时以及您更改级别时，页脚也会简要显示为您的会话设置的级别。

#### 新 effort 级别何时生效

在终端会话中，一旦 `/effort` 确认了新级别，您发送的下一条消息就会以该级别运行。您可以在当前轮次结束后发送它，也可以[在 Claude 仍在工作时](https://code.claude.com/docs/zh-CN/interactive-mode.md#queue-messages-while-claude-works)发送：

* **轮次结束后**：在此之前您无需执行任何操作，如果该轮次以旧级别完成，这属于预期行为
* **Claude 仍在工作时**：输入一条消息（例如 `keep going`），然后按 `Enter` 将其加入队列，而不会中断当前轮次。Claude 会在其当前工具调用完成时（最迟在轮次结束时）收到该消息，并从那时起以新级别工作。

在 v2.1.242 之前，Claude Code 可能会将 `/effort` 本身排队直到轮次结束；在不[获取功能标志](https://code.claude.com/docs/zh-CN/env-vars.md#features-that-need-feature-flag-fetching)的会话中（例如在[第三方提供商](https://code.claude.com/docs/zh-CN/third-party-integrations.md)上）则始终如此。

#### 不发送 effort 级别

当 Claude Code 不向[支持的模型](#adjust-effort-level)发送 effort 级别时，将应用 [API 的默认值](https://platform.claude.com/docs/en/build-with-claude/effort)，而不是 Claude Code 的默认值。要让 Claude Code 不发送任何级别，请从导出了 `CLAUDE_CODE_EFFORT_LEVEL` 且其值为 `auto` 或 `unset` 的 shell 中启动 `claude`，这两个值的作用相同：

```bash theme={null}
export CLAUDE_CODE_EFFORT_LEVEL=auto
```

未发送任何级别时，会话标题和页脚不显示 effort 级别。要确认未发送任何级别，请在 Claude Code 提示符处运行 `/effort status`：

* **`Effort level: auto`**：未发送任何 effort 级别。
* **`Effort level: auto (currently <level>)`**：仍在发送级别。如果低于 `max` 的 [`maxEffortLevel`](https://code.claude.com/docs/zh-CN/settings-reference#maxeffortlevel) 设置适用于活动模型，Claude Code 会发送其自己为该模型设定的默认值，并降低到该上限。请移除该设置或将其设置为 `max`。

> [!NOTE]
>
> 在 v2.1.304 之前，即使未发送任何级别，`/effort status` 也会打印 `Effort level: auto (currently high)`。

#### 自适应推理和固定思考预算

自适应推理使思考在每一步上可选，因此 Claude 可以更快地响应常规提示词，并为受益于它的步骤保留更深入的思考。如果您想要 Claude 比当前级别产生的更频繁或更少地思考，您可以直接在您的提示词或 `CLAUDE.md` 中说出来；模型在其 effort 设置内响应该指导。

Fable 模型、Sonnet 5 及更高版本、Haiku 5.5 和 Opus 4.7 及更高版本始终使用自适应推理。固定思考预算模式和 `CLAUDE_CODE_DISABLE_ADAPTIVE_THINKING` 不适用于它们。

在 Opus 4.6 和 Sonnet 4.6 上，您可以设置 `CLAUDE_CODE_DISABLE_ADAPTIVE_THINKING=1` 以恢复到由 `MAX_THINKING_TOKENS` 控制的先前固定思考预算。请参阅[环境变量](https://code.claude.com/docs/zh-CN/env-vars.md)。

### 扩展思考

扩展思考是 Claude 在响应前发出的推理。在支持[自适应推理](#adjust-effort-level)的模型上，effort 级别是对发生多少思考的主要控制；下面的设置打开或关闭思考并控制它如何显示。在 Anthropic API 上关闭思考时，Claude Code 向它知道[不接受该组合](https://code.claude.com/docs/zh-CN/errors.md#effort-isnt-available-with-thinking-turned-off)的模型（如 Opus 5）发送 effort `high` 而不是更高级别。

| 控制 | 如何设置 |
| :- | :- |
| 当前会话的切换 | 在 macOS 上按 `Option+T` 或在 Windows 和 Linux 上按 `Alt+T` |
| 设置全局默认值 | 运行 `/config` 并切换思考模式。保存为 `~/.claude/settings.json` 中的 `alwaysThinkingEnabled` |
| 通过环境变量禁用 | 设置 [`MAX_THINKING_TOKENS=0`](https://code.claude.com/docs/zh-CN/env-vars)，这在 Anthropic API 上关闭思考，除了 Opus 5.5、Sonnet 5.5、Haiku 5.5 和 Fable 模型。在[第三方提供商](https://code.claude.com/docs/zh-CN/third-party-integrations.md)上，Claude Code 改为省略 `thinking` 参数，自适应推理模型可能仍然思考 |

您不能在 Opus 5.5、Sonnet 5.5、Haiku 5.5 或 Fable 模型上关闭思考。对于这些模型，会话切换和 `/config` 行显示 `Thinking can't be turned off`，而不是提供切换，保存的 `alwaysThinkingEnabled: false` 或 `MAX_THINKING_TOKENS=0` 在那里没有效果。在这些模型上，模型根据 effort 级别按步骤决定思考多少。保存的设置在您切换到接受它的模型时再次应用。

Claude Code 默认折叠思考输出。按 `Ctrl+O` 切换详细模式并将推理视为灰色斜体文本。Anthropic API 上的交互式会话默认接收编辑的思考块，因此如果您想要完整摘要在展开时可用，在[设置](https://code.claude.com/docs/zh-CN/settings.md)中设置 `showThinkingSummaries: true`。您需要为所有生成的思考 token 付费，即使折叠或编辑。

### 扩展上下文

Fable 5.1、Fable 5、Sonnet 5 及更高版本、Haiku 5.5、Opus 4.6 及更高版本和 Sonnet 4.6 支持[100 万 token 上下文窗口](https://platform.claude.com/docs/en/build-with-claude/context-windows#context-window-sizes-by-model)，用于具有大型代码库的长会话。

Fable 5.1、Fable 5、Sonnet 5 及更高版本、Haiku 5.5 和 Opus 4.7 及更高版本默认运行 1M 窗口，无需 `[1m]` 后缀。这包括 Amazon Bedrock、Google Cloud's Agent Platform 和 Microsoft Foundry 上的会话，以及 [Claude apps gateway](https://code.claude.com/docs/zh-CN/claude-apps-gateway) 会话。要改为以 200K 窗口运行它们，请参阅[关闭 1M 上下文](#turn-off-1m-context)。

Opus 4.6 和 Sonnet 4.6 仅通过其 `[1m]` 变体达到 1M，对该变体的访问取决于您的套餐。在 Max、Ultra、Team 和 Enterprise 套餐上，包括 Team Standard 和 Team Premium 席位，Opus 4.6 与 1M 上下文包含在您的订阅中。Sonnet 4.6 与 1M 上下文在每个订阅套餐上都需要[使用额度](https://support.claude.com/en/articles/12429409-extra-usage-for-paid-claude-plans)，包括 Max 和 Ultra。

| 套餐 | Opus 4.6 与 1M 上下文 | Sonnet 4.6 与 1M 上下文 |
| - | - | - |
| Max、Ultra、Team 和 Enterprise | 包含在订阅中 | 需要[使用额度](https://support.claude.com/en/articles/12429409-extra-usage-for-paid-claude-plans) |
| Pro | 需要[使用额度](https://support.claude.com/en/articles/12429409-extra-usage-for-paid-claude-plans) | 需要[使用额度](https://support.claude.com/en/articles/12429409-extra-usage-for-paid-claude-plans) |
| API 和按量付费 | 完全访问 | 完全访问 |

Claude Code 仅在直接连接到 Anthropic API 时检查这些套餐要求。如果您将 `ANTHROPIC_BASE_URL` 指向 [LLM 网关](https://code.claude.com/docs/zh-CN/llm-gateway#subscriptions-and-gateways)，且您保存的 claude.ai 登录仍为活跃凭据，Claude Code 不检查您套餐的使用额度。`/model` 中的 `[1m]` 选项保持可用，网关决定请求是否成功。在 v2.1.229 之前，当 Claude Code 无法确认账户上的使用额度时，它在该配置中拒绝 `/model sonnet[1m]`。

在 Anthropic API 上，1M 上下文窗口使用标准模型定价，超过 200K 的 token 没有溢价，但 Haiku 5.5 除外，它[在提示词超过 100K token 时费用更高](#haiku-5-5-context-window-and-pricing)。对于扩展上下文包含在您的订阅中的套餐，使用仍由您的订阅覆盖。对于通过使用额度访问扩展上下文的套餐，token 计费到使用额度。

#### 为 Opus 4.6 或 Sonnet 4.6 选择 1M 上下文

要按名称选择 1M 变体，请在模型别名或完整模型名称后附加 `[1m]` 后缀：

```text theme={null}
# Append [1m] to a full model name
/model claude-opus-4-6[1m]
/model claude-sonnet-4-6[1m]

# Or to an alias: the suffix applies to the model the alias resolves to
/model opus[1m]
```

#### LLM 网关后面的上下文窗口

如果您将 `ANTHROPIC_BASE_URL` 设置为 [LLM 网关](https://code.claude.com/docs/zh-CN/llm-gateway)或另一个代理，Claude Code 给每个它识别的模型与该模型在 Anthropic API 上具有的相同上下文窗口。Fable 5.1、Fable 5、Sonnet 5 及更高版本、Haiku 5.5 和 Opus 4.7 及更高版本获得 1M 窗口，没有 `[1m]` 变体可选择，仅通过其 `[1m]` 变体达到 1M 的模型（如 Opus 4.6）在没有它的情况下运行在 200K。Claude Code 无法检测网关或其后面的服务器强制的更低限制。如果您的网关拒绝超过 200K token 的请求，请在启动 Claude Code 的环境中设置 [`CLAUDE_CODE_AUTO_COMPACT_WINDOW=200000`](https://code.claude.com/docs/zh-CN/env-vars)，以便所有模型上的会话都[在该边界处压缩](#set-the-auto-compact-window)。

#### 关闭 1M 上下文

要将会话保持在 200K 窗口，请在您的 shell 或[设置文件](https://code.claude.com/docs/zh-CN/env-vars.md#set-environment-variables)中设置 `CLAUDE_CODE_DISABLE_1M_CONTEXT=1`。Claude Code 从模型选择器中删除 `[1m]` 模型变体。在默认运行 1M 窗口的模型上，例如 Fable 模型、Sonnet 5 及更高版本和 Opus 4.7 及更高版本，它也将模型视为具有 200K 上下文窗口：

* 启用自动压缩时，会话在 200K 边界处通过[自动压缩](#set-the-auto-compact-window)进行压缩。将自动压缩窗口设置在 200K 以上不会解除该限制，因为 Claude Code 将该窗口限制为模型的上下文窗口。
* 禁用自动压缩时，会话在 200K 边界处停止，出现[上下文限制错误](https://code.claude.com/docs/zh-CN/errors.md#prompt-is-too-long)，而不是压缩。

#### Haiku 5.5 上下文窗口和定价

在 Anthropic API 上，Haiku 5.5 在每个套餐上都运行 1M 上下文窗口，没有 `[1m]` 后缀可选择。其模型 ID 为 `claude-haiku-5-5`。要使用它，请在会话中运行 `/model claude-haiku-5-5`，或在 shell 中使用 `claude --model claude-haiku-5-5` 启动 Claude Code。

当 Haiku 5.5 请求的提示词超过 100K token 时，每 token 的费用更高。两种费率请参阅 [Anthropic 定价](https://platform.claude.com/docs/en/about-claude/pricing)。

会话默认在约 967K token 时自动压缩。要更早压缩，请为该模型[设置更小的自动压缩窗口](#set-the-auto-compact-window)。

## 上下文窗口和自动压缩

自动压缩窗口是指在 Claude Code 压缩对话之前，上下文窗口可以有多满。关于压缩保留和删除的内容，请参阅[压缩后保留的内容](https://code.claude.com/docs/zh-CN/context-window.md#what-survives-compaction)。

### 设置自动压缩窗口

> [!NOTE]
>
> 如果 Claude Code 就 `CLAUDE_CODE_AUTO_COMPACT_WINDOW` 的值发出了警告，请参阅[该变量的值和警告](#claude_code_auto_compact_window-values)。

您可以在以下位置设置自动压缩窗口：

* **对于当前模型，在此会话及以后的会话中**：运行 `/autocompact` 命令并指定一个值，例如 `/autocompact 500k`。Claude Code 将其保存到您的用户设置中 [`modelSettings`](https://code.claude.com/docs/zh-CN/settings-reference#modelsettings) 下的当前模型条目，并将其应用于当前会话。如果更高优先级的[设置作用域](https://code.claude.com/docs/zh-CN/settings.md#settings-precedence)（例如托管设置）为该模型或所有模型设置了自己的窗口，该命令会保存您的值，但会话会保持该作用域的窗口，命令会说明这一点。运行 `/autocompact auto` 以返回为您的模型调整的窗口。在 v2.1.288 之前，该命令会为所有模型保存同一个窗口，即顶层的 `autoCompactWindow`。
* **对于所有模型**：在设置文件中设置 [`autoCompactWindow`](https://code.claude.com/docs/zh-CN/settings-reference#autocompactwindow)，例如在 `~/.claude/settings.json` 中设置 `"autoCompactWindow": 200000`。对于某个模型，您使用 `/autocompact` 为该模型保存的窗口优先于同一文件中的此键。
* **对于一次启动**：启动 Claude Code 时传递 [`--autocompact`](https://code.claude.com/docs/zh-CN/cli-reference#cli-flags)。该标志会为该次启动覆盖您保存的设置，而不会更改它，`claude --autocompact auto` 会以调整的窗口运行会话，即使您保存的设置有一个值。与 `/autocompact` 不同，该标志不会被更高优先级的设置作用域（例如托管设置）抢占。
* **在脚本和云环境中**：将 [`CLAUDE_CODE_AUTO_COMPACT_WINDOW`](https://code.claude.com/docs/zh-CN/env-vars) 设置为 token 数，例如 `200000`。除非 Claude Code [忽略该值](#claude_code_auto_compact_window-values)，否则该变量优先于命令、标志和 `autoCompactWindow` 设置。此时 `/autocompact` 会报告该覆盖而不保存。状态栏的 `used_percentage` 是相对于模型的完整上下文窗口计算的，因此在设置该变量时，该百分比并不表示何时会运行压缩。
* **对于单个子代理**：请参阅[为单个子代理设置自动压缩窗口](https://code.claude.com/docs/zh-CN/sub-agents.md#subagent-auto-compact-window)。

命令和标志接受 100K 到 1M token 的窗口大小，采用以下任何形式：

* 纯 token 计数，例如 `200000`
* `k` 或 `M` 后缀，例如 `500k` 或 `1M`
* 100 到 1000 之间的裸数字，表示千位，所以 `200` 设置 200,000

无论您以哪种方式设置窗口，Claude Code 都会将其限制在模型的上下文窗口内。

#### `CLAUDE_CODE_AUTO_COMPACT_WINDOW` 的值和警告

如果警告中引用了您的 `CLAUDE_CODE_AUTO_COMPACT_WINDOW` 值，则表示该值超出了 100,000 到 1,000,000 的范围，或者不是 token 数。要消除该警告，请设置该范围内的数字，例如 `CLAUDE_CODE_AUTO_COMPACT_WINDOW=200000`。在 Claude Code v2.1.304 或更高版本上，您也可以用 `k` 表示千、用 `M` 表示百万来书写该数字，例如 `200k` 或 `1M`。

在第一列中找到您的警告措辞，然后横向查看会话使用的窗口以及需要更改的内容：

| 您的警告包含 | 会话使用的窗口 | 需要更改的内容 |
| :- | :- | :- |
| `reads as <N> tokens, so Claude Code treats it as the 100k minimum` | 100,000 token 与模型上下文窗口中的较小者 | 至少设置为 `100000`，例如 `200000`；如果您的 `200` 是指千位，则设置为 `200k` |
| `is too large, so Claude Code treats it as the 1M maximum` | 1,000,000 token 与模型上下文窗口中的较小者 | 您已经拥有最大的窗口。要消除该警告，请设置 `1M` 或 `1000000` |
| `isn't a usable size, so Claude Code ignores it` | 来自 `--autocompact`、`/autocompact` 或 `autoCompactWindow` 的窗口；如果均未设置，则为[默认值](#default-auto-compact-thresholds) | 设置一个数字，例如 `200000`，或取消设置该变量 |

不带后缀的数字会被读取为相应数量的 token，因此 `CLAUDE_CODE_AUTO_COMPACT_WINDOW=200` 表示 200 个 token，而 `/autocompact 200` 表示 200,000。

### 默认自动压缩阈值

如果您没有设置自动压缩窗口，Claude Code 会在对话达到模型的上下文限制时进行压缩，除了以下会话：

* [云端会话](https://code.claude.com/docs/zh-CN/claude-code-on-the-web.md)在对话接近模型限制时进行压缩
* Sonnet 4.6 和 Opus 4.6（不带[扩展上下文](#extended-context)）在 200K 边界处进行压缩
* 当您设置 [`CLAUDE_CODE_DISABLE_1M_CONTEXT=1`](https://code.claude.com/docs/zh-CN/env-vars) 时，具有原生 1M 窗口的模型（例如 Sonnet 5 和 Fable 模型）在 200K 边界处进行压缩
* 使用原生 1M 窗口运行的模型在窗口填满之前进行压缩，默认情况下约为 967K token。这些模型包括 Fable 模型、Sonnet 5 及更高版本、Haiku 5.5 以及 Opus 4.7 及更高版本。在自定义 `ANTHROPIC_BASE_URL` 后面，请参阅[网关后面的上下文窗口](#context-window-behind-a-gateway)
* 在 Claude Code 不识别的模型 ID（例如 [LLM 网关](https://code.claude.com/docs/zh-CN/llm-gateway)别名）上的会话在 Claude Code 为该 ID 假设的上下文窗口处进行压缩；请参阅[为网关或自定义模型 ID 更正窗口](#correct-the-window-for-a-gateway-or-custom-model-id)

关于 Claude Code 何时还会压缩您闲置的对话，请参阅 [`idleCompaction`](https://code.claude.com/docs/zh-CN/settings-reference#idlecompaction) 设置。

### 为网关或自定义模型 ID 更正窗口

以下任一启动警告都表示 Claude Code 无法识别您会话的模型 ID，因此无法得知该模型的上下文窗口，并假设其为 200K token：

```text theme={null}
⚠ Model "my-gateway-opus" isn't recognized · context assumed to be 200k tokens · ...
Claude Code doesn't recognize the model "my-gateway-opus", so it assumes a context window of 200k tokens. Update Claude Code, or see https://code.claude.com/docs/en/model-config#correct-the-window-for-a-gateway-or-custom-model-id to set the window.
```

Claude Code 会在假设的 200K 处[自动压缩](#default-auto-compact-thresholds)对话，因此如果模型的实际上下文窗口为 200K，则无需更正。否则，请根据您的 ID 选择以下适用的方式，将实际窗口告知 Claude Code：

* **比您的 Claude Code 版本更新的模型**：在 shell 中运行 `claude update`。如果更新后 Claude Code 能识别该 ID，则无需再配置其他内容。
* **您的网关或提供商为 Claude Code 已知模型使用的名称**：添加一个 [`modelOverrides`](#override-model-ids-per-version) 条目，将 Anthropic 模型 ID 映射到您的 ID，例如 `"claude-opus-4-8": "my-gateway-opus"`。您也可以改为在 [`modelPicker`](https://code.claude.com/docs/zh-CN/settings-reference#fields-for-modelpicker) 行中列出您的 ID，并将 `behavesAs` 设置为该模型。无论采用哪种方式，Claude Code 随后都会将该 ID 视为该模型。
* **具有 1M 窗口的模型**：在设置模型的位置为 ID 追加 `[1m]`，例如在 shell 中运行 `claude --model 'my-gateway-opus[1m]'`。如果您[关闭了 1M 上下文](#turn-off-1m-context)，则无论是否带有该后缀，您获得的窗口都相同。
* **任何其他窗口**：在启动 Claude Code 的环境中将 [`CLAUDE_CODE_MAX_CONTEXT_TOKENS`](https://code.claude.com/docs/zh-CN/env-vars) 设置为以 token 为单位的窗口大小，例如 `CLAUDE_CODE_MAX_CONTEXT_TOKENS=400000`。对于不以 `claude-` 开头且没有 `[1m]` 后缀的无法识别的 ID，该变量会单独生效。对于任何其他 ID，请先查看[它适用于哪些 ID](#which-ids-claude_code_max_context_tokens-applies-to)。

一旦 Claude Code 识别了该 ID 或使用了您提供的窗口，在该 ID 上启动会话时就不会再出现该警告。

#### `CLAUDE_CODE_MAX_CONTEXT_TOKENS` 适用于哪些 ID

在 [LLM 网关](https://code.claude.com/docs/zh-CN/llm-gateway)或其他自定义部署上，Claude Code 可能会为模型 ID 假设一个与模型实际窗口不同的上下文窗口，无论它是否将 ID 解析为 Claude 模型。设置 [`CLAUDE_CODE_MAX_CONTEXT_TOKENS`](https://code.claude.com/docs/zh-CN/env-vars) 为 Claude Code 应该假设的窗口。

变量的应用方式取决于 ID。当 ID 不以 `claude-`（任何大小写）开头时，或当它携带 Claude Code 在读取 ID 时剥离的后缀（例如 Google Cloud 的 Agent Platform 上使用的 `@YYYYMMDD` 日期）时，Claude Code 将 ID 视为提供商或自定义拼写。在 v2.1.259 之前，Claude Code 没有计算剥离的后缀，所以带有日期后缀的无法识别的 `claude-` ID 被视为裸 `claude-` 名称。

无法识别的提供商或自定义拼写、相同拼写加上 `[1m]` 和所有其他 ID 是三种不同的情况：

* 如果 Claude Code 无法将提供商或自定义拼写解析为它识别的模型，且 ID 不包含 `[1m]`，则变量直接应用，主动压缩在声明的窗口处继续。
* 如果 Claude Code 无法将提供商或自定义拼写解析为它识别的模型，且 ID 包含 `[1m]`（任何大小写），Claude Code 为其假设 1M 窗口，变量本身不适用。要在保持主动压缩的同时更正窗口，还要设置 [`CLAUDE_CODE_DISABLE_1M_CONTEXT=1`](https://code.claude.com/docs/zh-CN/env-vars)。设置该变量后，Claude Code 会像相同拼写但不带 `[1m]` 一样调整 ID 的大小，所以当 `CLAUDE_CODE_MAX_CONTEXT_TOKENS` 适用于该无标记拼写时，它就会应用。

  当声明的窗口大于 200K 时，在此配置中出现[200K 限制未被强制执行](https://code.claude.com/docs/zh-CN/errors.md#the-200k-limit-isnt-enforced)的警告是预期的。
* 如果 ID 解析为 Claude Code 识别的模型，或 ID 是没有 Claude Code 要剥离的后缀的裸 `claude-` 名称（任何大小写），变量仅在您也设置 [`DISABLE_COMPACT`](https://code.claude.com/docs/zh-CN/env-vars) 时生效，这会禁用所有压缩。

  例如，包含 Claude Code 知道的 Claude 模型名称的 ID（例如 `anthropic/claude-opus-4-8`、`us.anthropic.claude-…-v1:0` 或带日期的 `claude-sonnet-4-5@20250929`）解析为该模型。这包括也包含 `[1m]` 的 ID：即使设置了 `CLAUDE_CODE_DISABLE_1M_CONTEXT`，Claude Code 也会将 `claude-opus-4-8[1m]` 解析为 Opus 4.8。

对于 Claude Code 不识别的模型 ID，设置 [`CLAUDE_CODE_DISABLE_UNKNOWN_MODEL_WINDOW_ENFORCEMENT=1`](https://code.claude.com/docs/zh-CN/env-vars) 以让 Claude Code 仅在 API 以 Claude Code 识别的[过长错误](https://code.claude.com/docs/zh-CN/errors.md#prompt-is-too-long)拒绝对话后才进行压缩。当网关[将错误重写](https://code.claude.com/docs/zh-CN/llm-gateway-connect.md#troubleshoot-gateway-errors)为 Claude Code 不识别的措辞时，Claude Code 不会运行该恢复。

## 检查您当前的模型

您可以在两个位置查看您当前使用的模型：

* 在[状态行](https://code.claude.com/docs/zh-CN/statusline.md)中（如果已配置）
* 在 `/status` 中，它也显示您的账户信息

## 添加自定义模型选项

使用 `ANTHROPIC_CUSTOM_MODEL_OPTION` 向 `/model` 选择器添加单个自定义条目，而无需替换内置别名。这对于测试 Claude Code 默认不列出的模型 ID 很有用。对于 LLM 网关部署，当设置 `CLAUDE_CODE_ENABLE_GATEWAY_MODEL_DISCOVERY=1` 时，Claude Code 可以从网关的 `/v1/models` 端点自动填充选择器，因此仅当发现被禁用或未返回您想要的模型时才需要此变量。请参阅 [网关模型发现](https://code.claude.com/docs/zh-CN/llm-gateway-protocol.md#model-discovery)。

要列出多个模型，按您自己的顺序和您选择的标签，请设置 [`modelPicker`](https://code.claude.com/docs/zh-CN/settings-reference#modelpicker)。其条目说明当该阵容替换内置阵容时选择器保留哪些行。

此示例设置所有三个变量以使网关路由的 Opus 部署可选择。Claude Code 在启动时读取环境变量，因此在启动 `claude` 之前运行导出，或重启现有会话以获取它们：

```bash theme={null}
export ANTHROPIC_CUSTOM_MODEL_OPTION="my-gateway/claude-opus-5-5"
export ANTHROPIC_CUSTOM_MODEL_OPTION_NAME="Opus via Gateway"
export ANTHROPIC_CUSTOM_MODEL_OPTION_DESCRIPTION="Custom deployment routed through the internal LLM gateway"
```

`ANTHROPIC_CUSTOM_MODEL_OPTION_NAME` 和 `ANTHROPIC_CUSTOM_MODEL_OPTION_DESCRIPTION` 是可选的：

* 如果您省略名称，当 Claude Code [识别 ID](#customize-pinned-model-display-and-capabilities) 时条目显示模型的名称，否则显示模型 ID。
* 如果您省略描述，Claude Code 使用 `Custom model (<model-id>)`。

Claude Code 在内置条目之后列出自定义条目，您追加的任何 [`modelPicker`](https://code.claude.com/docs/zh-CN/settings-reference#modelpicker) 行都在其后面。

Claude Code 跳过对 `ANTHROPIC_CUSTOM_MODEL_OPTION` 中设置的模型 ID 的验证，因此您可以使用您的 API 端点接受的任何字符串。

当设置 [`availableModels`](#restrict-model-selection) 时，也要在允许列表中包含自定义模型 ID。否则 Claude Code 会从选择器中过滤自定义条目，并拒绝对其进行 `--model` 选择，就像任何其他被排除的模型一样。

嵌入了系列名称的自定义 ID（例如 `my-gateway/claude-opus-5-5`），计为该系列的特定条目并禁用其通配符，因此还要列出您打算保持可选择的版本。请参阅 [合并行为](#merge-behavior)。

## 环境变量

使用以下环境变量来控制别名映射到的模型名称。每个值必须是完整的模型名称，或您的 API 提供商的等效标识符。要选择会话启动时使用的模型，请设置 [`ANTHROPIC_DEFAULT_MODEL`](#set-a-default-model-for-new-sessions)，此表中省略了该变量。

| 环境变量 | 描述 |
| - | - |
| `ANTHROPIC_DEFAULT_FABLE_MODEL` | 用于 `fable` 的模型，以及 Claude Code 识别为 Fable 模型的模型 ID，用于[第三方提供商上的自动模型回退](#automatic-model-fallback) |
| `ANTHROPIC_DEFAULT_OPUS_MODEL` | 用于 `opus` 的模型，或在计划模式活跃时用于 `opusplan` 的模型。 |
| `ANTHROPIC_DEFAULT_SONNET_MODEL` | 用于 `sonnet` 的模型，或在计划模式不活跃时用于 `opusplan` 的模型。 |
| `ANTHROPIC_DEFAULT_HAIKU_MODEL` | 用于 `haiku` 的模型，或[后台功能](https://code.claude.com/docs/zh-CN/costs.md#background-token-usage) |
| `CLAUDE_CODE_SUBAGENT_MODEL` | [子代理](https://code.claude.com/docs/zh-CN/sub-agents.md#choose-a-model)、[agent team](https://code.claude.com/docs/zh-CN/agent-teams.md#specify-teammates-and-models) 队友和[工作流](https://code.claude.com/docs/zh-CN/workflows.md) Agent 的默认模型，适用于未以其他方式分配模型的情况。接受别名（如 `haiku`）或完整模型名称。每次调用的模型或定义的 `model` 字段（包括 `inherit`）优先。要改变这一点，请设置 [`CLAUDE_CODE_SUBAGENT_MODEL_FORCE`](https://code.claude.com/docs/zh-CN/sub-agents#run-every-subagent-on-one-model) |

在第三方提供商上，[自定义固定模型显示和功能](#customize-pinned-model-display-and-capabilities)描述了固定模型在 `/model` 选择器中的行显示的内容。

注意：`ANTHROPIC_SMALL_FAST_MODEL` 已弃用，改为使用 `ANTHROPIC_DEFAULT_HAIKU_MODEL`。

### 为第三方部署固定模型

当通过 [Amazon Bedrock](https://code.claude.com/docs/zh-CN/amazon-bedrock.md)、[Google Cloud's Agent Platform](https://code.claude.com/docs/zh-CN/google-vertex-ai.md)、[Microsoft Foundry](https://code.claude.com/docs/zh-CN/microsoft-foundry.md) 或 [Claude Platform on AWS](https://code.claude.com/docs/zh-CN/claude-platform-on-aws.md) 部署 Claude Code 时，在向用户推出前固定模型版本。

不固定模型，Claude Code 会使用模型别名（如 `fable`、`opus`、`sonnet` 和 `haiku`），这些别名会解析为每个提供商的内置默认模型 ID。该默认值可能滞后于最新的 Anthropic 版本，并且它指向的模型可能尚未在用户账户中启用。当默认值不可用时，Amazon Bedrock 和 Google Cloud's Agent Platform 用户会看到通知，会话会回退到该默认模型的早期版本，或当默认值是 Opus 模型且没有 Opus 版本可用时回退到默认 Sonnet 模型。Microsoft Foundry 用户则会看到错误，因为 Microsoft Foundry 没有等效的启动检查。

在 Amazon Bedrock 和 Google Cloud's Agent Platform 上，以特定 Sonnet 或 Opus 版本启动会话的用户（例如使用 `--model`、`ANTHROPIC_MODEL` 或 `model` 设置），会将该版本固定为会话的默认值，用于匹配的别名：启动检查会跳过它替换的内置默认值，并且不显示回退通知。在 v2.1.211 之前，即使会话模型被显式配置，检查也会运行并可能显示通知。

> [!WARNING]
>
> 在初始设置中将模型环境变量设置为特定版本 ID。固定让您控制用户何时迁移到新模型。

对您的提供商使用以下环境变量和特定版本的模型 ID：

| 提供商 | 示例 |
| :- | :- |
| Amazon Bedrock | `export ANTHROPIC_DEFAULT_OPUS_MODEL='us.anthropic.claude-opus-4-8'` |
| Google Cloud's Agent Platform | `export ANTHROPIC_DEFAULT_OPUS_MODEL='claude-opus-4-8'` |
| Microsoft Foundry | `export ANTHROPIC_DEFAULT_OPUS_MODEL='claude-opus-4-8'` |

对 `ANTHROPIC_DEFAULT_FABLE_MODEL`、`ANTHROPIC_DEFAULT_SONNET_MODEL` 和 `ANTHROPIC_DEFAULT_HAIKU_MODEL` 应用相同的模式。有关所有提供商的当前和旧版模型 ID，请参阅[模型概览](https://platform.claude.com/docs/en/about-claude/models/overview)。要将用户升级到新模型版本，请更新这些环境变量并重新部署。

具有原生 1M 窗口的固定模型（如 Opus 4.8 或 Sonnet 5），当 Claude Code 能够将固定 ID 与该模型匹配时，无需任何后缀即可以 [1M 上下文窗口](#extended-context)运行。当 ID 包含该模型的 Anthropic API ID（例如 `us.anthropic.claude-opus-4-8` 包含 `claude-opus-4-8`），或 [`modelOverrides`](#override-model-ids-per-version) 条目将该模型映射到该 ID 时，即视为匹配。对于 Claude Code 无法与模型匹配的固定 ID，会话默认以 200K 窗口运行，除非该 ID 带有 `[1m]` 后缀。

对于通过其 `[1m]` 变体达到 1M 的模型（如 Opus 4.6 或 Sonnet 4.6），请在 `ANTHROPIC_DEFAULT_OPUS_MODEL` 或 `ANTHROPIC_DEFAULT_SONNET_MODEL` 中的模型 ID 后附加 `[1m]` 以启用扩展上下文：

```bash theme={null}
export ANTHROPIC_DEFAULT_OPUS_MODEL='claude-opus-4-6[1m]'
```

使用 `[1m]` 后缀，1M 上下文窗口适用于固定别名的所有使用，包括 [`opusplan`](#opusplan-model-setting) 的计划模式 Opus 阶段和 `model` frontmatter 指定该别名的[子代理](https://code.claude.com/docs/zh-CN/sub-agents.md#choose-a-model)。

* Claude Code 在将模型 ID 发送到您的提供商之前会删除该后缀。
* 仅当底层模型[支持 1M 上下文](https://platform.claude.com/docs/en/build-with-claude/context-windows#context-window-sizes-by-model)时才附加 `[1m]`。
* 该后缀按变量读取，而不是按模型读取。在 Amazon Bedrock、Google Cloud's Agent Platform 和 Microsoft Foundry 上，一个变量中没有 `[1m]` 的 Opus 4.6 或 Sonnet 4.6 ID 使用 200K 上下文，即使另一个变量使用相同的模型和后缀。

当您设置 `ANTHROPIC_DEFAULT_*_MODEL` 变量时，`/model` 选择器会显示该模型的一行来替代该家族的内置行，包括任何 1M 上下文行。要在不向该变量添加后缀的情况下到达 1M 窗口，您的用户运行 `/model opus[1m]`，Claude Code 会将后缀应用于该变量命名的模型。`/model sonnet[1m]` 的工作方式相同。

> [!NOTE]
>
> 通过 [MDM 或托管设置文件](https://code.claude.com/docs/zh-CN/managed-settings.md#delivery-mechanisms) 提供的 `availableModels` 允许列表在使用第三方提供商时仍然适用；[服务器托管设置不会在那里提供](https://code.claude.com/docs/zh-CN/server-managed-settings.md#platform-availability)。
>
> 过滤与模型别名（如 `opus`）、版本前缀（如 `claude-opus-4-8`）或完整提供商形式的模型 ID 匹配。提供商特定的前缀（如 `us.anthropic.`）不会被删除，因此要允许特定模型，请通过 [`modelOverrides`](#override-model-ids-per-version) 映射它，或列出其完整提供商形式 ID：
>
> * **Google Cloud's Agent Platform**：当 Claude Code 请求的 ID 带有 `@` 日期（如 `claude-sonnet-4-5@20250929`）时，请在允许列表条目中包含该日期。单独的 `claude-sonnet-4-5` 在任一 [`availableModelsMatch`](https://code.claude.com/docs/zh-CN/settings-reference#availablemodelsmatch) 设置下都不会匹配该 ID。
> * **固定模型**：列出您在该模型的 `ANTHROPIC_DEFAULT_*_MODEL` 变量中设置的值
>
> 任何 `[1m]` 后缀在匹配前都会从允许列表条目和请求的模型中删除。

### 自定义固定模型显示和功能

当您在第三方提供商上固定模型时，其在 `/model` 选择器中的行如果 Claude Code 识别固定 ID，则默认显示模型的名称，否则显示原始 ID：

* **已识别**：Claude Code 知道的模型的确切 ID，例如其 Anthropic API ID 或您的提供商或网关的形式，带或不带 `[1m]` 后缀。固定 `us.anthropic.claude-sonnet-4-5-20250929-v1:0`，该行显示 `Sonnet 4.5`。
* **未识别**：任何其他 ID，例如应用推理配置文件 ARN 或 Claude Code 不知道的模型版本，除非 [`modelOverrides`](#override-model-ids-per-version) 条目将模型映射到该 ID。在 Microsoft Foundry 上，部署名称是用户定义的，因此 Claude Code 在那里永远不会识别固定 ID，无论是否映射，该行默认显示部署名称。

当行显示模型的名称时，其默认描述包括固定 ID，以便您仍然可以看到固定了哪个 ID。

Claude Code 也可能无法识别固定模型支持的功能。您可以自己设置显示名称和描述，并为每个固定模型使用伴随环境变量声明功能。

这些变量在第三方提供商（如 Amazon Bedrock、Google Cloud's Agent Platform 和 Microsoft Foundry）上生效。`_NAME` 和 `_DESCRIPTION` 变量在 `ANTHROPIC_BASE_URL` 指向 [LLM 网关](https://code.claude.com/docs/zh-CN/llm-gateway) 时也生效。当直接连接到 `api.anthropic.com` 时无效。

| 环境变量 | 描述 |
| - | - |
| `ANTHROPIC_DEFAULT_OPUS_MODEL_NAME` | 固定 Opus 模型在 `/model` 选择器中的显示名称。未设置时，如果 Claude Code 识别固定 ID，该行显示模型的名称，否则显示固定 ID |
| `ANTHROPIC_DEFAULT_OPUS_MODEL_DESCRIPTION` | 固定 Opus 模型在 `/model` 选择器中的显示描述。未设置时，该行显示以 `Custom Opus model` 开头的默认描述 |
| `ANTHROPIC_DEFAULT_OPUS_MODEL_SUPPORTED_CAPABILITIES` | 固定 Opus 模型支持的功能的逗号分隔列表 |

相同的 `_NAME`、`_DESCRIPTION` 和 `_SUPPORTED_CAPABILITIES` 后缀可用于 `ANTHROPIC_DEFAULT_SONNET_MODEL`、`ANTHROPIC_DEFAULT_HAIKU_MODEL`、`ANTHROPIC_DEFAULT_FABLE_MODEL` 和 `ANTHROPIC_CUSTOM_MODEL_OPTION`。

Claude Code 通过将模型 ID 与已知模式匹配来启用 [effort 级别](#adjust-effort-level)和[扩展思考](#extended-thinking)等功能。提供商特定的 ID（如 Amazon Bedrock ARN 或自定义部署名称）通常与这些模式不匹配，导致支持的功能被禁用。设置 `_SUPPORTED_CAPABILITIES` 以告诉 Claude Code 模型实际支持的功能：

| 功能值 | 启用 |
| - | - |
| `effort` | [effort 级别](#adjust-effort-level)和 `/effort` 命令 |
| `xhigh_effort` | `xhigh` effort 级别 |
| `max_effort` | `max` effort 级别 |
| `thinking` | [扩展思考](#extended-thinking) |
| `adaptive_thinking` | 根据任务复杂性动态分配思考的自适应推理 |
| `interleaved_thinking` | 工具调用之间的思考 |

设置 `_SUPPORTED_CAPABILITIES` 时，Claude Code 会为匹配的固定模型启用列出的功能，并禁用未列出的功能。未设置该变量时，Claude Code 回退到基于模型 ID 的内置检测。

此示例将 Opus 固定到 Amazon Bedrock 自定义模型 ARN，设置友好名称，并声明其功能：

```bash theme={null}
export ANTHROPIC_DEFAULT_OPUS_MODEL='arn:aws:bedrock:us-east-1:123456789012:custom-model/abc'
export ANTHROPIC_DEFAULT_OPUS_MODEL_NAME='Opus via Bedrock'
export ANTHROPIC_DEFAULT_OPUS_MODEL_DESCRIPTION='Opus 4.7 routed through a Bedrock custom endpoint'
export ANTHROPIC_DEFAULT_OPUS_MODEL_SUPPORTED_CAPABILITIES='effort,xhigh_effort,max_effort,thinking,adaptive_thinking,interleaved_thinking'
```

### 按版本覆盖模型 ID

在嵌入 Claude Code 并设置 [`CLAUDE_CODE_PROVIDER_MANAGED_BY_HOST`](https://code.claude.com/docs/zh-CN/env-vars) 的平台上，主机的模型配置优先于托管模型设置，而托管 `availableModels` 允许列表保持有效，除非主机提供自己的；[托管设置优先级的例外](https://code.claude.com/docs/zh-CN/settings.md#exceptions-to-managed-settings-precedence)说明主机覆盖的键和变量。

上面的家族级环境变量为每个家族别名配置一个模型 ID。如果您需要将同一家族中的多个版本映射到不同的提供商 ID，请改用 `modelOverrides` 设置。

`modelOverrides` 将单个 Anthropic 模型 ID 映射到 Claude Code 发送到您的提供商 API 的提供商特定字符串。当用户在 `/model` 选择器中选择映射的模型时，Claude Code 会使用您配置的值而不是内置默认值。

这让企业管理员可以将每个模型版本路由到特定的 Amazon Bedrock 推理配置文件 ARN、Google Cloud's Agent Platform 版本名称或 Microsoft Foundry 部署名称，用于治理、成本分配或区域路由。

在您的[设置文件](https://code.claude.com/docs/zh-CN/settings.md#where-settings-live)中设置 `modelOverrides`：

```json theme={null}
{
  "modelOverrides": {
    "claude-opus-4-7": "arn:aws:bedrock:us-east-2:123456789012:application-inference-profile/opus-prod",
    "claude-opus-4-6": "arn:aws:bedrock:us-east-2:123456789012:application-inference-profile/opus-46-prod",
    "claude-sonnet-4-6": "arn:aws:bedrock:us-east-2:123456789012:application-inference-profile/sonnet-prod"
  }
}
```

键必须是[模型概览](https://platform.claude.com/docs/en/about-claude/models/overview)中列出的 Anthropic 模型 ID。对于带日期的模型 ID，请包含日期后缀，完全按照其显示的方式。未知的键会被忽略。

要停止针对网关别名等 ID 的 `[claude-code:unrecognized_model]` [诊断行](https://code.claude.com/docs/zh-CN/errors.md#unrecognized-model-id-on-a-request)，请添加一个以该 ID 作为其值的条目。

覆盖替换了支持 `/model` 选择器中每个条目的内置模型 ID。在 Amazon Bedrock 上，`modelOverrides` 条目优先于 Claude Code 在启动时自动发现的任何推理配置文件。Claude Code 将已经是提供商原生的值（如 Amazon Bedrock 推理配置文件 ARN 或 Microsoft Foundry 部署名称）按原样传递给提供商。

当您通过 `--model`、`ANTHROPIC_MODEL` 环境变量或 `ANTHROPIC_DEFAULT_*_MODEL` 环境变量直接传递 Anthropic 模型 ID 时，覆盖也适用。在 Amazon Bedrock、Google Cloud's Agent Platform 和 [Mantle](https://code.claude.com/docs/zh-CN/amazon-bedrock.md#use-the-mantle-endpoint) 上，没有 `modelOverrides` 条目的 Anthropic 模型 ID 解析为与该版本的 `/model` 选择器行相同的提供商特定 ID（当提供商支持该版本时）。Mantle 支持版本的子集。对于该子集之外的 Anthropic 模型 ID，Claude Code 将原始 ID 发送到 Mantle 而不进行映射，除非 `modelOverrides` 条目覆盖它。在 v2.1.200 之前，`--model` 和环境变量值直接到达提供商，不经过覆盖映射。

`modelOverrides` 与 `availableModels` 一起工作。允许列表针对 Anthropic 模型 ID 进行评估，而不是覆盖值，因此 `availableModels` 中的条目（如 `"opus"`）即使在 Opus 版本映射到 ARN 时也会继续匹配。当在托管设置中设置 `enforceAvailableModels` 时，强制执行的默认值仅通过来自[托管设置](https://code.claude.com/docs/zh-CN/managed-settings.md#how-claude-code-combines-managed-sources)的 `modelOverrides` 解析。管理员的映射（如固定到推理配置文件 ARN 的版本）在强制执行的默认值中得到遵守。来自用户或项目设置的覆盖不会影响它。

当 `availableModels` 在[托管设置](https://code.claude.com/docs/zh-CN/managed-settings.md)中设置时，仅来自托管设置的 `modelOverrides` 适用于通过 `--model` 或上述环境变量直接传递的 Anthropic 模型 ID。Claude Code 忽略用户或项目设置中针对这些 ID 的覆盖，并且永远不会通过任何设置源的 `modelOverrides` 解析托管列表排除的 ID。此托管源限制需要 Claude Code v2.1.200 或更高版本。有关如何处理被阻止的 ID，请参阅[限制模型选择](#restrict-model-selection)。

### 提示缓存配置

Claude Code 自动使用[提示缓存](https://code.claude.com/docs/zh-CN/prompt-caching.md)来优化性能并降低成本。您可以全局禁用提示缓存或针对特定模型层级禁用：

| 环境变量 | 描述 |
| - | - |
| `DISABLE_PROMPT_CACHING` | 设置为 `1` 以禁用所有模型的提示缓存。优先于按模型设置 |
| `DISABLE_PROMPT_CACHING_HAIKU` | 设置为 `1` 以禁用[默认 Haiku 模型](https://code.claude.com/docs/zh-CN/prompt-caching.md#disable-prompt-caching)的提示缓存 |
| `DISABLE_PROMPT_CACHING_SONNET` | 设置为 `1` 以禁用[默认 Sonnet 模型](https://code.claude.com/docs/zh-CN/prompt-caching.md#disable-prompt-caching)的提示缓存 |
| `DISABLE_PROMPT_CACHING_OPUS` | 设置为 `1` 以禁用[默认 Opus 模型](https://code.claude.com/docs/zh-CN/prompt-caching.md#disable-prompt-caching)的提示缓存 |
| `DISABLE_PROMPT_CACHING_FABLE` | 设置为 `1` 以仅禁用 Fable 模型的提示缓存 |

要为主对话和子代理分别选择缓存 TTL，请参阅[自己选择 TTL](https://code.claude.com/docs/zh-CN/prompt-caching.md#choose-the-ttl-yourself)。有关什么会触发缓存未命中，请参阅 [Claude Code 如何使用提示缓存](https://code.claude.com/docs/zh-CN/prompt-caching.md)。

## 版本历史

此表列出了每个模型别名更改其解析模型的 Claude Code 版本，最新版本在前。

| 版本 | 更改 |
| :- | :- |
| v2.1.293 | `haiku` 在 Anthropic API 上解析为 Haiku 5.5 |
| v2.1.284 | `sonnet` 在 Anthropic API 上解析为 Sonnet 5.5 |
| v2.1.280 | `opus` 在 Anthropic API、AWS 上的 Claude Platform、Amazon Bedrock 和 Google Cloud 的 Agent Platform 上解析为 Opus 5.5 |
| v2.1.257 | `fable` 解析为 Fable 5.1，Claude 应用网关会话中除外 |
| v2.1.219 | `opus` 在 Anthropic API、AWS 上的 Claude Platform、Amazon Bedrock 和 Agent Platform 上解析为 Opus 5 |
| v2.1.207 | `opus` 在 AWS 上的 Claude Platform、Amazon Bedrock 和 Agent Platform 上解析为 Opus 4.8 |
| v2.1.197 | `sonnet` 在 Anthropic API 上解析为 Sonnet 5 |
| v2.1.154 | `opus` 在 Anthropic API 上解析为 Opus 4.8 |
| 更早版本 | `opus` 在 AWS 上的 Claude Platform 上解析为 Opus 4.7，在 Amazon Bedrock 和 Agent Platform 上解析为 Opus 4.6。`fable` 在每个提供商上解析为 Fable 5，`haiku` 在每个提供商上解析为 Haiku 4.5 |
