> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# 示例设置文件

> 为开发者、团队和组织提供的现实 settings.json 文件：复制一个，保留你想要的键，并更改值。

本页包含三个示例 `settings.json` 文件，每个文件对应一个保存设置的位置：

* 开发者的 `~/.claude/settings.json`
* 团队的 `.claude/settings.json`，提交到仓库
* 组织的 `managed-settings.json`

每个文件都是该读者的合理文件，因此你可以看到其结构并复制你想要的部分。它们都不是推荐的基线。每个值都来自 [settings reference](https://code.claude.com/docs/zh-CN/settings-reference.md) 上的键条目，该条目包含其类型、默认值和可以设置的位置。

每个示例有两个选项卡。**Copyable settings file** 是你保存的文件。**What each key does** 是同一个文件，每个键上方都有注释；Claude Code 不接受设置文件中的注释，因此请从第一个选项卡复制。

## 你自己的设置

一个开发者的个人设置。它选择一个模型和工作量级别，调整终端，并预先批准一个只读命令和一个文件读取。未列出的所有内容都保持其默认值。这样的文件放在 `~/.claude/settings.json` 中，它适用于你打开的每个项目。

> [tab] Copyable settings file
>
> 将其保存为 `~/.claude/settings.json`。这是有效的 JSON，没有注释，因此你可以按原样粘贴它并删除你不想要的键。
>
> ```json ~/.claude/settings.json theme={null}
> {
>   "model": "claude-sonnet-5",
>   "modelSettings": {
>     "claude-sonnet-5": { "effortLevel": "xhigh" }
>   },
>   "editorMode": "vim",
>   "theme": "light-daltonized",
>   "statusLine": {
>     "type": "command",
>     "command": "jq -r '\"[\\(.model.display_name)] \\(.context_window.used_percentage // 0)% context\"'",
>     "padding": 2
>   },
>   "spinnerTipsEnabled": false,
>   "preferredNotifChannel": "terminal_bell",
>   "permissions": {
>     "allow": [
>       "Bash(git diff *)",
>       "Read(~/.zshrc)"
>     ]
>   },
>   "autoUpdatesChannel": "stable",
>   "cleanupPeriodDays": 20
> }
> ```

> [tab] What each key does
>
> 同一个文件，每个键上方都有注释。在这里阅读；从另一个选项卡复制，因为 Claude Code 不接受设置文件中的注释。
>
> ```jsonc ~/.claude/settings.json theme={null}
> {
>   // 在 Sonnet 5 上启动每个会话
>   "model": "claude-sonnet-5",
>   // 在 Sonnet 5 上运行高于其默认高级别；/effort 为每个模型保存一个级别，--effort 为单个会话设置一个级别
>   "modelSettings": {
>     "claude-sonnet-5": { "effortLevel": "xhigh" }
>   },
>   // 提示中的 Vim 快捷键
>   "editorMode": "vim",
>   // 色盲友好的浅色主题
>   "theme": "light-daltonized",
>   // 提示下方的状态行：模型名称和使用的上下文
>   "statusLine": {
>     "type": "command",
>     "command": "jq -r '\"[\\(.model.display_name)] \\(.context_window.used_percentage // 0)% context\"'",
>     "padding": 2
>   },
>   // 隐藏在加载器下旋转的提示
>   "spinnerTipsEnabled": false,
>   // 为通知（例如完成的任务或等待的权限提示）响铃终端铃声
>   "preferredNotifChannel": "terminal_bell",
>   // 让 Claude Code 运行 git diff 并读取你的 .zshrc，无需询问
>   "permissions": {
>     "allow": [
>       "Bash(git diff *)",
>       "Read(~/.zshrc)"
>     ]
>   },
>   // 从稳定频道获取更新
>   "autoUpdatesChannel": "stable",
>   // 删除超过 20 天的会话记录和其他本地会话数据
>   "cleanupPeriodDays": 20
> }
> ```

## 团队的共享设置

一个团队的共享设置，提交到仓库，以便克隆它的每个人都获得相同的权限、hooks 和插件市场。在仓库顶部的 `.claude/settings.json` 处保存这样的文件。在提交之前需要了解的内容：

* **[项目](https://code.claude.com/docs/zh-CN/claude-projects.md#what-threads-pick-up-from-your-repositories)线程之外的云端会话也会读取它。** 包含单个仓库的[云端会话](https://code.claude.com/docs/zh-CN/settings.md#settings-in-cloud-sessions)在克隆内部启动，因此提交的文件也适用于那里。
* **遥测数据放在托管或个人设置中。** Claude Code 忽略仓库设置文件中的 [OpenTelemetry 导出器变量](https://code.claude.com/docs/zh-CN/settings-reference.md#variables-claude-code-ignores-in-env)，除了一些关闭遥测的值。在 [托管设置](https://code.claude.com/docs/zh-CN/monitoring-usage.md#administrator-configuration) 中为你的组织设置它们，或在每个人的 `~/.claude/settings.json` 中设置。
* **Allow 规则等待信任。** Allow 规则和 `extraKnownMarketplaces` 条目在每个人 [信任此文件夹本身](https://code.claude.com/docs/zh-CN/permissions.md#project-allow-rules-and-workspace-trust) 后生效，而不仅仅是父文件夹；deny 和 ask 规则在每个会话中应用，无论是否受信任。
* **hook 是仓库中的脚本。** 此文件的 hook 运行 `.claude/hooks/block-rm.sh`；[How a hook resolves](https://code.claude.com/docs/zh-CN/hooks.md#how-a-hook-resolves) 介绍了如何编写它。
* **规则匹配按写入的命令和路径。** `Bash(git push *)` 不匹配 [`git -C . push`](https://code.claude.com/docs/zh-CN/permissions#bash-rule-limits)。`Read(./.env)` 单独停止文件工具和命名文件的命令，例如 `cat .env`，但不停止 [`grep -r` 在目录上运行](https://code.claude.com/docs/zh-CN/permissions#read-and-edit)；此文件中的 `sandbox` 块关闭了该间隙，因为 sandbox [添加你的 `Read` deny 路径](https://code.claude.com/docs/zh-CN/settings-reference#sandbox-filesystem-denyread) 到每个沙箱命令无法读取的内容。

> [tab] Copyable settings file
>
> 将其保存为仓库顶部的 `.claude/settings.json` 并提交。这是有效的 JSON，没有注释，因此你可以按原样粘贴它并删除你不想要的键。
>
> ```json .claude/settings.json theme={null}
> {
>   "permissions": {
>     "allow": [
>       "Bash(npm run *)"
>     ],
>     "ask": [
>       "Bash(git push *)"
>     ],
>     "deny": [
>       "Read(./.env)",
>       "Read(./.env.*)",
>       "Read(./secrets/**)"
>     ]
>   },
>   "hooks": {
>     "PreToolUse": [
>       {
>         "matcher": "Bash",
>         "hooks": [
>           {
>             "type": "command",
>             "command": "${CLAUDE_PROJECT_DIR}/.claude/hooks/block-rm.sh"
>           }
>         ]
>       }
>     ]
>   },
>   "extraKnownMarketplaces": {
>     "acme-tools": {
>       "source": {
>         "source": "github",
>         "repo": "acme-corp/claude-plugins"
>       }
>     }
>   },
>   "enabledPlugins": {
>     "code-formatter@acme-tools": true
>   },
>   "sandbox": {
>     "enabled": true,
>     "filesystem": {
>       "allowWrite": [
>         "/tmp/build"
>       ]
>     },
>     "network": {
>       "allowedDomains": [
>         "registry.npmjs.org",
>         "*.example.com"
>       ]
>     }
>   },
>   "plansDirectory": "./plans"
> }
> ```

> [tab] What each key does
>
> 同一个文件，每个键上方都有注释。在这里阅读；从另一个选项卡复制，因为 Claude Code 不接受设置文件中的注释。
>
> ```jsonc .claude/settings.json theme={null}
> {
>   "permissions": {
>     // 无需询问即可运行 npm 脚本
>     "allow": [
>       "Bash(npm run *)"
>     ],
>     // 在 git push 命令前确认
>     "ask": [
>       "Bash(git push *)"
>     ],
>     // 拒绝文件工具和文件读取命令读取 env 文件和 secrets 文件夹
>     "deny": [
>       "Read(./.env)",
>       "Read(./.env.*)",
>       "Read(./secrets/**)"
>     ]
>   },
>   // 在每个 Bash 命令之前，运行仓库中可以阻止它的脚本
>   "hooks": {
>     "PreToolUse": [
>       {
>         "matcher": "Bash",
>         "hooks": [
>           {
>             "type": "command",
>             "command": "${CLAUDE_PROJECT_DIR}/.claude/hooks/block-rm.sh"
>           }
>         ]
>       }
>     ]
>   },
>   // 在每个克隆上注册团队的插件市场
>   "extraKnownMarketplaces": {
>     "acme-tools": {
>       "source": {
>         "source": "github",
>         "repo": "acme-corp/claude-plugins"
>       }
>     }
>   },
>   // 启用该市场中的一个插件；来自外部源（例如 GitHub 仓库）的插件仍然需要每个人安装一次
>   "enabledPlugins": {
>     "code-formatter@acme-tools": true
>   },
>   // 沙箱命令：可写的构建目录；npm 和 example.com 预先允许
>   "sandbox": {
>     "enabled": true,
>     "filesystem": {
>       "allowWrite": [
>         "/tmp/build"
>       ]
>     },
>     "network": {
>       "allowedDomains": [
>         "registry.npmjs.org",
>         "*.example.com"
>       ]
>     }
>   },
>   // 将计划文件保存在仓库内
>   "plansDirectory": "./plans"
> }
> ```

## 组织的托管设置

一个 `managed-settings.json` 文件，显示托管键的形状，每个键都有一个合理的值。这不是推荐的策略：选择与你自己的要求相匹配的键并设置你自己的值。该示例设置这些键：

* `forceLoginMethod` 和 `forceLoginOrgUUID` 固定登录方法和组织
* `availableModels` 和 `enforceAvailableModels` 限制会话可以使用的模型
* `permissions.deny` 拒绝两个文件读取和 `curl` 命令 [如 Claude 编写的那样](https://code.claude.com/docs/zh-CN/permissions.md#bash-rule-limits)，`disableBypassPermissionsMode` 删除绕过权限模式
* [`allowManagedPermissionRulesOnly`](https://code.claude.com/docs/zh-CN/settings-reference#allowmanagedpermissionrulesonly) 和 [`allowManagedMcpServersOnly`](https://code.claude.com/docs/zh-CN/settings-reference#allowmanagedmcpserversonly) 使托管权限和 MCP 允许列表成为唯一适用的列表
* `allowedMcpServers` 通过 URL 固定 MCP 服务器
* `strictKnownMarketplaces` 允许一个插件市场
* `sandbox` 使用固定的网络允许列表对命令进行沙箱处理，无需无沙箱重试。其 `failIfUnavailable` 键会[在沙箱无法运行的环境中阻止 Claude Code 启动](https://code.claude.com/docs/zh-CN/sandboxing.md#enforce-sandboxing-with-managed-settings)
* `requiredMinimumVersion` 设置最低 Claude Code 版本
* `cleanupPeriodDays` 将会话记录和其他本地数据的保留期缩短为七天
* `companyAnnouncements` 在启动时显示消息

管理员将这样的文件部署为 `managed-settings.json`，或通过 MDM 或 [server-managed settings](https://code.claude.com/docs/zh-CN/server-managed-settings.md) 部署相同的 JSON。一个部署的文件适用于它到达的每台机器或帐户。要为一个组提供不同的值，请将不同的文件或配置文件部署到该组，因为 [server-managed settings 还不支持按组策略](https://code.claude.com/docs/zh-CN/server-managed-settings.md#current-limitations)。

> [tab] Copyable settings file
>
> 将其部署为 `managed-settings.json`，或通过 MDM 或 claude.ai 控制台部署相同的 JSON。这是有效的 JSON，没有注释；将示例组织 UUID、服务器 URL 和市场替换为你自己的，并删除你不想要的键。
>
> ```json managed-settings.json theme={null}
> {
>   "forceLoginMethod": "claudeai",
>   "forceLoginOrgUUID": [
>     "xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx"
>   ],
>   "availableModels": [
>     "opus",
>     "sonnet"
>   ],
>   "enforceAvailableModels": true,
>   "permissions": {
>     "deny": [
>       "Bash(curl *)",
>       "Read(./.env)",
>       "Read(./secrets/**)"
>     ],
>     "disableBypassPermissionsMode": "disable"
>   },
>   "allowManagedPermissionRulesOnly": true,
>   "allowedMcpServers": [
>     {
>       "serverUrl": "https://api.githubcopilot.com/*"
>     }
>   ],
>   "allowManagedMcpServersOnly": true,
>   "strictKnownMarketplaces": [
>     {
>       "source": "github",
>       "repo": "acme-corp/approved-plugins"
>     }
>   ],
>   "sandbox": {
>     "enabled": true,
>     "failIfUnavailable": true,
>     "allowUnsandboxedCommands": false,
>     "network": {
>       "allowedDomains": [
>         "registry.npmjs.org",
>         "github.com"
>       ],
>       "allowManagedDomainsOnly": true
>     }
>   },
>   "requiredMinimumVersion": "2.1.150",
>   "cleanupPeriodDays": 7,
>   "companyAnnouncements": [
>     "Welcome to Acme Corp! Review our code guidelines at docs.example.com"
>   ]
> }
> ```

> [tab] What each key does
>
> 同一个文件，每个键上方都有注释。在这里阅读；从另一个选项卡复制，因为 Claude Code 不接受设置文件中的注释。
>
> ```jsonc managed-settings.json theme={null}
> {
>   // 仅 claude.ai 登录，且仅在此组织中
>   "forceLoginMethod": "claudeai",
>   "forceLoginOrgUUID": [
>     "xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx"
>   ],
>   // 仅 Opus 和 Sonnet 模型；使用 enforceAvailableModels，默认选项也遵守列表
>   "availableModels": [
>     "opus",
>     "sonnet"
>   ],
>   "enforceAvailableModels": true,
>   "permissions": {
>     // 在每台机器上拒绝 curl 命令和读取项目的 .env 文件和 secrets 文件夹
>     "deny": [
>       "Bash(curl *)",
>       "Read(./.env)",
>       "Read(./secrets/**)"
>     ],
>     // 从每个会话中删除绕过权限模式
>     "disableBypassPermissionsMode": "disable"
>   },
>   // 忽略来自用户、项目和本地设置的权限规则
>   "allowManagedPermissionRulesOnly": true,
>   // 仅 GitHub MCP 服务器，通过 URL 而不是名称匹配，因为用户可以
>   // 将任何服务器命名为 "github"。不匹配的用户添加的服务器不会加载，包括
>   // 当列表仅有 URL 条目时的每个 stdio 服务器。下面的 allowManagedMcpServersOnly
>   // 键使此托管列表成为唯一适用的允许列表
>   "allowedMcpServers": [
>     {
>       "serverUrl": "https://api.githubcopilot.com/*"
>     }
>   ],
>   "allowManagedMcpServersOnly": true,
>   // 插件只能来自此市场
>   "strictKnownMarketplaces": [
>     {
>       "source": "github",
>       "repo": "acme-corp/approved-plugins"
>     }
>   ],
>   // 对 Claude 运行的每个命令进行沙箱处理，如果无法设置沙箱则拒绝启动，
>   // 并且永远不要让被阻止的命令在沙箱外重试；网络
>   // 限制为 npm 和 GitHub，用户无法添加域
>   "sandbox": {
>     "enabled": true,
>     "failIfUnavailable": true,
>     "allowUnsandboxedCommands": false,
>     "network": {
>       "allowedDomains": [
>         "registry.npmjs.org",
>         "github.com"
>       ],
>       "allowManagedDomainsOnly": true
>     }
>   },
>   // 拒绝在 2.1.150 之前的版本上启动
>   "requiredMinimumVersion": "2.1.150",
>   // 7 天后删除会话记录和其他本地会话数据
>   "cleanupPeriodDays": 7,
>   // 每个用户在启动时看到的消息
>   "companyAnnouncements": [
>     "Welcome to Acme Corp! Review our code guidelines at docs.example.com"
>   ]
> }
> ```
