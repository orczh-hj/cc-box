> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# 为您的组织推荐插件

> 向 marketplace 插件条目添加相关性块，以便当用户的工作匹配时 Claude Code 会建议安装这些插件，并在托管设置中将 marketplace 列入允许列表。

Claude Code 可以在用户的会话与您为该插件定义的信号匹配时，建议从您组织的 marketplace 安装插件。信号包括工作目录、Claude 已读取的文件以及 Claude 已运行的命令。您可以通过向 `marketplace.json` 中的插件条目添加 `relevance` 块来定义这些信号。

市场运营商编写 `relevance` 条目。然后管理员在托管设置中将市场列入允许列表。在终端中，市场被列入允许列表之前，用户看不到来自该市场的任何建议。

> [!NOTE]
>
> 这些情况在其他页面上有介绍：
>
> * **您想安装插件**：请参阅[安装和管理插件](https://code.claude.com/docs/zh-CN/plugins/install.md)
> * **您想关闭建议**：请参阅[了解插件相关性的工作原理](#understand-how-plugin-relevance-works)；对于 Claude 在云端会话的对话中提出的建议，请参阅[关闭插件建议](https://code.claude.com/docs/zh-CN/claude-code-on-the-web.md#turn-off-plugin-suggestions)

从适合您角色的部分开始：

* **Marketplace 运营商**：阅读[建议如何工作](#understand-how-plugin-relevance-works)，然后[向插件条目添加相关性](#add-relevance-to-a-plugin-entry)和[验证您的 marketplace](#validate-your-marketplace)
* **管理员**：[在托管设置中启用建议](#enable-suggestions-in-managed-settings)

## 了解插件相关性的工作原理

`marketplace.json` 中的每个插件条目都可以包含一个 `relevance` 对象。该对象命名一个主题和一个或多个信号。信号是 Claude Code 针对当前会话测试的模式，例如工作目录或 Claude 已读取的文件。

信号匹配在用户的机器上本地进行，不会增加网络流量。Claude Code 不会向 Anthropic 或 marketplace 运营者报告哪些信号匹配或其值。

当信号匹配且插件尚未安装时，Claude Code 在以下位置建议该插件：

* **Spinner 提示**：当 Claude 正在响应时，包含 `/plugin install` 命令的消息出现在 spinner 下方。
* **会话启动通知**：如果 `cwd` 信号与工作目录匹配，在用户发送第一条消息之前会出现一行通知。
* **`/plugin` Discover 标签页**：该插件被固定到 Discover 列表的顶部。

[预览用户看到的内容](#preview-what-the-user-sees) 显示每个的确切文本以及它们重复的频率。

Claude Code 永远不会自动安装插件。用户始终需要确认。

当用户或项目将 [`spinnerTipsEnabled`](https://code.claude.com/docs/zh-CN/settings-reference#spinnertipsenabled) 设置为 `false`，或当 [`spinnerTipsOverride`](https://code.claude.com/docs/zh-CN/settings-reference#spinnertipsoverride) 带有 `excludeDefault` 替换内置提示时，spinner 提示和会话启动通知都会停止出现。Discover 标签页的固定不受这两个设置的影响。

## 向插件条目添加相关性

向您的 `marketplace.json` 中的插件条目添加 `relevance` 对象。以下示例声明当 Claude 读取 `.tf` 文件或运行 `terraform` 时，`terraform-helpers` 插件是相关的：

```json theme={null}
{
  "name": "your-marketplace",
  "owner": { "name": "Your Org" },
  "plugins": [
    {
      "name": "terraform-helpers",
      "source": "./plugins/terraform-helpers",
      "description": "Your organization's Terraform conventions and helpers",
      "relevance": {
        "topic": "Terraform",
        "signals": {
          "cli": ["terraform"],
          "filesRead": ["**/*.tf"]
        }
      }
    }
  ]
}
```

当其信号都不匹配时，该插件在 Discover 列表中保持其正常位置，不会显示为 spinner 提示。

要在发布前检查该块，请 [验证您的 marketplace](#validate-your-marketplace)。

## 字段参考

`relevance` 对象及其嵌套的 `signals` 对象接受以下表格中的字段。

较旧的客户端仍然可以加载使用它们不识别的 `relevance` 字段的 marketplace，因为在加载时会忽略 `relevance` 和 `relevance.signals` 下的未知字段。一个已识别的字段，其值超过 [字段参考](#field-reference) 中的限制，会使整个插件条目失效，用户无法从 marketplace 安装该插件，直到您修复它；`claude plugin validate` 报告相同的限制。

### `relevance`

| 字段 | 类型 | 描述 |
| :- | :- | :- |
| `topic` | 字符串 | 可选。填充 spinner 提示中"使用 *topic*？"的短语。默认为插件名称，每个连字符段首字母大写。最多 64 个字符。 |
| `signals` | 对象 | 确定插件何时相关的匹配器。Claude Code 仅在至少设置一个信号时建议该插件。请参阅 [`relevance.signals`](#relevance-signals)。 |

`topic` 通常是产品名称，例如 `Terraform`。当插件名称作为主题听起来不自然时，使用诸如 `design` 之类的域。

### `relevance.signals`

`signals` 对象接受以下字段。

| 字段 | 类型 | 描述 | 限制 |
| :- | :- | :- | :- |
| `cwd` | 字符串数组 | 与会话工作目录匹配的 Glob 模式。请参阅 [工作目录匹配](#working-directory-matching)。 | 10 个模式，每个 256 个字符 |
| `cli` | 字符串数组 | Claude 在此会话中运行的 shell 命令中的命令名称，例如 `["terraform"]`。精确匹配。请参阅 [命令名称匹配](#command-name-matching)。 | 10 个条目，每个 64 个字符 |
| `hosts` | 字符串数组 | 此会话中 Bash 命令中 `http://` 或 `https://` URL 中看到的主机名，例如 `["registry.terraform.io"]`。仅限裸小写主机名：无方案、端口或路径。精确不区分大小写匹配。 | 20 个条目，每个 128 个字符 |
| `filesRead` | 字符串数组 | 与 Claude 在此会话中已读取的文件路径匹配的 Glob 模式，例如 `["**/*.tf"]`。正斜杠规范化且不区分大小写。 | 10 个模式，每个 256 个字符 |
| `manifestDeps` | 对象数组 | Claude 在此会话中已读取的包清单中声明的依赖项。每个条目是 `{ "file": "...", "pattern": "..." }`，其中两个值都是正则表达式。请参阅 [清单依赖项匹配](#manifest-dependency-matching)。 | 10 个条目，每个值最多 256 个字符。大于 512 KB 的清单文件被跳过 |

`filesRead` 和 `manifestDeps` 信号也与 Claude 在此会话中已写入或编辑的文件以及项目的自动加载的 `CLAUDE.md` 内存文件匹配。

#### 工作目录匹配

`cwd` 是唯一可以在会话启动时匹配的信号，在用户发送第一条消息之前。

Claude Code 按如下方式匹配每个 `cwd` 模式：

* 该模式作为绝对路径与工作目录匹配。当会话在 git 存储库内时，它也与工作目录相对于存储库根目录的路径匹配。
* 匹配是正斜杠规范化且不区分大小写的。
* 每个模式都匹配目录本身及其下的所有内容，因此 `infra`、`infra/` 和 `infra/**` 的行为相同。

#### 命令名称匹配

Claude Code 为 Claude 运行的每个 shell 命令记录一个命令名称：任何前导环境变量赋值和 `sudo` 之后的第一个令牌。复合命令仅贡献其前导命令，因此 `cd infra && terraform plan` 记录 `cd`，而不是 `terraform`。

#### 清单依赖项匹配

每个 `manifestDeps` 条目配对两个 JavaScript `RegExp` 源字符串：

* `file`：不区分大小写地与清单文件的路径匹配。路径通常是绝对的，因此在末尾而不是开头锚定模式。路径对于此信号不是分隔符规范化的，因此 Windows 路径使用反斜杠。
* `pattern`：区分大小写地与该文件的内容匹配。

以下示例使用 `manifestDeps` 在 Claude 读取了依赖于您的 SDK npm 包（此处名为 `your-sdk`）的 `package.json` 后建议您的插件。

```json theme={null}
{
  "name": "your-plugin",
  "source": "./plugins/your-plugin",
  "relevance": {
    "signals": {
      "manifestDeps": [
        {
          "file": "[/\\\\]package\\.json$",
          "pattern": "\"your-sdk\"\\s*:"
        }
      ]
    }
  }
}
```

在此示例中，`file` 模式使用 `[/\\\\]` 以匹配正斜杠和反斜杠路径分隔符，使用 `\\.` 以使点为字面。在 JSON 中，正则表达式中的每个反斜杠都写两次。

## 验证您的 marketplace

在您的 shell 中，针对您的 marketplace 目录运行 `claude plugin validate` 以在发布前检查 `relevance` 块：

```bash theme={null}
claude plugin validate ./my-marketplace
```

验证器报告 `relevance` 块上的错误和警告，包括这些：

* 将 `relevance` 和 `relevance.signals` 下的未知键报告为警告
* 标记不是对象的 `relevance` 值
* 拒绝包含方案、端口或路径的 `signals.hosts` 条目

每个发现都与其关注的字段的路径一起打印，输出以 `Validation passed`、`Validation passed with warnings` 或 `Validation failed` 结尾。

## 在托管设置中启用建议

在终端中，即使市场的 `marketplace.json` 声明了 `relevance`，在管理员于[托管设置](https://code.claude.com/docs/zh-CN/plugins/org.md)中将其加入允许列表之前，用户也看不到来自该市场的任何建议。

要将 marketplace 加入允许列表，请按如下方式编辑您的托管设置：

* 将 marketplace 名称添加到 `pluginSuggestionMarketplaces`。
* 对于除官方 Anthropic marketplace 之外的任何 marketplace，还要声明 marketplace 源，可以是 [`extraKnownMarketplaces`](https://code.claude.com/docs/zh-CN/plugins/org#require-a-marketplace-and-its-plugins) 中该名称的条目，或 [`strictKnownMarketplaces`](https://code.claude.com/docs/zh-CN/plugins/org#allowlist-with-strictknownmarketplaces) 中的条目。

在未注册 marketplace 的机器上，或在从不同源注册的允许列表名称下注册的机器上，来自它的任何建议都不会出现。源检查阻止不相关的源以允许列表名称注册以在您的组织中建议其插件。

以下 `managed-settings.json` 从 GitHub 存储库注册组织 marketplace 并启用其建议：

```json theme={null}
{
  "extraKnownMarketplaces": {
    "your-marketplace": {
      "source": {
        "source": "github",
        "repo": "your-org/your-marketplace"
      }
    }
  },
  "pluginSuggestionMarketplaces": ["your-marketplace"]
}
```

官方 marketplace 的名称只能从官方 Anthropic 源注册，因此不需要源声明。对于官方 marketplace，仅将名称加入允许列表：

```json theme={null}
{
  "pluginSuggestionMarketplaces": ["claude-plugins-official"]
}
```

## 预览用户看到的内容

当插件的 `relevance` 信号在会话期间匹配时，spinner 下方的提示读取：

```text theme={null}
Working with Terraform? Install the terraform-helpers plugin:
/plugin install terraform-helpers@your-marketplace
```

当 `cwd` 信号在会话启动时匹配时，一行通知读取：

```text theme={null}
plugin suggestion: terraform-helpers@your-marketplace · /plugin
```

在 `/plugin` Discover 标签页中，该插件被固定在其他结果上方，带有命名匹配信号的注释，例如 `suggested for this directory` 或 `suggested for terraform commands`。

Claude Code 限制建议给定插件的频率：

* 该建议在 spinner 提示和会话启动通知的组合中最多每三个会话出现一次。
* 一旦 spinner 提示和通知总共显示了该插件两次，会话启动通知就停止出现。
* 一旦安装了插件，spinner 提示和会话启动通知都不会重复。
* Discover 标签页在用户首次打开标签页时固定该插件，同时插件的信号匹配。Claude Code 在 `~/.claude.json` 中记录这一点，因此每次用户稍后在该机器上打开 `/plugin` 时，该插件都以正常顺序出现。

## 另请参阅

* [托管 marketplace](https://code.claude.com/docs/zh-CN/plugins/host-marketplace.md)：运行托管您的插件的 marketplace
* [Marketplace 参考](https://code.claude.com/docs/zh-CN/plugins/marketplace-reference.md#plugin-entries)：插件条目接受的每个字段
* [从您的 CLI 推荐您的插件](https://code.claude.com/docs/zh-CN/plugins/cli-hints.md)：从您自己的 CLI 而不是从 Claude Code 的会话信号提示用户
* [为您的组织管理插件](https://code.claude.com/docs/zh-CN/plugins/org.md)：`extraKnownMarketplaces`、`strictKnownMarketplaces` 和其余的插件策略键
