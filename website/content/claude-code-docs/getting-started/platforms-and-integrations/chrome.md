> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# 在 Chrome 中使用 Claude Code

> 将 Claude Code 连接到 Chrome 浏览器，以测试网络应用、使用控制台日志进行调试、自动填充表单以及从网页中提取数据。

Claude Code 与 [Claude in Chrome 浏览器扩展程序](https://chromewebstore.google.com/detail/claude/fcoeoabgfenejglbffodgkkbkcdhcgfn) 集成，为您提供从 CLI 或 [VS Code 扩展程序](https://code.claude.com/docs/zh-CN/vs-code.md#automate-browser-tasks-with-chrome) 进行浏览器自动化的功能。构建您的代码，然后在浏览器中测试和调试，无需切换上下文。

Claude 为浏览器任务打开新标签页，并共享您浏览器的登录状态，因此它可以访问您已登录的任何网站。浏览器操作在实时可见的 Chrome 窗口中运行。当 Claude 遇到登录页面或 CAPTCHA 时，它会暂停并要求您手动处理。

扩展程序将 Claude 打开的标签页收集到与您的会话相关联的 Chrome 标签页组中。在本地会话中，Claude Code 是否在会话结束时关闭该组取决于会话如何结束：

* 当您输入 `/clear` 时，Claude Code 会关闭该组（包括打开的页面），除非仍在运行的工作在清除后仍然存在
* 当您使用 `/resume` 等命令切换会话、退出 Claude Code 或在仍在运行的工作中运行 `/clear` 时，Claude Code 仅在该组仅包含空的新标签页时才关闭该组，因此您可能仍在阅读的页面保持打开状态

> [!NOTE]
>
> Chrome 集成适用于 Google Chrome 和 Microsoft Edge。Claude Code 还会检测扩展程序并在其他基于 Chromium 的浏览器中设置连接，包括 Brave、Arc、Vivaldi 和 Opera。Windows 子系统 for Linux (WSL) 不支持 Chrome 集成。

## 功能

连接 Chrome 后，您可以在单个工作流中链接浏览器操作和编码任务：

* **实时调试**：直接读取控制台错误和 DOM 状态，然后修复导致这些错误的代码
* **设计验证**：从 Figma 模型构建 UI，然后在浏览器中打开它以验证它是否匹配
* **网络应用测试**：测试表单验证、检查视觉回归或验证用户流程
* **已认证的网络应用**：与 Google Docs、Gmail、Notion 或您已登录的任何应用交互，无需 API 连接器
* **数据提取**：从网页中提取结构化信息并将其保存到本地
* **任务自动化**：自动化重复的浏览器任务，如数据输入、表单填充或多站点工作流
* **文件上传**：将您计算机中的文件附加到网页上的上传字段
* **会话录制**：将浏览器交互录制为 GIF，以记录或分享发生的情况

## 前提条件

在使用 Claude Code 与 Chrome 之前，您需要：

* [Google Chrome](https://www.google.com/chrome/)、[Microsoft Edge](https://www.microsoft.com/edge) 或其他基于 Chromium 的浏览器，如 Brave、Arc、Vivaldi 或 Opera
* [Claude in Chrome 扩展程序](https://chromewebstore.google.com/detail/claude/fcoeoabgfenejglbffodgkkbkcdhcgfn) 版本 1.0.36 或更高版本，可在 Chrome Web Store 中获得
* [Claude Code](https://code.claude.com/docs/zh-CN/quickstart.md#step-1-install-claude-code)
* 直接 Anthropic 计划（Pro、Max、Ultra、Team 或 Enterprise）

在已启用 HIPAA 的 Enterprise 组织中，Claude in Chrome 默认处于关闭状态，[Owner](https://code.claude.com/docs/zh-CN/server-managed-settings.md#access-control) 可以在 [**Organization settings > Claude in Chrome**](https://claude.ai/admin-settings/browser-extension) 中将其启用。您与 Anthropic 签订的商业伙伴协议（BAA）不涵盖通过 Claude in Chrome 发送到第三方网站的数据。有关合格服务（Eligible Services）的列表，请参阅[实施指南](https://trust.anthropic.com/resources?s=l1wrssd9hsbi4gak0tp5a6\&name=%5Banthropic%5D-hipaa-ready-offering-implementation-guide)。

Chrome 集成还需要使用 `/login` 登录。如果您使用 API 密钥或来自 [`claude setup-token`](https://code.claude.com/docs/zh-CN/authentication#generate-a-long-lived-token) 的长期令牌进行身份验证，即使您传递 `--chrome`，Claude Code 也会保持 Chrome 集成处于关闭状态。浏览器扩展程序无法使用这些凭据进行身份验证。

> [!NOTE]
>
> Chrome 集成不可通过 Amazon Bedrock、Google Cloud 的 Agent Platform 或 Microsoft Foundry 等第三方提供商获得。如果您仅通过第三方提供商访问 Claude，则需要单独的 claude.ai 账户来使用此功能。

## 在 CLI 中开始使用

**1. 使用 Chrome 启动 Claude Code**

使用 `--chrome` 标志启动 Claude Code：

```bash theme={null}
claude --chrome
```

首次使用 Chrome 启动时，Claude Code 会显示一个一次性对话框，介绍该集成并说明站点权限的工作方式。按 Enter 继续。

如需在以后的会话中无需该标志即可启用 Chrome，请参阅[默认启用 Chrome](#enable-chrome-by-default)。

**2. 让 Claude 使用浏览器**

此示例会导航到一个页面、与之交互并报告发现的内容，所有操作都可以在终端或编辑器中完成：

```text wrap theme={null}
Go to code.claude.com/docs, click on the search box,
type "hooks", and tell me what results appear
```

如果 Claude Code 在执行浏览器操作前请求权限，请批准。该对话框以 `Claude in Chrome wants to` 开头，并提供在本次会话中允许该站点上所有操作的选项。Claude 会打开一个新标签页并开始执行任务。

随时运行 `/chrome` 可检查连接状态、管理权限、重新连接扩展程序，或选择要使用的已连接浏览器。当状态面板显示“状态：已启用”和“扩展程序：已安装”时，表示集成正常工作。

如果连接了多个浏览器，您可以选择 Claude 使用哪一个。如果在您做出选择之前就开始了浏览器操作，Claude 会提示您选择一个。如需之后切换浏览器，请运行 `/chrome` 并选择 **Select browser…**。即使有其他浏览器连接，Claude 也会继续使用您选择的浏览器。

对于 VS Code，请参阅 [VS Code 中的浏览器自动化](https://code.claude.com/docs/zh-CN/vs-code.md#automate-browser-tasks-with-chrome)。

### 在 Claude 询问时安装扩展程序

当 Claude 在交互式会话中需要使用您的浏览器，而 Claude Code 未检测到扩展程序时，Claude Code 会显示一个标题为“Claude wants to use your browser”的安装提示。Claude Code 每个会话最多询问一次。

该提示提供三个选项：

* **Install extension**：在浏览器中打开扩展程序安装页面并启动引导式设置。Claude Code 会等待安装完成、连接扩展程序，并在同一会话中启用浏览器工具。连接就绪后，选择“Continue with browser tools”，Claude 会在您的浏览器中继续执行任务。您可以选择“Continue without browser tools”退出设置，之后再通过 `/chrome` 完成设置。
* **Not now**：在不使用浏览器工具的情况下继续执行任务。Claude Code 可能会在之后的会话中再次询问。
* **Don't ask again**：在以后的会话中不再显示该提示。您仍可随时通过 `/chrome` 设置该集成。

有两种托管 MCP 策略会关闭该提示：

* 如果您的组织通过 [`deniedMcpServers` 托管设置](https://code.claude.com/docs/zh-CN/managed-mcp#policy-based-control-with-allowlists-and-denylists)阻止了 `claude-in-chrome` MCP 服务器，Claude Code 不会显示安装提示。
* 如果您的组织部署了 [`managed-mcp.json`](https://code.claude.com/docs/zh-CN/managed-mcp#exclusive-control-with-managed-mcp-json) 文件，但没有[在托管集之外允许 Claude in Chrome](https://code.claude.com/docs/zh-CN/managed-mcp.md#allow-claude-in-chrome-alongside-the-managed-set)，Claude Code 不会显示安装提示。

### 默认启用 Chrome

如需无需每次传递 `--chrome` 即可连接 Chrome，请在 CLI 会话中运行 `/chrome` 并选择**默认启用**。在 [VS Code 扩展](https://code.claude.com/docs/zh-CN/vs-code.md#automate-browser-tasks-with-chrome)中，在输入框中输入 `/chrome` 并打开**默认启用**开关。两者共享同一个设置，因此在任一处启用后，CLI 和 VS Code 都会启用该设置。

启用该设置并使用 Claude Code v2.1.287 或更高版本时，每个 VS Code 会话都会在启动时连接到您的浏览器，因此 Claude 无需您输入 `@browser` 即可使用浏览器。在更早的版本中，或者关闭该设置时，VS Code 会话会在您输入 `@browser` 时连接。

Chrome 未运行时，Claude Code 会正常启动。

> [!NOTE]
>
> 默认启用 Chrome 会增加上下文使用量，因为浏览器工具及其说明会始终被加载。如果您注意到上下文消耗增加，请关闭该设置，仅在需要时连接：在 CLI 中使用 `--chrome`，或在 VS Code 输入框中输入 [`@browser`](https://code.claude.com/docs/zh-CN/vs-code#automate-browser-tasks-with-chrome)。

### 回答启动对话框

如果启动 `claude` 时出现“Claude in Chrome extension detected”对话框，您的回答会更改[**默认启用**设置](#enable-chrome-by-default)：

* **Yes, use my browser**：启用该设置并连接本次会话
* **No, keep browser tools off** 或 Esc：关闭该设置，Claude Code 将不再提供设置 Chrome 的选项

如需为以后的会话更改您的回答，请运行 `/chrome`。

### 管理站点权限

站点级权限继承自 Chrome 扩展程序。在 Chrome 扩展程序设置中管理权限，以控制 Claude 可以在哪些站点上浏览、点击和输入。在[自动模式](https://code.claude.com/docs/zh-CN/permission-modes.md#eliminate-prompts-with-auto-mode)下，当自动模式分类器本身批准了对某个站点的浏览器调用时，扩展程序会跳过该调用的逐站点检查，除非您的权限规则拒绝 Claude in Chrome 访问任何站点。

### VS Code 会话中的权限提示

在 VS Code 会话中，当 Claude Code 在执行浏览器操作前进行询问时，提示会以卡片形式显示在聊天面板中。当操作针对您尚未允许的站点时，该卡片还会提供允许该站点的选项。

在因启用了[默认启用](#enable-chrome-by-default)而在启动时连接到浏览器的会话中，无论处于 Manual、Edit automatically、Auto 还是 Bypass permissions 模式，Claude Code 在对您尚未允许的站点执行浏览器操作前都会询问您。在 Auto 和 Bypass permissions 模式下，此行为会持续到您在该会话中输入 `@browser` 为止。

### 计划模式下的浏览器工具

在[计划模式](https://code.claude.com/docs/zh-CN/permission-modes.md#analyze-before-you-edit-with-plan-mode)下，Claude 在录制 GIF、打开新标签页或运行快捷方式之前会显示权限提示。在交互式 CLI 会话中，如果[绕过权限模式可用](https://code.claude.com/docs/zh-CN/permission-modes.md#skip-all-checks-with-bypasspermissions-mode)且[功能标志获取](https://code.claude.com/docs/zh-CN/env-vars.md#features-that-need-feature-flag-fetching)已关闭，这些调用将在不提示的情况下运行。

设置了 `createIfEmpty` 的 `tabs_context_mcp` 调用也会触发提示，包含上述任一操作的 `browser_batch` 调用同样如此。

## 示例工作流

以下示例展示了将浏览器操作与编码任务结合使用的常见方式。运行 `/mcp`，选择 `claude-in-chrome`，然后选择 **View tools**，即可查看可用浏览器工具的完整列表。

### 测试本地 Web 应用程序

开发 Web 应用时，可以让 Claude 验证您的更改是否正常工作：

```text wrap theme={null}
I just updated the login form validation. Can you open localhost:3000,
try submitting the form with invalid data, and check if the error
messages appear correctly?
```

Claude 会导航到您的本地服务器，与表单进行交互，并报告其观察到的情况。

### 使用控制台日志进行调试

Claude 可以读取控制台输出来帮助诊断问题。由于日志可能非常冗长，请告诉 Claude 需要查找哪些模式，而不是要求获取全部控制台输出：

```text wrap theme={null}
Open the dashboard page and check the console for any errors when
the page loads.
```

Claude 会读取控制台消息，并可以筛选特定模式或错误类型。

### 自动填写表单

加快重复性数据录入任务：

```text wrap theme={null}
I have a spreadsheet of customer contacts in contacts.csv. For each row,
go to the CRM at crm.example.com, click "Add Contact", and fill in the
name, email, and phone fields.
```

Claude 会读取您的本地文件，在 Web 界面中导航，并为每条记录输入数据。

### 向网页上传文件

Claude 可以将您计算机上的文件附加到页面上的上传字段中。Claude Code 会读取文件并将其内容发送到浏览器，因此上传在本地会话和远程会话中均可正常工作。

以下示例将一个日志文件附加到表单：

```text wrap theme={null}
Open the bug tracker at bugs.example.com, create a new issue,
and attach logs/session.log to it
```

如果 Claude 拒绝附加文件或上传失败，请检查以下原因：

* **权限**：只有当会话被允许读取某个文件时，Claude 才能上传该文件，因此拒绝对文件进行 `Read` 访问的[权限规则](https://code.claude.com/docs/zh-CN/settings-reference.md#permission-settings)也会阻止上传该文件。
* **大小**：单次上传的文件总大小最多为 10 MB。
* **硬链接**：Claude 会拒绝具有多个硬链接的文件，这种情况在 `node_modules` 等包管理器存储中很常见。请复制该文件并上传副本。
* **凭据名称**：如果文件的名称或所在文件夹属于通常用于存放凭据的位置，例如 `.env`、`.pem` 或 `.key` 文件，或 `.ssh` 下的任何内容，Claude 会拒绝该文件。需要 Claude Code v2.1.293 或更高版本。

### 在 Google Docs 中起草内容

使用 Claude 直接在您的文档中撰写内容，无需进行 API 设置：

```text wrap theme={null}
Draft a project update based on the recent commits and add it to my
Google Doc at docs.google.com/document/d/abc123
```

Claude 会打开文档，点击进入编辑器并输入内容。这适用于您已登录的任何 Web 应用：Gmail、Notion、Sheets 等。

### 从网页中提取数据

从网站中提取结构化信息：

```text wrap theme={null}
Go to the product listings page and extract the name, price, and
availability for each item. Save the results as a CSV file.
```

Claude 会导航到该页面，读取内容，并将数据整理为结构化格式。

### 运行多站点工作流

跨多个网站协调任务：

```text wrap theme={null}
Check my calendar for meetings tomorrow, then for each meeting with
an external attendee, look up their company website and add a note
about what they do.
```

Claude 会在多个标签页之间工作，以收集信息并完成工作流。

### 录制演示 GIF

创建可分享的浏览器交互录制：

```text wrap theme={null}
Record a GIF showing how to complete the checkout flow, from adding
an item to the cart through to the confirmation page.
```

Claude 会录制交互过程并将其保存为 GIF 文件。录制内容会捕获浏览器中可见的所有内容，包括已登录页面上的账户详细信息，因此在与团队以外的人员分享之前，请先进行检查。

### 将屏幕截图保存到磁盘

让 Claude 将屏幕截图保存为文件：

```text wrap theme={null}
Take a screenshot of the checkout page and save it to disk
```

Claude 会将图像保存到磁盘并报告文件路径。

## 故障排除

### 未检测到扩展程序

如果 Claude Code 无法检测到 Chrome 扩展程序：

1. 验证 Chrome 扩展程序已安装并在 `chrome://extensions` 中启用
2. 通过运行 `claude --version` 验证 Claude Code 是最新的
3. 检查 Chrome 是否正在运行
4. 运行 `/chrome` 并选择"重新连接扩展程序"以重新建立连接
5. 如果问题仍然存在，请重新启动 Claude Code 和 Chrome

第一次启用 Chrome 集成时，Claude Code 会安装本机消息传递主机配置文件。Chrome 在启动时读取此文件，因此如果扩展程序在您的第一次尝试中未被检测到，请重新启动 Chrome 以获取新配置。

Claude Code 在首次安装时会打开一个浏览器标签页，提示您连接扩展程序。稍后重写配置文件的会话（例如在切换构建或配置目录后）不会重新打开它。

如果连接仍然失败，请验证主机配置文件是否存在于：

对于 Chrome：

* **macOS**：`~/Library/Application Support/Google/Chrome/NativeMessagingHosts/com.anthropic.claude_code_browser_extension.json`
* **Linux**：`~/.config/google-chrome/NativeMessagingHosts/com.anthropic.claude_code_browser_extension.json`
* **Windows**：检查 Windows 注册表中的 `HKCU\Software\Google\Chrome\NativeMessagingHosts\`

对于 Edge：

* **macOS**：`~/Library/Application Support/Microsoft Edge/NativeMessagingHosts/com.anthropic.claude_code_browser_extension.json`
* **Linux**：`~/.config/microsoft-edge/NativeMessagingHosts/com.anthropic.claude_code_browser_extension.json`
* **Windows**：检查 Windows 注册表中的 `HKCU\Software\Microsoft\Edge\NativeMessagingHosts\`

其他基于 Chromium 的浏览器从其自己的配置目录读取相同的文件，该目录以浏览器命名。例如，macOS 上的 Brave 使用 `~/Library/Application Support/BraveSoftware/Brave-Browser/NativeMessagingHosts/`，在 Windows 上每个浏览器都有自己的注册表项，例如 `HKCU\Software\BraveSoftware\Brave-Browser\NativeMessagingHosts\`。

### 项目设置无法启用 Chrome

终端中出现以下警告，表示您正在使用的项目尝试启用 Chrome 集成，但 Claude Code 未允许：

```text wrap theme={null}
Claude Code ignored CLAUDE_CODE_ENABLE_CFC in this project's settings: a project can't turn on Claude in Chrome. To turn it on yourself, run /chrome or start with --chrome.
```

该项目的 `.claude/settings.json` 或 `.claude/settings.local.json` 在其 `env` 块中将 [`CLAUDE_CODE_ENABLE_CFC`](https://code.claude.com/docs/zh-CN/env-vars#variables) 设置为 `1`，以启用 Chrome 集成。Claude Code 未应用该设置，因此本会话中 Chrome 集成处于关闭状态，Claude 没有浏览器工具。

Claude Code 会跳过该设置，因为这些文件存储在项目目录中，而您检出的仓库不得能够将 Claude 连接到您的浏览器。

您可以照常继续工作。如果您需要浏览器工具，或希望消除该警告，请执行以下操作之一：

* **立即获取浏览器工具**：退出，然后在 shell 中使用 `claude --chrome` 重新启动。
* **在后续会话中获取浏览器工具**：在 Claude Code 输入框中运行 `/chrome` 并选择[**默认启用**](#enable-chrome-by-default)。这适用于之后启动的会话，而不是当前正在运行的会话。
* **在不使用浏览器工具的情况下停止警告**：从项目的设置文件中删除 `CLAUDE_CODE_ENABLE_CFC` 这一行。

### 浏览器无响应

如果 Claude 的浏览器命令停止工作：

1. 检查是否有模态对话框（alert、confirm、prompt）阻止页面。JavaScript 对话框阻止浏览器事件并防止 Claude 接收命令。手动关闭对话框，然后告诉 Claude 继续。
2. 要求 Claude 创建新标签页并重试
3. 通过在 `chrome://extensions` 中禁用并重新启用来重新启动 Chrome 扩展程序

### 长会话期间连接断开

Chrome 扩展程序的 service worker 在扩展会话期间可能会进入空闲状态，这会破坏连接。如果浏览器工具在一段时间不活动后停止工作，请运行 `/chrome` 并选择"重新连接扩展程序"。

运行 `/chrome` 时，请查看其 `Status` 行。如果显示"未连接"，则表示正在运行的会话自身与 Chrome 的连接已失败。选择"重新连接扩展程序"以重新启动该连接。连接成功后，扩展程序的重新连接页面会在 Chrome 中打开。在 v2.1.290 之前，"重新连接扩展程序"仅会打开该页面，而不会重新启动失败的连接，因此如果在较早版本上浏览器工具未恢复，请更新 Claude Code。

### 扩展程序登录到了不同的组织

如果您属于多个 claude.ai 组织，扩展程序必须登录到与 Claude Code 相同的组织。如果两者不同，即使两者使用相同的 claude.ai 账户，Claude 的浏览器工具也会返回"Browser extension is not connected"。

要查看 Claude Code 登录的是哪个组织，请在 Claude Code 输入框中运行 [`/status`](https://code.claude.com/docs/zh-CN/commands) 并查看 `Organization` 行。

> [!WARNING]
>
> 如果您从扩展程序中注销，将丢失其中保存的快捷方式和定时任务。请先尝试[常见错误消息](#common-error-messages)下的其他修复方法。

要更改扩展程序的组织，请在扩展程序的设置中注销，然后重新登录并选择 `/status` 显示的组织。

### Windows 特定问题

在 Windows 上，您可能会遇到：

* **命名管道冲突 (EADDRINUSE)**：如果另一个进程正在使用相同的命名管道，请重新启动 Claude Code。关闭任何可能使用 Chrome 的其他 Claude Code 会话。
* **本机消息传递主机错误**：如果本机消息传递主机在启动时崩溃，请尝试重新安装 Claude Code 以重新生成主机配置。
* **设置页面无法打开**：更新 Claude Code。在 v2.1.211 之前，提示您连接扩展程序的浏览器标签页在 Windows 上可能无法打开。

### 常见错误消息

这些是最常见的错误及其解决方法：

| 错误 | 原因 | 修复 |
| - | - | - |
| "浏览器扩展程序未连接" | 扩展程序未在 Chrome 中安装并运行，扩展程序登录的 claude.ai 账户或组织与 Claude Code 不同，或您的组织的 IP 允许列表拒绝了到 `bridge.claudeusercontent.com` 的连接 | 检查扩展程序登录的 claude.ai 账户和[组织](#extension-signed-in-to-a-different-organization)是否与 Claude Code 相同，重新启动 Chrome 和 Claude Code，然后运行 `/chrome` 以重新连接。如果您的组织使用 IP 允许列表且错误仍然存在，请参阅[组织 IP 允许列表和代理出口](https://code.claude.com/docs/zh-CN/network-config.md#organization-ip-allowlists-and-proxy-egress) |
| 扩展程序在 `/chrome` 中显示"未检测到" | Chrome 扩展程序未安装或已禁用 | 在 `chrome://extensions` 中安装或启用扩展程序 |
| "没有可用的标签页" | Claude 在标签页准备好之前尝试操作 | 要求 Claude 创建新标签页并重试 |
| "接收端不存在" | 扩展程序 service worker 进入空闲状态 | 运行 `/chrome` 并选择"重新连接扩展程序" |
| "没有已连接的浏览器" | Chrome 未打开，或扩展程序登录的 claude.ai 账户和组织与 Claude Code 不同，或 Claude Code 无法访问 `bridge.claudeusercontent.com` | 确认 Chrome 已打开，且扩展程序登录的 claude.ai 账户和[组织](#extension-signed-in-to-a-different-organization)与 Claude Code 相同。如果是这样，请检查您的网络是否允许 Claude Code 访问 `bridge.claudeusercontent.com`，该地址列于[网络访问要求](https://code.claude.com/docs/zh-CN/network-config.md#network-access-requirements)中 |

## 另请参阅

* [计算机使用](https://code.claude.com/docs/zh-CN/computer-use.md)：当任务无法在浏览器中完成时控制本机 macOS 应用
* [在 VS Code 中使用 Claude Code](https://code.claude.com/docs/zh-CN/vs-code.md#automate-browser-tasks-with-chrome)：VS Code 扩展程序中的浏览器自动化
* [CLI 参考](https://code.claude.com/docs/zh-CN/cli-reference.md)：命令行标志，包括 `--chrome`
* [常见工作流](https://code.claude.com/docs/zh-CN/common-workflows.md)：更多使用 Claude Code 的方式
* [数据和隐私](https://code.claude.com/docs/zh-CN/data-usage.md)：Claude Code 如何处理您的数据
* [Claude in Chrome 入门](https://support.claude.com/en/articles/12012173-getting-started-with-claude-in-chrome)：Chrome 扩展程序的完整文档，包括快捷键、计划和权限
