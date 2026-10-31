> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Claude Code 最佳实践

> 充分发挥 Claude Code 的作用：告诉 Claude 它看不到的信息，给它一个可以运行的检查，并强制执行不允许例外的规则。

Claude Code 的设计使得在大多数情况下，您在新会话中的第一个普通提示词就已经能做对事情，无需您进行任何设置。

例如：

* 为了让命令安全运行而无需您逐一批准，新的终端和 VS Code 会话以[自动模式](https://code.claude.com/docs/zh-CN/permission-modes.md#eliminate-prompts-with-auto-mode)启动，由分类器在任何有风险的操作运行之前进行审查。
* 为了在独立工作并行进行时保持主对话的专注，Claude 会将工作委派给子代理，并且只在必要时与您确认。
* 为了让长会话持续进行而无需您管理，对话会在上下文窗口填满之前自动压缩。

本页介绍默认设置不够用时该怎么做。这种情况通常发生在您的项目拥有 Claude 无法仅从代码中看到的知识、规则或约束时：

* 只存在于某个人脑子里的部署流程
* 绝不能被读取的文件
* 必须通过才算完成的检查
* 您需要控制在其内的预算

没有任何默认设置能知道您项目的这些情况，所以在这里您做一点设置就能获得回报。

当会话没有按您的预期进行时，几乎总是以下情况之一：

* Claude 构建的东西与您设想的不同，或者它每次会话开始时都要重新发现同一个事实。它缺少无法通过阅读仓库获得的信息。请参阅[告诉 Claude 它看不到的信息](#tell-claude-what-it-cant-see)。
* Claude 说已经完成了，但其实还差一点，这通常是因为它没有办法检查自己的工作。请参阅[给 Claude 一个可以运行的检查](#give-claude-a-check-to-run)。
* Claude 做了在这个仓库中绝不应该发生的事，或者跳过了必须始终执行的步骤。仅靠提示词无法保证这两者。请参阅[强制执行不允许例外的规则](#enforce-the-rules-that-cant-have-exceptions)。
* Claude 做对了，但花费的时间、token 或权限提示超出了任务本身的价值。请参阅[更改默认设置](#change-a-default)。

[让 Claude 在无人值守时运行](#let-claude-run-without-you)将这四点应用于无人值守的运行，而[您可以不再做的事](#what-you-can-stop-doing)列出了当前模型已使其变得不再必要的建议。

> [!NOTE]
>
> 面向整个团队的设置在单独的页面上：[为您的组织设置 Claude Code](https://code.claude.com/docs/zh-CN/admin-setup.md)。

## 确定性指令与非确定性指令

您给 Claude Code 的每一条指令都属于以下之一：

1. 非确定性：您编写指令，由 Claude（模型）读取。这可以是您的提示词、`CLAUDE.md` 或 skill。然后由模型决定如何处理这些信息。它通常会遵循您的命令，当前模型也很擅长在没有太多上下文的情况下理解您的意图，但并不*保证*它会遵循您的指令。它仍然可能决定不做某些事，或者做其他事。
2. 确定性：您配置指令，由 Claude Code（harness）*必须*执行。这可以是 hook、拒绝规则或托管设置。无论模型决定做什么，Claude Code 每次都会应用它。

Claude Code 中的大多数东西都有这两种形式：

| | 非确定性（由 Claude 决定） | 确定性（由 Claude Code 强制执行） |
| :- | :- | :- |
| 做什么 | 您的提示词、`CLAUDE.md`、Claude 自行选用的 skill | hook，以及只有您才能启动的 skill |
| 能触及什么 | 自动模式的分类器 | 拒绝规则、[严格模式](https://code.claude.com/docs/zh-CN/sandboxing.md#turn-off-the-retry-with-strict-sandbox-mode)下的沙箱、托管设置 |
| 何时算完成 | 要求提供证据、`/goal`、`/code-review` | 测试、类型检查、`Stop` hook、CI |
| 花费多少 | effort 级别、Claude 自己的节奏 | 配合 `-p` 使用的 `--max-turns` 或 [`--max-budget-usd`](https://code.claude.com/docs/zh-CN/cli-reference)、您设置的[支出限额](https://code.claude.com/docs/zh-CN/costs.md) |
| 谁拥有最终决定权 | Claude 对其所做工作的总结 | 分支保护、批准合并的人 |

大多数时候，您希望由 Claude 来决定。几乎每条准则都有例外，而当前模型很擅长阅读眼前的代码并判断某条准则是否适合当前情况。有了实际代码在眼前，Claude 通常比您事先写好的规则更适合做出这个判断。

确定性的那一类适用于您想确保某些事*始终*发生（或永不发生）的情况，无论模型作何决定。这不一定是什么重大的事。自动模式已经[阻止了那些重大的事](https://code.claude.com/docs/zh-CN/permission-modes.md#what-the-classifier-blocks-by-default)，比如生产部署或强制推送覆盖他人的工作。它通常是*您的*项目特有的东西，可以要多小众就多小众：

* 永远不要编辑 `src/gen/` 中生成的客户端
* 创建文件后始终运行 license-header 脚本

您本可以把这两条写成 `CLAUDE.md` 中的一行或写在提示词里，Claude 几乎每次都会遵循。但无论您的措辞多么强烈，Claude 在某一轮次中是否遵循那一行，仍然由 Claude 决定。

> [!TIP]
>
> 如果您不断给 `CLAUDE.md` 中的某一行加强调，而 Claude 仍然遗漏它，这说明这一行应该放进 hook。反过来也成立：如果您为一些只是偏好的事情设置了 hook，请删除它们，让 Claude 来决定。

## 告诉 Claude 它看不到的信息

> [!TIP]
>
> 只写下 Claude 通过阅读仓库找不到的内容，或者它原本每次会话都要花时间重新发现的内容。

Claude 在开始每个任务时都会阅读您的代码，而当前模型在这方面*非常*擅长。它们会找到您的文件布局、约定、测试框架以及现有组件的构建方式，通常比您解释得还快。这意味着 Claude 已经知道您可能想为它写下的大部分内容。

它找不到的，是从未存在于仓库中的信息：

* 您的团队在会议上做出的决定
* 只存在于某一个人脑子里的部署流程
* 集成测试所需的环境变量

还有一些信息*确实*在仓库里，但需要花几分钟挖掘才能找到，而 Claude 每次会话开始时都必须重新挖掘一遍。

在为 Claude 写下任何东西之前，一个好的检验方法：

1. 它能通过阅读代码找到这个吗？
2. 它真的会去查找吗？

如果两个答案都是肯定的，就不用写。

### 编写具体的提示词

具体的提示词之所以有帮助，原因相同：它给了 Claude 原本没有的信息。如果您一开始就指明文件、描述症状并说明"修复"是什么样子，通常可以省去之后的一轮更正。

对于两种合理方法之间的偏好也是如此：Claude 无法从代码中读出这一点，所以如果您在意它选择哪种，请一开始就说明。

| 不要这样 | 试试这样 |
| :- | :- |
| *"修复登录错误"* | *"用户报告会话超时后登录失败。查看 src/auth/ 中的 token 刷新，编写一个能重现问题的失败测试，然后修复它"* |
| *"添加一个日历小部件"* | *"按照 HotDogWidget.php 中的模式，构建一个日历小部件，让用户可以选择月份并在年份之间翻页。不要引入新库"* |

如果您已经有 Claude 需要的东西，直接交给它，而不是去描述它：

* 文件：输入 `@` 和路径，即可在消息中[包含文件内容](https://code.claude.com/docs/zh-CN/common-workflows.md#reference-files-and-directories)。
* 图像：将设计或错误的屏幕截图粘贴到提示词中。
* 命令输出：从 shell 中通过管道传入，例如 `cat error.log | claude -p "what's going on here"`。

像 `think step by step` 或 `be thorough` 这样的短语只是普通的提示词文本，而不是关键词。控制 Claude 思考深度的主要手段是 [effort 级别](https://code.claude.com/docs/zh-CN/model-config.md#adjust-effort-level)，[更改模型和 effort](#change-the-model-and-effort) 中有相关介绍。这类措辞仍然可以在 effort 设置范围内轻微推动 Claude，但不会让它超出该设置。

有一个短语仍然有帮助，那就是 `be ambitious`。Claude 往往会保守地理解请求，构建能满足它的最小实现，因为它看不到您期望的程度。当您想要更完整的版本，处理好边界情况并包含自然的后续工作时，请明确说出来：

```text theme={null}
build the settings page from the mockup. be ambitious: empty and error states, the save flow wired up, keyboard navigation, not just the layout
```

这是在告诉 Claude 范围，而范围只有您知道，而不是告诉它要思考多深。

当您在探索而不是修复某个具体问题时，像 `what would you improve in this file?` 这样模糊的提示词完全没问题。您会在过程中多做一些更正，这就是其中的取舍。

### 先与 Claude 进行头脑风暴

当您还不确定自己想要什么时，不必在提示之前就把一切想清楚。可以让 Claude 来采访您：

```text theme={null}
I want to add rate limiting to the public API but haven't thought it through yet. Interview me until you could write the spec, then write it to SPEC.md.
```

Claude 会就您尚未确定的部分提出多项选择题：按用户还是按密钥限流、超出限制的请求如何处理、哪些不在范围内。它会一直提问，直到答案足以构成一份规范。

`SPEC.md` 生成之后，开启一个新会话并基于它进行构建，这样构建就从干净的上下文和一份书面的完成定义开始。之后在[给 Claude 一个可以运行的检查](#give-claude-a-check-to-run)中，您也会让审查者参照这个文件。

### 保持 CLAUDE.md 简短

Claude Code 会将 [`CLAUDE.md`](https://code.claude.com/docs/zh-CN/memory) 加载到每个会话中，这意味着其中每一行在*每个*任务上都会占用 Claude 的一点注意力，无论是否相关。这就是为什么这个文件只包含 Claude 无法从代码本身推断出的内容时效果最好：

* 如何在这个仓库中确认工作：确切的测试、类型检查和 lint 命令，以及如何运行单个测试文件
* 代码具有误导性的地方：随处可见但新代码不得照搬的旧模式，或者看起来没有被使用但实际在用的目录
* 不在仓库中的决定：团队选定了哪个错误辅助函数、部署如何进行、什么需要迁移审查
* 任何您已经更正过两次的事情

通过这一检验的 `CLAUDE.md` 会保持简短：

```markdown CLAUDE.md theme={null}
# Commands
- Install: pnpm install
- Run one test file: pnpm vitest run path/to/file.test.ts
- Typecheck: pnpm tsc --noEmit

# Conventions
- Use the error helpers in src/lib/errors.ts, never throw raw strings
- src/legacy/ is still live but don't copy its patterns into new code

# Gotchas
- Integration tests need DATABASE_URL set, see .env.example
```

在会话中运行 `/context` 以确认 Claude 已加载该文件。

请注意，类型检查那一行只是命令本身。Claude 需要知道这个命令，以便在工作过程中检查自己的工作。如果您还希望 Claude Code 在 Claude 完成之前*强制*要求类型检查通过，那是 [`Stop` hook](#enforce-the-check-with-a-stop-hook) 的工作。在这个文件中写 `Always typecheck before you finish` 这样的一行，大多数时候有效，但并非每次都有效。

您不必从头编写这个文件：

* `/init` 会根据您的代码库生成一个初始版本。之后的工作主要是删减行。[记忆指南](https://code.claude.com/docs/zh-CN/memory.md#write-effective-instructions)建议保持在 200 行以内。
* `/doctor` 会针对 Claude [已能从代码中推断出](https://code.claude.com/docs/zh-CN/memory.md#my-claude-md-is-too-large)的内容建议删减。
* `/doctor prompt-audit` 会标记为旧模型编写的指令、指向已不存在文件的指令，或[相互矛盾](https://code.claude.com/docs/zh-CN/memory.md#audit-your-instruction-files)的指令。

您的个人偏好应放在 `CLAUDE.local.md` 而不是 `CLAUDE.md` 中，并将该文件添加到 `.gitignore`，使其不进入仓库。

> [!NOTE]
>
> 如果 Claude 一直忽略*确实*写在 `CLAUDE.md` 中的某一行，添加 IMPORTANT 可能解决不了问题。要么是文件已经长到让这一行被淹没，要么这一行是一条不允许例外的规则，在这种情况下它应该放进 [hook](#enforce-the-rules-that-cant-have-exceptions)。

### 将规则限定到代码库的某一部分

如果某条规则只在代码库的某一部分有意义，请将它放在 [`.claude/rules/`](https://code.claude.com/docs/zh-CN/memory#path-specific-rules) 中，并加上 `paths:` 过滤器，这样它只会在 Claude 实际在那里工作时加载。例如，下面这个 `.claude/rules/api.md` 会针对 `src/api/` 下的 TypeScript 文件加载：

```markdown .claude/rules/api.md theme={null}
---
paths:
  - "src/api/**/*.ts"
---
- Validate request bodies with the zod schemas in src/api/schemas/, don't hand-roll checks
- Every new endpoint needs an entry in openapi.yaml, CI fails without it
```

### 将团队流程放进 skill

能力更强的模型懂得更多工程知识，但并不知道*您的*团队如何发布版本、您的事故处理手册写了什么，或者您希望 API 文档怎样编写。这类知识应放在 [skill](https://code.claude.com/docs/zh-CN/skills.md) 中：保存在 `.claude/skills/<name>/SKILL.md` 的流程或团队知识。

对于这类内容，skill 对上下文的占用也比 `CLAUDE.md` 更轻。对于 Claude 可以自行启动的 skill，它在每个会话中只看到名称和描述，只有在任务需要时才会[读取完整文件](https://code.claude.com/docs/zh-CN/skills.md#skill-content-lifecycle)。放在 skill 中的冗长发布流程，不会像放在 `CLAUDE.md` 中那样拖累您无关的错误修复。

> [!NOTE]
>
> `CLAUDE.md` 的规则同样适用于 skill：只写下 Claude 自己找不到的内容。

对于有副作用的工作流，比如真正发布那个版本，请添加 `disable-model-invocation: true`。这样 Claude 就[无法自行启动该 skill](https://code.claude.com/docs/zh-CN/skills.md#control-who-invokes-a-skill)，只有当您自己输入 `/release` 时它才会运行。保存为 `.claude/skills/release/SKILL.md` 后，它可能如下所示：

```markdown .claude/skills/release/SKILL.md theme={null}
---
name: release
description: Cut a new release. Bumps the version, writes the changelog entry, tags it and pushes the tag so CI can publish.
disable-model-invocation: true
---
1. Make sure you're on main with a clean tree. If not, stop and ask.
2. Run `pnpm changeset version` and commit the result.
3. Summarize the PRs merged since the last tag into CHANGELOG.md, one line each, user-facing changes only.
4. Tag `v<new version>` and push the tag. CI publishes from there, never publish from your machine.
```

### 连接 Claude 需要的工具

Claude 看不到的一些信息完全存在于其他系统中，比如您的问题跟踪器或错误仪表板。如果该系统有 CLI，那通常是最简单的接入方式。例如，安装并登录 `gh` 后，Claude 无需您做任何解释就会创建 Pull Request 并读取审查评论。对于它没见过的 CLI，让它查看 `--help` 就足够了。

对于没有 CLI 的系统，可以使用 [MCP](https://code.claude.com/docs/zh-CN/mcp.md)。您可以从 shell 中添加服务器：

```bash theme={null}
claude mcp add --transport http notion https://mcp.notion.com/mcp
```

之后该服务器会出现在会话内的 `/mcp` 下。已连接的服务器即使在您不使用时，也会将其工具名称和说明添加到上下文中，因此值得在那里[关闭您不需要的服务器](https://code.claude.com/docs/zh-CN/mcp.md#disable-a-server-without-removing-it)。

如果您使用类型化语言工作，[代码智能插件](https://code.claude.com/docs/zh-CN/plugins/code-intelligence.md)可以让 Claude 直接跳转到定义，而不必阅读许多文件去寻找它。

## 给 Claude 一个可以运行的检查

当工作看起来完成时，Claude 就会停止。如果它没有可以运行的东西来告诉它工作是否*确实*完成，那么"看起来完成"就是它唯一的依据。实际上，这会让您成为测试套件，每个错误都会留在那里，直到您碰巧注意到它。

如果您给它一个返回通过或失败的东西，Claude 就可以自己闭合这个循环。它进行更改，运行检查，如果有东西出错，它会读取输出并修复，然后再次运行检查。

> [!TIP]
>
> 给 Claude 一个可以运行并返回通过或失败的东西。一旦有了它，Claude 就可以自行继续工作，您不必盯着。

检查几乎可以是任何东西，只要 Claude 能读取结果：

* 测试文件
* 构建的退出码
* linter
* 将输出与固定数据进行 diff 对比的脚本
* 放在设计稿旁边的[浏览器屏幕截图](https://code.claude.com/docs/zh-CN/chrome.md)

如果您的仓库中已经有其中之一，那么 `CLAUDE.md` 中最有用的一行就是运行它的命令。

检查要多严格由您决定。可选方案从由 Claude 自行决定一直到由 Claude Code 强制执行：

* 在提示词中：`run the auth tests after, and keep going until they pass`。由 Claude 自行决定何时满意。这适用于任何任务，今天就能用，无需设置。
* 作为目标：[`/goal all tests in tests/auth pass and lint is clean`](https://code.claude.com/docs/zh-CN/goal) 会设置一个条件，由一个单独的模型在每个轮次之后对照对话进行检查，Claude 会一直工作直到条件满足。您在会话中很少需要亲自使用它。它对长时间的后台运行很重要，在这类运行中，Claude 有时会在仍有未完成事项时以进度总结结束一个轮次，而目标会把它送回去继续工作。
* 作为 Stop hook：[`Stop` hook](https://code.claude.com/docs/zh-CN/hooks#stop) 会在 Claude 每次尝试结束其轮次时运行您的脚本，如果脚本以退出码 2 退出，就让 Claude 回去继续工作。这一种由 Claude Code（harness）强制执行，因此在类型检查通过或达到[阻止上限](https://code.claude.com/docs/zh-CN/hooks.md#stop-input)之前，轮次不会结束。[下文有一个示例](#enforce-the-check-with-a-stop-hook)。
* 由没有编写代码的人检查：在终端中，[`/code-review`](https://code.claude.com/docs/zh-CN/code-review-local) 会在一个从未见过您对话的新子代理中审查 diff。它不会被产生代码的推理过程所左右。

无论使用哪种方案，都要向 Claude 索要*证据*：它运行的命令及其输出，或屏幕截图。单独一句"所有测试都通过了"能告诉您的信息很少，而阅读输出比自己重新运行检查更快。对于您没有盯着的会话，这也是您唯一的证据。

> [!TIP]
>
> Claude 说完成之后，您还可以在会话中运行 [`/verify`](https://code.claude.com/docs/zh-CN/skills#run-and-verify-your-app)，让 Claude 构建并运行应用，然后查看结果。这能发现测试和类型检查遗漏的那类问题。

### 对照您的规范进行审查

如果您更希望审查是对照您的计划检查工作，而不是泛泛地寻找错误，请自己编写审查提示词：

```text theme={null}
use a subagent to review the rate limiter diff against SPEC.md, check every requirement is implemented and the listed edge cases have tests, report gaps that affect correctness and skip style
```

> [!NOTE]
>
> 不要删掉那个提示词中的 `skip style` 部分。被要求找问题的审查者总会找到*一些东西*。如果您修复每一个发现，最终会得到多余的抽象层，以及针对不可能发生的情况的测试。

### 用 Stop hook 强制执行检查

类型检查规则的 `Stop` hook 版本只需在 `.claude/settings.json` 中写几行。退出码 2 是阻止停止的关键。命令打印到 stderr 的任何内容都会作为 Claude 尚不能完成的原因返回给它，这就是编译器输出被重定向到那里的原因：

```json .claude/settings.json theme={null}
{
  "hooks": {
    "Stop": [
      {
        "hooks": [
          { "type": "command", "command": "pnpm tsc --noEmit 1>&2 || exit 2" }
        ]
      }
    ]
  }
}
```

要查看效果，请让 Claude 做一个会破坏类型的更改：它不会就此结束，而是收到编译器错误并继续工作，直到 `tsc` 通过。如果该 hook [连续阻止了八次停止](https://code.claude.com/docs/zh-CN/hooks.md#stop-input)且期间没有任何工具调用，Claude Code 会覆盖下一次阻止并结束该轮次。要提高上限，请设置 [`CLAUDE_CODE_STOP_HOOK_BLOCK_CAP`](https://code.claude.com/docs/zh-CN/env-vars)。

## 强制执行不允许例外的规则

> [!TIP]
>
> 如果某件事必须每次都发生，请将其设为 hook、拒绝规则或托管设置。`CLAUDE.md` 用于准则，而 Claude 可以判定某条准则不适用。

以您现在可能写在 `CLAUDE.md` 中的三条规则为例：`Never read .env`、`Always run the formatter after editing`、`Don't touch the migrations folder`。Claude 几乎每次都会遵循它们。对于少数"几乎"还不够好的规则，请将它们从 `CLAUDE.md` 中移出，放入由 Claude Code（harness）自行应用的配置中。从那以后，结果就不再取决于模型记住了什么或决定了什么。

### 用 hook 强制执行步骤

[Hooks](https://code.claude.com/docs/zh-CN/hooks-guide.md) 会在 Claude 循环中的固定时间点运行您的某样东西：工具调用之前或之后、Claude 尝试停止时、会话开始时等等。这样东西可以是：

* shell 命令（常见情况）
* 向 HTTP 端点发送 POST 请求
* 对您某个 MCP 服务器的工具调用
* [`prompt` 或 `agent` hook](https://code.claude.com/docs/zh-CN/hooks#hook-handler-fields)，让模型来决定

`prompt` 或 `agent` hook 会让*另一个模型*来决定，所以它也不是确定性的。要真正强制执行，请使用 `command` 或 `http` hook，如果在 hook 本身崩溃或超时时也应阻止该操作，请添加 [`onFailure: "block"`](https://code.claude.com/docs/zh-CN/hooks#block-the-action-when-a-hook-fails)。例如，对于扫描提交中是否含有密钥的 hook，在扫描器宕机时，您宁愿阻止提交，也不愿让它未经扫描就通过。

下面是将格式化规则写成 `PostToolUse` hook 的样子。它在每次 `Edit` 或 `Write` 之后运行，Claude 是否记得这条规则已不再重要，因为运行它的不是 Claude：

```json .claude/settings.json theme={null}
{
  "hooks": {
    "PostToolUse": [
      {
        "matcher": "Edit|Write",
        "hooks": [
          { "type": "command", "command": "npx prettier --write --ignore-unknown \"$(jq -r '.tool_input.file_path')\"" }
        ]
      }
    ]
  }
}
```

要检验该 hook，请让 Claude 在一个由 Prettier 格式化的文件中添加一行缩进错误的代码。保存后的文件会恢复正确的缩进。

迁移规则则变成一个 [`PreToolUse` hook](https://code.claude.com/docs/zh-CN/hooks-guide#block-edits-to-protected-files)，对 `migrations/` 下的任何路径以退出码 2 退出。您不必自己编写它：`write a hook that blocks edits to the migrations folder` 作为提示词就能奏效，`/hooks` 会显示结果。不过不要过度使用：hook 是给*零*例外的规则用的，而 `Prefer named exports` 有很多例外，所以那一条应留在 `CLAUDE.md` 中。

### 用权限规则和沙箱限制访问

[权限规则](https://code.claude.com/docs/zh-CN/permissions.md)决定 Claude 可以运行、读取和获取什么。`deny` 规则在每种权限模式下都有效，包括自动模式和 `bypassPermissions`，这正是 `.env` 规则应放在这里的原因：

```json .claude/settings.json theme={null}
{
  "permissions": {
    "allow": ["Bash(pnpm tsc --noEmit)"],
    "ask": ["Bash(git push *)"],
    "deny": ["Read(./.env)", "Read(./.env.local)", "Read(./secrets/**)"]
  }
}
```

保存之后，询问 Claude `.env` 中有什么将会失败，无论是通过 `Read` 工具还是通过 `cat .env`。

规则匹配的是工具调用，而不是意图，因此值得了解其边界在哪里：

* `Read` 规则*确实*能拦截 `cat .env`，因为 Claude Code 能识别[常见的文件读取命令](https://code.claude.com/docs/zh-CN/permissions.md#read-and-edit)。
* 它无法拦截打开该文件的脚本。
* 针对某个域名的 `WebFetch` 拒绝规则对指向同一域名的 `curl` 不起作用，除非启用了沙箱。

要对文件系统或网络访问进行操作系统级的限制，请在会话中使用 `/sandbox` 启用[沙箱](https://code.claude.com/docs/zh-CN/sandboxing.md)。它会在 macOS、Linux 和 WSL2 上限制 Claude 的 shell 命令，并且还会[采纳您的 `Read` 拒绝规则和 `WebFetch(domain:...)` 规则](https://code.claude.com/docs/zh-CN/sandboxing#permission-rules)，因此对于任何与安全相关的事项，您需要两者兼用。

默认情况下，Claude 仍然可以请求在沙箱之外重试被阻止的命令，该请求会经过正常的权限流程。`/sandbox` 的 **Overrides** 选项卡中的[严格沙箱模式](https://code.claude.com/docs/zh-CN/sandboxing.md#turn-off-the-retry-with-strict-sandbox-mode)会关闭这种重试。

> [!WARNING]
>
> 自动模式是一个对每个操作进行判断的分类器模型，而不是硬性规则。它很擅长这件事，但和任何判断一样，它可能出错。您还可以调整它认定为有风险的内容，您的组织也可以完全关闭它，所以您的"绝不"规则应放在拒绝规则或 hook 中。

### 选择规则适用于谁

规则放在哪里，决定了它适用于谁：

* `.claude/settings.local.json` 和 `~/.claude/settings.json` 只属于您自己。当您为某个命令选择 **Yes, and don't ask again** 时，Claude Code 会将该规则保存到本地文件中。
* `.claude/settings.json` 及其中的 hook 属于仓库。一旦您提交它们，它们就会适用于在该仓库中工作的每个人。其中的拒绝规则立即生效。其中的 `allow` 规则要等到每个人接受该文件夹的[信任对话框](https://code.claude.com/docs/zh-CN/permissions.md#what-runs-before-you-trust-a-folder)后才生效，[工作区信任](https://code.claude.com/docs/zh-CN/hooks.md#workspace-trust)也决定了其中的 hook 何时开始在他们的机器上运行。
* [托管设置](https://code.claude.com/docs/zh-CN/managed-settings.md)属于组织。它们来自管理控制台、MDM 或 `managed-settings.json` 文件，任何个人设置文件都无法覆盖它们，这使它们成为安全团队策略的唯一归宿。如果您是负责设置它们的人，请参阅[为您的组织设置 Claude Code](https://code.claude.com/docs/zh-CN/admin-setup.md)。

无论作用域如何，都要让强制执行的规则集保持精简。每个 hook 和拒绝规则偶尔都会在不该触发时触发，而且每一个都需要有人维护。如果您发现自己已经有四十条，其中大多数可能只是 Claude 本可以自行权衡的偏好。

## 更改默认设置

> [!TIP]
>
> 默认设置已经足够好，可以保持不动，直到某个任务花费的时间、token 或中断超出应有的程度。发生这种情况时，在会话中运行 `/usage` 查看消耗在哪里。

新会话已经为您做了几项决定：

* 交互式终端和 VS Code 会话以自动模式启动。
* 在大多数套餐和提供商上，它运行 [medium effort 的 Claude Opus 5.5](https://code.claude.com/docs/zh-CN/model-config.md#default-model-setting)。
* 当它认为有理由时，会将工作委派给子代理。
* 随着对话增长，它会压缩对话。

大多数时候您根本不会想到这些。当某个任务确实花费过多时，原因通常是这四项之一。

### 更改权限模式

权限范围的两端都有代价：

* **手动模式**：您需要亲自批准每次编辑和每个非只读命令。安全，但到第十次提示之后，您就开始不看内容直接批准了，而且当您离开键盘时，一切都会停滞。
* **`bypassPermissions`**：批准所有未被您的拒绝规则和询问规则覆盖的操作，这只在一次性容器中才有意义。

[自动模式](https://code.claude.com/docs/zh-CN/permission-modes.md#eliminate-prompts-with-auto-mode)介于两者之间，也是交互式终端和 VS Code 会话[启动时使用的模式](https://code.claude.com/docs/zh-CN/permission-modes.md#which-mode-a-session-starts-in)。Claude 在工作时无需为常规批准而停下，一个单独的分类器模型会在操作运行前按[固定顺序](https://code.claude.com/docs/zh-CN/permission-modes.md#how-the-classifier-evaluates-actions)进行检查。它专注于[您希望有人出面阻止的事情](https://code.claude.com/docs/zh-CN/permission-modes.md#what-the-classifier-blocks-by-default)：

* `curl | bash`
* 生产部署和迁移
* 强制推送覆盖他人的工作
* 将敏感数据发送到外部端点
* 看起来是由 Claude 读取的内容而不是由您驱动的操作

当它确实阻止了某项操作时，您会收到通知，该操作会出现在 `/permissions` 的 **Recently denied** 下，您可以在那里通过手动批准重试它。在[连续三次阻止](https://code.claude.com/docs/zh-CN/permission-modes.md#when-auto-mode-falls-back)（或总计二十次）之后，自动模式会暂停并恢复为向您询问。

这些都不会取代您的确定性控制，因为它在它们*之后*运行。拒绝规则、询问规则和 `PreToolUse` hook 会在分类器看到任何内容之前生效，因此拒绝规则在自动模式中的阻止效果与在其他任何地方完全相同。沙箱仍然会限制[它所包装的 shell 命令](https://code.claude.com/docs/zh-CN/sandboxing.md#what-runs-outside-the-sandbox)。

> [!NOTE]
>
> 您也可以在对话中说 `don't push until I've reviewed`，分类器会[将其视为阻止信号](https://code.claude.com/docs/zh-CN/permission-modes.md#boundaries-you-state-in-conversation)。但这仍然是一个模型在阅读会话记录，而且这句话可能在对话压缩时丢失，所以任何重要的事项都应放在拒绝规则中。

相反的问题是，分类器反复阻止在您的环境中属于常规的操作。与其每次都手动批准，不如将其添加到 `autoMode.allow`，或在[自动模式配置](https://code.claude.com/docs/zh-CN/auto-mode-config.md)中描述您信任的基础设施。它使用的是普通英语而不是模式，[`/auto-mode-setup`](https://code.claude.com/docs/zh-CN/auto-mode-config#generate-environment-entries) 可以根据您的项目和近期会话为您起草 `environment` 部分：

```json ~/.claude/settings.json theme={null}
{
  "autoMode": {
    "environment": [
      "$defaults",
      "Source control: github.com/example-org and every repo under it",
      "Trusted internal domains: *.internal.example.com"
    ],
    "allow": [
      "$defaults",
      "Deploying to the staging namespace is fine, it's isolated from production and resets nightly"
    ]
  }
}
```

> [!WARNING]
>
> 在您设置的每个列表中都保留 `"$defaults"`，否则您的条目会*替换*该列表的内置规则，而不是添加到其中。对于 `environment` 和 `allow`，您会失去内置的上下文和例外；而缺少它的 `soft_deny` 列表会丢掉对强制推送和生产部署的内置阻止。
>
> `autoMode` 块还必须放在 `~/.claude/settings.json` 中，而不是项目中，因为分类器会忽略项目设置中的 `autoMode`。

如果您更希望在每次编辑和命令发生之前都能看到它们，可以在会话中按 `Shift+Tab` 在各权限模式（包括手动模式）之间循环切换，或者从 shell 中使用 `claude --permission-mode manual` 启动。如果手动模式随后询问过于频繁，`/fewer-permission-prompts` 会扫描您近期的会话记录，找出常见的只读 Bash 和 MCP 调用，并将它们添加到 `.claude/settings.json` 中的允许列表。

### 更改模型和 effort

[Effort](https://code.claude.com/docs/zh-CN/model-config.md#adjust-effort-level) 是控制 Claude 思考时长的主要手段，它取代了人们过去通过提示词措辞所做的大部分工作。Claude Opus 5.5、Sonnet 5.5 和 Haiku 5.5 默认为 `medium`，这对于范围明确的日常功能开发已经足够。您会在会话中使用 `/effort` 调整它的情况：

* 当做对事情取决于容易被忽略的情况时，使用 `/effort high` 或 `/effort xhigh` 调高。比如您没写过的代码中的错误、不稳定的测试，或者 Claude 必须找到旧 schema 的每一个读取方的 schema 变更。Claude 随后会在回答前检查更多边界情况并验证更多工作，额外的时间和 token 就花在这里。
* 当您快速来回交流并且无论如何都会审查每个结果时，使用 `/effort low` 调低：头脑风暴、初步草图、跨几个文件的重命名。Claude 会更快地给您一个起点，而且由于您在引导每一步，额外的深思熟虑除了让您多等之外不会改变太多。
* 将 `max` 留给您希望 Claude 在没有您参与的情况下攻克的难题，比如寻找安全漏洞。在普通任务上它往往会过度思考，所以您付出更多，得到的答案却可能并不比 `high` 更好。
* 当只有下一条消息需要它时，比如在一系列常规工作中间出现一个棘手的问题，请在那条提示词中加入 [`ultrathink`](https://code.claude.com/docs/zh-CN/model-config#use-ultrathink-for-one-off-deep-reasoning)，而不是更改设置。

至于模型本身，默认的 Opus 5.5 能很好地处理大多数编码工作，而 Sonnet 在常规工作上成本更低。当您通过 `/model fable` 明确选择时，可以使用 [Claude Fable 5.1](https://code.claude.com/docs/zh-CN/model-config.md#work-with-fable)，而 Haiku 很适合执行简单任务的子代理。[模型配置](https://code.claude.com/docs/zh-CN/model-config.md)列出了各套餐和提供商的完整列表。

### 了解子代理的成本

Claude 会自行委派给[子代理](https://code.claude.com/docs/zh-CN/sub-agents.md)：

* 广泛的代码库搜索交给内置的 Explore Agent。
* 在 fork 模式开启时（默认），附带任务会被[分叉](https://code.claude.com/docs/zh-CN/sub-agents.md#fork-the-current-conversation)。
* 它启动的子代理默认在后台运行，因此您的主对话可以继续推进。

您很少需要主动要求这些，但了解您得到的是什么以及代价是什么会有所帮助。

子代理在以下情况下物有所值：

* 同时运行相互独立的工作，让整个任务更快完成
* 将长输出（比如完整的测试运行或大范围搜索）排除在主对话之外，否则它会在会话的剩余部分一直伴随着您
* 获得一个没有看到代码编写过程的审查者
* 给某个任务提供比主会话更少的工具，比如一个能读取但不能编辑的审查者

它们的代价主要是 token，因为每个子代理都会发送自己的请求，并且与您的主对话计入相同的用量。有时还有时间成本：新的子代理必须先收集您的对话中已有的上下文，因此它可能比在当前会话中直接完成任务*更慢*。

而且默认情况下，它是同一个模型（或更小的模型）处理同一个任务。您从中获得的是并行性或一双新的眼睛，而不是更聪明的答案。

要在子代理上花得更少：

* 如果子代理的工作很简单，比如收集导入某个模块的文件或检查文档中的链接，请在其定义中设置 [`model: haiku`](https://code.claude.com/docs/zh-CN/sub-agents#choose-a-model)。这类工作不需要更大的模型，而 Haiku 每次请求的成本要低得多。
* 要求返回简短的总结，这样二十个结果就不会反而填满您的上下文。
* 查看 `/usage`，了解每种子代理类型占您用量的多少，从而判断哪些物有所值。
* 给 `.claude/agents/` 中的自定义子代理只配置 `tools: Read, Grep, Glob`。这样它就是一个无法编辑任何内容的审查者，而且这一限制由 Claude Code 强制执行，而不是在提示词中请求。

这个审查者只是一个简短的文件，`.claude/agents/reviewer.md`：

```markdown .claude/agents/reviewer.md theme={null}
---
name: reviewer
description: Reviews changed files against the spec they were built from. Use after implementing a feature, before opening the PR.
tools: Read, Grep, Glob
---
You're reviewing code you didn't write. Compare the changed files against the spec you're given,
check that every requirement is implemented and the listed edge cases have tests,
and report only the gaps that affect correctness. Skip style.
```

让 Claude `have the reviewer compare the rate limiter against SPEC.md`，发现的缺口就会回到您的会话中，由 Claude 来修复。

### 控制成本和上下文

由于大多数当前模型拥有 1M token 的窗口，并且在接近上限时进行压缩，您很少会用完上下文。不过，您仍然可能为不需要的上下文*付费*。每个请求都会发送整个对话，因此一个累积了三个无关任务的会话，在处理第四个任务的每个轮次时都会重新读取前三个。在[缓存有效期间会有折扣](https://code.claude.com/docs/zh-CN/costs.md#why-usage-climbs-in-a-long-session)，而缓存过期后则按全价计费。

[`/usage`](https://code.claude.com/docs/zh-CN/costs) 是首先要查看的地方。在 Pro、Max、Ultra、Team 和 Enterprise 上，它会按 skill、子代理、插件和 MCP 服务器细分您的用量。当长上下文或频繁缓存未命中等模式之一超过您用量的 10% 时，它也会标记出来。常见的解决方法有：

* 在不相关的任务之间使用 `/clear`。它不花任何成本，并且意味着下一个任务不必继续为上一个任务的历史付费。`/clear oauth-migration` 会为您离开的会话命名，以便您之后回到它。
* 带焦点地使用 `/compact`，比如 `/compact keep the API changes and the test commands`。在长会话中，`/compact` 本身就是一个大请求，因为它要读取整个对话来进行总结，缓存过期后更是如此。
* 留意后台工作：`/loop`、目标在每个轮次之后的检查，以及 Claude 自己的子代理，都会在您没有输入时持续发送请求。

> [!NOTE]
>
> [提示缓存](https://code.claude.com/docs/zh-CN/prompt-caching.md)在订阅下持续约一小时，在 API 密钥、云提供商或使用额度下为五分钟。除非您更改，否则子代理为五分钟。
>
> 在一小时缓存下，超过约 200K token 的空闲会话会在缓存过期前不久被压缩，因此午饭回来后您会从一份总结继续。如果您完全不需要这些历史，开启新会话的成本仍然更低。

## 引导会话

> [!TIP]
>
> 留意第一分钟，一旦 Claude 偏离方向就按 `Esc`，在两次更正失败后运行 `/clear` 并写一个更好的提示词，而不是进行第三次更正。

再多的配置也无法取代留意第一分钟，并在 Claude 朝您不想要的方向前进时介入。用于此目的的控制手段只有几个，值得熟记于心：

* `Esc` 会在 Claude 执行过程中停止它，并保留到那时为止的所有内容，因此您可以重新引导而不丢失工作。
* `/rewind`，或在空提示词上按两次 `Esc`，会打开 [rewind 菜单](https://code.claude.com/docs/zh-CN/checkpointing.md#rewind-and-summarize)，您可以在其中将代码、对话或两者回滚到更早的提示词。这让先尝试更大胆的方法、行不通再回滚变得风险很低。当对话的某部分已经过时，同一个菜单还可以对其进行总结。
* `/clear` 会重新开始。如果您已经两次更正同一件事而它仍然是错的，第三次更正多半只是往一个已经包含两次失败尝试的对话中添加内容，而 Claude 会不断重读这些内容。`/clear` 加上一个融合了您所学内容的新提示词，几乎每次效果都更好。

> [!WARNING]
>
> 检查点涵盖的是 Claude 在主对话中使用其自身文件工具编辑的文件，因此只被 Bash 命令改动过的文件[不会被恢复](https://code.claude.com/docs/zh-CN/checkpointing.md#bash-command-changes-not-tracked)。它们与 git 配合使用，而不是取代 git。

还有两个较小的工具：[`/btw`](https://code.claude.com/docs/zh-CN/interactive-mode#side-questions-with-%2Fbtw) 可以回答一个快速问题，而不会向历史中添加任何内容。对于跨越数天的工作，在会话中使用 `/rename oauth-migration` 为其命名，之后就可以从 shell 中使用 [`claude --resume oauth-migration`](https://code.claude.com/docs/zh-CN/sessions#resume-by-session-id-or-name) 继续，包括检查点在内。`claude --continue` 会恢复最近的一个会话。

## 让 Claude 在无人值守时运行

> [!TIP]
>
> 在让某次运行无人值守之前，确保它知道完成是什么样子、能检查自己的工作，并且只能触及它应该触及的内容。给它设置轮次或支出上限，并在最后由一个人批准结果。

一旦您不在场去发现错误，本页上的所有内容就显得更加重要。当您在盯着时，Claude 缺少某部分上下文并不是什么大问题：您会在一两条消息内注意到并告诉它。而在您睡前启动的运行中，没有人告诉它，您第二天早上才发现它整晚都在基于错误的假设进行构建。

在让任务自行运行之前，值得针对该具体任务逐一检查以下五个问题：

1. 它知道完成是什么样子吗？一个目标条件、一个规范文件，或一个它必须让其通过的失败测试。
2. 它能检查自己的工作吗？测试套件、`Stop` hook、某种审查步骤。
3. 它被允许触及什么？权限规则、`dontAsk`、沙箱、worktree、它可以推送的那一个分支。
4. 什么会让它停下来？`--max-turns`、`--max-budget-usd`、支出限额。
5. 结果去哪里，谁来签核？一个 PR、一条评论或一份总结，然后由一个人批准合并、部署或任何其他无法撤回的操作。

一旦开始运行，Claude 会自行完成一个长任务。仍由您负责的部分是：启动它、决定这一次"完成"意味着什么，以及在最后批准合并。

### 从脚本运行 Claude

[`claude -p "prompt"`](https://code.claude.com/docs/zh-CN/headless) 运行一次后退出，Claude Code 就是这样进入 CI 作业、git 钩子和 shell 循环的。

在脚本中，您几乎总是需要再加几个标志：

* [`--bare`](https://code.claude.com/docs/zh-CN/headless#start-faster-with-bare-mode) 会跳过 hook、skill、插件、MCP 服务器和 `CLAUDE.md` 文件（包括仓库的和机器的），因此无论在哪里运行，结果都一样。它也从不触及您的 claude.ai 登录：身份验证来自 `ANTHROPIC_API_KEY`、您通过 `--settings` 传入的文件中的 `apiKeyHelper`，或您的云提供商的凭据。
* `--append-system-prompt`、`--mcp-config` 和 `--settings` 会显式传入运行*确实*需要的内容。
* `--allowedTools` 会预先批准其工具，而 [`--permission-mode dontAsk`](https://code.claude.com/docs/zh-CN/permission-modes#allow-only-pre-approved-tools-with-dontask-mode) 会拒绝其他任何原本会触发提示的操作。
* `--output-format json` 会为您提供一个可供解析的单一结果对象，而搭配 `--verbose` 的 `stream-json` 会在事件发生时流式输出。

组合起来如下所示：

```bash theme={null}
for file in $(cat files.txt); do
  claude --bare -p "Migrate $file to the new logger API and commit. Reply OK or FAIL." \
    --allowedTools "Read,Edit,Bash(git add *),Bash(git commit *)" \
    --permission-mode dontAsk \
    --max-turns 20
done
```

运行结束时，Claude 会回复 `OK` 或 `FAIL`，而达到轮次上限的运行则会以错误退出。先在三个文件上试一试，看看哪里出了问题，修正提示词，然后再运行其余部分。

如果传入 `--permission-mode auto`，自动模式也可以与 `-p` 一起使用。在非交互式运行中，分类器的阻止只会拒绝那一个操作，Claude 会继续工作，而且重复的阻止不会像在终端中那样暂停运行。在依赖它之前，请阅读[阈值在那里如何应用](https://code.claude.com/docs/zh-CN/permission-modes.md#repeated-block-thresholds)。

### 按计划或事件启动运行

一旦运行开始，Claude 就能自行处理剩下的事情。它唯一做不到的是决定*开始*，所以必须有某样东西来启动它：您、一个计时器，或者一个事件，比如 CI 检查失败或新的 PR。

在已打开的会话中，可以使用 [`/loop`](https://code.claude.com/docs/zh-CN/scheduled-tasks#run-a-prompt-repeatedly-with-%2Floop)，它会按间隔重复一个提示词：

```text theme={null}
/loop 15m check CI on this branch, and if it's red pull the failing job's log, fix it and push
```

每次运行都是同一会话中的另一个轮次，所以不要在长对话中让它一直运行。对于任何应在您的笔记本电脑合上时进行的事情，[`/schedule`](https://code.claude.com/docs/zh-CN/routines#create-from-the-cli) 会设置一个按计时器运行的[云端定时任务](https://code.claude.com/docs/zh-CN/routines.md)。它需要 claude.ai 登录，而不是 API 密钥。您可以通过询问 Claude 添加 GitHub 事件触发器，或在 [claude.ai/code/routines](https://claude.ai/code/routines) 添加 API 触发器。

> [!WARNING]
>
> 在让云端定时任务持续运行之前，请使用[该任务无法绕过的 GitHub 分支规则](https://code.claude.com/docs/zh-CN/routines.md#repositories-and-branch-permissions)保护 `main`。对于任何新的任务，在信任它无人值守运行之前，请先观察最初的几次运行。

### 并行运行会话

当您同时进行多项独立的工作时，给每项工作分配独立的检出，以免编辑相互冲突。[`claude -w feature-auth`](https://code.claude.com/docs/zh-CN/worktrees#start-claude-in-a-worktree) 会在新的 worktree 中启动会话，[桌面应用](https://code.claude.com/docs/zh-CN/desktop.md#work-in-parallel-with-sessions)可以并排运行多个这样的会话，而[云端会话](https://code.claude.com/docs/zh-CN/claude-code-on-the-web.md)会在云端而不是您的机器上运行它们。

worktree 是一个全新的检出，这意味着像 `.env` 这样被 gitignore 的文件不在其中，依赖也没有安装。对于前者，请在仓库根目录的 [`.worktreeinclude`](https://code.claude.com/docs/zh-CN/worktrees#copy-gitignored-files-into-worktrees) 中列出您需要的被 gitignore 的文件（语法与 `.gitignore` 相同），Claude Code 会将它们复制到它创建的每个 worktree 中：

```text .worktreeinclude theme={null}
.env
.env.local
config/dev-credentials.json
```

只放小文件：它是复制而不是链接，所以如果把 `node_modules` 放进去，就意味着每个 worktree 都要完整复制一份。对于大型目录，可以使用 [`worktree.symlinkDirectories`](https://code.claude.com/docs/zh-CN/settings-reference#worktree-symlinkdirectories) 设置，或者将安装命令写在 `CLAUDE.md` 中，让 Claude 每次在新检出中启动时运行它。

您很少需要自己清理这些：

* 当您退出一个未命名的 worktree 会话时，如果其中没有任何内容，Claude Code 会[删除该 worktree](https://code.claude.com/docs/zh-CN/worktrees.md#clean-up-worktrees)。对于已命名的会话或有更改的会话，它会询问如何处理。
* 子代理的 worktree 会在子代理无更改地完成时消失。
* 确实被遗留下来的子代理和后台会话 worktree，一旦超过 `cleanupPeriodDays` 且不包含任何工作，就会被清理掉。

> [!NOTE]
>
> `claude -p --worktree` 运行需要您自己清理，因为它退出时不会询问。使用 `git worktree remove <path>` 删除它们；如果 git 因 worktree 被锁定而拒绝，请先执行 `git worktree unlock <path>`。

对于跨许多文件的一次大型机械性更改，[`/batch migrate every handler to the v2 response helpers`](https://code.claude.com/docs/zh-CN/commands) 会将工作拆分为 5 到 30 个单元并向您展示计划。然后它会在各自的 worktree 中运行每个单元，并为其创建 PR。从 shell 中，[`claude agents`](https://code.claude.com/docs/zh-CN/agent-view)（目前仍为研究预览版）可以在一个屏幕上显示您机器后台运行的所有内容。

第二个会话也是您能获得的最简单的独立审查者。一个会话编写速率限制器，另一个会话在没有看到编写过程的情况下审查 `@src/middleware/rateLimiter.ts`，然后您将发现粘贴回第一个会话。这与 `/code-review` 的思路相同，只是由您来传递消息。

## 您可以不再做的事

以下每一条在写下时都是好建议。其中大多数是对模型当时还不能可靠完成的事情的变通办法，而模型现在已经赶上来了。如果其中一些仍是您的习惯，以下是可以放心放弃它们的原因。

| 旧建议 | 为何不再需要 |
| :- | :- |
| 上下文窗口是您的主要约束，要积极地管理它 | 大多数当前模型拥有 1M token 的窗口，而且压缩会自动进行，因此很少会用完。在任务之间使用 `/clear` 仍然值得，因为专注的上下文能带来更好的答案，并且成本更低。 |
| 对 Claude 一直遗漏的指令添加 IMPORTANT 或全大写 | 较旧的模型有时需要这种强调。当前模型能遵循措辞平实的一行，而堆砌强调多半会让 Claude 过度应用该规则。如果这是一条不能变通的规则，请将其改为 hook 或拒绝规则。如果这一行在长文件中被淹没，那就需要缩短文件。 |
| 使用子代理来保护您的上下文窗口 | 用完上下文已经很少成为问题。子代理在并行工作、让嘈杂的输出不碍事以及独立审查方面仍然很好用，但每一种用法都要花费额外的 token。 |
| 委派给子代理以获得更好的答案 | 把同一个任务交给子代理并不会让答案变得更好。新的审查者确实有帮助，因为它没有看到您的推理过程，但这与委派是两回事。 |
| 在提示词中写 `think hard`、`think harder`、`be thorough` | Effort 才是控制这一点的手段。这些短语只是普通的提示词文本，会在 effort 设置范围内轻微推动 Claude，而不是提升 effort 的关键词。`ultrathink` 仍然有效：它会在单个轮次中请求更深入的推理，而不改变您的 effort 设置。 |
| 告诉 Claude 继续下去、循环直到完成、不要偷懒 | 当前模型通常会在无需提醒的情况下完成已商定的任务，而这类话更可能导致过度工作，而不是解决停滞。给 Claude 一个必须通过的检查则不同，仍然值得去做。如果某次运行确实提前停止，说 `continue` 即可；对于无人盯着的运行，`/goal` 或 `Stop` hook 会强制执行这一点。 |
| 在 `CLAUDE.md` 中描述您的架构和文件布局 | Claude 会自己阅读仓库。只写下其中没有的内容。 |

如果本页上的其他建议开始让您觉得是在做无用功，那很可能说明它也应该列入这张表。

## 相关资源

* [Claude Code 如何工作](https://code.claude.com/docs/zh-CN/how-claude-code-works.md)：循环、工具以及启动时加载的内容
* [探索上下文窗口](https://code.claude.com/docs/zh-CN/context-window.md)：会话中哪些内容占用空间以及何时加载
* [扩展 Claude Code](https://code.claude.com/docs/zh-CN/features-overview.md)：在 skill、hook、MCP、子代理和插件之间进行选择
* [选择权限模式](https://code.claude.com/docs/zh-CN/permission-modes.md)和[配置权限](https://code.claude.com/docs/zh-CN/permissions.md)
* [有效管理成本](https://code.claude.com/docs/zh-CN/costs.md)
* [以编程方式运行 Claude Code](https://code.claude.com/docs/zh-CN/headless.md)和[使用云端定时任务自动化工作](https://code.claude.com/docs/zh-CN/routines.md)
* 面向团队负责人和管理员：[为您的组织设置 Claude Code](https://code.claude.com/docs/zh-CN/admin-setup.md)
