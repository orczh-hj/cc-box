> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# 配置自动模式

> 告诉自动模式分类器您的组织信任哪些代码库、存储桶和域。设置环境上下文，覆盖默认的阻止和允许规则，并使用自动模式 CLI 子命令检查您的有效配置。

[自动模式](https://code.claude.com/docs/zh-CN/permission-modes.md#eliminate-prompts-with-auto-mode)让 Claude Code 无需常规权限提示即可运行，通过将工具调用路由到一个分类器，该分类器会阻止任何不可逆、破坏性或针对您环境外的操作。拒绝和显式询问规则在分类器之前进行评估，仍然会阻止或提示。使用 `autoMode` 设置块告诉该分类器您的组织信任哪些代码库、存储桶和域，以便它停止阻止常规内部操作。

> [!NOTE]
>
> 本页是配置参考。打开和关闭自动模式在权限模式页面上有介绍：
>
> * **在会话中期切换到自动模式，或退出自动模式**：请参阅[切换权限模式](https://code.claude.com/docs/zh-CN/permission-modes.md#switch-permission-modes)
> * **在自动模式下启动会话**：请参阅[以不同的权限模式启动](https://code.claude.com/docs/zh-CN/permission-modes.md#start-in-a-different-mode)

自动模式可供所有提供商上的所有用户使用，包括 Anthropic API、[AWS 上的 Claude Platform](https://code.claude.com/docs/zh-CN/claude-platform-on-aws.md)、Amazon Bedrock、Google Cloud 的 Agent Platform、Microsoft Foundry 和已登录的 [Claude 应用网关](https://code.claude.com/docs/zh-CN/claude-apps-gateway)会话。如果 Claude Code 报告您的账户无法使用自动模式，请检查[完整要求](https://code.claude.com/docs/zh-CN/permission-modes.md#eliminate-prompts-with-auto-mode)，其中还涵盖了支持的模型和 Team 和 Enterprise 计划上的组织级控制。

默认情况下，分类器仅信任工作目录和当前代码库的已配置远程。推送到您公司的源代码控制组织或写入团队云存储桶等操作会被阻止，直到您将它们添加到 `autoMode.environment`。

本页涵盖如何：

* [为推送和拉取请求添加人工检查点](#add-a-human-checkpoint)，使用 `permissions.ask`
* [定义受信任的基础设施](#define-trusted-infrastructure)，使用 `autoMode.environment`
* [生成环境条目](#generate-environment-entries)，使用 `/auto-mode-setup`
* [查看拒绝](#review-denials)，以便您知道接下来要添加什么

## 常见边界

自动模式允许推送到您正在处理的存储库的任何分支（包括默认分支），并默认创建拉取请求。标记为部署或发布目标的非默认分支（例如 `production`、`release` 或 `gh-pages`）不受该默认值的约束：分类器会根据其自身条件判断对该分支的推送，包括作为生产部署。推送的内容仍然会被检查，因此强制推送、提交中出现的密钥或在 CI 或部署管道运行时会将密钥发送到存储库外的更改仍然会被阻止。

如果您想在 Claude 的推送和拉取请求命令之前进行人工检查点，请添加权限规则：下面的[配方](#add-a-human-checkpoint)为其他所有操作保持自动模式开启。

### 添加人工检查点

最直接的机制是 [`permissions.ask`](https://code.claude.com/docs/zh-CN/permissions#permission-rule-syntax)。内容范围的 ask 规则（如下面的规则）在分类器之前进行评估，并且即使在自动模式下也始终强制权限提示，因为显式 ask 规则是您明确表示要对该操作进行提示的意图。在您的 [settings](https://code.claude.com/docs/zh-CN/settings.md#where-settings-live) 中添加规则：

```json theme={null}
{
  "permissions": {
    "ask": [
      "Bash(git push *)",
      "Bash(gh pr create *)"
    ]
  }
}
```

这些规则匹配以 `git push` 或 `gh pr create` 开头的命令。Claude 以其他方式编写的推送，例如 `git -C <dir> push` 或 `git -c <key>=<value> push`，[不匹配该规则](https://code.claude.com/docs/zh-CN/permissions.md#bash-rule-limits)，因此不会被检查点。对于检查完整命令文本的检查点，请添加 [PreToolUse hook](https://code.claude.com/docs/zh-CN/hooks.md#pretooluse)。

选择与边界需要的严格程度相匹配的机制：

| 边界 | 机制 | 自动模式中的行为 |
| :- | :- | :- |
| 在操作前提示 | `permissions.ask` | 始终为匹配内容范围规则（如上面的配方）的命令进行提示。分类器无法自动批准匹配的操作。 |
| 永不运行操作 | `permissions.deny` | 在咨询分类器之前阻止。分类器和用户意图都无法覆盖它。 |
| 此会话的一次性边界 | 在对话中说明，例如"在我审查之前不要推送" | 分类器阻止匹配的操作，但如果 [context compaction](https://code.claude.com/docs/zh-CN/costs.md#reduce-token-usage) 删除了说明该边界的消息，边界可能会丢失。使用 ask 或 deny 规则以获得持久保证。 |

## 分类器读取配置的位置

分类器读取与 Claude 本身加载的相同 [CLAUDE.md](https://code.claude.com/docs/zh-CN/memory.md) 内容，因此项目的 CLAUDE.md 中的指令（如"从不强制推送"）会同时引导 Claude 和分类器。从那里开始了解项目约定和行为规则。

对于跨项目应用的规则，例如受信任的基础设施或组织范围的拒绝规则，请使用 `autoMode` 设置块。分类器从以下范围读取 `autoMode`：

| 范围 | 文件 | 用途 |
| :- | :- | :- |
| 单个开发者 | `~/.claude/settings.json` | 个人受信任的基础设施 |
| 组织范围 | [托管设置](https://code.claude.com/docs/zh-CN/server-managed-settings.md) | 分发给所有开发者的受信任基础设施 |
| `--settings` 标志或 Agent SDK | 内联 JSON | 自动化的每次调用覆盖 |

分类器不从 `.claude/settings.json` 或 `.claude/settings.local.json` 中的项目设置读取 `autoMode`。两个文件都位于仓库目录中，因此已检入的仓库或构建步骤可能会注入自己的允许规则。将 `.claude/settings.local.json` 中的任何 `autoMode` 块移动到 `~/.claude/settings.json`。

来自每个范围的条目被合并。开发者可以使用个人条目扩展 `environment`、`allow`、`soft_deny` 和 `hard_deny`，但不能删除托管设置提供的条目。由于允许规则在分类器内充当软块规则的例外，开发者添加的 `allow` 条目可以覆盖组织的 `soft_deny` 条目：组合是累加的，而不是硬策略边界。

> [!NOTE]
>
> 分类器是在[权限系统](https://code.claude.com/docs/zh-CN/permissions.md)之后运行的第二道门。对于必须永远不运行的操作，无论用户意图或分类器配置如何，请在托管设置中使用 `permissions.deny`，它在咨询分类器之前阻止操作，无法被覆盖。

## 定义受信基础设施

对于大多数组织，`autoMode.environment` 是您唯一需要设置的字段。它告诉分类器哪些仓库、存储桶和域是受信的：分类器使用它来决定"外部"的含义，因此任何未列出的目标都是潜在的数据泄露目标。

`claude auto-mode defaults` 打印三种环境条目。

* **Context slots**：描述您的组织、技术栈和安全态势，以便分类器读取您上下文中的其他规则。每个默认为 `None configured` 或保守假设（如下所示）：
  * **Organization**
  * **Claude Code 的主要用途**：默认为软件开发
  * **云提供商**
  * **Repository visibility**：除非其远程主机和名称另有说明，或分类器在对话中较早读取了显示其为公开的可见性检查，否则假定仓库为私有。

    在 Claude Code 本身发送的分类器请求中，分类器读取您的消息和 Claude 运行的命令，而不是它们的输出。证据必须是分类器能够读取的内容，例如您自己的消息将仓库命名为公开；单独的 `gh repo view` 输出无法到达它。
  * **Internal sharing / snippet hosting**：公开粘贴和 gist 服务被视为在信任边界之外，直到您命名其中一个
  * **Org-specific CLIs**
  * **Secrets management**
  * **CI/CD deploy targets**
  * **Network posture**
  * **Host containment**：默认为具有开放互联网的普通开发者机器或 CI 运行器。如果 Claude Code 在具有出站允许列表或不能接触的邻居的容器、VM 或 pod 中运行，请命名允许的主机、云元数据端点是否应该可达，以及任务使用的云项目、集群或注册表以及使用什么身份。在此条目命名该身份之前，分类器[阻止](https://code.claude.com/docs/zh-CN/permission-modes.md#what-the-classifier-blocks-by-default)对主机自身凭证的请求。需要 Claude Code v2.1.257 或更高版本
  * **Protected deployment namespaces / environments**：在您命名它们之前回退到 Sensitive remote targets 启发式
  * **Data retention / declassification**
* **Trust slots**：命名分类器视为在您边界内的内容。这些 slots 是 Trusted repo、Source control、Trusted internal domains、Trusted cloud buckets、Key internal services 和 Internal package registry。repo 和 source-control 条目默认为工作仓库及其配置的远程。所有其他信任 slot 默认为 `None configured`，因此在您添加之前没有其他内容是受信的。仓库的可见性仅限制机密材料：私有仓库是机密材料的可接受目标，但将仓库设为私有永远不会授权将秘密、个人或受信数据放入其中，分类器将从工作仓库外部移植、重新指向或首次读取的内容视为不是该仓库自身的工作。
* **Sensitivity slots**：命名保护规则视为高风险的内容。这些 slots 是 Sensitive data locations & audiences、Sensitive remote targets 和 Protected IaC scopes。每个默认为广泛的启发式，例如将任何名称中包含 `prod` 或 `production` 的主机或命名空间视为敏感远程目标，因此保护规则在您配置任何内容之前就处于活跃状态。在敏感 slot 中命名具体目标会使这些规则应用于命名的目标而不是启发式。

要在默认值旁边添加您自己的条目，请在数组中包含字面字符串 `"$defaults"`。默认条目在该位置被拼接，因此您的自定义条目可以在它们之前或之后。

以下示例保留默认条目并添加组织的仓库、存储桶、域和服务。

```json theme={null}
{
  "autoMode": {
    "environment": [
      "$defaults",
      "Source control: github.example.com/acme-corp and all repos under it",
      "Trusted cloud buckets: s3://acme-build-artifacts, gs://acme-ml-datasets",
      "Trusted internal domains: *.corp.example.com, api.internal.example.com",
      "Key internal services: Jenkins at ci.example.com, Artifactory at artifacts.example.com"
    ]
  }
}
```

保存设置后，运行 `claude auto-mode config` 以[确认有效规则](#inspect-the-defaults-and-your-effective-config)包含您的条目。

条目是散文，不是正则表达式或工具模式。分类器将它们读取为自然语言规则。按照您向新工程师描述基础设施的方式编写它们。彻底的环境部分涵盖：

* **Organization**：您的公司名称以及 Claude Code 主要用于什么，例如软件开发、基础设施自动化或数据工程
* **Source control**：您的开发人员推送到的每个 GitHub、GitLab 或 Bitbucket 组织
* **Cloud providers and trusted buckets**：Claude 应该能够读取和写入的存储桶名称或前缀
* **Trusted internal domains**：网络内 API、仪表板和服务的主机名，例如 `*.internal.example.com`
* **Key internal services**：CI、工件注册表、内部包索引、事件工具
* **Internal package registry**：安装应该通过的私有 npm、PyPI 或其他注册表，因此绕过它安装到公开注册表的安装会被阻止
* **Sensitive data locations & audiences**：包含个人数据、机密业务数据、凭证、受管制数据或类似敏感材料的存储桶、数据库或路径，以及每个位置中的数据可能与之共享的受众，以便分类器保护这些位置而不是从内容猜测
* **Sensitive remote targets**：计为生产的命名空间、主机或容器，因此远程 shell 和端口转发到它们需要您的明确批准
* **Protected IaC scopes**：应用或销毁应始终需要您命名更改的基础设施资源
* **Additional context**：受管制行业约束、多租户基础设施或影响分类器应视为风险的合规要求

一个有用的起始模板：填写括号中的字段并删除任何不适用的行。

```json theme={null}
{
  "autoMode": {
    "environment": [
      "$defaults",
      "Organization: {COMPANY_NAME}. Primary use: {PRIMARY_USE_CASE, e.g. software development, infrastructure automation}",
      "Source control: {SOURCE_CONTROL, e.g. GitHub org github.example.com/acme-corp}",
      "Cloud provider(s): {CLOUD_PROVIDERS, e.g. AWS, GCP, Azure}",
      "Trusted cloud buckets: {TRUSTED_BUCKETS, e.g. s3://acme-builds, gs://acme-datasets}",
      "Trusted internal domains: {TRUSTED_DOMAINS, e.g. *.internal.example.com, api.example.com}",
      "Key internal services: {SERVICES, e.g. Jenkins at ci.example.com, Artifactory at artifacts.example.com}",
      "Additional context: {EXTRA, e.g. regulated industry, multi-tenant infrastructure, compliance requirements}"
    ]
  }
}
```

您不需要一次性填写所有内容。合理的推出方式：从默认值开始，添加您的源代码控制组织和关键内部服务，这解决了最常见的误报，例如推送到您自己的仓库。接下来添加受信域和云存储桶。当出现阻止时填写其余部分。

## 使用 `/auto-mode-setup` 生成环境条目

运行 `/auto-mode-setup`，让 Claude Code 根据您的项目以及您在该项目中的近期会话，起草 `autoMode.environment` 条目，有时也会起草[规则条目](#override-the-block-and-allow-rules)。如果您接受该草稿，Claude Code 会将其写入 `~/.claude/settings.json`。

> [!NOTE]
>
> `/auto-mode-setup` 适用于 Pro、Max、Ultra、Team 和 Enterprise 计划，以及使用 Claude Console API 密钥的情况。在 Pro、Max、Ultra 或 Team 计划中，您需要 v2.1.228 或更高版本，在原生 Windows 上则需要 v2.1.233 或更高版本。在 Enterprise 计划中或使用 Claude Console API 密钥时，您需要 v2.1.283 或更高版本。如果您的会话中没有该命令，请参阅 [`/auto-mode-setup` 返回 "Unknown command"](#auto-mode-setup-returns-unknown-command)。

### `/auto-mode-setup` 读取的内容

如果 `~/.claude/settings.json` 中已有 `autoMode` 条目，Claude Code 会首先询问是添加到您的环境列表还是替换它，无论哪种方式都会保留您编写的规则。随后，Claude Code 会询问您如何使用此项目，并在扫描任何内容之前提供两项可选扫描。在扫描中，Claude Code 始终读取以下来源：

* 此项目的 `CLAUDE.md`、`README.md`、配置文件和 Git 远程地址
* 您的 `autoMode` 和 `permissions.allow` 设置
* Claude 在您于此项目中的近期会话里运行的命令所涉及的主机、存储桶和命令名称，但绝不包括您的消息

两项可选扫描各增加一个来源：

* 您的 shell 历史记录中每条命令的第一个单词
* 您主目录下各仓库的远程主机和名称

### 审阅并保存草稿

Claude Code 在后台进行扫描，然后向您展示草稿。您只能整体接受或放弃该草稿，因此如需调整单个条目，请在之后编辑 `~/.claude/settings.json`。当您接受时，Claude Code 会写入草稿，并将其与您已有的设置进行协调：

* Claude Code 写入的 `environment` 列表不包含 `"$defaults"`，因为草稿已逐一列出其未更改的内置条目
* 对于草稿添加了条目的 `allow`、`soft_deny` 和 `hard_deny` 列表，Claude Code 会在每个列表中包含 `"$defaults"`，除非您已经编写了不含它的 `allow` 列表，这样您未替换的[内置规则](#override-the-block-and-allow-rules)将继续生效
* 保存后，Claude Code 会提议删除 `~/.claude/settings.json` 中被自动模式忽略的 `permissions.allow` 规则（例如 `Bash(*)`），或会自动批准破坏性命令的规则

然后运行 `claude auto-mode config` 以[查看最终生效的结果](#inspect-the-defaults-and-your-effective-config)。

### 关闭 `/auto-mode-setup`

当自动模式已阻止了若干操作，而您仍没有任何 `autoMode.environment` 条目时，Claude Code 会在某一轮次结束时显示一个标题为 "Teach auto mode about your environment?" 的对话框，并提议为您运行 `/auto-mode-setup`。若要停止此提议但保留该命令，请在该对话框中选择 **Don't show again**。

若要同时关闭该命令和此提议，请将以下 [`skillOverrides`](https://code.claude.com/docs/zh-CN/skills#override-skill-visibility-from-settings) 条目添加到 `~/.claude/settings.json`：

```json theme={null}
{
  "skillOverrides": {
    "auto-mode-setup": "off"
  }
}
```

`/auto-mode-setup` 是内置命令而非[随附 skill](https://code.claude.com/docs/zh-CN/skills.md#bundled-skills)，因此此 `skillOverrides` 条目仍然适用于它，但 [`disableBundledSkills`](https://code.claude.com/docs/zh-CN/settings-reference#disablebundledskills) 无法将其关闭。

### `/auto-mode-setup` 返回 "Unknown command"

如果在 Claude Code 输入框中键入 `/auto-mode-setup` 时输出 `Unknown command: /auto-mode-setup`，请检查以下原因：

* **安装或升级后的第一个会话**：启动一个新会话。如果新会话中仍然没有该命令，请参阅[后续会话启动时也不包含该命令的设置情况](https://code.claude.com/docs/zh-CN/env-vars.md#first-session-after-an-install-or-upgrade)。
* **版本过旧**：在 shell 中运行 `claude --version`。如果版本低于[您所用计划的最低版本](#generate-environment-entries)，请[更新 Claude Code](https://code.claude.com/docs/zh-CN/setup.md#update-claude-code)。
* **遥测已关闭、第三方提供商或 Claude 应用网关**：在 v2.1.304 之前，该命令在[已关闭功能标志获取的会话](https://code.claude.com/docs/zh-CN/env-vars.md#features-that-need-feature-flag-fetching)中不可用。如果您的会话属于其中之一，请将 Claude Code [更新](https://code.claude.com/docs/zh-CN/setup.md#update-claude-code)到 v2.1.304 或更高版本。

在[云端会话](https://code.claude.com/docs/zh-CN/claude-code-on-the-web.md)中，该命令会改为输出 `/auto-mode-setup isn't available in this environment.`。请在终端的交互式会话中运行它。

## 覆盖阻止和允许规则

三个额外的字段让您替换分类器的内置规则列表：

* `autoMode.hard_deny`：无条件安全边界
* `autoMode.soft_deny`：用户意图可以清除的破坏性操作
* `autoMode.allow`：软阻止规则的例外

每个都是散文描述的数组，读作自然语言规则。对于在分类器之前运行的基于工具模式的硬阻止，请使用 [`permissions.deny`](https://code.claude.com/docs/zh-CN/permissions)。

在分类器内，优先级分为四个层级：

* `hard_deny` 规则无条件阻止。用户意图和 `allow` 例外不适用。
* `soft_deny` 规则接下来阻止。用户意图和 `allow` 例外可以覆盖这些。
* `allow` 规则然后覆盖匹配的 `soft_deny` 规则作为例外。
* 明确的用户意图覆盖剩余的软阻止：如果用户的消息直接且具体地描述 Claude 即将采取的确切操作，分类器允许它，即使 `soft_deny` 规则匹配。

一般请求不算作明确意图。要求 Claude"清理代码库"不授权强制推送，但要求 Claude"强制推送此分支"则授权。

要放松，当分类器重复标记默认例外不涵盖的常规模式时，添加到 `allow`。要收紧，为您的环境特定的破坏性风险添加到 `soft_deny`（默认值会遗漏），或为必须永远不能跨越的安全边界添加到 `hard_deny`。要保持内置规则同时添加您自己的规则，请在数组中包含字面字符串 `"$defaults"`。默认规则会在该位置拼接，因此您的自定义规则可以在它们之前或之后，并且当内置列表在版本发布中更改时，您继续继承更新。

以下示例在所有四个列表中保持默认值，并向每个列表添加特定于组织的规则。

```json theme={null}
{
  "autoMode": {
    "environment": [
      "$defaults",
      "Source control: github.example.com/acme-corp and all repos under it"
    ],
    "allow": [
      "$defaults",
      "Deploying to the staging namespace is allowed: staging is isolated from production and resets nightly",
      "Writing to s3://acme-scratch/ is allowed: ephemeral bucket with a 7-day lifecycle policy"
    ],
    "soft_deny": [
      "$defaults",
      "Never run database migrations outside the migrations CLI, even against dev databases",
      "Never modify files under infra/terraform/prod/: production infrastructure changes go through the review workflow"
    ],
    "hard_deny": [
      "$defaults",
      "Never send repository contents to third-party code-review APIs"
    ]
  }
}
```

> [!CAUTION]
>
> 在不包含 `"$defaults"` 的情况下设置 `environment`、`allow`、`soft_deny` 或 `hard_deny` 中的任何一个会替换该部分的整个默认列表。如果您设置一个没有 `"$defaults"` 的数组，您会丢弃该部分的内置规则：
>
> * `soft_deny`：每个内置软阻止规则，包括强制推送、`curl | bash`、生产部署和自动模式绕过
> * `hard_deny`：内置的数据泄露规则

每个部分独立评估，因此单独设置 `environment` 会保持默认 `allow`、`soft_deny` 和 `hard_deny` 列表完整。仅在您打算完全拥有该列表时才省略 `"$defaults"`。要安全地执行此操作，请运行 `claude auto-mode defaults` 打印内置规则，将它们复制到您的设置文件中，然后根据您自己的管道和风险容限审查每条规则。

## 从 `/permissions` 编辑规则

要在不打开设置文件的情况下查看和编辑分类器规则和 `environment` 条目，请运行 [`/permissions`](https://code.claude.com/docs/zh-CN/permissions#manage-permissions) 并选择 **Auto mode** 选项卡。该选项卡需要 Claude Code v2.1.246 或更高版本，仅当 [auto mode 对您的会话可用](https://code.claude.com/docs/zh-CN/permission-modes.md#eliminate-prompts-with-auto-mode) 时才会显示。

Claude Code 将来自 [managed settings](https://code.claude.com/docs/zh-CN/server-managed-settings.md) 或 `--settings` 标志的条目显示为只读，并将您在该选项卡上所做的每项更改保存到 `~/.claude/settings.json`。

## 通过分类器路由所有 shell 命令

默认情况下，narrow Bash 和 PowerShell 允许规则（如 `Bash(npm test)`）在自动模式下保持有效。Claude Code 在分类器运行之前解析它们，除非命令携带[按命令允许的域](https://code.claude.com/docs/zh-CN/sandboxing.md#per-command-allowed-domains-in-auto-mode)。Claude Code 仅暂停授予任意代码执行权限的广泛规则，例如 `Bash(*)` 或通配符解释器，以及每个命名 [`Monitor`](https://code.claude.com/docs/zh-CN/tools-reference#monitor-tool) 的规则，因为 Monitor 命令通过 shell 运行。这意味着 narrow 规则仍然可以让分类器看不到的破坏性参数通过，例如规则前缀未预期的脚本路径或标志。

将 `autoMode.classifyAllShell` 设置为 `true`，以在自动模式处于活动状态时暂停每个 Bash 和 PowerShell 允许规则，使分类器评估每个 shell 命令，无论您的允许列表如何，除了[关键路径移除](https://code.claude.com/docs/zh-CN/permission-modes.md#critical-paths)。

```json theme={null}
{
  "autoMode": {
    "classifyAllShell": true
  }
}
```

这用延迟换取覆盖范围：允许规则会立即批准的命令现在等待分类器决策，每个 shell 命令都计为一次分类器调用。

该设置仅在自动模式处于活动状态时适用，您的允许规则在其他权限模式中表现正常。

## 检查默认值和有效配置

`claude auto-mode` 子命令可帮助您检查、验证和重置配置。

将内置的 `environment`、`allow`、`soft_deny` 和 `hard_deny` 规则打印为 JSON：

```bash theme={null}
claude auto-mode defaults
```

要读取一条规则的完整措辞而不通过 `jq` 管道，请传递 `--label` 和规则标签的开头，例如 `claude auto-mode defaults --label 'Git Destructive'`。匹配是对每条规则标签的不区分大小写的前缀，没有匹配的部分将打印为空列表。

打印分类器实际使用的内容为 JSON，其中应用了您的设置（如果已设置）或使用默认值：

```bash theme={null}
claude auto-mode config
```

`defaults` 和 `config` 都将四个规则列表打印为单个 JSON 对象，每条规则都是散文字符串。这是一个截断的示例：

```json theme={null}
{
  "allow": [
    ...
    "Test Artifacts: Hardcoded test API keys, placeholder credentials in examples, or hardcoding test cases. Placeholder means authored as a placeholder — a file or value copied from a real secret or sensitive path is never a test artifact (see Sensitive-Source Provenance).",
    ...
  ],
  "soft_deny": [
    "Git Destructive [named+specifics — **must name:** the destructive operation and its target]: Force pushing (`git push --force`), deleting remote branches, tags, or releases, or rewriting remote history. Also `git commit --amend` when the commit being rewritten is not the agent's own unpushed work: either no prior `git commit` is visible (HEAD pre-dates the session), or a `git push` of the current branch is visible after the most recent commit (it has been pushed). Clears when the user asked to amend/reword/fixup, or when it is a message-only reword (`--amend -m …`, nothing newly staged) of a commit the agent visibly created this session.",
    ...
  ],
  "hard_deny": [...],
  "environment": [
    ...
    "**Trusted repo**: The git repository the agent started in (its working directory) and its configured remote(s). When the repo's public/private visibility is given — by the Repository visibility entry or the user's own message — use it to scope what is OK to commit or push there: confidential material is fine in a private repo; in a public one, only that repo's own work is — and content ported, repointed, or first read from outside this session's repo is not its own work, whoever directed the port. Visibility scopes confidential material only: secrets and sensitive data (personal & entrusted) are never cleared into any repo by its visibility (see Definitions).",
    ...
  ]
}
```

获取关于您的自定义 `allow`、`soft_deny`、`hard_deny` 和 `environment` 条目的 AI 反馈：

```bash theme={null}
claude auto-mode critique
```

如果您编写了自定义规则，`claude auto-mode critique` 会审查它们并标记模糊、冗余或可能导致误报的条目。

要放弃您的自定义设置并返回内置默认值，请运行 reset 子命令，该命令会从您的用户设置文件中删除 `autoMode` 部分：

```bash theme={null}
claude auto-mode reset
```

该命令总结将删除的内容，并在写入前询问 `Reset auto mode configuration to defaults?`；传递 `--yes` 以跳过确认。Reset 仅更改 `~/.claude/settings.json`：来自[托管设置](https://code.claude.com/docs/zh-CN/server-managed-settings.md)或 `--settings` 标志的 `autoMode` 规则仍然适用。

## 审查拒绝

要审查和重试 auto mode 分类器拒绝的操作，请打开 `/permissions` 并选择 **Recently denied** 选项卡，Claude Code 在该选项卡中记录每个拒绝。在拒绝的操作上按 `r` 将其标记为重试：当您退出对话框时，Claude Code 会发送一条消息，告诉模型它可以重试该工具调用并恢复对话。

当分类器对操作[无法做出判决](https://code.claude.com/docs/zh-CN/errors.md#auto-mode-cannot-determine-the-safety-of-an-action)时，因为与 auto mode 分离的安全检查拒绝了分类器自己的请求或其响应无法解析，Claude Code 会拒绝该操作而不在 **Recently denied** 下记录它。链接的错误条目涵盖了 Claude 被告知的内容以及如果您需要它如何运行该操作。

### 使用允许规则、环境条目或重试来修复拒绝

要查看诸如 `bash denied by auto mode · [Data Exfiltration] · /permissions` 之类的通知背后的命令或 URL，请使用 **Recently denied** 选项卡、hook 或对话：

* **Recently denied 选项卡**：在 `/permissions` 对话框中，高亮显示被拒绝的 Bash 或 PowerShell 操作，即可在列表下方查看其命令。在[屏幕阅读器模式](https://code.claude.com/docs/zh-CN/accessibility.md)下，所有命令会改为显示在列表之前。
* **Hook**：要以编程方式捕获拒绝的确切输入，请添加一个 [`PermissionDenied` hook](https://code.claude.com/docs/zh-CN/hooks#permissiondenied)，它将其作为 `tool_input` 接收。
* **对话**：找到工具调用。如果调用显示为缩短或折叠成摘要行（例如 `Ran 3 shell commands`），请按 `Ctrl+O` 打开[会话记录查看器](https://code.claude.com/docs/zh-CN/interactive-mode.md#transcript-viewer)，它会展开该调用。

调用下方的文本告诉您是否有任何需要修复的内容。一行暗色的 `Not run · auto mode's check had no usable answer`，或报告分类器本身问题的文本（例如 `Auto mode could not evaluate this action`），意味着 Claude Code 在没有来自分类器的判决的情况下阻止了调用。对于 `Not run` 行，请按 `Ctrl+O` 阅读完整消息，然后参阅[自动模式无法确定操作的安全性](https://code.claude.com/docs/zh-CN/errors.md#auto-mode-cannot-determine-the-safety-of-an-action)或[服务器未返回安全判决](https://code.claude.com/docs/zh-CN/errors.md#the-server-returned-no-safety-verdict)了解该怎么做。

否则，一行显示 `Denied by auto mode classifier` 并带有 `[Production Deploy]` 或 `Blocked by classifier` 等原因意味着分类器判断调用不安全，因此从调用试图到达或执行的内容中选择修复：

* Claude 在整个任务中需要的目标，例如包注册表、内部域或存储库主机：将其添加到 `autoMode.environment`。
* 您想从现在开始运行而无需审查的命令：添加一个 `allow` 规则。
* 您确实打算执行的一次性操作：在您的下一条消息中说明该意图，让 Claude 重试。

您可以从 `/permissions` 对话框的 [**Auto mode** 选项卡](#edit-rules-from-permissions)添加环境条目或 `allow` 规则。

方括号中的文本，例如 `[Data Exfiltration]`，是分类器匹配的规则的名称。要阅读该规则的完整措辞，请参阅[检查默认值和您的有效配置](#inspect-the-defaults-and-your-effective-config)。

### 修复重复拒绝

对同一目标的重复拒绝通常意味着分类器缺少上下文。将该目标添加到 `autoMode.environment`，或[运行 `/auto-mode-setup`](#generate-environment-entries) 让 Claude Code 起草条目，然后运行 `claude auto-mode config` 以确认更改已生效。

## 另请参阅

* [Permission modes](https://code.claude.com/docs/zh-CN/permission-modes.md#eliminate-prompts-with-auto-mode)：什么是自动模式、它默认阻止什么，以及哪些会话在其中启动
* [Managed settings](https://code.claude.com/docs/zh-CN/server-managed-settings.md)：在整个组织中部署 `autoMode` 配置
* [Permissions](https://code.claude.com/docs/zh-CN/permissions.md)：在分类器运行之前应用的允许、询问和拒绝规则
* [All settings](https://code.claude.com/docs/zh-CN/settings-reference.md#automode)：每个设置键，包括 `autoMode`
