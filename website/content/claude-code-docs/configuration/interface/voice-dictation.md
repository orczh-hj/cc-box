> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# 语音听写

> 在 Claude Code CLI 中使用按住录音或点击录音的语音听写功能来说出你的提示词。

在 Claude Code CLI 中说出你的提示词，而不是输入它们。你的语音会实时转录到提示词输入中，所以你可以在同一条消息中混合使用语音和输入。使用 `/voice` 启用听写，然后要么在说话时按住一个键，要么点击一次开始，再点击一次发送。

在[按住模式](#hold-to-record)下，听写功能也适用于 [Agent 视图](https://code.claude.com/docs/zh-CN/agent-view.md#peek-and-reply)。在 Dispatch 输入框或窥视面板回复获得焦点时，按住您的按键通话键，即可向后台会话进行听写。

## 要求

语音听写将你录制的音频流传输到 Anthropic 的服务器进行转录。音频不在本地处理。它需要以下所有条件：

* **一个 Claude.ai 账户**：语音转文本服务仅在你使用 Claude.ai 账户进行身份验证时可用，当 Claude Code 配置为直接使用 Anthropic API 密钥、Amazon Bedrock、Google Cloud 的 Agent Platform 或 Microsoft Foundry 时不可用。
* **一个本地麦克风**：语音听写在[云会话](https://code.claude.com/docs/zh-CN/claude-code-on-the-web.md)或 SSH 会话中不起作用。
* **如果你在 WSL 中运行 Claude Code，则需要 WSLg**：WSLg 包含在 Windows 10 或 11 上从 Microsoft Store 安装的 WSL2 中。如果 WSLg 不可用，例如在 WSL1 上，改为在本机 Windows 中运行 Claude Code。

转录不消耗 Claude 消息或令牌，也不计入 `/usage` 中显示的限制。有关 Anthropic 如何处理你的数据，请参阅[数据使用](https://code.claude.com/docs/zh-CN/data-usage.md)。

音频录制在 macOS、Linux 和 Windows 上使用内置的本机模块。在 Linux 上，如果本机模块无法加载，Claude Code 会回退到 ALSA utils 中的 `arecord` 或 SoX 中的 `rec`。如果两者都不可用，`/voice` 会打印你的包管理器的安装命令。

Claude Code [VS Code 扩展](https://code.claude.com/docs/zh-CN/vs-code.md)也支持语音听写，具有相同的 Claude.ai 账户要求。它在 VS Code Remote 会话中不可用，包括 SSH、Dev Containers 和 Codespaces，因为麦克风在你的本地机器上，而扩展在远程主机上运行。

## 启用语音听写

运行 `/voice` 启用听写。启用时，Claude Code 会运行麦克风检查。在 macOS 上，如果之前从未授予过权限，这会触发终端的系统麦克风权限提示。

```
/voice
Voice mode enabled (hold). Hold space to record. Dictation language: en (/config to change).
```

`/voice` 接受一个可选的模式参数：

| 命令 | 效果 |
| :- | :- |
| `/voice` | 切换开或关，保持当前模式 |
| `/voice hold` | 在[按住模式](#hold-to-record)中启用 |
| `/voice tap` | 在[点击模式](#tap-to-record-and-send)中启用 |
| `/voice off` | 禁用 |

语音听写在会话之间持续。直接在你的[用户设置文件](https://code.claude.com/docs/zh-CN/settings.md)中设置它，而不是运行 `/voice`：

```json theme={null}
{
  "voice": {
    "enabled": true,
    "mode": "tap"
  }
}
```

启用语音听写的前三个会话中，当提示词为空时，输入页脚会显示 `hold space to speak` 提示。提示文本反映你当前的 `voice:pushToTalk` 快捷键绑定，如果你[重新绑定听写键](#rebind-the-dictation-key)，它会更新。提示文本在两种模式中都相同，如果你配置了[自定义状态行](https://code.claude.com/docs/zh-CN/statusline.md)，则不会显示。

转录在两种模式中都针对编码词汇进行了调整。常见的开发术语如 `regex`、`OAuth`、`JSON` 和 `localhost` 被正确识别，你当前的项目名称和 git 分支名称会自动添加为识别提示。

## 按住录音

按住模式是按键通话：当你按住键时录制运行，释放时停止。这是默认模式。

按住 `Space` 开始录制。Claude Code 通过监视来自你的终端的快速按键重复事件来检测按住的键，因此在录制开始之前有一个简短的预热。页脚在预热期间显示 `keep holding…`，然后在录制激活后显示 `listening…`。录制时，提示词光标变成一个随麦克风电平上下移动的条形，除非你打开了 [`prefersReducedMotion`](https://code.claude.com/docs/zh-CN/settings-reference#prefersreducedmotion)。

前几个按键重复字符在预热期间输入到输入中，当录制激活时会自动删除。单个 `Space` 点击仍然会输入一个空格，因为按住检测仅在快速重复时触发。

按住或点击 `Space` 仅在按键会输入到提示词的地方开始听写。在[转录查看器](https://code.claude.com/docs/zh-CN/interactive-mode.md#transcript-viewer)中，`Space` 翻页浏览对话，在[vim 模式](https://code.claude.com/docs/zh-CN/interactive-mode.md#vim-editor-mode)的非 INSERT 状态下它是一个命令。[重新绑定的修饰符组合](#rebind-the-dictation-key)（如 `meta+k`）永远不会输入文本，所以它也可以从这些地方开始听写。

> [!TIP]
>
> 要跳过预热，使用 `/voice tap` 切换到[点击模式](#tap-to-record-and-send)，或[重新绑定到修饰符组合](#rebind-the-dictation-key)，如 `meta+k`。修饰符组合在第一次按键时开始录制。

你的语音在你说话时出现在提示词中，在转录最终确定之前会变暗。释放 `Space` 停止录制并最终确定文本。转录被插入到你的光标位置，光标保持在插入文本的末尾，所以你可以以任何顺序混合输入和听写。再次按住 `Space` 追加另一个录制，或先移动光标以在提示词中的其他地方插入语音：

```
> refactor the auth middleware to ▮
  # hold space, speak "use the new token validation helper"
> refactor the auth middleware to use the new token validation helper▮
```

默认情况下，释放键会插入转录并等待你按 `Enter`。在 `voice` 设置对象中设置 `"autoSubmit": true` 以在释放键时自动发送提示词，只要转录至少有三个单词。

## 点击录音并发送

点击模式使用单个按键切换录制：点击一次开始，说话，然后再点击一次发送提示词。没有预热，你不需要保持键被按住。

使用 `/voice tap` 启用点击模式。当提示词输入为空时，点击 `Space` 开始录制。页脚在录制时显示 `● REC · tap to send`。再次点击 `Space` 停止。

Claude Code 插入转录并在转录至少有三个单词时自动提交提示词。较短的转录被插入但不提交，所以意外点击不会发送一个杂散的单词。

三个单词的阈值计算不使用空格书写的语言中的单词。日语、中文和泰语转录计算单个单词，所以它们在点击模式和带有 `autoSubmit` 的保持模式下自动提交。

第一次点击仅在提示词输入为空时开始录制，所以你仍然可以在撰写消息时正常输入空格。第二次点击停止录制，无论输入内容如何。录制也会在 15 秒无声或总共两分钟后自动停止。

## 取消录音

按 `Esc` 或 `Ctrl+C` 可以取消听写，而不是完成它。Claude Code 会停止麦克风，丢弃转录文本，并将提示恢复到录音开始前的状态。

这两个键也可以在完成的录音的转录文本仍在处理时取消。在处理期间编辑或提交的提示会保持您留下的状态。

这两个键在取消的按键中不执行任何其他操作：`Esc` 不会中断 Claude 的响应，`Ctrl+C` 不会清除提示或计为 [退出 Claude Code 的两次按键中的第一次](https://code.claude.com/docs/zh-CN/interactive-mode.md#general-controls)。

## 更改听写语言

语音听写使用与控制 Claude 响应语言相同的[`language` 设置](https://code.claude.com/docs/zh-CN/settings-reference#language)。如果该设置为空，听写默认为英语。在 VS Code 扩展中，如果 `language` 为空，听写在默认为英语之前使用 VS Code 的 `accessibility.voice.speechLanguage` 设置。

> [accordion] 支持的听写语言
>
> | 语言 | 代码 |
> | :- | :- |
> | 捷克语 | `cs` |
> | 丹麦语 | `da` |
> | 荷兰语 | `nl` |
> | 英语 | `en` |
> | 法语 | `fr` |
> | 德语 | `de` |
> | 希腊语 | `el` |
> | 印地语 | `hi` |
> | 印度尼西亚语 | `id` |
> | 意大利语 | `it` |
> | 日语 | `ja` |
> | 韩语 | `ko` |
> | 挪威语 | `no` |
> | 波兰语 | `pl` |
> | 葡萄牙语 | `pt` |
> | 俄语 | `ru` |
> | 西班牙语 | `es` |
> | 瑞典语 | `sv` |
> | 土耳其语 | `tr` |
> | 乌克兰语 | `uk` |

在 `/config` 中或直接在设置中设置语言。你可以使用 [BCP 47 语言代码](https://en.wikipedia.org/wiki/IETF_language_tag)或语言名称：

```json theme={null}
{
  "language": "japanese"
}
```

如果你的 `language` 设置不在支持的列表中，`/voice` 在启用时会警告你，并为听写回退到英语。Claude 的文本响应不受此回退的影响。

## 重新绑定听写键

听写键在 `Chat` 上下文中绑定到 `voice:pushToTalk`，默认为 `Space`。相同的绑定控制按住和点击模式。在 [`~/.claude/keybindings.json`](https://code.claude.com/docs/zh-CN/keybindings) 中重新绑定它：

```json theme={null}
{
  "bindings": [
    {
      "context": "Chat",
      "bindings": {
        "meta+k": "voice:pushToTalk",
        "space": null
      }
    }
  ]
}
```

`voice:pushToTalk` 操作一次使用一个键。当你绑定自定义键时，它会替换默认的 `Space` 绑定，而不是添加第二个触发器，所以此示例中的 `"space": null` 行是为了清晰起见，可以省略而不改变行为。

在按住模式中，避免绑定裸字母键如 `v`，因为按住检测依赖于按键重复，字母在预热期间输入到提示词中。使用 `Space`，或使用修饰符组合如 `meta+k` 在第一次按键时开始录制，无需预热。点击模式没有预热，所以大多数键都可以。

某些键不会传递到终端应用程序，根本无法绑定。例如，如果你尝试绑定 `Caps Lock`，会显示错误。有关完整的快捷键语法和保留快捷键列表，请参阅[自定义键盘快捷键](https://code.claude.com/docs/zh-CN/keybindings.md)。

## 故障排除

语音听写不激活或不录制时的常见问题：

* **`Unknown command: /voice`**：只有在以 claude.ai 账户作为当前登录方式时才能使用 `/voice`。如果您未使用 claude.ai 账户登录，请运行 `/login`。如果正在使用 `ANTHROPIC_API_KEY`、`ANTHROPIC_AUTH_TOKEN`、`apiKeyHelper` 设置或[第三方提供商](#requirements)，它们会优先于 claude.ai 登录，因此请将其移除并重新启动 Claude Code。
* **`Voice mode requires a Claude.ai account`**：在您运行 `/voice` 或开始录制时，Claude Code 找不到可用的 claude.ai 登录。运行 `/login` 重新登录。
* **`Voice mode is disabled by your organization's policy`**：你的组织的管理员策略关闭了语音听写。联系你的组织管理员以确认你的组织是否可以使用语音听写。
* **`Microphone access is denied`**：在系统设置中授予你的终端麦克风权限。在 macOS 上，转到系统设置 → 隐私和安全 → 麦克风并启用你的终端应用，然后再次运行 `/voice`。在 Windows 上，转到设置 → 隐私和安全 → 麦克风并为桌面应用打开麦克风访问，然后再次运行 `/voice`。如果你的终端未在 macOS 设置中列出，请参阅[终端未在 macOS 麦克风设置中列出](#terminal-not-listed-in-macos-microphone-settings)。
* **`Voice mode requires SoX for audio recording` on Linux**：本机音频模块无法加载，没有安装回退。使用错误消息中显示的命令安装 SoX，例如 `sudo apt-get install sox`。
* **`Voice mode requires a microphone, but SoX could not open an audio capture device`**：SoX 已安装，但主机没有音频捕获设备，例如无头服务器或容器。在有麦克风的机器上运行 Claude Code。从 v2.1.195 开始，Linux 上的 Claude Code 在这种情况下报告此消息；早期版本即使已安装 SoX 也会要求你安装 SoX。
* **`Voice mode could not find a working audio recorder in WSL`**：WSLg 通过 PulseAudio 而不是 ALSA 设备路由音频，因此 SoX 需要显式安装其 PulseAudio 后端。运行 `sudo apt install sox libsox-fmt-pulse`。单独安装 `sox` 会拉入 ALSA 后端，它无法在 WSL 上录制，因为没有 `/dev/snd` 设备。
* **`Voice input is failing repeatedly and has been paused`**：语音听写在 10 秒内遇到三次失败。Claude Code 暂停听写，直到自第一次失败以来已经过了 10 秒。这通常意味着此主机上的麦克风或音频堆栈无法捕获音频，例如无头服务器、没有音频直通的远程 shell 或被拒绝的麦克风权限。确认工作输入设备，从上面的条目中修复根本原因，然后再次触发语音。在 v2.1.202 之前，只有启动失败计入暂停。
* **在按住模式中按住 `Space` 时没有任何反应**：在按住时观察提示词输入。如果空格不断累积，语音听写可能已关闭；运行 `/voice hold` 启用它。如果只出现一两个空格然后没有任何反应，语音听写已打开但按住检测未触发。按住检测需要你的终端发送按键重复事件，所以如果在操作系统级别禁用了按键重复，它无法检测按住的键。使用 `/voice tap` 切换到点击模式以避免按键重复要求。
* **在点击模式中点击 `Space` 输入空格而不是录制**：第一次点击仅在提示词输入为空时开始录制。先清除输入，或通过运行 `/voice tap` 检查你是否处于点击模式。
* **`No audio detected from microphone`**：录制开始但捕获了静音。确认正确的输入设备设置为系统默认值，其输入级别未静音或接近零。在 Windows 上，打开设置 → 系统 → 声音 → 输入并选择你的麦克风。在 macOS 上，打开系统设置 → 声音 → 输入。
* **`Voice connection failed`**：你的录制从未到达转录服务，因为连接失败。检查你的网络并重试。捕获无音频的录制报告 `No audio detected from microphone` 而不是此消息。在 v2.1.200 之前，静音麦克风可能报告连接失败，这表示网络问题，而实际问题是输入设备。
* **`Dictation error: WebSocket error:` 后跟证书错误，例如 `unable to get local issuer certificate`**：请参阅 VS Code 页面上的[在执行 TLS 检查的代理后听写或 Web 选项卡失败](https://code.claude.com/docs/zh-CN/vs-code.md#dictation-or-the-web-tab-fails-behind-a-tls-inspecting-proxy)。
* **`Voice stream error: WebSocket upgrade rejected with HTTP <status>`**：服务器以显示的 HTTP 状态拒绝了你的连接，所以这不是网络中断。400 范围内的状态通常意味着过期的登录、代理或机器人保护服务代替转录服务进行应答。运行 `/login` 刷新你的登录，如果状态仍然存在，请检查你的网络路径上是否有 VPN 或代理。如果拒绝到达时你仍在录制，Claude Code 会在显示此消息之前重试一次 400 范围外的状态；它不会重试 400 范围内的状态。在 v2.1.229 到 v2.1.231 中，本机构建没有显示此消息：Claude Code 继续录制，按住模式页脚仍然显示 `listening…`，在你停止录制后报告 `Voice connection failed`。
* **`No speech detected`**：音频到达转录服务但未识别任何单词。靠近麦克风说话，减少背景噪音，并确认你的[听写语言](#change-the-dictation-language)与你说话的语言匹配。
* **`Dictation stopped: no speech recognized for 15 seconds` 或 `Dictation stopped at its 2-minute limit`**：在 VS Code 扩展中，如果 15 秒内没有识别到任何单词，或在开始两分钟后，录制就会结束。如果您在这 15 秒内一直在说话，请尝试 `No speech detected` 的修复方法。已识别的单词会保留在输入框中，不会被发送。要添加更多内容，请再次单击麦克风按钮。
* **转录是乱码或使用了错误的语言**：听写默认为英语。如果你用另一种语言听写，请先在 `/config` 中设置它。请参阅[更改听写语言](#change-the-dictation-language)。

### 终端未在 macOS 麦克风设置中列出

如果你的终端应用未出现在系统设置 → 隐私和安全 → 麦克风下，则没有你可以启用的切换。重置你的终端的权限状态，以便下一次 `/voice` 运行触发新的 macOS 权限提示。

**1. 重置你的终端的麦克风权限**

运行 `tccutil reset Microphone <bundle-id>`，将 `<bundle-id>` 替换为你的终端的标识符：内置终端为 `com.apple.Terminal`，或 iTerm2 为 `com.googlecode.iterm2`。对于其他终端，使用 `osascript -e 'id of app "AppName"'` 查找标识符。

> [!WARNING]
>
> 你可以运行 `tccutil reset Microphone` 而不带 bundle ID，但这会撤销 Mac 上每个应用的麦克风访问权限，包括 Zoom 或 Slack 等应用。每个应用在下次使用时都需要重新请求访问权限，所以不要在活跃通话期间运行它。

**2. 退出并重新启动你的终端**

macOS 不会重新提示已在运行的进程。使用 Cmd+Q 退出终端应用，而不仅仅是关闭其窗口，然后再次打开它。

**3. 触发新的提示**

启动 Claude Code 并运行 `/voice`。macOS 提示输入麦克风访问权限；允许它。

## 另请参阅

* [自定义键盘快捷键](https://code.claude.com/docs/zh-CN/keybindings.md)：重新绑定 `voice:pushToTalk` 和其他 CLI 键盘操作
* [所有设置](https://code.claude.com/docs/zh-CN/settings-reference.md#voice)：`voice`、`language` 和其他设置键
* [交互模式](https://code.claude.com/docs/zh-CN/interactive-mode.md)：键盘快捷键、输入模式和会话控制
* [命令](https://code.claude.com/docs/zh-CN/commands.md)：`/voice`、`/config` 和所有其他命令的参考
