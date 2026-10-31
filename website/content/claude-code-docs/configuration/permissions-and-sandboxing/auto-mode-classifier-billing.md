> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# 自动模式分类器请求费用

> 解决 Claude Code 通知说此会话不符合自动模式免费分类器请求条件的问题：它的含义、为什么出现，以及该怎么做。

在 [auto mode](https://code.claude.com/docs/zh-CN/permission-modes.md#eliminate-prompts-with-auto-mode) 中，分类器在 shell 命令和网络请求等操作运行前对其进行检查。在 [服务器端检查启用](https://code.claude.com/docs/zh-CN/permission-modes.md#server-side-classifier-review) 的地方，服务器将这些检查作为会话自身模型请求的一部分执行，不收费。此通知意味着服务器的检查无法到达您的会话，因此 Claude Code 正在发出自己的分类器请求，在您的账户上这些请求计入您的令牌使用量：

```text theme={null}
We're changing auto mode to no longer charge for classifier requests in Claude Code. However, this session isn't eligible.
```

在该提示处，Claude Code 会暂停它要以这种方式检查的第一个操作，直到您回答。自动模式继续工作，其分类器请求按之前的方式计费。

最常见的原因是 Claude Code 和 API 之间存在 LLM 网关或代理，当 Claude Code 能够识别出它时，通知会将其名称显示出来。按 **Enter** 继续，或参阅[使会话符合条件](#make-the-session-eligible)以防止其在新会话中出现。

## 响应通知

该通知会保持操作处于待处理状态，直到您回答：

* **Enter** 继续：保持的操作和会话的其余部分使用 Claude Code 自己的分类器请求，按照之前的方式计费为令牌使用，该通知在该会话中不会再次出现。当通知命名了网关时，确认它会防止它在这台机器上的 24 小时内再次出现。当它没有命名时，通知会在下次会话回退时返回。
* **Esc** 或 **Ctrl+C** 取消：保持的操作不会运行，当前轮次停止，会话仍处于自动模式。没有任何内容被记住，所以通知会在下一个检查的操作之前再次出现。

要停止使用自动模式，请在您回答后使用 `Shift+Tab` 切换权限模式。

在无法等待答案的地方，Claude Code 报告相同的文本，会话继续以自动模式运行，除非此机器上过去 24 小时内的网关确认已将其关闭。在 [non-interactive mode](https://code.claude.com/docs/zh-CN/headless.md) 中使用 `-p` 时，它会将文本打印到 stderr，在 `stream-json` 输出中，它会发出 `system` 警告消息，Agent SDK 应用程序可以从消息流中读取该消息。

## 使会话符合条件

如果网关是原因，请要求您公司的管理员或网关提供商原封不动地传递请求和回复。这意味着按原样转发请求头和正文字段，包括网关不识别的字段，例如 `safeguards` 请求字段，以及返回响应和流式事件而不删除键（例如 `safeguard_results` 字段）或重写工具使用 ID，如[网关兼容性指南](https://code.claude.com/docs/zh-CN/llm-gateway-protocol.md#feature-pass-through)所述。以这种方式传递流量的网关可继续与此功能和未来功能配合使用。新会话随后再次使用服务器的检查。

如果您已经知道您的网关无法提供服务器的检查，请在启动会话之前通过在 shell 中或在 [`env` 设置键](https://code.claude.com/docs/zh-CN/settings-reference#env)中将 `CLAUDE_CODE_AUTO_MODE_SERVER` 设置为 `0` 来告诉 Claude Code 不要在那里请求它们：

```bash theme={null}
export CLAUDE_CODE_AUTO_MODE_SERVER=0
```

分类器请求随后始终是 Claude Code 自己的，以相同方式计费，通知不会出现。在与 Anthropic API 的直接连接上，该变量需要 Claude Code v2.1.281 或更高版本。当 `CLAUDE_CODE_AUTO_MODE_SERVER` 未设置时，设置 `CLAUDE_CODE_DISABLE_EXPERIMENTAL_BETAS=1` 也会关闭服务器的检查，除了[禁用预发布功能](https://code.claude.com/docs/zh-CN/llm-gateway-protocol.md#disable-pre-release-capabilities)所述的情况。

`CLAUDE_CODE_AUTO_MODE_SERVER` 是一个临时设置，可能在后续版本中被移除。

## 为什么会出现通知

使用 Pro、Max、Ultra 或 Team 计划时，永远不会看到该通知。要了解哪些会话会向服务器请求分类器检查，请参阅[服务器端分类器审查](https://code.claude.com/docs/zh-CN/permission-modes.md#server-side-classifier-review)。如果您看到该通知，请检查以下常见原因：

* **路径中存在 LLM 网关或代理**：一个会剥离或重写请求头、丢弃它不识别的请求字段或编辑响应的网关。服务器随后永远不会收到检查请求，或 Claude Code 永远不会收到结果。当您的配置或响应识别网关时，通知会将其命名。
* **服务器端检查尚未到达您的平台、区域或凭证**：平台或区域是否执行检查取决于该平台的推出情况。如果您看到通知且路径中没有网关或代理，并且它持续出现，这是可能的原因。要确认，请联系支持或您公司的管理员，或使用 `/feedback` 报告。

要检查处于自动模式的会话，请在 Claude Code 提示符处运行 `/status`：其**自动模式服务器**行在服务器的检查决定会话的操作时读取 `Enabled`，在会话回退后读取 `Disabled`。

当网关截断响应或将结果重写为 Claude Code 无法读取的形式时，您会收到没有判决的拒绝，而不是此通知；请参阅[服务器端分类器审查](https://code.claude.com/docs/zh-CN/permission-modes.md#server-side-classifier-review)。

## 相关资源

* [Auto mode](https://code.claude.com/docs/zh-CN/permission-modes.md#eliminate-prompts-with-auto-mode)：什么是 auto mode 以及它默认阻止的内容
* [Server-side classifier review](https://code.claude.com/docs/zh-CN/permission-modes.md#server-side-classifier-review)：哪些会话要求服务器检查操作，以及每个会话所需的 Claude Code 版本
* [Gateway compatibility guide](https://code.claude.com/docs/zh-CN/llm-gateway-protocol.md#feature-pass-through)：当网关删除请求头或正文字段时会出现什么问题
* [The server returned no safety verdict](https://code.claude.com/docs/zh-CN/errors.md#the-server-returned-no-safety-verdict)：当服务器对操作没有给出判决时您看到的拒绝
* [Manage costs effectively](https://code.claude.com/docs/zh-CN/costs.md)：跟踪令牌使用情况并降低 Claude Code 成本
