> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# 创建 Claude Code 插件

> 从空目录构建您的第一个 Claude Code 插件，在没有市场的情况下测试它，并转换现有的 .claude/ 设置。

插件是一个包含技能、代理、hooks 和 MCP 服务器的目录，加上一个名为 `plugin.json` 的文件（称为清单），用于命名插件。Claude Code 将该目录作为一个单元加载，因此您可以与团队成员共享它、在多个项目中安装它，或将其发布到市场。

本页面适用于编写自己插件的人员。

> [!NOTE]
>
> 其他页面涵盖了这些情况：
>
> * **安装他人的插件**：请参阅[安装插件](https://code.claude.com/docs/zh-CN/plugins/install.md)
> * **不确定是否需要插件**：请参阅概述中的[决定是否需要插件](https://code.claude.com/docs/zh-CN/plugins/overview.md#decide-whether-you-need-a-plugin)
> * **您的插件用户在 claude.ai 或 Cowork 中**：同一文件夹在那里安装，但组件子集不同。请参阅[插件结构和测试](https://claude.com/docs/plugins/build)和[组件支持表](https://claude.com/docs/plugins/platform-support#compare-component-support-by-app)

从与您已有内容相匹配的部分开始：

* **还没有任何内容**：按照[创建您的第一个插件](#create-your-first-plugin)，然后[在没有市场的情况下开发](#develop-without-a-marketplace)和[测试和调试](#test-and-debug)。
* **`.claude/` 下已有文件**：完成一次第一个插件演练以了解布局，然后按照[转换现有的 `.claude/` 设置](#convert-an-existing-claude-setup)。

## 决定何时使用插件

技能、代理、hooks 和 MCP 服务器都可以在您的项目或主目录中独立工作。当它只为一个项目或仅为您服务时，保持该独立设置。当您想与团队成员共享设置、在多个项目中安装它或发布版本化发布时，创建一个插件。

当您将独立的技能、代理、hooks 和 MCP 配置移动到插件中时，它们的位置和名称会改变：

* **文件的位置**：在插件自己的目录（称为插件根目录）下，作为 `skills/`、`agents/`、`hooks/hooks.json` 和 `.mcp.json`。
* **它们的命名方式**：插件技能和代理获得插件名称作为前缀，例如 `/my-plugin:hello`，因此两个插件可以各自提供一个 `hello` 技能而不会冲突。

要将现有设置移动到插件中，请参阅[转换现有的 `.claude/` 设置](#convert-an-existing-claude-setup)。

## 创建您的第一个插件

在本演练中，您创建一个插件，其唯一组件是一个技能（问候），并使用 `--plugin-dir` 运行它，该选项为一个会话加载插件而不安装它。插件可以包含任何[组件](https://code.claude.com/docs/zh-CN/plugins/components.md)的混合，例如技能、代理、hooks 和 MCP 服务器，没有任何是必需的；一个技能是显示布局的最小示例。

您需要 Claude Code [已安装并登录](https://code.claude.com/docs/zh-CN/quickstart.md#step-1-install-claude-code)。

在您想保留插件的目录（例如 `~/projects`）中打开终端，并从中运行这些步骤中的命令。您可以将插件保留在任何地方，因为您在启动会话时将其路径传递给 Claude Code。

**1. 创建插件目录**

创建插件目录，其中包含一个 `.claude-plugin/` 文件夹来保存清单：

```bash theme={null}
mkdir -p my-first-plugin/.claude-plugin
```

**2. 编写清单**

[清单](https://code.claude.com/docs/zh-CN/plugins/manifest-reference.md)是一个名为 `plugin.json` 的 JSON 文件，它告诉 Claude Code 插件的名称并描述它。将此清单保存为 `my-first-plugin/.claude-plugin/plugin.json`：

```json my-first-plugin/.claude-plugin/plugin.json theme={null}
{
  "name": "my-first-plugin",
  "description": "A greeting plugin to learn the basics",
  "version": "1.0.0",
  "author": {
    "name": "Your Name"
  }
}
```

这四个字段的作用如下：

* **`name`**：必需。它标识插件并成为插件提供的每个技能和代理的前缀。不要在其中放置空格。
* **`description`**：用户在 `/plugin` 中看到的插件文本。
* **`version`**：可选。设置它可以让用户保持该版本，直到您更改它；[发布新版本](https://code.claude.com/docs/zh-CN/plugins/host-marketplace.md#release-a-new-version)说明何时设置或省略它。
* **`author`**：要归功的人。其中 `name` 是必需的；`email` 和 `url` 是可选的。

每个其他字段都在[清单参考](https://code.claude.com/docs/zh-CN/plugins/manifest-reference.md#fields)上。

只有 `plugin.json` 放在 `.claude-plugin/` 内。您接下来添加的技能直接放在 `my-first-plugin/` 下，在该文件夹旁边。

**3. 添加技能**

此插件的一个组件是一个技能。每个技能是 `skills/` 下的一个目录，包含一个 `SKILL.md` 文件。创建技能的目录：

```bash theme={null}
mkdir -p my-first-plugin/skills/hello
```

然后使用以下内容创建 `my-first-plugin/skills/hello/SKILL.md`：

```markdown my-first-plugin/skills/hello/SKILL.md theme={null}
---
name: hello
description: Greet the user with a friendly message
disable-model-invocation: true
---

Greet the user warmly and ask how you can help them today.
```

`disable-model-invocation: true` 行意味着 Claude 不会自行运行该 skill。从您希望 Claude 自行运行的 skill 中删除该行。skill 的命令结合了插件名称和 skill 的名称，因此您将此 skill 作为 `/my-first-plugin:hello` 运行。对于其他 frontmatter 字段，请参阅[skill frontmatter 参考](https://code.claude.com/docs/zh-CN/skills.md#frontmatter-reference)。

**4. 验证插件**

在运行任何内容之前检查清单和 skill 的 frontmatter：

```bash theme={null}
claude plugin validate ./my-first-plugin
```

该命令打印它检查的清单路径和 `✔ Validation passed`。如果它打印 `✘ Validation failed`，则上面该结果行的每一行都命名要修复的字段。在[`claude plugin validate` 报告错误](https://code.claude.com/docs/zh-CN/plugins/troubleshooting#claude-plugin-validate-reports-errors)下查找每条消息。

**5. 使用插件运行 Claude Code**

启动加载了插件的会话：

```bash theme={null}
claude --plugin-dir ./my-first-plugin
```

Claude Code 启动后，运行该技能：

```text theme={null}
/my-first-plugin:hello
```

Claude 用问候语回复。

插件仅在您使用 `--plugin-dir` 启动的会话中加载。要继续处理它而不使用该标志，或测试 `.zip` 构建，请参阅[在没有市场的情况下开发](#develop-without-a-marketplace)。

要让 Claude 为您搭建和检查更大的插件，请从 `claude-plugins-official` 市场[安装](https://code.claude.com/docs/zh-CN/plugins/install.md#install-a-plugin) Anthropic 的 `plugin-dev` 插件，它添加了用于编写技能、hooks 和 MCP 服务器等组件的技能和代理，以及用于验证完成的插件的技能和代理。安装后，运行 `/plugin-dev:create-plugin` 后跟您想要的插件的描述，Claude 将引导您完成设计、创建和验证。

### 共享您的插件

使用[创建您的第一个插件](#create-your-first-plugin)构建的插件仅存在于您的机器上。当它准备好供其他人使用时，有三种方式可以将其提供给他们：

* **直接发送给少数人**：给他们插件的目录或其 `.zip`，无需发布任何内容。请参阅[在没有市场的情况下共享插件](https://code.claude.com/docs/zh-CN/plugins/publish.md#share-a-plugin-without-a-marketplace)。
* **在您自己的市场中列出它**：团队成员添加您的市场一次并按名称安装插件，他们会收到您的更新。请参阅[通过您自己的市场发布](https://code.claude.com/docs/zh-CN/plugins/publish.md#publish-through-your-own-marketplace)。
* **提交到 Anthropic 的目录**：通过审查后，人们可以在 claude.ai 和 Cowork 中添加它，它通过他们的账户到达 Claude Code。请参阅[提交到 Anthropic 的目录](https://code.claude.com/docs/zh-CN/plugins/publish.md#submit-to-anthropics-directory)。

### 插件布局

每种[组件](https://code.claude.com/docs/zh-CN/plugins/components.md)（例如技能、代理、hooks 和 MCP 服务器）都在插件根目录下的固定目录中，插件根目录是您传递给 `--plugin-dir` 的目录。仅添加您使用的目录。要点击完整的插件目录并阅读每个文件的作用，请打开[插件浏览器](https://code.claude.com/docs/zh-CN/plugins/components.md#explore-the-plugin-directory)。

该表列出了大多数插件开始使用的目录，[完整布局](https://code.claude.com/docs/zh-CN/plugins/manifest-reference.md#standard-layout)列出了其余的。

| 位置 | 内容 |
| :- | :- |
| `.claude-plugin/plugin.json` | 清单。当您使用 `--plugin-dir` 加载插件且它没有清单时，Claude Code 会以其目录命名插件 |
| `skills/` | 每个技能一个 `<name>/SKILL.md` 目录 |
| `commands/` | 平面 Markdown 文件，技能的较旧形式。对于新插件，使用 `skills/` |
| `agents/` | 每个子代理一个 Markdown 文件 |
| `hooks/hooks.json` | Hook 配置：一个顶级 `"hooks"` 键，其值的形状与设置文件中的 `hooks` 相同 |
| `.mcp.json` | MCP 服务器定义 |

> [!WARNING]
>
> 只有 `plugin.json` 放在 `.claude-plugin/` 内。保存在那里的组件不会加载。
>
> 插件根目录是插件自己的目录，不是 `~/.claude/` 本身。保存在 `~/.claude/.mcp.json` 的 `.mcp.json` 不会加载。

## 在没有市场的情况下开发

您不需要[市场](https://code.claude.com/docs/zh-CN/plugins/overview.md#get-plugins-from-a-marketplace)来运行您正在编写的插件。改为直接从磁盘或 URL 加载它：

* [`--plugin-dir`](#load-a-directory-or-archive-for-one-session)：为一个会话加载目录或 `.zip` 存档。
* [`--plugin-url`](#fetch-an-archive-from-a-url-for-one-session)：为一个会话从 URL 获取 `.zip` 存档。
* [`claude plugin init`](#scaffold-a-plugin-that-loads-every-session)：在 `~/.claude/skills/` 下搭建一个可跨会话加载的插件。

如果以不同方式加载的两个插件共享一个名称，请参阅[名称冲突](https://code.claude.com/docs/zh-CN/plugins/loading.md#name-conflicts)以了解 Claude Code 保留哪一个。

### 为一个会话加载插件

您可以通过三种方式为单个会话加载插件：使用 `--plugin-dir` 从磁盘上的目录或 `.zip` 存档，使用 `--plugin-url` 从 URL，或从环境变量（当您无法添加标志时）。每个插件仅为该会话加载，不会为其写入任何内容到您的设置中。当您在会话期间编辑插件的文件时，运行 `/reload-plugins` 以加载更改。

#### 从目录或 `.zip` 加载插件

当您从 shell 启动 `claude` 时，使用插件的根目录或其 `.zip` 存档传递 `--plugin-dir`。重复该标志以加载多个插件：

```bash theme={null}
claude --plugin-dir ./my-first-plugin --plugin-dir ./other-plugin.zip
```

#### 加载插件文件夹

要从一个地方加载多个插件，请传递一个包含它们的文件夹，例如 `--plugin-dir ./plugins`。加载插件文件夹需要 Claude Code v2.1.265 或更高版本。

如果文件夹没有 `.claude-plugin/` 目录且其顶级没有插件组件，Claude Code 会将其视为插件文件夹。然后，每个具有 `.claude-plugin/plugin.json` 清单的直接子文件夹都作为单独的插件加载。文件夹中的所有其他内容都被跳过而不出错，包括没有清单的子文件夹。如果文件夹中的插件不加载，请检查其子文件夹是否具有 `.claude-plugin/plugin.json`。

您还可以传递一个在其插件文件夹旁边保留 `.claude-plugin/marketplace.json` 的文件夹。只要该 `.claude-plugin/` 目录不包含 `plugin.json`，插件文件夹仍然会加载。不会从市场文件安装或启用任何内容，因为 Claude Code 不读取它。从这样的文件夹加载插件需要 Claude Code v2.1.281 或更高版本。

在交互式会话中，您还可以在启动后在文件夹中添加和删除插件：

* 您添加的子文件夹一旦其清单存在就作为新插件加载。
* 当您删除子文件夹时，其插件卸载。

会话中会为这些更改中的每一个显示一条消息。如果在对话中间加载或卸载插件会[使提示缓存失效](https://code.claude.com/docs/zh-CN/prompt-caching.md#enabling-or-disabling-a-plugin)，则更改会被保留，消息会告诉您运行 `/reload-plugins` 以应用它。

#### 从 URL 加载插件

当您从 shell 启动 `claude` 时，使用 `.zip` 存档的地址传递 `--plugin-url`，例如您的 CI 发布的构建工件：

```bash theme={null}
claude --plugin-url https://example.com/my-first-plugin.zip
```

Claude Code 在启动时下载存档。要加载多个，重复该标志或在一个带引号的参数中传递以空格分隔的 URL。

仅将该标志指向您控制或信任的存档。

如果 Claude Code 无法获取存档或存档无效，它会在没有插件的情况下启动，并记录一个插件加载错误，您可以在 `/plugin` 管理器的**错误**选项卡中查看。

#### 从环境变量加载插件

要在无法添加 `--plugin-dir` 标志的会话中加载插件，请在 [`CLAUDE_CODE_PLUGIN_DIRS`](https://code.claude.com/docs/zh-CN/env-vars#variables) 环境变量中列出它们的绝对路径。Claude Code 将每个路径作为 `--plugin-dir` 路径加载。这些插件除了您使用 `--plugin-dir` 传递的任何插件外还会加载。[项目和本地设置无法设置此变量](https://code.claude.com/docs/zh-CN/settings-reference.md#variables-claude-code-ignores-in-env)。`CLAUDE_CODE_PLUGIN_DIRS` 需要 Claude Code v2.1.280 或更高版本。

托管设置可以关闭 `--plugin-dir` 和 `CLAUDE_CODE_PLUGIN_DIRS`。请参阅[为一个会话加载插件的标志](https://code.claude.com/docs/zh-CN/plugins/cli-reference.md#flags-that-load-a-plugin-for-one-session)。要测试插件及其依赖项，请参阅[在本地测试插件及其依赖项](https://code.claude.com/docs/zh-CN/plugins/dependencies.md#test-a-plugin-and-its-dependency-locally)。

### 使插件在每个会话中加载

您的个人 skill 目录是 `~/.claude/skills/`，其中包含 `.claude-plugin/plugin.json` 的文件夹即为 skill 目录插件。除非您的管理员已[阻止 skill 目录插件](https://code.claude.com/docs/zh-CN/plugins/loading.md#skills-directory-plugins-blocked-by-managed-settings)，否则 Claude Code 会在会话启动时加载每个此类插件，无需标志，也无需安装步骤。`claude plugin init` 可为您搭建一个此类插件。

#### 使用 `claude plugin init` 搭建插件

`claude plugin init` 在 `~/.claude/skills/` 下写入一个启动插件。从您的 shell 搭建一个：

```bash theme={null}
claude plugin init my-tool
```

该命令创建 `~/.claude/skills/my-tool/`，其中包含 `.claude-plugin/plugin.json` 和根 `SKILL.md`。它打印 `✔ Created plugin "my-tool" at ~/.claude/skills/my-tool`，然后是 `It will auto-load next session as my-tool@skills-dir. Run /reload-plugins to load it now.`

传递 `--with skills` 以让 `claude plugin init` 为您在 `skills/` 下搭建一个技能。其他 `--with` 值在[插件命令参考](https://code.claude.com/docs/zh-CN/plugins/cli-reference.md#plugin-init)上。

#### 命名插件的技能

`~/.claude/skills/my-tool/SKILL.md` 处的根技能也是个人技能，因此您将其作为 `/my-tool` 而不是 `/my-tool:my-tool` 调用。您在插件内 `skills/` 下添加的技能获得插件名称前缀，例如 `/my-tool:example`。

#### 停止加载插件

要停止加载搭建的插件，删除其目录，或在 shell 中使用 `claude plugin init` 打印的 `my-tool@skills-dir` 名称运行 `claude plugin disable my-tool@skills-dir`。在 ID `my-tool@skills-dir` 中，`skills-dir` 代替市场名称，因为插件从您的技能目录而不是从市场加载。

#### 通过存储库共享插件

`claude plugin init` 将插件写入您的个人 skill 目录 `~/.claude/skills/`，本机上的所有项目都共享该目录。要使插件为一个仓库中的每个人加载，请在 `<project>/.claude/skills/<name>/` 处自己创建相同的布局，包括其 `.claude-plugin/plugin.json`。请参阅[通过仓库共享的插件](https://code.claude.com/docs/zh-CN/plugins/loading.md#plugins-shared-through-a-repository)以了解 Claude Code 加载它的条件。

## 测试和调试

当对插件的更改没有显示时，按顺序完成这些检查。每一个都告诉您 Claude Code 对插件做了什么：

1. 在您的 shell 中，运行 `claude plugin validate <path>`。它检查清单和每个技能、代理和命令文件的 frontmatter，并在 `Validation passed` 时退出 `0`。添加 `--strict` 也会在警告时失败。退出代码和目录处理在[插件命令参考](https://code.claude.com/docs/zh-CN/plugins/cli-reference.md#plugin-validate)上。
2. 在运行的会话中，运行 `/reload-plugins` 以应用您在磁盘上所做的编辑。它打印一个 `Reloaded:` 行，其中包含计数。然后通过键入其 `/plugin-name:skill` 命令或在 `/plugin` **已安装**选项卡中找到插件来确认技能已加载。
3. 在同一会话中，运行 `/plugin`。**已安装**选项卡列出您的插件，在插件的详细信息中，Claude Code 找到的组件。**错误**选项卡列出了什么未能加载以及原因，例如清单中不存在的路径。
4. 回到您的 shell，运行 `claude plugin list`。它在各自的部分中打印仅会话和技能目录插件，带有 `Status: ✔ loaded` 或加载错误。要包括您正在开发的插件，请在 `plugin list` 之前使用其路径传递 `--plugin-dir`。
5. 如果您的插件位于 skills 目录中，而 `claude plugin list` 没有打印它，且该处和 `/plugin` 中都没有错误，请参阅[被托管设置阻止的 skills 目录插件](https://code.claude.com/docs/zh-CN/plugins/loading.md#skills-directory-plugins-blocked-by-managed-settings)。

要检查 MCP 服务器，请在会话中运行 `/mcp` 以查看服务器的状态。当服务器健康时，`/mcp` 将其列为已连接。如果不是，请参阅[不启动的 MCP 服务器](https://code.claude.com/docs/zh-CN/plugins/troubleshooting.md#invalid-mcp-server-config-for-and-mcp-servers-that-dont-start)。

要检查 hook，触发它匹配的事件。例如，要求 Claude 编辑文件以触发 `PostToolUse` hook。然后阅读[调试日志](https://code.claude.com/docs/zh-CN/hooks.md#debug-hooks)，它显示哪些 hooks 匹配、它们的退出代码和它们的输出。

下一部分涵盖您在开发时最可能遇到的失败，[故障排除页面](https://code.claude.com/docs/zh-CN/plugins/troubleshooting.md#build-a-plugin)对每一个都有完整的条目。

### 找不到组件路径

`/plugin` 的**错误**选项卡显示 `<component> path not found: <path>`，例如 `commands path not found`。清单中的组件路径（例如 `commands`、`skills`、`agents` 或 `hooks`）指向不存在的内容。修复路径或创建目录，然后在会话中运行 `/reload-plugins`。请参阅[`commands path not found`](https://code.claude.com/docs/zh-CN/plugins/troubleshooting#commands-path-not-found)。

### `--plugin-dir` 在市场根目录不加载 `plugins/` 下的插件

`--plugin-dir` 采用插件的根目录，即包含 `.claude-plugin/plugin.json` 和组件目录（如 `skills/`）的目录。如果您改为将其指向市场根目录，Claude Code 不会读取 `marketplace.json`，因此 `plugins/` 下的插件不会加载，您看不到错误。将标志指向一个插件的文件夹，或添加市场。请参阅[故障排除条目](https://code.claude.com/docs/zh-CN/plugins/troubleshooting.md#plugin-dir-loads-a-plugin-with-no-components)。

### 插件加载但其技能缺失

`skills/` 目录在 `.claude-plugin/` 内，或清单中的 `skills` 条目指向一个文件。将 `skills/` 移动到插件根目录，将每个 `skills` 条目指向包含 `SKILL.md` 的目录，并在会话中运行 `/reload-plugins`。请参阅[插件加载但其技能缺失](https://code.claude.com/docs/zh-CN/plugins/troubleshooting.md#plugin-loads-but-its-skills-are-missing)。

### `userConfig` 对话框从不出现

您的插件的 [`userConfig`](https://code.claude.com/docs/zh-CN/plugins/components#user-configuration) 选项的对话框是通过会话中的 `/plugin` 安装的一部分。使用 `--plugin-dir` 加载不会显示它，`claude plugin install` 在 shell 中也不会。加载插件后，在会话中运行 `/plugin configure <plugin-name>` 以打开它。请参阅[`userConfig` 对话框从不出现](https://code.claude.com/docs/zh-CN/plugins/troubleshooting#the-userconfig-dialog-never-appears)。

### 检查插件是否改变了 Claude 的行为

加载时没有错误的插件仍然可能无法按您的意图引导 Claude。`claude plugin eval`（您在 shell 中运行）使用和不使用插件运行您的测试用例，并对差异进行评分。请参阅[使用 evals 测试插件](https://code.claude.com/docs/zh-CN/plugin-evals.md)，从[创建您的第一个 eval 套件](https://code.claude.com/docs/zh-CN/plugin-evals.md#create-your-first-eval-suite)开始。

## 转换现有的 `.claude/` 设置

如果您已经在项目的 `.claude/` 目录下有技能、代理或 hooks，您可以将它们移动到插件中而无需重写它们。

从项目根目录（包含 `.claude/` 的目录）运行这些步骤中的命令，因为 `cp` 路径相对于它。

**1. 创建插件结构**

在 `.claude/` 旁边创建插件目录及其 `.claude-plugin/` 文件夹。您之后可以将插件移动到任何地方。

```bash theme={null}
mkdir -p my-plugin/.claude-plugin
```

创建 `my-plugin/.claude-plugin/plugin.json`：

```json my-plugin/.claude-plugin/plugin.json theme={null}
{
  "name": "my-plugin",
  "description": "Migrated from standalone configuration",
  "version": "1.0.0"
}
```

**2. 复制您现有的文件**

将您拥有的每个配置目录复制到插件根目录，并跳过您没有的任何目录的命令。

```bash theme={null}
cp -r .claude/commands my-plugin/
```

```bash theme={null}
cp -r .claude/agents my-plugin/
```

```bash theme={null}
cp -r .claude/skills my-plugin/
```

运行 `ls -a my-plugin` 以确认您复制的每个目录都出现在 `.claude-plugin` 旁边。

**3. 移动您的 hooks**

如果您在 `.claude/settings.json` 或 `.claude/settings.local.json` 中有 hooks，请创建一个 hooks 目录：

```bash theme={null}
mkdir -p my-plugin/hooks
```

创建 `my-plugin/hooks/hooks.json` 并将您的设置文件中的 `hooks` 对象复制到其中。格式相同。

此示例显示了形状，其中一个 hook 在 Claude 写入或编辑每个文件时运行 linter。用您自己的 `hooks` 对象替换示例。

```json my-plugin/hooks/hooks.json theme={null}
{
  "hooks": {
    "PostToolUse": [
      {
        "matcher": "Write|Edit",
        "hooks": [{ "type": "command", "command": "jq -r '.tool_input.file_path' | xargs npm run lint:fix" }]
      }
    ]
  }
}
```

**4. 测试迁移的插件**

为一个会话加载插件：

```bash theme={null}
claude --plugin-dir ./my-plugin
```

在其新名称下检查每个组件：

* **技能**：对于曾经是 `/deploy` 的技能，运行 `/my-plugin:deploy`。
* **子代理**：要求 Claude 为曾经是 `reviewer` 的代理使用 `my-plugin:reviewer` 代理。
* **Hooks**：触发每个 hook 匹配的事件。

如果缺少什么，请完成[测试和调试](#test-and-debug)。

虽然原始文件仍在 `.claude/` 下，但它们与插件的副本一起保持加载：

* **技能和代理**：这两个集合不会冲突，因为插件的技能和代理带有 `my-plugin:` 前缀。`/deploy` 和 `/my-plugin:deploy` 都有效，Claude 将 `reviewer` 和 `my-plugin:reviewer` 视为两个子代理。
* **Hooks**：hooks 没有前缀，因此同时在您的设置文件和 `hooks/hooks.json` 中的 hook 在其事件每次触发时运行两次。

在您确认插件有效后，从 `.claude/` 中删除原始文件，并从您的设置文件中删除 `hooks` 对象。

## 后续步骤

* [插件组件](https://code.claude.com/docs/zh-CN/plugins/components.md)：向您的插件添加代理、hooks、MCP 服务器、LSP 服务器和用户配置
* [使用 evals 测试插件](https://code.claude.com/docs/zh-CN/plugin-evals.md)：编写 eval 用例并使用 `claude plugin eval` 运行它们以检查插件引导 Claude 行为的可靠性
* [发布插件](https://code.claude.com/docs/zh-CN/plugins/publish.md)：对其进行版本控制，将其放在市场中，并提交以供审查
* [插件结构和测试](https://claude.com/docs/plugins/build)：同一插件文件夹在 claude.ai 和 Cowork 中安装。某些组件仅限 Claude Code，[组件支持表](https://claude.com/docs/plugins/platform-support#compare-component-support-by-app)列出了在每个平台上加载的组件
* [插件清单参考](https://code.claude.com/docs/zh-CN/plugins/manifest-reference.md)：每个 `plugin.json` 字段、路径规则和目录
* [技能](https://code.claude.com/docs/zh-CN/skills.md)：编写您的插件提供的技能
* [Anthropic 在 claude-plugins-official 仓库中的插件](https://github.com/anthropics/claude-plugins-official/tree/main/plugins)：本页面布局的完整工作示例，例如 `feature-dev` 和 `code-review`
