> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# 通信工具包

> 推出公告、滴灌式营销信息和常见问题解答，用于在您的工程组织中推出 Claude Code。

本页面适用于在团队中推出 Claude Code 的管理员和工程主管。它提供了即用型的推出公告、技巧和窍门滴灌式营销活动，以及针对您最常被问到的问题的单行常见问题解答。

> [!NOTE]
>
> 将此处的所有内容视为草稿副本，而不是最终副本。用您组织的语气重写每条消息，用您自己代码库中的真实错误和模块替换示例任务，并在发送前替换 `[括号占位符]`。推动采用的公告是那些看起来像您公司某人写的公告。

## 推出通信

一个公告分为两种格式，加上两个可选变体。选择最适合您的推出方式的版本，然后从那里开始重写。

### 发送前

在公告发出前，请完成此清单。每一项都会关闭一个差距，否则会变成推出当天的支持线程。

| 项目 | 为什么重要 |
| - | - |
| `#claude-code` 频道已创建并在消息中链接 | 为问题提供一个统一的落地点 |
| 在您环境中至少一台机器上测试了安装命令 | 在所有人同时遇到代理或防火墙问题之前捕获它们 |
| 安全和数据处理链接已准备好（[数据使用](https://code.claude.com/docs/zh-CN/data-usage.md) 或您的内部等效项） | "我的代码去哪里了？" 将是第一个回复 |
| 已选择一个具体的首个任务，您代码库中的真实错误或文件 | 通用示例不会转化；"修复 `auth_test.go` 中的不稳定测试" 会 |
| 为前 48 小时指定的频道所有者 | 未回答的推出当天问题会杀死势头 |
| 已安排一位 C 级高管赞助商发送或共同签署公告 | 由高管发送的推出在第一周采用率上始终比由管理员或工具团队发送的相同消息更高 |

### 公告

将此用作您的标准组织范围推出消息。它涵盖了 Claude Code 是什么，提供了两分钟的安装路径，为读者提供了一个具体的任务来尝试，并在任何人必须询问之前回答了"我的代码去哪里了？"。

> [tab] 电子邮件
>
> ```text theme={null}
> 主题：Claude Code 现已为 [工程部门 / 您的团队] 推出
>
> 团队，
>
> 从今天开始，您可以访问 Claude Code，这是一个在您的终端中运行、读取您的实际代码库并端到端处理真实任务的 AI 编码 Agent：调试、重构、测试、PR。它不是自动完成，也不是聊天窗口。它会编辑文件并运行您的命令。
>
> 在两分钟内开始运行：
>
>     curl -fsSL https://claude.ai/install.sh | bash
>     cd <your-repo>
>     claude
>
> 然后运行 /init 一次。Claude 读取您的项目并写入一个 CLAUDE.md，其中包含您的构建命令和约定，因此您不再需要重新解释基础知识。
>
> 然后在您已经在的仓库上尝试以下其中之一：
>
>   - "文件 [file] 中的测试不稳定。找出原因并修复它"
>   - "向我介绍 [module] 如何处理 [X]"
>   - "查看我当前工作的 diff，并在我推送之前告诉我有哪些风险"
>
> 您的代码去哪里了：Claude Code 在您的终端中运行，直接与 Anthropic 的 API 通信。在我们的 Team 或 Enterprise 计划下，Anthropic 不会使用您的代码或提示词来训练其模型。
> 详情：https://code.claude.com/docs/en/data-usage
>       https://code.claude.com/docs/en/security
>
> 有问题去哪里：#claude-code。[所有者名称] 本周在关注它。
>
> - [名称]
>
> 附注：更喜欢您的编辑器？有一个 VS Code 扩展和一个 JetBrains 插件。相同的 Agent，不需要终端。
> ```

> [tab] Slack 或 Teams
>
> ```markdown theme={null}
> 🚀 *Claude Code 现已为 [团队] 推出*
>
> AI 编码 Agent，在您的终端中运行，读取您的仓库，完成真实工作：
> 错误、重构、测试、PR。
>
> `curl -fsSL https://claude.ai/install.sh | bash` → `cd your-repo` → `claude`
>
> *首先尝试的事情* → 运行 `/init`，然后："文件 [file] 中的测试不稳定，
> 找出原因并修复它。"
>
> 🔒 在您的终端中运行，直接与 Anthropic 的 API 通信。在我们的 Team 或
> Enterprise 计划下，您的代码和提示词不会用于训练模型。
> 数据使用 → https://code.claude.com/docs/en/data-usage
>
> 📚 快速入门 · VS Code · 免费 1 小时课程
>    https://code.claude.com/docs/en/quickstart
>    https://code.claude.com/docs/en/vs-code
>    https://academy.claude.com/courses/claude-code-in-action
>
> 问题 → 此线程。[所有者] 在处理。
> ```

### 执行赞助商变体

从您的赞助执行官（如 CTO、CIO 或 SVP 工程）的名义和他们的账户发送此消息。以高管名义发出的推出在开启率和第一周激活速度上始终比来自管理员或工具团队的相同消息更高。它表示公司优先级而不是可选实验。

此版本故意精简为一个要求：安装它并在一个真实任务上运行它。高管的工作是让要求落地；标准公告和 `#claude-code` 处理方式。

> [tab] 电子邮件
>
> ```text theme={null}
> 主题：我希望每位工程师本周尝试的一件事
>
> 团队，
>
> 我们已为所有工程部门启用了 Claude Code。这是一个直接在您的终端中工作、在您的实际代码库上工作的 AI 代理，已经使用它的团队的早期结果足够强劲，我希望每个人本周都使用它。
>
> 我要求十分钟：
>
>     curl -fsSL https://claude.ai/install.sh | bash
>     cd <your-repo>
>     claude
>
> 然后给它一个真实的任务：您一直在推迟的错误，或"向我介绍 [module] 如何工作"。
>
> 这就是全部要求。[所有者名称] 和团队在 #claude-code 中处理您遇到的任何问题。
>
> - [执行官名称]
>   [职位]
> ```

> [tab] Slack 或 Teams
>
> ```markdown theme={null}
> 📣 *来自 [执行官名称]：本周尝试的一件事*
>
> 我们已为所有工程部门启用了 *Claude Code*。早期结果足够强劲，我要求每个人本周在真实工作上给它十分钟。
>
> `curl -fsSL https://claude.ai/install.sh | bash` → `cd your-repo` →
> `claude` → 给它一个真实的任务。
>
> 就这样。问题 → #claude-code。
> ```

### 试点组变体

用于分阶段推出。仅发送给试点队列。

```text theme={null}
主题：您在 Claude Code 试点中

[名称 / 团队]，

您在 [公司] 的 Claude Code 第一波中。我们选择了这个小组，因为您会在真实问题上使用它，并告诉我们关于它的真实情况。

要求：本周在至少一个真实任务上使用它，然后在 #claude-code-pilot 中留下一条说明，涵盖什么有效、什么令人烦恼以及什么让您感到惊讶。该反馈决定了我们如何向其他人推出。

[继续标准公告中的"在两分钟内开始运行"]

试点的一个额外事项：在您的第一个多文件更改时，按 Shift+Tab 直到您看到"plan"。Claude 将在不编辑您源代码的情况下列出它打算做什么。这是校准您应该信任多少的最快方式。
```

### 冠军招募直接消息

推出后，直接消息给在 `#claude-code` 中最活跃的两三个人。

```text theme={null}
嘿 [名称]，您的 #claude-code 帖子对采用的推动比我的公告做得更多。几个人告诉我您的 [线程 / 截图] 是他们实际尝试它的原因。

想让这成为半官方的吗？低投入：主要是继续发布您正在发布的内容，加上新功能的第一次尝试和与 Anthropic 团队的直接联系。如果您有兴趣，我可以分享一个简短的剧本。
```

## 技巧和窍门营销活动

设计用于在推出后推动功能激活的即用型 Slack 或 Teams 消息。每个都遵循相同的模式：一个钩子、收益、一个"现在尝试"提示和一个文档链接。每周在 `#claude-code` 中滴灌一个或两个，或选择与您团队差距相匹配的少数几个。它们独立存在，没有必需的顺序。

### 开始

**选择正确的模型**

```markdown theme={null}
🎯 *技巧：将模型与时刻相匹配*

使用 Opus 修复打字错误会浪费计算。使用 Haiku 进行 12 文件重构
是在要求重做。

Claude Code 在与 Claude 应用相同的模型上运行，您可以在会话中间切换。*Sonnet* 是日常功能工作、错误、测试和审查的主力默认值。在大型重构、复杂调试或任何高风险的事情上使用 *Opus*。对于快速问题、格式化和速度获胜的机械编辑，降低到 *Haiku*。*Fable* 是您最困难、最长时间运行任务的最强大模型；它不是默认值，所以使用 `/model fable` 选择它，请注意网络安全和生物学内容会自动回退到 Opus。Opus 5.5、Sonnet 5.5 和 Opus 5 运行自己的检查：标记的内容会切换到同一系列中的较早模型，除了 Opus 5 或 Sonnet 5.5 上标记的生物学内容被拒绝。

*现在尝试：* 输入 `/model` 并选择 Sonnet（如果您还没有的话）。它是大多数任务的正确默认值。

📖 Model configuration → https://code.claude.com/docs/zh-CN/model-config
```

| 模型 | 最适合 |
| - | - |
| Fable | 最困难、最长时间运行的任务。仅选择加入：使用 `/model fable` 选择它。网络安全或生物学内容触发[自动模型回退到 Opus](https://code.claude.com/docs/zh-CN/model-config.md#automatic-model-fallback) |
| Opus | 大规模重构、复杂调试、架构决策、高风险更改。在 Opus 5.5 和 Opus 5 上，网络安全或生物学内容触发[自动模型回退或拒绝](https://code.claude.com/docs/zh-CN/model-config.md#automatic-model-fallback) |
| Sonnet | 日常功能工作、错误修复、测试、文档、代码审查。推荐默认值。在 Sonnet 5.5 上，网络安全或生物学内容触发[自动模型回退或拒绝](https://code.claude.com/docs/zh-CN/model-config.md#automatic-model-fallback) |
| Haiku | 快速问题、格式化、机械编辑、快速迭代 |

**快速赢得尝试首先**

```markdown theme={null}
🚀 *技巧：在您的前 10 分钟尝试的三件事*

安装了 Claude Code 但不确定实际要求什么？从一直困扰您整周的东西开始。

  - 修复令人烦恼的东西："文件 [file] 中的测试不稳定，找出原因"
  - 在您没有写的代码中定向："向我介绍 [module] 如何工作"
  - 在您推送前进行理智检查："查看我的工作差异并告诉我什么看起来有风险"

这些都不需要设置。只需 `cd` 进入您的仓库并运行 `claude`。

*现在尝试：* 选择您一直在避免的错误并粘贴错误消息。

📖 Quickstart → https://code.claude.com/docs/zh-CN/quickstart
```

### 项目记忆

**`/init` 和 CLAUDE.md**

```markdown theme={null}
📁 *技巧：停止每个会话重新解释您的仓库*

第五次告诉 Claude "我们使用 pnpm，而不是 npm"？有一个一次性修复。

每个仓库运行一次 `/init`。Claude 读取您的项目结构并写入一个 CLAUDE.md 文件，其中包含您的构建命令、架构和约定。该仓库中的每个未来会话都会自动从此文件开始。保持在两个屏幕以下。这是一个速查表，不是文档。

*现在尝试：* 打开您的主仓库，运行 `claude`，输入 `/init`。三十秒，在之后的每个会话中都有回报。

📖 CLAUDE.md and project memory → https://code.claude.com/docs/zh-CN/memory
```

**@-引用**

```markdown theme={null}
📎 *技巧：停止将文件内容粘贴到聊天中*

将一个组件的 200 行复制到您的提示中，以便 Claude 可以"看到"它？您不必这样做。

输入 `@` 然后是文件路径。Claude 直接将文件拉入上下文。也适用于整个目录。

> @src/components/Button.tsx 中的样式看起来不对，检查 @docs/design-system.md

*现在尝试：* 输入 `@` 然后 Tab。自动完成显示您可以到达的每个文件。

📖 Referencing files → https://code.claude.com/docs/zh-CN/common-workflows
```

### 控制和安全

**权限模式**

```markdown theme={null}
🛡️ *技巧：一个按键在"看但不要触及"和"就做吧"之间*

有时您希望 Claude 在每次编辑之前先询问。有时您只是希望它直接交付。您不应该永远只选择一种。

*Shift+Tab* 循环切换 Claude 无需询问即可执行的操作范围：*Manual*（`default` 设置值）在文件编辑和大多数 shell 命令之前询问，*acceptEdits* 让文件编辑和常见文件系统命令直接通过，同时仍在其他 shell 命令之前检查，*plan* 进行研究并提出更改建议，而不编辑您的源代码。计划模式是建立信任的方式，所以对于任何触及多个文件的工作，从那里开始。

*现在尝试：* 在您的下一个重构上，按 Shift+Tab 直到您看到"plan"，然后描述更改。您将获得一份完整的提议供您审查。

📖 Permission modes → https://code.claude.com/docs/zh-CN/permissions
```

**Checkpointing 和 `/rewind`**

```markdown theme={null}
⏪ *技巧：整个对话有一个撤销按钮*

Claude 三轮前走错了路，现在您在解开它？您不必向前修复。

`/rewind` 回滚到对话中的较早点，包括 Claude 沿途所做的文件更改。Checkpointing 是自动的；您不需要设置任何东西。

*现在尝试：* 按 *Esc* 两次打开倒带菜单，或输入 `/rewind`。选择事情变得不对劲之前的点。

📖 Checkpointing → https://code.claude.com/docs/zh-CN/checkpointing
```

### 连接您的工具

**MCP 连接器**

```markdown theme={null}
🔌 *技巧：让 Claude 读取您的问题跟踪器，这样您就不必粘贴票证*

将 Jira 票证复制粘贴到终端感觉像是向后退一步。确实是。

一个配置文件（您的项目根目录中的 `.mcp.json`）将 Claude 连接到 GitHub、Jira、Linear 或您使用的任何跟踪器。然后"分配给我的最高优先级问题是什么？"和"继续修复它"在同一对话中发生。

*现在尝试：* 问 Claude "在这个仓库中为 [GitHub/Jira/Linear] 设置一个 MCP 连接器"。它将为您写配置。

📖 MCP connectors → https://code.claude.com/docs/zh-CN/mcp
```

### 自动化您的工作流

**Skills**

```markdown theme={null}
⚡ *技巧：将您一直重新输入的提示变成命令*

本周三次输入"从 git log 总结我今天所做的工作，为站立会议格式化"？那是一个等待发生的斜杠命令。

`.claude/skills/<name>/` 中的 SKILL.md 文件变成可重用的提示；输入 `/name` 来运行它。第二次输入您之前输入过的多步骤提示时制作一个。最简单的路径：要求 Claude 为您制作它。

*现在尝试：* 输入"为我制作一个 /standup skill，从 git log 总结我今天所做的工作"，然后明天早上运行 `/standup`。

📖 Skills → https://code.claude.com/docs/zh-CN/skills
```

**Hooks**

```markdown theme={null}
🔔 *技巧：当您的重构完成时获得通知*

坐在您的办公桌前看 Claude 完成一个长任务？您在接下来的八分钟内有更好的事情要做。

Hooks 是在 Claude Code 事件上触发的 shell 命令。一个发送桌面通知的 Stop hook 意味着您可以启动一个长重构、走开，并在完成的那一刻获得通知。

*现在尝试：* 问 Claude "添加一个 Stop hook，当您完成时发送桌面通知"。它将写脚本并连接它。

📖 Hooks guide → https://code.claude.com/docs/zh-CN/hooks-guide
```

### 日常开发

**截图和图像**

```markdown theme={null}
📸 *技巧：停止描述错误对话框。只需显示它。*

输入"有一个红色框说关于空引用的东西，它指向第 47 行左右"？截图它。

直接将截图拖到终端中，Claude 看到它：错误对话框、UI 模型、白板照片、Figma 导出。*Ctrl+V* 从剪贴板粘贴，或在 Windows 和 WSL 上使用 *Alt+V*。

*现在尝试：* 下次视觉上出现问题时，截图并直接粘贴到提示中。然后只需输入"这里出了什么问题？"

📖 Working with images → https://code.claude.com/docs/zh-CN/common-workflows
```

**Git 工作流**

```markdown theme={null}
🌿 *技巧：交接整个 git 仪式*

修复花了 5 分钟。提交消息、分支和 PR 描述花了 15 分钟。这个比例是错误的。

Claude 处理完整的 git 流：带有常规消息的提交、分支、带有适当摘要的 PR。一个要求："修复偏差一，用常规提交消息提交，并打开一个 PR。"审查别人的工作？粘贴 PR URL 并要求 Claude 向您介绍差异。

*现在尝试：* 在您的下一个修复后，而不是切换到您的 git 客户端，只需输入"用一个好消息提交这个并打开一个 PR"。

📖 Creating pull requests → https://code.claude.com/docs/zh-CN/common-workflows
```

### 分享和扩展

**Plugins**

```markdown theme={null}
📦 *技巧：有人可能已经构建了那个 skill*

即将花一个小时构建一个 `/deploy` 命令？检查它是否已经存在。

Skills 被捆绑并作为插件共享。`/plugin` 浏览可用的内容并在一个步骤中安装。五分钟的浏览可以节省一小时的构建。

*现在尝试：* 输入 `/plugin` 并滚动浏览。您会找到至少一件您不知道自己想要的东西。

📖 Plugins → https://code.claude.com/docs/zh-CN/plugins
```

### 安全和管理

**安全架构**

```markdown theme={null}
🔐 *技巧：下次被问到时"这安全吗？"的答案*

您团队中的某个人会问"等等，我的代码去哪里了？"
这是您可以粘贴的简短版本。

权限模式决定 Claude 无需先询问您即可执行哪些操作。CLI 在您的终端中运行，直接与 Anthropic 的 API 通信，并支持对 shell 命令进行可选的操作系统级沙箱隔离。在 Team 或 Enterprise 计划上，Anthropic 不使用您的代码或提示词来训练其模型。

*现在尝试：* 保存这两个链接以备下次问题出现。它们回答了大多数安全审查问题。

📖 https://code.claude.com/docs/zh-CN/security
📖 https://code.claude.com/docs/zh-CN/data-usage
```

**最佳实践**

```markdown theme={null}
✅ *技巧：分离"尝试一次"和"每天使用"的 4 个习惯*

大多数从 Claude Code 反弹的人跳过了其中之一。大多数坚持的人在第一周做了全部四个。

  - 对于任何触及多个文件的东西，从 plan 模式开始
  - 早期运行 /init；上下文复合
  - 在提交前审查差异；Claude 可以自信地错误
  - 验证触及关键路径的更改；将其视为锐利的初级，而不是预言家

*现在尝试：* 如果您只做了其中一两个，选择您缺少的那个并在您的下一个任务上做。在 #claude-code 中发布什么改变了。

📖 Best practices → https://code.claude.com/docs/zh-CN/best-practices
```

## 快速参考

### 常见问题解答回复

针对您最常被问到的问题的单行回复。

| 问题 | 回复 |
| - | - |
| "它在 VS Code 中工作吗？" | 是的。有一个 VS Code 扩展和一个 JetBrains 插件，具有相同的功能，嵌入在您的编辑器中。[VS Code →](https://code.claude.com/docs/zh-CN/vs-code.md) |
| "我必须先配置什么吗？" | 不。安装，然后在任何仓库中运行 `claude`。运行一次 `/init`，您就设置好了。[快速入门 →](https://code.claude.com/docs/zh-CN/quickstart.md) |
| "我的代码去哪里了？" | CLI 在您的终端中运行，并将上下文发送到 Anthropic 的 API 进行推理。在 Team 或 Enterprise 计划上，您的代码和提示词不会用于训练模型。[数据使用 →](https://code.claude.com/docs/zh-CN/data-usage.md) |
| "它能看到我的整个仓库吗？" | 它读取您授予其访问权限的内容。您工作目录内的文件读取不会弹出提示。[权限 →](https://code.claude.com/docs/zh-CN/permissions.md) |
| "这与 Copilot 有什么不同？" | Copilot 自动完成行。Claude Code 是一个读取文件、运行命令和进行多文件编辑的代理。[概述 →](https://code.claude.com/docs/zh-CN/overview.md) |
| "我应该首先尝试什么？" | 您一直在推迟的错误，因为它很乏味。"文件 \[file] 中的测试不稳定，找出原因。" [快速入门 →](https://code.claude.com/docs/zh-CN/quickstart.md) |

### 提示模板

与已安装但不确定要求什么的工程师分享这些入门提示。每一个都以它在真实会话中输入的方式表述；用您自己仓库中的文件替换括号部分。

| 任务 | 提示 |
| - | - |
| 修复错误 | "文件 \[file] 中的测试失败，找出原因并修复它" |
| 理解代码 | "向我介绍 \[module] 如何工作，然后告诉我入口点在哪里" |
| 安全重构 | "重构 \[module] 到 \[goal]，使用 plan 模式，以便我可以先审查" |
| 编写测试 | "为 \[file] 编写测试，涵盖 \[scenario] 周围的边界情况" |
| 提交前审查 | "查看我的工作差异并告诉我什么看起来有风险" |
| 打开 PR | "修复 \[issue]，写一个常规提交，并用摘要打开一个 PR" |
| 制作 skill | "为我制作一个 /ship skill，在提交前运行测试和 lint" |
| 调试堆栈跟踪 | "这是堆栈跟踪，找到根本原因，不要只是掩盖它" |

> [!TIP]
>
> Claude Code 频繁发货。在内部分发前，根据[文档主页](https://code.claude.com/docs/zh-CN/overview.md)验证版本特定的详情。
