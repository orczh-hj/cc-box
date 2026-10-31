> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# 扫描代码库中的漏洞

> 安装 Claude Security plugin 以在 Claude Code 会话中扫描代码库中的漏洞，并将发现的问题转化为您可以审查和应用的补丁。

Claude Security plugin 在 Claude Code 会话中对您的代码库运行多 Agent 漏洞扫描。[默认情况下](#choose-a-scan-effort)，当您扫描整个仓库时，Claude Agent 会映射您的架构、构建威胁模型、搜寻漏洞，并在编写报告前独立审查每个发现。

使用该插件扫描整个仓库或[仅扫描一组更改](#scan-only-your-changes)，例如分支的 diff、Pull Request 的 diff 或单个提交，然后将您选择的发现转化为您自己审查和应用的补丁。

该插件在您的会话中本地运行，使用[您在 Claude Code 中可以访问的任何模型](#models-and-providers)，每次扫描都计入您的[使用量](https://code.claude.com/docs/zh-CN/costs.md)。如果您想要一个监控您的仓库的托管服务，或想要在 [Claude Mythos](https://platform.claude.com/docs/en/about-claude/models/introducing-claude-fable-5-and-claude-mythos-5) 上运行扫描，请参阅 [Claude Security](https://claude.com/product/claude-security) 产品，该产品在企业计划中提供。

该插件也不同于 Claude Code 中已有的审查工具：[security guidance plugin](https://code.claude.com/docs/zh-CN/security-guidance.md) 在 Claude 编写代码时审查代码，[`/security-review`](https://code.claude.com/docs/zh-CN/commands#all-commands) 对您的分支运行单次扫描，[Code Review](https://code.claude.com/docs/zh-CN/code-review.md) 审查拉取请求。有关这些层如何堆叠的信息，请参阅[插件如何与其他安全工具配合](#how-the-plugin-fits-with-other-security-tools)。

## 前置条件

要运行该插件，您需要：

* 付费计划、Anthropic API 访问权限或[第三方提供商](#models-and-providers)，用于扫描使用的[动态工作流](https://code.claude.com/docs/zh-CN/workflows.md)来编排其代理。在 Pro 版本上，从 `/config` 中的"Dynamic workflows"行启用它们。
* Python 3.9 或更高版本，在您的 `PATH` 中以 `python3` 的形式可用。使用 `python3 --version` 检查。该插件的工具仅使用 Python 标准库，因此无需安装任何内容。
* Linux、macOS 或 Windows。
* Git，用于变更扫描和将发现结果转换为补丁；这些任务不支持其他版本控制系统。完整扫描可在任何目录中工作，无论是否有版本控制。

## 模型和提供商

扫描在您的 Claude Code 会话中运行。该插件本身不进行模型调用，因此没有单独的 API 密钥或提供商设置需要配置。

* **模型**：搜索漏洞、验证发现、编写和审查补丁的代理在[您会话的模型](https://code.claude.com/docs/zh-CN/sub-agents.md#choose-a-model)上运行。要更改它，请在开始扫描前在您的会话中运行[`/model`](https://code.claude.com/docs/zh-CN/model-config#setting-your-model)。一些支持步骤，例如映射存储库，改为使用[`sonnet` 别名](https://code.claude.com/docs/zh-CN/model-config#model-aliases)。
* **提供商**：扫描在付费计划上运行，具有 Anthropic API 访问权限，或在[第三方提供商](https://code.claude.com/docs/zh-CN/third-party-integrations.md)上运行，例如 [Amazon Bedrock](https://code.claude.com/docs/zh-CN/amazon-bedrock.md)、[Google Cloud 的 Agent Platform](https://code.claude.com/docs/zh-CN/google-vertex-ai.md) 或 [Microsoft Foundry](https://code.claude.com/docs/zh-CN/microsoft-foundry.md)。

在第三方提供商上，`sonnet` 别名可能解析为与 Anthropic API 上不同的版本。如果您的账户无法使用该版本，请[固定您的模型版本](https://code.claude.com/docs/zh-CN/model-config.md#pin-models-for-third-party-deployments)，包括 `ANTHROPIC_DEFAULT_SONNET_MODEL`。

[自动模型回退](https://code.claude.com/docs/zh-CN/model-config.md#automatic-model-fallback)重新运行被模型的安全防护标记的请求。在 Amazon Bedrock、Google Cloud 的 Agent Platform 和 Microsoft Foundry 上，根据[您的部署设置方式](https://code.claude.com/docs/zh-CN/model-config.md#enable-fallback-on-bedrock-agent-platform-and-foundry)，请求可能以拒绝消息结束。

## 安装插件

在 VS Code 扩展或桌面应用中，请按照[安装插件](https://code.claude.com/docs/zh-CN/plugins/install.md#install-a-plugin)进行安装。在终端中，运行 `claude` 启动 Claude Code，然后在其输入框中输入以下内容，从[官方 Anthropic 市场](https://code.claude.com/docs/zh-CN/plugins/anthropic-marketplaces.md)安装：

```text theme={null}
/plugin install claude-security@claude-plugins-official
```

该命令打开插件的详细信息，您可以在其中选择[安装范围](https://code.claude.com/docs/zh-CN/plugins/install.md#install-a-plugin)来开始安装。

如果安装失败，请对照 Claude Code 报告的消息进行处理：

* `Marketplace "claude-plugins-official" not found`：使用 `/plugin marketplace add anthropics/claude-plugins-official` 添加市场，然后重试安装。
* `Plugin "claude-security" not found in marketplace "claude-plugins-official"`：您本地的市场目录副本可能已过期。使用 `/plugin marketplace update claude-plugins-official` 刷新它，然后重试安装。如果重试时输出相同的消息，请参阅[在市场中找不到插件](https://code.claude.com/docs/zh-CN/plugins/troubleshooting.md#plugin-not-found-in-marketplace)。

检查安装摘要。如果它报告 `Run /reload-plugins to apply.`，请参阅[应用插件更改而无需重启](https://code.claude.com/docs/zh-CN/plugins/cli-reference.md#reload-plugins)以在当前会话中激活插件。

一旦插件处于活跃状态，您已准备好[扫描和修复您的代码库](#scan-and-fix-your-codebase)。

### 更新插件

要自行更新插件，请参阅[立即更新插件](https://code.claude.com/docs/zh-CN/plugins/install.md#update-plugins-now)。除非关闭了自动更新，否则 Claude Code 还会[自动更新插件](https://code.claude.com/docs/zh-CN/plugins/install.md#keep-plugins-updated)。

### 卸载插件

要删除该插件，从 `/plugin` 菜单卸载它，或在您的终端中运行 `claude plugin uninstall claude-security`。

## 扫描和修复您的代码库

该插件添加了一个命令 `/claude-security`，它打开其三个任务的菜单：扫描代码库、扫描一组更改和建议补丁。标准流程运行完整扫描，然后将其发现转化为补丁：

**1. 打开 Claude Security 菜单**

运行 `/claude-security` 并选择 **Scan codebase**。

**2. 选择要扫描的内容**

该插件首先读取您的存储库，然后提供整个存储库或聚焦区域，每个选项都说明了文件计数和相对成本。选择整个存储库，或回答"我不知道"，插件会为您的存储库大小选择一个合理的默认值。

**3. 确认运行**

扫描可能需要一段时间，可能使用大量令牌，并需要在完成期间保持 Claude Code 打开。在您确认之前，不会运行任何内容。

**4. 阅读报告**

扫描运行时，它会在每个阶段开始时报告，详细信息可在 [`/workflows`](https://code.claude.com/docs/zh-CN/workflows) 下获得。结果进入您的存储库中的时间戳目录，在[阅读扫描结果](#read-the-scan-results)中描述。

**5. 将发现转化为补丁**

再次运行 `/claude-security` 并选择 **Suggest patches**，然后选择要解决的发现。审查过的补丁进入报告的 `patches/` 文件夹；[修复发现](#fix-findings)涵盖了每个补丁如何构建和审查。

**6. 应用您接受的补丁**

从您的 shell 中使用 `git apply` 应用每个补丁，在其自己的拉取请求中。补丁永远不会自动应用。

您不必从菜单开始：直接要求一个任务，作为命令的参数，例如 `/claude-security scan my branch`，或用纯语言，例如"scan commit abc1234"。该插件在[自动模式](https://code.claude.com/docs/zh-CN/permission-modes.md)中效果最佳，这允许扫描的代理在每一步都无需权限提示地进行。

### 仅扫描您的更改

当您的分支有其基础没有的提交时，`/claude-security` 菜单会提供仅扫描该差异的选项，以便您可以在合并前检查分支。您也可以扫描您的一个开放拉取请求，或通过要求它来扫描单个提交，例如"scan commit abc1234"。仅扫描已提交的更改：首先提交或 stash 进行中的编辑，或运行完整扫描，它读取工作树。

更改扫描需要 git 存储库；未版本化目录的完整扫描仍然有效。查找您的开放拉取请求是唯一到达网络的步骤，仅当您的会话已有权限运行 GitHub CLI 且 `gh` 已登录时才提供。

### 限制大型存储库的范围

在大型存储库上，一次扫描一个区域而不是整个树。选择插件提供的聚焦范围之一，例如您的 API 层或您的身份验证代码，运行会根据您选择的内容调整大小。报告的覆盖部分说明了什么被检查了，什么没有。随时在不同区域运行另一次扫描。

### 选择扫描工作量

扫描工作量决定扫描的深度。在 `low`、`medium` 和 `high` 下，您的发现都会经过相同的独立验证。

* **`low`**：快速初步检查。扫描代码库时，它会跳过架构图和威胁模型。
* **`medium`**：默认值。
* **`high`**：执行 `medium` 的所有工作，并在同一代码中进一步搜寻漏洞。

在 Claude Code 输入框中的请求里指明扫描工作量，例如 `/claude-security scan my public API code at high` 或 `/claude-security scan my branch at high`。

如果对大型仓库的扫描发现的问题少于预期，请选择 `high` 扫描工作量、再扫描一次，或两者都做。每种方式都可能报告出一次 `medium` 扫描未发现的问题。

### 阅读扫描结果

每次扫描都会将其结果写入您的存储库中的时间戳 `CLAUDE-SECURITY-<timestamp>/` 目录：

* **`CLAUDE-SECURITY-RESULTS.md`**：报告，包含每个发现的 ID，例如 `F1`，加上其影响、利用场景、严重性、置信度和建议
* **`CLAUDE-SECURITY-RESULTS.jsonl`**：相同的发现以机器可读的形式，每行一个 JSON 对象
* **`CLAUDE-SECURITY-RESULTS.sarif`**：相同的发现作为 [SARIF 2.1.0](https://docs.oasis-open.org/sarif/sarif/v2.1.0/sarif-v2.1.0.html) 日志，用于 GitHub 代码扫描和任何其他读取该标准的工具。扫描将发现分类在其 [CWE](https://cwe.mitre.org/) 弱点类别下
* **`CLAUDE-SECURITY-REVISION-<commit>.json`**：修订戳，记录扫描了哪个提交、以什么[扫描工作量](#choose-a-scan-effort)、未提交的更改是否是扫描树的一部分，以及运行的验证程度如何，因此报告始终与它描述的代码相关联。版本控制外的扫描在提交位置戳上 `UNVERSIONED`

该目录是扫描对您的检出所做的唯一更改，它有自己的 `.gitignore`，因此随意的 `git add` 永远不会将报告扫入提交。要在历史中保留报告以供审计跟踪，删除那一个 `.gitignore` 文件并像任何其他文件一样提交目录。

发现仅在独立验证代理分析它们后才出现在报告中，这使报告简短且值得阅读。扫描是非确定性的：同一代码的两次扫描可能会发现不同的发现。定期运行扫描，并使用修订戳将每个报告归属于它覆盖的确切代码和设置。

## 修复发现

通过从 `/claude-security` 菜单选择 **Suggest patches** 开始修复流程，或用纯语言要求，例如"fix finding F3"，然后选择要解决的报告中的哪些发现。补丁是针对已提交的代码构建的，报告必须仍然描述您拥有的代码：其代码已更改的发现会被跳过并附注，插件会提供新扫描而不是从陈旧报告修补。每个补丁都在您的存储库的临时副本中起草，因此您的源文件保持不变，直到您自己应用补丁。

在交付前，每个补丁都由独立于编写它的代理的代理审查，当代码有测试时它会针对更改运行您的项目测试，并自行读取差异以查看它可能引入的任何新内容。仅当该审查可以保证更改解决了一个发现、不引入新漏洞并保持其他行为不变时，才会编写补丁。当它无法保证所有三个时，您会得到一个简短的注释解释原因，而不是补丁。

### 补丁永远不会自动应用

应用补丁始终是您的决定。补丁进入报告的 `patches/` 文件夹，每个发现一个 `F<n>.patch`，旁边有一个注释解释更改。从您的 shell 应用一个，或要求 Claude 应用它并打开拉取请求：

```bash theme={null}
git apply CLAUDE-SECURITY-<timestamp>/patches/F1.patch
```

当修补的代码没有测试时，补丁的注释会说明这一点，因此您知道其审查在没有测试通过的情况下运行。在其自己的拉取请求中应用每个补丁，以便可以独立审查和测试。

## 该插件如何与其他安全工具配合

Claude Security 插件是深度扫描层，在纵深防御堆栈中，与[security guidance 插件](https://code.claude.com/docs/zh-CN/security-guidance.md)、[`/security-review`](https://code.claude.com/docs/zh-CN/commands#all-commands)、[Code Review](https://code.claude.com/docs/zh-CN/code-review.md)、托管的 [Claude Security](https://claude.com/product/claude-security) 产品和您现有的扫描器一起：

| 阶段 | 工具 | 覆盖内容 |
| :- | :- | :- |
| 在会话中 | [Security guidance 插件](https://code.claude.com/docs/zh-CN/security-guidance.md) | Claude 编写的代码中的常见漏洞，在同一会话中修复 |
| 按需，单次扫描 | [`/security-review`](https://code.claude.com/docs/zh-CN/commands#all-commands) | 当前分支上的一次性安全扫描 |
| 按需，深度扫描 | Claude Security 插件 | 存储库或差异的多代理扫描，具有独立审查的发现和补丁 |
| 在拉取请求上 | [Code Review](https://code.claude.com/docs/zh-CN/code-review.md)，Team 和 Enterprise 计划 | 具有完整代码库上下文的多代理正确性和安全审查 |
| 托管 | [Claude Security](https://claude.com/product/claude-security)，Enterprise 计划 | 监控连接存储库的托管扫描 |
| 在 CI 中 | 您现有的静态分析和依赖扫描器 | 特定于语言的规则、供应链检查和策略执行 |

该插件不会替换您现有的源代码安全工具。与静态分析、依赖扫描和代码审查一起运行它：它以人类安全研究人员的方式推理您的代码，这补充了这些工具提供的确定性检查。

## 故障排除

**`/claude-security` 菜单打开时出现 Python 警告。** 该插件需要 `python3` 3.9 或更高版本在您的 `PATH` 上。当它根本找不到 `python3` 时，菜单警告 Claude Security 在安装一个之前不会工作；当您的 `PATH` 上的第一个 `python3` 较旧时，警告会命名它找到的版本。安装 Python 3，或在您的 `PATH` 上放置一个较新的 `python3`，然后启动一个新会话。

**使用 Fable 模型扫描时，您可能会看到"safeguards flagged this message"通知。** 该消息命名您正在运行的模型。Fable 的网络安全安全分类器标记某些请求，Claude Code 通过[自动模型回退](https://code.claude.com/docs/zh-CN/model-config.md#automatic-model-fallback)在 Opus 模型上重新运行标记的请求。这是预期的。当请求重新运行时，扫描应该仍然成功完成。

## 相关资源

要深入了解此页面涉及的部分：

* [Security guidance 插件](https://code.claude.com/docs/zh-CN/security-guidance.md)：在同一会话中，在 Claude 编写代码时捕获问题
* [Code Review](https://code.claude.com/docs/zh-CN/code-review.md)：设置 PR 时间多代理审查
* [Claude Security](https://claude.com/product/claude-security)：监控连接存储库的托管服务
* [Claude Code 安全](https://code.claude.com/docs/zh-CN/security.md)：Claude Code 如何处理信任、权限和保护措施
* [安装和管理插件](https://code.claude.com/docs/zh-CN/plugins/install.md)：从官方市场查找和安装其他插件
