> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# 在 monorepo 或大型代码库中设置 Claude Code

> 为 monorepo 和大型单树代码库配置 Claude Code，使用嵌套的 CLAUDE.md 文件、稀疏 worktrees、代码智能和按包技能，使 Claude 专注于你正在处理的代码。

大型代码库可以是拥有数百万行代码的单个存储库，也可以是包含许多包的 monorepo。Claude Code 可以在任何规模下工作，但随着代码库的增长，为较小项目调整的默认设置可能会用与任务无关的指令和文件读取填满上下文窗口，浪费 tokens 并降低 Claude 的性能。

本指南向个人开发者和工程团队展示如何将 Claude 的范围限制在任务涉及的代码库部分。每个部分都说明该设置是个人的还是提交到存储库的。

## 本指南涵盖的内容

下面的[表格](#settings-on-this-page)列出了每个设置及其作用。[之后的文件树](#the-example-monorepo)是本页面每个代码示例所引用的示例 monorepo。

### 本页面的设置

下面的每个设置都是独立的。它们相互叠加而不是相互替换，因此应用适合你的存储库的任何设置。[选择从哪里启动 Claude](#choose-where-to-start-claude) 决定了你的设置文件的位置，所以先阅读它。[将其整合在一起](#put-it-together) 展示了所有这些设置的组合。

| 我想要 | 使用 |
| :- | :- |
| 仅加载你接触的代码的约定，而不是一个根文件覆盖每个子系统 | 按目录的 [CLAUDE.md 文件](#layer-claude-md-files-by-directory) |
| 排除你从不处理的包的 CLAUDE.md 文件 | [`claudeMdExcludes`](#exclude-irrelevant-claude-md-files) |
| 阻止 Claude 打开构建输出、生成的代码和供应商依赖 | `permissions.deny` 中的 [`Read` 拒绝规则](#block-reads-of-generated-and-vendored-code) |
| 通过语言服务器而不是扫描文件来查找符号的定义或调用者 | [代码智能插件](#reduce-file-reads-with-code-intelligence) |
| 当 Claude 创建 worktree 时仅检出任务需要的目录 | [`worktree.sparsePaths`](#check-out-only-the-directories-you-need) |
| 从同一会话中读取和编辑同级包或另一个存储库 | [`--add-dir`](#grant-access-across-packages-or-repositories) 或 `additionalDirectories` |
| 给 Claude 特定于一个区域的程序，仅在相关时加载 | 按目录的 [skills](#add-per-directory-skills) |
| 用一套每个人都安装的约定替换许多按目录的 CLAUDE.md 文件 | 内部市场中的 [plugin](#centralize-conventions-when-layering-stops-scaling) |

> [!TIP]
>
> 有关在任何仓库中保持上下文较小的工作流技术，请参阅 [Claude Code 的最佳实践](https://code.claude.com/docs/zh-CN/best-practices.md)。一个例子是[在子代理中运行探索](https://code.claude.com/docs/zh-CN/best-practices.md#know-what-subagents-cost)，以使文件读取不会进入主对话。要向组织中的每个开发者推出基线配置，请参阅[为您的组织设置 Claude Code](https://code.claude.com/docs/zh-CN/admin-setup.md)。

### 示例 monorepo

本页面的示例引用了一个包含三个包的 monorepo。相同的模式适用于大型单树代码库：其中示例使用 `packages/api/`，替换为你自己的子系统目录，例如 `src/backend/` 或 `lib/core/`。

```text theme={null}
monorepo/
  CLAUDE.md                     # 根指令
  packages/
    api/
      CLAUDE.md                 # API 特定指令
      .claude/skills/
      src/
    web/
      CLAUDE.md                 # 前端特定指令
      .claude/skills/
      src/
    shared/
      CLAUDE.md                 # 共享库指令
      src/
```

## 选择从哪里启动 Claude

你启动 `claude` 的位置决定了 Claude 可以读取和编辑哪些文件而无需额外权限授予、在启动时加载哪些 CLAUDE.md 文件，以及哪些项目设置适用。

| 从以下位置启动 | 文件访问 | 启动时加载的 CLAUDE.md | 使用场景 |
| :- | :- | :- | :- |
| 仓库根目录 | 每个文件 | 仅根目录；子目录文件按需加载 | 任务跨越多个包或子系统 |
| 子目录 | 仅该子树，直到你授予更多权限 | 该目录的加上每个祖先的 | 工作范围限于一个包或子系统 |

`.claude/settings.json` 中的项目设置不像 CLAUDE.md 文件那样从父目录继承。关于会话读取哪个目录的 `.claude/settings.json`，请参阅 [Claude Code 查找每个文件的位置](https://code.claude.com/docs/zh-CN/settings.md#where-claude-code-looks-for-each-file)。

下面的每个部分都说明其设置文件应该位于存储库根目录还是你启动的子目录中，以及它是提交的还是保持本地的。

## 按目录分层 CLAUDE.md 文件

在大型代码库中，存储库根目录的单个 CLAUDE.md 往往要么增长到覆盖每个子系统的约定，在与当前任务无关的指令上浪费上下文，要么保持太通用而无用。将指令分散在按目录的文件中意味着 Claude 加载存储库范围的规则加上仅你正在处理的代码的约定。

Claude Code 在启动时从您的工作目录和每个父目录加载每个 [CLAUDE.md](https://code.claude.com/docs/zh-CN/memory.md) 文件，然后[按需](https://code.claude.com/docs/zh-CN/memory.md#how-claude-md-files-load)加载每个子目录的文件。根文件设置仓库范围的规则，每个子目录添加自己的规则。

常见的分割是两个级别：

* **根 `CLAUDE.md`**：适用于任何地方的指令，例如编码标准和提交约定
* **按子目录 `CLAUDE.md`**：特定于该区域堆栈的约定。在 monorepo 中，这是每个包一个。在大型单树中，它是每个子系统一个，例如 `src/db/` 或 `src/api/`

将这些文件提交到存储库，以便队友继承它们。每个目录的所有者通常维护其文件。

要修剪已签入的文件，请运行 [`/doctor` 检查](https://code.claude.com/docs/zh-CN/memory#my-claude-md-is-too-large)。根 `CLAUDE.md` 包含在每个包中适用的规则：

```markdown CLAUDE.md theme={null}
Run package scripts from the package directory, not the monorepo root.
Prefix commit subjects with the package name, for example `api: add rate limiting`.
Never edit files under packages/*/generated/. Run `npm run codegen` in the package instead.
```

每个子目录的 `CLAUDE.md`，这里是 `packages/api/CLAUDE.md`，添加特定于该区域的约定：

```markdown packages/api/CLAUDE.md theme={null}
Copy `.env.example` to `.env` before running anything. Tests and the dev server fail without it.
Write database queries with the Knex query builder. Never put raw SQL strings in route handlers.
Never edit a migration after it has merged. Add a new migration instead.
```

当你从 `packages/api/` 启动 Claude 时，它加载 `packages/api/CLAUDE.md` 和根 `CLAUDE.md`。Claude 看到本地指令与存储库范围的规则一起，上下文中没有来自 `packages/web/` 的指令。对于非 monorepo 树中的任何子目录也是如此。要确认加载了哪些文件，请运行 `/context` 并检查 **Memory files** 下的列表。

保持文件随着代码库和模型变化而最新的几种方法：

* **在拉取请求中审查**：像对待任何其他文档更改一样对待 CLAUDE.md 编辑，以便约定跟踪代码
* **在主要模型发布后重新访问**：适用于较旧模型限制的指令一旦较新模型自己处理该情况，可能会变成开销。例如，强制单文件重构的规则一旦限制消失就可以删除
* **添加一个 Stop hook 来提议更新**：[`Stop` hook](https://code.claude.com/docs/zh-CN/hooks#stop) 在 Claude 完成响应时接收会话记录的路径，所以脚本可以审查会话并在暴露的差距仍然新鲜时提议 CLAUDE.md 更新

有关 CLAUDE.md 文件如何加载和交互的更多信息，请参阅[内存和项目指令](https://code.claude.com/docs/zh-CN/memory.md)。

### 在按目录 CLAUDE.md 和路径范围规则之间选择

按目录的 `CLAUDE.md` 文件和 `.claude/rules/` 下的[路径范围规则](https://code.claude.com/docs/zh-CN/memory.md#path-specific-rules)都允许你将指令定向到树的一部分。它们在文件位置和加载时间上有所不同。

| 方法 | 文件位置 | 加载时间 | 使用场景 |
| :- | :- | :- | :- |
| 按目录 `CLAUDE.md` | 在目录内，与其代码一起 | 从该目录启动时在启动时加载，或按需加载 | 目录所有者维护自己的约定；指令与代码一起版本化 |
| `.claude/rules/` 中的路径范围规则 | 存储库根目录的中央 `.claude/` | 当 Claude 处理与规则的 `paths:` glob 匹配的文件时 | 你想要一个地方的所有约定，或相同的规则适用于许多分散的路径 |

有关也涵盖 skills 的比较，请参阅[比较相似功能](https://code.claude.com/docs/zh-CN/features-overview.md#compare-similar-features)。

### 排除不相关的 CLAUDE.md 文件

当您从仓库根目录启动 Claude 时，每个子目录的 CLAUDE.md 都可能在会话期间[按需加载](https://code.claude.com/docs/zh-CN/memory.md#how-claude-md-files-load)。`claudeMdExcludes` 设置按路径或 glob 模式跳过特定文件，以便它们永远不会加载。

对你从不处理的目录使用此功能，例如其他团队的包、遗留代码或供应商子树。排除列表是静态的，不是按任务的开关。要今天专注于一个包，明天专注于另一个包，[从该包的目录启动 Claude](#choose-where-to-start-claude) 而不是编辑排除。

如果你只想为自己排除这些，将设置放在 `.claude/settings.local.json` 中。Claude Code 在保存设置时会将该文件添加到你的全局 gitignore。由于你在这里手动创建它，请将其添加到你的 gitignore。模式使用 glob 语法与绝对文件路径匹配，所以以相对样式模式开头的 `**/` 以在树中的任何地方匹配。下面的示例排除其他团队拥有的包：

```json .claude/settings.local.json theme={null}
{
  "claudeMdExcludes": [
    "**/packages/web/**"
  ]
}
```

这跳过该包下的每个 CLAUDE.md 和规则文件。根 CLAUDE.md 和你处理的包仍然正常加载。

这些模式涵盖其他常见情况：

* `"**/packages/*/CLAUDE.md"`：排除每个包的 CLAUDE.md，同时保留根目录
* `"**/packages/legacy-*/**"`：排除每个名称与 glob 匹配的包，包括规则
* `"/home/user/monorepo/legacy/CLAUDE.md"`：按绝对路径排除一个特定文件

您可以在任何[设置作用域](https://code.claude.com/docs/zh-CN/settings.md#where-settings-live)设置 `claudeMdExcludes`：用户、项目、本地或托管。数组会跨作用域合并，因此团队可以设置项目级别的默认值，同时个人添加本地覆盖。如果您的组织部署了[托管 CLAUDE.md 文件](https://code.claude.com/docs/zh-CN/memory.md#deploy-organization-wide-claude-md)，则无论由哪个作用域设置模式，与其匹配的模式都不会跳过它。

有关完整的排除文档，请参阅[排除特定 CLAUDE.md 文件](https://code.claude.com/docs/zh-CN/memory.md#exclude-specific-claude-md-files)。

## 减少 Claude 读取的内容

指令只是最终进入 Claude 上下文的一部分。文件读取是另一个随着代码库增长而增加的成本。下面的设置阻止读取不相关的路径，并用语言服务器查找替换详尽的文件扫描。

### 阻止读取生成的和供应商代码

Claude 的内容搜索默认尊重 `.gitignore`，所以已列在其中的路径，例如 `node_modules/`、`dist/` 和 `build/`，无需额外配置就会保持在搜索结果之外。

对于已检入的路径，例如供应商 SDK 或提交的生成代码，在 `permissions.deny` 中添加 `Read` 拒绝规则以阻止 Claude 打开这些文件。

拒绝规则可以覆盖在存储库中工作的每个人、仅你自己或机器上的每个会话，具体取决于你将它们放在哪个设置文件中：

* **在存储库中工作的每个人**：将规则提交到 `.claude/settings.json`，位于存储库根目录（如果你从那里启动 Claude），或位于每个包的 `.claude/`（如果你从子目录启动）。与本页面的其他项目设置一样，该文件不会从父目录继承。
* **仅您自己**：使用位于仓库根目录的 `.claude/settings.local.json`。它在仓库内的每个 CLI 会话中加载，无论启动目录如何，除了 Claude Code [不使用仓库根目录](https://code.claude.com/docs/zh-CN/settings.md#where-claude-code-looks-for-each-file)的情况，例如在 Windows 上。

  相对模式（如示例中的 `Read(./**/vendor/**/*)`）仍然[锚定在会话的当前工作目录](https://code.claude.com/docs/zh-CN/permissions.md#read-and-edit)而不是仓库根目录。如果您从子目录启动会话，请在此文件中将规则写为 `//` 绝对路径，例如 `Read(//absolute/path/to/repo/**/vendor/**/*)`。
* **每个人，在每个会话中强制执行**：在[托管设置](https://code.claude.com/docs/zh-CN/managed-settings.md)中设置规则，用户和项目设置无法覆盖。

下面的示例阻止构建工件和供应商 SDK。其目录模式以 `/**/*` 而不是 `/**` 结尾，以便每个规则覆盖目录内的所有内容，但不覆盖目录本身。Claude 仍然可以列出这些目录或进入它们，例如使用 `ls dist` 或 `cd build`。

```json .claude/settings.json theme={null}
{
  "permissions": {
    "deny": [
      "Read(./**/dist/**/*)",
      "Read(./**/build/**/*)",
      "Read(./**/*.generated.*)",
      "Read(./**/vendor/**/*)"
    ]
  }
}
```

拒绝规则涵盖 Claude 的内置文件工具。在 Bash 中，它们涵盖 Claude Code 识别的文件命令，例如 `cat`、`head`、`grep` 和 `find`，当拒绝的路径作为参数出现时，以及[重定向](https://code.claude.com/docs/zh-CN/permissions.md#redirections)的目标，例如 `< file`。Claude Code 还尽力尝试将拒绝的路径排除在内置 Grep 和 Glob 工具的结果之外。对包含拒绝文件的目录进行的 Bash 搜索（例如 `grep -r` 或 `find`）仍然会在其输出中包含它们。

拒绝规则不涵盖自己打开文件的子进程。有关完整的模式语法，请参阅 [Read 和 Edit 权限规则](https://code.claude.com/docs/zh-CN/permissions.md#read-and-edit)。

### 使用代码智能减少文件读取

在大型代码库中，查找符号的定义或使用位置可能需要许多文件读取和 grep 调用。[代码智能插件](https://code.claude.com/docs/zh-CN/plugins/code-intelligence.md)将 Claude 连接到语言服务器，以便它可以跳转到定义、查找引用和直接显示类型错误，而不是扫描树。

官方市场有 TypeScript、Python、Go、Rust 和其他常见语言的插件。在 VS Code 扩展或桌面应用中，请按照[安装插件](https://code.claude.com/docs/zh-CN/plugins/install.md#install-a-plugin)进行安装。在终端中，运行 `claude` 启动 Claude Code，然后在其输入框中输入以下内容来安装 TypeScript 插件：

```shell theme={null}
/plugin install typescript-lsp@claude-plugins-official
```

如果安装失败，请匹配 Claude Code 报告的消息：

* `Marketplace "claude-plugins-official" not found`：使用 `/plugin marketplace add anthropics/claude-plugins-official` 添加市场，然后重试安装。
* `Plugin "<name>" not found in marketplace "claude-plugins-official"`：您本地的市场目录副本可能已过期。使用 `/plugin marketplace update claude-plugins-official` 刷新它，然后重试安装。如果重试后仍显示相同的消息，请参阅[在市场中找不到插件](https://code.claude.com/docs/zh-CN/plugins/troubleshooting.md#plugin-not-found-in-marketplace)。

要为存储库中的每个人启用插件而不是自己安装，请将其添加到 [`enabledPlugins` 项目设置](https://code.claude.com/docs/zh-CN/settings-reference#plugin-settings)。

代码智能插件需要每个开发者机器上的语言的语言服务器二进制文件。查看[每种语言需要哪个二进制文件](https://code.claude.com/docs/zh-CN/plugins/code-intelligence.md)。从官方市场安装需要网络访问 GitHub，市场在那里托管。在受限网络上，[从内部 Git 主机或本地路径添加市场](https://code.claude.com/docs/zh-CN/plugins/install.md#add-a-marketplace)。

这与上面的 `claudeMdExcludes` 和 `Read` 拒绝规则配对良好。那些保持不相关的内容不进入上下文，代码智能保持 Claude 不读取剩余的内容来定位定义。

## 范围 worktrees 和文件访问

这些设置控制 worktrees 中磁盘上的内容以及 Claude 可以读取和写入的超出启动点的目录。

### 仅检出你需要的目录

`--worktree` 标志在新的 git worktree 中启动会话，以便更改与主检出隔离。默认情况下，它检出整个存储库。在大型存储库中，`worktree.sparsePaths` 设置使用 git sparse-checkout 仅将列出的目录加上根级文件写入磁盘，以便 worktrees 启动更快并使用更少空间。

如果在此目录中工作的每个人都需要相同的路径，将设置提交到 `.claude/settings.json`。要为自己添加路径，使用 `.claude/settings.local.json`：列表在范围内合并，所以本地文件可以向提交的列表添加路径但不能删除它们。

本页面上的 JSON 示例一次显示一个设置。如果你的 `.claude/settings.json` 已经包含其他键，例如上面的 `permissions.deny` 规则，请在它们旁边添加 `worktree` 键，而不是替换文件。[将其放在一起](#put-it-together)显示组合结果。

下面的示例显示提交的文件：

```json .claude/settings.json theme={null}
{
  "worktree": {
    "sparsePaths": [
      ".claude",
      "packages/api",
      "packages/shared"
    ]
  }
}
```

当 Claude 创建 worktree 时，它仅检出 `.claude/`、`packages/api/` 和 `packages/shared/` 而不是完整树。`sparsePaths` 中的路径相对于存储库根目录，无论你从哪个子目录启动 Claude。任何目录路径都可以在这里工作，不仅仅是包根。

这对于[子代理 worktree 隔离](https://code.claude.com/docs/zh-CN/worktrees.md#isolate-subagents-with-worktrees)特别有用。子代理是为子任务生成的并行 Claude 实例，每个在 worktree 中运行的都获得轻量级检出而不是完整树。会话中的所有 worktrees 共享相同的 `sparsePaths`，所以如果一个子代理需要 `packages/api/` 而另一个需要 `packages/web/`，列出两者。

在 `sparsePaths` 中列出目录，而不是单个文件。根级文件如 `package.json`、`tsconfig.base.json` 和锁文件始终与你列出的目录一起检出。根级目录不是，所以如果你想要存储库根目录的 `.claude/settings.json` 或 `.claude/rules/` 在 worktree 内可用，请在列表中包含 `.claude`。对于项目 skills、代理和命令，请参阅[worktrees 与主检出共享的内容](https://code.claude.com/docs/zh-CN/worktrees.md#what-worktrees-share-with-the-main-checkout)。

Sparse checkout 需要 git 在存在 sparse worktree 时在仓库的共享 `.git/config` 中启用 `extensions.worktreeConfig`。Claude Code 在删除最后一个 worktree 后会删除该条目，但仅当 Claude Code 添加了它时。它永远不会删除您自己设置的值。

要避免在 worktrees 中复制大型目录如 `node_modules`，将 `sparsePaths` 与同一 `.claude/settings.json` 中的 `symlinkDirectories` 配对：

```json .claude/settings.json theme={null}
{
  "worktree": {
    "sparsePaths": [
      ".claude",
      "packages/api",
      "packages/shared"
    ],
    "symlinkDirectories": [
      "node_modules"
    ]
  }
}
```

这创建了一个从每个 worktree 的 `node_modules/` 回到主存储库副本的符号链接，而不是在磁盘上复制它。

> [!NOTE]
>
> `sparsePaths` 和 `symlinkDirectories` 设置在创建 worktree 之前从你的启动目录读取。创建后，会话的工作目录是 worktree 根，而不是你启动的子目录。因此，worktree 内的项目设置从 worktree 根的 `.claude/settings.json`（存储库根文件的检出副本）加载。将你在 worktrees 内需要的任何其他设置（例如权限规则或 hooks）放在存储库根的 `.claude/settings.json` 中。

有关完整的 worktree 设置参考，请参阅 [Worktree 设置](https://code.claude.com/docs/zh-CN/settings-reference.md#worktree)。

### 跨包或存储库授予访问权限

当你从子目录启动 Claude 时，或当任务跨越多个检出时，本部分适用。如果你在单个大型树中从存储库根目录启动，Claude 已经可以访问每个文件，你可以跳过此部分。

当您从 `packages/api/` 启动 Claude 时，它可以读取和写入该目录内的文件。如果任务需要跨包更改，例如更新 `api` 和 `web` 都导入的共享类型，您需要授予对同级目录的访问权限。相同的机制也可授予对单独检出的仓库的访问权限。

`.claude/settings.json` 中的 `additionalDirectories` 设置给 Claude 访问工作目录外的目录。下面的示例授予对两个同级包的访问权限：

```json packages/api/.claude/settings.json theme={null}
{
  "permissions": {
    "additionalDirectories": [
      "../shared",
      "../web"
    ]
  }
}
```

相对路径相对于你启动 Claude 的目录解析。使用此配置，Claude 可以在从 `packages/api/` 工作时读取和编辑 `packages/shared/` 和 `packages/web/` 中的文件。

你也可以在运行时不编辑设置而授予访问权限，通过在启动 Claude 时传递 `--add-dir`：

```bash theme={null}
claude --add-dir ../shared
```

无论你如何添加目录，Claude 都可以读取和编辑其中的文件。目录的 CLAUDE.md、`.claude/rules/` 文件和 skills 是否也加载取决于你如何添加它：

| 添加方式 | 加载 CLAUDE.md 和规则 | 加载 skills |
| :- | :- | :- |
| `additionalDirectories` 设置 | 从不 | 从不 |
| `--add-dir` 标志或 `/add-dir` 命令 | 仅使用下面的环境变量 | 是 |

要从使用 `--add-dir` 或 `/add-dir` 添加的目录加载 CLAUDE.md 和规则文件，设置 `CLAUDE_CODE_ADDITIONAL_DIRECTORIES_CLAUDE_MD` 环境变量：

```bash theme={null}
CLAUDE_CODE_ADDITIONAL_DIRECTORIES_CLAUDE_MD=1 claude --add-dir ../shared
```

环境变量对 `additionalDirectories` 设置中列出的目录没有影响。详见[从其他目录加载](https://code.claude.com/docs/zh-CN/memory.md#load-from-additional-directories)。

对于此区域中的每个人都需要的同级目录，将 `additionalDirectories` 提交到 `.claude/settings.json`。对于个人选择或一次性访问，使用 `.claude/settings.local.json` 或在启动时传递 `--add-dir`。

## 添加按目录 skills

任何子目录都可以定义范围限于其自己技术栈的 [skill](https://code.claude.com/docs/zh-CN/skills.md)。skill 在 Claude 确定其相关时按需加载。

Skills 位于目录内的 `.claude/skills/` 下。将它们与该区域的代码一起提交，以便克隆存储库的任何人都能获得它们。在 monorepo 中，这可以是每个包一套 skills。在大型单树代码库中，它是每个子系统一套，例如 `src/db/.claude/skills/`。

在子目录内创建一个 skill 目录：

```bash theme={null}
mkdir -p packages/api/.claude/skills/api-testing
```

然后在该目录内写 `SKILL.md`，这里是 `packages/api/.claude/skills/api-testing/SKILL.md`。此示例教 Claude API 包的测试模式：

```markdown packages/api/.claude/skills/api-testing/SKILL.md theme={null}
---
name: api-testing
description: API 包的测试模式。在 packages/api/ 中编写或修改测试时使用。
---

## 测试结构

测试在 `src/__tests__/` 中，镜像 `src/` 目录结构。
每个路由文件都有一个对应的 `.test.ts` 文件。

## 运行测试

- 所有测试：`npm test`
- 单个文件：`npm test -- src/__tests__/routes/users.test.ts`
- 监视模式：`npm test -- --watch`

## 测试实用程序

- `src/__tests__/helpers/db.ts`：提供 `setupTestDb()` 和 `teardownTestDb()` 用于数据库测试
- `src/__tests__/helpers/auth.ts`：提供 `createTestUser()` 和 `getAuthToken()` 用于认证端点

## 模式

- 使用 `supertest` 进行 HTTP 断言，而不是原始 fetch
- 始终在回滚的事务中包装数据库测试
- 在 `src/__tests__/mocks/` 中模拟外部服务
```

不同的子目录以相同的方式保存不同的 skill：`packages/web/.claude/skills/component-patterns/` 描述前端的组件约定而不是测试。当 Claude 处理 `packages/api/` 中的文件时，它加载 api-testing skill。当它在 `packages/web/` 中工作时，它加载 component-patterns。

您也可以按文件模式而不是按放置位置来限定 skill 的范围。[`paths` frontmatter 字段](https://code.claude.com/docs/zh-CN/skills#frontmatter-reference)采用 glob 模式，Claude 仅在处理匹配文件时自动加载 skill。对于位于仓库根目录的 `.claude/skills/` 中但仅适用于某些文件的 skill，可使用此功能，例如限定于 `**/migrations/**` 的数据库迁移 skill。

有关创建和组织 skills 的更多信息，请参阅 [Skills](https://code.claude.com/docs/zh-CN/skills.md)。

### 保持 skills 可发现

随着 skills 分散在许多目录中，Claude 选择的列表可能会增长很大。Claude 通过读取每个发现的 skill 的名称和描述来选择 skill，只有选定的 skill 的完整内容加载到上下文中。本部分涵盖如何保持该列表较小。

哪些 skills 在范围内取决于你从哪里启动 Claude：

* **从子目录如 `packages/api/`**：来自该目录、每个父目录直到存储库根目录以及用户和企业级别的 skills
* **从存储库根目录**：根 skills，加上来自 Claude 在会话期间接触的每个子目录的 skills，可能累积到数百个
* **在使用 [`--add-dir`](#grant-access-across-packages-or-repositories) 添加同级后**：该同级的 skills 也加载。`additionalDirectories` 设置仅授予文件访问权限，不加载 skills

名称始终加载，但[当有许多时，某些 skills 会完全失去其描述](https://code.claude.com/docs/zh-CN/skills.md#skill-descriptions-are-cut-short)，这可能会剥离 Claude 用来决定 skill 是否适用的关键字。保持描述简短并以请求会包含的词开头，例如"在 `packages/api/` 中编写或修改测试"。

对于许多目录共享的 skills，例如 PR 约定或部署检查清单，将它们放在存储库根目录的 `.claude/skills/` 中，以便从任何启动目录加载。当共享 skills 需要自己的版本历史或必须跨存储库工作时，改为将它们打包为[插件](https://code.claude.com/docs/zh-CN/plugins/overview.md)。插件 skills 使用 `plugin-name:skill-name` 命名空间，所以它们永远不会与按目录的 skills 冲突。平台团队可以在一个地方对它们进行版本化和更新。

要查找哪些 skills 未被使用，启用 OpenTelemetry [日志导出器](https://code.claude.com/docs/zh-CN/monitoring-usage.md)并设置 `OTEL_LOG_TOOL_DETAILS=1` 以便 skill 名称被逐字记录而不是被编辑。[`skill_activated` 事件](https://code.claude.com/docs/zh-CN/monitoring-usage#skill-activated-event)在其 `skill.name` 属性中记录每个调用，`invocation_trigger` 记录命令、Claude 或嵌套 skill 是否调用它，这告诉你要合并或停用什么。

## 当分层停止扩展时集中约定

随着代码库的增长，按目录的 CLAUDE.md 文件可能变得难以管理。约定漂移，文件变得陈旧，没有人拥有根目录。解决这个问题通常落在维护存储库 Claude Code 设置的团队身上，而不是在自己的区域中工作的每个开发者。

将约定和参考内容从始终加载的 CLAUDE.md 移出到按需加载的机制中：

* [Skills](https://code.claude.com/docs/zh-CN/skills.md)：Claude 仅在与任务相关时加载的参考材料
* [插件](https://code.claude.com/docs/zh-CN/plugins/overview.md)：由 skill、hook 和命令组成的版本化包，由平台团队集中拥有，并可[通过托管设置为所有人安装](https://code.claude.com/docs/zh-CN/plugins/org.md#pre-install-and-require-plugins)
* [MCP servers](https://code.claude.com/docs/zh-CN/mcp.md)：如果你的组织已经在存储库上运行代码搜索或 RAG 索引，将其公开为 MCP 工具，以便 Claude 查询它而不是直接读取文件

有关平台团队如何集中强制这些的信息，请参阅[服务器管理或端点管理的设置](https://code.claude.com/docs/zh-CN/server-managed-settings.md#choose-between-server-managed-and-endpoint-managed-settings)。

### 在会话启动时推荐正确的插件

一旦约定位于插件中，在树的陌生部分启动 Claude 的队友就没有关于该区域所有者维护哪个插件的信号。[`SessionStart` hook](https://code.claude.com/docs/zh-CN/hooks#sessionstart) 可以弥补这个差距，因为 hook 打印到 stdout 的任何内容都会在第一个提示之前添加到 Claude 的上下文中。

例如，你可以编写一个脚本，从[hook 输入](https://code.claude.com/docs/zh-CN/hooks.md#common-input-fields)读取启动目录，在提交到存储库的路径到插件映射中查找它，并打印建议供 Claude 在其第一个回复中中继。查看[使用 hooks 自动化操作](https://code.claude.com/docs/zh-CN/hooks-guide.md)来编写和注册 hook。

## 将其整合在一起

下面的组合配置使用 monorepo 布局。相同的文件适用于大型单树中的任何子目录。每个子目录的 `.claude/settings.json` 必须是自包含的而不是分层在根文件上。

示例在 `.claude/settings.json` 中提交 `worktree`、`additionalDirectories` 和 `Read` 拒绝规则，以便 `packages/api/` 中的每个开发者获得相同的同级访问、稀疏路径和排除。下面的文件是 `packages/api/` 的提交的按区域设置：

```json packages/api/.claude/settings.json theme={null}
{
  "worktree": {
    "sparsePaths": [
      ".claude",
      "packages/api",
      "packages/shared"
    ],
    "symlinkDirectories": [
      "node_modules"
    ]
  },
  "permissions": {
    "additionalDirectories": [
      "../shared"
    ],
    "deny": [
      "Read(./**/dist/**/*)",
      "Read(./**/build/**/*)"
    ]
  }
}
```

因为此会话从 `packages/api/` 启动，同级包的 CLAUDE.md 文件已经超出范围，所以这里不需要 `claudeMdExcludes`。如果你也从根目录启动会话，改为将其添加到存储库根目录的 `.claude/settings.local.json`。

`additionalDirectories` 条目在你直接从 `packages/api/` 启动 Claude 时适用。在从此会话创建的 worktree 内，工作目录是 worktree 根，所以此设置文件不加载。同级包已经在 worktree 内可达而无需它，但拒绝规则需要在存储库根目录的 `.claude/settings.json` 中的第二个副本，以便 worktree 会话获取它们，如[worktree 设置注释](#check-out-only-the-directories-you-need)所述：

```json .claude/settings.json theme={null}
{
  "permissions": {
    "deny": [
      "Read(./**/dist/**/*)",
      "Read(./**/build/**/*)"
    ]
  }
}
```

设置后，存储库具有此布局：

```text theme={null}
monorepo/
  CLAUDE.md
  .claude/settings.json                           # worktree 会话的拒绝规则
  packages/
    api/
      CLAUDE.md
      .claude/settings.json                       # worktree、additionalDirectories、拒绝规则
      .claude/skills/api-testing/SKILL.md
    web/
      CLAUDE.md
      .claude/skills/component-patterns/SKILL.md
    shared/
      CLAUDE.md
```

使用此设置，从 `packages/api/` 启动 Claude：

* 加载根 CLAUDE.md 和 `packages/api/CLAUDE.md`，跳过 `packages/web/CLAUDE.md`
* 可以读取和编辑 `packages/api/` 和 `packages/shared/` 中的文件
* 跳过 `packages/api/` 中 `dist/` 和 `build/` 下的构建输出读取
* 有 api-testing skill 按需可用
* 创建包含 `.claude/`、`packages/api/`、`packages/shared/` 和根级文件的 worktrees，拒绝规则从根设置文件应用于整个 worktree

## 范围和计划跨包的更改

上面的配置控制 Claude 看到的内容。当单个更改涉及多个包时，例如更新共享类型以及使用它的每个调用站点，你如何范围和排序任务也会影响结果。

两种技术帮助保持跨包更改的一致性：

* **在一个会话中给 Claude 整个更改**：将共享编辑及其调用站点一起交付保持每个编辑背后的决策一致，而不是按包重新推导它们
* **在编辑前计划**：先在[计划模式](https://code.claude.com/docs/zh-CN/permission-modes.md#analyze-before-you-edit-with-plan-mode)中进行计划，Claude 会将计划写入文件。长的跨包会话会在进行过程中[压缩其上下文](https://code.claude.com/docs/zh-CN/context-window.md#what-survives-compaction)。Claude Code 在每次压缩后重新注入计划文件，因此即使对话历史可能无法保留，计划也能保留下来

## 后续步骤

一旦此配置就位，你可以细化它：

* 使用 [hooks](https://code.claude.com/docs/zh-CN/hooks-guide.md) 在 Claude 编辑文件后运行按目录的 linters 或类型检查器
* 查看[有效管理成本](https://code.claude.com/docs/zh-CN/costs.md)以了解代码库大小如何影响 token 使用以及如何在更广泛推出前设置支出限制
* 在 Claude 博客上阅读[Claude Code 如何在大型代码库中工作](https://claude.com/blog/how-claude-code-works-in-large-codebases-best-practices-and-where-to-start)，了解组织推出模式和所有权模型，这些模型位于本页面的按存储库配置之上
