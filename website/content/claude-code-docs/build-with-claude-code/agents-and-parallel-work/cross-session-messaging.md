> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# 消息传递到您的其他 Claude Code 会话

> 让 Claude 列出并消息传递到您在此机器上的其他 Claude Code 会话，并到达您在其他机器或网络上的会话。

> [!NOTE]
>
> 跨会话消息传递需要 macOS 和 Linux 上的 Claude Code v2.1.224 或更高版本，包括 WSL 2 内的 Linux。在原生 Windows 上，它需要 Claude Code v2.1.234 或更高版本。当会话满足要求时，消息传递功能默认启用，无需任何配置。请参阅[可用性](#availability)了解提供商要求以及如何确认会话具有此功能。

跨会话消息传递让 Claude 能够将消息从您的一个 Claude Code 会话传递到另一个。当一个会话中的更改破坏了另一个会话正在构建的内容时，Claude 可以在您注意到之前警告该会话。当一个会话解决了另一个会话被阻止的问题时，Claude 可以跨会话发送答案。

消息是一个 Claude 写给另一个 Claude 的文本片段，永远不包括发送者的对话历史或文件。要移动整个对话或其上下文，请[恢复会话](https://code.claude.com/docs/zh-CN/sessions.md#resume-a-session)。

## 何时使用跨会话消息传递

当您的一个会话有另一个会话在任务中期需要的内容时，使用消息传递。Claude 可以在看到需要时自动发送消息，例如在进行影响另一个会话正在进行的工作的更改后，或者您可以要求它发送一条。常见情况包括：

* **移交发现**：当一个会话发现破坏性更改或做出决定时，Claude 为处理受影响区域的会话总结它，而不是您在那里重新解释。
* **协调并行 worktrees**：当会话在单独的 [worktrees](https://code.claude.com/docs/zh-CN/worktrees.md) 中处理同一存储库时，Claude 可以告诉其他会话已合并的内容。
* **获取长期运行工作的状态**：让迁移或测试运行报告回您正在观看的会话，或从那里自己询问。如果该会话在此机器上，Claude 还可以[在它下次空闲或退出时要求一条通知](#get-a-notice-when-another-session-goes-idle)。
* **跨机器发送消息**：到达您在另一台机器或网络上的一个会话。

## 向另一个会话发送消息

当你的一个会话学到另一个会话需要的东西时，比如一个发现、一个状态或一个决定，Claude 会将其传递过去，而不是让你在终端之间复制粘贴。Claude 使用 `ListAgents` 发现目标，并使用 `SendMessage` 发送，所以你永远不需要自己调用这两个工具。Claude 可以在没有被要求的情况下决定发送消息，你也可以提示它发送一条消息。

要自己提示一条消息，告诉 Claude 你想让另一个会话知道或做什么。这个例子是你输入的提示，而不是 Claude 发送的消息：

```text wrap theme={null}
Ask the session running in my other terminal whether the migration finished
```

Claude 会自己编写实际的消息，所以你的提示可以将内容留给 Claude。这个提示要求一个摘要而不指定其措辞，Claude 发送的内容会有所不同：

```text wrap theme={null}
Explain what we just did to the session working on the payments API
```

要自己命名目标，在你的提示中提及会话：输入 `@` 后跟会话名称的首字母，然后从类型提示中选择会话，方式与 [@-提及子代理](https://code.claude.com/docs/zh-CN/sub-agents.md#invoke-subagents-explicitly) 相同。需要 Claude Code v2.1.232 或更高版本。Claude Code 会插入提及，例如 `@api-worker`，并告诉 Claude 它命名的是哪个会话，这样 Claude 可以向该会话发送消息而无需先列出你的会话。这个提示用提及命名目标：

```text wrap theme={null}
Let @api-worker know the schema migration finished
```

类型提示列出你在这台机器上的其他活跃会话。两种情况需要超过名称的首字母：

* **这台机器之外的会话**：云会话或远程控制会话仅在 Claude 列出或向你这台机器之外的会话发送消息后才会出现在类型提示中，所以请先要求 Claude 列出它们。
* **包含空格或字母、数字、连字符和下划线以外字符的名称**：用双引号输入，例如 `@"release notes"`。当你从类型提示中选择会话时，Claude Code 会为你插入引号。

你也可以不使用选择器直接输入提及。当多个活跃会话响应提及的名称时，Claude 会在发送前询问你指的是哪一个。

关于 Claude 编写的消息到达时的样子，包括一个例子，请参见 [消息看起来像什么](#what-a-message-looks-like)。

### 消息传递

接收 Claude 在活跃轮次中的工具调用之间读取消息，所以运行中的工具永远不会被中断。当接收会话处于空闲状态时，Claude Code 会用消息启动新的轮次。

来自另一个会话的消息以纯文本形式到达。如果它用 `@` 提及文件或 [MCP 资源](https://code.claude.com/docs/zh-CN/mcp.md#use-mcp-resources)，Claude 会看到按原样写的提及，Claude Code 不会附加任何内容，无论消息是启动新轮次还是在现有轮次中到达。Claude 仍然可以用自己的工具在接收机器上打开提及的路径，受该会话的权限限制。

Claude Code 在以下情况下拒绝消息：

* 消息 [超过大小限制](#limitations)。Claude Code 在发送会话中拒绝它，在它离开之前。
* 对这台机器上的会话的快速突发已达到 [该会话的收件箱接受的内容](#limitations)。Claude Code 拒绝向该会话发送进一步的消息。
* 这台机器之外的某个会话被[列为无法接收跨会话消息](#message-sessions-on-other-machines)。Claude Code 会在发送会话中、消息离开这台机器之前拒绝它。
* 这台机器上的回复目标未通过安全检查，例如符号链接目标。[拒绝发送跨会话消息](https://code.claude.com/docs/zh-CN/errors.md#refusing-to-send-a-cross-session-message) 列出了这些检查。

接收会话根据其自己的 [入站控制](#control-inbound-messages) 检查每条到达的消息，检查以三种结果之一结束：

* **已传递**：Claude Code 将消息传递给接收 Claude。
* **已保留**：Claude Code 将消息搁置未传递。保留的消息仅在你批准它或稍后的模式或设置更改允许它时才到达 Claude。
* **已拒绝**：Claude Code 丢弃消息而不传递它。

消息传递后，会像您输入的提示词一样计入[使用量](https://code.claude.com/docs/zh-CN/costs.md)。接收方 Claude 可以使用 `SendMessage` 回复发送者，但[来自其他机器和云端会话的回复](#replies-from-other-machines-and-cloud-sessions)和[关闭跨会话消息](#turn-off-cross-session-messaging)中所述的情况除外。

权限边界保持按会话。Claude 被指示永远不要要求另一个会话执行在其自己的会话中被拒绝或阻止的操作，或其自己的权限设置会阻止的操作，而是将该工作路由回你。在接收端，[接收会话自己的权限提示和规则仍然适用](#how-a-session-treats-an-incoming-message) 于消息要求的任何内容。

### 当另一个会话变为空闲时获得通知

Claude 可以要求你在这台机器上的一个会话在该会话下一次变为空闲或退出时发回一条通知。这里的空闲意味着会话完成了一个轮次，没有任何排队的内容。当你在另一个会话中等待长任务并想听到它完成时而不是检查时使用它。需要两个会话中都有 Claude Code v2.1.236 或更高版本。

#### 请求通知

告诉 Claude 你在等什么。这个提示要求来自迁移会话的通知：

```text wrap theme={null}
Tell me when the migration session finishes what it's working on
```

Claude 使用 `SendMessage` 工具的 `notify_when_idle` 输入订阅，要么附加到它正在发送的消息，要么单独订阅。单独订阅时，Claude Code 订阅而不在被监视的会话中启动轮次或花费令牌，如果该会话已经空闲，会立即发送通知。附加到消息时，Claude Code 先传递消息，然后稍后发送通知。

#### 每个会话显示什么

被监视的会话显示一行说另一个进程要求在会话下一次空闲时被告知。要求会话显示通知为一行命名被监视的会话。该行可以包括该会话轮次完成的时间和该轮次的单行状态。如果要求会话处于空闲状态，Claude Code 会用通知启动新的轮次。

#### 限制

如果在 12 小时内没有通知到达，Claude Code 会删除订阅并告诉 Claude，所以它不会继续等待。

每一方的 [入站控制](#control-inbound-messages) 适用于像消息一样的通知：

* **任一方的 `refuse`**：什么都不会到达。被监视的会话删除请求而不记录或回答它，所以订阅在 12 小时后无答复过期，具有 `refuse` 的要求会话永远不会订阅。
* **任一方的 `hold`**：通知到达时内容较少。被监视的会话省略单行状态，要求会话在你的记录中显示通知而不将其传递给 Claude。

只有你主要对话中的 Claude 可以订阅，并且仅订阅你在这台机器上的会话。当 Claude 要求来自任何其他目标的通知时，例如队友、子代理或这台机器之外的会话，Claude Code 拒绝整个调用，包括附加到它的任何消息。

### 查看 Claude 可以到达的会话

Claude 自己找到消息的目标，所以你不需要在要求它发送之前运行任何东西。要自己查看 Claude 可以到达的会话，运行 `/list-agents` 命令。第一行（如果存在）是这个会话自己的名称，你的其他会话用来向它发送消息的名称。下面的行是 Claude 可以到达的会话：

* **子代理**：在当前会话内运行的代理。
* **队友**：这个会话自己的 [代理团队](https://code.claude.com/docs/zh-CN/agent-teams.md) 队友。
* **你的其他本地会话**：在同一台机器上运行的 Claude Code 会话，包括 [后台会话](https://code.claude.com/docs/zh-CN/agent-view.md)。会话仅在绑定 [收件箱套接字](#the-sessions-inbox-socket) 时出现。
* **你的 [云会话](https://code.claude.com/docs/zh-CN/claude-code-on-the-web.md)**：在此会话连接到 [远程控制](https://code.claude.com/docs/zh-CN/remote-control.md) 时显示。
* **你在其他机器上的远程控制会话**：在此会话连接到 [远程控制](https://code.claude.com/docs/zh-CN/remote-control.md) 时显示，并标记为 `Remote Control`。Claude Code 显示 `offline` 作为远程控制连接已断开的会话的状态。

当此会话连接到 [远程控制](https://code.claude.com/docs/zh-CN/remote-control.md) 时，Claude Code 从 `/list-agents` 输出中隐瞒你的本地会话的一些详细信息，而不改变 Claude 本身在寻找要向其发送消息的会话时看到的内容：

* **工作目录**：它省略每个本地会话的工作目录。
* **会话名称**：它省略任何它不能归属于某个人的会话名称，所以没有名称的行读作 `(unnamed session)`。
* **第一行**：它省略包含此会话自己名称的行，除非你在此终端用 `--name` 或 `/rename` 和名称输入了该名称，自从你启动或最后恢复会话以来。

当输出列出任何内容时，它以一条说明详细信息被隐瞒的注释结束。在会话自己的键盘上运行 `/rename` 后跟未使用的名称会给该会话一个出现在输出中的名称。

会话响应你用 [`/rename`](https://code.claude.com/docs/zh-CN/commands) 命令或 [`--name`](https://code.claude.com/docs/zh-CN/cli-reference#cli-flags) 标志设置的名称。当你不设置一个时，Claude Code 自己命名会话。对于交互式会话，这是在 [运行会话的列表](https://code.claude.com/docs/zh-CN/sessions.md#name-your-sessions) 中显示的名称。

除非此会话连接到 Remote Control，否则 Claude Code 会在 `/list-agents` 输出中显示每个本地会话的工作目录，以便您在同名会话运行于不同目录时区分它们。Claude 会根据有多少活跃会话对应该名称，以以下两种方式之一指定消息地址：

* **一个会话响应该名称**：Claude Code 仅在名称上传递消息。
* **多个会话共享该名称，或 Claude Code 无法检查你的会话运行的所有地方**：Claude 为其列表的每一行添加一个短标识符，并在地址中使用该标识符。

### 向其他机器上的会话发送消息

消息如何传播，以及它是否通过 Anthropic 服务器，取决于目标会话运行的位置：

| 其他会话运行的位置 | 消息如何传播 |
| :- | :- |
| 在这台机器上 | 在 macOS 和 Linux 上通过每个会话的套接字，或在本机 Windows 上通过每个会话的命名管道，永远不通过 Anthropic 服务器 |
| 在你的另一台机器上 | 通过 Anthropic 服务器，通过该机器的 [远程控制](https://code.claude.com/docs/zh-CN/remote-control.md) 连接到达 |
| 在 [云中](https://code.claude.com/docs/zh-CN/claude-code-on-the-web.md) | 通过 Anthropic 服务器，直接到云会话 |

启动与你另一台机器上的会话的对话需要 Claude Code v2.1.225 或更高版本和一个 [出现在列表中](#see-which-sessions-claude-can-reach) 的目标。

会话在[列表](#see-which-sessions-claude-can-reach)中的行可能显示某种状况，这会影响 Claude 向该会话发送消息时的结果：

* **`offline`**：该会话的 Remote Control 连接已断开。消息可以发出，但仅在该会话的机器重新连接后才会到达。
* **`can't receive cross-session messages (off in that session)`**：该会话中的消息功能[不可用](#availability)，或其 [`crossSessionInbound`](https://code.claude.com/docs/zh-CN/settings-reference#crosssessioninbound) 值为 `refuse`。Claude Code 会在消息离开这台机器之前拒绝发往该会话的消息。Claude 的 `SendMessage` 调用返回的结果以 `Not sent` 开头并说明原因。在该会话中解决问题后，之后的列表将不再显示此状况，Claude 即可向该会话发送消息。

容器内的会话和主机上的会话无法相互到达。同一容器内的两个会话仍然可以相互发送消息，包括在 [自托管运行器](https://code.claude.com/docs/zh-CN/self-hosted-environments) 上。WSL 2 内的会话和同一计算机上的本机 Windows 会话也无法相互到达。

要在任何消息超出这台机器之前要求你的批准，设置 [`isolatePeerMachines`](#require-approval-for-cross-machine-messages)。

#### 来自其他机器和云端会话的回复

[云端会话](https://code.claude.com/docs/zh-CN/claude-code-on-the-web.md)（包括在[自托管运行器](https://code.claude.com/docs/zh-CN/self-hosted-environments)上运行的云端会话）无法使用 `SendMessage` 回复来自另一台机器的消息，即使该消息带有[回复地址](#what-a-message-looks-like)也不行。消息仍然会到达。您可以在 [claude.ai/code](https://claude.ai/code) 上该云端会话的会话记录中查看其回复。

如果 Claude 向这台机器之外的会话发送消息时，此会话未连接到 Remote Control，消息仍然会发出，但不带[回复地址](#what-a-message-looks-like)。接收方 Claude 无法回复该消息。

## 会话如何处理传入消息

当会话 A 向会话 B 发送消息时，Claude Code 会告诉 B 的 Claude 该消息来自另一个会话，而不是来自您，并限制该消息可以做什么：

* **它不能批准任何内容**：来自另一个会话的消息永远不会被视为您的同意，因此它不能代表您回答待处理的权限提示。
* **它不能更改配置**：Claude Code 指示接收方 Claude 永远不要因为另一个会话的要求而更改权限设置、`CLAUDE.md` 或其他配置。
* **命令不会运行**：消息文本中的命令（如 `/compact`）以纯文本形式到达。Claude Code 永远不会执行它。
* **权限提示仍会触发**：如果处理该消息需要接收会话不具备的权限，您会看到与任何其他工作相同的提示。

### 消息的样子

当消息到达时，Claude Code 会在对话中将其显示为暗淡的单行预览，该预览行之后会保留在对话中。预览包含发送者的名称和消息的第一行，消息较长时会用 `…` 截断，例如 `› Message from @api-worker: Schema migration finished (ctrl+o to expand)`。

以下任一方式都会向您显示完整文本：

* 按 `Ctrl+O` 打开[会话记录查看器](https://code.claude.com/docs/zh-CN/interactive-mode.md#transcript-viewer)，并在发送者的会话名称下阅读完整文本。

  如果 Claude 正忙且尚未收到该消息，请滚动到会话记录的末尾，该消息会列在 `Waiting to be delivered` 下。需要 Claude Code v2.1.309 或更高版本。

* 在[全屏渲染](https://code.claude.com/docs/zh-CN/fullscreen.md#use-the-mouse)中，点击未完整显示消息的预览行，即可将其就地展开。

* 在使用 [`--verbose`](https://code.claude.com/docs/zh-CN/cli-reference#cli-flags) 启动的会话中，Claude Code 会显示完整文本而不是预览。

预览仅缩短您看到的内容。无论您是否展开它，Claude 都会读取完整消息。

Claude 接收消息时会附带发送者的名称和回复地址，但[单向跨机器消息](#message-sessions-on-other-machines)除外，它不携带回复地址。

以下示例是一个 Claude 写给另一个 Claude 的消息，展示了展开后的完整文本：

```text wrap theme={null}
Schema migration finished
The new column is tenant_id, and rebasing on main is safe now.
```

### 控制入站消息

设置 [`crossSessionInbound`](https://code.claude.com/docs/zh-CN/settings-reference#crosssessioninbound) 以选择会话如何处理来自您其他会话的消息：

| 值 | 行为 |
| :- | :- |
| `accept` | Claude Code 将每条消息传递给 Claude |
| `hold` | Claude Code 为每条消息显示通知，但不传递它。如果之后根据[优先级规则](https://code.claude.com/docs/zh-CN/settings-reference.md#crosssessioninbound)适用 `accept`，Claude Code 会释放被保留的消息 |
| `refuse` | Claude Code 丢弃每条消息而不传递它 |

除了编辑设置文件，您还可以在 `/config` 的 **Messages from your other sessions** 行中选择该值。Claude Code 会将您选择的值写入您的用户设置。该行需要 Claude Code v2.1.232 或更高版本，并且当托管设置或 `--settings` 标志设置了该设置项时不会出现，因为此时用户设置中的值不会生效。对于此设置项，Claude Code 会拒绝 `/config crossSessionInbound=value` 简写形式。

要查看哪个值适用，请遵循[设置参考](https://code.claude.com/docs/zh-CN/settings-reference.md#crosssessioninbound)中的 `crossSessionInbound` 优先级规则。

当没有值适用时，Claude Code 会根据两个会话的权限模式逐条消息做出决定。它将[绕过权限提示](https://code.claude.com/docs/zh-CN/permission-modes.md#skip-all-checks-with-bypasspermissions-mode)的会话归为一类，将其他所有会话归为另一类。在可使用绕过权限的交互式终端会话中，计划模式被视为绕过，而 [auto](https://code.claude.com/docs/zh-CN/permission-modes.md#eliminate-prompts-with-auto-mode)、`acceptEdits` 和 `dontAsk` 被视为提示：

* **接收会话会提示权限**：Claude Code 传递每条消息。仅当发送会话将自身标识为绕过权限提示时，它才会保留该消息以等待您的批准。
* **接收会话绕过权限提示**：Claude Code 保留每条消息以等待您的批准。仅当发送会话也将自身标识为绕过时，它才会传递该消息。

当默认行为在交互式终端会话中保留消息时，Claude Code 会在该会话中打开批准对话框。对话框显示发送者和预览：

* **Approve** 将该条消息传递给 Claude。
* **Deny** 或关闭对话框会丢弃该消息。
* 当对话框在 [`dialogExpiry`](https://code.claude.com/docs/zh-CN/settings-reference#dialogexpiry) 截止时间后仍未得到回应时，Claude Code 会关闭它并丢弃消息。截止时间默认为五分钟。
* 当没有终端连接到[后台会话](https://code.claude.com/docs/zh-CN/agent-view.md)时，Claude Code 会让对话框在截止时间之后保持打开。在您连接后，如果对话框在一个完整的截止时间段内仍未得到回应，Claude Code 会关闭它并丢弃消息。
* 如果在消息被保留期间此会话的权限模式类别发生变化，Claude Code 会重新应用入站规则，传递现在被接受的消息，并显示通知。

VS Code 扩展或 Desktop 应用中的会话无法显示该对话框。在这些会话中，Claude Code 会将被保留的消息保留到相同的截止时间，如[非交互式会话](#non-interactive-sessions)中所述。

Claude Code 最多保留 100 条消息，超出后会丢弃最旧的消息。

### 非交互式会话

Claude Code 会像交互式会话一样为 [`claude -p`](https://code.claude.com/docs/zh-CN/headless) 会话绑定收件箱套接字，因此长期运行的 `-p` 工作进程可以接收消息并出现在列表中。当您在 [bare 模式](https://code.claude.com/docs/zh-CN/headless.md#start-faster-with-bare-mode)下启动会话时，Claude Code 不会绑定套接字，因此该会话无法接收消息，也不会出现在 Agent 列表中。

`-p` 会话无法显示批准对话框。当[入站默认行为](#control-inbound-messages)在此类会话中保留消息时，Claude Code 会将其保留到对话框所使用的相同 [`dialogExpiry`](https://code.claude.com/docs/zh-CN/settings-reference#dialogexpiry) 截止时间，默认为五分钟：

* **截止时间之前**：如果模式或设置的更改允许该消息，Claude Code 会传递它。
* **截止时间之后**：Claude Code 会丢弃该消息，并向其能够联系到的发送者报告该消息已过期。

将 `dialogExpiry` 设置为 `"never"`，可将默认暂挂的消息保留到会话结束。由显式 `hold` 设置暂挂的消息不会过期；当之后有 `accept` 生效时，Claude Code 会投递该消息。

要让 `-p` 工作进程在无人值守的情况下接收消息，请在其 `--settings` 值中将 `crossSessionInbound` 设置为 `accept` 来启动它。在您的用户设置中设置 `accept` 也有效，但会应用于您运行的每个会话。

### 会话的收件箱套接字

当您预期的会话不在 Agent 列表中、当您希望脚本或 hook 向会话发布消息，或者当沙箱中的命令无法访问套接字时，请阅读本节。

Claude Code 会为每个启用了跨会话消息传递的会话绑定一个收件箱套接字，同一机器上的其他会话通过它传递消息。在 macOS 和 Linux（包括 WSL 2 内的 Linux）上，该套接字是 Unix 域套接字；在原生 Windows 上则是命名管道。关于哪些类型的会话会绑定套接字，请参阅[非交互式会话](#non-interactive-sessions)。

您可以在两个地方找到套接字的路径：

* `/status` 在 `Peer address` 行中显示它。路径以 `uds:` 为前缀。
* Claude Code 会将其作为 [`CLAUDE_CODE_MESSAGING_SOCKET`](https://code.claude.com/docs/zh-CN/env-vars#variables) 环境变量导出给 [hook](https://code.claude.com/docs/zh-CN/hooks.md) 和 Bash 命令：
  * 在启动时即开启消息传递的会话中，Claude Code 会在任何 hook 运行之前导出该变量，包括 `SessionStart`。

在 macOS 和 Linux 上，Claude Code 将套接字限制为仅供您的操作系统用户使用。在原生 Windows 上，它改为要求每个连接首先使用只有您的操作系统用户才能读取的密钥进行身份验证。无论哪种方式，在共享机器上，其他用户的会话都无法向其传递消息。

在 macOS 和 Linux 上，Claude Code 还会拒绝在它无法接受的目录（例如由其他用户拥有的目录）中创建套接字，而是改用私有的每用户目录 `/tmp/cc-socks-<uid>`。当它无法接受任何目录时，会话将在没有收件箱的情况下运行：Claude Code 显示通知，`/status` 在其 `Peer address` 行中显示 `unavailable` 及原因，[`--debug`](https://code.claude.com/docs/zh-CN/cli-reference#cli-flags) 日志会记录完整的拒绝信息。

除了套接字的路径，Claude Code 还会将每个会话的令牌导出为 [`CLAUDE_CODE_MESSAGING_TOKEN`](https://code.claude.com/docs/zh-CN/env-vars#variables)。向其自身会话的套接字发布消息的脚本可以将 `{"type":"auth","token":"<token>"}` 作为其连接的第一行发送，其中 `<token>` 是 `CLAUDE_CODE_MESSAGING_TOKEN` 的值。Claude Code 是否要求该行取决于平台：

* **macOS 和 Linux，包括 WSL 2**：该行是可选的。无论是否包含该行，Claude Code 都会接受连接。
* **原生 Windows**：该行是必需的。Claude Code 会关闭第一行不是有效身份验证行的任何连接，并且不会传递来自该连接的任何内容。

仅在您要发布的消息准备就绪时才打开连接。Claude Code 会关闭在 30 秒内未发送完整一行的连接，因此请先捕获慢速命令的输出，然后再打开连接发送它。

对于通过套接字到达的消息，Claude Code 会应用与任何其他对等消息相同的[入站控制](#control-inbound-messages)，但有一个例外和一个前提条件：

* **自身子进程的消息**：当没有 `crossSessionInbound` 值适用时，Claude Code 会传递经其验证来自会话自身子进程的消息，例如 hook 或 Bash 命令向其自身会话的套接字回发的消息。
  * 在 Linux 上（包括 WSL 2 内），即使子进程已经退出，Claude Code 也可以通过进程证据进行验证。在 macOS 上，它只能在发布进程仍在运行时以这种方式验证；而在 Claude Code 以进程 ID 1 运行的容器中，它完全没有进程证据。在原生 Windows 上它也没有进程证据。
  * 在 macOS 上发布进程退出之后，以及在 Claude Code 以进程 ID 1 运行的容器中，该进程证据缺失，Claude Code 改为验证在打开其连接的身份验证行中发送了会话所导出的 [`CLAUDE_CODE_MESSAGING_TOKEN`](https://code.claude.com/docs/zh-CN/env-vars#variables) 的子进程。在原生 Windows 上，该令牌是 Claude Code 验证自身子进程消息的唯一方式。
  * 当 Claude Code 无法通过任何一种方式验证时，它会像对待任何其他未声明权限类别的消息一样对待该消息，因此绕过权限提示的会话会保留它以等待您的批准。
* **沙箱中的会话**：使用沙箱的 Unix 套接字设置 [`sandbox.network.allowAllUnixSockets` 和 `sandbox.network.allowUnixSockets`](https://code.claude.com/docs/zh-CN/settings-reference#sandbox-settings) 控制 Bash 命令能否从[沙箱](https://code.claude.com/docs/zh-CN/sandboxing.md)内部访问套接字。

## 限制跨会话消息传递

除了每条消息的默认值，您可以通过两种方式缩小消息传递。在任何消息离开机器之前要求您的批准，或为会话或组织关闭消息传递。

### 要求批准跨机器消息

设置 [`isolatePeerMachines`](https://code.claude.com/docs/zh-CN/settings-reference#isolatepeermachines) 为 `true` 以要求您的明确批准，在任何 `SendMessage` 到达超出此机器的会话之前：

```json theme={null}
{
  "isolatePeerMachines": true
}
```

设置此项后，Claude Code 在 Claude 的消息到达超出此机器的会话之前要求您的批准，即使在 `bypassPermissions` 模式中，它跳过普通权限提示。任何设置范围中的 `true` 适用，因此检查的项目文件可以打开要求但不关闭。Claude Code 不提示同一机器上的会话之间的消息。

### 关闭跨会话消息传递

接收和发送是单独的控制，因此关闭您需要的任何方向，或两者。对到达的消息使用 `crossSessionInbound`，对 Claude 可以发送或列出的内容使用权限规则：

* **停止接收**：设置 `crossSessionInbound` 为 `refuse`，Claude Code 删除入站对等消息而不传递它们。从项目或本地设置，`refuse` 适用于每个其他来源，从您的用户设置它适用，除非托管设置或 `--settings` 标志设置值。
* **停止发送和列出**：添加[权限拒绝规则](https://code.claude.com/docs/zh-CN/permissions.md#tool-specific-permission-rules)命名 `SendMessage` 和 `ListAgents`。两者都采用没有说明符的裸工具名称。

管理员可以在[托管设置](https://code.claude.com/docs/zh-CN/managed-settings.md)中为组织关闭两个方向，结合拒绝规则与 `refuse`：

```json theme={null}
{
  "permissions": {
    "deny": ["SendMessage", "ListAgents"]
  },
  "crossSessionInbound": "refuse"
}
```

设置此项后，Claude Code 仍然为每个会话绑定收件箱套接字，但删除到达它的每条消息而不向 Claude 传递任何内容。拒绝 `SendMessage` 也删除向子代理和代理团队队友的消息传递，因为相同的工具服务两者。拒绝的会话在其自己的 `/status` 或同一机器上其他会话的列表中显示无可见更改，因此要确认它，检查适用于该会话的设置文件而不是其状态。

## 可用性

跨会话消息传递需要 macOS、Linux 和 WSL 2 上的 Claude Code v2.1.224 或更高版本，以及原生 Windows 上的 v2.1.234 或更高版本。可用性以及 Claude 可以向其发送消息的会话也取决于您的操作系统、提供商和配置：

* **操作系统**：在 macOS、Windows 和 Linux 上可用，包括 WSL 2 内的 Linux。

* **此机器上的会话**：在每个提供商上可用，包括 Amazon Bedrock、Claude Platform on AWS、Google Cloud 的 Agent Platform 和 Microsoft Foundry，以及在[功能标志获取](https://code.claude.com/docs/zh-CN/env-vars.md#features-that-need-feature-flag-fetching)关闭的会话中运行。在这些提供商上，以及标志获取关闭时，同机器消息传递需要 Claude Code v2.1.248 或更高版本。Claude Code 通过您机器上的[每个会话套接字](#the-sessions-inbox-socket)传递这些消息，永远不通过 Anthropic 服务器。

  要停止会话接收它们，设置 [`crossSessionInbound`](#turn-off-cross-session-messaging) 为 `refuse`。

* **超出此机器的会话**：Claude 从连接到远程控制的会话找到您的[网络上的 Claude Code](https://code.claude.com/docs/zh-CN/claude-code-on-the-web.md) 会话和您在其他机器上的会话，这需要 claude.ai 登录作为此会话的活跃身份验证和其他[远程控制要求](https://code.claude.com/docs/zh-CN/remote-control.md#requirements)。Claude 无法在 Amazon Bedrock、Claude Platform on AWS、Google Cloud 的 Agent Platform 和 Microsoft Foundry 上使用 API 密钥或找到这些会话。

要检查会话，输入 `/list-agents`，也可用作 `/peers`。结果将没有该功能的会话与更窄的东西阻止消息的会话分开，如缺少 `SendMessage` 工具或拒绝的发送：

* **`/list-agents` 无法识别**：会话没有跨会话消息传递。通过上面的要求工作，从 `claude --version` 开始获取版本要求。
* **`/list-agents` 有效但发送未到达**：消息传递已启用，适用的是范围更窄的原因：
  * **拒绝规则**：[权限拒绝规则](#turn-off-cross-session-messaging)删除 `SendMessage` 和 `ListAgents` 工具。
  * **入站控制**：[接收会话的入站控制](#control-inbound-messages)可以保留或删除您发送给它的内容。
  * **云端会话缺失**：云端会话仅在此会话连接到 [Remote Control](https://code.claude.com/docs/zh-CN/remote-control.md) 时出现。
  * **其他机器会话缺失**：您另一台机器上的会话仅在它使用 [Remote Control](https://code.claude.com/docs/zh-CN/remote-control.md) 运行且此会话也已连接时出现。
  * **其他机器会话 `offline`**：向列为 `offline` 的会话发送的消息会通过，但[仅在该会话的机器重新连接后到达](#message-sessions-on-other-machines)。
  * **云端或其他机器会话 `can't receive cross-session messages`**：向列有此状态的会话发送的消息[不会离开此机器](#message-sessions-on-other-machines)，并且 `SendMessage` 下的结果以 `Not sent` 开头。
  * **较旧的云端或其他机器会话缺失**：Claude Code 按从新到旧的顺序读取这些会话列表，并在有限数量的页面后停止，因此 Claude 无法按名称向超出这些页面的会话发送消息。

在具有消息传递的会话中，`/status` 也显示 `Peer address` 行，带有会话自己的收件箱地址，或 `unavailable` 和原因，当 Claude Code [无法设置收件箱](#the-sessions-inbox-socket)时。

## 限制

这里的限制是消息传递通道本身的属性，在该功能运行的任何地方适用。对于平台和提供商差距，请改为参阅[可用性](#availability)。

* **仅纯文本**：Claude 仅在会话之间发送纯文本。结构化[代理团队](https://code.claude.com/docs/zh-CN/agent-teams.md)协议消息保留在团队内。
* **同机器消息大小有上限**：Claude Code 拒绝到此机器上的会话的消息，一旦其序列化形式超过约一百万个字符。拒绝[命名确切大小](https://code.claude.com/docs/zh-CN/errors.md#message-too-large-for-cross-session-delivery)。什么都不到达接收会话。
* **对一个会话的快速突发在发送者处被拒绝**：一旦对此机器上的会话的快速突发消息达到该会话的收件箱接受的内容，Claude Code 拒绝发送会话中的进一步发送。[拒绝命名突发](https://code.claude.com/docs/zh-CN/errors.md#too-many-messages-to-this-session-just-now)并告诉 Claude 将其余的批处理为一条消息或等待。
* **消息循环被限制**：在接收会话中，Claude Code 对每个发送者的重复消息进行速率限制，删除在短窗口内到达的相同重复，并最多为 Claude 读取排队 50 条接受的消息。因此两个会话之间的消息循环自己停止。

## 相关资源

* [子代理](https://code.claude.com/docs/zh-CN/sub-agents.md#resume-subagents)和[代理团队](https://code.claude.com/docs/zh-CN/agent-teams.md#messages-between-agents)：单个会话或团队内的消息传递
* [后台代理](https://code.claude.com/docs/zh-CN/agent-view.md)：分派和监控您可能向其发送消息的并行会话
* [远程控制](https://code.claude.com/docs/zh-CN/remote-control.md)：连接此会话以到达您在其他机器上的会话
* [频道](https://code.claude.com/docs/zh-CN/channels.md)：将外部事件（如 CI 结果或聊天消息）推送到会话中
* [设置](https://code.claude.com/docs/zh-CN/settings-reference.md#all-settings)：`crossSessionInbound`、`isolatePeerMachines` 和 `dialogExpiry`
* [权限模式](https://code.claude.com/docs/zh-CN/permission-modes.md)：入站默认的两个类背后的模式
* [工具参考](https://code.claude.com/docs/zh-CN/tools-reference.md)：工具表中的 `ListAgents` 和 `SendMessage` 行
* [并行运行代理](https://code.claude.com/docs/zh-CN/agents.md)：比较 Claude Code 运行多个代理的方式
