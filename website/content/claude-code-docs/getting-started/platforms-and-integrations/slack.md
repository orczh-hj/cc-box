> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Slack 中的 Claude Code

> 从 Slack 委派编码任务。此早期版本仅在未连接到 Claude Tag 的工作区中回应来自 Pro、Max 和 Ultra 账户的提及。

> [!WARNING]
>
> 此早期版本的 Slack 中的 Claude Code 仅回应来自 Pro、Max 和 Ultra 账户的频道 @提及，并且仅在尚未有任何组织将其连接到 [Claude Tag](https://claude.com/product/tag) 的 Slack 工作区中回应。每个会话都以单个用户的账户运行。Claude Tag 以您组织的共享身份运行 @Claude，具有管理员配置的访问权限。
>
> * **Pro、Max 和 Ultra 计划：** Claude Tag 在个人计划上不可用，因此请在未连接到 Claude Tag 的工作区中[设置此版本](#setting-up-claude-code-in-slack)。
> * **Team 和 Enterprise 计划：** 您现有的 Slack 应用和 @Claude 用户名保持不变。为新工作区[设置 Claude Tag](https://claude.com/docs/claude-tag/overview)；要迁移已经使用此版本的工作区，请参阅[从早期 Claude in Slack 迁移](https://claude.com/docs/claude-tag/admins/migrate-from-earlier)。
> * **收到通知而非回答：** 请查阅 @Claude 回复的[设置通知](#this-workspace-isnt-set-up-for-claude-tag-yet)或[停用通知](#the-legacy-claude-in-slack-bot-is-retired)。

Slack 中的 Claude Code 将 Claude Code 的强大功能直接引入您的 Slack 工作区。当您使用编码任务提及 `@Claude` 时，Claude 会自动检测意图并创建 Claude Code 云端会话，允许您在不离开团队对话的情况下委派开发工作。

此集成基于现有的 Claude for Slack 应用程序构建，但为与编码相关的请求添加了到 Claude Code 云端会话的智能路由。每个会话在您自己的 Claude 账户下运行，使用您连接的仓库和您的计划限制。

## 用例

* **Bug 调查和修复**：要求 Claude 在 Slack 频道中报告 Bug 时立即调查和修复。
* **快速代码审查和修改**：让 Claude 根据团队反馈实现小功能或重构代码。
* **协作调试**：当团队讨论提供关键背景信息（例如错误重现或用户报告）时，Claude 可以使用该信息来指导其调试方法。
* **并行任务执行**：在 Slack 中启动编码任务，同时继续其他工作，完成时收到通知。

## 前提条件

在使用 Slack 中的 Claude Code 之前，请确保您具有以下条件：

| 要求 | 详情 |
| :- | :- |
| Claude 计划 | Pro、Max 或 Ultra |
| Slack 工作区 | 未被任何组织连接到 [Claude Tag](https://claude.com/docs/claude-tag/overview)。如果 @Claude 回复了[停用通知](#the-legacy-claude-in-slack-bot-is-retired)，则说明该工作区已被连接 |
| 云端会话 | [云端会话](https://code.claude.com/docs/zh-CN/claude-code-on-the-web.md)已为您的账户启用 |
| GitHub 账户 | 在 [claude.ai/code](https://claude.ai/code) 连接，至少有一个仓库已通过身份验证 |
| Slack 身份验证 | 您的 Slack 账户通过 Claude 应用程序链接到您的 Claude 账户 |

## 在 Slack 中设置 Claude Code

**1. 在 Slack 中安装 Claude 应用程序**

工作区管理员必须从 Slack 应用程序市场安装 Claude 应用程序。访问 [Slack 应用程序市场](https://slack.com/marketplace/A08SF47R6P4)并单击"Add to Slack"开始安装过程。

**2. 连接您的 Claude 账户**

安装应用程序后，认证您的个人 Claude 账户：

1. 通过单击您的应用程序部分中的"Claude"在 Slack 中打开 Claude 应用程序
2. 打开应用程序主页选项卡
3. 单击"Connect"将您的 Slack 账户与您的 Claude 账户链接
4. 在浏览器中完成认证流程

**3. 配置云会话**

确保为您的账户正确配置云会话：

* 访问 [claude.ai/code](https://claude.ai/code) 并使用您连接到 Slack 的同一账户登录
* 如果尚未连接，请连接您的 GitHub 账户
* 认证至少一个您希望 Claude 使用的存储库

**4. 选择您的路由模式**

连接您的账户后，配置 Claude 如何在 Slack 中处理您的消息。打开 Slack 中的 Claude 应用程序主页以找到**路由模式**设置。

| 模式 | 行为 |
| :- | :- |
| **仅代码** | Claude 将所有 @mentions 路由到 Claude Code 会话。最适合仅将 Claude 用于 Slack 中开发任务的团队。 |
| **代码 + 聊天** | Claude 分析每条消息并在 Claude Code（用于编码任务）和 Claude Chat（用于写作、分析和常见问题）之间智能路由。最适合希望为所有类型工作提供单一 @Claude 入口点的团队。 |

> [!NOTE]
>
> 在代码 + 聊天模式下，如果 Claude 将消息路由到聊天但您想要编码会话，您可以单击"Retry as Code"来创建 Claude Code 会话。类似地，如果它被路由到代码但您想要聊天会话，您可以在该线程中选择该选项。

**5. 将 Claude 添加到频道**

安装后，Claude 不会自动添加到任何频道。要在频道中使用 Claude，请通过在该频道中键入 `/invite @Claude` 来邀请它。Claude 只能在已添加它的频道中响应 @mentions。

## 工作原理

### 自动检测

在 Code + Chat 路由模式下，当您在 Slack 频道或线程中提及 @Claude 时，Claude 会自动检测您的消息是否是编码任务。编码任务会被发送到 Claude Code 云会话。其他任何内容都会获得常规聊天回复。在仅 Code 模式下，每个 @mention 都会发送到 Claude Code。

您也可以明确告诉 Claude 将请求作为编码任务处理，即使它没有自动检测到。

> [!NOTE]
>
> Slack 中的 Claude Code 仅在频道（公开或私有）中工作。它在直接消息 (DM) 中不起作用。

### 上下文收集

**来自线程**：当您在线程中 @mention Claude 时，它会从该线程中的所有消息收集上下文以理解完整的对话。

**来自频道**：当直接在频道中提及时，Claude 会查看最近的频道消息以获取相关上下文。

此上下文帮助 Claude 理解问题、选择适当的存储库并指导其任务方法。

> [!WARNING]
>
> 当在 Slack 中调用 @Claude 时，Claude 可以访问对话上下文以更好地理解您的请求。Claude 可能会遵循上下文中其他消息的指示，因此用户应确保仅在受信任的 Slack 对话中使用 Claude。

### 会话流程

1. **启动**：您使用编码请求 @mention Claude
2. **检测**：Claude 分析您的消息并检测编码意图
3. **会话创建**：在 claude.ai/code 上创建新的 Claude Code 会话
4. **进度更新**：Claude 在工作进行时向您的 Slack 线程发布状态更新
5. **完成**：完成后，Claude @mentions 您并提供摘要和操作按钮
6. **审查**：单击"View Session"查看完整记录，或单击"Create PR"打开拉取请求

## 用户界面元素

### 消息操作

* **View Session**：在浏览器中打开完整的 Claude Code 会话，您可以在其中查看所有执行的工作、继续会话或提出其他请求。
* **Create PR**：直接从会话的更改创建拉取请求。
* **Retry as Code**：如果 Claude 最初作为聊天助手响应但您想要编码会话，请单击此按钮将请求重试为 Claude Code 任务。
* **Change Repo**：如果 Claude 选择不正确，允许您选择不同的存储库。

### 存储库选择

Claude 根据 Slack 对话中的上下文自动选择存储库。如果多个存储库可能适用，Claude 可能会显示一个下拉菜单，允许您选择正确的存储库。

## 访问和权限

### 用户级访问

| 访问类型 | 要求 |
| :- | :- |
| Claude Code 会话 | 每个用户在其自己的 Claude 账户下运行会话 |
| 使用情况和速率限制 | 会话计入个人用户的计划限制 |
| 存储库访问 | 用户只能访问他们个人连接的存储库 |
| 会话历史 | 会话出现在您在 claude.ai/code 上的 Claude Code 历史中 |

### 工作区级访问

Slack 工作区管理员控制 Claude 应用程序是否可以在其工作区中使用：

| 控制 | 描述 |
| :- | :- |
| 应用程序安装 | 工作区管理员决定是否从 Slack 应用程序市场安装 Claude 应用程序 |
| Enterprise Grid 分发 | 对于 Enterprise Grid 组织，组织管理员可以控制哪些工作区有权访问 Claude 应用程序 |
| 应用程序移除 | 从工作区移除应用程序会立即撤销该工作区中所有用户的访问权限 |

### 基于频道的访问控制

安装应用程序不会将 Claude 添加到任何频道。Claude 仅在已添加它的频道中响应 @mentions；使用 `/invite @Claude` 邀请它。它在公开和私有频道中都可以工作。管理员可以通过管理哪些频道邀请了 Claude 以及谁有权访问这些频道来控制谁使用 Claude Code。这在工作区级权限之外增加了一层访问控制。

## 什么可以在哪里访问

**在 Slack 中**：您将看到状态更新、完成摘要和操作按钮。完整记录被保留并始终可访问。

**在 claude.ai/code**：完整的 Claude Code 会话，包含完整的对话历史、所有代码更改和文件操作。会话保存在您的 Claude Code 历史记录中，位于 [claude.ai/code](https://claude.ai/code)，您可以在那里继续过去的会话、参考它们或创建拉取请求。

对于 Enterprise 和 Team 账户，从 Slack 中的 Claude 创建的会话会自动对组织可见。有关更多详情，请参阅 [云会话共享](https://code.claude.com/docs/zh-CN/claude-code-on-the-web.md#share-sessions)。

## 最佳实践

### 编写有效的请求

* **具体说明**：在相关时包括文件名、函数名或错误消息。
* **提供上下文**：如果从对话中不清楚，请提及存储库或项目。
* **定义成功**：解释"完成"的样子。Claude 应该编写测试吗？更新文档？创建 PR？
* **使用线程**：在讨论 Bug 或功能时在线程中回复，以便 Claude 可以收集完整的上下文。

### 何时使用 Slack 与网络

**在以下情况下使用 Slack**：上下文已存在于 Slack 讨论中，您想异步启动任务，或者您正在与需要可见性的团队成员协作。

**直接在网络上使用**：当您需要上传文件、想要在开发过程中进行实时交互或处理更长、更复杂的任务时。

## 故障排除

### "This workspace isn't set up for Claude Tag yet"

当以下两个条件同时满足时，@Claude 会在话题中回复此通知，而不是给出答案：

* 没有任何组织将您的 Slack 工作区连接到 [Claude Tag](https://claude.com/docs/claude-tag/overview)。
* 您在 Slack 中关联的 Claude 账户不属于 Pro、Max 或 Ultra 计划。

Claude Tag 适用于 Team 和 Enterprise 计划。要连接工作区，请先使用通知中提到的 `@Claude connect` 命令，然后按照[设置 Claude Tag](https://claude.com/docs/claude-tag/overview) 进行操作。

### "The legacy Claude in Slack bot is retired"

该通知以 `The legacy Claude in Slack bot is retired effective October 5, 2026 and no longer responds in channels.` 开头。您的 Slack 工作区已连接到某个 Claude 组织。在该组织的 Claude Tag 设置中，频道、工作区或组织默认值仍选择了早期版本。此通知与您自己的计划无关，因此 Pro、Max 和 Ultra 账户也会收到。

如果您是该组织的所有者，请打开 [Claude 管理设置](https://claude.ai/admin-settings/claude-tag)，并为该频道启用 Claude Tag。要修复所有继承该设置的频道，请改为在工作区或组织默认值上更改该设置。有关完整的迁移步骤，请参阅[从早期版本的 Claude in Slack 迁移](https://claude.com/docs/claude-tag/admins/migrate-from-earlier)。如果您不是所有者，请将此条目发送给所有者。

### "Claude Code is not enabled for your account"

此错误意味着您的 Claude 账户还没有云环境。使用连接到 Slack 的同一账户在 [claude.ai/code](https://claude.ai/code) 登录一次，并完成 [Web 入门](https://code.claude.com/docs/zh-CN/web-quickstart.md#connect-github)，这将创建您的默认云环境或要求您创建它。错误将在您下次提及时清除。每个用户必须单独执行此操作。

### 会话未启动

1. 验证您的 Claude 账户在 Claude 应用程序主页中已连接
2. 检查您的账户是否启用了云会话
3. 确保您至少有一个 GitHub 仓库连接到 Claude Code

### 来自 Claude Tag 频道的会话启动失败

此条目适用于使用 [Claude Tag](https://claude.com/docs/claude-tag/overview) 的工作区。如果您在 [claude.ai/code](https://claude.ai/code) 创建了频道的云环境，它属于您的个人账户，Claude 无法在个人环境中启动频道会话。Claude Code 会立即使会话失败，重试也无法帮助。

如果您是所有者且环境是您自己的，请从环境选择器中[与组织共享](https://code.claude.com/docs/zh-CN/cloud-environments.md#organization-shared-environments)。否则，所有者可从[管理设置](https://claude.ai/admin-settings)中的**云环境**页面将其重新创建为组织共享环境。

您可以通过两种方式应用它：

* 在 [claude.ai/admin-settings/claude-code](https://claude.ai/admin-settings/claude-code) 将其设置为组织默认值。
* [在 Claude Tag 管理设置中的频道上设置它](https://claude.com/docs/claude-tag/admins/troubleshooting#channel-sessions-use-the-wrong-environment-or-can%E2%80%99t-find-one)。

如果您不是所有者，请将此条目发送给所有者。

### 仓库未显示

1. 在 [claude.ai/code](https://claude.ai/code) 连接仓库
2. 验证您对该仓库的 GitHub 权限
3. 尝试断开并重新连接您的 GitHub 账户

### 选择了错误的仓库

1. 单击"Change Repo"按钮选择不同的仓库
2. 在您的请求中包括仓库名称以获得更准确的选择

### 身份验证错误

1. 在应用程序主页中断开并重新连接您的 Claude 账户
2. 确保您在浏览器中登录到正确的 Claude 账户

## 当前限制

* **仅 GitHub**：仓库必须在 GitHub 上。
* **一次一个 PR**：每个会话可以创建一个拉取请求。
* **需要云端会话访问权限**：用户需要访问[云端会话](https://code.claude.com/docs/zh-CN/claude-code-on-the-web.md)。

## 相关资源

> [card] [云端 Claude Code](https://code.claude.com/docs/zh-CN/claude-code-on-the-web.md)
>
> 了解有关云端会话的更多信息

> [card] [Claude for Slack](https://claude.com/claude-and-slack)
>
> Claude for Slack 常规文档

> [card] [Claude Tag](https://claude.com/docs/claude-tag/overview)
>
> Slack 中由组织管理的 @Claude，具有管理员配置的访问权限

> [card] [Slack 应用程序市场](https://slack.com/marketplace/A08SF47R6P4)
>
> 从 Slack 市场安装 Claude 应用程序

> [card] [Claude 帮助中心](https://support.claude.com)
>
> 获取额外支持
