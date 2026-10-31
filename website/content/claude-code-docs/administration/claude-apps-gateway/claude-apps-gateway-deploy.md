> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Claude 应用网关部署和运维

> 向身份提供商注册网关，构建容器，在 Kubernetes 或 Cloud Run 上部署，并运维它：健康检查、密钥轮换、升级和安全。

> [!NOTE]
>
> **请先规划网关的网络。** 登录时，如果 Claude 应用网关的主机名解析为公共 IP 地址，Claude Code 会拒绝连接，即使该地址无法从互联网访问也是如此。
>
> Claude 应用网关可以向用户的机器推送设置，包括运行 shell 命令的 hook。此检查有助于防止用户意外登录到公共互联网上的恶意网关。也请不要让您自己的网关暴露在互联网上。
>
> 在选择网关的运行位置之前，请先选择网关的地址。通常这是一个私有地址，用户可以在内部网络上或通过 VPN 访问。如果您的内部网络使用公共 IPv4 地址段，您可以列出一个同时包含网关和用户机器的地址段。Claude Code 会将该匹配视为网关位于您内部网络上的标志。请参阅 [为网关选择地址](#choose-an-address-for-the-gateway)。如果以上两种方式都不适合您的网络，请联系您的 Anthropic 客户团队。

本页面涵盖运行 [Claude 应用网关](https://code.claude.com/docs/zh-CN/claude-apps-gateway) 的运维方面：在身份提供商 (IdP) 中注册 OAuth 客户端、将网关部署为容器，以及日常运行。关于网关在启动时读取的 `gateway.yaml` 文件中的每个选项，请参阅 [配置参考](https://code.claude.com/docs/zh-CN/claude-apps-gateway-config.md)。

生产部署按顺序遵循四个步骤，下面的部分与之相对应。前两个是您做出选择的地方；后两个是在运行后参考的材料。

1. [设置身份提供商](#identity-provider-setup)：注册 OAuth 客户端并查看 Okta、Entra 和 Google 的特定 IdP 说明
2. [部署网关](#deployment)：构建固定版本的容器镜像并在 Kubernetes、Cloud Run 或您自己的平台上运行它。本部分还涵盖成本、绕过、多网关和无服务器决策
3. [设置运维](#operations)：日志、健康探针、中断行为、密钥轮换和升级。当您连接监控和运行手册时参考的材料
4. [审查安全态势](#security)：数据流向何处、威胁模型和合规性答案。用于安全审查的参考

如果在此过程中登录或启动失败，请直接转到 [故障排除](#troubleshooting)，该部分按您看到的错误进行索引。

## 身份提供商设置

向身份提供商注册一个机密 OAuth/OpenID Connect (OIDC) Web 应用程序，使用单个重定向 URI `https://<gateway>/oauth/callback`，并将其分配给应该有网关访问权限的用户或组。网关使用该注册的客户端密钥向 IdP 进行身份验证；如果您的 IdP 改用[证书凭据](https://code.claude.com/docs/zh-CN/claude-apps-gateway-config.md#certificate-client-authentication)，则使用您上传到该注册的证书进行身份验证。

任何符合 OIDC 的 IdP 都可以工作：Okta、Microsoft Entra ID、Google Workspace、Keycloak、Dex、PingFederate 等。IdP 必须满足三个要求：

* 在生产环境中通过 HTTPS 提供 `/.well-known/openid-configuration`；网关接受 [`http://` 发行者](https://code.claude.com/docs/zh-CN/claude-apps-gateway-config#oidc)，本地环回发行者另外需要 `CLAUDE_GATEWAY_ALLOW_LOOPBACK=1`
* 支持授权代码流。PKCE（代码交换证明密钥）默认启用；对于不支持它的 IdP，使用 `oidc.use_pkce: false` 禁用它
* 在 id\_token 中返回 `email` 和可选的 `groups`，或使用 `oidc.userinfo_fallback: true` 从 userinfo 端点提供它们

对于私有 PKI，设置 `oidc.ca_cert_pem`。

一些提供商处理电子邮件和组声明的方式不同：

* **Okta**：位于 `https://example.okta.com` 的组织授权服务器返回一个省略 `email` 和 `groups` 的简化 id\_token，因此当您将其用作 `issuer` 时设置 `oidc.userinfo_fallback: true`。包含 id\_token 中 `email` 和可选 `groups` 的自定义授权服务器（如 `https://example.okta.com/oauth2/default`）直接发出它们，不需要回退。Okta 仅在 `oidc.scopes` 中请求 `groups` 作用域且应用的组声明过滤器允许时才发出 `groups`；`userinfo_fallback` 无法填充 IdP 未被要求的声明。
* **Microsoft Entra ID**：`issuer` = `https://login.microsoftonline.com/<tenant-id>/v2.0`。Entra 发出组对象 ID 而不是名称，因此在 `managed.policies.match.groups` 中使用 GUID，或使用应用角色获得人类可读的名称。如果您的租户在 `roles` 而不是 `groups` 下发出角色，设置 `oidc.groups_claim: roles`。
* **Google Workspace**：`issuer` = `https://accounts.google.com`。Google 的 id\_token 不包含组。要在 Google 作为 IdP 时使用基于组的 `allowed_groups` 或 `managed.policies`，配置 [`oidc.google_groups`](https://code.claude.com/docs/zh-CN/claude-apps-gateway-config#oidc)，它使用具有域范围委派的服务账户通过 Admin SDK Directory API 查找每个用户的组。没有它，使用 `oidc.allowed_email_domains` 进行成员资格门控，使用 `managed.policies.match.email_domain` 进行策略分配。Google 也忽略标准 `offline_access` 作用域。对于刷新令牌，设置 `oidc.scopes: [openid, profile, email]` 和 `oidc.extra_auth_params: { access_type: offline, prompt: consent }`。

> [!WARNING]
>
> 刷新令牌让网关可以在不将开发者发送回浏览器的情况下无声地续订开发者的会话。它们也驱动取消配置，因为当 IdP 禁用用户时，下一次刷新失败，会话在 `ttl_hours` 内结束。网关默认请求 `offline_access` 以获取刷新令牌。如果您的 IdP 需要明确同意离线访问，配置 OAuth 客户端以允许它。
>
> 如果您的 IdP 根本无法发出刷新令牌，网关仍然可以工作，但没有无声续订，因此开发者在会话过期时重新运行浏览器登录。为了防止每小时都发生这种情况，将 [`session.ttl_hours`](https://code.claude.com/docs/zh-CN/claude-apps-gateway-config#session) 提高到 `8` 或 `12`。权衡是取消配置延迟，因为没有刷新令牌，禁用的用户在更长的 TTL 过期之前保持访问权限。

## 部署

网关是一个单一的无状态 Linux 二进制文件，通过 Postgres 进行协调，因此按照您在环境中部署任何其他无状态服务的方式部署它。将其保持在您的网络内，让您的开发者可以通过 HTTPS 到达它，且它可以到达您的 IdP，并将网关视为任何持有生产凭据的服务。

除了运行位置外，还有一些决策塑造部署：

* **成本**：没有单独的许可证或按座位费用。网关是 `claude` 二进制文件的一部分，因此您通过现有承诺为推理付费，加上它运行的计算。
* **绕过**：网关不强制执行通过它的唯一模型路由。具有自己凭证的开发者仍然可以直接调用提供商，因此关闭该路径是网络策略决策，例如阻止到 `api.anthropic.com` 的出口，除了来自网关的。阻止该出口也会破坏 [WebFetch 域安全检查](https://code.claude.com/docs/zh-CN/data-usage.md#webfetch-domain-safety-check)，它从每个开发者的机器调用 `api.anthropic.com`。在托管策略中设置 `skipWebFetchPreflight: true` 以禁用它。
* **多个网关**：每个是一个单独的部署，有自己的配置，CLI 按网关主机名存储信任和凭证，因此团队可以使用不同的网关而不会冲突。要提供多个 OIDC 发行者，运行单独的实例。
* **无服务器**：Cloud Run 可以工作，如果您设置 `min-instances: 1` 以避免冷 OIDC 发现。Lambda 和 Cloud Functions 不行，因为网关是一个长时间运行的 HTTP 服务器。

这里的每个生产拓扑都在普通 HTTP 副本前面放置一个 L7 代理，如 Ingress、Cloud Run 的前端或 ALB。设置 [`listen.trusted_proxies`](https://code.claude.com/docs/zh-CN/claude-apps-gateway-config#listen) 为代理的源范围，以便网关从 `X-Forwarded-For` 读取客户端 IP。网关仅在 TCP 对等体受信任时才遵守该标头。[Google Cloud](https://code.claude.com/docs/zh-CN/claude-apps-gateway-on-gcp.md) 和 [AWS](https://code.claude.com/docs/zh-CN/claude-apps-gateway-on-aws.md) 工作示例为每个拓扑提供具体值。没有受信任的代理，每个请求似乎都来自代理的 IP，这会将按 IP 速率限制折叠为一个共享桶，并在审计事件中记录代理的 IP。

不要在网关的设备授权和令牌端点处重定向请求，例如在入口处使用 HTTP 到 HTTPS 或主机规范化重写。Claude Code 不会在这些请求上跟随重定向，因此会破坏登录和令牌刷新的入口规则。

给代理任何空闲超时时间长于网关的保活间隔，这取决于上游：

* 在除 `provider: anthropic` 之外的每个上游上，一旦流已经沉默约 15 秒，网关就会写入一个 SSE `ping`。
* 在 `provider: anthropic` 上，网关原样传递响应，包括 Anthropic API 自己的 ping。

默认值（如 ALB 的 60 秒）足以保持安静的流打开。[AWS 工作示例](https://code.claude.com/docs/zh-CN/claude-apps-gateway-on-aws.md#troubleshooting) 无论如何将其提高到一小时，其故障排除行涵盖早于 v2.1.229 的网关，这些网关在现在获得 ping 的上游上的安静期间没有发送任何内容。

### 为网关选择地址

Claude Code 通过以下两种方式之一接受网关的地址：

* **私有地址**：将网关置于内部负载均衡器或 VPN 之后，并使用仅解析为私有地址（如 RFC 1918 或 CGNAT `100.64.0.0/10`）的主机名。用户的机器可以使用任何地址。[私有网络前提条件](https://code.claude.com/docs/zh-CN/claude-apps-gateway#prerequisites) 列出了可接受的地址范围。
* **声明的地址块**：如果您的内部网络使用贵组织拥有的公共 IPv4 地址空间，请在 `gatewayInternalNetworks` 托管设置中列出该地址块。网关和用户的机器都必须位于该地址块内。请参阅 [允许网关使用您拥有的公共地址空间](https://code.claude.com/docs/zh-CN/claude-apps-gateway#allow-a-gateway-on-public-address-space-you-own)。

如果没有单个地址块能同时包含两者，请改为给网关分配私有地址。

### 容器镜像

围绕标准 Claude Code 版本中的本机 `claude` 二进制文件构建您自己的镜像：

1. 从固定版本下载您的镜像架构的 Linux 构建；请参阅 [安装特定版本](https://code.claude.com/docs/zh-CN/setup.md#install-a-specific-version) 了解下载 URL。
2. 根据版本的 GPG 签名 `manifest.json` 验证它，如 [二进制完整性和代码签名](https://code.claude.com/docs/zh-CN/setup.md#binary-integrity-and-code-signing) 中所述。
3. 将其复制到构建上下文中。

如果您的构建无法到达版本主机，请将版本镜像到您的内部注册表中，并固定您的舰队运行的版本。

除了二进制文件外，镜像还需要：

* **基于 glibc 的镜像**：glibc 构建的唯一动态依赖项是 glibc 库。基于 Musl 的镜像需要 `linux-x64-musl` 或 `linux-arm64-musl` 构建加上额外的包；请参阅 [Alpine Linux 设置](https://code.claude.com/docs/zh-CN/setup.md#alpine-linux-and-musl-based-distributions)。
* **可写状态目录**：网关以任何用户身份运行，但最小镜像没有可写的主目录。将 `CLAUDE_CONFIG_DIR` 设置为可写路径，如 `/tmp/.claude`。
* **容器命令**：`claude gateway --config /etc/claude/gateway.yaml`，配置文件以只读方式挂载，密钥作为环境变量提供；网关在 `listen.port` 上监听，默认为 `8080`。

### Kubernetes

将网关作为 Deployment 运行，就像任何无状态服务一样：

* 从 ConfigMap 挂载配置，从 Secret 挂载密钥；通过 `${file:/path/to/secret}` 或作为环境变量在 YAML 中引用密钥
* 在 Ingress 处终止 TLS 并将 `listen.public_url` 设置为 Ingress 主机名
* 将就绪探针指向 `GET /readyz`，将活跃探针指向 `GET /healthz`

有关 AWS 上的完整工作示例，涵盖 ECS Fargate 或 EKS、Amazon RDS 和 AWS Secrets Manager，请参阅 [在 AWS 上部署](https://code.claude.com/docs/zh-CN/claude-apps-gateway-on-aws.md)。

优先使用平台的工作负载身份而不是静态密钥；[`upstreams` 参考](https://code.claude.com/docs/zh-CN/claude-apps-gateway-config#upstreams) 有每个平台的设置详情。对于跨云配对，如 GKE 上的 Amazon Bedrock 上游，在上游的 `auth` 块中设置显式凭证。

### Cloud Run

按如下方式配置服务：

* 将 `listen.port` 保留在其默认值 `8080`，这与 Cloud Run 的默认 `PORT` 匹配，或设置 `port: ${PORT}`
* 将 `public_url` 设置为外部可达的源。对于生产，这通常是内部负载均衡器的主机名，因为 `/login` [拒绝公共地址](https://code.claude.com/docs/zh-CN/claude-apps-gateway#prerequisites)，而 `*.run.app` URL 解析为一个，所以单独的 Cloud Run URL 仅适用于 `curl` 或浏览器烟雾测试。例外是一个网络，其中 `*.run.app` 通过 Private Service Connect 和 Cloud DNS 私有区域私下解析；在该拓扑中，Cloud Run URL 是有效的 `public_url`。[Google Cloud 工作示例](https://code.claude.com/docs/zh-CN/claude-apps-gateway-on-gcp.md#deploy-the-gateway) 涵盖两者。
* 将配置作为密钥卷挂载
* 设置 `min-instances: 1` 以避免首次请求时的冷 OIDC 发现

有关 Google Cloud 上的完整工作示例，涵盖 Cloud Run 或 GKE、Cloud SQL 和 Secret Manager，请参阅 [在 Google Cloud 上部署](https://code.claude.com/docs/zh-CN/claude-apps-gateway-on-gcp.md)。

### 将网关 URL 推送到开发者机器

一旦网关开始提供服务，通过托管设置、MDM 或直接写入每个操作系统的 `managed-settings.json` 将 `forceLoginMethod`、`forceLoginGatewayUrl` 和 `parentSettingsBehavior: "merge"` 推送到每个开发者的机器。

一旦您部署密钥，Claude Code 停止使用机器上剩余的 API 密钥或 claude.ai 登录，因此计划与您的登录说明一起推送。[管理员策略需要 Cloud 网关登录](https://code.claude.com/docs/zh-CN/errors.md#administrator-policy-requires-a-cloud-gateway-sign-in) 描述开发者看到的消息。

请参阅 [每个机制存储策略的位置](https://code.claude.com/docs/zh-CN/managed-settings.md#where-each-mechanism-stores-the-policy) 了解文件路径，以及 [客户端托管设置](https://code.claude.com/docs/zh-CN/claude-apps-gateway-config.md#client-side-managed-settings) 了解 Claude Desktop `bootstrapUrl` 等效项。

### 大规模推出

登录按客户端 IP 地址进行速率限制，默认值适合小团队。每个地址每 10 分钟获得 30 次登录开始和 10 次代码提交。向数千名开发者的推出可能在第一个早上达到这些限制，原因有两个：

* **网关看不到您的负载均衡器后面。** 没有 [`listen.trusted_proxies`](https://code.claude.com/docs/zh-CN/claude-apps-gateway-config#listen)，每个开发者似乎都来自负载均衡器的地址并共享一个限制。首先设置它。网关在第一次忽略 `X-Forwarded-For` 标头时会记录警告。
* **许多开发者共享几个 NAT 或 VPN 出口地址。** 即使 `trusted_proxies` 正确，他们也共享这些地址的限制。提高 [`rate_limits`](https://code.claude.com/docs/zh-CN/claude-apps-gateway-config#http-tuning) 以适应。

要调整 `max`，将开发者数除以他们共享的出口地址数。估计在一个 `window_seconds` 周期内有多少人登录，默认为 10 分钟。然后将其加倍以覆盖重试和同时登录 Claude Code 和 Claude Desktop 的开发者。

例如，10,000 名开发者在 4 个出口地址后面在一小时内均匀登录。这是每个地址 2,500 名开发者，每 10 分钟约 420 名，您将其加倍并四舍五入到 1,000。下面的示例将两个限制都设置为 1,000：

```yaml theme={null}
rate_limits:
  device_authorization: { max: 1000, window_seconds: 600 }
  device_verify: { max: 1000, window_seconds: 600 }
```

`device_verify` 是阻止某人猜测另一个开发者登录代码的原因，因此仅在您的估计需要的范围内提高它。即使在这些限制下，代码也是来自 20 字符字母表的 8 个字符，并在 10 分钟后过期，因此猜测仍然不切实际；请参阅 [用户代码暴力破解抵抗](#user-code-brute-force-resistance)。

当您的 IdP 发出刷新令牌时，Claude Code 会无声地续订会话，因此您可以在推出后将限制放回。没有刷新令牌，开发者每 [`session.ttl_hours`](https://code.claude.com/docs/zh-CN/claude-apps-gateway-config#session) 再次登录。为该稳定速率调整两个限制的大小，并保持它们提高。

当达到限制时，Claude Code v2.1.274 或更高版本显示 `The gateway is limiting sign-in attempts right now`。v2.1.274 或更高版本的网关在验证页面上显示 `Too many attempts came from your network address`，并提供要检查的设置。它还写入一条 `sign-in refused` 日志行，命名要更改的设置。

## 运维

一旦网关开始提供流量，日常运维就是读取其日志、探测其健康状况，以及按您的计划轮换其密钥。下面的小节涵盖每一个，加上 Postgres 持有的内容以及升级和回滚的行为。

### 日志

网关向 stderr 写入两个流，都是 JSON 友好的：

* **审计事件**：每个安全相关事件一行 JSON。将 stderr 管道传输到您的日志聚合器。

  发出的事件包括 `config.load`、`session.mint`、`session.refresh`、`device.authorize`、`device.verify`、`device.callback`、`auth.denied`、`access.denied`、`access.public_client`、`inference`、`managed.serve`、`desktop_bootstrap.serve`、`desktop_bootstrap.denied`、`spend.blocked`、`admin.denied`、`admin.limit.upsert` 和 `admin.limit.delete`。字段因事件而异：

  * 成功的 mint 和 refresh 事件携带 `sub`、`email`、`client_ip` 和结果
  * `auth.denied` 和 `access.denied` 携带原因和客户端 IP，加上 `auth.denied` 的请求路径，因为在这些拒绝时不存在用户身份。两个 `access.denied` 原因改变事件携带的内容：
    * `xff_unparseable`：事件也携带无法读取的 `X-Forwarded-For` 条目
    * `client_ip_unknown`：事件不携带客户端 IP，因为连接没有对等地址，而设置了 `access_control` 列表
  * `access.public_client` 携带每个进程从公共地址到达的第一个请求的客户端 IP，而 `access_control.allow_cidrs` 为空。网关照常提供请求；该事件表示网关可能可从公共互联网访问。请参阅 [`access_control` 参考](https://code.claude.com/docs/zh-CN/claude-apps-gateway-config#http-tuning) 了解什么被视为公共以及推荐的允许列表。
  * `inference` 记录哪个上游提供了请求以及响应状态
  * `desktop_bootstrap.denied` 记录被拒绝的 Claude Desktop bootstrap 获取，包含原因（`not_configured`、`policy_not_opted_in` 或 `no_policy_matched`）和用户的身份
  * `admin.denied` 记录被拒绝的管理员 API 身份验证尝试，包括客户端 IP、方法、路径和原因，不包括呈现的密钥材料：当呈现了 `x-api-key` 但与配置的密钥不匹配时为 `invalid_key`，当仅呈现了 `Authorization` 标头且其未验证为 `admin.admin_groups` 中的网关会话时为 `bearer_rejected`，或当两个标头都未呈现时为 `no_credentials`
* **运维日志**：人类可读的 `[gateway]` 前缀行，用于启动、警告和上游错误。`CLAUDE_GATEWAY_LOG_LEVEL` 环境变量控制详细程度，接受 `debug`、`info`、`warn` 或 `error`，默认为 `info`。在 `debug` 时，每次登录和刷新也记录 id\_token 中声明的名称（不是值），加上当 `userinfo_fallback` 提供任何时 userinfo 声明的名称，因此您可以诊断 `email_claim` 和 `groups_claim` 设置而不记录 PII。它不影响审计事件，这些事件总是被发出。

### 健康

网关提供 `GET /healthz` 作为活跃探针和 `GET /readyz` 作为就绪探针。`/readyz` 验证存储是否可达。如果您设置了 [`store.readiness_grace_seconds`](https://code.claude.com/docs/zh-CN/claude-apps-gateway-config#store)，`/readyz` 在存储停止应答后最多继续报告就绪达到那么多秒。

两个端点都免除 `access_control.allow_cidrs`，因此探针在锁定的监听器上继续工作。

`/.well-known/oauth-authorization-server` 处的 OAuth 发现文档也仅在配置加载、OIDC 发现、上游客户端构造和 Postgres 迁移全部成功后才返回 `200`，因此它也充当端到端启动检查。

### 并发上游请求

默认情况下，每个网关副本最多同时向上游发送 256 个请求。流式响应在流结束前计入限制。

当副本达到限制时到达的请求在网关内等待一个空闲槽位。开发者看到一个响应缓慢启动或似乎挂起。在 `provider: anthropic` 上游上，等待时间超过 [`timeouts.upstream_ttfb_ms`](https://code.claude.com/docs/zh-CN/claude-apps-gateway-config#http-tuning) 的请求放弃该上游，当没有后续上游提供它时以 502 失败。

包含 `upstream requests:` 的启动日志行显示生效的限制。当副本的打开请求数超过限制时，它也最多每分钟记录一次包含 `client requests are open` 的警告。

要同时提供更多请求，您有两个选项：

* 添加副本。
* 提高每个副本上的限制。在网关容器上设置 `BUN_CONFIG_MAX_HTTP_REQUESTS` 环境变量为 1 到 65535 之间的整数，然后重启容器。

副本以约限制除以请求保持打开的平均秒数的请求速率填充其限制。例如，如果请求平均保持打开 10 秒，默认限制为 256 的副本以约每秒 26 个请求的速率填充它。

如果您在 CPU 上自动扩展，限制处的副本排队请求而不触发扩展，因此将目标设置在副本在记录 `client requests are open` 警告时显示的 CPU 级别以下。

> [!WARNING]
>
> 每个打开的请求在网关进程中持有内存，同时它流式传输，同时它等待一个槽位。如果您将限制保持在 256，过载副本上的内存仍然增长，因为等待请求保持其请求体。根据峰值时打开的请求数调整容器的内存，当您更改限制时观察内存。耗尽内存的副本被杀死并丢弃它持有的每个流。

### 使 `API_TIMEOUT_MS` 高于网关的首字节时间

如果您降低 [`API_TIMEOUT_MS`](https://code.claude.com/docs/zh-CN/network-config#streaming-idle-watchdogs) 以便更快地重试未得到应答的请求，请使其高于网关发送流式响应首批字节所需的时间。等待时间更长的请求会超时，同样等待那么久的每次重试也会超时，直到该轮次以 `Request timed out` 结束，除非[首字节截止时间](#retry-an-unanswered-request-sooner-with-the-first-byte-deadline)先将其结束。

* **`provider: anthropic`**：将 `API_TIMEOUT_MS` 设置为高于您的上游发送其第一个事件所需的最长时间。
* **其他所有上游**：将其设置为比您的上游返回响应标头所需的最长时间多出 20 秒以上。网关在这些标头之后约 15 秒发送其第一个 [keepalive `ping`](#deployment)，除非上游的第一个事件更早到达。早于 v2.1.229 的网关不发送 ping，因此在那里请使用 `provider: anthropic` 规则。

在此时间之上，还要考虑任何[等待空闲槽位](#concurrent-upstream-requests)的时间，以及在[故障转移](https://code.claude.com/docs/zh-CN/claude-apps-gateway-config.md#multiple-upstreams)之前等待已宕机上游的时间。同时也要使该值高于最长的[非流式请求](https://code.claude.com/docs/zh-CN/network-config.md#streaming-idle-watchdogs)。

### 使用首字节截止时间更快地重试未得到应答的请求

要让 Claude Code 更快地重试未得到应答的流式请求，请在开发者的机器上设置 [`CLAUDE_STREAM_FIRST_BYTE_TIMEOUT_MS`](https://code.claude.com/docs/zh-CN/env-vars)，为其设定首字节截止时间。该截止时间限制首次尝试等待响应标头的时长。没有它，该尝试可能等待 5 到 6 分钟。需要开发者机器上的 Claude Code v2.1.307 或更高版本。

将该变量设置为以毫秒为单位的正整数，可以在[网关策略的 `env` 块](#deliver-the-value-in-a-policy)中或开发者的 shell 中设置。请使该值高于网关最长的[首字节时间](#keep-api_timeout_ms-above-the-gateways-time-to-first-byte)，否则 Claude Code 可能会中止并重新发送标头返回较慢的健康请求。此示例在开发者的 shell 中设置 60 秒：

```bash theme={null}
export CLAUDE_STREAM_FIRST_BYTE_TIMEOUT_MS=60000
```

未得到应答的重新发送可能等待 5 到 6 分钟，直到[响应体空闲超时](https://code.claude.com/docs/zh-CN/network-config.md#streaming-idle-watchdogs)将其结束，然后 Claude Code 在[重试预算](https://code.claude.com/docs/zh-CN/errors.md#tune-retry-behavior)内重试它。如果您降低 `API_TIMEOUT_MS` 只是为了更快地重试，请移除该值，否则一次未得到应答的重新发送就可能以 [No response from API](https://code.claude.com/docs/zh-CN/errors.md#no-response-from-api) 结束该轮次。

要确认截止时间已触发，请让开发者在启用[调试日志](https://code.claude.com/docs/zh-CN/network-config.md#verify-your-configuration)的情况下启动 Claude Code，然后在日志中搜索 `[first-byte] no response headers`。

### 中断行为

如果 Postgres 宕机，网关本身继续为已登录的开发者提供服务，新登录失败。开发者是否真的继续工作取决于您的编排器如何处理就绪：

* **现有会话**：持有者令牌使用 JWT 密钥在本地验证，会话刷新不接触存储，网关进程仍然可以提供推理
* **新登录**：失败直到 Postgres 恢复，因为设备流及其速率限制计数器存在于 Postgres 中
* **[支出限制执行](https://code.claude.com/docs/zh-CN/claude-apps-gateway-spend-limits.md#postgres-availability)**：在中断期间默认失败开放，因此推理仍然流动；如果您宁愿阻止而不是无计量运行，将其翻转为失败关闭
* **就绪**：默认情况下，`/readyz` 在 Postgres 无法访问时立即报告未就绪，因此每个副本一次失败其就绪检查。在流量仅到达通过检查的副本的地方，所有流量，包括网关仍然可以提供的推理，在 Postgres 恢复前失败。`/healthz` 上的活跃探针在整个过程中继续通过。

如果您的 IdP 宕机，现有会话工作直到 `ttl_hours`，新登录失败。会话刷新获得重试答案并在 IdP 恢复后成功。如果您的 IdP 有频繁的维护窗口，设置更长的 `ttl_hours`。

#### 就绪宽限期

要让已登录的开发者在短 Postgres 中断（如数据库故障转移）期间继续工作，将 [`store.readiness_grace_seconds`](https://code.claude.com/docs/zh-CN/claude-apps-gateway-config#store) 设置为比故障转移花费的时间更长，例如 `300`。启用支出限制并使用默认失败开放行为，通过保持就绪的副本的请求在 Postgres 恢复前无计量，因此将值保持为低至覆盖您的故障转移。如果您设置了 [`enforcement.fail_closed_on_error: true`](https://code.claude.com/docs/zh-CN/claude-apps-gateway-config#enforcement)，网关拒绝已登录开发者的推理，带有 `429` `spend limit unavailable` 消息，直到 Postgres 恢复，即使副本仍然通过其就绪检查。

该设置需要网关服务器上的 Claude Code v2.1.282 或更高版本。较早的网关在找到密钥时拒绝启动，因此在添加它之前升级每个副本。[升级](#upgrades) 涵盖回滚。

如果您将就绪探针指向 `/healthz` 而不是，副本也在中断期间继续通过它，但 `/healthz` 永远不报告未就绪，因此 Postgres 连接未恢复的副本继续通过。

### JWT 密钥轮换

通过三个步骤轮换签名密钥，以便现有会话保持有效：

1. 生成一个新密钥。将其前置到 `session.jwt_secret` 数组。
2. 滚动部署。新令牌使用新密钥签名；旧令牌仍然验证。
3. 在 `ttl_hours` 加上一个边距之后，移除旧密钥并再次滚动。

轮换也是在过期前强制会话退出的唯一方式：持有者令牌针对 JWT 密钥在本地验证，因此没有按会话撤销。直接替换密钥，不在数组中保留旧密钥，一次使每个未完成的会话失效。对于个人离职，在您的 IdP 中取消配置用户；他们的会话在 `ttl_hours` 内结束。

### Postgres

网关将其状态存储在 PostgreSQL 数据库中：

* **数据库**：PostgreSQL 本身，自托管或托管均可，版本为[最低版本](https://code.claude.com/docs/zh-CN/claude-apps-gateway#prerequisites)或更高。仅实现 Postgres 协议的数据库（例如分布式 SQL 数据库）不受支持。
* **地址**：`store.postgres_url` 接受一个主机。如果数据库有多个节点，请使用位于它们前面的地址，例如您的托管服务的端点、负载均衡器或虚拟 IP。设置一个比故障转移所需时间更长的[就绪宽限期](#readiness-grace-period)。

网关持有五个数据表加上一个 `_migrations` 表，全部由其启动时迁移创建：

| 表 | 内容 | 保留 |
| - | - | - |
| `kv` | 设备授权（10 分钟 TTL）和速率限制计数器 | 每行 TTL |
| `spend` | 每个主体期间至今支出计数器，以美分计 | `admin.spend_retention_months`，默认 13 |
| `spend_limits` | 配置的支出上限 | 直到通过 API 删除 |
| `admin_audit` | 管理员 API 变更跟踪 | `admin.audit_retention_days`，默认 365 |
| `principal_emails` | 每个主体的最后看到的电子邮件、显示名称和 IdP 组。包含 PII。 | `admin.identity_retention_days` 自上次活动以来，默认 90 |

一个 30 秒的循环过期 `kv` 行超过其 TTL，一个每小时的扫描在支出表上强制保留窗口，因此没有什么无限增长。没有 [支出限制](https://code.claude.com/docs/zh-CN/claude-apps-gateway-spend-limits.md) 配置，只有 `kv` 被写入。网关在启动时应用其自己的架构迁移，在每次升级时也是如此，因此其数据库角色需要创建和修改表的权限。将其指向专用于网关的数据库或架构，以保持该授权范围狭窄。

使用支出限制，丢失的数据库意味着丢失支出跟踪和上限，不仅仅是开发者重新登录，因此运行定期备份。要立即删除一个离职的开发者而不是等待保留，直接运行 `DELETE FROM principal_emails WHERE principal = '<sub>'`；这移除了唯一持有其电子邮件、名称和组的表。`spend` 和 `admin_audit` 行仅引用伪匿名 OIDC `sub`。

#### 确定连接池大小

每个副本最多打开 [`store.max_connections`](https://code.claude.com/docs/zh-CN/claude-apps-gateway-config#store) 个 Postgres 连接，默认为五个。请使副本数 × `store.max_connections` 不超过数据库为网关剩余的连接数：

* **副本**：统计同时运行的最大数量。从您的自动扩展最大值开始，如果您不使用自动扩展，则从副本数开始，再加上滚动更新在停止旧副本之前启动的额外副本。在使用默认设置的 Amazon ECS 上以及在 Cloud Run 上，额外副本可能与原数量一样多。
* **剩余连接**：取数据库的 `max_connections`，然后减去数据库保留的连接以及其他客户端持有的连接。PostgreSQL 保留 `superuser_reserved_connections`，默认为三个。托管服务可能保留更多，因此请查看其文档。

例如，假设自动扩展最大值为六，滚动更新最多启动两个额外副本，并使用默认的五个连接池。这需要 (6 + 2) × 5 = 40 个连接。一个 `max_connections` 为 50 且其他客户端持有两个连接的数据库剩余 50 − 3 − 2 = 45 个连接，因此总数可以容纳。

如果您的总数无法容纳，请更改以下其中一项：

* **数据库**：提高其 `max_connections`
* **副本**：运行更少的副本，或让您的编排器在滚动更新中启动更少的额外副本
* **连接池**：降低 `store.max_connections`，但在启用[支出限制](https://code.claude.com/docs/zh-CN/claude-apps-gateway-spend-limits.md)时，请使其足够大以满足每个副本的负载。否则支出检查可能在等待连接时[超时](https://code.claude.com/docs/zh-CN/claude-apps-gateway-spend-limits.md#postgres-availability)，网关会记录 `spend check failed: spend precheck timed out`。

### 升级

副本是无状态的，因此滚动重启不会丢失任何网关状态。网关在启动时运行架构迁移，这意味着部署新二进制文件会自动迁移数据库。并发副本在 Postgres 咨询锁上序列化，因此只有一个应用每个迁移。

当您的编排器使用 `SIGTERM` 停止副本时，如在滚动重启或缩减中，网关停止接受新连接，让已在途中的请求和流完成后再退出。它最多等待 25 秒，称为排空窗口，然后关闭仍然打开的任何东西。`SIGINT`（如终端中的 Ctrl+C）启动相同的排空，排空期间的第二个信号关闭打开的请求并立即退出。排空需要网关 v2.1.274 或更高版本。

长生成可以流式传输数分钟。在 Kubernetes 和 Amazon ECS 上，将这两个一起提高以给这些流更多时间：

* **排空窗口**：在网关容器上设置 `CLAUDE_GATEWAY_DRAIN_TIMEOUT_MS` 环境变量为正整数毫秒数，如 `120000`。网关忽略任何其他形式的值，如 `120s`，并保持 25 秒的默认值
* **您的编排器的宽限期**：Kubernetes 上的 `terminationGracePeriodSeconds`，或 Amazon ECS 上的 `stopTimeout`

宽限期在两个平台上默认为 30 秒。将其保持至少比排空窗口长 5 秒，否则编排器在排空完成前杀死网关。在 Kubernetes 上，也添加任何 `preStop` 钩子的持续时间，因为宽限期在钩子运行之前而不是网关接收 `SIGTERM` 时开始计数。

您的平台也可能限制排空可以运行多长时间：

* **Amazon ECS on Fargate**：`stopTimeout` 最多允许 120 秒
* **Cloud Run**：在 `SIGTERM` 后 10 秒停止实例，因此打开的流在那里最多获得 10 秒，无论排空窗口是什么

当排空窗口结束时仍有请求打开，网关记录一个包含 `drain window over after` 的警告，计数它切断的请求，并命名两个要提高的设置。

迁移是仅追加的，因此回滚到知道较少迁移的先前二进制文件是安全的；它忽略额外的行。回滚也重新验证 YAML 针对较旧二进制文件的架构，因此采用由较新版本引入的密钥的配置在较旧版本上启动失败。在回滚前移除新密钥。

因为您在自己的镜像中固定网关的版本，新 Claude Code 版本中的修复，包括安全修复，仅在您更新固定并重新部署时才到达您的部署。在您用于持有生产凭证的其他服务的相同修补周期中包括网关。

## 安全

本部分回答安全审查提出的问题：什么数据流经网关以及它去哪里，设计防御哪些攻击，以及哪些答案属于合规性问卷。

### 数据流

| 数据 | 路径 | 由网关发送给 Anthropic |
| - | - | - |
| 推理（提示、完成） | CLI → 网关 → 您的上游 | 仅当 Anthropic API 是配置的上游时 |
| 遥测（OTLP 指标，加上 [选择加入日志和跟踪](https://code.claude.com/docs/zh-CN/claude-apps-gateway-config.md#telemetry)） | CLI → 网关 → 您的收集器 | 从不 |
| 身份（电子邮件、组、sub） | IdP → 网关 → CLI；CLI 在 OTLP 导出上标记它。如果您打开 [`forward_user_identity`](https://code.claude.com/docs/zh-CN/claude-apps-gateway-config#per-user-identity-headers-for-a-proxy-you-run)，网关也会将开发者的电子邮件和 IdP 主体作为标头发送到您的代理 | 从不 |
| 托管设置 | 您的网关 YAML → CLI | 从不 |
| 审计日志 | 网关 stderr → 您的聚合器 | 从不 |

### 威胁模型摘要

网关位于您的网络周边内，但个别开发者笔记本电脑不被视为受信任。设计通过三种方式考虑这一点：

* 开发者持有短期 JWT 而不是原始上游密钥。CLI 到网关的腿使用 RFC 8628 设备授权，网关与 IdP 的授权代码交换在默认配置中运行 PKCE，因此拦截的 IdP 授权代码是无用的。
* 设备验证页面强制执行同源 POST 和每个 RFC 8628 §5.1 的每 IP 速率限制。请参阅 [用户代码暴力破解抵抗](#user-code-brute-force-resistance)。
* 网关对您的 IdP、您的 OTLP 收集器和 `provider: anthropic` 上游的请求通过服务器端请求伪造 (SSRF) 防护，解析 DNS，阻止链接本地和云元数据地址加上默认的本地环回，并将连接固定到解析的 IP，因此操作员影响的 URL 无法重定向到云元数据端点。RFC 1918 私有范围被故意允许，因为 IdP 和 OTLP 收集器通常存在于私有 IP 上。对于其他提供商，网关在加载配置时拒绝命名这些地址或元数据主机名之一的 `base_url`，然后提供商的 SDK 连接而不进行 DNS 检查。

  如果您打开 [仅代理出口](https://code.claude.com/docs/zh-CN/claude-apps-gateway-config.md#proxy-only-egress)，该地址检查会移到您的转发代理：网关交付主机名，代理的允许列表必须拒绝这些目的地。

  仅当网关必须合法到达的某些内容存在于本地环回时（例如本地开发 IdP 或 `localhost` 上的 sidecar OTLP 收集器），在网关的环境中设置 `CLAUDE_GATEWAY_ALLOW_LOOPBACK=1`。该变量为每个操作员配置的 URL 放宽本地环回块，也跳过启动时警告，该警告检查 pod 是否可以到达云元数据端点，因此更倾向于为收集器提供其自己的内部地址。

如果您添加自己的出口控制，网关必须在使用实例元数据凭证（如工作负载身份）时到达元数据服务器。

两个威胁超出范围，因为它们是您的基础设施来保护：

* **受损的网关主机**：主机既持有上游凭证，又向每个连接的开发者分发 [托管设置](https://code.claude.com/docs/zh-CN/claude-apps-gateway-config.md#managed)，因此对网关配置的控制与对您的 MDM 的控制相当。CLI 的 [批准对话框](https://code.claude.com/docs/zh-CN/server-managed-settings.md#approval-memory) 用于 shell 能力设置限制无声更改，但不替代主机安全。
* **恶意 OIDC 提供商**：提供商签署网关信任的 id\_tokens，因此它可以声称任何身份。审查和保护您的 IdP 是您的责任。

### 用户代码暴力破解抵抗

开发者在 `/device` 验证页面中输入的 `user_code` 是从 20 字符字母表中抽取的 8 个字符，产生 20⁸ 或约 2.56×10¹⁰ 个组合，在 10 分钟后过期。

网关在设备授权端点上应用按 IP 速率限制，可通过 [`rate_limits`](https://code.claude.com/docs/zh-CN/claude-apps-gateway-config#http-tuning) 配置。如果许多开发者从单个共享公司 NAT 地址登录，提高限制。[大规模推出](#large-rollouts) 显示如何调整它们的大小。限制仅适用于登录流，不适用于推理。

### 合规性态势

* **数据驻留**：网关自己的数据平面除非 Anthropic API 是配置的上游，否则不向 Anthropic 发送任何内容；当它是时，您现有的数据处理协议适用于推理路径。遥测、审计、身份和设置仅去往您配置的目的地。
* **主机进程流量**：主机进程是 Claude Code CLI。`claude gateway` 在与 Amazon Bedrock 和 Google Cloud 的 Agent Platform 部署相同的第三方规则下运行，不向 Anthropic 发送任何内容。在 v2.1.227 之前，主机进程发送启动遥测，例如产品版本和平台，在容器环境中设置 `CLAUDE_CODE_DISABLE_NONESSENTIAL_TRAFFIC=1` 关闭。这些版本也在启动时发送一个 `HEAD` 请求，没有正文或凭证，到 `https://api.anthropic.com` 上的 `/api/hello`，或在环境设置时的 `ANTHROPIC_BASE_URL` 上，除非环境也设置了代理变量（如 `HTTPS_PROXY`）或 mTLS 客户端证书。它们忽略了响应，因此在出口防火墙处阻止该请求不影响网关。
* **客户端分析**：CLI 在登录到网关时禁用自己的使用分析和错误报告。在第一次登录之前，CLI 仍然向 Anthropic 发送启动事件，包括在托管设置强制网关登录的机器上。要保持这些关闭，在强制网关登录的相同 [客户端托管设置](https://code.claude.com/docs/zh-CN/claude-apps-gateway-config.md#client-side-managed-settings) 中传递 [`DISABLE_TELEMETRY`](https://code.claude.com/docs/zh-CN/managed-settings#turn-telemetry-off-for-your-organization)。
* **错误报告**：每当 CLI 的模型请求去往 Anthropic 的第一方 API 以外的任何端点（如 Amazon Bedrock 或自定义 `ANTHROPIC_BASE_URL`）时，CLI 关闭错误报告。
* **客户端机器**：开发者的 CLI 仍然向 Anthropic 发送 WebFetch 主机名检查和版本检查，除非设置了 `CLAUDE_CODE_DISABLE_NONESSENTIAL_TRAFFIC=1` 和 `skipWebFetchPreflight: true`。[插件市场请求](#plugin-marketplace-requests) 有各自的关闭开关。请参阅 [数据使用](https://code.claude.com/docs/zh-CN/data-usage.md)。
* **调查评分**：在登录到网关时，CLI 禁用 Anthropic 绑定的评分上传以及分析流，因此它不向 Anthropic 发送评分。
* **成绩单共享**：在调查的成绩单共享提示上选择"是"会在 `~/.claude/feedback-bundles/` 下写入本地文件，而不是上传到 Anthropic。
* **客户端更新**：更新检查与网关流量分开。通过您自己的分发固定版本，如果笔记本电脑不得获取版本，设置 `DISABLE_UPDATES`。`DISABLE_AUTOUPDATER` 仅停止后台更新，而 `claude update` 仍然有效。
* **TLS**：在生产中通过 HTTPS 提供 `public_url`，要么从网关自己的监听器通过 `listen.tls`，要么从 TLS 终止入口在普通 HTTP 副本前面，在两种情况下都设置 `listen.public_url`。网关不拒绝普通 HTTP。IdP 必须在生产中提供 HTTPS，Postgres 支持 `?sslmode=require`。在您的入口处设置 `Strict-Transport-Security`。
* **漏洞披露**：遵循 [报告安全问题](https://code.claude.com/docs/zh-CN/security.md#reporting-security-issues)

### 插件市场请求

Claude Code 直接从每位开发者的机器获取插件市场，而不是通过网关。[网络访问要求](https://code.claude.com/docs/zh-CN/network-config.md#network-access-requirements) 列出了相关主机。

开发者首次启动交互式终端会话时，Claude Code 会注册官方市场 `claude-plugins-official`。它从 `downloads.claude.ai` 下载目录，如果失败，则从 `github.com` 克隆。[哪些市场和插件会自动更新](https://code.claude.com/docs/zh-CN/plugins/loading.md#which-marketplaces-and-plugins-auto-update) 介绍了后续刷新。

`CLAUDE_CODE_DISABLE_NONESSENTIAL_TRAFFIC=1` 不会阻止首次注册。以下任一托管设置可以阻止：

* **市场列表**：不包含该市场的 [`strictKnownMarketplaces`](https://code.claude.com/docs/zh-CN/plugins/org#allowlist-with-strictknownmarketplaces) 允许列表，或指明该市场的 [`blockedMarketplaces`](https://code.claude.com/docs/zh-CN/plugins/org#blocklist-with-blockedmarketplaces) 条目
* **环境变量**：在托管的 [`env` 块](https://code.claude.com/docs/zh-CN/plugins/org#turn-updates-off-for-the-whole-fleet) 中将 `CLAUDE_CODE_DISABLE_OFFICIAL_MARKETPLACE_AUTOINSTALL` 设置为 `"1"`

首次注册可能在开发者登录网关之前运行，此时尚未收到任何网关策略。为覆盖首次启动，请在 [客户端托管设置](https://code.claude.com/docs/zh-CN/claude-apps-gateway-config.md#client-side-managed-settings) 以及网关策略的 [`cli` 或 `code` 块](https://code.claude.com/docs/zh-CN/claude-apps-gateway-config#what-goes-in-cli) 中同时传递您的选择。

## 故障排除

如有问题和反馈，请使用 [Claude Code 支持](https://support.claude.com/en/collections/14445694-claude-code)，或在 [Claude Code GitHub 仓库](https://github.com/anthropics/claude-code/issues)上提交问题。您也可以联系您的 Anthropic 客户团队。报告问题时，请包括：

* **网关问题**：相关时间窗口内网关的 stderr、您的 `gateway.yaml`（已隐藏密钥）、网关版本（显示在 `/` 的登陆页面和 `/managed/settings` 的 `x-cc-gateway-version` 响应头中），以及最近的更改
* **登录问题**：开发者运行 `claude --debug-file ./claude-debug.txt`、重现问题，然后发送该文件以及同一时间窗口内网关的审计日志
* **推理问题**：请求的模型、配置的上游服务，以及该请求的网关审计日志，其中记录了由哪个上游提供服务以及响应状态

网关的 stderr 包含审计事件流，审计日志记录开发者身份，调试文件记录来自开发者机器的 hook 和 MCP 服务器输出。在发布到公开问题前，请审查并隐藏这些内容。

| 症状 | 原因 | 解决方案 |
| - | - | - |
| 开发者的 `/login` 显示标准账户选择器而不是 **Cloud gateway** 屏幕 | 该机器的托管设置中未设置 `forceLoginMethod` 或 `forceLoginGatewayUrl` | 将[托管设置文件](https://code.claude.com/docs/zh-CN/claude-apps-gateway#set-the-gateway-url)部署到设备；`/login` 从那里读取网关 URL |
| 开发者的请求失败，显示 `Not signed in to the Cloud gateway — run /login.` | 机器的托管设置设置了 `forceLoginMethod: "gateway"` 或 `forceLoginGatewayUrl`，且会话没有网关登录。残留的 claude.ai 登录不满足要求。 | 让开发者运行 `/login` 并完成网关登录。另请参阅[管理员策略需要 Cloud gateway 登录](https://code.claude.com/docs/zh-CN/errors.md#administrator-policy-requires-a-cloud-gateway-sign-in)。 |
| Claude Desktop 报告其引导配置无法获取 | `/user/bootstrap` 返回 404：与用户匹配的策略不包含 `desktop` 键，或没有策略匹配。网关的审计日志将每次拒绝记录为 `desktop_bootstrap.denied` 并说明原因。 | 将 `desktop` 块添加到与用户匹配的策略，或添加到 `match: {}` 基础层；空的 `desktop: {}` 就足够了。请参阅 [Claude Desktop 覆盖](https://code.claude.com/docs/zh-CN/claude-apps-gateway-config.md#claude-desktop-overlay)。 |
| 审计日志在 `/v1/logs` 或 `/v1/metrics` 上显示 `auth.denied`，并带有 `reason: invalid_token` | 您在某台机器的托管设置中设置了 [`otelHeadersHelper`](https://code.claude.com/docs/zh-CN/settings-reference#otelheadershelper)。在该机器上的 Claude Desktop 会话中，Claude Code 会将来自该 helper 的 `Authorization` 头发送到网关的遥测中继，以代替会话令牌。如果 Claude Desktop 捆绑的 Claude Code 早于 v2.1.307，而您在这些设置中通过 `OTEL_EXPORTER_OTLP_HEADERS` 或其按信号区分的形式设置了 `Authorization` 头，也会出现同样的拒绝。 | 从这些设置中删除该 helper。如果 Claude Desktop 捆绑的 Claude Code 早于 v2.1.307，请删除该变量。 |
| 启动显示 `Gateway login is configured in managed settings, but this Claude Code build does not include Cloud gateway support.` | 已安装的 Claude Code 版本早于网关支持 | 让开发者更新 Claude Code 到包含 Cloud gateway 支持的版本 |
| 启动退出，显示 `Administrator policy requires a Cloud gateway sign-in on this machine` | 开发者的环境设置了 `ANTHROPIC_API_KEY` 或 `ANTHROPIC_AUTH_TOKEN`，其设置配置了 [`apiKeyHelper`](https://code.claude.com/docs/zh-CN/settings-reference#apikeyhelper)，或来自早期 Claude Console 登录的 API 密钥仍然保存 | 让开发者清除每个适用的项：取消设置变量、删除 `apiKeyHelper` 条目，或运行 `claude auth logout` 删除保存的密钥。之后，使用 `CLAUDE_CODE_USE_*` 选择云提供商的会话无需登录即可启动；对于其他所有会话，让他们启动 `claude` 并使用 `/login` 登录。另请参阅[管理员策略需要 Cloud gateway 登录](https://code.claude.com/docs/zh-CN/errors.md#administrator-policy-requires-a-cloud-gateway-sign-in)。 |
| 启动或 `/login` 在托管设置加载时返回 403 后报告 `Claude Code may not be enabled for your organization` | 网关或其前面的某个组件用 403 响应了 `/managed/settings` 请求。网关自身的设置路由从不返回 403。状态来自 [`access_control`](https://code.claude.com/docs/zh-CN/claude-apps-gateway-config#http-tuning) IP 检查或网关前面的代理或 WAF。审计日志将 IP 检查拒绝记录为 `access.denied` 并说明原因。开发者保持登录状态。 | 检查审计日志中失败时的 `access.denied`，修复 `access_control` 列表或前端，然后让开发者再次启动 `claude` |
| CLI `/login`：`The gateway is limiting sign-in attempts right now`，或在较旧版本上 `Request failed with status code 429`。`/device` 页面可能向尚未尝试过的开发者显示 `Too many attempts` | 达到了每 IP 登录速率限制。要么 `listen.trusted_proxies` 不覆盖负载均衡器，所以每个开发者共享其地址，要么许多开发者共享一个 NAT 或 VPN 出口地址。具有 `result: rate_limited` 的审计事件显示相同的一个或几个 `client_ip` 值。 | 首先将 `listen.trusted_proxies` 设置为负载均衡器的源范围，然后如果开发者仍然共享地址，提高 `rate_limits`。请参阅[大规模推出](#large-rollouts)。 |
| CLI `/login`：`Gateway hosts must be on your organization's private network; <host> resolves to the public (or unrecognized) address <ip>` | 网关主机名解析为至少一个公网 IP 地址。Claude Code 检查每个解析的地址，要求每个都是私网。常见原因是双栈名称，其中一个族解析为公网地址，包括 AWS 内部双栈负载均衡器，它们返回公网范围的 AAAA 地址。 | 让网关名称在开发者机器上仅解析为私网地址。对于双栈名称，删除公网范围的记录或提供单独的仅内部 DNS 名称。请参阅[私网前提条件](https://code.claude.com/docs/zh-CN/claude-apps-gateway#prerequisites)。如果地址是您的组织拥有并在内部使用的公网空间，请[声明该块](https://code.claude.com/docs/zh-CN/claude-apps-gateway#allow-a-gateway-on-public-address-space-you-own)。 |
| CLI `/login`：`Gateway login would go through proxy <proxy>, which is not on a private network` | `HTTPS_PROXY` 或 `HTTP_PROXY` 适用于网关主机，且代理的主机名解析为公网地址。主机名仅解析为私网地址的代理是允许的，不会触发此错误 | 在开发者的机器上将网关主机添加到 `NO_PROXY`，以便连接是直接的，或使用主机名解析为私网地址的代理。消息会命名要添加的确切 `NO_PROXY` 条目 |
| CLI `/login`：`Claude Code only signs in to <host> from inside its declared network <block> (managed settings), and this machine is connecting from <ip>, outside it` | 网关在 [`gatewayInternalNetworks`](https://code.claude.com/docs/zh-CN/claude-apps-gateway#allow-a-gateway-on-public-address-space-you-own) 中声明的块上，开发者的机器从该块外的地址到达它：VPN 地址池、容器或 WSL2 NAT 段，或不是您的网络 | 让开发者从您的网络上的主机 OS 运行 `/login`。如果显示的地址也是您的组织自己的公网空间，将网关的条目替换为覆盖两者的块，最多 `/8`；第二个重叠条目会被拒绝。如果没有能覆盖两者的块，请参阅[为网关选择地址](#choose-an-address-for-the-gateway) |
| CLI `/login`：`Every address for gateway host <host> must be inside its declared network <block>, and it also resolves to <ip>` | 网关的名称解析为 [`gatewayInternalNetworks`](https://code.claude.com/docs/zh-CN/claude-apps-gateway#allow-a-gateway-on-public-address-space-you-own) 中声明的块外的地址：第二个站点，或双栈名称上的 IPv6 记录。在声明的块下，每条记录都必须在该单个 IPv4 块内，包括私网和 IPv6 地址 | 在开发者机器上仅为网关名称发布块内的记录，或提供单独的仅内部名称 |
| CLI `/login`：`<host> is on the declared network <block>, which Claude Code checks over a direct connection, not through an HTTP proxy` | `HTTPS_PROXY` 或 `HTTP_PROXY` 适用于声明块上的网关 | 在开发者的机器上，添加消息命名的 `NO_PROXY` 条目 |
| CLI `/login`：消息以 `gatewayInternalNetworks in managed settings` 开头 | 该值违反了[验证规则](https://code.claude.com/docs/zh-CN/claude-apps-gateway#allow-a-gateway-on-public-address-space-you-own)之一，消息会命名哪一个。在您修复它之前，Claude Code 拒绝机器上的每个新网关 `/login`，包括私网上的网关；现有登录保持工作 | 在您部署的托管设置源中，更正消息命名的条目，然后重新运行 `/login` |
| CLI `/login`：`Could not resolve the configured HTTP proxy` | `HTTPS_PROXY` 或 `HTTP_PROXY` 中的主机名无法从开发者的机器解析，通常是因为它未连接到公司网络 | 让开发者连接到您的网络或 VPN 并重试，或修复代理 URL |
| CLI `/login`：`Could not resolve gateway host <host>` | 机器无法解析网关的内部 DNS 名称，通常是因为它不在公司网络上 | 让开发者连接到您的网络或 VPN，然后重试 `/login` |
| 启动退出，显示配置验证错误，命名 `store.postgres_url` | 未配置 Postgres；网关需要 Postgres | 设置 `store.postgres_url`。对于本地开发，使用一次性容器：`docker run --rm -p 5432:5432 -e POSTGRES_HOST_AUTH_METHOD=trust postgres`。 |
| 启动退出：`store.postgres_url in <path> is not a URL the gateway can read`，或在 v2.1.290 之前仅显示 `Invalid URL` 或 `URI error` | 无法解析该 URL，例如因为它列出了多个主机，或其密码包含未编码的 `/`、`?`、`#` 或 `%` | 仅指定[一个主机](#postgres)，并将密码移到 [`store.password`](https://code.claude.com/docs/zh-CN/claude-apps-gateway-config#store) 中 |
| 启动退出：`requires the native binary` | 在 Node 下运行而不是本地二进制 | 使用[独立安装方法](https://code.claude.com/docs/zh-CN/setup.md)之一安装 Claude Code |
| 启动退出，在 `config.load` 后显示 OIDC 发现错误 | `oidc.issuer` 无法访问，或 TLS 链不受信任 | 检查发行者是否可从 pod 访问并提供 `/.well-known/openid-configuration`。为私有 PKI 设置 `ca_cert_pem`。如果 pod 仅通过前向代理到达 IdP，设置 [`oidc.use_proxy: true`](https://code.claude.com/docs/zh-CN/claude-apps-gateway-config#idp-requests-through-a-forward-proxy)；在 v2.1.227 之前的版本上，改为给 pod 一条到 IdP 每个端点的直接路由。如果 pod 也无法解析 IdP 的主机名，或代理拒绝 `CONNECT` 到 IP 地址，请参阅[仅代理出口](https://code.claude.com/docs/zh-CN/claude-apps-gateway-config.md#proxy-only-egress)，这需要 v2.1.277 或更高版本。 |
| 启动退出，显示 Postgres 权限错误 | 数据库角色在其 schema 上缺少 DDL 权限 | 授予角色对网关 schema 的 `CREATE` 权限，以便它可以在启动时创建和修改其表 |
| 日志：来自 PostgreSQL 的 `remaining connection slots are reserved` 或 `sorry, too many clients already`。它出现在正在运行的副本的 `postgres connection closed` 警告中，或新副本的启动日志中。在启动日志中，`remaining connection slots are reserved` 可能出现在 `attempt 1 of 3` 之后，且该副本仍可能启动。 | 副本请求的连接数超过了数据库为网关剩余的连接数，PostgreSQL 拒绝了多出的连接 | 按照[调整连接池大小](#size-the-connection-pool)中的说明使总数符合限制：运行更少的副本、降低 `store.max_connections`，或提高数据库的 `max_connections`。如果总数仅在滚动更新期间超出，而您的编排器支持，也可以让编排器在启动新副本之前先停止旧副本。 |
| 日志：`could not connect to Postgres at boot, attempt 1 of 3`，且原因不是 `remaining connection slots are reserved` | 网关启动时数据库尚无法访问，例如在网络仍在启动的冷实例上 | 如果网关随后完成启动，无需采取任何措施。当数据库无法访问时，网关在退出前尝试连接三次，间隔两秒。如果它以 `could not connect to Postgres` 退出，检查 `store.postgres_url`（包括它是否只指定了一个主机）以及到数据库的网络路径。如果尝试超时而不是被拒绝，提高 [`store.connect_timeout_seconds`](https://code.claude.com/docs/zh-CN/claude-apps-gateway-config#store) 以给每个尝试更长的时间。 |
| 日志：`Postgres <version> is below the supported floor` | 数据库运行的 PostgreSQL 版本低于您的网关版本所支持的版本。网关会继续运行。 | 升级数据库。在 PostgreSQL 11、12 或 13 上，您也可以改为升级网关，以满足 [PostgreSQL 要求](https://code.claude.com/docs/zh-CN/claude-apps-gateway#prerequisites)。 |
| `/oauth/callback` 显示"Sign-in could not be completed" | 电子邮件域被拒绝、id\_token 验证失败，或 `email_verified` 显式为 `false`，网关总是拒绝且无覆盖 | 检查 `allowed_email_domains` 和 IdP 是否返回已验证的 `email` 声明。对于 `email_verified: false`，修复 IdP 端验证。如果您的 IdP 在不同的声明名称下发出电子邮件，设置 `oidc.email_claim`。 |
| 日志：`token exchange failed request_id=<id>: id_token missing email claim` | IdP 默认不在 id\_token 中包含 `email`。此拒绝仅在设置 `allowed_email_domains` 时触发；没有它，缺少的电子邮件会创建没有电子邮件的会话 | 配置 IdP 在 id\_token 中发出 `email`。Okta：将 `email` 添加到自定义授权服务器的 ID 令牌声明。Entra：在应用注册上添加 `email` 作为可选声明。PingFederate：启用发出 `email` 的 OpenID Connect 策略。如果 IdP 从 userinfo 端点提供 `email` 但不会在 id\_token 中包含它，例如 Okta 组织授权服务器，设置 `oidc.userinfo_fallback: true`。 |
| 日志：`refresh failed request_id=<id>: invalid_token (…) (at userinfo_no_id_token, …)`，开发者每 `session.ttl_hours` 看到 `Cloud gateway session expired` | IdP 接受了刷新令牌但没有随之返回 id\_token，所以网关询问了 IdP 的 userinfo 端点以获取用户的声明。IdP 在那里拒绝了刷新的访问令牌。网关回答 `temporarily_unavailable`，所以 Claude Code 保留刷新令牌但无法续订会话。v2.1.260 之前的网关版本记录相同的行但没有 `(at …)` 详情。 | 设置 [`oidc.scope_on_refresh: true`](https://code.claude.com/docs/zh-CN/claude-apps-gateway-config#oidc)，在网关 v2.1.260 或更高版本中可用，以便刷新请求再次请求 `openid`。某些 IdP（如 Okta）仅在被要求时在刷新时返回 id\_token。在 PingFederate 上，改为在 **Applications > OAuth > OpenID Connect Policy Management** 下启用 **Return ID Token On Refresh Grant**。该设置项不会改变 PingFederate 的行为。对于仍然省略它的其他 IdP，检查 userinfo 端点是否接受由刷新发出的访问令牌。作为临时措施，提高 [`session.ttl_hours`](https://code.claude.com/docs/zh-CN/claude-apps-gateway-config#session)。请参阅[身份提供者设置](#identity-provider-setup)了解取消配置权衡。 |
| 开发者登录后，来自该会话的每个请求都失败并返回 `431` 错误 | 每个请求的 `Authorization` 头中的会话令牌列出了开发者的 IdP 组，因此对于属于许多组的开发者，请求头总大小可能超过网关接受的上限 | 请参阅[登录后请求头过大](#request-headers-too-large-after-sign-in)，了解适用哪个限制以及需要更改什么 |
| 每个 Amazon Bedrock 请求都返回 502；日志显示 `Could not load credentials from any providers` | 在 EC2 上，IMDSv2 的默认跳数限制为 1，阻止了来自容器内的实例元数据请求。启动和 `/readyz` 仍然通过，因为 AWS SDK 在第一个请求时解析实例凭据，而不是在客户端构造时 | 使用 `aws ec2 modify-instance-metadata-options --instance-id <id> --http-put-response-hop-limit 2` 提高跳数限制，或在启动模板中设置它。更改适用于实例上的每个容器。在可用的地方优先使用 ECS 任务角色，它们从 ECS 容器凭据端点读取凭据并完全避免更改，或在专用网关实例上应用更改以限制暴露。 |
| 在峰值负载下，响应开始缓慢或似乎挂起，或在上游健康时失败，显示 502 `all upstreams failed` | 副本打开的请求比它一次发送到上游的请求多，所以额外的请求在网关内等待。在 `provider: anthropic` 上游上，等待时间超过 `timeouts.upstream_ttfb_ms` 的请求放弃该上游，当没有后续上游提供服务时会产生 502。日志显示包含 `client requests are open` 的警告。 | 添加副本，或提高每个副本上的限制。请参阅[并发上游请求](#concurrent-upstream-requests)。 |
| 在 `provider: anthropic` 以外的上游上，包含较长思考停顿的轮次失败。网关前面的代理记录了一个保持打开约 12.5 分钟的请求，随后是一连串 `504` 错误 | Claude Code 结束了一个已传送首个响应事件、此后仅携带 keepalive ping 的流。它以非流式方式重新发送该请求，而每个回复所需的时间都超过了代理的超时时间。 | 为您的开发者提高 `CLAUDE_STREAM_IDLE_TIMEOUT_MS`。请参阅[长时间思考导致代理超时](#long-thinking-ends-in-proxy-timeouts)。 |
| 在 `provider: anthropic` 上，包含较长思考停顿的轮次失败。代理记录了一个保持打开约 20 分钟的请求，随后是 `502` 或 `504` 错误 | Claude Code 结束了一个已传送首个响应事件、此后仅携带 keepalive ping 的流。它以非流式方式重新发送该请求，而每个回复所需的时间都超过了网关对上游的等待时间 `timeouts.upstream_ttfb_ms` 或代理的超时时间。 | 为您的开发者提高 `CLAUDE_STREAM_IDLE_TIMEOUT_MS`。请参阅[Anthropic API 上游上的长时间思考](#long-thinking-on-an-anthropic-api-upstream)。 |
| IdP 错误：unknown or unsupported scope | IdP 拒绝它不识别的作用域 | 将 `oidc.scopes` 设置为您的 IdP 接受的确切列表；它必须包含 `openid`。默认值为 `openid profile email offline_access`。 |
| 设置 `oidc.scopes` 后会话不会静默续订 | `offline_access` 从覆盖中删除了 | 如果您的 IdP 支持，添加 `offline_access` 回来。没有刷新令牌，开发者每 `session.ttl_hours` 重新运行浏览器登录。 |
| 浏览器显示"This request came from another site and was blocked" | 跨站点表单 POST，作为 CSRF 保护被阻止。对于嵌入或代理的页面是预期的 | 直接打开验证链接 |
| Chrome 用"Refused to send form data … violates … Content Security Policy directive: form-action"阻止"Approve"按钮，但相同的页面在 Safari 或 Firefox 中工作 | Chrome 对整个重定向链强制执行 `form-action`。您的 IdP 重定向到第二个主机，该主机未被列入白名单。 | 将重定向链中的每个额外来源添加到 `oidc.form_action_origins`。在"Approve"页面上打开 Chrome DevTools → Console 以查看哪个来源被阻止。 |
| 登录在 IdP 处完成但回调失败，Chrome 中出现 CSP 错误或 Safari 中出现"this sign-in link has expired" | IdP 通过 `response_mode=form_post` 返回了代码，它通过 POST 跨源自动提交到 `/oauth/callback`。Chrome 在严格 CSP 下阻止了这个；Safari 允许提交但回调仅读取查询字符串。 | 确保您的 IdP 遵守 `response_mode=query`，网关明确请求它以便回调是普通重定向 |
| 登录在本地工作但在 ALB 后面失败 | `public_url` 仍然命名本地或内部 `http://` 来源，所以 IdP 获得了错误的 `redirect_uri` | 将 `listen.public_url` 设置为外部 `https://` 来源，并向 IdP 注册 `<public_url>/oauth/callback` |
| 开发者重复看到信任提示 | TLS 证书按副本或按请求轮换 | 在入口处使用稳定的证书，或终止 TLS 一次并在内部通过普通 HTTP 运行副本 |
| CLI `/login`："Could not verify the gateway's TLS certificate"或 `SELF_SIGNED_CERT_IN_CHAIN` | 网关的 TLS 链由 CLI 主机的信任存储中不存在的私有 CA 签名 | Claude Code 在本地二进制以及 Node 22.15 或更高版本上默认读取 OS 信任存储；[`CLAUDE_CODE_CERT_STORE`](https://code.claude.com/docs/zh-CN/network-config#ca-certificate-store) 控制此行为。如果 CA 安装在 OS 信任存储中，确保开发者使用当前运行时。否则在启动前将 `NODE_EXTRA_CA_CERTS` 设置为 CA 证书 PEM。首次连接指纹提示仍然适用。 |
| CLI `/login` 完成浏览器登录，然后会话以 `Cloud gateway sign-in was not completed` 和 TLS 证书不匹配结束 | 在登录后的第一个请求中，网关提供了与 Claude Code 固定的指纹不匹配的证书，所以 Claude Code 没有保留任何网关凭据。常见原因是一个地址后面的副本提供不同的证书，或网络路径上的某个东西拦截了 TLS。 | 为主机名提供一个证书，例如在入口处终止 TLS 一次，然后让开发者再次运行 `/login`。如果该证书与固定的不同，Claude Code 会在[信任提示](https://code.claude.com/docs/zh-CN/claude-apps-gateway#connect-developers)处显示警告，说明证书已更改。 |
| CLI `/login` 停止，显示 `The gateway's TLS certificate changed during sign-in: it no longer matches the one you trusted` | 登录请求到达了一个证书与开发者在 `/login` 启动时接受的证书不匹配的服务器：一个地址后面的副本提供不同的证书、路径上的 TLS 拦截，或登录进行中的证书轮换。 | 为主机名提供一个证书，然后让开发者再次启动登录并在[信任提示](https://code.claude.com/docs/zh-CN/claude-apps-gateway#connect-developers)处审查新证书。 |

`Cloud gateway sign-in was not completed` 消息会命名网关主机名。当 Claude Code 同时拥有固定指纹和呈现的指纹时，消息还显示每个的前 16 个字符。

如果 Claude Code 在网关登录后报告 `couldn't load your organization's managed settings`，Claude Code 会命名原因、就地重启并恢复对话。如果 Claude Code 无法重启，例如在后台会话中，Claude Code 会结束会话并保留登录。

### 登录后请求头过大

当开发者属于许多 IdP 组时，其请求可能在登录后失败并返回 `431` 错误。

当请求头总大小超过 256 KiB，或者在您设置了 [`limits.max_request_header_bytes`](https://code.claude.com/docs/zh-CN/claude-apps-gateway-config#http-tuning) 时超过该值，网关会返回 `431`。对于这些请求，网关不会写入任何日志行或审计事件。v2.1.284 之前的网关版本在超过 16 KiB 时返回 `431`。

从以下各项中第一个适用于您网关的情况开始处理：

* **网关版本早于 v2.1.284**：升级网关
* **设置了 `limits.max_request_header_bytes`**：提高该值或删除该键
* **以上均不适用，或之后仍出现 `431`**：让您的 IdP 发出更少的组。[身份提供者设置](#identity-provider-setup)介绍了 Okta、Microsoft Entra ID 和 Google Workspace 如何提供组

精简组声明时，请保留您在以下设置中指定的组，这些设置决定开发者的访问权限、策略和支出上限：

* **[`oidc.allowed_groups`](https://code.claude.com/docs/zh-CN/claude-apps-gateway-config#oidc)**：决定谁可以登录
* **[`admin.admin_groups`](https://code.claude.com/docs/zh-CN/claude-apps-gateway-config#admin)**：决定谁可以使用其网关会话调用管理 API
* **[`managed.policies`](https://code.claude.com/docs/zh-CN/claude-apps-gateway-config#managed) 中的 `match.groups`**：决定哪个策略适用于开发者
* **`rbac_group` [支出上限](https://code.claude.com/docs/zh-CN/claude-apps-gateway-spend-limits.md)**：决定哪些组上限适用于开发者

### 长时间思考导致代理超时

如果您的上游是 `provider: anthropic`，请改用 [Anthropic API 上游上的长时间思考](#long-thinking-on-an-anthropic-api-upstream)：在那里您同样需要提高 `CLAUDE_STREAM_IDLE_TIMEOUT_MS`，但时间和错误有所不同。

如果轮次在长时间思考后失败，且您的代理记录了一个保持打开约 12.5 分钟的请求，随后是一连串 `504` 错误，请在网关策略中提高 [`CLAUDE_STREAM_IDLE_TIMEOUT_MS`](https://code.claude.com/docs/zh-CN/env-vars)。例如，如果代理的空闲超时时间为 240 秒，这一连串错误就是 10 个请求，每个都在 240 秒时被切断。

* **第一个请求**：一个流，它传送首个响应事件 `message_start`，然后在 Claude 思考期间仅发送 ping。在默认设置下，Claude Code 会在约 12.5 分钟后结束该流。
* **其余 10 个**：同一个请求，以[非流式方式](https://code.claude.com/docs/zh-CN/errors.md#streaming-response-ended-before-any-complete-data-was-received)发送并随后重试，默认总共发送 10 次。它们不会收到 ping，因此较慢的回复会触发代理的超时。

此修复不需要更长的代理超时时间。如果您只提高代理的超时时间，一旦回复所需时间超过 Claude Code 自身的等待时间（默认 300 秒），非流式请求仍会失败。

#### 确认原因

让开发者使用 `claude --debug-file ./claude-debug.txt` 重现该失败，然后在该文件中搜索以下行：

```text theme={null}
Error streaming, falling back to non-streaming mode: Stream idle timeout - no chunks received
```

如果您的开发者已将 [`CLAUDE_CODE_DISABLE_NONSTREAMING_FALLBACK`](https://code.claude.com/docs/zh-CN/env-vars) 设置为 `1`，您不会看到任何非流式请求，该轮次会以 `API Error: Stream idle timeout - no chunks received` 结束。同样的修复方法适用。

#### 选择值

选择一个 `CLAUDE_STREAM_IDLE_TIMEOUT_MS` 值，使其第二列的时间能覆盖您最长的思考停顿；如果您不清楚，可以从 `1800000` 开始。流仅靠 ping 维持的时间可比该值长约 7.5 分钟：

| `CLAUDE_STREAM_IDLE_TIMEOUT_MS` | 流仅靠 ping 可维持的时间 |
| - | - |
| `300000`（默认值） | 约 12.5 分钟 |
| `900000` | 约 22.5 分钟 |
| `1800000` | 约 37.5 分钟 |

如果上游挂起而网关持续发送 ping，您的开发者也需要等待同样长的时间，Claude Code 才会放弃该流。完全静默的连接仍会在 5 到 6 分钟后结束，因为[响应体空闲超时](https://code.claude.com/docs/zh-CN/network-config.md#streaming-idle-watchdogs)不受此变量影响。

#### 通过策略下发该值

在 `gateway.yaml` 中，将 `env` 条目添加到已向您要覆盖的开发者下发 Claude Code 设置的每个策略中。将其放在该策略已有的 `code`、`cli` 或 `settings` 键下。如需先试用该值，可以从匹配单个组的策略开始。有关策略如何匹配和继承，请参阅 [`managed`](https://code.claude.com/docs/zh-CN/claude-apps-gateway-config#managed)。以下示例将该条目添加到键为 `cli` 的 `match: {}` 策略中：

```yaml theme={null}
managed:
  policies:
    - match: {}
      cli:
        env:
          CLAUDE_STREAM_IDLE_TIMEOUT_MS: "1800000"
```

使用 `cli` 键时，您可以覆盖终端、IDE 扩展和 Agent SDK 会话。对于 Claude Desktop 的 Code 标签页，请参阅[选择 `cli` 或 `code`](https://code.claude.com/docs/zh-CN/claude-apps-gateway-config#choose-cli-or-code)。

重启网关，使其读取编辑后的文件。然后让开发者重启 Claude Code 以获取 `CLAUDE_STREAM_IDLE_TIMEOUT_MS`。

### Anthropic API 上游上的长时间思考

如果 `provider: anthropic` 是该模型唯一的上游，且您的代理记录了一个保持打开约 20 分钟的请求，随后是 `502` 或 `504` 错误，请在网关策略中提高 [`CLAUDE_STREAM_IDLE_TIMEOUT_MS`](https://code.claude.com/docs/zh-CN/env-vars)。

* **长时间请求**：一个流，它传送 `message_start`，然后在 Claude 思考期间仅发送 ping。在默认设置下，Claude Code 会在约 20 分钟后结束它；提高该变量后，结束时间会相应推迟。
* **错误**：Claude Code 随后以[非流式方式](https://code.claude.com/docs/zh-CN/errors.md#streaming-response-ended-before-any-complete-data-was-received)发送的请求。每个请求都会因网关返回的 `502` 而失败；如果代理的超时时间短于网关的等待时间，则会因代理的 `504` 而失败。该等待时间即 [`timeouts.upstream_ttfb_ms`](https://code.claude.com/docs/zh-CN/claude-apps-gateway-config#http-tuning)，此修复无需提高它。

要确认原因，请参阅[确认原因](#confirm-the-cause)。

选择时间能覆盖您最长思考停顿的值：默认值约为 20 分钟，`900000` 约为 30 分钟，`1800000` 约为 45 分钟。然后按照[通过策略下发该值](#deliver-the-value-in-a-policy)操作。

## 相关

* [Claude 应用网关概述](https://code.claude.com/docs/zh-CN/claude-apps-gateway)：快速入门和开发者连接
* [配置参考](https://code.claude.com/docs/zh-CN/claude-apps-gateway-config.md)：每个 `gateway.yaml` 选项
