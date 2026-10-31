> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# 将 Claude Code 连接到 LLM 网关

> 将 Claude Code 指向您组织的 LLM 网关。检查您的管理员是否已配置它，或自行设置基础 URL 和凭证，然后验证连接并修复网关错误。

[LLM 网关](https://code.claude.com/docs/zh-CN/llm-gateway)是您的组织在 Claude Code 和模型提供商之间运行的代理。当您的组织使用网关时，Claude Code 使用您的组织颁发的凭证而不是您的个人 claude.ai 登录来向网关进行身份验证。

本页面适用于通过其组织运营的网关运行 Claude Code 的开发人员。它涵盖两条路径：[检查您的管理员是否已为您配置它](#check-for-an-existing-configuration)，以及[在他们没有配置时自行配置](#configure-claude-code-yourself)。

> [!NOTE]
>
> * 要为您的组织部署网关，请参阅[推出 LLM 网关](https://code.claude.com/docs/zh-CN/llm-gateway-rollout.md)
> * 有关 Claude Code 发送到网关的内容，请参阅[网关协议参考](https://code.claude.com/docs/zh-CN/llm-gateway-protocol.md)

## 检查现有配置

管理员可以通过[托管设置](https://code.claude.com/docs/zh-CN/managed-settings.md)、设备管理或 [`apiKeyHelper`](#rotate-credentials-with-apikeyhelper) 分发网关地址和凭证，以便 Claude Code 在启动时自动获取，无需您进行任何设置。要检查您的组织是否已这样做：

**1. 启动 Claude Code**

运行 `claude`。如果它打开到登录屏幕而不是会话，则没有分发网关凭证；[自行配置](#configure-claude-code-yourself)如下。

**2. 检查状态选项卡**

如果 Claude Code 启动了会话而没有显示登录屏幕，运行 `/status`，打开**状态**选项卡，并检查两行：

* `Anthropic base URL`：仅当设置了网关地址时才显示此行。如果不存在，Claude Code 未指向网关；[自行配置](#configure-claude-code-yourself)如下。
* `Auth token` 或 `API key`：命名 `ANTHROPIC_AUTH_TOKEN`、`ANTHROPIC_API_KEY` 或 `apiKeyHelper` 的行确认网关凭证处于活动状态。命名 claude.ai 账户的 `Login method` 行意味着凭证未被分发；[自行设置](#set-the-credential-variable)。

**3. 发送测试消息**

关闭 `/status` 菜单并在 Claude Code 中发送任何提示。来自 Claude 的正常响应，没有错误，确认网关连接有效。

如果 `/status` 菜单中的两行看起来都正确，但向 Claude 的消息失败，请参阅[故障排除表](#troubleshoot-gateway-errors)。

## 自行配置 Claude Code

要自行为网关配置 Claude Code，您需要从网关团队获得：

* 网关的基础 URL
* 凭证：密钥或令牌字符串，或获取凭证的命令
  * 如果您的网关团队没有说明凭证的类型，下面的[凭证变量部分](#set-the-credential-variable)涵盖了要尝试的内容

下面的部分按顺序涵盖配置：

* [设置凭证变量](#set-the-credential-variable)和[设置基础 URL](#set-the-base-url-and-credential)：每个网关连接需要的两个变量
* [验证连接](#verify-the-connection)：在保存任何内容之前确认它有效
* [配置每个界面](#configure-each-surface)：如果您使用除 Claude Code CLI 之外的界面（如 VS Code），请查看如何使用网关凭证配置它
* [其他配置](#additional-configuration)：某些网关需要的变量超出基础 URL 和凭证，例如自定义标头、凭证助手、模型发现、提供商格式的基础 URL 或关闭网关路径外的流量。仅在您的管理员命名它们或您的网络限制出站流量时设置这些

### 设置凭证变量

要向网关验证 Claude Code，请在环境变量中设置您的凭证。哪个变量取决于您的网关团队告诉您的内容：

| 在以下位置设置凭证 | 使用时机 |
| :- | :- |
| `ANTHROPIC_AUTH_TOKEN` | 您的网关团队说"bearer token"或"Authorization header" |
| `ANTHROPIC_API_KEY` | 您的网关团队说"API key"或"x-api-key" |
| [`apiKeyHelper`](#rotate-credentials-with-apikeyhelper) | 凭证轮换或来自保管库 |

如果您没有被告知是哪种类型，请使用 `ANTHROPIC_AUTH_TOKEN`；下面的[验证请求](#verify-the-connection)显示了如何判断您是否需要切换。

### 设置基础 URL 和凭证

将网关的基础 URL 和您上面选择的凭证变量设置为环境变量。示例使用 `ANTHROPIC_AUTH_TOKEN`；如果那是[您选择的变量](#set-the-credential-variable)，请将其替换为 `ANTHROPIC_API_KEY`。您可以[在您的 shell 中](#set-as-shell-environment-variables)设置它们，这仅持续一个终端会话，或[在 Claude Code 设置文件中](#set-in-a-settings-file)设置它们，这在 Claude Code 运行的任何地方都持续。

对于您的第一次连接，从 shell 导出开始，并在将值移动到设置文件之前运行[验证请求](#verify-the-connection)。

#### 设置为 shell 环境变量

将值替换为您的网关团队给您的值：

> [tab] Bash or Zsh
>
> ```bash theme={null}
> export ANTHROPIC_BASE_URL=https://llm-gateway.example.com
> export ANTHROPIC_AUTH_TOKEN=sk-gateway-key
> ```

> [tab] PowerShell
>
> ```powershell theme={null}
> $env:ANTHROPIC_BASE_URL = "https://llm-gateway.example.com"
> $env:ANTHROPIC_AUTH_TOKEN = "sk-gateway-key"
> ```

Shell 导出仅适用于该终端会话和从它启动的程序。从 dock 或开始菜单启动的编辑器不会看到它们。要使值在新终端中持续，请将相同的行添加到您的 shell 配置文件，例如 `~/.zshrc`、`~/.bashrc` 或您的 PowerShell `$PROFILE`。

如果您仅在 shell 中导出网关，它不会可靠地到达由[主管程序](https://code.claude.com/docs/zh-CN/agent-view.md#how-background-sessions-are-hosted)托管的后台代理；请参阅[每个后台会话如何获取其网关](https://code.claude.com/docs/zh-CN/agent-view.md#llm-gateway)。对于后台代理必须始终路由的任何网关，请使用设置文件。

#### 在设置文件中设置

要使配置在 Claude Code 运行的任何地方应用，包括[后台代理](https://code.claude.com/docs/zh-CN/agent-view.md#how-background-sessions-are-hosted)，请在[设置文件](https://code.claude.com/docs/zh-CN/settings.md)的 `env` 块中设置变量，而不是依赖您的 shell。设置文件有不同的范围：

* `~/.claude/settings.json` 适用于您的所有项目。在 Windows 上，路径是 `%USERPROFILE%\.claude\settings.json`
* `.claude/settings.local.json` 适用于一个项目。Claude Code 在保存设置时将其添加到您的全局 gitignore；如果您手动创建它或让 Claude 编写它，请首先自己将其添加到您的 gitignore，以便您不会意外提交您的凭证

> [!WARNING]
>
> 不要将凭证放在项目的 `.claude/settings.json` 中。该文件被提交并与克隆存储库的每个人共享。

`env` 块在任一文件中看起来相同：

```json theme={null}
{
  "env": {
    "ANTHROPIC_BASE_URL": "https://llm-gateway.example.com",
    "ANTHROPIC_AUTH_TOKEN": "sk-gateway-key"
  }
}
```

当 shell 导出和设置文件 `env` 块都设置相同的变量时，设置文件值适用。运行 `/status` 以查看 Claude Code 使用的基础 URL 和凭证源。

### 验证连接

使用在 shell 中导出的变量，向网关直接发送一个单令牌请求。这在您打开 Claude Code 之前确认 URL 和凭证有效，因此失败指向网关而不是您的配置。下面的命令读取 shell 变量，因此即使您也将值放在设置文件中，它们也需要[shell 导出](#set-as-shell-environment-variables)。

> [tab] Bash or Zsh
>
> ```bash theme={null}
> curl -sS -w '\n%{http_code}\n' -X POST "$ANTHROPIC_BASE_URL/v1/messages" \
>   -H "Authorization: Bearer $ANTHROPIC_AUTH_TOKEN" \
>   -H "anthropic-version: 2023-06-01" \
>   -H "content-type: application/json" \
>   -d '{"model": "claude-sonnet-4-6", "max_tokens": 1, "messages": [{"role": "user", "content": "."}]}'
> ```

> [tab] PowerShell
>
> ```powershell theme={null}
> Invoke-RestMethod -Method Post -Uri "$env:ANTHROPIC_BASE_URL/v1/messages" `
>   -Headers @{ "Authorization" = "Bearer $env:ANTHROPIC_AUTH_TOKEN"; "anthropic-version" = "2023-06-01" } `
>   -ContentType "application/json" `
>   -Body '{"model": "claude-sonnet-4-6", "max_tokens": 1, "messages": [{"role": "user", "content": "."}]}'
> ```

如果您的网关期望 `x-api-key` 标头中的密钥，请在 Bash 命令中将 `Authorization` 标头替换为 `x-api-key: $ANTHROPIC_API_KEY`，或在 PowerShell 命令中将 `"Authorization"` 哈希表条目替换为 `"x-api-key" = "$env:ANTHROPIC_API_KEY"`。

以 `{"id":"msg_` 开头并包含 `"content":[...]` 字段的 JSON 响应意味着网关可达且凭证有效。命名未知模型的错误仍然证明 URL 和凭证有效，因为网关在拒绝模型名称之前验证了请求；您不需要为此测试找到您的网关提供的模型。`401` 意味着凭证被拒绝：如果您猜测了变量，请切换到另一个并重新导出。

#### 在 Claude Code 中确认

从同一 shell 启动 `claude`，以便它继承导出，发送消息，并运行 `/status`。

在**状态**选项卡上，`Anthropic base URL` 行应显示您的网关地址，这确认请求正在路由到那里；如果该行不存在，变量没有到达会话。命名您设置的变量的 `Auth token` 或 `API key` 行确认网关凭证处于活动状态而不是保存的 claude.ai 登录。

如果消息失败，或 `/status` 不显示网关 URL，请参阅下面的[故障排除表](#troubleshoot-gateway-errors)。

### 凭证变量如何映射到标头

每个变量在不同的 HTTP 标头中发送凭证：`ANTHROPIC_AUTH_TOKEN` 在 `Authorization: Bearer` 中，`ANTHROPIC_API_KEY` 在 `x-api-key` 中，`apiKeyHelper` 在两者中。错误变量中的凭证到达网关时处于它不读取的标头中，请求失败并返回 `401`。如果验证请求返回 `401`，请切换到另一个变量并重试。

### 与现有登录的冲突

网关凭证变量优先于保存的 claude.ai 登录或 Console 密钥。您的 claude.ai 登录在设置变量时保持保存和未使用；取消设置变量，Claude Code 返回到它。使用 `ANTHROPIC_AUTH_TOKEN`，变量立即优先。使用 `ANTHROPIC_API_KEY`，在交互模式下提示您一次以批准密钥，然后它接管。

运行 `/status` 以确认哪个凭证源处于活动状态。如果启动显示命名两个源的身份验证冲突警告，请参阅[故障排除表](#troubleshoot-gateway-errors)的第一行，了解要删除哪一个。要清除保存的登录，以便仅保留网关凭证，请运行 `/logout`。

## 配置每个界面

CLI 读取上面的环境变量和设置文件。其他界面是 VS Code 扩展、桌面应用、GitHub Actions、Agent SDK 和云界面（如 Slack 和网络）；下面的部分涵盖这些设置是否到达每一个。

### VS Code 扩展

在 VS Code 自己的用户设置中的 `claudeCode.environmentVariables` 中为 [VS Code 扩展](https://code.claude.com/docs/zh-CN/vs-code.md)设置网关变量，使用**首选项：打开用户设置 (JSON)** 命令打开。扩展在启动前检查此设置中的凭证，因此这是网关凭证的可靠位置；`~/.claude/settings.json` 中的值到达生成的进程但不到达扩展自己的登录检查。

```json theme={null}
{
  "claudeCode.environmentVariables": [
    { "name": "ANTHROPIC_BASE_URL", "value": "https://llm-gateway.example.com" },
    { "name": "ANTHROPIC_AUTH_TOKEN", "value": "sk-gateway-key" }
  ]
}
```

### 桌面应用

桌面应用从其[第三方推理配置](https://claude.com/docs/third-party/claude-desktop/gateway)读取网关路由，而不是从 `ANTHROPIC_BASE_URL` 或 `settings.json` 读取。该配置可以来自您的组织或来自应用本身中的表单：

* **由管理员分发**：如果您的组织已[部署配置](https://code.claude.com/docs/zh-CN/llm-gateway-rollout.md#distribute-through-managed-settings)，桌面应用通过网关路由，无需您进行任何设置
* **本地配置**：对于没有管理员分发配置的设备，打开帮助 → 故障排除 → 启用开发者模式，这将重新启动应用并显示开发者菜单。然后打开开发者 → 配置第三方推理并输入您的网关基础 URL。管理员分发的配置优先级更高，使此表单为只读

启用网关配置后，环境选择器不提供 Anthropic 托管的云环境，并且 [Remote Control](https://code.claude.com/docs/zh-CN/remote-control.md) 不可用。

在网关配置下，SSH 会话处于 beta 阶段，需要 Claude Desktop v1.40609.0 或更高版本。连接之前，请检查允许列表和网关地址：

* **允许的主机**：SSH 会话默认关闭。要启用它们，您或您的管理员需要在第三方推理配置的 [`sshHostAllowlist`](https://claude.com/docs/third-party/claude-desktop/configuration#sshhostallowlist) 键中列出允许的主机
* **网关地址**：远程机器会自行连接网关，因此位于您计算机 `localhost` 上的网关不适用于 SSH 会话

请参阅 [SSH remote sessions in Claude Desktop on 3P](https://claude.com/docs/third-party/claude-desktop/ssh-remote-sessions)。您也可以在远程主机上运行 CLI，并在那里设置 [`ANTHROPIC_BASE_URL` 和网关凭据](#set-the-base-url-and-credential)。

如果桌面应用显示 `Gateway was unreachable`，应用在启动时无法到达配置的基础 URL；使用上面的 [curl 测试](#verify-the-connection)检查 URL 和网络路径。

### GitHub Actions

[Claude Code GitHub Actions](https://code.claude.com/docs/zh-CN/github-actions.md) 从工作流的 `env` 块读取 `ANTHROPIC_BASE_URL` 和 `ANTHROPIC_CUSTOM_HEADERS`。将凭证作为操作的 `anthropic_api_key` 输入传递；操作将其设置为 `ANTHROPIC_API_KEY`，因此它到达网关时处于 `x-api-key` 标头中。

对于 `x-api-key` 网关，在 `env` 中设置基础 URL 并将网关密钥作为输入传递：

```yaml theme={null}
env:
  ANTHROPIC_BASE_URL: https://llm-gateway.example.com

steps:
  - uses: anthropics/claude-code-action@v1
    with:
      anthropic_api_key: ${{ secrets.GATEWAY_API_KEY }}
```

对于 bearer 令牌网关，将相同的密钥作为 `anthropic_api_key` 输入和工作流 `env` 块中的 `ANTHROPIC_AUTH_TOKEN` 传递。操作在启动 Claude Code 之前需要 `anthropic_api_key`、`CLAUDE_CODE_OAUTH_TOKEN` 或工作负载身份联合，并且它不读取 `ANTHROPIC_AUTH_TOKEN`，因此输入满足该启动检查。env 变量是将密钥放在网关读取的 `Authorization` 标头中的内容；`x-api-key` 中的副本被忽略：

```yaml theme={null}
env:
  ANTHROPIC_BASE_URL: https://llm-gateway.example.com
  ANTHROPIC_AUTH_TOKEN: ${{ secrets.GATEWAY_API_KEY }}

steps:
  - uses: anthropics/claude-code-action@v1
    with:
      anthropic_api_key: ${{ secrets.GATEWAY_API_KEY }}
```

对于操作的其他身份验证选项，包括 `CLAUDE_CODE_OAUTH_TOKEN` 和工作负载身份联合，请参阅 [Claude Code GitHub Actions](https://code.claude.com/docs/zh-CN/github-actions.md) 和操作的 [README](https://github.com/anthropics/claude-code-action#readme)。

### Agent SDK

[Agent SDK](https://code.claude.com/docs/zh-CN/agent-sdk/overview.md) 没有网关特定的选项；它将环境变量传递给它生成的 Claude Code 进程。每个 SDK 接受设置生成进程环境的 `env` 选项，TypeScript 和 Python SDK 以不同方式处理它：

* TypeScript：生成的进程默认继承父环境，但设置 `options.env` 完全替换环境。将 `process.env` 扩展到其中以保留您的网关变量。
* Python：`ClaudeAgentOptions(env=...)` 合并到继承的环境之上，因此在父进程中设置的网关变量无需扩展即可通过。

```ts TypeScript theme={null}
const result = query({
  prompt: "...",
  options: {
    env: {
      ...process.env,
      ANTHROPIC_BASE_URL: "https://llm-gateway.example.com",
      ANTHROPIC_AUTH_TOKEN: process.env.GATEWAY_KEY,
    },
  },
})
```

```python Python theme={null}
options = ClaudeAgentOptions(
    env={
        "ANTHROPIC_BASE_URL": "https://llm-gateway.example.com",
        "ANTHROPIC_AUTH_TOKEN": os.environ["GATEWAY_KEY"],
    }
)
```

### Slack、云会话和远程控制

[Slack 中的 Claude Code](https://code.claude.com/docs/zh-CN/slack.md) 和[云端会话](https://code.claude.com/docs/zh-CN/claude-code-on-the-web.md)不属于网关部署的一部分。在云端会话的环境配置中设置的网关变量不会生效。如果您的流量必须保持在网关上，请不要为这些用户启用这些使用入口。

[Remote Control](https://code.claude.com/docs/zh-CN/remote-control.md) 和[语音听写](https://code.claude.com/docs/zh-CN/voice-dictation.md)都依赖于 claude.ai 身份：Remote Control 将实时会话与您的账户配对，语音听写到达 claude.ai 转录端点。当 `ANTHROPIC_API_KEY`、`ANTHROPIC_AUTH_TOKEN` 或 `apiKeyHelper` 处于活动状态时，它们不可用。Remote Control 在 `ANTHROPIC_BASE_URL` 指向非 Anthropic 主机时也被禁用，因此仅使用 claude.ai 登录是不够的。

要恢复任一功能，请使用 claude.ai 登录并取消设置该功能检查的网关变量。`claude doctor` 的远程控制部分命名当前阻止远程控制的内容。

* 语音听写：取消设置网关凭证
* 远程控制：取消设置网关凭证和 `ANTHROPIC_BASE_URL`

## 其他配置

这些设置涵盖超出基础 URL 和凭证的情况。仅在您的管理员的说明、您的网络的出站规则或[故障排除表](#troubleshoot-gateway-errors)要求一个时设置它们。

### 发送其他标头

某些网关使用除凭证外的自定义标头来路由或标记请求，例如租户标识符或路由密钥。要发送一个，请设置 [`ANTHROPIC_CUSTOM_HEADERS`](https://code.claude.com/docs/zh-CN/env-vars)，每行一个 `Name: Value` 对。下面的示例添加了一个名为 `X-Org-Route` 的路由标头：

> [tab] Bash or Zsh
>
> ```bash theme={null}
> export ANTHROPIC_CUSTOM_HEADERS="X-Org-Route: prod"
> ```

> [tab] PowerShell
>
> ```powershell theme={null}
> $env:ANTHROPIC_CUSTOM_HEADERS = "X-Org-Route: prod"
> ```

您也可以在设置文件的 `env` 块中设置 `ANTHROPIC_CUSTOM_HEADERS`。在那里使用 `\n` 在对之间，因为 JSON 字符串不能跨多行：

```json theme={null}
{
  "env": {
    "ANTHROPIC_CUSTOM_HEADERS": "X-Org-Route: prod\nX-Tenant: example"
  }
}
```

像这样的路由和租户标头名称计为[需要批准的标头](https://code.claude.com/docs/zh-CN/server-managed-settings.md#environment-variables-and-the-approval-dialog)。当标头来自项目设置文件时，Claude Code 在[应用 `env` 值的规则](https://code.claude.com/docs/zh-CN/settings-reference#when-claude-code-applies-env-values)下应用它们。

### 将网关模型添加到模型选择器

启用模型发现后，Claude Code 在启动时查询网关的模型列表，并将这些名称添加到 `/model` 选择器中，与内置条目一起。如果您或您的管理员在 [`modelPicker`](https://code.claude.com/docs/zh-CN/settings-reference#modelpicker) 排列中设置了 `replaceBuiltInOptions`，Claude Code 也会隐藏发现的名称。它为会话已在使用的模型保留一行。

如果您的网关提供不在 Claude Code 内置列表中的模型名称，并且您想从选择器中选择它们，请启用它。如果内置模型是您使用的，您不需要发现；您的管理员也可能已通过托管设置启用它。

要启用它，请在您的 shell 或 `~/.claude/settings.json` 的 `env` 块中设置 `CLAUDE_CODE_ENABLE_GATEWAY_MODEL_DISCOVERY=1`。

发现的模型显示为其他 `/model` 条目。每个条目显示您的网关为模型提供的描述，或在它不提供描述时显示 `From gateway`。

要确认发现运行，启动 `claude --debug` 并在 `~/.claude/debug/<session-id>.txt` 的调试日志中查找 `[gatewayDiscovery]` 行。第一次发现成功时，Claude Code 记录它缓存了多少模型，仅当网关的列表更改时才再次记录。`404`、超时或重定向也会出现在那里。有关发现何时运行、它过滤什么以及网关提供的响应格式，请参阅[模型发现参考](https://code.claude.com/docs/zh-CN/llm-gateway-protocol.md#model-discovery)。

### 使用 apiKeyHelper 轮换凭证

`apiKeyHelper` 是 Claude Code 运行以获取您的网关凭证的命令，而不是从静态环境变量读取它。

当凭证按计划过期、来自保管库或 SSO 命令，或您的管理员告诉您配置一个时，使用助手。如果您的凭证是您设置一次的固定字符串，[凭证变量](#set-the-credential-variable)是您需要的全部，您可以跳过本部分。

助手是任何将当前凭证打印到 stdout 的 shell 命令。Claude Code 通过您的系统 shell 运行它，因此在 Windows 上它可以是可执行文件或 PowerShell 调用。使命令仅打印凭证，不打印其他内容。在 Claude Code v2.1.227 或更高版本上，与密钥一起打印的横幅或日志行会使[助手失败](https://code.claude.com/docs/zh-CN/errors.md#your-apikeyhelper-script-is-failing)。编写脚本，使其可执行，并从您的[设置文件](https://code.claude.com/docs/zh-CN/settings.md)中的 `apiKeyHelper` 引用它：

> [tab] Bash or Zsh
>
> 例如，从保管库读取的脚本：
>
> ```bash theme={null}
> #!/bin/bash
> vault kv get -field=api_key secret/llm-gateway/claude-code
> ```
>
> 在 `~/.claude/settings.json` 中引用其路径：
>
> ```json theme={null}
> {
>   "apiKeyHelper": "~/bin/get-gateway-key.sh"
> }
> ```

> [tab] PowerShell
>
> 例如，从保管库读取的脚本：
>
> ```powershell theme={null}
> vault kv get -field=api_key secret/llm-gateway/claude-code
> ```
>
> 在 `%USERPROFILE%\.claude\settings.json` 中引用 PowerShell 调用，转义 JSON 字符串中的反斜杠：
>
> ```json theme={null}
> {
>   "apiKeyHelper": "powershell -NoProfile -File C:\\scripts\\get-gateway-key.ps1"
> }
> ```

Claude Code 默认缓存助手的输出五分钟，并在缓存生命周期过期后重新运行助手。要更改生命周期，请以毫秒为单位设置 `CLAUDE_CODE_API_KEY_HELPER_TTL_MS`，例如 `CLAUDE_CODE_API_KEY_HELPER_TTL_MS=900000` 表示 15 分钟。

有关 Claude Code 重新运行助手的其他情况，请参阅 [`apiKeyHelper`](https://code.claude.com/docs/zh-CN/settings-reference#apikeyhelper)。

助手的值在 `Authorization` 和 `x-api-key` 标头中都发送，因此它适用于您的网关读取的任何标头。

### 关闭网关路径外的流量

网关承载模型请求，但 Claude Code 也向网关路径外发送非必要的后台流量，发送到 Anthropic 和第三方服务（如 GitHub）：版本检查、遥测、发行说明和类似请求。在仅允许出站到网关的网络上，这些请求失败，并可能在您的出站监控中显示为被阻止的连接。

要关闭该流量，请在与网关变量相同的 shell 导出或设置文件 `env` 块中设置 `CLAUDE_CODE_DISABLE_NONESSENTIAL_TRAFFIC=1`：

> [tab] Bash or Zsh
>
> ```bash theme={null}
> export CLAUDE_CODE_DISABLE_NONESSENTIAL_TRAFFIC=1
> ```

> [tab] PowerShell
>
> ```powershell theme={null}
> $env:CLAUDE_CODE_DISABLE_NONESSENTIAL_TRAFFIC = "1"
> ```

设置该变量具有以下效果和限制：

* 它禁用自动更新，因此请为另一个更新路径做计划，例如您的包管理器或托管分发。
* 它抑制 [fast mode](https://code.claude.com/docs/zh-CN/fast-mode.md) 可用性检查。除非之前的检查已在机器上启用了 fast mode，否则 `/fast` 报告 fast mode 不可用。
* 它不影响[网关模型发现](#add-gateway-models-to-the-model-picker)，它仅查询您的网关。在 v2.1.257 之前，该变量也停止了发现刷新，因此选择器保留了之前缓存的列表。
* WebFetch 工具的[域安全检查](https://code.claude.com/docs/zh-CN/data-usage.md#webfetch-domain-safety-check)不受影响，仍然调用 `api.anthropic.com`。如果您的网络阻止该主机，请在[设置](https://code.claude.com/docs/zh-CN/settings.md)中使用 `skipWebFetchPreflight: true` 单独关闭它。
* 对于每个遥测流及控制它的变量，请参阅[遥测服务](https://code.claude.com/docs/zh-CN/data-usage.md#telemetry-services)。

### 通过网关路由到云提供商

这些配置使用提供商特定的基础 URL 变量代替 `ANTHROPIC_BASE_URL` 将 Claude Code 指向通过网关的云提供商。Amazon Bedrock 和 Google Cloud 的 Agent Platform 网关接受这些提供商的本机请求格式；Microsoft Foundry 和 AWS 上的 Claude Platform 网关接受 Anthropic Messages 格式。在 Amazon Bedrock 和 Google Cloud 的 Agent Platform 路由上，Claude Code 也将它发送的 beta 标头和请求字段限制为该提供商接受的集合。有关您的网关在每条路由上接收的内容，请参阅[网关兼容性指南](https://code.claude.com/docs/zh-CN/llm-gateway-protocol.md)。

仅在您的网关团队特别命名 Amazon Bedrock、Google Cloud 的 Agent Platform、Microsoft Foundry 或 AWS 上的 Claude Platform 时使用一个。如果上面的[验证请求](#verify-the-connection)返回 JSON，您可以跳过本部分。

为您的网关团队命名的提供商设置块。Amazon Bedrock、Google Cloud 的 Agent Platform 和 AWS 上的 Claude Platform 块中的跳过身份验证变量告诉 Claude Code 不要使用提供商凭证签署请求，因为网关持有这些。如果网关也需要自己的令牌，您放置它的位置取决于提供商：

* **Amazon Bedrock、Google Cloud 的 Agent Platform 或 AWS 上的 Claude Platform**：在块后添加 `ANTHROPIC_AUTH_TOKEN`。Claude Code 将其作为 `Authorization: Bearer` 标头发送到网关。对于不同方案或标头中的凭证，请改用 [`ANTHROPIC_CUSTOM_HEADERS`](#send-additional-headers)。无论如何都保持跳过身份验证变量设置，因为没有它，Claude Code 会删除 `ANTHROPIC_AUTH_TOKEN`、[`apiKeyHelper`](#rotate-credentials-with-apikeyhelper) 或 `ANTHROPIC_CUSTOM_HEADERS` 会添加的任何 `Authorization` 标头。
* **Microsoft Foundry**：使用 `ANTHROPIC_FOUNDRY_API_KEY`，如[其块](#microsoft-foundry)所示

#### Amazon Bedrock

当网关发出自己的凭证时，将 `AWS_BEARER_TOKEN_BEDROCK` 保留为未设置。如果您设置它，Claude Code 会将该 [Amazon Bedrock API 密钥](https://code.claude.com/docs/zh-CN/amazon-bedrock.md#2-configure-aws-credentials)作为 `Authorization` 标头发送，而不是您的网关令牌，即使设置了 `CLAUDE_CODE_SKIP_BEDROCK_AUTH`。

> [tab] Bash or Zsh
>
> ```bash theme={null}
> export ANTHROPIC_BEDROCK_BASE_URL=https://llm-gateway.example.com/bedrock
> export CLAUDE_CODE_SKIP_BEDROCK_AUTH=1
> export CLAUDE_CODE_USE_BEDROCK=1
> ```

> [tab] PowerShell
>
> ```powershell theme={null}
> $env:ANTHROPIC_BEDROCK_BASE_URL = "https://llm-gateway.example.com/bedrock"
> $env:CLAUDE_CODE_SKIP_BEDROCK_AUTH = "1"
> $env:CLAUDE_CODE_USE_BEDROCK = "1"
> ```

#### Google Cloud 的 Agent Platform

将项目 ID 和区域替换为您自己的值。Claude Code 在它发送到网关的每个请求的路径中包含两者：

> [tab] Bash or Zsh
>
> ```bash theme={null}
> export ANTHROPIC_VERTEX_BASE_URL=https://llm-gateway.example.com/vertex
> export ANTHROPIC_VERTEX_PROJECT_ID=your-gcp-project-id
> export CLAUDE_CODE_SKIP_VERTEX_AUTH=1
> export CLAUDE_CODE_USE_VERTEX=1
> export CLOUD_ML_REGION=us-east5
> ```

> [tab] PowerShell
>
> ```powershell theme={null}
> $env:ANTHROPIC_VERTEX_BASE_URL = "https://llm-gateway.example.com/vertex"
> $env:ANTHROPIC_VERTEX_PROJECT_ID = "your-gcp-project-id"
> $env:CLAUDE_CODE_SKIP_VERTEX_AUTH = "1"
> $env:CLAUDE_CODE_USE_VERTEX = "1"
> $env:CLOUD_ML_REGION = "us-east5"
> ```

该块涵盖路由和身份验证。来自 [Agent Platform 设置](https://code.claude.com/docs/zh-CN/google-vertex-ai.md#4-configure-claude-code)的区域覆盖和模型固定也通过网关应用：

* **按模型区域**：如果您的网关从 `CLOUD_ML_REGION` 以外的区域提供某些模型，请为每个设置匹配的 `VERTEX_REGION_CLAUDE_*` 变量，例如 `VERTEX_REGION_CLAUDE_4_6_SONNET=europe-west1`。[环境变量参考](https://code.claude.com/docs/zh-CN/env-vars.md)列出了确切的名称。
* **模型版本**：如[固定模型版本](https://code.claude.com/docs/zh-CN/google-vertex-ai.md#5-pin-model-versions)中所示，固定 `ANTHROPIC_DEFAULT_OPUS_MODEL`、`ANTHROPIC_DEFAULT_SONNET_MODEL` 和 `ANTHROPIC_DEFAULT_HAIKU_MODEL`。设置 `ANTHROPIC_DEFAULT_HAIKU_MODEL` 也会将后台任务（如会话标题）移动到该模型，该部分解释了哪个模型在其他情况下运行它们。
* **模型功能**：如果您固定您的 Claude Code 版本不识别的模型 ID，功能（如努力级别或扩展思考）可能在其上保持禁用。使用 [`ANTHROPIC_DEFAULT_OPUS_MODEL_SUPPORTED_CAPABILITIES`](https://code.claude.com/docs/zh-CN/model-config#customize-pinned-model-display-and-capabilities) 及其 Sonnet 和 Haiku 对应项声明模型支持的内容。

#### Microsoft Foundry

将网关的凭证放在 `ANTHROPIC_FOUNDRY_API_KEY` 中；它作为 `x-api-key` 标头发送到网关。期望持有者令牌的网关可以改用 [`ANTHROPIC_FOUNDRY_AUTH_TOKEN`](https://code.claude.com/docs/zh-CN/env-vars)。Claude Code 将该值作为 `Authorization: Bearer` 标头发送，当两者都设置时，它优先于 `ANTHROPIC_FOUNDRY_API_KEY`。需要 Claude Code v2.1.203 或更高版本。

对于注入自己的 `Authorization` 标头的网关，设置 `CLAUDE_CODE_SKIP_FOUNDRY_AUTH=1` 并将两个凭据变量都保留为未设置。Claude Code 然后发送没有 Azure 凭据的请求，并保留您提供的 `Authorization` 标头，例如通过 `ANTHROPIC_CUSTOM_HEADERS`。

> [tab] Bash or Zsh
>
> ```bash theme={null}
> export ANTHROPIC_FOUNDRY_BASE_URL=https://llm-gateway.example.com/foundry
> export ANTHROPIC_FOUNDRY_API_KEY=sk-gateway-key
> export CLAUDE_CODE_USE_FOUNDRY=1
> ```

> [tab] PowerShell
>
> ```powershell theme={null}
> $env:ANTHROPIC_FOUNDRY_BASE_URL = "https://llm-gateway.example.com/foundry"
> $env:ANTHROPIC_FOUNDRY_API_KEY = "sk-gateway-key"
> $env:CLAUDE_CODE_USE_FOUNDRY = "1"
> ```

#### AWS 上的 Claude Platform

有关工作区 ID，请参阅 [AWS 上的 Claude Platform](https://code.claude.com/docs/zh-CN/claude-platform-on-aws.md)。

> [tab] Bash or Zsh
>
> ```bash theme={null}
> export ANTHROPIC_AWS_BASE_URL=https://llm-gateway.example.com/anthropic-aws
> export ANTHROPIC_AWS_WORKSPACE_ID=wrkspc_01ABCDEFGHIJKLMN
> export CLAUDE_CODE_SKIP_ANTHROPIC_AWS_AUTH=1
> export CLAUDE_CODE_USE_ANTHROPIC_AWS=1
> ```

> [tab] PowerShell
>
> ```powershell theme={null}
> $env:ANTHROPIC_AWS_BASE_URL = "https://llm-gateway.example.com/anthropic-aws"
> $env:ANTHROPIC_AWS_WORKSPACE_ID = "wrkspc_01ABCDEFGHIJKLMN"
> $env:CLAUDE_CODE_SKIP_ANTHROPIC_AWS_AUTH = "1"
> $env:CLAUDE_CODE_USE_ANTHROPIC_AWS = "1"
> ```

#### 确认提供商路由

从您设置块的 shell 启动 `claude` 并运行 `/status`。使用 Amazon Bedrock 块，**Status** 标签页显示如下行：

```text theme={null}
API provider: Amazon Bedrock
Bedrock base URL: https://llm-gateway.example.com/bedrock
AWS auth skipped
```

其他块在其提供商的名称下产生相同的行，例如 Google Cloud 的 Agent Platform 的 `Vertex base URL` 和 `GCP auth skipped`；Microsoft Foundry 块仅在您设置 `CLAUDE_CODE_SKIP_FOUNDRY_AUTH` 时显示跳过身份验证行。如果您也通过公司代理路由，`Proxy` 行显示代理 URL。如果基础 URL 行缺失，该变量没有到达会话。

## 故障排除网关错误

这些是通过网关运行 Claude Code 时最常见的错误，包括网关端的原因和修复：

| 错误 | 原因 | 修复 |
| :- | :- | :- |
| 启动警告命名两个凭证源并以 `auth may not work as expected` 结尾。较旧的版本显示 `Auth conflict: Both a token (SOURCE) and an API key (SOURCE) are set` 代替。 | 网关凭证和保存的登录都处于活动状态；变量用于请求，但过时的登录可能导致意外的身份验证行为 | 取消设置变量以使用保存的登录，或运行 `/logout` 以使用网关凭证 |
| `401` 错误命名无效或无法识别的令牌 | 凭证不是网关颁发的，或它处于网关不读取的标头中 | 确认变量与[凭证表](#set-the-credential-variable)中的凭证类型匹配，如果凭证被撤销，请在网关处重新生成密钥 |
| `Your apiKeyHelper script is failing`，或在非交互模式下 stderr 上的 `apiKeyHelper failed:` | [`apiKeyHelper`](https://code.claude.com/docs/zh-CN/settings-reference#apikeyhelper) 设置中的命令未生成可用的密钥，因此请求携带占位符密钥 | 直接运行该命令以查看失败原因，如果报告会话过期，请使用您的凭证提供商重新身份验证；请参阅[错误参考](https://code.claude.com/docs/zh-CN/errors.md#your-apikeyhelper-script-is-failing) |
| 当没有任何东西在地址处应答时 `Connection refused — a firewall or proxy may be blocking it (ConnectionRefused)`，或当主机名无法解析时 `Can't reach the API server — check your internet or DNS (ENOTFOUND)`，通常在 Claude Code [使用退避重试](https://code.claude.com/docs/zh-CN/errors.md#automatic-retries)时的静默暂停之后。括号中的代码会变化；[Unable to connect to API](https://code.claude.com/docs/zh-CN/errors.md#unable-to-connect-to-api) 涵盖代码拼写和较早的措辞 | 没有任何东西在基础 URL 处应答：地址错误，或 VPN 或防火墙阻止了网关的路径 | 运行上面的 [curl 测试](#verify-the-connection)，它会立即因相同原因失败，并与您的网关团队确认 URL 和网络路径 |
| `API returned an empty or malformed response (HTTP 200)` | 网关或中间代理返回了非 API 响应，通常是 HTML 错误或登录页面 | 使用上面的 [curl 请求](#verify-the-connection)测试；修复返回非 Claude API 响应的网关路由。[错误参考](https://code.claude.com/docs/zh-CN/errors.md#api-returned-an-empty-or-malformed-response)解释了消息报告的详细信息 |
| `400` 错误命名 `context_management`、`Extra inputs are not permitted` 或其他无法识别的字段 | 网关将请求转发到上游，该上游拒绝 Claude Code 发送到 Anthropic 格式端点的字段 | 设置 `CLAUDE_CODE_DISABLE_EXPERIMENTAL_BETAS=1`，它抑制大多数预发布字段；请参阅[功能传递](https://code.claude.com/docs/zh-CN/llm-gateway-protocol.md#feature-pass-through)。某些 beta 不受此标志限制；对于那些，设置匹配的 `CLAUDE_CODE_USE_*` 提供商变量，以便 Claude Code 仅发送该提供商接受的内容。有关取决于消息所指字段的步骤，请参阅 [Extra inputs are not permitted](https://code.claude.com/docs/zh-CN/errors.md#extra-inputs-are-not-permitted) |
| `400` 错误命名 `thinking` 或 `adaptive`，例如 `Input tag 'adaptive' found` | 上游模型构建不接受自适应推理，而 Claude Code 会为 Claude Opus 4.6、Claude Sonnet 4.6、Claude Haiku 5.5、Claude Fable 5 及更高版本的模型请求自适应推理 | 升级网关的上游。在 Opus 4.6 和 Sonnet 4.6 上，`CLAUDE_CODE_DISABLE_ADAPTIVE_THINKING=1` 代替有效。[模型配置](https://code.claude.com/docs/zh-CN/model-config.md)能力变量仅适用于提供商配置，例如 `CLAUDE_CODE_USE_BEDROCK` 和 `CLAUDE_CODE_USE_VERTEX`，不在 `ANTHROPIC_BASE_URL` 网关后面 |
| `400` 错误声明网关自己的措辞中的上下文或令牌限制，例如 `ContextWindowExceededError` 或 `prompt token count of N exceeds the limit of M` | 网关强制执行比模型的本机窗口更小的上下文，并重写上游错误，因此 Claude Code 不会将其识别为[过长错误](https://code.claude.com/docs/zh-CN/errors.md#prompt-is-too-long)，也不会自动紧凑和重试 | 运行 `/compact` 以恢复会话。要防止它，请将 `CLAUDE_CODE_AUTO_COMPACT_WINDOW` 设置为网关的限制；Claude Code 将该值限制在至少 100,000 令牌和最多模型的上下文窗口，因此您无法匹配低于 100,000 的网关限制，`/compact` 仍然是那里的恢复。还要将 `CLAUDE_CODE_MAX_OUTPUT_TOKENS` 设置为低于网关模型的输出限制 |
| `400` 错误在每个请求上，在网关自己的措辞中拒绝工具的输入架构或其 `pattern`，在 Claude Code v2.1.265 到 v2.1.267 上 | 在这些版本的逐步推出中，[Artifact 工具](https://code.claude.com/docs/zh-CN/artifacts.md#availability)架构携带带有 `\p{...}` Unicode 字符类的正则表达式。Anthropic API 接受它，但检查每个工具架构的 `pattern` 的网关或上游使用其自己的正则表达式引擎拒绝整个请求 | 更新到 v2.1.268 或更高版本，它不发送正则表达式。在受影响的版本上，[关闭 artifacts](https://code.claude.com/docs/zh-CN/artifacts.md#disable-artifacts)，这会从请求中删除工具及其架构 |
| `400` 错误在每个请求上，在网关自己的措辞中拒绝无法识别的工具类型，例如 `Input tag 'advisor_20260301'`，在 Claude Code v2.1.275 上 | 在该版本的逐步推出中，即使关闭了 advisor，请求也会携带 [advisor 工具](https://code.claude.com/docs/zh-CN/advisor.md)条目。Anthropic API 接受它，但验证工具类型的网关或上游拒绝整个请求；一个[按原样转发请求正文字段](https://code.claude.com/docs/zh-CN/llm-gateway-protocol.md#forward-as-open-lists)的网关不受影响地通过它。该条目是一个不携带任何对话内容的声明 | 更新到 v2.1.276 或更高版本，除非您打开 advisor，否则它不会在 `ANTHROPIC_BASE_URL` 网关后面发送该条目。在 v2.1.275 上，设置 [`CLAUDE_CODE_DISABLE_ADVISOR_TOOL=1`](https://code.claude.com/docs/zh-CN/env-vars)，这会从请求中删除该条目 |
| 模型从 `/model` 选择器中缺失 | 网关模型名称不在 Claude Code 的内置列表中，或 Claude Code 显示替换内置选项的 [`modelPicker`](https://code.claude.com/docs/zh-CN/settings-reference#modelpicker) 阵容 | 启用[网关模型发现](#add-gateway-models-to-the-model-picker)或使用[模型配置](https://code.claude.com/docs/zh-CN/model-config.md)变量添加名称。如果 Claude Code 显示替换 `modelPicker` 阵容，请将网关模型添加到其中，或在托管设置提供时要求您的管理员添加它们 |
| `/fast` 报告 `Fast mode unavailable due to network connectivity issues`，而推理请求有效 | [快速模式](https://code.claude.com/docs/zh-CN/fast-mode.md)可用性检查直接转到 `api.anthropic.com`，不遵循 `ANTHROPIC_BASE_URL`，因此阻止的直接出口会导致检查失败 | 如果出口被阻止，请将 `api.anthropic.com` 加入允许列表，或设置跳过变量。请参阅[在代理和 LLM 网关后面使用快速模式](https://code.claude.com/docs/zh-CN/fast-mode.md#use-fast-mode-behind-proxies-and-llm-gateways) |
| `/fast` 在使用 `ANTHROPIC_API_KEY` 中网关颁发的密钥进行身份验证的会话中报告 `Fast mode is currently unavailable` | 当 `ANTHROPIC_BASE_URL` 指向您的网关且密钥不符合 Anthropic API 密钥的格式时，Claude Code 不会发送[快速模式](https://code.claude.com/docs/zh-CN/fast-mode.md)可用性检查。在没有缓存先前成功检查结果的情况下，它会将快速模式视为不可用 | 设置 `CLAUDE_CODE_SKIP_FAST_MODE_NETWORK_ERRORS=1`；请参阅[在代理和 LLM 网关后面使用快速模式](https://code.claude.com/docs/zh-CN/fast-mode.md#use-fast-mode-behind-proxies-and-llm-gateways) |
| `/fast` 在使用 `ANTHROPIC_AUTH_TOKEN` 进行身份验证的会话中报告 `Fast mode has been disabled by your organization`，即使组织已启用快速模式 | 可用性检查需要 claude.ai 登录或 API 密钥；仅使用持有者令牌，Claude Code 会将快速模式视为已禁用，而不发送检查 | 设置 `CLAUDE_CODE_SKIP_FAST_MODE_ORG_CHECK=1`；请参阅[在代理和 LLM 网关后面使用快速模式](https://code.claude.com/docs/zh-CN/fast-mode.md#use-fast-mode-behind-proxies-and-llm-gateways) |
| Claude Code 要求您登录，即使 [curl 测试](#verify-the-connection)成功 | CLI 没有自己的凭证：可达的基础 URL 不是一个，在交互会话中，项目的 `.claude/settings.json` 或 `.claude/settings.local.json` 中的 `env` 块仅在首次运行向导和[信任提示](https://code.claude.com/docs/zh-CN/permissions.md#what-runs-before-you-trust-a-folder)之后应用 | 在 Claude Code 在首次运行设置之前读取的某处设置 `ANTHROPIC_AUTH_TOKEN`：shell 导出、`~/.claude/settings.json` 中的 `env` 块或托管设置 |
| `ANTHROPIC_API_KEY` 已设置但被忽略，没有提示 | 密钥需要在交互会话中进行一次性批准，之前拒绝的密钥被忽略而不再询问 | 在 `/config` 下使用 `Use custom API key` 选项启用它 |
| `This machine's managed settings require a first-party login`，或当托管设置将 `forceLoginMethod` 设置为 `"gateway"` 或也设置 `forceLoginGatewayUrl` 时 [`Administrator policy requires a Cloud gateway sign-in`](https://code.claude.com/docs/zh-CN/errors#administrator-policy-requires-a-cloud-gateway-sign-in) | 托管设置包括 `forceLoginMethod` 或 `forceLoginOrgUUID`，不能与 `ANTHROPIC_API_KEY`、`ANTHROPIC_AUTH_TOKEN` 或 `apiKeyHelper` 共存 | 您的管理员必须从托管设置中删除 `forceLoginMethod`、`forceLoginOrgUUID` 和 `forceLoginGatewayUrl` 以使用网关凭证，或删除网关凭证以使用托管设置要求的登录。两者不能组合 |
| `403` 带有 HTML 正文，例如 `403 Forbidden`，当网关自己的日志显示没有收到请求时 | 网关前面的 Web 应用防火墙或反向代理在请求到达网关之前阻止了请求正文。Claude Code 提示包括 XML 样式标签和与跨站脚本正文规则匹配的源代码，因此短 curl 测试通过而实际会话不通过 | 从请求正文检查中豁免网关的 `/v1/messages` 路径。在 AWS WAF 上这是 `CrossSiteScripting_Body` 托管规则；在带有 ModSecurity 的 nginx 上它是等效的 OWASP CRS 正文规则 |
| 证书或 TLS 错误，例如 `SSL certificate verification failed` 或 `Self-signed certificate detected`，当 [curl 测试](#verify-the-connection)成功时 | Claude Code 的运行时不信任 `curl` 使用的相同证书颁发机构。常见于企业 TLS 检查代理后面 | 将 `NODE_EXTRA_CA_CERTS` 设置为 CA 包路径；请参阅 [CA 证书存储](https://code.claude.com/docs/zh-CN/network-config.md#ca-certificate-store) |

如果 Claude Code 在删除网关配置后重复提示您登录，原因通常是凭证存储而不是网关；请参阅[身份验证错误](https://code.claude.com/docs/zh-CN/errors.md#authentication-errors)。

## 相关资源

* [LLM 网关概述](https://code.claude.com/docs/zh-CN/llm-gateway)：什么是网关以及它如何与 claude.ai 订阅交互
* [为您的组织推出 LLM 网关](https://code.claude.com/docs/zh-CN/llm-gateway-rollout.md)：部署和分发网关配置的面向管理员的检查清单
* [网关兼容性指南](https://code.claude.com/docs/zh-CN/llm-gateway-protocol.md)：Claude Code 发送到网关的内容，包括网关必须转发的标头和字段
* [设置](https://code.claude.com/docs/zh-CN/settings.md)：设置文件的位置以及如何读取 `env` 块
* [身份验证](https://code.claude.com/docs/zh-CN/authentication.md)：凭证变量、`apiKeyHelper` 和 OAuth 登录如何交互
