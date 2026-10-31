> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# 代码智能插件

> 安装语言服务器插件，使 Claude 在编辑后能看到类型错误并通过符号导航代码，并回答 LSP 插件推荐对话框。

代码智能插件为 Claude 提供编辑器具有的实时诊断和转到定义功能，因此 Claude 可以在运行构建之前捕获其自身编辑引入的类型错误和缺失的导入，并通过符号而不是文本搜索来查找定义和引用。

每个插件通过语言服务器协议 (LSP) 将 Claude Code 连接到一种语言的语言服务器。您从 Anthropic 的官方市场安装插件，并在您的机器上安装语言服务器二进制文件。

> [!NOTE]
>
> 代码智能插件在终端会话中工作。在 [云会话](https://code.claude.com/docs/zh-CN/claude-code-on-the-web.md) 中，Claude Code 不启动插件语言服务器，因此 Claude 在那里无法获得诊断或代码导航。要编写自己的语言服务器插件，或连接没有插件的语言服务器，请参阅 [插件组件中的 LSP 服务器](https://code.claude.com/docs/zh-CN/plugins/components.md#lsp-servers)。

要开始使用，请在 [安装代码智能插件](#install-a-code-intelligence-plugin) 下的表格中找到您的语言。该表格中的插件来自 Anthropic 的 [官方插件市场](https://code.claude.com/docs/zh-CN/plugins/anthropic-marketplaces.md)。

如果您已经看到 **LSP 插件推荐** 对话框，请参阅 [接受或关闭推荐对话框](#accept-or-dismiss-the-recommendation-dialog) 了解每个选择的作用。

## 安装代码智能插件

代码智能插件告诉 Claude Code 哪个命令启动语言服务器以及它处理哪些文件扩展名。它不包括语言服务器。首先安装语言服务器二进制文件，然后安装插件，最后确认服务器启动。

**1. 安装语言服务器二进制文件**

在下表中找到您的语言并安装其行中的二进制文件。如果您的语言未列出，请参阅[添加没有官方插件的语言](#add-a-language-without-an-official-plugin)。

| 语言 | 插件 | 二进制文件 |
| :- | :- | :- |
| C/C++ | [`clangd-lsp`](https://github.com/anthropics/claude-plugins-official/tree/main/plugins/clangd-lsp) | `clangd` |
| C# | [`csharp-lsp`](https://github.com/anthropics/claude-plugins-official/tree/main/plugins/csharp-lsp) | `csharp-ls` |
| Go | [`gopls-lsp`](https://github.com/anthropics/claude-plugins-official/tree/main/plugins/gopls-lsp) | `gopls` |
| Java | [`jdtls-lsp`](https://github.com/anthropics/claude-plugins-official/tree/main/plugins/jdtls-lsp) | `jdtls` |
| Kotlin | [`kotlin-lsp`](https://github.com/anthropics/claude-plugins-official/tree/main/plugins/kotlin-lsp) | `kotlin-lsp` |
| Liquid | [`liquid-lsp`](https://github.com/Shopify/liquid-skills/tree/main/plugins/liquid-lsp) | `shopify`，来自 Shopify CLI |
| Lua | [`lua-lsp`](https://github.com/anthropics/claude-plugins-official/tree/main/plugins/lua-lsp) | `lua-language-server` |
| PHP | [`php-lsp`](https://github.com/anthropics/claude-plugins-official/tree/main/plugins/php-lsp) | `intelephense` |
| Python | [`pyright-lsp`](https://github.com/anthropics/claude-plugins-official/tree/main/plugins/pyright-lsp) | `pyright-langserver` |
| Ruby | [`ruby-lsp`](https://github.com/anthropics/claude-plugins-official/tree/main/plugins/ruby-lsp) | `ruby-lsp` |
| Rust | [`rust-analyzer-lsp`](https://github.com/anthropics/claude-plugins-official/tree/main/plugins/rust-analyzer-lsp) | `rust-analyzer` |
| Swift | [`swift-lsp`](https://github.com/anthropics/claude-plugins-official/tree/main/plugins/swift-lsp) | `sourcekit-lsp` |
| TypeScript 和 JavaScript | [`typescript-lsp`](https://github.com/anthropics/claude-plugins-official/tree/main/plugins/typescript-lsp) | `typescript-language-server` |

Anthropic 维护表格中的每个插件，除了 `liquid-lsp`，由 Shopify 维护，官方市场列出。

要找到安装二进制文件的命令，请按照表格中的插件链接进入其 README。对于 TypeScript，该命令是 `npm install -g typescript-language-server typescript`。

安装二进制文件后，确认它在您启动 `claude` 的 shell 的 `PATH` 上，例如使用 `which typescript-language-server`，或在 PowerShell 中使用 `Get-Command typescript-language-server`。

**2. 安装插件**

在 VS Code 扩展或桌面应用中，请按照[安装插件](https://code.claude.com/docs/zh-CN/plugins/install.md#install-a-plugin)操作，而不是执行此步骤。在终端中，运行 `claude` 启动 Claude Code，然后在其输入框中输入以下内容，将 `typescript-lsp` 替换为步骤 1 表格中为您的语言列出的插件：

```
/plugin install typescript-lsp@claude-plugins-official
```

确认消息会说明插件现在是否处于活动状态或需要 `/reload-plugins`。如果安装失败并显示 `Marketplace "claude-plugins-official" not found`，请参阅[该错误的故障排除条目](https://code.claude.com/docs/zh-CN/plugins/troubleshooting.md#marketplace-claude-plugins-official-not-found)。要控制插件的安装位置，或从 shell 而不是在 Claude Code 内运行安装，请参阅[安装插件](https://code.claude.com/docs/zh-CN/plugins/install.md)。

**3. 确认服务器启动**

语言服务器在 Claude 首次编辑具有插件扩展名之一的文件时启动。要查看其工作情况，请要求 Claude 在该语言的文件中引入类型错误，然后修复它。然后检查对话中的诊断行：

* **诊断行出现**：编辑下方的 `Found N new diagnostic issues in M files (ctrl+o to expand)` 表示服务器已启动。
* **没有诊断行出现**：运行 `/plugin` 并打开**错误**选项卡。读取 `Executable not found in $PATH: "<binary>"` 的行命名要安装的二进制文件。如果选项卡中没有这样的行，请参阅[故障排除代码智能](#troubleshoot-code-intelligence)。

安装缺失的二进制文件后，Claude Code 会在 Claude 下次编辑匹配文件时重试。如果您将二进制文件安装到不在您启动 `claude` 的 shell 的 `PATH` 上的目录中，请从 shell 启动新会话，其中它在 `PATH` 上。

## 查看 Claude 获得的功能

运行语言服务器后，Claude 获得诊断和代码导航：

* **编辑后的诊断**：每次 Claude 编辑或写入服务器处理的文件时，Claude 都会获得服务器报告的错误和警告。它会看到它引入的类型错误、缺失导入或语法错误，而无需运行编译器。
* **代码导航**：Claude 获得一个 `LSP` 工具，通过服务器查找符号，而不是搜索文本。该工具是只读的。有关 Claude 可以使用该工具查找的内容以及权限如何应用于它，请参阅 [LSP 工具行为](https://code.claude.com/docs/zh-CN/tools-reference.md#lsp-tool-behavior)。

### 自己阅读诊断

Claude 编辑服务器处理的文件后，对话仅显示 `Found N new diagnostic issues` 摘要。要阅读问题本身，请按 **Ctrl+O**。

## 接受或关闭推荐对话框

如果语言服务器二进制文件已在您的 `PATH` 上，但使用它的插件未安装，Claude Code 会在标题为 **LSP 插件推荐**的对话框中提供为您安装插件。

### 推荐对话框何时出现

**LSP 插件推荐**对话框可以在 Claude 编辑文件后出现。这些条件决定它是否出现以及它提供哪个插件：

* **插件匹配文件**：您添加的市场之一或 Claude Code 为您注册的官方市场列出了该文件扩展名的代码智能插件，并且插件的二进制文件已安装。
* **官方优先**：当多个市场为该扩展名提供插件时，对话框提供官方市场的插件。
* **每个会话一次**：对话框在一个会话中最多出现一次，针对 Claude 编辑的第一个匹配文件。
* **不适用于云会话**：当您的终端连接到云会话（例如您使用 [`claude --cloud`](https://code.claude.com/docs/zh-CN/claude-code-on-the-web#from-terminal-to-cloud) 启动的会话）时，对话框永远不会出现。

### 响应推荐对话框

**LSP 插件推荐**对话框命名插件并提供以下选择：

* **是，安装**：Claude Code 为您的用户帐户安装插件并打印 `<plugin> installed · restart to apply`。启动新会话以加载服务器。
* **否，暂不**：对话框关闭，稍后的会话可以再次提供该插件。按 **Esc** 也会执行相同操作。
* **永不为此插件**：对话框停止为该插件出现，但仍为其他插件出现。
* **禁用所有 LSP 推荐**：对话框停止为每种语言出现。

如果您不选择选项，Claude Code 会在 30 秒后关闭它，并将其计为忽略。计数在会话中保持。忽略五个对话框后，Claude Code 停止推荐插件，与您选择**禁用所有 LSP 推荐**相同。

### 重新打开推荐

**LSP 插件推荐**对话框在您选择**禁用所有 LSP 推荐**或忽略它五次后停止出现。

* **禁用或忽略五次**：要在任一情况下重新打开它，请从 `~/.claude.json`（Claude Code 自己的配置文件）中删除 `lspRecommendationDisabled` 和 `lspRecommendationIgnoredCount` 键。
* **永不为此插件**：如果您选择了**永不为此插件**并希望再次提供该插件，请从同一文件中的 `lspRecommendationNeverPlugins` 列表中删除其 `name@marketplace` id。

## 故障排除代码智能

插件故障排除页面在[语言服务器不启动、使用过多内存或报告错误诊断](https://code.claude.com/docs/zh-CN/plugins/troubleshooting.md#language-server-doesnt-start)下涵盖特定于代码智能插件的症状：

* **语言服务器不启动**：您在 `/plugin` 的**错误**选项卡中看到 `Executable not found in $PATH`，或 Claude 从不报告该语言的诊断。
* **高内存使用**：当服务器索引项目时，内存使用增加。
* **monorepo 中的误报诊断**：诊断报告导入为未解决，但实际上已解决。

## 添加没有官方插件的语言

如果您的语言不在[官方插件表格](#install-a-code-intelligence-plugin)中，您仍然可以连接语言服务器。

1. 使用 `.lsp.json` 文件编写插件，该文件命名服务器命令和它处理的文件扩展名。
2. 然后使用 [`--plugin-dir`](https://code.claude.com/docs/zh-CN/plugins/cli-reference#flags-that-load-a-plugin-for-one-session) 加载插件或将其发布到市场。

有关文件的字段和实际示例，请参阅[插件组件中的 LSP 服务器](https://code.claude.com/docs/zh-CN/plugins/components.md#lsp-servers)。

## 后续步骤

* [插件组件中的 LSP 服务器](https://code.claude.com/docs/zh-CN/plugins/components.md#lsp-servers)：为没有官方插件的语言服务器编写 `.lsp.json`
* [安装和管理插件](https://code.claude.com/docs/zh-CN/plugins/install.md)：范围、更新和卸载
* [故障排除插件](https://code.claude.com/docs/zh-CN/plugins/troubleshooting.md)：超出本页语言服务器的加载错误
* [在官方市场中查找插件](https://code.claude.com/docs/zh-CN/plugins/anthropic-marketplaces.md#find-plugins-in-the-official-marketplace)：浏览官方市场其余部分的位置
