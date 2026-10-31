> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# 插件命令参考

> claude plugin shell 命令、会话中的 /plugin 和 /reload-plugins 的参考，以及在一个会话中加载插件的标志。

您可以从 shell 或脚本中以 `claude plugin` 的形式运行 plugin 命令，或在 Claude Code 会话中以 `/plugin` 和 `/reload-plugins` 的形式运行。本参考给出每个命令的标志、默认值、输出和退出代码，以及在一个会话中加载 plugin 的两个标志。

下表列出了每个子命令的常用选项，而非全部选项。在 shell 中运行 `claude plugin --help` 可查看您的版本具有哪些子命令，运行 `claude plugin <subcommand> --help` 可查看某个子命令的完整选项列表。

> [!NOTE]
>
> 这些情况在其他页面上有介绍：
>
> * **安装和管理步骤，以及 `/plugin` 运行的位置**：请参阅 [安装和管理 plugins](https://code.claude.com/docs/zh-CN/plugins/install.md)
> * **命令在磁盘上更改的内容以及哪个作用域优先**：请参阅 [Plugin 加载参考](https://code.claude.com/docs/zh-CN/plugins/loading.md)
> * **错误消息的含义**：请参阅 [Plugin 故障排除](https://code.claude.com/docs/zh-CN/plugins/troubleshooting.md)

## claude plugin 命令

在 Claude Code 会话外部，从 shell 或脚本运行 `claude plugin <subcommand>`。这些子命令用于安装和管理插件，无需打开 [`/plugin`](#plugin-in-a-session) 面板。

`claude plugins` 是 `claude plugin` 的别名。

每个子命令共享以下退出码、插件参数和作用域值：

* **退出码**：成功时为 `0`，失败时为 `1`。`validate` 额外使用退出码 `2` 表示意外错误，`eval` 额外使用[其章节](#plugin-eval)中列出的退出码。
* **插件参数**：`<plugin>` 参数是插件 `name` 或 `name@marketplace`。当两个市场提供相同的名称时，请使用限定形式。`configure` 仅接受限定形式。
* **作用域**：`--scope` 接受 `user`、`project` 或 `local`，用于指定命令写入的设置文件。`update` 还接受 `managed`。

### plugin init

在 `~/.claude/skills/<name>/` 处搭建新插件。除非您的管理员已[阻止 skills-directory 插件](https://code.claude.com/docs/zh-CN/plugins/loading.md#skills-directory-plugins-blocked-by-managed-settings)，否则它会在您的下一个会话中作为 `<name>@skills-dir` 加载，无需安装步骤。

`new` 是 `init` 的别名。

有关从此命令开始的创建、测试和编辑工作流，请参阅[创建插件](https://code.claude.com/docs/zh-CN/plugins/create.md)。

```bash theme={null}
claude plugin init <name> [options]
```

`<name>` 将成为 `~/.claude/skills/` 下的目录名称以及插件清单中的 `name`。

该命令没有用于指定其他位置的标志。如需在项目内搭建，请参阅[创建插件](https://code.claude.com/docs/zh-CN/plugins/create.md)。

| 标志 | 描述 |
| :- | :- |
| `--description <text>` | 清单描述 |
| `--author <name>` | 作者名称。默认为 `git config user.name` |
| `--author-email <email>` | 作者电子邮件。默认为 `git config user.email` |
| `--with <components...>` | 同时为 `skills`、`agents`、`hooks`、`mcp`、`lsp`、`output-style` 或 `channel` 搭建起始文件 |
| `-f, --force` | 覆盖目标处现有的 `.claude-plugin/` |

搭建带有起始 skill 和 hook 文件的插件：

```bash theme={null}
claude plugin init my-helper --with skills hooks
```

Claude Code 会验证其写入的内容并打印 `Created plugin "my-helper" at ~/.claude/skills/my-helper`，随后打印其加载时使用的 id，以及用于关闭它的 `claude plugin disable` 命令。

当 Claude Code 无法安全搭建时，它会以 `1` 退出且不写入任何内容，并在消息中说明原因。常见原因如下：

* 未知的 `--with` 值
* 目标处已存在搭建内容，且未使用 `--force`
* 托管的市场允许列表未包含 `skills-dir` 来源，或拒绝列表中列出了该来源。消息为 [`Plugins from ~/.claude/skills/ are blocked by your organization's managed settings`](https://code.claude.com/docs/zh-CN/plugins/troubleshooting#plugins-from-claude-skills-are-blocked-by-your-organizations-managed-s)。

### plugin install

从您已添加的市场安装插件。`i` 是 `install` 的别名。

```bash theme={null}
claude plugin install <plugin> [options]
```

大多数插件无需提示即可安装。对于其市场条目[通过运行命令来安装](https://code.claude.com/docs/zh-CN/plugins/host-marketplace.md)或[为下载设置了 `headersHelper`](https://code.claude.com/docs/zh-CN/plugins/host-marketplace#how-users-accept-a-headershelper-command) 的插件，Claude Code 会先打印该命令并询问 `Run this command now? [y/N]`。

| 标志 | 描述 |
| :- | :- |
| `-s, --scope <scope>` | 安装作用域：`user`、`project` 或 `local`。默认为 `user` |
| `--config <key=value>` | 设置插件清单声明的 [`userConfig`](https://code.claude.com/docs/zh-CN/plugins/manifest-reference) 选项。每个选项重复一次该标志。写作 `<server>.<key>` 的键改为设置[捆绑 MCP 服务器](https://code.claude.com/docs/zh-CN/plugins/components.md#include-a-packaged-mcpb-server)在其自身 `user_config` 中声明的设置，适用于随插件附带的捆绑文件。`<server>.<key>` 形式需要 Claude Code v2.1.285 或更高版本 |
| `-y, --yes` | 接受显示的安装命令，不出现 `Run this command now?` 提示。在 Claude Code 会话内运行命令时（例如从 Bash 工具或 hook 运行）会被忽略。需要 Claude Code v2.1.229 或更高版本 |
| `--accept-command <sha256>` | 代替 `-y`，接受之前某次 [`--json` 运行](#plugin-json-result)在 `shownCommand` 中报告了其 `sha256` 的显示安装命令。不能与 `-y` 组合使用。请参阅[接受显示的安装命令](#accept-a-displayed-install-command)。需要 Claude Code v2.1.271 或更高版本 |
| `--json` | 将结果作为一个 JSON 对象打印在 stdout 的最后一行，而不是人类可读的消息，供脚本使用。请参阅 [JSON 结果格式](#plugin-json-result)。需要 Claude Code v2.1.268 或更高版本 |
| `--marketplace <source>` | 从位于 `<source>` 的市场安装以裸名称指定的 `<plugin>`，如果您尚未添加该市场，则先添加它。请参阅[通过一条命令添加市场并安装](https://code.claude.com/docs/zh-CN/plugins/install.md#add-a-marketplace-and-install-in-one-command)。需要 Claude Code v2.1.292 或更高版本 |

在 shell 中运行 `claude plugin install --help`，可查看您的版本支持的所有选项。

从您自己的终端传递 `-y`，即可接受显示的命令而不出现提示。以下是没有 TTY 以及由 Claude 运行命令时的情况：

* **stdin 或 stdout 不是 TTY，且既未传递 `-y` 也未传递 `--accept-command`**：安装被拒绝。输出会说明命令仅被显示，退出码为 `1`
* **Claude 通过其 Bash 工具运行命令**：`-y` 会被忽略。请改为从您自己的终端运行该命令

为克隆该项目的所有人安装插件：

```bash theme={null}
claude plugin install formatter@my-marketplace --scope project
```

Claude Code 打印 `Successfully installed plugin: formatter@my-marketplace (scope: project)`。当没有安装任何新内容时，输出会说明原因：

* **已在该作用域安装**：输出为 `Plugin "formatter@my-marketplace" is already installed (scope: project)`，退出码为 `0`
* **您拒绝了命令源提示**：输出为 `Aborted.`，退出码为 `1`
* **您拒绝了 `headersHelper` 提示，或在没有 TTY 的情况下无法确认**：输出为 `Aborted — the command was not run.`，退出码为 `1`

#### JSON 结果格式

向 `plugin install` 传递 `--json` 时，stdout 的最后一行是一个 JSON 对象。请仅解析该行，因为 Claude Code 会在其之前打印市场声明的任何命令。

以下三个字段始终存在：

* `command`：运行的子命令，例如 `install`
* `outcome`：`ok` 或 `failed`
* `message`：结果的人类可读描述

其他字段（例如 `pluginId`、`scope` 和 `failureCode`）仅在适用时出现。

`plugin uninstall`、`plugin update`、`plugin enable` 和 `plugin disable` 上的 `--json` 选项会打印相同的对象，并带有各子命令自己的字段。

使用错误（例如无效的 `--scope`）不会打印结果行，而是以 `1` 退出，并在 stderr 上给出原因。

#### 市场命令的 JSON 结果

在 `plugin marketplace add`、`plugin marketplace remove` 和 `plugin marketplace update` 上，`--json` 会在 stdout 的最后一行打印一个 JSON 对象，包含 `command`、`outcome` 和 `message` 字段。以下是 `claude plugin marketplace remove your-marketplace --json` 的结果：

```json theme={null}
{"command":"marketplace-remove","outcome":"ok","marketplace":"your-marketplace","message":"Successfully removed marketplace: your-marketplace"}
```

`command` 的值为 `marketplace-add`、`marketplace-remove` 或 `marketplace-update`。以下字段仅在适用时出现：

* `marketplace`：命令所作用的市场名称
* `failureCode`：表示命令失败原因的代码，例如 `invalid_source`

当参数为[保留名称](https://code.claude.com/docs/zh-CN/plugins/marketplace-reference.md#reserved-names) `anthropic-plugin-directory` 时，`plugin marketplace add` 和 `plugin marketplace remove` 可能不打印结果行，因此对于该名称请检查退出码。

#### 接受显示的安装命令

当 `--json` 运行显示了市场声明的命令但未运行它时，`failed` 结果还会携带一个 `shownCommand` 对象。其字段包括显示的命令、该命令所属的插件以及命令的 `sha256`。

要恰好接受该命令，请从您自己的终端重新运行，并将该 `sha256` 作为 `--accept-command` 传入，因为该标志在 Claude Code 会话内无效。需要 Claude Code v2.1.271 或更高版本。

`sha256` 仅对完全相同的命令、插件和市场目录视为接受。如果自命令显示以来其中任何一项发生了变化，Claude Code 不会接受该 `sha256`，并会再次显示命令。运行自身的市场刷新所获取的更改也算作此类变化。

如果 `shownCommand.acceptCommandMatched` 为 `false`，则您传递的 `sha256` 与当前显示的命令不匹配。请在使用其 `sha256` 重新运行之前检查该命令。

### plugin uninstall

从一个作用域移除已安装的插件。`remove` 和 `rm` 是 `uninstall` 的别名。

```bash theme={null}
claude plugin uninstall <plugin> [options]
```

| 标志 | 描述 |
| :- | :- |
| `-s, --scope <scope>` | 从指定作用域卸载：`user`、`project` 或 `local`。默认为 `user` |
| `--keep-data` | 保留插件的持久数据目录 `~/.claude/plugins/data/<id>/` |
| `--prune` | 同时移除不再被任何剩余插件需要的自动安装[依赖项](https://code.claude.com/docs/zh-CN/plugins/dependencies.md) |
| `-y, --yes` | 跳过 `--prune` 确认提示。当 stdin 或 stdout 不是 TTY 时，与 `--prune` 一起使用时必须提供 |
| `--json` | 将结果作为一个 JSON 对象打印在 stdout 的最后一行，格式与 [`plugin install --json`](#plugin-json-result) 相同。不能与 `--prune` 组合使用。需要 Claude Code v2.1.268 或更高版本 |

从项目作用域卸载插件：

```bash theme={null}
claude plugin uninstall formatter@my-marketplace --scope project
```

Claude Code 打印 `Successfully uninstalled plugin: formatter (scope: project)`。当插件未在该作用域安装时，命令会打印以 `Failed to uninstall plugin "formatter@my-marketplace":` 开头的行并以 `1` 退出。

如果失败行后接 `"formatter" was not uninstalled:` 并指出某个设置文件，说明 Claude Code 无法确认该作用域的设置已不再启用该插件，因此插件保持安装状态，其保存的所有内容也都保留。使用 `--json` 时，结果携带 `failureCode: "settings_still_on"`。此设置检查需要 Claude Code v2.1.282 或更高版本。

#### 卸载会删除和保留的内容

当您从插件安装所在的最后一个作用域卸载插件时，Claude Code 还会删除插件存储的[选项和密钥](https://code.claude.com/docs/zh-CN/plugins/manifest-reference.md#user-configuration)及其数据目录 `~/.claude/plugins/data/<id>/`。有三个例外：

* 使用 `--keep-data` 时，数据目录会保留
* 当另一个已安装的插件使用相同的文件夹时（例如其 ID 与此插件仅在字母大小写上不同），数据目录会保留
* 当 Claude Code 从该作用域移除插件后无法读回已安装插件列表时，选项、密钥和数据目录都会保留，因为插件可能仍安装在另一个作用域。卸载仍然成功。消息会列出保留的内容以及如何删除它们，使用 `--json` 时结果携带 `savedKept: "install_records_unreadable"`

使用 `--json` 时，`keptData` 报告目录是否保留，目录保留时 `/plugin` 会显示 `· data preserved`。对于未使用 `--keep-data` 却保留的目录，此报告需要 Claude Code v2.1.281 或更高版本。`savedKept` 字段需要 Claude Code v2.1.282 或更高版本。

### plugin enable

启用已禁用的插件。对于[从 claude.ai 同步的插件](https://code.claude.com/docs/zh-CN/plugins/loading.md#synced-plugins)，请传递 `<name>@synced` 作为插件。

```bash theme={null}
claude plugin enable <plugin> [options]
```

| 标志 | 描述 |
| :- | :- |
| `-s, --scope <scope>` | 启用的作用域：`user`、`project` 或 `local`。省略时自动检测 |
| `--json` | 将结果作为一个 JSON 对象打印在 stdout 的最后一行，格式与 [`plugin install --json`](#plugin-json-result) 相同。需要 Claude Code v2.1.268 或更高版本 |

不使用 `--scope` 时，命令按 local、project、user 的顺序检查您的设置文件，并使用第一个提及该插件的作用域。

如果传递的 `--scope` 并非插件声明所在的作用域，命令要么写入覆盖，要么失败：

* **[优先级高于](https://code.claude.com/docs/zh-CN/plugins/loading.md)声明作用域的作用域**：Claude Code 在您传递的作用域写入覆盖。例如，`claude plugin disable formatter --scope local` 仅为您自己关闭在项目中启用的插件
* **任何其他作用域**：命令失败，并显示 `Plugin "formatter" is installed at project scope, not user. Use --scope project or omit --scope to auto-detect.`

如果插件已在解析出的作用域中启用，命令会打印 `Plugin "formatter" is already enabled` 并以 `1` 退出。使用 `--json` 时，结果包含 `"failureCode": "already_in_goal_state"` 和 `"alreadyInGoalState": true`，因此脚本可以将这种情况视为成功。

当插件声明了[依赖](https://code.claude.com/docs/zh-CN/plugins/dependencies.md)时，Claude Code 也会启用这些依赖。在以下情况下命令会失败：

* **某个依赖未安装**：启用失败，并为每个缺失的依赖打印 `claude plugin install` 命令
* **某个依赖被您组织的插件策略阻止**：启用失败，并指出被阻止的依赖
* **某个依赖在优先级高于目标作用域的作用域中被设置为 `false`**：启用失败。请在该作用域启用该依赖，或传递 `--scope` 以写入该作用域

在插件声明所在的任何位置重新启用插件：

```bash theme={null}
claude plugin enable formatter
```

Claude Code 打印 `Successfully enabled plugin: formatter (scope: project)`，并指出它检测到的作用域。

### plugin disable

禁用插件而不卸载它。对于[从 claude.ai 同步的插件](https://code.claude.com/docs/zh-CN/plugins/loading.md#synced-plugins)，请传递 `<name>@synced` 作为插件。

```bash theme={null}
claude plugin disable [plugin] [options]
```

| 标志 | 描述 |
| :- | :- |
| `-a, --all` | 禁用所有已启用的插件。不能与插件名称或 `--scope` 组合使用 |
| `-s, --scope <scope>` | 禁用的作用域：`user`、`project` 或 `local`。省略时自动检测 |
| `--json` | 将结果作为一个 JSON 对象打印在 stdout 的最后一行，格式与 [`plugin install --json`](#plugin-json-result) 相同。需要 Claude Code v2.1.268 或更高版本 |

不使用 `--scope` 时，作用域按与 [`plugin enable`](#plugin-enable) 相同的 local、project、user 顺序自动检测。

如果既未传递插件名称也未传递 `--all`，Claude Code 会打印 `Please specify a plugin name or use --all to disable all plugins` 并以 `1` 退出。禁用已禁用的插件会打印 `Plugin "formatter" is already disabled` 并以 `1` 退出，与 [`plugin enable`](#plugin-enable) 对已启用插件的处理方式相同。

对于仍被需要的插件，命令会失败：

* **另一个已启用的插件[依赖](https://code.claude.com/docs/zh-CN/plugins/dependencies.md)它**：命令失败，并列出需要先禁用的依赖方插件
* **您的组织要求将其作为同步插件**：命令失败且不保存任何内容

禁用一个插件：

```bash theme={null}
claude plugin disable formatter
```

Claude Code 打印 `Successfully disabled plugin: formatter (scope: project)`。

### plugin update

将插件更新到其市场提供的最新版本。新版本会在您的下一个会话中加载，或在正在运行的会话中运行 `/reload-plugins` 后加载。

```bash theme={null}
claude plugin update <plugin> [options]
```

| 标志 | 描述 |
| :- | :- |
| `-s, --scope <scope>` | 要更新的作用域：`user`、`project`、`local` 或 `managed`。省略时自动检测 |
| `-y, --yes` | 接受[命令源](https://code.claude.com/docs/zh-CN/plugins/host-marketplace.md)插件已更改的安装命令，不出现提示。当 stdin 或 stdout 不是 TTY 时必须提供，除非传递了 `--accept-command`。需要 Claude Code v2.1.229 或更高版本 |
| `--accept-command <sha256>` | 代替 `-y`，接受之前某次 [`--json` 运行](#plugin-json-result)在 `shownCommand` 中报告了其 `sha256` 的市场声明命令。不能与 `-y` 组合使用。需要 Claude Code v2.1.271 或更高版本 |
| `--json` | 将结果作为一个 JSON 对象打印在 stdout 的最后一行，格式与 [`plugin install --json`](#plugin-json-result) 相同。需要 Claude Code v2.1.268 或更高版本 |

更新插件：

```bash theme={null}
claude plugin update formatter@my-marketplace
```

Claude Code 打印 `Checking for updates for plugin "formatter@my-marketplace"…`，然后打印结果。当没有更新的版本时，它会打印 `formatter is already at the latest version (1.0.0).` 并以 `0` 退出，除非它[重试插件的依赖安装](#retry-an-unfinished-dependency-install)且该安装失败。

#### 命令更新哪个作用域

如果省略 `--scope`，命令会在当前项目中插件安装所在的最具体作用域更新插件，依次检查 local、project、user，然后是 managed。

在 v2.1.281 之前，省略 `--scope` 时命令使用 `user`，因此更新仅安装在 project 或 local 作用域的插件会失败，并显示 `Plugin "<name>" is not installed at scope user`。在这些版本上，请传递 `--scope`。

`managed` 是唯一可以更新但不能安装到的作用域。有关管理员安装的插件，请参阅[为组织管理插件](https://code.claude.com/docs/zh-CN/plugins/org.md)。

#### 按裸名称更新

您可以传递不带市场的插件名称，命令会将其与已安装的插件进行匹配。当来自不同市场的已安装插件同名时，命令会拒绝更新，并列出应改为运行的限定 `plugin-name@marketplace-name` 命令。按裸名称更新需要 Claude Code v2.1.246 或更高版本。

#### 重试未完成的依赖安装

当插件已是最新版本时，该命令还可以在其缓存副本中重试未完成的依赖安装。有关该重试会运行或被跳过的情况，请参阅[其列出的软件包未安装](https://code.claude.com/docs/zh-CN/plugins/troubleshooting.md#the-packages-it-lists-are-not-installed)。如果重试失败，输出为 `Failed to update plugin "formatter@my-marketplace"` 及原因，退出码为 `1`。在 v2.1.287 之前，该命令会报告插件已是最新版本，而不重试安装。

### plugin list

列出已安装的插件及其版本、作用域和状态。

```bash theme={null}
claude plugin list [options]
```

| 标志 | 描述 |
| :- | :- |
| `--json` | 以 JSON 格式打印列表 |
| `--available` | 同时列出您的市场提供但尚未安装的插件。不使用 `--json` 时无效 |
| `--data-size [plugin]` | 测量每个已安装插件的[已保存数据目录](#what-an-uninstall-deletes-and-keeps)，或仅测量以 `name@marketplace` 形式指定的插件。不使用 `--json` 时无效。如果该名称没有安装记录，命令会打印 `--data-size names a plugin that is not installed` 并以 `1` 退出，而不是打印列表。需要 Claude Code v2.1.285 或更高版本 |

Claude Code 按各插件的加载方式对人类可读的输出进行分组：

* **`Installed plugins:`**：您从市场安装的插件
* **`Session-only plugins (--plugin-dir / --plugin-url):`**：由同一命令中的这些标志加载的插件，如 `claude --plugin-dir ./my-plugin plugin list`
* **`Skills-directory plugins (.claude/skills/*):`**：Claude Code 在 skills 目录中找到的插件
* **`Synced from claude.ai`**：[从您的 claude.ai 账户同步的插件](https://code.claude.com/docs/zh-CN/plugins/loading.md#synced-plugins)

当所有分组都为空时，Claude Code 打印 ``No plugins installed. Use `claude plugin install` to install a plugin.``

#### JSON 输出

使用 `--json` 时，Claude Code 打印一个数组，每个安装对应一个对象。每个对象携带以下字段。`id`、`version`、`scope`、`enabled` 和 `installPath` 始终存在，其他字段仅在适用时出现。

| 字段 | 类型 | 描述 |
| :- | :- | :- |
| `id` | string | 安装的插件为 `name@marketplace`，仅限会话的插件为 `name@inline`，skills-directory 插件为 `name@skills-dir`，从 claude.ai 同步的插件为 `name@synced` |
| `version` | string | 对于市场安装，为 [Claude Code 在安装时计算的版本](https://code.claude.com/docs/zh-CN/plugins/loading.md#versions-and-updates)。对于仅限会话、skills-directory 或同步插件，为清单的 `version`，未声明时为 `unknown` |
| `scope` | string | 安装的插件为 `user`、`project`、`local` 或 `managed`；skills-directory 插件为 `user` 或 `project`；仅限会话的插件为 `session`；从 claude.ai 同步的插件为 `synced` |
| `enabled` | boolean | 插件在合并后的设置中是否启用 |
| `installPath` | string | 插件加载所在的目录，但会话从其市场文件夹中[就地加载](https://code.claude.com/docs/zh-CN/plugins/loading.md#in-place-and-copied-plugins)的插件除外 |
| `readFromFolder` | string | 对于会话从其市场文件夹中[就地加载](https://code.claude.com/docs/zh-CN/plugins/loading.md#in-place-and-copied-plugins)的插件，为该文件夹内插件的源目录。需要 Claude Code v2.1.289 或更高版本 |
| `folderVersion` | string | 与 `readFromFolder` 一起出现，为 Claude Code 从该文件夹加载插件时插件的 `version`，可能与上面的 `version` 字段不同。插件未加载或未声明版本时不存在。需要 Claude Code v2.1.289 或更高版本 |
| `installedAt` | string | 安装的 ISO 时间戳。仅限市场安装 |
| `lastUpdated` | string | 最后更新的 ISO 时间戳。仅限市场安装 |
| `projectPath` | string | 安装所属的项目。仅限 `project` 和 `local` 作用域 |
| `mcpServers` | object | 插件的 MCP 服务器定义，仅当市场安装的插件包含 MCP 服务器时出现 |
| `errors` | array of strings | 加载错误，仅当插件加载失败时出现 |
| `notes` | array of strings | 非加载错误的警告，例如编写问题或[未安装的软件包](https://code.claude.com/docs/zh-CN/plugins/loading.md#when-the-dependency-install-fails-or-is-skipped) |
| `errorDetails` | array of objects | 每个 `errors` 条目对应一个对象，给出其诊断 `type` 以及它所引用的名称，例如插件、市场、服务器或文件。需要 Claude Code v2.1.268 或更高版本 |
| `noteDetails` | array of objects | 每个 `notes` 条目对应的相同详细对象。需要 Claude Code v2.1.268 或更高版本 |
| `hasUserConfig` | boolean | 当插件已加载且其清单声明了 [`userConfig` 选项](https://code.claude.com/docs/zh-CN/plugins/manifest-reference#user-configuration)时存在且为 `true`。对于加载失败的插件，无论其清单声明什么，该字段都不存在。永远不包含已保存的值。需要 Claude Code v2.1.285 或更高版本 |
| `projectEnabled` | boolean | 项目共享的 `.claude/settings.json` 是否启用该插件。仅限市场安装。需要 Claude Code v2.1.285 或更高版本 |
| `dataDirSize` | object | 使用 `--data-size` 时，以 `bytes` 和 `human` 表示的插件[已保存数据目录](#what-an-uninstall-deletes-and-keeps)大小；目录缺失或为空时不存在。仅限市场安装。需要 Claude Code v2.1.285 或更高版本 |
| `dataDirUnreadable` | boolean | 使用 `--data-size` 时，如果已保存的数据目录存在但无法测量，则为 `true`。仅限市场安装。需要 Claude Code v2.1.285 或更高版本 |

使用 `--json --available` 时，Claude Code 打印一个对象而不是数组。其 `installed` 字段包含已安装插件对象的数组，其 `available` 字段为每个未安装的市场插件包含一个对象，带有以下字段。

| 字段 | 类型 | 描述 |
| :- | :- | :- |
| `pluginId` | string | `name@marketplace` |
| `name` | string | 插件在市场中的名称 |
| `marketplaceName` | string | 提供该插件的市场 |
| `source` | string or object | 市场条目的 [source](https://code.claude.com/docs/zh-CN/plugins/marketplace-reference.md)：相对路径为字符串，否则为对象 |
| `description` | string | 条目的描述（如有） |
| `version` | string | 条目的版本（如有声明） |
| `installCount` | number | 安装次数（当 Claude Code 有该插件的安装次数时） |

### plugin details

显示插件的组件清单及其预计 token 成本。

插件必须已加载：已安装、在 skills 目录中找到，或在同一命令中通过 `--plugin-dir` 或 `--plugin-url` 传入。`<name>` 是插件 `name` 或 `name@marketplace`。

```bash theme={null}
claude plugin details <name>
```

该命令除 `--help` 外不接受任何标志。

显示已安装插件提供的内容：

```bash theme={null}
claude plugin details formatter
```

Claude Code 打印插件的名称、版本、描述和来源，然后打印以下部分：

* **`Component inventory`**：插件的 skill、Agent、hook、MCP 服务器和 LSP 服务器
* **`Projected token cost`**：插件添加到每个会话的常驻 token
* **`Per-component (rounded)`**：每个 skill、Agent 和命令的常驻和调用时估算值。插件没有这些组件时省略

有关这两个成本数字的含义，请参阅[衡量插件成本和使用情况](https://code.claude.com/docs/zh-CN/plugins/measure.md)。

对于未加载的插件，Claude Code 打印以下消息并以 `1` 退出：

```text theme={null}
Plugin "formatter" not found. Run `claude plugin list` to see installed plugins, or pass --plugin-dir <path> to load one from disk.
```

以 `@builtin` 结尾的 ID 指代某个 [Claude Code 内置插件](https://code.claude.com/docs/zh-CN/plugins/mods/overview.md#mods-built-into-claude-code)。如果您传递此类 ID 而 Claude Code 找不到该插件，消息将改为：

```text theme={null}
Plugin "<name>@builtin" not found. Built-in plugins don't appear in `claude plugin list`. To see the ones available in a session, run /plugin in Claude Code and open the Installed tab.
```

### plugin configure

显示已安装插件的 [`userConfig`](https://code.claude.com/docs/zh-CN/plugins/manifest-reference#user-configuration) 选项及哪些已设置，或保存通过 stdin 管道传入的值。需要 Claude Code v2.1.285 或更高版本。

```bash theme={null}
claude plugin configure <plugin>
```

| 标志 | 描述 |
| :- | :- |
| `--values-stdin` | 从 stdin 读取以单行字符串组成的 JSON 对象形式的选项值并保存。未提供的选项保留其已保存的值 |
| `--json` | 将结果作为一个 JSON 对象打印在 stdout 上。不使用 `--values-stdin` 时，该对象携带选项的 `schema` 和 `choices`、它们的初始 `inputs`，以及 `configured` 和 `unconfigured` 选项名称。使用 `--values-stdin` 时，它携带 `saved` 选项名称，以及在可读回时的 `unconfigured` 选项名称 |

不使用标志时，命令列出每个选项，最多带三个标签：`required` 或 `optional`，然后是用于清单声明为敏感的选项的 `sensitive`，然后是 `set` 或 `not set`。它不打印已保存的值。使用 `--json` 时，输出包括非敏感选项的已保存值，但永远不包括敏感选项的文本。

要保存值，请将其写入一个文件，作为将选项键映射到字符串值的 JSON 对象，然后通过 stdin 传入该文件。将 `formatter@my-marketplace` 替换为 `claude plugin list` 中显示的您自己插件的 id。此示例从包含 `{"api_url": "https://example.com"}` 的文件 `values.json` 设置一个名为 `api_url` 的选项：

```bash theme={null}
claude plugin configure formatter@my-marketplace --values-stdin < values.json
```

Claude Code 根据选项声明的类型验证每个值，并打印 `Configuration saved. Restart Claude Code to apply it.` 如果传递了清单未声明的键，或未通过验证的值，命令不保存任何内容，打印 `Failed to save configuration:` 及原因，并以 `1` 退出。使用 `--json` 时，被拒绝的值还会在 stdout 上打印一个对象，其 `refused` 字段携带 `message`，以及在某个选项出错时该选项的 `option` 键。

请传递 `claude plugin list` 中显示的插件完整 `name@marketplace` id。`configure` 不接受裸 `name`。当没有已加载的插件具有该 id 时，命令打印 `No installed plugin has the id "<plugin>".` 并以 `1` 退出。

有关捆绑 MCP 服务器的设置，请参阅 [`plugin install --config`](#plugin-install) 或 `/plugin` 中的 **Configure** 项。

### plugin prune

移除不再被任何已安装插件需要的自动安装[依赖项](https://code.claude.com/docs/zh-CN/plugins/dependencies.md)。该命令永远不会移除您自己安装的插件。`autoremove` 是 `prune` 的别名。

```bash theme={null}
claude plugin prune [options]
```

| 标志 | 描述 |
| :- | :- |
| `-s, --scope <scope>` | 在指定作用域清理：`user`、`project` 或 `local`。默认为 `user` |
| `--dry-run` | 列出将被移除的内容，但不实际移除 |
| `-y, --yes` | 跳过确认提示。当 stdin 或 stdout 不是 TTY 时必须提供 |

预览清理将移除的内容：

```bash theme={null}
claude plugin prune --dry-run
```

Claude Code 列出孤立的依赖项，并以 `(dry run — nothing removed)` 结尾。没有可移除的内容时，它会打印以 `Nothing to prune` 开头的行。

不使用 `--dry-run` 时，命令仅在您于提示处确认或传递 `-y` 后才移除孤立的依赖项。

无论您在提示处如何回答，退出码都是 `0`。

`prune` 的行为取决于是否连接了终端以及是否传递了 `-y`：

| 终端和标志 | 发生的情况 |
| :- | :- |
| 交互式终端，无 `-y` | 列出孤立的依赖项并询问 `Remove? [y/N]` |
| 任何终端，`-y` | 移除它们并打印 `Removed N auto-installed plugins: <names>` |
| 非 TTY stdin 或 stdout，无 `-y` | 打印列表并显示 ``Not a TTY — run `claude plugin prune -y` to remove.``，不移除任何内容 |

### plugin eval

运行插件的 [eval 案例](https://code.claude.com/docs/zh-CN/plugin-evals.md)并报告评分结果。需要 Claude Code v2.1.269 或更高版本。

每个案例由一个提示词加评分器组成。Claude Code 在仅加载目标插件的隔离会话中多次运行它，并且默认还会在不加载插件的情况下运行，以便报告显示两者的差异。

有关案例格式、评分器、结果和 CI 用法，请参阅[使用 evals 测试插件](https://code.claude.com/docs/zh-CN/plugin-evals.md)。

```bash theme={null}
claude plugin eval [target] [options]
```

可选的 `target` 默认为当前目录，可采用以下任一形式：

* 插件目录
* 单个 `prompt.md` 或 `case.yaml` 文件
* 以 `name` 或 `name@marketplace` 形式指定的已安装插件
* `name@skills-dir`

请将目标放在 `--tag`、`--allow-tools` 和 `--json` 之前。这些选项会将其后的单词都作为自己的值，因此写在它们之后的目标会被读作标签、工具名称或 JSON 输出路径，而不是目标。

此表列出大多数运行使用的选项。运行 `claude plugin eval --help` 可查看完整选项集，包括 `--case`、`--tag`、`--output-dir`、`--report`、`--allow-real-servers`、`--keep-temp` 和 `--verbose`。

| 选项 | 描述 | 默认值 |
| :- | :- | :- |
| `--runs <n>` | 每个 [arm](https://code.claude.com/docs/zh-CN/plugin-evals.md#compare-against-a-no-plugin-baseline) 中每个案例的运行次数 | 每个案例的 `runs`，否则为 3 |
| `-j, --concurrency <n>` | 同时运行的 Agent 会话数，1 到 8。它们共享您的速率限制 | `1` |
| `--model <model>` | 被测 Agent 使用的模型 | 每个案例的 `model`；否则如已设置则为 `ANTHROPIC_MODEL`；否则为 Claude Code 的默认模型 |
| `--judge-model <model>` | `llm` 和 `baseline` 评分器使用的模型 | [后台任务](https://code.claude.com/docs/zh-CN/plugin-evals.md#grade-the-result)使用的模型 |
| `--ablation <mode>` | `none` 或 `with-without`。请参阅[根据无插件基线评分](https://code.claude.com/docs/zh-CN/plugin-evals.md#compare-against-a-no-plugin-baseline) | 按案例决定，如该章节所述 |
| `--threshold <0..1>` | 如果任何案例的得分低于此值，则以 1 退出 | `1.0` |
| `--max-cost-usd <usd>` | 一旦支出达到此值，在下一次运行前停止，以 2 退出，并报告部分结果 | 无限制 |
| `--allow-tools <tools...>` | 授予只读工具集之外的工具，例如 `Bash`、`Write`、`Edit` 或 `"mcp__plugin_<plugin>_<server>__*"`。请参阅[授予工具](https://code.claude.com/docs/zh-CN/plugin-evals.md#grant-tools) | |
| `--scaffold` | 运行每个案例的 [`scaffold_script`](https://code.claude.com/docs/zh-CN/plugin-evals#add-setup-or-history-with-case-yaml) | 关闭 |
| `--trust-plugin` | 跳过首次运行的信任提示，用于 CI。请参阅[运行可以访问的内容](https://code.claude.com/docs/zh-CN/plugin-evals.md#security) | 关闭 |
| `--mocks <mode>` | `record` 或 `off`。请参阅[模拟 MCP 服务器](https://code.claude.com/docs/zh-CN/plugin-evals.md#mock-mcp-servers) | `record` |
| `--eval-dir <dir>` | 插件下存放案例的目录 | 清单的 `experimental.evals`，否则为 `evals` |
| `--json [path]` | 将[结果文档](https://code.claude.com/docs/zh-CN/plugin-evals.md#json-result)打印到 stdout，或写入 `.json` 路径 | |
| `--no-publish` | 将 HTML 报告保留在本地 | |

退出码反映运行的结束方式。如需在流水线中据此采取操作，请参阅[在 CI 中运行 evals](https://code.claude.com/docs/zh-CN/plugin-evals.md#run-evals-in-ci)。

| 退出码 | 含义 |
| :- | :- |
| `0` | 套件通过：每个案例都达到阈值，且没有其他原因导致运行失败 |
| `1` | 存在失败的案例、加载错误、不受信任的插件目录、[因 API 错误而结束](https://code.claude.com/docs/zh-CN/plugin-evals.md#runs-ended-on-an-api-error)的运行，或[在 CI 中运行 evals](https://code.claude.com/docs/zh-CN/plugin-evals.md#run-evals-in-ci) 下列出的其他原因 |
| `2` | 部分运行 |
| `130` | 被中断 |
| `143` | 被终止 |

### plugin eval init

为当前目录中的插件创建 eval 套件。需要 Claude Code v2.1.269 或更高版本。请参阅[创建您的第一个 eval 套件](https://code.claude.com/docs/zh-CN/plugin-evals.md#create-your-first-eval-suite)。

```bash theme={null}
claude plugin eval init [name] [options]
```

请从插件的根文件夹运行该命令，即包含 `.claude-plugin/plugin.json` 或 skill 的 `SKILL.md` 的目录。如需有意在其他目录中搭建套件，请传递 `--eval-dir`。

在终端中，该命令会打开一个交互式 Claude Code 会话进行编写访谈。在访谈中，Claude 会执行以下操作：

1. 读取插件
2. 询问您插件应擅长做什么
3. 提议案例和评分器
4. 写入案例文件
5. 运行案例并与您一起审查评分，以检查评分器的打分方式是否与您一致

使用 `--bare` 或没有终端时，该命令改为写入一个空白的单案例模板。当 Claude 从 Claude Code 会话内运行该命令时，命令会打印供该会话遵循的访谈说明，而不是写入模板。

可选的 `name` 是案例名称。使用 `--bare` 或没有终端时必须提供，因为命令会为该案例写入空白模板。案例名称以字母或数字开头，且仅包含字母、数字、`.`、`_` 和 `-`。在所有平台上，命令还会拒绝 Windows 无法存储的名称，例如 `con` 或以 `.` 结尾的名称。

该命令接受以下选项：

| 选项 | 描述 | 默认值 |
| :- | :- | :- |
| `--bare` | 为 `<name>` 写入空白的 `prompt.md` 和 `graders/criteria.md`，而不是运行访谈 | |
| `-i, --interactive` | 要求进行访谈。没有终端时失败，而不是写入模板 | |
| `--eval-dir <dir>` | 当前目录下写入案例的目录 | 清单的 `experimental.evals`，否则为 `evals` |

### plugin tag

为插件发布创建名为 `<name>--v<version>` 的带注释 git 标签。在打标签之前，命令会检查插件的 `plugin.json` 与列出该插件的任何市场条目在版本上是否一致。

有关何时为发布打标签，请参阅[发布插件](https://code.claude.com/docs/zh-CN/plugins/publish.md)。

```bash theme={null}
claude plugin tag [path] [options]
```

`[path]` 是插件目录，默认为当前目录。命令从该目录向上查找，直到找到列出该插件的 `.claude-plugin/marketplace.json`，以此定位市场条目。

| 标志 | 描述 |
| :- | :- |
| `--push` | 创建标签后将其推送到 `--remote` |
| `--dry-run` | 打印将要打的标签，但不创建标签 |
| `-f, --force` | 跳过工作树不干净和标签已存在的检查 |
| `-m, --message <msg>` | 标签注释消息。`%s` 代表版本。默认为 `<name> <version>` |
| `--remote <name>` | 使用 `--push` 时推送到的远程。默认为 `origin` |

预览市场检出中某个插件的标签：

```bash theme={null}
claude plugin tag plugins/formatter --dry-run
```

Claude Code 打印计划：

* 插件名称
* 版本及其来源文件
* 匹配的市场条目（如有）
* 标签名称
* 它将运行的 `git tag` 和 `git push` 命令

不使用 `--dry-run` 时，Claude Code 打印 `Created tag formatter--v1.0.0`，并打印 `Pushed to origin` 或需要您自己运行的推送命令。如果推送失败，标签仍会在本地创建，命令以错误退出。

当无法安全打标签时，命令以 `1` 退出并打印原因。常见原因如下：

* `plugin.json` 或市场条目中没有 `version`
* 标签已存在
* 工作树不干净

### plugin test

运行 [mod](https://code.claude.com/docs/zh-CN/plugins/mods/overview.md) 的测试，mod 是通过代码注册事件处理程序的插件。该命令无需会话、登录或网络。有关如何编写测试，请参阅[测试 mod](https://code.claude.com/docs/zh-CN/plugins/mods/test.md)。

```bash theme={null}
claude plugin test [directory]
```

`[directory]` 是 mod 的目录，默认为当前目录。该命令会运行其下所有名称以 `.test.ts` 或 `.test.tsx` 结尾的文件，并在有测试失败时以状态 1 退出。

运行 `./first-mod` 中 mod 的测试：

```bash theme={null}
claude plugin test ./first-mod
```

### plugin validate

验证插件清单、市场清单或目录中的 skill、Agent 和命令，并以 CI 作业可据此操作的退出码退出。有关创建、测试和编辑工作流，请参阅[创建插件](https://code.claude.com/docs/zh-CN/plugins/create.md)。有关验证器在各清单中检查的内容，请参阅[插件清单参考](https://code.claude.com/docs/zh-CN/plugins/manifest-reference.md)和[市场参考](https://code.claude.com/docs/zh-CN/plugins/marketplace-reference.md)。

```bash theme={null}
claude plugin validate <path> [options]
```

| 标志 | 描述 |
| :- | :- |
| `--strict` | 将警告视为错误，使运行时可容忍的未识别字段和缺失元数据导致运行失败 |
| `--json` | 将验证报告输出为一个 JSON 对象，退出码相同。需要 Claude Code v2.1.259 或更高版本 |

在提交前验证插件：

```bash theme={null}
claude plugin validate ./my-plugin --strict
```

#### 验证目录

`<path>` 是清单文件或目录。给定目录时，Claude Code 根据在其中找到的内容选择要验证的对象：

* `.claude-plugin/marketplace.json`（如果存在）
* 否则为 `.claude-plugin/plugin.json`
* 否则为组件文件，根据目录名称选择。在没有清单的情况下验证组件文件需要 Claude Code v2.1.233 或更高版本：
  * 名为 `skills`、`agents` 或 `commands` 的目录：其中的文件
  * 名为 `.claude` 的目录：其中的 `skills`、`agents` 和 `commands` 目录
  * 任何其他目录：其 `.claude` 下的这三个目录

当目录同时包含 `.claude-plugin/marketplace.json` 和 `.claude-plugin/plugin.json` 时，Claude Code 会验证市场，同时也验证插件的清单和组件文件。这需要 Claude Code v2.1.289 或更高版本。

Claude Code 不会跟随您指定的目录内的符号链接。其行为取决于链接所在的位置：

* **插件或 `.claude` 根目录下作为链接的 `skills`、`agents` 或 `commands` 目录**：Claude Code 会警告其中的任何内容都未被读取。
* **`skills`、`agents` 或 `commands` 目录内的链接条目**：Claude Code 会跳过它，并按目录警告跳过了多少会话本会加载的条目。
* **您指定的 `skills`、`agents` 或 `commands` 目录本身是符号链接，或其父级 `.claude` 目录是符号链接**：Claude Code 报告错误，不检查其中的任何内容。请改为指定真实目录。

有少数文件不会被验证运行读取：

* **插件根目录下的 `SKILL.md`**：针对插件目录运行 `claude plugin validate` 时，Claude Code 不会检查插件根目录下的 `SKILL.md`
* **插件根目录下的 `CLAUDE.md`**：在插件运行中，Claude Code 还会对插件根目录下的 `CLAUDE.md` 发出警告
* **市场运行中的插件文件**：从市场目录运行时，Claude Code 不会打开市场在其他目录中列出的插件的 skill、Agent、命令或 hook 文件，也不会打开它们捆绑的 MCP 服务器文件。要查找这些文件中的错误，请分别验证每个插件目录

#### 输出和退出码

Claude Code 打印所验证的文件、所有错误和警告及其路径，以及一行结论。退出码与结论一致：

| 退出码 | 结论行 | 含义 |
| :- | :- | :- |
| `0` | `Validation passed` 或 `Validation passed with warnings` | 清单可以加载。使用 `--strict` 时，也没有警告 |
| `1` | `Validation failed` 或 `Validation failed (--strict treats warnings as errors)` | 存在错误，或在 `--strict` 下存在警告。无论是否使用 `--strict`，[建议](#advice-on-the-readme)都不计入 |
| `2` | `Unexpected error during validation: <reason>` | 验证器本身失败，例如遇到不可读的路径 |

使用 `--json` 时，Claude Code 将报告作为一个 JSON 对象写入 stdout，包含以下顶级字段：

* `success`：与退出码相同的结论
* `strict`：运行是否将警告视为错误
* `target`：Claude Code 验证的解析后路径
* `manifest`：清单自身的结果，对于没有清单的运行为 `null`
* `contents`：每个文件的结果，各自指明其 `file`，并携带 `errors`、`warnings` 和 `notes` 数组。没有错误、警告或说明的文件不会有条目，因此没有可报告的内容时 `contents` 为空。对于[同时包含 `marketplace.json` 和 `plugin.json` 的目录](#validate-a-directory)，`manifest` 为 `marketplace.json` 的结果，`plugin.json` 的结果放在 `contents` 中
  * `gatingHooks`：每个可以拒绝操作的 [mod](https://code.claude.com/docs/zh-CN/plugins/mods/overview.md) hook（例如 `tool.call` hook）是否具有 [`.catch` 处理程序](https://code.claude.com/docs/zh-CN/plugins/mods/events#handle-a-hook-that-fails)。每一项给出 `module`、`pattern`、`hook` 和 `hasCatch`。需要 Claude Code v2.1.290 或更高版本
* `advice`：本次运行给出的[建议](#advice-on-the-readme)，每个条目以 `file` 指明 README，并携带 `message` 以及要粘贴的 `installLine`。没有建议时为空。需要 Claude Code v2.1.295 或更高版本

以 `2` 退出时，命令不向 stdout 写入任何内容。错误消息输出到 stderr。

#### 关于 README 的建议

`claude plugin validate` 输出中的 `Advice` 块表示某个插件的 README 中没有从市场安装该插件的行，并为您提供一行可添加的内容。建议既不是错误也不是警告。无论是否使用 `--strict`，它都不会改变结论行或退出码。它需要 Claude Code v2.1.295 或更高版本。

例如，验证 `acme-tools` 市场的目录时，会为其 `deploy-helper` 插件打印以下建议：

```text theme={null}
Validating marketplace manifest: /path/to/acme-tools/.claude-plugin/marketplace.json

ℹ Advice (does not change the verdict):

  ❯ /path/to/acme-tools/plugins/deploy-helper/README.md: This README has no line that installs "deploy-helper" from a marketplace. Add the install line, which works in one step on Claude Code 2.1.275 or later.
      /plugin install deploy-helper --marketplace acme/tools

✔ Validation passed
```

要清除该建议，请将建议的行粘贴到其指出的 README 中，然后在 shell 中再次运行 `claude plugin validate`。`Advice` 块将消失。

如果建议的行包含占位符 `<owner>/<repo>`，原样粘贴不会清除该建议。请先获取真实来源：

* **您验证的是插件自身的目录**：当市场从其自身内部的某个目录列出该插件时，请改为验证该市场的目录。当市场从其他位置（例如 GitHub 仓库）列出该插件时，请自行填写 `<owner>/<repo>`
* **您验证的是市场的目录**：Claude Code 会根据该目录顶层 `.git` 目录的 `origin` 远程填写来源。当无法使用该远程时（例如不存在远程），请自行将 `<owner>/<repo>` 替换为您市场的来源

当 `plugin install` 或其简写形式 `plugin i` 在同一行后接以下任一内容时，您已编写的安装行会被计入。`/plugin` 和 `claude plugin` 两种写法都计入：

* `<plugin>@<marketplace>`，例如 `/plugin install deploy-helper@acme-tools`
* 插件名称和 `--marketplace <source>`，例如 `/plugin install deploy-helper --marketplace acme/tools`

## claude plugin marketplace 命令

从您的 shell 运行 `claude plugin marketplace <subcommand>` 来添加、列出、刷新和移除您安装插件的市场。

* **退出码**：这些子命令遵循插件命令的[退出码约定](#claude-plugin-commands)
* **作用域**：它们的 `--scope` 标志没有 `-s` 短形式

关于市场是什么以及 Claude Code 如何缓存它，请参阅[插件加载参考](https://code.claude.com/docs/zh-CN/plugins/loading.md)。

### plugin marketplace add

从 GitHub 仓库、git URL、托管的 `marketplace.json` 或本地路径添加市场，并在设置文件中声明它。

添加后，Claude Code 会安装您已安装的插件缺失的任何[依赖项](https://code.claude.com/docs/zh-CN/plugins/dependencies.md)。

```bash theme={null}
claude plugin marketplace add <source> [options]
```

| 标志 | 描述 |
| :- | :- |
| `--scope <scope>` | 声明市场的设置文件：`user`、`project` 或 `local`。默认为 `user` |
| `--sparse <paths...>` | 将 git 检出限制在这些目录，用于 monorepos。仅限 `github` 和 `git` 源 |
| `--claudeai` | 将参数读取为[托管在 claude.ai 上的市场](https://code.claude.com/docs/zh-CN/plugins/install.md#add-from-claude-ai)的名称，而不是源。需要 Claude Code v2.1.273 或更高版本 |
| `--json` | 以 [JSON 结果格式](#plugin-json-result)在 stdout 的最后一行以一个 JSON 对象打印命令是否成功及其消息。与 `--claudeai` 一起使用时无效。需要 Claude Code v2.1.287 或更高版本 |

`<source>` 采用下表中的任何形式，其形式决定了源类型以及 Claude Code 如何获取市场。关于生成的源对象，请参阅[市场参考](https://code.claude.com/docs/zh-CN/plugins/marketplace-reference.md)。

| 您输入的 | 源类型 | Claude Code 如何获取它 |
| :- | :- | :- |
| `owner/repo`、`owner/repo#ref` 或 `owner/repo@ref` | `github` | 克隆 GitHub 仓库，给定时固定到 `ref`。所有者和仓库必须遵循 GitHub 命名规则 |
| `user@host:path[.git][#ref]` | `git` | 通过 SSH 克隆 |
| 以 `.git[#ref]` 结尾或包含 `/_git/` 的 `http://` 或 `https://` URL，例如 `https://example.com/repo.git` | `git` | 克隆 URL，包括 Azure DevOps URL |
| `https://github.com/owner/repo` 或 `https://gitlab.com/namespace/project`，或相同的 `http://` 形式 | `git` | 在追加 `.git` 后克隆 URL |
| 任何其他 `http://` 或 `https://` URL，包括没有 `.git` 的自托管 git 主机 | `url` | 将 URL 作为 `marketplace.json` 获取。要改为克隆那里的仓库，请追加 `.git` |
| `./path`、`../path`、`/path` 或 `~/path` 到目录 | `directory` | 就地读取目录。在 Windows 上，`.\`、`..\` 和 `C:\` 形式也可以工作 |
| 相同的路径形式，到 `.json` 文件 | `file` | 就地读取文件 |

对于克隆 URL 不带 `.git` 后缀的主机（如 AWS CodeCommit），请改为在 [`extraKnownMarketplaces`](https://code.claude.com/docs/zh-CN/settings-reference#extraknownmarketplaces) 中将市场添加为 git 条目。Claude Code 克隆 git 条目，无论其 URL 是否以 `.git` 结尾。

Claude Code 也克隆具有嵌套子组的 `gitlab.com` URL，例如 `https://gitlab.com/group/subgroup/project`。

添加市场并与项目共享：

```bash theme={null}
claude plugin marketplace add your-org/your-marketplace --scope project
```

Claude Code 打印 `Successfully added marketplace: your-marketplace (declared in project settings)`，使用市场自己清单中的 `name`。重复添加或无效源会改为打印以下结果之一：

* **市场已在磁盘上**：输出为 `Marketplace 'your-marketplace' already on disk — declared in project settings`，退出码为 `0`
* **无法识别的源**：输出为 `Invalid marketplace source format. Try: owner/repo, https://..., or ./path`，退出码为 `1`
* **裸主机，如 `gitlab.example.com/team/plugins`**：添加失败，作为无效的 `owner/repo` 简写，消息告诉您添加 `https://` 或使用本地路径

通过 `claude plugin marketplace list` 的 `From claude.ai:` 部分中打印的名称添加[托管在 claude.ai 上的市场](https://code.claude.com/docs/zh-CN/plugins/install.md#add-from-claude-ai)：

```bash theme={null}
claude plugin marketplace add --claudeai claudeai-organization-library
```

使用 `--claudeai` 时，命令拒绝 `--scope` 和 `--sparse`。市场为您的账户托管，未在设置文件中声明，因此您无法通过项目的 `.claude/settings.json` 共享它。

### plugin marketplace list

列出您添加的每个市场及其源。

```bash theme={null}
claude plugin marketplace list [options]
```

| 标志 | 描述 |
| :- | :- |
| `--json` | 将列表打印为 JSON |

Claude Code 打印 `Configured marketplaces:` 和每个市场一行 `Source:`，或 `No marketplaces configured`。

使用 `--json` 时，Claude Code 打印一个数组，每个市场一个对象，包含下面的字段。每个字段都是字符串。

| 字段 | 描述 |
| :- | :- |
| `name` | 市场的名称 |
| `source` | `github`、`git`、`url`、`directory`、`file` 或 `claudeai` |
| `repo` | `owner/repo`。仅限 `github` 源 |
| `url` | 克隆或获取 URL。仅限 `git` 和 `url` 源 |
| `path` | 本地路径。仅限 `directory` 和 `file` 源 |
| `ref` | 固定的分支或标签。`github` 和 `git` 源，仅在固定时 |
| `installLocation` | Claude Code 缓存市场的位置 |

添加的 [claude.ai 市场](https://code.claude.com/docs/zh-CN/plugins/install.md#add-from-claude-ai)没有本地克隆，因此其条目在 `installLocation` 的位置携带其 claude.ai 标识符 `marketplaceId` 和 `organizationUuid`。它也在记录时携带 `scope` 和 `status`。

如果您的终端会话[从您的 claude.ai 账户同步插件](https://code.claude.com/docs/zh-CN/plugins/loading.md#synced-plugins)，文本列表以 `From claude.ai:` 部分结尾。该部分列出 claude.ai 为您的账户列出但您尚未添加的市场，包括基于 git 的和托管的。它需要 Claude Code v2.1.273 或更高版本。

要从该部分添加市场，请参阅[从 claude.ai 添加市场](https://code.claude.com/docs/zh-CN/plugins/install.md#add-from-claude-ai)。

`--json` 输出仅覆盖已配置的市场，并排除该部分。

### plugin marketplace remove

从您的设置中移除市场的声明。`rm` 是 `remove` 的别名。

> [!WARNING]
>
> 当您从最后一个声明市场的作用域中移除市场时，Claude Code 也会删除其缓存并卸载您从中安装的每个插件。它也会删除它们保存的[选项和密钥](https://code.claude.com/docs/zh-CN/plugins/manifest-reference.md#user-configuration)和[数据](https://code.claude.com/docs/zh-CN/plugins/components.md#path-variables-and-persistent-data)（如果可以的话）。
>
> 要在不丢失其插件的情况下刷新市场，请改为运行 `plugin marketplace update`。

```bash theme={null}
claude plugin marketplace remove <name> [options]
```

`<name>` 是 `plugin marketplace list` 显示的市场名称，而不是您传递给 `add` 的源。

| 标志 | 描述 |
| :- | :- |
| `--scope <scope>` | 从一个设置作用域中移除声明：`user`、`project` 或 `local`。不使用它时，Claude Code 从每个作用域中移除声明 |
| `--json` | 以 [JSON 结果格式](#plugin-json-result)在 stdout 的最后一行以一个 JSON 对象打印命令是否成功及其消息。需要 Claude Code v2.1.287 或更高版本 |

从每个作用域中移除市场：

```bash theme={null}
claude plugin marketplace remove your-marketplace
```

Claude Code 打印 `Successfully removed marketplace: your-marketplace`。当命令卸载插件时，输出在诸如 `Also uninstalled 2 plugins from this marketplace:` 的行下列出它们。要再次使用其中一个，请添加市场并重新安装插件。

如果您限定作用域到不声明市场的设置文件，命令失败，显示 `Marketplace 'your-marketplace' is not declared in project settings. Omit --scope to remove it from all scopes.`

### plugin marketplace update

从其源刷新一个市场或每个市场，以获取新插件和版本。使用分支或标签 `ref` 添加的市场更新到该 ref 的最新提交，而不是仓库的默认分支。

```bash theme={null}
claude plugin marketplace update [name] [options]
```

| 标志 | 描述 |
| :- | :- |
| `--json` | 以 [JSON 结果格式](#plugin-json-result)在 stdout 的最后一行以一个 JSON 对象打印命令是否成功及其消息。未提供名称时，命令拒绝 `--json` 并以 `1` 退出。需要 Claude Code v2.1.287 或更高版本 |

刷新一个市场：

```bash theme={null}
claude plugin marketplace update your-marketplace
```

Claude Code 打印 `Successfully updated marketplace: your-marketplace`。当您省略名称时，它打印计数，如 `Successfully updated 2 marketplaces`。

## 会话中的 /plugin

在交互式会话中，`/plugin` 打开 plugin 面板。每个子命令在选项卡上打开面板、在那里运行操作或内联打印结果。`/plugins` 和 `/marketplace` 是 `/plugin` 的别名。

您只能在交互式终端会话中运行这些命令。在非交互式运行（例如 `claude -p`）中，Claude Code 回复 `/plugin` 在此环境中不可用。

有关哪些表面有 `/plugin`、如何在没有它的情况下安装以及每个面板选项卡显示的内容，请参阅 [安装和管理 plugins](https://code.claude.com/docs/zh-CN/plugins/install.md)。

`<plugin>` 是 plugin `name` 或 `name@marketplace`。

下表列出每个会话形式。shell 子命令 `init`、`update`、`details`、`prune`、`eval`、`eval init` 和 `test` 没有会话形式。

| 命令 | 别名 | 它做什么 |
| :- | :- | :- |
| `/plugin` | | 在 **Discover** 选项卡上打开面板。`/plugin` 后的任何无法识别的第一个单词也这样做 |
| `/plugin help` | `/plugin --help`、`/plugin -h` | 显示 `/plugin` 子命令的使用列表 |
| `/plugin list [--enabled\|--disabled]` | `ls` | 内联打印您从市场安装的插件，带有版本、作用域和状态。过滤标志仅显示该状态。启用状态尚未应用的插件标记为 `— run /reload-plugins to apply` |
| `/plugin install` | `i` | 打开 **Discover** 选项卡 |
| `/plugin install <plugin>` | `i` | 在 **Discover** 选项卡中打开 plugin 的详细信息。使用 `name@marketplace`，在该市场的列表中打开它们 |
| `/plugin install <source>` | `i` | 当目标是路径、URL 或 `owner/repo` 时报告 [marketplace not found](https://code.claude.com/docs/zh-CN/plugins/troubleshooting.md#marketplace-not-found) 错误并不安装任何内容，即使是您已经添加的源。要从源安装，请参阅 [在一个命令中添加市场和安装](https://code.claude.com/docs/zh-CN/plugins/install.md#add-a-marketplace-and-install-in-one-command) |
| `/plugin install <plugin> --marketplace <source>` | `i` | 当您尚未添加时添加 `<source>` 处的市场，要求您首先确认，然后打开 plugin 的详细信息。请参阅 [在一个命令中添加市场和安装](https://code.claude.com/docs/zh-CN/plugins/install.md#add-a-marketplace-and-install-in-one-command)。需要 Claude Code v2.1.275 或更高版本 |
| `/plugin manage` | | 打开 **Installed** 选项卡 |
| `/plugin stats` | | 打开 **Stats** 选项卡，在 [`/skill-doctor`](https://code.claude.com/docs/zh-CN/skills#find-unused-skills) 可用的会话中。其他任何地方它在 **Discover** 选项卡上打开面板 |
| `/plugin enable <plugin>` | | 在 plugin 处打开 **Installed** 选项卡并启用它 |
| `/plugin disable <plugin>` | | 在 plugin 处打开 **Installed** 选项卡并禁用它 |
| `/plugin uninstall <plugin>` | | 在 plugin 处打开 **Installed** 选项卡并卸载它 |
| `/plugin configure <plugin>` | `config` | 打开插件的 [`userConfig`](https://code.claude.com/docs/zh-CN/plugins/manifest-reference) 对话框，或报告该插件未声明任何配置 |
| `/plugin validate <path>` | | 打印与 `claude plugin validate` 相同的报告，内联 |
| `/plugin tag [path] [--push] [--dry-run] [--force]` | | 创建发布标签，如 `claude plugin tag` 所做的那样。接受 `--push`、`--dry-run` 和 `--force` 或 `-f`；使用任何其他标志或额外参数，Claude Code 改为打印使用 |
| `/plugin marketplace` | `market` | 不做任何可见的事情。传递 `add`、`list`、`update` 或 `remove` |
| `/plugin marketplace add [source]` | `market add` | 使用源，添加它并报告结果。不使用源，打开 **Add marketplace** 输入 |
| `/plugin marketplace list` | `market list` | 内联打印您的市场名称 |
| `/plugin marketplace update [name]` | `market update` | 打开 **Marketplaces** 选项卡。使用名称，在那里刷新该市场 |
| `/plugin marketplace remove [name]` | `market remove`、`market rm`、`marketplace rm` | 打开 **Marketplaces** 选项卡。使用名称，在那里删除该市场 |

如果您在 `/plugin enable`、`disable`、`uninstall` 或 `configure` 中命名当前项目中未安装的 plugin，Claude Code 打印 `Plugin "<plugin>" is not installed in this project` 而不是操作。

## /reload-plugins

应用待处理的插件更改到正在运行的会话中，无需重新启动。待处理的更改是指自会话启动以来在磁盘上安装、更新、启用、禁用或编辑的插件。

当你关闭 `/plugin` 面板时，如果你在其中进行了待处理的更改，Claude Code 会为你运行 `/reload-plugins`。在面板外发生的插件更改（例如你在另一个终端中运行的 `claude plugin` 命令）之后，请自己运行它。

```text theme={null}
/reload-plugins [--force]
```

| 标志 | 描述 |
| :- | :- |
| `--force` | 应用重新加载，即使它会使 prompt 缓存失效。不带破折号的 `force` 也可以 |

### 重新加载摘要

Claude Code 重新加载每个活跃的插件并打印一行摘要，`Reloaded: N plugins · N skills · N agents · N hooks · N plugin MCP servers · N plugin LSP servers`，在没有交互式终端的会话中省略插件 MCP 服务器计数。当任何插件失败时，摘要会添加 `N errors during load. Run /plugin for details.`

技能计数涵盖插件提供的每个技能，包括其 `commands/` 条目和其 SKILL.md 技能。代理计数是会话中加载的代理数量，包括不来自插件的代理。

当重新加载的插件的[依赖项](https://code.claude.com/docs/zh-CN/plugins/dependencies.md)缺失时，Claude Code 会安装它们，再次重新加载，并在摘要中附加 `(+ N dependencies: <names>) resolved`。

### 更改 MCP 工具的重新加载

当重新加载会添加或删除插件 MCP 服务器或 `LSP` 工具时，该更改会使[prompt 缓存](https://code.claude.com/docs/zh-CN/prompt-caching.md#enabling-or-disabling-a-plugin)失效，Claude Code 不会应用重新加载。它会打印一行，例如 `This reload changes MCP tools (<server>) — your next message will re-read the whole conversation instead of using the cache. Run /reload-plugins --force to apply.` 传递 `--force` 以应用它。

### 没有交互式终端的会话

`/reload-plugins` 也在没有交互式终端的会话中运行，例如桌面应用、Agent SDK 和带有 `-p` 的[非交互模式](https://code.claude.com/docs/zh-CN/headless.md)。需要 Claude Code v2.1.260 或更高版本。

在这些会话中，该命令仅在你自己将其键入会话时运行，例如在 `-p` 提示或桌面应用的提示框中。当它以其他方式到达时，例如通过[远程控制](https://code.claude.com/docs/zh-CN/remote-control.md)或从 Slack 中继的消息，该命令回复 `/reload-plugins isn't available over a remote connection in this session.` 并且不重新加载任何内容。

这些会话中的重新加载不连接或断开插件 MCP 服务器。这些更改在你的下一个会话中生效。

## 为一个会话加载 plugin 的标志

两个 `claude` 标志仅为一个会话加载 plugin，而不安装它。两者都是可重复的。

Plugin 作者使用它们在发布前测试 plugin。对于加载-编辑-重新加载工作流，请参阅 [在没有市场的情况下开发](https://code.claude.com/docs/zh-CN/plugins/create.md#develop-without-a-marketplace)。

| 标志 | 描述 | 示例 |
| :- | :- | :- |
| `--plugin-dir <path>` | 从目录或其 `.zip` 存档加载 plugin。plugins 的文件夹加载每个包含 `.claude-plugin/plugin.json` 的子文件夹。每个标志接受一个路径 | `claude --plugin-dir ./my-plugin --plugin-dir ./other.zip` |
| `--plugin-url <url>` | 从 URL 获取 plugin `.zip` 存档。重复标志，或在一个引用值中传递多个 URL 空格分隔 | `claude --plugin-url "https://example.com/a.zip https://example.com/b.zip"` |

任一标志加载的 plugin 是会话内 plugin。[`claude plugin list`](#plugin-list) 将其显示为 `<name>@inline`，作用域为 `session`，但仅当相同的标志在子命令前时，例如 `claude --plugin-dir ./my-plugin plugin list`。该 plugin 在以 `Session-only plugins` 开头的标题下显示为 `<name>@inline`，`--json` 将其 `scope` 报告为 `session`。

当会话内 plugin 与已安装的 plugin 共享名称时，Claude Code 为该会话加载会话内副本并跳过已安装的副本。如果您使用 `claude plugin disable <name>@inline` 禁用了会话内副本，或托管设置锁定该 plugin 名称，已安装的副本改为加载。有关优先级，请参阅 [Plugin 加载参考](https://code.claude.com/docs/zh-CN/plugins/loading.md)。

管理员可以拒绝两个标志和 [`CLAUDE_CODE_PLUGIN_DIRS`](https://code.claude.com/docs/zh-CN/env-vars#variables) 变量中命名的文件夹，使用托管 [`disableSideloadFlags`](https://code.claude.com/docs/zh-CN/settings-reference#disablesideloadflags) 设置。Claude Code 然后打印标志被您组织的托管设置禁用，并退出 `1` 而不启动。

从 Agent SDK，[`plugins`](https://code.claude.com/docs/zh-CN/agent-sdk/plugins) 选项等同于 `--plugin-dir`。

## 后续步骤

* [安装和管理 plugins](https://code.claude.com/docs/zh-CN/plugins/install.md)：与步骤相同的操作，带有您在每个步骤看到的内容
* [Plugin 加载参考](https://code.claude.com/docs/zh-CN/plugins/loading.md)：每个命令在磁盘上更改的内容以及哪个作用域生效
* [Plugin 故障排除](https://code.claude.com/docs/zh-CN/plugins/troubleshooting.md)：安装、市场、加载和验证错误消息及其修复
* [Plugin 清单参考](https://code.claude.com/docs/zh-CN/plugins/manifest-reference.md)：`claude plugin validate` 检查的字段
