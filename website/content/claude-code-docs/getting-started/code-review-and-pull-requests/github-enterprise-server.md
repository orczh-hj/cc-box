> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Claude Code 与 GitHub Enterprise Server

> 将 Claude Code 连接到自托管的 GitHub Enterprise Server 实例，用于云会话、代码审查和插件市场。

> [!NOTE]
>
> GitHub Enterprise Server 支持适用于 Team 和 Enterprise 计划。

GitHub Enterprise Server (GHES) 支持让您的组织使用 Claude Code 处理托管在自管理 GitHub 实例上的存储库，而不是 github.com。一旦所有者连接您的 GHES 实例，开发人员可以运行云会话和获得自动化代码审查，无需任何按存储库的配置。您实例上托管的插件市场也受支持；凭证要求因表面而异，如 [GHES 上的插件市场](#plugin-marketplaces-on-ghes) 中所述。

对于 github.com 上的存储库，请参阅 [云上的 Claude Code](https://code.claude.com/docs/zh-CN/claude-code-on-the-web.md) 和 [代码审查](https://code.claude.com/docs/zh-CN/code-review.md)。要在您自己的 CI 基础设施中运行 Claude，请参阅 [GitHub Actions](https://code.claude.com/docs/zh-CN/github-actions.md)。

## GitHub Enterprise Server 支持的功能

下表显示了哪些 Claude Code 功能支持 GHES 以及与 github.com 行为的任何差异。

| 功能 | GHES 支持 | 备注 |
| :- | :- | :- |
| 云端会话 | ✅ 支持 | 所有者连接 GHES 实例一次；开发人员使用 `claude --cloud` 或 [claude.ai/code](https://claude.ai/code) |
| 代码审查 | ✅ 支持 | 与 github.com 相同的自动化 PR 审查 |
| Claude Security | ✅ 支持 | 在 [claude.ai/security](https://claude.ai/security) 为 Enterprise 计划提供公开测试版 |
| Teleport 会话 | ✅ 支持 | 使用 `--teleport` 在云和终端之间移动会话 |
| 插件市场 | ✅ 支持 | 凭证要求因表面而异。请参阅 [GHES 上的插件市场](#plugin-marketplaces-on-ghes) |
| 贡献指标 | ❌ 不支持 | 需要托管在 github.com 上的仓库。[分析仪表板](https://code.claude.com/docs/zh-CN/analytics.md) 仍会显示 GHES 仓库中工作的使用指标 |
| GitHub Actions | ✅ 支持 | 需要手动工作流设置；`/install-github-app` 仅适用于 github.com |
| GitHub MCP server | ❌ 不支持 | GitHub MCP server 不适用于 GHES 实例 |

## 管理员设置

所有者只需将您的 GHES 实例连接到 Claude Code 一次。您需要在 Claude 组织中具有所有者或主要所有者角色，以及在 GHES 实例上创建 GitHub App 的权限。

引导式设置会生成 GitHub App 清单，并将您重定向到 GHES 实例以一键创建应用。如果您的环境阻止重定向流程，可以使用[替代手动设置](#manual-setup)。

**1. 打开 Git 提供商设置**

转到 [**Organization settings > Git providers**](https://claude.ai/admin-settings/source-control#github-enterprise) 并找到 GitHub Enterprise 部分。

**2. 启动引导式设置**

点击 **Connect**，如果已连接某个实例，则点击 **Add instance**，然后选择 **Set up automatically**。输入连接的显示名称（最多 20 个字符）和您的 GHES 主机名，例如 `github.example.com`。如果您的 GHES 实例使用自签名或私有证书颁发机构，请在可选字段中粘贴 CA 证书。

**3. 创建 GitHub App**

点击 **Continue to GitHub Enterprise**。您的浏览器会重定向到您的 GHES 实例，并显示预填充的应用清单。审查配置并点击 **Create GitHub App**。GHES 会将您重定向回 Claude，应用凭据将自动存储。

**4. 在您的仓库上安装应用**

从您的 GHES 实例上的 GitHub App 页面，在您希望 Claude 访问的仓库或组织上安装应用。您可以从一个子集开始，稍后再添加更多。

**5. 启用 Code Review**

转到 [claude.ai/admin-settings/claude-code](https://claude.ai/admin-settings/claude-code)，使用与 github.com 相同的配置为您的 GHES 仓库启用 [Code Review](https://code.claude.com/docs/zh-CN/code-review-setup.md)。

### GitHub App 权限

清单使用以下权限和 webhook 事件配置 GitHub App，这些权限和事件共同涵盖云端会话、Code Review、Claude Security 和插件市场：

| 权限 | 访问 | 用途 |
| :- | :- | :- |
| Contents | 读写 | 克隆仓库和推送分支 |
| Pull requests | 读写 | 创建 PR 和发布审查评论 |
| Issues | 读写 | 响应 issue 中的提及 |
| Checks | 读写 | 发布 Code Review 检查运行 |
| Actions | 读 | 读取 CI 状态以进行自动修复 |
| Commit statuses | 读 | 从报告提交状态而非检查运行的提供商读取 CI 状态 |
| Repository hooks | 读写 | 当 [**Organization settings > Plugins & skills**](https://claude.ai/admin-settings/skills?tab=marketplaces) 中的某个市场开启 **Sync automatically** 时，在插件市场仓库上创建 webhook |
| Metadata | 读 | GitHub 对所有应用的要求 |
| Organization members | 读 | 与 github.com 上的 Claude GitHub App 保持一致，该应用使用此权限在链接安装时检查连接用户的组织角色 |

应用订阅 `pull_request`、`issue_comment`、`pull_request_review_comment`、`pull_request_review`、`check_run` 和 `status` 事件。

GitHub 仅在创建应用时应用清单，因此从早期版本的清单创建的应用会保留创建时的权限和事件。如果您的应用缺少上述任何权限或事件，请在您的 GHES 实例上的应用设置中添加它们。GitHub 随后会要求每个安装的所有者批准新权限，在他们批准之前，安装将保留其旧权限。

### 手动设置

如果引导式重定向流程被您的网络配置阻止，请点击 **Connect** 或 **Add instance**，然后选择 **Add manually** 而不是 **Set up automatically**。在您的 GHES 实例上创建具有[上述权限和事件](#github-app-permissions)的 GitHub App，然后在表单中输入连接详情：显示名称、您的 GHES 主机名和可选端口，以及应用的 ID、客户端 ID、客户端密钥、webhook 密钥和私钥。表单还接受可选的自定义 CA 证书和只读副本主机名。

Claude 会在您保存连接时生成应用的 webhook URL。点击 **Add configuration** 后，打开连接的 **More options** 菜单，选择 **Copy webhook URL**，并将 URL 粘贴到您的 GHES 实例上的应用 webhook 设置中。使用您在表单中输入的相同 webhook 密钥。

### 网络要求

对于 Anthropic 托管的会话，您的 GHES 实例必须可从 Anthropic 基础设施访问，以便 Claude 可以克隆仓库和发布审查评论。如果您的 GHES 实例位于防火墙后面，请将 Anthropic 的[出站 IP 地址](https://platform.claude.com/docs/en/api/ip-addresses#outbound-ip-addresses)加入允许列表。[自托管环境](https://code.claude.com/docs/zh-CN/self-hosted-environments-deploy.md#configure-git)中的会话则从您的网络内部克隆，除非运行器选择使用 [Anthropic git 代理](https://code.claude.com/docs/zh-CN/self-hosted-environments-deploy.md#use-the-anthropic-git-proxy)，该代理从 Anthropic 一侧获取，因此需要相同的可达性。

托管的会话前流程（例如仓库选择器）会在会话启动前在 Anthropic 一侧运行。即使会话在自托管环境中运行，这些流程也需要您的 GHES 实例可从 Anthropic 基础设施访问。[SCM 连接器](https://code.claude.com/docs/zh-CN/self-hosted-environments-reference.md#scm-connector-flags)不可用，因此这些流程无法访问仅在内部可路由的 GHES 主机。

## 开发人员工作流

Claude Code 从您工作目录中的 git 远程自动检测您的 GHES 主机名。

像往常一样从您的 GHES 实例克隆存储库，将 `github.example.com` 和存储库路径替换为您的 GHES 主机名和存储库：

```bash theme={null}
git clone git@github.example.com:platform/api-service.git
cd api-service
```

然后启动云会话。Claude 从您的 git 远程检测 GHES 主机，并通过您组织的配置实例路由会话：

```bash theme={null}
claude --cloud "Add retry logic to the payment webhook handler"
```

会话从 GHES 克隆您的仓库，并将更改推送回分支。在 [claude.ai/code](https://claude.ai/code) 监控进度。有关完整的云端会话工作流（包括 diff 审查、自动修复和云端定时任务），请参阅[在云端使用 Claude Code](https://code.claude.com/docs/zh-CN/claude-code-on-the-web.md)。

### 将会话 Teleport 到您的终端

使用 `claude --teleport` 将云会话拉入您的本地终端。Teleport 在获取分支和加载会话历史之前验证您在同一 GHES 存储库的检出中。有关详细信息，请参阅 [teleport 要求](https://code.claude.com/docs/zh-CN/claude-code-on-the-web.md#teleport-requirements)。

## GHES 上的插件市场

在您的 GHES 实例上托管插件市场，以在您的组织中分发内部工具。市场结构与 github.com 托管的市场相同，但安装方式因您添加市场的位置而异，并且凭证在不同的界面上有所不同：

| 界面 | 安装方式 | 每个用户需要什么 |
| :- | :- | :- |
| Claude Code CLI 和桌面应用 | Claude Code 使用机器现有的 git 凭证克隆市场存储库 | 从其机器对您的 GHES 主机的 Git 访问权限 |
| 托管设置（`extraKnownMarketplaces`） | Claude Code 注册条目并使用机器现有的 git 凭证克隆存储库 | 从其机器对您的 GHES 主机的 Git 访问权限 |
| claude.ai 组织插件设置 | 所有者选择 GHES 实例作为源；Anthropic 的后端使用来自 [admin setup](#admin-setup) 的 GitHub App 获取并同步存储库 | 添加后每个用户无需任何操作。添加它的所有者需要连接自己的 GitHub Enterprise 账户作为访问检查，并且 GitHub App 必须安装在市场存储库上 |
| claude.ai 用户设置 | Anthropic 的后端使用提交用户的 GitHub Enterprise 连接获取存储库 | 连接到 Claude 的自己的 GitHub Enterprise 账户 |
| Cloud sessions | Cloud sessions 在会话沙箱内克隆市场。沙箱只有在会话的存储库位于同一实例上时才能访问您的 GHES 实例，其 git 凭证的范围限于会话的存储库 | 对于 GHES 托管的市场不可靠：与会话存储库不同的主机无法访问，即使是同一实例的安装也可能失败。改用 CLI、托管设置或 claude.ai |

> [!WARNING]
>
> 当从用户设置添加市场时，claude.ai 上的 GitHub Enterprise 连接是按用户的。[admin setup](#admin-setup) 将您的 GHES 实例连接到您的组织，但它不连接单个用户账户：每个从自己的设置添加 GHES 市场的用户必须首先连接自己的 GitHub Enterprise 账户，一个用户的连接（包括所有者的）不会覆盖任何其他人。由所有者在组织插件设置中添加的市场不会对用户施加此要求，因为持续的获取使用组织的 GitHub App。添加市场的所有者仍然需要在添加时连接自己的 GitHub Enterprise 账户。

### 添加 GHES 市场

`owner/repo` 简写始终解析为 github.com。对于 GHES 托管的市场，使用完整的 git URL，将 `github.example.com` 和存储库路径替换为您自己的。建议使用 HTTPS URL：

```bash theme={null}
/plugin marketplace add https://github.example.com/platform/claude-plugins.git
```

如果机器已经信任您的 GHES 主机，SSH URL 也可以工作：

```bash theme={null}
/plugin marketplace add git@github.example.com:platform/claude-plugins.git
```

Claude Code 以非交互方式运行 git，并拒绝连接到不在机器 `known_hosts` 文件中的主机的 SSH 连接。带有 git 凭证助手的 HTTPS URL 避免了 `known_hosts` 要求。

有关构建市场的完整指南，请参阅 [创建和分发插件市场](https://code.claude.com/docs/zh-CN/plugins/create-marketplace.md)。

### 使用托管设置预注册 GHES 市场

`extraKnownMarketplaces` 设置预注册市场，以便开发人员无需手动设置即可获得它。它可以从 [任何设置文件](https://code.claude.com/docs/zh-CN/settings-reference.md#extraknownmarketplaces) 工作，包括存储库的 `.claude/settings.json`；托管设置在整个组织范围内提供它：

```json theme={null}
{
  "extraKnownMarketplaces": {
    "internal-tools": {
      "source": {
        "source": "git",
        "url": "https://github.example.com/platform/claude-plugins.git"
      }
    }
  }
}
```

Claude Code 在本地安装这些市场：它注册每个条目并使用机器现有的 git 凭证克隆存储库。此路径不经过 claude.ai，因此不需要按用户的 GitHub Enterprise 连接。为了成功推出：

* **使用完整的 git URL。** `owner/repo` 简写始终解析为 github.com，无法引用 GHES 主机。
* **优先使用 HTTPS URL。** SSH 克隆在不信任您的 GHES 主机密钥的机器上失败。带有您组织标准 git 凭证助手的 HTTPS URL 在任何配置了凭证的机器上都可以工作。
* **确认每台机器都可以从您的 GHES 主机克隆。** 如果机器缺少凭证，市场会被注册但永远不会安装，其插件报告为未找到而不是提示输入凭证。
* **确认设置到达每台机器。** 托管设置文件仅在部署到的机器上生效，例如通过您的设备管理系统。有关文件位置，请参阅 [部署托管设置](https://code.claude.com/docs/zh-CN/managed-settings.md#delivery-mechanisms)。

### 在托管设置中将 GHES 市场加入白名单

如果您的组织使用 [托管设置](https://code.claude.com/docs/zh-CN/settings.md) 来限制开发人员可以添加哪些市场，请使用 `hostPattern` 源类型来允许来自您的 GHES 实例的所有市场，而无需枚举每个存储库。有关每个平台上的文件位置，请参阅 [部署机制](https://code.claude.com/docs/zh-CN/managed-settings.md#delivery-mechanisms)。将 JSON 添加到您的 `managed-settings.json` 文件或等效的 MDM 策略：

```json theme={null}
{
  "strictKnownMarketplaces": [
    {
      "source": "hostPattern",
      "hostPattern": "^github\\.example\\.com$"
    }
  ]
}
```

有关完整的架构，请参阅 [strictKnownMarketplaces](https://code.claude.com/docs/zh-CN/settings-reference.md#strictknownmarketplaces) 和 [extraKnownMarketplaces](https://code.claude.com/docs/zh-CN/settings-reference.md#extraknownmarketplaces) 设置参考。

## 限制

一些功能在 GHES 上的行为与 github.com 上不同。[功能表](#what-works-with-github-enterprise-server) 总结了支持；本部分涵盖了解决方法。

* **`/install-github-app` 命令**：改为在 claude.ai 上遵循 [管理员设置](#admin-setup) 流程。如果您还想在 GHES 上使用 GitHub Actions 工作流，请手动调整 [示例工作流](https://github.com/anthropics/claude-code-action/blob/main/examples/claude.yml)。
* **GitHub MCP server**：改为使用为您的 GHES 主机配置的 `gh` CLI。运行 `gh auth login --hostname github.example.com` 进行身份验证，然后 Claude 可以在会话中使用 `gh` 命令。

## 故障排除

### 云会话无法克隆存储库

如果 `claude --cloud` 因克隆错误而失败，请验证 Owner 已完成您的 GHES 实例的设置，并且 GitHub App 已安装在您正在处理的存储库上。与连接该实例的 Owner 确认在 Claude 设置中注册的主机名与您的 git 远程中的主机名匹配。

如果会话因"Authentication failed while accessing the repository"而停止，您可能尚未连接自己的 GitHub Enterprise 账户。请从 [claude.ai/code](https://claude.ai/code) 上的仓库选择器进行连接，然后启动新会话。

### 市场添加因策略错误而失败

如果 `/plugin marketplace add` 因您的 GHES URL 而被阻止，您的组织已限制市场源。要求您的管理员在 [托管设置](#allowlist-ghes-marketplaces-in-managed-settings) 中为您的 GHES 主机名添加 `hostPattern` 条目。

### claude.ai 上的市场添加因 GitHub 访问错误而失败

如果从您的用户设置添加 GHES 市场失败并出现通用错误（如"无法添加市场"），请先检查您的 GitHub Enterprise 连接。这是当您自己的 GitHub Enterprise 账户未连接到 Claude 时出现的情况，即使您的组织的 GHES 实例已配置且其他用户已连接。该对话框不会指向 GitHub Enterprise 连接流程，"浏览"选项卡上的"连接到 GitHub"选项会登录到 github.com，这不会授予对 GHES 存储库的访问权限。

在以下任一位置连接您的 GitHub Enterprise 账户，然后再次添加市场：

* **仓库选择器**：在 [claude.ai/code](https://claude.ai/code) 上，仓库选择器为每个已配置的 GHES 实例提供连接选项。
* **Git providers 页面**：如果您是 Owner，请前往 [**Organization settings > Git providers**](https://claude.ai/admin-settings/source-control) 的 GitHub 部分并点击 **Connect**，或在已连接账户后点击 **Add organization**。在 **GitHub instance** 下选择 GHES 主机名，然后点击 **Connect**。

或者，要求 Owner 在组织插件设置中添加市场，这样可以消除每个用户的连接要求。

在其他 claude.ai 界面上，GHES 市场上的"找不到存储库。如果是私有的，需要 GitHub 访问"错误通常表示相同的缺失连接。通过上述路径之一连接您的 GitHub Enterprise 账户，然后重试。

### GHES 实例无法访问

如果审查或 Anthropic 托管的云会话超时，您的 GHES 实例可能无法从 Anthropic 基础设施访问。确认您的防火墙允许来自 Anthropic 的 [出站 IP 地址](https://platform.claude.com/docs/en/api/ip-addresses#outbound-ip-addresses) 的入站连接。[自托管环境](https://code.claude.com/docs/zh-CN/self-hosted-environments) 中的会话从您的网络内部访问 GHES，因此当其中一个无法克隆时，请改为检查运行器自己的网络路径。对于存储库选择器和其他托管的会话前流程，请参阅 [网络要求](#network-requirements)。

### 连接测试通过但第二项测试失败

第二项测试以 Anthropic 托管的云端会话的方式连接到您的 GHES 实例。根据您看到的消息，确定应检查防火墙还是重试：

* **`Connection test passed, but a second test failed.`**：如果您的实例位于防火墙之后，请允许 Anthropic 的所有 [出站 IP 地址](https://platform.claude.com/docs/en/api/ip-addresses#outbound-ip-addresses)，因为即使防火墙阻止了其中部分地址，连接测试也可能通过。您也可以在几分钟后重试。
* **`Connection test passed, but a second test could not run.`**：请在几分钟后重试。

要再次测试，请在 [**Organization settings > Git providers**](https://claude.ai/admin-settings/source-control#github-enterprise) 中打开该连接的 **More options** 菜单，然后选择 **Test connection**。

### 会话启动失败，显示 `Unable to get organization UUID`

Claude Code 无法从您的凭据中读取 claude.ai 组织。使用 `/login` 登录，使用您的 Team 或 Enterprise 组织中的账户，因为 GitHub Enterprise Server 支持仅限于这些计划。有关原因和此状态产生的其他消息，请参阅 [Unable to get organization UUID](https://code.claude.com/docs/zh-CN/claude-code-on-the-web.md#unable-to-get-organization-uuid)。

## 相关资源

这些页面更深入地涵盖了本指南中引用的功能：

* [网络上的 Claude Code](https://code.claude.com/docs/zh-CN/claude-code-on-the-web.md)：在云基础设施上运行 Claude Code 会话
* [代码审查](https://code.claude.com/docs/zh-CN/code-review.md)：自动化 PR 审查
* [插件市场](https://code.claude.com/docs/zh-CN/plugins/host-marketplace.md)：构建和分发插件目录
* [分析](https://code.claude.com/docs/zh-CN/analytics.md)：跟踪整个组织的 Claude Code 使用情况
* [托管设置](https://code.claude.com/docs/zh-CN/settings.md)：组织范围的策略配置
* [网络配置](https://code.claude.com/docs/zh-CN/network-config.md)：防火墙和 IP 白名单要求
