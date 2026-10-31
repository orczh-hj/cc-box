> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# 提示词库

> 复制粘贴提示词到 Claude Code，按任务和角色标记。

这是一个提示词库，可以复制到 Claude Code 中使用。使用它来探索你还没有尝试过的工作方式，或者当你不确定从哪里开始时。

这些提示词来自各种 Anthropic 指南，包括[常见工作流](https://code.claude.com/docs/zh-CN/common-workflows.md)、[最佳实践](https://code.claude.com/docs/zh-CN/best-practices.md)和[Anthropic 团队如何使用 Claude Code](https://claude.com/blog/how-anthropic-teams-use-claude-code)。它们是起点而不是脚本。打开任何提示词下的**为什么这样做有效**来查看其背后的模式，这样你可以编写自己的提示词。

## 这些提示词为什么有效

上面的提示词共享一些模式。识别它们有助于你将此处的任何提示词调整到你自己的任务。

**描述结果，而不是步骤。** 说出你想要的内容，让 Claude 找到文件。下面的提示词无需命名单个文件路径即可工作。

```text wrap theme={null}
add rate limiting to the public API and make sure existing tests still pass
```

**给它一种检查自己工作的方式。** 在同一提示词中要求运行、测试、比较或验证，以便 Claude 迭代而不是在一次尝试后停止。要检查完成的更改与运行中的应用程序，请运行 [`/verify`](https://code.claude.com/docs/zh-CN/skills#run-and-verify-your-app)。

```text wrap theme={null}
write the migration, run it against the dev database, and confirm the schema matches
```

**指向参考。** 命名现有文件、测试或模式以匹配，以便新代码与你已有的内容一致。

```text wrap theme={null}
add a settings page that follows the same layout as the profile page
```

**说明可测量的目标。** 当目标是性能或覆盖率时，给出指标和阈值，以便完成是明确的。

```text wrap theme={null}
get the bundle size under 200KB and show me what you removed
```

**给它工件。** 直接在提示词中粘贴错误、日志、屏幕截图和计划输出，或键入 `@` 来引用文件。Claude 读取源而不是你对它的描述。

```text wrap theme={null}
why is the build failing? @build.log
```

**说出你想要答案的方式。** 命名格式、长度或受众，以便解释适合你将如何使用它。要使格式成为每个响应的默认值，请设置 [输出样式](https://code.claude.com/docs/zh-CN/output-styles.md)。

```text wrap theme={null}
explain how the payment retry logic works as an HTML page with a diagram, then open it in my browser
```

有关每个模式的更多信息，请参阅[最佳实践](https://code.claude.com/docs/zh-CN/best-practices.md)。

## 这些来自哪里

这些提示词基于已发布的 Anthropic 资源中的模式。每张卡片都链接到其来源：

* [常见工作流](https://code.claude.com/docs/zh-CN/common-workflows.md)：核心任务的分步指南
* [最佳实践](https://code.claude.com/docs/zh-CN/best-practices.md)：提示词模式和项目设置
* [Anthropic 团队如何使用 Claude Code](https://claude.com/blog/how-anthropic-teams-use-claude-code)：来自工程、产品、设计和数据团队的真实工作流，深入探讨[法律](https://claude.com/blog/how-anthropic-uses-claude-legal)、[营销](https://claude.com/blog/how-anthropic-uses-claude-marketing)和[网络安全](https://claude.com/blog/how-anthropic-uses-claude-cybersecurity)
* [扩展代理编码指南](https://resources.anthropic.com/hubfs/Scaling%20agentic%20coding%20across%20your%20organization.pdf)：企业采用指南

有关这些模式的视频演练，请参阅 [Claude Academy](https://academy.claude.com/) 上的免费 [Claude Code in Action](https://academy.claude.com/courses/claude-code-in-action) 课程。

## 相关资源

此页面上的提示词是起点。一旦一个对你的项目有效，下一步是使其可重复：将其保存为 [技能](https://code.claude.com/docs/zh-CN/skills.md)，以便团队中的任何人都可以将其作为 `/command` 运行，并在 [CLAUDE.md](https://code.claude.com/docs/zh-CN/memory.md) 中记录 Claude 学到的约定，以便每个会话都以该上下文开始，而不是 Claude 重新学习它。对于更大或更危险的更改，[Plan Mode](https://code.claude.com/docs/zh-CN/permission-modes.md#analyze-before-you-edit-with-plan-mode) 在任何编辑发生前显示文件列表。

如果你在团队中引入 Claude Code，请参阅[管理](https://code.claude.com/docs/zh-CN/admin-setup.md)以获取托管设置和策略，以及[成本和使用](https://code.claude.com/docs/zh-CN/costs.md)以了解此工作如何在你的计划上计费。
