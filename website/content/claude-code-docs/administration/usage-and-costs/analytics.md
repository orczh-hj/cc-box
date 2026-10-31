> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# 使用分析跟踪团队使用情况

> 在分析仪表板中查看 Claude Code 使用指标、跟踪采用情况并衡量工程速度。

Claude Code 提供分析仪表板，帮助组织了解开发者使用模式、跟踪贡献指标，并衡量 Claude Code 对工程速度的影响。访问您计划的仪表板：

| 计划 | 仪表板 URL | 包含内容 | 了解更多 |
| - | - | - | - |
| Claude for Teams / Enterprise | [claude.ai/analytics/claude-code](https://claude.ai/analytics/claude-code) | 使用指标、带 GitHub 集成的贡献指标、排行榜、数据导出 | [详情](#access-analytics-for-team-and-enterprise) |
| API (Claude Console) | [platform.claude.com/claude-code](https://platform.claude.com/claude-code) | 使用指标、支出跟踪、团队洞察 | [详情](#access-analytics-for-api-customers) |

Code Review 有其独立的仪表板。有关审查活动和支出，请参阅[查看使用情况](https://code.claude.com/docs/zh-CN/code-review-costs.md#view-usage)。

## 访问 Team 和 Enterprise 分析

导航到 [claude.ai/analytics/claude-code](https://claude.ai/analytics/claude-code)。管理员和所有者可以查看仪表板。

Team 和 Enterprise 仪表板包括：

* **使用指标**：接受的代码行数、建议接受率、日活跃用户和会话数
* **贡献指标**：使用 Claude Code 协助的 PR 和已发布的代码行数，带有 [GitHub 集成](#enable-contribution-metrics)
* **排行榜**：按 Claude Code 使用情况排名的顶级贡献者
* **数据导出**：将贡献数据下载为 CSV 格式以进行自定义报告

对于每个用户的令牌计数和成本估计，请配置 [OpenTelemetry 导出](https://code.claude.com/docs/zh-CN/monitoring-usage.md)，或从您组织的分析设置中导出 [支出报告](https://support.claude.com/en/articles/12883420-view-usage-analytics-for-team-and-enterprise-plans)，其中列出了每个用户和每个模型的令牌使用情况和估计的使用额度支出。

### 启用贡献指标

> [!NOTE]
>
> 贡献指标处于公开测试版，可用于 Claude for Teams 和 Claude for Enterprise 计划。这些指标仅涵盖您 claude.ai 组织内的用户。通过 Claude Console API 或第三方集成的使用不包括在内。

使用和采用数据可用于所有 Claude for Teams 和 Claude for Enterprise 账户。贡献指标需要额外设置来连接您的 GitHub 组织。

您需要所有者角色来配置分析设置。GitHub 管理员必须安装 GitHub 应用。

> [!WARNING]
>
> 启用了 [Zero Data Retention](https://code.claude.com/docs/zh-CN/zero-data-retention.md) 的组织无法使用贡献指标。分析仪表板将仅显示使用指标。

**1. 安装 GitHub 应用**

GitHub 管理员在您组织的 GitHub 账户上安装 Claude GitHub 应用，地址为 [github.com/apps/claude](https://github.com/apps/claude)。

**2. 启用 Claude Code 分析**

Claude 所有者导航到 [claude.ai/admin-settings/claude-code](https://claude.ai/admin-settings/claude-code) 并启用 Claude Code 分析功能。

**3. 启用 GitHub 分析**

在同一页面上，启用"GitHub 分析"切换。

**4. 使用 GitHub 进行身份验证**

完成 GitHub 身份验证流程并选择要包含在分析中的 GitHub 组织。

启用后，数据通常在 24 小时内出现，并进行每日更新。如果没有数据出现，您可能会看到以下消息之一：

* **"GitHub 应用必需"**：安装 GitHub 应用以查看贡献指标
* **"数据处理进行中"**：几天后重新检查，如果数据未出现，请确认 GitHub 应用已安装

贡献指标涵盖托管在 github.com 上的仓库。对于 [GitHub Enterprise Server](https://code.claude.com/docs/zh-CN/github-enterprise-server.md) 上的仓库，分析仪表板仅显示使用指标。

### 查看摘要指标

> [!NOTE]
>
> 这些指标故意保守，代表对 Claude Code 实际影响的低估。仅计算有高度信心涉及 Claude Code 的代码行和 PR。

仪表板在顶部显示这些摘要指标：

* **带 CC 的 PR**：包含至少一行使用 Claude Code 编写的代码的已合并拉取请求的总计数
* **带 CC 的代码行**：所有已合并 PR 中使用 Claude Code 协助编写的代码行总数。仅计算"有效行"：规范化后超过 3 个字符的行，不包括空行和仅包含括号或琐碎标点符号的行。
* **带 Claude Code 的 PR (%)**：包含 Claude Code 协助代码的所有已合并 PR 的百分比
* **建议接受率**：用户接受 Claude Code 代码编辑建议的次数百分比，包括 Edit、Write 和 NotebookEdit 工具使用
* **接受的代码行**：Claude Code 编写且用户在其会话中接受的代码行总数。这不包括被拒绝的建议，也不跟踪后续删除。

### 探索图表

仪表板包括多个图表来可视化一段时间内的趋势。

#### 跟踪采用

采用图表显示每日使用趋势：

* **用户**：日活跃用户
* **会话**：每天的活跃 Claude Code 会话数

#### 衡量每个用户的 PR

此图表显示一段时间内的个人开发者活动：

* **每个用户的 PR**：每天合并的 PR 总数除以日活跃用户
* **用户**：日活跃用户

使用此功能了解随着 Claude Code 采用增加，个人生产力如何变化。

#### 查看拉取请求分解

拉取请求图表显示已合并 PR 的每日分解：

* **带 CC 的 PR**：包含 Claude Code 协助代码的拉取请求
* **不带 CC 的 PR**：不包含 Claude Code 协助代码的拉取请求

切换到**代码行**视图以按代码行而不是 PR 计数查看相同的分解。

#### 查找顶级贡献者

排行榜显示按贡献量排名的前 10 个用户。在以下之间切换：

* **拉取请求**：显示每个用户的带 Claude Code 的 PR 与所有 PR
* **代码行**：显示每个用户的带 Claude Code 的行与所有行

单击**导出所有用户**以将所有用户的完整贡献数据下载为 CSV 文件。导出包括所有用户，而不仅仅是显示的前 10 个。

### PR 归属

启用贡献指标后，Claude Code 会分析已合并的拉取请求，以确定哪些代码是使用 Claude Code 协助编写的。这是通过将 Claude Code 会话活动与每个 PR 中的代码进行匹配来完成的。

#### 归属过程

当拉取请求被合并时：

1. 从 PR diff 中提取添加的行
2. 识别在时间窗口内编辑匹配文件的 Claude Code 会话
3. 使用多种策略将 PR 行与 Claude Code 输出进行匹配
4. 计算 AI 协助行和总行的指标

在比较之前，行被规范化：空格被修剪、多个空格被折叠、引号被标准化、文本被转换为小写。

包含 Claude Code 协助行的已合并拉取请求在 GitHub 中被标记为 `claude-code-assisted`。

#### 时间窗口

PR 合并日期前 21 天到后 2 天的会话被考虑用于归属匹配。

#### 排除的文件

某些文件会自动从分析中排除，因为它们是自动生成的：

* 锁定文件：package-lock.json、yarn.lock、Cargo.lock 等
* 生成的代码：Protobuf 输出、构建工件、缩小的文件
* 构建目录：dist/、build/、node\_modules/、target/
* 测试夹具：快照、磁带、模拟数据
* 超过 1,000 个字符的行，可能是缩小或生成的

#### 归属说明

在解释归属数据时，请记住这些额外的细节：

* 由开发者大幅重写的代码（差异超过 20%）不归属于 Claude Code
* 不考虑 21 天窗口外的会话
* 该算法在执行归属时不考虑 PR 源或目标分支

### 从分析中获得最大收益

使用贡献指标来展示 ROI、识别采用模式，并找到可以帮助他人入门的团队成员。

#### 监控采用

跟踪采用图表和用户计数以识别：

* 可以分享最佳实践的活跃用户
* 整个组织的整体采用趋势
* 可能表示摩擦或问题的使用下降

#### 衡量 ROI

贡献指标帮助回答"这个工具值得投资吗？"，使用来自您自己代码库的数据：

* 随着采用增加，跟踪一段时间内每个用户的 PR 变化
* 比较使用和不使用 Claude Code 发布的 PR 和代码行
* 与 [DORA 指标](https://dora.dev/)、冲刺速度或其他工程 KPI 一起使用，以了解采用 Claude Code 的变化

#### 识别超级用户

排行榜帮助您找到具有高 Claude Code 采用率的团队成员，他们可以：

* 与团队分享提示技术和工作流
* 提供关于什么运行良好的反馈
* 帮助新用户入门

#### 以编程方式访问数据

在 Enterprise 计划上，[Claude Enterprise Analytics API](https://platform.claude.com/docs/en/api/admin/analytics) 为您的组织返回每个用户的参与度、使用情况和成本报告，涵盖 Claude 的所有表面，包括 Claude Code。主要所有者在 [claude.ai/analytics/api-keys](https://claude.ai/analytics/api-keys) 处使用 `read:analytics` 范围创建密钥。该 API 在 Teams 计划上不可用。

要通过 GitHub 查询贡献数据，请搜索标记为 `claude-code-assisted` 的 PR。

## 访问 API 客户的分析

使用 Claude Console 的 API 客户可以在 [platform.claude.com/claude-code](https://platform.claude.com/claude-code) 访问分析。您需要 UsageView 权限来访问仪表板，该权限授予开发者、计费、管理员、所有者和主要所有者角色。要以编程方式拉取相同的每日每用户指标，请使用 [Claude Code Analytics API](https://platform.claude.com/docs/en/build-with-claude/claude-code-analytics-api) 和管理员 API 密钥。

> [!NOTE]
>
> 贡献指标与 GitHub 集成目前不可用于 API 客户。Console 仪表板仅显示使用和支出指标。

Console 仪表板显示：

* **接受的代码行**：Claude Code 编写且用户在其会话中接受的代码行总数。这不包括被拒绝的建议，也不跟踪后续删除。
* **建议接受率**：用户接受代码编辑工具使用的次数百分比，包括 Edit、Write 和 NotebookEdit 工具。
* **活动**：图表上显示的日活跃用户和会话。
* **支出**：每日 API 成本（美元）与用户计数一起显示。

### 查看团队洞察

团队洞察表显示每个用户的指标：

* **成员**：所有已向 Claude Code 进行身份验证的用户。API 密钥用户按密钥标识符显示，OAuth 用户按电子邮件地址显示。
* **本月支出**：每个用户当前月份的 API 成本总计。
* **本月代码行**：每个用户当前月份接受的代码行总数。

> [!NOTE]
>
> Console 仪表板中的支出数字是用于分析目的的估计值。有关实际成本，请参阅您的计费页面。

## 相关资源

* [使用 OpenTelemetry 进行监控](https://code.claude.com/docs/zh-CN/monitoring-usage.md)：将实时指标和事件导出到您的可观测性堆栈
* [有效管理成本](https://code.claude.com/docs/zh-CN/costs.md)：设置支出限制并优化令牌使用
* [权限](https://code.claude.com/docs/zh-CN/permissions.md)：配置角色和权限
