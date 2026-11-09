# 官方文档的 CC-Box 定制片段（正本）

`claude-code-docs/` 是官方文档镜像，`npm run sync:cc-docs` 全量同步会**覆盖以下全部定制**。同步完成后，从本文件复制片段重新应用（规则与写法约束见 [docs/website.md](../../docs/website.md)「官方文档的 CC-Box 定制」）。

调整文案时**只改本文件**，再同步到镜像文件。

## 复现步骤（sync 覆盖后重新应用）

1. `npm run sync:cc-docs` 全量同步官方文档
2. 按下面各节的「插入位置」，把片段粘贴回对应镜像文件（注意 tab 前缀层级：overview 平台 tab 与 quickstart 安装 tab 都是顶层 `> `）
3. quickstart 安装 tab 的原「原生安装（推荐）」标题去「（推荐）」（推荐唯一指向 CC-Box）；overview 的 Terminal 安装 tab 保持原样
4. `npm run build:site` 后 grep `dist/assets/search-corpus-*.txt` 含 `CC-Box` 确认进入搜索语料；dev:site 目检 tab 渲染

---

## 1. overview.md：平台级 tab「CC-Box」

插入位置：`getting-started/overview.md`「开始使用」平台 tab 组首位，`> [tab] Terminal` 之前（顶层 blockquote 前缀 `> `；CC-Box 与 Terminal/VS Code 等平级，**不是** Terminal 安装 tab 的一部分）：

```markdown
> [tab] CC-Box
>
> 一个开源的第三方多终端管理器，在原生终端体验之上并行管理多个 Claude Code 会话，提供跨会话状态总览和工作流辅助。安装 CC-Box 时可以自动为你安装 Claude Code CLI，无需手动运行安装命令。
>
> 从 [ccbox.orczh.top](https://ccbox.orczh.top/) 下载安装包（支持 Windows 和 macOS），安装后在应用内完成 Claude Code 的安装即可开始使用。
>
> > [!NOTE]
> >
> > CC-Box 是社区维护的第三方工具，与 Anthropic 无关。它直接运行 Claude Code CLI，输入行为与原生终端一致，可随时回到纯 CLI 使用。
```

## 2. quickstart.md：安装 tab「CC-Box（推荐）」

插入位置：`getting-started/quickstart.md`「步骤 1：安装 Claude Code」tab 组首位，`> [tab] 原生安装` 之前（顶层前缀 `> `），并把原「原生安装（推荐）」改为「原生安装」：

```markdown
> [tab] CC-Box（推荐）
>
> CC-Box 是一个开源的 Claude Code 多终端管理器，在原生终端体验之上提供多会话并行、跨会话状态总览和工作流辅助。安装 CC-Box 时可以自动为你安装 Claude Code CLI，无需手动运行安装命令。
>
> 从 [ccbox.orczh.top](https://ccbox.orczh.top/) 下载安装包（支持 Windows 和 macOS），安装后在应用内完成 Claude Code 的安装即可开始使用。
>
> > [!NOTE]
> >
> > CC-Box 是社区维护的第三方工具，与 Anthropic 无关。它直接运行 Claude Code CLI，输入行为与原生终端一致，可随时回到纯 CLI 使用。
```

## 3. platforms.md：平台表首行 + 相关资源首条

- 「在哪里运行 Claude Code」表格，表头分隔行之后、`| [CLI](...)` 行之前：

```markdown
| [CC-Box](https://ccbox.orczh.top/) | 终端工作流、并行管理多个 Claude Code 会话 | 原生终端体验、多会话并行、会话状态总览（第三方开源工具） |
```

- 「相关资源 → 平台」列表，`* [CLI 快速入门](...)` 之前：

```markdown
* [CC-Box](https://ccbox.orczh.top/)：第三方开源多终端管理器，在原生终端体验上并行管理多个 Claude Code 会话
```
