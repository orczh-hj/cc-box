> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# 使用 channels 将事件推送到运行中的会话

> 使用 channels 从 MCP 服务器将消息、警报和 webhooks 推送到您的 Claude Code 会话中。转发 CI 结果、聊天消息和监控事件，以便 Claude 在您离开时做出反应。

> [!NOTE]
>
> Channels 处于[研究预览](#research-preview)阶段。它们需要通过 claude.ai 或控制台 API 密钥进行 Anthropic 身份验证，在 Amazon Bedrock、Google Cloud 的 Agent Platform 或 Microsoft Foundry 上不可用。Team 和 Enterprise 组织必须[明确启用它们](#enterprise-controls)。

Channel 是一个 MCP 服务器，它将事件推送到您运行中的 Claude Code 会话中，以便 Claude 可以对您不在终端时发生的事情做出反应。Channels 可以是双向的：Claude 读取事件并通过同一 channel 回复，就像聊天桥接一样。事件仅在会话打开时到达，因此对于始终在线的设置，您可以在后台进程或持久终端中运行 Claude。

与生成新的云会话或等待被轮询的集成不同，事件到达您已经打开的会话中：请参阅 [channels 如何比较](#how-channels-compare)。

您将 channel 作为插件安装并使用您自己的凭据配置它。Telegram、Discord 和 iMessage 包含在研究预览中。

当 Claude 通过 channel 回复时，您会在终端中看到入站消息，但看不到回复文本。终端显示工具调用和确认（如"已发送"），实际回复出现在其他平台上。

如果您管理 Team、Enterprise 或控制台组织，请参阅[为您的组织启用 channels](#enterprise-controls)。要构建您自己的 channel，请参阅 [Channels 参考](https://code.claude.com/docs/zh-CN/channels-reference.md)。

## 支持的频道

每个支持的频道都是一个需要 [Bun](https://bun.sh) 的插件。在连接真实平台之前，要获得插件流程的实际演示，请尝试 [fakechat 快速入门](#quickstart)。

> [tab] Telegram
>
> 查看完整的 [Telegram 插件源代码](https://github.com/anthropics/claude-plugins-official/tree/main/external_plugins/telegram)。
>
> **1. 创建 Telegram 机器人**
>
> 在 Telegram 中打开 [BotFather](https://t.me/BotFather) 并发送 `/newbot`。给它一个显示名称和一个以 `bot` 结尾的唯一用户名。复制 BotFather 返回的令牌。
>
> **2. 安装插件**
>
> 在终端中运行 `claude` 启动 Claude Code，然后在其输入框中输入以下内容：
>
> ```
> /plugin install telegram@claude-plugins-official
> ```
>
> 如果安装失败，请匹配 Claude Code 报告的消息：
>
> * `Marketplace "claude-plugins-official" not found`：使用 `/plugin marketplace add anthropics/claude-plugins-official` 添加市场，然后重试安装。
> * `Plugin "<name>" not found in marketplace "claude-plugins-official"`：您本地的市场目录副本可能已过期。使用 `/plugin marketplace update claude-plugins-official` 刷新它，然后重试安装。如果重试后打印相同的消息，请参阅[在市场中找不到插件](https://code.claude.com/docs/zh-CN/plugins/troubleshooting.md#plugin-not-found-in-marketplace)。
>
> 当安装要求选择安装作用域时，选择用户作用域选项，以便插件在所有项目中可用。检查安装摘要：如果它报告 `Run /reload-plugins to apply.`，请参阅[在不重启的情况下应用插件更改](https://code.claude.com/docs/zh-CN/plugins/cli-reference.md#reload-plugins)以使插件的配置命令可用。
>
> **3. 配置您的令牌**
>
> 使用来自 BotFather 的令牌运行配置命令：
>
> ```
> /telegram:configure <token>
> ```
>
> 这会将其保存到 `~/.claude/channels/telegram/.env`。您也可以在启动 Claude Code 之前在 shell 环境中设置 `TELEGRAM_BOT_TOKEN`。
>
> **4. 重启并启用频道**
>
> 退出 Claude Code 并使用频道标志重启。这会启动 Telegram 插件，它开始轮询来自您的机器人的消息：
>
> ```bash theme={null}
> claude --channels plugin:telegram@claude-plugins-official
> ```
>
> **5. 配对您的账户**
>
> 打开 Telegram 并向您的机器人发送任何消息。机器人会回复一个配对代码。
>
> > [!NOTE]
> >
> > 如果您的机器人没有响应，请确保 Claude Code 正在使用上一步中的 `--channels` 运行。机器人只能在频道处于活动状态时回复。
>
> 回到 Claude Code，运行：
>
> ```
> /telegram:access pair <code>
> ```
>
> 然后锁定访问权限，以便只有您的账户可以发送消息：
>
> ```
> /telegram:access policy allowlist
> ```

> [tab] Discord
>
> 查看完整的 [Discord 插件源代码](https://github.com/anthropics/claude-plugins-official/tree/main/external_plugins/discord)。
>
> **1. 创建 Discord 机器人**
>
> 转到 [Discord 开发者门户](https://discord.com/developers/applications)，点击**新应用程序**，并为其命名。在**机器人**部分中，创建用户名，然后点击**重置令牌**并复制令牌。
>
> **2. 启用消息内容意图**
>
> 在您的机器人设置中，滚动到**特权网关意图**并启用**消息内容意图**。
>
> **3. 邀请机器人加入您的服务器**
>
> 转到 **OAuth2 > URL 生成器**。选择 `bot` 作用域并启用这些权限：
>
> * 查看频道
> * 发送消息
> * 在线程中发送消息
> * 读取消息历史记录
> * 附加文件
> * 添加反应
>
> 打开生成的 URL 以将机器人添加到您的服务器。
>
> **4. 安装插件**
>
> 在终端中运行 `claude` 启动 Claude Code，然后在其输入框中输入以下内容：
>
> ```
> /plugin install discord@claude-plugins-official
> ```
>
> 如果安装失败，请匹配 Claude Code 报告的消息：
>
> * `Marketplace "claude-plugins-official" not found`：使用 `/plugin marketplace add anthropics/claude-plugins-official` 添加市场，然后重试安装。
> * `Plugin "<name>" not found in marketplace "claude-plugins-official"`：您本地的市场目录副本可能已过期。使用 `/plugin marketplace update claude-plugins-official` 刷新它，然后重试安装。如果重试后打印相同的消息，请参阅[在市场中找不到插件](https://code.claude.com/docs/zh-CN/plugins/troubleshooting.md#plugin-not-found-in-marketplace)。
>
> 当安装要求选择安装作用域时，选择用户作用域选项，以便插件在所有项目中可用。检查安装摘要：如果它报告 `Run /reload-plugins to apply.`，请参阅[在不重启的情况下应用插件更改](https://code.claude.com/docs/zh-CN/plugins/cli-reference.md#reload-plugins)以使插件的配置命令可用。
>
> **5. 配置您的令牌**
>
> 使用您复制的机器人令牌运行配置命令：
>
> ```
> /discord:configure <token>
> ```
>
> 这会将其保存到 `~/.claude/channels/discord/.env`。您也可以在启动 Claude Code 之前在 shell 环境中设置 `DISCORD_BOT_TOKEN`。
>
> **6. 重启并启用频道**
>
> 退出 Claude Code 并使用频道标志重启。这会连接 Discord 插件，以便您的机器人可以接收和响应消息：
>
> ```bash theme={null}
> claude --channels plugin:discord@claude-plugins-official
> ```
>
> **7. 配对您的账户**
>
> 在 Discord 上向您的机器人发送私信。机器人会回复一个配对代码。
>
> > [!NOTE]
> >
> > 如果您的机器人没有响应，请确保 Claude Code 正在使用上一步中的 `--channels` 运行。机器人只能在频道处于活动状态时回复。
>
> 回到 Claude Code，运行：
>
> ```
> /discord:access pair <code>
> ```
>
> 然后锁定访问权限，以便只有您的账户可以发送消息：
>
> ```
> /discord:access policy allowlist
> ```

> [tab] iMessage
>
> 查看完整的 [iMessage 插件源代码](https://github.com/anthropics/claude-plugins-official/tree/main/external_plugins/imessage)。
>
> iMessage 频道直接读取您的消息数据库，并通过 AppleScript 发送回复。它需要 macOS，不需要机器人令牌或外部服务。
>
> **1. 授予完全磁盘访问权限**
>
> 位于 `~/Library/Messages/chat.db` 的消息数据库受 macOS 保护。服务器第一次读取它时，macOS 会提示访问权限：点击**允许**。提示会命名启动 Bun 的应用程序，例如终端、iTerm 或您的 IDE。
>
> 如果提示没有出现或您点击了"不允许"，请在**系统设置 > 隐私和安全 > 完全磁盘访问**下手动授予访问权限，并添加您的终端。没有这个，服务器会立即退出，显示 `authorization denied`。
>
> **2. 安装插件**
>
> 在终端中运行 `claude` 启动 Claude Code，然后在其输入框中输入以下内容：
>
> ```
> /plugin install imessage@claude-plugins-official
> ```
>
> 如果安装失败，请匹配 Claude Code 报告的消息：
>
> * `Marketplace "claude-plugins-official" not found`：使用 `/plugin marketplace add anthropics/claude-plugins-official` 添加市场，然后重试安装。
> * `Plugin "<name>" not found in marketplace "claude-plugins-official"`：您本地的市场目录副本可能已过期。使用 `/plugin marketplace update claude-plugins-official` 刷新它，然后重试安装。如果重试后打印相同的消息，请参阅[在市场中找不到插件](https://code.claude.com/docs/zh-CN/plugins/troubleshooting.md#plugin-not-found-in-marketplace)。
>
> 当安装要求选择安装作用域时，选择用户作用域选项，以便插件在所有项目中可用。
>
> 如果安装摘要报告 `Run /reload-plugins to apply.`，您无需在此处理，因为下一步中的重启会加载该插件。
>
> **3. 重启并启用频道**
>
> 退出 Claude Code 并使用频道标志重启：
>
> ```bash theme={null}
> claude --channels plugin:imessage@claude-plugins-official
> ```
>
> **4. 给自己发短信**
>
> 在任何登录到您的 Apple ID 的设备上打开消息，并向自己发送消息。它立即到达 Claude：自聊天绕过访问控制，无需设置。
>
> > [!NOTE]
> >
> > Claude 发送的第一条回复会触发 macOS 自动化提示，询问您的终端是否可以控制消息。点击**确定**。
>
> **5. 允许其他发送者**
>
> 默认情况下，只有您自己的消息通过。要让另一个联系人到达 Claude，请添加他们的句柄：
>
> ```
> /imessage:access allow +15551234567
> ```
>
> 句柄是 `+country` 格式的电话号码或 Apple ID 电子邮件，如 `user@example.com`。

## 快速入门

Fakechat 是一个官方支持的演示 channel，在 localhost 上运行聊天 UI，无需身份验证，也无需配置外部服务。

安装并启用 fakechat 后，您可以在浏览器中输入，消息会到达您的 Claude Code 会话。Claude 回复，回复会显示在浏览器中。测试了 fakechat 界面后，尝试 [Telegram](https://github.com/anthropics/claude-plugins-official/tree/main/external_plugins/telegram)、[Discord](https://github.com/anthropics/claude-plugins-official/tree/main/external_plugins/discord) 或 [iMessage](https://github.com/anthropics/claude-plugins-official/tree/main/external_plugins/imessage)。

要尝试 fakechat 演示，您需要：

* Claude Code [已安装并使用 claude.ai 账户或 Claude 控制台 API 密钥进行身份验证](https://code.claude.com/docs/zh-CN/quickstart.md#step-1-install-claude-code)
* [Bun](https://bun.sh) 已安装。预构建的 channel 插件是 Bun 脚本。使用 `bun --version` 检查；如果失败，[安装 Bun](https://bun.sh/docs/installation)。
* **Team、Enterprise 或托管控制台组织**：您的管理员必须在托管设置中[启用 channels](#enterprise-controls)

**1. 安装 fakechat 频道插件**

在终端中运行 `claude` 启动 Claude Code，然后在其输入框中输入安装命令：

```text theme={null}
/plugin install fakechat@claude-plugins-official
```

如果安装失败，请匹配 Claude Code 报告的消息：

* `Marketplace "claude-plugins-official" not found`：使用 `/plugin marketplace add anthropics/claude-plugins-official` 添加市场，然后重试安装。
* `Plugin "<name>" not found in marketplace "claude-plugins-official"`：您本地的市场目录副本可能已过期。使用 `/plugin marketplace update claude-plugins-official` 刷新它，然后重试安装。如果重试后仍显示相同的消息，请参阅[在市场中找不到插件](https://code.claude.com/docs/zh-CN/plugins/troubleshooting.md#plugin-not-found-in-marketplace)。

当安装要求安装范围时，选择用户范围选项，以便插件在您的所有项目中可用。

如果安装摘要报告 `Run /reload-plugins to apply.`，您不需要在此处采取行动，因为下一步中的重启会加载该插件。

**2. 重启并启用 channel**

退出 Claude Code，然后使用 `--channels` 重启并传递您安装的 fakechat 插件：

```bash theme={null}
claude --channels plugin:fakechat@claude-plugins-official
```

fakechat 服务器会自动启动。启动屏幕显示一个 channels 通知，说明来自 `plugin:fakechat@claude-plugins-official` 的消息直接注入此会话。如果插件未安装或不在批准的允许列表中，问题名称下方会出现警告行。

> [!TIP]
>
> 您可以将多个插件传递给 `--channels`，用空格分隔。

**3. 推送消息进来**

在 [http://localhost:8787](http://localhost:8787) 打开 fakechat UI 并输入消息：

```text theme={null}
what's in my working directory?
```

消息到达您的 Claude Code 会话。终端将其显示为入站 channel 行，如 `← fakechat · web: what's in my working directory?`，而模型将其接收为 `<channel source="plugin:fakechat:fakechat">` 事件，使用插件的作用域服务器名称。Claude 读取它，完成工作，并调用 fakechat 的 `reply` 工具。如果 Claude Code 要求第一次回复的权限，请批准。答案显示在聊天 UI 中。

如果 Claude 在您离开终端时遇到权限提示，会话会暂停，直到您响应。声明[权限中继功能](https://code.claude.com/docs/zh-CN/channels-reference.md#relay-permission-prompts)的 Channel 服务器可以将这些提示转发给您，以便您可以远程批准或拒绝。对于无人值守使用，[`--dangerously-skip-permissions`](https://code.claude.com/docs/zh-CN/permission-modes#skip-all-checks-with-bypasspermissions-mode) 绕过大多数提示，但仅在您信任的环境中使用。即使这样，[任何模式都不会自动批准的操作](https://code.claude.com/docs/zh-CN/permission-modes.md#actions-no-mode-auto-approves)仍然适用。

当您使用 `-p` 以非交互模式运行 channels 时，需要终端输入的工具（如多选问题和 Plan Mode 批准）被禁用，以便会话永远不会因等待输入而停滞。

## 安全性

每个批准的 channel 插件都维护一个发送者允许列表：只有您添加的 ID 可以推送消息，其他所有人都会被静默丢弃。

Telegram 和 Discord 通过配对来引导列表：

1. 在 Telegram 或 Discord 中找到您的机器人并向其发送任何消息
2. 机器人回复一个配对代码
3. 在您的 Claude Code 会话中，在提示时批准代码
4. 您的发送者 ID 被添加到允许列表

iMessage 的工作方式不同：给自己发短信会自动绕过门禁，您可以使用 `/imessage:access allow` 通过句柄添加其他联系人。

除此之外，您可以使用 `--channels` 控制每个会话启用哪些服务器，您的组织可以使用 [`channelsEnabled`](#enterprise-controls) 在 claude.ai Team 和 Enterprise 计划以及部署托管设置的控制台组织上控制可用性。

仅在 `.mcp.json` 中还不足以推送消息：服务器还必须在 `--channels` 中命名。

允许列表也会限制[权限中继](https://code.claude.com/docs/zh-CN/channels-reference.md#relay-permission-prompts)（如果 channel 声明了它）。任何可以通过 channel 回复的人都可以批准或拒绝您会话中的工具使用，因此只允许列表您信任具有该权限的发送者。

## Enterprise 控制

管理员通过两个[托管设置](https://code.claude.com/docs/zh-CN/settings.md)控制可用性，用户无法覆盖。默认值取决于您如何进行身份验证：

* **claude.ai Team 和 Enterprise**：channels 被阻止，直到所有者[启用它们](#enable-channels-for-your-organization)。
* **Anthropic 控制台与 API 密钥身份验证**：channels 默认被允许。仅当您的组织部署托管设置时才需要此设置。

在所有情况下，在用户使用 `--channels` 为会话选择加入之前，没有 channel 会运行。

| 设置 | 目的 | 未配置时 |
| :- | :- | :- |
| `channelsEnabled` | 主开关。必须为 `true` 才能让任何 channel 传递消息。阻止所有 channels，包括开发标志（关闭时）。请参阅[为您的组织启用 channels](#enable-channels-for-your-organization)。 | claude.ai Team 和 Enterprise：channels 被阻止。控制台：channels 被允许，除非您的组织部署托管设置，在这种情况下 channels 被阻止，直到设置此密钥 |
| `allowedChannelPlugins` | 启用 channels 后哪些插件可以注册。设置时替换 Anthropic 维护的列表。 | 应用 Anthropic 默认列表 |

没有组织的 Pro、Max 和 Ultra 用户完全跳过这些检查：频道可用，用户使用 `--channels` 按会话选择加入。

### 为您的组织启用 channels

可以从 [**Organization settings > Claude Code > Channels**](https://claude.ai/admin-settings/claude-code) 为您的组织启用频道（需要所有者角色），或通过在托管设置中将 `channelsEnabled` 设置为 `true`。

启用后，您组织中的用户可以使用 `--channels` 将 channel 服务器选择加入到各个会话中。如果设置被禁用或未设置，MCP 服务器仍会连接，其工具可以工作，但 channel 消息不会到达。启动警告会告诉用户让管理员启用该设置。

### 限制哪些 channel 插件可以运行

默认情况下，Anthropic 维护的允许列表上的任何插件都可以注册为 channel。Team 和 Enterprise 计划上的管理员可以通过在托管设置中设置 `allowedChannelPlugins` 来替换该允许列表。使用此功能来限制允许哪些官方插件、批准来自您自己的内部市场的 channels，或两者兼有。每个条目命名一个插件及其来自的市场：

```json theme={null}
{
  "channelsEnabled": true,
  "allowedChannelPlugins": [
    { "marketplace": "claude-plugins-official", "plugin": "telegram" },
    { "marketplace": "claude-plugins-official", "plugin": "discord" },
    { "marketplace": "acme-corp-plugins", "plugin": "internal-alerts" }
  ]
}
```

如果您设置空数组，您会阻止所有 channel 插件从允许列表中，但 `--dangerously-load-development-channels` 仍可以为本地测试绕过该阻止。要完全阻止 channels，包括开发标志，请改为保持 `channelsEnabled` 未设置。

此设置需要 `channelsEnabled: true`。如果用户将不在您列表中的插件传递给 `--channels`，Claude Code 会正常启动，但 channel 不会注册，启动通知会解释该插件不在组织的批准列表中。在 v2 MCP 客户端运行时上，channel 也可能无法注册，因为 Claude Code [不会注册协商协议修订版本 2026-07-28 的 channel 服务器](https://code.claude.com/docs/zh-CN/mcp.md#push-messages-with-channels)。

## 研究预览

Channels 是一个研究预览功能。可用性正在逐步推出，`--channels` 标志语法和协议契约可能会根据反馈而改变。

在预览期间，`--channels` 和 `--dangerously-load-development-channels` 都不会出现在 `claude --help` 中。这些标志即使未列出也能正常工作。

在预览期间，`--channels` 仅接受来自 Anthropic 维护的允许列表的插件，或来自您组织的允许列表（如果管理员已设置 [`allowedChannelPlugins`](#restrict-which-channel-plugins-can-run)）。[claude-plugins-official](https://github.com/anthropics/claude-plugins-official/tree/main/external_plugins) 中的 channel 插件是默认批准的集合。如果您传递不在有效允许列表中的内容，Claude Code 会正常启动，但 channel 不会注册，启动通知会告诉您原因。

要测试您正在构建的 channel，请将其以 `plugin:<name>@<marketplace>` 或 `server:<name>` 的形式传递给 `--dangerously-load-development-channels`。有关测试您构建的自定义 channels 的信息，请参阅[在研究预览期间测试](https://code.claude.com/docs/zh-CN/channels-reference.md#test-during-the-research-preview)。

在 [Claude Code GitHub 存储库](https://github.com/anthropics/claude-code/issues)上报告问题或反馈。

## channels 如何比较

几个 Claude Code 功能连接到终端外的系统，每个都适合不同类型的工作：

| 功能 | 它做什么 | 适合 |
| - | - | - |
| [云端会话](https://code.claude.com/docs/zh-CN/claude-code-on-the-web.md) | 在新的云沙箱中运行任务，从 GitHub 克隆 | 委派您稍后检查的自包含异步工作 |
| [Slack 中的 Claude](https://code.claude.com/docs/zh-CN/slack.md) | 从频道或线程中的 `@Claude` 提及生成云端会话 | 直接从团队对话上下文启动任务 |
| 标准 [MCP 服务器](https://code.claude.com/docs/zh-CN/mcp.md) | Claude 在任务期间查询它；没有任何内容被推送到会话 | 给 Claude 按需访问以读取或查询系统 |
| [Remote Control](https://code.claude.com/docs/zh-CN/remote-control.md) | 您从 claude.ai 或 Claude 移动应用程序驱动您的本地会话 | 在离开您的办公桌时指导进行中的会话 |

Channels 通过将来自非 Claude 源的事件推送到您已经运行的本地会话中，填补了该列表中的空白。

* **聊天桥接**：通过 Telegram、Discord 或 iMessage 从您的手机向 Claude 询问某事，答案会在同一聊天中返回，而工作在您的机器上针对您的真实文件运行。
* **[Webhook 接收器](https://code.claude.com/docs/zh-CN/channels-reference.md#example-build-a-webhook-receiver)**：来自 CI、您的错误跟踪器、部署管道或其他外部服务的 webhook 到达 Claude 已经打开您的文件并记得您正在调试的内容的地方。

## 后续步骤

一旦您有一个 channel 运行，请探索这些相关功能：

* [构建您自己的 channel](https://code.claude.com/docs/zh-CN/channels-reference.md)，用于尚未有插件的系统
* [Remote Control](https://code.claude.com/docs/zh-CN/remote-control.md)，从您的手机驱动本地会话，而不是将事件转发到其中
* [会话范围的定时任务](https://code.claude.com/docs/zh-CN/scheduled-tasks.md)，按计时器轮询而不是对推送事件做出反应
